#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        var_cgs0_t: f64,
        var_cgs0_t_db0: f64,
        var_cgs0_t_db1: f64,
        var_cgs0_t_db10: f64,
        var_cgs0_t_db11: f64,
        var_cgs0_t_db12: f64,
        var_cgs0_t_db13: f64,
        var_cgs0_t_db14: f64,
        var_cgs0_t_db2: f64,
        var_cgs0_t_db3: f64,
        var_cgs0_t_db4: f64,
        var_cgs0_t_db5: f64,
        var_cgs0_t_db6: f64,
        var_cgs0_t_db7: f64,
        var_cgs0_t_db8: f64,
        var_cgs0_t_db9: f64,
        var_cgs0_t_dn0: f64,
        var_cgs0_t_dn1: f64,
        var_cgs0_t_dn10: f64,
        var_cgs0_t_dn11: f64,
        var_cgs0_t_dn12: f64,
        var_cgs0_t_dn13: f64,
        var_cgs0_t_dn14: f64,
        var_cgs0_t_dn15: f64,
        var_cgs0_t_dn2: f64,
        var_cgs0_t_dn3: f64,
        var_cgs0_t_dn4: f64,
        var_cgs0_t_dn5: f64,
        var_cgs0_t_dn6: f64,
        var_cgs0_t_dn7: f64,
        var_cgs0_t_dn8: f64,
        var_cgs0_t_dn9: f64,
        var_guard13: f64,
        var_guard14: f64,
        var_guard15: f64,
        var_lc10: f64,
        var_lc10_db0: f64,
        var_lc10_db1: f64,
        var_lc10_db10: f64,
        var_lc10_db11: f64,
        var_lc10_db12: f64,
        var_lc10_db13: f64,
        var_lc10_db14: f64,
        var_lc10_db2: f64,
        var_lc10_db3: f64,
        var_lc10_db4: f64,
        var_lc10_db5: f64,
        var_lc10_db6: f64,
        var_lc10_db7: f64,
        var_lc10_db8: f64,
        var_lc10_db9: f64,
        var_lc10_dn0: f64,
        var_lc10_dn1: f64,
        var_lc10_dn10: f64,
        var_lc10_dn11: f64,
        var_lc10_dn12: f64,
        var_lc10_dn13: f64,
        var_lc10_dn14: f64,
        var_lc10_dn15: f64,
        var_lc10_dn2: f64,
        var_lc10_dn3: f64,
        var_lc10_dn4: f64,
        var_lc10_dn5: f64,
        var_lc10_dn6: f64,
        var_lc10_dn7: f64,
        var_lc10_dn8: f64,
        var_lc10_dn9: f64,
        var_p10_t: f64,
        var_p10_t_db0: f64,
        var_p10_t_db1: f64,
        var_p10_t_db10: f64,
        var_p10_t_db11: f64,
        var_p10_t_db12: f64,
        var_p10_t_db13: f64,
        var_p10_t_db14: f64,
        var_p10_t_db2: f64,
        var_p10_t_db3: f64,
        var_p10_t_db4: f64,
        var_p10_t_db5: f64,
        var_p10_t_db6: f64,
        var_p10_t_db7: f64,
        var_p10_t_db8: f64,
        var_p10_t_db9: f64,
        var_p10_t_dn0: f64,
        var_p10_t_dn1: f64,
        var_p10_t_dn10: f64,
        var_p10_t_dn11: f64,
        var_p10_t_dn12: f64,
        var_p10_t_dn13: f64,
        var_p10_t_dn14: f64,
        var_p10_t_dn15: f64,
        var_p10_t_dn2: f64,
        var_p10_t_dn3: f64,
        var_p10_t_dn4: f64,
        var_p10_t_dn5: f64,
        var_p10_t_dn6: f64,
        var_p10_t_dn7: f64,
        var_p10_t_dn8: f64,
        var_p10_t_dn9: f64,
        var_p40_t: f64,
        var_p40_t_db0: f64,
        var_p40_t_db1: f64,
        var_p40_t_db10: f64,
        var_p40_t_db11: f64,
        var_p40_t_db12: f64,
        var_p40_t_db13: f64,
        var_p40_t_db14: f64,
        var_p40_t_db2: f64,
        var_p40_t_db3: f64,
        var_p40_t_db4: f64,
        var_p40_t_db5: f64,
        var_p40_t_db6: f64,
        var_p40_t_db7: f64,
        var_p40_t_db8: f64,
        var_p40_t_db9: f64,
        var_p40_t_dn0: f64,
        var_p40_t_dn1: f64,
        var_p40_t_dn10: f64,
        var_p40_t_dn11: f64,
        var_p40_t_dn12: f64,
        var_p40_t_dn13: f64,
        var_p40_t_dn14: f64,
        var_p40_t_dn15: f64,
        var_p40_t_dn2: f64,
        var_p40_t_dn3: f64,
        var_p40_t_dn4: f64,
        var_p40_t_dn5: f64,
        var_p40_t_dn6: f64,
        var_p40_t_dn7: f64,
        var_p40_t_dn8: f64,
        var_p40_t_dn9: f64,
        var_psi_1: f64,
        var_psi_1_db0: f64,
        var_psi_1_db1: f64,
        var_psi_1_db10: f64,
        var_psi_1_db11: f64,
        var_psi_1_db12: f64,
        var_psi_1_db13: f64,
        var_psi_1_db14: f64,
        var_psi_1_db2: f64,
        var_psi_1_db3: f64,
        var_psi_1_db4: f64,
        var_psi_1_db5: f64,
        var_psi_1_db6: f64,
        var_psi_1_db7: f64,
        var_psi_1_db8: f64,
        var_psi_1_db9: f64,
        var_psi_1_dn0: f64,
        var_psi_1_dn1: f64,
        var_psi_1_dn10: f64,
        var_psi_1_dn11: f64,
        var_psi_1_dn12: f64,
        var_psi_1_dn13: f64,
        var_psi_1_dn14: f64,
        var_psi_1_dn15: f64,
        var_psi_1_dn2: f64,
        var_psi_1_dn3: f64,
        var_psi_1_dn4: f64,
        var_psi_1_dn5: f64,
        var_psi_1_dn6: f64,
        var_psi_1_dn7: f64,
        var_psi_1_dn8: f64,
        var_psi_1_dn9: f64,
        var_tanh2: f64,
        var_tanh2_db0: f64,
        var_tanh2_db1: f64,
        var_tanh2_db10: f64,
        var_tanh2_db11: f64,
        var_tanh2_db12: f64,
        var_tanh2_db13: f64,
        var_tanh2_db14: f64,
        var_tanh2_db2: f64,
        var_tanh2_db3: f64,
        var_tanh2_db4: f64,
        var_tanh2_db5: f64,
        var_tanh2_db6: f64,
        var_tanh2_db7: f64,
        var_tanh2_db8: f64,
        var_tanh2_db9: f64,
        var_tanh2_dn0: f64,
        var_tanh2_dn1: f64,
        var_tanh2_dn10: f64,
        var_tanh2_dn11: f64,
        var_tanh2_dn12: f64,
        var_tanh2_dn13: f64,
        var_tanh2_dn14: f64,
        var_tanh2_dn15: f64,
        var_tanh2_dn2: f64,
        var_tanh2_dn3: f64,
        var_tanh2_dn4: f64,
        var_tanh2_dn5: f64,
        var_tanh2_dn6: f64,
        var_tanh2_dn7: f64,
        var_tanh2_dn8: f64,
        var_tanh2_dn9: f64,
        var_vds: f64,
        var_vds_db0: f64,
        var_vds_db1: f64,
        var_vds_db10: f64,
        var_vds_db11: f64,
        var_vds_db12: f64,
        var_vds_db13: f64,
        var_vds_db14: f64,
        var_vds_db2: f64,
        var_vds_db3: f64,
        var_vds_db4: f64,
        var_vds_db5: f64,
        var_vds_db6: f64,
        var_vds_db7: f64,
        var_vds_db8: f64,
        var_vds_db9: f64,
        var_vds_dn0: f64,
        var_vds_dn1: f64,
        var_vds_dn10: f64,
        var_vds_dn11: f64,
        var_vds_dn12: f64,
        var_vds_dn13: f64,
        var_vds_dn14: f64,
        var_vds_dn15: f64,
        var_vds_dn2: f64,
        var_vds_dn3: f64,
        var_vds_dn4: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vds_dn8: f64,
        var_vds_dn9: f64,
        var_vgsc: f64,
        var_vgsc_db0: f64,
        var_vgsc_db1: f64,
        var_vgsc_db10: f64,
        var_vgsc_db11: f64,
        var_vgsc_db12: f64,
        var_vgsc_db13: f64,
        var_vgsc_db14: f64,
        var_vgsc_db2: f64,
        var_vgsc_db3: f64,
        var_vgsc_db4: f64,
        var_vgsc_db5: f64,
        var_vgsc_db6: f64,
        var_vgsc_db7: f64,
        var_vgsc_db8: f64,
        var_vgsc_db9: f64,
        var_vgsc_dn0: f64,
        var_vgsc_dn1: f64,
        var_vgsc_dn10: f64,
        var_vgsc_dn11: f64,
        var_vgsc_dn12: f64,
        var_vgsc_dn13: f64,
        var_vgsc_dn14: f64,
        var_vgsc_dn15: f64,
        var_vgsc_dn2: f64,
        var_vgsc_dn3: f64,
        var_vgsc_dn4: f64,
        var_vgsc_dn5: f64,
        var_vgsc_dn6: f64,
        var_vgsc_dn7: f64,
        var_vgsc_dn8: f64,
        var_vgsc_dn9: f64,
        var_cosh0_slot: &mut f64,
        var_cosh0_db0_slot: &mut f64,
        var_cosh0_db1_slot: &mut f64,
        var_cosh0_db10_slot: &mut f64,
        var_cosh0_db11_slot: &mut f64,
        var_cosh0_db12_slot: &mut f64,
        var_cosh0_db13_slot: &mut f64,
        var_cosh0_db14_slot: &mut f64,
        var_cosh0_db15_slot: &mut f64,
        var_cosh0_db16_slot: &mut f64,
        var_cosh0_db17_slot: &mut f64,
        var_cosh0_db18_slot: &mut f64,
        var_cosh0_db2_slot: &mut f64,
        var_cosh0_db3_slot: &mut f64,
        var_cosh0_db4_slot: &mut f64,
        var_cosh0_db5_slot: &mut f64,
        var_cosh0_db6_slot: &mut f64,
        var_cosh0_db7_slot: &mut f64,
        var_cosh0_db8_slot: &mut f64,
        var_cosh0_db9_slot: &mut f64,
        var_cosh0_dn0_slot: &mut f64,
        var_cosh0_dn1_slot: &mut f64,
        var_cosh0_dn10_slot: &mut f64,
        var_cosh0_dn11_slot: &mut f64,
        var_cosh0_dn12_slot: &mut f64,
        var_cosh0_dn13_slot: &mut f64,
        var_cosh0_dn14_slot: &mut f64,
        var_cosh0_dn15_slot: &mut f64,
        var_cosh0_dn2_slot: &mut f64,
        var_cosh0_dn3_slot: &mut f64,
        var_cosh0_dn4_slot: &mut f64,
        var_cosh0_dn5_slot: &mut f64,
        var_cosh0_dn6_slot: &mut f64,
        var_cosh0_dn7_slot: &mut f64,
        var_cosh0_dn8_slot: &mut f64,
        var_cosh0_dn9_slot: &mut f64,
        var_cosh0_rdb0_slot: &mut f64,
        var_cosh0_rdb1_slot: &mut f64,
        var_cosh0_rdb10_slot: &mut f64,
        var_cosh0_rdb11_slot: &mut f64,
        var_cosh0_rdb12_slot: &mut f64,
        var_cosh0_rdb13_slot: &mut f64,
        var_cosh0_rdb14_slot: &mut f64,
        var_cosh0_rdb15_slot: &mut f64,
        var_cosh0_rdb16_slot: &mut f64,
        var_cosh0_rdb17_slot: &mut f64,
        var_cosh0_rdb18_slot: &mut f64,
        var_cosh0_rdb2_slot: &mut f64,
        var_cosh0_rdb3_slot: &mut f64,
        var_cosh0_rdb4_slot: &mut f64,
        var_cosh0_rdb5_slot: &mut f64,
        var_cosh0_rdb6_slot: &mut f64,
        var_cosh0_rdb7_slot: &mut f64,
        var_cosh0_rdb8_slot: &mut f64,
        var_cosh0_rdb9_slot: &mut f64,
        var_cosh0_rdn0_slot: &mut f64,
        var_cosh0_rdn1_slot: &mut f64,
        var_cosh0_rdn10_slot: &mut f64,
        var_cosh0_rdn11_slot: &mut f64,
        var_cosh0_rdn12_slot: &mut f64,
        var_cosh0_rdn13_slot: &mut f64,
        var_cosh0_rdn14_slot: &mut f64,
        var_cosh0_rdn15_slot: &mut f64,
        var_cosh0_rdn2_slot: &mut f64,
        var_cosh0_rdn3_slot: &mut f64,
        var_cosh0_rdn4_slot: &mut f64,
        var_cosh0_rdn5_slot: &mut f64,
        var_cosh0_rdn6_slot: &mut f64,
        var_cosh0_rdn7_slot: &mut f64,
        var_cosh0_rdn8_slot: &mut f64,
        var_cosh0_rdn9_slot: &mut f64,
        var_cosh0_rv_slot: &mut f64,
        var_cosh1_slot: &mut f64,
        var_cosh1_db0_slot: &mut f64,
        var_cosh1_db1_slot: &mut f64,
        var_cosh1_db10_slot: &mut f64,
        var_cosh1_db11_slot: &mut f64,
        var_cosh1_db12_slot: &mut f64,
        var_cosh1_db13_slot: &mut f64,
        var_cosh1_db14_slot: &mut f64,
        var_cosh1_db15_slot: &mut f64,
        var_cosh1_db16_slot: &mut f64,
        var_cosh1_db17_slot: &mut f64,
        var_cosh1_db18_slot: &mut f64,
        var_cosh1_db2_slot: &mut f64,
        var_cosh1_db3_slot: &mut f64,
        var_cosh1_db4_slot: &mut f64,
        var_cosh1_db5_slot: &mut f64,
        var_cosh1_db6_slot: &mut f64,
        var_cosh1_db7_slot: &mut f64,
        var_cosh1_db8_slot: &mut f64,
        var_cosh1_db9_slot: &mut f64,
        var_cosh1_dn0_slot: &mut f64,
        var_cosh1_dn1_slot: &mut f64,
        var_cosh1_dn10_slot: &mut f64,
        var_cosh1_dn11_slot: &mut f64,
        var_cosh1_dn12_slot: &mut f64,
        var_cosh1_dn13_slot: &mut f64,
        var_cosh1_dn14_slot: &mut f64,
        var_cosh1_dn15_slot: &mut f64,
        var_cosh1_dn2_slot: &mut f64,
        var_cosh1_dn3_slot: &mut f64,
        var_cosh1_dn4_slot: &mut f64,
        var_cosh1_dn5_slot: &mut f64,
        var_cosh1_dn6_slot: &mut f64,
        var_cosh1_dn7_slot: &mut f64,
        var_cosh1_dn8_slot: &mut f64,
        var_cosh1_dn9_slot: &mut f64,
        var_cosh1_rdb0_slot: &mut f64,
        var_cosh1_rdb1_slot: &mut f64,
        var_cosh1_rdb10_slot: &mut f64,
        var_cosh1_rdb11_slot: &mut f64,
        var_cosh1_rdb12_slot: &mut f64,
        var_cosh1_rdb13_slot: &mut f64,
        var_cosh1_rdb14_slot: &mut f64,
        var_cosh1_rdb15_slot: &mut f64,
        var_cosh1_rdb16_slot: &mut f64,
        var_cosh1_rdb17_slot: &mut f64,
        var_cosh1_rdb18_slot: &mut f64,
        var_cosh1_rdb2_slot: &mut f64,
        var_cosh1_rdb3_slot: &mut f64,
        var_cosh1_rdb4_slot: &mut f64,
        var_cosh1_rdb5_slot: &mut f64,
        var_cosh1_rdb6_slot: &mut f64,
        var_cosh1_rdb7_slot: &mut f64,
        var_cosh1_rdb8_slot: &mut f64,
        var_cosh1_rdb9_slot: &mut f64,
        var_cosh1_rdn0_slot: &mut f64,
        var_cosh1_rdn1_slot: &mut f64,
        var_cosh1_rdn10_slot: &mut f64,
        var_cosh1_rdn11_slot: &mut f64,
        var_cosh1_rdn12_slot: &mut f64,
        var_cosh1_rdn13_slot: &mut f64,
        var_cosh1_rdn14_slot: &mut f64,
        var_cosh1_rdn15_slot: &mut f64,
        var_cosh1_rdn2_slot: &mut f64,
        var_cosh1_rdn3_slot: &mut f64,
        var_cosh1_rdn4_slot: &mut f64,
        var_cosh1_rdn5_slot: &mut f64,
        var_cosh1_rdn6_slot: &mut f64,
        var_cosh1_rdn7_slot: &mut f64,
        var_cosh1_rdn8_slot: &mut f64,
        var_cosh1_rdn9_slot: &mut f64,
        var_cosh1_rv_slot: &mut f64,
        var_lc1_slot: &mut f64,
        var_lc1_db0_slot: &mut f64,
        var_lc1_db1_slot: &mut f64,
        var_lc1_db10_slot: &mut f64,
        var_lc1_db11_slot: &mut f64,
        var_lc1_db12_slot: &mut f64,
        var_lc1_db13_slot: &mut f64,
        var_lc1_db14_slot: &mut f64,
        var_lc1_db15_slot: &mut f64,
        var_lc1_db16_slot: &mut f64,
        var_lc1_db17_slot: &mut f64,
        var_lc1_db18_slot: &mut f64,
        var_lc1_db2_slot: &mut f64,
        var_lc1_db3_slot: &mut f64,
        var_lc1_db4_slot: &mut f64,
        var_lc1_db5_slot: &mut f64,
        var_lc1_db6_slot: &mut f64,
        var_lc1_db7_slot: &mut f64,
        var_lc1_db8_slot: &mut f64,
        var_lc1_db9_slot: &mut f64,
        var_lc1_dn0_slot: &mut f64,
        var_lc1_dn1_slot: &mut f64,
        var_lc1_dn10_slot: &mut f64,
        var_lc1_dn11_slot: &mut f64,
        var_lc1_dn12_slot: &mut f64,
        var_lc1_dn13_slot: &mut f64,
        var_lc1_dn14_slot: &mut f64,
        var_lc1_dn15_slot: &mut f64,
        var_lc1_dn2_slot: &mut f64,
        var_lc1_dn3_slot: &mut f64,
        var_lc1_dn4_slot: &mut f64,
        var_lc1_dn5_slot: &mut f64,
        var_lc1_dn6_slot: &mut f64,
        var_lc1_dn7_slot: &mut f64,
        var_lc1_dn8_slot: &mut f64,
        var_lc1_dn9_slot: &mut f64,
        var_lc1_rdb0_slot: &mut f64,
        var_lc1_rdb1_slot: &mut f64,
        var_lc1_rdb10_slot: &mut f64,
        var_lc1_rdb11_slot: &mut f64,
        var_lc1_rdb12_slot: &mut f64,
        var_lc1_rdb13_slot: &mut f64,
        var_lc1_rdb14_slot: &mut f64,
        var_lc1_rdb15_slot: &mut f64,
        var_lc1_rdb16_slot: &mut f64,
        var_lc1_rdb17_slot: &mut f64,
        var_lc1_rdb18_slot: &mut f64,
        var_lc1_rdb2_slot: &mut f64,
        var_lc1_rdb3_slot: &mut f64,
        var_lc1_rdb4_slot: &mut f64,
        var_lc1_rdb5_slot: &mut f64,
        var_lc1_rdb6_slot: &mut f64,
        var_lc1_rdb7_slot: &mut f64,
        var_lc1_rdb8_slot: &mut f64,
        var_lc1_rdb9_slot: &mut f64,
        var_lc1_rdn0_slot: &mut f64,
        var_lc1_rdn1_slot: &mut f64,
        var_lc1_rdn10_slot: &mut f64,
        var_lc1_rdn11_slot: &mut f64,
        var_lc1_rdn12_slot: &mut f64,
        var_lc1_rdn13_slot: &mut f64,
        var_lc1_rdn14_slot: &mut f64,
        var_lc1_rdn15_slot: &mut f64,
        var_lc1_rdn2_slot: &mut f64,
        var_lc1_rdn3_slot: &mut f64,
        var_lc1_rdn4_slot: &mut f64,
        var_lc1_rdn5_slot: &mut f64,
        var_lc1_rdn6_slot: &mut f64,
        var_lc1_rdn7_slot: &mut f64,
        var_lc1_rdn8_slot: &mut f64,
        var_lc1_rdn9_slot: &mut f64,
        var_lc1_rv_slot: &mut f64,
        var_lc40_slot: &mut f64,
        var_lc40_db0_slot: &mut f64,
        var_lc40_db1_slot: &mut f64,
        var_lc40_db10_slot: &mut f64,
        var_lc40_db11_slot: &mut f64,
        var_lc40_db12_slot: &mut f64,
        var_lc40_db13_slot: &mut f64,
        var_lc40_db14_slot: &mut f64,
        var_lc40_db15_slot: &mut f64,
        var_lc40_db16_slot: &mut f64,
        var_lc40_db17_slot: &mut f64,
        var_lc40_db18_slot: &mut f64,
        var_lc40_db2_slot: &mut f64,
        var_lc40_db3_slot: &mut f64,
        var_lc40_db4_slot: &mut f64,
        var_lc40_db5_slot: &mut f64,
        var_lc40_db6_slot: &mut f64,
        var_lc40_db7_slot: &mut f64,
        var_lc40_db8_slot: &mut f64,
        var_lc40_db9_slot: &mut f64,
        var_lc40_dn0_slot: &mut f64,
        var_lc40_dn1_slot: &mut f64,
        var_lc40_dn10_slot: &mut f64,
        var_lc40_dn11_slot: &mut f64,
        var_lc40_dn12_slot: &mut f64,
        var_lc40_dn13_slot: &mut f64,
        var_lc40_dn14_slot: &mut f64,
        var_lc40_dn15_slot: &mut f64,
        var_lc40_dn2_slot: &mut f64,
        var_lc40_dn3_slot: &mut f64,
        var_lc40_dn4_slot: &mut f64,
        var_lc40_dn5_slot: &mut f64,
        var_lc40_dn6_slot: &mut f64,
        var_lc40_dn7_slot: &mut f64,
        var_lc40_dn8_slot: &mut f64,
        var_lc40_dn9_slot: &mut f64,
        var_lc40_rdb0_slot: &mut f64,
        var_lc40_rdb1_slot: &mut f64,
        var_lc40_rdb10_slot: &mut f64,
        var_lc40_rdb11_slot: &mut f64,
        var_lc40_rdb12_slot: &mut f64,
        var_lc40_rdb13_slot: &mut f64,
        var_lc40_rdb14_slot: &mut f64,
        var_lc40_rdb15_slot: &mut f64,
        var_lc40_rdb16_slot: &mut f64,
        var_lc40_rdb17_slot: &mut f64,
        var_lc40_rdb18_slot: &mut f64,
        var_lc40_rdb2_slot: &mut f64,
        var_lc40_rdb3_slot: &mut f64,
        var_lc40_rdb4_slot: &mut f64,
        var_lc40_rdb5_slot: &mut f64,
        var_lc40_rdb6_slot: &mut f64,
        var_lc40_rdb7_slot: &mut f64,
        var_lc40_rdb8_slot: &mut f64,
        var_lc40_rdb9_slot: &mut f64,
        var_lc40_rdn0_slot: &mut f64,
        var_lc40_rdn1_slot: &mut f64,
        var_lc40_rdn10_slot: &mut f64,
        var_lc40_rdn11_slot: &mut f64,
        var_lc40_rdn12_slot: &mut f64,
        var_lc40_rdn13_slot: &mut f64,
        var_lc40_rdn14_slot: &mut f64,
        var_lc40_rdn15_slot: &mut f64,
        var_lc40_rdn2_slot: &mut f64,
        var_lc40_rdn3_slot: &mut f64,
        var_lc40_rdn4_slot: &mut f64,
        var_lc40_rdn5_slot: &mut f64,
        var_lc40_rdn6_slot: &mut f64,
        var_lc40_rdn7_slot: &mut f64,
        var_lc40_rdn8_slot: &mut f64,
        var_lc40_rdn9_slot: &mut f64,
        var_lc40_rv_slot: &mut f64,
        var_qgs_slot: &mut f64,
        var_qgs0_slot: &mut f64,
        var_qgs0_db0_slot: &mut f64,
        var_qgs0_db1_slot: &mut f64,
        var_qgs0_db10_slot: &mut f64,
        var_qgs0_db11_slot: &mut f64,
        var_qgs0_db12_slot: &mut f64,
        var_qgs0_db13_slot: &mut f64,
        var_qgs0_db14_slot: &mut f64,
        var_qgs0_db15_slot: &mut f64,
        var_qgs0_db16_slot: &mut f64,
        var_qgs0_db17_slot: &mut f64,
        var_qgs0_db18_slot: &mut f64,
        var_qgs0_db2_slot: &mut f64,
        var_qgs0_db3_slot: &mut f64,
        var_qgs0_db4_slot: &mut f64,
        var_qgs0_db5_slot: &mut f64,
        var_qgs0_db6_slot: &mut f64,
        var_qgs0_db7_slot: &mut f64,
        var_qgs0_db8_slot: &mut f64,
        var_qgs0_db9_slot: &mut f64,
        var_qgs0_dn0_slot: &mut f64,
        var_qgs0_dn1_slot: &mut f64,
        var_qgs0_dn10_slot: &mut f64,
        var_qgs0_dn11_slot: &mut f64,
        var_qgs0_dn12_slot: &mut f64,
        var_qgs0_dn13_slot: &mut f64,
        var_qgs0_dn14_slot: &mut f64,
        var_qgs0_dn15_slot: &mut f64,
        var_qgs0_dn2_slot: &mut f64,
        var_qgs0_dn3_slot: &mut f64,
        var_qgs0_dn4_slot: &mut f64,
        var_qgs0_dn5_slot: &mut f64,
        var_qgs0_dn6_slot: &mut f64,
        var_qgs0_dn7_slot: &mut f64,
        var_qgs0_dn8_slot: &mut f64,
        var_qgs0_dn9_slot: &mut f64,
        var_qgs0_rdb0_slot: &mut f64,
        var_qgs0_rdb1_slot: &mut f64,
        var_qgs0_rdb10_slot: &mut f64,
        var_qgs0_rdb11_slot: &mut f64,
        var_qgs0_rdb12_slot: &mut f64,
        var_qgs0_rdb13_slot: &mut f64,
        var_qgs0_rdb14_slot: &mut f64,
        var_qgs0_rdb15_slot: &mut f64,
        var_qgs0_rdb16_slot: &mut f64,
        var_qgs0_rdb17_slot: &mut f64,
        var_qgs0_rdb18_slot: &mut f64,
        var_qgs0_rdb2_slot: &mut f64,
        var_qgs0_rdb3_slot: &mut f64,
        var_qgs0_rdb4_slot: &mut f64,
        var_qgs0_rdb5_slot: &mut f64,
        var_qgs0_rdb6_slot: &mut f64,
        var_qgs0_rdb7_slot: &mut f64,
        var_qgs0_rdb8_slot: &mut f64,
        var_qgs0_rdb9_slot: &mut f64,
        var_qgs0_rdn0_slot: &mut f64,
        var_qgs0_rdn1_slot: &mut f64,
        var_qgs0_rdn10_slot: &mut f64,
        var_qgs0_rdn11_slot: &mut f64,
        var_qgs0_rdn12_slot: &mut f64,
        var_qgs0_rdn13_slot: &mut f64,
        var_qgs0_rdn14_slot: &mut f64,
        var_qgs0_rdn15_slot: &mut f64,
        var_qgs0_rdn2_slot: &mut f64,
        var_qgs0_rdn3_slot: &mut f64,
        var_qgs0_rdn4_slot: &mut f64,
        var_qgs0_rdn5_slot: &mut f64,
        var_qgs0_rdn6_slot: &mut f64,
        var_qgs0_rdn7_slot: &mut f64,
        var_qgs0_rdn8_slot: &mut f64,
        var_qgs0_rdn9_slot: &mut f64,
        var_qgs0_rv_slot: &mut f64,
        var_qgs_db0_slot: &mut f64,
        var_qgs_db1_slot: &mut f64,
        var_qgs_db10_slot: &mut f64,
        var_qgs_db11_slot: &mut f64,
        var_qgs_db12_slot: &mut f64,
        var_qgs_db13_slot: &mut f64,
        var_qgs_db14_slot: &mut f64,
        var_qgs_db15_slot: &mut f64,
        var_qgs_db16_slot: &mut f64,
        var_qgs_db17_slot: &mut f64,
        var_qgs_db18_slot: &mut f64,
        var_qgs_db2_slot: &mut f64,
        var_qgs_db3_slot: &mut f64,
        var_qgs_db4_slot: &mut f64,
        var_qgs_db5_slot: &mut f64,
        var_qgs_db6_slot: &mut f64,
        var_qgs_db7_slot: &mut f64,
        var_qgs_db8_slot: &mut f64,
        var_qgs_db9_slot: &mut f64,
        var_qgs_dn0_slot: &mut f64,
        var_qgs_dn1_slot: &mut f64,
        var_qgs_dn10_slot: &mut f64,
        var_qgs_dn11_slot: &mut f64,
        var_qgs_dn12_slot: &mut f64,
        var_qgs_dn13_slot: &mut f64,
        var_qgs_dn14_slot: &mut f64,
        var_qgs_dn15_slot: &mut f64,
        var_qgs_dn2_slot: &mut f64,
        var_qgs_dn3_slot: &mut f64,
        var_qgs_dn4_slot: &mut f64,
        var_qgs_dn5_slot: &mut f64,
        var_qgs_dn6_slot: &mut f64,
        var_qgs_dn7_slot: &mut f64,
        var_qgs_dn8_slot: &mut f64,
        var_qgs_dn9_slot: &mut f64,
        var_qgs_rdb0_slot: &mut f64,
        var_qgs_rdb1_slot: &mut f64,
        var_qgs_rdb10_slot: &mut f64,
        var_qgs_rdb11_slot: &mut f64,
        var_qgs_rdb12_slot: &mut f64,
        var_qgs_rdb13_slot: &mut f64,
        var_qgs_rdb14_slot: &mut f64,
        var_qgs_rdb15_slot: &mut f64,
        var_qgs_rdb16_slot: &mut f64,
        var_qgs_rdb17_slot: &mut f64,
        var_qgs_rdb18_slot: &mut f64,
        var_qgs_rdb2_slot: &mut f64,
        var_qgs_rdb3_slot: &mut f64,
        var_qgs_rdb4_slot: &mut f64,
        var_qgs_rdb5_slot: &mut f64,
        var_qgs_rdb6_slot: &mut f64,
        var_qgs_rdb7_slot: &mut f64,
        var_qgs_rdb8_slot: &mut f64,
        var_qgs_rdb9_slot: &mut f64,
        var_qgs_rdn0_slot: &mut f64,
        var_qgs_rdn1_slot: &mut f64,
        var_qgs_rdn10_slot: &mut f64,
        var_qgs_rdn11_slot: &mut f64,
        var_qgs_rdn12_slot: &mut f64,
        var_qgs_rdn13_slot: &mut f64,
        var_qgs_rdn14_slot: &mut f64,
        var_qgs_rdn15_slot: &mut f64,
        var_qgs_rdn2_slot: &mut f64,
        var_qgs_rdn3_slot: &mut f64,
        var_qgs_rdn4_slot: &mut f64,
        var_qgs_rdn5_slot: &mut f64,
        var_qgs_rdn6_slot: &mut f64,
        var_qgs_rdn7_slot: &mut f64,
        var_qgs_rdn8_slot: &mut f64,
        var_qgs_rdn9_slot: &mut f64,
        var_qgs_rv_slot: &mut f64,
    ) {
        let mut var_cosh0: f64 = *var_cosh0_slot;
        let mut var_cosh0_db0: f64 = *var_cosh0_db0_slot;
        let mut var_cosh0_db1: f64 = *var_cosh0_db1_slot;
        let mut var_cosh0_db10: f64 = *var_cosh0_db10_slot;
        let mut var_cosh0_db11: f64 = *var_cosh0_db11_slot;
        let mut var_cosh0_db12: f64 = *var_cosh0_db12_slot;
        let mut var_cosh0_db13: f64 = *var_cosh0_db13_slot;
        let mut var_cosh0_db14: f64 = *var_cosh0_db14_slot;
        let mut var_cosh0_db15: f64 = *var_cosh0_db15_slot;
        let mut var_cosh0_db16: f64 = *var_cosh0_db16_slot;
        let mut var_cosh0_db17: f64 = *var_cosh0_db17_slot;
        let mut var_cosh0_db18: f64 = *var_cosh0_db18_slot;
        let mut var_cosh0_db2: f64 = *var_cosh0_db2_slot;
        let mut var_cosh0_db3: f64 = *var_cosh0_db3_slot;
        let mut var_cosh0_db4: f64 = *var_cosh0_db4_slot;
        let mut var_cosh0_db5: f64 = *var_cosh0_db5_slot;
        let mut var_cosh0_db6: f64 = *var_cosh0_db6_slot;
        let mut var_cosh0_db7: f64 = *var_cosh0_db7_slot;
        let mut var_cosh0_db8: f64 = *var_cosh0_db8_slot;
        let mut var_cosh0_db9: f64 = *var_cosh0_db9_slot;
        let mut var_cosh0_dn0: f64 = *var_cosh0_dn0_slot;
        let mut var_cosh0_dn1: f64 = *var_cosh0_dn1_slot;
        let mut var_cosh0_dn10: f64 = *var_cosh0_dn10_slot;
        let mut var_cosh0_dn11: f64 = *var_cosh0_dn11_slot;
        let mut var_cosh0_dn12: f64 = *var_cosh0_dn12_slot;
        let mut var_cosh0_dn13: f64 = *var_cosh0_dn13_slot;
        let mut var_cosh0_dn14: f64 = *var_cosh0_dn14_slot;
        let mut var_cosh0_dn15: f64 = *var_cosh0_dn15_slot;
        let mut var_cosh0_dn2: f64 = *var_cosh0_dn2_slot;
        let mut var_cosh0_dn3: f64 = *var_cosh0_dn3_slot;
        let mut var_cosh0_dn4: f64 = *var_cosh0_dn4_slot;
        let mut var_cosh0_dn5: f64 = *var_cosh0_dn5_slot;
        let mut var_cosh0_dn6: f64 = *var_cosh0_dn6_slot;
        let mut var_cosh0_dn7: f64 = *var_cosh0_dn7_slot;
        let mut var_cosh0_dn8: f64 = *var_cosh0_dn8_slot;
        let mut var_cosh0_dn9: f64 = *var_cosh0_dn9_slot;
        let mut var_cosh0_rdb0: f64 = *var_cosh0_rdb0_slot;
        let mut var_cosh0_rdb1: f64 = *var_cosh0_rdb1_slot;
        let mut var_cosh0_rdb10: f64 = *var_cosh0_rdb10_slot;
        let mut var_cosh0_rdb11: f64 = *var_cosh0_rdb11_slot;
        let mut var_cosh0_rdb12: f64 = *var_cosh0_rdb12_slot;
        let mut var_cosh0_rdb13: f64 = *var_cosh0_rdb13_slot;
        let mut var_cosh0_rdb14: f64 = *var_cosh0_rdb14_slot;
        let mut var_cosh0_rdb15: f64 = *var_cosh0_rdb15_slot;
        let mut var_cosh0_rdb16: f64 = *var_cosh0_rdb16_slot;
        let mut var_cosh0_rdb17: f64 = *var_cosh0_rdb17_slot;
        let mut var_cosh0_rdb18: f64 = *var_cosh0_rdb18_slot;
        let mut var_cosh0_rdb2: f64 = *var_cosh0_rdb2_slot;
        let mut var_cosh0_rdb3: f64 = *var_cosh0_rdb3_slot;
        let mut var_cosh0_rdb4: f64 = *var_cosh0_rdb4_slot;
        let mut var_cosh0_rdb5: f64 = *var_cosh0_rdb5_slot;
        let mut var_cosh0_rdb6: f64 = *var_cosh0_rdb6_slot;
        let mut var_cosh0_rdb7: f64 = *var_cosh0_rdb7_slot;
        let mut var_cosh0_rdb8: f64 = *var_cosh0_rdb8_slot;
        let mut var_cosh0_rdb9: f64 = *var_cosh0_rdb9_slot;
        let mut var_cosh0_rdn0: f64 = *var_cosh0_rdn0_slot;
        let mut var_cosh0_rdn1: f64 = *var_cosh0_rdn1_slot;
        let mut var_cosh0_rdn10: f64 = *var_cosh0_rdn10_slot;
        let mut var_cosh0_rdn11: f64 = *var_cosh0_rdn11_slot;
        let mut var_cosh0_rdn12: f64 = *var_cosh0_rdn12_slot;
        let mut var_cosh0_rdn13: f64 = *var_cosh0_rdn13_slot;
        let mut var_cosh0_rdn14: f64 = *var_cosh0_rdn14_slot;
        let mut var_cosh0_rdn15: f64 = *var_cosh0_rdn15_slot;
        let mut var_cosh0_rdn2: f64 = *var_cosh0_rdn2_slot;
        let mut var_cosh0_rdn3: f64 = *var_cosh0_rdn3_slot;
        let mut var_cosh0_rdn4: f64 = *var_cosh0_rdn4_slot;
        let mut var_cosh0_rdn5: f64 = *var_cosh0_rdn5_slot;
        let mut var_cosh0_rdn6: f64 = *var_cosh0_rdn6_slot;
        let mut var_cosh0_rdn7: f64 = *var_cosh0_rdn7_slot;
        let mut var_cosh0_rdn8: f64 = *var_cosh0_rdn8_slot;
        let mut var_cosh0_rdn9: f64 = *var_cosh0_rdn9_slot;
        let mut var_cosh0_rv: f64 = *var_cosh0_rv_slot;
        let mut var_cosh1: f64 = *var_cosh1_slot;
        let mut var_cosh1_db0: f64 = *var_cosh1_db0_slot;
        let mut var_cosh1_db1: f64 = *var_cosh1_db1_slot;
        let mut var_cosh1_db10: f64 = *var_cosh1_db10_slot;
        let mut var_cosh1_db11: f64 = *var_cosh1_db11_slot;
        let mut var_cosh1_db12: f64 = *var_cosh1_db12_slot;
        let mut var_cosh1_db13: f64 = *var_cosh1_db13_slot;
        let mut var_cosh1_db14: f64 = *var_cosh1_db14_slot;
        let mut var_cosh1_db15: f64 = *var_cosh1_db15_slot;
        let mut var_cosh1_db16: f64 = *var_cosh1_db16_slot;
        let mut var_cosh1_db17: f64 = *var_cosh1_db17_slot;
        let mut var_cosh1_db18: f64 = *var_cosh1_db18_slot;
        let mut var_cosh1_db2: f64 = *var_cosh1_db2_slot;
        let mut var_cosh1_db3: f64 = *var_cosh1_db3_slot;
        let mut var_cosh1_db4: f64 = *var_cosh1_db4_slot;
        let mut var_cosh1_db5: f64 = *var_cosh1_db5_slot;
        let mut var_cosh1_db6: f64 = *var_cosh1_db6_slot;
        let mut var_cosh1_db7: f64 = *var_cosh1_db7_slot;
        let mut var_cosh1_db8: f64 = *var_cosh1_db8_slot;
        let mut var_cosh1_db9: f64 = *var_cosh1_db9_slot;
        let mut var_cosh1_dn0: f64 = *var_cosh1_dn0_slot;
        let mut var_cosh1_dn1: f64 = *var_cosh1_dn1_slot;
        let mut var_cosh1_dn10: f64 = *var_cosh1_dn10_slot;
        let mut var_cosh1_dn11: f64 = *var_cosh1_dn11_slot;
        let mut var_cosh1_dn12: f64 = *var_cosh1_dn12_slot;
        let mut var_cosh1_dn13: f64 = *var_cosh1_dn13_slot;
        let mut var_cosh1_dn14: f64 = *var_cosh1_dn14_slot;
        let mut var_cosh1_dn15: f64 = *var_cosh1_dn15_slot;
        let mut var_cosh1_dn2: f64 = *var_cosh1_dn2_slot;
        let mut var_cosh1_dn3: f64 = *var_cosh1_dn3_slot;
        let mut var_cosh1_dn4: f64 = *var_cosh1_dn4_slot;
        let mut var_cosh1_dn5: f64 = *var_cosh1_dn5_slot;
        let mut var_cosh1_dn6: f64 = *var_cosh1_dn6_slot;
        let mut var_cosh1_dn7: f64 = *var_cosh1_dn7_slot;
        let mut var_cosh1_dn8: f64 = *var_cosh1_dn8_slot;
        let mut var_cosh1_dn9: f64 = *var_cosh1_dn9_slot;
        let mut var_cosh1_rdb0: f64 = *var_cosh1_rdb0_slot;
        let mut var_cosh1_rdb1: f64 = *var_cosh1_rdb1_slot;
        let mut var_cosh1_rdb10: f64 = *var_cosh1_rdb10_slot;
        let mut var_cosh1_rdb11: f64 = *var_cosh1_rdb11_slot;
        let mut var_cosh1_rdb12: f64 = *var_cosh1_rdb12_slot;
        let mut var_cosh1_rdb13: f64 = *var_cosh1_rdb13_slot;
        let mut var_cosh1_rdb14: f64 = *var_cosh1_rdb14_slot;
        let mut var_cosh1_rdb15: f64 = *var_cosh1_rdb15_slot;
        let mut var_cosh1_rdb16: f64 = *var_cosh1_rdb16_slot;
        let mut var_cosh1_rdb17: f64 = *var_cosh1_rdb17_slot;
        let mut var_cosh1_rdb18: f64 = *var_cosh1_rdb18_slot;
        let mut var_cosh1_rdb2: f64 = *var_cosh1_rdb2_slot;
        let mut var_cosh1_rdb3: f64 = *var_cosh1_rdb3_slot;
        let mut var_cosh1_rdb4: f64 = *var_cosh1_rdb4_slot;
        let mut var_cosh1_rdb5: f64 = *var_cosh1_rdb5_slot;
        let mut var_cosh1_rdb6: f64 = *var_cosh1_rdb6_slot;
        let mut var_cosh1_rdb7: f64 = *var_cosh1_rdb7_slot;
        let mut var_cosh1_rdb8: f64 = *var_cosh1_rdb8_slot;
        let mut var_cosh1_rdb9: f64 = *var_cosh1_rdb9_slot;
        let mut var_cosh1_rdn0: f64 = *var_cosh1_rdn0_slot;
        let mut var_cosh1_rdn1: f64 = *var_cosh1_rdn1_slot;
        let mut var_cosh1_rdn10: f64 = *var_cosh1_rdn10_slot;
        let mut var_cosh1_rdn11: f64 = *var_cosh1_rdn11_slot;
        let mut var_cosh1_rdn12: f64 = *var_cosh1_rdn12_slot;
        let mut var_cosh1_rdn13: f64 = *var_cosh1_rdn13_slot;
        let mut var_cosh1_rdn14: f64 = *var_cosh1_rdn14_slot;
        let mut var_cosh1_rdn15: f64 = *var_cosh1_rdn15_slot;
        let mut var_cosh1_rdn2: f64 = *var_cosh1_rdn2_slot;
        let mut var_cosh1_rdn3: f64 = *var_cosh1_rdn3_slot;
        let mut var_cosh1_rdn4: f64 = *var_cosh1_rdn4_slot;
        let mut var_cosh1_rdn5: f64 = *var_cosh1_rdn5_slot;
        let mut var_cosh1_rdn6: f64 = *var_cosh1_rdn6_slot;
        let mut var_cosh1_rdn7: f64 = *var_cosh1_rdn7_slot;
        let mut var_cosh1_rdn8: f64 = *var_cosh1_rdn8_slot;
        let mut var_cosh1_rdn9: f64 = *var_cosh1_rdn9_slot;
        let mut var_cosh1_rv: f64 = *var_cosh1_rv_slot;
        let mut var_lc1: f64 = *var_lc1_slot;
        let mut var_lc1_db0: f64 = *var_lc1_db0_slot;
        let mut var_lc1_db1: f64 = *var_lc1_db1_slot;
        let mut var_lc1_db10: f64 = *var_lc1_db10_slot;
        let mut var_lc1_db11: f64 = *var_lc1_db11_slot;
        let mut var_lc1_db12: f64 = *var_lc1_db12_slot;
        let mut var_lc1_db13: f64 = *var_lc1_db13_slot;
        let mut var_lc1_db14: f64 = *var_lc1_db14_slot;
        let mut var_lc1_db15: f64 = *var_lc1_db15_slot;
        let mut var_lc1_db16: f64 = *var_lc1_db16_slot;
        let mut var_lc1_db17: f64 = *var_lc1_db17_slot;
        let mut var_lc1_db18: f64 = *var_lc1_db18_slot;
        let mut var_lc1_db2: f64 = *var_lc1_db2_slot;
        let mut var_lc1_db3: f64 = *var_lc1_db3_slot;
        let mut var_lc1_db4: f64 = *var_lc1_db4_slot;
        let mut var_lc1_db5: f64 = *var_lc1_db5_slot;
        let mut var_lc1_db6: f64 = *var_lc1_db6_slot;
        let mut var_lc1_db7: f64 = *var_lc1_db7_slot;
        let mut var_lc1_db8: f64 = *var_lc1_db8_slot;
        let mut var_lc1_db9: f64 = *var_lc1_db9_slot;
        let mut var_lc1_dn0: f64 = *var_lc1_dn0_slot;
        let mut var_lc1_dn1: f64 = *var_lc1_dn1_slot;
        let mut var_lc1_dn10: f64 = *var_lc1_dn10_slot;
        let mut var_lc1_dn11: f64 = *var_lc1_dn11_slot;
        let mut var_lc1_dn12: f64 = *var_lc1_dn12_slot;
        let mut var_lc1_dn13: f64 = *var_lc1_dn13_slot;
        let mut var_lc1_dn14: f64 = *var_lc1_dn14_slot;
        let mut var_lc1_dn15: f64 = *var_lc1_dn15_slot;
        let mut var_lc1_dn2: f64 = *var_lc1_dn2_slot;
        let mut var_lc1_dn3: f64 = *var_lc1_dn3_slot;
        let mut var_lc1_dn4: f64 = *var_lc1_dn4_slot;
        let mut var_lc1_dn5: f64 = *var_lc1_dn5_slot;
        let mut var_lc1_dn6: f64 = *var_lc1_dn6_slot;
        let mut var_lc1_dn7: f64 = *var_lc1_dn7_slot;
        let mut var_lc1_dn8: f64 = *var_lc1_dn8_slot;
        let mut var_lc1_dn9: f64 = *var_lc1_dn9_slot;
        let mut var_lc1_rdb0: f64 = *var_lc1_rdb0_slot;
        let mut var_lc1_rdb1: f64 = *var_lc1_rdb1_slot;
        let mut var_lc1_rdb10: f64 = *var_lc1_rdb10_slot;
        let mut var_lc1_rdb11: f64 = *var_lc1_rdb11_slot;
        let mut var_lc1_rdb12: f64 = *var_lc1_rdb12_slot;
        let mut var_lc1_rdb13: f64 = *var_lc1_rdb13_slot;
        let mut var_lc1_rdb14: f64 = *var_lc1_rdb14_slot;
        let mut var_lc1_rdb15: f64 = *var_lc1_rdb15_slot;
        let mut var_lc1_rdb16: f64 = *var_lc1_rdb16_slot;
        let mut var_lc1_rdb17: f64 = *var_lc1_rdb17_slot;
        let mut var_lc1_rdb18: f64 = *var_lc1_rdb18_slot;
        let mut var_lc1_rdb2: f64 = *var_lc1_rdb2_slot;
        let mut var_lc1_rdb3: f64 = *var_lc1_rdb3_slot;
        let mut var_lc1_rdb4: f64 = *var_lc1_rdb4_slot;
        let mut var_lc1_rdb5: f64 = *var_lc1_rdb5_slot;
        let mut var_lc1_rdb6: f64 = *var_lc1_rdb6_slot;
        let mut var_lc1_rdb7: f64 = *var_lc1_rdb7_slot;
        let mut var_lc1_rdb8: f64 = *var_lc1_rdb8_slot;
        let mut var_lc1_rdb9: f64 = *var_lc1_rdb9_slot;
        let mut var_lc1_rdn0: f64 = *var_lc1_rdn0_slot;
        let mut var_lc1_rdn1: f64 = *var_lc1_rdn1_slot;
        let mut var_lc1_rdn10: f64 = *var_lc1_rdn10_slot;
        let mut var_lc1_rdn11: f64 = *var_lc1_rdn11_slot;
        let mut var_lc1_rdn12: f64 = *var_lc1_rdn12_slot;
        let mut var_lc1_rdn13: f64 = *var_lc1_rdn13_slot;
        let mut var_lc1_rdn14: f64 = *var_lc1_rdn14_slot;
        let mut var_lc1_rdn15: f64 = *var_lc1_rdn15_slot;
        let mut var_lc1_rdn2: f64 = *var_lc1_rdn2_slot;
        let mut var_lc1_rdn3: f64 = *var_lc1_rdn3_slot;
        let mut var_lc1_rdn4: f64 = *var_lc1_rdn4_slot;
        let mut var_lc1_rdn5: f64 = *var_lc1_rdn5_slot;
        let mut var_lc1_rdn6: f64 = *var_lc1_rdn6_slot;
        let mut var_lc1_rdn7: f64 = *var_lc1_rdn7_slot;
        let mut var_lc1_rdn8: f64 = *var_lc1_rdn8_slot;
        let mut var_lc1_rdn9: f64 = *var_lc1_rdn9_slot;
        let mut var_lc1_rv: f64 = *var_lc1_rv_slot;
        let mut var_lc40: f64 = *var_lc40_slot;
        let mut var_lc40_db0: f64 = *var_lc40_db0_slot;
        let mut var_lc40_db1: f64 = *var_lc40_db1_slot;
        let mut var_lc40_db10: f64 = *var_lc40_db10_slot;
        let mut var_lc40_db11: f64 = *var_lc40_db11_slot;
        let mut var_lc40_db12: f64 = *var_lc40_db12_slot;
        let mut var_lc40_db13: f64 = *var_lc40_db13_slot;
        let mut var_lc40_db14: f64 = *var_lc40_db14_slot;
        let mut var_lc40_db15: f64 = *var_lc40_db15_slot;
        let mut var_lc40_db16: f64 = *var_lc40_db16_slot;
        let mut var_lc40_db17: f64 = *var_lc40_db17_slot;
        let mut var_lc40_db18: f64 = *var_lc40_db18_slot;
        let mut var_lc40_db2: f64 = *var_lc40_db2_slot;
        let mut var_lc40_db3: f64 = *var_lc40_db3_slot;
        let mut var_lc40_db4: f64 = *var_lc40_db4_slot;
        let mut var_lc40_db5: f64 = *var_lc40_db5_slot;
        let mut var_lc40_db6: f64 = *var_lc40_db6_slot;
        let mut var_lc40_db7: f64 = *var_lc40_db7_slot;
        let mut var_lc40_db8: f64 = *var_lc40_db8_slot;
        let mut var_lc40_db9: f64 = *var_lc40_db9_slot;
        let mut var_lc40_dn0: f64 = *var_lc40_dn0_slot;
        let mut var_lc40_dn1: f64 = *var_lc40_dn1_slot;
        let mut var_lc40_dn10: f64 = *var_lc40_dn10_slot;
        let mut var_lc40_dn11: f64 = *var_lc40_dn11_slot;
        let mut var_lc40_dn12: f64 = *var_lc40_dn12_slot;
        let mut var_lc40_dn13: f64 = *var_lc40_dn13_slot;
        let mut var_lc40_dn14: f64 = *var_lc40_dn14_slot;
        let mut var_lc40_dn15: f64 = *var_lc40_dn15_slot;
        let mut var_lc40_dn2: f64 = *var_lc40_dn2_slot;
        let mut var_lc40_dn3: f64 = *var_lc40_dn3_slot;
        let mut var_lc40_dn4: f64 = *var_lc40_dn4_slot;
        let mut var_lc40_dn5: f64 = *var_lc40_dn5_slot;
        let mut var_lc40_dn6: f64 = *var_lc40_dn6_slot;
        let mut var_lc40_dn7: f64 = *var_lc40_dn7_slot;
        let mut var_lc40_dn8: f64 = *var_lc40_dn8_slot;
        let mut var_lc40_dn9: f64 = *var_lc40_dn9_slot;
        let mut var_lc40_rdb0: f64 = *var_lc40_rdb0_slot;
        let mut var_lc40_rdb1: f64 = *var_lc40_rdb1_slot;
        let mut var_lc40_rdb10: f64 = *var_lc40_rdb10_slot;
        let mut var_lc40_rdb11: f64 = *var_lc40_rdb11_slot;
        let mut var_lc40_rdb12: f64 = *var_lc40_rdb12_slot;
        let mut var_lc40_rdb13: f64 = *var_lc40_rdb13_slot;
        let mut var_lc40_rdb14: f64 = *var_lc40_rdb14_slot;
        let mut var_lc40_rdb15: f64 = *var_lc40_rdb15_slot;
        let mut var_lc40_rdb16: f64 = *var_lc40_rdb16_slot;
        let mut var_lc40_rdb17: f64 = *var_lc40_rdb17_slot;
        let mut var_lc40_rdb18: f64 = *var_lc40_rdb18_slot;
        let mut var_lc40_rdb2: f64 = *var_lc40_rdb2_slot;
        let mut var_lc40_rdb3: f64 = *var_lc40_rdb3_slot;
        let mut var_lc40_rdb4: f64 = *var_lc40_rdb4_slot;
        let mut var_lc40_rdb5: f64 = *var_lc40_rdb5_slot;
        let mut var_lc40_rdb6: f64 = *var_lc40_rdb6_slot;
        let mut var_lc40_rdb7: f64 = *var_lc40_rdb7_slot;
        let mut var_lc40_rdb8: f64 = *var_lc40_rdb8_slot;
        let mut var_lc40_rdb9: f64 = *var_lc40_rdb9_slot;
        let mut var_lc40_rdn0: f64 = *var_lc40_rdn0_slot;
        let mut var_lc40_rdn1: f64 = *var_lc40_rdn1_slot;
        let mut var_lc40_rdn10: f64 = *var_lc40_rdn10_slot;
        let mut var_lc40_rdn11: f64 = *var_lc40_rdn11_slot;
        let mut var_lc40_rdn12: f64 = *var_lc40_rdn12_slot;
        let mut var_lc40_rdn13: f64 = *var_lc40_rdn13_slot;
        let mut var_lc40_rdn14: f64 = *var_lc40_rdn14_slot;
        let mut var_lc40_rdn15: f64 = *var_lc40_rdn15_slot;
        let mut var_lc40_rdn2: f64 = *var_lc40_rdn2_slot;
        let mut var_lc40_rdn3: f64 = *var_lc40_rdn3_slot;
        let mut var_lc40_rdn4: f64 = *var_lc40_rdn4_slot;
        let mut var_lc40_rdn5: f64 = *var_lc40_rdn5_slot;
        let mut var_lc40_rdn6: f64 = *var_lc40_rdn6_slot;
        let mut var_lc40_rdn7: f64 = *var_lc40_rdn7_slot;
        let mut var_lc40_rdn8: f64 = *var_lc40_rdn8_slot;
        let mut var_lc40_rdn9: f64 = *var_lc40_rdn9_slot;
        let mut var_lc40_rv: f64 = *var_lc40_rv_slot;
        let mut var_qgs: f64 = *var_qgs_slot;
        let mut var_qgs0: f64 = *var_qgs0_slot;
        let mut var_qgs0_db0: f64 = *var_qgs0_db0_slot;
        let mut var_qgs0_db1: f64 = *var_qgs0_db1_slot;
        let mut var_qgs0_db10: f64 = *var_qgs0_db10_slot;
        let mut var_qgs0_db11: f64 = *var_qgs0_db11_slot;
        let mut var_qgs0_db12: f64 = *var_qgs0_db12_slot;
        let mut var_qgs0_db13: f64 = *var_qgs0_db13_slot;
        let mut var_qgs0_db14: f64 = *var_qgs0_db14_slot;
        let mut var_qgs0_db15: f64 = *var_qgs0_db15_slot;
        let mut var_qgs0_db16: f64 = *var_qgs0_db16_slot;
        let mut var_qgs0_db17: f64 = *var_qgs0_db17_slot;
        let mut var_qgs0_db18: f64 = *var_qgs0_db18_slot;
        let mut var_qgs0_db2: f64 = *var_qgs0_db2_slot;
        let mut var_qgs0_db3: f64 = *var_qgs0_db3_slot;
        let mut var_qgs0_db4: f64 = *var_qgs0_db4_slot;
        let mut var_qgs0_db5: f64 = *var_qgs0_db5_slot;
        let mut var_qgs0_db6: f64 = *var_qgs0_db6_slot;
        let mut var_qgs0_db7: f64 = *var_qgs0_db7_slot;
        let mut var_qgs0_db8: f64 = *var_qgs0_db8_slot;
        let mut var_qgs0_db9: f64 = *var_qgs0_db9_slot;
        let mut var_qgs0_dn0: f64 = *var_qgs0_dn0_slot;
        let mut var_qgs0_dn1: f64 = *var_qgs0_dn1_slot;
        let mut var_qgs0_dn10: f64 = *var_qgs0_dn10_slot;
        let mut var_qgs0_dn11: f64 = *var_qgs0_dn11_slot;
        let mut var_qgs0_dn12: f64 = *var_qgs0_dn12_slot;
        let mut var_qgs0_dn13: f64 = *var_qgs0_dn13_slot;
        let mut var_qgs0_dn14: f64 = *var_qgs0_dn14_slot;
        let mut var_qgs0_dn15: f64 = *var_qgs0_dn15_slot;
        let mut var_qgs0_dn2: f64 = *var_qgs0_dn2_slot;
        let mut var_qgs0_dn3: f64 = *var_qgs0_dn3_slot;
        let mut var_qgs0_dn4: f64 = *var_qgs0_dn4_slot;
        let mut var_qgs0_dn5: f64 = *var_qgs0_dn5_slot;
        let mut var_qgs0_dn6: f64 = *var_qgs0_dn6_slot;
        let mut var_qgs0_dn7: f64 = *var_qgs0_dn7_slot;
        let mut var_qgs0_dn8: f64 = *var_qgs0_dn8_slot;
        let mut var_qgs0_dn9: f64 = *var_qgs0_dn9_slot;
        let mut var_qgs0_rdb0: f64 = *var_qgs0_rdb0_slot;
        let mut var_qgs0_rdb1: f64 = *var_qgs0_rdb1_slot;
        let mut var_qgs0_rdb10: f64 = *var_qgs0_rdb10_slot;
        let mut var_qgs0_rdb11: f64 = *var_qgs0_rdb11_slot;
        let mut var_qgs0_rdb12: f64 = *var_qgs0_rdb12_slot;
        let mut var_qgs0_rdb13: f64 = *var_qgs0_rdb13_slot;
        let mut var_qgs0_rdb14: f64 = *var_qgs0_rdb14_slot;
        let mut var_qgs0_rdb15: f64 = *var_qgs0_rdb15_slot;
        let mut var_qgs0_rdb16: f64 = *var_qgs0_rdb16_slot;
        let mut var_qgs0_rdb17: f64 = *var_qgs0_rdb17_slot;
        let mut var_qgs0_rdb18: f64 = *var_qgs0_rdb18_slot;
        let mut var_qgs0_rdb2: f64 = *var_qgs0_rdb2_slot;
        let mut var_qgs0_rdb3: f64 = *var_qgs0_rdb3_slot;
        let mut var_qgs0_rdb4: f64 = *var_qgs0_rdb4_slot;
        let mut var_qgs0_rdb5: f64 = *var_qgs0_rdb5_slot;
        let mut var_qgs0_rdb6: f64 = *var_qgs0_rdb6_slot;
        let mut var_qgs0_rdb7: f64 = *var_qgs0_rdb7_slot;
        let mut var_qgs0_rdb8: f64 = *var_qgs0_rdb8_slot;
        let mut var_qgs0_rdb9: f64 = *var_qgs0_rdb9_slot;
        let mut var_qgs0_rdn0: f64 = *var_qgs0_rdn0_slot;
        let mut var_qgs0_rdn1: f64 = *var_qgs0_rdn1_slot;
        let mut var_qgs0_rdn10: f64 = *var_qgs0_rdn10_slot;
        let mut var_qgs0_rdn11: f64 = *var_qgs0_rdn11_slot;
        let mut var_qgs0_rdn12: f64 = *var_qgs0_rdn12_slot;
        let mut var_qgs0_rdn13: f64 = *var_qgs0_rdn13_slot;
        let mut var_qgs0_rdn14: f64 = *var_qgs0_rdn14_slot;
        let mut var_qgs0_rdn15: f64 = *var_qgs0_rdn15_slot;
        let mut var_qgs0_rdn2: f64 = *var_qgs0_rdn2_slot;
        let mut var_qgs0_rdn3: f64 = *var_qgs0_rdn3_slot;
        let mut var_qgs0_rdn4: f64 = *var_qgs0_rdn4_slot;
        let mut var_qgs0_rdn5: f64 = *var_qgs0_rdn5_slot;
        let mut var_qgs0_rdn6: f64 = *var_qgs0_rdn6_slot;
        let mut var_qgs0_rdn7: f64 = *var_qgs0_rdn7_slot;
        let mut var_qgs0_rdn8: f64 = *var_qgs0_rdn8_slot;
        let mut var_qgs0_rdn9: f64 = *var_qgs0_rdn9_slot;
        let mut var_qgs0_rv: f64 = *var_qgs0_rv_slot;
        let mut var_qgs_db0: f64 = *var_qgs_db0_slot;
        let mut var_qgs_db1: f64 = *var_qgs_db1_slot;
        let mut var_qgs_db10: f64 = *var_qgs_db10_slot;
        let mut var_qgs_db11: f64 = *var_qgs_db11_slot;
        let mut var_qgs_db12: f64 = *var_qgs_db12_slot;
        let mut var_qgs_db13: f64 = *var_qgs_db13_slot;
        let mut var_qgs_db14: f64 = *var_qgs_db14_slot;
        let mut var_qgs_db15: f64 = *var_qgs_db15_slot;
        let mut var_qgs_db16: f64 = *var_qgs_db16_slot;
        let mut var_qgs_db17: f64 = *var_qgs_db17_slot;
        let mut var_qgs_db18: f64 = *var_qgs_db18_slot;
        let mut var_qgs_db2: f64 = *var_qgs_db2_slot;
        let mut var_qgs_db3: f64 = *var_qgs_db3_slot;
        let mut var_qgs_db4: f64 = *var_qgs_db4_slot;
        let mut var_qgs_db5: f64 = *var_qgs_db5_slot;
        let mut var_qgs_db6: f64 = *var_qgs_db6_slot;
        let mut var_qgs_db7: f64 = *var_qgs_db7_slot;
        let mut var_qgs_db8: f64 = *var_qgs_db8_slot;
        let mut var_qgs_db9: f64 = *var_qgs_db9_slot;
        let mut var_qgs_dn0: f64 = *var_qgs_dn0_slot;
        let mut var_qgs_dn1: f64 = *var_qgs_dn1_slot;
        let mut var_qgs_dn10: f64 = *var_qgs_dn10_slot;
        let mut var_qgs_dn11: f64 = *var_qgs_dn11_slot;
        let mut var_qgs_dn12: f64 = *var_qgs_dn12_slot;
        let mut var_qgs_dn13: f64 = *var_qgs_dn13_slot;
        let mut var_qgs_dn14: f64 = *var_qgs_dn14_slot;
        let mut var_qgs_dn15: f64 = *var_qgs_dn15_slot;
        let mut var_qgs_dn2: f64 = *var_qgs_dn2_slot;
        let mut var_qgs_dn3: f64 = *var_qgs_dn3_slot;
        let mut var_qgs_dn4: f64 = *var_qgs_dn4_slot;
        let mut var_qgs_dn5: f64 = *var_qgs_dn5_slot;
        let mut var_qgs_dn6: f64 = *var_qgs_dn6_slot;
        let mut var_qgs_dn7: f64 = *var_qgs_dn7_slot;
        let mut var_qgs_dn8: f64 = *var_qgs_dn8_slot;
        let mut var_qgs_dn9: f64 = *var_qgs_dn9_slot;
        let mut var_qgs_rdb0: f64 = *var_qgs_rdb0_slot;
        let mut var_qgs_rdb1: f64 = *var_qgs_rdb1_slot;
        let mut var_qgs_rdb10: f64 = *var_qgs_rdb10_slot;
        let mut var_qgs_rdb11: f64 = *var_qgs_rdb11_slot;
        let mut var_qgs_rdb12: f64 = *var_qgs_rdb12_slot;
        let mut var_qgs_rdb13: f64 = *var_qgs_rdb13_slot;
        let mut var_qgs_rdb14: f64 = *var_qgs_rdb14_slot;
        let mut var_qgs_rdb15: f64 = *var_qgs_rdb15_slot;
        let mut var_qgs_rdb16: f64 = *var_qgs_rdb16_slot;
        let mut var_qgs_rdb17: f64 = *var_qgs_rdb17_slot;
        let mut var_qgs_rdb18: f64 = *var_qgs_rdb18_slot;
        let mut var_qgs_rdb2: f64 = *var_qgs_rdb2_slot;
        let mut var_qgs_rdb3: f64 = *var_qgs_rdb3_slot;
        let mut var_qgs_rdb4: f64 = *var_qgs_rdb4_slot;
        let mut var_qgs_rdb5: f64 = *var_qgs_rdb5_slot;
        let mut var_qgs_rdb6: f64 = *var_qgs_rdb6_slot;
        let mut var_qgs_rdb7: f64 = *var_qgs_rdb7_slot;
        let mut var_qgs_rdb8: f64 = *var_qgs_rdb8_slot;
        let mut var_qgs_rdb9: f64 = *var_qgs_rdb9_slot;
        let mut var_qgs_rdn0: f64 = *var_qgs_rdn0_slot;
        let mut var_qgs_rdn1: f64 = *var_qgs_rdn1_slot;
        let mut var_qgs_rdn10: f64 = *var_qgs_rdn10_slot;
        let mut var_qgs_rdn11: f64 = *var_qgs_rdn11_slot;
        let mut var_qgs_rdn12: f64 = *var_qgs_rdn12_slot;
        let mut var_qgs_rdn13: f64 = *var_qgs_rdn13_slot;
        let mut var_qgs_rdn14: f64 = *var_qgs_rdn14_slot;
        let mut var_qgs_rdn15: f64 = *var_qgs_rdn15_slot;
        let mut var_qgs_rdn2: f64 = *var_qgs_rdn2_slot;
        let mut var_qgs_rdn3: f64 = *var_qgs_rdn3_slot;
        let mut var_qgs_rdn4: f64 = *var_qgs_rdn4_slot;
        let mut var_qgs_rdn5: f64 = *var_qgs_rdn5_slot;
        let mut var_qgs_rdn6: f64 = *var_qgs_rdn6_slot;
        let mut var_qgs_rdn7: f64 = *var_qgs_rdn7_slot;
        let mut var_qgs_rdn8: f64 = *var_qgs_rdn8_slot;
        let mut var_qgs_rdn9: f64 = *var_qgs_rdn9_slot;
        let mut var_qgs_rv: f64 = *var_qgs_rv_slot;

        let (assign1460_e1858, assign1460_e1858_d_n0, assign1460_e1858_d_n1, assign1460_e1858_d_n2, assign1460_e1858_d_n3, assign1460_e1858_d_n4, assign1460_e1858_d_n5, assign1460_e1858_d_n6, assign1460_e1858_d_n7, assign1460_e1858_d_n8, assign1460_e1858_d_n9, assign1460_e1858_d_n10, assign1460_e1858_d_n11, assign1460_e1858_d_n12, assign1460_e1858_d_n13, assign1460_e1858_d_n14, assign1460_e1858_d_n15, assign1460_e1858_d_b0, assign1460_e1858_d_b1, assign1460_e1858_d_b2, assign1460_e1858_d_b3, assign1460_e1858_d_b4, assign1460_e1858_d_b5, assign1460_e1858_d_b6, assign1460_e1858_d_b7, assign1460_e1858_d_b8, assign1460_e1858_d_b9, assign1460_e1858_d_b10, assign1460_e1858_d_b11, assign1460_e1858_d_b12, assign1460_e1858_d_b13, assign1460_e1858_d_b14,) = {
    if ((var_guard15 != 0.0) && (!((var_guard13 != 0.0) || (var_guard14 != 0.0)))) {
        let assign1460_e1856: f64 = (var_psi_1).cosh();
        (assign1460_e1856, ((var_psi_1).sinh() * var_psi_1_dn0), ((var_psi_1).sinh() * var_psi_1_dn1), ((var_psi_1).sinh() * var_psi_1_dn2), ((var_psi_1).sinh() * var_psi_1_dn3), ((var_psi_1).sinh() * var_psi_1_dn4), ((var_psi_1).sinh() * var_psi_1_dn5), ((var_psi_1).sinh() * var_psi_1_dn6), ((var_psi_1).sinh() * var_psi_1_dn7), ((var_psi_1).sinh() * var_psi_1_dn8), ((var_psi_1).sinh() * var_psi_1_dn9), ((var_psi_1).sinh() * var_psi_1_dn10), ((var_psi_1).sinh() * var_psi_1_dn11), ((var_psi_1).sinh() * var_psi_1_dn12), ((var_psi_1).sinh() * var_psi_1_dn13), ((var_psi_1).sinh() * var_psi_1_dn14), ((var_psi_1).sinh() * var_psi_1_dn15), ((var_psi_1).sinh() * var_psi_1_db0), ((var_psi_1).sinh() * var_psi_1_db1), ((var_psi_1).sinh() * var_psi_1_db2), ((var_psi_1).sinh() * var_psi_1_db3), ((var_psi_1).sinh() * var_psi_1_db4), ((var_psi_1).sinh() * var_psi_1_db5), ((var_psi_1).sinh() * var_psi_1_db6), ((var_psi_1).sinh() * var_psi_1_db7), ((var_psi_1).sinh() * var_psi_1_db8), ((var_psi_1).sinh() * var_psi_1_db9), ((var_psi_1).sinh() * var_psi_1_db10), ((var_psi_1).sinh() * var_psi_1_db11), ((var_psi_1).sinh() * var_psi_1_db12), ((var_psi_1).sinh() * var_psi_1_db13), ((var_psi_1).sinh() * var_psi_1_db14),)
    } else {
        (var_cosh1, var_cosh1_dn0, var_cosh1_dn1, var_cosh1_dn2, var_cosh1_dn3, var_cosh1_dn4, var_cosh1_dn5, var_cosh1_dn6, var_cosh1_dn7, var_cosh1_dn8, var_cosh1_dn9, var_cosh1_dn10, var_cosh1_dn11, var_cosh1_dn12, var_cosh1_dn13, var_cosh1_dn14, var_cosh1_dn15, var_cosh1_db0, var_cosh1_db1, var_cosh1_db2, var_cosh1_db3, var_cosh1_db4, var_cosh1_db5, var_cosh1_db6, var_cosh1_db7, var_cosh1_db8, var_cosh1_db9, var_cosh1_db10, var_cosh1_db11, var_cosh1_db12, var_cosh1_db13, var_cosh1_db14,)
    }
};
        var_cosh1 = assign1460_e1858;
        var_cosh1_dn0 = assign1460_e1858_d_n0;
        var_cosh1_dn1 = assign1460_e1858_d_n1;
        var_cosh1_dn2 = assign1460_e1858_d_n2;
        var_cosh1_dn3 = assign1460_e1858_d_n3;
        var_cosh1_dn4 = assign1460_e1858_d_n4;
        var_cosh1_dn5 = assign1460_e1858_d_n5;
        var_cosh1_dn6 = assign1460_e1858_d_n6;
        var_cosh1_dn7 = assign1460_e1858_d_n7;
        var_cosh1_dn8 = assign1460_e1858_d_n8;
        var_cosh1_dn9 = assign1460_e1858_d_n9;
        var_cosh1_dn10 = assign1460_e1858_d_n10;
        var_cosh1_dn11 = assign1460_e1858_d_n11;
        var_cosh1_dn12 = assign1460_e1858_d_n12;
        var_cosh1_dn13 = assign1460_e1858_d_n13;
        var_cosh1_dn14 = assign1460_e1858_d_n14;
        var_cosh1_dn15 = assign1460_e1858_d_n15;
        var_cosh1_db0 = assign1460_e1858_d_b0;
        var_cosh1_db1 = assign1460_e1858_d_b1;
        var_cosh1_db2 = assign1460_e1858_d_b2;
        var_cosh1_db3 = assign1460_e1858_d_b3;
        var_cosh1_db4 = assign1460_e1858_d_b4;
        var_cosh1_db5 = assign1460_e1858_d_b5;
        var_cosh1_db6 = assign1460_e1858_d_b6;
        var_cosh1_db7 = assign1460_e1858_d_b7;
        var_cosh1_db8 = assign1460_e1858_d_b8;
        var_cosh1_db9 = assign1460_e1858_d_b9;
        var_cosh1_db10 = assign1460_e1858_d_b10;
        var_cosh1_db11 = assign1460_e1858_d_b11;
        var_cosh1_db12 = assign1460_e1858_d_b12;
        var_cosh1_db13 = assign1460_e1858_d_b13;
        var_cosh1_db14 = assign1460_e1858_d_b14;
        var_cosh1_db15 = 0.0;
        var_cosh1_db16 = 0.0;
        var_cosh1_db17 = 0.0;
        var_cosh1_db18 = 0.0;
        var_cosh1_rv = 0.0;
        var_cosh1_rdn0 = 0.0;
        var_cosh1_rdn1 = 0.0;
        var_cosh1_rdn2 = 0.0;
        var_cosh1_rdn3 = 0.0;
        var_cosh1_rdn4 = 0.0;
        var_cosh1_rdn5 = 0.0;
        var_cosh1_rdn6 = 0.0;
        var_cosh1_rdn7 = 0.0;
        var_cosh1_rdn8 = 0.0;
        var_cosh1_rdn9 = 0.0;
        var_cosh1_rdn10 = 0.0;
        var_cosh1_rdn11 = 0.0;
        var_cosh1_rdn12 = 0.0;
        var_cosh1_rdn13 = 0.0;
        var_cosh1_rdn14 = 0.0;
        var_cosh1_rdn15 = 0.0;
        var_cosh1_rdb0 = 0.0;
        var_cosh1_rdb1 = 0.0;
        var_cosh1_rdb2 = 0.0;
        var_cosh1_rdb3 = 0.0;
        var_cosh1_rdb4 = 0.0;
        var_cosh1_rdb5 = 0.0;
        var_cosh1_rdb6 = 0.0;
        var_cosh1_rdb7 = 0.0;
        var_cosh1_rdb8 = 0.0;
        var_cosh1_rdb9 = 0.0;
        var_cosh1_rdb10 = 0.0;
        var_cosh1_rdb11 = 0.0;
        var_cosh1_rdb12 = 0.0;
        var_cosh1_rdb13 = 0.0;
        var_cosh1_rdb14 = 0.0;
        var_cosh1_rdb15 = 0.0;
        var_cosh1_rdb16 = 0.0;
        var_cosh1_rdb17 = 0.0;
        var_cosh1_rdb18 = 0.0;

        let (assign1470_e1868, assign1470_e1868_d_n0, assign1470_e1868_d_n1, assign1470_e1868_d_n2, assign1470_e1868_d_n3, assign1470_e1868_d_n4, assign1470_e1868_d_n5, assign1470_e1868_d_n6, assign1470_e1868_d_n7, assign1470_e1868_d_n8, assign1470_e1868_d_n9, assign1470_e1868_d_n10, assign1470_e1868_d_n11, assign1470_e1868_d_n12, assign1470_e1868_d_n13, assign1470_e1868_d_n14, assign1470_e1868_d_n15, assign1470_e1868_d_b0, assign1470_e1868_d_b1, assign1470_e1868_d_b2, assign1470_e1868_d_b3, assign1470_e1868_d_b4, assign1470_e1868_d_b5, assign1470_e1868_d_b6, assign1470_e1868_d_b7, assign1470_e1868_d_b8, assign1470_e1868_d_b9, assign1470_e1868_d_b10, assign1470_e1868_d_b11, assign1470_e1868_d_b12, assign1470_e1868_d_b13, assign1470_e1868_d_b14,) = {
    if ((var_guard15 != 0.0) && (!((var_guard13 != 0.0) || (var_guard14 != 0.0)))) {
        let assign1470_e1866: f64 = (var_cosh1).ln();
        (assign1470_e1866, (var_cosh1_dn0 / var_cosh1), (var_cosh1_dn1 / var_cosh1), (var_cosh1_dn2 / var_cosh1), (var_cosh1_dn3 / var_cosh1), (var_cosh1_dn4 / var_cosh1), (var_cosh1_dn5 / var_cosh1), (var_cosh1_dn6 / var_cosh1), (var_cosh1_dn7 / var_cosh1), (var_cosh1_dn8 / var_cosh1), (var_cosh1_dn9 / var_cosh1), (var_cosh1_dn10 / var_cosh1), (var_cosh1_dn11 / var_cosh1), (var_cosh1_dn12 / var_cosh1), (var_cosh1_dn13 / var_cosh1), (var_cosh1_dn14 / var_cosh1), (var_cosh1_dn15 / var_cosh1), (var_cosh1_db0 / var_cosh1), (var_cosh1_db1 / var_cosh1), (var_cosh1_db2 / var_cosh1), (var_cosh1_db3 / var_cosh1), (var_cosh1_db4 / var_cosh1), (var_cosh1_db5 / var_cosh1), (var_cosh1_db6 / var_cosh1), (var_cosh1_db7 / var_cosh1), (var_cosh1_db8 / var_cosh1), (var_cosh1_db9 / var_cosh1), (var_cosh1_db10 / var_cosh1), (var_cosh1_db11 / var_cosh1), (var_cosh1_db12 / var_cosh1), (var_cosh1_db13 / var_cosh1), (var_cosh1_db14 / var_cosh1),)
    } else {
        (var_lc1, var_lc1_dn0, var_lc1_dn1, var_lc1_dn2, var_lc1_dn3, var_lc1_dn4, var_lc1_dn5, var_lc1_dn6, var_lc1_dn7, var_lc1_dn8, var_lc1_dn9, var_lc1_dn10, var_lc1_dn11, var_lc1_dn12, var_lc1_dn13, var_lc1_dn14, var_lc1_dn15, var_lc1_db0, var_lc1_db1, var_lc1_db2, var_lc1_db3, var_lc1_db4, var_lc1_db5, var_lc1_db6, var_lc1_db7, var_lc1_db8, var_lc1_db9, var_lc1_db10, var_lc1_db11, var_lc1_db12, var_lc1_db13, var_lc1_db14,)
    }
};
        var_lc1 = assign1470_e1868;
        var_lc1_dn0 = assign1470_e1868_d_n0;
        var_lc1_dn1 = assign1470_e1868_d_n1;
        var_lc1_dn2 = assign1470_e1868_d_n2;
        var_lc1_dn3 = assign1470_e1868_d_n3;
        var_lc1_dn4 = assign1470_e1868_d_n4;
        var_lc1_dn5 = assign1470_e1868_d_n5;
        var_lc1_dn6 = assign1470_e1868_d_n6;
        var_lc1_dn7 = assign1470_e1868_d_n7;
        var_lc1_dn8 = assign1470_e1868_d_n8;
        var_lc1_dn9 = assign1470_e1868_d_n9;
        var_lc1_dn10 = assign1470_e1868_d_n10;
        var_lc1_dn11 = assign1470_e1868_d_n11;
        var_lc1_dn12 = assign1470_e1868_d_n12;
        var_lc1_dn13 = assign1470_e1868_d_n13;
        var_lc1_dn14 = assign1470_e1868_d_n14;
        var_lc1_dn15 = assign1470_e1868_d_n15;
        var_lc1_db0 = assign1470_e1868_d_b0;
        var_lc1_db1 = assign1470_e1868_d_b1;
        var_lc1_db2 = assign1470_e1868_d_b2;
        var_lc1_db3 = assign1470_e1868_d_b3;
        var_lc1_db4 = assign1470_e1868_d_b4;
        var_lc1_db5 = assign1470_e1868_d_b5;
        var_lc1_db6 = assign1470_e1868_d_b6;
        var_lc1_db7 = assign1470_e1868_d_b7;
        var_lc1_db8 = assign1470_e1868_d_b8;
        var_lc1_db9 = assign1470_e1868_d_b9;
        var_lc1_db10 = assign1470_e1868_d_b10;
        var_lc1_db11 = assign1470_e1868_d_b11;
        var_lc1_db12 = assign1470_e1868_d_b12;
        var_lc1_db13 = assign1470_e1868_d_b13;
        var_lc1_db14 = assign1470_e1868_d_b14;
        var_lc1_db15 = 0.0;
        var_lc1_db16 = 0.0;
        var_lc1_db17 = 0.0;
        var_lc1_db18 = 0.0;
        var_lc1_rv = 0.0;
        var_lc1_rdn0 = 0.0;
        var_lc1_rdn1 = 0.0;
        var_lc1_rdn2 = 0.0;
        var_lc1_rdn3 = 0.0;
        var_lc1_rdn4 = 0.0;
        var_lc1_rdn5 = 0.0;
        var_lc1_rdn6 = 0.0;
        var_lc1_rdn7 = 0.0;
        var_lc1_rdn8 = 0.0;
        var_lc1_rdn9 = 0.0;
        var_lc1_rdn10 = 0.0;
        var_lc1_rdn11 = 0.0;
        var_lc1_rdn12 = 0.0;
        var_lc1_rdn13 = 0.0;
        var_lc1_rdn14 = 0.0;
        var_lc1_rdn15 = 0.0;
        var_lc1_rdb0 = 0.0;
        var_lc1_rdb1 = 0.0;
        var_lc1_rdb2 = 0.0;
        var_lc1_rdb3 = 0.0;
        var_lc1_rdb4 = 0.0;
        var_lc1_rdb5 = 0.0;
        var_lc1_rdb6 = 0.0;
        var_lc1_rdb7 = 0.0;
        var_lc1_rdb8 = 0.0;
        var_lc1_rdb9 = 0.0;
        var_lc1_rdb10 = 0.0;
        var_lc1_rdb11 = 0.0;
        var_lc1_rdb12 = 0.0;
        var_lc1_rdb13 = 0.0;
        var_lc1_rdb14 = 0.0;
        var_lc1_rdb15 = 0.0;
        var_lc1_rdb16 = 0.0;
        var_lc1_rdb17 = 0.0;
        var_lc1_rdb18 = 0.0;

        let (assign1480_e1883, assign1480_e1883_d_n0, assign1480_e1883_d_n1, assign1480_e1883_d_n2, assign1480_e1883_d_n3, assign1480_e1883_d_n4, assign1480_e1883_d_n5, assign1480_e1883_d_n6, assign1480_e1883_d_n7, assign1480_e1883_d_n8, assign1480_e1883_d_n9, assign1480_e1883_d_n10, assign1480_e1883_d_n11, assign1480_e1883_d_n12, assign1480_e1883_d_n13, assign1480_e1883_d_n14, assign1480_e1883_d_n15, assign1480_e1883_d_b0, assign1480_e1883_d_b1, assign1480_e1883_d_b2, assign1480_e1883_d_b3, assign1480_e1883_d_b4, assign1480_e1883_d_b5, assign1480_e1883_d_b6, assign1480_e1883_d_b7, assign1480_e1883_d_b8, assign1480_e1883_d_b9, assign1480_e1883_d_b10, assign1480_e1883_d_b11, assign1480_e1883_d_b12, assign1480_e1883_d_b13, assign1480_e1883_d_b14,) = {
    if ((var_guard15 != 0.0) && (!((var_guard13 != 0.0) || (var_guard14 != 0.0)))) {
        let assign1480_e1878: f64 = (p.p37 * var_vds);
        let assign1480_e1879: f64 = (var_p10_t + assign1480_e1878);
        let assign1480_e1881: f64 = (assign1480_e1879 + var_lc10);
        (assign1480_e1881, ((var_p10_t_dn0 + (p.p37 * var_vds_dn0)) + var_lc10_dn0), ((var_p10_t_dn1 + (p.p37 * var_vds_dn1)) + var_lc10_dn1), ((var_p10_t_dn2 + (p.p37 * var_vds_dn2)) + var_lc10_dn2), ((var_p10_t_dn3 + (p.p37 * var_vds_dn3)) + var_lc10_dn3), ((var_p10_t_dn4 + (p.p37 * var_vds_dn4)) + var_lc10_dn4), ((var_p10_t_dn5 + (p.p37 * var_vds_dn5)) + var_lc10_dn5), ((var_p10_t_dn6 + (p.p37 * var_vds_dn6)) + var_lc10_dn6), ((var_p10_t_dn7 + (p.p37 * var_vds_dn7)) + var_lc10_dn7), ((var_p10_t_dn8 + (p.p37 * var_vds_dn8)) + var_lc10_dn8), ((var_p10_t_dn9 + (p.p37 * var_vds_dn9)) + var_lc10_dn9), ((var_p10_t_dn10 + (p.p37 * var_vds_dn10)) + var_lc10_dn10), ((var_p10_t_dn11 + (p.p37 * var_vds_dn11)) + var_lc10_dn11), ((var_p10_t_dn12 + (p.p37 * var_vds_dn12)) + var_lc10_dn12), ((var_p10_t_dn13 + (p.p37 * var_vds_dn13)) + var_lc10_dn13), ((var_p10_t_dn14 + (p.p37 * var_vds_dn14)) + var_lc10_dn14), ((var_p10_t_dn15 + (p.p37 * var_vds_dn15)) + var_lc10_dn15), ((var_p10_t_db0 + (p.p37 * var_vds_db0)) + var_lc10_db0), ((var_p10_t_db1 + (p.p37 * var_vds_db1)) + var_lc10_db1), ((var_p10_t_db2 + (p.p37 * var_vds_db2)) + var_lc10_db2), ((var_p10_t_db3 + (p.p37 * var_vds_db3)) + var_lc10_db3), ((var_p10_t_db4 + (p.p37 * var_vds_db4)) + var_lc10_db4), ((var_p10_t_db5 + (p.p37 * var_vds_db5)) + var_lc10_db5), ((var_p10_t_db6 + (p.p37 * var_vds_db6)) + var_lc10_db6), ((var_p10_t_db7 + (p.p37 * var_vds_db7)) + var_lc10_db7), ((var_p10_t_db8 + (p.p37 * var_vds_db8)) + var_lc10_db8), ((var_p10_t_db9 + (p.p37 * var_vds_db9)) + var_lc10_db9), ((var_p10_t_db10 + (p.p37 * var_vds_db10)) + var_lc10_db10), ((var_p10_t_db11 + (p.p37 * var_vds_db11)) + var_lc10_db11), ((var_p10_t_db12 + (p.p37 * var_vds_db12)) + var_lc10_db12), ((var_p10_t_db13 + (p.p37 * var_vds_db13)) + var_lc10_db13), ((var_p10_t_db14 + (p.p37 * var_vds_db14)) + var_lc10_db14),)
    } else {
        (var_qgs0, var_qgs0_dn0, var_qgs0_dn1, var_qgs0_dn2, var_qgs0_dn3, var_qgs0_dn4, var_qgs0_dn5, var_qgs0_dn6, var_qgs0_dn7, var_qgs0_dn8, var_qgs0_dn9, var_qgs0_dn10, var_qgs0_dn11, var_qgs0_dn12, var_qgs0_dn13, var_qgs0_dn14, var_qgs0_dn15, var_qgs0_db0, var_qgs0_db1, var_qgs0_db2, var_qgs0_db3, var_qgs0_db4, var_qgs0_db5, var_qgs0_db6, var_qgs0_db7, var_qgs0_db8, var_qgs0_db9, var_qgs0_db10, var_qgs0_db11, var_qgs0_db12, var_qgs0_db13, var_qgs0_db14,)
    }
};
        var_qgs0 = assign1480_e1883;
        var_qgs0_dn0 = assign1480_e1883_d_n0;
        var_qgs0_dn1 = assign1480_e1883_d_n1;
        var_qgs0_dn2 = assign1480_e1883_d_n2;
        var_qgs0_dn3 = assign1480_e1883_d_n3;
        var_qgs0_dn4 = assign1480_e1883_d_n4;
        var_qgs0_dn5 = assign1480_e1883_d_n5;
        var_qgs0_dn6 = assign1480_e1883_d_n6;
        var_qgs0_dn7 = assign1480_e1883_d_n7;
        var_qgs0_dn8 = assign1480_e1883_d_n8;
        var_qgs0_dn9 = assign1480_e1883_d_n9;
        var_qgs0_dn10 = assign1480_e1883_d_n10;
        var_qgs0_dn11 = assign1480_e1883_d_n11;
        var_qgs0_dn12 = assign1480_e1883_d_n12;
        var_qgs0_dn13 = assign1480_e1883_d_n13;
        var_qgs0_dn14 = assign1480_e1883_d_n14;
        var_qgs0_dn15 = assign1480_e1883_d_n15;
        var_qgs0_db0 = assign1480_e1883_d_b0;
        var_qgs0_db1 = assign1480_e1883_d_b1;
        var_qgs0_db2 = assign1480_e1883_d_b2;
        var_qgs0_db3 = assign1480_e1883_d_b3;
        var_qgs0_db4 = assign1480_e1883_d_b4;
        var_qgs0_db5 = assign1480_e1883_d_b5;
        var_qgs0_db6 = assign1480_e1883_d_b6;
        var_qgs0_db7 = assign1480_e1883_d_b7;
        var_qgs0_db8 = assign1480_e1883_d_b8;
        var_qgs0_db9 = assign1480_e1883_d_b9;
        var_qgs0_db10 = assign1480_e1883_d_b10;
        var_qgs0_db11 = assign1480_e1883_d_b11;
        var_qgs0_db12 = assign1480_e1883_d_b12;
        var_qgs0_db13 = assign1480_e1883_d_b13;
        var_qgs0_db14 = assign1480_e1883_d_b14;
        var_qgs0_db15 = 0.0;
        var_qgs0_db16 = 0.0;
        var_qgs0_db17 = 0.0;
        var_qgs0_db18 = 0.0;
        var_qgs0_rv = 0.0;
        var_qgs0_rdn0 = 0.0;
        var_qgs0_rdn1 = 0.0;
        var_qgs0_rdn2 = 0.0;
        var_qgs0_rdn3 = 0.0;
        var_qgs0_rdn4 = 0.0;
        var_qgs0_rdn5 = 0.0;
        var_qgs0_rdn6 = 0.0;
        var_qgs0_rdn7 = 0.0;
        var_qgs0_rdn8 = 0.0;
        var_qgs0_rdn9 = 0.0;
        var_qgs0_rdn10 = 0.0;
        var_qgs0_rdn11 = 0.0;
        var_qgs0_rdn12 = 0.0;
        var_qgs0_rdn13 = 0.0;
        var_qgs0_rdn14 = 0.0;
        var_qgs0_rdn15 = 0.0;
        var_qgs0_rdb0 = 0.0;
        var_qgs0_rdb1 = 0.0;
        var_qgs0_rdb2 = 0.0;
        var_qgs0_rdb3 = 0.0;
        var_qgs0_rdb4 = 0.0;
        var_qgs0_rdb5 = 0.0;
        var_qgs0_rdb6 = 0.0;
        var_qgs0_rdb7 = 0.0;
        var_qgs0_rdb8 = 0.0;
        var_qgs0_rdb9 = 0.0;
        var_qgs0_rdb10 = 0.0;
        var_qgs0_rdb11 = 0.0;
        var_qgs0_rdb12 = 0.0;
        var_qgs0_rdb13 = 0.0;
        var_qgs0_rdb14 = 0.0;
        var_qgs0_rdb15 = 0.0;
        var_qgs0_rdb16 = 0.0;
        var_qgs0_rdb17 = 0.0;
        var_qgs0_rdb18 = 0.0;

        let (assign1490_e1912, assign1490_e1912_d_n0, assign1490_e1912_d_n1, assign1490_e1912_d_n2, assign1490_e1912_d_n3, assign1490_e1912_d_n4, assign1490_e1912_d_n5, assign1490_e1912_d_n6, assign1490_e1912_d_n7, assign1490_e1912_d_n8, assign1490_e1912_d_n9, assign1490_e1912_d_n10, assign1490_e1912_d_n11, assign1490_e1912_d_n12, assign1490_e1912_d_n13, assign1490_e1912_d_n14, assign1490_e1912_d_n15, assign1490_e1912_d_b0, assign1490_e1912_d_b1, assign1490_e1912_d_b2, assign1490_e1912_d_b3, assign1490_e1912_d_b4, assign1490_e1912_d_b5, assign1490_e1912_d_b6, assign1490_e1912_d_b7, assign1490_e1912_d_b8, assign1490_e1912_d_b9, assign1490_e1912_d_b10, assign1490_e1912_d_b11, assign1490_e1912_d_b12, assign1490_e1912_d_b13, assign1490_e1912_d_b14,) = {
    if ((var_guard15 != 0.0) && (!((var_guard13 != 0.0) || (var_guard14 != 0.0)))) {
        let assign1490_e1893: f64 = (var_psi_1 + var_lc1);
        let assign1490_e1895: f64 = (assign1490_e1893 - var_qgs0);
        let assign1490_e1897: f64 = (assign1490_e1895 * var_tanh2);
        let assign1490_e1899: f64 = (assign1490_e1897 / p.p30);
        let assign1490_e1902: f64 = (2.0 * p.p37);
        let assign1490_e1904: f64 = (assign1490_e1902 * var_vgsc);
        let assign1490_e1905: f64 = (assign1490_e1899 + assign1490_e1904);
        let assign1490_e1906: f64 = (var_cgs0_t * assign1490_e1905);
        let assign1490_e1909: f64 = (p.p24 * var_vgsc);
        let assign1490_e1910: f64 = (assign1490_e1906 + assign1490_e1909);
        (assign1490_e1910, (((var_cgs0_t_dn0 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_dn0 + var_lc1_dn0) - var_qgs0_dn0) * var_tanh2) + (assign1490_e1895 * var_tanh2_dn0)) / p.p30) + (assign1490_e1902 * var_vgsc_dn0)))) + (p.p24 * var_vgsc_dn0)), (((var_cgs0_t_dn1 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_dn1 + var_lc1_dn1) - var_qgs0_dn1) * var_tanh2) + (assign1490_e1895 * var_tanh2_dn1)) / p.p30) + (assign1490_e1902 * var_vgsc_dn1)))) + (p.p24 * var_vgsc_dn1)), (((var_cgs0_t_dn2 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_dn2 + var_lc1_dn2) - var_qgs0_dn2) * var_tanh2) + (assign1490_e1895 * var_tanh2_dn2)) / p.p30) + (assign1490_e1902 * var_vgsc_dn2)))) + (p.p24 * var_vgsc_dn2)), (((var_cgs0_t_dn3 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_dn3 + var_lc1_dn3) - var_qgs0_dn3) * var_tanh2) + (assign1490_e1895 * var_tanh2_dn3)) / p.p30) + (assign1490_e1902 * var_vgsc_dn3)))) + (p.p24 * var_vgsc_dn3)), (((var_cgs0_t_dn4 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_dn4 + var_lc1_dn4) - var_qgs0_dn4) * var_tanh2) + (assign1490_e1895 * var_tanh2_dn4)) / p.p30) + (assign1490_e1902 * var_vgsc_dn4)))) + (p.p24 * var_vgsc_dn4)), (((var_cgs0_t_dn5 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_dn5 + var_lc1_dn5) - var_qgs0_dn5) * var_tanh2) + (assign1490_e1895 * var_tanh2_dn5)) / p.p30) + (assign1490_e1902 * var_vgsc_dn5)))) + (p.p24 * var_vgsc_dn5)), (((var_cgs0_t_dn6 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_dn6 + var_lc1_dn6) - var_qgs0_dn6) * var_tanh2) + (assign1490_e1895 * var_tanh2_dn6)) / p.p30) + (assign1490_e1902 * var_vgsc_dn6)))) + (p.p24 * var_vgsc_dn6)), (((var_cgs0_t_dn7 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_dn7 + var_lc1_dn7) - var_qgs0_dn7) * var_tanh2) + (assign1490_e1895 * var_tanh2_dn7)) / p.p30) + (assign1490_e1902 * var_vgsc_dn7)))) + (p.p24 * var_vgsc_dn7)), (((var_cgs0_t_dn8 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_dn8 + var_lc1_dn8) - var_qgs0_dn8) * var_tanh2) + (assign1490_e1895 * var_tanh2_dn8)) / p.p30) + (assign1490_e1902 * var_vgsc_dn8)))) + (p.p24 * var_vgsc_dn8)), (((var_cgs0_t_dn9 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_dn9 + var_lc1_dn9) - var_qgs0_dn9) * var_tanh2) + (assign1490_e1895 * var_tanh2_dn9)) / p.p30) + (assign1490_e1902 * var_vgsc_dn9)))) + (p.p24 * var_vgsc_dn9)), (((var_cgs0_t_dn10 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_dn10 + var_lc1_dn10) - var_qgs0_dn10) * var_tanh2) + (assign1490_e1895 * var_tanh2_dn10)) / p.p30) + (assign1490_e1902 * var_vgsc_dn10)))) + (p.p24 * var_vgsc_dn10)), (((var_cgs0_t_dn11 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_dn11 + var_lc1_dn11) - var_qgs0_dn11) * var_tanh2) + (assign1490_e1895 * var_tanh2_dn11)) / p.p30) + (assign1490_e1902 * var_vgsc_dn11)))) + (p.p24 * var_vgsc_dn11)), (((var_cgs0_t_dn12 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_dn12 + var_lc1_dn12) - var_qgs0_dn12) * var_tanh2) + (assign1490_e1895 * var_tanh2_dn12)) / p.p30) + (assign1490_e1902 * var_vgsc_dn12)))) + (p.p24 * var_vgsc_dn12)), (((var_cgs0_t_dn13 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_dn13 + var_lc1_dn13) - var_qgs0_dn13) * var_tanh2) + (assign1490_e1895 * var_tanh2_dn13)) / p.p30) + (assign1490_e1902 * var_vgsc_dn13)))) + (p.p24 * var_vgsc_dn13)), (((var_cgs0_t_dn14 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_dn14 + var_lc1_dn14) - var_qgs0_dn14) * var_tanh2) + (assign1490_e1895 * var_tanh2_dn14)) / p.p30) + (assign1490_e1902 * var_vgsc_dn14)))) + (p.p24 * var_vgsc_dn14)), (((var_cgs0_t_dn15 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_dn15 + var_lc1_dn15) - var_qgs0_dn15) * var_tanh2) + (assign1490_e1895 * var_tanh2_dn15)) / p.p30) + (assign1490_e1902 * var_vgsc_dn15)))) + (p.p24 * var_vgsc_dn15)), (((var_cgs0_t_db0 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_db0 + var_lc1_db0) - var_qgs0_db0) * var_tanh2) + (assign1490_e1895 * var_tanh2_db0)) / p.p30) + (assign1490_e1902 * var_vgsc_db0)))) + (p.p24 * var_vgsc_db0)), (((var_cgs0_t_db1 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_db1 + var_lc1_db1) - var_qgs0_db1) * var_tanh2) + (assign1490_e1895 * var_tanh2_db1)) / p.p30) + (assign1490_e1902 * var_vgsc_db1)))) + (p.p24 * var_vgsc_db1)), (((var_cgs0_t_db2 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_db2 + var_lc1_db2) - var_qgs0_db2) * var_tanh2) + (assign1490_e1895 * var_tanh2_db2)) / p.p30) + (assign1490_e1902 * var_vgsc_db2)))) + (p.p24 * var_vgsc_db2)), (((var_cgs0_t_db3 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_db3 + var_lc1_db3) - var_qgs0_db3) * var_tanh2) + (assign1490_e1895 * var_tanh2_db3)) / p.p30) + (assign1490_e1902 * var_vgsc_db3)))) + (p.p24 * var_vgsc_db3)), (((var_cgs0_t_db4 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_db4 + var_lc1_db4) - var_qgs0_db4) * var_tanh2) + (assign1490_e1895 * var_tanh2_db4)) / p.p30) + (assign1490_e1902 * var_vgsc_db4)))) + (p.p24 * var_vgsc_db4)), (((var_cgs0_t_db5 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_db5 + var_lc1_db5) - var_qgs0_db5) * var_tanh2) + (assign1490_e1895 * var_tanh2_db5)) / p.p30) + (assign1490_e1902 * var_vgsc_db5)))) + (p.p24 * var_vgsc_db5)), (((var_cgs0_t_db6 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_db6 + var_lc1_db6) - var_qgs0_db6) * var_tanh2) + (assign1490_e1895 * var_tanh2_db6)) / p.p30) + (assign1490_e1902 * var_vgsc_db6)))) + (p.p24 * var_vgsc_db6)), (((var_cgs0_t_db7 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_db7 + var_lc1_db7) - var_qgs0_db7) * var_tanh2) + (assign1490_e1895 * var_tanh2_db7)) / p.p30) + (assign1490_e1902 * var_vgsc_db7)))) + (p.p24 * var_vgsc_db7)), (((var_cgs0_t_db8 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_db8 + var_lc1_db8) - var_qgs0_db8) * var_tanh2) + (assign1490_e1895 * var_tanh2_db8)) / p.p30) + (assign1490_e1902 * var_vgsc_db8)))) + (p.p24 * var_vgsc_db8)), (((var_cgs0_t_db9 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_db9 + var_lc1_db9) - var_qgs0_db9) * var_tanh2) + (assign1490_e1895 * var_tanh2_db9)) / p.p30) + (assign1490_e1902 * var_vgsc_db9)))) + (p.p24 * var_vgsc_db9)), (((var_cgs0_t_db10 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_db10 + var_lc1_db10) - var_qgs0_db10) * var_tanh2) + (assign1490_e1895 * var_tanh2_db10)) / p.p30) + (assign1490_e1902 * var_vgsc_db10)))) + (p.p24 * var_vgsc_db10)), (((var_cgs0_t_db11 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_db11 + var_lc1_db11) - var_qgs0_db11) * var_tanh2) + (assign1490_e1895 * var_tanh2_db11)) / p.p30) + (assign1490_e1902 * var_vgsc_db11)))) + (p.p24 * var_vgsc_db11)), (((var_cgs0_t_db12 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_db12 + var_lc1_db12) - var_qgs0_db12) * var_tanh2) + (assign1490_e1895 * var_tanh2_db12)) / p.p30) + (assign1490_e1902 * var_vgsc_db12)))) + (p.p24 * var_vgsc_db12)), (((var_cgs0_t_db13 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_db13 + var_lc1_db13) - var_qgs0_db13) * var_tanh2) + (assign1490_e1895 * var_tanh2_db13)) / p.p30) + (assign1490_e1902 * var_vgsc_db13)))) + (p.p24 * var_vgsc_db13)), (((var_cgs0_t_db14 * assign1490_e1905) + (var_cgs0_t * ((((((var_psi_1_db14 + var_lc1_db14) - var_qgs0_db14) * var_tanh2) + (assign1490_e1895 * var_tanh2_db14)) / p.p30) + (assign1490_e1902 * var_vgsc_db14)))) + (p.p24 * var_vgsc_db14)),)
    } else {
        (var_qgs, var_qgs_dn0, var_qgs_dn1, var_qgs_dn2, var_qgs_dn3, var_qgs_dn4, var_qgs_dn5, var_qgs_dn6, var_qgs_dn7, var_qgs_dn8, var_qgs_dn9, var_qgs_dn10, var_qgs_dn11, var_qgs_dn12, var_qgs_dn13, var_qgs_dn14, var_qgs_dn15, var_qgs_db0, var_qgs_db1, var_qgs_db2, var_qgs_db3, var_qgs_db4, var_qgs_db5, var_qgs_db6, var_qgs_db7, var_qgs_db8, var_qgs_db9, var_qgs_db10, var_qgs_db11, var_qgs_db12, var_qgs_db13, var_qgs_db14,)
    }
};
        var_qgs = assign1490_e1912;
        var_qgs_dn0 = assign1490_e1912_d_n0;
        var_qgs_dn1 = assign1490_e1912_d_n1;
        var_qgs_dn2 = assign1490_e1912_d_n2;
        var_qgs_dn3 = assign1490_e1912_d_n3;
        var_qgs_dn4 = assign1490_e1912_d_n4;
        var_qgs_dn5 = assign1490_e1912_d_n5;
        var_qgs_dn6 = assign1490_e1912_d_n6;
        var_qgs_dn7 = assign1490_e1912_d_n7;
        var_qgs_dn8 = assign1490_e1912_d_n8;
        var_qgs_dn9 = assign1490_e1912_d_n9;
        var_qgs_dn10 = assign1490_e1912_d_n10;
        var_qgs_dn11 = assign1490_e1912_d_n11;
        var_qgs_dn12 = assign1490_e1912_d_n12;
        var_qgs_dn13 = assign1490_e1912_d_n13;
        var_qgs_dn14 = assign1490_e1912_d_n14;
        var_qgs_dn15 = assign1490_e1912_d_n15;
        var_qgs_db0 = assign1490_e1912_d_b0;
        var_qgs_db1 = assign1490_e1912_d_b1;
        var_qgs_db2 = assign1490_e1912_d_b2;
        var_qgs_db3 = assign1490_e1912_d_b3;
        var_qgs_db4 = assign1490_e1912_d_b4;
        var_qgs_db5 = assign1490_e1912_d_b5;
        var_qgs_db6 = assign1490_e1912_d_b6;
        var_qgs_db7 = assign1490_e1912_d_b7;
        var_qgs_db8 = assign1490_e1912_d_b8;
        var_qgs_db9 = assign1490_e1912_d_b9;
        var_qgs_db10 = assign1490_e1912_d_b10;
        var_qgs_db11 = assign1490_e1912_d_b11;
        var_qgs_db12 = assign1490_e1912_d_b12;
        var_qgs_db13 = assign1490_e1912_d_b13;
        var_qgs_db14 = assign1490_e1912_d_b14;
        var_qgs_db15 = 0.0;
        var_qgs_db16 = 0.0;
        var_qgs_db17 = 0.0;
        var_qgs_db18 = 0.0;
        var_qgs_rv = 0.0;
        var_qgs_rdn0 = 0.0;
        var_qgs_rdn1 = 0.0;
        var_qgs_rdn2 = 0.0;
        var_qgs_rdn3 = 0.0;
        var_qgs_rdn4 = 0.0;
        var_qgs_rdn5 = 0.0;
        var_qgs_rdn6 = 0.0;
        var_qgs_rdn7 = 0.0;
        var_qgs_rdn8 = 0.0;
        var_qgs_rdn9 = 0.0;
        var_qgs_rdn10 = 0.0;
        var_qgs_rdn11 = 0.0;
        var_qgs_rdn12 = 0.0;
        var_qgs_rdn13 = 0.0;
        var_qgs_rdn14 = 0.0;
        var_qgs_rdn15 = 0.0;
        var_qgs_rdb0 = 0.0;
        var_qgs_rdb1 = 0.0;
        var_qgs_rdb2 = 0.0;
        var_qgs_rdb3 = 0.0;
        var_qgs_rdb4 = 0.0;
        var_qgs_rdb5 = 0.0;
        var_qgs_rdb6 = 0.0;
        var_qgs_rdb7 = 0.0;
        var_qgs_rdb8 = 0.0;
        var_qgs_rdb9 = 0.0;
        var_qgs_rdb10 = 0.0;
        var_qgs_rdb11 = 0.0;
        var_qgs_rdb12 = 0.0;
        var_qgs_rdb13 = 0.0;
        var_qgs_rdb14 = 0.0;
        var_qgs_rdb15 = 0.0;
        var_qgs_rdb16 = 0.0;
        var_qgs_rdb17 = 0.0;
        var_qgs_rdb18 = 0.0;

        let (assign1500_e1926, assign1500_e1926_d_n0, assign1500_e1926_d_n1, assign1500_e1926_d_n2, assign1500_e1926_d_n3, assign1500_e1926_d_n4, assign1500_e1926_d_n5, assign1500_e1926_d_n6, assign1500_e1926_d_n7, assign1500_e1926_d_n8, assign1500_e1926_d_n9, assign1500_e1926_d_n10, assign1500_e1926_d_n11, assign1500_e1926_d_n12, assign1500_e1926_d_n13, assign1500_e1926_d_n14, assign1500_e1926_d_n15, assign1500_e1926_d_b0, assign1500_e1926_d_b1, assign1500_e1926_d_b2, assign1500_e1926_d_b3, assign1500_e1926_d_b4, assign1500_e1926_d_b5, assign1500_e1926_d_b6, assign1500_e1926_d_b7, assign1500_e1926_d_b8, assign1500_e1926_d_b9, assign1500_e1926_d_b10, assign1500_e1926_d_b11, assign1500_e1926_d_b12, assign1500_e1926_d_b13, assign1500_e1926_d_b14,) = {
    if ((var_guard15 != 0.0) && (!((var_guard13 != 0.0) || (var_guard14 != 0.0)))) {
        let assign1500_e1922: f64 = (p.p37 * var_vds);
        let assign1500_e1923: f64 = (var_p40_t - assign1500_e1922);
        let assign1500_e1924: f64 = (assign1500_e1923).cosh();
        (assign1500_e1924, ((assign1500_e1923).sinh() * (var_p40_t_dn0 - (p.p37 * var_vds_dn0))), ((assign1500_e1923).sinh() * (var_p40_t_dn1 - (p.p37 * var_vds_dn1))), ((assign1500_e1923).sinh() * (var_p40_t_dn2 - (p.p37 * var_vds_dn2))), ((assign1500_e1923).sinh() * (var_p40_t_dn3 - (p.p37 * var_vds_dn3))), ((assign1500_e1923).sinh() * (var_p40_t_dn4 - (p.p37 * var_vds_dn4))), ((assign1500_e1923).sinh() * (var_p40_t_dn5 - (p.p37 * var_vds_dn5))), ((assign1500_e1923).sinh() * (var_p40_t_dn6 - (p.p37 * var_vds_dn6))), ((assign1500_e1923).sinh() * (var_p40_t_dn7 - (p.p37 * var_vds_dn7))), ((assign1500_e1923).sinh() * (var_p40_t_dn8 - (p.p37 * var_vds_dn8))), ((assign1500_e1923).sinh() * (var_p40_t_dn9 - (p.p37 * var_vds_dn9))), ((assign1500_e1923).sinh() * (var_p40_t_dn10 - (p.p37 * var_vds_dn10))), ((assign1500_e1923).sinh() * (var_p40_t_dn11 - (p.p37 * var_vds_dn11))), ((assign1500_e1923).sinh() * (var_p40_t_dn12 - (p.p37 * var_vds_dn12))), ((assign1500_e1923).sinh() * (var_p40_t_dn13 - (p.p37 * var_vds_dn13))), ((assign1500_e1923).sinh() * (var_p40_t_dn14 - (p.p37 * var_vds_dn14))), ((assign1500_e1923).sinh() * (var_p40_t_dn15 - (p.p37 * var_vds_dn15))), ((assign1500_e1923).sinh() * (var_p40_t_db0 - (p.p37 * var_vds_db0))), ((assign1500_e1923).sinh() * (var_p40_t_db1 - (p.p37 * var_vds_db1))), ((assign1500_e1923).sinh() * (var_p40_t_db2 - (p.p37 * var_vds_db2))), ((assign1500_e1923).sinh() * (var_p40_t_db3 - (p.p37 * var_vds_db3))), ((assign1500_e1923).sinh() * (var_p40_t_db4 - (p.p37 * var_vds_db4))), ((assign1500_e1923).sinh() * (var_p40_t_db5 - (p.p37 * var_vds_db5))), ((assign1500_e1923).sinh() * (var_p40_t_db6 - (p.p37 * var_vds_db6))), ((assign1500_e1923).sinh() * (var_p40_t_db7 - (p.p37 * var_vds_db7))), ((assign1500_e1923).sinh() * (var_p40_t_db8 - (p.p37 * var_vds_db8))), ((assign1500_e1923).sinh() * (var_p40_t_db9 - (p.p37 * var_vds_db9))), ((assign1500_e1923).sinh() * (var_p40_t_db10 - (p.p37 * var_vds_db10))), ((assign1500_e1923).sinh() * (var_p40_t_db11 - (p.p37 * var_vds_db11))), ((assign1500_e1923).sinh() * (var_p40_t_db12 - (p.p37 * var_vds_db12))), ((assign1500_e1923).sinh() * (var_p40_t_db13 - (p.p37 * var_vds_db13))), ((assign1500_e1923).sinh() * (var_p40_t_db14 - (p.p37 * var_vds_db14))),)
    } else {
        (var_cosh0, var_cosh0_dn0, var_cosh0_dn1, var_cosh0_dn2, var_cosh0_dn3, var_cosh0_dn4, var_cosh0_dn5, var_cosh0_dn6, var_cosh0_dn7, var_cosh0_dn8, var_cosh0_dn9, var_cosh0_dn10, var_cosh0_dn11, var_cosh0_dn12, var_cosh0_dn13, var_cosh0_dn14, var_cosh0_dn15, var_cosh0_db0, var_cosh0_db1, var_cosh0_db2, var_cosh0_db3, var_cosh0_db4, var_cosh0_db5, var_cosh0_db6, var_cosh0_db7, var_cosh0_db8, var_cosh0_db9, var_cosh0_db10, var_cosh0_db11, var_cosh0_db12, var_cosh0_db13, var_cosh0_db14,)
    }
};
        var_cosh0 = assign1500_e1926;
        var_cosh0_dn0 = assign1500_e1926_d_n0;
        var_cosh0_dn1 = assign1500_e1926_d_n1;
        var_cosh0_dn2 = assign1500_e1926_d_n2;
        var_cosh0_dn3 = assign1500_e1926_d_n3;
        var_cosh0_dn4 = assign1500_e1926_d_n4;
        var_cosh0_dn5 = assign1500_e1926_d_n5;
        var_cosh0_dn6 = assign1500_e1926_d_n6;
        var_cosh0_dn7 = assign1500_e1926_d_n7;
        var_cosh0_dn8 = assign1500_e1926_d_n8;
        var_cosh0_dn9 = assign1500_e1926_d_n9;
        var_cosh0_dn10 = assign1500_e1926_d_n10;
        var_cosh0_dn11 = assign1500_e1926_d_n11;
        var_cosh0_dn12 = assign1500_e1926_d_n12;
        var_cosh0_dn13 = assign1500_e1926_d_n13;
        var_cosh0_dn14 = assign1500_e1926_d_n14;
        var_cosh0_dn15 = assign1500_e1926_d_n15;
        var_cosh0_db0 = assign1500_e1926_d_b0;
        var_cosh0_db1 = assign1500_e1926_d_b1;
        var_cosh0_db2 = assign1500_e1926_d_b2;
        var_cosh0_db3 = assign1500_e1926_d_b3;
        var_cosh0_db4 = assign1500_e1926_d_b4;
        var_cosh0_db5 = assign1500_e1926_d_b5;
        var_cosh0_db6 = assign1500_e1926_d_b6;
        var_cosh0_db7 = assign1500_e1926_d_b7;
        var_cosh0_db8 = assign1500_e1926_d_b8;
        var_cosh0_db9 = assign1500_e1926_d_b9;
        var_cosh0_db10 = assign1500_e1926_d_b10;
        var_cosh0_db11 = assign1500_e1926_d_b11;
        var_cosh0_db12 = assign1500_e1926_d_b12;
        var_cosh0_db13 = assign1500_e1926_d_b13;
        var_cosh0_db14 = assign1500_e1926_d_b14;
        var_cosh0_db15 = 0.0;
        var_cosh0_db16 = 0.0;
        var_cosh0_db17 = 0.0;
        var_cosh0_db18 = 0.0;
        var_cosh0_rv = 0.0;
        var_cosh0_rdn0 = 0.0;
        var_cosh0_rdn1 = 0.0;
        var_cosh0_rdn2 = 0.0;
        var_cosh0_rdn3 = 0.0;
        var_cosh0_rdn4 = 0.0;
        var_cosh0_rdn5 = 0.0;
        var_cosh0_rdn6 = 0.0;
        var_cosh0_rdn7 = 0.0;
        var_cosh0_rdn8 = 0.0;
        var_cosh0_rdn9 = 0.0;
        var_cosh0_rdn10 = 0.0;
        var_cosh0_rdn11 = 0.0;
        var_cosh0_rdn12 = 0.0;
        var_cosh0_rdn13 = 0.0;
        var_cosh0_rdn14 = 0.0;
        var_cosh0_rdn15 = 0.0;
        var_cosh0_rdb0 = 0.0;
        var_cosh0_rdb1 = 0.0;
        var_cosh0_rdb2 = 0.0;
        var_cosh0_rdb3 = 0.0;
        var_cosh0_rdb4 = 0.0;
        var_cosh0_rdb5 = 0.0;
        var_cosh0_rdb6 = 0.0;
        var_cosh0_rdb7 = 0.0;
        var_cosh0_rdb8 = 0.0;
        var_cosh0_rdb9 = 0.0;
        var_cosh0_rdb10 = 0.0;
        var_cosh0_rdb11 = 0.0;
        var_cosh0_rdb12 = 0.0;
        var_cosh0_rdb13 = 0.0;
        var_cosh0_rdb14 = 0.0;
        var_cosh0_rdb15 = 0.0;
        var_cosh0_rdb16 = 0.0;
        var_cosh0_rdb17 = 0.0;
        var_cosh0_rdb18 = 0.0;

        let (assign1510_e1936, assign1510_e1936_d_n0, assign1510_e1936_d_n1, assign1510_e1936_d_n2, assign1510_e1936_d_n3, assign1510_e1936_d_n4, assign1510_e1936_d_n5, assign1510_e1936_d_n6, assign1510_e1936_d_n7, assign1510_e1936_d_n8, assign1510_e1936_d_n9, assign1510_e1936_d_n10, assign1510_e1936_d_n11, assign1510_e1936_d_n12, assign1510_e1936_d_n13, assign1510_e1936_d_n14, assign1510_e1936_d_n15, assign1510_e1936_d_b0, assign1510_e1936_d_b1, assign1510_e1936_d_b2, assign1510_e1936_d_b3, assign1510_e1936_d_b4, assign1510_e1936_d_b5, assign1510_e1936_d_b6, assign1510_e1936_d_b7, assign1510_e1936_d_b8, assign1510_e1936_d_b9, assign1510_e1936_d_b10, assign1510_e1936_d_b11, assign1510_e1936_d_b12, assign1510_e1936_d_b13, assign1510_e1936_d_b14,) = {
    if ((var_guard15 != 0.0) && (!((var_guard13 != 0.0) || (var_guard14 != 0.0)))) {
        let assign1510_e1934: f64 = (var_cosh0).ln();
        (assign1510_e1934, (var_cosh0_dn0 / var_cosh0), (var_cosh0_dn1 / var_cosh0), (var_cosh0_dn2 / var_cosh0), (var_cosh0_dn3 / var_cosh0), (var_cosh0_dn4 / var_cosh0), (var_cosh0_dn5 / var_cosh0), (var_cosh0_dn6 / var_cosh0), (var_cosh0_dn7 / var_cosh0), (var_cosh0_dn8 / var_cosh0), (var_cosh0_dn9 / var_cosh0), (var_cosh0_dn10 / var_cosh0), (var_cosh0_dn11 / var_cosh0), (var_cosh0_dn12 / var_cosh0), (var_cosh0_dn13 / var_cosh0), (var_cosh0_dn14 / var_cosh0), (var_cosh0_dn15 / var_cosh0), (var_cosh0_db0 / var_cosh0), (var_cosh0_db1 / var_cosh0), (var_cosh0_db2 / var_cosh0), (var_cosh0_db3 / var_cosh0), (var_cosh0_db4 / var_cosh0), (var_cosh0_db5 / var_cosh0), (var_cosh0_db6 / var_cosh0), (var_cosh0_db7 / var_cosh0), (var_cosh0_db8 / var_cosh0), (var_cosh0_db9 / var_cosh0), (var_cosh0_db10 / var_cosh0), (var_cosh0_db11 / var_cosh0), (var_cosh0_db12 / var_cosh0), (var_cosh0_db13 / var_cosh0), (var_cosh0_db14 / var_cosh0),)
    } else {
        (var_lc40, var_lc40_dn0, var_lc40_dn1, var_lc40_dn2, var_lc40_dn3, var_lc40_dn4, var_lc40_dn5, var_lc40_dn6, var_lc40_dn7, var_lc40_dn8, var_lc40_dn9, var_lc40_dn10, var_lc40_dn11, var_lc40_dn12, var_lc40_dn13, var_lc40_dn14, var_lc40_dn15, var_lc40_db0, var_lc40_db1, var_lc40_db2, var_lc40_db3, var_lc40_db4, var_lc40_db5, var_lc40_db6, var_lc40_db7, var_lc40_db8, var_lc40_db9, var_lc40_db10, var_lc40_db11, var_lc40_db12, var_lc40_db13, var_lc40_db14,)
    }
};
        var_lc40 = assign1510_e1936;
        var_lc40_dn0 = assign1510_e1936_d_n0;
        var_lc40_dn1 = assign1510_e1936_d_n1;
        var_lc40_dn2 = assign1510_e1936_d_n2;
        var_lc40_dn3 = assign1510_e1936_d_n3;
        var_lc40_dn4 = assign1510_e1936_d_n4;
        var_lc40_dn5 = assign1510_e1936_d_n5;
        var_lc40_dn6 = assign1510_e1936_d_n6;
        var_lc40_dn7 = assign1510_e1936_d_n7;
        var_lc40_dn8 = assign1510_e1936_d_n8;
        var_lc40_dn9 = assign1510_e1936_d_n9;
        var_lc40_dn10 = assign1510_e1936_d_n10;
        var_lc40_dn11 = assign1510_e1936_d_n11;
        var_lc40_dn12 = assign1510_e1936_d_n12;
        var_lc40_dn13 = assign1510_e1936_d_n13;
        var_lc40_dn14 = assign1510_e1936_d_n14;
        var_lc40_dn15 = assign1510_e1936_d_n15;
        var_lc40_db0 = assign1510_e1936_d_b0;
        var_lc40_db1 = assign1510_e1936_d_b1;
        var_lc40_db2 = assign1510_e1936_d_b2;
        var_lc40_db3 = assign1510_e1936_d_b3;
        var_lc40_db4 = assign1510_e1936_d_b4;
        var_lc40_db5 = assign1510_e1936_d_b5;
        var_lc40_db6 = assign1510_e1936_d_b6;
        var_lc40_db7 = assign1510_e1936_d_b7;
        var_lc40_db8 = assign1510_e1936_d_b8;
        var_lc40_db9 = assign1510_e1936_d_b9;
        var_lc40_db10 = assign1510_e1936_d_b10;
        var_lc40_db11 = assign1510_e1936_d_b11;
        var_lc40_db12 = assign1510_e1936_d_b12;
        var_lc40_db13 = assign1510_e1936_d_b13;
        var_lc40_db14 = assign1510_e1936_d_b14;
        var_lc40_db15 = 0.0;
        var_lc40_db16 = 0.0;
        var_lc40_db17 = 0.0;
        var_lc40_db18 = 0.0;
        var_lc40_rv = 0.0;
        var_lc40_rdn0 = 0.0;
        var_lc40_rdn1 = 0.0;
        var_lc40_rdn2 = 0.0;
        var_lc40_rdn3 = 0.0;
        var_lc40_rdn4 = 0.0;
        var_lc40_rdn5 = 0.0;
        var_lc40_rdn6 = 0.0;
        var_lc40_rdn7 = 0.0;
        var_lc40_rdn8 = 0.0;
        var_lc40_rdn9 = 0.0;
        var_lc40_rdn10 = 0.0;
        var_lc40_rdn11 = 0.0;
        var_lc40_rdn12 = 0.0;
        var_lc40_rdn13 = 0.0;
        var_lc40_rdn14 = 0.0;
        var_lc40_rdn15 = 0.0;
        var_lc40_rdb0 = 0.0;
        var_lc40_rdb1 = 0.0;
        var_lc40_rdb2 = 0.0;
        var_lc40_rdb3 = 0.0;
        var_lc40_rdb4 = 0.0;
        var_lc40_rdb5 = 0.0;
        var_lc40_rdb6 = 0.0;
        var_lc40_rdb7 = 0.0;
        var_lc40_rdb8 = 0.0;
        var_lc40_rdb9 = 0.0;
        var_lc40_rdb10 = 0.0;
        var_lc40_rdb11 = 0.0;
        var_lc40_rdb12 = 0.0;
        var_lc40_rdb13 = 0.0;
        var_lc40_rdb14 = 0.0;
        var_lc40_rdb15 = 0.0;
        var_lc40_rdb16 = 0.0;
        var_lc40_rdb17 = 0.0;
        var_lc40_rdb18 = 0.0;

        *var_cosh0_slot = var_cosh0;
        *var_cosh0_db0_slot = var_cosh0_db0;
        *var_cosh0_db1_slot = var_cosh0_db1;
        *var_cosh0_db10_slot = var_cosh0_db10;
        *var_cosh0_db11_slot = var_cosh0_db11;
        *var_cosh0_db12_slot = var_cosh0_db12;
        *var_cosh0_db13_slot = var_cosh0_db13;
        *var_cosh0_db14_slot = var_cosh0_db14;
        *var_cosh0_db15_slot = var_cosh0_db15;
        *var_cosh0_db16_slot = var_cosh0_db16;
        *var_cosh0_db17_slot = var_cosh0_db17;
        *var_cosh0_db18_slot = var_cosh0_db18;
        *var_cosh0_db2_slot = var_cosh0_db2;
        *var_cosh0_db3_slot = var_cosh0_db3;
        *var_cosh0_db4_slot = var_cosh0_db4;
        *var_cosh0_db5_slot = var_cosh0_db5;
        *var_cosh0_db6_slot = var_cosh0_db6;
        *var_cosh0_db7_slot = var_cosh0_db7;
        *var_cosh0_db8_slot = var_cosh0_db8;
        *var_cosh0_db9_slot = var_cosh0_db9;
        *var_cosh0_dn0_slot = var_cosh0_dn0;
        *var_cosh0_dn1_slot = var_cosh0_dn1;
        *var_cosh0_dn10_slot = var_cosh0_dn10;
        *var_cosh0_dn11_slot = var_cosh0_dn11;
        *var_cosh0_dn12_slot = var_cosh0_dn12;
        *var_cosh0_dn13_slot = var_cosh0_dn13;
        *var_cosh0_dn14_slot = var_cosh0_dn14;
        *var_cosh0_dn15_slot = var_cosh0_dn15;
        *var_cosh0_dn2_slot = var_cosh0_dn2;
        *var_cosh0_dn3_slot = var_cosh0_dn3;
        *var_cosh0_dn4_slot = var_cosh0_dn4;
        *var_cosh0_dn5_slot = var_cosh0_dn5;
        *var_cosh0_dn6_slot = var_cosh0_dn6;
        *var_cosh0_dn7_slot = var_cosh0_dn7;
        *var_cosh0_dn8_slot = var_cosh0_dn8;
        *var_cosh0_dn9_slot = var_cosh0_dn9;
        *var_cosh0_rdb0_slot = var_cosh0_rdb0;
        *var_cosh0_rdb1_slot = var_cosh0_rdb1;
        *var_cosh0_rdb10_slot = var_cosh0_rdb10;
        *var_cosh0_rdb11_slot = var_cosh0_rdb11;
        *var_cosh0_rdb12_slot = var_cosh0_rdb12;
        *var_cosh0_rdb13_slot = var_cosh0_rdb13;
        *var_cosh0_rdb14_slot = var_cosh0_rdb14;
        *var_cosh0_rdb15_slot = var_cosh0_rdb15;
        *var_cosh0_rdb16_slot = var_cosh0_rdb16;
        *var_cosh0_rdb17_slot = var_cosh0_rdb17;
        *var_cosh0_rdb18_slot = var_cosh0_rdb18;
        *var_cosh0_rdb2_slot = var_cosh0_rdb2;
        *var_cosh0_rdb3_slot = var_cosh0_rdb3;
        *var_cosh0_rdb4_slot = var_cosh0_rdb4;
        *var_cosh0_rdb5_slot = var_cosh0_rdb5;
        *var_cosh0_rdb6_slot = var_cosh0_rdb6;
        *var_cosh0_rdb7_slot = var_cosh0_rdb7;
        *var_cosh0_rdb8_slot = var_cosh0_rdb8;
        *var_cosh0_rdb9_slot = var_cosh0_rdb9;
        *var_cosh0_rdn0_slot = var_cosh0_rdn0;
        *var_cosh0_rdn1_slot = var_cosh0_rdn1;
        *var_cosh0_rdn10_slot = var_cosh0_rdn10;
        *var_cosh0_rdn11_slot = var_cosh0_rdn11;
        *var_cosh0_rdn12_slot = var_cosh0_rdn12;
        *var_cosh0_rdn13_slot = var_cosh0_rdn13;
        *var_cosh0_rdn14_slot = var_cosh0_rdn14;
        *var_cosh0_rdn15_slot = var_cosh0_rdn15;
        *var_cosh0_rdn2_slot = var_cosh0_rdn2;
        *var_cosh0_rdn3_slot = var_cosh0_rdn3;
        *var_cosh0_rdn4_slot = var_cosh0_rdn4;
        *var_cosh0_rdn5_slot = var_cosh0_rdn5;
        *var_cosh0_rdn6_slot = var_cosh0_rdn6;
        *var_cosh0_rdn7_slot = var_cosh0_rdn7;
        *var_cosh0_rdn8_slot = var_cosh0_rdn8;
        *var_cosh0_rdn9_slot = var_cosh0_rdn9;
        *var_cosh0_rv_slot = var_cosh0_rv;
        *var_cosh1_slot = var_cosh1;
        *var_cosh1_db0_slot = var_cosh1_db0;
        *var_cosh1_db1_slot = var_cosh1_db1;
        *var_cosh1_db10_slot = var_cosh1_db10;
        *var_cosh1_db11_slot = var_cosh1_db11;
        *var_cosh1_db12_slot = var_cosh1_db12;
        *var_cosh1_db13_slot = var_cosh1_db13;
        *var_cosh1_db14_slot = var_cosh1_db14;
        *var_cosh1_db15_slot = var_cosh1_db15;
        *var_cosh1_db16_slot = var_cosh1_db16;
        *var_cosh1_db17_slot = var_cosh1_db17;
        *var_cosh1_db18_slot = var_cosh1_db18;
        *var_cosh1_db2_slot = var_cosh1_db2;
        *var_cosh1_db3_slot = var_cosh1_db3;
        *var_cosh1_db4_slot = var_cosh1_db4;
        *var_cosh1_db5_slot = var_cosh1_db5;
        *var_cosh1_db6_slot = var_cosh1_db6;
        *var_cosh1_db7_slot = var_cosh1_db7;
        *var_cosh1_db8_slot = var_cosh1_db8;
        *var_cosh1_db9_slot = var_cosh1_db9;
        *var_cosh1_dn0_slot = var_cosh1_dn0;
        *var_cosh1_dn1_slot = var_cosh1_dn1;
        *var_cosh1_dn10_slot = var_cosh1_dn10;
        *var_cosh1_dn11_slot = var_cosh1_dn11;
        *var_cosh1_dn12_slot = var_cosh1_dn12;
        *var_cosh1_dn13_slot = var_cosh1_dn13;
        *var_cosh1_dn14_slot = var_cosh1_dn14;
        *var_cosh1_dn15_slot = var_cosh1_dn15;
        *var_cosh1_dn2_slot = var_cosh1_dn2;
        *var_cosh1_dn3_slot = var_cosh1_dn3;
        *var_cosh1_dn4_slot = var_cosh1_dn4;
        *var_cosh1_dn5_slot = var_cosh1_dn5;
        *var_cosh1_dn6_slot = var_cosh1_dn6;
        *var_cosh1_dn7_slot = var_cosh1_dn7;
        *var_cosh1_dn8_slot = var_cosh1_dn8;
        *var_cosh1_dn9_slot = var_cosh1_dn9;
        *var_cosh1_rdb0_slot = var_cosh1_rdb0;
        *var_cosh1_rdb1_slot = var_cosh1_rdb1;
        *var_cosh1_rdb10_slot = var_cosh1_rdb10;
        *var_cosh1_rdb11_slot = var_cosh1_rdb11;
        *var_cosh1_rdb12_slot = var_cosh1_rdb12;
        *var_cosh1_rdb13_slot = var_cosh1_rdb13;
        *var_cosh1_rdb14_slot = var_cosh1_rdb14;
        *var_cosh1_rdb15_slot = var_cosh1_rdb15;
        *var_cosh1_rdb16_slot = var_cosh1_rdb16;
        *var_cosh1_rdb17_slot = var_cosh1_rdb17;
        *var_cosh1_rdb18_slot = var_cosh1_rdb18;
        *var_cosh1_rdb2_slot = var_cosh1_rdb2;
        *var_cosh1_rdb3_slot = var_cosh1_rdb3;
        *var_cosh1_rdb4_slot = var_cosh1_rdb4;
        *var_cosh1_rdb5_slot = var_cosh1_rdb5;
        *var_cosh1_rdb6_slot = var_cosh1_rdb6;
        *var_cosh1_rdb7_slot = var_cosh1_rdb7;
        *var_cosh1_rdb8_slot = var_cosh1_rdb8;
        *var_cosh1_rdb9_slot = var_cosh1_rdb9;
        *var_cosh1_rdn0_slot = var_cosh1_rdn0;
        *var_cosh1_rdn1_slot = var_cosh1_rdn1;
        *var_cosh1_rdn10_slot = var_cosh1_rdn10;
        *var_cosh1_rdn11_slot = var_cosh1_rdn11;
        *var_cosh1_rdn12_slot = var_cosh1_rdn12;
        *var_cosh1_rdn13_slot = var_cosh1_rdn13;
        *var_cosh1_rdn14_slot = var_cosh1_rdn14;
        *var_cosh1_rdn15_slot = var_cosh1_rdn15;
        *var_cosh1_rdn2_slot = var_cosh1_rdn2;
        *var_cosh1_rdn3_slot = var_cosh1_rdn3;
        *var_cosh1_rdn4_slot = var_cosh1_rdn4;
        *var_cosh1_rdn5_slot = var_cosh1_rdn5;
        *var_cosh1_rdn6_slot = var_cosh1_rdn6;
        *var_cosh1_rdn7_slot = var_cosh1_rdn7;
        *var_cosh1_rdn8_slot = var_cosh1_rdn8;
        *var_cosh1_rdn9_slot = var_cosh1_rdn9;
        *var_cosh1_rv_slot = var_cosh1_rv;
        *var_lc1_slot = var_lc1;
        *var_lc1_db0_slot = var_lc1_db0;
        *var_lc1_db1_slot = var_lc1_db1;
        *var_lc1_db10_slot = var_lc1_db10;
        *var_lc1_db11_slot = var_lc1_db11;
        *var_lc1_db12_slot = var_lc1_db12;
        *var_lc1_db13_slot = var_lc1_db13;
        *var_lc1_db14_slot = var_lc1_db14;
        *var_lc1_db15_slot = var_lc1_db15;
        *var_lc1_db16_slot = var_lc1_db16;
        *var_lc1_db17_slot = var_lc1_db17;
        *var_lc1_db18_slot = var_lc1_db18;
        *var_lc1_db2_slot = var_lc1_db2;
        *var_lc1_db3_slot = var_lc1_db3;
        *var_lc1_db4_slot = var_lc1_db4;
        *var_lc1_db5_slot = var_lc1_db5;
        *var_lc1_db6_slot = var_lc1_db6;
        *var_lc1_db7_slot = var_lc1_db7;
        *var_lc1_db8_slot = var_lc1_db8;
        *var_lc1_db9_slot = var_lc1_db9;
        *var_lc1_dn0_slot = var_lc1_dn0;
        *var_lc1_dn1_slot = var_lc1_dn1;
        *var_lc1_dn10_slot = var_lc1_dn10;
        *var_lc1_dn11_slot = var_lc1_dn11;
        *var_lc1_dn12_slot = var_lc1_dn12;
        *var_lc1_dn13_slot = var_lc1_dn13;
        *var_lc1_dn14_slot = var_lc1_dn14;
        *var_lc1_dn15_slot = var_lc1_dn15;
        *var_lc1_dn2_slot = var_lc1_dn2;
        *var_lc1_dn3_slot = var_lc1_dn3;
        *var_lc1_dn4_slot = var_lc1_dn4;
        *var_lc1_dn5_slot = var_lc1_dn5;
        *var_lc1_dn6_slot = var_lc1_dn6;
        *var_lc1_dn7_slot = var_lc1_dn7;
        *var_lc1_dn8_slot = var_lc1_dn8;
        *var_lc1_dn9_slot = var_lc1_dn9;
        *var_lc1_rdb0_slot = var_lc1_rdb0;
        *var_lc1_rdb1_slot = var_lc1_rdb1;
        *var_lc1_rdb10_slot = var_lc1_rdb10;
        *var_lc1_rdb11_slot = var_lc1_rdb11;
        *var_lc1_rdb12_slot = var_lc1_rdb12;
        *var_lc1_rdb13_slot = var_lc1_rdb13;
        *var_lc1_rdb14_slot = var_lc1_rdb14;
        *var_lc1_rdb15_slot = var_lc1_rdb15;
        *var_lc1_rdb16_slot = var_lc1_rdb16;
        *var_lc1_rdb17_slot = var_lc1_rdb17;
        *var_lc1_rdb18_slot = var_lc1_rdb18;
        *var_lc1_rdb2_slot = var_lc1_rdb2;
        *var_lc1_rdb3_slot = var_lc1_rdb3;
        *var_lc1_rdb4_slot = var_lc1_rdb4;
        *var_lc1_rdb5_slot = var_lc1_rdb5;
        *var_lc1_rdb6_slot = var_lc1_rdb6;
        *var_lc1_rdb7_slot = var_lc1_rdb7;
        *var_lc1_rdb8_slot = var_lc1_rdb8;
        *var_lc1_rdb9_slot = var_lc1_rdb9;
        *var_lc1_rdn0_slot = var_lc1_rdn0;
        *var_lc1_rdn1_slot = var_lc1_rdn1;
        *var_lc1_rdn10_slot = var_lc1_rdn10;
        *var_lc1_rdn11_slot = var_lc1_rdn11;
        *var_lc1_rdn12_slot = var_lc1_rdn12;
        *var_lc1_rdn13_slot = var_lc1_rdn13;
        *var_lc1_rdn14_slot = var_lc1_rdn14;
        *var_lc1_rdn15_slot = var_lc1_rdn15;
        *var_lc1_rdn2_slot = var_lc1_rdn2;
        *var_lc1_rdn3_slot = var_lc1_rdn3;
        *var_lc1_rdn4_slot = var_lc1_rdn4;
        *var_lc1_rdn5_slot = var_lc1_rdn5;
        *var_lc1_rdn6_slot = var_lc1_rdn6;
        *var_lc1_rdn7_slot = var_lc1_rdn7;
        *var_lc1_rdn8_slot = var_lc1_rdn8;
        *var_lc1_rdn9_slot = var_lc1_rdn9;
        *var_lc1_rv_slot = var_lc1_rv;
        *var_lc40_slot = var_lc40;
        *var_lc40_db0_slot = var_lc40_db0;
        *var_lc40_db1_slot = var_lc40_db1;
        *var_lc40_db10_slot = var_lc40_db10;
        *var_lc40_db11_slot = var_lc40_db11;
        *var_lc40_db12_slot = var_lc40_db12;
        *var_lc40_db13_slot = var_lc40_db13;
        *var_lc40_db14_slot = var_lc40_db14;
        *var_lc40_db15_slot = var_lc40_db15;
        *var_lc40_db16_slot = var_lc40_db16;
        *var_lc40_db17_slot = var_lc40_db17;
        *var_lc40_db18_slot = var_lc40_db18;
        *var_lc40_db2_slot = var_lc40_db2;
        *var_lc40_db3_slot = var_lc40_db3;
        *var_lc40_db4_slot = var_lc40_db4;
        *var_lc40_db5_slot = var_lc40_db5;
        *var_lc40_db6_slot = var_lc40_db6;
        *var_lc40_db7_slot = var_lc40_db7;
        *var_lc40_db8_slot = var_lc40_db8;
        *var_lc40_db9_slot = var_lc40_db9;
        *var_lc40_dn0_slot = var_lc40_dn0;
        *var_lc40_dn1_slot = var_lc40_dn1;
        *var_lc40_dn10_slot = var_lc40_dn10;
        *var_lc40_dn11_slot = var_lc40_dn11;
        *var_lc40_dn12_slot = var_lc40_dn12;
        *var_lc40_dn13_slot = var_lc40_dn13;
        *var_lc40_dn14_slot = var_lc40_dn14;
        *var_lc40_dn15_slot = var_lc40_dn15;
        *var_lc40_dn2_slot = var_lc40_dn2;
        *var_lc40_dn3_slot = var_lc40_dn3;
        *var_lc40_dn4_slot = var_lc40_dn4;
        *var_lc40_dn5_slot = var_lc40_dn5;
        *var_lc40_dn6_slot = var_lc40_dn6;
        *var_lc40_dn7_slot = var_lc40_dn7;
        *var_lc40_dn8_slot = var_lc40_dn8;
        *var_lc40_dn9_slot = var_lc40_dn9;
        *var_lc40_rdb0_slot = var_lc40_rdb0;
        *var_lc40_rdb1_slot = var_lc40_rdb1;
        *var_lc40_rdb10_slot = var_lc40_rdb10;
        *var_lc40_rdb11_slot = var_lc40_rdb11;
        *var_lc40_rdb12_slot = var_lc40_rdb12;
        *var_lc40_rdb13_slot = var_lc40_rdb13;
        *var_lc40_rdb14_slot = var_lc40_rdb14;
        *var_lc40_rdb15_slot = var_lc40_rdb15;
        *var_lc40_rdb16_slot = var_lc40_rdb16;
        *var_lc40_rdb17_slot = var_lc40_rdb17;
        *var_lc40_rdb18_slot = var_lc40_rdb18;
        *var_lc40_rdb2_slot = var_lc40_rdb2;
        *var_lc40_rdb3_slot = var_lc40_rdb3;
        *var_lc40_rdb4_slot = var_lc40_rdb4;
        *var_lc40_rdb5_slot = var_lc40_rdb5;
        *var_lc40_rdb6_slot = var_lc40_rdb6;
        *var_lc40_rdb7_slot = var_lc40_rdb7;
        *var_lc40_rdb8_slot = var_lc40_rdb8;
        *var_lc40_rdb9_slot = var_lc40_rdb9;
        *var_lc40_rdn0_slot = var_lc40_rdn0;
        *var_lc40_rdn1_slot = var_lc40_rdn1;
        *var_lc40_rdn10_slot = var_lc40_rdn10;
        *var_lc40_rdn11_slot = var_lc40_rdn11;
        *var_lc40_rdn12_slot = var_lc40_rdn12;
        *var_lc40_rdn13_slot = var_lc40_rdn13;
        *var_lc40_rdn14_slot = var_lc40_rdn14;
        *var_lc40_rdn15_slot = var_lc40_rdn15;
        *var_lc40_rdn2_slot = var_lc40_rdn2;
        *var_lc40_rdn3_slot = var_lc40_rdn3;
        *var_lc40_rdn4_slot = var_lc40_rdn4;
        *var_lc40_rdn5_slot = var_lc40_rdn5;
        *var_lc40_rdn6_slot = var_lc40_rdn6;
        *var_lc40_rdn7_slot = var_lc40_rdn7;
        *var_lc40_rdn8_slot = var_lc40_rdn8;
        *var_lc40_rdn9_slot = var_lc40_rdn9;
        *var_lc40_rv_slot = var_lc40_rv;
        *var_qgs_slot = var_qgs;
        *var_qgs0_slot = var_qgs0;
        *var_qgs0_db0_slot = var_qgs0_db0;
        *var_qgs0_db1_slot = var_qgs0_db1;
        *var_qgs0_db10_slot = var_qgs0_db10;
        *var_qgs0_db11_slot = var_qgs0_db11;
        *var_qgs0_db12_slot = var_qgs0_db12;
        *var_qgs0_db13_slot = var_qgs0_db13;
        *var_qgs0_db14_slot = var_qgs0_db14;
        *var_qgs0_db15_slot = var_qgs0_db15;
        *var_qgs0_db16_slot = var_qgs0_db16;
        *var_qgs0_db17_slot = var_qgs0_db17;
        *var_qgs0_db18_slot = var_qgs0_db18;
        *var_qgs0_db2_slot = var_qgs0_db2;
        *var_qgs0_db3_slot = var_qgs0_db3;
        *var_qgs0_db4_slot = var_qgs0_db4;
        *var_qgs0_db5_slot = var_qgs0_db5;
        *var_qgs0_db6_slot = var_qgs0_db6;
        *var_qgs0_db7_slot = var_qgs0_db7;
        *var_qgs0_db8_slot = var_qgs0_db8;
        *var_qgs0_db9_slot = var_qgs0_db9;
        *var_qgs0_dn0_slot = var_qgs0_dn0;
        *var_qgs0_dn1_slot = var_qgs0_dn1;
        *var_qgs0_dn10_slot = var_qgs0_dn10;
        *var_qgs0_dn11_slot = var_qgs0_dn11;
        *var_qgs0_dn12_slot = var_qgs0_dn12;
        *var_qgs0_dn13_slot = var_qgs0_dn13;
        *var_qgs0_dn14_slot = var_qgs0_dn14;
        *var_qgs0_dn15_slot = var_qgs0_dn15;
        *var_qgs0_dn2_slot = var_qgs0_dn2;
        *var_qgs0_dn3_slot = var_qgs0_dn3;
        *var_qgs0_dn4_slot = var_qgs0_dn4;
        *var_qgs0_dn5_slot = var_qgs0_dn5;
        *var_qgs0_dn6_slot = var_qgs0_dn6;
        *var_qgs0_dn7_slot = var_qgs0_dn7;
        *var_qgs0_dn8_slot = var_qgs0_dn8;
        *var_qgs0_dn9_slot = var_qgs0_dn9;
        *var_qgs0_rdb0_slot = var_qgs0_rdb0;
        *var_qgs0_rdb1_slot = var_qgs0_rdb1;
        *var_qgs0_rdb10_slot = var_qgs0_rdb10;
        *var_qgs0_rdb11_slot = var_qgs0_rdb11;
        *var_qgs0_rdb12_slot = var_qgs0_rdb12;
        *var_qgs0_rdb13_slot = var_qgs0_rdb13;
        *var_qgs0_rdb14_slot = var_qgs0_rdb14;
        *var_qgs0_rdb15_slot = var_qgs0_rdb15;
        *var_qgs0_rdb16_slot = var_qgs0_rdb16;
        *var_qgs0_rdb17_slot = var_qgs0_rdb17;
        *var_qgs0_rdb18_slot = var_qgs0_rdb18;
        *var_qgs0_rdb2_slot = var_qgs0_rdb2;
        *var_qgs0_rdb3_slot = var_qgs0_rdb3;
        *var_qgs0_rdb4_slot = var_qgs0_rdb4;
        *var_qgs0_rdb5_slot = var_qgs0_rdb5;
        *var_qgs0_rdb6_slot = var_qgs0_rdb6;
        *var_qgs0_rdb7_slot = var_qgs0_rdb7;
        *var_qgs0_rdb8_slot = var_qgs0_rdb8;
        *var_qgs0_rdb9_slot = var_qgs0_rdb9;
        *var_qgs0_rdn0_slot = var_qgs0_rdn0;
        *var_qgs0_rdn1_slot = var_qgs0_rdn1;
        *var_qgs0_rdn10_slot = var_qgs0_rdn10;
        *var_qgs0_rdn11_slot = var_qgs0_rdn11;
        *var_qgs0_rdn12_slot = var_qgs0_rdn12;
        *var_qgs0_rdn13_slot = var_qgs0_rdn13;
        *var_qgs0_rdn14_slot = var_qgs0_rdn14;
        *var_qgs0_rdn15_slot = var_qgs0_rdn15;
        *var_qgs0_rdn2_slot = var_qgs0_rdn2;
        *var_qgs0_rdn3_slot = var_qgs0_rdn3;
        *var_qgs0_rdn4_slot = var_qgs0_rdn4;
        *var_qgs0_rdn5_slot = var_qgs0_rdn5;
        *var_qgs0_rdn6_slot = var_qgs0_rdn6;
        *var_qgs0_rdn7_slot = var_qgs0_rdn7;
        *var_qgs0_rdn8_slot = var_qgs0_rdn8;
        *var_qgs0_rdn9_slot = var_qgs0_rdn9;
        *var_qgs0_rv_slot = var_qgs0_rv;
        *var_qgs_db0_slot = var_qgs_db0;
        *var_qgs_db1_slot = var_qgs_db1;
        *var_qgs_db10_slot = var_qgs_db10;
        *var_qgs_db11_slot = var_qgs_db11;
        *var_qgs_db12_slot = var_qgs_db12;
        *var_qgs_db13_slot = var_qgs_db13;
        *var_qgs_db14_slot = var_qgs_db14;
        *var_qgs_db15_slot = var_qgs_db15;
        *var_qgs_db16_slot = var_qgs_db16;
        *var_qgs_db17_slot = var_qgs_db17;
        *var_qgs_db18_slot = var_qgs_db18;
        *var_qgs_db2_slot = var_qgs_db2;
        *var_qgs_db3_slot = var_qgs_db3;
        *var_qgs_db4_slot = var_qgs_db4;
        *var_qgs_db5_slot = var_qgs_db5;
        *var_qgs_db6_slot = var_qgs_db6;
        *var_qgs_db7_slot = var_qgs_db7;
        *var_qgs_db8_slot = var_qgs_db8;
        *var_qgs_db9_slot = var_qgs_db9;
        *var_qgs_dn0_slot = var_qgs_dn0;
        *var_qgs_dn1_slot = var_qgs_dn1;
        *var_qgs_dn10_slot = var_qgs_dn10;
        *var_qgs_dn11_slot = var_qgs_dn11;
        *var_qgs_dn12_slot = var_qgs_dn12;
        *var_qgs_dn13_slot = var_qgs_dn13;
        *var_qgs_dn14_slot = var_qgs_dn14;
        *var_qgs_dn15_slot = var_qgs_dn15;
        *var_qgs_dn2_slot = var_qgs_dn2;
        *var_qgs_dn3_slot = var_qgs_dn3;
        *var_qgs_dn4_slot = var_qgs_dn4;
        *var_qgs_dn5_slot = var_qgs_dn5;
        *var_qgs_dn6_slot = var_qgs_dn6;
        *var_qgs_dn7_slot = var_qgs_dn7;
        *var_qgs_dn8_slot = var_qgs_dn8;
        *var_qgs_dn9_slot = var_qgs_dn9;
        *var_qgs_rdb0_slot = var_qgs_rdb0;
        *var_qgs_rdb1_slot = var_qgs_rdb1;
        *var_qgs_rdb10_slot = var_qgs_rdb10;
        *var_qgs_rdb11_slot = var_qgs_rdb11;
        *var_qgs_rdb12_slot = var_qgs_rdb12;
        *var_qgs_rdb13_slot = var_qgs_rdb13;
        *var_qgs_rdb14_slot = var_qgs_rdb14;
        *var_qgs_rdb15_slot = var_qgs_rdb15;
        *var_qgs_rdb16_slot = var_qgs_rdb16;
        *var_qgs_rdb17_slot = var_qgs_rdb17;
        *var_qgs_rdb18_slot = var_qgs_rdb18;
        *var_qgs_rdb2_slot = var_qgs_rdb2;
        *var_qgs_rdb3_slot = var_qgs_rdb3;
        *var_qgs_rdb4_slot = var_qgs_rdb4;
        *var_qgs_rdb5_slot = var_qgs_rdb5;
        *var_qgs_rdb6_slot = var_qgs_rdb6;
        *var_qgs_rdb7_slot = var_qgs_rdb7;
        *var_qgs_rdb8_slot = var_qgs_rdb8;
        *var_qgs_rdb9_slot = var_qgs_rdb9;
        *var_qgs_rdn0_slot = var_qgs_rdn0;
        *var_qgs_rdn1_slot = var_qgs_rdn1;
        *var_qgs_rdn10_slot = var_qgs_rdn10;
        *var_qgs_rdn11_slot = var_qgs_rdn11;
        *var_qgs_rdn12_slot = var_qgs_rdn12;
        *var_qgs_rdn13_slot = var_qgs_rdn13;
        *var_qgs_rdn14_slot = var_qgs_rdn14;
        *var_qgs_rdn15_slot = var_qgs_rdn15;
        *var_qgs_rdn2_slot = var_qgs_rdn2;
        *var_qgs_rdn3_slot = var_qgs_rdn3;
        *var_qgs_rdn4_slot = var_qgs_rdn4;
        *var_qgs_rdn5_slot = var_qgs_rdn5;
        *var_qgs_rdn6_slot = var_qgs_rdn6;
        *var_qgs_rdn7_slot = var_qgs_rdn7;
        *var_qgs_rdn8_slot = var_qgs_rdn8;
        *var_qgs_rdn9_slot = var_qgs_rdn9;
        *var_qgs_rv_slot = var_qgs_rv;
    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        var_cgd0_t: f64,
        var_cgd0_t_db0: f64,
        var_cgd0_t_db1: f64,
        var_cgd0_t_db10: f64,
        var_cgd0_t_db11: f64,
        var_cgd0_t_db12: f64,
        var_cgd0_t_db13: f64,
        var_cgd0_t_db14: f64,
        var_cgd0_t_db2: f64,
        var_cgd0_t_db3: f64,
        var_cgd0_t_db4: f64,
        var_cgd0_t_db5: f64,
        var_cgd0_t_db6: f64,
        var_cgd0_t_db7: f64,
        var_cgd0_t_db8: f64,
        var_cgd0_t_db9: f64,
        var_cgd0_t_dn0: f64,
        var_cgd0_t_dn1: f64,
        var_cgd0_t_dn10: f64,
        var_cgd0_t_dn11: f64,
        var_cgd0_t_dn12: f64,
        var_cgd0_t_dn13: f64,
        var_cgd0_t_dn14: f64,
        var_cgd0_t_dn15: f64,
        var_cgd0_t_dn2: f64,
        var_cgd0_t_dn3: f64,
        var_cgd0_t_dn4: f64,
        var_cgd0_t_dn5: f64,
        var_cgd0_t_dn6: f64,
        var_cgd0_t_dn7: f64,
        var_cgd0_t_dn8: f64,
        var_cgd0_t_dn9: f64,
        var_guard13: f64,
        var_guard14: f64,
        var_guard15: f64,
        var_lc40: f64,
        var_lc40_db0: f64,
        var_lc40_db1: f64,
        var_lc40_db10: f64,
        var_lc40_db11: f64,
        var_lc40_db12: f64,
        var_lc40_db13: f64,
        var_lc40_db14: f64,
        var_lc40_db2: f64,
        var_lc40_db3: f64,
        var_lc40_db4: f64,
        var_lc40_db5: f64,
        var_lc40_db6: f64,
        var_lc40_db7: f64,
        var_lc40_db8: f64,
        var_lc40_db9: f64,
        var_lc40_dn0: f64,
        var_lc40_dn1: f64,
        var_lc40_dn10: f64,
        var_lc40_dn11: f64,
        var_lc40_dn12: f64,
        var_lc40_dn13: f64,
        var_lc40_dn14: f64,
        var_lc40_dn15: f64,
        var_lc40_dn2: f64,
        var_lc40_dn3: f64,
        var_lc40_dn4: f64,
        var_lc40_dn5: f64,
        var_lc40_dn6: f64,
        var_lc40_dn7: f64,
        var_lc40_dn8: f64,
        var_lc40_dn9: f64,
        var_p40_t: f64,
        var_p40_t_db0: f64,
        var_p40_t_db1: f64,
        var_p40_t_db10: f64,
        var_p40_t_db11: f64,
        var_p40_t_db12: f64,
        var_p40_t_db13: f64,
        var_p40_t_db14: f64,
        var_p40_t_db2: f64,
        var_p40_t_db3: f64,
        var_p40_t_db4: f64,
        var_p40_t_db5: f64,
        var_p40_t_db6: f64,
        var_p40_t_db7: f64,
        var_p40_t_db8: f64,
        var_p40_t_db9: f64,
        var_p40_t_dn0: f64,
        var_p40_t_dn1: f64,
        var_p40_t_dn10: f64,
        var_p40_t_dn11: f64,
        var_p40_t_dn12: f64,
        var_p40_t_dn13: f64,
        var_p40_t_dn14: f64,
        var_p40_t_dn15: f64,
        var_p40_t_dn2: f64,
        var_p40_t_dn3: f64,
        var_p40_t_dn4: f64,
        var_p40_t_dn5: f64,
        var_p40_t_dn6: f64,
        var_p40_t_dn7: f64,
        var_p40_t_dn8: f64,
        var_p40_t_dn9: f64,
        var_psi_4: f64,
        var_psi_4_db0: f64,
        var_psi_4_db1: f64,
        var_psi_4_db10: f64,
        var_psi_4_db11: f64,
        var_psi_4_db12: f64,
        var_psi_4_db13: f64,
        var_psi_4_db14: f64,
        var_psi_4_db2: f64,
        var_psi_4_db3: f64,
        var_psi_4_db4: f64,
        var_psi_4_db5: f64,
        var_psi_4_db6: f64,
        var_psi_4_db7: f64,
        var_psi_4_db8: f64,
        var_psi_4_db9: f64,
        var_psi_4_dn0: f64,
        var_psi_4_dn1: f64,
        var_psi_4_dn10: f64,
        var_psi_4_dn11: f64,
        var_psi_4_dn12: f64,
        var_psi_4_dn13: f64,
        var_psi_4_dn14: f64,
        var_psi_4_dn15: f64,
        var_psi_4_dn2: f64,
        var_psi_4_dn3: f64,
        var_psi_4_dn4: f64,
        var_psi_4_dn5: f64,
        var_psi_4_dn6: f64,
        var_psi_4_dn7: f64,
        var_psi_4_dn8: f64,
        var_psi_4_dn9: f64,
        var_qgs_dn8: f64,
        var_tanh3: f64,
        var_tanh3_db0: f64,
        var_tanh3_db1: f64,
        var_tanh3_db10: f64,
        var_tanh3_db11: f64,
        var_tanh3_db12: f64,
        var_tanh3_db13: f64,
        var_tanh3_db14: f64,
        var_tanh3_db2: f64,
        var_tanh3_db3: f64,
        var_tanh3_db4: f64,
        var_tanh3_db5: f64,
        var_tanh3_db6: f64,
        var_tanh3_db7: f64,
        var_tanh3_db8: f64,
        var_tanh3_db9: f64,
        var_tanh3_dn0: f64,
        var_tanh3_dn1: f64,
        var_tanh3_dn10: f64,
        var_tanh3_dn11: f64,
        var_tanh3_dn12: f64,
        var_tanh3_dn13: f64,
        var_tanh3_dn14: f64,
        var_tanh3_dn15: f64,
        var_tanh3_dn2: f64,
        var_tanh3_dn3: f64,
        var_tanh3_dn4: f64,
        var_tanh3_dn5: f64,
        var_tanh3_dn6: f64,
        var_tanh3_dn7: f64,
        var_tanh3_dn8: f64,
        var_tanh3_dn9: f64,
        var_vds: f64,
        var_vds_db0: f64,
        var_vds_db1: f64,
        var_vds_db10: f64,
        var_vds_db11: f64,
        var_vds_db12: f64,
        var_vds_db13: f64,
        var_vds_db14: f64,
        var_vds_db2: f64,
        var_vds_db3: f64,
        var_vds_db4: f64,
        var_vds_db5: f64,
        var_vds_db6: f64,
        var_vds_db7: f64,
        var_vds_db8: f64,
        var_vds_db9: f64,
        var_vds_dn0: f64,
        var_vds_dn1: f64,
        var_vds_dn10: f64,
        var_vds_dn11: f64,
        var_vds_dn12: f64,
        var_vds_dn13: f64,
        var_vds_dn14: f64,
        var_vds_dn15: f64,
        var_vds_dn2: f64,
        var_vds_dn3: f64,
        var_vds_dn4: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vds_dn8: f64,
        var_vds_dn9: f64,
        var_vgdc: f64,
        var_vgdc_db0: f64,
        var_vgdc_db1: f64,
        var_vgdc_db10: f64,
        var_vgdc_db11: f64,
        var_vgdc_db12: f64,
        var_vgdc_db13: f64,
        var_vgdc_db14: f64,
        var_vgdc_db2: f64,
        var_vgdc_db3: f64,
        var_vgdc_db4: f64,
        var_vgdc_db5: f64,
        var_vgdc_db6: f64,
        var_vgdc_db7: f64,
        var_vgdc_db8: f64,
        var_vgdc_db9: f64,
        var_vgdc_dn0: f64,
        var_vgdc_dn1: f64,
        var_vgdc_dn10: f64,
        var_vgdc_dn11: f64,
        var_vgdc_dn12: f64,
        var_vgdc_dn13: f64,
        var_vgdc_dn14: f64,
        var_vgdc_dn15: f64,
        var_vgdc_dn2: f64,
        var_vgdc_dn3: f64,
        var_vgdc_dn4: f64,
        var_vgdc_dn5: f64,
        var_vgdc_dn6: f64,
        var_vgdc_dn7: f64,
        var_vgdc_dn8: f64,
        var_vgdc_dn9: f64,
        var_cgd_slot: &mut f64,
        var_cgd_db0_slot: &mut f64,
        var_cgd_db1_slot: &mut f64,
        var_cgd_db10_slot: &mut f64,
        var_cgd_db11_slot: &mut f64,
        var_cgd_db12_slot: &mut f64,
        var_cgd_db13_slot: &mut f64,
        var_cgd_db14_slot: &mut f64,
        var_cgd_db15_slot: &mut f64,
        var_cgd_db16_slot: &mut f64,
        var_cgd_db17_slot: &mut f64,
        var_cgd_db18_slot: &mut f64,
        var_cgd_db2_slot: &mut f64,
        var_cgd_db3_slot: &mut f64,
        var_cgd_db4_slot: &mut f64,
        var_cgd_db5_slot: &mut f64,
        var_cgd_db6_slot: &mut f64,
        var_cgd_db7_slot: &mut f64,
        var_cgd_db8_slot: &mut f64,
        var_cgd_db9_slot: &mut f64,
        var_cgd_dn0_slot: &mut f64,
        var_cgd_dn1_slot: &mut f64,
        var_cgd_dn10_slot: &mut f64,
        var_cgd_dn11_slot: &mut f64,
        var_cgd_dn12_slot: &mut f64,
        var_cgd_dn13_slot: &mut f64,
        var_cgd_dn14_slot: &mut f64,
        var_cgd_dn15_slot: &mut f64,
        var_cgd_dn2_slot: &mut f64,
        var_cgd_dn3_slot: &mut f64,
        var_cgd_dn4_slot: &mut f64,
        var_cgd_dn5_slot: &mut f64,
        var_cgd_dn6_slot: &mut f64,
        var_cgd_dn7_slot: &mut f64,
        var_cgd_dn8_slot: &mut f64,
        var_cgd_dn9_slot: &mut f64,
        var_cgd_rdb0_slot: &mut f64,
        var_cgd_rdb1_slot: &mut f64,
        var_cgd_rdb10_slot: &mut f64,
        var_cgd_rdb11_slot: &mut f64,
        var_cgd_rdb12_slot: &mut f64,
        var_cgd_rdb13_slot: &mut f64,
        var_cgd_rdb14_slot: &mut f64,
        var_cgd_rdb15_slot: &mut f64,
        var_cgd_rdb16_slot: &mut f64,
        var_cgd_rdb17_slot: &mut f64,
        var_cgd_rdb18_slot: &mut f64,
        var_cgd_rdb2_slot: &mut f64,
        var_cgd_rdb3_slot: &mut f64,
        var_cgd_rdb4_slot: &mut f64,
        var_cgd_rdb5_slot: &mut f64,
        var_cgd_rdb6_slot: &mut f64,
        var_cgd_rdb7_slot: &mut f64,
        var_cgd_rdb8_slot: &mut f64,
        var_cgd_rdb9_slot: &mut f64,
        var_cgd_rdn0_slot: &mut f64,
        var_cgd_rdn1_slot: &mut f64,
        var_cgd_rdn10_slot: &mut f64,
        var_cgd_rdn11_slot: &mut f64,
        var_cgd_rdn12_slot: &mut f64,
        var_cgd_rdn13_slot: &mut f64,
        var_cgd_rdn14_slot: &mut f64,
        var_cgd_rdn15_slot: &mut f64,
        var_cgd_rdn2_slot: &mut f64,
        var_cgd_rdn3_slot: &mut f64,
        var_cgd_rdn4_slot: &mut f64,
        var_cgd_rdn5_slot: &mut f64,
        var_cgd_rdn6_slot: &mut f64,
        var_cgd_rdn7_slot: &mut f64,
        var_cgd_rdn8_slot: &mut f64,
        var_cgd_rdn9_slot: &mut f64,
        var_cgd_rv_slot: &mut f64,
        var_cgs_slot: &mut f64,
        var_cgs_db0_slot: &mut f64,
        var_cgs_db1_slot: &mut f64,
        var_cgs_db10_slot: &mut f64,
        var_cgs_db11_slot: &mut f64,
        var_cgs_db12_slot: &mut f64,
        var_cgs_db13_slot: &mut f64,
        var_cgs_db14_slot: &mut f64,
        var_cgs_db15_slot: &mut f64,
        var_cgs_db16_slot: &mut f64,
        var_cgs_db17_slot: &mut f64,
        var_cgs_db18_slot: &mut f64,
        var_cgs_db2_slot: &mut f64,
        var_cgs_db3_slot: &mut f64,
        var_cgs_db4_slot: &mut f64,
        var_cgs_db5_slot: &mut f64,
        var_cgs_db6_slot: &mut f64,
        var_cgs_db7_slot: &mut f64,
        var_cgs_db8_slot: &mut f64,
        var_cgs_db9_slot: &mut f64,
        var_cgs_dn0_slot: &mut f64,
        var_cgs_dn1_slot: &mut f64,
        var_cgs_dn10_slot: &mut f64,
        var_cgs_dn11_slot: &mut f64,
        var_cgs_dn12_slot: &mut f64,
        var_cgs_dn13_slot: &mut f64,
        var_cgs_dn14_slot: &mut f64,
        var_cgs_dn15_slot: &mut f64,
        var_cgs_dn2_slot: &mut f64,
        var_cgs_dn3_slot: &mut f64,
        var_cgs_dn4_slot: &mut f64,
        var_cgs_dn5_slot: &mut f64,
        var_cgs_dn6_slot: &mut f64,
        var_cgs_dn7_slot: &mut f64,
        var_cgs_dn8_slot: &mut f64,
        var_cgs_dn9_slot: &mut f64,
        var_cgs_rdb0_slot: &mut f64,
        var_cgs_rdb1_slot: &mut f64,
        var_cgs_rdb10_slot: &mut f64,
        var_cgs_rdb11_slot: &mut f64,
        var_cgs_rdb12_slot: &mut f64,
        var_cgs_rdb13_slot: &mut f64,
        var_cgs_rdb14_slot: &mut f64,
        var_cgs_rdb15_slot: &mut f64,
        var_cgs_rdb16_slot: &mut f64,
        var_cgs_rdb17_slot: &mut f64,
        var_cgs_rdb18_slot: &mut f64,
        var_cgs_rdb2_slot: &mut f64,
        var_cgs_rdb3_slot: &mut f64,
        var_cgs_rdb4_slot: &mut f64,
        var_cgs_rdb5_slot: &mut f64,
        var_cgs_rdb6_slot: &mut f64,
        var_cgs_rdb7_slot: &mut f64,
        var_cgs_rdb8_slot: &mut f64,
        var_cgs_rdb9_slot: &mut f64,
        var_cgs_rdn0_slot: &mut f64,
        var_cgs_rdn1_slot: &mut f64,
        var_cgs_rdn10_slot: &mut f64,
        var_cgs_rdn11_slot: &mut f64,
        var_cgs_rdn12_slot: &mut f64,
        var_cgs_rdn13_slot: &mut f64,
        var_cgs_rdn14_slot: &mut f64,
        var_cgs_rdn15_slot: &mut f64,
        var_cgs_rdn2_slot: &mut f64,
        var_cgs_rdn3_slot: &mut f64,
        var_cgs_rdn4_slot: &mut f64,
        var_cgs_rdn5_slot: &mut f64,
        var_cgs_rdn6_slot: &mut f64,
        var_cgs_rdn7_slot: &mut f64,
        var_cgs_rdn8_slot: &mut f64,
        var_cgs_rdn9_slot: &mut f64,
        var_cgs_rv_slot: &mut f64,
        var_cosh1_slot: &mut f64,
        var_cosh1_db0_slot: &mut f64,
        var_cosh1_db1_slot: &mut f64,
        var_cosh1_db10_slot: &mut f64,
        var_cosh1_db11_slot: &mut f64,
        var_cosh1_db12_slot: &mut f64,
        var_cosh1_db13_slot: &mut f64,
        var_cosh1_db14_slot: &mut f64,
        var_cosh1_db15_slot: &mut f64,
        var_cosh1_db16_slot: &mut f64,
        var_cosh1_db17_slot: &mut f64,
        var_cosh1_db18_slot: &mut f64,
        var_cosh1_db2_slot: &mut f64,
        var_cosh1_db3_slot: &mut f64,
        var_cosh1_db4_slot: &mut f64,
        var_cosh1_db5_slot: &mut f64,
        var_cosh1_db6_slot: &mut f64,
        var_cosh1_db7_slot: &mut f64,
        var_cosh1_db8_slot: &mut f64,
        var_cosh1_db9_slot: &mut f64,
        var_cosh1_dn0_slot: &mut f64,
        var_cosh1_dn1_slot: &mut f64,
        var_cosh1_dn10_slot: &mut f64,
        var_cosh1_dn11_slot: &mut f64,
        var_cosh1_dn12_slot: &mut f64,
        var_cosh1_dn13_slot: &mut f64,
        var_cosh1_dn14_slot: &mut f64,
        var_cosh1_dn15_slot: &mut f64,
        var_cosh1_dn2_slot: &mut f64,
        var_cosh1_dn3_slot: &mut f64,
        var_cosh1_dn4_slot: &mut f64,
        var_cosh1_dn5_slot: &mut f64,
        var_cosh1_dn6_slot: &mut f64,
        var_cosh1_dn7_slot: &mut f64,
        var_cosh1_dn8_slot: &mut f64,
        var_cosh1_dn9_slot: &mut f64,
        var_cosh1_rdb0_slot: &mut f64,
        var_cosh1_rdb1_slot: &mut f64,
        var_cosh1_rdb10_slot: &mut f64,
        var_cosh1_rdb11_slot: &mut f64,
        var_cosh1_rdb12_slot: &mut f64,
        var_cosh1_rdb13_slot: &mut f64,
        var_cosh1_rdb14_slot: &mut f64,
        var_cosh1_rdb15_slot: &mut f64,
        var_cosh1_rdb16_slot: &mut f64,
        var_cosh1_rdb17_slot: &mut f64,
        var_cosh1_rdb18_slot: &mut f64,
        var_cosh1_rdb2_slot: &mut f64,
        var_cosh1_rdb3_slot: &mut f64,
        var_cosh1_rdb4_slot: &mut f64,
        var_cosh1_rdb5_slot: &mut f64,
        var_cosh1_rdb6_slot: &mut f64,
        var_cosh1_rdb7_slot: &mut f64,
        var_cosh1_rdb8_slot: &mut f64,
        var_cosh1_rdb9_slot: &mut f64,
        var_cosh1_rdn0_slot: &mut f64,
        var_cosh1_rdn1_slot: &mut f64,
        var_cosh1_rdn10_slot: &mut f64,
        var_cosh1_rdn11_slot: &mut f64,
        var_cosh1_rdn12_slot: &mut f64,
        var_cosh1_rdn13_slot: &mut f64,
        var_cosh1_rdn14_slot: &mut f64,
        var_cosh1_rdn15_slot: &mut f64,
        var_cosh1_rdn2_slot: &mut f64,
        var_cosh1_rdn3_slot: &mut f64,
        var_cosh1_rdn4_slot: &mut f64,
        var_cosh1_rdn5_slot: &mut f64,
        var_cosh1_rdn6_slot: &mut f64,
        var_cosh1_rdn7_slot: &mut f64,
        var_cosh1_rdn8_slot: &mut f64,
        var_cosh1_rdn9_slot: &mut f64,
        var_cosh1_rv_slot: &mut f64,
        var_lc4_slot: &mut f64,
        var_lc4_db0_slot: &mut f64,
        var_lc4_db1_slot: &mut f64,
        var_lc4_db10_slot: &mut f64,
        var_lc4_db11_slot: &mut f64,
        var_lc4_db12_slot: &mut f64,
        var_lc4_db13_slot: &mut f64,
        var_lc4_db14_slot: &mut f64,
        var_lc4_db15_slot: &mut f64,
        var_lc4_db16_slot: &mut f64,
        var_lc4_db17_slot: &mut f64,
        var_lc4_db18_slot: &mut f64,
        var_lc4_db2_slot: &mut f64,
        var_lc4_db3_slot: &mut f64,
        var_lc4_db4_slot: &mut f64,
        var_lc4_db5_slot: &mut f64,
        var_lc4_db6_slot: &mut f64,
        var_lc4_db7_slot: &mut f64,
        var_lc4_db8_slot: &mut f64,
        var_lc4_db9_slot: &mut f64,
        var_lc4_dn0_slot: &mut f64,
        var_lc4_dn1_slot: &mut f64,
        var_lc4_dn10_slot: &mut f64,
        var_lc4_dn11_slot: &mut f64,
        var_lc4_dn12_slot: &mut f64,
        var_lc4_dn13_slot: &mut f64,
        var_lc4_dn14_slot: &mut f64,
        var_lc4_dn15_slot: &mut f64,
        var_lc4_dn2_slot: &mut f64,
        var_lc4_dn3_slot: &mut f64,
        var_lc4_dn4_slot: &mut f64,
        var_lc4_dn5_slot: &mut f64,
        var_lc4_dn6_slot: &mut f64,
        var_lc4_dn7_slot: &mut f64,
        var_lc4_dn8_slot: &mut f64,
        var_lc4_dn9_slot: &mut f64,
        var_lc4_rdb0_slot: &mut f64,
        var_lc4_rdb1_slot: &mut f64,
        var_lc4_rdb10_slot: &mut f64,
        var_lc4_rdb11_slot: &mut f64,
        var_lc4_rdb12_slot: &mut f64,
        var_lc4_rdb13_slot: &mut f64,
        var_lc4_rdb14_slot: &mut f64,
        var_lc4_rdb15_slot: &mut f64,
        var_lc4_rdb16_slot: &mut f64,
        var_lc4_rdb17_slot: &mut f64,
        var_lc4_rdb18_slot: &mut f64,
        var_lc4_rdb2_slot: &mut f64,
        var_lc4_rdb3_slot: &mut f64,
        var_lc4_rdb4_slot: &mut f64,
        var_lc4_rdb5_slot: &mut f64,
        var_lc4_rdb6_slot: &mut f64,
        var_lc4_rdb7_slot: &mut f64,
        var_lc4_rdb8_slot: &mut f64,
        var_lc4_rdb9_slot: &mut f64,
        var_lc4_rdn0_slot: &mut f64,
        var_lc4_rdn1_slot: &mut f64,
        var_lc4_rdn10_slot: &mut f64,
        var_lc4_rdn11_slot: &mut f64,
        var_lc4_rdn12_slot: &mut f64,
        var_lc4_rdn13_slot: &mut f64,
        var_lc4_rdn14_slot: &mut f64,
        var_lc4_rdn15_slot: &mut f64,
        var_lc4_rdn2_slot: &mut f64,
        var_lc4_rdn3_slot: &mut f64,
        var_lc4_rdn4_slot: &mut f64,
        var_lc4_rdn5_slot: &mut f64,
        var_lc4_rdn6_slot: &mut f64,
        var_lc4_rdn7_slot: &mut f64,
        var_lc4_rdn8_slot: &mut f64,
        var_lc4_rdn9_slot: &mut f64,
        var_lc4_rv_slot: &mut f64,
        var_qgd_slot: &mut f64,
        var_qgd0_slot: &mut f64,
        var_qgd0_db0_slot: &mut f64,
        var_qgd0_db1_slot: &mut f64,
        var_qgd0_db10_slot: &mut f64,
        var_qgd0_db11_slot: &mut f64,
        var_qgd0_db12_slot: &mut f64,
        var_qgd0_db13_slot: &mut f64,
        var_qgd0_db14_slot: &mut f64,
        var_qgd0_db15_slot: &mut f64,
        var_qgd0_db16_slot: &mut f64,
        var_qgd0_db17_slot: &mut f64,
        var_qgd0_db18_slot: &mut f64,
        var_qgd0_db2_slot: &mut f64,
        var_qgd0_db3_slot: &mut f64,
        var_qgd0_db4_slot: &mut f64,
        var_qgd0_db5_slot: &mut f64,
        var_qgd0_db6_slot: &mut f64,
        var_qgd0_db7_slot: &mut f64,
        var_qgd0_db8_slot: &mut f64,
        var_qgd0_db9_slot: &mut f64,
        var_qgd0_dn0_slot: &mut f64,
        var_qgd0_dn1_slot: &mut f64,
        var_qgd0_dn10_slot: &mut f64,
        var_qgd0_dn11_slot: &mut f64,
        var_qgd0_dn12_slot: &mut f64,
        var_qgd0_dn13_slot: &mut f64,
        var_qgd0_dn14_slot: &mut f64,
        var_qgd0_dn15_slot: &mut f64,
        var_qgd0_dn2_slot: &mut f64,
        var_qgd0_dn3_slot: &mut f64,
        var_qgd0_dn4_slot: &mut f64,
        var_qgd0_dn5_slot: &mut f64,
        var_qgd0_dn6_slot: &mut f64,
        var_qgd0_dn7_slot: &mut f64,
        var_qgd0_dn8_slot: &mut f64,
        var_qgd0_dn9_slot: &mut f64,
        var_qgd0_rdb0_slot: &mut f64,
        var_qgd0_rdb1_slot: &mut f64,
        var_qgd0_rdb10_slot: &mut f64,
        var_qgd0_rdb11_slot: &mut f64,
        var_qgd0_rdb12_slot: &mut f64,
        var_qgd0_rdb13_slot: &mut f64,
        var_qgd0_rdb14_slot: &mut f64,
        var_qgd0_rdb15_slot: &mut f64,
        var_qgd0_rdb16_slot: &mut f64,
        var_qgd0_rdb17_slot: &mut f64,
        var_qgd0_rdb18_slot: &mut f64,
        var_qgd0_rdb2_slot: &mut f64,
        var_qgd0_rdb3_slot: &mut f64,
        var_qgd0_rdb4_slot: &mut f64,
        var_qgd0_rdb5_slot: &mut f64,
        var_qgd0_rdb6_slot: &mut f64,
        var_qgd0_rdb7_slot: &mut f64,
        var_qgd0_rdb8_slot: &mut f64,
        var_qgd0_rdb9_slot: &mut f64,
        var_qgd0_rdn0_slot: &mut f64,
        var_qgd0_rdn1_slot: &mut f64,
        var_qgd0_rdn10_slot: &mut f64,
        var_qgd0_rdn11_slot: &mut f64,
        var_qgd0_rdn12_slot: &mut f64,
        var_qgd0_rdn13_slot: &mut f64,
        var_qgd0_rdn14_slot: &mut f64,
        var_qgd0_rdn15_slot: &mut f64,
        var_qgd0_rdn2_slot: &mut f64,
        var_qgd0_rdn3_slot: &mut f64,
        var_qgd0_rdn4_slot: &mut f64,
        var_qgd0_rdn5_slot: &mut f64,
        var_qgd0_rdn6_slot: &mut f64,
        var_qgd0_rdn7_slot: &mut f64,
        var_qgd0_rdn8_slot: &mut f64,
        var_qgd0_rdn9_slot: &mut f64,
        var_qgd0_rv_slot: &mut f64,
        var_qgd_db0_slot: &mut f64,
        var_qgd_db1_slot: &mut f64,
        var_qgd_db10_slot: &mut f64,
        var_qgd_db11_slot: &mut f64,
        var_qgd_db12_slot: &mut f64,
        var_qgd_db13_slot: &mut f64,
        var_qgd_db14_slot: &mut f64,
        var_qgd_db15_slot: &mut f64,
        var_qgd_db16_slot: &mut f64,
        var_qgd_db17_slot: &mut f64,
        var_qgd_db18_slot: &mut f64,
        var_qgd_db2_slot: &mut f64,
        var_qgd_db3_slot: &mut f64,
        var_qgd_db4_slot: &mut f64,
        var_qgd_db5_slot: &mut f64,
        var_qgd_db6_slot: &mut f64,
        var_qgd_db7_slot: &mut f64,
        var_qgd_db8_slot: &mut f64,
        var_qgd_db9_slot: &mut f64,
        var_qgd_dn0_slot: &mut f64,
        var_qgd_dn1_slot: &mut f64,
        var_qgd_dn10_slot: &mut f64,
        var_qgd_dn11_slot: &mut f64,
        var_qgd_dn12_slot: &mut f64,
        var_qgd_dn13_slot: &mut f64,
        var_qgd_dn14_slot: &mut f64,
        var_qgd_dn15_slot: &mut f64,
        var_qgd_dn2_slot: &mut f64,
        var_qgd_dn3_slot: &mut f64,
        var_qgd_dn4_slot: &mut f64,
        var_qgd_dn5_slot: &mut f64,
        var_qgd_dn6_slot: &mut f64,
        var_qgd_dn7_slot: &mut f64,
        var_qgd_dn8_slot: &mut f64,
        var_qgd_dn9_slot: &mut f64,
        var_qgd_rdb0_slot: &mut f64,
        var_qgd_rdb1_slot: &mut f64,
        var_qgd_rdb10_slot: &mut f64,
        var_qgd_rdb11_slot: &mut f64,
        var_qgd_rdb12_slot: &mut f64,
        var_qgd_rdb13_slot: &mut f64,
        var_qgd_rdb14_slot: &mut f64,
        var_qgd_rdb15_slot: &mut f64,
        var_qgd_rdb16_slot: &mut f64,
        var_qgd_rdb17_slot: &mut f64,
        var_qgd_rdb18_slot: &mut f64,
        var_qgd_rdb2_slot: &mut f64,
        var_qgd_rdb3_slot: &mut f64,
        var_qgd_rdb4_slot: &mut f64,
        var_qgd_rdb5_slot: &mut f64,
        var_qgd_rdb6_slot: &mut f64,
        var_qgd_rdb7_slot: &mut f64,
        var_qgd_rdb8_slot: &mut f64,
        var_qgd_rdb9_slot: &mut f64,
        var_qgd_rdn0_slot: &mut f64,
        var_qgd_rdn1_slot: &mut f64,
        var_qgd_rdn10_slot: &mut f64,
        var_qgd_rdn11_slot: &mut f64,
        var_qgd_rdn12_slot: &mut f64,
        var_qgd_rdn13_slot: &mut f64,
        var_qgd_rdn14_slot: &mut f64,
        var_qgd_rdn15_slot: &mut f64,
        var_qgd_rdn2_slot: &mut f64,
        var_qgd_rdn3_slot: &mut f64,
        var_qgd_rdn4_slot: &mut f64,
        var_qgd_rdn5_slot: &mut f64,
        var_qgd_rdn6_slot: &mut f64,
        var_qgd_rdn7_slot: &mut f64,
        var_qgd_rdn8_slot: &mut f64,
        var_qgd_rdn9_slot: &mut f64,
        var_qgd_rv_slot: &mut f64,
    ) {
        let mut var_cgd: f64 = *var_cgd_slot;
        let mut var_cgd_db0: f64 = *var_cgd_db0_slot;
        let mut var_cgd_db1: f64 = *var_cgd_db1_slot;
        let mut var_cgd_db10: f64 = *var_cgd_db10_slot;
        let mut var_cgd_db11: f64 = *var_cgd_db11_slot;
        let mut var_cgd_db12: f64 = *var_cgd_db12_slot;
        let mut var_cgd_db13: f64 = *var_cgd_db13_slot;
        let mut var_cgd_db14: f64 = *var_cgd_db14_slot;
        let mut var_cgd_db15: f64 = *var_cgd_db15_slot;
        let mut var_cgd_db16: f64 = *var_cgd_db16_slot;
        let mut var_cgd_db17: f64 = *var_cgd_db17_slot;
        let mut var_cgd_db18: f64 = *var_cgd_db18_slot;
        let mut var_cgd_db2: f64 = *var_cgd_db2_slot;
        let mut var_cgd_db3: f64 = *var_cgd_db3_slot;
        let mut var_cgd_db4: f64 = *var_cgd_db4_slot;
        let mut var_cgd_db5: f64 = *var_cgd_db5_slot;
        let mut var_cgd_db6: f64 = *var_cgd_db6_slot;
        let mut var_cgd_db7: f64 = *var_cgd_db7_slot;
        let mut var_cgd_db8: f64 = *var_cgd_db8_slot;
        let mut var_cgd_db9: f64 = *var_cgd_db9_slot;
        let mut var_cgd_dn0: f64 = *var_cgd_dn0_slot;
        let mut var_cgd_dn1: f64 = *var_cgd_dn1_slot;
        let mut var_cgd_dn10: f64 = *var_cgd_dn10_slot;
        let mut var_cgd_dn11: f64 = *var_cgd_dn11_slot;
        let mut var_cgd_dn12: f64 = *var_cgd_dn12_slot;
        let mut var_cgd_dn13: f64 = *var_cgd_dn13_slot;
        let mut var_cgd_dn14: f64 = *var_cgd_dn14_slot;
        let mut var_cgd_dn15: f64 = *var_cgd_dn15_slot;
        let mut var_cgd_dn2: f64 = *var_cgd_dn2_slot;
        let mut var_cgd_dn3: f64 = *var_cgd_dn3_slot;
        let mut var_cgd_dn4: f64 = *var_cgd_dn4_slot;
        let mut var_cgd_dn5: f64 = *var_cgd_dn5_slot;
        let mut var_cgd_dn6: f64 = *var_cgd_dn6_slot;
        let mut var_cgd_dn7: f64 = *var_cgd_dn7_slot;
        let mut var_cgd_dn8: f64 = *var_cgd_dn8_slot;
        let mut var_cgd_dn9: f64 = *var_cgd_dn9_slot;
        let mut var_cgd_rdb0: f64 = *var_cgd_rdb0_slot;
        let mut var_cgd_rdb1: f64 = *var_cgd_rdb1_slot;
        let mut var_cgd_rdb10: f64 = *var_cgd_rdb10_slot;
        let mut var_cgd_rdb11: f64 = *var_cgd_rdb11_slot;
        let mut var_cgd_rdb12: f64 = *var_cgd_rdb12_slot;
        let mut var_cgd_rdb13: f64 = *var_cgd_rdb13_slot;
        let mut var_cgd_rdb14: f64 = *var_cgd_rdb14_slot;
        let mut var_cgd_rdb15: f64 = *var_cgd_rdb15_slot;
        let mut var_cgd_rdb16: f64 = *var_cgd_rdb16_slot;
        let mut var_cgd_rdb17: f64 = *var_cgd_rdb17_slot;
        let mut var_cgd_rdb18: f64 = *var_cgd_rdb18_slot;
        let mut var_cgd_rdb2: f64 = *var_cgd_rdb2_slot;
        let mut var_cgd_rdb3: f64 = *var_cgd_rdb3_slot;
        let mut var_cgd_rdb4: f64 = *var_cgd_rdb4_slot;
        let mut var_cgd_rdb5: f64 = *var_cgd_rdb5_slot;
        let mut var_cgd_rdb6: f64 = *var_cgd_rdb6_slot;
        let mut var_cgd_rdb7: f64 = *var_cgd_rdb7_slot;
        let mut var_cgd_rdb8: f64 = *var_cgd_rdb8_slot;
        let mut var_cgd_rdb9: f64 = *var_cgd_rdb9_slot;
        let mut var_cgd_rdn0: f64 = *var_cgd_rdn0_slot;
        let mut var_cgd_rdn1: f64 = *var_cgd_rdn1_slot;
        let mut var_cgd_rdn10: f64 = *var_cgd_rdn10_slot;
        let mut var_cgd_rdn11: f64 = *var_cgd_rdn11_slot;
        let mut var_cgd_rdn12: f64 = *var_cgd_rdn12_slot;
        let mut var_cgd_rdn13: f64 = *var_cgd_rdn13_slot;
        let mut var_cgd_rdn14: f64 = *var_cgd_rdn14_slot;
        let mut var_cgd_rdn15: f64 = *var_cgd_rdn15_slot;
        let mut var_cgd_rdn2: f64 = *var_cgd_rdn2_slot;
        let mut var_cgd_rdn3: f64 = *var_cgd_rdn3_slot;
        let mut var_cgd_rdn4: f64 = *var_cgd_rdn4_slot;
        let mut var_cgd_rdn5: f64 = *var_cgd_rdn5_slot;
        let mut var_cgd_rdn6: f64 = *var_cgd_rdn6_slot;
        let mut var_cgd_rdn7: f64 = *var_cgd_rdn7_slot;
        let mut var_cgd_rdn8: f64 = *var_cgd_rdn8_slot;
        let mut var_cgd_rdn9: f64 = *var_cgd_rdn9_slot;
        let mut var_cgd_rv: f64 = *var_cgd_rv_slot;
        let mut var_cgs: f64 = *var_cgs_slot;
        let mut var_cgs_db0: f64 = *var_cgs_db0_slot;
        let mut var_cgs_db1: f64 = *var_cgs_db1_slot;
        let mut var_cgs_db10: f64 = *var_cgs_db10_slot;
        let mut var_cgs_db11: f64 = *var_cgs_db11_slot;
        let mut var_cgs_db12: f64 = *var_cgs_db12_slot;
        let mut var_cgs_db13: f64 = *var_cgs_db13_slot;
        let mut var_cgs_db14: f64 = *var_cgs_db14_slot;
        let mut var_cgs_db15: f64 = *var_cgs_db15_slot;
        let mut var_cgs_db16: f64 = *var_cgs_db16_slot;
        let mut var_cgs_db17: f64 = *var_cgs_db17_slot;
        let mut var_cgs_db18: f64 = *var_cgs_db18_slot;
        let mut var_cgs_db2: f64 = *var_cgs_db2_slot;
        let mut var_cgs_db3: f64 = *var_cgs_db3_slot;
        let mut var_cgs_db4: f64 = *var_cgs_db4_slot;
        let mut var_cgs_db5: f64 = *var_cgs_db5_slot;
        let mut var_cgs_db6: f64 = *var_cgs_db6_slot;
        let mut var_cgs_db7: f64 = *var_cgs_db7_slot;
        let mut var_cgs_db8: f64 = *var_cgs_db8_slot;
        let mut var_cgs_db9: f64 = *var_cgs_db9_slot;
        let mut var_cgs_dn0: f64 = *var_cgs_dn0_slot;
        let mut var_cgs_dn1: f64 = *var_cgs_dn1_slot;
        let mut var_cgs_dn10: f64 = *var_cgs_dn10_slot;
        let mut var_cgs_dn11: f64 = *var_cgs_dn11_slot;
        let mut var_cgs_dn12: f64 = *var_cgs_dn12_slot;
        let mut var_cgs_dn13: f64 = *var_cgs_dn13_slot;
        let mut var_cgs_dn14: f64 = *var_cgs_dn14_slot;
        let mut var_cgs_dn15: f64 = *var_cgs_dn15_slot;
        let mut var_cgs_dn2: f64 = *var_cgs_dn2_slot;
        let mut var_cgs_dn3: f64 = *var_cgs_dn3_slot;
        let mut var_cgs_dn4: f64 = *var_cgs_dn4_slot;
        let mut var_cgs_dn5: f64 = *var_cgs_dn5_slot;
        let mut var_cgs_dn6: f64 = *var_cgs_dn6_slot;
        let mut var_cgs_dn7: f64 = *var_cgs_dn7_slot;
        let mut var_cgs_dn8: f64 = *var_cgs_dn8_slot;
        let mut var_cgs_dn9: f64 = *var_cgs_dn9_slot;
        let mut var_cgs_rdb0: f64 = *var_cgs_rdb0_slot;
        let mut var_cgs_rdb1: f64 = *var_cgs_rdb1_slot;
        let mut var_cgs_rdb10: f64 = *var_cgs_rdb10_slot;
        let mut var_cgs_rdb11: f64 = *var_cgs_rdb11_slot;
        let mut var_cgs_rdb12: f64 = *var_cgs_rdb12_slot;
        let mut var_cgs_rdb13: f64 = *var_cgs_rdb13_slot;
        let mut var_cgs_rdb14: f64 = *var_cgs_rdb14_slot;
        let mut var_cgs_rdb15: f64 = *var_cgs_rdb15_slot;
        let mut var_cgs_rdb16: f64 = *var_cgs_rdb16_slot;
        let mut var_cgs_rdb17: f64 = *var_cgs_rdb17_slot;
        let mut var_cgs_rdb18: f64 = *var_cgs_rdb18_slot;
        let mut var_cgs_rdb2: f64 = *var_cgs_rdb2_slot;
        let mut var_cgs_rdb3: f64 = *var_cgs_rdb3_slot;
        let mut var_cgs_rdb4: f64 = *var_cgs_rdb4_slot;
        let mut var_cgs_rdb5: f64 = *var_cgs_rdb5_slot;
        let mut var_cgs_rdb6: f64 = *var_cgs_rdb6_slot;
        let mut var_cgs_rdb7: f64 = *var_cgs_rdb7_slot;
        let mut var_cgs_rdb8: f64 = *var_cgs_rdb8_slot;
        let mut var_cgs_rdb9: f64 = *var_cgs_rdb9_slot;
        let mut var_cgs_rdn0: f64 = *var_cgs_rdn0_slot;
        let mut var_cgs_rdn1: f64 = *var_cgs_rdn1_slot;
        let mut var_cgs_rdn10: f64 = *var_cgs_rdn10_slot;
        let mut var_cgs_rdn11: f64 = *var_cgs_rdn11_slot;
        let mut var_cgs_rdn12: f64 = *var_cgs_rdn12_slot;
        let mut var_cgs_rdn13: f64 = *var_cgs_rdn13_slot;
        let mut var_cgs_rdn14: f64 = *var_cgs_rdn14_slot;
        let mut var_cgs_rdn15: f64 = *var_cgs_rdn15_slot;
        let mut var_cgs_rdn2: f64 = *var_cgs_rdn2_slot;
        let mut var_cgs_rdn3: f64 = *var_cgs_rdn3_slot;
        let mut var_cgs_rdn4: f64 = *var_cgs_rdn4_slot;
        let mut var_cgs_rdn5: f64 = *var_cgs_rdn5_slot;
        let mut var_cgs_rdn6: f64 = *var_cgs_rdn6_slot;
        let mut var_cgs_rdn7: f64 = *var_cgs_rdn7_slot;
        let mut var_cgs_rdn8: f64 = *var_cgs_rdn8_slot;
        let mut var_cgs_rdn9: f64 = *var_cgs_rdn9_slot;
        let mut var_cgs_rv: f64 = *var_cgs_rv_slot;
        let mut var_cosh1: f64 = *var_cosh1_slot;
        let mut var_cosh1_db0: f64 = *var_cosh1_db0_slot;
        let mut var_cosh1_db1: f64 = *var_cosh1_db1_slot;
        let mut var_cosh1_db10: f64 = *var_cosh1_db10_slot;
        let mut var_cosh1_db11: f64 = *var_cosh1_db11_slot;
        let mut var_cosh1_db12: f64 = *var_cosh1_db12_slot;
        let mut var_cosh1_db13: f64 = *var_cosh1_db13_slot;
        let mut var_cosh1_db14: f64 = *var_cosh1_db14_slot;
        let mut var_cosh1_db15: f64 = *var_cosh1_db15_slot;
        let mut var_cosh1_db16: f64 = *var_cosh1_db16_slot;
        let mut var_cosh1_db17: f64 = *var_cosh1_db17_slot;
        let mut var_cosh1_db18: f64 = *var_cosh1_db18_slot;
        let mut var_cosh1_db2: f64 = *var_cosh1_db2_slot;
        let mut var_cosh1_db3: f64 = *var_cosh1_db3_slot;
        let mut var_cosh1_db4: f64 = *var_cosh1_db4_slot;
        let mut var_cosh1_db5: f64 = *var_cosh1_db5_slot;
        let mut var_cosh1_db6: f64 = *var_cosh1_db6_slot;
        let mut var_cosh1_db7: f64 = *var_cosh1_db7_slot;
        let mut var_cosh1_db8: f64 = *var_cosh1_db8_slot;
        let mut var_cosh1_db9: f64 = *var_cosh1_db9_slot;
        let mut var_cosh1_dn0: f64 = *var_cosh1_dn0_slot;
        let mut var_cosh1_dn1: f64 = *var_cosh1_dn1_slot;
        let mut var_cosh1_dn10: f64 = *var_cosh1_dn10_slot;
        let mut var_cosh1_dn11: f64 = *var_cosh1_dn11_slot;
        let mut var_cosh1_dn12: f64 = *var_cosh1_dn12_slot;
        let mut var_cosh1_dn13: f64 = *var_cosh1_dn13_slot;
        let mut var_cosh1_dn14: f64 = *var_cosh1_dn14_slot;
        let mut var_cosh1_dn15: f64 = *var_cosh1_dn15_slot;
        let mut var_cosh1_dn2: f64 = *var_cosh1_dn2_slot;
        let mut var_cosh1_dn3: f64 = *var_cosh1_dn3_slot;
        let mut var_cosh1_dn4: f64 = *var_cosh1_dn4_slot;
        let mut var_cosh1_dn5: f64 = *var_cosh1_dn5_slot;
        let mut var_cosh1_dn6: f64 = *var_cosh1_dn6_slot;
        let mut var_cosh1_dn7: f64 = *var_cosh1_dn7_slot;
        let mut var_cosh1_dn8: f64 = *var_cosh1_dn8_slot;
        let mut var_cosh1_dn9: f64 = *var_cosh1_dn9_slot;
        let mut var_cosh1_rdb0: f64 = *var_cosh1_rdb0_slot;
        let mut var_cosh1_rdb1: f64 = *var_cosh1_rdb1_slot;
        let mut var_cosh1_rdb10: f64 = *var_cosh1_rdb10_slot;
        let mut var_cosh1_rdb11: f64 = *var_cosh1_rdb11_slot;
        let mut var_cosh1_rdb12: f64 = *var_cosh1_rdb12_slot;
        let mut var_cosh1_rdb13: f64 = *var_cosh1_rdb13_slot;
        let mut var_cosh1_rdb14: f64 = *var_cosh1_rdb14_slot;
        let mut var_cosh1_rdb15: f64 = *var_cosh1_rdb15_slot;
        let mut var_cosh1_rdb16: f64 = *var_cosh1_rdb16_slot;
        let mut var_cosh1_rdb17: f64 = *var_cosh1_rdb17_slot;
        let mut var_cosh1_rdb18: f64 = *var_cosh1_rdb18_slot;
        let mut var_cosh1_rdb2: f64 = *var_cosh1_rdb2_slot;
        let mut var_cosh1_rdb3: f64 = *var_cosh1_rdb3_slot;
        let mut var_cosh1_rdb4: f64 = *var_cosh1_rdb4_slot;
        let mut var_cosh1_rdb5: f64 = *var_cosh1_rdb5_slot;
        let mut var_cosh1_rdb6: f64 = *var_cosh1_rdb6_slot;
        let mut var_cosh1_rdb7: f64 = *var_cosh1_rdb7_slot;
        let mut var_cosh1_rdb8: f64 = *var_cosh1_rdb8_slot;
        let mut var_cosh1_rdb9: f64 = *var_cosh1_rdb9_slot;
        let mut var_cosh1_rdn0: f64 = *var_cosh1_rdn0_slot;
        let mut var_cosh1_rdn1: f64 = *var_cosh1_rdn1_slot;
        let mut var_cosh1_rdn10: f64 = *var_cosh1_rdn10_slot;
        let mut var_cosh1_rdn11: f64 = *var_cosh1_rdn11_slot;
        let mut var_cosh1_rdn12: f64 = *var_cosh1_rdn12_slot;
        let mut var_cosh1_rdn13: f64 = *var_cosh1_rdn13_slot;
        let mut var_cosh1_rdn14: f64 = *var_cosh1_rdn14_slot;
        let mut var_cosh1_rdn15: f64 = *var_cosh1_rdn15_slot;
        let mut var_cosh1_rdn2: f64 = *var_cosh1_rdn2_slot;
        let mut var_cosh1_rdn3: f64 = *var_cosh1_rdn3_slot;
        let mut var_cosh1_rdn4: f64 = *var_cosh1_rdn4_slot;
        let mut var_cosh1_rdn5: f64 = *var_cosh1_rdn5_slot;
        let mut var_cosh1_rdn6: f64 = *var_cosh1_rdn6_slot;
        let mut var_cosh1_rdn7: f64 = *var_cosh1_rdn7_slot;
        let mut var_cosh1_rdn8: f64 = *var_cosh1_rdn8_slot;
        let mut var_cosh1_rdn9: f64 = *var_cosh1_rdn9_slot;
        let mut var_cosh1_rv: f64 = *var_cosh1_rv_slot;
        let mut var_lc4: f64 = *var_lc4_slot;
        let mut var_lc4_db0: f64 = *var_lc4_db0_slot;
        let mut var_lc4_db1: f64 = *var_lc4_db1_slot;
        let mut var_lc4_db10: f64 = *var_lc4_db10_slot;
        let mut var_lc4_db11: f64 = *var_lc4_db11_slot;
        let mut var_lc4_db12: f64 = *var_lc4_db12_slot;
        let mut var_lc4_db13: f64 = *var_lc4_db13_slot;
        let mut var_lc4_db14: f64 = *var_lc4_db14_slot;
        let mut var_lc4_db15: f64 = *var_lc4_db15_slot;
        let mut var_lc4_db16: f64 = *var_lc4_db16_slot;
        let mut var_lc4_db17: f64 = *var_lc4_db17_slot;
        let mut var_lc4_db18: f64 = *var_lc4_db18_slot;
        let mut var_lc4_db2: f64 = *var_lc4_db2_slot;
        let mut var_lc4_db3: f64 = *var_lc4_db3_slot;
        let mut var_lc4_db4: f64 = *var_lc4_db4_slot;
        let mut var_lc4_db5: f64 = *var_lc4_db5_slot;
        let mut var_lc4_db6: f64 = *var_lc4_db6_slot;
        let mut var_lc4_db7: f64 = *var_lc4_db7_slot;
        let mut var_lc4_db8: f64 = *var_lc4_db8_slot;
        let mut var_lc4_db9: f64 = *var_lc4_db9_slot;
        let mut var_lc4_dn0: f64 = *var_lc4_dn0_slot;
        let mut var_lc4_dn1: f64 = *var_lc4_dn1_slot;
        let mut var_lc4_dn10: f64 = *var_lc4_dn10_slot;
        let mut var_lc4_dn11: f64 = *var_lc4_dn11_slot;
        let mut var_lc4_dn12: f64 = *var_lc4_dn12_slot;
        let mut var_lc4_dn13: f64 = *var_lc4_dn13_slot;
        let mut var_lc4_dn14: f64 = *var_lc4_dn14_slot;
        let mut var_lc4_dn15: f64 = *var_lc4_dn15_slot;
        let mut var_lc4_dn2: f64 = *var_lc4_dn2_slot;
        let mut var_lc4_dn3: f64 = *var_lc4_dn3_slot;
        let mut var_lc4_dn4: f64 = *var_lc4_dn4_slot;
        let mut var_lc4_dn5: f64 = *var_lc4_dn5_slot;
        let mut var_lc4_dn6: f64 = *var_lc4_dn6_slot;
        let mut var_lc4_dn7: f64 = *var_lc4_dn7_slot;
        let mut var_lc4_dn8: f64 = *var_lc4_dn8_slot;
        let mut var_lc4_dn9: f64 = *var_lc4_dn9_slot;
        let mut var_lc4_rdb0: f64 = *var_lc4_rdb0_slot;
        let mut var_lc4_rdb1: f64 = *var_lc4_rdb1_slot;
        let mut var_lc4_rdb10: f64 = *var_lc4_rdb10_slot;
        let mut var_lc4_rdb11: f64 = *var_lc4_rdb11_slot;
        let mut var_lc4_rdb12: f64 = *var_lc4_rdb12_slot;
        let mut var_lc4_rdb13: f64 = *var_lc4_rdb13_slot;
        let mut var_lc4_rdb14: f64 = *var_lc4_rdb14_slot;
        let mut var_lc4_rdb15: f64 = *var_lc4_rdb15_slot;
        let mut var_lc4_rdb16: f64 = *var_lc4_rdb16_slot;
        let mut var_lc4_rdb17: f64 = *var_lc4_rdb17_slot;
        let mut var_lc4_rdb18: f64 = *var_lc4_rdb18_slot;
        let mut var_lc4_rdb2: f64 = *var_lc4_rdb2_slot;
        let mut var_lc4_rdb3: f64 = *var_lc4_rdb3_slot;
        let mut var_lc4_rdb4: f64 = *var_lc4_rdb4_slot;
        let mut var_lc4_rdb5: f64 = *var_lc4_rdb5_slot;
        let mut var_lc4_rdb6: f64 = *var_lc4_rdb6_slot;
        let mut var_lc4_rdb7: f64 = *var_lc4_rdb7_slot;
        let mut var_lc4_rdb8: f64 = *var_lc4_rdb8_slot;
        let mut var_lc4_rdb9: f64 = *var_lc4_rdb9_slot;
        let mut var_lc4_rdn0: f64 = *var_lc4_rdn0_slot;
        let mut var_lc4_rdn1: f64 = *var_lc4_rdn1_slot;
        let mut var_lc4_rdn10: f64 = *var_lc4_rdn10_slot;
        let mut var_lc4_rdn11: f64 = *var_lc4_rdn11_slot;
        let mut var_lc4_rdn12: f64 = *var_lc4_rdn12_slot;
        let mut var_lc4_rdn13: f64 = *var_lc4_rdn13_slot;
        let mut var_lc4_rdn14: f64 = *var_lc4_rdn14_slot;
        let mut var_lc4_rdn15: f64 = *var_lc4_rdn15_slot;
        let mut var_lc4_rdn2: f64 = *var_lc4_rdn2_slot;
        let mut var_lc4_rdn3: f64 = *var_lc4_rdn3_slot;
        let mut var_lc4_rdn4: f64 = *var_lc4_rdn4_slot;
        let mut var_lc4_rdn5: f64 = *var_lc4_rdn5_slot;
        let mut var_lc4_rdn6: f64 = *var_lc4_rdn6_slot;
        let mut var_lc4_rdn7: f64 = *var_lc4_rdn7_slot;
        let mut var_lc4_rdn8: f64 = *var_lc4_rdn8_slot;
        let mut var_lc4_rdn9: f64 = *var_lc4_rdn9_slot;
        let mut var_lc4_rv: f64 = *var_lc4_rv_slot;
        let mut var_qgd: f64 = *var_qgd_slot;
        let mut var_qgd0: f64 = *var_qgd0_slot;
        let mut var_qgd0_db0: f64 = *var_qgd0_db0_slot;
        let mut var_qgd0_db1: f64 = *var_qgd0_db1_slot;
        let mut var_qgd0_db10: f64 = *var_qgd0_db10_slot;
        let mut var_qgd0_db11: f64 = *var_qgd0_db11_slot;
        let mut var_qgd0_db12: f64 = *var_qgd0_db12_slot;
        let mut var_qgd0_db13: f64 = *var_qgd0_db13_slot;
        let mut var_qgd0_db14: f64 = *var_qgd0_db14_slot;
        let mut var_qgd0_db15: f64 = *var_qgd0_db15_slot;
        let mut var_qgd0_db16: f64 = *var_qgd0_db16_slot;
        let mut var_qgd0_db17: f64 = *var_qgd0_db17_slot;
        let mut var_qgd0_db18: f64 = *var_qgd0_db18_slot;
        let mut var_qgd0_db2: f64 = *var_qgd0_db2_slot;
        let mut var_qgd0_db3: f64 = *var_qgd0_db3_slot;
        let mut var_qgd0_db4: f64 = *var_qgd0_db4_slot;
        let mut var_qgd0_db5: f64 = *var_qgd0_db5_slot;
        let mut var_qgd0_db6: f64 = *var_qgd0_db6_slot;
        let mut var_qgd0_db7: f64 = *var_qgd0_db7_slot;
        let mut var_qgd0_db8: f64 = *var_qgd0_db8_slot;
        let mut var_qgd0_db9: f64 = *var_qgd0_db9_slot;
        let mut var_qgd0_dn0: f64 = *var_qgd0_dn0_slot;
        let mut var_qgd0_dn1: f64 = *var_qgd0_dn1_slot;
        let mut var_qgd0_dn10: f64 = *var_qgd0_dn10_slot;
        let mut var_qgd0_dn11: f64 = *var_qgd0_dn11_slot;
        let mut var_qgd0_dn12: f64 = *var_qgd0_dn12_slot;
        let mut var_qgd0_dn13: f64 = *var_qgd0_dn13_slot;
        let mut var_qgd0_dn14: f64 = *var_qgd0_dn14_slot;
        let mut var_qgd0_dn15: f64 = *var_qgd0_dn15_slot;
        let mut var_qgd0_dn2: f64 = *var_qgd0_dn2_slot;
        let mut var_qgd0_dn3: f64 = *var_qgd0_dn3_slot;
        let mut var_qgd0_dn4: f64 = *var_qgd0_dn4_slot;
        let mut var_qgd0_dn5: f64 = *var_qgd0_dn5_slot;
        let mut var_qgd0_dn6: f64 = *var_qgd0_dn6_slot;
        let mut var_qgd0_dn7: f64 = *var_qgd0_dn7_slot;
        let mut var_qgd0_dn8: f64 = *var_qgd0_dn8_slot;
        let mut var_qgd0_dn9: f64 = *var_qgd0_dn9_slot;
        let mut var_qgd0_rdb0: f64 = *var_qgd0_rdb0_slot;
        let mut var_qgd0_rdb1: f64 = *var_qgd0_rdb1_slot;
        let mut var_qgd0_rdb10: f64 = *var_qgd0_rdb10_slot;
        let mut var_qgd0_rdb11: f64 = *var_qgd0_rdb11_slot;
        let mut var_qgd0_rdb12: f64 = *var_qgd0_rdb12_slot;
        let mut var_qgd0_rdb13: f64 = *var_qgd0_rdb13_slot;
        let mut var_qgd0_rdb14: f64 = *var_qgd0_rdb14_slot;
        let mut var_qgd0_rdb15: f64 = *var_qgd0_rdb15_slot;
        let mut var_qgd0_rdb16: f64 = *var_qgd0_rdb16_slot;
        let mut var_qgd0_rdb17: f64 = *var_qgd0_rdb17_slot;
        let mut var_qgd0_rdb18: f64 = *var_qgd0_rdb18_slot;
        let mut var_qgd0_rdb2: f64 = *var_qgd0_rdb2_slot;
        let mut var_qgd0_rdb3: f64 = *var_qgd0_rdb3_slot;
        let mut var_qgd0_rdb4: f64 = *var_qgd0_rdb4_slot;
        let mut var_qgd0_rdb5: f64 = *var_qgd0_rdb5_slot;
        let mut var_qgd0_rdb6: f64 = *var_qgd0_rdb6_slot;
        let mut var_qgd0_rdb7: f64 = *var_qgd0_rdb7_slot;
        let mut var_qgd0_rdb8: f64 = *var_qgd0_rdb8_slot;
        let mut var_qgd0_rdb9: f64 = *var_qgd0_rdb9_slot;
        let mut var_qgd0_rdn0: f64 = *var_qgd0_rdn0_slot;
        let mut var_qgd0_rdn1: f64 = *var_qgd0_rdn1_slot;
        let mut var_qgd0_rdn10: f64 = *var_qgd0_rdn10_slot;
        let mut var_qgd0_rdn11: f64 = *var_qgd0_rdn11_slot;
        let mut var_qgd0_rdn12: f64 = *var_qgd0_rdn12_slot;
        let mut var_qgd0_rdn13: f64 = *var_qgd0_rdn13_slot;
        let mut var_qgd0_rdn14: f64 = *var_qgd0_rdn14_slot;
        let mut var_qgd0_rdn15: f64 = *var_qgd0_rdn15_slot;
        let mut var_qgd0_rdn2: f64 = *var_qgd0_rdn2_slot;
        let mut var_qgd0_rdn3: f64 = *var_qgd0_rdn3_slot;
        let mut var_qgd0_rdn4: f64 = *var_qgd0_rdn4_slot;
        let mut var_qgd0_rdn5: f64 = *var_qgd0_rdn5_slot;
        let mut var_qgd0_rdn6: f64 = *var_qgd0_rdn6_slot;
        let mut var_qgd0_rdn7: f64 = *var_qgd0_rdn7_slot;
        let mut var_qgd0_rdn8: f64 = *var_qgd0_rdn8_slot;
        let mut var_qgd0_rdn9: f64 = *var_qgd0_rdn9_slot;
        let mut var_qgd0_rv: f64 = *var_qgd0_rv_slot;
        let mut var_qgd_db0: f64 = *var_qgd_db0_slot;
        let mut var_qgd_db1: f64 = *var_qgd_db1_slot;
        let mut var_qgd_db10: f64 = *var_qgd_db10_slot;
        let mut var_qgd_db11: f64 = *var_qgd_db11_slot;
        let mut var_qgd_db12: f64 = *var_qgd_db12_slot;
        let mut var_qgd_db13: f64 = *var_qgd_db13_slot;
        let mut var_qgd_db14: f64 = *var_qgd_db14_slot;
        let mut var_qgd_db15: f64 = *var_qgd_db15_slot;
        let mut var_qgd_db16: f64 = *var_qgd_db16_slot;
        let mut var_qgd_db17: f64 = *var_qgd_db17_slot;
        let mut var_qgd_db18: f64 = *var_qgd_db18_slot;
        let mut var_qgd_db2: f64 = *var_qgd_db2_slot;
        let mut var_qgd_db3: f64 = *var_qgd_db3_slot;
        let mut var_qgd_db4: f64 = *var_qgd_db4_slot;
        let mut var_qgd_db5: f64 = *var_qgd_db5_slot;
        let mut var_qgd_db6: f64 = *var_qgd_db6_slot;
        let mut var_qgd_db7: f64 = *var_qgd_db7_slot;
        let mut var_qgd_db8: f64 = *var_qgd_db8_slot;
        let mut var_qgd_db9: f64 = *var_qgd_db9_slot;
        let mut var_qgd_dn0: f64 = *var_qgd_dn0_slot;
        let mut var_qgd_dn1: f64 = *var_qgd_dn1_slot;
        let mut var_qgd_dn10: f64 = *var_qgd_dn10_slot;
        let mut var_qgd_dn11: f64 = *var_qgd_dn11_slot;
        let mut var_qgd_dn12: f64 = *var_qgd_dn12_slot;
        let mut var_qgd_dn13: f64 = *var_qgd_dn13_slot;
        let mut var_qgd_dn14: f64 = *var_qgd_dn14_slot;
        let mut var_qgd_dn15: f64 = *var_qgd_dn15_slot;
        let mut var_qgd_dn2: f64 = *var_qgd_dn2_slot;
        let mut var_qgd_dn3: f64 = *var_qgd_dn3_slot;
        let mut var_qgd_dn4: f64 = *var_qgd_dn4_slot;
        let mut var_qgd_dn5: f64 = *var_qgd_dn5_slot;
        let mut var_qgd_dn6: f64 = *var_qgd_dn6_slot;
        let mut var_qgd_dn7: f64 = *var_qgd_dn7_slot;
        let mut var_qgd_dn8: f64 = *var_qgd_dn8_slot;
        let mut var_qgd_dn9: f64 = *var_qgd_dn9_slot;
        let mut var_qgd_rdb0: f64 = *var_qgd_rdb0_slot;
        let mut var_qgd_rdb1: f64 = *var_qgd_rdb1_slot;
        let mut var_qgd_rdb10: f64 = *var_qgd_rdb10_slot;
        let mut var_qgd_rdb11: f64 = *var_qgd_rdb11_slot;
        let mut var_qgd_rdb12: f64 = *var_qgd_rdb12_slot;
        let mut var_qgd_rdb13: f64 = *var_qgd_rdb13_slot;
        let mut var_qgd_rdb14: f64 = *var_qgd_rdb14_slot;
        let mut var_qgd_rdb15: f64 = *var_qgd_rdb15_slot;
        let mut var_qgd_rdb16: f64 = *var_qgd_rdb16_slot;
        let mut var_qgd_rdb17: f64 = *var_qgd_rdb17_slot;
        let mut var_qgd_rdb18: f64 = *var_qgd_rdb18_slot;
        let mut var_qgd_rdb2: f64 = *var_qgd_rdb2_slot;
        let mut var_qgd_rdb3: f64 = *var_qgd_rdb3_slot;
        let mut var_qgd_rdb4: f64 = *var_qgd_rdb4_slot;
        let mut var_qgd_rdb5: f64 = *var_qgd_rdb5_slot;
        let mut var_qgd_rdb6: f64 = *var_qgd_rdb6_slot;
        let mut var_qgd_rdb7: f64 = *var_qgd_rdb7_slot;
        let mut var_qgd_rdb8: f64 = *var_qgd_rdb8_slot;
        let mut var_qgd_rdb9: f64 = *var_qgd_rdb9_slot;
        let mut var_qgd_rdn0: f64 = *var_qgd_rdn0_slot;
        let mut var_qgd_rdn1: f64 = *var_qgd_rdn1_slot;
        let mut var_qgd_rdn10: f64 = *var_qgd_rdn10_slot;
        let mut var_qgd_rdn11: f64 = *var_qgd_rdn11_slot;
        let mut var_qgd_rdn12: f64 = *var_qgd_rdn12_slot;
        let mut var_qgd_rdn13: f64 = *var_qgd_rdn13_slot;
        let mut var_qgd_rdn14: f64 = *var_qgd_rdn14_slot;
        let mut var_qgd_rdn15: f64 = *var_qgd_rdn15_slot;
        let mut var_qgd_rdn2: f64 = *var_qgd_rdn2_slot;
        let mut var_qgd_rdn3: f64 = *var_qgd_rdn3_slot;
        let mut var_qgd_rdn4: f64 = *var_qgd_rdn4_slot;
        let mut var_qgd_rdn5: f64 = *var_qgd_rdn5_slot;
        let mut var_qgd_rdn6: f64 = *var_qgd_rdn6_slot;
        let mut var_qgd_rdn7: f64 = *var_qgd_rdn7_slot;
        let mut var_qgd_rdn8: f64 = *var_qgd_rdn8_slot;
        let mut var_qgd_rdn9: f64 = *var_qgd_rdn9_slot;
        let mut var_qgd_rv: f64 = *var_qgd_rv_slot;

        let (assign1520_e1946, assign1520_e1946_d_n0, assign1520_e1946_d_n1, assign1520_e1946_d_n2, assign1520_e1946_d_n3, assign1520_e1946_d_n4, assign1520_e1946_d_n5, assign1520_e1946_d_n6, assign1520_e1946_d_n7, assign1520_e1946_d_n8, assign1520_e1946_d_n9, assign1520_e1946_d_n10, assign1520_e1946_d_n11, assign1520_e1946_d_n12, assign1520_e1946_d_n13, assign1520_e1946_d_n14, assign1520_e1946_d_n15, assign1520_e1946_d_b0, assign1520_e1946_d_b1, assign1520_e1946_d_b2, assign1520_e1946_d_b3, assign1520_e1946_d_b4, assign1520_e1946_d_b5, assign1520_e1946_d_b6, assign1520_e1946_d_b7, assign1520_e1946_d_b8, assign1520_e1946_d_b9, assign1520_e1946_d_b10, assign1520_e1946_d_b11, assign1520_e1946_d_b12, assign1520_e1946_d_b13, assign1520_e1946_d_b14,) = {
    if ((var_guard15 != 0.0) && (!((var_guard13 != 0.0) || (var_guard14 != 0.0)))) {
        let assign1520_e1944: f64 = (var_psi_4).cosh();
        (assign1520_e1944, ((var_psi_4).sinh() * var_psi_4_dn0), ((var_psi_4).sinh() * var_psi_4_dn1), ((var_psi_4).sinh() * var_psi_4_dn2), ((var_psi_4).sinh() * var_psi_4_dn3), ((var_psi_4).sinh() * var_psi_4_dn4), ((var_psi_4).sinh() * var_psi_4_dn5), ((var_psi_4).sinh() * var_psi_4_dn6), ((var_psi_4).sinh() * var_psi_4_dn7), ((var_psi_4).sinh() * var_psi_4_dn8), ((var_psi_4).sinh() * var_psi_4_dn9), ((var_psi_4).sinh() * var_psi_4_dn10), ((var_psi_4).sinh() * var_psi_4_dn11), ((var_psi_4).sinh() * var_psi_4_dn12), ((var_psi_4).sinh() * var_psi_4_dn13), ((var_psi_4).sinh() * var_psi_4_dn14), ((var_psi_4).sinh() * var_psi_4_dn15), ((var_psi_4).sinh() * var_psi_4_db0), ((var_psi_4).sinh() * var_psi_4_db1), ((var_psi_4).sinh() * var_psi_4_db2), ((var_psi_4).sinh() * var_psi_4_db3), ((var_psi_4).sinh() * var_psi_4_db4), ((var_psi_4).sinh() * var_psi_4_db5), ((var_psi_4).sinh() * var_psi_4_db6), ((var_psi_4).sinh() * var_psi_4_db7), ((var_psi_4).sinh() * var_psi_4_db8), ((var_psi_4).sinh() * var_psi_4_db9), ((var_psi_4).sinh() * var_psi_4_db10), ((var_psi_4).sinh() * var_psi_4_db11), ((var_psi_4).sinh() * var_psi_4_db12), ((var_psi_4).sinh() * var_psi_4_db13), ((var_psi_4).sinh() * var_psi_4_db14),)
    } else {
        (var_cosh1, var_cosh1_dn0, var_cosh1_dn1, var_cosh1_dn2, var_cosh1_dn3, var_cosh1_dn4, var_cosh1_dn5, var_cosh1_dn6, var_cosh1_dn7, var_cosh1_dn8, var_cosh1_dn9, var_cosh1_dn10, var_cosh1_dn11, var_cosh1_dn12, var_cosh1_dn13, var_cosh1_dn14, var_cosh1_dn15, var_cosh1_db0, var_cosh1_db1, var_cosh1_db2, var_cosh1_db3, var_cosh1_db4, var_cosh1_db5, var_cosh1_db6, var_cosh1_db7, var_cosh1_db8, var_cosh1_db9, var_cosh1_db10, var_cosh1_db11, var_cosh1_db12, var_cosh1_db13, var_cosh1_db14,)
    }
};
        var_cosh1 = assign1520_e1946;
        var_cosh1_dn0 = assign1520_e1946_d_n0;
        var_cosh1_dn1 = assign1520_e1946_d_n1;
        var_cosh1_dn2 = assign1520_e1946_d_n2;
        var_cosh1_dn3 = assign1520_e1946_d_n3;
        var_cosh1_dn4 = assign1520_e1946_d_n4;
        var_cosh1_dn5 = assign1520_e1946_d_n5;
        var_cosh1_dn6 = assign1520_e1946_d_n6;
        var_cosh1_dn7 = assign1520_e1946_d_n7;
        var_cosh1_dn8 = assign1520_e1946_d_n8;
        var_cosh1_dn9 = assign1520_e1946_d_n9;
        var_cosh1_dn10 = assign1520_e1946_d_n10;
        var_cosh1_dn11 = assign1520_e1946_d_n11;
        var_cosh1_dn12 = assign1520_e1946_d_n12;
        var_cosh1_dn13 = assign1520_e1946_d_n13;
        var_cosh1_dn14 = assign1520_e1946_d_n14;
        var_cosh1_dn15 = assign1520_e1946_d_n15;
        var_cosh1_db0 = assign1520_e1946_d_b0;
        var_cosh1_db1 = assign1520_e1946_d_b1;
        var_cosh1_db2 = assign1520_e1946_d_b2;
        var_cosh1_db3 = assign1520_e1946_d_b3;
        var_cosh1_db4 = assign1520_e1946_d_b4;
        var_cosh1_db5 = assign1520_e1946_d_b5;
        var_cosh1_db6 = assign1520_e1946_d_b6;
        var_cosh1_db7 = assign1520_e1946_d_b7;
        var_cosh1_db8 = assign1520_e1946_d_b8;
        var_cosh1_db9 = assign1520_e1946_d_b9;
        var_cosh1_db10 = assign1520_e1946_d_b10;
        var_cosh1_db11 = assign1520_e1946_d_b11;
        var_cosh1_db12 = assign1520_e1946_d_b12;
        var_cosh1_db13 = assign1520_e1946_d_b13;
        var_cosh1_db14 = assign1520_e1946_d_b14;
        var_cosh1_db15 = 0.0;
        var_cosh1_db16 = 0.0;
        var_cosh1_db17 = 0.0;
        var_cosh1_db18 = 0.0;
        var_cosh1_rv = 0.0;
        var_cosh1_rdn0 = 0.0;
        var_cosh1_rdn1 = 0.0;
        var_cosh1_rdn2 = 0.0;
        var_cosh1_rdn3 = 0.0;
        var_cosh1_rdn4 = 0.0;
        var_cosh1_rdn5 = 0.0;
        var_cosh1_rdn6 = 0.0;
        var_cosh1_rdn7 = 0.0;
        var_cosh1_rdn8 = 0.0;
        var_cosh1_rdn9 = 0.0;
        var_cosh1_rdn10 = 0.0;
        var_cosh1_rdn11 = 0.0;
        var_cosh1_rdn12 = 0.0;
        var_cosh1_rdn13 = 0.0;
        var_cosh1_rdn14 = 0.0;
        var_cosh1_rdn15 = 0.0;
        var_cosh1_rdb0 = 0.0;
        var_cosh1_rdb1 = 0.0;
        var_cosh1_rdb2 = 0.0;
        var_cosh1_rdb3 = 0.0;
        var_cosh1_rdb4 = 0.0;
        var_cosh1_rdb5 = 0.0;
        var_cosh1_rdb6 = 0.0;
        var_cosh1_rdb7 = 0.0;
        var_cosh1_rdb8 = 0.0;
        var_cosh1_rdb9 = 0.0;
        var_cosh1_rdb10 = 0.0;
        var_cosh1_rdb11 = 0.0;
        var_cosh1_rdb12 = 0.0;
        var_cosh1_rdb13 = 0.0;
        var_cosh1_rdb14 = 0.0;
        var_cosh1_rdb15 = 0.0;
        var_cosh1_rdb16 = 0.0;
        var_cosh1_rdb17 = 0.0;
        var_cosh1_rdb18 = 0.0;

        let (assign1530_e1956, assign1530_e1956_d_n0, assign1530_e1956_d_n1, assign1530_e1956_d_n2, assign1530_e1956_d_n3, assign1530_e1956_d_n4, assign1530_e1956_d_n5, assign1530_e1956_d_n6, assign1530_e1956_d_n7, assign1530_e1956_d_n8, assign1530_e1956_d_n9, assign1530_e1956_d_n10, assign1530_e1956_d_n11, assign1530_e1956_d_n12, assign1530_e1956_d_n13, assign1530_e1956_d_n14, assign1530_e1956_d_n15, assign1530_e1956_d_b0, assign1530_e1956_d_b1, assign1530_e1956_d_b2, assign1530_e1956_d_b3, assign1530_e1956_d_b4, assign1530_e1956_d_b5, assign1530_e1956_d_b6, assign1530_e1956_d_b7, assign1530_e1956_d_b8, assign1530_e1956_d_b9, assign1530_e1956_d_b10, assign1530_e1956_d_b11, assign1530_e1956_d_b12, assign1530_e1956_d_b13, assign1530_e1956_d_b14,) = {
    if ((var_guard15 != 0.0) && (!((var_guard13 != 0.0) || (var_guard14 != 0.0)))) {
        let assign1530_e1954: f64 = (var_cosh1).ln();
        (assign1530_e1954, (var_cosh1_dn0 / var_cosh1), (var_cosh1_dn1 / var_cosh1), (var_cosh1_dn2 / var_cosh1), (var_cosh1_dn3 / var_cosh1), (var_cosh1_dn4 / var_cosh1), (var_cosh1_dn5 / var_cosh1), (var_cosh1_dn6 / var_cosh1), (var_cosh1_dn7 / var_cosh1), (var_cosh1_dn8 / var_cosh1), (var_cosh1_dn9 / var_cosh1), (var_cosh1_dn10 / var_cosh1), (var_cosh1_dn11 / var_cosh1), (var_cosh1_dn12 / var_cosh1), (var_cosh1_dn13 / var_cosh1), (var_cosh1_dn14 / var_cosh1), (var_cosh1_dn15 / var_cosh1), (var_cosh1_db0 / var_cosh1), (var_cosh1_db1 / var_cosh1), (var_cosh1_db2 / var_cosh1), (var_cosh1_db3 / var_cosh1), (var_cosh1_db4 / var_cosh1), (var_cosh1_db5 / var_cosh1), (var_cosh1_db6 / var_cosh1), (var_cosh1_db7 / var_cosh1), (var_cosh1_db8 / var_cosh1), (var_cosh1_db9 / var_cosh1), (var_cosh1_db10 / var_cosh1), (var_cosh1_db11 / var_cosh1), (var_cosh1_db12 / var_cosh1), (var_cosh1_db13 / var_cosh1), (var_cosh1_db14 / var_cosh1),)
    } else {
        (var_lc4, var_lc4_dn0, var_lc4_dn1, var_lc4_dn2, var_lc4_dn3, var_lc4_dn4, var_lc4_dn5, var_lc4_dn6, var_lc4_dn7, var_lc4_dn8, var_lc4_dn9, var_lc4_dn10, var_lc4_dn11, var_lc4_dn12, var_lc4_dn13, var_lc4_dn14, var_lc4_dn15, var_lc4_db0, var_lc4_db1, var_lc4_db2, var_lc4_db3, var_lc4_db4, var_lc4_db5, var_lc4_db6, var_lc4_db7, var_lc4_db8, var_lc4_db9, var_lc4_db10, var_lc4_db11, var_lc4_db12, var_lc4_db13, var_lc4_db14,)
    }
};
        var_lc4 = assign1530_e1956;
        var_lc4_dn0 = assign1530_e1956_d_n0;
        var_lc4_dn1 = assign1530_e1956_d_n1;
        var_lc4_dn2 = assign1530_e1956_d_n2;
        var_lc4_dn3 = assign1530_e1956_d_n3;
        var_lc4_dn4 = assign1530_e1956_d_n4;
        var_lc4_dn5 = assign1530_e1956_d_n5;
        var_lc4_dn6 = assign1530_e1956_d_n6;
        var_lc4_dn7 = assign1530_e1956_d_n7;
        var_lc4_dn8 = assign1530_e1956_d_n8;
        var_lc4_dn9 = assign1530_e1956_d_n9;
        var_lc4_dn10 = assign1530_e1956_d_n10;
        var_lc4_dn11 = assign1530_e1956_d_n11;
        var_lc4_dn12 = assign1530_e1956_d_n12;
        var_lc4_dn13 = assign1530_e1956_d_n13;
        var_lc4_dn14 = assign1530_e1956_d_n14;
        var_lc4_dn15 = assign1530_e1956_d_n15;
        var_lc4_db0 = assign1530_e1956_d_b0;
        var_lc4_db1 = assign1530_e1956_d_b1;
        var_lc4_db2 = assign1530_e1956_d_b2;
        var_lc4_db3 = assign1530_e1956_d_b3;
        var_lc4_db4 = assign1530_e1956_d_b4;
        var_lc4_db5 = assign1530_e1956_d_b5;
        var_lc4_db6 = assign1530_e1956_d_b6;
        var_lc4_db7 = assign1530_e1956_d_b7;
        var_lc4_db8 = assign1530_e1956_d_b8;
        var_lc4_db9 = assign1530_e1956_d_b9;
        var_lc4_db10 = assign1530_e1956_d_b10;
        var_lc4_db11 = assign1530_e1956_d_b11;
        var_lc4_db12 = assign1530_e1956_d_b12;
        var_lc4_db13 = assign1530_e1956_d_b13;
        var_lc4_db14 = assign1530_e1956_d_b14;
        var_lc4_db15 = 0.0;
        var_lc4_db16 = 0.0;
        var_lc4_db17 = 0.0;
        var_lc4_db18 = 0.0;
        var_lc4_rv = 0.0;
        var_lc4_rdn0 = 0.0;
        var_lc4_rdn1 = 0.0;
        var_lc4_rdn2 = 0.0;
        var_lc4_rdn3 = 0.0;
        var_lc4_rdn4 = 0.0;
        var_lc4_rdn5 = 0.0;
        var_lc4_rdn6 = 0.0;
        var_lc4_rdn7 = 0.0;
        var_lc4_rdn8 = 0.0;
        var_lc4_rdn9 = 0.0;
        var_lc4_rdn10 = 0.0;
        var_lc4_rdn11 = 0.0;
        var_lc4_rdn12 = 0.0;
        var_lc4_rdn13 = 0.0;
        var_lc4_rdn14 = 0.0;
        var_lc4_rdn15 = 0.0;
        var_lc4_rdb0 = 0.0;
        var_lc4_rdb1 = 0.0;
        var_lc4_rdb2 = 0.0;
        var_lc4_rdb3 = 0.0;
        var_lc4_rdb4 = 0.0;
        var_lc4_rdb5 = 0.0;
        var_lc4_rdb6 = 0.0;
        var_lc4_rdb7 = 0.0;
        var_lc4_rdb8 = 0.0;
        var_lc4_rdb9 = 0.0;
        var_lc4_rdb10 = 0.0;
        var_lc4_rdb11 = 0.0;
        var_lc4_rdb12 = 0.0;
        var_lc4_rdb13 = 0.0;
        var_lc4_rdb14 = 0.0;
        var_lc4_rdb15 = 0.0;
        var_lc4_rdb16 = 0.0;
        var_lc4_rdb17 = 0.0;
        var_lc4_rdb18 = 0.0;

        let (assign1540_e1971, assign1540_e1971_d_n0, assign1540_e1971_d_n1, assign1540_e1971_d_n2, assign1540_e1971_d_n3, assign1540_e1971_d_n4, assign1540_e1971_d_n5, assign1540_e1971_d_n6, assign1540_e1971_d_n7, assign1540_e1971_d_n8, assign1540_e1971_d_n9, assign1540_e1971_d_n10, assign1540_e1971_d_n11, assign1540_e1971_d_n12, assign1540_e1971_d_n13, assign1540_e1971_d_n14, assign1540_e1971_d_n15, assign1540_e1971_d_b0, assign1540_e1971_d_b1, assign1540_e1971_d_b2, assign1540_e1971_d_b3, assign1540_e1971_d_b4, assign1540_e1971_d_b5, assign1540_e1971_d_b6, assign1540_e1971_d_b7, assign1540_e1971_d_b8, assign1540_e1971_d_b9, assign1540_e1971_d_b10, assign1540_e1971_d_b11, assign1540_e1971_d_b12, assign1540_e1971_d_b13, assign1540_e1971_d_b14,) = {
    if ((var_guard15 != 0.0) && (!((var_guard13 != 0.0) || (var_guard14 != 0.0)))) {
        let assign1540_e1966: f64 = (p.p37 * var_vds);
        let assign1540_e1967: f64 = (var_p40_t - assign1540_e1966);
        let assign1540_e1969: f64 = (assign1540_e1967 + var_lc40);
        (assign1540_e1969, ((var_p40_t_dn0 - (p.p37 * var_vds_dn0)) + var_lc40_dn0), ((var_p40_t_dn1 - (p.p37 * var_vds_dn1)) + var_lc40_dn1), ((var_p40_t_dn2 - (p.p37 * var_vds_dn2)) + var_lc40_dn2), ((var_p40_t_dn3 - (p.p37 * var_vds_dn3)) + var_lc40_dn3), ((var_p40_t_dn4 - (p.p37 * var_vds_dn4)) + var_lc40_dn4), ((var_p40_t_dn5 - (p.p37 * var_vds_dn5)) + var_lc40_dn5), ((var_p40_t_dn6 - (p.p37 * var_vds_dn6)) + var_lc40_dn6), ((var_p40_t_dn7 - (p.p37 * var_vds_dn7)) + var_lc40_dn7), ((var_p40_t_dn8 - (p.p37 * var_vds_dn8)) + var_lc40_dn8), ((var_p40_t_dn9 - (p.p37 * var_vds_dn9)) + var_lc40_dn9), ((var_p40_t_dn10 - (p.p37 * var_vds_dn10)) + var_lc40_dn10), ((var_p40_t_dn11 - (p.p37 * var_vds_dn11)) + var_lc40_dn11), ((var_p40_t_dn12 - (p.p37 * var_vds_dn12)) + var_lc40_dn12), ((var_p40_t_dn13 - (p.p37 * var_vds_dn13)) + var_lc40_dn13), ((var_p40_t_dn14 - (p.p37 * var_vds_dn14)) + var_lc40_dn14), ((var_p40_t_dn15 - (p.p37 * var_vds_dn15)) + var_lc40_dn15), ((var_p40_t_db0 - (p.p37 * var_vds_db0)) + var_lc40_db0), ((var_p40_t_db1 - (p.p37 * var_vds_db1)) + var_lc40_db1), ((var_p40_t_db2 - (p.p37 * var_vds_db2)) + var_lc40_db2), ((var_p40_t_db3 - (p.p37 * var_vds_db3)) + var_lc40_db3), ((var_p40_t_db4 - (p.p37 * var_vds_db4)) + var_lc40_db4), ((var_p40_t_db5 - (p.p37 * var_vds_db5)) + var_lc40_db5), ((var_p40_t_db6 - (p.p37 * var_vds_db6)) + var_lc40_db6), ((var_p40_t_db7 - (p.p37 * var_vds_db7)) + var_lc40_db7), ((var_p40_t_db8 - (p.p37 * var_vds_db8)) + var_lc40_db8), ((var_p40_t_db9 - (p.p37 * var_vds_db9)) + var_lc40_db9), ((var_p40_t_db10 - (p.p37 * var_vds_db10)) + var_lc40_db10), ((var_p40_t_db11 - (p.p37 * var_vds_db11)) + var_lc40_db11), ((var_p40_t_db12 - (p.p37 * var_vds_db12)) + var_lc40_db12), ((var_p40_t_db13 - (p.p37 * var_vds_db13)) + var_lc40_db13), ((var_p40_t_db14 - (p.p37 * var_vds_db14)) + var_lc40_db14),)
    } else {
        (var_qgd0, var_qgd0_dn0, var_qgd0_dn1, var_qgd0_dn2, var_qgd0_dn3, var_qgd0_dn4, var_qgd0_dn5, var_qgd0_dn6, var_qgd0_dn7, var_qgd0_dn8, var_qgd0_dn9, var_qgd0_dn10, var_qgd0_dn11, var_qgd0_dn12, var_qgd0_dn13, var_qgd0_dn14, var_qgd0_dn15, var_qgd0_db0, var_qgd0_db1, var_qgd0_db2, var_qgd0_db3, var_qgd0_db4, var_qgd0_db5, var_qgd0_db6, var_qgd0_db7, var_qgd0_db8, var_qgd0_db9, var_qgd0_db10, var_qgd0_db11, var_qgd0_db12, var_qgd0_db13, var_qgd0_db14,)
    }
};
        var_qgd0 = assign1540_e1971;
        var_qgd0_dn0 = assign1540_e1971_d_n0;
        var_qgd0_dn1 = assign1540_e1971_d_n1;
        var_qgd0_dn2 = assign1540_e1971_d_n2;
        var_qgd0_dn3 = assign1540_e1971_d_n3;
        var_qgd0_dn4 = assign1540_e1971_d_n4;
        var_qgd0_dn5 = assign1540_e1971_d_n5;
        var_qgd0_dn6 = assign1540_e1971_d_n6;
        var_qgd0_dn7 = assign1540_e1971_d_n7;
        var_qgd0_dn8 = assign1540_e1971_d_n8;
        var_qgd0_dn9 = assign1540_e1971_d_n9;
        var_qgd0_dn10 = assign1540_e1971_d_n10;
        var_qgd0_dn11 = assign1540_e1971_d_n11;
        var_qgd0_dn12 = assign1540_e1971_d_n12;
        var_qgd0_dn13 = assign1540_e1971_d_n13;
        var_qgd0_dn14 = assign1540_e1971_d_n14;
        var_qgd0_dn15 = assign1540_e1971_d_n15;
        var_qgd0_db0 = assign1540_e1971_d_b0;
        var_qgd0_db1 = assign1540_e1971_d_b1;
        var_qgd0_db2 = assign1540_e1971_d_b2;
        var_qgd0_db3 = assign1540_e1971_d_b3;
        var_qgd0_db4 = assign1540_e1971_d_b4;
        var_qgd0_db5 = assign1540_e1971_d_b5;
        var_qgd0_db6 = assign1540_e1971_d_b6;
        var_qgd0_db7 = assign1540_e1971_d_b7;
        var_qgd0_db8 = assign1540_e1971_d_b8;
        var_qgd0_db9 = assign1540_e1971_d_b9;
        var_qgd0_db10 = assign1540_e1971_d_b10;
        var_qgd0_db11 = assign1540_e1971_d_b11;
        var_qgd0_db12 = assign1540_e1971_d_b12;
        var_qgd0_db13 = assign1540_e1971_d_b13;
        var_qgd0_db14 = assign1540_e1971_d_b14;
        var_qgd0_db15 = 0.0;
        var_qgd0_db16 = 0.0;
        var_qgd0_db17 = 0.0;
        var_qgd0_db18 = 0.0;
        var_qgd0_rv = 0.0;
        var_qgd0_rdn0 = 0.0;
        var_qgd0_rdn1 = 0.0;
        var_qgd0_rdn2 = 0.0;
        var_qgd0_rdn3 = 0.0;
        var_qgd0_rdn4 = 0.0;
        var_qgd0_rdn5 = 0.0;
        var_qgd0_rdn6 = 0.0;
        var_qgd0_rdn7 = 0.0;
        var_qgd0_rdn8 = 0.0;
        var_qgd0_rdn9 = 0.0;
        var_qgd0_rdn10 = 0.0;
        var_qgd0_rdn11 = 0.0;
        var_qgd0_rdn12 = 0.0;
        var_qgd0_rdn13 = 0.0;
        var_qgd0_rdn14 = 0.0;
        var_qgd0_rdn15 = 0.0;
        var_qgd0_rdb0 = 0.0;
        var_qgd0_rdb1 = 0.0;
        var_qgd0_rdb2 = 0.0;
        var_qgd0_rdb3 = 0.0;
        var_qgd0_rdb4 = 0.0;
        var_qgd0_rdb5 = 0.0;
        var_qgd0_rdb6 = 0.0;
        var_qgd0_rdb7 = 0.0;
        var_qgd0_rdb8 = 0.0;
        var_qgd0_rdb9 = 0.0;
        var_qgd0_rdb10 = 0.0;
        var_qgd0_rdb11 = 0.0;
        var_qgd0_rdb12 = 0.0;
        var_qgd0_rdb13 = 0.0;
        var_qgd0_rdb14 = 0.0;
        var_qgd0_rdb15 = 0.0;
        var_qgd0_rdb16 = 0.0;
        var_qgd0_rdb17 = 0.0;
        var_qgd0_rdb18 = 0.0;

        let (assign1550_e2000, assign1550_e2000_d_n0, assign1550_e2000_d_n1, assign1550_e2000_d_n2, assign1550_e2000_d_n3, assign1550_e2000_d_n4, assign1550_e2000_d_n5, assign1550_e2000_d_n6, assign1550_e2000_d_n7, assign1550_e2000_d_n8, assign1550_e2000_d_n9, assign1550_e2000_d_n10, assign1550_e2000_d_n11, assign1550_e2000_d_n12, assign1550_e2000_d_n13, assign1550_e2000_d_n14, assign1550_e2000_d_n15, assign1550_e2000_d_b0, assign1550_e2000_d_b1, assign1550_e2000_d_b2, assign1550_e2000_d_b3, assign1550_e2000_d_b4, assign1550_e2000_d_b5, assign1550_e2000_d_b6, assign1550_e2000_d_b7, assign1550_e2000_d_b8, assign1550_e2000_d_b9, assign1550_e2000_d_b10, assign1550_e2000_d_b11, assign1550_e2000_d_b12, assign1550_e2000_d_b13, assign1550_e2000_d_b14,) = {
    if ((var_guard15 != 0.0) && (!((var_guard13 != 0.0) || (var_guard14 != 0.0)))) {
        let assign1550_e1981: f64 = (var_psi_4 + var_lc4);
        let assign1550_e1983: f64 = (assign1550_e1981 - var_qgd0);
        let assign1550_e1985: f64 = (assign1550_e1983 * var_tanh3);
        let assign1550_e1987: f64 = (assign1550_e1985 / p.p36);
        let assign1550_e1990: f64 = (2.0 * p.p37);
        let assign1550_e1992: f64 = (assign1550_e1990 * var_vgdc);
        let assign1550_e1993: f64 = (assign1550_e1987 + assign1550_e1992);
        let assign1550_e1994: f64 = (var_cgd0_t * assign1550_e1993);
        let assign1550_e1997: f64 = (p.p26 * var_vgdc);
        let assign1550_e1998: f64 = (assign1550_e1994 + assign1550_e1997);
        (assign1550_e1998, (((var_cgd0_t_dn0 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_dn0 + var_lc4_dn0) - var_qgd0_dn0) * var_tanh3) + (assign1550_e1983 * var_tanh3_dn0)) / p.p36) + (assign1550_e1990 * var_vgdc_dn0)))) + (p.p26 * var_vgdc_dn0)), (((var_cgd0_t_dn1 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_dn1 + var_lc4_dn1) - var_qgd0_dn1) * var_tanh3) + (assign1550_e1983 * var_tanh3_dn1)) / p.p36) + (assign1550_e1990 * var_vgdc_dn1)))) + (p.p26 * var_vgdc_dn1)), (((var_cgd0_t_dn2 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_dn2 + var_lc4_dn2) - var_qgd0_dn2) * var_tanh3) + (assign1550_e1983 * var_tanh3_dn2)) / p.p36) + (assign1550_e1990 * var_vgdc_dn2)))) + (p.p26 * var_vgdc_dn2)), (((var_cgd0_t_dn3 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_dn3 + var_lc4_dn3) - var_qgd0_dn3) * var_tanh3) + (assign1550_e1983 * var_tanh3_dn3)) / p.p36) + (assign1550_e1990 * var_vgdc_dn3)))) + (p.p26 * var_vgdc_dn3)), (((var_cgd0_t_dn4 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_dn4 + var_lc4_dn4) - var_qgd0_dn4) * var_tanh3) + (assign1550_e1983 * var_tanh3_dn4)) / p.p36) + (assign1550_e1990 * var_vgdc_dn4)))) + (p.p26 * var_vgdc_dn4)), (((var_cgd0_t_dn5 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_dn5 + var_lc4_dn5) - var_qgd0_dn5) * var_tanh3) + (assign1550_e1983 * var_tanh3_dn5)) / p.p36) + (assign1550_e1990 * var_vgdc_dn5)))) + (p.p26 * var_vgdc_dn5)), (((var_cgd0_t_dn6 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_dn6 + var_lc4_dn6) - var_qgd0_dn6) * var_tanh3) + (assign1550_e1983 * var_tanh3_dn6)) / p.p36) + (assign1550_e1990 * var_vgdc_dn6)))) + (p.p26 * var_vgdc_dn6)), (((var_cgd0_t_dn7 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_dn7 + var_lc4_dn7) - var_qgd0_dn7) * var_tanh3) + (assign1550_e1983 * var_tanh3_dn7)) / p.p36) + (assign1550_e1990 * var_vgdc_dn7)))) + (p.p26 * var_vgdc_dn7)), (((var_cgd0_t_dn8 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_dn8 + var_lc4_dn8) - var_qgd0_dn8) * var_tanh3) + (assign1550_e1983 * var_tanh3_dn8)) / p.p36) + (assign1550_e1990 * var_vgdc_dn8)))) + (p.p26 * var_vgdc_dn8)), (((var_cgd0_t_dn9 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_dn9 + var_lc4_dn9) - var_qgd0_dn9) * var_tanh3) + (assign1550_e1983 * var_tanh3_dn9)) / p.p36) + (assign1550_e1990 * var_vgdc_dn9)))) + (p.p26 * var_vgdc_dn9)), (((var_cgd0_t_dn10 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_dn10 + var_lc4_dn10) - var_qgd0_dn10) * var_tanh3) + (assign1550_e1983 * var_tanh3_dn10)) / p.p36) + (assign1550_e1990 * var_vgdc_dn10)))) + (p.p26 * var_vgdc_dn10)), (((var_cgd0_t_dn11 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_dn11 + var_lc4_dn11) - var_qgd0_dn11) * var_tanh3) + (assign1550_e1983 * var_tanh3_dn11)) / p.p36) + (assign1550_e1990 * var_vgdc_dn11)))) + (p.p26 * var_vgdc_dn11)), (((var_cgd0_t_dn12 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_dn12 + var_lc4_dn12) - var_qgd0_dn12) * var_tanh3) + (assign1550_e1983 * var_tanh3_dn12)) / p.p36) + (assign1550_e1990 * var_vgdc_dn12)))) + (p.p26 * var_vgdc_dn12)), (((var_cgd0_t_dn13 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_dn13 + var_lc4_dn13) - var_qgd0_dn13) * var_tanh3) + (assign1550_e1983 * var_tanh3_dn13)) / p.p36) + (assign1550_e1990 * var_vgdc_dn13)))) + (p.p26 * var_vgdc_dn13)), (((var_cgd0_t_dn14 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_dn14 + var_lc4_dn14) - var_qgd0_dn14) * var_tanh3) + (assign1550_e1983 * var_tanh3_dn14)) / p.p36) + (assign1550_e1990 * var_vgdc_dn14)))) + (p.p26 * var_vgdc_dn14)), (((var_cgd0_t_dn15 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_dn15 + var_lc4_dn15) - var_qgd0_dn15) * var_tanh3) + (assign1550_e1983 * var_tanh3_dn15)) / p.p36) + (assign1550_e1990 * var_vgdc_dn15)))) + (p.p26 * var_vgdc_dn15)), (((var_cgd0_t_db0 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_db0 + var_lc4_db0) - var_qgd0_db0) * var_tanh3) + (assign1550_e1983 * var_tanh3_db0)) / p.p36) + (assign1550_e1990 * var_vgdc_db0)))) + (p.p26 * var_vgdc_db0)), (((var_cgd0_t_db1 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_db1 + var_lc4_db1) - var_qgd0_db1) * var_tanh3) + (assign1550_e1983 * var_tanh3_db1)) / p.p36) + (assign1550_e1990 * var_vgdc_db1)))) + (p.p26 * var_vgdc_db1)), (((var_cgd0_t_db2 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_db2 + var_lc4_db2) - var_qgd0_db2) * var_tanh3) + (assign1550_e1983 * var_tanh3_db2)) / p.p36) + (assign1550_e1990 * var_vgdc_db2)))) + (p.p26 * var_vgdc_db2)), (((var_cgd0_t_db3 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_db3 + var_lc4_db3) - var_qgd0_db3) * var_tanh3) + (assign1550_e1983 * var_tanh3_db3)) / p.p36) + (assign1550_e1990 * var_vgdc_db3)))) + (p.p26 * var_vgdc_db3)), (((var_cgd0_t_db4 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_db4 + var_lc4_db4) - var_qgd0_db4) * var_tanh3) + (assign1550_e1983 * var_tanh3_db4)) / p.p36) + (assign1550_e1990 * var_vgdc_db4)))) + (p.p26 * var_vgdc_db4)), (((var_cgd0_t_db5 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_db5 + var_lc4_db5) - var_qgd0_db5) * var_tanh3) + (assign1550_e1983 * var_tanh3_db5)) / p.p36) + (assign1550_e1990 * var_vgdc_db5)))) + (p.p26 * var_vgdc_db5)), (((var_cgd0_t_db6 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_db6 + var_lc4_db6) - var_qgd0_db6) * var_tanh3) + (assign1550_e1983 * var_tanh3_db6)) / p.p36) + (assign1550_e1990 * var_vgdc_db6)))) + (p.p26 * var_vgdc_db6)), (((var_cgd0_t_db7 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_db7 + var_lc4_db7) - var_qgd0_db7) * var_tanh3) + (assign1550_e1983 * var_tanh3_db7)) / p.p36) + (assign1550_e1990 * var_vgdc_db7)))) + (p.p26 * var_vgdc_db7)), (((var_cgd0_t_db8 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_db8 + var_lc4_db8) - var_qgd0_db8) * var_tanh3) + (assign1550_e1983 * var_tanh3_db8)) / p.p36) + (assign1550_e1990 * var_vgdc_db8)))) + (p.p26 * var_vgdc_db8)), (((var_cgd0_t_db9 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_db9 + var_lc4_db9) - var_qgd0_db9) * var_tanh3) + (assign1550_e1983 * var_tanh3_db9)) / p.p36) + (assign1550_e1990 * var_vgdc_db9)))) + (p.p26 * var_vgdc_db9)), (((var_cgd0_t_db10 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_db10 + var_lc4_db10) - var_qgd0_db10) * var_tanh3) + (assign1550_e1983 * var_tanh3_db10)) / p.p36) + (assign1550_e1990 * var_vgdc_db10)))) + (p.p26 * var_vgdc_db10)), (((var_cgd0_t_db11 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_db11 + var_lc4_db11) - var_qgd0_db11) * var_tanh3) + (assign1550_e1983 * var_tanh3_db11)) / p.p36) + (assign1550_e1990 * var_vgdc_db11)))) + (p.p26 * var_vgdc_db11)), (((var_cgd0_t_db12 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_db12 + var_lc4_db12) - var_qgd0_db12) * var_tanh3) + (assign1550_e1983 * var_tanh3_db12)) / p.p36) + (assign1550_e1990 * var_vgdc_db12)))) + (p.p26 * var_vgdc_db12)), (((var_cgd0_t_db13 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_db13 + var_lc4_db13) - var_qgd0_db13) * var_tanh3) + (assign1550_e1983 * var_tanh3_db13)) / p.p36) + (assign1550_e1990 * var_vgdc_db13)))) + (p.p26 * var_vgdc_db13)), (((var_cgd0_t_db14 * assign1550_e1993) + (var_cgd0_t * ((((((var_psi_4_db14 + var_lc4_db14) - var_qgd0_db14) * var_tanh3) + (assign1550_e1983 * var_tanh3_db14)) / p.p36) + (assign1550_e1990 * var_vgdc_db14)))) + (p.p26 * var_vgdc_db14)),)
    } else {
        (var_qgd, var_qgd_dn0, var_qgd_dn1, var_qgd_dn2, var_qgd_dn3, var_qgd_dn4, var_qgd_dn5, var_qgd_dn6, var_qgd_dn7, var_qgd_dn8, var_qgd_dn9, var_qgd_dn10, var_qgd_dn11, var_qgd_dn12, var_qgd_dn13, var_qgd_dn14, var_qgd_dn15, var_qgd_db0, var_qgd_db1, var_qgd_db2, var_qgd_db3, var_qgd_db4, var_qgd_db5, var_qgd_db6, var_qgd_db7, var_qgd_db8, var_qgd_db9, var_qgd_db10, var_qgd_db11, var_qgd_db12, var_qgd_db13, var_qgd_db14,)
    }
};
        var_qgd = assign1550_e2000;
        var_qgd_dn0 = assign1550_e2000_d_n0;
        var_qgd_dn1 = assign1550_e2000_d_n1;
        var_qgd_dn2 = assign1550_e2000_d_n2;
        var_qgd_dn3 = assign1550_e2000_d_n3;
        var_qgd_dn4 = assign1550_e2000_d_n4;
        var_qgd_dn5 = assign1550_e2000_d_n5;
        var_qgd_dn6 = assign1550_e2000_d_n6;
        var_qgd_dn7 = assign1550_e2000_d_n7;
        var_qgd_dn8 = assign1550_e2000_d_n8;
        var_qgd_dn9 = assign1550_e2000_d_n9;
        var_qgd_dn10 = assign1550_e2000_d_n10;
        var_qgd_dn11 = assign1550_e2000_d_n11;
        var_qgd_dn12 = assign1550_e2000_d_n12;
        var_qgd_dn13 = assign1550_e2000_d_n13;
        var_qgd_dn14 = assign1550_e2000_d_n14;
        var_qgd_dn15 = assign1550_e2000_d_n15;
        var_qgd_db0 = assign1550_e2000_d_b0;
        var_qgd_db1 = assign1550_e2000_d_b1;
        var_qgd_db2 = assign1550_e2000_d_b2;
        var_qgd_db3 = assign1550_e2000_d_b3;
        var_qgd_db4 = assign1550_e2000_d_b4;
        var_qgd_db5 = assign1550_e2000_d_b5;
        var_qgd_db6 = assign1550_e2000_d_b6;
        var_qgd_db7 = assign1550_e2000_d_b7;
        var_qgd_db8 = assign1550_e2000_d_b8;
        var_qgd_db9 = assign1550_e2000_d_b9;
        var_qgd_db10 = assign1550_e2000_d_b10;
        var_qgd_db11 = assign1550_e2000_d_b11;
        var_qgd_db12 = assign1550_e2000_d_b12;
        var_qgd_db13 = assign1550_e2000_d_b13;
        var_qgd_db14 = assign1550_e2000_d_b14;
        var_qgd_db15 = 0.0;
        var_qgd_db16 = 0.0;
        var_qgd_db17 = 0.0;
        var_qgd_db18 = 0.0;
        var_qgd_rv = 0.0;
        var_qgd_rdn0 = 0.0;
        var_qgd_rdn1 = 0.0;
        var_qgd_rdn2 = 0.0;
        var_qgd_rdn3 = 0.0;
        var_qgd_rdn4 = 0.0;
        var_qgd_rdn5 = 0.0;
        var_qgd_rdn6 = 0.0;
        var_qgd_rdn7 = 0.0;
        var_qgd_rdn8 = 0.0;
        var_qgd_rdn9 = 0.0;
        var_qgd_rdn10 = 0.0;
        var_qgd_rdn11 = 0.0;
        var_qgd_rdn12 = 0.0;
        var_qgd_rdn13 = 0.0;
        var_qgd_rdn14 = 0.0;
        var_qgd_rdn15 = 0.0;
        var_qgd_rdb0 = 0.0;
        var_qgd_rdb1 = 0.0;
        var_qgd_rdb2 = 0.0;
        var_qgd_rdb3 = 0.0;
        var_qgd_rdb4 = 0.0;
        var_qgd_rdb5 = 0.0;
        var_qgd_rdb6 = 0.0;
        var_qgd_rdb7 = 0.0;
        var_qgd_rdb8 = 0.0;
        var_qgd_rdb9 = 0.0;
        var_qgd_rdb10 = 0.0;
        var_qgd_rdb11 = 0.0;
        var_qgd_rdb12 = 0.0;
        var_qgd_rdb13 = 0.0;
        var_qgd_rdb14 = 0.0;
        var_qgd_rdb15 = 0.0;
        var_qgd_rdb16 = 0.0;
        var_qgd_rdb17 = 0.0;
        var_qgd_rdb18 = 0.0;

        let (assign1560_e2011, assign1560_e2011_d_n0, assign1560_e2011_d_n1, assign1560_e2011_d_n2, assign1560_e2011_d_n3, assign1560_e2011_d_n4, assign1560_e2011_d_n5, assign1560_e2011_d_n6, assign1560_e2011_d_n7, assign1560_e2011_d_n8, assign1560_e2011_d_n9, assign1560_e2011_d_n10, assign1560_e2011_d_n11, assign1560_e2011_d_n12, assign1560_e2011_d_n13, assign1560_e2011_d_n14, assign1560_e2011_d_n15, assign1560_e2011_d_b0, assign1560_e2011_d_b1, assign1560_e2011_d_b2, assign1560_e2011_d_b3, assign1560_e2011_d_b4, assign1560_e2011_d_b5, assign1560_e2011_d_b6, assign1560_e2011_d_b7, assign1560_e2011_d_b8, assign1560_e2011_d_b9, assign1560_e2011_d_b10, assign1560_e2011_d_b11, assign1560_e2011_d_b12, assign1560_e2011_d_b13, assign1560_e2011_d_b14,) = {
    if ((var_guard15 != 0.0) && (!((var_guard13 != 0.0) || (var_guard14 != 0.0)))) {
        let assign1560_e2009: f64 = var_qgs_dn8;
        (assign1560_e2009, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgs, var_cgs_dn0, var_cgs_dn1, var_cgs_dn2, var_cgs_dn3, var_cgs_dn4, var_cgs_dn5, var_cgs_dn6, var_cgs_dn7, var_cgs_dn8, var_cgs_dn9, var_cgs_dn10, var_cgs_dn11, var_cgs_dn12, var_cgs_dn13, var_cgs_dn14, var_cgs_dn15, var_cgs_db0, var_cgs_db1, var_cgs_db2, var_cgs_db3, var_cgs_db4, var_cgs_db5, var_cgs_db6, var_cgs_db7, var_cgs_db8, var_cgs_db9, var_cgs_db10, var_cgs_db11, var_cgs_db12, var_cgs_db13, var_cgs_db14,)
    }
};
        var_cgs = assign1560_e2011;
        var_cgs_dn0 = assign1560_e2011_d_n0;
        var_cgs_dn1 = assign1560_e2011_d_n1;
        var_cgs_dn2 = assign1560_e2011_d_n2;
        var_cgs_dn3 = assign1560_e2011_d_n3;
        var_cgs_dn4 = assign1560_e2011_d_n4;
        var_cgs_dn5 = assign1560_e2011_d_n5;
        var_cgs_dn6 = assign1560_e2011_d_n6;
        var_cgs_dn7 = assign1560_e2011_d_n7;
        var_cgs_dn8 = assign1560_e2011_d_n8;
        var_cgs_dn9 = assign1560_e2011_d_n9;
        var_cgs_dn10 = assign1560_e2011_d_n10;
        var_cgs_dn11 = assign1560_e2011_d_n11;
        var_cgs_dn12 = assign1560_e2011_d_n12;
        var_cgs_dn13 = assign1560_e2011_d_n13;
        var_cgs_dn14 = assign1560_e2011_d_n14;
        var_cgs_dn15 = assign1560_e2011_d_n15;
        var_cgs_db0 = assign1560_e2011_d_b0;
        var_cgs_db1 = assign1560_e2011_d_b1;
        var_cgs_db2 = assign1560_e2011_d_b2;
        var_cgs_db3 = assign1560_e2011_d_b3;
        var_cgs_db4 = assign1560_e2011_d_b4;
        var_cgs_db5 = assign1560_e2011_d_b5;
        var_cgs_db6 = assign1560_e2011_d_b6;
        var_cgs_db7 = assign1560_e2011_d_b7;
        var_cgs_db8 = assign1560_e2011_d_b8;
        var_cgs_db9 = assign1560_e2011_d_b9;
        var_cgs_db10 = assign1560_e2011_d_b10;
        var_cgs_db11 = assign1560_e2011_d_b11;
        var_cgs_db12 = assign1560_e2011_d_b12;
        var_cgs_db13 = assign1560_e2011_d_b13;
        var_cgs_db14 = assign1560_e2011_d_b14;
        var_cgs_db15 = 0.0;
        var_cgs_db16 = 0.0;
        var_cgs_db17 = 0.0;
        var_cgs_db18 = 0.0;
        var_cgs_rv = 0.0;
        var_cgs_rdn0 = 0.0;
        var_cgs_rdn1 = 0.0;
        var_cgs_rdn2 = 0.0;
        var_cgs_rdn3 = 0.0;
        var_cgs_rdn4 = 0.0;
        var_cgs_rdn5 = 0.0;
        var_cgs_rdn6 = 0.0;
        var_cgs_rdn7 = 0.0;
        var_cgs_rdn8 = 0.0;
        var_cgs_rdn9 = 0.0;
        var_cgs_rdn10 = 0.0;
        var_cgs_rdn11 = 0.0;
        var_cgs_rdn12 = 0.0;
        var_cgs_rdn13 = 0.0;
        var_cgs_rdn14 = 0.0;
        var_cgs_rdn15 = 0.0;
        var_cgs_rdb0 = 0.0;
        var_cgs_rdb1 = 0.0;
        var_cgs_rdb2 = 0.0;
        var_cgs_rdb3 = 0.0;
        var_cgs_rdb4 = 0.0;
        var_cgs_rdb5 = 0.0;
        var_cgs_rdb6 = 0.0;
        var_cgs_rdb7 = 0.0;
        var_cgs_rdb8 = 0.0;
        var_cgs_rdb9 = 0.0;
        var_cgs_rdb10 = 0.0;
        var_cgs_rdb11 = 0.0;
        var_cgs_rdb12 = 0.0;
        var_cgs_rdb13 = 0.0;
        var_cgs_rdb14 = 0.0;
        var_cgs_rdb15 = 0.0;
        var_cgs_rdb16 = 0.0;
        var_cgs_rdb17 = 0.0;
        var_cgs_rdb18 = 0.0;

        let (assign1570_e2022, assign1570_e2022_d_n0, assign1570_e2022_d_n1, assign1570_e2022_d_n2, assign1570_e2022_d_n3, assign1570_e2022_d_n4, assign1570_e2022_d_n5, assign1570_e2022_d_n6, assign1570_e2022_d_n7, assign1570_e2022_d_n8, assign1570_e2022_d_n9, assign1570_e2022_d_n10, assign1570_e2022_d_n11, assign1570_e2022_d_n12, assign1570_e2022_d_n13, assign1570_e2022_d_n14, assign1570_e2022_d_n15, assign1570_e2022_d_b0, assign1570_e2022_d_b1, assign1570_e2022_d_b2, assign1570_e2022_d_b3, assign1570_e2022_d_b4, assign1570_e2022_d_b5, assign1570_e2022_d_b6, assign1570_e2022_d_b7, assign1570_e2022_d_b8, assign1570_e2022_d_b9, assign1570_e2022_d_b10, assign1570_e2022_d_b11, assign1570_e2022_d_b12, assign1570_e2022_d_b13, assign1570_e2022_d_b14,) = {
    if ((var_guard15 != 0.0) && (!((var_guard13 != 0.0) || (var_guard14 != 0.0)))) {
        let assign1570_e2020: f64 = var_qgd_dn7;
        (assign1570_e2020, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgd, var_cgd_dn0, var_cgd_dn1, var_cgd_dn2, var_cgd_dn3, var_cgd_dn4, var_cgd_dn5, var_cgd_dn6, var_cgd_dn7, var_cgd_dn8, var_cgd_dn9, var_cgd_dn10, var_cgd_dn11, var_cgd_dn12, var_cgd_dn13, var_cgd_dn14, var_cgd_dn15, var_cgd_db0, var_cgd_db1, var_cgd_db2, var_cgd_db3, var_cgd_db4, var_cgd_db5, var_cgd_db6, var_cgd_db7, var_cgd_db8, var_cgd_db9, var_cgd_db10, var_cgd_db11, var_cgd_db12, var_cgd_db13, var_cgd_db14,)
    }
};
        var_cgd = assign1570_e2022;
        var_cgd_dn0 = assign1570_e2022_d_n0;
        var_cgd_dn1 = assign1570_e2022_d_n1;
        var_cgd_dn2 = assign1570_e2022_d_n2;
        var_cgd_dn3 = assign1570_e2022_d_n3;
        var_cgd_dn4 = assign1570_e2022_d_n4;
        var_cgd_dn5 = assign1570_e2022_d_n5;
        var_cgd_dn6 = assign1570_e2022_d_n6;
        var_cgd_dn7 = assign1570_e2022_d_n7;
        var_cgd_dn8 = assign1570_e2022_d_n8;
        var_cgd_dn9 = assign1570_e2022_d_n9;
        var_cgd_dn10 = assign1570_e2022_d_n10;
        var_cgd_dn11 = assign1570_e2022_d_n11;
        var_cgd_dn12 = assign1570_e2022_d_n12;
        var_cgd_dn13 = assign1570_e2022_d_n13;
        var_cgd_dn14 = assign1570_e2022_d_n14;
        var_cgd_dn15 = assign1570_e2022_d_n15;
        var_cgd_db0 = assign1570_e2022_d_b0;
        var_cgd_db1 = assign1570_e2022_d_b1;
        var_cgd_db2 = assign1570_e2022_d_b2;
        var_cgd_db3 = assign1570_e2022_d_b3;
        var_cgd_db4 = assign1570_e2022_d_b4;
        var_cgd_db5 = assign1570_e2022_d_b5;
        var_cgd_db6 = assign1570_e2022_d_b6;
        var_cgd_db7 = assign1570_e2022_d_b7;
        var_cgd_db8 = assign1570_e2022_d_b8;
        var_cgd_db9 = assign1570_e2022_d_b9;
        var_cgd_db10 = assign1570_e2022_d_b10;
        var_cgd_db11 = assign1570_e2022_d_b11;
        var_cgd_db12 = assign1570_e2022_d_b12;
        var_cgd_db13 = assign1570_e2022_d_b13;
        var_cgd_db14 = assign1570_e2022_d_b14;
        var_cgd_db15 = 0.0;
        var_cgd_db16 = 0.0;
        var_cgd_db17 = 0.0;
        var_cgd_db18 = 0.0;
        var_cgd_rv = 0.0;
        var_cgd_rdn0 = 0.0;
        var_cgd_rdn1 = 0.0;
        var_cgd_rdn2 = 0.0;
        var_cgd_rdn3 = 0.0;
        var_cgd_rdn4 = 0.0;
        var_cgd_rdn5 = 0.0;
        var_cgd_rdn6 = 0.0;
        var_cgd_rdn7 = 0.0;
        var_cgd_rdn8 = 0.0;
        var_cgd_rdn9 = 0.0;
        var_cgd_rdn10 = 0.0;
        var_cgd_rdn11 = 0.0;
        var_cgd_rdn12 = 0.0;
        var_cgd_rdn13 = 0.0;
        var_cgd_rdn14 = 0.0;
        var_cgd_rdn15 = 0.0;
        var_cgd_rdb0 = 0.0;
        var_cgd_rdb1 = 0.0;
        var_cgd_rdb2 = 0.0;
        var_cgd_rdb3 = 0.0;
        var_cgd_rdb4 = 0.0;
        var_cgd_rdb5 = 0.0;
        var_cgd_rdb6 = 0.0;
        var_cgd_rdb7 = 0.0;
        var_cgd_rdb8 = 0.0;
        var_cgd_rdb9 = 0.0;
        var_cgd_rdb10 = 0.0;
        var_cgd_rdb11 = 0.0;
        var_cgd_rdb12 = 0.0;
        var_cgd_rdb13 = 0.0;
        var_cgd_rdb14 = 0.0;
        var_cgd_rdb15 = 0.0;
        var_cgd_rdb16 = 0.0;
        var_cgd_rdb17 = 0.0;
        var_cgd_rdb18 = 0.0;

        *var_cgd_slot = var_cgd;
        *var_cgd_db0_slot = var_cgd_db0;
        *var_cgd_db1_slot = var_cgd_db1;
        *var_cgd_db10_slot = var_cgd_db10;
        *var_cgd_db11_slot = var_cgd_db11;
        *var_cgd_db12_slot = var_cgd_db12;
        *var_cgd_db13_slot = var_cgd_db13;
        *var_cgd_db14_slot = var_cgd_db14;
        *var_cgd_db15_slot = var_cgd_db15;
        *var_cgd_db16_slot = var_cgd_db16;
        *var_cgd_db17_slot = var_cgd_db17;
        *var_cgd_db18_slot = var_cgd_db18;
        *var_cgd_db2_slot = var_cgd_db2;
        *var_cgd_db3_slot = var_cgd_db3;
        *var_cgd_db4_slot = var_cgd_db4;
        *var_cgd_db5_slot = var_cgd_db5;
        *var_cgd_db6_slot = var_cgd_db6;
        *var_cgd_db7_slot = var_cgd_db7;
        *var_cgd_db8_slot = var_cgd_db8;
        *var_cgd_db9_slot = var_cgd_db9;
        *var_cgd_dn0_slot = var_cgd_dn0;
        *var_cgd_dn1_slot = var_cgd_dn1;
        *var_cgd_dn10_slot = var_cgd_dn10;
        *var_cgd_dn11_slot = var_cgd_dn11;
        *var_cgd_dn12_slot = var_cgd_dn12;
        *var_cgd_dn13_slot = var_cgd_dn13;
        *var_cgd_dn14_slot = var_cgd_dn14;
        *var_cgd_dn15_slot = var_cgd_dn15;
        *var_cgd_dn2_slot = var_cgd_dn2;
        *var_cgd_dn3_slot = var_cgd_dn3;
        *var_cgd_dn4_slot = var_cgd_dn4;
        *var_cgd_dn5_slot = var_cgd_dn5;
        *var_cgd_dn6_slot = var_cgd_dn6;
        *var_cgd_dn7_slot = var_cgd_dn7;
        *var_cgd_dn8_slot = var_cgd_dn8;
        *var_cgd_dn9_slot = var_cgd_dn9;
        *var_cgd_rdb0_slot = var_cgd_rdb0;
        *var_cgd_rdb1_slot = var_cgd_rdb1;
        *var_cgd_rdb10_slot = var_cgd_rdb10;
        *var_cgd_rdb11_slot = var_cgd_rdb11;
        *var_cgd_rdb12_slot = var_cgd_rdb12;
        *var_cgd_rdb13_slot = var_cgd_rdb13;
        *var_cgd_rdb14_slot = var_cgd_rdb14;
        *var_cgd_rdb15_slot = var_cgd_rdb15;
        *var_cgd_rdb16_slot = var_cgd_rdb16;
        *var_cgd_rdb17_slot = var_cgd_rdb17;
        *var_cgd_rdb18_slot = var_cgd_rdb18;
        *var_cgd_rdb2_slot = var_cgd_rdb2;
        *var_cgd_rdb3_slot = var_cgd_rdb3;
        *var_cgd_rdb4_slot = var_cgd_rdb4;
        *var_cgd_rdb5_slot = var_cgd_rdb5;
        *var_cgd_rdb6_slot = var_cgd_rdb6;
        *var_cgd_rdb7_slot = var_cgd_rdb7;
        *var_cgd_rdb8_slot = var_cgd_rdb8;
        *var_cgd_rdb9_slot = var_cgd_rdb9;
        *var_cgd_rdn0_slot = var_cgd_rdn0;
        *var_cgd_rdn1_slot = var_cgd_rdn1;
        *var_cgd_rdn10_slot = var_cgd_rdn10;
        *var_cgd_rdn11_slot = var_cgd_rdn11;
        *var_cgd_rdn12_slot = var_cgd_rdn12;
        *var_cgd_rdn13_slot = var_cgd_rdn13;
        *var_cgd_rdn14_slot = var_cgd_rdn14;
        *var_cgd_rdn15_slot = var_cgd_rdn15;
        *var_cgd_rdn2_slot = var_cgd_rdn2;
        *var_cgd_rdn3_slot = var_cgd_rdn3;
        *var_cgd_rdn4_slot = var_cgd_rdn4;
        *var_cgd_rdn5_slot = var_cgd_rdn5;
        *var_cgd_rdn6_slot = var_cgd_rdn6;
        *var_cgd_rdn7_slot = var_cgd_rdn7;
        *var_cgd_rdn8_slot = var_cgd_rdn8;
        *var_cgd_rdn9_slot = var_cgd_rdn9;
        *var_cgd_rv_slot = var_cgd_rv;
        *var_cgs_slot = var_cgs;
        *var_cgs_db0_slot = var_cgs_db0;
        *var_cgs_db1_slot = var_cgs_db1;
        *var_cgs_db10_slot = var_cgs_db10;
        *var_cgs_db11_slot = var_cgs_db11;
        *var_cgs_db12_slot = var_cgs_db12;
        *var_cgs_db13_slot = var_cgs_db13;
        *var_cgs_db14_slot = var_cgs_db14;
        *var_cgs_db15_slot = var_cgs_db15;
        *var_cgs_db16_slot = var_cgs_db16;
        *var_cgs_db17_slot = var_cgs_db17;
        *var_cgs_db18_slot = var_cgs_db18;
        *var_cgs_db2_slot = var_cgs_db2;
        *var_cgs_db3_slot = var_cgs_db3;
        *var_cgs_db4_slot = var_cgs_db4;
        *var_cgs_db5_slot = var_cgs_db5;
        *var_cgs_db6_slot = var_cgs_db6;
        *var_cgs_db7_slot = var_cgs_db7;
        *var_cgs_db8_slot = var_cgs_db8;
        *var_cgs_db9_slot = var_cgs_db9;
        *var_cgs_dn0_slot = var_cgs_dn0;
        *var_cgs_dn1_slot = var_cgs_dn1;
        *var_cgs_dn10_slot = var_cgs_dn10;
        *var_cgs_dn11_slot = var_cgs_dn11;
        *var_cgs_dn12_slot = var_cgs_dn12;
        *var_cgs_dn13_slot = var_cgs_dn13;
        *var_cgs_dn14_slot = var_cgs_dn14;
        *var_cgs_dn15_slot = var_cgs_dn15;
        *var_cgs_dn2_slot = var_cgs_dn2;
        *var_cgs_dn3_slot = var_cgs_dn3;
        *var_cgs_dn4_slot = var_cgs_dn4;
        *var_cgs_dn5_slot = var_cgs_dn5;
        *var_cgs_dn6_slot = var_cgs_dn6;
        *var_cgs_dn7_slot = var_cgs_dn7;
        *var_cgs_dn8_slot = var_cgs_dn8;
        *var_cgs_dn9_slot = var_cgs_dn9;
        *var_cgs_rdb0_slot = var_cgs_rdb0;
        *var_cgs_rdb1_slot = var_cgs_rdb1;
        *var_cgs_rdb10_slot = var_cgs_rdb10;
        *var_cgs_rdb11_slot = var_cgs_rdb11;
        *var_cgs_rdb12_slot = var_cgs_rdb12;
        *var_cgs_rdb13_slot = var_cgs_rdb13;
        *var_cgs_rdb14_slot = var_cgs_rdb14;
        *var_cgs_rdb15_slot = var_cgs_rdb15;
        *var_cgs_rdb16_slot = var_cgs_rdb16;
        *var_cgs_rdb17_slot = var_cgs_rdb17;
        *var_cgs_rdb18_slot = var_cgs_rdb18;
        *var_cgs_rdb2_slot = var_cgs_rdb2;
        *var_cgs_rdb3_slot = var_cgs_rdb3;
        *var_cgs_rdb4_slot = var_cgs_rdb4;
        *var_cgs_rdb5_slot = var_cgs_rdb5;
        *var_cgs_rdb6_slot = var_cgs_rdb6;
        *var_cgs_rdb7_slot = var_cgs_rdb7;
        *var_cgs_rdb8_slot = var_cgs_rdb8;
        *var_cgs_rdb9_slot = var_cgs_rdb9;
        *var_cgs_rdn0_slot = var_cgs_rdn0;
        *var_cgs_rdn1_slot = var_cgs_rdn1;
        *var_cgs_rdn10_slot = var_cgs_rdn10;
        *var_cgs_rdn11_slot = var_cgs_rdn11;
        *var_cgs_rdn12_slot = var_cgs_rdn12;
        *var_cgs_rdn13_slot = var_cgs_rdn13;
        *var_cgs_rdn14_slot = var_cgs_rdn14;
        *var_cgs_rdn15_slot = var_cgs_rdn15;
        *var_cgs_rdn2_slot = var_cgs_rdn2;
        *var_cgs_rdn3_slot = var_cgs_rdn3;
        *var_cgs_rdn4_slot = var_cgs_rdn4;
        *var_cgs_rdn5_slot = var_cgs_rdn5;
        *var_cgs_rdn6_slot = var_cgs_rdn6;
        *var_cgs_rdn7_slot = var_cgs_rdn7;
        *var_cgs_rdn8_slot = var_cgs_rdn8;
        *var_cgs_rdn9_slot = var_cgs_rdn9;
        *var_cgs_rv_slot = var_cgs_rv;
        *var_cosh1_slot = var_cosh1;
        *var_cosh1_db0_slot = var_cosh1_db0;
        *var_cosh1_db1_slot = var_cosh1_db1;
        *var_cosh1_db10_slot = var_cosh1_db10;
        *var_cosh1_db11_slot = var_cosh1_db11;
        *var_cosh1_db12_slot = var_cosh1_db12;
        *var_cosh1_db13_slot = var_cosh1_db13;
        *var_cosh1_db14_slot = var_cosh1_db14;
        *var_cosh1_db15_slot = var_cosh1_db15;
        *var_cosh1_db16_slot = var_cosh1_db16;
        *var_cosh1_db17_slot = var_cosh1_db17;
        *var_cosh1_db18_slot = var_cosh1_db18;
        *var_cosh1_db2_slot = var_cosh1_db2;
        *var_cosh1_db3_slot = var_cosh1_db3;
        *var_cosh1_db4_slot = var_cosh1_db4;
        *var_cosh1_db5_slot = var_cosh1_db5;
        *var_cosh1_db6_slot = var_cosh1_db6;
        *var_cosh1_db7_slot = var_cosh1_db7;
        *var_cosh1_db8_slot = var_cosh1_db8;
        *var_cosh1_db9_slot = var_cosh1_db9;
        *var_cosh1_dn0_slot = var_cosh1_dn0;
        *var_cosh1_dn1_slot = var_cosh1_dn1;
        *var_cosh1_dn10_slot = var_cosh1_dn10;
        *var_cosh1_dn11_slot = var_cosh1_dn11;
        *var_cosh1_dn12_slot = var_cosh1_dn12;
        *var_cosh1_dn13_slot = var_cosh1_dn13;
        *var_cosh1_dn14_slot = var_cosh1_dn14;
        *var_cosh1_dn15_slot = var_cosh1_dn15;
        *var_cosh1_dn2_slot = var_cosh1_dn2;
        *var_cosh1_dn3_slot = var_cosh1_dn3;
        *var_cosh1_dn4_slot = var_cosh1_dn4;
        *var_cosh1_dn5_slot = var_cosh1_dn5;
        *var_cosh1_dn6_slot = var_cosh1_dn6;
        *var_cosh1_dn7_slot = var_cosh1_dn7;
        *var_cosh1_dn8_slot = var_cosh1_dn8;
        *var_cosh1_dn9_slot = var_cosh1_dn9;
        *var_cosh1_rdb0_slot = var_cosh1_rdb0;
        *var_cosh1_rdb1_slot = var_cosh1_rdb1;
        *var_cosh1_rdb10_slot = var_cosh1_rdb10;
        *var_cosh1_rdb11_slot = var_cosh1_rdb11;
        *var_cosh1_rdb12_slot = var_cosh1_rdb12;
        *var_cosh1_rdb13_slot = var_cosh1_rdb13;
        *var_cosh1_rdb14_slot = var_cosh1_rdb14;
        *var_cosh1_rdb15_slot = var_cosh1_rdb15;
        *var_cosh1_rdb16_slot = var_cosh1_rdb16;
        *var_cosh1_rdb17_slot = var_cosh1_rdb17;
        *var_cosh1_rdb18_slot = var_cosh1_rdb18;
        *var_cosh1_rdb2_slot = var_cosh1_rdb2;
        *var_cosh1_rdb3_slot = var_cosh1_rdb3;
        *var_cosh1_rdb4_slot = var_cosh1_rdb4;
        *var_cosh1_rdb5_slot = var_cosh1_rdb5;
        *var_cosh1_rdb6_slot = var_cosh1_rdb6;
        *var_cosh1_rdb7_slot = var_cosh1_rdb7;
        *var_cosh1_rdb8_slot = var_cosh1_rdb8;
        *var_cosh1_rdb9_slot = var_cosh1_rdb9;
        *var_cosh1_rdn0_slot = var_cosh1_rdn0;
        *var_cosh1_rdn1_slot = var_cosh1_rdn1;
        *var_cosh1_rdn10_slot = var_cosh1_rdn10;
        *var_cosh1_rdn11_slot = var_cosh1_rdn11;
        *var_cosh1_rdn12_slot = var_cosh1_rdn12;
        *var_cosh1_rdn13_slot = var_cosh1_rdn13;
        *var_cosh1_rdn14_slot = var_cosh1_rdn14;
        *var_cosh1_rdn15_slot = var_cosh1_rdn15;
        *var_cosh1_rdn2_slot = var_cosh1_rdn2;
        *var_cosh1_rdn3_slot = var_cosh1_rdn3;
        *var_cosh1_rdn4_slot = var_cosh1_rdn4;
        *var_cosh1_rdn5_slot = var_cosh1_rdn5;
        *var_cosh1_rdn6_slot = var_cosh1_rdn6;
        *var_cosh1_rdn7_slot = var_cosh1_rdn7;
        *var_cosh1_rdn8_slot = var_cosh1_rdn8;
        *var_cosh1_rdn9_slot = var_cosh1_rdn9;
        *var_cosh1_rv_slot = var_cosh1_rv;
        *var_lc4_slot = var_lc4;
        *var_lc4_db0_slot = var_lc4_db0;
        *var_lc4_db1_slot = var_lc4_db1;
        *var_lc4_db10_slot = var_lc4_db10;
        *var_lc4_db11_slot = var_lc4_db11;
        *var_lc4_db12_slot = var_lc4_db12;
        *var_lc4_db13_slot = var_lc4_db13;
        *var_lc4_db14_slot = var_lc4_db14;
        *var_lc4_db15_slot = var_lc4_db15;
        *var_lc4_db16_slot = var_lc4_db16;
        *var_lc4_db17_slot = var_lc4_db17;
        *var_lc4_db18_slot = var_lc4_db18;
        *var_lc4_db2_slot = var_lc4_db2;
        *var_lc4_db3_slot = var_lc4_db3;
        *var_lc4_db4_slot = var_lc4_db4;
        *var_lc4_db5_slot = var_lc4_db5;
        *var_lc4_db6_slot = var_lc4_db6;
        *var_lc4_db7_slot = var_lc4_db7;
        *var_lc4_db8_slot = var_lc4_db8;
        *var_lc4_db9_slot = var_lc4_db9;
        *var_lc4_dn0_slot = var_lc4_dn0;
        *var_lc4_dn1_slot = var_lc4_dn1;
        *var_lc4_dn10_slot = var_lc4_dn10;
        *var_lc4_dn11_slot = var_lc4_dn11;
        *var_lc4_dn12_slot = var_lc4_dn12;
        *var_lc4_dn13_slot = var_lc4_dn13;
        *var_lc4_dn14_slot = var_lc4_dn14;
        *var_lc4_dn15_slot = var_lc4_dn15;
        *var_lc4_dn2_slot = var_lc4_dn2;
        *var_lc4_dn3_slot = var_lc4_dn3;
        *var_lc4_dn4_slot = var_lc4_dn4;
        *var_lc4_dn5_slot = var_lc4_dn5;
        *var_lc4_dn6_slot = var_lc4_dn6;
        *var_lc4_dn7_slot = var_lc4_dn7;
        *var_lc4_dn8_slot = var_lc4_dn8;
        *var_lc4_dn9_slot = var_lc4_dn9;
        *var_lc4_rdb0_slot = var_lc4_rdb0;
        *var_lc4_rdb1_slot = var_lc4_rdb1;
        *var_lc4_rdb10_slot = var_lc4_rdb10;
        *var_lc4_rdb11_slot = var_lc4_rdb11;
        *var_lc4_rdb12_slot = var_lc4_rdb12;
        *var_lc4_rdb13_slot = var_lc4_rdb13;
        *var_lc4_rdb14_slot = var_lc4_rdb14;
        *var_lc4_rdb15_slot = var_lc4_rdb15;
        *var_lc4_rdb16_slot = var_lc4_rdb16;
        *var_lc4_rdb17_slot = var_lc4_rdb17;
        *var_lc4_rdb18_slot = var_lc4_rdb18;
        *var_lc4_rdb2_slot = var_lc4_rdb2;
        *var_lc4_rdb3_slot = var_lc4_rdb3;
        *var_lc4_rdb4_slot = var_lc4_rdb4;
        *var_lc4_rdb5_slot = var_lc4_rdb5;
        *var_lc4_rdb6_slot = var_lc4_rdb6;
        *var_lc4_rdb7_slot = var_lc4_rdb7;
        *var_lc4_rdb8_slot = var_lc4_rdb8;
        *var_lc4_rdb9_slot = var_lc4_rdb9;
        *var_lc4_rdn0_slot = var_lc4_rdn0;
        *var_lc4_rdn1_slot = var_lc4_rdn1;
        *var_lc4_rdn10_slot = var_lc4_rdn10;
        *var_lc4_rdn11_slot = var_lc4_rdn11;
        *var_lc4_rdn12_slot = var_lc4_rdn12;
        *var_lc4_rdn13_slot = var_lc4_rdn13;
        *var_lc4_rdn14_slot = var_lc4_rdn14;
        *var_lc4_rdn15_slot = var_lc4_rdn15;
        *var_lc4_rdn2_slot = var_lc4_rdn2;
        *var_lc4_rdn3_slot = var_lc4_rdn3;
        *var_lc4_rdn4_slot = var_lc4_rdn4;
        *var_lc4_rdn5_slot = var_lc4_rdn5;
        *var_lc4_rdn6_slot = var_lc4_rdn6;
        *var_lc4_rdn7_slot = var_lc4_rdn7;
        *var_lc4_rdn8_slot = var_lc4_rdn8;
        *var_lc4_rdn9_slot = var_lc4_rdn9;
        *var_lc4_rv_slot = var_lc4_rv;
        *var_qgd_slot = var_qgd;
        *var_qgd0_slot = var_qgd0;
        *var_qgd0_db0_slot = var_qgd0_db0;
        *var_qgd0_db1_slot = var_qgd0_db1;
        *var_qgd0_db10_slot = var_qgd0_db10;
        *var_qgd0_db11_slot = var_qgd0_db11;
        *var_qgd0_db12_slot = var_qgd0_db12;
        *var_qgd0_db13_slot = var_qgd0_db13;
        *var_qgd0_db14_slot = var_qgd0_db14;
        *var_qgd0_db15_slot = var_qgd0_db15;
        *var_qgd0_db16_slot = var_qgd0_db16;
        *var_qgd0_db17_slot = var_qgd0_db17;
        *var_qgd0_db18_slot = var_qgd0_db18;
        *var_qgd0_db2_slot = var_qgd0_db2;
        *var_qgd0_db3_slot = var_qgd0_db3;
        *var_qgd0_db4_slot = var_qgd0_db4;
        *var_qgd0_db5_slot = var_qgd0_db5;
        *var_qgd0_db6_slot = var_qgd0_db6;
        *var_qgd0_db7_slot = var_qgd0_db7;
        *var_qgd0_db8_slot = var_qgd0_db8;
        *var_qgd0_db9_slot = var_qgd0_db9;
        *var_qgd0_dn0_slot = var_qgd0_dn0;
        *var_qgd0_dn1_slot = var_qgd0_dn1;
        *var_qgd0_dn10_slot = var_qgd0_dn10;
        *var_qgd0_dn11_slot = var_qgd0_dn11;
        *var_qgd0_dn12_slot = var_qgd0_dn12;
        *var_qgd0_dn13_slot = var_qgd0_dn13;
        *var_qgd0_dn14_slot = var_qgd0_dn14;
        *var_qgd0_dn15_slot = var_qgd0_dn15;
        *var_qgd0_dn2_slot = var_qgd0_dn2;
        *var_qgd0_dn3_slot = var_qgd0_dn3;
        *var_qgd0_dn4_slot = var_qgd0_dn4;
        *var_qgd0_dn5_slot = var_qgd0_dn5;
        *var_qgd0_dn6_slot = var_qgd0_dn6;
        *var_qgd0_dn7_slot = var_qgd0_dn7;
        *var_qgd0_dn8_slot = var_qgd0_dn8;
        *var_qgd0_dn9_slot = var_qgd0_dn9;
        *var_qgd0_rdb0_slot = var_qgd0_rdb0;
        *var_qgd0_rdb1_slot = var_qgd0_rdb1;
        *var_qgd0_rdb10_slot = var_qgd0_rdb10;
        *var_qgd0_rdb11_slot = var_qgd0_rdb11;
        *var_qgd0_rdb12_slot = var_qgd0_rdb12;
        *var_qgd0_rdb13_slot = var_qgd0_rdb13;
        *var_qgd0_rdb14_slot = var_qgd0_rdb14;
        *var_qgd0_rdb15_slot = var_qgd0_rdb15;
        *var_qgd0_rdb16_slot = var_qgd0_rdb16;
        *var_qgd0_rdb17_slot = var_qgd0_rdb17;
        *var_qgd0_rdb18_slot = var_qgd0_rdb18;
        *var_qgd0_rdb2_slot = var_qgd0_rdb2;
        *var_qgd0_rdb3_slot = var_qgd0_rdb3;
        *var_qgd0_rdb4_slot = var_qgd0_rdb4;
        *var_qgd0_rdb5_slot = var_qgd0_rdb5;
        *var_qgd0_rdb6_slot = var_qgd0_rdb6;
        *var_qgd0_rdb7_slot = var_qgd0_rdb7;
        *var_qgd0_rdb8_slot = var_qgd0_rdb8;
        *var_qgd0_rdb9_slot = var_qgd0_rdb9;
        *var_qgd0_rdn0_slot = var_qgd0_rdn0;
        *var_qgd0_rdn1_slot = var_qgd0_rdn1;
        *var_qgd0_rdn10_slot = var_qgd0_rdn10;
        *var_qgd0_rdn11_slot = var_qgd0_rdn11;
        *var_qgd0_rdn12_slot = var_qgd0_rdn12;
        *var_qgd0_rdn13_slot = var_qgd0_rdn13;
        *var_qgd0_rdn14_slot = var_qgd0_rdn14;
        *var_qgd0_rdn15_slot = var_qgd0_rdn15;
        *var_qgd0_rdn2_slot = var_qgd0_rdn2;
        *var_qgd0_rdn3_slot = var_qgd0_rdn3;
        *var_qgd0_rdn4_slot = var_qgd0_rdn4;
        *var_qgd0_rdn5_slot = var_qgd0_rdn5;
        *var_qgd0_rdn6_slot = var_qgd0_rdn6;
        *var_qgd0_rdn7_slot = var_qgd0_rdn7;
        *var_qgd0_rdn8_slot = var_qgd0_rdn8;
        *var_qgd0_rdn9_slot = var_qgd0_rdn9;
        *var_qgd0_rv_slot = var_qgd0_rv;
        *var_qgd_db0_slot = var_qgd_db0;
        *var_qgd_db1_slot = var_qgd_db1;
        *var_qgd_db10_slot = var_qgd_db10;
        *var_qgd_db11_slot = var_qgd_db11;
        *var_qgd_db12_slot = var_qgd_db12;
        *var_qgd_db13_slot = var_qgd_db13;
        *var_qgd_db14_slot = var_qgd_db14;
        *var_qgd_db15_slot = var_qgd_db15;
        *var_qgd_db16_slot = var_qgd_db16;
        *var_qgd_db17_slot = var_qgd_db17;
        *var_qgd_db18_slot = var_qgd_db18;
        *var_qgd_db2_slot = var_qgd_db2;
        *var_qgd_db3_slot = var_qgd_db3;
        *var_qgd_db4_slot = var_qgd_db4;
        *var_qgd_db5_slot = var_qgd_db5;
        *var_qgd_db6_slot = var_qgd_db6;
        *var_qgd_db7_slot = var_qgd_db7;
        *var_qgd_db8_slot = var_qgd_db8;
        *var_qgd_db9_slot = var_qgd_db9;
        *var_qgd_dn0_slot = var_qgd_dn0;
        *var_qgd_dn1_slot = var_qgd_dn1;
        *var_qgd_dn10_slot = var_qgd_dn10;
        *var_qgd_dn11_slot = var_qgd_dn11;
        *var_qgd_dn12_slot = var_qgd_dn12;
        *var_qgd_dn13_slot = var_qgd_dn13;
        *var_qgd_dn14_slot = var_qgd_dn14;
        *var_qgd_dn15_slot = var_qgd_dn15;
        *var_qgd_dn2_slot = var_qgd_dn2;
        *var_qgd_dn3_slot = var_qgd_dn3;
        *var_qgd_dn4_slot = var_qgd_dn4;
        *var_qgd_dn5_slot = var_qgd_dn5;
        *var_qgd_dn6_slot = var_qgd_dn6;
        *var_qgd_dn7_slot = var_qgd_dn7;
        *var_qgd_dn8_slot = var_qgd_dn8;
        *var_qgd_dn9_slot = var_qgd_dn9;
        *var_qgd_rdb0_slot = var_qgd_rdb0;
        *var_qgd_rdb1_slot = var_qgd_rdb1;
        *var_qgd_rdb10_slot = var_qgd_rdb10;
        *var_qgd_rdb11_slot = var_qgd_rdb11;
        *var_qgd_rdb12_slot = var_qgd_rdb12;
        *var_qgd_rdb13_slot = var_qgd_rdb13;
        *var_qgd_rdb14_slot = var_qgd_rdb14;
        *var_qgd_rdb15_slot = var_qgd_rdb15;
        *var_qgd_rdb16_slot = var_qgd_rdb16;
        *var_qgd_rdb17_slot = var_qgd_rdb17;
        *var_qgd_rdb18_slot = var_qgd_rdb18;
        *var_qgd_rdb2_slot = var_qgd_rdb2;
        *var_qgd_rdb3_slot = var_qgd_rdb3;
        *var_qgd_rdb4_slot = var_qgd_rdb4;
        *var_qgd_rdb5_slot = var_qgd_rdb5;
        *var_qgd_rdb6_slot = var_qgd_rdb6;
        *var_qgd_rdb7_slot = var_qgd_rdb7;
        *var_qgd_rdb8_slot = var_qgd_rdb8;
        *var_qgd_rdb9_slot = var_qgd_rdb9;
        *var_qgd_rdn0_slot = var_qgd_rdn0;
        *var_qgd_rdn1_slot = var_qgd_rdn1;
        *var_qgd_rdn10_slot = var_qgd_rdn10;
        *var_qgd_rdn11_slot = var_qgd_rdn11;
        *var_qgd_rdn12_slot = var_qgd_rdn12;
        *var_qgd_rdn13_slot = var_qgd_rdn13;
        *var_qgd_rdn14_slot = var_qgd_rdn14;
        *var_qgd_rdn15_slot = var_qgd_rdn15;
        *var_qgd_rdn2_slot = var_qgd_rdn2;
        *var_qgd_rdn3_slot = var_qgd_rdn3;
        *var_qgd_rdn4_slot = var_qgd_rdn4;
        *var_qgd_rdn5_slot = var_qgd_rdn5;
        *var_qgd_rdn6_slot = var_qgd_rdn6;
        *var_qgd_rdn7_slot = var_qgd_rdn7;
        *var_qgd_rdn8_slot = var_qgd_rdn8;
        *var_qgd_rdn9_slot = var_qgd_rdn9;
        *var_qgd_rv_slot = var_qgd_rv;
    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        var_guard16_slot: &mut f64,
        var_guard16_db0_slot: &mut f64,
        var_guard16_db1_slot: &mut f64,
        var_guard16_db10_slot: &mut f64,
        var_guard16_db11_slot: &mut f64,
        var_guard16_db12_slot: &mut f64,
        var_guard16_db13_slot: &mut f64,
        var_guard16_db14_slot: &mut f64,
        var_guard16_db15_slot: &mut f64,
        var_guard16_db16_slot: &mut f64,
        var_guard16_db17_slot: &mut f64,
        var_guard16_db18_slot: &mut f64,
        var_guard16_db2_slot: &mut f64,
        var_guard16_db3_slot: &mut f64,
        var_guard16_db4_slot: &mut f64,
        var_guard16_db5_slot: &mut f64,
        var_guard16_db6_slot: &mut f64,
        var_guard16_db7_slot: &mut f64,
        var_guard16_db8_slot: &mut f64,
        var_guard16_db9_slot: &mut f64,
        var_guard16_dn0_slot: &mut f64,
        var_guard16_dn1_slot: &mut f64,
        var_guard16_dn10_slot: &mut f64,
        var_guard16_dn11_slot: &mut f64,
        var_guard16_dn12_slot: &mut f64,
        var_guard16_dn13_slot: &mut f64,
        var_guard16_dn14_slot: &mut f64,
        var_guard16_dn15_slot: &mut f64,
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
        var_guard16_rdb10_slot: &mut f64,
        var_guard16_rdb11_slot: &mut f64,
        var_guard16_rdb12_slot: &mut f64,
        var_guard16_rdb13_slot: &mut f64,
        var_guard16_rdb14_slot: &mut f64,
        var_guard16_rdb15_slot: &mut f64,
        var_guard16_rdb16_slot: &mut f64,
        var_guard16_rdb17_slot: &mut f64,
        var_guard16_rdb18_slot: &mut f64,
        var_guard16_rdb2_slot: &mut f64,
        var_guard16_rdb3_slot: &mut f64,
        var_guard16_rdb4_slot: &mut f64,
        var_guard16_rdb5_slot: &mut f64,
        var_guard16_rdb6_slot: &mut f64,
        var_guard16_rdb7_slot: &mut f64,
        var_guard16_rdb8_slot: &mut f64,
        var_guard16_rdb9_slot: &mut f64,
        var_guard16_rdn0_slot: &mut f64,
        var_guard16_rdn1_slot: &mut f64,
        var_guard16_rdn10_slot: &mut f64,
        var_guard16_rdn11_slot: &mut f64,
        var_guard16_rdn12_slot: &mut f64,
        var_guard16_rdn13_slot: &mut f64,
        var_guard16_rdn14_slot: &mut f64,
        var_guard16_rdn15_slot: &mut f64,
        var_guard16_rdn2_slot: &mut f64,
        var_guard16_rdn3_slot: &mut f64,
        var_guard16_rdn4_slot: &mut f64,
        var_guard16_rdn5_slot: &mut f64,
        var_guard16_rdn6_slot: &mut f64,
        var_guard16_rdn7_slot: &mut f64,
        var_guard16_rdn8_slot: &mut f64,
        var_guard16_rdn9_slot: &mut f64,
        var_guard16_rv_slot: &mut f64,
        var_guard21_slot: &mut f64,
        var_guard21_db0_slot: &mut f64,
        var_guard21_db1_slot: &mut f64,
        var_guard21_db10_slot: &mut f64,
        var_guard21_db11_slot: &mut f64,
        var_guard21_db12_slot: &mut f64,
        var_guard21_db13_slot: &mut f64,
        var_guard21_db14_slot: &mut f64,
        var_guard21_db15_slot: &mut f64,
        var_guard21_db16_slot: &mut f64,
        var_guard21_db17_slot: &mut f64,
        var_guard21_db18_slot: &mut f64,
        var_guard21_db2_slot: &mut f64,
        var_guard21_db3_slot: &mut f64,
        var_guard21_db4_slot: &mut f64,
        var_guard21_db5_slot: &mut f64,
        var_guard21_db6_slot: &mut f64,
        var_guard21_db7_slot: &mut f64,
        var_guard21_db8_slot: &mut f64,
        var_guard21_db9_slot: &mut f64,
        var_guard21_dn0_slot: &mut f64,
        var_guard21_dn1_slot: &mut f64,
        var_guard21_dn10_slot: &mut f64,
        var_guard21_dn11_slot: &mut f64,
        var_guard21_dn12_slot: &mut f64,
        var_guard21_dn13_slot: &mut f64,
        var_guard21_dn14_slot: &mut f64,
        var_guard21_dn15_slot: &mut f64,
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
        var_guard21_rdb10_slot: &mut f64,
        var_guard21_rdb11_slot: &mut f64,
        var_guard21_rdb12_slot: &mut f64,
        var_guard21_rdb13_slot: &mut f64,
        var_guard21_rdb14_slot: &mut f64,
        var_guard21_rdb15_slot: &mut f64,
        var_guard21_rdb16_slot: &mut f64,
        var_guard21_rdb17_slot: &mut f64,
        var_guard21_rdb18_slot: &mut f64,
        var_guard21_rdb2_slot: &mut f64,
        var_guard21_rdb3_slot: &mut f64,
        var_guard21_rdb4_slot: &mut f64,
        var_guard21_rdb5_slot: &mut f64,
        var_guard21_rdb6_slot: &mut f64,
        var_guard21_rdb7_slot: &mut f64,
        var_guard21_rdb8_slot: &mut f64,
        var_guard21_rdb9_slot: &mut f64,
        var_guard21_rdn0_slot: &mut f64,
        var_guard21_rdn1_slot: &mut f64,
        var_guard21_rdn10_slot: &mut f64,
        var_guard21_rdn11_slot: &mut f64,
        var_guard21_rdn12_slot: &mut f64,
        var_guard21_rdn13_slot: &mut f64,
        var_guard21_rdn14_slot: &mut f64,
        var_guard21_rdn15_slot: &mut f64,
        var_guard21_rdn2_slot: &mut f64,
        var_guard21_rdn3_slot: &mut f64,
        var_guard21_rdn4_slot: &mut f64,
        var_guard21_rdn5_slot: &mut f64,
        var_guard21_rdn6_slot: &mut f64,
        var_guard21_rdn7_slot: &mut f64,
        var_guard21_rdn8_slot: &mut f64,
        var_guard21_rdn9_slot: &mut f64,
        var_guard21_rv_slot: &mut f64,
        var_guard22_slot: &mut f64,
        var_guard22_db0_slot: &mut f64,
        var_guard22_db1_slot: &mut f64,
        var_guard22_db10_slot: &mut f64,
        var_guard22_db11_slot: &mut f64,
        var_guard22_db12_slot: &mut f64,
        var_guard22_db13_slot: &mut f64,
        var_guard22_db14_slot: &mut f64,
        var_guard22_db15_slot: &mut f64,
        var_guard22_db16_slot: &mut f64,
        var_guard22_db17_slot: &mut f64,
        var_guard22_db18_slot: &mut f64,
        var_guard22_db2_slot: &mut f64,
        var_guard22_db3_slot: &mut f64,
        var_guard22_db4_slot: &mut f64,
        var_guard22_db5_slot: &mut f64,
        var_guard22_db6_slot: &mut f64,
        var_guard22_db7_slot: &mut f64,
        var_guard22_db8_slot: &mut f64,
        var_guard22_db9_slot: &mut f64,
        var_guard22_dn0_slot: &mut f64,
        var_guard22_dn1_slot: &mut f64,
        var_guard22_dn10_slot: &mut f64,
        var_guard22_dn11_slot: &mut f64,
        var_guard22_dn12_slot: &mut f64,
        var_guard22_dn13_slot: &mut f64,
        var_guard22_dn14_slot: &mut f64,
        var_guard22_dn15_slot: &mut f64,
        var_guard22_dn2_slot: &mut f64,
        var_guard22_dn3_slot: &mut f64,
        var_guard22_dn4_slot: &mut f64,
        var_guard22_dn5_slot: &mut f64,
        var_guard22_dn6_slot: &mut f64,
        var_guard22_dn7_slot: &mut f64,
        var_guard22_dn8_slot: &mut f64,
        var_guard22_dn9_slot: &mut f64,
        var_guard22_rdb0_slot: &mut f64,
        var_guard22_rdb1_slot: &mut f64,
        var_guard22_rdb10_slot: &mut f64,
        var_guard22_rdb11_slot: &mut f64,
        var_guard22_rdb12_slot: &mut f64,
        var_guard22_rdb13_slot: &mut f64,
        var_guard22_rdb14_slot: &mut f64,
        var_guard22_rdb15_slot: &mut f64,
        var_guard22_rdb16_slot: &mut f64,
        var_guard22_rdb17_slot: &mut f64,
        var_guard22_rdb18_slot: &mut f64,
        var_guard22_rdb2_slot: &mut f64,
        var_guard22_rdb3_slot: &mut f64,
        var_guard22_rdb4_slot: &mut f64,
        var_guard22_rdb5_slot: &mut f64,
        var_guard22_rdb6_slot: &mut f64,
        var_guard22_rdb7_slot: &mut f64,
        var_guard22_rdb8_slot: &mut f64,
        var_guard22_rdb9_slot: &mut f64,
        var_guard22_rdn0_slot: &mut f64,
        var_guard22_rdn1_slot: &mut f64,
        var_guard22_rdn10_slot: &mut f64,
        var_guard22_rdn11_slot: &mut f64,
        var_guard22_rdn12_slot: &mut f64,
        var_guard22_rdn13_slot: &mut f64,
        var_guard22_rdn14_slot: &mut f64,
        var_guard22_rdn15_slot: &mut f64,
        var_guard22_rdn2_slot: &mut f64,
        var_guard22_rdn3_slot: &mut f64,
        var_guard22_rdn4_slot: &mut f64,
        var_guard22_rdn5_slot: &mut f64,
        var_guard22_rdn6_slot: &mut f64,
        var_guard22_rdn7_slot: &mut f64,
        var_guard22_rdn8_slot: &mut f64,
        var_guard22_rdn9_slot: &mut f64,
        var_guard22_rv_slot: &mut f64,
        var_guard24_slot: &mut f64,
        var_guard24_db0_slot: &mut f64,
        var_guard24_db1_slot: &mut f64,
        var_guard24_db10_slot: &mut f64,
        var_guard24_db11_slot: &mut f64,
        var_guard24_db12_slot: &mut f64,
        var_guard24_db13_slot: &mut f64,
        var_guard24_db14_slot: &mut f64,
        var_guard24_db15_slot: &mut f64,
        var_guard24_db16_slot: &mut f64,
        var_guard24_db17_slot: &mut f64,
        var_guard24_db18_slot: &mut f64,
        var_guard24_db2_slot: &mut f64,
        var_guard24_db3_slot: &mut f64,
        var_guard24_db4_slot: &mut f64,
        var_guard24_db5_slot: &mut f64,
        var_guard24_db6_slot: &mut f64,
        var_guard24_db7_slot: &mut f64,
        var_guard24_db8_slot: &mut f64,
        var_guard24_db9_slot: &mut f64,
        var_guard24_dn0_slot: &mut f64,
        var_guard24_dn1_slot: &mut f64,
        var_guard24_dn10_slot: &mut f64,
        var_guard24_dn11_slot: &mut f64,
        var_guard24_dn12_slot: &mut f64,
        var_guard24_dn13_slot: &mut f64,
        var_guard24_dn14_slot: &mut f64,
        var_guard24_dn15_slot: &mut f64,
        var_guard24_dn2_slot: &mut f64,
        var_guard24_dn3_slot: &mut f64,
        var_guard24_dn4_slot: &mut f64,
        var_guard24_dn5_slot: &mut f64,
        var_guard24_dn6_slot: &mut f64,
        var_guard24_dn7_slot: &mut f64,
        var_guard24_dn8_slot: &mut f64,
        var_guard24_dn9_slot: &mut f64,
        var_guard24_rdb0_slot: &mut f64,
        var_guard24_rdb1_slot: &mut f64,
        var_guard24_rdb10_slot: &mut f64,
        var_guard24_rdb11_slot: &mut f64,
        var_guard24_rdb12_slot: &mut f64,
        var_guard24_rdb13_slot: &mut f64,
        var_guard24_rdb14_slot: &mut f64,
        var_guard24_rdb15_slot: &mut f64,
        var_guard24_rdb16_slot: &mut f64,
        var_guard24_rdb17_slot: &mut f64,
        var_guard24_rdb18_slot: &mut f64,
        var_guard24_rdb2_slot: &mut f64,
        var_guard24_rdb3_slot: &mut f64,
        var_guard24_rdb4_slot: &mut f64,
        var_guard24_rdb5_slot: &mut f64,
        var_guard24_rdb6_slot: &mut f64,
        var_guard24_rdb7_slot: &mut f64,
        var_guard24_rdb8_slot: &mut f64,
        var_guard24_rdb9_slot: &mut f64,
        var_guard24_rdn0_slot: &mut f64,
        var_guard24_rdn1_slot: &mut f64,
        var_guard24_rdn10_slot: &mut f64,
        var_guard24_rdn11_slot: &mut f64,
        var_guard24_rdn12_slot: &mut f64,
        var_guard24_rdn13_slot: &mut f64,
        var_guard24_rdn14_slot: &mut f64,
        var_guard24_rdn15_slot: &mut f64,
        var_guard24_rdn2_slot: &mut f64,
        var_guard24_rdn3_slot: &mut f64,
        var_guard24_rdn4_slot: &mut f64,
        var_guard24_rdn5_slot: &mut f64,
        var_guard24_rdn6_slot: &mut f64,
        var_guard24_rdn7_slot: &mut f64,
        var_guard24_rdn8_slot: &mut f64,
        var_guard24_rdn9_slot: &mut f64,
        var_guard24_rv_slot: &mut f64,
        var_guard25_slot: &mut f64,
        var_guard25_db0_slot: &mut f64,
        var_guard25_db1_slot: &mut f64,
        var_guard25_db10_slot: &mut f64,
        var_guard25_db11_slot: &mut f64,
        var_guard25_db12_slot: &mut f64,
        var_guard25_db13_slot: &mut f64,
        var_guard25_db14_slot: &mut f64,
        var_guard25_db15_slot: &mut f64,
        var_guard25_db16_slot: &mut f64,
        var_guard25_db17_slot: &mut f64,
        var_guard25_db18_slot: &mut f64,
        var_guard25_db2_slot: &mut f64,
        var_guard25_db3_slot: &mut f64,
        var_guard25_db4_slot: &mut f64,
        var_guard25_db5_slot: &mut f64,
        var_guard25_db6_slot: &mut f64,
        var_guard25_db7_slot: &mut f64,
        var_guard25_db8_slot: &mut f64,
        var_guard25_db9_slot: &mut f64,
        var_guard25_dn0_slot: &mut f64,
        var_guard25_dn1_slot: &mut f64,
        var_guard25_dn10_slot: &mut f64,
        var_guard25_dn11_slot: &mut f64,
        var_guard25_dn12_slot: &mut f64,
        var_guard25_dn13_slot: &mut f64,
        var_guard25_dn14_slot: &mut f64,
        var_guard25_dn15_slot: &mut f64,
        var_guard25_dn2_slot: &mut f64,
        var_guard25_dn3_slot: &mut f64,
        var_guard25_dn4_slot: &mut f64,
        var_guard25_dn5_slot: &mut f64,
        var_guard25_dn6_slot: &mut f64,
        var_guard25_dn7_slot: &mut f64,
        var_guard25_dn8_slot: &mut f64,
        var_guard25_dn9_slot: &mut f64,
        var_guard25_rdb0_slot: &mut f64,
        var_guard25_rdb1_slot: &mut f64,
        var_guard25_rdb10_slot: &mut f64,
        var_guard25_rdb11_slot: &mut f64,
        var_guard25_rdb12_slot: &mut f64,
        var_guard25_rdb13_slot: &mut f64,
        var_guard25_rdb14_slot: &mut f64,
        var_guard25_rdb15_slot: &mut f64,
        var_guard25_rdb16_slot: &mut f64,
        var_guard25_rdb17_slot: &mut f64,
        var_guard25_rdb18_slot: &mut f64,
        var_guard25_rdb2_slot: &mut f64,
        var_guard25_rdb3_slot: &mut f64,
        var_guard25_rdb4_slot: &mut f64,
        var_guard25_rdb5_slot: &mut f64,
        var_guard25_rdb6_slot: &mut f64,
        var_guard25_rdb7_slot: &mut f64,
        var_guard25_rdb8_slot: &mut f64,
        var_guard25_rdb9_slot: &mut f64,
        var_guard25_rdn0_slot: &mut f64,
        var_guard25_rdn1_slot: &mut f64,
        var_guard25_rdn10_slot: &mut f64,
        var_guard25_rdn11_slot: &mut f64,
        var_guard25_rdn12_slot: &mut f64,
        var_guard25_rdn13_slot: &mut f64,
        var_guard25_rdn14_slot: &mut f64,
        var_guard25_rdn15_slot: &mut f64,
        var_guard25_rdn2_slot: &mut f64,
        var_guard25_rdn3_slot: &mut f64,
        var_guard25_rdn4_slot: &mut f64,
        var_guard25_rdn5_slot: &mut f64,
        var_guard25_rdn6_slot: &mut f64,
        var_guard25_rdn7_slot: &mut f64,
        var_guard25_rdn8_slot: &mut f64,
        var_guard25_rdn9_slot: &mut f64,
        var_guard25_rv_slot: &mut f64,
        var_guard26_slot: &mut f64,
        var_guard26_db0_slot: &mut f64,
        var_guard26_db1_slot: &mut f64,
        var_guard26_db10_slot: &mut f64,
        var_guard26_db11_slot: &mut f64,
        var_guard26_db12_slot: &mut f64,
        var_guard26_db13_slot: &mut f64,
        var_guard26_db14_slot: &mut f64,
        var_guard26_db15_slot: &mut f64,
        var_guard26_db16_slot: &mut f64,
        var_guard26_db17_slot: &mut f64,
        var_guard26_db18_slot: &mut f64,
        var_guard26_db2_slot: &mut f64,
        var_guard26_db3_slot: &mut f64,
        var_guard26_db4_slot: &mut f64,
        var_guard26_db5_slot: &mut f64,
        var_guard26_db6_slot: &mut f64,
        var_guard26_db7_slot: &mut f64,
        var_guard26_db8_slot: &mut f64,
        var_guard26_db9_slot: &mut f64,
        var_guard26_dn0_slot: &mut f64,
        var_guard26_dn1_slot: &mut f64,
        var_guard26_dn10_slot: &mut f64,
        var_guard26_dn11_slot: &mut f64,
        var_guard26_dn12_slot: &mut f64,
        var_guard26_dn13_slot: &mut f64,
        var_guard26_dn14_slot: &mut f64,
        var_guard26_dn15_slot: &mut f64,
        var_guard26_dn2_slot: &mut f64,
        var_guard26_dn3_slot: &mut f64,
        var_guard26_dn4_slot: &mut f64,
        var_guard26_dn5_slot: &mut f64,
        var_guard26_dn6_slot: &mut f64,
        var_guard26_dn7_slot: &mut f64,
        var_guard26_dn8_slot: &mut f64,
        var_guard26_dn9_slot: &mut f64,
        var_guard26_rdb0_slot: &mut f64,
        var_guard26_rdb1_slot: &mut f64,
        var_guard26_rdb10_slot: &mut f64,
        var_guard26_rdb11_slot: &mut f64,
        var_guard26_rdb12_slot: &mut f64,
        var_guard26_rdb13_slot: &mut f64,
        var_guard26_rdb14_slot: &mut f64,
        var_guard26_rdb15_slot: &mut f64,
        var_guard26_rdb16_slot: &mut f64,
        var_guard26_rdb17_slot: &mut f64,
        var_guard26_rdb18_slot: &mut f64,
        var_guard26_rdb2_slot: &mut f64,
        var_guard26_rdb3_slot: &mut f64,
        var_guard26_rdb4_slot: &mut f64,
        var_guard26_rdb5_slot: &mut f64,
        var_guard26_rdb6_slot: &mut f64,
        var_guard26_rdb7_slot: &mut f64,
        var_guard26_rdb8_slot: &mut f64,
        var_guard26_rdb9_slot: &mut f64,
        var_guard26_rdn0_slot: &mut f64,
        var_guard26_rdn1_slot: &mut f64,
        var_guard26_rdn10_slot: &mut f64,
        var_guard26_rdn11_slot: &mut f64,
        var_guard26_rdn12_slot: &mut f64,
        var_guard26_rdn13_slot: &mut f64,
        var_guard26_rdn14_slot: &mut f64,
        var_guard26_rdn15_slot: &mut f64,
        var_guard26_rdn2_slot: &mut f64,
        var_guard26_rdn3_slot: &mut f64,
        var_guard26_rdn4_slot: &mut f64,
        var_guard26_rdn5_slot: &mut f64,
        var_guard26_rdn6_slot: &mut f64,
        var_guard26_rdn7_slot: &mut f64,
        var_guard26_rdn8_slot: &mut f64,
        var_guard26_rdn9_slot: &mut f64,
        var_guard26_rv_slot: &mut f64,
    ) {
        let mut var_guard16: f64 = *var_guard16_slot;
        let mut var_guard16_db0: f64 = *var_guard16_db0_slot;
        let mut var_guard16_db1: f64 = *var_guard16_db1_slot;
        let mut var_guard16_db10: f64 = *var_guard16_db10_slot;
        let mut var_guard16_db11: f64 = *var_guard16_db11_slot;
        let mut var_guard16_db12: f64 = *var_guard16_db12_slot;
        let mut var_guard16_db13: f64 = *var_guard16_db13_slot;
        let mut var_guard16_db14: f64 = *var_guard16_db14_slot;
        let mut var_guard16_db15: f64 = *var_guard16_db15_slot;
        let mut var_guard16_db16: f64 = *var_guard16_db16_slot;
        let mut var_guard16_db17: f64 = *var_guard16_db17_slot;
        let mut var_guard16_db18: f64 = *var_guard16_db18_slot;
        let mut var_guard16_db2: f64 = *var_guard16_db2_slot;
        let mut var_guard16_db3: f64 = *var_guard16_db3_slot;
        let mut var_guard16_db4: f64 = *var_guard16_db4_slot;
        let mut var_guard16_db5: f64 = *var_guard16_db5_slot;
        let mut var_guard16_db6: f64 = *var_guard16_db6_slot;
        let mut var_guard16_db7: f64 = *var_guard16_db7_slot;
        let mut var_guard16_db8: f64 = *var_guard16_db8_slot;
        let mut var_guard16_db9: f64 = *var_guard16_db9_slot;
        let mut var_guard16_dn0: f64 = *var_guard16_dn0_slot;
        let mut var_guard16_dn1: f64 = *var_guard16_dn1_slot;
        let mut var_guard16_dn10: f64 = *var_guard16_dn10_slot;
        let mut var_guard16_dn11: f64 = *var_guard16_dn11_slot;
        let mut var_guard16_dn12: f64 = *var_guard16_dn12_slot;
        let mut var_guard16_dn13: f64 = *var_guard16_dn13_slot;
        let mut var_guard16_dn14: f64 = *var_guard16_dn14_slot;
        let mut var_guard16_dn15: f64 = *var_guard16_dn15_slot;
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
        let mut var_guard16_rdb10: f64 = *var_guard16_rdb10_slot;
        let mut var_guard16_rdb11: f64 = *var_guard16_rdb11_slot;
        let mut var_guard16_rdb12: f64 = *var_guard16_rdb12_slot;
        let mut var_guard16_rdb13: f64 = *var_guard16_rdb13_slot;
        let mut var_guard16_rdb14: f64 = *var_guard16_rdb14_slot;
        let mut var_guard16_rdb15: f64 = *var_guard16_rdb15_slot;
        let mut var_guard16_rdb16: f64 = *var_guard16_rdb16_slot;
        let mut var_guard16_rdb17: f64 = *var_guard16_rdb17_slot;
        let mut var_guard16_rdb18: f64 = *var_guard16_rdb18_slot;
        let mut var_guard16_rdb2: f64 = *var_guard16_rdb2_slot;
        let mut var_guard16_rdb3: f64 = *var_guard16_rdb3_slot;
        let mut var_guard16_rdb4: f64 = *var_guard16_rdb4_slot;
        let mut var_guard16_rdb5: f64 = *var_guard16_rdb5_slot;
        let mut var_guard16_rdb6: f64 = *var_guard16_rdb6_slot;
        let mut var_guard16_rdb7: f64 = *var_guard16_rdb7_slot;
        let mut var_guard16_rdb8: f64 = *var_guard16_rdb8_slot;
        let mut var_guard16_rdb9: f64 = *var_guard16_rdb9_slot;
        let mut var_guard16_rdn0: f64 = *var_guard16_rdn0_slot;
        let mut var_guard16_rdn1: f64 = *var_guard16_rdn1_slot;
        let mut var_guard16_rdn10: f64 = *var_guard16_rdn10_slot;
        let mut var_guard16_rdn11: f64 = *var_guard16_rdn11_slot;
        let mut var_guard16_rdn12: f64 = *var_guard16_rdn12_slot;
        let mut var_guard16_rdn13: f64 = *var_guard16_rdn13_slot;
        let mut var_guard16_rdn14: f64 = *var_guard16_rdn14_slot;
        let mut var_guard16_rdn15: f64 = *var_guard16_rdn15_slot;
        let mut var_guard16_rdn2: f64 = *var_guard16_rdn2_slot;
        let mut var_guard16_rdn3: f64 = *var_guard16_rdn3_slot;
        let mut var_guard16_rdn4: f64 = *var_guard16_rdn4_slot;
        let mut var_guard16_rdn5: f64 = *var_guard16_rdn5_slot;
        let mut var_guard16_rdn6: f64 = *var_guard16_rdn6_slot;
        let mut var_guard16_rdn7: f64 = *var_guard16_rdn7_slot;
        let mut var_guard16_rdn8: f64 = *var_guard16_rdn8_slot;
        let mut var_guard16_rdn9: f64 = *var_guard16_rdn9_slot;
        let mut var_guard16_rv: f64 = *var_guard16_rv_slot;
        let mut var_guard21: f64 = *var_guard21_slot;
        let mut var_guard21_db0: f64 = *var_guard21_db0_slot;
        let mut var_guard21_db1: f64 = *var_guard21_db1_slot;
        let mut var_guard21_db10: f64 = *var_guard21_db10_slot;
        let mut var_guard21_db11: f64 = *var_guard21_db11_slot;
        let mut var_guard21_db12: f64 = *var_guard21_db12_slot;
        let mut var_guard21_db13: f64 = *var_guard21_db13_slot;
        let mut var_guard21_db14: f64 = *var_guard21_db14_slot;
        let mut var_guard21_db15: f64 = *var_guard21_db15_slot;
        let mut var_guard21_db16: f64 = *var_guard21_db16_slot;
        let mut var_guard21_db17: f64 = *var_guard21_db17_slot;
        let mut var_guard21_db18: f64 = *var_guard21_db18_slot;
        let mut var_guard21_db2: f64 = *var_guard21_db2_slot;
        let mut var_guard21_db3: f64 = *var_guard21_db3_slot;
        let mut var_guard21_db4: f64 = *var_guard21_db4_slot;
        let mut var_guard21_db5: f64 = *var_guard21_db5_slot;
        let mut var_guard21_db6: f64 = *var_guard21_db6_slot;
        let mut var_guard21_db7: f64 = *var_guard21_db7_slot;
        let mut var_guard21_db8: f64 = *var_guard21_db8_slot;
        let mut var_guard21_db9: f64 = *var_guard21_db9_slot;
        let mut var_guard21_dn0: f64 = *var_guard21_dn0_slot;
        let mut var_guard21_dn1: f64 = *var_guard21_dn1_slot;
        let mut var_guard21_dn10: f64 = *var_guard21_dn10_slot;
        let mut var_guard21_dn11: f64 = *var_guard21_dn11_slot;
        let mut var_guard21_dn12: f64 = *var_guard21_dn12_slot;
        let mut var_guard21_dn13: f64 = *var_guard21_dn13_slot;
        let mut var_guard21_dn14: f64 = *var_guard21_dn14_slot;
        let mut var_guard21_dn15: f64 = *var_guard21_dn15_slot;
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
        let mut var_guard21_rdb10: f64 = *var_guard21_rdb10_slot;
        let mut var_guard21_rdb11: f64 = *var_guard21_rdb11_slot;
        let mut var_guard21_rdb12: f64 = *var_guard21_rdb12_slot;
        let mut var_guard21_rdb13: f64 = *var_guard21_rdb13_slot;
        let mut var_guard21_rdb14: f64 = *var_guard21_rdb14_slot;
        let mut var_guard21_rdb15: f64 = *var_guard21_rdb15_slot;
        let mut var_guard21_rdb16: f64 = *var_guard21_rdb16_slot;
        let mut var_guard21_rdb17: f64 = *var_guard21_rdb17_slot;
        let mut var_guard21_rdb18: f64 = *var_guard21_rdb18_slot;
        let mut var_guard21_rdb2: f64 = *var_guard21_rdb2_slot;
        let mut var_guard21_rdb3: f64 = *var_guard21_rdb3_slot;
        let mut var_guard21_rdb4: f64 = *var_guard21_rdb4_slot;
        let mut var_guard21_rdb5: f64 = *var_guard21_rdb5_slot;
        let mut var_guard21_rdb6: f64 = *var_guard21_rdb6_slot;
        let mut var_guard21_rdb7: f64 = *var_guard21_rdb7_slot;
        let mut var_guard21_rdb8: f64 = *var_guard21_rdb8_slot;
        let mut var_guard21_rdb9: f64 = *var_guard21_rdb9_slot;
        let mut var_guard21_rdn0: f64 = *var_guard21_rdn0_slot;
        let mut var_guard21_rdn1: f64 = *var_guard21_rdn1_slot;
        let mut var_guard21_rdn10: f64 = *var_guard21_rdn10_slot;
        let mut var_guard21_rdn11: f64 = *var_guard21_rdn11_slot;
        let mut var_guard21_rdn12: f64 = *var_guard21_rdn12_slot;
        let mut var_guard21_rdn13: f64 = *var_guard21_rdn13_slot;
        let mut var_guard21_rdn14: f64 = *var_guard21_rdn14_slot;
        let mut var_guard21_rdn15: f64 = *var_guard21_rdn15_slot;
        let mut var_guard21_rdn2: f64 = *var_guard21_rdn2_slot;
        let mut var_guard21_rdn3: f64 = *var_guard21_rdn3_slot;
        let mut var_guard21_rdn4: f64 = *var_guard21_rdn4_slot;
        let mut var_guard21_rdn5: f64 = *var_guard21_rdn5_slot;
        let mut var_guard21_rdn6: f64 = *var_guard21_rdn6_slot;
        let mut var_guard21_rdn7: f64 = *var_guard21_rdn7_slot;
        let mut var_guard21_rdn8: f64 = *var_guard21_rdn8_slot;
        let mut var_guard21_rdn9: f64 = *var_guard21_rdn9_slot;
        let mut var_guard21_rv: f64 = *var_guard21_rv_slot;
        let mut var_guard22: f64 = *var_guard22_slot;
        let mut var_guard22_db0: f64 = *var_guard22_db0_slot;
        let mut var_guard22_db1: f64 = *var_guard22_db1_slot;
        let mut var_guard22_db10: f64 = *var_guard22_db10_slot;
        let mut var_guard22_db11: f64 = *var_guard22_db11_slot;
        let mut var_guard22_db12: f64 = *var_guard22_db12_slot;
        let mut var_guard22_db13: f64 = *var_guard22_db13_slot;
        let mut var_guard22_db14: f64 = *var_guard22_db14_slot;
        let mut var_guard22_db15: f64 = *var_guard22_db15_slot;
        let mut var_guard22_db16: f64 = *var_guard22_db16_slot;
        let mut var_guard22_db17: f64 = *var_guard22_db17_slot;
        let mut var_guard22_db18: f64 = *var_guard22_db18_slot;
        let mut var_guard22_db2: f64 = *var_guard22_db2_slot;
        let mut var_guard22_db3: f64 = *var_guard22_db3_slot;
        let mut var_guard22_db4: f64 = *var_guard22_db4_slot;
        let mut var_guard22_db5: f64 = *var_guard22_db5_slot;
        let mut var_guard22_db6: f64 = *var_guard22_db6_slot;
        let mut var_guard22_db7: f64 = *var_guard22_db7_slot;
        let mut var_guard22_db8: f64 = *var_guard22_db8_slot;
        let mut var_guard22_db9: f64 = *var_guard22_db9_slot;
        let mut var_guard22_dn0: f64 = *var_guard22_dn0_slot;
        let mut var_guard22_dn1: f64 = *var_guard22_dn1_slot;
        let mut var_guard22_dn10: f64 = *var_guard22_dn10_slot;
        let mut var_guard22_dn11: f64 = *var_guard22_dn11_slot;
        let mut var_guard22_dn12: f64 = *var_guard22_dn12_slot;
        let mut var_guard22_dn13: f64 = *var_guard22_dn13_slot;
        let mut var_guard22_dn14: f64 = *var_guard22_dn14_slot;
        let mut var_guard22_dn15: f64 = *var_guard22_dn15_slot;
        let mut var_guard22_dn2: f64 = *var_guard22_dn2_slot;
        let mut var_guard22_dn3: f64 = *var_guard22_dn3_slot;
        let mut var_guard22_dn4: f64 = *var_guard22_dn4_slot;
        let mut var_guard22_dn5: f64 = *var_guard22_dn5_slot;
        let mut var_guard22_dn6: f64 = *var_guard22_dn6_slot;
        let mut var_guard22_dn7: f64 = *var_guard22_dn7_slot;
        let mut var_guard22_dn8: f64 = *var_guard22_dn8_slot;
        let mut var_guard22_dn9: f64 = *var_guard22_dn9_slot;
        let mut var_guard22_rdb0: f64 = *var_guard22_rdb0_slot;
        let mut var_guard22_rdb1: f64 = *var_guard22_rdb1_slot;
        let mut var_guard22_rdb10: f64 = *var_guard22_rdb10_slot;
        let mut var_guard22_rdb11: f64 = *var_guard22_rdb11_slot;
        let mut var_guard22_rdb12: f64 = *var_guard22_rdb12_slot;
        let mut var_guard22_rdb13: f64 = *var_guard22_rdb13_slot;
        let mut var_guard22_rdb14: f64 = *var_guard22_rdb14_slot;
        let mut var_guard22_rdb15: f64 = *var_guard22_rdb15_slot;
        let mut var_guard22_rdb16: f64 = *var_guard22_rdb16_slot;
        let mut var_guard22_rdb17: f64 = *var_guard22_rdb17_slot;
        let mut var_guard22_rdb18: f64 = *var_guard22_rdb18_slot;
        let mut var_guard22_rdb2: f64 = *var_guard22_rdb2_slot;
        let mut var_guard22_rdb3: f64 = *var_guard22_rdb3_slot;
        let mut var_guard22_rdb4: f64 = *var_guard22_rdb4_slot;
        let mut var_guard22_rdb5: f64 = *var_guard22_rdb5_slot;
        let mut var_guard22_rdb6: f64 = *var_guard22_rdb6_slot;
        let mut var_guard22_rdb7: f64 = *var_guard22_rdb7_slot;
        let mut var_guard22_rdb8: f64 = *var_guard22_rdb8_slot;
        let mut var_guard22_rdb9: f64 = *var_guard22_rdb9_slot;
        let mut var_guard22_rdn0: f64 = *var_guard22_rdn0_slot;
        let mut var_guard22_rdn1: f64 = *var_guard22_rdn1_slot;
        let mut var_guard22_rdn10: f64 = *var_guard22_rdn10_slot;
        let mut var_guard22_rdn11: f64 = *var_guard22_rdn11_slot;
        let mut var_guard22_rdn12: f64 = *var_guard22_rdn12_slot;
        let mut var_guard22_rdn13: f64 = *var_guard22_rdn13_slot;
        let mut var_guard22_rdn14: f64 = *var_guard22_rdn14_slot;
        let mut var_guard22_rdn15: f64 = *var_guard22_rdn15_slot;
        let mut var_guard22_rdn2: f64 = *var_guard22_rdn2_slot;
        let mut var_guard22_rdn3: f64 = *var_guard22_rdn3_slot;
        let mut var_guard22_rdn4: f64 = *var_guard22_rdn4_slot;
        let mut var_guard22_rdn5: f64 = *var_guard22_rdn5_slot;
        let mut var_guard22_rdn6: f64 = *var_guard22_rdn6_slot;
        let mut var_guard22_rdn7: f64 = *var_guard22_rdn7_slot;
        let mut var_guard22_rdn8: f64 = *var_guard22_rdn8_slot;
        let mut var_guard22_rdn9: f64 = *var_guard22_rdn9_slot;
        let mut var_guard22_rv: f64 = *var_guard22_rv_slot;
        let mut var_guard24: f64 = *var_guard24_slot;
        let mut var_guard24_db0: f64 = *var_guard24_db0_slot;
        let mut var_guard24_db1: f64 = *var_guard24_db1_slot;
        let mut var_guard24_db10: f64 = *var_guard24_db10_slot;
        let mut var_guard24_db11: f64 = *var_guard24_db11_slot;
        let mut var_guard24_db12: f64 = *var_guard24_db12_slot;
        let mut var_guard24_db13: f64 = *var_guard24_db13_slot;
        let mut var_guard24_db14: f64 = *var_guard24_db14_slot;
        let mut var_guard24_db15: f64 = *var_guard24_db15_slot;
        let mut var_guard24_db16: f64 = *var_guard24_db16_slot;
        let mut var_guard24_db17: f64 = *var_guard24_db17_slot;
        let mut var_guard24_db18: f64 = *var_guard24_db18_slot;
        let mut var_guard24_db2: f64 = *var_guard24_db2_slot;
        let mut var_guard24_db3: f64 = *var_guard24_db3_slot;
        let mut var_guard24_db4: f64 = *var_guard24_db4_slot;
        let mut var_guard24_db5: f64 = *var_guard24_db5_slot;
        let mut var_guard24_db6: f64 = *var_guard24_db6_slot;
        let mut var_guard24_db7: f64 = *var_guard24_db7_slot;
        let mut var_guard24_db8: f64 = *var_guard24_db8_slot;
        let mut var_guard24_db9: f64 = *var_guard24_db9_slot;
        let mut var_guard24_dn0: f64 = *var_guard24_dn0_slot;
        let mut var_guard24_dn1: f64 = *var_guard24_dn1_slot;
        let mut var_guard24_dn10: f64 = *var_guard24_dn10_slot;
        let mut var_guard24_dn11: f64 = *var_guard24_dn11_slot;
        let mut var_guard24_dn12: f64 = *var_guard24_dn12_slot;
        let mut var_guard24_dn13: f64 = *var_guard24_dn13_slot;
        let mut var_guard24_dn14: f64 = *var_guard24_dn14_slot;
        let mut var_guard24_dn15: f64 = *var_guard24_dn15_slot;
        let mut var_guard24_dn2: f64 = *var_guard24_dn2_slot;
        let mut var_guard24_dn3: f64 = *var_guard24_dn3_slot;
        let mut var_guard24_dn4: f64 = *var_guard24_dn4_slot;
        let mut var_guard24_dn5: f64 = *var_guard24_dn5_slot;
        let mut var_guard24_dn6: f64 = *var_guard24_dn6_slot;
        let mut var_guard24_dn7: f64 = *var_guard24_dn7_slot;
        let mut var_guard24_dn8: f64 = *var_guard24_dn8_slot;
        let mut var_guard24_dn9: f64 = *var_guard24_dn9_slot;
        let mut var_guard24_rdb0: f64 = *var_guard24_rdb0_slot;
        let mut var_guard24_rdb1: f64 = *var_guard24_rdb1_slot;
        let mut var_guard24_rdb10: f64 = *var_guard24_rdb10_slot;
        let mut var_guard24_rdb11: f64 = *var_guard24_rdb11_slot;
        let mut var_guard24_rdb12: f64 = *var_guard24_rdb12_slot;
        let mut var_guard24_rdb13: f64 = *var_guard24_rdb13_slot;
        let mut var_guard24_rdb14: f64 = *var_guard24_rdb14_slot;
        let mut var_guard24_rdb15: f64 = *var_guard24_rdb15_slot;
        let mut var_guard24_rdb16: f64 = *var_guard24_rdb16_slot;
        let mut var_guard24_rdb17: f64 = *var_guard24_rdb17_slot;
        let mut var_guard24_rdb18: f64 = *var_guard24_rdb18_slot;
        let mut var_guard24_rdb2: f64 = *var_guard24_rdb2_slot;
        let mut var_guard24_rdb3: f64 = *var_guard24_rdb3_slot;
        let mut var_guard24_rdb4: f64 = *var_guard24_rdb4_slot;
        let mut var_guard24_rdb5: f64 = *var_guard24_rdb5_slot;
        let mut var_guard24_rdb6: f64 = *var_guard24_rdb6_slot;
        let mut var_guard24_rdb7: f64 = *var_guard24_rdb7_slot;
        let mut var_guard24_rdb8: f64 = *var_guard24_rdb8_slot;
        let mut var_guard24_rdb9: f64 = *var_guard24_rdb9_slot;
        let mut var_guard24_rdn0: f64 = *var_guard24_rdn0_slot;
        let mut var_guard24_rdn1: f64 = *var_guard24_rdn1_slot;
        let mut var_guard24_rdn10: f64 = *var_guard24_rdn10_slot;
        let mut var_guard24_rdn11: f64 = *var_guard24_rdn11_slot;
        let mut var_guard24_rdn12: f64 = *var_guard24_rdn12_slot;
        let mut var_guard24_rdn13: f64 = *var_guard24_rdn13_slot;
        let mut var_guard24_rdn14: f64 = *var_guard24_rdn14_slot;
        let mut var_guard24_rdn15: f64 = *var_guard24_rdn15_slot;
        let mut var_guard24_rdn2: f64 = *var_guard24_rdn2_slot;
        let mut var_guard24_rdn3: f64 = *var_guard24_rdn3_slot;
        let mut var_guard24_rdn4: f64 = *var_guard24_rdn4_slot;
        let mut var_guard24_rdn5: f64 = *var_guard24_rdn5_slot;
        let mut var_guard24_rdn6: f64 = *var_guard24_rdn6_slot;
        let mut var_guard24_rdn7: f64 = *var_guard24_rdn7_slot;
        let mut var_guard24_rdn8: f64 = *var_guard24_rdn8_slot;
        let mut var_guard24_rdn9: f64 = *var_guard24_rdn9_slot;
        let mut var_guard24_rv: f64 = *var_guard24_rv_slot;
        let mut var_guard25: f64 = *var_guard25_slot;
        let mut var_guard25_db0: f64 = *var_guard25_db0_slot;
        let mut var_guard25_db1: f64 = *var_guard25_db1_slot;
        let mut var_guard25_db10: f64 = *var_guard25_db10_slot;
        let mut var_guard25_db11: f64 = *var_guard25_db11_slot;
        let mut var_guard25_db12: f64 = *var_guard25_db12_slot;
        let mut var_guard25_db13: f64 = *var_guard25_db13_slot;
        let mut var_guard25_db14: f64 = *var_guard25_db14_slot;
        let mut var_guard25_db15: f64 = *var_guard25_db15_slot;
        let mut var_guard25_db16: f64 = *var_guard25_db16_slot;
        let mut var_guard25_db17: f64 = *var_guard25_db17_slot;
        let mut var_guard25_db18: f64 = *var_guard25_db18_slot;
        let mut var_guard25_db2: f64 = *var_guard25_db2_slot;
        let mut var_guard25_db3: f64 = *var_guard25_db3_slot;
        let mut var_guard25_db4: f64 = *var_guard25_db4_slot;
        let mut var_guard25_db5: f64 = *var_guard25_db5_slot;
        let mut var_guard25_db6: f64 = *var_guard25_db6_slot;
        let mut var_guard25_db7: f64 = *var_guard25_db7_slot;
        let mut var_guard25_db8: f64 = *var_guard25_db8_slot;
        let mut var_guard25_db9: f64 = *var_guard25_db9_slot;
        let mut var_guard25_dn0: f64 = *var_guard25_dn0_slot;
        let mut var_guard25_dn1: f64 = *var_guard25_dn1_slot;
        let mut var_guard25_dn10: f64 = *var_guard25_dn10_slot;
        let mut var_guard25_dn11: f64 = *var_guard25_dn11_slot;
        let mut var_guard25_dn12: f64 = *var_guard25_dn12_slot;
        let mut var_guard25_dn13: f64 = *var_guard25_dn13_slot;
        let mut var_guard25_dn14: f64 = *var_guard25_dn14_slot;
        let mut var_guard25_dn15: f64 = *var_guard25_dn15_slot;
        let mut var_guard25_dn2: f64 = *var_guard25_dn2_slot;
        let mut var_guard25_dn3: f64 = *var_guard25_dn3_slot;
        let mut var_guard25_dn4: f64 = *var_guard25_dn4_slot;
        let mut var_guard25_dn5: f64 = *var_guard25_dn5_slot;
        let mut var_guard25_dn6: f64 = *var_guard25_dn6_slot;
        let mut var_guard25_dn7: f64 = *var_guard25_dn7_slot;
        let mut var_guard25_dn8: f64 = *var_guard25_dn8_slot;
        let mut var_guard25_dn9: f64 = *var_guard25_dn9_slot;
        let mut var_guard25_rdb0: f64 = *var_guard25_rdb0_slot;
        let mut var_guard25_rdb1: f64 = *var_guard25_rdb1_slot;
        let mut var_guard25_rdb10: f64 = *var_guard25_rdb10_slot;
        let mut var_guard25_rdb11: f64 = *var_guard25_rdb11_slot;
        let mut var_guard25_rdb12: f64 = *var_guard25_rdb12_slot;
        let mut var_guard25_rdb13: f64 = *var_guard25_rdb13_slot;
        let mut var_guard25_rdb14: f64 = *var_guard25_rdb14_slot;
        let mut var_guard25_rdb15: f64 = *var_guard25_rdb15_slot;
        let mut var_guard25_rdb16: f64 = *var_guard25_rdb16_slot;
        let mut var_guard25_rdb17: f64 = *var_guard25_rdb17_slot;
        let mut var_guard25_rdb18: f64 = *var_guard25_rdb18_slot;
        let mut var_guard25_rdb2: f64 = *var_guard25_rdb2_slot;
        let mut var_guard25_rdb3: f64 = *var_guard25_rdb3_slot;
        let mut var_guard25_rdb4: f64 = *var_guard25_rdb4_slot;
        let mut var_guard25_rdb5: f64 = *var_guard25_rdb5_slot;
        let mut var_guard25_rdb6: f64 = *var_guard25_rdb6_slot;
        let mut var_guard25_rdb7: f64 = *var_guard25_rdb7_slot;
        let mut var_guard25_rdb8: f64 = *var_guard25_rdb8_slot;
        let mut var_guard25_rdb9: f64 = *var_guard25_rdb9_slot;
        let mut var_guard25_rdn0: f64 = *var_guard25_rdn0_slot;
        let mut var_guard25_rdn1: f64 = *var_guard25_rdn1_slot;
        let mut var_guard25_rdn10: f64 = *var_guard25_rdn10_slot;
        let mut var_guard25_rdn11: f64 = *var_guard25_rdn11_slot;
        let mut var_guard25_rdn12: f64 = *var_guard25_rdn12_slot;
        let mut var_guard25_rdn13: f64 = *var_guard25_rdn13_slot;
        let mut var_guard25_rdn14: f64 = *var_guard25_rdn14_slot;
        let mut var_guard25_rdn15: f64 = *var_guard25_rdn15_slot;
        let mut var_guard25_rdn2: f64 = *var_guard25_rdn2_slot;
        let mut var_guard25_rdn3: f64 = *var_guard25_rdn3_slot;
        let mut var_guard25_rdn4: f64 = *var_guard25_rdn4_slot;
        let mut var_guard25_rdn5: f64 = *var_guard25_rdn5_slot;
        let mut var_guard25_rdn6: f64 = *var_guard25_rdn6_slot;
        let mut var_guard25_rdn7: f64 = *var_guard25_rdn7_slot;
        let mut var_guard25_rdn8: f64 = *var_guard25_rdn8_slot;
        let mut var_guard25_rdn9: f64 = *var_guard25_rdn9_slot;
        let mut var_guard25_rv: f64 = *var_guard25_rv_slot;
        let mut var_guard26: f64 = *var_guard26_slot;
        let mut var_guard26_db0: f64 = *var_guard26_db0_slot;
        let mut var_guard26_db1: f64 = *var_guard26_db1_slot;
        let mut var_guard26_db10: f64 = *var_guard26_db10_slot;
        let mut var_guard26_db11: f64 = *var_guard26_db11_slot;
        let mut var_guard26_db12: f64 = *var_guard26_db12_slot;
        let mut var_guard26_db13: f64 = *var_guard26_db13_slot;
        let mut var_guard26_db14: f64 = *var_guard26_db14_slot;
        let mut var_guard26_db15: f64 = *var_guard26_db15_slot;
        let mut var_guard26_db16: f64 = *var_guard26_db16_slot;
        let mut var_guard26_db17: f64 = *var_guard26_db17_slot;
        let mut var_guard26_db18: f64 = *var_guard26_db18_slot;
        let mut var_guard26_db2: f64 = *var_guard26_db2_slot;
        let mut var_guard26_db3: f64 = *var_guard26_db3_slot;
        let mut var_guard26_db4: f64 = *var_guard26_db4_slot;
        let mut var_guard26_db5: f64 = *var_guard26_db5_slot;
        let mut var_guard26_db6: f64 = *var_guard26_db6_slot;
        let mut var_guard26_db7: f64 = *var_guard26_db7_slot;
        let mut var_guard26_db8: f64 = *var_guard26_db8_slot;
        let mut var_guard26_db9: f64 = *var_guard26_db9_slot;
        let mut var_guard26_dn0: f64 = *var_guard26_dn0_slot;
        let mut var_guard26_dn1: f64 = *var_guard26_dn1_slot;
        let mut var_guard26_dn10: f64 = *var_guard26_dn10_slot;
        let mut var_guard26_dn11: f64 = *var_guard26_dn11_slot;
        let mut var_guard26_dn12: f64 = *var_guard26_dn12_slot;
        let mut var_guard26_dn13: f64 = *var_guard26_dn13_slot;
        let mut var_guard26_dn14: f64 = *var_guard26_dn14_slot;
        let mut var_guard26_dn15: f64 = *var_guard26_dn15_slot;
        let mut var_guard26_dn2: f64 = *var_guard26_dn2_slot;
        let mut var_guard26_dn3: f64 = *var_guard26_dn3_slot;
        let mut var_guard26_dn4: f64 = *var_guard26_dn4_slot;
        let mut var_guard26_dn5: f64 = *var_guard26_dn5_slot;
        let mut var_guard26_dn6: f64 = *var_guard26_dn6_slot;
        let mut var_guard26_dn7: f64 = *var_guard26_dn7_slot;
        let mut var_guard26_dn8: f64 = *var_guard26_dn8_slot;
        let mut var_guard26_dn9: f64 = *var_guard26_dn9_slot;
        let mut var_guard26_rdb0: f64 = *var_guard26_rdb0_slot;
        let mut var_guard26_rdb1: f64 = *var_guard26_rdb1_slot;
        let mut var_guard26_rdb10: f64 = *var_guard26_rdb10_slot;
        let mut var_guard26_rdb11: f64 = *var_guard26_rdb11_slot;
        let mut var_guard26_rdb12: f64 = *var_guard26_rdb12_slot;
        let mut var_guard26_rdb13: f64 = *var_guard26_rdb13_slot;
        let mut var_guard26_rdb14: f64 = *var_guard26_rdb14_slot;
        let mut var_guard26_rdb15: f64 = *var_guard26_rdb15_slot;
        let mut var_guard26_rdb16: f64 = *var_guard26_rdb16_slot;
        let mut var_guard26_rdb17: f64 = *var_guard26_rdb17_slot;
        let mut var_guard26_rdb18: f64 = *var_guard26_rdb18_slot;
        let mut var_guard26_rdb2: f64 = *var_guard26_rdb2_slot;
        let mut var_guard26_rdb3: f64 = *var_guard26_rdb3_slot;
        let mut var_guard26_rdb4: f64 = *var_guard26_rdb4_slot;
        let mut var_guard26_rdb5: f64 = *var_guard26_rdb5_slot;
        let mut var_guard26_rdb6: f64 = *var_guard26_rdb6_slot;
        let mut var_guard26_rdb7: f64 = *var_guard26_rdb7_slot;
        let mut var_guard26_rdb8: f64 = *var_guard26_rdb8_slot;
        let mut var_guard26_rdb9: f64 = *var_guard26_rdb9_slot;
        let mut var_guard26_rdn0: f64 = *var_guard26_rdn0_slot;
        let mut var_guard26_rdn1: f64 = *var_guard26_rdn1_slot;
        let mut var_guard26_rdn10: f64 = *var_guard26_rdn10_slot;
        let mut var_guard26_rdn11: f64 = *var_guard26_rdn11_slot;
        let mut var_guard26_rdn12: f64 = *var_guard26_rdn12_slot;
        let mut var_guard26_rdn13: f64 = *var_guard26_rdn13_slot;
        let mut var_guard26_rdn14: f64 = *var_guard26_rdn14_slot;
        let mut var_guard26_rdn15: f64 = *var_guard26_rdn15_slot;
        let mut var_guard26_rdn2: f64 = *var_guard26_rdn2_slot;
        let mut var_guard26_rdn3: f64 = *var_guard26_rdn3_slot;
        let mut var_guard26_rdn4: f64 = *var_guard26_rdn4_slot;
        let mut var_guard26_rdn5: f64 = *var_guard26_rdn5_slot;
        let mut var_guard26_rdn6: f64 = *var_guard26_rdn6_slot;
        let mut var_guard26_rdn7: f64 = *var_guard26_rdn7_slot;
        let mut var_guard26_rdn8: f64 = *var_guard26_rdn8_slot;
        let mut var_guard26_rdn9: f64 = *var_guard26_rdn9_slot;
        let mut var_guard26_rv: f64 = *var_guard26_rv_slot;

        let assign1580_e2025: f64 = if p.p6 == 2.0 { 1.0 } else { 0.0 };
        var_guard16 = assign1580_e2025;
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
        var_guard16_dn10 = 0.0;
        var_guard16_dn11 = 0.0;
        var_guard16_dn12 = 0.0;
        var_guard16_dn13 = 0.0;
        var_guard16_dn14 = 0.0;
        var_guard16_dn15 = 0.0;
        var_guard16_db0 = 0.0;
        var_guard16_db1 = 0.0;
        var_guard16_db2 = 0.0;
        var_guard16_db3 = 0.0;
        var_guard16_db4 = 0.0;
        var_guard16_db5 = 0.0;
        var_guard16_db6 = 0.0;
        var_guard16_db7 = 0.0;
        var_guard16_db8 = 0.0;
        var_guard16_db9 = 0.0;
        var_guard16_db10 = 0.0;
        var_guard16_db11 = 0.0;
        var_guard16_db12 = 0.0;
        var_guard16_db13 = 0.0;
        var_guard16_db14 = 0.0;
        var_guard16_db15 = 0.0;
        var_guard16_db16 = 0.0;
        var_guard16_db17 = 0.0;
        var_guard16_db18 = 0.0;
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
        var_guard16_rdn10 = 0.0;
        var_guard16_rdn11 = 0.0;
        var_guard16_rdn12 = 0.0;
        var_guard16_rdn13 = 0.0;
        var_guard16_rdn14 = 0.0;
        var_guard16_rdn15 = 0.0;
        var_guard16_rdb0 = 0.0;
        var_guard16_rdb1 = 0.0;
        var_guard16_rdb2 = 0.0;
        var_guard16_rdb3 = 0.0;
        var_guard16_rdb4 = 0.0;
        var_guard16_rdb5 = 0.0;
        var_guard16_rdb6 = 0.0;
        var_guard16_rdb7 = 0.0;
        var_guard16_rdb8 = 0.0;
        var_guard16_rdb9 = 0.0;
        var_guard16_rdb10 = 0.0;
        var_guard16_rdb11 = 0.0;
        var_guard16_rdb12 = 0.0;
        var_guard16_rdb13 = 0.0;
        var_guard16_rdb14 = 0.0;
        var_guard16_rdb15 = 0.0;
        var_guard16_rdb16 = 0.0;
        var_guard16_rdb17 = 0.0;
        var_guard16_rdb18 = 0.0;

        let assign1630_e2040: f64 = if p.p42 > 0.0 { 1.0 } else { 0.0 };
        var_guard21 = assign1630_e2040;
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
        var_guard21_dn10 = 0.0;
        var_guard21_dn11 = 0.0;
        var_guard21_dn12 = 0.0;
        var_guard21_dn13 = 0.0;
        var_guard21_dn14 = 0.0;
        var_guard21_dn15 = 0.0;
        var_guard21_db0 = 0.0;
        var_guard21_db1 = 0.0;
        var_guard21_db2 = 0.0;
        var_guard21_db3 = 0.0;
        var_guard21_db4 = 0.0;
        var_guard21_db5 = 0.0;
        var_guard21_db6 = 0.0;
        var_guard21_db7 = 0.0;
        var_guard21_db8 = 0.0;
        var_guard21_db9 = 0.0;
        var_guard21_db10 = 0.0;
        var_guard21_db11 = 0.0;
        var_guard21_db12 = 0.0;
        var_guard21_db13 = 0.0;
        var_guard21_db14 = 0.0;
        var_guard21_db15 = 0.0;
        var_guard21_db16 = 0.0;
        var_guard21_db17 = 0.0;
        var_guard21_db18 = 0.0;
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
        var_guard21_rdn10 = 0.0;
        var_guard21_rdn11 = 0.0;
        var_guard21_rdn12 = 0.0;
        var_guard21_rdn13 = 0.0;
        var_guard21_rdn14 = 0.0;
        var_guard21_rdn15 = 0.0;
        var_guard21_rdb0 = 0.0;
        var_guard21_rdb1 = 0.0;
        var_guard21_rdb2 = 0.0;
        var_guard21_rdb3 = 0.0;
        var_guard21_rdb4 = 0.0;
        var_guard21_rdb5 = 0.0;
        var_guard21_rdb6 = 0.0;
        var_guard21_rdb7 = 0.0;
        var_guard21_rdb8 = 0.0;
        var_guard21_rdb9 = 0.0;
        var_guard21_rdb10 = 0.0;
        var_guard21_rdb11 = 0.0;
        var_guard21_rdb12 = 0.0;
        var_guard21_rdb13 = 0.0;
        var_guard21_rdb14 = 0.0;
        var_guard21_rdb15 = 0.0;
        var_guard21_rdb16 = 0.0;
        var_guard21_rdb17 = 0.0;
        var_guard21_rdb18 = 0.0;

        let assign1640_e2043: f64 = if p.p50 > 0.0 { 1.0 } else { 0.0 };
        var_guard22 = assign1640_e2043;
        var_guard22_dn0 = 0.0;
        var_guard22_dn1 = 0.0;
        var_guard22_dn2 = 0.0;
        var_guard22_dn3 = 0.0;
        var_guard22_dn4 = 0.0;
        var_guard22_dn5 = 0.0;
        var_guard22_dn6 = 0.0;
        var_guard22_dn7 = 0.0;
        var_guard22_dn8 = 0.0;
        var_guard22_dn9 = 0.0;
        var_guard22_dn10 = 0.0;
        var_guard22_dn11 = 0.0;
        var_guard22_dn12 = 0.0;
        var_guard22_dn13 = 0.0;
        var_guard22_dn14 = 0.0;
        var_guard22_dn15 = 0.0;
        var_guard22_db0 = 0.0;
        var_guard22_db1 = 0.0;
        var_guard22_db2 = 0.0;
        var_guard22_db3 = 0.0;
        var_guard22_db4 = 0.0;
        var_guard22_db5 = 0.0;
        var_guard22_db6 = 0.0;
        var_guard22_db7 = 0.0;
        var_guard22_db8 = 0.0;
        var_guard22_db9 = 0.0;
        var_guard22_db10 = 0.0;
        var_guard22_db11 = 0.0;
        var_guard22_db12 = 0.0;
        var_guard22_db13 = 0.0;
        var_guard22_db14 = 0.0;
        var_guard22_db15 = 0.0;
        var_guard22_db16 = 0.0;
        var_guard22_db17 = 0.0;
        var_guard22_db18 = 0.0;
        var_guard22_rv = 0.0;
        var_guard22_rdn0 = 0.0;
        var_guard22_rdn1 = 0.0;
        var_guard22_rdn2 = 0.0;
        var_guard22_rdn3 = 0.0;
        var_guard22_rdn4 = 0.0;
        var_guard22_rdn5 = 0.0;
        var_guard22_rdn6 = 0.0;
        var_guard22_rdn7 = 0.0;
        var_guard22_rdn8 = 0.0;
        var_guard22_rdn9 = 0.0;
        var_guard22_rdn10 = 0.0;
        var_guard22_rdn11 = 0.0;
        var_guard22_rdn12 = 0.0;
        var_guard22_rdn13 = 0.0;
        var_guard22_rdn14 = 0.0;
        var_guard22_rdn15 = 0.0;
        var_guard22_rdb0 = 0.0;
        var_guard22_rdb1 = 0.0;
        var_guard22_rdb2 = 0.0;
        var_guard22_rdb3 = 0.0;
        var_guard22_rdb4 = 0.0;
        var_guard22_rdb5 = 0.0;
        var_guard22_rdb6 = 0.0;
        var_guard22_rdb7 = 0.0;
        var_guard22_rdb8 = 0.0;
        var_guard22_rdb9 = 0.0;
        var_guard22_rdb10 = 0.0;
        var_guard22_rdb11 = 0.0;
        var_guard22_rdb12 = 0.0;
        var_guard22_rdb13 = 0.0;
        var_guard22_rdb14 = 0.0;
        var_guard22_rdb15 = 0.0;
        var_guard22_rdb16 = 0.0;
        var_guard22_rdb17 = 0.0;
        var_guard22_rdb18 = 0.0;

        let assign1660_e2053: f64 = if ((p.p43 > 0.0) || (p.p44 > 0.0)) { 1.0 } else { 0.0 };
        var_guard24 = assign1660_e2053;
        var_guard24_dn0 = 0.0;
        var_guard24_dn1 = 0.0;
        var_guard24_dn2 = 0.0;
        var_guard24_dn3 = 0.0;
        var_guard24_dn4 = 0.0;
        var_guard24_dn5 = 0.0;
        var_guard24_dn6 = 0.0;
        var_guard24_dn7 = 0.0;
        var_guard24_dn8 = 0.0;
        var_guard24_dn9 = 0.0;
        var_guard24_dn10 = 0.0;
        var_guard24_dn11 = 0.0;
        var_guard24_dn12 = 0.0;
        var_guard24_dn13 = 0.0;
        var_guard24_dn14 = 0.0;
        var_guard24_dn15 = 0.0;
        var_guard24_db0 = 0.0;
        var_guard24_db1 = 0.0;
        var_guard24_db2 = 0.0;
        var_guard24_db3 = 0.0;
        var_guard24_db4 = 0.0;
        var_guard24_db5 = 0.0;
        var_guard24_db6 = 0.0;
        var_guard24_db7 = 0.0;
        var_guard24_db8 = 0.0;
        var_guard24_db9 = 0.0;
        var_guard24_db10 = 0.0;
        var_guard24_db11 = 0.0;
        var_guard24_db12 = 0.0;
        var_guard24_db13 = 0.0;
        var_guard24_db14 = 0.0;
        var_guard24_db15 = 0.0;
        var_guard24_db16 = 0.0;
        var_guard24_db17 = 0.0;
        var_guard24_db18 = 0.0;
        var_guard24_rv = 0.0;
        var_guard24_rdn0 = 0.0;
        var_guard24_rdn1 = 0.0;
        var_guard24_rdn2 = 0.0;
        var_guard24_rdn3 = 0.0;
        var_guard24_rdn4 = 0.0;
        var_guard24_rdn5 = 0.0;
        var_guard24_rdn6 = 0.0;
        var_guard24_rdn7 = 0.0;
        var_guard24_rdn8 = 0.0;
        var_guard24_rdn9 = 0.0;
        var_guard24_rdn10 = 0.0;
        var_guard24_rdn11 = 0.0;
        var_guard24_rdn12 = 0.0;
        var_guard24_rdn13 = 0.0;
        var_guard24_rdn14 = 0.0;
        var_guard24_rdn15 = 0.0;
        var_guard24_rdb0 = 0.0;
        var_guard24_rdb1 = 0.0;
        var_guard24_rdb2 = 0.0;
        var_guard24_rdb3 = 0.0;
        var_guard24_rdb4 = 0.0;
        var_guard24_rdb5 = 0.0;
        var_guard24_rdb6 = 0.0;
        var_guard24_rdb7 = 0.0;
        var_guard24_rdb8 = 0.0;
        var_guard24_rdb9 = 0.0;
        var_guard24_rdb10 = 0.0;
        var_guard24_rdb11 = 0.0;
        var_guard24_rdb12 = 0.0;
        var_guard24_rdb13 = 0.0;
        var_guard24_rdb14 = 0.0;
        var_guard24_rdb15 = 0.0;
        var_guard24_rdb16 = 0.0;
        var_guard24_rdb17 = 0.0;
        var_guard24_rdb18 = 0.0;

        let assign1670_e2056: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        var_guard25 = assign1670_e2056;
        var_guard25_dn0 = 0.0;
        var_guard25_dn1 = 0.0;
        var_guard25_dn2 = 0.0;
        var_guard25_dn3 = 0.0;
        var_guard25_dn4 = 0.0;
        var_guard25_dn5 = 0.0;
        var_guard25_dn6 = 0.0;
        var_guard25_dn7 = 0.0;
        var_guard25_dn8 = 0.0;
        var_guard25_dn9 = 0.0;
        var_guard25_dn10 = 0.0;
        var_guard25_dn11 = 0.0;
        var_guard25_dn12 = 0.0;
        var_guard25_dn13 = 0.0;
        var_guard25_dn14 = 0.0;
        var_guard25_dn15 = 0.0;
        var_guard25_db0 = 0.0;
        var_guard25_db1 = 0.0;
        var_guard25_db2 = 0.0;
        var_guard25_db3 = 0.0;
        var_guard25_db4 = 0.0;
        var_guard25_db5 = 0.0;
        var_guard25_db6 = 0.0;
        var_guard25_db7 = 0.0;
        var_guard25_db8 = 0.0;
        var_guard25_db9 = 0.0;
        var_guard25_db10 = 0.0;
        var_guard25_db11 = 0.0;
        var_guard25_db12 = 0.0;
        var_guard25_db13 = 0.0;
        var_guard25_db14 = 0.0;
        var_guard25_db15 = 0.0;
        var_guard25_db16 = 0.0;
        var_guard25_db17 = 0.0;
        var_guard25_db18 = 0.0;
        var_guard25_rv = 0.0;
        var_guard25_rdn0 = 0.0;
        var_guard25_rdn1 = 0.0;
        var_guard25_rdn2 = 0.0;
        var_guard25_rdn3 = 0.0;
        var_guard25_rdn4 = 0.0;
        var_guard25_rdn5 = 0.0;
        var_guard25_rdn6 = 0.0;
        var_guard25_rdn7 = 0.0;
        var_guard25_rdn8 = 0.0;
        var_guard25_rdn9 = 0.0;
        var_guard25_rdn10 = 0.0;
        var_guard25_rdn11 = 0.0;
        var_guard25_rdn12 = 0.0;
        var_guard25_rdn13 = 0.0;
        var_guard25_rdn14 = 0.0;
        var_guard25_rdn15 = 0.0;
        var_guard25_rdb0 = 0.0;
        var_guard25_rdb1 = 0.0;
        var_guard25_rdb2 = 0.0;
        var_guard25_rdb3 = 0.0;
        var_guard25_rdb4 = 0.0;
        var_guard25_rdb5 = 0.0;
        var_guard25_rdb6 = 0.0;
        var_guard25_rdb7 = 0.0;
        var_guard25_rdb8 = 0.0;
        var_guard25_rdb9 = 0.0;
        var_guard25_rdb10 = 0.0;
        var_guard25_rdb11 = 0.0;
        var_guard25_rdb12 = 0.0;
        var_guard25_rdb13 = 0.0;
        var_guard25_rdb14 = 0.0;
        var_guard25_rdb15 = 0.0;
        var_guard25_rdb16 = 0.0;
        var_guard25_rdb17 = 0.0;
        var_guard25_rdb18 = 0.0;

        let assign1680_e2059: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        var_guard26 = assign1680_e2059;
        var_guard26_dn0 = 0.0;
        var_guard26_dn1 = 0.0;
        var_guard26_dn2 = 0.0;
        var_guard26_dn3 = 0.0;
        var_guard26_dn4 = 0.0;
        var_guard26_dn5 = 0.0;
        var_guard26_dn6 = 0.0;
        var_guard26_dn7 = 0.0;
        var_guard26_dn8 = 0.0;
        var_guard26_dn9 = 0.0;
        var_guard26_dn10 = 0.0;
        var_guard26_dn11 = 0.0;
        var_guard26_dn12 = 0.0;
        var_guard26_dn13 = 0.0;
        var_guard26_dn14 = 0.0;
        var_guard26_dn15 = 0.0;
        var_guard26_db0 = 0.0;
        var_guard26_db1 = 0.0;
        var_guard26_db2 = 0.0;
        var_guard26_db3 = 0.0;
        var_guard26_db4 = 0.0;
        var_guard26_db5 = 0.0;
        var_guard26_db6 = 0.0;
        var_guard26_db7 = 0.0;
        var_guard26_db8 = 0.0;
        var_guard26_db9 = 0.0;
        var_guard26_db10 = 0.0;
        var_guard26_db11 = 0.0;
        var_guard26_db12 = 0.0;
        var_guard26_db13 = 0.0;
        var_guard26_db14 = 0.0;
        var_guard26_db15 = 0.0;
        var_guard26_db16 = 0.0;
        var_guard26_db17 = 0.0;
        var_guard26_db18 = 0.0;
        var_guard26_rv = 0.0;
        var_guard26_rdn0 = 0.0;
        var_guard26_rdn1 = 0.0;
        var_guard26_rdn2 = 0.0;
        var_guard26_rdn3 = 0.0;
        var_guard26_rdn4 = 0.0;
        var_guard26_rdn5 = 0.0;
        var_guard26_rdn6 = 0.0;
        var_guard26_rdn7 = 0.0;
        var_guard26_rdn8 = 0.0;
        var_guard26_rdn9 = 0.0;
        var_guard26_rdn10 = 0.0;
        var_guard26_rdn11 = 0.0;
        var_guard26_rdn12 = 0.0;
        var_guard26_rdn13 = 0.0;
        var_guard26_rdn14 = 0.0;
        var_guard26_rdn15 = 0.0;
        var_guard26_rdb0 = 0.0;
        var_guard26_rdb1 = 0.0;
        var_guard26_rdb2 = 0.0;
        var_guard26_rdb3 = 0.0;
        var_guard26_rdb4 = 0.0;
        var_guard26_rdb5 = 0.0;
        var_guard26_rdb6 = 0.0;
        var_guard26_rdb7 = 0.0;
        var_guard26_rdb8 = 0.0;
        var_guard26_rdb9 = 0.0;
        var_guard26_rdb10 = 0.0;
        var_guard26_rdb11 = 0.0;
        var_guard26_rdb12 = 0.0;
        var_guard26_rdb13 = 0.0;
        var_guard26_rdb14 = 0.0;
        var_guard26_rdb15 = 0.0;
        var_guard26_rdb16 = 0.0;
        var_guard26_rdb17 = 0.0;
        var_guard26_rdb18 = 0.0;

        *var_guard16_slot = var_guard16;
        *var_guard16_db0_slot = var_guard16_db0;
        *var_guard16_db1_slot = var_guard16_db1;
        *var_guard16_db10_slot = var_guard16_db10;
        *var_guard16_db11_slot = var_guard16_db11;
        *var_guard16_db12_slot = var_guard16_db12;
        *var_guard16_db13_slot = var_guard16_db13;
        *var_guard16_db14_slot = var_guard16_db14;
        *var_guard16_db15_slot = var_guard16_db15;
        *var_guard16_db16_slot = var_guard16_db16;
        *var_guard16_db17_slot = var_guard16_db17;
        *var_guard16_db18_slot = var_guard16_db18;
        *var_guard16_db2_slot = var_guard16_db2;
        *var_guard16_db3_slot = var_guard16_db3;
        *var_guard16_db4_slot = var_guard16_db4;
        *var_guard16_db5_slot = var_guard16_db5;
        *var_guard16_db6_slot = var_guard16_db6;
        *var_guard16_db7_slot = var_guard16_db7;
        *var_guard16_db8_slot = var_guard16_db8;
        *var_guard16_db9_slot = var_guard16_db9;
        *var_guard16_dn0_slot = var_guard16_dn0;
        *var_guard16_dn1_slot = var_guard16_dn1;
        *var_guard16_dn10_slot = var_guard16_dn10;
        *var_guard16_dn11_slot = var_guard16_dn11;
        *var_guard16_dn12_slot = var_guard16_dn12;
        *var_guard16_dn13_slot = var_guard16_dn13;
        *var_guard16_dn14_slot = var_guard16_dn14;
        *var_guard16_dn15_slot = var_guard16_dn15;
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
        *var_guard16_rdb10_slot = var_guard16_rdb10;
        *var_guard16_rdb11_slot = var_guard16_rdb11;
        *var_guard16_rdb12_slot = var_guard16_rdb12;
        *var_guard16_rdb13_slot = var_guard16_rdb13;
        *var_guard16_rdb14_slot = var_guard16_rdb14;
        *var_guard16_rdb15_slot = var_guard16_rdb15;
        *var_guard16_rdb16_slot = var_guard16_rdb16;
        *var_guard16_rdb17_slot = var_guard16_rdb17;
        *var_guard16_rdb18_slot = var_guard16_rdb18;
        *var_guard16_rdb2_slot = var_guard16_rdb2;
        *var_guard16_rdb3_slot = var_guard16_rdb3;
        *var_guard16_rdb4_slot = var_guard16_rdb4;
        *var_guard16_rdb5_slot = var_guard16_rdb5;
        *var_guard16_rdb6_slot = var_guard16_rdb6;
        *var_guard16_rdb7_slot = var_guard16_rdb7;
        *var_guard16_rdb8_slot = var_guard16_rdb8;
        *var_guard16_rdb9_slot = var_guard16_rdb9;
        *var_guard16_rdn0_slot = var_guard16_rdn0;
        *var_guard16_rdn1_slot = var_guard16_rdn1;
        *var_guard16_rdn10_slot = var_guard16_rdn10;
        *var_guard16_rdn11_slot = var_guard16_rdn11;
        *var_guard16_rdn12_slot = var_guard16_rdn12;
        *var_guard16_rdn13_slot = var_guard16_rdn13;
        *var_guard16_rdn14_slot = var_guard16_rdn14;
        *var_guard16_rdn15_slot = var_guard16_rdn15;
        *var_guard16_rdn2_slot = var_guard16_rdn2;
        *var_guard16_rdn3_slot = var_guard16_rdn3;
        *var_guard16_rdn4_slot = var_guard16_rdn4;
        *var_guard16_rdn5_slot = var_guard16_rdn5;
        *var_guard16_rdn6_slot = var_guard16_rdn6;
        *var_guard16_rdn7_slot = var_guard16_rdn7;
        *var_guard16_rdn8_slot = var_guard16_rdn8;
        *var_guard16_rdn9_slot = var_guard16_rdn9;
        *var_guard16_rv_slot = var_guard16_rv;
        *var_guard21_slot = var_guard21;
        *var_guard21_db0_slot = var_guard21_db0;
        *var_guard21_db1_slot = var_guard21_db1;
        *var_guard21_db10_slot = var_guard21_db10;
        *var_guard21_db11_slot = var_guard21_db11;
        *var_guard21_db12_slot = var_guard21_db12;
        *var_guard21_db13_slot = var_guard21_db13;
        *var_guard21_db14_slot = var_guard21_db14;
        *var_guard21_db15_slot = var_guard21_db15;
        *var_guard21_db16_slot = var_guard21_db16;
        *var_guard21_db17_slot = var_guard21_db17;
        *var_guard21_db18_slot = var_guard21_db18;
        *var_guard21_db2_slot = var_guard21_db2;
        *var_guard21_db3_slot = var_guard21_db3;
        *var_guard21_db4_slot = var_guard21_db4;
        *var_guard21_db5_slot = var_guard21_db5;
        *var_guard21_db6_slot = var_guard21_db6;
        *var_guard21_db7_slot = var_guard21_db7;
        *var_guard21_db8_slot = var_guard21_db8;
        *var_guard21_db9_slot = var_guard21_db9;
        *var_guard21_dn0_slot = var_guard21_dn0;
        *var_guard21_dn1_slot = var_guard21_dn1;
        *var_guard21_dn10_slot = var_guard21_dn10;
        *var_guard21_dn11_slot = var_guard21_dn11;
        *var_guard21_dn12_slot = var_guard21_dn12;
        *var_guard21_dn13_slot = var_guard21_dn13;
        *var_guard21_dn14_slot = var_guard21_dn14;
        *var_guard21_dn15_slot = var_guard21_dn15;
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
        *var_guard21_rdb10_slot = var_guard21_rdb10;
        *var_guard21_rdb11_slot = var_guard21_rdb11;
        *var_guard21_rdb12_slot = var_guard21_rdb12;
        *var_guard21_rdb13_slot = var_guard21_rdb13;
        *var_guard21_rdb14_slot = var_guard21_rdb14;
        *var_guard21_rdb15_slot = var_guard21_rdb15;
        *var_guard21_rdb16_slot = var_guard21_rdb16;
        *var_guard21_rdb17_slot = var_guard21_rdb17;
        *var_guard21_rdb18_slot = var_guard21_rdb18;
        *var_guard21_rdb2_slot = var_guard21_rdb2;
        *var_guard21_rdb3_slot = var_guard21_rdb3;
        *var_guard21_rdb4_slot = var_guard21_rdb4;
        *var_guard21_rdb5_slot = var_guard21_rdb5;
        *var_guard21_rdb6_slot = var_guard21_rdb6;
        *var_guard21_rdb7_slot = var_guard21_rdb7;
        *var_guard21_rdb8_slot = var_guard21_rdb8;
        *var_guard21_rdb9_slot = var_guard21_rdb9;
        *var_guard21_rdn0_slot = var_guard21_rdn0;
        *var_guard21_rdn1_slot = var_guard21_rdn1;
        *var_guard21_rdn10_slot = var_guard21_rdn10;
        *var_guard21_rdn11_slot = var_guard21_rdn11;
        *var_guard21_rdn12_slot = var_guard21_rdn12;
        *var_guard21_rdn13_slot = var_guard21_rdn13;
        *var_guard21_rdn14_slot = var_guard21_rdn14;
        *var_guard21_rdn15_slot = var_guard21_rdn15;
        *var_guard21_rdn2_slot = var_guard21_rdn2;
        *var_guard21_rdn3_slot = var_guard21_rdn3;
        *var_guard21_rdn4_slot = var_guard21_rdn4;
        *var_guard21_rdn5_slot = var_guard21_rdn5;
        *var_guard21_rdn6_slot = var_guard21_rdn6;
        *var_guard21_rdn7_slot = var_guard21_rdn7;
        *var_guard21_rdn8_slot = var_guard21_rdn8;
        *var_guard21_rdn9_slot = var_guard21_rdn9;
        *var_guard21_rv_slot = var_guard21_rv;
        *var_guard22_slot = var_guard22;
        *var_guard22_db0_slot = var_guard22_db0;
        *var_guard22_db1_slot = var_guard22_db1;
        *var_guard22_db10_slot = var_guard22_db10;
        *var_guard22_db11_slot = var_guard22_db11;
        *var_guard22_db12_slot = var_guard22_db12;
        *var_guard22_db13_slot = var_guard22_db13;
        *var_guard22_db14_slot = var_guard22_db14;
        *var_guard22_db15_slot = var_guard22_db15;
        *var_guard22_db16_slot = var_guard22_db16;
        *var_guard22_db17_slot = var_guard22_db17;
        *var_guard22_db18_slot = var_guard22_db18;
        *var_guard22_db2_slot = var_guard22_db2;
        *var_guard22_db3_slot = var_guard22_db3;
        *var_guard22_db4_slot = var_guard22_db4;
        *var_guard22_db5_slot = var_guard22_db5;
        *var_guard22_db6_slot = var_guard22_db6;
        *var_guard22_db7_slot = var_guard22_db7;
        *var_guard22_db8_slot = var_guard22_db8;
        *var_guard22_db9_slot = var_guard22_db9;
        *var_guard22_dn0_slot = var_guard22_dn0;
        *var_guard22_dn1_slot = var_guard22_dn1;
        *var_guard22_dn10_slot = var_guard22_dn10;
        *var_guard22_dn11_slot = var_guard22_dn11;
        *var_guard22_dn12_slot = var_guard22_dn12;
        *var_guard22_dn13_slot = var_guard22_dn13;
        *var_guard22_dn14_slot = var_guard22_dn14;
        *var_guard22_dn15_slot = var_guard22_dn15;
        *var_guard22_dn2_slot = var_guard22_dn2;
        *var_guard22_dn3_slot = var_guard22_dn3;
        *var_guard22_dn4_slot = var_guard22_dn4;
        *var_guard22_dn5_slot = var_guard22_dn5;
        *var_guard22_dn6_slot = var_guard22_dn6;
        *var_guard22_dn7_slot = var_guard22_dn7;
        *var_guard22_dn8_slot = var_guard22_dn8;
        *var_guard22_dn9_slot = var_guard22_dn9;
        *var_guard22_rdb0_slot = var_guard22_rdb0;
        *var_guard22_rdb1_slot = var_guard22_rdb1;
        *var_guard22_rdb10_slot = var_guard22_rdb10;
        *var_guard22_rdb11_slot = var_guard22_rdb11;
        *var_guard22_rdb12_slot = var_guard22_rdb12;
        *var_guard22_rdb13_slot = var_guard22_rdb13;
        *var_guard22_rdb14_slot = var_guard22_rdb14;
        *var_guard22_rdb15_slot = var_guard22_rdb15;
        *var_guard22_rdb16_slot = var_guard22_rdb16;
        *var_guard22_rdb17_slot = var_guard22_rdb17;
        *var_guard22_rdb18_slot = var_guard22_rdb18;
        *var_guard22_rdb2_slot = var_guard22_rdb2;
        *var_guard22_rdb3_slot = var_guard22_rdb3;
        *var_guard22_rdb4_slot = var_guard22_rdb4;
        *var_guard22_rdb5_slot = var_guard22_rdb5;
        *var_guard22_rdb6_slot = var_guard22_rdb6;
        *var_guard22_rdb7_slot = var_guard22_rdb7;
        *var_guard22_rdb8_slot = var_guard22_rdb8;
        *var_guard22_rdb9_slot = var_guard22_rdb9;
        *var_guard22_rdn0_slot = var_guard22_rdn0;
        *var_guard22_rdn1_slot = var_guard22_rdn1;
        *var_guard22_rdn10_slot = var_guard22_rdn10;
        *var_guard22_rdn11_slot = var_guard22_rdn11;
        *var_guard22_rdn12_slot = var_guard22_rdn12;
        *var_guard22_rdn13_slot = var_guard22_rdn13;
        *var_guard22_rdn14_slot = var_guard22_rdn14;
        *var_guard22_rdn15_slot = var_guard22_rdn15;
        *var_guard22_rdn2_slot = var_guard22_rdn2;
        *var_guard22_rdn3_slot = var_guard22_rdn3;
        *var_guard22_rdn4_slot = var_guard22_rdn4;
        *var_guard22_rdn5_slot = var_guard22_rdn5;
        *var_guard22_rdn6_slot = var_guard22_rdn6;
        *var_guard22_rdn7_slot = var_guard22_rdn7;
        *var_guard22_rdn8_slot = var_guard22_rdn8;
        *var_guard22_rdn9_slot = var_guard22_rdn9;
        *var_guard22_rv_slot = var_guard22_rv;
        *var_guard24_slot = var_guard24;
        *var_guard24_db0_slot = var_guard24_db0;
        *var_guard24_db1_slot = var_guard24_db1;
        *var_guard24_db10_slot = var_guard24_db10;
        *var_guard24_db11_slot = var_guard24_db11;
        *var_guard24_db12_slot = var_guard24_db12;
        *var_guard24_db13_slot = var_guard24_db13;
        *var_guard24_db14_slot = var_guard24_db14;
        *var_guard24_db15_slot = var_guard24_db15;
        *var_guard24_db16_slot = var_guard24_db16;
        *var_guard24_db17_slot = var_guard24_db17;
        *var_guard24_db18_slot = var_guard24_db18;
        *var_guard24_db2_slot = var_guard24_db2;
        *var_guard24_db3_slot = var_guard24_db3;
        *var_guard24_db4_slot = var_guard24_db4;
        *var_guard24_db5_slot = var_guard24_db5;
        *var_guard24_db6_slot = var_guard24_db6;
        *var_guard24_db7_slot = var_guard24_db7;
        *var_guard24_db8_slot = var_guard24_db8;
        *var_guard24_db9_slot = var_guard24_db9;
        *var_guard24_dn0_slot = var_guard24_dn0;
        *var_guard24_dn1_slot = var_guard24_dn1;
        *var_guard24_dn10_slot = var_guard24_dn10;
        *var_guard24_dn11_slot = var_guard24_dn11;
        *var_guard24_dn12_slot = var_guard24_dn12;
        *var_guard24_dn13_slot = var_guard24_dn13;
        *var_guard24_dn14_slot = var_guard24_dn14;
        *var_guard24_dn15_slot = var_guard24_dn15;
        *var_guard24_dn2_slot = var_guard24_dn2;
        *var_guard24_dn3_slot = var_guard24_dn3;
        *var_guard24_dn4_slot = var_guard24_dn4;
        *var_guard24_dn5_slot = var_guard24_dn5;
        *var_guard24_dn6_slot = var_guard24_dn6;
        *var_guard24_dn7_slot = var_guard24_dn7;
        *var_guard24_dn8_slot = var_guard24_dn8;
        *var_guard24_dn9_slot = var_guard24_dn9;
        *var_guard24_rdb0_slot = var_guard24_rdb0;
        *var_guard24_rdb1_slot = var_guard24_rdb1;
        *var_guard24_rdb10_slot = var_guard24_rdb10;
        *var_guard24_rdb11_slot = var_guard24_rdb11;
        *var_guard24_rdb12_slot = var_guard24_rdb12;
        *var_guard24_rdb13_slot = var_guard24_rdb13;
        *var_guard24_rdb14_slot = var_guard24_rdb14;
        *var_guard24_rdb15_slot = var_guard24_rdb15;
        *var_guard24_rdb16_slot = var_guard24_rdb16;
        *var_guard24_rdb17_slot = var_guard24_rdb17;
        *var_guard24_rdb18_slot = var_guard24_rdb18;
        *var_guard24_rdb2_slot = var_guard24_rdb2;
        *var_guard24_rdb3_slot = var_guard24_rdb3;
        *var_guard24_rdb4_slot = var_guard24_rdb4;
        *var_guard24_rdb5_slot = var_guard24_rdb5;
        *var_guard24_rdb6_slot = var_guard24_rdb6;
        *var_guard24_rdb7_slot = var_guard24_rdb7;
        *var_guard24_rdb8_slot = var_guard24_rdb8;
        *var_guard24_rdb9_slot = var_guard24_rdb9;
        *var_guard24_rdn0_slot = var_guard24_rdn0;
        *var_guard24_rdn1_slot = var_guard24_rdn1;
        *var_guard24_rdn10_slot = var_guard24_rdn10;
        *var_guard24_rdn11_slot = var_guard24_rdn11;
        *var_guard24_rdn12_slot = var_guard24_rdn12;
        *var_guard24_rdn13_slot = var_guard24_rdn13;
        *var_guard24_rdn14_slot = var_guard24_rdn14;
        *var_guard24_rdn15_slot = var_guard24_rdn15;
        *var_guard24_rdn2_slot = var_guard24_rdn2;
        *var_guard24_rdn3_slot = var_guard24_rdn3;
        *var_guard24_rdn4_slot = var_guard24_rdn4;
        *var_guard24_rdn5_slot = var_guard24_rdn5;
        *var_guard24_rdn6_slot = var_guard24_rdn6;
        *var_guard24_rdn7_slot = var_guard24_rdn7;
        *var_guard24_rdn8_slot = var_guard24_rdn8;
        *var_guard24_rdn9_slot = var_guard24_rdn9;
        *var_guard24_rv_slot = var_guard24_rv;
        *var_guard25_slot = var_guard25;
        *var_guard25_db0_slot = var_guard25_db0;
        *var_guard25_db1_slot = var_guard25_db1;
        *var_guard25_db10_slot = var_guard25_db10;
        *var_guard25_db11_slot = var_guard25_db11;
        *var_guard25_db12_slot = var_guard25_db12;
        *var_guard25_db13_slot = var_guard25_db13;
        *var_guard25_db14_slot = var_guard25_db14;
        *var_guard25_db15_slot = var_guard25_db15;
        *var_guard25_db16_slot = var_guard25_db16;
        *var_guard25_db17_slot = var_guard25_db17;
        *var_guard25_db18_slot = var_guard25_db18;
        *var_guard25_db2_slot = var_guard25_db2;
        *var_guard25_db3_slot = var_guard25_db3;
        *var_guard25_db4_slot = var_guard25_db4;
        *var_guard25_db5_slot = var_guard25_db5;
        *var_guard25_db6_slot = var_guard25_db6;
        *var_guard25_db7_slot = var_guard25_db7;
        *var_guard25_db8_slot = var_guard25_db8;
        *var_guard25_db9_slot = var_guard25_db9;
        *var_guard25_dn0_slot = var_guard25_dn0;
        *var_guard25_dn1_slot = var_guard25_dn1;
        *var_guard25_dn10_slot = var_guard25_dn10;
        *var_guard25_dn11_slot = var_guard25_dn11;
        *var_guard25_dn12_slot = var_guard25_dn12;
        *var_guard25_dn13_slot = var_guard25_dn13;
        *var_guard25_dn14_slot = var_guard25_dn14;
        *var_guard25_dn15_slot = var_guard25_dn15;
        *var_guard25_dn2_slot = var_guard25_dn2;
        *var_guard25_dn3_slot = var_guard25_dn3;
        *var_guard25_dn4_slot = var_guard25_dn4;
        *var_guard25_dn5_slot = var_guard25_dn5;
        *var_guard25_dn6_slot = var_guard25_dn6;
        *var_guard25_dn7_slot = var_guard25_dn7;
        *var_guard25_dn8_slot = var_guard25_dn8;
        *var_guard25_dn9_slot = var_guard25_dn9;
        *var_guard25_rdb0_slot = var_guard25_rdb0;
        *var_guard25_rdb1_slot = var_guard25_rdb1;
        *var_guard25_rdb10_slot = var_guard25_rdb10;
        *var_guard25_rdb11_slot = var_guard25_rdb11;
        *var_guard25_rdb12_slot = var_guard25_rdb12;
        *var_guard25_rdb13_slot = var_guard25_rdb13;
        *var_guard25_rdb14_slot = var_guard25_rdb14;
        *var_guard25_rdb15_slot = var_guard25_rdb15;
        *var_guard25_rdb16_slot = var_guard25_rdb16;
        *var_guard25_rdb17_slot = var_guard25_rdb17;
        *var_guard25_rdb18_slot = var_guard25_rdb18;
        *var_guard25_rdb2_slot = var_guard25_rdb2;
        *var_guard25_rdb3_slot = var_guard25_rdb3;
        *var_guard25_rdb4_slot = var_guard25_rdb4;
        *var_guard25_rdb5_slot = var_guard25_rdb5;
        *var_guard25_rdb6_slot = var_guard25_rdb6;
        *var_guard25_rdb7_slot = var_guard25_rdb7;
        *var_guard25_rdb8_slot = var_guard25_rdb8;
        *var_guard25_rdb9_slot = var_guard25_rdb9;
        *var_guard25_rdn0_slot = var_guard25_rdn0;
        *var_guard25_rdn1_slot = var_guard25_rdn1;
        *var_guard25_rdn10_slot = var_guard25_rdn10;
        *var_guard25_rdn11_slot = var_guard25_rdn11;
        *var_guard25_rdn12_slot = var_guard25_rdn12;
        *var_guard25_rdn13_slot = var_guard25_rdn13;
        *var_guard25_rdn14_slot = var_guard25_rdn14;
        *var_guard25_rdn15_slot = var_guard25_rdn15;
        *var_guard25_rdn2_slot = var_guard25_rdn2;
        *var_guard25_rdn3_slot = var_guard25_rdn3;
        *var_guard25_rdn4_slot = var_guard25_rdn4;
        *var_guard25_rdn5_slot = var_guard25_rdn5;
        *var_guard25_rdn6_slot = var_guard25_rdn6;
        *var_guard25_rdn7_slot = var_guard25_rdn7;
        *var_guard25_rdn8_slot = var_guard25_rdn8;
        *var_guard25_rdn9_slot = var_guard25_rdn9;
        *var_guard25_rv_slot = var_guard25_rv;
        *var_guard26_slot = var_guard26;
        *var_guard26_db0_slot = var_guard26_db0;
        *var_guard26_db1_slot = var_guard26_db1;
        *var_guard26_db10_slot = var_guard26_db10;
        *var_guard26_db11_slot = var_guard26_db11;
        *var_guard26_db12_slot = var_guard26_db12;
        *var_guard26_db13_slot = var_guard26_db13;
        *var_guard26_db14_slot = var_guard26_db14;
        *var_guard26_db15_slot = var_guard26_db15;
        *var_guard26_db16_slot = var_guard26_db16;
        *var_guard26_db17_slot = var_guard26_db17;
        *var_guard26_db18_slot = var_guard26_db18;
        *var_guard26_db2_slot = var_guard26_db2;
        *var_guard26_db3_slot = var_guard26_db3;
        *var_guard26_db4_slot = var_guard26_db4;
        *var_guard26_db5_slot = var_guard26_db5;
        *var_guard26_db6_slot = var_guard26_db6;
        *var_guard26_db7_slot = var_guard26_db7;
        *var_guard26_db8_slot = var_guard26_db8;
        *var_guard26_db9_slot = var_guard26_db9;
        *var_guard26_dn0_slot = var_guard26_dn0;
        *var_guard26_dn1_slot = var_guard26_dn1;
        *var_guard26_dn10_slot = var_guard26_dn10;
        *var_guard26_dn11_slot = var_guard26_dn11;
        *var_guard26_dn12_slot = var_guard26_dn12;
        *var_guard26_dn13_slot = var_guard26_dn13;
        *var_guard26_dn14_slot = var_guard26_dn14;
        *var_guard26_dn15_slot = var_guard26_dn15;
        *var_guard26_dn2_slot = var_guard26_dn2;
        *var_guard26_dn3_slot = var_guard26_dn3;
        *var_guard26_dn4_slot = var_guard26_dn4;
        *var_guard26_dn5_slot = var_guard26_dn5;
        *var_guard26_dn6_slot = var_guard26_dn6;
        *var_guard26_dn7_slot = var_guard26_dn7;
        *var_guard26_dn8_slot = var_guard26_dn8;
        *var_guard26_dn9_slot = var_guard26_dn9;
        *var_guard26_rdb0_slot = var_guard26_rdb0;
        *var_guard26_rdb1_slot = var_guard26_rdb1;
        *var_guard26_rdb10_slot = var_guard26_rdb10;
        *var_guard26_rdb11_slot = var_guard26_rdb11;
        *var_guard26_rdb12_slot = var_guard26_rdb12;
        *var_guard26_rdb13_slot = var_guard26_rdb13;
        *var_guard26_rdb14_slot = var_guard26_rdb14;
        *var_guard26_rdb15_slot = var_guard26_rdb15;
        *var_guard26_rdb16_slot = var_guard26_rdb16;
        *var_guard26_rdb17_slot = var_guard26_rdb17;
        *var_guard26_rdb18_slot = var_guard26_rdb18;
        *var_guard26_rdb2_slot = var_guard26_rdb2;
        *var_guard26_rdb3_slot = var_guard26_rdb3;
        *var_guard26_rdb4_slot = var_guard26_rdb4;
        *var_guard26_rdb5_slot = var_guard26_rdb5;
        *var_guard26_rdb6_slot = var_guard26_rdb6;
        *var_guard26_rdb7_slot = var_guard26_rdb7;
        *var_guard26_rdb8_slot = var_guard26_rdb8;
        *var_guard26_rdb9_slot = var_guard26_rdb9;
        *var_guard26_rdn0_slot = var_guard26_rdn0;
        *var_guard26_rdn1_slot = var_guard26_rdn1;
        *var_guard26_rdn10_slot = var_guard26_rdn10;
        *var_guard26_rdn11_slot = var_guard26_rdn11;
        *var_guard26_rdn12_slot = var_guard26_rdn12;
        *var_guard26_rdn13_slot = var_guard26_rdn13;
        *var_guard26_rdn14_slot = var_guard26_rdn14;
        *var_guard26_rdn15_slot = var_guard26_rdn15;
        *var_guard26_rdn2_slot = var_guard26_rdn2;
        *var_guard26_rdn3_slot = var_guard26_rdn3;
        *var_guard26_rdn4_slot = var_guard26_rdn4;
        *var_guard26_rdn5_slot = var_guard26_rdn5;
        *var_guard26_rdn6_slot = var_guard26_rdn6;
        *var_guard26_rdn7_slot = var_guard26_rdn7;
        *var_guard26_rdn8_slot = var_guard26_rdn8;
        *var_guard26_rdn9_slot = var_guard26_rdn9;
        *var_guard26_rv_slot = var_guard26_rv;
    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        var_cgs0_t: f64,
        var_cgs0_t_db0: f64,
        var_cgs0_t_db1: f64,
        var_cgs0_t_db10: f64,
        var_cgs0_t_db11: f64,
        var_cgs0_t_db12: f64,
        var_cgs0_t_db13: f64,
        var_cgs0_t_db14: f64,
        var_cgs0_t_db2: f64,
        var_cgs0_t_db3: f64,
        var_cgs0_t_db4: f64,
        var_cgs0_t_db5: f64,
        var_cgs0_t_db6: f64,
        var_cgs0_t_db7: f64,
        var_cgs0_t_db8: f64,
        var_cgs0_t_db9: f64,
        var_cgs0_t_dn0: f64,
        var_cgs0_t_dn1: f64,
        var_cgs0_t_dn10: f64,
        var_cgs0_t_dn11: f64,
        var_cgs0_t_dn12: f64,
        var_cgs0_t_dn13: f64,
        var_cgs0_t_dn14: f64,
        var_cgs0_t_dn15: f64,
        var_cgs0_t_dn2: f64,
        var_cgs0_t_dn3: f64,
        var_cgs0_t_dn4: f64,
        var_cgs0_t_dn5: f64,
        var_cgs0_t_dn6: f64,
        var_cgs0_t_dn7: f64,
        var_cgs0_t_dn8: f64,
        var_cgs0_t_dn9: f64,
        var_guard26: f64,
        var_t: f64,
        var_t_db0: f64,
        var_t_db1: f64,
        var_t_db10: f64,
        var_t_db11: f64,
        var_t_db12: f64,
        var_t_db13: f64,
        var_t_db14: f64,
        var_t_db2: f64,
        var_t_db3: f64,
        var_t_db4: f64,
        var_t_db5: f64,
        var_t_db6: f64,
        var_t_db7: f64,
        var_t_db8: f64,
        var_t_db9: f64,
        var_t_dn0: f64,
        var_t_dn1: f64,
        var_t_dn10: f64,
        var_t_dn11: f64,
        var_t_dn12: f64,
        var_t_dn13: f64,
        var_t_dn14: f64,
        var_t_dn15: f64,
        var_t_dn2: f64,
        var_t_dn3: f64,
        var_t_dn4: f64,
        var_t_dn5: f64,
        var_t_dn6: f64,
        var_t_dn7: f64,
        var_t_dn8: f64,
        var_t_dn9: f64,
        var_ci_slot: &mut f64,
        var_ci_db0_slot: &mut f64,
        var_ci_db1_slot: &mut f64,
        var_ci_db10_slot: &mut f64,
        var_ci_db11_slot: &mut f64,
        var_ci_db12_slot: &mut f64,
        var_ci_db13_slot: &mut f64,
        var_ci_db14_slot: &mut f64,
        var_ci_db15_slot: &mut f64,
        var_ci_db16_slot: &mut f64,
        var_ci_db17_slot: &mut f64,
        var_ci_db18_slot: &mut f64,
        var_ci_db2_slot: &mut f64,
        var_ci_db3_slot: &mut f64,
        var_ci_db4_slot: &mut f64,
        var_ci_db5_slot: &mut f64,
        var_ci_db6_slot: &mut f64,
        var_ci_db7_slot: &mut f64,
        var_ci_db8_slot: &mut f64,
        var_ci_db9_slot: &mut f64,
        var_ci_dn0_slot: &mut f64,
        var_ci_dn1_slot: &mut f64,
        var_ci_dn10_slot: &mut f64,
        var_ci_dn11_slot: &mut f64,
        var_ci_dn12_slot: &mut f64,
        var_ci_dn13_slot: &mut f64,
        var_ci_dn14_slot: &mut f64,
        var_ci_dn15_slot: &mut f64,
        var_ci_dn2_slot: &mut f64,
        var_ci_dn3_slot: &mut f64,
        var_ci_dn4_slot: &mut f64,
        var_ci_dn5_slot: &mut f64,
        var_ci_dn6_slot: &mut f64,
        var_ci_dn7_slot: &mut f64,
        var_ci_dn8_slot: &mut f64,
        var_ci_dn9_slot: &mut f64,
        var_ci_rdb0_slot: &mut f64,
        var_ci_rdb1_slot: &mut f64,
        var_ci_rdb10_slot: &mut f64,
        var_ci_rdb11_slot: &mut f64,
        var_ci_rdb12_slot: &mut f64,
        var_ci_rdb13_slot: &mut f64,
        var_ci_rdb14_slot: &mut f64,
        var_ci_rdb15_slot: &mut f64,
        var_ci_rdb16_slot: &mut f64,
        var_ci_rdb17_slot: &mut f64,
        var_ci_rdb18_slot: &mut f64,
        var_ci_rdb2_slot: &mut f64,
        var_ci_rdb3_slot: &mut f64,
        var_ci_rdb4_slot: &mut f64,
        var_ci_rdb5_slot: &mut f64,
        var_ci_rdb6_slot: &mut f64,
        var_ci_rdb7_slot: &mut f64,
        var_ci_rdb8_slot: &mut f64,
        var_ci_rdb9_slot: &mut f64,
        var_ci_rdn0_slot: &mut f64,
        var_ci_rdn1_slot: &mut f64,
        var_ci_rdn10_slot: &mut f64,
        var_ci_rdn11_slot: &mut f64,
        var_ci_rdn12_slot: &mut f64,
        var_ci_rdn13_slot: &mut f64,
        var_ci_rdn14_slot: &mut f64,
        var_ci_rdn15_slot: &mut f64,
        var_ci_rdn2_slot: &mut f64,
        var_ci_rdn3_slot: &mut f64,
        var_ci_rdn4_slot: &mut f64,
        var_ci_rdn5_slot: &mut f64,
        var_ci_rdn6_slot: &mut f64,
        var_ci_rdn7_slot: &mut f64,
        var_ci_rdn8_slot: &mut f64,
        var_ci_rdn9_slot: &mut f64,
        var_ci_rv_slot: &mut f64,
        var_guard27_slot: &mut f64,
        var_guard27_db0_slot: &mut f64,
        var_guard27_db1_slot: &mut f64,
        var_guard27_db10_slot: &mut f64,
        var_guard27_db11_slot: &mut f64,
        var_guard27_db12_slot: &mut f64,
        var_guard27_db13_slot: &mut f64,
        var_guard27_db14_slot: &mut f64,
        var_guard27_db15_slot: &mut f64,
        var_guard27_db16_slot: &mut f64,
        var_guard27_db17_slot: &mut f64,
        var_guard27_db18_slot: &mut f64,
        var_guard27_db2_slot: &mut f64,
        var_guard27_db3_slot: &mut f64,
        var_guard27_db4_slot: &mut f64,
        var_guard27_db5_slot: &mut f64,
        var_guard27_db6_slot: &mut f64,
        var_guard27_db7_slot: &mut f64,
        var_guard27_db8_slot: &mut f64,
        var_guard27_db9_slot: &mut f64,
        var_guard27_dn0_slot: &mut f64,
        var_guard27_dn1_slot: &mut f64,
        var_guard27_dn10_slot: &mut f64,
        var_guard27_dn11_slot: &mut f64,
        var_guard27_dn12_slot: &mut f64,
        var_guard27_dn13_slot: &mut f64,
        var_guard27_dn14_slot: &mut f64,
        var_guard27_dn15_slot: &mut f64,
        var_guard27_dn2_slot: &mut f64,
        var_guard27_dn3_slot: &mut f64,
        var_guard27_dn4_slot: &mut f64,
        var_guard27_dn5_slot: &mut f64,
        var_guard27_dn6_slot: &mut f64,
        var_guard27_dn7_slot: &mut f64,
        var_guard27_dn8_slot: &mut f64,
        var_guard27_dn9_slot: &mut f64,
        var_guard27_rdb0_slot: &mut f64,
        var_guard27_rdb1_slot: &mut f64,
        var_guard27_rdb10_slot: &mut f64,
        var_guard27_rdb11_slot: &mut f64,
        var_guard27_rdb12_slot: &mut f64,
        var_guard27_rdb13_slot: &mut f64,
        var_guard27_rdb14_slot: &mut f64,
        var_guard27_rdb15_slot: &mut f64,
        var_guard27_rdb16_slot: &mut f64,
        var_guard27_rdb17_slot: &mut f64,
        var_guard27_rdb18_slot: &mut f64,
        var_guard27_rdb2_slot: &mut f64,
        var_guard27_rdb3_slot: &mut f64,
        var_guard27_rdb4_slot: &mut f64,
        var_guard27_rdb5_slot: &mut f64,
        var_guard27_rdb6_slot: &mut f64,
        var_guard27_rdb7_slot: &mut f64,
        var_guard27_rdb8_slot: &mut f64,
        var_guard27_rdb9_slot: &mut f64,
        var_guard27_rdn0_slot: &mut f64,
        var_guard27_rdn1_slot: &mut f64,
        var_guard27_rdn10_slot: &mut f64,
        var_guard27_rdn11_slot: &mut f64,
        var_guard27_rdn12_slot: &mut f64,
        var_guard27_rdn13_slot: &mut f64,
        var_guard27_rdn14_slot: &mut f64,
        var_guard27_rdn15_slot: &mut f64,
        var_guard27_rdn2_slot: &mut f64,
        var_guard27_rdn3_slot: &mut f64,
        var_guard27_rdn4_slot: &mut f64,
        var_guard27_rdn5_slot: &mut f64,
        var_guard27_rdn6_slot: &mut f64,
        var_guard27_rdn7_slot: &mut f64,
        var_guard27_rdn8_slot: &mut f64,
        var_guard27_rdn9_slot: &mut f64,
        var_guard27_rv_slot: &mut f64,
        var_guard43_slot: &mut f64,
        var_guard43_db0_slot: &mut f64,
        var_guard43_db1_slot: &mut f64,
        var_guard43_db10_slot: &mut f64,
        var_guard43_db11_slot: &mut f64,
        var_guard43_db12_slot: &mut f64,
        var_guard43_db13_slot: &mut f64,
        var_guard43_db14_slot: &mut f64,
        var_guard43_db15_slot: &mut f64,
        var_guard43_db16_slot: &mut f64,
        var_guard43_db17_slot: &mut f64,
        var_guard43_db18_slot: &mut f64,
        var_guard43_db2_slot: &mut f64,
        var_guard43_db3_slot: &mut f64,
        var_guard43_db4_slot: &mut f64,
        var_guard43_db5_slot: &mut f64,
        var_guard43_db6_slot: &mut f64,
        var_guard43_db7_slot: &mut f64,
        var_guard43_db8_slot: &mut f64,
        var_guard43_db9_slot: &mut f64,
        var_guard43_dn0_slot: &mut f64,
        var_guard43_dn1_slot: &mut f64,
        var_guard43_dn10_slot: &mut f64,
        var_guard43_dn11_slot: &mut f64,
        var_guard43_dn12_slot: &mut f64,
        var_guard43_dn13_slot: &mut f64,
        var_guard43_dn14_slot: &mut f64,
        var_guard43_dn15_slot: &mut f64,
        var_guard43_dn2_slot: &mut f64,
        var_guard43_dn3_slot: &mut f64,
        var_guard43_dn4_slot: &mut f64,
        var_guard43_dn5_slot: &mut f64,
        var_guard43_dn6_slot: &mut f64,
        var_guard43_dn7_slot: &mut f64,
        var_guard43_dn8_slot: &mut f64,
        var_guard43_dn9_slot: &mut f64,
        var_guard43_rdb0_slot: &mut f64,
        var_guard43_rdb1_slot: &mut f64,
        var_guard43_rdb10_slot: &mut f64,
        var_guard43_rdb11_slot: &mut f64,
        var_guard43_rdb12_slot: &mut f64,
        var_guard43_rdb13_slot: &mut f64,
        var_guard43_rdb14_slot: &mut f64,
        var_guard43_rdb15_slot: &mut f64,
        var_guard43_rdb16_slot: &mut f64,
        var_guard43_rdb17_slot: &mut f64,
        var_guard43_rdb18_slot: &mut f64,
        var_guard43_rdb2_slot: &mut f64,
        var_guard43_rdb3_slot: &mut f64,
        var_guard43_rdb4_slot: &mut f64,
        var_guard43_rdb5_slot: &mut f64,
        var_guard43_rdb6_slot: &mut f64,
        var_guard43_rdb7_slot: &mut f64,
        var_guard43_rdb8_slot: &mut f64,
        var_guard43_rdb9_slot: &mut f64,
        var_guard43_rdn0_slot: &mut f64,
        var_guard43_rdn1_slot: &mut f64,
        var_guard43_rdn10_slot: &mut f64,
        var_guard43_rdn11_slot: &mut f64,
        var_guard43_rdn12_slot: &mut f64,
        var_guard43_rdn13_slot: &mut f64,
        var_guard43_rdn14_slot: &mut f64,
        var_guard43_rdn15_slot: &mut f64,
        var_guard43_rdn2_slot: &mut f64,
        var_guard43_rdn3_slot: &mut f64,
        var_guard43_rdn4_slot: &mut f64,
        var_guard43_rdn5_slot: &mut f64,
        var_guard43_rdn6_slot: &mut f64,
        var_guard43_rdn7_slot: &mut f64,
        var_guard43_rdn8_slot: &mut f64,
        var_guard43_rdn9_slot: &mut f64,
        var_guard43_rv_slot: &mut f64,
        var_k_slot: &mut f64,
        var_k_db0_slot: &mut f64,
        var_k_db1_slot: &mut f64,
        var_k_db10_slot: &mut f64,
        var_k_db11_slot: &mut f64,
        var_k_db12_slot: &mut f64,
        var_k_db13_slot: &mut f64,
        var_k_db14_slot: &mut f64,
        var_k_db15_slot: &mut f64,
        var_k_db16_slot: &mut f64,
        var_k_db17_slot: &mut f64,
        var_k_db18_slot: &mut f64,
        var_k_db2_slot: &mut f64,
        var_k_db3_slot: &mut f64,
        var_k_db4_slot: &mut f64,
        var_k_db5_slot: &mut f64,
        var_k_db6_slot: &mut f64,
        var_k_db7_slot: &mut f64,
        var_k_db8_slot: &mut f64,
        var_k_db9_slot: &mut f64,
        var_k_dn0_slot: &mut f64,
        var_k_dn1_slot: &mut f64,
        var_k_dn10_slot: &mut f64,
        var_k_dn11_slot: &mut f64,
        var_k_dn12_slot: &mut f64,
        var_k_dn13_slot: &mut f64,
        var_k_dn14_slot: &mut f64,
        var_k_dn15_slot: &mut f64,
        var_k_dn2_slot: &mut f64,
        var_k_dn3_slot: &mut f64,
        var_k_dn4_slot: &mut f64,
        var_k_dn5_slot: &mut f64,
        var_k_dn6_slot: &mut f64,
        var_k_dn7_slot: &mut f64,
        var_k_dn8_slot: &mut f64,
        var_k_dn9_slot: &mut f64,
        var_k_rdb0_slot: &mut f64,
        var_k_rdb1_slot: &mut f64,
        var_k_rdb10_slot: &mut f64,
        var_k_rdb11_slot: &mut f64,
        var_k_rdb12_slot: &mut f64,
        var_k_rdb13_slot: &mut f64,
        var_k_rdb14_slot: &mut f64,
        var_k_rdb15_slot: &mut f64,
        var_k_rdb16_slot: &mut f64,
        var_k_rdb17_slot: &mut f64,
        var_k_rdb18_slot: &mut f64,
        var_k_rdb2_slot: &mut f64,
        var_k_rdb3_slot: &mut f64,
        var_k_rdb4_slot: &mut f64,
        var_k_rdb5_slot: &mut f64,
        var_k_rdb6_slot: &mut f64,
        var_k_rdb7_slot: &mut f64,
        var_k_rdb8_slot: &mut f64,
        var_k_rdb9_slot: &mut f64,
        var_k_rdn0_slot: &mut f64,
        var_k_rdn1_slot: &mut f64,
        var_k_rdn10_slot: &mut f64,
        var_k_rdn11_slot: &mut f64,
        var_k_rdn12_slot: &mut f64,
        var_k_rdn13_slot: &mut f64,
        var_k_rdn14_slot: &mut f64,
        var_k_rdn15_slot: &mut f64,
        var_k_rdn2_slot: &mut f64,
        var_k_rdn3_slot: &mut f64,
        var_k_rdn4_slot: &mut f64,
        var_k_rdn5_slot: &mut f64,
        var_k_rdn6_slot: &mut f64,
        var_k_rdn7_slot: &mut f64,
        var_k_rdn8_slot: &mut f64,
        var_k_rdn9_slot: &mut f64,
        var_k_rv_slot: &mut f64,
    ) {
        let mut var_ci: f64 = *var_ci_slot;
        let mut var_ci_db0: f64 = *var_ci_db0_slot;
        let mut var_ci_db1: f64 = *var_ci_db1_slot;
        let mut var_ci_db10: f64 = *var_ci_db10_slot;
        let mut var_ci_db11: f64 = *var_ci_db11_slot;
        let mut var_ci_db12: f64 = *var_ci_db12_slot;
        let mut var_ci_db13: f64 = *var_ci_db13_slot;
        let mut var_ci_db14: f64 = *var_ci_db14_slot;
        let mut var_ci_db15: f64 = *var_ci_db15_slot;
        let mut var_ci_db16: f64 = *var_ci_db16_slot;
        let mut var_ci_db17: f64 = *var_ci_db17_slot;
        let mut var_ci_db18: f64 = *var_ci_db18_slot;
        let mut var_ci_db2: f64 = *var_ci_db2_slot;
        let mut var_ci_db3: f64 = *var_ci_db3_slot;
        let mut var_ci_db4: f64 = *var_ci_db4_slot;
        let mut var_ci_db5: f64 = *var_ci_db5_slot;
        let mut var_ci_db6: f64 = *var_ci_db6_slot;
        let mut var_ci_db7: f64 = *var_ci_db7_slot;
        let mut var_ci_db8: f64 = *var_ci_db8_slot;
        let mut var_ci_db9: f64 = *var_ci_db9_slot;
        let mut var_ci_dn0: f64 = *var_ci_dn0_slot;
        let mut var_ci_dn1: f64 = *var_ci_dn1_slot;
        let mut var_ci_dn10: f64 = *var_ci_dn10_slot;
        let mut var_ci_dn11: f64 = *var_ci_dn11_slot;
        let mut var_ci_dn12: f64 = *var_ci_dn12_slot;
        let mut var_ci_dn13: f64 = *var_ci_dn13_slot;
        let mut var_ci_dn14: f64 = *var_ci_dn14_slot;
        let mut var_ci_dn15: f64 = *var_ci_dn15_slot;
        let mut var_ci_dn2: f64 = *var_ci_dn2_slot;
        let mut var_ci_dn3: f64 = *var_ci_dn3_slot;
        let mut var_ci_dn4: f64 = *var_ci_dn4_slot;
        let mut var_ci_dn5: f64 = *var_ci_dn5_slot;
        let mut var_ci_dn6: f64 = *var_ci_dn6_slot;
        let mut var_ci_dn7: f64 = *var_ci_dn7_slot;
        let mut var_ci_dn8: f64 = *var_ci_dn8_slot;
        let mut var_ci_dn9: f64 = *var_ci_dn9_slot;
        let mut var_ci_rdb0: f64 = *var_ci_rdb0_slot;
        let mut var_ci_rdb1: f64 = *var_ci_rdb1_slot;
        let mut var_ci_rdb10: f64 = *var_ci_rdb10_slot;
        let mut var_ci_rdb11: f64 = *var_ci_rdb11_slot;
        let mut var_ci_rdb12: f64 = *var_ci_rdb12_slot;
        let mut var_ci_rdb13: f64 = *var_ci_rdb13_slot;
        let mut var_ci_rdb14: f64 = *var_ci_rdb14_slot;
        let mut var_ci_rdb15: f64 = *var_ci_rdb15_slot;
        let mut var_ci_rdb16: f64 = *var_ci_rdb16_slot;
        let mut var_ci_rdb17: f64 = *var_ci_rdb17_slot;
        let mut var_ci_rdb18: f64 = *var_ci_rdb18_slot;
        let mut var_ci_rdb2: f64 = *var_ci_rdb2_slot;
        let mut var_ci_rdb3: f64 = *var_ci_rdb3_slot;
        let mut var_ci_rdb4: f64 = *var_ci_rdb4_slot;
        let mut var_ci_rdb5: f64 = *var_ci_rdb5_slot;
        let mut var_ci_rdb6: f64 = *var_ci_rdb6_slot;
        let mut var_ci_rdb7: f64 = *var_ci_rdb7_slot;
        let mut var_ci_rdb8: f64 = *var_ci_rdb8_slot;
        let mut var_ci_rdb9: f64 = *var_ci_rdb9_slot;
        let mut var_ci_rdn0: f64 = *var_ci_rdn0_slot;
        let mut var_ci_rdn1: f64 = *var_ci_rdn1_slot;
        let mut var_ci_rdn10: f64 = *var_ci_rdn10_slot;
        let mut var_ci_rdn11: f64 = *var_ci_rdn11_slot;
        let mut var_ci_rdn12: f64 = *var_ci_rdn12_slot;
        let mut var_ci_rdn13: f64 = *var_ci_rdn13_slot;
        let mut var_ci_rdn14: f64 = *var_ci_rdn14_slot;
        let mut var_ci_rdn15: f64 = *var_ci_rdn15_slot;
        let mut var_ci_rdn2: f64 = *var_ci_rdn2_slot;
        let mut var_ci_rdn3: f64 = *var_ci_rdn3_slot;
        let mut var_ci_rdn4: f64 = *var_ci_rdn4_slot;
        let mut var_ci_rdn5: f64 = *var_ci_rdn5_slot;
        let mut var_ci_rdn6: f64 = *var_ci_rdn6_slot;
        let mut var_ci_rdn7: f64 = *var_ci_rdn7_slot;
        let mut var_ci_rdn8: f64 = *var_ci_rdn8_slot;
        let mut var_ci_rdn9: f64 = *var_ci_rdn9_slot;
        let mut var_ci_rv: f64 = *var_ci_rv_slot;
        let mut var_guard27: f64 = *var_guard27_slot;
        let mut var_guard27_db0: f64 = *var_guard27_db0_slot;
        let mut var_guard27_db1: f64 = *var_guard27_db1_slot;
        let mut var_guard27_db10: f64 = *var_guard27_db10_slot;
        let mut var_guard27_db11: f64 = *var_guard27_db11_slot;
        let mut var_guard27_db12: f64 = *var_guard27_db12_slot;
        let mut var_guard27_db13: f64 = *var_guard27_db13_slot;
        let mut var_guard27_db14: f64 = *var_guard27_db14_slot;
        let mut var_guard27_db15: f64 = *var_guard27_db15_slot;
        let mut var_guard27_db16: f64 = *var_guard27_db16_slot;
        let mut var_guard27_db17: f64 = *var_guard27_db17_slot;
        let mut var_guard27_db18: f64 = *var_guard27_db18_slot;
        let mut var_guard27_db2: f64 = *var_guard27_db2_slot;
        let mut var_guard27_db3: f64 = *var_guard27_db3_slot;
        let mut var_guard27_db4: f64 = *var_guard27_db4_slot;
        let mut var_guard27_db5: f64 = *var_guard27_db5_slot;
        let mut var_guard27_db6: f64 = *var_guard27_db6_slot;
        let mut var_guard27_db7: f64 = *var_guard27_db7_slot;
        let mut var_guard27_db8: f64 = *var_guard27_db8_slot;
        let mut var_guard27_db9: f64 = *var_guard27_db9_slot;
        let mut var_guard27_dn0: f64 = *var_guard27_dn0_slot;
        let mut var_guard27_dn1: f64 = *var_guard27_dn1_slot;
        let mut var_guard27_dn10: f64 = *var_guard27_dn10_slot;
        let mut var_guard27_dn11: f64 = *var_guard27_dn11_slot;
        let mut var_guard27_dn12: f64 = *var_guard27_dn12_slot;
        let mut var_guard27_dn13: f64 = *var_guard27_dn13_slot;
        let mut var_guard27_dn14: f64 = *var_guard27_dn14_slot;
        let mut var_guard27_dn15: f64 = *var_guard27_dn15_slot;
        let mut var_guard27_dn2: f64 = *var_guard27_dn2_slot;
        let mut var_guard27_dn3: f64 = *var_guard27_dn3_slot;
        let mut var_guard27_dn4: f64 = *var_guard27_dn4_slot;
        let mut var_guard27_dn5: f64 = *var_guard27_dn5_slot;
        let mut var_guard27_dn6: f64 = *var_guard27_dn6_slot;
        let mut var_guard27_dn7: f64 = *var_guard27_dn7_slot;
        let mut var_guard27_dn8: f64 = *var_guard27_dn8_slot;
        let mut var_guard27_dn9: f64 = *var_guard27_dn9_slot;
        let mut var_guard27_rdb0: f64 = *var_guard27_rdb0_slot;
        let mut var_guard27_rdb1: f64 = *var_guard27_rdb1_slot;
        let mut var_guard27_rdb10: f64 = *var_guard27_rdb10_slot;
        let mut var_guard27_rdb11: f64 = *var_guard27_rdb11_slot;
        let mut var_guard27_rdb12: f64 = *var_guard27_rdb12_slot;
        let mut var_guard27_rdb13: f64 = *var_guard27_rdb13_slot;
        let mut var_guard27_rdb14: f64 = *var_guard27_rdb14_slot;
        let mut var_guard27_rdb15: f64 = *var_guard27_rdb15_slot;
        let mut var_guard27_rdb16: f64 = *var_guard27_rdb16_slot;
        let mut var_guard27_rdb17: f64 = *var_guard27_rdb17_slot;
        let mut var_guard27_rdb18: f64 = *var_guard27_rdb18_slot;
        let mut var_guard27_rdb2: f64 = *var_guard27_rdb2_slot;
        let mut var_guard27_rdb3: f64 = *var_guard27_rdb3_slot;
        let mut var_guard27_rdb4: f64 = *var_guard27_rdb4_slot;
        let mut var_guard27_rdb5: f64 = *var_guard27_rdb5_slot;
        let mut var_guard27_rdb6: f64 = *var_guard27_rdb6_slot;
        let mut var_guard27_rdb7: f64 = *var_guard27_rdb7_slot;
        let mut var_guard27_rdb8: f64 = *var_guard27_rdb8_slot;
        let mut var_guard27_rdb9: f64 = *var_guard27_rdb9_slot;
        let mut var_guard27_rdn0: f64 = *var_guard27_rdn0_slot;
        let mut var_guard27_rdn1: f64 = *var_guard27_rdn1_slot;
        let mut var_guard27_rdn10: f64 = *var_guard27_rdn10_slot;
        let mut var_guard27_rdn11: f64 = *var_guard27_rdn11_slot;
        let mut var_guard27_rdn12: f64 = *var_guard27_rdn12_slot;
        let mut var_guard27_rdn13: f64 = *var_guard27_rdn13_slot;
        let mut var_guard27_rdn14: f64 = *var_guard27_rdn14_slot;
        let mut var_guard27_rdn15: f64 = *var_guard27_rdn15_slot;
        let mut var_guard27_rdn2: f64 = *var_guard27_rdn2_slot;
        let mut var_guard27_rdn3: f64 = *var_guard27_rdn3_slot;
        let mut var_guard27_rdn4: f64 = *var_guard27_rdn4_slot;
        let mut var_guard27_rdn5: f64 = *var_guard27_rdn5_slot;
        let mut var_guard27_rdn6: f64 = *var_guard27_rdn6_slot;
        let mut var_guard27_rdn7: f64 = *var_guard27_rdn7_slot;
        let mut var_guard27_rdn8: f64 = *var_guard27_rdn8_slot;
        let mut var_guard27_rdn9: f64 = *var_guard27_rdn9_slot;
        let mut var_guard27_rv: f64 = *var_guard27_rv_slot;
        let mut var_guard43: f64 = *var_guard43_slot;
        let mut var_guard43_db0: f64 = *var_guard43_db0_slot;
        let mut var_guard43_db1: f64 = *var_guard43_db1_slot;
        let mut var_guard43_db10: f64 = *var_guard43_db10_slot;
        let mut var_guard43_db11: f64 = *var_guard43_db11_slot;
        let mut var_guard43_db12: f64 = *var_guard43_db12_slot;
        let mut var_guard43_db13: f64 = *var_guard43_db13_slot;
        let mut var_guard43_db14: f64 = *var_guard43_db14_slot;
        let mut var_guard43_db15: f64 = *var_guard43_db15_slot;
        let mut var_guard43_db16: f64 = *var_guard43_db16_slot;
        let mut var_guard43_db17: f64 = *var_guard43_db17_slot;
        let mut var_guard43_db18: f64 = *var_guard43_db18_slot;
        let mut var_guard43_db2: f64 = *var_guard43_db2_slot;
        let mut var_guard43_db3: f64 = *var_guard43_db3_slot;
        let mut var_guard43_db4: f64 = *var_guard43_db4_slot;
        let mut var_guard43_db5: f64 = *var_guard43_db5_slot;
        let mut var_guard43_db6: f64 = *var_guard43_db6_slot;
        let mut var_guard43_db7: f64 = *var_guard43_db7_slot;
        let mut var_guard43_db8: f64 = *var_guard43_db8_slot;
        let mut var_guard43_db9: f64 = *var_guard43_db9_slot;
        let mut var_guard43_dn0: f64 = *var_guard43_dn0_slot;
        let mut var_guard43_dn1: f64 = *var_guard43_dn1_slot;
        let mut var_guard43_dn10: f64 = *var_guard43_dn10_slot;
        let mut var_guard43_dn11: f64 = *var_guard43_dn11_slot;
        let mut var_guard43_dn12: f64 = *var_guard43_dn12_slot;
        let mut var_guard43_dn13: f64 = *var_guard43_dn13_slot;
        let mut var_guard43_dn14: f64 = *var_guard43_dn14_slot;
        let mut var_guard43_dn15: f64 = *var_guard43_dn15_slot;
        let mut var_guard43_dn2: f64 = *var_guard43_dn2_slot;
        let mut var_guard43_dn3: f64 = *var_guard43_dn3_slot;
        let mut var_guard43_dn4: f64 = *var_guard43_dn4_slot;
        let mut var_guard43_dn5: f64 = *var_guard43_dn5_slot;
        let mut var_guard43_dn6: f64 = *var_guard43_dn6_slot;
        let mut var_guard43_dn7: f64 = *var_guard43_dn7_slot;
        let mut var_guard43_dn8: f64 = *var_guard43_dn8_slot;
        let mut var_guard43_dn9: f64 = *var_guard43_dn9_slot;
        let mut var_guard43_rdb0: f64 = *var_guard43_rdb0_slot;
        let mut var_guard43_rdb1: f64 = *var_guard43_rdb1_slot;
        let mut var_guard43_rdb10: f64 = *var_guard43_rdb10_slot;
        let mut var_guard43_rdb11: f64 = *var_guard43_rdb11_slot;
        let mut var_guard43_rdb12: f64 = *var_guard43_rdb12_slot;
        let mut var_guard43_rdb13: f64 = *var_guard43_rdb13_slot;
        let mut var_guard43_rdb14: f64 = *var_guard43_rdb14_slot;
        let mut var_guard43_rdb15: f64 = *var_guard43_rdb15_slot;
        let mut var_guard43_rdb16: f64 = *var_guard43_rdb16_slot;
        let mut var_guard43_rdb17: f64 = *var_guard43_rdb17_slot;
        let mut var_guard43_rdb18: f64 = *var_guard43_rdb18_slot;
        let mut var_guard43_rdb2: f64 = *var_guard43_rdb2_slot;
        let mut var_guard43_rdb3: f64 = *var_guard43_rdb3_slot;
        let mut var_guard43_rdb4: f64 = *var_guard43_rdb4_slot;
        let mut var_guard43_rdb5: f64 = *var_guard43_rdb5_slot;
        let mut var_guard43_rdb6: f64 = *var_guard43_rdb6_slot;
        let mut var_guard43_rdb7: f64 = *var_guard43_rdb7_slot;
        let mut var_guard43_rdb8: f64 = *var_guard43_rdb8_slot;
        let mut var_guard43_rdb9: f64 = *var_guard43_rdb9_slot;
        let mut var_guard43_rdn0: f64 = *var_guard43_rdn0_slot;
        let mut var_guard43_rdn1: f64 = *var_guard43_rdn1_slot;
        let mut var_guard43_rdn10: f64 = *var_guard43_rdn10_slot;
        let mut var_guard43_rdn11: f64 = *var_guard43_rdn11_slot;
        let mut var_guard43_rdn12: f64 = *var_guard43_rdn12_slot;
        let mut var_guard43_rdn13: f64 = *var_guard43_rdn13_slot;
        let mut var_guard43_rdn14: f64 = *var_guard43_rdn14_slot;
        let mut var_guard43_rdn15: f64 = *var_guard43_rdn15_slot;
        let mut var_guard43_rdn2: f64 = *var_guard43_rdn2_slot;
        let mut var_guard43_rdn3: f64 = *var_guard43_rdn3_slot;
        let mut var_guard43_rdn4: f64 = *var_guard43_rdn4_slot;
        let mut var_guard43_rdn5: f64 = *var_guard43_rdn5_slot;
        let mut var_guard43_rdn6: f64 = *var_guard43_rdn6_slot;
        let mut var_guard43_rdn7: f64 = *var_guard43_rdn7_slot;
        let mut var_guard43_rdn8: f64 = *var_guard43_rdn8_slot;
        let mut var_guard43_rdn9: f64 = *var_guard43_rdn9_slot;
        let mut var_guard43_rv: f64 = *var_guard43_rv_slot;
        let mut var_k: f64 = *var_k_slot;
        let mut var_k_db0: f64 = *var_k_db0_slot;
        let mut var_k_db1: f64 = *var_k_db1_slot;
        let mut var_k_db10: f64 = *var_k_db10_slot;
        let mut var_k_db11: f64 = *var_k_db11_slot;
        let mut var_k_db12: f64 = *var_k_db12_slot;
        let mut var_k_db13: f64 = *var_k_db13_slot;
        let mut var_k_db14: f64 = *var_k_db14_slot;
        let mut var_k_db15: f64 = *var_k_db15_slot;
        let mut var_k_db16: f64 = *var_k_db16_slot;
        let mut var_k_db17: f64 = *var_k_db17_slot;
        let mut var_k_db18: f64 = *var_k_db18_slot;
        let mut var_k_db2: f64 = *var_k_db2_slot;
        let mut var_k_db3: f64 = *var_k_db3_slot;
        let mut var_k_db4: f64 = *var_k_db4_slot;
        let mut var_k_db5: f64 = *var_k_db5_slot;
        let mut var_k_db6: f64 = *var_k_db6_slot;
        let mut var_k_db7: f64 = *var_k_db7_slot;
        let mut var_k_db8: f64 = *var_k_db8_slot;
        let mut var_k_db9: f64 = *var_k_db9_slot;
        let mut var_k_dn0: f64 = *var_k_dn0_slot;
        let mut var_k_dn1: f64 = *var_k_dn1_slot;
        let mut var_k_dn10: f64 = *var_k_dn10_slot;
        let mut var_k_dn11: f64 = *var_k_dn11_slot;
        let mut var_k_dn12: f64 = *var_k_dn12_slot;
        let mut var_k_dn13: f64 = *var_k_dn13_slot;
        let mut var_k_dn14: f64 = *var_k_dn14_slot;
        let mut var_k_dn15: f64 = *var_k_dn15_slot;
        let mut var_k_dn2: f64 = *var_k_dn2_slot;
        let mut var_k_dn3: f64 = *var_k_dn3_slot;
        let mut var_k_dn4: f64 = *var_k_dn4_slot;
        let mut var_k_dn5: f64 = *var_k_dn5_slot;
        let mut var_k_dn6: f64 = *var_k_dn6_slot;
        let mut var_k_dn7: f64 = *var_k_dn7_slot;
        let mut var_k_dn8: f64 = *var_k_dn8_slot;
        let mut var_k_dn9: f64 = *var_k_dn9_slot;
        let mut var_k_rdb0: f64 = *var_k_rdb0_slot;
        let mut var_k_rdb1: f64 = *var_k_rdb1_slot;
        let mut var_k_rdb10: f64 = *var_k_rdb10_slot;
        let mut var_k_rdb11: f64 = *var_k_rdb11_slot;
        let mut var_k_rdb12: f64 = *var_k_rdb12_slot;
        let mut var_k_rdb13: f64 = *var_k_rdb13_slot;
        let mut var_k_rdb14: f64 = *var_k_rdb14_slot;
        let mut var_k_rdb15: f64 = *var_k_rdb15_slot;
        let mut var_k_rdb16: f64 = *var_k_rdb16_slot;
        let mut var_k_rdb17: f64 = *var_k_rdb17_slot;
        let mut var_k_rdb18: f64 = *var_k_rdb18_slot;
        let mut var_k_rdb2: f64 = *var_k_rdb2_slot;
        let mut var_k_rdb3: f64 = *var_k_rdb3_slot;
        let mut var_k_rdb4: f64 = *var_k_rdb4_slot;
        let mut var_k_rdb5: f64 = *var_k_rdb5_slot;
        let mut var_k_rdb6: f64 = *var_k_rdb6_slot;
        let mut var_k_rdb7: f64 = *var_k_rdb7_slot;
        let mut var_k_rdb8: f64 = *var_k_rdb8_slot;
        let mut var_k_rdb9: f64 = *var_k_rdb9_slot;
        let mut var_k_rdn0: f64 = *var_k_rdn0_slot;
        let mut var_k_rdn1: f64 = *var_k_rdn1_slot;
        let mut var_k_rdn10: f64 = *var_k_rdn10_slot;
        let mut var_k_rdn11: f64 = *var_k_rdn11_slot;
        let mut var_k_rdn12: f64 = *var_k_rdn12_slot;
        let mut var_k_rdn13: f64 = *var_k_rdn13_slot;
        let mut var_k_rdn14: f64 = *var_k_rdn14_slot;
        let mut var_k_rdn15: f64 = *var_k_rdn15_slot;
        let mut var_k_rdn2: f64 = *var_k_rdn2_slot;
        let mut var_k_rdn3: f64 = *var_k_rdn3_slot;
        let mut var_k_rdn4: f64 = *var_k_rdn4_slot;
        let mut var_k_rdn5: f64 = *var_k_rdn5_slot;
        let mut var_k_rdn6: f64 = *var_k_rdn6_slot;
        let mut var_k_rdn7: f64 = *var_k_rdn7_slot;
        let mut var_k_rdn8: f64 = *var_k_rdn8_slot;
        let mut var_k_rdn9: f64 = *var_k_rdn9_slot;
        let mut var_k_rv: f64 = *var_k_rv_slot;

        let assign1690_e2062: f64 = if p.p7 == 1.0 { 1.0 } else { 0.0 };
        var_guard27 = assign1690_e2062;
        var_guard27_dn0 = 0.0;
        var_guard27_dn1 = 0.0;
        var_guard27_dn2 = 0.0;
        var_guard27_dn3 = 0.0;
        var_guard27_dn4 = 0.0;
        var_guard27_dn5 = 0.0;
        var_guard27_dn6 = 0.0;
        var_guard27_dn7 = 0.0;
        var_guard27_dn8 = 0.0;
        var_guard27_dn9 = 0.0;
        var_guard27_dn10 = 0.0;
        var_guard27_dn11 = 0.0;
        var_guard27_dn12 = 0.0;
        var_guard27_dn13 = 0.0;
        var_guard27_dn14 = 0.0;
        var_guard27_dn15 = 0.0;
        var_guard27_db0 = 0.0;
        var_guard27_db1 = 0.0;
        var_guard27_db2 = 0.0;
        var_guard27_db3 = 0.0;
        var_guard27_db4 = 0.0;
        var_guard27_db5 = 0.0;
        var_guard27_db6 = 0.0;
        var_guard27_db7 = 0.0;
        var_guard27_db8 = 0.0;
        var_guard27_db9 = 0.0;
        var_guard27_db10 = 0.0;
        var_guard27_db11 = 0.0;
        var_guard27_db12 = 0.0;
        var_guard27_db13 = 0.0;
        var_guard27_db14 = 0.0;
        var_guard27_db15 = 0.0;
        var_guard27_db16 = 0.0;
        var_guard27_db17 = 0.0;
        var_guard27_db18 = 0.0;
        var_guard27_rv = 0.0;
        var_guard27_rdn0 = 0.0;
        var_guard27_rdn1 = 0.0;
        var_guard27_rdn2 = 0.0;
        var_guard27_rdn3 = 0.0;
        var_guard27_rdn4 = 0.0;
        var_guard27_rdn5 = 0.0;
        var_guard27_rdn6 = 0.0;
        var_guard27_rdn7 = 0.0;
        var_guard27_rdn8 = 0.0;
        var_guard27_rdn9 = 0.0;
        var_guard27_rdn10 = 0.0;
        var_guard27_rdn11 = 0.0;
        var_guard27_rdn12 = 0.0;
        var_guard27_rdn13 = 0.0;
        var_guard27_rdn14 = 0.0;
        var_guard27_rdn15 = 0.0;
        var_guard27_rdb0 = 0.0;
        var_guard27_rdb1 = 0.0;
        var_guard27_rdb2 = 0.0;
        var_guard27_rdb3 = 0.0;
        var_guard27_rdb4 = 0.0;
        var_guard27_rdb5 = 0.0;
        var_guard27_rdb6 = 0.0;
        var_guard27_rdb7 = 0.0;
        var_guard27_rdb8 = 0.0;
        var_guard27_rdb9 = 0.0;
        var_guard27_rdb10 = 0.0;
        var_guard27_rdb11 = 0.0;
        var_guard27_rdb12 = 0.0;
        var_guard27_rdb13 = 0.0;
        var_guard27_rdb14 = 0.0;
        var_guard27_rdb15 = 0.0;
        var_guard27_rdb16 = 0.0;
        var_guard27_rdb17 = 0.0;
        var_guard27_rdb18 = 0.0;

        let (assign1790_e2206, assign1790_e2206_d_n0, assign1790_e2206_d_n1, assign1790_e2206_d_n2, assign1790_e2206_d_n3, assign1790_e2206_d_n4, assign1790_e2206_d_n5, assign1790_e2206_d_n6, assign1790_e2206_d_n7, assign1790_e2206_d_n8, assign1790_e2206_d_n9, assign1790_e2206_d_n10, assign1790_e2206_d_n11, assign1790_e2206_d_n12, assign1790_e2206_d_n13, assign1790_e2206_d_n14, assign1790_e2206_d_n15, assign1790_e2206_d_b0, assign1790_e2206_d_b1, assign1790_e2206_d_b2, assign1790_e2206_d_b3, assign1790_e2206_d_b4, assign1790_e2206_d_b5, assign1790_e2206_d_b6, assign1790_e2206_d_b7, assign1790_e2206_d_b8, assign1790_e2206_d_b9, assign1790_e2206_d_b10, assign1790_e2206_d_b11, assign1790_e2206_d_b12, assign1790_e2206_d_b13, assign1790_e2206_d_b14,) = {
    if (((var_guard27 != 0.0) && (var_guard26 == 0.0)) && (p.p0 != 0.0)) {
        let assign1790_e2193: f64 = (4.0 * 1.3806503e-23);
        let assign1790_e2195: f64 = (assign1790_e2193 * var_t);
        let assign1790_e2197: f64 = (assign1790_e2195 * p.p73);
        let assign1790_e2199: f64 = (assign1790_e2197 * var_cgs0_t);
        let assign1790_e2202: f64 = (p.p72 * p.p71);
        let assign1790_e2203: f64 = (assign1790_e2202).sqrt();
        let assign1790_e2204: f64 = (assign1790_e2199 * assign1790_e2203);
        (assign1790_e2204, (((((assign1790_e2193 * var_t_dn0) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_dn0)) * assign1790_e2203), (((((assign1790_e2193 * var_t_dn1) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_dn1)) * assign1790_e2203), (((((assign1790_e2193 * var_t_dn2) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_dn2)) * assign1790_e2203), (((((assign1790_e2193 * var_t_dn3) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_dn3)) * assign1790_e2203), (((((assign1790_e2193 * var_t_dn4) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_dn4)) * assign1790_e2203), (((((assign1790_e2193 * var_t_dn5) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_dn5)) * assign1790_e2203), (((((assign1790_e2193 * var_t_dn6) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_dn6)) * assign1790_e2203), (((((assign1790_e2193 * var_t_dn7) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_dn7)) * assign1790_e2203), (((((assign1790_e2193 * var_t_dn8) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_dn8)) * assign1790_e2203), (((((assign1790_e2193 * var_t_dn9) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_dn9)) * assign1790_e2203), (((((assign1790_e2193 * var_t_dn10) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_dn10)) * assign1790_e2203), (((((assign1790_e2193 * var_t_dn11) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_dn11)) * assign1790_e2203), (((((assign1790_e2193 * var_t_dn12) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_dn12)) * assign1790_e2203), (((((assign1790_e2193 * var_t_dn13) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_dn13)) * assign1790_e2203), (((((assign1790_e2193 * var_t_dn14) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_dn14)) * assign1790_e2203), (((((assign1790_e2193 * var_t_dn15) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_dn15)) * assign1790_e2203), (((((assign1790_e2193 * var_t_db0) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_db0)) * assign1790_e2203), (((((assign1790_e2193 * var_t_db1) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_db1)) * assign1790_e2203), (((((assign1790_e2193 * var_t_db2) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_db2)) * assign1790_e2203), (((((assign1790_e2193 * var_t_db3) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_db3)) * assign1790_e2203), (((((assign1790_e2193 * var_t_db4) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_db4)) * assign1790_e2203), (((((assign1790_e2193 * var_t_db5) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_db5)) * assign1790_e2203), (((((assign1790_e2193 * var_t_db6) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_db6)) * assign1790_e2203), (((((assign1790_e2193 * var_t_db7) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_db7)) * assign1790_e2203), (((((assign1790_e2193 * var_t_db8) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_db8)) * assign1790_e2203), (((((assign1790_e2193 * var_t_db9) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_db9)) * assign1790_e2203), (((((assign1790_e2193 * var_t_db10) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_db10)) * assign1790_e2203), (((((assign1790_e2193 * var_t_db11) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_db11)) * assign1790_e2203), (((((assign1790_e2193 * var_t_db12) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_db12)) * assign1790_e2203), (((((assign1790_e2193 * var_t_db13) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_db13)) * assign1790_e2203), (((((assign1790_e2193 * var_t_db14) * p.p73) * var_cgs0_t) + (assign1790_e2197 * var_cgs0_t_db14)) * assign1790_e2203),)
    } else {
        (var_k, var_k_dn0, var_k_dn1, var_k_dn2, var_k_dn3, var_k_dn4, var_k_dn5, var_k_dn6, var_k_dn7, var_k_dn8, var_k_dn9, var_k_dn10, var_k_dn11, var_k_dn12, var_k_dn13, var_k_dn14, var_k_dn15, var_k_db0, var_k_db1, var_k_db2, var_k_db3, var_k_db4, var_k_db5, var_k_db6, var_k_db7, var_k_db8, var_k_db9, var_k_db10, var_k_db11, var_k_db12, var_k_db13, var_k_db14,)
    }
};
        var_k = assign1790_e2206;
        var_k_dn0 = assign1790_e2206_d_n0;
        var_k_dn1 = assign1790_e2206_d_n1;
        var_k_dn2 = assign1790_e2206_d_n2;
        var_k_dn3 = assign1790_e2206_d_n3;
        var_k_dn4 = assign1790_e2206_d_n4;
        var_k_dn5 = assign1790_e2206_d_n5;
        var_k_dn6 = assign1790_e2206_d_n6;
        var_k_dn7 = assign1790_e2206_d_n7;
        var_k_dn8 = assign1790_e2206_d_n8;
        var_k_dn9 = assign1790_e2206_d_n9;
        var_k_dn10 = assign1790_e2206_d_n10;
        var_k_dn11 = assign1790_e2206_d_n11;
        var_k_dn12 = assign1790_e2206_d_n12;
        var_k_dn13 = assign1790_e2206_d_n13;
        var_k_dn14 = assign1790_e2206_d_n14;
        var_k_dn15 = assign1790_e2206_d_n15;
        var_k_db0 = assign1790_e2206_d_b0;
        var_k_db1 = assign1790_e2206_d_b1;
        var_k_db2 = assign1790_e2206_d_b2;
        var_k_db3 = assign1790_e2206_d_b3;
        var_k_db4 = assign1790_e2206_d_b4;
        var_k_db5 = assign1790_e2206_d_b5;
        var_k_db6 = assign1790_e2206_d_b6;
        var_k_db7 = assign1790_e2206_d_b7;
        var_k_db8 = assign1790_e2206_d_b8;
        var_k_db9 = assign1790_e2206_d_b9;
        var_k_db10 = assign1790_e2206_d_b10;
        var_k_db11 = assign1790_e2206_d_b11;
        var_k_db12 = assign1790_e2206_d_b12;
        var_k_db13 = assign1790_e2206_d_b13;
        var_k_db14 = assign1790_e2206_d_b14;
        var_k_db15 = 0.0;
        var_k_db16 = 0.0;
        var_k_db17 = 0.0;
        var_k_db18 = 0.0;
        var_k_rv = 0.0;
        var_k_rdn0 = 0.0;
        var_k_rdn1 = 0.0;
        var_k_rdn2 = 0.0;
        var_k_rdn3 = 0.0;
        var_k_rdn4 = 0.0;
        var_k_rdn5 = 0.0;
        var_k_rdn6 = 0.0;
        var_k_rdn7 = 0.0;
        var_k_rdn8 = 0.0;
        var_k_rdn9 = 0.0;
        var_k_rdn10 = 0.0;
        var_k_rdn11 = 0.0;
        var_k_rdn12 = 0.0;
        var_k_rdn13 = 0.0;
        var_k_rdn14 = 0.0;
        var_k_rdn15 = 0.0;
        var_k_rdb0 = 0.0;
        var_k_rdb1 = 0.0;
        var_k_rdb2 = 0.0;
        var_k_rdb3 = 0.0;
        var_k_rdb4 = 0.0;
        var_k_rdb5 = 0.0;
        var_k_rdb6 = 0.0;
        var_k_rdb7 = 0.0;
        var_k_rdb8 = 0.0;
        var_k_rdb9 = 0.0;
        var_k_rdb10 = 0.0;
        var_k_rdb11 = 0.0;
        var_k_rdb12 = 0.0;
        var_k_rdb13 = 0.0;
        var_k_rdb14 = 0.0;
        var_k_rdb15 = 0.0;
        var_k_rdb16 = 0.0;
        var_k_rdb17 = 0.0;
        var_k_rdb18 = 0.0;

        let (assign1820_e2243, assign1820_e2243_d_n0, assign1820_e2243_d_n1, assign1820_e2243_d_n2, assign1820_e2243_d_n3, assign1820_e2243_d_n4, assign1820_e2243_d_n5, assign1820_e2243_d_n6, assign1820_e2243_d_n7, assign1820_e2243_d_n8, assign1820_e2243_d_n9, assign1820_e2243_d_n10, assign1820_e2243_d_n11, assign1820_e2243_d_n12, assign1820_e2243_d_n13, assign1820_e2243_d_n14, assign1820_e2243_d_n15, assign1820_e2243_d_b0, assign1820_e2243_d_b1, assign1820_e2243_d_b2, assign1820_e2243_d_b3, assign1820_e2243_d_b4, assign1820_e2243_d_b5, assign1820_e2243_d_b6, assign1820_e2243_d_b7, assign1820_e2243_d_b8, assign1820_e2243_d_b9, assign1820_e2243_d_b10, assign1820_e2243_d_b11, assign1820_e2243_d_b12, assign1820_e2243_d_b13, assign1820_e2243_d_b14,) = {
    if (((var_guard27 != 0.0) && (var_guard26 == 0.0)) && (p.p0 != 0.0)) {
        let assign1820_e2241: f64 = (var_k * 3.141592653589793);
        (assign1820_e2241, (var_k_dn0 * 3.141592653589793), (var_k_dn1 * 3.141592653589793), (var_k_dn2 * 3.141592653589793), (var_k_dn3 * 3.141592653589793), (var_k_dn4 * 3.141592653589793), (var_k_dn5 * 3.141592653589793), (var_k_dn6 * 3.141592653589793), (var_k_dn7 * 3.141592653589793), (var_k_dn8 * 3.141592653589793), (var_k_dn9 * 3.141592653589793), (var_k_dn10 * 3.141592653589793), (var_k_dn11 * 3.141592653589793), (var_k_dn12 * 3.141592653589793), (var_k_dn13 * 3.141592653589793), (var_k_dn14 * 3.141592653589793), (var_k_dn15 * 3.141592653589793), (var_k_db0 * 3.141592653589793), (var_k_db1 * 3.141592653589793), (var_k_db2 * 3.141592653589793), (var_k_db3 * 3.141592653589793), (var_k_db4 * 3.141592653589793), (var_k_db5 * 3.141592653589793), (var_k_db6 * 3.141592653589793), (var_k_db7 * 3.141592653589793), (var_k_db8 * 3.141592653589793), (var_k_db9 * 3.141592653589793), (var_k_db10 * 3.141592653589793), (var_k_db11 * 3.141592653589793), (var_k_db12 * 3.141592653589793), (var_k_db13 * 3.141592653589793), (var_k_db14 * 3.141592653589793),)
    } else {
        (var_ci, var_ci_dn0, var_ci_dn1, var_ci_dn2, var_ci_dn3, var_ci_dn4, var_ci_dn5, var_ci_dn6, var_ci_dn7, var_ci_dn8, var_ci_dn9, var_ci_dn10, var_ci_dn11, var_ci_dn12, var_ci_dn13, var_ci_dn14, var_ci_dn15, var_ci_db0, var_ci_db1, var_ci_db2, var_ci_db3, var_ci_db4, var_ci_db5, var_ci_db6, var_ci_db7, var_ci_db8, var_ci_db9, var_ci_db10, var_ci_db11, var_ci_db12, var_ci_db13, var_ci_db14,)
    }
};
        var_ci = assign1820_e2243;
        var_ci_dn0 = assign1820_e2243_d_n0;
        var_ci_dn1 = assign1820_e2243_d_n1;
        var_ci_dn2 = assign1820_e2243_d_n2;
        var_ci_dn3 = assign1820_e2243_d_n3;
        var_ci_dn4 = assign1820_e2243_d_n4;
        var_ci_dn5 = assign1820_e2243_d_n5;
        var_ci_dn6 = assign1820_e2243_d_n6;
        var_ci_dn7 = assign1820_e2243_d_n7;
        var_ci_dn8 = assign1820_e2243_d_n8;
        var_ci_dn9 = assign1820_e2243_d_n9;
        var_ci_dn10 = assign1820_e2243_d_n10;
        var_ci_dn11 = assign1820_e2243_d_n11;
        var_ci_dn12 = assign1820_e2243_d_n12;
        var_ci_dn13 = assign1820_e2243_d_n13;
        var_ci_dn14 = assign1820_e2243_d_n14;
        var_ci_dn15 = assign1820_e2243_d_n15;
        var_ci_db0 = assign1820_e2243_d_b0;
        var_ci_db1 = assign1820_e2243_d_b1;
        var_ci_db2 = assign1820_e2243_d_b2;
        var_ci_db3 = assign1820_e2243_d_b3;
        var_ci_db4 = assign1820_e2243_d_b4;
        var_ci_db5 = assign1820_e2243_d_b5;
        var_ci_db6 = assign1820_e2243_d_b6;
        var_ci_db7 = assign1820_e2243_d_b7;
        var_ci_db8 = assign1820_e2243_d_b8;
        var_ci_db9 = assign1820_e2243_d_b9;
        var_ci_db10 = assign1820_e2243_d_b10;
        var_ci_db11 = assign1820_e2243_d_b11;
        var_ci_db12 = assign1820_e2243_d_b12;
        var_ci_db13 = assign1820_e2243_d_b13;
        var_ci_db14 = assign1820_e2243_d_b14;
        var_ci_db15 = 0.0;
        var_ci_db16 = 0.0;
        var_ci_db17 = 0.0;
        var_ci_db18 = 0.0;
        var_ci_rv = 0.0;
        var_ci_rdn0 = 0.0;
        var_ci_rdn1 = 0.0;
        var_ci_rdn2 = 0.0;
        var_ci_rdn3 = 0.0;
        var_ci_rdn4 = 0.0;
        var_ci_rdn5 = 0.0;
        var_ci_rdn6 = 0.0;
        var_ci_rdn7 = 0.0;
        var_ci_rdn8 = 0.0;
        var_ci_rdn9 = 0.0;
        var_ci_rdn10 = 0.0;
        var_ci_rdn11 = 0.0;
        var_ci_rdn12 = 0.0;
        var_ci_rdn13 = 0.0;
        var_ci_rdn14 = 0.0;
        var_ci_rdn15 = 0.0;
        var_ci_rdb0 = 0.0;
        var_ci_rdb1 = 0.0;
        var_ci_rdb2 = 0.0;
        var_ci_rdb3 = 0.0;
        var_ci_rdb4 = 0.0;
        var_ci_rdb5 = 0.0;
        var_ci_rdb6 = 0.0;
        var_ci_rdb7 = 0.0;
        var_ci_rdb8 = 0.0;
        var_ci_rdb9 = 0.0;
        var_ci_rdb10 = 0.0;
        var_ci_rdb11 = 0.0;
        var_ci_rdb12 = 0.0;
        var_ci_rdb13 = 0.0;
        var_ci_rdb14 = 0.0;
        var_ci_rdb15 = 0.0;
        var_ci_rdb16 = 0.0;
        var_ci_rdb17 = 0.0;
        var_ci_rdb18 = 0.0;

        let assign1860_e2271: f64 = if ((p.p1 != 0.0) && (p.p57 != 0.0)) { 1.0 } else { 0.0 };
        var_guard43 = assign1860_e2271;
        var_guard43_dn0 = 0.0;
        var_guard43_dn1 = 0.0;
        var_guard43_dn2 = 0.0;
        var_guard43_dn3 = 0.0;
        var_guard43_dn4 = 0.0;
        var_guard43_dn5 = 0.0;
        var_guard43_dn6 = 0.0;
        var_guard43_dn7 = 0.0;
        var_guard43_dn8 = 0.0;
        var_guard43_dn9 = 0.0;
        var_guard43_dn10 = 0.0;
        var_guard43_dn11 = 0.0;
        var_guard43_dn12 = 0.0;
        var_guard43_dn13 = 0.0;
        var_guard43_dn14 = 0.0;
        var_guard43_dn15 = 0.0;
        var_guard43_db0 = 0.0;
        var_guard43_db1 = 0.0;
        var_guard43_db2 = 0.0;
        var_guard43_db3 = 0.0;
        var_guard43_db4 = 0.0;
        var_guard43_db5 = 0.0;
        var_guard43_db6 = 0.0;
        var_guard43_db7 = 0.0;
        var_guard43_db8 = 0.0;
        var_guard43_db9 = 0.0;
        var_guard43_db10 = 0.0;
        var_guard43_db11 = 0.0;
        var_guard43_db12 = 0.0;
        var_guard43_db13 = 0.0;
        var_guard43_db14 = 0.0;
        var_guard43_db15 = 0.0;
        var_guard43_db16 = 0.0;
        var_guard43_db17 = 0.0;
        var_guard43_db18 = 0.0;
        var_guard43_rv = 0.0;
        var_guard43_rdn0 = 0.0;
        var_guard43_rdn1 = 0.0;
        var_guard43_rdn2 = 0.0;
        var_guard43_rdn3 = 0.0;
        var_guard43_rdn4 = 0.0;
        var_guard43_rdn5 = 0.0;
        var_guard43_rdn6 = 0.0;
        var_guard43_rdn7 = 0.0;
        var_guard43_rdn8 = 0.0;
        var_guard43_rdn9 = 0.0;
        var_guard43_rdn10 = 0.0;
        var_guard43_rdn11 = 0.0;
        var_guard43_rdn12 = 0.0;
        var_guard43_rdn13 = 0.0;
        var_guard43_rdn14 = 0.0;
        var_guard43_rdn15 = 0.0;
        var_guard43_rdb0 = 0.0;
        var_guard43_rdb1 = 0.0;
        var_guard43_rdb2 = 0.0;
        var_guard43_rdb3 = 0.0;
        var_guard43_rdb4 = 0.0;
        var_guard43_rdb5 = 0.0;
        var_guard43_rdb6 = 0.0;
        var_guard43_rdb7 = 0.0;
        var_guard43_rdb8 = 0.0;
        var_guard43_rdb9 = 0.0;
        var_guard43_rdb10 = 0.0;
        var_guard43_rdb11 = 0.0;
        var_guard43_rdb12 = 0.0;
        var_guard43_rdb13 = 0.0;
        var_guard43_rdb14 = 0.0;
        var_guard43_rdb15 = 0.0;
        var_guard43_rdb16 = 0.0;
        var_guard43_rdb17 = 0.0;
        var_guard43_rdb18 = 0.0;

        *var_ci_slot = var_ci;
        *var_ci_db0_slot = var_ci_db0;
        *var_ci_db1_slot = var_ci_db1;
        *var_ci_db10_slot = var_ci_db10;
        *var_ci_db11_slot = var_ci_db11;
        *var_ci_db12_slot = var_ci_db12;
        *var_ci_db13_slot = var_ci_db13;
        *var_ci_db14_slot = var_ci_db14;
        *var_ci_db15_slot = var_ci_db15;
        *var_ci_db16_slot = var_ci_db16;
        *var_ci_db17_slot = var_ci_db17;
        *var_ci_db18_slot = var_ci_db18;
        *var_ci_db2_slot = var_ci_db2;
        *var_ci_db3_slot = var_ci_db3;
        *var_ci_db4_slot = var_ci_db4;
        *var_ci_db5_slot = var_ci_db5;
        *var_ci_db6_slot = var_ci_db6;
        *var_ci_db7_slot = var_ci_db7;
        *var_ci_db8_slot = var_ci_db8;
        *var_ci_db9_slot = var_ci_db9;
        *var_ci_dn0_slot = var_ci_dn0;
        *var_ci_dn1_slot = var_ci_dn1;
        *var_ci_dn10_slot = var_ci_dn10;
        *var_ci_dn11_slot = var_ci_dn11;
        *var_ci_dn12_slot = var_ci_dn12;
        *var_ci_dn13_slot = var_ci_dn13;
        *var_ci_dn14_slot = var_ci_dn14;
        *var_ci_dn15_slot = var_ci_dn15;
        *var_ci_dn2_slot = var_ci_dn2;
        *var_ci_dn3_slot = var_ci_dn3;
        *var_ci_dn4_slot = var_ci_dn4;
        *var_ci_dn5_slot = var_ci_dn5;
        *var_ci_dn6_slot = var_ci_dn6;
        *var_ci_dn7_slot = var_ci_dn7;
        *var_ci_dn8_slot = var_ci_dn8;
        *var_ci_dn9_slot = var_ci_dn9;
        *var_ci_rdb0_slot = var_ci_rdb0;
        *var_ci_rdb1_slot = var_ci_rdb1;
        *var_ci_rdb10_slot = var_ci_rdb10;
        *var_ci_rdb11_slot = var_ci_rdb11;
        *var_ci_rdb12_slot = var_ci_rdb12;
        *var_ci_rdb13_slot = var_ci_rdb13;
        *var_ci_rdb14_slot = var_ci_rdb14;
        *var_ci_rdb15_slot = var_ci_rdb15;
        *var_ci_rdb16_slot = var_ci_rdb16;
        *var_ci_rdb17_slot = var_ci_rdb17;
        *var_ci_rdb18_slot = var_ci_rdb18;
        *var_ci_rdb2_slot = var_ci_rdb2;
        *var_ci_rdb3_slot = var_ci_rdb3;
        *var_ci_rdb4_slot = var_ci_rdb4;
        *var_ci_rdb5_slot = var_ci_rdb5;
        *var_ci_rdb6_slot = var_ci_rdb6;
        *var_ci_rdb7_slot = var_ci_rdb7;
        *var_ci_rdb8_slot = var_ci_rdb8;
        *var_ci_rdb9_slot = var_ci_rdb9;
        *var_ci_rdn0_slot = var_ci_rdn0;
        *var_ci_rdn1_slot = var_ci_rdn1;
        *var_ci_rdn10_slot = var_ci_rdn10;
        *var_ci_rdn11_slot = var_ci_rdn11;
        *var_ci_rdn12_slot = var_ci_rdn12;
        *var_ci_rdn13_slot = var_ci_rdn13;
        *var_ci_rdn14_slot = var_ci_rdn14;
        *var_ci_rdn15_slot = var_ci_rdn15;
        *var_ci_rdn2_slot = var_ci_rdn2;
        *var_ci_rdn3_slot = var_ci_rdn3;
        *var_ci_rdn4_slot = var_ci_rdn4;
        *var_ci_rdn5_slot = var_ci_rdn5;
        *var_ci_rdn6_slot = var_ci_rdn6;
        *var_ci_rdn7_slot = var_ci_rdn7;
        *var_ci_rdn8_slot = var_ci_rdn8;
        *var_ci_rdn9_slot = var_ci_rdn9;
        *var_ci_rv_slot = var_ci_rv;
        *var_guard27_slot = var_guard27;
        *var_guard27_db0_slot = var_guard27_db0;
        *var_guard27_db1_slot = var_guard27_db1;
        *var_guard27_db10_slot = var_guard27_db10;
        *var_guard27_db11_slot = var_guard27_db11;
        *var_guard27_db12_slot = var_guard27_db12;
        *var_guard27_db13_slot = var_guard27_db13;
        *var_guard27_db14_slot = var_guard27_db14;
        *var_guard27_db15_slot = var_guard27_db15;
        *var_guard27_db16_slot = var_guard27_db16;
        *var_guard27_db17_slot = var_guard27_db17;
        *var_guard27_db18_slot = var_guard27_db18;
        *var_guard27_db2_slot = var_guard27_db2;
        *var_guard27_db3_slot = var_guard27_db3;
        *var_guard27_db4_slot = var_guard27_db4;
        *var_guard27_db5_slot = var_guard27_db5;
        *var_guard27_db6_slot = var_guard27_db6;
        *var_guard27_db7_slot = var_guard27_db7;
        *var_guard27_db8_slot = var_guard27_db8;
        *var_guard27_db9_slot = var_guard27_db9;
        *var_guard27_dn0_slot = var_guard27_dn0;
        *var_guard27_dn1_slot = var_guard27_dn1;
        *var_guard27_dn10_slot = var_guard27_dn10;
        *var_guard27_dn11_slot = var_guard27_dn11;
        *var_guard27_dn12_slot = var_guard27_dn12;
        *var_guard27_dn13_slot = var_guard27_dn13;
        *var_guard27_dn14_slot = var_guard27_dn14;
        *var_guard27_dn15_slot = var_guard27_dn15;
        *var_guard27_dn2_slot = var_guard27_dn2;
        *var_guard27_dn3_slot = var_guard27_dn3;
        *var_guard27_dn4_slot = var_guard27_dn4;
        *var_guard27_dn5_slot = var_guard27_dn5;
        *var_guard27_dn6_slot = var_guard27_dn6;
        *var_guard27_dn7_slot = var_guard27_dn7;
        *var_guard27_dn8_slot = var_guard27_dn8;
        *var_guard27_dn9_slot = var_guard27_dn9;
        *var_guard27_rdb0_slot = var_guard27_rdb0;
        *var_guard27_rdb1_slot = var_guard27_rdb1;
        *var_guard27_rdb10_slot = var_guard27_rdb10;
        *var_guard27_rdb11_slot = var_guard27_rdb11;
        *var_guard27_rdb12_slot = var_guard27_rdb12;
        *var_guard27_rdb13_slot = var_guard27_rdb13;
        *var_guard27_rdb14_slot = var_guard27_rdb14;
        *var_guard27_rdb15_slot = var_guard27_rdb15;
        *var_guard27_rdb16_slot = var_guard27_rdb16;
        *var_guard27_rdb17_slot = var_guard27_rdb17;
        *var_guard27_rdb18_slot = var_guard27_rdb18;
        *var_guard27_rdb2_slot = var_guard27_rdb2;
        *var_guard27_rdb3_slot = var_guard27_rdb3;
        *var_guard27_rdb4_slot = var_guard27_rdb4;
        *var_guard27_rdb5_slot = var_guard27_rdb5;
        *var_guard27_rdb6_slot = var_guard27_rdb6;
        *var_guard27_rdb7_slot = var_guard27_rdb7;
        *var_guard27_rdb8_slot = var_guard27_rdb8;
        *var_guard27_rdb9_slot = var_guard27_rdb9;
        *var_guard27_rdn0_slot = var_guard27_rdn0;
        *var_guard27_rdn1_slot = var_guard27_rdn1;
        *var_guard27_rdn10_slot = var_guard27_rdn10;
        *var_guard27_rdn11_slot = var_guard27_rdn11;
        *var_guard27_rdn12_slot = var_guard27_rdn12;
        *var_guard27_rdn13_slot = var_guard27_rdn13;
        *var_guard27_rdn14_slot = var_guard27_rdn14;
        *var_guard27_rdn15_slot = var_guard27_rdn15;
        *var_guard27_rdn2_slot = var_guard27_rdn2;
        *var_guard27_rdn3_slot = var_guard27_rdn3;
        *var_guard27_rdn4_slot = var_guard27_rdn4;
        *var_guard27_rdn5_slot = var_guard27_rdn5;
        *var_guard27_rdn6_slot = var_guard27_rdn6;
        *var_guard27_rdn7_slot = var_guard27_rdn7;
        *var_guard27_rdn8_slot = var_guard27_rdn8;
        *var_guard27_rdn9_slot = var_guard27_rdn9;
        *var_guard27_rv_slot = var_guard27_rv;
        *var_guard43_slot = var_guard43;
        *var_guard43_db0_slot = var_guard43_db0;
        *var_guard43_db1_slot = var_guard43_db1;
        *var_guard43_db10_slot = var_guard43_db10;
        *var_guard43_db11_slot = var_guard43_db11;
        *var_guard43_db12_slot = var_guard43_db12;
        *var_guard43_db13_slot = var_guard43_db13;
        *var_guard43_db14_slot = var_guard43_db14;
        *var_guard43_db15_slot = var_guard43_db15;
        *var_guard43_db16_slot = var_guard43_db16;
        *var_guard43_db17_slot = var_guard43_db17;
        *var_guard43_db18_slot = var_guard43_db18;
        *var_guard43_db2_slot = var_guard43_db2;
        *var_guard43_db3_slot = var_guard43_db3;
        *var_guard43_db4_slot = var_guard43_db4;
        *var_guard43_db5_slot = var_guard43_db5;
        *var_guard43_db6_slot = var_guard43_db6;
        *var_guard43_db7_slot = var_guard43_db7;
        *var_guard43_db8_slot = var_guard43_db8;
        *var_guard43_db9_slot = var_guard43_db9;
        *var_guard43_dn0_slot = var_guard43_dn0;
        *var_guard43_dn1_slot = var_guard43_dn1;
        *var_guard43_dn10_slot = var_guard43_dn10;
        *var_guard43_dn11_slot = var_guard43_dn11;
        *var_guard43_dn12_slot = var_guard43_dn12;
        *var_guard43_dn13_slot = var_guard43_dn13;
        *var_guard43_dn14_slot = var_guard43_dn14;
        *var_guard43_dn15_slot = var_guard43_dn15;
        *var_guard43_dn2_slot = var_guard43_dn2;
        *var_guard43_dn3_slot = var_guard43_dn3;
        *var_guard43_dn4_slot = var_guard43_dn4;
        *var_guard43_dn5_slot = var_guard43_dn5;
        *var_guard43_dn6_slot = var_guard43_dn6;
        *var_guard43_dn7_slot = var_guard43_dn7;
        *var_guard43_dn8_slot = var_guard43_dn8;
        *var_guard43_dn9_slot = var_guard43_dn9;
        *var_guard43_rdb0_slot = var_guard43_rdb0;
        *var_guard43_rdb1_slot = var_guard43_rdb1;
        *var_guard43_rdb10_slot = var_guard43_rdb10;
        *var_guard43_rdb11_slot = var_guard43_rdb11;
        *var_guard43_rdb12_slot = var_guard43_rdb12;
        *var_guard43_rdb13_slot = var_guard43_rdb13;
        *var_guard43_rdb14_slot = var_guard43_rdb14;
        *var_guard43_rdb15_slot = var_guard43_rdb15;
        *var_guard43_rdb16_slot = var_guard43_rdb16;
        *var_guard43_rdb17_slot = var_guard43_rdb17;
        *var_guard43_rdb18_slot = var_guard43_rdb18;
        *var_guard43_rdb2_slot = var_guard43_rdb2;
        *var_guard43_rdb3_slot = var_guard43_rdb3;
        *var_guard43_rdb4_slot = var_guard43_rdb4;
        *var_guard43_rdb5_slot = var_guard43_rdb5;
        *var_guard43_rdb6_slot = var_guard43_rdb6;
        *var_guard43_rdb7_slot = var_guard43_rdb7;
        *var_guard43_rdb8_slot = var_guard43_rdb8;
        *var_guard43_rdb9_slot = var_guard43_rdb9;
        *var_guard43_rdn0_slot = var_guard43_rdn0;
        *var_guard43_rdn1_slot = var_guard43_rdn1;
        *var_guard43_rdn10_slot = var_guard43_rdn10;
        *var_guard43_rdn11_slot = var_guard43_rdn11;
        *var_guard43_rdn12_slot = var_guard43_rdn12;
        *var_guard43_rdn13_slot = var_guard43_rdn13;
        *var_guard43_rdn14_slot = var_guard43_rdn14;
        *var_guard43_rdn15_slot = var_guard43_rdn15;
        *var_guard43_rdn2_slot = var_guard43_rdn2;
        *var_guard43_rdn3_slot = var_guard43_rdn3;
        *var_guard43_rdn4_slot = var_guard43_rdn4;
        *var_guard43_rdn5_slot = var_guard43_rdn5;
        *var_guard43_rdn6_slot = var_guard43_rdn6;
        *var_guard43_rdn7_slot = var_guard43_rdn7;
        *var_guard43_rdn8_slot = var_guard43_rdn8;
        *var_guard43_rdn9_slot = var_guard43_rdn9;
        *var_guard43_rv_slot = var_guard43_rv;
        *var_k_slot = var_k;
        *var_k_db0_slot = var_k_db0;
        *var_k_db1_slot = var_k_db1;
        *var_k_db10_slot = var_k_db10;
        *var_k_db11_slot = var_k_db11;
        *var_k_db12_slot = var_k_db12;
        *var_k_db13_slot = var_k_db13;
        *var_k_db14_slot = var_k_db14;
        *var_k_db15_slot = var_k_db15;
        *var_k_db16_slot = var_k_db16;
        *var_k_db17_slot = var_k_db17;
        *var_k_db18_slot = var_k_db18;
        *var_k_db2_slot = var_k_db2;
        *var_k_db3_slot = var_k_db3;
        *var_k_db4_slot = var_k_db4;
        *var_k_db5_slot = var_k_db5;
        *var_k_db6_slot = var_k_db6;
        *var_k_db7_slot = var_k_db7;
        *var_k_db8_slot = var_k_db8;
        *var_k_db9_slot = var_k_db9;
        *var_k_dn0_slot = var_k_dn0;
        *var_k_dn1_slot = var_k_dn1;
        *var_k_dn10_slot = var_k_dn10;
        *var_k_dn11_slot = var_k_dn11;
        *var_k_dn12_slot = var_k_dn12;
        *var_k_dn13_slot = var_k_dn13;
        *var_k_dn14_slot = var_k_dn14;
        *var_k_dn15_slot = var_k_dn15;
        *var_k_dn2_slot = var_k_dn2;
        *var_k_dn3_slot = var_k_dn3;
        *var_k_dn4_slot = var_k_dn4;
        *var_k_dn5_slot = var_k_dn5;
        *var_k_dn6_slot = var_k_dn6;
        *var_k_dn7_slot = var_k_dn7;
        *var_k_dn8_slot = var_k_dn8;
        *var_k_dn9_slot = var_k_dn9;
        *var_k_rdb0_slot = var_k_rdb0;
        *var_k_rdb1_slot = var_k_rdb1;
        *var_k_rdb10_slot = var_k_rdb10;
        *var_k_rdb11_slot = var_k_rdb11;
        *var_k_rdb12_slot = var_k_rdb12;
        *var_k_rdb13_slot = var_k_rdb13;
        *var_k_rdb14_slot = var_k_rdb14;
        *var_k_rdb15_slot = var_k_rdb15;
        *var_k_rdb16_slot = var_k_rdb16;
        *var_k_rdb17_slot = var_k_rdb17;
        *var_k_rdb18_slot = var_k_rdb18;
        *var_k_rdb2_slot = var_k_rdb2;
        *var_k_rdb3_slot = var_k_rdb3;
        *var_k_rdb4_slot = var_k_rdb4;
        *var_k_rdb5_slot = var_k_rdb5;
        *var_k_rdb6_slot = var_k_rdb6;
        *var_k_rdb7_slot = var_k_rdb7;
        *var_k_rdb8_slot = var_k_rdb8;
        *var_k_rdb9_slot = var_k_rdb9;
        *var_k_rdn0_slot = var_k_rdn0;
        *var_k_rdn1_slot = var_k_rdn1;
        *var_k_rdn10_slot = var_k_rdn10;
        *var_k_rdn11_slot = var_k_rdn11;
        *var_k_rdn12_slot = var_k_rdn12;
        *var_k_rdn13_slot = var_k_rdn13;
        *var_k_rdn14_slot = var_k_rdn14;
        *var_k_rdn15_slot = var_k_rdn15;
        *var_k_rdn2_slot = var_k_rdn2;
        *var_k_rdn3_slot = var_k_rdn3;
        *var_k_rdn4_slot = var_k_rdn4;
        *var_k_rdn5_slot = var_k_rdn5;
        *var_k_rdn6_slot = var_k_rdn6;
        *var_k_rdn7_slot = var_k_rdn7;
        *var_k_rdn8_slot = var_k_rdn8;
        *var_k_rdn9_slot = var_k_rdn9;
        *var_k_rv_slot = var_k_rv;
    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
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
        var_cgd: f64,
        var_cgd_db0: f64,
        var_cgd_db1: f64,
        var_cgd_db10: f64,
        var_cgd_db11: f64,
        var_cgd_db12: f64,
        var_cgd_db13: f64,
        var_cgd_db14: f64,
        var_cgd_db15: f64,
        var_cgd_db16: f64,
        var_cgd_db17: f64,
        var_cgd_db18: f64,
        var_cgd_db2: f64,
        var_cgd_db3: f64,
        var_cgd_db4: f64,
        var_cgd_db5: f64,
        var_cgd_db6: f64,
        var_cgd_db7: f64,
        var_cgd_db8: f64,
        var_cgd_db9: f64,
        var_cgd_dn0: f64,
        var_cgd_dn1: f64,
        var_cgd_dn10: f64,
        var_cgd_dn11: f64,
        var_cgd_dn12: f64,
        var_cgd_dn13: f64,
        var_cgd_dn14: f64,
        var_cgd_dn15: f64,
        var_cgd_dn2: f64,
        var_cgd_dn3: f64,
        var_cgd_dn4: f64,
        var_cgd_dn5: f64,
        var_cgd_dn6: f64,
        var_cgd_dn7: f64,
        var_cgd_dn8: f64,
        var_cgd_dn9: f64,
        var_cgs: f64,
        var_cgs_db0: f64,
        var_cgs_db1: f64,
        var_cgs_db10: f64,
        var_cgs_db11: f64,
        var_cgs_db12: f64,
        var_cgs_db13: f64,
        var_cgs_db14: f64,
        var_cgs_db15: f64,
        var_cgs_db16: f64,
        var_cgs_db17: f64,
        var_cgs_db18: f64,
        var_cgs_db2: f64,
        var_cgs_db3: f64,
        var_cgs_db4: f64,
        var_cgs_db5: f64,
        var_cgs_db6: f64,
        var_cgs_db7: f64,
        var_cgs_db8: f64,
        var_cgs_db9: f64,
        var_cgs_dn0: f64,
        var_cgs_dn1: f64,
        var_cgs_dn10: f64,
        var_cgs_dn11: f64,
        var_cgs_dn12: f64,
        var_cgs_dn13: f64,
        var_cgs_dn14: f64,
        var_cgs_dn15: f64,
        var_cgs_dn2: f64,
        var_cgs_dn3: f64,
        var_cgs_dn4: f64,
        var_cgs_dn5: f64,
        var_cgs_dn6: f64,
        var_cgs_dn7: f64,
        var_cgs_dn8: f64,
        var_cgs_dn9: f64,
        var_ci: f64,
        var_ci_db0: f64,
        var_ci_db1: f64,
        var_ci_db10: f64,
        var_ci_db11: f64,
        var_ci_db12: f64,
        var_ci_db13: f64,
        var_ci_db14: f64,
        var_ci_db2: f64,
        var_ci_db3: f64,
        var_ci_db4: f64,
        var_ci_db5: f64,
        var_ci_db6: f64,
        var_ci_db7: f64,
        var_ci_db8: f64,
        var_ci_db9: f64,
        var_ci_dn0: f64,
        var_ci_dn1: f64,
        var_ci_dn10: f64,
        var_ci_dn11: f64,
        var_ci_dn12: f64,
        var_ci_dn13: f64,
        var_ci_dn14: f64,
        var_ci_dn15: f64,
        var_ci_dn2: f64,
        var_ci_dn3: f64,
        var_ci_dn4: f64,
        var_ci_dn5: f64,
        var_ci_dn6: f64,
        var_ci_dn7: f64,
        var_ci_dn8: f64,
        var_ci_dn9: f64,
        var_guard16: f64,
        var_guard21: f64,
        var_guard22: f64,
        var_guard23: f64,
        var_guard24: f64,
        var_guard25: f64,
        var_guard26: f64,
        var_guard27: f64,
        var_guard43: f64,
        var_qgd: f64,
        var_qgd_db0: f64,
        var_qgd_db1: f64,
        var_qgd_db10: f64,
        var_qgd_db11: f64,
        var_qgd_db12: f64,
        var_qgd_db13: f64,
        var_qgd_db14: f64,
        var_qgd_db2: f64,
        var_qgd_db3: f64,
        var_qgd_db4: f64,
        var_qgd_db5: f64,
        var_qgd_db6: f64,
        var_qgd_db7: f64,
        var_qgd_db8: f64,
        var_qgd_db9: f64,
        var_qgd_dn0: f64,
        var_qgd_dn1: f64,
        var_qgd_dn10: f64,
        var_qgd_dn11: f64,
        var_qgd_dn12: f64,
        var_qgd_dn13: f64,
        var_qgd_dn14: f64,
        var_qgd_dn15: f64,
        var_qgd_dn2: f64,
        var_qgd_dn3: f64,
        var_qgd_dn4: f64,
        var_qgd_dn5: f64,
        var_qgd_dn6: f64,
        var_qgd_dn7: f64,
        var_qgd_dn8: f64,
        var_qgd_dn9: f64,
        var_qgs: f64,
        var_qgs_db0: f64,
        var_qgs_db1: f64,
        var_qgs_db10: f64,
        var_qgs_db11: f64,
        var_qgs_db12: f64,
        var_qgs_db13: f64,
        var_qgs_db14: f64,
        var_qgs_db2: f64,
        var_qgs_db3: f64,
        var_qgs_db4: f64,
        var_qgs_db5: f64,
        var_qgs_db6: f64,
        var_qgs_db7: f64,
        var_qgs_db8: f64,
        var_qgs_db9: f64,
        var_qgs_dn0: f64,
        var_qgs_dn1: f64,
        var_qgs_dn10: f64,
        var_qgs_dn11: f64,
        var_qgs_dn12: f64,
        var_qgs_dn13: f64,
        var_qgs_dn14: f64,
        var_qgs_dn15: f64,
        var_qgs_dn2: f64,
        var_qgs_dn3: f64,
        var_qgs_dn4: f64,
        var_qgs_dn5: f64,
        var_qgs_dn6: f64,
        var_qgs_dn7: f64,
        var_qgs_dn8: f64,
        var_qgs_dn9: f64,
        var_rd1_t: f64,
        var_rd1_t_db0: f64,
        var_rd1_t_db1: f64,
        var_rd1_t_db10: f64,
        var_rd1_t_db11: f64,
        var_rd1_t_db12: f64,
        var_rd1_t_db13: f64,
        var_rd1_t_db14: f64,
        var_rd1_t_db2: f64,
        var_rd1_t_db3: f64,
        var_rd1_t_db4: f64,
        var_rd1_t_db5: f64,
        var_rd1_t_db6: f64,
        var_rd1_t_db7: f64,
        var_rd1_t_db8: f64,
        var_rd1_t_db9: f64,
        var_rd1_t_dn0: f64,
        var_rd1_t_dn1: f64,
        var_rd1_t_dn10: f64,
        var_rd1_t_dn11: f64,
        var_rd1_t_dn12: f64,
        var_rd1_t_dn13: f64,
        var_rd1_t_dn14: f64,
        var_rd1_t_dn15: f64,
        var_rd1_t_dn2: f64,
        var_rd1_t_dn3: f64,
        var_rd1_t_dn4: f64,
        var_rd1_t_dn5: f64,
        var_rd1_t_dn6: f64,
        var_rd1_t_dn7: f64,
        var_rd1_t_dn8: f64,
        var_rd1_t_dn9: f64,
        var_rs_t: f64,
        var_rs_t_db0: f64,
        var_rs_t_db1: f64,
        var_rs_t_db10: f64,
        var_rs_t_db11: f64,
        var_rs_t_db12: f64,
        var_rs_t_db13: f64,
        var_rs_t_db14: f64,
        var_rs_t_db2: f64,
        var_rs_t_db3: f64,
        var_rs_t_db4: f64,
        var_rs_t_db5: f64,
        var_rs_t_db6: f64,
        var_rs_t_db7: f64,
        var_rs_t_db8: f64,
        var_rs_t_db9: f64,
        var_rs_t_dn0: f64,
        var_rs_t_dn1: f64,
        var_rs_t_dn10: f64,
        var_rs_t_dn11: f64,
        var_rs_t_dn12: f64,
        var_rs_t_dn13: f64,
        var_rs_t_dn14: f64,
        var_rs_t_dn15: f64,
        var_rs_t_dn2: f64,
        var_rs_t_dn3: f64,
        var_rs_t_dn4: f64,
        var_rs_t_dn5: f64,
        var_rs_t_dn6: f64,
        var_rs_t_dn7: f64,
        var_rs_t_dn8: f64,
        var_rs_t_dn9: f64,
        var_vgdc: f64,
        var_vgdc_db0: f64,
        var_vgdc_db1: f64,
        var_vgdc_db10: f64,
        var_vgdc_db11: f64,
        var_vgdc_db12: f64,
        var_vgdc_db13: f64,
        var_vgdc_db14: f64,
        var_vgdc_db15: f64,
        var_vgdc_db16: f64,
        var_vgdc_db17: f64,
        var_vgdc_db18: f64,
        var_vgdc_db2: f64,
        var_vgdc_db3: f64,
        var_vgdc_db4: f64,
        var_vgdc_db5: f64,
        var_vgdc_db6: f64,
        var_vgdc_db7: f64,
        var_vgdc_db8: f64,
        var_vgdc_db9: f64,
        var_vgdc_dn0: f64,
        var_vgdc_dn1: f64,
        var_vgdc_dn10: f64,
        var_vgdc_dn11: f64,
        var_vgdc_dn12: f64,
        var_vgdc_dn13: f64,
        var_vgdc_dn14: f64,
        var_vgdc_dn15: f64,
        var_vgdc_dn2: f64,
        var_vgdc_dn3: f64,
        var_vgdc_dn4: f64,
        var_vgdc_dn5: f64,
        var_vgdc_dn6: f64,
        var_vgdc_dn7: f64,
        var_vgdc_dn8: f64,
        var_vgdc_dn9: f64,
        var_vgsc: f64,
        var_vgsc_db0: f64,
        var_vgsc_db1: f64,
        var_vgsc_db10: f64,
        var_vgsc_db11: f64,
        var_vgsc_db12: f64,
        var_vgsc_db13: f64,
        var_vgsc_db14: f64,
        var_vgsc_db15: f64,
        var_vgsc_db16: f64,
        var_vgsc_db17: f64,
        var_vgsc_db18: f64,
        var_vgsc_db2: f64,
        var_vgsc_db3: f64,
        var_vgsc_db4: f64,
        var_vgsc_db5: f64,
        var_vgsc_db6: f64,
        var_vgsc_db7: f64,
        var_vgsc_db8: f64,
        var_vgsc_db9: f64,
        var_vgsc_dn0: f64,
        var_vgsc_dn1: f64,
        var_vgsc_dn10: f64,
        var_vgsc_dn11: f64,
        var_vgsc_dn12: f64,
        var_vgsc_dn13: f64,
        var_vgsc_dn14: f64,
        var_vgsc_dn15: f64,
        var_vgsc_dn2: f64,
        var_vgsc_dn3: f64,
        var_vgsc_dn4: f64,
        var_vgsc_dn5: f64,
        var_vgsc_dn6: f64,
        var_vgsc_dn7: f64,
        var_vgsc_dn8: f64,
        var_vgsc_dn9: f64,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let bi0 = ctx.branch_current(branches[0]);
        let bi5 = ctx.branch_current(branches[5]);
        let bi10 = ctx.branch_current(branches[10]);
        let bi13 = ctx.branch_current(branches[13]);
        let bi14 = ctx.branch_current(branches[14]);
        let eq3_e99: f64 = (p.p51 / 3.0);
        let eq3_e101: f64 = (eq3_e99 * bi0);
        let eq3_e102: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq3_e101);
        let eq3_value: f64 = eq3_e102;
        stamper.stamp_potential_branch1_local(
            0,
            eq3_value,
            0,
            (eq3_e99 * ddt_scale),
        );
        let (eq7_e110, eq7_e110_d_n0, eq7_e110_d_n1, eq7_e110_d_n2, eq7_e110_d_n3, eq7_e110_d_n4, eq7_e110_d_n5, eq7_e110_d_n6, eq7_e110_d_n7, eq7_e110_d_n8, eq7_e110_d_n9, eq7_e110_d_n10, eq7_e110_d_n11, eq7_e110_d_n12, eq7_e110_d_n13, eq7_e110_d_n14, eq7_e110_d_n15, eq7_e110_d_b0, eq7_e110_d_b1, eq7_e110_d_b2, eq7_e110_d_b3, eq7_e110_d_b4, eq7_e110_d_b5, eq7_e110_d_b6, eq7_e110_d_b7, eq7_e110_d_b8, eq7_e110_d_b9, eq7_e110_d_b10, eq7_e110_d_b11, eq7_e110_d_b12, eq7_e110_d_b13, eq7_e110_d_b14,) = {
    if (var_guard16 != 0.0) {
        let eq7_e108: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_qgd);
        (eq7_e108, (var_qgd_dn0 * ddt_scale), (var_qgd_dn1 * ddt_scale), (var_qgd_dn2 * ddt_scale), (var_qgd_dn3 * ddt_scale), (var_qgd_dn4 * ddt_scale), (var_qgd_dn5 * ddt_scale), (var_qgd_dn6 * ddt_scale), (var_qgd_dn7 * ddt_scale), (var_qgd_dn8 * ddt_scale), (var_qgd_dn9 * ddt_scale), (var_qgd_dn10 * ddt_scale), (var_qgd_dn11 * ddt_scale), (var_qgd_dn12 * ddt_scale), (var_qgd_dn13 * ddt_scale), (var_qgd_dn14 * ddt_scale), (var_qgd_dn15 * ddt_scale), (var_qgd_db0 * ddt_scale), (var_qgd_db1 * ddt_scale), (var_qgd_db2 * ddt_scale), (var_qgd_db3 * ddt_scale), (var_qgd_db4 * ddt_scale), (var_qgd_db5 * ddt_scale), (var_qgd_db6 * ddt_scale), (var_qgd_db7 * ddt_scale), (var_qgd_db8 * ddt_scale), (var_qgd_db9 * ddt_scale), (var_qgd_db10 * ddt_scale), (var_qgd_db11 * ddt_scale), (var_qgd_db12 * ddt_scale), (var_qgd_db13 * ddt_scale), (var_qgd_db14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e110;
        let eq7_node_derivatives: [f64; 16] = [eq7_e110_d_n0, eq7_e110_d_n1, eq7_e110_d_n2, eq7_e110_d_n3, eq7_e110_d_n4, eq7_e110_d_n5, eq7_e110_d_n6, eq7_e110_d_n7, eq7_e110_d_n8, eq7_e110_d_n9, eq7_e110_d_n10, eq7_e110_d_n11, eq7_e110_d_n12, eq7_e110_d_n13, eq7_e110_d_n14, eq7_e110_d_n15];
        let eq7_branch_derivatives: [f64; 15] = [eq7_e110_d_b0, eq7_e110_d_b1, eq7_e110_d_b2, eq7_e110_d_b3, eq7_e110_d_b4, eq7_e110_d_b5, eq7_e110_d_b6, eq7_e110_d_b7, eq7_e110_d_b8, eq7_e110_d_b9, eq7_e110_d_b10, eq7_e110_d_b11, eq7_e110_d_b12, eq7_e110_d_b13, eq7_e110_d_b14];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e115, eq8_e115_d_n0, eq8_e115_d_n1, eq8_e115_d_n2, eq8_e115_d_n3, eq8_e115_d_n4, eq8_e115_d_n5, eq8_e115_d_n6, eq8_e115_d_n7, eq8_e115_d_n8, eq8_e115_d_n9, eq8_e115_d_n10, eq8_e115_d_n11, eq8_e115_d_n12, eq8_e115_d_n13, eq8_e115_d_n14, eq8_e115_d_n15, eq8_e115_d_b0, eq8_e115_d_b1, eq8_e115_d_b2, eq8_e115_d_b3, eq8_e115_d_b4, eq8_e115_d_b5, eq8_e115_d_b6, eq8_e115_d_b7, eq8_e115_d_b8, eq8_e115_d_b9, eq8_e115_d_b10, eq8_e115_d_b11, eq8_e115_d_b12, eq8_e115_d_b13, eq8_e115_d_b14,) = {
    if (var_guard16 != 0.0) {
        let eq8_e113: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, var_qgs);
        (eq8_e113, (var_qgs_dn0 * ddt_scale), (var_qgs_dn1 * ddt_scale), (var_qgs_dn2 * ddt_scale), (var_qgs_dn3 * ddt_scale), (var_qgs_dn4 * ddt_scale), (var_qgs_dn5 * ddt_scale), (var_qgs_dn6 * ddt_scale), (var_qgs_dn7 * ddt_scale), (var_qgs_dn8 * ddt_scale), (var_qgs_dn9 * ddt_scale), (var_qgs_dn10 * ddt_scale), (var_qgs_dn11 * ddt_scale), (var_qgs_dn12 * ddt_scale), (var_qgs_dn13 * ddt_scale), (var_qgs_dn14 * ddt_scale), (var_qgs_dn15 * ddt_scale), (var_qgs_db0 * ddt_scale), (var_qgs_db1 * ddt_scale), (var_qgs_db2 * ddt_scale), (var_qgs_db3 * ddt_scale), (var_qgs_db4 * ddt_scale), (var_qgs_db5 * ddt_scale), (var_qgs_db6 * ddt_scale), (var_qgs_db7 * ddt_scale), (var_qgs_db8 * ddt_scale), (var_qgs_db9 * ddt_scale), (var_qgs_db10 * ddt_scale), (var_qgs_db11 * ddt_scale), (var_qgs_db12 * ddt_scale), (var_qgs_db13 * ddt_scale), (var_qgs_db14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e115;
        let eq8_node_derivatives: [f64; 16] = [eq8_e115_d_n0, eq8_e115_d_n1, eq8_e115_d_n2, eq8_e115_d_n3, eq8_e115_d_n4, eq8_e115_d_n5, eq8_e115_d_n6, eq8_e115_d_n7, eq8_e115_d_n8, eq8_e115_d_n9, eq8_e115_d_n10, eq8_e115_d_n11, eq8_e115_d_n12, eq8_e115_d_n13, eq8_e115_d_n14, eq8_e115_d_n15];
        let eq8_branch_derivatives: [f64; 15] = [eq8_e115_d_b0, eq8_e115_d_b1, eq8_e115_d_b2, eq8_e115_d_b3, eq8_e115_d_b4, eq8_e115_d_b5, eq8_e115_d_b6, eq8_e115_d_b7, eq8_e115_d_b8, eq8_e115_d_b9, eq8_e115_d_b10, eq8_e115_d_b11, eq8_e115_d_b12, eq8_e115_d_b13, eq8_e115_d_b14];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e123, eq9_e123_d_n0, eq9_e123_d_n1, eq9_e123_d_n2, eq9_e123_d_n3, eq9_e123_d_n4, eq9_e123_d_n5, eq9_e123_d_n6, eq9_e123_d_n7, eq9_e123_d_n8, eq9_e123_d_n9, eq9_e123_d_n10, eq9_e123_d_n11, eq9_e123_d_n12, eq9_e123_d_n13, eq9_e123_d_n14, eq9_e123_d_n15, eq9_e123_d_b0, eq9_e123_d_b1, eq9_e123_d_b2, eq9_e123_d_b3, eq9_e123_d_b4, eq9_e123_d_b5, eq9_e123_d_b6, eq9_e123_d_b7, eq9_e123_d_b8, eq9_e123_d_b9, eq9_e123_d_b10, eq9_e123_d_b11, eq9_e123_d_b12, eq9_e123_d_b13, eq9_e123_d_b14,) = {
    if (var_guard16 == 0.0) {
        let eq9_e120: f64 = (var_cgd * var_vgdc);
        let eq9_e120_d_n0: f64 = ((var_cgd_dn0 * var_vgdc) + (var_cgd * var_vgdc_dn0));
        let eq9_e120_d_n1: f64 = ((var_cgd_dn1 * var_vgdc) + (var_cgd * var_vgdc_dn1));
        let eq9_e120_d_n2: f64 = ((var_cgd_dn2 * var_vgdc) + (var_cgd * var_vgdc_dn2));
        let eq9_e120_d_n3: f64 = ((var_cgd_dn3 * var_vgdc) + (var_cgd * var_vgdc_dn3));
        let eq9_e120_d_n4: f64 = ((var_cgd_dn4 * var_vgdc) + (var_cgd * var_vgdc_dn4));
        let eq9_e120_d_n5: f64 = ((var_cgd_dn5 * var_vgdc) + (var_cgd * var_vgdc_dn5));
        let eq9_e120_d_n6: f64 = ((var_cgd_dn6 * var_vgdc) + (var_cgd * var_vgdc_dn6));
        let eq9_e120_d_n7: f64 = ((var_cgd_dn7 * var_vgdc) + (var_cgd * var_vgdc_dn7));
        let eq9_e120_d_n8: f64 = ((var_cgd_dn8 * var_vgdc) + (var_cgd * var_vgdc_dn8));
        let eq9_e120_d_n9: f64 = ((var_cgd_dn9 * var_vgdc) + (var_cgd * var_vgdc_dn9));
        let eq9_e120_d_n10: f64 = ((var_cgd_dn10 * var_vgdc) + (var_cgd * var_vgdc_dn10));
        let eq9_e120_d_n11: f64 = ((var_cgd_dn11 * var_vgdc) + (var_cgd * var_vgdc_dn11));
        let eq9_e120_d_n12: f64 = ((var_cgd_dn12 * var_vgdc) + (var_cgd * var_vgdc_dn12));
        let eq9_e120_d_n13: f64 = ((var_cgd_dn13 * var_vgdc) + (var_cgd * var_vgdc_dn13));
        let eq9_e120_d_n14: f64 = ((var_cgd_dn14 * var_vgdc) + (var_cgd * var_vgdc_dn14));
        let eq9_e120_d_n15: f64 = ((var_cgd_dn15 * var_vgdc) + (var_cgd * var_vgdc_dn15));
        let eq9_e120_d_b0: f64 = ((var_cgd_db0 * var_vgdc) + (var_cgd * var_vgdc_db0));
        let eq9_e120_d_b1: f64 = ((var_cgd_db1 * var_vgdc) + (var_cgd * var_vgdc_db1));
        let eq9_e120_d_b2: f64 = ((var_cgd_db2 * var_vgdc) + (var_cgd * var_vgdc_db2));
        let eq9_e120_d_b3: f64 = ((var_cgd_db3 * var_vgdc) + (var_cgd * var_vgdc_db3));
        let eq9_e120_d_b4: f64 = ((var_cgd_db4 * var_vgdc) + (var_cgd * var_vgdc_db4));
        let eq9_e120_d_b5: f64 = ((var_cgd_db5 * var_vgdc) + (var_cgd * var_vgdc_db5));
        let eq9_e120_d_b6: f64 = ((var_cgd_db6 * var_vgdc) + (var_cgd * var_vgdc_db6));
        let eq9_e120_d_b7: f64 = ((var_cgd_db7 * var_vgdc) + (var_cgd * var_vgdc_db7));
        let eq9_e120_d_b8: f64 = ((var_cgd_db8 * var_vgdc) + (var_cgd * var_vgdc_db8));
        let eq9_e120_d_b9: f64 = ((var_cgd_db9 * var_vgdc) + (var_cgd * var_vgdc_db9));
        let eq9_e120_d_b10: f64 = ((var_cgd_db10 * var_vgdc) + (var_cgd * var_vgdc_db10));
        let eq9_e120_d_b11: f64 = ((var_cgd_db11 * var_vgdc) + (var_cgd * var_vgdc_db11));
        let eq9_e120_d_b12: f64 = ((var_cgd_db12 * var_vgdc) + (var_cgd * var_vgdc_db12));
        let eq9_e120_d_b13: f64 = ((var_cgd_db13 * var_vgdc) + (var_cgd * var_vgdc_db13));
        let eq9_e120_d_b14: f64 = ((var_cgd_db14 * var_vgdc) + (var_cgd * var_vgdc_db14));
        let eq9_e120_d_b15: f64 = ((var_cgd_db15 * var_vgdc) + (var_cgd * var_vgdc_db15));
        let eq9_e120_d_b16: f64 = ((var_cgd_db16 * var_vgdc) + (var_cgd * var_vgdc_db16));
        let eq9_e120_d_b17: f64 = ((var_cgd_db17 * var_vgdc) + (var_cgd * var_vgdc_db17));
        let eq9_e120_d_b18: f64 = ((var_cgd_db18 * var_vgdc) + (var_cgd * var_vgdc_db18));
        let eq9_e121: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq9_e120);
        (eq9_e121, (eq9_e120_d_n0 * ddt_scale), (eq9_e120_d_n1 * ddt_scale), (eq9_e120_d_n2 * ddt_scale), (eq9_e120_d_n3 * ddt_scale), (eq9_e120_d_n4 * ddt_scale), (eq9_e120_d_n5 * ddt_scale), (eq9_e120_d_n6 * ddt_scale), (eq9_e120_d_n7 * ddt_scale), (eq9_e120_d_n8 * ddt_scale), (eq9_e120_d_n9 * ddt_scale), (eq9_e120_d_n10 * ddt_scale), (eq9_e120_d_n11 * ddt_scale), (eq9_e120_d_n12 * ddt_scale), (eq9_e120_d_n13 * ddt_scale), (eq9_e120_d_n14 * ddt_scale), (eq9_e120_d_n15 * ddt_scale), (eq9_e120_d_b0 * ddt_scale), (eq9_e120_d_b1 * ddt_scale), (eq9_e120_d_b2 * ddt_scale), (eq9_e120_d_b3 * ddt_scale), (eq9_e120_d_b4 * ddt_scale), (eq9_e120_d_b5 * ddt_scale), (eq9_e120_d_b6 * ddt_scale), (eq9_e120_d_b7 * ddt_scale), (eq9_e120_d_b8 * ddt_scale), (eq9_e120_d_b9 * ddt_scale), (eq9_e120_d_b10 * ddt_scale), (eq9_e120_d_b11 * ddt_scale), (eq9_e120_d_b12 * ddt_scale), (eq9_e120_d_b13 * ddt_scale), (eq9_e120_d_b14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e123;
        let eq9_node_derivatives: [f64; 16] = [eq9_e123_d_n0, eq9_e123_d_n1, eq9_e123_d_n2, eq9_e123_d_n3, eq9_e123_d_n4, eq9_e123_d_n5, eq9_e123_d_n6, eq9_e123_d_n7, eq9_e123_d_n8, eq9_e123_d_n9, eq9_e123_d_n10, eq9_e123_d_n11, eq9_e123_d_n12, eq9_e123_d_n13, eq9_e123_d_n14, eq9_e123_d_n15];
        let eq9_branch_derivatives: [f64; 15] = [eq9_e123_d_b0, eq9_e123_d_b1, eq9_e123_d_b2, eq9_e123_d_b3, eq9_e123_d_b4, eq9_e123_d_b5, eq9_e123_d_b6, eq9_e123_d_b7, eq9_e123_d_b8, eq9_e123_d_b9, eq9_e123_d_b10, eq9_e123_d_b11, eq9_e123_d_b12, eq9_e123_d_b13, eq9_e123_d_b14];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let (eq10_e131, eq10_e131_d_n0, eq10_e131_d_n1, eq10_e131_d_n2, eq10_e131_d_n3, eq10_e131_d_n4, eq10_e131_d_n5, eq10_e131_d_n6, eq10_e131_d_n7, eq10_e131_d_n8, eq10_e131_d_n9, eq10_e131_d_n10, eq10_e131_d_n11, eq10_e131_d_n12, eq10_e131_d_n13, eq10_e131_d_n14, eq10_e131_d_n15, eq10_e131_d_b0, eq10_e131_d_b1, eq10_e131_d_b2, eq10_e131_d_b3, eq10_e131_d_b4, eq10_e131_d_b5, eq10_e131_d_b6, eq10_e131_d_b7, eq10_e131_d_b8, eq10_e131_d_b9, eq10_e131_d_b10, eq10_e131_d_b11, eq10_e131_d_b12, eq10_e131_d_b13, eq10_e131_d_b14,) = {
    if (var_guard16 == 0.0) {
        let eq10_e128: f64 = (var_cgs * var_vgsc);
        let eq10_e128_d_n0: f64 = ((var_cgs_dn0 * var_vgsc) + (var_cgs * var_vgsc_dn0));
        let eq10_e128_d_n1: f64 = ((var_cgs_dn1 * var_vgsc) + (var_cgs * var_vgsc_dn1));
        let eq10_e128_d_n2: f64 = ((var_cgs_dn2 * var_vgsc) + (var_cgs * var_vgsc_dn2));
        let eq10_e128_d_n3: f64 = ((var_cgs_dn3 * var_vgsc) + (var_cgs * var_vgsc_dn3));
        let eq10_e128_d_n4: f64 = ((var_cgs_dn4 * var_vgsc) + (var_cgs * var_vgsc_dn4));
        let eq10_e128_d_n5: f64 = ((var_cgs_dn5 * var_vgsc) + (var_cgs * var_vgsc_dn5));
        let eq10_e128_d_n6: f64 = ((var_cgs_dn6 * var_vgsc) + (var_cgs * var_vgsc_dn6));
        let eq10_e128_d_n7: f64 = ((var_cgs_dn7 * var_vgsc) + (var_cgs * var_vgsc_dn7));
        let eq10_e128_d_n8: f64 = ((var_cgs_dn8 * var_vgsc) + (var_cgs * var_vgsc_dn8));
        let eq10_e128_d_n9: f64 = ((var_cgs_dn9 * var_vgsc) + (var_cgs * var_vgsc_dn9));
        let eq10_e128_d_n10: f64 = ((var_cgs_dn10 * var_vgsc) + (var_cgs * var_vgsc_dn10));
        let eq10_e128_d_n11: f64 = ((var_cgs_dn11 * var_vgsc) + (var_cgs * var_vgsc_dn11));
        let eq10_e128_d_n12: f64 = ((var_cgs_dn12 * var_vgsc) + (var_cgs * var_vgsc_dn12));
        let eq10_e128_d_n13: f64 = ((var_cgs_dn13 * var_vgsc) + (var_cgs * var_vgsc_dn13));
        let eq10_e128_d_n14: f64 = ((var_cgs_dn14 * var_vgsc) + (var_cgs * var_vgsc_dn14));
        let eq10_e128_d_n15: f64 = ((var_cgs_dn15 * var_vgsc) + (var_cgs * var_vgsc_dn15));
        let eq10_e128_d_b0: f64 = ((var_cgs_db0 * var_vgsc) + (var_cgs * var_vgsc_db0));
        let eq10_e128_d_b1: f64 = ((var_cgs_db1 * var_vgsc) + (var_cgs * var_vgsc_db1));
        let eq10_e128_d_b2: f64 = ((var_cgs_db2 * var_vgsc) + (var_cgs * var_vgsc_db2));
        let eq10_e128_d_b3: f64 = ((var_cgs_db3 * var_vgsc) + (var_cgs * var_vgsc_db3));
        let eq10_e128_d_b4: f64 = ((var_cgs_db4 * var_vgsc) + (var_cgs * var_vgsc_db4));
        let eq10_e128_d_b5: f64 = ((var_cgs_db5 * var_vgsc) + (var_cgs * var_vgsc_db5));
        let eq10_e128_d_b6: f64 = ((var_cgs_db6 * var_vgsc) + (var_cgs * var_vgsc_db6));
        let eq10_e128_d_b7: f64 = ((var_cgs_db7 * var_vgsc) + (var_cgs * var_vgsc_db7));
        let eq10_e128_d_b8: f64 = ((var_cgs_db8 * var_vgsc) + (var_cgs * var_vgsc_db8));
        let eq10_e128_d_b9: f64 = ((var_cgs_db9 * var_vgsc) + (var_cgs * var_vgsc_db9));
        let eq10_e128_d_b10: f64 = ((var_cgs_db10 * var_vgsc) + (var_cgs * var_vgsc_db10));
        let eq10_e128_d_b11: f64 = ((var_cgs_db11 * var_vgsc) + (var_cgs * var_vgsc_db11));
        let eq10_e128_d_b12: f64 = ((var_cgs_db12 * var_vgsc) + (var_cgs * var_vgsc_db12));
        let eq10_e128_d_b13: f64 = ((var_cgs_db13 * var_vgsc) + (var_cgs * var_vgsc_db13));
        let eq10_e128_d_b14: f64 = ((var_cgs_db14 * var_vgsc) + (var_cgs * var_vgsc_db14));
        let eq10_e128_d_b15: f64 = ((var_cgs_db15 * var_vgsc) + (var_cgs * var_vgsc_db15));
        let eq10_e128_d_b16: f64 = ((var_cgs_db16 * var_vgsc) + (var_cgs * var_vgsc_db16));
        let eq10_e128_d_b17: f64 = ((var_cgs_db17 * var_vgsc) + (var_cgs * var_vgsc_db17));
        let eq10_e128_d_b18: f64 = ((var_cgs_db18 * var_vgsc) + (var_cgs * var_vgsc_db18));
        let eq10_e129: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq10_e128);
        (eq10_e129, (eq10_e128_d_n0 * ddt_scale), (eq10_e128_d_n1 * ddt_scale), (eq10_e128_d_n2 * ddt_scale), (eq10_e128_d_n3 * ddt_scale), (eq10_e128_d_n4 * ddt_scale), (eq10_e128_d_n5 * ddt_scale), (eq10_e128_d_n6 * ddt_scale), (eq10_e128_d_n7 * ddt_scale), (eq10_e128_d_n8 * ddt_scale), (eq10_e128_d_n9 * ddt_scale), (eq10_e128_d_n10 * ddt_scale), (eq10_e128_d_n11 * ddt_scale), (eq10_e128_d_n12 * ddt_scale), (eq10_e128_d_n13 * ddt_scale), (eq10_e128_d_n14 * ddt_scale), (eq10_e128_d_n15 * ddt_scale), (eq10_e128_d_b0 * ddt_scale), (eq10_e128_d_b1 * ddt_scale), (eq10_e128_d_b2 * ddt_scale), (eq10_e128_d_b3 * ddt_scale), (eq10_e128_d_b4 * ddt_scale), (eq10_e128_d_b5 * ddt_scale), (eq10_e128_d_b6 * ddt_scale), (eq10_e128_d_b7 * ddt_scale), (eq10_e128_d_b8 * ddt_scale), (eq10_e128_d_b9 * ddt_scale), (eq10_e128_d_b10 * ddt_scale), (eq10_e128_d_b11 * ddt_scale), (eq10_e128_d_b12 * ddt_scale), (eq10_e128_d_b13 * ddt_scale), (eq10_e128_d_b14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e131;
        let eq10_node_derivatives: [f64; 16] = [eq10_e131_d_n0, eq10_e131_d_n1, eq10_e131_d_n2, eq10_e131_d_n3, eq10_e131_d_n4, eq10_e131_d_n5, eq10_e131_d_n6, eq10_e131_d_n7, eq10_e131_d_n8, eq10_e131_d_n9, eq10_e131_d_n10, eq10_e131_d_n11, eq10_e131_d_n12, eq10_e131_d_n13, eq10_e131_d_n14, eq10_e131_d_n15];
        let eq10_branch_derivatives: [f64; 15] = [eq10_e131_d_b0, eq10_e131_d_b1, eq10_e131_d_b2, eq10_e131_d_b3, eq10_e131_d_b4, eq10_e131_d_b5, eq10_e131_d_b6, eq10_e131_d_b7, eq10_e131_d_b8, eq10_e131_d_b9, eq10_e131_d_b10, eq10_e131_d_b11, eq10_e131_d_b12, eq10_e131_d_b13, eq10_e131_d_b14];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let (eq24_e211, eq24_e211_d_b5,) = {
    if (var_guard21 != 0.0) {
        let eq24_e209: f64 = (bi5 * p.p42);
        (eq24_e209, p.p42,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e211;
        stamper.stamp_potential_branch1_local(
            5,
            eq24_value,
            5,
            eq24_e211_d_b5,
        );
        let (eq25_e218, eq25_e218_d_b5,) = {
    if (var_guard21 != 0.0) {
        let eq25_e215: f64 = (p.p50 * bi5);
        let eq25_e216: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq25_e215);
        (eq25_e216, (p.p50 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e218;
        stamper.stamp_potential_branch1_local(
            6,
            eq25_value,
            5,
            eq25_e218_d_b5,
        );
        let (eq26_e232,) = {
    if ((var_guard21 != 0.0) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e232;
        stamper.stamp_potential_const_local(
            7,
            eq26_value,
        );
        let (eq27_e242, eq27_e242_d_b5,) = {
    if ((var_guard21 == 0.0) && (var_guard22 != 0.0)) {
        let eq27_e239: f64 = (p.p50 * bi5);
        let eq27_e240: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq27_e239);
        (eq27_e240, (p.p50 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e242;
        stamper.stamp_potential_branch1_local(
            8,
            eq27_value,
            5,
            eq27_e242_d_b5,
        );
        let (eq29_e256, eq29_e256_d_n0, eq29_e256_d_n1, eq29_e256_d_n2, eq29_e256_d_n3, eq29_e256_d_n4, eq29_e256_d_n5, eq29_e256_d_n6, eq29_e256_d_n7, eq29_e256_d_n8, eq29_e256_d_n9, eq29_e256_d_n10, eq29_e256_d_n11, eq29_e256_d_n12, eq29_e256_d_n13, eq29_e256_d_n14, eq29_e256_d_n15, eq29_e256_d_b0, eq29_e256_d_b1, eq29_e256_d_b2, eq29_e256_d_b3, eq29_e256_d_b4, eq29_e256_d_b5, eq29_e256_d_b6, eq29_e256_d_b7, eq29_e256_d_b8, eq29_e256_d_b9, eq29_e256_d_b10, eq29_e256_d_b11, eq29_e256_d_b12, eq29_e256_d_b13, eq29_e256_d_b14,) = {
    if (var_guard23 != 0.0) {
        let eq29_e254: f64 = (bi10 * var_rs_t);
        let eq29_e254_d_n0: f64 = (bi10 * var_rs_t_dn0);
        let eq29_e254_d_n1: f64 = (bi10 * var_rs_t_dn1);
        let eq29_e254_d_n2: f64 = (bi10 * var_rs_t_dn2);
        let eq29_e254_d_n3: f64 = (bi10 * var_rs_t_dn3);
        let eq29_e254_d_n4: f64 = (bi10 * var_rs_t_dn4);
        let eq29_e254_d_n5: f64 = (bi10 * var_rs_t_dn5);
        let eq29_e254_d_n6: f64 = (bi10 * var_rs_t_dn6);
        let eq29_e254_d_n7: f64 = (bi10 * var_rs_t_dn7);
        let eq29_e254_d_n8: f64 = (bi10 * var_rs_t_dn8);
        let eq29_e254_d_n9: f64 = (bi10 * var_rs_t_dn9);
        let eq29_e254_d_n10: f64 = (bi10 * var_rs_t_dn10);
        let eq29_e254_d_n11: f64 = (bi10 * var_rs_t_dn11);
        let eq29_e254_d_n12: f64 = (bi10 * var_rs_t_dn12);
        let eq29_e254_d_n13: f64 = (bi10 * var_rs_t_dn13);
        let eq29_e254_d_n14: f64 = (bi10 * var_rs_t_dn14);
        let eq29_e254_d_n15: f64 = (bi10 * var_rs_t_dn15);
        let eq29_e254_d_b0: f64 = (bi10 * var_rs_t_db0);
        let eq29_e254_d_b1: f64 = (bi10 * var_rs_t_db1);
        let eq29_e254_d_b2: f64 = (bi10 * var_rs_t_db2);
        let eq29_e254_d_b3: f64 = (bi10 * var_rs_t_db3);
        let eq29_e254_d_b4: f64 = (bi10 * var_rs_t_db4);
        let eq29_e254_d_b5: f64 = (bi10 * var_rs_t_db5);
        let eq29_e254_d_b6: f64 = (bi10 * var_rs_t_db6);
        let eq29_e254_d_b7: f64 = (bi10 * var_rs_t_db7);
        let eq29_e254_d_b8: f64 = (bi10 * var_rs_t_db8);
        let eq29_e254_d_b9: f64 = (bi10 * var_rs_t_db9);
        let eq29_e254_d_b10: f64 = (var_rs_t + (bi10 * var_rs_t_db10));
        let eq29_e254_d_b11: f64 = (bi10 * var_rs_t_db11);
        let eq29_e254_d_b12: f64 = (bi10 * var_rs_t_db12);
        let eq29_e254_d_b13: f64 = (bi10 * var_rs_t_db13);
        let eq29_e254_d_b14: f64 = (bi10 * var_rs_t_db14);
        (eq29_e254, eq29_e254_d_n0, eq29_e254_d_n1, eq29_e254_d_n2, eq29_e254_d_n3, eq29_e254_d_n4, eq29_e254_d_n5, eq29_e254_d_n6, eq29_e254_d_n7, eq29_e254_d_n8, eq29_e254_d_n9, eq29_e254_d_n10, eq29_e254_d_n11, eq29_e254_d_n12, eq29_e254_d_n13, eq29_e254_d_n14, eq29_e254_d_n15, eq29_e254_d_b0, eq29_e254_d_b1, eq29_e254_d_b2, eq29_e254_d_b3, eq29_e254_d_b4, eq29_e254_d_b5, eq29_e254_d_b6, eq29_e254_d_b7, eq29_e254_d_b8, eq29_e254_d_b9, eq29_e254_d_b10, eq29_e254_d_b11, eq29_e254_d_b12, eq29_e254_d_b13, eq29_e254_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e256;
        let eq29_node_derivatives: [f64; 16] = [eq29_e256_d_n0, eq29_e256_d_n1, eq29_e256_d_n2, eq29_e256_d_n3, eq29_e256_d_n4, eq29_e256_d_n5, eq29_e256_d_n6, eq29_e256_d_n7, eq29_e256_d_n8, eq29_e256_d_n9, eq29_e256_d_n10, eq29_e256_d_n11, eq29_e256_d_n12, eq29_e256_d_n13, eq29_e256_d_n14, eq29_e256_d_n15];
        let eq29_branch_derivatives: [f64; 15] = [eq29_e256_d_b0, eq29_e256_d_b1, eq29_e256_d_b2, eq29_e256_d_b3, eq29_e256_d_b4, eq29_e256_d_b5, eq29_e256_d_b6, eq29_e256_d_b7, eq29_e256_d_b8, eq29_e256_d_b9, eq29_e256_d_b10, eq29_e256_d_b11, eq29_e256_d_b12, eq29_e256_d_b13, eq29_e256_d_b14];
        stamper.stamp_potential_dense_local(
            10,
            eq29_value,
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
        );
        let (eq30_e270,) = {
    if ((var_guard23 != 0.0) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq30_value: f64 = eq30_e270;
        stamper.stamp_potential_const_local(
            11,
            eq30_value,
        );
        let eq32_e278: f64 = (p.p49 * bi13);
        let eq32_e279: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq32_e278);
        let eq32_value: f64 = eq32_e279;
        stamper.stamp_potential_branch1_local(
            13,
            eq32_value,
            13,
            (p.p49 * ddt_scale),
        );
        let (eq33_e285, eq33_e285_d_n0, eq33_e285_d_n1, eq33_e285_d_n2, eq33_e285_d_n3, eq33_e285_d_n4, eq33_e285_d_n5, eq33_e285_d_n6, eq33_e285_d_n7, eq33_e285_d_n8, eq33_e285_d_n9, eq33_e285_d_n10, eq33_e285_d_n11, eq33_e285_d_n12, eq33_e285_d_n13, eq33_e285_d_n14, eq33_e285_d_n15, eq33_e285_d_b0, eq33_e285_d_b1, eq33_e285_d_b2, eq33_e285_d_b3, eq33_e285_d_b4, eq33_e285_d_b5, eq33_e285_d_b6, eq33_e285_d_b7, eq33_e285_d_b8, eq33_e285_d_b9, eq33_e285_d_b10, eq33_e285_d_b11, eq33_e285_d_b12, eq33_e285_d_b13, eq33_e285_d_b14,) = {
    if (var_guard24 != 0.0) {
        let eq33_e283: f64 = (bi14 * var_rd1_t);
        let eq33_e283_d_n0: f64 = (bi14 * var_rd1_t_dn0);
        let eq33_e283_d_n1: f64 = (bi14 * var_rd1_t_dn1);
        let eq33_e283_d_n2: f64 = (bi14 * var_rd1_t_dn2);
        let eq33_e283_d_n3: f64 = (bi14 * var_rd1_t_dn3);
        let eq33_e283_d_n4: f64 = (bi14 * var_rd1_t_dn4);
        let eq33_e283_d_n5: f64 = (bi14 * var_rd1_t_dn5);
        let eq33_e283_d_n6: f64 = (bi14 * var_rd1_t_dn6);
        let eq33_e283_d_n7: f64 = (bi14 * var_rd1_t_dn7);
        let eq33_e283_d_n8: f64 = (bi14 * var_rd1_t_dn8);
        let eq33_e283_d_n9: f64 = (bi14 * var_rd1_t_dn9);
        let eq33_e283_d_n10: f64 = (bi14 * var_rd1_t_dn10);
        let eq33_e283_d_n11: f64 = (bi14 * var_rd1_t_dn11);
        let eq33_e283_d_n12: f64 = (bi14 * var_rd1_t_dn12);
        let eq33_e283_d_n13: f64 = (bi14 * var_rd1_t_dn13);
        let eq33_e283_d_n14: f64 = (bi14 * var_rd1_t_dn14);
        let eq33_e283_d_n15: f64 = (bi14 * var_rd1_t_dn15);
        let eq33_e283_d_b0: f64 = (bi14 * var_rd1_t_db0);
        let eq33_e283_d_b1: f64 = (bi14 * var_rd1_t_db1);
        let eq33_e283_d_b2: f64 = (bi14 * var_rd1_t_db2);
        let eq33_e283_d_b3: f64 = (bi14 * var_rd1_t_db3);
        let eq33_e283_d_b4: f64 = (bi14 * var_rd1_t_db4);
        let eq33_e283_d_b5: f64 = (bi14 * var_rd1_t_db5);
        let eq33_e283_d_b6: f64 = (bi14 * var_rd1_t_db6);
        let eq33_e283_d_b7: f64 = (bi14 * var_rd1_t_db7);
        let eq33_e283_d_b8: f64 = (bi14 * var_rd1_t_db8);
        let eq33_e283_d_b9: f64 = (bi14 * var_rd1_t_db9);
        let eq33_e283_d_b10: f64 = (bi14 * var_rd1_t_db10);
        let eq33_e283_d_b11: f64 = (bi14 * var_rd1_t_db11);
        let eq33_e283_d_b12: f64 = (bi14 * var_rd1_t_db12);
        let eq33_e283_d_b13: f64 = (bi14 * var_rd1_t_db13);
        let eq33_e283_d_b14: f64 = (var_rd1_t + (bi14 * var_rd1_t_db14));
        (eq33_e283, eq33_e283_d_n0, eq33_e283_d_n1, eq33_e283_d_n2, eq33_e283_d_n3, eq33_e283_d_n4, eq33_e283_d_n5, eq33_e283_d_n6, eq33_e283_d_n7, eq33_e283_d_n8, eq33_e283_d_n9, eq33_e283_d_n10, eq33_e283_d_n11, eq33_e283_d_n12, eq33_e283_d_n13, eq33_e283_d_n14, eq33_e283_d_n15, eq33_e283_d_b0, eq33_e283_d_b1, eq33_e283_d_b2, eq33_e283_d_b3, eq33_e283_d_b4, eq33_e283_d_b5, eq33_e283_d_b6, eq33_e283_d_b7, eq33_e283_d_b8, eq33_e283_d_b9, eq33_e283_d_b10, eq33_e283_d_b11, eq33_e283_d_b12, eq33_e283_d_b13, eq33_e283_d_b14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e285;
        let eq33_node_derivatives: [f64; 16] = [eq33_e285_d_n0, eq33_e285_d_n1, eq33_e285_d_n2, eq33_e285_d_n3, eq33_e285_d_n4, eq33_e285_d_n5, eq33_e285_d_n6, eq33_e285_d_n7, eq33_e285_d_n8, eq33_e285_d_n9, eq33_e285_d_n10, eq33_e285_d_n11, eq33_e285_d_n12, eq33_e285_d_n13, eq33_e285_d_n14, eq33_e285_d_n15];
        let eq33_branch_derivatives: [f64; 15] = [eq33_e285_d_b0, eq33_e285_d_b1, eq33_e285_d_b2, eq33_e285_d_b3, eq33_e285_d_b4, eq33_e285_d_b5, eq33_e285_d_b6, eq33_e285_d_b7, eq33_e285_d_b8, eq33_e285_d_b9, eq33_e285_d_b10, eq33_e285_d_b11, eq33_e285_d_b12, eq33_e285_d_b13, eq33_e285_d_b14];
        stamper.stamp_potential_dense_local(
            14,
            eq33_value,
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
        );
        let (eq34_e292, eq34_e292_d_b14,) = {
    if (var_guard24 != 0.0) {
        let eq34_e289: f64 = (p.p48 * bi14);
        let eq34_e290: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq34_e289);
        (eq34_e290, (p.p48 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e292;
        stamper.stamp_potential_branch1_local(
            15,
            eq34_value,
            14,
            eq34_e292_d_b14,
        );
        let (eq35_e306,) = {
    if ((var_guard24 != 0.0) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e306;
        stamper.stamp_potential_const_local(
            16,
            eq35_value,
        );
        let (eq36_e316, eq36_e316_d_b14,) = {
    if ((var_guard24 == 0.0) && (var_guard25 != 0.0)) {
        let eq36_e313: f64 = (p.p48 * bi14);
        let eq36_e314: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq36_e313);
        (eq36_e314, (p.p48 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e316;
        stamper.stamp_potential_branch1_local(
            17,
            eq36_value,
            14,
            eq36_e316_d_b14,
        );
        let (eq46_e420, eq46_e420_d_n0, eq46_e420_d_n1, eq46_e420_d_n2, eq46_e420_d_n3, eq46_e420_d_n4, eq46_e420_d_n5, eq46_e420_d_n6, eq46_e420_d_n7, eq46_e420_d_n8, eq46_e420_d_n9, eq46_e420_d_n10, eq46_e420_d_n11, eq46_e420_d_n12, eq46_e420_d_n13, eq46_e420_d_n14, eq46_e420_d_n15, eq46_e420_d_b0, eq46_e420_d_b1, eq46_e420_d_b2, eq46_e420_d_b3, eq46_e420_d_b4, eq46_e420_d_b5, eq46_e420_d_b6, eq46_e420_d_b7, eq46_e420_d_b8, eq46_e420_d_b9, eq46_e420_d_b10, eq46_e420_d_b11, eq46_e420_d_b12, eq46_e420_d_b13, eq46_e420_d_b14,) = {
    if (((var_guard27 != 0.0) && (var_guard26 == 0.0)) && (p.p0 != 0.0)) {
        let eq46_e415: f64 = (-var_ci);
        let eq46_e417: f64 = (eq46_e415 * (nv14 - 0.0));
        let eq46_e417_d_n0: f64 = ((-var_ci_dn0) * (nv14 - 0.0));
        let eq46_e417_d_n1: f64 = ((-var_ci_dn1) * (nv14 - 0.0));
        let eq46_e417_d_n2: f64 = ((-var_ci_dn2) * (nv14 - 0.0));
        let eq46_e417_d_n3: f64 = ((-var_ci_dn3) * (nv14 - 0.0));
        let eq46_e417_d_n4: f64 = ((-var_ci_dn4) * (nv14 - 0.0));
        let eq46_e417_d_n5: f64 = ((-var_ci_dn5) * (nv14 - 0.0));
        let eq46_e417_d_n6: f64 = ((-var_ci_dn6) * (nv14 - 0.0));
        let eq46_e417_d_n7: f64 = ((-var_ci_dn7) * (nv14 - 0.0));
        let eq46_e417_d_n8: f64 = ((-var_ci_dn8) * (nv14 - 0.0));
        let eq46_e417_d_n9: f64 = ((-var_ci_dn9) * (nv14 - 0.0));
        let eq46_e417_d_n10: f64 = ((-var_ci_dn10) * (nv14 - 0.0));
        let eq46_e417_d_n11: f64 = ((-var_ci_dn11) * (nv14 - 0.0));
        let eq46_e417_d_n12: f64 = ((-var_ci_dn12) * (nv14 - 0.0));
        let eq46_e417_d_n13: f64 = ((-var_ci_dn13) * (nv14 - 0.0));
        let eq46_e417_d_n14: f64 = (((-var_ci_dn14) * (nv14 - 0.0)) + eq46_e415);
        let eq46_e417_d_n15: f64 = ((-var_ci_dn15) * (nv14 - 0.0));
        let eq46_e417_d_b0: f64 = ((-var_ci_db0) * (nv14 - 0.0));
        let eq46_e417_d_b1: f64 = ((-var_ci_db1) * (nv14 - 0.0));
        let eq46_e417_d_b2: f64 = ((-var_ci_db2) * (nv14 - 0.0));
        let eq46_e417_d_b3: f64 = ((-var_ci_db3) * (nv14 - 0.0));
        let eq46_e417_d_b4: f64 = ((-var_ci_db4) * (nv14 - 0.0));
        let eq46_e417_d_b5: f64 = ((-var_ci_db5) * (nv14 - 0.0));
        let eq46_e417_d_b6: f64 = ((-var_ci_db6) * (nv14 - 0.0));
        let eq46_e417_d_b7: f64 = ((-var_ci_db7) * (nv14 - 0.0));
        let eq46_e417_d_b8: f64 = ((-var_ci_db8) * (nv14 - 0.0));
        let eq46_e417_d_b9: f64 = ((-var_ci_db9) * (nv14 - 0.0));
        let eq46_e417_d_b10: f64 = ((-var_ci_db10) * (nv14 - 0.0));
        let eq46_e417_d_b11: f64 = ((-var_ci_db11) * (nv14 - 0.0));
        let eq46_e417_d_b12: f64 = ((-var_ci_db12) * (nv14 - 0.0));
        let eq46_e417_d_b13: f64 = ((-var_ci_db13) * (nv14 - 0.0));
        let eq46_e417_d_b14: f64 = ((-var_ci_db14) * (nv14 - 0.0));
        let eq46_e418: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq46_e417);
        (eq46_e418, (eq46_e417_d_n0 * ddt_scale), (eq46_e417_d_n1 * ddt_scale), (eq46_e417_d_n2 * ddt_scale), (eq46_e417_d_n3 * ddt_scale), (eq46_e417_d_n4 * ddt_scale), (eq46_e417_d_n5 * ddt_scale), (eq46_e417_d_n6 * ddt_scale), (eq46_e417_d_n7 * ddt_scale), (eq46_e417_d_n8 * ddt_scale), (eq46_e417_d_n9 * ddt_scale), (eq46_e417_d_n10 * ddt_scale), (eq46_e417_d_n11 * ddt_scale), (eq46_e417_d_n12 * ddt_scale), (eq46_e417_d_n13 * ddt_scale), (eq46_e417_d_n14 * ddt_scale), (eq46_e417_d_n15 * ddt_scale), (eq46_e417_d_b0 * ddt_scale), (eq46_e417_d_b1 * ddt_scale), (eq46_e417_d_b2 * ddt_scale), (eq46_e417_d_b3 * ddt_scale), (eq46_e417_d_b4 * ddt_scale), (eq46_e417_d_b5 * ddt_scale), (eq46_e417_d_b6 * ddt_scale), (eq46_e417_d_b7 * ddt_scale), (eq46_e417_d_b8 * ddt_scale), (eq46_e417_d_b9 * ddt_scale), (eq46_e417_d_b10 * ddt_scale), (eq46_e417_d_b11 * ddt_scale), (eq46_e417_d_b12 * ddt_scale), (eq46_e417_d_b13 * ddt_scale), (eq46_e417_d_b14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e420;
        let eq46_node_derivatives: [f64; 16] = [eq46_e420_d_n0, eq46_e420_d_n1, eq46_e420_d_n2, eq46_e420_d_n3, eq46_e420_d_n4, eq46_e420_d_n5, eq46_e420_d_n6, eq46_e420_d_n7, eq46_e420_d_n8, eq46_e420_d_n9, eq46_e420_d_n10, eq46_e420_d_n11, eq46_e420_d_n12, eq46_e420_d_n13, eq46_e420_d_n14, eq46_e420_d_n15];
        let eq46_branch_derivatives: [f64; 15] = [eq46_e420_d_b0, eq46_e420_d_b1, eq46_e420_d_b2, eq46_e420_d_b3, eq46_e420_d_b4, eq46_e420_d_b5, eq46_e420_d_b6, eq46_e420_d_b7, eq46_e420_d_b8, eq46_e420_d_b9, eq46_e420_d_b10, eq46_e420_d_b11, eq46_e420_d_b12, eq46_e420_d_b13, eq46_e420_d_b14];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(3),
            multiplicity * (eq46_value),
            &eq46_node_derivatives,
            &eq46_branch_derivatives,
            multiplicity,
        );
        let (eq57_e532, eq57_e532_d_n11,) = {
    if (var_guard43 != 0.0) {
        let eq57_e529: f64 = (p.p58 * (nv11 - 0.0));
        let eq57_e530: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq57_e529);
        (eq57_e530, (p.p58 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e532;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (eq57_value),
            11,
            multiplicity * (eq57_e532_d_n11),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_cgd: f64,
        var_cgd_db0: f64,
        var_cgd_db1: f64,
        var_cgd_db10: f64,
        var_cgd_db11: f64,
        var_cgd_db12: f64,
        var_cgd_db13: f64,
        var_cgd_db14: f64,
        var_cgd_db15: f64,
        var_cgd_db16: f64,
        var_cgd_db17: f64,
        var_cgd_db18: f64,
        var_cgd_db2: f64,
        var_cgd_db3: f64,
        var_cgd_db4: f64,
        var_cgd_db5: f64,
        var_cgd_db6: f64,
        var_cgd_db7: f64,
        var_cgd_db8: f64,
        var_cgd_db9: f64,
        var_cgd_dn0: f64,
        var_cgd_dn1: f64,
        var_cgd_dn10: f64,
        var_cgd_dn11: f64,
        var_cgd_dn12: f64,
        var_cgd_dn13: f64,
        var_cgd_dn14: f64,
        var_cgd_dn15: f64,
        var_cgd_dn2: f64,
        var_cgd_dn3: f64,
        var_cgd_dn4: f64,
        var_cgd_dn5: f64,
        var_cgd_dn6: f64,
        var_cgd_dn7: f64,
        var_cgd_dn8: f64,
        var_cgd_dn9: f64,
        var_cgs: f64,
        var_cgs_db0: f64,
        var_cgs_db1: f64,
        var_cgs_db10: f64,
        var_cgs_db11: f64,
        var_cgs_db12: f64,
        var_cgs_db13: f64,
        var_cgs_db14: f64,
        var_cgs_db15: f64,
        var_cgs_db16: f64,
        var_cgs_db17: f64,
        var_cgs_db18: f64,
        var_cgs_db2: f64,
        var_cgs_db3: f64,
        var_cgs_db4: f64,
        var_cgs_db5: f64,
        var_cgs_db6: f64,
        var_cgs_db7: f64,
        var_cgs_db8: f64,
        var_cgs_db9: f64,
        var_cgs_dn0: f64,
        var_cgs_dn1: f64,
        var_cgs_dn10: f64,
        var_cgs_dn11: f64,
        var_cgs_dn12: f64,
        var_cgs_dn13: f64,
        var_cgs_dn14: f64,
        var_cgs_dn15: f64,
        var_cgs_dn2: f64,
        var_cgs_dn3: f64,
        var_cgs_dn4: f64,
        var_cgs_dn5: f64,
        var_cgs_dn6: f64,
        var_cgs_dn7: f64,
        var_cgs_dn8: f64,
        var_cgs_dn9: f64,
        var_ci: f64,
        var_ci_db0: f64,
        var_ci_db1: f64,
        var_ci_db10: f64,
        var_ci_db11: f64,
        var_ci_db12: f64,
        var_ci_db13: f64,
        var_ci_db14: f64,
        var_ci_db2: f64,
        var_ci_db3: f64,
        var_ci_db4: f64,
        var_ci_db5: f64,
        var_ci_db6: f64,
        var_ci_db7: f64,
        var_ci_db8: f64,
        var_ci_db9: f64,
        var_ci_dn0: f64,
        var_ci_dn1: f64,
        var_ci_dn10: f64,
        var_ci_dn11: f64,
        var_ci_dn12: f64,
        var_ci_dn13: f64,
        var_ci_dn14: f64,
        var_ci_dn15: f64,
        var_ci_dn2: f64,
        var_ci_dn3: f64,
        var_ci_dn4: f64,
        var_ci_dn5: f64,
        var_ci_dn6: f64,
        var_ci_dn7: f64,
        var_ci_dn8: f64,
        var_ci_dn9: f64,
        var_guard16: f64,
        var_guard21: f64,
        var_guard22: f64,
        var_guard24: f64,
        var_guard25: f64,
        var_guard26: f64,
        var_guard27: f64,
        var_guard43: f64,
        var_qgd: f64,
        var_qgd_db0: f64,
        var_qgd_db1: f64,
        var_qgd_db10: f64,
        var_qgd_db11: f64,
        var_qgd_db12: f64,
        var_qgd_db13: f64,
        var_qgd_db14: f64,
        var_qgd_db2: f64,
        var_qgd_db3: f64,
        var_qgd_db4: f64,
        var_qgd_db5: f64,
        var_qgd_db6: f64,
        var_qgd_db7: f64,
        var_qgd_db8: f64,
        var_qgd_db9: f64,
        var_qgd_dn0: f64,
        var_qgd_dn1: f64,
        var_qgd_dn10: f64,
        var_qgd_dn11: f64,
        var_qgd_dn12: f64,
        var_qgd_dn13: f64,
        var_qgd_dn14: f64,
        var_qgd_dn15: f64,
        var_qgd_dn2: f64,
        var_qgd_dn3: f64,
        var_qgd_dn4: f64,
        var_qgd_dn5: f64,
        var_qgd_dn6: f64,
        var_qgd_dn7: f64,
        var_qgd_dn8: f64,
        var_qgd_dn9: f64,
        var_qgs: f64,
        var_qgs_db0: f64,
        var_qgs_db1: f64,
        var_qgs_db10: f64,
        var_qgs_db11: f64,
        var_qgs_db12: f64,
        var_qgs_db13: f64,
        var_qgs_db14: f64,
        var_qgs_db2: f64,
        var_qgs_db3: f64,
        var_qgs_db4: f64,
        var_qgs_db5: f64,
        var_qgs_db6: f64,
        var_qgs_db7: f64,
        var_qgs_db8: f64,
        var_qgs_db9: f64,
        var_qgs_dn0: f64,
        var_qgs_dn1: f64,
        var_qgs_dn10: f64,
        var_qgs_dn11: f64,
        var_qgs_dn12: f64,
        var_qgs_dn13: f64,
        var_qgs_dn14: f64,
        var_qgs_dn15: f64,
        var_qgs_dn2: f64,
        var_qgs_dn3: f64,
        var_qgs_dn4: f64,
        var_qgs_dn5: f64,
        var_qgs_dn6: f64,
        var_qgs_dn7: f64,
        var_qgs_dn8: f64,
        var_qgs_dn9: f64,
        var_vgdc: f64,
        var_vgdc_db0: f64,
        var_vgdc_db1: f64,
        var_vgdc_db10: f64,
        var_vgdc_db11: f64,
        var_vgdc_db12: f64,
        var_vgdc_db13: f64,
        var_vgdc_db14: f64,
        var_vgdc_db15: f64,
        var_vgdc_db16: f64,
        var_vgdc_db17: f64,
        var_vgdc_db18: f64,
        var_vgdc_db2: f64,
        var_vgdc_db3: f64,
        var_vgdc_db4: f64,
        var_vgdc_db5: f64,
        var_vgdc_db6: f64,
        var_vgdc_db7: f64,
        var_vgdc_db8: f64,
        var_vgdc_db9: f64,
        var_vgdc_dn0: f64,
        var_vgdc_dn1: f64,
        var_vgdc_dn10: f64,
        var_vgdc_dn11: f64,
        var_vgdc_dn12: f64,
        var_vgdc_dn13: f64,
        var_vgdc_dn14: f64,
        var_vgdc_dn15: f64,
        var_vgdc_dn2: f64,
        var_vgdc_dn3: f64,
        var_vgdc_dn4: f64,
        var_vgdc_dn5: f64,
        var_vgdc_dn6: f64,
        var_vgdc_dn7: f64,
        var_vgdc_dn8: f64,
        var_vgdc_dn9: f64,
        var_vgsc: f64,
        var_vgsc_db0: f64,
        var_vgsc_db1: f64,
        var_vgsc_db10: f64,
        var_vgsc_db11: f64,
        var_vgsc_db12: f64,
        var_vgsc_db13: f64,
        var_vgsc_db14: f64,
        var_vgsc_db15: f64,
        var_vgsc_db16: f64,
        var_vgsc_db17: f64,
        var_vgsc_db18: f64,
        var_vgsc_db2: f64,
        var_vgsc_db3: f64,
        var_vgsc_db4: f64,
        var_vgsc_db5: f64,
        var_vgsc_db6: f64,
        var_vgsc_db7: f64,
        var_vgsc_db8: f64,
        var_vgsc_db9: f64,
        var_vgsc_dn0: f64,
        var_vgsc_dn1: f64,
        var_vgsc_dn10: f64,
        var_vgsc_dn11: f64,
        var_vgsc_dn12: f64,
        var_vgsc_dn13: f64,
        var_vgsc_dn14: f64,
        var_vgsc_dn15: f64,
        var_vgsc_dn2: f64,
        var_vgsc_dn3: f64,
        var_vgsc_dn4: f64,
        var_vgsc_dn5: f64,
        var_vgsc_dn6: f64,
        var_vgsc_dn7: f64,
        var_vgsc_dn8: f64,
        var_vgsc_dn9: f64,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let bi0 = ctx.branch_current(branches[0]);
        let bi5 = ctx.branch_current(branches[5]);
        let bi13 = ctx.branch_current(branches[13]);
        let bi14 = ctx.branch_current(branches[14]);
        let eq3_e99: f64 = (p.p51 / 3.0);
        let eq3_e101: f64 = (eq3_e99 * bi0);
        let eq3_e102_q: f64 = eq3_e101;
        stamper.stamp_potential_reactive_branch1(
            branches[0],
            branches[0],
            eq3_e99,
        );
        let (eq7_e110, eq7_e110_d_n0, eq7_e110_d_n1, eq7_e110_d_n2, eq7_e110_d_n3, eq7_e110_d_n4, eq7_e110_d_n5, eq7_e110_d_n6, eq7_e110_d_n7, eq7_e110_d_n8, eq7_e110_d_n9, eq7_e110_d_n10, eq7_e110_d_n11, eq7_e110_d_n12, eq7_e110_d_n13, eq7_e110_d_n14, eq7_e110_d_n15, eq7_e110_d_b0, eq7_e110_d_b1, eq7_e110_d_b2, eq7_e110_d_b3, eq7_e110_d_b4, eq7_e110_d_b5, eq7_e110_d_b6, eq7_e110_d_b7, eq7_e110_d_b8, eq7_e110_d_b9, eq7_e110_d_b10, eq7_e110_d_b11, eq7_e110_d_b12, eq7_e110_d_b13, eq7_e110_d_b14, eq7_e110_q,) = {
    if (var_guard16 != 0.0) {
        let eq7_e108_q: f64 = var_qgd;
        (var_qgd, var_qgd_dn0, var_qgd_dn1, var_qgd_dn2, var_qgd_dn3, var_qgd_dn4, var_qgd_dn5, var_qgd_dn6, var_qgd_dn7, var_qgd_dn8, var_qgd_dn9, var_qgd_dn10, var_qgd_dn11, var_qgd_dn12, var_qgd_dn13, var_qgd_dn14, var_qgd_dn15, var_qgd_db0, var_qgd_db1, var_qgd_db2, var_qgd_db3, var_qgd_db4, var_qgd_db5, var_qgd_db6, var_qgd_db7, var_qgd_db8, var_qgd_db9, var_qgd_db10, var_qgd_db11, var_qgd_db12, var_qgd_db13, var_qgd_db14, eq7_e108_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 16] = [eq7_e110_d_n0, eq7_e110_d_n1, eq7_e110_d_n2, eq7_e110_d_n3, eq7_e110_d_n4, eq7_e110_d_n5, eq7_e110_d_n6, eq7_e110_d_n7, eq7_e110_d_n8, eq7_e110_d_n9, eq7_e110_d_n10, eq7_e110_d_n11, eq7_e110_d_n12, eq7_e110_d_n13, eq7_e110_d_n14, eq7_e110_d_n15];
        let eq7_reactive_branch_derivatives: [f64; 15] = [eq7_e110_d_b0, eq7_e110_d_b1, eq7_e110_d_b2, eq7_e110_d_b3, eq7_e110_d_b4, eq7_e110_d_b5, eq7_e110_d_b6, eq7_e110_d_b7, eq7_e110_d_b8, eq7_e110_d_b9, eq7_e110_d_b10, eq7_e110_d_b11, eq7_e110_d_b12, eq7_e110_d_b13, eq7_e110_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq8_e115, eq8_e115_d_n0, eq8_e115_d_n1, eq8_e115_d_n2, eq8_e115_d_n3, eq8_e115_d_n4, eq8_e115_d_n5, eq8_e115_d_n6, eq8_e115_d_n7, eq8_e115_d_n8, eq8_e115_d_n9, eq8_e115_d_n10, eq8_e115_d_n11, eq8_e115_d_n12, eq8_e115_d_n13, eq8_e115_d_n14, eq8_e115_d_n15, eq8_e115_d_b0, eq8_e115_d_b1, eq8_e115_d_b2, eq8_e115_d_b3, eq8_e115_d_b4, eq8_e115_d_b5, eq8_e115_d_b6, eq8_e115_d_b7, eq8_e115_d_b8, eq8_e115_d_b9, eq8_e115_d_b10, eq8_e115_d_b11, eq8_e115_d_b12, eq8_e115_d_b13, eq8_e115_d_b14, eq8_e115_q,) = {
    if (var_guard16 != 0.0) {
        let eq8_e113_q: f64 = var_qgs;
        (var_qgs, var_qgs_dn0, var_qgs_dn1, var_qgs_dn2, var_qgs_dn3, var_qgs_dn4, var_qgs_dn5, var_qgs_dn6, var_qgs_dn7, var_qgs_dn8, var_qgs_dn9, var_qgs_dn10, var_qgs_dn11, var_qgs_dn12, var_qgs_dn13, var_qgs_dn14, var_qgs_dn15, var_qgs_db0, var_qgs_db1, var_qgs_db2, var_qgs_db3, var_qgs_db4, var_qgs_db5, var_qgs_db6, var_qgs_db7, var_qgs_db8, var_qgs_db9, var_qgs_db10, var_qgs_db11, var_qgs_db12, var_qgs_db13, var_qgs_db14, eq8_e113_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 16] = [eq8_e115_d_n0, eq8_e115_d_n1, eq8_e115_d_n2, eq8_e115_d_n3, eq8_e115_d_n4, eq8_e115_d_n5, eq8_e115_d_n6, eq8_e115_d_n7, eq8_e115_d_n8, eq8_e115_d_n9, eq8_e115_d_n10, eq8_e115_d_n11, eq8_e115_d_n12, eq8_e115_d_n13, eq8_e115_d_n14, eq8_e115_d_n15];
        let eq8_reactive_branch_derivatives: [f64; 15] = [eq8_e115_d_b0, eq8_e115_d_b1, eq8_e115_d_b2, eq8_e115_d_b3, eq8_e115_d_b4, eq8_e115_d_b5, eq8_e115_d_b6, eq8_e115_d_b7, eq8_e115_d_b8, eq8_e115_d_b9, eq8_e115_d_b10, eq8_e115_d_b11, eq8_e115_d_b12, eq8_e115_d_b13, eq8_e115_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq9_e123, eq9_e123_d_n0, eq9_e123_d_n1, eq9_e123_d_n2, eq9_e123_d_n3, eq9_e123_d_n4, eq9_e123_d_n5, eq9_e123_d_n6, eq9_e123_d_n7, eq9_e123_d_n8, eq9_e123_d_n9, eq9_e123_d_n10, eq9_e123_d_n11, eq9_e123_d_n12, eq9_e123_d_n13, eq9_e123_d_n14, eq9_e123_d_n15, eq9_e123_d_b0, eq9_e123_d_b1, eq9_e123_d_b2, eq9_e123_d_b3, eq9_e123_d_b4, eq9_e123_d_b5, eq9_e123_d_b6, eq9_e123_d_b7, eq9_e123_d_b8, eq9_e123_d_b9, eq9_e123_d_b10, eq9_e123_d_b11, eq9_e123_d_b12, eq9_e123_d_b13, eq9_e123_d_b14, eq9_e123_q,) = {
    if (var_guard16 == 0.0) {
        let eq9_e120: f64 = (var_cgd * var_vgdc);
        let eq9_e120_d_n0: f64 = ((var_cgd_dn0 * var_vgdc) + (var_cgd * var_vgdc_dn0));
        let eq9_e120_d_n1: f64 = ((var_cgd_dn1 * var_vgdc) + (var_cgd * var_vgdc_dn1));
        let eq9_e120_d_n2: f64 = ((var_cgd_dn2 * var_vgdc) + (var_cgd * var_vgdc_dn2));
        let eq9_e120_d_n3: f64 = ((var_cgd_dn3 * var_vgdc) + (var_cgd * var_vgdc_dn3));
        let eq9_e120_d_n4: f64 = ((var_cgd_dn4 * var_vgdc) + (var_cgd * var_vgdc_dn4));
        let eq9_e120_d_n5: f64 = ((var_cgd_dn5 * var_vgdc) + (var_cgd * var_vgdc_dn5));
        let eq9_e120_d_n6: f64 = ((var_cgd_dn6 * var_vgdc) + (var_cgd * var_vgdc_dn6));
        let eq9_e120_d_n7: f64 = ((var_cgd_dn7 * var_vgdc) + (var_cgd * var_vgdc_dn7));
        let eq9_e120_d_n8: f64 = ((var_cgd_dn8 * var_vgdc) + (var_cgd * var_vgdc_dn8));
        let eq9_e120_d_n9: f64 = ((var_cgd_dn9 * var_vgdc) + (var_cgd * var_vgdc_dn9));
        let eq9_e120_d_n10: f64 = ((var_cgd_dn10 * var_vgdc) + (var_cgd * var_vgdc_dn10));
        let eq9_e120_d_n11: f64 = ((var_cgd_dn11 * var_vgdc) + (var_cgd * var_vgdc_dn11));
        let eq9_e120_d_n12: f64 = ((var_cgd_dn12 * var_vgdc) + (var_cgd * var_vgdc_dn12));
        let eq9_e120_d_n13: f64 = ((var_cgd_dn13 * var_vgdc) + (var_cgd * var_vgdc_dn13));
        let eq9_e120_d_n14: f64 = ((var_cgd_dn14 * var_vgdc) + (var_cgd * var_vgdc_dn14));
        let eq9_e120_d_n15: f64 = ((var_cgd_dn15 * var_vgdc) + (var_cgd * var_vgdc_dn15));
        let eq9_e120_d_b0: f64 = ((var_cgd_db0 * var_vgdc) + (var_cgd * var_vgdc_db0));
        let eq9_e120_d_b1: f64 = ((var_cgd_db1 * var_vgdc) + (var_cgd * var_vgdc_db1));
        let eq9_e120_d_b2: f64 = ((var_cgd_db2 * var_vgdc) + (var_cgd * var_vgdc_db2));
        let eq9_e120_d_b3: f64 = ((var_cgd_db3 * var_vgdc) + (var_cgd * var_vgdc_db3));
        let eq9_e120_d_b4: f64 = ((var_cgd_db4 * var_vgdc) + (var_cgd * var_vgdc_db4));
        let eq9_e120_d_b5: f64 = ((var_cgd_db5 * var_vgdc) + (var_cgd * var_vgdc_db5));
        let eq9_e120_d_b6: f64 = ((var_cgd_db6 * var_vgdc) + (var_cgd * var_vgdc_db6));
        let eq9_e120_d_b7: f64 = ((var_cgd_db7 * var_vgdc) + (var_cgd * var_vgdc_db7));
        let eq9_e120_d_b8: f64 = ((var_cgd_db8 * var_vgdc) + (var_cgd * var_vgdc_db8));
        let eq9_e120_d_b9: f64 = ((var_cgd_db9 * var_vgdc) + (var_cgd * var_vgdc_db9));
        let eq9_e120_d_b10: f64 = ((var_cgd_db10 * var_vgdc) + (var_cgd * var_vgdc_db10));
        let eq9_e120_d_b11: f64 = ((var_cgd_db11 * var_vgdc) + (var_cgd * var_vgdc_db11));
        let eq9_e120_d_b12: f64 = ((var_cgd_db12 * var_vgdc) + (var_cgd * var_vgdc_db12));
        let eq9_e120_d_b13: f64 = ((var_cgd_db13 * var_vgdc) + (var_cgd * var_vgdc_db13));
        let eq9_e120_d_b14: f64 = ((var_cgd_db14 * var_vgdc) + (var_cgd * var_vgdc_db14));
        let eq9_e120_d_b15: f64 = ((var_cgd_db15 * var_vgdc) + (var_cgd * var_vgdc_db15));
        let eq9_e120_d_b16: f64 = ((var_cgd_db16 * var_vgdc) + (var_cgd * var_vgdc_db16));
        let eq9_e120_d_b17: f64 = ((var_cgd_db17 * var_vgdc) + (var_cgd * var_vgdc_db17));
        let eq9_e120_d_b18: f64 = ((var_cgd_db18 * var_vgdc) + (var_cgd * var_vgdc_db18));
        let eq9_e121_q: f64 = eq9_e120;
        (eq9_e120, eq9_e120_d_n0, eq9_e120_d_n1, eq9_e120_d_n2, eq9_e120_d_n3, eq9_e120_d_n4, eq9_e120_d_n5, eq9_e120_d_n6, eq9_e120_d_n7, eq9_e120_d_n8, eq9_e120_d_n9, eq9_e120_d_n10, eq9_e120_d_n11, eq9_e120_d_n12, eq9_e120_d_n13, eq9_e120_d_n14, eq9_e120_d_n15, eq9_e120_d_b0, eq9_e120_d_b1, eq9_e120_d_b2, eq9_e120_d_b3, eq9_e120_d_b4, eq9_e120_d_b5, eq9_e120_d_b6, eq9_e120_d_b7, eq9_e120_d_b8, eq9_e120_d_b9, eq9_e120_d_b10, eq9_e120_d_b11, eq9_e120_d_b12, eq9_e120_d_b13, eq9_e120_d_b14, eq9_e121_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_reactive_node_derivatives: [f64; 16] = [eq9_e123_d_n0, eq9_e123_d_n1, eq9_e123_d_n2, eq9_e123_d_n3, eq9_e123_d_n4, eq9_e123_d_n5, eq9_e123_d_n6, eq9_e123_d_n7, eq9_e123_d_n8, eq9_e123_d_n9, eq9_e123_d_n10, eq9_e123_d_n11, eq9_e123_d_n12, eq9_e123_d_n13, eq9_e123_d_n14, eq9_e123_d_n15];
        let eq9_reactive_branch_derivatives: [f64; 15] = [eq9_e123_d_b0, eq9_e123_d_b1, eq9_e123_d_b2, eq9_e123_d_b3, eq9_e123_d_b4, eq9_e123_d_b5, eq9_e123_d_b6, eq9_e123_d_b7, eq9_e123_d_b8, eq9_e123_d_b9, eq9_e123_d_b10, eq9_e123_d_b11, eq9_e123_d_b12, eq9_e123_d_b13, eq9_e123_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &eq9_reactive_node_derivatives,
            branches,
            &eq9_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq10_e131, eq10_e131_d_n0, eq10_e131_d_n1, eq10_e131_d_n2, eq10_e131_d_n3, eq10_e131_d_n4, eq10_e131_d_n5, eq10_e131_d_n6, eq10_e131_d_n7, eq10_e131_d_n8, eq10_e131_d_n9, eq10_e131_d_n10, eq10_e131_d_n11, eq10_e131_d_n12, eq10_e131_d_n13, eq10_e131_d_n14, eq10_e131_d_n15, eq10_e131_d_b0, eq10_e131_d_b1, eq10_e131_d_b2, eq10_e131_d_b3, eq10_e131_d_b4, eq10_e131_d_b5, eq10_e131_d_b6, eq10_e131_d_b7, eq10_e131_d_b8, eq10_e131_d_b9, eq10_e131_d_b10, eq10_e131_d_b11, eq10_e131_d_b12, eq10_e131_d_b13, eq10_e131_d_b14, eq10_e131_q,) = {
    if (var_guard16 == 0.0) {
        let eq10_e128: f64 = (var_cgs * var_vgsc);
        let eq10_e128_d_n0: f64 = ((var_cgs_dn0 * var_vgsc) + (var_cgs * var_vgsc_dn0));
        let eq10_e128_d_n1: f64 = ((var_cgs_dn1 * var_vgsc) + (var_cgs * var_vgsc_dn1));
        let eq10_e128_d_n2: f64 = ((var_cgs_dn2 * var_vgsc) + (var_cgs * var_vgsc_dn2));
        let eq10_e128_d_n3: f64 = ((var_cgs_dn3 * var_vgsc) + (var_cgs * var_vgsc_dn3));
        let eq10_e128_d_n4: f64 = ((var_cgs_dn4 * var_vgsc) + (var_cgs * var_vgsc_dn4));
        let eq10_e128_d_n5: f64 = ((var_cgs_dn5 * var_vgsc) + (var_cgs * var_vgsc_dn5));
        let eq10_e128_d_n6: f64 = ((var_cgs_dn6 * var_vgsc) + (var_cgs * var_vgsc_dn6));
        let eq10_e128_d_n7: f64 = ((var_cgs_dn7 * var_vgsc) + (var_cgs * var_vgsc_dn7));
        let eq10_e128_d_n8: f64 = ((var_cgs_dn8 * var_vgsc) + (var_cgs * var_vgsc_dn8));
        let eq10_e128_d_n9: f64 = ((var_cgs_dn9 * var_vgsc) + (var_cgs * var_vgsc_dn9));
        let eq10_e128_d_n10: f64 = ((var_cgs_dn10 * var_vgsc) + (var_cgs * var_vgsc_dn10));
        let eq10_e128_d_n11: f64 = ((var_cgs_dn11 * var_vgsc) + (var_cgs * var_vgsc_dn11));
        let eq10_e128_d_n12: f64 = ((var_cgs_dn12 * var_vgsc) + (var_cgs * var_vgsc_dn12));
        let eq10_e128_d_n13: f64 = ((var_cgs_dn13 * var_vgsc) + (var_cgs * var_vgsc_dn13));
        let eq10_e128_d_n14: f64 = ((var_cgs_dn14 * var_vgsc) + (var_cgs * var_vgsc_dn14));
        let eq10_e128_d_n15: f64 = ((var_cgs_dn15 * var_vgsc) + (var_cgs * var_vgsc_dn15));
        let eq10_e128_d_b0: f64 = ((var_cgs_db0 * var_vgsc) + (var_cgs * var_vgsc_db0));
        let eq10_e128_d_b1: f64 = ((var_cgs_db1 * var_vgsc) + (var_cgs * var_vgsc_db1));
        let eq10_e128_d_b2: f64 = ((var_cgs_db2 * var_vgsc) + (var_cgs * var_vgsc_db2));
        let eq10_e128_d_b3: f64 = ((var_cgs_db3 * var_vgsc) + (var_cgs * var_vgsc_db3));
        let eq10_e128_d_b4: f64 = ((var_cgs_db4 * var_vgsc) + (var_cgs * var_vgsc_db4));
        let eq10_e128_d_b5: f64 = ((var_cgs_db5 * var_vgsc) + (var_cgs * var_vgsc_db5));
        let eq10_e128_d_b6: f64 = ((var_cgs_db6 * var_vgsc) + (var_cgs * var_vgsc_db6));
        let eq10_e128_d_b7: f64 = ((var_cgs_db7 * var_vgsc) + (var_cgs * var_vgsc_db7));
        let eq10_e128_d_b8: f64 = ((var_cgs_db8 * var_vgsc) + (var_cgs * var_vgsc_db8));
        let eq10_e128_d_b9: f64 = ((var_cgs_db9 * var_vgsc) + (var_cgs * var_vgsc_db9));
        let eq10_e128_d_b10: f64 = ((var_cgs_db10 * var_vgsc) + (var_cgs * var_vgsc_db10));
        let eq10_e128_d_b11: f64 = ((var_cgs_db11 * var_vgsc) + (var_cgs * var_vgsc_db11));
        let eq10_e128_d_b12: f64 = ((var_cgs_db12 * var_vgsc) + (var_cgs * var_vgsc_db12));
        let eq10_e128_d_b13: f64 = ((var_cgs_db13 * var_vgsc) + (var_cgs * var_vgsc_db13));
        let eq10_e128_d_b14: f64 = ((var_cgs_db14 * var_vgsc) + (var_cgs * var_vgsc_db14));
        let eq10_e128_d_b15: f64 = ((var_cgs_db15 * var_vgsc) + (var_cgs * var_vgsc_db15));
        let eq10_e128_d_b16: f64 = ((var_cgs_db16 * var_vgsc) + (var_cgs * var_vgsc_db16));
        let eq10_e128_d_b17: f64 = ((var_cgs_db17 * var_vgsc) + (var_cgs * var_vgsc_db17));
        let eq10_e128_d_b18: f64 = ((var_cgs_db18 * var_vgsc) + (var_cgs * var_vgsc_db18));
        let eq10_e129_q: f64 = eq10_e128;
        (eq10_e128, eq10_e128_d_n0, eq10_e128_d_n1, eq10_e128_d_n2, eq10_e128_d_n3, eq10_e128_d_n4, eq10_e128_d_n5, eq10_e128_d_n6, eq10_e128_d_n7, eq10_e128_d_n8, eq10_e128_d_n9, eq10_e128_d_n10, eq10_e128_d_n11, eq10_e128_d_n12, eq10_e128_d_n13, eq10_e128_d_n14, eq10_e128_d_n15, eq10_e128_d_b0, eq10_e128_d_b1, eq10_e128_d_b2, eq10_e128_d_b3, eq10_e128_d_b4, eq10_e128_d_b5, eq10_e128_d_b6, eq10_e128_d_b7, eq10_e128_d_b8, eq10_e128_d_b9, eq10_e128_d_b10, eq10_e128_d_b11, eq10_e128_d_b12, eq10_e128_d_b13, eq10_e128_d_b14, eq10_e129_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_reactive_node_derivatives: [f64; 16] = [eq10_e131_d_n0, eq10_e131_d_n1, eq10_e131_d_n2, eq10_e131_d_n3, eq10_e131_d_n4, eq10_e131_d_n5, eq10_e131_d_n6, eq10_e131_d_n7, eq10_e131_d_n8, eq10_e131_d_n9, eq10_e131_d_n10, eq10_e131_d_n11, eq10_e131_d_n12, eq10_e131_d_n13, eq10_e131_d_n14, eq10_e131_d_n15];
        let eq10_reactive_branch_derivatives: [f64; 15] = [eq10_e131_d_b0, eq10_e131_d_b1, eq10_e131_d_b2, eq10_e131_d_b3, eq10_e131_d_b4, eq10_e131_d_b5, eq10_e131_d_b6, eq10_e131_d_b7, eq10_e131_d_b8, eq10_e131_d_b9, eq10_e131_d_b10, eq10_e131_d_b11, eq10_e131_d_b12, eq10_e131_d_b13, eq10_e131_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq25_e218, eq25_e218_d_b5, eq25_e218_q,) = {
    if (var_guard21 != 0.0) {
        let eq25_e215: f64 = (p.p50 * bi5);
        let eq25_e216_q: f64 = eq25_e215;
        (eq25_e215, p.p50, eq25_e216_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[6],
            branches[5],
            eq25_e218_d_b5,
        );
        let (eq27_e242, eq27_e242_d_b5, eq27_e242_q,) = {
    if ((var_guard21 == 0.0) && (var_guard22 != 0.0)) {
        let eq27_e239: f64 = (p.p50 * bi5);
        let eq27_e240_q: f64 = eq27_e239;
        (eq27_e239, p.p50, eq27_e240_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[8],
            branches[5],
            eq27_e242_d_b5,
        );
        let eq32_e278: f64 = (p.p49 * bi13);
        let eq32_e279_q: f64 = eq32_e278;
        stamper.stamp_potential_reactive_branch1(
            branches[13],
            branches[13],
            p.p49,
        );
        let (eq34_e292, eq34_e292_d_b14, eq34_e292_q,) = {
    if (var_guard24 != 0.0) {
        let eq34_e289: f64 = (p.p48 * bi14);
        let eq34_e290_q: f64 = eq34_e289;
        (eq34_e289, p.p48, eq34_e290_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[15],
            branches[14],
            eq34_e292_d_b14,
        );
        let (eq36_e316, eq36_e316_d_b14, eq36_e316_q,) = {
    if ((var_guard24 == 0.0) && (var_guard25 != 0.0)) {
        let eq36_e313: f64 = (p.p48 * bi14);
        let eq36_e314_q: f64 = eq36_e313;
        (eq36_e313, p.p48, eq36_e314_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[17],
            branches[14],
            eq36_e316_d_b14,
        );
        let (eq46_e420, eq46_e420_d_n0, eq46_e420_d_n1, eq46_e420_d_n2, eq46_e420_d_n3, eq46_e420_d_n4, eq46_e420_d_n5, eq46_e420_d_n6, eq46_e420_d_n7, eq46_e420_d_n8, eq46_e420_d_n9, eq46_e420_d_n10, eq46_e420_d_n11, eq46_e420_d_n12, eq46_e420_d_n13, eq46_e420_d_n14, eq46_e420_d_n15, eq46_e420_d_b0, eq46_e420_d_b1, eq46_e420_d_b2, eq46_e420_d_b3, eq46_e420_d_b4, eq46_e420_d_b5, eq46_e420_d_b6, eq46_e420_d_b7, eq46_e420_d_b8, eq46_e420_d_b9, eq46_e420_d_b10, eq46_e420_d_b11, eq46_e420_d_b12, eq46_e420_d_b13, eq46_e420_d_b14, eq46_e420_q,) = {
    if (((var_guard27 != 0.0) && (var_guard26 == 0.0)) && (p.p0 != 0.0)) {
        let eq46_e415: f64 = (-var_ci);
        let eq46_e417: f64 = (eq46_e415 * (nv14 - 0.0));
        let eq46_e417_d_n0: f64 = ((-var_ci_dn0) * (nv14 - 0.0));
        let eq46_e417_d_n1: f64 = ((-var_ci_dn1) * (nv14 - 0.0));
        let eq46_e417_d_n2: f64 = ((-var_ci_dn2) * (nv14 - 0.0));
        let eq46_e417_d_n3: f64 = ((-var_ci_dn3) * (nv14 - 0.0));
        let eq46_e417_d_n4: f64 = ((-var_ci_dn4) * (nv14 - 0.0));
        let eq46_e417_d_n5: f64 = ((-var_ci_dn5) * (nv14 - 0.0));
        let eq46_e417_d_n6: f64 = ((-var_ci_dn6) * (nv14 - 0.0));
        let eq46_e417_d_n7: f64 = ((-var_ci_dn7) * (nv14 - 0.0));
        let eq46_e417_d_n8: f64 = ((-var_ci_dn8) * (nv14 - 0.0));
        let eq46_e417_d_n9: f64 = ((-var_ci_dn9) * (nv14 - 0.0));
        let eq46_e417_d_n10: f64 = ((-var_ci_dn10) * (nv14 - 0.0));
        let eq46_e417_d_n11: f64 = ((-var_ci_dn11) * (nv14 - 0.0));
        let eq46_e417_d_n12: f64 = ((-var_ci_dn12) * (nv14 - 0.0));
        let eq46_e417_d_n13: f64 = ((-var_ci_dn13) * (nv14 - 0.0));
        let eq46_e417_d_n14: f64 = (((-var_ci_dn14) * (nv14 - 0.0)) + eq46_e415);
        let eq46_e417_d_n15: f64 = ((-var_ci_dn15) * (nv14 - 0.0));
        let eq46_e417_d_b0: f64 = ((-var_ci_db0) * (nv14 - 0.0));
        let eq46_e417_d_b1: f64 = ((-var_ci_db1) * (nv14 - 0.0));
        let eq46_e417_d_b2: f64 = ((-var_ci_db2) * (nv14 - 0.0));
        let eq46_e417_d_b3: f64 = ((-var_ci_db3) * (nv14 - 0.0));
        let eq46_e417_d_b4: f64 = ((-var_ci_db4) * (nv14 - 0.0));
        let eq46_e417_d_b5: f64 = ((-var_ci_db5) * (nv14 - 0.0));
        let eq46_e417_d_b6: f64 = ((-var_ci_db6) * (nv14 - 0.0));
        let eq46_e417_d_b7: f64 = ((-var_ci_db7) * (nv14 - 0.0));
        let eq46_e417_d_b8: f64 = ((-var_ci_db8) * (nv14 - 0.0));
        let eq46_e417_d_b9: f64 = ((-var_ci_db9) * (nv14 - 0.0));
        let eq46_e417_d_b10: f64 = ((-var_ci_db10) * (nv14 - 0.0));
        let eq46_e417_d_b11: f64 = ((-var_ci_db11) * (nv14 - 0.0));
        let eq46_e417_d_b12: f64 = ((-var_ci_db12) * (nv14 - 0.0));
        let eq46_e417_d_b13: f64 = ((-var_ci_db13) * (nv14 - 0.0));
        let eq46_e417_d_b14: f64 = ((-var_ci_db14) * (nv14 - 0.0));
        let eq46_e418_q: f64 = eq46_e417;
        (eq46_e417, eq46_e417_d_n0, eq46_e417_d_n1, eq46_e417_d_n2, eq46_e417_d_n3, eq46_e417_d_n4, eq46_e417_d_n5, eq46_e417_d_n6, eq46_e417_d_n7, eq46_e417_d_n8, eq46_e417_d_n9, eq46_e417_d_n10, eq46_e417_d_n11, eq46_e417_d_n12, eq46_e417_d_n13, eq46_e417_d_n14, eq46_e417_d_n15, eq46_e417_d_b0, eq46_e417_d_b1, eq46_e417_d_b2, eq46_e417_d_b3, eq46_e417_d_b4, eq46_e417_d_b5, eq46_e417_d_b6, eq46_e417_d_b7, eq46_e417_d_b8, eq46_e417_d_b9, eq46_e417_d_b10, eq46_e417_d_b11, eq46_e417_d_b12, eq46_e417_d_b13, eq46_e417_d_b14, eq46_e418_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_reactive_node_derivatives: [f64; 16] = [eq46_e420_d_n0, eq46_e420_d_n1, eq46_e420_d_n2, eq46_e420_d_n3, eq46_e420_d_n4, eq46_e420_d_n5, eq46_e420_d_n6, eq46_e420_d_n7, eq46_e420_d_n8, eq46_e420_d_n9, eq46_e420_d_n10, eq46_e420_d_n11, eq46_e420_d_n12, eq46_e420_d_n13, eq46_e420_d_n14, eq46_e420_d_n15];
        let eq46_reactive_branch_derivatives: [f64; 15] = [eq46_e420_d_b0, eq46_e420_d_b1, eq46_e420_d_b2, eq46_e420_d_b3, eq46_e420_d_b4, eq46_e420_d_b5, eq46_e420_d_b6, eq46_e420_d_b7, eq46_e420_d_b8, eq46_e420_d_b9, eq46_e420_d_b10, eq46_e420_d_b11, eq46_e420_d_b12, eq46_e420_d_b13, eq46_e420_d_b14];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq57_e532, eq57_e532_d_n11, eq57_e532_q,) = {
    if (var_guard43 != 0.0) {
        let eq57_e529: f64 = (p.p58 * (nv11 - 0.0));
        let eq57_e530_q: f64 = eq57_e529;
        (eq57_e529, p.p58, eq57_e530_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[11]),
            None,
            nodes[11],
            multiplicity * (eq57_e532_d_n11),
        );
    }
}
