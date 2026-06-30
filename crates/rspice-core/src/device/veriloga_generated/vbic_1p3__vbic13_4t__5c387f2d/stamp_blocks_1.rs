#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        var_avcx2_t: f64,
        var_avcx2_t_dn0: f64,
        var_avcx2_t_dn1: f64,
        var_avcx2_t_dn10: f64,
        var_avcx2_t_dn11: f64,
        var_avcx2_t_dn12: f64,
        var_avcx2_t_dn13: f64,
        var_avcx2_t_dn2: f64,
        var_avcx2_t_dn3: f64,
        var_avcx2_t_dn4: f64,
        var_avcx2_t_dn5: f64,
        var_avcx2_t_dn6: f64,
        var_avcx2_t_dn7: f64,
        var_avcx2_t_dn8: f64,
        var_avcx2_t_dn9: f64,
        var_ibcj: f64,
        var_ibcj_dn0: f64,
        var_ibcj_dn1: f64,
        var_ibcj_dn10: f64,
        var_ibcj_dn11: f64,
        var_ibcj_dn12: f64,
        var_ibcj_dn13: f64,
        var_ibcj_dn2: f64,
        var_ibcj_dn3: f64,
        var_ibcj_dn4: f64,
        var_ibcj_dn5: f64,
        var_ibcj_dn6: f64,
        var_ibcj_dn7: f64,
        var_ibcj_dn8: f64,
        var_ibcj_dn9: f64,
        var_igc: f64,
        var_igc_dn0: f64,
        var_igc_dn1: f64,
        var_igc_dn10: f64,
        var_igc_dn11: f64,
        var_igc_dn12: f64,
        var_igc_dn13: f64,
        var_igc_dn2: f64,
        var_igc_dn3: f64,
        var_igc_dn4: f64,
        var_igc_dn5: f64,
        var_igc_dn6: f64,
        var_igc_dn7: f64,
        var_igc_dn8: f64,
        var_igc_dn9: f64,
        var_ircx: f64,
        var_ircx_dn0: f64,
        var_ircx_dn1: f64,
        var_ircx_dn10: f64,
        var_ircx_dn11: f64,
        var_ircx_dn12: f64,
        var_ircx_dn13: f64,
        var_ircx_dn2: f64,
        var_ircx_dn3: f64,
        var_ircx_dn4: f64,
        var_ircx_dn5: f64,
        var_ircx_dn6: f64,
        var_ircx_dn7: f64,
        var_ircx_dn8: f64,
        var_ircx_dn9: f64,
        var_itzf: f64,
        var_itzf_dn0: f64,
        var_itzf_dn1: f64,
        var_itzf_dn10: f64,
        var_itzf_dn11: f64,
        var_itzf_dn12: f64,
        var_itzf_dn13: f64,
        var_itzf_dn2: f64,
        var_itzf_dn3: f64,
        var_itzf_dn4: f64,
        var_itzf_dn5: f64,
        var_itzf_dn6: f64,
        var_itzf_dn7: f64,
        var_itzf_dn8: f64,
        var_itzf_dn9: f64,
        var_maxvibcip: f64,
        var_maxvibcip_dn0: f64,
        var_maxvibcip_dn1: f64,
        var_maxvibcip_dn10: f64,
        var_maxvibcip_dn11: f64,
        var_maxvibcip_dn12: f64,
        var_maxvibcip_dn13: f64,
        var_maxvibcip_dn2: f64,
        var_maxvibcip_dn3: f64,
        var_maxvibcip_dn4: f64,
        var_maxvibcip_dn5: f64,
        var_maxvibcip_dn6: f64,
        var_maxvibcip_dn7: f64,
        var_maxvibcip_dn8: f64,
        var_maxvibcip_dn9: f64,
        var_vbci: f64,
        var_vbci_dn0: f64,
        var_vbci_dn1: f64,
        var_vbci_dn10: f64,
        var_vbci_dn11: f64,
        var_vbci_dn12: f64,
        var_vbci_dn13: f64,
        var_vbci_dn2: f64,
        var_vbci_dn3: f64,
        var_vbci_dn4: f64,
        var_vbci_dn5: f64,
        var_vbci_dn6: f64,
        var_vbci_dn7: f64,
        var_vbci_dn8: f64,
        var_vbci_dn9: f64,
        var_vbcp: f64,
        var_vbcp_dn0: f64,
        var_vbcp_dn1: f64,
        var_vbcp_dn10: f64,
        var_vbcp_dn11: f64,
        var_vbcp_dn12: f64,
        var_vbcp_dn13: f64,
        var_vbcp_dn2: f64,
        var_vbcp_dn3: f64,
        var_vbcp_dn4: f64,
        var_vbcp_dn5: f64,
        var_vbcp_dn6: f64,
        var_vbcp_dn7: f64,
        var_vbcp_dn8: f64,
        var_vbcp_dn9: f64,
        var_vbxcx: f64,
        var_vbxcx_dn0: f64,
        var_vbxcx_dn1: f64,
        var_vbxcx_dn10: f64,
        var_vbxcx_dn11: f64,
        var_vbxcx_dn12: f64,
        var_vbxcx_dn13: f64,
        var_vbxcx_dn2: f64,
        var_vbxcx_dn3: f64,
        var_vbxcx_dn4: f64,
        var_vbxcx_dn5: f64,
        var_vbxcx_dn6: f64,
        var_vbxcx_dn7: f64,
        var_vbxcx_dn8: f64,
        var_vbxcx_dn9: f64,
        var_vmaxexp: f64,
        var_vtv: f64,
        var_vtv_dn0: f64,
        var_vtv_dn1: f64,
        var_vtv_dn10: f64,
        var_vtv_dn11: f64,
        var_vtv_dn12: f64,
        var_vtv_dn13: f64,
        var_vtv_dn2: f64,
        var_vtv_dn3: f64,
        var_vtv_dn4: f64,
        var_vtv_dn5: f64,
        var_vtv_dn6: f64,
        var_vtv_dn7: f64,
        var_vtv_dn8: f64,
        var_vtv_dn9: f64,
        var_afac_slot: &mut f64,
        var_afac_dn0_slot: &mut f64,
        var_afac_dn1_slot: &mut f64,
        var_afac_dn10_slot: &mut f64,
        var_afac_dn11_slot: &mut f64,
        var_afac_dn12_slot: &mut f64,
        var_afac_dn13_slot: &mut f64,
        var_afac_dn2_slot: &mut f64,
        var_afac_dn3_slot: &mut f64,
        var_afac_dn4_slot: &mut f64,
        var_afac_dn5_slot: &mut f64,
        var_afac_dn6_slot: &mut f64,
        var_afac_dn7_slot: &mut f64,
        var_afac_dn8_slot: &mut f64,
        var_afac_dn9_slot: &mut f64,
        var_avalf_slot: &mut f64,
        var_avalf_dn0_slot: &mut f64,
        var_avalf_dn1_slot: &mut f64,
        var_avalf_dn10_slot: &mut f64,
        var_avalf_dn11_slot: &mut f64,
        var_avalf_dn12_slot: &mut f64,
        var_avalf_dn13_slot: &mut f64,
        var_avalf_dn2_slot: &mut f64,
        var_avalf_dn3_slot: &mut f64,
        var_avalf_dn4_slot: &mut f64,
        var_avalf_dn5_slot: &mut f64,
        var_avalf_dn6_slot: &mut f64,
        var_avalf_dn7_slot: &mut f64,
        var_avalf_dn8_slot: &mut f64,
        var_avalf_dn9_slot: &mut f64,
        var_expi_slot: &mut f64,
        var_expi__blk127_slot: &mut f64,
        var_expi__blk127_dn0_slot: &mut f64,
        var_expi__blk127_dn1_slot: &mut f64,
        var_expi__blk127_dn10_slot: &mut f64,
        var_expi__blk127_dn11_slot: &mut f64,
        var_expi__blk127_dn12_slot: &mut f64,
        var_expi__blk127_dn13_slot: &mut f64,
        var_expi__blk127_dn2_slot: &mut f64,
        var_expi__blk127_dn3_slot: &mut f64,
        var_expi__blk127_dn4_slot: &mut f64,
        var_expi__blk127_dn5_slot: &mut f64,
        var_expi__blk127_dn6_slot: &mut f64,
        var_expi__blk127_dn7_slot: &mut f64,
        var_expi__blk127_dn8_slot: &mut f64,
        var_expi__blk127_dn9_slot: &mut f64,
        var_expi_dn0_slot: &mut f64,
        var_expi_dn1_slot: &mut f64,
        var_expi_dn10_slot: &mut f64,
        var_expi_dn11_slot: &mut f64,
        var_expi_dn12_slot: &mut f64,
        var_expi_dn13_slot: &mut f64,
        var_expi_dn2_slot: &mut f64,
        var_expi_dn3_slot: &mut f64,
        var_expi_dn4_slot: &mut f64,
        var_expi_dn5_slot: &mut f64,
        var_expi_dn6_slot: &mut f64,
        var_expi_dn7_slot: &mut f64,
        var_expi_dn8_slot: &mut f64,
        var_expi_dn9_slot: &mut f64,
        var_expl__blk128_slot: &mut f64,
        var_guard123_slot: &mut f64,
        var_guard129_slot: &mut f64,
        var_guard130_slot: &mut f64,
        var_guard131_slot: &mut f64,
        var_guard132_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_ibc_slot: &mut f64,
        var_ibc_dn0_slot: &mut f64,
        var_ibc_dn1_slot: &mut f64,
        var_ibc_dn10_slot: &mut f64,
        var_ibc_dn11_slot: &mut f64,
        var_ibc_dn12_slot: &mut f64,
        var_ibc_dn13_slot: &mut f64,
        var_ibc_dn2_slot: &mut f64,
        var_ibc_dn3_slot: &mut f64,
        var_ibc_dn4_slot: &mut f64,
        var_ibc_dn5_slot: &mut f64,
        var_ibc_dn6_slot: &mut f64,
        var_ibc_dn7_slot: &mut f64,
        var_ibc_dn8_slot: &mut f64,
        var_ibc_dn9_slot: &mut f64,
        var_ibk_slot: &mut f64,
        var_ibk_dn0_slot: &mut f64,
        var_ibk_dn1_slot: &mut f64,
        var_ibk_dn10_slot: &mut f64,
        var_ibk_dn11_slot: &mut f64,
        var_ibk_dn12_slot: &mut f64,
        var_ibk_dn13_slot: &mut f64,
        var_ibk_dn2_slot: &mut f64,
        var_ibk_dn3_slot: &mut f64,
        var_ibk_dn4_slot: &mut f64,
        var_ibk_dn5_slot: &mut f64,
        var_ibk_dn6_slot: &mut f64,
        var_ibk_dn7_slot: &mut f64,
        var_ibk_dn8_slot: &mut f64,
        var_ibk_dn9_slot: &mut f64,
        var_igcx_slot: &mut f64,
        var_igcx_dn0_slot: &mut f64,
        var_igcx_dn1_slot: &mut f64,
        var_igcx_dn10_slot: &mut f64,
        var_igcx_dn11_slot: &mut f64,
        var_igcx_dn12_slot: &mut f64,
        var_igcx_dn13_slot: &mut f64,
        var_igcx_dn2_slot: &mut f64,
        var_igcx_dn3_slot: &mut f64,
        var_igcx_dn4_slot: &mut f64,
        var_igcx_dn5_slot: &mut f64,
        var_igcx_dn6_slot: &mut f64,
        var_igcx_dn7_slot: &mut f64,
        var_igcx_dn8_slot: &mut f64,
        var_igcx_dn9_slot: &mut f64,
        var_iibk_slot: &mut f64,
        var_iibk_dn0_slot: &mut f64,
        var_iibk_dn1_slot: &mut f64,
        var_iibk_dn10_slot: &mut f64,
        var_iibk_dn11_slot: &mut f64,
        var_iibk_dn12_slot: &mut f64,
        var_iibk_dn13_slot: &mut f64,
        var_iibk_dn2_slot: &mut f64,
        var_iibk_dn3_slot: &mut f64,
        var_iibk_dn4_slot: &mut f64,
        var_iibk_dn5_slot: &mut f64,
        var_iibk_dn6_slot: &mut f64,
        var_iibk_dn7_slot: &mut f64,
        var_iibk_dn8_slot: &mut f64,
        var_iibk_dn9_slot: &mut f64,
        var_mac1__blk126_slot: &mut f64,
        var_mac1__blk126_dn0_slot: &mut f64,
        var_mac1__blk126_dn1_slot: &mut f64,
        var_mac1__blk126_dn10_slot: &mut f64,
        var_mac1__blk126_dn11_slot: &mut f64,
        var_mac1__blk126_dn12_slot: &mut f64,
        var_mac1__blk126_dn13_slot: &mut f64,
        var_mac1__blk126_dn2_slot: &mut f64,
        var_mac1__blk126_dn3_slot: &mut f64,
        var_mac1__blk126_dn4_slot: &mut f64,
        var_mac1__blk126_dn5_slot: &mut f64,
        var_mac1__blk126_dn6_slot: &mut f64,
        var_mac1__blk126_dn7_slot: &mut f64,
        var_mac1__blk126_dn8_slot: &mut f64,
        var_mac1__blk126_dn9_slot: &mut f64,
        var_vcbfac_slot: &mut f64,
        var_vcbfac_dn0_slot: &mut f64,
        var_vcbfac_dn1_slot: &mut f64,
        var_vcbfac_dn10_slot: &mut f64,
        var_vcbfac_dn11_slot: &mut f64,
        var_vcbfac_dn12_slot: &mut f64,
        var_vcbfac_dn13_slot: &mut f64,
        var_vcbfac_dn2_slot: &mut f64,
        var_vcbfac_dn3_slot: &mut f64,
        var_vcbfac_dn4_slot: &mut f64,
        var_vcbfac_dn5_slot: &mut f64,
        var_vcbfac_dn6_slot: &mut f64,
        var_vcbfac_dn7_slot: &mut f64,
        var_vcbfac_dn8_slot: &mut f64,
        var_vcbfac_dn9_slot: &mut f64,
        var_vl__blk125_slot: &mut f64,
        var_vl__blk125_dn0_slot: &mut f64,
        var_vl__blk125_dn1_slot: &mut f64,
        var_vl__blk125_dn10_slot: &mut f64,
        var_vl__blk125_dn11_slot: &mut f64,
        var_vl__blk125_dn12_slot: &mut f64,
        var_vl__blk125_dn13_slot: &mut f64,
        var_vl__blk125_dn2_slot: &mut f64,
        var_vl__blk125_dn3_slot: &mut f64,
        var_vl__blk125_dn4_slot: &mut f64,
        var_vl__blk125_dn5_slot: &mut f64,
        var_vl__blk125_dn6_slot: &mut f64,
        var_vl__blk125_dn7_slot: &mut f64,
        var_vl__blk125_dn8_slot: &mut f64,
        var_vl__blk125_dn9_slot: &mut f64,
        var_vminm__blk124_slot: &mut f64,
        var_vminm__blk124_dn0_slot: &mut f64,
        var_vminm__blk124_dn1_slot: &mut f64,
        var_vminm__blk124_dn10_slot: &mut f64,
        var_vminm__blk124_dn11_slot: &mut f64,
        var_vminm__blk124_dn12_slot: &mut f64,
        var_vminm__blk124_dn13_slot: &mut f64,
        var_vminm__blk124_dn2_slot: &mut f64,
        var_vminm__blk124_dn3_slot: &mut f64,
        var_vminm__blk124_dn4_slot: &mut f64,
        var_vminm__blk124_dn5_slot: &mut f64,
        var_vminm__blk124_dn6_slot: &mut f64,
        var_vminm__blk124_dn7_slot: &mut f64,
        var_vminm__blk124_dn8_slot: &mut f64,
        var_vminm__blk124_dn9_slot: &mut f64,
    ) {
        let mut var_afac: f64 = *var_afac_slot;
        let mut var_afac_dn0: f64 = *var_afac_dn0_slot;
        let mut var_afac_dn1: f64 = *var_afac_dn1_slot;
        let mut var_afac_dn10: f64 = *var_afac_dn10_slot;
        let mut var_afac_dn11: f64 = *var_afac_dn11_slot;
        let mut var_afac_dn12: f64 = *var_afac_dn12_slot;
        let mut var_afac_dn13: f64 = *var_afac_dn13_slot;
        let mut var_afac_dn2: f64 = *var_afac_dn2_slot;
        let mut var_afac_dn3: f64 = *var_afac_dn3_slot;
        let mut var_afac_dn4: f64 = *var_afac_dn4_slot;
        let mut var_afac_dn5: f64 = *var_afac_dn5_slot;
        let mut var_afac_dn6: f64 = *var_afac_dn6_slot;
        let mut var_afac_dn7: f64 = *var_afac_dn7_slot;
        let mut var_afac_dn8: f64 = *var_afac_dn8_slot;
        let mut var_afac_dn9: f64 = *var_afac_dn9_slot;
        let mut var_avalf: f64 = *var_avalf_slot;
        let mut var_avalf_dn0: f64 = *var_avalf_dn0_slot;
        let mut var_avalf_dn1: f64 = *var_avalf_dn1_slot;
        let mut var_avalf_dn10: f64 = *var_avalf_dn10_slot;
        let mut var_avalf_dn11: f64 = *var_avalf_dn11_slot;
        let mut var_avalf_dn12: f64 = *var_avalf_dn12_slot;
        let mut var_avalf_dn13: f64 = *var_avalf_dn13_slot;
        let mut var_avalf_dn2: f64 = *var_avalf_dn2_slot;
        let mut var_avalf_dn3: f64 = *var_avalf_dn3_slot;
        let mut var_avalf_dn4: f64 = *var_avalf_dn4_slot;
        let mut var_avalf_dn5: f64 = *var_avalf_dn5_slot;
        let mut var_avalf_dn6: f64 = *var_avalf_dn6_slot;
        let mut var_avalf_dn7: f64 = *var_avalf_dn7_slot;
        let mut var_avalf_dn8: f64 = *var_avalf_dn8_slot;
        let mut var_avalf_dn9: f64 = *var_avalf_dn9_slot;
        let mut var_expi: f64 = *var_expi_slot;
        let mut var_expi__blk127: f64 = *var_expi__blk127_slot;
        let mut var_expi__blk127_dn0: f64 = *var_expi__blk127_dn0_slot;
        let mut var_expi__blk127_dn1: f64 = *var_expi__blk127_dn1_slot;
        let mut var_expi__blk127_dn10: f64 = *var_expi__blk127_dn10_slot;
        let mut var_expi__blk127_dn11: f64 = *var_expi__blk127_dn11_slot;
        let mut var_expi__blk127_dn12: f64 = *var_expi__blk127_dn12_slot;
        let mut var_expi__blk127_dn13: f64 = *var_expi__blk127_dn13_slot;
        let mut var_expi__blk127_dn2: f64 = *var_expi__blk127_dn2_slot;
        let mut var_expi__blk127_dn3: f64 = *var_expi__blk127_dn3_slot;
        let mut var_expi__blk127_dn4: f64 = *var_expi__blk127_dn4_slot;
        let mut var_expi__blk127_dn5: f64 = *var_expi__blk127_dn5_slot;
        let mut var_expi__blk127_dn6: f64 = *var_expi__blk127_dn6_slot;
        let mut var_expi__blk127_dn7: f64 = *var_expi__blk127_dn7_slot;
        let mut var_expi__blk127_dn8: f64 = *var_expi__blk127_dn8_slot;
        let mut var_expi__blk127_dn9: f64 = *var_expi__blk127_dn9_slot;
        let mut var_expi_dn0: f64 = *var_expi_dn0_slot;
        let mut var_expi_dn1: f64 = *var_expi_dn1_slot;
        let mut var_expi_dn10: f64 = *var_expi_dn10_slot;
        let mut var_expi_dn11: f64 = *var_expi_dn11_slot;
        let mut var_expi_dn12: f64 = *var_expi_dn12_slot;
        let mut var_expi_dn13: f64 = *var_expi_dn13_slot;
        let mut var_expi_dn2: f64 = *var_expi_dn2_slot;
        let mut var_expi_dn3: f64 = *var_expi_dn3_slot;
        let mut var_expi_dn4: f64 = *var_expi_dn4_slot;
        let mut var_expi_dn5: f64 = *var_expi_dn5_slot;
        let mut var_expi_dn6: f64 = *var_expi_dn6_slot;
        let mut var_expi_dn7: f64 = *var_expi_dn7_slot;
        let mut var_expi_dn8: f64 = *var_expi_dn8_slot;
        let mut var_expi_dn9: f64 = *var_expi_dn9_slot;
        let mut var_expl__blk128: f64 = *var_expl__blk128_slot;
        let mut var_guard123: f64 = *var_guard123_slot;
        let mut var_guard129: f64 = *var_guard129_slot;
        let mut var_guard130: f64 = *var_guard130_slot;
        let mut var_guard131: f64 = *var_guard131_slot;
        let mut var_guard132: f64 = *var_guard132_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_ibc: f64 = *var_ibc_slot;
        let mut var_ibc_dn0: f64 = *var_ibc_dn0_slot;
        let mut var_ibc_dn1: f64 = *var_ibc_dn1_slot;
        let mut var_ibc_dn10: f64 = *var_ibc_dn10_slot;
        let mut var_ibc_dn11: f64 = *var_ibc_dn11_slot;
        let mut var_ibc_dn12: f64 = *var_ibc_dn12_slot;
        let mut var_ibc_dn13: f64 = *var_ibc_dn13_slot;
        let mut var_ibc_dn2: f64 = *var_ibc_dn2_slot;
        let mut var_ibc_dn3: f64 = *var_ibc_dn3_slot;
        let mut var_ibc_dn4: f64 = *var_ibc_dn4_slot;
        let mut var_ibc_dn5: f64 = *var_ibc_dn5_slot;
        let mut var_ibc_dn6: f64 = *var_ibc_dn6_slot;
        let mut var_ibc_dn7: f64 = *var_ibc_dn7_slot;
        let mut var_ibc_dn8: f64 = *var_ibc_dn8_slot;
        let mut var_ibc_dn9: f64 = *var_ibc_dn9_slot;
        let mut var_ibk: f64 = *var_ibk_slot;
        let mut var_ibk_dn0: f64 = *var_ibk_dn0_slot;
        let mut var_ibk_dn1: f64 = *var_ibk_dn1_slot;
        let mut var_ibk_dn10: f64 = *var_ibk_dn10_slot;
        let mut var_ibk_dn11: f64 = *var_ibk_dn11_slot;
        let mut var_ibk_dn12: f64 = *var_ibk_dn12_slot;
        let mut var_ibk_dn13: f64 = *var_ibk_dn13_slot;
        let mut var_ibk_dn2: f64 = *var_ibk_dn2_slot;
        let mut var_ibk_dn3: f64 = *var_ibk_dn3_slot;
        let mut var_ibk_dn4: f64 = *var_ibk_dn4_slot;
        let mut var_ibk_dn5: f64 = *var_ibk_dn5_slot;
        let mut var_ibk_dn6: f64 = *var_ibk_dn6_slot;
        let mut var_ibk_dn7: f64 = *var_ibk_dn7_slot;
        let mut var_ibk_dn8: f64 = *var_ibk_dn8_slot;
        let mut var_ibk_dn9: f64 = *var_ibk_dn9_slot;
        let mut var_igcx: f64 = *var_igcx_slot;
        let mut var_igcx_dn0: f64 = *var_igcx_dn0_slot;
        let mut var_igcx_dn1: f64 = *var_igcx_dn1_slot;
        let mut var_igcx_dn10: f64 = *var_igcx_dn10_slot;
        let mut var_igcx_dn11: f64 = *var_igcx_dn11_slot;
        let mut var_igcx_dn12: f64 = *var_igcx_dn12_slot;
        let mut var_igcx_dn13: f64 = *var_igcx_dn13_slot;
        let mut var_igcx_dn2: f64 = *var_igcx_dn2_slot;
        let mut var_igcx_dn3: f64 = *var_igcx_dn3_slot;
        let mut var_igcx_dn4: f64 = *var_igcx_dn4_slot;
        let mut var_igcx_dn5: f64 = *var_igcx_dn5_slot;
        let mut var_igcx_dn6: f64 = *var_igcx_dn6_slot;
        let mut var_igcx_dn7: f64 = *var_igcx_dn7_slot;
        let mut var_igcx_dn8: f64 = *var_igcx_dn8_slot;
        let mut var_igcx_dn9: f64 = *var_igcx_dn9_slot;
        let mut var_iibk: f64 = *var_iibk_slot;
        let mut var_iibk_dn0: f64 = *var_iibk_dn0_slot;
        let mut var_iibk_dn1: f64 = *var_iibk_dn1_slot;
        let mut var_iibk_dn10: f64 = *var_iibk_dn10_slot;
        let mut var_iibk_dn11: f64 = *var_iibk_dn11_slot;
        let mut var_iibk_dn12: f64 = *var_iibk_dn12_slot;
        let mut var_iibk_dn13: f64 = *var_iibk_dn13_slot;
        let mut var_iibk_dn2: f64 = *var_iibk_dn2_slot;
        let mut var_iibk_dn3: f64 = *var_iibk_dn3_slot;
        let mut var_iibk_dn4: f64 = *var_iibk_dn4_slot;
        let mut var_iibk_dn5: f64 = *var_iibk_dn5_slot;
        let mut var_iibk_dn6: f64 = *var_iibk_dn6_slot;
        let mut var_iibk_dn7: f64 = *var_iibk_dn7_slot;
        let mut var_iibk_dn8: f64 = *var_iibk_dn8_slot;
        let mut var_iibk_dn9: f64 = *var_iibk_dn9_slot;
        let mut var_mac1__blk126: f64 = *var_mac1__blk126_slot;
        let mut var_mac1__blk126_dn0: f64 = *var_mac1__blk126_dn0_slot;
        let mut var_mac1__blk126_dn1: f64 = *var_mac1__blk126_dn1_slot;
        let mut var_mac1__blk126_dn10: f64 = *var_mac1__blk126_dn10_slot;
        let mut var_mac1__blk126_dn11: f64 = *var_mac1__blk126_dn11_slot;
        let mut var_mac1__blk126_dn12: f64 = *var_mac1__blk126_dn12_slot;
        let mut var_mac1__blk126_dn13: f64 = *var_mac1__blk126_dn13_slot;
        let mut var_mac1__blk126_dn2: f64 = *var_mac1__blk126_dn2_slot;
        let mut var_mac1__blk126_dn3: f64 = *var_mac1__blk126_dn3_slot;
        let mut var_mac1__blk126_dn4: f64 = *var_mac1__blk126_dn4_slot;
        let mut var_mac1__blk126_dn5: f64 = *var_mac1__blk126_dn5_slot;
        let mut var_mac1__blk126_dn6: f64 = *var_mac1__blk126_dn6_slot;
        let mut var_mac1__blk126_dn7: f64 = *var_mac1__blk126_dn7_slot;
        let mut var_mac1__blk126_dn8: f64 = *var_mac1__blk126_dn8_slot;
        let mut var_mac1__blk126_dn9: f64 = *var_mac1__blk126_dn9_slot;
        let mut var_vcbfac: f64 = *var_vcbfac_slot;
        let mut var_vcbfac_dn0: f64 = *var_vcbfac_dn0_slot;
        let mut var_vcbfac_dn1: f64 = *var_vcbfac_dn1_slot;
        let mut var_vcbfac_dn10: f64 = *var_vcbfac_dn10_slot;
        let mut var_vcbfac_dn11: f64 = *var_vcbfac_dn11_slot;
        let mut var_vcbfac_dn12: f64 = *var_vcbfac_dn12_slot;
        let mut var_vcbfac_dn13: f64 = *var_vcbfac_dn13_slot;
        let mut var_vcbfac_dn2: f64 = *var_vcbfac_dn2_slot;
        let mut var_vcbfac_dn3: f64 = *var_vcbfac_dn3_slot;
        let mut var_vcbfac_dn4: f64 = *var_vcbfac_dn4_slot;
        let mut var_vcbfac_dn5: f64 = *var_vcbfac_dn5_slot;
        let mut var_vcbfac_dn6: f64 = *var_vcbfac_dn6_slot;
        let mut var_vcbfac_dn7: f64 = *var_vcbfac_dn7_slot;
        let mut var_vcbfac_dn8: f64 = *var_vcbfac_dn8_slot;
        let mut var_vcbfac_dn9: f64 = *var_vcbfac_dn9_slot;
        let mut var_vl__blk125: f64 = *var_vl__blk125_slot;
        let mut var_vl__blk125_dn0: f64 = *var_vl__blk125_dn0_slot;
        let mut var_vl__blk125_dn1: f64 = *var_vl__blk125_dn1_slot;
        let mut var_vl__blk125_dn10: f64 = *var_vl__blk125_dn10_slot;
        let mut var_vl__blk125_dn11: f64 = *var_vl__blk125_dn11_slot;
        let mut var_vl__blk125_dn12: f64 = *var_vl__blk125_dn12_slot;
        let mut var_vl__blk125_dn13: f64 = *var_vl__blk125_dn13_slot;
        let mut var_vl__blk125_dn2: f64 = *var_vl__blk125_dn2_slot;
        let mut var_vl__blk125_dn3: f64 = *var_vl__blk125_dn3_slot;
        let mut var_vl__blk125_dn4: f64 = *var_vl__blk125_dn4_slot;
        let mut var_vl__blk125_dn5: f64 = *var_vl__blk125_dn5_slot;
        let mut var_vl__blk125_dn6: f64 = *var_vl__blk125_dn6_slot;
        let mut var_vl__blk125_dn7: f64 = *var_vl__blk125_dn7_slot;
        let mut var_vl__blk125_dn8: f64 = *var_vl__blk125_dn8_slot;
        let mut var_vl__blk125_dn9: f64 = *var_vl__blk125_dn9_slot;
        let mut var_vminm__blk124: f64 = *var_vminm__blk124_slot;
        let mut var_vminm__blk124_dn0: f64 = *var_vminm__blk124_dn0_slot;
        let mut var_vminm__blk124_dn1: f64 = *var_vminm__blk124_dn1_slot;
        let mut var_vminm__blk124_dn10: f64 = *var_vminm__blk124_dn10_slot;
        let mut var_vminm__blk124_dn11: f64 = *var_vminm__blk124_dn11_slot;
        let mut var_vminm__blk124_dn12: f64 = *var_vminm__blk124_dn12_slot;
        let mut var_vminm__blk124_dn13: f64 = *var_vminm__blk124_dn13_slot;
        let mut var_vminm__blk124_dn2: f64 = *var_vminm__blk124_dn2_slot;
        let mut var_vminm__blk124_dn3: f64 = *var_vminm__blk124_dn3_slot;
        let mut var_vminm__blk124_dn4: f64 = *var_vminm__blk124_dn4_slot;
        let mut var_vminm__blk124_dn5: f64 = *var_vminm__blk124_dn5_slot;
        let mut var_vminm__blk124_dn6: f64 = *var_vminm__blk124_dn6_slot;
        let mut var_vminm__blk124_dn7: f64 = *var_vminm__blk124_dn7_slot;
        let mut var_vminm__blk124_dn8: f64 = *var_vminm__blk124_dn8_slot;
        let mut var_vminm__blk124_dn9: f64 = *var_vminm__blk124_dn9_slot;

        let assign4140_e4559: f64 = if p.p85 > 0.0 { 1.0 } else { 0.0 };
        var_guard123 = assign4140_e4559;

        let (assign4150_e4573, assign4150_e4573_d_n0, assign4150_e4573_d_n1, assign4150_e4573_d_n2, assign4150_e4573_d_n3, assign4150_e4573_d_n4, assign4150_e4573_d_n5, assign4150_e4573_d_n6, assign4150_e4573_d_n7, assign4150_e4573_d_n8, assign4150_e4573_d_n9, assign4150_e4573_d_n10, assign4150_e4573_d_n11, assign4150_e4573_d_n12, assign4150_e4573_d_n13,) = {
    if (var_guard123 != 0.0) {
        let assign4150_e4564: f64 = (var_avcx2_t + 1.0);
        let assign4150_e4565: f64 = (0.02 * assign4150_e4564);
        let assign4150_e4569: f64 = (1.01 - p.p87);
        let assign4150_e4570: f64 = (1.0 / assign4150_e4569);
        let assign4150_e4571: f64 = (assign4150_e4565).powf(assign4150_e4570);
        (assign4150_e4571, if 0.0 == 0.0 && ((assign4150_e4570) as f64).is_finite() && ((assign4150_e4570) as f64).fract() == 0.0 { if assign4150_e4570 == 0.0 { 0.0 } else { (assign4150_e4570 * ((assign4150_e4565).powf(assign4150_e4570 - 1.0) * (0.02 * var_avcx2_t_dn0))) } } else { (assign4150_e4571 * (assign4150_e4570 * ((0.02 * var_avcx2_t_dn0) / assign4150_e4565))) }, if 0.0 == 0.0 && ((assign4150_e4570) as f64).is_finite() && ((assign4150_e4570) as f64).fract() == 0.0 { if assign4150_e4570 == 0.0 { 0.0 } else { (assign4150_e4570 * ((assign4150_e4565).powf(assign4150_e4570 - 1.0) * (0.02 * var_avcx2_t_dn1))) } } else { (assign4150_e4571 * (assign4150_e4570 * ((0.02 * var_avcx2_t_dn1) / assign4150_e4565))) }, if 0.0 == 0.0 && ((assign4150_e4570) as f64).is_finite() && ((assign4150_e4570) as f64).fract() == 0.0 { if assign4150_e4570 == 0.0 { 0.0 } else { (assign4150_e4570 * ((assign4150_e4565).powf(assign4150_e4570 - 1.0) * (0.02 * var_avcx2_t_dn2))) } } else { (assign4150_e4571 * (assign4150_e4570 * ((0.02 * var_avcx2_t_dn2) / assign4150_e4565))) }, if 0.0 == 0.0 && ((assign4150_e4570) as f64).is_finite() && ((assign4150_e4570) as f64).fract() == 0.0 { if assign4150_e4570 == 0.0 { 0.0 } else { (assign4150_e4570 * ((assign4150_e4565).powf(assign4150_e4570 - 1.0) * (0.02 * var_avcx2_t_dn3))) } } else { (assign4150_e4571 * (assign4150_e4570 * ((0.02 * var_avcx2_t_dn3) / assign4150_e4565))) }, if 0.0 == 0.0 && ((assign4150_e4570) as f64).is_finite() && ((assign4150_e4570) as f64).fract() == 0.0 { if assign4150_e4570 == 0.0 { 0.0 } else { (assign4150_e4570 * ((assign4150_e4565).powf(assign4150_e4570 - 1.0) * (0.02 * var_avcx2_t_dn4))) } } else { (assign4150_e4571 * (assign4150_e4570 * ((0.02 * var_avcx2_t_dn4) / assign4150_e4565))) }, if 0.0 == 0.0 && ((assign4150_e4570) as f64).is_finite() && ((assign4150_e4570) as f64).fract() == 0.0 { if assign4150_e4570 == 0.0 { 0.0 } else { (assign4150_e4570 * ((assign4150_e4565).powf(assign4150_e4570 - 1.0) * (0.02 * var_avcx2_t_dn5))) } } else { (assign4150_e4571 * (assign4150_e4570 * ((0.02 * var_avcx2_t_dn5) / assign4150_e4565))) }, if 0.0 == 0.0 && ((assign4150_e4570) as f64).is_finite() && ((assign4150_e4570) as f64).fract() == 0.0 { if assign4150_e4570 == 0.0 { 0.0 } else { (assign4150_e4570 * ((assign4150_e4565).powf(assign4150_e4570 - 1.0) * (0.02 * var_avcx2_t_dn6))) } } else { (assign4150_e4571 * (assign4150_e4570 * ((0.02 * var_avcx2_t_dn6) / assign4150_e4565))) }, if 0.0 == 0.0 && ((assign4150_e4570) as f64).is_finite() && ((assign4150_e4570) as f64).fract() == 0.0 { if assign4150_e4570 == 0.0 { 0.0 } else { (assign4150_e4570 * ((assign4150_e4565).powf(assign4150_e4570 - 1.0) * (0.02 * var_avcx2_t_dn7))) } } else { (assign4150_e4571 * (assign4150_e4570 * ((0.02 * var_avcx2_t_dn7) / assign4150_e4565))) }, if 0.0 == 0.0 && ((assign4150_e4570) as f64).is_finite() && ((assign4150_e4570) as f64).fract() == 0.0 { if assign4150_e4570 == 0.0 { 0.0 } else { (assign4150_e4570 * ((assign4150_e4565).powf(assign4150_e4570 - 1.0) * (0.02 * var_avcx2_t_dn8))) } } else { (assign4150_e4571 * (assign4150_e4570 * ((0.02 * var_avcx2_t_dn8) / assign4150_e4565))) }, if 0.0 == 0.0 && ((assign4150_e4570) as f64).is_finite() && ((assign4150_e4570) as f64).fract() == 0.0 { if assign4150_e4570 == 0.0 { 0.0 } else { (assign4150_e4570 * ((assign4150_e4565).powf(assign4150_e4570 - 1.0) * (0.02 * var_avcx2_t_dn9))) } } else { (assign4150_e4571 * (assign4150_e4570 * ((0.02 * var_avcx2_t_dn9) / assign4150_e4565))) }, if 0.0 == 0.0 && ((assign4150_e4570) as f64).is_finite() && ((assign4150_e4570) as f64).fract() == 0.0 { if assign4150_e4570 == 0.0 { 0.0 } else { (assign4150_e4570 * ((assign4150_e4565).powf(assign4150_e4570 - 1.0) * (0.02 * var_avcx2_t_dn10))) } } else { (assign4150_e4571 * (assign4150_e4570 * ((0.02 * var_avcx2_t_dn10) / assign4150_e4565))) }, if 0.0 == 0.0 && ((assign4150_e4570) as f64).is_finite() && ((assign4150_e4570) as f64).fract() == 0.0 { if assign4150_e4570 == 0.0 { 0.0 } else { (assign4150_e4570 * ((assign4150_e4565).powf(assign4150_e4570 - 1.0) * (0.02 * var_avcx2_t_dn11))) } } else { (assign4150_e4571 * (assign4150_e4570 * ((0.02 * var_avcx2_t_dn11) / assign4150_e4565))) }, if 0.0 == 0.0 && ((assign4150_e4570) as f64).is_finite() && ((assign4150_e4570) as f64).fract() == 0.0 { if assign4150_e4570 == 0.0 { 0.0 } else { (assign4150_e4570 * ((assign4150_e4565).powf(assign4150_e4570 - 1.0) * (0.02 * var_avcx2_t_dn12))) } } else { (assign4150_e4571 * (assign4150_e4570 * ((0.02 * var_avcx2_t_dn12) / assign4150_e4565))) }, if 0.0 == 0.0 && ((assign4150_e4570) as f64).is_finite() && ((assign4150_e4570) as f64).fract() == 0.0 { if assign4150_e4570 == 0.0 { 0.0 } else { (assign4150_e4570 * ((assign4150_e4565).powf(assign4150_e4570 - 1.0) * (0.02 * var_avcx2_t_dn13))) } } else { (assign4150_e4571 * (assign4150_e4570 * ((0.02 * var_avcx2_t_dn13) / assign4150_e4565))) },)
    } else {
        (var_vminm__blk124, var_vminm__blk124_dn0, var_vminm__blk124_dn1, var_vminm__blk124_dn2, var_vminm__blk124_dn3, var_vminm__blk124_dn4, var_vminm__blk124_dn5, var_vminm__blk124_dn6, var_vminm__blk124_dn7, var_vminm__blk124_dn8, var_vminm__blk124_dn9, var_vminm__blk124_dn10, var_vminm__blk124_dn11, var_vminm__blk124_dn12, var_vminm__blk124_dn13,)
    }
};
        var_vminm__blk124 = assign4150_e4573;
        var_vminm__blk124_dn0 = assign4150_e4573_d_n0;
        var_vminm__blk124_dn1 = assign4150_e4573_d_n1;
        var_vminm__blk124_dn2 = assign4150_e4573_d_n2;
        var_vminm__blk124_dn3 = assign4150_e4573_d_n3;
        var_vminm__blk124_dn4 = assign4150_e4573_d_n4;
        var_vminm__blk124_dn5 = assign4150_e4573_d_n5;
        var_vminm__blk124_dn6 = assign4150_e4573_d_n6;
        var_vminm__blk124_dn7 = assign4150_e4573_d_n7;
        var_vminm__blk124_dn8 = assign4150_e4573_d_n8;
        var_vminm__blk124_dn9 = assign4150_e4573_d_n9;
        var_vminm__blk124_dn10 = assign4150_e4573_d_n10;
        var_vminm__blk124_dn11 = assign4150_e4573_d_n11;
        var_vminm__blk124_dn12 = assign4150_e4573_d_n12;
        var_vminm__blk124_dn13 = assign4150_e4573_d_n13;

        let (assign4160_e4600, assign4160_e4600_d_n0, assign4160_e4600_d_n1, assign4160_e4600_d_n2, assign4160_e4600_d_n3, assign4160_e4600_d_n4, assign4160_e4600_d_n5, assign4160_e4600_d_n6, assign4160_e4600_d_n7, assign4160_e4600_d_n8, assign4160_e4600_d_n9, assign4160_e4600_d_n10, assign4160_e4600_d_n11, assign4160_e4600_d_n12, assign4160_e4600_d_n13,) = {
    if (var_guard123 != 0.0) {
        let assign4160_e4578: f64 = (-var_vbxcx);
        let assign4160_e4580: f64 = (assign4160_e4578 - var_vminm__blk124);
        let assign4160_e4583: f64 = (-var_vbxcx);
        let assign4160_e4585: f64 = (assign4160_e4583 - var_vminm__blk124);
        let assign4160_e4586: f64 = (assign4160_e4580 * assign4160_e4585);
        let assign4160_e4588: f64 = (assign4160_e4586 + 0.01);
        let assign4160_e4589: f64 = (assign4160_e4588).sqrt();
        let assign4160_e4592: f64 = (-var_vbxcx);
        let assign4160_e4594: f64 = (assign4160_e4592 - var_vminm__blk124);
        let assign4160_e4595: f64 = (assign4160_e4589 + assign4160_e4594);
        let assign4160_e4596: f64 = (0.5 * assign4160_e4595);
        let assign4160_e4598: f64 = (assign4160_e4596 + var_vminm__blk124);
        (assign4160_e4598, ((0.5 * ((((((-var_vbxcx_dn0) - var_vminm__blk124_dn0) * assign4160_e4585) + (assign4160_e4580 * ((-var_vbxcx_dn0) - var_vminm__blk124_dn0))) / (2.0 * assign4160_e4589)) + ((-var_vbxcx_dn0) - var_vminm__blk124_dn0))) + var_vminm__blk124_dn0), ((0.5 * ((((((-var_vbxcx_dn1) - var_vminm__blk124_dn1) * assign4160_e4585) + (assign4160_e4580 * ((-var_vbxcx_dn1) - var_vminm__blk124_dn1))) / (2.0 * assign4160_e4589)) + ((-var_vbxcx_dn1) - var_vminm__blk124_dn1))) + var_vminm__blk124_dn1), ((0.5 * ((((((-var_vbxcx_dn2) - var_vminm__blk124_dn2) * assign4160_e4585) + (assign4160_e4580 * ((-var_vbxcx_dn2) - var_vminm__blk124_dn2))) / (2.0 * assign4160_e4589)) + ((-var_vbxcx_dn2) - var_vminm__blk124_dn2))) + var_vminm__blk124_dn2), ((0.5 * ((((((-var_vbxcx_dn3) - var_vminm__blk124_dn3) * assign4160_e4585) + (assign4160_e4580 * ((-var_vbxcx_dn3) - var_vminm__blk124_dn3))) / (2.0 * assign4160_e4589)) + ((-var_vbxcx_dn3) - var_vminm__blk124_dn3))) + var_vminm__blk124_dn3), ((0.5 * ((((((-var_vbxcx_dn4) - var_vminm__blk124_dn4) * assign4160_e4585) + (assign4160_e4580 * ((-var_vbxcx_dn4) - var_vminm__blk124_dn4))) / (2.0 * assign4160_e4589)) + ((-var_vbxcx_dn4) - var_vminm__blk124_dn4))) + var_vminm__blk124_dn4), ((0.5 * ((((((-var_vbxcx_dn5) - var_vminm__blk124_dn5) * assign4160_e4585) + (assign4160_e4580 * ((-var_vbxcx_dn5) - var_vminm__blk124_dn5))) / (2.0 * assign4160_e4589)) + ((-var_vbxcx_dn5) - var_vminm__blk124_dn5))) + var_vminm__blk124_dn5), ((0.5 * ((((((-var_vbxcx_dn6) - var_vminm__blk124_dn6) * assign4160_e4585) + (assign4160_e4580 * ((-var_vbxcx_dn6) - var_vminm__blk124_dn6))) / (2.0 * assign4160_e4589)) + ((-var_vbxcx_dn6) - var_vminm__blk124_dn6))) + var_vminm__blk124_dn6), ((0.5 * ((((((-var_vbxcx_dn7) - var_vminm__blk124_dn7) * assign4160_e4585) + (assign4160_e4580 * ((-var_vbxcx_dn7) - var_vminm__blk124_dn7))) / (2.0 * assign4160_e4589)) + ((-var_vbxcx_dn7) - var_vminm__blk124_dn7))) + var_vminm__blk124_dn7), ((0.5 * ((((((-var_vbxcx_dn8) - var_vminm__blk124_dn8) * assign4160_e4585) + (assign4160_e4580 * ((-var_vbxcx_dn8) - var_vminm__blk124_dn8))) / (2.0 * assign4160_e4589)) + ((-var_vbxcx_dn8) - var_vminm__blk124_dn8))) + var_vminm__blk124_dn8), ((0.5 * ((((((-var_vbxcx_dn9) - var_vminm__blk124_dn9) * assign4160_e4585) + (assign4160_e4580 * ((-var_vbxcx_dn9) - var_vminm__blk124_dn9))) / (2.0 * assign4160_e4589)) + ((-var_vbxcx_dn9) - var_vminm__blk124_dn9))) + var_vminm__blk124_dn9), ((0.5 * ((((((-var_vbxcx_dn10) - var_vminm__blk124_dn10) * assign4160_e4585) + (assign4160_e4580 * ((-var_vbxcx_dn10) - var_vminm__blk124_dn10))) / (2.0 * assign4160_e4589)) + ((-var_vbxcx_dn10) - var_vminm__blk124_dn10))) + var_vminm__blk124_dn10), ((0.5 * ((((((-var_vbxcx_dn11) - var_vminm__blk124_dn11) * assign4160_e4585) + (assign4160_e4580 * ((-var_vbxcx_dn11) - var_vminm__blk124_dn11))) / (2.0 * assign4160_e4589)) + ((-var_vbxcx_dn11) - var_vminm__blk124_dn11))) + var_vminm__blk124_dn11), ((0.5 * ((((((-var_vbxcx_dn12) - var_vminm__blk124_dn12) * assign4160_e4585) + (assign4160_e4580 * ((-var_vbxcx_dn12) - var_vminm__blk124_dn12))) / (2.0 * assign4160_e4589)) + ((-var_vbxcx_dn12) - var_vminm__blk124_dn12))) + var_vminm__blk124_dn12), ((0.5 * ((((((-var_vbxcx_dn13) - var_vminm__blk124_dn13) * assign4160_e4585) + (assign4160_e4580 * ((-var_vbxcx_dn13) - var_vminm__blk124_dn13))) / (2.0 * assign4160_e4589)) + ((-var_vbxcx_dn13) - var_vminm__blk124_dn13))) + var_vminm__blk124_dn13),)
    } else {
        (var_vl__blk125, var_vl__blk125_dn0, var_vl__blk125_dn1, var_vl__blk125_dn2, var_vl__blk125_dn3, var_vl__blk125_dn4, var_vl__blk125_dn5, var_vl__blk125_dn6, var_vl__blk125_dn7, var_vl__blk125_dn8, var_vl__blk125_dn9, var_vl__blk125_dn10, var_vl__blk125_dn11, var_vl__blk125_dn12, var_vl__blk125_dn13,)
    }
};
        var_vl__blk125 = assign4160_e4600;
        var_vl__blk125_dn0 = assign4160_e4600_d_n0;
        var_vl__blk125_dn1 = assign4160_e4600_d_n1;
        var_vl__blk125_dn2 = assign4160_e4600_d_n2;
        var_vl__blk125_dn3 = assign4160_e4600_d_n3;
        var_vl__blk125_dn4 = assign4160_e4600_d_n4;
        var_vl__blk125_dn5 = assign4160_e4600_d_n5;
        var_vl__blk125_dn6 = assign4160_e4600_d_n6;
        var_vl__blk125_dn7 = assign4160_e4600_d_n7;
        var_vl__blk125_dn8 = assign4160_e4600_d_n8;
        var_vl__blk125_dn9 = assign4160_e4600_d_n9;
        var_vl__blk125_dn10 = assign4160_e4600_d_n10;
        var_vl__blk125_dn11 = assign4160_e4600_d_n11;
        var_vl__blk125_dn12 = assign4160_e4600_d_n12;
        var_vl__blk125_dn13 = assign4160_e4600_d_n13;

        let (assign4170_e4611, assign4170_e4611_d_n0, assign4170_e4611_d_n1, assign4170_e4611_d_n2, assign4170_e4611_d_n3, assign4170_e4611_d_n4, assign4170_e4611_d_n5, assign4170_e4611_d_n6, assign4170_e4611_d_n7, assign4170_e4611_d_n8, assign4170_e4611_d_n9, assign4170_e4611_d_n10, assign4170_e4611_d_n11, assign4170_e4611_d_n12, assign4170_e4611_d_n13,) = {
    if (var_guard123 != 0.0) {
        let assign4170_e4603: f64 = (-var_avcx2_t);
        let assign4170_e4607: f64 = (p.p87 - 1.0);
        let assign4170_e4608: f64 = (var_vl__blk125).powf(assign4170_e4607);
        let assign4170_e4609: f64 = (assign4170_e4603 * assign4170_e4608);
        (assign4170_e4609, (((-var_avcx2_t_dn0) * assign4170_e4608) + (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((var_vl__blk125).powf(assign4170_e4607 - 1.0) * var_vl__blk125_dn0)) } } else { (assign4170_e4608 * (assign4170_e4607 * (var_vl__blk125_dn0 / var_vl__blk125))) })), (((-var_avcx2_t_dn1) * assign4170_e4608) + (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((var_vl__blk125).powf(assign4170_e4607 - 1.0) * var_vl__blk125_dn1)) } } else { (assign4170_e4608 * (assign4170_e4607 * (var_vl__blk125_dn1 / var_vl__blk125))) })), (((-var_avcx2_t_dn2) * assign4170_e4608) + (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((var_vl__blk125).powf(assign4170_e4607 - 1.0) * var_vl__blk125_dn2)) } } else { (assign4170_e4608 * (assign4170_e4607 * (var_vl__blk125_dn2 / var_vl__blk125))) })), (((-var_avcx2_t_dn3) * assign4170_e4608) + (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((var_vl__blk125).powf(assign4170_e4607 - 1.0) * var_vl__blk125_dn3)) } } else { (assign4170_e4608 * (assign4170_e4607 * (var_vl__blk125_dn3 / var_vl__blk125))) })), (((-var_avcx2_t_dn4) * assign4170_e4608) + (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((var_vl__blk125).powf(assign4170_e4607 - 1.0) * var_vl__blk125_dn4)) } } else { (assign4170_e4608 * (assign4170_e4607 * (var_vl__blk125_dn4 / var_vl__blk125))) })), (((-var_avcx2_t_dn5) * assign4170_e4608) + (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((var_vl__blk125).powf(assign4170_e4607 - 1.0) * var_vl__blk125_dn5)) } } else { (assign4170_e4608 * (assign4170_e4607 * (var_vl__blk125_dn5 / var_vl__blk125))) })), (((-var_avcx2_t_dn6) * assign4170_e4608) + (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((var_vl__blk125).powf(assign4170_e4607 - 1.0) * var_vl__blk125_dn6)) } } else { (assign4170_e4608 * (assign4170_e4607 * (var_vl__blk125_dn6 / var_vl__blk125))) })), (((-var_avcx2_t_dn7) * assign4170_e4608) + (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((var_vl__blk125).powf(assign4170_e4607 - 1.0) * var_vl__blk125_dn7)) } } else { (assign4170_e4608 * (assign4170_e4607 * (var_vl__blk125_dn7 / var_vl__blk125))) })), (((-var_avcx2_t_dn8) * assign4170_e4608) + (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((var_vl__blk125).powf(assign4170_e4607 - 1.0) * var_vl__blk125_dn8)) } } else { (assign4170_e4608 * (assign4170_e4607 * (var_vl__blk125_dn8 / var_vl__blk125))) })), (((-var_avcx2_t_dn9) * assign4170_e4608) + (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((var_vl__blk125).powf(assign4170_e4607 - 1.0) * var_vl__blk125_dn9)) } } else { (assign4170_e4608 * (assign4170_e4607 * (var_vl__blk125_dn9 / var_vl__blk125))) })), (((-var_avcx2_t_dn10) * assign4170_e4608) + (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((var_vl__blk125).powf(assign4170_e4607 - 1.0) * var_vl__blk125_dn10)) } } else { (assign4170_e4608 * (assign4170_e4607 * (var_vl__blk125_dn10 / var_vl__blk125))) })), (((-var_avcx2_t_dn11) * assign4170_e4608) + (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((var_vl__blk125).powf(assign4170_e4607 - 1.0) * var_vl__blk125_dn11)) } } else { (assign4170_e4608 * (assign4170_e4607 * (var_vl__blk125_dn11 / var_vl__blk125))) })), (((-var_avcx2_t_dn12) * assign4170_e4608) + (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((var_vl__blk125).powf(assign4170_e4607 - 1.0) * var_vl__blk125_dn12)) } } else { (assign4170_e4608 * (assign4170_e4607 * (var_vl__blk125_dn12 / var_vl__blk125))) })), (((-var_avcx2_t_dn13) * assign4170_e4608) + (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((var_vl__blk125).powf(assign4170_e4607 - 1.0) * var_vl__blk125_dn13)) } } else { (assign4170_e4608 * (assign4170_e4607 * (var_vl__blk125_dn13 / var_vl__blk125))) })),)
    } else {
        (var_mac1__blk126, var_mac1__blk126_dn0, var_mac1__blk126_dn1, var_mac1__blk126_dn2, var_mac1__blk126_dn3, var_mac1__blk126_dn4, var_mac1__blk126_dn5, var_mac1__blk126_dn6, var_mac1__blk126_dn7, var_mac1__blk126_dn8, var_mac1__blk126_dn9, var_mac1__blk126_dn10, var_mac1__blk126_dn11, var_mac1__blk126_dn12, var_mac1__blk126_dn13,)
    }
};
        var_mac1__blk126 = assign4170_e4611;
        var_mac1__blk126_dn0 = assign4170_e4611_d_n0;
        var_mac1__blk126_dn1 = assign4170_e4611_d_n1;
        var_mac1__blk126_dn2 = assign4170_e4611_d_n2;
        var_mac1__blk126_dn3 = assign4170_e4611_d_n3;
        var_mac1__blk126_dn4 = assign4170_e4611_d_n4;
        var_mac1__blk126_dn5 = assign4170_e4611_d_n5;
        var_mac1__blk126_dn6 = assign4170_e4611_d_n6;
        var_mac1__blk126_dn7 = assign4170_e4611_d_n7;
        var_mac1__blk126_dn8 = assign4170_e4611_d_n8;
        var_mac1__blk126_dn9 = assign4170_e4611_d_n9;
        var_mac1__blk126_dn10 = assign4170_e4611_d_n10;
        var_mac1__blk126_dn11 = assign4170_e4611_d_n11;
        var_mac1__blk126_dn12 = assign4170_e4611_d_n12;
        var_mac1__blk126_dn13 = assign4170_e4611_d_n13;

        let assign4180_e4614: f64 = if var_mac1__blk126 < var_vmaxexp { 1.0 } else { 0.0 };
        var_guard129 = assign4180_e4614;

        let (assign4190_e4621, assign4190_e4621_d_n0, assign4190_e4621_d_n1, assign4190_e4621_d_n2, assign4190_e4621_d_n3, assign4190_e4621_d_n4, assign4190_e4621_d_n5, assign4190_e4621_d_n6, assign4190_e4621_d_n7, assign4190_e4621_d_n8, assign4190_e4621_d_n9, assign4190_e4621_d_n10, assign4190_e4621_d_n11, assign4190_e4621_d_n12, assign4190_e4621_d_n13,) = {
    if ((var_guard123 != 0.0) && (var_guard129 != 0.0)) {
        let assign4190_e4619: f64 = (var_mac1__blk126).exp();
        (assign4190_e4619, (assign4190_e4619 * var_mac1__blk126_dn0), (assign4190_e4619 * var_mac1__blk126_dn1), (assign4190_e4619 * var_mac1__blk126_dn2), (assign4190_e4619 * var_mac1__blk126_dn3), (assign4190_e4619 * var_mac1__blk126_dn4), (assign4190_e4619 * var_mac1__blk126_dn5), (assign4190_e4619 * var_mac1__blk126_dn6), (assign4190_e4619 * var_mac1__blk126_dn7), (assign4190_e4619 * var_mac1__blk126_dn8), (assign4190_e4619 * var_mac1__blk126_dn9), (assign4190_e4619 * var_mac1__blk126_dn10), (assign4190_e4619 * var_mac1__blk126_dn11), (assign4190_e4619 * var_mac1__blk126_dn12), (assign4190_e4619 * var_mac1__blk126_dn13),)
    } else {
        (var_expi__blk127, var_expi__blk127_dn0, var_expi__blk127_dn1, var_expi__blk127_dn2, var_expi__blk127_dn3, var_expi__blk127_dn4, var_expi__blk127_dn5, var_expi__blk127_dn6, var_expi__blk127_dn7, var_expi__blk127_dn8, var_expi__blk127_dn9, var_expi__blk127_dn10, var_expi__blk127_dn11, var_expi__blk127_dn12, var_expi__blk127_dn13,)
    }
};
        var_expi__blk127 = assign4190_e4621;
        var_expi__blk127_dn0 = assign4190_e4621_d_n0;
        var_expi__blk127_dn1 = assign4190_e4621_d_n1;
        var_expi__blk127_dn2 = assign4190_e4621_d_n2;
        var_expi__blk127_dn3 = assign4190_e4621_d_n3;
        var_expi__blk127_dn4 = assign4190_e4621_d_n4;
        var_expi__blk127_dn5 = assign4190_e4621_d_n5;
        var_expi__blk127_dn6 = assign4190_e4621_d_n6;
        var_expi__blk127_dn7 = assign4190_e4621_d_n7;
        var_expi__blk127_dn8 = assign4190_e4621_d_n8;
        var_expi__blk127_dn9 = assign4190_e4621_d_n9;
        var_expi__blk127_dn10 = assign4190_e4621_d_n10;
        var_expi__blk127_dn11 = assign4190_e4621_d_n11;
        var_expi__blk127_dn12 = assign4190_e4621_d_n12;
        var_expi__blk127_dn13 = assign4190_e4621_d_n13;

        let (assign4200_e4629,) = {
    if ((var_guard123 != 0.0) && (var_guard129 == 0.0)) {
        let assign4200_e4627: f64 = (var_vmaxexp).exp();
        (assign4200_e4627,)
    } else {
        (var_expl__blk128,)
    }
};
        var_expl__blk128 = assign4200_e4629;

        let (assign4210_e4642, assign4210_e4642_d_n0, assign4210_e4642_d_n1, assign4210_e4642_d_n2, assign4210_e4642_d_n3, assign4210_e4642_d_n4, assign4210_e4642_d_n5, assign4210_e4642_d_n6, assign4210_e4642_d_n7, assign4210_e4642_d_n8, assign4210_e4642_d_n9, assign4210_e4642_d_n10, assign4210_e4642_d_n11, assign4210_e4642_d_n12, assign4210_e4642_d_n13,) = {
    if ((var_guard123 != 0.0) && (var_guard129 == 0.0)) {
        let assign4210_e4638: f64 = (var_mac1__blk126 - var_vmaxexp);
        let assign4210_e4639: f64 = (1.0 + assign4210_e4638);
        let assign4210_e4640: f64 = (var_expl__blk128 * assign4210_e4639);
        (assign4210_e4640, (var_expl__blk128 * var_mac1__blk126_dn0), (var_expl__blk128 * var_mac1__blk126_dn1), (var_expl__blk128 * var_mac1__blk126_dn2), (var_expl__blk128 * var_mac1__blk126_dn3), (var_expl__blk128 * var_mac1__blk126_dn4), (var_expl__blk128 * var_mac1__blk126_dn5), (var_expl__blk128 * var_mac1__blk126_dn6), (var_expl__blk128 * var_mac1__blk126_dn7), (var_expl__blk128 * var_mac1__blk126_dn8), (var_expl__blk128 * var_mac1__blk126_dn9), (var_expl__blk128 * var_mac1__blk126_dn10), (var_expl__blk128 * var_mac1__blk126_dn11), (var_expl__blk128 * var_mac1__blk126_dn12), (var_expl__blk128 * var_mac1__blk126_dn13),)
    } else {
        (var_expi__blk127, var_expi__blk127_dn0, var_expi__blk127_dn1, var_expi__blk127_dn2, var_expi__blk127_dn3, var_expi__blk127_dn4, var_expi__blk127_dn5, var_expi__blk127_dn6, var_expi__blk127_dn7, var_expi__blk127_dn8, var_expi__blk127_dn9, var_expi__blk127_dn10, var_expi__blk127_dn11, var_expi__blk127_dn12, var_expi__blk127_dn13,)
    }
};
        var_expi__blk127 = assign4210_e4642;
        var_expi__blk127_dn0 = assign4210_e4642_d_n0;
        var_expi__blk127_dn1 = assign4210_e4642_d_n1;
        var_expi__blk127_dn2 = assign4210_e4642_d_n2;
        var_expi__blk127_dn3 = assign4210_e4642_d_n3;
        var_expi__blk127_dn4 = assign4210_e4642_d_n4;
        var_expi__blk127_dn5 = assign4210_e4642_d_n5;
        var_expi__blk127_dn6 = assign4210_e4642_d_n6;
        var_expi__blk127_dn7 = assign4210_e4642_d_n7;
        var_expi__blk127_dn8 = assign4210_e4642_d_n8;
        var_expi__blk127_dn9 = assign4210_e4642_d_n9;
        var_expi__blk127_dn10 = assign4210_e4642_d_n10;
        var_expi__blk127_dn11 = assign4210_e4642_d_n11;
        var_expi__blk127_dn12 = assign4210_e4642_d_n12;
        var_expi__blk127_dn13 = assign4210_e4642_d_n13;

        let (assign4220_e4650, assign4220_e4650_d_n0, assign4220_e4650_d_n1, assign4220_e4650_d_n2, assign4220_e4650_d_n3, assign4220_e4650_d_n4, assign4220_e4650_d_n5, assign4220_e4650_d_n6, assign4220_e4650_d_n7, assign4220_e4650_d_n8, assign4220_e4650_d_n9, assign4220_e4650_d_n10, assign4220_e4650_d_n11, assign4220_e4650_d_n12, assign4220_e4650_d_n13,) = {
    if (var_guard123 != 0.0) {
        let assign4220_e4646: f64 = (p.p85 * var_vl__blk125);
        let assign4220_e4648: f64 = (assign4220_e4646 * var_expi__blk127);
        (assign4220_e4648, (((p.p85 * var_vl__blk125_dn0) * var_expi__blk127) + (assign4220_e4646 * var_expi__blk127_dn0)), (((p.p85 * var_vl__blk125_dn1) * var_expi__blk127) + (assign4220_e4646 * var_expi__blk127_dn1)), (((p.p85 * var_vl__blk125_dn2) * var_expi__blk127) + (assign4220_e4646 * var_expi__blk127_dn2)), (((p.p85 * var_vl__blk125_dn3) * var_expi__blk127) + (assign4220_e4646 * var_expi__blk127_dn3)), (((p.p85 * var_vl__blk125_dn4) * var_expi__blk127) + (assign4220_e4646 * var_expi__blk127_dn4)), (((p.p85 * var_vl__blk125_dn5) * var_expi__blk127) + (assign4220_e4646 * var_expi__blk127_dn5)), (((p.p85 * var_vl__blk125_dn6) * var_expi__blk127) + (assign4220_e4646 * var_expi__blk127_dn6)), (((p.p85 * var_vl__blk125_dn7) * var_expi__blk127) + (assign4220_e4646 * var_expi__blk127_dn7)), (((p.p85 * var_vl__blk125_dn8) * var_expi__blk127) + (assign4220_e4646 * var_expi__blk127_dn8)), (((p.p85 * var_vl__blk125_dn9) * var_expi__blk127) + (assign4220_e4646 * var_expi__blk127_dn9)), (((p.p85 * var_vl__blk125_dn10) * var_expi__blk127) + (assign4220_e4646 * var_expi__blk127_dn10)), (((p.p85 * var_vl__blk125_dn11) * var_expi__blk127) + (assign4220_e4646 * var_expi__blk127_dn11)), (((p.p85 * var_vl__blk125_dn12) * var_expi__blk127) + (assign4220_e4646 * var_expi__blk127_dn12)), (((p.p85 * var_vl__blk125_dn13) * var_expi__blk127) + (assign4220_e4646 * var_expi__blk127_dn13)),)
    } else {
        (var_avalf, var_avalf_dn0, var_avalf_dn1, var_avalf_dn2, var_avalf_dn3, var_avalf_dn4, var_avalf_dn5, var_avalf_dn6, var_avalf_dn7, var_avalf_dn8, var_avalf_dn9, var_avalf_dn10, var_avalf_dn11, var_avalf_dn12, var_avalf_dn13,)
    }
};
        var_avalf = assign4220_e4650;
        var_avalf_dn0 = assign4220_e4650_d_n0;
        var_avalf_dn1 = assign4220_e4650_d_n1;
        var_avalf_dn2 = assign4220_e4650_d_n2;
        var_avalf_dn3 = assign4220_e4650_d_n3;
        var_avalf_dn4 = assign4220_e4650_d_n4;
        var_avalf_dn5 = assign4220_e4650_d_n5;
        var_avalf_dn6 = assign4220_e4650_d_n6;
        var_avalf_dn7 = assign4220_e4650_d_n7;
        var_avalf_dn8 = assign4220_e4650_d_n8;
        var_avalf_dn9 = assign4220_e4650_d_n9;
        var_avalf_dn10 = assign4220_e4650_d_n10;
        var_avalf_dn11 = assign4220_e4650_d_n11;
        var_avalf_dn12 = assign4220_e4650_d_n12;
        var_avalf_dn13 = assign4220_e4650_d_n13;

        let (assign4230_e4657, assign4230_e4657_d_n0, assign4230_e4657_d_n1, assign4230_e4657_d_n2, assign4230_e4657_d_n3, assign4230_e4657_d_n4, assign4230_e4657_d_n5, assign4230_e4657_d_n6, assign4230_e4657_d_n7, assign4230_e4657_d_n8, assign4230_e4657_d_n9, assign4230_e4657_d_n10, assign4230_e4657_d_n11, assign4230_e4657_d_n12, assign4230_e4657_d_n13,) = {
    if (var_guard123 != 0.0) {
        let assign4230_e4653: f64 = (-var_ircx);
        let assign4230_e4655: f64 = (assign4230_e4653 * var_avalf);
        (assign4230_e4655, (((-var_ircx_dn0) * var_avalf) + (assign4230_e4653 * var_avalf_dn0)), (((-var_ircx_dn1) * var_avalf) + (assign4230_e4653 * var_avalf_dn1)), (((-var_ircx_dn2) * var_avalf) + (assign4230_e4653 * var_avalf_dn2)), (((-var_ircx_dn3) * var_avalf) + (assign4230_e4653 * var_avalf_dn3)), (((-var_ircx_dn4) * var_avalf) + (assign4230_e4653 * var_avalf_dn4)), (((-var_ircx_dn5) * var_avalf) + (assign4230_e4653 * var_avalf_dn5)), (((-var_ircx_dn6) * var_avalf) + (assign4230_e4653 * var_avalf_dn6)), (((-var_ircx_dn7) * var_avalf) + (assign4230_e4653 * var_avalf_dn7)), (((-var_ircx_dn8) * var_avalf) + (assign4230_e4653 * var_avalf_dn8)), (((-var_ircx_dn9) * var_avalf) + (assign4230_e4653 * var_avalf_dn9)), (((-var_ircx_dn10) * var_avalf) + (assign4230_e4653 * var_avalf_dn10)), (((-var_ircx_dn11) * var_avalf) + (assign4230_e4653 * var_avalf_dn11)), (((-var_ircx_dn12) * var_avalf) + (assign4230_e4653 * var_avalf_dn12)), (((-var_ircx_dn13) * var_avalf) + (assign4230_e4653 * var_avalf_dn13)),)
    } else {
        (var_igcx, var_igcx_dn0, var_igcx_dn1, var_igcx_dn2, var_igcx_dn3, var_igcx_dn4, var_igcx_dn5, var_igcx_dn6, var_igcx_dn7, var_igcx_dn8, var_igcx_dn9, var_igcx_dn10, var_igcx_dn11, var_igcx_dn12, var_igcx_dn13,)
    }
};
        var_igcx = assign4230_e4657;
        var_igcx_dn0 = assign4230_e4657_d_n0;
        var_igcx_dn1 = assign4230_e4657_d_n1;
        var_igcx_dn2 = assign4230_e4657_d_n2;
        var_igcx_dn3 = assign4230_e4657_d_n3;
        var_igcx_dn4 = assign4230_e4657_d_n4;
        var_igcx_dn5 = assign4230_e4657_d_n5;
        var_igcx_dn6 = assign4230_e4657_d_n6;
        var_igcx_dn7 = assign4230_e4657_d_n7;
        var_igcx_dn8 = assign4230_e4657_d_n8;
        var_igcx_dn9 = assign4230_e4657_d_n9;
        var_igcx_dn10 = assign4230_e4657_d_n10;
        var_igcx_dn11 = assign4230_e4657_d_n11;
        var_igcx_dn12 = assign4230_e4657_d_n12;
        var_igcx_dn13 = assign4230_e4657_d_n13;

        let (assign4240_e4662, assign4240_e4662_d_n0, assign4240_e4662_d_n1, assign4240_e4662_d_n2, assign4240_e4662_d_n3, assign4240_e4662_d_n4, assign4240_e4662_d_n5, assign4240_e4662_d_n6, assign4240_e4662_d_n7, assign4240_e4662_d_n8, assign4240_e4662_d_n9, assign4240_e4662_d_n10, assign4240_e4662_d_n11, assign4240_e4662_d_n12, assign4240_e4662_d_n13,) = {
    if (var_guard123 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_igcx, var_igcx_dn0, var_igcx_dn1, var_igcx_dn2, var_igcx_dn3, var_igcx_dn4, var_igcx_dn5, var_igcx_dn6, var_igcx_dn7, var_igcx_dn8, var_igcx_dn9, var_igcx_dn10, var_igcx_dn11, var_igcx_dn12, var_igcx_dn13,)
    }
};
        var_igcx = assign4240_e4662;
        var_igcx_dn0 = assign4240_e4662_d_n0;
        var_igcx_dn1 = assign4240_e4662_d_n1;
        var_igcx_dn2 = assign4240_e4662_d_n2;
        var_igcx_dn3 = assign4240_e4662_d_n3;
        var_igcx_dn4 = assign4240_e4662_d_n4;
        var_igcx_dn5 = assign4240_e4662_d_n5;
        var_igcx_dn6 = assign4240_e4662_d_n6;
        var_igcx_dn7 = assign4240_e4662_d_n7;
        var_igcx_dn8 = assign4240_e4662_d_n8;
        var_igcx_dn9 = assign4240_e4662_d_n9;
        var_igcx_dn10 = assign4240_e4662_d_n10;
        var_igcx_dn11 = assign4240_e4662_d_n11;
        var_igcx_dn12 = assign4240_e4662_d_n12;
        var_igcx_dn13 = assign4240_e4662_d_n13;

        let assign4250_e4669: f64 = if ((p.p97 > 0.0) && (p.p95 > 0.0)) { 1.0 } else { 0.0 };
        var_guard130 = assign4250_e4669;

        let assign4260_e4672: f64 = if p.p94 > 0.0 { 1.0 } else { 0.0 };
        var_guard131 = assign4260_e4672;

        let (assign4270_e4684, assign4270_e4684_d_n0, assign4270_e4684_d_n1, assign4270_e4684_d_n2, assign4270_e4684_d_n3, assign4270_e4684_d_n4, assign4270_e4684_d_n5, assign4270_e4684_d_n6, assign4270_e4684_d_n7, assign4270_e4684_d_n8, assign4270_e4684_d_n9, assign4270_e4684_d_n10, assign4270_e4684_d_n11, assign4270_e4684_d_n12, assign4270_e4684_d_n13,) = {
    if ((var_guard130 != 0.0) && (var_guard131 != 0.0)) {
        let assign4270_e4679: f64 = (var_vbci / p.p94);
        let assign4270_e4680: f64 = (1.0 - assign4270_e4679);
        let assign4270_e4682: f64 = (assign4270_e4680 - 0.1);
        (assign4270_e4682, (-(var_vbci_dn0 / p.p94)), (-(var_vbci_dn1 / p.p94)), (-(var_vbci_dn2 / p.p94)), (-(var_vbci_dn3 / p.p94)), (-(var_vbci_dn4 / p.p94)), (-(var_vbci_dn5 / p.p94)), (-(var_vbci_dn6 / p.p94)), (-(var_vbci_dn7 / p.p94)), (-(var_vbci_dn8 / p.p94)), (-(var_vbci_dn9 / p.p94)), (-(var_vbci_dn10 / p.p94)), (-(var_vbci_dn11 / p.p94)), (-(var_vbci_dn12 / p.p94)), (-(var_vbci_dn13 / p.p94)),)
    } else {
        (var_vcbfac, var_vcbfac_dn0, var_vcbfac_dn1, var_vcbfac_dn2, var_vcbfac_dn3, var_vcbfac_dn4, var_vcbfac_dn5, var_vcbfac_dn6, var_vcbfac_dn7, var_vcbfac_dn8, var_vcbfac_dn9, var_vcbfac_dn10, var_vcbfac_dn11, var_vcbfac_dn12, var_vcbfac_dn13,)
    }
};
        var_vcbfac = assign4270_e4684;
        var_vcbfac_dn0 = assign4270_e4684_d_n0;
        var_vcbfac_dn1 = assign4270_e4684_d_n1;
        var_vcbfac_dn2 = assign4270_e4684_d_n2;
        var_vcbfac_dn3 = assign4270_e4684_d_n3;
        var_vcbfac_dn4 = assign4270_e4684_d_n4;
        var_vcbfac_dn5 = assign4270_e4684_d_n5;
        var_vcbfac_dn6 = assign4270_e4684_d_n6;
        var_vcbfac_dn7 = assign4270_e4684_d_n7;
        var_vcbfac_dn8 = assign4270_e4684_d_n8;
        var_vcbfac_dn9 = assign4270_e4684_d_n9;
        var_vcbfac_dn10 = assign4270_e4684_d_n10;
        var_vcbfac_dn11 = assign4270_e4684_d_n11;
        var_vcbfac_dn12 = assign4270_e4684_d_n12;
        var_vcbfac_dn13 = assign4270_e4684_d_n13;

        let (assign4280_e4701, assign4280_e4701_d_n0, assign4280_e4701_d_n1, assign4280_e4701_d_n2, assign4280_e4701_d_n3, assign4280_e4701_d_n4, assign4280_e4701_d_n5, assign4280_e4701_d_n6, assign4280_e4701_d_n7, assign4280_e4701_d_n8, assign4280_e4701_d_n9, assign4280_e4701_d_n10, assign4280_e4701_d_n11, assign4280_e4701_d_n12, assign4280_e4701_d_n13,) = {
    if ((var_guard130 != 0.0) && (var_guard131 != 0.0)) {
        let assign4280_e4693: f64 = (var_vcbfac * var_vcbfac);
        let assign4280_e4695: f64 = (assign4280_e4693 + 0.0001);
        let assign4280_e4696: f64 = (assign4280_e4695).sqrt();
        let assign4280_e4697: f64 = (var_vcbfac + assign4280_e4696);
        let assign4280_e4698: f64 = (0.5 * assign4280_e4697);
        let assign4280_e4699: f64 = (0.1 + assign4280_e4698);
        (assign4280_e4699, (0.5 * (var_vcbfac_dn0 + (((var_vcbfac_dn0 * var_vcbfac) + (var_vcbfac * var_vcbfac_dn0)) / (2.0 * assign4280_e4696)))), (0.5 * (var_vcbfac_dn1 + (((var_vcbfac_dn1 * var_vcbfac) + (var_vcbfac * var_vcbfac_dn1)) / (2.0 * assign4280_e4696)))), (0.5 * (var_vcbfac_dn2 + (((var_vcbfac_dn2 * var_vcbfac) + (var_vcbfac * var_vcbfac_dn2)) / (2.0 * assign4280_e4696)))), (0.5 * (var_vcbfac_dn3 + (((var_vcbfac_dn3 * var_vcbfac) + (var_vcbfac * var_vcbfac_dn3)) / (2.0 * assign4280_e4696)))), (0.5 * (var_vcbfac_dn4 + (((var_vcbfac_dn4 * var_vcbfac) + (var_vcbfac * var_vcbfac_dn4)) / (2.0 * assign4280_e4696)))), (0.5 * (var_vcbfac_dn5 + (((var_vcbfac_dn5 * var_vcbfac) + (var_vcbfac * var_vcbfac_dn5)) / (2.0 * assign4280_e4696)))), (0.5 * (var_vcbfac_dn6 + (((var_vcbfac_dn6 * var_vcbfac) + (var_vcbfac * var_vcbfac_dn6)) / (2.0 * assign4280_e4696)))), (0.5 * (var_vcbfac_dn7 + (((var_vcbfac_dn7 * var_vcbfac) + (var_vcbfac * var_vcbfac_dn7)) / (2.0 * assign4280_e4696)))), (0.5 * (var_vcbfac_dn8 + (((var_vcbfac_dn8 * var_vcbfac) + (var_vcbfac * var_vcbfac_dn8)) / (2.0 * assign4280_e4696)))), (0.5 * (var_vcbfac_dn9 + (((var_vcbfac_dn9 * var_vcbfac) + (var_vcbfac * var_vcbfac_dn9)) / (2.0 * assign4280_e4696)))), (0.5 * (var_vcbfac_dn10 + (((var_vcbfac_dn10 * var_vcbfac) + (var_vcbfac * var_vcbfac_dn10)) / (2.0 * assign4280_e4696)))), (0.5 * (var_vcbfac_dn11 + (((var_vcbfac_dn11 * var_vcbfac) + (var_vcbfac * var_vcbfac_dn11)) / (2.0 * assign4280_e4696)))), (0.5 * (var_vcbfac_dn12 + (((var_vcbfac_dn12 * var_vcbfac) + (var_vcbfac * var_vcbfac_dn12)) / (2.0 * assign4280_e4696)))), (0.5 * (var_vcbfac_dn13 + (((var_vcbfac_dn13 * var_vcbfac) + (var_vcbfac * var_vcbfac_dn13)) / (2.0 * assign4280_e4696)))),)
    } else {
        (var_vcbfac, var_vcbfac_dn0, var_vcbfac_dn1, var_vcbfac_dn2, var_vcbfac_dn3, var_vcbfac_dn4, var_vcbfac_dn5, var_vcbfac_dn6, var_vcbfac_dn7, var_vcbfac_dn8, var_vcbfac_dn9, var_vcbfac_dn10, var_vcbfac_dn11, var_vcbfac_dn12, var_vcbfac_dn13,)
    }
};
        var_vcbfac = assign4280_e4701;
        var_vcbfac_dn0 = assign4280_e4701_d_n0;
        var_vcbfac_dn1 = assign4280_e4701_d_n1;
        var_vcbfac_dn2 = assign4280_e4701_d_n2;
        var_vcbfac_dn3 = assign4280_e4701_d_n3;
        var_vcbfac_dn4 = assign4280_e4701_d_n4;
        var_vcbfac_dn5 = assign4280_e4701_d_n5;
        var_vcbfac_dn6 = assign4280_e4701_d_n6;
        var_vcbfac_dn7 = assign4280_e4701_d_n7;
        var_vcbfac_dn8 = assign4280_e4701_d_n8;
        var_vcbfac_dn9 = assign4280_e4701_d_n9;
        var_vcbfac_dn10 = assign4280_e4701_d_n10;
        var_vcbfac_dn11 = assign4280_e4701_d_n11;
        var_vcbfac_dn12 = assign4280_e4701_d_n12;
        var_vcbfac_dn13 = assign4280_e4701_d_n13;

        let (assign4290_e4709, assign4290_e4709_d_n0, assign4290_e4709_d_n1, assign4290_e4709_d_n2, assign4290_e4709_d_n3, assign4290_e4709_d_n4, assign4290_e4709_d_n5, assign4290_e4709_d_n6, assign4290_e4709_d_n7, assign4290_e4709_d_n8, assign4290_e4709_d_n9, assign4290_e4709_d_n10, assign4290_e4709_d_n11, assign4290_e4709_d_n12, assign4290_e4709_d_n13,) = {
    if ((var_guard130 != 0.0) && (var_guard131 != 0.0)) {
        let assign4290_e4707: f64 = (p.p95 * var_vcbfac);
        (assign4290_e4707, (p.p95 * var_vcbfac_dn0), (p.p95 * var_vcbfac_dn1), (p.p95 * var_vcbfac_dn2), (p.p95 * var_vcbfac_dn3), (p.p95 * var_vcbfac_dn4), (p.p95 * var_vcbfac_dn5), (p.p95 * var_vcbfac_dn6), (p.p95 * var_vcbfac_dn7), (p.p95 * var_vcbfac_dn8), (p.p95 * var_vcbfac_dn9), (p.p95 * var_vcbfac_dn10), (p.p95 * var_vcbfac_dn11), (p.p95 * var_vcbfac_dn12), (p.p95 * var_vcbfac_dn13),)
    } else {
        (var_iibk, var_iibk_dn0, var_iibk_dn1, var_iibk_dn2, var_iibk_dn3, var_iibk_dn4, var_iibk_dn5, var_iibk_dn6, var_iibk_dn7, var_iibk_dn8, var_iibk_dn9, var_iibk_dn10, var_iibk_dn11, var_iibk_dn12, var_iibk_dn13,)
    }
};
        var_iibk = assign4290_e4709;
        var_iibk_dn0 = assign4290_e4709_d_n0;
        var_iibk_dn1 = assign4290_e4709_d_n1;
        var_iibk_dn2 = assign4290_e4709_d_n2;
        var_iibk_dn3 = assign4290_e4709_d_n3;
        var_iibk_dn4 = assign4290_e4709_d_n4;
        var_iibk_dn5 = assign4290_e4709_d_n5;
        var_iibk_dn6 = assign4290_e4709_d_n6;
        var_iibk_dn7 = assign4290_e4709_d_n7;
        var_iibk_dn8 = assign4290_e4709_d_n8;
        var_iibk_dn9 = assign4290_e4709_d_n9;
        var_iibk_dn10 = assign4290_e4709_d_n10;
        var_iibk_dn11 = assign4290_e4709_d_n11;
        var_iibk_dn12 = assign4290_e4709_d_n12;
        var_iibk_dn13 = assign4290_e4709_d_n13;

        let (assign4300_e4716, assign4300_e4716_d_n0, assign4300_e4716_d_n1, assign4300_e4716_d_n2, assign4300_e4716_d_n3, assign4300_e4716_d_n4, assign4300_e4716_d_n5, assign4300_e4716_d_n6, assign4300_e4716_d_n7, assign4300_e4716_d_n8, assign4300_e4716_d_n9, assign4300_e4716_d_n10, assign4300_e4716_d_n11, assign4300_e4716_d_n12, assign4300_e4716_d_n13,) = {
    if ((var_guard130 != 0.0) && (var_guard131 == 0.0)) {
        (p.p95, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iibk, var_iibk_dn0, var_iibk_dn1, var_iibk_dn2, var_iibk_dn3, var_iibk_dn4, var_iibk_dn5, var_iibk_dn6, var_iibk_dn7, var_iibk_dn8, var_iibk_dn9, var_iibk_dn10, var_iibk_dn11, var_iibk_dn12, var_iibk_dn13,)
    }
};
        var_iibk = assign4300_e4716;
        var_iibk_dn0 = assign4300_e4716_d_n0;
        var_iibk_dn1 = assign4300_e4716_d_n1;
        var_iibk_dn2 = assign4300_e4716_d_n2;
        var_iibk_dn3 = assign4300_e4716_d_n3;
        var_iibk_dn4 = assign4300_e4716_d_n4;
        var_iibk_dn5 = assign4300_e4716_d_n5;
        var_iibk_dn6 = assign4300_e4716_d_n6;
        var_iibk_dn7 = assign4300_e4716_d_n7;
        var_iibk_dn8 = assign4300_e4716_d_n8;
        var_iibk_dn9 = assign4300_e4716_d_n9;
        var_iibk_dn10 = assign4300_e4716_d_n10;
        var_iibk_dn11 = assign4300_e4716_d_n11;
        var_iibk_dn12 = assign4300_e4716_d_n12;
        var_iibk_dn13 = assign4300_e4716_d_n13;

        let (assign4310_e4728, assign4310_e4728_d_n0, assign4310_e4728_d_n1, assign4310_e4728_d_n2, assign4310_e4728_d_n3, assign4310_e4728_d_n4, assign4310_e4728_d_n5, assign4310_e4728_d_n6, assign4310_e4728_d_n7, assign4310_e4728_d_n8, assign4310_e4728_d_n9, assign4310_e4728_d_n10, assign4310_e4728_d_n11, assign4310_e4728_d_n12, assign4310_e4728_d_n13,) = {
    if (var_guard130 != 0.0) {
        let assign4310_e4721: f64 = (var_itzf / var_iibk);
        let assign4310_e4723: f64 = (assign4310_e4721 - 1.0);
        let assign4310_e4725: f64 = (assign4310_e4723).powf(p.p96);
        let assign4310_e4726: f64 = (p.p97 * assign4310_e4725);
        (assign4310_e4726, (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (((var_itzf_dn0 * var_iibk) - (var_itzf * var_iibk_dn0)) / (var_iibk * var_iibk)))) } } else { (assign4310_e4725 * (p.p96 * ((((var_itzf_dn0 * var_iibk) - (var_itzf * var_iibk_dn0)) / (var_iibk * var_iibk)) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (((var_itzf_dn1 * var_iibk) - (var_itzf * var_iibk_dn1)) / (var_iibk * var_iibk)))) } } else { (assign4310_e4725 * (p.p96 * ((((var_itzf_dn1 * var_iibk) - (var_itzf * var_iibk_dn1)) / (var_iibk * var_iibk)) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (((var_itzf_dn2 * var_iibk) - (var_itzf * var_iibk_dn2)) / (var_iibk * var_iibk)))) } } else { (assign4310_e4725 * (p.p96 * ((((var_itzf_dn2 * var_iibk) - (var_itzf * var_iibk_dn2)) / (var_iibk * var_iibk)) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (((var_itzf_dn3 * var_iibk) - (var_itzf * var_iibk_dn3)) / (var_iibk * var_iibk)))) } } else { (assign4310_e4725 * (p.p96 * ((((var_itzf_dn3 * var_iibk) - (var_itzf * var_iibk_dn3)) / (var_iibk * var_iibk)) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (((var_itzf_dn4 * var_iibk) - (var_itzf * var_iibk_dn4)) / (var_iibk * var_iibk)))) } } else { (assign4310_e4725 * (p.p96 * ((((var_itzf_dn4 * var_iibk) - (var_itzf * var_iibk_dn4)) / (var_iibk * var_iibk)) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (((var_itzf_dn5 * var_iibk) - (var_itzf * var_iibk_dn5)) / (var_iibk * var_iibk)))) } } else { (assign4310_e4725 * (p.p96 * ((((var_itzf_dn5 * var_iibk) - (var_itzf * var_iibk_dn5)) / (var_iibk * var_iibk)) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (((var_itzf_dn6 * var_iibk) - (var_itzf * var_iibk_dn6)) / (var_iibk * var_iibk)))) } } else { (assign4310_e4725 * (p.p96 * ((((var_itzf_dn6 * var_iibk) - (var_itzf * var_iibk_dn6)) / (var_iibk * var_iibk)) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (((var_itzf_dn7 * var_iibk) - (var_itzf * var_iibk_dn7)) / (var_iibk * var_iibk)))) } } else { (assign4310_e4725 * (p.p96 * ((((var_itzf_dn7 * var_iibk) - (var_itzf * var_iibk_dn7)) / (var_iibk * var_iibk)) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (((var_itzf_dn8 * var_iibk) - (var_itzf * var_iibk_dn8)) / (var_iibk * var_iibk)))) } } else { (assign4310_e4725 * (p.p96 * ((((var_itzf_dn8 * var_iibk) - (var_itzf * var_iibk_dn8)) / (var_iibk * var_iibk)) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (((var_itzf_dn9 * var_iibk) - (var_itzf * var_iibk_dn9)) / (var_iibk * var_iibk)))) } } else { (assign4310_e4725 * (p.p96 * ((((var_itzf_dn9 * var_iibk) - (var_itzf * var_iibk_dn9)) / (var_iibk * var_iibk)) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (((var_itzf_dn10 * var_iibk) - (var_itzf * var_iibk_dn10)) / (var_iibk * var_iibk)))) } } else { (assign4310_e4725 * (p.p96 * ((((var_itzf_dn10 * var_iibk) - (var_itzf * var_iibk_dn10)) / (var_iibk * var_iibk)) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (((var_itzf_dn11 * var_iibk) - (var_itzf * var_iibk_dn11)) / (var_iibk * var_iibk)))) } } else { (assign4310_e4725 * (p.p96 * ((((var_itzf_dn11 * var_iibk) - (var_itzf * var_iibk_dn11)) / (var_iibk * var_iibk)) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (((var_itzf_dn12 * var_iibk) - (var_itzf * var_iibk_dn12)) / (var_iibk * var_iibk)))) } } else { (assign4310_e4725 * (p.p96 * ((((var_itzf_dn12 * var_iibk) - (var_itzf * var_iibk_dn12)) / (var_iibk * var_iibk)) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (((var_itzf_dn13 * var_iibk) - (var_itzf * var_iibk_dn13)) / (var_iibk * var_iibk)))) } } else { (assign4310_e4725 * (p.p96 * ((((var_itzf_dn13 * var_iibk) - (var_itzf * var_iibk_dn13)) / (var_iibk * var_iibk)) / assign4310_e4723))) }),)
    } else {
        (var_ibk, var_ibk_dn0, var_ibk_dn1, var_ibk_dn2, var_ibk_dn3, var_ibk_dn4, var_ibk_dn5, var_ibk_dn6, var_ibk_dn7, var_ibk_dn8, var_ibk_dn9, var_ibk_dn10, var_ibk_dn11, var_ibk_dn12, var_ibk_dn13,)
    }
};
        var_ibk = assign4310_e4728;
        var_ibk_dn0 = assign4310_e4728_d_n0;
        var_ibk_dn1 = assign4310_e4728_d_n1;
        var_ibk_dn2 = assign4310_e4728_d_n2;
        var_ibk_dn3 = assign4310_e4728_d_n3;
        var_ibk_dn4 = assign4310_e4728_d_n4;
        var_ibk_dn5 = assign4310_e4728_d_n5;
        var_ibk_dn6 = assign4310_e4728_d_n6;
        var_ibk_dn7 = assign4310_e4728_d_n7;
        var_ibk_dn8 = assign4310_e4728_d_n8;
        var_ibk_dn9 = assign4310_e4728_d_n9;
        var_ibk_dn10 = assign4310_e4728_d_n10;
        var_ibk_dn11 = assign4310_e4728_d_n11;
        var_ibk_dn12 = assign4310_e4728_d_n12;
        var_ibk_dn13 = assign4310_e4728_d_n13;

        let (assign4320_e4733, assign4320_e4733_d_n0, assign4320_e4733_d_n1, assign4320_e4733_d_n2, assign4320_e4733_d_n3, assign4320_e4733_d_n4, assign4320_e4733_d_n5, assign4320_e4733_d_n6, assign4320_e4733_d_n7, assign4320_e4733_d_n8, assign4320_e4733_d_n9, assign4320_e4733_d_n10, assign4320_e4733_d_n11, assign4320_e4733_d_n12, assign4320_e4733_d_n13,) = {
    if (var_guard130 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibk, var_ibk_dn0, var_ibk_dn1, var_ibk_dn2, var_ibk_dn3, var_ibk_dn4, var_ibk_dn5, var_ibk_dn6, var_ibk_dn7, var_ibk_dn8, var_ibk_dn9, var_ibk_dn10, var_ibk_dn11, var_ibk_dn12, var_ibk_dn13,)
    }
};
        var_ibk = assign4320_e4733;
        var_ibk_dn0 = assign4320_e4733_d_n0;
        var_ibk_dn1 = assign4320_e4733_d_n1;
        var_ibk_dn2 = assign4320_e4733_d_n2;
        var_ibk_dn3 = assign4320_e4733_d_n3;
        var_ibk_dn4 = assign4320_e4733_d_n4;
        var_ibk_dn5 = assign4320_e4733_d_n5;
        var_ibk_dn6 = assign4320_e4733_d_n6;
        var_ibk_dn7 = assign4320_e4733_d_n7;
        var_ibk_dn8 = assign4320_e4733_d_n8;
        var_ibk_dn9 = assign4320_e4733_d_n9;
        var_ibk_dn10 = assign4320_e4733_d_n10;
        var_ibk_dn11 = assign4320_e4733_d_n11;
        var_ibk_dn12 = assign4320_e4733_d_n12;
        var_ibk_dn13 = assign4320_e4733_d_n13;

        let assign4330_e4736: f64 = (var_ibcj - var_igc);
        let assign4330_e4738: f64 = (assign4330_e4736 - var_ibk);
        var_ibc = assign4330_e4738;
        var_ibc_dn0 = ((var_ibcj_dn0 - var_igc_dn0) - var_ibk_dn0);
        var_ibc_dn1 = ((var_ibcj_dn1 - var_igc_dn1) - var_ibk_dn1);
        var_ibc_dn2 = ((var_ibcj_dn2 - var_igc_dn2) - var_ibk_dn2);
        var_ibc_dn3 = ((var_ibcj_dn3 - var_igc_dn3) - var_ibk_dn3);
        var_ibc_dn4 = ((var_ibcj_dn4 - var_igc_dn4) - var_ibk_dn4);
        var_ibc_dn5 = ((var_ibcj_dn5 - var_igc_dn5) - var_ibk_dn5);
        var_ibc_dn6 = ((var_ibcj_dn6 - var_igc_dn6) - var_ibk_dn6);
        var_ibc_dn7 = ((var_ibcj_dn7 - var_igc_dn7) - var_ibk_dn7);
        var_ibc_dn8 = ((var_ibcj_dn8 - var_igc_dn8) - var_ibk_dn8);
        var_ibc_dn9 = ((var_ibcj_dn9 - var_igc_dn9) - var_ibk_dn9);
        var_ibc_dn10 = ((var_ibcj_dn10 - var_igc_dn10) - var_ibk_dn10);
        var_ibc_dn11 = ((var_ibcj_dn11 - var_igc_dn11) - var_ibk_dn11);
        var_ibc_dn12 = ((var_ibcj_dn12 - var_igc_dn12) - var_ibk_dn12);
        var_ibc_dn13 = ((var_ibcj_dn13 - var_igc_dn13) - var_ibk_dn13);

        let assign4340_e4745: f64 = if ((p.p66 > 0.0) || (p.p68 > 0.0)) { 1.0 } else { 0.0 };
        var_guard132 = assign4340_e4745;

        let (assign4350_e4753, assign4350_e4753_d_n0, assign4350_e4753_d_n1, assign4350_e4753_d_n2, assign4350_e4753_d_n3, assign4350_e4753_d_n4, assign4350_e4753_d_n5, assign4350_e4753_d_n6, assign4350_e4753_d_n7, assign4350_e4753_d_n8, assign4350_e4753_d_n9, assign4350_e4753_d_n10, assign4350_e4753_d_n11, assign4350_e4753_d_n12, assign4350_e4753_d_n13,) = {
    if (var_guard132 != 0.0) {
        let assign4350_e4750: f64 = (p.p67 * var_vtv);
        let assign4350_e4751: f64 = (1.0 / assign4350_e4750);
        (assign4350_e4751, (-((p.p67 * var_vtv_dn0) / (assign4350_e4750 * assign4350_e4750))), (-((p.p67 * var_vtv_dn1) / (assign4350_e4750 * assign4350_e4750))), (-((p.p67 * var_vtv_dn2) / (assign4350_e4750 * assign4350_e4750))), (-((p.p67 * var_vtv_dn3) / (assign4350_e4750 * assign4350_e4750))), (-((p.p67 * var_vtv_dn4) / (assign4350_e4750 * assign4350_e4750))), (-((p.p67 * var_vtv_dn5) / (assign4350_e4750 * assign4350_e4750))), (-((p.p67 * var_vtv_dn6) / (assign4350_e4750 * assign4350_e4750))), (-((p.p67 * var_vtv_dn7) / (assign4350_e4750 * assign4350_e4750))), (-((p.p67 * var_vtv_dn8) / (assign4350_e4750 * assign4350_e4750))), (-((p.p67 * var_vtv_dn9) / (assign4350_e4750 * assign4350_e4750))), (-((p.p67 * var_vtv_dn10) / (assign4350_e4750 * assign4350_e4750))), (-((p.p67 * var_vtv_dn11) / (assign4350_e4750 * assign4350_e4750))), (-((p.p67 * var_vtv_dn12) / (assign4350_e4750 * assign4350_e4750))), (-((p.p67 * var_vtv_dn13) / (assign4350_e4750 * assign4350_e4750))),)
    } else {
        (var_afac, var_afac_dn0, var_afac_dn1, var_afac_dn2, var_afac_dn3, var_afac_dn4, var_afac_dn5, var_afac_dn6, var_afac_dn7, var_afac_dn8, var_afac_dn9, var_afac_dn10, var_afac_dn11, var_afac_dn12, var_afac_dn13,)
    }
};
        var_afac = assign4350_e4753;
        var_afac_dn0 = assign4350_e4753_d_n0;
        var_afac_dn1 = assign4350_e4753_d_n1;
        var_afac_dn2 = assign4350_e4753_d_n2;
        var_afac_dn3 = assign4350_e4753_d_n3;
        var_afac_dn4 = assign4350_e4753_d_n4;
        var_afac_dn5 = assign4350_e4753_d_n5;
        var_afac_dn6 = assign4350_e4753_d_n6;
        var_afac_dn7 = assign4350_e4753_d_n7;
        var_afac_dn8 = assign4350_e4753_d_n8;
        var_afac_dn9 = assign4350_e4753_d_n9;
        var_afac_dn10 = assign4350_e4753_d_n10;
        var_afac_dn11 = assign4350_e4753_d_n11;
        var_afac_dn12 = assign4350_e4753_d_n12;
        var_afac_dn13 = assign4350_e4753_d_n13;

        let assign4360_e4756: f64 = if var_vbcp < var_maxvibcip { 1.0 } else { 0.0 };
        var_guard133 = assign4360_e4756;

        let (assign4370_e4765, assign4370_e4765_d_n0, assign4370_e4765_d_n1, assign4370_e4765_d_n2, assign4370_e4765_d_n3, assign4370_e4765_d_n4, assign4370_e4765_d_n5, assign4370_e4765_d_n6, assign4370_e4765_d_n7, assign4370_e4765_d_n8, assign4370_e4765_d_n9, assign4370_e4765_d_n10, assign4370_e4765_d_n11, assign4370_e4765_d_n12, assign4370_e4765_d_n13,) = {
    if ((var_guard132 != 0.0) && (var_guard133 != 0.0)) {
        let assign4370_e4762: f64 = (var_vbcp * var_afac);
        let assign4370_e4763: f64 = (assign4370_e4762).exp();
        (assign4370_e4763, (assign4370_e4763 * ((var_vbcp_dn0 * var_afac) + (var_vbcp * var_afac_dn0))), (assign4370_e4763 * ((var_vbcp_dn1 * var_afac) + (var_vbcp * var_afac_dn1))), (assign4370_e4763 * ((var_vbcp_dn2 * var_afac) + (var_vbcp * var_afac_dn2))), (assign4370_e4763 * ((var_vbcp_dn3 * var_afac) + (var_vbcp * var_afac_dn3))), (assign4370_e4763 * ((var_vbcp_dn4 * var_afac) + (var_vbcp * var_afac_dn4))), (assign4370_e4763 * ((var_vbcp_dn5 * var_afac) + (var_vbcp * var_afac_dn5))), (assign4370_e4763 * ((var_vbcp_dn6 * var_afac) + (var_vbcp * var_afac_dn6))), (assign4370_e4763 * ((var_vbcp_dn7 * var_afac) + (var_vbcp * var_afac_dn7))), (assign4370_e4763 * ((var_vbcp_dn8 * var_afac) + (var_vbcp * var_afac_dn8))), (assign4370_e4763 * ((var_vbcp_dn9 * var_afac) + (var_vbcp * var_afac_dn9))), (assign4370_e4763 * ((var_vbcp_dn10 * var_afac) + (var_vbcp * var_afac_dn10))), (assign4370_e4763 * ((var_vbcp_dn11 * var_afac) + (var_vbcp * var_afac_dn11))), (assign4370_e4763 * ((var_vbcp_dn12 * var_afac) + (var_vbcp * var_afac_dn12))), (assign4370_e4763 * ((var_vbcp_dn13 * var_afac) + (var_vbcp * var_afac_dn13))),)
    } else {
        (var_expi, var_expi_dn0, var_expi_dn1, var_expi_dn2, var_expi_dn3, var_expi_dn4, var_expi_dn5, var_expi_dn6, var_expi_dn7, var_expi_dn8, var_expi_dn9, var_expi_dn10, var_expi_dn11, var_expi_dn12, var_expi_dn13,)
    }
};
        var_expi = assign4370_e4765;
        var_expi_dn0 = assign4370_e4765_d_n0;
        var_expi_dn1 = assign4370_e4765_d_n1;
        var_expi_dn2 = assign4370_e4765_d_n2;
        var_expi_dn3 = assign4370_e4765_d_n3;
        var_expi_dn4 = assign4370_e4765_d_n4;
        var_expi_dn5 = assign4370_e4765_d_n5;
        var_expi_dn6 = assign4370_e4765_d_n6;
        var_expi_dn7 = assign4370_e4765_d_n7;
        var_expi_dn8 = assign4370_e4765_d_n8;
        var_expi_dn9 = assign4370_e4765_d_n9;
        var_expi_dn10 = assign4370_e4765_d_n10;
        var_expi_dn11 = assign4370_e4765_d_n11;
        var_expi_dn12 = assign4370_e4765_d_n12;
        var_expi_dn13 = assign4370_e4765_d_n13;

        let (assign4380_e4783, assign4380_e4783_d_n0, assign4380_e4783_d_n1, assign4380_e4783_d_n2, assign4380_e4783_d_n3, assign4380_e4783_d_n4, assign4380_e4783_d_n5, assign4380_e4783_d_n6, assign4380_e4783_d_n7, assign4380_e4783_d_n8, assign4380_e4783_d_n9, assign4380_e4783_d_n10, assign4380_e4783_d_n11, assign4380_e4783_d_n12, assign4380_e4783_d_n13,) = {
    if ((var_guard132 != 0.0) && (var_guard133 == 0.0)) {
        let assign4380_e4772: f64 = (var_maxvibcip * var_afac);
        let assign4380_e4773: f64 = (assign4380_e4772).exp();
        let assign4380_e4777: f64 = (var_vbcp - var_maxvibcip);
        let assign4380_e4779: f64 = (assign4380_e4777 * var_afac);
        let assign4380_e4780: f64 = (1.0 + assign4380_e4779);
        let assign4380_e4781: f64 = (assign4380_e4773 * assign4380_e4780);
        (assign4380_e4781, (((assign4380_e4773 * ((var_maxvibcip_dn0 * var_afac) + (var_maxvibcip * var_afac_dn0))) * assign4380_e4780) + (assign4380_e4773 * (((var_vbcp_dn0 - var_maxvibcip_dn0) * var_afac) + (assign4380_e4777 * var_afac_dn0)))), (((assign4380_e4773 * ((var_maxvibcip_dn1 * var_afac) + (var_maxvibcip * var_afac_dn1))) * assign4380_e4780) + (assign4380_e4773 * (((var_vbcp_dn1 - var_maxvibcip_dn1) * var_afac) + (assign4380_e4777 * var_afac_dn1)))), (((assign4380_e4773 * ((var_maxvibcip_dn2 * var_afac) + (var_maxvibcip * var_afac_dn2))) * assign4380_e4780) + (assign4380_e4773 * (((var_vbcp_dn2 - var_maxvibcip_dn2) * var_afac) + (assign4380_e4777 * var_afac_dn2)))), (((assign4380_e4773 * ((var_maxvibcip_dn3 * var_afac) + (var_maxvibcip * var_afac_dn3))) * assign4380_e4780) + (assign4380_e4773 * (((var_vbcp_dn3 - var_maxvibcip_dn3) * var_afac) + (assign4380_e4777 * var_afac_dn3)))), (((assign4380_e4773 * ((var_maxvibcip_dn4 * var_afac) + (var_maxvibcip * var_afac_dn4))) * assign4380_e4780) + (assign4380_e4773 * (((var_vbcp_dn4 - var_maxvibcip_dn4) * var_afac) + (assign4380_e4777 * var_afac_dn4)))), (((assign4380_e4773 * ((var_maxvibcip_dn5 * var_afac) + (var_maxvibcip * var_afac_dn5))) * assign4380_e4780) + (assign4380_e4773 * (((var_vbcp_dn5 - var_maxvibcip_dn5) * var_afac) + (assign4380_e4777 * var_afac_dn5)))), (((assign4380_e4773 * ((var_maxvibcip_dn6 * var_afac) + (var_maxvibcip * var_afac_dn6))) * assign4380_e4780) + (assign4380_e4773 * (((var_vbcp_dn6 - var_maxvibcip_dn6) * var_afac) + (assign4380_e4777 * var_afac_dn6)))), (((assign4380_e4773 * ((var_maxvibcip_dn7 * var_afac) + (var_maxvibcip * var_afac_dn7))) * assign4380_e4780) + (assign4380_e4773 * (((var_vbcp_dn7 - var_maxvibcip_dn7) * var_afac) + (assign4380_e4777 * var_afac_dn7)))), (((assign4380_e4773 * ((var_maxvibcip_dn8 * var_afac) + (var_maxvibcip * var_afac_dn8))) * assign4380_e4780) + (assign4380_e4773 * (((var_vbcp_dn8 - var_maxvibcip_dn8) * var_afac) + (assign4380_e4777 * var_afac_dn8)))), (((assign4380_e4773 * ((var_maxvibcip_dn9 * var_afac) + (var_maxvibcip * var_afac_dn9))) * assign4380_e4780) + (assign4380_e4773 * (((var_vbcp_dn9 - var_maxvibcip_dn9) * var_afac) + (assign4380_e4777 * var_afac_dn9)))), (((assign4380_e4773 * ((var_maxvibcip_dn10 * var_afac) + (var_maxvibcip * var_afac_dn10))) * assign4380_e4780) + (assign4380_e4773 * (((var_vbcp_dn10 - var_maxvibcip_dn10) * var_afac) + (assign4380_e4777 * var_afac_dn10)))), (((assign4380_e4773 * ((var_maxvibcip_dn11 * var_afac) + (var_maxvibcip * var_afac_dn11))) * assign4380_e4780) + (assign4380_e4773 * (((var_vbcp_dn11 - var_maxvibcip_dn11) * var_afac) + (assign4380_e4777 * var_afac_dn11)))), (((assign4380_e4773 * ((var_maxvibcip_dn12 * var_afac) + (var_maxvibcip * var_afac_dn12))) * assign4380_e4780) + (assign4380_e4773 * (((var_vbcp_dn12 - var_maxvibcip_dn12) * var_afac) + (assign4380_e4777 * var_afac_dn12)))), (((assign4380_e4773 * ((var_maxvibcip_dn13 * var_afac) + (var_maxvibcip * var_afac_dn13))) * assign4380_e4780) + (assign4380_e4773 * (((var_vbcp_dn13 - var_maxvibcip_dn13) * var_afac) + (assign4380_e4777 * var_afac_dn13)))),)
    } else {
        (var_expi, var_expi_dn0, var_expi_dn1, var_expi_dn2, var_expi_dn3, var_expi_dn4, var_expi_dn5, var_expi_dn6, var_expi_dn7, var_expi_dn8, var_expi_dn9, var_expi_dn10, var_expi_dn11, var_expi_dn12, var_expi_dn13,)
    }
};
        var_expi = assign4380_e4783;
        var_expi_dn0 = assign4380_e4783_d_n0;
        var_expi_dn1 = assign4380_e4783_d_n1;
        var_expi_dn2 = assign4380_e4783_d_n2;
        var_expi_dn3 = assign4380_e4783_d_n3;
        var_expi_dn4 = assign4380_e4783_d_n4;
        var_expi_dn5 = assign4380_e4783_d_n5;
        var_expi_dn6 = assign4380_e4783_d_n6;
        var_expi_dn7 = assign4380_e4783_d_n7;
        var_expi_dn8 = assign4380_e4783_d_n8;
        var_expi_dn9 = assign4380_e4783_d_n9;
        var_expi_dn10 = assign4380_e4783_d_n10;
        var_expi_dn11 = assign4380_e4783_d_n11;
        var_expi_dn12 = assign4380_e4783_d_n12;
        var_expi_dn13 = assign4380_e4783_d_n13;


        *var_afac_slot = var_afac;
        *var_afac_dn0_slot = var_afac_dn0;
        *var_afac_dn1_slot = var_afac_dn1;
        *var_afac_dn10_slot = var_afac_dn10;
        *var_afac_dn11_slot = var_afac_dn11;
        *var_afac_dn12_slot = var_afac_dn12;
        *var_afac_dn13_slot = var_afac_dn13;
        *var_afac_dn2_slot = var_afac_dn2;
        *var_afac_dn3_slot = var_afac_dn3;
        *var_afac_dn4_slot = var_afac_dn4;
        *var_afac_dn5_slot = var_afac_dn5;
        *var_afac_dn6_slot = var_afac_dn6;
        *var_afac_dn7_slot = var_afac_dn7;
        *var_afac_dn8_slot = var_afac_dn8;
        *var_afac_dn9_slot = var_afac_dn9;
        *var_avalf_slot = var_avalf;
        *var_avalf_dn0_slot = var_avalf_dn0;
        *var_avalf_dn1_slot = var_avalf_dn1;
        *var_avalf_dn10_slot = var_avalf_dn10;
        *var_avalf_dn11_slot = var_avalf_dn11;
        *var_avalf_dn12_slot = var_avalf_dn12;
        *var_avalf_dn13_slot = var_avalf_dn13;
        *var_avalf_dn2_slot = var_avalf_dn2;
        *var_avalf_dn3_slot = var_avalf_dn3;
        *var_avalf_dn4_slot = var_avalf_dn4;
        *var_avalf_dn5_slot = var_avalf_dn5;
        *var_avalf_dn6_slot = var_avalf_dn6;
        *var_avalf_dn7_slot = var_avalf_dn7;
        *var_avalf_dn8_slot = var_avalf_dn8;
        *var_avalf_dn9_slot = var_avalf_dn9;
        *var_expi_slot = var_expi;
        *var_expi__blk127_slot = var_expi__blk127;
        *var_expi__blk127_dn0_slot = var_expi__blk127_dn0;
        *var_expi__blk127_dn1_slot = var_expi__blk127_dn1;
        *var_expi__blk127_dn10_slot = var_expi__blk127_dn10;
        *var_expi__blk127_dn11_slot = var_expi__blk127_dn11;
        *var_expi__blk127_dn12_slot = var_expi__blk127_dn12;
        *var_expi__blk127_dn13_slot = var_expi__blk127_dn13;
        *var_expi__blk127_dn2_slot = var_expi__blk127_dn2;
        *var_expi__blk127_dn3_slot = var_expi__blk127_dn3;
        *var_expi__blk127_dn4_slot = var_expi__blk127_dn4;
        *var_expi__blk127_dn5_slot = var_expi__blk127_dn5;
        *var_expi__blk127_dn6_slot = var_expi__blk127_dn6;
        *var_expi__blk127_dn7_slot = var_expi__blk127_dn7;
        *var_expi__blk127_dn8_slot = var_expi__blk127_dn8;
        *var_expi__blk127_dn9_slot = var_expi__blk127_dn9;
        *var_expi_dn0_slot = var_expi_dn0;
        *var_expi_dn1_slot = var_expi_dn1;
        *var_expi_dn10_slot = var_expi_dn10;
        *var_expi_dn11_slot = var_expi_dn11;
        *var_expi_dn12_slot = var_expi_dn12;
        *var_expi_dn13_slot = var_expi_dn13;
        *var_expi_dn2_slot = var_expi_dn2;
        *var_expi_dn3_slot = var_expi_dn3;
        *var_expi_dn4_slot = var_expi_dn4;
        *var_expi_dn5_slot = var_expi_dn5;
        *var_expi_dn6_slot = var_expi_dn6;
        *var_expi_dn7_slot = var_expi_dn7;
        *var_expi_dn8_slot = var_expi_dn8;
        *var_expi_dn9_slot = var_expi_dn9;
        *var_expl__blk128_slot = var_expl__blk128;
        *var_guard123_slot = var_guard123;
        *var_guard129_slot = var_guard129;
        *var_guard130_slot = var_guard130;
        *var_guard131_slot = var_guard131;
        *var_guard132_slot = var_guard132;
        *var_guard133_slot = var_guard133;
        *var_ibc_slot = var_ibc;
        *var_ibc_dn0_slot = var_ibc_dn0;
        *var_ibc_dn1_slot = var_ibc_dn1;
        *var_ibc_dn10_slot = var_ibc_dn10;
        *var_ibc_dn11_slot = var_ibc_dn11;
        *var_ibc_dn12_slot = var_ibc_dn12;
        *var_ibc_dn13_slot = var_ibc_dn13;
        *var_ibc_dn2_slot = var_ibc_dn2;
        *var_ibc_dn3_slot = var_ibc_dn3;
        *var_ibc_dn4_slot = var_ibc_dn4;
        *var_ibc_dn5_slot = var_ibc_dn5;
        *var_ibc_dn6_slot = var_ibc_dn6;
        *var_ibc_dn7_slot = var_ibc_dn7;
        *var_ibc_dn8_slot = var_ibc_dn8;
        *var_ibc_dn9_slot = var_ibc_dn9;
        *var_ibk_slot = var_ibk;
        *var_ibk_dn0_slot = var_ibk_dn0;
        *var_ibk_dn1_slot = var_ibk_dn1;
        *var_ibk_dn10_slot = var_ibk_dn10;
        *var_ibk_dn11_slot = var_ibk_dn11;
        *var_ibk_dn12_slot = var_ibk_dn12;
        *var_ibk_dn13_slot = var_ibk_dn13;
        *var_ibk_dn2_slot = var_ibk_dn2;
        *var_ibk_dn3_slot = var_ibk_dn3;
        *var_ibk_dn4_slot = var_ibk_dn4;
        *var_ibk_dn5_slot = var_ibk_dn5;
        *var_ibk_dn6_slot = var_ibk_dn6;
        *var_ibk_dn7_slot = var_ibk_dn7;
        *var_ibk_dn8_slot = var_ibk_dn8;
        *var_ibk_dn9_slot = var_ibk_dn9;
        *var_igcx_slot = var_igcx;
        *var_igcx_dn0_slot = var_igcx_dn0;
        *var_igcx_dn1_slot = var_igcx_dn1;
        *var_igcx_dn10_slot = var_igcx_dn10;
        *var_igcx_dn11_slot = var_igcx_dn11;
        *var_igcx_dn12_slot = var_igcx_dn12;
        *var_igcx_dn13_slot = var_igcx_dn13;
        *var_igcx_dn2_slot = var_igcx_dn2;
        *var_igcx_dn3_slot = var_igcx_dn3;
        *var_igcx_dn4_slot = var_igcx_dn4;
        *var_igcx_dn5_slot = var_igcx_dn5;
        *var_igcx_dn6_slot = var_igcx_dn6;
        *var_igcx_dn7_slot = var_igcx_dn7;
        *var_igcx_dn8_slot = var_igcx_dn8;
        *var_igcx_dn9_slot = var_igcx_dn9;
        *var_iibk_slot = var_iibk;
        *var_iibk_dn0_slot = var_iibk_dn0;
        *var_iibk_dn1_slot = var_iibk_dn1;
        *var_iibk_dn10_slot = var_iibk_dn10;
        *var_iibk_dn11_slot = var_iibk_dn11;
        *var_iibk_dn12_slot = var_iibk_dn12;
        *var_iibk_dn13_slot = var_iibk_dn13;
        *var_iibk_dn2_slot = var_iibk_dn2;
        *var_iibk_dn3_slot = var_iibk_dn3;
        *var_iibk_dn4_slot = var_iibk_dn4;
        *var_iibk_dn5_slot = var_iibk_dn5;
        *var_iibk_dn6_slot = var_iibk_dn6;
        *var_iibk_dn7_slot = var_iibk_dn7;
        *var_iibk_dn8_slot = var_iibk_dn8;
        *var_iibk_dn9_slot = var_iibk_dn9;
        *var_mac1__blk126_slot = var_mac1__blk126;
        *var_mac1__blk126_dn0_slot = var_mac1__blk126_dn0;
        *var_mac1__blk126_dn1_slot = var_mac1__blk126_dn1;
        *var_mac1__blk126_dn10_slot = var_mac1__blk126_dn10;
        *var_mac1__blk126_dn11_slot = var_mac1__blk126_dn11;
        *var_mac1__blk126_dn12_slot = var_mac1__blk126_dn12;
        *var_mac1__blk126_dn13_slot = var_mac1__blk126_dn13;
        *var_mac1__blk126_dn2_slot = var_mac1__blk126_dn2;
        *var_mac1__blk126_dn3_slot = var_mac1__blk126_dn3;
        *var_mac1__blk126_dn4_slot = var_mac1__blk126_dn4;
        *var_mac1__blk126_dn5_slot = var_mac1__blk126_dn5;
        *var_mac1__blk126_dn6_slot = var_mac1__blk126_dn6;
        *var_mac1__blk126_dn7_slot = var_mac1__blk126_dn7;
        *var_mac1__blk126_dn8_slot = var_mac1__blk126_dn8;
        *var_mac1__blk126_dn9_slot = var_mac1__blk126_dn9;
        *var_vcbfac_slot = var_vcbfac;
        *var_vcbfac_dn0_slot = var_vcbfac_dn0;
        *var_vcbfac_dn1_slot = var_vcbfac_dn1;
        *var_vcbfac_dn10_slot = var_vcbfac_dn10;
        *var_vcbfac_dn11_slot = var_vcbfac_dn11;
        *var_vcbfac_dn12_slot = var_vcbfac_dn12;
        *var_vcbfac_dn13_slot = var_vcbfac_dn13;
        *var_vcbfac_dn2_slot = var_vcbfac_dn2;
        *var_vcbfac_dn3_slot = var_vcbfac_dn3;
        *var_vcbfac_dn4_slot = var_vcbfac_dn4;
        *var_vcbfac_dn5_slot = var_vcbfac_dn5;
        *var_vcbfac_dn6_slot = var_vcbfac_dn6;
        *var_vcbfac_dn7_slot = var_vcbfac_dn7;
        *var_vcbfac_dn8_slot = var_vcbfac_dn8;
        *var_vcbfac_dn9_slot = var_vcbfac_dn9;
        *var_vl__blk125_slot = var_vl__blk125;
        *var_vl__blk125_dn0_slot = var_vl__blk125_dn0;
        *var_vl__blk125_dn1_slot = var_vl__blk125_dn1;
        *var_vl__blk125_dn10_slot = var_vl__blk125_dn10;
        *var_vl__blk125_dn11_slot = var_vl__blk125_dn11;
        *var_vl__blk125_dn12_slot = var_vl__blk125_dn12;
        *var_vl__blk125_dn13_slot = var_vl__blk125_dn13;
        *var_vl__blk125_dn2_slot = var_vl__blk125_dn2;
        *var_vl__blk125_dn3_slot = var_vl__blk125_dn3;
        *var_vl__blk125_dn4_slot = var_vl__blk125_dn4;
        *var_vl__blk125_dn5_slot = var_vl__blk125_dn5;
        *var_vl__blk125_dn6_slot = var_vl__blk125_dn6;
        *var_vl__blk125_dn7_slot = var_vl__blk125_dn7;
        *var_vl__blk125_dn8_slot = var_vl__blk125_dn8;
        *var_vl__blk125_dn9_slot = var_vl__blk125_dn9;
        *var_vminm__blk124_slot = var_vminm__blk124;
        *var_vminm__blk124_dn0_slot = var_vminm__blk124_dn0;
        *var_vminm__blk124_dn1_slot = var_vminm__blk124_dn1;
        *var_vminm__blk124_dn10_slot = var_vminm__blk124_dn10;
        *var_vminm__blk124_dn11_slot = var_vminm__blk124_dn11;
        *var_vminm__blk124_dn12_slot = var_vminm__blk124_dn12;
        *var_vminm__blk124_dn13_slot = var_vminm__blk124_dn13;
        *var_vminm__blk124_dn2_slot = var_vminm__blk124_dn2;
        *var_vminm__blk124_dn3_slot = var_vminm__blk124_dn3;
        *var_vminm__blk124_dn4_slot = var_vminm__blk124_dn4;
        *var_vminm__blk124_dn5_slot = var_vminm__blk124_dn5;
        *var_vminm__blk124_dn6_slot = var_vminm__blk124_dn6;
        *var_vminm__blk124_dn7_slot = var_vminm__blk124_dn7;
        *var_vminm__blk124_dn8_slot = var_vminm__blk124_dn8;
        *var_vminm__blk124_dn9_slot = var_vminm__blk124_dn9;
    }

    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        var_dt_et: f64,
        var_dt_et_dn0: f64,
        var_dt_et_dn1: f64,
        var_dt_et_dn10: f64,
        var_dt_et_dn11: f64,
        var_dt_et_dn12: f64,
        var_dt_et_dn13: f64,
        var_dt_et_dn2: f64,
        var_dt_et_dn3: f64,
        var_dt_et_dn4: f64,
        var_dt_et_dn5: f64,
        var_dt_et_dn6: f64,
        var_dt_et_dn7: f64,
        var_dt_et_dn8: f64,
        var_dt_et_dn9: f64,
        var_expi: f64,
        var_expi_dn0: f64,
        var_expi_dn1: f64,
        var_expi_dn10: f64,
        var_expi_dn11: f64,
        var_expi_dn12: f64,
        var_expi_dn13: f64,
        var_expi_dn2: f64,
        var_expi_dn3: f64,
        var_expi_dn4: f64,
        var_expi_dn5: f64,
        var_expi_dn6: f64,
        var_expi_dn7: f64,
        var_expi_dn8: f64,
        var_expi_dn9: f64,
        var_gminmod: f64,
        var_gth: f64,
        var_gth_dn0: f64,
        var_gth_dn1: f64,
        var_gth_dn10: f64,
        var_gth_dn11: f64,
        var_gth_dn12: f64,
        var_gth_dn13: f64,
        var_gth_dn2: f64,
        var_gth_dn3: f64,
        var_gth_dn4: f64,
        var_gth_dn5: f64,
        var_gth_dn6: f64,
        var_gth_dn7: f64,
        var_gth_dn8: f64,
        var_gth_dn9: f64,
        var_guard132: f64,
        var_ibcip_t: f64,
        var_ibcip_t_dn0: f64,
        var_ibcip_t_dn1: f64,
        var_ibcip_t_dn10: f64,
        var_ibcip_t_dn11: f64,
        var_ibcip_t_dn12: f64,
        var_ibcip_t_dn13: f64,
        var_ibcip_t_dn2: f64,
        var_ibcip_t_dn3: f64,
        var_ibcip_t_dn4: f64,
        var_ibcip_t_dn5: f64,
        var_ibcip_t_dn6: f64,
        var_ibcip_t_dn7: f64,
        var_ibcip_t_dn8: f64,
        var_ibcip_t_dn9: f64,
        var_ibcnp_t: f64,
        var_ibcnp_t_dn0: f64,
        var_ibcnp_t_dn1: f64,
        var_ibcnp_t_dn10: f64,
        var_ibcnp_t_dn11: f64,
        var_ibcnp_t_dn12: f64,
        var_ibcnp_t_dn13: f64,
        var_ibcnp_t_dn2: f64,
        var_ibcnp_t_dn3: f64,
        var_ibcnp_t_dn4: f64,
        var_ibcnp_t_dn5: f64,
        var_ibcnp_t_dn6: f64,
        var_ibcnp_t_dn7: f64,
        var_ibcnp_t_dn8: f64,
        var_ibcnp_t_dn9: f64,
        var_iccp: f64,
        var_iccp_dn0: f64,
        var_iccp_dn1: f64,
        var_iccp_dn10: f64,
        var_iccp_dn11: f64,
        var_iccp_dn12: f64,
        var_iccp_dn13: f64,
        var_iccp_dn2: f64,
        var_iccp_dn3: f64,
        var_iccp_dn4: f64,
        var_iccp_dn5: f64,
        var_iccp_dn6: f64,
        var_iccp_dn7: f64,
        var_iccp_dn8: f64,
        var_iccp_dn9: f64,
        var_irbi: f64,
        var_irbi_dn0: f64,
        var_irbi_dn1: f64,
        var_irbi_dn10: f64,
        var_irbi_dn11: f64,
        var_irbi_dn12: f64,
        var_irbi_dn13: f64,
        var_irbi_dn2: f64,
        var_irbi_dn3: f64,
        var_irbi_dn4: f64,
        var_irbi_dn5: f64,
        var_irbi_dn6: f64,
        var_irbi_dn7: f64,
        var_irbi_dn8: f64,
        var_irbi_dn9: f64,
        var_irbp: f64,
        var_irbp_dn0: f64,
        var_irbp_dn1: f64,
        var_irbp_dn10: f64,
        var_irbp_dn11: f64,
        var_irbp_dn12: f64,
        var_irbp_dn13: f64,
        var_irbp_dn2: f64,
        var_irbp_dn3: f64,
        var_irbp_dn4: f64,
        var_irbp_dn5: f64,
        var_irbp_dn6: f64,
        var_irbp_dn7: f64,
        var_irbp_dn8: f64,
        var_irbp_dn9: f64,
        var_irbx: f64,
        var_irbx_dn0: f64,
        var_irbx_dn1: f64,
        var_irbx_dn10: f64,
        var_irbx_dn11: f64,
        var_irbx_dn12: f64,
        var_irbx_dn13: f64,
        var_irbx_dn2: f64,
        var_irbx_dn3: f64,
        var_irbx_dn4: f64,
        var_irbx_dn5: f64,
        var_irbx_dn6: f64,
        var_irbx_dn7: f64,
        var_irbx_dn8: f64,
        var_irbx_dn9: f64,
        var_irci: f64,
        var_irci_dn0: f64,
        var_irci_dn1: f64,
        var_irci_dn10: f64,
        var_irci_dn11: f64,
        var_irci_dn12: f64,
        var_irci_dn13: f64,
        var_irci_dn2: f64,
        var_irci_dn3: f64,
        var_irci_dn4: f64,
        var_irci_dn5: f64,
        var_irci_dn6: f64,
        var_irci_dn7: f64,
        var_irci_dn8: f64,
        var_irci_dn9: f64,
        var_ire: f64,
        var_ire_dn0: f64,
        var_ire_dn1: f64,
        var_ire_dn10: f64,
        var_ire_dn11: f64,
        var_ire_dn12: f64,
        var_ire_dn13: f64,
        var_ire_dn2: f64,
        var_ire_dn3: f64,
        var_ire_dn4: f64,
        var_ire_dn5: f64,
        var_ire_dn6: f64,
        var_ire_dn7: f64,
        var_ire_dn8: f64,
        var_ire_dn9: f64,
        var_irs: f64,
        var_irs_dn0: f64,
        var_irs_dn1: f64,
        var_irs_dn10: f64,
        var_irs_dn11: f64,
        var_irs_dn12: f64,
        var_irs_dn13: f64,
        var_irs_dn2: f64,
        var_irs_dn3: f64,
        var_irs_dn4: f64,
        var_irs_dn5: f64,
        var_irs_dn6: f64,
        var_irs_dn7: f64,
        var_irs_dn8: f64,
        var_irs_dn9: f64,
        var_maxvibcnp: f64,
        var_maxvibcnp_dn0: f64,
        var_maxvibcnp_dn1: f64,
        var_maxvibcnp_dn10: f64,
        var_maxvibcnp_dn11: f64,
        var_maxvibcnp_dn12: f64,
        var_maxvibcnp_dn13: f64,
        var_maxvibcnp_dn2: f64,
        var_maxvibcnp_dn3: f64,
        var_maxvibcnp_dn4: f64,
        var_maxvibcnp_dn5: f64,
        var_maxvibcnp_dn6: f64,
        var_maxvibcnp_dn7: f64,
        var_maxvibcnp_dn8: f64,
        var_maxvibcnp_dn9: f64,
        var_vbci: f64,
        var_vbci_dn0: f64,
        var_vbci_dn1: f64,
        var_vbci_dn10: f64,
        var_vbci_dn11: f64,
        var_vbci_dn12: f64,
        var_vbci_dn13: f64,
        var_vbci_dn2: f64,
        var_vbci_dn3: f64,
        var_vbci_dn4: f64,
        var_vbci_dn5: f64,
        var_vbci_dn6: f64,
        var_vbci_dn7: f64,
        var_vbci_dn8: f64,
        var_vbci_dn9: f64,
        var_vbcp: f64,
        var_vbcp_dn0: f64,
        var_vbcp_dn1: f64,
        var_vbcp_dn10: f64,
        var_vbcp_dn11: f64,
        var_vbcp_dn12: f64,
        var_vbcp_dn13: f64,
        var_vbcp_dn2: f64,
        var_vbcp_dn3: f64,
        var_vbcp_dn4: f64,
        var_vbcp_dn5: f64,
        var_vbcp_dn6: f64,
        var_vbcp_dn7: f64,
        var_vbcp_dn8: f64,
        var_vbcp_dn9: f64,
        var_vbei: f64,
        var_vbei_dn0: f64,
        var_vbei_dn1: f64,
        var_vbei_dn10: f64,
        var_vbei_dn11: f64,
        var_vbei_dn12: f64,
        var_vbei_dn13: f64,
        var_vbei_dn2: f64,
        var_vbei_dn3: f64,
        var_vbei_dn4: f64,
        var_vbei_dn5: f64,
        var_vbei_dn6: f64,
        var_vbei_dn7: f64,
        var_vbei_dn8: f64,
        var_vbei_dn9: f64,
        var_vbep: f64,
        var_vbep_dn0: f64,
        var_vbep_dn1: f64,
        var_vbep_dn10: f64,
        var_vbep_dn11: f64,
        var_vbep_dn12: f64,
        var_vbep_dn13: f64,
        var_vbep_dn2: f64,
        var_vbep_dn3: f64,
        var_vbep_dn4: f64,
        var_vbep_dn5: f64,
        var_vbep_dn6: f64,
        var_vbep_dn7: f64,
        var_vbep_dn8: f64,
        var_vbep_dn9: f64,
        var_vbex: f64,
        var_vbex_dn0: f64,
        var_vbex_dn1: f64,
        var_vbex_dn10: f64,
        var_vbex_dn11: f64,
        var_vbex_dn12: f64,
        var_vbex_dn13: f64,
        var_vbex_dn2: f64,
        var_vbex_dn3: f64,
        var_vbex_dn4: f64,
        var_vbex_dn5: f64,
        var_vbex_dn6: f64,
        var_vbex_dn7: f64,
        var_vbex_dn8: f64,
        var_vbex_dn9: f64,
        var_vbictype: f64,
        var_vbxcx: f64,
        var_vbxcx_dn0: f64,
        var_vbxcx_dn1: f64,
        var_vbxcx_dn10: f64,
        var_vbxcx_dn11: f64,
        var_vbxcx_dn12: f64,
        var_vbxcx_dn13: f64,
        var_vbxcx_dn2: f64,
        var_vbxcx_dn3: f64,
        var_vbxcx_dn4: f64,
        var_vbxcx_dn5: f64,
        var_vbxcx_dn6: f64,
        var_vbxcx_dn7: f64,
        var_vbxcx_dn8: f64,
        var_vbxcx_dn9: f64,
        var_vcei: f64,
        var_vcei_dn0: f64,
        var_vcei_dn1: f64,
        var_vcei_dn10: f64,
        var_vcei_dn11: f64,
        var_vcei_dn12: f64,
        var_vcei_dn13: f64,
        var_vcei_dn2: f64,
        var_vcei_dn3: f64,
        var_vcei_dn4: f64,
        var_vcei_dn5: f64,
        var_vcei_dn6: f64,
        var_vcei_dn7: f64,
        var_vcei_dn8: f64,
        var_vcei_dn9: f64,
        var_vcep: f64,
        var_vcep_dn0: f64,
        var_vcep_dn1: f64,
        var_vcep_dn10: f64,
        var_vcep_dn11: f64,
        var_vcep_dn12: f64,
        var_vcep_dn13: f64,
        var_vcep_dn2: f64,
        var_vcep_dn3: f64,
        var_vcep_dn4: f64,
        var_vcep_dn5: f64,
        var_vcep_dn6: f64,
        var_vcep_dn7: f64,
        var_vcep_dn8: f64,
        var_vcep_dn9: f64,
        var_vrbi: f64,
        var_vrbi_dn0: f64,
        var_vrbi_dn1: f64,
        var_vrbi_dn10: f64,
        var_vrbi_dn11: f64,
        var_vrbi_dn12: f64,
        var_vrbi_dn13: f64,
        var_vrbi_dn2: f64,
        var_vrbi_dn3: f64,
        var_vrbi_dn4: f64,
        var_vrbi_dn5: f64,
        var_vrbi_dn6: f64,
        var_vrbi_dn7: f64,
        var_vrbi_dn8: f64,
        var_vrbi_dn9: f64,
        var_vrbp: f64,
        var_vrbp_dn0: f64,
        var_vrbp_dn1: f64,
        var_vrbp_dn10: f64,
        var_vrbp_dn11: f64,
        var_vrbp_dn12: f64,
        var_vrbp_dn13: f64,
        var_vrbp_dn2: f64,
        var_vrbp_dn3: f64,
        var_vrbp_dn4: f64,
        var_vrbp_dn5: f64,
        var_vrbp_dn6: f64,
        var_vrbp_dn7: f64,
        var_vrbp_dn8: f64,
        var_vrbp_dn9: f64,
        var_vrbx: f64,
        var_vrbx_dn0: f64,
        var_vrbx_dn1: f64,
        var_vrbx_dn10: f64,
        var_vrbx_dn11: f64,
        var_vrbx_dn12: f64,
        var_vrbx_dn13: f64,
        var_vrbx_dn2: f64,
        var_vrbx_dn3: f64,
        var_vrbx_dn4: f64,
        var_vrbx_dn5: f64,
        var_vrbx_dn6: f64,
        var_vrbx_dn7: f64,
        var_vrbx_dn8: f64,
        var_vrbx_dn9: f64,
        var_vrci: f64,
        var_vrci_dn0: f64,
        var_vrci_dn1: f64,
        var_vrci_dn10: f64,
        var_vrci_dn11: f64,
        var_vrci_dn12: f64,
        var_vrci_dn13: f64,
        var_vrci_dn2: f64,
        var_vrci_dn3: f64,
        var_vrci_dn4: f64,
        var_vrci_dn5: f64,
        var_vrci_dn6: f64,
        var_vrci_dn7: f64,
        var_vrci_dn8: f64,
        var_vrci_dn9: f64,
        var_vrcx: f64,
        var_vrcx_dn0: f64,
        var_vrcx_dn1: f64,
        var_vrcx_dn10: f64,
        var_vrcx_dn11: f64,
        var_vrcx_dn12: f64,
        var_vrcx_dn13: f64,
        var_vrcx_dn2: f64,
        var_vrcx_dn3: f64,
        var_vrcx_dn4: f64,
        var_vrcx_dn5: f64,
        var_vrcx_dn6: f64,
        var_vrcx_dn7: f64,
        var_vrcx_dn8: f64,
        var_vrcx_dn9: f64,
        var_vre: f64,
        var_vre_dn0: f64,
        var_vre_dn1: f64,
        var_vre_dn10: f64,
        var_vre_dn11: f64,
        var_vre_dn12: f64,
        var_vre_dn13: f64,
        var_vre_dn2: f64,
        var_vre_dn3: f64,
        var_vre_dn4: f64,
        var_vre_dn5: f64,
        var_vre_dn6: f64,
        var_vre_dn7: f64,
        var_vre_dn8: f64,
        var_vre_dn9: f64,
        var_vrs: f64,
        var_vrs_dn0: f64,
        var_vrs_dn1: f64,
        var_vrs_dn10: f64,
        var_vrs_dn11: f64,
        var_vrs_dn12: f64,
        var_vrs_dn13: f64,
        var_vrs_dn2: f64,
        var_vrs_dn3: f64,
        var_vrs_dn4: f64,
        var_vrs_dn5: f64,
        var_vrs_dn6: f64,
        var_vrs_dn7: f64,
        var_vrs_dn8: f64,
        var_vrs_dn9: f64,
        var_vtv: f64,
        var_vtv_dn0: f64,
        var_vtv_dn1: f64,
        var_vtv_dn10: f64,
        var_vtv_dn11: f64,
        var_vtv_dn12: f64,
        var_vtv_dn13: f64,
        var_vtv_dn2: f64,
        var_vtv_dn3: f64,
        var_vtv_dn4: f64,
        var_vtv_dn5: f64,
        var_vtv_dn6: f64,
        var_vtv_dn7: f64,
        var_vtv_dn8: f64,
        var_vtv_dn9: f64,
        var_vxf2: f64,
        var_vxf2_dn0: f64,
        var_vxf2_dn1: f64,
        var_vxf2_dn10: f64,
        var_vxf2_dn11: f64,
        var_vxf2_dn12: f64,
        var_vxf2_dn13: f64,
        var_vxf2_dn2: f64,
        var_vxf2_dn3: f64,
        var_vxf2_dn4: f64,
        var_vxf2_dn5: f64,
        var_vxf2_dn6: f64,
        var_vxf2_dn7: f64,
        var_vxf2_dn8: f64,
        var_vxf2_dn9: f64,
        var_afac_slot: &mut f64,
        var_afac_dn0_slot: &mut f64,
        var_afac_dn1_slot: &mut f64,
        var_afac_dn10_slot: &mut f64,
        var_afac_dn11_slot: &mut f64,
        var_afac_dn12_slot: &mut f64,
        var_afac_dn13_slot: &mut f64,
        var_afac_dn2_slot: &mut f64,
        var_afac_dn3_slot: &mut f64,
        var_afac_dn4_slot: &mut f64,
        var_afac_dn5_slot: &mut f64,
        var_afac_dn6_slot: &mut f64,
        var_afac_dn7_slot: &mut f64,
        var_afac_dn8_slot: &mut f64,
        var_afac_dn9_slot: &mut f64,
        var_expn_slot: &mut f64,
        var_expn_dn0_slot: &mut f64,
        var_expn_dn1_slot: &mut f64,
        var_expn_dn10_slot: &mut f64,
        var_expn_dn11_slot: &mut f64,
        var_expn_dn12_slot: &mut f64,
        var_expn_dn13_slot: &mut f64,
        var_expn_dn2_slot: &mut f64,
        var_expn_dn3_slot: &mut f64,
        var_expn_dn4_slot: &mut f64,
        var_expn_dn5_slot: &mut f64,
        var_expn_dn6_slot: &mut f64,
        var_expn_dn7_slot: &mut f64,
        var_expn_dn8_slot: &mut f64,
        var_expn_dn9_slot: &mut f64,
        var_guard134_slot: &mut f64,
        var_ibc_slot: &mut f64,
        var_ibc_dn0_slot: &mut f64,
        var_ibc_dn1_slot: &mut f64,
        var_ibc_dn10_slot: &mut f64,
        var_ibc_dn11_slot: &mut f64,
        var_ibc_dn12_slot: &mut f64,
        var_ibc_dn13_slot: &mut f64,
        var_ibc_dn2_slot: &mut f64,
        var_ibc_dn3_slot: &mut f64,
        var_ibc_dn4_slot: &mut f64,
        var_ibc_dn5_slot: &mut f64,
        var_ibc_dn6_slot: &mut f64,
        var_ibc_dn7_slot: &mut f64,
        var_ibc_dn8_slot: &mut f64,
        var_ibc_dn9_slot: &mut f64,
        var_ibcp_slot: &mut f64,
        var_ibcp_dn0_slot: &mut f64,
        var_ibcp_dn1_slot: &mut f64,
        var_ibcp_dn10_slot: &mut f64,
        var_ibcp_dn11_slot: &mut f64,
        var_ibcp_dn12_slot: &mut f64,
        var_ibcp_dn13_slot: &mut f64,
        var_ibcp_dn2_slot: &mut f64,
        var_ibcp_dn3_slot: &mut f64,
        var_ibcp_dn4_slot: &mut f64,
        var_ibcp_dn5_slot: &mut f64,
        var_ibcp_dn6_slot: &mut f64,
        var_ibcp_dn7_slot: &mut f64,
        var_ibcp_dn8_slot: &mut f64,
        var_ibcp_dn9_slot: &mut f64,
        var_ibe_slot: &mut f64,
        var_ibe_dn0_slot: &mut f64,
        var_ibe_dn1_slot: &mut f64,
        var_ibe_dn10_slot: &mut f64,
        var_ibe_dn11_slot: &mut f64,
        var_ibe_dn12_slot: &mut f64,
        var_ibe_dn13_slot: &mut f64,
        var_ibe_dn2_slot: &mut f64,
        var_ibe_dn3_slot: &mut f64,
        var_ibe_dn4_slot: &mut f64,
        var_ibe_dn5_slot: &mut f64,
        var_ibe_dn6_slot: &mut f64,
        var_ibe_dn7_slot: &mut f64,
        var_ibe_dn8_slot: &mut f64,
        var_ibe_dn9_slot: &mut f64,
        var_ibep_slot: &mut f64,
        var_ibep_dn0_slot: &mut f64,
        var_ibep_dn1_slot: &mut f64,
        var_ibep_dn10_slot: &mut f64,
        var_ibep_dn11_slot: &mut f64,
        var_ibep_dn12_slot: &mut f64,
        var_ibep_dn13_slot: &mut f64,
        var_ibep_dn2_slot: &mut f64,
        var_ibep_dn3_slot: &mut f64,
        var_ibep_dn4_slot: &mut f64,
        var_ibep_dn5_slot: &mut f64,
        var_ibep_dn6_slot: &mut f64,
        var_ibep_dn7_slot: &mut f64,
        var_ibep_dn8_slot: &mut f64,
        var_ibep_dn9_slot: &mut f64,
        var_ibex_slot: &mut f64,
        var_ibex_dn0_slot: &mut f64,
        var_ibex_dn1_slot: &mut f64,
        var_ibex_dn10_slot: &mut f64,
        var_ibex_dn11_slot: &mut f64,
        var_ibex_dn12_slot: &mut f64,
        var_ibex_dn13_slot: &mut f64,
        var_ibex_dn2_slot: &mut f64,
        var_ibex_dn3_slot: &mut f64,
        var_ibex_dn4_slot: &mut f64,
        var_ibex_dn5_slot: &mut f64,
        var_ibex_dn6_slot: &mut f64,
        var_ibex_dn7_slot: &mut f64,
        var_ibex_dn8_slot: &mut f64,
        var_ibex_dn9_slot: &mut f64,
        var_igcx_slot: &mut f64,
        var_igcx_dn0_slot: &mut f64,
        var_igcx_dn1_slot: &mut f64,
        var_igcx_dn10_slot: &mut f64,
        var_igcx_dn11_slot: &mut f64,
        var_igcx_dn12_slot: &mut f64,
        var_igcx_dn13_slot: &mut f64,
        var_igcx_dn2_slot: &mut f64,
        var_igcx_dn3_slot: &mut f64,
        var_igcx_dn4_slot: &mut f64,
        var_igcx_dn5_slot: &mut f64,
        var_igcx_dn6_slot: &mut f64,
        var_igcx_dn7_slot: &mut f64,
        var_igcx_dn8_slot: &mut f64,
        var_igcx_dn9_slot: &mut f64,
        var_ircx_slot: &mut f64,
        var_ircx_dn0_slot: &mut f64,
        var_ircx_dn1_slot: &mut f64,
        var_ircx_dn10_slot: &mut f64,
        var_ircx_dn11_slot: &mut f64,
        var_ircx_dn12_slot: &mut f64,
        var_ircx_dn13_slot: &mut f64,
        var_ircx_dn2_slot: &mut f64,
        var_ircx_dn3_slot: &mut f64,
        var_ircx_dn4_slot: &mut f64,
        var_ircx_dn5_slot: &mut f64,
        var_ircx_dn6_slot: &mut f64,
        var_ircx_dn7_slot: &mut f64,
        var_ircx_dn8_slot: &mut f64,
        var_ircx_dn9_slot: &mut f64,
        var_irth_slot: &mut f64,
        var_irth_dn0_slot: &mut f64,
        var_irth_dn1_slot: &mut f64,
        var_irth_dn10_slot: &mut f64,
        var_irth_dn11_slot: &mut f64,
        var_irth_dn12_slot: &mut f64,
        var_irth_dn13_slot: &mut f64,
        var_irth_dn2_slot: &mut f64,
        var_irth_dn3_slot: &mut f64,
        var_irth_dn4_slot: &mut f64,
        var_irth_dn5_slot: &mut f64,
        var_irth_dn6_slot: &mut f64,
        var_irth_dn7_slot: &mut f64,
        var_irth_dn8_slot: &mut f64,
        var_irth_dn9_slot: &mut f64,
        var_ith_slot: &mut f64,
        var_ith_dn0_slot: &mut f64,
        var_ith_dn1_slot: &mut f64,
        var_ith_dn10_slot: &mut f64,
        var_ith_dn11_slot: &mut f64,
        var_ith_dn12_slot: &mut f64,
        var_ith_dn13_slot: &mut f64,
        var_ith_dn2_slot: &mut f64,
        var_ith_dn3_slot: &mut f64,
        var_ith_dn4_slot: &mut f64,
        var_ith_dn5_slot: &mut f64,
        var_ith_dn6_slot: &mut f64,
        var_ith_dn7_slot: &mut f64,
        var_ith_dn8_slot: &mut f64,
        var_ith_dn9_slot: &mut f64,
        var_itxf_slot: &mut f64,
        var_itxf_dn0_slot: &mut f64,
        var_itxf_dn1_slot: &mut f64,
        var_itxf_dn10_slot: &mut f64,
        var_itxf_dn11_slot: &mut f64,
        var_itxf_dn12_slot: &mut f64,
        var_itxf_dn13_slot: &mut f64,
        var_itxf_dn2_slot: &mut f64,
        var_itxf_dn3_slot: &mut f64,
        var_itxf_dn4_slot: &mut f64,
        var_itxf_dn5_slot: &mut f64,
        var_itxf_dn6_slot: &mut f64,
        var_itxf_dn7_slot: &mut f64,
        var_itxf_dn8_slot: &mut f64,
        var_itxf_dn9_slot: &mut f64,
        var_itzf_slot: &mut f64,
        var_itzf_dn0_slot: &mut f64,
        var_itzf_dn1_slot: &mut f64,
        var_itzf_dn10_slot: &mut f64,
        var_itzf_dn11_slot: &mut f64,
        var_itzf_dn12_slot: &mut f64,
        var_itzf_dn13_slot: &mut f64,
        var_itzf_dn2_slot: &mut f64,
        var_itzf_dn3_slot: &mut f64,
        var_itzf_dn4_slot: &mut f64,
        var_itzf_dn5_slot: &mut f64,
        var_itzf_dn6_slot: &mut f64,
        var_itzf_dn7_slot: &mut f64,
        var_itzf_dn8_slot: &mut f64,
        var_itzf_dn9_slot: &mut f64,
        var_itzr_slot: &mut f64,
        var_itzr_dn0_slot: &mut f64,
        var_itzr_dn1_slot: &mut f64,
        var_itzr_dn10_slot: &mut f64,
        var_itzr_dn11_slot: &mut f64,
        var_itzr_dn12_slot: &mut f64,
        var_itzr_dn13_slot: &mut f64,
        var_itzr_dn2_slot: &mut f64,
        var_itzr_dn3_slot: &mut f64,
        var_itzr_dn4_slot: &mut f64,
        var_itzr_dn5_slot: &mut f64,
        var_itzr_dn6_slot: &mut f64,
        var_itzr_dn7_slot: &mut f64,
        var_itzr_dn8_slot: &mut f64,
        var_itzr_dn9_slot: &mut f64,
        var_ixf1_slot: &mut f64,
        var_ixf1_dn0_slot: &mut f64,
        var_ixf1_dn1_slot: &mut f64,
        var_ixf1_dn10_slot: &mut f64,
        var_ixf1_dn11_slot: &mut f64,
        var_ixf1_dn12_slot: &mut f64,
        var_ixf1_dn13_slot: &mut f64,
        var_ixf1_dn2_slot: &mut f64,
        var_ixf1_dn3_slot: &mut f64,
        var_ixf1_dn4_slot: &mut f64,
        var_ixf1_dn5_slot: &mut f64,
        var_ixf1_dn6_slot: &mut f64,
        var_ixf1_dn7_slot: &mut f64,
        var_ixf1_dn8_slot: &mut f64,
        var_ixf1_dn9_slot: &mut f64,
        var_power_slot: &mut f64,
        var_power_dn0_slot: &mut f64,
        var_power_dn1_slot: &mut f64,
        var_power_dn10_slot: &mut f64,
        var_power_dn11_slot: &mut f64,
        var_power_dn12_slot: &mut f64,
        var_power_dn13_slot: &mut f64,
        var_power_dn2_slot: &mut f64,
        var_power_dn3_slot: &mut f64,
        var_power_dn4_slot: &mut f64,
        var_power_dn5_slot: &mut f64,
        var_power_dn6_slot: &mut f64,
        var_power_dn7_slot: &mut f64,
        var_power_dn8_slot: &mut f64,
        var_power_dn9_slot: &mut f64,
    ) {
        let mut var_afac: f64 = *var_afac_slot;
        let mut var_afac_dn0: f64 = *var_afac_dn0_slot;
        let mut var_afac_dn1: f64 = *var_afac_dn1_slot;
        let mut var_afac_dn10: f64 = *var_afac_dn10_slot;
        let mut var_afac_dn11: f64 = *var_afac_dn11_slot;
        let mut var_afac_dn12: f64 = *var_afac_dn12_slot;
        let mut var_afac_dn13: f64 = *var_afac_dn13_slot;
        let mut var_afac_dn2: f64 = *var_afac_dn2_slot;
        let mut var_afac_dn3: f64 = *var_afac_dn3_slot;
        let mut var_afac_dn4: f64 = *var_afac_dn4_slot;
        let mut var_afac_dn5: f64 = *var_afac_dn5_slot;
        let mut var_afac_dn6: f64 = *var_afac_dn6_slot;
        let mut var_afac_dn7: f64 = *var_afac_dn7_slot;
        let mut var_afac_dn8: f64 = *var_afac_dn8_slot;
        let mut var_afac_dn9: f64 = *var_afac_dn9_slot;
        let mut var_expn: f64 = *var_expn_slot;
        let mut var_expn_dn0: f64 = *var_expn_dn0_slot;
        let mut var_expn_dn1: f64 = *var_expn_dn1_slot;
        let mut var_expn_dn10: f64 = *var_expn_dn10_slot;
        let mut var_expn_dn11: f64 = *var_expn_dn11_slot;
        let mut var_expn_dn12: f64 = *var_expn_dn12_slot;
        let mut var_expn_dn13: f64 = *var_expn_dn13_slot;
        let mut var_expn_dn2: f64 = *var_expn_dn2_slot;
        let mut var_expn_dn3: f64 = *var_expn_dn3_slot;
        let mut var_expn_dn4: f64 = *var_expn_dn4_slot;
        let mut var_expn_dn5: f64 = *var_expn_dn5_slot;
        let mut var_expn_dn6: f64 = *var_expn_dn6_slot;
        let mut var_expn_dn7: f64 = *var_expn_dn7_slot;
        let mut var_expn_dn8: f64 = *var_expn_dn8_slot;
        let mut var_expn_dn9: f64 = *var_expn_dn9_slot;
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_ibc: f64 = *var_ibc_slot;
        let mut var_ibc_dn0: f64 = *var_ibc_dn0_slot;
        let mut var_ibc_dn1: f64 = *var_ibc_dn1_slot;
        let mut var_ibc_dn10: f64 = *var_ibc_dn10_slot;
        let mut var_ibc_dn11: f64 = *var_ibc_dn11_slot;
        let mut var_ibc_dn12: f64 = *var_ibc_dn12_slot;
        let mut var_ibc_dn13: f64 = *var_ibc_dn13_slot;
        let mut var_ibc_dn2: f64 = *var_ibc_dn2_slot;
        let mut var_ibc_dn3: f64 = *var_ibc_dn3_slot;
        let mut var_ibc_dn4: f64 = *var_ibc_dn4_slot;
        let mut var_ibc_dn5: f64 = *var_ibc_dn5_slot;
        let mut var_ibc_dn6: f64 = *var_ibc_dn6_slot;
        let mut var_ibc_dn7: f64 = *var_ibc_dn7_slot;
        let mut var_ibc_dn8: f64 = *var_ibc_dn8_slot;
        let mut var_ibc_dn9: f64 = *var_ibc_dn9_slot;
        let mut var_ibcp: f64 = *var_ibcp_slot;
        let mut var_ibcp_dn0: f64 = *var_ibcp_dn0_slot;
        let mut var_ibcp_dn1: f64 = *var_ibcp_dn1_slot;
        let mut var_ibcp_dn10: f64 = *var_ibcp_dn10_slot;
        let mut var_ibcp_dn11: f64 = *var_ibcp_dn11_slot;
        let mut var_ibcp_dn12: f64 = *var_ibcp_dn12_slot;
        let mut var_ibcp_dn13: f64 = *var_ibcp_dn13_slot;
        let mut var_ibcp_dn2: f64 = *var_ibcp_dn2_slot;
        let mut var_ibcp_dn3: f64 = *var_ibcp_dn3_slot;
        let mut var_ibcp_dn4: f64 = *var_ibcp_dn4_slot;
        let mut var_ibcp_dn5: f64 = *var_ibcp_dn5_slot;
        let mut var_ibcp_dn6: f64 = *var_ibcp_dn6_slot;
        let mut var_ibcp_dn7: f64 = *var_ibcp_dn7_slot;
        let mut var_ibcp_dn8: f64 = *var_ibcp_dn8_slot;
        let mut var_ibcp_dn9: f64 = *var_ibcp_dn9_slot;
        let mut var_ibe: f64 = *var_ibe_slot;
        let mut var_ibe_dn0: f64 = *var_ibe_dn0_slot;
        let mut var_ibe_dn1: f64 = *var_ibe_dn1_slot;
        let mut var_ibe_dn10: f64 = *var_ibe_dn10_slot;
        let mut var_ibe_dn11: f64 = *var_ibe_dn11_slot;
        let mut var_ibe_dn12: f64 = *var_ibe_dn12_slot;
        let mut var_ibe_dn13: f64 = *var_ibe_dn13_slot;
        let mut var_ibe_dn2: f64 = *var_ibe_dn2_slot;
        let mut var_ibe_dn3: f64 = *var_ibe_dn3_slot;
        let mut var_ibe_dn4: f64 = *var_ibe_dn4_slot;
        let mut var_ibe_dn5: f64 = *var_ibe_dn5_slot;
        let mut var_ibe_dn6: f64 = *var_ibe_dn6_slot;
        let mut var_ibe_dn7: f64 = *var_ibe_dn7_slot;
        let mut var_ibe_dn8: f64 = *var_ibe_dn8_slot;
        let mut var_ibe_dn9: f64 = *var_ibe_dn9_slot;
        let mut var_ibep: f64 = *var_ibep_slot;
        let mut var_ibep_dn0: f64 = *var_ibep_dn0_slot;
        let mut var_ibep_dn1: f64 = *var_ibep_dn1_slot;
        let mut var_ibep_dn10: f64 = *var_ibep_dn10_slot;
        let mut var_ibep_dn11: f64 = *var_ibep_dn11_slot;
        let mut var_ibep_dn12: f64 = *var_ibep_dn12_slot;
        let mut var_ibep_dn13: f64 = *var_ibep_dn13_slot;
        let mut var_ibep_dn2: f64 = *var_ibep_dn2_slot;
        let mut var_ibep_dn3: f64 = *var_ibep_dn3_slot;
        let mut var_ibep_dn4: f64 = *var_ibep_dn4_slot;
        let mut var_ibep_dn5: f64 = *var_ibep_dn5_slot;
        let mut var_ibep_dn6: f64 = *var_ibep_dn6_slot;
        let mut var_ibep_dn7: f64 = *var_ibep_dn7_slot;
        let mut var_ibep_dn8: f64 = *var_ibep_dn8_slot;
        let mut var_ibep_dn9: f64 = *var_ibep_dn9_slot;
        let mut var_ibex: f64 = *var_ibex_slot;
        let mut var_ibex_dn0: f64 = *var_ibex_dn0_slot;
        let mut var_ibex_dn1: f64 = *var_ibex_dn1_slot;
        let mut var_ibex_dn10: f64 = *var_ibex_dn10_slot;
        let mut var_ibex_dn11: f64 = *var_ibex_dn11_slot;
        let mut var_ibex_dn12: f64 = *var_ibex_dn12_slot;
        let mut var_ibex_dn13: f64 = *var_ibex_dn13_slot;
        let mut var_ibex_dn2: f64 = *var_ibex_dn2_slot;
        let mut var_ibex_dn3: f64 = *var_ibex_dn3_slot;
        let mut var_ibex_dn4: f64 = *var_ibex_dn4_slot;
        let mut var_ibex_dn5: f64 = *var_ibex_dn5_slot;
        let mut var_ibex_dn6: f64 = *var_ibex_dn6_slot;
        let mut var_ibex_dn7: f64 = *var_ibex_dn7_slot;
        let mut var_ibex_dn8: f64 = *var_ibex_dn8_slot;
        let mut var_ibex_dn9: f64 = *var_ibex_dn9_slot;
        let mut var_igcx: f64 = *var_igcx_slot;
        let mut var_igcx_dn0: f64 = *var_igcx_dn0_slot;
        let mut var_igcx_dn1: f64 = *var_igcx_dn1_slot;
        let mut var_igcx_dn10: f64 = *var_igcx_dn10_slot;
        let mut var_igcx_dn11: f64 = *var_igcx_dn11_slot;
        let mut var_igcx_dn12: f64 = *var_igcx_dn12_slot;
        let mut var_igcx_dn13: f64 = *var_igcx_dn13_slot;
        let mut var_igcx_dn2: f64 = *var_igcx_dn2_slot;
        let mut var_igcx_dn3: f64 = *var_igcx_dn3_slot;
        let mut var_igcx_dn4: f64 = *var_igcx_dn4_slot;
        let mut var_igcx_dn5: f64 = *var_igcx_dn5_slot;
        let mut var_igcx_dn6: f64 = *var_igcx_dn6_slot;
        let mut var_igcx_dn7: f64 = *var_igcx_dn7_slot;
        let mut var_igcx_dn8: f64 = *var_igcx_dn8_slot;
        let mut var_igcx_dn9: f64 = *var_igcx_dn9_slot;
        let mut var_ircx: f64 = *var_ircx_slot;
        let mut var_ircx_dn0: f64 = *var_ircx_dn0_slot;
        let mut var_ircx_dn1: f64 = *var_ircx_dn1_slot;
        let mut var_ircx_dn10: f64 = *var_ircx_dn10_slot;
        let mut var_ircx_dn11: f64 = *var_ircx_dn11_slot;
        let mut var_ircx_dn12: f64 = *var_ircx_dn12_slot;
        let mut var_ircx_dn13: f64 = *var_ircx_dn13_slot;
        let mut var_ircx_dn2: f64 = *var_ircx_dn2_slot;
        let mut var_ircx_dn3: f64 = *var_ircx_dn3_slot;
        let mut var_ircx_dn4: f64 = *var_ircx_dn4_slot;
        let mut var_ircx_dn5: f64 = *var_ircx_dn5_slot;
        let mut var_ircx_dn6: f64 = *var_ircx_dn6_slot;
        let mut var_ircx_dn7: f64 = *var_ircx_dn7_slot;
        let mut var_ircx_dn8: f64 = *var_ircx_dn8_slot;
        let mut var_ircx_dn9: f64 = *var_ircx_dn9_slot;
        let mut var_irth: f64 = *var_irth_slot;
        let mut var_irth_dn0: f64 = *var_irth_dn0_slot;
        let mut var_irth_dn1: f64 = *var_irth_dn1_slot;
        let mut var_irth_dn10: f64 = *var_irth_dn10_slot;
        let mut var_irth_dn11: f64 = *var_irth_dn11_slot;
        let mut var_irth_dn12: f64 = *var_irth_dn12_slot;
        let mut var_irth_dn13: f64 = *var_irth_dn13_slot;
        let mut var_irth_dn2: f64 = *var_irth_dn2_slot;
        let mut var_irth_dn3: f64 = *var_irth_dn3_slot;
        let mut var_irth_dn4: f64 = *var_irth_dn4_slot;
        let mut var_irth_dn5: f64 = *var_irth_dn5_slot;
        let mut var_irth_dn6: f64 = *var_irth_dn6_slot;
        let mut var_irth_dn7: f64 = *var_irth_dn7_slot;
        let mut var_irth_dn8: f64 = *var_irth_dn8_slot;
        let mut var_irth_dn9: f64 = *var_irth_dn9_slot;
        let mut var_ith: f64 = *var_ith_slot;
        let mut var_ith_dn0: f64 = *var_ith_dn0_slot;
        let mut var_ith_dn1: f64 = *var_ith_dn1_slot;
        let mut var_ith_dn10: f64 = *var_ith_dn10_slot;
        let mut var_ith_dn11: f64 = *var_ith_dn11_slot;
        let mut var_ith_dn12: f64 = *var_ith_dn12_slot;
        let mut var_ith_dn13: f64 = *var_ith_dn13_slot;
        let mut var_ith_dn2: f64 = *var_ith_dn2_slot;
        let mut var_ith_dn3: f64 = *var_ith_dn3_slot;
        let mut var_ith_dn4: f64 = *var_ith_dn4_slot;
        let mut var_ith_dn5: f64 = *var_ith_dn5_slot;
        let mut var_ith_dn6: f64 = *var_ith_dn6_slot;
        let mut var_ith_dn7: f64 = *var_ith_dn7_slot;
        let mut var_ith_dn8: f64 = *var_ith_dn8_slot;
        let mut var_ith_dn9: f64 = *var_ith_dn9_slot;
        let mut var_itxf: f64 = *var_itxf_slot;
        let mut var_itxf_dn0: f64 = *var_itxf_dn0_slot;
        let mut var_itxf_dn1: f64 = *var_itxf_dn1_slot;
        let mut var_itxf_dn10: f64 = *var_itxf_dn10_slot;
        let mut var_itxf_dn11: f64 = *var_itxf_dn11_slot;
        let mut var_itxf_dn12: f64 = *var_itxf_dn12_slot;
        let mut var_itxf_dn13: f64 = *var_itxf_dn13_slot;
        let mut var_itxf_dn2: f64 = *var_itxf_dn2_slot;
        let mut var_itxf_dn3: f64 = *var_itxf_dn3_slot;
        let mut var_itxf_dn4: f64 = *var_itxf_dn4_slot;
        let mut var_itxf_dn5: f64 = *var_itxf_dn5_slot;
        let mut var_itxf_dn6: f64 = *var_itxf_dn6_slot;
        let mut var_itxf_dn7: f64 = *var_itxf_dn7_slot;
        let mut var_itxf_dn8: f64 = *var_itxf_dn8_slot;
        let mut var_itxf_dn9: f64 = *var_itxf_dn9_slot;
        let mut var_itzf: f64 = *var_itzf_slot;
        let mut var_itzf_dn0: f64 = *var_itzf_dn0_slot;
        let mut var_itzf_dn1: f64 = *var_itzf_dn1_slot;
        let mut var_itzf_dn10: f64 = *var_itzf_dn10_slot;
        let mut var_itzf_dn11: f64 = *var_itzf_dn11_slot;
        let mut var_itzf_dn12: f64 = *var_itzf_dn12_slot;
        let mut var_itzf_dn13: f64 = *var_itzf_dn13_slot;
        let mut var_itzf_dn2: f64 = *var_itzf_dn2_slot;
        let mut var_itzf_dn3: f64 = *var_itzf_dn3_slot;
        let mut var_itzf_dn4: f64 = *var_itzf_dn4_slot;
        let mut var_itzf_dn5: f64 = *var_itzf_dn5_slot;
        let mut var_itzf_dn6: f64 = *var_itzf_dn6_slot;
        let mut var_itzf_dn7: f64 = *var_itzf_dn7_slot;
        let mut var_itzf_dn8: f64 = *var_itzf_dn8_slot;
        let mut var_itzf_dn9: f64 = *var_itzf_dn9_slot;
        let mut var_itzr: f64 = *var_itzr_slot;
        let mut var_itzr_dn0: f64 = *var_itzr_dn0_slot;
        let mut var_itzr_dn1: f64 = *var_itzr_dn1_slot;
        let mut var_itzr_dn10: f64 = *var_itzr_dn10_slot;
        let mut var_itzr_dn11: f64 = *var_itzr_dn11_slot;
        let mut var_itzr_dn12: f64 = *var_itzr_dn12_slot;
        let mut var_itzr_dn13: f64 = *var_itzr_dn13_slot;
        let mut var_itzr_dn2: f64 = *var_itzr_dn2_slot;
        let mut var_itzr_dn3: f64 = *var_itzr_dn3_slot;
        let mut var_itzr_dn4: f64 = *var_itzr_dn4_slot;
        let mut var_itzr_dn5: f64 = *var_itzr_dn5_slot;
        let mut var_itzr_dn6: f64 = *var_itzr_dn6_slot;
        let mut var_itzr_dn7: f64 = *var_itzr_dn7_slot;
        let mut var_itzr_dn8: f64 = *var_itzr_dn8_slot;
        let mut var_itzr_dn9: f64 = *var_itzr_dn9_slot;
        let mut var_ixf1: f64 = *var_ixf1_slot;
        let mut var_ixf1_dn0: f64 = *var_ixf1_dn0_slot;
        let mut var_ixf1_dn1: f64 = *var_ixf1_dn1_slot;
        let mut var_ixf1_dn10: f64 = *var_ixf1_dn10_slot;
        let mut var_ixf1_dn11: f64 = *var_ixf1_dn11_slot;
        let mut var_ixf1_dn12: f64 = *var_ixf1_dn12_slot;
        let mut var_ixf1_dn13: f64 = *var_ixf1_dn13_slot;
        let mut var_ixf1_dn2: f64 = *var_ixf1_dn2_slot;
        let mut var_ixf1_dn3: f64 = *var_ixf1_dn3_slot;
        let mut var_ixf1_dn4: f64 = *var_ixf1_dn4_slot;
        let mut var_ixf1_dn5: f64 = *var_ixf1_dn5_slot;
        let mut var_ixf1_dn6: f64 = *var_ixf1_dn6_slot;
        let mut var_ixf1_dn7: f64 = *var_ixf1_dn7_slot;
        let mut var_ixf1_dn8: f64 = *var_ixf1_dn8_slot;
        let mut var_ixf1_dn9: f64 = *var_ixf1_dn9_slot;
        let mut var_power: f64 = *var_power_slot;
        let mut var_power_dn0: f64 = *var_power_dn0_slot;
        let mut var_power_dn1: f64 = *var_power_dn1_slot;
        let mut var_power_dn10: f64 = *var_power_dn10_slot;
        let mut var_power_dn11: f64 = *var_power_dn11_slot;
        let mut var_power_dn12: f64 = *var_power_dn12_slot;
        let mut var_power_dn13: f64 = *var_power_dn13_slot;
        let mut var_power_dn2: f64 = *var_power_dn2_slot;
        let mut var_power_dn3: f64 = *var_power_dn3_slot;
        let mut var_power_dn4: f64 = *var_power_dn4_slot;
        let mut var_power_dn5: f64 = *var_power_dn5_slot;
        let mut var_power_dn6: f64 = *var_power_dn6_slot;
        let mut var_power_dn7: f64 = *var_power_dn7_slot;
        let mut var_power_dn8: f64 = *var_power_dn8_slot;
        let mut var_power_dn9: f64 = *var_power_dn9_slot;

        let (assign4390_e4791, assign4390_e4791_d_n0, assign4390_e4791_d_n1, assign4390_e4791_d_n2, assign4390_e4791_d_n3, assign4390_e4791_d_n4, assign4390_e4791_d_n5, assign4390_e4791_d_n6, assign4390_e4791_d_n7, assign4390_e4791_d_n8, assign4390_e4791_d_n9, assign4390_e4791_d_n10, assign4390_e4791_d_n11, assign4390_e4791_d_n12, assign4390_e4791_d_n13,) = {
    if (var_guard132 != 0.0) {
        let assign4390_e4788: f64 = (p.p69 * var_vtv);
        let assign4390_e4789: f64 = (1.0 / assign4390_e4788);
        (assign4390_e4789, (-((p.p69 * var_vtv_dn0) / (assign4390_e4788 * assign4390_e4788))), (-((p.p69 * var_vtv_dn1) / (assign4390_e4788 * assign4390_e4788))), (-((p.p69 * var_vtv_dn2) / (assign4390_e4788 * assign4390_e4788))), (-((p.p69 * var_vtv_dn3) / (assign4390_e4788 * assign4390_e4788))), (-((p.p69 * var_vtv_dn4) / (assign4390_e4788 * assign4390_e4788))), (-((p.p69 * var_vtv_dn5) / (assign4390_e4788 * assign4390_e4788))), (-((p.p69 * var_vtv_dn6) / (assign4390_e4788 * assign4390_e4788))), (-((p.p69 * var_vtv_dn7) / (assign4390_e4788 * assign4390_e4788))), (-((p.p69 * var_vtv_dn8) / (assign4390_e4788 * assign4390_e4788))), (-((p.p69 * var_vtv_dn9) / (assign4390_e4788 * assign4390_e4788))), (-((p.p69 * var_vtv_dn10) / (assign4390_e4788 * assign4390_e4788))), (-((p.p69 * var_vtv_dn11) / (assign4390_e4788 * assign4390_e4788))), (-((p.p69 * var_vtv_dn12) / (assign4390_e4788 * assign4390_e4788))), (-((p.p69 * var_vtv_dn13) / (assign4390_e4788 * assign4390_e4788))),)
    } else {
        (var_afac, var_afac_dn0, var_afac_dn1, var_afac_dn2, var_afac_dn3, var_afac_dn4, var_afac_dn5, var_afac_dn6, var_afac_dn7, var_afac_dn8, var_afac_dn9, var_afac_dn10, var_afac_dn11, var_afac_dn12, var_afac_dn13,)
    }
};
        var_afac = assign4390_e4791;
        var_afac_dn0 = assign4390_e4791_d_n0;
        var_afac_dn1 = assign4390_e4791_d_n1;
        var_afac_dn2 = assign4390_e4791_d_n2;
        var_afac_dn3 = assign4390_e4791_d_n3;
        var_afac_dn4 = assign4390_e4791_d_n4;
        var_afac_dn5 = assign4390_e4791_d_n5;
        var_afac_dn6 = assign4390_e4791_d_n6;
        var_afac_dn7 = assign4390_e4791_d_n7;
        var_afac_dn8 = assign4390_e4791_d_n8;
        var_afac_dn9 = assign4390_e4791_d_n9;
        var_afac_dn10 = assign4390_e4791_d_n10;
        var_afac_dn11 = assign4390_e4791_d_n11;
        var_afac_dn12 = assign4390_e4791_d_n12;
        var_afac_dn13 = assign4390_e4791_d_n13;

        let assign4400_e4794: f64 = if var_vbcp < var_maxvibcnp { 1.0 } else { 0.0 };
        var_guard134 = assign4400_e4794;

        let (assign4410_e4803, assign4410_e4803_d_n0, assign4410_e4803_d_n1, assign4410_e4803_d_n2, assign4410_e4803_d_n3, assign4410_e4803_d_n4, assign4410_e4803_d_n5, assign4410_e4803_d_n6, assign4410_e4803_d_n7, assign4410_e4803_d_n8, assign4410_e4803_d_n9, assign4410_e4803_d_n10, assign4410_e4803_d_n11, assign4410_e4803_d_n12, assign4410_e4803_d_n13,) = {
    if ((var_guard132 != 0.0) && (var_guard134 != 0.0)) {
        let assign4410_e4800: f64 = (var_vbcp * var_afac);
        let assign4410_e4801: f64 = (assign4410_e4800).exp();
        (assign4410_e4801, (assign4410_e4801 * ((var_vbcp_dn0 * var_afac) + (var_vbcp * var_afac_dn0))), (assign4410_e4801 * ((var_vbcp_dn1 * var_afac) + (var_vbcp * var_afac_dn1))), (assign4410_e4801 * ((var_vbcp_dn2 * var_afac) + (var_vbcp * var_afac_dn2))), (assign4410_e4801 * ((var_vbcp_dn3 * var_afac) + (var_vbcp * var_afac_dn3))), (assign4410_e4801 * ((var_vbcp_dn4 * var_afac) + (var_vbcp * var_afac_dn4))), (assign4410_e4801 * ((var_vbcp_dn5 * var_afac) + (var_vbcp * var_afac_dn5))), (assign4410_e4801 * ((var_vbcp_dn6 * var_afac) + (var_vbcp * var_afac_dn6))), (assign4410_e4801 * ((var_vbcp_dn7 * var_afac) + (var_vbcp * var_afac_dn7))), (assign4410_e4801 * ((var_vbcp_dn8 * var_afac) + (var_vbcp * var_afac_dn8))), (assign4410_e4801 * ((var_vbcp_dn9 * var_afac) + (var_vbcp * var_afac_dn9))), (assign4410_e4801 * ((var_vbcp_dn10 * var_afac) + (var_vbcp * var_afac_dn10))), (assign4410_e4801 * ((var_vbcp_dn11 * var_afac) + (var_vbcp * var_afac_dn11))), (assign4410_e4801 * ((var_vbcp_dn12 * var_afac) + (var_vbcp * var_afac_dn12))), (assign4410_e4801 * ((var_vbcp_dn13 * var_afac) + (var_vbcp * var_afac_dn13))),)
    } else {
        (var_expn, var_expn_dn0, var_expn_dn1, var_expn_dn2, var_expn_dn3, var_expn_dn4, var_expn_dn5, var_expn_dn6, var_expn_dn7, var_expn_dn8, var_expn_dn9, var_expn_dn10, var_expn_dn11, var_expn_dn12, var_expn_dn13,)
    }
};
        var_expn = assign4410_e4803;
        var_expn_dn0 = assign4410_e4803_d_n0;
        var_expn_dn1 = assign4410_e4803_d_n1;
        var_expn_dn2 = assign4410_e4803_d_n2;
        var_expn_dn3 = assign4410_e4803_d_n3;
        var_expn_dn4 = assign4410_e4803_d_n4;
        var_expn_dn5 = assign4410_e4803_d_n5;
        var_expn_dn6 = assign4410_e4803_d_n6;
        var_expn_dn7 = assign4410_e4803_d_n7;
        var_expn_dn8 = assign4410_e4803_d_n8;
        var_expn_dn9 = assign4410_e4803_d_n9;
        var_expn_dn10 = assign4410_e4803_d_n10;
        var_expn_dn11 = assign4410_e4803_d_n11;
        var_expn_dn12 = assign4410_e4803_d_n12;
        var_expn_dn13 = assign4410_e4803_d_n13;

        let (assign4420_e4821, assign4420_e4821_d_n0, assign4420_e4821_d_n1, assign4420_e4821_d_n2, assign4420_e4821_d_n3, assign4420_e4821_d_n4, assign4420_e4821_d_n5, assign4420_e4821_d_n6, assign4420_e4821_d_n7, assign4420_e4821_d_n8, assign4420_e4821_d_n9, assign4420_e4821_d_n10, assign4420_e4821_d_n11, assign4420_e4821_d_n12, assign4420_e4821_d_n13,) = {
    if ((var_guard132 != 0.0) && (var_guard134 == 0.0)) {
        let assign4420_e4810: f64 = (var_maxvibcnp * var_afac);
        let assign4420_e4811: f64 = (assign4420_e4810).exp();
        let assign4420_e4815: f64 = (var_vbcp - var_maxvibcnp);
        let assign4420_e4817: f64 = (assign4420_e4815 * var_afac);
        let assign4420_e4818: f64 = (1.0 + assign4420_e4817);
        let assign4420_e4819: f64 = (assign4420_e4811 * assign4420_e4818);
        (assign4420_e4819, (((assign4420_e4811 * ((var_maxvibcnp_dn0 * var_afac) + (var_maxvibcnp * var_afac_dn0))) * assign4420_e4818) + (assign4420_e4811 * (((var_vbcp_dn0 - var_maxvibcnp_dn0) * var_afac) + (assign4420_e4815 * var_afac_dn0)))), (((assign4420_e4811 * ((var_maxvibcnp_dn1 * var_afac) + (var_maxvibcnp * var_afac_dn1))) * assign4420_e4818) + (assign4420_e4811 * (((var_vbcp_dn1 - var_maxvibcnp_dn1) * var_afac) + (assign4420_e4815 * var_afac_dn1)))), (((assign4420_e4811 * ((var_maxvibcnp_dn2 * var_afac) + (var_maxvibcnp * var_afac_dn2))) * assign4420_e4818) + (assign4420_e4811 * (((var_vbcp_dn2 - var_maxvibcnp_dn2) * var_afac) + (assign4420_e4815 * var_afac_dn2)))), (((assign4420_e4811 * ((var_maxvibcnp_dn3 * var_afac) + (var_maxvibcnp * var_afac_dn3))) * assign4420_e4818) + (assign4420_e4811 * (((var_vbcp_dn3 - var_maxvibcnp_dn3) * var_afac) + (assign4420_e4815 * var_afac_dn3)))), (((assign4420_e4811 * ((var_maxvibcnp_dn4 * var_afac) + (var_maxvibcnp * var_afac_dn4))) * assign4420_e4818) + (assign4420_e4811 * (((var_vbcp_dn4 - var_maxvibcnp_dn4) * var_afac) + (assign4420_e4815 * var_afac_dn4)))), (((assign4420_e4811 * ((var_maxvibcnp_dn5 * var_afac) + (var_maxvibcnp * var_afac_dn5))) * assign4420_e4818) + (assign4420_e4811 * (((var_vbcp_dn5 - var_maxvibcnp_dn5) * var_afac) + (assign4420_e4815 * var_afac_dn5)))), (((assign4420_e4811 * ((var_maxvibcnp_dn6 * var_afac) + (var_maxvibcnp * var_afac_dn6))) * assign4420_e4818) + (assign4420_e4811 * (((var_vbcp_dn6 - var_maxvibcnp_dn6) * var_afac) + (assign4420_e4815 * var_afac_dn6)))), (((assign4420_e4811 * ((var_maxvibcnp_dn7 * var_afac) + (var_maxvibcnp * var_afac_dn7))) * assign4420_e4818) + (assign4420_e4811 * (((var_vbcp_dn7 - var_maxvibcnp_dn7) * var_afac) + (assign4420_e4815 * var_afac_dn7)))), (((assign4420_e4811 * ((var_maxvibcnp_dn8 * var_afac) + (var_maxvibcnp * var_afac_dn8))) * assign4420_e4818) + (assign4420_e4811 * (((var_vbcp_dn8 - var_maxvibcnp_dn8) * var_afac) + (assign4420_e4815 * var_afac_dn8)))), (((assign4420_e4811 * ((var_maxvibcnp_dn9 * var_afac) + (var_maxvibcnp * var_afac_dn9))) * assign4420_e4818) + (assign4420_e4811 * (((var_vbcp_dn9 - var_maxvibcnp_dn9) * var_afac) + (assign4420_e4815 * var_afac_dn9)))), (((assign4420_e4811 * ((var_maxvibcnp_dn10 * var_afac) + (var_maxvibcnp * var_afac_dn10))) * assign4420_e4818) + (assign4420_e4811 * (((var_vbcp_dn10 - var_maxvibcnp_dn10) * var_afac) + (assign4420_e4815 * var_afac_dn10)))), (((assign4420_e4811 * ((var_maxvibcnp_dn11 * var_afac) + (var_maxvibcnp * var_afac_dn11))) * assign4420_e4818) + (assign4420_e4811 * (((var_vbcp_dn11 - var_maxvibcnp_dn11) * var_afac) + (assign4420_e4815 * var_afac_dn11)))), (((assign4420_e4811 * ((var_maxvibcnp_dn12 * var_afac) + (var_maxvibcnp * var_afac_dn12))) * assign4420_e4818) + (assign4420_e4811 * (((var_vbcp_dn12 - var_maxvibcnp_dn12) * var_afac) + (assign4420_e4815 * var_afac_dn12)))), (((assign4420_e4811 * ((var_maxvibcnp_dn13 * var_afac) + (var_maxvibcnp * var_afac_dn13))) * assign4420_e4818) + (assign4420_e4811 * (((var_vbcp_dn13 - var_maxvibcnp_dn13) * var_afac) + (assign4420_e4815 * var_afac_dn13)))),)
    } else {
        (var_expn, var_expn_dn0, var_expn_dn1, var_expn_dn2, var_expn_dn3, var_expn_dn4, var_expn_dn5, var_expn_dn6, var_expn_dn7, var_expn_dn8, var_expn_dn9, var_expn_dn10, var_expn_dn11, var_expn_dn12, var_expn_dn13,)
    }
};
        var_expn = assign4420_e4821;
        var_expn_dn0 = assign4420_e4821_d_n0;
        var_expn_dn1 = assign4420_e4821_d_n1;
        var_expn_dn2 = assign4420_e4821_d_n2;
        var_expn_dn3 = assign4420_e4821_d_n3;
        var_expn_dn4 = assign4420_e4821_d_n4;
        var_expn_dn5 = assign4420_e4821_d_n5;
        var_expn_dn6 = assign4420_e4821_d_n6;
        var_expn_dn7 = assign4420_e4821_d_n7;
        var_expn_dn8 = assign4420_e4821_d_n8;
        var_expn_dn9 = assign4420_e4821_d_n9;
        var_expn_dn10 = assign4420_e4821_d_n10;
        var_expn_dn11 = assign4420_e4821_d_n11;
        var_expn_dn12 = assign4420_e4821_d_n12;
        var_expn_dn13 = assign4420_e4821_d_n13;

        let (assign4430_e4835, assign4430_e4835_d_n0, assign4430_e4835_d_n1, assign4430_e4835_d_n2, assign4430_e4835_d_n3, assign4430_e4835_d_n4, assign4430_e4835_d_n5, assign4430_e4835_d_n6, assign4430_e4835_d_n7, assign4430_e4835_d_n8, assign4430_e4835_d_n9, assign4430_e4835_d_n10, assign4430_e4835_d_n11, assign4430_e4835_d_n12, assign4430_e4835_d_n13,) = {
    if (var_guard132 != 0.0) {
        let assign4430_e4826: f64 = (var_expi - 1.0);
        let assign4430_e4827: f64 = (var_ibcip_t * assign4430_e4826);
        let assign4430_e4831: f64 = (var_expn - 1.0);
        let assign4430_e4832: f64 = (var_ibcnp_t * assign4430_e4831);
        let assign4430_e4833: f64 = (assign4430_e4827 + assign4430_e4832);
        (assign4430_e4833, (((var_ibcip_t_dn0 * assign4430_e4826) + (var_ibcip_t * var_expi_dn0)) + ((var_ibcnp_t_dn0 * assign4430_e4831) + (var_ibcnp_t * var_expn_dn0))), (((var_ibcip_t_dn1 * assign4430_e4826) + (var_ibcip_t * var_expi_dn1)) + ((var_ibcnp_t_dn1 * assign4430_e4831) + (var_ibcnp_t * var_expn_dn1))), (((var_ibcip_t_dn2 * assign4430_e4826) + (var_ibcip_t * var_expi_dn2)) + ((var_ibcnp_t_dn2 * assign4430_e4831) + (var_ibcnp_t * var_expn_dn2))), (((var_ibcip_t_dn3 * assign4430_e4826) + (var_ibcip_t * var_expi_dn3)) + ((var_ibcnp_t_dn3 * assign4430_e4831) + (var_ibcnp_t * var_expn_dn3))), (((var_ibcip_t_dn4 * assign4430_e4826) + (var_ibcip_t * var_expi_dn4)) + ((var_ibcnp_t_dn4 * assign4430_e4831) + (var_ibcnp_t * var_expn_dn4))), (((var_ibcip_t_dn5 * assign4430_e4826) + (var_ibcip_t * var_expi_dn5)) + ((var_ibcnp_t_dn5 * assign4430_e4831) + (var_ibcnp_t * var_expn_dn5))), (((var_ibcip_t_dn6 * assign4430_e4826) + (var_ibcip_t * var_expi_dn6)) + ((var_ibcnp_t_dn6 * assign4430_e4831) + (var_ibcnp_t * var_expn_dn6))), (((var_ibcip_t_dn7 * assign4430_e4826) + (var_ibcip_t * var_expi_dn7)) + ((var_ibcnp_t_dn7 * assign4430_e4831) + (var_ibcnp_t * var_expn_dn7))), (((var_ibcip_t_dn8 * assign4430_e4826) + (var_ibcip_t * var_expi_dn8)) + ((var_ibcnp_t_dn8 * assign4430_e4831) + (var_ibcnp_t * var_expn_dn8))), (((var_ibcip_t_dn9 * assign4430_e4826) + (var_ibcip_t * var_expi_dn9)) + ((var_ibcnp_t_dn9 * assign4430_e4831) + (var_ibcnp_t * var_expn_dn9))), (((var_ibcip_t_dn10 * assign4430_e4826) + (var_ibcip_t * var_expi_dn10)) + ((var_ibcnp_t_dn10 * assign4430_e4831) + (var_ibcnp_t * var_expn_dn10))), (((var_ibcip_t_dn11 * assign4430_e4826) + (var_ibcip_t * var_expi_dn11)) + ((var_ibcnp_t_dn11 * assign4430_e4831) + (var_ibcnp_t * var_expn_dn11))), (((var_ibcip_t_dn12 * assign4430_e4826) + (var_ibcip_t * var_expi_dn12)) + ((var_ibcnp_t_dn12 * assign4430_e4831) + (var_ibcnp_t * var_expn_dn12))), (((var_ibcip_t_dn13 * assign4430_e4826) + (var_ibcip_t * var_expi_dn13)) + ((var_ibcnp_t_dn13 * assign4430_e4831) + (var_ibcnp_t * var_expn_dn13))),)
    } else {
        (var_ibcp, var_ibcp_dn0, var_ibcp_dn1, var_ibcp_dn2, var_ibcp_dn3, var_ibcp_dn4, var_ibcp_dn5, var_ibcp_dn6, var_ibcp_dn7, var_ibcp_dn8, var_ibcp_dn9, var_ibcp_dn10, var_ibcp_dn11, var_ibcp_dn12, var_ibcp_dn13,)
    }
};
        var_ibcp = assign4430_e4835;
        var_ibcp_dn0 = assign4430_e4835_d_n0;
        var_ibcp_dn1 = assign4430_e4835_d_n1;
        var_ibcp_dn2 = assign4430_e4835_d_n2;
        var_ibcp_dn3 = assign4430_e4835_d_n3;
        var_ibcp_dn4 = assign4430_e4835_d_n4;
        var_ibcp_dn5 = assign4430_e4835_d_n5;
        var_ibcp_dn6 = assign4430_e4835_d_n6;
        var_ibcp_dn7 = assign4430_e4835_d_n7;
        var_ibcp_dn8 = assign4430_e4835_d_n8;
        var_ibcp_dn9 = assign4430_e4835_d_n9;
        var_ibcp_dn10 = assign4430_e4835_d_n10;
        var_ibcp_dn11 = assign4430_e4835_d_n11;
        var_ibcp_dn12 = assign4430_e4835_d_n12;
        var_ibcp_dn13 = assign4430_e4835_d_n13;

        let (assign4440_e4840, assign4440_e4840_d_n0, assign4440_e4840_d_n1, assign4440_e4840_d_n2, assign4440_e4840_d_n3, assign4440_e4840_d_n4, assign4440_e4840_d_n5, assign4440_e4840_d_n6, assign4440_e4840_d_n7, assign4440_e4840_d_n8, assign4440_e4840_d_n9, assign4440_e4840_d_n10, assign4440_e4840_d_n11, assign4440_e4840_d_n12, assign4440_e4840_d_n13,) = {
    if (var_guard132 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibcp, var_ibcp_dn0, var_ibcp_dn1, var_ibcp_dn2, var_ibcp_dn3, var_ibcp_dn4, var_ibcp_dn5, var_ibcp_dn6, var_ibcp_dn7, var_ibcp_dn8, var_ibcp_dn9, var_ibcp_dn10, var_ibcp_dn11, var_ibcp_dn12, var_ibcp_dn13,)
    }
};
        var_ibcp = assign4440_e4840;
        var_ibcp_dn0 = assign4440_e4840_d_n0;
        var_ibcp_dn1 = assign4440_e4840_d_n1;
        var_ibcp_dn2 = assign4440_e4840_d_n2;
        var_ibcp_dn3 = assign4440_e4840_d_n3;
        var_ibcp_dn4 = assign4440_e4840_d_n4;
        var_ibcp_dn5 = assign4440_e4840_d_n5;
        var_ibcp_dn6 = assign4440_e4840_d_n6;
        var_ibcp_dn7 = assign4440_e4840_d_n7;
        var_ibcp_dn8 = assign4440_e4840_d_n8;
        var_ibcp_dn9 = assign4440_e4840_d_n9;
        var_ibcp_dn10 = assign4440_e4840_d_n10;
        var_ibcp_dn11 = assign4440_e4840_d_n11;
        var_ibcp_dn12 = assign4440_e4840_d_n12;
        var_ibcp_dn13 = assign4440_e4840_d_n13;

        let assign4450_e4843: f64 = (var_ibe * var_vbei);
        let assign4450_e4846: f64 = (var_ibc * var_vbci);
        let assign4450_e4847: f64 = (assign4450_e4843 + assign4450_e4846);
        let assign4450_e4850: f64 = (var_itxf - var_itzr);
        let assign4450_e4852: f64 = (assign4450_e4850 * var_vcei);
        let assign4450_e4853: f64 = (assign4450_e4847 + assign4450_e4852);
        let assign4450_e4856: f64 = (var_ibex * var_vbex);
        let assign4450_e4857: f64 = (assign4450_e4853 + assign4450_e4856);
        let assign4450_e4860: f64 = (var_ibep * var_vbep);
        let assign4450_e4861: f64 = (assign4450_e4857 + assign4450_e4860);
        let assign4450_e4864: f64 = (var_irs * var_vrs);
        let assign4450_e4865: f64 = (assign4450_e4861 + assign4450_e4864);
        let assign4450_e4868: f64 = (var_ibcp * var_vbcp);
        let assign4450_e4869: f64 = (assign4450_e4865 + assign4450_e4868);
        let assign4450_e4872: f64 = (var_iccp * var_vcep);
        let assign4450_e4873: f64 = (assign4450_e4869 + assign4450_e4872);
        let assign4450_e4876: f64 = (var_ircx * var_vrcx);
        let assign4450_e4877: f64 = (assign4450_e4873 + assign4450_e4876);
        let assign4450_e4880: f64 = (var_irci * var_vrci);
        let assign4450_e4881: f64 = (assign4450_e4877 + assign4450_e4880);
        let assign4450_e4884: f64 = (var_irbx * var_vrbx);
        let assign4450_e4885: f64 = (assign4450_e4881 + assign4450_e4884);
        let assign4450_e4888: f64 = (var_irbi * var_vrbi);
        let assign4450_e4889: f64 = (assign4450_e4885 + assign4450_e4888);
        let assign4450_e4892: f64 = (var_ire * var_vre);
        let assign4450_e4893: f64 = (assign4450_e4889 + assign4450_e4892);
        let assign4450_e4896: f64 = (var_irbp * var_vrbp);
        let assign4450_e4897: f64 = (assign4450_e4893 + assign4450_e4896);
        var_power = assign4450_e4897;
        var_power_dn0 = (((((((((((((((var_ibe_dn0 * var_vbei) + (var_ibe * var_vbei_dn0)) + ((var_ibc_dn0 * var_vbci) + (var_ibc * var_vbci_dn0))) + (((var_itxf_dn0 - var_itzr_dn0) * var_vcei) + (assign4450_e4850 * var_vcei_dn0))) + ((var_ibex_dn0 * var_vbex) + (var_ibex * var_vbex_dn0))) + ((var_ibep_dn0 * var_vbep) + (var_ibep * var_vbep_dn0))) + ((var_irs_dn0 * var_vrs) + (var_irs * var_vrs_dn0))) + ((var_ibcp_dn0 * var_vbcp) + (var_ibcp * var_vbcp_dn0))) + ((var_iccp_dn0 * var_vcep) + (var_iccp * var_vcep_dn0))) + ((var_ircx_dn0 * var_vrcx) + (var_ircx * var_vrcx_dn0))) + ((var_irci_dn0 * var_vrci) + (var_irci * var_vrci_dn0))) + ((var_irbx_dn0 * var_vrbx) + (var_irbx * var_vrbx_dn0))) + ((var_irbi_dn0 * var_vrbi) + (var_irbi * var_vrbi_dn0))) + ((var_ire_dn0 * var_vre) + (var_ire * var_vre_dn0))) + ((var_irbp_dn0 * var_vrbp) + (var_irbp * var_vrbp_dn0)));
        var_power_dn1 = (((((((((((((((var_ibe_dn1 * var_vbei) + (var_ibe * var_vbei_dn1)) + ((var_ibc_dn1 * var_vbci) + (var_ibc * var_vbci_dn1))) + (((var_itxf_dn1 - var_itzr_dn1) * var_vcei) + (assign4450_e4850 * var_vcei_dn1))) + ((var_ibex_dn1 * var_vbex) + (var_ibex * var_vbex_dn1))) + ((var_ibep_dn1 * var_vbep) + (var_ibep * var_vbep_dn1))) + ((var_irs_dn1 * var_vrs) + (var_irs * var_vrs_dn1))) + ((var_ibcp_dn1 * var_vbcp) + (var_ibcp * var_vbcp_dn1))) + ((var_iccp_dn1 * var_vcep) + (var_iccp * var_vcep_dn1))) + ((var_ircx_dn1 * var_vrcx) + (var_ircx * var_vrcx_dn1))) + ((var_irci_dn1 * var_vrci) + (var_irci * var_vrci_dn1))) + ((var_irbx_dn1 * var_vrbx) + (var_irbx * var_vrbx_dn1))) + ((var_irbi_dn1 * var_vrbi) + (var_irbi * var_vrbi_dn1))) + ((var_ire_dn1 * var_vre) + (var_ire * var_vre_dn1))) + ((var_irbp_dn1 * var_vrbp) + (var_irbp * var_vrbp_dn1)));
        var_power_dn2 = (((((((((((((((var_ibe_dn2 * var_vbei) + (var_ibe * var_vbei_dn2)) + ((var_ibc_dn2 * var_vbci) + (var_ibc * var_vbci_dn2))) + (((var_itxf_dn2 - var_itzr_dn2) * var_vcei) + (assign4450_e4850 * var_vcei_dn2))) + ((var_ibex_dn2 * var_vbex) + (var_ibex * var_vbex_dn2))) + ((var_ibep_dn2 * var_vbep) + (var_ibep * var_vbep_dn2))) + ((var_irs_dn2 * var_vrs) + (var_irs * var_vrs_dn2))) + ((var_ibcp_dn2 * var_vbcp) + (var_ibcp * var_vbcp_dn2))) + ((var_iccp_dn2 * var_vcep) + (var_iccp * var_vcep_dn2))) + ((var_ircx_dn2 * var_vrcx) + (var_ircx * var_vrcx_dn2))) + ((var_irci_dn2 * var_vrci) + (var_irci * var_vrci_dn2))) + ((var_irbx_dn2 * var_vrbx) + (var_irbx * var_vrbx_dn2))) + ((var_irbi_dn2 * var_vrbi) + (var_irbi * var_vrbi_dn2))) + ((var_ire_dn2 * var_vre) + (var_ire * var_vre_dn2))) + ((var_irbp_dn2 * var_vrbp) + (var_irbp * var_vrbp_dn2)));
        var_power_dn3 = (((((((((((((((var_ibe_dn3 * var_vbei) + (var_ibe * var_vbei_dn3)) + ((var_ibc_dn3 * var_vbci) + (var_ibc * var_vbci_dn3))) + (((var_itxf_dn3 - var_itzr_dn3) * var_vcei) + (assign4450_e4850 * var_vcei_dn3))) + ((var_ibex_dn3 * var_vbex) + (var_ibex * var_vbex_dn3))) + ((var_ibep_dn3 * var_vbep) + (var_ibep * var_vbep_dn3))) + ((var_irs_dn3 * var_vrs) + (var_irs * var_vrs_dn3))) + ((var_ibcp_dn3 * var_vbcp) + (var_ibcp * var_vbcp_dn3))) + ((var_iccp_dn3 * var_vcep) + (var_iccp * var_vcep_dn3))) + ((var_ircx_dn3 * var_vrcx) + (var_ircx * var_vrcx_dn3))) + ((var_irci_dn3 * var_vrci) + (var_irci * var_vrci_dn3))) + ((var_irbx_dn3 * var_vrbx) + (var_irbx * var_vrbx_dn3))) + ((var_irbi_dn3 * var_vrbi) + (var_irbi * var_vrbi_dn3))) + ((var_ire_dn3 * var_vre) + (var_ire * var_vre_dn3))) + ((var_irbp_dn3 * var_vrbp) + (var_irbp * var_vrbp_dn3)));
        var_power_dn4 = (((((((((((((((var_ibe_dn4 * var_vbei) + (var_ibe * var_vbei_dn4)) + ((var_ibc_dn4 * var_vbci) + (var_ibc * var_vbci_dn4))) + (((var_itxf_dn4 - var_itzr_dn4) * var_vcei) + (assign4450_e4850 * var_vcei_dn4))) + ((var_ibex_dn4 * var_vbex) + (var_ibex * var_vbex_dn4))) + ((var_ibep_dn4 * var_vbep) + (var_ibep * var_vbep_dn4))) + ((var_irs_dn4 * var_vrs) + (var_irs * var_vrs_dn4))) + ((var_ibcp_dn4 * var_vbcp) + (var_ibcp * var_vbcp_dn4))) + ((var_iccp_dn4 * var_vcep) + (var_iccp * var_vcep_dn4))) + ((var_ircx_dn4 * var_vrcx) + (var_ircx * var_vrcx_dn4))) + ((var_irci_dn4 * var_vrci) + (var_irci * var_vrci_dn4))) + ((var_irbx_dn4 * var_vrbx) + (var_irbx * var_vrbx_dn4))) + ((var_irbi_dn4 * var_vrbi) + (var_irbi * var_vrbi_dn4))) + ((var_ire_dn4 * var_vre) + (var_ire * var_vre_dn4))) + ((var_irbp_dn4 * var_vrbp) + (var_irbp * var_vrbp_dn4)));
        var_power_dn5 = (((((((((((((((var_ibe_dn5 * var_vbei) + (var_ibe * var_vbei_dn5)) + ((var_ibc_dn5 * var_vbci) + (var_ibc * var_vbci_dn5))) + (((var_itxf_dn5 - var_itzr_dn5) * var_vcei) + (assign4450_e4850 * var_vcei_dn5))) + ((var_ibex_dn5 * var_vbex) + (var_ibex * var_vbex_dn5))) + ((var_ibep_dn5 * var_vbep) + (var_ibep * var_vbep_dn5))) + ((var_irs_dn5 * var_vrs) + (var_irs * var_vrs_dn5))) + ((var_ibcp_dn5 * var_vbcp) + (var_ibcp * var_vbcp_dn5))) + ((var_iccp_dn5 * var_vcep) + (var_iccp * var_vcep_dn5))) + ((var_ircx_dn5 * var_vrcx) + (var_ircx * var_vrcx_dn5))) + ((var_irci_dn5 * var_vrci) + (var_irci * var_vrci_dn5))) + ((var_irbx_dn5 * var_vrbx) + (var_irbx * var_vrbx_dn5))) + ((var_irbi_dn5 * var_vrbi) + (var_irbi * var_vrbi_dn5))) + ((var_ire_dn5 * var_vre) + (var_ire * var_vre_dn5))) + ((var_irbp_dn5 * var_vrbp) + (var_irbp * var_vrbp_dn5)));
        var_power_dn6 = (((((((((((((((var_ibe_dn6 * var_vbei) + (var_ibe * var_vbei_dn6)) + ((var_ibc_dn6 * var_vbci) + (var_ibc * var_vbci_dn6))) + (((var_itxf_dn6 - var_itzr_dn6) * var_vcei) + (assign4450_e4850 * var_vcei_dn6))) + ((var_ibex_dn6 * var_vbex) + (var_ibex * var_vbex_dn6))) + ((var_ibep_dn6 * var_vbep) + (var_ibep * var_vbep_dn6))) + ((var_irs_dn6 * var_vrs) + (var_irs * var_vrs_dn6))) + ((var_ibcp_dn6 * var_vbcp) + (var_ibcp * var_vbcp_dn6))) + ((var_iccp_dn6 * var_vcep) + (var_iccp * var_vcep_dn6))) + ((var_ircx_dn6 * var_vrcx) + (var_ircx * var_vrcx_dn6))) + ((var_irci_dn6 * var_vrci) + (var_irci * var_vrci_dn6))) + ((var_irbx_dn6 * var_vrbx) + (var_irbx * var_vrbx_dn6))) + ((var_irbi_dn6 * var_vrbi) + (var_irbi * var_vrbi_dn6))) + ((var_ire_dn6 * var_vre) + (var_ire * var_vre_dn6))) + ((var_irbp_dn6 * var_vrbp) + (var_irbp * var_vrbp_dn6)));
        var_power_dn7 = (((((((((((((((var_ibe_dn7 * var_vbei) + (var_ibe * var_vbei_dn7)) + ((var_ibc_dn7 * var_vbci) + (var_ibc * var_vbci_dn7))) + (((var_itxf_dn7 - var_itzr_dn7) * var_vcei) + (assign4450_e4850 * var_vcei_dn7))) + ((var_ibex_dn7 * var_vbex) + (var_ibex * var_vbex_dn7))) + ((var_ibep_dn7 * var_vbep) + (var_ibep * var_vbep_dn7))) + ((var_irs_dn7 * var_vrs) + (var_irs * var_vrs_dn7))) + ((var_ibcp_dn7 * var_vbcp) + (var_ibcp * var_vbcp_dn7))) + ((var_iccp_dn7 * var_vcep) + (var_iccp * var_vcep_dn7))) + ((var_ircx_dn7 * var_vrcx) + (var_ircx * var_vrcx_dn7))) + ((var_irci_dn7 * var_vrci) + (var_irci * var_vrci_dn7))) + ((var_irbx_dn7 * var_vrbx) + (var_irbx * var_vrbx_dn7))) + ((var_irbi_dn7 * var_vrbi) + (var_irbi * var_vrbi_dn7))) + ((var_ire_dn7 * var_vre) + (var_ire * var_vre_dn7))) + ((var_irbp_dn7 * var_vrbp) + (var_irbp * var_vrbp_dn7)));
        var_power_dn8 = (((((((((((((((var_ibe_dn8 * var_vbei) + (var_ibe * var_vbei_dn8)) + ((var_ibc_dn8 * var_vbci) + (var_ibc * var_vbci_dn8))) + (((var_itxf_dn8 - var_itzr_dn8) * var_vcei) + (assign4450_e4850 * var_vcei_dn8))) + ((var_ibex_dn8 * var_vbex) + (var_ibex * var_vbex_dn8))) + ((var_ibep_dn8 * var_vbep) + (var_ibep * var_vbep_dn8))) + ((var_irs_dn8 * var_vrs) + (var_irs * var_vrs_dn8))) + ((var_ibcp_dn8 * var_vbcp) + (var_ibcp * var_vbcp_dn8))) + ((var_iccp_dn8 * var_vcep) + (var_iccp * var_vcep_dn8))) + ((var_ircx_dn8 * var_vrcx) + (var_ircx * var_vrcx_dn8))) + ((var_irci_dn8 * var_vrci) + (var_irci * var_vrci_dn8))) + ((var_irbx_dn8 * var_vrbx) + (var_irbx * var_vrbx_dn8))) + ((var_irbi_dn8 * var_vrbi) + (var_irbi * var_vrbi_dn8))) + ((var_ire_dn8 * var_vre) + (var_ire * var_vre_dn8))) + ((var_irbp_dn8 * var_vrbp) + (var_irbp * var_vrbp_dn8)));
        var_power_dn9 = (((((((((((((((var_ibe_dn9 * var_vbei) + (var_ibe * var_vbei_dn9)) + ((var_ibc_dn9 * var_vbci) + (var_ibc * var_vbci_dn9))) + (((var_itxf_dn9 - var_itzr_dn9) * var_vcei) + (assign4450_e4850 * var_vcei_dn9))) + ((var_ibex_dn9 * var_vbex) + (var_ibex * var_vbex_dn9))) + ((var_ibep_dn9 * var_vbep) + (var_ibep * var_vbep_dn9))) + ((var_irs_dn9 * var_vrs) + (var_irs * var_vrs_dn9))) + ((var_ibcp_dn9 * var_vbcp) + (var_ibcp * var_vbcp_dn9))) + ((var_iccp_dn9 * var_vcep) + (var_iccp * var_vcep_dn9))) + ((var_ircx_dn9 * var_vrcx) + (var_ircx * var_vrcx_dn9))) + ((var_irci_dn9 * var_vrci) + (var_irci * var_vrci_dn9))) + ((var_irbx_dn9 * var_vrbx) + (var_irbx * var_vrbx_dn9))) + ((var_irbi_dn9 * var_vrbi) + (var_irbi * var_vrbi_dn9))) + ((var_ire_dn9 * var_vre) + (var_ire * var_vre_dn9))) + ((var_irbp_dn9 * var_vrbp) + (var_irbp * var_vrbp_dn9)));
        var_power_dn10 = (((((((((((((((var_ibe_dn10 * var_vbei) + (var_ibe * var_vbei_dn10)) + ((var_ibc_dn10 * var_vbci) + (var_ibc * var_vbci_dn10))) + (((var_itxf_dn10 - var_itzr_dn10) * var_vcei) + (assign4450_e4850 * var_vcei_dn10))) + ((var_ibex_dn10 * var_vbex) + (var_ibex * var_vbex_dn10))) + ((var_ibep_dn10 * var_vbep) + (var_ibep * var_vbep_dn10))) + ((var_irs_dn10 * var_vrs) + (var_irs * var_vrs_dn10))) + ((var_ibcp_dn10 * var_vbcp) + (var_ibcp * var_vbcp_dn10))) + ((var_iccp_dn10 * var_vcep) + (var_iccp * var_vcep_dn10))) + ((var_ircx_dn10 * var_vrcx) + (var_ircx * var_vrcx_dn10))) + ((var_irci_dn10 * var_vrci) + (var_irci * var_vrci_dn10))) + ((var_irbx_dn10 * var_vrbx) + (var_irbx * var_vrbx_dn10))) + ((var_irbi_dn10 * var_vrbi) + (var_irbi * var_vrbi_dn10))) + ((var_ire_dn10 * var_vre) + (var_ire * var_vre_dn10))) + ((var_irbp_dn10 * var_vrbp) + (var_irbp * var_vrbp_dn10)));
        var_power_dn11 = (((((((((((((((var_ibe_dn11 * var_vbei) + (var_ibe * var_vbei_dn11)) + ((var_ibc_dn11 * var_vbci) + (var_ibc * var_vbci_dn11))) + (((var_itxf_dn11 - var_itzr_dn11) * var_vcei) + (assign4450_e4850 * var_vcei_dn11))) + ((var_ibex_dn11 * var_vbex) + (var_ibex * var_vbex_dn11))) + ((var_ibep_dn11 * var_vbep) + (var_ibep * var_vbep_dn11))) + ((var_irs_dn11 * var_vrs) + (var_irs * var_vrs_dn11))) + ((var_ibcp_dn11 * var_vbcp) + (var_ibcp * var_vbcp_dn11))) + ((var_iccp_dn11 * var_vcep) + (var_iccp * var_vcep_dn11))) + ((var_ircx_dn11 * var_vrcx) + (var_ircx * var_vrcx_dn11))) + ((var_irci_dn11 * var_vrci) + (var_irci * var_vrci_dn11))) + ((var_irbx_dn11 * var_vrbx) + (var_irbx * var_vrbx_dn11))) + ((var_irbi_dn11 * var_vrbi) + (var_irbi * var_vrbi_dn11))) + ((var_ire_dn11 * var_vre) + (var_ire * var_vre_dn11))) + ((var_irbp_dn11 * var_vrbp) + (var_irbp * var_vrbp_dn11)));
        var_power_dn12 = (((((((((((((((var_ibe_dn12 * var_vbei) + (var_ibe * var_vbei_dn12)) + ((var_ibc_dn12 * var_vbci) + (var_ibc * var_vbci_dn12))) + (((var_itxf_dn12 - var_itzr_dn12) * var_vcei) + (assign4450_e4850 * var_vcei_dn12))) + ((var_ibex_dn12 * var_vbex) + (var_ibex * var_vbex_dn12))) + ((var_ibep_dn12 * var_vbep) + (var_ibep * var_vbep_dn12))) + ((var_irs_dn12 * var_vrs) + (var_irs * var_vrs_dn12))) + ((var_ibcp_dn12 * var_vbcp) + (var_ibcp * var_vbcp_dn12))) + ((var_iccp_dn12 * var_vcep) + (var_iccp * var_vcep_dn12))) + ((var_ircx_dn12 * var_vrcx) + (var_ircx * var_vrcx_dn12))) + ((var_irci_dn12 * var_vrci) + (var_irci * var_vrci_dn12))) + ((var_irbx_dn12 * var_vrbx) + (var_irbx * var_vrbx_dn12))) + ((var_irbi_dn12 * var_vrbi) + (var_irbi * var_vrbi_dn12))) + ((var_ire_dn12 * var_vre) + (var_ire * var_vre_dn12))) + ((var_irbp_dn12 * var_vrbp) + (var_irbp * var_vrbp_dn12)));
        var_power_dn13 = (((((((((((((((var_ibe_dn13 * var_vbei) + (var_ibe * var_vbei_dn13)) + ((var_ibc_dn13 * var_vbci) + (var_ibc * var_vbci_dn13))) + (((var_itxf_dn13 - var_itzr_dn13) * var_vcei) + (assign4450_e4850 * var_vcei_dn13))) + ((var_ibex_dn13 * var_vbex) + (var_ibex * var_vbex_dn13))) + ((var_ibep_dn13 * var_vbep) + (var_ibep * var_vbep_dn13))) + ((var_irs_dn13 * var_vrs) + (var_irs * var_vrs_dn13))) + ((var_ibcp_dn13 * var_vbcp) + (var_ibcp * var_vbcp_dn13))) + ((var_iccp_dn13 * var_vcep) + (var_iccp * var_vcep_dn13))) + ((var_ircx_dn13 * var_vrcx) + (var_ircx * var_vrcx_dn13))) + ((var_irci_dn13 * var_vrci) + (var_irci * var_vrci_dn13))) + ((var_irbx_dn13 * var_vrbx) + (var_irbx * var_vrbx_dn13))) + ((var_irbi_dn13 * var_vrbi) + (var_irbi * var_vrbi_dn13))) + ((var_ire_dn13 * var_vre) + (var_ire * var_vre_dn13))) + ((var_irbp_dn13 * var_vrbp) + (var_irbp * var_vrbp_dn13)));

        let assign4460_e4899: f64 = (-p.p2);
        let assign4460_e4901: f64 = (assign4460_e4899 * var_power);
        var_ith = assign4460_e4901;
        var_ith_dn0 = (assign4460_e4899 * var_power_dn0);
        var_ith_dn1 = (assign4460_e4899 * var_power_dn1);
        var_ith_dn2 = (assign4460_e4899 * var_power_dn2);
        var_ith_dn3 = (assign4460_e4899 * var_power_dn3);
        var_ith_dn4 = (assign4460_e4899 * var_power_dn4);
        var_ith_dn5 = (assign4460_e4899 * var_power_dn5);
        var_ith_dn6 = (assign4460_e4899 * var_power_dn6);
        var_ith_dn7 = (assign4460_e4899 * var_power_dn7);
        var_ith_dn8 = (assign4460_e4899 * var_power_dn8);
        var_ith_dn9 = (assign4460_e4899 * var_power_dn9);
        var_ith_dn10 = (assign4460_e4899 * var_power_dn10);
        var_ith_dn11 = (assign4460_e4899 * var_power_dn11);
        var_ith_dn12 = (assign4460_e4899 * var_power_dn12);
        var_ith_dn13 = (assign4460_e4899 * var_power_dn13);

        let assign4470_e4904: f64 = (var_dt_et * var_gth);
        var_irth = assign4470_e4904;
        var_irth_dn0 = ((var_dt_et_dn0 * var_gth) + (var_dt_et * var_gth_dn0));
        var_irth_dn1 = ((var_dt_et_dn1 * var_gth) + (var_dt_et * var_gth_dn1));
        var_irth_dn2 = ((var_dt_et_dn2 * var_gth) + (var_dt_et * var_gth_dn2));
        var_irth_dn3 = ((var_dt_et_dn3 * var_gth) + (var_dt_et * var_gth_dn3));
        var_irth_dn4 = ((var_dt_et_dn4 * var_gth) + (var_dt_et * var_gth_dn4));
        var_irth_dn5 = ((var_dt_et_dn5 * var_gth) + (var_dt_et * var_gth_dn5));
        var_irth_dn6 = ((var_dt_et_dn6 * var_gth) + (var_dt_et * var_gth_dn6));
        var_irth_dn7 = ((var_dt_et_dn7 * var_gth) + (var_dt_et * var_gth_dn7));
        var_irth_dn8 = ((var_dt_et_dn8 * var_gth) + (var_dt_et * var_gth_dn8));
        var_irth_dn9 = ((var_dt_et_dn9 * var_gth) + (var_dt_et * var_gth_dn9));
        var_irth_dn10 = ((var_dt_et_dn10 * var_gth) + (var_dt_et * var_gth_dn10));
        var_irth_dn11 = ((var_dt_et_dn11 * var_gth) + (var_dt_et * var_gth_dn11));
        var_irth_dn12 = ((var_dt_et_dn12 * var_gth) + (var_dt_et * var_gth_dn12));
        var_irth_dn13 = ((var_dt_et_dn13 * var_gth) + (var_dt_et * var_gth_dn13));

        let assign4480_e4907: f64 = (var_vxf2 - var_itzf);
        var_ixf1 = assign4480_e4907;
        var_ixf1_dn0 = (var_vxf2_dn0 - var_itzf_dn0);
        var_ixf1_dn1 = (var_vxf2_dn1 - var_itzf_dn1);
        var_ixf1_dn2 = (var_vxf2_dn2 - var_itzf_dn2);
        var_ixf1_dn3 = (var_vxf2_dn3 - var_itzf_dn3);
        var_ixf1_dn4 = (var_vxf2_dn4 - var_itzf_dn4);
        var_ixf1_dn5 = (var_vxf2_dn5 - var_itzf_dn5);
        var_ixf1_dn6 = (var_vxf2_dn6 - var_itzf_dn6);
        var_ixf1_dn7 = (var_vxf2_dn7 - var_itzf_dn7);
        var_ixf1_dn8 = (var_vxf2_dn8 - var_itzf_dn8);
        var_ixf1_dn9 = (var_vxf2_dn9 - var_itzf_dn9);
        var_ixf1_dn10 = (var_vxf2_dn10 - var_itzf_dn10);
        var_ixf1_dn11 = (var_vxf2_dn11 - var_itzf_dn11);
        var_ixf1_dn12 = (var_vxf2_dn12 - var_itzf_dn12);
        var_ixf1_dn13 = (var_vxf2_dn13 - var_itzf_dn13);

        let assign4500_e4914: f64 = (var_gminmod * var_vbei);
        let assign4500_e4915: f64 = (var_ibe + assign4500_e4914);
        var_ibe = assign4500_e4915;
        var_ibe_dn0 = (var_ibe_dn0 + (var_gminmod * var_vbei_dn0));
        var_ibe_dn1 = (var_ibe_dn1 + (var_gminmod * var_vbei_dn1));
        var_ibe_dn2 = (var_ibe_dn2 + (var_gminmod * var_vbei_dn2));
        var_ibe_dn3 = (var_ibe_dn3 + (var_gminmod * var_vbei_dn3));
        var_ibe_dn4 = (var_ibe_dn4 + (var_gminmod * var_vbei_dn4));
        var_ibe_dn5 = (var_ibe_dn5 + (var_gminmod * var_vbei_dn5));
        var_ibe_dn6 = (var_ibe_dn6 + (var_gminmod * var_vbei_dn6));
        var_ibe_dn7 = (var_ibe_dn7 + (var_gminmod * var_vbei_dn7));
        var_ibe_dn8 = (var_ibe_dn8 + (var_gminmod * var_vbei_dn8));
        var_ibe_dn9 = (var_ibe_dn9 + (var_gminmod * var_vbei_dn9));
        var_ibe_dn10 = (var_ibe_dn10 + (var_gminmod * var_vbei_dn10));
        var_ibe_dn11 = (var_ibe_dn11 + (var_gminmod * var_vbei_dn11));
        var_ibe_dn12 = (var_ibe_dn12 + (var_gminmod * var_vbei_dn12));
        var_ibe_dn13 = (var_ibe_dn13 + (var_gminmod * var_vbei_dn13));

        let assign4510_e4919: f64 = (var_gminmod * var_vbex);
        let assign4510_e4920: f64 = (var_ibex + assign4510_e4919);
        var_ibex = assign4510_e4920;
        var_ibex_dn0 = (var_ibex_dn0 + (var_gminmod * var_vbex_dn0));
        var_ibex_dn1 = (var_ibex_dn1 + (var_gminmod * var_vbex_dn1));
        var_ibex_dn2 = (var_ibex_dn2 + (var_gminmod * var_vbex_dn2));
        var_ibex_dn3 = (var_ibex_dn3 + (var_gminmod * var_vbex_dn3));
        var_ibex_dn4 = (var_ibex_dn4 + (var_gminmod * var_vbex_dn4));
        var_ibex_dn5 = (var_ibex_dn5 + (var_gminmod * var_vbex_dn5));
        var_ibex_dn6 = (var_ibex_dn6 + (var_gminmod * var_vbex_dn6));
        var_ibex_dn7 = (var_ibex_dn7 + (var_gminmod * var_vbex_dn7));
        var_ibex_dn8 = (var_ibex_dn8 + (var_gminmod * var_vbex_dn8));
        var_ibex_dn9 = (var_ibex_dn9 + (var_gminmod * var_vbex_dn9));
        var_ibex_dn10 = (var_ibex_dn10 + (var_gminmod * var_vbex_dn10));
        var_ibex_dn11 = (var_ibex_dn11 + (var_gminmod * var_vbex_dn11));
        var_ibex_dn12 = (var_ibex_dn12 + (var_gminmod * var_vbex_dn12));
        var_ibex_dn13 = (var_ibex_dn13 + (var_gminmod * var_vbex_dn13));

        let assign4520_e4924: f64 = (var_gminmod * var_vbep);
        let assign4520_e4925: f64 = (var_ibep + assign4520_e4924);
        var_ibep = assign4520_e4925;
        var_ibep_dn0 = (var_ibep_dn0 + (var_gminmod * var_vbep_dn0));
        var_ibep_dn1 = (var_ibep_dn1 + (var_gminmod * var_vbep_dn1));
        var_ibep_dn2 = (var_ibep_dn2 + (var_gminmod * var_vbep_dn2));
        var_ibep_dn3 = (var_ibep_dn3 + (var_gminmod * var_vbep_dn3));
        var_ibep_dn4 = (var_ibep_dn4 + (var_gminmod * var_vbep_dn4));
        var_ibep_dn5 = (var_ibep_dn5 + (var_gminmod * var_vbep_dn5));
        var_ibep_dn6 = (var_ibep_dn6 + (var_gminmod * var_vbep_dn6));
        var_ibep_dn7 = (var_ibep_dn7 + (var_gminmod * var_vbep_dn7));
        var_ibep_dn8 = (var_ibep_dn8 + (var_gminmod * var_vbep_dn8));
        var_ibep_dn9 = (var_ibep_dn9 + (var_gminmod * var_vbep_dn9));
        var_ibep_dn10 = (var_ibep_dn10 + (var_gminmod * var_vbep_dn10));
        var_ibep_dn11 = (var_ibep_dn11 + (var_gminmod * var_vbep_dn11));
        var_ibep_dn12 = (var_ibep_dn12 + (var_gminmod * var_vbep_dn12));
        var_ibep_dn13 = (var_ibep_dn13 + (var_gminmod * var_vbep_dn13));

        let assign4530_e4929: f64 = (var_gminmod * var_vbci);
        let assign4530_e4930: f64 = (var_ibc + assign4530_e4929);
        var_ibc = assign4530_e4930;
        var_ibc_dn0 = (var_ibc_dn0 + (var_gminmod * var_vbci_dn0));
        var_ibc_dn1 = (var_ibc_dn1 + (var_gminmod * var_vbci_dn1));
        var_ibc_dn2 = (var_ibc_dn2 + (var_gminmod * var_vbci_dn2));
        var_ibc_dn3 = (var_ibc_dn3 + (var_gminmod * var_vbci_dn3));
        var_ibc_dn4 = (var_ibc_dn4 + (var_gminmod * var_vbci_dn4));
        var_ibc_dn5 = (var_ibc_dn5 + (var_gminmod * var_vbci_dn5));
        var_ibc_dn6 = (var_ibc_dn6 + (var_gminmod * var_vbci_dn6));
        var_ibc_dn7 = (var_ibc_dn7 + (var_gminmod * var_vbci_dn7));
        var_ibc_dn8 = (var_ibc_dn8 + (var_gminmod * var_vbci_dn8));
        var_ibc_dn9 = (var_ibc_dn9 + (var_gminmod * var_vbci_dn9));
        var_ibc_dn10 = (var_ibc_dn10 + (var_gminmod * var_vbci_dn10));
        var_ibc_dn11 = (var_ibc_dn11 + (var_gminmod * var_vbci_dn11));
        var_ibc_dn12 = (var_ibc_dn12 + (var_gminmod * var_vbci_dn12));
        var_ibc_dn13 = (var_ibc_dn13 + (var_gminmod * var_vbci_dn13));

        let assign4540_e4934: f64 = (var_gminmod * var_vbxcx);
        let assign4540_e4935: f64 = (var_igcx + assign4540_e4934);
        var_igcx = assign4540_e4935;
        var_igcx_dn0 = (var_igcx_dn0 + (var_gminmod * var_vbxcx_dn0));
        var_igcx_dn1 = (var_igcx_dn1 + (var_gminmod * var_vbxcx_dn1));
        var_igcx_dn2 = (var_igcx_dn2 + (var_gminmod * var_vbxcx_dn2));
        var_igcx_dn3 = (var_igcx_dn3 + (var_gminmod * var_vbxcx_dn3));
        var_igcx_dn4 = (var_igcx_dn4 + (var_gminmod * var_vbxcx_dn4));
        var_igcx_dn5 = (var_igcx_dn5 + (var_gminmod * var_vbxcx_dn5));
        var_igcx_dn6 = (var_igcx_dn6 + (var_gminmod * var_vbxcx_dn6));
        var_igcx_dn7 = (var_igcx_dn7 + (var_gminmod * var_vbxcx_dn7));
        var_igcx_dn8 = (var_igcx_dn8 + (var_gminmod * var_vbxcx_dn8));
        var_igcx_dn9 = (var_igcx_dn9 + (var_gminmod * var_vbxcx_dn9));
        var_igcx_dn10 = (var_igcx_dn10 + (var_gminmod * var_vbxcx_dn10));
        var_igcx_dn11 = (var_igcx_dn11 + (var_gminmod * var_vbxcx_dn11));
        var_igcx_dn12 = (var_igcx_dn12 + (var_gminmod * var_vbxcx_dn12));
        var_igcx_dn13 = (var_igcx_dn13 + (var_gminmod * var_vbxcx_dn13));

        let assign4550_e4939: f64 = (var_gminmod * var_vbcp);
        let assign4550_e4940: f64 = (var_ibcp + assign4550_e4939);
        var_ibcp = assign4550_e4940;
        var_ibcp_dn0 = (var_ibcp_dn0 + (var_gminmod * var_vbcp_dn0));
        var_ibcp_dn1 = (var_ibcp_dn1 + (var_gminmod * var_vbcp_dn1));
        var_ibcp_dn2 = (var_ibcp_dn2 + (var_gminmod * var_vbcp_dn2));
        var_ibcp_dn3 = (var_ibcp_dn3 + (var_gminmod * var_vbcp_dn3));
        var_ibcp_dn4 = (var_ibcp_dn4 + (var_gminmod * var_vbcp_dn4));
        var_ibcp_dn5 = (var_ibcp_dn5 + (var_gminmod * var_vbcp_dn5));
        var_ibcp_dn6 = (var_ibcp_dn6 + (var_gminmod * var_vbcp_dn6));
        var_ibcp_dn7 = (var_ibcp_dn7 + (var_gminmod * var_vbcp_dn7));
        var_ibcp_dn8 = (var_ibcp_dn8 + (var_gminmod * var_vbcp_dn8));
        var_ibcp_dn9 = (var_ibcp_dn9 + (var_gminmod * var_vbcp_dn9));
        var_ibcp_dn10 = (var_ibcp_dn10 + (var_gminmod * var_vbcp_dn10));
        var_ibcp_dn11 = (var_ibcp_dn11 + (var_gminmod * var_vbcp_dn11));
        var_ibcp_dn12 = (var_ibcp_dn12 + (var_gminmod * var_vbcp_dn12));
        var_ibcp_dn13 = (var_ibcp_dn13 + (var_gminmod * var_vbcp_dn13));

        let assign4560_e4943: f64 = var_vbictype;
        let assign4560_e4945: f64 = (assign4560_e4943 * var_ibe);
        var_ibe = assign4560_e4945;
        var_ibe_dn0 = (assign4560_e4943 * var_ibe_dn0);
        var_ibe_dn1 = (assign4560_e4943 * var_ibe_dn1);
        var_ibe_dn2 = (assign4560_e4943 * var_ibe_dn2);
        var_ibe_dn3 = (assign4560_e4943 * var_ibe_dn3);
        var_ibe_dn4 = (assign4560_e4943 * var_ibe_dn4);
        var_ibe_dn5 = (assign4560_e4943 * var_ibe_dn5);
        var_ibe_dn6 = (assign4560_e4943 * var_ibe_dn6);
        var_ibe_dn7 = (assign4560_e4943 * var_ibe_dn7);
        var_ibe_dn8 = (assign4560_e4943 * var_ibe_dn8);
        var_ibe_dn9 = (assign4560_e4943 * var_ibe_dn9);
        var_ibe_dn10 = (assign4560_e4943 * var_ibe_dn10);
        var_ibe_dn11 = (assign4560_e4943 * var_ibe_dn11);
        var_ibe_dn12 = (assign4560_e4943 * var_ibe_dn12);
        var_ibe_dn13 = (assign4560_e4943 * var_ibe_dn13);

        let assign4570_e4948: f64 = var_vbictype;
        let assign4570_e4950: f64 = (assign4570_e4948 * var_ibex);
        var_ibex = assign4570_e4950;
        var_ibex_dn0 = (assign4570_e4948 * var_ibex_dn0);
        var_ibex_dn1 = (assign4570_e4948 * var_ibex_dn1);
        var_ibex_dn2 = (assign4570_e4948 * var_ibex_dn2);
        var_ibex_dn3 = (assign4570_e4948 * var_ibex_dn3);
        var_ibex_dn4 = (assign4570_e4948 * var_ibex_dn4);
        var_ibex_dn5 = (assign4570_e4948 * var_ibex_dn5);
        var_ibex_dn6 = (assign4570_e4948 * var_ibex_dn6);
        var_ibex_dn7 = (assign4570_e4948 * var_ibex_dn7);
        var_ibex_dn8 = (assign4570_e4948 * var_ibex_dn8);
        var_ibex_dn9 = (assign4570_e4948 * var_ibex_dn9);
        var_ibex_dn10 = (assign4570_e4948 * var_ibex_dn10);
        var_ibex_dn11 = (assign4570_e4948 * var_ibex_dn11);
        var_ibex_dn12 = (assign4570_e4948 * var_ibex_dn12);
        var_ibex_dn13 = (assign4570_e4948 * var_ibex_dn13);

        let assign4580_e4953: f64 = var_vbictype;
        let assign4580_e4955: f64 = (assign4580_e4953 * var_itzf);
        var_itzf = assign4580_e4955;
        var_itzf_dn0 = (assign4580_e4953 * var_itzf_dn0);
        var_itzf_dn1 = (assign4580_e4953 * var_itzf_dn1);
        var_itzf_dn2 = (assign4580_e4953 * var_itzf_dn2);
        var_itzf_dn3 = (assign4580_e4953 * var_itzf_dn3);
        var_itzf_dn4 = (assign4580_e4953 * var_itzf_dn4);
        var_itzf_dn5 = (assign4580_e4953 * var_itzf_dn5);
        var_itzf_dn6 = (assign4580_e4953 * var_itzf_dn6);
        var_itzf_dn7 = (assign4580_e4953 * var_itzf_dn7);
        var_itzf_dn8 = (assign4580_e4953 * var_itzf_dn8);
        var_itzf_dn9 = (assign4580_e4953 * var_itzf_dn9);
        var_itzf_dn10 = (assign4580_e4953 * var_itzf_dn10);
        var_itzf_dn11 = (assign4580_e4953 * var_itzf_dn11);
        var_itzf_dn12 = (assign4580_e4953 * var_itzf_dn12);
        var_itzf_dn13 = (assign4580_e4953 * var_itzf_dn13);

        let assign4590_e4958: f64 = var_vbictype;
        let assign4590_e4960: f64 = (assign4590_e4958 * var_itxf);
        var_itxf = assign4590_e4960;
        var_itxf_dn0 = (assign4590_e4958 * var_itxf_dn0);
        var_itxf_dn1 = (assign4590_e4958 * var_itxf_dn1);
        var_itxf_dn2 = (assign4590_e4958 * var_itxf_dn2);
        var_itxf_dn3 = (assign4590_e4958 * var_itxf_dn3);
        var_itxf_dn4 = (assign4590_e4958 * var_itxf_dn4);
        var_itxf_dn5 = (assign4590_e4958 * var_itxf_dn5);
        var_itxf_dn6 = (assign4590_e4958 * var_itxf_dn6);
        var_itxf_dn7 = (assign4590_e4958 * var_itxf_dn7);
        var_itxf_dn8 = (assign4590_e4958 * var_itxf_dn8);
        var_itxf_dn9 = (assign4590_e4958 * var_itxf_dn9);
        var_itxf_dn10 = (assign4590_e4958 * var_itxf_dn10);
        var_itxf_dn11 = (assign4590_e4958 * var_itxf_dn11);
        var_itxf_dn12 = (assign4590_e4958 * var_itxf_dn12);
        var_itxf_dn13 = (assign4590_e4958 * var_itxf_dn13);

        let assign4600_e4963: f64 = var_vbictype;
        let assign4600_e4965: f64 = (assign4600_e4963 * var_itzr);
        var_itzr = assign4600_e4965;
        var_itzr_dn0 = (assign4600_e4963 * var_itzr_dn0);
        var_itzr_dn1 = (assign4600_e4963 * var_itzr_dn1);
        var_itzr_dn2 = (assign4600_e4963 * var_itzr_dn2);
        var_itzr_dn3 = (assign4600_e4963 * var_itzr_dn3);
        var_itzr_dn4 = (assign4600_e4963 * var_itzr_dn4);
        var_itzr_dn5 = (assign4600_e4963 * var_itzr_dn5);
        var_itzr_dn6 = (assign4600_e4963 * var_itzr_dn6);
        var_itzr_dn7 = (assign4600_e4963 * var_itzr_dn7);
        var_itzr_dn8 = (assign4600_e4963 * var_itzr_dn8);
        var_itzr_dn9 = (assign4600_e4963 * var_itzr_dn9);
        var_itzr_dn10 = (assign4600_e4963 * var_itzr_dn10);
        var_itzr_dn11 = (assign4600_e4963 * var_itzr_dn11);
        var_itzr_dn12 = (assign4600_e4963 * var_itzr_dn12);
        var_itzr_dn13 = (assign4600_e4963 * var_itzr_dn13);

        let assign4610_e4968: f64 = var_vbictype;
        let assign4610_e4970: f64 = (assign4610_e4968 * var_ibc);
        var_ibc = assign4610_e4970;
        var_ibc_dn0 = (assign4610_e4968 * var_ibc_dn0);
        var_ibc_dn1 = (assign4610_e4968 * var_ibc_dn1);
        var_ibc_dn2 = (assign4610_e4968 * var_ibc_dn2);
        var_ibc_dn3 = (assign4610_e4968 * var_ibc_dn3);
        var_ibc_dn4 = (assign4610_e4968 * var_ibc_dn4);
        var_ibc_dn5 = (assign4610_e4968 * var_ibc_dn5);
        var_ibc_dn6 = (assign4610_e4968 * var_ibc_dn6);
        var_ibc_dn7 = (assign4610_e4968 * var_ibc_dn7);
        var_ibc_dn8 = (assign4610_e4968 * var_ibc_dn8);
        var_ibc_dn9 = (assign4610_e4968 * var_ibc_dn9);
        var_ibc_dn10 = (assign4610_e4968 * var_ibc_dn10);
        var_ibc_dn11 = (assign4610_e4968 * var_ibc_dn11);
        var_ibc_dn12 = (assign4610_e4968 * var_ibc_dn12);
        var_ibc_dn13 = (assign4610_e4968 * var_ibc_dn13);

        let assign4620_e4973: f64 = var_vbictype;
        let assign4620_e4975: f64 = (assign4620_e4973 * var_igcx);
        var_igcx = assign4620_e4975;
        var_igcx_dn0 = (assign4620_e4973 * var_igcx_dn0);
        var_igcx_dn1 = (assign4620_e4973 * var_igcx_dn1);
        var_igcx_dn2 = (assign4620_e4973 * var_igcx_dn2);
        var_igcx_dn3 = (assign4620_e4973 * var_igcx_dn3);
        var_igcx_dn4 = (assign4620_e4973 * var_igcx_dn4);
        var_igcx_dn5 = (assign4620_e4973 * var_igcx_dn5);
        var_igcx_dn6 = (assign4620_e4973 * var_igcx_dn6);
        var_igcx_dn7 = (assign4620_e4973 * var_igcx_dn7);
        var_igcx_dn8 = (assign4620_e4973 * var_igcx_dn8);
        var_igcx_dn9 = (assign4620_e4973 * var_igcx_dn9);
        var_igcx_dn10 = (assign4620_e4973 * var_igcx_dn10);
        var_igcx_dn11 = (assign4620_e4973 * var_igcx_dn11);
        var_igcx_dn12 = (assign4620_e4973 * var_igcx_dn12);
        var_igcx_dn13 = (assign4620_e4973 * var_igcx_dn13);

        let assign4630_e4978: f64 = var_vbictype;
        let assign4630_e4980: f64 = (assign4630_e4978 * var_ibep);
        var_ibep = assign4630_e4980;
        var_ibep_dn0 = (assign4630_e4978 * var_ibep_dn0);
        var_ibep_dn1 = (assign4630_e4978 * var_ibep_dn1);
        var_ibep_dn2 = (assign4630_e4978 * var_ibep_dn2);
        var_ibep_dn3 = (assign4630_e4978 * var_ibep_dn3);
        var_ibep_dn4 = (assign4630_e4978 * var_ibep_dn4);
        var_ibep_dn5 = (assign4630_e4978 * var_ibep_dn5);
        var_ibep_dn6 = (assign4630_e4978 * var_ibep_dn6);
        var_ibep_dn7 = (assign4630_e4978 * var_ibep_dn7);
        var_ibep_dn8 = (assign4630_e4978 * var_ibep_dn8);
        var_ibep_dn9 = (assign4630_e4978 * var_ibep_dn9);
        var_ibep_dn10 = (assign4630_e4978 * var_ibep_dn10);
        var_ibep_dn11 = (assign4630_e4978 * var_ibep_dn11);
        var_ibep_dn12 = (assign4630_e4978 * var_ibep_dn12);
        var_ibep_dn13 = (assign4630_e4978 * var_ibep_dn13);

        let assign4640_e4983: f64 = var_ircx;
        var_ircx = assign4640_e4983;
        var_ircx_dn0 = var_ircx_dn0;
        var_ircx_dn1 = var_ircx_dn1;
        var_ircx_dn2 = var_ircx_dn2;
        var_ircx_dn3 = var_ircx_dn3;
        var_ircx_dn4 = var_ircx_dn4;
        var_ircx_dn5 = var_ircx_dn5;
        var_ircx_dn6 = var_ircx_dn6;
        var_ircx_dn7 = var_ircx_dn7;
        var_ircx_dn8 = var_ircx_dn8;
        var_ircx_dn9 = var_ircx_dn9;
        var_ircx_dn10 = var_ircx_dn10;
        var_ircx_dn11 = var_ircx_dn11;
        var_ircx_dn12 = var_ircx_dn12;
        var_ircx_dn13 = var_ircx_dn13;


        *var_afac_slot = var_afac;
        *var_afac_dn0_slot = var_afac_dn0;
        *var_afac_dn1_slot = var_afac_dn1;
        *var_afac_dn10_slot = var_afac_dn10;
        *var_afac_dn11_slot = var_afac_dn11;
        *var_afac_dn12_slot = var_afac_dn12;
        *var_afac_dn13_slot = var_afac_dn13;
        *var_afac_dn2_slot = var_afac_dn2;
        *var_afac_dn3_slot = var_afac_dn3;
        *var_afac_dn4_slot = var_afac_dn4;
        *var_afac_dn5_slot = var_afac_dn5;
        *var_afac_dn6_slot = var_afac_dn6;
        *var_afac_dn7_slot = var_afac_dn7;
        *var_afac_dn8_slot = var_afac_dn8;
        *var_afac_dn9_slot = var_afac_dn9;
        *var_expn_slot = var_expn;
        *var_expn_dn0_slot = var_expn_dn0;
        *var_expn_dn1_slot = var_expn_dn1;
        *var_expn_dn10_slot = var_expn_dn10;
        *var_expn_dn11_slot = var_expn_dn11;
        *var_expn_dn12_slot = var_expn_dn12;
        *var_expn_dn13_slot = var_expn_dn13;
        *var_expn_dn2_slot = var_expn_dn2;
        *var_expn_dn3_slot = var_expn_dn3;
        *var_expn_dn4_slot = var_expn_dn4;
        *var_expn_dn5_slot = var_expn_dn5;
        *var_expn_dn6_slot = var_expn_dn6;
        *var_expn_dn7_slot = var_expn_dn7;
        *var_expn_dn8_slot = var_expn_dn8;
        *var_expn_dn9_slot = var_expn_dn9;
        *var_guard134_slot = var_guard134;
        *var_ibc_slot = var_ibc;
        *var_ibc_dn0_slot = var_ibc_dn0;
        *var_ibc_dn1_slot = var_ibc_dn1;
        *var_ibc_dn10_slot = var_ibc_dn10;
        *var_ibc_dn11_slot = var_ibc_dn11;
        *var_ibc_dn12_slot = var_ibc_dn12;
        *var_ibc_dn13_slot = var_ibc_dn13;
        *var_ibc_dn2_slot = var_ibc_dn2;
        *var_ibc_dn3_slot = var_ibc_dn3;
        *var_ibc_dn4_slot = var_ibc_dn4;
        *var_ibc_dn5_slot = var_ibc_dn5;
        *var_ibc_dn6_slot = var_ibc_dn6;
        *var_ibc_dn7_slot = var_ibc_dn7;
        *var_ibc_dn8_slot = var_ibc_dn8;
        *var_ibc_dn9_slot = var_ibc_dn9;
        *var_ibcp_slot = var_ibcp;
        *var_ibcp_dn0_slot = var_ibcp_dn0;
        *var_ibcp_dn1_slot = var_ibcp_dn1;
        *var_ibcp_dn10_slot = var_ibcp_dn10;
        *var_ibcp_dn11_slot = var_ibcp_dn11;
        *var_ibcp_dn12_slot = var_ibcp_dn12;
        *var_ibcp_dn13_slot = var_ibcp_dn13;
        *var_ibcp_dn2_slot = var_ibcp_dn2;
        *var_ibcp_dn3_slot = var_ibcp_dn3;
        *var_ibcp_dn4_slot = var_ibcp_dn4;
        *var_ibcp_dn5_slot = var_ibcp_dn5;
        *var_ibcp_dn6_slot = var_ibcp_dn6;
        *var_ibcp_dn7_slot = var_ibcp_dn7;
        *var_ibcp_dn8_slot = var_ibcp_dn8;
        *var_ibcp_dn9_slot = var_ibcp_dn9;
        *var_ibe_slot = var_ibe;
        *var_ibe_dn0_slot = var_ibe_dn0;
        *var_ibe_dn1_slot = var_ibe_dn1;
        *var_ibe_dn10_slot = var_ibe_dn10;
        *var_ibe_dn11_slot = var_ibe_dn11;
        *var_ibe_dn12_slot = var_ibe_dn12;
        *var_ibe_dn13_slot = var_ibe_dn13;
        *var_ibe_dn2_slot = var_ibe_dn2;
        *var_ibe_dn3_slot = var_ibe_dn3;
        *var_ibe_dn4_slot = var_ibe_dn4;
        *var_ibe_dn5_slot = var_ibe_dn5;
        *var_ibe_dn6_slot = var_ibe_dn6;
        *var_ibe_dn7_slot = var_ibe_dn7;
        *var_ibe_dn8_slot = var_ibe_dn8;
        *var_ibe_dn9_slot = var_ibe_dn9;
        *var_ibep_slot = var_ibep;
        *var_ibep_dn0_slot = var_ibep_dn0;
        *var_ibep_dn1_slot = var_ibep_dn1;
        *var_ibep_dn10_slot = var_ibep_dn10;
        *var_ibep_dn11_slot = var_ibep_dn11;
        *var_ibep_dn12_slot = var_ibep_dn12;
        *var_ibep_dn13_slot = var_ibep_dn13;
        *var_ibep_dn2_slot = var_ibep_dn2;
        *var_ibep_dn3_slot = var_ibep_dn3;
        *var_ibep_dn4_slot = var_ibep_dn4;
        *var_ibep_dn5_slot = var_ibep_dn5;
        *var_ibep_dn6_slot = var_ibep_dn6;
        *var_ibep_dn7_slot = var_ibep_dn7;
        *var_ibep_dn8_slot = var_ibep_dn8;
        *var_ibep_dn9_slot = var_ibep_dn9;
        *var_ibex_slot = var_ibex;
        *var_ibex_dn0_slot = var_ibex_dn0;
        *var_ibex_dn1_slot = var_ibex_dn1;
        *var_ibex_dn10_slot = var_ibex_dn10;
        *var_ibex_dn11_slot = var_ibex_dn11;
        *var_ibex_dn12_slot = var_ibex_dn12;
        *var_ibex_dn13_slot = var_ibex_dn13;
        *var_ibex_dn2_slot = var_ibex_dn2;
        *var_ibex_dn3_slot = var_ibex_dn3;
        *var_ibex_dn4_slot = var_ibex_dn4;
        *var_ibex_dn5_slot = var_ibex_dn5;
        *var_ibex_dn6_slot = var_ibex_dn6;
        *var_ibex_dn7_slot = var_ibex_dn7;
        *var_ibex_dn8_slot = var_ibex_dn8;
        *var_ibex_dn9_slot = var_ibex_dn9;
        *var_igcx_slot = var_igcx;
        *var_igcx_dn0_slot = var_igcx_dn0;
        *var_igcx_dn1_slot = var_igcx_dn1;
        *var_igcx_dn10_slot = var_igcx_dn10;
        *var_igcx_dn11_slot = var_igcx_dn11;
        *var_igcx_dn12_slot = var_igcx_dn12;
        *var_igcx_dn13_slot = var_igcx_dn13;
        *var_igcx_dn2_slot = var_igcx_dn2;
        *var_igcx_dn3_slot = var_igcx_dn3;
        *var_igcx_dn4_slot = var_igcx_dn4;
        *var_igcx_dn5_slot = var_igcx_dn5;
        *var_igcx_dn6_slot = var_igcx_dn6;
        *var_igcx_dn7_slot = var_igcx_dn7;
        *var_igcx_dn8_slot = var_igcx_dn8;
        *var_igcx_dn9_slot = var_igcx_dn9;
        *var_ircx_slot = var_ircx;
        *var_ircx_dn0_slot = var_ircx_dn0;
        *var_ircx_dn1_slot = var_ircx_dn1;
        *var_ircx_dn10_slot = var_ircx_dn10;
        *var_ircx_dn11_slot = var_ircx_dn11;
        *var_ircx_dn12_slot = var_ircx_dn12;
        *var_ircx_dn13_slot = var_ircx_dn13;
        *var_ircx_dn2_slot = var_ircx_dn2;
        *var_ircx_dn3_slot = var_ircx_dn3;
        *var_ircx_dn4_slot = var_ircx_dn4;
        *var_ircx_dn5_slot = var_ircx_dn5;
        *var_ircx_dn6_slot = var_ircx_dn6;
        *var_ircx_dn7_slot = var_ircx_dn7;
        *var_ircx_dn8_slot = var_ircx_dn8;
        *var_ircx_dn9_slot = var_ircx_dn9;
        *var_irth_slot = var_irth;
        *var_irth_dn0_slot = var_irth_dn0;
        *var_irth_dn1_slot = var_irth_dn1;
        *var_irth_dn10_slot = var_irth_dn10;
        *var_irth_dn11_slot = var_irth_dn11;
        *var_irth_dn12_slot = var_irth_dn12;
        *var_irth_dn13_slot = var_irth_dn13;
        *var_irth_dn2_slot = var_irth_dn2;
        *var_irth_dn3_slot = var_irth_dn3;
        *var_irth_dn4_slot = var_irth_dn4;
        *var_irth_dn5_slot = var_irth_dn5;
        *var_irth_dn6_slot = var_irth_dn6;
        *var_irth_dn7_slot = var_irth_dn7;
        *var_irth_dn8_slot = var_irth_dn8;
        *var_irth_dn9_slot = var_irth_dn9;
        *var_ith_slot = var_ith;
        *var_ith_dn0_slot = var_ith_dn0;
        *var_ith_dn1_slot = var_ith_dn1;
        *var_ith_dn10_slot = var_ith_dn10;
        *var_ith_dn11_slot = var_ith_dn11;
        *var_ith_dn12_slot = var_ith_dn12;
        *var_ith_dn13_slot = var_ith_dn13;
        *var_ith_dn2_slot = var_ith_dn2;
        *var_ith_dn3_slot = var_ith_dn3;
        *var_ith_dn4_slot = var_ith_dn4;
        *var_ith_dn5_slot = var_ith_dn5;
        *var_ith_dn6_slot = var_ith_dn6;
        *var_ith_dn7_slot = var_ith_dn7;
        *var_ith_dn8_slot = var_ith_dn8;
        *var_ith_dn9_slot = var_ith_dn9;
        *var_itxf_slot = var_itxf;
        *var_itxf_dn0_slot = var_itxf_dn0;
        *var_itxf_dn1_slot = var_itxf_dn1;
        *var_itxf_dn10_slot = var_itxf_dn10;
        *var_itxf_dn11_slot = var_itxf_dn11;
        *var_itxf_dn12_slot = var_itxf_dn12;
        *var_itxf_dn13_slot = var_itxf_dn13;
        *var_itxf_dn2_slot = var_itxf_dn2;
        *var_itxf_dn3_slot = var_itxf_dn3;
        *var_itxf_dn4_slot = var_itxf_dn4;
        *var_itxf_dn5_slot = var_itxf_dn5;
        *var_itxf_dn6_slot = var_itxf_dn6;
        *var_itxf_dn7_slot = var_itxf_dn7;
        *var_itxf_dn8_slot = var_itxf_dn8;
        *var_itxf_dn9_slot = var_itxf_dn9;
        *var_itzf_slot = var_itzf;
        *var_itzf_dn0_slot = var_itzf_dn0;
        *var_itzf_dn1_slot = var_itzf_dn1;
        *var_itzf_dn10_slot = var_itzf_dn10;
        *var_itzf_dn11_slot = var_itzf_dn11;
        *var_itzf_dn12_slot = var_itzf_dn12;
        *var_itzf_dn13_slot = var_itzf_dn13;
        *var_itzf_dn2_slot = var_itzf_dn2;
        *var_itzf_dn3_slot = var_itzf_dn3;
        *var_itzf_dn4_slot = var_itzf_dn4;
        *var_itzf_dn5_slot = var_itzf_dn5;
        *var_itzf_dn6_slot = var_itzf_dn6;
        *var_itzf_dn7_slot = var_itzf_dn7;
        *var_itzf_dn8_slot = var_itzf_dn8;
        *var_itzf_dn9_slot = var_itzf_dn9;
        *var_itzr_slot = var_itzr;
        *var_itzr_dn0_slot = var_itzr_dn0;
        *var_itzr_dn1_slot = var_itzr_dn1;
        *var_itzr_dn10_slot = var_itzr_dn10;
        *var_itzr_dn11_slot = var_itzr_dn11;
        *var_itzr_dn12_slot = var_itzr_dn12;
        *var_itzr_dn13_slot = var_itzr_dn13;
        *var_itzr_dn2_slot = var_itzr_dn2;
        *var_itzr_dn3_slot = var_itzr_dn3;
        *var_itzr_dn4_slot = var_itzr_dn4;
        *var_itzr_dn5_slot = var_itzr_dn5;
        *var_itzr_dn6_slot = var_itzr_dn6;
        *var_itzr_dn7_slot = var_itzr_dn7;
        *var_itzr_dn8_slot = var_itzr_dn8;
        *var_itzr_dn9_slot = var_itzr_dn9;
        *var_ixf1_slot = var_ixf1;
        *var_ixf1_dn0_slot = var_ixf1_dn0;
        *var_ixf1_dn1_slot = var_ixf1_dn1;
        *var_ixf1_dn10_slot = var_ixf1_dn10;
        *var_ixf1_dn11_slot = var_ixf1_dn11;
        *var_ixf1_dn12_slot = var_ixf1_dn12;
        *var_ixf1_dn13_slot = var_ixf1_dn13;
        *var_ixf1_dn2_slot = var_ixf1_dn2;
        *var_ixf1_dn3_slot = var_ixf1_dn3;
        *var_ixf1_dn4_slot = var_ixf1_dn4;
        *var_ixf1_dn5_slot = var_ixf1_dn5;
        *var_ixf1_dn6_slot = var_ixf1_dn6;
        *var_ixf1_dn7_slot = var_ixf1_dn7;
        *var_ixf1_dn8_slot = var_ixf1_dn8;
        *var_ixf1_dn9_slot = var_ixf1_dn9;
        *var_power_slot = var_power;
        *var_power_dn0_slot = var_power_dn0;
        *var_power_dn1_slot = var_power_dn1;
        *var_power_dn10_slot = var_power_dn10;
        *var_power_dn11_slot = var_power_dn11;
        *var_power_dn12_slot = var_power_dn12;
        *var_power_dn13_slot = var_power_dn13;
        *var_power_dn2_slot = var_power_dn2;
        *var_power_dn3_slot = var_power_dn3;
        *var_power_dn4_slot = var_power_dn4;
        *var_power_dn5_slot = var_power_dn5;
        *var_power_dn6_slot = var_power_dn6;
        *var_power_dn7_slot = var_power_dn7;
        *var_power_dn8_slot = var_power_dn8;
        *var_power_dn9_slot = var_power_dn9;
    }

    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        var_ps_t: f64,
        var_ps_t_dn0: f64,
        var_ps_t_dn1: f64,
        var_ps_t_dn10: f64,
        var_ps_t_dn11: f64,
        var_ps_t_dn12: f64,
        var_ps_t_dn13: f64,
        var_ps_t_dn2: f64,
        var_ps_t_dn3: f64,
        var_ps_t_dn4: f64,
        var_ps_t_dn5: f64,
        var_ps_t_dn6: f64,
        var_ps_t_dn7: f64,
        var_ps_t_dn8: f64,
        var_ps_t_dn9: f64,
        var_vbcp: f64,
        var_vbcp_dn0: f64,
        var_vbcp_dn1: f64,
        var_vbcp_dn10: f64,
        var_vbcp_dn11: f64,
        var_vbcp_dn12: f64,
        var_vbcp_dn13: f64,
        var_vbcp_dn2: f64,
        var_vbcp_dn3: f64,
        var_vbcp_dn4: f64,
        var_vbcp_dn5: f64,
        var_vbcp_dn6: f64,
        var_vbcp_dn7: f64,
        var_vbcp_dn8: f64,
        var_vbcp_dn9: f64,
        var_vbictype: f64,
        var_dv0__blk136_slot: &mut f64,
        var_dv0__blk136_dn0_slot: &mut f64,
        var_dv0__blk136_dn1_slot: &mut f64,
        var_dv0__blk136_dn10_slot: &mut f64,
        var_dv0__blk136_dn11_slot: &mut f64,
        var_dv0__blk136_dn12_slot: &mut f64,
        var_dv0__blk136_dn13_slot: &mut f64,
        var_dv0__blk136_dn2_slot: &mut f64,
        var_dv0__blk136_dn3_slot: &mut f64,
        var_dv0__blk136_dn4_slot: &mut f64,
        var_dv0__blk136_dn5_slot: &mut f64,
        var_dv0__blk136_dn6_slot: &mut f64,
        var_dv0__blk136_dn7_slot: &mut f64,
        var_dv0__blk136_dn8_slot: &mut f64,
        var_dv0__blk136_dn9_slot: &mut f64,
        var_dv__blk144_slot: &mut f64,
        var_dv__blk144_dn0_slot: &mut f64,
        var_dv__blk144_dn1_slot: &mut f64,
        var_dv__blk144_dn10_slot: &mut f64,
        var_dv__blk144_dn11_slot: &mut f64,
        var_dv__blk144_dn12_slot: &mut f64,
        var_dv__blk144_dn13_slot: &mut f64,
        var_dv__blk144_dn2_slot: &mut f64,
        var_dv__blk144_dn3_slot: &mut f64,
        var_dv__blk144_dn4_slot: &mut f64,
        var_dv__blk144_dn5_slot: &mut f64,
        var_dv__blk144_dn6_slot: &mut f64,
        var_dv__blk144_dn7_slot: &mut f64,
        var_dv__blk144_dn8_slot: &mut f64,
        var_dv__blk144_dn9_slot: &mut f64,
        var_dvh__blk137_slot: &mut f64,
        var_dvh__blk137_dn0_slot: &mut f64,
        var_dvh__blk137_dn1_slot: &mut f64,
        var_dvh__blk137_dn10_slot: &mut f64,
        var_dvh__blk137_dn11_slot: &mut f64,
        var_dvh__blk137_dn12_slot: &mut f64,
        var_dvh__blk137_dn13_slot: &mut f64,
        var_dvh__blk137_dn2_slot: &mut f64,
        var_dvh__blk137_dn3_slot: &mut f64,
        var_dvh__blk137_dn4_slot: &mut f64,
        var_dvh__blk137_dn5_slot: &mut f64,
        var_dvh__blk137_dn6_slot: &mut f64,
        var_dvh__blk137_dn7_slot: &mut f64,
        var_dvh__blk137_dn8_slot: &mut f64,
        var_dvh__blk137_dn9_slot: &mut f64,
        var_guard135_slot: &mut f64,
        var_guard147_slot: &mut f64,
        var_guard148_slot: &mut f64,
        var_ibcp_slot: &mut f64,
        var_ibcp_dn0_slot: &mut f64,
        var_ibcp_dn1_slot: &mut f64,
        var_ibcp_dn10_slot: &mut f64,
        var_ibcp_dn11_slot: &mut f64,
        var_ibcp_dn12_slot: &mut f64,
        var_ibcp_dn13_slot: &mut f64,
        var_ibcp_dn2_slot: &mut f64,
        var_ibcp_dn3_slot: &mut f64,
        var_ibcp_dn4_slot: &mut f64,
        var_ibcp_dn5_slot: &mut f64,
        var_ibcp_dn6_slot: &mut f64,
        var_ibcp_dn7_slot: &mut f64,
        var_ibcp_dn8_slot: &mut f64,
        var_ibcp_dn9_slot: &mut f64,
        var_iccp_slot: &mut f64,
        var_iccp_dn0_slot: &mut f64,
        var_iccp_dn1_slot: &mut f64,
        var_iccp_dn10_slot: &mut f64,
        var_iccp_dn11_slot: &mut f64,
        var_iccp_dn12_slot: &mut f64,
        var_iccp_dn13_slot: &mut f64,
        var_iccp_dn2_slot: &mut f64,
        var_iccp_dn3_slot: &mut f64,
        var_iccp_dn4_slot: &mut f64,
        var_iccp_dn5_slot: &mut f64,
        var_iccp_dn6_slot: &mut f64,
        var_iccp_dn7_slot: &mut f64,
        var_iccp_dn8_slot: &mut f64,
        var_iccp_dn9_slot: &mut f64,
        var_irbi_slot: &mut f64,
        var_irbi_dn0_slot: &mut f64,
        var_irbi_dn1_slot: &mut f64,
        var_irbi_dn10_slot: &mut f64,
        var_irbi_dn11_slot: &mut f64,
        var_irbi_dn12_slot: &mut f64,
        var_irbi_dn13_slot: &mut f64,
        var_irbi_dn2_slot: &mut f64,
        var_irbi_dn3_slot: &mut f64,
        var_irbi_dn4_slot: &mut f64,
        var_irbi_dn5_slot: &mut f64,
        var_irbi_dn6_slot: &mut f64,
        var_irbi_dn7_slot: &mut f64,
        var_irbi_dn8_slot: &mut f64,
        var_irbi_dn9_slot: &mut f64,
        var_irbp_slot: &mut f64,
        var_irbp_dn0_slot: &mut f64,
        var_irbp_dn1_slot: &mut f64,
        var_irbp_dn10_slot: &mut f64,
        var_irbp_dn11_slot: &mut f64,
        var_irbp_dn12_slot: &mut f64,
        var_irbp_dn13_slot: &mut f64,
        var_irbp_dn2_slot: &mut f64,
        var_irbp_dn3_slot: &mut f64,
        var_irbp_dn4_slot: &mut f64,
        var_irbp_dn5_slot: &mut f64,
        var_irbp_dn6_slot: &mut f64,
        var_irbp_dn7_slot: &mut f64,
        var_irbp_dn8_slot: &mut f64,
        var_irbp_dn9_slot: &mut f64,
        var_irbx_slot: &mut f64,
        var_irbx_dn0_slot: &mut f64,
        var_irbx_dn1_slot: &mut f64,
        var_irbx_dn10_slot: &mut f64,
        var_irbx_dn11_slot: &mut f64,
        var_irbx_dn12_slot: &mut f64,
        var_irbx_dn13_slot: &mut f64,
        var_irbx_dn2_slot: &mut f64,
        var_irbx_dn3_slot: &mut f64,
        var_irbx_dn4_slot: &mut f64,
        var_irbx_dn5_slot: &mut f64,
        var_irbx_dn6_slot: &mut f64,
        var_irbx_dn7_slot: &mut f64,
        var_irbx_dn8_slot: &mut f64,
        var_irbx_dn9_slot: &mut f64,
        var_irci_slot: &mut f64,
        var_irci_dn0_slot: &mut f64,
        var_irci_dn1_slot: &mut f64,
        var_irci_dn10_slot: &mut f64,
        var_irci_dn11_slot: &mut f64,
        var_irci_dn12_slot: &mut f64,
        var_irci_dn13_slot: &mut f64,
        var_irci_dn2_slot: &mut f64,
        var_irci_dn3_slot: &mut f64,
        var_irci_dn4_slot: &mut f64,
        var_irci_dn5_slot: &mut f64,
        var_irci_dn6_slot: &mut f64,
        var_irci_dn7_slot: &mut f64,
        var_irci_dn8_slot: &mut f64,
        var_irci_dn9_slot: &mut f64,
        var_ire_slot: &mut f64,
        var_ire_dn0_slot: &mut f64,
        var_ire_dn1_slot: &mut f64,
        var_ire_dn10_slot: &mut f64,
        var_ire_dn11_slot: &mut f64,
        var_ire_dn12_slot: &mut f64,
        var_ire_dn13_slot: &mut f64,
        var_ire_dn2_slot: &mut f64,
        var_ire_dn3_slot: &mut f64,
        var_ire_dn4_slot: &mut f64,
        var_ire_dn5_slot: &mut f64,
        var_ire_dn6_slot: &mut f64,
        var_ire_dn7_slot: &mut f64,
        var_ire_dn8_slot: &mut f64,
        var_ire_dn9_slot: &mut f64,
        var_irs_slot: &mut f64,
        var_irs_dn0_slot: &mut f64,
        var_irs_dn1_slot: &mut f64,
        var_irs_dn10_slot: &mut f64,
        var_irs_dn11_slot: &mut f64,
        var_irs_dn12_slot: &mut f64,
        var_irs_dn13_slot: &mut f64,
        var_irs_dn2_slot: &mut f64,
        var_irs_dn3_slot: &mut f64,
        var_irs_dn4_slot: &mut f64,
        var_irs_dn5_slot: &mut f64,
        var_irs_dn6_slot: &mut f64,
        var_irs_dn7_slot: &mut f64,
        var_irs_dn8_slot: &mut f64,
        var_irs_dn9_slot: &mut f64,
        var_irth_slot: &mut f64,
        var_irth_dn0_slot: &mut f64,
        var_irth_dn1_slot: &mut f64,
        var_irth_dn10_slot: &mut f64,
        var_irth_dn11_slot: &mut f64,
        var_irth_dn12_slot: &mut f64,
        var_irth_dn13_slot: &mut f64,
        var_irth_dn2_slot: &mut f64,
        var_irth_dn3_slot: &mut f64,
        var_irth_dn4_slot: &mut f64,
        var_irth_dn5_slot: &mut f64,
        var_irth_dn6_slot: &mut f64,
        var_irth_dn7_slot: &mut f64,
        var_irth_dn8_slot: &mut f64,
        var_irth_dn9_slot: &mut f64,
        var_ith_slot: &mut f64,
        var_ith_dn0_slot: &mut f64,
        var_ith_dn1_slot: &mut f64,
        var_ith_dn10_slot: &mut f64,
        var_ith_dn11_slot: &mut f64,
        var_ith_dn12_slot: &mut f64,
        var_ith_dn13_slot: &mut f64,
        var_ith_dn2_slot: &mut f64,
        var_ith_dn3_slot: &mut f64,
        var_ith_dn4_slot: &mut f64,
        var_ith_dn5_slot: &mut f64,
        var_ith_dn6_slot: &mut f64,
        var_ith_dn7_slot: &mut f64,
        var_ith_dn8_slot: &mut f64,
        var_ith_dn9_slot: &mut f64,
        var_mv0__blk141_slot: &mut f64,
        var_mv0__blk141_dn0_slot: &mut f64,
        var_mv0__blk141_dn1_slot: &mut f64,
        var_mv0__blk141_dn10_slot: &mut f64,
        var_mv0__blk141_dn11_slot: &mut f64,
        var_mv0__blk141_dn12_slot: &mut f64,
        var_mv0__blk141_dn13_slot: &mut f64,
        var_mv0__blk141_dn2_slot: &mut f64,
        var_mv0__blk141_dn3_slot: &mut f64,
        var_mv0__blk141_dn4_slot: &mut f64,
        var_mv0__blk141_dn5_slot: &mut f64,
        var_mv0__blk141_dn6_slot: &mut f64,
        var_mv0__blk141_dn7_slot: &mut f64,
        var_mv0__blk141_dn8_slot: &mut f64,
        var_mv0__blk141_dn9_slot: &mut f64,
        var_pwq__blk138_slot: &mut f64,
        var_q0__blk143_slot: &mut f64,
        var_q0__blk143_dn0_slot: &mut f64,
        var_q0__blk143_dn1_slot: &mut f64,
        var_q0__blk143_dn10_slot: &mut f64,
        var_q0__blk143_dn11_slot: &mut f64,
        var_q0__blk143_dn12_slot: &mut f64,
        var_q0__blk143_dn13_slot: &mut f64,
        var_q0__blk143_dn2_slot: &mut f64,
        var_q0__blk143_dn3_slot: &mut f64,
        var_q0__blk143_dn4_slot: &mut f64,
        var_q0__blk143_dn5_slot: &mut f64,
        var_q0__blk143_dn6_slot: &mut f64,
        var_q0__blk143_dn7_slot: &mut f64,
        var_q0__blk143_dn8_slot: &mut f64,
        var_q0__blk143_dn9_slot: &mut f64,
        var_qdbcp_slot: &mut f64,
        var_qdbcp_dn0_slot: &mut f64,
        var_qdbcp_dn1_slot: &mut f64,
        var_qdbcp_dn10_slot: &mut f64,
        var_qdbcp_dn11_slot: &mut f64,
        var_qdbcp_dn12_slot: &mut f64,
        var_qdbcp_dn13_slot: &mut f64,
        var_qdbcp_dn2_slot: &mut f64,
        var_qdbcp_dn3_slot: &mut f64,
        var_qdbcp_dn4_slot: &mut f64,
        var_qdbcp_dn5_slot: &mut f64,
        var_qdbcp_dn6_slot: &mut f64,
        var_qdbcp_dn7_slot: &mut f64,
        var_qdbcp_dn8_slot: &mut f64,
        var_qdbcp_dn9_slot: &mut f64,
        var_qhi__blk140_slot: &mut f64,
        var_qhi__blk140_dn0_slot: &mut f64,
        var_qhi__blk140_dn1_slot: &mut f64,
        var_qhi__blk140_dn10_slot: &mut f64,
        var_qhi__blk140_dn11_slot: &mut f64,
        var_qhi__blk140_dn12_slot: &mut f64,
        var_qhi__blk140_dn13_slot: &mut f64,
        var_qhi__blk140_dn2_slot: &mut f64,
        var_qhi__blk140_dn3_slot: &mut f64,
        var_qhi__blk140_dn4_slot: &mut f64,
        var_qhi__blk140_dn5_slot: &mut f64,
        var_qhi__blk140_dn6_slot: &mut f64,
        var_qhi__blk140_dn7_slot: &mut f64,
        var_qhi__blk140_dn8_slot: &mut f64,
        var_qhi__blk140_dn9_slot: &mut f64,
        var_qlo__blk139_slot: &mut f64,
        var_qlo__blk139_dn0_slot: &mut f64,
        var_qlo__blk139_dn1_slot: &mut f64,
        var_qlo__blk139_dn10_slot: &mut f64,
        var_qlo__blk139_dn11_slot: &mut f64,
        var_qlo__blk139_dn12_slot: &mut f64,
        var_qlo__blk139_dn13_slot: &mut f64,
        var_qlo__blk139_dn2_slot: &mut f64,
        var_qlo__blk139_dn3_slot: &mut f64,
        var_qlo__blk139_dn4_slot: &mut f64,
        var_qlo__blk139_dn5_slot: &mut f64,
        var_qlo__blk139_dn6_slot: &mut f64,
        var_qlo__blk139_dn7_slot: &mut f64,
        var_qlo__blk139_dn8_slot: &mut f64,
        var_qlo__blk139_dn9_slot: &mut f64,
        var_vl0__blk142_slot: &mut f64,
        var_vl0__blk142_dn0_slot: &mut f64,
        var_vl0__blk142_dn1_slot: &mut f64,
        var_vl0__blk142_dn10_slot: &mut f64,
        var_vl0__blk142_dn11_slot: &mut f64,
        var_vl0__blk142_dn12_slot: &mut f64,
        var_vl0__blk142_dn13_slot: &mut f64,
        var_vl0__blk142_dn2_slot: &mut f64,
        var_vl0__blk142_dn3_slot: &mut f64,
        var_vl0__blk142_dn4_slot: &mut f64,
        var_vl0__blk142_dn5_slot: &mut f64,
        var_vl0__blk142_dn6_slot: &mut f64,
        var_vl0__blk142_dn7_slot: &mut f64,
        var_vl0__blk142_dn8_slot: &mut f64,
        var_vl0__blk142_dn9_slot: &mut f64,
    ) {
        let mut var_dv0__blk136: f64 = *var_dv0__blk136_slot;
        let mut var_dv0__blk136_dn0: f64 = *var_dv0__blk136_dn0_slot;
        let mut var_dv0__blk136_dn1: f64 = *var_dv0__blk136_dn1_slot;
        let mut var_dv0__blk136_dn10: f64 = *var_dv0__blk136_dn10_slot;
        let mut var_dv0__blk136_dn11: f64 = *var_dv0__blk136_dn11_slot;
        let mut var_dv0__blk136_dn12: f64 = *var_dv0__blk136_dn12_slot;
        let mut var_dv0__blk136_dn13: f64 = *var_dv0__blk136_dn13_slot;
        let mut var_dv0__blk136_dn2: f64 = *var_dv0__blk136_dn2_slot;
        let mut var_dv0__blk136_dn3: f64 = *var_dv0__blk136_dn3_slot;
        let mut var_dv0__blk136_dn4: f64 = *var_dv0__blk136_dn4_slot;
        let mut var_dv0__blk136_dn5: f64 = *var_dv0__blk136_dn5_slot;
        let mut var_dv0__blk136_dn6: f64 = *var_dv0__blk136_dn6_slot;
        let mut var_dv0__blk136_dn7: f64 = *var_dv0__blk136_dn7_slot;
        let mut var_dv0__blk136_dn8: f64 = *var_dv0__blk136_dn8_slot;
        let mut var_dv0__blk136_dn9: f64 = *var_dv0__blk136_dn9_slot;
        let mut var_dv__blk144: f64 = *var_dv__blk144_slot;
        let mut var_dv__blk144_dn0: f64 = *var_dv__blk144_dn0_slot;
        let mut var_dv__blk144_dn1: f64 = *var_dv__blk144_dn1_slot;
        let mut var_dv__blk144_dn10: f64 = *var_dv__blk144_dn10_slot;
        let mut var_dv__blk144_dn11: f64 = *var_dv__blk144_dn11_slot;
        let mut var_dv__blk144_dn12: f64 = *var_dv__blk144_dn12_slot;
        let mut var_dv__blk144_dn13: f64 = *var_dv__blk144_dn13_slot;
        let mut var_dv__blk144_dn2: f64 = *var_dv__blk144_dn2_slot;
        let mut var_dv__blk144_dn3: f64 = *var_dv__blk144_dn3_slot;
        let mut var_dv__blk144_dn4: f64 = *var_dv__blk144_dn4_slot;
        let mut var_dv__blk144_dn5: f64 = *var_dv__blk144_dn5_slot;
        let mut var_dv__blk144_dn6: f64 = *var_dv__blk144_dn6_slot;
        let mut var_dv__blk144_dn7: f64 = *var_dv__blk144_dn7_slot;
        let mut var_dv__blk144_dn8: f64 = *var_dv__blk144_dn8_slot;
        let mut var_dv__blk144_dn9: f64 = *var_dv__blk144_dn9_slot;
        let mut var_dvh__blk137: f64 = *var_dvh__blk137_slot;
        let mut var_dvh__blk137_dn0: f64 = *var_dvh__blk137_dn0_slot;
        let mut var_dvh__blk137_dn1: f64 = *var_dvh__blk137_dn1_slot;
        let mut var_dvh__blk137_dn10: f64 = *var_dvh__blk137_dn10_slot;
        let mut var_dvh__blk137_dn11: f64 = *var_dvh__blk137_dn11_slot;
        let mut var_dvh__blk137_dn12: f64 = *var_dvh__blk137_dn12_slot;
        let mut var_dvh__blk137_dn13: f64 = *var_dvh__blk137_dn13_slot;
        let mut var_dvh__blk137_dn2: f64 = *var_dvh__blk137_dn2_slot;
        let mut var_dvh__blk137_dn3: f64 = *var_dvh__blk137_dn3_slot;
        let mut var_dvh__blk137_dn4: f64 = *var_dvh__blk137_dn4_slot;
        let mut var_dvh__blk137_dn5: f64 = *var_dvh__blk137_dn5_slot;
        let mut var_dvh__blk137_dn6: f64 = *var_dvh__blk137_dn6_slot;
        let mut var_dvh__blk137_dn7: f64 = *var_dvh__blk137_dn7_slot;
        let mut var_dvh__blk137_dn8: f64 = *var_dvh__blk137_dn8_slot;
        let mut var_dvh__blk137_dn9: f64 = *var_dvh__blk137_dn9_slot;
        let mut var_guard135: f64 = *var_guard135_slot;
        let mut var_guard147: f64 = *var_guard147_slot;
        let mut var_guard148: f64 = *var_guard148_slot;
        let mut var_ibcp: f64 = *var_ibcp_slot;
        let mut var_ibcp_dn0: f64 = *var_ibcp_dn0_slot;
        let mut var_ibcp_dn1: f64 = *var_ibcp_dn1_slot;
        let mut var_ibcp_dn10: f64 = *var_ibcp_dn10_slot;
        let mut var_ibcp_dn11: f64 = *var_ibcp_dn11_slot;
        let mut var_ibcp_dn12: f64 = *var_ibcp_dn12_slot;
        let mut var_ibcp_dn13: f64 = *var_ibcp_dn13_slot;
        let mut var_ibcp_dn2: f64 = *var_ibcp_dn2_slot;
        let mut var_ibcp_dn3: f64 = *var_ibcp_dn3_slot;
        let mut var_ibcp_dn4: f64 = *var_ibcp_dn4_slot;
        let mut var_ibcp_dn5: f64 = *var_ibcp_dn5_slot;
        let mut var_ibcp_dn6: f64 = *var_ibcp_dn6_slot;
        let mut var_ibcp_dn7: f64 = *var_ibcp_dn7_slot;
        let mut var_ibcp_dn8: f64 = *var_ibcp_dn8_slot;
        let mut var_ibcp_dn9: f64 = *var_ibcp_dn9_slot;
        let mut var_iccp: f64 = *var_iccp_slot;
        let mut var_iccp_dn0: f64 = *var_iccp_dn0_slot;
        let mut var_iccp_dn1: f64 = *var_iccp_dn1_slot;
        let mut var_iccp_dn10: f64 = *var_iccp_dn10_slot;
        let mut var_iccp_dn11: f64 = *var_iccp_dn11_slot;
        let mut var_iccp_dn12: f64 = *var_iccp_dn12_slot;
        let mut var_iccp_dn13: f64 = *var_iccp_dn13_slot;
        let mut var_iccp_dn2: f64 = *var_iccp_dn2_slot;
        let mut var_iccp_dn3: f64 = *var_iccp_dn3_slot;
        let mut var_iccp_dn4: f64 = *var_iccp_dn4_slot;
        let mut var_iccp_dn5: f64 = *var_iccp_dn5_slot;
        let mut var_iccp_dn6: f64 = *var_iccp_dn6_slot;
        let mut var_iccp_dn7: f64 = *var_iccp_dn7_slot;
        let mut var_iccp_dn8: f64 = *var_iccp_dn8_slot;
        let mut var_iccp_dn9: f64 = *var_iccp_dn9_slot;
        let mut var_irbi: f64 = *var_irbi_slot;
        let mut var_irbi_dn0: f64 = *var_irbi_dn0_slot;
        let mut var_irbi_dn1: f64 = *var_irbi_dn1_slot;
        let mut var_irbi_dn10: f64 = *var_irbi_dn10_slot;
        let mut var_irbi_dn11: f64 = *var_irbi_dn11_slot;
        let mut var_irbi_dn12: f64 = *var_irbi_dn12_slot;
        let mut var_irbi_dn13: f64 = *var_irbi_dn13_slot;
        let mut var_irbi_dn2: f64 = *var_irbi_dn2_slot;
        let mut var_irbi_dn3: f64 = *var_irbi_dn3_slot;
        let mut var_irbi_dn4: f64 = *var_irbi_dn4_slot;
        let mut var_irbi_dn5: f64 = *var_irbi_dn5_slot;
        let mut var_irbi_dn6: f64 = *var_irbi_dn6_slot;
        let mut var_irbi_dn7: f64 = *var_irbi_dn7_slot;
        let mut var_irbi_dn8: f64 = *var_irbi_dn8_slot;
        let mut var_irbi_dn9: f64 = *var_irbi_dn9_slot;
        let mut var_irbp: f64 = *var_irbp_slot;
        let mut var_irbp_dn0: f64 = *var_irbp_dn0_slot;
        let mut var_irbp_dn1: f64 = *var_irbp_dn1_slot;
        let mut var_irbp_dn10: f64 = *var_irbp_dn10_slot;
        let mut var_irbp_dn11: f64 = *var_irbp_dn11_slot;
        let mut var_irbp_dn12: f64 = *var_irbp_dn12_slot;
        let mut var_irbp_dn13: f64 = *var_irbp_dn13_slot;
        let mut var_irbp_dn2: f64 = *var_irbp_dn2_slot;
        let mut var_irbp_dn3: f64 = *var_irbp_dn3_slot;
        let mut var_irbp_dn4: f64 = *var_irbp_dn4_slot;
        let mut var_irbp_dn5: f64 = *var_irbp_dn5_slot;
        let mut var_irbp_dn6: f64 = *var_irbp_dn6_slot;
        let mut var_irbp_dn7: f64 = *var_irbp_dn7_slot;
        let mut var_irbp_dn8: f64 = *var_irbp_dn8_slot;
        let mut var_irbp_dn9: f64 = *var_irbp_dn9_slot;
        let mut var_irbx: f64 = *var_irbx_slot;
        let mut var_irbx_dn0: f64 = *var_irbx_dn0_slot;
        let mut var_irbx_dn1: f64 = *var_irbx_dn1_slot;
        let mut var_irbx_dn10: f64 = *var_irbx_dn10_slot;
        let mut var_irbx_dn11: f64 = *var_irbx_dn11_slot;
        let mut var_irbx_dn12: f64 = *var_irbx_dn12_slot;
        let mut var_irbx_dn13: f64 = *var_irbx_dn13_slot;
        let mut var_irbx_dn2: f64 = *var_irbx_dn2_slot;
        let mut var_irbx_dn3: f64 = *var_irbx_dn3_slot;
        let mut var_irbx_dn4: f64 = *var_irbx_dn4_slot;
        let mut var_irbx_dn5: f64 = *var_irbx_dn5_slot;
        let mut var_irbx_dn6: f64 = *var_irbx_dn6_slot;
        let mut var_irbx_dn7: f64 = *var_irbx_dn7_slot;
        let mut var_irbx_dn8: f64 = *var_irbx_dn8_slot;
        let mut var_irbx_dn9: f64 = *var_irbx_dn9_slot;
        let mut var_irci: f64 = *var_irci_slot;
        let mut var_irci_dn0: f64 = *var_irci_dn0_slot;
        let mut var_irci_dn1: f64 = *var_irci_dn1_slot;
        let mut var_irci_dn10: f64 = *var_irci_dn10_slot;
        let mut var_irci_dn11: f64 = *var_irci_dn11_slot;
        let mut var_irci_dn12: f64 = *var_irci_dn12_slot;
        let mut var_irci_dn13: f64 = *var_irci_dn13_slot;
        let mut var_irci_dn2: f64 = *var_irci_dn2_slot;
        let mut var_irci_dn3: f64 = *var_irci_dn3_slot;
        let mut var_irci_dn4: f64 = *var_irci_dn4_slot;
        let mut var_irci_dn5: f64 = *var_irci_dn5_slot;
        let mut var_irci_dn6: f64 = *var_irci_dn6_slot;
        let mut var_irci_dn7: f64 = *var_irci_dn7_slot;
        let mut var_irci_dn8: f64 = *var_irci_dn8_slot;
        let mut var_irci_dn9: f64 = *var_irci_dn9_slot;
        let mut var_ire: f64 = *var_ire_slot;
        let mut var_ire_dn0: f64 = *var_ire_dn0_slot;
        let mut var_ire_dn1: f64 = *var_ire_dn1_slot;
        let mut var_ire_dn10: f64 = *var_ire_dn10_slot;
        let mut var_ire_dn11: f64 = *var_ire_dn11_slot;
        let mut var_ire_dn12: f64 = *var_ire_dn12_slot;
        let mut var_ire_dn13: f64 = *var_ire_dn13_slot;
        let mut var_ire_dn2: f64 = *var_ire_dn2_slot;
        let mut var_ire_dn3: f64 = *var_ire_dn3_slot;
        let mut var_ire_dn4: f64 = *var_ire_dn4_slot;
        let mut var_ire_dn5: f64 = *var_ire_dn5_slot;
        let mut var_ire_dn6: f64 = *var_ire_dn6_slot;
        let mut var_ire_dn7: f64 = *var_ire_dn7_slot;
        let mut var_ire_dn8: f64 = *var_ire_dn8_slot;
        let mut var_ire_dn9: f64 = *var_ire_dn9_slot;
        let mut var_irs: f64 = *var_irs_slot;
        let mut var_irs_dn0: f64 = *var_irs_dn0_slot;
        let mut var_irs_dn1: f64 = *var_irs_dn1_slot;
        let mut var_irs_dn10: f64 = *var_irs_dn10_slot;
        let mut var_irs_dn11: f64 = *var_irs_dn11_slot;
        let mut var_irs_dn12: f64 = *var_irs_dn12_slot;
        let mut var_irs_dn13: f64 = *var_irs_dn13_slot;
        let mut var_irs_dn2: f64 = *var_irs_dn2_slot;
        let mut var_irs_dn3: f64 = *var_irs_dn3_slot;
        let mut var_irs_dn4: f64 = *var_irs_dn4_slot;
        let mut var_irs_dn5: f64 = *var_irs_dn5_slot;
        let mut var_irs_dn6: f64 = *var_irs_dn6_slot;
        let mut var_irs_dn7: f64 = *var_irs_dn7_slot;
        let mut var_irs_dn8: f64 = *var_irs_dn8_slot;
        let mut var_irs_dn9: f64 = *var_irs_dn9_slot;
        let mut var_irth: f64 = *var_irth_slot;
        let mut var_irth_dn0: f64 = *var_irth_dn0_slot;
        let mut var_irth_dn1: f64 = *var_irth_dn1_slot;
        let mut var_irth_dn10: f64 = *var_irth_dn10_slot;
        let mut var_irth_dn11: f64 = *var_irth_dn11_slot;
        let mut var_irth_dn12: f64 = *var_irth_dn12_slot;
        let mut var_irth_dn13: f64 = *var_irth_dn13_slot;
        let mut var_irth_dn2: f64 = *var_irth_dn2_slot;
        let mut var_irth_dn3: f64 = *var_irth_dn3_slot;
        let mut var_irth_dn4: f64 = *var_irth_dn4_slot;
        let mut var_irth_dn5: f64 = *var_irth_dn5_slot;
        let mut var_irth_dn6: f64 = *var_irth_dn6_slot;
        let mut var_irth_dn7: f64 = *var_irth_dn7_slot;
        let mut var_irth_dn8: f64 = *var_irth_dn8_slot;
        let mut var_irth_dn9: f64 = *var_irth_dn9_slot;
        let mut var_ith: f64 = *var_ith_slot;
        let mut var_ith_dn0: f64 = *var_ith_dn0_slot;
        let mut var_ith_dn1: f64 = *var_ith_dn1_slot;
        let mut var_ith_dn10: f64 = *var_ith_dn10_slot;
        let mut var_ith_dn11: f64 = *var_ith_dn11_slot;
        let mut var_ith_dn12: f64 = *var_ith_dn12_slot;
        let mut var_ith_dn13: f64 = *var_ith_dn13_slot;
        let mut var_ith_dn2: f64 = *var_ith_dn2_slot;
        let mut var_ith_dn3: f64 = *var_ith_dn3_slot;
        let mut var_ith_dn4: f64 = *var_ith_dn4_slot;
        let mut var_ith_dn5: f64 = *var_ith_dn5_slot;
        let mut var_ith_dn6: f64 = *var_ith_dn6_slot;
        let mut var_ith_dn7: f64 = *var_ith_dn7_slot;
        let mut var_ith_dn8: f64 = *var_ith_dn8_slot;
        let mut var_ith_dn9: f64 = *var_ith_dn9_slot;
        let mut var_mv0__blk141: f64 = *var_mv0__blk141_slot;
        let mut var_mv0__blk141_dn0: f64 = *var_mv0__blk141_dn0_slot;
        let mut var_mv0__blk141_dn1: f64 = *var_mv0__blk141_dn1_slot;
        let mut var_mv0__blk141_dn10: f64 = *var_mv0__blk141_dn10_slot;
        let mut var_mv0__blk141_dn11: f64 = *var_mv0__blk141_dn11_slot;
        let mut var_mv0__blk141_dn12: f64 = *var_mv0__blk141_dn12_slot;
        let mut var_mv0__blk141_dn13: f64 = *var_mv0__blk141_dn13_slot;
        let mut var_mv0__blk141_dn2: f64 = *var_mv0__blk141_dn2_slot;
        let mut var_mv0__blk141_dn3: f64 = *var_mv0__blk141_dn3_slot;
        let mut var_mv0__blk141_dn4: f64 = *var_mv0__blk141_dn4_slot;
        let mut var_mv0__blk141_dn5: f64 = *var_mv0__blk141_dn5_slot;
        let mut var_mv0__blk141_dn6: f64 = *var_mv0__blk141_dn6_slot;
        let mut var_mv0__blk141_dn7: f64 = *var_mv0__blk141_dn7_slot;
        let mut var_mv0__blk141_dn8: f64 = *var_mv0__blk141_dn8_slot;
        let mut var_mv0__blk141_dn9: f64 = *var_mv0__blk141_dn9_slot;
        let mut var_pwq__blk138: f64 = *var_pwq__blk138_slot;
        let mut var_q0__blk143: f64 = *var_q0__blk143_slot;
        let mut var_q0__blk143_dn0: f64 = *var_q0__blk143_dn0_slot;
        let mut var_q0__blk143_dn1: f64 = *var_q0__blk143_dn1_slot;
        let mut var_q0__blk143_dn10: f64 = *var_q0__blk143_dn10_slot;
        let mut var_q0__blk143_dn11: f64 = *var_q0__blk143_dn11_slot;
        let mut var_q0__blk143_dn12: f64 = *var_q0__blk143_dn12_slot;
        let mut var_q0__blk143_dn13: f64 = *var_q0__blk143_dn13_slot;
        let mut var_q0__blk143_dn2: f64 = *var_q0__blk143_dn2_slot;
        let mut var_q0__blk143_dn3: f64 = *var_q0__blk143_dn3_slot;
        let mut var_q0__blk143_dn4: f64 = *var_q0__blk143_dn4_slot;
        let mut var_q0__blk143_dn5: f64 = *var_q0__blk143_dn5_slot;
        let mut var_q0__blk143_dn6: f64 = *var_q0__blk143_dn6_slot;
        let mut var_q0__blk143_dn7: f64 = *var_q0__blk143_dn7_slot;
        let mut var_q0__blk143_dn8: f64 = *var_q0__blk143_dn8_slot;
        let mut var_q0__blk143_dn9: f64 = *var_q0__blk143_dn9_slot;
        let mut var_qdbcp: f64 = *var_qdbcp_slot;
        let mut var_qdbcp_dn0: f64 = *var_qdbcp_dn0_slot;
        let mut var_qdbcp_dn1: f64 = *var_qdbcp_dn1_slot;
        let mut var_qdbcp_dn10: f64 = *var_qdbcp_dn10_slot;
        let mut var_qdbcp_dn11: f64 = *var_qdbcp_dn11_slot;
        let mut var_qdbcp_dn12: f64 = *var_qdbcp_dn12_slot;
        let mut var_qdbcp_dn13: f64 = *var_qdbcp_dn13_slot;
        let mut var_qdbcp_dn2: f64 = *var_qdbcp_dn2_slot;
        let mut var_qdbcp_dn3: f64 = *var_qdbcp_dn3_slot;
        let mut var_qdbcp_dn4: f64 = *var_qdbcp_dn4_slot;
        let mut var_qdbcp_dn5: f64 = *var_qdbcp_dn5_slot;
        let mut var_qdbcp_dn6: f64 = *var_qdbcp_dn6_slot;
        let mut var_qdbcp_dn7: f64 = *var_qdbcp_dn7_slot;
        let mut var_qdbcp_dn8: f64 = *var_qdbcp_dn8_slot;
        let mut var_qdbcp_dn9: f64 = *var_qdbcp_dn9_slot;
        let mut var_qhi__blk140: f64 = *var_qhi__blk140_slot;
        let mut var_qhi__blk140_dn0: f64 = *var_qhi__blk140_dn0_slot;
        let mut var_qhi__blk140_dn1: f64 = *var_qhi__blk140_dn1_slot;
        let mut var_qhi__blk140_dn10: f64 = *var_qhi__blk140_dn10_slot;
        let mut var_qhi__blk140_dn11: f64 = *var_qhi__blk140_dn11_slot;
        let mut var_qhi__blk140_dn12: f64 = *var_qhi__blk140_dn12_slot;
        let mut var_qhi__blk140_dn13: f64 = *var_qhi__blk140_dn13_slot;
        let mut var_qhi__blk140_dn2: f64 = *var_qhi__blk140_dn2_slot;
        let mut var_qhi__blk140_dn3: f64 = *var_qhi__blk140_dn3_slot;
        let mut var_qhi__blk140_dn4: f64 = *var_qhi__blk140_dn4_slot;
        let mut var_qhi__blk140_dn5: f64 = *var_qhi__blk140_dn5_slot;
        let mut var_qhi__blk140_dn6: f64 = *var_qhi__blk140_dn6_slot;
        let mut var_qhi__blk140_dn7: f64 = *var_qhi__blk140_dn7_slot;
        let mut var_qhi__blk140_dn8: f64 = *var_qhi__blk140_dn8_slot;
        let mut var_qhi__blk140_dn9: f64 = *var_qhi__blk140_dn9_slot;
        let mut var_qlo__blk139: f64 = *var_qlo__blk139_slot;
        let mut var_qlo__blk139_dn0: f64 = *var_qlo__blk139_dn0_slot;
        let mut var_qlo__blk139_dn1: f64 = *var_qlo__blk139_dn1_slot;
        let mut var_qlo__blk139_dn10: f64 = *var_qlo__blk139_dn10_slot;
        let mut var_qlo__blk139_dn11: f64 = *var_qlo__blk139_dn11_slot;
        let mut var_qlo__blk139_dn12: f64 = *var_qlo__blk139_dn12_slot;
        let mut var_qlo__blk139_dn13: f64 = *var_qlo__blk139_dn13_slot;
        let mut var_qlo__blk139_dn2: f64 = *var_qlo__blk139_dn2_slot;
        let mut var_qlo__blk139_dn3: f64 = *var_qlo__blk139_dn3_slot;
        let mut var_qlo__blk139_dn4: f64 = *var_qlo__blk139_dn4_slot;
        let mut var_qlo__blk139_dn5: f64 = *var_qlo__blk139_dn5_slot;
        let mut var_qlo__blk139_dn6: f64 = *var_qlo__blk139_dn6_slot;
        let mut var_qlo__blk139_dn7: f64 = *var_qlo__blk139_dn7_slot;
        let mut var_qlo__blk139_dn8: f64 = *var_qlo__blk139_dn8_slot;
        let mut var_qlo__blk139_dn9: f64 = *var_qlo__blk139_dn9_slot;
        let mut var_vl0__blk142: f64 = *var_vl0__blk142_slot;
        let mut var_vl0__blk142_dn0: f64 = *var_vl0__blk142_dn0_slot;
        let mut var_vl0__blk142_dn1: f64 = *var_vl0__blk142_dn1_slot;
        let mut var_vl0__blk142_dn10: f64 = *var_vl0__blk142_dn10_slot;
        let mut var_vl0__blk142_dn11: f64 = *var_vl0__blk142_dn11_slot;
        let mut var_vl0__blk142_dn12: f64 = *var_vl0__blk142_dn12_slot;
        let mut var_vl0__blk142_dn13: f64 = *var_vl0__blk142_dn13_slot;
        let mut var_vl0__blk142_dn2: f64 = *var_vl0__blk142_dn2_slot;
        let mut var_vl0__blk142_dn3: f64 = *var_vl0__blk142_dn3_slot;
        let mut var_vl0__blk142_dn4: f64 = *var_vl0__blk142_dn4_slot;
        let mut var_vl0__blk142_dn5: f64 = *var_vl0__blk142_dn5_slot;
        let mut var_vl0__blk142_dn6: f64 = *var_vl0__blk142_dn6_slot;
        let mut var_vl0__blk142_dn7: f64 = *var_vl0__blk142_dn7_slot;
        let mut var_vl0__blk142_dn8: f64 = *var_vl0__blk142_dn8_slot;
        let mut var_vl0__blk142_dn9: f64 = *var_vl0__blk142_dn9_slot;

        let assign4650_e4986: f64 = var_vbictype;
        let assign4650_e4988: f64 = (assign4650_e4986 * var_irci);
        var_irci = assign4650_e4988;
        var_irci_dn0 = (assign4650_e4986 * var_irci_dn0);
        var_irci_dn1 = (assign4650_e4986 * var_irci_dn1);
        var_irci_dn2 = (assign4650_e4986 * var_irci_dn2);
        var_irci_dn3 = (assign4650_e4986 * var_irci_dn3);
        var_irci_dn4 = (assign4650_e4986 * var_irci_dn4);
        var_irci_dn5 = (assign4650_e4986 * var_irci_dn5);
        var_irci_dn6 = (assign4650_e4986 * var_irci_dn6);
        var_irci_dn7 = (assign4650_e4986 * var_irci_dn7);
        var_irci_dn8 = (assign4650_e4986 * var_irci_dn8);
        var_irci_dn9 = (assign4650_e4986 * var_irci_dn9);
        var_irci_dn10 = (assign4650_e4986 * var_irci_dn10);
        var_irci_dn11 = (assign4650_e4986 * var_irci_dn11);
        var_irci_dn12 = (assign4650_e4986 * var_irci_dn12);
        var_irci_dn13 = (assign4650_e4986 * var_irci_dn13);

        let assign4660_e4991: f64 = var_irbx;
        var_irbx = assign4660_e4991;
        var_irbx_dn0 = var_irbx_dn0;
        var_irbx_dn1 = var_irbx_dn1;
        var_irbx_dn2 = var_irbx_dn2;
        var_irbx_dn3 = var_irbx_dn3;
        var_irbx_dn4 = var_irbx_dn4;
        var_irbx_dn5 = var_irbx_dn5;
        var_irbx_dn6 = var_irbx_dn6;
        var_irbx_dn7 = var_irbx_dn7;
        var_irbx_dn8 = var_irbx_dn8;
        var_irbx_dn9 = var_irbx_dn9;
        var_irbx_dn10 = var_irbx_dn10;
        var_irbx_dn11 = var_irbx_dn11;
        var_irbx_dn12 = var_irbx_dn12;
        var_irbx_dn13 = var_irbx_dn13;

        let assign4670_e4994: f64 = var_irbi;
        var_irbi = assign4670_e4994;
        var_irbi_dn0 = var_irbi_dn0;
        var_irbi_dn1 = var_irbi_dn1;
        var_irbi_dn2 = var_irbi_dn2;
        var_irbi_dn3 = var_irbi_dn3;
        var_irbi_dn4 = var_irbi_dn4;
        var_irbi_dn5 = var_irbi_dn5;
        var_irbi_dn6 = var_irbi_dn6;
        var_irbi_dn7 = var_irbi_dn7;
        var_irbi_dn8 = var_irbi_dn8;
        var_irbi_dn9 = var_irbi_dn9;
        var_irbi_dn10 = var_irbi_dn10;
        var_irbi_dn11 = var_irbi_dn11;
        var_irbi_dn12 = var_irbi_dn12;
        var_irbi_dn13 = var_irbi_dn13;

        let assign4680_e4997: f64 = var_ire;
        var_ire = assign4680_e4997;
        var_ire_dn0 = var_ire_dn0;
        var_ire_dn1 = var_ire_dn1;
        var_ire_dn2 = var_ire_dn2;
        var_ire_dn3 = var_ire_dn3;
        var_ire_dn4 = var_ire_dn4;
        var_ire_dn5 = var_ire_dn5;
        var_ire_dn6 = var_ire_dn6;
        var_ire_dn7 = var_ire_dn7;
        var_ire_dn8 = var_ire_dn8;
        var_ire_dn9 = var_ire_dn9;
        var_ire_dn10 = var_ire_dn10;
        var_ire_dn11 = var_ire_dn11;
        var_ire_dn12 = var_ire_dn12;
        var_ire_dn13 = var_ire_dn13;

        let assign4690_e5000: f64 = var_irbp;
        var_irbp = assign4690_e5000;
        var_irbp_dn0 = var_irbp_dn0;
        var_irbp_dn1 = var_irbp_dn1;
        var_irbp_dn2 = var_irbp_dn2;
        var_irbp_dn3 = var_irbp_dn3;
        var_irbp_dn4 = var_irbp_dn4;
        var_irbp_dn5 = var_irbp_dn5;
        var_irbp_dn6 = var_irbp_dn6;
        var_irbp_dn7 = var_irbp_dn7;
        var_irbp_dn8 = var_irbp_dn8;
        var_irbp_dn9 = var_irbp_dn9;
        var_irbp_dn10 = var_irbp_dn10;
        var_irbp_dn11 = var_irbp_dn11;
        var_irbp_dn12 = var_irbp_dn12;
        var_irbp_dn13 = var_irbp_dn13;

        let assign4700_e5003: f64 = var_vbictype;
        let assign4700_e5005: f64 = (assign4700_e5003 * var_ibcp);
        var_ibcp = assign4700_e5005;
        var_ibcp_dn0 = (assign4700_e5003 * var_ibcp_dn0);
        var_ibcp_dn1 = (assign4700_e5003 * var_ibcp_dn1);
        var_ibcp_dn2 = (assign4700_e5003 * var_ibcp_dn2);
        var_ibcp_dn3 = (assign4700_e5003 * var_ibcp_dn3);
        var_ibcp_dn4 = (assign4700_e5003 * var_ibcp_dn4);
        var_ibcp_dn5 = (assign4700_e5003 * var_ibcp_dn5);
        var_ibcp_dn6 = (assign4700_e5003 * var_ibcp_dn6);
        var_ibcp_dn7 = (assign4700_e5003 * var_ibcp_dn7);
        var_ibcp_dn8 = (assign4700_e5003 * var_ibcp_dn8);
        var_ibcp_dn9 = (assign4700_e5003 * var_ibcp_dn9);
        var_ibcp_dn10 = (assign4700_e5003 * var_ibcp_dn10);
        var_ibcp_dn11 = (assign4700_e5003 * var_ibcp_dn11);
        var_ibcp_dn12 = (assign4700_e5003 * var_ibcp_dn12);
        var_ibcp_dn13 = (assign4700_e5003 * var_ibcp_dn13);

        let assign4710_e5008: f64 = var_vbictype;
        let assign4710_e5010: f64 = (assign4710_e5008 * var_iccp);
        var_iccp = assign4710_e5010;
        var_iccp_dn0 = (assign4710_e5008 * var_iccp_dn0);
        var_iccp_dn1 = (assign4710_e5008 * var_iccp_dn1);
        var_iccp_dn2 = (assign4710_e5008 * var_iccp_dn2);
        var_iccp_dn3 = (assign4710_e5008 * var_iccp_dn3);
        var_iccp_dn4 = (assign4710_e5008 * var_iccp_dn4);
        var_iccp_dn5 = (assign4710_e5008 * var_iccp_dn5);
        var_iccp_dn6 = (assign4710_e5008 * var_iccp_dn6);
        var_iccp_dn7 = (assign4710_e5008 * var_iccp_dn7);
        var_iccp_dn8 = (assign4710_e5008 * var_iccp_dn8);
        var_iccp_dn9 = (assign4710_e5008 * var_iccp_dn9);
        var_iccp_dn10 = (assign4710_e5008 * var_iccp_dn10);
        var_iccp_dn11 = (assign4710_e5008 * var_iccp_dn11);
        var_iccp_dn12 = (assign4710_e5008 * var_iccp_dn12);
        var_iccp_dn13 = (assign4710_e5008 * var_iccp_dn13);

        let assign4720_e5013: f64 = var_irs;
        var_irs = assign4720_e5013;
        var_irs_dn0 = var_irs_dn0;
        var_irs_dn1 = var_irs_dn1;
        var_irs_dn2 = var_irs_dn2;
        var_irs_dn3 = var_irs_dn3;
        var_irs_dn4 = var_irs_dn4;
        var_irs_dn5 = var_irs_dn5;
        var_irs_dn6 = var_irs_dn6;
        var_irs_dn7 = var_irs_dn7;
        var_irs_dn8 = var_irs_dn8;
        var_irs_dn9 = var_irs_dn9;
        var_irs_dn10 = var_irs_dn10;
        var_irs_dn11 = var_irs_dn11;
        var_irs_dn12 = var_irs_dn12;
        var_irs_dn13 = var_irs_dn13;

        let assign4730_e5016: f64 = var_ith;
        var_ith = assign4730_e5016;
        var_ith_dn0 = var_ith_dn0;
        var_ith_dn1 = var_ith_dn1;
        var_ith_dn2 = var_ith_dn2;
        var_ith_dn3 = var_ith_dn3;
        var_ith_dn4 = var_ith_dn4;
        var_ith_dn5 = var_ith_dn5;
        var_ith_dn6 = var_ith_dn6;
        var_ith_dn7 = var_ith_dn7;
        var_ith_dn8 = var_ith_dn8;
        var_ith_dn9 = var_ith_dn9;
        var_ith_dn10 = var_ith_dn10;
        var_ith_dn11 = var_ith_dn11;
        var_ith_dn12 = var_ith_dn12;
        var_ith_dn13 = var_ith_dn13;

        let assign4740_e5019: f64 = var_irth;
        var_irth = assign4740_e5019;
        var_irth_dn0 = var_irth_dn0;
        var_irth_dn1 = var_irth_dn1;
        var_irth_dn2 = var_irth_dn2;
        var_irth_dn3 = var_irth_dn3;
        var_irth_dn4 = var_irth_dn4;
        var_irth_dn5 = var_irth_dn5;
        var_irth_dn6 = var_irth_dn6;
        var_irth_dn7 = var_irth_dn7;
        var_irth_dn8 = var_irth_dn8;
        var_irth_dn9 = var_irth_dn9;
        var_irth_dn10 = var_irth_dn10;
        var_irth_dn11 = var_irth_dn11;
        var_irth_dn12 = var_irth_dn12;
        var_irth_dn13 = var_irth_dn13;

        let assign4750_e5022: f64 = if p.p49 > 0.0 { 1.0 } else { 0.0 };
        var_guard135 = assign4750_e5022;

        let (assign4760_e5029, assign4760_e5029_d_n0, assign4760_e5029_d_n1, assign4760_e5029_d_n2, assign4760_e5029_d_n3, assign4760_e5029_d_n4, assign4760_e5029_d_n5, assign4760_e5029_d_n6, assign4760_e5029_d_n7, assign4760_e5029_d_n8, assign4760_e5029_d_n9, assign4760_e5029_d_n10, assign4760_e5029_d_n11, assign4760_e5029_d_n12, assign4760_e5029_d_n13,) = {
    if (var_guard135 != 0.0) {
        let assign4760_e5025: f64 = (-var_ps_t);
        let assign4760_e5027: f64 = (assign4760_e5025 * p.p34);
        (assign4760_e5027, ((-var_ps_t_dn0) * p.p34), ((-var_ps_t_dn1) * p.p34), ((-var_ps_t_dn2) * p.p34), ((-var_ps_t_dn3) * p.p34), ((-var_ps_t_dn4) * p.p34), ((-var_ps_t_dn5) * p.p34), ((-var_ps_t_dn6) * p.p34), ((-var_ps_t_dn7) * p.p34), ((-var_ps_t_dn8) * p.p34), ((-var_ps_t_dn9) * p.p34), ((-var_ps_t_dn10) * p.p34), ((-var_ps_t_dn11) * p.p34), ((-var_ps_t_dn12) * p.p34), ((-var_ps_t_dn13) * p.p34),)
    } else {
        (var_dv0__blk136, var_dv0__blk136_dn0, var_dv0__blk136_dn1, var_dv0__blk136_dn2, var_dv0__blk136_dn3, var_dv0__blk136_dn4, var_dv0__blk136_dn5, var_dv0__blk136_dn6, var_dv0__blk136_dn7, var_dv0__blk136_dn8, var_dv0__blk136_dn9, var_dv0__blk136_dn10, var_dv0__blk136_dn11, var_dv0__blk136_dn12, var_dv0__blk136_dn13,)
    }
};
        var_dv0__blk136 = assign4760_e5029;
        var_dv0__blk136_dn0 = assign4760_e5029_d_n0;
        var_dv0__blk136_dn1 = assign4760_e5029_d_n1;
        var_dv0__blk136_dn2 = assign4760_e5029_d_n2;
        var_dv0__blk136_dn3 = assign4760_e5029_d_n3;
        var_dv0__blk136_dn4 = assign4760_e5029_d_n4;
        var_dv0__blk136_dn5 = assign4760_e5029_d_n5;
        var_dv0__blk136_dn6 = assign4760_e5029_d_n6;
        var_dv0__blk136_dn7 = assign4760_e5029_d_n7;
        var_dv0__blk136_dn8 = assign4760_e5029_d_n8;
        var_dv0__blk136_dn9 = assign4760_e5029_d_n9;
        var_dv0__blk136_dn10 = assign4760_e5029_d_n10;
        var_dv0__blk136_dn11 = assign4760_e5029_d_n11;
        var_dv0__blk136_dn12 = assign4760_e5029_d_n12;
        var_dv0__blk136_dn13 = assign4760_e5029_d_n13;

        let assign4770_e5032: f64 = if p.p52 <= 0.0 { 1.0 } else { 0.0 };
        var_guard147 = assign4770_e5032;

        let (assign4780_e5040, assign4780_e5040_d_n0, assign4780_e5040_d_n1, assign4780_e5040_d_n2, assign4780_e5040_d_n3, assign4780_e5040_d_n4, assign4780_e5040_d_n5, assign4780_e5040_d_n6, assign4780_e5040_d_n7, assign4780_e5040_d_n8, assign4780_e5040_d_n9, assign4780_e5040_d_n10, assign4780_e5040_d_n11, assign4780_e5040_d_n12, assign4780_e5040_d_n13,) = {
    if ((var_guard135 != 0.0) && (var_guard147 != 0.0)) {
        let assign4780_e5038: f64 = (var_vbcp + var_dv0__blk136);
        (assign4780_e5038, (var_vbcp_dn0 + var_dv0__blk136_dn0), (var_vbcp_dn1 + var_dv0__blk136_dn1), (var_vbcp_dn2 + var_dv0__blk136_dn2), (var_vbcp_dn3 + var_dv0__blk136_dn3), (var_vbcp_dn4 + var_dv0__blk136_dn4), (var_vbcp_dn5 + var_dv0__blk136_dn5), (var_vbcp_dn6 + var_dv0__blk136_dn6), (var_vbcp_dn7 + var_dv0__blk136_dn7), (var_vbcp_dn8 + var_dv0__blk136_dn8), (var_vbcp_dn9 + var_dv0__blk136_dn9), (var_vbcp_dn10 + var_dv0__blk136_dn10), (var_vbcp_dn11 + var_dv0__blk136_dn11), (var_vbcp_dn12 + var_dv0__blk136_dn12), (var_vbcp_dn13 + var_dv0__blk136_dn13),)
    } else {
        (var_dvh__blk137, var_dvh__blk137_dn0, var_dvh__blk137_dn1, var_dvh__blk137_dn2, var_dvh__blk137_dn3, var_dvh__blk137_dn4, var_dvh__blk137_dn5, var_dvh__blk137_dn6, var_dvh__blk137_dn7, var_dvh__blk137_dn8, var_dvh__blk137_dn9, var_dvh__blk137_dn10, var_dvh__blk137_dn11, var_dvh__blk137_dn12, var_dvh__blk137_dn13,)
    }
};
        var_dvh__blk137 = assign4780_e5040;
        var_dvh__blk137_dn0 = assign4780_e5040_d_n0;
        var_dvh__blk137_dn1 = assign4780_e5040_d_n1;
        var_dvh__blk137_dn2 = assign4780_e5040_d_n2;
        var_dvh__blk137_dn3 = assign4780_e5040_d_n3;
        var_dvh__blk137_dn4 = assign4780_e5040_d_n4;
        var_dvh__blk137_dn5 = assign4780_e5040_d_n5;
        var_dvh__blk137_dn6 = assign4780_e5040_d_n6;
        var_dvh__blk137_dn7 = assign4780_e5040_d_n7;
        var_dvh__blk137_dn8 = assign4780_e5040_d_n8;
        var_dvh__blk137_dn9 = assign4780_e5040_d_n9;
        var_dvh__blk137_dn10 = assign4780_e5040_d_n10;
        var_dvh__blk137_dn11 = assign4780_e5040_d_n11;
        var_dvh__blk137_dn12 = assign4780_e5040_d_n12;
        var_dvh__blk137_dn13 = assign4780_e5040_d_n13;

        let assign4790_e5043: f64 = if var_dvh__blk137 > 0.0 { 1.0 } else { 0.0 };
        var_guard148 = assign4790_e5043;

        let (assign4800_e5056,) = {
    if (((var_guard135 != 0.0) && (var_guard147 != 0.0)) && (var_guard148 != 0.0)) {
        let assign4800_e5051: f64 = (1.0 - p.p34);
        let assign4800_e5053: f64 = (-p.p51);
        let assign4800_e5054: f64 = (assign4800_e5051).powf(assign4800_e5053);
        (assign4800_e5054,)
    } else {
        (var_pwq__blk138,)
    }
};
        var_pwq__blk138 = assign4800_e5056;

        let (assign4810_e5076, assign4810_e5076_d_n0, assign4810_e5076_d_n1, assign4810_e5076_d_n2, assign4810_e5076_d_n3, assign4810_e5076_d_n4, assign4810_e5076_d_n5, assign4810_e5076_d_n6, assign4810_e5076_d_n7, assign4810_e5076_d_n8, assign4810_e5076_d_n9, assign4810_e5076_d_n10, assign4810_e5076_d_n11, assign4810_e5076_d_n12, assign4810_e5076_d_n13,) = {
    if (((var_guard135 != 0.0) && (var_guard147 != 0.0)) && (var_guard148 != 0.0)) {
        let assign4810_e5067: f64 = (1.0 - p.p34);
        let assign4810_e5068: f64 = (var_pwq__blk138 * assign4810_e5067);
        let assign4810_e5069: f64 = (1.0 - assign4810_e5068);
        let assign4810_e5070: f64 = (var_ps_t * assign4810_e5069);
        let assign4810_e5073: f64 = (1.0 - p.p51);
        let assign4810_e5074: f64 = (assign4810_e5070 / assign4810_e5073);
        (assign4810_e5074, ((var_ps_t_dn0 * assign4810_e5069) / assign4810_e5073), ((var_ps_t_dn1 * assign4810_e5069) / assign4810_e5073), ((var_ps_t_dn2 * assign4810_e5069) / assign4810_e5073), ((var_ps_t_dn3 * assign4810_e5069) / assign4810_e5073), ((var_ps_t_dn4 * assign4810_e5069) / assign4810_e5073), ((var_ps_t_dn5 * assign4810_e5069) / assign4810_e5073), ((var_ps_t_dn6 * assign4810_e5069) / assign4810_e5073), ((var_ps_t_dn7 * assign4810_e5069) / assign4810_e5073), ((var_ps_t_dn8 * assign4810_e5069) / assign4810_e5073), ((var_ps_t_dn9 * assign4810_e5069) / assign4810_e5073), ((var_ps_t_dn10 * assign4810_e5069) / assign4810_e5073), ((var_ps_t_dn11 * assign4810_e5069) / assign4810_e5073), ((var_ps_t_dn12 * assign4810_e5069) / assign4810_e5073), ((var_ps_t_dn13 * assign4810_e5069) / assign4810_e5073),)
    } else {
        (var_qlo__blk139, var_qlo__blk139_dn0, var_qlo__blk139_dn1, var_qlo__blk139_dn2, var_qlo__blk139_dn3, var_qlo__blk139_dn4, var_qlo__blk139_dn5, var_qlo__blk139_dn6, var_qlo__blk139_dn7, var_qlo__blk139_dn8, var_qlo__blk139_dn9, var_qlo__blk139_dn10, var_qlo__blk139_dn11, var_qlo__blk139_dn12, var_qlo__blk139_dn13,)
    }
};
        var_qlo__blk139 = assign4810_e5076;
        var_qlo__blk139_dn0 = assign4810_e5076_d_n0;
        var_qlo__blk139_dn1 = assign4810_e5076_d_n1;
        var_qlo__blk139_dn2 = assign4810_e5076_d_n2;
        var_qlo__blk139_dn3 = assign4810_e5076_d_n3;
        var_qlo__blk139_dn4 = assign4810_e5076_d_n4;
        var_qlo__blk139_dn5 = assign4810_e5076_d_n5;
        var_qlo__blk139_dn6 = assign4810_e5076_d_n6;
        var_qlo__blk139_dn7 = assign4810_e5076_d_n7;
        var_qlo__blk139_dn8 = assign4810_e5076_d_n8;
        var_qlo__blk139_dn9 = assign4810_e5076_d_n9;
        var_qlo__blk139_dn10 = assign4810_e5076_d_n10;
        var_qlo__blk139_dn11 = assign4810_e5076_d_n11;
        var_qlo__blk139_dn12 = assign4810_e5076_d_n12;
        var_qlo__blk139_dn13 = assign4810_e5076_d_n13;

        let (assign4820_e5100, assign4820_e5100_d_n0, assign4820_e5100_d_n1, assign4820_e5100_d_n2, assign4820_e5100_d_n3, assign4820_e5100_d_n4, assign4820_e5100_d_n5, assign4820_e5100_d_n6, assign4820_e5100_d_n7, assign4820_e5100_d_n8, assign4820_e5100_d_n9, assign4820_e5100_d_n10, assign4820_e5100_d_n11, assign4820_e5100_d_n12, assign4820_e5100_d_n13,) = {
    if (((var_guard135 != 0.0) && (var_guard147 != 0.0)) && (var_guard148 != 0.0)) {
        let assign4820_e5086: f64 = (0.5 * p.p51);
        let assign4820_e5088: f64 = (assign4820_e5086 * var_dvh__blk137);
        let assign4820_e5092: f64 = (1.0 - p.p34);
        let assign4820_e5093: f64 = (var_ps_t * assign4820_e5092);
        let assign4820_e5094: f64 = (assign4820_e5088 / assign4820_e5093);
        let assign4820_e5095: f64 = (1.0 + assign4820_e5094);
        let assign4820_e5096: f64 = (var_dvh__blk137 * assign4820_e5095);
        let assign4820_e5098: f64 = (assign4820_e5096 * var_pwq__blk138);
        (assign4820_e5098, (((var_dvh__blk137_dn0 * assign4820_e5095) + (var_dvh__blk137 * ((((assign4820_e5086 * var_dvh__blk137_dn0) * assign4820_e5093) - (assign4820_e5088 * (var_ps_t_dn0 * assign4820_e5092))) / (assign4820_e5093 * assign4820_e5093)))) * var_pwq__blk138), (((var_dvh__blk137_dn1 * assign4820_e5095) + (var_dvh__blk137 * ((((assign4820_e5086 * var_dvh__blk137_dn1) * assign4820_e5093) - (assign4820_e5088 * (var_ps_t_dn1 * assign4820_e5092))) / (assign4820_e5093 * assign4820_e5093)))) * var_pwq__blk138), (((var_dvh__blk137_dn2 * assign4820_e5095) + (var_dvh__blk137 * ((((assign4820_e5086 * var_dvh__blk137_dn2) * assign4820_e5093) - (assign4820_e5088 * (var_ps_t_dn2 * assign4820_e5092))) / (assign4820_e5093 * assign4820_e5093)))) * var_pwq__blk138), (((var_dvh__blk137_dn3 * assign4820_e5095) + (var_dvh__blk137 * ((((assign4820_e5086 * var_dvh__blk137_dn3) * assign4820_e5093) - (assign4820_e5088 * (var_ps_t_dn3 * assign4820_e5092))) / (assign4820_e5093 * assign4820_e5093)))) * var_pwq__blk138), (((var_dvh__blk137_dn4 * assign4820_e5095) + (var_dvh__blk137 * ((((assign4820_e5086 * var_dvh__blk137_dn4) * assign4820_e5093) - (assign4820_e5088 * (var_ps_t_dn4 * assign4820_e5092))) / (assign4820_e5093 * assign4820_e5093)))) * var_pwq__blk138), (((var_dvh__blk137_dn5 * assign4820_e5095) + (var_dvh__blk137 * ((((assign4820_e5086 * var_dvh__blk137_dn5) * assign4820_e5093) - (assign4820_e5088 * (var_ps_t_dn5 * assign4820_e5092))) / (assign4820_e5093 * assign4820_e5093)))) * var_pwq__blk138), (((var_dvh__blk137_dn6 * assign4820_e5095) + (var_dvh__blk137 * ((((assign4820_e5086 * var_dvh__blk137_dn6) * assign4820_e5093) - (assign4820_e5088 * (var_ps_t_dn6 * assign4820_e5092))) / (assign4820_e5093 * assign4820_e5093)))) * var_pwq__blk138), (((var_dvh__blk137_dn7 * assign4820_e5095) + (var_dvh__blk137 * ((((assign4820_e5086 * var_dvh__blk137_dn7) * assign4820_e5093) - (assign4820_e5088 * (var_ps_t_dn7 * assign4820_e5092))) / (assign4820_e5093 * assign4820_e5093)))) * var_pwq__blk138), (((var_dvh__blk137_dn8 * assign4820_e5095) + (var_dvh__blk137 * ((((assign4820_e5086 * var_dvh__blk137_dn8) * assign4820_e5093) - (assign4820_e5088 * (var_ps_t_dn8 * assign4820_e5092))) / (assign4820_e5093 * assign4820_e5093)))) * var_pwq__blk138), (((var_dvh__blk137_dn9 * assign4820_e5095) + (var_dvh__blk137 * ((((assign4820_e5086 * var_dvh__blk137_dn9) * assign4820_e5093) - (assign4820_e5088 * (var_ps_t_dn9 * assign4820_e5092))) / (assign4820_e5093 * assign4820_e5093)))) * var_pwq__blk138), (((var_dvh__blk137_dn10 * assign4820_e5095) + (var_dvh__blk137 * ((((assign4820_e5086 * var_dvh__blk137_dn10) * assign4820_e5093) - (assign4820_e5088 * (var_ps_t_dn10 * assign4820_e5092))) / (assign4820_e5093 * assign4820_e5093)))) * var_pwq__blk138), (((var_dvh__blk137_dn11 * assign4820_e5095) + (var_dvh__blk137 * ((((assign4820_e5086 * var_dvh__blk137_dn11) * assign4820_e5093) - (assign4820_e5088 * (var_ps_t_dn11 * assign4820_e5092))) / (assign4820_e5093 * assign4820_e5093)))) * var_pwq__blk138), (((var_dvh__blk137_dn12 * assign4820_e5095) + (var_dvh__blk137 * ((((assign4820_e5086 * var_dvh__blk137_dn12) * assign4820_e5093) - (assign4820_e5088 * (var_ps_t_dn12 * assign4820_e5092))) / (assign4820_e5093 * assign4820_e5093)))) * var_pwq__blk138), (((var_dvh__blk137_dn13 * assign4820_e5095) + (var_dvh__blk137 * ((((assign4820_e5086 * var_dvh__blk137_dn13) * assign4820_e5093) - (assign4820_e5088 * (var_ps_t_dn13 * assign4820_e5092))) / (assign4820_e5093 * assign4820_e5093)))) * var_pwq__blk138),)
    } else {
        (var_qhi__blk140, var_qhi__blk140_dn0, var_qhi__blk140_dn1, var_qhi__blk140_dn2, var_qhi__blk140_dn3, var_qhi__blk140_dn4, var_qhi__blk140_dn5, var_qhi__blk140_dn6, var_qhi__blk140_dn7, var_qhi__blk140_dn8, var_qhi__blk140_dn9, var_qhi__blk140_dn10, var_qhi__blk140_dn11, var_qhi__blk140_dn12, var_qhi__blk140_dn13,)
    }
};
        var_qhi__blk140 = assign4820_e5100;
        var_qhi__blk140_dn0 = assign4820_e5100_d_n0;
        var_qhi__blk140_dn1 = assign4820_e5100_d_n1;
        var_qhi__blk140_dn2 = assign4820_e5100_d_n2;
        var_qhi__blk140_dn3 = assign4820_e5100_d_n3;
        var_qhi__blk140_dn4 = assign4820_e5100_d_n4;
        var_qhi__blk140_dn5 = assign4820_e5100_d_n5;
        var_qhi__blk140_dn6 = assign4820_e5100_d_n6;
        var_qhi__blk140_dn7 = assign4820_e5100_d_n7;
        var_qhi__blk140_dn8 = assign4820_e5100_d_n8;
        var_qhi__blk140_dn9 = assign4820_e5100_d_n9;
        var_qhi__blk140_dn10 = assign4820_e5100_d_n10;
        var_qhi__blk140_dn11 = assign4820_e5100_d_n11;
        var_qhi__blk140_dn12 = assign4820_e5100_d_n12;
        var_qhi__blk140_dn13 = assign4820_e5100_d_n13;

        let (assign4830_e5125, assign4830_e5125_d_n0, assign4830_e5125_d_n1, assign4830_e5125_d_n2, assign4830_e5125_d_n3, assign4830_e5125_d_n4, assign4830_e5125_d_n5, assign4830_e5125_d_n6, assign4830_e5125_d_n7, assign4830_e5125_d_n8, assign4830_e5125_d_n9, assign4830_e5125_d_n10, assign4830_e5125_d_n11, assign4830_e5125_d_n12, assign4830_e5125_d_n13,) = {
    if (((var_guard135 != 0.0) && (var_guard147 != 0.0)) && (var_guard148 == 0.0)) {
        let assign4830_e5112: f64 = (var_vbcp / var_ps_t);
        let assign4830_e5113: f64 = (1.0 - assign4830_e5112);
        let assign4830_e5116: f64 = (1.0 - p.p51);
        let assign4830_e5117: f64 = (assign4830_e5113).powf(assign4830_e5116);
        let assign4830_e5118: f64 = (1.0 - assign4830_e5117);
        let assign4830_e5119: f64 = (var_ps_t * assign4830_e5118);
        let assign4830_e5122: f64 = (1.0 - p.p51);
        let assign4830_e5123: f64 = (assign4830_e5119 / assign4830_e5122);
        (assign4830_e5123, (((var_ps_t_dn0 * assign4830_e5118) + (var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(((var_vbcp_dn0 * var_ps_t) - (var_vbcp * var_ps_t_dn0)) / (var_ps_t * var_ps_t))))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(((var_vbcp_dn0 * var_ps_t) - (var_vbcp * var_ps_t_dn0)) / (var_ps_t * var_ps_t))) / assign4830_e5113))) }))) / assign4830_e5122), (((var_ps_t_dn1 * assign4830_e5118) + (var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(((var_vbcp_dn1 * var_ps_t) - (var_vbcp * var_ps_t_dn1)) / (var_ps_t * var_ps_t))))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(((var_vbcp_dn1 * var_ps_t) - (var_vbcp * var_ps_t_dn1)) / (var_ps_t * var_ps_t))) / assign4830_e5113))) }))) / assign4830_e5122), (((var_ps_t_dn2 * assign4830_e5118) + (var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(((var_vbcp_dn2 * var_ps_t) - (var_vbcp * var_ps_t_dn2)) / (var_ps_t * var_ps_t))))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(((var_vbcp_dn2 * var_ps_t) - (var_vbcp * var_ps_t_dn2)) / (var_ps_t * var_ps_t))) / assign4830_e5113))) }))) / assign4830_e5122), (((var_ps_t_dn3 * assign4830_e5118) + (var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(((var_vbcp_dn3 * var_ps_t) - (var_vbcp * var_ps_t_dn3)) / (var_ps_t * var_ps_t))))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(((var_vbcp_dn3 * var_ps_t) - (var_vbcp * var_ps_t_dn3)) / (var_ps_t * var_ps_t))) / assign4830_e5113))) }))) / assign4830_e5122), (((var_ps_t_dn4 * assign4830_e5118) + (var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(((var_vbcp_dn4 * var_ps_t) - (var_vbcp * var_ps_t_dn4)) / (var_ps_t * var_ps_t))))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(((var_vbcp_dn4 * var_ps_t) - (var_vbcp * var_ps_t_dn4)) / (var_ps_t * var_ps_t))) / assign4830_e5113))) }))) / assign4830_e5122), (((var_ps_t_dn5 * assign4830_e5118) + (var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(((var_vbcp_dn5 * var_ps_t) - (var_vbcp * var_ps_t_dn5)) / (var_ps_t * var_ps_t))))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(((var_vbcp_dn5 * var_ps_t) - (var_vbcp * var_ps_t_dn5)) / (var_ps_t * var_ps_t))) / assign4830_e5113))) }))) / assign4830_e5122), (((var_ps_t_dn6 * assign4830_e5118) + (var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(((var_vbcp_dn6 * var_ps_t) - (var_vbcp * var_ps_t_dn6)) / (var_ps_t * var_ps_t))))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(((var_vbcp_dn6 * var_ps_t) - (var_vbcp * var_ps_t_dn6)) / (var_ps_t * var_ps_t))) / assign4830_e5113))) }))) / assign4830_e5122), (((var_ps_t_dn7 * assign4830_e5118) + (var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(((var_vbcp_dn7 * var_ps_t) - (var_vbcp * var_ps_t_dn7)) / (var_ps_t * var_ps_t))))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(((var_vbcp_dn7 * var_ps_t) - (var_vbcp * var_ps_t_dn7)) / (var_ps_t * var_ps_t))) / assign4830_e5113))) }))) / assign4830_e5122), (((var_ps_t_dn8 * assign4830_e5118) + (var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(((var_vbcp_dn8 * var_ps_t) - (var_vbcp * var_ps_t_dn8)) / (var_ps_t * var_ps_t))))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(((var_vbcp_dn8 * var_ps_t) - (var_vbcp * var_ps_t_dn8)) / (var_ps_t * var_ps_t))) / assign4830_e5113))) }))) / assign4830_e5122), (((var_ps_t_dn9 * assign4830_e5118) + (var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(((var_vbcp_dn9 * var_ps_t) - (var_vbcp * var_ps_t_dn9)) / (var_ps_t * var_ps_t))))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(((var_vbcp_dn9 * var_ps_t) - (var_vbcp * var_ps_t_dn9)) / (var_ps_t * var_ps_t))) / assign4830_e5113))) }))) / assign4830_e5122), (((var_ps_t_dn10 * assign4830_e5118) + (var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(((var_vbcp_dn10 * var_ps_t) - (var_vbcp * var_ps_t_dn10)) / (var_ps_t * var_ps_t))))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(((var_vbcp_dn10 * var_ps_t) - (var_vbcp * var_ps_t_dn10)) / (var_ps_t * var_ps_t))) / assign4830_e5113))) }))) / assign4830_e5122), (((var_ps_t_dn11 * assign4830_e5118) + (var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(((var_vbcp_dn11 * var_ps_t) - (var_vbcp * var_ps_t_dn11)) / (var_ps_t * var_ps_t))))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(((var_vbcp_dn11 * var_ps_t) - (var_vbcp * var_ps_t_dn11)) / (var_ps_t * var_ps_t))) / assign4830_e5113))) }))) / assign4830_e5122), (((var_ps_t_dn12 * assign4830_e5118) + (var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(((var_vbcp_dn12 * var_ps_t) - (var_vbcp * var_ps_t_dn12)) / (var_ps_t * var_ps_t))))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(((var_vbcp_dn12 * var_ps_t) - (var_vbcp * var_ps_t_dn12)) / (var_ps_t * var_ps_t))) / assign4830_e5113))) }))) / assign4830_e5122), (((var_ps_t_dn13 * assign4830_e5118) + (var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(((var_vbcp_dn13 * var_ps_t) - (var_vbcp * var_ps_t_dn13)) / (var_ps_t * var_ps_t))))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(((var_vbcp_dn13 * var_ps_t) - (var_vbcp * var_ps_t_dn13)) / (var_ps_t * var_ps_t))) / assign4830_e5113))) }))) / assign4830_e5122),)
    } else {
        (var_qlo__blk139, var_qlo__blk139_dn0, var_qlo__blk139_dn1, var_qlo__blk139_dn2, var_qlo__blk139_dn3, var_qlo__blk139_dn4, var_qlo__blk139_dn5, var_qlo__blk139_dn6, var_qlo__blk139_dn7, var_qlo__blk139_dn8, var_qlo__blk139_dn9, var_qlo__blk139_dn10, var_qlo__blk139_dn11, var_qlo__blk139_dn12, var_qlo__blk139_dn13,)
    }
};
        var_qlo__blk139 = assign4830_e5125;
        var_qlo__blk139_dn0 = assign4830_e5125_d_n0;
        var_qlo__blk139_dn1 = assign4830_e5125_d_n1;
        var_qlo__blk139_dn2 = assign4830_e5125_d_n2;
        var_qlo__blk139_dn3 = assign4830_e5125_d_n3;
        var_qlo__blk139_dn4 = assign4830_e5125_d_n4;
        var_qlo__blk139_dn5 = assign4830_e5125_d_n5;
        var_qlo__blk139_dn6 = assign4830_e5125_d_n6;
        var_qlo__blk139_dn7 = assign4830_e5125_d_n7;
        var_qlo__blk139_dn8 = assign4830_e5125_d_n8;
        var_qlo__blk139_dn9 = assign4830_e5125_d_n9;
        var_qlo__blk139_dn10 = assign4830_e5125_d_n10;
        var_qlo__blk139_dn11 = assign4830_e5125_d_n11;
        var_qlo__blk139_dn12 = assign4830_e5125_d_n12;
        var_qlo__blk139_dn13 = assign4830_e5125_d_n13;

        let (assign4840_e5134, assign4840_e5134_d_n0, assign4840_e5134_d_n1, assign4840_e5134_d_n2, assign4840_e5134_d_n3, assign4840_e5134_d_n4, assign4840_e5134_d_n5, assign4840_e5134_d_n6, assign4840_e5134_d_n7, assign4840_e5134_d_n8, assign4840_e5134_d_n9, assign4840_e5134_d_n10, assign4840_e5134_d_n11, assign4840_e5134_d_n12, assign4840_e5134_d_n13,) = {
    if (((var_guard135 != 0.0) && (var_guard147 != 0.0)) && (var_guard148 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi__blk140, var_qhi__blk140_dn0, var_qhi__blk140_dn1, var_qhi__blk140_dn2, var_qhi__blk140_dn3, var_qhi__blk140_dn4, var_qhi__blk140_dn5, var_qhi__blk140_dn6, var_qhi__blk140_dn7, var_qhi__blk140_dn8, var_qhi__blk140_dn9, var_qhi__blk140_dn10, var_qhi__blk140_dn11, var_qhi__blk140_dn12, var_qhi__blk140_dn13,)
    }
};
        var_qhi__blk140 = assign4840_e5134;
        var_qhi__blk140_dn0 = assign4840_e5134_d_n0;
        var_qhi__blk140_dn1 = assign4840_e5134_d_n1;
        var_qhi__blk140_dn2 = assign4840_e5134_d_n2;
        var_qhi__blk140_dn3 = assign4840_e5134_d_n3;
        var_qhi__blk140_dn4 = assign4840_e5134_d_n4;
        var_qhi__blk140_dn5 = assign4840_e5134_d_n5;
        var_qhi__blk140_dn6 = assign4840_e5134_d_n6;
        var_qhi__blk140_dn7 = assign4840_e5134_d_n7;
        var_qhi__blk140_dn8 = assign4840_e5134_d_n8;
        var_qhi__blk140_dn9 = assign4840_e5134_d_n9;
        var_qhi__blk140_dn10 = assign4840_e5134_d_n10;
        var_qhi__blk140_dn11 = assign4840_e5134_d_n11;
        var_qhi__blk140_dn12 = assign4840_e5134_d_n12;
        var_qhi__blk140_dn13 = assign4840_e5134_d_n13;

        let (assign4850_e5142, assign4850_e5142_d_n0, assign4850_e5142_d_n1, assign4850_e5142_d_n2, assign4850_e5142_d_n3, assign4850_e5142_d_n4, assign4850_e5142_d_n5, assign4850_e5142_d_n6, assign4850_e5142_d_n7, assign4850_e5142_d_n8, assign4850_e5142_d_n9, assign4850_e5142_d_n10, assign4850_e5142_d_n11, assign4850_e5142_d_n12, assign4850_e5142_d_n13,) = {
    if ((var_guard135 != 0.0) && (var_guard147 != 0.0)) {
        let assign4850_e5140: f64 = (var_qlo__blk139 + var_qhi__blk140);
        (assign4850_e5140, (var_qlo__blk139_dn0 + var_qhi__blk140_dn0), (var_qlo__blk139_dn1 + var_qhi__blk140_dn1), (var_qlo__blk139_dn2 + var_qhi__blk140_dn2), (var_qlo__blk139_dn3 + var_qhi__blk140_dn3), (var_qlo__blk139_dn4 + var_qhi__blk140_dn4), (var_qlo__blk139_dn5 + var_qhi__blk140_dn5), (var_qlo__blk139_dn6 + var_qhi__blk140_dn6), (var_qlo__blk139_dn7 + var_qhi__blk140_dn7), (var_qlo__blk139_dn8 + var_qhi__blk140_dn8), (var_qlo__blk139_dn9 + var_qhi__blk140_dn9), (var_qlo__blk139_dn10 + var_qhi__blk140_dn10), (var_qlo__blk139_dn11 + var_qhi__blk140_dn11), (var_qlo__blk139_dn12 + var_qhi__blk140_dn12), (var_qlo__blk139_dn13 + var_qhi__blk140_dn13),)
    } else {
        (var_qdbcp, var_qdbcp_dn0, var_qdbcp_dn1, var_qdbcp_dn2, var_qdbcp_dn3, var_qdbcp_dn4, var_qdbcp_dn5, var_qdbcp_dn6, var_qdbcp_dn7, var_qdbcp_dn8, var_qdbcp_dn9, var_qdbcp_dn10, var_qdbcp_dn11, var_qdbcp_dn12, var_qdbcp_dn13,)
    }
};
        var_qdbcp = assign4850_e5142;
        var_qdbcp_dn0 = assign4850_e5142_d_n0;
        var_qdbcp_dn1 = assign4850_e5142_d_n1;
        var_qdbcp_dn2 = assign4850_e5142_d_n2;
        var_qdbcp_dn3 = assign4850_e5142_d_n3;
        var_qdbcp_dn4 = assign4850_e5142_d_n4;
        var_qdbcp_dn5 = assign4850_e5142_d_n5;
        var_qdbcp_dn6 = assign4850_e5142_d_n6;
        var_qdbcp_dn7 = assign4850_e5142_d_n7;
        var_qdbcp_dn8 = assign4850_e5142_d_n8;
        var_qdbcp_dn9 = assign4850_e5142_d_n9;
        var_qdbcp_dn10 = assign4850_e5142_d_n10;
        var_qdbcp_dn11 = assign4850_e5142_d_n11;
        var_qdbcp_dn12 = assign4850_e5142_d_n12;
        var_qdbcp_dn13 = assign4850_e5142_d_n13;

        let (assign4860_e5158, assign4860_e5158_d_n0, assign4860_e5158_d_n1, assign4860_e5158_d_n2, assign4860_e5158_d_n3, assign4860_e5158_d_n4, assign4860_e5158_d_n5, assign4860_e5158_d_n6, assign4860_e5158_d_n7, assign4860_e5158_d_n8, assign4860_e5158_d_n9, assign4860_e5158_d_n10, assign4860_e5158_d_n11, assign4860_e5158_d_n12, assign4860_e5158_d_n13,) = {
    if ((var_guard135 != 0.0) && (var_guard147 == 0.0)) {
        let assign4860_e5149: f64 = (var_dv0__blk136 * var_dv0__blk136);
        let assign4860_e5152: f64 = (4.0 * p.p52);
        let assign4860_e5154: f64 = (assign4860_e5152 * p.p52);
        let assign4860_e5155: f64 = (assign4860_e5149 + assign4860_e5154);
        let assign4860_e5156: f64 = (assign4860_e5155).sqrt();
        (assign4860_e5156, (((var_dv0__blk136_dn0 * var_dv0__blk136) + (var_dv0__blk136 * var_dv0__blk136_dn0)) / (2.0 * assign4860_e5156)), (((var_dv0__blk136_dn1 * var_dv0__blk136) + (var_dv0__blk136 * var_dv0__blk136_dn1)) / (2.0 * assign4860_e5156)), (((var_dv0__blk136_dn2 * var_dv0__blk136) + (var_dv0__blk136 * var_dv0__blk136_dn2)) / (2.0 * assign4860_e5156)), (((var_dv0__blk136_dn3 * var_dv0__blk136) + (var_dv0__blk136 * var_dv0__blk136_dn3)) / (2.0 * assign4860_e5156)), (((var_dv0__blk136_dn4 * var_dv0__blk136) + (var_dv0__blk136 * var_dv0__blk136_dn4)) / (2.0 * assign4860_e5156)), (((var_dv0__blk136_dn5 * var_dv0__blk136) + (var_dv0__blk136 * var_dv0__blk136_dn5)) / (2.0 * assign4860_e5156)), (((var_dv0__blk136_dn6 * var_dv0__blk136) + (var_dv0__blk136 * var_dv0__blk136_dn6)) / (2.0 * assign4860_e5156)), (((var_dv0__blk136_dn7 * var_dv0__blk136) + (var_dv0__blk136 * var_dv0__blk136_dn7)) / (2.0 * assign4860_e5156)), (((var_dv0__blk136_dn8 * var_dv0__blk136) + (var_dv0__blk136 * var_dv0__blk136_dn8)) / (2.0 * assign4860_e5156)), (((var_dv0__blk136_dn9 * var_dv0__blk136) + (var_dv0__blk136 * var_dv0__blk136_dn9)) / (2.0 * assign4860_e5156)), (((var_dv0__blk136_dn10 * var_dv0__blk136) + (var_dv0__blk136 * var_dv0__blk136_dn10)) / (2.0 * assign4860_e5156)), (((var_dv0__blk136_dn11 * var_dv0__blk136) + (var_dv0__blk136 * var_dv0__blk136_dn11)) / (2.0 * assign4860_e5156)), (((var_dv0__blk136_dn12 * var_dv0__blk136) + (var_dv0__blk136 * var_dv0__blk136_dn12)) / (2.0 * assign4860_e5156)), (((var_dv0__blk136_dn13 * var_dv0__blk136) + (var_dv0__blk136 * var_dv0__blk136_dn13)) / (2.0 * assign4860_e5156)),)
    } else {
        (var_mv0__blk141, var_mv0__blk141_dn0, var_mv0__blk141_dn1, var_mv0__blk141_dn2, var_mv0__blk141_dn3, var_mv0__blk141_dn4, var_mv0__blk141_dn5, var_mv0__blk141_dn6, var_mv0__blk141_dn7, var_mv0__blk141_dn8, var_mv0__blk141_dn9, var_mv0__blk141_dn10, var_mv0__blk141_dn11, var_mv0__blk141_dn12, var_mv0__blk141_dn13,)
    }
};
        var_mv0__blk141 = assign4860_e5158;
        var_mv0__blk141_dn0 = assign4860_e5158_d_n0;
        var_mv0__blk141_dn1 = assign4860_e5158_d_n1;
        var_mv0__blk141_dn2 = assign4860_e5158_d_n2;
        var_mv0__blk141_dn3 = assign4860_e5158_d_n3;
        var_mv0__blk141_dn4 = assign4860_e5158_d_n4;
        var_mv0__blk141_dn5 = assign4860_e5158_d_n5;
        var_mv0__blk141_dn6 = assign4860_e5158_d_n6;
        var_mv0__blk141_dn7 = assign4860_e5158_d_n7;
        var_mv0__blk141_dn8 = assign4860_e5158_d_n8;
        var_mv0__blk141_dn9 = assign4860_e5158_d_n9;
        var_mv0__blk141_dn10 = assign4860_e5158_d_n10;
        var_mv0__blk141_dn11 = assign4860_e5158_d_n11;
        var_mv0__blk141_dn12 = assign4860_e5158_d_n12;
        var_mv0__blk141_dn13 = assign4860_e5158_d_n13;

        let (assign4870_e5170, assign4870_e5170_d_n0, assign4870_e5170_d_n1, assign4870_e5170_d_n2, assign4870_e5170_d_n3, assign4870_e5170_d_n4, assign4870_e5170_d_n5, assign4870_e5170_d_n6, assign4870_e5170_d_n7, assign4870_e5170_d_n8, assign4870_e5170_d_n9, assign4870_e5170_d_n10, assign4870_e5170_d_n11, assign4870_e5170_d_n12, assign4870_e5170_d_n13,) = {
    if ((var_guard135 != 0.0) && (var_guard147 == 0.0)) {
        let assign4870_e5164: f64 = (-0.5);
        let assign4870_e5167: f64 = (var_dv0__blk136 + var_mv0__blk141);
        let assign4870_e5168: f64 = (assign4870_e5164 * assign4870_e5167);
        (assign4870_e5168, (assign4870_e5164 * (var_dv0__blk136_dn0 + var_mv0__blk141_dn0)), (assign4870_e5164 * (var_dv0__blk136_dn1 + var_mv0__blk141_dn1)), (assign4870_e5164 * (var_dv0__blk136_dn2 + var_mv0__blk141_dn2)), (assign4870_e5164 * (var_dv0__blk136_dn3 + var_mv0__blk141_dn3)), (assign4870_e5164 * (var_dv0__blk136_dn4 + var_mv0__blk141_dn4)), (assign4870_e5164 * (var_dv0__blk136_dn5 + var_mv0__blk141_dn5)), (assign4870_e5164 * (var_dv0__blk136_dn6 + var_mv0__blk141_dn6)), (assign4870_e5164 * (var_dv0__blk136_dn7 + var_mv0__blk141_dn7)), (assign4870_e5164 * (var_dv0__blk136_dn8 + var_mv0__blk141_dn8)), (assign4870_e5164 * (var_dv0__blk136_dn9 + var_mv0__blk141_dn9)), (assign4870_e5164 * (var_dv0__blk136_dn10 + var_mv0__blk141_dn10)), (assign4870_e5164 * (var_dv0__blk136_dn11 + var_mv0__blk141_dn11)), (assign4870_e5164 * (var_dv0__blk136_dn12 + var_mv0__blk141_dn12)), (assign4870_e5164 * (var_dv0__blk136_dn13 + var_mv0__blk141_dn13)),)
    } else {
        (var_vl0__blk142, var_vl0__blk142_dn0, var_vl0__blk142_dn1, var_vl0__blk142_dn2, var_vl0__blk142_dn3, var_vl0__blk142_dn4, var_vl0__blk142_dn5, var_vl0__blk142_dn6, var_vl0__blk142_dn7, var_vl0__blk142_dn8, var_vl0__blk142_dn9, var_vl0__blk142_dn10, var_vl0__blk142_dn11, var_vl0__blk142_dn12, var_vl0__blk142_dn13,)
    }
};
        var_vl0__blk142 = assign4870_e5170;
        var_vl0__blk142_dn0 = assign4870_e5170_d_n0;
        var_vl0__blk142_dn1 = assign4870_e5170_d_n1;
        var_vl0__blk142_dn2 = assign4870_e5170_d_n2;
        var_vl0__blk142_dn3 = assign4870_e5170_d_n3;
        var_vl0__blk142_dn4 = assign4870_e5170_d_n4;
        var_vl0__blk142_dn5 = assign4870_e5170_d_n5;
        var_vl0__blk142_dn6 = assign4870_e5170_d_n6;
        var_vl0__blk142_dn7 = assign4870_e5170_d_n7;
        var_vl0__blk142_dn8 = assign4870_e5170_d_n8;
        var_vl0__blk142_dn9 = assign4870_e5170_d_n9;
        var_vl0__blk142_dn10 = assign4870_e5170_d_n10;
        var_vl0__blk142_dn11 = assign4870_e5170_d_n11;
        var_vl0__blk142_dn12 = assign4870_e5170_d_n12;
        var_vl0__blk142_dn13 = assign4870_e5170_d_n13;

        let (assign4880_e5192, assign4880_e5192_d_n0, assign4880_e5192_d_n1, assign4880_e5192_d_n2, assign4880_e5192_d_n3, assign4880_e5192_d_n4, assign4880_e5192_d_n5, assign4880_e5192_d_n6, assign4880_e5192_d_n7, assign4880_e5192_d_n8, assign4880_e5192_d_n9, assign4880_e5192_d_n10, assign4880_e5192_d_n11, assign4880_e5192_d_n12, assign4880_e5192_d_n13,) = {
    if ((var_guard135 != 0.0) && (var_guard147 == 0.0)) {
        let assign4880_e5176: f64 = (-var_ps_t);
        let assign4880_e5180: f64 = (var_vl0__blk142 / var_ps_t);
        let assign4880_e5181: f64 = (1.0 - assign4880_e5180);
        let assign4880_e5184: f64 = (1.0 - p.p51);
        let assign4880_e5185: f64 = (assign4880_e5181).powf(assign4880_e5184);
        let assign4880_e5186: f64 = (assign4880_e5176 * assign4880_e5185);
        let assign4880_e5189: f64 = (1.0 - p.p51);
        let assign4880_e5190: f64 = (assign4880_e5186 / assign4880_e5189);
        (assign4880_e5190, ((((-var_ps_t_dn0) * assign4880_e5185) + (assign4880_e5176 * if 0.0 == 0.0 && ((assign4880_e5184) as f64).is_finite() && ((assign4880_e5184) as f64).fract() == 0.0 { if assign4880_e5184 == 0.0 { 0.0 } else { (assign4880_e5184 * ((assign4880_e5181).powf(assign4880_e5184 - 1.0) * (-(((var_vl0__blk142_dn0 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn0)) / (var_ps_t * var_ps_t))))) } } else { (assign4880_e5185 * (assign4880_e5184 * ((-(((var_vl0__blk142_dn0 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn0)) / (var_ps_t * var_ps_t))) / assign4880_e5181))) })) / assign4880_e5189), ((((-var_ps_t_dn1) * assign4880_e5185) + (assign4880_e5176 * if 0.0 == 0.0 && ((assign4880_e5184) as f64).is_finite() && ((assign4880_e5184) as f64).fract() == 0.0 { if assign4880_e5184 == 0.0 { 0.0 } else { (assign4880_e5184 * ((assign4880_e5181).powf(assign4880_e5184 - 1.0) * (-(((var_vl0__blk142_dn1 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn1)) / (var_ps_t * var_ps_t))))) } } else { (assign4880_e5185 * (assign4880_e5184 * ((-(((var_vl0__blk142_dn1 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn1)) / (var_ps_t * var_ps_t))) / assign4880_e5181))) })) / assign4880_e5189), ((((-var_ps_t_dn2) * assign4880_e5185) + (assign4880_e5176 * if 0.0 == 0.0 && ((assign4880_e5184) as f64).is_finite() && ((assign4880_e5184) as f64).fract() == 0.0 { if assign4880_e5184 == 0.0 { 0.0 } else { (assign4880_e5184 * ((assign4880_e5181).powf(assign4880_e5184 - 1.0) * (-(((var_vl0__blk142_dn2 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn2)) / (var_ps_t * var_ps_t))))) } } else { (assign4880_e5185 * (assign4880_e5184 * ((-(((var_vl0__blk142_dn2 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn2)) / (var_ps_t * var_ps_t))) / assign4880_e5181))) })) / assign4880_e5189), ((((-var_ps_t_dn3) * assign4880_e5185) + (assign4880_e5176 * if 0.0 == 0.0 && ((assign4880_e5184) as f64).is_finite() && ((assign4880_e5184) as f64).fract() == 0.0 { if assign4880_e5184 == 0.0 { 0.0 } else { (assign4880_e5184 * ((assign4880_e5181).powf(assign4880_e5184 - 1.0) * (-(((var_vl0__blk142_dn3 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn3)) / (var_ps_t * var_ps_t))))) } } else { (assign4880_e5185 * (assign4880_e5184 * ((-(((var_vl0__blk142_dn3 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn3)) / (var_ps_t * var_ps_t))) / assign4880_e5181))) })) / assign4880_e5189), ((((-var_ps_t_dn4) * assign4880_e5185) + (assign4880_e5176 * if 0.0 == 0.0 && ((assign4880_e5184) as f64).is_finite() && ((assign4880_e5184) as f64).fract() == 0.0 { if assign4880_e5184 == 0.0 { 0.0 } else { (assign4880_e5184 * ((assign4880_e5181).powf(assign4880_e5184 - 1.0) * (-(((var_vl0__blk142_dn4 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn4)) / (var_ps_t * var_ps_t))))) } } else { (assign4880_e5185 * (assign4880_e5184 * ((-(((var_vl0__blk142_dn4 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn4)) / (var_ps_t * var_ps_t))) / assign4880_e5181))) })) / assign4880_e5189), ((((-var_ps_t_dn5) * assign4880_e5185) + (assign4880_e5176 * if 0.0 == 0.0 && ((assign4880_e5184) as f64).is_finite() && ((assign4880_e5184) as f64).fract() == 0.0 { if assign4880_e5184 == 0.0 { 0.0 } else { (assign4880_e5184 * ((assign4880_e5181).powf(assign4880_e5184 - 1.0) * (-(((var_vl0__blk142_dn5 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn5)) / (var_ps_t * var_ps_t))))) } } else { (assign4880_e5185 * (assign4880_e5184 * ((-(((var_vl0__blk142_dn5 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn5)) / (var_ps_t * var_ps_t))) / assign4880_e5181))) })) / assign4880_e5189), ((((-var_ps_t_dn6) * assign4880_e5185) + (assign4880_e5176 * if 0.0 == 0.0 && ((assign4880_e5184) as f64).is_finite() && ((assign4880_e5184) as f64).fract() == 0.0 { if assign4880_e5184 == 0.0 { 0.0 } else { (assign4880_e5184 * ((assign4880_e5181).powf(assign4880_e5184 - 1.0) * (-(((var_vl0__blk142_dn6 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn6)) / (var_ps_t * var_ps_t))))) } } else { (assign4880_e5185 * (assign4880_e5184 * ((-(((var_vl0__blk142_dn6 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn6)) / (var_ps_t * var_ps_t))) / assign4880_e5181))) })) / assign4880_e5189), ((((-var_ps_t_dn7) * assign4880_e5185) + (assign4880_e5176 * if 0.0 == 0.0 && ((assign4880_e5184) as f64).is_finite() && ((assign4880_e5184) as f64).fract() == 0.0 { if assign4880_e5184 == 0.0 { 0.0 } else { (assign4880_e5184 * ((assign4880_e5181).powf(assign4880_e5184 - 1.0) * (-(((var_vl0__blk142_dn7 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn7)) / (var_ps_t * var_ps_t))))) } } else { (assign4880_e5185 * (assign4880_e5184 * ((-(((var_vl0__blk142_dn7 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn7)) / (var_ps_t * var_ps_t))) / assign4880_e5181))) })) / assign4880_e5189), ((((-var_ps_t_dn8) * assign4880_e5185) + (assign4880_e5176 * if 0.0 == 0.0 && ((assign4880_e5184) as f64).is_finite() && ((assign4880_e5184) as f64).fract() == 0.0 { if assign4880_e5184 == 0.0 { 0.0 } else { (assign4880_e5184 * ((assign4880_e5181).powf(assign4880_e5184 - 1.0) * (-(((var_vl0__blk142_dn8 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn8)) / (var_ps_t * var_ps_t))))) } } else { (assign4880_e5185 * (assign4880_e5184 * ((-(((var_vl0__blk142_dn8 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn8)) / (var_ps_t * var_ps_t))) / assign4880_e5181))) })) / assign4880_e5189), ((((-var_ps_t_dn9) * assign4880_e5185) + (assign4880_e5176 * if 0.0 == 0.0 && ((assign4880_e5184) as f64).is_finite() && ((assign4880_e5184) as f64).fract() == 0.0 { if assign4880_e5184 == 0.0 { 0.0 } else { (assign4880_e5184 * ((assign4880_e5181).powf(assign4880_e5184 - 1.0) * (-(((var_vl0__blk142_dn9 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn9)) / (var_ps_t * var_ps_t))))) } } else { (assign4880_e5185 * (assign4880_e5184 * ((-(((var_vl0__blk142_dn9 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn9)) / (var_ps_t * var_ps_t))) / assign4880_e5181))) })) / assign4880_e5189), ((((-var_ps_t_dn10) * assign4880_e5185) + (assign4880_e5176 * if 0.0 == 0.0 && ((assign4880_e5184) as f64).is_finite() && ((assign4880_e5184) as f64).fract() == 0.0 { if assign4880_e5184 == 0.0 { 0.0 } else { (assign4880_e5184 * ((assign4880_e5181).powf(assign4880_e5184 - 1.0) * (-(((var_vl0__blk142_dn10 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn10)) / (var_ps_t * var_ps_t))))) } } else { (assign4880_e5185 * (assign4880_e5184 * ((-(((var_vl0__blk142_dn10 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn10)) / (var_ps_t * var_ps_t))) / assign4880_e5181))) })) / assign4880_e5189), ((((-var_ps_t_dn11) * assign4880_e5185) + (assign4880_e5176 * if 0.0 == 0.0 && ((assign4880_e5184) as f64).is_finite() && ((assign4880_e5184) as f64).fract() == 0.0 { if assign4880_e5184 == 0.0 { 0.0 } else { (assign4880_e5184 * ((assign4880_e5181).powf(assign4880_e5184 - 1.0) * (-(((var_vl0__blk142_dn11 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn11)) / (var_ps_t * var_ps_t))))) } } else { (assign4880_e5185 * (assign4880_e5184 * ((-(((var_vl0__blk142_dn11 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn11)) / (var_ps_t * var_ps_t))) / assign4880_e5181))) })) / assign4880_e5189), ((((-var_ps_t_dn12) * assign4880_e5185) + (assign4880_e5176 * if 0.0 == 0.0 && ((assign4880_e5184) as f64).is_finite() && ((assign4880_e5184) as f64).fract() == 0.0 { if assign4880_e5184 == 0.0 { 0.0 } else { (assign4880_e5184 * ((assign4880_e5181).powf(assign4880_e5184 - 1.0) * (-(((var_vl0__blk142_dn12 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn12)) / (var_ps_t * var_ps_t))))) } } else { (assign4880_e5185 * (assign4880_e5184 * ((-(((var_vl0__blk142_dn12 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn12)) / (var_ps_t * var_ps_t))) / assign4880_e5181))) })) / assign4880_e5189), ((((-var_ps_t_dn13) * assign4880_e5185) + (assign4880_e5176 * if 0.0 == 0.0 && ((assign4880_e5184) as f64).is_finite() && ((assign4880_e5184) as f64).fract() == 0.0 { if assign4880_e5184 == 0.0 { 0.0 } else { (assign4880_e5184 * ((assign4880_e5181).powf(assign4880_e5184 - 1.0) * (-(((var_vl0__blk142_dn13 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn13)) / (var_ps_t * var_ps_t))))) } } else { (assign4880_e5185 * (assign4880_e5184 * ((-(((var_vl0__blk142_dn13 * var_ps_t) - (var_vl0__blk142 * var_ps_t_dn13)) / (var_ps_t * var_ps_t))) / assign4880_e5181))) })) / assign4880_e5189),)
    } else {
        (var_q0__blk143, var_q0__blk143_dn0, var_q0__blk143_dn1, var_q0__blk143_dn2, var_q0__blk143_dn3, var_q0__blk143_dn4, var_q0__blk143_dn5, var_q0__blk143_dn6, var_q0__blk143_dn7, var_q0__blk143_dn8, var_q0__blk143_dn9, var_q0__blk143_dn10, var_q0__blk143_dn11, var_q0__blk143_dn12, var_q0__blk143_dn13,)
    }
};
        var_q0__blk143 = assign4880_e5192;
        var_q0__blk143_dn0 = assign4880_e5192_d_n0;
        var_q0__blk143_dn1 = assign4880_e5192_d_n1;
        var_q0__blk143_dn2 = assign4880_e5192_d_n2;
        var_q0__blk143_dn3 = assign4880_e5192_d_n3;
        var_q0__blk143_dn4 = assign4880_e5192_d_n4;
        var_q0__blk143_dn5 = assign4880_e5192_d_n5;
        var_q0__blk143_dn6 = assign4880_e5192_d_n6;
        var_q0__blk143_dn7 = assign4880_e5192_d_n7;
        var_q0__blk143_dn8 = assign4880_e5192_d_n8;
        var_q0__blk143_dn9 = assign4880_e5192_d_n9;
        var_q0__blk143_dn10 = assign4880_e5192_d_n10;
        var_q0__blk143_dn11 = assign4880_e5192_d_n11;
        var_q0__blk143_dn12 = assign4880_e5192_d_n12;
        var_q0__blk143_dn13 = assign4880_e5192_d_n13;

        let (assign4890_e5201, assign4890_e5201_d_n0, assign4890_e5201_d_n1, assign4890_e5201_d_n2, assign4890_e5201_d_n3, assign4890_e5201_d_n4, assign4890_e5201_d_n5, assign4890_e5201_d_n6, assign4890_e5201_d_n7, assign4890_e5201_d_n8, assign4890_e5201_d_n9, assign4890_e5201_d_n10, assign4890_e5201_d_n11, assign4890_e5201_d_n12, assign4890_e5201_d_n13,) = {
    if ((var_guard135 != 0.0) && (var_guard147 == 0.0)) {
        let assign4890_e5199: f64 = (var_vbcp + var_dv0__blk136);
        (assign4890_e5199, (var_vbcp_dn0 + var_dv0__blk136_dn0), (var_vbcp_dn1 + var_dv0__blk136_dn1), (var_vbcp_dn2 + var_dv0__blk136_dn2), (var_vbcp_dn3 + var_dv0__blk136_dn3), (var_vbcp_dn4 + var_dv0__blk136_dn4), (var_vbcp_dn5 + var_dv0__blk136_dn5), (var_vbcp_dn6 + var_dv0__blk136_dn6), (var_vbcp_dn7 + var_dv0__blk136_dn7), (var_vbcp_dn8 + var_dv0__blk136_dn8), (var_vbcp_dn9 + var_dv0__blk136_dn9), (var_vbcp_dn10 + var_dv0__blk136_dn10), (var_vbcp_dn11 + var_dv0__blk136_dn11), (var_vbcp_dn12 + var_dv0__blk136_dn12), (var_vbcp_dn13 + var_dv0__blk136_dn13),)
    } else {
        (var_dv__blk144, var_dv__blk144_dn0, var_dv__blk144_dn1, var_dv__blk144_dn2, var_dv__blk144_dn3, var_dv__blk144_dn4, var_dv__blk144_dn5, var_dv__blk144_dn6, var_dv__blk144_dn7, var_dv__blk144_dn8, var_dv__blk144_dn9, var_dv__blk144_dn10, var_dv__blk144_dn11, var_dv__blk144_dn12, var_dv__blk144_dn13,)
    }
};
        var_dv__blk144 = assign4890_e5201;
        var_dv__blk144_dn0 = assign4890_e5201_d_n0;
        var_dv__blk144_dn1 = assign4890_e5201_d_n1;
        var_dv__blk144_dn2 = assign4890_e5201_d_n2;
        var_dv__blk144_dn3 = assign4890_e5201_d_n3;
        var_dv__blk144_dn4 = assign4890_e5201_d_n4;
        var_dv__blk144_dn5 = assign4890_e5201_d_n5;
        var_dv__blk144_dn6 = assign4890_e5201_d_n6;
        var_dv__blk144_dn7 = assign4890_e5201_d_n7;
        var_dv__blk144_dn8 = assign4890_e5201_d_n8;
        var_dv__blk144_dn9 = assign4890_e5201_d_n9;
        var_dv__blk144_dn10 = assign4890_e5201_d_n10;
        var_dv__blk144_dn11 = assign4890_e5201_d_n11;
        var_dv__blk144_dn12 = assign4890_e5201_d_n12;
        var_dv__blk144_dn13 = assign4890_e5201_d_n13;


        *var_dv0__blk136_slot = var_dv0__blk136;
        *var_dv0__blk136_dn0_slot = var_dv0__blk136_dn0;
        *var_dv0__blk136_dn1_slot = var_dv0__blk136_dn1;
        *var_dv0__blk136_dn10_slot = var_dv0__blk136_dn10;
        *var_dv0__blk136_dn11_slot = var_dv0__blk136_dn11;
        *var_dv0__blk136_dn12_slot = var_dv0__blk136_dn12;
        *var_dv0__blk136_dn13_slot = var_dv0__blk136_dn13;
        *var_dv0__blk136_dn2_slot = var_dv0__blk136_dn2;
        *var_dv0__blk136_dn3_slot = var_dv0__blk136_dn3;
        *var_dv0__blk136_dn4_slot = var_dv0__blk136_dn4;
        *var_dv0__blk136_dn5_slot = var_dv0__blk136_dn5;
        *var_dv0__blk136_dn6_slot = var_dv0__blk136_dn6;
        *var_dv0__blk136_dn7_slot = var_dv0__blk136_dn7;
        *var_dv0__blk136_dn8_slot = var_dv0__blk136_dn8;
        *var_dv0__blk136_dn9_slot = var_dv0__blk136_dn9;
        *var_dv__blk144_slot = var_dv__blk144;
        *var_dv__blk144_dn0_slot = var_dv__blk144_dn0;
        *var_dv__blk144_dn1_slot = var_dv__blk144_dn1;
        *var_dv__blk144_dn10_slot = var_dv__blk144_dn10;
        *var_dv__blk144_dn11_slot = var_dv__blk144_dn11;
        *var_dv__blk144_dn12_slot = var_dv__blk144_dn12;
        *var_dv__blk144_dn13_slot = var_dv__blk144_dn13;
        *var_dv__blk144_dn2_slot = var_dv__blk144_dn2;
        *var_dv__blk144_dn3_slot = var_dv__blk144_dn3;
        *var_dv__blk144_dn4_slot = var_dv__blk144_dn4;
        *var_dv__blk144_dn5_slot = var_dv__blk144_dn5;
        *var_dv__blk144_dn6_slot = var_dv__blk144_dn6;
        *var_dv__blk144_dn7_slot = var_dv__blk144_dn7;
        *var_dv__blk144_dn8_slot = var_dv__blk144_dn8;
        *var_dv__blk144_dn9_slot = var_dv__blk144_dn9;
        *var_dvh__blk137_slot = var_dvh__blk137;
        *var_dvh__blk137_dn0_slot = var_dvh__blk137_dn0;
        *var_dvh__blk137_dn1_slot = var_dvh__blk137_dn1;
        *var_dvh__blk137_dn10_slot = var_dvh__blk137_dn10;
        *var_dvh__blk137_dn11_slot = var_dvh__blk137_dn11;
        *var_dvh__blk137_dn12_slot = var_dvh__blk137_dn12;
        *var_dvh__blk137_dn13_slot = var_dvh__blk137_dn13;
        *var_dvh__blk137_dn2_slot = var_dvh__blk137_dn2;
        *var_dvh__blk137_dn3_slot = var_dvh__blk137_dn3;
        *var_dvh__blk137_dn4_slot = var_dvh__blk137_dn4;
        *var_dvh__blk137_dn5_slot = var_dvh__blk137_dn5;
        *var_dvh__blk137_dn6_slot = var_dvh__blk137_dn6;
        *var_dvh__blk137_dn7_slot = var_dvh__blk137_dn7;
        *var_dvh__blk137_dn8_slot = var_dvh__blk137_dn8;
        *var_dvh__blk137_dn9_slot = var_dvh__blk137_dn9;
        *var_guard135_slot = var_guard135;
        *var_guard147_slot = var_guard147;
        *var_guard148_slot = var_guard148;
        *var_ibcp_slot = var_ibcp;
        *var_ibcp_dn0_slot = var_ibcp_dn0;
        *var_ibcp_dn1_slot = var_ibcp_dn1;
        *var_ibcp_dn10_slot = var_ibcp_dn10;
        *var_ibcp_dn11_slot = var_ibcp_dn11;
        *var_ibcp_dn12_slot = var_ibcp_dn12;
        *var_ibcp_dn13_slot = var_ibcp_dn13;
        *var_ibcp_dn2_slot = var_ibcp_dn2;
        *var_ibcp_dn3_slot = var_ibcp_dn3;
        *var_ibcp_dn4_slot = var_ibcp_dn4;
        *var_ibcp_dn5_slot = var_ibcp_dn5;
        *var_ibcp_dn6_slot = var_ibcp_dn6;
        *var_ibcp_dn7_slot = var_ibcp_dn7;
        *var_ibcp_dn8_slot = var_ibcp_dn8;
        *var_ibcp_dn9_slot = var_ibcp_dn9;
        *var_iccp_slot = var_iccp;
        *var_iccp_dn0_slot = var_iccp_dn0;
        *var_iccp_dn1_slot = var_iccp_dn1;
        *var_iccp_dn10_slot = var_iccp_dn10;
        *var_iccp_dn11_slot = var_iccp_dn11;
        *var_iccp_dn12_slot = var_iccp_dn12;
        *var_iccp_dn13_slot = var_iccp_dn13;
        *var_iccp_dn2_slot = var_iccp_dn2;
        *var_iccp_dn3_slot = var_iccp_dn3;
        *var_iccp_dn4_slot = var_iccp_dn4;
        *var_iccp_dn5_slot = var_iccp_dn5;
        *var_iccp_dn6_slot = var_iccp_dn6;
        *var_iccp_dn7_slot = var_iccp_dn7;
        *var_iccp_dn8_slot = var_iccp_dn8;
        *var_iccp_dn9_slot = var_iccp_dn9;
        *var_irbi_slot = var_irbi;
        *var_irbi_dn0_slot = var_irbi_dn0;
        *var_irbi_dn1_slot = var_irbi_dn1;
        *var_irbi_dn10_slot = var_irbi_dn10;
        *var_irbi_dn11_slot = var_irbi_dn11;
        *var_irbi_dn12_slot = var_irbi_dn12;
        *var_irbi_dn13_slot = var_irbi_dn13;
        *var_irbi_dn2_slot = var_irbi_dn2;
        *var_irbi_dn3_slot = var_irbi_dn3;
        *var_irbi_dn4_slot = var_irbi_dn4;
        *var_irbi_dn5_slot = var_irbi_dn5;
        *var_irbi_dn6_slot = var_irbi_dn6;
        *var_irbi_dn7_slot = var_irbi_dn7;
        *var_irbi_dn8_slot = var_irbi_dn8;
        *var_irbi_dn9_slot = var_irbi_dn9;
        *var_irbp_slot = var_irbp;
        *var_irbp_dn0_slot = var_irbp_dn0;
        *var_irbp_dn1_slot = var_irbp_dn1;
        *var_irbp_dn10_slot = var_irbp_dn10;
        *var_irbp_dn11_slot = var_irbp_dn11;
        *var_irbp_dn12_slot = var_irbp_dn12;
        *var_irbp_dn13_slot = var_irbp_dn13;
        *var_irbp_dn2_slot = var_irbp_dn2;
        *var_irbp_dn3_slot = var_irbp_dn3;
        *var_irbp_dn4_slot = var_irbp_dn4;
        *var_irbp_dn5_slot = var_irbp_dn5;
        *var_irbp_dn6_slot = var_irbp_dn6;
        *var_irbp_dn7_slot = var_irbp_dn7;
        *var_irbp_dn8_slot = var_irbp_dn8;
        *var_irbp_dn9_slot = var_irbp_dn9;
        *var_irbx_slot = var_irbx;
        *var_irbx_dn0_slot = var_irbx_dn0;
        *var_irbx_dn1_slot = var_irbx_dn1;
        *var_irbx_dn10_slot = var_irbx_dn10;
        *var_irbx_dn11_slot = var_irbx_dn11;
        *var_irbx_dn12_slot = var_irbx_dn12;
        *var_irbx_dn13_slot = var_irbx_dn13;
        *var_irbx_dn2_slot = var_irbx_dn2;
        *var_irbx_dn3_slot = var_irbx_dn3;
        *var_irbx_dn4_slot = var_irbx_dn4;
        *var_irbx_dn5_slot = var_irbx_dn5;
        *var_irbx_dn6_slot = var_irbx_dn6;
        *var_irbx_dn7_slot = var_irbx_dn7;
        *var_irbx_dn8_slot = var_irbx_dn8;
        *var_irbx_dn9_slot = var_irbx_dn9;
        *var_irci_slot = var_irci;
        *var_irci_dn0_slot = var_irci_dn0;
        *var_irci_dn1_slot = var_irci_dn1;
        *var_irci_dn10_slot = var_irci_dn10;
        *var_irci_dn11_slot = var_irci_dn11;
        *var_irci_dn12_slot = var_irci_dn12;
        *var_irci_dn13_slot = var_irci_dn13;
        *var_irci_dn2_slot = var_irci_dn2;
        *var_irci_dn3_slot = var_irci_dn3;
        *var_irci_dn4_slot = var_irci_dn4;
        *var_irci_dn5_slot = var_irci_dn5;
        *var_irci_dn6_slot = var_irci_dn6;
        *var_irci_dn7_slot = var_irci_dn7;
        *var_irci_dn8_slot = var_irci_dn8;
        *var_irci_dn9_slot = var_irci_dn9;
        *var_ire_slot = var_ire;
        *var_ire_dn0_slot = var_ire_dn0;
        *var_ire_dn1_slot = var_ire_dn1;
        *var_ire_dn10_slot = var_ire_dn10;
        *var_ire_dn11_slot = var_ire_dn11;
        *var_ire_dn12_slot = var_ire_dn12;
        *var_ire_dn13_slot = var_ire_dn13;
        *var_ire_dn2_slot = var_ire_dn2;
        *var_ire_dn3_slot = var_ire_dn3;
        *var_ire_dn4_slot = var_ire_dn4;
        *var_ire_dn5_slot = var_ire_dn5;
        *var_ire_dn6_slot = var_ire_dn6;
        *var_ire_dn7_slot = var_ire_dn7;
        *var_ire_dn8_slot = var_ire_dn8;
        *var_ire_dn9_slot = var_ire_dn9;
        *var_irs_slot = var_irs;
        *var_irs_dn0_slot = var_irs_dn0;
        *var_irs_dn1_slot = var_irs_dn1;
        *var_irs_dn10_slot = var_irs_dn10;
        *var_irs_dn11_slot = var_irs_dn11;
        *var_irs_dn12_slot = var_irs_dn12;
        *var_irs_dn13_slot = var_irs_dn13;
        *var_irs_dn2_slot = var_irs_dn2;
        *var_irs_dn3_slot = var_irs_dn3;
        *var_irs_dn4_slot = var_irs_dn4;
        *var_irs_dn5_slot = var_irs_dn5;
        *var_irs_dn6_slot = var_irs_dn6;
        *var_irs_dn7_slot = var_irs_dn7;
        *var_irs_dn8_slot = var_irs_dn8;
        *var_irs_dn9_slot = var_irs_dn9;
        *var_irth_slot = var_irth;
        *var_irth_dn0_slot = var_irth_dn0;
        *var_irth_dn1_slot = var_irth_dn1;
        *var_irth_dn10_slot = var_irth_dn10;
        *var_irth_dn11_slot = var_irth_dn11;
        *var_irth_dn12_slot = var_irth_dn12;
        *var_irth_dn13_slot = var_irth_dn13;
        *var_irth_dn2_slot = var_irth_dn2;
        *var_irth_dn3_slot = var_irth_dn3;
        *var_irth_dn4_slot = var_irth_dn4;
        *var_irth_dn5_slot = var_irth_dn5;
        *var_irth_dn6_slot = var_irth_dn6;
        *var_irth_dn7_slot = var_irth_dn7;
        *var_irth_dn8_slot = var_irth_dn8;
        *var_irth_dn9_slot = var_irth_dn9;
        *var_ith_slot = var_ith;
        *var_ith_dn0_slot = var_ith_dn0;
        *var_ith_dn1_slot = var_ith_dn1;
        *var_ith_dn10_slot = var_ith_dn10;
        *var_ith_dn11_slot = var_ith_dn11;
        *var_ith_dn12_slot = var_ith_dn12;
        *var_ith_dn13_slot = var_ith_dn13;
        *var_ith_dn2_slot = var_ith_dn2;
        *var_ith_dn3_slot = var_ith_dn3;
        *var_ith_dn4_slot = var_ith_dn4;
        *var_ith_dn5_slot = var_ith_dn5;
        *var_ith_dn6_slot = var_ith_dn6;
        *var_ith_dn7_slot = var_ith_dn7;
        *var_ith_dn8_slot = var_ith_dn8;
        *var_ith_dn9_slot = var_ith_dn9;
        *var_mv0__blk141_slot = var_mv0__blk141;
        *var_mv0__blk141_dn0_slot = var_mv0__blk141_dn0;
        *var_mv0__blk141_dn1_slot = var_mv0__blk141_dn1;
        *var_mv0__blk141_dn10_slot = var_mv0__blk141_dn10;
        *var_mv0__blk141_dn11_slot = var_mv0__blk141_dn11;
        *var_mv0__blk141_dn12_slot = var_mv0__blk141_dn12;
        *var_mv0__blk141_dn13_slot = var_mv0__blk141_dn13;
        *var_mv0__blk141_dn2_slot = var_mv0__blk141_dn2;
        *var_mv0__blk141_dn3_slot = var_mv0__blk141_dn3;
        *var_mv0__blk141_dn4_slot = var_mv0__blk141_dn4;
        *var_mv0__blk141_dn5_slot = var_mv0__blk141_dn5;
        *var_mv0__blk141_dn6_slot = var_mv0__blk141_dn6;
        *var_mv0__blk141_dn7_slot = var_mv0__blk141_dn7;
        *var_mv0__blk141_dn8_slot = var_mv0__blk141_dn8;
        *var_mv0__blk141_dn9_slot = var_mv0__blk141_dn9;
        *var_pwq__blk138_slot = var_pwq__blk138;
        *var_q0__blk143_slot = var_q0__blk143;
        *var_q0__blk143_dn0_slot = var_q0__blk143_dn0;
        *var_q0__blk143_dn1_slot = var_q0__blk143_dn1;
        *var_q0__blk143_dn10_slot = var_q0__blk143_dn10;
        *var_q0__blk143_dn11_slot = var_q0__blk143_dn11;
        *var_q0__blk143_dn12_slot = var_q0__blk143_dn12;
        *var_q0__blk143_dn13_slot = var_q0__blk143_dn13;
        *var_q0__blk143_dn2_slot = var_q0__blk143_dn2;
        *var_q0__blk143_dn3_slot = var_q0__blk143_dn3;
        *var_q0__blk143_dn4_slot = var_q0__blk143_dn4;
        *var_q0__blk143_dn5_slot = var_q0__blk143_dn5;
        *var_q0__blk143_dn6_slot = var_q0__blk143_dn6;
        *var_q0__blk143_dn7_slot = var_q0__blk143_dn7;
        *var_q0__blk143_dn8_slot = var_q0__blk143_dn8;
        *var_q0__blk143_dn9_slot = var_q0__blk143_dn9;
        *var_qdbcp_slot = var_qdbcp;
        *var_qdbcp_dn0_slot = var_qdbcp_dn0;
        *var_qdbcp_dn1_slot = var_qdbcp_dn1;
        *var_qdbcp_dn10_slot = var_qdbcp_dn10;
        *var_qdbcp_dn11_slot = var_qdbcp_dn11;
        *var_qdbcp_dn12_slot = var_qdbcp_dn12;
        *var_qdbcp_dn13_slot = var_qdbcp_dn13;
        *var_qdbcp_dn2_slot = var_qdbcp_dn2;
        *var_qdbcp_dn3_slot = var_qdbcp_dn3;
        *var_qdbcp_dn4_slot = var_qdbcp_dn4;
        *var_qdbcp_dn5_slot = var_qdbcp_dn5;
        *var_qdbcp_dn6_slot = var_qdbcp_dn6;
        *var_qdbcp_dn7_slot = var_qdbcp_dn7;
        *var_qdbcp_dn8_slot = var_qdbcp_dn8;
        *var_qdbcp_dn9_slot = var_qdbcp_dn9;
        *var_qhi__blk140_slot = var_qhi__blk140;
        *var_qhi__blk140_dn0_slot = var_qhi__blk140_dn0;
        *var_qhi__blk140_dn1_slot = var_qhi__blk140_dn1;
        *var_qhi__blk140_dn10_slot = var_qhi__blk140_dn10;
        *var_qhi__blk140_dn11_slot = var_qhi__blk140_dn11;
        *var_qhi__blk140_dn12_slot = var_qhi__blk140_dn12;
        *var_qhi__blk140_dn13_slot = var_qhi__blk140_dn13;
        *var_qhi__blk140_dn2_slot = var_qhi__blk140_dn2;
        *var_qhi__blk140_dn3_slot = var_qhi__blk140_dn3;
        *var_qhi__blk140_dn4_slot = var_qhi__blk140_dn4;
        *var_qhi__blk140_dn5_slot = var_qhi__blk140_dn5;
        *var_qhi__blk140_dn6_slot = var_qhi__blk140_dn6;
        *var_qhi__blk140_dn7_slot = var_qhi__blk140_dn7;
        *var_qhi__blk140_dn8_slot = var_qhi__blk140_dn8;
        *var_qhi__blk140_dn9_slot = var_qhi__blk140_dn9;
        *var_qlo__blk139_slot = var_qlo__blk139;
        *var_qlo__blk139_dn0_slot = var_qlo__blk139_dn0;
        *var_qlo__blk139_dn1_slot = var_qlo__blk139_dn1;
        *var_qlo__blk139_dn10_slot = var_qlo__blk139_dn10;
        *var_qlo__blk139_dn11_slot = var_qlo__blk139_dn11;
        *var_qlo__blk139_dn12_slot = var_qlo__blk139_dn12;
        *var_qlo__blk139_dn13_slot = var_qlo__blk139_dn13;
        *var_qlo__blk139_dn2_slot = var_qlo__blk139_dn2;
        *var_qlo__blk139_dn3_slot = var_qlo__blk139_dn3;
        *var_qlo__blk139_dn4_slot = var_qlo__blk139_dn4;
        *var_qlo__blk139_dn5_slot = var_qlo__blk139_dn5;
        *var_qlo__blk139_dn6_slot = var_qlo__blk139_dn6;
        *var_qlo__blk139_dn7_slot = var_qlo__blk139_dn7;
        *var_qlo__blk139_dn8_slot = var_qlo__blk139_dn8;
        *var_qlo__blk139_dn9_slot = var_qlo__blk139_dn9;
        *var_vl0__blk142_slot = var_vl0__blk142;
        *var_vl0__blk142_dn0_slot = var_vl0__blk142_dn0;
        *var_vl0__blk142_dn1_slot = var_vl0__blk142_dn1;
        *var_vl0__blk142_dn10_slot = var_vl0__blk142_dn10;
        *var_vl0__blk142_dn11_slot = var_vl0__blk142_dn11;
        *var_vl0__blk142_dn12_slot = var_vl0__blk142_dn12;
        *var_vl0__blk142_dn13_slot = var_vl0__blk142_dn13;
        *var_vl0__blk142_dn2_slot = var_vl0__blk142_dn2;
        *var_vl0__blk142_dn3_slot = var_vl0__blk142_dn3;
        *var_vl0__blk142_dn4_slot = var_vl0__blk142_dn4;
        *var_vl0__blk142_dn5_slot = var_vl0__blk142_dn5;
        *var_vl0__blk142_dn6_slot = var_vl0__blk142_dn6;
        *var_vl0__blk142_dn7_slot = var_vl0__blk142_dn7;
        *var_vl0__blk142_dn8_slot = var_vl0__blk142_dn8;
        *var_vl0__blk142_dn9_slot = var_vl0__blk142_dn9;
    }

    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        var_dv0__blk136: f64,
        var_dv0__blk136_dn0: f64,
        var_dv0__blk136_dn1: f64,
        var_dv0__blk136_dn10: f64,
        var_dv0__blk136_dn11: f64,
        var_dv0__blk136_dn12: f64,
        var_dv0__blk136_dn13: f64,
        var_dv0__blk136_dn2: f64,
        var_dv0__blk136_dn3: f64,
        var_dv0__blk136_dn4: f64,
        var_dv0__blk136_dn5: f64,
        var_dv0__blk136_dn6: f64,
        var_dv0__blk136_dn7: f64,
        var_dv0__blk136_dn8: f64,
        var_dv0__blk136_dn9: f64,
        var_dv__blk144: f64,
        var_dv__blk144_dn0: f64,
        var_dv__blk144_dn1: f64,
        var_dv__blk144_dn10: f64,
        var_dv__blk144_dn11: f64,
        var_dv__blk144_dn12: f64,
        var_dv__blk144_dn13: f64,
        var_dv__blk144_dn2: f64,
        var_dv__blk144_dn3: f64,
        var_dv__blk144_dn4: f64,
        var_dv__blk144_dn5: f64,
        var_dv__blk144_dn6: f64,
        var_dv__blk144_dn7: f64,
        var_dv__blk144_dn8: f64,
        var_dv__blk144_dn9: f64,
        var_guard135: f64,
        var_guard147: f64,
        var_pe_t: f64,
        var_pe_t_dn0: f64,
        var_pe_t_dn1: f64,
        var_pe_t_dn10: f64,
        var_pe_t_dn11: f64,
        var_pe_t_dn12: f64,
        var_pe_t_dn13: f64,
        var_pe_t_dn2: f64,
        var_pe_t_dn3: f64,
        var_pe_t_dn4: f64,
        var_pe_t_dn5: f64,
        var_pe_t_dn6: f64,
        var_pe_t_dn7: f64,
        var_pe_t_dn8: f64,
        var_pe_t_dn9: f64,
        var_ps_t: f64,
        var_ps_t_dn0: f64,
        var_ps_t_dn1: f64,
        var_ps_t_dn10: f64,
        var_ps_t_dn11: f64,
        var_ps_t_dn12: f64,
        var_ps_t_dn13: f64,
        var_ps_t_dn2: f64,
        var_ps_t_dn3: f64,
        var_ps_t_dn4: f64,
        var_ps_t_dn5: f64,
        var_ps_t_dn6: f64,
        var_ps_t_dn7: f64,
        var_ps_t_dn8: f64,
        var_ps_t_dn9: f64,
        var_q0__blk143: f64,
        var_q0__blk143_dn0: f64,
        var_q0__blk143_dn1: f64,
        var_q0__blk143_dn10: f64,
        var_q0__blk143_dn11: f64,
        var_q0__blk143_dn12: f64,
        var_q0__blk143_dn13: f64,
        var_q0__blk143_dn2: f64,
        var_q0__blk143_dn3: f64,
        var_q0__blk143_dn4: f64,
        var_q0__blk143_dn5: f64,
        var_q0__blk143_dn6: f64,
        var_q0__blk143_dn7: f64,
        var_q0__blk143_dn8: f64,
        var_q0__blk143_dn9: f64,
        var_vbcp: f64,
        var_vbcp_dn0: f64,
        var_vbcp_dn1: f64,
        var_vbcp_dn10: f64,
        var_vbcp_dn11: f64,
        var_vbcp_dn12: f64,
        var_vbcp_dn13: f64,
        var_vbcp_dn2: f64,
        var_vbcp_dn3: f64,
        var_vbcp_dn4: f64,
        var_vbcp_dn5: f64,
        var_vbcp_dn6: f64,
        var_vbcp_dn7: f64,
        var_vbcp_dn8: f64,
        var_vbcp_dn9: f64,
        var_vbex: f64,
        var_vbex_dn0: f64,
        var_vbex_dn1: f64,
        var_vbex_dn10: f64,
        var_vbex_dn11: f64,
        var_vbex_dn12: f64,
        var_vbex_dn13: f64,
        var_vbex_dn2: f64,
        var_vbex_dn3: f64,
        var_vbex_dn4: f64,
        var_vbex_dn5: f64,
        var_vbex_dn6: f64,
        var_vbex_dn7: f64,
        var_vbex_dn8: f64,
        var_vbex_dn9: f64,
        var_vl0__blk142: f64,
        var_vl0__blk142_dn0: f64,
        var_vl0__blk142_dn1: f64,
        var_vl0__blk142_dn10: f64,
        var_vl0__blk142_dn11: f64,
        var_vl0__blk142_dn12: f64,
        var_vl0__blk142_dn13: f64,
        var_vl0__blk142_dn2: f64,
        var_vl0__blk142_dn3: f64,
        var_vl0__blk142_dn4: f64,
        var_vl0__blk142_dn5: f64,
        var_vl0__blk142_dn6: f64,
        var_vl0__blk142_dn7: f64,
        var_vl0__blk142_dn8: f64,
        var_vl0__blk142_dn9: f64,
        var_dv0__blk149_slot: &mut f64,
        var_dv0__blk149_dn0_slot: &mut f64,
        var_dv0__blk149_dn1_slot: &mut f64,
        var_dv0__blk149_dn10_slot: &mut f64,
        var_dv0__blk149_dn11_slot: &mut f64,
        var_dv0__blk149_dn12_slot: &mut f64,
        var_dv0__blk149_dn13_slot: &mut f64,
        var_dv0__blk149_dn2_slot: &mut f64,
        var_dv0__blk149_dn3_slot: &mut f64,
        var_dv0__blk149_dn4_slot: &mut f64,
        var_dv0__blk149_dn5_slot: &mut f64,
        var_dv0__blk149_dn6_slot: &mut f64,
        var_dv0__blk149_dn7_slot: &mut f64,
        var_dv0__blk149_dn8_slot: &mut f64,
        var_dv0__blk149_dn9_slot: &mut f64,
        var_dv__blk157_slot: &mut f64,
        var_dv__blk157_dn0_slot: &mut f64,
        var_dv__blk157_dn1_slot: &mut f64,
        var_dv__blk157_dn10_slot: &mut f64,
        var_dv__blk157_dn11_slot: &mut f64,
        var_dv__blk157_dn12_slot: &mut f64,
        var_dv__blk157_dn13_slot: &mut f64,
        var_dv__blk157_dn2_slot: &mut f64,
        var_dv__blk157_dn3_slot: &mut f64,
        var_dv__blk157_dn4_slot: &mut f64,
        var_dv__blk157_dn5_slot: &mut f64,
        var_dv__blk157_dn6_slot: &mut f64,
        var_dv__blk157_dn7_slot: &mut f64,
        var_dv__blk157_dn8_slot: &mut f64,
        var_dv__blk157_dn9_slot: &mut f64,
        var_dvh__blk150_slot: &mut f64,
        var_dvh__blk150_dn0_slot: &mut f64,
        var_dvh__blk150_dn1_slot: &mut f64,
        var_dvh__blk150_dn10_slot: &mut f64,
        var_dvh__blk150_dn11_slot: &mut f64,
        var_dvh__blk150_dn12_slot: &mut f64,
        var_dvh__blk150_dn13_slot: &mut f64,
        var_dvh__blk150_dn2_slot: &mut f64,
        var_dvh__blk150_dn3_slot: &mut f64,
        var_dvh__blk150_dn4_slot: &mut f64,
        var_dvh__blk150_dn5_slot: &mut f64,
        var_dvh__blk150_dn6_slot: &mut f64,
        var_dvh__blk150_dn7_slot: &mut f64,
        var_dvh__blk150_dn8_slot: &mut f64,
        var_dvh__blk150_dn9_slot: &mut f64,
        var_guard160_slot: &mut f64,
        var_guard161_slot: &mut f64,
        var_mv0__blk154_slot: &mut f64,
        var_mv0__blk154_dn0_slot: &mut f64,
        var_mv0__blk154_dn1_slot: &mut f64,
        var_mv0__blk154_dn10_slot: &mut f64,
        var_mv0__blk154_dn11_slot: &mut f64,
        var_mv0__blk154_dn12_slot: &mut f64,
        var_mv0__blk154_dn13_slot: &mut f64,
        var_mv0__blk154_dn2_slot: &mut f64,
        var_mv0__blk154_dn3_slot: &mut f64,
        var_mv0__blk154_dn4_slot: &mut f64,
        var_mv0__blk154_dn5_slot: &mut f64,
        var_mv0__blk154_dn6_slot: &mut f64,
        var_mv0__blk154_dn7_slot: &mut f64,
        var_mv0__blk154_dn8_slot: &mut f64,
        var_mv0__blk154_dn9_slot: &mut f64,
        var_mv__blk145_slot: &mut f64,
        var_mv__blk145_dn0_slot: &mut f64,
        var_mv__blk145_dn1_slot: &mut f64,
        var_mv__blk145_dn10_slot: &mut f64,
        var_mv__blk145_dn11_slot: &mut f64,
        var_mv__blk145_dn12_slot: &mut f64,
        var_mv__blk145_dn13_slot: &mut f64,
        var_mv__blk145_dn2_slot: &mut f64,
        var_mv__blk145_dn3_slot: &mut f64,
        var_mv__blk145_dn4_slot: &mut f64,
        var_mv__blk145_dn5_slot: &mut f64,
        var_mv__blk145_dn6_slot: &mut f64,
        var_mv__blk145_dn7_slot: &mut f64,
        var_mv__blk145_dn8_slot: &mut f64,
        var_mv__blk145_dn9_slot: &mut f64,
        var_mv__blk158_slot: &mut f64,
        var_mv__blk158_dn0_slot: &mut f64,
        var_mv__blk158_dn1_slot: &mut f64,
        var_mv__blk158_dn10_slot: &mut f64,
        var_mv__blk158_dn11_slot: &mut f64,
        var_mv__blk158_dn12_slot: &mut f64,
        var_mv__blk158_dn13_slot: &mut f64,
        var_mv__blk158_dn2_slot: &mut f64,
        var_mv__blk158_dn3_slot: &mut f64,
        var_mv__blk158_dn4_slot: &mut f64,
        var_mv__blk158_dn5_slot: &mut f64,
        var_mv__blk158_dn6_slot: &mut f64,
        var_mv__blk158_dn7_slot: &mut f64,
        var_mv__blk158_dn8_slot: &mut f64,
        var_mv__blk158_dn9_slot: &mut f64,
        var_pwq__blk151_slot: &mut f64,
        var_q0__blk156_slot: &mut f64,
        var_q0__blk156_dn0_slot: &mut f64,
        var_q0__blk156_dn1_slot: &mut f64,
        var_q0__blk156_dn10_slot: &mut f64,
        var_q0__blk156_dn11_slot: &mut f64,
        var_q0__blk156_dn12_slot: &mut f64,
        var_q0__blk156_dn13_slot: &mut f64,
        var_q0__blk156_dn2_slot: &mut f64,
        var_q0__blk156_dn3_slot: &mut f64,
        var_q0__blk156_dn4_slot: &mut f64,
        var_q0__blk156_dn5_slot: &mut f64,
        var_q0__blk156_dn6_slot: &mut f64,
        var_q0__blk156_dn7_slot: &mut f64,
        var_q0__blk156_dn8_slot: &mut f64,
        var_q0__blk156_dn9_slot: &mut f64,
        var_qdbcp_slot: &mut f64,
        var_qdbcp_dn0_slot: &mut f64,
        var_qdbcp_dn1_slot: &mut f64,
        var_qdbcp_dn10_slot: &mut f64,
        var_qdbcp_dn11_slot: &mut f64,
        var_qdbcp_dn12_slot: &mut f64,
        var_qdbcp_dn13_slot: &mut f64,
        var_qdbcp_dn2_slot: &mut f64,
        var_qdbcp_dn3_slot: &mut f64,
        var_qdbcp_dn4_slot: &mut f64,
        var_qdbcp_dn5_slot: &mut f64,
        var_qdbcp_dn6_slot: &mut f64,
        var_qdbcp_dn7_slot: &mut f64,
        var_qdbcp_dn8_slot: &mut f64,
        var_qdbcp_dn9_slot: &mut f64,
        var_qdbex_slot: &mut f64,
        var_qdbex_dn0_slot: &mut f64,
        var_qdbex_dn1_slot: &mut f64,
        var_qdbex_dn10_slot: &mut f64,
        var_qdbex_dn11_slot: &mut f64,
        var_qdbex_dn12_slot: &mut f64,
        var_qdbex_dn13_slot: &mut f64,
        var_qdbex_dn2_slot: &mut f64,
        var_qdbex_dn3_slot: &mut f64,
        var_qdbex_dn4_slot: &mut f64,
        var_qdbex_dn5_slot: &mut f64,
        var_qdbex_dn6_slot: &mut f64,
        var_qdbex_dn7_slot: &mut f64,
        var_qdbex_dn8_slot: &mut f64,
        var_qdbex_dn9_slot: &mut f64,
        var_qhi__blk153_slot: &mut f64,
        var_qhi__blk153_dn0_slot: &mut f64,
        var_qhi__blk153_dn1_slot: &mut f64,
        var_qhi__blk153_dn10_slot: &mut f64,
        var_qhi__blk153_dn11_slot: &mut f64,
        var_qhi__blk153_dn12_slot: &mut f64,
        var_qhi__blk153_dn13_slot: &mut f64,
        var_qhi__blk153_dn2_slot: &mut f64,
        var_qhi__blk153_dn3_slot: &mut f64,
        var_qhi__blk153_dn4_slot: &mut f64,
        var_qhi__blk153_dn5_slot: &mut f64,
        var_qhi__blk153_dn6_slot: &mut f64,
        var_qhi__blk153_dn7_slot: &mut f64,
        var_qhi__blk153_dn8_slot: &mut f64,
        var_qhi__blk153_dn9_slot: &mut f64,
        var_qlo__blk139_slot: &mut f64,
        var_qlo__blk139_dn0_slot: &mut f64,
        var_qlo__blk139_dn1_slot: &mut f64,
        var_qlo__blk139_dn10_slot: &mut f64,
        var_qlo__blk139_dn11_slot: &mut f64,
        var_qlo__blk139_dn12_slot: &mut f64,
        var_qlo__blk139_dn13_slot: &mut f64,
        var_qlo__blk139_dn2_slot: &mut f64,
        var_qlo__blk139_dn3_slot: &mut f64,
        var_qlo__blk139_dn4_slot: &mut f64,
        var_qlo__blk139_dn5_slot: &mut f64,
        var_qlo__blk139_dn6_slot: &mut f64,
        var_qlo__blk139_dn7_slot: &mut f64,
        var_qlo__blk139_dn8_slot: &mut f64,
        var_qlo__blk139_dn9_slot: &mut f64,
        var_qlo__blk152_slot: &mut f64,
        var_qlo__blk152_dn0_slot: &mut f64,
        var_qlo__blk152_dn1_slot: &mut f64,
        var_qlo__blk152_dn10_slot: &mut f64,
        var_qlo__blk152_dn11_slot: &mut f64,
        var_qlo__blk152_dn12_slot: &mut f64,
        var_qlo__blk152_dn13_slot: &mut f64,
        var_qlo__blk152_dn2_slot: &mut f64,
        var_qlo__blk152_dn3_slot: &mut f64,
        var_qlo__blk152_dn4_slot: &mut f64,
        var_qlo__blk152_dn5_slot: &mut f64,
        var_qlo__blk152_dn6_slot: &mut f64,
        var_qlo__blk152_dn7_slot: &mut f64,
        var_qlo__blk152_dn8_slot: &mut f64,
        var_qlo__blk152_dn9_slot: &mut f64,
        var_vl0__blk155_slot: &mut f64,
        var_vl0__blk155_dn0_slot: &mut f64,
        var_vl0__blk155_dn1_slot: &mut f64,
        var_vl0__blk155_dn10_slot: &mut f64,
        var_vl0__blk155_dn11_slot: &mut f64,
        var_vl0__blk155_dn12_slot: &mut f64,
        var_vl0__blk155_dn13_slot: &mut f64,
        var_vl0__blk155_dn2_slot: &mut f64,
        var_vl0__blk155_dn3_slot: &mut f64,
        var_vl0__blk155_dn4_slot: &mut f64,
        var_vl0__blk155_dn5_slot: &mut f64,
        var_vl0__blk155_dn6_slot: &mut f64,
        var_vl0__blk155_dn7_slot: &mut f64,
        var_vl0__blk155_dn8_slot: &mut f64,
        var_vl0__blk155_dn9_slot: &mut f64,
        var_vl__blk146_slot: &mut f64,
        var_vl__blk146_dn0_slot: &mut f64,
        var_vl__blk146_dn1_slot: &mut f64,
        var_vl__blk146_dn10_slot: &mut f64,
        var_vl__blk146_dn11_slot: &mut f64,
        var_vl__blk146_dn12_slot: &mut f64,
        var_vl__blk146_dn13_slot: &mut f64,
        var_vl__blk146_dn2_slot: &mut f64,
        var_vl__blk146_dn3_slot: &mut f64,
        var_vl__blk146_dn4_slot: &mut f64,
        var_vl__blk146_dn5_slot: &mut f64,
        var_vl__blk146_dn6_slot: &mut f64,
        var_vl__blk146_dn7_slot: &mut f64,
        var_vl__blk146_dn8_slot: &mut f64,
        var_vl__blk146_dn9_slot: &mut f64,
        var_vl__blk159_slot: &mut f64,
        var_vl__blk159_dn0_slot: &mut f64,
        var_vl__blk159_dn1_slot: &mut f64,
        var_vl__blk159_dn10_slot: &mut f64,
        var_vl__blk159_dn11_slot: &mut f64,
        var_vl__blk159_dn12_slot: &mut f64,
        var_vl__blk159_dn13_slot: &mut f64,
        var_vl__blk159_dn2_slot: &mut f64,
        var_vl__blk159_dn3_slot: &mut f64,
        var_vl__blk159_dn4_slot: &mut f64,
        var_vl__blk159_dn5_slot: &mut f64,
        var_vl__blk159_dn6_slot: &mut f64,
        var_vl__blk159_dn7_slot: &mut f64,
        var_vl__blk159_dn8_slot: &mut f64,
        var_vl__blk159_dn9_slot: &mut f64,
    ) {
        let mut var_dv0__blk149: f64 = *var_dv0__blk149_slot;
        let mut var_dv0__blk149_dn0: f64 = *var_dv0__blk149_dn0_slot;
        let mut var_dv0__blk149_dn1: f64 = *var_dv0__blk149_dn1_slot;
        let mut var_dv0__blk149_dn10: f64 = *var_dv0__blk149_dn10_slot;
        let mut var_dv0__blk149_dn11: f64 = *var_dv0__blk149_dn11_slot;
        let mut var_dv0__blk149_dn12: f64 = *var_dv0__blk149_dn12_slot;
        let mut var_dv0__blk149_dn13: f64 = *var_dv0__blk149_dn13_slot;
        let mut var_dv0__blk149_dn2: f64 = *var_dv0__blk149_dn2_slot;
        let mut var_dv0__blk149_dn3: f64 = *var_dv0__blk149_dn3_slot;
        let mut var_dv0__blk149_dn4: f64 = *var_dv0__blk149_dn4_slot;
        let mut var_dv0__blk149_dn5: f64 = *var_dv0__blk149_dn5_slot;
        let mut var_dv0__blk149_dn6: f64 = *var_dv0__blk149_dn6_slot;
        let mut var_dv0__blk149_dn7: f64 = *var_dv0__blk149_dn7_slot;
        let mut var_dv0__blk149_dn8: f64 = *var_dv0__blk149_dn8_slot;
        let mut var_dv0__blk149_dn9: f64 = *var_dv0__blk149_dn9_slot;
        let mut var_dv__blk157: f64 = *var_dv__blk157_slot;
        let mut var_dv__blk157_dn0: f64 = *var_dv__blk157_dn0_slot;
        let mut var_dv__blk157_dn1: f64 = *var_dv__blk157_dn1_slot;
        let mut var_dv__blk157_dn10: f64 = *var_dv__blk157_dn10_slot;
        let mut var_dv__blk157_dn11: f64 = *var_dv__blk157_dn11_slot;
        let mut var_dv__blk157_dn12: f64 = *var_dv__blk157_dn12_slot;
        let mut var_dv__blk157_dn13: f64 = *var_dv__blk157_dn13_slot;
        let mut var_dv__blk157_dn2: f64 = *var_dv__blk157_dn2_slot;
        let mut var_dv__blk157_dn3: f64 = *var_dv__blk157_dn3_slot;
        let mut var_dv__blk157_dn4: f64 = *var_dv__blk157_dn4_slot;
        let mut var_dv__blk157_dn5: f64 = *var_dv__blk157_dn5_slot;
        let mut var_dv__blk157_dn6: f64 = *var_dv__blk157_dn6_slot;
        let mut var_dv__blk157_dn7: f64 = *var_dv__blk157_dn7_slot;
        let mut var_dv__blk157_dn8: f64 = *var_dv__blk157_dn8_slot;
        let mut var_dv__blk157_dn9: f64 = *var_dv__blk157_dn9_slot;
        let mut var_dvh__blk150: f64 = *var_dvh__blk150_slot;
        let mut var_dvh__blk150_dn0: f64 = *var_dvh__blk150_dn0_slot;
        let mut var_dvh__blk150_dn1: f64 = *var_dvh__blk150_dn1_slot;
        let mut var_dvh__blk150_dn10: f64 = *var_dvh__blk150_dn10_slot;
        let mut var_dvh__blk150_dn11: f64 = *var_dvh__blk150_dn11_slot;
        let mut var_dvh__blk150_dn12: f64 = *var_dvh__blk150_dn12_slot;
        let mut var_dvh__blk150_dn13: f64 = *var_dvh__blk150_dn13_slot;
        let mut var_dvh__blk150_dn2: f64 = *var_dvh__blk150_dn2_slot;
        let mut var_dvh__blk150_dn3: f64 = *var_dvh__blk150_dn3_slot;
        let mut var_dvh__blk150_dn4: f64 = *var_dvh__blk150_dn4_slot;
        let mut var_dvh__blk150_dn5: f64 = *var_dvh__blk150_dn5_slot;
        let mut var_dvh__blk150_dn6: f64 = *var_dvh__blk150_dn6_slot;
        let mut var_dvh__blk150_dn7: f64 = *var_dvh__blk150_dn7_slot;
        let mut var_dvh__blk150_dn8: f64 = *var_dvh__blk150_dn8_slot;
        let mut var_dvh__blk150_dn9: f64 = *var_dvh__blk150_dn9_slot;
        let mut var_guard160: f64 = *var_guard160_slot;
        let mut var_guard161: f64 = *var_guard161_slot;
        let mut var_mv0__blk154: f64 = *var_mv0__blk154_slot;
        let mut var_mv0__blk154_dn0: f64 = *var_mv0__blk154_dn0_slot;
        let mut var_mv0__blk154_dn1: f64 = *var_mv0__blk154_dn1_slot;
        let mut var_mv0__blk154_dn10: f64 = *var_mv0__blk154_dn10_slot;
        let mut var_mv0__blk154_dn11: f64 = *var_mv0__blk154_dn11_slot;
        let mut var_mv0__blk154_dn12: f64 = *var_mv0__blk154_dn12_slot;
        let mut var_mv0__blk154_dn13: f64 = *var_mv0__blk154_dn13_slot;
        let mut var_mv0__blk154_dn2: f64 = *var_mv0__blk154_dn2_slot;
        let mut var_mv0__blk154_dn3: f64 = *var_mv0__blk154_dn3_slot;
        let mut var_mv0__blk154_dn4: f64 = *var_mv0__blk154_dn4_slot;
        let mut var_mv0__blk154_dn5: f64 = *var_mv0__blk154_dn5_slot;
        let mut var_mv0__blk154_dn6: f64 = *var_mv0__blk154_dn6_slot;
        let mut var_mv0__blk154_dn7: f64 = *var_mv0__blk154_dn7_slot;
        let mut var_mv0__blk154_dn8: f64 = *var_mv0__blk154_dn8_slot;
        let mut var_mv0__blk154_dn9: f64 = *var_mv0__blk154_dn9_slot;
        let mut var_mv__blk145: f64 = *var_mv__blk145_slot;
        let mut var_mv__blk145_dn0: f64 = *var_mv__blk145_dn0_slot;
        let mut var_mv__blk145_dn1: f64 = *var_mv__blk145_dn1_slot;
        let mut var_mv__blk145_dn10: f64 = *var_mv__blk145_dn10_slot;
        let mut var_mv__blk145_dn11: f64 = *var_mv__blk145_dn11_slot;
        let mut var_mv__blk145_dn12: f64 = *var_mv__blk145_dn12_slot;
        let mut var_mv__blk145_dn13: f64 = *var_mv__blk145_dn13_slot;
        let mut var_mv__blk145_dn2: f64 = *var_mv__blk145_dn2_slot;
        let mut var_mv__blk145_dn3: f64 = *var_mv__blk145_dn3_slot;
        let mut var_mv__blk145_dn4: f64 = *var_mv__blk145_dn4_slot;
        let mut var_mv__blk145_dn5: f64 = *var_mv__blk145_dn5_slot;
        let mut var_mv__blk145_dn6: f64 = *var_mv__blk145_dn6_slot;
        let mut var_mv__blk145_dn7: f64 = *var_mv__blk145_dn7_slot;
        let mut var_mv__blk145_dn8: f64 = *var_mv__blk145_dn8_slot;
        let mut var_mv__blk145_dn9: f64 = *var_mv__blk145_dn9_slot;
        let mut var_mv__blk158: f64 = *var_mv__blk158_slot;
        let mut var_mv__blk158_dn0: f64 = *var_mv__blk158_dn0_slot;
        let mut var_mv__blk158_dn1: f64 = *var_mv__blk158_dn1_slot;
        let mut var_mv__blk158_dn10: f64 = *var_mv__blk158_dn10_slot;
        let mut var_mv__blk158_dn11: f64 = *var_mv__blk158_dn11_slot;
        let mut var_mv__blk158_dn12: f64 = *var_mv__blk158_dn12_slot;
        let mut var_mv__blk158_dn13: f64 = *var_mv__blk158_dn13_slot;
        let mut var_mv__blk158_dn2: f64 = *var_mv__blk158_dn2_slot;
        let mut var_mv__blk158_dn3: f64 = *var_mv__blk158_dn3_slot;
        let mut var_mv__blk158_dn4: f64 = *var_mv__blk158_dn4_slot;
        let mut var_mv__blk158_dn5: f64 = *var_mv__blk158_dn5_slot;
        let mut var_mv__blk158_dn6: f64 = *var_mv__blk158_dn6_slot;
        let mut var_mv__blk158_dn7: f64 = *var_mv__blk158_dn7_slot;
        let mut var_mv__blk158_dn8: f64 = *var_mv__blk158_dn8_slot;
        let mut var_mv__blk158_dn9: f64 = *var_mv__blk158_dn9_slot;
        let mut var_pwq__blk151: f64 = *var_pwq__blk151_slot;
        let mut var_q0__blk156: f64 = *var_q0__blk156_slot;
        let mut var_q0__blk156_dn0: f64 = *var_q0__blk156_dn0_slot;
        let mut var_q0__blk156_dn1: f64 = *var_q0__blk156_dn1_slot;
        let mut var_q0__blk156_dn10: f64 = *var_q0__blk156_dn10_slot;
        let mut var_q0__blk156_dn11: f64 = *var_q0__blk156_dn11_slot;
        let mut var_q0__blk156_dn12: f64 = *var_q0__blk156_dn12_slot;
        let mut var_q0__blk156_dn13: f64 = *var_q0__blk156_dn13_slot;
        let mut var_q0__blk156_dn2: f64 = *var_q0__blk156_dn2_slot;
        let mut var_q0__blk156_dn3: f64 = *var_q0__blk156_dn3_slot;
        let mut var_q0__blk156_dn4: f64 = *var_q0__blk156_dn4_slot;
        let mut var_q0__blk156_dn5: f64 = *var_q0__blk156_dn5_slot;
        let mut var_q0__blk156_dn6: f64 = *var_q0__blk156_dn6_slot;
        let mut var_q0__blk156_dn7: f64 = *var_q0__blk156_dn7_slot;
        let mut var_q0__blk156_dn8: f64 = *var_q0__blk156_dn8_slot;
        let mut var_q0__blk156_dn9: f64 = *var_q0__blk156_dn9_slot;
        let mut var_qdbcp: f64 = *var_qdbcp_slot;
        let mut var_qdbcp_dn0: f64 = *var_qdbcp_dn0_slot;
        let mut var_qdbcp_dn1: f64 = *var_qdbcp_dn1_slot;
        let mut var_qdbcp_dn10: f64 = *var_qdbcp_dn10_slot;
        let mut var_qdbcp_dn11: f64 = *var_qdbcp_dn11_slot;
        let mut var_qdbcp_dn12: f64 = *var_qdbcp_dn12_slot;
        let mut var_qdbcp_dn13: f64 = *var_qdbcp_dn13_slot;
        let mut var_qdbcp_dn2: f64 = *var_qdbcp_dn2_slot;
        let mut var_qdbcp_dn3: f64 = *var_qdbcp_dn3_slot;
        let mut var_qdbcp_dn4: f64 = *var_qdbcp_dn4_slot;
        let mut var_qdbcp_dn5: f64 = *var_qdbcp_dn5_slot;
        let mut var_qdbcp_dn6: f64 = *var_qdbcp_dn6_slot;
        let mut var_qdbcp_dn7: f64 = *var_qdbcp_dn7_slot;
        let mut var_qdbcp_dn8: f64 = *var_qdbcp_dn8_slot;
        let mut var_qdbcp_dn9: f64 = *var_qdbcp_dn9_slot;
        let mut var_qdbex: f64 = *var_qdbex_slot;
        let mut var_qdbex_dn0: f64 = *var_qdbex_dn0_slot;
        let mut var_qdbex_dn1: f64 = *var_qdbex_dn1_slot;
        let mut var_qdbex_dn10: f64 = *var_qdbex_dn10_slot;
        let mut var_qdbex_dn11: f64 = *var_qdbex_dn11_slot;
        let mut var_qdbex_dn12: f64 = *var_qdbex_dn12_slot;
        let mut var_qdbex_dn13: f64 = *var_qdbex_dn13_slot;
        let mut var_qdbex_dn2: f64 = *var_qdbex_dn2_slot;
        let mut var_qdbex_dn3: f64 = *var_qdbex_dn3_slot;
        let mut var_qdbex_dn4: f64 = *var_qdbex_dn4_slot;
        let mut var_qdbex_dn5: f64 = *var_qdbex_dn5_slot;
        let mut var_qdbex_dn6: f64 = *var_qdbex_dn6_slot;
        let mut var_qdbex_dn7: f64 = *var_qdbex_dn7_slot;
        let mut var_qdbex_dn8: f64 = *var_qdbex_dn8_slot;
        let mut var_qdbex_dn9: f64 = *var_qdbex_dn9_slot;
        let mut var_qhi__blk153: f64 = *var_qhi__blk153_slot;
        let mut var_qhi__blk153_dn0: f64 = *var_qhi__blk153_dn0_slot;
        let mut var_qhi__blk153_dn1: f64 = *var_qhi__blk153_dn1_slot;
        let mut var_qhi__blk153_dn10: f64 = *var_qhi__blk153_dn10_slot;
        let mut var_qhi__blk153_dn11: f64 = *var_qhi__blk153_dn11_slot;
        let mut var_qhi__blk153_dn12: f64 = *var_qhi__blk153_dn12_slot;
        let mut var_qhi__blk153_dn13: f64 = *var_qhi__blk153_dn13_slot;
        let mut var_qhi__blk153_dn2: f64 = *var_qhi__blk153_dn2_slot;
        let mut var_qhi__blk153_dn3: f64 = *var_qhi__blk153_dn3_slot;
        let mut var_qhi__blk153_dn4: f64 = *var_qhi__blk153_dn4_slot;
        let mut var_qhi__blk153_dn5: f64 = *var_qhi__blk153_dn5_slot;
        let mut var_qhi__blk153_dn6: f64 = *var_qhi__blk153_dn6_slot;
        let mut var_qhi__blk153_dn7: f64 = *var_qhi__blk153_dn7_slot;
        let mut var_qhi__blk153_dn8: f64 = *var_qhi__blk153_dn8_slot;
        let mut var_qhi__blk153_dn9: f64 = *var_qhi__blk153_dn9_slot;
        let mut var_qlo__blk139: f64 = *var_qlo__blk139_slot;
        let mut var_qlo__blk139_dn0: f64 = *var_qlo__blk139_dn0_slot;
        let mut var_qlo__blk139_dn1: f64 = *var_qlo__blk139_dn1_slot;
        let mut var_qlo__blk139_dn10: f64 = *var_qlo__blk139_dn10_slot;
        let mut var_qlo__blk139_dn11: f64 = *var_qlo__blk139_dn11_slot;
        let mut var_qlo__blk139_dn12: f64 = *var_qlo__blk139_dn12_slot;
        let mut var_qlo__blk139_dn13: f64 = *var_qlo__blk139_dn13_slot;
        let mut var_qlo__blk139_dn2: f64 = *var_qlo__blk139_dn2_slot;
        let mut var_qlo__blk139_dn3: f64 = *var_qlo__blk139_dn3_slot;
        let mut var_qlo__blk139_dn4: f64 = *var_qlo__blk139_dn4_slot;
        let mut var_qlo__blk139_dn5: f64 = *var_qlo__blk139_dn5_slot;
        let mut var_qlo__blk139_dn6: f64 = *var_qlo__blk139_dn6_slot;
        let mut var_qlo__blk139_dn7: f64 = *var_qlo__blk139_dn7_slot;
        let mut var_qlo__blk139_dn8: f64 = *var_qlo__blk139_dn8_slot;
        let mut var_qlo__blk139_dn9: f64 = *var_qlo__blk139_dn9_slot;
        let mut var_qlo__blk152: f64 = *var_qlo__blk152_slot;
        let mut var_qlo__blk152_dn0: f64 = *var_qlo__blk152_dn0_slot;
        let mut var_qlo__blk152_dn1: f64 = *var_qlo__blk152_dn1_slot;
        let mut var_qlo__blk152_dn10: f64 = *var_qlo__blk152_dn10_slot;
        let mut var_qlo__blk152_dn11: f64 = *var_qlo__blk152_dn11_slot;
        let mut var_qlo__blk152_dn12: f64 = *var_qlo__blk152_dn12_slot;
        let mut var_qlo__blk152_dn13: f64 = *var_qlo__blk152_dn13_slot;
        let mut var_qlo__blk152_dn2: f64 = *var_qlo__blk152_dn2_slot;
        let mut var_qlo__blk152_dn3: f64 = *var_qlo__blk152_dn3_slot;
        let mut var_qlo__blk152_dn4: f64 = *var_qlo__blk152_dn4_slot;
        let mut var_qlo__blk152_dn5: f64 = *var_qlo__blk152_dn5_slot;
        let mut var_qlo__blk152_dn6: f64 = *var_qlo__blk152_dn6_slot;
        let mut var_qlo__blk152_dn7: f64 = *var_qlo__blk152_dn7_slot;
        let mut var_qlo__blk152_dn8: f64 = *var_qlo__blk152_dn8_slot;
        let mut var_qlo__blk152_dn9: f64 = *var_qlo__blk152_dn9_slot;
        let mut var_vl0__blk155: f64 = *var_vl0__blk155_slot;
        let mut var_vl0__blk155_dn0: f64 = *var_vl0__blk155_dn0_slot;
        let mut var_vl0__blk155_dn1: f64 = *var_vl0__blk155_dn1_slot;
        let mut var_vl0__blk155_dn10: f64 = *var_vl0__blk155_dn10_slot;
        let mut var_vl0__blk155_dn11: f64 = *var_vl0__blk155_dn11_slot;
        let mut var_vl0__blk155_dn12: f64 = *var_vl0__blk155_dn12_slot;
        let mut var_vl0__blk155_dn13: f64 = *var_vl0__blk155_dn13_slot;
        let mut var_vl0__blk155_dn2: f64 = *var_vl0__blk155_dn2_slot;
        let mut var_vl0__blk155_dn3: f64 = *var_vl0__blk155_dn3_slot;
        let mut var_vl0__blk155_dn4: f64 = *var_vl0__blk155_dn4_slot;
        let mut var_vl0__blk155_dn5: f64 = *var_vl0__blk155_dn5_slot;
        let mut var_vl0__blk155_dn6: f64 = *var_vl0__blk155_dn6_slot;
        let mut var_vl0__blk155_dn7: f64 = *var_vl0__blk155_dn7_slot;
        let mut var_vl0__blk155_dn8: f64 = *var_vl0__blk155_dn8_slot;
        let mut var_vl0__blk155_dn9: f64 = *var_vl0__blk155_dn9_slot;
        let mut var_vl__blk146: f64 = *var_vl__blk146_slot;
        let mut var_vl__blk146_dn0: f64 = *var_vl__blk146_dn0_slot;
        let mut var_vl__blk146_dn1: f64 = *var_vl__blk146_dn1_slot;
        let mut var_vl__blk146_dn10: f64 = *var_vl__blk146_dn10_slot;
        let mut var_vl__blk146_dn11: f64 = *var_vl__blk146_dn11_slot;
        let mut var_vl__blk146_dn12: f64 = *var_vl__blk146_dn12_slot;
        let mut var_vl__blk146_dn13: f64 = *var_vl__blk146_dn13_slot;
        let mut var_vl__blk146_dn2: f64 = *var_vl__blk146_dn2_slot;
        let mut var_vl__blk146_dn3: f64 = *var_vl__blk146_dn3_slot;
        let mut var_vl__blk146_dn4: f64 = *var_vl__blk146_dn4_slot;
        let mut var_vl__blk146_dn5: f64 = *var_vl__blk146_dn5_slot;
        let mut var_vl__blk146_dn6: f64 = *var_vl__blk146_dn6_slot;
        let mut var_vl__blk146_dn7: f64 = *var_vl__blk146_dn7_slot;
        let mut var_vl__blk146_dn8: f64 = *var_vl__blk146_dn8_slot;
        let mut var_vl__blk146_dn9: f64 = *var_vl__blk146_dn9_slot;
        let mut var_vl__blk159: f64 = *var_vl__blk159_slot;
        let mut var_vl__blk159_dn0: f64 = *var_vl__blk159_dn0_slot;
        let mut var_vl__blk159_dn1: f64 = *var_vl__blk159_dn1_slot;
        let mut var_vl__blk159_dn10: f64 = *var_vl__blk159_dn10_slot;
        let mut var_vl__blk159_dn11: f64 = *var_vl__blk159_dn11_slot;
        let mut var_vl__blk159_dn12: f64 = *var_vl__blk159_dn12_slot;
        let mut var_vl__blk159_dn13: f64 = *var_vl__blk159_dn13_slot;
        let mut var_vl__blk159_dn2: f64 = *var_vl__blk159_dn2_slot;
        let mut var_vl__blk159_dn3: f64 = *var_vl__blk159_dn3_slot;
        let mut var_vl__blk159_dn4: f64 = *var_vl__blk159_dn4_slot;
        let mut var_vl__blk159_dn5: f64 = *var_vl__blk159_dn5_slot;
        let mut var_vl__blk159_dn6: f64 = *var_vl__blk159_dn6_slot;
        let mut var_vl__blk159_dn7: f64 = *var_vl__blk159_dn7_slot;
        let mut var_vl__blk159_dn8: f64 = *var_vl__blk159_dn8_slot;
        let mut var_vl__blk159_dn9: f64 = *var_vl__blk159_dn9_slot;

        let (assign4900_e5217, assign4900_e5217_d_n0, assign4900_e5217_d_n1, assign4900_e5217_d_n2, assign4900_e5217_d_n3, assign4900_e5217_d_n4, assign4900_e5217_d_n5, assign4900_e5217_d_n6, assign4900_e5217_d_n7, assign4900_e5217_d_n8, assign4900_e5217_d_n9, assign4900_e5217_d_n10, assign4900_e5217_d_n11, assign4900_e5217_d_n12, assign4900_e5217_d_n13,) = {
    if ((var_guard135 != 0.0) && (var_guard147 == 0.0)) {
        let assign4900_e5208: f64 = (var_dv__blk144 * var_dv__blk144);
        let assign4900_e5211: f64 = (4.0 * p.p52);
        let assign4900_e5213: f64 = (assign4900_e5211 * p.p52);
        let assign4900_e5214: f64 = (assign4900_e5208 + assign4900_e5213);
        let assign4900_e5215: f64 = (assign4900_e5214).sqrt();
        (assign4900_e5215, (((var_dv__blk144_dn0 * var_dv__blk144) + (var_dv__blk144 * var_dv__blk144_dn0)) / (2.0 * assign4900_e5215)), (((var_dv__blk144_dn1 * var_dv__blk144) + (var_dv__blk144 * var_dv__blk144_dn1)) / (2.0 * assign4900_e5215)), (((var_dv__blk144_dn2 * var_dv__blk144) + (var_dv__blk144 * var_dv__blk144_dn2)) / (2.0 * assign4900_e5215)), (((var_dv__blk144_dn3 * var_dv__blk144) + (var_dv__blk144 * var_dv__blk144_dn3)) / (2.0 * assign4900_e5215)), (((var_dv__blk144_dn4 * var_dv__blk144) + (var_dv__blk144 * var_dv__blk144_dn4)) / (2.0 * assign4900_e5215)), (((var_dv__blk144_dn5 * var_dv__blk144) + (var_dv__blk144 * var_dv__blk144_dn5)) / (2.0 * assign4900_e5215)), (((var_dv__blk144_dn6 * var_dv__blk144) + (var_dv__blk144 * var_dv__blk144_dn6)) / (2.0 * assign4900_e5215)), (((var_dv__blk144_dn7 * var_dv__blk144) + (var_dv__blk144 * var_dv__blk144_dn7)) / (2.0 * assign4900_e5215)), (((var_dv__blk144_dn8 * var_dv__blk144) + (var_dv__blk144 * var_dv__blk144_dn8)) / (2.0 * assign4900_e5215)), (((var_dv__blk144_dn9 * var_dv__blk144) + (var_dv__blk144 * var_dv__blk144_dn9)) / (2.0 * assign4900_e5215)), (((var_dv__blk144_dn10 * var_dv__blk144) + (var_dv__blk144 * var_dv__blk144_dn10)) / (2.0 * assign4900_e5215)), (((var_dv__blk144_dn11 * var_dv__blk144) + (var_dv__blk144 * var_dv__blk144_dn11)) / (2.0 * assign4900_e5215)), (((var_dv__blk144_dn12 * var_dv__blk144) + (var_dv__blk144 * var_dv__blk144_dn12)) / (2.0 * assign4900_e5215)), (((var_dv__blk144_dn13 * var_dv__blk144) + (var_dv__blk144 * var_dv__blk144_dn13)) / (2.0 * assign4900_e5215)),)
    } else {
        (var_mv__blk145, var_mv__blk145_dn0, var_mv__blk145_dn1, var_mv__blk145_dn2, var_mv__blk145_dn3, var_mv__blk145_dn4, var_mv__blk145_dn5, var_mv__blk145_dn6, var_mv__blk145_dn7, var_mv__blk145_dn8, var_mv__blk145_dn9, var_mv__blk145_dn10, var_mv__blk145_dn11, var_mv__blk145_dn12, var_mv__blk145_dn13,)
    }
};
        var_mv__blk145 = assign4900_e5217;
        var_mv__blk145_dn0 = assign4900_e5217_d_n0;
        var_mv__blk145_dn1 = assign4900_e5217_d_n1;
        var_mv__blk145_dn2 = assign4900_e5217_d_n2;
        var_mv__blk145_dn3 = assign4900_e5217_d_n3;
        var_mv__blk145_dn4 = assign4900_e5217_d_n4;
        var_mv__blk145_dn5 = assign4900_e5217_d_n5;
        var_mv__blk145_dn6 = assign4900_e5217_d_n6;
        var_mv__blk145_dn7 = assign4900_e5217_d_n7;
        var_mv__blk145_dn8 = assign4900_e5217_d_n8;
        var_mv__blk145_dn9 = assign4900_e5217_d_n9;
        var_mv__blk145_dn10 = assign4900_e5217_d_n10;
        var_mv__blk145_dn11 = assign4900_e5217_d_n11;
        var_mv__blk145_dn12 = assign4900_e5217_d_n12;
        var_mv__blk145_dn13 = assign4900_e5217_d_n13;

        let (assign4910_e5230, assign4910_e5230_d_n0, assign4910_e5230_d_n1, assign4910_e5230_d_n2, assign4910_e5230_d_n3, assign4910_e5230_d_n4, assign4910_e5230_d_n5, assign4910_e5230_d_n6, assign4910_e5230_d_n7, assign4910_e5230_d_n8, assign4910_e5230_d_n9, assign4910_e5230_d_n10, assign4910_e5230_d_n11, assign4910_e5230_d_n12, assign4910_e5230_d_n13,) = {
    if ((var_guard135 != 0.0) && (var_guard147 == 0.0)) {
        let assign4910_e5225: f64 = (var_dv__blk144 - var_mv__blk145);
        let assign4910_e5226: f64 = (0.5 * assign4910_e5225);
        let assign4910_e5228: f64 = (assign4910_e5226 - var_dv0__blk136);
        (assign4910_e5228, ((0.5 * (var_dv__blk144_dn0 - var_mv__blk145_dn0)) - var_dv0__blk136_dn0), ((0.5 * (var_dv__blk144_dn1 - var_mv__blk145_dn1)) - var_dv0__blk136_dn1), ((0.5 * (var_dv__blk144_dn2 - var_mv__blk145_dn2)) - var_dv0__blk136_dn2), ((0.5 * (var_dv__blk144_dn3 - var_mv__blk145_dn3)) - var_dv0__blk136_dn3), ((0.5 * (var_dv__blk144_dn4 - var_mv__blk145_dn4)) - var_dv0__blk136_dn4), ((0.5 * (var_dv__blk144_dn5 - var_mv__blk145_dn5)) - var_dv0__blk136_dn5), ((0.5 * (var_dv__blk144_dn6 - var_mv__blk145_dn6)) - var_dv0__blk136_dn6), ((0.5 * (var_dv__blk144_dn7 - var_mv__blk145_dn7)) - var_dv0__blk136_dn7), ((0.5 * (var_dv__blk144_dn8 - var_mv__blk145_dn8)) - var_dv0__blk136_dn8), ((0.5 * (var_dv__blk144_dn9 - var_mv__blk145_dn9)) - var_dv0__blk136_dn9), ((0.5 * (var_dv__blk144_dn10 - var_mv__blk145_dn10)) - var_dv0__blk136_dn10), ((0.5 * (var_dv__blk144_dn11 - var_mv__blk145_dn11)) - var_dv0__blk136_dn11), ((0.5 * (var_dv__blk144_dn12 - var_mv__blk145_dn12)) - var_dv0__blk136_dn12), ((0.5 * (var_dv__blk144_dn13 - var_mv__blk145_dn13)) - var_dv0__blk136_dn13),)
    } else {
        (var_vl__blk146, var_vl__blk146_dn0, var_vl__blk146_dn1, var_vl__blk146_dn2, var_vl__blk146_dn3, var_vl__blk146_dn4, var_vl__blk146_dn5, var_vl__blk146_dn6, var_vl__blk146_dn7, var_vl__blk146_dn8, var_vl__blk146_dn9, var_vl__blk146_dn10, var_vl__blk146_dn11, var_vl__blk146_dn12, var_vl__blk146_dn13,)
    }
};
        var_vl__blk146 = assign4910_e5230;
        var_vl__blk146_dn0 = assign4910_e5230_d_n0;
        var_vl__blk146_dn1 = assign4910_e5230_d_n1;
        var_vl__blk146_dn2 = assign4910_e5230_d_n2;
        var_vl__blk146_dn3 = assign4910_e5230_d_n3;
        var_vl__blk146_dn4 = assign4910_e5230_d_n4;
        var_vl__blk146_dn5 = assign4910_e5230_d_n5;
        var_vl__blk146_dn6 = assign4910_e5230_d_n6;
        var_vl__blk146_dn7 = assign4910_e5230_d_n7;
        var_vl__blk146_dn8 = assign4910_e5230_d_n8;
        var_vl__blk146_dn9 = assign4910_e5230_d_n9;
        var_vl__blk146_dn10 = assign4910_e5230_d_n10;
        var_vl__blk146_dn11 = assign4910_e5230_d_n11;
        var_vl__blk146_dn12 = assign4910_e5230_d_n12;
        var_vl__blk146_dn13 = assign4910_e5230_d_n13;

        let (assign4920_e5252, assign4920_e5252_d_n0, assign4920_e5252_d_n1, assign4920_e5252_d_n2, assign4920_e5252_d_n3, assign4920_e5252_d_n4, assign4920_e5252_d_n5, assign4920_e5252_d_n6, assign4920_e5252_d_n7, assign4920_e5252_d_n8, assign4920_e5252_d_n9, assign4920_e5252_d_n10, assign4920_e5252_d_n11, assign4920_e5252_d_n12, assign4920_e5252_d_n13,) = {
    if ((var_guard135 != 0.0) && (var_guard147 == 0.0)) {
        let assign4920_e5236: f64 = (-var_ps_t);
        let assign4920_e5240: f64 = (var_vl__blk146 / var_ps_t);
        let assign4920_e5241: f64 = (1.0 - assign4920_e5240);
        let assign4920_e5244: f64 = (1.0 - p.p51);
        let assign4920_e5245: f64 = (assign4920_e5241).powf(assign4920_e5244);
        let assign4920_e5246: f64 = (assign4920_e5236 * assign4920_e5245);
        let assign4920_e5249: f64 = (1.0 - p.p51);
        let assign4920_e5250: f64 = (assign4920_e5246 / assign4920_e5249);
        (assign4920_e5250, ((((-var_ps_t_dn0) * assign4920_e5245) + (assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(((var_vl__blk146_dn0 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn0)) / (var_ps_t * var_ps_t))))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(((var_vl__blk146_dn0 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn0)) / (var_ps_t * var_ps_t))) / assign4920_e5241))) })) / assign4920_e5249), ((((-var_ps_t_dn1) * assign4920_e5245) + (assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(((var_vl__blk146_dn1 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn1)) / (var_ps_t * var_ps_t))))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(((var_vl__blk146_dn1 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn1)) / (var_ps_t * var_ps_t))) / assign4920_e5241))) })) / assign4920_e5249), ((((-var_ps_t_dn2) * assign4920_e5245) + (assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(((var_vl__blk146_dn2 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn2)) / (var_ps_t * var_ps_t))))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(((var_vl__blk146_dn2 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn2)) / (var_ps_t * var_ps_t))) / assign4920_e5241))) })) / assign4920_e5249), ((((-var_ps_t_dn3) * assign4920_e5245) + (assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(((var_vl__blk146_dn3 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn3)) / (var_ps_t * var_ps_t))))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(((var_vl__blk146_dn3 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn3)) / (var_ps_t * var_ps_t))) / assign4920_e5241))) })) / assign4920_e5249), ((((-var_ps_t_dn4) * assign4920_e5245) + (assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(((var_vl__blk146_dn4 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn4)) / (var_ps_t * var_ps_t))))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(((var_vl__blk146_dn4 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn4)) / (var_ps_t * var_ps_t))) / assign4920_e5241))) })) / assign4920_e5249), ((((-var_ps_t_dn5) * assign4920_e5245) + (assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(((var_vl__blk146_dn5 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn5)) / (var_ps_t * var_ps_t))))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(((var_vl__blk146_dn5 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn5)) / (var_ps_t * var_ps_t))) / assign4920_e5241))) })) / assign4920_e5249), ((((-var_ps_t_dn6) * assign4920_e5245) + (assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(((var_vl__blk146_dn6 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn6)) / (var_ps_t * var_ps_t))))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(((var_vl__blk146_dn6 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn6)) / (var_ps_t * var_ps_t))) / assign4920_e5241))) })) / assign4920_e5249), ((((-var_ps_t_dn7) * assign4920_e5245) + (assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(((var_vl__blk146_dn7 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn7)) / (var_ps_t * var_ps_t))))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(((var_vl__blk146_dn7 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn7)) / (var_ps_t * var_ps_t))) / assign4920_e5241))) })) / assign4920_e5249), ((((-var_ps_t_dn8) * assign4920_e5245) + (assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(((var_vl__blk146_dn8 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn8)) / (var_ps_t * var_ps_t))))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(((var_vl__blk146_dn8 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn8)) / (var_ps_t * var_ps_t))) / assign4920_e5241))) })) / assign4920_e5249), ((((-var_ps_t_dn9) * assign4920_e5245) + (assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(((var_vl__blk146_dn9 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn9)) / (var_ps_t * var_ps_t))))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(((var_vl__blk146_dn9 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn9)) / (var_ps_t * var_ps_t))) / assign4920_e5241))) })) / assign4920_e5249), ((((-var_ps_t_dn10) * assign4920_e5245) + (assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(((var_vl__blk146_dn10 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn10)) / (var_ps_t * var_ps_t))))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(((var_vl__blk146_dn10 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn10)) / (var_ps_t * var_ps_t))) / assign4920_e5241))) })) / assign4920_e5249), ((((-var_ps_t_dn11) * assign4920_e5245) + (assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(((var_vl__blk146_dn11 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn11)) / (var_ps_t * var_ps_t))))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(((var_vl__blk146_dn11 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn11)) / (var_ps_t * var_ps_t))) / assign4920_e5241))) })) / assign4920_e5249), ((((-var_ps_t_dn12) * assign4920_e5245) + (assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(((var_vl__blk146_dn12 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn12)) / (var_ps_t * var_ps_t))))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(((var_vl__blk146_dn12 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn12)) / (var_ps_t * var_ps_t))) / assign4920_e5241))) })) / assign4920_e5249), ((((-var_ps_t_dn13) * assign4920_e5245) + (assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(((var_vl__blk146_dn13 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn13)) / (var_ps_t * var_ps_t))))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(((var_vl__blk146_dn13 * var_ps_t) - (var_vl__blk146 * var_ps_t_dn13)) / (var_ps_t * var_ps_t))) / assign4920_e5241))) })) / assign4920_e5249),)
    } else {
        (var_qlo__blk139, var_qlo__blk139_dn0, var_qlo__blk139_dn1, var_qlo__blk139_dn2, var_qlo__blk139_dn3, var_qlo__blk139_dn4, var_qlo__blk139_dn5, var_qlo__blk139_dn6, var_qlo__blk139_dn7, var_qlo__blk139_dn8, var_qlo__blk139_dn9, var_qlo__blk139_dn10, var_qlo__blk139_dn11, var_qlo__blk139_dn12, var_qlo__blk139_dn13,)
    }
};
        var_qlo__blk139 = assign4920_e5252;
        var_qlo__blk139_dn0 = assign4920_e5252_d_n0;
        var_qlo__blk139_dn1 = assign4920_e5252_d_n1;
        var_qlo__blk139_dn2 = assign4920_e5252_d_n2;
        var_qlo__blk139_dn3 = assign4920_e5252_d_n3;
        var_qlo__blk139_dn4 = assign4920_e5252_d_n4;
        var_qlo__blk139_dn5 = assign4920_e5252_d_n5;
        var_qlo__blk139_dn6 = assign4920_e5252_d_n6;
        var_qlo__blk139_dn7 = assign4920_e5252_d_n7;
        var_qlo__blk139_dn8 = assign4920_e5252_d_n8;
        var_qlo__blk139_dn9 = assign4920_e5252_d_n9;
        var_qlo__blk139_dn10 = assign4920_e5252_d_n10;
        var_qlo__blk139_dn11 = assign4920_e5252_d_n11;
        var_qlo__blk139_dn12 = assign4920_e5252_d_n12;
        var_qlo__blk139_dn13 = assign4920_e5252_d_n13;

        let (assign4930_e5292, assign4930_e5292_d_n0, assign4930_e5292_d_n1, assign4930_e5292_d_n2, assign4930_e5292_d_n3, assign4930_e5292_d_n4, assign4930_e5292_d_n5, assign4930_e5292_d_n6, assign4930_e5292_d_n7, assign4930_e5292_d_n8, assign4930_e5292_d_n9, assign4930_e5292_d_n10, assign4930_e5292_d_n11, assign4930_e5292_d_n12, assign4930_e5292_d_n13,) = {
    if ((var_guard135 != 0.0) && (var_guard147 == 0.0)) {
        let assign4930_e5260: f64 = (1.0 - p.p34);
        let assign4930_e5262: f64 = (-p.p51);
        let assign4930_e5263: f64 = (assign4930_e5260).powf(assign4930_e5262);
        let assign4930_e5266: f64 = (var_vbcp - var_vl__blk146);
        let assign4930_e5268: f64 = (assign4930_e5266 + var_vl0__blk142);
        let assign4930_e5269: f64 = (assign4930_e5263 * assign4930_e5268);
        let assign4930_e5273: f64 = (0.5 * p.p51);
        let assign4930_e5276: f64 = (var_vbcp - var_vl__blk146);
        let assign4930_e5278: f64 = (assign4930_e5276 + var_vl0__blk142);
        let assign4930_e5279: f64 = (assign4930_e5273 * assign4930_e5278);
        let assign4930_e5283: f64 = (1.0 - p.p34);
        let assign4930_e5284: f64 = (var_ps_t * assign4930_e5283);
        let assign4930_e5285: f64 = (assign4930_e5279 / assign4930_e5284);
        let assign4930_e5286: f64 = (1.0 + assign4930_e5285);
        let assign4930_e5287: f64 = (assign4930_e5269 * assign4930_e5286);
        let assign4930_e5288: f64 = (var_qlo__blk139 + assign4930_e5287);
        let assign4930_e5290: f64 = (assign4930_e5288 - var_q0__blk143);
        (assign4930_e5290, ((var_qlo__blk139_dn0 + (((assign4930_e5263 * ((var_vbcp_dn0 - var_vl__blk146_dn0) + var_vl0__blk142_dn0)) * assign4930_e5286) + (assign4930_e5269 * ((((assign4930_e5273 * ((var_vbcp_dn0 - var_vl__blk146_dn0) + var_vl0__blk142_dn0)) * assign4930_e5284) - (assign4930_e5279 * (var_ps_t_dn0 * assign4930_e5283))) / (assign4930_e5284 * assign4930_e5284))))) - var_q0__blk143_dn0), ((var_qlo__blk139_dn1 + (((assign4930_e5263 * ((var_vbcp_dn1 - var_vl__blk146_dn1) + var_vl0__blk142_dn1)) * assign4930_e5286) + (assign4930_e5269 * ((((assign4930_e5273 * ((var_vbcp_dn1 - var_vl__blk146_dn1) + var_vl0__blk142_dn1)) * assign4930_e5284) - (assign4930_e5279 * (var_ps_t_dn1 * assign4930_e5283))) / (assign4930_e5284 * assign4930_e5284))))) - var_q0__blk143_dn1), ((var_qlo__blk139_dn2 + (((assign4930_e5263 * ((var_vbcp_dn2 - var_vl__blk146_dn2) + var_vl0__blk142_dn2)) * assign4930_e5286) + (assign4930_e5269 * ((((assign4930_e5273 * ((var_vbcp_dn2 - var_vl__blk146_dn2) + var_vl0__blk142_dn2)) * assign4930_e5284) - (assign4930_e5279 * (var_ps_t_dn2 * assign4930_e5283))) / (assign4930_e5284 * assign4930_e5284))))) - var_q0__blk143_dn2), ((var_qlo__blk139_dn3 + (((assign4930_e5263 * ((var_vbcp_dn3 - var_vl__blk146_dn3) + var_vl0__blk142_dn3)) * assign4930_e5286) + (assign4930_e5269 * ((((assign4930_e5273 * ((var_vbcp_dn3 - var_vl__blk146_dn3) + var_vl0__blk142_dn3)) * assign4930_e5284) - (assign4930_e5279 * (var_ps_t_dn3 * assign4930_e5283))) / (assign4930_e5284 * assign4930_e5284))))) - var_q0__blk143_dn3), ((var_qlo__blk139_dn4 + (((assign4930_e5263 * ((var_vbcp_dn4 - var_vl__blk146_dn4) + var_vl0__blk142_dn4)) * assign4930_e5286) + (assign4930_e5269 * ((((assign4930_e5273 * ((var_vbcp_dn4 - var_vl__blk146_dn4) + var_vl0__blk142_dn4)) * assign4930_e5284) - (assign4930_e5279 * (var_ps_t_dn4 * assign4930_e5283))) / (assign4930_e5284 * assign4930_e5284))))) - var_q0__blk143_dn4), ((var_qlo__blk139_dn5 + (((assign4930_e5263 * ((var_vbcp_dn5 - var_vl__blk146_dn5) + var_vl0__blk142_dn5)) * assign4930_e5286) + (assign4930_e5269 * ((((assign4930_e5273 * ((var_vbcp_dn5 - var_vl__blk146_dn5) + var_vl0__blk142_dn5)) * assign4930_e5284) - (assign4930_e5279 * (var_ps_t_dn5 * assign4930_e5283))) / (assign4930_e5284 * assign4930_e5284))))) - var_q0__blk143_dn5), ((var_qlo__blk139_dn6 + (((assign4930_e5263 * ((var_vbcp_dn6 - var_vl__blk146_dn6) + var_vl0__blk142_dn6)) * assign4930_e5286) + (assign4930_e5269 * ((((assign4930_e5273 * ((var_vbcp_dn6 - var_vl__blk146_dn6) + var_vl0__blk142_dn6)) * assign4930_e5284) - (assign4930_e5279 * (var_ps_t_dn6 * assign4930_e5283))) / (assign4930_e5284 * assign4930_e5284))))) - var_q0__blk143_dn6), ((var_qlo__blk139_dn7 + (((assign4930_e5263 * ((var_vbcp_dn7 - var_vl__blk146_dn7) + var_vl0__blk142_dn7)) * assign4930_e5286) + (assign4930_e5269 * ((((assign4930_e5273 * ((var_vbcp_dn7 - var_vl__blk146_dn7) + var_vl0__blk142_dn7)) * assign4930_e5284) - (assign4930_e5279 * (var_ps_t_dn7 * assign4930_e5283))) / (assign4930_e5284 * assign4930_e5284))))) - var_q0__blk143_dn7), ((var_qlo__blk139_dn8 + (((assign4930_e5263 * ((var_vbcp_dn8 - var_vl__blk146_dn8) + var_vl0__blk142_dn8)) * assign4930_e5286) + (assign4930_e5269 * ((((assign4930_e5273 * ((var_vbcp_dn8 - var_vl__blk146_dn8) + var_vl0__blk142_dn8)) * assign4930_e5284) - (assign4930_e5279 * (var_ps_t_dn8 * assign4930_e5283))) / (assign4930_e5284 * assign4930_e5284))))) - var_q0__blk143_dn8), ((var_qlo__blk139_dn9 + (((assign4930_e5263 * ((var_vbcp_dn9 - var_vl__blk146_dn9) + var_vl0__blk142_dn9)) * assign4930_e5286) + (assign4930_e5269 * ((((assign4930_e5273 * ((var_vbcp_dn9 - var_vl__blk146_dn9) + var_vl0__blk142_dn9)) * assign4930_e5284) - (assign4930_e5279 * (var_ps_t_dn9 * assign4930_e5283))) / (assign4930_e5284 * assign4930_e5284))))) - var_q0__blk143_dn9), ((var_qlo__blk139_dn10 + (((assign4930_e5263 * ((var_vbcp_dn10 - var_vl__blk146_dn10) + var_vl0__blk142_dn10)) * assign4930_e5286) + (assign4930_e5269 * ((((assign4930_e5273 * ((var_vbcp_dn10 - var_vl__blk146_dn10) + var_vl0__blk142_dn10)) * assign4930_e5284) - (assign4930_e5279 * (var_ps_t_dn10 * assign4930_e5283))) / (assign4930_e5284 * assign4930_e5284))))) - var_q0__blk143_dn10), ((var_qlo__blk139_dn11 + (((assign4930_e5263 * ((var_vbcp_dn11 - var_vl__blk146_dn11) + var_vl0__blk142_dn11)) * assign4930_e5286) + (assign4930_e5269 * ((((assign4930_e5273 * ((var_vbcp_dn11 - var_vl__blk146_dn11) + var_vl0__blk142_dn11)) * assign4930_e5284) - (assign4930_e5279 * (var_ps_t_dn11 * assign4930_e5283))) / (assign4930_e5284 * assign4930_e5284))))) - var_q0__blk143_dn11), ((var_qlo__blk139_dn12 + (((assign4930_e5263 * ((var_vbcp_dn12 - var_vl__blk146_dn12) + var_vl0__blk142_dn12)) * assign4930_e5286) + (assign4930_e5269 * ((((assign4930_e5273 * ((var_vbcp_dn12 - var_vl__blk146_dn12) + var_vl0__blk142_dn12)) * assign4930_e5284) - (assign4930_e5279 * (var_ps_t_dn12 * assign4930_e5283))) / (assign4930_e5284 * assign4930_e5284))))) - var_q0__blk143_dn12), ((var_qlo__blk139_dn13 + (((assign4930_e5263 * ((var_vbcp_dn13 - var_vl__blk146_dn13) + var_vl0__blk142_dn13)) * assign4930_e5286) + (assign4930_e5269 * ((((assign4930_e5273 * ((var_vbcp_dn13 - var_vl__blk146_dn13) + var_vl0__blk142_dn13)) * assign4930_e5284) - (assign4930_e5279 * (var_ps_t_dn13 * assign4930_e5283))) / (assign4930_e5284 * assign4930_e5284))))) - var_q0__blk143_dn13),)
    } else {
        (var_qdbcp, var_qdbcp_dn0, var_qdbcp_dn1, var_qdbcp_dn2, var_qdbcp_dn3, var_qdbcp_dn4, var_qdbcp_dn5, var_qdbcp_dn6, var_qdbcp_dn7, var_qdbcp_dn8, var_qdbcp_dn9, var_qdbcp_dn10, var_qdbcp_dn11, var_qdbcp_dn12, var_qdbcp_dn13,)
    }
};
        var_qdbcp = assign4930_e5292;
        var_qdbcp_dn0 = assign4930_e5292_d_n0;
        var_qdbcp_dn1 = assign4930_e5292_d_n1;
        var_qdbcp_dn2 = assign4930_e5292_d_n2;
        var_qdbcp_dn3 = assign4930_e5292_d_n3;
        var_qdbcp_dn4 = assign4930_e5292_d_n4;
        var_qdbcp_dn5 = assign4930_e5292_d_n5;
        var_qdbcp_dn6 = assign4930_e5292_d_n6;
        var_qdbcp_dn7 = assign4930_e5292_d_n7;
        var_qdbcp_dn8 = assign4930_e5292_d_n8;
        var_qdbcp_dn9 = assign4930_e5292_d_n9;
        var_qdbcp_dn10 = assign4930_e5292_d_n10;
        var_qdbcp_dn11 = assign4930_e5292_d_n11;
        var_qdbcp_dn12 = assign4930_e5292_d_n12;
        var_qdbcp_dn13 = assign4930_e5292_d_n13;

        let (assign4940_e5297, assign4940_e5297_d_n0, assign4940_e5297_d_n1, assign4940_e5297_d_n2, assign4940_e5297_d_n3, assign4940_e5297_d_n4, assign4940_e5297_d_n5, assign4940_e5297_d_n6, assign4940_e5297_d_n7, assign4940_e5297_d_n8, assign4940_e5297_d_n9, assign4940_e5297_d_n10, assign4940_e5297_d_n11, assign4940_e5297_d_n12, assign4940_e5297_d_n13,) = {
    if (var_guard135 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qdbcp, var_qdbcp_dn0, var_qdbcp_dn1, var_qdbcp_dn2, var_qdbcp_dn3, var_qdbcp_dn4, var_qdbcp_dn5, var_qdbcp_dn6, var_qdbcp_dn7, var_qdbcp_dn8, var_qdbcp_dn9, var_qdbcp_dn10, var_qdbcp_dn11, var_qdbcp_dn12, var_qdbcp_dn13,)
    }
};
        var_qdbcp = assign4940_e5297;
        var_qdbcp_dn0 = assign4940_e5297_d_n0;
        var_qdbcp_dn1 = assign4940_e5297_d_n1;
        var_qdbcp_dn2 = assign4940_e5297_d_n2;
        var_qdbcp_dn3 = assign4940_e5297_d_n3;
        var_qdbcp_dn4 = assign4940_e5297_d_n4;
        var_qdbcp_dn5 = assign4940_e5297_d_n5;
        var_qdbcp_dn6 = assign4940_e5297_d_n6;
        var_qdbcp_dn7 = assign4940_e5297_d_n7;
        var_qdbcp_dn8 = assign4940_e5297_d_n8;
        var_qdbcp_dn9 = assign4940_e5297_d_n9;
        var_qdbcp_dn10 = assign4940_e5297_d_n10;
        var_qdbcp_dn11 = assign4940_e5297_d_n11;
        var_qdbcp_dn12 = assign4940_e5297_d_n12;
        var_qdbcp_dn13 = assign4940_e5297_d_n13;

        let assign4950_e5299: f64 = (-var_pe_t);
        let assign4950_e5301: f64 = (assign4950_e5299 * p.p34);
        var_dv0__blk149 = assign4950_e5301;
        var_dv0__blk149_dn0 = ((-var_pe_t_dn0) * p.p34);
        var_dv0__blk149_dn1 = ((-var_pe_t_dn1) * p.p34);
        var_dv0__blk149_dn2 = ((-var_pe_t_dn2) * p.p34);
        var_dv0__blk149_dn3 = ((-var_pe_t_dn3) * p.p34);
        var_dv0__blk149_dn4 = ((-var_pe_t_dn4) * p.p34);
        var_dv0__blk149_dn5 = ((-var_pe_t_dn5) * p.p34);
        var_dv0__blk149_dn6 = ((-var_pe_t_dn6) * p.p34);
        var_dv0__blk149_dn7 = ((-var_pe_t_dn7) * p.p34);
        var_dv0__blk149_dn8 = ((-var_pe_t_dn8) * p.p34);
        var_dv0__blk149_dn9 = ((-var_pe_t_dn9) * p.p34);
        var_dv0__blk149_dn10 = ((-var_pe_t_dn10) * p.p34);
        var_dv0__blk149_dn11 = ((-var_pe_t_dn11) * p.p34);
        var_dv0__blk149_dn12 = ((-var_pe_t_dn12) * p.p34);
        var_dv0__blk149_dn13 = ((-var_pe_t_dn13) * p.p34);

        let assign4960_e5304: f64 = if p.p39 <= 0.0 { 1.0 } else { 0.0 };
        var_guard160 = assign4960_e5304;

        let (assign4970_e5310, assign4970_e5310_d_n0, assign4970_e5310_d_n1, assign4970_e5310_d_n2, assign4970_e5310_d_n3, assign4970_e5310_d_n4, assign4970_e5310_d_n5, assign4970_e5310_d_n6, assign4970_e5310_d_n7, assign4970_e5310_d_n8, assign4970_e5310_d_n9, assign4970_e5310_d_n10, assign4970_e5310_d_n11, assign4970_e5310_d_n12, assign4970_e5310_d_n13,) = {
    if (var_guard160 != 0.0) {
        let assign4970_e5308: f64 = (var_vbex + var_dv0__blk149);
        (assign4970_e5308, (var_vbex_dn0 + var_dv0__blk149_dn0), (var_vbex_dn1 + var_dv0__blk149_dn1), (var_vbex_dn2 + var_dv0__blk149_dn2), (var_vbex_dn3 + var_dv0__blk149_dn3), (var_vbex_dn4 + var_dv0__blk149_dn4), (var_vbex_dn5 + var_dv0__blk149_dn5), (var_vbex_dn6 + var_dv0__blk149_dn6), (var_vbex_dn7 + var_dv0__blk149_dn7), (var_vbex_dn8 + var_dv0__blk149_dn8), (var_vbex_dn9 + var_dv0__blk149_dn9), (var_vbex_dn10 + var_dv0__blk149_dn10), (var_vbex_dn11 + var_dv0__blk149_dn11), (var_vbex_dn12 + var_dv0__blk149_dn12), (var_vbex_dn13 + var_dv0__blk149_dn13),)
    } else {
        (var_dvh__blk150, var_dvh__blk150_dn0, var_dvh__blk150_dn1, var_dvh__blk150_dn2, var_dvh__blk150_dn3, var_dvh__blk150_dn4, var_dvh__blk150_dn5, var_dvh__blk150_dn6, var_dvh__blk150_dn7, var_dvh__blk150_dn8, var_dvh__blk150_dn9, var_dvh__blk150_dn10, var_dvh__blk150_dn11, var_dvh__blk150_dn12, var_dvh__blk150_dn13,)
    }
};
        var_dvh__blk150 = assign4970_e5310;
        var_dvh__blk150_dn0 = assign4970_e5310_d_n0;
        var_dvh__blk150_dn1 = assign4970_e5310_d_n1;
        var_dvh__blk150_dn2 = assign4970_e5310_d_n2;
        var_dvh__blk150_dn3 = assign4970_e5310_d_n3;
        var_dvh__blk150_dn4 = assign4970_e5310_d_n4;
        var_dvh__blk150_dn5 = assign4970_e5310_d_n5;
        var_dvh__blk150_dn6 = assign4970_e5310_d_n6;
        var_dvh__blk150_dn7 = assign4970_e5310_d_n7;
        var_dvh__blk150_dn8 = assign4970_e5310_d_n8;
        var_dvh__blk150_dn9 = assign4970_e5310_d_n9;
        var_dvh__blk150_dn10 = assign4970_e5310_d_n10;
        var_dvh__blk150_dn11 = assign4970_e5310_d_n11;
        var_dvh__blk150_dn12 = assign4970_e5310_d_n12;
        var_dvh__blk150_dn13 = assign4970_e5310_d_n13;

        let assign4980_e5313: f64 = if var_dvh__blk150 > 0.0 { 1.0 } else { 0.0 };
        var_guard161 = assign4980_e5313;

        let (assign4990_e5324,) = {
    if ((var_guard160 != 0.0) && (var_guard161 != 0.0)) {
        let assign4990_e5319: f64 = (1.0 - p.p34);
        let assign4990_e5321: f64 = (-p.p38);
        let assign4990_e5322: f64 = (assign4990_e5319).powf(assign4990_e5321);
        (assign4990_e5322,)
    } else {
        (var_pwq__blk151,)
    }
};
        var_pwq__blk151 = assign4990_e5324;

        let (assign5000_e5342, assign5000_e5342_d_n0, assign5000_e5342_d_n1, assign5000_e5342_d_n2, assign5000_e5342_d_n3, assign5000_e5342_d_n4, assign5000_e5342_d_n5, assign5000_e5342_d_n6, assign5000_e5342_d_n7, assign5000_e5342_d_n8, assign5000_e5342_d_n9, assign5000_e5342_d_n10, assign5000_e5342_d_n11, assign5000_e5342_d_n12, assign5000_e5342_d_n13,) = {
    if ((var_guard160 != 0.0) && (var_guard161 != 0.0)) {
        let assign5000_e5333: f64 = (1.0 - p.p34);
        let assign5000_e5334: f64 = (var_pwq__blk151 * assign5000_e5333);
        let assign5000_e5335: f64 = (1.0 - assign5000_e5334);
        let assign5000_e5336: f64 = (var_pe_t * assign5000_e5335);
        let assign5000_e5339: f64 = (1.0 - p.p38);
        let assign5000_e5340: f64 = (assign5000_e5336 / assign5000_e5339);
        (assign5000_e5340, ((var_pe_t_dn0 * assign5000_e5335) / assign5000_e5339), ((var_pe_t_dn1 * assign5000_e5335) / assign5000_e5339), ((var_pe_t_dn2 * assign5000_e5335) / assign5000_e5339), ((var_pe_t_dn3 * assign5000_e5335) / assign5000_e5339), ((var_pe_t_dn4 * assign5000_e5335) / assign5000_e5339), ((var_pe_t_dn5 * assign5000_e5335) / assign5000_e5339), ((var_pe_t_dn6 * assign5000_e5335) / assign5000_e5339), ((var_pe_t_dn7 * assign5000_e5335) / assign5000_e5339), ((var_pe_t_dn8 * assign5000_e5335) / assign5000_e5339), ((var_pe_t_dn9 * assign5000_e5335) / assign5000_e5339), ((var_pe_t_dn10 * assign5000_e5335) / assign5000_e5339), ((var_pe_t_dn11 * assign5000_e5335) / assign5000_e5339), ((var_pe_t_dn12 * assign5000_e5335) / assign5000_e5339), ((var_pe_t_dn13 * assign5000_e5335) / assign5000_e5339),)
    } else {
        (var_qlo__blk152, var_qlo__blk152_dn0, var_qlo__blk152_dn1, var_qlo__blk152_dn2, var_qlo__blk152_dn3, var_qlo__blk152_dn4, var_qlo__blk152_dn5, var_qlo__blk152_dn6, var_qlo__blk152_dn7, var_qlo__blk152_dn8, var_qlo__blk152_dn9, var_qlo__blk152_dn10, var_qlo__blk152_dn11, var_qlo__blk152_dn12, var_qlo__blk152_dn13,)
    }
};
        var_qlo__blk152 = assign5000_e5342;
        var_qlo__blk152_dn0 = assign5000_e5342_d_n0;
        var_qlo__blk152_dn1 = assign5000_e5342_d_n1;
        var_qlo__blk152_dn2 = assign5000_e5342_d_n2;
        var_qlo__blk152_dn3 = assign5000_e5342_d_n3;
        var_qlo__blk152_dn4 = assign5000_e5342_d_n4;
        var_qlo__blk152_dn5 = assign5000_e5342_d_n5;
        var_qlo__blk152_dn6 = assign5000_e5342_d_n6;
        var_qlo__blk152_dn7 = assign5000_e5342_d_n7;
        var_qlo__blk152_dn8 = assign5000_e5342_d_n8;
        var_qlo__blk152_dn9 = assign5000_e5342_d_n9;
        var_qlo__blk152_dn10 = assign5000_e5342_d_n10;
        var_qlo__blk152_dn11 = assign5000_e5342_d_n11;
        var_qlo__blk152_dn12 = assign5000_e5342_d_n12;
        var_qlo__blk152_dn13 = assign5000_e5342_d_n13;

        let (assign5010_e5364, assign5010_e5364_d_n0, assign5010_e5364_d_n1, assign5010_e5364_d_n2, assign5010_e5364_d_n3, assign5010_e5364_d_n4, assign5010_e5364_d_n5, assign5010_e5364_d_n6, assign5010_e5364_d_n7, assign5010_e5364_d_n8, assign5010_e5364_d_n9, assign5010_e5364_d_n10, assign5010_e5364_d_n11, assign5010_e5364_d_n12, assign5010_e5364_d_n13,) = {
    if ((var_guard160 != 0.0) && (var_guard161 != 0.0)) {
        let assign5010_e5350: f64 = (0.5 * p.p38);
        let assign5010_e5352: f64 = (assign5010_e5350 * var_dvh__blk150);
        let assign5010_e5356: f64 = (1.0 - p.p34);
        let assign5010_e5357: f64 = (var_pe_t * assign5010_e5356);
        let assign5010_e5358: f64 = (assign5010_e5352 / assign5010_e5357);
        let assign5010_e5359: f64 = (1.0 + assign5010_e5358);
        let assign5010_e5360: f64 = (var_dvh__blk150 * assign5010_e5359);
        let assign5010_e5362: f64 = (assign5010_e5360 * var_pwq__blk151);
        (assign5010_e5362, (((var_dvh__blk150_dn0 * assign5010_e5359) + (var_dvh__blk150 * ((((assign5010_e5350 * var_dvh__blk150_dn0) * assign5010_e5357) - (assign5010_e5352 * (var_pe_t_dn0 * assign5010_e5356))) / (assign5010_e5357 * assign5010_e5357)))) * var_pwq__blk151), (((var_dvh__blk150_dn1 * assign5010_e5359) + (var_dvh__blk150 * ((((assign5010_e5350 * var_dvh__blk150_dn1) * assign5010_e5357) - (assign5010_e5352 * (var_pe_t_dn1 * assign5010_e5356))) / (assign5010_e5357 * assign5010_e5357)))) * var_pwq__blk151), (((var_dvh__blk150_dn2 * assign5010_e5359) + (var_dvh__blk150 * ((((assign5010_e5350 * var_dvh__blk150_dn2) * assign5010_e5357) - (assign5010_e5352 * (var_pe_t_dn2 * assign5010_e5356))) / (assign5010_e5357 * assign5010_e5357)))) * var_pwq__blk151), (((var_dvh__blk150_dn3 * assign5010_e5359) + (var_dvh__blk150 * ((((assign5010_e5350 * var_dvh__blk150_dn3) * assign5010_e5357) - (assign5010_e5352 * (var_pe_t_dn3 * assign5010_e5356))) / (assign5010_e5357 * assign5010_e5357)))) * var_pwq__blk151), (((var_dvh__blk150_dn4 * assign5010_e5359) + (var_dvh__blk150 * ((((assign5010_e5350 * var_dvh__blk150_dn4) * assign5010_e5357) - (assign5010_e5352 * (var_pe_t_dn4 * assign5010_e5356))) / (assign5010_e5357 * assign5010_e5357)))) * var_pwq__blk151), (((var_dvh__blk150_dn5 * assign5010_e5359) + (var_dvh__blk150 * ((((assign5010_e5350 * var_dvh__blk150_dn5) * assign5010_e5357) - (assign5010_e5352 * (var_pe_t_dn5 * assign5010_e5356))) / (assign5010_e5357 * assign5010_e5357)))) * var_pwq__blk151), (((var_dvh__blk150_dn6 * assign5010_e5359) + (var_dvh__blk150 * ((((assign5010_e5350 * var_dvh__blk150_dn6) * assign5010_e5357) - (assign5010_e5352 * (var_pe_t_dn6 * assign5010_e5356))) / (assign5010_e5357 * assign5010_e5357)))) * var_pwq__blk151), (((var_dvh__blk150_dn7 * assign5010_e5359) + (var_dvh__blk150 * ((((assign5010_e5350 * var_dvh__blk150_dn7) * assign5010_e5357) - (assign5010_e5352 * (var_pe_t_dn7 * assign5010_e5356))) / (assign5010_e5357 * assign5010_e5357)))) * var_pwq__blk151), (((var_dvh__blk150_dn8 * assign5010_e5359) + (var_dvh__blk150 * ((((assign5010_e5350 * var_dvh__blk150_dn8) * assign5010_e5357) - (assign5010_e5352 * (var_pe_t_dn8 * assign5010_e5356))) / (assign5010_e5357 * assign5010_e5357)))) * var_pwq__blk151), (((var_dvh__blk150_dn9 * assign5010_e5359) + (var_dvh__blk150 * ((((assign5010_e5350 * var_dvh__blk150_dn9) * assign5010_e5357) - (assign5010_e5352 * (var_pe_t_dn9 * assign5010_e5356))) / (assign5010_e5357 * assign5010_e5357)))) * var_pwq__blk151), (((var_dvh__blk150_dn10 * assign5010_e5359) + (var_dvh__blk150 * ((((assign5010_e5350 * var_dvh__blk150_dn10) * assign5010_e5357) - (assign5010_e5352 * (var_pe_t_dn10 * assign5010_e5356))) / (assign5010_e5357 * assign5010_e5357)))) * var_pwq__blk151), (((var_dvh__blk150_dn11 * assign5010_e5359) + (var_dvh__blk150 * ((((assign5010_e5350 * var_dvh__blk150_dn11) * assign5010_e5357) - (assign5010_e5352 * (var_pe_t_dn11 * assign5010_e5356))) / (assign5010_e5357 * assign5010_e5357)))) * var_pwq__blk151), (((var_dvh__blk150_dn12 * assign5010_e5359) + (var_dvh__blk150 * ((((assign5010_e5350 * var_dvh__blk150_dn12) * assign5010_e5357) - (assign5010_e5352 * (var_pe_t_dn12 * assign5010_e5356))) / (assign5010_e5357 * assign5010_e5357)))) * var_pwq__blk151), (((var_dvh__blk150_dn13 * assign5010_e5359) + (var_dvh__blk150 * ((((assign5010_e5350 * var_dvh__blk150_dn13) * assign5010_e5357) - (assign5010_e5352 * (var_pe_t_dn13 * assign5010_e5356))) / (assign5010_e5357 * assign5010_e5357)))) * var_pwq__blk151),)
    } else {
        (var_qhi__blk153, var_qhi__blk153_dn0, var_qhi__blk153_dn1, var_qhi__blk153_dn2, var_qhi__blk153_dn3, var_qhi__blk153_dn4, var_qhi__blk153_dn5, var_qhi__blk153_dn6, var_qhi__blk153_dn7, var_qhi__blk153_dn8, var_qhi__blk153_dn9, var_qhi__blk153_dn10, var_qhi__blk153_dn11, var_qhi__blk153_dn12, var_qhi__blk153_dn13,)
    }
};
        var_qhi__blk153 = assign5010_e5364;
        var_qhi__blk153_dn0 = assign5010_e5364_d_n0;
        var_qhi__blk153_dn1 = assign5010_e5364_d_n1;
        var_qhi__blk153_dn2 = assign5010_e5364_d_n2;
        var_qhi__blk153_dn3 = assign5010_e5364_d_n3;
        var_qhi__blk153_dn4 = assign5010_e5364_d_n4;
        var_qhi__blk153_dn5 = assign5010_e5364_d_n5;
        var_qhi__blk153_dn6 = assign5010_e5364_d_n6;
        var_qhi__blk153_dn7 = assign5010_e5364_d_n7;
        var_qhi__blk153_dn8 = assign5010_e5364_d_n8;
        var_qhi__blk153_dn9 = assign5010_e5364_d_n9;
        var_qhi__blk153_dn10 = assign5010_e5364_d_n10;
        var_qhi__blk153_dn11 = assign5010_e5364_d_n11;
        var_qhi__blk153_dn12 = assign5010_e5364_d_n12;
        var_qhi__blk153_dn13 = assign5010_e5364_d_n13;

        let (assign5020_e5387, assign5020_e5387_d_n0, assign5020_e5387_d_n1, assign5020_e5387_d_n2, assign5020_e5387_d_n3, assign5020_e5387_d_n4, assign5020_e5387_d_n5, assign5020_e5387_d_n6, assign5020_e5387_d_n7, assign5020_e5387_d_n8, assign5020_e5387_d_n9, assign5020_e5387_d_n10, assign5020_e5387_d_n11, assign5020_e5387_d_n12, assign5020_e5387_d_n13,) = {
    if ((var_guard160 != 0.0) && (var_guard161 == 0.0)) {
        let assign5020_e5374: f64 = (var_vbex / var_pe_t);
        let assign5020_e5375: f64 = (1.0 - assign5020_e5374);
        let assign5020_e5378: f64 = (1.0 - p.p38);
        let assign5020_e5379: f64 = (assign5020_e5375).powf(assign5020_e5378);
        let assign5020_e5380: f64 = (1.0 - assign5020_e5379);
        let assign5020_e5381: f64 = (var_pe_t * assign5020_e5380);
        let assign5020_e5384: f64 = (1.0 - p.p38);
        let assign5020_e5385: f64 = (assign5020_e5381 / assign5020_e5384);
        (assign5020_e5385, (((var_pe_t_dn0 * assign5020_e5380) + (var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(((var_vbex_dn0 * var_pe_t) - (var_vbex * var_pe_t_dn0)) / (var_pe_t * var_pe_t))))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(((var_vbex_dn0 * var_pe_t) - (var_vbex * var_pe_t_dn0)) / (var_pe_t * var_pe_t))) / assign5020_e5375))) }))) / assign5020_e5384), (((var_pe_t_dn1 * assign5020_e5380) + (var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(((var_vbex_dn1 * var_pe_t) - (var_vbex * var_pe_t_dn1)) / (var_pe_t * var_pe_t))))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(((var_vbex_dn1 * var_pe_t) - (var_vbex * var_pe_t_dn1)) / (var_pe_t * var_pe_t))) / assign5020_e5375))) }))) / assign5020_e5384), (((var_pe_t_dn2 * assign5020_e5380) + (var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(((var_vbex_dn2 * var_pe_t) - (var_vbex * var_pe_t_dn2)) / (var_pe_t * var_pe_t))))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(((var_vbex_dn2 * var_pe_t) - (var_vbex * var_pe_t_dn2)) / (var_pe_t * var_pe_t))) / assign5020_e5375))) }))) / assign5020_e5384), (((var_pe_t_dn3 * assign5020_e5380) + (var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(((var_vbex_dn3 * var_pe_t) - (var_vbex * var_pe_t_dn3)) / (var_pe_t * var_pe_t))))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(((var_vbex_dn3 * var_pe_t) - (var_vbex * var_pe_t_dn3)) / (var_pe_t * var_pe_t))) / assign5020_e5375))) }))) / assign5020_e5384), (((var_pe_t_dn4 * assign5020_e5380) + (var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(((var_vbex_dn4 * var_pe_t) - (var_vbex * var_pe_t_dn4)) / (var_pe_t * var_pe_t))))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(((var_vbex_dn4 * var_pe_t) - (var_vbex * var_pe_t_dn4)) / (var_pe_t * var_pe_t))) / assign5020_e5375))) }))) / assign5020_e5384), (((var_pe_t_dn5 * assign5020_e5380) + (var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(((var_vbex_dn5 * var_pe_t) - (var_vbex * var_pe_t_dn5)) / (var_pe_t * var_pe_t))))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(((var_vbex_dn5 * var_pe_t) - (var_vbex * var_pe_t_dn5)) / (var_pe_t * var_pe_t))) / assign5020_e5375))) }))) / assign5020_e5384), (((var_pe_t_dn6 * assign5020_e5380) + (var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(((var_vbex_dn6 * var_pe_t) - (var_vbex * var_pe_t_dn6)) / (var_pe_t * var_pe_t))))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(((var_vbex_dn6 * var_pe_t) - (var_vbex * var_pe_t_dn6)) / (var_pe_t * var_pe_t))) / assign5020_e5375))) }))) / assign5020_e5384), (((var_pe_t_dn7 * assign5020_e5380) + (var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(((var_vbex_dn7 * var_pe_t) - (var_vbex * var_pe_t_dn7)) / (var_pe_t * var_pe_t))))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(((var_vbex_dn7 * var_pe_t) - (var_vbex * var_pe_t_dn7)) / (var_pe_t * var_pe_t))) / assign5020_e5375))) }))) / assign5020_e5384), (((var_pe_t_dn8 * assign5020_e5380) + (var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(((var_vbex_dn8 * var_pe_t) - (var_vbex * var_pe_t_dn8)) / (var_pe_t * var_pe_t))))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(((var_vbex_dn8 * var_pe_t) - (var_vbex * var_pe_t_dn8)) / (var_pe_t * var_pe_t))) / assign5020_e5375))) }))) / assign5020_e5384), (((var_pe_t_dn9 * assign5020_e5380) + (var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(((var_vbex_dn9 * var_pe_t) - (var_vbex * var_pe_t_dn9)) / (var_pe_t * var_pe_t))))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(((var_vbex_dn9 * var_pe_t) - (var_vbex * var_pe_t_dn9)) / (var_pe_t * var_pe_t))) / assign5020_e5375))) }))) / assign5020_e5384), (((var_pe_t_dn10 * assign5020_e5380) + (var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(((var_vbex_dn10 * var_pe_t) - (var_vbex * var_pe_t_dn10)) / (var_pe_t * var_pe_t))))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(((var_vbex_dn10 * var_pe_t) - (var_vbex * var_pe_t_dn10)) / (var_pe_t * var_pe_t))) / assign5020_e5375))) }))) / assign5020_e5384), (((var_pe_t_dn11 * assign5020_e5380) + (var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(((var_vbex_dn11 * var_pe_t) - (var_vbex * var_pe_t_dn11)) / (var_pe_t * var_pe_t))))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(((var_vbex_dn11 * var_pe_t) - (var_vbex * var_pe_t_dn11)) / (var_pe_t * var_pe_t))) / assign5020_e5375))) }))) / assign5020_e5384), (((var_pe_t_dn12 * assign5020_e5380) + (var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(((var_vbex_dn12 * var_pe_t) - (var_vbex * var_pe_t_dn12)) / (var_pe_t * var_pe_t))))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(((var_vbex_dn12 * var_pe_t) - (var_vbex * var_pe_t_dn12)) / (var_pe_t * var_pe_t))) / assign5020_e5375))) }))) / assign5020_e5384), (((var_pe_t_dn13 * assign5020_e5380) + (var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(((var_vbex_dn13 * var_pe_t) - (var_vbex * var_pe_t_dn13)) / (var_pe_t * var_pe_t))))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(((var_vbex_dn13 * var_pe_t) - (var_vbex * var_pe_t_dn13)) / (var_pe_t * var_pe_t))) / assign5020_e5375))) }))) / assign5020_e5384),)
    } else {
        (var_qlo__blk152, var_qlo__blk152_dn0, var_qlo__blk152_dn1, var_qlo__blk152_dn2, var_qlo__blk152_dn3, var_qlo__blk152_dn4, var_qlo__blk152_dn5, var_qlo__blk152_dn6, var_qlo__blk152_dn7, var_qlo__blk152_dn8, var_qlo__blk152_dn9, var_qlo__blk152_dn10, var_qlo__blk152_dn11, var_qlo__blk152_dn12, var_qlo__blk152_dn13,)
    }
};
        var_qlo__blk152 = assign5020_e5387;
        var_qlo__blk152_dn0 = assign5020_e5387_d_n0;
        var_qlo__blk152_dn1 = assign5020_e5387_d_n1;
        var_qlo__blk152_dn2 = assign5020_e5387_d_n2;
        var_qlo__blk152_dn3 = assign5020_e5387_d_n3;
        var_qlo__blk152_dn4 = assign5020_e5387_d_n4;
        var_qlo__blk152_dn5 = assign5020_e5387_d_n5;
        var_qlo__blk152_dn6 = assign5020_e5387_d_n6;
        var_qlo__blk152_dn7 = assign5020_e5387_d_n7;
        var_qlo__blk152_dn8 = assign5020_e5387_d_n8;
        var_qlo__blk152_dn9 = assign5020_e5387_d_n9;
        var_qlo__blk152_dn10 = assign5020_e5387_d_n10;
        var_qlo__blk152_dn11 = assign5020_e5387_d_n11;
        var_qlo__blk152_dn12 = assign5020_e5387_d_n12;
        var_qlo__blk152_dn13 = assign5020_e5387_d_n13;

        let (assign5030_e5394, assign5030_e5394_d_n0, assign5030_e5394_d_n1, assign5030_e5394_d_n2, assign5030_e5394_d_n3, assign5030_e5394_d_n4, assign5030_e5394_d_n5, assign5030_e5394_d_n6, assign5030_e5394_d_n7, assign5030_e5394_d_n8, assign5030_e5394_d_n9, assign5030_e5394_d_n10, assign5030_e5394_d_n11, assign5030_e5394_d_n12, assign5030_e5394_d_n13,) = {
    if ((var_guard160 != 0.0) && (var_guard161 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi__blk153, var_qhi__blk153_dn0, var_qhi__blk153_dn1, var_qhi__blk153_dn2, var_qhi__blk153_dn3, var_qhi__blk153_dn4, var_qhi__blk153_dn5, var_qhi__blk153_dn6, var_qhi__blk153_dn7, var_qhi__blk153_dn8, var_qhi__blk153_dn9, var_qhi__blk153_dn10, var_qhi__blk153_dn11, var_qhi__blk153_dn12, var_qhi__blk153_dn13,)
    }
};
        var_qhi__blk153 = assign5030_e5394;
        var_qhi__blk153_dn0 = assign5030_e5394_d_n0;
        var_qhi__blk153_dn1 = assign5030_e5394_d_n1;
        var_qhi__blk153_dn2 = assign5030_e5394_d_n2;
        var_qhi__blk153_dn3 = assign5030_e5394_d_n3;
        var_qhi__blk153_dn4 = assign5030_e5394_d_n4;
        var_qhi__blk153_dn5 = assign5030_e5394_d_n5;
        var_qhi__blk153_dn6 = assign5030_e5394_d_n6;
        var_qhi__blk153_dn7 = assign5030_e5394_d_n7;
        var_qhi__blk153_dn8 = assign5030_e5394_d_n8;
        var_qhi__blk153_dn9 = assign5030_e5394_d_n9;
        var_qhi__blk153_dn10 = assign5030_e5394_d_n10;
        var_qhi__blk153_dn11 = assign5030_e5394_d_n11;
        var_qhi__blk153_dn12 = assign5030_e5394_d_n12;
        var_qhi__blk153_dn13 = assign5030_e5394_d_n13;

        let (assign5040_e5400, assign5040_e5400_d_n0, assign5040_e5400_d_n1, assign5040_e5400_d_n2, assign5040_e5400_d_n3, assign5040_e5400_d_n4, assign5040_e5400_d_n5, assign5040_e5400_d_n6, assign5040_e5400_d_n7, assign5040_e5400_d_n8, assign5040_e5400_d_n9, assign5040_e5400_d_n10, assign5040_e5400_d_n11, assign5040_e5400_d_n12, assign5040_e5400_d_n13,) = {
    if (var_guard160 != 0.0) {
        let assign5040_e5398: f64 = (var_qlo__blk152 + var_qhi__blk153);
        (assign5040_e5398, (var_qlo__blk152_dn0 + var_qhi__blk153_dn0), (var_qlo__blk152_dn1 + var_qhi__blk153_dn1), (var_qlo__blk152_dn2 + var_qhi__blk153_dn2), (var_qlo__blk152_dn3 + var_qhi__blk153_dn3), (var_qlo__blk152_dn4 + var_qhi__blk153_dn4), (var_qlo__blk152_dn5 + var_qhi__blk153_dn5), (var_qlo__blk152_dn6 + var_qhi__blk153_dn6), (var_qlo__blk152_dn7 + var_qhi__blk153_dn7), (var_qlo__blk152_dn8 + var_qhi__blk153_dn8), (var_qlo__blk152_dn9 + var_qhi__blk153_dn9), (var_qlo__blk152_dn10 + var_qhi__blk153_dn10), (var_qlo__blk152_dn11 + var_qhi__blk153_dn11), (var_qlo__blk152_dn12 + var_qhi__blk153_dn12), (var_qlo__blk152_dn13 + var_qhi__blk153_dn13),)
    } else {
        (var_qdbex, var_qdbex_dn0, var_qdbex_dn1, var_qdbex_dn2, var_qdbex_dn3, var_qdbex_dn4, var_qdbex_dn5, var_qdbex_dn6, var_qdbex_dn7, var_qdbex_dn8, var_qdbex_dn9, var_qdbex_dn10, var_qdbex_dn11, var_qdbex_dn12, var_qdbex_dn13,)
    }
};
        var_qdbex = assign5040_e5400;
        var_qdbex_dn0 = assign5040_e5400_d_n0;
        var_qdbex_dn1 = assign5040_e5400_d_n1;
        var_qdbex_dn2 = assign5040_e5400_d_n2;
        var_qdbex_dn3 = assign5040_e5400_d_n3;
        var_qdbex_dn4 = assign5040_e5400_d_n4;
        var_qdbex_dn5 = assign5040_e5400_d_n5;
        var_qdbex_dn6 = assign5040_e5400_d_n6;
        var_qdbex_dn7 = assign5040_e5400_d_n7;
        var_qdbex_dn8 = assign5040_e5400_d_n8;
        var_qdbex_dn9 = assign5040_e5400_d_n9;
        var_qdbex_dn10 = assign5040_e5400_d_n10;
        var_qdbex_dn11 = assign5040_e5400_d_n11;
        var_qdbex_dn12 = assign5040_e5400_d_n12;
        var_qdbex_dn13 = assign5040_e5400_d_n13;

        let (assign5050_e5414, assign5050_e5414_d_n0, assign5050_e5414_d_n1, assign5050_e5414_d_n2, assign5050_e5414_d_n3, assign5050_e5414_d_n4, assign5050_e5414_d_n5, assign5050_e5414_d_n6, assign5050_e5414_d_n7, assign5050_e5414_d_n8, assign5050_e5414_d_n9, assign5050_e5414_d_n10, assign5050_e5414_d_n11, assign5050_e5414_d_n12, assign5050_e5414_d_n13,) = {
    if (var_guard160 == 0.0) {
        let assign5050_e5405: f64 = (var_dv0__blk149 * var_dv0__blk149);
        let assign5050_e5408: f64 = (4.0 * p.p39);
        let assign5050_e5410: f64 = (assign5050_e5408 * p.p39);
        let assign5050_e5411: f64 = (assign5050_e5405 + assign5050_e5410);
        let assign5050_e5412: f64 = (assign5050_e5411).sqrt();
        (assign5050_e5412, (((var_dv0__blk149_dn0 * var_dv0__blk149) + (var_dv0__blk149 * var_dv0__blk149_dn0)) / (2.0 * assign5050_e5412)), (((var_dv0__blk149_dn1 * var_dv0__blk149) + (var_dv0__blk149 * var_dv0__blk149_dn1)) / (2.0 * assign5050_e5412)), (((var_dv0__blk149_dn2 * var_dv0__blk149) + (var_dv0__blk149 * var_dv0__blk149_dn2)) / (2.0 * assign5050_e5412)), (((var_dv0__blk149_dn3 * var_dv0__blk149) + (var_dv0__blk149 * var_dv0__blk149_dn3)) / (2.0 * assign5050_e5412)), (((var_dv0__blk149_dn4 * var_dv0__blk149) + (var_dv0__blk149 * var_dv0__blk149_dn4)) / (2.0 * assign5050_e5412)), (((var_dv0__blk149_dn5 * var_dv0__blk149) + (var_dv0__blk149 * var_dv0__blk149_dn5)) / (2.0 * assign5050_e5412)), (((var_dv0__blk149_dn6 * var_dv0__blk149) + (var_dv0__blk149 * var_dv0__blk149_dn6)) / (2.0 * assign5050_e5412)), (((var_dv0__blk149_dn7 * var_dv0__blk149) + (var_dv0__blk149 * var_dv0__blk149_dn7)) / (2.0 * assign5050_e5412)), (((var_dv0__blk149_dn8 * var_dv0__blk149) + (var_dv0__blk149 * var_dv0__blk149_dn8)) / (2.0 * assign5050_e5412)), (((var_dv0__blk149_dn9 * var_dv0__blk149) + (var_dv0__blk149 * var_dv0__blk149_dn9)) / (2.0 * assign5050_e5412)), (((var_dv0__blk149_dn10 * var_dv0__blk149) + (var_dv0__blk149 * var_dv0__blk149_dn10)) / (2.0 * assign5050_e5412)), (((var_dv0__blk149_dn11 * var_dv0__blk149) + (var_dv0__blk149 * var_dv0__blk149_dn11)) / (2.0 * assign5050_e5412)), (((var_dv0__blk149_dn12 * var_dv0__blk149) + (var_dv0__blk149 * var_dv0__blk149_dn12)) / (2.0 * assign5050_e5412)), (((var_dv0__blk149_dn13 * var_dv0__blk149) + (var_dv0__blk149 * var_dv0__blk149_dn13)) / (2.0 * assign5050_e5412)),)
    } else {
        (var_mv0__blk154, var_mv0__blk154_dn0, var_mv0__blk154_dn1, var_mv0__blk154_dn2, var_mv0__blk154_dn3, var_mv0__blk154_dn4, var_mv0__blk154_dn5, var_mv0__blk154_dn6, var_mv0__blk154_dn7, var_mv0__blk154_dn8, var_mv0__blk154_dn9, var_mv0__blk154_dn10, var_mv0__blk154_dn11, var_mv0__blk154_dn12, var_mv0__blk154_dn13,)
    }
};
        var_mv0__blk154 = assign5050_e5414;
        var_mv0__blk154_dn0 = assign5050_e5414_d_n0;
        var_mv0__blk154_dn1 = assign5050_e5414_d_n1;
        var_mv0__blk154_dn2 = assign5050_e5414_d_n2;
        var_mv0__blk154_dn3 = assign5050_e5414_d_n3;
        var_mv0__blk154_dn4 = assign5050_e5414_d_n4;
        var_mv0__blk154_dn5 = assign5050_e5414_d_n5;
        var_mv0__blk154_dn6 = assign5050_e5414_d_n6;
        var_mv0__blk154_dn7 = assign5050_e5414_d_n7;
        var_mv0__blk154_dn8 = assign5050_e5414_d_n8;
        var_mv0__blk154_dn9 = assign5050_e5414_d_n9;
        var_mv0__blk154_dn10 = assign5050_e5414_d_n10;
        var_mv0__blk154_dn11 = assign5050_e5414_d_n11;
        var_mv0__blk154_dn12 = assign5050_e5414_d_n12;
        var_mv0__blk154_dn13 = assign5050_e5414_d_n13;

        let (assign5060_e5424, assign5060_e5424_d_n0, assign5060_e5424_d_n1, assign5060_e5424_d_n2, assign5060_e5424_d_n3, assign5060_e5424_d_n4, assign5060_e5424_d_n5, assign5060_e5424_d_n6, assign5060_e5424_d_n7, assign5060_e5424_d_n8, assign5060_e5424_d_n9, assign5060_e5424_d_n10, assign5060_e5424_d_n11, assign5060_e5424_d_n12, assign5060_e5424_d_n13,) = {
    if (var_guard160 == 0.0) {
        let assign5060_e5418: f64 = (-0.5);
        let assign5060_e5421: f64 = (var_dv0__blk149 + var_mv0__blk154);
        let assign5060_e5422: f64 = (assign5060_e5418 * assign5060_e5421);
        (assign5060_e5422, (assign5060_e5418 * (var_dv0__blk149_dn0 + var_mv0__blk154_dn0)), (assign5060_e5418 * (var_dv0__blk149_dn1 + var_mv0__blk154_dn1)), (assign5060_e5418 * (var_dv0__blk149_dn2 + var_mv0__blk154_dn2)), (assign5060_e5418 * (var_dv0__blk149_dn3 + var_mv0__blk154_dn3)), (assign5060_e5418 * (var_dv0__blk149_dn4 + var_mv0__blk154_dn4)), (assign5060_e5418 * (var_dv0__blk149_dn5 + var_mv0__blk154_dn5)), (assign5060_e5418 * (var_dv0__blk149_dn6 + var_mv0__blk154_dn6)), (assign5060_e5418 * (var_dv0__blk149_dn7 + var_mv0__blk154_dn7)), (assign5060_e5418 * (var_dv0__blk149_dn8 + var_mv0__blk154_dn8)), (assign5060_e5418 * (var_dv0__blk149_dn9 + var_mv0__blk154_dn9)), (assign5060_e5418 * (var_dv0__blk149_dn10 + var_mv0__blk154_dn10)), (assign5060_e5418 * (var_dv0__blk149_dn11 + var_mv0__blk154_dn11)), (assign5060_e5418 * (var_dv0__blk149_dn12 + var_mv0__blk154_dn12)), (assign5060_e5418 * (var_dv0__blk149_dn13 + var_mv0__blk154_dn13)),)
    } else {
        (var_vl0__blk155, var_vl0__blk155_dn0, var_vl0__blk155_dn1, var_vl0__blk155_dn2, var_vl0__blk155_dn3, var_vl0__blk155_dn4, var_vl0__blk155_dn5, var_vl0__blk155_dn6, var_vl0__blk155_dn7, var_vl0__blk155_dn8, var_vl0__blk155_dn9, var_vl0__blk155_dn10, var_vl0__blk155_dn11, var_vl0__blk155_dn12, var_vl0__blk155_dn13,)
    }
};
        var_vl0__blk155 = assign5060_e5424;
        var_vl0__blk155_dn0 = assign5060_e5424_d_n0;
        var_vl0__blk155_dn1 = assign5060_e5424_d_n1;
        var_vl0__blk155_dn2 = assign5060_e5424_d_n2;
        var_vl0__blk155_dn3 = assign5060_e5424_d_n3;
        var_vl0__blk155_dn4 = assign5060_e5424_d_n4;
        var_vl0__blk155_dn5 = assign5060_e5424_d_n5;
        var_vl0__blk155_dn6 = assign5060_e5424_d_n6;
        var_vl0__blk155_dn7 = assign5060_e5424_d_n7;
        var_vl0__blk155_dn8 = assign5060_e5424_d_n8;
        var_vl0__blk155_dn9 = assign5060_e5424_d_n9;
        var_vl0__blk155_dn10 = assign5060_e5424_d_n10;
        var_vl0__blk155_dn11 = assign5060_e5424_d_n11;
        var_vl0__blk155_dn12 = assign5060_e5424_d_n12;
        var_vl0__blk155_dn13 = assign5060_e5424_d_n13;

        let (assign5070_e5444, assign5070_e5444_d_n0, assign5070_e5444_d_n1, assign5070_e5444_d_n2, assign5070_e5444_d_n3, assign5070_e5444_d_n4, assign5070_e5444_d_n5, assign5070_e5444_d_n6, assign5070_e5444_d_n7, assign5070_e5444_d_n8, assign5070_e5444_d_n9, assign5070_e5444_d_n10, assign5070_e5444_d_n11, assign5070_e5444_d_n12, assign5070_e5444_d_n13,) = {
    if (var_guard160 == 0.0) {
        let assign5070_e5428: f64 = (-var_pe_t);
        let assign5070_e5432: f64 = (var_vl0__blk155 / var_pe_t);
        let assign5070_e5433: f64 = (1.0 - assign5070_e5432);
        let assign5070_e5436: f64 = (1.0 - p.p38);
        let assign5070_e5437: f64 = (assign5070_e5433).powf(assign5070_e5436);
        let assign5070_e5438: f64 = (assign5070_e5428 * assign5070_e5437);
        let assign5070_e5441: f64 = (1.0 - p.p38);
        let assign5070_e5442: f64 = (assign5070_e5438 / assign5070_e5441);
        (assign5070_e5442, ((((-var_pe_t_dn0) * assign5070_e5437) + (assign5070_e5428 * if 0.0 == 0.0 && ((assign5070_e5436) as f64).is_finite() && ((assign5070_e5436) as f64).fract() == 0.0 { if assign5070_e5436 == 0.0 { 0.0 } else { (assign5070_e5436 * ((assign5070_e5433).powf(assign5070_e5436 - 1.0) * (-(((var_vl0__blk155_dn0 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn0)) / (var_pe_t * var_pe_t))))) } } else { (assign5070_e5437 * (assign5070_e5436 * ((-(((var_vl0__blk155_dn0 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn0)) / (var_pe_t * var_pe_t))) / assign5070_e5433))) })) / assign5070_e5441), ((((-var_pe_t_dn1) * assign5070_e5437) + (assign5070_e5428 * if 0.0 == 0.0 && ((assign5070_e5436) as f64).is_finite() && ((assign5070_e5436) as f64).fract() == 0.0 { if assign5070_e5436 == 0.0 { 0.0 } else { (assign5070_e5436 * ((assign5070_e5433).powf(assign5070_e5436 - 1.0) * (-(((var_vl0__blk155_dn1 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn1)) / (var_pe_t * var_pe_t))))) } } else { (assign5070_e5437 * (assign5070_e5436 * ((-(((var_vl0__blk155_dn1 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn1)) / (var_pe_t * var_pe_t))) / assign5070_e5433))) })) / assign5070_e5441), ((((-var_pe_t_dn2) * assign5070_e5437) + (assign5070_e5428 * if 0.0 == 0.0 && ((assign5070_e5436) as f64).is_finite() && ((assign5070_e5436) as f64).fract() == 0.0 { if assign5070_e5436 == 0.0 { 0.0 } else { (assign5070_e5436 * ((assign5070_e5433).powf(assign5070_e5436 - 1.0) * (-(((var_vl0__blk155_dn2 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn2)) / (var_pe_t * var_pe_t))))) } } else { (assign5070_e5437 * (assign5070_e5436 * ((-(((var_vl0__blk155_dn2 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn2)) / (var_pe_t * var_pe_t))) / assign5070_e5433))) })) / assign5070_e5441), ((((-var_pe_t_dn3) * assign5070_e5437) + (assign5070_e5428 * if 0.0 == 0.0 && ((assign5070_e5436) as f64).is_finite() && ((assign5070_e5436) as f64).fract() == 0.0 { if assign5070_e5436 == 0.0 { 0.0 } else { (assign5070_e5436 * ((assign5070_e5433).powf(assign5070_e5436 - 1.0) * (-(((var_vl0__blk155_dn3 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn3)) / (var_pe_t * var_pe_t))))) } } else { (assign5070_e5437 * (assign5070_e5436 * ((-(((var_vl0__blk155_dn3 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn3)) / (var_pe_t * var_pe_t))) / assign5070_e5433))) })) / assign5070_e5441), ((((-var_pe_t_dn4) * assign5070_e5437) + (assign5070_e5428 * if 0.0 == 0.0 && ((assign5070_e5436) as f64).is_finite() && ((assign5070_e5436) as f64).fract() == 0.0 { if assign5070_e5436 == 0.0 { 0.0 } else { (assign5070_e5436 * ((assign5070_e5433).powf(assign5070_e5436 - 1.0) * (-(((var_vl0__blk155_dn4 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn4)) / (var_pe_t * var_pe_t))))) } } else { (assign5070_e5437 * (assign5070_e5436 * ((-(((var_vl0__blk155_dn4 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn4)) / (var_pe_t * var_pe_t))) / assign5070_e5433))) })) / assign5070_e5441), ((((-var_pe_t_dn5) * assign5070_e5437) + (assign5070_e5428 * if 0.0 == 0.0 && ((assign5070_e5436) as f64).is_finite() && ((assign5070_e5436) as f64).fract() == 0.0 { if assign5070_e5436 == 0.0 { 0.0 } else { (assign5070_e5436 * ((assign5070_e5433).powf(assign5070_e5436 - 1.0) * (-(((var_vl0__blk155_dn5 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn5)) / (var_pe_t * var_pe_t))))) } } else { (assign5070_e5437 * (assign5070_e5436 * ((-(((var_vl0__blk155_dn5 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn5)) / (var_pe_t * var_pe_t))) / assign5070_e5433))) })) / assign5070_e5441), ((((-var_pe_t_dn6) * assign5070_e5437) + (assign5070_e5428 * if 0.0 == 0.0 && ((assign5070_e5436) as f64).is_finite() && ((assign5070_e5436) as f64).fract() == 0.0 { if assign5070_e5436 == 0.0 { 0.0 } else { (assign5070_e5436 * ((assign5070_e5433).powf(assign5070_e5436 - 1.0) * (-(((var_vl0__blk155_dn6 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn6)) / (var_pe_t * var_pe_t))))) } } else { (assign5070_e5437 * (assign5070_e5436 * ((-(((var_vl0__blk155_dn6 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn6)) / (var_pe_t * var_pe_t))) / assign5070_e5433))) })) / assign5070_e5441), ((((-var_pe_t_dn7) * assign5070_e5437) + (assign5070_e5428 * if 0.0 == 0.0 && ((assign5070_e5436) as f64).is_finite() && ((assign5070_e5436) as f64).fract() == 0.0 { if assign5070_e5436 == 0.0 { 0.0 } else { (assign5070_e5436 * ((assign5070_e5433).powf(assign5070_e5436 - 1.0) * (-(((var_vl0__blk155_dn7 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn7)) / (var_pe_t * var_pe_t))))) } } else { (assign5070_e5437 * (assign5070_e5436 * ((-(((var_vl0__blk155_dn7 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn7)) / (var_pe_t * var_pe_t))) / assign5070_e5433))) })) / assign5070_e5441), ((((-var_pe_t_dn8) * assign5070_e5437) + (assign5070_e5428 * if 0.0 == 0.0 && ((assign5070_e5436) as f64).is_finite() && ((assign5070_e5436) as f64).fract() == 0.0 { if assign5070_e5436 == 0.0 { 0.0 } else { (assign5070_e5436 * ((assign5070_e5433).powf(assign5070_e5436 - 1.0) * (-(((var_vl0__blk155_dn8 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn8)) / (var_pe_t * var_pe_t))))) } } else { (assign5070_e5437 * (assign5070_e5436 * ((-(((var_vl0__blk155_dn8 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn8)) / (var_pe_t * var_pe_t))) / assign5070_e5433))) })) / assign5070_e5441), ((((-var_pe_t_dn9) * assign5070_e5437) + (assign5070_e5428 * if 0.0 == 0.0 && ((assign5070_e5436) as f64).is_finite() && ((assign5070_e5436) as f64).fract() == 0.0 { if assign5070_e5436 == 0.0 { 0.0 } else { (assign5070_e5436 * ((assign5070_e5433).powf(assign5070_e5436 - 1.0) * (-(((var_vl0__blk155_dn9 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn9)) / (var_pe_t * var_pe_t))))) } } else { (assign5070_e5437 * (assign5070_e5436 * ((-(((var_vl0__blk155_dn9 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn9)) / (var_pe_t * var_pe_t))) / assign5070_e5433))) })) / assign5070_e5441), ((((-var_pe_t_dn10) * assign5070_e5437) + (assign5070_e5428 * if 0.0 == 0.0 && ((assign5070_e5436) as f64).is_finite() && ((assign5070_e5436) as f64).fract() == 0.0 { if assign5070_e5436 == 0.0 { 0.0 } else { (assign5070_e5436 * ((assign5070_e5433).powf(assign5070_e5436 - 1.0) * (-(((var_vl0__blk155_dn10 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn10)) / (var_pe_t * var_pe_t))))) } } else { (assign5070_e5437 * (assign5070_e5436 * ((-(((var_vl0__blk155_dn10 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn10)) / (var_pe_t * var_pe_t))) / assign5070_e5433))) })) / assign5070_e5441), ((((-var_pe_t_dn11) * assign5070_e5437) + (assign5070_e5428 * if 0.0 == 0.0 && ((assign5070_e5436) as f64).is_finite() && ((assign5070_e5436) as f64).fract() == 0.0 { if assign5070_e5436 == 0.0 { 0.0 } else { (assign5070_e5436 * ((assign5070_e5433).powf(assign5070_e5436 - 1.0) * (-(((var_vl0__blk155_dn11 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn11)) / (var_pe_t * var_pe_t))))) } } else { (assign5070_e5437 * (assign5070_e5436 * ((-(((var_vl0__blk155_dn11 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn11)) / (var_pe_t * var_pe_t))) / assign5070_e5433))) })) / assign5070_e5441), ((((-var_pe_t_dn12) * assign5070_e5437) + (assign5070_e5428 * if 0.0 == 0.0 && ((assign5070_e5436) as f64).is_finite() && ((assign5070_e5436) as f64).fract() == 0.0 { if assign5070_e5436 == 0.0 { 0.0 } else { (assign5070_e5436 * ((assign5070_e5433).powf(assign5070_e5436 - 1.0) * (-(((var_vl0__blk155_dn12 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn12)) / (var_pe_t * var_pe_t))))) } } else { (assign5070_e5437 * (assign5070_e5436 * ((-(((var_vl0__blk155_dn12 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn12)) / (var_pe_t * var_pe_t))) / assign5070_e5433))) })) / assign5070_e5441), ((((-var_pe_t_dn13) * assign5070_e5437) + (assign5070_e5428 * if 0.0 == 0.0 && ((assign5070_e5436) as f64).is_finite() && ((assign5070_e5436) as f64).fract() == 0.0 { if assign5070_e5436 == 0.0 { 0.0 } else { (assign5070_e5436 * ((assign5070_e5433).powf(assign5070_e5436 - 1.0) * (-(((var_vl0__blk155_dn13 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn13)) / (var_pe_t * var_pe_t))))) } } else { (assign5070_e5437 * (assign5070_e5436 * ((-(((var_vl0__blk155_dn13 * var_pe_t) - (var_vl0__blk155 * var_pe_t_dn13)) / (var_pe_t * var_pe_t))) / assign5070_e5433))) })) / assign5070_e5441),)
    } else {
        (var_q0__blk156, var_q0__blk156_dn0, var_q0__blk156_dn1, var_q0__blk156_dn2, var_q0__blk156_dn3, var_q0__blk156_dn4, var_q0__blk156_dn5, var_q0__blk156_dn6, var_q0__blk156_dn7, var_q0__blk156_dn8, var_q0__blk156_dn9, var_q0__blk156_dn10, var_q0__blk156_dn11, var_q0__blk156_dn12, var_q0__blk156_dn13,)
    }
};
        var_q0__blk156 = assign5070_e5444;
        var_q0__blk156_dn0 = assign5070_e5444_d_n0;
        var_q0__blk156_dn1 = assign5070_e5444_d_n1;
        var_q0__blk156_dn2 = assign5070_e5444_d_n2;
        var_q0__blk156_dn3 = assign5070_e5444_d_n3;
        var_q0__blk156_dn4 = assign5070_e5444_d_n4;
        var_q0__blk156_dn5 = assign5070_e5444_d_n5;
        var_q0__blk156_dn6 = assign5070_e5444_d_n6;
        var_q0__blk156_dn7 = assign5070_e5444_d_n7;
        var_q0__blk156_dn8 = assign5070_e5444_d_n8;
        var_q0__blk156_dn9 = assign5070_e5444_d_n9;
        var_q0__blk156_dn10 = assign5070_e5444_d_n10;
        var_q0__blk156_dn11 = assign5070_e5444_d_n11;
        var_q0__blk156_dn12 = assign5070_e5444_d_n12;
        var_q0__blk156_dn13 = assign5070_e5444_d_n13;

        let (assign5080_e5451, assign5080_e5451_d_n0, assign5080_e5451_d_n1, assign5080_e5451_d_n2, assign5080_e5451_d_n3, assign5080_e5451_d_n4, assign5080_e5451_d_n5, assign5080_e5451_d_n6, assign5080_e5451_d_n7, assign5080_e5451_d_n8, assign5080_e5451_d_n9, assign5080_e5451_d_n10, assign5080_e5451_d_n11, assign5080_e5451_d_n12, assign5080_e5451_d_n13,) = {
    if (var_guard160 == 0.0) {
        let assign5080_e5449: f64 = (var_vbex + var_dv0__blk149);
        (assign5080_e5449, (var_vbex_dn0 + var_dv0__blk149_dn0), (var_vbex_dn1 + var_dv0__blk149_dn1), (var_vbex_dn2 + var_dv0__blk149_dn2), (var_vbex_dn3 + var_dv0__blk149_dn3), (var_vbex_dn4 + var_dv0__blk149_dn4), (var_vbex_dn5 + var_dv0__blk149_dn5), (var_vbex_dn6 + var_dv0__blk149_dn6), (var_vbex_dn7 + var_dv0__blk149_dn7), (var_vbex_dn8 + var_dv0__blk149_dn8), (var_vbex_dn9 + var_dv0__blk149_dn9), (var_vbex_dn10 + var_dv0__blk149_dn10), (var_vbex_dn11 + var_dv0__blk149_dn11), (var_vbex_dn12 + var_dv0__blk149_dn12), (var_vbex_dn13 + var_dv0__blk149_dn13),)
    } else {
        (var_dv__blk157, var_dv__blk157_dn0, var_dv__blk157_dn1, var_dv__blk157_dn2, var_dv__blk157_dn3, var_dv__blk157_dn4, var_dv__blk157_dn5, var_dv__blk157_dn6, var_dv__blk157_dn7, var_dv__blk157_dn8, var_dv__blk157_dn9, var_dv__blk157_dn10, var_dv__blk157_dn11, var_dv__blk157_dn12, var_dv__blk157_dn13,)
    }
};
        var_dv__blk157 = assign5080_e5451;
        var_dv__blk157_dn0 = assign5080_e5451_d_n0;
        var_dv__blk157_dn1 = assign5080_e5451_d_n1;
        var_dv__blk157_dn2 = assign5080_e5451_d_n2;
        var_dv__blk157_dn3 = assign5080_e5451_d_n3;
        var_dv__blk157_dn4 = assign5080_e5451_d_n4;
        var_dv__blk157_dn5 = assign5080_e5451_d_n5;
        var_dv__blk157_dn6 = assign5080_e5451_d_n6;
        var_dv__blk157_dn7 = assign5080_e5451_d_n7;
        var_dv__blk157_dn8 = assign5080_e5451_d_n8;
        var_dv__blk157_dn9 = assign5080_e5451_d_n9;
        var_dv__blk157_dn10 = assign5080_e5451_d_n10;
        var_dv__blk157_dn11 = assign5080_e5451_d_n11;
        var_dv__blk157_dn12 = assign5080_e5451_d_n12;
        var_dv__blk157_dn13 = assign5080_e5451_d_n13;

        let (assign5090_e5465, assign5090_e5465_d_n0, assign5090_e5465_d_n1, assign5090_e5465_d_n2, assign5090_e5465_d_n3, assign5090_e5465_d_n4, assign5090_e5465_d_n5, assign5090_e5465_d_n6, assign5090_e5465_d_n7, assign5090_e5465_d_n8, assign5090_e5465_d_n9, assign5090_e5465_d_n10, assign5090_e5465_d_n11, assign5090_e5465_d_n12, assign5090_e5465_d_n13,) = {
    if (var_guard160 == 0.0) {
        let assign5090_e5456: f64 = (var_dv__blk157 * var_dv__blk157);
        let assign5090_e5459: f64 = (4.0 * p.p39);
        let assign5090_e5461: f64 = (assign5090_e5459 * p.p39);
        let assign5090_e5462: f64 = (assign5090_e5456 + assign5090_e5461);
        let assign5090_e5463: f64 = (assign5090_e5462).sqrt();
        (assign5090_e5463, (((var_dv__blk157_dn0 * var_dv__blk157) + (var_dv__blk157 * var_dv__blk157_dn0)) / (2.0 * assign5090_e5463)), (((var_dv__blk157_dn1 * var_dv__blk157) + (var_dv__blk157 * var_dv__blk157_dn1)) / (2.0 * assign5090_e5463)), (((var_dv__blk157_dn2 * var_dv__blk157) + (var_dv__blk157 * var_dv__blk157_dn2)) / (2.0 * assign5090_e5463)), (((var_dv__blk157_dn3 * var_dv__blk157) + (var_dv__blk157 * var_dv__blk157_dn3)) / (2.0 * assign5090_e5463)), (((var_dv__blk157_dn4 * var_dv__blk157) + (var_dv__blk157 * var_dv__blk157_dn4)) / (2.0 * assign5090_e5463)), (((var_dv__blk157_dn5 * var_dv__blk157) + (var_dv__blk157 * var_dv__blk157_dn5)) / (2.0 * assign5090_e5463)), (((var_dv__blk157_dn6 * var_dv__blk157) + (var_dv__blk157 * var_dv__blk157_dn6)) / (2.0 * assign5090_e5463)), (((var_dv__blk157_dn7 * var_dv__blk157) + (var_dv__blk157 * var_dv__blk157_dn7)) / (2.0 * assign5090_e5463)), (((var_dv__blk157_dn8 * var_dv__blk157) + (var_dv__blk157 * var_dv__blk157_dn8)) / (2.0 * assign5090_e5463)), (((var_dv__blk157_dn9 * var_dv__blk157) + (var_dv__blk157 * var_dv__blk157_dn9)) / (2.0 * assign5090_e5463)), (((var_dv__blk157_dn10 * var_dv__blk157) + (var_dv__blk157 * var_dv__blk157_dn10)) / (2.0 * assign5090_e5463)), (((var_dv__blk157_dn11 * var_dv__blk157) + (var_dv__blk157 * var_dv__blk157_dn11)) / (2.0 * assign5090_e5463)), (((var_dv__blk157_dn12 * var_dv__blk157) + (var_dv__blk157 * var_dv__blk157_dn12)) / (2.0 * assign5090_e5463)), (((var_dv__blk157_dn13 * var_dv__blk157) + (var_dv__blk157 * var_dv__blk157_dn13)) / (2.0 * assign5090_e5463)),)
    } else {
        (var_mv__blk158, var_mv__blk158_dn0, var_mv__blk158_dn1, var_mv__blk158_dn2, var_mv__blk158_dn3, var_mv__blk158_dn4, var_mv__blk158_dn5, var_mv__blk158_dn6, var_mv__blk158_dn7, var_mv__blk158_dn8, var_mv__blk158_dn9, var_mv__blk158_dn10, var_mv__blk158_dn11, var_mv__blk158_dn12, var_mv__blk158_dn13,)
    }
};
        var_mv__blk158 = assign5090_e5465;
        var_mv__blk158_dn0 = assign5090_e5465_d_n0;
        var_mv__blk158_dn1 = assign5090_e5465_d_n1;
        var_mv__blk158_dn2 = assign5090_e5465_d_n2;
        var_mv__blk158_dn3 = assign5090_e5465_d_n3;
        var_mv__blk158_dn4 = assign5090_e5465_d_n4;
        var_mv__blk158_dn5 = assign5090_e5465_d_n5;
        var_mv__blk158_dn6 = assign5090_e5465_d_n6;
        var_mv__blk158_dn7 = assign5090_e5465_d_n7;
        var_mv__blk158_dn8 = assign5090_e5465_d_n8;
        var_mv__blk158_dn9 = assign5090_e5465_d_n9;
        var_mv__blk158_dn10 = assign5090_e5465_d_n10;
        var_mv__blk158_dn11 = assign5090_e5465_d_n11;
        var_mv__blk158_dn12 = assign5090_e5465_d_n12;
        var_mv__blk158_dn13 = assign5090_e5465_d_n13;

        let (assign5100_e5476, assign5100_e5476_d_n0, assign5100_e5476_d_n1, assign5100_e5476_d_n2, assign5100_e5476_d_n3, assign5100_e5476_d_n4, assign5100_e5476_d_n5, assign5100_e5476_d_n6, assign5100_e5476_d_n7, assign5100_e5476_d_n8, assign5100_e5476_d_n9, assign5100_e5476_d_n10, assign5100_e5476_d_n11, assign5100_e5476_d_n12, assign5100_e5476_d_n13,) = {
    if (var_guard160 == 0.0) {
        let assign5100_e5471: f64 = (var_dv__blk157 - var_mv__blk158);
        let assign5100_e5472: f64 = (0.5 * assign5100_e5471);
        let assign5100_e5474: f64 = (assign5100_e5472 - var_dv0__blk149);
        (assign5100_e5474, ((0.5 * (var_dv__blk157_dn0 - var_mv__blk158_dn0)) - var_dv0__blk149_dn0), ((0.5 * (var_dv__blk157_dn1 - var_mv__blk158_dn1)) - var_dv0__blk149_dn1), ((0.5 * (var_dv__blk157_dn2 - var_mv__blk158_dn2)) - var_dv0__blk149_dn2), ((0.5 * (var_dv__blk157_dn3 - var_mv__blk158_dn3)) - var_dv0__blk149_dn3), ((0.5 * (var_dv__blk157_dn4 - var_mv__blk158_dn4)) - var_dv0__blk149_dn4), ((0.5 * (var_dv__blk157_dn5 - var_mv__blk158_dn5)) - var_dv0__blk149_dn5), ((0.5 * (var_dv__blk157_dn6 - var_mv__blk158_dn6)) - var_dv0__blk149_dn6), ((0.5 * (var_dv__blk157_dn7 - var_mv__blk158_dn7)) - var_dv0__blk149_dn7), ((0.5 * (var_dv__blk157_dn8 - var_mv__blk158_dn8)) - var_dv0__blk149_dn8), ((0.5 * (var_dv__blk157_dn9 - var_mv__blk158_dn9)) - var_dv0__blk149_dn9), ((0.5 * (var_dv__blk157_dn10 - var_mv__blk158_dn10)) - var_dv0__blk149_dn10), ((0.5 * (var_dv__blk157_dn11 - var_mv__blk158_dn11)) - var_dv0__blk149_dn11), ((0.5 * (var_dv__blk157_dn12 - var_mv__blk158_dn12)) - var_dv0__blk149_dn12), ((0.5 * (var_dv__blk157_dn13 - var_mv__blk158_dn13)) - var_dv0__blk149_dn13),)
    } else {
        (var_vl__blk159, var_vl__blk159_dn0, var_vl__blk159_dn1, var_vl__blk159_dn2, var_vl__blk159_dn3, var_vl__blk159_dn4, var_vl__blk159_dn5, var_vl__blk159_dn6, var_vl__blk159_dn7, var_vl__blk159_dn8, var_vl__blk159_dn9, var_vl__blk159_dn10, var_vl__blk159_dn11, var_vl__blk159_dn12, var_vl__blk159_dn13,)
    }
};
        var_vl__blk159 = assign5100_e5476;
        var_vl__blk159_dn0 = assign5100_e5476_d_n0;
        var_vl__blk159_dn1 = assign5100_e5476_d_n1;
        var_vl__blk159_dn2 = assign5100_e5476_d_n2;
        var_vl__blk159_dn3 = assign5100_e5476_d_n3;
        var_vl__blk159_dn4 = assign5100_e5476_d_n4;
        var_vl__blk159_dn5 = assign5100_e5476_d_n5;
        var_vl__blk159_dn6 = assign5100_e5476_d_n6;
        var_vl__blk159_dn7 = assign5100_e5476_d_n7;
        var_vl__blk159_dn8 = assign5100_e5476_d_n8;
        var_vl__blk159_dn9 = assign5100_e5476_d_n9;
        var_vl__blk159_dn10 = assign5100_e5476_d_n10;
        var_vl__blk159_dn11 = assign5100_e5476_d_n11;
        var_vl__blk159_dn12 = assign5100_e5476_d_n12;
        var_vl__blk159_dn13 = assign5100_e5476_d_n13;


        *var_dv0__blk149_slot = var_dv0__blk149;
        *var_dv0__blk149_dn0_slot = var_dv0__blk149_dn0;
        *var_dv0__blk149_dn1_slot = var_dv0__blk149_dn1;
        *var_dv0__blk149_dn10_slot = var_dv0__blk149_dn10;
        *var_dv0__blk149_dn11_slot = var_dv0__blk149_dn11;
        *var_dv0__blk149_dn12_slot = var_dv0__blk149_dn12;
        *var_dv0__blk149_dn13_slot = var_dv0__blk149_dn13;
        *var_dv0__blk149_dn2_slot = var_dv0__blk149_dn2;
        *var_dv0__blk149_dn3_slot = var_dv0__blk149_dn3;
        *var_dv0__blk149_dn4_slot = var_dv0__blk149_dn4;
        *var_dv0__blk149_dn5_slot = var_dv0__blk149_dn5;
        *var_dv0__blk149_dn6_slot = var_dv0__blk149_dn6;
        *var_dv0__blk149_dn7_slot = var_dv0__blk149_dn7;
        *var_dv0__blk149_dn8_slot = var_dv0__blk149_dn8;
        *var_dv0__blk149_dn9_slot = var_dv0__blk149_dn9;
        *var_dv__blk157_slot = var_dv__blk157;
        *var_dv__blk157_dn0_slot = var_dv__blk157_dn0;
        *var_dv__blk157_dn1_slot = var_dv__blk157_dn1;
        *var_dv__blk157_dn10_slot = var_dv__blk157_dn10;
        *var_dv__blk157_dn11_slot = var_dv__blk157_dn11;
        *var_dv__blk157_dn12_slot = var_dv__blk157_dn12;
        *var_dv__blk157_dn13_slot = var_dv__blk157_dn13;
        *var_dv__blk157_dn2_slot = var_dv__blk157_dn2;
        *var_dv__blk157_dn3_slot = var_dv__blk157_dn3;
        *var_dv__blk157_dn4_slot = var_dv__blk157_dn4;
        *var_dv__blk157_dn5_slot = var_dv__blk157_dn5;
        *var_dv__blk157_dn6_slot = var_dv__blk157_dn6;
        *var_dv__blk157_dn7_slot = var_dv__blk157_dn7;
        *var_dv__blk157_dn8_slot = var_dv__blk157_dn8;
        *var_dv__blk157_dn9_slot = var_dv__blk157_dn9;
        *var_dvh__blk150_slot = var_dvh__blk150;
        *var_dvh__blk150_dn0_slot = var_dvh__blk150_dn0;
        *var_dvh__blk150_dn1_slot = var_dvh__blk150_dn1;
        *var_dvh__blk150_dn10_slot = var_dvh__blk150_dn10;
        *var_dvh__blk150_dn11_slot = var_dvh__blk150_dn11;
        *var_dvh__blk150_dn12_slot = var_dvh__blk150_dn12;
        *var_dvh__blk150_dn13_slot = var_dvh__blk150_dn13;
        *var_dvh__blk150_dn2_slot = var_dvh__blk150_dn2;
        *var_dvh__blk150_dn3_slot = var_dvh__blk150_dn3;
        *var_dvh__blk150_dn4_slot = var_dvh__blk150_dn4;
        *var_dvh__blk150_dn5_slot = var_dvh__blk150_dn5;
        *var_dvh__blk150_dn6_slot = var_dvh__blk150_dn6;
        *var_dvh__blk150_dn7_slot = var_dvh__blk150_dn7;
        *var_dvh__blk150_dn8_slot = var_dvh__blk150_dn8;
        *var_dvh__blk150_dn9_slot = var_dvh__blk150_dn9;
        *var_guard160_slot = var_guard160;
        *var_guard161_slot = var_guard161;
        *var_mv0__blk154_slot = var_mv0__blk154;
        *var_mv0__blk154_dn0_slot = var_mv0__blk154_dn0;
        *var_mv0__blk154_dn1_slot = var_mv0__blk154_dn1;
        *var_mv0__blk154_dn10_slot = var_mv0__blk154_dn10;
        *var_mv0__blk154_dn11_slot = var_mv0__blk154_dn11;
        *var_mv0__blk154_dn12_slot = var_mv0__blk154_dn12;
        *var_mv0__blk154_dn13_slot = var_mv0__blk154_dn13;
        *var_mv0__blk154_dn2_slot = var_mv0__blk154_dn2;
        *var_mv0__blk154_dn3_slot = var_mv0__blk154_dn3;
        *var_mv0__blk154_dn4_slot = var_mv0__blk154_dn4;
        *var_mv0__blk154_dn5_slot = var_mv0__blk154_dn5;
        *var_mv0__blk154_dn6_slot = var_mv0__blk154_dn6;
        *var_mv0__blk154_dn7_slot = var_mv0__blk154_dn7;
        *var_mv0__blk154_dn8_slot = var_mv0__blk154_dn8;
        *var_mv0__blk154_dn9_slot = var_mv0__blk154_dn9;
        *var_mv__blk145_slot = var_mv__blk145;
        *var_mv__blk145_dn0_slot = var_mv__blk145_dn0;
        *var_mv__blk145_dn1_slot = var_mv__blk145_dn1;
        *var_mv__blk145_dn10_slot = var_mv__blk145_dn10;
        *var_mv__blk145_dn11_slot = var_mv__blk145_dn11;
        *var_mv__blk145_dn12_slot = var_mv__blk145_dn12;
        *var_mv__blk145_dn13_slot = var_mv__blk145_dn13;
        *var_mv__blk145_dn2_slot = var_mv__blk145_dn2;
        *var_mv__blk145_dn3_slot = var_mv__blk145_dn3;
        *var_mv__blk145_dn4_slot = var_mv__blk145_dn4;
        *var_mv__blk145_dn5_slot = var_mv__blk145_dn5;
        *var_mv__blk145_dn6_slot = var_mv__blk145_dn6;
        *var_mv__blk145_dn7_slot = var_mv__blk145_dn7;
        *var_mv__blk145_dn8_slot = var_mv__blk145_dn8;
        *var_mv__blk145_dn9_slot = var_mv__blk145_dn9;
        *var_mv__blk158_slot = var_mv__blk158;
        *var_mv__blk158_dn0_slot = var_mv__blk158_dn0;
        *var_mv__blk158_dn1_slot = var_mv__blk158_dn1;
        *var_mv__blk158_dn10_slot = var_mv__blk158_dn10;
        *var_mv__blk158_dn11_slot = var_mv__blk158_dn11;
        *var_mv__blk158_dn12_slot = var_mv__blk158_dn12;
        *var_mv__blk158_dn13_slot = var_mv__blk158_dn13;
        *var_mv__blk158_dn2_slot = var_mv__blk158_dn2;
        *var_mv__blk158_dn3_slot = var_mv__blk158_dn3;
        *var_mv__blk158_dn4_slot = var_mv__blk158_dn4;
        *var_mv__blk158_dn5_slot = var_mv__blk158_dn5;
        *var_mv__blk158_dn6_slot = var_mv__blk158_dn6;
        *var_mv__blk158_dn7_slot = var_mv__blk158_dn7;
        *var_mv__blk158_dn8_slot = var_mv__blk158_dn8;
        *var_mv__blk158_dn9_slot = var_mv__blk158_dn9;
        *var_pwq__blk151_slot = var_pwq__blk151;
        *var_q0__blk156_slot = var_q0__blk156;
        *var_q0__blk156_dn0_slot = var_q0__blk156_dn0;
        *var_q0__blk156_dn1_slot = var_q0__blk156_dn1;
        *var_q0__blk156_dn10_slot = var_q0__blk156_dn10;
        *var_q0__blk156_dn11_slot = var_q0__blk156_dn11;
        *var_q0__blk156_dn12_slot = var_q0__blk156_dn12;
        *var_q0__blk156_dn13_slot = var_q0__blk156_dn13;
        *var_q0__blk156_dn2_slot = var_q0__blk156_dn2;
        *var_q0__blk156_dn3_slot = var_q0__blk156_dn3;
        *var_q0__blk156_dn4_slot = var_q0__blk156_dn4;
        *var_q0__blk156_dn5_slot = var_q0__blk156_dn5;
        *var_q0__blk156_dn6_slot = var_q0__blk156_dn6;
        *var_q0__blk156_dn7_slot = var_q0__blk156_dn7;
        *var_q0__blk156_dn8_slot = var_q0__blk156_dn8;
        *var_q0__blk156_dn9_slot = var_q0__blk156_dn9;
        *var_qdbcp_slot = var_qdbcp;
        *var_qdbcp_dn0_slot = var_qdbcp_dn0;
        *var_qdbcp_dn1_slot = var_qdbcp_dn1;
        *var_qdbcp_dn10_slot = var_qdbcp_dn10;
        *var_qdbcp_dn11_slot = var_qdbcp_dn11;
        *var_qdbcp_dn12_slot = var_qdbcp_dn12;
        *var_qdbcp_dn13_slot = var_qdbcp_dn13;
        *var_qdbcp_dn2_slot = var_qdbcp_dn2;
        *var_qdbcp_dn3_slot = var_qdbcp_dn3;
        *var_qdbcp_dn4_slot = var_qdbcp_dn4;
        *var_qdbcp_dn5_slot = var_qdbcp_dn5;
        *var_qdbcp_dn6_slot = var_qdbcp_dn6;
        *var_qdbcp_dn7_slot = var_qdbcp_dn7;
        *var_qdbcp_dn8_slot = var_qdbcp_dn8;
        *var_qdbcp_dn9_slot = var_qdbcp_dn9;
        *var_qdbex_slot = var_qdbex;
        *var_qdbex_dn0_slot = var_qdbex_dn0;
        *var_qdbex_dn1_slot = var_qdbex_dn1;
        *var_qdbex_dn10_slot = var_qdbex_dn10;
        *var_qdbex_dn11_slot = var_qdbex_dn11;
        *var_qdbex_dn12_slot = var_qdbex_dn12;
        *var_qdbex_dn13_slot = var_qdbex_dn13;
        *var_qdbex_dn2_slot = var_qdbex_dn2;
        *var_qdbex_dn3_slot = var_qdbex_dn3;
        *var_qdbex_dn4_slot = var_qdbex_dn4;
        *var_qdbex_dn5_slot = var_qdbex_dn5;
        *var_qdbex_dn6_slot = var_qdbex_dn6;
        *var_qdbex_dn7_slot = var_qdbex_dn7;
        *var_qdbex_dn8_slot = var_qdbex_dn8;
        *var_qdbex_dn9_slot = var_qdbex_dn9;
        *var_qhi__blk153_slot = var_qhi__blk153;
        *var_qhi__blk153_dn0_slot = var_qhi__blk153_dn0;
        *var_qhi__blk153_dn1_slot = var_qhi__blk153_dn1;
        *var_qhi__blk153_dn10_slot = var_qhi__blk153_dn10;
        *var_qhi__blk153_dn11_slot = var_qhi__blk153_dn11;
        *var_qhi__blk153_dn12_slot = var_qhi__blk153_dn12;
        *var_qhi__blk153_dn13_slot = var_qhi__blk153_dn13;
        *var_qhi__blk153_dn2_slot = var_qhi__blk153_dn2;
        *var_qhi__blk153_dn3_slot = var_qhi__blk153_dn3;
        *var_qhi__blk153_dn4_slot = var_qhi__blk153_dn4;
        *var_qhi__blk153_dn5_slot = var_qhi__blk153_dn5;
        *var_qhi__blk153_dn6_slot = var_qhi__blk153_dn6;
        *var_qhi__blk153_dn7_slot = var_qhi__blk153_dn7;
        *var_qhi__blk153_dn8_slot = var_qhi__blk153_dn8;
        *var_qhi__blk153_dn9_slot = var_qhi__blk153_dn9;
        *var_qlo__blk139_slot = var_qlo__blk139;
        *var_qlo__blk139_dn0_slot = var_qlo__blk139_dn0;
        *var_qlo__blk139_dn1_slot = var_qlo__blk139_dn1;
        *var_qlo__blk139_dn10_slot = var_qlo__blk139_dn10;
        *var_qlo__blk139_dn11_slot = var_qlo__blk139_dn11;
        *var_qlo__blk139_dn12_slot = var_qlo__blk139_dn12;
        *var_qlo__blk139_dn13_slot = var_qlo__blk139_dn13;
        *var_qlo__blk139_dn2_slot = var_qlo__blk139_dn2;
        *var_qlo__blk139_dn3_slot = var_qlo__blk139_dn3;
        *var_qlo__blk139_dn4_slot = var_qlo__blk139_dn4;
        *var_qlo__blk139_dn5_slot = var_qlo__blk139_dn5;
        *var_qlo__blk139_dn6_slot = var_qlo__blk139_dn6;
        *var_qlo__blk139_dn7_slot = var_qlo__blk139_dn7;
        *var_qlo__blk139_dn8_slot = var_qlo__blk139_dn8;
        *var_qlo__blk139_dn9_slot = var_qlo__blk139_dn9;
        *var_qlo__blk152_slot = var_qlo__blk152;
        *var_qlo__blk152_dn0_slot = var_qlo__blk152_dn0;
        *var_qlo__blk152_dn1_slot = var_qlo__blk152_dn1;
        *var_qlo__blk152_dn10_slot = var_qlo__blk152_dn10;
        *var_qlo__blk152_dn11_slot = var_qlo__blk152_dn11;
        *var_qlo__blk152_dn12_slot = var_qlo__blk152_dn12;
        *var_qlo__blk152_dn13_slot = var_qlo__blk152_dn13;
        *var_qlo__blk152_dn2_slot = var_qlo__blk152_dn2;
        *var_qlo__blk152_dn3_slot = var_qlo__blk152_dn3;
        *var_qlo__blk152_dn4_slot = var_qlo__blk152_dn4;
        *var_qlo__blk152_dn5_slot = var_qlo__blk152_dn5;
        *var_qlo__blk152_dn6_slot = var_qlo__blk152_dn6;
        *var_qlo__blk152_dn7_slot = var_qlo__blk152_dn7;
        *var_qlo__blk152_dn8_slot = var_qlo__blk152_dn8;
        *var_qlo__blk152_dn9_slot = var_qlo__blk152_dn9;
        *var_vl0__blk155_slot = var_vl0__blk155;
        *var_vl0__blk155_dn0_slot = var_vl0__blk155_dn0;
        *var_vl0__blk155_dn1_slot = var_vl0__blk155_dn1;
        *var_vl0__blk155_dn10_slot = var_vl0__blk155_dn10;
        *var_vl0__blk155_dn11_slot = var_vl0__blk155_dn11;
        *var_vl0__blk155_dn12_slot = var_vl0__blk155_dn12;
        *var_vl0__blk155_dn13_slot = var_vl0__blk155_dn13;
        *var_vl0__blk155_dn2_slot = var_vl0__blk155_dn2;
        *var_vl0__blk155_dn3_slot = var_vl0__blk155_dn3;
        *var_vl0__blk155_dn4_slot = var_vl0__blk155_dn4;
        *var_vl0__blk155_dn5_slot = var_vl0__blk155_dn5;
        *var_vl0__blk155_dn6_slot = var_vl0__blk155_dn6;
        *var_vl0__blk155_dn7_slot = var_vl0__blk155_dn7;
        *var_vl0__blk155_dn8_slot = var_vl0__blk155_dn8;
        *var_vl0__blk155_dn9_slot = var_vl0__blk155_dn9;
        *var_vl__blk146_slot = var_vl__blk146;
        *var_vl__blk146_dn0_slot = var_vl__blk146_dn0;
        *var_vl__blk146_dn1_slot = var_vl__blk146_dn1;
        *var_vl__blk146_dn10_slot = var_vl__blk146_dn10;
        *var_vl__blk146_dn11_slot = var_vl__blk146_dn11;
        *var_vl__blk146_dn12_slot = var_vl__blk146_dn12;
        *var_vl__blk146_dn13_slot = var_vl__blk146_dn13;
        *var_vl__blk146_dn2_slot = var_vl__blk146_dn2;
        *var_vl__blk146_dn3_slot = var_vl__blk146_dn3;
        *var_vl__blk146_dn4_slot = var_vl__blk146_dn4;
        *var_vl__blk146_dn5_slot = var_vl__blk146_dn5;
        *var_vl__blk146_dn6_slot = var_vl__blk146_dn6;
        *var_vl__blk146_dn7_slot = var_vl__blk146_dn7;
        *var_vl__blk146_dn8_slot = var_vl__blk146_dn8;
        *var_vl__blk146_dn9_slot = var_vl__blk146_dn9;
        *var_vl__blk159_slot = var_vl__blk159;
        *var_vl__blk159_dn0_slot = var_vl__blk159_dn0;
        *var_vl__blk159_dn1_slot = var_vl__blk159_dn1;
        *var_vl__blk159_dn10_slot = var_vl__blk159_dn10;
        *var_vl__blk159_dn11_slot = var_vl__blk159_dn11;
        *var_vl__blk159_dn12_slot = var_vl__blk159_dn12;
        *var_vl__blk159_dn13_slot = var_vl__blk159_dn13;
        *var_vl__blk159_dn2_slot = var_vl__blk159_dn2;
        *var_vl__blk159_dn3_slot = var_vl__blk159_dn3;
        *var_vl__blk159_dn4_slot = var_vl__blk159_dn4;
        *var_vl__blk159_dn5_slot = var_vl__blk159_dn5;
        *var_vl__blk159_dn6_slot = var_vl__blk159_dn6;
        *var_vl__blk159_dn7_slot = var_vl__blk159_dn7;
        *var_vl__blk159_dn8_slot = var_vl__blk159_dn8;
        *var_vl__blk159_dn9_slot = var_vl__blk159_dn9;
    }

    pub(super) fn stamp_transient_block_20(
        p: &Parameters,
        var_guard160: f64,
        var_pc_t: f64,
        var_pc_t_dn0: f64,
        var_pc_t_dn1: f64,
        var_pc_t_dn10: f64,
        var_pc_t_dn11: f64,
        var_pc_t_dn12: f64,
        var_pc_t_dn13: f64,
        var_pc_t_dn2: f64,
        var_pc_t_dn3: f64,
        var_pc_t_dn4: f64,
        var_pc_t_dn5: f64,
        var_pc_t_dn6: f64,
        var_pc_t_dn7: f64,
        var_pc_t_dn8: f64,
        var_pc_t_dn9: f64,
        var_pe_t: f64,
        var_pe_t_dn0: f64,
        var_pe_t_dn1: f64,
        var_pe_t_dn10: f64,
        var_pe_t_dn11: f64,
        var_pe_t_dn12: f64,
        var_pe_t_dn13: f64,
        var_pe_t_dn2: f64,
        var_pe_t_dn3: f64,
        var_pe_t_dn4: f64,
        var_pe_t_dn5: f64,
        var_pe_t_dn6: f64,
        var_pe_t_dn7: f64,
        var_pe_t_dn8: f64,
        var_pe_t_dn9: f64,
        var_q0__blk156: f64,
        var_q0__blk156_dn0: f64,
        var_q0__blk156_dn1: f64,
        var_q0__blk156_dn10: f64,
        var_q0__blk156_dn11: f64,
        var_q0__blk156_dn12: f64,
        var_q0__blk156_dn13: f64,
        var_q0__blk156_dn2: f64,
        var_q0__blk156_dn3: f64,
        var_q0__blk156_dn4: f64,
        var_q0__blk156_dn5: f64,
        var_q0__blk156_dn6: f64,
        var_q0__blk156_dn7: f64,
        var_q0__blk156_dn8: f64,
        var_q0__blk156_dn9: f64,
        var_vbep: f64,
        var_vbep_dn0: f64,
        var_vbep_dn1: f64,
        var_vbep_dn10: f64,
        var_vbep_dn11: f64,
        var_vbep_dn12: f64,
        var_vbep_dn13: f64,
        var_vbep_dn2: f64,
        var_vbep_dn3: f64,
        var_vbep_dn4: f64,
        var_vbep_dn5: f64,
        var_vbep_dn6: f64,
        var_vbep_dn7: f64,
        var_vbep_dn8: f64,
        var_vbep_dn9: f64,
        var_vbex: f64,
        var_vbex_dn0: f64,
        var_vbex_dn1: f64,
        var_vbex_dn10: f64,
        var_vbex_dn11: f64,
        var_vbex_dn12: f64,
        var_vbex_dn13: f64,
        var_vbex_dn2: f64,
        var_vbex_dn3: f64,
        var_vbex_dn4: f64,
        var_vbex_dn5: f64,
        var_vbex_dn6: f64,
        var_vbex_dn7: f64,
        var_vbex_dn8: f64,
        var_vbex_dn9: f64,
        var_vl0__blk155: f64,
        var_vl0__blk155_dn0: f64,
        var_vl0__blk155_dn1: f64,
        var_vl0__blk155_dn10: f64,
        var_vl0__blk155_dn11: f64,
        var_vl0__blk155_dn12: f64,
        var_vl0__blk155_dn13: f64,
        var_vl0__blk155_dn2: f64,
        var_vl0__blk155_dn3: f64,
        var_vl0__blk155_dn4: f64,
        var_vl0__blk155_dn5: f64,
        var_vl0__blk155_dn6: f64,
        var_vl0__blk155_dn7: f64,
        var_vl0__blk155_dn8: f64,
        var_vl0__blk155_dn9: f64,
        var_vl__blk159: f64,
        var_vl__blk159_dn0: f64,
        var_vl__blk159_dn1: f64,
        var_vl__blk159_dn10: f64,
        var_vl__blk159_dn11: f64,
        var_vl__blk159_dn12: f64,
        var_vl__blk159_dn13: f64,
        var_vl__blk159_dn2: f64,
        var_vl__blk159_dn3: f64,
        var_vl__blk159_dn4: f64,
        var_vl__blk159_dn5: f64,
        var_vl__blk159_dn6: f64,
        var_vl__blk159_dn7: f64,
        var_vl__blk159_dn8: f64,
        var_vl__blk159_dn9: f64,
        var_dv0__blk162_slot: &mut f64,
        var_dv0__blk162_dn0_slot: &mut f64,
        var_dv0__blk162_dn1_slot: &mut f64,
        var_dv0__blk162_dn10_slot: &mut f64,
        var_dv0__blk162_dn11_slot: &mut f64,
        var_dv0__blk162_dn12_slot: &mut f64,
        var_dv0__blk162_dn13_slot: &mut f64,
        var_dv0__blk162_dn2_slot: &mut f64,
        var_dv0__blk162_dn3_slot: &mut f64,
        var_dv0__blk162_dn4_slot: &mut f64,
        var_dv0__blk162_dn5_slot: &mut f64,
        var_dv0__blk162_dn6_slot: &mut f64,
        var_dv0__blk162_dn7_slot: &mut f64,
        var_dv0__blk162_dn8_slot: &mut f64,
        var_dv0__blk162_dn9_slot: &mut f64,
        var_dvh__blk163_slot: &mut f64,
        var_dvh__blk163_dn0_slot: &mut f64,
        var_dvh__blk163_dn1_slot: &mut f64,
        var_dvh__blk163_dn10_slot: &mut f64,
        var_dvh__blk163_dn11_slot: &mut f64,
        var_dvh__blk163_dn12_slot: &mut f64,
        var_dvh__blk163_dn13_slot: &mut f64,
        var_dvh__blk163_dn2_slot: &mut f64,
        var_dvh__blk163_dn3_slot: &mut f64,
        var_dvh__blk163_dn4_slot: &mut f64,
        var_dvh__blk163_dn5_slot: &mut f64,
        var_dvh__blk163_dn6_slot: &mut f64,
        var_dvh__blk163_dn7_slot: &mut f64,
        var_dvh__blk163_dn8_slot: &mut f64,
        var_dvh__blk163_dn9_slot: &mut f64,
        var_guard183_slot: &mut f64,
        var_guard184_slot: &mut f64,
        var_guard185_slot: &mut f64,
        var_guard186_slot: &mut f64,
        var_pwq__blk164_slot: &mut f64,
        var_qdbep_slot: &mut f64,
        var_qdbep_dn0_slot: &mut f64,
        var_qdbep_dn1_slot: &mut f64,
        var_qdbep_dn10_slot: &mut f64,
        var_qdbep_dn11_slot: &mut f64,
        var_qdbep_dn12_slot: &mut f64,
        var_qdbep_dn13_slot: &mut f64,
        var_qdbep_dn2_slot: &mut f64,
        var_qdbep_dn3_slot: &mut f64,
        var_qdbep_dn4_slot: &mut f64,
        var_qdbep_dn5_slot: &mut f64,
        var_qdbep_dn6_slot: &mut f64,
        var_qdbep_dn7_slot: &mut f64,
        var_qdbep_dn8_slot: &mut f64,
        var_qdbep_dn9_slot: &mut f64,
        var_qdbex_slot: &mut f64,
        var_qdbex_dn0_slot: &mut f64,
        var_qdbex_dn1_slot: &mut f64,
        var_qdbex_dn10_slot: &mut f64,
        var_qdbex_dn11_slot: &mut f64,
        var_qdbex_dn12_slot: &mut f64,
        var_qdbex_dn13_slot: &mut f64,
        var_qdbex_dn2_slot: &mut f64,
        var_qdbex_dn3_slot: &mut f64,
        var_qdbex_dn4_slot: &mut f64,
        var_qdbex_dn5_slot: &mut f64,
        var_qdbex_dn6_slot: &mut f64,
        var_qdbex_dn7_slot: &mut f64,
        var_qdbex_dn8_slot: &mut f64,
        var_qdbex_dn9_slot: &mut f64,
        var_qhi__blk166_slot: &mut f64,
        var_qhi__blk166_dn0_slot: &mut f64,
        var_qhi__blk166_dn1_slot: &mut f64,
        var_qhi__blk166_dn10_slot: &mut f64,
        var_qhi__blk166_dn11_slot: &mut f64,
        var_qhi__blk166_dn12_slot: &mut f64,
        var_qhi__blk166_dn13_slot: &mut f64,
        var_qhi__blk166_dn2_slot: &mut f64,
        var_qhi__blk166_dn3_slot: &mut f64,
        var_qhi__blk166_dn4_slot: &mut f64,
        var_qhi__blk166_dn5_slot: &mut f64,
        var_qhi__blk166_dn6_slot: &mut f64,
        var_qhi__blk166_dn7_slot: &mut f64,
        var_qhi__blk166_dn8_slot: &mut f64,
        var_qhi__blk166_dn9_slot: &mut f64,
        var_qlo0__blk170_slot: &mut f64,
        var_qlo0__blk170_dn0_slot: &mut f64,
        var_qlo0__blk170_dn1_slot: &mut f64,
        var_qlo0__blk170_dn10_slot: &mut f64,
        var_qlo0__blk170_dn11_slot: &mut f64,
        var_qlo0__blk170_dn12_slot: &mut f64,
        var_qlo0__blk170_dn13_slot: &mut f64,
        var_qlo0__blk170_dn2_slot: &mut f64,
        var_qlo0__blk170_dn3_slot: &mut f64,
        var_qlo0__blk170_dn4_slot: &mut f64,
        var_qlo0__blk170_dn5_slot: &mut f64,
        var_qlo0__blk170_dn6_slot: &mut f64,
        var_qlo0__blk170_dn7_slot: &mut f64,
        var_qlo0__blk170_dn8_slot: &mut f64,
        var_qlo0__blk170_dn9_slot: &mut f64,
        var_qlo__blk152_slot: &mut f64,
        var_qlo__blk152_dn0_slot: &mut f64,
        var_qlo__blk152_dn1_slot: &mut f64,
        var_qlo__blk152_dn10_slot: &mut f64,
        var_qlo__blk152_dn11_slot: &mut f64,
        var_qlo__blk152_dn12_slot: &mut f64,
        var_qlo__blk152_dn13_slot: &mut f64,
        var_qlo__blk152_dn2_slot: &mut f64,
        var_qlo__blk152_dn3_slot: &mut f64,
        var_qlo__blk152_dn4_slot: &mut f64,
        var_qlo__blk152_dn5_slot: &mut f64,
        var_qlo__blk152_dn6_slot: &mut f64,
        var_qlo__blk152_dn7_slot: &mut f64,
        var_qlo__blk152_dn8_slot: &mut f64,
        var_qlo__blk152_dn9_slot: &mut f64,
        var_qlo__blk165_slot: &mut f64,
        var_qlo__blk165_dn0_slot: &mut f64,
        var_qlo__blk165_dn1_slot: &mut f64,
        var_qlo__blk165_dn10_slot: &mut f64,
        var_qlo__blk165_dn11_slot: &mut f64,
        var_qlo__blk165_dn12_slot: &mut f64,
        var_qlo__blk165_dn13_slot: &mut f64,
        var_qlo__blk165_dn2_slot: &mut f64,
        var_qlo__blk165_dn3_slot: &mut f64,
        var_qlo__blk165_dn4_slot: &mut f64,
        var_qlo__blk165_dn5_slot: &mut f64,
        var_qlo__blk165_dn6_slot: &mut f64,
        var_qlo__blk165_dn7_slot: &mut f64,
        var_qlo__blk165_dn8_slot: &mut f64,
        var_qlo__blk165_dn9_slot: &mut f64,
        var_vl0__blk169_slot: &mut f64,
        var_vl0__blk169_dn0_slot: &mut f64,
        var_vl0__blk169_dn1_slot: &mut f64,
        var_vl0__blk169_dn10_slot: &mut f64,
        var_vl0__blk169_dn11_slot: &mut f64,
        var_vl0__blk169_dn12_slot: &mut f64,
        var_vl0__blk169_dn13_slot: &mut f64,
        var_vl0__blk169_dn2_slot: &mut f64,
        var_vl0__blk169_dn3_slot: &mut f64,
        var_vl0__blk169_dn4_slot: &mut f64,
        var_vl0__blk169_dn5_slot: &mut f64,
        var_vl0__blk169_dn6_slot: &mut f64,
        var_vl0__blk169_dn7_slot: &mut f64,
        var_vl0__blk169_dn8_slot: &mut f64,
        var_vl0__blk169_dn9_slot: &mut f64,
        var_vn0__blk167_slot: &mut f64,
        var_vn0__blk167_dn0_slot: &mut f64,
        var_vn0__blk167_dn1_slot: &mut f64,
        var_vn0__blk167_dn10_slot: &mut f64,
        var_vn0__blk167_dn11_slot: &mut f64,
        var_vn0__blk167_dn12_slot: &mut f64,
        var_vn0__blk167_dn13_slot: &mut f64,
        var_vn0__blk167_dn2_slot: &mut f64,
        var_vn0__blk167_dn3_slot: &mut f64,
        var_vn0__blk167_dn4_slot: &mut f64,
        var_vn0__blk167_dn5_slot: &mut f64,
        var_vn0__blk167_dn6_slot: &mut f64,
        var_vn0__blk167_dn7_slot: &mut f64,
        var_vn0__blk167_dn8_slot: &mut f64,
        var_vn0__blk167_dn9_slot: &mut f64,
        var_vn__blk171_slot: &mut f64,
        var_vn__blk171_dn0_slot: &mut f64,
        var_vn__blk171_dn1_slot: &mut f64,
        var_vn__blk171_dn10_slot: &mut f64,
        var_vn__blk171_dn11_slot: &mut f64,
        var_vn__blk171_dn12_slot: &mut f64,
        var_vn__blk171_dn13_slot: &mut f64,
        var_vn__blk171_dn2_slot: &mut f64,
        var_vn__blk171_dn3_slot: &mut f64,
        var_vn__blk171_dn4_slot: &mut f64,
        var_vn__blk171_dn5_slot: &mut f64,
        var_vn__blk171_dn6_slot: &mut f64,
        var_vn__blk171_dn7_slot: &mut f64,
        var_vn__blk171_dn8_slot: &mut f64,
        var_vn__blk171_dn9_slot: &mut f64,
        var_vnl0__blk168_slot: &mut f64,
        var_vnl0__blk168_dn0_slot: &mut f64,
        var_vnl0__blk168_dn1_slot: &mut f64,
        var_vnl0__blk168_dn10_slot: &mut f64,
        var_vnl0__blk168_dn11_slot: &mut f64,
        var_vnl0__blk168_dn12_slot: &mut f64,
        var_vnl0__blk168_dn13_slot: &mut f64,
        var_vnl0__blk168_dn2_slot: &mut f64,
        var_vnl0__blk168_dn3_slot: &mut f64,
        var_vnl0__blk168_dn4_slot: &mut f64,
        var_vnl0__blk168_dn5_slot: &mut f64,
        var_vnl0__blk168_dn6_slot: &mut f64,
        var_vnl0__blk168_dn7_slot: &mut f64,
        var_vnl0__blk168_dn8_slot: &mut f64,
        var_vnl0__blk168_dn9_slot: &mut f64,
        var_vnl__blk172_slot: &mut f64,
        var_vnl__blk172_dn0_slot: &mut f64,
        var_vnl__blk172_dn1_slot: &mut f64,
        var_vnl__blk172_dn10_slot: &mut f64,
        var_vnl__blk172_dn11_slot: &mut f64,
        var_vnl__blk172_dn12_slot: &mut f64,
        var_vnl__blk172_dn13_slot: &mut f64,
        var_vnl__blk172_dn2_slot: &mut f64,
        var_vnl__blk172_dn3_slot: &mut f64,
        var_vnl__blk172_dn4_slot: &mut f64,
        var_vnl__blk172_dn5_slot: &mut f64,
        var_vnl__blk172_dn6_slot: &mut f64,
        var_vnl__blk172_dn7_slot: &mut f64,
        var_vnl__blk172_dn8_slot: &mut f64,
        var_vnl__blk172_dn9_slot: &mut f64,
    ) {
        let mut var_dv0__blk162: f64 = *var_dv0__blk162_slot;
        let mut var_dv0__blk162_dn0: f64 = *var_dv0__blk162_dn0_slot;
        let mut var_dv0__blk162_dn1: f64 = *var_dv0__blk162_dn1_slot;
        let mut var_dv0__blk162_dn10: f64 = *var_dv0__blk162_dn10_slot;
        let mut var_dv0__blk162_dn11: f64 = *var_dv0__blk162_dn11_slot;
        let mut var_dv0__blk162_dn12: f64 = *var_dv0__blk162_dn12_slot;
        let mut var_dv0__blk162_dn13: f64 = *var_dv0__blk162_dn13_slot;
        let mut var_dv0__blk162_dn2: f64 = *var_dv0__blk162_dn2_slot;
        let mut var_dv0__blk162_dn3: f64 = *var_dv0__blk162_dn3_slot;
        let mut var_dv0__blk162_dn4: f64 = *var_dv0__blk162_dn4_slot;
        let mut var_dv0__blk162_dn5: f64 = *var_dv0__blk162_dn5_slot;
        let mut var_dv0__blk162_dn6: f64 = *var_dv0__blk162_dn6_slot;
        let mut var_dv0__blk162_dn7: f64 = *var_dv0__blk162_dn7_slot;
        let mut var_dv0__blk162_dn8: f64 = *var_dv0__blk162_dn8_slot;
        let mut var_dv0__blk162_dn9: f64 = *var_dv0__blk162_dn9_slot;
        let mut var_dvh__blk163: f64 = *var_dvh__blk163_slot;
        let mut var_dvh__blk163_dn0: f64 = *var_dvh__blk163_dn0_slot;
        let mut var_dvh__blk163_dn1: f64 = *var_dvh__blk163_dn1_slot;
        let mut var_dvh__blk163_dn10: f64 = *var_dvh__blk163_dn10_slot;
        let mut var_dvh__blk163_dn11: f64 = *var_dvh__blk163_dn11_slot;
        let mut var_dvh__blk163_dn12: f64 = *var_dvh__blk163_dn12_slot;
        let mut var_dvh__blk163_dn13: f64 = *var_dvh__blk163_dn13_slot;
        let mut var_dvh__blk163_dn2: f64 = *var_dvh__blk163_dn2_slot;
        let mut var_dvh__blk163_dn3: f64 = *var_dvh__blk163_dn3_slot;
        let mut var_dvh__blk163_dn4: f64 = *var_dvh__blk163_dn4_slot;
        let mut var_dvh__blk163_dn5: f64 = *var_dvh__blk163_dn5_slot;
        let mut var_dvh__blk163_dn6: f64 = *var_dvh__blk163_dn6_slot;
        let mut var_dvh__blk163_dn7: f64 = *var_dvh__blk163_dn7_slot;
        let mut var_dvh__blk163_dn8: f64 = *var_dvh__blk163_dn8_slot;
        let mut var_dvh__blk163_dn9: f64 = *var_dvh__blk163_dn9_slot;
        let mut var_guard183: f64 = *var_guard183_slot;
        let mut var_guard184: f64 = *var_guard184_slot;
        let mut var_guard185: f64 = *var_guard185_slot;
        let mut var_guard186: f64 = *var_guard186_slot;
        let mut var_pwq__blk164: f64 = *var_pwq__blk164_slot;
        let mut var_qdbep: f64 = *var_qdbep_slot;
        let mut var_qdbep_dn0: f64 = *var_qdbep_dn0_slot;
        let mut var_qdbep_dn1: f64 = *var_qdbep_dn1_slot;
        let mut var_qdbep_dn10: f64 = *var_qdbep_dn10_slot;
        let mut var_qdbep_dn11: f64 = *var_qdbep_dn11_slot;
        let mut var_qdbep_dn12: f64 = *var_qdbep_dn12_slot;
        let mut var_qdbep_dn13: f64 = *var_qdbep_dn13_slot;
        let mut var_qdbep_dn2: f64 = *var_qdbep_dn2_slot;
        let mut var_qdbep_dn3: f64 = *var_qdbep_dn3_slot;
        let mut var_qdbep_dn4: f64 = *var_qdbep_dn4_slot;
        let mut var_qdbep_dn5: f64 = *var_qdbep_dn5_slot;
        let mut var_qdbep_dn6: f64 = *var_qdbep_dn6_slot;
        let mut var_qdbep_dn7: f64 = *var_qdbep_dn7_slot;
        let mut var_qdbep_dn8: f64 = *var_qdbep_dn8_slot;
        let mut var_qdbep_dn9: f64 = *var_qdbep_dn9_slot;
        let mut var_qdbex: f64 = *var_qdbex_slot;
        let mut var_qdbex_dn0: f64 = *var_qdbex_dn0_slot;
        let mut var_qdbex_dn1: f64 = *var_qdbex_dn1_slot;
        let mut var_qdbex_dn10: f64 = *var_qdbex_dn10_slot;
        let mut var_qdbex_dn11: f64 = *var_qdbex_dn11_slot;
        let mut var_qdbex_dn12: f64 = *var_qdbex_dn12_slot;
        let mut var_qdbex_dn13: f64 = *var_qdbex_dn13_slot;
        let mut var_qdbex_dn2: f64 = *var_qdbex_dn2_slot;
        let mut var_qdbex_dn3: f64 = *var_qdbex_dn3_slot;
        let mut var_qdbex_dn4: f64 = *var_qdbex_dn4_slot;
        let mut var_qdbex_dn5: f64 = *var_qdbex_dn5_slot;
        let mut var_qdbex_dn6: f64 = *var_qdbex_dn6_slot;
        let mut var_qdbex_dn7: f64 = *var_qdbex_dn7_slot;
        let mut var_qdbex_dn8: f64 = *var_qdbex_dn8_slot;
        let mut var_qdbex_dn9: f64 = *var_qdbex_dn9_slot;
        let mut var_qhi__blk166: f64 = *var_qhi__blk166_slot;
        let mut var_qhi__blk166_dn0: f64 = *var_qhi__blk166_dn0_slot;
        let mut var_qhi__blk166_dn1: f64 = *var_qhi__blk166_dn1_slot;
        let mut var_qhi__blk166_dn10: f64 = *var_qhi__blk166_dn10_slot;
        let mut var_qhi__blk166_dn11: f64 = *var_qhi__blk166_dn11_slot;
        let mut var_qhi__blk166_dn12: f64 = *var_qhi__blk166_dn12_slot;
        let mut var_qhi__blk166_dn13: f64 = *var_qhi__blk166_dn13_slot;
        let mut var_qhi__blk166_dn2: f64 = *var_qhi__blk166_dn2_slot;
        let mut var_qhi__blk166_dn3: f64 = *var_qhi__blk166_dn3_slot;
        let mut var_qhi__blk166_dn4: f64 = *var_qhi__blk166_dn4_slot;
        let mut var_qhi__blk166_dn5: f64 = *var_qhi__blk166_dn5_slot;
        let mut var_qhi__blk166_dn6: f64 = *var_qhi__blk166_dn6_slot;
        let mut var_qhi__blk166_dn7: f64 = *var_qhi__blk166_dn7_slot;
        let mut var_qhi__blk166_dn8: f64 = *var_qhi__blk166_dn8_slot;
        let mut var_qhi__blk166_dn9: f64 = *var_qhi__blk166_dn9_slot;
        let mut var_qlo0__blk170: f64 = *var_qlo0__blk170_slot;
        let mut var_qlo0__blk170_dn0: f64 = *var_qlo0__blk170_dn0_slot;
        let mut var_qlo0__blk170_dn1: f64 = *var_qlo0__blk170_dn1_slot;
        let mut var_qlo0__blk170_dn10: f64 = *var_qlo0__blk170_dn10_slot;
        let mut var_qlo0__blk170_dn11: f64 = *var_qlo0__blk170_dn11_slot;
        let mut var_qlo0__blk170_dn12: f64 = *var_qlo0__blk170_dn12_slot;
        let mut var_qlo0__blk170_dn13: f64 = *var_qlo0__blk170_dn13_slot;
        let mut var_qlo0__blk170_dn2: f64 = *var_qlo0__blk170_dn2_slot;
        let mut var_qlo0__blk170_dn3: f64 = *var_qlo0__blk170_dn3_slot;
        let mut var_qlo0__blk170_dn4: f64 = *var_qlo0__blk170_dn4_slot;
        let mut var_qlo0__blk170_dn5: f64 = *var_qlo0__blk170_dn5_slot;
        let mut var_qlo0__blk170_dn6: f64 = *var_qlo0__blk170_dn6_slot;
        let mut var_qlo0__blk170_dn7: f64 = *var_qlo0__blk170_dn7_slot;
        let mut var_qlo0__blk170_dn8: f64 = *var_qlo0__blk170_dn8_slot;
        let mut var_qlo0__blk170_dn9: f64 = *var_qlo0__blk170_dn9_slot;
        let mut var_qlo__blk152: f64 = *var_qlo__blk152_slot;
        let mut var_qlo__blk152_dn0: f64 = *var_qlo__blk152_dn0_slot;
        let mut var_qlo__blk152_dn1: f64 = *var_qlo__blk152_dn1_slot;
        let mut var_qlo__blk152_dn10: f64 = *var_qlo__blk152_dn10_slot;
        let mut var_qlo__blk152_dn11: f64 = *var_qlo__blk152_dn11_slot;
        let mut var_qlo__blk152_dn12: f64 = *var_qlo__blk152_dn12_slot;
        let mut var_qlo__blk152_dn13: f64 = *var_qlo__blk152_dn13_slot;
        let mut var_qlo__blk152_dn2: f64 = *var_qlo__blk152_dn2_slot;
        let mut var_qlo__blk152_dn3: f64 = *var_qlo__blk152_dn3_slot;
        let mut var_qlo__blk152_dn4: f64 = *var_qlo__blk152_dn4_slot;
        let mut var_qlo__blk152_dn5: f64 = *var_qlo__blk152_dn5_slot;
        let mut var_qlo__blk152_dn6: f64 = *var_qlo__blk152_dn6_slot;
        let mut var_qlo__blk152_dn7: f64 = *var_qlo__blk152_dn7_slot;
        let mut var_qlo__blk152_dn8: f64 = *var_qlo__blk152_dn8_slot;
        let mut var_qlo__blk152_dn9: f64 = *var_qlo__blk152_dn9_slot;
        let mut var_qlo__blk165: f64 = *var_qlo__blk165_slot;
        let mut var_qlo__blk165_dn0: f64 = *var_qlo__blk165_dn0_slot;
        let mut var_qlo__blk165_dn1: f64 = *var_qlo__blk165_dn1_slot;
        let mut var_qlo__blk165_dn10: f64 = *var_qlo__blk165_dn10_slot;
        let mut var_qlo__blk165_dn11: f64 = *var_qlo__blk165_dn11_slot;
        let mut var_qlo__blk165_dn12: f64 = *var_qlo__blk165_dn12_slot;
        let mut var_qlo__blk165_dn13: f64 = *var_qlo__blk165_dn13_slot;
        let mut var_qlo__blk165_dn2: f64 = *var_qlo__blk165_dn2_slot;
        let mut var_qlo__blk165_dn3: f64 = *var_qlo__blk165_dn3_slot;
        let mut var_qlo__blk165_dn4: f64 = *var_qlo__blk165_dn4_slot;
        let mut var_qlo__blk165_dn5: f64 = *var_qlo__blk165_dn5_slot;
        let mut var_qlo__blk165_dn6: f64 = *var_qlo__blk165_dn6_slot;
        let mut var_qlo__blk165_dn7: f64 = *var_qlo__blk165_dn7_slot;
        let mut var_qlo__blk165_dn8: f64 = *var_qlo__blk165_dn8_slot;
        let mut var_qlo__blk165_dn9: f64 = *var_qlo__blk165_dn9_slot;
        let mut var_vl0__blk169: f64 = *var_vl0__blk169_slot;
        let mut var_vl0__blk169_dn0: f64 = *var_vl0__blk169_dn0_slot;
        let mut var_vl0__blk169_dn1: f64 = *var_vl0__blk169_dn1_slot;
        let mut var_vl0__blk169_dn10: f64 = *var_vl0__blk169_dn10_slot;
        let mut var_vl0__blk169_dn11: f64 = *var_vl0__blk169_dn11_slot;
        let mut var_vl0__blk169_dn12: f64 = *var_vl0__blk169_dn12_slot;
        let mut var_vl0__blk169_dn13: f64 = *var_vl0__blk169_dn13_slot;
        let mut var_vl0__blk169_dn2: f64 = *var_vl0__blk169_dn2_slot;
        let mut var_vl0__blk169_dn3: f64 = *var_vl0__blk169_dn3_slot;
        let mut var_vl0__blk169_dn4: f64 = *var_vl0__blk169_dn4_slot;
        let mut var_vl0__blk169_dn5: f64 = *var_vl0__blk169_dn5_slot;
        let mut var_vl0__blk169_dn6: f64 = *var_vl0__blk169_dn6_slot;
        let mut var_vl0__blk169_dn7: f64 = *var_vl0__blk169_dn7_slot;
        let mut var_vl0__blk169_dn8: f64 = *var_vl0__blk169_dn8_slot;
        let mut var_vl0__blk169_dn9: f64 = *var_vl0__blk169_dn9_slot;
        let mut var_vn0__blk167: f64 = *var_vn0__blk167_slot;
        let mut var_vn0__blk167_dn0: f64 = *var_vn0__blk167_dn0_slot;
        let mut var_vn0__blk167_dn1: f64 = *var_vn0__blk167_dn1_slot;
        let mut var_vn0__blk167_dn10: f64 = *var_vn0__blk167_dn10_slot;
        let mut var_vn0__blk167_dn11: f64 = *var_vn0__blk167_dn11_slot;
        let mut var_vn0__blk167_dn12: f64 = *var_vn0__blk167_dn12_slot;
        let mut var_vn0__blk167_dn13: f64 = *var_vn0__blk167_dn13_slot;
        let mut var_vn0__blk167_dn2: f64 = *var_vn0__blk167_dn2_slot;
        let mut var_vn0__blk167_dn3: f64 = *var_vn0__blk167_dn3_slot;
        let mut var_vn0__blk167_dn4: f64 = *var_vn0__blk167_dn4_slot;
        let mut var_vn0__blk167_dn5: f64 = *var_vn0__blk167_dn5_slot;
        let mut var_vn0__blk167_dn6: f64 = *var_vn0__blk167_dn6_slot;
        let mut var_vn0__blk167_dn7: f64 = *var_vn0__blk167_dn7_slot;
        let mut var_vn0__blk167_dn8: f64 = *var_vn0__blk167_dn8_slot;
        let mut var_vn0__blk167_dn9: f64 = *var_vn0__blk167_dn9_slot;
        let mut var_vn__blk171: f64 = *var_vn__blk171_slot;
        let mut var_vn__blk171_dn0: f64 = *var_vn__blk171_dn0_slot;
        let mut var_vn__blk171_dn1: f64 = *var_vn__blk171_dn1_slot;
        let mut var_vn__blk171_dn10: f64 = *var_vn__blk171_dn10_slot;
        let mut var_vn__blk171_dn11: f64 = *var_vn__blk171_dn11_slot;
        let mut var_vn__blk171_dn12: f64 = *var_vn__blk171_dn12_slot;
        let mut var_vn__blk171_dn13: f64 = *var_vn__blk171_dn13_slot;
        let mut var_vn__blk171_dn2: f64 = *var_vn__blk171_dn2_slot;
        let mut var_vn__blk171_dn3: f64 = *var_vn__blk171_dn3_slot;
        let mut var_vn__blk171_dn4: f64 = *var_vn__blk171_dn4_slot;
        let mut var_vn__blk171_dn5: f64 = *var_vn__blk171_dn5_slot;
        let mut var_vn__blk171_dn6: f64 = *var_vn__blk171_dn6_slot;
        let mut var_vn__blk171_dn7: f64 = *var_vn__blk171_dn7_slot;
        let mut var_vn__blk171_dn8: f64 = *var_vn__blk171_dn8_slot;
        let mut var_vn__blk171_dn9: f64 = *var_vn__blk171_dn9_slot;
        let mut var_vnl0__blk168: f64 = *var_vnl0__blk168_slot;
        let mut var_vnl0__blk168_dn0: f64 = *var_vnl0__blk168_dn0_slot;
        let mut var_vnl0__blk168_dn1: f64 = *var_vnl0__blk168_dn1_slot;
        let mut var_vnl0__blk168_dn10: f64 = *var_vnl0__blk168_dn10_slot;
        let mut var_vnl0__blk168_dn11: f64 = *var_vnl0__blk168_dn11_slot;
        let mut var_vnl0__blk168_dn12: f64 = *var_vnl0__blk168_dn12_slot;
        let mut var_vnl0__blk168_dn13: f64 = *var_vnl0__blk168_dn13_slot;
        let mut var_vnl0__blk168_dn2: f64 = *var_vnl0__blk168_dn2_slot;
        let mut var_vnl0__blk168_dn3: f64 = *var_vnl0__blk168_dn3_slot;
        let mut var_vnl0__blk168_dn4: f64 = *var_vnl0__blk168_dn4_slot;
        let mut var_vnl0__blk168_dn5: f64 = *var_vnl0__blk168_dn5_slot;
        let mut var_vnl0__blk168_dn6: f64 = *var_vnl0__blk168_dn6_slot;
        let mut var_vnl0__blk168_dn7: f64 = *var_vnl0__blk168_dn7_slot;
        let mut var_vnl0__blk168_dn8: f64 = *var_vnl0__blk168_dn8_slot;
        let mut var_vnl0__blk168_dn9: f64 = *var_vnl0__blk168_dn9_slot;
        let mut var_vnl__blk172: f64 = *var_vnl__blk172_slot;
        let mut var_vnl__blk172_dn0: f64 = *var_vnl__blk172_dn0_slot;
        let mut var_vnl__blk172_dn1: f64 = *var_vnl__blk172_dn1_slot;
        let mut var_vnl__blk172_dn10: f64 = *var_vnl__blk172_dn10_slot;
        let mut var_vnl__blk172_dn11: f64 = *var_vnl__blk172_dn11_slot;
        let mut var_vnl__blk172_dn12: f64 = *var_vnl__blk172_dn12_slot;
        let mut var_vnl__blk172_dn13: f64 = *var_vnl__blk172_dn13_slot;
        let mut var_vnl__blk172_dn2: f64 = *var_vnl__blk172_dn2_slot;
        let mut var_vnl__blk172_dn3: f64 = *var_vnl__blk172_dn3_slot;
        let mut var_vnl__blk172_dn4: f64 = *var_vnl__blk172_dn4_slot;
        let mut var_vnl__blk172_dn5: f64 = *var_vnl__blk172_dn5_slot;
        let mut var_vnl__blk172_dn6: f64 = *var_vnl__blk172_dn6_slot;
        let mut var_vnl__blk172_dn7: f64 = *var_vnl__blk172_dn7_slot;
        let mut var_vnl__blk172_dn8: f64 = *var_vnl__blk172_dn8_slot;
        let mut var_vnl__blk172_dn9: f64 = *var_vnl__blk172_dn9_slot;

        let (assign5110_e5496, assign5110_e5496_d_n0, assign5110_e5496_d_n1, assign5110_e5496_d_n2, assign5110_e5496_d_n3, assign5110_e5496_d_n4, assign5110_e5496_d_n5, assign5110_e5496_d_n6, assign5110_e5496_d_n7, assign5110_e5496_d_n8, assign5110_e5496_d_n9, assign5110_e5496_d_n10, assign5110_e5496_d_n11, assign5110_e5496_d_n12, assign5110_e5496_d_n13,) = {
    if (var_guard160 == 0.0) {
        let assign5110_e5480: f64 = (-var_pe_t);
        let assign5110_e5484: f64 = (var_vl__blk159 / var_pe_t);
        let assign5110_e5485: f64 = (1.0 - assign5110_e5484);
        let assign5110_e5488: f64 = (1.0 - p.p38);
        let assign5110_e5489: f64 = (assign5110_e5485).powf(assign5110_e5488);
        let assign5110_e5490: f64 = (assign5110_e5480 * assign5110_e5489);
        let assign5110_e5493: f64 = (1.0 - p.p38);
        let assign5110_e5494: f64 = (assign5110_e5490 / assign5110_e5493);
        (assign5110_e5494, ((((-var_pe_t_dn0) * assign5110_e5489) + (assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(((var_vl__blk159_dn0 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn0)) / (var_pe_t * var_pe_t))))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(((var_vl__blk159_dn0 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn0)) / (var_pe_t * var_pe_t))) / assign5110_e5485))) })) / assign5110_e5493), ((((-var_pe_t_dn1) * assign5110_e5489) + (assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(((var_vl__blk159_dn1 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn1)) / (var_pe_t * var_pe_t))))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(((var_vl__blk159_dn1 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn1)) / (var_pe_t * var_pe_t))) / assign5110_e5485))) })) / assign5110_e5493), ((((-var_pe_t_dn2) * assign5110_e5489) + (assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(((var_vl__blk159_dn2 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn2)) / (var_pe_t * var_pe_t))))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(((var_vl__blk159_dn2 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn2)) / (var_pe_t * var_pe_t))) / assign5110_e5485))) })) / assign5110_e5493), ((((-var_pe_t_dn3) * assign5110_e5489) + (assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(((var_vl__blk159_dn3 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn3)) / (var_pe_t * var_pe_t))))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(((var_vl__blk159_dn3 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn3)) / (var_pe_t * var_pe_t))) / assign5110_e5485))) })) / assign5110_e5493), ((((-var_pe_t_dn4) * assign5110_e5489) + (assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(((var_vl__blk159_dn4 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn4)) / (var_pe_t * var_pe_t))))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(((var_vl__blk159_dn4 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn4)) / (var_pe_t * var_pe_t))) / assign5110_e5485))) })) / assign5110_e5493), ((((-var_pe_t_dn5) * assign5110_e5489) + (assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(((var_vl__blk159_dn5 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn5)) / (var_pe_t * var_pe_t))))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(((var_vl__blk159_dn5 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn5)) / (var_pe_t * var_pe_t))) / assign5110_e5485))) })) / assign5110_e5493), ((((-var_pe_t_dn6) * assign5110_e5489) + (assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(((var_vl__blk159_dn6 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn6)) / (var_pe_t * var_pe_t))))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(((var_vl__blk159_dn6 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn6)) / (var_pe_t * var_pe_t))) / assign5110_e5485))) })) / assign5110_e5493), ((((-var_pe_t_dn7) * assign5110_e5489) + (assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(((var_vl__blk159_dn7 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn7)) / (var_pe_t * var_pe_t))))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(((var_vl__blk159_dn7 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn7)) / (var_pe_t * var_pe_t))) / assign5110_e5485))) })) / assign5110_e5493), ((((-var_pe_t_dn8) * assign5110_e5489) + (assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(((var_vl__blk159_dn8 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn8)) / (var_pe_t * var_pe_t))))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(((var_vl__blk159_dn8 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn8)) / (var_pe_t * var_pe_t))) / assign5110_e5485))) })) / assign5110_e5493), ((((-var_pe_t_dn9) * assign5110_e5489) + (assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(((var_vl__blk159_dn9 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn9)) / (var_pe_t * var_pe_t))))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(((var_vl__blk159_dn9 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn9)) / (var_pe_t * var_pe_t))) / assign5110_e5485))) })) / assign5110_e5493), ((((-var_pe_t_dn10) * assign5110_e5489) + (assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(((var_vl__blk159_dn10 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn10)) / (var_pe_t * var_pe_t))))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(((var_vl__blk159_dn10 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn10)) / (var_pe_t * var_pe_t))) / assign5110_e5485))) })) / assign5110_e5493), ((((-var_pe_t_dn11) * assign5110_e5489) + (assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(((var_vl__blk159_dn11 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn11)) / (var_pe_t * var_pe_t))))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(((var_vl__blk159_dn11 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn11)) / (var_pe_t * var_pe_t))) / assign5110_e5485))) })) / assign5110_e5493), ((((-var_pe_t_dn12) * assign5110_e5489) + (assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(((var_vl__blk159_dn12 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn12)) / (var_pe_t * var_pe_t))))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(((var_vl__blk159_dn12 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn12)) / (var_pe_t * var_pe_t))) / assign5110_e5485))) })) / assign5110_e5493), ((((-var_pe_t_dn13) * assign5110_e5489) + (assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(((var_vl__blk159_dn13 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn13)) / (var_pe_t * var_pe_t))))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(((var_vl__blk159_dn13 * var_pe_t) - (var_vl__blk159 * var_pe_t_dn13)) / (var_pe_t * var_pe_t))) / assign5110_e5485))) })) / assign5110_e5493),)
    } else {
        (var_qlo__blk152, var_qlo__blk152_dn0, var_qlo__blk152_dn1, var_qlo__blk152_dn2, var_qlo__blk152_dn3, var_qlo__blk152_dn4, var_qlo__blk152_dn5, var_qlo__blk152_dn6, var_qlo__blk152_dn7, var_qlo__blk152_dn8, var_qlo__blk152_dn9, var_qlo__blk152_dn10, var_qlo__blk152_dn11, var_qlo__blk152_dn12, var_qlo__blk152_dn13,)
    }
};
        var_qlo__blk152 = assign5110_e5496;
        var_qlo__blk152_dn0 = assign5110_e5496_d_n0;
        var_qlo__blk152_dn1 = assign5110_e5496_d_n1;
        var_qlo__blk152_dn2 = assign5110_e5496_d_n2;
        var_qlo__blk152_dn3 = assign5110_e5496_d_n3;
        var_qlo__blk152_dn4 = assign5110_e5496_d_n4;
        var_qlo__blk152_dn5 = assign5110_e5496_d_n5;
        var_qlo__blk152_dn6 = assign5110_e5496_d_n6;
        var_qlo__blk152_dn7 = assign5110_e5496_d_n7;
        var_qlo__blk152_dn8 = assign5110_e5496_d_n8;
        var_qlo__blk152_dn9 = assign5110_e5496_d_n9;
        var_qlo__blk152_dn10 = assign5110_e5496_d_n10;
        var_qlo__blk152_dn11 = assign5110_e5496_d_n11;
        var_qlo__blk152_dn12 = assign5110_e5496_d_n12;
        var_qlo__blk152_dn13 = assign5110_e5496_d_n13;

        let (assign5120_e5534, assign5120_e5534_d_n0, assign5120_e5534_d_n1, assign5120_e5534_d_n2, assign5120_e5534_d_n3, assign5120_e5534_d_n4, assign5120_e5534_d_n5, assign5120_e5534_d_n6, assign5120_e5534_d_n7, assign5120_e5534_d_n8, assign5120_e5534_d_n9, assign5120_e5534_d_n10, assign5120_e5534_d_n11, assign5120_e5534_d_n12, assign5120_e5534_d_n13,) = {
    if (var_guard160 == 0.0) {
        let assign5120_e5502: f64 = (1.0 - p.p34);
        let assign5120_e5504: f64 = (-p.p38);
        let assign5120_e5505: f64 = (assign5120_e5502).powf(assign5120_e5504);
        let assign5120_e5508: f64 = (var_vbex - var_vl__blk159);
        let assign5120_e5510: f64 = (assign5120_e5508 + var_vl0__blk155);
        let assign5120_e5511: f64 = (assign5120_e5505 * assign5120_e5510);
        let assign5120_e5515: f64 = (0.5 * p.p38);
        let assign5120_e5518: f64 = (var_vbex - var_vl__blk159);
        let assign5120_e5520: f64 = (assign5120_e5518 + var_vl0__blk155);
        let assign5120_e5521: f64 = (assign5120_e5515 * assign5120_e5520);
        let assign5120_e5525: f64 = (1.0 - p.p34);
        let assign5120_e5526: f64 = (var_pe_t * assign5120_e5525);
        let assign5120_e5527: f64 = (assign5120_e5521 / assign5120_e5526);
        let assign5120_e5528: f64 = (1.0 + assign5120_e5527);
        let assign5120_e5529: f64 = (assign5120_e5511 * assign5120_e5528);
        let assign5120_e5530: f64 = (var_qlo__blk152 + assign5120_e5529);
        let assign5120_e5532: f64 = (assign5120_e5530 - var_q0__blk156);
        (assign5120_e5532, ((var_qlo__blk152_dn0 + (((assign5120_e5505 * ((var_vbex_dn0 - var_vl__blk159_dn0) + var_vl0__blk155_dn0)) * assign5120_e5528) + (assign5120_e5511 * ((((assign5120_e5515 * ((var_vbex_dn0 - var_vl__blk159_dn0) + var_vl0__blk155_dn0)) * assign5120_e5526) - (assign5120_e5521 * (var_pe_t_dn0 * assign5120_e5525))) / (assign5120_e5526 * assign5120_e5526))))) - var_q0__blk156_dn0), ((var_qlo__blk152_dn1 + (((assign5120_e5505 * ((var_vbex_dn1 - var_vl__blk159_dn1) + var_vl0__blk155_dn1)) * assign5120_e5528) + (assign5120_e5511 * ((((assign5120_e5515 * ((var_vbex_dn1 - var_vl__blk159_dn1) + var_vl0__blk155_dn1)) * assign5120_e5526) - (assign5120_e5521 * (var_pe_t_dn1 * assign5120_e5525))) / (assign5120_e5526 * assign5120_e5526))))) - var_q0__blk156_dn1), ((var_qlo__blk152_dn2 + (((assign5120_e5505 * ((var_vbex_dn2 - var_vl__blk159_dn2) + var_vl0__blk155_dn2)) * assign5120_e5528) + (assign5120_e5511 * ((((assign5120_e5515 * ((var_vbex_dn2 - var_vl__blk159_dn2) + var_vl0__blk155_dn2)) * assign5120_e5526) - (assign5120_e5521 * (var_pe_t_dn2 * assign5120_e5525))) / (assign5120_e5526 * assign5120_e5526))))) - var_q0__blk156_dn2), ((var_qlo__blk152_dn3 + (((assign5120_e5505 * ((var_vbex_dn3 - var_vl__blk159_dn3) + var_vl0__blk155_dn3)) * assign5120_e5528) + (assign5120_e5511 * ((((assign5120_e5515 * ((var_vbex_dn3 - var_vl__blk159_dn3) + var_vl0__blk155_dn3)) * assign5120_e5526) - (assign5120_e5521 * (var_pe_t_dn3 * assign5120_e5525))) / (assign5120_e5526 * assign5120_e5526))))) - var_q0__blk156_dn3), ((var_qlo__blk152_dn4 + (((assign5120_e5505 * ((var_vbex_dn4 - var_vl__blk159_dn4) + var_vl0__blk155_dn4)) * assign5120_e5528) + (assign5120_e5511 * ((((assign5120_e5515 * ((var_vbex_dn4 - var_vl__blk159_dn4) + var_vl0__blk155_dn4)) * assign5120_e5526) - (assign5120_e5521 * (var_pe_t_dn4 * assign5120_e5525))) / (assign5120_e5526 * assign5120_e5526))))) - var_q0__blk156_dn4), ((var_qlo__blk152_dn5 + (((assign5120_e5505 * ((var_vbex_dn5 - var_vl__blk159_dn5) + var_vl0__blk155_dn5)) * assign5120_e5528) + (assign5120_e5511 * ((((assign5120_e5515 * ((var_vbex_dn5 - var_vl__blk159_dn5) + var_vl0__blk155_dn5)) * assign5120_e5526) - (assign5120_e5521 * (var_pe_t_dn5 * assign5120_e5525))) / (assign5120_e5526 * assign5120_e5526))))) - var_q0__blk156_dn5), ((var_qlo__blk152_dn6 + (((assign5120_e5505 * ((var_vbex_dn6 - var_vl__blk159_dn6) + var_vl0__blk155_dn6)) * assign5120_e5528) + (assign5120_e5511 * ((((assign5120_e5515 * ((var_vbex_dn6 - var_vl__blk159_dn6) + var_vl0__blk155_dn6)) * assign5120_e5526) - (assign5120_e5521 * (var_pe_t_dn6 * assign5120_e5525))) / (assign5120_e5526 * assign5120_e5526))))) - var_q0__blk156_dn6), ((var_qlo__blk152_dn7 + (((assign5120_e5505 * ((var_vbex_dn7 - var_vl__blk159_dn7) + var_vl0__blk155_dn7)) * assign5120_e5528) + (assign5120_e5511 * ((((assign5120_e5515 * ((var_vbex_dn7 - var_vl__blk159_dn7) + var_vl0__blk155_dn7)) * assign5120_e5526) - (assign5120_e5521 * (var_pe_t_dn7 * assign5120_e5525))) / (assign5120_e5526 * assign5120_e5526))))) - var_q0__blk156_dn7), ((var_qlo__blk152_dn8 + (((assign5120_e5505 * ((var_vbex_dn8 - var_vl__blk159_dn8) + var_vl0__blk155_dn8)) * assign5120_e5528) + (assign5120_e5511 * ((((assign5120_e5515 * ((var_vbex_dn8 - var_vl__blk159_dn8) + var_vl0__blk155_dn8)) * assign5120_e5526) - (assign5120_e5521 * (var_pe_t_dn8 * assign5120_e5525))) / (assign5120_e5526 * assign5120_e5526))))) - var_q0__blk156_dn8), ((var_qlo__blk152_dn9 + (((assign5120_e5505 * ((var_vbex_dn9 - var_vl__blk159_dn9) + var_vl0__blk155_dn9)) * assign5120_e5528) + (assign5120_e5511 * ((((assign5120_e5515 * ((var_vbex_dn9 - var_vl__blk159_dn9) + var_vl0__blk155_dn9)) * assign5120_e5526) - (assign5120_e5521 * (var_pe_t_dn9 * assign5120_e5525))) / (assign5120_e5526 * assign5120_e5526))))) - var_q0__blk156_dn9), ((var_qlo__blk152_dn10 + (((assign5120_e5505 * ((var_vbex_dn10 - var_vl__blk159_dn10) + var_vl0__blk155_dn10)) * assign5120_e5528) + (assign5120_e5511 * ((((assign5120_e5515 * ((var_vbex_dn10 - var_vl__blk159_dn10) + var_vl0__blk155_dn10)) * assign5120_e5526) - (assign5120_e5521 * (var_pe_t_dn10 * assign5120_e5525))) / (assign5120_e5526 * assign5120_e5526))))) - var_q0__blk156_dn10), ((var_qlo__blk152_dn11 + (((assign5120_e5505 * ((var_vbex_dn11 - var_vl__blk159_dn11) + var_vl0__blk155_dn11)) * assign5120_e5528) + (assign5120_e5511 * ((((assign5120_e5515 * ((var_vbex_dn11 - var_vl__blk159_dn11) + var_vl0__blk155_dn11)) * assign5120_e5526) - (assign5120_e5521 * (var_pe_t_dn11 * assign5120_e5525))) / (assign5120_e5526 * assign5120_e5526))))) - var_q0__blk156_dn11), ((var_qlo__blk152_dn12 + (((assign5120_e5505 * ((var_vbex_dn12 - var_vl__blk159_dn12) + var_vl0__blk155_dn12)) * assign5120_e5528) + (assign5120_e5511 * ((((assign5120_e5515 * ((var_vbex_dn12 - var_vl__blk159_dn12) + var_vl0__blk155_dn12)) * assign5120_e5526) - (assign5120_e5521 * (var_pe_t_dn12 * assign5120_e5525))) / (assign5120_e5526 * assign5120_e5526))))) - var_q0__blk156_dn12), ((var_qlo__blk152_dn13 + (((assign5120_e5505 * ((var_vbex_dn13 - var_vl__blk159_dn13) + var_vl0__blk155_dn13)) * assign5120_e5528) + (assign5120_e5511 * ((((assign5120_e5515 * ((var_vbex_dn13 - var_vl__blk159_dn13) + var_vl0__blk155_dn13)) * assign5120_e5526) - (assign5120_e5521 * (var_pe_t_dn13 * assign5120_e5525))) / (assign5120_e5526 * assign5120_e5526))))) - var_q0__blk156_dn13),)
    } else {
        (var_qdbex, var_qdbex_dn0, var_qdbex_dn1, var_qdbex_dn2, var_qdbex_dn3, var_qdbex_dn4, var_qdbex_dn5, var_qdbex_dn6, var_qdbex_dn7, var_qdbex_dn8, var_qdbex_dn9, var_qdbex_dn10, var_qdbex_dn11, var_qdbex_dn12, var_qdbex_dn13,)
    }
};
        var_qdbex = assign5120_e5534;
        var_qdbex_dn0 = assign5120_e5534_d_n0;
        var_qdbex_dn1 = assign5120_e5534_d_n1;
        var_qdbex_dn2 = assign5120_e5534_d_n2;
        var_qdbex_dn3 = assign5120_e5534_d_n3;
        var_qdbex_dn4 = assign5120_e5534_d_n4;
        var_qdbex_dn5 = assign5120_e5534_d_n5;
        var_qdbex_dn6 = assign5120_e5534_d_n6;
        var_qdbex_dn7 = assign5120_e5534_d_n7;
        var_qdbex_dn8 = assign5120_e5534_d_n8;
        var_qdbex_dn9 = assign5120_e5534_d_n9;
        var_qdbex_dn10 = assign5120_e5534_d_n10;
        var_qdbex_dn11 = assign5120_e5534_d_n11;
        var_qdbex_dn12 = assign5120_e5534_d_n12;
        var_qdbex_dn13 = assign5120_e5534_d_n13;

        let assign5130_e5536: f64 = (-var_pc_t);
        let assign5130_e5538: f64 = (assign5130_e5536 * p.p34);
        var_dv0__blk162 = assign5130_e5538;
        var_dv0__blk162_dn0 = ((-var_pc_t_dn0) * p.p34);
        var_dv0__blk162_dn1 = ((-var_pc_t_dn1) * p.p34);
        var_dv0__blk162_dn2 = ((-var_pc_t_dn2) * p.p34);
        var_dv0__blk162_dn3 = ((-var_pc_t_dn3) * p.p34);
        var_dv0__blk162_dn4 = ((-var_pc_t_dn4) * p.p34);
        var_dv0__blk162_dn5 = ((-var_pc_t_dn5) * p.p34);
        var_dv0__blk162_dn6 = ((-var_pc_t_dn6) * p.p34);
        var_dv0__blk162_dn7 = ((-var_pc_t_dn7) * p.p34);
        var_dv0__blk162_dn8 = ((-var_pc_t_dn8) * p.p34);
        var_dv0__blk162_dn9 = ((-var_pc_t_dn9) * p.p34);
        var_dv0__blk162_dn10 = ((-var_pc_t_dn10) * p.p34);
        var_dv0__blk162_dn11 = ((-var_pc_t_dn11) * p.p34);
        var_dv0__blk162_dn12 = ((-var_pc_t_dn12) * p.p34);
        var_dv0__blk162_dn13 = ((-var_pc_t_dn13) * p.p34);

        let assign5140_e5541: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        var_guard183 = assign5140_e5541;

        let (assign5150_e5547, assign5150_e5547_d_n0, assign5150_e5547_d_n1, assign5150_e5547_d_n2, assign5150_e5547_d_n3, assign5150_e5547_d_n4, assign5150_e5547_d_n5, assign5150_e5547_d_n6, assign5150_e5547_d_n7, assign5150_e5547_d_n8, assign5150_e5547_d_n9, assign5150_e5547_d_n10, assign5150_e5547_d_n11, assign5150_e5547_d_n12, assign5150_e5547_d_n13,) = {
    if (var_guard183 != 0.0) {
        let assign5150_e5545: f64 = (var_vbep + var_dv0__blk162);
        (assign5150_e5545, (var_vbep_dn0 + var_dv0__blk162_dn0), (var_vbep_dn1 + var_dv0__blk162_dn1), (var_vbep_dn2 + var_dv0__blk162_dn2), (var_vbep_dn3 + var_dv0__blk162_dn3), (var_vbep_dn4 + var_dv0__blk162_dn4), (var_vbep_dn5 + var_dv0__blk162_dn5), (var_vbep_dn6 + var_dv0__blk162_dn6), (var_vbep_dn7 + var_dv0__blk162_dn7), (var_vbep_dn8 + var_dv0__blk162_dn8), (var_vbep_dn9 + var_dv0__blk162_dn9), (var_vbep_dn10 + var_dv0__blk162_dn10), (var_vbep_dn11 + var_dv0__blk162_dn11), (var_vbep_dn12 + var_dv0__blk162_dn12), (var_vbep_dn13 + var_dv0__blk162_dn13),)
    } else {
        (var_dvh__blk163, var_dvh__blk163_dn0, var_dvh__blk163_dn1, var_dvh__blk163_dn2, var_dvh__blk163_dn3, var_dvh__blk163_dn4, var_dvh__blk163_dn5, var_dvh__blk163_dn6, var_dvh__blk163_dn7, var_dvh__blk163_dn8, var_dvh__blk163_dn9, var_dvh__blk163_dn10, var_dvh__blk163_dn11, var_dvh__blk163_dn12, var_dvh__blk163_dn13,)
    }
};
        var_dvh__blk163 = assign5150_e5547;
        var_dvh__blk163_dn0 = assign5150_e5547_d_n0;
        var_dvh__blk163_dn1 = assign5150_e5547_d_n1;
        var_dvh__blk163_dn2 = assign5150_e5547_d_n2;
        var_dvh__blk163_dn3 = assign5150_e5547_d_n3;
        var_dvh__blk163_dn4 = assign5150_e5547_d_n4;
        var_dvh__blk163_dn5 = assign5150_e5547_d_n5;
        var_dvh__blk163_dn6 = assign5150_e5547_d_n6;
        var_dvh__blk163_dn7 = assign5150_e5547_d_n7;
        var_dvh__blk163_dn8 = assign5150_e5547_d_n8;
        var_dvh__blk163_dn9 = assign5150_e5547_d_n9;
        var_dvh__blk163_dn10 = assign5150_e5547_d_n10;
        var_dvh__blk163_dn11 = assign5150_e5547_d_n11;
        var_dvh__blk163_dn12 = assign5150_e5547_d_n12;
        var_dvh__blk163_dn13 = assign5150_e5547_d_n13;

        let assign5160_e5550: f64 = if var_dvh__blk163 > 0.0 { 1.0 } else { 0.0 };
        var_guard184 = assign5160_e5550;

        let (assign5170_e5563,) = {
    if ((var_guard183 != 0.0) && (var_guard184 != 0.0)) {
        let assign5170_e5556: f64 = (1.0 - p.p34);
        let assign5170_e5558: f64 = (-1.0);
        let assign5170_e5560: f64 = (assign5170_e5558 - p.p43);
        let assign5170_e5561: f64 = (assign5170_e5556).powf(assign5170_e5560);
        (assign5170_e5561,)
    } else {
        (var_pwq__blk164,)
    }
};
        var_pwq__blk164 = assign5170_e5563;

        let (assign5180_e5585, assign5180_e5585_d_n0, assign5180_e5585_d_n1, assign5180_e5585_d_n2, assign5180_e5585_d_n3, assign5180_e5585_d_n4, assign5180_e5585_d_n5, assign5180_e5585_d_n6, assign5180_e5585_d_n7, assign5180_e5585_d_n8, assign5180_e5585_d_n9, assign5180_e5585_d_n10, assign5180_e5585_d_n11, assign5180_e5585_d_n12, assign5180_e5585_d_n13,) = {
    if ((var_guard183 != 0.0) && (var_guard184 != 0.0)) {
        let assign5180_e5572: f64 = (1.0 - p.p34);
        let assign5180_e5573: f64 = (var_pwq__blk164 * assign5180_e5572);
        let assign5180_e5576: f64 = (1.0 - p.p34);
        let assign5180_e5577: f64 = (assign5180_e5573 * assign5180_e5576);
        let assign5180_e5578: f64 = (1.0 - assign5180_e5577);
        let assign5180_e5579: f64 = (var_pc_t * assign5180_e5578);
        let assign5180_e5582: f64 = (1.0 - p.p43);
        let assign5180_e5583: f64 = (assign5180_e5579 / assign5180_e5582);
        (assign5180_e5583, ((var_pc_t_dn0 * assign5180_e5578) / assign5180_e5582), ((var_pc_t_dn1 * assign5180_e5578) / assign5180_e5582), ((var_pc_t_dn2 * assign5180_e5578) / assign5180_e5582), ((var_pc_t_dn3 * assign5180_e5578) / assign5180_e5582), ((var_pc_t_dn4 * assign5180_e5578) / assign5180_e5582), ((var_pc_t_dn5 * assign5180_e5578) / assign5180_e5582), ((var_pc_t_dn6 * assign5180_e5578) / assign5180_e5582), ((var_pc_t_dn7 * assign5180_e5578) / assign5180_e5582), ((var_pc_t_dn8 * assign5180_e5578) / assign5180_e5582), ((var_pc_t_dn9 * assign5180_e5578) / assign5180_e5582), ((var_pc_t_dn10 * assign5180_e5578) / assign5180_e5582), ((var_pc_t_dn11 * assign5180_e5578) / assign5180_e5582), ((var_pc_t_dn12 * assign5180_e5578) / assign5180_e5582), ((var_pc_t_dn13 * assign5180_e5578) / assign5180_e5582),)
    } else {
        (var_qlo__blk165, var_qlo__blk165_dn0, var_qlo__blk165_dn1, var_qlo__blk165_dn2, var_qlo__blk165_dn3, var_qlo__blk165_dn4, var_qlo__blk165_dn5, var_qlo__blk165_dn6, var_qlo__blk165_dn7, var_qlo__blk165_dn8, var_qlo__blk165_dn9, var_qlo__blk165_dn10, var_qlo__blk165_dn11, var_qlo__blk165_dn12, var_qlo__blk165_dn13,)
    }
};
        var_qlo__blk165 = assign5180_e5585;
        var_qlo__blk165_dn0 = assign5180_e5585_d_n0;
        var_qlo__blk165_dn1 = assign5180_e5585_d_n1;
        var_qlo__blk165_dn2 = assign5180_e5585_d_n2;
        var_qlo__blk165_dn3 = assign5180_e5585_d_n3;
        var_qlo__blk165_dn4 = assign5180_e5585_d_n4;
        var_qlo__blk165_dn5 = assign5180_e5585_d_n5;
        var_qlo__blk165_dn6 = assign5180_e5585_d_n6;
        var_qlo__blk165_dn7 = assign5180_e5585_d_n7;
        var_qlo__blk165_dn8 = assign5180_e5585_d_n8;
        var_qlo__blk165_dn9 = assign5180_e5585_d_n9;
        var_qlo__blk165_dn10 = assign5180_e5585_d_n10;
        var_qlo__blk165_dn11 = assign5180_e5585_d_n11;
        var_qlo__blk165_dn12 = assign5180_e5585_d_n12;
        var_qlo__blk165_dn13 = assign5180_e5585_d_n13;

        let (assign5190_e5605, assign5190_e5605_d_n0, assign5190_e5605_d_n1, assign5190_e5605_d_n2, assign5190_e5605_d_n3, assign5190_e5605_d_n4, assign5190_e5605_d_n5, assign5190_e5605_d_n6, assign5190_e5605_d_n7, assign5190_e5605_d_n8, assign5190_e5605_d_n9, assign5190_e5605_d_n10, assign5190_e5605_d_n11, assign5190_e5605_d_n12, assign5190_e5605_d_n13,) = {
    if ((var_guard183 != 0.0) && (var_guard184 != 0.0)) {
        let assign5190_e5592: f64 = (1.0 - p.p34);
        let assign5190_e5595: f64 = (0.5 * p.p43);
        let assign5190_e5597: f64 = (assign5190_e5595 * var_dvh__blk163);
        let assign5190_e5599: f64 = (assign5190_e5597 / var_pc_t);
        let assign5190_e5600: f64 = (assign5190_e5592 + assign5190_e5599);
        let assign5190_e5601: f64 = (var_dvh__blk163 * assign5190_e5600);
        let assign5190_e5603: f64 = (assign5190_e5601 * var_pwq__blk164);
        (assign5190_e5603, (((var_dvh__blk163_dn0 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn0) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn0)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164), (((var_dvh__blk163_dn1 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn1) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn1)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164), (((var_dvh__blk163_dn2 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn2) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn2)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164), (((var_dvh__blk163_dn3 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn3) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn3)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164), (((var_dvh__blk163_dn4 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn4) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn4)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164), (((var_dvh__blk163_dn5 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn5) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn5)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164), (((var_dvh__blk163_dn6 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn6) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn6)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164), (((var_dvh__blk163_dn7 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn7) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn7)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164), (((var_dvh__blk163_dn8 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn8) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn8)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164), (((var_dvh__blk163_dn9 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn9) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn9)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164), (((var_dvh__blk163_dn10 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn10) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn10)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164), (((var_dvh__blk163_dn11 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn11) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn11)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164), (((var_dvh__blk163_dn12 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn12) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn12)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164), (((var_dvh__blk163_dn13 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn13) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn13)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164),)
    } else {
        (var_qhi__blk166, var_qhi__blk166_dn0, var_qhi__blk166_dn1, var_qhi__blk166_dn2, var_qhi__blk166_dn3, var_qhi__blk166_dn4, var_qhi__blk166_dn5, var_qhi__blk166_dn6, var_qhi__blk166_dn7, var_qhi__blk166_dn8, var_qhi__blk166_dn9, var_qhi__blk166_dn10, var_qhi__blk166_dn11, var_qhi__blk166_dn12, var_qhi__blk166_dn13,)
    }
};
        var_qhi__blk166 = assign5190_e5605;
        var_qhi__blk166_dn0 = assign5190_e5605_d_n0;
        var_qhi__blk166_dn1 = assign5190_e5605_d_n1;
        var_qhi__blk166_dn2 = assign5190_e5605_d_n2;
        var_qhi__blk166_dn3 = assign5190_e5605_d_n3;
        var_qhi__blk166_dn4 = assign5190_e5605_d_n4;
        var_qhi__blk166_dn5 = assign5190_e5605_d_n5;
        var_qhi__blk166_dn6 = assign5190_e5605_d_n6;
        var_qhi__blk166_dn7 = assign5190_e5605_d_n7;
        var_qhi__blk166_dn8 = assign5190_e5605_d_n8;
        var_qhi__blk166_dn9 = assign5190_e5605_d_n9;
        var_qhi__blk166_dn10 = assign5190_e5605_d_n10;
        var_qhi__blk166_dn11 = assign5190_e5605_d_n11;
        var_qhi__blk166_dn12 = assign5190_e5605_d_n12;
        var_qhi__blk166_dn13 = assign5190_e5605_d_n13;

        let assign5200_e5611: f64 = (-p.p45);
        let assign5200_e5613: f64 = if ((p.p45 > 0.0) && (var_vbep < assign5200_e5611)) { 1.0 } else { 0.0 };
        var_guard185 = assign5200_e5613;

        let (assign5210_e5652, assign5210_e5652_d_n0, assign5210_e5652_d_n1, assign5210_e5652_d_n2, assign5210_e5652_d_n3, assign5210_e5652_d_n4, assign5210_e5652_d_n5, assign5210_e5652_d_n6, assign5210_e5652_d_n7, assign5210_e5652_d_n8, assign5210_e5652_d_n9, assign5210_e5652_d_n10, assign5210_e5652_d_n11, assign5210_e5652_d_n12, assign5210_e5652_d_n13,) = {
    if (((var_guard183 != 0.0) && (var_guard184 == 0.0)) && (var_guard185 != 0.0)) {
        let assign5210_e5625: f64 = (p.p45 / var_pc_t);
        let assign5210_e5626: f64 = (1.0 + assign5210_e5625);
        let assign5210_e5629: f64 = (1.0 - p.p43);
        let assign5210_e5630: f64 = (assign5210_e5626).powf(assign5210_e5629);
        let assign5210_e5634: f64 = (1.0 - p.p43);
        let assign5210_e5637: f64 = (var_vbep + p.p45);
        let assign5210_e5638: f64 = (assign5210_e5634 * assign5210_e5637);
        let assign5210_e5641: f64 = (var_pc_t + p.p45);
        let assign5210_e5642: f64 = (assign5210_e5638 / assign5210_e5641);
        let assign5210_e5643: f64 = (1.0 - assign5210_e5642);
        let assign5210_e5644: f64 = (assign5210_e5630 * assign5210_e5643);
        let assign5210_e5645: f64 = (1.0 - assign5210_e5644);
        let assign5210_e5646: f64 = (var_pc_t * assign5210_e5645);
        let assign5210_e5649: f64 = (1.0 - p.p43);
        let assign5210_e5650: f64 = (assign5210_e5646 / assign5210_e5649);
        (assign5210_e5650, (((var_pc_t_dn0 * assign5210_e5645) + (var_pc_t * (-((if 0.0 == 0.0 && ((assign5210_e5629) as f64).is_finite() && ((assign5210_e5629) as f64).fract() == 0.0 { if assign5210_e5629 == 0.0 { 0.0 } else { (assign5210_e5629 * ((assign5210_e5626).powf(assign5210_e5629 - 1.0) * (-((p.p45 * var_pc_t_dn0) / (var_pc_t * var_pc_t))))) } } else { (assign5210_e5630 * (assign5210_e5629 * ((-((p.p45 * var_pc_t_dn0) / (var_pc_t * var_pc_t))) / assign5210_e5626))) } * assign5210_e5643) + (assign5210_e5630 * (-((((assign5210_e5634 * var_vbep_dn0) * assign5210_e5641) - (assign5210_e5638 * var_pc_t_dn0)) / (assign5210_e5641 * assign5210_e5641)))))))) / assign5210_e5649), (((var_pc_t_dn1 * assign5210_e5645) + (var_pc_t * (-((if 0.0 == 0.0 && ((assign5210_e5629) as f64).is_finite() && ((assign5210_e5629) as f64).fract() == 0.0 { if assign5210_e5629 == 0.0 { 0.0 } else { (assign5210_e5629 * ((assign5210_e5626).powf(assign5210_e5629 - 1.0) * (-((p.p45 * var_pc_t_dn1) / (var_pc_t * var_pc_t))))) } } else { (assign5210_e5630 * (assign5210_e5629 * ((-((p.p45 * var_pc_t_dn1) / (var_pc_t * var_pc_t))) / assign5210_e5626))) } * assign5210_e5643) + (assign5210_e5630 * (-((((assign5210_e5634 * var_vbep_dn1) * assign5210_e5641) - (assign5210_e5638 * var_pc_t_dn1)) / (assign5210_e5641 * assign5210_e5641)))))))) / assign5210_e5649), (((var_pc_t_dn2 * assign5210_e5645) + (var_pc_t * (-((if 0.0 == 0.0 && ((assign5210_e5629) as f64).is_finite() && ((assign5210_e5629) as f64).fract() == 0.0 { if assign5210_e5629 == 0.0 { 0.0 } else { (assign5210_e5629 * ((assign5210_e5626).powf(assign5210_e5629 - 1.0) * (-((p.p45 * var_pc_t_dn2) / (var_pc_t * var_pc_t))))) } } else { (assign5210_e5630 * (assign5210_e5629 * ((-((p.p45 * var_pc_t_dn2) / (var_pc_t * var_pc_t))) / assign5210_e5626))) } * assign5210_e5643) + (assign5210_e5630 * (-((((assign5210_e5634 * var_vbep_dn2) * assign5210_e5641) - (assign5210_e5638 * var_pc_t_dn2)) / (assign5210_e5641 * assign5210_e5641)))))))) / assign5210_e5649), (((var_pc_t_dn3 * assign5210_e5645) + (var_pc_t * (-((if 0.0 == 0.0 && ((assign5210_e5629) as f64).is_finite() && ((assign5210_e5629) as f64).fract() == 0.0 { if assign5210_e5629 == 0.0 { 0.0 } else { (assign5210_e5629 * ((assign5210_e5626).powf(assign5210_e5629 - 1.0) * (-((p.p45 * var_pc_t_dn3) / (var_pc_t * var_pc_t))))) } } else { (assign5210_e5630 * (assign5210_e5629 * ((-((p.p45 * var_pc_t_dn3) / (var_pc_t * var_pc_t))) / assign5210_e5626))) } * assign5210_e5643) + (assign5210_e5630 * (-((((assign5210_e5634 * var_vbep_dn3) * assign5210_e5641) - (assign5210_e5638 * var_pc_t_dn3)) / (assign5210_e5641 * assign5210_e5641)))))))) / assign5210_e5649), (((var_pc_t_dn4 * assign5210_e5645) + (var_pc_t * (-((if 0.0 == 0.0 && ((assign5210_e5629) as f64).is_finite() && ((assign5210_e5629) as f64).fract() == 0.0 { if assign5210_e5629 == 0.0 { 0.0 } else { (assign5210_e5629 * ((assign5210_e5626).powf(assign5210_e5629 - 1.0) * (-((p.p45 * var_pc_t_dn4) / (var_pc_t * var_pc_t))))) } } else { (assign5210_e5630 * (assign5210_e5629 * ((-((p.p45 * var_pc_t_dn4) / (var_pc_t * var_pc_t))) / assign5210_e5626))) } * assign5210_e5643) + (assign5210_e5630 * (-((((assign5210_e5634 * var_vbep_dn4) * assign5210_e5641) - (assign5210_e5638 * var_pc_t_dn4)) / (assign5210_e5641 * assign5210_e5641)))))))) / assign5210_e5649), (((var_pc_t_dn5 * assign5210_e5645) + (var_pc_t * (-((if 0.0 == 0.0 && ((assign5210_e5629) as f64).is_finite() && ((assign5210_e5629) as f64).fract() == 0.0 { if assign5210_e5629 == 0.0 { 0.0 } else { (assign5210_e5629 * ((assign5210_e5626).powf(assign5210_e5629 - 1.0) * (-((p.p45 * var_pc_t_dn5) / (var_pc_t * var_pc_t))))) } } else { (assign5210_e5630 * (assign5210_e5629 * ((-((p.p45 * var_pc_t_dn5) / (var_pc_t * var_pc_t))) / assign5210_e5626))) } * assign5210_e5643) + (assign5210_e5630 * (-((((assign5210_e5634 * var_vbep_dn5) * assign5210_e5641) - (assign5210_e5638 * var_pc_t_dn5)) / (assign5210_e5641 * assign5210_e5641)))))))) / assign5210_e5649), (((var_pc_t_dn6 * assign5210_e5645) + (var_pc_t * (-((if 0.0 == 0.0 && ((assign5210_e5629) as f64).is_finite() && ((assign5210_e5629) as f64).fract() == 0.0 { if assign5210_e5629 == 0.0 { 0.0 } else { (assign5210_e5629 * ((assign5210_e5626).powf(assign5210_e5629 - 1.0) * (-((p.p45 * var_pc_t_dn6) / (var_pc_t * var_pc_t))))) } } else { (assign5210_e5630 * (assign5210_e5629 * ((-((p.p45 * var_pc_t_dn6) / (var_pc_t * var_pc_t))) / assign5210_e5626))) } * assign5210_e5643) + (assign5210_e5630 * (-((((assign5210_e5634 * var_vbep_dn6) * assign5210_e5641) - (assign5210_e5638 * var_pc_t_dn6)) / (assign5210_e5641 * assign5210_e5641)))))))) / assign5210_e5649), (((var_pc_t_dn7 * assign5210_e5645) + (var_pc_t * (-((if 0.0 == 0.0 && ((assign5210_e5629) as f64).is_finite() && ((assign5210_e5629) as f64).fract() == 0.0 { if assign5210_e5629 == 0.0 { 0.0 } else { (assign5210_e5629 * ((assign5210_e5626).powf(assign5210_e5629 - 1.0) * (-((p.p45 * var_pc_t_dn7) / (var_pc_t * var_pc_t))))) } } else { (assign5210_e5630 * (assign5210_e5629 * ((-((p.p45 * var_pc_t_dn7) / (var_pc_t * var_pc_t))) / assign5210_e5626))) } * assign5210_e5643) + (assign5210_e5630 * (-((((assign5210_e5634 * var_vbep_dn7) * assign5210_e5641) - (assign5210_e5638 * var_pc_t_dn7)) / (assign5210_e5641 * assign5210_e5641)))))))) / assign5210_e5649), (((var_pc_t_dn8 * assign5210_e5645) + (var_pc_t * (-((if 0.0 == 0.0 && ((assign5210_e5629) as f64).is_finite() && ((assign5210_e5629) as f64).fract() == 0.0 { if assign5210_e5629 == 0.0 { 0.0 } else { (assign5210_e5629 * ((assign5210_e5626).powf(assign5210_e5629 - 1.0) * (-((p.p45 * var_pc_t_dn8) / (var_pc_t * var_pc_t))))) } } else { (assign5210_e5630 * (assign5210_e5629 * ((-((p.p45 * var_pc_t_dn8) / (var_pc_t * var_pc_t))) / assign5210_e5626))) } * assign5210_e5643) + (assign5210_e5630 * (-((((assign5210_e5634 * var_vbep_dn8) * assign5210_e5641) - (assign5210_e5638 * var_pc_t_dn8)) / (assign5210_e5641 * assign5210_e5641)))))))) / assign5210_e5649), (((var_pc_t_dn9 * assign5210_e5645) + (var_pc_t * (-((if 0.0 == 0.0 && ((assign5210_e5629) as f64).is_finite() && ((assign5210_e5629) as f64).fract() == 0.0 { if assign5210_e5629 == 0.0 { 0.0 } else { (assign5210_e5629 * ((assign5210_e5626).powf(assign5210_e5629 - 1.0) * (-((p.p45 * var_pc_t_dn9) / (var_pc_t * var_pc_t))))) } } else { (assign5210_e5630 * (assign5210_e5629 * ((-((p.p45 * var_pc_t_dn9) / (var_pc_t * var_pc_t))) / assign5210_e5626))) } * assign5210_e5643) + (assign5210_e5630 * (-((((assign5210_e5634 * var_vbep_dn9) * assign5210_e5641) - (assign5210_e5638 * var_pc_t_dn9)) / (assign5210_e5641 * assign5210_e5641)))))))) / assign5210_e5649), (((var_pc_t_dn10 * assign5210_e5645) + (var_pc_t * (-((if 0.0 == 0.0 && ((assign5210_e5629) as f64).is_finite() && ((assign5210_e5629) as f64).fract() == 0.0 { if assign5210_e5629 == 0.0 { 0.0 } else { (assign5210_e5629 * ((assign5210_e5626).powf(assign5210_e5629 - 1.0) * (-((p.p45 * var_pc_t_dn10) / (var_pc_t * var_pc_t))))) } } else { (assign5210_e5630 * (assign5210_e5629 * ((-((p.p45 * var_pc_t_dn10) / (var_pc_t * var_pc_t))) / assign5210_e5626))) } * assign5210_e5643) + (assign5210_e5630 * (-((((assign5210_e5634 * var_vbep_dn10) * assign5210_e5641) - (assign5210_e5638 * var_pc_t_dn10)) / (assign5210_e5641 * assign5210_e5641)))))))) / assign5210_e5649), (((var_pc_t_dn11 * assign5210_e5645) + (var_pc_t * (-((if 0.0 == 0.0 && ((assign5210_e5629) as f64).is_finite() && ((assign5210_e5629) as f64).fract() == 0.0 { if assign5210_e5629 == 0.0 { 0.0 } else { (assign5210_e5629 * ((assign5210_e5626).powf(assign5210_e5629 - 1.0) * (-((p.p45 * var_pc_t_dn11) / (var_pc_t * var_pc_t))))) } } else { (assign5210_e5630 * (assign5210_e5629 * ((-((p.p45 * var_pc_t_dn11) / (var_pc_t * var_pc_t))) / assign5210_e5626))) } * assign5210_e5643) + (assign5210_e5630 * (-((((assign5210_e5634 * var_vbep_dn11) * assign5210_e5641) - (assign5210_e5638 * var_pc_t_dn11)) / (assign5210_e5641 * assign5210_e5641)))))))) / assign5210_e5649), (((var_pc_t_dn12 * assign5210_e5645) + (var_pc_t * (-((if 0.0 == 0.0 && ((assign5210_e5629) as f64).is_finite() && ((assign5210_e5629) as f64).fract() == 0.0 { if assign5210_e5629 == 0.0 { 0.0 } else { (assign5210_e5629 * ((assign5210_e5626).powf(assign5210_e5629 - 1.0) * (-((p.p45 * var_pc_t_dn12) / (var_pc_t * var_pc_t))))) } } else { (assign5210_e5630 * (assign5210_e5629 * ((-((p.p45 * var_pc_t_dn12) / (var_pc_t * var_pc_t))) / assign5210_e5626))) } * assign5210_e5643) + (assign5210_e5630 * (-((((assign5210_e5634 * var_vbep_dn12) * assign5210_e5641) - (assign5210_e5638 * var_pc_t_dn12)) / (assign5210_e5641 * assign5210_e5641)))))))) / assign5210_e5649), (((var_pc_t_dn13 * assign5210_e5645) + (var_pc_t * (-((if 0.0 == 0.0 && ((assign5210_e5629) as f64).is_finite() && ((assign5210_e5629) as f64).fract() == 0.0 { if assign5210_e5629 == 0.0 { 0.0 } else { (assign5210_e5629 * ((assign5210_e5626).powf(assign5210_e5629 - 1.0) * (-((p.p45 * var_pc_t_dn13) / (var_pc_t * var_pc_t))))) } } else { (assign5210_e5630 * (assign5210_e5629 * ((-((p.p45 * var_pc_t_dn13) / (var_pc_t * var_pc_t))) / assign5210_e5626))) } * assign5210_e5643) + (assign5210_e5630 * (-((((assign5210_e5634 * var_vbep_dn13) * assign5210_e5641) - (assign5210_e5638 * var_pc_t_dn13)) / (assign5210_e5641 * assign5210_e5641)))))))) / assign5210_e5649),)
    } else {
        (var_qlo__blk165, var_qlo__blk165_dn0, var_qlo__blk165_dn1, var_qlo__blk165_dn2, var_qlo__blk165_dn3, var_qlo__blk165_dn4, var_qlo__blk165_dn5, var_qlo__blk165_dn6, var_qlo__blk165_dn7, var_qlo__blk165_dn8, var_qlo__blk165_dn9, var_qlo__blk165_dn10, var_qlo__blk165_dn11, var_qlo__blk165_dn12, var_qlo__blk165_dn13,)
    }
};
        var_qlo__blk165 = assign5210_e5652;
        var_qlo__blk165_dn0 = assign5210_e5652_d_n0;
        var_qlo__blk165_dn1 = assign5210_e5652_d_n1;
        var_qlo__blk165_dn2 = assign5210_e5652_d_n2;
        var_qlo__blk165_dn3 = assign5210_e5652_d_n3;
        var_qlo__blk165_dn4 = assign5210_e5652_d_n4;
        var_qlo__blk165_dn5 = assign5210_e5652_d_n5;
        var_qlo__blk165_dn6 = assign5210_e5652_d_n6;
        var_qlo__blk165_dn7 = assign5210_e5652_d_n7;
        var_qlo__blk165_dn8 = assign5210_e5652_d_n8;
        var_qlo__blk165_dn9 = assign5210_e5652_d_n9;
        var_qlo__blk165_dn10 = assign5210_e5652_d_n10;
        var_qlo__blk165_dn11 = assign5210_e5652_d_n11;
        var_qlo__blk165_dn12 = assign5210_e5652_d_n12;
        var_qlo__blk165_dn13 = assign5210_e5652_d_n13;

        let (assign5220_e5678, assign5220_e5678_d_n0, assign5220_e5678_d_n1, assign5220_e5678_d_n2, assign5220_e5678_d_n3, assign5220_e5678_d_n4, assign5220_e5678_d_n5, assign5220_e5678_d_n6, assign5220_e5678_d_n7, assign5220_e5678_d_n8, assign5220_e5678_d_n9, assign5220_e5678_d_n10, assign5220_e5678_d_n11, assign5220_e5678_d_n12, assign5220_e5678_d_n13,) = {
    if (((var_guard183 != 0.0) && (var_guard184 == 0.0)) && (var_guard185 == 0.0)) {
        let assign5220_e5665: f64 = (var_vbep / var_pc_t);
        let assign5220_e5666: f64 = (1.0 - assign5220_e5665);
        let assign5220_e5669: f64 = (1.0 - p.p43);
        let assign5220_e5670: f64 = (assign5220_e5666).powf(assign5220_e5669);
        let assign5220_e5671: f64 = (1.0 - assign5220_e5670);
        let assign5220_e5672: f64 = (var_pc_t * assign5220_e5671);
        let assign5220_e5675: f64 = (1.0 - p.p43);
        let assign5220_e5676: f64 = (assign5220_e5672 / assign5220_e5675);
        (assign5220_e5676, (((var_pc_t_dn0 * assign5220_e5671) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(((var_vbep_dn0 * var_pc_t) - (var_vbep * var_pc_t_dn0)) / (var_pc_t * var_pc_t))))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(((var_vbep_dn0 * var_pc_t) - (var_vbep * var_pc_t_dn0)) / (var_pc_t * var_pc_t))) / assign5220_e5666))) }))) / assign5220_e5675), (((var_pc_t_dn1 * assign5220_e5671) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(((var_vbep_dn1 * var_pc_t) - (var_vbep * var_pc_t_dn1)) / (var_pc_t * var_pc_t))))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(((var_vbep_dn1 * var_pc_t) - (var_vbep * var_pc_t_dn1)) / (var_pc_t * var_pc_t))) / assign5220_e5666))) }))) / assign5220_e5675), (((var_pc_t_dn2 * assign5220_e5671) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(((var_vbep_dn2 * var_pc_t) - (var_vbep * var_pc_t_dn2)) / (var_pc_t * var_pc_t))))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(((var_vbep_dn2 * var_pc_t) - (var_vbep * var_pc_t_dn2)) / (var_pc_t * var_pc_t))) / assign5220_e5666))) }))) / assign5220_e5675), (((var_pc_t_dn3 * assign5220_e5671) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(((var_vbep_dn3 * var_pc_t) - (var_vbep * var_pc_t_dn3)) / (var_pc_t * var_pc_t))))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(((var_vbep_dn3 * var_pc_t) - (var_vbep * var_pc_t_dn3)) / (var_pc_t * var_pc_t))) / assign5220_e5666))) }))) / assign5220_e5675), (((var_pc_t_dn4 * assign5220_e5671) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(((var_vbep_dn4 * var_pc_t) - (var_vbep * var_pc_t_dn4)) / (var_pc_t * var_pc_t))))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(((var_vbep_dn4 * var_pc_t) - (var_vbep * var_pc_t_dn4)) / (var_pc_t * var_pc_t))) / assign5220_e5666))) }))) / assign5220_e5675), (((var_pc_t_dn5 * assign5220_e5671) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(((var_vbep_dn5 * var_pc_t) - (var_vbep * var_pc_t_dn5)) / (var_pc_t * var_pc_t))))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(((var_vbep_dn5 * var_pc_t) - (var_vbep * var_pc_t_dn5)) / (var_pc_t * var_pc_t))) / assign5220_e5666))) }))) / assign5220_e5675), (((var_pc_t_dn6 * assign5220_e5671) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(((var_vbep_dn6 * var_pc_t) - (var_vbep * var_pc_t_dn6)) / (var_pc_t * var_pc_t))))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(((var_vbep_dn6 * var_pc_t) - (var_vbep * var_pc_t_dn6)) / (var_pc_t * var_pc_t))) / assign5220_e5666))) }))) / assign5220_e5675), (((var_pc_t_dn7 * assign5220_e5671) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(((var_vbep_dn7 * var_pc_t) - (var_vbep * var_pc_t_dn7)) / (var_pc_t * var_pc_t))))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(((var_vbep_dn7 * var_pc_t) - (var_vbep * var_pc_t_dn7)) / (var_pc_t * var_pc_t))) / assign5220_e5666))) }))) / assign5220_e5675), (((var_pc_t_dn8 * assign5220_e5671) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(((var_vbep_dn8 * var_pc_t) - (var_vbep * var_pc_t_dn8)) / (var_pc_t * var_pc_t))))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(((var_vbep_dn8 * var_pc_t) - (var_vbep * var_pc_t_dn8)) / (var_pc_t * var_pc_t))) / assign5220_e5666))) }))) / assign5220_e5675), (((var_pc_t_dn9 * assign5220_e5671) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(((var_vbep_dn9 * var_pc_t) - (var_vbep * var_pc_t_dn9)) / (var_pc_t * var_pc_t))))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(((var_vbep_dn9 * var_pc_t) - (var_vbep * var_pc_t_dn9)) / (var_pc_t * var_pc_t))) / assign5220_e5666))) }))) / assign5220_e5675), (((var_pc_t_dn10 * assign5220_e5671) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(((var_vbep_dn10 * var_pc_t) - (var_vbep * var_pc_t_dn10)) / (var_pc_t * var_pc_t))))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(((var_vbep_dn10 * var_pc_t) - (var_vbep * var_pc_t_dn10)) / (var_pc_t * var_pc_t))) / assign5220_e5666))) }))) / assign5220_e5675), (((var_pc_t_dn11 * assign5220_e5671) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(((var_vbep_dn11 * var_pc_t) - (var_vbep * var_pc_t_dn11)) / (var_pc_t * var_pc_t))))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(((var_vbep_dn11 * var_pc_t) - (var_vbep * var_pc_t_dn11)) / (var_pc_t * var_pc_t))) / assign5220_e5666))) }))) / assign5220_e5675), (((var_pc_t_dn12 * assign5220_e5671) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(((var_vbep_dn12 * var_pc_t) - (var_vbep * var_pc_t_dn12)) / (var_pc_t * var_pc_t))))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(((var_vbep_dn12 * var_pc_t) - (var_vbep * var_pc_t_dn12)) / (var_pc_t * var_pc_t))) / assign5220_e5666))) }))) / assign5220_e5675), (((var_pc_t_dn13 * assign5220_e5671) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(((var_vbep_dn13 * var_pc_t) - (var_vbep * var_pc_t_dn13)) / (var_pc_t * var_pc_t))))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(((var_vbep_dn13 * var_pc_t) - (var_vbep * var_pc_t_dn13)) / (var_pc_t * var_pc_t))) / assign5220_e5666))) }))) / assign5220_e5675),)
    } else {
        (var_qlo__blk165, var_qlo__blk165_dn0, var_qlo__blk165_dn1, var_qlo__blk165_dn2, var_qlo__blk165_dn3, var_qlo__blk165_dn4, var_qlo__blk165_dn5, var_qlo__blk165_dn6, var_qlo__blk165_dn7, var_qlo__blk165_dn8, var_qlo__blk165_dn9, var_qlo__blk165_dn10, var_qlo__blk165_dn11, var_qlo__blk165_dn12, var_qlo__blk165_dn13,)
    }
};
        var_qlo__blk165 = assign5220_e5678;
        var_qlo__blk165_dn0 = assign5220_e5678_d_n0;
        var_qlo__blk165_dn1 = assign5220_e5678_d_n1;
        var_qlo__blk165_dn2 = assign5220_e5678_d_n2;
        var_qlo__blk165_dn3 = assign5220_e5678_d_n3;
        var_qlo__blk165_dn4 = assign5220_e5678_d_n4;
        var_qlo__blk165_dn5 = assign5220_e5678_d_n5;
        var_qlo__blk165_dn6 = assign5220_e5678_d_n6;
        var_qlo__blk165_dn7 = assign5220_e5678_d_n7;
        var_qlo__blk165_dn8 = assign5220_e5678_d_n8;
        var_qlo__blk165_dn9 = assign5220_e5678_d_n9;
        var_qlo__blk165_dn10 = assign5220_e5678_d_n10;
        var_qlo__blk165_dn11 = assign5220_e5678_d_n11;
        var_qlo__blk165_dn12 = assign5220_e5678_d_n12;
        var_qlo__blk165_dn13 = assign5220_e5678_d_n13;

        let (assign5230_e5685, assign5230_e5685_d_n0, assign5230_e5685_d_n1, assign5230_e5685_d_n2, assign5230_e5685_d_n3, assign5230_e5685_d_n4, assign5230_e5685_d_n5, assign5230_e5685_d_n6, assign5230_e5685_d_n7, assign5230_e5685_d_n8, assign5230_e5685_d_n9, assign5230_e5685_d_n10, assign5230_e5685_d_n11, assign5230_e5685_d_n12, assign5230_e5685_d_n13,) = {
    if ((var_guard183 != 0.0) && (var_guard184 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi__blk166, var_qhi__blk166_dn0, var_qhi__blk166_dn1, var_qhi__blk166_dn2, var_qhi__blk166_dn3, var_qhi__blk166_dn4, var_qhi__blk166_dn5, var_qhi__blk166_dn6, var_qhi__blk166_dn7, var_qhi__blk166_dn8, var_qhi__blk166_dn9, var_qhi__blk166_dn10, var_qhi__blk166_dn11, var_qhi__blk166_dn12, var_qhi__blk166_dn13,)
    }
};
        var_qhi__blk166 = assign5230_e5685;
        var_qhi__blk166_dn0 = assign5230_e5685_d_n0;
        var_qhi__blk166_dn1 = assign5230_e5685_d_n1;
        var_qhi__blk166_dn2 = assign5230_e5685_d_n2;
        var_qhi__blk166_dn3 = assign5230_e5685_d_n3;
        var_qhi__blk166_dn4 = assign5230_e5685_d_n4;
        var_qhi__blk166_dn5 = assign5230_e5685_d_n5;
        var_qhi__blk166_dn6 = assign5230_e5685_d_n6;
        var_qhi__blk166_dn7 = assign5230_e5685_d_n7;
        var_qhi__blk166_dn8 = assign5230_e5685_d_n8;
        var_qhi__blk166_dn9 = assign5230_e5685_d_n9;
        var_qhi__blk166_dn10 = assign5230_e5685_d_n10;
        var_qhi__blk166_dn11 = assign5230_e5685_d_n11;
        var_qhi__blk166_dn12 = assign5230_e5685_d_n12;
        var_qhi__blk166_dn13 = assign5230_e5685_d_n13;

        let (assign5240_e5691, assign5240_e5691_d_n0, assign5240_e5691_d_n1, assign5240_e5691_d_n2, assign5240_e5691_d_n3, assign5240_e5691_d_n4, assign5240_e5691_d_n5, assign5240_e5691_d_n6, assign5240_e5691_d_n7, assign5240_e5691_d_n8, assign5240_e5691_d_n9, assign5240_e5691_d_n10, assign5240_e5691_d_n11, assign5240_e5691_d_n12, assign5240_e5691_d_n13,) = {
    if (var_guard183 != 0.0) {
        let assign5240_e5689: f64 = (var_qlo__blk165 + var_qhi__blk166);
        (assign5240_e5689, (var_qlo__blk165_dn0 + var_qhi__blk166_dn0), (var_qlo__blk165_dn1 + var_qhi__blk166_dn1), (var_qlo__blk165_dn2 + var_qhi__blk166_dn2), (var_qlo__blk165_dn3 + var_qhi__blk166_dn3), (var_qlo__blk165_dn4 + var_qhi__blk166_dn4), (var_qlo__blk165_dn5 + var_qhi__blk166_dn5), (var_qlo__blk165_dn6 + var_qhi__blk166_dn6), (var_qlo__blk165_dn7 + var_qhi__blk166_dn7), (var_qlo__blk165_dn8 + var_qhi__blk166_dn8), (var_qlo__blk165_dn9 + var_qhi__blk166_dn9), (var_qlo__blk165_dn10 + var_qhi__blk166_dn10), (var_qlo__blk165_dn11 + var_qhi__blk166_dn11), (var_qlo__blk165_dn12 + var_qhi__blk166_dn12), (var_qlo__blk165_dn13 + var_qhi__blk166_dn13),)
    } else {
        (var_qdbep, var_qdbep_dn0, var_qdbep_dn1, var_qdbep_dn2, var_qdbep_dn3, var_qdbep_dn4, var_qdbep_dn5, var_qdbep_dn6, var_qdbep_dn7, var_qdbep_dn8, var_qdbep_dn9, var_qdbep_dn10, var_qdbep_dn11, var_qdbep_dn12, var_qdbep_dn13,)
    }
};
        var_qdbep = assign5240_e5691;
        var_qdbep_dn0 = assign5240_e5691_d_n0;
        var_qdbep_dn1 = assign5240_e5691_d_n1;
        var_qdbep_dn2 = assign5240_e5691_d_n2;
        var_qdbep_dn3 = assign5240_e5691_d_n3;
        var_qdbep_dn4 = assign5240_e5691_d_n4;
        var_qdbep_dn5 = assign5240_e5691_d_n5;
        var_qdbep_dn6 = assign5240_e5691_d_n6;
        var_qdbep_dn7 = assign5240_e5691_d_n7;
        var_qdbep_dn8 = assign5240_e5691_d_n8;
        var_qdbep_dn9 = assign5240_e5691_d_n9;
        var_qdbep_dn10 = assign5240_e5691_d_n10;
        var_qdbep_dn11 = assign5240_e5691_d_n11;
        var_qdbep_dn12 = assign5240_e5691_d_n12;
        var_qdbep_dn13 = assign5240_e5691_d_n13;

        let assign5250_e5698: f64 = if ((p.p45 > 0.0) && (p.p46 > 0.0)) { 1.0 } else { 0.0 };
        var_guard186 = assign5250_e5698;

        let (assign5260_e5711, assign5260_e5711_d_n0, assign5260_e5711_d_n1, assign5260_e5711_d_n2, assign5260_e5711_d_n3, assign5260_e5711_d_n4, assign5260_e5711_d_n5, assign5260_e5711_d_n6, assign5260_e5711_d_n7, assign5260_e5711_d_n8, assign5260_e5711_d_n9, assign5260_e5711_d_n10, assign5260_e5711_d_n11, assign5260_e5711_d_n12, assign5260_e5711_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 != 0.0)) {
        let assign5260_e5705: f64 = (p.p45 + var_dv0__blk162);
        let assign5260_e5708: f64 = (p.p45 - var_dv0__blk162);
        let assign5260_e5709: f64 = (assign5260_e5705 / assign5260_e5708);
        (assign5260_e5709, (((var_dv0__blk162_dn0 * assign5260_e5708) - (assign5260_e5705 * (-var_dv0__blk162_dn0))) / (assign5260_e5708 * assign5260_e5708)), (((var_dv0__blk162_dn1 * assign5260_e5708) - (assign5260_e5705 * (-var_dv0__blk162_dn1))) / (assign5260_e5708 * assign5260_e5708)), (((var_dv0__blk162_dn2 * assign5260_e5708) - (assign5260_e5705 * (-var_dv0__blk162_dn2))) / (assign5260_e5708 * assign5260_e5708)), (((var_dv0__blk162_dn3 * assign5260_e5708) - (assign5260_e5705 * (-var_dv0__blk162_dn3))) / (assign5260_e5708 * assign5260_e5708)), (((var_dv0__blk162_dn4 * assign5260_e5708) - (assign5260_e5705 * (-var_dv0__blk162_dn4))) / (assign5260_e5708 * assign5260_e5708)), (((var_dv0__blk162_dn5 * assign5260_e5708) - (assign5260_e5705 * (-var_dv0__blk162_dn5))) / (assign5260_e5708 * assign5260_e5708)), (((var_dv0__blk162_dn6 * assign5260_e5708) - (assign5260_e5705 * (-var_dv0__blk162_dn6))) / (assign5260_e5708 * assign5260_e5708)), (((var_dv0__blk162_dn7 * assign5260_e5708) - (assign5260_e5705 * (-var_dv0__blk162_dn7))) / (assign5260_e5708 * assign5260_e5708)), (((var_dv0__blk162_dn8 * assign5260_e5708) - (assign5260_e5705 * (-var_dv0__blk162_dn8))) / (assign5260_e5708 * assign5260_e5708)), (((var_dv0__blk162_dn9 * assign5260_e5708) - (assign5260_e5705 * (-var_dv0__blk162_dn9))) / (assign5260_e5708 * assign5260_e5708)), (((var_dv0__blk162_dn10 * assign5260_e5708) - (assign5260_e5705 * (-var_dv0__blk162_dn10))) / (assign5260_e5708 * assign5260_e5708)), (((var_dv0__blk162_dn11 * assign5260_e5708) - (assign5260_e5705 * (-var_dv0__blk162_dn11))) / (assign5260_e5708 * assign5260_e5708)), (((var_dv0__blk162_dn12 * assign5260_e5708) - (assign5260_e5705 * (-var_dv0__blk162_dn12))) / (assign5260_e5708 * assign5260_e5708)), (((var_dv0__blk162_dn13 * assign5260_e5708) - (assign5260_e5705 * (-var_dv0__blk162_dn13))) / (assign5260_e5708 * assign5260_e5708)),)
    } else {
        (var_vn0__blk167, var_vn0__blk167_dn0, var_vn0__blk167_dn1, var_vn0__blk167_dn2, var_vn0__blk167_dn3, var_vn0__blk167_dn4, var_vn0__blk167_dn5, var_vn0__blk167_dn6, var_vn0__blk167_dn7, var_vn0__blk167_dn8, var_vn0__blk167_dn9, var_vn0__blk167_dn10, var_vn0__blk167_dn11, var_vn0__blk167_dn12, var_vn0__blk167_dn13,)
    }
};
        var_vn0__blk167 = assign5260_e5711;
        var_vn0__blk167_dn0 = assign5260_e5711_d_n0;
        var_vn0__blk167_dn1 = assign5260_e5711_d_n1;
        var_vn0__blk167_dn2 = assign5260_e5711_d_n2;
        var_vn0__blk167_dn3 = assign5260_e5711_d_n3;
        var_vn0__blk167_dn4 = assign5260_e5711_d_n4;
        var_vn0__blk167_dn5 = assign5260_e5711_d_n5;
        var_vn0__blk167_dn6 = assign5260_e5711_d_n6;
        var_vn0__blk167_dn7 = assign5260_e5711_d_n7;
        var_vn0__blk167_dn8 = assign5260_e5711_d_n8;
        var_vn0__blk167_dn9 = assign5260_e5711_d_n9;
        var_vn0__blk167_dn10 = assign5260_e5711_d_n10;
        var_vn0__blk167_dn11 = assign5260_e5711_d_n11;
        var_vn0__blk167_dn12 = assign5260_e5711_d_n12;
        var_vn0__blk167_dn13 = assign5260_e5711_d_n13;

        let (assign5270_e5750, assign5270_e5750_d_n0, assign5270_e5750_d_n1, assign5270_e5750_d_n2, assign5270_e5750_d_n3, assign5270_e5750_d_n4, assign5270_e5750_d_n5, assign5270_e5750_d_n6, assign5270_e5750_d_n7, assign5270_e5750_d_n8, assign5270_e5750_d_n9, assign5270_e5750_d_n10, assign5270_e5750_d_n11, assign5270_e5750_d_n12, assign5270_e5750_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 != 0.0)) {
        let assign5270_e5718: f64 = (2.0 * var_vn0__blk167);
        let assign5270_e5721: f64 = (var_vn0__blk167 - 1.0);
        let assign5270_e5724: f64 = (var_vn0__blk167 - 1.0);
        let assign5270_e5725: f64 = (assign5270_e5721 * assign5270_e5724);
        let assign5270_e5728: f64 = (4.0 * p.p44);
        let assign5270_e5730: f64 = (assign5270_e5728 * p.p44);
        let assign5270_e5731: f64 = (assign5270_e5725 + assign5270_e5730);
        let assign5270_e5732: f64 = (assign5270_e5731).sqrt();
        let assign5270_e5735: f64 = (var_vn0__blk167 + 1.0);
        let assign5270_e5738: f64 = (var_vn0__blk167 + 1.0);
        let assign5270_e5739: f64 = (assign5270_e5735 * assign5270_e5738);
        let assign5270_e5742: f64 = (4.0 * p.p46);
        let assign5270_e5744: f64 = (assign5270_e5742 * p.p46);
        let assign5270_e5745: f64 = (assign5270_e5739 + assign5270_e5744);
        let assign5270_e5746: f64 = (assign5270_e5745).sqrt();
        let assign5270_e5747: f64 = (assign5270_e5732 + assign5270_e5746);
        let assign5270_e5748: f64 = (assign5270_e5718 / assign5270_e5747);
        (assign5270_e5748, ((((2.0 * var_vn0__blk167_dn0) * assign5270_e5747) - (assign5270_e5718 * ((((var_vn0__blk167_dn0 * assign5270_e5724) + (assign5270_e5721 * var_vn0__blk167_dn0)) / (2.0 * assign5270_e5732)) + (((var_vn0__blk167_dn0 * assign5270_e5738) + (assign5270_e5735 * var_vn0__blk167_dn0)) / (2.0 * assign5270_e5746))))) / (assign5270_e5747 * assign5270_e5747)), ((((2.0 * var_vn0__blk167_dn1) * assign5270_e5747) - (assign5270_e5718 * ((((var_vn0__blk167_dn1 * assign5270_e5724) + (assign5270_e5721 * var_vn0__blk167_dn1)) / (2.0 * assign5270_e5732)) + (((var_vn0__blk167_dn1 * assign5270_e5738) + (assign5270_e5735 * var_vn0__blk167_dn1)) / (2.0 * assign5270_e5746))))) / (assign5270_e5747 * assign5270_e5747)), ((((2.0 * var_vn0__blk167_dn2) * assign5270_e5747) - (assign5270_e5718 * ((((var_vn0__blk167_dn2 * assign5270_e5724) + (assign5270_e5721 * var_vn0__blk167_dn2)) / (2.0 * assign5270_e5732)) + (((var_vn0__blk167_dn2 * assign5270_e5738) + (assign5270_e5735 * var_vn0__blk167_dn2)) / (2.0 * assign5270_e5746))))) / (assign5270_e5747 * assign5270_e5747)), ((((2.0 * var_vn0__blk167_dn3) * assign5270_e5747) - (assign5270_e5718 * ((((var_vn0__blk167_dn3 * assign5270_e5724) + (assign5270_e5721 * var_vn0__blk167_dn3)) / (2.0 * assign5270_e5732)) + (((var_vn0__blk167_dn3 * assign5270_e5738) + (assign5270_e5735 * var_vn0__blk167_dn3)) / (2.0 * assign5270_e5746))))) / (assign5270_e5747 * assign5270_e5747)), ((((2.0 * var_vn0__blk167_dn4) * assign5270_e5747) - (assign5270_e5718 * ((((var_vn0__blk167_dn4 * assign5270_e5724) + (assign5270_e5721 * var_vn0__blk167_dn4)) / (2.0 * assign5270_e5732)) + (((var_vn0__blk167_dn4 * assign5270_e5738) + (assign5270_e5735 * var_vn0__blk167_dn4)) / (2.0 * assign5270_e5746))))) / (assign5270_e5747 * assign5270_e5747)), ((((2.0 * var_vn0__blk167_dn5) * assign5270_e5747) - (assign5270_e5718 * ((((var_vn0__blk167_dn5 * assign5270_e5724) + (assign5270_e5721 * var_vn0__blk167_dn5)) / (2.0 * assign5270_e5732)) + (((var_vn0__blk167_dn5 * assign5270_e5738) + (assign5270_e5735 * var_vn0__blk167_dn5)) / (2.0 * assign5270_e5746))))) / (assign5270_e5747 * assign5270_e5747)), ((((2.0 * var_vn0__blk167_dn6) * assign5270_e5747) - (assign5270_e5718 * ((((var_vn0__blk167_dn6 * assign5270_e5724) + (assign5270_e5721 * var_vn0__blk167_dn6)) / (2.0 * assign5270_e5732)) + (((var_vn0__blk167_dn6 * assign5270_e5738) + (assign5270_e5735 * var_vn0__blk167_dn6)) / (2.0 * assign5270_e5746))))) / (assign5270_e5747 * assign5270_e5747)), ((((2.0 * var_vn0__blk167_dn7) * assign5270_e5747) - (assign5270_e5718 * ((((var_vn0__blk167_dn7 * assign5270_e5724) + (assign5270_e5721 * var_vn0__blk167_dn7)) / (2.0 * assign5270_e5732)) + (((var_vn0__blk167_dn7 * assign5270_e5738) + (assign5270_e5735 * var_vn0__blk167_dn7)) / (2.0 * assign5270_e5746))))) / (assign5270_e5747 * assign5270_e5747)), ((((2.0 * var_vn0__blk167_dn8) * assign5270_e5747) - (assign5270_e5718 * ((((var_vn0__blk167_dn8 * assign5270_e5724) + (assign5270_e5721 * var_vn0__blk167_dn8)) / (2.0 * assign5270_e5732)) + (((var_vn0__blk167_dn8 * assign5270_e5738) + (assign5270_e5735 * var_vn0__blk167_dn8)) / (2.0 * assign5270_e5746))))) / (assign5270_e5747 * assign5270_e5747)), ((((2.0 * var_vn0__blk167_dn9) * assign5270_e5747) - (assign5270_e5718 * ((((var_vn0__blk167_dn9 * assign5270_e5724) + (assign5270_e5721 * var_vn0__blk167_dn9)) / (2.0 * assign5270_e5732)) + (((var_vn0__blk167_dn9 * assign5270_e5738) + (assign5270_e5735 * var_vn0__blk167_dn9)) / (2.0 * assign5270_e5746))))) / (assign5270_e5747 * assign5270_e5747)), ((((2.0 * var_vn0__blk167_dn10) * assign5270_e5747) - (assign5270_e5718 * ((((var_vn0__blk167_dn10 * assign5270_e5724) + (assign5270_e5721 * var_vn0__blk167_dn10)) / (2.0 * assign5270_e5732)) + (((var_vn0__blk167_dn10 * assign5270_e5738) + (assign5270_e5735 * var_vn0__blk167_dn10)) / (2.0 * assign5270_e5746))))) / (assign5270_e5747 * assign5270_e5747)), ((((2.0 * var_vn0__blk167_dn11) * assign5270_e5747) - (assign5270_e5718 * ((((var_vn0__blk167_dn11 * assign5270_e5724) + (assign5270_e5721 * var_vn0__blk167_dn11)) / (2.0 * assign5270_e5732)) + (((var_vn0__blk167_dn11 * assign5270_e5738) + (assign5270_e5735 * var_vn0__blk167_dn11)) / (2.0 * assign5270_e5746))))) / (assign5270_e5747 * assign5270_e5747)), ((((2.0 * var_vn0__blk167_dn12) * assign5270_e5747) - (assign5270_e5718 * ((((var_vn0__blk167_dn12 * assign5270_e5724) + (assign5270_e5721 * var_vn0__blk167_dn12)) / (2.0 * assign5270_e5732)) + (((var_vn0__blk167_dn12 * assign5270_e5738) + (assign5270_e5735 * var_vn0__blk167_dn12)) / (2.0 * assign5270_e5746))))) / (assign5270_e5747 * assign5270_e5747)), ((((2.0 * var_vn0__blk167_dn13) * assign5270_e5747) - (assign5270_e5718 * ((((var_vn0__blk167_dn13 * assign5270_e5724) + (assign5270_e5721 * var_vn0__blk167_dn13)) / (2.0 * assign5270_e5732)) + (((var_vn0__blk167_dn13 * assign5270_e5738) + (assign5270_e5735 * var_vn0__blk167_dn13)) / (2.0 * assign5270_e5746))))) / (assign5270_e5747 * assign5270_e5747)),)
    } else {
        (var_vnl0__blk168, var_vnl0__blk168_dn0, var_vnl0__blk168_dn1, var_vnl0__blk168_dn2, var_vnl0__blk168_dn3, var_vnl0__blk168_dn4, var_vnl0__blk168_dn5, var_vnl0__blk168_dn6, var_vnl0__blk168_dn7, var_vnl0__blk168_dn8, var_vnl0__blk168_dn9, var_vnl0__blk168_dn10, var_vnl0__blk168_dn11, var_vnl0__blk168_dn12, var_vnl0__blk168_dn13,)
    }
};
        var_vnl0__blk168 = assign5270_e5750;
        var_vnl0__blk168_dn0 = assign5270_e5750_d_n0;
        var_vnl0__blk168_dn1 = assign5270_e5750_d_n1;
        var_vnl0__blk168_dn2 = assign5270_e5750_d_n2;
        var_vnl0__blk168_dn3 = assign5270_e5750_d_n3;
        var_vnl0__blk168_dn4 = assign5270_e5750_d_n4;
        var_vnl0__blk168_dn5 = assign5270_e5750_d_n5;
        var_vnl0__blk168_dn6 = assign5270_e5750_d_n6;
        var_vnl0__blk168_dn7 = assign5270_e5750_d_n7;
        var_vnl0__blk168_dn8 = assign5270_e5750_d_n8;
        var_vnl0__blk168_dn9 = assign5270_e5750_d_n9;
        var_vnl0__blk168_dn10 = assign5270_e5750_d_n10;
        var_vnl0__blk168_dn11 = assign5270_e5750_d_n11;
        var_vnl0__blk168_dn12 = assign5270_e5750_d_n12;
        var_vnl0__blk168_dn13 = assign5270_e5750_d_n13;

        let (assign5280_e5767, assign5280_e5767_d_n0, assign5280_e5767_d_n1, assign5280_e5767_d_n2, assign5280_e5767_d_n3, assign5280_e5767_d_n4, assign5280_e5767_d_n5, assign5280_e5767_d_n6, assign5280_e5767_d_n7, assign5280_e5767_d_n8, assign5280_e5767_d_n9, assign5280_e5767_d_n10, assign5280_e5767_d_n11, assign5280_e5767_d_n12, assign5280_e5767_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 != 0.0)) {
        let assign5280_e5759: f64 = (p.p45 - var_dv0__blk162);
        let assign5280_e5760: f64 = (var_vnl0__blk168 * assign5280_e5759);
        let assign5280_e5762: f64 = (assign5280_e5760 - p.p45);
        let assign5280_e5764: f64 = (assign5280_e5762 - var_dv0__blk162);
        let assign5280_e5765: f64 = (0.5 * assign5280_e5764);
        (assign5280_e5765, (0.5 * (((var_vnl0__blk168_dn0 * assign5280_e5759) + (var_vnl0__blk168 * (-var_dv0__blk162_dn0))) - var_dv0__blk162_dn0)), (0.5 * (((var_vnl0__blk168_dn1 * assign5280_e5759) + (var_vnl0__blk168 * (-var_dv0__blk162_dn1))) - var_dv0__blk162_dn1)), (0.5 * (((var_vnl0__blk168_dn2 * assign5280_e5759) + (var_vnl0__blk168 * (-var_dv0__blk162_dn2))) - var_dv0__blk162_dn2)), (0.5 * (((var_vnl0__blk168_dn3 * assign5280_e5759) + (var_vnl0__blk168 * (-var_dv0__blk162_dn3))) - var_dv0__blk162_dn3)), (0.5 * (((var_vnl0__blk168_dn4 * assign5280_e5759) + (var_vnl0__blk168 * (-var_dv0__blk162_dn4))) - var_dv0__blk162_dn4)), (0.5 * (((var_vnl0__blk168_dn5 * assign5280_e5759) + (var_vnl0__blk168 * (-var_dv0__blk162_dn5))) - var_dv0__blk162_dn5)), (0.5 * (((var_vnl0__blk168_dn6 * assign5280_e5759) + (var_vnl0__blk168 * (-var_dv0__blk162_dn6))) - var_dv0__blk162_dn6)), (0.5 * (((var_vnl0__blk168_dn7 * assign5280_e5759) + (var_vnl0__blk168 * (-var_dv0__blk162_dn7))) - var_dv0__blk162_dn7)), (0.5 * (((var_vnl0__blk168_dn8 * assign5280_e5759) + (var_vnl0__blk168 * (-var_dv0__blk162_dn8))) - var_dv0__blk162_dn8)), (0.5 * (((var_vnl0__blk168_dn9 * assign5280_e5759) + (var_vnl0__blk168 * (-var_dv0__blk162_dn9))) - var_dv0__blk162_dn9)), (0.5 * (((var_vnl0__blk168_dn10 * assign5280_e5759) + (var_vnl0__blk168 * (-var_dv0__blk162_dn10))) - var_dv0__blk162_dn10)), (0.5 * (((var_vnl0__blk168_dn11 * assign5280_e5759) + (var_vnl0__blk168 * (-var_dv0__blk162_dn11))) - var_dv0__blk162_dn11)), (0.5 * (((var_vnl0__blk168_dn12 * assign5280_e5759) + (var_vnl0__blk168 * (-var_dv0__blk162_dn12))) - var_dv0__blk162_dn12)), (0.5 * (((var_vnl0__blk168_dn13 * assign5280_e5759) + (var_vnl0__blk168 * (-var_dv0__blk162_dn13))) - var_dv0__blk162_dn13)),)
    } else {
        (var_vl0__blk169, var_vl0__blk169_dn0, var_vl0__blk169_dn1, var_vl0__blk169_dn2, var_vl0__blk169_dn3, var_vl0__blk169_dn4, var_vl0__blk169_dn5, var_vl0__blk169_dn6, var_vl0__blk169_dn7, var_vl0__blk169_dn8, var_vl0__blk169_dn9, var_vl0__blk169_dn10, var_vl0__blk169_dn11, var_vl0__blk169_dn12, var_vl0__blk169_dn13,)
    }
};
        var_vl0__blk169 = assign5280_e5767;
        var_vl0__blk169_dn0 = assign5280_e5767_d_n0;
        var_vl0__blk169_dn1 = assign5280_e5767_d_n1;
        var_vl0__blk169_dn2 = assign5280_e5767_d_n2;
        var_vl0__blk169_dn3 = assign5280_e5767_d_n3;
        var_vl0__blk169_dn4 = assign5280_e5767_d_n4;
        var_vl0__blk169_dn5 = assign5280_e5767_d_n5;
        var_vl0__blk169_dn6 = assign5280_e5767_d_n6;
        var_vl0__blk169_dn7 = assign5280_e5767_d_n7;
        var_vl0__blk169_dn8 = assign5280_e5767_d_n8;
        var_vl0__blk169_dn9 = assign5280_e5767_d_n9;
        var_vl0__blk169_dn10 = assign5280_e5767_d_n10;
        var_vl0__blk169_dn11 = assign5280_e5767_d_n11;
        var_vl0__blk169_dn12 = assign5280_e5767_d_n12;
        var_vl0__blk169_dn13 = assign5280_e5767_d_n13;

        let (assign5290_e5790, assign5290_e5790_d_n0, assign5290_e5790_d_n1, assign5290_e5790_d_n2, assign5290_e5790_d_n3, assign5290_e5790_d_n4, assign5290_e5790_d_n5, assign5290_e5790_d_n6, assign5290_e5790_d_n7, assign5290_e5790_d_n8, assign5290_e5790_d_n9, assign5290_e5790_d_n10, assign5290_e5790_d_n11, assign5290_e5790_d_n12, assign5290_e5790_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 != 0.0)) {
        let assign5290_e5777: f64 = (var_vl0__blk169 / var_pc_t);
        let assign5290_e5778: f64 = (1.0 - assign5290_e5777);
        let assign5290_e5781: f64 = (1.0 - p.p43);
        let assign5290_e5782: f64 = (assign5290_e5778).powf(assign5290_e5781);
        let assign5290_e5783: f64 = (1.0 - assign5290_e5782);
        let assign5290_e5784: f64 = (var_pc_t * assign5290_e5783);
        let assign5290_e5787: f64 = (1.0 - p.p43);
        let assign5290_e5788: f64 = (assign5290_e5784 / assign5290_e5787);
        (assign5290_e5788, (((var_pc_t_dn0 * assign5290_e5783) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5290_e5781) as f64).is_finite() && ((assign5290_e5781) as f64).fract() == 0.0 { if assign5290_e5781 == 0.0 { 0.0 } else { (assign5290_e5781 * ((assign5290_e5778).powf(assign5290_e5781 - 1.0) * (-(((var_vl0__blk169_dn0 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn0)) / (var_pc_t * var_pc_t))))) } } else { (assign5290_e5782 * (assign5290_e5781 * ((-(((var_vl0__blk169_dn0 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn0)) / (var_pc_t * var_pc_t))) / assign5290_e5778))) }))) / assign5290_e5787), (((var_pc_t_dn1 * assign5290_e5783) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5290_e5781) as f64).is_finite() && ((assign5290_e5781) as f64).fract() == 0.0 { if assign5290_e5781 == 0.0 { 0.0 } else { (assign5290_e5781 * ((assign5290_e5778).powf(assign5290_e5781 - 1.0) * (-(((var_vl0__blk169_dn1 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn1)) / (var_pc_t * var_pc_t))))) } } else { (assign5290_e5782 * (assign5290_e5781 * ((-(((var_vl0__blk169_dn1 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn1)) / (var_pc_t * var_pc_t))) / assign5290_e5778))) }))) / assign5290_e5787), (((var_pc_t_dn2 * assign5290_e5783) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5290_e5781) as f64).is_finite() && ((assign5290_e5781) as f64).fract() == 0.0 { if assign5290_e5781 == 0.0 { 0.0 } else { (assign5290_e5781 * ((assign5290_e5778).powf(assign5290_e5781 - 1.0) * (-(((var_vl0__blk169_dn2 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn2)) / (var_pc_t * var_pc_t))))) } } else { (assign5290_e5782 * (assign5290_e5781 * ((-(((var_vl0__blk169_dn2 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn2)) / (var_pc_t * var_pc_t))) / assign5290_e5778))) }))) / assign5290_e5787), (((var_pc_t_dn3 * assign5290_e5783) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5290_e5781) as f64).is_finite() && ((assign5290_e5781) as f64).fract() == 0.0 { if assign5290_e5781 == 0.0 { 0.0 } else { (assign5290_e5781 * ((assign5290_e5778).powf(assign5290_e5781 - 1.0) * (-(((var_vl0__blk169_dn3 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn3)) / (var_pc_t * var_pc_t))))) } } else { (assign5290_e5782 * (assign5290_e5781 * ((-(((var_vl0__blk169_dn3 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn3)) / (var_pc_t * var_pc_t))) / assign5290_e5778))) }))) / assign5290_e5787), (((var_pc_t_dn4 * assign5290_e5783) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5290_e5781) as f64).is_finite() && ((assign5290_e5781) as f64).fract() == 0.0 { if assign5290_e5781 == 0.0 { 0.0 } else { (assign5290_e5781 * ((assign5290_e5778).powf(assign5290_e5781 - 1.0) * (-(((var_vl0__blk169_dn4 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn4)) / (var_pc_t * var_pc_t))))) } } else { (assign5290_e5782 * (assign5290_e5781 * ((-(((var_vl0__blk169_dn4 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn4)) / (var_pc_t * var_pc_t))) / assign5290_e5778))) }))) / assign5290_e5787), (((var_pc_t_dn5 * assign5290_e5783) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5290_e5781) as f64).is_finite() && ((assign5290_e5781) as f64).fract() == 0.0 { if assign5290_e5781 == 0.0 { 0.0 } else { (assign5290_e5781 * ((assign5290_e5778).powf(assign5290_e5781 - 1.0) * (-(((var_vl0__blk169_dn5 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn5)) / (var_pc_t * var_pc_t))))) } } else { (assign5290_e5782 * (assign5290_e5781 * ((-(((var_vl0__blk169_dn5 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn5)) / (var_pc_t * var_pc_t))) / assign5290_e5778))) }))) / assign5290_e5787), (((var_pc_t_dn6 * assign5290_e5783) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5290_e5781) as f64).is_finite() && ((assign5290_e5781) as f64).fract() == 0.0 { if assign5290_e5781 == 0.0 { 0.0 } else { (assign5290_e5781 * ((assign5290_e5778).powf(assign5290_e5781 - 1.0) * (-(((var_vl0__blk169_dn6 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn6)) / (var_pc_t * var_pc_t))))) } } else { (assign5290_e5782 * (assign5290_e5781 * ((-(((var_vl0__blk169_dn6 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn6)) / (var_pc_t * var_pc_t))) / assign5290_e5778))) }))) / assign5290_e5787), (((var_pc_t_dn7 * assign5290_e5783) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5290_e5781) as f64).is_finite() && ((assign5290_e5781) as f64).fract() == 0.0 { if assign5290_e5781 == 0.0 { 0.0 } else { (assign5290_e5781 * ((assign5290_e5778).powf(assign5290_e5781 - 1.0) * (-(((var_vl0__blk169_dn7 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn7)) / (var_pc_t * var_pc_t))))) } } else { (assign5290_e5782 * (assign5290_e5781 * ((-(((var_vl0__blk169_dn7 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn7)) / (var_pc_t * var_pc_t))) / assign5290_e5778))) }))) / assign5290_e5787), (((var_pc_t_dn8 * assign5290_e5783) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5290_e5781) as f64).is_finite() && ((assign5290_e5781) as f64).fract() == 0.0 { if assign5290_e5781 == 0.0 { 0.0 } else { (assign5290_e5781 * ((assign5290_e5778).powf(assign5290_e5781 - 1.0) * (-(((var_vl0__blk169_dn8 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn8)) / (var_pc_t * var_pc_t))))) } } else { (assign5290_e5782 * (assign5290_e5781 * ((-(((var_vl0__blk169_dn8 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn8)) / (var_pc_t * var_pc_t))) / assign5290_e5778))) }))) / assign5290_e5787), (((var_pc_t_dn9 * assign5290_e5783) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5290_e5781) as f64).is_finite() && ((assign5290_e5781) as f64).fract() == 0.0 { if assign5290_e5781 == 0.0 { 0.0 } else { (assign5290_e5781 * ((assign5290_e5778).powf(assign5290_e5781 - 1.0) * (-(((var_vl0__blk169_dn9 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn9)) / (var_pc_t * var_pc_t))))) } } else { (assign5290_e5782 * (assign5290_e5781 * ((-(((var_vl0__blk169_dn9 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn9)) / (var_pc_t * var_pc_t))) / assign5290_e5778))) }))) / assign5290_e5787), (((var_pc_t_dn10 * assign5290_e5783) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5290_e5781) as f64).is_finite() && ((assign5290_e5781) as f64).fract() == 0.0 { if assign5290_e5781 == 0.0 { 0.0 } else { (assign5290_e5781 * ((assign5290_e5778).powf(assign5290_e5781 - 1.0) * (-(((var_vl0__blk169_dn10 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn10)) / (var_pc_t * var_pc_t))))) } } else { (assign5290_e5782 * (assign5290_e5781 * ((-(((var_vl0__blk169_dn10 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn10)) / (var_pc_t * var_pc_t))) / assign5290_e5778))) }))) / assign5290_e5787), (((var_pc_t_dn11 * assign5290_e5783) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5290_e5781) as f64).is_finite() && ((assign5290_e5781) as f64).fract() == 0.0 { if assign5290_e5781 == 0.0 { 0.0 } else { (assign5290_e5781 * ((assign5290_e5778).powf(assign5290_e5781 - 1.0) * (-(((var_vl0__blk169_dn11 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn11)) / (var_pc_t * var_pc_t))))) } } else { (assign5290_e5782 * (assign5290_e5781 * ((-(((var_vl0__blk169_dn11 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn11)) / (var_pc_t * var_pc_t))) / assign5290_e5778))) }))) / assign5290_e5787), (((var_pc_t_dn12 * assign5290_e5783) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5290_e5781) as f64).is_finite() && ((assign5290_e5781) as f64).fract() == 0.0 { if assign5290_e5781 == 0.0 { 0.0 } else { (assign5290_e5781 * ((assign5290_e5778).powf(assign5290_e5781 - 1.0) * (-(((var_vl0__blk169_dn12 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn12)) / (var_pc_t * var_pc_t))))) } } else { (assign5290_e5782 * (assign5290_e5781 * ((-(((var_vl0__blk169_dn12 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn12)) / (var_pc_t * var_pc_t))) / assign5290_e5778))) }))) / assign5290_e5787), (((var_pc_t_dn13 * assign5290_e5783) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5290_e5781) as f64).is_finite() && ((assign5290_e5781) as f64).fract() == 0.0 { if assign5290_e5781 == 0.0 { 0.0 } else { (assign5290_e5781 * ((assign5290_e5778).powf(assign5290_e5781 - 1.0) * (-(((var_vl0__blk169_dn13 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn13)) / (var_pc_t * var_pc_t))))) } } else { (assign5290_e5782 * (assign5290_e5781 * ((-(((var_vl0__blk169_dn13 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn13)) / (var_pc_t * var_pc_t))) / assign5290_e5778))) }))) / assign5290_e5787),)
    } else {
        (var_qlo0__blk170, var_qlo0__blk170_dn0, var_qlo0__blk170_dn1, var_qlo0__blk170_dn2, var_qlo0__blk170_dn3, var_qlo0__blk170_dn4, var_qlo0__blk170_dn5, var_qlo0__blk170_dn6, var_qlo0__blk170_dn7, var_qlo0__blk170_dn8, var_qlo0__blk170_dn9, var_qlo0__blk170_dn10, var_qlo0__blk170_dn11, var_qlo0__blk170_dn12, var_qlo0__blk170_dn13,)
    }
};
        var_qlo0__blk170 = assign5290_e5790;
        var_qlo0__blk170_dn0 = assign5290_e5790_d_n0;
        var_qlo0__blk170_dn1 = assign5290_e5790_d_n1;
        var_qlo0__blk170_dn2 = assign5290_e5790_d_n2;
        var_qlo0__blk170_dn3 = assign5290_e5790_d_n3;
        var_qlo0__blk170_dn4 = assign5290_e5790_d_n4;
        var_qlo0__blk170_dn5 = assign5290_e5790_d_n5;
        var_qlo0__blk170_dn6 = assign5290_e5790_d_n6;
        var_qlo0__blk170_dn7 = assign5290_e5790_d_n7;
        var_qlo0__blk170_dn8 = assign5290_e5790_d_n8;
        var_qlo0__blk170_dn9 = assign5290_e5790_d_n9;
        var_qlo0__blk170_dn10 = assign5290_e5790_d_n10;
        var_qlo0__blk170_dn11 = assign5290_e5790_d_n11;
        var_qlo0__blk170_dn12 = assign5290_e5790_d_n12;
        var_qlo0__blk170_dn13 = assign5290_e5790_d_n13;

        let (assign5300_e5807, assign5300_e5807_d_n0, assign5300_e5807_d_n1, assign5300_e5807_d_n2, assign5300_e5807_d_n3, assign5300_e5807_d_n4, assign5300_e5807_d_n5, assign5300_e5807_d_n6, assign5300_e5807_d_n7, assign5300_e5807_d_n8, assign5300_e5807_d_n9, assign5300_e5807_d_n10, assign5300_e5807_d_n11, assign5300_e5807_d_n12, assign5300_e5807_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 != 0.0)) {
        let assign5300_e5797: f64 = (2.0 * var_vbep);
        let assign5300_e5799: f64 = (assign5300_e5797 + p.p45);
        let assign5300_e5801: f64 = (assign5300_e5799 + var_dv0__blk162);
        let assign5300_e5804: f64 = (p.p45 - var_dv0__blk162);
        let assign5300_e5805: f64 = (assign5300_e5801 / assign5300_e5804);
        (assign5300_e5805, (((((2.0 * var_vbep_dn0) + var_dv0__blk162_dn0) * assign5300_e5804) - (assign5300_e5801 * (-var_dv0__blk162_dn0))) / (assign5300_e5804 * assign5300_e5804)), (((((2.0 * var_vbep_dn1) + var_dv0__blk162_dn1) * assign5300_e5804) - (assign5300_e5801 * (-var_dv0__blk162_dn1))) / (assign5300_e5804 * assign5300_e5804)), (((((2.0 * var_vbep_dn2) + var_dv0__blk162_dn2) * assign5300_e5804) - (assign5300_e5801 * (-var_dv0__blk162_dn2))) / (assign5300_e5804 * assign5300_e5804)), (((((2.0 * var_vbep_dn3) + var_dv0__blk162_dn3) * assign5300_e5804) - (assign5300_e5801 * (-var_dv0__blk162_dn3))) / (assign5300_e5804 * assign5300_e5804)), (((((2.0 * var_vbep_dn4) + var_dv0__blk162_dn4) * assign5300_e5804) - (assign5300_e5801 * (-var_dv0__blk162_dn4))) / (assign5300_e5804 * assign5300_e5804)), (((((2.0 * var_vbep_dn5) + var_dv0__blk162_dn5) * assign5300_e5804) - (assign5300_e5801 * (-var_dv0__blk162_dn5))) / (assign5300_e5804 * assign5300_e5804)), (((((2.0 * var_vbep_dn6) + var_dv0__blk162_dn6) * assign5300_e5804) - (assign5300_e5801 * (-var_dv0__blk162_dn6))) / (assign5300_e5804 * assign5300_e5804)), (((((2.0 * var_vbep_dn7) + var_dv0__blk162_dn7) * assign5300_e5804) - (assign5300_e5801 * (-var_dv0__blk162_dn7))) / (assign5300_e5804 * assign5300_e5804)), (((((2.0 * var_vbep_dn8) + var_dv0__blk162_dn8) * assign5300_e5804) - (assign5300_e5801 * (-var_dv0__blk162_dn8))) / (assign5300_e5804 * assign5300_e5804)), (((((2.0 * var_vbep_dn9) + var_dv0__blk162_dn9) * assign5300_e5804) - (assign5300_e5801 * (-var_dv0__blk162_dn9))) / (assign5300_e5804 * assign5300_e5804)), (((((2.0 * var_vbep_dn10) + var_dv0__blk162_dn10) * assign5300_e5804) - (assign5300_e5801 * (-var_dv0__blk162_dn10))) / (assign5300_e5804 * assign5300_e5804)), (((((2.0 * var_vbep_dn11) + var_dv0__blk162_dn11) * assign5300_e5804) - (assign5300_e5801 * (-var_dv0__blk162_dn11))) / (assign5300_e5804 * assign5300_e5804)), (((((2.0 * var_vbep_dn12) + var_dv0__blk162_dn12) * assign5300_e5804) - (assign5300_e5801 * (-var_dv0__blk162_dn12))) / (assign5300_e5804 * assign5300_e5804)), (((((2.0 * var_vbep_dn13) + var_dv0__blk162_dn13) * assign5300_e5804) - (assign5300_e5801 * (-var_dv0__blk162_dn13))) / (assign5300_e5804 * assign5300_e5804)),)
    } else {
        (var_vn__blk171, var_vn__blk171_dn0, var_vn__blk171_dn1, var_vn__blk171_dn2, var_vn__blk171_dn3, var_vn__blk171_dn4, var_vn__blk171_dn5, var_vn__blk171_dn6, var_vn__blk171_dn7, var_vn__blk171_dn8, var_vn__blk171_dn9, var_vn__blk171_dn10, var_vn__blk171_dn11, var_vn__blk171_dn12, var_vn__blk171_dn13,)
    }
};
        var_vn__blk171 = assign5300_e5807;
        var_vn__blk171_dn0 = assign5300_e5807_d_n0;
        var_vn__blk171_dn1 = assign5300_e5807_d_n1;
        var_vn__blk171_dn2 = assign5300_e5807_d_n2;
        var_vn__blk171_dn3 = assign5300_e5807_d_n3;
        var_vn__blk171_dn4 = assign5300_e5807_d_n4;
        var_vn__blk171_dn5 = assign5300_e5807_d_n5;
        var_vn__blk171_dn6 = assign5300_e5807_d_n6;
        var_vn__blk171_dn7 = assign5300_e5807_d_n7;
        var_vn__blk171_dn8 = assign5300_e5807_d_n8;
        var_vn__blk171_dn9 = assign5300_e5807_d_n9;
        var_vn__blk171_dn10 = assign5300_e5807_d_n10;
        var_vn__blk171_dn11 = assign5300_e5807_d_n11;
        var_vn__blk171_dn12 = assign5300_e5807_d_n12;
        var_vn__blk171_dn13 = assign5300_e5807_d_n13;

        let (assign5310_e5846, assign5310_e5846_d_n0, assign5310_e5846_d_n1, assign5310_e5846_d_n2, assign5310_e5846_d_n3, assign5310_e5846_d_n4, assign5310_e5846_d_n5, assign5310_e5846_d_n6, assign5310_e5846_d_n7, assign5310_e5846_d_n8, assign5310_e5846_d_n9, assign5310_e5846_d_n10, assign5310_e5846_d_n11, assign5310_e5846_d_n12, assign5310_e5846_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 != 0.0)) {
        let assign5310_e5814: f64 = (2.0 * var_vn__blk171);
        let assign5310_e5817: f64 = (var_vn__blk171 - 1.0);
        let assign5310_e5820: f64 = (var_vn__blk171 - 1.0);
        let assign5310_e5821: f64 = (assign5310_e5817 * assign5310_e5820);
        let assign5310_e5824: f64 = (4.0 * p.p44);
        let assign5310_e5826: f64 = (assign5310_e5824 * p.p44);
        let assign5310_e5827: f64 = (assign5310_e5821 + assign5310_e5826);
        let assign5310_e5828: f64 = (assign5310_e5827).sqrt();
        let assign5310_e5831: f64 = (var_vn__blk171 + 1.0);
        let assign5310_e5834: f64 = (var_vn__blk171 + 1.0);
        let assign5310_e5835: f64 = (assign5310_e5831 * assign5310_e5834);
        let assign5310_e5838: f64 = (4.0 * p.p46);
        let assign5310_e5840: f64 = (assign5310_e5838 * p.p46);
        let assign5310_e5841: f64 = (assign5310_e5835 + assign5310_e5840);
        let assign5310_e5842: f64 = (assign5310_e5841).sqrt();
        let assign5310_e5843: f64 = (assign5310_e5828 + assign5310_e5842);
        let assign5310_e5844: f64 = (assign5310_e5814 / assign5310_e5843);
        (assign5310_e5844, ((((2.0 * var_vn__blk171_dn0) * assign5310_e5843) - (assign5310_e5814 * ((((var_vn__blk171_dn0 * assign5310_e5820) + (assign5310_e5817 * var_vn__blk171_dn0)) / (2.0 * assign5310_e5828)) + (((var_vn__blk171_dn0 * assign5310_e5834) + (assign5310_e5831 * var_vn__blk171_dn0)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * var_vn__blk171_dn1) * assign5310_e5843) - (assign5310_e5814 * ((((var_vn__blk171_dn1 * assign5310_e5820) + (assign5310_e5817 * var_vn__blk171_dn1)) / (2.0 * assign5310_e5828)) + (((var_vn__blk171_dn1 * assign5310_e5834) + (assign5310_e5831 * var_vn__blk171_dn1)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * var_vn__blk171_dn2) * assign5310_e5843) - (assign5310_e5814 * ((((var_vn__blk171_dn2 * assign5310_e5820) + (assign5310_e5817 * var_vn__blk171_dn2)) / (2.0 * assign5310_e5828)) + (((var_vn__blk171_dn2 * assign5310_e5834) + (assign5310_e5831 * var_vn__blk171_dn2)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * var_vn__blk171_dn3) * assign5310_e5843) - (assign5310_e5814 * ((((var_vn__blk171_dn3 * assign5310_e5820) + (assign5310_e5817 * var_vn__blk171_dn3)) / (2.0 * assign5310_e5828)) + (((var_vn__blk171_dn3 * assign5310_e5834) + (assign5310_e5831 * var_vn__blk171_dn3)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * var_vn__blk171_dn4) * assign5310_e5843) - (assign5310_e5814 * ((((var_vn__blk171_dn4 * assign5310_e5820) + (assign5310_e5817 * var_vn__blk171_dn4)) / (2.0 * assign5310_e5828)) + (((var_vn__blk171_dn4 * assign5310_e5834) + (assign5310_e5831 * var_vn__blk171_dn4)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * var_vn__blk171_dn5) * assign5310_e5843) - (assign5310_e5814 * ((((var_vn__blk171_dn5 * assign5310_e5820) + (assign5310_e5817 * var_vn__blk171_dn5)) / (2.0 * assign5310_e5828)) + (((var_vn__blk171_dn5 * assign5310_e5834) + (assign5310_e5831 * var_vn__blk171_dn5)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * var_vn__blk171_dn6) * assign5310_e5843) - (assign5310_e5814 * ((((var_vn__blk171_dn6 * assign5310_e5820) + (assign5310_e5817 * var_vn__blk171_dn6)) / (2.0 * assign5310_e5828)) + (((var_vn__blk171_dn6 * assign5310_e5834) + (assign5310_e5831 * var_vn__blk171_dn6)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * var_vn__blk171_dn7) * assign5310_e5843) - (assign5310_e5814 * ((((var_vn__blk171_dn7 * assign5310_e5820) + (assign5310_e5817 * var_vn__blk171_dn7)) / (2.0 * assign5310_e5828)) + (((var_vn__blk171_dn7 * assign5310_e5834) + (assign5310_e5831 * var_vn__blk171_dn7)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * var_vn__blk171_dn8) * assign5310_e5843) - (assign5310_e5814 * ((((var_vn__blk171_dn8 * assign5310_e5820) + (assign5310_e5817 * var_vn__blk171_dn8)) / (2.0 * assign5310_e5828)) + (((var_vn__blk171_dn8 * assign5310_e5834) + (assign5310_e5831 * var_vn__blk171_dn8)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * var_vn__blk171_dn9) * assign5310_e5843) - (assign5310_e5814 * ((((var_vn__blk171_dn9 * assign5310_e5820) + (assign5310_e5817 * var_vn__blk171_dn9)) / (2.0 * assign5310_e5828)) + (((var_vn__blk171_dn9 * assign5310_e5834) + (assign5310_e5831 * var_vn__blk171_dn9)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * var_vn__blk171_dn10) * assign5310_e5843) - (assign5310_e5814 * ((((var_vn__blk171_dn10 * assign5310_e5820) + (assign5310_e5817 * var_vn__blk171_dn10)) / (2.0 * assign5310_e5828)) + (((var_vn__blk171_dn10 * assign5310_e5834) + (assign5310_e5831 * var_vn__blk171_dn10)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * var_vn__blk171_dn11) * assign5310_e5843) - (assign5310_e5814 * ((((var_vn__blk171_dn11 * assign5310_e5820) + (assign5310_e5817 * var_vn__blk171_dn11)) / (2.0 * assign5310_e5828)) + (((var_vn__blk171_dn11 * assign5310_e5834) + (assign5310_e5831 * var_vn__blk171_dn11)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * var_vn__blk171_dn12) * assign5310_e5843) - (assign5310_e5814 * ((((var_vn__blk171_dn12 * assign5310_e5820) + (assign5310_e5817 * var_vn__blk171_dn12)) / (2.0 * assign5310_e5828)) + (((var_vn__blk171_dn12 * assign5310_e5834) + (assign5310_e5831 * var_vn__blk171_dn12)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * var_vn__blk171_dn13) * assign5310_e5843) - (assign5310_e5814 * ((((var_vn__blk171_dn13 * assign5310_e5820) + (assign5310_e5817 * var_vn__blk171_dn13)) / (2.0 * assign5310_e5828)) + (((var_vn__blk171_dn13 * assign5310_e5834) + (assign5310_e5831 * var_vn__blk171_dn13)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)),)
    } else {
        (var_vnl__blk172, var_vnl__blk172_dn0, var_vnl__blk172_dn1, var_vnl__blk172_dn2, var_vnl__blk172_dn3, var_vnl__blk172_dn4, var_vnl__blk172_dn5, var_vnl__blk172_dn6, var_vnl__blk172_dn7, var_vnl__blk172_dn8, var_vnl__blk172_dn9, var_vnl__blk172_dn10, var_vnl__blk172_dn11, var_vnl__blk172_dn12, var_vnl__blk172_dn13,)
    }
};
        var_vnl__blk172 = assign5310_e5846;
        var_vnl__blk172_dn0 = assign5310_e5846_d_n0;
        var_vnl__blk172_dn1 = assign5310_e5846_d_n1;
        var_vnl__blk172_dn2 = assign5310_e5846_d_n2;
        var_vnl__blk172_dn3 = assign5310_e5846_d_n3;
        var_vnl__blk172_dn4 = assign5310_e5846_d_n4;
        var_vnl__blk172_dn5 = assign5310_e5846_d_n5;
        var_vnl__blk172_dn6 = assign5310_e5846_d_n6;
        var_vnl__blk172_dn7 = assign5310_e5846_d_n7;
        var_vnl__blk172_dn8 = assign5310_e5846_d_n8;
        var_vnl__blk172_dn9 = assign5310_e5846_d_n9;
        var_vnl__blk172_dn10 = assign5310_e5846_d_n10;
        var_vnl__blk172_dn11 = assign5310_e5846_d_n11;
        var_vnl__blk172_dn12 = assign5310_e5846_d_n12;
        var_vnl__blk172_dn13 = assign5310_e5846_d_n13;


        *var_dv0__blk162_slot = var_dv0__blk162;
        *var_dv0__blk162_dn0_slot = var_dv0__blk162_dn0;
        *var_dv0__blk162_dn1_slot = var_dv0__blk162_dn1;
        *var_dv0__blk162_dn10_slot = var_dv0__blk162_dn10;
        *var_dv0__blk162_dn11_slot = var_dv0__blk162_dn11;
        *var_dv0__blk162_dn12_slot = var_dv0__blk162_dn12;
        *var_dv0__blk162_dn13_slot = var_dv0__blk162_dn13;
        *var_dv0__blk162_dn2_slot = var_dv0__blk162_dn2;
        *var_dv0__blk162_dn3_slot = var_dv0__blk162_dn3;
        *var_dv0__blk162_dn4_slot = var_dv0__blk162_dn4;
        *var_dv0__blk162_dn5_slot = var_dv0__blk162_dn5;
        *var_dv0__blk162_dn6_slot = var_dv0__blk162_dn6;
        *var_dv0__blk162_dn7_slot = var_dv0__blk162_dn7;
        *var_dv0__blk162_dn8_slot = var_dv0__blk162_dn8;
        *var_dv0__blk162_dn9_slot = var_dv0__blk162_dn9;
        *var_dvh__blk163_slot = var_dvh__blk163;
        *var_dvh__blk163_dn0_slot = var_dvh__blk163_dn0;
        *var_dvh__blk163_dn1_slot = var_dvh__blk163_dn1;
        *var_dvh__blk163_dn10_slot = var_dvh__blk163_dn10;
        *var_dvh__blk163_dn11_slot = var_dvh__blk163_dn11;
        *var_dvh__blk163_dn12_slot = var_dvh__blk163_dn12;
        *var_dvh__blk163_dn13_slot = var_dvh__blk163_dn13;
        *var_dvh__blk163_dn2_slot = var_dvh__blk163_dn2;
        *var_dvh__blk163_dn3_slot = var_dvh__blk163_dn3;
        *var_dvh__blk163_dn4_slot = var_dvh__blk163_dn4;
        *var_dvh__blk163_dn5_slot = var_dvh__blk163_dn5;
        *var_dvh__blk163_dn6_slot = var_dvh__blk163_dn6;
        *var_dvh__blk163_dn7_slot = var_dvh__blk163_dn7;
        *var_dvh__blk163_dn8_slot = var_dvh__blk163_dn8;
        *var_dvh__blk163_dn9_slot = var_dvh__blk163_dn9;
        *var_guard183_slot = var_guard183;
        *var_guard184_slot = var_guard184;
        *var_guard185_slot = var_guard185;
        *var_guard186_slot = var_guard186;
        *var_pwq__blk164_slot = var_pwq__blk164;
        *var_qdbep_slot = var_qdbep;
        *var_qdbep_dn0_slot = var_qdbep_dn0;
        *var_qdbep_dn1_slot = var_qdbep_dn1;
        *var_qdbep_dn10_slot = var_qdbep_dn10;
        *var_qdbep_dn11_slot = var_qdbep_dn11;
        *var_qdbep_dn12_slot = var_qdbep_dn12;
        *var_qdbep_dn13_slot = var_qdbep_dn13;
        *var_qdbep_dn2_slot = var_qdbep_dn2;
        *var_qdbep_dn3_slot = var_qdbep_dn3;
        *var_qdbep_dn4_slot = var_qdbep_dn4;
        *var_qdbep_dn5_slot = var_qdbep_dn5;
        *var_qdbep_dn6_slot = var_qdbep_dn6;
        *var_qdbep_dn7_slot = var_qdbep_dn7;
        *var_qdbep_dn8_slot = var_qdbep_dn8;
        *var_qdbep_dn9_slot = var_qdbep_dn9;
        *var_qdbex_slot = var_qdbex;
        *var_qdbex_dn0_slot = var_qdbex_dn0;
        *var_qdbex_dn1_slot = var_qdbex_dn1;
        *var_qdbex_dn10_slot = var_qdbex_dn10;
        *var_qdbex_dn11_slot = var_qdbex_dn11;
        *var_qdbex_dn12_slot = var_qdbex_dn12;
        *var_qdbex_dn13_slot = var_qdbex_dn13;
        *var_qdbex_dn2_slot = var_qdbex_dn2;
        *var_qdbex_dn3_slot = var_qdbex_dn3;
        *var_qdbex_dn4_slot = var_qdbex_dn4;
        *var_qdbex_dn5_slot = var_qdbex_dn5;
        *var_qdbex_dn6_slot = var_qdbex_dn6;
        *var_qdbex_dn7_slot = var_qdbex_dn7;
        *var_qdbex_dn8_slot = var_qdbex_dn8;
        *var_qdbex_dn9_slot = var_qdbex_dn9;
        *var_qhi__blk166_slot = var_qhi__blk166;
        *var_qhi__blk166_dn0_slot = var_qhi__blk166_dn0;
        *var_qhi__blk166_dn1_slot = var_qhi__blk166_dn1;
        *var_qhi__blk166_dn10_slot = var_qhi__blk166_dn10;
        *var_qhi__blk166_dn11_slot = var_qhi__blk166_dn11;
        *var_qhi__blk166_dn12_slot = var_qhi__blk166_dn12;
        *var_qhi__blk166_dn13_slot = var_qhi__blk166_dn13;
        *var_qhi__blk166_dn2_slot = var_qhi__blk166_dn2;
        *var_qhi__blk166_dn3_slot = var_qhi__blk166_dn3;
        *var_qhi__blk166_dn4_slot = var_qhi__blk166_dn4;
        *var_qhi__blk166_dn5_slot = var_qhi__blk166_dn5;
        *var_qhi__blk166_dn6_slot = var_qhi__blk166_dn6;
        *var_qhi__blk166_dn7_slot = var_qhi__blk166_dn7;
        *var_qhi__blk166_dn8_slot = var_qhi__blk166_dn8;
        *var_qhi__blk166_dn9_slot = var_qhi__blk166_dn9;
        *var_qlo0__blk170_slot = var_qlo0__blk170;
        *var_qlo0__blk170_dn0_slot = var_qlo0__blk170_dn0;
        *var_qlo0__blk170_dn1_slot = var_qlo0__blk170_dn1;
        *var_qlo0__blk170_dn10_slot = var_qlo0__blk170_dn10;
        *var_qlo0__blk170_dn11_slot = var_qlo0__blk170_dn11;
        *var_qlo0__blk170_dn12_slot = var_qlo0__blk170_dn12;
        *var_qlo0__blk170_dn13_slot = var_qlo0__blk170_dn13;
        *var_qlo0__blk170_dn2_slot = var_qlo0__blk170_dn2;
        *var_qlo0__blk170_dn3_slot = var_qlo0__blk170_dn3;
        *var_qlo0__blk170_dn4_slot = var_qlo0__blk170_dn4;
        *var_qlo0__blk170_dn5_slot = var_qlo0__blk170_dn5;
        *var_qlo0__blk170_dn6_slot = var_qlo0__blk170_dn6;
        *var_qlo0__blk170_dn7_slot = var_qlo0__blk170_dn7;
        *var_qlo0__blk170_dn8_slot = var_qlo0__blk170_dn8;
        *var_qlo0__blk170_dn9_slot = var_qlo0__blk170_dn9;
        *var_qlo__blk152_slot = var_qlo__blk152;
        *var_qlo__blk152_dn0_slot = var_qlo__blk152_dn0;
        *var_qlo__blk152_dn1_slot = var_qlo__blk152_dn1;
        *var_qlo__blk152_dn10_slot = var_qlo__blk152_dn10;
        *var_qlo__blk152_dn11_slot = var_qlo__blk152_dn11;
        *var_qlo__blk152_dn12_slot = var_qlo__blk152_dn12;
        *var_qlo__blk152_dn13_slot = var_qlo__blk152_dn13;
        *var_qlo__blk152_dn2_slot = var_qlo__blk152_dn2;
        *var_qlo__blk152_dn3_slot = var_qlo__blk152_dn3;
        *var_qlo__blk152_dn4_slot = var_qlo__blk152_dn4;
        *var_qlo__blk152_dn5_slot = var_qlo__blk152_dn5;
        *var_qlo__blk152_dn6_slot = var_qlo__blk152_dn6;
        *var_qlo__blk152_dn7_slot = var_qlo__blk152_dn7;
        *var_qlo__blk152_dn8_slot = var_qlo__blk152_dn8;
        *var_qlo__blk152_dn9_slot = var_qlo__blk152_dn9;
        *var_qlo__blk165_slot = var_qlo__blk165;
        *var_qlo__blk165_dn0_slot = var_qlo__blk165_dn0;
        *var_qlo__blk165_dn1_slot = var_qlo__blk165_dn1;
        *var_qlo__blk165_dn10_slot = var_qlo__blk165_dn10;
        *var_qlo__blk165_dn11_slot = var_qlo__blk165_dn11;
        *var_qlo__blk165_dn12_slot = var_qlo__blk165_dn12;
        *var_qlo__blk165_dn13_slot = var_qlo__blk165_dn13;
        *var_qlo__blk165_dn2_slot = var_qlo__blk165_dn2;
        *var_qlo__blk165_dn3_slot = var_qlo__blk165_dn3;
        *var_qlo__blk165_dn4_slot = var_qlo__blk165_dn4;
        *var_qlo__blk165_dn5_slot = var_qlo__blk165_dn5;
        *var_qlo__blk165_dn6_slot = var_qlo__blk165_dn6;
        *var_qlo__blk165_dn7_slot = var_qlo__blk165_dn7;
        *var_qlo__blk165_dn8_slot = var_qlo__blk165_dn8;
        *var_qlo__blk165_dn9_slot = var_qlo__blk165_dn9;
        *var_vl0__blk169_slot = var_vl0__blk169;
        *var_vl0__blk169_dn0_slot = var_vl0__blk169_dn0;
        *var_vl0__blk169_dn1_slot = var_vl0__blk169_dn1;
        *var_vl0__blk169_dn10_slot = var_vl0__blk169_dn10;
        *var_vl0__blk169_dn11_slot = var_vl0__blk169_dn11;
        *var_vl0__blk169_dn12_slot = var_vl0__blk169_dn12;
        *var_vl0__blk169_dn13_slot = var_vl0__blk169_dn13;
        *var_vl0__blk169_dn2_slot = var_vl0__blk169_dn2;
        *var_vl0__blk169_dn3_slot = var_vl0__blk169_dn3;
        *var_vl0__blk169_dn4_slot = var_vl0__blk169_dn4;
        *var_vl0__blk169_dn5_slot = var_vl0__blk169_dn5;
        *var_vl0__blk169_dn6_slot = var_vl0__blk169_dn6;
        *var_vl0__blk169_dn7_slot = var_vl0__blk169_dn7;
        *var_vl0__blk169_dn8_slot = var_vl0__blk169_dn8;
        *var_vl0__blk169_dn9_slot = var_vl0__blk169_dn9;
        *var_vn0__blk167_slot = var_vn0__blk167;
        *var_vn0__blk167_dn0_slot = var_vn0__blk167_dn0;
        *var_vn0__blk167_dn1_slot = var_vn0__blk167_dn1;
        *var_vn0__blk167_dn10_slot = var_vn0__blk167_dn10;
        *var_vn0__blk167_dn11_slot = var_vn0__blk167_dn11;
        *var_vn0__blk167_dn12_slot = var_vn0__blk167_dn12;
        *var_vn0__blk167_dn13_slot = var_vn0__blk167_dn13;
        *var_vn0__blk167_dn2_slot = var_vn0__blk167_dn2;
        *var_vn0__blk167_dn3_slot = var_vn0__blk167_dn3;
        *var_vn0__blk167_dn4_slot = var_vn0__blk167_dn4;
        *var_vn0__blk167_dn5_slot = var_vn0__blk167_dn5;
        *var_vn0__blk167_dn6_slot = var_vn0__blk167_dn6;
        *var_vn0__blk167_dn7_slot = var_vn0__blk167_dn7;
        *var_vn0__blk167_dn8_slot = var_vn0__blk167_dn8;
        *var_vn0__blk167_dn9_slot = var_vn0__blk167_dn9;
        *var_vn__blk171_slot = var_vn__blk171;
        *var_vn__blk171_dn0_slot = var_vn__blk171_dn0;
        *var_vn__blk171_dn1_slot = var_vn__blk171_dn1;
        *var_vn__blk171_dn10_slot = var_vn__blk171_dn10;
        *var_vn__blk171_dn11_slot = var_vn__blk171_dn11;
        *var_vn__blk171_dn12_slot = var_vn__blk171_dn12;
        *var_vn__blk171_dn13_slot = var_vn__blk171_dn13;
        *var_vn__blk171_dn2_slot = var_vn__blk171_dn2;
        *var_vn__blk171_dn3_slot = var_vn__blk171_dn3;
        *var_vn__blk171_dn4_slot = var_vn__blk171_dn4;
        *var_vn__blk171_dn5_slot = var_vn__blk171_dn5;
        *var_vn__blk171_dn6_slot = var_vn__blk171_dn6;
        *var_vn__blk171_dn7_slot = var_vn__blk171_dn7;
        *var_vn__blk171_dn8_slot = var_vn__blk171_dn8;
        *var_vn__blk171_dn9_slot = var_vn__blk171_dn9;
        *var_vnl0__blk168_slot = var_vnl0__blk168;
        *var_vnl0__blk168_dn0_slot = var_vnl0__blk168_dn0;
        *var_vnl0__blk168_dn1_slot = var_vnl0__blk168_dn1;
        *var_vnl0__blk168_dn10_slot = var_vnl0__blk168_dn10;
        *var_vnl0__blk168_dn11_slot = var_vnl0__blk168_dn11;
        *var_vnl0__blk168_dn12_slot = var_vnl0__blk168_dn12;
        *var_vnl0__blk168_dn13_slot = var_vnl0__blk168_dn13;
        *var_vnl0__blk168_dn2_slot = var_vnl0__blk168_dn2;
        *var_vnl0__blk168_dn3_slot = var_vnl0__blk168_dn3;
        *var_vnl0__blk168_dn4_slot = var_vnl0__blk168_dn4;
        *var_vnl0__blk168_dn5_slot = var_vnl0__blk168_dn5;
        *var_vnl0__blk168_dn6_slot = var_vnl0__blk168_dn6;
        *var_vnl0__blk168_dn7_slot = var_vnl0__blk168_dn7;
        *var_vnl0__blk168_dn8_slot = var_vnl0__blk168_dn8;
        *var_vnl0__blk168_dn9_slot = var_vnl0__blk168_dn9;
        *var_vnl__blk172_slot = var_vnl__blk172;
        *var_vnl__blk172_dn0_slot = var_vnl__blk172_dn0;
        *var_vnl__blk172_dn1_slot = var_vnl__blk172_dn1;
        *var_vnl__blk172_dn10_slot = var_vnl__blk172_dn10;
        *var_vnl__blk172_dn11_slot = var_vnl__blk172_dn11;
        *var_vnl__blk172_dn12_slot = var_vnl__blk172_dn12;
        *var_vnl__blk172_dn13_slot = var_vnl__blk172_dn13;
        *var_vnl__blk172_dn2_slot = var_vnl__blk172_dn2;
        *var_vnl__blk172_dn3_slot = var_vnl__blk172_dn3;
        *var_vnl__blk172_dn4_slot = var_vnl__blk172_dn4;
        *var_vnl__blk172_dn5_slot = var_vnl__blk172_dn5;
        *var_vnl__blk172_dn6_slot = var_vnl__blk172_dn6;
        *var_vnl__blk172_dn7_slot = var_vnl__blk172_dn7;
        *var_vnl__blk172_dn8_slot = var_vnl__blk172_dn8;
        *var_vnl__blk172_dn9_slot = var_vnl__blk172_dn9;
    }

    pub(super) fn stamp_transient_block_21(
        p: &Parameters,
        var_dv0__blk162: f64,
        var_dv0__blk162_dn0: f64,
        var_dv0__blk162_dn1: f64,
        var_dv0__blk162_dn10: f64,
        var_dv0__blk162_dn11: f64,
        var_dv0__blk162_dn12: f64,
        var_dv0__blk162_dn13: f64,
        var_dv0__blk162_dn2: f64,
        var_dv0__blk162_dn3: f64,
        var_dv0__blk162_dn4: f64,
        var_dv0__blk162_dn5: f64,
        var_dv0__blk162_dn6: f64,
        var_dv0__blk162_dn7: f64,
        var_dv0__blk162_dn8: f64,
        var_dv0__blk162_dn9: f64,
        var_guard183: f64,
        var_guard186: f64,
        var_ifi: f64,
        var_ifi_dn0: f64,
        var_ifi_dn1: f64,
        var_ifi_dn10: f64,
        var_ifi_dn11: f64,
        var_ifi_dn12: f64,
        var_ifi_dn13: f64,
        var_ifi_dn2: f64,
        var_ifi_dn3: f64,
        var_ifi_dn4: f64,
        var_ifi_dn5: f64,
        var_ifi_dn6: f64,
        var_ifi_dn7: f64,
        var_ifi_dn8: f64,
        var_ifi_dn9: f64,
        var_iitf: f64,
        var_ivtf: f64,
        var_pc_t: f64,
        var_pc_t_dn0: f64,
        var_pc_t_dn1: f64,
        var_pc_t_dn10: f64,
        var_pc_t_dn11: f64,
        var_pc_t_dn12: f64,
        var_pc_t_dn13: f64,
        var_pc_t_dn2: f64,
        var_pc_t_dn3: f64,
        var_pc_t_dn4: f64,
        var_pc_t_dn5: f64,
        var_pc_t_dn6: f64,
        var_pc_t_dn7: f64,
        var_pc_t_dn8: f64,
        var_pc_t_dn9: f64,
        var_qlo0__blk170: f64,
        var_qlo0__blk170_dn0: f64,
        var_qlo0__blk170_dn1: f64,
        var_qlo0__blk170_dn10: f64,
        var_qlo0__blk170_dn11: f64,
        var_qlo0__blk170_dn12: f64,
        var_qlo0__blk170_dn13: f64,
        var_qlo0__blk170_dn2: f64,
        var_qlo0__blk170_dn3: f64,
        var_qlo0__blk170_dn4: f64,
        var_qlo0__blk170_dn5: f64,
        var_qlo0__blk170_dn6: f64,
        var_qlo0__blk170_dn7: f64,
        var_qlo0__blk170_dn8: f64,
        var_qlo0__blk170_dn9: f64,
        var_vbci: f64,
        var_vbci_dn0: f64,
        var_vbci_dn1: f64,
        var_vbci_dn10: f64,
        var_vbci_dn11: f64,
        var_vbci_dn12: f64,
        var_vbci_dn13: f64,
        var_vbci_dn2: f64,
        var_vbci_dn3: f64,
        var_vbci_dn4: f64,
        var_vbci_dn5: f64,
        var_vbci_dn6: f64,
        var_vbci_dn7: f64,
        var_vbci_dn8: f64,
        var_vbci_dn9: f64,
        var_vbep: f64,
        var_vbep_dn0: f64,
        var_vbep_dn1: f64,
        var_vbep_dn10: f64,
        var_vbep_dn11: f64,
        var_vbep_dn12: f64,
        var_vbep_dn13: f64,
        var_vbep_dn2: f64,
        var_vbep_dn3: f64,
        var_vbep_dn4: f64,
        var_vbep_dn5: f64,
        var_vbep_dn6: f64,
        var_vbep_dn7: f64,
        var_vbep_dn8: f64,
        var_vbep_dn9: f64,
        var_vmaxexp: f64,
        var_vnl__blk172: f64,
        var_vnl__blk172_dn0: f64,
        var_vnl__blk172_dn1: f64,
        var_vnl__blk172_dn10: f64,
        var_vnl__blk172_dn11: f64,
        var_vnl__blk172_dn12: f64,
        var_vnl__blk172_dn13: f64,
        var_vnl__blk172_dn2: f64,
        var_vnl__blk172_dn3: f64,
        var_vnl__blk172_dn4: f64,
        var_vnl__blk172_dn5: f64,
        var_vnl__blk172_dn6: f64,
        var_vnl__blk172_dn7: f64,
        var_vnl__blk172_dn8: f64,
        var_vnl__blk172_dn9: f64,
        var_arg_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn1_slot: &mut f64,
        var_arg_dn10_slot: &mut f64,
        var_arg_dn11_slot: &mut f64,
        var_arg_dn12_slot: &mut f64,
        var_arg_dn13_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_arg_dn8_slot: &mut f64,
        var_arg_dn9_slot: &mut f64,
        var_cl__blk177_slot: &mut f64,
        var_cl__blk177_dn0_slot: &mut f64,
        var_cl__blk177_dn1_slot: &mut f64,
        var_cl__blk177_dn10_slot: &mut f64,
        var_cl__blk177_dn11_slot: &mut f64,
        var_cl__blk177_dn12_slot: &mut f64,
        var_cl__blk177_dn13_slot: &mut f64,
        var_cl__blk177_dn2_slot: &mut f64,
        var_cl__blk177_dn3_slot: &mut f64,
        var_cl__blk177_dn4_slot: &mut f64,
        var_cl__blk177_dn5_slot: &mut f64,
        var_cl__blk177_dn6_slot: &mut f64,
        var_cl__blk177_dn7_slot: &mut f64,
        var_cl__blk177_dn8_slot: &mut f64,
        var_cl__blk177_dn9_slot: &mut f64,
        var_cmx__blk176_slot: &mut f64,
        var_cmx__blk176_dn0_slot: &mut f64,
        var_cmx__blk176_dn1_slot: &mut f64,
        var_cmx__blk176_dn10_slot: &mut f64,
        var_cmx__blk176_dn11_slot: &mut f64,
        var_cmx__blk176_dn12_slot: &mut f64,
        var_cmx__blk176_dn13_slot: &mut f64,
        var_cmx__blk176_dn2_slot: &mut f64,
        var_cmx__blk176_dn3_slot: &mut f64,
        var_cmx__blk176_dn4_slot: &mut f64,
        var_cmx__blk176_dn5_slot: &mut f64,
        var_cmx__blk176_dn6_slot: &mut f64,
        var_cmx__blk176_dn7_slot: &mut f64,
        var_cmx__blk176_dn8_slot: &mut f64,
        var_cmx__blk176_dn9_slot: &mut f64,
        var_crt__blk175_slot: &mut f64,
        var_crt__blk175_dn0_slot: &mut f64,
        var_crt__blk175_dn1_slot: &mut f64,
        var_crt__blk175_dn10_slot: &mut f64,
        var_crt__blk175_dn11_slot: &mut f64,
        var_crt__blk175_dn12_slot: &mut f64,
        var_crt__blk175_dn13_slot: &mut f64,
        var_crt__blk175_dn2_slot: &mut f64,
        var_crt__blk175_dn3_slot: &mut f64,
        var_crt__blk175_dn4_slot: &mut f64,
        var_crt__blk175_dn5_slot: &mut f64,
        var_crt__blk175_dn6_slot: &mut f64,
        var_crt__blk175_dn7_slot: &mut f64,
        var_crt__blk175_dn8_slot: &mut f64,
        var_crt__blk175_dn9_slot: &mut f64,
        var_dv__blk181_slot: &mut f64,
        var_dv__blk181_dn0_slot: &mut f64,
        var_dv__blk181_dn1_slot: &mut f64,
        var_dv__blk181_dn10_slot: &mut f64,
        var_dv__blk181_dn11_slot: &mut f64,
        var_dv__blk181_dn12_slot: &mut f64,
        var_dv__blk181_dn13_slot: &mut f64,
        var_dv__blk181_dn2_slot: &mut f64,
        var_dv__blk181_dn3_slot: &mut f64,
        var_dv__blk181_dn4_slot: &mut f64,
        var_dv__blk181_dn5_slot: &mut f64,
        var_dv__blk181_dn6_slot: &mut f64,
        var_dv__blk181_dn7_slot: &mut f64,
        var_dv__blk181_dn8_slot: &mut f64,
        var_dv__blk181_dn9_slot: &mut f64,
        var_guard187_slot: &mut f64,
        var_mif_slot: &mut f64,
        var_mif_dn0_slot: &mut f64,
        var_mif_dn1_slot: &mut f64,
        var_mif_dn10_slot: &mut f64,
        var_mif_dn11_slot: &mut f64,
        var_mif_dn12_slot: &mut f64,
        var_mif_dn13_slot: &mut f64,
        var_mif_dn2_slot: &mut f64,
        var_mif_dn3_slot: &mut f64,
        var_mif_dn4_slot: &mut f64,
        var_mif_dn5_slot: &mut f64,
        var_mif_dn6_slot: &mut f64,
        var_mif_dn7_slot: &mut f64,
        var_mif_dn8_slot: &mut f64,
        var_mif_dn9_slot: &mut f64,
        var_mv0__blk179_slot: &mut f64,
        var_mv0__blk179_dn0_slot: &mut f64,
        var_mv0__blk179_dn1_slot: &mut f64,
        var_mv0__blk179_dn10_slot: &mut f64,
        var_mv0__blk179_dn11_slot: &mut f64,
        var_mv0__blk179_dn12_slot: &mut f64,
        var_mv0__blk179_dn13_slot: &mut f64,
        var_mv0__blk179_dn2_slot: &mut f64,
        var_mv0__blk179_dn3_slot: &mut f64,
        var_mv0__blk179_dn4_slot: &mut f64,
        var_mv0__blk179_dn5_slot: &mut f64,
        var_mv0__blk179_dn6_slot: &mut f64,
        var_mv0__blk179_dn7_slot: &mut f64,
        var_mv0__blk179_dn8_slot: &mut f64,
        var_mv0__blk179_dn9_slot: &mut f64,
        var_mv__blk182_slot: &mut f64,
        var_mv__blk182_dn0_slot: &mut f64,
        var_mv__blk182_dn1_slot: &mut f64,
        var_mv__blk182_dn10_slot: &mut f64,
        var_mv__blk182_dn11_slot: &mut f64,
        var_mv__blk182_dn12_slot: &mut f64,
        var_mv__blk182_dn13_slot: &mut f64,
        var_mv__blk182_dn2_slot: &mut f64,
        var_mv__blk182_dn3_slot: &mut f64,
        var_mv__blk182_dn4_slot: &mut f64,
        var_mv__blk182_dn5_slot: &mut f64,
        var_mv__blk182_dn6_slot: &mut f64,
        var_mv__blk182_dn7_slot: &mut f64,
        var_mv__blk182_dn8_slot: &mut f64,
        var_mv__blk182_dn9_slot: &mut f64,
        var_q0__blk180_slot: &mut f64,
        var_q0__blk180_dn0_slot: &mut f64,
        var_q0__blk180_dn1_slot: &mut f64,
        var_q0__blk180_dn10_slot: &mut f64,
        var_q0__blk180_dn11_slot: &mut f64,
        var_q0__blk180_dn12_slot: &mut f64,
        var_q0__blk180_dn13_slot: &mut f64,
        var_q0__blk180_dn2_slot: &mut f64,
        var_q0__blk180_dn3_slot: &mut f64,
        var_q0__blk180_dn4_slot: &mut f64,
        var_q0__blk180_dn5_slot: &mut f64,
        var_q0__blk180_dn6_slot: &mut f64,
        var_q0__blk180_dn7_slot: &mut f64,
        var_q0__blk180_dn8_slot: &mut f64,
        var_q0__blk180_dn9_slot: &mut f64,
        var_qdbep_slot: &mut f64,
        var_qdbep_dn0_slot: &mut f64,
        var_qdbep_dn1_slot: &mut f64,
        var_qdbep_dn10_slot: &mut f64,
        var_qdbep_dn11_slot: &mut f64,
        var_qdbep_dn12_slot: &mut f64,
        var_qdbep_dn13_slot: &mut f64,
        var_qdbep_dn2_slot: &mut f64,
        var_qdbep_dn3_slot: &mut f64,
        var_qdbep_dn4_slot: &mut f64,
        var_qdbep_dn5_slot: &mut f64,
        var_qdbep_dn6_slot: &mut f64,
        var_qdbep_dn7_slot: &mut f64,
        var_qdbep_dn8_slot: &mut f64,
        var_qdbep_dn9_slot: &mut f64,
        var_ql__blk178_slot: &mut f64,
        var_ql__blk178_dn0_slot: &mut f64,
        var_ql__blk178_dn1_slot: &mut f64,
        var_ql__blk178_dn10_slot: &mut f64,
        var_ql__blk178_dn11_slot: &mut f64,
        var_ql__blk178_dn12_slot: &mut f64,
        var_ql__blk178_dn13_slot: &mut f64,
        var_ql__blk178_dn2_slot: &mut f64,
        var_ql__blk178_dn3_slot: &mut f64,
        var_ql__blk178_dn4_slot: &mut f64,
        var_ql__blk178_dn5_slot: &mut f64,
        var_ql__blk178_dn6_slot: &mut f64,
        var_ql__blk178_dn7_slot: &mut f64,
        var_ql__blk178_dn8_slot: &mut f64,
        var_ql__blk178_dn9_slot: &mut f64,
        var_qlo__blk165_slot: &mut f64,
        var_qlo__blk165_dn0_slot: &mut f64,
        var_qlo__blk165_dn1_slot: &mut f64,
        var_qlo__blk165_dn10_slot: &mut f64,
        var_qlo__blk165_dn11_slot: &mut f64,
        var_qlo__blk165_dn12_slot: &mut f64,
        var_qlo__blk165_dn13_slot: &mut f64,
        var_qlo__blk165_dn2_slot: &mut f64,
        var_qlo__blk165_dn3_slot: &mut f64,
        var_qlo__blk165_dn4_slot: &mut f64,
        var_qlo__blk165_dn5_slot: &mut f64,
        var_qlo__blk165_dn6_slot: &mut f64,
        var_qlo__blk165_dn7_slot: &mut f64,
        var_qlo__blk165_dn8_slot: &mut f64,
        var_qlo__blk165_dn9_slot: &mut f64,
        var_rif_slot: &mut f64,
        var_rif_dn0_slot: &mut f64,
        var_rif_dn1_slot: &mut f64,
        var_rif_dn10_slot: &mut f64,
        var_rif_dn11_slot: &mut f64,
        var_rif_dn12_slot: &mut f64,
        var_rif_dn13_slot: &mut f64,
        var_rif_dn2_slot: &mut f64,
        var_rif_dn3_slot: &mut f64,
        var_rif_dn4_slot: &mut f64,
        var_rif_dn5_slot: &mut f64,
        var_rif_dn6_slot: &mut f64,
        var_rif_dn7_slot: &mut f64,
        var_rif_dn8_slot: &mut f64,
        var_rif_dn9_slot: &mut f64,
        var_sel__blk174_slot: &mut f64,
        var_sel__blk174_dn0_slot: &mut f64,
        var_sel__blk174_dn1_slot: &mut f64,
        var_sel__blk174_dn10_slot: &mut f64,
        var_sel__blk174_dn11_slot: &mut f64,
        var_sel__blk174_dn12_slot: &mut f64,
        var_sel__blk174_dn13_slot: &mut f64,
        var_sel__blk174_dn2_slot: &mut f64,
        var_sel__blk174_dn3_slot: &mut f64,
        var_sel__blk174_dn4_slot: &mut f64,
        var_sel__blk174_dn5_slot: &mut f64,
        var_sel__blk174_dn6_slot: &mut f64,
        var_sel__blk174_dn7_slot: &mut f64,
        var_sel__blk174_dn8_slot: &mut f64,
        var_sel__blk174_dn9_slot: &mut f64,
        var_sgif_slot: &mut f64,
        var_vl0__blk169_slot: &mut f64,
        var_vl0__blk169_dn0_slot: &mut f64,
        var_vl0__blk169_dn1_slot: &mut f64,
        var_vl0__blk169_dn10_slot: &mut f64,
        var_vl0__blk169_dn11_slot: &mut f64,
        var_vl0__blk169_dn12_slot: &mut f64,
        var_vl0__blk169_dn13_slot: &mut f64,
        var_vl0__blk169_dn2_slot: &mut f64,
        var_vl0__blk169_dn3_slot: &mut f64,
        var_vl0__blk169_dn4_slot: &mut f64,
        var_vl0__blk169_dn5_slot: &mut f64,
        var_vl0__blk169_dn6_slot: &mut f64,
        var_vl0__blk169_dn7_slot: &mut f64,
        var_vl0__blk169_dn8_slot: &mut f64,
        var_vl0__blk169_dn9_slot: &mut f64,
        var_vl__blk173_slot: &mut f64,
        var_vl__blk173_dn0_slot: &mut f64,
        var_vl__blk173_dn1_slot: &mut f64,
        var_vl__blk173_dn10_slot: &mut f64,
        var_vl__blk173_dn11_slot: &mut f64,
        var_vl__blk173_dn12_slot: &mut f64,
        var_vl__blk173_dn13_slot: &mut f64,
        var_vl__blk173_dn2_slot: &mut f64,
        var_vl__blk173_dn3_slot: &mut f64,
        var_vl__blk173_dn4_slot: &mut f64,
        var_vl__blk173_dn5_slot: &mut f64,
        var_vl__blk173_dn6_slot: &mut f64,
        var_vl__blk173_dn7_slot: &mut f64,
        var_vl__blk173_dn8_slot: &mut f64,
        var_vl__blk173_dn9_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn1: f64 = *var_arg_dn1_slot;
        let mut var_arg_dn10: f64 = *var_arg_dn10_slot;
        let mut var_arg_dn11: f64 = *var_arg_dn11_slot;
        let mut var_arg_dn12: f64 = *var_arg_dn12_slot;
        let mut var_arg_dn13: f64 = *var_arg_dn13_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_arg_dn8: f64 = *var_arg_dn8_slot;
        let mut var_arg_dn9: f64 = *var_arg_dn9_slot;
        let mut var_cl__blk177: f64 = *var_cl__blk177_slot;
        let mut var_cl__blk177_dn0: f64 = *var_cl__blk177_dn0_slot;
        let mut var_cl__blk177_dn1: f64 = *var_cl__blk177_dn1_slot;
        let mut var_cl__blk177_dn10: f64 = *var_cl__blk177_dn10_slot;
        let mut var_cl__blk177_dn11: f64 = *var_cl__blk177_dn11_slot;
        let mut var_cl__blk177_dn12: f64 = *var_cl__blk177_dn12_slot;
        let mut var_cl__blk177_dn13: f64 = *var_cl__blk177_dn13_slot;
        let mut var_cl__blk177_dn2: f64 = *var_cl__blk177_dn2_slot;
        let mut var_cl__blk177_dn3: f64 = *var_cl__blk177_dn3_slot;
        let mut var_cl__blk177_dn4: f64 = *var_cl__blk177_dn4_slot;
        let mut var_cl__blk177_dn5: f64 = *var_cl__blk177_dn5_slot;
        let mut var_cl__blk177_dn6: f64 = *var_cl__blk177_dn6_slot;
        let mut var_cl__blk177_dn7: f64 = *var_cl__blk177_dn7_slot;
        let mut var_cl__blk177_dn8: f64 = *var_cl__blk177_dn8_slot;
        let mut var_cl__blk177_dn9: f64 = *var_cl__blk177_dn9_slot;
        let mut var_cmx__blk176: f64 = *var_cmx__blk176_slot;
        let mut var_cmx__blk176_dn0: f64 = *var_cmx__blk176_dn0_slot;
        let mut var_cmx__blk176_dn1: f64 = *var_cmx__blk176_dn1_slot;
        let mut var_cmx__blk176_dn10: f64 = *var_cmx__blk176_dn10_slot;
        let mut var_cmx__blk176_dn11: f64 = *var_cmx__blk176_dn11_slot;
        let mut var_cmx__blk176_dn12: f64 = *var_cmx__blk176_dn12_slot;
        let mut var_cmx__blk176_dn13: f64 = *var_cmx__blk176_dn13_slot;
        let mut var_cmx__blk176_dn2: f64 = *var_cmx__blk176_dn2_slot;
        let mut var_cmx__blk176_dn3: f64 = *var_cmx__blk176_dn3_slot;
        let mut var_cmx__blk176_dn4: f64 = *var_cmx__blk176_dn4_slot;
        let mut var_cmx__blk176_dn5: f64 = *var_cmx__blk176_dn5_slot;
        let mut var_cmx__blk176_dn6: f64 = *var_cmx__blk176_dn6_slot;
        let mut var_cmx__blk176_dn7: f64 = *var_cmx__blk176_dn7_slot;
        let mut var_cmx__blk176_dn8: f64 = *var_cmx__blk176_dn8_slot;
        let mut var_cmx__blk176_dn9: f64 = *var_cmx__blk176_dn9_slot;
        let mut var_crt__blk175: f64 = *var_crt__blk175_slot;
        let mut var_crt__blk175_dn0: f64 = *var_crt__blk175_dn0_slot;
        let mut var_crt__blk175_dn1: f64 = *var_crt__blk175_dn1_slot;
        let mut var_crt__blk175_dn10: f64 = *var_crt__blk175_dn10_slot;
        let mut var_crt__blk175_dn11: f64 = *var_crt__blk175_dn11_slot;
        let mut var_crt__blk175_dn12: f64 = *var_crt__blk175_dn12_slot;
        let mut var_crt__blk175_dn13: f64 = *var_crt__blk175_dn13_slot;
        let mut var_crt__blk175_dn2: f64 = *var_crt__blk175_dn2_slot;
        let mut var_crt__blk175_dn3: f64 = *var_crt__blk175_dn3_slot;
        let mut var_crt__blk175_dn4: f64 = *var_crt__blk175_dn4_slot;
        let mut var_crt__blk175_dn5: f64 = *var_crt__blk175_dn5_slot;
        let mut var_crt__blk175_dn6: f64 = *var_crt__blk175_dn6_slot;
        let mut var_crt__blk175_dn7: f64 = *var_crt__blk175_dn7_slot;
        let mut var_crt__blk175_dn8: f64 = *var_crt__blk175_dn8_slot;
        let mut var_crt__blk175_dn9: f64 = *var_crt__blk175_dn9_slot;
        let mut var_dv__blk181: f64 = *var_dv__blk181_slot;
        let mut var_dv__blk181_dn0: f64 = *var_dv__blk181_dn0_slot;
        let mut var_dv__blk181_dn1: f64 = *var_dv__blk181_dn1_slot;
        let mut var_dv__blk181_dn10: f64 = *var_dv__blk181_dn10_slot;
        let mut var_dv__blk181_dn11: f64 = *var_dv__blk181_dn11_slot;
        let mut var_dv__blk181_dn12: f64 = *var_dv__blk181_dn12_slot;
        let mut var_dv__blk181_dn13: f64 = *var_dv__blk181_dn13_slot;
        let mut var_dv__blk181_dn2: f64 = *var_dv__blk181_dn2_slot;
        let mut var_dv__blk181_dn3: f64 = *var_dv__blk181_dn3_slot;
        let mut var_dv__blk181_dn4: f64 = *var_dv__blk181_dn4_slot;
        let mut var_dv__blk181_dn5: f64 = *var_dv__blk181_dn5_slot;
        let mut var_dv__blk181_dn6: f64 = *var_dv__blk181_dn6_slot;
        let mut var_dv__blk181_dn7: f64 = *var_dv__blk181_dn7_slot;
        let mut var_dv__blk181_dn8: f64 = *var_dv__blk181_dn8_slot;
        let mut var_dv__blk181_dn9: f64 = *var_dv__blk181_dn9_slot;
        let mut var_guard187: f64 = *var_guard187_slot;
        let mut var_mif: f64 = *var_mif_slot;
        let mut var_mif_dn0: f64 = *var_mif_dn0_slot;
        let mut var_mif_dn1: f64 = *var_mif_dn1_slot;
        let mut var_mif_dn10: f64 = *var_mif_dn10_slot;
        let mut var_mif_dn11: f64 = *var_mif_dn11_slot;
        let mut var_mif_dn12: f64 = *var_mif_dn12_slot;
        let mut var_mif_dn13: f64 = *var_mif_dn13_slot;
        let mut var_mif_dn2: f64 = *var_mif_dn2_slot;
        let mut var_mif_dn3: f64 = *var_mif_dn3_slot;
        let mut var_mif_dn4: f64 = *var_mif_dn4_slot;
        let mut var_mif_dn5: f64 = *var_mif_dn5_slot;
        let mut var_mif_dn6: f64 = *var_mif_dn6_slot;
        let mut var_mif_dn7: f64 = *var_mif_dn7_slot;
        let mut var_mif_dn8: f64 = *var_mif_dn8_slot;
        let mut var_mif_dn9: f64 = *var_mif_dn9_slot;
        let mut var_mv0__blk179: f64 = *var_mv0__blk179_slot;
        let mut var_mv0__blk179_dn0: f64 = *var_mv0__blk179_dn0_slot;
        let mut var_mv0__blk179_dn1: f64 = *var_mv0__blk179_dn1_slot;
        let mut var_mv0__blk179_dn10: f64 = *var_mv0__blk179_dn10_slot;
        let mut var_mv0__blk179_dn11: f64 = *var_mv0__blk179_dn11_slot;
        let mut var_mv0__blk179_dn12: f64 = *var_mv0__blk179_dn12_slot;
        let mut var_mv0__blk179_dn13: f64 = *var_mv0__blk179_dn13_slot;
        let mut var_mv0__blk179_dn2: f64 = *var_mv0__blk179_dn2_slot;
        let mut var_mv0__blk179_dn3: f64 = *var_mv0__blk179_dn3_slot;
        let mut var_mv0__blk179_dn4: f64 = *var_mv0__blk179_dn4_slot;
        let mut var_mv0__blk179_dn5: f64 = *var_mv0__blk179_dn5_slot;
        let mut var_mv0__blk179_dn6: f64 = *var_mv0__blk179_dn6_slot;
        let mut var_mv0__blk179_dn7: f64 = *var_mv0__blk179_dn7_slot;
        let mut var_mv0__blk179_dn8: f64 = *var_mv0__blk179_dn8_slot;
        let mut var_mv0__blk179_dn9: f64 = *var_mv0__blk179_dn9_slot;
        let mut var_mv__blk182: f64 = *var_mv__blk182_slot;
        let mut var_mv__blk182_dn0: f64 = *var_mv__blk182_dn0_slot;
        let mut var_mv__blk182_dn1: f64 = *var_mv__blk182_dn1_slot;
        let mut var_mv__blk182_dn10: f64 = *var_mv__blk182_dn10_slot;
        let mut var_mv__blk182_dn11: f64 = *var_mv__blk182_dn11_slot;
        let mut var_mv__blk182_dn12: f64 = *var_mv__blk182_dn12_slot;
        let mut var_mv__blk182_dn13: f64 = *var_mv__blk182_dn13_slot;
        let mut var_mv__blk182_dn2: f64 = *var_mv__blk182_dn2_slot;
        let mut var_mv__blk182_dn3: f64 = *var_mv__blk182_dn3_slot;
        let mut var_mv__blk182_dn4: f64 = *var_mv__blk182_dn4_slot;
        let mut var_mv__blk182_dn5: f64 = *var_mv__blk182_dn5_slot;
        let mut var_mv__blk182_dn6: f64 = *var_mv__blk182_dn6_slot;
        let mut var_mv__blk182_dn7: f64 = *var_mv__blk182_dn7_slot;
        let mut var_mv__blk182_dn8: f64 = *var_mv__blk182_dn8_slot;
        let mut var_mv__blk182_dn9: f64 = *var_mv__blk182_dn9_slot;
        let mut var_q0__blk180: f64 = *var_q0__blk180_slot;
        let mut var_q0__blk180_dn0: f64 = *var_q0__blk180_dn0_slot;
        let mut var_q0__blk180_dn1: f64 = *var_q0__blk180_dn1_slot;
        let mut var_q0__blk180_dn10: f64 = *var_q0__blk180_dn10_slot;
        let mut var_q0__blk180_dn11: f64 = *var_q0__blk180_dn11_slot;
        let mut var_q0__blk180_dn12: f64 = *var_q0__blk180_dn12_slot;
        let mut var_q0__blk180_dn13: f64 = *var_q0__blk180_dn13_slot;
        let mut var_q0__blk180_dn2: f64 = *var_q0__blk180_dn2_slot;
        let mut var_q0__blk180_dn3: f64 = *var_q0__blk180_dn3_slot;
        let mut var_q0__blk180_dn4: f64 = *var_q0__blk180_dn4_slot;
        let mut var_q0__blk180_dn5: f64 = *var_q0__blk180_dn5_slot;
        let mut var_q0__blk180_dn6: f64 = *var_q0__blk180_dn6_slot;
        let mut var_q0__blk180_dn7: f64 = *var_q0__blk180_dn7_slot;
        let mut var_q0__blk180_dn8: f64 = *var_q0__blk180_dn8_slot;
        let mut var_q0__blk180_dn9: f64 = *var_q0__blk180_dn9_slot;
        let mut var_qdbep: f64 = *var_qdbep_slot;
        let mut var_qdbep_dn0: f64 = *var_qdbep_dn0_slot;
        let mut var_qdbep_dn1: f64 = *var_qdbep_dn1_slot;
        let mut var_qdbep_dn10: f64 = *var_qdbep_dn10_slot;
        let mut var_qdbep_dn11: f64 = *var_qdbep_dn11_slot;
        let mut var_qdbep_dn12: f64 = *var_qdbep_dn12_slot;
        let mut var_qdbep_dn13: f64 = *var_qdbep_dn13_slot;
        let mut var_qdbep_dn2: f64 = *var_qdbep_dn2_slot;
        let mut var_qdbep_dn3: f64 = *var_qdbep_dn3_slot;
        let mut var_qdbep_dn4: f64 = *var_qdbep_dn4_slot;
        let mut var_qdbep_dn5: f64 = *var_qdbep_dn5_slot;
        let mut var_qdbep_dn6: f64 = *var_qdbep_dn6_slot;
        let mut var_qdbep_dn7: f64 = *var_qdbep_dn7_slot;
        let mut var_qdbep_dn8: f64 = *var_qdbep_dn8_slot;
        let mut var_qdbep_dn9: f64 = *var_qdbep_dn9_slot;
        let mut var_ql__blk178: f64 = *var_ql__blk178_slot;
        let mut var_ql__blk178_dn0: f64 = *var_ql__blk178_dn0_slot;
        let mut var_ql__blk178_dn1: f64 = *var_ql__blk178_dn1_slot;
        let mut var_ql__blk178_dn10: f64 = *var_ql__blk178_dn10_slot;
        let mut var_ql__blk178_dn11: f64 = *var_ql__blk178_dn11_slot;
        let mut var_ql__blk178_dn12: f64 = *var_ql__blk178_dn12_slot;
        let mut var_ql__blk178_dn13: f64 = *var_ql__blk178_dn13_slot;
        let mut var_ql__blk178_dn2: f64 = *var_ql__blk178_dn2_slot;
        let mut var_ql__blk178_dn3: f64 = *var_ql__blk178_dn3_slot;
        let mut var_ql__blk178_dn4: f64 = *var_ql__blk178_dn4_slot;
        let mut var_ql__blk178_dn5: f64 = *var_ql__blk178_dn5_slot;
        let mut var_ql__blk178_dn6: f64 = *var_ql__blk178_dn6_slot;
        let mut var_ql__blk178_dn7: f64 = *var_ql__blk178_dn7_slot;
        let mut var_ql__blk178_dn8: f64 = *var_ql__blk178_dn8_slot;
        let mut var_ql__blk178_dn9: f64 = *var_ql__blk178_dn9_slot;
        let mut var_qlo__blk165: f64 = *var_qlo__blk165_slot;
        let mut var_qlo__blk165_dn0: f64 = *var_qlo__blk165_dn0_slot;
        let mut var_qlo__blk165_dn1: f64 = *var_qlo__blk165_dn1_slot;
        let mut var_qlo__blk165_dn10: f64 = *var_qlo__blk165_dn10_slot;
        let mut var_qlo__blk165_dn11: f64 = *var_qlo__blk165_dn11_slot;
        let mut var_qlo__blk165_dn12: f64 = *var_qlo__blk165_dn12_slot;
        let mut var_qlo__blk165_dn13: f64 = *var_qlo__blk165_dn13_slot;
        let mut var_qlo__blk165_dn2: f64 = *var_qlo__blk165_dn2_slot;
        let mut var_qlo__blk165_dn3: f64 = *var_qlo__blk165_dn3_slot;
        let mut var_qlo__blk165_dn4: f64 = *var_qlo__blk165_dn4_slot;
        let mut var_qlo__blk165_dn5: f64 = *var_qlo__blk165_dn5_slot;
        let mut var_qlo__blk165_dn6: f64 = *var_qlo__blk165_dn6_slot;
        let mut var_qlo__blk165_dn7: f64 = *var_qlo__blk165_dn7_slot;
        let mut var_qlo__blk165_dn8: f64 = *var_qlo__blk165_dn8_slot;
        let mut var_qlo__blk165_dn9: f64 = *var_qlo__blk165_dn9_slot;
        let mut var_rif: f64 = *var_rif_slot;
        let mut var_rif_dn0: f64 = *var_rif_dn0_slot;
        let mut var_rif_dn1: f64 = *var_rif_dn1_slot;
        let mut var_rif_dn10: f64 = *var_rif_dn10_slot;
        let mut var_rif_dn11: f64 = *var_rif_dn11_slot;
        let mut var_rif_dn12: f64 = *var_rif_dn12_slot;
        let mut var_rif_dn13: f64 = *var_rif_dn13_slot;
        let mut var_rif_dn2: f64 = *var_rif_dn2_slot;
        let mut var_rif_dn3: f64 = *var_rif_dn3_slot;
        let mut var_rif_dn4: f64 = *var_rif_dn4_slot;
        let mut var_rif_dn5: f64 = *var_rif_dn5_slot;
        let mut var_rif_dn6: f64 = *var_rif_dn6_slot;
        let mut var_rif_dn7: f64 = *var_rif_dn7_slot;
        let mut var_rif_dn8: f64 = *var_rif_dn8_slot;
        let mut var_rif_dn9: f64 = *var_rif_dn9_slot;
        let mut var_sel__blk174: f64 = *var_sel__blk174_slot;
        let mut var_sel__blk174_dn0: f64 = *var_sel__blk174_dn0_slot;
        let mut var_sel__blk174_dn1: f64 = *var_sel__blk174_dn1_slot;
        let mut var_sel__blk174_dn10: f64 = *var_sel__blk174_dn10_slot;
        let mut var_sel__blk174_dn11: f64 = *var_sel__blk174_dn11_slot;
        let mut var_sel__blk174_dn12: f64 = *var_sel__blk174_dn12_slot;
        let mut var_sel__blk174_dn13: f64 = *var_sel__blk174_dn13_slot;
        let mut var_sel__blk174_dn2: f64 = *var_sel__blk174_dn2_slot;
        let mut var_sel__blk174_dn3: f64 = *var_sel__blk174_dn3_slot;
        let mut var_sel__blk174_dn4: f64 = *var_sel__blk174_dn4_slot;
        let mut var_sel__blk174_dn5: f64 = *var_sel__blk174_dn5_slot;
        let mut var_sel__blk174_dn6: f64 = *var_sel__blk174_dn6_slot;
        let mut var_sel__blk174_dn7: f64 = *var_sel__blk174_dn7_slot;
        let mut var_sel__blk174_dn8: f64 = *var_sel__blk174_dn8_slot;
        let mut var_sel__blk174_dn9: f64 = *var_sel__blk174_dn9_slot;
        let mut var_sgif: f64 = *var_sgif_slot;
        let mut var_vl0__blk169: f64 = *var_vl0__blk169_slot;
        let mut var_vl0__blk169_dn0: f64 = *var_vl0__blk169_dn0_slot;
        let mut var_vl0__blk169_dn1: f64 = *var_vl0__blk169_dn1_slot;
        let mut var_vl0__blk169_dn10: f64 = *var_vl0__blk169_dn10_slot;
        let mut var_vl0__blk169_dn11: f64 = *var_vl0__blk169_dn11_slot;
        let mut var_vl0__blk169_dn12: f64 = *var_vl0__blk169_dn12_slot;
        let mut var_vl0__blk169_dn13: f64 = *var_vl0__blk169_dn13_slot;
        let mut var_vl0__blk169_dn2: f64 = *var_vl0__blk169_dn2_slot;
        let mut var_vl0__blk169_dn3: f64 = *var_vl0__blk169_dn3_slot;
        let mut var_vl0__blk169_dn4: f64 = *var_vl0__blk169_dn4_slot;
        let mut var_vl0__blk169_dn5: f64 = *var_vl0__blk169_dn5_slot;
        let mut var_vl0__blk169_dn6: f64 = *var_vl0__blk169_dn6_slot;
        let mut var_vl0__blk169_dn7: f64 = *var_vl0__blk169_dn7_slot;
        let mut var_vl0__blk169_dn8: f64 = *var_vl0__blk169_dn8_slot;
        let mut var_vl0__blk169_dn9: f64 = *var_vl0__blk169_dn9_slot;
        let mut var_vl__blk173: f64 = *var_vl__blk173_slot;
        let mut var_vl__blk173_dn0: f64 = *var_vl__blk173_dn0_slot;
        let mut var_vl__blk173_dn1: f64 = *var_vl__blk173_dn1_slot;
        let mut var_vl__blk173_dn10: f64 = *var_vl__blk173_dn10_slot;
        let mut var_vl__blk173_dn11: f64 = *var_vl__blk173_dn11_slot;
        let mut var_vl__blk173_dn12: f64 = *var_vl__blk173_dn12_slot;
        let mut var_vl__blk173_dn13: f64 = *var_vl__blk173_dn13_slot;
        let mut var_vl__blk173_dn2: f64 = *var_vl__blk173_dn2_slot;
        let mut var_vl__blk173_dn3: f64 = *var_vl__blk173_dn3_slot;
        let mut var_vl__blk173_dn4: f64 = *var_vl__blk173_dn4_slot;
        let mut var_vl__blk173_dn5: f64 = *var_vl__blk173_dn5_slot;
        let mut var_vl__blk173_dn6: f64 = *var_vl__blk173_dn6_slot;
        let mut var_vl__blk173_dn7: f64 = *var_vl__blk173_dn7_slot;
        let mut var_vl__blk173_dn8: f64 = *var_vl__blk173_dn8_slot;
        let mut var_vl__blk173_dn9: f64 = *var_vl__blk173_dn9_slot;

        let (assign5320_e5863, assign5320_e5863_d_n0, assign5320_e5863_d_n1, assign5320_e5863_d_n2, assign5320_e5863_d_n3, assign5320_e5863_d_n4, assign5320_e5863_d_n5, assign5320_e5863_d_n6, assign5320_e5863_d_n7, assign5320_e5863_d_n8, assign5320_e5863_d_n9, assign5320_e5863_d_n10, assign5320_e5863_d_n11, assign5320_e5863_d_n12, assign5320_e5863_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 != 0.0)) {
        let assign5320_e5855: f64 = (p.p45 - var_dv0__blk162);
        let assign5320_e5856: f64 = (var_vnl__blk172 * assign5320_e5855);
        let assign5320_e5858: f64 = (assign5320_e5856 - p.p45);
        let assign5320_e5860: f64 = (assign5320_e5858 - var_dv0__blk162);
        let assign5320_e5861: f64 = (0.5 * assign5320_e5860);
        (assign5320_e5861, (0.5 * (((var_vnl__blk172_dn0 * assign5320_e5855) + (var_vnl__blk172 * (-var_dv0__blk162_dn0))) - var_dv0__blk162_dn0)), (0.5 * (((var_vnl__blk172_dn1 * assign5320_e5855) + (var_vnl__blk172 * (-var_dv0__blk162_dn1))) - var_dv0__blk162_dn1)), (0.5 * (((var_vnl__blk172_dn2 * assign5320_e5855) + (var_vnl__blk172 * (-var_dv0__blk162_dn2))) - var_dv0__blk162_dn2)), (0.5 * (((var_vnl__blk172_dn3 * assign5320_e5855) + (var_vnl__blk172 * (-var_dv0__blk162_dn3))) - var_dv0__blk162_dn3)), (0.5 * (((var_vnl__blk172_dn4 * assign5320_e5855) + (var_vnl__blk172 * (-var_dv0__blk162_dn4))) - var_dv0__blk162_dn4)), (0.5 * (((var_vnl__blk172_dn5 * assign5320_e5855) + (var_vnl__blk172 * (-var_dv0__blk162_dn5))) - var_dv0__blk162_dn5)), (0.5 * (((var_vnl__blk172_dn6 * assign5320_e5855) + (var_vnl__blk172 * (-var_dv0__blk162_dn6))) - var_dv0__blk162_dn6)), (0.5 * (((var_vnl__blk172_dn7 * assign5320_e5855) + (var_vnl__blk172 * (-var_dv0__blk162_dn7))) - var_dv0__blk162_dn7)), (0.5 * (((var_vnl__blk172_dn8 * assign5320_e5855) + (var_vnl__blk172 * (-var_dv0__blk162_dn8))) - var_dv0__blk162_dn8)), (0.5 * (((var_vnl__blk172_dn9 * assign5320_e5855) + (var_vnl__blk172 * (-var_dv0__blk162_dn9))) - var_dv0__blk162_dn9)), (0.5 * (((var_vnl__blk172_dn10 * assign5320_e5855) + (var_vnl__blk172 * (-var_dv0__blk162_dn10))) - var_dv0__blk162_dn10)), (0.5 * (((var_vnl__blk172_dn11 * assign5320_e5855) + (var_vnl__blk172 * (-var_dv0__blk162_dn11))) - var_dv0__blk162_dn11)), (0.5 * (((var_vnl__blk172_dn12 * assign5320_e5855) + (var_vnl__blk172 * (-var_dv0__blk162_dn12))) - var_dv0__blk162_dn12)), (0.5 * (((var_vnl__blk172_dn13 * assign5320_e5855) + (var_vnl__blk172 * (-var_dv0__blk162_dn13))) - var_dv0__blk162_dn13)),)
    } else {
        (var_vl__blk173, var_vl__blk173_dn0, var_vl__blk173_dn1, var_vl__blk173_dn2, var_vl__blk173_dn3, var_vl__blk173_dn4, var_vl__blk173_dn5, var_vl__blk173_dn6, var_vl__blk173_dn7, var_vl__blk173_dn8, var_vl__blk173_dn9, var_vl__blk173_dn10, var_vl__blk173_dn11, var_vl__blk173_dn12, var_vl__blk173_dn13,)
    }
};
        var_vl__blk173 = assign5320_e5863;
        var_vl__blk173_dn0 = assign5320_e5863_d_n0;
        var_vl__blk173_dn1 = assign5320_e5863_d_n1;
        var_vl__blk173_dn2 = assign5320_e5863_d_n2;
        var_vl__blk173_dn3 = assign5320_e5863_d_n3;
        var_vl__blk173_dn4 = assign5320_e5863_d_n4;
        var_vl__blk173_dn5 = assign5320_e5863_d_n5;
        var_vl__blk173_dn6 = assign5320_e5863_d_n6;
        var_vl__blk173_dn7 = assign5320_e5863_d_n7;
        var_vl__blk173_dn8 = assign5320_e5863_d_n8;
        var_vl__blk173_dn9 = assign5320_e5863_d_n9;
        var_vl__blk173_dn10 = assign5320_e5863_d_n10;
        var_vl__blk173_dn11 = assign5320_e5863_d_n11;
        var_vl__blk173_dn12 = assign5320_e5863_d_n12;
        var_vl__blk173_dn13 = assign5320_e5863_d_n13;

        let (assign5330_e5886, assign5330_e5886_d_n0, assign5330_e5886_d_n1, assign5330_e5886_d_n2, assign5330_e5886_d_n3, assign5330_e5886_d_n4, assign5330_e5886_d_n5, assign5330_e5886_d_n6, assign5330_e5886_d_n7, assign5330_e5886_d_n8, assign5330_e5886_d_n9, assign5330_e5886_d_n10, assign5330_e5886_d_n11, assign5330_e5886_d_n12, assign5330_e5886_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 != 0.0)) {
        let assign5330_e5873: f64 = (var_vl__blk173 / var_pc_t);
        let assign5330_e5874: f64 = (1.0 - assign5330_e5873);
        let assign5330_e5877: f64 = (1.0 - p.p43);
        let assign5330_e5878: f64 = (assign5330_e5874).powf(assign5330_e5877);
        let assign5330_e5879: f64 = (1.0 - assign5330_e5878);
        let assign5330_e5880: f64 = (var_pc_t * assign5330_e5879);
        let assign5330_e5883: f64 = (1.0 - p.p43);
        let assign5330_e5884: f64 = (assign5330_e5880 / assign5330_e5883);
        (assign5330_e5884, (((var_pc_t_dn0 * assign5330_e5879) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(((var_vl__blk173_dn0 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn0)) / (var_pc_t * var_pc_t))))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(((var_vl__blk173_dn0 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn0)) / (var_pc_t * var_pc_t))) / assign5330_e5874))) }))) / assign5330_e5883), (((var_pc_t_dn1 * assign5330_e5879) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(((var_vl__blk173_dn1 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn1)) / (var_pc_t * var_pc_t))))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(((var_vl__blk173_dn1 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn1)) / (var_pc_t * var_pc_t))) / assign5330_e5874))) }))) / assign5330_e5883), (((var_pc_t_dn2 * assign5330_e5879) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(((var_vl__blk173_dn2 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn2)) / (var_pc_t * var_pc_t))))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(((var_vl__blk173_dn2 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn2)) / (var_pc_t * var_pc_t))) / assign5330_e5874))) }))) / assign5330_e5883), (((var_pc_t_dn3 * assign5330_e5879) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(((var_vl__blk173_dn3 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn3)) / (var_pc_t * var_pc_t))))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(((var_vl__blk173_dn3 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn3)) / (var_pc_t * var_pc_t))) / assign5330_e5874))) }))) / assign5330_e5883), (((var_pc_t_dn4 * assign5330_e5879) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(((var_vl__blk173_dn4 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn4)) / (var_pc_t * var_pc_t))))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(((var_vl__blk173_dn4 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn4)) / (var_pc_t * var_pc_t))) / assign5330_e5874))) }))) / assign5330_e5883), (((var_pc_t_dn5 * assign5330_e5879) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(((var_vl__blk173_dn5 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn5)) / (var_pc_t * var_pc_t))))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(((var_vl__blk173_dn5 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn5)) / (var_pc_t * var_pc_t))) / assign5330_e5874))) }))) / assign5330_e5883), (((var_pc_t_dn6 * assign5330_e5879) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(((var_vl__blk173_dn6 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn6)) / (var_pc_t * var_pc_t))))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(((var_vl__blk173_dn6 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn6)) / (var_pc_t * var_pc_t))) / assign5330_e5874))) }))) / assign5330_e5883), (((var_pc_t_dn7 * assign5330_e5879) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(((var_vl__blk173_dn7 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn7)) / (var_pc_t * var_pc_t))))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(((var_vl__blk173_dn7 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn7)) / (var_pc_t * var_pc_t))) / assign5330_e5874))) }))) / assign5330_e5883), (((var_pc_t_dn8 * assign5330_e5879) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(((var_vl__blk173_dn8 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn8)) / (var_pc_t * var_pc_t))))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(((var_vl__blk173_dn8 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn8)) / (var_pc_t * var_pc_t))) / assign5330_e5874))) }))) / assign5330_e5883), (((var_pc_t_dn9 * assign5330_e5879) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(((var_vl__blk173_dn9 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn9)) / (var_pc_t * var_pc_t))))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(((var_vl__blk173_dn9 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn9)) / (var_pc_t * var_pc_t))) / assign5330_e5874))) }))) / assign5330_e5883), (((var_pc_t_dn10 * assign5330_e5879) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(((var_vl__blk173_dn10 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn10)) / (var_pc_t * var_pc_t))))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(((var_vl__blk173_dn10 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn10)) / (var_pc_t * var_pc_t))) / assign5330_e5874))) }))) / assign5330_e5883), (((var_pc_t_dn11 * assign5330_e5879) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(((var_vl__blk173_dn11 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn11)) / (var_pc_t * var_pc_t))))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(((var_vl__blk173_dn11 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn11)) / (var_pc_t * var_pc_t))) / assign5330_e5874))) }))) / assign5330_e5883), (((var_pc_t_dn12 * assign5330_e5879) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(((var_vl__blk173_dn12 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn12)) / (var_pc_t * var_pc_t))))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(((var_vl__blk173_dn12 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn12)) / (var_pc_t * var_pc_t))) / assign5330_e5874))) }))) / assign5330_e5883), (((var_pc_t_dn13 * assign5330_e5879) + (var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(((var_vl__blk173_dn13 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn13)) / (var_pc_t * var_pc_t))))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(((var_vl__blk173_dn13 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn13)) / (var_pc_t * var_pc_t))) / assign5330_e5874))) }))) / assign5330_e5883),)
    } else {
        (var_qlo__blk165, var_qlo__blk165_dn0, var_qlo__blk165_dn1, var_qlo__blk165_dn2, var_qlo__blk165_dn3, var_qlo__blk165_dn4, var_qlo__blk165_dn5, var_qlo__blk165_dn6, var_qlo__blk165_dn7, var_qlo__blk165_dn8, var_qlo__blk165_dn9, var_qlo__blk165_dn10, var_qlo__blk165_dn11, var_qlo__blk165_dn12, var_qlo__blk165_dn13,)
    }
};
        var_qlo__blk165 = assign5330_e5886;
        var_qlo__blk165_dn0 = assign5330_e5886_d_n0;
        var_qlo__blk165_dn1 = assign5330_e5886_d_n1;
        var_qlo__blk165_dn2 = assign5330_e5886_d_n2;
        var_qlo__blk165_dn3 = assign5330_e5886_d_n3;
        var_qlo__blk165_dn4 = assign5330_e5886_d_n4;
        var_qlo__blk165_dn5 = assign5330_e5886_d_n5;
        var_qlo__blk165_dn6 = assign5330_e5886_d_n6;
        var_qlo__blk165_dn7 = assign5330_e5886_d_n7;
        var_qlo__blk165_dn8 = assign5330_e5886_d_n8;
        var_qlo__blk165_dn9 = assign5330_e5886_d_n9;
        var_qlo__blk165_dn10 = assign5330_e5886_d_n10;
        var_qlo__blk165_dn11 = assign5330_e5886_d_n11;
        var_qlo__blk165_dn12 = assign5330_e5886_d_n12;
        var_qlo__blk165_dn13 = assign5330_e5886_d_n13;

        let (assign5340_e5897, assign5340_e5897_d_n0, assign5340_e5897_d_n1, assign5340_e5897_d_n2, assign5340_e5897_d_n3, assign5340_e5897_d_n4, assign5340_e5897_d_n5, assign5340_e5897_d_n6, assign5340_e5897_d_n7, assign5340_e5897_d_n8, assign5340_e5897_d_n9, assign5340_e5897_d_n10, assign5340_e5897_d_n11, assign5340_e5897_d_n12, assign5340_e5897_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 != 0.0)) {
        let assign5340_e5894: f64 = (var_vnl__blk172 + 1.0);
        let assign5340_e5895: f64 = (0.5 * assign5340_e5894);
        (assign5340_e5895, (0.5 * var_vnl__blk172_dn0), (0.5 * var_vnl__blk172_dn1), (0.5 * var_vnl__blk172_dn2), (0.5 * var_vnl__blk172_dn3), (0.5 * var_vnl__blk172_dn4), (0.5 * var_vnl__blk172_dn5), (0.5 * var_vnl__blk172_dn6), (0.5 * var_vnl__blk172_dn7), (0.5 * var_vnl__blk172_dn8), (0.5 * var_vnl__blk172_dn9), (0.5 * var_vnl__blk172_dn10), (0.5 * var_vnl__blk172_dn11), (0.5 * var_vnl__blk172_dn12), (0.5 * var_vnl__blk172_dn13),)
    } else {
        (var_sel__blk174, var_sel__blk174_dn0, var_sel__blk174_dn1, var_sel__blk174_dn2, var_sel__blk174_dn3, var_sel__blk174_dn4, var_sel__blk174_dn5, var_sel__blk174_dn6, var_sel__blk174_dn7, var_sel__blk174_dn8, var_sel__blk174_dn9, var_sel__blk174_dn10, var_sel__blk174_dn11, var_sel__blk174_dn12, var_sel__blk174_dn13,)
    }
};
        var_sel__blk174 = assign5340_e5897;
        var_sel__blk174_dn0 = assign5340_e5897_d_n0;
        var_sel__blk174_dn1 = assign5340_e5897_d_n1;
        var_sel__blk174_dn2 = assign5340_e5897_d_n2;
        var_sel__blk174_dn3 = assign5340_e5897_d_n3;
        var_sel__blk174_dn4 = assign5340_e5897_d_n4;
        var_sel__blk174_dn5 = assign5340_e5897_d_n5;
        var_sel__blk174_dn6 = assign5340_e5897_d_n6;
        var_sel__blk174_dn7 = assign5340_e5897_d_n7;
        var_sel__blk174_dn8 = assign5340_e5897_d_n8;
        var_sel__blk174_dn9 = assign5340_e5897_d_n9;
        var_sel__blk174_dn10 = assign5340_e5897_d_n10;
        var_sel__blk174_dn11 = assign5340_e5897_d_n11;
        var_sel__blk174_dn12 = assign5340_e5897_d_n12;
        var_sel__blk174_dn13 = assign5340_e5897_d_n13;

        let (assign5350_e5911, assign5350_e5911_d_n0, assign5350_e5911_d_n1, assign5350_e5911_d_n2, assign5350_e5911_d_n3, assign5350_e5911_d_n4, assign5350_e5911_d_n5, assign5350_e5911_d_n6, assign5350_e5911_d_n7, assign5350_e5911_d_n8, assign5350_e5911_d_n9, assign5350_e5911_d_n10, assign5350_e5911_d_n11, assign5350_e5911_d_n12, assign5350_e5911_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 != 0.0)) {
        let assign5350_e5905: f64 = (p.p45 / var_pc_t);
        let assign5350_e5906: f64 = (1.0 + assign5350_e5905);
        let assign5350_e5908: f64 = (-p.p43);
        let assign5350_e5909: f64 = (assign5350_e5906).powf(assign5350_e5908);
        (assign5350_e5909, if 0.0 == 0.0 && ((assign5350_e5908) as f64).is_finite() && ((assign5350_e5908) as f64).fract() == 0.0 { if assign5350_e5908 == 0.0 { 0.0 } else { (assign5350_e5908 * ((assign5350_e5906).powf(assign5350_e5908 - 1.0) * (-((p.p45 * var_pc_t_dn0) / (var_pc_t * var_pc_t))))) } } else { (assign5350_e5909 * (assign5350_e5908 * ((-((p.p45 * var_pc_t_dn0) / (var_pc_t * var_pc_t))) / assign5350_e5906))) }, if 0.0 == 0.0 && ((assign5350_e5908) as f64).is_finite() && ((assign5350_e5908) as f64).fract() == 0.0 { if assign5350_e5908 == 0.0 { 0.0 } else { (assign5350_e5908 * ((assign5350_e5906).powf(assign5350_e5908 - 1.0) * (-((p.p45 * var_pc_t_dn1) / (var_pc_t * var_pc_t))))) } } else { (assign5350_e5909 * (assign5350_e5908 * ((-((p.p45 * var_pc_t_dn1) / (var_pc_t * var_pc_t))) / assign5350_e5906))) }, if 0.0 == 0.0 && ((assign5350_e5908) as f64).is_finite() && ((assign5350_e5908) as f64).fract() == 0.0 { if assign5350_e5908 == 0.0 { 0.0 } else { (assign5350_e5908 * ((assign5350_e5906).powf(assign5350_e5908 - 1.0) * (-((p.p45 * var_pc_t_dn2) / (var_pc_t * var_pc_t))))) } } else { (assign5350_e5909 * (assign5350_e5908 * ((-((p.p45 * var_pc_t_dn2) / (var_pc_t * var_pc_t))) / assign5350_e5906))) }, if 0.0 == 0.0 && ((assign5350_e5908) as f64).is_finite() && ((assign5350_e5908) as f64).fract() == 0.0 { if assign5350_e5908 == 0.0 { 0.0 } else { (assign5350_e5908 * ((assign5350_e5906).powf(assign5350_e5908 - 1.0) * (-((p.p45 * var_pc_t_dn3) / (var_pc_t * var_pc_t))))) } } else { (assign5350_e5909 * (assign5350_e5908 * ((-((p.p45 * var_pc_t_dn3) / (var_pc_t * var_pc_t))) / assign5350_e5906))) }, if 0.0 == 0.0 && ((assign5350_e5908) as f64).is_finite() && ((assign5350_e5908) as f64).fract() == 0.0 { if assign5350_e5908 == 0.0 { 0.0 } else { (assign5350_e5908 * ((assign5350_e5906).powf(assign5350_e5908 - 1.0) * (-((p.p45 * var_pc_t_dn4) / (var_pc_t * var_pc_t))))) } } else { (assign5350_e5909 * (assign5350_e5908 * ((-((p.p45 * var_pc_t_dn4) / (var_pc_t * var_pc_t))) / assign5350_e5906))) }, if 0.0 == 0.0 && ((assign5350_e5908) as f64).is_finite() && ((assign5350_e5908) as f64).fract() == 0.0 { if assign5350_e5908 == 0.0 { 0.0 } else { (assign5350_e5908 * ((assign5350_e5906).powf(assign5350_e5908 - 1.0) * (-((p.p45 * var_pc_t_dn5) / (var_pc_t * var_pc_t))))) } } else { (assign5350_e5909 * (assign5350_e5908 * ((-((p.p45 * var_pc_t_dn5) / (var_pc_t * var_pc_t))) / assign5350_e5906))) }, if 0.0 == 0.0 && ((assign5350_e5908) as f64).is_finite() && ((assign5350_e5908) as f64).fract() == 0.0 { if assign5350_e5908 == 0.0 { 0.0 } else { (assign5350_e5908 * ((assign5350_e5906).powf(assign5350_e5908 - 1.0) * (-((p.p45 * var_pc_t_dn6) / (var_pc_t * var_pc_t))))) } } else { (assign5350_e5909 * (assign5350_e5908 * ((-((p.p45 * var_pc_t_dn6) / (var_pc_t * var_pc_t))) / assign5350_e5906))) }, if 0.0 == 0.0 && ((assign5350_e5908) as f64).is_finite() && ((assign5350_e5908) as f64).fract() == 0.0 { if assign5350_e5908 == 0.0 { 0.0 } else { (assign5350_e5908 * ((assign5350_e5906).powf(assign5350_e5908 - 1.0) * (-((p.p45 * var_pc_t_dn7) / (var_pc_t * var_pc_t))))) } } else { (assign5350_e5909 * (assign5350_e5908 * ((-((p.p45 * var_pc_t_dn7) / (var_pc_t * var_pc_t))) / assign5350_e5906))) }, if 0.0 == 0.0 && ((assign5350_e5908) as f64).is_finite() && ((assign5350_e5908) as f64).fract() == 0.0 { if assign5350_e5908 == 0.0 { 0.0 } else { (assign5350_e5908 * ((assign5350_e5906).powf(assign5350_e5908 - 1.0) * (-((p.p45 * var_pc_t_dn8) / (var_pc_t * var_pc_t))))) } } else { (assign5350_e5909 * (assign5350_e5908 * ((-((p.p45 * var_pc_t_dn8) / (var_pc_t * var_pc_t))) / assign5350_e5906))) }, if 0.0 == 0.0 && ((assign5350_e5908) as f64).is_finite() && ((assign5350_e5908) as f64).fract() == 0.0 { if assign5350_e5908 == 0.0 { 0.0 } else { (assign5350_e5908 * ((assign5350_e5906).powf(assign5350_e5908 - 1.0) * (-((p.p45 * var_pc_t_dn9) / (var_pc_t * var_pc_t))))) } } else { (assign5350_e5909 * (assign5350_e5908 * ((-((p.p45 * var_pc_t_dn9) / (var_pc_t * var_pc_t))) / assign5350_e5906))) }, if 0.0 == 0.0 && ((assign5350_e5908) as f64).is_finite() && ((assign5350_e5908) as f64).fract() == 0.0 { if assign5350_e5908 == 0.0 { 0.0 } else { (assign5350_e5908 * ((assign5350_e5906).powf(assign5350_e5908 - 1.0) * (-((p.p45 * var_pc_t_dn10) / (var_pc_t * var_pc_t))))) } } else { (assign5350_e5909 * (assign5350_e5908 * ((-((p.p45 * var_pc_t_dn10) / (var_pc_t * var_pc_t))) / assign5350_e5906))) }, if 0.0 == 0.0 && ((assign5350_e5908) as f64).is_finite() && ((assign5350_e5908) as f64).fract() == 0.0 { if assign5350_e5908 == 0.0 { 0.0 } else { (assign5350_e5908 * ((assign5350_e5906).powf(assign5350_e5908 - 1.0) * (-((p.p45 * var_pc_t_dn11) / (var_pc_t * var_pc_t))))) } } else { (assign5350_e5909 * (assign5350_e5908 * ((-((p.p45 * var_pc_t_dn11) / (var_pc_t * var_pc_t))) / assign5350_e5906))) }, if 0.0 == 0.0 && ((assign5350_e5908) as f64).is_finite() && ((assign5350_e5908) as f64).fract() == 0.0 { if assign5350_e5908 == 0.0 { 0.0 } else { (assign5350_e5908 * ((assign5350_e5906).powf(assign5350_e5908 - 1.0) * (-((p.p45 * var_pc_t_dn12) / (var_pc_t * var_pc_t))))) } } else { (assign5350_e5909 * (assign5350_e5908 * ((-((p.p45 * var_pc_t_dn12) / (var_pc_t * var_pc_t))) / assign5350_e5906))) }, if 0.0 == 0.0 && ((assign5350_e5908) as f64).is_finite() && ((assign5350_e5908) as f64).fract() == 0.0 { if assign5350_e5908 == 0.0 { 0.0 } else { (assign5350_e5908 * ((assign5350_e5906).powf(assign5350_e5908 - 1.0) * (-((p.p45 * var_pc_t_dn13) / (var_pc_t * var_pc_t))))) } } else { (assign5350_e5909 * (assign5350_e5908 * ((-((p.p45 * var_pc_t_dn13) / (var_pc_t * var_pc_t))) / assign5350_e5906))) },)
    } else {
        (var_crt__blk175, var_crt__blk175_dn0, var_crt__blk175_dn1, var_crt__blk175_dn2, var_crt__blk175_dn3, var_crt__blk175_dn4, var_crt__blk175_dn5, var_crt__blk175_dn6, var_crt__blk175_dn7, var_crt__blk175_dn8, var_crt__blk175_dn9, var_crt__blk175_dn10, var_crt__blk175_dn11, var_crt__blk175_dn12, var_crt__blk175_dn13,)
    }
};
        var_crt__blk175 = assign5350_e5911;
        var_crt__blk175_dn0 = assign5350_e5911_d_n0;
        var_crt__blk175_dn1 = assign5350_e5911_d_n1;
        var_crt__blk175_dn2 = assign5350_e5911_d_n2;
        var_crt__blk175_dn3 = assign5350_e5911_d_n3;
        var_crt__blk175_dn4 = assign5350_e5911_d_n4;
        var_crt__blk175_dn5 = assign5350_e5911_d_n5;
        var_crt__blk175_dn6 = assign5350_e5911_d_n6;
        var_crt__blk175_dn7 = assign5350_e5911_d_n7;
        var_crt__blk175_dn8 = assign5350_e5911_d_n8;
        var_crt__blk175_dn9 = assign5350_e5911_d_n9;
        var_crt__blk175_dn10 = assign5350_e5911_d_n10;
        var_crt__blk175_dn11 = assign5350_e5911_d_n11;
        var_crt__blk175_dn12 = assign5350_e5911_d_n12;
        var_crt__blk175_dn13 = assign5350_e5911_d_n13;

        let (assign5360_e5925, assign5360_e5925_d_n0, assign5360_e5925_d_n1, assign5360_e5925_d_n2, assign5360_e5925_d_n3, assign5360_e5925_d_n4, assign5360_e5925_d_n5, assign5360_e5925_d_n6, assign5360_e5925_d_n7, assign5360_e5925_d_n8, assign5360_e5925_d_n9, assign5360_e5925_d_n10, assign5360_e5925_d_n11, assign5360_e5925_d_n12, assign5360_e5925_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 != 0.0)) {
        let assign5360_e5919: f64 = (var_dv0__blk162 / var_pc_t);
        let assign5360_e5920: f64 = (1.0 + assign5360_e5919);
        let assign5360_e5922: f64 = (-p.p43);
        let assign5360_e5923: f64 = (assign5360_e5920).powf(assign5360_e5922);
        (assign5360_e5923, if 0.0 == 0.0 && ((assign5360_e5922) as f64).is_finite() && ((assign5360_e5922) as f64).fract() == 0.0 { if assign5360_e5922 == 0.0 { 0.0 } else { (assign5360_e5922 * ((assign5360_e5920).powf(assign5360_e5922 - 1.0) * (((var_dv0__blk162_dn0 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn0)) / (var_pc_t * var_pc_t)))) } } else { (assign5360_e5923 * (assign5360_e5922 * ((((var_dv0__blk162_dn0 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn0)) / (var_pc_t * var_pc_t)) / assign5360_e5920))) }, if 0.0 == 0.0 && ((assign5360_e5922) as f64).is_finite() && ((assign5360_e5922) as f64).fract() == 0.0 { if assign5360_e5922 == 0.0 { 0.0 } else { (assign5360_e5922 * ((assign5360_e5920).powf(assign5360_e5922 - 1.0) * (((var_dv0__blk162_dn1 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn1)) / (var_pc_t * var_pc_t)))) } } else { (assign5360_e5923 * (assign5360_e5922 * ((((var_dv0__blk162_dn1 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn1)) / (var_pc_t * var_pc_t)) / assign5360_e5920))) }, if 0.0 == 0.0 && ((assign5360_e5922) as f64).is_finite() && ((assign5360_e5922) as f64).fract() == 0.0 { if assign5360_e5922 == 0.0 { 0.0 } else { (assign5360_e5922 * ((assign5360_e5920).powf(assign5360_e5922 - 1.0) * (((var_dv0__blk162_dn2 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn2)) / (var_pc_t * var_pc_t)))) } } else { (assign5360_e5923 * (assign5360_e5922 * ((((var_dv0__blk162_dn2 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn2)) / (var_pc_t * var_pc_t)) / assign5360_e5920))) }, if 0.0 == 0.0 && ((assign5360_e5922) as f64).is_finite() && ((assign5360_e5922) as f64).fract() == 0.0 { if assign5360_e5922 == 0.0 { 0.0 } else { (assign5360_e5922 * ((assign5360_e5920).powf(assign5360_e5922 - 1.0) * (((var_dv0__blk162_dn3 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn3)) / (var_pc_t * var_pc_t)))) } } else { (assign5360_e5923 * (assign5360_e5922 * ((((var_dv0__blk162_dn3 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn3)) / (var_pc_t * var_pc_t)) / assign5360_e5920))) }, if 0.0 == 0.0 && ((assign5360_e5922) as f64).is_finite() && ((assign5360_e5922) as f64).fract() == 0.0 { if assign5360_e5922 == 0.0 { 0.0 } else { (assign5360_e5922 * ((assign5360_e5920).powf(assign5360_e5922 - 1.0) * (((var_dv0__blk162_dn4 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn4)) / (var_pc_t * var_pc_t)))) } } else { (assign5360_e5923 * (assign5360_e5922 * ((((var_dv0__blk162_dn4 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn4)) / (var_pc_t * var_pc_t)) / assign5360_e5920))) }, if 0.0 == 0.0 && ((assign5360_e5922) as f64).is_finite() && ((assign5360_e5922) as f64).fract() == 0.0 { if assign5360_e5922 == 0.0 { 0.0 } else { (assign5360_e5922 * ((assign5360_e5920).powf(assign5360_e5922 - 1.0) * (((var_dv0__blk162_dn5 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn5)) / (var_pc_t * var_pc_t)))) } } else { (assign5360_e5923 * (assign5360_e5922 * ((((var_dv0__blk162_dn5 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn5)) / (var_pc_t * var_pc_t)) / assign5360_e5920))) }, if 0.0 == 0.0 && ((assign5360_e5922) as f64).is_finite() && ((assign5360_e5922) as f64).fract() == 0.0 { if assign5360_e5922 == 0.0 { 0.0 } else { (assign5360_e5922 * ((assign5360_e5920).powf(assign5360_e5922 - 1.0) * (((var_dv0__blk162_dn6 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn6)) / (var_pc_t * var_pc_t)))) } } else { (assign5360_e5923 * (assign5360_e5922 * ((((var_dv0__blk162_dn6 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn6)) / (var_pc_t * var_pc_t)) / assign5360_e5920))) }, if 0.0 == 0.0 && ((assign5360_e5922) as f64).is_finite() && ((assign5360_e5922) as f64).fract() == 0.0 { if assign5360_e5922 == 0.0 { 0.0 } else { (assign5360_e5922 * ((assign5360_e5920).powf(assign5360_e5922 - 1.0) * (((var_dv0__blk162_dn7 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn7)) / (var_pc_t * var_pc_t)))) } } else { (assign5360_e5923 * (assign5360_e5922 * ((((var_dv0__blk162_dn7 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn7)) / (var_pc_t * var_pc_t)) / assign5360_e5920))) }, if 0.0 == 0.0 && ((assign5360_e5922) as f64).is_finite() && ((assign5360_e5922) as f64).fract() == 0.0 { if assign5360_e5922 == 0.0 { 0.0 } else { (assign5360_e5922 * ((assign5360_e5920).powf(assign5360_e5922 - 1.0) * (((var_dv0__blk162_dn8 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn8)) / (var_pc_t * var_pc_t)))) } } else { (assign5360_e5923 * (assign5360_e5922 * ((((var_dv0__blk162_dn8 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn8)) / (var_pc_t * var_pc_t)) / assign5360_e5920))) }, if 0.0 == 0.0 && ((assign5360_e5922) as f64).is_finite() && ((assign5360_e5922) as f64).fract() == 0.0 { if assign5360_e5922 == 0.0 { 0.0 } else { (assign5360_e5922 * ((assign5360_e5920).powf(assign5360_e5922 - 1.0) * (((var_dv0__blk162_dn9 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn9)) / (var_pc_t * var_pc_t)))) } } else { (assign5360_e5923 * (assign5360_e5922 * ((((var_dv0__blk162_dn9 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn9)) / (var_pc_t * var_pc_t)) / assign5360_e5920))) }, if 0.0 == 0.0 && ((assign5360_e5922) as f64).is_finite() && ((assign5360_e5922) as f64).fract() == 0.0 { if assign5360_e5922 == 0.0 { 0.0 } else { (assign5360_e5922 * ((assign5360_e5920).powf(assign5360_e5922 - 1.0) * (((var_dv0__blk162_dn10 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn10)) / (var_pc_t * var_pc_t)))) } } else { (assign5360_e5923 * (assign5360_e5922 * ((((var_dv0__blk162_dn10 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn10)) / (var_pc_t * var_pc_t)) / assign5360_e5920))) }, if 0.0 == 0.0 && ((assign5360_e5922) as f64).is_finite() && ((assign5360_e5922) as f64).fract() == 0.0 { if assign5360_e5922 == 0.0 { 0.0 } else { (assign5360_e5922 * ((assign5360_e5920).powf(assign5360_e5922 - 1.0) * (((var_dv0__blk162_dn11 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn11)) / (var_pc_t * var_pc_t)))) } } else { (assign5360_e5923 * (assign5360_e5922 * ((((var_dv0__blk162_dn11 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn11)) / (var_pc_t * var_pc_t)) / assign5360_e5920))) }, if 0.0 == 0.0 && ((assign5360_e5922) as f64).is_finite() && ((assign5360_e5922) as f64).fract() == 0.0 { if assign5360_e5922 == 0.0 { 0.0 } else { (assign5360_e5922 * ((assign5360_e5920).powf(assign5360_e5922 - 1.0) * (((var_dv0__blk162_dn12 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn12)) / (var_pc_t * var_pc_t)))) } } else { (assign5360_e5923 * (assign5360_e5922 * ((((var_dv0__blk162_dn12 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn12)) / (var_pc_t * var_pc_t)) / assign5360_e5920))) }, if 0.0 == 0.0 && ((assign5360_e5922) as f64).is_finite() && ((assign5360_e5922) as f64).fract() == 0.0 { if assign5360_e5922 == 0.0 { 0.0 } else { (assign5360_e5922 * ((assign5360_e5920).powf(assign5360_e5922 - 1.0) * (((var_dv0__blk162_dn13 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn13)) / (var_pc_t * var_pc_t)))) } } else { (assign5360_e5923 * (assign5360_e5922 * ((((var_dv0__blk162_dn13 * var_pc_t) - (var_dv0__blk162 * var_pc_t_dn13)) / (var_pc_t * var_pc_t)) / assign5360_e5920))) },)
    } else {
        (var_cmx__blk176, var_cmx__blk176_dn0, var_cmx__blk176_dn1, var_cmx__blk176_dn2, var_cmx__blk176_dn3, var_cmx__blk176_dn4, var_cmx__blk176_dn5, var_cmx__blk176_dn6, var_cmx__blk176_dn7, var_cmx__blk176_dn8, var_cmx__blk176_dn9, var_cmx__blk176_dn10, var_cmx__blk176_dn11, var_cmx__blk176_dn12, var_cmx__blk176_dn13,)
    }
};
        var_cmx__blk176 = assign5360_e5925;
        var_cmx__blk176_dn0 = assign5360_e5925_d_n0;
        var_cmx__blk176_dn1 = assign5360_e5925_d_n1;
        var_cmx__blk176_dn2 = assign5360_e5925_d_n2;
        var_cmx__blk176_dn3 = assign5360_e5925_d_n3;
        var_cmx__blk176_dn4 = assign5360_e5925_d_n4;
        var_cmx__blk176_dn5 = assign5360_e5925_d_n5;
        var_cmx__blk176_dn6 = assign5360_e5925_d_n6;
        var_cmx__blk176_dn7 = assign5360_e5925_d_n7;
        var_cmx__blk176_dn8 = assign5360_e5925_d_n8;
        var_cmx__blk176_dn9 = assign5360_e5925_d_n9;
        var_cmx__blk176_dn10 = assign5360_e5925_d_n10;
        var_cmx__blk176_dn11 = assign5360_e5925_d_n11;
        var_cmx__blk176_dn12 = assign5360_e5925_d_n12;
        var_cmx__blk176_dn13 = assign5360_e5925_d_n13;

        let (assign5370_e5940, assign5370_e5940_d_n0, assign5370_e5940_d_n1, assign5370_e5940_d_n2, assign5370_e5940_d_n3, assign5370_e5940_d_n4, assign5370_e5940_d_n5, assign5370_e5940_d_n6, assign5370_e5940_d_n7, assign5370_e5940_d_n8, assign5370_e5940_d_n9, assign5370_e5940_d_n10, assign5370_e5940_d_n11, assign5370_e5940_d_n12, assign5370_e5940_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 != 0.0)) {
        let assign5370_e5932: f64 = (1.0 - var_sel__blk174);
        let assign5370_e5934: f64 = (assign5370_e5932 * var_crt__blk175);
        let assign5370_e5937: f64 = (var_sel__blk174 * var_cmx__blk176);
        let assign5370_e5938: f64 = (assign5370_e5934 + assign5370_e5937);
        (assign5370_e5938, ((((-var_sel__blk174_dn0) * var_crt__blk175) + (assign5370_e5932 * var_crt__blk175_dn0)) + ((var_sel__blk174_dn0 * var_cmx__blk176) + (var_sel__blk174 * var_cmx__blk176_dn0))), ((((-var_sel__blk174_dn1) * var_crt__blk175) + (assign5370_e5932 * var_crt__blk175_dn1)) + ((var_sel__blk174_dn1 * var_cmx__blk176) + (var_sel__blk174 * var_cmx__blk176_dn1))), ((((-var_sel__blk174_dn2) * var_crt__blk175) + (assign5370_e5932 * var_crt__blk175_dn2)) + ((var_sel__blk174_dn2 * var_cmx__blk176) + (var_sel__blk174 * var_cmx__blk176_dn2))), ((((-var_sel__blk174_dn3) * var_crt__blk175) + (assign5370_e5932 * var_crt__blk175_dn3)) + ((var_sel__blk174_dn3 * var_cmx__blk176) + (var_sel__blk174 * var_cmx__blk176_dn3))), ((((-var_sel__blk174_dn4) * var_crt__blk175) + (assign5370_e5932 * var_crt__blk175_dn4)) + ((var_sel__blk174_dn4 * var_cmx__blk176) + (var_sel__blk174 * var_cmx__blk176_dn4))), ((((-var_sel__blk174_dn5) * var_crt__blk175) + (assign5370_e5932 * var_crt__blk175_dn5)) + ((var_sel__blk174_dn5 * var_cmx__blk176) + (var_sel__blk174 * var_cmx__blk176_dn5))), ((((-var_sel__blk174_dn6) * var_crt__blk175) + (assign5370_e5932 * var_crt__blk175_dn6)) + ((var_sel__blk174_dn6 * var_cmx__blk176) + (var_sel__blk174 * var_cmx__blk176_dn6))), ((((-var_sel__blk174_dn7) * var_crt__blk175) + (assign5370_e5932 * var_crt__blk175_dn7)) + ((var_sel__blk174_dn7 * var_cmx__blk176) + (var_sel__blk174 * var_cmx__blk176_dn7))), ((((-var_sel__blk174_dn8) * var_crt__blk175) + (assign5370_e5932 * var_crt__blk175_dn8)) + ((var_sel__blk174_dn8 * var_cmx__blk176) + (var_sel__blk174 * var_cmx__blk176_dn8))), ((((-var_sel__blk174_dn9) * var_crt__blk175) + (assign5370_e5932 * var_crt__blk175_dn9)) + ((var_sel__blk174_dn9 * var_cmx__blk176) + (var_sel__blk174 * var_cmx__blk176_dn9))), ((((-var_sel__blk174_dn10) * var_crt__blk175) + (assign5370_e5932 * var_crt__blk175_dn10)) + ((var_sel__blk174_dn10 * var_cmx__blk176) + (var_sel__blk174 * var_cmx__blk176_dn10))), ((((-var_sel__blk174_dn11) * var_crt__blk175) + (assign5370_e5932 * var_crt__blk175_dn11)) + ((var_sel__blk174_dn11 * var_cmx__blk176) + (var_sel__blk174 * var_cmx__blk176_dn11))), ((((-var_sel__blk174_dn12) * var_crt__blk175) + (assign5370_e5932 * var_crt__blk175_dn12)) + ((var_sel__blk174_dn12 * var_cmx__blk176) + (var_sel__blk174 * var_cmx__blk176_dn12))), ((((-var_sel__blk174_dn13) * var_crt__blk175) + (assign5370_e5932 * var_crt__blk175_dn13)) + ((var_sel__blk174_dn13 * var_cmx__blk176) + (var_sel__blk174 * var_cmx__blk176_dn13))),)
    } else {
        (var_cl__blk177, var_cl__blk177_dn0, var_cl__blk177_dn1, var_cl__blk177_dn2, var_cl__blk177_dn3, var_cl__blk177_dn4, var_cl__blk177_dn5, var_cl__blk177_dn6, var_cl__blk177_dn7, var_cl__blk177_dn8, var_cl__blk177_dn9, var_cl__blk177_dn10, var_cl__blk177_dn11, var_cl__blk177_dn12, var_cl__blk177_dn13,)
    }
};
        var_cl__blk177 = assign5370_e5940;
        var_cl__blk177_dn0 = assign5370_e5940_d_n0;
        var_cl__blk177_dn1 = assign5370_e5940_d_n1;
        var_cl__blk177_dn2 = assign5370_e5940_d_n2;
        var_cl__blk177_dn3 = assign5370_e5940_d_n3;
        var_cl__blk177_dn4 = assign5370_e5940_d_n4;
        var_cl__blk177_dn5 = assign5370_e5940_d_n5;
        var_cl__blk177_dn6 = assign5370_e5940_d_n6;
        var_cl__blk177_dn7 = assign5370_e5940_d_n7;
        var_cl__blk177_dn8 = assign5370_e5940_d_n8;
        var_cl__blk177_dn9 = assign5370_e5940_d_n9;
        var_cl__blk177_dn10 = assign5370_e5940_d_n10;
        var_cl__blk177_dn11 = assign5370_e5940_d_n11;
        var_cl__blk177_dn12 = assign5370_e5940_d_n12;
        var_cl__blk177_dn13 = assign5370_e5940_d_n13;

        let (assign5380_e5953, assign5380_e5953_d_n0, assign5380_e5953_d_n1, assign5380_e5953_d_n2, assign5380_e5953_d_n3, assign5380_e5953_d_n4, assign5380_e5953_d_n5, assign5380_e5953_d_n6, assign5380_e5953_d_n7, assign5380_e5953_d_n8, assign5380_e5953_d_n9, assign5380_e5953_d_n10, assign5380_e5953_d_n11, assign5380_e5953_d_n12, assign5380_e5953_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 != 0.0)) {
        let assign5380_e5947: f64 = (var_vbep - var_vl__blk173);
        let assign5380_e5949: f64 = (assign5380_e5947 + var_vl0__blk169);
        let assign5380_e5951: f64 = (assign5380_e5949 * var_cl__blk177);
        (assign5380_e5951, ((((var_vbep_dn0 - var_vl__blk173_dn0) + var_vl0__blk169_dn0) * var_cl__blk177) + (assign5380_e5949 * var_cl__blk177_dn0)), ((((var_vbep_dn1 - var_vl__blk173_dn1) + var_vl0__blk169_dn1) * var_cl__blk177) + (assign5380_e5949 * var_cl__blk177_dn1)), ((((var_vbep_dn2 - var_vl__blk173_dn2) + var_vl0__blk169_dn2) * var_cl__blk177) + (assign5380_e5949 * var_cl__blk177_dn2)), ((((var_vbep_dn3 - var_vl__blk173_dn3) + var_vl0__blk169_dn3) * var_cl__blk177) + (assign5380_e5949 * var_cl__blk177_dn3)), ((((var_vbep_dn4 - var_vl__blk173_dn4) + var_vl0__blk169_dn4) * var_cl__blk177) + (assign5380_e5949 * var_cl__blk177_dn4)), ((((var_vbep_dn5 - var_vl__blk173_dn5) + var_vl0__blk169_dn5) * var_cl__blk177) + (assign5380_e5949 * var_cl__blk177_dn5)), ((((var_vbep_dn6 - var_vl__blk173_dn6) + var_vl0__blk169_dn6) * var_cl__blk177) + (assign5380_e5949 * var_cl__blk177_dn6)), ((((var_vbep_dn7 - var_vl__blk173_dn7) + var_vl0__blk169_dn7) * var_cl__blk177) + (assign5380_e5949 * var_cl__blk177_dn7)), ((((var_vbep_dn8 - var_vl__blk173_dn8) + var_vl0__blk169_dn8) * var_cl__blk177) + (assign5380_e5949 * var_cl__blk177_dn8)), ((((var_vbep_dn9 - var_vl__blk173_dn9) + var_vl0__blk169_dn9) * var_cl__blk177) + (assign5380_e5949 * var_cl__blk177_dn9)), ((((var_vbep_dn10 - var_vl__blk173_dn10) + var_vl0__blk169_dn10) * var_cl__blk177) + (assign5380_e5949 * var_cl__blk177_dn10)), ((((var_vbep_dn11 - var_vl__blk173_dn11) + var_vl0__blk169_dn11) * var_cl__blk177) + (assign5380_e5949 * var_cl__blk177_dn11)), ((((var_vbep_dn12 - var_vl__blk173_dn12) + var_vl0__blk169_dn12) * var_cl__blk177) + (assign5380_e5949 * var_cl__blk177_dn12)), ((((var_vbep_dn13 - var_vl__blk173_dn13) + var_vl0__blk169_dn13) * var_cl__blk177) + (assign5380_e5949 * var_cl__blk177_dn13)),)
    } else {
        (var_ql__blk178, var_ql__blk178_dn0, var_ql__blk178_dn1, var_ql__blk178_dn2, var_ql__blk178_dn3, var_ql__blk178_dn4, var_ql__blk178_dn5, var_ql__blk178_dn6, var_ql__blk178_dn7, var_ql__blk178_dn8, var_ql__blk178_dn9, var_ql__blk178_dn10, var_ql__blk178_dn11, var_ql__blk178_dn12, var_ql__blk178_dn13,)
    }
};
        var_ql__blk178 = assign5380_e5953;
        var_ql__blk178_dn0 = assign5380_e5953_d_n0;
        var_ql__blk178_dn1 = assign5380_e5953_d_n1;
        var_ql__blk178_dn2 = assign5380_e5953_d_n2;
        var_ql__blk178_dn3 = assign5380_e5953_d_n3;
        var_ql__blk178_dn4 = assign5380_e5953_d_n4;
        var_ql__blk178_dn5 = assign5380_e5953_d_n5;
        var_ql__blk178_dn6 = assign5380_e5953_d_n6;
        var_ql__blk178_dn7 = assign5380_e5953_d_n7;
        var_ql__blk178_dn8 = assign5380_e5953_d_n8;
        var_ql__blk178_dn9 = assign5380_e5953_d_n9;
        var_ql__blk178_dn10 = assign5380_e5953_d_n10;
        var_ql__blk178_dn11 = assign5380_e5953_d_n11;
        var_ql__blk178_dn12 = assign5380_e5953_d_n12;
        var_ql__blk178_dn13 = assign5380_e5953_d_n13;

        let (assign5390_e5964, assign5390_e5964_d_n0, assign5390_e5964_d_n1, assign5390_e5964_d_n2, assign5390_e5964_d_n3, assign5390_e5964_d_n4, assign5390_e5964_d_n5, assign5390_e5964_d_n6, assign5390_e5964_d_n7, assign5390_e5964_d_n8, assign5390_e5964_d_n9, assign5390_e5964_d_n10, assign5390_e5964_d_n11, assign5390_e5964_d_n12, assign5390_e5964_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 != 0.0)) {
        let assign5390_e5960: f64 = (var_ql__blk178 + var_qlo__blk165);
        let assign5390_e5962: f64 = (assign5390_e5960 - var_qlo0__blk170);
        (assign5390_e5962, ((var_ql__blk178_dn0 + var_qlo__blk165_dn0) - var_qlo0__blk170_dn0), ((var_ql__blk178_dn1 + var_qlo__blk165_dn1) - var_qlo0__blk170_dn1), ((var_ql__blk178_dn2 + var_qlo__blk165_dn2) - var_qlo0__blk170_dn2), ((var_ql__blk178_dn3 + var_qlo__blk165_dn3) - var_qlo0__blk170_dn3), ((var_ql__blk178_dn4 + var_qlo__blk165_dn4) - var_qlo0__blk170_dn4), ((var_ql__blk178_dn5 + var_qlo__blk165_dn5) - var_qlo0__blk170_dn5), ((var_ql__blk178_dn6 + var_qlo__blk165_dn6) - var_qlo0__blk170_dn6), ((var_ql__blk178_dn7 + var_qlo__blk165_dn7) - var_qlo0__blk170_dn7), ((var_ql__blk178_dn8 + var_qlo__blk165_dn8) - var_qlo0__blk170_dn8), ((var_ql__blk178_dn9 + var_qlo__blk165_dn9) - var_qlo0__blk170_dn9), ((var_ql__blk178_dn10 + var_qlo__blk165_dn10) - var_qlo0__blk170_dn10), ((var_ql__blk178_dn11 + var_qlo__blk165_dn11) - var_qlo0__blk170_dn11), ((var_ql__blk178_dn12 + var_qlo__blk165_dn12) - var_qlo0__blk170_dn12), ((var_ql__blk178_dn13 + var_qlo__blk165_dn13) - var_qlo0__blk170_dn13),)
    } else {
        (var_qdbep, var_qdbep_dn0, var_qdbep_dn1, var_qdbep_dn2, var_qdbep_dn3, var_qdbep_dn4, var_qdbep_dn5, var_qdbep_dn6, var_qdbep_dn7, var_qdbep_dn8, var_qdbep_dn9, var_qdbep_dn10, var_qdbep_dn11, var_qdbep_dn12, var_qdbep_dn13,)
    }
};
        var_qdbep = assign5390_e5964;
        var_qdbep_dn0 = assign5390_e5964_d_n0;
        var_qdbep_dn1 = assign5390_e5964_d_n1;
        var_qdbep_dn2 = assign5390_e5964_d_n2;
        var_qdbep_dn3 = assign5390_e5964_d_n3;
        var_qdbep_dn4 = assign5390_e5964_d_n4;
        var_qdbep_dn5 = assign5390_e5964_d_n5;
        var_qdbep_dn6 = assign5390_e5964_d_n6;
        var_qdbep_dn7 = assign5390_e5964_d_n7;
        var_qdbep_dn8 = assign5390_e5964_d_n8;
        var_qdbep_dn9 = assign5390_e5964_d_n9;
        var_qdbep_dn10 = assign5390_e5964_d_n10;
        var_qdbep_dn11 = assign5390_e5964_d_n11;
        var_qdbep_dn12 = assign5390_e5964_d_n12;
        var_qdbep_dn13 = assign5390_e5964_d_n13;

        let (assign5400_e5981, assign5400_e5981_d_n0, assign5400_e5981_d_n1, assign5400_e5981_d_n2, assign5400_e5981_d_n3, assign5400_e5981_d_n4, assign5400_e5981_d_n5, assign5400_e5981_d_n6, assign5400_e5981_d_n7, assign5400_e5981_d_n8, assign5400_e5981_d_n9, assign5400_e5981_d_n10, assign5400_e5981_d_n11, assign5400_e5981_d_n12, assign5400_e5981_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 == 0.0)) {
        let assign5400_e5972: f64 = (var_dv0__blk162 * var_dv0__blk162);
        let assign5400_e5975: f64 = (4.0 * p.p44);
        let assign5400_e5977: f64 = (assign5400_e5975 * p.p44);
        let assign5400_e5978: f64 = (assign5400_e5972 + assign5400_e5977);
        let assign5400_e5979: f64 = (assign5400_e5978).sqrt();
        (assign5400_e5979, (((var_dv0__blk162_dn0 * var_dv0__blk162) + (var_dv0__blk162 * var_dv0__blk162_dn0)) / (2.0 * assign5400_e5979)), (((var_dv0__blk162_dn1 * var_dv0__blk162) + (var_dv0__blk162 * var_dv0__blk162_dn1)) / (2.0 * assign5400_e5979)), (((var_dv0__blk162_dn2 * var_dv0__blk162) + (var_dv0__blk162 * var_dv0__blk162_dn2)) / (2.0 * assign5400_e5979)), (((var_dv0__blk162_dn3 * var_dv0__blk162) + (var_dv0__blk162 * var_dv0__blk162_dn3)) / (2.0 * assign5400_e5979)), (((var_dv0__blk162_dn4 * var_dv0__blk162) + (var_dv0__blk162 * var_dv0__blk162_dn4)) / (2.0 * assign5400_e5979)), (((var_dv0__blk162_dn5 * var_dv0__blk162) + (var_dv0__blk162 * var_dv0__blk162_dn5)) / (2.0 * assign5400_e5979)), (((var_dv0__blk162_dn6 * var_dv0__blk162) + (var_dv0__blk162 * var_dv0__blk162_dn6)) / (2.0 * assign5400_e5979)), (((var_dv0__blk162_dn7 * var_dv0__blk162) + (var_dv0__blk162 * var_dv0__blk162_dn7)) / (2.0 * assign5400_e5979)), (((var_dv0__blk162_dn8 * var_dv0__blk162) + (var_dv0__blk162 * var_dv0__blk162_dn8)) / (2.0 * assign5400_e5979)), (((var_dv0__blk162_dn9 * var_dv0__blk162) + (var_dv0__blk162 * var_dv0__blk162_dn9)) / (2.0 * assign5400_e5979)), (((var_dv0__blk162_dn10 * var_dv0__blk162) + (var_dv0__blk162 * var_dv0__blk162_dn10)) / (2.0 * assign5400_e5979)), (((var_dv0__blk162_dn11 * var_dv0__blk162) + (var_dv0__blk162 * var_dv0__blk162_dn11)) / (2.0 * assign5400_e5979)), (((var_dv0__blk162_dn12 * var_dv0__blk162) + (var_dv0__blk162 * var_dv0__blk162_dn12)) / (2.0 * assign5400_e5979)), (((var_dv0__blk162_dn13 * var_dv0__blk162) + (var_dv0__blk162 * var_dv0__blk162_dn13)) / (2.0 * assign5400_e5979)),)
    } else {
        (var_mv0__blk179, var_mv0__blk179_dn0, var_mv0__blk179_dn1, var_mv0__blk179_dn2, var_mv0__blk179_dn3, var_mv0__blk179_dn4, var_mv0__blk179_dn5, var_mv0__blk179_dn6, var_mv0__blk179_dn7, var_mv0__blk179_dn8, var_mv0__blk179_dn9, var_mv0__blk179_dn10, var_mv0__blk179_dn11, var_mv0__blk179_dn12, var_mv0__blk179_dn13,)
    }
};
        var_mv0__blk179 = assign5400_e5981;
        var_mv0__blk179_dn0 = assign5400_e5981_d_n0;
        var_mv0__blk179_dn1 = assign5400_e5981_d_n1;
        var_mv0__blk179_dn2 = assign5400_e5981_d_n2;
        var_mv0__blk179_dn3 = assign5400_e5981_d_n3;
        var_mv0__blk179_dn4 = assign5400_e5981_d_n4;
        var_mv0__blk179_dn5 = assign5400_e5981_d_n5;
        var_mv0__blk179_dn6 = assign5400_e5981_d_n6;
        var_mv0__blk179_dn7 = assign5400_e5981_d_n7;
        var_mv0__blk179_dn8 = assign5400_e5981_d_n8;
        var_mv0__blk179_dn9 = assign5400_e5981_d_n9;
        var_mv0__blk179_dn10 = assign5400_e5981_d_n10;
        var_mv0__blk179_dn11 = assign5400_e5981_d_n11;
        var_mv0__blk179_dn12 = assign5400_e5981_d_n12;
        var_mv0__blk179_dn13 = assign5400_e5981_d_n13;

        let (assign5410_e5994, assign5410_e5994_d_n0, assign5410_e5994_d_n1, assign5410_e5994_d_n2, assign5410_e5994_d_n3, assign5410_e5994_d_n4, assign5410_e5994_d_n5, assign5410_e5994_d_n6, assign5410_e5994_d_n7, assign5410_e5994_d_n8, assign5410_e5994_d_n9, assign5410_e5994_d_n10, assign5410_e5994_d_n11, assign5410_e5994_d_n12, assign5410_e5994_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 == 0.0)) {
        let assign5410_e5988: f64 = (-0.5);
        let assign5410_e5991: f64 = (var_dv0__blk162 + var_mv0__blk179);
        let assign5410_e5992: f64 = (assign5410_e5988 * assign5410_e5991);
        (assign5410_e5992, (assign5410_e5988 * (var_dv0__blk162_dn0 + var_mv0__blk179_dn0)), (assign5410_e5988 * (var_dv0__blk162_dn1 + var_mv0__blk179_dn1)), (assign5410_e5988 * (var_dv0__blk162_dn2 + var_mv0__blk179_dn2)), (assign5410_e5988 * (var_dv0__blk162_dn3 + var_mv0__blk179_dn3)), (assign5410_e5988 * (var_dv0__blk162_dn4 + var_mv0__blk179_dn4)), (assign5410_e5988 * (var_dv0__blk162_dn5 + var_mv0__blk179_dn5)), (assign5410_e5988 * (var_dv0__blk162_dn6 + var_mv0__blk179_dn6)), (assign5410_e5988 * (var_dv0__blk162_dn7 + var_mv0__blk179_dn7)), (assign5410_e5988 * (var_dv0__blk162_dn8 + var_mv0__blk179_dn8)), (assign5410_e5988 * (var_dv0__blk162_dn9 + var_mv0__blk179_dn9)), (assign5410_e5988 * (var_dv0__blk162_dn10 + var_mv0__blk179_dn10)), (assign5410_e5988 * (var_dv0__blk162_dn11 + var_mv0__blk179_dn11)), (assign5410_e5988 * (var_dv0__blk162_dn12 + var_mv0__blk179_dn12)), (assign5410_e5988 * (var_dv0__blk162_dn13 + var_mv0__blk179_dn13)),)
    } else {
        (var_vl0__blk169, var_vl0__blk169_dn0, var_vl0__blk169_dn1, var_vl0__blk169_dn2, var_vl0__blk169_dn3, var_vl0__blk169_dn4, var_vl0__blk169_dn5, var_vl0__blk169_dn6, var_vl0__blk169_dn7, var_vl0__blk169_dn8, var_vl0__blk169_dn9, var_vl0__blk169_dn10, var_vl0__blk169_dn11, var_vl0__blk169_dn12, var_vl0__blk169_dn13,)
    }
};
        var_vl0__blk169 = assign5410_e5994;
        var_vl0__blk169_dn0 = assign5410_e5994_d_n0;
        var_vl0__blk169_dn1 = assign5410_e5994_d_n1;
        var_vl0__blk169_dn2 = assign5410_e5994_d_n2;
        var_vl0__blk169_dn3 = assign5410_e5994_d_n3;
        var_vl0__blk169_dn4 = assign5410_e5994_d_n4;
        var_vl0__blk169_dn5 = assign5410_e5994_d_n5;
        var_vl0__blk169_dn6 = assign5410_e5994_d_n6;
        var_vl0__blk169_dn7 = assign5410_e5994_d_n7;
        var_vl0__blk169_dn8 = assign5410_e5994_d_n8;
        var_vl0__blk169_dn9 = assign5410_e5994_d_n9;
        var_vl0__blk169_dn10 = assign5410_e5994_d_n10;
        var_vl0__blk169_dn11 = assign5410_e5994_d_n11;
        var_vl0__blk169_dn12 = assign5410_e5994_d_n12;
        var_vl0__blk169_dn13 = assign5410_e5994_d_n13;

        let (assign5420_e6017, assign5420_e6017_d_n0, assign5420_e6017_d_n1, assign5420_e6017_d_n2, assign5420_e6017_d_n3, assign5420_e6017_d_n4, assign5420_e6017_d_n5, assign5420_e6017_d_n6, assign5420_e6017_d_n7, assign5420_e6017_d_n8, assign5420_e6017_d_n9, assign5420_e6017_d_n10, assign5420_e6017_d_n11, assign5420_e6017_d_n12, assign5420_e6017_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 == 0.0)) {
        let assign5420_e6001: f64 = (-var_pc_t);
        let assign5420_e6005: f64 = (var_vl0__blk169 / var_pc_t);
        let assign5420_e6006: f64 = (1.0 - assign5420_e6005);
        let assign5420_e6009: f64 = (1.0 - p.p43);
        let assign5420_e6010: f64 = (assign5420_e6006).powf(assign5420_e6009);
        let assign5420_e6011: f64 = (assign5420_e6001 * assign5420_e6010);
        let assign5420_e6014: f64 = (1.0 - p.p43);
        let assign5420_e6015: f64 = (assign5420_e6011 / assign5420_e6014);
        (assign5420_e6015, ((((-var_pc_t_dn0) * assign5420_e6010) + (assign5420_e6001 * if 0.0 == 0.0 && ((assign5420_e6009) as f64).is_finite() && ((assign5420_e6009) as f64).fract() == 0.0 { if assign5420_e6009 == 0.0 { 0.0 } else { (assign5420_e6009 * ((assign5420_e6006).powf(assign5420_e6009 - 1.0) * (-(((var_vl0__blk169_dn0 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn0)) / (var_pc_t * var_pc_t))))) } } else { (assign5420_e6010 * (assign5420_e6009 * ((-(((var_vl0__blk169_dn0 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn0)) / (var_pc_t * var_pc_t))) / assign5420_e6006))) })) / assign5420_e6014), ((((-var_pc_t_dn1) * assign5420_e6010) + (assign5420_e6001 * if 0.0 == 0.0 && ((assign5420_e6009) as f64).is_finite() && ((assign5420_e6009) as f64).fract() == 0.0 { if assign5420_e6009 == 0.0 { 0.0 } else { (assign5420_e6009 * ((assign5420_e6006).powf(assign5420_e6009 - 1.0) * (-(((var_vl0__blk169_dn1 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn1)) / (var_pc_t * var_pc_t))))) } } else { (assign5420_e6010 * (assign5420_e6009 * ((-(((var_vl0__blk169_dn1 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn1)) / (var_pc_t * var_pc_t))) / assign5420_e6006))) })) / assign5420_e6014), ((((-var_pc_t_dn2) * assign5420_e6010) + (assign5420_e6001 * if 0.0 == 0.0 && ((assign5420_e6009) as f64).is_finite() && ((assign5420_e6009) as f64).fract() == 0.0 { if assign5420_e6009 == 0.0 { 0.0 } else { (assign5420_e6009 * ((assign5420_e6006).powf(assign5420_e6009 - 1.0) * (-(((var_vl0__blk169_dn2 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn2)) / (var_pc_t * var_pc_t))))) } } else { (assign5420_e6010 * (assign5420_e6009 * ((-(((var_vl0__blk169_dn2 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn2)) / (var_pc_t * var_pc_t))) / assign5420_e6006))) })) / assign5420_e6014), ((((-var_pc_t_dn3) * assign5420_e6010) + (assign5420_e6001 * if 0.0 == 0.0 && ((assign5420_e6009) as f64).is_finite() && ((assign5420_e6009) as f64).fract() == 0.0 { if assign5420_e6009 == 0.0 { 0.0 } else { (assign5420_e6009 * ((assign5420_e6006).powf(assign5420_e6009 - 1.0) * (-(((var_vl0__blk169_dn3 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn3)) / (var_pc_t * var_pc_t))))) } } else { (assign5420_e6010 * (assign5420_e6009 * ((-(((var_vl0__blk169_dn3 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn3)) / (var_pc_t * var_pc_t))) / assign5420_e6006))) })) / assign5420_e6014), ((((-var_pc_t_dn4) * assign5420_e6010) + (assign5420_e6001 * if 0.0 == 0.0 && ((assign5420_e6009) as f64).is_finite() && ((assign5420_e6009) as f64).fract() == 0.0 { if assign5420_e6009 == 0.0 { 0.0 } else { (assign5420_e6009 * ((assign5420_e6006).powf(assign5420_e6009 - 1.0) * (-(((var_vl0__blk169_dn4 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn4)) / (var_pc_t * var_pc_t))))) } } else { (assign5420_e6010 * (assign5420_e6009 * ((-(((var_vl0__blk169_dn4 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn4)) / (var_pc_t * var_pc_t))) / assign5420_e6006))) })) / assign5420_e6014), ((((-var_pc_t_dn5) * assign5420_e6010) + (assign5420_e6001 * if 0.0 == 0.0 && ((assign5420_e6009) as f64).is_finite() && ((assign5420_e6009) as f64).fract() == 0.0 { if assign5420_e6009 == 0.0 { 0.0 } else { (assign5420_e6009 * ((assign5420_e6006).powf(assign5420_e6009 - 1.0) * (-(((var_vl0__blk169_dn5 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn5)) / (var_pc_t * var_pc_t))))) } } else { (assign5420_e6010 * (assign5420_e6009 * ((-(((var_vl0__blk169_dn5 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn5)) / (var_pc_t * var_pc_t))) / assign5420_e6006))) })) / assign5420_e6014), ((((-var_pc_t_dn6) * assign5420_e6010) + (assign5420_e6001 * if 0.0 == 0.0 && ((assign5420_e6009) as f64).is_finite() && ((assign5420_e6009) as f64).fract() == 0.0 { if assign5420_e6009 == 0.0 { 0.0 } else { (assign5420_e6009 * ((assign5420_e6006).powf(assign5420_e6009 - 1.0) * (-(((var_vl0__blk169_dn6 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn6)) / (var_pc_t * var_pc_t))))) } } else { (assign5420_e6010 * (assign5420_e6009 * ((-(((var_vl0__blk169_dn6 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn6)) / (var_pc_t * var_pc_t))) / assign5420_e6006))) })) / assign5420_e6014), ((((-var_pc_t_dn7) * assign5420_e6010) + (assign5420_e6001 * if 0.0 == 0.0 && ((assign5420_e6009) as f64).is_finite() && ((assign5420_e6009) as f64).fract() == 0.0 { if assign5420_e6009 == 0.0 { 0.0 } else { (assign5420_e6009 * ((assign5420_e6006).powf(assign5420_e6009 - 1.0) * (-(((var_vl0__blk169_dn7 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn7)) / (var_pc_t * var_pc_t))))) } } else { (assign5420_e6010 * (assign5420_e6009 * ((-(((var_vl0__blk169_dn7 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn7)) / (var_pc_t * var_pc_t))) / assign5420_e6006))) })) / assign5420_e6014), ((((-var_pc_t_dn8) * assign5420_e6010) + (assign5420_e6001 * if 0.0 == 0.0 && ((assign5420_e6009) as f64).is_finite() && ((assign5420_e6009) as f64).fract() == 0.0 { if assign5420_e6009 == 0.0 { 0.0 } else { (assign5420_e6009 * ((assign5420_e6006).powf(assign5420_e6009 - 1.0) * (-(((var_vl0__blk169_dn8 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn8)) / (var_pc_t * var_pc_t))))) } } else { (assign5420_e6010 * (assign5420_e6009 * ((-(((var_vl0__blk169_dn8 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn8)) / (var_pc_t * var_pc_t))) / assign5420_e6006))) })) / assign5420_e6014), ((((-var_pc_t_dn9) * assign5420_e6010) + (assign5420_e6001 * if 0.0 == 0.0 && ((assign5420_e6009) as f64).is_finite() && ((assign5420_e6009) as f64).fract() == 0.0 { if assign5420_e6009 == 0.0 { 0.0 } else { (assign5420_e6009 * ((assign5420_e6006).powf(assign5420_e6009 - 1.0) * (-(((var_vl0__blk169_dn9 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn9)) / (var_pc_t * var_pc_t))))) } } else { (assign5420_e6010 * (assign5420_e6009 * ((-(((var_vl0__blk169_dn9 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn9)) / (var_pc_t * var_pc_t))) / assign5420_e6006))) })) / assign5420_e6014), ((((-var_pc_t_dn10) * assign5420_e6010) + (assign5420_e6001 * if 0.0 == 0.0 && ((assign5420_e6009) as f64).is_finite() && ((assign5420_e6009) as f64).fract() == 0.0 { if assign5420_e6009 == 0.0 { 0.0 } else { (assign5420_e6009 * ((assign5420_e6006).powf(assign5420_e6009 - 1.0) * (-(((var_vl0__blk169_dn10 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn10)) / (var_pc_t * var_pc_t))))) } } else { (assign5420_e6010 * (assign5420_e6009 * ((-(((var_vl0__blk169_dn10 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn10)) / (var_pc_t * var_pc_t))) / assign5420_e6006))) })) / assign5420_e6014), ((((-var_pc_t_dn11) * assign5420_e6010) + (assign5420_e6001 * if 0.0 == 0.0 && ((assign5420_e6009) as f64).is_finite() && ((assign5420_e6009) as f64).fract() == 0.0 { if assign5420_e6009 == 0.0 { 0.0 } else { (assign5420_e6009 * ((assign5420_e6006).powf(assign5420_e6009 - 1.0) * (-(((var_vl0__blk169_dn11 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn11)) / (var_pc_t * var_pc_t))))) } } else { (assign5420_e6010 * (assign5420_e6009 * ((-(((var_vl0__blk169_dn11 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn11)) / (var_pc_t * var_pc_t))) / assign5420_e6006))) })) / assign5420_e6014), ((((-var_pc_t_dn12) * assign5420_e6010) + (assign5420_e6001 * if 0.0 == 0.0 && ((assign5420_e6009) as f64).is_finite() && ((assign5420_e6009) as f64).fract() == 0.0 { if assign5420_e6009 == 0.0 { 0.0 } else { (assign5420_e6009 * ((assign5420_e6006).powf(assign5420_e6009 - 1.0) * (-(((var_vl0__blk169_dn12 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn12)) / (var_pc_t * var_pc_t))))) } } else { (assign5420_e6010 * (assign5420_e6009 * ((-(((var_vl0__blk169_dn12 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn12)) / (var_pc_t * var_pc_t))) / assign5420_e6006))) })) / assign5420_e6014), ((((-var_pc_t_dn13) * assign5420_e6010) + (assign5420_e6001 * if 0.0 == 0.0 && ((assign5420_e6009) as f64).is_finite() && ((assign5420_e6009) as f64).fract() == 0.0 { if assign5420_e6009 == 0.0 { 0.0 } else { (assign5420_e6009 * ((assign5420_e6006).powf(assign5420_e6009 - 1.0) * (-(((var_vl0__blk169_dn13 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn13)) / (var_pc_t * var_pc_t))))) } } else { (assign5420_e6010 * (assign5420_e6009 * ((-(((var_vl0__blk169_dn13 * var_pc_t) - (var_vl0__blk169 * var_pc_t_dn13)) / (var_pc_t * var_pc_t))) / assign5420_e6006))) })) / assign5420_e6014),)
    } else {
        (var_q0__blk180, var_q0__blk180_dn0, var_q0__blk180_dn1, var_q0__blk180_dn2, var_q0__blk180_dn3, var_q0__blk180_dn4, var_q0__blk180_dn5, var_q0__blk180_dn6, var_q0__blk180_dn7, var_q0__blk180_dn8, var_q0__blk180_dn9, var_q0__blk180_dn10, var_q0__blk180_dn11, var_q0__blk180_dn12, var_q0__blk180_dn13,)
    }
};
        var_q0__blk180 = assign5420_e6017;
        var_q0__blk180_dn0 = assign5420_e6017_d_n0;
        var_q0__blk180_dn1 = assign5420_e6017_d_n1;
        var_q0__blk180_dn2 = assign5420_e6017_d_n2;
        var_q0__blk180_dn3 = assign5420_e6017_d_n3;
        var_q0__blk180_dn4 = assign5420_e6017_d_n4;
        var_q0__blk180_dn5 = assign5420_e6017_d_n5;
        var_q0__blk180_dn6 = assign5420_e6017_d_n6;
        var_q0__blk180_dn7 = assign5420_e6017_d_n7;
        var_q0__blk180_dn8 = assign5420_e6017_d_n8;
        var_q0__blk180_dn9 = assign5420_e6017_d_n9;
        var_q0__blk180_dn10 = assign5420_e6017_d_n10;
        var_q0__blk180_dn11 = assign5420_e6017_d_n11;
        var_q0__blk180_dn12 = assign5420_e6017_d_n12;
        var_q0__blk180_dn13 = assign5420_e6017_d_n13;

        let (assign5430_e6027, assign5430_e6027_d_n0, assign5430_e6027_d_n1, assign5430_e6027_d_n2, assign5430_e6027_d_n3, assign5430_e6027_d_n4, assign5430_e6027_d_n5, assign5430_e6027_d_n6, assign5430_e6027_d_n7, assign5430_e6027_d_n8, assign5430_e6027_d_n9, assign5430_e6027_d_n10, assign5430_e6027_d_n11, assign5430_e6027_d_n12, assign5430_e6027_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 == 0.0)) {
        let assign5430_e6025: f64 = (var_vbep + var_dv0__blk162);
        (assign5430_e6025, (var_vbep_dn0 + var_dv0__blk162_dn0), (var_vbep_dn1 + var_dv0__blk162_dn1), (var_vbep_dn2 + var_dv0__blk162_dn2), (var_vbep_dn3 + var_dv0__blk162_dn3), (var_vbep_dn4 + var_dv0__blk162_dn4), (var_vbep_dn5 + var_dv0__blk162_dn5), (var_vbep_dn6 + var_dv0__blk162_dn6), (var_vbep_dn7 + var_dv0__blk162_dn7), (var_vbep_dn8 + var_dv0__blk162_dn8), (var_vbep_dn9 + var_dv0__blk162_dn9), (var_vbep_dn10 + var_dv0__blk162_dn10), (var_vbep_dn11 + var_dv0__blk162_dn11), (var_vbep_dn12 + var_dv0__blk162_dn12), (var_vbep_dn13 + var_dv0__blk162_dn13),)
    } else {
        (var_dv__blk181, var_dv__blk181_dn0, var_dv__blk181_dn1, var_dv__blk181_dn2, var_dv__blk181_dn3, var_dv__blk181_dn4, var_dv__blk181_dn5, var_dv__blk181_dn6, var_dv__blk181_dn7, var_dv__blk181_dn8, var_dv__blk181_dn9, var_dv__blk181_dn10, var_dv__blk181_dn11, var_dv__blk181_dn12, var_dv__blk181_dn13,)
    }
};
        var_dv__blk181 = assign5430_e6027;
        var_dv__blk181_dn0 = assign5430_e6027_d_n0;
        var_dv__blk181_dn1 = assign5430_e6027_d_n1;
        var_dv__blk181_dn2 = assign5430_e6027_d_n2;
        var_dv__blk181_dn3 = assign5430_e6027_d_n3;
        var_dv__blk181_dn4 = assign5430_e6027_d_n4;
        var_dv__blk181_dn5 = assign5430_e6027_d_n5;
        var_dv__blk181_dn6 = assign5430_e6027_d_n6;
        var_dv__blk181_dn7 = assign5430_e6027_d_n7;
        var_dv__blk181_dn8 = assign5430_e6027_d_n8;
        var_dv__blk181_dn9 = assign5430_e6027_d_n9;
        var_dv__blk181_dn10 = assign5430_e6027_d_n10;
        var_dv__blk181_dn11 = assign5430_e6027_d_n11;
        var_dv__blk181_dn12 = assign5430_e6027_d_n12;
        var_dv__blk181_dn13 = assign5430_e6027_d_n13;

        let (assign5440_e6044, assign5440_e6044_d_n0, assign5440_e6044_d_n1, assign5440_e6044_d_n2, assign5440_e6044_d_n3, assign5440_e6044_d_n4, assign5440_e6044_d_n5, assign5440_e6044_d_n6, assign5440_e6044_d_n7, assign5440_e6044_d_n8, assign5440_e6044_d_n9, assign5440_e6044_d_n10, assign5440_e6044_d_n11, assign5440_e6044_d_n12, assign5440_e6044_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 == 0.0)) {
        let assign5440_e6035: f64 = (var_dv__blk181 * var_dv__blk181);
        let assign5440_e6038: f64 = (4.0 * p.p44);
        let assign5440_e6040: f64 = (assign5440_e6038 * p.p44);
        let assign5440_e6041: f64 = (assign5440_e6035 + assign5440_e6040);
        let assign5440_e6042: f64 = (assign5440_e6041).sqrt();
        (assign5440_e6042, (((var_dv__blk181_dn0 * var_dv__blk181) + (var_dv__blk181 * var_dv__blk181_dn0)) / (2.0 * assign5440_e6042)), (((var_dv__blk181_dn1 * var_dv__blk181) + (var_dv__blk181 * var_dv__blk181_dn1)) / (2.0 * assign5440_e6042)), (((var_dv__blk181_dn2 * var_dv__blk181) + (var_dv__blk181 * var_dv__blk181_dn2)) / (2.0 * assign5440_e6042)), (((var_dv__blk181_dn3 * var_dv__blk181) + (var_dv__blk181 * var_dv__blk181_dn3)) / (2.0 * assign5440_e6042)), (((var_dv__blk181_dn4 * var_dv__blk181) + (var_dv__blk181 * var_dv__blk181_dn4)) / (2.0 * assign5440_e6042)), (((var_dv__blk181_dn5 * var_dv__blk181) + (var_dv__blk181 * var_dv__blk181_dn5)) / (2.0 * assign5440_e6042)), (((var_dv__blk181_dn6 * var_dv__blk181) + (var_dv__blk181 * var_dv__blk181_dn6)) / (2.0 * assign5440_e6042)), (((var_dv__blk181_dn7 * var_dv__blk181) + (var_dv__blk181 * var_dv__blk181_dn7)) / (2.0 * assign5440_e6042)), (((var_dv__blk181_dn8 * var_dv__blk181) + (var_dv__blk181 * var_dv__blk181_dn8)) / (2.0 * assign5440_e6042)), (((var_dv__blk181_dn9 * var_dv__blk181) + (var_dv__blk181 * var_dv__blk181_dn9)) / (2.0 * assign5440_e6042)), (((var_dv__blk181_dn10 * var_dv__blk181) + (var_dv__blk181 * var_dv__blk181_dn10)) / (2.0 * assign5440_e6042)), (((var_dv__blk181_dn11 * var_dv__blk181) + (var_dv__blk181 * var_dv__blk181_dn11)) / (2.0 * assign5440_e6042)), (((var_dv__blk181_dn12 * var_dv__blk181) + (var_dv__blk181 * var_dv__blk181_dn12)) / (2.0 * assign5440_e6042)), (((var_dv__blk181_dn13 * var_dv__blk181) + (var_dv__blk181 * var_dv__blk181_dn13)) / (2.0 * assign5440_e6042)),)
    } else {
        (var_mv__blk182, var_mv__blk182_dn0, var_mv__blk182_dn1, var_mv__blk182_dn2, var_mv__blk182_dn3, var_mv__blk182_dn4, var_mv__blk182_dn5, var_mv__blk182_dn6, var_mv__blk182_dn7, var_mv__blk182_dn8, var_mv__blk182_dn9, var_mv__blk182_dn10, var_mv__blk182_dn11, var_mv__blk182_dn12, var_mv__blk182_dn13,)
    }
};
        var_mv__blk182 = assign5440_e6044;
        var_mv__blk182_dn0 = assign5440_e6044_d_n0;
        var_mv__blk182_dn1 = assign5440_e6044_d_n1;
        var_mv__blk182_dn2 = assign5440_e6044_d_n2;
        var_mv__blk182_dn3 = assign5440_e6044_d_n3;
        var_mv__blk182_dn4 = assign5440_e6044_d_n4;
        var_mv__blk182_dn5 = assign5440_e6044_d_n5;
        var_mv__blk182_dn6 = assign5440_e6044_d_n6;
        var_mv__blk182_dn7 = assign5440_e6044_d_n7;
        var_mv__blk182_dn8 = assign5440_e6044_d_n8;
        var_mv__blk182_dn9 = assign5440_e6044_d_n9;
        var_mv__blk182_dn10 = assign5440_e6044_d_n10;
        var_mv__blk182_dn11 = assign5440_e6044_d_n11;
        var_mv__blk182_dn12 = assign5440_e6044_d_n12;
        var_mv__blk182_dn13 = assign5440_e6044_d_n13;

        let (assign5450_e6058, assign5450_e6058_d_n0, assign5450_e6058_d_n1, assign5450_e6058_d_n2, assign5450_e6058_d_n3, assign5450_e6058_d_n4, assign5450_e6058_d_n5, assign5450_e6058_d_n6, assign5450_e6058_d_n7, assign5450_e6058_d_n8, assign5450_e6058_d_n9, assign5450_e6058_d_n10, assign5450_e6058_d_n11, assign5450_e6058_d_n12, assign5450_e6058_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 == 0.0)) {
        let assign5450_e6053: f64 = (var_dv__blk181 - var_mv__blk182);
        let assign5450_e6054: f64 = (0.5 * assign5450_e6053);
        let assign5450_e6056: f64 = (assign5450_e6054 - var_dv0__blk162);
        (assign5450_e6056, ((0.5 * (var_dv__blk181_dn0 - var_mv__blk182_dn0)) - var_dv0__blk162_dn0), ((0.5 * (var_dv__blk181_dn1 - var_mv__blk182_dn1)) - var_dv0__blk162_dn1), ((0.5 * (var_dv__blk181_dn2 - var_mv__blk182_dn2)) - var_dv0__blk162_dn2), ((0.5 * (var_dv__blk181_dn3 - var_mv__blk182_dn3)) - var_dv0__blk162_dn3), ((0.5 * (var_dv__blk181_dn4 - var_mv__blk182_dn4)) - var_dv0__blk162_dn4), ((0.5 * (var_dv__blk181_dn5 - var_mv__blk182_dn5)) - var_dv0__blk162_dn5), ((0.5 * (var_dv__blk181_dn6 - var_mv__blk182_dn6)) - var_dv0__blk162_dn6), ((0.5 * (var_dv__blk181_dn7 - var_mv__blk182_dn7)) - var_dv0__blk162_dn7), ((0.5 * (var_dv__blk181_dn8 - var_mv__blk182_dn8)) - var_dv0__blk162_dn8), ((0.5 * (var_dv__blk181_dn9 - var_mv__blk182_dn9)) - var_dv0__blk162_dn9), ((0.5 * (var_dv__blk181_dn10 - var_mv__blk182_dn10)) - var_dv0__blk162_dn10), ((0.5 * (var_dv__blk181_dn11 - var_mv__blk182_dn11)) - var_dv0__blk162_dn11), ((0.5 * (var_dv__blk181_dn12 - var_mv__blk182_dn12)) - var_dv0__blk162_dn12), ((0.5 * (var_dv__blk181_dn13 - var_mv__blk182_dn13)) - var_dv0__blk162_dn13),)
    } else {
        (var_vl__blk173, var_vl__blk173_dn0, var_vl__blk173_dn1, var_vl__blk173_dn2, var_vl__blk173_dn3, var_vl__blk173_dn4, var_vl__blk173_dn5, var_vl__blk173_dn6, var_vl__blk173_dn7, var_vl__blk173_dn8, var_vl__blk173_dn9, var_vl__blk173_dn10, var_vl__blk173_dn11, var_vl__blk173_dn12, var_vl__blk173_dn13,)
    }
};
        var_vl__blk173 = assign5450_e6058;
        var_vl__blk173_dn0 = assign5450_e6058_d_n0;
        var_vl__blk173_dn1 = assign5450_e6058_d_n1;
        var_vl__blk173_dn2 = assign5450_e6058_d_n2;
        var_vl__blk173_dn3 = assign5450_e6058_d_n3;
        var_vl__blk173_dn4 = assign5450_e6058_d_n4;
        var_vl__blk173_dn5 = assign5450_e6058_d_n5;
        var_vl__blk173_dn6 = assign5450_e6058_d_n6;
        var_vl__blk173_dn7 = assign5450_e6058_d_n7;
        var_vl__blk173_dn8 = assign5450_e6058_d_n8;
        var_vl__blk173_dn9 = assign5450_e6058_d_n9;
        var_vl__blk173_dn10 = assign5450_e6058_d_n10;
        var_vl__blk173_dn11 = assign5450_e6058_d_n11;
        var_vl__blk173_dn12 = assign5450_e6058_d_n12;
        var_vl__blk173_dn13 = assign5450_e6058_d_n13;

        let (assign5460_e6081, assign5460_e6081_d_n0, assign5460_e6081_d_n1, assign5460_e6081_d_n2, assign5460_e6081_d_n3, assign5460_e6081_d_n4, assign5460_e6081_d_n5, assign5460_e6081_d_n6, assign5460_e6081_d_n7, assign5460_e6081_d_n8, assign5460_e6081_d_n9, assign5460_e6081_d_n10, assign5460_e6081_d_n11, assign5460_e6081_d_n12, assign5460_e6081_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 == 0.0)) {
        let assign5460_e6065: f64 = (-var_pc_t);
        let assign5460_e6069: f64 = (var_vl__blk173 / var_pc_t);
        let assign5460_e6070: f64 = (1.0 - assign5460_e6069);
        let assign5460_e6073: f64 = (1.0 - p.p43);
        let assign5460_e6074: f64 = (assign5460_e6070).powf(assign5460_e6073);
        let assign5460_e6075: f64 = (assign5460_e6065 * assign5460_e6074);
        let assign5460_e6078: f64 = (1.0 - p.p43);
        let assign5460_e6079: f64 = (assign5460_e6075 / assign5460_e6078);
        (assign5460_e6079, ((((-var_pc_t_dn0) * assign5460_e6074) + (assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(((var_vl__blk173_dn0 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn0)) / (var_pc_t * var_pc_t))))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(((var_vl__blk173_dn0 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn0)) / (var_pc_t * var_pc_t))) / assign5460_e6070))) })) / assign5460_e6078), ((((-var_pc_t_dn1) * assign5460_e6074) + (assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(((var_vl__blk173_dn1 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn1)) / (var_pc_t * var_pc_t))))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(((var_vl__blk173_dn1 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn1)) / (var_pc_t * var_pc_t))) / assign5460_e6070))) })) / assign5460_e6078), ((((-var_pc_t_dn2) * assign5460_e6074) + (assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(((var_vl__blk173_dn2 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn2)) / (var_pc_t * var_pc_t))))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(((var_vl__blk173_dn2 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn2)) / (var_pc_t * var_pc_t))) / assign5460_e6070))) })) / assign5460_e6078), ((((-var_pc_t_dn3) * assign5460_e6074) + (assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(((var_vl__blk173_dn3 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn3)) / (var_pc_t * var_pc_t))))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(((var_vl__blk173_dn3 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn3)) / (var_pc_t * var_pc_t))) / assign5460_e6070))) })) / assign5460_e6078), ((((-var_pc_t_dn4) * assign5460_e6074) + (assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(((var_vl__blk173_dn4 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn4)) / (var_pc_t * var_pc_t))))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(((var_vl__blk173_dn4 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn4)) / (var_pc_t * var_pc_t))) / assign5460_e6070))) })) / assign5460_e6078), ((((-var_pc_t_dn5) * assign5460_e6074) + (assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(((var_vl__blk173_dn5 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn5)) / (var_pc_t * var_pc_t))))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(((var_vl__blk173_dn5 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn5)) / (var_pc_t * var_pc_t))) / assign5460_e6070))) })) / assign5460_e6078), ((((-var_pc_t_dn6) * assign5460_e6074) + (assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(((var_vl__blk173_dn6 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn6)) / (var_pc_t * var_pc_t))))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(((var_vl__blk173_dn6 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn6)) / (var_pc_t * var_pc_t))) / assign5460_e6070))) })) / assign5460_e6078), ((((-var_pc_t_dn7) * assign5460_e6074) + (assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(((var_vl__blk173_dn7 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn7)) / (var_pc_t * var_pc_t))))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(((var_vl__blk173_dn7 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn7)) / (var_pc_t * var_pc_t))) / assign5460_e6070))) })) / assign5460_e6078), ((((-var_pc_t_dn8) * assign5460_e6074) + (assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(((var_vl__blk173_dn8 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn8)) / (var_pc_t * var_pc_t))))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(((var_vl__blk173_dn8 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn8)) / (var_pc_t * var_pc_t))) / assign5460_e6070))) })) / assign5460_e6078), ((((-var_pc_t_dn9) * assign5460_e6074) + (assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(((var_vl__blk173_dn9 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn9)) / (var_pc_t * var_pc_t))))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(((var_vl__blk173_dn9 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn9)) / (var_pc_t * var_pc_t))) / assign5460_e6070))) })) / assign5460_e6078), ((((-var_pc_t_dn10) * assign5460_e6074) + (assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(((var_vl__blk173_dn10 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn10)) / (var_pc_t * var_pc_t))))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(((var_vl__blk173_dn10 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn10)) / (var_pc_t * var_pc_t))) / assign5460_e6070))) })) / assign5460_e6078), ((((-var_pc_t_dn11) * assign5460_e6074) + (assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(((var_vl__blk173_dn11 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn11)) / (var_pc_t * var_pc_t))))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(((var_vl__blk173_dn11 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn11)) / (var_pc_t * var_pc_t))) / assign5460_e6070))) })) / assign5460_e6078), ((((-var_pc_t_dn12) * assign5460_e6074) + (assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(((var_vl__blk173_dn12 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn12)) / (var_pc_t * var_pc_t))))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(((var_vl__blk173_dn12 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn12)) / (var_pc_t * var_pc_t))) / assign5460_e6070))) })) / assign5460_e6078), ((((-var_pc_t_dn13) * assign5460_e6074) + (assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(((var_vl__blk173_dn13 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn13)) / (var_pc_t * var_pc_t))))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(((var_vl__blk173_dn13 * var_pc_t) - (var_vl__blk173 * var_pc_t_dn13)) / (var_pc_t * var_pc_t))) / assign5460_e6070))) })) / assign5460_e6078),)
    } else {
        (var_qlo__blk165, var_qlo__blk165_dn0, var_qlo__blk165_dn1, var_qlo__blk165_dn2, var_qlo__blk165_dn3, var_qlo__blk165_dn4, var_qlo__blk165_dn5, var_qlo__blk165_dn6, var_qlo__blk165_dn7, var_qlo__blk165_dn8, var_qlo__blk165_dn9, var_qlo__blk165_dn10, var_qlo__blk165_dn11, var_qlo__blk165_dn12, var_qlo__blk165_dn13,)
    }
};
        var_qlo__blk165 = assign5460_e6081;
        var_qlo__blk165_dn0 = assign5460_e6081_d_n0;
        var_qlo__blk165_dn1 = assign5460_e6081_d_n1;
        var_qlo__blk165_dn2 = assign5460_e6081_d_n2;
        var_qlo__blk165_dn3 = assign5460_e6081_d_n3;
        var_qlo__blk165_dn4 = assign5460_e6081_d_n4;
        var_qlo__blk165_dn5 = assign5460_e6081_d_n5;
        var_qlo__blk165_dn6 = assign5460_e6081_d_n6;
        var_qlo__blk165_dn7 = assign5460_e6081_d_n7;
        var_qlo__blk165_dn8 = assign5460_e6081_d_n8;
        var_qlo__blk165_dn9 = assign5460_e6081_d_n9;
        var_qlo__blk165_dn10 = assign5460_e6081_d_n10;
        var_qlo__blk165_dn11 = assign5460_e6081_d_n11;
        var_qlo__blk165_dn12 = assign5460_e6081_d_n12;
        var_qlo__blk165_dn13 = assign5460_e6081_d_n13;

        let (assign5470_e6104, assign5470_e6104_d_n0, assign5470_e6104_d_n1, assign5470_e6104_d_n2, assign5470_e6104_d_n3, assign5470_e6104_d_n4, assign5470_e6104_d_n5, assign5470_e6104_d_n6, assign5470_e6104_d_n7, assign5470_e6104_d_n8, assign5470_e6104_d_n9, assign5470_e6104_d_n10, assign5470_e6104_d_n11, assign5470_e6104_d_n12, assign5470_e6104_d_n13,) = {
    if ((var_guard183 == 0.0) && (var_guard186 == 0.0)) {
        let assign5470_e6090: f64 = (1.0 - p.p34);
        let assign5470_e6092: f64 = (-p.p43);
        let assign5470_e6093: f64 = (assign5470_e6090).powf(assign5470_e6092);
        let assign5470_e6096: f64 = (var_vbep - var_vl__blk173);
        let assign5470_e6098: f64 = (assign5470_e6096 + var_vl0__blk169);
        let assign5470_e6099: f64 = (assign5470_e6093 * assign5470_e6098);
        let assign5470_e6100: f64 = (var_qlo__blk165 + assign5470_e6099);
        let assign5470_e6102: f64 = (assign5470_e6100 - var_q0__blk180);
        (assign5470_e6102, ((var_qlo__blk165_dn0 + (assign5470_e6093 * ((var_vbep_dn0 - var_vl__blk173_dn0) + var_vl0__blk169_dn0))) - var_q0__blk180_dn0), ((var_qlo__blk165_dn1 + (assign5470_e6093 * ((var_vbep_dn1 - var_vl__blk173_dn1) + var_vl0__blk169_dn1))) - var_q0__blk180_dn1), ((var_qlo__blk165_dn2 + (assign5470_e6093 * ((var_vbep_dn2 - var_vl__blk173_dn2) + var_vl0__blk169_dn2))) - var_q0__blk180_dn2), ((var_qlo__blk165_dn3 + (assign5470_e6093 * ((var_vbep_dn3 - var_vl__blk173_dn3) + var_vl0__blk169_dn3))) - var_q0__blk180_dn3), ((var_qlo__blk165_dn4 + (assign5470_e6093 * ((var_vbep_dn4 - var_vl__blk173_dn4) + var_vl0__blk169_dn4))) - var_q0__blk180_dn4), ((var_qlo__blk165_dn5 + (assign5470_e6093 * ((var_vbep_dn5 - var_vl__blk173_dn5) + var_vl0__blk169_dn5))) - var_q0__blk180_dn5), ((var_qlo__blk165_dn6 + (assign5470_e6093 * ((var_vbep_dn6 - var_vl__blk173_dn6) + var_vl0__blk169_dn6))) - var_q0__blk180_dn6), ((var_qlo__blk165_dn7 + (assign5470_e6093 * ((var_vbep_dn7 - var_vl__blk173_dn7) + var_vl0__blk169_dn7))) - var_q0__blk180_dn7), ((var_qlo__blk165_dn8 + (assign5470_e6093 * ((var_vbep_dn8 - var_vl__blk173_dn8) + var_vl0__blk169_dn8))) - var_q0__blk180_dn8), ((var_qlo__blk165_dn9 + (assign5470_e6093 * ((var_vbep_dn9 - var_vl__blk173_dn9) + var_vl0__blk169_dn9))) - var_q0__blk180_dn9), ((var_qlo__blk165_dn10 + (assign5470_e6093 * ((var_vbep_dn10 - var_vl__blk173_dn10) + var_vl0__blk169_dn10))) - var_q0__blk180_dn10), ((var_qlo__blk165_dn11 + (assign5470_e6093 * ((var_vbep_dn11 - var_vl__blk173_dn11) + var_vl0__blk169_dn11))) - var_q0__blk180_dn11), ((var_qlo__blk165_dn12 + (assign5470_e6093 * ((var_vbep_dn12 - var_vl__blk173_dn12) + var_vl0__blk169_dn12))) - var_q0__blk180_dn12), ((var_qlo__blk165_dn13 + (assign5470_e6093 * ((var_vbep_dn13 - var_vl__blk173_dn13) + var_vl0__blk169_dn13))) - var_q0__blk180_dn13),)
    } else {
        (var_qdbep, var_qdbep_dn0, var_qdbep_dn1, var_qdbep_dn2, var_qdbep_dn3, var_qdbep_dn4, var_qdbep_dn5, var_qdbep_dn6, var_qdbep_dn7, var_qdbep_dn8, var_qdbep_dn9, var_qdbep_dn10, var_qdbep_dn11, var_qdbep_dn12, var_qdbep_dn13,)
    }
};
        var_qdbep = assign5470_e6104;
        var_qdbep_dn0 = assign5470_e6104_d_n0;
        var_qdbep_dn1 = assign5470_e6104_d_n1;
        var_qdbep_dn2 = assign5470_e6104_d_n2;
        var_qdbep_dn3 = assign5470_e6104_d_n3;
        var_qdbep_dn4 = assign5470_e6104_d_n4;
        var_qdbep_dn5 = assign5470_e6104_d_n5;
        var_qdbep_dn6 = assign5470_e6104_d_n6;
        var_qdbep_dn7 = assign5470_e6104_d_n7;
        var_qdbep_dn8 = assign5470_e6104_d_n8;
        var_qdbep_dn9 = assign5470_e6104_d_n9;
        var_qdbep_dn10 = assign5470_e6104_d_n10;
        var_qdbep_dn11 = assign5470_e6104_d_n11;
        var_qdbep_dn12 = assign5470_e6104_d_n12;
        var_qdbep_dn13 = assign5470_e6104_d_n13;

        let (assign5480_e6110,) = {
    if (var_ifi > 0.0) {
        (1.0,)
    } else {
        (0.0,)
    }
};
        var_sgif = assign5480_e6110;

        let assign5490_e6113: f64 = (var_ifi * var_sgif);
        let assign5490_e6115: f64 = (assign5490_e6113 * var_iitf);
        var_rif = assign5490_e6115;
        var_rif_dn0 = ((var_ifi_dn0 * var_sgif) * var_iitf);
        var_rif_dn1 = ((var_ifi_dn1 * var_sgif) * var_iitf);
        var_rif_dn2 = ((var_ifi_dn2 * var_sgif) * var_iitf);
        var_rif_dn3 = ((var_ifi_dn3 * var_sgif) * var_iitf);
        var_rif_dn4 = ((var_ifi_dn4 * var_sgif) * var_iitf);
        var_rif_dn5 = ((var_ifi_dn5 * var_sgif) * var_iitf);
        var_rif_dn6 = ((var_ifi_dn6 * var_sgif) * var_iitf);
        var_rif_dn7 = ((var_ifi_dn7 * var_sgif) * var_iitf);
        var_rif_dn8 = ((var_ifi_dn8 * var_sgif) * var_iitf);
        var_rif_dn9 = ((var_ifi_dn9 * var_sgif) * var_iitf);
        var_rif_dn10 = ((var_ifi_dn10 * var_sgif) * var_iitf);
        var_rif_dn11 = ((var_ifi_dn11 * var_sgif) * var_iitf);
        var_rif_dn12 = ((var_ifi_dn12 * var_sgif) * var_iitf);
        var_rif_dn13 = ((var_ifi_dn13 * var_sgif) * var_iitf);

        let assign5500_e6119: f64 = (var_rif + 1.0);
        let assign5500_e6120: f64 = (var_rif / assign5500_e6119);
        var_mif = assign5500_e6120;
        var_mif_dn0 = (((var_rif_dn0 * assign5500_e6119) - (var_rif * var_rif_dn0)) / (assign5500_e6119 * assign5500_e6119));
        var_mif_dn1 = (((var_rif_dn1 * assign5500_e6119) - (var_rif * var_rif_dn1)) / (assign5500_e6119 * assign5500_e6119));
        var_mif_dn2 = (((var_rif_dn2 * assign5500_e6119) - (var_rif * var_rif_dn2)) / (assign5500_e6119 * assign5500_e6119));
        var_mif_dn3 = (((var_rif_dn3 * assign5500_e6119) - (var_rif * var_rif_dn3)) / (assign5500_e6119 * assign5500_e6119));
        var_mif_dn4 = (((var_rif_dn4 * assign5500_e6119) - (var_rif * var_rif_dn4)) / (assign5500_e6119 * assign5500_e6119));
        var_mif_dn5 = (((var_rif_dn5 * assign5500_e6119) - (var_rif * var_rif_dn5)) / (assign5500_e6119 * assign5500_e6119));
        var_mif_dn6 = (((var_rif_dn6 * assign5500_e6119) - (var_rif * var_rif_dn6)) / (assign5500_e6119 * assign5500_e6119));
        var_mif_dn7 = (((var_rif_dn7 * assign5500_e6119) - (var_rif * var_rif_dn7)) / (assign5500_e6119 * assign5500_e6119));
        var_mif_dn8 = (((var_rif_dn8 * assign5500_e6119) - (var_rif * var_rif_dn8)) / (assign5500_e6119 * assign5500_e6119));
        var_mif_dn9 = (((var_rif_dn9 * assign5500_e6119) - (var_rif * var_rif_dn9)) / (assign5500_e6119 * assign5500_e6119));
        var_mif_dn10 = (((var_rif_dn10 * assign5500_e6119) - (var_rif * var_rif_dn10)) / (assign5500_e6119 * assign5500_e6119));
        var_mif_dn11 = (((var_rif_dn11 * assign5500_e6119) - (var_rif * var_rif_dn11)) / (assign5500_e6119 * assign5500_e6119));
        var_mif_dn12 = (((var_rif_dn12 * assign5500_e6119) - (var_rif * var_rif_dn12)) / (assign5500_e6119 * assign5500_e6119));
        var_mif_dn13 = (((var_rif_dn13 * assign5500_e6119) - (var_rif * var_rif_dn13)) / (assign5500_e6119 * assign5500_e6119));

        let assign5510_e6123: f64 = (var_vbci * var_ivtf);
        let assign5510_e6125: f64 = (assign5510_e6123 / 1.44);
        var_arg = assign5510_e6125;
        var_arg_dn0 = ((var_vbci_dn0 * var_ivtf) / 1.44);
        var_arg_dn1 = ((var_vbci_dn1 * var_ivtf) / 1.44);
        var_arg_dn2 = ((var_vbci_dn2 * var_ivtf) / 1.44);
        var_arg_dn3 = ((var_vbci_dn3 * var_ivtf) / 1.44);
        var_arg_dn4 = ((var_vbci_dn4 * var_ivtf) / 1.44);
        var_arg_dn5 = ((var_vbci_dn5 * var_ivtf) / 1.44);
        var_arg_dn6 = ((var_vbci_dn6 * var_ivtf) / 1.44);
        var_arg_dn7 = ((var_vbci_dn7 * var_ivtf) / 1.44);
        var_arg_dn8 = ((var_vbci_dn8 * var_ivtf) / 1.44);
        var_arg_dn9 = ((var_vbci_dn9 * var_ivtf) / 1.44);
        var_arg_dn10 = ((var_vbci_dn10 * var_ivtf) / 1.44);
        var_arg_dn11 = ((var_vbci_dn11 * var_ivtf) / 1.44);
        var_arg_dn12 = ((var_vbci_dn12 * var_ivtf) / 1.44);
        var_arg_dn13 = ((var_vbci_dn13 * var_ivtf) / 1.44);

        let assign5520_e6128: f64 = if var_arg < var_vmaxexp { 1.0 } else { 0.0 };
        var_guard187 = assign5520_e6128;


        *var_arg_slot = var_arg;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn1_slot = var_arg_dn1;
        *var_arg_dn10_slot = var_arg_dn10;
        *var_arg_dn11_slot = var_arg_dn11;
        *var_arg_dn12_slot = var_arg_dn12;
        *var_arg_dn13_slot = var_arg_dn13;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_arg_dn8_slot = var_arg_dn8;
        *var_arg_dn9_slot = var_arg_dn9;
        *var_cl__blk177_slot = var_cl__blk177;
        *var_cl__blk177_dn0_slot = var_cl__blk177_dn0;
        *var_cl__blk177_dn1_slot = var_cl__blk177_dn1;
        *var_cl__blk177_dn10_slot = var_cl__blk177_dn10;
        *var_cl__blk177_dn11_slot = var_cl__blk177_dn11;
        *var_cl__blk177_dn12_slot = var_cl__blk177_dn12;
        *var_cl__blk177_dn13_slot = var_cl__blk177_dn13;
        *var_cl__blk177_dn2_slot = var_cl__blk177_dn2;
        *var_cl__blk177_dn3_slot = var_cl__blk177_dn3;
        *var_cl__blk177_dn4_slot = var_cl__blk177_dn4;
        *var_cl__blk177_dn5_slot = var_cl__blk177_dn5;
        *var_cl__blk177_dn6_slot = var_cl__blk177_dn6;
        *var_cl__blk177_dn7_slot = var_cl__blk177_dn7;
        *var_cl__blk177_dn8_slot = var_cl__blk177_dn8;
        *var_cl__blk177_dn9_slot = var_cl__blk177_dn9;
        *var_cmx__blk176_slot = var_cmx__blk176;
        *var_cmx__blk176_dn0_slot = var_cmx__blk176_dn0;
        *var_cmx__blk176_dn1_slot = var_cmx__blk176_dn1;
        *var_cmx__blk176_dn10_slot = var_cmx__blk176_dn10;
        *var_cmx__blk176_dn11_slot = var_cmx__blk176_dn11;
        *var_cmx__blk176_dn12_slot = var_cmx__blk176_dn12;
        *var_cmx__blk176_dn13_slot = var_cmx__blk176_dn13;
        *var_cmx__blk176_dn2_slot = var_cmx__blk176_dn2;
        *var_cmx__blk176_dn3_slot = var_cmx__blk176_dn3;
        *var_cmx__blk176_dn4_slot = var_cmx__blk176_dn4;
        *var_cmx__blk176_dn5_slot = var_cmx__blk176_dn5;
        *var_cmx__blk176_dn6_slot = var_cmx__blk176_dn6;
        *var_cmx__blk176_dn7_slot = var_cmx__blk176_dn7;
        *var_cmx__blk176_dn8_slot = var_cmx__blk176_dn8;
        *var_cmx__blk176_dn9_slot = var_cmx__blk176_dn9;
        *var_crt__blk175_slot = var_crt__blk175;
        *var_crt__blk175_dn0_slot = var_crt__blk175_dn0;
        *var_crt__blk175_dn1_slot = var_crt__blk175_dn1;
        *var_crt__blk175_dn10_slot = var_crt__blk175_dn10;
        *var_crt__blk175_dn11_slot = var_crt__blk175_dn11;
        *var_crt__blk175_dn12_slot = var_crt__blk175_dn12;
        *var_crt__blk175_dn13_slot = var_crt__blk175_dn13;
        *var_crt__blk175_dn2_slot = var_crt__blk175_dn2;
        *var_crt__blk175_dn3_slot = var_crt__blk175_dn3;
        *var_crt__blk175_dn4_slot = var_crt__blk175_dn4;
        *var_crt__blk175_dn5_slot = var_crt__blk175_dn5;
        *var_crt__blk175_dn6_slot = var_crt__blk175_dn6;
        *var_crt__blk175_dn7_slot = var_crt__blk175_dn7;
        *var_crt__blk175_dn8_slot = var_crt__blk175_dn8;
        *var_crt__blk175_dn9_slot = var_crt__blk175_dn9;
        *var_dv__blk181_slot = var_dv__blk181;
        *var_dv__blk181_dn0_slot = var_dv__blk181_dn0;
        *var_dv__blk181_dn1_slot = var_dv__blk181_dn1;
        *var_dv__blk181_dn10_slot = var_dv__blk181_dn10;
        *var_dv__blk181_dn11_slot = var_dv__blk181_dn11;
        *var_dv__blk181_dn12_slot = var_dv__blk181_dn12;
        *var_dv__blk181_dn13_slot = var_dv__blk181_dn13;
        *var_dv__blk181_dn2_slot = var_dv__blk181_dn2;
        *var_dv__blk181_dn3_slot = var_dv__blk181_dn3;
        *var_dv__blk181_dn4_slot = var_dv__blk181_dn4;
        *var_dv__blk181_dn5_slot = var_dv__blk181_dn5;
        *var_dv__blk181_dn6_slot = var_dv__blk181_dn6;
        *var_dv__blk181_dn7_slot = var_dv__blk181_dn7;
        *var_dv__blk181_dn8_slot = var_dv__blk181_dn8;
        *var_dv__blk181_dn9_slot = var_dv__blk181_dn9;
        *var_guard187_slot = var_guard187;
        *var_mif_slot = var_mif;
        *var_mif_dn0_slot = var_mif_dn0;
        *var_mif_dn1_slot = var_mif_dn1;
        *var_mif_dn10_slot = var_mif_dn10;
        *var_mif_dn11_slot = var_mif_dn11;
        *var_mif_dn12_slot = var_mif_dn12;
        *var_mif_dn13_slot = var_mif_dn13;
        *var_mif_dn2_slot = var_mif_dn2;
        *var_mif_dn3_slot = var_mif_dn3;
        *var_mif_dn4_slot = var_mif_dn4;
        *var_mif_dn5_slot = var_mif_dn5;
        *var_mif_dn6_slot = var_mif_dn6;
        *var_mif_dn7_slot = var_mif_dn7;
        *var_mif_dn8_slot = var_mif_dn8;
        *var_mif_dn9_slot = var_mif_dn9;
        *var_mv0__blk179_slot = var_mv0__blk179;
        *var_mv0__blk179_dn0_slot = var_mv0__blk179_dn0;
        *var_mv0__blk179_dn1_slot = var_mv0__blk179_dn1;
        *var_mv0__blk179_dn10_slot = var_mv0__blk179_dn10;
        *var_mv0__blk179_dn11_slot = var_mv0__blk179_dn11;
        *var_mv0__blk179_dn12_slot = var_mv0__blk179_dn12;
        *var_mv0__blk179_dn13_slot = var_mv0__blk179_dn13;
        *var_mv0__blk179_dn2_slot = var_mv0__blk179_dn2;
        *var_mv0__blk179_dn3_slot = var_mv0__blk179_dn3;
        *var_mv0__blk179_dn4_slot = var_mv0__blk179_dn4;
        *var_mv0__blk179_dn5_slot = var_mv0__blk179_dn5;
        *var_mv0__blk179_dn6_slot = var_mv0__blk179_dn6;
        *var_mv0__blk179_dn7_slot = var_mv0__blk179_dn7;
        *var_mv0__blk179_dn8_slot = var_mv0__blk179_dn8;
        *var_mv0__blk179_dn9_slot = var_mv0__blk179_dn9;
        *var_mv__blk182_slot = var_mv__blk182;
        *var_mv__blk182_dn0_slot = var_mv__blk182_dn0;
        *var_mv__blk182_dn1_slot = var_mv__blk182_dn1;
        *var_mv__blk182_dn10_slot = var_mv__blk182_dn10;
        *var_mv__blk182_dn11_slot = var_mv__blk182_dn11;
        *var_mv__blk182_dn12_slot = var_mv__blk182_dn12;
        *var_mv__blk182_dn13_slot = var_mv__blk182_dn13;
        *var_mv__blk182_dn2_slot = var_mv__blk182_dn2;
        *var_mv__blk182_dn3_slot = var_mv__blk182_dn3;
        *var_mv__blk182_dn4_slot = var_mv__blk182_dn4;
        *var_mv__blk182_dn5_slot = var_mv__blk182_dn5;
        *var_mv__blk182_dn6_slot = var_mv__blk182_dn6;
        *var_mv__blk182_dn7_slot = var_mv__blk182_dn7;
        *var_mv__blk182_dn8_slot = var_mv__blk182_dn8;
        *var_mv__blk182_dn9_slot = var_mv__blk182_dn9;
        *var_q0__blk180_slot = var_q0__blk180;
        *var_q0__blk180_dn0_slot = var_q0__blk180_dn0;
        *var_q0__blk180_dn1_slot = var_q0__blk180_dn1;
        *var_q0__blk180_dn10_slot = var_q0__blk180_dn10;
        *var_q0__blk180_dn11_slot = var_q0__blk180_dn11;
        *var_q0__blk180_dn12_slot = var_q0__blk180_dn12;
        *var_q0__blk180_dn13_slot = var_q0__blk180_dn13;
        *var_q0__blk180_dn2_slot = var_q0__blk180_dn2;
        *var_q0__blk180_dn3_slot = var_q0__blk180_dn3;
        *var_q0__blk180_dn4_slot = var_q0__blk180_dn4;
        *var_q0__blk180_dn5_slot = var_q0__blk180_dn5;
        *var_q0__blk180_dn6_slot = var_q0__blk180_dn6;
        *var_q0__blk180_dn7_slot = var_q0__blk180_dn7;
        *var_q0__blk180_dn8_slot = var_q0__blk180_dn8;
        *var_q0__blk180_dn9_slot = var_q0__blk180_dn9;
        *var_qdbep_slot = var_qdbep;
        *var_qdbep_dn0_slot = var_qdbep_dn0;
        *var_qdbep_dn1_slot = var_qdbep_dn1;
        *var_qdbep_dn10_slot = var_qdbep_dn10;
        *var_qdbep_dn11_slot = var_qdbep_dn11;
        *var_qdbep_dn12_slot = var_qdbep_dn12;
        *var_qdbep_dn13_slot = var_qdbep_dn13;
        *var_qdbep_dn2_slot = var_qdbep_dn2;
        *var_qdbep_dn3_slot = var_qdbep_dn3;
        *var_qdbep_dn4_slot = var_qdbep_dn4;
        *var_qdbep_dn5_slot = var_qdbep_dn5;
        *var_qdbep_dn6_slot = var_qdbep_dn6;
        *var_qdbep_dn7_slot = var_qdbep_dn7;
        *var_qdbep_dn8_slot = var_qdbep_dn8;
        *var_qdbep_dn9_slot = var_qdbep_dn9;
        *var_ql__blk178_slot = var_ql__blk178;
        *var_ql__blk178_dn0_slot = var_ql__blk178_dn0;
        *var_ql__blk178_dn1_slot = var_ql__blk178_dn1;
        *var_ql__blk178_dn10_slot = var_ql__blk178_dn10;
        *var_ql__blk178_dn11_slot = var_ql__blk178_dn11;
        *var_ql__blk178_dn12_slot = var_ql__blk178_dn12;
        *var_ql__blk178_dn13_slot = var_ql__blk178_dn13;
        *var_ql__blk178_dn2_slot = var_ql__blk178_dn2;
        *var_ql__blk178_dn3_slot = var_ql__blk178_dn3;
        *var_ql__blk178_dn4_slot = var_ql__blk178_dn4;
        *var_ql__blk178_dn5_slot = var_ql__blk178_dn5;
        *var_ql__blk178_dn6_slot = var_ql__blk178_dn6;
        *var_ql__blk178_dn7_slot = var_ql__blk178_dn7;
        *var_ql__blk178_dn8_slot = var_ql__blk178_dn8;
        *var_ql__blk178_dn9_slot = var_ql__blk178_dn9;
        *var_qlo__blk165_slot = var_qlo__blk165;
        *var_qlo__blk165_dn0_slot = var_qlo__blk165_dn0;
        *var_qlo__blk165_dn1_slot = var_qlo__blk165_dn1;
        *var_qlo__blk165_dn10_slot = var_qlo__blk165_dn10;
        *var_qlo__blk165_dn11_slot = var_qlo__blk165_dn11;
        *var_qlo__blk165_dn12_slot = var_qlo__blk165_dn12;
        *var_qlo__blk165_dn13_slot = var_qlo__blk165_dn13;
        *var_qlo__blk165_dn2_slot = var_qlo__blk165_dn2;
        *var_qlo__blk165_dn3_slot = var_qlo__blk165_dn3;
        *var_qlo__blk165_dn4_slot = var_qlo__blk165_dn4;
        *var_qlo__blk165_dn5_slot = var_qlo__blk165_dn5;
        *var_qlo__blk165_dn6_slot = var_qlo__blk165_dn6;
        *var_qlo__blk165_dn7_slot = var_qlo__blk165_dn7;
        *var_qlo__blk165_dn8_slot = var_qlo__blk165_dn8;
        *var_qlo__blk165_dn9_slot = var_qlo__blk165_dn9;
        *var_rif_slot = var_rif;
        *var_rif_dn0_slot = var_rif_dn0;
        *var_rif_dn1_slot = var_rif_dn1;
        *var_rif_dn10_slot = var_rif_dn10;
        *var_rif_dn11_slot = var_rif_dn11;
        *var_rif_dn12_slot = var_rif_dn12;
        *var_rif_dn13_slot = var_rif_dn13;
        *var_rif_dn2_slot = var_rif_dn2;
        *var_rif_dn3_slot = var_rif_dn3;
        *var_rif_dn4_slot = var_rif_dn4;
        *var_rif_dn5_slot = var_rif_dn5;
        *var_rif_dn6_slot = var_rif_dn6;
        *var_rif_dn7_slot = var_rif_dn7;
        *var_rif_dn8_slot = var_rif_dn8;
        *var_rif_dn9_slot = var_rif_dn9;
        *var_sel__blk174_slot = var_sel__blk174;
        *var_sel__blk174_dn0_slot = var_sel__blk174_dn0;
        *var_sel__blk174_dn1_slot = var_sel__blk174_dn1;
        *var_sel__blk174_dn10_slot = var_sel__blk174_dn10;
        *var_sel__blk174_dn11_slot = var_sel__blk174_dn11;
        *var_sel__blk174_dn12_slot = var_sel__blk174_dn12;
        *var_sel__blk174_dn13_slot = var_sel__blk174_dn13;
        *var_sel__blk174_dn2_slot = var_sel__blk174_dn2;
        *var_sel__blk174_dn3_slot = var_sel__blk174_dn3;
        *var_sel__blk174_dn4_slot = var_sel__blk174_dn4;
        *var_sel__blk174_dn5_slot = var_sel__blk174_dn5;
        *var_sel__blk174_dn6_slot = var_sel__blk174_dn6;
        *var_sel__blk174_dn7_slot = var_sel__blk174_dn7;
        *var_sel__blk174_dn8_slot = var_sel__blk174_dn8;
        *var_sel__blk174_dn9_slot = var_sel__blk174_dn9;
        *var_sgif_slot = var_sgif;
        *var_vl0__blk169_slot = var_vl0__blk169;
        *var_vl0__blk169_dn0_slot = var_vl0__blk169_dn0;
        *var_vl0__blk169_dn1_slot = var_vl0__blk169_dn1;
        *var_vl0__blk169_dn10_slot = var_vl0__blk169_dn10;
        *var_vl0__blk169_dn11_slot = var_vl0__blk169_dn11;
        *var_vl0__blk169_dn12_slot = var_vl0__blk169_dn12;
        *var_vl0__blk169_dn13_slot = var_vl0__blk169_dn13;
        *var_vl0__blk169_dn2_slot = var_vl0__blk169_dn2;
        *var_vl0__blk169_dn3_slot = var_vl0__blk169_dn3;
        *var_vl0__blk169_dn4_slot = var_vl0__blk169_dn4;
        *var_vl0__blk169_dn5_slot = var_vl0__blk169_dn5;
        *var_vl0__blk169_dn6_slot = var_vl0__blk169_dn6;
        *var_vl0__blk169_dn7_slot = var_vl0__blk169_dn7;
        *var_vl0__blk169_dn8_slot = var_vl0__blk169_dn8;
        *var_vl0__blk169_dn9_slot = var_vl0__blk169_dn9;
        *var_vl__blk173_slot = var_vl__blk173;
        *var_vl__blk173_dn0_slot = var_vl__blk173_dn0;
        *var_vl__blk173_dn1_slot = var_vl__blk173_dn1;
        *var_vl__blk173_dn10_slot = var_vl__blk173_dn10;
        *var_vl__blk173_dn11_slot = var_vl__blk173_dn11;
        *var_vl__blk173_dn12_slot = var_vl__blk173_dn12;
        *var_vl__blk173_dn13_slot = var_vl__blk173_dn13;
        *var_vl__blk173_dn2_slot = var_vl__blk173_dn2;
        *var_vl__blk173_dn3_slot = var_vl__blk173_dn3;
        *var_vl__blk173_dn4_slot = var_vl__blk173_dn4;
        *var_vl__blk173_dn5_slot = var_vl__blk173_dn5;
        *var_vl__blk173_dn6_slot = var_vl__blk173_dn6;
        *var_vl__blk173_dn7_slot = var_vl__blk173_dn7;
        *var_vl__blk173_dn8_slot = var_vl__blk173_dn8;
        *var_vl__blk173_dn9_slot = var_vl__blk173_dn9;
    }

    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        var_arg: f64,
        var_arg_dn0: f64,
        var_arg_dn1: f64,
        var_arg_dn10: f64,
        var_arg_dn11: f64,
        var_arg_dn12: f64,
        var_arg_dn13: f64,
        var_arg_dn2: f64,
        var_arg_dn3: f64,
        var_arg_dn4: f64,
        var_arg_dn5: f64,
        var_arg_dn6: f64,
        var_arg_dn7: f64,
        var_arg_dn8: f64,
        var_arg_dn9: f64,
        var_cjc_t: f64,
        var_cjc_t_dn0: f64,
        var_cjc_t_dn1: f64,
        var_cjc_t_dn10: f64,
        var_cjc_t_dn11: f64,
        var_cjc_t_dn12: f64,
        var_cjc_t_dn13: f64,
        var_cjc_t_dn2: f64,
        var_cjc_t_dn3: f64,
        var_cjc_t_dn4: f64,
        var_cjc_t_dn5: f64,
        var_cjc_t_dn6: f64,
        var_cjc_t_dn7: f64,
        var_cjc_t_dn8: f64,
        var_cjc_t_dn9: f64,
        var_cjcp_t: f64,
        var_cjcp_t_dn0: f64,
        var_cjcp_t_dn1: f64,
        var_cjcp_t_dn10: f64,
        var_cjcp_t_dn11: f64,
        var_cjcp_t_dn12: f64,
        var_cjcp_t_dn13: f64,
        var_cjcp_t_dn2: f64,
        var_cjcp_t_dn3: f64,
        var_cjcp_t_dn4: f64,
        var_cjcp_t_dn5: f64,
        var_cjcp_t_dn6: f64,
        var_cjcp_t_dn7: f64,
        var_cjcp_t_dn8: f64,
        var_cjcp_t_dn9: f64,
        var_cje_t: f64,
        var_cje_t_dn0: f64,
        var_cje_t_dn1: f64,
        var_cje_t_dn10: f64,
        var_cje_t_dn11: f64,
        var_cje_t_dn12: f64,
        var_cje_t_dn13: f64,
        var_cje_t_dn2: f64,
        var_cje_t_dn3: f64,
        var_cje_t_dn4: f64,
        var_cje_t_dn5: f64,
        var_cje_t_dn6: f64,
        var_cje_t_dn7: f64,
        var_cje_t_dn8: f64,
        var_cje_t_dn9: f64,
        var_cjep_t: f64,
        var_cjep_t_dn0: f64,
        var_cjep_t_dn1: f64,
        var_cjep_t_dn10: f64,
        var_cjep_t_dn11: f64,
        var_cjep_t_dn12: f64,
        var_cjep_t_dn13: f64,
        var_cjep_t_dn2: f64,
        var_cjep_t_dn3: f64,
        var_cjep_t_dn4: f64,
        var_cjep_t_dn5: f64,
        var_cjep_t_dn6: f64,
        var_cjep_t_dn7: f64,
        var_cjep_t_dn8: f64,
        var_cjep_t_dn9: f64,
        var_dt_et: f64,
        var_dt_et_dn0: f64,
        var_dt_et_dn1: f64,
        var_dt_et_dn10: f64,
        var_dt_et_dn11: f64,
        var_dt_et_dn12: f64,
        var_dt_et_dn13: f64,
        var_dt_et_dn2: f64,
        var_dt_et_dn3: f64,
        var_dt_et_dn4: f64,
        var_dt_et_dn5: f64,
        var_dt_et_dn6: f64,
        var_dt_et_dn7: f64,
        var_dt_et_dn8: f64,
        var_dt_et_dn9: f64,
        var_guard187: f64,
        var_ifi: f64,
        var_ifi_dn0: f64,
        var_ifi_dn1: f64,
        var_ifi_dn10: f64,
        var_ifi_dn11: f64,
        var_ifi_dn12: f64,
        var_ifi_dn13: f64,
        var_ifi_dn2: f64,
        var_ifi_dn3: f64,
        var_ifi_dn4: f64,
        var_ifi_dn5: f64,
        var_ifi_dn6: f64,
        var_ifi_dn7: f64,
        var_ifi_dn8: f64,
        var_ifi_dn9: f64,
        var_ifp: f64,
        var_ifp_dn0: f64,
        var_ifp_dn1: f64,
        var_ifp_dn10: f64,
        var_ifp_dn11: f64,
        var_ifp_dn12: f64,
        var_ifp_dn13: f64,
        var_ifp_dn2: f64,
        var_ifp_dn3: f64,
        var_ifp_dn4: f64,
        var_ifp_dn5: f64,
        var_ifp_dn6: f64,
        var_ifp_dn7: f64,
        var_ifp_dn8: f64,
        var_ifp_dn9: f64,
        var_iri: f64,
        var_iri_dn0: f64,
        var_iri_dn1: f64,
        var_iri_dn10: f64,
        var_iri_dn11: f64,
        var_iri_dn12: f64,
        var_iri_dn13: f64,
        var_iri_dn2: f64,
        var_iri_dn3: f64,
        var_iri_dn4: f64,
        var_iri_dn5: f64,
        var_iri_dn6: f64,
        var_iri_dn7: f64,
        var_iri_dn8: f64,
        var_iri_dn9: f64,
        var_kbci: f64,
        var_kbci_dn0: f64,
        var_kbci_dn1: f64,
        var_kbci_dn10: f64,
        var_kbci_dn11: f64,
        var_kbci_dn12: f64,
        var_kbci_dn13: f64,
        var_kbci_dn2: f64,
        var_kbci_dn3: f64,
        var_kbci_dn4: f64,
        var_kbci_dn5: f64,
        var_kbci_dn6: f64,
        var_kbci_dn7: f64,
        var_kbci_dn8: f64,
        var_kbci_dn9: f64,
        var_kbcx: f64,
        var_kbcx_dn0: f64,
        var_kbcx_dn1: f64,
        var_kbcx_dn10: f64,
        var_kbcx_dn11: f64,
        var_kbcx_dn12: f64,
        var_kbcx_dn13: f64,
        var_kbcx_dn2: f64,
        var_kbcx_dn3: f64,
        var_kbcx_dn4: f64,
        var_kbcx_dn5: f64,
        var_kbcx_dn6: f64,
        var_kbcx_dn7: f64,
        var_kbcx_dn8: f64,
        var_kbcx_dn9: f64,
        var_mif: f64,
        var_mif_dn0: f64,
        var_mif_dn1: f64,
        var_mif_dn10: f64,
        var_mif_dn11: f64,
        var_mif_dn12: f64,
        var_mif_dn13: f64,
        var_mif_dn2: f64,
        var_mif_dn3: f64,
        var_mif_dn4: f64,
        var_mif_dn5: f64,
        var_mif_dn6: f64,
        var_mif_dn7: f64,
        var_mif_dn8: f64,
        var_mif_dn9: f64,
        var_q1: f64,
        var_q1_dn0: f64,
        var_q1_dn1: f64,
        var_q1_dn10: f64,
        var_q1_dn11: f64,
        var_q1_dn12: f64,
        var_q1_dn13: f64,
        var_q1_dn2: f64,
        var_q1_dn3: f64,
        var_q1_dn4: f64,
        var_q1_dn5: f64,
        var_q1_dn6: f64,
        var_q1_dn7: f64,
        var_q1_dn8: f64,
        var_q1_dn9: f64,
        var_qb: f64,
        var_qb_dn0: f64,
        var_qb_dn1: f64,
        var_qb_dn10: f64,
        var_qb_dn11: f64,
        var_qb_dn12: f64,
        var_qb_dn13: f64,
        var_qb_dn2: f64,
        var_qb_dn3: f64,
        var_qb_dn4: f64,
        var_qb_dn5: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qb_dn9: f64,
        var_qdbc: f64,
        var_qdbc_dn0: f64,
        var_qdbc_dn1: f64,
        var_qdbc_dn10: f64,
        var_qdbc_dn11: f64,
        var_qdbc_dn12: f64,
        var_qdbc_dn13: f64,
        var_qdbc_dn2: f64,
        var_qdbc_dn3: f64,
        var_qdbc_dn4: f64,
        var_qdbc_dn5: f64,
        var_qdbc_dn6: f64,
        var_qdbc_dn7: f64,
        var_qdbc_dn8: f64,
        var_qdbc_dn9: f64,
        var_qdbcp: f64,
        var_qdbcp_dn0: f64,
        var_qdbcp_dn1: f64,
        var_qdbcp_dn10: f64,
        var_qdbcp_dn11: f64,
        var_qdbcp_dn12: f64,
        var_qdbcp_dn13: f64,
        var_qdbcp_dn2: f64,
        var_qdbcp_dn3: f64,
        var_qdbcp_dn4: f64,
        var_qdbcp_dn5: f64,
        var_qdbcp_dn6: f64,
        var_qdbcp_dn7: f64,
        var_qdbcp_dn8: f64,
        var_qdbcp_dn9: f64,
        var_qdbe: f64,
        var_qdbe_dn0: f64,
        var_qdbe_dn1: f64,
        var_qdbe_dn10: f64,
        var_qdbe_dn11: f64,
        var_qdbe_dn12: f64,
        var_qdbe_dn13: f64,
        var_qdbe_dn2: f64,
        var_qdbe_dn3: f64,
        var_qdbe_dn4: f64,
        var_qdbe_dn5: f64,
        var_qdbe_dn6: f64,
        var_qdbe_dn7: f64,
        var_qdbe_dn8: f64,
        var_qdbe_dn9: f64,
        var_qdbep: f64,
        var_qdbep_dn0: f64,
        var_qdbep_dn1: f64,
        var_qdbep_dn10: f64,
        var_qdbep_dn11: f64,
        var_qdbep_dn12: f64,
        var_qdbep_dn13: f64,
        var_qdbep_dn2: f64,
        var_qdbep_dn3: f64,
        var_qdbep_dn4: f64,
        var_qdbep_dn5: f64,
        var_qdbep_dn6: f64,
        var_qdbep_dn7: f64,
        var_qdbep_dn8: f64,
        var_qdbep_dn9: f64,
        var_qdbex: f64,
        var_qdbex_dn0: f64,
        var_qdbex_dn1: f64,
        var_qdbex_dn10: f64,
        var_qdbex_dn11: f64,
        var_qdbex_dn12: f64,
        var_qdbex_dn13: f64,
        var_qdbex_dn2: f64,
        var_qdbex_dn3: f64,
        var_qdbex_dn4: f64,
        var_qdbex_dn5: f64,
        var_qdbex_dn6: f64,
        var_qdbex_dn7: f64,
        var_qdbex_dn8: f64,
        var_qdbex_dn9: f64,
        var_sgif: f64,
        var_sltf: f64,
        var_vbcp: f64,
        var_vbcp_dn0: f64,
        var_vbcp_dn1: f64,
        var_vbcp_dn10: f64,
        var_vbcp_dn11: f64,
        var_vbcp_dn12: f64,
        var_vbcp_dn13: f64,
        var_vbcp_dn2: f64,
        var_vbcp_dn3: f64,
        var_vbcp_dn4: f64,
        var_vbcp_dn5: f64,
        var_vbcp_dn6: f64,
        var_vbcp_dn7: f64,
        var_vbcp_dn8: f64,
        var_vbcp_dn9: f64,
        var_vbictype: f64,
        var_vmaxexp: f64,
        var_expi_slot: &mut f64,
        var_expi_dn0_slot: &mut f64,
        var_expi_dn1_slot: &mut f64,
        var_expi_dn10_slot: &mut f64,
        var_expi_dn11_slot: &mut f64,
        var_expi_dn12_slot: &mut f64,
        var_expi_dn13_slot: &mut f64,
        var_expi_dn2_slot: &mut f64,
        var_expi_dn3_slot: &mut f64,
        var_expi_dn4_slot: &mut f64,
        var_expi_dn5_slot: &mut f64,
        var_expi_dn6_slot: &mut f64,
        var_expi_dn7_slot: &mut f64,
        var_expi_dn8_slot: &mut f64,
        var_expi_dn9_slot: &mut f64,
        var_qbc_slot: &mut f64,
        var_qbc_dn0_slot: &mut f64,
        var_qbc_dn1_slot: &mut f64,
        var_qbc_dn10_slot: &mut f64,
        var_qbc_dn11_slot: &mut f64,
        var_qbc_dn12_slot: &mut f64,
        var_qbc_dn13_slot: &mut f64,
        var_qbc_dn2_slot: &mut f64,
        var_qbc_dn3_slot: &mut f64,
        var_qbc_dn4_slot: &mut f64,
        var_qbc_dn5_slot: &mut f64,
        var_qbc_dn6_slot: &mut f64,
        var_qbc_dn7_slot: &mut f64,
        var_qbc_dn8_slot: &mut f64,
        var_qbc_dn9_slot: &mut f64,
        var_qbcp_slot: &mut f64,
        var_qbcp_dn0_slot: &mut f64,
        var_qbcp_dn1_slot: &mut f64,
        var_qbcp_dn10_slot: &mut f64,
        var_qbcp_dn11_slot: &mut f64,
        var_qbcp_dn12_slot: &mut f64,
        var_qbcp_dn13_slot: &mut f64,
        var_qbcp_dn2_slot: &mut f64,
        var_qbcp_dn3_slot: &mut f64,
        var_qbcp_dn4_slot: &mut f64,
        var_qbcp_dn5_slot: &mut f64,
        var_qbcp_dn6_slot: &mut f64,
        var_qbcp_dn7_slot: &mut f64,
        var_qbcp_dn8_slot: &mut f64,
        var_qbcp_dn9_slot: &mut f64,
        var_qbcx_slot: &mut f64,
        var_qbcx_dn0_slot: &mut f64,
        var_qbcx_dn1_slot: &mut f64,
        var_qbcx_dn10_slot: &mut f64,
        var_qbcx_dn11_slot: &mut f64,
        var_qbcx_dn12_slot: &mut f64,
        var_qbcx_dn13_slot: &mut f64,
        var_qbcx_dn2_slot: &mut f64,
        var_qbcx_dn3_slot: &mut f64,
        var_qbcx_dn4_slot: &mut f64,
        var_qbcx_dn5_slot: &mut f64,
        var_qbcx_dn6_slot: &mut f64,
        var_qbcx_dn7_slot: &mut f64,
        var_qbcx_dn8_slot: &mut f64,
        var_qbcx_dn9_slot: &mut f64,
        var_qbe_slot: &mut f64,
        var_qbe_dn0_slot: &mut f64,
        var_qbe_dn1_slot: &mut f64,
        var_qbe_dn10_slot: &mut f64,
        var_qbe_dn11_slot: &mut f64,
        var_qbe_dn12_slot: &mut f64,
        var_qbe_dn13_slot: &mut f64,
        var_qbe_dn2_slot: &mut f64,
        var_qbe_dn3_slot: &mut f64,
        var_qbe_dn4_slot: &mut f64,
        var_qbe_dn5_slot: &mut f64,
        var_qbe_dn6_slot: &mut f64,
        var_qbe_dn7_slot: &mut f64,
        var_qbe_dn8_slot: &mut f64,
        var_qbe_dn9_slot: &mut f64,
        var_qbep_slot: &mut f64,
        var_qbep_dn0_slot: &mut f64,
        var_qbep_dn1_slot: &mut f64,
        var_qbep_dn10_slot: &mut f64,
        var_qbep_dn11_slot: &mut f64,
        var_qbep_dn12_slot: &mut f64,
        var_qbep_dn13_slot: &mut f64,
        var_qbep_dn2_slot: &mut f64,
        var_qbep_dn3_slot: &mut f64,
        var_qbep_dn4_slot: &mut f64,
        var_qbep_dn5_slot: &mut f64,
        var_qbep_dn6_slot: &mut f64,
        var_qbep_dn7_slot: &mut f64,
        var_qbep_dn8_slot: &mut f64,
        var_qbep_dn9_slot: &mut f64,
        var_qbex_slot: &mut f64,
        var_qbex_dn0_slot: &mut f64,
        var_qbex_dn1_slot: &mut f64,
        var_qbex_dn10_slot: &mut f64,
        var_qbex_dn11_slot: &mut f64,
        var_qbex_dn12_slot: &mut f64,
        var_qbex_dn13_slot: &mut f64,
        var_qbex_dn2_slot: &mut f64,
        var_qbex_dn3_slot: &mut f64,
        var_qbex_dn4_slot: &mut f64,
        var_qbex_dn5_slot: &mut f64,
        var_qbex_dn6_slot: &mut f64,
        var_qbex_dn7_slot: &mut f64,
        var_qbex_dn8_slot: &mut f64,
        var_qbex_dn9_slot: &mut f64,
        var_qcth_slot: &mut f64,
        var_qcth_dn0_slot: &mut f64,
        var_qcth_dn1_slot: &mut f64,
        var_qcth_dn10_slot: &mut f64,
        var_qcth_dn11_slot: &mut f64,
        var_qcth_dn12_slot: &mut f64,
        var_qcth_dn13_slot: &mut f64,
        var_qcth_dn2_slot: &mut f64,
        var_qcth_dn3_slot: &mut f64,
        var_qcth_dn4_slot: &mut f64,
        var_qcth_dn5_slot: &mut f64,
        var_qcth_dn6_slot: &mut f64,
        var_qcth_dn7_slot: &mut f64,
        var_qcth_dn8_slot: &mut f64,
        var_qcth_dn9_slot: &mut f64,
        var_tff_slot: &mut f64,
        var_tff_dn0_slot: &mut f64,
        var_tff_dn1_slot: &mut f64,
        var_tff_dn10_slot: &mut f64,
        var_tff_dn11_slot: &mut f64,
        var_tff_dn12_slot: &mut f64,
        var_tff_dn13_slot: &mut f64,
        var_tff_dn2_slot: &mut f64,
        var_tff_dn3_slot: &mut f64,
        var_tff_dn4_slot: &mut f64,
        var_tff_dn5_slot: &mut f64,
        var_tff_dn6_slot: &mut f64,
        var_tff_dn7_slot: &mut f64,
        var_tff_dn8_slot: &mut f64,
        var_tff_dn9_slot: &mut f64,
    ) {
        let mut var_expi: f64 = *var_expi_slot;
        let mut var_expi_dn0: f64 = *var_expi_dn0_slot;
        let mut var_expi_dn1: f64 = *var_expi_dn1_slot;
        let mut var_expi_dn10: f64 = *var_expi_dn10_slot;
        let mut var_expi_dn11: f64 = *var_expi_dn11_slot;
        let mut var_expi_dn12: f64 = *var_expi_dn12_slot;
        let mut var_expi_dn13: f64 = *var_expi_dn13_slot;
        let mut var_expi_dn2: f64 = *var_expi_dn2_slot;
        let mut var_expi_dn3: f64 = *var_expi_dn3_slot;
        let mut var_expi_dn4: f64 = *var_expi_dn4_slot;
        let mut var_expi_dn5: f64 = *var_expi_dn5_slot;
        let mut var_expi_dn6: f64 = *var_expi_dn6_slot;
        let mut var_expi_dn7: f64 = *var_expi_dn7_slot;
        let mut var_expi_dn8: f64 = *var_expi_dn8_slot;
        let mut var_expi_dn9: f64 = *var_expi_dn9_slot;
        let mut var_qbc: f64 = *var_qbc_slot;
        let mut var_qbc_dn0: f64 = *var_qbc_dn0_slot;
        let mut var_qbc_dn1: f64 = *var_qbc_dn1_slot;
        let mut var_qbc_dn10: f64 = *var_qbc_dn10_slot;
        let mut var_qbc_dn11: f64 = *var_qbc_dn11_slot;
        let mut var_qbc_dn12: f64 = *var_qbc_dn12_slot;
        let mut var_qbc_dn13: f64 = *var_qbc_dn13_slot;
        let mut var_qbc_dn2: f64 = *var_qbc_dn2_slot;
        let mut var_qbc_dn3: f64 = *var_qbc_dn3_slot;
        let mut var_qbc_dn4: f64 = *var_qbc_dn4_slot;
        let mut var_qbc_dn5: f64 = *var_qbc_dn5_slot;
        let mut var_qbc_dn6: f64 = *var_qbc_dn6_slot;
        let mut var_qbc_dn7: f64 = *var_qbc_dn7_slot;
        let mut var_qbc_dn8: f64 = *var_qbc_dn8_slot;
        let mut var_qbc_dn9: f64 = *var_qbc_dn9_slot;
        let mut var_qbcp: f64 = *var_qbcp_slot;
        let mut var_qbcp_dn0: f64 = *var_qbcp_dn0_slot;
        let mut var_qbcp_dn1: f64 = *var_qbcp_dn1_slot;
        let mut var_qbcp_dn10: f64 = *var_qbcp_dn10_slot;
        let mut var_qbcp_dn11: f64 = *var_qbcp_dn11_slot;
        let mut var_qbcp_dn12: f64 = *var_qbcp_dn12_slot;
        let mut var_qbcp_dn13: f64 = *var_qbcp_dn13_slot;
        let mut var_qbcp_dn2: f64 = *var_qbcp_dn2_slot;
        let mut var_qbcp_dn3: f64 = *var_qbcp_dn3_slot;
        let mut var_qbcp_dn4: f64 = *var_qbcp_dn4_slot;
        let mut var_qbcp_dn5: f64 = *var_qbcp_dn5_slot;
        let mut var_qbcp_dn6: f64 = *var_qbcp_dn6_slot;
        let mut var_qbcp_dn7: f64 = *var_qbcp_dn7_slot;
        let mut var_qbcp_dn8: f64 = *var_qbcp_dn8_slot;
        let mut var_qbcp_dn9: f64 = *var_qbcp_dn9_slot;
        let mut var_qbcx: f64 = *var_qbcx_slot;
        let mut var_qbcx_dn0: f64 = *var_qbcx_dn0_slot;
        let mut var_qbcx_dn1: f64 = *var_qbcx_dn1_slot;
        let mut var_qbcx_dn10: f64 = *var_qbcx_dn10_slot;
        let mut var_qbcx_dn11: f64 = *var_qbcx_dn11_slot;
        let mut var_qbcx_dn12: f64 = *var_qbcx_dn12_slot;
        let mut var_qbcx_dn13: f64 = *var_qbcx_dn13_slot;
        let mut var_qbcx_dn2: f64 = *var_qbcx_dn2_slot;
        let mut var_qbcx_dn3: f64 = *var_qbcx_dn3_slot;
        let mut var_qbcx_dn4: f64 = *var_qbcx_dn4_slot;
        let mut var_qbcx_dn5: f64 = *var_qbcx_dn5_slot;
        let mut var_qbcx_dn6: f64 = *var_qbcx_dn6_slot;
        let mut var_qbcx_dn7: f64 = *var_qbcx_dn7_slot;
        let mut var_qbcx_dn8: f64 = *var_qbcx_dn8_slot;
        let mut var_qbcx_dn9: f64 = *var_qbcx_dn9_slot;
        let mut var_qbe: f64 = *var_qbe_slot;
        let mut var_qbe_dn0: f64 = *var_qbe_dn0_slot;
        let mut var_qbe_dn1: f64 = *var_qbe_dn1_slot;
        let mut var_qbe_dn10: f64 = *var_qbe_dn10_slot;
        let mut var_qbe_dn11: f64 = *var_qbe_dn11_slot;
        let mut var_qbe_dn12: f64 = *var_qbe_dn12_slot;
        let mut var_qbe_dn13: f64 = *var_qbe_dn13_slot;
        let mut var_qbe_dn2: f64 = *var_qbe_dn2_slot;
        let mut var_qbe_dn3: f64 = *var_qbe_dn3_slot;
        let mut var_qbe_dn4: f64 = *var_qbe_dn4_slot;
        let mut var_qbe_dn5: f64 = *var_qbe_dn5_slot;
        let mut var_qbe_dn6: f64 = *var_qbe_dn6_slot;
        let mut var_qbe_dn7: f64 = *var_qbe_dn7_slot;
        let mut var_qbe_dn8: f64 = *var_qbe_dn8_slot;
        let mut var_qbe_dn9: f64 = *var_qbe_dn9_slot;
        let mut var_qbep: f64 = *var_qbep_slot;
        let mut var_qbep_dn0: f64 = *var_qbep_dn0_slot;
        let mut var_qbep_dn1: f64 = *var_qbep_dn1_slot;
        let mut var_qbep_dn10: f64 = *var_qbep_dn10_slot;
        let mut var_qbep_dn11: f64 = *var_qbep_dn11_slot;
        let mut var_qbep_dn12: f64 = *var_qbep_dn12_slot;
        let mut var_qbep_dn13: f64 = *var_qbep_dn13_slot;
        let mut var_qbep_dn2: f64 = *var_qbep_dn2_slot;
        let mut var_qbep_dn3: f64 = *var_qbep_dn3_slot;
        let mut var_qbep_dn4: f64 = *var_qbep_dn4_slot;
        let mut var_qbep_dn5: f64 = *var_qbep_dn5_slot;
        let mut var_qbep_dn6: f64 = *var_qbep_dn6_slot;
        let mut var_qbep_dn7: f64 = *var_qbep_dn7_slot;
        let mut var_qbep_dn8: f64 = *var_qbep_dn8_slot;
        let mut var_qbep_dn9: f64 = *var_qbep_dn9_slot;
        let mut var_qbex: f64 = *var_qbex_slot;
        let mut var_qbex_dn0: f64 = *var_qbex_dn0_slot;
        let mut var_qbex_dn1: f64 = *var_qbex_dn1_slot;
        let mut var_qbex_dn10: f64 = *var_qbex_dn10_slot;
        let mut var_qbex_dn11: f64 = *var_qbex_dn11_slot;
        let mut var_qbex_dn12: f64 = *var_qbex_dn12_slot;
        let mut var_qbex_dn13: f64 = *var_qbex_dn13_slot;
        let mut var_qbex_dn2: f64 = *var_qbex_dn2_slot;
        let mut var_qbex_dn3: f64 = *var_qbex_dn3_slot;
        let mut var_qbex_dn4: f64 = *var_qbex_dn4_slot;
        let mut var_qbex_dn5: f64 = *var_qbex_dn5_slot;
        let mut var_qbex_dn6: f64 = *var_qbex_dn6_slot;
        let mut var_qbex_dn7: f64 = *var_qbex_dn7_slot;
        let mut var_qbex_dn8: f64 = *var_qbex_dn8_slot;
        let mut var_qbex_dn9: f64 = *var_qbex_dn9_slot;
        let mut var_qcth: f64 = *var_qcth_slot;
        let mut var_qcth_dn0: f64 = *var_qcth_dn0_slot;
        let mut var_qcth_dn1: f64 = *var_qcth_dn1_slot;
        let mut var_qcth_dn10: f64 = *var_qcth_dn10_slot;
        let mut var_qcth_dn11: f64 = *var_qcth_dn11_slot;
        let mut var_qcth_dn12: f64 = *var_qcth_dn12_slot;
        let mut var_qcth_dn13: f64 = *var_qcth_dn13_slot;
        let mut var_qcth_dn2: f64 = *var_qcth_dn2_slot;
        let mut var_qcth_dn3: f64 = *var_qcth_dn3_slot;
        let mut var_qcth_dn4: f64 = *var_qcth_dn4_slot;
        let mut var_qcth_dn5: f64 = *var_qcth_dn5_slot;
        let mut var_qcth_dn6: f64 = *var_qcth_dn6_slot;
        let mut var_qcth_dn7: f64 = *var_qcth_dn7_slot;
        let mut var_qcth_dn8: f64 = *var_qcth_dn8_slot;
        let mut var_qcth_dn9: f64 = *var_qcth_dn9_slot;
        let mut var_tff: f64 = *var_tff_slot;
        let mut var_tff_dn0: f64 = *var_tff_dn0_slot;
        let mut var_tff_dn1: f64 = *var_tff_dn1_slot;
        let mut var_tff_dn10: f64 = *var_tff_dn10_slot;
        let mut var_tff_dn11: f64 = *var_tff_dn11_slot;
        let mut var_tff_dn12: f64 = *var_tff_dn12_slot;
        let mut var_tff_dn13: f64 = *var_tff_dn13_slot;
        let mut var_tff_dn2: f64 = *var_tff_dn2_slot;
        let mut var_tff_dn3: f64 = *var_tff_dn3_slot;
        let mut var_tff_dn4: f64 = *var_tff_dn4_slot;
        let mut var_tff_dn5: f64 = *var_tff_dn5_slot;
        let mut var_tff_dn6: f64 = *var_tff_dn6_slot;
        let mut var_tff_dn7: f64 = *var_tff_dn7_slot;
        let mut var_tff_dn8: f64 = *var_tff_dn8_slot;
        let mut var_tff_dn9: f64 = *var_tff_dn9_slot;

        let (assign5530_e6133, assign5530_e6133_d_n0, assign5530_e6133_d_n1, assign5530_e6133_d_n2, assign5530_e6133_d_n3, assign5530_e6133_d_n4, assign5530_e6133_d_n5, assign5530_e6133_d_n6, assign5530_e6133_d_n7, assign5530_e6133_d_n8, assign5530_e6133_d_n9, assign5530_e6133_d_n10, assign5530_e6133_d_n11, assign5530_e6133_d_n12, assign5530_e6133_d_n13,) = {
    if (var_guard187 != 0.0) {
        let assign5530_e6131: f64 = (var_arg).exp();
        (assign5530_e6131, (assign5530_e6131 * var_arg_dn0), (assign5530_e6131 * var_arg_dn1), (assign5530_e6131 * var_arg_dn2), (assign5530_e6131 * var_arg_dn3), (assign5530_e6131 * var_arg_dn4), (assign5530_e6131 * var_arg_dn5), (assign5530_e6131 * var_arg_dn6), (assign5530_e6131 * var_arg_dn7), (assign5530_e6131 * var_arg_dn8), (assign5530_e6131 * var_arg_dn9), (assign5530_e6131 * var_arg_dn10), (assign5530_e6131 * var_arg_dn11), (assign5530_e6131 * var_arg_dn12), (assign5530_e6131 * var_arg_dn13),)
    } else {
        (var_expi, var_expi_dn0, var_expi_dn1, var_expi_dn2, var_expi_dn3, var_expi_dn4, var_expi_dn5, var_expi_dn6, var_expi_dn7, var_expi_dn8, var_expi_dn9, var_expi_dn10, var_expi_dn11, var_expi_dn12, var_expi_dn13,)
    }
};
        var_expi = assign5530_e6133;
        var_expi_dn0 = assign5530_e6133_d_n0;
        var_expi_dn1 = assign5530_e6133_d_n1;
        var_expi_dn2 = assign5530_e6133_d_n2;
        var_expi_dn3 = assign5530_e6133_d_n3;
        var_expi_dn4 = assign5530_e6133_d_n4;
        var_expi_dn5 = assign5530_e6133_d_n5;
        var_expi_dn6 = assign5530_e6133_d_n6;
        var_expi_dn7 = assign5530_e6133_d_n7;
        var_expi_dn8 = assign5530_e6133_d_n8;
        var_expi_dn9 = assign5530_e6133_d_n9;
        var_expi_dn10 = assign5530_e6133_d_n10;
        var_expi_dn11 = assign5530_e6133_d_n11;
        var_expi_dn12 = assign5530_e6133_d_n12;
        var_expi_dn13 = assign5530_e6133_d_n13;

        let (assign5540_e6145, assign5540_e6145_d_n0, assign5540_e6145_d_n1, assign5540_e6145_d_n2, assign5540_e6145_d_n3, assign5540_e6145_d_n4, assign5540_e6145_d_n5, assign5540_e6145_d_n6, assign5540_e6145_d_n7, assign5540_e6145_d_n8, assign5540_e6145_d_n9, assign5540_e6145_d_n10, assign5540_e6145_d_n11, assign5540_e6145_d_n12, assign5540_e6145_d_n13,) = {
    if (var_guard187 == 0.0) {
        let assign5540_e6137: f64 = (var_vmaxexp).exp();
        let assign5540_e6141: f64 = (var_arg - var_vmaxexp);
        let assign5540_e6142: f64 = (1.0 + assign5540_e6141);
        let assign5540_e6143: f64 = (assign5540_e6137 * assign5540_e6142);
        (assign5540_e6143, (assign5540_e6137 * var_arg_dn0), (assign5540_e6137 * var_arg_dn1), (assign5540_e6137 * var_arg_dn2), (assign5540_e6137 * var_arg_dn3), (assign5540_e6137 * var_arg_dn4), (assign5540_e6137 * var_arg_dn5), (assign5540_e6137 * var_arg_dn6), (assign5540_e6137 * var_arg_dn7), (assign5540_e6137 * var_arg_dn8), (assign5540_e6137 * var_arg_dn9), (assign5540_e6137 * var_arg_dn10), (assign5540_e6137 * var_arg_dn11), (assign5540_e6137 * var_arg_dn12), (assign5540_e6137 * var_arg_dn13),)
    } else {
        (var_expi, var_expi_dn0, var_expi_dn1, var_expi_dn2, var_expi_dn3, var_expi_dn4, var_expi_dn5, var_expi_dn6, var_expi_dn7, var_expi_dn8, var_expi_dn9, var_expi_dn10, var_expi_dn11, var_expi_dn12, var_expi_dn13,)
    }
};
        var_expi = assign5540_e6145;
        var_expi_dn0 = assign5540_e6145_d_n0;
        var_expi_dn1 = assign5540_e6145_d_n1;
        var_expi_dn2 = assign5540_e6145_d_n2;
        var_expi_dn3 = assign5540_e6145_d_n3;
        var_expi_dn4 = assign5540_e6145_d_n4;
        var_expi_dn5 = assign5540_e6145_d_n5;
        var_expi_dn6 = assign5540_e6145_d_n6;
        var_expi_dn7 = assign5540_e6145_d_n7;
        var_expi_dn8 = assign5540_e6145_d_n8;
        var_expi_dn9 = assign5540_e6145_d_n9;
        var_expi_dn10 = assign5540_e6145_d_n10;
        var_expi_dn11 = assign5540_e6145_d_n11;
        var_expi_dn12 = assign5540_e6145_d_n12;
        var_expi_dn13 = assign5540_e6145_d_n13;

        let assign5550_e6150: f64 = (p.p77 * var_q1);
        let assign5550_e6151: f64 = (1.0 + assign5550_e6150);
        let assign5550_e6152: f64 = (p.p76 * assign5550_e6151);
        let assign5550_e6156: f64 = (p.p78 * var_expi);
        let assign5550_e6160: f64 = (var_mif * var_mif);
        let assign5550_e6161: f64 = (var_sltf + assign5550_e6160);
        let assign5550_e6162: f64 = (assign5550_e6156 * assign5550_e6161);
        let assign5550_e6164: f64 = (assign5550_e6162 * var_sgif);
        let assign5550_e6165: f64 = (1.0 + assign5550_e6164);
        let assign5550_e6166: f64 = (assign5550_e6152 * assign5550_e6165);
        var_tff = assign5550_e6166;
        var_tff_dn0 = (((p.p76 * (p.p77 * var_q1_dn0)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * var_expi_dn0) * assign5550_e6161) + (assign5550_e6156 * ((var_mif_dn0 * var_mif) + (var_mif * var_mif_dn0)))) * var_sgif)));
        var_tff_dn1 = (((p.p76 * (p.p77 * var_q1_dn1)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * var_expi_dn1) * assign5550_e6161) + (assign5550_e6156 * ((var_mif_dn1 * var_mif) + (var_mif * var_mif_dn1)))) * var_sgif)));
        var_tff_dn2 = (((p.p76 * (p.p77 * var_q1_dn2)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * var_expi_dn2) * assign5550_e6161) + (assign5550_e6156 * ((var_mif_dn2 * var_mif) + (var_mif * var_mif_dn2)))) * var_sgif)));
        var_tff_dn3 = (((p.p76 * (p.p77 * var_q1_dn3)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * var_expi_dn3) * assign5550_e6161) + (assign5550_e6156 * ((var_mif_dn3 * var_mif) + (var_mif * var_mif_dn3)))) * var_sgif)));
        var_tff_dn4 = (((p.p76 * (p.p77 * var_q1_dn4)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * var_expi_dn4) * assign5550_e6161) + (assign5550_e6156 * ((var_mif_dn4 * var_mif) + (var_mif * var_mif_dn4)))) * var_sgif)));
        var_tff_dn5 = (((p.p76 * (p.p77 * var_q1_dn5)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * var_expi_dn5) * assign5550_e6161) + (assign5550_e6156 * ((var_mif_dn5 * var_mif) + (var_mif * var_mif_dn5)))) * var_sgif)));
        var_tff_dn6 = (((p.p76 * (p.p77 * var_q1_dn6)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * var_expi_dn6) * assign5550_e6161) + (assign5550_e6156 * ((var_mif_dn6 * var_mif) + (var_mif * var_mif_dn6)))) * var_sgif)));
        var_tff_dn7 = (((p.p76 * (p.p77 * var_q1_dn7)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * var_expi_dn7) * assign5550_e6161) + (assign5550_e6156 * ((var_mif_dn7 * var_mif) + (var_mif * var_mif_dn7)))) * var_sgif)));
        var_tff_dn8 = (((p.p76 * (p.p77 * var_q1_dn8)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * var_expi_dn8) * assign5550_e6161) + (assign5550_e6156 * ((var_mif_dn8 * var_mif) + (var_mif * var_mif_dn8)))) * var_sgif)));
        var_tff_dn9 = (((p.p76 * (p.p77 * var_q1_dn9)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * var_expi_dn9) * assign5550_e6161) + (assign5550_e6156 * ((var_mif_dn9 * var_mif) + (var_mif * var_mif_dn9)))) * var_sgif)));
        var_tff_dn10 = (((p.p76 * (p.p77 * var_q1_dn10)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * var_expi_dn10) * assign5550_e6161) + (assign5550_e6156 * ((var_mif_dn10 * var_mif) + (var_mif * var_mif_dn10)))) * var_sgif)));
        var_tff_dn11 = (((p.p76 * (p.p77 * var_q1_dn11)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * var_expi_dn11) * assign5550_e6161) + (assign5550_e6156 * ((var_mif_dn11 * var_mif) + (var_mif * var_mif_dn11)))) * var_sgif)));
        var_tff_dn12 = (((p.p76 * (p.p77 * var_q1_dn12)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * var_expi_dn12) * assign5550_e6161) + (assign5550_e6156 * ((var_mif_dn12 * var_mif) + (var_mif * var_mif_dn12)))) * var_sgif)));
        var_tff_dn13 = (((p.p76 * (p.p77 * var_q1_dn13)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * var_expi_dn13) * assign5550_e6161) + (assign5550_e6156 * ((var_mif_dn13 * var_mif) + (var_mif * var_mif_dn13)))) * var_sgif)));

        let assign5560_e6169: f64 = (var_cje_t * var_qdbe);
        let assign5560_e6171: f64 = (assign5560_e6169 * p.p55);
        let assign5560_e6174: f64 = (var_tff * var_ifi);
        let assign5560_e6176: f64 = (assign5560_e6174 / var_qb);
        let assign5560_e6177: f64 = (assign5560_e6171 + assign5560_e6176);
        var_qbe = assign5560_e6177;
        var_qbe_dn0 = ((((var_cje_t_dn0 * var_qdbe) + (var_cje_t * var_qdbe_dn0)) * p.p55) + (((((var_tff_dn0 * var_ifi) + (var_tff * var_ifi_dn0)) * var_qb) - (assign5560_e6174 * var_qb_dn0)) / (var_qb * var_qb)));
        var_qbe_dn1 = ((((var_cje_t_dn1 * var_qdbe) + (var_cje_t * var_qdbe_dn1)) * p.p55) + (((((var_tff_dn1 * var_ifi) + (var_tff * var_ifi_dn1)) * var_qb) - (assign5560_e6174 * var_qb_dn1)) / (var_qb * var_qb)));
        var_qbe_dn2 = ((((var_cje_t_dn2 * var_qdbe) + (var_cje_t * var_qdbe_dn2)) * p.p55) + (((((var_tff_dn2 * var_ifi) + (var_tff * var_ifi_dn2)) * var_qb) - (assign5560_e6174 * var_qb_dn2)) / (var_qb * var_qb)));
        var_qbe_dn3 = ((((var_cje_t_dn3 * var_qdbe) + (var_cje_t * var_qdbe_dn3)) * p.p55) + (((((var_tff_dn3 * var_ifi) + (var_tff * var_ifi_dn3)) * var_qb) - (assign5560_e6174 * var_qb_dn3)) / (var_qb * var_qb)));
        var_qbe_dn4 = ((((var_cje_t_dn4 * var_qdbe) + (var_cje_t * var_qdbe_dn4)) * p.p55) + (((((var_tff_dn4 * var_ifi) + (var_tff * var_ifi_dn4)) * var_qb) - (assign5560_e6174 * var_qb_dn4)) / (var_qb * var_qb)));
        var_qbe_dn5 = ((((var_cje_t_dn5 * var_qdbe) + (var_cje_t * var_qdbe_dn5)) * p.p55) + (((((var_tff_dn5 * var_ifi) + (var_tff * var_ifi_dn5)) * var_qb) - (assign5560_e6174 * var_qb_dn5)) / (var_qb * var_qb)));
        var_qbe_dn6 = ((((var_cje_t_dn6 * var_qdbe) + (var_cje_t * var_qdbe_dn6)) * p.p55) + (((((var_tff_dn6 * var_ifi) + (var_tff * var_ifi_dn6)) * var_qb) - (assign5560_e6174 * var_qb_dn6)) / (var_qb * var_qb)));
        var_qbe_dn7 = ((((var_cje_t_dn7 * var_qdbe) + (var_cje_t * var_qdbe_dn7)) * p.p55) + (((((var_tff_dn7 * var_ifi) + (var_tff * var_ifi_dn7)) * var_qb) - (assign5560_e6174 * var_qb_dn7)) / (var_qb * var_qb)));
        var_qbe_dn8 = ((((var_cje_t_dn8 * var_qdbe) + (var_cje_t * var_qdbe_dn8)) * p.p55) + (((((var_tff_dn8 * var_ifi) + (var_tff * var_ifi_dn8)) * var_qb) - (assign5560_e6174 * var_qb_dn8)) / (var_qb * var_qb)));
        var_qbe_dn9 = ((((var_cje_t_dn9 * var_qdbe) + (var_cje_t * var_qdbe_dn9)) * p.p55) + (((((var_tff_dn9 * var_ifi) + (var_tff * var_ifi_dn9)) * var_qb) - (assign5560_e6174 * var_qb_dn9)) / (var_qb * var_qb)));
        var_qbe_dn10 = ((((var_cje_t_dn10 * var_qdbe) + (var_cje_t * var_qdbe_dn10)) * p.p55) + (((((var_tff_dn10 * var_ifi) + (var_tff * var_ifi_dn10)) * var_qb) - (assign5560_e6174 * var_qb_dn10)) / (var_qb * var_qb)));
        var_qbe_dn11 = ((((var_cje_t_dn11 * var_qdbe) + (var_cje_t * var_qdbe_dn11)) * p.p55) + (((((var_tff_dn11 * var_ifi) + (var_tff * var_ifi_dn11)) * var_qb) - (assign5560_e6174 * var_qb_dn11)) / (var_qb * var_qb)));
        var_qbe_dn12 = ((((var_cje_t_dn12 * var_qdbe) + (var_cje_t * var_qdbe_dn12)) * p.p55) + (((((var_tff_dn12 * var_ifi) + (var_tff * var_ifi_dn12)) * var_qb) - (assign5560_e6174 * var_qb_dn12)) / (var_qb * var_qb)));
        var_qbe_dn13 = ((((var_cje_t_dn13 * var_qdbe) + (var_cje_t * var_qdbe_dn13)) * p.p55) + (((((var_tff_dn13 * var_ifi) + (var_tff * var_ifi_dn13)) * var_qb) - (assign5560_e6174 * var_qb_dn13)) / (var_qb * var_qb)));

        let assign5570_e6180: f64 = (var_cje_t * var_qdbex);
        let assign5570_e6183: f64 = (1.0 - p.p55);
        let assign5570_e6184: f64 = (assign5570_e6180 * assign5570_e6183);
        var_qbex = assign5570_e6184;
        var_qbex_dn0 = (((var_cje_t_dn0 * var_qdbex) + (var_cje_t * var_qdbex_dn0)) * assign5570_e6183);
        var_qbex_dn1 = (((var_cje_t_dn1 * var_qdbex) + (var_cje_t * var_qdbex_dn1)) * assign5570_e6183);
        var_qbex_dn2 = (((var_cje_t_dn2 * var_qdbex) + (var_cje_t * var_qdbex_dn2)) * assign5570_e6183);
        var_qbex_dn3 = (((var_cje_t_dn3 * var_qdbex) + (var_cje_t * var_qdbex_dn3)) * assign5570_e6183);
        var_qbex_dn4 = (((var_cje_t_dn4 * var_qdbex) + (var_cje_t * var_qdbex_dn4)) * assign5570_e6183);
        var_qbex_dn5 = (((var_cje_t_dn5 * var_qdbex) + (var_cje_t * var_qdbex_dn5)) * assign5570_e6183);
        var_qbex_dn6 = (((var_cje_t_dn6 * var_qdbex) + (var_cje_t * var_qdbex_dn6)) * assign5570_e6183);
        var_qbex_dn7 = (((var_cje_t_dn7 * var_qdbex) + (var_cje_t * var_qdbex_dn7)) * assign5570_e6183);
        var_qbex_dn8 = (((var_cje_t_dn8 * var_qdbex) + (var_cje_t * var_qdbex_dn8)) * assign5570_e6183);
        var_qbex_dn9 = (((var_cje_t_dn9 * var_qdbex) + (var_cje_t * var_qdbex_dn9)) * assign5570_e6183);
        var_qbex_dn10 = (((var_cje_t_dn10 * var_qdbex) + (var_cje_t * var_qdbex_dn10)) * assign5570_e6183);
        var_qbex_dn11 = (((var_cje_t_dn11 * var_qdbex) + (var_cje_t * var_qdbex_dn11)) * assign5570_e6183);
        var_qbex_dn12 = (((var_cje_t_dn12 * var_qdbex) + (var_cje_t * var_qdbex_dn12)) * assign5570_e6183);
        var_qbex_dn13 = (((var_cje_t_dn13 * var_qdbex) + (var_cje_t * var_qdbex_dn13)) * assign5570_e6183);

        let assign5580_e6187: f64 = (var_cjc_t * var_qdbc);
        let assign5580_e6190: f64 = (p.p81 * var_iri);
        let assign5580_e6191: f64 = (assign5580_e6187 + assign5580_e6190);
        let assign5580_e6194: f64 = (p.p47 * var_kbci);
        let assign5580_e6195: f64 = (assign5580_e6191 + assign5580_e6194);
        var_qbc = assign5580_e6195;
        var_qbc_dn0 = ((((var_cjc_t_dn0 * var_qdbc) + (var_cjc_t * var_qdbc_dn0)) + (p.p81 * var_iri_dn0)) + (p.p47 * var_kbci_dn0));
        var_qbc_dn1 = ((((var_cjc_t_dn1 * var_qdbc) + (var_cjc_t * var_qdbc_dn1)) + (p.p81 * var_iri_dn1)) + (p.p47 * var_kbci_dn1));
        var_qbc_dn2 = ((((var_cjc_t_dn2 * var_qdbc) + (var_cjc_t * var_qdbc_dn2)) + (p.p81 * var_iri_dn2)) + (p.p47 * var_kbci_dn2));
        var_qbc_dn3 = ((((var_cjc_t_dn3 * var_qdbc) + (var_cjc_t * var_qdbc_dn3)) + (p.p81 * var_iri_dn3)) + (p.p47 * var_kbci_dn3));
        var_qbc_dn4 = ((((var_cjc_t_dn4 * var_qdbc) + (var_cjc_t * var_qdbc_dn4)) + (p.p81 * var_iri_dn4)) + (p.p47 * var_kbci_dn4));
        var_qbc_dn5 = ((((var_cjc_t_dn5 * var_qdbc) + (var_cjc_t * var_qdbc_dn5)) + (p.p81 * var_iri_dn5)) + (p.p47 * var_kbci_dn5));
        var_qbc_dn6 = ((((var_cjc_t_dn6 * var_qdbc) + (var_cjc_t * var_qdbc_dn6)) + (p.p81 * var_iri_dn6)) + (p.p47 * var_kbci_dn6));
        var_qbc_dn7 = ((((var_cjc_t_dn7 * var_qdbc) + (var_cjc_t * var_qdbc_dn7)) + (p.p81 * var_iri_dn7)) + (p.p47 * var_kbci_dn7));
        var_qbc_dn8 = ((((var_cjc_t_dn8 * var_qdbc) + (var_cjc_t * var_qdbc_dn8)) + (p.p81 * var_iri_dn8)) + (p.p47 * var_kbci_dn8));
        var_qbc_dn9 = ((((var_cjc_t_dn9 * var_qdbc) + (var_cjc_t * var_qdbc_dn9)) + (p.p81 * var_iri_dn9)) + (p.p47 * var_kbci_dn9));
        var_qbc_dn10 = ((((var_cjc_t_dn10 * var_qdbc) + (var_cjc_t * var_qdbc_dn10)) + (p.p81 * var_iri_dn10)) + (p.p47 * var_kbci_dn10));
        var_qbc_dn11 = ((((var_cjc_t_dn11 * var_qdbc) + (var_cjc_t * var_qdbc_dn11)) + (p.p81 * var_iri_dn11)) + (p.p47 * var_kbci_dn11));
        var_qbc_dn12 = ((((var_cjc_t_dn12 * var_qdbc) + (var_cjc_t * var_qdbc_dn12)) + (p.p81 * var_iri_dn12)) + (p.p47 * var_kbci_dn12));
        var_qbc_dn13 = ((((var_cjc_t_dn13 * var_qdbc) + (var_cjc_t * var_qdbc_dn13)) + (p.p81 * var_iri_dn13)) + (p.p47 * var_kbci_dn13));

        let assign5590_e6198: f64 = (p.p47 * var_kbcx);
        var_qbcx = assign5590_e6198;
        var_qbcx_dn0 = (p.p47 * var_kbcx_dn0);
        var_qbcx_dn1 = (p.p47 * var_kbcx_dn1);
        var_qbcx_dn2 = (p.p47 * var_kbcx_dn2);
        var_qbcx_dn3 = (p.p47 * var_kbcx_dn3);
        var_qbcx_dn4 = (p.p47 * var_kbcx_dn4);
        var_qbcx_dn5 = (p.p47 * var_kbcx_dn5);
        var_qbcx_dn6 = (p.p47 * var_kbcx_dn6);
        var_qbcx_dn7 = (p.p47 * var_kbcx_dn7);
        var_qbcx_dn8 = (p.p47 * var_kbcx_dn8);
        var_qbcx_dn9 = (p.p47 * var_kbcx_dn9);
        var_qbcx_dn10 = (p.p47 * var_kbcx_dn10);
        var_qbcx_dn11 = (p.p47 * var_kbcx_dn11);
        var_qbcx_dn12 = (p.p47 * var_kbcx_dn12);
        var_qbcx_dn13 = (p.p47 * var_kbcx_dn13);

        let assign5600_e6201: f64 = (var_cjep_t * var_qdbep);
        let assign5600_e6204: f64 = (p.p81 * var_ifp);
        let assign5600_e6205: f64 = (assign5600_e6201 + assign5600_e6204);
        var_qbep = assign5600_e6205;
        var_qbep_dn0 = (((var_cjep_t_dn0 * var_qdbep) + (var_cjep_t * var_qdbep_dn0)) + (p.p81 * var_ifp_dn0));
        var_qbep_dn1 = (((var_cjep_t_dn1 * var_qdbep) + (var_cjep_t * var_qdbep_dn1)) + (p.p81 * var_ifp_dn1));
        var_qbep_dn2 = (((var_cjep_t_dn2 * var_qdbep) + (var_cjep_t * var_qdbep_dn2)) + (p.p81 * var_ifp_dn2));
        var_qbep_dn3 = (((var_cjep_t_dn3 * var_qdbep) + (var_cjep_t * var_qdbep_dn3)) + (p.p81 * var_ifp_dn3));
        var_qbep_dn4 = (((var_cjep_t_dn4 * var_qdbep) + (var_cjep_t * var_qdbep_dn4)) + (p.p81 * var_ifp_dn4));
        var_qbep_dn5 = (((var_cjep_t_dn5 * var_qdbep) + (var_cjep_t * var_qdbep_dn5)) + (p.p81 * var_ifp_dn5));
        var_qbep_dn6 = (((var_cjep_t_dn6 * var_qdbep) + (var_cjep_t * var_qdbep_dn6)) + (p.p81 * var_ifp_dn6));
        var_qbep_dn7 = (((var_cjep_t_dn7 * var_qdbep) + (var_cjep_t * var_qdbep_dn7)) + (p.p81 * var_ifp_dn7));
        var_qbep_dn8 = (((var_cjep_t_dn8 * var_qdbep) + (var_cjep_t * var_qdbep_dn8)) + (p.p81 * var_ifp_dn8));
        var_qbep_dn9 = (((var_cjep_t_dn9 * var_qdbep) + (var_cjep_t * var_qdbep_dn9)) + (p.p81 * var_ifp_dn9));
        var_qbep_dn10 = (((var_cjep_t_dn10 * var_qdbep) + (var_cjep_t * var_qdbep_dn10)) + (p.p81 * var_ifp_dn10));
        var_qbep_dn11 = (((var_cjep_t_dn11 * var_qdbep) + (var_cjep_t * var_qdbep_dn11)) + (p.p81 * var_ifp_dn11));
        var_qbep_dn12 = (((var_cjep_t_dn12 * var_qdbep) + (var_cjep_t * var_qdbep_dn12)) + (p.p81 * var_ifp_dn12));
        var_qbep_dn13 = (((var_cjep_t_dn13 * var_qdbep) + (var_cjep_t * var_qdbep_dn13)) + (p.p81 * var_ifp_dn13));

        let assign5610_e6208: f64 = (var_cjcp_t * var_qdbcp);
        let assign5610_e6211: f64 = (p.p53 * var_vbcp);
        let assign5610_e6212: f64 = (assign5610_e6208 + assign5610_e6211);
        var_qbcp = assign5610_e6212;
        var_qbcp_dn0 = (((var_cjcp_t_dn0 * var_qdbcp) + (var_cjcp_t * var_qdbcp_dn0)) + (p.p53 * var_vbcp_dn0));
        var_qbcp_dn1 = (((var_cjcp_t_dn1 * var_qdbcp) + (var_cjcp_t * var_qdbcp_dn1)) + (p.p53 * var_vbcp_dn1));
        var_qbcp_dn2 = (((var_cjcp_t_dn2 * var_qdbcp) + (var_cjcp_t * var_qdbcp_dn2)) + (p.p53 * var_vbcp_dn2));
        var_qbcp_dn3 = (((var_cjcp_t_dn3 * var_qdbcp) + (var_cjcp_t * var_qdbcp_dn3)) + (p.p53 * var_vbcp_dn3));
        var_qbcp_dn4 = (((var_cjcp_t_dn4 * var_qdbcp) + (var_cjcp_t * var_qdbcp_dn4)) + (p.p53 * var_vbcp_dn4));
        var_qbcp_dn5 = (((var_cjcp_t_dn5 * var_qdbcp) + (var_cjcp_t * var_qdbcp_dn5)) + (p.p53 * var_vbcp_dn5));
        var_qbcp_dn6 = (((var_cjcp_t_dn6 * var_qdbcp) + (var_cjcp_t * var_qdbcp_dn6)) + (p.p53 * var_vbcp_dn6));
        var_qbcp_dn7 = (((var_cjcp_t_dn7 * var_qdbcp) + (var_cjcp_t * var_qdbcp_dn7)) + (p.p53 * var_vbcp_dn7));
        var_qbcp_dn8 = (((var_cjcp_t_dn8 * var_qdbcp) + (var_cjcp_t * var_qdbcp_dn8)) + (p.p53 * var_vbcp_dn8));
        var_qbcp_dn9 = (((var_cjcp_t_dn9 * var_qdbcp) + (var_cjcp_t * var_qdbcp_dn9)) + (p.p53 * var_vbcp_dn9));
        var_qbcp_dn10 = (((var_cjcp_t_dn10 * var_qdbcp) + (var_cjcp_t * var_qdbcp_dn10)) + (p.p53 * var_vbcp_dn10));
        var_qbcp_dn11 = (((var_cjcp_t_dn11 * var_qdbcp) + (var_cjcp_t * var_qdbcp_dn11)) + (p.p53 * var_vbcp_dn11));
        var_qbcp_dn12 = (((var_cjcp_t_dn12 * var_qdbcp) + (var_cjcp_t * var_qdbcp_dn12)) + (p.p53 * var_vbcp_dn12));
        var_qbcp_dn13 = (((var_cjcp_t_dn13 * var_qdbcp) + (var_cjcp_t * var_qdbcp_dn13)) + (p.p53 * var_vbcp_dn13));

        let assign5640_e6221: f64 = (var_dt_et * p.p102);
        var_qcth = assign5640_e6221;
        var_qcth_dn0 = (var_dt_et_dn0 * p.p102);
        var_qcth_dn1 = (var_dt_et_dn1 * p.p102);
        var_qcth_dn2 = (var_dt_et_dn2 * p.p102);
        var_qcth_dn3 = (var_dt_et_dn3 * p.p102);
        var_qcth_dn4 = (var_dt_et_dn4 * p.p102);
        var_qcth_dn5 = (var_dt_et_dn5 * p.p102);
        var_qcth_dn6 = (var_dt_et_dn6 * p.p102);
        var_qcth_dn7 = (var_dt_et_dn7 * p.p102);
        var_qcth_dn8 = (var_dt_et_dn8 * p.p102);
        var_qcth_dn9 = (var_dt_et_dn9 * p.p102);
        var_qcth_dn10 = (var_dt_et_dn10 * p.p102);
        var_qcth_dn11 = (var_dt_et_dn11 * p.p102);
        var_qcth_dn12 = (var_dt_et_dn12 * p.p102);
        var_qcth_dn13 = (var_dt_et_dn13 * p.p102);

        let assign5670_e6232: f64 = var_vbictype;
        let assign5670_e6234: f64 = (assign5670_e6232 * var_qbe);
        var_qbe = assign5670_e6234;
        var_qbe_dn0 = (assign5670_e6232 * var_qbe_dn0);
        var_qbe_dn1 = (assign5670_e6232 * var_qbe_dn1);
        var_qbe_dn2 = (assign5670_e6232 * var_qbe_dn2);
        var_qbe_dn3 = (assign5670_e6232 * var_qbe_dn3);
        var_qbe_dn4 = (assign5670_e6232 * var_qbe_dn4);
        var_qbe_dn5 = (assign5670_e6232 * var_qbe_dn5);
        var_qbe_dn6 = (assign5670_e6232 * var_qbe_dn6);
        var_qbe_dn7 = (assign5670_e6232 * var_qbe_dn7);
        var_qbe_dn8 = (assign5670_e6232 * var_qbe_dn8);
        var_qbe_dn9 = (assign5670_e6232 * var_qbe_dn9);
        var_qbe_dn10 = (assign5670_e6232 * var_qbe_dn10);
        var_qbe_dn11 = (assign5670_e6232 * var_qbe_dn11);
        var_qbe_dn12 = (assign5670_e6232 * var_qbe_dn12);
        var_qbe_dn13 = (assign5670_e6232 * var_qbe_dn13);

        let assign5680_e6237: f64 = var_vbictype;
        let assign5680_e6239: f64 = (assign5680_e6237 * var_qbex);
        var_qbex = assign5680_e6239;
        var_qbex_dn0 = (assign5680_e6237 * var_qbex_dn0);
        var_qbex_dn1 = (assign5680_e6237 * var_qbex_dn1);
        var_qbex_dn2 = (assign5680_e6237 * var_qbex_dn2);
        var_qbex_dn3 = (assign5680_e6237 * var_qbex_dn3);
        var_qbex_dn4 = (assign5680_e6237 * var_qbex_dn4);
        var_qbex_dn5 = (assign5680_e6237 * var_qbex_dn5);
        var_qbex_dn6 = (assign5680_e6237 * var_qbex_dn6);
        var_qbex_dn7 = (assign5680_e6237 * var_qbex_dn7);
        var_qbex_dn8 = (assign5680_e6237 * var_qbex_dn8);
        var_qbex_dn9 = (assign5680_e6237 * var_qbex_dn9);
        var_qbex_dn10 = (assign5680_e6237 * var_qbex_dn10);
        var_qbex_dn11 = (assign5680_e6237 * var_qbex_dn11);
        var_qbex_dn12 = (assign5680_e6237 * var_qbex_dn12);
        var_qbex_dn13 = (assign5680_e6237 * var_qbex_dn13);

        let assign5690_e6242: f64 = var_vbictype;
        let assign5690_e6244: f64 = (assign5690_e6242 * var_qbc);
        var_qbc = assign5690_e6244;
        var_qbc_dn0 = (assign5690_e6242 * var_qbc_dn0);
        var_qbc_dn1 = (assign5690_e6242 * var_qbc_dn1);
        var_qbc_dn2 = (assign5690_e6242 * var_qbc_dn2);
        var_qbc_dn3 = (assign5690_e6242 * var_qbc_dn3);
        var_qbc_dn4 = (assign5690_e6242 * var_qbc_dn4);
        var_qbc_dn5 = (assign5690_e6242 * var_qbc_dn5);
        var_qbc_dn6 = (assign5690_e6242 * var_qbc_dn6);
        var_qbc_dn7 = (assign5690_e6242 * var_qbc_dn7);
        var_qbc_dn8 = (assign5690_e6242 * var_qbc_dn8);
        var_qbc_dn9 = (assign5690_e6242 * var_qbc_dn9);
        var_qbc_dn10 = (assign5690_e6242 * var_qbc_dn10);
        var_qbc_dn11 = (assign5690_e6242 * var_qbc_dn11);
        var_qbc_dn12 = (assign5690_e6242 * var_qbc_dn12);
        var_qbc_dn13 = (assign5690_e6242 * var_qbc_dn13);

        let assign5700_e6247: f64 = var_vbictype;
        let assign5700_e6249: f64 = (assign5700_e6247 * var_qbcx);
        var_qbcx = assign5700_e6249;
        var_qbcx_dn0 = (assign5700_e6247 * var_qbcx_dn0);
        var_qbcx_dn1 = (assign5700_e6247 * var_qbcx_dn1);
        var_qbcx_dn2 = (assign5700_e6247 * var_qbcx_dn2);
        var_qbcx_dn3 = (assign5700_e6247 * var_qbcx_dn3);
        var_qbcx_dn4 = (assign5700_e6247 * var_qbcx_dn4);
        var_qbcx_dn5 = (assign5700_e6247 * var_qbcx_dn5);
        var_qbcx_dn6 = (assign5700_e6247 * var_qbcx_dn6);
        var_qbcx_dn7 = (assign5700_e6247 * var_qbcx_dn7);
        var_qbcx_dn8 = (assign5700_e6247 * var_qbcx_dn8);
        var_qbcx_dn9 = (assign5700_e6247 * var_qbcx_dn9);
        var_qbcx_dn10 = (assign5700_e6247 * var_qbcx_dn10);
        var_qbcx_dn11 = (assign5700_e6247 * var_qbcx_dn11);
        var_qbcx_dn12 = (assign5700_e6247 * var_qbcx_dn12);
        var_qbcx_dn13 = (assign5700_e6247 * var_qbcx_dn13);

        let assign5710_e6252: f64 = var_vbictype;
        let assign5710_e6254: f64 = (assign5710_e6252 * var_qbep);
        var_qbep = assign5710_e6254;
        var_qbep_dn0 = (assign5710_e6252 * var_qbep_dn0);
        var_qbep_dn1 = (assign5710_e6252 * var_qbep_dn1);
        var_qbep_dn2 = (assign5710_e6252 * var_qbep_dn2);
        var_qbep_dn3 = (assign5710_e6252 * var_qbep_dn3);
        var_qbep_dn4 = (assign5710_e6252 * var_qbep_dn4);
        var_qbep_dn5 = (assign5710_e6252 * var_qbep_dn5);
        var_qbep_dn6 = (assign5710_e6252 * var_qbep_dn6);
        var_qbep_dn7 = (assign5710_e6252 * var_qbep_dn7);
        var_qbep_dn8 = (assign5710_e6252 * var_qbep_dn8);
        var_qbep_dn9 = (assign5710_e6252 * var_qbep_dn9);
        var_qbep_dn10 = (assign5710_e6252 * var_qbep_dn10);
        var_qbep_dn11 = (assign5710_e6252 * var_qbep_dn11);
        var_qbep_dn12 = (assign5710_e6252 * var_qbep_dn12);
        var_qbep_dn13 = (assign5710_e6252 * var_qbep_dn13);

        let assign5740_e6263: f64 = var_vbictype;
        let assign5740_e6265: f64 = (assign5740_e6263 * var_qbcp);
        var_qbcp = assign5740_e6265;
        var_qbcp_dn0 = (assign5740_e6263 * var_qbcp_dn0);
        var_qbcp_dn1 = (assign5740_e6263 * var_qbcp_dn1);
        var_qbcp_dn2 = (assign5740_e6263 * var_qbcp_dn2);
        var_qbcp_dn3 = (assign5740_e6263 * var_qbcp_dn3);
        var_qbcp_dn4 = (assign5740_e6263 * var_qbcp_dn4);
        var_qbcp_dn5 = (assign5740_e6263 * var_qbcp_dn5);
        var_qbcp_dn6 = (assign5740_e6263 * var_qbcp_dn6);
        var_qbcp_dn7 = (assign5740_e6263 * var_qbcp_dn7);
        var_qbcp_dn8 = (assign5740_e6263 * var_qbcp_dn8);
        var_qbcp_dn9 = (assign5740_e6263 * var_qbcp_dn9);
        var_qbcp_dn10 = (assign5740_e6263 * var_qbcp_dn10);
        var_qbcp_dn11 = (assign5740_e6263 * var_qbcp_dn11);
        var_qbcp_dn12 = (assign5740_e6263 * var_qbcp_dn12);
        var_qbcp_dn13 = (assign5740_e6263 * var_qbcp_dn13);

        let assign5750_e6268: f64 = var_qcth;
        var_qcth = assign5750_e6268;
        var_qcth_dn0 = var_qcth_dn0;
        var_qcth_dn1 = var_qcth_dn1;
        var_qcth_dn2 = var_qcth_dn2;
        var_qcth_dn3 = var_qcth_dn3;
        var_qcth_dn4 = var_qcth_dn4;
        var_qcth_dn5 = var_qcth_dn5;
        var_qcth_dn6 = var_qcth_dn6;
        var_qcth_dn7 = var_qcth_dn7;
        var_qcth_dn8 = var_qcth_dn8;
        var_qcth_dn9 = var_qcth_dn9;
        var_qcth_dn10 = var_qcth_dn10;
        var_qcth_dn11 = var_qcth_dn11;
        var_qcth_dn12 = var_qcth_dn12;
        var_qcth_dn13 = var_qcth_dn13;


        *var_expi_slot = var_expi;
        *var_expi_dn0_slot = var_expi_dn0;
        *var_expi_dn1_slot = var_expi_dn1;
        *var_expi_dn10_slot = var_expi_dn10;
        *var_expi_dn11_slot = var_expi_dn11;
        *var_expi_dn12_slot = var_expi_dn12;
        *var_expi_dn13_slot = var_expi_dn13;
        *var_expi_dn2_slot = var_expi_dn2;
        *var_expi_dn3_slot = var_expi_dn3;
        *var_expi_dn4_slot = var_expi_dn4;
        *var_expi_dn5_slot = var_expi_dn5;
        *var_expi_dn6_slot = var_expi_dn6;
        *var_expi_dn7_slot = var_expi_dn7;
        *var_expi_dn8_slot = var_expi_dn8;
        *var_expi_dn9_slot = var_expi_dn9;
        *var_qbc_slot = var_qbc;
        *var_qbc_dn0_slot = var_qbc_dn0;
        *var_qbc_dn1_slot = var_qbc_dn1;
        *var_qbc_dn10_slot = var_qbc_dn10;
        *var_qbc_dn11_slot = var_qbc_dn11;
        *var_qbc_dn12_slot = var_qbc_dn12;
        *var_qbc_dn13_slot = var_qbc_dn13;
        *var_qbc_dn2_slot = var_qbc_dn2;
        *var_qbc_dn3_slot = var_qbc_dn3;
        *var_qbc_dn4_slot = var_qbc_dn4;
        *var_qbc_dn5_slot = var_qbc_dn5;
        *var_qbc_dn6_slot = var_qbc_dn6;
        *var_qbc_dn7_slot = var_qbc_dn7;
        *var_qbc_dn8_slot = var_qbc_dn8;
        *var_qbc_dn9_slot = var_qbc_dn9;
        *var_qbcp_slot = var_qbcp;
        *var_qbcp_dn0_slot = var_qbcp_dn0;
        *var_qbcp_dn1_slot = var_qbcp_dn1;
        *var_qbcp_dn10_slot = var_qbcp_dn10;
        *var_qbcp_dn11_slot = var_qbcp_dn11;
        *var_qbcp_dn12_slot = var_qbcp_dn12;
        *var_qbcp_dn13_slot = var_qbcp_dn13;
        *var_qbcp_dn2_slot = var_qbcp_dn2;
        *var_qbcp_dn3_slot = var_qbcp_dn3;
        *var_qbcp_dn4_slot = var_qbcp_dn4;
        *var_qbcp_dn5_slot = var_qbcp_dn5;
        *var_qbcp_dn6_slot = var_qbcp_dn6;
        *var_qbcp_dn7_slot = var_qbcp_dn7;
        *var_qbcp_dn8_slot = var_qbcp_dn8;
        *var_qbcp_dn9_slot = var_qbcp_dn9;
        *var_qbcx_slot = var_qbcx;
        *var_qbcx_dn0_slot = var_qbcx_dn0;
        *var_qbcx_dn1_slot = var_qbcx_dn1;
        *var_qbcx_dn10_slot = var_qbcx_dn10;
        *var_qbcx_dn11_slot = var_qbcx_dn11;
        *var_qbcx_dn12_slot = var_qbcx_dn12;
        *var_qbcx_dn13_slot = var_qbcx_dn13;
        *var_qbcx_dn2_slot = var_qbcx_dn2;
        *var_qbcx_dn3_slot = var_qbcx_dn3;
        *var_qbcx_dn4_slot = var_qbcx_dn4;
        *var_qbcx_dn5_slot = var_qbcx_dn5;
        *var_qbcx_dn6_slot = var_qbcx_dn6;
        *var_qbcx_dn7_slot = var_qbcx_dn7;
        *var_qbcx_dn8_slot = var_qbcx_dn8;
        *var_qbcx_dn9_slot = var_qbcx_dn9;
        *var_qbe_slot = var_qbe;
        *var_qbe_dn0_slot = var_qbe_dn0;
        *var_qbe_dn1_slot = var_qbe_dn1;
        *var_qbe_dn10_slot = var_qbe_dn10;
        *var_qbe_dn11_slot = var_qbe_dn11;
        *var_qbe_dn12_slot = var_qbe_dn12;
        *var_qbe_dn13_slot = var_qbe_dn13;
        *var_qbe_dn2_slot = var_qbe_dn2;
        *var_qbe_dn3_slot = var_qbe_dn3;
        *var_qbe_dn4_slot = var_qbe_dn4;
        *var_qbe_dn5_slot = var_qbe_dn5;
        *var_qbe_dn6_slot = var_qbe_dn6;
        *var_qbe_dn7_slot = var_qbe_dn7;
        *var_qbe_dn8_slot = var_qbe_dn8;
        *var_qbe_dn9_slot = var_qbe_dn9;
        *var_qbep_slot = var_qbep;
        *var_qbep_dn0_slot = var_qbep_dn0;
        *var_qbep_dn1_slot = var_qbep_dn1;
        *var_qbep_dn10_slot = var_qbep_dn10;
        *var_qbep_dn11_slot = var_qbep_dn11;
        *var_qbep_dn12_slot = var_qbep_dn12;
        *var_qbep_dn13_slot = var_qbep_dn13;
        *var_qbep_dn2_slot = var_qbep_dn2;
        *var_qbep_dn3_slot = var_qbep_dn3;
        *var_qbep_dn4_slot = var_qbep_dn4;
        *var_qbep_dn5_slot = var_qbep_dn5;
        *var_qbep_dn6_slot = var_qbep_dn6;
        *var_qbep_dn7_slot = var_qbep_dn7;
        *var_qbep_dn8_slot = var_qbep_dn8;
        *var_qbep_dn9_slot = var_qbep_dn9;
        *var_qbex_slot = var_qbex;
        *var_qbex_dn0_slot = var_qbex_dn0;
        *var_qbex_dn1_slot = var_qbex_dn1;
        *var_qbex_dn10_slot = var_qbex_dn10;
        *var_qbex_dn11_slot = var_qbex_dn11;
        *var_qbex_dn12_slot = var_qbex_dn12;
        *var_qbex_dn13_slot = var_qbex_dn13;
        *var_qbex_dn2_slot = var_qbex_dn2;
        *var_qbex_dn3_slot = var_qbex_dn3;
        *var_qbex_dn4_slot = var_qbex_dn4;
        *var_qbex_dn5_slot = var_qbex_dn5;
        *var_qbex_dn6_slot = var_qbex_dn6;
        *var_qbex_dn7_slot = var_qbex_dn7;
        *var_qbex_dn8_slot = var_qbex_dn8;
        *var_qbex_dn9_slot = var_qbex_dn9;
        *var_qcth_slot = var_qcth;
        *var_qcth_dn0_slot = var_qcth_dn0;
        *var_qcth_dn1_slot = var_qcth_dn1;
        *var_qcth_dn10_slot = var_qcth_dn10;
        *var_qcth_dn11_slot = var_qcth_dn11;
        *var_qcth_dn12_slot = var_qcth_dn12;
        *var_qcth_dn13_slot = var_qcth_dn13;
        *var_qcth_dn2_slot = var_qcth_dn2;
        *var_qcth_dn3_slot = var_qcth_dn3;
        *var_qcth_dn4_slot = var_qcth_dn4;
        *var_qcth_dn5_slot = var_qcth_dn5;
        *var_qcth_dn6_slot = var_qcth_dn6;
        *var_qcth_dn7_slot = var_qcth_dn7;
        *var_qcth_dn8_slot = var_qcth_dn8;
        *var_qcth_dn9_slot = var_qcth_dn9;
        *var_tff_slot = var_tff;
        *var_tff_dn0_slot = var_tff_dn0;
        *var_tff_dn1_slot = var_tff_dn1;
        *var_tff_dn10_slot = var_tff_dn10;
        *var_tff_dn11_slot = var_tff_dn11;
        *var_tff_dn12_slot = var_tff_dn12;
        *var_tff_dn13_slot = var_tff_dn13;
        *var_tff_dn2_slot = var_tff_dn2;
        *var_tff_dn3_slot = var_tff_dn3;
        *var_tff_dn4_slot = var_tff_dn4;
        *var_tff_dn5_slot = var_tff_dn5;
        *var_tff_dn6_slot = var_tff_dn6;
        *var_tff_dn7_slot = var_tff_dn7;
        *var_tff_dn8_slot = var_tff_dn8;
        *var_tff_dn9_slot = var_tff_dn9;
    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.b[172] = ctx.analysis_static();
        s.store_scalar(172, if s.b[172] { 1.0 } else { 0.0 });

        s.b[176] = param_given[11];
        s.store_scalar(176, if s.b[176] { 1.0 } else { 0.0 });

        if (s.b[172] && s.b[176]) {
            s.store_scalar(166, p.p11);
        }

        if (s.b[172] && (!s.b[176])) {
            s.store_scalar(166, 1.0);
        }

        s.b[177] = param_given[3];
        s.store_scalar(177, if s.b[177] { 1.0 } else { 0.0 });

        if (s.b[172] && s.b[177]) {
            s.store_scalar(162, 1.0);
        }

        s.b[178] = param_given[4];
        s.store_scalar(178, if s.b[178] { 1.0 } else { 0.0 });

        if ((s.b[172] && (!s.b[177])) && s.b[178]) {
            s.store_scalar(162, (-1.0));
        }

        s.b[179] = param_given[5];
        s.store_scalar(179, if s.b[179] { 1.0 } else { 0.0 });

        if (((s.b[172] && (!s.b[177])) && (!s.b[178])) && s.b[179]) {
            s.store_scalar(162, p.p5);
        }

        if (((s.b[172] && (!s.b[177])) && (!s.b[178])) && (!s.b[179])) {
            s.store_scalar(162, 1.0);
        }

        if s.b[172] {
            s.store_scalar(113, ((p.p12) as f64).ln());
        }

        if s.b[172] {
            s.store_scalar(46, (if (p.p74 > 0.0) { (1.0 / p.p74) } else { 0.0 }));
        }

        if s.b[172] {
            s.store_scalar(47, (if (p.p75 > 0.0) { (1.0 / p.p75) } else { 0.0 }));
        }

        if s.b[172] {
            s.store_scalar(50, (if (p.p79 > 0.0) { (1.0 / p.p79) } else { 0.0 }));
        }

        if s.b[172] {
            s.store_scalar(51, (if (p.p80 > 0.0) { (1.0 / p.p80) } else { 0.0 }));
        }

        if s.b[172] {
            s.store_scalar(52, (if (p.p80 > 0.0) { 0.0 } else { 1.0 }));
        }

        if s.b[172] {
            s.store_scalar(40, (273.15 + p.p13));
        }

        s.store_scalar(38, ((ctx_temp + p.p0) - 273.15));

        s.b[182] = (s.v[38] < (p.p14 + 1.0));
        s.store_scalar(182, if s.b[182] { 1.0 } else { 0.0 });

        if s.b[182] {
            s.store_scalar(38, (p.p14 + ((((s.v[38] - p.p14) - 1.0)) as f64).exp()));
        }

        s.b[183] = (s.v[38] > (p.p15 - 1.0));
        s.store_scalar(183, if s.b[183] { 1.0 } else { 0.0 });

        if ((!s.b[182]) && s.b[183]) {
            s.store_sub_from_scalar_ad(38, p.p15, A::exp(A::offset(A::sub_from_scalar(p.p15, s.ad_value(38)), (-1.0))));
        }

        if ((!s.b[182]) && (!s.b[183])) {
        }

        s.store_offset(39, 38, 273.15);

        s.store_scale(73, 39, (1.380662e-23 * 6.241460901304403e18));

        s.store_div(41, 39, 40);

        s.b[184] = (p.p90 > 0.0);
        s.store_scalar(184, if s.b[184] { 1.0 } else { 0.0 });

        if s.b[184] {
            s.store_mul_scaled_ln_ad_rhs(64, 73, p.p89, A::add_scaled_inputs(A::exp(A::div_from_scalar((-p.p88), A::scale(s.ad_value(73), p.p89))), 1.0, s.ad_value(166), 1.0 / (p.p90)));
        }

        if (!s.b[184]) {
            s.store_scalar(64, 0.0);
        }

        s.store_scaled_mul_ad(0, A::powf(s.ad_value(41), (p.p122 / p.p28)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p113)), (-p.p113), s.ad_value(73), p.p28)), p.p26);

        s.b[185] = (s.v[0] > 0.0);
        s.store_scalar(185, if s.b[185] { 1.0 } else { 0.0 });

        s.b[186] = ((p.p72 > 0.0) && (s.v[166] > p.p72));
        s.store_scalar(186, if s.b[186] { 1.0 } else { 0.0 });

        if (s.b[185] && s.b[186]) {
            s.store_mul_scaled_ln_ad_rhs(61, 73, p.p28, A::offset(A::div(A::powf(A::scale(s.ad_value(166), (0.5 * (((4.0 / p.p72)) as f64).powf(p.p73))), (1.0 / (1.0 - p.p73))), s.ad_value(0)), 1.0));
        }

        if (s.b[185] && (!s.b[186])) {
            s.store_mul_scaled_ln_ad_rhs(61, 73, p.p28, A::offset(A::div(s.ad_value(166), s.ad_value(0)), 1.0));
        }

        if (!s.b[185]) {
            s.store_scalar(61, 0.0);
        }

        s.store_scaled_mul_ad(1, A::powf(s.ad_value(41), (p.p125 / p.p29)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p121)), (-p.p121), s.ad_value(73), p.p29)), p.p27);

        s.b[187] = ((s.v[0] > 0.0) && (s.v[1] > 0.0));
        s.store_scalar(187, if s.b[187] { 1.0 } else { 0.0 });

        s.b[188] = ((p.p74 > 0.0) && (s.v[166] > p.p74));
        s.store_scalar(188, if s.b[188] { 1.0 } else { 0.0 });

        if (s.b[187] && s.b[188]) {
            s.store_mul_scaled_ln_ad_rhs(62, 73, p.p29, A::offset(A::div(A::powf(A::scale(s.ad_value(166), (0.5 * (((4.0 / p.p74)) as f64).powf(p.p73))), (1.0 / (1.0 - p.p73))), A::mul(s.ad_value(0), s.ad_value(1))), 1.0));
        }

        if (s.b[187] && (!s.b[188])) {
            s.store_mul_scaled_ln_ad_rhs(62, 73, p.p29, A::offset(A::div(s.ad_value(166), A::mul(s.ad_value(0), s.ad_value(1))), 1.0));
        }

        if (!s.b[187]) {
            s.store_scalar(62, 0.0);
        }

        s.store_scaled_mul_ad(5, A::powf(s.ad_value(41), (p.p122 / p.p33)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p120)), (-p.p120), s.ad_value(73), p.p33)), p.p31);

        s.b[189] = (s.v[5] > 0.0);
        s.store_scalar(189, if s.b[189] { 1.0 } else { 0.0 });

        s.b[190] = ((p.p75 > 0.0) && (s.v[166] > p.p75));
        s.store_scalar(190, if s.b[190] { 1.0 } else { 0.0 });

        if (s.b[189] && s.b[190]) {
            s.store_mul_scaled_ln_ad_rhs(63, 73, p.p33, A::offset(A::div_scaled_product(A::square(s.ad_value(166)), s.ad_value(47), 1.0, s.ad_value(5), 1.0), 1.0));
        }

        if (s.b[189] && (!s.b[190])) {
            s.store_mul_scaled_ln_ad_rhs(63, 73, p.p33, A::offset(A::div(s.ad_value(166), s.ad_value(5)), 1.0));
        }

        if (!s.b[189]) {
            s.store_scalar(63, 0.0);
        }

        s.store_scaled_mul_ad(3, A::powf(s.ad_value(41), (p.p123 / p.p56)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p114)), (-p.p114), s.ad_value(73), p.p56)), p.p54);

        s.b[191] = (s.v[3] > 0.0);
        s.store_scalar(191, if s.b[191] { 1.0 } else { 0.0 });

        if s.b[191] {
            s.store_mul_scaled_ln_ad_rhs(65, 73, p.p56, A::offset(A::div(s.ad_value(166), s.ad_value(3)), 1.0));
        }

        if (!s.b[191]) {
            s.store_scalar(65, 0.0);
        }

        s.store_scaled_mul_ad(4, A::powf(s.ad_value(41), (p.p123 / p.p61)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p115)), (-p.p115), s.ad_value(73), p.p61)), p.p60);

        s.b[193] = (s.v[4] > 0.0);
        s.store_scalar(193, if s.b[193] { 1.0 } else { 0.0 });

        if s.b[193] {
            s.store_mul_scaled_ln_ad_rhs(67, 73, p.p61, A::offset(A::div(s.ad_value(166), s.ad_value(4)), 1.0));
        }

        if (!s.b[193]) {
            s.store_scalar(67, 0.0);
        }

        s.store_scaled_mul_ad(8, A::powf(s.ad_value(41), (p.p123 / p.p61)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p115)), (-p.p115), s.ad_value(73), p.p61)), p.p64);

        s.b[195] = (s.v[8] > 0.0);
        s.store_scalar(195, if s.b[195] { 1.0 } else { 0.0 });

        if s.b[195] {
            s.store_mul_scaled_ln_ad_rhs(69, 73, p.p61, A::offset(A::div(s.ad_value(166), s.ad_value(8)), 1.0));
        }

        if (!s.b[195]) {
            s.store_scalar(69, 0.0);
        }

        s.store_scaled_mul_ad(10, A::powf(s.ad_value(41), (p.p123 / p.p67)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p116)), (-p.p116), s.ad_value(73), p.p67)), p.p66);

        s.b[197] = (s.v[10] > 0.0);
        s.store_scalar(197, if s.b[197] { 1.0 } else { 0.0 });

        if s.b[197] {
            s.store_mul_scaled_ln_ad_rhs(71, 73, p.p67, A::offset(A::div(s.ad_value(166), s.ad_value(10)), 1.0));
        }

        if (!s.b[197]) {
            s.store_scalar(71, 0.0);
        }

        s.store_voltage(138, ctx, nodes, Some(4), None);

        s.store_offset(38, 138, (((ctx_temp + p.p0)) + ((-273.15))));

        s.b[199] = (s.v[38] < (p.p14 + 1.0));
        s.store_scalar(199, if s.b[199] { 1.0 } else { 0.0 });

        if s.b[199] {
            s.store_offset_exp_ad(38, A::offset(s.ad_value(38), (((-p.p14)) + ((-1.0)))), p.p14);
        }

        s.b[200] = (s.v[38] > (p.p15 - 1.0));
        s.store_scalar(200, if s.b[200] { 1.0 } else { 0.0 });

        if ((!s.b[199]) && s.b[200]) {
            s.store_sub_from_scalar_ad(38, p.p15, A::exp(A::offset(A::sub_from_scalar(p.p15, s.ad_value(38)), (-1.0))));
        }

        if ((!s.b[199]) && (!s.b[200])) {
        }

        s.store_offset(39, 38, 273.15);

        s.store_scale(73, 39, (1.380662e-23 * 6.241460901304403e18));

        s.store_div(41, 39, 40);

        s.store_sub(42, 39, 40);

        s.store_scale_ad(2, A::powf(s.ad_value(41), p.p126), p.p72);

        s.store_scaled_mul_ad(0, A::powf(s.ad_value(41), (p.p122 / p.p28)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p113)), (-p.p113), s.ad_value(73), p.p28)), p.p26);

        s.store_scaled_mul_ad(1, A::powf(s.ad_value(41), (p.p125 / p.p29)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p121)), (-p.p121), s.ad_value(73), p.p29)), p.p27);

        s.store_scaled_mul_ad(5, A::powf(s.ad_value(41), (p.p122 / p.p33)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p120)), (-p.p120), s.ad_value(73), p.p33)), p.p31);

        s.store_scaled_mul_ad(3, A::powf(s.ad_value(41), (p.p123 / p.p56)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p114)), (-p.p114), s.ad_value(73), p.p56)), p.p54);

        s.store_scaled_mul_ad(4, A::powf(s.ad_value(41), (p.p123 / p.p61)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p115)), (-p.p115), s.ad_value(73), p.p61)), p.p60);

        s.store_scaled_mul_ad(8, A::powf(s.ad_value(41), (p.p123 / p.p61)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p115)), (-p.p115), s.ad_value(73), p.p61)), p.p64);

        s.store_scaled_mul_ad(10, A::powf(s.ad_value(41), (p.p123 / p.p67)), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p116)), (-p.p116), s.ad_value(73), p.p67)), p.p66);

        s.store_offset_scaled(27, 42, ((p.p129) * (p.p28)), p.p28);

        s.store_offset_scaled(28, 42, ((p.p129) * (p.p29)), p.p29);

        s.store_scaled_offset_ad(31, A::mul(s.ad_value(42), A::scale_offset(s.ad_value(42), p.p92, p.p91)), 1.0, p.p88);

        s.store_offset_scaled(32, 42, ((p.p93) * (p.p89)), p.p89);

        s.store_scaled_mul_ad(206, A::div(s.ad_value(73), s.ad_value(41)), A::ln(A::sub(A::exp_div_scaled_inputs(s.ad_value(41), (0.5 * p.p37), s.ad_value(73), 1.0), A::exp_div_scaled_inputs(s.ad_value(41), ((-0.5) * p.p37), s.ad_value(73), 1.0))), 2.0);

        s.store_sub_ad(207, A::add_scaled_products(s.ad_value(206), s.ad_value(41), 1.0, s.ad_value(73), A::ln(s.ad_value(41)), (-3.0)), A::scaled_offset(s.ad_value(41), (-1.0), p.p114));

        s.store_add_scaled_product_right_ad(20, 207, 1.0, 73, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp_div_scaled_inputs(s.ad_value(207), -1.0, s.ad_value(73), 1.0), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scaled_mul_ad(208, A::div(s.ad_value(73), s.ad_value(41)), A::ln(A::sub(A::exp_div_scaled_inputs(s.ad_value(41), (0.5 * p.p42), s.ad_value(73), 1.0), A::exp_div_scaled_inputs(s.ad_value(41), ((-0.5) * p.p42), s.ad_value(73), 1.0))), 2.0);

        s.store_sub_ad(209, A::add_scaled_products(s.ad_value(208), s.ad_value(41), 1.0, s.ad_value(73), A::ln(s.ad_value(41)), (-3.0)), A::scaled_offset(s.ad_value(41), (-1.0), p.p115));

        s.store_add_scaled_product_right_ad(21, 209, 1.0, 73, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp_div_scaled_inputs(s.ad_value(209), -1.0, s.ad_value(73), 1.0), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scaled_mul_ad(210, A::div(s.ad_value(73), s.ad_value(41)), A::ln(A::sub(A::exp_div_scaled_inputs(s.ad_value(41), (0.5 * p.p50), s.ad_value(73), 1.0), A::exp_div_scaled_inputs(s.ad_value(41), ((-0.5) * p.p50), s.ad_value(73), 1.0))), 2.0);

        s.store_sub_ad(211, A::add_scaled_products(s.ad_value(210), s.ad_value(41), 1.0, s.ad_value(73), A::ln(s.ad_value(41)), (-3.0)), A::scaled_offset(s.ad_value(41), (-1.0), p.p116));

        s.store_add_scaled_product_right_ad(22, 211, 1.0, 73, A::ln_scaled_input(A::offset(A::sqrt(A::scale_offset(A::exp_div_scaled_inputs(s.ad_value(211), -1.0, s.ad_value(73), 1.0), 4.0, 1.0)), 1.0), 0.5), 2.0);

        s.store_scaled_powf_ad(23, A::div_from_scalar(p.p37, s.ad_value(20)), p.p38, p.p36);

        s.store_scaled_powf_ad(24, A::div_from_scalar(p.p42, s.ad_value(21)), p.p43, p.p41);

        s.store_scaled_powf_ad(25, A::div_from_scalar(p.p42, s.ad_value(21)), p.p43, p.p48);

        s.store_scaled_powf_ad(26, A::div_from_scalar(p.p50, s.ad_value(22)), p.p51, p.p49);

        s.store_scaled_mul_ad(33, A::powf(s.ad_value(41), p.p122), A::exp(A::div_scaled_offset_numerator(s.ad_value(41), (-(-p.p113)), (-p.p113), s.ad_value(73), 1.0)), p.p19);

        s.store_offset_scaled(36, 42, ((p.p130) * (p.p70)), p.p70);

        s.store_offset_scaled(37, 42, ((p.p131) * (p.p71)), p.p71);

        if (s.v[36] > 0.0) {
            s.store_div_from_scalar(43, 1.0, 36);
        } else {
            s.store_scalar(43, 0.0);
        }

        if (s.v[37] > 0.0) {
            s.store_div_from_scalar(44, 1.0, 37);
        } else {
            s.store_scalar(44, 0.0);
        }

        if (s.v[2] > 0.0) {
            s.store_div_from_scalar(45, 1.0, 2);
        } else {
            s.store_scalar(45, 0.0);
        }

        s.store_mul_voltage_ad(143, s.ad_value(162), ctx, nodes, Some(8), Some(9));

        s.store_mul_voltage_ad(145, s.ad_value(162), ctx, nodes, Some(7), Some(9));

        s.store_mul_voltage_ad(144, s.ad_value(162), ctx, nodes, Some(8), Some(6));

        s.store_mul_voltage_ad(148, s.ad_value(162), ctx, nodes, Some(8), Some(5));

        s.store_mul_voltage_ad(146, s.ad_value(162), ctx, nodes, Some(7), Some(10));

        s.store_mul_voltage_ad(147, s.ad_value(162), ctx, nodes, Some(11), Some(10));

        s.store_scale(212, 20, (-p.p34));

        s.b[223] = (p.p39 <= 0.0);
        s.store_scalar(223, if s.b[223] { 1.0 } else { 0.0 });

        if s.b[223] {
            s.store_add(213, 143, 212);
        }

        s.b[224] = (s.v[213] > 0.0);
        s.store_scalar(224, if s.b[224] { 1.0 } else { 0.0 });

        if (s.b[223] && s.b[224]) {
            s.store_scalar(214, (((1.0 - p.p34)) as f64).powf((-p.p38)));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(215, 20, 1.0, 214, (1.0 - p.p34), 1.0 / ((1.0 - p.p38)));
            s.store_mul_ad_product_lhs_mixed_ia(216, 213, A::offset(A::div_scaled_inputs(s.ad_value(213), (0.5 * p.p38), s.ad_value(20), (1.0 - p.p34)), 1.0), 214);
        }

        if (s.b[223] && (!s.b[224])) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(215, 20, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(143), s.ad_value(20))), (1.0 - p.p38)), 1.0 / ((1.0 - p.p38)));
            s.store_scalar(216, 0.0);
        }

        if s.b[223] {
            s.store_add(114, 215, 216);
        }

        if (!s.b[223]) {
            s.store_sqrt_square_offset(217, 212, ((4.0 * p.p39) * p.p39));
            s.store_scaled_add(218, 212, 217, (-0.5));
            s.store_mul_scaled_powf_ad_rhs(219, 20, (-1.0 / ((1.0 - p.p38))), A::sub_from_scalar(1.0, A::div(s.ad_value(218), s.ad_value(20))), (1.0 - p.p38));
            s.store_add(220, 143, 212);
            s.store_sqrt_square_offset(221, 220, ((4.0 * p.p39) * p.p39));
            s.store_add_scaled_inputs3_indices(222, 220, 0.5, 221, (-0.5), 212, -1.0);
            s.store_mul_scaled_powf_ad_rhs(215, 20, (-1.0 / ((1.0 - p.p38))), A::sub_from_scalar(1.0, A::div(s.ad_value(222), s.ad_value(20))), (1.0 - p.p38));
            s.store_sub_ad_lhs(114, A::add_scaled_offset_product_rhs(s.ad_value(215), 1.0, A::add_scaled_inputs3(s.ad_value(143), 1.0, s.ad_value(222), (-1.0), s.ad_value(218), 1.0), A::div_scaled_inputs3(s.ad_value(143), (0.5 * p.p38), s.ad_value(222), ((-1.0) * (0.5 * p.p38)), s.ad_value(218), (0.5 * p.p38), s.ad_value(20), (1.0 - p.p34)), 1.0, (((1.0 - p.p34)) as f64).powf((-p.p38))), 219);
        }

        s.store_scale(225, 21, (-p.p34));

        s.b[246] = (p.p44 <= 0.0);
        s.store_scalar(246, if s.b[246] { 1.0 } else { 0.0 });

        if s.b[246] {
            s.store_add(226, 144, 225);
        }

        s.b[247] = (s.v[226] > 0.0);
        s.store_scalar(247, if s.b[247] { 1.0 } else { 0.0 });

        if (s.b[246] && s.b[247]) {
            s.store_scalar(227, (((1.0 - p.p34)) as f64).powf(((-1.0) - p.p43)));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(228, 21, 1.0, 227, ((1.0 - p.p34) * (1.0 - p.p34)), 1.0 / ((1.0 - p.p43)));
            s.store_mul_ad_product_lhs_mixed_ia(229, 226, A::offset(A::div_scaled_inputs(s.ad_value(226), (0.5 * p.p43), s.ad_value(21), 1.0), (1.0 - p.p34)), 227);
        }

        s.b[248] = ((p.p45 > 0.0) && (s.v[144] < (-p.p45)));
        s.store_scalar(248, if s.b[248] { 1.0 } else { 0.0 });

        if ((s.b[246] && (!s.b[247])) && s.b[248]) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(228, 21, 1.0, A::mul_sub_from_scalar_rhs(A::powf(A::offset(A::div_from_scalar(p.p45, s.ad_value(21)), 1.0), (1.0 - p.p43)), 1.0, A::div_scaled_offset_numerator(s.ad_value(144), (1.0 - p.p43), (p.p45 * (1.0 - p.p43)), A::offset(s.ad_value(21), p.p45), 1.0)), 1.0 / ((1.0 - p.p43)));
        }

        if ((s.b[246] && (!s.b[247])) && (!s.b[248])) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(228, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(144), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
        }

        if (s.b[246] && (!s.b[247])) {
            s.store_scalar(229, 0.0);
        }

        if s.b[246] {
            s.store_add(116, 228, 229);
        }

        s.b[249] = ((p.p45 > 0.0) && (p.p46 > 0.0));
        s.store_scalar(249, if s.b[249] { 1.0 } else { 0.0 });

        if ((!s.b[246]) && s.b[249]) {
            s.store_div_scaled_offset_numerator(230, s.ad_value(225), 1.0, p.p45, A::sub_from_scalar(p.p45, s.ad_value(225)), 1.0);
            s.store_div_scaled_inputs_mixed_ia(231, 230, 2.0, A::add(A::sqrt_square_offset(A::offset(s.ad_value(230), (-1.0)), ((4.0 * p.p44) * p.p44)), A::sqrt_square_offset(A::offset(s.ad_value(230), 1.0), ((4.0 * p.p46) * p.p46))), 1.0);
            s.store_scaled_sub_ad_lhs(232, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(231), p.p45, s.ad_value(225)), (-p.p45)), 225, 0.5);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(233, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(232), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
            s.store_div_scaled_inputs2_mixed_aia(234, A::scale_offset(s.ad_value(144), 2.0, p.p45), 1.0, 225, 1.0, A::sub_from_scalar(p.p45, s.ad_value(225)), 1.0);
            s.store_div_scaled_inputs_mixed_ia(235, 234, 2.0, A::add(A::sqrt_square_offset(A::offset(s.ad_value(234), (-1.0)), ((4.0 * p.p44) * p.p44)), A::sqrt_square_offset(A::offset(s.ad_value(234), 1.0), ((4.0 * p.p46) * p.p46))), 1.0);
            s.store_scaled_sub_ad_lhs(236, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(235), p.p45, s.ad_value(225)), (-p.p45)), 225, 0.5);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(228, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(236), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
            s.store_scaled_offset(237, 235, 1.0, 0.5);
            s.store_powf_ad(238, A::offset(A::div_from_scalar(p.p45, s.ad_value(21)), 1.0), (-p.p43));
            s.store_powf_ad(239, A::offset(A::div(s.ad_value(225), s.ad_value(21)), 1.0), (-p.p43));
            s.store_add_scaled_product_value_ad(240, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(237), s.ad_value(238)), 1.0, 237, 239, 1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(241, 240, s.ad_value(144), 1.0, s.ad_value(236), (-1.0), s.ad_value(232), 1.0, 0.0);
            s.store_add_scaled_inputs3_indices(116, 241, 1.0, 228, 1.0, 233, -1.0);
        }

        if ((!s.b[246]) && (!s.b[249])) {
            s.store_sqrt_square_offset(242, 225, ((4.0 * p.p44) * p.p44));
            s.store_scaled_add(232, 225, 242, (-0.5));
            s.store_mul_scaled_powf_ad_rhs(243, 21, (-1.0 / ((1.0 - p.p43))), A::sub_from_scalar(1.0, A::div(s.ad_value(232), s.ad_value(21))), (1.0 - p.p43));
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[246]) && (!s.b[249])) {
            s.store_add(244, 144, 225);
            s.store_sqrt_square_offset(245, 244, ((4.0 * p.p44) * p.p44));
            s.store_add_scaled_inputs3_indices(236, 244, 0.5, 245, (-0.5), 225, -1.0);
            s.store_mul_scaled_powf_ad_rhs(228, 21, (-1.0 / ((1.0 - p.p43))), A::sub_from_scalar(1.0, A::div(s.ad_value(236), s.ad_value(21))), (1.0 - p.p43));
            s.store_sub_add_scaled_inputs4_lhs_indices(116, 228, 1.0, 144, (((1.0 - p.p34)) as f64).powf((-p.p43)), 236, ((-1.0) * (((1.0 - p.p34)) as f64).powf((-p.p43))), 232, (((1.0 - p.p34)) as f64).powf((-p.p43)), 243);
        }

        s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(27), s.ad_value(73));

        s.b[250] = (s.v[143] < s.v[61]);
        s.store_scalar(250, if s.b[250] { 1.0 } else { 0.0 });

        if s.b[250] {
            s.store_exp_mul(109, 143, 112);
        }

        if (!s.b[250]) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(61), s.ad_value(112))), A::mul(A::sub(s.ad_value(143), s.ad_value(61)), s.ad_value(112)), 1.0);
        }

        s.store_mul_offset_rhs(74, 0, 109, (-1.0));

        s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(28), s.ad_value(73));

        s.b[251] = (s.v[144] < s.v[62]);
        s.store_scalar(251, if s.b[251] { 1.0 } else { 0.0 });

        if s.b[251] {
            s.store_exp_mul(109, 144, 112);
        }

        if (!s.b[251]) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(62), s.ad_value(112))), A::mul(A::sub(s.ad_value(144), s.ad_value(62)), s.ad_value(112)), 1.0);
        }

        s.store_mul_ad_product_rhs_mixed_ia(75, 0, 1, A::offset(s.ad_value(109), (-1.0)));

        s.store_offset_add_scaled_product(78, A::offset(A::mul(s.ad_value(114), s.ad_value(44)), 1.0), 1.0, s.ad_value(116), s.ad_value(43), 1.0, (-0.0001));

        s.store_offset_add_scaled_inputs_mixed_ai(79, A::sqrt_square_offset(s.ad_value(78), 1e-8), 0.5, 78, 0.5, 0.0001);

        s.store_add_scaled_products_indices(80, 74, 45, 1.0, 75, 46, 1.0);

        s.b[252] = (p.p30 < 0.5);
        s.store_scalar(252, if s.b[252] { 1.0 } else { 0.0 });

        if s.b[252] {
            s.store_add_scaled_ad_lhs(108, A::powf(s.ad_value(79), (1.0 / p.p73)), 80, 4.0);
        }

        s.b[253] = (s.v[108] > 1e-8);
        s.store_scalar(253, if s.b[253] { 1.0 } else { 0.0 });

        if (s.b[252] && s.b[253]) {
            s.store_scaled_add_ad_rhs(81, 79, A::powf(s.ad_value(108), p.p73), 0.5);
        }

        if (s.b[252] && (!s.b[253])) {
            s.store_scaled_offset(81, 79, ((1e-8) as f64).powf(p.p73), 0.5);
        }

        if (!s.b[252]) {
            s.store_offset_scaled(108, 80, 4.0, 1.0);
        }

        s.b[254] = (s.v[108] > 1e-8);
        s.store_scalar(254, if s.b[254] { 1.0 } else { 0.0 });

        if ((!s.b[252]) && s.b[254]) {
            s.store_mul_scaled_offset_ad_rhs(81, 79, 0.5, A::powf(s.ad_value(108), p.p73), 1.0);
        }

        if ((!s.b[252]) && (!s.b[254])) {
            s.store_scale(81, 79, (0.5 * (1.0 + ((1e-8) as f64).powf(p.p73))));
        }

        s.b[255] = (p.p31 > 0.0);
        s.store_scalar(255, if s.b[255] { 1.0 } else { 0.0 });

        if s.b[255] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p33);
        }

        s.b[256] = (s.v[146] < s.v[63]);
        s.store_scalar(256, if s.b[256] { 1.0 } else { 0.0 });

        if (s.b[255] && s.b[256]) {
            s.store_exp_mul(109, 146, 112);
        }

        if (s.b[255] && (!s.b[256])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(63), s.ad_value(112))), A::mul(A::sub(s.ad_value(146), s.ad_value(63)), s.ad_value(112)), 1.0);
        }

        s.b[257] = (s.v[144] < s.v[63]);
        s.store_scalar(257, if s.b[257] { 1.0 } else { 0.0 });

        if (s.b[255] && s.b[257]) {
            s.store_exp_mul(111, 144, 112);
        }

        if (s.b[255] && (!s.b[257])) {
            s.store_mul_offset_rhs_ad(111, A::exp(A::mul(s.ad_value(63), s.ad_value(112))), A::mul(A::sub(s.ad_value(144), s.ad_value(63)), s.ad_value(112)), 1.0);
        }

        if s.b[255] {
            s.store_mul_offset_ad_rhs(82, 5, A::add_scaled_inputs(s.ad_value(109), p.p32, s.ad_value(111), (1.0 - p.p32)), (-1.0));
            s.store_mul(85, 82, 47);
            s.store_offset_scaled(108, 85, 4.0, 1.0);
        }

        s.b[259] = (s.v[147] < s.v[63]);
        s.store_scalar(259, if s.b[259] { 1.0 } else { 0.0 });

        if (s.b[255] && s.b[259]) {
            s.store_exp_mul(109, 147, 112);
        }

        if (s.b[255] && (!s.b[259])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(63), s.ad_value(112))), A::mul(A::sub(s.ad_value(147), s.ad_value(63)), s.ad_value(112)), 1.0);
        }

        if (!s.b[255]) {
            s.store_scalar(82, 0.0);
        }

        s.b[260] = (p.p55 == 1.0);
        s.store_scalar(260, if s.b[260] { 1.0 } else { 0.0 });

        if s.b[260] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p56);
        }

        s.b[261] = (s.v[143] < s.v[65]);
        s.store_scalar(261, if s.b[261] { 1.0 } else { 0.0 });

        if (s.b[260] && s.b[261]) {
            s.store_exp_mul(109, 143, 112);
        }

        if (s.b[260] && (!s.b[261])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(65), s.ad_value(112))), A::mul(A::sub(s.ad_value(143), s.ad_value(65)), s.ad_value(112)), 1.0);
        }

        if s.b[260] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p59);
        }

        s.b[264] = (p.p88 > 0.0);
        s.store_scalar(264, if s.b[264] { 1.0 } else { 0.0 });

        if (s.b[260] && s.b[264]) {
            s.store_sub_scaled_inputs(150, 31, -1.0, 143, 1.0);
            s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(32), s.ad_value(73));
        }

        s.b[265] = (s.v[150] < s.v[64]);
        s.store_scalar(265, if s.b[265] { 1.0 } else { 0.0 });

        if ((s.b[260] && s.b[264]) && s.b[265]) {
            s.store_exp_mul(111, 150, 112);
        }

        if ((s.b[260] && s.b[264]) && (!s.b[265])) {
            s.store_mul_offset_rhs_ad(111, A::exp(A::mul(s.ad_value(64), s.ad_value(112))), A::mul(A::sub(s.ad_value(150), s.ad_value(64)), s.ad_value(112)), 1.0);
        }

        s.b[266] = (p.p55 == 0.0);
        s.store_scalar(266, if s.b[266] { 1.0 } else { 0.0 });

        if ((!s.b[260]) && s.b[266]) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p56);
        }

        s.b[267] = (s.v[145] < s.v[65]);
        s.store_scalar(267, if s.b[267] { 1.0 } else { 0.0 });

        if (((!s.b[260]) && s.b[266]) && s.b[267]) {
            s.store_exp_mul(109, 145, 112);
        }

        if (((!s.b[260]) && s.b[266]) && (!s.b[267])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(65), s.ad_value(112))), A::mul(A::sub(s.ad_value(145), s.ad_value(65)), s.ad_value(112)), 1.0);
        }

        if ((!s.b[260]) && s.b[266]) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p59);
        }

        s.b[269] = (p.p88 > 0.0);
        s.store_scalar(269, if s.b[269] { 1.0 } else { 0.0 });

        if (((!s.b[260]) && s.b[266]) && s.b[269]) {
            s.store_sub_scaled_inputs(150, 31, -1.0, 143, 1.0);
            s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(32), s.ad_value(73));
        }

        s.b[270] = (s.v[150] < s.v[64]);
        s.store_scalar(270, if s.b[270] { 1.0 } else { 0.0 });

        if ((((!s.b[260]) && s.b[266]) && s.b[269]) && s.b[270]) {
            s.store_exp_mul(111, 150, 112);
        }

        if ((((!s.b[260]) && s.b[266]) && s.b[269]) && (!s.b[270])) {
            s.store_mul_offset_rhs_ad(111, A::exp(A::mul(s.ad_value(64), s.ad_value(112))), A::mul(A::sub(s.ad_value(150), s.ad_value(64)), s.ad_value(112)), 1.0);
        }

        if ((!s.b[260]) && (!s.b[266])) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p56);
        }

        s.b[271] = (s.v[143] < s.v[65]);
        s.store_scalar(271, if s.b[271] { 1.0 } else { 0.0 });

        if (((!s.b[260]) && (!s.b[266])) && s.b[271]) {
            s.store_exp_mul(109, 143, 112);
        }

        if (((!s.b[260]) && (!s.b[266])) && (!s.b[271])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(65), s.ad_value(112))), A::mul(A::sub(s.ad_value(143), s.ad_value(65)), s.ad_value(112)), 1.0);
        }

        if ((!s.b[260]) && (!s.b[266])) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p59);
        }

        s.b[274] = (p.p88 > 0.0);
        s.store_scalar(274, if s.b[274] { 1.0 } else { 0.0 });

        if (((!s.b[260]) && (!s.b[266])) && s.b[274]) {
            s.store_sub_scaled_inputs(150, 31, -1.0, 143, 1.0);
            s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(32), s.ad_value(73));
        }

        s.b[275] = (s.v[150] < s.v[64]);
        s.store_scalar(275, if s.b[275] { 1.0 } else { 0.0 });

        if ((((!s.b[260]) && (!s.b[266])) && s.b[274]) && s.b[275]) {
            s.store_exp_mul(111, 150, 112);
        }

        if ((((!s.b[260]) && (!s.b[266])) && s.b[274]) && (!s.b[275])) {
            s.store_mul_offset_rhs_ad(111, A::exp(A::mul(s.ad_value(64), s.ad_value(112))), A::mul(A::sub(s.ad_value(150), s.ad_value(64)), s.ad_value(112)), 1.0);
        }

        if ((!s.b[260]) && (!s.b[266])) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p56);
        }

        s.b[276] = (s.v[145] < s.v[65]);
        s.store_scalar(276, if s.b[276] { 1.0 } else { 0.0 });

        if (((!s.b[260]) && (!s.b[266])) && s.b[276]) {
            s.store_exp_mul(109, 145, 112);
        }

        if (((!s.b[260]) && (!s.b[266])) && (!s.b[276])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(65), s.ad_value(112))), A::mul(A::sub(s.ad_value(145), s.ad_value(65)), s.ad_value(112)), 1.0);
        }

        if ((!s.b[260]) && (!s.b[266])) {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p59);
        }

        s.b[278] = (p.p88 > 0.0);
        s.store_scalar(278, if s.b[278] { 1.0 } else { 0.0 });

        if (((!s.b[260]) && (!s.b[266])) && s.b[278]) {
            s.store_sub_scaled_inputs(150, 31, -1.0, 143, 1.0);
            s.store_div_from_scalar_mul_ad(112, 1.0, s.ad_value(32), s.ad_value(73));
        }

        s.b[279] = (s.v[150] < s.v[64]);
        s.store_scalar(279, if s.b[279] { 1.0 } else { 0.0 });

        if ((((!s.b[260]) && (!s.b[266])) && s.b[278]) && s.b[279]) {
            s.store_exp_mul(111, 150, 112);
        }

        if ((((!s.b[260]) && (!s.b[266])) && s.b[278]) && (!s.b[279])) {
            s.store_mul_offset_rhs_ad(111, A::exp(A::mul(s.ad_value(64), s.ad_value(112))), A::mul(A::sub(s.ad_value(150), s.ad_value(64)), s.ad_value(112)), 1.0);
        }

        s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p61);

        s.b[280] = (s.v[144] < s.v[67]);
        s.store_scalar(280, if s.b[280] { 1.0 } else { 0.0 });

        if s.b[280] {
            s.store_exp_mul(109, 144, 112);
        }

        if (!s.b[280]) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(67), s.ad_value(112))), A::mul(A::sub(s.ad_value(144), s.ad_value(67)), s.ad_value(112)), 1.0);
        }

        s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p63);

        s.b[282] = ((p.p64 > 0.0) || (p.p65 > 0.0));
        s.store_scalar(282, if s.b[282] { 1.0 } else { 0.0 });

        if s.b[282] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p61);
        }

        s.b[283] = (s.v[146] < s.v[69]);
        s.store_scalar(283, if s.b[283] { 1.0 } else { 0.0 });

        if (s.b[282] && s.b[283]) {
            s.store_exp_mul(109, 146, 112);
        }

        if (s.b[282] && (!s.b[283])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(69), s.ad_value(112))), A::mul(A::sub(s.ad_value(146), s.ad_value(69)), s.ad_value(112)), 1.0);
        }

        if s.b[282] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p63);
        }

        s.store_div(108, 144, 73);

        s.b[285] = (s.v[108] < s.v[113]);
        s.store_scalar(285, if s.b[285] { 1.0 } else { 0.0 });

        if s.b[285] {
            s.store_exp(109, 108);
        }

        if (!s.b[285]) {
            s.store_mul_offset_rhs_ad(109, A::exp(s.ad_value(113)), A::sub(s.ad_value(108), s.ad_value(113)), 1.0);
        }

        s.store_div(108, 148, 73);

        s.b[286] = (s.v[108] < s.v[113]);
        s.store_scalar(286, if s.b[286] { 1.0 } else { 0.0 });

        if s.b[286] {
            s.store_exp(111, 108);
        }

        if (!s.b[286]) {
            s.store_mul_offset_rhs_ad(111, A::exp(s.ad_value(113)), A::sub(s.ad_value(108), s.ad_value(113)), 1.0);
        }

        s.store_sqrt_offset_ad(103, A::mul(s.ad_value(33), s.ad_value(109)), 1.0);

        s.store_sqrt_offset_ad(104, A::mul(s.ad_value(33), s.ad_value(111)), 1.0);

        s.b[303] = ((p.p66 > 0.0) || (p.p68 > 0.0));
        s.store_scalar(303, if s.b[303] { 1.0 } else { 0.0 });

        if s.b[303] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p67);
        }

        s.b[304] = (s.v[147] < s.v[71]);
        s.store_scalar(304, if s.b[304] { 1.0 } else { 0.0 });

        if (s.b[303] && s.b[304]) {
            s.store_exp_mul(109, 147, 112);
        }

        if (s.b[303] && (!s.b[304])) {
            s.store_mul_offset_rhs_ad(109, A::exp(A::mul(s.ad_value(71), s.ad_value(112))), A::mul(A::sub(s.ad_value(147), s.ad_value(71)), s.ad_value(112)), 1.0);
        }

        if s.b[303] {
            s.store_div_from_scalar_scaled_input(112, 1.0, 73, p.p69);
        }

        s.b[306] = (p.p49 > 0.0);
        s.store_scalar(306, if s.b[306] { 1.0 } else { 0.0 });

        if s.b[306] {
            s.store_scale(307, 22, (-p.p34));
        }

        s.b[318] = (p.p52 <= 0.0);
        s.store_scalar(318, if s.b[318] { 1.0 } else { 0.0 });

        if (s.b[306] && s.b[318]) {
            s.store_add(308, 147, 307);
        }

        s.b[319] = (s.v[308] > 0.0);
        s.store_scalar(319, if s.b[319] { 1.0 } else { 0.0 });

        if ((s.b[306] && s.b[318]) && s.b[319]) {
            s.store_scalar(309, (((1.0 - p.p34)) as f64).powf((-p.p51)));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(310, 22, 1.0, 309, (1.0 - p.p34), 1.0 / ((1.0 - p.p51)));
            s.store_mul_ad_product_lhs_mixed_ia(311, 308, A::offset(A::div_scaled_inputs(s.ad_value(308), (0.5 * p.p51), s.ad_value(22), (1.0 - p.p34)), 1.0), 309);
        }

        if ((s.b[306] && s.b[318]) && (!s.b[319])) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(310, 22, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(147), s.ad_value(22))), (1.0 - p.p51)), 1.0 / ((1.0 - p.p51)));
            s.store_scalar(311, 0.0);
        }

        if (s.b[306] && s.b[318]) {
            s.store_add(118, 310, 311);
        }

        if (s.b[306] && (!s.b[318])) {
            s.store_sqrt_square_offset(312, 307, ((4.0 * p.p52) * p.p52));
            s.store_scaled_add(313, 307, 312, (-0.5));
            s.store_mul_scaled_powf_ad_rhs(314, 22, (-1.0 / ((1.0 - p.p51))), A::sub_from_scalar(1.0, A::div(s.ad_value(313), s.ad_value(22))), (1.0 - p.p51));
            s.store_add(315, 147, 307);
            s.store_sqrt_square_offset(316, 315, ((4.0 * p.p52) * p.p52));
            s.store_add_scaled_inputs3_indices(317, 315, 0.5, 316, (-0.5), 307, -1.0);
            s.store_mul_scaled_powf_ad_rhs(310, 22, (-1.0 / ((1.0 - p.p51))), A::sub_from_scalar(1.0, A::div(s.ad_value(317), s.ad_value(22))), (1.0 - p.p51));
            s.store_sub_ad_lhs(118, A::add_scaled_offset_product_rhs(s.ad_value(310), 1.0, A::add_scaled_inputs3(s.ad_value(147), 1.0, s.ad_value(317), (-1.0), s.ad_value(313), 1.0), A::div_scaled_inputs3(s.ad_value(147), (0.5 * p.p51), s.ad_value(317), ((-1.0) * (0.5 * p.p51)), s.ad_value(313), (0.5 * p.p51), s.ad_value(22), (1.0 - p.p34)), 1.0, (((1.0 - p.p34)) as f64).powf((-p.p51))), 314);
        }

        if (!s.b[306]) {
            s.store_scalar(118, 0.0);
        }

        s.store_scale(320, 20, (-p.p34));

        s.b[331] = (p.p39 <= 0.0);
        s.store_scalar(331, if s.b[331] { 1.0 } else { 0.0 });

        if s.b[331] {
            s.store_add(321, 145, 320);
        }

        s.b[332] = (s.v[321] > 0.0);
        s.store_scalar(332, if s.b[332] { 1.0 } else { 0.0 });

        if (s.b[331] && s.b[332]) {
            s.store_scalar(322, (((1.0 - p.p34)) as f64).powf((-p.p38)));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(323, 20, 1.0, 322, (1.0 - p.p34), 1.0 / ((1.0 - p.p38)));
            s.store_mul_ad_product_lhs_mixed_ia(324, 321, A::offset(A::div_scaled_inputs(s.ad_value(321), (0.5 * p.p38), s.ad_value(20), (1.0 - p.p34)), 1.0), 322);
        }

        if (s.b[331] && (!s.b[332])) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(323, 20, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(145), s.ad_value(20))), (1.0 - p.p38)), 1.0 / ((1.0 - p.p38)));
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[331] && (!s.b[332])) {
            s.store_scalar(324, 0.0);
        }

        if s.b[331] {
            s.store_add(115, 323, 324);
        }

        if (!s.b[331]) {
            s.store_sqrt_square_offset(325, 320, ((4.0 * p.p39) * p.p39));
            s.store_scaled_add(326, 320, 325, (-0.5));
            s.store_mul_scaled_powf_ad_rhs(327, 20, (-1.0 / ((1.0 - p.p38))), A::sub_from_scalar(1.0, A::div(s.ad_value(326), s.ad_value(20))), (1.0 - p.p38));
            s.store_add(328, 145, 320);
            s.store_sqrt_square_offset(329, 328, ((4.0 * p.p39) * p.p39));
            s.store_add_scaled_inputs3_indices(330, 328, 0.5, 329, (-0.5), 320, -1.0);
            s.store_mul_scaled_powf_ad_rhs(323, 20, (-1.0 / ((1.0 - p.p38))), A::sub_from_scalar(1.0, A::div(s.ad_value(330), s.ad_value(20))), (1.0 - p.p38));
            s.store_sub_ad_lhs(115, A::add_scaled_offset_product_rhs(s.ad_value(323), 1.0, A::add_scaled_inputs3(s.ad_value(145), 1.0, s.ad_value(330), (-1.0), s.ad_value(326), 1.0), A::div_scaled_inputs3(s.ad_value(145), (0.5 * p.p38), s.ad_value(330), ((-1.0) * (0.5 * p.p38)), s.ad_value(326), (0.5 * p.p38), s.ad_value(20), (1.0 - p.p34)), 1.0, (((1.0 - p.p34)) as f64).powf((-p.p38))), 327);
        }

        s.store_scale(333, 21, (-p.p34));

        s.b[354] = (p.p44 <= 0.0);
        s.store_scalar(354, if s.b[354] { 1.0 } else { 0.0 });

        if s.b[354] {
            s.store_add(334, 146, 333);
        }

        s.b[355] = (s.v[334] > 0.0);
        s.store_scalar(355, if s.b[355] { 1.0 } else { 0.0 });

        if (s.b[354] && s.b[355]) {
            s.store_scalar(335, (((1.0 - p.p34)) as f64).powf(((-1.0) - p.p43)));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(336, 21, 1.0, 335, ((1.0 - p.p34) * (1.0 - p.p34)), 1.0 / ((1.0 - p.p43)));
            s.store_mul_ad_product_lhs_mixed_ia(337, 334, A::offset(A::div_scaled_inputs(s.ad_value(334), (0.5 * p.p43), s.ad_value(21), 1.0), (1.0 - p.p34)), 335);
        }

        s.b[356] = ((p.p45 > 0.0) && (s.v[146] < (-p.p45)));
        s.store_scalar(356, if s.b[356] { 1.0 } else { 0.0 });

        if ((s.b[354] && (!s.b[355])) && s.b[356]) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(336, 21, 1.0, A::mul_sub_from_scalar_rhs(A::powf(A::offset(A::div_from_scalar(p.p45, s.ad_value(21)), 1.0), (1.0 - p.p43)), 1.0, A::div_scaled_offset_numerator(s.ad_value(146), (1.0 - p.p43), (p.p45 * (1.0 - p.p43)), A::offset(s.ad_value(21), p.p45), 1.0)), 1.0 / ((1.0 - p.p43)));
        }

        if ((s.b[354] && (!s.b[355])) && (!s.b[356])) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(336, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(146), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
        }

        if (s.b[354] && (!s.b[355])) {
            s.store_scalar(337, 0.0);
        }

        if s.b[354] {
            s.store_add(117, 336, 337);
        }

        s.b[357] = ((p.p45 > 0.0) && (p.p46 > 0.0));
        s.store_scalar(357, if s.b[357] { 1.0 } else { 0.0 });

        if ((!s.b[354]) && s.b[357]) {
            s.store_div_scaled_offset_numerator(338, s.ad_value(333), 1.0, p.p45, A::sub_from_scalar(p.p45, s.ad_value(333)), 1.0);
            s.store_div_scaled_inputs_mixed_ia(339, 338, 2.0, A::add(A::sqrt_square_offset(A::offset(s.ad_value(338), (-1.0)), ((4.0 * p.p44) * p.p44)), A::sqrt_square_offset(A::offset(s.ad_value(338), 1.0), ((4.0 * p.p46) * p.p46))), 1.0);
            s.store_scaled_sub_ad_lhs(340, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(339), p.p45, s.ad_value(333)), (-p.p45)), 333, 0.5);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(341, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(340), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
            s.store_div_scaled_inputs2_mixed_aia(342, A::scale_offset(s.ad_value(146), 2.0, p.p45), 1.0, 333, 1.0, A::sub_from_scalar(p.p45, s.ad_value(333)), 1.0);
            s.store_div_scaled_inputs_mixed_ia(343, 342, 2.0, A::add(A::sqrt_square_offset(A::offset(s.ad_value(342), (-1.0)), ((4.0 * p.p44) * p.p44)), A::sqrt_square_offset(A::offset(s.ad_value(342), 1.0), ((4.0 * p.p46) * p.p46))), 1.0);
            s.store_scaled_sub_ad_lhs(344, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(343), p.p45, s.ad_value(333)), (-p.p45)), 333, 0.5);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(336, 21, 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(344), s.ad_value(21))), (1.0 - p.p43)), 1.0 / ((1.0 - p.p43)));
            s.store_scaled_offset(345, 343, 1.0, 0.5);
            s.store_powf_ad(346, A::offset(A::div_from_scalar(p.p45, s.ad_value(21)), 1.0), (-p.p43));
            s.store_powf_ad(347, A::offset(A::div(s.ad_value(333), s.ad_value(21)), 1.0), (-p.p43));
            s.store_add_scaled_product_value_ad(348, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(345), s.ad_value(346)), 1.0, 345, 347, 1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(349, 348, s.ad_value(146), 1.0, s.ad_value(344), (-1.0), s.ad_value(340), 1.0, 0.0);
            s.store_add_scaled_inputs3_indices(117, 349, 1.0, 336, 1.0, 341, -1.0);
        }

        if ((!s.b[354]) && (!s.b[357])) {
            s.store_sqrt_square_offset(350, 333, ((4.0 * p.p44) * p.p44));
            s.store_scaled_add(340, 333, 350, (-0.5));
            s.store_mul_scaled_powf_ad_rhs(351, 21, (-1.0 / ((1.0 - p.p43))), A::sub_from_scalar(1.0, A::div(s.ad_value(340), s.ad_value(21))), (1.0 - p.p43));
            s.store_add(352, 146, 333);
            s.store_sqrt_square_offset(353, 352, ((4.0 * p.p44) * p.p44));
            s.store_add_scaled_inputs3_indices(344, 352, 0.5, 353, (-0.5), 333, -1.0);
            s.store_mul_scaled_powf_ad_rhs(336, 21, (-1.0 / ((1.0 - p.p43))), A::sub_from_scalar(1.0, A::div(s.ad_value(344), s.ad_value(21))), (1.0 - p.p43));
            s.store_sub_add_scaled_inputs4_lhs_indices(117, 336, 1.0, 146, (((1.0 - p.p34)) as f64).powf((-p.p43)), 344, ((-1.0) * (((1.0 - p.p34)) as f64).powf((-p.p43))), 340, (((1.0 - p.p34)) as f64).powf((-p.p43)), 351);
        }

        s.b[119] = (s.v[74] > 0.0);
        s.store_scalar(119, if s.b[119] { 1.0 } else { 0.0 });

        s.store_scaled_mul(120, 74, 51, s.v[119]);

        s.store_div_scaled_value_offset_denominator(121, s.ad_value(120), 1.0, s.ad_value(120), 1.0, 1.0);

        s.store_scaled_mul(108, 144, 50, 0.6944444444444444);

        s.b[358] = (s.v[108] < s.v[113]);
        s.store_scalar(358, if s.b[358] { 1.0 } else { 0.0 });

        if s.b[358] {
            s.store_exp(109, 108);
        }

        if (!s.b[358]) {
            s.store_mul_offset_rhs_ad(109, A::exp(s.ad_value(113)), A::sub(s.ad_value(108), s.ad_value(113)), 1.0);
        }

        s.store_scaled_mul_scale_offset_rhs_ad(122, A::offset(A::mul_scaled_output(s.ad_value(109), A::add(s.ad_value(52), A::square(s.ad_value(121))), (p.p78 * s.v[119])), 1.0), 79, p.p77, 1.0, p.p76);

        s.store_add_scaled_product_value_ad(123, A::div_scaled_product(s.ad_value(122), s.ad_value(74), 1.0, s.ad_value(81), 1.0), 1.0, 23, 114, p.p55);

        s.store_scaled_mul(124, 23, 115, (1.0 - p.p55));

        s.store_add_scaled_ad_lhs(125, A::add_scaled_product(s.ad_value(75), p.p81, s.ad_value(24), s.ad_value(116), 1.0), 103, p.p47);

        s.store_scale(126, 104, p.p47);

        s.store_add_scaled_product_indices(127, 82, p.p81, 25, 117, 1.0);

        s.store_add_scaled_product_indices(128, 147, p.p53, 26, 118, 1.0);

        s.store_scale(142, 138, p.p102);

        s.store_scaled_mul(123, 162, 123, 1.0);

        s.store_scaled_mul(124, 162, 124, 1.0);

        s.store_scaled_mul(125, 162, 125, 1.0);

        s.store_scaled_mul(126, 162, 126, 1.0);

        s.store_scaled_mul(127, 162, 127, 1.0);

        s.store_scaled_mul(128, 162, 128, 1.0);

    }

    pub(super) fn stamp_transient_equations_block_0(
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
        var_ibc: f64,
        var_ibc_dn0: f64,
        var_ibc_dn1: f64,
        var_ibc_dn10: f64,
        var_ibc_dn11: f64,
        var_ibc_dn12: f64,
        var_ibc_dn13: f64,
        var_ibc_dn2: f64,
        var_ibc_dn3: f64,
        var_ibc_dn4: f64,
        var_ibc_dn5: f64,
        var_ibc_dn6: f64,
        var_ibc_dn7: f64,
        var_ibc_dn8: f64,
        var_ibc_dn9: f64,
        var_ibcp: f64,
        var_ibcp_dn0: f64,
        var_ibcp_dn1: f64,
        var_ibcp_dn10: f64,
        var_ibcp_dn11: f64,
        var_ibcp_dn12: f64,
        var_ibcp_dn13: f64,
        var_ibcp_dn2: f64,
        var_ibcp_dn3: f64,
        var_ibcp_dn4: f64,
        var_ibcp_dn5: f64,
        var_ibcp_dn6: f64,
        var_ibcp_dn7: f64,
        var_ibcp_dn8: f64,
        var_ibcp_dn9: f64,
        var_ibe: f64,
        var_ibe_dn0: f64,
        var_ibe_dn1: f64,
        var_ibe_dn10: f64,
        var_ibe_dn11: f64,
        var_ibe_dn12: f64,
        var_ibe_dn13: f64,
        var_ibe_dn2: f64,
        var_ibe_dn3: f64,
        var_ibe_dn4: f64,
        var_ibe_dn5: f64,
        var_ibe_dn6: f64,
        var_ibe_dn7: f64,
        var_ibe_dn8: f64,
        var_ibe_dn9: f64,
        var_ibep: f64,
        var_ibep_dn0: f64,
        var_ibep_dn1: f64,
        var_ibep_dn10: f64,
        var_ibep_dn11: f64,
        var_ibep_dn12: f64,
        var_ibep_dn13: f64,
        var_ibep_dn2: f64,
        var_ibep_dn3: f64,
        var_ibep_dn4: f64,
        var_ibep_dn5: f64,
        var_ibep_dn6: f64,
        var_ibep_dn7: f64,
        var_ibep_dn8: f64,
        var_ibep_dn9: f64,
        var_ibex: f64,
        var_ibex_dn0: f64,
        var_ibex_dn1: f64,
        var_ibex_dn10: f64,
        var_ibex_dn11: f64,
        var_ibex_dn12: f64,
        var_ibex_dn13: f64,
        var_ibex_dn2: f64,
        var_ibex_dn3: f64,
        var_ibex_dn4: f64,
        var_ibex_dn5: f64,
        var_ibex_dn6: f64,
        var_ibex_dn7: f64,
        var_ibex_dn8: f64,
        var_ibex_dn9: f64,
        var_iccp: f64,
        var_iccp_dn0: f64,
        var_iccp_dn1: f64,
        var_iccp_dn10: f64,
        var_iccp_dn11: f64,
        var_iccp_dn12: f64,
        var_iccp_dn13: f64,
        var_iccp_dn2: f64,
        var_iccp_dn3: f64,
        var_iccp_dn4: f64,
        var_iccp_dn5: f64,
        var_iccp_dn6: f64,
        var_iccp_dn7: f64,
        var_iccp_dn8: f64,
        var_iccp_dn9: f64,
        var_igcx: f64,
        var_igcx_dn0: f64,
        var_igcx_dn1: f64,
        var_igcx_dn10: f64,
        var_igcx_dn11: f64,
        var_igcx_dn12: f64,
        var_igcx_dn13: f64,
        var_igcx_dn2: f64,
        var_igcx_dn3: f64,
        var_igcx_dn4: f64,
        var_igcx_dn5: f64,
        var_igcx_dn6: f64,
        var_igcx_dn7: f64,
        var_igcx_dn8: f64,
        var_igcx_dn9: f64,
        var_irbi: f64,
        var_irbi_dn0: f64,
        var_irbi_dn1: f64,
        var_irbi_dn10: f64,
        var_irbi_dn11: f64,
        var_irbi_dn12: f64,
        var_irbi_dn13: f64,
        var_irbi_dn2: f64,
        var_irbi_dn3: f64,
        var_irbi_dn4: f64,
        var_irbi_dn5: f64,
        var_irbi_dn6: f64,
        var_irbi_dn7: f64,
        var_irbi_dn8: f64,
        var_irbi_dn9: f64,
        var_irbp: f64,
        var_irbp_dn0: f64,
        var_irbp_dn1: f64,
        var_irbp_dn10: f64,
        var_irbp_dn11: f64,
        var_irbp_dn12: f64,
        var_irbp_dn13: f64,
        var_irbp_dn2: f64,
        var_irbp_dn3: f64,
        var_irbp_dn4: f64,
        var_irbp_dn5: f64,
        var_irbp_dn6: f64,
        var_irbp_dn7: f64,
        var_irbp_dn8: f64,
        var_irbp_dn9: f64,
        var_irbx: f64,
        var_irbx_dn0: f64,
        var_irbx_dn1: f64,
        var_irbx_dn10: f64,
        var_irbx_dn11: f64,
        var_irbx_dn12: f64,
        var_irbx_dn13: f64,
        var_irbx_dn2: f64,
        var_irbx_dn3: f64,
        var_irbx_dn4: f64,
        var_irbx_dn5: f64,
        var_irbx_dn6: f64,
        var_irbx_dn7: f64,
        var_irbx_dn8: f64,
        var_irbx_dn9: f64,
        var_irci: f64,
        var_irci_dn0: f64,
        var_irci_dn1: f64,
        var_irci_dn10: f64,
        var_irci_dn11: f64,
        var_irci_dn12: f64,
        var_irci_dn13: f64,
        var_irci_dn2: f64,
        var_irci_dn3: f64,
        var_irci_dn4: f64,
        var_irci_dn5: f64,
        var_irci_dn6: f64,
        var_irci_dn7: f64,
        var_irci_dn8: f64,
        var_irci_dn9: f64,
        var_ircx: f64,
        var_ircx_dn0: f64,
        var_ircx_dn1: f64,
        var_ircx_dn10: f64,
        var_ircx_dn11: f64,
        var_ircx_dn12: f64,
        var_ircx_dn13: f64,
        var_ircx_dn2: f64,
        var_ircx_dn3: f64,
        var_ircx_dn4: f64,
        var_ircx_dn5: f64,
        var_ircx_dn6: f64,
        var_ircx_dn7: f64,
        var_ircx_dn8: f64,
        var_ircx_dn9: f64,
        var_ire: f64,
        var_ire_dn0: f64,
        var_ire_dn1: f64,
        var_ire_dn10: f64,
        var_ire_dn11: f64,
        var_ire_dn12: f64,
        var_ire_dn13: f64,
        var_ire_dn2: f64,
        var_ire_dn3: f64,
        var_ire_dn4: f64,
        var_ire_dn5: f64,
        var_ire_dn6: f64,
        var_ire_dn7: f64,
        var_ire_dn8: f64,
        var_ire_dn9: f64,
        var_irs: f64,
        var_irs_dn0: f64,
        var_irs_dn1: f64,
        var_irs_dn10: f64,
        var_irs_dn11: f64,
        var_irs_dn12: f64,
        var_irs_dn13: f64,
        var_irs_dn2: f64,
        var_irs_dn3: f64,
        var_irs_dn4: f64,
        var_irs_dn5: f64,
        var_irs_dn6: f64,
        var_irs_dn7: f64,
        var_irs_dn8: f64,
        var_irs_dn9: f64,
        var_irth: f64,
        var_irth_dn0: f64,
        var_irth_dn1: f64,
        var_irth_dn10: f64,
        var_irth_dn11: f64,
        var_irth_dn12: f64,
        var_irth_dn13: f64,
        var_irth_dn2: f64,
        var_irth_dn3: f64,
        var_irth_dn4: f64,
        var_irth_dn5: f64,
        var_irth_dn6: f64,
        var_irth_dn7: f64,
        var_irth_dn8: f64,
        var_irth_dn9: f64,
        var_ith: f64,
        var_ith_dn0: f64,
        var_ith_dn1: f64,
        var_ith_dn10: f64,
        var_ith_dn11: f64,
        var_ith_dn12: f64,
        var_ith_dn13: f64,
        var_ith_dn2: f64,
        var_ith_dn3: f64,
        var_ith_dn4: f64,
        var_ith_dn5: f64,
        var_ith_dn6: f64,
        var_ith_dn7: f64,
        var_ith_dn8: f64,
        var_ith_dn9: f64,
        var_itzr: f64,
        var_itzr_dn0: f64,
        var_itzr_dn1: f64,
        var_itzr_dn10: f64,
        var_itzr_dn11: f64,
        var_itzr_dn12: f64,
        var_itzr_dn13: f64,
        var_itzr_dn2: f64,
        var_itzr_dn3: f64,
        var_itzr_dn4: f64,
        var_itzr_dn5: f64,
        var_itzr_dn6: f64,
        var_itzr_dn7: f64,
        var_itzr_dn8: f64,
        var_itzr_dn9: f64,
        var_ixf1: f64,
        var_ixf1_dn0: f64,
        var_ixf1_dn1: f64,
        var_ixf1_dn10: f64,
        var_ixf1_dn11: f64,
        var_ixf1_dn12: f64,
        var_ixf1_dn13: f64,
        var_ixf1_dn2: f64,
        var_ixf1_dn3: f64,
        var_ixf1_dn4: f64,
        var_ixf1_dn5: f64,
        var_ixf1_dn6: f64,
        var_ixf1_dn7: f64,
        var_ixf1_dn8: f64,
        var_ixf1_dn9: f64,
        var_qbc: f64,
        var_qbc_dn0: f64,
        var_qbc_dn1: f64,
        var_qbc_dn10: f64,
        var_qbc_dn11: f64,
        var_qbc_dn12: f64,
        var_qbc_dn13: f64,
        var_qbc_dn2: f64,
        var_qbc_dn3: f64,
        var_qbc_dn4: f64,
        var_qbc_dn5: f64,
        var_qbc_dn6: f64,
        var_qbc_dn7: f64,
        var_qbc_dn8: f64,
        var_qbc_dn9: f64,
        var_qbcp: f64,
        var_qbcp_dn0: f64,
        var_qbcp_dn1: f64,
        var_qbcp_dn10: f64,
        var_qbcp_dn11: f64,
        var_qbcp_dn12: f64,
        var_qbcp_dn13: f64,
        var_qbcp_dn2: f64,
        var_qbcp_dn3: f64,
        var_qbcp_dn4: f64,
        var_qbcp_dn5: f64,
        var_qbcp_dn6: f64,
        var_qbcp_dn7: f64,
        var_qbcp_dn8: f64,
        var_qbcp_dn9: f64,
        var_qbcx: f64,
        var_qbcx_dn0: f64,
        var_qbcx_dn1: f64,
        var_qbcx_dn10: f64,
        var_qbcx_dn11: f64,
        var_qbcx_dn12: f64,
        var_qbcx_dn13: f64,
        var_qbcx_dn2: f64,
        var_qbcx_dn3: f64,
        var_qbcx_dn4: f64,
        var_qbcx_dn5: f64,
        var_qbcx_dn6: f64,
        var_qbcx_dn7: f64,
        var_qbcx_dn8: f64,
        var_qbcx_dn9: f64,
        var_qbe: f64,
        var_qbe_dn0: f64,
        var_qbe_dn1: f64,
        var_qbe_dn10: f64,
        var_qbe_dn11: f64,
        var_qbe_dn12: f64,
        var_qbe_dn13: f64,
        var_qbe_dn2: f64,
        var_qbe_dn3: f64,
        var_qbe_dn4: f64,
        var_qbe_dn5: f64,
        var_qbe_dn6: f64,
        var_qbe_dn7: f64,
        var_qbe_dn8: f64,
        var_qbe_dn9: f64,
        var_qbep: f64,
        var_qbep_dn0: f64,
        var_qbep_dn1: f64,
        var_qbep_dn10: f64,
        var_qbep_dn11: f64,
        var_qbep_dn12: f64,
        var_qbep_dn13: f64,
        var_qbep_dn2: f64,
        var_qbep_dn3: f64,
        var_qbep_dn4: f64,
        var_qbep_dn5: f64,
        var_qbep_dn6: f64,
        var_qbep_dn7: f64,
        var_qbep_dn8: f64,
        var_qbep_dn9: f64,
        var_qbex: f64,
        var_qbex_dn0: f64,
        var_qbex_dn1: f64,
        var_qbex_dn10: f64,
        var_qbex_dn11: f64,
        var_qbex_dn12: f64,
        var_qbex_dn13: f64,
        var_qbex_dn2: f64,
        var_qbex_dn3: f64,
        var_qbex_dn4: f64,
        var_qbex_dn5: f64,
        var_qbex_dn6: f64,
        var_qbex_dn7: f64,
        var_qbex_dn8: f64,
        var_qbex_dn9: f64,
        var_qcth: f64,
        var_qcth_dn0: f64,
        var_qcth_dn1: f64,
        var_qcth_dn10: f64,
        var_qcth_dn11: f64,
        var_qcth_dn12: f64,
        var_qcth_dn13: f64,
        var_qcth_dn2: f64,
        var_qcth_dn3: f64,
        var_qcth_dn4: f64,
        var_qcth_dn5: f64,
        var_qcth_dn6: f64,
        var_qcth_dn7: f64,
        var_qcth_dn8: f64,
        var_qcth_dn9: f64,
    ) {
        let eq0_value: f64 = var_ibe;
        let eq0_node_derivatives: [f64; 14] = [var_ibe_dn0, var_ibe_dn1, var_ibe_dn2, var_ibe_dn3, var_ibe_dn4, var_ibe_dn5, var_ibe_dn6, var_ibe_dn7, var_ibe_dn8, var_ibe_dn9, var_ibe_dn10, var_ibe_dn11, var_ibe_dn12, var_ibe_dn13];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &[],
            multiplicity,
        );
        let eq1_value: f64 = var_ibex;
        let eq1_node_derivatives: [f64; 14] = [var_ibex_dn0, var_ibex_dn1, var_ibex_dn2, var_ibex_dn3, var_ibex_dn4, var_ibex_dn5, var_ibex_dn6, var_ibex_dn7, var_ibex_dn8, var_ibex_dn9, var_ibex_dn10, var_ibex_dn11, var_ibex_dn12, var_ibex_dn13];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &[],
            multiplicity,
        );
        let eq3_value: f64 = var_itzr;
        let eq3_node_derivatives: [f64; 14] = [var_itzr_dn0, var_itzr_dn1, var_itzr_dn2, var_itzr_dn3, var_itzr_dn4, var_itzr_dn5, var_itzr_dn6, var_itzr_dn7, var_itzr_dn8, var_itzr_dn9, var_itzr_dn10, var_itzr_dn11, var_itzr_dn12, var_itzr_dn13];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &[],
            multiplicity,
        );
        let eq4_value: f64 = var_ibc;
        let eq4_node_derivatives: [f64; 14] = [var_ibc_dn0, var_ibc_dn1, var_ibc_dn2, var_ibc_dn3, var_ibc_dn4, var_ibc_dn5, var_ibc_dn6, var_ibc_dn7, var_ibc_dn8, var_ibc_dn9, var_ibc_dn10, var_ibc_dn11, var_ibc_dn12, var_ibc_dn13];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &[],
            multiplicity,
        );
        let eq5_value: f64 = var_igcx;
        let eq5_node_derivatives: [f64; 14] = [var_igcx_dn0, var_igcx_dn1, var_igcx_dn2, var_igcx_dn3, var_igcx_dn4, var_igcx_dn5, var_igcx_dn6, var_igcx_dn7, var_igcx_dn8, var_igcx_dn9, var_igcx_dn10, var_igcx_dn11, var_igcx_dn12, var_igcx_dn13];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &[],
            multiplicity,
        );
        let eq6_value: f64 = var_ibep;
        let eq6_node_derivatives: [f64; 14] = [var_ibep_dn0, var_ibep_dn1, var_ibep_dn2, var_ibep_dn3, var_ibep_dn4, var_ibep_dn5, var_ibep_dn6, var_ibep_dn7, var_ibep_dn8, var_ibep_dn9, var_ibep_dn10, var_ibep_dn11, var_ibep_dn12, var_ibep_dn13];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &[],
            multiplicity,
        );
        let eq7_value: f64 = var_ircx;
        let eq7_node_derivatives: [f64; 14] = [var_ircx_dn0, var_ircx_dn1, var_ircx_dn2, var_ircx_dn3, var_ircx_dn4, var_ircx_dn5, var_ircx_dn6, var_ircx_dn7, var_ircx_dn8, var_ircx_dn9, var_ircx_dn10, var_ircx_dn11, var_ircx_dn12, var_ircx_dn13];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(5),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &[],
            multiplicity,
        );
        let eq8_value: f64 = var_irci;
        let eq8_node_derivatives: [f64; 14] = [var_irci_dn0, var_irci_dn1, var_irci_dn2, var_irci_dn3, var_irci_dn4, var_irci_dn5, var_irci_dn6, var_irci_dn7, var_irci_dn8, var_irci_dn9, var_irci_dn10, var_irci_dn11, var_irci_dn12, var_irci_dn13];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &[],
            multiplicity,
        );
        let eq9_value: f64 = var_irbx;
        let eq9_node_derivatives: [f64; 14] = [var_irbx_dn0, var_irbx_dn1, var_irbx_dn2, var_irbx_dn3, var_irbx_dn4, var_irbx_dn5, var_irbx_dn6, var_irbx_dn7, var_irbx_dn8, var_irbx_dn9, var_irbx_dn10, var_irbx_dn11, var_irbx_dn12, var_irbx_dn13];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(7),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &[],
            multiplicity,
        );
        let eq10_value: f64 = var_irbi;
        let eq10_node_derivatives: [f64; 14] = [var_irbi_dn0, var_irbi_dn1, var_irbi_dn2, var_irbi_dn3, var_irbi_dn4, var_irbi_dn5, var_irbi_dn6, var_irbi_dn7, var_irbi_dn8, var_irbi_dn9, var_irbi_dn10, var_irbi_dn11, var_irbi_dn12, var_irbi_dn13];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &[],
            multiplicity,
        );
        let eq11_value: f64 = var_ire;
        let eq11_node_derivatives: [f64; 14] = [var_ire_dn0, var_ire_dn1, var_ire_dn2, var_ire_dn3, var_ire_dn4, var_ire_dn5, var_ire_dn6, var_ire_dn7, var_ire_dn8, var_ire_dn9, var_ire_dn10, var_ire_dn11, var_ire_dn12, var_ire_dn13];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(9),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &[],
            multiplicity,
        );
        let eq12_value: f64 = var_irbp;
        let eq12_node_derivatives: [f64; 14] = [var_irbp_dn0, var_irbp_dn1, var_irbp_dn2, var_irbp_dn3, var_irbp_dn4, var_irbp_dn5, var_irbp_dn6, var_irbp_dn7, var_irbp_dn8, var_irbp_dn9, var_irbp_dn10, var_irbp_dn11, var_irbp_dn12, var_irbp_dn13];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &[],
            multiplicity,
        );
        let eq13_value: f64 = var_ibcp;
        let eq13_node_derivatives: [f64; 14] = [var_ibcp_dn0, var_ibcp_dn1, var_ibcp_dn2, var_ibcp_dn3, var_ibcp_dn4, var_ibcp_dn5, var_ibcp_dn6, var_ibcp_dn7, var_ibcp_dn8, var_ibcp_dn9, var_ibcp_dn10, var_ibcp_dn11, var_ibcp_dn12, var_ibcp_dn13];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(10),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &[],
            multiplicity,
        );
        let eq14_value: f64 = var_iccp;
        let eq14_node_derivatives: [f64; 14] = [var_iccp_dn0, var_iccp_dn1, var_iccp_dn2, var_iccp_dn3, var_iccp_dn4, var_iccp_dn5, var_iccp_dn6, var_iccp_dn7, var_iccp_dn8, var_iccp_dn9, var_iccp_dn10, var_iccp_dn11, var_iccp_dn12, var_iccp_dn13];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &[],
            multiplicity,
        );
        let eq15_value: f64 = var_irs;
        let eq15_node_derivatives: [f64; 14] = [var_irs_dn0, var_irs_dn1, var_irs_dn2, var_irs_dn3, var_irs_dn4, var_irs_dn5, var_irs_dn6, var_irs_dn7, var_irs_dn8, var_irs_dn9, var_irs_dn10, var_irs_dn11, var_irs_dn12, var_irs_dn13];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(11),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &[],
            multiplicity,
        );
        let eq16_value: f64 = var_ixf1;
        let eq16_node_derivatives: [f64; 14] = [var_ixf1_dn0, var_ixf1_dn1, var_ixf1_dn2, var_ixf1_dn3, var_ixf1_dn4, var_ixf1_dn5, var_ixf1_dn6, var_ixf1_dn7, var_ixf1_dn8, var_ixf1_dn9, var_ixf1_dn10, var_ixf1_dn11, var_ixf1_dn12, var_ixf1_dn13];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &[],
            multiplicity,
        );
        let eq18_value: f64 = var_irth;
        let eq18_node_derivatives: [f64; 14] = [var_irth_dn0, var_irth_dn1, var_irth_dn2, var_irth_dn3, var_irth_dn4, var_irth_dn5, var_irth_dn6, var_irth_dn7, var_irth_dn8, var_irth_dn9, var_irth_dn10, var_irth_dn11, var_irth_dn12, var_irth_dn13];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &[],
            multiplicity,
        );
        let eq19_value: f64 = var_ith;
        let eq19_node_derivatives: [f64; 14] = [var_ith_dn0, var_ith_dn1, var_ith_dn2, var_ith_dn3, var_ith_dn4, var_ith_dn5, var_ith_dn6, var_ith_dn7, var_ith_dn8, var_ith_dn9, var_ith_dn10, var_ith_dn11, var_ith_dn12, var_ith_dn13];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &[],
            multiplicity,
        );
        let eq20_e159: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, var_qbe);
        let eq20_value: f64 = eq20_e159;
        let eq20_node_derivatives: [f64; 14] = [(var_qbe_dn0 * ddt_scale), (var_qbe_dn1 * ddt_scale), (var_qbe_dn2 * ddt_scale), (var_qbe_dn3 * ddt_scale), (var_qbe_dn4 * ddt_scale), (var_qbe_dn5 * ddt_scale), (var_qbe_dn6 * ddt_scale), (var_qbe_dn7 * ddt_scale), (var_qbe_dn8 * ddt_scale), (var_qbe_dn9 * ddt_scale), (var_qbe_dn10 * ddt_scale), (var_qbe_dn11 * ddt_scale), (var_qbe_dn12 * ddt_scale), (var_qbe_dn13 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &[],
            multiplicity,
        );
        let eq21_e161: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, var_qbex);
        let eq21_value: f64 = eq21_e161;
        let eq21_node_derivatives: [f64; 14] = [(var_qbex_dn0 * ddt_scale), (var_qbex_dn1 * ddt_scale), (var_qbex_dn2 * ddt_scale), (var_qbex_dn3 * ddt_scale), (var_qbex_dn4 * ddt_scale), (var_qbex_dn5 * ddt_scale), (var_qbex_dn6 * ddt_scale), (var_qbex_dn7 * ddt_scale), (var_qbex_dn8 * ddt_scale), (var_qbex_dn9 * ddt_scale), (var_qbex_dn10 * ddt_scale), (var_qbex_dn11 * ddt_scale), (var_qbex_dn12 * ddt_scale), (var_qbex_dn13 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &[],
            multiplicity,
        );
        let eq22_e163: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_qbc);
        let eq22_value: f64 = eq22_e163;
        let eq22_node_derivatives: [f64; 14] = [(var_qbc_dn0 * ddt_scale), (var_qbc_dn1 * ddt_scale), (var_qbc_dn2 * ddt_scale), (var_qbc_dn3 * ddt_scale), (var_qbc_dn4 * ddt_scale), (var_qbc_dn5 * ddt_scale), (var_qbc_dn6 * ddt_scale), (var_qbc_dn7 * ddt_scale), (var_qbc_dn8 * ddt_scale), (var_qbc_dn9 * ddt_scale), (var_qbc_dn10 * ddt_scale), (var_qbc_dn11 * ddt_scale), (var_qbc_dn12 * ddt_scale), (var_qbc_dn13 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &[],
            multiplicity,
        );
        let eq23_e165: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, var_qbcx);
        let eq23_value: f64 = eq23_e165;
        let eq23_node_derivatives: [f64; 14] = [(var_qbcx_dn0 * ddt_scale), (var_qbcx_dn1 * ddt_scale), (var_qbcx_dn2 * ddt_scale), (var_qbcx_dn3 * ddt_scale), (var_qbcx_dn4 * ddt_scale), (var_qbcx_dn5 * ddt_scale), (var_qbcx_dn6 * ddt_scale), (var_qbcx_dn7 * ddt_scale), (var_qbcx_dn8 * ddt_scale), (var_qbcx_dn9 * ddt_scale), (var_qbcx_dn10 * ddt_scale), (var_qbcx_dn11 * ddt_scale), (var_qbcx_dn12 * ddt_scale), (var_qbcx_dn13 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &[],
            multiplicity,
        );
        let eq24_e167: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, var_qbep);
        let eq24_value: f64 = eq24_e167;
        let eq24_node_derivatives: [f64; 14] = [(var_qbep_dn0 * ddt_scale), (var_qbep_dn1 * ddt_scale), (var_qbep_dn2 * ddt_scale), (var_qbep_dn3 * ddt_scale), (var_qbep_dn4 * ddt_scale), (var_qbep_dn5 * ddt_scale), (var_qbep_dn6 * ddt_scale), (var_qbep_dn7 * ddt_scale), (var_qbep_dn8 * ddt_scale), (var_qbep_dn9 * ddt_scale), (var_qbep_dn10 * ddt_scale), (var_qbep_dn11 * ddt_scale), (var_qbep_dn12 * ddt_scale), (var_qbep_dn13 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &[],
            multiplicity,
        );
        let eq27_e173: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, var_qbcp);
        let eq27_value: f64 = eq27_e173;
        let eq27_node_derivatives: [f64; 14] = [(var_qbcp_dn0 * ddt_scale), (var_qbcp_dn1 * ddt_scale), (var_qbcp_dn2 * ddt_scale), (var_qbcp_dn3 * ddt_scale), (var_qbcp_dn4 * ddt_scale), (var_qbcp_dn5 * ddt_scale), (var_qbcp_dn6 * ddt_scale), (var_qbcp_dn7 * ddt_scale), (var_qbcp_dn8 * ddt_scale), (var_qbcp_dn9 * ddt_scale), (var_qbcp_dn10 * ddt_scale), (var_qbcp_dn11 * ddt_scale), (var_qbcp_dn12 * ddt_scale), (var_qbcp_dn13 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(10),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &[],
            multiplicity,
        );
        let eq30_e179: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, var_qcth);
        let eq30_value: f64 = eq30_e179;
        let eq30_node_derivatives: [f64; 14] = [(var_qcth_dn0 * ddt_scale), (var_qcth_dn1 * ddt_scale), (var_qcth_dn2 * ddt_scale), (var_qcth_dn3 * ddt_scale), (var_qcth_dn4 * ddt_scale), (var_qcth_dn5 * ddt_scale), (var_qcth_dn6 * ddt_scale), (var_qcth_dn7 * ddt_scale), (var_qcth_dn8 * ddt_scale), (var_qcth_dn9 * ddt_scale), (var_qcth_dn10 * ddt_scale), (var_qcth_dn11 * ddt_scale), (var_qcth_dn12 * ddt_scale), (var_qcth_dn13 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &[],
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let eq20_e159_q: f64 = s.v[123];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            nodes,
            &s.dn[123],
            branches,
            &[],
            multiplicity,
        );
        let eq21_e161_q: f64 = s.v[124];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &s.dn[124],
            branches,
            &[],
            multiplicity,
        );
        let eq22_e163_q: f64 = s.v[125];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &s.dn[125],
            branches,
            &[],
            multiplicity,
        );
        let eq23_e165_q: f64 = s.v[126];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &s.dn[126],
            branches,
            &[],
            multiplicity,
        );
        let eq24_e167_q: f64 = s.v[127];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            nodes,
            &s.dn[127],
            branches,
            &[],
            multiplicity,
        );
        let eq27_e173_q: f64 = s.v[128];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[10]),
            nodes,
            &s.dn[128],
            branches,
            &[],
            multiplicity,
        );
        let eq30_e179_q: f64 = s.v[142];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &s.dn[142],
            branches,
            &[],
            multiplicity,
        );
    }
}
