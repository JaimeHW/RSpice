#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_35(
        p: &Parameters,
        var_cosh1: f64,
        var_cosh1_db0: f64,
        var_cosh1_db1: f64,
        var_cosh1_db10: f64,
        var_cosh1_db11: f64,
        var_cosh1_db12: f64,
        var_cosh1_db13: f64,
        var_cosh1_db14: f64,
        var_cosh1_db15: f64,
        var_cosh1_db16: f64,
        var_cosh1_db17: f64,
        var_cosh1_db18: f64,
        var_cosh1_db2: f64,
        var_cosh1_db3: f64,
        var_cosh1_db4: f64,
        var_cosh1_db5: f64,
        var_cosh1_db6: f64,
        var_cosh1_db7: f64,
        var_cosh1_db8: f64,
        var_cosh1_db9: f64,
        var_cosh1_dn0: f64,
        var_cosh1_dn1: f64,
        var_cosh1_dn10: f64,
        var_cosh1_dn11: f64,
        var_cosh1_dn12: f64,
        var_cosh1_dn13: f64,
        var_cosh1_dn14: f64,
        var_cosh1_dn15: f64,
        var_cosh1_dn16: f64,
        var_cosh1_dn17: f64,
        var_cosh1_dn18: f64,
        var_cosh1_dn2: f64,
        var_cosh1_dn3: f64,
        var_cosh1_dn4: f64,
        var_cosh1_dn5: f64,
        var_cosh1_dn6: f64,
        var_cosh1_dn7: f64,
        var_cosh1_dn8: f64,
        var_cosh1_dn9: f64,
        var_guard14: f64,
        var_guard15: f64,
        var_guard16: f64,
        var_guard17: f64,
        var_guard18: f64,
        var_lc10: f64,
        var_lc10_db0: f64,
        var_lc10_db1: f64,
        var_lc10_db10: f64,
        var_lc10_db11: f64,
        var_lc10_db12: f64,
        var_lc10_db13: f64,
        var_lc10_db14: f64,
        var_lc10_db15: f64,
        var_lc10_db16: f64,
        var_lc10_db17: f64,
        var_lc10_db18: f64,
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
        var_lc10_dn16: f64,
        var_lc10_dn17: f64,
        var_lc10_dn18: f64,
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
        var_p10_t_db15: f64,
        var_p10_t_db16: f64,
        var_p10_t_db17: f64,
        var_p10_t_db18: f64,
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
        var_p10_t_dn16: f64,
        var_p10_t_dn17: f64,
        var_p10_t_dn18: f64,
        var_p10_t_dn2: f64,
        var_p10_t_dn3: f64,
        var_p10_t_dn4: f64,
        var_p10_t_dn5: f64,
        var_p10_t_dn6: f64,
        var_p10_t_dn7: f64,
        var_p10_t_dn8: f64,
        var_p10_t_dn9: f64,
        var_vds: f64,
        var_vds_db0: f64,
        var_vds_db1: f64,
        var_vds_db10: f64,
        var_vds_db11: f64,
        var_vds_db12: f64,
        var_vds_db13: f64,
        var_vds_db14: f64,
        var_vds_db15: f64,
        var_vds_db16: f64,
        var_vds_db17: f64,
        var_vds_db18: f64,
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
        var_vds_dn16: f64,
        var_vds_dn17: f64,
        var_vds_dn18: f64,
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
        var_vgsc_dn16: f64,
        var_vgsc_dn17: f64,
        var_vgsc_dn18: f64,
        var_vgsc_dn2: f64,
        var_vgsc_dn3: f64,
        var_vgsc_dn4: f64,
        var_vgsc_dn5: f64,
        var_vgsc_dn6: f64,
        var_vgsc_dn7: f64,
        var_vgsc_dn8: f64,
        var_vgsc_dn9: f64,
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
        var_lc1_dn16_slot: &mut f64,
        var_lc1_dn17_slot: &mut f64,
        var_lc1_dn18_slot: &mut f64,
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
        var_lc1_rdn16_slot: &mut f64,
        var_lc1_rdn17_slot: &mut f64,
        var_lc1_rdn18_slot: &mut f64,
        var_lc1_rdn2_slot: &mut f64,
        var_lc1_rdn3_slot: &mut f64,
        var_lc1_rdn4_slot: &mut f64,
        var_lc1_rdn5_slot: &mut f64,
        var_lc1_rdn6_slot: &mut f64,
        var_lc1_rdn7_slot: &mut f64,
        var_lc1_rdn8_slot: &mut f64,
        var_lc1_rdn9_slot: &mut f64,
        var_lc1_rv_slot: &mut f64,
        var_mjc_slot: &mut f64,
        var_mjc_db0_slot: &mut f64,
        var_mjc_db1_slot: &mut f64,
        var_mjc_db10_slot: &mut f64,
        var_mjc_db11_slot: &mut f64,
        var_mjc_db12_slot: &mut f64,
        var_mjc_db13_slot: &mut f64,
        var_mjc_db14_slot: &mut f64,
        var_mjc_db15_slot: &mut f64,
        var_mjc_db16_slot: &mut f64,
        var_mjc_db17_slot: &mut f64,
        var_mjc_db18_slot: &mut f64,
        var_mjc_db2_slot: &mut f64,
        var_mjc_db3_slot: &mut f64,
        var_mjc_db4_slot: &mut f64,
        var_mjc_db5_slot: &mut f64,
        var_mjc_db6_slot: &mut f64,
        var_mjc_db7_slot: &mut f64,
        var_mjc_db8_slot: &mut f64,
        var_mjc_db9_slot: &mut f64,
        var_mjc_dn0_slot: &mut f64,
        var_mjc_dn1_slot: &mut f64,
        var_mjc_dn10_slot: &mut f64,
        var_mjc_dn11_slot: &mut f64,
        var_mjc_dn12_slot: &mut f64,
        var_mjc_dn13_slot: &mut f64,
        var_mjc_dn14_slot: &mut f64,
        var_mjc_dn15_slot: &mut f64,
        var_mjc_dn16_slot: &mut f64,
        var_mjc_dn17_slot: &mut f64,
        var_mjc_dn18_slot: &mut f64,
        var_mjc_dn2_slot: &mut f64,
        var_mjc_dn3_slot: &mut f64,
        var_mjc_dn4_slot: &mut f64,
        var_mjc_dn5_slot: &mut f64,
        var_mjc_dn6_slot: &mut f64,
        var_mjc_dn7_slot: &mut f64,
        var_mjc_dn8_slot: &mut f64,
        var_mjc_dn9_slot: &mut f64,
        var_mjc_rdb0_slot: &mut f64,
        var_mjc_rdb1_slot: &mut f64,
        var_mjc_rdb10_slot: &mut f64,
        var_mjc_rdb11_slot: &mut f64,
        var_mjc_rdb12_slot: &mut f64,
        var_mjc_rdb13_slot: &mut f64,
        var_mjc_rdb14_slot: &mut f64,
        var_mjc_rdb15_slot: &mut f64,
        var_mjc_rdb16_slot: &mut f64,
        var_mjc_rdb17_slot: &mut f64,
        var_mjc_rdb18_slot: &mut f64,
        var_mjc_rdb2_slot: &mut f64,
        var_mjc_rdb3_slot: &mut f64,
        var_mjc_rdb4_slot: &mut f64,
        var_mjc_rdb5_slot: &mut f64,
        var_mjc_rdb6_slot: &mut f64,
        var_mjc_rdb7_slot: &mut f64,
        var_mjc_rdb8_slot: &mut f64,
        var_mjc_rdb9_slot: &mut f64,
        var_mjc_rdn0_slot: &mut f64,
        var_mjc_rdn1_slot: &mut f64,
        var_mjc_rdn10_slot: &mut f64,
        var_mjc_rdn11_slot: &mut f64,
        var_mjc_rdn12_slot: &mut f64,
        var_mjc_rdn13_slot: &mut f64,
        var_mjc_rdn14_slot: &mut f64,
        var_mjc_rdn15_slot: &mut f64,
        var_mjc_rdn16_slot: &mut f64,
        var_mjc_rdn17_slot: &mut f64,
        var_mjc_rdn18_slot: &mut f64,
        var_mjc_rdn2_slot: &mut f64,
        var_mjc_rdn3_slot: &mut f64,
        var_mjc_rdn4_slot: &mut f64,
        var_mjc_rdn5_slot: &mut f64,
        var_mjc_rdn6_slot: &mut f64,
        var_mjc_rdn7_slot: &mut f64,
        var_mjc_rdn8_slot: &mut f64,
        var_mjc_rdn9_slot: &mut f64,
        var_mjc_rv_slot: &mut f64,
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
        var_qgs0_dn16_slot: &mut f64,
        var_qgs0_dn17_slot: &mut f64,
        var_qgs0_dn18_slot: &mut f64,
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
        var_qgs0_rdn16_slot: &mut f64,
        var_qgs0_rdn17_slot: &mut f64,
        var_qgs0_rdn18_slot: &mut f64,
        var_qgs0_rdn2_slot: &mut f64,
        var_qgs0_rdn3_slot: &mut f64,
        var_qgs0_rdn4_slot: &mut f64,
        var_qgs0_rdn5_slot: &mut f64,
        var_qgs0_rdn6_slot: &mut f64,
        var_qgs0_rdn7_slot: &mut f64,
        var_qgs0_rdn8_slot: &mut f64,
        var_qgs0_rdn9_slot: &mut f64,
        var_qgs0_rv_slot: &mut f64,
        var_qgsdepl_slot: &mut f64,
        var_qgsdepl0_slot: &mut f64,
        var_qgsdepl0_db0_slot: &mut f64,
        var_qgsdepl0_db1_slot: &mut f64,
        var_qgsdepl0_db10_slot: &mut f64,
        var_qgsdepl0_db11_slot: &mut f64,
        var_qgsdepl0_db12_slot: &mut f64,
        var_qgsdepl0_db13_slot: &mut f64,
        var_qgsdepl0_db14_slot: &mut f64,
        var_qgsdepl0_db15_slot: &mut f64,
        var_qgsdepl0_db16_slot: &mut f64,
        var_qgsdepl0_db17_slot: &mut f64,
        var_qgsdepl0_db18_slot: &mut f64,
        var_qgsdepl0_db2_slot: &mut f64,
        var_qgsdepl0_db3_slot: &mut f64,
        var_qgsdepl0_db4_slot: &mut f64,
        var_qgsdepl0_db5_slot: &mut f64,
        var_qgsdepl0_db6_slot: &mut f64,
        var_qgsdepl0_db7_slot: &mut f64,
        var_qgsdepl0_db8_slot: &mut f64,
        var_qgsdepl0_db9_slot: &mut f64,
        var_qgsdepl0_dn0_slot: &mut f64,
        var_qgsdepl0_dn1_slot: &mut f64,
        var_qgsdepl0_dn10_slot: &mut f64,
        var_qgsdepl0_dn11_slot: &mut f64,
        var_qgsdepl0_dn12_slot: &mut f64,
        var_qgsdepl0_dn13_slot: &mut f64,
        var_qgsdepl0_dn14_slot: &mut f64,
        var_qgsdepl0_dn15_slot: &mut f64,
        var_qgsdepl0_dn16_slot: &mut f64,
        var_qgsdepl0_dn17_slot: &mut f64,
        var_qgsdepl0_dn18_slot: &mut f64,
        var_qgsdepl0_dn2_slot: &mut f64,
        var_qgsdepl0_dn3_slot: &mut f64,
        var_qgsdepl0_dn4_slot: &mut f64,
        var_qgsdepl0_dn5_slot: &mut f64,
        var_qgsdepl0_dn6_slot: &mut f64,
        var_qgsdepl0_dn7_slot: &mut f64,
        var_qgsdepl0_dn8_slot: &mut f64,
        var_qgsdepl0_dn9_slot: &mut f64,
        var_qgsdepl0_rdb0_slot: &mut f64,
        var_qgsdepl0_rdb1_slot: &mut f64,
        var_qgsdepl0_rdb10_slot: &mut f64,
        var_qgsdepl0_rdb11_slot: &mut f64,
        var_qgsdepl0_rdb12_slot: &mut f64,
        var_qgsdepl0_rdb13_slot: &mut f64,
        var_qgsdepl0_rdb14_slot: &mut f64,
        var_qgsdepl0_rdb15_slot: &mut f64,
        var_qgsdepl0_rdb16_slot: &mut f64,
        var_qgsdepl0_rdb17_slot: &mut f64,
        var_qgsdepl0_rdb18_slot: &mut f64,
        var_qgsdepl0_rdb2_slot: &mut f64,
        var_qgsdepl0_rdb3_slot: &mut f64,
        var_qgsdepl0_rdb4_slot: &mut f64,
        var_qgsdepl0_rdb5_slot: &mut f64,
        var_qgsdepl0_rdb6_slot: &mut f64,
        var_qgsdepl0_rdb7_slot: &mut f64,
        var_qgsdepl0_rdb8_slot: &mut f64,
        var_qgsdepl0_rdb9_slot: &mut f64,
        var_qgsdepl0_rdn0_slot: &mut f64,
        var_qgsdepl0_rdn1_slot: &mut f64,
        var_qgsdepl0_rdn10_slot: &mut f64,
        var_qgsdepl0_rdn11_slot: &mut f64,
        var_qgsdepl0_rdn12_slot: &mut f64,
        var_qgsdepl0_rdn13_slot: &mut f64,
        var_qgsdepl0_rdn14_slot: &mut f64,
        var_qgsdepl0_rdn15_slot: &mut f64,
        var_qgsdepl0_rdn16_slot: &mut f64,
        var_qgsdepl0_rdn17_slot: &mut f64,
        var_qgsdepl0_rdn18_slot: &mut f64,
        var_qgsdepl0_rdn2_slot: &mut f64,
        var_qgsdepl0_rdn3_slot: &mut f64,
        var_qgsdepl0_rdn4_slot: &mut f64,
        var_qgsdepl0_rdn5_slot: &mut f64,
        var_qgsdepl0_rdn6_slot: &mut f64,
        var_qgsdepl0_rdn7_slot: &mut f64,
        var_qgsdepl0_rdn8_slot: &mut f64,
        var_qgsdepl0_rdn9_slot: &mut f64,
        var_qgsdepl0_rv_slot: &mut f64,
        var_qgsdepl_db0_slot: &mut f64,
        var_qgsdepl_db1_slot: &mut f64,
        var_qgsdepl_db10_slot: &mut f64,
        var_qgsdepl_db11_slot: &mut f64,
        var_qgsdepl_db12_slot: &mut f64,
        var_qgsdepl_db13_slot: &mut f64,
        var_qgsdepl_db14_slot: &mut f64,
        var_qgsdepl_db15_slot: &mut f64,
        var_qgsdepl_db16_slot: &mut f64,
        var_qgsdepl_db17_slot: &mut f64,
        var_qgsdepl_db18_slot: &mut f64,
        var_qgsdepl_db2_slot: &mut f64,
        var_qgsdepl_db3_slot: &mut f64,
        var_qgsdepl_db4_slot: &mut f64,
        var_qgsdepl_db5_slot: &mut f64,
        var_qgsdepl_db6_slot: &mut f64,
        var_qgsdepl_db7_slot: &mut f64,
        var_qgsdepl_db8_slot: &mut f64,
        var_qgsdepl_db9_slot: &mut f64,
        var_qgsdepl_dn0_slot: &mut f64,
        var_qgsdepl_dn1_slot: &mut f64,
        var_qgsdepl_dn10_slot: &mut f64,
        var_qgsdepl_dn11_slot: &mut f64,
        var_qgsdepl_dn12_slot: &mut f64,
        var_qgsdepl_dn13_slot: &mut f64,
        var_qgsdepl_dn14_slot: &mut f64,
        var_qgsdepl_dn15_slot: &mut f64,
        var_qgsdepl_dn16_slot: &mut f64,
        var_qgsdepl_dn17_slot: &mut f64,
        var_qgsdepl_dn18_slot: &mut f64,
        var_qgsdepl_dn2_slot: &mut f64,
        var_qgsdepl_dn3_slot: &mut f64,
        var_qgsdepl_dn4_slot: &mut f64,
        var_qgsdepl_dn5_slot: &mut f64,
        var_qgsdepl_dn6_slot: &mut f64,
        var_qgsdepl_dn7_slot: &mut f64,
        var_qgsdepl_dn8_slot: &mut f64,
        var_qgsdepl_dn9_slot: &mut f64,
        var_qgsdepl_rdb0_slot: &mut f64,
        var_qgsdepl_rdb1_slot: &mut f64,
        var_qgsdepl_rdb10_slot: &mut f64,
        var_qgsdepl_rdb11_slot: &mut f64,
        var_qgsdepl_rdb12_slot: &mut f64,
        var_qgsdepl_rdb13_slot: &mut f64,
        var_qgsdepl_rdb14_slot: &mut f64,
        var_qgsdepl_rdb15_slot: &mut f64,
        var_qgsdepl_rdb16_slot: &mut f64,
        var_qgsdepl_rdb17_slot: &mut f64,
        var_qgsdepl_rdb18_slot: &mut f64,
        var_qgsdepl_rdb2_slot: &mut f64,
        var_qgsdepl_rdb3_slot: &mut f64,
        var_qgsdepl_rdb4_slot: &mut f64,
        var_qgsdepl_rdb5_slot: &mut f64,
        var_qgsdepl_rdb6_slot: &mut f64,
        var_qgsdepl_rdb7_slot: &mut f64,
        var_qgsdepl_rdb8_slot: &mut f64,
        var_qgsdepl_rdb9_slot: &mut f64,
        var_qgsdepl_rdn0_slot: &mut f64,
        var_qgsdepl_rdn1_slot: &mut f64,
        var_qgsdepl_rdn10_slot: &mut f64,
        var_qgsdepl_rdn11_slot: &mut f64,
        var_qgsdepl_rdn12_slot: &mut f64,
        var_qgsdepl_rdn13_slot: &mut f64,
        var_qgsdepl_rdn14_slot: &mut f64,
        var_qgsdepl_rdn15_slot: &mut f64,
        var_qgsdepl_rdn16_slot: &mut f64,
        var_qgsdepl_rdn17_slot: &mut f64,
        var_qgsdepl_rdn18_slot: &mut f64,
        var_qgsdepl_rdn2_slot: &mut f64,
        var_qgsdepl_rdn3_slot: &mut f64,
        var_qgsdepl_rdn4_slot: &mut f64,
        var_qgsdepl_rdn5_slot: &mut f64,
        var_qgsdepl_rdn6_slot: &mut f64,
        var_qgsdepl_rdn7_slot: &mut f64,
        var_qgsdepl_rdn8_slot: &mut f64,
        var_qgsdepl_rdn9_slot: &mut f64,
        var_qgsdepl_rv_slot: &mut f64,
    ) {
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
        let mut var_lc1_dn16: f64 = *var_lc1_dn16_slot;
        let mut var_lc1_dn17: f64 = *var_lc1_dn17_slot;
        let mut var_lc1_dn18: f64 = *var_lc1_dn18_slot;
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
        let mut var_lc1_rdn16: f64 = *var_lc1_rdn16_slot;
        let mut var_lc1_rdn17: f64 = *var_lc1_rdn17_slot;
        let mut var_lc1_rdn18: f64 = *var_lc1_rdn18_slot;
        let mut var_lc1_rdn2: f64 = *var_lc1_rdn2_slot;
        let mut var_lc1_rdn3: f64 = *var_lc1_rdn3_slot;
        let mut var_lc1_rdn4: f64 = *var_lc1_rdn4_slot;
        let mut var_lc1_rdn5: f64 = *var_lc1_rdn5_slot;
        let mut var_lc1_rdn6: f64 = *var_lc1_rdn6_slot;
        let mut var_lc1_rdn7: f64 = *var_lc1_rdn7_slot;
        let mut var_lc1_rdn8: f64 = *var_lc1_rdn8_slot;
        let mut var_lc1_rdn9: f64 = *var_lc1_rdn9_slot;
        let mut var_lc1_rv: f64 = *var_lc1_rv_slot;
        let mut var_mjc: f64 = *var_mjc_slot;
        let mut var_mjc_db0: f64 = *var_mjc_db0_slot;
        let mut var_mjc_db1: f64 = *var_mjc_db1_slot;
        let mut var_mjc_db10: f64 = *var_mjc_db10_slot;
        let mut var_mjc_db11: f64 = *var_mjc_db11_slot;
        let mut var_mjc_db12: f64 = *var_mjc_db12_slot;
        let mut var_mjc_db13: f64 = *var_mjc_db13_slot;
        let mut var_mjc_db14: f64 = *var_mjc_db14_slot;
        let mut var_mjc_db15: f64 = *var_mjc_db15_slot;
        let mut var_mjc_db16: f64 = *var_mjc_db16_slot;
        let mut var_mjc_db17: f64 = *var_mjc_db17_slot;
        let mut var_mjc_db18: f64 = *var_mjc_db18_slot;
        let mut var_mjc_db2: f64 = *var_mjc_db2_slot;
        let mut var_mjc_db3: f64 = *var_mjc_db3_slot;
        let mut var_mjc_db4: f64 = *var_mjc_db4_slot;
        let mut var_mjc_db5: f64 = *var_mjc_db5_slot;
        let mut var_mjc_db6: f64 = *var_mjc_db6_slot;
        let mut var_mjc_db7: f64 = *var_mjc_db7_slot;
        let mut var_mjc_db8: f64 = *var_mjc_db8_slot;
        let mut var_mjc_db9: f64 = *var_mjc_db9_slot;
        let mut var_mjc_dn0: f64 = *var_mjc_dn0_slot;
        let mut var_mjc_dn1: f64 = *var_mjc_dn1_slot;
        let mut var_mjc_dn10: f64 = *var_mjc_dn10_slot;
        let mut var_mjc_dn11: f64 = *var_mjc_dn11_slot;
        let mut var_mjc_dn12: f64 = *var_mjc_dn12_slot;
        let mut var_mjc_dn13: f64 = *var_mjc_dn13_slot;
        let mut var_mjc_dn14: f64 = *var_mjc_dn14_slot;
        let mut var_mjc_dn15: f64 = *var_mjc_dn15_slot;
        let mut var_mjc_dn16: f64 = *var_mjc_dn16_slot;
        let mut var_mjc_dn17: f64 = *var_mjc_dn17_slot;
        let mut var_mjc_dn18: f64 = *var_mjc_dn18_slot;
        let mut var_mjc_dn2: f64 = *var_mjc_dn2_slot;
        let mut var_mjc_dn3: f64 = *var_mjc_dn3_slot;
        let mut var_mjc_dn4: f64 = *var_mjc_dn4_slot;
        let mut var_mjc_dn5: f64 = *var_mjc_dn5_slot;
        let mut var_mjc_dn6: f64 = *var_mjc_dn6_slot;
        let mut var_mjc_dn7: f64 = *var_mjc_dn7_slot;
        let mut var_mjc_dn8: f64 = *var_mjc_dn8_slot;
        let mut var_mjc_dn9: f64 = *var_mjc_dn9_slot;
        let mut var_mjc_rdb0: f64 = *var_mjc_rdb0_slot;
        let mut var_mjc_rdb1: f64 = *var_mjc_rdb1_slot;
        let mut var_mjc_rdb10: f64 = *var_mjc_rdb10_slot;
        let mut var_mjc_rdb11: f64 = *var_mjc_rdb11_slot;
        let mut var_mjc_rdb12: f64 = *var_mjc_rdb12_slot;
        let mut var_mjc_rdb13: f64 = *var_mjc_rdb13_slot;
        let mut var_mjc_rdb14: f64 = *var_mjc_rdb14_slot;
        let mut var_mjc_rdb15: f64 = *var_mjc_rdb15_slot;
        let mut var_mjc_rdb16: f64 = *var_mjc_rdb16_slot;
        let mut var_mjc_rdb17: f64 = *var_mjc_rdb17_slot;
        let mut var_mjc_rdb18: f64 = *var_mjc_rdb18_slot;
        let mut var_mjc_rdb2: f64 = *var_mjc_rdb2_slot;
        let mut var_mjc_rdb3: f64 = *var_mjc_rdb3_slot;
        let mut var_mjc_rdb4: f64 = *var_mjc_rdb4_slot;
        let mut var_mjc_rdb5: f64 = *var_mjc_rdb5_slot;
        let mut var_mjc_rdb6: f64 = *var_mjc_rdb6_slot;
        let mut var_mjc_rdb7: f64 = *var_mjc_rdb7_slot;
        let mut var_mjc_rdb8: f64 = *var_mjc_rdb8_slot;
        let mut var_mjc_rdb9: f64 = *var_mjc_rdb9_slot;
        let mut var_mjc_rdn0: f64 = *var_mjc_rdn0_slot;
        let mut var_mjc_rdn1: f64 = *var_mjc_rdn1_slot;
        let mut var_mjc_rdn10: f64 = *var_mjc_rdn10_slot;
        let mut var_mjc_rdn11: f64 = *var_mjc_rdn11_slot;
        let mut var_mjc_rdn12: f64 = *var_mjc_rdn12_slot;
        let mut var_mjc_rdn13: f64 = *var_mjc_rdn13_slot;
        let mut var_mjc_rdn14: f64 = *var_mjc_rdn14_slot;
        let mut var_mjc_rdn15: f64 = *var_mjc_rdn15_slot;
        let mut var_mjc_rdn16: f64 = *var_mjc_rdn16_slot;
        let mut var_mjc_rdn17: f64 = *var_mjc_rdn17_slot;
        let mut var_mjc_rdn18: f64 = *var_mjc_rdn18_slot;
        let mut var_mjc_rdn2: f64 = *var_mjc_rdn2_slot;
        let mut var_mjc_rdn3: f64 = *var_mjc_rdn3_slot;
        let mut var_mjc_rdn4: f64 = *var_mjc_rdn4_slot;
        let mut var_mjc_rdn5: f64 = *var_mjc_rdn5_slot;
        let mut var_mjc_rdn6: f64 = *var_mjc_rdn6_slot;
        let mut var_mjc_rdn7: f64 = *var_mjc_rdn7_slot;
        let mut var_mjc_rdn8: f64 = *var_mjc_rdn8_slot;
        let mut var_mjc_rdn9: f64 = *var_mjc_rdn9_slot;
        let mut var_mjc_rv: f64 = *var_mjc_rv_slot;
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
        let mut var_qgs0_dn16: f64 = *var_qgs0_dn16_slot;
        let mut var_qgs0_dn17: f64 = *var_qgs0_dn17_slot;
        let mut var_qgs0_dn18: f64 = *var_qgs0_dn18_slot;
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
        let mut var_qgs0_rdn16: f64 = *var_qgs0_rdn16_slot;
        let mut var_qgs0_rdn17: f64 = *var_qgs0_rdn17_slot;
        let mut var_qgs0_rdn18: f64 = *var_qgs0_rdn18_slot;
        let mut var_qgs0_rdn2: f64 = *var_qgs0_rdn2_slot;
        let mut var_qgs0_rdn3: f64 = *var_qgs0_rdn3_slot;
        let mut var_qgs0_rdn4: f64 = *var_qgs0_rdn4_slot;
        let mut var_qgs0_rdn5: f64 = *var_qgs0_rdn5_slot;
        let mut var_qgs0_rdn6: f64 = *var_qgs0_rdn6_slot;
        let mut var_qgs0_rdn7: f64 = *var_qgs0_rdn7_slot;
        let mut var_qgs0_rdn8: f64 = *var_qgs0_rdn8_slot;
        let mut var_qgs0_rdn9: f64 = *var_qgs0_rdn9_slot;
        let mut var_qgs0_rv: f64 = *var_qgs0_rv_slot;
        let mut var_qgsdepl: f64 = *var_qgsdepl_slot;
        let mut var_qgsdepl0: f64 = *var_qgsdepl0_slot;
        let mut var_qgsdepl0_db0: f64 = *var_qgsdepl0_db0_slot;
        let mut var_qgsdepl0_db1: f64 = *var_qgsdepl0_db1_slot;
        let mut var_qgsdepl0_db10: f64 = *var_qgsdepl0_db10_slot;
        let mut var_qgsdepl0_db11: f64 = *var_qgsdepl0_db11_slot;
        let mut var_qgsdepl0_db12: f64 = *var_qgsdepl0_db12_slot;
        let mut var_qgsdepl0_db13: f64 = *var_qgsdepl0_db13_slot;
        let mut var_qgsdepl0_db14: f64 = *var_qgsdepl0_db14_slot;
        let mut var_qgsdepl0_db15: f64 = *var_qgsdepl0_db15_slot;
        let mut var_qgsdepl0_db16: f64 = *var_qgsdepl0_db16_slot;
        let mut var_qgsdepl0_db17: f64 = *var_qgsdepl0_db17_slot;
        let mut var_qgsdepl0_db18: f64 = *var_qgsdepl0_db18_slot;
        let mut var_qgsdepl0_db2: f64 = *var_qgsdepl0_db2_slot;
        let mut var_qgsdepl0_db3: f64 = *var_qgsdepl0_db3_slot;
        let mut var_qgsdepl0_db4: f64 = *var_qgsdepl0_db4_slot;
        let mut var_qgsdepl0_db5: f64 = *var_qgsdepl0_db5_slot;
        let mut var_qgsdepl0_db6: f64 = *var_qgsdepl0_db6_slot;
        let mut var_qgsdepl0_db7: f64 = *var_qgsdepl0_db7_slot;
        let mut var_qgsdepl0_db8: f64 = *var_qgsdepl0_db8_slot;
        let mut var_qgsdepl0_db9: f64 = *var_qgsdepl0_db9_slot;
        let mut var_qgsdepl0_dn0: f64 = *var_qgsdepl0_dn0_slot;
        let mut var_qgsdepl0_dn1: f64 = *var_qgsdepl0_dn1_slot;
        let mut var_qgsdepl0_dn10: f64 = *var_qgsdepl0_dn10_slot;
        let mut var_qgsdepl0_dn11: f64 = *var_qgsdepl0_dn11_slot;
        let mut var_qgsdepl0_dn12: f64 = *var_qgsdepl0_dn12_slot;
        let mut var_qgsdepl0_dn13: f64 = *var_qgsdepl0_dn13_slot;
        let mut var_qgsdepl0_dn14: f64 = *var_qgsdepl0_dn14_slot;
        let mut var_qgsdepl0_dn15: f64 = *var_qgsdepl0_dn15_slot;
        let mut var_qgsdepl0_dn16: f64 = *var_qgsdepl0_dn16_slot;
        let mut var_qgsdepl0_dn17: f64 = *var_qgsdepl0_dn17_slot;
        let mut var_qgsdepl0_dn18: f64 = *var_qgsdepl0_dn18_slot;
        let mut var_qgsdepl0_dn2: f64 = *var_qgsdepl0_dn2_slot;
        let mut var_qgsdepl0_dn3: f64 = *var_qgsdepl0_dn3_slot;
        let mut var_qgsdepl0_dn4: f64 = *var_qgsdepl0_dn4_slot;
        let mut var_qgsdepl0_dn5: f64 = *var_qgsdepl0_dn5_slot;
        let mut var_qgsdepl0_dn6: f64 = *var_qgsdepl0_dn6_slot;
        let mut var_qgsdepl0_dn7: f64 = *var_qgsdepl0_dn7_slot;
        let mut var_qgsdepl0_dn8: f64 = *var_qgsdepl0_dn8_slot;
        let mut var_qgsdepl0_dn9: f64 = *var_qgsdepl0_dn9_slot;
        let mut var_qgsdepl0_rdb0: f64 = *var_qgsdepl0_rdb0_slot;
        let mut var_qgsdepl0_rdb1: f64 = *var_qgsdepl0_rdb1_slot;
        let mut var_qgsdepl0_rdb10: f64 = *var_qgsdepl0_rdb10_slot;
        let mut var_qgsdepl0_rdb11: f64 = *var_qgsdepl0_rdb11_slot;
        let mut var_qgsdepl0_rdb12: f64 = *var_qgsdepl0_rdb12_slot;
        let mut var_qgsdepl0_rdb13: f64 = *var_qgsdepl0_rdb13_slot;
        let mut var_qgsdepl0_rdb14: f64 = *var_qgsdepl0_rdb14_slot;
        let mut var_qgsdepl0_rdb15: f64 = *var_qgsdepl0_rdb15_slot;
        let mut var_qgsdepl0_rdb16: f64 = *var_qgsdepl0_rdb16_slot;
        let mut var_qgsdepl0_rdb17: f64 = *var_qgsdepl0_rdb17_slot;
        let mut var_qgsdepl0_rdb18: f64 = *var_qgsdepl0_rdb18_slot;
        let mut var_qgsdepl0_rdb2: f64 = *var_qgsdepl0_rdb2_slot;
        let mut var_qgsdepl0_rdb3: f64 = *var_qgsdepl0_rdb3_slot;
        let mut var_qgsdepl0_rdb4: f64 = *var_qgsdepl0_rdb4_slot;
        let mut var_qgsdepl0_rdb5: f64 = *var_qgsdepl0_rdb5_slot;
        let mut var_qgsdepl0_rdb6: f64 = *var_qgsdepl0_rdb6_slot;
        let mut var_qgsdepl0_rdb7: f64 = *var_qgsdepl0_rdb7_slot;
        let mut var_qgsdepl0_rdb8: f64 = *var_qgsdepl0_rdb8_slot;
        let mut var_qgsdepl0_rdb9: f64 = *var_qgsdepl0_rdb9_slot;
        let mut var_qgsdepl0_rdn0: f64 = *var_qgsdepl0_rdn0_slot;
        let mut var_qgsdepl0_rdn1: f64 = *var_qgsdepl0_rdn1_slot;
        let mut var_qgsdepl0_rdn10: f64 = *var_qgsdepl0_rdn10_slot;
        let mut var_qgsdepl0_rdn11: f64 = *var_qgsdepl0_rdn11_slot;
        let mut var_qgsdepl0_rdn12: f64 = *var_qgsdepl0_rdn12_slot;
        let mut var_qgsdepl0_rdn13: f64 = *var_qgsdepl0_rdn13_slot;
        let mut var_qgsdepl0_rdn14: f64 = *var_qgsdepl0_rdn14_slot;
        let mut var_qgsdepl0_rdn15: f64 = *var_qgsdepl0_rdn15_slot;
        let mut var_qgsdepl0_rdn16: f64 = *var_qgsdepl0_rdn16_slot;
        let mut var_qgsdepl0_rdn17: f64 = *var_qgsdepl0_rdn17_slot;
        let mut var_qgsdepl0_rdn18: f64 = *var_qgsdepl0_rdn18_slot;
        let mut var_qgsdepl0_rdn2: f64 = *var_qgsdepl0_rdn2_slot;
        let mut var_qgsdepl0_rdn3: f64 = *var_qgsdepl0_rdn3_slot;
        let mut var_qgsdepl0_rdn4: f64 = *var_qgsdepl0_rdn4_slot;
        let mut var_qgsdepl0_rdn5: f64 = *var_qgsdepl0_rdn5_slot;
        let mut var_qgsdepl0_rdn6: f64 = *var_qgsdepl0_rdn6_slot;
        let mut var_qgsdepl0_rdn7: f64 = *var_qgsdepl0_rdn7_slot;
        let mut var_qgsdepl0_rdn8: f64 = *var_qgsdepl0_rdn8_slot;
        let mut var_qgsdepl0_rdn9: f64 = *var_qgsdepl0_rdn9_slot;
        let mut var_qgsdepl0_rv: f64 = *var_qgsdepl0_rv_slot;
        let mut var_qgsdepl_db0: f64 = *var_qgsdepl_db0_slot;
        let mut var_qgsdepl_db1: f64 = *var_qgsdepl_db1_slot;
        let mut var_qgsdepl_db10: f64 = *var_qgsdepl_db10_slot;
        let mut var_qgsdepl_db11: f64 = *var_qgsdepl_db11_slot;
        let mut var_qgsdepl_db12: f64 = *var_qgsdepl_db12_slot;
        let mut var_qgsdepl_db13: f64 = *var_qgsdepl_db13_slot;
        let mut var_qgsdepl_db14: f64 = *var_qgsdepl_db14_slot;
        let mut var_qgsdepl_db15: f64 = *var_qgsdepl_db15_slot;
        let mut var_qgsdepl_db16: f64 = *var_qgsdepl_db16_slot;
        let mut var_qgsdepl_db17: f64 = *var_qgsdepl_db17_slot;
        let mut var_qgsdepl_db18: f64 = *var_qgsdepl_db18_slot;
        let mut var_qgsdepl_db2: f64 = *var_qgsdepl_db2_slot;
        let mut var_qgsdepl_db3: f64 = *var_qgsdepl_db3_slot;
        let mut var_qgsdepl_db4: f64 = *var_qgsdepl_db4_slot;
        let mut var_qgsdepl_db5: f64 = *var_qgsdepl_db5_slot;
        let mut var_qgsdepl_db6: f64 = *var_qgsdepl_db6_slot;
        let mut var_qgsdepl_db7: f64 = *var_qgsdepl_db7_slot;
        let mut var_qgsdepl_db8: f64 = *var_qgsdepl_db8_slot;
        let mut var_qgsdepl_db9: f64 = *var_qgsdepl_db9_slot;
        let mut var_qgsdepl_dn0: f64 = *var_qgsdepl_dn0_slot;
        let mut var_qgsdepl_dn1: f64 = *var_qgsdepl_dn1_slot;
        let mut var_qgsdepl_dn10: f64 = *var_qgsdepl_dn10_slot;
        let mut var_qgsdepl_dn11: f64 = *var_qgsdepl_dn11_slot;
        let mut var_qgsdepl_dn12: f64 = *var_qgsdepl_dn12_slot;
        let mut var_qgsdepl_dn13: f64 = *var_qgsdepl_dn13_slot;
        let mut var_qgsdepl_dn14: f64 = *var_qgsdepl_dn14_slot;
        let mut var_qgsdepl_dn15: f64 = *var_qgsdepl_dn15_slot;
        let mut var_qgsdepl_dn16: f64 = *var_qgsdepl_dn16_slot;
        let mut var_qgsdepl_dn17: f64 = *var_qgsdepl_dn17_slot;
        let mut var_qgsdepl_dn18: f64 = *var_qgsdepl_dn18_slot;
        let mut var_qgsdepl_dn2: f64 = *var_qgsdepl_dn2_slot;
        let mut var_qgsdepl_dn3: f64 = *var_qgsdepl_dn3_slot;
        let mut var_qgsdepl_dn4: f64 = *var_qgsdepl_dn4_slot;
        let mut var_qgsdepl_dn5: f64 = *var_qgsdepl_dn5_slot;
        let mut var_qgsdepl_dn6: f64 = *var_qgsdepl_dn6_slot;
        let mut var_qgsdepl_dn7: f64 = *var_qgsdepl_dn7_slot;
        let mut var_qgsdepl_dn8: f64 = *var_qgsdepl_dn8_slot;
        let mut var_qgsdepl_dn9: f64 = *var_qgsdepl_dn9_slot;
        let mut var_qgsdepl_rdb0: f64 = *var_qgsdepl_rdb0_slot;
        let mut var_qgsdepl_rdb1: f64 = *var_qgsdepl_rdb1_slot;
        let mut var_qgsdepl_rdb10: f64 = *var_qgsdepl_rdb10_slot;
        let mut var_qgsdepl_rdb11: f64 = *var_qgsdepl_rdb11_slot;
        let mut var_qgsdepl_rdb12: f64 = *var_qgsdepl_rdb12_slot;
        let mut var_qgsdepl_rdb13: f64 = *var_qgsdepl_rdb13_slot;
        let mut var_qgsdepl_rdb14: f64 = *var_qgsdepl_rdb14_slot;
        let mut var_qgsdepl_rdb15: f64 = *var_qgsdepl_rdb15_slot;
        let mut var_qgsdepl_rdb16: f64 = *var_qgsdepl_rdb16_slot;
        let mut var_qgsdepl_rdb17: f64 = *var_qgsdepl_rdb17_slot;
        let mut var_qgsdepl_rdb18: f64 = *var_qgsdepl_rdb18_slot;
        let mut var_qgsdepl_rdb2: f64 = *var_qgsdepl_rdb2_slot;
        let mut var_qgsdepl_rdb3: f64 = *var_qgsdepl_rdb3_slot;
        let mut var_qgsdepl_rdb4: f64 = *var_qgsdepl_rdb4_slot;
        let mut var_qgsdepl_rdb5: f64 = *var_qgsdepl_rdb5_slot;
        let mut var_qgsdepl_rdb6: f64 = *var_qgsdepl_rdb6_slot;
        let mut var_qgsdepl_rdb7: f64 = *var_qgsdepl_rdb7_slot;
        let mut var_qgsdepl_rdb8: f64 = *var_qgsdepl_rdb8_slot;
        let mut var_qgsdepl_rdb9: f64 = *var_qgsdepl_rdb9_slot;
        let mut var_qgsdepl_rdn0: f64 = *var_qgsdepl_rdn0_slot;
        let mut var_qgsdepl_rdn1: f64 = *var_qgsdepl_rdn1_slot;
        let mut var_qgsdepl_rdn10: f64 = *var_qgsdepl_rdn10_slot;
        let mut var_qgsdepl_rdn11: f64 = *var_qgsdepl_rdn11_slot;
        let mut var_qgsdepl_rdn12: f64 = *var_qgsdepl_rdn12_slot;
        let mut var_qgsdepl_rdn13: f64 = *var_qgsdepl_rdn13_slot;
        let mut var_qgsdepl_rdn14: f64 = *var_qgsdepl_rdn14_slot;
        let mut var_qgsdepl_rdn15: f64 = *var_qgsdepl_rdn15_slot;
        let mut var_qgsdepl_rdn16: f64 = *var_qgsdepl_rdn16_slot;
        let mut var_qgsdepl_rdn17: f64 = *var_qgsdepl_rdn17_slot;
        let mut var_qgsdepl_rdn18: f64 = *var_qgsdepl_rdn18_slot;
        let mut var_qgsdepl_rdn2: f64 = *var_qgsdepl_rdn2_slot;
        let mut var_qgsdepl_rdn3: f64 = *var_qgsdepl_rdn3_slot;
        let mut var_qgsdepl_rdn4: f64 = *var_qgsdepl_rdn4_slot;
        let mut var_qgsdepl_rdn5: f64 = *var_qgsdepl_rdn5_slot;
        let mut var_qgsdepl_rdn6: f64 = *var_qgsdepl_rdn6_slot;
        let mut var_qgsdepl_rdn7: f64 = *var_qgsdepl_rdn7_slot;
        let mut var_qgsdepl_rdn8: f64 = *var_qgsdepl_rdn8_slot;
        let mut var_qgsdepl_rdn9: f64 = *var_qgsdepl_rdn9_slot;
        let mut var_qgsdepl_rv: f64 = *var_qgsdepl_rv_slot;

        let (assign1940_e2549, assign1940_e2549_d_n0, assign1940_e2549_d_n1, assign1940_e2549_d_n2, assign1940_e2549_d_n3, assign1940_e2549_d_n4, assign1940_e2549_d_n5, assign1940_e2549_d_n6, assign1940_e2549_d_n7, assign1940_e2549_d_n8, assign1940_e2549_d_n9, assign1940_e2549_d_n10, assign1940_e2549_d_n11, assign1940_e2549_d_n12, assign1940_e2549_d_n13, assign1940_e2549_d_n14, assign1940_e2549_d_n15, assign1940_e2549_d_n16, assign1940_e2549_d_n17, assign1940_e2549_d_n18, assign1940_e2549_d_b0, assign1940_e2549_d_b1, assign1940_e2549_d_b2, assign1940_e2549_d_b3, assign1940_e2549_d_b4, assign1940_e2549_d_b5, assign1940_e2549_d_b6, assign1940_e2549_d_b7, assign1940_e2549_d_b8, assign1940_e2549_d_b9, assign1940_e2549_d_b10, assign1940_e2549_d_b11, assign1940_e2549_d_b12, assign1940_e2549_d_b13, assign1940_e2549_d_b14, assign1940_e2549_d_b15, assign1940_e2549_d_b16, assign1940_e2549_d_b17, assign1940_e2549_d_b18,) = {
    if ((var_guard18 != 0.0) && (!((((var_guard14 != 0.0) || (var_guard15 != 0.0)) || (var_guard16 != 0.0)) || (var_guard17 != 0.0)))) {
        let assign1940_e2547: f64 = (var_cosh1).ln();
        (assign1940_e2547, (var_cosh1_dn0 / var_cosh1), (var_cosh1_dn1 / var_cosh1), (var_cosh1_dn2 / var_cosh1), (var_cosh1_dn3 / var_cosh1), (var_cosh1_dn4 / var_cosh1), (var_cosh1_dn5 / var_cosh1), (var_cosh1_dn6 / var_cosh1), (var_cosh1_dn7 / var_cosh1), (var_cosh1_dn8 / var_cosh1), (var_cosh1_dn9 / var_cosh1), (var_cosh1_dn10 / var_cosh1), (var_cosh1_dn11 / var_cosh1), (var_cosh1_dn12 / var_cosh1), (var_cosh1_dn13 / var_cosh1), (var_cosh1_dn14 / var_cosh1), (var_cosh1_dn15 / var_cosh1), (var_cosh1_dn16 / var_cosh1), (var_cosh1_dn17 / var_cosh1), (var_cosh1_dn18 / var_cosh1), (var_cosh1_db0 / var_cosh1), (var_cosh1_db1 / var_cosh1), (var_cosh1_db2 / var_cosh1), (var_cosh1_db3 / var_cosh1), (var_cosh1_db4 / var_cosh1), (var_cosh1_db5 / var_cosh1), (var_cosh1_db6 / var_cosh1), (var_cosh1_db7 / var_cosh1), (var_cosh1_db8 / var_cosh1), (var_cosh1_db9 / var_cosh1), (var_cosh1_db10 / var_cosh1), (var_cosh1_db11 / var_cosh1), (var_cosh1_db12 / var_cosh1), (var_cosh1_db13 / var_cosh1), (var_cosh1_db14 / var_cosh1), (var_cosh1_db15 / var_cosh1), (var_cosh1_db16 / var_cosh1), (var_cosh1_db17 / var_cosh1), (var_cosh1_db18 / var_cosh1),)
    } else {
        (var_lc1, var_lc1_dn0, var_lc1_dn1, var_lc1_dn2, var_lc1_dn3, var_lc1_dn4, var_lc1_dn5, var_lc1_dn6, var_lc1_dn7, var_lc1_dn8, var_lc1_dn9, var_lc1_dn10, var_lc1_dn11, var_lc1_dn12, var_lc1_dn13, var_lc1_dn14, var_lc1_dn15, var_lc1_dn16, var_lc1_dn17, var_lc1_dn18, var_lc1_db0, var_lc1_db1, var_lc1_db2, var_lc1_db3, var_lc1_db4, var_lc1_db5, var_lc1_db6, var_lc1_db7, var_lc1_db8, var_lc1_db9, var_lc1_db10, var_lc1_db11, var_lc1_db12, var_lc1_db13, var_lc1_db14, var_lc1_db15, var_lc1_db16, var_lc1_db17, var_lc1_db18,)
    }
};
        var_lc1 = assign1940_e2549;
        var_lc1_dn0 = assign1940_e2549_d_n0;
        var_lc1_dn1 = assign1940_e2549_d_n1;
        var_lc1_dn2 = assign1940_e2549_d_n2;
        var_lc1_dn3 = assign1940_e2549_d_n3;
        var_lc1_dn4 = assign1940_e2549_d_n4;
        var_lc1_dn5 = assign1940_e2549_d_n5;
        var_lc1_dn6 = assign1940_e2549_d_n6;
        var_lc1_dn7 = assign1940_e2549_d_n7;
        var_lc1_dn8 = assign1940_e2549_d_n8;
        var_lc1_dn9 = assign1940_e2549_d_n9;
        var_lc1_dn10 = assign1940_e2549_d_n10;
        var_lc1_dn11 = assign1940_e2549_d_n11;
        var_lc1_dn12 = assign1940_e2549_d_n12;
        var_lc1_dn13 = assign1940_e2549_d_n13;
        var_lc1_dn14 = assign1940_e2549_d_n14;
        var_lc1_dn15 = assign1940_e2549_d_n15;
        var_lc1_dn16 = assign1940_e2549_d_n16;
        var_lc1_dn17 = assign1940_e2549_d_n17;
        var_lc1_dn18 = assign1940_e2549_d_n18;
        var_lc1_db0 = assign1940_e2549_d_b0;
        var_lc1_db1 = assign1940_e2549_d_b1;
        var_lc1_db2 = assign1940_e2549_d_b2;
        var_lc1_db3 = assign1940_e2549_d_b3;
        var_lc1_db4 = assign1940_e2549_d_b4;
        var_lc1_db5 = assign1940_e2549_d_b5;
        var_lc1_db6 = assign1940_e2549_d_b6;
        var_lc1_db7 = assign1940_e2549_d_b7;
        var_lc1_db8 = assign1940_e2549_d_b8;
        var_lc1_db9 = assign1940_e2549_d_b9;
        var_lc1_db10 = assign1940_e2549_d_b10;
        var_lc1_db11 = assign1940_e2549_d_b11;
        var_lc1_db12 = assign1940_e2549_d_b12;
        var_lc1_db13 = assign1940_e2549_d_b13;
        var_lc1_db14 = assign1940_e2549_d_b14;
        var_lc1_db15 = assign1940_e2549_d_b15;
        var_lc1_db16 = assign1940_e2549_d_b16;
        var_lc1_db17 = assign1940_e2549_d_b17;
        var_lc1_db18 = assign1940_e2549_d_b18;
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
        var_lc1_rdn16 = 0.0;
        var_lc1_rdn17 = 0.0;
        var_lc1_rdn18 = 0.0;
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

        let (assign1950_e2562, assign1950_e2562_d_n0, assign1950_e2562_d_n1, assign1950_e2562_d_n2, assign1950_e2562_d_n3, assign1950_e2562_d_n4, assign1950_e2562_d_n5, assign1950_e2562_d_n6, assign1950_e2562_d_n7, assign1950_e2562_d_n8, assign1950_e2562_d_n9, assign1950_e2562_d_n10, assign1950_e2562_d_n11, assign1950_e2562_d_n12, assign1950_e2562_d_n13, assign1950_e2562_d_n14, assign1950_e2562_d_n15, assign1950_e2562_d_n16, assign1950_e2562_d_n17, assign1950_e2562_d_n18, assign1950_e2562_d_b0, assign1950_e2562_d_b1, assign1950_e2562_d_b2, assign1950_e2562_d_b3, assign1950_e2562_d_b4, assign1950_e2562_d_b5, assign1950_e2562_d_b6, assign1950_e2562_d_b7, assign1950_e2562_d_b8, assign1950_e2562_d_b9, assign1950_e2562_d_b10, assign1950_e2562_d_b11, assign1950_e2562_d_b12, assign1950_e2562_d_b13, assign1950_e2562_d_b14, assign1950_e2562_d_b15, assign1950_e2562_d_b16, assign1950_e2562_d_b17, assign1950_e2562_d_b18,) = {
    if ((var_guard18 != 0.0) && (!((((var_guard14 != 0.0) || (var_guard15 != 0.0)) || (var_guard16 != 0.0)) || (var_guard17 != 0.0)))) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_mjc, var_mjc_dn0, var_mjc_dn1, var_mjc_dn2, var_mjc_dn3, var_mjc_dn4, var_mjc_dn5, var_mjc_dn6, var_mjc_dn7, var_mjc_dn8, var_mjc_dn9, var_mjc_dn10, var_mjc_dn11, var_mjc_dn12, var_mjc_dn13, var_mjc_dn14, var_mjc_dn15, var_mjc_dn16, var_mjc_dn17, var_mjc_dn18, var_mjc_db0, var_mjc_db1, var_mjc_db2, var_mjc_db3, var_mjc_db4, var_mjc_db5, var_mjc_db6, var_mjc_db7, var_mjc_db8, var_mjc_db9, var_mjc_db10, var_mjc_db11, var_mjc_db12, var_mjc_db13, var_mjc_db14, var_mjc_db15, var_mjc_db16, var_mjc_db17, var_mjc_db18,)
    }
};
        var_mjc = assign1950_e2562;
        var_mjc_dn0 = assign1950_e2562_d_n0;
        var_mjc_dn1 = assign1950_e2562_d_n1;
        var_mjc_dn2 = assign1950_e2562_d_n2;
        var_mjc_dn3 = assign1950_e2562_d_n3;
        var_mjc_dn4 = assign1950_e2562_d_n4;
        var_mjc_dn5 = assign1950_e2562_d_n5;
        var_mjc_dn6 = assign1950_e2562_d_n6;
        var_mjc_dn7 = assign1950_e2562_d_n7;
        var_mjc_dn8 = assign1950_e2562_d_n8;
        var_mjc_dn9 = assign1950_e2562_d_n9;
        var_mjc_dn10 = assign1950_e2562_d_n10;
        var_mjc_dn11 = assign1950_e2562_d_n11;
        var_mjc_dn12 = assign1950_e2562_d_n12;
        var_mjc_dn13 = assign1950_e2562_d_n13;
        var_mjc_dn14 = assign1950_e2562_d_n14;
        var_mjc_dn15 = assign1950_e2562_d_n15;
        var_mjc_dn16 = assign1950_e2562_d_n16;
        var_mjc_dn17 = assign1950_e2562_d_n17;
        var_mjc_dn18 = assign1950_e2562_d_n18;
        var_mjc_db0 = assign1950_e2562_d_b0;
        var_mjc_db1 = assign1950_e2562_d_b1;
        var_mjc_db2 = assign1950_e2562_d_b2;
        var_mjc_db3 = assign1950_e2562_d_b3;
        var_mjc_db4 = assign1950_e2562_d_b4;
        var_mjc_db5 = assign1950_e2562_d_b5;
        var_mjc_db6 = assign1950_e2562_d_b6;
        var_mjc_db7 = assign1950_e2562_d_b7;
        var_mjc_db8 = assign1950_e2562_d_b8;
        var_mjc_db9 = assign1950_e2562_d_b9;
        var_mjc_db10 = assign1950_e2562_d_b10;
        var_mjc_db11 = assign1950_e2562_d_b11;
        var_mjc_db12 = assign1950_e2562_d_b12;
        var_mjc_db13 = assign1950_e2562_d_b13;
        var_mjc_db14 = assign1950_e2562_d_b14;
        var_mjc_db15 = assign1950_e2562_d_b15;
        var_mjc_db16 = assign1950_e2562_d_b16;
        var_mjc_db17 = assign1950_e2562_d_b17;
        var_mjc_db18 = assign1950_e2562_d_b18;
        var_mjc_rv = 0.0;
        var_mjc_rdn0 = 0.0;
        var_mjc_rdn1 = 0.0;
        var_mjc_rdn2 = 0.0;
        var_mjc_rdn3 = 0.0;
        var_mjc_rdn4 = 0.0;
        var_mjc_rdn5 = 0.0;
        var_mjc_rdn6 = 0.0;
        var_mjc_rdn7 = 0.0;
        var_mjc_rdn8 = 0.0;
        var_mjc_rdn9 = 0.0;
        var_mjc_rdn10 = 0.0;
        var_mjc_rdn11 = 0.0;
        var_mjc_rdn12 = 0.0;
        var_mjc_rdn13 = 0.0;
        var_mjc_rdn14 = 0.0;
        var_mjc_rdn15 = 0.0;
        var_mjc_rdn16 = 0.0;
        var_mjc_rdn17 = 0.0;
        var_mjc_rdn18 = 0.0;
        var_mjc_rdb0 = 0.0;
        var_mjc_rdb1 = 0.0;
        var_mjc_rdb2 = 0.0;
        var_mjc_rdb3 = 0.0;
        var_mjc_rdb4 = 0.0;
        var_mjc_rdb5 = 0.0;
        var_mjc_rdb6 = 0.0;
        var_mjc_rdb7 = 0.0;
        var_mjc_rdb8 = 0.0;
        var_mjc_rdb9 = 0.0;
        var_mjc_rdb10 = 0.0;
        var_mjc_rdb11 = 0.0;
        var_mjc_rdb12 = 0.0;
        var_mjc_rdb13 = 0.0;
        var_mjc_rdb14 = 0.0;
        var_mjc_rdb15 = 0.0;
        var_mjc_rdb16 = 0.0;
        var_mjc_rdb17 = 0.0;
        var_mjc_rdb18 = 0.0;

        let (assign1960_e2593, assign1960_e2593_d_n0, assign1960_e2593_d_n1, assign1960_e2593_d_n2, assign1960_e2593_d_n3, assign1960_e2593_d_n4, assign1960_e2593_d_n5, assign1960_e2593_d_n6, assign1960_e2593_d_n7, assign1960_e2593_d_n8, assign1960_e2593_d_n9, assign1960_e2593_d_n10, assign1960_e2593_d_n11, assign1960_e2593_d_n12, assign1960_e2593_d_n13, assign1960_e2593_d_n14, assign1960_e2593_d_n15, assign1960_e2593_d_n16, assign1960_e2593_d_n17, assign1960_e2593_d_n18, assign1960_e2593_d_b0, assign1960_e2593_d_b1, assign1960_e2593_d_b2, assign1960_e2593_d_b3, assign1960_e2593_d_b4, assign1960_e2593_d_b5, assign1960_e2593_d_b6, assign1960_e2593_d_b7, assign1960_e2593_d_b8, assign1960_e2593_d_b9, assign1960_e2593_d_b10, assign1960_e2593_d_b11, assign1960_e2593_d_b12, assign1960_e2593_d_b13, assign1960_e2593_d_b14, assign1960_e2593_d_b15, assign1960_e2593_d_b16, assign1960_e2593_d_b17, assign1960_e2593_d_b18,) = {
    if ((var_guard18 != 0.0) && (!((((var_guard14 != 0.0) || (var_guard15 != 0.0)) || (var_guard16 != 0.0)) || (var_guard17 != 0.0)))) {
        let assign1960_e2576: f64 = (p.p40 + var_vgsc);
        let assign1960_e2577: f64 = (p.p39 * assign1960_e2576);
        let assign1960_e2580: f64 = (-1.0);
        let assign1960_e2583: f64 = (var_vgsc / p.p40);
        let assign1960_e2584: f64 = (assign1960_e2580 + assign1960_e2583);
        let assign1960_e2586: f64 = (assign1960_e2584).powf(2.0);
        let assign1960_e2587: f64 = (p.p41 + assign1960_e2586);
        let assign1960_e2589: f64 = (-var_mjc);
        let assign1960_e2590: f64 = (assign1960_e2587).powf(assign1960_e2589);
        let assign1960_e2591: f64 = (assign1960_e2577 * assign1960_e2590);
        (assign1960_e2591, (((p.p39 * var_vgsc_dn0) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn0) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn0 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn0 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn0) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn0 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn0 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn1) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn1) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn1 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn1 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn1) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn1 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn1 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn2) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn2) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn2 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn2 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn2) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn2 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn2 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn3) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn3) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn3 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn3 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn3) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn3 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn3 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn4) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn4) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn4 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn4 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn4) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn4 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn4 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn5) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn5) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn5 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn5 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn5) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn5 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn5 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn6) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn6) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn6 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn6 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn6) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn6 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn6 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn7) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn7) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn7 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn7 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn7) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn7 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn7 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn8) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn8) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn8 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn8 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn8) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn8 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn8 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn9) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn9) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn9 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn9 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn9) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn9 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn9 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn10) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn10) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn10 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn10 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn10) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn10 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn10 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn11) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn11) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn11 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn11 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn11) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn11 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn11 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn12) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn12) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn12 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn12 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn12) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn12 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn12 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn13) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn13) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn13 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn13 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn13) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn13 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn13 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn14) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn14) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn14 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn14 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn14) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn14 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn14 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn15) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn15) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn15 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn15 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn15) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn15 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn15 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn16) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn16) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn16 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn16 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn16) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn16 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn16 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn17) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn17) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn17 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn17 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn17) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn17 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn17 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_dn18) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_dn18) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn18 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn18 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_dn18) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_dn18 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_dn18 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db0) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db0) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db0 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db0 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db0) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db0 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db0 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db1) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db1) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db1 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db1 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db1) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db1 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db1 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db2) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db2) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db2 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db2 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db2) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db2 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db2 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db3) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db3) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db3 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db3 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db3) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db3 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db3 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db4) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db4) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db4 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db4 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db4) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db4 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db4 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db5) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db5) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db5 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db5 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db5) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db5 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db5 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db6) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db6) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db6 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db6 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db6) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db6 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db6 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db7) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db7) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db7 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db7 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db7) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db7 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db7 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db8) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db8) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db8 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db8 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db8) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db8 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db8 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db9) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db9) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db9 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db9 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db9) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db9 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db9 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db10) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db10) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db10 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db10 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db10) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db10 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db10 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db11) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db11) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db11 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db11 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db11) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db11 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db11 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db12) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db12) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db12 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db12 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db12) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db12 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db12 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db13) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db13) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db13 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db13 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db13) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db13 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db13 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db14) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db14) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db14 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db14 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db14) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db14 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db14 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db15) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db15) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db15 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db15 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db15) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db15 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db15 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db16) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db16) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db16 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db16 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db16) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db16 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db16 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db17) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db17) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db17 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db17 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db17) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db17 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db17 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })), (((p.p39 * var_vgsc_db18) * assign1960_e2590) + (assign1960_e2577 * if (-var_mjc_db18) == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db18 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db18 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (((-var_mjc_db18) * (assign1960_e2587).ln()) + (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (var_vgsc_db18 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((var_vgsc_db18 / p.p40) / assign1960_e2584))) } / assign1960_e2587)))) })),)
    } else {
        (var_qgsdepl, var_qgsdepl_dn0, var_qgsdepl_dn1, var_qgsdepl_dn2, var_qgsdepl_dn3, var_qgsdepl_dn4, var_qgsdepl_dn5, var_qgsdepl_dn6, var_qgsdepl_dn7, var_qgsdepl_dn8, var_qgsdepl_dn9, var_qgsdepl_dn10, var_qgsdepl_dn11, var_qgsdepl_dn12, var_qgsdepl_dn13, var_qgsdepl_dn14, var_qgsdepl_dn15, var_qgsdepl_dn16, var_qgsdepl_dn17, var_qgsdepl_dn18, var_qgsdepl_db0, var_qgsdepl_db1, var_qgsdepl_db2, var_qgsdepl_db3, var_qgsdepl_db4, var_qgsdepl_db5, var_qgsdepl_db6, var_qgsdepl_db7, var_qgsdepl_db8, var_qgsdepl_db9, var_qgsdepl_db10, var_qgsdepl_db11, var_qgsdepl_db12, var_qgsdepl_db13, var_qgsdepl_db14, var_qgsdepl_db15, var_qgsdepl_db16, var_qgsdepl_db17, var_qgsdepl_db18,)
    }
};
        var_qgsdepl = assign1960_e2593;
        var_qgsdepl_dn0 = assign1960_e2593_d_n0;
        var_qgsdepl_dn1 = assign1960_e2593_d_n1;
        var_qgsdepl_dn2 = assign1960_e2593_d_n2;
        var_qgsdepl_dn3 = assign1960_e2593_d_n3;
        var_qgsdepl_dn4 = assign1960_e2593_d_n4;
        var_qgsdepl_dn5 = assign1960_e2593_d_n5;
        var_qgsdepl_dn6 = assign1960_e2593_d_n6;
        var_qgsdepl_dn7 = assign1960_e2593_d_n7;
        var_qgsdepl_dn8 = assign1960_e2593_d_n8;
        var_qgsdepl_dn9 = assign1960_e2593_d_n9;
        var_qgsdepl_dn10 = assign1960_e2593_d_n10;
        var_qgsdepl_dn11 = assign1960_e2593_d_n11;
        var_qgsdepl_dn12 = assign1960_e2593_d_n12;
        var_qgsdepl_dn13 = assign1960_e2593_d_n13;
        var_qgsdepl_dn14 = assign1960_e2593_d_n14;
        var_qgsdepl_dn15 = assign1960_e2593_d_n15;
        var_qgsdepl_dn16 = assign1960_e2593_d_n16;
        var_qgsdepl_dn17 = assign1960_e2593_d_n17;
        var_qgsdepl_dn18 = assign1960_e2593_d_n18;
        var_qgsdepl_db0 = assign1960_e2593_d_b0;
        var_qgsdepl_db1 = assign1960_e2593_d_b1;
        var_qgsdepl_db2 = assign1960_e2593_d_b2;
        var_qgsdepl_db3 = assign1960_e2593_d_b3;
        var_qgsdepl_db4 = assign1960_e2593_d_b4;
        var_qgsdepl_db5 = assign1960_e2593_d_b5;
        var_qgsdepl_db6 = assign1960_e2593_d_b6;
        var_qgsdepl_db7 = assign1960_e2593_d_b7;
        var_qgsdepl_db8 = assign1960_e2593_d_b8;
        var_qgsdepl_db9 = assign1960_e2593_d_b9;
        var_qgsdepl_db10 = assign1960_e2593_d_b10;
        var_qgsdepl_db11 = assign1960_e2593_d_b11;
        var_qgsdepl_db12 = assign1960_e2593_d_b12;
        var_qgsdepl_db13 = assign1960_e2593_d_b13;
        var_qgsdepl_db14 = assign1960_e2593_d_b14;
        var_qgsdepl_db15 = assign1960_e2593_d_b15;
        var_qgsdepl_db16 = assign1960_e2593_d_b16;
        var_qgsdepl_db17 = assign1960_e2593_d_b17;
        var_qgsdepl_db18 = assign1960_e2593_d_b18;
        var_qgsdepl_rv = 0.0;
        var_qgsdepl_rdn0 = 0.0;
        var_qgsdepl_rdn1 = 0.0;
        var_qgsdepl_rdn2 = 0.0;
        var_qgsdepl_rdn3 = 0.0;
        var_qgsdepl_rdn4 = 0.0;
        var_qgsdepl_rdn5 = 0.0;
        var_qgsdepl_rdn6 = 0.0;
        var_qgsdepl_rdn7 = 0.0;
        var_qgsdepl_rdn8 = 0.0;
        var_qgsdepl_rdn9 = 0.0;
        var_qgsdepl_rdn10 = 0.0;
        var_qgsdepl_rdn11 = 0.0;
        var_qgsdepl_rdn12 = 0.0;
        var_qgsdepl_rdn13 = 0.0;
        var_qgsdepl_rdn14 = 0.0;
        var_qgsdepl_rdn15 = 0.0;
        var_qgsdepl_rdn16 = 0.0;
        var_qgsdepl_rdn17 = 0.0;
        var_qgsdepl_rdn18 = 0.0;
        var_qgsdepl_rdb0 = 0.0;
        var_qgsdepl_rdb1 = 0.0;
        var_qgsdepl_rdb2 = 0.0;
        var_qgsdepl_rdb3 = 0.0;
        var_qgsdepl_rdb4 = 0.0;
        var_qgsdepl_rdb5 = 0.0;
        var_qgsdepl_rdb6 = 0.0;
        var_qgsdepl_rdb7 = 0.0;
        var_qgsdepl_rdb8 = 0.0;
        var_qgsdepl_rdb9 = 0.0;
        var_qgsdepl_rdb10 = 0.0;
        var_qgsdepl_rdb11 = 0.0;
        var_qgsdepl_rdb12 = 0.0;
        var_qgsdepl_rdb13 = 0.0;
        var_qgsdepl_rdb14 = 0.0;
        var_qgsdepl_rdb15 = 0.0;
        var_qgsdepl_rdb16 = 0.0;
        var_qgsdepl_rdb17 = 0.0;
        var_qgsdepl_rdb18 = 0.0;

        let (assign1970_e2615, assign1970_e2615_d_n0, assign1970_e2615_d_n1, assign1970_e2615_d_n2, assign1970_e2615_d_n3, assign1970_e2615_d_n4, assign1970_e2615_d_n5, assign1970_e2615_d_n6, assign1970_e2615_d_n7, assign1970_e2615_d_n8, assign1970_e2615_d_n9, assign1970_e2615_d_n10, assign1970_e2615_d_n11, assign1970_e2615_d_n12, assign1970_e2615_d_n13, assign1970_e2615_d_n14, assign1970_e2615_d_n15, assign1970_e2615_d_n16, assign1970_e2615_d_n17, assign1970_e2615_d_n18, assign1970_e2615_d_b0, assign1970_e2615_d_b1, assign1970_e2615_d_b2, assign1970_e2615_d_b3, assign1970_e2615_d_b4, assign1970_e2615_d_b5, assign1970_e2615_d_b6, assign1970_e2615_d_b7, assign1970_e2615_d_b8, assign1970_e2615_d_b9, assign1970_e2615_d_b10, assign1970_e2615_d_b11, assign1970_e2615_d_b12, assign1970_e2615_d_b13, assign1970_e2615_d_b14, assign1970_e2615_d_b15, assign1970_e2615_d_b16, assign1970_e2615_d_b17, assign1970_e2615_d_b18,) = {
    if ((var_guard18 != 0.0) && (!((((var_guard14 != 0.0) || (var_guard15 != 0.0)) || (var_guard16 != 0.0)) || (var_guard17 != 0.0)))) {
        let assign1970_e2606: f64 = (p.p39 * p.p40);
        let assign1970_e2609: f64 = (p.p41 + 1.0);
        let assign1970_e2611: f64 = (-var_mjc);
        let assign1970_e2612: f64 = (assign1970_e2609).powf(assign1970_e2611);
        let assign1970_e2613: f64 = (assign1970_e2606 * assign1970_e2612);
        (assign1970_e2613, (assign1970_e2606 * if (-var_mjc_dn0) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn0) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn1) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn1) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn2) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn2) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn3) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn3) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn4) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn4) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn5) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn5) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn6) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn6) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn7) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn7) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn8) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn8) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn9) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn9) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn10) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn10) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn11) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn11) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn12) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn12) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn13) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn13) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn14) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn14) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn15) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn15) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn16) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn16) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn17) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn17) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_dn18) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_dn18) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db0) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db0) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db1) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db1) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db2) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db2) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db3) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db3) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db4) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db4) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db5) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db5) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db6) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db6) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db7) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db7) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db8) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db8) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db9) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db9) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db10) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db10) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db11) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db11) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db12) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db12) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db13) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db13) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db14) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db14) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db15) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db15) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db16) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db16) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db17) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db17) * (assign1970_e2609).ln())) }), (assign1970_e2606 * if (-var_mjc_db18) == 0.0 && ((assign1970_e2611) as f64).is_finite() && ((assign1970_e2611) as f64).fract() == 0.0 { 0.0 } else { (assign1970_e2612 * ((-var_mjc_db18) * (assign1970_e2609).ln())) }),)
    } else {
        (var_qgsdepl0, var_qgsdepl0_dn0, var_qgsdepl0_dn1, var_qgsdepl0_dn2, var_qgsdepl0_dn3, var_qgsdepl0_dn4, var_qgsdepl0_dn5, var_qgsdepl0_dn6, var_qgsdepl0_dn7, var_qgsdepl0_dn8, var_qgsdepl0_dn9, var_qgsdepl0_dn10, var_qgsdepl0_dn11, var_qgsdepl0_dn12, var_qgsdepl0_dn13, var_qgsdepl0_dn14, var_qgsdepl0_dn15, var_qgsdepl0_dn16, var_qgsdepl0_dn17, var_qgsdepl0_dn18, var_qgsdepl0_db0, var_qgsdepl0_db1, var_qgsdepl0_db2, var_qgsdepl0_db3, var_qgsdepl0_db4, var_qgsdepl0_db5, var_qgsdepl0_db6, var_qgsdepl0_db7, var_qgsdepl0_db8, var_qgsdepl0_db9, var_qgsdepl0_db10, var_qgsdepl0_db11, var_qgsdepl0_db12, var_qgsdepl0_db13, var_qgsdepl0_db14, var_qgsdepl0_db15, var_qgsdepl0_db16, var_qgsdepl0_db17, var_qgsdepl0_db18,)
    }
};
        var_qgsdepl0 = assign1970_e2615;
        var_qgsdepl0_dn0 = assign1970_e2615_d_n0;
        var_qgsdepl0_dn1 = assign1970_e2615_d_n1;
        var_qgsdepl0_dn2 = assign1970_e2615_d_n2;
        var_qgsdepl0_dn3 = assign1970_e2615_d_n3;
        var_qgsdepl0_dn4 = assign1970_e2615_d_n4;
        var_qgsdepl0_dn5 = assign1970_e2615_d_n5;
        var_qgsdepl0_dn6 = assign1970_e2615_d_n6;
        var_qgsdepl0_dn7 = assign1970_e2615_d_n7;
        var_qgsdepl0_dn8 = assign1970_e2615_d_n8;
        var_qgsdepl0_dn9 = assign1970_e2615_d_n9;
        var_qgsdepl0_dn10 = assign1970_e2615_d_n10;
        var_qgsdepl0_dn11 = assign1970_e2615_d_n11;
        var_qgsdepl0_dn12 = assign1970_e2615_d_n12;
        var_qgsdepl0_dn13 = assign1970_e2615_d_n13;
        var_qgsdepl0_dn14 = assign1970_e2615_d_n14;
        var_qgsdepl0_dn15 = assign1970_e2615_d_n15;
        var_qgsdepl0_dn16 = assign1970_e2615_d_n16;
        var_qgsdepl0_dn17 = assign1970_e2615_d_n17;
        var_qgsdepl0_dn18 = assign1970_e2615_d_n18;
        var_qgsdepl0_db0 = assign1970_e2615_d_b0;
        var_qgsdepl0_db1 = assign1970_e2615_d_b1;
        var_qgsdepl0_db2 = assign1970_e2615_d_b2;
        var_qgsdepl0_db3 = assign1970_e2615_d_b3;
        var_qgsdepl0_db4 = assign1970_e2615_d_b4;
        var_qgsdepl0_db5 = assign1970_e2615_d_b5;
        var_qgsdepl0_db6 = assign1970_e2615_d_b6;
        var_qgsdepl0_db7 = assign1970_e2615_d_b7;
        var_qgsdepl0_db8 = assign1970_e2615_d_b8;
        var_qgsdepl0_db9 = assign1970_e2615_d_b9;
        var_qgsdepl0_db10 = assign1970_e2615_d_b10;
        var_qgsdepl0_db11 = assign1970_e2615_d_b11;
        var_qgsdepl0_db12 = assign1970_e2615_d_b12;
        var_qgsdepl0_db13 = assign1970_e2615_d_b13;
        var_qgsdepl0_db14 = assign1970_e2615_d_b14;
        var_qgsdepl0_db15 = assign1970_e2615_d_b15;
        var_qgsdepl0_db16 = assign1970_e2615_d_b16;
        var_qgsdepl0_db17 = assign1970_e2615_d_b17;
        var_qgsdepl0_db18 = assign1970_e2615_d_b18;
        var_qgsdepl0_rv = 0.0;
        var_qgsdepl0_rdn0 = 0.0;
        var_qgsdepl0_rdn1 = 0.0;
        var_qgsdepl0_rdn2 = 0.0;
        var_qgsdepl0_rdn3 = 0.0;
        var_qgsdepl0_rdn4 = 0.0;
        var_qgsdepl0_rdn5 = 0.0;
        var_qgsdepl0_rdn6 = 0.0;
        var_qgsdepl0_rdn7 = 0.0;
        var_qgsdepl0_rdn8 = 0.0;
        var_qgsdepl0_rdn9 = 0.0;
        var_qgsdepl0_rdn10 = 0.0;
        var_qgsdepl0_rdn11 = 0.0;
        var_qgsdepl0_rdn12 = 0.0;
        var_qgsdepl0_rdn13 = 0.0;
        var_qgsdepl0_rdn14 = 0.0;
        var_qgsdepl0_rdn15 = 0.0;
        var_qgsdepl0_rdn16 = 0.0;
        var_qgsdepl0_rdn17 = 0.0;
        var_qgsdepl0_rdn18 = 0.0;
        var_qgsdepl0_rdb0 = 0.0;
        var_qgsdepl0_rdb1 = 0.0;
        var_qgsdepl0_rdb2 = 0.0;
        var_qgsdepl0_rdb3 = 0.0;
        var_qgsdepl0_rdb4 = 0.0;
        var_qgsdepl0_rdb5 = 0.0;
        var_qgsdepl0_rdb6 = 0.0;
        var_qgsdepl0_rdb7 = 0.0;
        var_qgsdepl0_rdb8 = 0.0;
        var_qgsdepl0_rdb9 = 0.0;
        var_qgsdepl0_rdb10 = 0.0;
        var_qgsdepl0_rdb11 = 0.0;
        var_qgsdepl0_rdb12 = 0.0;
        var_qgsdepl0_rdb13 = 0.0;
        var_qgsdepl0_rdb14 = 0.0;
        var_qgsdepl0_rdb15 = 0.0;
        var_qgsdepl0_rdb16 = 0.0;
        var_qgsdepl0_rdb17 = 0.0;
        var_qgsdepl0_rdb18 = 0.0;

        let (assign1980_e2634, assign1980_e2634_d_n0, assign1980_e2634_d_n1, assign1980_e2634_d_n2, assign1980_e2634_d_n3, assign1980_e2634_d_n4, assign1980_e2634_d_n5, assign1980_e2634_d_n6, assign1980_e2634_d_n7, assign1980_e2634_d_n8, assign1980_e2634_d_n9, assign1980_e2634_d_n10, assign1980_e2634_d_n11, assign1980_e2634_d_n12, assign1980_e2634_d_n13, assign1980_e2634_d_n14, assign1980_e2634_d_n15, assign1980_e2634_d_n16, assign1980_e2634_d_n17, assign1980_e2634_d_n18, assign1980_e2634_d_b0, assign1980_e2634_d_b1, assign1980_e2634_d_b2, assign1980_e2634_d_b3, assign1980_e2634_d_b4, assign1980_e2634_d_b5, assign1980_e2634_d_b6, assign1980_e2634_d_b7, assign1980_e2634_d_b8, assign1980_e2634_d_b9, assign1980_e2634_d_b10, assign1980_e2634_d_b11, assign1980_e2634_d_b12, assign1980_e2634_d_b13, assign1980_e2634_d_b14, assign1980_e2634_d_b15, assign1980_e2634_d_b16, assign1980_e2634_d_b17, assign1980_e2634_d_b18,) = {
    if ((var_guard18 != 0.0) && (!((((var_guard14 != 0.0) || (var_guard15 != 0.0)) || (var_guard16 != 0.0)) || (var_guard17 != 0.0)))) {
        let assign1980_e2629: f64 = (p.p38 * var_vds);
        let assign1980_e2630: f64 = (var_p10_t + assign1980_e2629);
        let assign1980_e2632: f64 = (assign1980_e2630 + var_lc10);
        (assign1980_e2632, ((var_p10_t_dn0 + (p.p38 * var_vds_dn0)) + var_lc10_dn0), ((var_p10_t_dn1 + (p.p38 * var_vds_dn1)) + var_lc10_dn1), ((var_p10_t_dn2 + (p.p38 * var_vds_dn2)) + var_lc10_dn2), ((var_p10_t_dn3 + (p.p38 * var_vds_dn3)) + var_lc10_dn3), ((var_p10_t_dn4 + (p.p38 * var_vds_dn4)) + var_lc10_dn4), ((var_p10_t_dn5 + (p.p38 * var_vds_dn5)) + var_lc10_dn5), ((var_p10_t_dn6 + (p.p38 * var_vds_dn6)) + var_lc10_dn6), ((var_p10_t_dn7 + (p.p38 * var_vds_dn7)) + var_lc10_dn7), ((var_p10_t_dn8 + (p.p38 * var_vds_dn8)) + var_lc10_dn8), ((var_p10_t_dn9 + (p.p38 * var_vds_dn9)) + var_lc10_dn9), ((var_p10_t_dn10 + (p.p38 * var_vds_dn10)) + var_lc10_dn10), ((var_p10_t_dn11 + (p.p38 * var_vds_dn11)) + var_lc10_dn11), ((var_p10_t_dn12 + (p.p38 * var_vds_dn12)) + var_lc10_dn12), ((var_p10_t_dn13 + (p.p38 * var_vds_dn13)) + var_lc10_dn13), ((var_p10_t_dn14 + (p.p38 * var_vds_dn14)) + var_lc10_dn14), ((var_p10_t_dn15 + (p.p38 * var_vds_dn15)) + var_lc10_dn15), ((var_p10_t_dn16 + (p.p38 * var_vds_dn16)) + var_lc10_dn16), ((var_p10_t_dn17 + (p.p38 * var_vds_dn17)) + var_lc10_dn17), ((var_p10_t_dn18 + (p.p38 * var_vds_dn18)) + var_lc10_dn18), ((var_p10_t_db0 + (p.p38 * var_vds_db0)) + var_lc10_db0), ((var_p10_t_db1 + (p.p38 * var_vds_db1)) + var_lc10_db1), ((var_p10_t_db2 + (p.p38 * var_vds_db2)) + var_lc10_db2), ((var_p10_t_db3 + (p.p38 * var_vds_db3)) + var_lc10_db3), ((var_p10_t_db4 + (p.p38 * var_vds_db4)) + var_lc10_db4), ((var_p10_t_db5 + (p.p38 * var_vds_db5)) + var_lc10_db5), ((var_p10_t_db6 + (p.p38 * var_vds_db6)) + var_lc10_db6), ((var_p10_t_db7 + (p.p38 * var_vds_db7)) + var_lc10_db7), ((var_p10_t_db8 + (p.p38 * var_vds_db8)) + var_lc10_db8), ((var_p10_t_db9 + (p.p38 * var_vds_db9)) + var_lc10_db9), ((var_p10_t_db10 + (p.p38 * var_vds_db10)) + var_lc10_db10), ((var_p10_t_db11 + (p.p38 * var_vds_db11)) + var_lc10_db11), ((var_p10_t_db12 + (p.p38 * var_vds_db12)) + var_lc10_db12), ((var_p10_t_db13 + (p.p38 * var_vds_db13)) + var_lc10_db13), ((var_p10_t_db14 + (p.p38 * var_vds_db14)) + var_lc10_db14), ((var_p10_t_db15 + (p.p38 * var_vds_db15)) + var_lc10_db15), ((var_p10_t_db16 + (p.p38 * var_vds_db16)) + var_lc10_db16), ((var_p10_t_db17 + (p.p38 * var_vds_db17)) + var_lc10_db17), ((var_p10_t_db18 + (p.p38 * var_vds_db18)) + var_lc10_db18),)
    } else {
        (var_qgs0, var_qgs0_dn0, var_qgs0_dn1, var_qgs0_dn2, var_qgs0_dn3, var_qgs0_dn4, var_qgs0_dn5, var_qgs0_dn6, var_qgs0_dn7, var_qgs0_dn8, var_qgs0_dn9, var_qgs0_dn10, var_qgs0_dn11, var_qgs0_dn12, var_qgs0_dn13, var_qgs0_dn14, var_qgs0_dn15, var_qgs0_dn16, var_qgs0_dn17, var_qgs0_dn18, var_qgs0_db0, var_qgs0_db1, var_qgs0_db2, var_qgs0_db3, var_qgs0_db4, var_qgs0_db5, var_qgs0_db6, var_qgs0_db7, var_qgs0_db8, var_qgs0_db9, var_qgs0_db10, var_qgs0_db11, var_qgs0_db12, var_qgs0_db13, var_qgs0_db14, var_qgs0_db15, var_qgs0_db16, var_qgs0_db17, var_qgs0_db18,)
    }
};
        var_qgs0 = assign1980_e2634;
        var_qgs0_dn0 = assign1980_e2634_d_n0;
        var_qgs0_dn1 = assign1980_e2634_d_n1;
        var_qgs0_dn2 = assign1980_e2634_d_n2;
        var_qgs0_dn3 = assign1980_e2634_d_n3;
        var_qgs0_dn4 = assign1980_e2634_d_n4;
        var_qgs0_dn5 = assign1980_e2634_d_n5;
        var_qgs0_dn6 = assign1980_e2634_d_n6;
        var_qgs0_dn7 = assign1980_e2634_d_n7;
        var_qgs0_dn8 = assign1980_e2634_d_n8;
        var_qgs0_dn9 = assign1980_e2634_d_n9;
        var_qgs0_dn10 = assign1980_e2634_d_n10;
        var_qgs0_dn11 = assign1980_e2634_d_n11;
        var_qgs0_dn12 = assign1980_e2634_d_n12;
        var_qgs0_dn13 = assign1980_e2634_d_n13;
        var_qgs0_dn14 = assign1980_e2634_d_n14;
        var_qgs0_dn15 = assign1980_e2634_d_n15;
        var_qgs0_dn16 = assign1980_e2634_d_n16;
        var_qgs0_dn17 = assign1980_e2634_d_n17;
        var_qgs0_dn18 = assign1980_e2634_d_n18;
        var_qgs0_db0 = assign1980_e2634_d_b0;
        var_qgs0_db1 = assign1980_e2634_d_b1;
        var_qgs0_db2 = assign1980_e2634_d_b2;
        var_qgs0_db3 = assign1980_e2634_d_b3;
        var_qgs0_db4 = assign1980_e2634_d_b4;
        var_qgs0_db5 = assign1980_e2634_d_b5;
        var_qgs0_db6 = assign1980_e2634_d_b6;
        var_qgs0_db7 = assign1980_e2634_d_b7;
        var_qgs0_db8 = assign1980_e2634_d_b8;
        var_qgs0_db9 = assign1980_e2634_d_b9;
        var_qgs0_db10 = assign1980_e2634_d_b10;
        var_qgs0_db11 = assign1980_e2634_d_b11;
        var_qgs0_db12 = assign1980_e2634_d_b12;
        var_qgs0_db13 = assign1980_e2634_d_b13;
        var_qgs0_db14 = assign1980_e2634_d_b14;
        var_qgs0_db15 = assign1980_e2634_d_b15;
        var_qgs0_db16 = assign1980_e2634_d_b16;
        var_qgs0_db17 = assign1980_e2634_d_b17;
        var_qgs0_db18 = assign1980_e2634_d_b18;
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
        var_qgs0_rdn16 = 0.0;
        var_qgs0_rdn17 = 0.0;
        var_qgs0_rdn18 = 0.0;
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
        *var_lc1_dn16_slot = var_lc1_dn16;
        *var_lc1_dn17_slot = var_lc1_dn17;
        *var_lc1_dn18_slot = var_lc1_dn18;
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
        *var_lc1_rdn16_slot = var_lc1_rdn16;
        *var_lc1_rdn17_slot = var_lc1_rdn17;
        *var_lc1_rdn18_slot = var_lc1_rdn18;
        *var_lc1_rdn2_slot = var_lc1_rdn2;
        *var_lc1_rdn3_slot = var_lc1_rdn3;
        *var_lc1_rdn4_slot = var_lc1_rdn4;
        *var_lc1_rdn5_slot = var_lc1_rdn5;
        *var_lc1_rdn6_slot = var_lc1_rdn6;
        *var_lc1_rdn7_slot = var_lc1_rdn7;
        *var_lc1_rdn8_slot = var_lc1_rdn8;
        *var_lc1_rdn9_slot = var_lc1_rdn9;
        *var_lc1_rv_slot = var_lc1_rv;
        *var_mjc_slot = var_mjc;
        *var_mjc_db0_slot = var_mjc_db0;
        *var_mjc_db1_slot = var_mjc_db1;
        *var_mjc_db10_slot = var_mjc_db10;
        *var_mjc_db11_slot = var_mjc_db11;
        *var_mjc_db12_slot = var_mjc_db12;
        *var_mjc_db13_slot = var_mjc_db13;
        *var_mjc_db14_slot = var_mjc_db14;
        *var_mjc_db15_slot = var_mjc_db15;
        *var_mjc_db16_slot = var_mjc_db16;
        *var_mjc_db17_slot = var_mjc_db17;
        *var_mjc_db18_slot = var_mjc_db18;
        *var_mjc_db2_slot = var_mjc_db2;
        *var_mjc_db3_slot = var_mjc_db3;
        *var_mjc_db4_slot = var_mjc_db4;
        *var_mjc_db5_slot = var_mjc_db5;
        *var_mjc_db6_slot = var_mjc_db6;
        *var_mjc_db7_slot = var_mjc_db7;
        *var_mjc_db8_slot = var_mjc_db8;
        *var_mjc_db9_slot = var_mjc_db9;
        *var_mjc_dn0_slot = var_mjc_dn0;
        *var_mjc_dn1_slot = var_mjc_dn1;
        *var_mjc_dn10_slot = var_mjc_dn10;
        *var_mjc_dn11_slot = var_mjc_dn11;
        *var_mjc_dn12_slot = var_mjc_dn12;
        *var_mjc_dn13_slot = var_mjc_dn13;
        *var_mjc_dn14_slot = var_mjc_dn14;
        *var_mjc_dn15_slot = var_mjc_dn15;
        *var_mjc_dn16_slot = var_mjc_dn16;
        *var_mjc_dn17_slot = var_mjc_dn17;
        *var_mjc_dn18_slot = var_mjc_dn18;
        *var_mjc_dn2_slot = var_mjc_dn2;
        *var_mjc_dn3_slot = var_mjc_dn3;
        *var_mjc_dn4_slot = var_mjc_dn4;
        *var_mjc_dn5_slot = var_mjc_dn5;
        *var_mjc_dn6_slot = var_mjc_dn6;
        *var_mjc_dn7_slot = var_mjc_dn7;
        *var_mjc_dn8_slot = var_mjc_dn8;
        *var_mjc_dn9_slot = var_mjc_dn9;
        *var_mjc_rdb0_slot = var_mjc_rdb0;
        *var_mjc_rdb1_slot = var_mjc_rdb1;
        *var_mjc_rdb10_slot = var_mjc_rdb10;
        *var_mjc_rdb11_slot = var_mjc_rdb11;
        *var_mjc_rdb12_slot = var_mjc_rdb12;
        *var_mjc_rdb13_slot = var_mjc_rdb13;
        *var_mjc_rdb14_slot = var_mjc_rdb14;
        *var_mjc_rdb15_slot = var_mjc_rdb15;
        *var_mjc_rdb16_slot = var_mjc_rdb16;
        *var_mjc_rdb17_slot = var_mjc_rdb17;
        *var_mjc_rdb18_slot = var_mjc_rdb18;
        *var_mjc_rdb2_slot = var_mjc_rdb2;
        *var_mjc_rdb3_slot = var_mjc_rdb3;
        *var_mjc_rdb4_slot = var_mjc_rdb4;
        *var_mjc_rdb5_slot = var_mjc_rdb5;
        *var_mjc_rdb6_slot = var_mjc_rdb6;
        *var_mjc_rdb7_slot = var_mjc_rdb7;
        *var_mjc_rdb8_slot = var_mjc_rdb8;
        *var_mjc_rdb9_slot = var_mjc_rdb9;
        *var_mjc_rdn0_slot = var_mjc_rdn0;
        *var_mjc_rdn1_slot = var_mjc_rdn1;
        *var_mjc_rdn10_slot = var_mjc_rdn10;
        *var_mjc_rdn11_slot = var_mjc_rdn11;
        *var_mjc_rdn12_slot = var_mjc_rdn12;
        *var_mjc_rdn13_slot = var_mjc_rdn13;
        *var_mjc_rdn14_slot = var_mjc_rdn14;
        *var_mjc_rdn15_slot = var_mjc_rdn15;
        *var_mjc_rdn16_slot = var_mjc_rdn16;
        *var_mjc_rdn17_slot = var_mjc_rdn17;
        *var_mjc_rdn18_slot = var_mjc_rdn18;
        *var_mjc_rdn2_slot = var_mjc_rdn2;
        *var_mjc_rdn3_slot = var_mjc_rdn3;
        *var_mjc_rdn4_slot = var_mjc_rdn4;
        *var_mjc_rdn5_slot = var_mjc_rdn5;
        *var_mjc_rdn6_slot = var_mjc_rdn6;
        *var_mjc_rdn7_slot = var_mjc_rdn7;
        *var_mjc_rdn8_slot = var_mjc_rdn8;
        *var_mjc_rdn9_slot = var_mjc_rdn9;
        *var_mjc_rv_slot = var_mjc_rv;
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
        *var_qgs0_dn16_slot = var_qgs0_dn16;
        *var_qgs0_dn17_slot = var_qgs0_dn17;
        *var_qgs0_dn18_slot = var_qgs0_dn18;
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
        *var_qgs0_rdn16_slot = var_qgs0_rdn16;
        *var_qgs0_rdn17_slot = var_qgs0_rdn17;
        *var_qgs0_rdn18_slot = var_qgs0_rdn18;
        *var_qgs0_rdn2_slot = var_qgs0_rdn2;
        *var_qgs0_rdn3_slot = var_qgs0_rdn3;
        *var_qgs0_rdn4_slot = var_qgs0_rdn4;
        *var_qgs0_rdn5_slot = var_qgs0_rdn5;
        *var_qgs0_rdn6_slot = var_qgs0_rdn6;
        *var_qgs0_rdn7_slot = var_qgs0_rdn7;
        *var_qgs0_rdn8_slot = var_qgs0_rdn8;
        *var_qgs0_rdn9_slot = var_qgs0_rdn9;
        *var_qgs0_rv_slot = var_qgs0_rv;
        *var_qgsdepl_slot = var_qgsdepl;
        *var_qgsdepl0_slot = var_qgsdepl0;
        *var_qgsdepl0_db0_slot = var_qgsdepl0_db0;
        *var_qgsdepl0_db1_slot = var_qgsdepl0_db1;
        *var_qgsdepl0_db10_slot = var_qgsdepl0_db10;
        *var_qgsdepl0_db11_slot = var_qgsdepl0_db11;
        *var_qgsdepl0_db12_slot = var_qgsdepl0_db12;
        *var_qgsdepl0_db13_slot = var_qgsdepl0_db13;
        *var_qgsdepl0_db14_slot = var_qgsdepl0_db14;
        *var_qgsdepl0_db15_slot = var_qgsdepl0_db15;
        *var_qgsdepl0_db16_slot = var_qgsdepl0_db16;
        *var_qgsdepl0_db17_slot = var_qgsdepl0_db17;
        *var_qgsdepl0_db18_slot = var_qgsdepl0_db18;
        *var_qgsdepl0_db2_slot = var_qgsdepl0_db2;
        *var_qgsdepl0_db3_slot = var_qgsdepl0_db3;
        *var_qgsdepl0_db4_slot = var_qgsdepl0_db4;
        *var_qgsdepl0_db5_slot = var_qgsdepl0_db5;
        *var_qgsdepl0_db6_slot = var_qgsdepl0_db6;
        *var_qgsdepl0_db7_slot = var_qgsdepl0_db7;
        *var_qgsdepl0_db8_slot = var_qgsdepl0_db8;
        *var_qgsdepl0_db9_slot = var_qgsdepl0_db9;
        *var_qgsdepl0_dn0_slot = var_qgsdepl0_dn0;
        *var_qgsdepl0_dn1_slot = var_qgsdepl0_dn1;
        *var_qgsdepl0_dn10_slot = var_qgsdepl0_dn10;
        *var_qgsdepl0_dn11_slot = var_qgsdepl0_dn11;
        *var_qgsdepl0_dn12_slot = var_qgsdepl0_dn12;
        *var_qgsdepl0_dn13_slot = var_qgsdepl0_dn13;
        *var_qgsdepl0_dn14_slot = var_qgsdepl0_dn14;
        *var_qgsdepl0_dn15_slot = var_qgsdepl0_dn15;
        *var_qgsdepl0_dn16_slot = var_qgsdepl0_dn16;
        *var_qgsdepl0_dn17_slot = var_qgsdepl0_dn17;
        *var_qgsdepl0_dn18_slot = var_qgsdepl0_dn18;
        *var_qgsdepl0_dn2_slot = var_qgsdepl0_dn2;
        *var_qgsdepl0_dn3_slot = var_qgsdepl0_dn3;
        *var_qgsdepl0_dn4_slot = var_qgsdepl0_dn4;
        *var_qgsdepl0_dn5_slot = var_qgsdepl0_dn5;
        *var_qgsdepl0_dn6_slot = var_qgsdepl0_dn6;
        *var_qgsdepl0_dn7_slot = var_qgsdepl0_dn7;
        *var_qgsdepl0_dn8_slot = var_qgsdepl0_dn8;
        *var_qgsdepl0_dn9_slot = var_qgsdepl0_dn9;
        *var_qgsdepl0_rdb0_slot = var_qgsdepl0_rdb0;
        *var_qgsdepl0_rdb1_slot = var_qgsdepl0_rdb1;
        *var_qgsdepl0_rdb10_slot = var_qgsdepl0_rdb10;
        *var_qgsdepl0_rdb11_slot = var_qgsdepl0_rdb11;
        *var_qgsdepl0_rdb12_slot = var_qgsdepl0_rdb12;
        *var_qgsdepl0_rdb13_slot = var_qgsdepl0_rdb13;
        *var_qgsdepl0_rdb14_slot = var_qgsdepl0_rdb14;
        *var_qgsdepl0_rdb15_slot = var_qgsdepl0_rdb15;
        *var_qgsdepl0_rdb16_slot = var_qgsdepl0_rdb16;
        *var_qgsdepl0_rdb17_slot = var_qgsdepl0_rdb17;
        *var_qgsdepl0_rdb18_slot = var_qgsdepl0_rdb18;
        *var_qgsdepl0_rdb2_slot = var_qgsdepl0_rdb2;
        *var_qgsdepl0_rdb3_slot = var_qgsdepl0_rdb3;
        *var_qgsdepl0_rdb4_slot = var_qgsdepl0_rdb4;
        *var_qgsdepl0_rdb5_slot = var_qgsdepl0_rdb5;
        *var_qgsdepl0_rdb6_slot = var_qgsdepl0_rdb6;
        *var_qgsdepl0_rdb7_slot = var_qgsdepl0_rdb7;
        *var_qgsdepl0_rdb8_slot = var_qgsdepl0_rdb8;
        *var_qgsdepl0_rdb9_slot = var_qgsdepl0_rdb9;
        *var_qgsdepl0_rdn0_slot = var_qgsdepl0_rdn0;
        *var_qgsdepl0_rdn1_slot = var_qgsdepl0_rdn1;
        *var_qgsdepl0_rdn10_slot = var_qgsdepl0_rdn10;
        *var_qgsdepl0_rdn11_slot = var_qgsdepl0_rdn11;
        *var_qgsdepl0_rdn12_slot = var_qgsdepl0_rdn12;
        *var_qgsdepl0_rdn13_slot = var_qgsdepl0_rdn13;
        *var_qgsdepl0_rdn14_slot = var_qgsdepl0_rdn14;
        *var_qgsdepl0_rdn15_slot = var_qgsdepl0_rdn15;
        *var_qgsdepl0_rdn16_slot = var_qgsdepl0_rdn16;
        *var_qgsdepl0_rdn17_slot = var_qgsdepl0_rdn17;
        *var_qgsdepl0_rdn18_slot = var_qgsdepl0_rdn18;
        *var_qgsdepl0_rdn2_slot = var_qgsdepl0_rdn2;
        *var_qgsdepl0_rdn3_slot = var_qgsdepl0_rdn3;
        *var_qgsdepl0_rdn4_slot = var_qgsdepl0_rdn4;
        *var_qgsdepl0_rdn5_slot = var_qgsdepl0_rdn5;
        *var_qgsdepl0_rdn6_slot = var_qgsdepl0_rdn6;
        *var_qgsdepl0_rdn7_slot = var_qgsdepl0_rdn7;
        *var_qgsdepl0_rdn8_slot = var_qgsdepl0_rdn8;
        *var_qgsdepl0_rdn9_slot = var_qgsdepl0_rdn9;
        *var_qgsdepl0_rv_slot = var_qgsdepl0_rv;
        *var_qgsdepl_db0_slot = var_qgsdepl_db0;
        *var_qgsdepl_db1_slot = var_qgsdepl_db1;
        *var_qgsdepl_db10_slot = var_qgsdepl_db10;
        *var_qgsdepl_db11_slot = var_qgsdepl_db11;
        *var_qgsdepl_db12_slot = var_qgsdepl_db12;
        *var_qgsdepl_db13_slot = var_qgsdepl_db13;
        *var_qgsdepl_db14_slot = var_qgsdepl_db14;
        *var_qgsdepl_db15_slot = var_qgsdepl_db15;
        *var_qgsdepl_db16_slot = var_qgsdepl_db16;
        *var_qgsdepl_db17_slot = var_qgsdepl_db17;
        *var_qgsdepl_db18_slot = var_qgsdepl_db18;
        *var_qgsdepl_db2_slot = var_qgsdepl_db2;
        *var_qgsdepl_db3_slot = var_qgsdepl_db3;
        *var_qgsdepl_db4_slot = var_qgsdepl_db4;
        *var_qgsdepl_db5_slot = var_qgsdepl_db5;
        *var_qgsdepl_db6_slot = var_qgsdepl_db6;
        *var_qgsdepl_db7_slot = var_qgsdepl_db7;
        *var_qgsdepl_db8_slot = var_qgsdepl_db8;
        *var_qgsdepl_db9_slot = var_qgsdepl_db9;
        *var_qgsdepl_dn0_slot = var_qgsdepl_dn0;
        *var_qgsdepl_dn1_slot = var_qgsdepl_dn1;
        *var_qgsdepl_dn10_slot = var_qgsdepl_dn10;
        *var_qgsdepl_dn11_slot = var_qgsdepl_dn11;
        *var_qgsdepl_dn12_slot = var_qgsdepl_dn12;
        *var_qgsdepl_dn13_slot = var_qgsdepl_dn13;
        *var_qgsdepl_dn14_slot = var_qgsdepl_dn14;
        *var_qgsdepl_dn15_slot = var_qgsdepl_dn15;
        *var_qgsdepl_dn16_slot = var_qgsdepl_dn16;
        *var_qgsdepl_dn17_slot = var_qgsdepl_dn17;
        *var_qgsdepl_dn18_slot = var_qgsdepl_dn18;
        *var_qgsdepl_dn2_slot = var_qgsdepl_dn2;
        *var_qgsdepl_dn3_slot = var_qgsdepl_dn3;
        *var_qgsdepl_dn4_slot = var_qgsdepl_dn4;
        *var_qgsdepl_dn5_slot = var_qgsdepl_dn5;
        *var_qgsdepl_dn6_slot = var_qgsdepl_dn6;
        *var_qgsdepl_dn7_slot = var_qgsdepl_dn7;
        *var_qgsdepl_dn8_slot = var_qgsdepl_dn8;
        *var_qgsdepl_dn9_slot = var_qgsdepl_dn9;
        *var_qgsdepl_rdb0_slot = var_qgsdepl_rdb0;
        *var_qgsdepl_rdb1_slot = var_qgsdepl_rdb1;
        *var_qgsdepl_rdb10_slot = var_qgsdepl_rdb10;
        *var_qgsdepl_rdb11_slot = var_qgsdepl_rdb11;
        *var_qgsdepl_rdb12_slot = var_qgsdepl_rdb12;
        *var_qgsdepl_rdb13_slot = var_qgsdepl_rdb13;
        *var_qgsdepl_rdb14_slot = var_qgsdepl_rdb14;
        *var_qgsdepl_rdb15_slot = var_qgsdepl_rdb15;
        *var_qgsdepl_rdb16_slot = var_qgsdepl_rdb16;
        *var_qgsdepl_rdb17_slot = var_qgsdepl_rdb17;
        *var_qgsdepl_rdb18_slot = var_qgsdepl_rdb18;
        *var_qgsdepl_rdb2_slot = var_qgsdepl_rdb2;
        *var_qgsdepl_rdb3_slot = var_qgsdepl_rdb3;
        *var_qgsdepl_rdb4_slot = var_qgsdepl_rdb4;
        *var_qgsdepl_rdb5_slot = var_qgsdepl_rdb5;
        *var_qgsdepl_rdb6_slot = var_qgsdepl_rdb6;
        *var_qgsdepl_rdb7_slot = var_qgsdepl_rdb7;
        *var_qgsdepl_rdb8_slot = var_qgsdepl_rdb8;
        *var_qgsdepl_rdb9_slot = var_qgsdepl_rdb9;
        *var_qgsdepl_rdn0_slot = var_qgsdepl_rdn0;
        *var_qgsdepl_rdn1_slot = var_qgsdepl_rdn1;
        *var_qgsdepl_rdn10_slot = var_qgsdepl_rdn10;
        *var_qgsdepl_rdn11_slot = var_qgsdepl_rdn11;
        *var_qgsdepl_rdn12_slot = var_qgsdepl_rdn12;
        *var_qgsdepl_rdn13_slot = var_qgsdepl_rdn13;
        *var_qgsdepl_rdn14_slot = var_qgsdepl_rdn14;
        *var_qgsdepl_rdn15_slot = var_qgsdepl_rdn15;
        *var_qgsdepl_rdn16_slot = var_qgsdepl_rdn16;
        *var_qgsdepl_rdn17_slot = var_qgsdepl_rdn17;
        *var_qgsdepl_rdn18_slot = var_qgsdepl_rdn18;
        *var_qgsdepl_rdn2_slot = var_qgsdepl_rdn2;
        *var_qgsdepl_rdn3_slot = var_qgsdepl_rdn3;
        *var_qgsdepl_rdn4_slot = var_qgsdepl_rdn4;
        *var_qgsdepl_rdn5_slot = var_qgsdepl_rdn5;
        *var_qgsdepl_rdn6_slot = var_qgsdepl_rdn6;
        *var_qgsdepl_rdn7_slot = var_qgsdepl_rdn7;
        *var_qgsdepl_rdn8_slot = var_qgsdepl_rdn8;
        *var_qgsdepl_rdn9_slot = var_qgsdepl_rdn9;
        *var_qgsdepl_rv_slot = var_qgsdepl_rv;
    }

    pub(super) fn stamp_reactive_block_36(
        p: &Parameters,
        var_cgs0_t: f64,
        var_cgs0_t_db0: f64,
        var_cgs0_t_db1: f64,
        var_cgs0_t_db10: f64,
        var_cgs0_t_db11: f64,
        var_cgs0_t_db12: f64,
        var_cgs0_t_db13: f64,
        var_cgs0_t_db14: f64,
        var_cgs0_t_db15: f64,
        var_cgs0_t_db16: f64,
        var_cgs0_t_db17: f64,
        var_cgs0_t_db18: f64,
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
        var_cgs0_t_dn16: f64,
        var_cgs0_t_dn17: f64,
        var_cgs0_t_dn18: f64,
        var_cgs0_t_dn2: f64,
        var_cgs0_t_dn3: f64,
        var_cgs0_t_dn4: f64,
        var_cgs0_t_dn5: f64,
        var_cgs0_t_dn6: f64,
        var_cgs0_t_dn7: f64,
        var_cgs0_t_dn8: f64,
        var_cgs0_t_dn9: f64,
        var_guard14: f64,
        var_guard15: f64,
        var_guard16: f64,
        var_guard17: f64,
        var_guard18: f64,
        var_lc1: f64,
        var_lc1_db0: f64,
        var_lc1_db1: f64,
        var_lc1_db10: f64,
        var_lc1_db11: f64,
        var_lc1_db12: f64,
        var_lc1_db13: f64,
        var_lc1_db14: f64,
        var_lc1_db15: f64,
        var_lc1_db16: f64,
        var_lc1_db17: f64,
        var_lc1_db18: f64,
        var_lc1_db2: f64,
        var_lc1_db3: f64,
        var_lc1_db4: f64,
        var_lc1_db5: f64,
        var_lc1_db6: f64,
        var_lc1_db7: f64,
        var_lc1_db8: f64,
        var_lc1_db9: f64,
        var_lc1_dn0: f64,
        var_lc1_dn1: f64,
        var_lc1_dn10: f64,
        var_lc1_dn11: f64,
        var_lc1_dn12: f64,
        var_lc1_dn13: f64,
        var_lc1_dn14: f64,
        var_lc1_dn15: f64,
        var_lc1_dn16: f64,
        var_lc1_dn17: f64,
        var_lc1_dn18: f64,
        var_lc1_dn2: f64,
        var_lc1_dn3: f64,
        var_lc1_dn4: f64,
        var_lc1_dn5: f64,
        var_lc1_dn6: f64,
        var_lc1_dn7: f64,
        var_lc1_dn8: f64,
        var_lc1_dn9: f64,
        var_p40_t: f64,
        var_p40_t_db0: f64,
        var_p40_t_db1: f64,
        var_p40_t_db10: f64,
        var_p40_t_db11: f64,
        var_p40_t_db12: f64,
        var_p40_t_db13: f64,
        var_p40_t_db14: f64,
        var_p40_t_db15: f64,
        var_p40_t_db16: f64,
        var_p40_t_db17: f64,
        var_p40_t_db18: f64,
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
        var_p40_t_dn16: f64,
        var_p40_t_dn17: f64,
        var_p40_t_dn18: f64,
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
        var_psi_1_db15: f64,
        var_psi_1_db16: f64,
        var_psi_1_db17: f64,
        var_psi_1_db18: f64,
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
        var_psi_1_dn16: f64,
        var_psi_1_dn17: f64,
        var_psi_1_dn18: f64,
        var_psi_1_dn2: f64,
        var_psi_1_dn3: f64,
        var_psi_1_dn4: f64,
        var_psi_1_dn5: f64,
        var_psi_1_dn6: f64,
        var_psi_1_dn7: f64,
        var_psi_1_dn8: f64,
        var_psi_1_dn9: f64,
        var_psi_2: f64,
        var_psi_2_db0: f64,
        var_psi_2_db1: f64,
        var_psi_2_db10: f64,
        var_psi_2_db11: f64,
        var_psi_2_db12: f64,
        var_psi_2_db13: f64,
        var_psi_2_db14: f64,
        var_psi_2_db15: f64,
        var_psi_2_db16: f64,
        var_psi_2_db17: f64,
        var_psi_2_db18: f64,
        var_psi_2_db2: f64,
        var_psi_2_db3: f64,
        var_psi_2_db4: f64,
        var_psi_2_db5: f64,
        var_psi_2_db6: f64,
        var_psi_2_db7: f64,
        var_psi_2_db8: f64,
        var_psi_2_db9: f64,
        var_psi_2_dn0: f64,
        var_psi_2_dn1: f64,
        var_psi_2_dn10: f64,
        var_psi_2_dn11: f64,
        var_psi_2_dn12: f64,
        var_psi_2_dn13: f64,
        var_psi_2_dn14: f64,
        var_psi_2_dn15: f64,
        var_psi_2_dn16: f64,
        var_psi_2_dn17: f64,
        var_psi_2_dn18: f64,
        var_psi_2_dn2: f64,
        var_psi_2_dn3: f64,
        var_psi_2_dn4: f64,
        var_psi_2_dn5: f64,
        var_psi_2_dn6: f64,
        var_psi_2_dn7: f64,
        var_psi_2_dn8: f64,
        var_psi_2_dn9: f64,
        var_psi_4: f64,
        var_psi_4_db0: f64,
        var_psi_4_db1: f64,
        var_psi_4_db10: f64,
        var_psi_4_db11: f64,
        var_psi_4_db12: f64,
        var_psi_4_db13: f64,
        var_psi_4_db14: f64,
        var_psi_4_db15: f64,
        var_psi_4_db16: f64,
        var_psi_4_db17: f64,
        var_psi_4_db18: f64,
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
        var_psi_4_dn16: f64,
        var_psi_4_dn17: f64,
        var_psi_4_dn18: f64,
        var_psi_4_dn2: f64,
        var_psi_4_dn3: f64,
        var_psi_4_dn4: f64,
        var_psi_4_dn5: f64,
        var_psi_4_dn6: f64,
        var_psi_4_dn7: f64,
        var_psi_4_dn8: f64,
        var_psi_4_dn9: f64,
        var_qgs0: f64,
        var_qgs0_db0: f64,
        var_qgs0_db1: f64,
        var_qgs0_db10: f64,
        var_qgs0_db11: f64,
        var_qgs0_db12: f64,
        var_qgs0_db13: f64,
        var_qgs0_db14: f64,
        var_qgs0_db15: f64,
        var_qgs0_db16: f64,
        var_qgs0_db17: f64,
        var_qgs0_db18: f64,
        var_qgs0_db2: f64,
        var_qgs0_db3: f64,
        var_qgs0_db4: f64,
        var_qgs0_db5: f64,
        var_qgs0_db6: f64,
        var_qgs0_db7: f64,
        var_qgs0_db8: f64,
        var_qgs0_db9: f64,
        var_qgs0_dn0: f64,
        var_qgs0_dn1: f64,
        var_qgs0_dn10: f64,
        var_qgs0_dn11: f64,
        var_qgs0_dn12: f64,
        var_qgs0_dn13: f64,
        var_qgs0_dn14: f64,
        var_qgs0_dn15: f64,
        var_qgs0_dn16: f64,
        var_qgs0_dn17: f64,
        var_qgs0_dn18: f64,
        var_qgs0_dn2: f64,
        var_qgs0_dn3: f64,
        var_qgs0_dn4: f64,
        var_qgs0_dn5: f64,
        var_qgs0_dn6: f64,
        var_qgs0_dn7: f64,
        var_qgs0_dn8: f64,
        var_qgs0_dn9: f64,
        var_qgsdepl: f64,
        var_qgsdepl0: f64,
        var_qgsdepl0_db0: f64,
        var_qgsdepl0_db1: f64,
        var_qgsdepl0_db10: f64,
        var_qgsdepl0_db11: f64,
        var_qgsdepl0_db12: f64,
        var_qgsdepl0_db13: f64,
        var_qgsdepl0_db14: f64,
        var_qgsdepl0_db15: f64,
        var_qgsdepl0_db16: f64,
        var_qgsdepl0_db17: f64,
        var_qgsdepl0_db18: f64,
        var_qgsdepl0_db2: f64,
        var_qgsdepl0_db3: f64,
        var_qgsdepl0_db4: f64,
        var_qgsdepl0_db5: f64,
        var_qgsdepl0_db6: f64,
        var_qgsdepl0_db7: f64,
        var_qgsdepl0_db8: f64,
        var_qgsdepl0_db9: f64,
        var_qgsdepl0_dn0: f64,
        var_qgsdepl0_dn1: f64,
        var_qgsdepl0_dn10: f64,
        var_qgsdepl0_dn11: f64,
        var_qgsdepl0_dn12: f64,
        var_qgsdepl0_dn13: f64,
        var_qgsdepl0_dn14: f64,
        var_qgsdepl0_dn15: f64,
        var_qgsdepl0_dn16: f64,
        var_qgsdepl0_dn17: f64,
        var_qgsdepl0_dn18: f64,
        var_qgsdepl0_dn2: f64,
        var_qgsdepl0_dn3: f64,
        var_qgsdepl0_dn4: f64,
        var_qgsdepl0_dn5: f64,
        var_qgsdepl0_dn6: f64,
        var_qgsdepl0_dn7: f64,
        var_qgsdepl0_dn8: f64,
        var_qgsdepl0_dn9: f64,
        var_qgsdepl_db0: f64,
        var_qgsdepl_db1: f64,
        var_qgsdepl_db10: f64,
        var_qgsdepl_db11: f64,
        var_qgsdepl_db12: f64,
        var_qgsdepl_db13: f64,
        var_qgsdepl_db14: f64,
        var_qgsdepl_db15: f64,
        var_qgsdepl_db16: f64,
        var_qgsdepl_db17: f64,
        var_qgsdepl_db18: f64,
        var_qgsdepl_db2: f64,
        var_qgsdepl_db3: f64,
        var_qgsdepl_db4: f64,
        var_qgsdepl_db5: f64,
        var_qgsdepl_db6: f64,
        var_qgsdepl_db7: f64,
        var_qgsdepl_db8: f64,
        var_qgsdepl_db9: f64,
        var_qgsdepl_dn0: f64,
        var_qgsdepl_dn1: f64,
        var_qgsdepl_dn10: f64,
        var_qgsdepl_dn11: f64,
        var_qgsdepl_dn12: f64,
        var_qgsdepl_dn13: f64,
        var_qgsdepl_dn14: f64,
        var_qgsdepl_dn15: f64,
        var_qgsdepl_dn16: f64,
        var_qgsdepl_dn17: f64,
        var_qgsdepl_dn18: f64,
        var_qgsdepl_dn2: f64,
        var_qgsdepl_dn3: f64,
        var_qgsdepl_dn4: f64,
        var_qgsdepl_dn5: f64,
        var_qgsdepl_dn6: f64,
        var_qgsdepl_dn7: f64,
        var_qgsdepl_dn8: f64,
        var_qgsdepl_dn9: f64,
        var_vds: f64,
        var_vds_db0: f64,
        var_vds_db1: f64,
        var_vds_db10: f64,
        var_vds_db11: f64,
        var_vds_db12: f64,
        var_vds_db13: f64,
        var_vds_db14: f64,
        var_vds_db15: f64,
        var_vds_db16: f64,
        var_vds_db17: f64,
        var_vds_db18: f64,
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
        var_vds_dn16: f64,
        var_vds_dn17: f64,
        var_vds_dn18: f64,
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
        var_vgsc_dn16: f64,
        var_vgsc_dn17: f64,
        var_vgsc_dn18: f64,
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
        var_cosh0_dn16_slot: &mut f64,
        var_cosh0_dn17_slot: &mut f64,
        var_cosh0_dn18_slot: &mut f64,
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
        var_cosh0_rdn16_slot: &mut f64,
        var_cosh0_rdn17_slot: &mut f64,
        var_cosh0_rdn18_slot: &mut f64,
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
        var_cosh1_dn16_slot: &mut f64,
        var_cosh1_dn17_slot: &mut f64,
        var_cosh1_dn18_slot: &mut f64,
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
        var_cosh1_rdn16_slot: &mut f64,
        var_cosh1_rdn17_slot: &mut f64,
        var_cosh1_rdn18_slot: &mut f64,
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
        var_lc40_dn16_slot: &mut f64,
        var_lc40_dn17_slot: &mut f64,
        var_lc40_dn18_slot: &mut f64,
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
        var_lc40_rdn16_slot: &mut f64,
        var_lc40_rdn17_slot: &mut f64,
        var_lc40_rdn18_slot: &mut f64,
        var_lc40_rdn2_slot: &mut f64,
        var_lc40_rdn3_slot: &mut f64,
        var_lc40_rdn4_slot: &mut f64,
        var_lc40_rdn5_slot: &mut f64,
        var_lc40_rdn6_slot: &mut f64,
        var_lc40_rdn7_slot: &mut f64,
        var_lc40_rdn8_slot: &mut f64,
        var_lc40_rdn9_slot: &mut f64,
        var_lc40_rv_slot: &mut f64,
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
        var_lc4_dn16_slot: &mut f64,
        var_lc4_dn17_slot: &mut f64,
        var_lc4_dn18_slot: &mut f64,
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
        var_lc4_rdn16_slot: &mut f64,
        var_lc4_rdn17_slot: &mut f64,
        var_lc4_rdn18_slot: &mut f64,
        var_lc4_rdn2_slot: &mut f64,
        var_lc4_rdn3_slot: &mut f64,
        var_lc4_rdn4_slot: &mut f64,
        var_lc4_rdn5_slot: &mut f64,
        var_lc4_rdn6_slot: &mut f64,
        var_lc4_rdn7_slot: &mut f64,
        var_lc4_rdn8_slot: &mut f64,
        var_lc4_rdn9_slot: &mut f64,
        var_lc4_rv_slot: &mut f64,
        var_qgs_slot: &mut f64,
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
        var_qgs_dn16_slot: &mut f64,
        var_qgs_dn17_slot: &mut f64,
        var_qgs_dn18_slot: &mut f64,
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
        var_qgs_rdn16_slot: &mut f64,
        var_qgs_rdn17_slot: &mut f64,
        var_qgs_rdn18_slot: &mut f64,
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
        let mut var_cosh0_dn16: f64 = *var_cosh0_dn16_slot;
        let mut var_cosh0_dn17: f64 = *var_cosh0_dn17_slot;
        let mut var_cosh0_dn18: f64 = *var_cosh0_dn18_slot;
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
        let mut var_cosh0_rdn16: f64 = *var_cosh0_rdn16_slot;
        let mut var_cosh0_rdn17: f64 = *var_cosh0_rdn17_slot;
        let mut var_cosh0_rdn18: f64 = *var_cosh0_rdn18_slot;
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
        let mut var_cosh1_dn16: f64 = *var_cosh1_dn16_slot;
        let mut var_cosh1_dn17: f64 = *var_cosh1_dn17_slot;
        let mut var_cosh1_dn18: f64 = *var_cosh1_dn18_slot;
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
        let mut var_cosh1_rdn16: f64 = *var_cosh1_rdn16_slot;
        let mut var_cosh1_rdn17: f64 = *var_cosh1_rdn17_slot;
        let mut var_cosh1_rdn18: f64 = *var_cosh1_rdn18_slot;
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
        let mut var_lc40_dn16: f64 = *var_lc40_dn16_slot;
        let mut var_lc40_dn17: f64 = *var_lc40_dn17_slot;
        let mut var_lc40_dn18: f64 = *var_lc40_dn18_slot;
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
        let mut var_lc40_rdn16: f64 = *var_lc40_rdn16_slot;
        let mut var_lc40_rdn17: f64 = *var_lc40_rdn17_slot;
        let mut var_lc40_rdn18: f64 = *var_lc40_rdn18_slot;
        let mut var_lc40_rdn2: f64 = *var_lc40_rdn2_slot;
        let mut var_lc40_rdn3: f64 = *var_lc40_rdn3_slot;
        let mut var_lc40_rdn4: f64 = *var_lc40_rdn4_slot;
        let mut var_lc40_rdn5: f64 = *var_lc40_rdn5_slot;
        let mut var_lc40_rdn6: f64 = *var_lc40_rdn6_slot;
        let mut var_lc40_rdn7: f64 = *var_lc40_rdn7_slot;
        let mut var_lc40_rdn8: f64 = *var_lc40_rdn8_slot;
        let mut var_lc40_rdn9: f64 = *var_lc40_rdn9_slot;
        let mut var_lc40_rv: f64 = *var_lc40_rv_slot;
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
        let mut var_lc4_dn16: f64 = *var_lc4_dn16_slot;
        let mut var_lc4_dn17: f64 = *var_lc4_dn17_slot;
        let mut var_lc4_dn18: f64 = *var_lc4_dn18_slot;
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
        let mut var_lc4_rdn16: f64 = *var_lc4_rdn16_slot;
        let mut var_lc4_rdn17: f64 = *var_lc4_rdn17_slot;
        let mut var_lc4_rdn18: f64 = *var_lc4_rdn18_slot;
        let mut var_lc4_rdn2: f64 = *var_lc4_rdn2_slot;
        let mut var_lc4_rdn3: f64 = *var_lc4_rdn3_slot;
        let mut var_lc4_rdn4: f64 = *var_lc4_rdn4_slot;
        let mut var_lc4_rdn5: f64 = *var_lc4_rdn5_slot;
        let mut var_lc4_rdn6: f64 = *var_lc4_rdn6_slot;
        let mut var_lc4_rdn7: f64 = *var_lc4_rdn7_slot;
        let mut var_lc4_rdn8: f64 = *var_lc4_rdn8_slot;
        let mut var_lc4_rdn9: f64 = *var_lc4_rdn9_slot;
        let mut var_lc4_rv: f64 = *var_lc4_rv_slot;
        let mut var_qgs: f64 = *var_qgs_slot;
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
        let mut var_qgs_dn16: f64 = *var_qgs_dn16_slot;
        let mut var_qgs_dn17: f64 = *var_qgs_dn17_slot;
        let mut var_qgs_dn18: f64 = *var_qgs_dn18_slot;
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
        let mut var_qgs_rdn16: f64 = *var_qgs_rdn16_slot;
        let mut var_qgs_rdn17: f64 = *var_qgs_rdn17_slot;
        let mut var_qgs_rdn18: f64 = *var_qgs_rdn18_slot;
        let mut var_qgs_rdn2: f64 = *var_qgs_rdn2_slot;
        let mut var_qgs_rdn3: f64 = *var_qgs_rdn3_slot;
        let mut var_qgs_rdn4: f64 = *var_qgs_rdn4_slot;
        let mut var_qgs_rdn5: f64 = *var_qgs_rdn5_slot;
        let mut var_qgs_rdn6: f64 = *var_qgs_rdn6_slot;
        let mut var_qgs_rdn7: f64 = *var_qgs_rdn7_slot;
        let mut var_qgs_rdn8: f64 = *var_qgs_rdn8_slot;
        let mut var_qgs_rdn9: f64 = *var_qgs_rdn9_slot;
        let mut var_qgs_rv: f64 = *var_qgs_rv_slot;

        let (assign1990_e2676, assign1990_e2676_d_n0, assign1990_e2676_d_n1, assign1990_e2676_d_n2, assign1990_e2676_d_n3, assign1990_e2676_d_n4, assign1990_e2676_d_n5, assign1990_e2676_d_n6, assign1990_e2676_d_n7, assign1990_e2676_d_n8, assign1990_e2676_d_n9, assign1990_e2676_d_n10, assign1990_e2676_d_n11, assign1990_e2676_d_n12, assign1990_e2676_d_n13, assign1990_e2676_d_n14, assign1990_e2676_d_n15, assign1990_e2676_d_n16, assign1990_e2676_d_n17, assign1990_e2676_d_n18, assign1990_e2676_d_b0, assign1990_e2676_d_b1, assign1990_e2676_d_b2, assign1990_e2676_d_b3, assign1990_e2676_d_b4, assign1990_e2676_d_b5, assign1990_e2676_d_b6, assign1990_e2676_d_b7, assign1990_e2676_d_b8, assign1990_e2676_d_b9, assign1990_e2676_d_b10, assign1990_e2676_d_b11, assign1990_e2676_d_b12, assign1990_e2676_d_b13, assign1990_e2676_d_b14, assign1990_e2676_d_b15, assign1990_e2676_d_b16, assign1990_e2676_d_b17, assign1990_e2676_d_b18,) = {
    if ((var_guard18 != 0.0) && (!((((var_guard14 != 0.0) || (var_guard15 != 0.0)) || (var_guard16 != 0.0)) || (var_guard17 != 0.0)))) {
        let assign1990_e2648: f64 = (var_psi_1 + var_lc1);
        let assign1990_e2650: f64 = (assign1990_e2648 - var_qgs0);
        let assign1990_e2652: f64 = (assign1990_e2650 + var_qgsdepl);
        let assign1990_e2654: f64 = (assign1990_e2652 - var_qgsdepl0);
        let assign1990_e2657: f64 = (1.0 - p.p38);
        let assign1990_e2659: f64 = (var_psi_2).tanh();
        let assign1990_e2660: f64 = (assign1990_e2657 + assign1990_e2659);
        let assign1990_e2661: f64 = (assign1990_e2654 * assign1990_e2660);
        let assign1990_e2663: f64 = (assign1990_e2661 / p.p31);
        let assign1990_e2666: f64 = (2.0 * p.p38);
        let assign1990_e2668: f64 = (assign1990_e2666 * var_vgsc);
        let assign1990_e2669: f64 = (assign1990_e2663 + assign1990_e2668);
        let assign1990_e2670: f64 = (var_cgs0_t * assign1990_e2669);
        let assign1990_e2673: f64 = (p.p25 * var_vgsc);
        let assign1990_e2674: f64 = (assign1990_e2670 + assign1990_e2673);
        (assign1990_e2674, (((var_cgs0_t_dn0 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn0 + var_lc1_dn0) - var_qgs0_dn0) + var_qgsdepl_dn0) - var_qgsdepl0_dn0) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn0 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn0)))) + (p.p25 * var_vgsc_dn0)), (((var_cgs0_t_dn1 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn1 + var_lc1_dn1) - var_qgs0_dn1) + var_qgsdepl_dn1) - var_qgsdepl0_dn1) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn1 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn1)))) + (p.p25 * var_vgsc_dn1)), (((var_cgs0_t_dn2 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn2 + var_lc1_dn2) - var_qgs0_dn2) + var_qgsdepl_dn2) - var_qgsdepl0_dn2) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn2 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn2)))) + (p.p25 * var_vgsc_dn2)), (((var_cgs0_t_dn3 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn3 + var_lc1_dn3) - var_qgs0_dn3) + var_qgsdepl_dn3) - var_qgsdepl0_dn3) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn3 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn3)))) + (p.p25 * var_vgsc_dn3)), (((var_cgs0_t_dn4 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn4 + var_lc1_dn4) - var_qgs0_dn4) + var_qgsdepl_dn4) - var_qgsdepl0_dn4) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn4 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn4)))) + (p.p25 * var_vgsc_dn4)), (((var_cgs0_t_dn5 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn5 + var_lc1_dn5) - var_qgs0_dn5) + var_qgsdepl_dn5) - var_qgsdepl0_dn5) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn5 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn5)))) + (p.p25 * var_vgsc_dn5)), (((var_cgs0_t_dn6 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn6 + var_lc1_dn6) - var_qgs0_dn6) + var_qgsdepl_dn6) - var_qgsdepl0_dn6) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn6 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn6)))) + (p.p25 * var_vgsc_dn6)), (((var_cgs0_t_dn7 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn7 + var_lc1_dn7) - var_qgs0_dn7) + var_qgsdepl_dn7) - var_qgsdepl0_dn7) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn7 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn7)))) + (p.p25 * var_vgsc_dn7)), (((var_cgs0_t_dn8 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn8 + var_lc1_dn8) - var_qgs0_dn8) + var_qgsdepl_dn8) - var_qgsdepl0_dn8) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn8 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn8)))) + (p.p25 * var_vgsc_dn8)), (((var_cgs0_t_dn9 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn9 + var_lc1_dn9) - var_qgs0_dn9) + var_qgsdepl_dn9) - var_qgsdepl0_dn9) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn9 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn9)))) + (p.p25 * var_vgsc_dn9)), (((var_cgs0_t_dn10 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn10 + var_lc1_dn10) - var_qgs0_dn10) + var_qgsdepl_dn10) - var_qgsdepl0_dn10) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn10 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn10)))) + (p.p25 * var_vgsc_dn10)), (((var_cgs0_t_dn11 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn11 + var_lc1_dn11) - var_qgs0_dn11) + var_qgsdepl_dn11) - var_qgsdepl0_dn11) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn11 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn11)))) + (p.p25 * var_vgsc_dn11)), (((var_cgs0_t_dn12 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn12 + var_lc1_dn12) - var_qgs0_dn12) + var_qgsdepl_dn12) - var_qgsdepl0_dn12) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn12 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn12)))) + (p.p25 * var_vgsc_dn12)), (((var_cgs0_t_dn13 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn13 + var_lc1_dn13) - var_qgs0_dn13) + var_qgsdepl_dn13) - var_qgsdepl0_dn13) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn13 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn13)))) + (p.p25 * var_vgsc_dn13)), (((var_cgs0_t_dn14 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn14 + var_lc1_dn14) - var_qgs0_dn14) + var_qgsdepl_dn14) - var_qgsdepl0_dn14) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn14 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn14)))) + (p.p25 * var_vgsc_dn14)), (((var_cgs0_t_dn15 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn15 + var_lc1_dn15) - var_qgs0_dn15) + var_qgsdepl_dn15) - var_qgsdepl0_dn15) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn15 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn15)))) + (p.p25 * var_vgsc_dn15)), (((var_cgs0_t_dn16 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn16 + var_lc1_dn16) - var_qgs0_dn16) + var_qgsdepl_dn16) - var_qgsdepl0_dn16) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn16 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn16)))) + (p.p25 * var_vgsc_dn16)), (((var_cgs0_t_dn17 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn17 + var_lc1_dn17) - var_qgs0_dn17) + var_qgsdepl_dn17) - var_qgsdepl0_dn17) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn17 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn17)))) + (p.p25 * var_vgsc_dn17)), (((var_cgs0_t_dn18 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_dn18 + var_lc1_dn18) - var_qgs0_dn18) + var_qgsdepl_dn18) - var_qgsdepl0_dn18) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_dn18 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_dn18)))) + (p.p25 * var_vgsc_dn18)), (((var_cgs0_t_db0 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db0 + var_lc1_db0) - var_qgs0_db0) + var_qgsdepl_db0) - var_qgsdepl0_db0) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db0 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db0)))) + (p.p25 * var_vgsc_db0)), (((var_cgs0_t_db1 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db1 + var_lc1_db1) - var_qgs0_db1) + var_qgsdepl_db1) - var_qgsdepl0_db1) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db1 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db1)))) + (p.p25 * var_vgsc_db1)), (((var_cgs0_t_db2 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db2 + var_lc1_db2) - var_qgs0_db2) + var_qgsdepl_db2) - var_qgsdepl0_db2) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db2 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db2)))) + (p.p25 * var_vgsc_db2)), (((var_cgs0_t_db3 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db3 + var_lc1_db3) - var_qgs0_db3) + var_qgsdepl_db3) - var_qgsdepl0_db3) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db3 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db3)))) + (p.p25 * var_vgsc_db3)), (((var_cgs0_t_db4 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db4 + var_lc1_db4) - var_qgs0_db4) + var_qgsdepl_db4) - var_qgsdepl0_db4) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db4 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db4)))) + (p.p25 * var_vgsc_db4)), (((var_cgs0_t_db5 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db5 + var_lc1_db5) - var_qgs0_db5) + var_qgsdepl_db5) - var_qgsdepl0_db5) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db5 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db5)))) + (p.p25 * var_vgsc_db5)), (((var_cgs0_t_db6 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db6 + var_lc1_db6) - var_qgs0_db6) + var_qgsdepl_db6) - var_qgsdepl0_db6) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db6 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db6)))) + (p.p25 * var_vgsc_db6)), (((var_cgs0_t_db7 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db7 + var_lc1_db7) - var_qgs0_db7) + var_qgsdepl_db7) - var_qgsdepl0_db7) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db7 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db7)))) + (p.p25 * var_vgsc_db7)), (((var_cgs0_t_db8 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db8 + var_lc1_db8) - var_qgs0_db8) + var_qgsdepl_db8) - var_qgsdepl0_db8) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db8 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db8)))) + (p.p25 * var_vgsc_db8)), (((var_cgs0_t_db9 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db9 + var_lc1_db9) - var_qgs0_db9) + var_qgsdepl_db9) - var_qgsdepl0_db9) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db9 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db9)))) + (p.p25 * var_vgsc_db9)), (((var_cgs0_t_db10 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db10 + var_lc1_db10) - var_qgs0_db10) + var_qgsdepl_db10) - var_qgsdepl0_db10) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db10 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db10)))) + (p.p25 * var_vgsc_db10)), (((var_cgs0_t_db11 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db11 + var_lc1_db11) - var_qgs0_db11) + var_qgsdepl_db11) - var_qgsdepl0_db11) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db11 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db11)))) + (p.p25 * var_vgsc_db11)), (((var_cgs0_t_db12 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db12 + var_lc1_db12) - var_qgs0_db12) + var_qgsdepl_db12) - var_qgsdepl0_db12) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db12 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db12)))) + (p.p25 * var_vgsc_db12)), (((var_cgs0_t_db13 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db13 + var_lc1_db13) - var_qgs0_db13) + var_qgsdepl_db13) - var_qgsdepl0_db13) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db13 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db13)))) + (p.p25 * var_vgsc_db13)), (((var_cgs0_t_db14 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db14 + var_lc1_db14) - var_qgs0_db14) + var_qgsdepl_db14) - var_qgsdepl0_db14) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db14 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db14)))) + (p.p25 * var_vgsc_db14)), (((var_cgs0_t_db15 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db15 + var_lc1_db15) - var_qgs0_db15) + var_qgsdepl_db15) - var_qgsdepl0_db15) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db15 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db15)))) + (p.p25 * var_vgsc_db15)), (((var_cgs0_t_db16 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db16 + var_lc1_db16) - var_qgs0_db16) + var_qgsdepl_db16) - var_qgsdepl0_db16) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db16 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db16)))) + (p.p25 * var_vgsc_db16)), (((var_cgs0_t_db17 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db17 + var_lc1_db17) - var_qgs0_db17) + var_qgsdepl_db17) - var_qgsdepl0_db17) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db17 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db17)))) + (p.p25 * var_vgsc_db17)), (((var_cgs0_t_db18 * assign1990_e2669) + (var_cgs0_t * ((((((((var_psi_1_db18 + var_lc1_db18) - var_qgs0_db18) + var_qgsdepl_db18) - var_qgsdepl0_db18) * assign1990_e2660) + (assign1990_e2654 * (var_psi_2_db18 / ((var_psi_2).cosh() * (var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * var_vgsc_db18)))) + (p.p25 * var_vgsc_db18)),)
    } else {
        (var_qgs, var_qgs_dn0, var_qgs_dn1, var_qgs_dn2, var_qgs_dn3, var_qgs_dn4, var_qgs_dn5, var_qgs_dn6, var_qgs_dn7, var_qgs_dn8, var_qgs_dn9, var_qgs_dn10, var_qgs_dn11, var_qgs_dn12, var_qgs_dn13, var_qgs_dn14, var_qgs_dn15, var_qgs_dn16, var_qgs_dn17, var_qgs_dn18, var_qgs_db0, var_qgs_db1, var_qgs_db2, var_qgs_db3, var_qgs_db4, var_qgs_db5, var_qgs_db6, var_qgs_db7, var_qgs_db8, var_qgs_db9, var_qgs_db10, var_qgs_db11, var_qgs_db12, var_qgs_db13, var_qgs_db14, var_qgs_db15, var_qgs_db16, var_qgs_db17, var_qgs_db18,)
    }
};
        var_qgs = assign1990_e2676;
        var_qgs_dn0 = assign1990_e2676_d_n0;
        var_qgs_dn1 = assign1990_e2676_d_n1;
        var_qgs_dn2 = assign1990_e2676_d_n2;
        var_qgs_dn3 = assign1990_e2676_d_n3;
        var_qgs_dn4 = assign1990_e2676_d_n4;
        var_qgs_dn5 = assign1990_e2676_d_n5;
        var_qgs_dn6 = assign1990_e2676_d_n6;
        var_qgs_dn7 = assign1990_e2676_d_n7;
        var_qgs_dn8 = assign1990_e2676_d_n8;
        var_qgs_dn9 = assign1990_e2676_d_n9;
        var_qgs_dn10 = assign1990_e2676_d_n10;
        var_qgs_dn11 = assign1990_e2676_d_n11;
        var_qgs_dn12 = assign1990_e2676_d_n12;
        var_qgs_dn13 = assign1990_e2676_d_n13;
        var_qgs_dn14 = assign1990_e2676_d_n14;
        var_qgs_dn15 = assign1990_e2676_d_n15;
        var_qgs_dn16 = assign1990_e2676_d_n16;
        var_qgs_dn17 = assign1990_e2676_d_n17;
        var_qgs_dn18 = assign1990_e2676_d_n18;
        var_qgs_db0 = assign1990_e2676_d_b0;
        var_qgs_db1 = assign1990_e2676_d_b1;
        var_qgs_db2 = assign1990_e2676_d_b2;
        var_qgs_db3 = assign1990_e2676_d_b3;
        var_qgs_db4 = assign1990_e2676_d_b4;
        var_qgs_db5 = assign1990_e2676_d_b5;
        var_qgs_db6 = assign1990_e2676_d_b6;
        var_qgs_db7 = assign1990_e2676_d_b7;
        var_qgs_db8 = assign1990_e2676_d_b8;
        var_qgs_db9 = assign1990_e2676_d_b9;
        var_qgs_db10 = assign1990_e2676_d_b10;
        var_qgs_db11 = assign1990_e2676_d_b11;
        var_qgs_db12 = assign1990_e2676_d_b12;
        var_qgs_db13 = assign1990_e2676_d_b13;
        var_qgs_db14 = assign1990_e2676_d_b14;
        var_qgs_db15 = assign1990_e2676_d_b15;
        var_qgs_db16 = assign1990_e2676_d_b16;
        var_qgs_db17 = assign1990_e2676_d_b17;
        var_qgs_db18 = assign1990_e2676_d_b18;
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
        var_qgs_rdn16 = 0.0;
        var_qgs_rdn17 = 0.0;
        var_qgs_rdn18 = 0.0;
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

        let (assign2000_e2694, assign2000_e2694_d_n0, assign2000_e2694_d_n1, assign2000_e2694_d_n2, assign2000_e2694_d_n3, assign2000_e2694_d_n4, assign2000_e2694_d_n5, assign2000_e2694_d_n6, assign2000_e2694_d_n7, assign2000_e2694_d_n8, assign2000_e2694_d_n9, assign2000_e2694_d_n10, assign2000_e2694_d_n11, assign2000_e2694_d_n12, assign2000_e2694_d_n13, assign2000_e2694_d_n14, assign2000_e2694_d_n15, assign2000_e2694_d_n16, assign2000_e2694_d_n17, assign2000_e2694_d_n18, assign2000_e2694_d_b0, assign2000_e2694_d_b1, assign2000_e2694_d_b2, assign2000_e2694_d_b3, assign2000_e2694_d_b4, assign2000_e2694_d_b5, assign2000_e2694_d_b6, assign2000_e2694_d_b7, assign2000_e2694_d_b8, assign2000_e2694_d_b9, assign2000_e2694_d_b10, assign2000_e2694_d_b11, assign2000_e2694_d_b12, assign2000_e2694_d_b13, assign2000_e2694_d_b14, assign2000_e2694_d_b15, assign2000_e2694_d_b16, assign2000_e2694_d_b17, assign2000_e2694_d_b18,) = {
    if ((var_guard18 != 0.0) && (!((((var_guard14 != 0.0) || (var_guard15 != 0.0)) || (var_guard16 != 0.0)) || (var_guard17 != 0.0)))) {
        let assign2000_e2690: f64 = (p.p38 * var_vds);
        let assign2000_e2691: f64 = (var_p40_t - assign2000_e2690);
        let assign2000_e2692: f64 = (assign2000_e2691).cosh();
        (assign2000_e2692, ((assign2000_e2691).sinh() * (var_p40_t_dn0 - (p.p38 * var_vds_dn0))), ((assign2000_e2691).sinh() * (var_p40_t_dn1 - (p.p38 * var_vds_dn1))), ((assign2000_e2691).sinh() * (var_p40_t_dn2 - (p.p38 * var_vds_dn2))), ((assign2000_e2691).sinh() * (var_p40_t_dn3 - (p.p38 * var_vds_dn3))), ((assign2000_e2691).sinh() * (var_p40_t_dn4 - (p.p38 * var_vds_dn4))), ((assign2000_e2691).sinh() * (var_p40_t_dn5 - (p.p38 * var_vds_dn5))), ((assign2000_e2691).sinh() * (var_p40_t_dn6 - (p.p38 * var_vds_dn6))), ((assign2000_e2691).sinh() * (var_p40_t_dn7 - (p.p38 * var_vds_dn7))), ((assign2000_e2691).sinh() * (var_p40_t_dn8 - (p.p38 * var_vds_dn8))), ((assign2000_e2691).sinh() * (var_p40_t_dn9 - (p.p38 * var_vds_dn9))), ((assign2000_e2691).sinh() * (var_p40_t_dn10 - (p.p38 * var_vds_dn10))), ((assign2000_e2691).sinh() * (var_p40_t_dn11 - (p.p38 * var_vds_dn11))), ((assign2000_e2691).sinh() * (var_p40_t_dn12 - (p.p38 * var_vds_dn12))), ((assign2000_e2691).sinh() * (var_p40_t_dn13 - (p.p38 * var_vds_dn13))), ((assign2000_e2691).sinh() * (var_p40_t_dn14 - (p.p38 * var_vds_dn14))), ((assign2000_e2691).sinh() * (var_p40_t_dn15 - (p.p38 * var_vds_dn15))), ((assign2000_e2691).sinh() * (var_p40_t_dn16 - (p.p38 * var_vds_dn16))), ((assign2000_e2691).sinh() * (var_p40_t_dn17 - (p.p38 * var_vds_dn17))), ((assign2000_e2691).sinh() * (var_p40_t_dn18 - (p.p38 * var_vds_dn18))), ((assign2000_e2691).sinh() * (var_p40_t_db0 - (p.p38 * var_vds_db0))), ((assign2000_e2691).sinh() * (var_p40_t_db1 - (p.p38 * var_vds_db1))), ((assign2000_e2691).sinh() * (var_p40_t_db2 - (p.p38 * var_vds_db2))), ((assign2000_e2691).sinh() * (var_p40_t_db3 - (p.p38 * var_vds_db3))), ((assign2000_e2691).sinh() * (var_p40_t_db4 - (p.p38 * var_vds_db4))), ((assign2000_e2691).sinh() * (var_p40_t_db5 - (p.p38 * var_vds_db5))), ((assign2000_e2691).sinh() * (var_p40_t_db6 - (p.p38 * var_vds_db6))), ((assign2000_e2691).sinh() * (var_p40_t_db7 - (p.p38 * var_vds_db7))), ((assign2000_e2691).sinh() * (var_p40_t_db8 - (p.p38 * var_vds_db8))), ((assign2000_e2691).sinh() * (var_p40_t_db9 - (p.p38 * var_vds_db9))), ((assign2000_e2691).sinh() * (var_p40_t_db10 - (p.p38 * var_vds_db10))), ((assign2000_e2691).sinh() * (var_p40_t_db11 - (p.p38 * var_vds_db11))), ((assign2000_e2691).sinh() * (var_p40_t_db12 - (p.p38 * var_vds_db12))), ((assign2000_e2691).sinh() * (var_p40_t_db13 - (p.p38 * var_vds_db13))), ((assign2000_e2691).sinh() * (var_p40_t_db14 - (p.p38 * var_vds_db14))), ((assign2000_e2691).sinh() * (var_p40_t_db15 - (p.p38 * var_vds_db15))), ((assign2000_e2691).sinh() * (var_p40_t_db16 - (p.p38 * var_vds_db16))), ((assign2000_e2691).sinh() * (var_p40_t_db17 - (p.p38 * var_vds_db17))), ((assign2000_e2691).sinh() * (var_p40_t_db18 - (p.p38 * var_vds_db18))),)
    } else {
        (var_cosh0, var_cosh0_dn0, var_cosh0_dn1, var_cosh0_dn2, var_cosh0_dn3, var_cosh0_dn4, var_cosh0_dn5, var_cosh0_dn6, var_cosh0_dn7, var_cosh0_dn8, var_cosh0_dn9, var_cosh0_dn10, var_cosh0_dn11, var_cosh0_dn12, var_cosh0_dn13, var_cosh0_dn14, var_cosh0_dn15, var_cosh0_dn16, var_cosh0_dn17, var_cosh0_dn18, var_cosh0_db0, var_cosh0_db1, var_cosh0_db2, var_cosh0_db3, var_cosh0_db4, var_cosh0_db5, var_cosh0_db6, var_cosh0_db7, var_cosh0_db8, var_cosh0_db9, var_cosh0_db10, var_cosh0_db11, var_cosh0_db12, var_cosh0_db13, var_cosh0_db14, var_cosh0_db15, var_cosh0_db16, var_cosh0_db17, var_cosh0_db18,)
    }
};
        var_cosh0 = assign2000_e2694;
        var_cosh0_dn0 = assign2000_e2694_d_n0;
        var_cosh0_dn1 = assign2000_e2694_d_n1;
        var_cosh0_dn2 = assign2000_e2694_d_n2;
        var_cosh0_dn3 = assign2000_e2694_d_n3;
        var_cosh0_dn4 = assign2000_e2694_d_n4;
        var_cosh0_dn5 = assign2000_e2694_d_n5;
        var_cosh0_dn6 = assign2000_e2694_d_n6;
        var_cosh0_dn7 = assign2000_e2694_d_n7;
        var_cosh0_dn8 = assign2000_e2694_d_n8;
        var_cosh0_dn9 = assign2000_e2694_d_n9;
        var_cosh0_dn10 = assign2000_e2694_d_n10;
        var_cosh0_dn11 = assign2000_e2694_d_n11;
        var_cosh0_dn12 = assign2000_e2694_d_n12;
        var_cosh0_dn13 = assign2000_e2694_d_n13;
        var_cosh0_dn14 = assign2000_e2694_d_n14;
        var_cosh0_dn15 = assign2000_e2694_d_n15;
        var_cosh0_dn16 = assign2000_e2694_d_n16;
        var_cosh0_dn17 = assign2000_e2694_d_n17;
        var_cosh0_dn18 = assign2000_e2694_d_n18;
        var_cosh0_db0 = assign2000_e2694_d_b0;
        var_cosh0_db1 = assign2000_e2694_d_b1;
        var_cosh0_db2 = assign2000_e2694_d_b2;
        var_cosh0_db3 = assign2000_e2694_d_b3;
        var_cosh0_db4 = assign2000_e2694_d_b4;
        var_cosh0_db5 = assign2000_e2694_d_b5;
        var_cosh0_db6 = assign2000_e2694_d_b6;
        var_cosh0_db7 = assign2000_e2694_d_b7;
        var_cosh0_db8 = assign2000_e2694_d_b8;
        var_cosh0_db9 = assign2000_e2694_d_b9;
        var_cosh0_db10 = assign2000_e2694_d_b10;
        var_cosh0_db11 = assign2000_e2694_d_b11;
        var_cosh0_db12 = assign2000_e2694_d_b12;
        var_cosh0_db13 = assign2000_e2694_d_b13;
        var_cosh0_db14 = assign2000_e2694_d_b14;
        var_cosh0_db15 = assign2000_e2694_d_b15;
        var_cosh0_db16 = assign2000_e2694_d_b16;
        var_cosh0_db17 = assign2000_e2694_d_b17;
        var_cosh0_db18 = assign2000_e2694_d_b18;
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
        var_cosh0_rdn16 = 0.0;
        var_cosh0_rdn17 = 0.0;
        var_cosh0_rdn18 = 0.0;
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

        let (assign2010_e2708, assign2010_e2708_d_n0, assign2010_e2708_d_n1, assign2010_e2708_d_n2, assign2010_e2708_d_n3, assign2010_e2708_d_n4, assign2010_e2708_d_n5, assign2010_e2708_d_n6, assign2010_e2708_d_n7, assign2010_e2708_d_n8, assign2010_e2708_d_n9, assign2010_e2708_d_n10, assign2010_e2708_d_n11, assign2010_e2708_d_n12, assign2010_e2708_d_n13, assign2010_e2708_d_n14, assign2010_e2708_d_n15, assign2010_e2708_d_n16, assign2010_e2708_d_n17, assign2010_e2708_d_n18, assign2010_e2708_d_b0, assign2010_e2708_d_b1, assign2010_e2708_d_b2, assign2010_e2708_d_b3, assign2010_e2708_d_b4, assign2010_e2708_d_b5, assign2010_e2708_d_b6, assign2010_e2708_d_b7, assign2010_e2708_d_b8, assign2010_e2708_d_b9, assign2010_e2708_d_b10, assign2010_e2708_d_b11, assign2010_e2708_d_b12, assign2010_e2708_d_b13, assign2010_e2708_d_b14, assign2010_e2708_d_b15, assign2010_e2708_d_b16, assign2010_e2708_d_b17, assign2010_e2708_d_b18,) = {
    if ((var_guard18 != 0.0) && (!((((var_guard14 != 0.0) || (var_guard15 != 0.0)) || (var_guard16 != 0.0)) || (var_guard17 != 0.0)))) {
        let assign2010_e2706: f64 = (var_cosh0).ln();
        (assign2010_e2706, (var_cosh0_dn0 / var_cosh0), (var_cosh0_dn1 / var_cosh0), (var_cosh0_dn2 / var_cosh0), (var_cosh0_dn3 / var_cosh0), (var_cosh0_dn4 / var_cosh0), (var_cosh0_dn5 / var_cosh0), (var_cosh0_dn6 / var_cosh0), (var_cosh0_dn7 / var_cosh0), (var_cosh0_dn8 / var_cosh0), (var_cosh0_dn9 / var_cosh0), (var_cosh0_dn10 / var_cosh0), (var_cosh0_dn11 / var_cosh0), (var_cosh0_dn12 / var_cosh0), (var_cosh0_dn13 / var_cosh0), (var_cosh0_dn14 / var_cosh0), (var_cosh0_dn15 / var_cosh0), (var_cosh0_dn16 / var_cosh0), (var_cosh0_dn17 / var_cosh0), (var_cosh0_dn18 / var_cosh0), (var_cosh0_db0 / var_cosh0), (var_cosh0_db1 / var_cosh0), (var_cosh0_db2 / var_cosh0), (var_cosh0_db3 / var_cosh0), (var_cosh0_db4 / var_cosh0), (var_cosh0_db5 / var_cosh0), (var_cosh0_db6 / var_cosh0), (var_cosh0_db7 / var_cosh0), (var_cosh0_db8 / var_cosh0), (var_cosh0_db9 / var_cosh0), (var_cosh0_db10 / var_cosh0), (var_cosh0_db11 / var_cosh0), (var_cosh0_db12 / var_cosh0), (var_cosh0_db13 / var_cosh0), (var_cosh0_db14 / var_cosh0), (var_cosh0_db15 / var_cosh0), (var_cosh0_db16 / var_cosh0), (var_cosh0_db17 / var_cosh0), (var_cosh0_db18 / var_cosh0),)
    } else {
        (var_lc40, var_lc40_dn0, var_lc40_dn1, var_lc40_dn2, var_lc40_dn3, var_lc40_dn4, var_lc40_dn5, var_lc40_dn6, var_lc40_dn7, var_lc40_dn8, var_lc40_dn9, var_lc40_dn10, var_lc40_dn11, var_lc40_dn12, var_lc40_dn13, var_lc40_dn14, var_lc40_dn15, var_lc40_dn16, var_lc40_dn17, var_lc40_dn18, var_lc40_db0, var_lc40_db1, var_lc40_db2, var_lc40_db3, var_lc40_db4, var_lc40_db5, var_lc40_db6, var_lc40_db7, var_lc40_db8, var_lc40_db9, var_lc40_db10, var_lc40_db11, var_lc40_db12, var_lc40_db13, var_lc40_db14, var_lc40_db15, var_lc40_db16, var_lc40_db17, var_lc40_db18,)
    }
};
        var_lc40 = assign2010_e2708;
        var_lc40_dn0 = assign2010_e2708_d_n0;
        var_lc40_dn1 = assign2010_e2708_d_n1;
        var_lc40_dn2 = assign2010_e2708_d_n2;
        var_lc40_dn3 = assign2010_e2708_d_n3;
        var_lc40_dn4 = assign2010_e2708_d_n4;
        var_lc40_dn5 = assign2010_e2708_d_n5;
        var_lc40_dn6 = assign2010_e2708_d_n6;
        var_lc40_dn7 = assign2010_e2708_d_n7;
        var_lc40_dn8 = assign2010_e2708_d_n8;
        var_lc40_dn9 = assign2010_e2708_d_n9;
        var_lc40_dn10 = assign2010_e2708_d_n10;
        var_lc40_dn11 = assign2010_e2708_d_n11;
        var_lc40_dn12 = assign2010_e2708_d_n12;
        var_lc40_dn13 = assign2010_e2708_d_n13;
        var_lc40_dn14 = assign2010_e2708_d_n14;
        var_lc40_dn15 = assign2010_e2708_d_n15;
        var_lc40_dn16 = assign2010_e2708_d_n16;
        var_lc40_dn17 = assign2010_e2708_d_n17;
        var_lc40_dn18 = assign2010_e2708_d_n18;
        var_lc40_db0 = assign2010_e2708_d_b0;
        var_lc40_db1 = assign2010_e2708_d_b1;
        var_lc40_db2 = assign2010_e2708_d_b2;
        var_lc40_db3 = assign2010_e2708_d_b3;
        var_lc40_db4 = assign2010_e2708_d_b4;
        var_lc40_db5 = assign2010_e2708_d_b5;
        var_lc40_db6 = assign2010_e2708_d_b6;
        var_lc40_db7 = assign2010_e2708_d_b7;
        var_lc40_db8 = assign2010_e2708_d_b8;
        var_lc40_db9 = assign2010_e2708_d_b9;
        var_lc40_db10 = assign2010_e2708_d_b10;
        var_lc40_db11 = assign2010_e2708_d_b11;
        var_lc40_db12 = assign2010_e2708_d_b12;
        var_lc40_db13 = assign2010_e2708_d_b13;
        var_lc40_db14 = assign2010_e2708_d_b14;
        var_lc40_db15 = assign2010_e2708_d_b15;
        var_lc40_db16 = assign2010_e2708_d_b16;
        var_lc40_db17 = assign2010_e2708_d_b17;
        var_lc40_db18 = assign2010_e2708_d_b18;
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
        var_lc40_rdn16 = 0.0;
        var_lc40_rdn17 = 0.0;
        var_lc40_rdn18 = 0.0;
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

        let (assign2020_e2722, assign2020_e2722_d_n0, assign2020_e2722_d_n1, assign2020_e2722_d_n2, assign2020_e2722_d_n3, assign2020_e2722_d_n4, assign2020_e2722_d_n5, assign2020_e2722_d_n6, assign2020_e2722_d_n7, assign2020_e2722_d_n8, assign2020_e2722_d_n9, assign2020_e2722_d_n10, assign2020_e2722_d_n11, assign2020_e2722_d_n12, assign2020_e2722_d_n13, assign2020_e2722_d_n14, assign2020_e2722_d_n15, assign2020_e2722_d_n16, assign2020_e2722_d_n17, assign2020_e2722_d_n18, assign2020_e2722_d_b0, assign2020_e2722_d_b1, assign2020_e2722_d_b2, assign2020_e2722_d_b3, assign2020_e2722_d_b4, assign2020_e2722_d_b5, assign2020_e2722_d_b6, assign2020_e2722_d_b7, assign2020_e2722_d_b8, assign2020_e2722_d_b9, assign2020_e2722_d_b10, assign2020_e2722_d_b11, assign2020_e2722_d_b12, assign2020_e2722_d_b13, assign2020_e2722_d_b14, assign2020_e2722_d_b15, assign2020_e2722_d_b16, assign2020_e2722_d_b17, assign2020_e2722_d_b18,) = {
    if ((var_guard18 != 0.0) && (!((((var_guard14 != 0.0) || (var_guard15 != 0.0)) || (var_guard16 != 0.0)) || (var_guard17 != 0.0)))) {
        let assign2020_e2720: f64 = (var_psi_4).cosh();
        (assign2020_e2720, ((var_psi_4).sinh() * var_psi_4_dn0), ((var_psi_4).sinh() * var_psi_4_dn1), ((var_psi_4).sinh() * var_psi_4_dn2), ((var_psi_4).sinh() * var_psi_4_dn3), ((var_psi_4).sinh() * var_psi_4_dn4), ((var_psi_4).sinh() * var_psi_4_dn5), ((var_psi_4).sinh() * var_psi_4_dn6), ((var_psi_4).sinh() * var_psi_4_dn7), ((var_psi_4).sinh() * var_psi_4_dn8), ((var_psi_4).sinh() * var_psi_4_dn9), ((var_psi_4).sinh() * var_psi_4_dn10), ((var_psi_4).sinh() * var_psi_4_dn11), ((var_psi_4).sinh() * var_psi_4_dn12), ((var_psi_4).sinh() * var_psi_4_dn13), ((var_psi_4).sinh() * var_psi_4_dn14), ((var_psi_4).sinh() * var_psi_4_dn15), ((var_psi_4).sinh() * var_psi_4_dn16), ((var_psi_4).sinh() * var_psi_4_dn17), ((var_psi_4).sinh() * var_psi_4_dn18), ((var_psi_4).sinh() * var_psi_4_db0), ((var_psi_4).sinh() * var_psi_4_db1), ((var_psi_4).sinh() * var_psi_4_db2), ((var_psi_4).sinh() * var_psi_4_db3), ((var_psi_4).sinh() * var_psi_4_db4), ((var_psi_4).sinh() * var_psi_4_db5), ((var_psi_4).sinh() * var_psi_4_db6), ((var_psi_4).sinh() * var_psi_4_db7), ((var_psi_4).sinh() * var_psi_4_db8), ((var_psi_4).sinh() * var_psi_4_db9), ((var_psi_4).sinh() * var_psi_4_db10), ((var_psi_4).sinh() * var_psi_4_db11), ((var_psi_4).sinh() * var_psi_4_db12), ((var_psi_4).sinh() * var_psi_4_db13), ((var_psi_4).sinh() * var_psi_4_db14), ((var_psi_4).sinh() * var_psi_4_db15), ((var_psi_4).sinh() * var_psi_4_db16), ((var_psi_4).sinh() * var_psi_4_db17), ((var_psi_4).sinh() * var_psi_4_db18),)
    } else {
        (var_cosh1, var_cosh1_dn0, var_cosh1_dn1, var_cosh1_dn2, var_cosh1_dn3, var_cosh1_dn4, var_cosh1_dn5, var_cosh1_dn6, var_cosh1_dn7, var_cosh1_dn8, var_cosh1_dn9, var_cosh1_dn10, var_cosh1_dn11, var_cosh1_dn12, var_cosh1_dn13, var_cosh1_dn14, var_cosh1_dn15, var_cosh1_dn16, var_cosh1_dn17, var_cosh1_dn18, var_cosh1_db0, var_cosh1_db1, var_cosh1_db2, var_cosh1_db3, var_cosh1_db4, var_cosh1_db5, var_cosh1_db6, var_cosh1_db7, var_cosh1_db8, var_cosh1_db9, var_cosh1_db10, var_cosh1_db11, var_cosh1_db12, var_cosh1_db13, var_cosh1_db14, var_cosh1_db15, var_cosh1_db16, var_cosh1_db17, var_cosh1_db18,)
    }
};
        var_cosh1 = assign2020_e2722;
        var_cosh1_dn0 = assign2020_e2722_d_n0;
        var_cosh1_dn1 = assign2020_e2722_d_n1;
        var_cosh1_dn2 = assign2020_e2722_d_n2;
        var_cosh1_dn3 = assign2020_e2722_d_n3;
        var_cosh1_dn4 = assign2020_e2722_d_n4;
        var_cosh1_dn5 = assign2020_e2722_d_n5;
        var_cosh1_dn6 = assign2020_e2722_d_n6;
        var_cosh1_dn7 = assign2020_e2722_d_n7;
        var_cosh1_dn8 = assign2020_e2722_d_n8;
        var_cosh1_dn9 = assign2020_e2722_d_n9;
        var_cosh1_dn10 = assign2020_e2722_d_n10;
        var_cosh1_dn11 = assign2020_e2722_d_n11;
        var_cosh1_dn12 = assign2020_e2722_d_n12;
        var_cosh1_dn13 = assign2020_e2722_d_n13;
        var_cosh1_dn14 = assign2020_e2722_d_n14;
        var_cosh1_dn15 = assign2020_e2722_d_n15;
        var_cosh1_dn16 = assign2020_e2722_d_n16;
        var_cosh1_dn17 = assign2020_e2722_d_n17;
        var_cosh1_dn18 = assign2020_e2722_d_n18;
        var_cosh1_db0 = assign2020_e2722_d_b0;
        var_cosh1_db1 = assign2020_e2722_d_b1;
        var_cosh1_db2 = assign2020_e2722_d_b2;
        var_cosh1_db3 = assign2020_e2722_d_b3;
        var_cosh1_db4 = assign2020_e2722_d_b4;
        var_cosh1_db5 = assign2020_e2722_d_b5;
        var_cosh1_db6 = assign2020_e2722_d_b6;
        var_cosh1_db7 = assign2020_e2722_d_b7;
        var_cosh1_db8 = assign2020_e2722_d_b8;
        var_cosh1_db9 = assign2020_e2722_d_b9;
        var_cosh1_db10 = assign2020_e2722_d_b10;
        var_cosh1_db11 = assign2020_e2722_d_b11;
        var_cosh1_db12 = assign2020_e2722_d_b12;
        var_cosh1_db13 = assign2020_e2722_d_b13;
        var_cosh1_db14 = assign2020_e2722_d_b14;
        var_cosh1_db15 = assign2020_e2722_d_b15;
        var_cosh1_db16 = assign2020_e2722_d_b16;
        var_cosh1_db17 = assign2020_e2722_d_b17;
        var_cosh1_db18 = assign2020_e2722_d_b18;
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
        var_cosh1_rdn16 = 0.0;
        var_cosh1_rdn17 = 0.0;
        var_cosh1_rdn18 = 0.0;
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

        let (assign2030_e2736, assign2030_e2736_d_n0, assign2030_e2736_d_n1, assign2030_e2736_d_n2, assign2030_e2736_d_n3, assign2030_e2736_d_n4, assign2030_e2736_d_n5, assign2030_e2736_d_n6, assign2030_e2736_d_n7, assign2030_e2736_d_n8, assign2030_e2736_d_n9, assign2030_e2736_d_n10, assign2030_e2736_d_n11, assign2030_e2736_d_n12, assign2030_e2736_d_n13, assign2030_e2736_d_n14, assign2030_e2736_d_n15, assign2030_e2736_d_n16, assign2030_e2736_d_n17, assign2030_e2736_d_n18, assign2030_e2736_d_b0, assign2030_e2736_d_b1, assign2030_e2736_d_b2, assign2030_e2736_d_b3, assign2030_e2736_d_b4, assign2030_e2736_d_b5, assign2030_e2736_d_b6, assign2030_e2736_d_b7, assign2030_e2736_d_b8, assign2030_e2736_d_b9, assign2030_e2736_d_b10, assign2030_e2736_d_b11, assign2030_e2736_d_b12, assign2030_e2736_d_b13, assign2030_e2736_d_b14, assign2030_e2736_d_b15, assign2030_e2736_d_b16, assign2030_e2736_d_b17, assign2030_e2736_d_b18,) = {
    if ((var_guard18 != 0.0) && (!((((var_guard14 != 0.0) || (var_guard15 != 0.0)) || (var_guard16 != 0.0)) || (var_guard17 != 0.0)))) {
        let assign2030_e2734: f64 = (var_cosh1).ln();
        (assign2030_e2734, (var_cosh1_dn0 / var_cosh1), (var_cosh1_dn1 / var_cosh1), (var_cosh1_dn2 / var_cosh1), (var_cosh1_dn3 / var_cosh1), (var_cosh1_dn4 / var_cosh1), (var_cosh1_dn5 / var_cosh1), (var_cosh1_dn6 / var_cosh1), (var_cosh1_dn7 / var_cosh1), (var_cosh1_dn8 / var_cosh1), (var_cosh1_dn9 / var_cosh1), (var_cosh1_dn10 / var_cosh1), (var_cosh1_dn11 / var_cosh1), (var_cosh1_dn12 / var_cosh1), (var_cosh1_dn13 / var_cosh1), (var_cosh1_dn14 / var_cosh1), (var_cosh1_dn15 / var_cosh1), (var_cosh1_dn16 / var_cosh1), (var_cosh1_dn17 / var_cosh1), (var_cosh1_dn18 / var_cosh1), (var_cosh1_db0 / var_cosh1), (var_cosh1_db1 / var_cosh1), (var_cosh1_db2 / var_cosh1), (var_cosh1_db3 / var_cosh1), (var_cosh1_db4 / var_cosh1), (var_cosh1_db5 / var_cosh1), (var_cosh1_db6 / var_cosh1), (var_cosh1_db7 / var_cosh1), (var_cosh1_db8 / var_cosh1), (var_cosh1_db9 / var_cosh1), (var_cosh1_db10 / var_cosh1), (var_cosh1_db11 / var_cosh1), (var_cosh1_db12 / var_cosh1), (var_cosh1_db13 / var_cosh1), (var_cosh1_db14 / var_cosh1), (var_cosh1_db15 / var_cosh1), (var_cosh1_db16 / var_cosh1), (var_cosh1_db17 / var_cosh1), (var_cosh1_db18 / var_cosh1),)
    } else {
        (var_lc4, var_lc4_dn0, var_lc4_dn1, var_lc4_dn2, var_lc4_dn3, var_lc4_dn4, var_lc4_dn5, var_lc4_dn6, var_lc4_dn7, var_lc4_dn8, var_lc4_dn9, var_lc4_dn10, var_lc4_dn11, var_lc4_dn12, var_lc4_dn13, var_lc4_dn14, var_lc4_dn15, var_lc4_dn16, var_lc4_dn17, var_lc4_dn18, var_lc4_db0, var_lc4_db1, var_lc4_db2, var_lc4_db3, var_lc4_db4, var_lc4_db5, var_lc4_db6, var_lc4_db7, var_lc4_db8, var_lc4_db9, var_lc4_db10, var_lc4_db11, var_lc4_db12, var_lc4_db13, var_lc4_db14, var_lc4_db15, var_lc4_db16, var_lc4_db17, var_lc4_db18,)
    }
};
        var_lc4 = assign2030_e2736;
        var_lc4_dn0 = assign2030_e2736_d_n0;
        var_lc4_dn1 = assign2030_e2736_d_n1;
        var_lc4_dn2 = assign2030_e2736_d_n2;
        var_lc4_dn3 = assign2030_e2736_d_n3;
        var_lc4_dn4 = assign2030_e2736_d_n4;
        var_lc4_dn5 = assign2030_e2736_d_n5;
        var_lc4_dn6 = assign2030_e2736_d_n6;
        var_lc4_dn7 = assign2030_e2736_d_n7;
        var_lc4_dn8 = assign2030_e2736_d_n8;
        var_lc4_dn9 = assign2030_e2736_d_n9;
        var_lc4_dn10 = assign2030_e2736_d_n10;
        var_lc4_dn11 = assign2030_e2736_d_n11;
        var_lc4_dn12 = assign2030_e2736_d_n12;
        var_lc4_dn13 = assign2030_e2736_d_n13;
        var_lc4_dn14 = assign2030_e2736_d_n14;
        var_lc4_dn15 = assign2030_e2736_d_n15;
        var_lc4_dn16 = assign2030_e2736_d_n16;
        var_lc4_dn17 = assign2030_e2736_d_n17;
        var_lc4_dn18 = assign2030_e2736_d_n18;
        var_lc4_db0 = assign2030_e2736_d_b0;
        var_lc4_db1 = assign2030_e2736_d_b1;
        var_lc4_db2 = assign2030_e2736_d_b2;
        var_lc4_db3 = assign2030_e2736_d_b3;
        var_lc4_db4 = assign2030_e2736_d_b4;
        var_lc4_db5 = assign2030_e2736_d_b5;
        var_lc4_db6 = assign2030_e2736_d_b6;
        var_lc4_db7 = assign2030_e2736_d_b7;
        var_lc4_db8 = assign2030_e2736_d_b8;
        var_lc4_db9 = assign2030_e2736_d_b9;
        var_lc4_db10 = assign2030_e2736_d_b10;
        var_lc4_db11 = assign2030_e2736_d_b11;
        var_lc4_db12 = assign2030_e2736_d_b12;
        var_lc4_db13 = assign2030_e2736_d_b13;
        var_lc4_db14 = assign2030_e2736_d_b14;
        var_lc4_db15 = assign2030_e2736_d_b15;
        var_lc4_db16 = assign2030_e2736_d_b16;
        var_lc4_db17 = assign2030_e2736_d_b17;
        var_lc4_db18 = assign2030_e2736_d_b18;
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
        var_lc4_rdn16 = 0.0;
        var_lc4_rdn17 = 0.0;
        var_lc4_rdn18 = 0.0;
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
        *var_cosh0_dn16_slot = var_cosh0_dn16;
        *var_cosh0_dn17_slot = var_cosh0_dn17;
        *var_cosh0_dn18_slot = var_cosh0_dn18;
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
        *var_cosh0_rdn16_slot = var_cosh0_rdn16;
        *var_cosh0_rdn17_slot = var_cosh0_rdn17;
        *var_cosh0_rdn18_slot = var_cosh0_rdn18;
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
        *var_cosh1_dn16_slot = var_cosh1_dn16;
        *var_cosh1_dn17_slot = var_cosh1_dn17;
        *var_cosh1_dn18_slot = var_cosh1_dn18;
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
        *var_cosh1_rdn16_slot = var_cosh1_rdn16;
        *var_cosh1_rdn17_slot = var_cosh1_rdn17;
        *var_cosh1_rdn18_slot = var_cosh1_rdn18;
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
        *var_lc40_dn16_slot = var_lc40_dn16;
        *var_lc40_dn17_slot = var_lc40_dn17;
        *var_lc40_dn18_slot = var_lc40_dn18;
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
        *var_lc40_rdn16_slot = var_lc40_rdn16;
        *var_lc40_rdn17_slot = var_lc40_rdn17;
        *var_lc40_rdn18_slot = var_lc40_rdn18;
        *var_lc40_rdn2_slot = var_lc40_rdn2;
        *var_lc40_rdn3_slot = var_lc40_rdn3;
        *var_lc40_rdn4_slot = var_lc40_rdn4;
        *var_lc40_rdn5_slot = var_lc40_rdn5;
        *var_lc40_rdn6_slot = var_lc40_rdn6;
        *var_lc40_rdn7_slot = var_lc40_rdn7;
        *var_lc40_rdn8_slot = var_lc40_rdn8;
        *var_lc40_rdn9_slot = var_lc40_rdn9;
        *var_lc40_rv_slot = var_lc40_rv;
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
        *var_lc4_dn16_slot = var_lc4_dn16;
        *var_lc4_dn17_slot = var_lc4_dn17;
        *var_lc4_dn18_slot = var_lc4_dn18;
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
        *var_lc4_rdn16_slot = var_lc4_rdn16;
        *var_lc4_rdn17_slot = var_lc4_rdn17;
        *var_lc4_rdn18_slot = var_lc4_rdn18;
        *var_lc4_rdn2_slot = var_lc4_rdn2;
        *var_lc4_rdn3_slot = var_lc4_rdn3;
        *var_lc4_rdn4_slot = var_lc4_rdn4;
        *var_lc4_rdn5_slot = var_lc4_rdn5;
        *var_lc4_rdn6_slot = var_lc4_rdn6;
        *var_lc4_rdn7_slot = var_lc4_rdn7;
        *var_lc4_rdn8_slot = var_lc4_rdn8;
        *var_lc4_rdn9_slot = var_lc4_rdn9;
        *var_lc4_rv_slot = var_lc4_rv;
        *var_qgs_slot = var_qgs;
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
        *var_qgs_dn16_slot = var_qgs_dn16;
        *var_qgs_dn17_slot = var_qgs_dn17;
        *var_qgs_dn18_slot = var_qgs_dn18;
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
        *var_qgs_rdn16_slot = var_qgs_rdn16;
        *var_qgs_rdn17_slot = var_qgs_rdn17;
        *var_qgs_rdn18_slot = var_qgs_rdn18;
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

    pub(super) fn stamp_reactive_block_37(
        p: &Parameters,
        var_cgd0_t: f64,
        var_cgd0_t_db0: f64,
        var_cgd0_t_db1: f64,
        var_cgd0_t_db10: f64,
        var_cgd0_t_db11: f64,
        var_cgd0_t_db12: f64,
        var_cgd0_t_db13: f64,
        var_cgd0_t_db14: f64,
        var_cgd0_t_db15: f64,
        var_cgd0_t_db16: f64,
        var_cgd0_t_db17: f64,
        var_cgd0_t_db18: f64,
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
        var_cgd0_t_dn16: f64,
        var_cgd0_t_dn17: f64,
        var_cgd0_t_dn18: f64,
        var_cgd0_t_dn2: f64,
        var_cgd0_t_dn3: f64,
        var_cgd0_t_dn4: f64,
        var_cgd0_t_dn5: f64,
        var_cgd0_t_dn6: f64,
        var_cgd0_t_dn7: f64,
        var_cgd0_t_dn8: f64,
        var_cgd0_t_dn9: f64,
        var_guard14: f64,
        var_guard15: f64,
        var_guard16: f64,
        var_guard17: f64,
        var_guard18: f64,
        var_lc4: f64,
        var_lc40: f64,
        var_lc40_db0: f64,
        var_lc40_db1: f64,
        var_lc40_db10: f64,
        var_lc40_db11: f64,
        var_lc40_db12: f64,
        var_lc40_db13: f64,
        var_lc40_db14: f64,
        var_lc40_db15: f64,
        var_lc40_db16: f64,
        var_lc40_db17: f64,
        var_lc40_db18: f64,
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
        var_lc40_dn16: f64,
        var_lc40_dn17: f64,
        var_lc40_dn18: f64,
        var_lc40_dn2: f64,
        var_lc40_dn3: f64,
        var_lc40_dn4: f64,
        var_lc40_dn5: f64,
        var_lc40_dn6: f64,
        var_lc40_dn7: f64,
        var_lc40_dn8: f64,
        var_lc40_dn9: f64,
        var_lc4_db0: f64,
        var_lc4_db1: f64,
        var_lc4_db10: f64,
        var_lc4_db11: f64,
        var_lc4_db12: f64,
        var_lc4_db13: f64,
        var_lc4_db14: f64,
        var_lc4_db15: f64,
        var_lc4_db16: f64,
        var_lc4_db17: f64,
        var_lc4_db18: f64,
        var_lc4_db2: f64,
        var_lc4_db3: f64,
        var_lc4_db4: f64,
        var_lc4_db5: f64,
        var_lc4_db6: f64,
        var_lc4_db7: f64,
        var_lc4_db8: f64,
        var_lc4_db9: f64,
        var_lc4_dn0: f64,
        var_lc4_dn1: f64,
        var_lc4_dn10: f64,
        var_lc4_dn11: f64,
        var_lc4_dn12: f64,
        var_lc4_dn13: f64,
        var_lc4_dn14: f64,
        var_lc4_dn15: f64,
        var_lc4_dn16: f64,
        var_lc4_dn17: f64,
        var_lc4_dn18: f64,
        var_lc4_dn2: f64,
        var_lc4_dn3: f64,
        var_lc4_dn4: f64,
        var_lc4_dn5: f64,
        var_lc4_dn6: f64,
        var_lc4_dn7: f64,
        var_lc4_dn8: f64,
        var_lc4_dn9: f64,
        var_p40_t: f64,
        var_p40_t_db0: f64,
        var_p40_t_db1: f64,
        var_p40_t_db10: f64,
        var_p40_t_db11: f64,
        var_p40_t_db12: f64,
        var_p40_t_db13: f64,
        var_p40_t_db14: f64,
        var_p40_t_db15: f64,
        var_p40_t_db16: f64,
        var_p40_t_db17: f64,
        var_p40_t_db18: f64,
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
        var_p40_t_dn16: f64,
        var_p40_t_dn17: f64,
        var_p40_t_dn18: f64,
        var_p40_t_dn2: f64,
        var_p40_t_dn3: f64,
        var_p40_t_dn4: f64,
        var_p40_t_dn5: f64,
        var_p40_t_dn6: f64,
        var_p40_t_dn7: f64,
        var_p40_t_dn8: f64,
        var_p40_t_dn9: f64,
        var_psi_3: f64,
        var_psi_3_db0: f64,
        var_psi_3_db1: f64,
        var_psi_3_db10: f64,
        var_psi_3_db11: f64,
        var_psi_3_db12: f64,
        var_psi_3_db13: f64,
        var_psi_3_db14: f64,
        var_psi_3_db15: f64,
        var_psi_3_db16: f64,
        var_psi_3_db17: f64,
        var_psi_3_db18: f64,
        var_psi_3_db2: f64,
        var_psi_3_db3: f64,
        var_psi_3_db4: f64,
        var_psi_3_db5: f64,
        var_psi_3_db6: f64,
        var_psi_3_db7: f64,
        var_psi_3_db8: f64,
        var_psi_3_db9: f64,
        var_psi_3_dn0: f64,
        var_psi_3_dn1: f64,
        var_psi_3_dn10: f64,
        var_psi_3_dn11: f64,
        var_psi_3_dn12: f64,
        var_psi_3_dn13: f64,
        var_psi_3_dn14: f64,
        var_psi_3_dn15: f64,
        var_psi_3_dn16: f64,
        var_psi_3_dn17: f64,
        var_psi_3_dn18: f64,
        var_psi_3_dn2: f64,
        var_psi_3_dn3: f64,
        var_psi_3_dn4: f64,
        var_psi_3_dn5: f64,
        var_psi_3_dn6: f64,
        var_psi_3_dn7: f64,
        var_psi_3_dn8: f64,
        var_psi_3_dn9: f64,
        var_psi_4: f64,
        var_psi_4_db0: f64,
        var_psi_4_db1: f64,
        var_psi_4_db10: f64,
        var_psi_4_db11: f64,
        var_psi_4_db12: f64,
        var_psi_4_db13: f64,
        var_psi_4_db14: f64,
        var_psi_4_db15: f64,
        var_psi_4_db16: f64,
        var_psi_4_db17: f64,
        var_psi_4_db18: f64,
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
        var_psi_4_dn16: f64,
        var_psi_4_dn17: f64,
        var_psi_4_dn18: f64,
        var_psi_4_dn2: f64,
        var_psi_4_dn3: f64,
        var_psi_4_dn4: f64,
        var_psi_4_dn5: f64,
        var_psi_4_dn6: f64,
        var_psi_4_dn7: f64,
        var_psi_4_dn8: f64,
        var_psi_4_dn9: f64,
        var_qgs_dn11: f64,
        var_vds: f64,
        var_vds_db0: f64,
        var_vds_db1: f64,
        var_vds_db10: f64,
        var_vds_db11: f64,
        var_vds_db12: f64,
        var_vds_db13: f64,
        var_vds_db14: f64,
        var_vds_db15: f64,
        var_vds_db16: f64,
        var_vds_db17: f64,
        var_vds_db18: f64,
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
        var_vds_dn16: f64,
        var_vds_dn17: f64,
        var_vds_dn18: f64,
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
        var_vgdc_dn16: f64,
        var_vgdc_dn17: f64,
        var_vgdc_dn18: f64,
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
        var_cgd_dn16_slot: &mut f64,
        var_cgd_dn17_slot: &mut f64,
        var_cgd_dn18_slot: &mut f64,
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
        var_cgd_rdn16_slot: &mut f64,
        var_cgd_rdn17_slot: &mut f64,
        var_cgd_rdn18_slot: &mut f64,
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
        var_cgs_dn16_slot: &mut f64,
        var_cgs_dn17_slot: &mut f64,
        var_cgs_dn18_slot: &mut f64,
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
        var_cgs_rdn16_slot: &mut f64,
        var_cgs_rdn17_slot: &mut f64,
        var_cgs_rdn18_slot: &mut f64,
        var_cgs_rdn2_slot: &mut f64,
        var_cgs_rdn3_slot: &mut f64,
        var_cgs_rdn4_slot: &mut f64,
        var_cgs_rdn5_slot: &mut f64,
        var_cgs_rdn6_slot: &mut f64,
        var_cgs_rdn7_slot: &mut f64,
        var_cgs_rdn8_slot: &mut f64,
        var_cgs_rdn9_slot: &mut f64,
        var_cgs_rv_slot: &mut f64,
        var_guard19_slot: &mut f64,
        var_guard19_db0_slot: &mut f64,
        var_guard19_db1_slot: &mut f64,
        var_guard19_db10_slot: &mut f64,
        var_guard19_db11_slot: &mut f64,
        var_guard19_db12_slot: &mut f64,
        var_guard19_db13_slot: &mut f64,
        var_guard19_db14_slot: &mut f64,
        var_guard19_db15_slot: &mut f64,
        var_guard19_db16_slot: &mut f64,
        var_guard19_db17_slot: &mut f64,
        var_guard19_db18_slot: &mut f64,
        var_guard19_db2_slot: &mut f64,
        var_guard19_db3_slot: &mut f64,
        var_guard19_db4_slot: &mut f64,
        var_guard19_db5_slot: &mut f64,
        var_guard19_db6_slot: &mut f64,
        var_guard19_db7_slot: &mut f64,
        var_guard19_db8_slot: &mut f64,
        var_guard19_db9_slot: &mut f64,
        var_guard19_dn0_slot: &mut f64,
        var_guard19_dn1_slot: &mut f64,
        var_guard19_dn10_slot: &mut f64,
        var_guard19_dn11_slot: &mut f64,
        var_guard19_dn12_slot: &mut f64,
        var_guard19_dn13_slot: &mut f64,
        var_guard19_dn14_slot: &mut f64,
        var_guard19_dn15_slot: &mut f64,
        var_guard19_dn16_slot: &mut f64,
        var_guard19_dn17_slot: &mut f64,
        var_guard19_dn18_slot: &mut f64,
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
        var_guard19_rdb10_slot: &mut f64,
        var_guard19_rdb11_slot: &mut f64,
        var_guard19_rdb12_slot: &mut f64,
        var_guard19_rdb13_slot: &mut f64,
        var_guard19_rdb14_slot: &mut f64,
        var_guard19_rdb15_slot: &mut f64,
        var_guard19_rdb16_slot: &mut f64,
        var_guard19_rdb17_slot: &mut f64,
        var_guard19_rdb18_slot: &mut f64,
        var_guard19_rdb2_slot: &mut f64,
        var_guard19_rdb3_slot: &mut f64,
        var_guard19_rdb4_slot: &mut f64,
        var_guard19_rdb5_slot: &mut f64,
        var_guard19_rdb6_slot: &mut f64,
        var_guard19_rdb7_slot: &mut f64,
        var_guard19_rdb8_slot: &mut f64,
        var_guard19_rdb9_slot: &mut f64,
        var_guard19_rdn0_slot: &mut f64,
        var_guard19_rdn1_slot: &mut f64,
        var_guard19_rdn10_slot: &mut f64,
        var_guard19_rdn11_slot: &mut f64,
        var_guard19_rdn12_slot: &mut f64,
        var_guard19_rdn13_slot: &mut f64,
        var_guard19_rdn14_slot: &mut f64,
        var_guard19_rdn15_slot: &mut f64,
        var_guard19_rdn16_slot: &mut f64,
        var_guard19_rdn17_slot: &mut f64,
        var_guard19_rdn18_slot: &mut f64,
        var_guard19_rdn2_slot: &mut f64,
        var_guard19_rdn3_slot: &mut f64,
        var_guard19_rdn4_slot: &mut f64,
        var_guard19_rdn5_slot: &mut f64,
        var_guard19_rdn6_slot: &mut f64,
        var_guard19_rdn7_slot: &mut f64,
        var_guard19_rdn8_slot: &mut f64,
        var_guard19_rdn9_slot: &mut f64,
        var_guard19_rv_slot: &mut f64,
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
        var_qgd0_dn16_slot: &mut f64,
        var_qgd0_dn17_slot: &mut f64,
        var_qgd0_dn18_slot: &mut f64,
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
        var_qgd0_rdn16_slot: &mut f64,
        var_qgd0_rdn17_slot: &mut f64,
        var_qgd0_rdn18_slot: &mut f64,
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
        var_qgd_dn16_slot: &mut f64,
        var_qgd_dn17_slot: &mut f64,
        var_qgd_dn18_slot: &mut f64,
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
        var_qgd_rdn16_slot: &mut f64,
        var_qgd_rdn17_slot: &mut f64,
        var_qgd_rdn18_slot: &mut f64,
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
        let mut var_cgd_dn16: f64 = *var_cgd_dn16_slot;
        let mut var_cgd_dn17: f64 = *var_cgd_dn17_slot;
        let mut var_cgd_dn18: f64 = *var_cgd_dn18_slot;
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
        let mut var_cgd_rdn16: f64 = *var_cgd_rdn16_slot;
        let mut var_cgd_rdn17: f64 = *var_cgd_rdn17_slot;
        let mut var_cgd_rdn18: f64 = *var_cgd_rdn18_slot;
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
        let mut var_cgs_dn16: f64 = *var_cgs_dn16_slot;
        let mut var_cgs_dn17: f64 = *var_cgs_dn17_slot;
        let mut var_cgs_dn18: f64 = *var_cgs_dn18_slot;
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
        let mut var_cgs_rdn16: f64 = *var_cgs_rdn16_slot;
        let mut var_cgs_rdn17: f64 = *var_cgs_rdn17_slot;
        let mut var_cgs_rdn18: f64 = *var_cgs_rdn18_slot;
        let mut var_cgs_rdn2: f64 = *var_cgs_rdn2_slot;
        let mut var_cgs_rdn3: f64 = *var_cgs_rdn3_slot;
        let mut var_cgs_rdn4: f64 = *var_cgs_rdn4_slot;
        let mut var_cgs_rdn5: f64 = *var_cgs_rdn5_slot;
        let mut var_cgs_rdn6: f64 = *var_cgs_rdn6_slot;
        let mut var_cgs_rdn7: f64 = *var_cgs_rdn7_slot;
        let mut var_cgs_rdn8: f64 = *var_cgs_rdn8_slot;
        let mut var_cgs_rdn9: f64 = *var_cgs_rdn9_slot;
        let mut var_cgs_rv: f64 = *var_cgs_rv_slot;
        let mut var_guard19: f64 = *var_guard19_slot;
        let mut var_guard19_db0: f64 = *var_guard19_db0_slot;
        let mut var_guard19_db1: f64 = *var_guard19_db1_slot;
        let mut var_guard19_db10: f64 = *var_guard19_db10_slot;
        let mut var_guard19_db11: f64 = *var_guard19_db11_slot;
        let mut var_guard19_db12: f64 = *var_guard19_db12_slot;
        let mut var_guard19_db13: f64 = *var_guard19_db13_slot;
        let mut var_guard19_db14: f64 = *var_guard19_db14_slot;
        let mut var_guard19_db15: f64 = *var_guard19_db15_slot;
        let mut var_guard19_db16: f64 = *var_guard19_db16_slot;
        let mut var_guard19_db17: f64 = *var_guard19_db17_slot;
        let mut var_guard19_db18: f64 = *var_guard19_db18_slot;
        let mut var_guard19_db2: f64 = *var_guard19_db2_slot;
        let mut var_guard19_db3: f64 = *var_guard19_db3_slot;
        let mut var_guard19_db4: f64 = *var_guard19_db4_slot;
        let mut var_guard19_db5: f64 = *var_guard19_db5_slot;
        let mut var_guard19_db6: f64 = *var_guard19_db6_slot;
        let mut var_guard19_db7: f64 = *var_guard19_db7_slot;
        let mut var_guard19_db8: f64 = *var_guard19_db8_slot;
        let mut var_guard19_db9: f64 = *var_guard19_db9_slot;
        let mut var_guard19_dn0: f64 = *var_guard19_dn0_slot;
        let mut var_guard19_dn1: f64 = *var_guard19_dn1_slot;
        let mut var_guard19_dn10: f64 = *var_guard19_dn10_slot;
        let mut var_guard19_dn11: f64 = *var_guard19_dn11_slot;
        let mut var_guard19_dn12: f64 = *var_guard19_dn12_slot;
        let mut var_guard19_dn13: f64 = *var_guard19_dn13_slot;
        let mut var_guard19_dn14: f64 = *var_guard19_dn14_slot;
        let mut var_guard19_dn15: f64 = *var_guard19_dn15_slot;
        let mut var_guard19_dn16: f64 = *var_guard19_dn16_slot;
        let mut var_guard19_dn17: f64 = *var_guard19_dn17_slot;
        let mut var_guard19_dn18: f64 = *var_guard19_dn18_slot;
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
        let mut var_guard19_rdb10: f64 = *var_guard19_rdb10_slot;
        let mut var_guard19_rdb11: f64 = *var_guard19_rdb11_slot;
        let mut var_guard19_rdb12: f64 = *var_guard19_rdb12_slot;
        let mut var_guard19_rdb13: f64 = *var_guard19_rdb13_slot;
        let mut var_guard19_rdb14: f64 = *var_guard19_rdb14_slot;
        let mut var_guard19_rdb15: f64 = *var_guard19_rdb15_slot;
        let mut var_guard19_rdb16: f64 = *var_guard19_rdb16_slot;
        let mut var_guard19_rdb17: f64 = *var_guard19_rdb17_slot;
        let mut var_guard19_rdb18: f64 = *var_guard19_rdb18_slot;
        let mut var_guard19_rdb2: f64 = *var_guard19_rdb2_slot;
        let mut var_guard19_rdb3: f64 = *var_guard19_rdb3_slot;
        let mut var_guard19_rdb4: f64 = *var_guard19_rdb4_slot;
        let mut var_guard19_rdb5: f64 = *var_guard19_rdb5_slot;
        let mut var_guard19_rdb6: f64 = *var_guard19_rdb6_slot;
        let mut var_guard19_rdb7: f64 = *var_guard19_rdb7_slot;
        let mut var_guard19_rdb8: f64 = *var_guard19_rdb8_slot;
        let mut var_guard19_rdb9: f64 = *var_guard19_rdb9_slot;
        let mut var_guard19_rdn0: f64 = *var_guard19_rdn0_slot;
        let mut var_guard19_rdn1: f64 = *var_guard19_rdn1_slot;
        let mut var_guard19_rdn10: f64 = *var_guard19_rdn10_slot;
        let mut var_guard19_rdn11: f64 = *var_guard19_rdn11_slot;
        let mut var_guard19_rdn12: f64 = *var_guard19_rdn12_slot;
        let mut var_guard19_rdn13: f64 = *var_guard19_rdn13_slot;
        let mut var_guard19_rdn14: f64 = *var_guard19_rdn14_slot;
        let mut var_guard19_rdn15: f64 = *var_guard19_rdn15_slot;
        let mut var_guard19_rdn16: f64 = *var_guard19_rdn16_slot;
        let mut var_guard19_rdn17: f64 = *var_guard19_rdn17_slot;
        let mut var_guard19_rdn18: f64 = *var_guard19_rdn18_slot;
        let mut var_guard19_rdn2: f64 = *var_guard19_rdn2_slot;
        let mut var_guard19_rdn3: f64 = *var_guard19_rdn3_slot;
        let mut var_guard19_rdn4: f64 = *var_guard19_rdn4_slot;
        let mut var_guard19_rdn5: f64 = *var_guard19_rdn5_slot;
        let mut var_guard19_rdn6: f64 = *var_guard19_rdn6_slot;
        let mut var_guard19_rdn7: f64 = *var_guard19_rdn7_slot;
        let mut var_guard19_rdn8: f64 = *var_guard19_rdn8_slot;
        let mut var_guard19_rdn9: f64 = *var_guard19_rdn9_slot;
        let mut var_guard19_rv: f64 = *var_guard19_rv_slot;
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
        let mut var_qgd0_dn16: f64 = *var_qgd0_dn16_slot;
        let mut var_qgd0_dn17: f64 = *var_qgd0_dn17_slot;
        let mut var_qgd0_dn18: f64 = *var_qgd0_dn18_slot;
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
        let mut var_qgd0_rdn16: f64 = *var_qgd0_rdn16_slot;
        let mut var_qgd0_rdn17: f64 = *var_qgd0_rdn17_slot;
        let mut var_qgd0_rdn18: f64 = *var_qgd0_rdn18_slot;
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
        let mut var_qgd_dn16: f64 = *var_qgd_dn16_slot;
        let mut var_qgd_dn17: f64 = *var_qgd_dn17_slot;
        let mut var_qgd_dn18: f64 = *var_qgd_dn18_slot;
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
        let mut var_qgd_rdn16: f64 = *var_qgd_rdn16_slot;
        let mut var_qgd_rdn17: f64 = *var_qgd_rdn17_slot;
        let mut var_qgd_rdn18: f64 = *var_qgd_rdn18_slot;
        let mut var_qgd_rdn2: f64 = *var_qgd_rdn2_slot;
        let mut var_qgd_rdn3: f64 = *var_qgd_rdn3_slot;
        let mut var_qgd_rdn4: f64 = *var_qgd_rdn4_slot;
        let mut var_qgd_rdn5: f64 = *var_qgd_rdn5_slot;
        let mut var_qgd_rdn6: f64 = *var_qgd_rdn6_slot;
        let mut var_qgd_rdn7: f64 = *var_qgd_rdn7_slot;
        let mut var_qgd_rdn8: f64 = *var_qgd_rdn8_slot;
        let mut var_qgd_rdn9: f64 = *var_qgd_rdn9_slot;
        let mut var_qgd_rv: f64 = *var_qgd_rv_slot;

        let (assign2040_e2755, assign2040_e2755_d_n0, assign2040_e2755_d_n1, assign2040_e2755_d_n2, assign2040_e2755_d_n3, assign2040_e2755_d_n4, assign2040_e2755_d_n5, assign2040_e2755_d_n6, assign2040_e2755_d_n7, assign2040_e2755_d_n8, assign2040_e2755_d_n9, assign2040_e2755_d_n10, assign2040_e2755_d_n11, assign2040_e2755_d_n12, assign2040_e2755_d_n13, assign2040_e2755_d_n14, assign2040_e2755_d_n15, assign2040_e2755_d_n16, assign2040_e2755_d_n17, assign2040_e2755_d_n18, assign2040_e2755_d_b0, assign2040_e2755_d_b1, assign2040_e2755_d_b2, assign2040_e2755_d_b3, assign2040_e2755_d_b4, assign2040_e2755_d_b5, assign2040_e2755_d_b6, assign2040_e2755_d_b7, assign2040_e2755_d_b8, assign2040_e2755_d_b9, assign2040_e2755_d_b10, assign2040_e2755_d_b11, assign2040_e2755_d_b12, assign2040_e2755_d_b13, assign2040_e2755_d_b14, assign2040_e2755_d_b15, assign2040_e2755_d_b16, assign2040_e2755_d_b17, assign2040_e2755_d_b18,) = {
    if ((var_guard18 != 0.0) && (!((((var_guard14 != 0.0) || (var_guard15 != 0.0)) || (var_guard16 != 0.0)) || (var_guard17 != 0.0)))) {
        let assign2040_e2750: f64 = (p.p38 * var_vds);
        let assign2040_e2751: f64 = (var_p40_t - assign2040_e2750);
        let assign2040_e2753: f64 = (assign2040_e2751 + var_lc40);
        (assign2040_e2753, ((var_p40_t_dn0 - (p.p38 * var_vds_dn0)) + var_lc40_dn0), ((var_p40_t_dn1 - (p.p38 * var_vds_dn1)) + var_lc40_dn1), ((var_p40_t_dn2 - (p.p38 * var_vds_dn2)) + var_lc40_dn2), ((var_p40_t_dn3 - (p.p38 * var_vds_dn3)) + var_lc40_dn3), ((var_p40_t_dn4 - (p.p38 * var_vds_dn4)) + var_lc40_dn4), ((var_p40_t_dn5 - (p.p38 * var_vds_dn5)) + var_lc40_dn5), ((var_p40_t_dn6 - (p.p38 * var_vds_dn6)) + var_lc40_dn6), ((var_p40_t_dn7 - (p.p38 * var_vds_dn7)) + var_lc40_dn7), ((var_p40_t_dn8 - (p.p38 * var_vds_dn8)) + var_lc40_dn8), ((var_p40_t_dn9 - (p.p38 * var_vds_dn9)) + var_lc40_dn9), ((var_p40_t_dn10 - (p.p38 * var_vds_dn10)) + var_lc40_dn10), ((var_p40_t_dn11 - (p.p38 * var_vds_dn11)) + var_lc40_dn11), ((var_p40_t_dn12 - (p.p38 * var_vds_dn12)) + var_lc40_dn12), ((var_p40_t_dn13 - (p.p38 * var_vds_dn13)) + var_lc40_dn13), ((var_p40_t_dn14 - (p.p38 * var_vds_dn14)) + var_lc40_dn14), ((var_p40_t_dn15 - (p.p38 * var_vds_dn15)) + var_lc40_dn15), ((var_p40_t_dn16 - (p.p38 * var_vds_dn16)) + var_lc40_dn16), ((var_p40_t_dn17 - (p.p38 * var_vds_dn17)) + var_lc40_dn17), ((var_p40_t_dn18 - (p.p38 * var_vds_dn18)) + var_lc40_dn18), ((var_p40_t_db0 - (p.p38 * var_vds_db0)) + var_lc40_db0), ((var_p40_t_db1 - (p.p38 * var_vds_db1)) + var_lc40_db1), ((var_p40_t_db2 - (p.p38 * var_vds_db2)) + var_lc40_db2), ((var_p40_t_db3 - (p.p38 * var_vds_db3)) + var_lc40_db3), ((var_p40_t_db4 - (p.p38 * var_vds_db4)) + var_lc40_db4), ((var_p40_t_db5 - (p.p38 * var_vds_db5)) + var_lc40_db5), ((var_p40_t_db6 - (p.p38 * var_vds_db6)) + var_lc40_db6), ((var_p40_t_db7 - (p.p38 * var_vds_db7)) + var_lc40_db7), ((var_p40_t_db8 - (p.p38 * var_vds_db8)) + var_lc40_db8), ((var_p40_t_db9 - (p.p38 * var_vds_db9)) + var_lc40_db9), ((var_p40_t_db10 - (p.p38 * var_vds_db10)) + var_lc40_db10), ((var_p40_t_db11 - (p.p38 * var_vds_db11)) + var_lc40_db11), ((var_p40_t_db12 - (p.p38 * var_vds_db12)) + var_lc40_db12), ((var_p40_t_db13 - (p.p38 * var_vds_db13)) + var_lc40_db13), ((var_p40_t_db14 - (p.p38 * var_vds_db14)) + var_lc40_db14), ((var_p40_t_db15 - (p.p38 * var_vds_db15)) + var_lc40_db15), ((var_p40_t_db16 - (p.p38 * var_vds_db16)) + var_lc40_db16), ((var_p40_t_db17 - (p.p38 * var_vds_db17)) + var_lc40_db17), ((var_p40_t_db18 - (p.p38 * var_vds_db18)) + var_lc40_db18),)
    } else {
        (var_qgd0, var_qgd0_dn0, var_qgd0_dn1, var_qgd0_dn2, var_qgd0_dn3, var_qgd0_dn4, var_qgd0_dn5, var_qgd0_dn6, var_qgd0_dn7, var_qgd0_dn8, var_qgd0_dn9, var_qgd0_dn10, var_qgd0_dn11, var_qgd0_dn12, var_qgd0_dn13, var_qgd0_dn14, var_qgd0_dn15, var_qgd0_dn16, var_qgd0_dn17, var_qgd0_dn18, var_qgd0_db0, var_qgd0_db1, var_qgd0_db2, var_qgd0_db3, var_qgd0_db4, var_qgd0_db5, var_qgd0_db6, var_qgd0_db7, var_qgd0_db8, var_qgd0_db9, var_qgd0_db10, var_qgd0_db11, var_qgd0_db12, var_qgd0_db13, var_qgd0_db14, var_qgd0_db15, var_qgd0_db16, var_qgd0_db17, var_qgd0_db18,)
    }
};
        var_qgd0 = assign2040_e2755;
        var_qgd0_dn0 = assign2040_e2755_d_n0;
        var_qgd0_dn1 = assign2040_e2755_d_n1;
        var_qgd0_dn2 = assign2040_e2755_d_n2;
        var_qgd0_dn3 = assign2040_e2755_d_n3;
        var_qgd0_dn4 = assign2040_e2755_d_n4;
        var_qgd0_dn5 = assign2040_e2755_d_n5;
        var_qgd0_dn6 = assign2040_e2755_d_n6;
        var_qgd0_dn7 = assign2040_e2755_d_n7;
        var_qgd0_dn8 = assign2040_e2755_d_n8;
        var_qgd0_dn9 = assign2040_e2755_d_n9;
        var_qgd0_dn10 = assign2040_e2755_d_n10;
        var_qgd0_dn11 = assign2040_e2755_d_n11;
        var_qgd0_dn12 = assign2040_e2755_d_n12;
        var_qgd0_dn13 = assign2040_e2755_d_n13;
        var_qgd0_dn14 = assign2040_e2755_d_n14;
        var_qgd0_dn15 = assign2040_e2755_d_n15;
        var_qgd0_dn16 = assign2040_e2755_d_n16;
        var_qgd0_dn17 = assign2040_e2755_d_n17;
        var_qgd0_dn18 = assign2040_e2755_d_n18;
        var_qgd0_db0 = assign2040_e2755_d_b0;
        var_qgd0_db1 = assign2040_e2755_d_b1;
        var_qgd0_db2 = assign2040_e2755_d_b2;
        var_qgd0_db3 = assign2040_e2755_d_b3;
        var_qgd0_db4 = assign2040_e2755_d_b4;
        var_qgd0_db5 = assign2040_e2755_d_b5;
        var_qgd0_db6 = assign2040_e2755_d_b6;
        var_qgd0_db7 = assign2040_e2755_d_b7;
        var_qgd0_db8 = assign2040_e2755_d_b8;
        var_qgd0_db9 = assign2040_e2755_d_b9;
        var_qgd0_db10 = assign2040_e2755_d_b10;
        var_qgd0_db11 = assign2040_e2755_d_b11;
        var_qgd0_db12 = assign2040_e2755_d_b12;
        var_qgd0_db13 = assign2040_e2755_d_b13;
        var_qgd0_db14 = assign2040_e2755_d_b14;
        var_qgd0_db15 = assign2040_e2755_d_b15;
        var_qgd0_db16 = assign2040_e2755_d_b16;
        var_qgd0_db17 = assign2040_e2755_d_b17;
        var_qgd0_db18 = assign2040_e2755_d_b18;
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
        var_qgd0_rdn16 = 0.0;
        var_qgd0_rdn17 = 0.0;
        var_qgd0_rdn18 = 0.0;
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

        let (assign2050_e2793, assign2050_e2793_d_n0, assign2050_e2793_d_n1, assign2050_e2793_d_n2, assign2050_e2793_d_n3, assign2050_e2793_d_n4, assign2050_e2793_d_n5, assign2050_e2793_d_n6, assign2050_e2793_d_n7, assign2050_e2793_d_n8, assign2050_e2793_d_n9, assign2050_e2793_d_n10, assign2050_e2793_d_n11, assign2050_e2793_d_n12, assign2050_e2793_d_n13, assign2050_e2793_d_n14, assign2050_e2793_d_n15, assign2050_e2793_d_n16, assign2050_e2793_d_n17, assign2050_e2793_d_n18, assign2050_e2793_d_b0, assign2050_e2793_d_b1, assign2050_e2793_d_b2, assign2050_e2793_d_b3, assign2050_e2793_d_b4, assign2050_e2793_d_b5, assign2050_e2793_d_b6, assign2050_e2793_d_b7, assign2050_e2793_d_b8, assign2050_e2793_d_b9, assign2050_e2793_d_b10, assign2050_e2793_d_b11, assign2050_e2793_d_b12, assign2050_e2793_d_b13, assign2050_e2793_d_b14, assign2050_e2793_d_b15, assign2050_e2793_d_b16, assign2050_e2793_d_b17, assign2050_e2793_d_b18,) = {
    if ((var_guard18 != 0.0) && (!((((var_guard14 != 0.0) || (var_guard15 != 0.0)) || (var_guard16 != 0.0)) || (var_guard17 != 0.0)))) {
        let assign2050_e2769: f64 = (var_psi_4 + var_lc4);
        let assign2050_e2771: f64 = (assign2050_e2769 - var_qgd0);
        let assign2050_e2774: f64 = (1.0 - p.p38);
        let assign2050_e2776: f64 = (var_psi_3).tanh();
        let assign2050_e2777: f64 = (assign2050_e2774 + assign2050_e2776);
        let assign2050_e2778: f64 = (assign2050_e2771 * assign2050_e2777);
        let assign2050_e2780: f64 = (assign2050_e2778 / p.p37);
        let assign2050_e2783: f64 = (2.0 * p.p38);
        let assign2050_e2785: f64 = (assign2050_e2783 * var_vgdc);
        let assign2050_e2786: f64 = (assign2050_e2780 + assign2050_e2785);
        let assign2050_e2787: f64 = (var_cgd0_t * assign2050_e2786);
        let assign2050_e2790: f64 = (p.p27 * var_vgdc);
        let assign2050_e2791: f64 = (assign2050_e2787 + assign2050_e2790);
        (assign2050_e2791, (((var_cgd0_t_dn0 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn0 + var_lc4_dn0) - var_qgd0_dn0) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn0 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn0)))) + (p.p27 * var_vgdc_dn0)), (((var_cgd0_t_dn1 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn1 + var_lc4_dn1) - var_qgd0_dn1) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn1 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn1)))) + (p.p27 * var_vgdc_dn1)), (((var_cgd0_t_dn2 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn2 + var_lc4_dn2) - var_qgd0_dn2) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn2 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn2)))) + (p.p27 * var_vgdc_dn2)), (((var_cgd0_t_dn3 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn3 + var_lc4_dn3) - var_qgd0_dn3) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn3 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn3)))) + (p.p27 * var_vgdc_dn3)), (((var_cgd0_t_dn4 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn4 + var_lc4_dn4) - var_qgd0_dn4) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn4 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn4)))) + (p.p27 * var_vgdc_dn4)), (((var_cgd0_t_dn5 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn5 + var_lc4_dn5) - var_qgd0_dn5) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn5 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn5)))) + (p.p27 * var_vgdc_dn5)), (((var_cgd0_t_dn6 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn6 + var_lc4_dn6) - var_qgd0_dn6) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn6 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn6)))) + (p.p27 * var_vgdc_dn6)), (((var_cgd0_t_dn7 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn7 + var_lc4_dn7) - var_qgd0_dn7) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn7 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn7)))) + (p.p27 * var_vgdc_dn7)), (((var_cgd0_t_dn8 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn8 + var_lc4_dn8) - var_qgd0_dn8) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn8 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn8)))) + (p.p27 * var_vgdc_dn8)), (((var_cgd0_t_dn9 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn9 + var_lc4_dn9) - var_qgd0_dn9) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn9 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn9)))) + (p.p27 * var_vgdc_dn9)), (((var_cgd0_t_dn10 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn10 + var_lc4_dn10) - var_qgd0_dn10) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn10 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn10)))) + (p.p27 * var_vgdc_dn10)), (((var_cgd0_t_dn11 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn11 + var_lc4_dn11) - var_qgd0_dn11) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn11 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn11)))) + (p.p27 * var_vgdc_dn11)), (((var_cgd0_t_dn12 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn12 + var_lc4_dn12) - var_qgd0_dn12) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn12 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn12)))) + (p.p27 * var_vgdc_dn12)), (((var_cgd0_t_dn13 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn13 + var_lc4_dn13) - var_qgd0_dn13) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn13 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn13)))) + (p.p27 * var_vgdc_dn13)), (((var_cgd0_t_dn14 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn14 + var_lc4_dn14) - var_qgd0_dn14) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn14 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn14)))) + (p.p27 * var_vgdc_dn14)), (((var_cgd0_t_dn15 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn15 + var_lc4_dn15) - var_qgd0_dn15) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn15 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn15)))) + (p.p27 * var_vgdc_dn15)), (((var_cgd0_t_dn16 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn16 + var_lc4_dn16) - var_qgd0_dn16) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn16 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn16)))) + (p.p27 * var_vgdc_dn16)), (((var_cgd0_t_dn17 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn17 + var_lc4_dn17) - var_qgd0_dn17) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn17 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn17)))) + (p.p27 * var_vgdc_dn17)), (((var_cgd0_t_dn18 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_dn18 + var_lc4_dn18) - var_qgd0_dn18) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_dn18 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_dn18)))) + (p.p27 * var_vgdc_dn18)), (((var_cgd0_t_db0 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db0 + var_lc4_db0) - var_qgd0_db0) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db0 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db0)))) + (p.p27 * var_vgdc_db0)), (((var_cgd0_t_db1 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db1 + var_lc4_db1) - var_qgd0_db1) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db1 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db1)))) + (p.p27 * var_vgdc_db1)), (((var_cgd0_t_db2 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db2 + var_lc4_db2) - var_qgd0_db2) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db2 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db2)))) + (p.p27 * var_vgdc_db2)), (((var_cgd0_t_db3 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db3 + var_lc4_db3) - var_qgd0_db3) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db3 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db3)))) + (p.p27 * var_vgdc_db3)), (((var_cgd0_t_db4 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db4 + var_lc4_db4) - var_qgd0_db4) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db4 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db4)))) + (p.p27 * var_vgdc_db4)), (((var_cgd0_t_db5 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db5 + var_lc4_db5) - var_qgd0_db5) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db5 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db5)))) + (p.p27 * var_vgdc_db5)), (((var_cgd0_t_db6 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db6 + var_lc4_db6) - var_qgd0_db6) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db6 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db6)))) + (p.p27 * var_vgdc_db6)), (((var_cgd0_t_db7 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db7 + var_lc4_db7) - var_qgd0_db7) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db7 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db7)))) + (p.p27 * var_vgdc_db7)), (((var_cgd0_t_db8 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db8 + var_lc4_db8) - var_qgd0_db8) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db8 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db8)))) + (p.p27 * var_vgdc_db8)), (((var_cgd0_t_db9 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db9 + var_lc4_db9) - var_qgd0_db9) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db9 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db9)))) + (p.p27 * var_vgdc_db9)), (((var_cgd0_t_db10 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db10 + var_lc4_db10) - var_qgd0_db10) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db10 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db10)))) + (p.p27 * var_vgdc_db10)), (((var_cgd0_t_db11 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db11 + var_lc4_db11) - var_qgd0_db11) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db11 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db11)))) + (p.p27 * var_vgdc_db11)), (((var_cgd0_t_db12 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db12 + var_lc4_db12) - var_qgd0_db12) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db12 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db12)))) + (p.p27 * var_vgdc_db12)), (((var_cgd0_t_db13 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db13 + var_lc4_db13) - var_qgd0_db13) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db13 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db13)))) + (p.p27 * var_vgdc_db13)), (((var_cgd0_t_db14 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db14 + var_lc4_db14) - var_qgd0_db14) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db14 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db14)))) + (p.p27 * var_vgdc_db14)), (((var_cgd0_t_db15 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db15 + var_lc4_db15) - var_qgd0_db15) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db15 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db15)))) + (p.p27 * var_vgdc_db15)), (((var_cgd0_t_db16 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db16 + var_lc4_db16) - var_qgd0_db16) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db16 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db16)))) + (p.p27 * var_vgdc_db16)), (((var_cgd0_t_db17 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db17 + var_lc4_db17) - var_qgd0_db17) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db17 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db17)))) + (p.p27 * var_vgdc_db17)), (((var_cgd0_t_db18 * assign2050_e2786) + (var_cgd0_t * ((((((var_psi_4_db18 + var_lc4_db18) - var_qgd0_db18) * assign2050_e2777) + (assign2050_e2771 * (var_psi_3_db18 / ((var_psi_3).cosh() * (var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * var_vgdc_db18)))) + (p.p27 * var_vgdc_db18)),)
    } else {
        (var_qgd, var_qgd_dn0, var_qgd_dn1, var_qgd_dn2, var_qgd_dn3, var_qgd_dn4, var_qgd_dn5, var_qgd_dn6, var_qgd_dn7, var_qgd_dn8, var_qgd_dn9, var_qgd_dn10, var_qgd_dn11, var_qgd_dn12, var_qgd_dn13, var_qgd_dn14, var_qgd_dn15, var_qgd_dn16, var_qgd_dn17, var_qgd_dn18, var_qgd_db0, var_qgd_db1, var_qgd_db2, var_qgd_db3, var_qgd_db4, var_qgd_db5, var_qgd_db6, var_qgd_db7, var_qgd_db8, var_qgd_db9, var_qgd_db10, var_qgd_db11, var_qgd_db12, var_qgd_db13, var_qgd_db14, var_qgd_db15, var_qgd_db16, var_qgd_db17, var_qgd_db18,)
    }
};
        var_qgd = assign2050_e2793;
        var_qgd_dn0 = assign2050_e2793_d_n0;
        var_qgd_dn1 = assign2050_e2793_d_n1;
        var_qgd_dn2 = assign2050_e2793_d_n2;
        var_qgd_dn3 = assign2050_e2793_d_n3;
        var_qgd_dn4 = assign2050_e2793_d_n4;
        var_qgd_dn5 = assign2050_e2793_d_n5;
        var_qgd_dn6 = assign2050_e2793_d_n6;
        var_qgd_dn7 = assign2050_e2793_d_n7;
        var_qgd_dn8 = assign2050_e2793_d_n8;
        var_qgd_dn9 = assign2050_e2793_d_n9;
        var_qgd_dn10 = assign2050_e2793_d_n10;
        var_qgd_dn11 = assign2050_e2793_d_n11;
        var_qgd_dn12 = assign2050_e2793_d_n12;
        var_qgd_dn13 = assign2050_e2793_d_n13;
        var_qgd_dn14 = assign2050_e2793_d_n14;
        var_qgd_dn15 = assign2050_e2793_d_n15;
        var_qgd_dn16 = assign2050_e2793_d_n16;
        var_qgd_dn17 = assign2050_e2793_d_n17;
        var_qgd_dn18 = assign2050_e2793_d_n18;
        var_qgd_db0 = assign2050_e2793_d_b0;
        var_qgd_db1 = assign2050_e2793_d_b1;
        var_qgd_db2 = assign2050_e2793_d_b2;
        var_qgd_db3 = assign2050_e2793_d_b3;
        var_qgd_db4 = assign2050_e2793_d_b4;
        var_qgd_db5 = assign2050_e2793_d_b5;
        var_qgd_db6 = assign2050_e2793_d_b6;
        var_qgd_db7 = assign2050_e2793_d_b7;
        var_qgd_db8 = assign2050_e2793_d_b8;
        var_qgd_db9 = assign2050_e2793_d_b9;
        var_qgd_db10 = assign2050_e2793_d_b10;
        var_qgd_db11 = assign2050_e2793_d_b11;
        var_qgd_db12 = assign2050_e2793_d_b12;
        var_qgd_db13 = assign2050_e2793_d_b13;
        var_qgd_db14 = assign2050_e2793_d_b14;
        var_qgd_db15 = assign2050_e2793_d_b15;
        var_qgd_db16 = assign2050_e2793_d_b16;
        var_qgd_db17 = assign2050_e2793_d_b17;
        var_qgd_db18 = assign2050_e2793_d_b18;
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
        var_qgd_rdn16 = 0.0;
        var_qgd_rdn17 = 0.0;
        var_qgd_rdn18 = 0.0;
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

        let (assign2060_e2808, assign2060_e2808_d_n0, assign2060_e2808_d_n1, assign2060_e2808_d_n2, assign2060_e2808_d_n3, assign2060_e2808_d_n4, assign2060_e2808_d_n5, assign2060_e2808_d_n6, assign2060_e2808_d_n7, assign2060_e2808_d_n8, assign2060_e2808_d_n9, assign2060_e2808_d_n10, assign2060_e2808_d_n11, assign2060_e2808_d_n12, assign2060_e2808_d_n13, assign2060_e2808_d_n14, assign2060_e2808_d_n15, assign2060_e2808_d_n16, assign2060_e2808_d_n17, assign2060_e2808_d_n18, assign2060_e2808_d_b0, assign2060_e2808_d_b1, assign2060_e2808_d_b2, assign2060_e2808_d_b3, assign2060_e2808_d_b4, assign2060_e2808_d_b5, assign2060_e2808_d_b6, assign2060_e2808_d_b7, assign2060_e2808_d_b8, assign2060_e2808_d_b9, assign2060_e2808_d_b10, assign2060_e2808_d_b11, assign2060_e2808_d_b12, assign2060_e2808_d_b13, assign2060_e2808_d_b14, assign2060_e2808_d_b15, assign2060_e2808_d_b16, assign2060_e2808_d_b17, assign2060_e2808_d_b18,) = {
    if ((var_guard18 != 0.0) && (!((((var_guard14 != 0.0) || (var_guard15 != 0.0)) || (var_guard16 != 0.0)) || (var_guard17 != 0.0)))) {
        let assign2060_e2806: f64 = var_qgs_dn11;
        (assign2060_e2806, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgs, var_cgs_dn0, var_cgs_dn1, var_cgs_dn2, var_cgs_dn3, var_cgs_dn4, var_cgs_dn5, var_cgs_dn6, var_cgs_dn7, var_cgs_dn8, var_cgs_dn9, var_cgs_dn10, var_cgs_dn11, var_cgs_dn12, var_cgs_dn13, var_cgs_dn14, var_cgs_dn15, var_cgs_dn16, var_cgs_dn17, var_cgs_dn18, var_cgs_db0, var_cgs_db1, var_cgs_db2, var_cgs_db3, var_cgs_db4, var_cgs_db5, var_cgs_db6, var_cgs_db7, var_cgs_db8, var_cgs_db9, var_cgs_db10, var_cgs_db11, var_cgs_db12, var_cgs_db13, var_cgs_db14, var_cgs_db15, var_cgs_db16, var_cgs_db17, var_cgs_db18,)
    }
};
        var_cgs = assign2060_e2808;
        var_cgs_dn0 = assign2060_e2808_d_n0;
        var_cgs_dn1 = assign2060_e2808_d_n1;
        var_cgs_dn2 = assign2060_e2808_d_n2;
        var_cgs_dn3 = assign2060_e2808_d_n3;
        var_cgs_dn4 = assign2060_e2808_d_n4;
        var_cgs_dn5 = assign2060_e2808_d_n5;
        var_cgs_dn6 = assign2060_e2808_d_n6;
        var_cgs_dn7 = assign2060_e2808_d_n7;
        var_cgs_dn8 = assign2060_e2808_d_n8;
        var_cgs_dn9 = assign2060_e2808_d_n9;
        var_cgs_dn10 = assign2060_e2808_d_n10;
        var_cgs_dn11 = assign2060_e2808_d_n11;
        var_cgs_dn12 = assign2060_e2808_d_n12;
        var_cgs_dn13 = assign2060_e2808_d_n13;
        var_cgs_dn14 = assign2060_e2808_d_n14;
        var_cgs_dn15 = assign2060_e2808_d_n15;
        var_cgs_dn16 = assign2060_e2808_d_n16;
        var_cgs_dn17 = assign2060_e2808_d_n17;
        var_cgs_dn18 = assign2060_e2808_d_n18;
        var_cgs_db0 = assign2060_e2808_d_b0;
        var_cgs_db1 = assign2060_e2808_d_b1;
        var_cgs_db2 = assign2060_e2808_d_b2;
        var_cgs_db3 = assign2060_e2808_d_b3;
        var_cgs_db4 = assign2060_e2808_d_b4;
        var_cgs_db5 = assign2060_e2808_d_b5;
        var_cgs_db6 = assign2060_e2808_d_b6;
        var_cgs_db7 = assign2060_e2808_d_b7;
        var_cgs_db8 = assign2060_e2808_d_b8;
        var_cgs_db9 = assign2060_e2808_d_b9;
        var_cgs_db10 = assign2060_e2808_d_b10;
        var_cgs_db11 = assign2060_e2808_d_b11;
        var_cgs_db12 = assign2060_e2808_d_b12;
        var_cgs_db13 = assign2060_e2808_d_b13;
        var_cgs_db14 = assign2060_e2808_d_b14;
        var_cgs_db15 = assign2060_e2808_d_b15;
        var_cgs_db16 = assign2060_e2808_d_b16;
        var_cgs_db17 = assign2060_e2808_d_b17;
        var_cgs_db18 = assign2060_e2808_d_b18;
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
        var_cgs_rdn16 = 0.0;
        var_cgs_rdn17 = 0.0;
        var_cgs_rdn18 = 0.0;
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

        let (assign2070_e2823, assign2070_e2823_d_n0, assign2070_e2823_d_n1, assign2070_e2823_d_n2, assign2070_e2823_d_n3, assign2070_e2823_d_n4, assign2070_e2823_d_n5, assign2070_e2823_d_n6, assign2070_e2823_d_n7, assign2070_e2823_d_n8, assign2070_e2823_d_n9, assign2070_e2823_d_n10, assign2070_e2823_d_n11, assign2070_e2823_d_n12, assign2070_e2823_d_n13, assign2070_e2823_d_n14, assign2070_e2823_d_n15, assign2070_e2823_d_n16, assign2070_e2823_d_n17, assign2070_e2823_d_n18, assign2070_e2823_d_b0, assign2070_e2823_d_b1, assign2070_e2823_d_b2, assign2070_e2823_d_b3, assign2070_e2823_d_b4, assign2070_e2823_d_b5, assign2070_e2823_d_b6, assign2070_e2823_d_b7, assign2070_e2823_d_b8, assign2070_e2823_d_b9, assign2070_e2823_d_b10, assign2070_e2823_d_b11, assign2070_e2823_d_b12, assign2070_e2823_d_b13, assign2070_e2823_d_b14, assign2070_e2823_d_b15, assign2070_e2823_d_b16, assign2070_e2823_d_b17, assign2070_e2823_d_b18,) = {
    if ((var_guard18 != 0.0) && (!((((var_guard14 != 0.0) || (var_guard15 != 0.0)) || (var_guard16 != 0.0)) || (var_guard17 != 0.0)))) {
        let assign2070_e2821: f64 = var_qgd_dn10;
        (assign2070_e2821, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgd, var_cgd_dn0, var_cgd_dn1, var_cgd_dn2, var_cgd_dn3, var_cgd_dn4, var_cgd_dn5, var_cgd_dn6, var_cgd_dn7, var_cgd_dn8, var_cgd_dn9, var_cgd_dn10, var_cgd_dn11, var_cgd_dn12, var_cgd_dn13, var_cgd_dn14, var_cgd_dn15, var_cgd_dn16, var_cgd_dn17, var_cgd_dn18, var_cgd_db0, var_cgd_db1, var_cgd_db2, var_cgd_db3, var_cgd_db4, var_cgd_db5, var_cgd_db6, var_cgd_db7, var_cgd_db8, var_cgd_db9, var_cgd_db10, var_cgd_db11, var_cgd_db12, var_cgd_db13, var_cgd_db14, var_cgd_db15, var_cgd_db16, var_cgd_db17, var_cgd_db18,)
    }
};
        var_cgd = assign2070_e2823;
        var_cgd_dn0 = assign2070_e2823_d_n0;
        var_cgd_dn1 = assign2070_e2823_d_n1;
        var_cgd_dn2 = assign2070_e2823_d_n2;
        var_cgd_dn3 = assign2070_e2823_d_n3;
        var_cgd_dn4 = assign2070_e2823_d_n4;
        var_cgd_dn5 = assign2070_e2823_d_n5;
        var_cgd_dn6 = assign2070_e2823_d_n6;
        var_cgd_dn7 = assign2070_e2823_d_n7;
        var_cgd_dn8 = assign2070_e2823_d_n8;
        var_cgd_dn9 = assign2070_e2823_d_n9;
        var_cgd_dn10 = assign2070_e2823_d_n10;
        var_cgd_dn11 = assign2070_e2823_d_n11;
        var_cgd_dn12 = assign2070_e2823_d_n12;
        var_cgd_dn13 = assign2070_e2823_d_n13;
        var_cgd_dn14 = assign2070_e2823_d_n14;
        var_cgd_dn15 = assign2070_e2823_d_n15;
        var_cgd_dn16 = assign2070_e2823_d_n16;
        var_cgd_dn17 = assign2070_e2823_d_n17;
        var_cgd_dn18 = assign2070_e2823_d_n18;
        var_cgd_db0 = assign2070_e2823_d_b0;
        var_cgd_db1 = assign2070_e2823_d_b1;
        var_cgd_db2 = assign2070_e2823_d_b2;
        var_cgd_db3 = assign2070_e2823_d_b3;
        var_cgd_db4 = assign2070_e2823_d_b4;
        var_cgd_db5 = assign2070_e2823_d_b5;
        var_cgd_db6 = assign2070_e2823_d_b6;
        var_cgd_db7 = assign2070_e2823_d_b7;
        var_cgd_db8 = assign2070_e2823_d_b8;
        var_cgd_db9 = assign2070_e2823_d_b9;
        var_cgd_db10 = assign2070_e2823_d_b10;
        var_cgd_db11 = assign2070_e2823_d_b11;
        var_cgd_db12 = assign2070_e2823_d_b12;
        var_cgd_db13 = assign2070_e2823_d_b13;
        var_cgd_db14 = assign2070_e2823_d_b14;
        var_cgd_db15 = assign2070_e2823_d_b15;
        var_cgd_db16 = assign2070_e2823_d_b16;
        var_cgd_db17 = assign2070_e2823_d_b17;
        var_cgd_db18 = assign2070_e2823_d_b18;
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
        var_cgd_rdn16 = 0.0;
        var_cgd_rdn17 = 0.0;
        var_cgd_rdn18 = 0.0;
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

        let assign2080_e2830: f64 = if ((p.p6 == 2.0) || (p.p6 == 4.0)) { 1.0 } else { 0.0 };
        var_guard19 = assign2080_e2830;
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
        var_guard19_dn10 = 0.0;
        var_guard19_dn11 = 0.0;
        var_guard19_dn12 = 0.0;
        var_guard19_dn13 = 0.0;
        var_guard19_dn14 = 0.0;
        var_guard19_dn15 = 0.0;
        var_guard19_dn16 = 0.0;
        var_guard19_dn17 = 0.0;
        var_guard19_dn18 = 0.0;
        var_guard19_db0 = 0.0;
        var_guard19_db1 = 0.0;
        var_guard19_db2 = 0.0;
        var_guard19_db3 = 0.0;
        var_guard19_db4 = 0.0;
        var_guard19_db5 = 0.0;
        var_guard19_db6 = 0.0;
        var_guard19_db7 = 0.0;
        var_guard19_db8 = 0.0;
        var_guard19_db9 = 0.0;
        var_guard19_db10 = 0.0;
        var_guard19_db11 = 0.0;
        var_guard19_db12 = 0.0;
        var_guard19_db13 = 0.0;
        var_guard19_db14 = 0.0;
        var_guard19_db15 = 0.0;
        var_guard19_db16 = 0.0;
        var_guard19_db17 = 0.0;
        var_guard19_db18 = 0.0;
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
        var_guard19_rdn10 = 0.0;
        var_guard19_rdn11 = 0.0;
        var_guard19_rdn12 = 0.0;
        var_guard19_rdn13 = 0.0;
        var_guard19_rdn14 = 0.0;
        var_guard19_rdn15 = 0.0;
        var_guard19_rdn16 = 0.0;
        var_guard19_rdn17 = 0.0;
        var_guard19_rdn18 = 0.0;
        var_guard19_rdb0 = 0.0;
        var_guard19_rdb1 = 0.0;
        var_guard19_rdb2 = 0.0;
        var_guard19_rdb3 = 0.0;
        var_guard19_rdb4 = 0.0;
        var_guard19_rdb5 = 0.0;
        var_guard19_rdb6 = 0.0;
        var_guard19_rdb7 = 0.0;
        var_guard19_rdb8 = 0.0;
        var_guard19_rdb9 = 0.0;
        var_guard19_rdb10 = 0.0;
        var_guard19_rdb11 = 0.0;
        var_guard19_rdb12 = 0.0;
        var_guard19_rdb13 = 0.0;
        var_guard19_rdb14 = 0.0;
        var_guard19_rdb15 = 0.0;
        var_guard19_rdb16 = 0.0;
        var_guard19_rdb17 = 0.0;
        var_guard19_rdb18 = 0.0;


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
        *var_cgd_dn16_slot = var_cgd_dn16;
        *var_cgd_dn17_slot = var_cgd_dn17;
        *var_cgd_dn18_slot = var_cgd_dn18;
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
        *var_cgd_rdn16_slot = var_cgd_rdn16;
        *var_cgd_rdn17_slot = var_cgd_rdn17;
        *var_cgd_rdn18_slot = var_cgd_rdn18;
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
        *var_cgs_dn16_slot = var_cgs_dn16;
        *var_cgs_dn17_slot = var_cgs_dn17;
        *var_cgs_dn18_slot = var_cgs_dn18;
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
        *var_cgs_rdn16_slot = var_cgs_rdn16;
        *var_cgs_rdn17_slot = var_cgs_rdn17;
        *var_cgs_rdn18_slot = var_cgs_rdn18;
        *var_cgs_rdn2_slot = var_cgs_rdn2;
        *var_cgs_rdn3_slot = var_cgs_rdn3;
        *var_cgs_rdn4_slot = var_cgs_rdn4;
        *var_cgs_rdn5_slot = var_cgs_rdn5;
        *var_cgs_rdn6_slot = var_cgs_rdn6;
        *var_cgs_rdn7_slot = var_cgs_rdn7;
        *var_cgs_rdn8_slot = var_cgs_rdn8;
        *var_cgs_rdn9_slot = var_cgs_rdn9;
        *var_cgs_rv_slot = var_cgs_rv;
        *var_guard19_slot = var_guard19;
        *var_guard19_db0_slot = var_guard19_db0;
        *var_guard19_db1_slot = var_guard19_db1;
        *var_guard19_db10_slot = var_guard19_db10;
        *var_guard19_db11_slot = var_guard19_db11;
        *var_guard19_db12_slot = var_guard19_db12;
        *var_guard19_db13_slot = var_guard19_db13;
        *var_guard19_db14_slot = var_guard19_db14;
        *var_guard19_db15_slot = var_guard19_db15;
        *var_guard19_db16_slot = var_guard19_db16;
        *var_guard19_db17_slot = var_guard19_db17;
        *var_guard19_db18_slot = var_guard19_db18;
        *var_guard19_db2_slot = var_guard19_db2;
        *var_guard19_db3_slot = var_guard19_db3;
        *var_guard19_db4_slot = var_guard19_db4;
        *var_guard19_db5_slot = var_guard19_db5;
        *var_guard19_db6_slot = var_guard19_db6;
        *var_guard19_db7_slot = var_guard19_db7;
        *var_guard19_db8_slot = var_guard19_db8;
        *var_guard19_db9_slot = var_guard19_db9;
        *var_guard19_dn0_slot = var_guard19_dn0;
        *var_guard19_dn1_slot = var_guard19_dn1;
        *var_guard19_dn10_slot = var_guard19_dn10;
        *var_guard19_dn11_slot = var_guard19_dn11;
        *var_guard19_dn12_slot = var_guard19_dn12;
        *var_guard19_dn13_slot = var_guard19_dn13;
        *var_guard19_dn14_slot = var_guard19_dn14;
        *var_guard19_dn15_slot = var_guard19_dn15;
        *var_guard19_dn16_slot = var_guard19_dn16;
        *var_guard19_dn17_slot = var_guard19_dn17;
        *var_guard19_dn18_slot = var_guard19_dn18;
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
        *var_guard19_rdb10_slot = var_guard19_rdb10;
        *var_guard19_rdb11_slot = var_guard19_rdb11;
        *var_guard19_rdb12_slot = var_guard19_rdb12;
        *var_guard19_rdb13_slot = var_guard19_rdb13;
        *var_guard19_rdb14_slot = var_guard19_rdb14;
        *var_guard19_rdb15_slot = var_guard19_rdb15;
        *var_guard19_rdb16_slot = var_guard19_rdb16;
        *var_guard19_rdb17_slot = var_guard19_rdb17;
        *var_guard19_rdb18_slot = var_guard19_rdb18;
        *var_guard19_rdb2_slot = var_guard19_rdb2;
        *var_guard19_rdb3_slot = var_guard19_rdb3;
        *var_guard19_rdb4_slot = var_guard19_rdb4;
        *var_guard19_rdb5_slot = var_guard19_rdb5;
        *var_guard19_rdb6_slot = var_guard19_rdb6;
        *var_guard19_rdb7_slot = var_guard19_rdb7;
        *var_guard19_rdb8_slot = var_guard19_rdb8;
        *var_guard19_rdb9_slot = var_guard19_rdb9;
        *var_guard19_rdn0_slot = var_guard19_rdn0;
        *var_guard19_rdn1_slot = var_guard19_rdn1;
        *var_guard19_rdn10_slot = var_guard19_rdn10;
        *var_guard19_rdn11_slot = var_guard19_rdn11;
        *var_guard19_rdn12_slot = var_guard19_rdn12;
        *var_guard19_rdn13_slot = var_guard19_rdn13;
        *var_guard19_rdn14_slot = var_guard19_rdn14;
        *var_guard19_rdn15_slot = var_guard19_rdn15;
        *var_guard19_rdn16_slot = var_guard19_rdn16;
        *var_guard19_rdn17_slot = var_guard19_rdn17;
        *var_guard19_rdn18_slot = var_guard19_rdn18;
        *var_guard19_rdn2_slot = var_guard19_rdn2;
        *var_guard19_rdn3_slot = var_guard19_rdn3;
        *var_guard19_rdn4_slot = var_guard19_rdn4;
        *var_guard19_rdn5_slot = var_guard19_rdn5;
        *var_guard19_rdn6_slot = var_guard19_rdn6;
        *var_guard19_rdn7_slot = var_guard19_rdn7;
        *var_guard19_rdn8_slot = var_guard19_rdn8;
        *var_guard19_rdn9_slot = var_guard19_rdn9;
        *var_guard19_rv_slot = var_guard19_rv;
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
        *var_qgd0_dn16_slot = var_qgd0_dn16;
        *var_qgd0_dn17_slot = var_qgd0_dn17;
        *var_qgd0_dn18_slot = var_qgd0_dn18;
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
        *var_qgd0_rdn16_slot = var_qgd0_rdn16;
        *var_qgd0_rdn17_slot = var_qgd0_rdn17;
        *var_qgd0_rdn18_slot = var_qgd0_rdn18;
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
        *var_qgd_dn16_slot = var_qgd_dn16;
        *var_qgd_dn17_slot = var_qgd_dn17;
        *var_qgd_dn18_slot = var_qgd_dn18;
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
        *var_qgd_rdn16_slot = var_qgd_rdn16;
        *var_qgd_rdn17_slot = var_qgd_rdn17;
        *var_qgd_rdn18_slot = var_qgd_rdn18;
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

    pub(super) fn stamp_reactive_block_38(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        branches: &[usize; Instance::BRANCH_COUNT],
        var_ids0_dn12: f64,
        var_gm_slot: &mut f64,
        var_gm_db0_slot: &mut f64,
        var_gm_db1_slot: &mut f64,
        var_gm_db10_slot: &mut f64,
        var_gm_db11_slot: &mut f64,
        var_gm_db12_slot: &mut f64,
        var_gm_db13_slot: &mut f64,
        var_gm_db14_slot: &mut f64,
        var_gm_db15_slot: &mut f64,
        var_gm_db16_slot: &mut f64,
        var_gm_db17_slot: &mut f64,
        var_gm_db18_slot: &mut f64,
        var_gm_db2_slot: &mut f64,
        var_gm_db3_slot: &mut f64,
        var_gm_db4_slot: &mut f64,
        var_gm_db5_slot: &mut f64,
        var_gm_db6_slot: &mut f64,
        var_gm_db7_slot: &mut f64,
        var_gm_db8_slot: &mut f64,
        var_gm_db9_slot: &mut f64,
        var_gm_dn0_slot: &mut f64,
        var_gm_dn1_slot: &mut f64,
        var_gm_dn10_slot: &mut f64,
        var_gm_dn11_slot: &mut f64,
        var_gm_dn12_slot: &mut f64,
        var_gm_dn13_slot: &mut f64,
        var_gm_dn14_slot: &mut f64,
        var_gm_dn15_slot: &mut f64,
        var_gm_dn16_slot: &mut f64,
        var_gm_dn17_slot: &mut f64,
        var_gm_dn18_slot: &mut f64,
        var_gm_dn2_slot: &mut f64,
        var_gm_dn3_slot: &mut f64,
        var_gm_dn4_slot: &mut f64,
        var_gm_dn5_slot: &mut f64,
        var_gm_dn6_slot: &mut f64,
        var_gm_dn7_slot: &mut f64,
        var_gm_dn8_slot: &mut f64,
        var_gm_dn9_slot: &mut f64,
        var_gm_rdb0_slot: &mut f64,
        var_gm_rdb1_slot: &mut f64,
        var_gm_rdb10_slot: &mut f64,
        var_gm_rdb11_slot: &mut f64,
        var_gm_rdb12_slot: &mut f64,
        var_gm_rdb13_slot: &mut f64,
        var_gm_rdb14_slot: &mut f64,
        var_gm_rdb15_slot: &mut f64,
        var_gm_rdb16_slot: &mut f64,
        var_gm_rdb17_slot: &mut f64,
        var_gm_rdb18_slot: &mut f64,
        var_gm_rdb2_slot: &mut f64,
        var_gm_rdb3_slot: &mut f64,
        var_gm_rdb4_slot: &mut f64,
        var_gm_rdb5_slot: &mut f64,
        var_gm_rdb6_slot: &mut f64,
        var_gm_rdb7_slot: &mut f64,
        var_gm_rdb8_slot: &mut f64,
        var_gm_rdb9_slot: &mut f64,
        var_gm_rdn0_slot: &mut f64,
        var_gm_rdn1_slot: &mut f64,
        var_gm_rdn10_slot: &mut f64,
        var_gm_rdn11_slot: &mut f64,
        var_gm_rdn12_slot: &mut f64,
        var_gm_rdn13_slot: &mut f64,
        var_gm_rdn14_slot: &mut f64,
        var_gm_rdn15_slot: &mut f64,
        var_gm_rdn16_slot: &mut f64,
        var_gm_rdn17_slot: &mut f64,
        var_gm_rdn18_slot: &mut f64,
        var_gm_rdn2_slot: &mut f64,
        var_gm_rdn3_slot: &mut f64,
        var_gm_rdn4_slot: &mut f64,
        var_gm_rdn5_slot: &mut f64,
        var_gm_rdn6_slot: &mut f64,
        var_gm_rdn7_slot: &mut f64,
        var_gm_rdn8_slot: &mut f64,
        var_gm_rdn9_slot: &mut f64,
        var_gm_rv_slot: &mut f64,
        var_guard20_slot: &mut f64,
        var_guard20_db0_slot: &mut f64,
        var_guard20_db1_slot: &mut f64,
        var_guard20_db10_slot: &mut f64,
        var_guard20_db11_slot: &mut f64,
        var_guard20_db12_slot: &mut f64,
        var_guard20_db13_slot: &mut f64,
        var_guard20_db14_slot: &mut f64,
        var_guard20_db15_slot: &mut f64,
        var_guard20_db16_slot: &mut f64,
        var_guard20_db17_slot: &mut f64,
        var_guard20_db18_slot: &mut f64,
        var_guard20_db2_slot: &mut f64,
        var_guard20_db3_slot: &mut f64,
        var_guard20_db4_slot: &mut f64,
        var_guard20_db5_slot: &mut f64,
        var_guard20_db6_slot: &mut f64,
        var_guard20_db7_slot: &mut f64,
        var_guard20_db8_slot: &mut f64,
        var_guard20_db9_slot: &mut f64,
        var_guard20_dn0_slot: &mut f64,
        var_guard20_dn1_slot: &mut f64,
        var_guard20_dn10_slot: &mut f64,
        var_guard20_dn11_slot: &mut f64,
        var_guard20_dn12_slot: &mut f64,
        var_guard20_dn13_slot: &mut f64,
        var_guard20_dn14_slot: &mut f64,
        var_guard20_dn15_slot: &mut f64,
        var_guard20_dn16_slot: &mut f64,
        var_guard20_dn17_slot: &mut f64,
        var_guard20_dn18_slot: &mut f64,
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
        var_guard20_rdb10_slot: &mut f64,
        var_guard20_rdb11_slot: &mut f64,
        var_guard20_rdb12_slot: &mut f64,
        var_guard20_rdb13_slot: &mut f64,
        var_guard20_rdb14_slot: &mut f64,
        var_guard20_rdb15_slot: &mut f64,
        var_guard20_rdb16_slot: &mut f64,
        var_guard20_rdb17_slot: &mut f64,
        var_guard20_rdb18_slot: &mut f64,
        var_guard20_rdb2_slot: &mut f64,
        var_guard20_rdb3_slot: &mut f64,
        var_guard20_rdb4_slot: &mut f64,
        var_guard20_rdb5_slot: &mut f64,
        var_guard20_rdb6_slot: &mut f64,
        var_guard20_rdb7_slot: &mut f64,
        var_guard20_rdb8_slot: &mut f64,
        var_guard20_rdb9_slot: &mut f64,
        var_guard20_rdn0_slot: &mut f64,
        var_guard20_rdn1_slot: &mut f64,
        var_guard20_rdn10_slot: &mut f64,
        var_guard20_rdn11_slot: &mut f64,
        var_guard20_rdn12_slot: &mut f64,
        var_guard20_rdn13_slot: &mut f64,
        var_guard20_rdn14_slot: &mut f64,
        var_guard20_rdn15_slot: &mut f64,
        var_guard20_rdn16_slot: &mut f64,
        var_guard20_rdn17_slot: &mut f64,
        var_guard20_rdn18_slot: &mut f64,
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
        var_guard21_dn16_slot: &mut f64,
        var_guard21_dn17_slot: &mut f64,
        var_guard21_dn18_slot: &mut f64,
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
        var_guard21_rdn16_slot: &mut f64,
        var_guard21_rdn17_slot: &mut f64,
        var_guard21_rdn18_slot: &mut f64,
        var_guard21_rdn2_slot: &mut f64,
        var_guard21_rdn3_slot: &mut f64,
        var_guard21_rdn4_slot: &mut f64,
        var_guard21_rdn5_slot: &mut f64,
        var_guard21_rdn6_slot: &mut f64,
        var_guard21_rdn7_slot: &mut f64,
        var_guard21_rdn8_slot: &mut f64,
        var_guard21_rdn9_slot: &mut f64,
        var_guard21_rv_slot: &mut f64,
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
        var_guard26_dn16_slot: &mut f64,
        var_guard26_dn17_slot: &mut f64,
        var_guard26_dn18_slot: &mut f64,
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
        var_guard26_rdn16_slot: &mut f64,
        var_guard26_rdn17_slot: &mut f64,
        var_guard26_rdn18_slot: &mut f64,
        var_guard26_rdn2_slot: &mut f64,
        var_guard26_rdn3_slot: &mut f64,
        var_guard26_rdn4_slot: &mut f64,
        var_guard26_rdn5_slot: &mut f64,
        var_guard26_rdn6_slot: &mut f64,
        var_guard26_rdn7_slot: &mut f64,
        var_guard26_rdn8_slot: &mut f64,
        var_guard26_rdn9_slot: &mut f64,
        var_guard26_rv_slot: &mut f64,
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
        var_guard27_dn16_slot: &mut f64,
        var_guard27_dn17_slot: &mut f64,
        var_guard27_dn18_slot: &mut f64,
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
        var_guard27_rdn16_slot: &mut f64,
        var_guard27_rdn17_slot: &mut f64,
        var_guard27_rdn18_slot: &mut f64,
        var_guard27_rdn2_slot: &mut f64,
        var_guard27_rdn3_slot: &mut f64,
        var_guard27_rdn4_slot: &mut f64,
        var_guard27_rdn5_slot: &mut f64,
        var_guard27_rdn6_slot: &mut f64,
        var_guard27_rdn7_slot: &mut f64,
        var_guard27_rdn8_slot: &mut f64,
        var_guard27_rdn9_slot: &mut f64,
        var_guard27_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_db0_slot: &mut f64,
        var_t0_db1_slot: &mut f64,
        var_t0_db10_slot: &mut f64,
        var_t0_db11_slot: &mut f64,
        var_t0_db12_slot: &mut f64,
        var_t0_db13_slot: &mut f64,
        var_t0_db14_slot: &mut f64,
        var_t0_db15_slot: &mut f64,
        var_t0_db16_slot: &mut f64,
        var_t0_db17_slot: &mut f64,
        var_t0_db18_slot: &mut f64,
        var_t0_db2_slot: &mut f64,
        var_t0_db3_slot: &mut f64,
        var_t0_db4_slot: &mut f64,
        var_t0_db5_slot: &mut f64,
        var_t0_db6_slot: &mut f64,
        var_t0_db7_slot: &mut f64,
        var_t0_db8_slot: &mut f64,
        var_t0_db9_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn1_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn15_slot: &mut f64,
        var_t0_dn16_slot: &mut f64,
        var_t0_dn17_slot: &mut f64,
        var_t0_dn18_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rdb0_slot: &mut f64,
        var_t0_rdb1_slot: &mut f64,
        var_t0_rdb10_slot: &mut f64,
        var_t0_rdb11_slot: &mut f64,
        var_t0_rdb12_slot: &mut f64,
        var_t0_rdb13_slot: &mut f64,
        var_t0_rdb14_slot: &mut f64,
        var_t0_rdb15_slot: &mut f64,
        var_t0_rdb16_slot: &mut f64,
        var_t0_rdb17_slot: &mut f64,
        var_t0_rdb18_slot: &mut f64,
        var_t0_rdb2_slot: &mut f64,
        var_t0_rdb3_slot: &mut f64,
        var_t0_rdb4_slot: &mut f64,
        var_t0_rdb5_slot: &mut f64,
        var_t0_rdb6_slot: &mut f64,
        var_t0_rdb7_slot: &mut f64,
        var_t0_rdb8_slot: &mut f64,
        var_t0_rdb9_slot: &mut f64,
        var_t0_rdn0_slot: &mut f64,
        var_t0_rdn1_slot: &mut f64,
        var_t0_rdn10_slot: &mut f64,
        var_t0_rdn11_slot: &mut f64,
        var_t0_rdn12_slot: &mut f64,
        var_t0_rdn13_slot: &mut f64,
        var_t0_rdn14_slot: &mut f64,
        var_t0_rdn15_slot: &mut f64,
        var_t0_rdn16_slot: &mut f64,
        var_t0_rdn17_slot: &mut f64,
        var_t0_rdn18_slot: &mut f64,
        var_t0_rdn2_slot: &mut f64,
        var_t0_rdn3_slot: &mut f64,
        var_t0_rdn4_slot: &mut f64,
        var_t0_rdn5_slot: &mut f64,
        var_t0_rdn6_slot: &mut f64,
        var_t0_rdn7_slot: &mut f64,
        var_t0_rdn8_slot: &mut f64,
        var_t0_rdn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
    ) {
        let bi1 = ctx.branch_current(branches[1]);
        let mut var_gm: f64 = *var_gm_slot;
        let mut var_gm_db0: f64 = *var_gm_db0_slot;
        let mut var_gm_db1: f64 = *var_gm_db1_slot;
        let mut var_gm_db10: f64 = *var_gm_db10_slot;
        let mut var_gm_db11: f64 = *var_gm_db11_slot;
        let mut var_gm_db12: f64 = *var_gm_db12_slot;
        let mut var_gm_db13: f64 = *var_gm_db13_slot;
        let mut var_gm_db14: f64 = *var_gm_db14_slot;
        let mut var_gm_db15: f64 = *var_gm_db15_slot;
        let mut var_gm_db16: f64 = *var_gm_db16_slot;
        let mut var_gm_db17: f64 = *var_gm_db17_slot;
        let mut var_gm_db18: f64 = *var_gm_db18_slot;
        let mut var_gm_db2: f64 = *var_gm_db2_slot;
        let mut var_gm_db3: f64 = *var_gm_db3_slot;
        let mut var_gm_db4: f64 = *var_gm_db4_slot;
        let mut var_gm_db5: f64 = *var_gm_db5_slot;
        let mut var_gm_db6: f64 = *var_gm_db6_slot;
        let mut var_gm_db7: f64 = *var_gm_db7_slot;
        let mut var_gm_db8: f64 = *var_gm_db8_slot;
        let mut var_gm_db9: f64 = *var_gm_db9_slot;
        let mut var_gm_dn0: f64 = *var_gm_dn0_slot;
        let mut var_gm_dn1: f64 = *var_gm_dn1_slot;
        let mut var_gm_dn10: f64 = *var_gm_dn10_slot;
        let mut var_gm_dn11: f64 = *var_gm_dn11_slot;
        let mut var_gm_dn12: f64 = *var_gm_dn12_slot;
        let mut var_gm_dn13: f64 = *var_gm_dn13_slot;
        let mut var_gm_dn14: f64 = *var_gm_dn14_slot;
        let mut var_gm_dn15: f64 = *var_gm_dn15_slot;
        let mut var_gm_dn16: f64 = *var_gm_dn16_slot;
        let mut var_gm_dn17: f64 = *var_gm_dn17_slot;
        let mut var_gm_dn18: f64 = *var_gm_dn18_slot;
        let mut var_gm_dn2: f64 = *var_gm_dn2_slot;
        let mut var_gm_dn3: f64 = *var_gm_dn3_slot;
        let mut var_gm_dn4: f64 = *var_gm_dn4_slot;
        let mut var_gm_dn5: f64 = *var_gm_dn5_slot;
        let mut var_gm_dn6: f64 = *var_gm_dn6_slot;
        let mut var_gm_dn7: f64 = *var_gm_dn7_slot;
        let mut var_gm_dn8: f64 = *var_gm_dn8_slot;
        let mut var_gm_dn9: f64 = *var_gm_dn9_slot;
        let mut var_gm_rdb0: f64 = *var_gm_rdb0_slot;
        let mut var_gm_rdb1: f64 = *var_gm_rdb1_slot;
        let mut var_gm_rdb10: f64 = *var_gm_rdb10_slot;
        let mut var_gm_rdb11: f64 = *var_gm_rdb11_slot;
        let mut var_gm_rdb12: f64 = *var_gm_rdb12_slot;
        let mut var_gm_rdb13: f64 = *var_gm_rdb13_slot;
        let mut var_gm_rdb14: f64 = *var_gm_rdb14_slot;
        let mut var_gm_rdb15: f64 = *var_gm_rdb15_slot;
        let mut var_gm_rdb16: f64 = *var_gm_rdb16_slot;
        let mut var_gm_rdb17: f64 = *var_gm_rdb17_slot;
        let mut var_gm_rdb18: f64 = *var_gm_rdb18_slot;
        let mut var_gm_rdb2: f64 = *var_gm_rdb2_slot;
        let mut var_gm_rdb3: f64 = *var_gm_rdb3_slot;
        let mut var_gm_rdb4: f64 = *var_gm_rdb4_slot;
        let mut var_gm_rdb5: f64 = *var_gm_rdb5_slot;
        let mut var_gm_rdb6: f64 = *var_gm_rdb6_slot;
        let mut var_gm_rdb7: f64 = *var_gm_rdb7_slot;
        let mut var_gm_rdb8: f64 = *var_gm_rdb8_slot;
        let mut var_gm_rdb9: f64 = *var_gm_rdb9_slot;
        let mut var_gm_rdn0: f64 = *var_gm_rdn0_slot;
        let mut var_gm_rdn1: f64 = *var_gm_rdn1_slot;
        let mut var_gm_rdn10: f64 = *var_gm_rdn10_slot;
        let mut var_gm_rdn11: f64 = *var_gm_rdn11_slot;
        let mut var_gm_rdn12: f64 = *var_gm_rdn12_slot;
        let mut var_gm_rdn13: f64 = *var_gm_rdn13_slot;
        let mut var_gm_rdn14: f64 = *var_gm_rdn14_slot;
        let mut var_gm_rdn15: f64 = *var_gm_rdn15_slot;
        let mut var_gm_rdn16: f64 = *var_gm_rdn16_slot;
        let mut var_gm_rdn17: f64 = *var_gm_rdn17_slot;
        let mut var_gm_rdn18: f64 = *var_gm_rdn18_slot;
        let mut var_gm_rdn2: f64 = *var_gm_rdn2_slot;
        let mut var_gm_rdn3: f64 = *var_gm_rdn3_slot;
        let mut var_gm_rdn4: f64 = *var_gm_rdn4_slot;
        let mut var_gm_rdn5: f64 = *var_gm_rdn5_slot;
        let mut var_gm_rdn6: f64 = *var_gm_rdn6_slot;
        let mut var_gm_rdn7: f64 = *var_gm_rdn7_slot;
        let mut var_gm_rdn8: f64 = *var_gm_rdn8_slot;
        let mut var_gm_rdn9: f64 = *var_gm_rdn9_slot;
        let mut var_gm_rv: f64 = *var_gm_rv_slot;
        let mut var_guard20: f64 = *var_guard20_slot;
        let mut var_guard20_db0: f64 = *var_guard20_db0_slot;
        let mut var_guard20_db1: f64 = *var_guard20_db1_slot;
        let mut var_guard20_db10: f64 = *var_guard20_db10_slot;
        let mut var_guard20_db11: f64 = *var_guard20_db11_slot;
        let mut var_guard20_db12: f64 = *var_guard20_db12_slot;
        let mut var_guard20_db13: f64 = *var_guard20_db13_slot;
        let mut var_guard20_db14: f64 = *var_guard20_db14_slot;
        let mut var_guard20_db15: f64 = *var_guard20_db15_slot;
        let mut var_guard20_db16: f64 = *var_guard20_db16_slot;
        let mut var_guard20_db17: f64 = *var_guard20_db17_slot;
        let mut var_guard20_db18: f64 = *var_guard20_db18_slot;
        let mut var_guard20_db2: f64 = *var_guard20_db2_slot;
        let mut var_guard20_db3: f64 = *var_guard20_db3_slot;
        let mut var_guard20_db4: f64 = *var_guard20_db4_slot;
        let mut var_guard20_db5: f64 = *var_guard20_db5_slot;
        let mut var_guard20_db6: f64 = *var_guard20_db6_slot;
        let mut var_guard20_db7: f64 = *var_guard20_db7_slot;
        let mut var_guard20_db8: f64 = *var_guard20_db8_slot;
        let mut var_guard20_db9: f64 = *var_guard20_db9_slot;
        let mut var_guard20_dn0: f64 = *var_guard20_dn0_slot;
        let mut var_guard20_dn1: f64 = *var_guard20_dn1_slot;
        let mut var_guard20_dn10: f64 = *var_guard20_dn10_slot;
        let mut var_guard20_dn11: f64 = *var_guard20_dn11_slot;
        let mut var_guard20_dn12: f64 = *var_guard20_dn12_slot;
        let mut var_guard20_dn13: f64 = *var_guard20_dn13_slot;
        let mut var_guard20_dn14: f64 = *var_guard20_dn14_slot;
        let mut var_guard20_dn15: f64 = *var_guard20_dn15_slot;
        let mut var_guard20_dn16: f64 = *var_guard20_dn16_slot;
        let mut var_guard20_dn17: f64 = *var_guard20_dn17_slot;
        let mut var_guard20_dn18: f64 = *var_guard20_dn18_slot;
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
        let mut var_guard20_rdb10: f64 = *var_guard20_rdb10_slot;
        let mut var_guard20_rdb11: f64 = *var_guard20_rdb11_slot;
        let mut var_guard20_rdb12: f64 = *var_guard20_rdb12_slot;
        let mut var_guard20_rdb13: f64 = *var_guard20_rdb13_slot;
        let mut var_guard20_rdb14: f64 = *var_guard20_rdb14_slot;
        let mut var_guard20_rdb15: f64 = *var_guard20_rdb15_slot;
        let mut var_guard20_rdb16: f64 = *var_guard20_rdb16_slot;
        let mut var_guard20_rdb17: f64 = *var_guard20_rdb17_slot;
        let mut var_guard20_rdb18: f64 = *var_guard20_rdb18_slot;
        let mut var_guard20_rdb2: f64 = *var_guard20_rdb2_slot;
        let mut var_guard20_rdb3: f64 = *var_guard20_rdb3_slot;
        let mut var_guard20_rdb4: f64 = *var_guard20_rdb4_slot;
        let mut var_guard20_rdb5: f64 = *var_guard20_rdb5_slot;
        let mut var_guard20_rdb6: f64 = *var_guard20_rdb6_slot;
        let mut var_guard20_rdb7: f64 = *var_guard20_rdb7_slot;
        let mut var_guard20_rdb8: f64 = *var_guard20_rdb8_slot;
        let mut var_guard20_rdb9: f64 = *var_guard20_rdb9_slot;
        let mut var_guard20_rdn0: f64 = *var_guard20_rdn0_slot;
        let mut var_guard20_rdn1: f64 = *var_guard20_rdn1_slot;
        let mut var_guard20_rdn10: f64 = *var_guard20_rdn10_slot;
        let mut var_guard20_rdn11: f64 = *var_guard20_rdn11_slot;
        let mut var_guard20_rdn12: f64 = *var_guard20_rdn12_slot;
        let mut var_guard20_rdn13: f64 = *var_guard20_rdn13_slot;
        let mut var_guard20_rdn14: f64 = *var_guard20_rdn14_slot;
        let mut var_guard20_rdn15: f64 = *var_guard20_rdn15_slot;
        let mut var_guard20_rdn16: f64 = *var_guard20_rdn16_slot;
        let mut var_guard20_rdn17: f64 = *var_guard20_rdn17_slot;
        let mut var_guard20_rdn18: f64 = *var_guard20_rdn18_slot;
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
        let mut var_guard21_dn16: f64 = *var_guard21_dn16_slot;
        let mut var_guard21_dn17: f64 = *var_guard21_dn17_slot;
        let mut var_guard21_dn18: f64 = *var_guard21_dn18_slot;
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
        let mut var_guard21_rdn16: f64 = *var_guard21_rdn16_slot;
        let mut var_guard21_rdn17: f64 = *var_guard21_rdn17_slot;
        let mut var_guard21_rdn18: f64 = *var_guard21_rdn18_slot;
        let mut var_guard21_rdn2: f64 = *var_guard21_rdn2_slot;
        let mut var_guard21_rdn3: f64 = *var_guard21_rdn3_slot;
        let mut var_guard21_rdn4: f64 = *var_guard21_rdn4_slot;
        let mut var_guard21_rdn5: f64 = *var_guard21_rdn5_slot;
        let mut var_guard21_rdn6: f64 = *var_guard21_rdn6_slot;
        let mut var_guard21_rdn7: f64 = *var_guard21_rdn7_slot;
        let mut var_guard21_rdn8: f64 = *var_guard21_rdn8_slot;
        let mut var_guard21_rdn9: f64 = *var_guard21_rdn9_slot;
        let mut var_guard21_rv: f64 = *var_guard21_rv_slot;
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
        let mut var_guard26_dn16: f64 = *var_guard26_dn16_slot;
        let mut var_guard26_dn17: f64 = *var_guard26_dn17_slot;
        let mut var_guard26_dn18: f64 = *var_guard26_dn18_slot;
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
        let mut var_guard26_rdn16: f64 = *var_guard26_rdn16_slot;
        let mut var_guard26_rdn17: f64 = *var_guard26_rdn17_slot;
        let mut var_guard26_rdn18: f64 = *var_guard26_rdn18_slot;
        let mut var_guard26_rdn2: f64 = *var_guard26_rdn2_slot;
        let mut var_guard26_rdn3: f64 = *var_guard26_rdn3_slot;
        let mut var_guard26_rdn4: f64 = *var_guard26_rdn4_slot;
        let mut var_guard26_rdn5: f64 = *var_guard26_rdn5_slot;
        let mut var_guard26_rdn6: f64 = *var_guard26_rdn6_slot;
        let mut var_guard26_rdn7: f64 = *var_guard26_rdn7_slot;
        let mut var_guard26_rdn8: f64 = *var_guard26_rdn8_slot;
        let mut var_guard26_rdn9: f64 = *var_guard26_rdn9_slot;
        let mut var_guard26_rv: f64 = *var_guard26_rv_slot;
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
        let mut var_guard27_dn16: f64 = *var_guard27_dn16_slot;
        let mut var_guard27_dn17: f64 = *var_guard27_dn17_slot;
        let mut var_guard27_dn18: f64 = *var_guard27_dn18_slot;
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
        let mut var_guard27_rdn16: f64 = *var_guard27_rdn16_slot;
        let mut var_guard27_rdn17: f64 = *var_guard27_rdn17_slot;
        let mut var_guard27_rdn18: f64 = *var_guard27_rdn18_slot;
        let mut var_guard27_rdn2: f64 = *var_guard27_rdn2_slot;
        let mut var_guard27_rdn3: f64 = *var_guard27_rdn3_slot;
        let mut var_guard27_rdn4: f64 = *var_guard27_rdn4_slot;
        let mut var_guard27_rdn5: f64 = *var_guard27_rdn5_slot;
        let mut var_guard27_rdn6: f64 = *var_guard27_rdn6_slot;
        let mut var_guard27_rdn7: f64 = *var_guard27_rdn7_slot;
        let mut var_guard27_rdn8: f64 = *var_guard27_rdn8_slot;
        let mut var_guard27_rdn9: f64 = *var_guard27_rdn9_slot;
        let mut var_guard27_rv: f64 = *var_guard27_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_db0: f64 = *var_t0_db0_slot;
        let mut var_t0_db1: f64 = *var_t0_db1_slot;
        let mut var_t0_db10: f64 = *var_t0_db10_slot;
        let mut var_t0_db11: f64 = *var_t0_db11_slot;
        let mut var_t0_db12: f64 = *var_t0_db12_slot;
        let mut var_t0_db13: f64 = *var_t0_db13_slot;
        let mut var_t0_db14: f64 = *var_t0_db14_slot;
        let mut var_t0_db15: f64 = *var_t0_db15_slot;
        let mut var_t0_db16: f64 = *var_t0_db16_slot;
        let mut var_t0_db17: f64 = *var_t0_db17_slot;
        let mut var_t0_db18: f64 = *var_t0_db18_slot;
        let mut var_t0_db2: f64 = *var_t0_db2_slot;
        let mut var_t0_db3: f64 = *var_t0_db3_slot;
        let mut var_t0_db4: f64 = *var_t0_db4_slot;
        let mut var_t0_db5: f64 = *var_t0_db5_slot;
        let mut var_t0_db6: f64 = *var_t0_db6_slot;
        let mut var_t0_db7: f64 = *var_t0_db7_slot;
        let mut var_t0_db8: f64 = *var_t0_db8_slot;
        let mut var_t0_db9: f64 = *var_t0_db9_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn1: f64 = *var_t0_dn1_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn15: f64 = *var_t0_dn15_slot;
        let mut var_t0_dn16: f64 = *var_t0_dn16_slot;
        let mut var_t0_dn17: f64 = *var_t0_dn17_slot;
        let mut var_t0_dn18: f64 = *var_t0_dn18_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rdb0: f64 = *var_t0_rdb0_slot;
        let mut var_t0_rdb1: f64 = *var_t0_rdb1_slot;
        let mut var_t0_rdb10: f64 = *var_t0_rdb10_slot;
        let mut var_t0_rdb11: f64 = *var_t0_rdb11_slot;
        let mut var_t0_rdb12: f64 = *var_t0_rdb12_slot;
        let mut var_t0_rdb13: f64 = *var_t0_rdb13_slot;
        let mut var_t0_rdb14: f64 = *var_t0_rdb14_slot;
        let mut var_t0_rdb15: f64 = *var_t0_rdb15_slot;
        let mut var_t0_rdb16: f64 = *var_t0_rdb16_slot;
        let mut var_t0_rdb17: f64 = *var_t0_rdb17_slot;
        let mut var_t0_rdb18: f64 = *var_t0_rdb18_slot;
        let mut var_t0_rdb2: f64 = *var_t0_rdb2_slot;
        let mut var_t0_rdb3: f64 = *var_t0_rdb3_slot;
        let mut var_t0_rdb4: f64 = *var_t0_rdb4_slot;
        let mut var_t0_rdb5: f64 = *var_t0_rdb5_slot;
        let mut var_t0_rdb6: f64 = *var_t0_rdb6_slot;
        let mut var_t0_rdb7: f64 = *var_t0_rdb7_slot;
        let mut var_t0_rdb8: f64 = *var_t0_rdb8_slot;
        let mut var_t0_rdb9: f64 = *var_t0_rdb9_slot;
        let mut var_t0_rdn0: f64 = *var_t0_rdn0_slot;
        let mut var_t0_rdn1: f64 = *var_t0_rdn1_slot;
        let mut var_t0_rdn10: f64 = *var_t0_rdn10_slot;
        let mut var_t0_rdn11: f64 = *var_t0_rdn11_slot;
        let mut var_t0_rdn12: f64 = *var_t0_rdn12_slot;
        let mut var_t0_rdn13: f64 = *var_t0_rdn13_slot;
        let mut var_t0_rdn14: f64 = *var_t0_rdn14_slot;
        let mut var_t0_rdn15: f64 = *var_t0_rdn15_slot;
        let mut var_t0_rdn16: f64 = *var_t0_rdn16_slot;
        let mut var_t0_rdn17: f64 = *var_t0_rdn17_slot;
        let mut var_t0_rdn18: f64 = *var_t0_rdn18_slot;
        let mut var_t0_rdn2: f64 = *var_t0_rdn2_slot;
        let mut var_t0_rdn3: f64 = *var_t0_rdn3_slot;
        let mut var_t0_rdn4: f64 = *var_t0_rdn4_slot;
        let mut var_t0_rdn5: f64 = *var_t0_rdn5_slot;
        let mut var_t0_rdn6: f64 = *var_t0_rdn6_slot;
        let mut var_t0_rdn7: f64 = *var_t0_rdn7_slot;
        let mut var_t0_rdn8: f64 = *var_t0_rdn8_slot;
        let mut var_t0_rdn9: f64 = *var_t0_rdn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;

        let assign2090_e2833: f64 = (p.p55 * bi1);
        let assign2090_e2834_q: f64 = assign2090_e2833;
        var_t0 = assign2090_e2833;
        var_t0_dn0 = 0.0;
        var_t0_dn1 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_dn15 = 0.0;
        var_t0_dn16 = 0.0;
        var_t0_dn17 = 0.0;
        var_t0_dn18 = 0.0;
        var_t0_db0 = 0.0;
        var_t0_db1 = p.p55;
        var_t0_db2 = 0.0;
        var_t0_db3 = 0.0;
        var_t0_db4 = 0.0;
        var_t0_db5 = 0.0;
        var_t0_db6 = 0.0;
        var_t0_db7 = 0.0;
        var_t0_db8 = 0.0;
        var_t0_db9 = 0.0;
        var_t0_db10 = 0.0;
        var_t0_db11 = 0.0;
        var_t0_db12 = 0.0;
        var_t0_db13 = 0.0;
        var_t0_db14 = 0.0;
        var_t0_db15 = 0.0;
        var_t0_db16 = 0.0;
        var_t0_db17 = 0.0;
        var_t0_db18 = 0.0;
        var_t0_rv = assign2090_e2834_q;
        var_t0_rdn0 = 0.0;
        var_t0_rdn1 = 0.0;
        var_t0_rdn2 = 0.0;
        var_t0_rdn3 = 0.0;
        var_t0_rdn4 = 0.0;
        var_t0_rdn5 = 0.0;
        var_t0_rdn6 = 0.0;
        var_t0_rdn7 = 0.0;
        var_t0_rdn8 = 0.0;
        var_t0_rdn9 = 0.0;
        var_t0_rdn10 = 0.0;
        var_t0_rdn11 = 0.0;
        var_t0_rdn12 = 0.0;
        var_t0_rdn13 = 0.0;
        var_t0_rdn14 = 0.0;
        var_t0_rdn15 = 0.0;
        var_t0_rdn16 = 0.0;
        var_t0_rdn17 = 0.0;
        var_t0_rdn18 = 0.0;
        var_t0_rdb0 = 0.0;
        var_t0_rdb1 = p.p55;
        var_t0_rdb2 = 0.0;
        var_t0_rdb3 = 0.0;
        var_t0_rdb4 = 0.0;
        var_t0_rdb5 = 0.0;
        var_t0_rdb6 = 0.0;
        var_t0_rdb7 = 0.0;
        var_t0_rdb8 = 0.0;
        var_t0_rdb9 = 0.0;
        var_t0_rdb10 = 0.0;
        var_t0_rdb11 = 0.0;
        var_t0_rdb12 = 0.0;
        var_t0_rdb13 = 0.0;
        var_t0_rdb14 = 0.0;
        var_t0_rdb15 = 0.0;
        var_t0_rdb16 = 0.0;
        var_t0_rdb17 = 0.0;
        var_t0_rdb18 = 0.0;

        let assign2100_e2837: f64 = if p.p58 > 0.0 { 1.0 } else { 0.0 };
        var_guard20 = assign2100_e2837;
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
        var_guard20_dn10 = 0.0;
        var_guard20_dn11 = 0.0;
        var_guard20_dn12 = 0.0;
        var_guard20_dn13 = 0.0;
        var_guard20_dn14 = 0.0;
        var_guard20_dn15 = 0.0;
        var_guard20_dn16 = 0.0;
        var_guard20_dn17 = 0.0;
        var_guard20_dn18 = 0.0;
        var_guard20_db0 = 0.0;
        var_guard20_db1 = 0.0;
        var_guard20_db2 = 0.0;
        var_guard20_db3 = 0.0;
        var_guard20_db4 = 0.0;
        var_guard20_db5 = 0.0;
        var_guard20_db6 = 0.0;
        var_guard20_db7 = 0.0;
        var_guard20_db8 = 0.0;
        var_guard20_db9 = 0.0;
        var_guard20_db10 = 0.0;
        var_guard20_db11 = 0.0;
        var_guard20_db12 = 0.0;
        var_guard20_db13 = 0.0;
        var_guard20_db14 = 0.0;
        var_guard20_db15 = 0.0;
        var_guard20_db16 = 0.0;
        var_guard20_db17 = 0.0;
        var_guard20_db18 = 0.0;
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
        var_guard20_rdn10 = 0.0;
        var_guard20_rdn11 = 0.0;
        var_guard20_rdn12 = 0.0;
        var_guard20_rdn13 = 0.0;
        var_guard20_rdn14 = 0.0;
        var_guard20_rdn15 = 0.0;
        var_guard20_rdn16 = 0.0;
        var_guard20_rdn17 = 0.0;
        var_guard20_rdn18 = 0.0;
        var_guard20_rdb0 = 0.0;
        var_guard20_rdb1 = 0.0;
        var_guard20_rdb2 = 0.0;
        var_guard20_rdb3 = 0.0;
        var_guard20_rdb4 = 0.0;
        var_guard20_rdb5 = 0.0;
        var_guard20_rdb6 = 0.0;
        var_guard20_rdb7 = 0.0;
        var_guard20_rdb8 = 0.0;
        var_guard20_rdb9 = 0.0;
        var_guard20_rdb10 = 0.0;
        var_guard20_rdb11 = 0.0;
        var_guard20_rdb12 = 0.0;
        var_guard20_rdb13 = 0.0;
        var_guard20_rdb14 = 0.0;
        var_guard20_rdb15 = 0.0;
        var_guard20_rdb16 = 0.0;
        var_guard20_rdb17 = 0.0;
        var_guard20_rdb18 = 0.0;

        let assign2110_e2844: f64 = if ((p.p63 > 0.0) || (p.p62 > 0.0)) { 1.0 } else { 0.0 };
        var_guard21 = assign2110_e2844;
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
        var_guard21_dn16 = 0.0;
        var_guard21_dn17 = 0.0;
        var_guard21_dn18 = 0.0;
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
        var_guard21_rdn16 = 0.0;
        var_guard21_rdn17 = 0.0;
        var_guard21_rdn18 = 0.0;
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

        let assign2160_e2859: f64 = if p.p50 > 0.0 { 1.0 } else { 0.0 };
        var_guard26 = assign2160_e2859;
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
        var_guard26_dn16 = 0.0;
        var_guard26_dn17 = 0.0;
        var_guard26_dn18 = 0.0;
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
        var_guard26_rdn16 = 0.0;
        var_guard26_rdn17 = 0.0;
        var_guard26_rdn18 = 0.0;
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

        let assign2170_e2866: f64 = if ((p.p47 > 0.0) || (p.p48 > 0.0)) { 1.0 } else { 0.0 };
        var_guard27 = assign2170_e2866;
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
        var_guard27_dn16 = 0.0;
        var_guard27_dn17 = 0.0;
        var_guard27_dn18 = 0.0;
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
        var_guard27_rdn16 = 0.0;
        var_guard27_rdn17 = 0.0;
        var_guard27_rdn18 = 0.0;
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

        let assign2180_e2869: f64 = var_ids0_dn12;
        var_gm = assign2180_e2869;
        var_gm_dn0 = 0.0;
        var_gm_dn1 = 0.0;
        var_gm_dn2 = 0.0;
        var_gm_dn3 = 0.0;
        var_gm_dn4 = 0.0;
        var_gm_dn5 = 0.0;
        var_gm_dn6 = 0.0;
        var_gm_dn7 = 0.0;
        var_gm_dn8 = 0.0;
        var_gm_dn9 = 0.0;
        var_gm_dn10 = 0.0;
        var_gm_dn11 = 0.0;
        var_gm_dn12 = 0.0;
        var_gm_dn13 = 0.0;
        var_gm_dn14 = 0.0;
        var_gm_dn15 = 0.0;
        var_gm_dn16 = 0.0;
        var_gm_dn17 = 0.0;
        var_gm_dn18 = 0.0;
        var_gm_db0 = 0.0;
        var_gm_db1 = 0.0;
        var_gm_db2 = 0.0;
        var_gm_db3 = 0.0;
        var_gm_db4 = 0.0;
        var_gm_db5 = 0.0;
        var_gm_db6 = 0.0;
        var_gm_db7 = 0.0;
        var_gm_db8 = 0.0;
        var_gm_db9 = 0.0;
        var_gm_db10 = 0.0;
        var_gm_db11 = 0.0;
        var_gm_db12 = 0.0;
        var_gm_db13 = 0.0;
        var_gm_db14 = 0.0;
        var_gm_db15 = 0.0;
        var_gm_db16 = 0.0;
        var_gm_db17 = 0.0;
        var_gm_db18 = 0.0;
        var_gm_rv = 0.0;
        var_gm_rdn0 = 0.0;
        var_gm_rdn1 = 0.0;
        var_gm_rdn2 = 0.0;
        var_gm_rdn3 = 0.0;
        var_gm_rdn4 = 0.0;
        var_gm_rdn5 = 0.0;
        var_gm_rdn6 = 0.0;
        var_gm_rdn7 = 0.0;
        var_gm_rdn8 = 0.0;
        var_gm_rdn9 = 0.0;
        var_gm_rdn10 = 0.0;
        var_gm_rdn11 = 0.0;
        var_gm_rdn12 = 0.0;
        var_gm_rdn13 = 0.0;
        var_gm_rdn14 = 0.0;
        var_gm_rdn15 = 0.0;
        var_gm_rdn16 = 0.0;
        var_gm_rdn17 = 0.0;
        var_gm_rdn18 = 0.0;
        var_gm_rdb0 = 0.0;
        var_gm_rdb1 = 0.0;
        var_gm_rdb2 = 0.0;
        var_gm_rdb3 = 0.0;
        var_gm_rdb4 = 0.0;
        var_gm_rdb5 = 0.0;
        var_gm_rdb6 = 0.0;
        var_gm_rdb7 = 0.0;
        var_gm_rdb8 = 0.0;
        var_gm_rdb9 = 0.0;
        var_gm_rdb10 = 0.0;
        var_gm_rdb11 = 0.0;
        var_gm_rdb12 = 0.0;
        var_gm_rdb13 = 0.0;
        var_gm_rdb14 = 0.0;
        var_gm_rdb15 = 0.0;
        var_gm_rdb16 = 0.0;
        var_gm_rdb17 = 0.0;
        var_gm_rdb18 = 0.0;


        *var_gm_slot = var_gm;
        *var_gm_db0_slot = var_gm_db0;
        *var_gm_db1_slot = var_gm_db1;
        *var_gm_db10_slot = var_gm_db10;
        *var_gm_db11_slot = var_gm_db11;
        *var_gm_db12_slot = var_gm_db12;
        *var_gm_db13_slot = var_gm_db13;
        *var_gm_db14_slot = var_gm_db14;
        *var_gm_db15_slot = var_gm_db15;
        *var_gm_db16_slot = var_gm_db16;
        *var_gm_db17_slot = var_gm_db17;
        *var_gm_db18_slot = var_gm_db18;
        *var_gm_db2_slot = var_gm_db2;
        *var_gm_db3_slot = var_gm_db3;
        *var_gm_db4_slot = var_gm_db4;
        *var_gm_db5_slot = var_gm_db5;
        *var_gm_db6_slot = var_gm_db6;
        *var_gm_db7_slot = var_gm_db7;
        *var_gm_db8_slot = var_gm_db8;
        *var_gm_db9_slot = var_gm_db9;
        *var_gm_dn0_slot = var_gm_dn0;
        *var_gm_dn1_slot = var_gm_dn1;
        *var_gm_dn10_slot = var_gm_dn10;
        *var_gm_dn11_slot = var_gm_dn11;
        *var_gm_dn12_slot = var_gm_dn12;
        *var_gm_dn13_slot = var_gm_dn13;
        *var_gm_dn14_slot = var_gm_dn14;
        *var_gm_dn15_slot = var_gm_dn15;
        *var_gm_dn16_slot = var_gm_dn16;
        *var_gm_dn17_slot = var_gm_dn17;
        *var_gm_dn18_slot = var_gm_dn18;
        *var_gm_dn2_slot = var_gm_dn2;
        *var_gm_dn3_slot = var_gm_dn3;
        *var_gm_dn4_slot = var_gm_dn4;
        *var_gm_dn5_slot = var_gm_dn5;
        *var_gm_dn6_slot = var_gm_dn6;
        *var_gm_dn7_slot = var_gm_dn7;
        *var_gm_dn8_slot = var_gm_dn8;
        *var_gm_dn9_slot = var_gm_dn9;
        *var_gm_rdb0_slot = var_gm_rdb0;
        *var_gm_rdb1_slot = var_gm_rdb1;
        *var_gm_rdb10_slot = var_gm_rdb10;
        *var_gm_rdb11_slot = var_gm_rdb11;
        *var_gm_rdb12_slot = var_gm_rdb12;
        *var_gm_rdb13_slot = var_gm_rdb13;
        *var_gm_rdb14_slot = var_gm_rdb14;
        *var_gm_rdb15_slot = var_gm_rdb15;
        *var_gm_rdb16_slot = var_gm_rdb16;
        *var_gm_rdb17_slot = var_gm_rdb17;
        *var_gm_rdb18_slot = var_gm_rdb18;
        *var_gm_rdb2_slot = var_gm_rdb2;
        *var_gm_rdb3_slot = var_gm_rdb3;
        *var_gm_rdb4_slot = var_gm_rdb4;
        *var_gm_rdb5_slot = var_gm_rdb5;
        *var_gm_rdb6_slot = var_gm_rdb6;
        *var_gm_rdb7_slot = var_gm_rdb7;
        *var_gm_rdb8_slot = var_gm_rdb8;
        *var_gm_rdb9_slot = var_gm_rdb9;
        *var_gm_rdn0_slot = var_gm_rdn0;
        *var_gm_rdn1_slot = var_gm_rdn1;
        *var_gm_rdn10_slot = var_gm_rdn10;
        *var_gm_rdn11_slot = var_gm_rdn11;
        *var_gm_rdn12_slot = var_gm_rdn12;
        *var_gm_rdn13_slot = var_gm_rdn13;
        *var_gm_rdn14_slot = var_gm_rdn14;
        *var_gm_rdn15_slot = var_gm_rdn15;
        *var_gm_rdn16_slot = var_gm_rdn16;
        *var_gm_rdn17_slot = var_gm_rdn17;
        *var_gm_rdn18_slot = var_gm_rdn18;
        *var_gm_rdn2_slot = var_gm_rdn2;
        *var_gm_rdn3_slot = var_gm_rdn3;
        *var_gm_rdn4_slot = var_gm_rdn4;
        *var_gm_rdn5_slot = var_gm_rdn5;
        *var_gm_rdn6_slot = var_gm_rdn6;
        *var_gm_rdn7_slot = var_gm_rdn7;
        *var_gm_rdn8_slot = var_gm_rdn8;
        *var_gm_rdn9_slot = var_gm_rdn9;
        *var_gm_rv_slot = var_gm_rv;
        *var_guard20_slot = var_guard20;
        *var_guard20_db0_slot = var_guard20_db0;
        *var_guard20_db1_slot = var_guard20_db1;
        *var_guard20_db10_slot = var_guard20_db10;
        *var_guard20_db11_slot = var_guard20_db11;
        *var_guard20_db12_slot = var_guard20_db12;
        *var_guard20_db13_slot = var_guard20_db13;
        *var_guard20_db14_slot = var_guard20_db14;
        *var_guard20_db15_slot = var_guard20_db15;
        *var_guard20_db16_slot = var_guard20_db16;
        *var_guard20_db17_slot = var_guard20_db17;
        *var_guard20_db18_slot = var_guard20_db18;
        *var_guard20_db2_slot = var_guard20_db2;
        *var_guard20_db3_slot = var_guard20_db3;
        *var_guard20_db4_slot = var_guard20_db4;
        *var_guard20_db5_slot = var_guard20_db5;
        *var_guard20_db6_slot = var_guard20_db6;
        *var_guard20_db7_slot = var_guard20_db7;
        *var_guard20_db8_slot = var_guard20_db8;
        *var_guard20_db9_slot = var_guard20_db9;
        *var_guard20_dn0_slot = var_guard20_dn0;
        *var_guard20_dn1_slot = var_guard20_dn1;
        *var_guard20_dn10_slot = var_guard20_dn10;
        *var_guard20_dn11_slot = var_guard20_dn11;
        *var_guard20_dn12_slot = var_guard20_dn12;
        *var_guard20_dn13_slot = var_guard20_dn13;
        *var_guard20_dn14_slot = var_guard20_dn14;
        *var_guard20_dn15_slot = var_guard20_dn15;
        *var_guard20_dn16_slot = var_guard20_dn16;
        *var_guard20_dn17_slot = var_guard20_dn17;
        *var_guard20_dn18_slot = var_guard20_dn18;
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
        *var_guard20_rdb10_slot = var_guard20_rdb10;
        *var_guard20_rdb11_slot = var_guard20_rdb11;
        *var_guard20_rdb12_slot = var_guard20_rdb12;
        *var_guard20_rdb13_slot = var_guard20_rdb13;
        *var_guard20_rdb14_slot = var_guard20_rdb14;
        *var_guard20_rdb15_slot = var_guard20_rdb15;
        *var_guard20_rdb16_slot = var_guard20_rdb16;
        *var_guard20_rdb17_slot = var_guard20_rdb17;
        *var_guard20_rdb18_slot = var_guard20_rdb18;
        *var_guard20_rdb2_slot = var_guard20_rdb2;
        *var_guard20_rdb3_slot = var_guard20_rdb3;
        *var_guard20_rdb4_slot = var_guard20_rdb4;
        *var_guard20_rdb5_slot = var_guard20_rdb5;
        *var_guard20_rdb6_slot = var_guard20_rdb6;
        *var_guard20_rdb7_slot = var_guard20_rdb7;
        *var_guard20_rdb8_slot = var_guard20_rdb8;
        *var_guard20_rdb9_slot = var_guard20_rdb9;
        *var_guard20_rdn0_slot = var_guard20_rdn0;
        *var_guard20_rdn1_slot = var_guard20_rdn1;
        *var_guard20_rdn10_slot = var_guard20_rdn10;
        *var_guard20_rdn11_slot = var_guard20_rdn11;
        *var_guard20_rdn12_slot = var_guard20_rdn12;
        *var_guard20_rdn13_slot = var_guard20_rdn13;
        *var_guard20_rdn14_slot = var_guard20_rdn14;
        *var_guard20_rdn15_slot = var_guard20_rdn15;
        *var_guard20_rdn16_slot = var_guard20_rdn16;
        *var_guard20_rdn17_slot = var_guard20_rdn17;
        *var_guard20_rdn18_slot = var_guard20_rdn18;
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
        *var_guard21_dn16_slot = var_guard21_dn16;
        *var_guard21_dn17_slot = var_guard21_dn17;
        *var_guard21_dn18_slot = var_guard21_dn18;
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
        *var_guard21_rdn16_slot = var_guard21_rdn16;
        *var_guard21_rdn17_slot = var_guard21_rdn17;
        *var_guard21_rdn18_slot = var_guard21_rdn18;
        *var_guard21_rdn2_slot = var_guard21_rdn2;
        *var_guard21_rdn3_slot = var_guard21_rdn3;
        *var_guard21_rdn4_slot = var_guard21_rdn4;
        *var_guard21_rdn5_slot = var_guard21_rdn5;
        *var_guard21_rdn6_slot = var_guard21_rdn6;
        *var_guard21_rdn7_slot = var_guard21_rdn7;
        *var_guard21_rdn8_slot = var_guard21_rdn8;
        *var_guard21_rdn9_slot = var_guard21_rdn9;
        *var_guard21_rv_slot = var_guard21_rv;
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
        *var_guard26_dn16_slot = var_guard26_dn16;
        *var_guard26_dn17_slot = var_guard26_dn17;
        *var_guard26_dn18_slot = var_guard26_dn18;
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
        *var_guard26_rdn16_slot = var_guard26_rdn16;
        *var_guard26_rdn17_slot = var_guard26_rdn17;
        *var_guard26_rdn18_slot = var_guard26_rdn18;
        *var_guard26_rdn2_slot = var_guard26_rdn2;
        *var_guard26_rdn3_slot = var_guard26_rdn3;
        *var_guard26_rdn4_slot = var_guard26_rdn4;
        *var_guard26_rdn5_slot = var_guard26_rdn5;
        *var_guard26_rdn6_slot = var_guard26_rdn6;
        *var_guard26_rdn7_slot = var_guard26_rdn7;
        *var_guard26_rdn8_slot = var_guard26_rdn8;
        *var_guard26_rdn9_slot = var_guard26_rdn9;
        *var_guard26_rv_slot = var_guard26_rv;
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
        *var_guard27_dn16_slot = var_guard27_dn16;
        *var_guard27_dn17_slot = var_guard27_dn17;
        *var_guard27_dn18_slot = var_guard27_dn18;
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
        *var_guard27_rdn16_slot = var_guard27_rdn16;
        *var_guard27_rdn17_slot = var_guard27_rdn17;
        *var_guard27_rdn18_slot = var_guard27_rdn18;
        *var_guard27_rdn2_slot = var_guard27_rdn2;
        *var_guard27_rdn3_slot = var_guard27_rdn3;
        *var_guard27_rdn4_slot = var_guard27_rdn4;
        *var_guard27_rdn5_slot = var_guard27_rdn5;
        *var_guard27_rdn6_slot = var_guard27_rdn6;
        *var_guard27_rdn7_slot = var_guard27_rdn7;
        *var_guard27_rdn8_slot = var_guard27_rdn8;
        *var_guard27_rdn9_slot = var_guard27_rdn9;
        *var_guard27_rv_slot = var_guard27_rv;
        *var_t0_slot = var_t0;
        *var_t0_db0_slot = var_t0_db0;
        *var_t0_db1_slot = var_t0_db1;
        *var_t0_db10_slot = var_t0_db10;
        *var_t0_db11_slot = var_t0_db11;
        *var_t0_db12_slot = var_t0_db12;
        *var_t0_db13_slot = var_t0_db13;
        *var_t0_db14_slot = var_t0_db14;
        *var_t0_db15_slot = var_t0_db15;
        *var_t0_db16_slot = var_t0_db16;
        *var_t0_db17_slot = var_t0_db17;
        *var_t0_db18_slot = var_t0_db18;
        *var_t0_db2_slot = var_t0_db2;
        *var_t0_db3_slot = var_t0_db3;
        *var_t0_db4_slot = var_t0_db4;
        *var_t0_db5_slot = var_t0_db5;
        *var_t0_db6_slot = var_t0_db6;
        *var_t0_db7_slot = var_t0_db7;
        *var_t0_db8_slot = var_t0_db8;
        *var_t0_db9_slot = var_t0_db9;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn1_slot = var_t0_dn1;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn13_slot = var_t0_dn13;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn15_slot = var_t0_dn15;
        *var_t0_dn16_slot = var_t0_dn16;
        *var_t0_dn17_slot = var_t0_dn17;
        *var_t0_dn18_slot = var_t0_dn18;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rdb0_slot = var_t0_rdb0;
        *var_t0_rdb1_slot = var_t0_rdb1;
        *var_t0_rdb10_slot = var_t0_rdb10;
        *var_t0_rdb11_slot = var_t0_rdb11;
        *var_t0_rdb12_slot = var_t0_rdb12;
        *var_t0_rdb13_slot = var_t0_rdb13;
        *var_t0_rdb14_slot = var_t0_rdb14;
        *var_t0_rdb15_slot = var_t0_rdb15;
        *var_t0_rdb16_slot = var_t0_rdb16;
        *var_t0_rdb17_slot = var_t0_rdb17;
        *var_t0_rdb18_slot = var_t0_rdb18;
        *var_t0_rdb2_slot = var_t0_rdb2;
        *var_t0_rdb3_slot = var_t0_rdb3;
        *var_t0_rdb4_slot = var_t0_rdb4;
        *var_t0_rdb5_slot = var_t0_rdb5;
        *var_t0_rdb6_slot = var_t0_rdb6;
        *var_t0_rdb7_slot = var_t0_rdb7;
        *var_t0_rdb8_slot = var_t0_rdb8;
        *var_t0_rdb9_slot = var_t0_rdb9;
        *var_t0_rdn0_slot = var_t0_rdn0;
        *var_t0_rdn1_slot = var_t0_rdn1;
        *var_t0_rdn10_slot = var_t0_rdn10;
        *var_t0_rdn11_slot = var_t0_rdn11;
        *var_t0_rdn12_slot = var_t0_rdn12;
        *var_t0_rdn13_slot = var_t0_rdn13;
        *var_t0_rdn14_slot = var_t0_rdn14;
        *var_t0_rdn15_slot = var_t0_rdn15;
        *var_t0_rdn16_slot = var_t0_rdn16;
        *var_t0_rdn17_slot = var_t0_rdn17;
        *var_t0_rdn18_slot = var_t0_rdn18;
        *var_t0_rdn2_slot = var_t0_rdn2;
        *var_t0_rdn3_slot = var_t0_rdn3;
        *var_t0_rdn4_slot = var_t0_rdn4;
        *var_t0_rdn5_slot = var_t0_rdn5;
        *var_t0_rdn6_slot = var_t0_rdn6;
        *var_t0_rdn7_slot = var_t0_rdn7;
        *var_t0_rdn8_slot = var_t0_rdn8;
        *var_t0_rdn9_slot = var_t0_rdn9;
        *var_t0_rv_slot = var_t0_rv;
    }

    pub(super) fn stamp_reactive_block_39(
        p: &Parameters,
        var_ids: f64,
        var_ids_db0: f64,
        var_ids_db1: f64,
        var_ids_db10: f64,
        var_ids_db11: f64,
        var_ids_db12: f64,
        var_ids_db13: f64,
        var_ids_db14: f64,
        var_ids_db15: f64,
        var_ids_db16: f64,
        var_ids_db17: f64,
        var_ids_db18: f64,
        var_ids_db2: f64,
        var_ids_db3: f64,
        var_ids_db4: f64,
        var_ids_db5: f64,
        var_ids_db6: f64,
        var_ids_db7: f64,
        var_ids_db8: f64,
        var_ids_db9: f64,
        var_ids_dn0: f64,
        var_ids_dn1: f64,
        var_ids_dn10: f64,
        var_ids_dn11: f64,
        var_ids_dn12: f64,
        var_ids_dn13: f64,
        var_ids_dn14: f64,
        var_ids_dn15: f64,
        var_ids_dn16: f64,
        var_ids_dn17: f64,
        var_ids_dn18: f64,
        var_ids_dn2: f64,
        var_ids_dn3: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_ids_dn9: f64,
        var_igd: f64,
        var_igd_db0: f64,
        var_igd_db1: f64,
        var_igd_db10: f64,
        var_igd_db11: f64,
        var_igd_db12: f64,
        var_igd_db13: f64,
        var_igd_db14: f64,
        var_igd_db15: f64,
        var_igd_db16: f64,
        var_igd_db17: f64,
        var_igd_db18: f64,
        var_igd_db2: f64,
        var_igd_db3: f64,
        var_igd_db4: f64,
        var_igd_db5: f64,
        var_igd_db6: f64,
        var_igd_db7: f64,
        var_igd_db8: f64,
        var_igd_db9: f64,
        var_igd_dn0: f64,
        var_igd_dn1: f64,
        var_igd_dn10: f64,
        var_igd_dn11: f64,
        var_igd_dn12: f64,
        var_igd_dn13: f64,
        var_igd_dn14: f64,
        var_igd_dn15: f64,
        var_igd_dn16: f64,
        var_igd_dn17: f64,
        var_igd_dn18: f64,
        var_igd_dn2: f64,
        var_igd_dn3: f64,
        var_igd_dn4: f64,
        var_igd_dn5: f64,
        var_igd_dn6: f64,
        var_igd_dn7: f64,
        var_igd_dn8: f64,
        var_igd_dn9: f64,
        var_tanh_alpha_vds: f64,
        var_tanh_alpha_vds_db0: f64,
        var_tanh_alpha_vds_db1: f64,
        var_tanh_alpha_vds_db10: f64,
        var_tanh_alpha_vds_db11: f64,
        var_tanh_alpha_vds_db12: f64,
        var_tanh_alpha_vds_db13: f64,
        var_tanh_alpha_vds_db14: f64,
        var_tanh_alpha_vds_db15: f64,
        var_tanh_alpha_vds_db16: f64,
        var_tanh_alpha_vds_db17: f64,
        var_tanh_alpha_vds_db18: f64,
        var_tanh_alpha_vds_db2: f64,
        var_tanh_alpha_vds_db3: f64,
        var_tanh_alpha_vds_db4: f64,
        var_tanh_alpha_vds_db5: f64,
        var_tanh_alpha_vds_db6: f64,
        var_tanh_alpha_vds_db7: f64,
        var_tanh_alpha_vds_db8: f64,
        var_tanh_alpha_vds_db9: f64,
        var_tanh_alpha_vds_dn0: f64,
        var_tanh_alpha_vds_dn1: f64,
        var_tanh_alpha_vds_dn10: f64,
        var_tanh_alpha_vds_dn11: f64,
        var_tanh_alpha_vds_dn12: f64,
        var_tanh_alpha_vds_dn13: f64,
        var_tanh_alpha_vds_dn14: f64,
        var_tanh_alpha_vds_dn15: f64,
        var_tanh_alpha_vds_dn16: f64,
        var_tanh_alpha_vds_dn17: f64,
        var_tanh_alpha_vds_dn18: f64,
        var_tanh_alpha_vds_dn2: f64,
        var_tanh_alpha_vds_dn3: f64,
        var_tanh_alpha_vds_dn4: f64,
        var_tanh_alpha_vds_dn5: f64,
        var_tanh_alpha_vds_dn6: f64,
        var_tanh_alpha_vds_dn7: f64,
        var_tanh_alpha_vds_dn8: f64,
        var_tanh_alpha_vds_dn9: f64,
        var_tanh_psi: f64,
        var_tanh_psi_db0: f64,
        var_tanh_psi_db1: f64,
        var_tanh_psi_db10: f64,
        var_tanh_psi_db11: f64,
        var_tanh_psi_db12: f64,
        var_tanh_psi_db13: f64,
        var_tanh_psi_db14: f64,
        var_tanh_psi_db15: f64,
        var_tanh_psi_db16: f64,
        var_tanh_psi_db17: f64,
        var_tanh_psi_db18: f64,
        var_tanh_psi_db2: f64,
        var_tanh_psi_db3: f64,
        var_tanh_psi_db4: f64,
        var_tanh_psi_db5: f64,
        var_tanh_psi_db6: f64,
        var_tanh_psi_db7: f64,
        var_tanh_psi_db8: f64,
        var_tanh_psi_db9: f64,
        var_tanh_psi_dn0: f64,
        var_tanh_psi_dn1: f64,
        var_tanh_psi_dn10: f64,
        var_tanh_psi_dn11: f64,
        var_tanh_psi_dn12: f64,
        var_tanh_psi_dn13: f64,
        var_tanh_psi_dn14: f64,
        var_tanh_psi_dn15: f64,
        var_tanh_psi_dn16: f64,
        var_tanh_psi_dn17: f64,
        var_tanh_psi_dn18: f64,
        var_tanh_psi_dn2: f64,
        var_tanh_psi_dn3: f64,
        var_tanh_psi_dn4: f64,
        var_tanh_psi_dn5: f64,
        var_tanh_psi_dn6: f64,
        var_tanh_psi_dn7: f64,
        var_tanh_psi_dn8: f64,
        var_tanh_psi_dn9: f64,
        var_vds: f64,
        var_vds_db0: f64,
        var_vds_db1: f64,
        var_vds_db10: f64,
        var_vds_db11: f64,
        var_vds_db12: f64,
        var_vds_db13: f64,
        var_vds_db14: f64,
        var_vds_db15: f64,
        var_vds_db16: f64,
        var_vds_db17: f64,
        var_vds_db18: f64,
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
        var_vds_dn16: f64,
        var_vds_dn17: f64,
        var_vds_dn18: f64,
        var_vds_dn2: f64,
        var_vds_dn3: f64,
        var_vds_dn4: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vds_dn8: f64,
        var_vds_dn9: f64,
        var_gm_slot: &mut f64,
        var_gm_db0_slot: &mut f64,
        var_gm_db1_slot: &mut f64,
        var_gm_db10_slot: &mut f64,
        var_gm_db11_slot: &mut f64,
        var_gm_db12_slot: &mut f64,
        var_gm_db13_slot: &mut f64,
        var_gm_db14_slot: &mut f64,
        var_gm_db15_slot: &mut f64,
        var_gm_db16_slot: &mut f64,
        var_gm_db17_slot: &mut f64,
        var_gm_db18_slot: &mut f64,
        var_gm_db2_slot: &mut f64,
        var_gm_db3_slot: &mut f64,
        var_gm_db4_slot: &mut f64,
        var_gm_db5_slot: &mut f64,
        var_gm_db6_slot: &mut f64,
        var_gm_db7_slot: &mut f64,
        var_gm_db8_slot: &mut f64,
        var_gm_db9_slot: &mut f64,
        var_gm_dn0_slot: &mut f64,
        var_gm_dn1_slot: &mut f64,
        var_gm_dn10_slot: &mut f64,
        var_gm_dn11_slot: &mut f64,
        var_gm_dn12_slot: &mut f64,
        var_gm_dn13_slot: &mut f64,
        var_gm_dn14_slot: &mut f64,
        var_gm_dn15_slot: &mut f64,
        var_gm_dn16_slot: &mut f64,
        var_gm_dn17_slot: &mut f64,
        var_gm_dn18_slot: &mut f64,
        var_gm_dn2_slot: &mut f64,
        var_gm_dn3_slot: &mut f64,
        var_gm_dn4_slot: &mut f64,
        var_gm_dn5_slot: &mut f64,
        var_gm_dn6_slot: &mut f64,
        var_gm_dn7_slot: &mut f64,
        var_gm_dn8_slot: &mut f64,
        var_gm_dn9_slot: &mut f64,
        var_gm_rdb0_slot: &mut f64,
        var_gm_rdb1_slot: &mut f64,
        var_gm_rdb10_slot: &mut f64,
        var_gm_rdb11_slot: &mut f64,
        var_gm_rdb12_slot: &mut f64,
        var_gm_rdb13_slot: &mut f64,
        var_gm_rdb14_slot: &mut f64,
        var_gm_rdb15_slot: &mut f64,
        var_gm_rdb16_slot: &mut f64,
        var_gm_rdb17_slot: &mut f64,
        var_gm_rdb18_slot: &mut f64,
        var_gm_rdb2_slot: &mut f64,
        var_gm_rdb3_slot: &mut f64,
        var_gm_rdb4_slot: &mut f64,
        var_gm_rdb5_slot: &mut f64,
        var_gm_rdb6_slot: &mut f64,
        var_gm_rdb7_slot: &mut f64,
        var_gm_rdb8_slot: &mut f64,
        var_gm_rdb9_slot: &mut f64,
        var_gm_rdn0_slot: &mut f64,
        var_gm_rdn1_slot: &mut f64,
        var_gm_rdn10_slot: &mut f64,
        var_gm_rdn11_slot: &mut f64,
        var_gm_rdn12_slot: &mut f64,
        var_gm_rdn13_slot: &mut f64,
        var_gm_rdn14_slot: &mut f64,
        var_gm_rdn15_slot: &mut f64,
        var_gm_rdn16_slot: &mut f64,
        var_gm_rdn17_slot: &mut f64,
        var_gm_rdn18_slot: &mut f64,
        var_gm_rdn2_slot: &mut f64,
        var_gm_rdn3_slot: &mut f64,
        var_gm_rdn4_slot: &mut f64,
        var_gm_rdn5_slot: &mut f64,
        var_gm_rdn6_slot: &mut f64,
        var_gm_rdn7_slot: &mut f64,
        var_gm_rdn8_slot: &mut f64,
        var_gm_rdn9_slot: &mut f64,
        var_gm_rv_slot: &mut f64,
        var_guard28_slot: &mut f64,
        var_guard28_db0_slot: &mut f64,
        var_guard28_db1_slot: &mut f64,
        var_guard28_db10_slot: &mut f64,
        var_guard28_db11_slot: &mut f64,
        var_guard28_db12_slot: &mut f64,
        var_guard28_db13_slot: &mut f64,
        var_guard28_db14_slot: &mut f64,
        var_guard28_db15_slot: &mut f64,
        var_guard28_db16_slot: &mut f64,
        var_guard28_db17_slot: &mut f64,
        var_guard28_db18_slot: &mut f64,
        var_guard28_db2_slot: &mut f64,
        var_guard28_db3_slot: &mut f64,
        var_guard28_db4_slot: &mut f64,
        var_guard28_db5_slot: &mut f64,
        var_guard28_db6_slot: &mut f64,
        var_guard28_db7_slot: &mut f64,
        var_guard28_db8_slot: &mut f64,
        var_guard28_db9_slot: &mut f64,
        var_guard28_dn0_slot: &mut f64,
        var_guard28_dn1_slot: &mut f64,
        var_guard28_dn10_slot: &mut f64,
        var_guard28_dn11_slot: &mut f64,
        var_guard28_dn12_slot: &mut f64,
        var_guard28_dn13_slot: &mut f64,
        var_guard28_dn14_slot: &mut f64,
        var_guard28_dn15_slot: &mut f64,
        var_guard28_dn16_slot: &mut f64,
        var_guard28_dn17_slot: &mut f64,
        var_guard28_dn18_slot: &mut f64,
        var_guard28_dn2_slot: &mut f64,
        var_guard28_dn3_slot: &mut f64,
        var_guard28_dn4_slot: &mut f64,
        var_guard28_dn5_slot: &mut f64,
        var_guard28_dn6_slot: &mut f64,
        var_guard28_dn7_slot: &mut f64,
        var_guard28_dn8_slot: &mut f64,
        var_guard28_dn9_slot: &mut f64,
        var_guard28_rdb0_slot: &mut f64,
        var_guard28_rdb1_slot: &mut f64,
        var_guard28_rdb10_slot: &mut f64,
        var_guard28_rdb11_slot: &mut f64,
        var_guard28_rdb12_slot: &mut f64,
        var_guard28_rdb13_slot: &mut f64,
        var_guard28_rdb14_slot: &mut f64,
        var_guard28_rdb15_slot: &mut f64,
        var_guard28_rdb16_slot: &mut f64,
        var_guard28_rdb17_slot: &mut f64,
        var_guard28_rdb18_slot: &mut f64,
        var_guard28_rdb2_slot: &mut f64,
        var_guard28_rdb3_slot: &mut f64,
        var_guard28_rdb4_slot: &mut f64,
        var_guard28_rdb5_slot: &mut f64,
        var_guard28_rdb6_slot: &mut f64,
        var_guard28_rdb7_slot: &mut f64,
        var_guard28_rdb8_slot: &mut f64,
        var_guard28_rdb9_slot: &mut f64,
        var_guard28_rdn0_slot: &mut f64,
        var_guard28_rdn1_slot: &mut f64,
        var_guard28_rdn10_slot: &mut f64,
        var_guard28_rdn11_slot: &mut f64,
        var_guard28_rdn12_slot: &mut f64,
        var_guard28_rdn13_slot: &mut f64,
        var_guard28_rdn14_slot: &mut f64,
        var_guard28_rdn15_slot: &mut f64,
        var_guard28_rdn16_slot: &mut f64,
        var_guard28_rdn17_slot: &mut f64,
        var_guard28_rdn18_slot: &mut f64,
        var_guard28_rdn2_slot: &mut f64,
        var_guard28_rdn3_slot: &mut f64,
        var_guard28_rdn4_slot: &mut f64,
        var_guard28_rdn5_slot: &mut f64,
        var_guard28_rdn6_slot: &mut f64,
        var_guard28_rdn7_slot: &mut f64,
        var_guard28_rdn8_slot: &mut f64,
        var_guard28_rdn9_slot: &mut f64,
        var_guard28_rv_slot: &mut f64,
        var_guard29_slot: &mut f64,
        var_guard29_db0_slot: &mut f64,
        var_guard29_db1_slot: &mut f64,
        var_guard29_db10_slot: &mut f64,
        var_guard29_db11_slot: &mut f64,
        var_guard29_db12_slot: &mut f64,
        var_guard29_db13_slot: &mut f64,
        var_guard29_db14_slot: &mut f64,
        var_guard29_db15_slot: &mut f64,
        var_guard29_db16_slot: &mut f64,
        var_guard29_db17_slot: &mut f64,
        var_guard29_db18_slot: &mut f64,
        var_guard29_db2_slot: &mut f64,
        var_guard29_db3_slot: &mut f64,
        var_guard29_db4_slot: &mut f64,
        var_guard29_db5_slot: &mut f64,
        var_guard29_db6_slot: &mut f64,
        var_guard29_db7_slot: &mut f64,
        var_guard29_db8_slot: &mut f64,
        var_guard29_db9_slot: &mut f64,
        var_guard29_dn0_slot: &mut f64,
        var_guard29_dn1_slot: &mut f64,
        var_guard29_dn10_slot: &mut f64,
        var_guard29_dn11_slot: &mut f64,
        var_guard29_dn12_slot: &mut f64,
        var_guard29_dn13_slot: &mut f64,
        var_guard29_dn14_slot: &mut f64,
        var_guard29_dn15_slot: &mut f64,
        var_guard29_dn16_slot: &mut f64,
        var_guard29_dn17_slot: &mut f64,
        var_guard29_dn18_slot: &mut f64,
        var_guard29_dn2_slot: &mut f64,
        var_guard29_dn3_slot: &mut f64,
        var_guard29_dn4_slot: &mut f64,
        var_guard29_dn5_slot: &mut f64,
        var_guard29_dn6_slot: &mut f64,
        var_guard29_dn7_slot: &mut f64,
        var_guard29_dn8_slot: &mut f64,
        var_guard29_dn9_slot: &mut f64,
        var_guard29_rdb0_slot: &mut f64,
        var_guard29_rdb1_slot: &mut f64,
        var_guard29_rdb10_slot: &mut f64,
        var_guard29_rdb11_slot: &mut f64,
        var_guard29_rdb12_slot: &mut f64,
        var_guard29_rdb13_slot: &mut f64,
        var_guard29_rdb14_slot: &mut f64,
        var_guard29_rdb15_slot: &mut f64,
        var_guard29_rdb16_slot: &mut f64,
        var_guard29_rdb17_slot: &mut f64,
        var_guard29_rdb18_slot: &mut f64,
        var_guard29_rdb2_slot: &mut f64,
        var_guard29_rdb3_slot: &mut f64,
        var_guard29_rdb4_slot: &mut f64,
        var_guard29_rdb5_slot: &mut f64,
        var_guard29_rdb6_slot: &mut f64,
        var_guard29_rdb7_slot: &mut f64,
        var_guard29_rdb8_slot: &mut f64,
        var_guard29_rdb9_slot: &mut f64,
        var_guard29_rdn0_slot: &mut f64,
        var_guard29_rdn1_slot: &mut f64,
        var_guard29_rdn10_slot: &mut f64,
        var_guard29_rdn11_slot: &mut f64,
        var_guard29_rdn12_slot: &mut f64,
        var_guard29_rdn13_slot: &mut f64,
        var_guard29_rdn14_slot: &mut f64,
        var_guard29_rdn15_slot: &mut f64,
        var_guard29_rdn16_slot: &mut f64,
        var_guard29_rdn17_slot: &mut f64,
        var_guard29_rdn18_slot: &mut f64,
        var_guard29_rdn2_slot: &mut f64,
        var_guard29_rdn3_slot: &mut f64,
        var_guard29_rdn4_slot: &mut f64,
        var_guard29_rdn5_slot: &mut f64,
        var_guard29_rdn6_slot: &mut f64,
        var_guard29_rdn7_slot: &mut f64,
        var_guard29_rdn8_slot: &mut f64,
        var_guard29_rdn9_slot: &mut f64,
        var_guard29_rv_slot: &mut f64,
        var_idtn_slot: &mut f64,
        var_idtn_db0_slot: &mut f64,
        var_idtn_db1_slot: &mut f64,
        var_idtn_db10_slot: &mut f64,
        var_idtn_db11_slot: &mut f64,
        var_idtn_db12_slot: &mut f64,
        var_idtn_db13_slot: &mut f64,
        var_idtn_db14_slot: &mut f64,
        var_idtn_db15_slot: &mut f64,
        var_idtn_db16_slot: &mut f64,
        var_idtn_db17_slot: &mut f64,
        var_idtn_db18_slot: &mut f64,
        var_idtn_db2_slot: &mut f64,
        var_idtn_db3_slot: &mut f64,
        var_idtn_db4_slot: &mut f64,
        var_idtn_db5_slot: &mut f64,
        var_idtn_db6_slot: &mut f64,
        var_idtn_db7_slot: &mut f64,
        var_idtn_db8_slot: &mut f64,
        var_idtn_db9_slot: &mut f64,
        var_idtn_dn0_slot: &mut f64,
        var_idtn_dn1_slot: &mut f64,
        var_idtn_dn10_slot: &mut f64,
        var_idtn_dn11_slot: &mut f64,
        var_idtn_dn12_slot: &mut f64,
        var_idtn_dn13_slot: &mut f64,
        var_idtn_dn14_slot: &mut f64,
        var_idtn_dn15_slot: &mut f64,
        var_idtn_dn16_slot: &mut f64,
        var_idtn_dn17_slot: &mut f64,
        var_idtn_dn18_slot: &mut f64,
        var_idtn_dn2_slot: &mut f64,
        var_idtn_dn3_slot: &mut f64,
        var_idtn_dn4_slot: &mut f64,
        var_idtn_dn5_slot: &mut f64,
        var_idtn_dn6_slot: &mut f64,
        var_idtn_dn7_slot: &mut f64,
        var_idtn_dn8_slot: &mut f64,
        var_idtn_dn9_slot: &mut f64,
        var_idtn_rdb0_slot: &mut f64,
        var_idtn_rdb1_slot: &mut f64,
        var_idtn_rdb10_slot: &mut f64,
        var_idtn_rdb11_slot: &mut f64,
        var_idtn_rdb12_slot: &mut f64,
        var_idtn_rdb13_slot: &mut f64,
        var_idtn_rdb14_slot: &mut f64,
        var_idtn_rdb15_slot: &mut f64,
        var_idtn_rdb16_slot: &mut f64,
        var_idtn_rdb17_slot: &mut f64,
        var_idtn_rdb18_slot: &mut f64,
        var_idtn_rdb2_slot: &mut f64,
        var_idtn_rdb3_slot: &mut f64,
        var_idtn_rdb4_slot: &mut f64,
        var_idtn_rdb5_slot: &mut f64,
        var_idtn_rdb6_slot: &mut f64,
        var_idtn_rdb7_slot: &mut f64,
        var_idtn_rdb8_slot: &mut f64,
        var_idtn_rdb9_slot: &mut f64,
        var_idtn_rdn0_slot: &mut f64,
        var_idtn_rdn1_slot: &mut f64,
        var_idtn_rdn10_slot: &mut f64,
        var_idtn_rdn11_slot: &mut f64,
        var_idtn_rdn12_slot: &mut f64,
        var_idtn_rdn13_slot: &mut f64,
        var_idtn_rdn14_slot: &mut f64,
        var_idtn_rdn15_slot: &mut f64,
        var_idtn_rdn16_slot: &mut f64,
        var_idtn_rdn17_slot: &mut f64,
        var_idtn_rdn18_slot: &mut f64,
        var_idtn_rdn2_slot: &mut f64,
        var_idtn_rdn3_slot: &mut f64,
        var_idtn_rdn4_slot: &mut f64,
        var_idtn_rdn5_slot: &mut f64,
        var_idtn_rdn6_slot: &mut f64,
        var_idtn_rdn7_slot: &mut f64,
        var_idtn_rdn8_slot: &mut f64,
        var_idtn_rdn9_slot: &mut f64,
        var_idtn_rv_slot: &mut f64,
        var_td_prime_slot: &mut f64,
        var_td_prime_db0_slot: &mut f64,
        var_td_prime_db1_slot: &mut f64,
        var_td_prime_db10_slot: &mut f64,
        var_td_prime_db11_slot: &mut f64,
        var_td_prime_db12_slot: &mut f64,
        var_td_prime_db13_slot: &mut f64,
        var_td_prime_db14_slot: &mut f64,
        var_td_prime_db15_slot: &mut f64,
        var_td_prime_db16_slot: &mut f64,
        var_td_prime_db17_slot: &mut f64,
        var_td_prime_db18_slot: &mut f64,
        var_td_prime_db2_slot: &mut f64,
        var_td_prime_db3_slot: &mut f64,
        var_td_prime_db4_slot: &mut f64,
        var_td_prime_db5_slot: &mut f64,
        var_td_prime_db6_slot: &mut f64,
        var_td_prime_db7_slot: &mut f64,
        var_td_prime_db8_slot: &mut f64,
        var_td_prime_db9_slot: &mut f64,
        var_td_prime_dn0_slot: &mut f64,
        var_td_prime_dn1_slot: &mut f64,
        var_td_prime_dn10_slot: &mut f64,
        var_td_prime_dn11_slot: &mut f64,
        var_td_prime_dn12_slot: &mut f64,
        var_td_prime_dn13_slot: &mut f64,
        var_td_prime_dn14_slot: &mut f64,
        var_td_prime_dn15_slot: &mut f64,
        var_td_prime_dn16_slot: &mut f64,
        var_td_prime_dn17_slot: &mut f64,
        var_td_prime_dn18_slot: &mut f64,
        var_td_prime_dn2_slot: &mut f64,
        var_td_prime_dn3_slot: &mut f64,
        var_td_prime_dn4_slot: &mut f64,
        var_td_prime_dn5_slot: &mut f64,
        var_td_prime_dn6_slot: &mut f64,
        var_td_prime_dn7_slot: &mut f64,
        var_td_prime_dn8_slot: &mut f64,
        var_td_prime_dn9_slot: &mut f64,
        var_td_prime_rdb0_slot: &mut f64,
        var_td_prime_rdb1_slot: &mut f64,
        var_td_prime_rdb10_slot: &mut f64,
        var_td_prime_rdb11_slot: &mut f64,
        var_td_prime_rdb12_slot: &mut f64,
        var_td_prime_rdb13_slot: &mut f64,
        var_td_prime_rdb14_slot: &mut f64,
        var_td_prime_rdb15_slot: &mut f64,
        var_td_prime_rdb16_slot: &mut f64,
        var_td_prime_rdb17_slot: &mut f64,
        var_td_prime_rdb18_slot: &mut f64,
        var_td_prime_rdb2_slot: &mut f64,
        var_td_prime_rdb3_slot: &mut f64,
        var_td_prime_rdb4_slot: &mut f64,
        var_td_prime_rdb5_slot: &mut f64,
        var_td_prime_rdb6_slot: &mut f64,
        var_td_prime_rdb7_slot: &mut f64,
        var_td_prime_rdb8_slot: &mut f64,
        var_td_prime_rdb9_slot: &mut f64,
        var_td_prime_rdn0_slot: &mut f64,
        var_td_prime_rdn1_slot: &mut f64,
        var_td_prime_rdn10_slot: &mut f64,
        var_td_prime_rdn11_slot: &mut f64,
        var_td_prime_rdn12_slot: &mut f64,
        var_td_prime_rdn13_slot: &mut f64,
        var_td_prime_rdn14_slot: &mut f64,
        var_td_prime_rdn15_slot: &mut f64,
        var_td_prime_rdn16_slot: &mut f64,
        var_td_prime_rdn17_slot: &mut f64,
        var_td_prime_rdn18_slot: &mut f64,
        var_td_prime_rdn2_slot: &mut f64,
        var_td_prime_rdn3_slot: &mut f64,
        var_td_prime_rdn4_slot: &mut f64,
        var_td_prime_rdn5_slot: &mut f64,
        var_td_prime_rdn6_slot: &mut f64,
        var_td_prime_rdn7_slot: &mut f64,
        var_td_prime_rdn8_slot: &mut f64,
        var_td_prime_rdn9_slot: &mut f64,
        var_td_prime_rv_slot: &mut f64,
    ) {
        let mut var_gm: f64 = *var_gm_slot;
        let mut var_gm_db0: f64 = *var_gm_db0_slot;
        let mut var_gm_db1: f64 = *var_gm_db1_slot;
        let mut var_gm_db10: f64 = *var_gm_db10_slot;
        let mut var_gm_db11: f64 = *var_gm_db11_slot;
        let mut var_gm_db12: f64 = *var_gm_db12_slot;
        let mut var_gm_db13: f64 = *var_gm_db13_slot;
        let mut var_gm_db14: f64 = *var_gm_db14_slot;
        let mut var_gm_db15: f64 = *var_gm_db15_slot;
        let mut var_gm_db16: f64 = *var_gm_db16_slot;
        let mut var_gm_db17: f64 = *var_gm_db17_slot;
        let mut var_gm_db18: f64 = *var_gm_db18_slot;
        let mut var_gm_db2: f64 = *var_gm_db2_slot;
        let mut var_gm_db3: f64 = *var_gm_db3_slot;
        let mut var_gm_db4: f64 = *var_gm_db4_slot;
        let mut var_gm_db5: f64 = *var_gm_db5_slot;
        let mut var_gm_db6: f64 = *var_gm_db6_slot;
        let mut var_gm_db7: f64 = *var_gm_db7_slot;
        let mut var_gm_db8: f64 = *var_gm_db8_slot;
        let mut var_gm_db9: f64 = *var_gm_db9_slot;
        let mut var_gm_dn0: f64 = *var_gm_dn0_slot;
        let mut var_gm_dn1: f64 = *var_gm_dn1_slot;
        let mut var_gm_dn10: f64 = *var_gm_dn10_slot;
        let mut var_gm_dn11: f64 = *var_gm_dn11_slot;
        let mut var_gm_dn12: f64 = *var_gm_dn12_slot;
        let mut var_gm_dn13: f64 = *var_gm_dn13_slot;
        let mut var_gm_dn14: f64 = *var_gm_dn14_slot;
        let mut var_gm_dn15: f64 = *var_gm_dn15_slot;
        let mut var_gm_dn16: f64 = *var_gm_dn16_slot;
        let mut var_gm_dn17: f64 = *var_gm_dn17_slot;
        let mut var_gm_dn18: f64 = *var_gm_dn18_slot;
        let mut var_gm_dn2: f64 = *var_gm_dn2_slot;
        let mut var_gm_dn3: f64 = *var_gm_dn3_slot;
        let mut var_gm_dn4: f64 = *var_gm_dn4_slot;
        let mut var_gm_dn5: f64 = *var_gm_dn5_slot;
        let mut var_gm_dn6: f64 = *var_gm_dn6_slot;
        let mut var_gm_dn7: f64 = *var_gm_dn7_slot;
        let mut var_gm_dn8: f64 = *var_gm_dn8_slot;
        let mut var_gm_dn9: f64 = *var_gm_dn9_slot;
        let mut var_gm_rdb0: f64 = *var_gm_rdb0_slot;
        let mut var_gm_rdb1: f64 = *var_gm_rdb1_slot;
        let mut var_gm_rdb10: f64 = *var_gm_rdb10_slot;
        let mut var_gm_rdb11: f64 = *var_gm_rdb11_slot;
        let mut var_gm_rdb12: f64 = *var_gm_rdb12_slot;
        let mut var_gm_rdb13: f64 = *var_gm_rdb13_slot;
        let mut var_gm_rdb14: f64 = *var_gm_rdb14_slot;
        let mut var_gm_rdb15: f64 = *var_gm_rdb15_slot;
        let mut var_gm_rdb16: f64 = *var_gm_rdb16_slot;
        let mut var_gm_rdb17: f64 = *var_gm_rdb17_slot;
        let mut var_gm_rdb18: f64 = *var_gm_rdb18_slot;
        let mut var_gm_rdb2: f64 = *var_gm_rdb2_slot;
        let mut var_gm_rdb3: f64 = *var_gm_rdb3_slot;
        let mut var_gm_rdb4: f64 = *var_gm_rdb4_slot;
        let mut var_gm_rdb5: f64 = *var_gm_rdb5_slot;
        let mut var_gm_rdb6: f64 = *var_gm_rdb6_slot;
        let mut var_gm_rdb7: f64 = *var_gm_rdb7_slot;
        let mut var_gm_rdb8: f64 = *var_gm_rdb8_slot;
        let mut var_gm_rdb9: f64 = *var_gm_rdb9_slot;
        let mut var_gm_rdn0: f64 = *var_gm_rdn0_slot;
        let mut var_gm_rdn1: f64 = *var_gm_rdn1_slot;
        let mut var_gm_rdn10: f64 = *var_gm_rdn10_slot;
        let mut var_gm_rdn11: f64 = *var_gm_rdn11_slot;
        let mut var_gm_rdn12: f64 = *var_gm_rdn12_slot;
        let mut var_gm_rdn13: f64 = *var_gm_rdn13_slot;
        let mut var_gm_rdn14: f64 = *var_gm_rdn14_slot;
        let mut var_gm_rdn15: f64 = *var_gm_rdn15_slot;
        let mut var_gm_rdn16: f64 = *var_gm_rdn16_slot;
        let mut var_gm_rdn17: f64 = *var_gm_rdn17_slot;
        let mut var_gm_rdn18: f64 = *var_gm_rdn18_slot;
        let mut var_gm_rdn2: f64 = *var_gm_rdn2_slot;
        let mut var_gm_rdn3: f64 = *var_gm_rdn3_slot;
        let mut var_gm_rdn4: f64 = *var_gm_rdn4_slot;
        let mut var_gm_rdn5: f64 = *var_gm_rdn5_slot;
        let mut var_gm_rdn6: f64 = *var_gm_rdn6_slot;
        let mut var_gm_rdn7: f64 = *var_gm_rdn7_slot;
        let mut var_gm_rdn8: f64 = *var_gm_rdn8_slot;
        let mut var_gm_rdn9: f64 = *var_gm_rdn9_slot;
        let mut var_gm_rv: f64 = *var_gm_rv_slot;
        let mut var_guard28: f64 = *var_guard28_slot;
        let mut var_guard28_db0: f64 = *var_guard28_db0_slot;
        let mut var_guard28_db1: f64 = *var_guard28_db1_slot;
        let mut var_guard28_db10: f64 = *var_guard28_db10_slot;
        let mut var_guard28_db11: f64 = *var_guard28_db11_slot;
        let mut var_guard28_db12: f64 = *var_guard28_db12_slot;
        let mut var_guard28_db13: f64 = *var_guard28_db13_slot;
        let mut var_guard28_db14: f64 = *var_guard28_db14_slot;
        let mut var_guard28_db15: f64 = *var_guard28_db15_slot;
        let mut var_guard28_db16: f64 = *var_guard28_db16_slot;
        let mut var_guard28_db17: f64 = *var_guard28_db17_slot;
        let mut var_guard28_db18: f64 = *var_guard28_db18_slot;
        let mut var_guard28_db2: f64 = *var_guard28_db2_slot;
        let mut var_guard28_db3: f64 = *var_guard28_db3_slot;
        let mut var_guard28_db4: f64 = *var_guard28_db4_slot;
        let mut var_guard28_db5: f64 = *var_guard28_db5_slot;
        let mut var_guard28_db6: f64 = *var_guard28_db6_slot;
        let mut var_guard28_db7: f64 = *var_guard28_db7_slot;
        let mut var_guard28_db8: f64 = *var_guard28_db8_slot;
        let mut var_guard28_db9: f64 = *var_guard28_db9_slot;
        let mut var_guard28_dn0: f64 = *var_guard28_dn0_slot;
        let mut var_guard28_dn1: f64 = *var_guard28_dn1_slot;
        let mut var_guard28_dn10: f64 = *var_guard28_dn10_slot;
        let mut var_guard28_dn11: f64 = *var_guard28_dn11_slot;
        let mut var_guard28_dn12: f64 = *var_guard28_dn12_slot;
        let mut var_guard28_dn13: f64 = *var_guard28_dn13_slot;
        let mut var_guard28_dn14: f64 = *var_guard28_dn14_slot;
        let mut var_guard28_dn15: f64 = *var_guard28_dn15_slot;
        let mut var_guard28_dn16: f64 = *var_guard28_dn16_slot;
        let mut var_guard28_dn17: f64 = *var_guard28_dn17_slot;
        let mut var_guard28_dn18: f64 = *var_guard28_dn18_slot;
        let mut var_guard28_dn2: f64 = *var_guard28_dn2_slot;
        let mut var_guard28_dn3: f64 = *var_guard28_dn3_slot;
        let mut var_guard28_dn4: f64 = *var_guard28_dn4_slot;
        let mut var_guard28_dn5: f64 = *var_guard28_dn5_slot;
        let mut var_guard28_dn6: f64 = *var_guard28_dn6_slot;
        let mut var_guard28_dn7: f64 = *var_guard28_dn7_slot;
        let mut var_guard28_dn8: f64 = *var_guard28_dn8_slot;
        let mut var_guard28_dn9: f64 = *var_guard28_dn9_slot;
        let mut var_guard28_rdb0: f64 = *var_guard28_rdb0_slot;
        let mut var_guard28_rdb1: f64 = *var_guard28_rdb1_slot;
        let mut var_guard28_rdb10: f64 = *var_guard28_rdb10_slot;
        let mut var_guard28_rdb11: f64 = *var_guard28_rdb11_slot;
        let mut var_guard28_rdb12: f64 = *var_guard28_rdb12_slot;
        let mut var_guard28_rdb13: f64 = *var_guard28_rdb13_slot;
        let mut var_guard28_rdb14: f64 = *var_guard28_rdb14_slot;
        let mut var_guard28_rdb15: f64 = *var_guard28_rdb15_slot;
        let mut var_guard28_rdb16: f64 = *var_guard28_rdb16_slot;
        let mut var_guard28_rdb17: f64 = *var_guard28_rdb17_slot;
        let mut var_guard28_rdb18: f64 = *var_guard28_rdb18_slot;
        let mut var_guard28_rdb2: f64 = *var_guard28_rdb2_slot;
        let mut var_guard28_rdb3: f64 = *var_guard28_rdb3_slot;
        let mut var_guard28_rdb4: f64 = *var_guard28_rdb4_slot;
        let mut var_guard28_rdb5: f64 = *var_guard28_rdb5_slot;
        let mut var_guard28_rdb6: f64 = *var_guard28_rdb6_slot;
        let mut var_guard28_rdb7: f64 = *var_guard28_rdb7_slot;
        let mut var_guard28_rdb8: f64 = *var_guard28_rdb8_slot;
        let mut var_guard28_rdb9: f64 = *var_guard28_rdb9_slot;
        let mut var_guard28_rdn0: f64 = *var_guard28_rdn0_slot;
        let mut var_guard28_rdn1: f64 = *var_guard28_rdn1_slot;
        let mut var_guard28_rdn10: f64 = *var_guard28_rdn10_slot;
        let mut var_guard28_rdn11: f64 = *var_guard28_rdn11_slot;
        let mut var_guard28_rdn12: f64 = *var_guard28_rdn12_slot;
        let mut var_guard28_rdn13: f64 = *var_guard28_rdn13_slot;
        let mut var_guard28_rdn14: f64 = *var_guard28_rdn14_slot;
        let mut var_guard28_rdn15: f64 = *var_guard28_rdn15_slot;
        let mut var_guard28_rdn16: f64 = *var_guard28_rdn16_slot;
        let mut var_guard28_rdn17: f64 = *var_guard28_rdn17_slot;
        let mut var_guard28_rdn18: f64 = *var_guard28_rdn18_slot;
        let mut var_guard28_rdn2: f64 = *var_guard28_rdn2_slot;
        let mut var_guard28_rdn3: f64 = *var_guard28_rdn3_slot;
        let mut var_guard28_rdn4: f64 = *var_guard28_rdn4_slot;
        let mut var_guard28_rdn5: f64 = *var_guard28_rdn5_slot;
        let mut var_guard28_rdn6: f64 = *var_guard28_rdn6_slot;
        let mut var_guard28_rdn7: f64 = *var_guard28_rdn7_slot;
        let mut var_guard28_rdn8: f64 = *var_guard28_rdn8_slot;
        let mut var_guard28_rdn9: f64 = *var_guard28_rdn9_slot;
        let mut var_guard28_rv: f64 = *var_guard28_rv_slot;
        let mut var_guard29: f64 = *var_guard29_slot;
        let mut var_guard29_db0: f64 = *var_guard29_db0_slot;
        let mut var_guard29_db1: f64 = *var_guard29_db1_slot;
        let mut var_guard29_db10: f64 = *var_guard29_db10_slot;
        let mut var_guard29_db11: f64 = *var_guard29_db11_slot;
        let mut var_guard29_db12: f64 = *var_guard29_db12_slot;
        let mut var_guard29_db13: f64 = *var_guard29_db13_slot;
        let mut var_guard29_db14: f64 = *var_guard29_db14_slot;
        let mut var_guard29_db15: f64 = *var_guard29_db15_slot;
        let mut var_guard29_db16: f64 = *var_guard29_db16_slot;
        let mut var_guard29_db17: f64 = *var_guard29_db17_slot;
        let mut var_guard29_db18: f64 = *var_guard29_db18_slot;
        let mut var_guard29_db2: f64 = *var_guard29_db2_slot;
        let mut var_guard29_db3: f64 = *var_guard29_db3_slot;
        let mut var_guard29_db4: f64 = *var_guard29_db4_slot;
        let mut var_guard29_db5: f64 = *var_guard29_db5_slot;
        let mut var_guard29_db6: f64 = *var_guard29_db6_slot;
        let mut var_guard29_db7: f64 = *var_guard29_db7_slot;
        let mut var_guard29_db8: f64 = *var_guard29_db8_slot;
        let mut var_guard29_db9: f64 = *var_guard29_db9_slot;
        let mut var_guard29_dn0: f64 = *var_guard29_dn0_slot;
        let mut var_guard29_dn1: f64 = *var_guard29_dn1_slot;
        let mut var_guard29_dn10: f64 = *var_guard29_dn10_slot;
        let mut var_guard29_dn11: f64 = *var_guard29_dn11_slot;
        let mut var_guard29_dn12: f64 = *var_guard29_dn12_slot;
        let mut var_guard29_dn13: f64 = *var_guard29_dn13_slot;
        let mut var_guard29_dn14: f64 = *var_guard29_dn14_slot;
        let mut var_guard29_dn15: f64 = *var_guard29_dn15_slot;
        let mut var_guard29_dn16: f64 = *var_guard29_dn16_slot;
        let mut var_guard29_dn17: f64 = *var_guard29_dn17_slot;
        let mut var_guard29_dn18: f64 = *var_guard29_dn18_slot;
        let mut var_guard29_dn2: f64 = *var_guard29_dn2_slot;
        let mut var_guard29_dn3: f64 = *var_guard29_dn3_slot;
        let mut var_guard29_dn4: f64 = *var_guard29_dn4_slot;
        let mut var_guard29_dn5: f64 = *var_guard29_dn5_slot;
        let mut var_guard29_dn6: f64 = *var_guard29_dn6_slot;
        let mut var_guard29_dn7: f64 = *var_guard29_dn7_slot;
        let mut var_guard29_dn8: f64 = *var_guard29_dn8_slot;
        let mut var_guard29_dn9: f64 = *var_guard29_dn9_slot;
        let mut var_guard29_rdb0: f64 = *var_guard29_rdb0_slot;
        let mut var_guard29_rdb1: f64 = *var_guard29_rdb1_slot;
        let mut var_guard29_rdb10: f64 = *var_guard29_rdb10_slot;
        let mut var_guard29_rdb11: f64 = *var_guard29_rdb11_slot;
        let mut var_guard29_rdb12: f64 = *var_guard29_rdb12_slot;
        let mut var_guard29_rdb13: f64 = *var_guard29_rdb13_slot;
        let mut var_guard29_rdb14: f64 = *var_guard29_rdb14_slot;
        let mut var_guard29_rdb15: f64 = *var_guard29_rdb15_slot;
        let mut var_guard29_rdb16: f64 = *var_guard29_rdb16_slot;
        let mut var_guard29_rdb17: f64 = *var_guard29_rdb17_slot;
        let mut var_guard29_rdb18: f64 = *var_guard29_rdb18_slot;
        let mut var_guard29_rdb2: f64 = *var_guard29_rdb2_slot;
        let mut var_guard29_rdb3: f64 = *var_guard29_rdb3_slot;
        let mut var_guard29_rdb4: f64 = *var_guard29_rdb4_slot;
        let mut var_guard29_rdb5: f64 = *var_guard29_rdb5_slot;
        let mut var_guard29_rdb6: f64 = *var_guard29_rdb6_slot;
        let mut var_guard29_rdb7: f64 = *var_guard29_rdb7_slot;
        let mut var_guard29_rdb8: f64 = *var_guard29_rdb8_slot;
        let mut var_guard29_rdb9: f64 = *var_guard29_rdb9_slot;
        let mut var_guard29_rdn0: f64 = *var_guard29_rdn0_slot;
        let mut var_guard29_rdn1: f64 = *var_guard29_rdn1_slot;
        let mut var_guard29_rdn10: f64 = *var_guard29_rdn10_slot;
        let mut var_guard29_rdn11: f64 = *var_guard29_rdn11_slot;
        let mut var_guard29_rdn12: f64 = *var_guard29_rdn12_slot;
        let mut var_guard29_rdn13: f64 = *var_guard29_rdn13_slot;
        let mut var_guard29_rdn14: f64 = *var_guard29_rdn14_slot;
        let mut var_guard29_rdn15: f64 = *var_guard29_rdn15_slot;
        let mut var_guard29_rdn16: f64 = *var_guard29_rdn16_slot;
        let mut var_guard29_rdn17: f64 = *var_guard29_rdn17_slot;
        let mut var_guard29_rdn18: f64 = *var_guard29_rdn18_slot;
        let mut var_guard29_rdn2: f64 = *var_guard29_rdn2_slot;
        let mut var_guard29_rdn3: f64 = *var_guard29_rdn3_slot;
        let mut var_guard29_rdn4: f64 = *var_guard29_rdn4_slot;
        let mut var_guard29_rdn5: f64 = *var_guard29_rdn5_slot;
        let mut var_guard29_rdn6: f64 = *var_guard29_rdn6_slot;
        let mut var_guard29_rdn7: f64 = *var_guard29_rdn7_slot;
        let mut var_guard29_rdn8: f64 = *var_guard29_rdn8_slot;
        let mut var_guard29_rdn9: f64 = *var_guard29_rdn9_slot;
        let mut var_guard29_rv: f64 = *var_guard29_rv_slot;
        let mut var_idtn: f64 = *var_idtn_slot;
        let mut var_idtn_db0: f64 = *var_idtn_db0_slot;
        let mut var_idtn_db1: f64 = *var_idtn_db1_slot;
        let mut var_idtn_db10: f64 = *var_idtn_db10_slot;
        let mut var_idtn_db11: f64 = *var_idtn_db11_slot;
        let mut var_idtn_db12: f64 = *var_idtn_db12_slot;
        let mut var_idtn_db13: f64 = *var_idtn_db13_slot;
        let mut var_idtn_db14: f64 = *var_idtn_db14_slot;
        let mut var_idtn_db15: f64 = *var_idtn_db15_slot;
        let mut var_idtn_db16: f64 = *var_idtn_db16_slot;
        let mut var_idtn_db17: f64 = *var_idtn_db17_slot;
        let mut var_idtn_db18: f64 = *var_idtn_db18_slot;
        let mut var_idtn_db2: f64 = *var_idtn_db2_slot;
        let mut var_idtn_db3: f64 = *var_idtn_db3_slot;
        let mut var_idtn_db4: f64 = *var_idtn_db4_slot;
        let mut var_idtn_db5: f64 = *var_idtn_db5_slot;
        let mut var_idtn_db6: f64 = *var_idtn_db6_slot;
        let mut var_idtn_db7: f64 = *var_idtn_db7_slot;
        let mut var_idtn_db8: f64 = *var_idtn_db8_slot;
        let mut var_idtn_db9: f64 = *var_idtn_db9_slot;
        let mut var_idtn_dn0: f64 = *var_idtn_dn0_slot;
        let mut var_idtn_dn1: f64 = *var_idtn_dn1_slot;
        let mut var_idtn_dn10: f64 = *var_idtn_dn10_slot;
        let mut var_idtn_dn11: f64 = *var_idtn_dn11_slot;
        let mut var_idtn_dn12: f64 = *var_idtn_dn12_slot;
        let mut var_idtn_dn13: f64 = *var_idtn_dn13_slot;
        let mut var_idtn_dn14: f64 = *var_idtn_dn14_slot;
        let mut var_idtn_dn15: f64 = *var_idtn_dn15_slot;
        let mut var_idtn_dn16: f64 = *var_idtn_dn16_slot;
        let mut var_idtn_dn17: f64 = *var_idtn_dn17_slot;
        let mut var_idtn_dn18: f64 = *var_idtn_dn18_slot;
        let mut var_idtn_dn2: f64 = *var_idtn_dn2_slot;
        let mut var_idtn_dn3: f64 = *var_idtn_dn3_slot;
        let mut var_idtn_dn4: f64 = *var_idtn_dn4_slot;
        let mut var_idtn_dn5: f64 = *var_idtn_dn5_slot;
        let mut var_idtn_dn6: f64 = *var_idtn_dn6_slot;
        let mut var_idtn_dn7: f64 = *var_idtn_dn7_slot;
        let mut var_idtn_dn8: f64 = *var_idtn_dn8_slot;
        let mut var_idtn_dn9: f64 = *var_idtn_dn9_slot;
        let mut var_idtn_rdb0: f64 = *var_idtn_rdb0_slot;
        let mut var_idtn_rdb1: f64 = *var_idtn_rdb1_slot;
        let mut var_idtn_rdb10: f64 = *var_idtn_rdb10_slot;
        let mut var_idtn_rdb11: f64 = *var_idtn_rdb11_slot;
        let mut var_idtn_rdb12: f64 = *var_idtn_rdb12_slot;
        let mut var_idtn_rdb13: f64 = *var_idtn_rdb13_slot;
        let mut var_idtn_rdb14: f64 = *var_idtn_rdb14_slot;
        let mut var_idtn_rdb15: f64 = *var_idtn_rdb15_slot;
        let mut var_idtn_rdb16: f64 = *var_idtn_rdb16_slot;
        let mut var_idtn_rdb17: f64 = *var_idtn_rdb17_slot;
        let mut var_idtn_rdb18: f64 = *var_idtn_rdb18_slot;
        let mut var_idtn_rdb2: f64 = *var_idtn_rdb2_slot;
        let mut var_idtn_rdb3: f64 = *var_idtn_rdb3_slot;
        let mut var_idtn_rdb4: f64 = *var_idtn_rdb4_slot;
        let mut var_idtn_rdb5: f64 = *var_idtn_rdb5_slot;
        let mut var_idtn_rdb6: f64 = *var_idtn_rdb6_slot;
        let mut var_idtn_rdb7: f64 = *var_idtn_rdb7_slot;
        let mut var_idtn_rdb8: f64 = *var_idtn_rdb8_slot;
        let mut var_idtn_rdb9: f64 = *var_idtn_rdb9_slot;
        let mut var_idtn_rdn0: f64 = *var_idtn_rdn0_slot;
        let mut var_idtn_rdn1: f64 = *var_idtn_rdn1_slot;
        let mut var_idtn_rdn10: f64 = *var_idtn_rdn10_slot;
        let mut var_idtn_rdn11: f64 = *var_idtn_rdn11_slot;
        let mut var_idtn_rdn12: f64 = *var_idtn_rdn12_slot;
        let mut var_idtn_rdn13: f64 = *var_idtn_rdn13_slot;
        let mut var_idtn_rdn14: f64 = *var_idtn_rdn14_slot;
        let mut var_idtn_rdn15: f64 = *var_idtn_rdn15_slot;
        let mut var_idtn_rdn16: f64 = *var_idtn_rdn16_slot;
        let mut var_idtn_rdn17: f64 = *var_idtn_rdn17_slot;
        let mut var_idtn_rdn18: f64 = *var_idtn_rdn18_slot;
        let mut var_idtn_rdn2: f64 = *var_idtn_rdn2_slot;
        let mut var_idtn_rdn3: f64 = *var_idtn_rdn3_slot;
        let mut var_idtn_rdn4: f64 = *var_idtn_rdn4_slot;
        let mut var_idtn_rdn5: f64 = *var_idtn_rdn5_slot;
        let mut var_idtn_rdn6: f64 = *var_idtn_rdn6_slot;
        let mut var_idtn_rdn7: f64 = *var_idtn_rdn7_slot;
        let mut var_idtn_rdn8: f64 = *var_idtn_rdn8_slot;
        let mut var_idtn_rdn9: f64 = *var_idtn_rdn9_slot;
        let mut var_idtn_rv: f64 = *var_idtn_rv_slot;
        let mut var_td_prime: f64 = *var_td_prime_slot;
        let mut var_td_prime_db0: f64 = *var_td_prime_db0_slot;
        let mut var_td_prime_db1: f64 = *var_td_prime_db1_slot;
        let mut var_td_prime_db10: f64 = *var_td_prime_db10_slot;
        let mut var_td_prime_db11: f64 = *var_td_prime_db11_slot;
        let mut var_td_prime_db12: f64 = *var_td_prime_db12_slot;
        let mut var_td_prime_db13: f64 = *var_td_prime_db13_slot;
        let mut var_td_prime_db14: f64 = *var_td_prime_db14_slot;
        let mut var_td_prime_db15: f64 = *var_td_prime_db15_slot;
        let mut var_td_prime_db16: f64 = *var_td_prime_db16_slot;
        let mut var_td_prime_db17: f64 = *var_td_prime_db17_slot;
        let mut var_td_prime_db18: f64 = *var_td_prime_db18_slot;
        let mut var_td_prime_db2: f64 = *var_td_prime_db2_slot;
        let mut var_td_prime_db3: f64 = *var_td_prime_db3_slot;
        let mut var_td_prime_db4: f64 = *var_td_prime_db4_slot;
        let mut var_td_prime_db5: f64 = *var_td_prime_db5_slot;
        let mut var_td_prime_db6: f64 = *var_td_prime_db6_slot;
        let mut var_td_prime_db7: f64 = *var_td_prime_db7_slot;
        let mut var_td_prime_db8: f64 = *var_td_prime_db8_slot;
        let mut var_td_prime_db9: f64 = *var_td_prime_db9_slot;
        let mut var_td_prime_dn0: f64 = *var_td_prime_dn0_slot;
        let mut var_td_prime_dn1: f64 = *var_td_prime_dn1_slot;
        let mut var_td_prime_dn10: f64 = *var_td_prime_dn10_slot;
        let mut var_td_prime_dn11: f64 = *var_td_prime_dn11_slot;
        let mut var_td_prime_dn12: f64 = *var_td_prime_dn12_slot;
        let mut var_td_prime_dn13: f64 = *var_td_prime_dn13_slot;
        let mut var_td_prime_dn14: f64 = *var_td_prime_dn14_slot;
        let mut var_td_prime_dn15: f64 = *var_td_prime_dn15_slot;
        let mut var_td_prime_dn16: f64 = *var_td_prime_dn16_slot;
        let mut var_td_prime_dn17: f64 = *var_td_prime_dn17_slot;
        let mut var_td_prime_dn18: f64 = *var_td_prime_dn18_slot;
        let mut var_td_prime_dn2: f64 = *var_td_prime_dn2_slot;
        let mut var_td_prime_dn3: f64 = *var_td_prime_dn3_slot;
        let mut var_td_prime_dn4: f64 = *var_td_prime_dn4_slot;
        let mut var_td_prime_dn5: f64 = *var_td_prime_dn5_slot;
        let mut var_td_prime_dn6: f64 = *var_td_prime_dn6_slot;
        let mut var_td_prime_dn7: f64 = *var_td_prime_dn7_slot;
        let mut var_td_prime_dn8: f64 = *var_td_prime_dn8_slot;
        let mut var_td_prime_dn9: f64 = *var_td_prime_dn9_slot;
        let mut var_td_prime_rdb0: f64 = *var_td_prime_rdb0_slot;
        let mut var_td_prime_rdb1: f64 = *var_td_prime_rdb1_slot;
        let mut var_td_prime_rdb10: f64 = *var_td_prime_rdb10_slot;
        let mut var_td_prime_rdb11: f64 = *var_td_prime_rdb11_slot;
        let mut var_td_prime_rdb12: f64 = *var_td_prime_rdb12_slot;
        let mut var_td_prime_rdb13: f64 = *var_td_prime_rdb13_slot;
        let mut var_td_prime_rdb14: f64 = *var_td_prime_rdb14_slot;
        let mut var_td_prime_rdb15: f64 = *var_td_prime_rdb15_slot;
        let mut var_td_prime_rdb16: f64 = *var_td_prime_rdb16_slot;
        let mut var_td_prime_rdb17: f64 = *var_td_prime_rdb17_slot;
        let mut var_td_prime_rdb18: f64 = *var_td_prime_rdb18_slot;
        let mut var_td_prime_rdb2: f64 = *var_td_prime_rdb2_slot;
        let mut var_td_prime_rdb3: f64 = *var_td_prime_rdb3_slot;
        let mut var_td_prime_rdb4: f64 = *var_td_prime_rdb4_slot;
        let mut var_td_prime_rdb5: f64 = *var_td_prime_rdb5_slot;
        let mut var_td_prime_rdb6: f64 = *var_td_prime_rdb6_slot;
        let mut var_td_prime_rdb7: f64 = *var_td_prime_rdb7_slot;
        let mut var_td_prime_rdb8: f64 = *var_td_prime_rdb8_slot;
        let mut var_td_prime_rdb9: f64 = *var_td_prime_rdb9_slot;
        let mut var_td_prime_rdn0: f64 = *var_td_prime_rdn0_slot;
        let mut var_td_prime_rdn1: f64 = *var_td_prime_rdn1_slot;
        let mut var_td_prime_rdn10: f64 = *var_td_prime_rdn10_slot;
        let mut var_td_prime_rdn11: f64 = *var_td_prime_rdn11_slot;
        let mut var_td_prime_rdn12: f64 = *var_td_prime_rdn12_slot;
        let mut var_td_prime_rdn13: f64 = *var_td_prime_rdn13_slot;
        let mut var_td_prime_rdn14: f64 = *var_td_prime_rdn14_slot;
        let mut var_td_prime_rdn15: f64 = *var_td_prime_rdn15_slot;
        let mut var_td_prime_rdn16: f64 = *var_td_prime_rdn16_slot;
        let mut var_td_prime_rdn17: f64 = *var_td_prime_rdn17_slot;
        let mut var_td_prime_rdn18: f64 = *var_td_prime_rdn18_slot;
        let mut var_td_prime_rdn2: f64 = *var_td_prime_rdn2_slot;
        let mut var_td_prime_rdn3: f64 = *var_td_prime_rdn3_slot;
        let mut var_td_prime_rdn4: f64 = *var_td_prime_rdn4_slot;
        let mut var_td_prime_rdn5: f64 = *var_td_prime_rdn5_slot;
        let mut var_td_prime_rdn6: f64 = *var_td_prime_rdn6_slot;
        let mut var_td_prime_rdn7: f64 = *var_td_prime_rdn7_slot;
        let mut var_td_prime_rdn8: f64 = *var_td_prime_rdn8_slot;
        let mut var_td_prime_rdn9: f64 = *var_td_prime_rdn9_slot;
        let mut var_td_prime_rv: f64 = *var_td_prime_rv_slot;

        let assign2190_e2874: f64 = (var_gm * p.p50);
        let assign2190_e2875: f64 = (1.0 + assign2190_e2874);
        let assign2190_e2876: f64 = (var_gm / assign2190_e2875);
        var_gm = assign2190_e2876;
        var_gm_dn0 = (((var_gm_dn0 * assign2190_e2875) - (var_gm * (var_gm_dn0 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn1 = (((var_gm_dn1 * assign2190_e2875) - (var_gm * (var_gm_dn1 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn2 = (((var_gm_dn2 * assign2190_e2875) - (var_gm * (var_gm_dn2 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn3 = (((var_gm_dn3 * assign2190_e2875) - (var_gm * (var_gm_dn3 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn4 = (((var_gm_dn4 * assign2190_e2875) - (var_gm * (var_gm_dn4 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn5 = (((var_gm_dn5 * assign2190_e2875) - (var_gm * (var_gm_dn5 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn6 = (((var_gm_dn6 * assign2190_e2875) - (var_gm * (var_gm_dn6 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn7 = (((var_gm_dn7 * assign2190_e2875) - (var_gm * (var_gm_dn7 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn8 = (((var_gm_dn8 * assign2190_e2875) - (var_gm * (var_gm_dn8 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn9 = (((var_gm_dn9 * assign2190_e2875) - (var_gm * (var_gm_dn9 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn10 = (((var_gm_dn10 * assign2190_e2875) - (var_gm * (var_gm_dn10 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn11 = (((var_gm_dn11 * assign2190_e2875) - (var_gm * (var_gm_dn11 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn12 = (((var_gm_dn12 * assign2190_e2875) - (var_gm * (var_gm_dn12 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn13 = (((var_gm_dn13 * assign2190_e2875) - (var_gm * (var_gm_dn13 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn14 = (((var_gm_dn14 * assign2190_e2875) - (var_gm * (var_gm_dn14 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn15 = (((var_gm_dn15 * assign2190_e2875) - (var_gm * (var_gm_dn15 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn16 = (((var_gm_dn16 * assign2190_e2875) - (var_gm * (var_gm_dn16 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn17 = (((var_gm_dn17 * assign2190_e2875) - (var_gm * (var_gm_dn17 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_dn18 = (((var_gm_dn18 * assign2190_e2875) - (var_gm * (var_gm_dn18 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db0 = (((var_gm_db0 * assign2190_e2875) - (var_gm * (var_gm_db0 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db1 = (((var_gm_db1 * assign2190_e2875) - (var_gm * (var_gm_db1 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db2 = (((var_gm_db2 * assign2190_e2875) - (var_gm * (var_gm_db2 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db3 = (((var_gm_db3 * assign2190_e2875) - (var_gm * (var_gm_db3 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db4 = (((var_gm_db4 * assign2190_e2875) - (var_gm * (var_gm_db4 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db5 = (((var_gm_db5 * assign2190_e2875) - (var_gm * (var_gm_db5 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db6 = (((var_gm_db6 * assign2190_e2875) - (var_gm * (var_gm_db6 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db7 = (((var_gm_db7 * assign2190_e2875) - (var_gm * (var_gm_db7 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db8 = (((var_gm_db8 * assign2190_e2875) - (var_gm * (var_gm_db8 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db9 = (((var_gm_db9 * assign2190_e2875) - (var_gm * (var_gm_db9 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db10 = (((var_gm_db10 * assign2190_e2875) - (var_gm * (var_gm_db10 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db11 = (((var_gm_db11 * assign2190_e2875) - (var_gm * (var_gm_db11 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db12 = (((var_gm_db12 * assign2190_e2875) - (var_gm * (var_gm_db12 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db13 = (((var_gm_db13 * assign2190_e2875) - (var_gm * (var_gm_db13 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db14 = (((var_gm_db14 * assign2190_e2875) - (var_gm * (var_gm_db14 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db15 = (((var_gm_db15 * assign2190_e2875) - (var_gm * (var_gm_db15 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db16 = (((var_gm_db16 * assign2190_e2875) - (var_gm * (var_gm_db16 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db17 = (((var_gm_db17 * assign2190_e2875) - (var_gm * (var_gm_db17 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_db18 = (((var_gm_db18 * assign2190_e2875) - (var_gm * (var_gm_db18 * p.p50))) / (assign2190_e2875 * assign2190_e2875));
        var_gm_rv = 0.0;
        var_gm_rdn0 = 0.0;
        var_gm_rdn1 = 0.0;
        var_gm_rdn2 = 0.0;
        var_gm_rdn3 = 0.0;
        var_gm_rdn4 = 0.0;
        var_gm_rdn5 = 0.0;
        var_gm_rdn6 = 0.0;
        var_gm_rdn7 = 0.0;
        var_gm_rdn8 = 0.0;
        var_gm_rdn9 = 0.0;
        var_gm_rdn10 = 0.0;
        var_gm_rdn11 = 0.0;
        var_gm_rdn12 = 0.0;
        var_gm_rdn13 = 0.0;
        var_gm_rdn14 = 0.0;
        var_gm_rdn15 = 0.0;
        var_gm_rdn16 = 0.0;
        var_gm_rdn17 = 0.0;
        var_gm_rdn18 = 0.0;
        var_gm_rdb0 = 0.0;
        var_gm_rdb1 = 0.0;
        var_gm_rdb2 = 0.0;
        var_gm_rdb3 = 0.0;
        var_gm_rdb4 = 0.0;
        var_gm_rdb5 = 0.0;
        var_gm_rdb6 = 0.0;
        var_gm_rdb7 = 0.0;
        var_gm_rdb8 = 0.0;
        var_gm_rdb9 = 0.0;
        var_gm_rdb10 = 0.0;
        var_gm_rdb11 = 0.0;
        var_gm_rdb12 = 0.0;
        var_gm_rdb13 = 0.0;
        var_gm_rdb14 = 0.0;
        var_gm_rdb15 = 0.0;
        var_gm_rdb16 = 0.0;
        var_gm_rdb17 = 0.0;
        var_gm_rdb18 = 0.0;

        let assign2210_e2882: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        var_guard28 = assign2210_e2882;
        var_guard28_dn0 = 0.0;
        var_guard28_dn1 = 0.0;
        var_guard28_dn2 = 0.0;
        var_guard28_dn3 = 0.0;
        var_guard28_dn4 = 0.0;
        var_guard28_dn5 = 0.0;
        var_guard28_dn6 = 0.0;
        var_guard28_dn7 = 0.0;
        var_guard28_dn8 = 0.0;
        var_guard28_dn9 = 0.0;
        var_guard28_dn10 = 0.0;
        var_guard28_dn11 = 0.0;
        var_guard28_dn12 = 0.0;
        var_guard28_dn13 = 0.0;
        var_guard28_dn14 = 0.0;
        var_guard28_dn15 = 0.0;
        var_guard28_dn16 = 0.0;
        var_guard28_dn17 = 0.0;
        var_guard28_dn18 = 0.0;
        var_guard28_db0 = 0.0;
        var_guard28_db1 = 0.0;
        var_guard28_db2 = 0.0;
        var_guard28_db3 = 0.0;
        var_guard28_db4 = 0.0;
        var_guard28_db5 = 0.0;
        var_guard28_db6 = 0.0;
        var_guard28_db7 = 0.0;
        var_guard28_db8 = 0.0;
        var_guard28_db9 = 0.0;
        var_guard28_db10 = 0.0;
        var_guard28_db11 = 0.0;
        var_guard28_db12 = 0.0;
        var_guard28_db13 = 0.0;
        var_guard28_db14 = 0.0;
        var_guard28_db15 = 0.0;
        var_guard28_db16 = 0.0;
        var_guard28_db17 = 0.0;
        var_guard28_db18 = 0.0;
        var_guard28_rv = 0.0;
        var_guard28_rdn0 = 0.0;
        var_guard28_rdn1 = 0.0;
        var_guard28_rdn2 = 0.0;
        var_guard28_rdn3 = 0.0;
        var_guard28_rdn4 = 0.0;
        var_guard28_rdn5 = 0.0;
        var_guard28_rdn6 = 0.0;
        var_guard28_rdn7 = 0.0;
        var_guard28_rdn8 = 0.0;
        var_guard28_rdn9 = 0.0;
        var_guard28_rdn10 = 0.0;
        var_guard28_rdn11 = 0.0;
        var_guard28_rdn12 = 0.0;
        var_guard28_rdn13 = 0.0;
        var_guard28_rdn14 = 0.0;
        var_guard28_rdn15 = 0.0;
        var_guard28_rdn16 = 0.0;
        var_guard28_rdn17 = 0.0;
        var_guard28_rdn18 = 0.0;
        var_guard28_rdb0 = 0.0;
        var_guard28_rdb1 = 0.0;
        var_guard28_rdb2 = 0.0;
        var_guard28_rdb3 = 0.0;
        var_guard28_rdb4 = 0.0;
        var_guard28_rdb5 = 0.0;
        var_guard28_rdb6 = 0.0;
        var_guard28_rdb7 = 0.0;
        var_guard28_rdb8 = 0.0;
        var_guard28_rdb9 = 0.0;
        var_guard28_rdb10 = 0.0;
        var_guard28_rdb11 = 0.0;
        var_guard28_rdb12 = 0.0;
        var_guard28_rdb13 = 0.0;
        var_guard28_rdb14 = 0.0;
        var_guard28_rdb15 = 0.0;
        var_guard28_rdb16 = 0.0;
        var_guard28_rdb17 = 0.0;
        var_guard28_rdb18 = 0.0;

        let assign2220_e2885: f64 = if p.p7 == 1.0 { 1.0 } else { 0.0 };
        var_guard29 = assign2220_e2885;
        var_guard29_dn0 = 0.0;
        var_guard29_dn1 = 0.0;
        var_guard29_dn2 = 0.0;
        var_guard29_dn3 = 0.0;
        var_guard29_dn4 = 0.0;
        var_guard29_dn5 = 0.0;
        var_guard29_dn6 = 0.0;
        var_guard29_dn7 = 0.0;
        var_guard29_dn8 = 0.0;
        var_guard29_dn9 = 0.0;
        var_guard29_dn10 = 0.0;
        var_guard29_dn11 = 0.0;
        var_guard29_dn12 = 0.0;
        var_guard29_dn13 = 0.0;
        var_guard29_dn14 = 0.0;
        var_guard29_dn15 = 0.0;
        var_guard29_dn16 = 0.0;
        var_guard29_dn17 = 0.0;
        var_guard29_dn18 = 0.0;
        var_guard29_db0 = 0.0;
        var_guard29_db1 = 0.0;
        var_guard29_db2 = 0.0;
        var_guard29_db3 = 0.0;
        var_guard29_db4 = 0.0;
        var_guard29_db5 = 0.0;
        var_guard29_db6 = 0.0;
        var_guard29_db7 = 0.0;
        var_guard29_db8 = 0.0;
        var_guard29_db9 = 0.0;
        var_guard29_db10 = 0.0;
        var_guard29_db11 = 0.0;
        var_guard29_db12 = 0.0;
        var_guard29_db13 = 0.0;
        var_guard29_db14 = 0.0;
        var_guard29_db15 = 0.0;
        var_guard29_db16 = 0.0;
        var_guard29_db17 = 0.0;
        var_guard29_db18 = 0.0;
        var_guard29_rv = 0.0;
        var_guard29_rdn0 = 0.0;
        var_guard29_rdn1 = 0.0;
        var_guard29_rdn2 = 0.0;
        var_guard29_rdn3 = 0.0;
        var_guard29_rdn4 = 0.0;
        var_guard29_rdn5 = 0.0;
        var_guard29_rdn6 = 0.0;
        var_guard29_rdn7 = 0.0;
        var_guard29_rdn8 = 0.0;
        var_guard29_rdn9 = 0.0;
        var_guard29_rdn10 = 0.0;
        var_guard29_rdn11 = 0.0;
        var_guard29_rdn12 = 0.0;
        var_guard29_rdn13 = 0.0;
        var_guard29_rdn14 = 0.0;
        var_guard29_rdn15 = 0.0;
        var_guard29_rdn16 = 0.0;
        var_guard29_rdn17 = 0.0;
        var_guard29_rdn18 = 0.0;
        var_guard29_rdb0 = 0.0;
        var_guard29_rdb1 = 0.0;
        var_guard29_rdb2 = 0.0;
        var_guard29_rdb3 = 0.0;
        var_guard29_rdb4 = 0.0;
        var_guard29_rdb5 = 0.0;
        var_guard29_rdb6 = 0.0;
        var_guard29_rdb7 = 0.0;
        var_guard29_rdb8 = 0.0;
        var_guard29_rdb9 = 0.0;
        var_guard29_rdb10 = 0.0;
        var_guard29_rdb11 = 0.0;
        var_guard29_rdb12 = 0.0;
        var_guard29_rdb13 = 0.0;
        var_guard29_rdb14 = 0.0;
        var_guard29_rdb15 = 0.0;
        var_guard29_rdb16 = 0.0;
        var_guard29_rdb17 = 0.0;
        var_guard29_rdb18 = 0.0;

        let (assign2240_e2896, assign2240_e2896_d_n0, assign2240_e2896_d_n1, assign2240_e2896_d_n2, assign2240_e2896_d_n3, assign2240_e2896_d_n4, assign2240_e2896_d_n5, assign2240_e2896_d_n6, assign2240_e2896_d_n7, assign2240_e2896_d_n8, assign2240_e2896_d_n9, assign2240_e2896_d_n10, assign2240_e2896_d_n11, assign2240_e2896_d_n12, assign2240_e2896_d_n13, assign2240_e2896_d_n14, assign2240_e2896_d_n15, assign2240_e2896_d_n16, assign2240_e2896_d_n17, assign2240_e2896_d_n18, assign2240_e2896_d_b0, assign2240_e2896_d_b1, assign2240_e2896_d_b2, assign2240_e2896_d_b3, assign2240_e2896_d_b4, assign2240_e2896_d_b5, assign2240_e2896_d_b6, assign2240_e2896_d_b7, assign2240_e2896_d_b8, assign2240_e2896_d_b9, assign2240_e2896_d_b10, assign2240_e2896_d_b11, assign2240_e2896_d_b12, assign2240_e2896_d_b13, assign2240_e2896_d_b14, assign2240_e2896_d_b15, assign2240_e2896_d_b16, assign2240_e2896_d_b17, assign2240_e2896_d_b18,) = {
    if (var_guard28 != 0.0) {
        let assign2240_e2891: f64 = (var_ids).abs();
        let assign2240_e2893: f64 = (var_igd).abs();
        let assign2240_e2894: f64 = (assign2240_e2891 + assign2240_e2893);
        (assign2240_e2894, (if var_ids >= 0.0 { var_ids_dn0 } else { (-var_ids_dn0) } + if var_igd >= 0.0 { var_igd_dn0 } else { (-var_igd_dn0) }), (if var_ids >= 0.0 { var_ids_dn1 } else { (-var_ids_dn1) } + if var_igd >= 0.0 { var_igd_dn1 } else { (-var_igd_dn1) }), (if var_ids >= 0.0 { var_ids_dn2 } else { (-var_ids_dn2) } + if var_igd >= 0.0 { var_igd_dn2 } else { (-var_igd_dn2) }), (if var_ids >= 0.0 { var_ids_dn3 } else { (-var_ids_dn3) } + if var_igd >= 0.0 { var_igd_dn3 } else { (-var_igd_dn3) }), (if var_ids >= 0.0 { var_ids_dn4 } else { (-var_ids_dn4) } + if var_igd >= 0.0 { var_igd_dn4 } else { (-var_igd_dn4) }), (if var_ids >= 0.0 { var_ids_dn5 } else { (-var_ids_dn5) } + if var_igd >= 0.0 { var_igd_dn5 } else { (-var_igd_dn5) }), (if var_ids >= 0.0 { var_ids_dn6 } else { (-var_ids_dn6) } + if var_igd >= 0.0 { var_igd_dn6 } else { (-var_igd_dn6) }), (if var_ids >= 0.0 { var_ids_dn7 } else { (-var_ids_dn7) } + if var_igd >= 0.0 { var_igd_dn7 } else { (-var_igd_dn7) }), (if var_ids >= 0.0 { var_ids_dn8 } else { (-var_ids_dn8) } + if var_igd >= 0.0 { var_igd_dn8 } else { (-var_igd_dn8) }), (if var_ids >= 0.0 { var_ids_dn9 } else { (-var_ids_dn9) } + if var_igd >= 0.0 { var_igd_dn9 } else { (-var_igd_dn9) }), (if var_ids >= 0.0 { var_ids_dn10 } else { (-var_ids_dn10) } + if var_igd >= 0.0 { var_igd_dn10 } else { (-var_igd_dn10) }), (if var_ids >= 0.0 { var_ids_dn11 } else { (-var_ids_dn11) } + if var_igd >= 0.0 { var_igd_dn11 } else { (-var_igd_dn11) }), (if var_ids >= 0.0 { var_ids_dn12 } else { (-var_ids_dn12) } + if var_igd >= 0.0 { var_igd_dn12 } else { (-var_igd_dn12) }), (if var_ids >= 0.0 { var_ids_dn13 } else { (-var_ids_dn13) } + if var_igd >= 0.0 { var_igd_dn13 } else { (-var_igd_dn13) }), (if var_ids >= 0.0 { var_ids_dn14 } else { (-var_ids_dn14) } + if var_igd >= 0.0 { var_igd_dn14 } else { (-var_igd_dn14) }), (if var_ids >= 0.0 { var_ids_dn15 } else { (-var_ids_dn15) } + if var_igd >= 0.0 { var_igd_dn15 } else { (-var_igd_dn15) }), (if var_ids >= 0.0 { var_ids_dn16 } else { (-var_ids_dn16) } + if var_igd >= 0.0 { var_igd_dn16 } else { (-var_igd_dn16) }), (if var_ids >= 0.0 { var_ids_dn17 } else { (-var_ids_dn17) } + if var_igd >= 0.0 { var_igd_dn17 } else { (-var_igd_dn17) }), (if var_ids >= 0.0 { var_ids_dn18 } else { (-var_ids_dn18) } + if var_igd >= 0.0 { var_igd_dn18 } else { (-var_igd_dn18) }), (if var_ids >= 0.0 { var_ids_db0 } else { (-var_ids_db0) } + if var_igd >= 0.0 { var_igd_db0 } else { (-var_igd_db0) }), (if var_ids >= 0.0 { var_ids_db1 } else { (-var_ids_db1) } + if var_igd >= 0.0 { var_igd_db1 } else { (-var_igd_db1) }), (if var_ids >= 0.0 { var_ids_db2 } else { (-var_ids_db2) } + if var_igd >= 0.0 { var_igd_db2 } else { (-var_igd_db2) }), (if var_ids >= 0.0 { var_ids_db3 } else { (-var_ids_db3) } + if var_igd >= 0.0 { var_igd_db3 } else { (-var_igd_db3) }), (if var_ids >= 0.0 { var_ids_db4 } else { (-var_ids_db4) } + if var_igd >= 0.0 { var_igd_db4 } else { (-var_igd_db4) }), (if var_ids >= 0.0 { var_ids_db5 } else { (-var_ids_db5) } + if var_igd >= 0.0 { var_igd_db5 } else { (-var_igd_db5) }), (if var_ids >= 0.0 { var_ids_db6 } else { (-var_ids_db6) } + if var_igd >= 0.0 { var_igd_db6 } else { (-var_igd_db6) }), (if var_ids >= 0.0 { var_ids_db7 } else { (-var_ids_db7) } + if var_igd >= 0.0 { var_igd_db7 } else { (-var_igd_db7) }), (if var_ids >= 0.0 { var_ids_db8 } else { (-var_ids_db8) } + if var_igd >= 0.0 { var_igd_db8 } else { (-var_igd_db8) }), (if var_ids >= 0.0 { var_ids_db9 } else { (-var_ids_db9) } + if var_igd >= 0.0 { var_igd_db9 } else { (-var_igd_db9) }), (if var_ids >= 0.0 { var_ids_db10 } else { (-var_ids_db10) } + if var_igd >= 0.0 { var_igd_db10 } else { (-var_igd_db10) }), (if var_ids >= 0.0 { var_ids_db11 } else { (-var_ids_db11) } + if var_igd >= 0.0 { var_igd_db11 } else { (-var_igd_db11) }), (if var_ids >= 0.0 { var_ids_db12 } else { (-var_ids_db12) } + if var_igd >= 0.0 { var_igd_db12 } else { (-var_igd_db12) }), (if var_ids >= 0.0 { var_ids_db13 } else { (-var_ids_db13) } + if var_igd >= 0.0 { var_igd_db13 } else { (-var_igd_db13) }), (if var_ids >= 0.0 { var_ids_db14 } else { (-var_ids_db14) } + if var_igd >= 0.0 { var_igd_db14 } else { (-var_igd_db14) }), (if var_ids >= 0.0 { var_ids_db15 } else { (-var_ids_db15) } + if var_igd >= 0.0 { var_igd_db15 } else { (-var_igd_db15) }), (if var_ids >= 0.0 { var_ids_db16 } else { (-var_ids_db16) } + if var_igd >= 0.0 { var_igd_db16 } else { (-var_igd_db16) }), (if var_ids >= 0.0 { var_ids_db17 } else { (-var_ids_db17) } + if var_igd >= 0.0 { var_igd_db17 } else { (-var_igd_db17) }), (if var_ids >= 0.0 { var_ids_db18 } else { (-var_ids_db18) } + if var_igd >= 0.0 { var_igd_db18 } else { (-var_igd_db18) }),)
    } else {
        (var_idtn, var_idtn_dn0, var_idtn_dn1, var_idtn_dn2, var_idtn_dn3, var_idtn_dn4, var_idtn_dn5, var_idtn_dn6, var_idtn_dn7, var_idtn_dn8, var_idtn_dn9, var_idtn_dn10, var_idtn_dn11, var_idtn_dn12, var_idtn_dn13, var_idtn_dn14, var_idtn_dn15, var_idtn_dn16, var_idtn_dn17, var_idtn_dn18, var_idtn_db0, var_idtn_db1, var_idtn_db2, var_idtn_db3, var_idtn_db4, var_idtn_db5, var_idtn_db6, var_idtn_db7, var_idtn_db8, var_idtn_db9, var_idtn_db10, var_idtn_db11, var_idtn_db12, var_idtn_db13, var_idtn_db14, var_idtn_db15, var_idtn_db16, var_idtn_db17, var_idtn_db18,)
    }
};
        var_idtn = assign2240_e2896;
        var_idtn_dn0 = assign2240_e2896_d_n0;
        var_idtn_dn1 = assign2240_e2896_d_n1;
        var_idtn_dn2 = assign2240_e2896_d_n2;
        var_idtn_dn3 = assign2240_e2896_d_n3;
        var_idtn_dn4 = assign2240_e2896_d_n4;
        var_idtn_dn5 = assign2240_e2896_d_n5;
        var_idtn_dn6 = assign2240_e2896_d_n6;
        var_idtn_dn7 = assign2240_e2896_d_n7;
        var_idtn_dn8 = assign2240_e2896_d_n8;
        var_idtn_dn9 = assign2240_e2896_d_n9;
        var_idtn_dn10 = assign2240_e2896_d_n10;
        var_idtn_dn11 = assign2240_e2896_d_n11;
        var_idtn_dn12 = assign2240_e2896_d_n12;
        var_idtn_dn13 = assign2240_e2896_d_n13;
        var_idtn_dn14 = assign2240_e2896_d_n14;
        var_idtn_dn15 = assign2240_e2896_d_n15;
        var_idtn_dn16 = assign2240_e2896_d_n16;
        var_idtn_dn17 = assign2240_e2896_d_n17;
        var_idtn_dn18 = assign2240_e2896_d_n18;
        var_idtn_db0 = assign2240_e2896_d_b0;
        var_idtn_db1 = assign2240_e2896_d_b1;
        var_idtn_db2 = assign2240_e2896_d_b2;
        var_idtn_db3 = assign2240_e2896_d_b3;
        var_idtn_db4 = assign2240_e2896_d_b4;
        var_idtn_db5 = assign2240_e2896_d_b5;
        var_idtn_db6 = assign2240_e2896_d_b6;
        var_idtn_db7 = assign2240_e2896_d_b7;
        var_idtn_db8 = assign2240_e2896_d_b8;
        var_idtn_db9 = assign2240_e2896_d_b9;
        var_idtn_db10 = assign2240_e2896_d_b10;
        var_idtn_db11 = assign2240_e2896_d_b11;
        var_idtn_db12 = assign2240_e2896_d_b12;
        var_idtn_db13 = assign2240_e2896_d_b13;
        var_idtn_db14 = assign2240_e2896_d_b14;
        var_idtn_db15 = assign2240_e2896_d_b15;
        var_idtn_db16 = assign2240_e2896_d_b16;
        var_idtn_db17 = assign2240_e2896_d_b17;
        var_idtn_db18 = assign2240_e2896_d_b18;
        var_idtn_rv = 0.0;
        var_idtn_rdn0 = 0.0;
        var_idtn_rdn1 = 0.0;
        var_idtn_rdn2 = 0.0;
        var_idtn_rdn3 = 0.0;
        var_idtn_rdn4 = 0.0;
        var_idtn_rdn5 = 0.0;
        var_idtn_rdn6 = 0.0;
        var_idtn_rdn7 = 0.0;
        var_idtn_rdn8 = 0.0;
        var_idtn_rdn9 = 0.0;
        var_idtn_rdn10 = 0.0;
        var_idtn_rdn11 = 0.0;
        var_idtn_rdn12 = 0.0;
        var_idtn_rdn13 = 0.0;
        var_idtn_rdn14 = 0.0;
        var_idtn_rdn15 = 0.0;
        var_idtn_rdn16 = 0.0;
        var_idtn_rdn17 = 0.0;
        var_idtn_rdn18 = 0.0;
        var_idtn_rdb0 = 0.0;
        var_idtn_rdb1 = 0.0;
        var_idtn_rdb2 = 0.0;
        var_idtn_rdb3 = 0.0;
        var_idtn_rdb4 = 0.0;
        var_idtn_rdb5 = 0.0;
        var_idtn_rdb6 = 0.0;
        var_idtn_rdb7 = 0.0;
        var_idtn_rdb8 = 0.0;
        var_idtn_rdb9 = 0.0;
        var_idtn_rdb10 = 0.0;
        var_idtn_rdb11 = 0.0;
        var_idtn_rdb12 = 0.0;
        var_idtn_rdb13 = 0.0;
        var_idtn_rdb14 = 0.0;
        var_idtn_rdb15 = 0.0;
        var_idtn_rdb16 = 0.0;
        var_idtn_rdb17 = 0.0;
        var_idtn_rdb18 = 0.0;

        let (assign2250_e2917, assign2250_e2917_d_n0, assign2250_e2917_d_n1, assign2250_e2917_d_n2, assign2250_e2917_d_n3, assign2250_e2917_d_n4, assign2250_e2917_d_n5, assign2250_e2917_d_n6, assign2250_e2917_d_n7, assign2250_e2917_d_n8, assign2250_e2917_d_n9, assign2250_e2917_d_n10, assign2250_e2917_d_n11, assign2250_e2917_d_n12, assign2250_e2917_d_n13, assign2250_e2917_d_n14, assign2250_e2917_d_n15, assign2250_e2917_d_n16, assign2250_e2917_d_n17, assign2250_e2917_d_n18, assign2250_e2917_d_b0, assign2250_e2917_d_b1, assign2250_e2917_d_b2, assign2250_e2917_d_b3, assign2250_e2917_d_b4, assign2250_e2917_d_b5, assign2250_e2917_d_b6, assign2250_e2917_d_b7, assign2250_e2917_d_b8, assign2250_e2917_d_b9, assign2250_e2917_d_b10, assign2250_e2917_d_b11, assign2250_e2917_d_b12, assign2250_e2917_d_b13, assign2250_e2917_d_b14, assign2250_e2917_d_b15, assign2250_e2917_d_b16, assign2250_e2917_d_b17, assign2250_e2917_d_b18,) = {
    if (var_guard28 != 0.0) {
        let assign2250_e2900: f64 = (p.p93 + 273.15);
        let assign2250_e2904: f64 = (p.p95 * var_tanh_psi);
        let assign2250_e2906: f64 = (var_tanh_alpha_vds).abs();
        let assign2250_e2907: f64 = (assign2250_e2904 * assign2250_e2906);
        let assign2250_e2911: f64 = (p.p16 * var_vds);
        let assign2250_e2912: f64 = (1.0 + assign2250_e2911);
        let assign2250_e2913: f64 = (assign2250_e2907 * assign2250_e2912);
        let assign2250_e2914: f64 = (1.0 + assign2250_e2913);
        let assign2250_e2915: f64 = (assign2250_e2900 * assign2250_e2914);
        (assign2250_e2915, (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn0) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn0 } else { (-var_tanh_alpha_vds_dn0) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn0)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn1) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn1 } else { (-var_tanh_alpha_vds_dn1) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn1)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn2) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn2 } else { (-var_tanh_alpha_vds_dn2) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn2)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn3) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn3 } else { (-var_tanh_alpha_vds_dn3) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn3)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn4) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn4 } else { (-var_tanh_alpha_vds_dn4) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn4)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn5) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn5 } else { (-var_tanh_alpha_vds_dn5) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn5)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn6) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn6 } else { (-var_tanh_alpha_vds_dn6) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn6)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn7) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn7 } else { (-var_tanh_alpha_vds_dn7) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn7)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn8) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn8 } else { (-var_tanh_alpha_vds_dn8) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn8)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn9) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn9 } else { (-var_tanh_alpha_vds_dn9) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn9)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn10) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn10 } else { (-var_tanh_alpha_vds_dn10) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn10)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn11) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn11 } else { (-var_tanh_alpha_vds_dn11) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn11)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn12) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn12 } else { (-var_tanh_alpha_vds_dn12) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn12)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn13) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn13 } else { (-var_tanh_alpha_vds_dn13) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn13)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn14) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn14 } else { (-var_tanh_alpha_vds_dn14) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn14)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn15) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn15 } else { (-var_tanh_alpha_vds_dn15) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn15)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn16) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn16 } else { (-var_tanh_alpha_vds_dn16) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn16)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn17) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn17 } else { (-var_tanh_alpha_vds_dn17) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn17)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_dn18) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_dn18 } else { (-var_tanh_alpha_vds_dn18) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_dn18)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db0) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db0 } else { (-var_tanh_alpha_vds_db0) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db0)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db1) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db1 } else { (-var_tanh_alpha_vds_db1) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db1)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db2) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db2 } else { (-var_tanh_alpha_vds_db2) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db2)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db3) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db3 } else { (-var_tanh_alpha_vds_db3) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db3)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db4) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db4 } else { (-var_tanh_alpha_vds_db4) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db4)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db5) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db5 } else { (-var_tanh_alpha_vds_db5) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db5)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db6) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db6 } else { (-var_tanh_alpha_vds_db6) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db6)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db7) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db7 } else { (-var_tanh_alpha_vds_db7) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db7)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db8) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db8 } else { (-var_tanh_alpha_vds_db8) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db8)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db9) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db9 } else { (-var_tanh_alpha_vds_db9) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db9)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db10) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db10 } else { (-var_tanh_alpha_vds_db10) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db10)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db11) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db11 } else { (-var_tanh_alpha_vds_db11) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db11)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db12) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db12 } else { (-var_tanh_alpha_vds_db12) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db12)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db13) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db13 } else { (-var_tanh_alpha_vds_db13) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db13)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db14) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db14 } else { (-var_tanh_alpha_vds_db14) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db14)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db15) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db15 } else { (-var_tanh_alpha_vds_db15) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db15)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db16) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db16 } else { (-var_tanh_alpha_vds_db16) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db16)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db17) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db17 } else { (-var_tanh_alpha_vds_db17) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db17)))), (assign2250_e2900 * (((((p.p95 * var_tanh_psi_db18) * assign2250_e2906) + (assign2250_e2904 * if var_tanh_alpha_vds >= 0.0 { var_tanh_alpha_vds_db18 } else { (-var_tanh_alpha_vds_db18) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * var_vds_db18)))),)
    } else {
        (var_td_prime, var_td_prime_dn0, var_td_prime_dn1, var_td_prime_dn2, var_td_prime_dn3, var_td_prime_dn4, var_td_prime_dn5, var_td_prime_dn6, var_td_prime_dn7, var_td_prime_dn8, var_td_prime_dn9, var_td_prime_dn10, var_td_prime_dn11, var_td_prime_dn12, var_td_prime_dn13, var_td_prime_dn14, var_td_prime_dn15, var_td_prime_dn16, var_td_prime_dn17, var_td_prime_dn18, var_td_prime_db0, var_td_prime_db1, var_td_prime_db2, var_td_prime_db3, var_td_prime_db4, var_td_prime_db5, var_td_prime_db6, var_td_prime_db7, var_td_prime_db8, var_td_prime_db9, var_td_prime_db10, var_td_prime_db11, var_td_prime_db12, var_td_prime_db13, var_td_prime_db14, var_td_prime_db15, var_td_prime_db16, var_td_prime_db17, var_td_prime_db18,)
    }
};
        var_td_prime = assign2250_e2917;
        var_td_prime_dn0 = assign2250_e2917_d_n0;
        var_td_prime_dn1 = assign2250_e2917_d_n1;
        var_td_prime_dn2 = assign2250_e2917_d_n2;
        var_td_prime_dn3 = assign2250_e2917_d_n3;
        var_td_prime_dn4 = assign2250_e2917_d_n4;
        var_td_prime_dn5 = assign2250_e2917_d_n5;
        var_td_prime_dn6 = assign2250_e2917_d_n6;
        var_td_prime_dn7 = assign2250_e2917_d_n7;
        var_td_prime_dn8 = assign2250_e2917_d_n8;
        var_td_prime_dn9 = assign2250_e2917_d_n9;
        var_td_prime_dn10 = assign2250_e2917_d_n10;
        var_td_prime_dn11 = assign2250_e2917_d_n11;
        var_td_prime_dn12 = assign2250_e2917_d_n12;
        var_td_prime_dn13 = assign2250_e2917_d_n13;
        var_td_prime_dn14 = assign2250_e2917_d_n14;
        var_td_prime_dn15 = assign2250_e2917_d_n15;
        var_td_prime_dn16 = assign2250_e2917_d_n16;
        var_td_prime_dn17 = assign2250_e2917_d_n17;
        var_td_prime_dn18 = assign2250_e2917_d_n18;
        var_td_prime_db0 = assign2250_e2917_d_b0;
        var_td_prime_db1 = assign2250_e2917_d_b1;
        var_td_prime_db2 = assign2250_e2917_d_b2;
        var_td_prime_db3 = assign2250_e2917_d_b3;
        var_td_prime_db4 = assign2250_e2917_d_b4;
        var_td_prime_db5 = assign2250_e2917_d_b5;
        var_td_prime_db6 = assign2250_e2917_d_b6;
        var_td_prime_db7 = assign2250_e2917_d_b7;
        var_td_prime_db8 = assign2250_e2917_d_b8;
        var_td_prime_db9 = assign2250_e2917_d_b9;
        var_td_prime_db10 = assign2250_e2917_d_b10;
        var_td_prime_db11 = assign2250_e2917_d_b11;
        var_td_prime_db12 = assign2250_e2917_d_b12;
        var_td_prime_db13 = assign2250_e2917_d_b13;
        var_td_prime_db14 = assign2250_e2917_d_b14;
        var_td_prime_db15 = assign2250_e2917_d_b15;
        var_td_prime_db16 = assign2250_e2917_d_b16;
        var_td_prime_db17 = assign2250_e2917_d_b17;
        var_td_prime_db18 = assign2250_e2917_d_b18;
        var_td_prime_rv = 0.0;
        var_td_prime_rdn0 = 0.0;
        var_td_prime_rdn1 = 0.0;
        var_td_prime_rdn2 = 0.0;
        var_td_prime_rdn3 = 0.0;
        var_td_prime_rdn4 = 0.0;
        var_td_prime_rdn5 = 0.0;
        var_td_prime_rdn6 = 0.0;
        var_td_prime_rdn7 = 0.0;
        var_td_prime_rdn8 = 0.0;
        var_td_prime_rdn9 = 0.0;
        var_td_prime_rdn10 = 0.0;
        var_td_prime_rdn11 = 0.0;
        var_td_prime_rdn12 = 0.0;
        var_td_prime_rdn13 = 0.0;
        var_td_prime_rdn14 = 0.0;
        var_td_prime_rdn15 = 0.0;
        var_td_prime_rdn16 = 0.0;
        var_td_prime_rdn17 = 0.0;
        var_td_prime_rdn18 = 0.0;
        var_td_prime_rdb0 = 0.0;
        var_td_prime_rdb1 = 0.0;
        var_td_prime_rdb2 = 0.0;
        var_td_prime_rdb3 = 0.0;
        var_td_prime_rdb4 = 0.0;
        var_td_prime_rdb5 = 0.0;
        var_td_prime_rdb6 = 0.0;
        var_td_prime_rdb7 = 0.0;
        var_td_prime_rdb8 = 0.0;
        var_td_prime_rdb9 = 0.0;
        var_td_prime_rdb10 = 0.0;
        var_td_prime_rdb11 = 0.0;
        var_td_prime_rdb12 = 0.0;
        var_td_prime_rdb13 = 0.0;
        var_td_prime_rdb14 = 0.0;
        var_td_prime_rdb15 = 0.0;
        var_td_prime_rdb16 = 0.0;
        var_td_prime_rdb17 = 0.0;
        var_td_prime_rdb18 = 0.0;


        *var_gm_slot = var_gm;
        *var_gm_db0_slot = var_gm_db0;
        *var_gm_db1_slot = var_gm_db1;
        *var_gm_db10_slot = var_gm_db10;
        *var_gm_db11_slot = var_gm_db11;
        *var_gm_db12_slot = var_gm_db12;
        *var_gm_db13_slot = var_gm_db13;
        *var_gm_db14_slot = var_gm_db14;
        *var_gm_db15_slot = var_gm_db15;
        *var_gm_db16_slot = var_gm_db16;
        *var_gm_db17_slot = var_gm_db17;
        *var_gm_db18_slot = var_gm_db18;
        *var_gm_db2_slot = var_gm_db2;
        *var_gm_db3_slot = var_gm_db3;
        *var_gm_db4_slot = var_gm_db4;
        *var_gm_db5_slot = var_gm_db5;
        *var_gm_db6_slot = var_gm_db6;
        *var_gm_db7_slot = var_gm_db7;
        *var_gm_db8_slot = var_gm_db8;
        *var_gm_db9_slot = var_gm_db9;
        *var_gm_dn0_slot = var_gm_dn0;
        *var_gm_dn1_slot = var_gm_dn1;
        *var_gm_dn10_slot = var_gm_dn10;
        *var_gm_dn11_slot = var_gm_dn11;
        *var_gm_dn12_slot = var_gm_dn12;
        *var_gm_dn13_slot = var_gm_dn13;
        *var_gm_dn14_slot = var_gm_dn14;
        *var_gm_dn15_slot = var_gm_dn15;
        *var_gm_dn16_slot = var_gm_dn16;
        *var_gm_dn17_slot = var_gm_dn17;
        *var_gm_dn18_slot = var_gm_dn18;
        *var_gm_dn2_slot = var_gm_dn2;
        *var_gm_dn3_slot = var_gm_dn3;
        *var_gm_dn4_slot = var_gm_dn4;
        *var_gm_dn5_slot = var_gm_dn5;
        *var_gm_dn6_slot = var_gm_dn6;
        *var_gm_dn7_slot = var_gm_dn7;
        *var_gm_dn8_slot = var_gm_dn8;
        *var_gm_dn9_slot = var_gm_dn9;
        *var_gm_rdb0_slot = var_gm_rdb0;
        *var_gm_rdb1_slot = var_gm_rdb1;
        *var_gm_rdb10_slot = var_gm_rdb10;
        *var_gm_rdb11_slot = var_gm_rdb11;
        *var_gm_rdb12_slot = var_gm_rdb12;
        *var_gm_rdb13_slot = var_gm_rdb13;
        *var_gm_rdb14_slot = var_gm_rdb14;
        *var_gm_rdb15_slot = var_gm_rdb15;
        *var_gm_rdb16_slot = var_gm_rdb16;
        *var_gm_rdb17_slot = var_gm_rdb17;
        *var_gm_rdb18_slot = var_gm_rdb18;
        *var_gm_rdb2_slot = var_gm_rdb2;
        *var_gm_rdb3_slot = var_gm_rdb3;
        *var_gm_rdb4_slot = var_gm_rdb4;
        *var_gm_rdb5_slot = var_gm_rdb5;
        *var_gm_rdb6_slot = var_gm_rdb6;
        *var_gm_rdb7_slot = var_gm_rdb7;
        *var_gm_rdb8_slot = var_gm_rdb8;
        *var_gm_rdb9_slot = var_gm_rdb9;
        *var_gm_rdn0_slot = var_gm_rdn0;
        *var_gm_rdn1_slot = var_gm_rdn1;
        *var_gm_rdn10_slot = var_gm_rdn10;
        *var_gm_rdn11_slot = var_gm_rdn11;
        *var_gm_rdn12_slot = var_gm_rdn12;
        *var_gm_rdn13_slot = var_gm_rdn13;
        *var_gm_rdn14_slot = var_gm_rdn14;
        *var_gm_rdn15_slot = var_gm_rdn15;
        *var_gm_rdn16_slot = var_gm_rdn16;
        *var_gm_rdn17_slot = var_gm_rdn17;
        *var_gm_rdn18_slot = var_gm_rdn18;
        *var_gm_rdn2_slot = var_gm_rdn2;
        *var_gm_rdn3_slot = var_gm_rdn3;
        *var_gm_rdn4_slot = var_gm_rdn4;
        *var_gm_rdn5_slot = var_gm_rdn5;
        *var_gm_rdn6_slot = var_gm_rdn6;
        *var_gm_rdn7_slot = var_gm_rdn7;
        *var_gm_rdn8_slot = var_gm_rdn8;
        *var_gm_rdn9_slot = var_gm_rdn9;
        *var_gm_rv_slot = var_gm_rv;
        *var_guard28_slot = var_guard28;
        *var_guard28_db0_slot = var_guard28_db0;
        *var_guard28_db1_slot = var_guard28_db1;
        *var_guard28_db10_slot = var_guard28_db10;
        *var_guard28_db11_slot = var_guard28_db11;
        *var_guard28_db12_slot = var_guard28_db12;
        *var_guard28_db13_slot = var_guard28_db13;
        *var_guard28_db14_slot = var_guard28_db14;
        *var_guard28_db15_slot = var_guard28_db15;
        *var_guard28_db16_slot = var_guard28_db16;
        *var_guard28_db17_slot = var_guard28_db17;
        *var_guard28_db18_slot = var_guard28_db18;
        *var_guard28_db2_slot = var_guard28_db2;
        *var_guard28_db3_slot = var_guard28_db3;
        *var_guard28_db4_slot = var_guard28_db4;
        *var_guard28_db5_slot = var_guard28_db5;
        *var_guard28_db6_slot = var_guard28_db6;
        *var_guard28_db7_slot = var_guard28_db7;
        *var_guard28_db8_slot = var_guard28_db8;
        *var_guard28_db9_slot = var_guard28_db9;
        *var_guard28_dn0_slot = var_guard28_dn0;
        *var_guard28_dn1_slot = var_guard28_dn1;
        *var_guard28_dn10_slot = var_guard28_dn10;
        *var_guard28_dn11_slot = var_guard28_dn11;
        *var_guard28_dn12_slot = var_guard28_dn12;
        *var_guard28_dn13_slot = var_guard28_dn13;
        *var_guard28_dn14_slot = var_guard28_dn14;
        *var_guard28_dn15_slot = var_guard28_dn15;
        *var_guard28_dn16_slot = var_guard28_dn16;
        *var_guard28_dn17_slot = var_guard28_dn17;
        *var_guard28_dn18_slot = var_guard28_dn18;
        *var_guard28_dn2_slot = var_guard28_dn2;
        *var_guard28_dn3_slot = var_guard28_dn3;
        *var_guard28_dn4_slot = var_guard28_dn4;
        *var_guard28_dn5_slot = var_guard28_dn5;
        *var_guard28_dn6_slot = var_guard28_dn6;
        *var_guard28_dn7_slot = var_guard28_dn7;
        *var_guard28_dn8_slot = var_guard28_dn8;
        *var_guard28_dn9_slot = var_guard28_dn9;
        *var_guard28_rdb0_slot = var_guard28_rdb0;
        *var_guard28_rdb1_slot = var_guard28_rdb1;
        *var_guard28_rdb10_slot = var_guard28_rdb10;
        *var_guard28_rdb11_slot = var_guard28_rdb11;
        *var_guard28_rdb12_slot = var_guard28_rdb12;
        *var_guard28_rdb13_slot = var_guard28_rdb13;
        *var_guard28_rdb14_slot = var_guard28_rdb14;
        *var_guard28_rdb15_slot = var_guard28_rdb15;
        *var_guard28_rdb16_slot = var_guard28_rdb16;
        *var_guard28_rdb17_slot = var_guard28_rdb17;
        *var_guard28_rdb18_slot = var_guard28_rdb18;
        *var_guard28_rdb2_slot = var_guard28_rdb2;
        *var_guard28_rdb3_slot = var_guard28_rdb3;
        *var_guard28_rdb4_slot = var_guard28_rdb4;
        *var_guard28_rdb5_slot = var_guard28_rdb5;
        *var_guard28_rdb6_slot = var_guard28_rdb6;
        *var_guard28_rdb7_slot = var_guard28_rdb7;
        *var_guard28_rdb8_slot = var_guard28_rdb8;
        *var_guard28_rdb9_slot = var_guard28_rdb9;
        *var_guard28_rdn0_slot = var_guard28_rdn0;
        *var_guard28_rdn1_slot = var_guard28_rdn1;
        *var_guard28_rdn10_slot = var_guard28_rdn10;
        *var_guard28_rdn11_slot = var_guard28_rdn11;
        *var_guard28_rdn12_slot = var_guard28_rdn12;
        *var_guard28_rdn13_slot = var_guard28_rdn13;
        *var_guard28_rdn14_slot = var_guard28_rdn14;
        *var_guard28_rdn15_slot = var_guard28_rdn15;
        *var_guard28_rdn16_slot = var_guard28_rdn16;
        *var_guard28_rdn17_slot = var_guard28_rdn17;
        *var_guard28_rdn18_slot = var_guard28_rdn18;
        *var_guard28_rdn2_slot = var_guard28_rdn2;
        *var_guard28_rdn3_slot = var_guard28_rdn3;
        *var_guard28_rdn4_slot = var_guard28_rdn4;
        *var_guard28_rdn5_slot = var_guard28_rdn5;
        *var_guard28_rdn6_slot = var_guard28_rdn6;
        *var_guard28_rdn7_slot = var_guard28_rdn7;
        *var_guard28_rdn8_slot = var_guard28_rdn8;
        *var_guard28_rdn9_slot = var_guard28_rdn9;
        *var_guard28_rv_slot = var_guard28_rv;
        *var_guard29_slot = var_guard29;
        *var_guard29_db0_slot = var_guard29_db0;
        *var_guard29_db1_slot = var_guard29_db1;
        *var_guard29_db10_slot = var_guard29_db10;
        *var_guard29_db11_slot = var_guard29_db11;
        *var_guard29_db12_slot = var_guard29_db12;
        *var_guard29_db13_slot = var_guard29_db13;
        *var_guard29_db14_slot = var_guard29_db14;
        *var_guard29_db15_slot = var_guard29_db15;
        *var_guard29_db16_slot = var_guard29_db16;
        *var_guard29_db17_slot = var_guard29_db17;
        *var_guard29_db18_slot = var_guard29_db18;
        *var_guard29_db2_slot = var_guard29_db2;
        *var_guard29_db3_slot = var_guard29_db3;
        *var_guard29_db4_slot = var_guard29_db4;
        *var_guard29_db5_slot = var_guard29_db5;
        *var_guard29_db6_slot = var_guard29_db6;
        *var_guard29_db7_slot = var_guard29_db7;
        *var_guard29_db8_slot = var_guard29_db8;
        *var_guard29_db9_slot = var_guard29_db9;
        *var_guard29_dn0_slot = var_guard29_dn0;
        *var_guard29_dn1_slot = var_guard29_dn1;
        *var_guard29_dn10_slot = var_guard29_dn10;
        *var_guard29_dn11_slot = var_guard29_dn11;
        *var_guard29_dn12_slot = var_guard29_dn12;
        *var_guard29_dn13_slot = var_guard29_dn13;
        *var_guard29_dn14_slot = var_guard29_dn14;
        *var_guard29_dn15_slot = var_guard29_dn15;
        *var_guard29_dn16_slot = var_guard29_dn16;
        *var_guard29_dn17_slot = var_guard29_dn17;
        *var_guard29_dn18_slot = var_guard29_dn18;
        *var_guard29_dn2_slot = var_guard29_dn2;
        *var_guard29_dn3_slot = var_guard29_dn3;
        *var_guard29_dn4_slot = var_guard29_dn4;
        *var_guard29_dn5_slot = var_guard29_dn5;
        *var_guard29_dn6_slot = var_guard29_dn6;
        *var_guard29_dn7_slot = var_guard29_dn7;
        *var_guard29_dn8_slot = var_guard29_dn8;
        *var_guard29_dn9_slot = var_guard29_dn9;
        *var_guard29_rdb0_slot = var_guard29_rdb0;
        *var_guard29_rdb1_slot = var_guard29_rdb1;
        *var_guard29_rdb10_slot = var_guard29_rdb10;
        *var_guard29_rdb11_slot = var_guard29_rdb11;
        *var_guard29_rdb12_slot = var_guard29_rdb12;
        *var_guard29_rdb13_slot = var_guard29_rdb13;
        *var_guard29_rdb14_slot = var_guard29_rdb14;
        *var_guard29_rdb15_slot = var_guard29_rdb15;
        *var_guard29_rdb16_slot = var_guard29_rdb16;
        *var_guard29_rdb17_slot = var_guard29_rdb17;
        *var_guard29_rdb18_slot = var_guard29_rdb18;
        *var_guard29_rdb2_slot = var_guard29_rdb2;
        *var_guard29_rdb3_slot = var_guard29_rdb3;
        *var_guard29_rdb4_slot = var_guard29_rdb4;
        *var_guard29_rdb5_slot = var_guard29_rdb5;
        *var_guard29_rdb6_slot = var_guard29_rdb6;
        *var_guard29_rdb7_slot = var_guard29_rdb7;
        *var_guard29_rdb8_slot = var_guard29_rdb8;
        *var_guard29_rdb9_slot = var_guard29_rdb9;
        *var_guard29_rdn0_slot = var_guard29_rdn0;
        *var_guard29_rdn1_slot = var_guard29_rdn1;
        *var_guard29_rdn10_slot = var_guard29_rdn10;
        *var_guard29_rdn11_slot = var_guard29_rdn11;
        *var_guard29_rdn12_slot = var_guard29_rdn12;
        *var_guard29_rdn13_slot = var_guard29_rdn13;
        *var_guard29_rdn14_slot = var_guard29_rdn14;
        *var_guard29_rdn15_slot = var_guard29_rdn15;
        *var_guard29_rdn16_slot = var_guard29_rdn16;
        *var_guard29_rdn17_slot = var_guard29_rdn17;
        *var_guard29_rdn18_slot = var_guard29_rdn18;
        *var_guard29_rdn2_slot = var_guard29_rdn2;
        *var_guard29_rdn3_slot = var_guard29_rdn3;
        *var_guard29_rdn4_slot = var_guard29_rdn4;
        *var_guard29_rdn5_slot = var_guard29_rdn5;
        *var_guard29_rdn6_slot = var_guard29_rdn6;
        *var_guard29_rdn7_slot = var_guard29_rdn7;
        *var_guard29_rdn8_slot = var_guard29_rdn8;
        *var_guard29_rdn9_slot = var_guard29_rdn9;
        *var_guard29_rv_slot = var_guard29_rv;
        *var_idtn_slot = var_idtn;
        *var_idtn_db0_slot = var_idtn_db0;
        *var_idtn_db1_slot = var_idtn_db1;
        *var_idtn_db10_slot = var_idtn_db10;
        *var_idtn_db11_slot = var_idtn_db11;
        *var_idtn_db12_slot = var_idtn_db12;
        *var_idtn_db13_slot = var_idtn_db13;
        *var_idtn_db14_slot = var_idtn_db14;
        *var_idtn_db15_slot = var_idtn_db15;
        *var_idtn_db16_slot = var_idtn_db16;
        *var_idtn_db17_slot = var_idtn_db17;
        *var_idtn_db18_slot = var_idtn_db18;
        *var_idtn_db2_slot = var_idtn_db2;
        *var_idtn_db3_slot = var_idtn_db3;
        *var_idtn_db4_slot = var_idtn_db4;
        *var_idtn_db5_slot = var_idtn_db5;
        *var_idtn_db6_slot = var_idtn_db6;
        *var_idtn_db7_slot = var_idtn_db7;
        *var_idtn_db8_slot = var_idtn_db8;
        *var_idtn_db9_slot = var_idtn_db9;
        *var_idtn_dn0_slot = var_idtn_dn0;
        *var_idtn_dn1_slot = var_idtn_dn1;
        *var_idtn_dn10_slot = var_idtn_dn10;
        *var_idtn_dn11_slot = var_idtn_dn11;
        *var_idtn_dn12_slot = var_idtn_dn12;
        *var_idtn_dn13_slot = var_idtn_dn13;
        *var_idtn_dn14_slot = var_idtn_dn14;
        *var_idtn_dn15_slot = var_idtn_dn15;
        *var_idtn_dn16_slot = var_idtn_dn16;
        *var_idtn_dn17_slot = var_idtn_dn17;
        *var_idtn_dn18_slot = var_idtn_dn18;
        *var_idtn_dn2_slot = var_idtn_dn2;
        *var_idtn_dn3_slot = var_idtn_dn3;
        *var_idtn_dn4_slot = var_idtn_dn4;
        *var_idtn_dn5_slot = var_idtn_dn5;
        *var_idtn_dn6_slot = var_idtn_dn6;
        *var_idtn_dn7_slot = var_idtn_dn7;
        *var_idtn_dn8_slot = var_idtn_dn8;
        *var_idtn_dn9_slot = var_idtn_dn9;
        *var_idtn_rdb0_slot = var_idtn_rdb0;
        *var_idtn_rdb1_slot = var_idtn_rdb1;
        *var_idtn_rdb10_slot = var_idtn_rdb10;
        *var_idtn_rdb11_slot = var_idtn_rdb11;
        *var_idtn_rdb12_slot = var_idtn_rdb12;
        *var_idtn_rdb13_slot = var_idtn_rdb13;
        *var_idtn_rdb14_slot = var_idtn_rdb14;
        *var_idtn_rdb15_slot = var_idtn_rdb15;
        *var_idtn_rdb16_slot = var_idtn_rdb16;
        *var_idtn_rdb17_slot = var_idtn_rdb17;
        *var_idtn_rdb18_slot = var_idtn_rdb18;
        *var_idtn_rdb2_slot = var_idtn_rdb2;
        *var_idtn_rdb3_slot = var_idtn_rdb3;
        *var_idtn_rdb4_slot = var_idtn_rdb4;
        *var_idtn_rdb5_slot = var_idtn_rdb5;
        *var_idtn_rdb6_slot = var_idtn_rdb6;
        *var_idtn_rdb7_slot = var_idtn_rdb7;
        *var_idtn_rdb8_slot = var_idtn_rdb8;
        *var_idtn_rdb9_slot = var_idtn_rdb9;
        *var_idtn_rdn0_slot = var_idtn_rdn0;
        *var_idtn_rdn1_slot = var_idtn_rdn1;
        *var_idtn_rdn10_slot = var_idtn_rdn10;
        *var_idtn_rdn11_slot = var_idtn_rdn11;
        *var_idtn_rdn12_slot = var_idtn_rdn12;
        *var_idtn_rdn13_slot = var_idtn_rdn13;
        *var_idtn_rdn14_slot = var_idtn_rdn14;
        *var_idtn_rdn15_slot = var_idtn_rdn15;
        *var_idtn_rdn16_slot = var_idtn_rdn16;
        *var_idtn_rdn17_slot = var_idtn_rdn17;
        *var_idtn_rdn18_slot = var_idtn_rdn18;
        *var_idtn_rdn2_slot = var_idtn_rdn2;
        *var_idtn_rdn3_slot = var_idtn_rdn3;
        *var_idtn_rdn4_slot = var_idtn_rdn4;
        *var_idtn_rdn5_slot = var_idtn_rdn5;
        *var_idtn_rdn6_slot = var_idtn_rdn6;
        *var_idtn_rdn7_slot = var_idtn_rdn7;
        *var_idtn_rdn8_slot = var_idtn_rdn8;
        *var_idtn_rdn9_slot = var_idtn_rdn9;
        *var_idtn_rv_slot = var_idtn_rv;
        *var_td_prime_slot = var_td_prime;
        *var_td_prime_db0_slot = var_td_prime_db0;
        *var_td_prime_db1_slot = var_td_prime_db1;
        *var_td_prime_db10_slot = var_td_prime_db10;
        *var_td_prime_db11_slot = var_td_prime_db11;
        *var_td_prime_db12_slot = var_td_prime_db12;
        *var_td_prime_db13_slot = var_td_prime_db13;
        *var_td_prime_db14_slot = var_td_prime_db14;
        *var_td_prime_db15_slot = var_td_prime_db15;
        *var_td_prime_db16_slot = var_td_prime_db16;
        *var_td_prime_db17_slot = var_td_prime_db17;
        *var_td_prime_db18_slot = var_td_prime_db18;
        *var_td_prime_db2_slot = var_td_prime_db2;
        *var_td_prime_db3_slot = var_td_prime_db3;
        *var_td_prime_db4_slot = var_td_prime_db4;
        *var_td_prime_db5_slot = var_td_prime_db5;
        *var_td_prime_db6_slot = var_td_prime_db6;
        *var_td_prime_db7_slot = var_td_prime_db7;
        *var_td_prime_db8_slot = var_td_prime_db8;
        *var_td_prime_db9_slot = var_td_prime_db9;
        *var_td_prime_dn0_slot = var_td_prime_dn0;
        *var_td_prime_dn1_slot = var_td_prime_dn1;
        *var_td_prime_dn10_slot = var_td_prime_dn10;
        *var_td_prime_dn11_slot = var_td_prime_dn11;
        *var_td_prime_dn12_slot = var_td_prime_dn12;
        *var_td_prime_dn13_slot = var_td_prime_dn13;
        *var_td_prime_dn14_slot = var_td_prime_dn14;
        *var_td_prime_dn15_slot = var_td_prime_dn15;
        *var_td_prime_dn16_slot = var_td_prime_dn16;
        *var_td_prime_dn17_slot = var_td_prime_dn17;
        *var_td_prime_dn18_slot = var_td_prime_dn18;
        *var_td_prime_dn2_slot = var_td_prime_dn2;
        *var_td_prime_dn3_slot = var_td_prime_dn3;
        *var_td_prime_dn4_slot = var_td_prime_dn4;
        *var_td_prime_dn5_slot = var_td_prime_dn5;
        *var_td_prime_dn6_slot = var_td_prime_dn6;
        *var_td_prime_dn7_slot = var_td_prime_dn7;
        *var_td_prime_dn8_slot = var_td_prime_dn8;
        *var_td_prime_dn9_slot = var_td_prime_dn9;
        *var_td_prime_rdb0_slot = var_td_prime_rdb0;
        *var_td_prime_rdb1_slot = var_td_prime_rdb1;
        *var_td_prime_rdb10_slot = var_td_prime_rdb10;
        *var_td_prime_rdb11_slot = var_td_prime_rdb11;
        *var_td_prime_rdb12_slot = var_td_prime_rdb12;
        *var_td_prime_rdb13_slot = var_td_prime_rdb13;
        *var_td_prime_rdb14_slot = var_td_prime_rdb14;
        *var_td_prime_rdb15_slot = var_td_prime_rdb15;
        *var_td_prime_rdb16_slot = var_td_prime_rdb16;
        *var_td_prime_rdb17_slot = var_td_prime_rdb17;
        *var_td_prime_rdb18_slot = var_td_prime_rdb18;
        *var_td_prime_rdb2_slot = var_td_prime_rdb2;
        *var_td_prime_rdb3_slot = var_td_prime_rdb3;
        *var_td_prime_rdb4_slot = var_td_prime_rdb4;
        *var_td_prime_rdb5_slot = var_td_prime_rdb5;
        *var_td_prime_rdb6_slot = var_td_prime_rdb6;
        *var_td_prime_rdb7_slot = var_td_prime_rdb7;
        *var_td_prime_rdb8_slot = var_td_prime_rdb8;
        *var_td_prime_rdb9_slot = var_td_prime_rdb9;
        *var_td_prime_rdn0_slot = var_td_prime_rdn0;
        *var_td_prime_rdn1_slot = var_td_prime_rdn1;
        *var_td_prime_rdn10_slot = var_td_prime_rdn10;
        *var_td_prime_rdn11_slot = var_td_prime_rdn11;
        *var_td_prime_rdn12_slot = var_td_prime_rdn12;
        *var_td_prime_rdn13_slot = var_td_prime_rdn13;
        *var_td_prime_rdn14_slot = var_td_prime_rdn14;
        *var_td_prime_rdn15_slot = var_td_prime_rdn15;
        *var_td_prime_rdn16_slot = var_td_prime_rdn16;
        *var_td_prime_rdn17_slot = var_td_prime_rdn17;
        *var_td_prime_rdn18_slot = var_td_prime_rdn18;
        *var_td_prime_rdn2_slot = var_td_prime_rdn2;
        *var_td_prime_rdn3_slot = var_td_prime_rdn3;
        *var_td_prime_rdn4_slot = var_td_prime_rdn4;
        *var_td_prime_rdn5_slot = var_td_prime_rdn5;
        *var_td_prime_rdn6_slot = var_td_prime_rdn6;
        *var_td_prime_rdn7_slot = var_td_prime_rdn7;
        *var_td_prime_rdn8_slot = var_td_prime_rdn8;
        *var_td_prime_rdn9_slot = var_td_prime_rdn9;
        *var_td_prime_rv_slot = var_td_prime_rv;
    }

    pub(super) fn stamp_reactive_block_40(
        p: &Parameters,
        var_cgs0_t: f64,
        var_cgs0_t_db0: f64,
        var_cgs0_t_db1: f64,
        var_cgs0_t_db10: f64,
        var_cgs0_t_db11: f64,
        var_cgs0_t_db12: f64,
        var_cgs0_t_db13: f64,
        var_cgs0_t_db14: f64,
        var_cgs0_t_db15: f64,
        var_cgs0_t_db16: f64,
        var_cgs0_t_db17: f64,
        var_cgs0_t_db18: f64,
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
        var_cgs0_t_dn16: f64,
        var_cgs0_t_dn17: f64,
        var_cgs0_t_dn18: f64,
        var_cgs0_t_dn2: f64,
        var_cgs0_t_dn3: f64,
        var_cgs0_t_dn4: f64,
        var_cgs0_t_dn5: f64,
        var_cgs0_t_dn6: f64,
        var_cgs0_t_dn7: f64,
        var_cgs0_t_dn8: f64,
        var_cgs0_t_dn9: f64,
        var_gm: f64,
        var_gm_db0: f64,
        var_gm_db1: f64,
        var_gm_db10: f64,
        var_gm_db11: f64,
        var_gm_db12: f64,
        var_gm_db13: f64,
        var_gm_db14: f64,
        var_gm_db15: f64,
        var_gm_db16: f64,
        var_gm_db17: f64,
        var_gm_db18: f64,
        var_gm_db2: f64,
        var_gm_db3: f64,
        var_gm_db4: f64,
        var_gm_db5: f64,
        var_gm_db6: f64,
        var_gm_db7: f64,
        var_gm_db8: f64,
        var_gm_db9: f64,
        var_gm_dn0: f64,
        var_gm_dn1: f64,
        var_gm_dn10: f64,
        var_gm_dn11: f64,
        var_gm_dn12: f64,
        var_gm_dn13: f64,
        var_gm_dn14: f64,
        var_gm_dn15: f64,
        var_gm_dn16: f64,
        var_gm_dn17: f64,
        var_gm_dn18: f64,
        var_gm_dn2: f64,
        var_gm_dn3: f64,
        var_gm_dn4: f64,
        var_gm_dn5: f64,
        var_gm_dn6: f64,
        var_gm_dn7: f64,
        var_gm_dn8: f64,
        var_gm_dn9: f64,
        var_guard28: f64,
        var_guard29: f64,
        var_idtn: f64,
        var_idtn_db0: f64,
        var_idtn_db1: f64,
        var_idtn_db10: f64,
        var_idtn_db11: f64,
        var_idtn_db12: f64,
        var_idtn_db13: f64,
        var_idtn_db14: f64,
        var_idtn_db15: f64,
        var_idtn_db16: f64,
        var_idtn_db17: f64,
        var_idtn_db18: f64,
        var_idtn_db2: f64,
        var_idtn_db3: f64,
        var_idtn_db4: f64,
        var_idtn_db5: f64,
        var_idtn_db6: f64,
        var_idtn_db7: f64,
        var_idtn_db8: f64,
        var_idtn_db9: f64,
        var_idtn_dn0: f64,
        var_idtn_dn1: f64,
        var_idtn_dn10: f64,
        var_idtn_dn11: f64,
        var_idtn_dn12: f64,
        var_idtn_dn13: f64,
        var_idtn_dn14: f64,
        var_idtn_dn15: f64,
        var_idtn_dn16: f64,
        var_idtn_dn17: f64,
        var_idtn_dn18: f64,
        var_idtn_dn2: f64,
        var_idtn_dn3: f64,
        var_idtn_dn4: f64,
        var_idtn_dn5: f64,
        var_idtn_dn6: f64,
        var_idtn_dn7: f64,
        var_idtn_dn8: f64,
        var_idtn_dn9: f64,
        var_t: f64,
        var_t_db0: f64,
        var_t_db1: f64,
        var_t_db10: f64,
        var_t_db11: f64,
        var_t_db12: f64,
        var_t_db13: f64,
        var_t_db14: f64,
        var_t_db15: f64,
        var_t_db16: f64,
        var_t_db17: f64,
        var_t_db18: f64,
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
        var_t_dn16: f64,
        var_t_dn17: f64,
        var_t_dn18: f64,
        var_t_dn2: f64,
        var_t_dn3: f64,
        var_t_dn4: f64,
        var_t_dn5: f64,
        var_t_dn6: f64,
        var_t_dn7: f64,
        var_t_dn8: f64,
        var_t_dn9: f64,
        var_td_prime: f64,
        var_td_prime_db0: f64,
        var_td_prime_db1: f64,
        var_td_prime_db10: f64,
        var_td_prime_db11: f64,
        var_td_prime_db12: f64,
        var_td_prime_db13: f64,
        var_td_prime_db14: f64,
        var_td_prime_db15: f64,
        var_td_prime_db16: f64,
        var_td_prime_db17: f64,
        var_td_prime_db18: f64,
        var_td_prime_db2: f64,
        var_td_prime_db3: f64,
        var_td_prime_db4: f64,
        var_td_prime_db5: f64,
        var_td_prime_db6: f64,
        var_td_prime_db7: f64,
        var_td_prime_db8: f64,
        var_td_prime_db9: f64,
        var_td_prime_dn0: f64,
        var_td_prime_dn1: f64,
        var_td_prime_dn10: f64,
        var_td_prime_dn11: f64,
        var_td_prime_dn12: f64,
        var_td_prime_dn13: f64,
        var_td_prime_dn14: f64,
        var_td_prime_dn15: f64,
        var_td_prime_dn16: f64,
        var_td_prime_dn17: f64,
        var_td_prime_dn18: f64,
        var_td_prime_dn2: f64,
        var_td_prime_dn3: f64,
        var_td_prime_dn4: f64,
        var_td_prime_dn5: f64,
        var_td_prime_dn6: f64,
        var_td_prime_dn7: f64,
        var_td_prime_dn8: f64,
        var_td_prime_dn9: f64,
        var_guard36_slot: &mut f64,
        var_guard36_db0_slot: &mut f64,
        var_guard36_db1_slot: &mut f64,
        var_guard36_db10_slot: &mut f64,
        var_guard36_db11_slot: &mut f64,
        var_guard36_db12_slot: &mut f64,
        var_guard36_db13_slot: &mut f64,
        var_guard36_db14_slot: &mut f64,
        var_guard36_db15_slot: &mut f64,
        var_guard36_db16_slot: &mut f64,
        var_guard36_db17_slot: &mut f64,
        var_guard36_db18_slot: &mut f64,
        var_guard36_db2_slot: &mut f64,
        var_guard36_db3_slot: &mut f64,
        var_guard36_db4_slot: &mut f64,
        var_guard36_db5_slot: &mut f64,
        var_guard36_db6_slot: &mut f64,
        var_guard36_db7_slot: &mut f64,
        var_guard36_db8_slot: &mut f64,
        var_guard36_db9_slot: &mut f64,
        var_guard36_dn0_slot: &mut f64,
        var_guard36_dn1_slot: &mut f64,
        var_guard36_dn10_slot: &mut f64,
        var_guard36_dn11_slot: &mut f64,
        var_guard36_dn12_slot: &mut f64,
        var_guard36_dn13_slot: &mut f64,
        var_guard36_dn14_slot: &mut f64,
        var_guard36_dn15_slot: &mut f64,
        var_guard36_dn16_slot: &mut f64,
        var_guard36_dn17_slot: &mut f64,
        var_guard36_dn18_slot: &mut f64,
        var_guard36_dn2_slot: &mut f64,
        var_guard36_dn3_slot: &mut f64,
        var_guard36_dn4_slot: &mut f64,
        var_guard36_dn5_slot: &mut f64,
        var_guard36_dn6_slot: &mut f64,
        var_guard36_dn7_slot: &mut f64,
        var_guard36_dn8_slot: &mut f64,
        var_guard36_dn9_slot: &mut f64,
        var_guard36_rdb0_slot: &mut f64,
        var_guard36_rdb1_slot: &mut f64,
        var_guard36_rdb10_slot: &mut f64,
        var_guard36_rdb11_slot: &mut f64,
        var_guard36_rdb12_slot: &mut f64,
        var_guard36_rdb13_slot: &mut f64,
        var_guard36_rdb14_slot: &mut f64,
        var_guard36_rdb15_slot: &mut f64,
        var_guard36_rdb16_slot: &mut f64,
        var_guard36_rdb17_slot: &mut f64,
        var_guard36_rdb18_slot: &mut f64,
        var_guard36_rdb2_slot: &mut f64,
        var_guard36_rdb3_slot: &mut f64,
        var_guard36_rdb4_slot: &mut f64,
        var_guard36_rdb5_slot: &mut f64,
        var_guard36_rdb6_slot: &mut f64,
        var_guard36_rdb7_slot: &mut f64,
        var_guard36_rdb8_slot: &mut f64,
        var_guard36_rdb9_slot: &mut f64,
        var_guard36_rdn0_slot: &mut f64,
        var_guard36_rdn1_slot: &mut f64,
        var_guard36_rdn10_slot: &mut f64,
        var_guard36_rdn11_slot: &mut f64,
        var_guard36_rdn12_slot: &mut f64,
        var_guard36_rdn13_slot: &mut f64,
        var_guard36_rdn14_slot: &mut f64,
        var_guard36_rdn15_slot: &mut f64,
        var_guard36_rdn16_slot: &mut f64,
        var_guard36_rdn17_slot: &mut f64,
        var_guard36_rdn18_slot: &mut f64,
        var_guard36_rdn2_slot: &mut f64,
        var_guard36_rdn3_slot: &mut f64,
        var_guard36_rdn4_slot: &mut f64,
        var_guard36_rdn5_slot: &mut f64,
        var_guard36_rdn6_slot: &mut f64,
        var_guard36_rdn7_slot: &mut f64,
        var_guard36_rdn8_slot: &mut f64,
        var_guard36_rdn9_slot: &mut f64,
        var_guard36_rv_slot: &mut f64,
        var_noisepwr_slot: &mut f64,
        var_noisepwr_db0_slot: &mut f64,
        var_noisepwr_db1_slot: &mut f64,
        var_noisepwr_db10_slot: &mut f64,
        var_noisepwr_db11_slot: &mut f64,
        var_noisepwr_db12_slot: &mut f64,
        var_noisepwr_db13_slot: &mut f64,
        var_noisepwr_db14_slot: &mut f64,
        var_noisepwr_db15_slot: &mut f64,
        var_noisepwr_db16_slot: &mut f64,
        var_noisepwr_db17_slot: &mut f64,
        var_noisepwr_db18_slot: &mut f64,
        var_noisepwr_db2_slot: &mut f64,
        var_noisepwr_db3_slot: &mut f64,
        var_noisepwr_db4_slot: &mut f64,
        var_noisepwr_db5_slot: &mut f64,
        var_noisepwr_db6_slot: &mut f64,
        var_noisepwr_db7_slot: &mut f64,
        var_noisepwr_db8_slot: &mut f64,
        var_noisepwr_db9_slot: &mut f64,
        var_noisepwr_dn0_slot: &mut f64,
        var_noisepwr_dn1_slot: &mut f64,
        var_noisepwr_dn10_slot: &mut f64,
        var_noisepwr_dn11_slot: &mut f64,
        var_noisepwr_dn12_slot: &mut f64,
        var_noisepwr_dn13_slot: &mut f64,
        var_noisepwr_dn14_slot: &mut f64,
        var_noisepwr_dn15_slot: &mut f64,
        var_noisepwr_dn16_slot: &mut f64,
        var_noisepwr_dn17_slot: &mut f64,
        var_noisepwr_dn18_slot: &mut f64,
        var_noisepwr_dn2_slot: &mut f64,
        var_noisepwr_dn3_slot: &mut f64,
        var_noisepwr_dn4_slot: &mut f64,
        var_noisepwr_dn5_slot: &mut f64,
        var_noisepwr_dn6_slot: &mut f64,
        var_noisepwr_dn7_slot: &mut f64,
        var_noisepwr_dn8_slot: &mut f64,
        var_noisepwr_dn9_slot: &mut f64,
        var_noisepwr_rdb0_slot: &mut f64,
        var_noisepwr_rdb1_slot: &mut f64,
        var_noisepwr_rdb10_slot: &mut f64,
        var_noisepwr_rdb11_slot: &mut f64,
        var_noisepwr_rdb12_slot: &mut f64,
        var_noisepwr_rdb13_slot: &mut f64,
        var_noisepwr_rdb14_slot: &mut f64,
        var_noisepwr_rdb15_slot: &mut f64,
        var_noisepwr_rdb16_slot: &mut f64,
        var_noisepwr_rdb17_slot: &mut f64,
        var_noisepwr_rdb18_slot: &mut f64,
        var_noisepwr_rdb2_slot: &mut f64,
        var_noisepwr_rdb3_slot: &mut f64,
        var_noisepwr_rdb4_slot: &mut f64,
        var_noisepwr_rdb5_slot: &mut f64,
        var_noisepwr_rdb6_slot: &mut f64,
        var_noisepwr_rdb7_slot: &mut f64,
        var_noisepwr_rdb8_slot: &mut f64,
        var_noisepwr_rdb9_slot: &mut f64,
        var_noisepwr_rdn0_slot: &mut f64,
        var_noisepwr_rdn1_slot: &mut f64,
        var_noisepwr_rdn10_slot: &mut f64,
        var_noisepwr_rdn11_slot: &mut f64,
        var_noisepwr_rdn12_slot: &mut f64,
        var_noisepwr_rdn13_slot: &mut f64,
        var_noisepwr_rdn14_slot: &mut f64,
        var_noisepwr_rdn15_slot: &mut f64,
        var_noisepwr_rdn16_slot: &mut f64,
        var_noisepwr_rdn17_slot: &mut f64,
        var_noisepwr_rdn18_slot: &mut f64,
        var_noisepwr_rdn2_slot: &mut f64,
        var_noisepwr_rdn3_slot: &mut f64,
        var_noisepwr_rdn4_slot: &mut f64,
        var_noisepwr_rdn5_slot: &mut f64,
        var_noisepwr_rdn6_slot: &mut f64,
        var_noisepwr_rdn7_slot: &mut f64,
        var_noisepwr_rdn8_slot: &mut f64,
        var_noisepwr_rdn9_slot: &mut f64,
        var_noisepwr_rv_slot: &mut f64,
        var_noisepwrd_slot: &mut f64,
        var_noisepwrd_db0_slot: &mut f64,
        var_noisepwrd_db1_slot: &mut f64,
        var_noisepwrd_db10_slot: &mut f64,
        var_noisepwrd_db11_slot: &mut f64,
        var_noisepwrd_db12_slot: &mut f64,
        var_noisepwrd_db13_slot: &mut f64,
        var_noisepwrd_db14_slot: &mut f64,
        var_noisepwrd_db15_slot: &mut f64,
        var_noisepwrd_db16_slot: &mut f64,
        var_noisepwrd_db17_slot: &mut f64,
        var_noisepwrd_db18_slot: &mut f64,
        var_noisepwrd_db2_slot: &mut f64,
        var_noisepwrd_db3_slot: &mut f64,
        var_noisepwrd_db4_slot: &mut f64,
        var_noisepwrd_db5_slot: &mut f64,
        var_noisepwrd_db6_slot: &mut f64,
        var_noisepwrd_db7_slot: &mut f64,
        var_noisepwrd_db8_slot: &mut f64,
        var_noisepwrd_db9_slot: &mut f64,
        var_noisepwrd_dn0_slot: &mut f64,
        var_noisepwrd_dn1_slot: &mut f64,
        var_noisepwrd_dn10_slot: &mut f64,
        var_noisepwrd_dn11_slot: &mut f64,
        var_noisepwrd_dn12_slot: &mut f64,
        var_noisepwrd_dn13_slot: &mut f64,
        var_noisepwrd_dn14_slot: &mut f64,
        var_noisepwrd_dn15_slot: &mut f64,
        var_noisepwrd_dn16_slot: &mut f64,
        var_noisepwrd_dn17_slot: &mut f64,
        var_noisepwrd_dn18_slot: &mut f64,
        var_noisepwrd_dn2_slot: &mut f64,
        var_noisepwrd_dn3_slot: &mut f64,
        var_noisepwrd_dn4_slot: &mut f64,
        var_noisepwrd_dn5_slot: &mut f64,
        var_noisepwrd_dn6_slot: &mut f64,
        var_noisepwrd_dn7_slot: &mut f64,
        var_noisepwrd_dn8_slot: &mut f64,
        var_noisepwrd_dn9_slot: &mut f64,
        var_noisepwrd_rdb0_slot: &mut f64,
        var_noisepwrd_rdb1_slot: &mut f64,
        var_noisepwrd_rdb10_slot: &mut f64,
        var_noisepwrd_rdb11_slot: &mut f64,
        var_noisepwrd_rdb12_slot: &mut f64,
        var_noisepwrd_rdb13_slot: &mut f64,
        var_noisepwrd_rdb14_slot: &mut f64,
        var_noisepwrd_rdb15_slot: &mut f64,
        var_noisepwrd_rdb16_slot: &mut f64,
        var_noisepwrd_rdb17_slot: &mut f64,
        var_noisepwrd_rdb18_slot: &mut f64,
        var_noisepwrd_rdb2_slot: &mut f64,
        var_noisepwrd_rdb3_slot: &mut f64,
        var_noisepwrd_rdb4_slot: &mut f64,
        var_noisepwrd_rdb5_slot: &mut f64,
        var_noisepwrd_rdb6_slot: &mut f64,
        var_noisepwrd_rdb7_slot: &mut f64,
        var_noisepwrd_rdb8_slot: &mut f64,
        var_noisepwrd_rdb9_slot: &mut f64,
        var_noisepwrd_rdn0_slot: &mut f64,
        var_noisepwrd_rdn1_slot: &mut f64,
        var_noisepwrd_rdn10_slot: &mut f64,
        var_noisepwrd_rdn11_slot: &mut f64,
        var_noisepwrd_rdn12_slot: &mut f64,
        var_noisepwrd_rdn13_slot: &mut f64,
        var_noisepwrd_rdn14_slot: &mut f64,
        var_noisepwrd_rdn15_slot: &mut f64,
        var_noisepwrd_rdn16_slot: &mut f64,
        var_noisepwrd_rdn17_slot: &mut f64,
        var_noisepwrd_rdn18_slot: &mut f64,
        var_noisepwrd_rdn2_slot: &mut f64,
        var_noisepwrd_rdn3_slot: &mut f64,
        var_noisepwrd_rdn4_slot: &mut f64,
        var_noisepwrd_rdn5_slot: &mut f64,
        var_noisepwrd_rdn6_slot: &mut f64,
        var_noisepwrd_rdn7_slot: &mut f64,
        var_noisepwrd_rdn8_slot: &mut f64,
        var_noisepwrd_rdn9_slot: &mut f64,
        var_noisepwrd_rv_slot: &mut f64,
        var_noisepwrg_slot: &mut f64,
        var_noisepwrg_db0_slot: &mut f64,
        var_noisepwrg_db1_slot: &mut f64,
        var_noisepwrg_db10_slot: &mut f64,
        var_noisepwrg_db11_slot: &mut f64,
        var_noisepwrg_db12_slot: &mut f64,
        var_noisepwrg_db13_slot: &mut f64,
        var_noisepwrg_db14_slot: &mut f64,
        var_noisepwrg_db15_slot: &mut f64,
        var_noisepwrg_db16_slot: &mut f64,
        var_noisepwrg_db17_slot: &mut f64,
        var_noisepwrg_db18_slot: &mut f64,
        var_noisepwrg_db2_slot: &mut f64,
        var_noisepwrg_db3_slot: &mut f64,
        var_noisepwrg_db4_slot: &mut f64,
        var_noisepwrg_db5_slot: &mut f64,
        var_noisepwrg_db6_slot: &mut f64,
        var_noisepwrg_db7_slot: &mut f64,
        var_noisepwrg_db8_slot: &mut f64,
        var_noisepwrg_db9_slot: &mut f64,
        var_noisepwrg_dn0_slot: &mut f64,
        var_noisepwrg_dn1_slot: &mut f64,
        var_noisepwrg_dn10_slot: &mut f64,
        var_noisepwrg_dn11_slot: &mut f64,
        var_noisepwrg_dn12_slot: &mut f64,
        var_noisepwrg_dn13_slot: &mut f64,
        var_noisepwrg_dn14_slot: &mut f64,
        var_noisepwrg_dn15_slot: &mut f64,
        var_noisepwrg_dn16_slot: &mut f64,
        var_noisepwrg_dn17_slot: &mut f64,
        var_noisepwrg_dn18_slot: &mut f64,
        var_noisepwrg_dn2_slot: &mut f64,
        var_noisepwrg_dn3_slot: &mut f64,
        var_noisepwrg_dn4_slot: &mut f64,
        var_noisepwrg_dn5_slot: &mut f64,
        var_noisepwrg_dn6_slot: &mut f64,
        var_noisepwrg_dn7_slot: &mut f64,
        var_noisepwrg_dn8_slot: &mut f64,
        var_noisepwrg_dn9_slot: &mut f64,
        var_noisepwrg_rdb0_slot: &mut f64,
        var_noisepwrg_rdb1_slot: &mut f64,
        var_noisepwrg_rdb10_slot: &mut f64,
        var_noisepwrg_rdb11_slot: &mut f64,
        var_noisepwrg_rdb12_slot: &mut f64,
        var_noisepwrg_rdb13_slot: &mut f64,
        var_noisepwrg_rdb14_slot: &mut f64,
        var_noisepwrg_rdb15_slot: &mut f64,
        var_noisepwrg_rdb16_slot: &mut f64,
        var_noisepwrg_rdb17_slot: &mut f64,
        var_noisepwrg_rdb18_slot: &mut f64,
        var_noisepwrg_rdb2_slot: &mut f64,
        var_noisepwrg_rdb3_slot: &mut f64,
        var_noisepwrg_rdb4_slot: &mut f64,
        var_noisepwrg_rdb5_slot: &mut f64,
        var_noisepwrg_rdb6_slot: &mut f64,
        var_noisepwrg_rdb7_slot: &mut f64,
        var_noisepwrg_rdb8_slot: &mut f64,
        var_noisepwrg_rdb9_slot: &mut f64,
        var_noisepwrg_rdn0_slot: &mut f64,
        var_noisepwrg_rdn1_slot: &mut f64,
        var_noisepwrg_rdn10_slot: &mut f64,
        var_noisepwrg_rdn11_slot: &mut f64,
        var_noisepwrg_rdn12_slot: &mut f64,
        var_noisepwrg_rdn13_slot: &mut f64,
        var_noisepwrg_rdn14_slot: &mut f64,
        var_noisepwrg_rdn15_slot: &mut f64,
        var_noisepwrg_rdn16_slot: &mut f64,
        var_noisepwrg_rdn17_slot: &mut f64,
        var_noisepwrg_rdn18_slot: &mut f64,
        var_noisepwrg_rdn2_slot: &mut f64,
        var_noisepwrg_rdn3_slot: &mut f64,
        var_noisepwrg_rdn4_slot: &mut f64,
        var_noisepwrg_rdn5_slot: &mut f64,
        var_noisepwrg_rdn6_slot: &mut f64,
        var_noisepwrg_rdn7_slot: &mut f64,
        var_noisepwrg_rdn8_slot: &mut f64,
        var_noisepwrg_rdn9_slot: &mut f64,
        var_noisepwrg_rv_slot: &mut f64,
    ) {
        let mut var_guard36: f64 = *var_guard36_slot;
        let mut var_guard36_db0: f64 = *var_guard36_db0_slot;
        let mut var_guard36_db1: f64 = *var_guard36_db1_slot;
        let mut var_guard36_db10: f64 = *var_guard36_db10_slot;
        let mut var_guard36_db11: f64 = *var_guard36_db11_slot;
        let mut var_guard36_db12: f64 = *var_guard36_db12_slot;
        let mut var_guard36_db13: f64 = *var_guard36_db13_slot;
        let mut var_guard36_db14: f64 = *var_guard36_db14_slot;
        let mut var_guard36_db15: f64 = *var_guard36_db15_slot;
        let mut var_guard36_db16: f64 = *var_guard36_db16_slot;
        let mut var_guard36_db17: f64 = *var_guard36_db17_slot;
        let mut var_guard36_db18: f64 = *var_guard36_db18_slot;
        let mut var_guard36_db2: f64 = *var_guard36_db2_slot;
        let mut var_guard36_db3: f64 = *var_guard36_db3_slot;
        let mut var_guard36_db4: f64 = *var_guard36_db4_slot;
        let mut var_guard36_db5: f64 = *var_guard36_db5_slot;
        let mut var_guard36_db6: f64 = *var_guard36_db6_slot;
        let mut var_guard36_db7: f64 = *var_guard36_db7_slot;
        let mut var_guard36_db8: f64 = *var_guard36_db8_slot;
        let mut var_guard36_db9: f64 = *var_guard36_db9_slot;
        let mut var_guard36_dn0: f64 = *var_guard36_dn0_slot;
        let mut var_guard36_dn1: f64 = *var_guard36_dn1_slot;
        let mut var_guard36_dn10: f64 = *var_guard36_dn10_slot;
        let mut var_guard36_dn11: f64 = *var_guard36_dn11_slot;
        let mut var_guard36_dn12: f64 = *var_guard36_dn12_slot;
        let mut var_guard36_dn13: f64 = *var_guard36_dn13_slot;
        let mut var_guard36_dn14: f64 = *var_guard36_dn14_slot;
        let mut var_guard36_dn15: f64 = *var_guard36_dn15_slot;
        let mut var_guard36_dn16: f64 = *var_guard36_dn16_slot;
        let mut var_guard36_dn17: f64 = *var_guard36_dn17_slot;
        let mut var_guard36_dn18: f64 = *var_guard36_dn18_slot;
        let mut var_guard36_dn2: f64 = *var_guard36_dn2_slot;
        let mut var_guard36_dn3: f64 = *var_guard36_dn3_slot;
        let mut var_guard36_dn4: f64 = *var_guard36_dn4_slot;
        let mut var_guard36_dn5: f64 = *var_guard36_dn5_slot;
        let mut var_guard36_dn6: f64 = *var_guard36_dn6_slot;
        let mut var_guard36_dn7: f64 = *var_guard36_dn7_slot;
        let mut var_guard36_dn8: f64 = *var_guard36_dn8_slot;
        let mut var_guard36_dn9: f64 = *var_guard36_dn9_slot;
        let mut var_guard36_rdb0: f64 = *var_guard36_rdb0_slot;
        let mut var_guard36_rdb1: f64 = *var_guard36_rdb1_slot;
        let mut var_guard36_rdb10: f64 = *var_guard36_rdb10_slot;
        let mut var_guard36_rdb11: f64 = *var_guard36_rdb11_slot;
        let mut var_guard36_rdb12: f64 = *var_guard36_rdb12_slot;
        let mut var_guard36_rdb13: f64 = *var_guard36_rdb13_slot;
        let mut var_guard36_rdb14: f64 = *var_guard36_rdb14_slot;
        let mut var_guard36_rdb15: f64 = *var_guard36_rdb15_slot;
        let mut var_guard36_rdb16: f64 = *var_guard36_rdb16_slot;
        let mut var_guard36_rdb17: f64 = *var_guard36_rdb17_slot;
        let mut var_guard36_rdb18: f64 = *var_guard36_rdb18_slot;
        let mut var_guard36_rdb2: f64 = *var_guard36_rdb2_slot;
        let mut var_guard36_rdb3: f64 = *var_guard36_rdb3_slot;
        let mut var_guard36_rdb4: f64 = *var_guard36_rdb4_slot;
        let mut var_guard36_rdb5: f64 = *var_guard36_rdb5_slot;
        let mut var_guard36_rdb6: f64 = *var_guard36_rdb6_slot;
        let mut var_guard36_rdb7: f64 = *var_guard36_rdb7_slot;
        let mut var_guard36_rdb8: f64 = *var_guard36_rdb8_slot;
        let mut var_guard36_rdb9: f64 = *var_guard36_rdb9_slot;
        let mut var_guard36_rdn0: f64 = *var_guard36_rdn0_slot;
        let mut var_guard36_rdn1: f64 = *var_guard36_rdn1_slot;
        let mut var_guard36_rdn10: f64 = *var_guard36_rdn10_slot;
        let mut var_guard36_rdn11: f64 = *var_guard36_rdn11_slot;
        let mut var_guard36_rdn12: f64 = *var_guard36_rdn12_slot;
        let mut var_guard36_rdn13: f64 = *var_guard36_rdn13_slot;
        let mut var_guard36_rdn14: f64 = *var_guard36_rdn14_slot;
        let mut var_guard36_rdn15: f64 = *var_guard36_rdn15_slot;
        let mut var_guard36_rdn16: f64 = *var_guard36_rdn16_slot;
        let mut var_guard36_rdn17: f64 = *var_guard36_rdn17_slot;
        let mut var_guard36_rdn18: f64 = *var_guard36_rdn18_slot;
        let mut var_guard36_rdn2: f64 = *var_guard36_rdn2_slot;
        let mut var_guard36_rdn3: f64 = *var_guard36_rdn3_slot;
        let mut var_guard36_rdn4: f64 = *var_guard36_rdn4_slot;
        let mut var_guard36_rdn5: f64 = *var_guard36_rdn5_slot;
        let mut var_guard36_rdn6: f64 = *var_guard36_rdn6_slot;
        let mut var_guard36_rdn7: f64 = *var_guard36_rdn7_slot;
        let mut var_guard36_rdn8: f64 = *var_guard36_rdn8_slot;
        let mut var_guard36_rdn9: f64 = *var_guard36_rdn9_slot;
        let mut var_guard36_rv: f64 = *var_guard36_rv_slot;
        let mut var_noisepwr: f64 = *var_noisepwr_slot;
        let mut var_noisepwr_db0: f64 = *var_noisepwr_db0_slot;
        let mut var_noisepwr_db1: f64 = *var_noisepwr_db1_slot;
        let mut var_noisepwr_db10: f64 = *var_noisepwr_db10_slot;
        let mut var_noisepwr_db11: f64 = *var_noisepwr_db11_slot;
        let mut var_noisepwr_db12: f64 = *var_noisepwr_db12_slot;
        let mut var_noisepwr_db13: f64 = *var_noisepwr_db13_slot;
        let mut var_noisepwr_db14: f64 = *var_noisepwr_db14_slot;
        let mut var_noisepwr_db15: f64 = *var_noisepwr_db15_slot;
        let mut var_noisepwr_db16: f64 = *var_noisepwr_db16_slot;
        let mut var_noisepwr_db17: f64 = *var_noisepwr_db17_slot;
        let mut var_noisepwr_db18: f64 = *var_noisepwr_db18_slot;
        let mut var_noisepwr_db2: f64 = *var_noisepwr_db2_slot;
        let mut var_noisepwr_db3: f64 = *var_noisepwr_db3_slot;
        let mut var_noisepwr_db4: f64 = *var_noisepwr_db4_slot;
        let mut var_noisepwr_db5: f64 = *var_noisepwr_db5_slot;
        let mut var_noisepwr_db6: f64 = *var_noisepwr_db6_slot;
        let mut var_noisepwr_db7: f64 = *var_noisepwr_db7_slot;
        let mut var_noisepwr_db8: f64 = *var_noisepwr_db8_slot;
        let mut var_noisepwr_db9: f64 = *var_noisepwr_db9_slot;
        let mut var_noisepwr_dn0: f64 = *var_noisepwr_dn0_slot;
        let mut var_noisepwr_dn1: f64 = *var_noisepwr_dn1_slot;
        let mut var_noisepwr_dn10: f64 = *var_noisepwr_dn10_slot;
        let mut var_noisepwr_dn11: f64 = *var_noisepwr_dn11_slot;
        let mut var_noisepwr_dn12: f64 = *var_noisepwr_dn12_slot;
        let mut var_noisepwr_dn13: f64 = *var_noisepwr_dn13_slot;
        let mut var_noisepwr_dn14: f64 = *var_noisepwr_dn14_slot;
        let mut var_noisepwr_dn15: f64 = *var_noisepwr_dn15_slot;
        let mut var_noisepwr_dn16: f64 = *var_noisepwr_dn16_slot;
        let mut var_noisepwr_dn17: f64 = *var_noisepwr_dn17_slot;
        let mut var_noisepwr_dn18: f64 = *var_noisepwr_dn18_slot;
        let mut var_noisepwr_dn2: f64 = *var_noisepwr_dn2_slot;
        let mut var_noisepwr_dn3: f64 = *var_noisepwr_dn3_slot;
        let mut var_noisepwr_dn4: f64 = *var_noisepwr_dn4_slot;
        let mut var_noisepwr_dn5: f64 = *var_noisepwr_dn5_slot;
        let mut var_noisepwr_dn6: f64 = *var_noisepwr_dn6_slot;
        let mut var_noisepwr_dn7: f64 = *var_noisepwr_dn7_slot;
        let mut var_noisepwr_dn8: f64 = *var_noisepwr_dn8_slot;
        let mut var_noisepwr_dn9: f64 = *var_noisepwr_dn9_slot;
        let mut var_noisepwr_rdb0: f64 = *var_noisepwr_rdb0_slot;
        let mut var_noisepwr_rdb1: f64 = *var_noisepwr_rdb1_slot;
        let mut var_noisepwr_rdb10: f64 = *var_noisepwr_rdb10_slot;
        let mut var_noisepwr_rdb11: f64 = *var_noisepwr_rdb11_slot;
        let mut var_noisepwr_rdb12: f64 = *var_noisepwr_rdb12_slot;
        let mut var_noisepwr_rdb13: f64 = *var_noisepwr_rdb13_slot;
        let mut var_noisepwr_rdb14: f64 = *var_noisepwr_rdb14_slot;
        let mut var_noisepwr_rdb15: f64 = *var_noisepwr_rdb15_slot;
        let mut var_noisepwr_rdb16: f64 = *var_noisepwr_rdb16_slot;
        let mut var_noisepwr_rdb17: f64 = *var_noisepwr_rdb17_slot;
        let mut var_noisepwr_rdb18: f64 = *var_noisepwr_rdb18_slot;
        let mut var_noisepwr_rdb2: f64 = *var_noisepwr_rdb2_slot;
        let mut var_noisepwr_rdb3: f64 = *var_noisepwr_rdb3_slot;
        let mut var_noisepwr_rdb4: f64 = *var_noisepwr_rdb4_slot;
        let mut var_noisepwr_rdb5: f64 = *var_noisepwr_rdb5_slot;
        let mut var_noisepwr_rdb6: f64 = *var_noisepwr_rdb6_slot;
        let mut var_noisepwr_rdb7: f64 = *var_noisepwr_rdb7_slot;
        let mut var_noisepwr_rdb8: f64 = *var_noisepwr_rdb8_slot;
        let mut var_noisepwr_rdb9: f64 = *var_noisepwr_rdb9_slot;
        let mut var_noisepwr_rdn0: f64 = *var_noisepwr_rdn0_slot;
        let mut var_noisepwr_rdn1: f64 = *var_noisepwr_rdn1_slot;
        let mut var_noisepwr_rdn10: f64 = *var_noisepwr_rdn10_slot;
        let mut var_noisepwr_rdn11: f64 = *var_noisepwr_rdn11_slot;
        let mut var_noisepwr_rdn12: f64 = *var_noisepwr_rdn12_slot;
        let mut var_noisepwr_rdn13: f64 = *var_noisepwr_rdn13_slot;
        let mut var_noisepwr_rdn14: f64 = *var_noisepwr_rdn14_slot;
        let mut var_noisepwr_rdn15: f64 = *var_noisepwr_rdn15_slot;
        let mut var_noisepwr_rdn16: f64 = *var_noisepwr_rdn16_slot;
        let mut var_noisepwr_rdn17: f64 = *var_noisepwr_rdn17_slot;
        let mut var_noisepwr_rdn18: f64 = *var_noisepwr_rdn18_slot;
        let mut var_noisepwr_rdn2: f64 = *var_noisepwr_rdn2_slot;
        let mut var_noisepwr_rdn3: f64 = *var_noisepwr_rdn3_slot;
        let mut var_noisepwr_rdn4: f64 = *var_noisepwr_rdn4_slot;
        let mut var_noisepwr_rdn5: f64 = *var_noisepwr_rdn5_slot;
        let mut var_noisepwr_rdn6: f64 = *var_noisepwr_rdn6_slot;
        let mut var_noisepwr_rdn7: f64 = *var_noisepwr_rdn7_slot;
        let mut var_noisepwr_rdn8: f64 = *var_noisepwr_rdn8_slot;
        let mut var_noisepwr_rdn9: f64 = *var_noisepwr_rdn9_slot;
        let mut var_noisepwr_rv: f64 = *var_noisepwr_rv_slot;
        let mut var_noisepwrd: f64 = *var_noisepwrd_slot;
        let mut var_noisepwrd_db0: f64 = *var_noisepwrd_db0_slot;
        let mut var_noisepwrd_db1: f64 = *var_noisepwrd_db1_slot;
        let mut var_noisepwrd_db10: f64 = *var_noisepwrd_db10_slot;
        let mut var_noisepwrd_db11: f64 = *var_noisepwrd_db11_slot;
        let mut var_noisepwrd_db12: f64 = *var_noisepwrd_db12_slot;
        let mut var_noisepwrd_db13: f64 = *var_noisepwrd_db13_slot;
        let mut var_noisepwrd_db14: f64 = *var_noisepwrd_db14_slot;
        let mut var_noisepwrd_db15: f64 = *var_noisepwrd_db15_slot;
        let mut var_noisepwrd_db16: f64 = *var_noisepwrd_db16_slot;
        let mut var_noisepwrd_db17: f64 = *var_noisepwrd_db17_slot;
        let mut var_noisepwrd_db18: f64 = *var_noisepwrd_db18_slot;
        let mut var_noisepwrd_db2: f64 = *var_noisepwrd_db2_slot;
        let mut var_noisepwrd_db3: f64 = *var_noisepwrd_db3_slot;
        let mut var_noisepwrd_db4: f64 = *var_noisepwrd_db4_slot;
        let mut var_noisepwrd_db5: f64 = *var_noisepwrd_db5_slot;
        let mut var_noisepwrd_db6: f64 = *var_noisepwrd_db6_slot;
        let mut var_noisepwrd_db7: f64 = *var_noisepwrd_db7_slot;
        let mut var_noisepwrd_db8: f64 = *var_noisepwrd_db8_slot;
        let mut var_noisepwrd_db9: f64 = *var_noisepwrd_db9_slot;
        let mut var_noisepwrd_dn0: f64 = *var_noisepwrd_dn0_slot;
        let mut var_noisepwrd_dn1: f64 = *var_noisepwrd_dn1_slot;
        let mut var_noisepwrd_dn10: f64 = *var_noisepwrd_dn10_slot;
        let mut var_noisepwrd_dn11: f64 = *var_noisepwrd_dn11_slot;
        let mut var_noisepwrd_dn12: f64 = *var_noisepwrd_dn12_slot;
        let mut var_noisepwrd_dn13: f64 = *var_noisepwrd_dn13_slot;
        let mut var_noisepwrd_dn14: f64 = *var_noisepwrd_dn14_slot;
        let mut var_noisepwrd_dn15: f64 = *var_noisepwrd_dn15_slot;
        let mut var_noisepwrd_dn16: f64 = *var_noisepwrd_dn16_slot;
        let mut var_noisepwrd_dn17: f64 = *var_noisepwrd_dn17_slot;
        let mut var_noisepwrd_dn18: f64 = *var_noisepwrd_dn18_slot;
        let mut var_noisepwrd_dn2: f64 = *var_noisepwrd_dn2_slot;
        let mut var_noisepwrd_dn3: f64 = *var_noisepwrd_dn3_slot;
        let mut var_noisepwrd_dn4: f64 = *var_noisepwrd_dn4_slot;
        let mut var_noisepwrd_dn5: f64 = *var_noisepwrd_dn5_slot;
        let mut var_noisepwrd_dn6: f64 = *var_noisepwrd_dn6_slot;
        let mut var_noisepwrd_dn7: f64 = *var_noisepwrd_dn7_slot;
        let mut var_noisepwrd_dn8: f64 = *var_noisepwrd_dn8_slot;
        let mut var_noisepwrd_dn9: f64 = *var_noisepwrd_dn9_slot;
        let mut var_noisepwrd_rdb0: f64 = *var_noisepwrd_rdb0_slot;
        let mut var_noisepwrd_rdb1: f64 = *var_noisepwrd_rdb1_slot;
        let mut var_noisepwrd_rdb10: f64 = *var_noisepwrd_rdb10_slot;
        let mut var_noisepwrd_rdb11: f64 = *var_noisepwrd_rdb11_slot;
        let mut var_noisepwrd_rdb12: f64 = *var_noisepwrd_rdb12_slot;
        let mut var_noisepwrd_rdb13: f64 = *var_noisepwrd_rdb13_slot;
        let mut var_noisepwrd_rdb14: f64 = *var_noisepwrd_rdb14_slot;
        let mut var_noisepwrd_rdb15: f64 = *var_noisepwrd_rdb15_slot;
        let mut var_noisepwrd_rdb16: f64 = *var_noisepwrd_rdb16_slot;
        let mut var_noisepwrd_rdb17: f64 = *var_noisepwrd_rdb17_slot;
        let mut var_noisepwrd_rdb18: f64 = *var_noisepwrd_rdb18_slot;
        let mut var_noisepwrd_rdb2: f64 = *var_noisepwrd_rdb2_slot;
        let mut var_noisepwrd_rdb3: f64 = *var_noisepwrd_rdb3_slot;
        let mut var_noisepwrd_rdb4: f64 = *var_noisepwrd_rdb4_slot;
        let mut var_noisepwrd_rdb5: f64 = *var_noisepwrd_rdb5_slot;
        let mut var_noisepwrd_rdb6: f64 = *var_noisepwrd_rdb6_slot;
        let mut var_noisepwrd_rdb7: f64 = *var_noisepwrd_rdb7_slot;
        let mut var_noisepwrd_rdb8: f64 = *var_noisepwrd_rdb8_slot;
        let mut var_noisepwrd_rdb9: f64 = *var_noisepwrd_rdb9_slot;
        let mut var_noisepwrd_rdn0: f64 = *var_noisepwrd_rdn0_slot;
        let mut var_noisepwrd_rdn1: f64 = *var_noisepwrd_rdn1_slot;
        let mut var_noisepwrd_rdn10: f64 = *var_noisepwrd_rdn10_slot;
        let mut var_noisepwrd_rdn11: f64 = *var_noisepwrd_rdn11_slot;
        let mut var_noisepwrd_rdn12: f64 = *var_noisepwrd_rdn12_slot;
        let mut var_noisepwrd_rdn13: f64 = *var_noisepwrd_rdn13_slot;
        let mut var_noisepwrd_rdn14: f64 = *var_noisepwrd_rdn14_slot;
        let mut var_noisepwrd_rdn15: f64 = *var_noisepwrd_rdn15_slot;
        let mut var_noisepwrd_rdn16: f64 = *var_noisepwrd_rdn16_slot;
        let mut var_noisepwrd_rdn17: f64 = *var_noisepwrd_rdn17_slot;
        let mut var_noisepwrd_rdn18: f64 = *var_noisepwrd_rdn18_slot;
        let mut var_noisepwrd_rdn2: f64 = *var_noisepwrd_rdn2_slot;
        let mut var_noisepwrd_rdn3: f64 = *var_noisepwrd_rdn3_slot;
        let mut var_noisepwrd_rdn4: f64 = *var_noisepwrd_rdn4_slot;
        let mut var_noisepwrd_rdn5: f64 = *var_noisepwrd_rdn5_slot;
        let mut var_noisepwrd_rdn6: f64 = *var_noisepwrd_rdn6_slot;
        let mut var_noisepwrd_rdn7: f64 = *var_noisepwrd_rdn7_slot;
        let mut var_noisepwrd_rdn8: f64 = *var_noisepwrd_rdn8_slot;
        let mut var_noisepwrd_rdn9: f64 = *var_noisepwrd_rdn9_slot;
        let mut var_noisepwrd_rv: f64 = *var_noisepwrd_rv_slot;
        let mut var_noisepwrg: f64 = *var_noisepwrg_slot;
        let mut var_noisepwrg_db0: f64 = *var_noisepwrg_db0_slot;
        let mut var_noisepwrg_db1: f64 = *var_noisepwrg_db1_slot;
        let mut var_noisepwrg_db10: f64 = *var_noisepwrg_db10_slot;
        let mut var_noisepwrg_db11: f64 = *var_noisepwrg_db11_slot;
        let mut var_noisepwrg_db12: f64 = *var_noisepwrg_db12_slot;
        let mut var_noisepwrg_db13: f64 = *var_noisepwrg_db13_slot;
        let mut var_noisepwrg_db14: f64 = *var_noisepwrg_db14_slot;
        let mut var_noisepwrg_db15: f64 = *var_noisepwrg_db15_slot;
        let mut var_noisepwrg_db16: f64 = *var_noisepwrg_db16_slot;
        let mut var_noisepwrg_db17: f64 = *var_noisepwrg_db17_slot;
        let mut var_noisepwrg_db18: f64 = *var_noisepwrg_db18_slot;
        let mut var_noisepwrg_db2: f64 = *var_noisepwrg_db2_slot;
        let mut var_noisepwrg_db3: f64 = *var_noisepwrg_db3_slot;
        let mut var_noisepwrg_db4: f64 = *var_noisepwrg_db4_slot;
        let mut var_noisepwrg_db5: f64 = *var_noisepwrg_db5_slot;
        let mut var_noisepwrg_db6: f64 = *var_noisepwrg_db6_slot;
        let mut var_noisepwrg_db7: f64 = *var_noisepwrg_db7_slot;
        let mut var_noisepwrg_db8: f64 = *var_noisepwrg_db8_slot;
        let mut var_noisepwrg_db9: f64 = *var_noisepwrg_db9_slot;
        let mut var_noisepwrg_dn0: f64 = *var_noisepwrg_dn0_slot;
        let mut var_noisepwrg_dn1: f64 = *var_noisepwrg_dn1_slot;
        let mut var_noisepwrg_dn10: f64 = *var_noisepwrg_dn10_slot;
        let mut var_noisepwrg_dn11: f64 = *var_noisepwrg_dn11_slot;
        let mut var_noisepwrg_dn12: f64 = *var_noisepwrg_dn12_slot;
        let mut var_noisepwrg_dn13: f64 = *var_noisepwrg_dn13_slot;
        let mut var_noisepwrg_dn14: f64 = *var_noisepwrg_dn14_slot;
        let mut var_noisepwrg_dn15: f64 = *var_noisepwrg_dn15_slot;
        let mut var_noisepwrg_dn16: f64 = *var_noisepwrg_dn16_slot;
        let mut var_noisepwrg_dn17: f64 = *var_noisepwrg_dn17_slot;
        let mut var_noisepwrg_dn18: f64 = *var_noisepwrg_dn18_slot;
        let mut var_noisepwrg_dn2: f64 = *var_noisepwrg_dn2_slot;
        let mut var_noisepwrg_dn3: f64 = *var_noisepwrg_dn3_slot;
        let mut var_noisepwrg_dn4: f64 = *var_noisepwrg_dn4_slot;
        let mut var_noisepwrg_dn5: f64 = *var_noisepwrg_dn5_slot;
        let mut var_noisepwrg_dn6: f64 = *var_noisepwrg_dn6_slot;
        let mut var_noisepwrg_dn7: f64 = *var_noisepwrg_dn7_slot;
        let mut var_noisepwrg_dn8: f64 = *var_noisepwrg_dn8_slot;
        let mut var_noisepwrg_dn9: f64 = *var_noisepwrg_dn9_slot;
        let mut var_noisepwrg_rdb0: f64 = *var_noisepwrg_rdb0_slot;
        let mut var_noisepwrg_rdb1: f64 = *var_noisepwrg_rdb1_slot;
        let mut var_noisepwrg_rdb10: f64 = *var_noisepwrg_rdb10_slot;
        let mut var_noisepwrg_rdb11: f64 = *var_noisepwrg_rdb11_slot;
        let mut var_noisepwrg_rdb12: f64 = *var_noisepwrg_rdb12_slot;
        let mut var_noisepwrg_rdb13: f64 = *var_noisepwrg_rdb13_slot;
        let mut var_noisepwrg_rdb14: f64 = *var_noisepwrg_rdb14_slot;
        let mut var_noisepwrg_rdb15: f64 = *var_noisepwrg_rdb15_slot;
        let mut var_noisepwrg_rdb16: f64 = *var_noisepwrg_rdb16_slot;
        let mut var_noisepwrg_rdb17: f64 = *var_noisepwrg_rdb17_slot;
        let mut var_noisepwrg_rdb18: f64 = *var_noisepwrg_rdb18_slot;
        let mut var_noisepwrg_rdb2: f64 = *var_noisepwrg_rdb2_slot;
        let mut var_noisepwrg_rdb3: f64 = *var_noisepwrg_rdb3_slot;
        let mut var_noisepwrg_rdb4: f64 = *var_noisepwrg_rdb4_slot;
        let mut var_noisepwrg_rdb5: f64 = *var_noisepwrg_rdb5_slot;
        let mut var_noisepwrg_rdb6: f64 = *var_noisepwrg_rdb6_slot;
        let mut var_noisepwrg_rdb7: f64 = *var_noisepwrg_rdb7_slot;
        let mut var_noisepwrg_rdb8: f64 = *var_noisepwrg_rdb8_slot;
        let mut var_noisepwrg_rdb9: f64 = *var_noisepwrg_rdb9_slot;
        let mut var_noisepwrg_rdn0: f64 = *var_noisepwrg_rdn0_slot;
        let mut var_noisepwrg_rdn1: f64 = *var_noisepwrg_rdn1_slot;
        let mut var_noisepwrg_rdn10: f64 = *var_noisepwrg_rdn10_slot;
        let mut var_noisepwrg_rdn11: f64 = *var_noisepwrg_rdn11_slot;
        let mut var_noisepwrg_rdn12: f64 = *var_noisepwrg_rdn12_slot;
        let mut var_noisepwrg_rdn13: f64 = *var_noisepwrg_rdn13_slot;
        let mut var_noisepwrg_rdn14: f64 = *var_noisepwrg_rdn14_slot;
        let mut var_noisepwrg_rdn15: f64 = *var_noisepwrg_rdn15_slot;
        let mut var_noisepwrg_rdn16: f64 = *var_noisepwrg_rdn16_slot;
        let mut var_noisepwrg_rdn17: f64 = *var_noisepwrg_rdn17_slot;
        let mut var_noisepwrg_rdn18: f64 = *var_noisepwrg_rdn18_slot;
        let mut var_noisepwrg_rdn2: f64 = *var_noisepwrg_rdn2_slot;
        let mut var_noisepwrg_rdn3: f64 = *var_noisepwrg_rdn3_slot;
        let mut var_noisepwrg_rdn4: f64 = *var_noisepwrg_rdn4_slot;
        let mut var_noisepwrg_rdn5: f64 = *var_noisepwrg_rdn5_slot;
        let mut var_noisepwrg_rdn6: f64 = *var_noisepwrg_rdn6_slot;
        let mut var_noisepwrg_rdn7: f64 = *var_noisepwrg_rdn7_slot;
        let mut var_noisepwrg_rdn8: f64 = *var_noisepwrg_rdn8_slot;
        let mut var_noisepwrg_rdn9: f64 = *var_noisepwrg_rdn9_slot;
        let mut var_noisepwrg_rv: f64 = *var_noisepwrg_rv_slot;

        let (assign2260_e2941, assign2260_e2941_d_n0, assign2260_e2941_d_n1, assign2260_e2941_d_n2, assign2260_e2941_d_n3, assign2260_e2941_d_n4, assign2260_e2941_d_n5, assign2260_e2941_d_n6, assign2260_e2941_d_n7, assign2260_e2941_d_n8, assign2260_e2941_d_n9, assign2260_e2941_d_n10, assign2260_e2941_d_n11, assign2260_e2941_d_n12, assign2260_e2941_d_n13, assign2260_e2941_d_n14, assign2260_e2941_d_n15, assign2260_e2941_d_n16, assign2260_e2941_d_n17, assign2260_e2941_d_n18, assign2260_e2941_d_b0, assign2260_e2941_d_b1, assign2260_e2941_d_b2, assign2260_e2941_d_b3, assign2260_e2941_d_b4, assign2260_e2941_d_b5, assign2260_e2941_d_b6, assign2260_e2941_d_b7, assign2260_e2941_d_b8, assign2260_e2941_d_b9, assign2260_e2941_d_b10, assign2260_e2941_d_b11, assign2260_e2941_d_b12, assign2260_e2941_d_b13, assign2260_e2941_d_b14, assign2260_e2941_d_b15, assign2260_e2941_d_b16, assign2260_e2941_d_b17, assign2260_e2941_d_b18,) = {
    if (var_guard28 != 0.0) {
        let assign2260_e2921: f64 = (p.p99 * 4.0);
        let assign2260_e2923: f64 = (assign2260_e2921 * 1.3806503e-23);
        let assign2260_e2925: f64 = (assign2260_e2923 * var_t);
        let assign2260_e2928: f64 = (var_td_prime / var_t);
        let assign2260_e2930: f64 = (assign2260_e2928 * var_idtn);
        let assign2260_e2933: f64 = (p.p94 * var_idtn);
        let assign2260_e2935: f64 = (assign2260_e2933 * var_idtn);
        let assign2260_e2936: f64 = (assign2260_e2930 + assign2260_e2935);
        let assign2260_e2937: f64 = (assign2260_e2936).abs();
        let assign2260_e2938: f64 = (assign2260_e2937).sqrt();
        let assign2260_e2939: f64 = (assign2260_e2925 * assign2260_e2938);
        (assign2260_e2939, (((assign2260_e2923 * var_t_dn0) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn0 * var_t) - (var_td_prime * var_t_dn0)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn0)) + (((p.p94 * var_idtn_dn0) * var_idtn) + (assign2260_e2933 * var_idtn_dn0))) } else { (-((((((var_td_prime_dn0 * var_t) - (var_td_prime * var_t_dn0)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn0)) + (((p.p94 * var_idtn_dn0) * var_idtn) + (assign2260_e2933 * var_idtn_dn0)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn1) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn1 * var_t) - (var_td_prime * var_t_dn1)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn1)) + (((p.p94 * var_idtn_dn1) * var_idtn) + (assign2260_e2933 * var_idtn_dn1))) } else { (-((((((var_td_prime_dn1 * var_t) - (var_td_prime * var_t_dn1)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn1)) + (((p.p94 * var_idtn_dn1) * var_idtn) + (assign2260_e2933 * var_idtn_dn1)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn2) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn2 * var_t) - (var_td_prime * var_t_dn2)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn2)) + (((p.p94 * var_idtn_dn2) * var_idtn) + (assign2260_e2933 * var_idtn_dn2))) } else { (-((((((var_td_prime_dn2 * var_t) - (var_td_prime * var_t_dn2)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn2)) + (((p.p94 * var_idtn_dn2) * var_idtn) + (assign2260_e2933 * var_idtn_dn2)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn3) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn3 * var_t) - (var_td_prime * var_t_dn3)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn3)) + (((p.p94 * var_idtn_dn3) * var_idtn) + (assign2260_e2933 * var_idtn_dn3))) } else { (-((((((var_td_prime_dn3 * var_t) - (var_td_prime * var_t_dn3)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn3)) + (((p.p94 * var_idtn_dn3) * var_idtn) + (assign2260_e2933 * var_idtn_dn3)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn4) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn4 * var_t) - (var_td_prime * var_t_dn4)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn4)) + (((p.p94 * var_idtn_dn4) * var_idtn) + (assign2260_e2933 * var_idtn_dn4))) } else { (-((((((var_td_prime_dn4 * var_t) - (var_td_prime * var_t_dn4)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn4)) + (((p.p94 * var_idtn_dn4) * var_idtn) + (assign2260_e2933 * var_idtn_dn4)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn5) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn5 * var_t) - (var_td_prime * var_t_dn5)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn5)) + (((p.p94 * var_idtn_dn5) * var_idtn) + (assign2260_e2933 * var_idtn_dn5))) } else { (-((((((var_td_prime_dn5 * var_t) - (var_td_prime * var_t_dn5)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn5)) + (((p.p94 * var_idtn_dn5) * var_idtn) + (assign2260_e2933 * var_idtn_dn5)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn6) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn6 * var_t) - (var_td_prime * var_t_dn6)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn6)) + (((p.p94 * var_idtn_dn6) * var_idtn) + (assign2260_e2933 * var_idtn_dn6))) } else { (-((((((var_td_prime_dn6 * var_t) - (var_td_prime * var_t_dn6)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn6)) + (((p.p94 * var_idtn_dn6) * var_idtn) + (assign2260_e2933 * var_idtn_dn6)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn7) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn7 * var_t) - (var_td_prime * var_t_dn7)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn7)) + (((p.p94 * var_idtn_dn7) * var_idtn) + (assign2260_e2933 * var_idtn_dn7))) } else { (-((((((var_td_prime_dn7 * var_t) - (var_td_prime * var_t_dn7)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn7)) + (((p.p94 * var_idtn_dn7) * var_idtn) + (assign2260_e2933 * var_idtn_dn7)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn8) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn8 * var_t) - (var_td_prime * var_t_dn8)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn8)) + (((p.p94 * var_idtn_dn8) * var_idtn) + (assign2260_e2933 * var_idtn_dn8))) } else { (-((((((var_td_prime_dn8 * var_t) - (var_td_prime * var_t_dn8)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn8)) + (((p.p94 * var_idtn_dn8) * var_idtn) + (assign2260_e2933 * var_idtn_dn8)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn9) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn9 * var_t) - (var_td_prime * var_t_dn9)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn9)) + (((p.p94 * var_idtn_dn9) * var_idtn) + (assign2260_e2933 * var_idtn_dn9))) } else { (-((((((var_td_prime_dn9 * var_t) - (var_td_prime * var_t_dn9)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn9)) + (((p.p94 * var_idtn_dn9) * var_idtn) + (assign2260_e2933 * var_idtn_dn9)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn10) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn10 * var_t) - (var_td_prime * var_t_dn10)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn10)) + (((p.p94 * var_idtn_dn10) * var_idtn) + (assign2260_e2933 * var_idtn_dn10))) } else { (-((((((var_td_prime_dn10 * var_t) - (var_td_prime * var_t_dn10)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn10)) + (((p.p94 * var_idtn_dn10) * var_idtn) + (assign2260_e2933 * var_idtn_dn10)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn11) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn11 * var_t) - (var_td_prime * var_t_dn11)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn11)) + (((p.p94 * var_idtn_dn11) * var_idtn) + (assign2260_e2933 * var_idtn_dn11))) } else { (-((((((var_td_prime_dn11 * var_t) - (var_td_prime * var_t_dn11)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn11)) + (((p.p94 * var_idtn_dn11) * var_idtn) + (assign2260_e2933 * var_idtn_dn11)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn12) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn12 * var_t) - (var_td_prime * var_t_dn12)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn12)) + (((p.p94 * var_idtn_dn12) * var_idtn) + (assign2260_e2933 * var_idtn_dn12))) } else { (-((((((var_td_prime_dn12 * var_t) - (var_td_prime * var_t_dn12)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn12)) + (((p.p94 * var_idtn_dn12) * var_idtn) + (assign2260_e2933 * var_idtn_dn12)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn13) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn13 * var_t) - (var_td_prime * var_t_dn13)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn13)) + (((p.p94 * var_idtn_dn13) * var_idtn) + (assign2260_e2933 * var_idtn_dn13))) } else { (-((((((var_td_prime_dn13 * var_t) - (var_td_prime * var_t_dn13)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn13)) + (((p.p94 * var_idtn_dn13) * var_idtn) + (assign2260_e2933 * var_idtn_dn13)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn14) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn14 * var_t) - (var_td_prime * var_t_dn14)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn14)) + (((p.p94 * var_idtn_dn14) * var_idtn) + (assign2260_e2933 * var_idtn_dn14))) } else { (-((((((var_td_prime_dn14 * var_t) - (var_td_prime * var_t_dn14)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn14)) + (((p.p94 * var_idtn_dn14) * var_idtn) + (assign2260_e2933 * var_idtn_dn14)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn15) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn15 * var_t) - (var_td_prime * var_t_dn15)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn15)) + (((p.p94 * var_idtn_dn15) * var_idtn) + (assign2260_e2933 * var_idtn_dn15))) } else { (-((((((var_td_prime_dn15 * var_t) - (var_td_prime * var_t_dn15)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn15)) + (((p.p94 * var_idtn_dn15) * var_idtn) + (assign2260_e2933 * var_idtn_dn15)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn16) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn16 * var_t) - (var_td_prime * var_t_dn16)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn16)) + (((p.p94 * var_idtn_dn16) * var_idtn) + (assign2260_e2933 * var_idtn_dn16))) } else { (-((((((var_td_prime_dn16 * var_t) - (var_td_prime * var_t_dn16)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn16)) + (((p.p94 * var_idtn_dn16) * var_idtn) + (assign2260_e2933 * var_idtn_dn16)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn17) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn17 * var_t) - (var_td_prime * var_t_dn17)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn17)) + (((p.p94 * var_idtn_dn17) * var_idtn) + (assign2260_e2933 * var_idtn_dn17))) } else { (-((((((var_td_prime_dn17 * var_t) - (var_td_prime * var_t_dn17)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn17)) + (((p.p94 * var_idtn_dn17) * var_idtn) + (assign2260_e2933 * var_idtn_dn17)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_dn18) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_dn18 * var_t) - (var_td_prime * var_t_dn18)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn18)) + (((p.p94 * var_idtn_dn18) * var_idtn) + (assign2260_e2933 * var_idtn_dn18))) } else { (-((((((var_td_prime_dn18 * var_t) - (var_td_prime * var_t_dn18)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_dn18)) + (((p.p94 * var_idtn_dn18) * var_idtn) + (assign2260_e2933 * var_idtn_dn18)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db0) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db0 * var_t) - (var_td_prime * var_t_db0)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db0)) + (((p.p94 * var_idtn_db0) * var_idtn) + (assign2260_e2933 * var_idtn_db0))) } else { (-((((((var_td_prime_db0 * var_t) - (var_td_prime * var_t_db0)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db0)) + (((p.p94 * var_idtn_db0) * var_idtn) + (assign2260_e2933 * var_idtn_db0)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db1) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db1 * var_t) - (var_td_prime * var_t_db1)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db1)) + (((p.p94 * var_idtn_db1) * var_idtn) + (assign2260_e2933 * var_idtn_db1))) } else { (-((((((var_td_prime_db1 * var_t) - (var_td_prime * var_t_db1)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db1)) + (((p.p94 * var_idtn_db1) * var_idtn) + (assign2260_e2933 * var_idtn_db1)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db2) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db2 * var_t) - (var_td_prime * var_t_db2)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db2)) + (((p.p94 * var_idtn_db2) * var_idtn) + (assign2260_e2933 * var_idtn_db2))) } else { (-((((((var_td_prime_db2 * var_t) - (var_td_prime * var_t_db2)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db2)) + (((p.p94 * var_idtn_db2) * var_idtn) + (assign2260_e2933 * var_idtn_db2)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db3) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db3 * var_t) - (var_td_prime * var_t_db3)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db3)) + (((p.p94 * var_idtn_db3) * var_idtn) + (assign2260_e2933 * var_idtn_db3))) } else { (-((((((var_td_prime_db3 * var_t) - (var_td_prime * var_t_db3)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db3)) + (((p.p94 * var_idtn_db3) * var_idtn) + (assign2260_e2933 * var_idtn_db3)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db4) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db4 * var_t) - (var_td_prime * var_t_db4)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db4)) + (((p.p94 * var_idtn_db4) * var_idtn) + (assign2260_e2933 * var_idtn_db4))) } else { (-((((((var_td_prime_db4 * var_t) - (var_td_prime * var_t_db4)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db4)) + (((p.p94 * var_idtn_db4) * var_idtn) + (assign2260_e2933 * var_idtn_db4)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db5) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db5 * var_t) - (var_td_prime * var_t_db5)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db5)) + (((p.p94 * var_idtn_db5) * var_idtn) + (assign2260_e2933 * var_idtn_db5))) } else { (-((((((var_td_prime_db5 * var_t) - (var_td_prime * var_t_db5)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db5)) + (((p.p94 * var_idtn_db5) * var_idtn) + (assign2260_e2933 * var_idtn_db5)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db6) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db6 * var_t) - (var_td_prime * var_t_db6)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db6)) + (((p.p94 * var_idtn_db6) * var_idtn) + (assign2260_e2933 * var_idtn_db6))) } else { (-((((((var_td_prime_db6 * var_t) - (var_td_prime * var_t_db6)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db6)) + (((p.p94 * var_idtn_db6) * var_idtn) + (assign2260_e2933 * var_idtn_db6)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db7) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db7 * var_t) - (var_td_prime * var_t_db7)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db7)) + (((p.p94 * var_idtn_db7) * var_idtn) + (assign2260_e2933 * var_idtn_db7))) } else { (-((((((var_td_prime_db7 * var_t) - (var_td_prime * var_t_db7)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db7)) + (((p.p94 * var_idtn_db7) * var_idtn) + (assign2260_e2933 * var_idtn_db7)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db8) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db8 * var_t) - (var_td_prime * var_t_db8)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db8)) + (((p.p94 * var_idtn_db8) * var_idtn) + (assign2260_e2933 * var_idtn_db8))) } else { (-((((((var_td_prime_db8 * var_t) - (var_td_prime * var_t_db8)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db8)) + (((p.p94 * var_idtn_db8) * var_idtn) + (assign2260_e2933 * var_idtn_db8)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db9) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db9 * var_t) - (var_td_prime * var_t_db9)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db9)) + (((p.p94 * var_idtn_db9) * var_idtn) + (assign2260_e2933 * var_idtn_db9))) } else { (-((((((var_td_prime_db9 * var_t) - (var_td_prime * var_t_db9)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db9)) + (((p.p94 * var_idtn_db9) * var_idtn) + (assign2260_e2933 * var_idtn_db9)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db10) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db10 * var_t) - (var_td_prime * var_t_db10)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db10)) + (((p.p94 * var_idtn_db10) * var_idtn) + (assign2260_e2933 * var_idtn_db10))) } else { (-((((((var_td_prime_db10 * var_t) - (var_td_prime * var_t_db10)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db10)) + (((p.p94 * var_idtn_db10) * var_idtn) + (assign2260_e2933 * var_idtn_db10)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db11) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db11 * var_t) - (var_td_prime * var_t_db11)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db11)) + (((p.p94 * var_idtn_db11) * var_idtn) + (assign2260_e2933 * var_idtn_db11))) } else { (-((((((var_td_prime_db11 * var_t) - (var_td_prime * var_t_db11)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db11)) + (((p.p94 * var_idtn_db11) * var_idtn) + (assign2260_e2933 * var_idtn_db11)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db12) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db12 * var_t) - (var_td_prime * var_t_db12)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db12)) + (((p.p94 * var_idtn_db12) * var_idtn) + (assign2260_e2933 * var_idtn_db12))) } else { (-((((((var_td_prime_db12 * var_t) - (var_td_prime * var_t_db12)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db12)) + (((p.p94 * var_idtn_db12) * var_idtn) + (assign2260_e2933 * var_idtn_db12)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db13) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db13 * var_t) - (var_td_prime * var_t_db13)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db13)) + (((p.p94 * var_idtn_db13) * var_idtn) + (assign2260_e2933 * var_idtn_db13))) } else { (-((((((var_td_prime_db13 * var_t) - (var_td_prime * var_t_db13)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db13)) + (((p.p94 * var_idtn_db13) * var_idtn) + (assign2260_e2933 * var_idtn_db13)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db14) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db14 * var_t) - (var_td_prime * var_t_db14)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db14)) + (((p.p94 * var_idtn_db14) * var_idtn) + (assign2260_e2933 * var_idtn_db14))) } else { (-((((((var_td_prime_db14 * var_t) - (var_td_prime * var_t_db14)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db14)) + (((p.p94 * var_idtn_db14) * var_idtn) + (assign2260_e2933 * var_idtn_db14)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db15) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db15 * var_t) - (var_td_prime * var_t_db15)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db15)) + (((p.p94 * var_idtn_db15) * var_idtn) + (assign2260_e2933 * var_idtn_db15))) } else { (-((((((var_td_prime_db15 * var_t) - (var_td_prime * var_t_db15)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db15)) + (((p.p94 * var_idtn_db15) * var_idtn) + (assign2260_e2933 * var_idtn_db15)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db16) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db16 * var_t) - (var_td_prime * var_t_db16)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db16)) + (((p.p94 * var_idtn_db16) * var_idtn) + (assign2260_e2933 * var_idtn_db16))) } else { (-((((((var_td_prime_db16 * var_t) - (var_td_prime * var_t_db16)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db16)) + (((p.p94 * var_idtn_db16) * var_idtn) + (assign2260_e2933 * var_idtn_db16)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db17) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db17 * var_t) - (var_td_prime * var_t_db17)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db17)) + (((p.p94 * var_idtn_db17) * var_idtn) + (assign2260_e2933 * var_idtn_db17))) } else { (-((((((var_td_prime_db17 * var_t) - (var_td_prime * var_t_db17)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db17)) + (((p.p94 * var_idtn_db17) * var_idtn) + (assign2260_e2933 * var_idtn_db17)))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * var_t_db18) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((var_td_prime_db18 * var_t) - (var_td_prime * var_t_db18)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db18)) + (((p.p94 * var_idtn_db18) * var_idtn) + (assign2260_e2933 * var_idtn_db18))) } else { (-((((((var_td_prime_db18 * var_t) - (var_td_prime * var_t_db18)) / (var_t * var_t)) * var_idtn) + (assign2260_e2928 * var_idtn_db18)) + (((p.p94 * var_idtn_db18) * var_idtn) + (assign2260_e2933 * var_idtn_db18)))) } / (2.0 * assign2260_e2938)))),)
    } else {
        (var_noisepwr, var_noisepwr_dn0, var_noisepwr_dn1, var_noisepwr_dn2, var_noisepwr_dn3, var_noisepwr_dn4, var_noisepwr_dn5, var_noisepwr_dn6, var_noisepwr_dn7, var_noisepwr_dn8, var_noisepwr_dn9, var_noisepwr_dn10, var_noisepwr_dn11, var_noisepwr_dn12, var_noisepwr_dn13, var_noisepwr_dn14, var_noisepwr_dn15, var_noisepwr_dn16, var_noisepwr_dn17, var_noisepwr_dn18, var_noisepwr_db0, var_noisepwr_db1, var_noisepwr_db2, var_noisepwr_db3, var_noisepwr_db4, var_noisepwr_db5, var_noisepwr_db6, var_noisepwr_db7, var_noisepwr_db8, var_noisepwr_db9, var_noisepwr_db10, var_noisepwr_db11, var_noisepwr_db12, var_noisepwr_db13, var_noisepwr_db14, var_noisepwr_db15, var_noisepwr_db16, var_noisepwr_db17, var_noisepwr_db18,)
    }
};
        var_noisepwr = assign2260_e2941;
        var_noisepwr_dn0 = assign2260_e2941_d_n0;
        var_noisepwr_dn1 = assign2260_e2941_d_n1;
        var_noisepwr_dn2 = assign2260_e2941_d_n2;
        var_noisepwr_dn3 = assign2260_e2941_d_n3;
        var_noisepwr_dn4 = assign2260_e2941_d_n4;
        var_noisepwr_dn5 = assign2260_e2941_d_n5;
        var_noisepwr_dn6 = assign2260_e2941_d_n6;
        var_noisepwr_dn7 = assign2260_e2941_d_n7;
        var_noisepwr_dn8 = assign2260_e2941_d_n8;
        var_noisepwr_dn9 = assign2260_e2941_d_n9;
        var_noisepwr_dn10 = assign2260_e2941_d_n10;
        var_noisepwr_dn11 = assign2260_e2941_d_n11;
        var_noisepwr_dn12 = assign2260_e2941_d_n12;
        var_noisepwr_dn13 = assign2260_e2941_d_n13;
        var_noisepwr_dn14 = assign2260_e2941_d_n14;
        var_noisepwr_dn15 = assign2260_e2941_d_n15;
        var_noisepwr_dn16 = assign2260_e2941_d_n16;
        var_noisepwr_dn17 = assign2260_e2941_d_n17;
        var_noisepwr_dn18 = assign2260_e2941_d_n18;
        var_noisepwr_db0 = assign2260_e2941_d_b0;
        var_noisepwr_db1 = assign2260_e2941_d_b1;
        var_noisepwr_db2 = assign2260_e2941_d_b2;
        var_noisepwr_db3 = assign2260_e2941_d_b3;
        var_noisepwr_db4 = assign2260_e2941_d_b4;
        var_noisepwr_db5 = assign2260_e2941_d_b5;
        var_noisepwr_db6 = assign2260_e2941_d_b6;
        var_noisepwr_db7 = assign2260_e2941_d_b7;
        var_noisepwr_db8 = assign2260_e2941_d_b8;
        var_noisepwr_db9 = assign2260_e2941_d_b9;
        var_noisepwr_db10 = assign2260_e2941_d_b10;
        var_noisepwr_db11 = assign2260_e2941_d_b11;
        var_noisepwr_db12 = assign2260_e2941_d_b12;
        var_noisepwr_db13 = assign2260_e2941_d_b13;
        var_noisepwr_db14 = assign2260_e2941_d_b14;
        var_noisepwr_db15 = assign2260_e2941_d_b15;
        var_noisepwr_db16 = assign2260_e2941_d_b16;
        var_noisepwr_db17 = assign2260_e2941_d_b17;
        var_noisepwr_db18 = assign2260_e2941_d_b18;
        var_noisepwr_rv = 0.0;
        var_noisepwr_rdn0 = 0.0;
        var_noisepwr_rdn1 = 0.0;
        var_noisepwr_rdn2 = 0.0;
        var_noisepwr_rdn3 = 0.0;
        var_noisepwr_rdn4 = 0.0;
        var_noisepwr_rdn5 = 0.0;
        var_noisepwr_rdn6 = 0.0;
        var_noisepwr_rdn7 = 0.0;
        var_noisepwr_rdn8 = 0.0;
        var_noisepwr_rdn9 = 0.0;
        var_noisepwr_rdn10 = 0.0;
        var_noisepwr_rdn11 = 0.0;
        var_noisepwr_rdn12 = 0.0;
        var_noisepwr_rdn13 = 0.0;
        var_noisepwr_rdn14 = 0.0;
        var_noisepwr_rdn15 = 0.0;
        var_noisepwr_rdn16 = 0.0;
        var_noisepwr_rdn17 = 0.0;
        var_noisepwr_rdn18 = 0.0;
        var_noisepwr_rdb0 = 0.0;
        var_noisepwr_rdb1 = 0.0;
        var_noisepwr_rdb2 = 0.0;
        var_noisepwr_rdb3 = 0.0;
        var_noisepwr_rdb4 = 0.0;
        var_noisepwr_rdb5 = 0.0;
        var_noisepwr_rdb6 = 0.0;
        var_noisepwr_rdb7 = 0.0;
        var_noisepwr_rdb8 = 0.0;
        var_noisepwr_rdb9 = 0.0;
        var_noisepwr_rdb10 = 0.0;
        var_noisepwr_rdb11 = 0.0;
        var_noisepwr_rdb12 = 0.0;
        var_noisepwr_rdb13 = 0.0;
        var_noisepwr_rdb14 = 0.0;
        var_noisepwr_rdb15 = 0.0;
        var_noisepwr_rdb16 = 0.0;
        var_noisepwr_rdb17 = 0.0;
        var_noisepwr_rdb18 = 0.0;

        let (assign2270_e2958, assign2270_e2958_d_n0, assign2270_e2958_d_n1, assign2270_e2958_d_n2, assign2270_e2958_d_n3, assign2270_e2958_d_n4, assign2270_e2958_d_n5, assign2270_e2958_d_n6, assign2270_e2958_d_n7, assign2270_e2958_d_n8, assign2270_e2958_d_n9, assign2270_e2958_d_n10, assign2270_e2958_d_n11, assign2270_e2958_d_n12, assign2270_e2958_d_n13, assign2270_e2958_d_n14, assign2270_e2958_d_n15, assign2270_e2958_d_n16, assign2270_e2958_d_n17, assign2270_e2958_d_n18, assign2270_e2958_d_b0, assign2270_e2958_d_b1, assign2270_e2958_d_b2, assign2270_e2958_d_b3, assign2270_e2958_d_b4, assign2270_e2958_d_b5, assign2270_e2958_d_b6, assign2270_e2958_d_b7, assign2270_e2958_d_b8, assign2270_e2958_d_b9, assign2270_e2958_d_b10, assign2270_e2958_d_b11, assign2270_e2958_d_b12, assign2270_e2958_d_b13, assign2270_e2958_d_b14, assign2270_e2958_d_b15, assign2270_e2958_d_b16, assign2270_e2958_d_b17, assign2270_e2958_d_b18,) = {
    if (((var_guard29 != 0.0) && (var_guard28 == 0.0)) && (p.p0 != 0.0)) {
        let assign2270_e2950: f64 = (4.0 * 1.3806503e-23);
        let assign2270_e2952: f64 = (assign2270_e2950 * var_t);
        let assign2270_e2954: f64 = (assign2270_e2952 * var_gm);
        let assign2270_e2956: f64 = (assign2270_e2954 * p.p87);
        (assign2270_e2956, ((((assign2270_e2950 * var_t_dn0) * var_gm) + (assign2270_e2952 * var_gm_dn0)) * p.p87), ((((assign2270_e2950 * var_t_dn1) * var_gm) + (assign2270_e2952 * var_gm_dn1)) * p.p87), ((((assign2270_e2950 * var_t_dn2) * var_gm) + (assign2270_e2952 * var_gm_dn2)) * p.p87), ((((assign2270_e2950 * var_t_dn3) * var_gm) + (assign2270_e2952 * var_gm_dn3)) * p.p87), ((((assign2270_e2950 * var_t_dn4) * var_gm) + (assign2270_e2952 * var_gm_dn4)) * p.p87), ((((assign2270_e2950 * var_t_dn5) * var_gm) + (assign2270_e2952 * var_gm_dn5)) * p.p87), ((((assign2270_e2950 * var_t_dn6) * var_gm) + (assign2270_e2952 * var_gm_dn6)) * p.p87), ((((assign2270_e2950 * var_t_dn7) * var_gm) + (assign2270_e2952 * var_gm_dn7)) * p.p87), ((((assign2270_e2950 * var_t_dn8) * var_gm) + (assign2270_e2952 * var_gm_dn8)) * p.p87), ((((assign2270_e2950 * var_t_dn9) * var_gm) + (assign2270_e2952 * var_gm_dn9)) * p.p87), ((((assign2270_e2950 * var_t_dn10) * var_gm) + (assign2270_e2952 * var_gm_dn10)) * p.p87), ((((assign2270_e2950 * var_t_dn11) * var_gm) + (assign2270_e2952 * var_gm_dn11)) * p.p87), ((((assign2270_e2950 * var_t_dn12) * var_gm) + (assign2270_e2952 * var_gm_dn12)) * p.p87), ((((assign2270_e2950 * var_t_dn13) * var_gm) + (assign2270_e2952 * var_gm_dn13)) * p.p87), ((((assign2270_e2950 * var_t_dn14) * var_gm) + (assign2270_e2952 * var_gm_dn14)) * p.p87), ((((assign2270_e2950 * var_t_dn15) * var_gm) + (assign2270_e2952 * var_gm_dn15)) * p.p87), ((((assign2270_e2950 * var_t_dn16) * var_gm) + (assign2270_e2952 * var_gm_dn16)) * p.p87), ((((assign2270_e2950 * var_t_dn17) * var_gm) + (assign2270_e2952 * var_gm_dn17)) * p.p87), ((((assign2270_e2950 * var_t_dn18) * var_gm) + (assign2270_e2952 * var_gm_dn18)) * p.p87), ((((assign2270_e2950 * var_t_db0) * var_gm) + (assign2270_e2952 * var_gm_db0)) * p.p87), ((((assign2270_e2950 * var_t_db1) * var_gm) + (assign2270_e2952 * var_gm_db1)) * p.p87), ((((assign2270_e2950 * var_t_db2) * var_gm) + (assign2270_e2952 * var_gm_db2)) * p.p87), ((((assign2270_e2950 * var_t_db3) * var_gm) + (assign2270_e2952 * var_gm_db3)) * p.p87), ((((assign2270_e2950 * var_t_db4) * var_gm) + (assign2270_e2952 * var_gm_db4)) * p.p87), ((((assign2270_e2950 * var_t_db5) * var_gm) + (assign2270_e2952 * var_gm_db5)) * p.p87), ((((assign2270_e2950 * var_t_db6) * var_gm) + (assign2270_e2952 * var_gm_db6)) * p.p87), ((((assign2270_e2950 * var_t_db7) * var_gm) + (assign2270_e2952 * var_gm_db7)) * p.p87), ((((assign2270_e2950 * var_t_db8) * var_gm) + (assign2270_e2952 * var_gm_db8)) * p.p87), ((((assign2270_e2950 * var_t_db9) * var_gm) + (assign2270_e2952 * var_gm_db9)) * p.p87), ((((assign2270_e2950 * var_t_db10) * var_gm) + (assign2270_e2952 * var_gm_db10)) * p.p87), ((((assign2270_e2950 * var_t_db11) * var_gm) + (assign2270_e2952 * var_gm_db11)) * p.p87), ((((assign2270_e2950 * var_t_db12) * var_gm) + (assign2270_e2952 * var_gm_db12)) * p.p87), ((((assign2270_e2950 * var_t_db13) * var_gm) + (assign2270_e2952 * var_gm_db13)) * p.p87), ((((assign2270_e2950 * var_t_db14) * var_gm) + (assign2270_e2952 * var_gm_db14)) * p.p87), ((((assign2270_e2950 * var_t_db15) * var_gm) + (assign2270_e2952 * var_gm_db15)) * p.p87), ((((assign2270_e2950 * var_t_db16) * var_gm) + (assign2270_e2952 * var_gm_db16)) * p.p87), ((((assign2270_e2950 * var_t_db17) * var_gm) + (assign2270_e2952 * var_gm_db17)) * p.p87), ((((assign2270_e2950 * var_t_db18) * var_gm) + (assign2270_e2952 * var_gm_db18)) * p.p87),)
    } else {
        (var_noisepwrd, var_noisepwrd_dn0, var_noisepwrd_dn1, var_noisepwrd_dn2, var_noisepwrd_dn3, var_noisepwrd_dn4, var_noisepwrd_dn5, var_noisepwrd_dn6, var_noisepwrd_dn7, var_noisepwrd_dn8, var_noisepwrd_dn9, var_noisepwrd_dn10, var_noisepwrd_dn11, var_noisepwrd_dn12, var_noisepwrd_dn13, var_noisepwrd_dn14, var_noisepwrd_dn15, var_noisepwrd_dn16, var_noisepwrd_dn17, var_noisepwrd_dn18, var_noisepwrd_db0, var_noisepwrd_db1, var_noisepwrd_db2, var_noisepwrd_db3, var_noisepwrd_db4, var_noisepwrd_db5, var_noisepwrd_db6, var_noisepwrd_db7, var_noisepwrd_db8, var_noisepwrd_db9, var_noisepwrd_db10, var_noisepwrd_db11, var_noisepwrd_db12, var_noisepwrd_db13, var_noisepwrd_db14, var_noisepwrd_db15, var_noisepwrd_db16, var_noisepwrd_db17, var_noisepwrd_db18,)
    }
};
        var_noisepwrd = assign2270_e2958;
        var_noisepwrd_dn0 = assign2270_e2958_d_n0;
        var_noisepwrd_dn1 = assign2270_e2958_d_n1;
        var_noisepwrd_dn2 = assign2270_e2958_d_n2;
        var_noisepwrd_dn3 = assign2270_e2958_d_n3;
        var_noisepwrd_dn4 = assign2270_e2958_d_n4;
        var_noisepwrd_dn5 = assign2270_e2958_d_n5;
        var_noisepwrd_dn6 = assign2270_e2958_d_n6;
        var_noisepwrd_dn7 = assign2270_e2958_d_n7;
        var_noisepwrd_dn8 = assign2270_e2958_d_n8;
        var_noisepwrd_dn9 = assign2270_e2958_d_n9;
        var_noisepwrd_dn10 = assign2270_e2958_d_n10;
        var_noisepwrd_dn11 = assign2270_e2958_d_n11;
        var_noisepwrd_dn12 = assign2270_e2958_d_n12;
        var_noisepwrd_dn13 = assign2270_e2958_d_n13;
        var_noisepwrd_dn14 = assign2270_e2958_d_n14;
        var_noisepwrd_dn15 = assign2270_e2958_d_n15;
        var_noisepwrd_dn16 = assign2270_e2958_d_n16;
        var_noisepwrd_dn17 = assign2270_e2958_d_n17;
        var_noisepwrd_dn18 = assign2270_e2958_d_n18;
        var_noisepwrd_db0 = assign2270_e2958_d_b0;
        var_noisepwrd_db1 = assign2270_e2958_d_b1;
        var_noisepwrd_db2 = assign2270_e2958_d_b2;
        var_noisepwrd_db3 = assign2270_e2958_d_b3;
        var_noisepwrd_db4 = assign2270_e2958_d_b4;
        var_noisepwrd_db5 = assign2270_e2958_d_b5;
        var_noisepwrd_db6 = assign2270_e2958_d_b6;
        var_noisepwrd_db7 = assign2270_e2958_d_b7;
        var_noisepwrd_db8 = assign2270_e2958_d_b8;
        var_noisepwrd_db9 = assign2270_e2958_d_b9;
        var_noisepwrd_db10 = assign2270_e2958_d_b10;
        var_noisepwrd_db11 = assign2270_e2958_d_b11;
        var_noisepwrd_db12 = assign2270_e2958_d_b12;
        var_noisepwrd_db13 = assign2270_e2958_d_b13;
        var_noisepwrd_db14 = assign2270_e2958_d_b14;
        var_noisepwrd_db15 = assign2270_e2958_d_b15;
        var_noisepwrd_db16 = assign2270_e2958_d_b16;
        var_noisepwrd_db17 = assign2270_e2958_d_b17;
        var_noisepwrd_db18 = assign2270_e2958_d_b18;
        var_noisepwrd_rv = 0.0;
        var_noisepwrd_rdn0 = 0.0;
        var_noisepwrd_rdn1 = 0.0;
        var_noisepwrd_rdn2 = 0.0;
        var_noisepwrd_rdn3 = 0.0;
        var_noisepwrd_rdn4 = 0.0;
        var_noisepwrd_rdn5 = 0.0;
        var_noisepwrd_rdn6 = 0.0;
        var_noisepwrd_rdn7 = 0.0;
        var_noisepwrd_rdn8 = 0.0;
        var_noisepwrd_rdn9 = 0.0;
        var_noisepwrd_rdn10 = 0.0;
        var_noisepwrd_rdn11 = 0.0;
        var_noisepwrd_rdn12 = 0.0;
        var_noisepwrd_rdn13 = 0.0;
        var_noisepwrd_rdn14 = 0.0;
        var_noisepwrd_rdn15 = 0.0;
        var_noisepwrd_rdn16 = 0.0;
        var_noisepwrd_rdn17 = 0.0;
        var_noisepwrd_rdn18 = 0.0;
        var_noisepwrd_rdb0 = 0.0;
        var_noisepwrd_rdb1 = 0.0;
        var_noisepwrd_rdb2 = 0.0;
        var_noisepwrd_rdb3 = 0.0;
        var_noisepwrd_rdb4 = 0.0;
        var_noisepwrd_rdb5 = 0.0;
        var_noisepwrd_rdb6 = 0.0;
        var_noisepwrd_rdb7 = 0.0;
        var_noisepwrd_rdb8 = 0.0;
        var_noisepwrd_rdb9 = 0.0;
        var_noisepwrd_rdb10 = 0.0;
        var_noisepwrd_rdb11 = 0.0;
        var_noisepwrd_rdb12 = 0.0;
        var_noisepwrd_rdb13 = 0.0;
        var_noisepwrd_rdb14 = 0.0;
        var_noisepwrd_rdb15 = 0.0;
        var_noisepwrd_rdb16 = 0.0;
        var_noisepwrd_rdb17 = 0.0;
        var_noisepwrd_rdb18 = 0.0;

        let assign2280_e2961: f64 = if var_gm > 0.0 { 1.0 } else { 0.0 };
        var_guard36 = assign2280_e2961;
        var_guard36_dn0 = 0.0;
        var_guard36_dn1 = 0.0;
        var_guard36_dn2 = 0.0;
        var_guard36_dn3 = 0.0;
        var_guard36_dn4 = 0.0;
        var_guard36_dn5 = 0.0;
        var_guard36_dn6 = 0.0;
        var_guard36_dn7 = 0.0;
        var_guard36_dn8 = 0.0;
        var_guard36_dn9 = 0.0;
        var_guard36_dn10 = 0.0;
        var_guard36_dn11 = 0.0;
        var_guard36_dn12 = 0.0;
        var_guard36_dn13 = 0.0;
        var_guard36_dn14 = 0.0;
        var_guard36_dn15 = 0.0;
        var_guard36_dn16 = 0.0;
        var_guard36_dn17 = 0.0;
        var_guard36_dn18 = 0.0;
        var_guard36_db0 = 0.0;
        var_guard36_db1 = 0.0;
        var_guard36_db2 = 0.0;
        var_guard36_db3 = 0.0;
        var_guard36_db4 = 0.0;
        var_guard36_db5 = 0.0;
        var_guard36_db6 = 0.0;
        var_guard36_db7 = 0.0;
        var_guard36_db8 = 0.0;
        var_guard36_db9 = 0.0;
        var_guard36_db10 = 0.0;
        var_guard36_db11 = 0.0;
        var_guard36_db12 = 0.0;
        var_guard36_db13 = 0.0;
        var_guard36_db14 = 0.0;
        var_guard36_db15 = 0.0;
        var_guard36_db16 = 0.0;
        var_guard36_db17 = 0.0;
        var_guard36_db18 = 0.0;
        var_guard36_rv = 0.0;
        var_guard36_rdn0 = 0.0;
        var_guard36_rdn1 = 0.0;
        var_guard36_rdn2 = 0.0;
        var_guard36_rdn3 = 0.0;
        var_guard36_rdn4 = 0.0;
        var_guard36_rdn5 = 0.0;
        var_guard36_rdn6 = 0.0;
        var_guard36_rdn7 = 0.0;
        var_guard36_rdn8 = 0.0;
        var_guard36_rdn9 = 0.0;
        var_guard36_rdn10 = 0.0;
        var_guard36_rdn11 = 0.0;
        var_guard36_rdn12 = 0.0;
        var_guard36_rdn13 = 0.0;
        var_guard36_rdn14 = 0.0;
        var_guard36_rdn15 = 0.0;
        var_guard36_rdn16 = 0.0;
        var_guard36_rdn17 = 0.0;
        var_guard36_rdn18 = 0.0;
        var_guard36_rdb0 = 0.0;
        var_guard36_rdb1 = 0.0;
        var_guard36_rdb2 = 0.0;
        var_guard36_rdb3 = 0.0;
        var_guard36_rdb4 = 0.0;
        var_guard36_rdb5 = 0.0;
        var_guard36_rdb6 = 0.0;
        var_guard36_rdb7 = 0.0;
        var_guard36_rdb8 = 0.0;
        var_guard36_rdb9 = 0.0;
        var_guard36_rdb10 = 0.0;
        var_guard36_rdb11 = 0.0;
        var_guard36_rdb12 = 0.0;
        var_guard36_rdb13 = 0.0;
        var_guard36_rdb14 = 0.0;
        var_guard36_rdb15 = 0.0;
        var_guard36_rdb16 = 0.0;
        var_guard36_rdb17 = 0.0;
        var_guard36_rdb18 = 0.0;

        let (assign2290_e2984, assign2290_e2984_d_n0, assign2290_e2984_d_n1, assign2290_e2984_d_n2, assign2290_e2984_d_n3, assign2290_e2984_d_n4, assign2290_e2984_d_n5, assign2290_e2984_d_n6, assign2290_e2984_d_n7, assign2290_e2984_d_n8, assign2290_e2984_d_n9, assign2290_e2984_d_n10, assign2290_e2984_d_n11, assign2290_e2984_d_n12, assign2290_e2984_d_n13, assign2290_e2984_d_n14, assign2290_e2984_d_n15, assign2290_e2984_d_n16, assign2290_e2984_d_n17, assign2290_e2984_d_n18, assign2290_e2984_d_b0, assign2290_e2984_d_b1, assign2290_e2984_d_b2, assign2290_e2984_d_b3, assign2290_e2984_d_b4, assign2290_e2984_d_b5, assign2290_e2984_d_b6, assign2290_e2984_d_b7, assign2290_e2984_d_b8, assign2290_e2984_d_b9, assign2290_e2984_d_b10, assign2290_e2984_d_b11, assign2290_e2984_d_b12, assign2290_e2984_d_b13, assign2290_e2984_d_b14, assign2290_e2984_d_b15, assign2290_e2984_d_b16, assign2290_e2984_d_b17, assign2290_e2984_d_b18,) = {
    if ((((var_guard29 != 0.0) && (var_guard28 == 0.0)) && (p.p0 != 0.0)) && (var_guard36 != 0.0)) {
        let assign2290_e2972: f64 = (var_cgs0_t * var_cgs0_t);
        let assign2290_e2974: f64 = (assign2290_e2972 * 4.0);
        let assign2290_e2976: f64 = (assign2290_e2974 * 1.3806503e-23);
        let assign2290_e2978: f64 = (assign2290_e2976 * var_t);
        let assign2290_e2980: f64 = (assign2290_e2978 * p.p86);
        let assign2290_e2982: f64 = (assign2290_e2980 / var_gm);
        (assign2290_e2982, ((((((((((var_cgs0_t_dn0 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn0)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn0)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn0)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn1 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn1)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn1)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn1)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn2 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn2)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn2)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn2)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn3 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn3)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn3)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn3)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn4 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn4)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn4)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn4)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn5 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn5)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn5)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn5)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn6 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn6)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn6)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn6)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn7 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn7)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn7)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn7)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn8 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn8)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn8)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn8)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn9 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn9)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn9)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn9)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn10 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn10)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn10)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn10)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn11 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn11)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn11)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn11)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn12 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn12)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn12)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn12)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn13 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn13)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn13)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn13)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn14 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn14)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn14)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn14)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn15 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn15)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn15)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn15)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn16 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn16)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn16)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn16)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn17 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn17)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn17)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn17)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_dn18 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_dn18)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_dn18)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_dn18)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db0 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db0)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db0)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db0)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db1 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db1)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db1)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db1)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db2 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db2)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db2)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db2)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db3 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db3)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db3)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db3)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db4 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db4)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db4)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db4)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db5 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db5)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db5)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db5)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db6 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db6)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db6)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db6)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db7 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db7)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db7)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db7)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db8 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db8)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db8)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db8)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db9 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db9)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db9)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db9)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db10 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db10)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db10)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db10)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db11 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db11)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db11)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db11)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db12 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db12)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db12)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db12)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db13 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db13)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db13)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db13)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db14 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db14)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db14)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db14)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db15 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db15)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db15)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db15)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db16 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db16)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db16)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db16)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db17 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db17)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db17)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db17)) / (var_gm * var_gm)), ((((((((((var_cgs0_t_db18 * var_cgs0_t) + (var_cgs0_t * var_cgs0_t_db18)) * 4.0) * 1.3806503e-23) * var_t) + (assign2290_e2976 * var_t_db18)) * p.p86) * var_gm) - (assign2290_e2980 * var_gm_db18)) / (var_gm * var_gm)),)
    } else {
        (var_noisepwrg, var_noisepwrg_dn0, var_noisepwrg_dn1, var_noisepwrg_dn2, var_noisepwrg_dn3, var_noisepwrg_dn4, var_noisepwrg_dn5, var_noisepwrg_dn6, var_noisepwrg_dn7, var_noisepwrg_dn8, var_noisepwrg_dn9, var_noisepwrg_dn10, var_noisepwrg_dn11, var_noisepwrg_dn12, var_noisepwrg_dn13, var_noisepwrg_dn14, var_noisepwrg_dn15, var_noisepwrg_dn16, var_noisepwrg_dn17, var_noisepwrg_dn18, var_noisepwrg_db0, var_noisepwrg_db1, var_noisepwrg_db2, var_noisepwrg_db3, var_noisepwrg_db4, var_noisepwrg_db5, var_noisepwrg_db6, var_noisepwrg_db7, var_noisepwrg_db8, var_noisepwrg_db9, var_noisepwrg_db10, var_noisepwrg_db11, var_noisepwrg_db12, var_noisepwrg_db13, var_noisepwrg_db14, var_noisepwrg_db15, var_noisepwrg_db16, var_noisepwrg_db17, var_noisepwrg_db18,)
    }
};
        var_noisepwrg = assign2290_e2984;
        var_noisepwrg_dn0 = assign2290_e2984_d_n0;
        var_noisepwrg_dn1 = assign2290_e2984_d_n1;
        var_noisepwrg_dn2 = assign2290_e2984_d_n2;
        var_noisepwrg_dn3 = assign2290_e2984_d_n3;
        var_noisepwrg_dn4 = assign2290_e2984_d_n4;
        var_noisepwrg_dn5 = assign2290_e2984_d_n5;
        var_noisepwrg_dn6 = assign2290_e2984_d_n6;
        var_noisepwrg_dn7 = assign2290_e2984_d_n7;
        var_noisepwrg_dn8 = assign2290_e2984_d_n8;
        var_noisepwrg_dn9 = assign2290_e2984_d_n9;
        var_noisepwrg_dn10 = assign2290_e2984_d_n10;
        var_noisepwrg_dn11 = assign2290_e2984_d_n11;
        var_noisepwrg_dn12 = assign2290_e2984_d_n12;
        var_noisepwrg_dn13 = assign2290_e2984_d_n13;
        var_noisepwrg_dn14 = assign2290_e2984_d_n14;
        var_noisepwrg_dn15 = assign2290_e2984_d_n15;
        var_noisepwrg_dn16 = assign2290_e2984_d_n16;
        var_noisepwrg_dn17 = assign2290_e2984_d_n17;
        var_noisepwrg_dn18 = assign2290_e2984_d_n18;
        var_noisepwrg_db0 = assign2290_e2984_d_b0;
        var_noisepwrg_db1 = assign2290_e2984_d_b1;
        var_noisepwrg_db2 = assign2290_e2984_d_b2;
        var_noisepwrg_db3 = assign2290_e2984_d_b3;
        var_noisepwrg_db4 = assign2290_e2984_d_b4;
        var_noisepwrg_db5 = assign2290_e2984_d_b5;
        var_noisepwrg_db6 = assign2290_e2984_d_b6;
        var_noisepwrg_db7 = assign2290_e2984_d_b7;
        var_noisepwrg_db8 = assign2290_e2984_d_b8;
        var_noisepwrg_db9 = assign2290_e2984_d_b9;
        var_noisepwrg_db10 = assign2290_e2984_d_b10;
        var_noisepwrg_db11 = assign2290_e2984_d_b11;
        var_noisepwrg_db12 = assign2290_e2984_d_b12;
        var_noisepwrg_db13 = assign2290_e2984_d_b13;
        var_noisepwrg_db14 = assign2290_e2984_d_b14;
        var_noisepwrg_db15 = assign2290_e2984_d_b15;
        var_noisepwrg_db16 = assign2290_e2984_d_b16;
        var_noisepwrg_db17 = assign2290_e2984_d_b17;
        var_noisepwrg_db18 = assign2290_e2984_d_b18;
        var_noisepwrg_rv = 0.0;
        var_noisepwrg_rdn0 = 0.0;
        var_noisepwrg_rdn1 = 0.0;
        var_noisepwrg_rdn2 = 0.0;
        var_noisepwrg_rdn3 = 0.0;
        var_noisepwrg_rdn4 = 0.0;
        var_noisepwrg_rdn5 = 0.0;
        var_noisepwrg_rdn6 = 0.0;
        var_noisepwrg_rdn7 = 0.0;
        var_noisepwrg_rdn8 = 0.0;
        var_noisepwrg_rdn9 = 0.0;
        var_noisepwrg_rdn10 = 0.0;
        var_noisepwrg_rdn11 = 0.0;
        var_noisepwrg_rdn12 = 0.0;
        var_noisepwrg_rdn13 = 0.0;
        var_noisepwrg_rdn14 = 0.0;
        var_noisepwrg_rdn15 = 0.0;
        var_noisepwrg_rdn16 = 0.0;
        var_noisepwrg_rdn17 = 0.0;
        var_noisepwrg_rdn18 = 0.0;
        var_noisepwrg_rdb0 = 0.0;
        var_noisepwrg_rdb1 = 0.0;
        var_noisepwrg_rdb2 = 0.0;
        var_noisepwrg_rdb3 = 0.0;
        var_noisepwrg_rdb4 = 0.0;
        var_noisepwrg_rdb5 = 0.0;
        var_noisepwrg_rdb6 = 0.0;
        var_noisepwrg_rdb7 = 0.0;
        var_noisepwrg_rdb8 = 0.0;
        var_noisepwrg_rdb9 = 0.0;
        var_noisepwrg_rdb10 = 0.0;
        var_noisepwrg_rdb11 = 0.0;
        var_noisepwrg_rdb12 = 0.0;
        var_noisepwrg_rdb13 = 0.0;
        var_noisepwrg_rdb14 = 0.0;
        var_noisepwrg_rdb15 = 0.0;
        var_noisepwrg_rdb16 = 0.0;
        var_noisepwrg_rdb17 = 0.0;
        var_noisepwrg_rdb18 = 0.0;

        let (assign2300_e2996, assign2300_e2996_d_n0, assign2300_e2996_d_n1, assign2300_e2996_d_n2, assign2300_e2996_d_n3, assign2300_e2996_d_n4, assign2300_e2996_d_n5, assign2300_e2996_d_n6, assign2300_e2996_d_n7, assign2300_e2996_d_n8, assign2300_e2996_d_n9, assign2300_e2996_d_n10, assign2300_e2996_d_n11, assign2300_e2996_d_n12, assign2300_e2996_d_n13, assign2300_e2996_d_n14, assign2300_e2996_d_n15, assign2300_e2996_d_n16, assign2300_e2996_d_n17, assign2300_e2996_d_n18, assign2300_e2996_d_b0, assign2300_e2996_d_b1, assign2300_e2996_d_b2, assign2300_e2996_d_b3, assign2300_e2996_d_b4, assign2300_e2996_d_b5, assign2300_e2996_d_b6, assign2300_e2996_d_b7, assign2300_e2996_d_b8, assign2300_e2996_d_b9, assign2300_e2996_d_b10, assign2300_e2996_d_b11, assign2300_e2996_d_b12, assign2300_e2996_d_b13, assign2300_e2996_d_b14, assign2300_e2996_d_b15, assign2300_e2996_d_b16, assign2300_e2996_d_b17, assign2300_e2996_d_b18,) = {
    if ((((var_guard29 != 0.0) && (var_guard28 == 0.0)) && (p.p0 != 0.0)) && (var_guard36 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_noisepwrg, var_noisepwrg_dn0, var_noisepwrg_dn1, var_noisepwrg_dn2, var_noisepwrg_dn3, var_noisepwrg_dn4, var_noisepwrg_dn5, var_noisepwrg_dn6, var_noisepwrg_dn7, var_noisepwrg_dn8, var_noisepwrg_dn9, var_noisepwrg_dn10, var_noisepwrg_dn11, var_noisepwrg_dn12, var_noisepwrg_dn13, var_noisepwrg_dn14, var_noisepwrg_dn15, var_noisepwrg_dn16, var_noisepwrg_dn17, var_noisepwrg_dn18, var_noisepwrg_db0, var_noisepwrg_db1, var_noisepwrg_db2, var_noisepwrg_db3, var_noisepwrg_db4, var_noisepwrg_db5, var_noisepwrg_db6, var_noisepwrg_db7, var_noisepwrg_db8, var_noisepwrg_db9, var_noisepwrg_db10, var_noisepwrg_db11, var_noisepwrg_db12, var_noisepwrg_db13, var_noisepwrg_db14, var_noisepwrg_db15, var_noisepwrg_db16, var_noisepwrg_db17, var_noisepwrg_db18,)
    }
};
        var_noisepwrg = assign2300_e2996;
        var_noisepwrg_dn0 = assign2300_e2996_d_n0;
        var_noisepwrg_dn1 = assign2300_e2996_d_n1;
        var_noisepwrg_dn2 = assign2300_e2996_d_n2;
        var_noisepwrg_dn3 = assign2300_e2996_d_n3;
        var_noisepwrg_dn4 = assign2300_e2996_d_n4;
        var_noisepwrg_dn5 = assign2300_e2996_d_n5;
        var_noisepwrg_dn6 = assign2300_e2996_d_n6;
        var_noisepwrg_dn7 = assign2300_e2996_d_n7;
        var_noisepwrg_dn8 = assign2300_e2996_d_n8;
        var_noisepwrg_dn9 = assign2300_e2996_d_n9;
        var_noisepwrg_dn10 = assign2300_e2996_d_n10;
        var_noisepwrg_dn11 = assign2300_e2996_d_n11;
        var_noisepwrg_dn12 = assign2300_e2996_d_n12;
        var_noisepwrg_dn13 = assign2300_e2996_d_n13;
        var_noisepwrg_dn14 = assign2300_e2996_d_n14;
        var_noisepwrg_dn15 = assign2300_e2996_d_n15;
        var_noisepwrg_dn16 = assign2300_e2996_d_n16;
        var_noisepwrg_dn17 = assign2300_e2996_d_n17;
        var_noisepwrg_dn18 = assign2300_e2996_d_n18;
        var_noisepwrg_db0 = assign2300_e2996_d_b0;
        var_noisepwrg_db1 = assign2300_e2996_d_b1;
        var_noisepwrg_db2 = assign2300_e2996_d_b2;
        var_noisepwrg_db3 = assign2300_e2996_d_b3;
        var_noisepwrg_db4 = assign2300_e2996_d_b4;
        var_noisepwrg_db5 = assign2300_e2996_d_b5;
        var_noisepwrg_db6 = assign2300_e2996_d_b6;
        var_noisepwrg_db7 = assign2300_e2996_d_b7;
        var_noisepwrg_db8 = assign2300_e2996_d_b8;
        var_noisepwrg_db9 = assign2300_e2996_d_b9;
        var_noisepwrg_db10 = assign2300_e2996_d_b10;
        var_noisepwrg_db11 = assign2300_e2996_d_b11;
        var_noisepwrg_db12 = assign2300_e2996_d_b12;
        var_noisepwrg_db13 = assign2300_e2996_d_b13;
        var_noisepwrg_db14 = assign2300_e2996_d_b14;
        var_noisepwrg_db15 = assign2300_e2996_d_b15;
        var_noisepwrg_db16 = assign2300_e2996_d_b16;
        var_noisepwrg_db17 = assign2300_e2996_d_b17;
        var_noisepwrg_db18 = assign2300_e2996_d_b18;
        var_noisepwrg_rv = 0.0;
        var_noisepwrg_rdn0 = 0.0;
        var_noisepwrg_rdn1 = 0.0;
        var_noisepwrg_rdn2 = 0.0;
        var_noisepwrg_rdn3 = 0.0;
        var_noisepwrg_rdn4 = 0.0;
        var_noisepwrg_rdn5 = 0.0;
        var_noisepwrg_rdn6 = 0.0;
        var_noisepwrg_rdn7 = 0.0;
        var_noisepwrg_rdn8 = 0.0;
        var_noisepwrg_rdn9 = 0.0;
        var_noisepwrg_rdn10 = 0.0;
        var_noisepwrg_rdn11 = 0.0;
        var_noisepwrg_rdn12 = 0.0;
        var_noisepwrg_rdn13 = 0.0;
        var_noisepwrg_rdn14 = 0.0;
        var_noisepwrg_rdn15 = 0.0;
        var_noisepwrg_rdn16 = 0.0;
        var_noisepwrg_rdn17 = 0.0;
        var_noisepwrg_rdn18 = 0.0;
        var_noisepwrg_rdb0 = 0.0;
        var_noisepwrg_rdb1 = 0.0;
        var_noisepwrg_rdb2 = 0.0;
        var_noisepwrg_rdb3 = 0.0;
        var_noisepwrg_rdb4 = 0.0;
        var_noisepwrg_rdb5 = 0.0;
        var_noisepwrg_rdb6 = 0.0;
        var_noisepwrg_rdb7 = 0.0;
        var_noisepwrg_rdb8 = 0.0;
        var_noisepwrg_rdb9 = 0.0;
        var_noisepwrg_rdb10 = 0.0;
        var_noisepwrg_rdb11 = 0.0;
        var_noisepwrg_rdb12 = 0.0;
        var_noisepwrg_rdb13 = 0.0;
        var_noisepwrg_rdb14 = 0.0;
        var_noisepwrg_rdb15 = 0.0;
        var_noisepwrg_rdb16 = 0.0;
        var_noisepwrg_rdb17 = 0.0;
        var_noisepwrg_rdb18 = 0.0;


        *var_guard36_slot = var_guard36;
        *var_guard36_db0_slot = var_guard36_db0;
        *var_guard36_db1_slot = var_guard36_db1;
        *var_guard36_db10_slot = var_guard36_db10;
        *var_guard36_db11_slot = var_guard36_db11;
        *var_guard36_db12_slot = var_guard36_db12;
        *var_guard36_db13_slot = var_guard36_db13;
        *var_guard36_db14_slot = var_guard36_db14;
        *var_guard36_db15_slot = var_guard36_db15;
        *var_guard36_db16_slot = var_guard36_db16;
        *var_guard36_db17_slot = var_guard36_db17;
        *var_guard36_db18_slot = var_guard36_db18;
        *var_guard36_db2_slot = var_guard36_db2;
        *var_guard36_db3_slot = var_guard36_db3;
        *var_guard36_db4_slot = var_guard36_db4;
        *var_guard36_db5_slot = var_guard36_db5;
        *var_guard36_db6_slot = var_guard36_db6;
        *var_guard36_db7_slot = var_guard36_db7;
        *var_guard36_db8_slot = var_guard36_db8;
        *var_guard36_db9_slot = var_guard36_db9;
        *var_guard36_dn0_slot = var_guard36_dn0;
        *var_guard36_dn1_slot = var_guard36_dn1;
        *var_guard36_dn10_slot = var_guard36_dn10;
        *var_guard36_dn11_slot = var_guard36_dn11;
        *var_guard36_dn12_slot = var_guard36_dn12;
        *var_guard36_dn13_slot = var_guard36_dn13;
        *var_guard36_dn14_slot = var_guard36_dn14;
        *var_guard36_dn15_slot = var_guard36_dn15;
        *var_guard36_dn16_slot = var_guard36_dn16;
        *var_guard36_dn17_slot = var_guard36_dn17;
        *var_guard36_dn18_slot = var_guard36_dn18;
        *var_guard36_dn2_slot = var_guard36_dn2;
        *var_guard36_dn3_slot = var_guard36_dn3;
        *var_guard36_dn4_slot = var_guard36_dn4;
        *var_guard36_dn5_slot = var_guard36_dn5;
        *var_guard36_dn6_slot = var_guard36_dn6;
        *var_guard36_dn7_slot = var_guard36_dn7;
        *var_guard36_dn8_slot = var_guard36_dn8;
        *var_guard36_dn9_slot = var_guard36_dn9;
        *var_guard36_rdb0_slot = var_guard36_rdb0;
        *var_guard36_rdb1_slot = var_guard36_rdb1;
        *var_guard36_rdb10_slot = var_guard36_rdb10;
        *var_guard36_rdb11_slot = var_guard36_rdb11;
        *var_guard36_rdb12_slot = var_guard36_rdb12;
        *var_guard36_rdb13_slot = var_guard36_rdb13;
        *var_guard36_rdb14_slot = var_guard36_rdb14;
        *var_guard36_rdb15_slot = var_guard36_rdb15;
        *var_guard36_rdb16_slot = var_guard36_rdb16;
        *var_guard36_rdb17_slot = var_guard36_rdb17;
        *var_guard36_rdb18_slot = var_guard36_rdb18;
        *var_guard36_rdb2_slot = var_guard36_rdb2;
        *var_guard36_rdb3_slot = var_guard36_rdb3;
        *var_guard36_rdb4_slot = var_guard36_rdb4;
        *var_guard36_rdb5_slot = var_guard36_rdb5;
        *var_guard36_rdb6_slot = var_guard36_rdb6;
        *var_guard36_rdb7_slot = var_guard36_rdb7;
        *var_guard36_rdb8_slot = var_guard36_rdb8;
        *var_guard36_rdb9_slot = var_guard36_rdb9;
        *var_guard36_rdn0_slot = var_guard36_rdn0;
        *var_guard36_rdn1_slot = var_guard36_rdn1;
        *var_guard36_rdn10_slot = var_guard36_rdn10;
        *var_guard36_rdn11_slot = var_guard36_rdn11;
        *var_guard36_rdn12_slot = var_guard36_rdn12;
        *var_guard36_rdn13_slot = var_guard36_rdn13;
        *var_guard36_rdn14_slot = var_guard36_rdn14;
        *var_guard36_rdn15_slot = var_guard36_rdn15;
        *var_guard36_rdn16_slot = var_guard36_rdn16;
        *var_guard36_rdn17_slot = var_guard36_rdn17;
        *var_guard36_rdn18_slot = var_guard36_rdn18;
        *var_guard36_rdn2_slot = var_guard36_rdn2;
        *var_guard36_rdn3_slot = var_guard36_rdn3;
        *var_guard36_rdn4_slot = var_guard36_rdn4;
        *var_guard36_rdn5_slot = var_guard36_rdn5;
        *var_guard36_rdn6_slot = var_guard36_rdn6;
        *var_guard36_rdn7_slot = var_guard36_rdn7;
        *var_guard36_rdn8_slot = var_guard36_rdn8;
        *var_guard36_rdn9_slot = var_guard36_rdn9;
        *var_guard36_rv_slot = var_guard36_rv;
        *var_noisepwr_slot = var_noisepwr;
        *var_noisepwr_db0_slot = var_noisepwr_db0;
        *var_noisepwr_db1_slot = var_noisepwr_db1;
        *var_noisepwr_db10_slot = var_noisepwr_db10;
        *var_noisepwr_db11_slot = var_noisepwr_db11;
        *var_noisepwr_db12_slot = var_noisepwr_db12;
        *var_noisepwr_db13_slot = var_noisepwr_db13;
        *var_noisepwr_db14_slot = var_noisepwr_db14;
        *var_noisepwr_db15_slot = var_noisepwr_db15;
        *var_noisepwr_db16_slot = var_noisepwr_db16;
        *var_noisepwr_db17_slot = var_noisepwr_db17;
        *var_noisepwr_db18_slot = var_noisepwr_db18;
        *var_noisepwr_db2_slot = var_noisepwr_db2;
        *var_noisepwr_db3_slot = var_noisepwr_db3;
        *var_noisepwr_db4_slot = var_noisepwr_db4;
        *var_noisepwr_db5_slot = var_noisepwr_db5;
        *var_noisepwr_db6_slot = var_noisepwr_db6;
        *var_noisepwr_db7_slot = var_noisepwr_db7;
        *var_noisepwr_db8_slot = var_noisepwr_db8;
        *var_noisepwr_db9_slot = var_noisepwr_db9;
        *var_noisepwr_dn0_slot = var_noisepwr_dn0;
        *var_noisepwr_dn1_slot = var_noisepwr_dn1;
        *var_noisepwr_dn10_slot = var_noisepwr_dn10;
        *var_noisepwr_dn11_slot = var_noisepwr_dn11;
        *var_noisepwr_dn12_slot = var_noisepwr_dn12;
        *var_noisepwr_dn13_slot = var_noisepwr_dn13;
        *var_noisepwr_dn14_slot = var_noisepwr_dn14;
        *var_noisepwr_dn15_slot = var_noisepwr_dn15;
        *var_noisepwr_dn16_slot = var_noisepwr_dn16;
        *var_noisepwr_dn17_slot = var_noisepwr_dn17;
        *var_noisepwr_dn18_slot = var_noisepwr_dn18;
        *var_noisepwr_dn2_slot = var_noisepwr_dn2;
        *var_noisepwr_dn3_slot = var_noisepwr_dn3;
        *var_noisepwr_dn4_slot = var_noisepwr_dn4;
        *var_noisepwr_dn5_slot = var_noisepwr_dn5;
        *var_noisepwr_dn6_slot = var_noisepwr_dn6;
        *var_noisepwr_dn7_slot = var_noisepwr_dn7;
        *var_noisepwr_dn8_slot = var_noisepwr_dn8;
        *var_noisepwr_dn9_slot = var_noisepwr_dn9;
        *var_noisepwr_rdb0_slot = var_noisepwr_rdb0;
        *var_noisepwr_rdb1_slot = var_noisepwr_rdb1;
        *var_noisepwr_rdb10_slot = var_noisepwr_rdb10;
        *var_noisepwr_rdb11_slot = var_noisepwr_rdb11;
        *var_noisepwr_rdb12_slot = var_noisepwr_rdb12;
        *var_noisepwr_rdb13_slot = var_noisepwr_rdb13;
        *var_noisepwr_rdb14_slot = var_noisepwr_rdb14;
        *var_noisepwr_rdb15_slot = var_noisepwr_rdb15;
        *var_noisepwr_rdb16_slot = var_noisepwr_rdb16;
        *var_noisepwr_rdb17_slot = var_noisepwr_rdb17;
        *var_noisepwr_rdb18_slot = var_noisepwr_rdb18;
        *var_noisepwr_rdb2_slot = var_noisepwr_rdb2;
        *var_noisepwr_rdb3_slot = var_noisepwr_rdb3;
        *var_noisepwr_rdb4_slot = var_noisepwr_rdb4;
        *var_noisepwr_rdb5_slot = var_noisepwr_rdb5;
        *var_noisepwr_rdb6_slot = var_noisepwr_rdb6;
        *var_noisepwr_rdb7_slot = var_noisepwr_rdb7;
        *var_noisepwr_rdb8_slot = var_noisepwr_rdb8;
        *var_noisepwr_rdb9_slot = var_noisepwr_rdb9;
        *var_noisepwr_rdn0_slot = var_noisepwr_rdn0;
        *var_noisepwr_rdn1_slot = var_noisepwr_rdn1;
        *var_noisepwr_rdn10_slot = var_noisepwr_rdn10;
        *var_noisepwr_rdn11_slot = var_noisepwr_rdn11;
        *var_noisepwr_rdn12_slot = var_noisepwr_rdn12;
        *var_noisepwr_rdn13_slot = var_noisepwr_rdn13;
        *var_noisepwr_rdn14_slot = var_noisepwr_rdn14;
        *var_noisepwr_rdn15_slot = var_noisepwr_rdn15;
        *var_noisepwr_rdn16_slot = var_noisepwr_rdn16;
        *var_noisepwr_rdn17_slot = var_noisepwr_rdn17;
        *var_noisepwr_rdn18_slot = var_noisepwr_rdn18;
        *var_noisepwr_rdn2_slot = var_noisepwr_rdn2;
        *var_noisepwr_rdn3_slot = var_noisepwr_rdn3;
        *var_noisepwr_rdn4_slot = var_noisepwr_rdn4;
        *var_noisepwr_rdn5_slot = var_noisepwr_rdn5;
        *var_noisepwr_rdn6_slot = var_noisepwr_rdn6;
        *var_noisepwr_rdn7_slot = var_noisepwr_rdn7;
        *var_noisepwr_rdn8_slot = var_noisepwr_rdn8;
        *var_noisepwr_rdn9_slot = var_noisepwr_rdn9;
        *var_noisepwr_rv_slot = var_noisepwr_rv;
        *var_noisepwrd_slot = var_noisepwrd;
        *var_noisepwrd_db0_slot = var_noisepwrd_db0;
        *var_noisepwrd_db1_slot = var_noisepwrd_db1;
        *var_noisepwrd_db10_slot = var_noisepwrd_db10;
        *var_noisepwrd_db11_slot = var_noisepwrd_db11;
        *var_noisepwrd_db12_slot = var_noisepwrd_db12;
        *var_noisepwrd_db13_slot = var_noisepwrd_db13;
        *var_noisepwrd_db14_slot = var_noisepwrd_db14;
        *var_noisepwrd_db15_slot = var_noisepwrd_db15;
        *var_noisepwrd_db16_slot = var_noisepwrd_db16;
        *var_noisepwrd_db17_slot = var_noisepwrd_db17;
        *var_noisepwrd_db18_slot = var_noisepwrd_db18;
        *var_noisepwrd_db2_slot = var_noisepwrd_db2;
        *var_noisepwrd_db3_slot = var_noisepwrd_db3;
        *var_noisepwrd_db4_slot = var_noisepwrd_db4;
        *var_noisepwrd_db5_slot = var_noisepwrd_db5;
        *var_noisepwrd_db6_slot = var_noisepwrd_db6;
        *var_noisepwrd_db7_slot = var_noisepwrd_db7;
        *var_noisepwrd_db8_slot = var_noisepwrd_db8;
        *var_noisepwrd_db9_slot = var_noisepwrd_db9;
        *var_noisepwrd_dn0_slot = var_noisepwrd_dn0;
        *var_noisepwrd_dn1_slot = var_noisepwrd_dn1;
        *var_noisepwrd_dn10_slot = var_noisepwrd_dn10;
        *var_noisepwrd_dn11_slot = var_noisepwrd_dn11;
        *var_noisepwrd_dn12_slot = var_noisepwrd_dn12;
        *var_noisepwrd_dn13_slot = var_noisepwrd_dn13;
        *var_noisepwrd_dn14_slot = var_noisepwrd_dn14;
        *var_noisepwrd_dn15_slot = var_noisepwrd_dn15;
        *var_noisepwrd_dn16_slot = var_noisepwrd_dn16;
        *var_noisepwrd_dn17_slot = var_noisepwrd_dn17;
        *var_noisepwrd_dn18_slot = var_noisepwrd_dn18;
        *var_noisepwrd_dn2_slot = var_noisepwrd_dn2;
        *var_noisepwrd_dn3_slot = var_noisepwrd_dn3;
        *var_noisepwrd_dn4_slot = var_noisepwrd_dn4;
        *var_noisepwrd_dn5_slot = var_noisepwrd_dn5;
        *var_noisepwrd_dn6_slot = var_noisepwrd_dn6;
        *var_noisepwrd_dn7_slot = var_noisepwrd_dn7;
        *var_noisepwrd_dn8_slot = var_noisepwrd_dn8;
        *var_noisepwrd_dn9_slot = var_noisepwrd_dn9;
        *var_noisepwrd_rdb0_slot = var_noisepwrd_rdb0;
        *var_noisepwrd_rdb1_slot = var_noisepwrd_rdb1;
        *var_noisepwrd_rdb10_slot = var_noisepwrd_rdb10;
        *var_noisepwrd_rdb11_slot = var_noisepwrd_rdb11;
        *var_noisepwrd_rdb12_slot = var_noisepwrd_rdb12;
        *var_noisepwrd_rdb13_slot = var_noisepwrd_rdb13;
        *var_noisepwrd_rdb14_slot = var_noisepwrd_rdb14;
        *var_noisepwrd_rdb15_slot = var_noisepwrd_rdb15;
        *var_noisepwrd_rdb16_slot = var_noisepwrd_rdb16;
        *var_noisepwrd_rdb17_slot = var_noisepwrd_rdb17;
        *var_noisepwrd_rdb18_slot = var_noisepwrd_rdb18;
        *var_noisepwrd_rdb2_slot = var_noisepwrd_rdb2;
        *var_noisepwrd_rdb3_slot = var_noisepwrd_rdb3;
        *var_noisepwrd_rdb4_slot = var_noisepwrd_rdb4;
        *var_noisepwrd_rdb5_slot = var_noisepwrd_rdb5;
        *var_noisepwrd_rdb6_slot = var_noisepwrd_rdb6;
        *var_noisepwrd_rdb7_slot = var_noisepwrd_rdb7;
        *var_noisepwrd_rdb8_slot = var_noisepwrd_rdb8;
        *var_noisepwrd_rdb9_slot = var_noisepwrd_rdb9;
        *var_noisepwrd_rdn0_slot = var_noisepwrd_rdn0;
        *var_noisepwrd_rdn1_slot = var_noisepwrd_rdn1;
        *var_noisepwrd_rdn10_slot = var_noisepwrd_rdn10;
        *var_noisepwrd_rdn11_slot = var_noisepwrd_rdn11;
        *var_noisepwrd_rdn12_slot = var_noisepwrd_rdn12;
        *var_noisepwrd_rdn13_slot = var_noisepwrd_rdn13;
        *var_noisepwrd_rdn14_slot = var_noisepwrd_rdn14;
        *var_noisepwrd_rdn15_slot = var_noisepwrd_rdn15;
        *var_noisepwrd_rdn16_slot = var_noisepwrd_rdn16;
        *var_noisepwrd_rdn17_slot = var_noisepwrd_rdn17;
        *var_noisepwrd_rdn18_slot = var_noisepwrd_rdn18;
        *var_noisepwrd_rdn2_slot = var_noisepwrd_rdn2;
        *var_noisepwrd_rdn3_slot = var_noisepwrd_rdn3;
        *var_noisepwrd_rdn4_slot = var_noisepwrd_rdn4;
        *var_noisepwrd_rdn5_slot = var_noisepwrd_rdn5;
        *var_noisepwrd_rdn6_slot = var_noisepwrd_rdn6;
        *var_noisepwrd_rdn7_slot = var_noisepwrd_rdn7;
        *var_noisepwrd_rdn8_slot = var_noisepwrd_rdn8;
        *var_noisepwrd_rdn9_slot = var_noisepwrd_rdn9;
        *var_noisepwrd_rv_slot = var_noisepwrd_rv;
        *var_noisepwrg_slot = var_noisepwrg;
        *var_noisepwrg_db0_slot = var_noisepwrg_db0;
        *var_noisepwrg_db1_slot = var_noisepwrg_db1;
        *var_noisepwrg_db10_slot = var_noisepwrg_db10;
        *var_noisepwrg_db11_slot = var_noisepwrg_db11;
        *var_noisepwrg_db12_slot = var_noisepwrg_db12;
        *var_noisepwrg_db13_slot = var_noisepwrg_db13;
        *var_noisepwrg_db14_slot = var_noisepwrg_db14;
        *var_noisepwrg_db15_slot = var_noisepwrg_db15;
        *var_noisepwrg_db16_slot = var_noisepwrg_db16;
        *var_noisepwrg_db17_slot = var_noisepwrg_db17;
        *var_noisepwrg_db18_slot = var_noisepwrg_db18;
        *var_noisepwrg_db2_slot = var_noisepwrg_db2;
        *var_noisepwrg_db3_slot = var_noisepwrg_db3;
        *var_noisepwrg_db4_slot = var_noisepwrg_db4;
        *var_noisepwrg_db5_slot = var_noisepwrg_db5;
        *var_noisepwrg_db6_slot = var_noisepwrg_db6;
        *var_noisepwrg_db7_slot = var_noisepwrg_db7;
        *var_noisepwrg_db8_slot = var_noisepwrg_db8;
        *var_noisepwrg_db9_slot = var_noisepwrg_db9;
        *var_noisepwrg_dn0_slot = var_noisepwrg_dn0;
        *var_noisepwrg_dn1_slot = var_noisepwrg_dn1;
        *var_noisepwrg_dn10_slot = var_noisepwrg_dn10;
        *var_noisepwrg_dn11_slot = var_noisepwrg_dn11;
        *var_noisepwrg_dn12_slot = var_noisepwrg_dn12;
        *var_noisepwrg_dn13_slot = var_noisepwrg_dn13;
        *var_noisepwrg_dn14_slot = var_noisepwrg_dn14;
        *var_noisepwrg_dn15_slot = var_noisepwrg_dn15;
        *var_noisepwrg_dn16_slot = var_noisepwrg_dn16;
        *var_noisepwrg_dn17_slot = var_noisepwrg_dn17;
        *var_noisepwrg_dn18_slot = var_noisepwrg_dn18;
        *var_noisepwrg_dn2_slot = var_noisepwrg_dn2;
        *var_noisepwrg_dn3_slot = var_noisepwrg_dn3;
        *var_noisepwrg_dn4_slot = var_noisepwrg_dn4;
        *var_noisepwrg_dn5_slot = var_noisepwrg_dn5;
        *var_noisepwrg_dn6_slot = var_noisepwrg_dn6;
        *var_noisepwrg_dn7_slot = var_noisepwrg_dn7;
        *var_noisepwrg_dn8_slot = var_noisepwrg_dn8;
        *var_noisepwrg_dn9_slot = var_noisepwrg_dn9;
        *var_noisepwrg_rdb0_slot = var_noisepwrg_rdb0;
        *var_noisepwrg_rdb1_slot = var_noisepwrg_rdb1;
        *var_noisepwrg_rdb10_slot = var_noisepwrg_rdb10;
        *var_noisepwrg_rdb11_slot = var_noisepwrg_rdb11;
        *var_noisepwrg_rdb12_slot = var_noisepwrg_rdb12;
        *var_noisepwrg_rdb13_slot = var_noisepwrg_rdb13;
        *var_noisepwrg_rdb14_slot = var_noisepwrg_rdb14;
        *var_noisepwrg_rdb15_slot = var_noisepwrg_rdb15;
        *var_noisepwrg_rdb16_slot = var_noisepwrg_rdb16;
        *var_noisepwrg_rdb17_slot = var_noisepwrg_rdb17;
        *var_noisepwrg_rdb18_slot = var_noisepwrg_rdb18;
        *var_noisepwrg_rdb2_slot = var_noisepwrg_rdb2;
        *var_noisepwrg_rdb3_slot = var_noisepwrg_rdb3;
        *var_noisepwrg_rdb4_slot = var_noisepwrg_rdb4;
        *var_noisepwrg_rdb5_slot = var_noisepwrg_rdb5;
        *var_noisepwrg_rdb6_slot = var_noisepwrg_rdb6;
        *var_noisepwrg_rdb7_slot = var_noisepwrg_rdb7;
        *var_noisepwrg_rdb8_slot = var_noisepwrg_rdb8;
        *var_noisepwrg_rdb9_slot = var_noisepwrg_rdb9;
        *var_noisepwrg_rdn0_slot = var_noisepwrg_rdn0;
        *var_noisepwrg_rdn1_slot = var_noisepwrg_rdn1;
        *var_noisepwrg_rdn10_slot = var_noisepwrg_rdn10;
        *var_noisepwrg_rdn11_slot = var_noisepwrg_rdn11;
        *var_noisepwrg_rdn12_slot = var_noisepwrg_rdn12;
        *var_noisepwrg_rdn13_slot = var_noisepwrg_rdn13;
        *var_noisepwrg_rdn14_slot = var_noisepwrg_rdn14;
        *var_noisepwrg_rdn15_slot = var_noisepwrg_rdn15;
        *var_noisepwrg_rdn16_slot = var_noisepwrg_rdn16;
        *var_noisepwrg_rdn17_slot = var_noisepwrg_rdn17;
        *var_noisepwrg_rdn18_slot = var_noisepwrg_rdn18;
        *var_noisepwrg_rdn2_slot = var_noisepwrg_rdn2;
        *var_noisepwrg_rdn3_slot = var_noisepwrg_rdn3;
        *var_noisepwrg_rdn4_slot = var_noisepwrg_rdn4;
        *var_noisepwrg_rdn5_slot = var_noisepwrg_rdn5;
        *var_noisepwrg_rdn6_slot = var_noisepwrg_rdn6;
        *var_noisepwrg_rdn7_slot = var_noisepwrg_rdn7;
        *var_noisepwrg_rdn8_slot = var_noisepwrg_rdn8;
        *var_noisepwrg_rdn9_slot = var_noisepwrg_rdn9;
        *var_noisepwrg_rv_slot = var_noisepwrg_rv;
    }

    pub(super) fn stamp_reactive_block_41(
        p: &Parameters,
        var_cgs0_t: f64,
        var_cgs0_t_db0: f64,
        var_cgs0_t_db1: f64,
        var_cgs0_t_db10: f64,
        var_cgs0_t_db11: f64,
        var_cgs0_t_db12: f64,
        var_cgs0_t_db13: f64,
        var_cgs0_t_db14: f64,
        var_cgs0_t_db15: f64,
        var_cgs0_t_db16: f64,
        var_cgs0_t_db17: f64,
        var_cgs0_t_db18: f64,
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
        var_cgs0_t_dn16: f64,
        var_cgs0_t_dn17: f64,
        var_cgs0_t_dn18: f64,
        var_cgs0_t_dn2: f64,
        var_cgs0_t_dn3: f64,
        var_cgs0_t_dn4: f64,
        var_cgs0_t_dn5: f64,
        var_cgs0_t_dn6: f64,
        var_cgs0_t_dn7: f64,
        var_cgs0_t_dn8: f64,
        var_cgs0_t_dn9: f64,
        var_gm: f64,
        var_gm_db0: f64,
        var_gm_db1: f64,
        var_gm_db10: f64,
        var_gm_db11: f64,
        var_gm_db12: f64,
        var_gm_db13: f64,
        var_gm_db14: f64,
        var_gm_db15: f64,
        var_gm_db16: f64,
        var_gm_db17: f64,
        var_gm_db18: f64,
        var_gm_db2: f64,
        var_gm_db3: f64,
        var_gm_db4: f64,
        var_gm_db5: f64,
        var_gm_db6: f64,
        var_gm_db7: f64,
        var_gm_db8: f64,
        var_gm_db9: f64,
        var_gm_dn0: f64,
        var_gm_dn1: f64,
        var_gm_dn10: f64,
        var_gm_dn11: f64,
        var_gm_dn12: f64,
        var_gm_dn13: f64,
        var_gm_dn14: f64,
        var_gm_dn15: f64,
        var_gm_dn16: f64,
        var_gm_dn17: f64,
        var_gm_dn18: f64,
        var_gm_dn2: f64,
        var_gm_dn3: f64,
        var_gm_dn4: f64,
        var_gm_dn5: f64,
        var_gm_dn6: f64,
        var_gm_dn7: f64,
        var_gm_dn8: f64,
        var_gm_dn9: f64,
        var_guard28: f64,
        var_guard29: f64,
        var_t: f64,
        var_t_db0: f64,
        var_t_db1: f64,
        var_t_db10: f64,
        var_t_db11: f64,
        var_t_db12: f64,
        var_t_db13: f64,
        var_t_db14: f64,
        var_t_db15: f64,
        var_t_db16: f64,
        var_t_db17: f64,
        var_t_db18: f64,
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
        var_t_dn16: f64,
        var_t_dn17: f64,
        var_t_dn18: f64,
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
        var_ci_dn16_slot: &mut f64,
        var_ci_dn17_slot: &mut f64,
        var_ci_dn18_slot: &mut f64,
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
        var_ci_rdn16_slot: &mut f64,
        var_ci_rdn17_slot: &mut f64,
        var_ci_rdn18_slot: &mut f64,
        var_ci_rdn2_slot: &mut f64,
        var_ci_rdn3_slot: &mut f64,
        var_ci_rdn4_slot: &mut f64,
        var_ci_rdn5_slot: &mut f64,
        var_ci_rdn6_slot: &mut f64,
        var_ci_rdn7_slot: &mut f64,
        var_ci_rdn8_slot: &mut f64,
        var_ci_rdn9_slot: &mut f64,
        var_ci_rv_slot: &mut f64,
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
        var_guard43_dn16_slot: &mut f64,
        var_guard43_dn17_slot: &mut f64,
        var_guard43_dn18_slot: &mut f64,
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
        var_guard43_rdn16_slot: &mut f64,
        var_guard43_rdn17_slot: &mut f64,
        var_guard43_rdn18_slot: &mut f64,
        var_guard43_rdn2_slot: &mut f64,
        var_guard43_rdn3_slot: &mut f64,
        var_guard43_rdn4_slot: &mut f64,
        var_guard43_rdn5_slot: &mut f64,
        var_guard43_rdn6_slot: &mut f64,
        var_guard43_rdn7_slot: &mut f64,
        var_guard43_rdn8_slot: &mut f64,
        var_guard43_rdn9_slot: &mut f64,
        var_guard43_rv_slot: &mut f64,
        var_guard44_slot: &mut f64,
        var_guard44_db0_slot: &mut f64,
        var_guard44_db1_slot: &mut f64,
        var_guard44_db10_slot: &mut f64,
        var_guard44_db11_slot: &mut f64,
        var_guard44_db12_slot: &mut f64,
        var_guard44_db13_slot: &mut f64,
        var_guard44_db14_slot: &mut f64,
        var_guard44_db15_slot: &mut f64,
        var_guard44_db16_slot: &mut f64,
        var_guard44_db17_slot: &mut f64,
        var_guard44_db18_slot: &mut f64,
        var_guard44_db2_slot: &mut f64,
        var_guard44_db3_slot: &mut f64,
        var_guard44_db4_slot: &mut f64,
        var_guard44_db5_slot: &mut f64,
        var_guard44_db6_slot: &mut f64,
        var_guard44_db7_slot: &mut f64,
        var_guard44_db8_slot: &mut f64,
        var_guard44_db9_slot: &mut f64,
        var_guard44_dn0_slot: &mut f64,
        var_guard44_dn1_slot: &mut f64,
        var_guard44_dn10_slot: &mut f64,
        var_guard44_dn11_slot: &mut f64,
        var_guard44_dn12_slot: &mut f64,
        var_guard44_dn13_slot: &mut f64,
        var_guard44_dn14_slot: &mut f64,
        var_guard44_dn15_slot: &mut f64,
        var_guard44_dn16_slot: &mut f64,
        var_guard44_dn17_slot: &mut f64,
        var_guard44_dn18_slot: &mut f64,
        var_guard44_dn2_slot: &mut f64,
        var_guard44_dn3_slot: &mut f64,
        var_guard44_dn4_slot: &mut f64,
        var_guard44_dn5_slot: &mut f64,
        var_guard44_dn6_slot: &mut f64,
        var_guard44_dn7_slot: &mut f64,
        var_guard44_dn8_slot: &mut f64,
        var_guard44_dn9_slot: &mut f64,
        var_guard44_rdb0_slot: &mut f64,
        var_guard44_rdb1_slot: &mut f64,
        var_guard44_rdb10_slot: &mut f64,
        var_guard44_rdb11_slot: &mut f64,
        var_guard44_rdb12_slot: &mut f64,
        var_guard44_rdb13_slot: &mut f64,
        var_guard44_rdb14_slot: &mut f64,
        var_guard44_rdb15_slot: &mut f64,
        var_guard44_rdb16_slot: &mut f64,
        var_guard44_rdb17_slot: &mut f64,
        var_guard44_rdb18_slot: &mut f64,
        var_guard44_rdb2_slot: &mut f64,
        var_guard44_rdb3_slot: &mut f64,
        var_guard44_rdb4_slot: &mut f64,
        var_guard44_rdb5_slot: &mut f64,
        var_guard44_rdb6_slot: &mut f64,
        var_guard44_rdb7_slot: &mut f64,
        var_guard44_rdb8_slot: &mut f64,
        var_guard44_rdb9_slot: &mut f64,
        var_guard44_rdn0_slot: &mut f64,
        var_guard44_rdn1_slot: &mut f64,
        var_guard44_rdn10_slot: &mut f64,
        var_guard44_rdn11_slot: &mut f64,
        var_guard44_rdn12_slot: &mut f64,
        var_guard44_rdn13_slot: &mut f64,
        var_guard44_rdn14_slot: &mut f64,
        var_guard44_rdn15_slot: &mut f64,
        var_guard44_rdn16_slot: &mut f64,
        var_guard44_rdn17_slot: &mut f64,
        var_guard44_rdn18_slot: &mut f64,
        var_guard44_rdn2_slot: &mut f64,
        var_guard44_rdn3_slot: &mut f64,
        var_guard44_rdn4_slot: &mut f64,
        var_guard44_rdn5_slot: &mut f64,
        var_guard44_rdn6_slot: &mut f64,
        var_guard44_rdn7_slot: &mut f64,
        var_guard44_rdn8_slot: &mut f64,
        var_guard44_rdn9_slot: &mut f64,
        var_guard44_rv_slot: &mut f64,
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
        var_k_dn16_slot: &mut f64,
        var_k_dn17_slot: &mut f64,
        var_k_dn18_slot: &mut f64,
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
        var_k_rdn16_slot: &mut f64,
        var_k_rdn17_slot: &mut f64,
        var_k_rdn18_slot: &mut f64,
        var_k_rdn2_slot: &mut f64,
        var_k_rdn3_slot: &mut f64,
        var_k_rdn4_slot: &mut f64,
        var_k_rdn5_slot: &mut f64,
        var_k_rdn6_slot: &mut f64,
        var_k_rdn7_slot: &mut f64,
        var_k_rdn8_slot: &mut f64,
        var_k_rdn9_slot: &mut f64,
        var_k_rv_slot: &mut f64,
        var_noisepwr__blk41_slot: &mut f64,
        var_noisepwr__blk41_db0_slot: &mut f64,
        var_noisepwr__blk41_db1_slot: &mut f64,
        var_noisepwr__blk41_db10_slot: &mut f64,
        var_noisepwr__blk41_db11_slot: &mut f64,
        var_noisepwr__blk41_db12_slot: &mut f64,
        var_noisepwr__blk41_db13_slot: &mut f64,
        var_noisepwr__blk41_db14_slot: &mut f64,
        var_noisepwr__blk41_db15_slot: &mut f64,
        var_noisepwr__blk41_db16_slot: &mut f64,
        var_noisepwr__blk41_db17_slot: &mut f64,
        var_noisepwr__blk41_db18_slot: &mut f64,
        var_noisepwr__blk41_db2_slot: &mut f64,
        var_noisepwr__blk41_db3_slot: &mut f64,
        var_noisepwr__blk41_db4_slot: &mut f64,
        var_noisepwr__blk41_db5_slot: &mut f64,
        var_noisepwr__blk41_db6_slot: &mut f64,
        var_noisepwr__blk41_db7_slot: &mut f64,
        var_noisepwr__blk41_db8_slot: &mut f64,
        var_noisepwr__blk41_db9_slot: &mut f64,
        var_noisepwr__blk41_dn0_slot: &mut f64,
        var_noisepwr__blk41_dn1_slot: &mut f64,
        var_noisepwr__blk41_dn10_slot: &mut f64,
        var_noisepwr__blk41_dn11_slot: &mut f64,
        var_noisepwr__blk41_dn12_slot: &mut f64,
        var_noisepwr__blk41_dn13_slot: &mut f64,
        var_noisepwr__blk41_dn14_slot: &mut f64,
        var_noisepwr__blk41_dn15_slot: &mut f64,
        var_noisepwr__blk41_dn16_slot: &mut f64,
        var_noisepwr__blk41_dn17_slot: &mut f64,
        var_noisepwr__blk41_dn18_slot: &mut f64,
        var_noisepwr__blk41_dn2_slot: &mut f64,
        var_noisepwr__blk41_dn3_slot: &mut f64,
        var_noisepwr__blk41_dn4_slot: &mut f64,
        var_noisepwr__blk41_dn5_slot: &mut f64,
        var_noisepwr__blk41_dn6_slot: &mut f64,
        var_noisepwr__blk41_dn7_slot: &mut f64,
        var_noisepwr__blk41_dn8_slot: &mut f64,
        var_noisepwr__blk41_dn9_slot: &mut f64,
        var_noisepwr__blk41_rdb0_slot: &mut f64,
        var_noisepwr__blk41_rdb1_slot: &mut f64,
        var_noisepwr__blk41_rdb10_slot: &mut f64,
        var_noisepwr__blk41_rdb11_slot: &mut f64,
        var_noisepwr__blk41_rdb12_slot: &mut f64,
        var_noisepwr__blk41_rdb13_slot: &mut f64,
        var_noisepwr__blk41_rdb14_slot: &mut f64,
        var_noisepwr__blk41_rdb15_slot: &mut f64,
        var_noisepwr__blk41_rdb16_slot: &mut f64,
        var_noisepwr__blk41_rdb17_slot: &mut f64,
        var_noisepwr__blk41_rdb18_slot: &mut f64,
        var_noisepwr__blk41_rdb2_slot: &mut f64,
        var_noisepwr__blk41_rdb3_slot: &mut f64,
        var_noisepwr__blk41_rdb4_slot: &mut f64,
        var_noisepwr__blk41_rdb5_slot: &mut f64,
        var_noisepwr__blk41_rdb6_slot: &mut f64,
        var_noisepwr__blk41_rdb7_slot: &mut f64,
        var_noisepwr__blk41_rdb8_slot: &mut f64,
        var_noisepwr__blk41_rdb9_slot: &mut f64,
        var_noisepwr__blk41_rdn0_slot: &mut f64,
        var_noisepwr__blk41_rdn1_slot: &mut f64,
        var_noisepwr__blk41_rdn10_slot: &mut f64,
        var_noisepwr__blk41_rdn11_slot: &mut f64,
        var_noisepwr__blk41_rdn12_slot: &mut f64,
        var_noisepwr__blk41_rdn13_slot: &mut f64,
        var_noisepwr__blk41_rdn14_slot: &mut f64,
        var_noisepwr__blk41_rdn15_slot: &mut f64,
        var_noisepwr__blk41_rdn16_slot: &mut f64,
        var_noisepwr__blk41_rdn17_slot: &mut f64,
        var_noisepwr__blk41_rdn18_slot: &mut f64,
        var_noisepwr__blk41_rdn2_slot: &mut f64,
        var_noisepwr__blk41_rdn3_slot: &mut f64,
        var_noisepwr__blk41_rdn4_slot: &mut f64,
        var_noisepwr__blk41_rdn5_slot: &mut f64,
        var_noisepwr__blk41_rdn6_slot: &mut f64,
        var_noisepwr__blk41_rdn7_slot: &mut f64,
        var_noisepwr__blk41_rdn8_slot: &mut f64,
        var_noisepwr__blk41_rdn9_slot: &mut f64,
        var_noisepwr__blk41_rv_slot: &mut f64,
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
        let mut var_ci_dn16: f64 = *var_ci_dn16_slot;
        let mut var_ci_dn17: f64 = *var_ci_dn17_slot;
        let mut var_ci_dn18: f64 = *var_ci_dn18_slot;
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
        let mut var_ci_rdn16: f64 = *var_ci_rdn16_slot;
        let mut var_ci_rdn17: f64 = *var_ci_rdn17_slot;
        let mut var_ci_rdn18: f64 = *var_ci_rdn18_slot;
        let mut var_ci_rdn2: f64 = *var_ci_rdn2_slot;
        let mut var_ci_rdn3: f64 = *var_ci_rdn3_slot;
        let mut var_ci_rdn4: f64 = *var_ci_rdn4_slot;
        let mut var_ci_rdn5: f64 = *var_ci_rdn5_slot;
        let mut var_ci_rdn6: f64 = *var_ci_rdn6_slot;
        let mut var_ci_rdn7: f64 = *var_ci_rdn7_slot;
        let mut var_ci_rdn8: f64 = *var_ci_rdn8_slot;
        let mut var_ci_rdn9: f64 = *var_ci_rdn9_slot;
        let mut var_ci_rv: f64 = *var_ci_rv_slot;
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
        let mut var_guard43_dn16: f64 = *var_guard43_dn16_slot;
        let mut var_guard43_dn17: f64 = *var_guard43_dn17_slot;
        let mut var_guard43_dn18: f64 = *var_guard43_dn18_slot;
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
        let mut var_guard43_rdn16: f64 = *var_guard43_rdn16_slot;
        let mut var_guard43_rdn17: f64 = *var_guard43_rdn17_slot;
        let mut var_guard43_rdn18: f64 = *var_guard43_rdn18_slot;
        let mut var_guard43_rdn2: f64 = *var_guard43_rdn2_slot;
        let mut var_guard43_rdn3: f64 = *var_guard43_rdn3_slot;
        let mut var_guard43_rdn4: f64 = *var_guard43_rdn4_slot;
        let mut var_guard43_rdn5: f64 = *var_guard43_rdn5_slot;
        let mut var_guard43_rdn6: f64 = *var_guard43_rdn6_slot;
        let mut var_guard43_rdn7: f64 = *var_guard43_rdn7_slot;
        let mut var_guard43_rdn8: f64 = *var_guard43_rdn8_slot;
        let mut var_guard43_rdn9: f64 = *var_guard43_rdn9_slot;
        let mut var_guard43_rv: f64 = *var_guard43_rv_slot;
        let mut var_guard44: f64 = *var_guard44_slot;
        let mut var_guard44_db0: f64 = *var_guard44_db0_slot;
        let mut var_guard44_db1: f64 = *var_guard44_db1_slot;
        let mut var_guard44_db10: f64 = *var_guard44_db10_slot;
        let mut var_guard44_db11: f64 = *var_guard44_db11_slot;
        let mut var_guard44_db12: f64 = *var_guard44_db12_slot;
        let mut var_guard44_db13: f64 = *var_guard44_db13_slot;
        let mut var_guard44_db14: f64 = *var_guard44_db14_slot;
        let mut var_guard44_db15: f64 = *var_guard44_db15_slot;
        let mut var_guard44_db16: f64 = *var_guard44_db16_slot;
        let mut var_guard44_db17: f64 = *var_guard44_db17_slot;
        let mut var_guard44_db18: f64 = *var_guard44_db18_slot;
        let mut var_guard44_db2: f64 = *var_guard44_db2_slot;
        let mut var_guard44_db3: f64 = *var_guard44_db3_slot;
        let mut var_guard44_db4: f64 = *var_guard44_db4_slot;
        let mut var_guard44_db5: f64 = *var_guard44_db5_slot;
        let mut var_guard44_db6: f64 = *var_guard44_db6_slot;
        let mut var_guard44_db7: f64 = *var_guard44_db7_slot;
        let mut var_guard44_db8: f64 = *var_guard44_db8_slot;
        let mut var_guard44_db9: f64 = *var_guard44_db9_slot;
        let mut var_guard44_dn0: f64 = *var_guard44_dn0_slot;
        let mut var_guard44_dn1: f64 = *var_guard44_dn1_slot;
        let mut var_guard44_dn10: f64 = *var_guard44_dn10_slot;
        let mut var_guard44_dn11: f64 = *var_guard44_dn11_slot;
        let mut var_guard44_dn12: f64 = *var_guard44_dn12_slot;
        let mut var_guard44_dn13: f64 = *var_guard44_dn13_slot;
        let mut var_guard44_dn14: f64 = *var_guard44_dn14_slot;
        let mut var_guard44_dn15: f64 = *var_guard44_dn15_slot;
        let mut var_guard44_dn16: f64 = *var_guard44_dn16_slot;
        let mut var_guard44_dn17: f64 = *var_guard44_dn17_slot;
        let mut var_guard44_dn18: f64 = *var_guard44_dn18_slot;
        let mut var_guard44_dn2: f64 = *var_guard44_dn2_slot;
        let mut var_guard44_dn3: f64 = *var_guard44_dn3_slot;
        let mut var_guard44_dn4: f64 = *var_guard44_dn4_slot;
        let mut var_guard44_dn5: f64 = *var_guard44_dn5_slot;
        let mut var_guard44_dn6: f64 = *var_guard44_dn6_slot;
        let mut var_guard44_dn7: f64 = *var_guard44_dn7_slot;
        let mut var_guard44_dn8: f64 = *var_guard44_dn8_slot;
        let mut var_guard44_dn9: f64 = *var_guard44_dn9_slot;
        let mut var_guard44_rdb0: f64 = *var_guard44_rdb0_slot;
        let mut var_guard44_rdb1: f64 = *var_guard44_rdb1_slot;
        let mut var_guard44_rdb10: f64 = *var_guard44_rdb10_slot;
        let mut var_guard44_rdb11: f64 = *var_guard44_rdb11_slot;
        let mut var_guard44_rdb12: f64 = *var_guard44_rdb12_slot;
        let mut var_guard44_rdb13: f64 = *var_guard44_rdb13_slot;
        let mut var_guard44_rdb14: f64 = *var_guard44_rdb14_slot;
        let mut var_guard44_rdb15: f64 = *var_guard44_rdb15_slot;
        let mut var_guard44_rdb16: f64 = *var_guard44_rdb16_slot;
        let mut var_guard44_rdb17: f64 = *var_guard44_rdb17_slot;
        let mut var_guard44_rdb18: f64 = *var_guard44_rdb18_slot;
        let mut var_guard44_rdb2: f64 = *var_guard44_rdb2_slot;
        let mut var_guard44_rdb3: f64 = *var_guard44_rdb3_slot;
        let mut var_guard44_rdb4: f64 = *var_guard44_rdb4_slot;
        let mut var_guard44_rdb5: f64 = *var_guard44_rdb5_slot;
        let mut var_guard44_rdb6: f64 = *var_guard44_rdb6_slot;
        let mut var_guard44_rdb7: f64 = *var_guard44_rdb7_slot;
        let mut var_guard44_rdb8: f64 = *var_guard44_rdb8_slot;
        let mut var_guard44_rdb9: f64 = *var_guard44_rdb9_slot;
        let mut var_guard44_rdn0: f64 = *var_guard44_rdn0_slot;
        let mut var_guard44_rdn1: f64 = *var_guard44_rdn1_slot;
        let mut var_guard44_rdn10: f64 = *var_guard44_rdn10_slot;
        let mut var_guard44_rdn11: f64 = *var_guard44_rdn11_slot;
        let mut var_guard44_rdn12: f64 = *var_guard44_rdn12_slot;
        let mut var_guard44_rdn13: f64 = *var_guard44_rdn13_slot;
        let mut var_guard44_rdn14: f64 = *var_guard44_rdn14_slot;
        let mut var_guard44_rdn15: f64 = *var_guard44_rdn15_slot;
        let mut var_guard44_rdn16: f64 = *var_guard44_rdn16_slot;
        let mut var_guard44_rdn17: f64 = *var_guard44_rdn17_slot;
        let mut var_guard44_rdn18: f64 = *var_guard44_rdn18_slot;
        let mut var_guard44_rdn2: f64 = *var_guard44_rdn2_slot;
        let mut var_guard44_rdn3: f64 = *var_guard44_rdn3_slot;
        let mut var_guard44_rdn4: f64 = *var_guard44_rdn4_slot;
        let mut var_guard44_rdn5: f64 = *var_guard44_rdn5_slot;
        let mut var_guard44_rdn6: f64 = *var_guard44_rdn6_slot;
        let mut var_guard44_rdn7: f64 = *var_guard44_rdn7_slot;
        let mut var_guard44_rdn8: f64 = *var_guard44_rdn8_slot;
        let mut var_guard44_rdn9: f64 = *var_guard44_rdn9_slot;
        let mut var_guard44_rv: f64 = *var_guard44_rv_slot;
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
        let mut var_k_dn16: f64 = *var_k_dn16_slot;
        let mut var_k_dn17: f64 = *var_k_dn17_slot;
        let mut var_k_dn18: f64 = *var_k_dn18_slot;
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
        let mut var_k_rdn16: f64 = *var_k_rdn16_slot;
        let mut var_k_rdn17: f64 = *var_k_rdn17_slot;
        let mut var_k_rdn18: f64 = *var_k_rdn18_slot;
        let mut var_k_rdn2: f64 = *var_k_rdn2_slot;
        let mut var_k_rdn3: f64 = *var_k_rdn3_slot;
        let mut var_k_rdn4: f64 = *var_k_rdn4_slot;
        let mut var_k_rdn5: f64 = *var_k_rdn5_slot;
        let mut var_k_rdn6: f64 = *var_k_rdn6_slot;
        let mut var_k_rdn7: f64 = *var_k_rdn7_slot;
        let mut var_k_rdn8: f64 = *var_k_rdn8_slot;
        let mut var_k_rdn9: f64 = *var_k_rdn9_slot;
        let mut var_k_rv: f64 = *var_k_rv_slot;
        let mut var_noisepwr__blk41: f64 = *var_noisepwr__blk41_slot;
        let mut var_noisepwr__blk41_db0: f64 = *var_noisepwr__blk41_db0_slot;
        let mut var_noisepwr__blk41_db1: f64 = *var_noisepwr__blk41_db1_slot;
        let mut var_noisepwr__blk41_db10: f64 = *var_noisepwr__blk41_db10_slot;
        let mut var_noisepwr__blk41_db11: f64 = *var_noisepwr__blk41_db11_slot;
        let mut var_noisepwr__blk41_db12: f64 = *var_noisepwr__blk41_db12_slot;
        let mut var_noisepwr__blk41_db13: f64 = *var_noisepwr__blk41_db13_slot;
        let mut var_noisepwr__blk41_db14: f64 = *var_noisepwr__blk41_db14_slot;
        let mut var_noisepwr__blk41_db15: f64 = *var_noisepwr__blk41_db15_slot;
        let mut var_noisepwr__blk41_db16: f64 = *var_noisepwr__blk41_db16_slot;
        let mut var_noisepwr__blk41_db17: f64 = *var_noisepwr__blk41_db17_slot;
        let mut var_noisepwr__blk41_db18: f64 = *var_noisepwr__blk41_db18_slot;
        let mut var_noisepwr__blk41_db2: f64 = *var_noisepwr__blk41_db2_slot;
        let mut var_noisepwr__blk41_db3: f64 = *var_noisepwr__blk41_db3_slot;
        let mut var_noisepwr__blk41_db4: f64 = *var_noisepwr__blk41_db4_slot;
        let mut var_noisepwr__blk41_db5: f64 = *var_noisepwr__blk41_db5_slot;
        let mut var_noisepwr__blk41_db6: f64 = *var_noisepwr__blk41_db6_slot;
        let mut var_noisepwr__blk41_db7: f64 = *var_noisepwr__blk41_db7_slot;
        let mut var_noisepwr__blk41_db8: f64 = *var_noisepwr__blk41_db8_slot;
        let mut var_noisepwr__blk41_db9: f64 = *var_noisepwr__blk41_db9_slot;
        let mut var_noisepwr__blk41_dn0: f64 = *var_noisepwr__blk41_dn0_slot;
        let mut var_noisepwr__blk41_dn1: f64 = *var_noisepwr__blk41_dn1_slot;
        let mut var_noisepwr__blk41_dn10: f64 = *var_noisepwr__blk41_dn10_slot;
        let mut var_noisepwr__blk41_dn11: f64 = *var_noisepwr__blk41_dn11_slot;
        let mut var_noisepwr__blk41_dn12: f64 = *var_noisepwr__blk41_dn12_slot;
        let mut var_noisepwr__blk41_dn13: f64 = *var_noisepwr__blk41_dn13_slot;
        let mut var_noisepwr__blk41_dn14: f64 = *var_noisepwr__blk41_dn14_slot;
        let mut var_noisepwr__blk41_dn15: f64 = *var_noisepwr__blk41_dn15_slot;
        let mut var_noisepwr__blk41_dn16: f64 = *var_noisepwr__blk41_dn16_slot;
        let mut var_noisepwr__blk41_dn17: f64 = *var_noisepwr__blk41_dn17_slot;
        let mut var_noisepwr__blk41_dn18: f64 = *var_noisepwr__blk41_dn18_slot;
        let mut var_noisepwr__blk41_dn2: f64 = *var_noisepwr__blk41_dn2_slot;
        let mut var_noisepwr__blk41_dn3: f64 = *var_noisepwr__blk41_dn3_slot;
        let mut var_noisepwr__blk41_dn4: f64 = *var_noisepwr__blk41_dn4_slot;
        let mut var_noisepwr__blk41_dn5: f64 = *var_noisepwr__blk41_dn5_slot;
        let mut var_noisepwr__blk41_dn6: f64 = *var_noisepwr__blk41_dn6_slot;
        let mut var_noisepwr__blk41_dn7: f64 = *var_noisepwr__blk41_dn7_slot;
        let mut var_noisepwr__blk41_dn8: f64 = *var_noisepwr__blk41_dn8_slot;
        let mut var_noisepwr__blk41_dn9: f64 = *var_noisepwr__blk41_dn9_slot;
        let mut var_noisepwr__blk41_rdb0: f64 = *var_noisepwr__blk41_rdb0_slot;
        let mut var_noisepwr__blk41_rdb1: f64 = *var_noisepwr__blk41_rdb1_slot;
        let mut var_noisepwr__blk41_rdb10: f64 = *var_noisepwr__blk41_rdb10_slot;
        let mut var_noisepwr__blk41_rdb11: f64 = *var_noisepwr__blk41_rdb11_slot;
        let mut var_noisepwr__blk41_rdb12: f64 = *var_noisepwr__blk41_rdb12_slot;
        let mut var_noisepwr__blk41_rdb13: f64 = *var_noisepwr__blk41_rdb13_slot;
        let mut var_noisepwr__blk41_rdb14: f64 = *var_noisepwr__blk41_rdb14_slot;
        let mut var_noisepwr__blk41_rdb15: f64 = *var_noisepwr__blk41_rdb15_slot;
        let mut var_noisepwr__blk41_rdb16: f64 = *var_noisepwr__blk41_rdb16_slot;
        let mut var_noisepwr__blk41_rdb17: f64 = *var_noisepwr__blk41_rdb17_slot;
        let mut var_noisepwr__blk41_rdb18: f64 = *var_noisepwr__blk41_rdb18_slot;
        let mut var_noisepwr__blk41_rdb2: f64 = *var_noisepwr__blk41_rdb2_slot;
        let mut var_noisepwr__blk41_rdb3: f64 = *var_noisepwr__blk41_rdb3_slot;
        let mut var_noisepwr__blk41_rdb4: f64 = *var_noisepwr__blk41_rdb4_slot;
        let mut var_noisepwr__blk41_rdb5: f64 = *var_noisepwr__blk41_rdb5_slot;
        let mut var_noisepwr__blk41_rdb6: f64 = *var_noisepwr__blk41_rdb6_slot;
        let mut var_noisepwr__blk41_rdb7: f64 = *var_noisepwr__blk41_rdb7_slot;
        let mut var_noisepwr__blk41_rdb8: f64 = *var_noisepwr__blk41_rdb8_slot;
        let mut var_noisepwr__blk41_rdb9: f64 = *var_noisepwr__blk41_rdb9_slot;
        let mut var_noisepwr__blk41_rdn0: f64 = *var_noisepwr__blk41_rdn0_slot;
        let mut var_noisepwr__blk41_rdn1: f64 = *var_noisepwr__blk41_rdn1_slot;
        let mut var_noisepwr__blk41_rdn10: f64 = *var_noisepwr__blk41_rdn10_slot;
        let mut var_noisepwr__blk41_rdn11: f64 = *var_noisepwr__blk41_rdn11_slot;
        let mut var_noisepwr__blk41_rdn12: f64 = *var_noisepwr__blk41_rdn12_slot;
        let mut var_noisepwr__blk41_rdn13: f64 = *var_noisepwr__blk41_rdn13_slot;
        let mut var_noisepwr__blk41_rdn14: f64 = *var_noisepwr__blk41_rdn14_slot;
        let mut var_noisepwr__blk41_rdn15: f64 = *var_noisepwr__blk41_rdn15_slot;
        let mut var_noisepwr__blk41_rdn16: f64 = *var_noisepwr__blk41_rdn16_slot;
        let mut var_noisepwr__blk41_rdn17: f64 = *var_noisepwr__blk41_rdn17_slot;
        let mut var_noisepwr__blk41_rdn18: f64 = *var_noisepwr__blk41_rdn18_slot;
        let mut var_noisepwr__blk41_rdn2: f64 = *var_noisepwr__blk41_rdn2_slot;
        let mut var_noisepwr__blk41_rdn3: f64 = *var_noisepwr__blk41_rdn3_slot;
        let mut var_noisepwr__blk41_rdn4: f64 = *var_noisepwr__blk41_rdn4_slot;
        let mut var_noisepwr__blk41_rdn5: f64 = *var_noisepwr__blk41_rdn5_slot;
        let mut var_noisepwr__blk41_rdn6: f64 = *var_noisepwr__blk41_rdn6_slot;
        let mut var_noisepwr__blk41_rdn7: f64 = *var_noisepwr__blk41_rdn7_slot;
        let mut var_noisepwr__blk41_rdn8: f64 = *var_noisepwr__blk41_rdn8_slot;
        let mut var_noisepwr__blk41_rdn9: f64 = *var_noisepwr__blk41_rdn9_slot;
        let mut var_noisepwr__blk41_rv: f64 = *var_noisepwr__blk41_rv_slot;

        let (assign2310_e3018, assign2310_e3018_d_n0, assign2310_e3018_d_n1, assign2310_e3018_d_n2, assign2310_e3018_d_n3, assign2310_e3018_d_n4, assign2310_e3018_d_n5, assign2310_e3018_d_n6, assign2310_e3018_d_n7, assign2310_e3018_d_n8, assign2310_e3018_d_n9, assign2310_e3018_d_n10, assign2310_e3018_d_n11, assign2310_e3018_d_n12, assign2310_e3018_d_n13, assign2310_e3018_d_n14, assign2310_e3018_d_n15, assign2310_e3018_d_n16, assign2310_e3018_d_n17, assign2310_e3018_d_n18, assign2310_e3018_d_b0, assign2310_e3018_d_b1, assign2310_e3018_d_b2, assign2310_e3018_d_b3, assign2310_e3018_d_b4, assign2310_e3018_d_b5, assign2310_e3018_d_b6, assign2310_e3018_d_b7, assign2310_e3018_d_b8, assign2310_e3018_d_b9, assign2310_e3018_d_b10, assign2310_e3018_d_b11, assign2310_e3018_d_b12, assign2310_e3018_d_b13, assign2310_e3018_d_b14, assign2310_e3018_d_b15, assign2310_e3018_d_b16, assign2310_e3018_d_b17, assign2310_e3018_d_b18,) = {
    if (((var_guard29 != 0.0) && (var_guard28 == 0.0)) && (p.p0 != 0.0)) {
        let assign2310_e3005: f64 = (4.0 * 1.3806503e-23);
        let assign2310_e3007: f64 = (assign2310_e3005 * var_t);
        let assign2310_e3009: f64 = (assign2310_e3007 * p.p88);
        let assign2310_e3011: f64 = (assign2310_e3009 * var_cgs0_t);
        let assign2310_e3014: f64 = (p.p87 * p.p86);
        let assign2310_e3015: f64 = (assign2310_e3014).sqrt();
        let assign2310_e3016: f64 = (assign2310_e3011 * assign2310_e3015);
        (assign2310_e3016, (((((assign2310_e3005 * var_t_dn0) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn0)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn1) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn1)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn2) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn2)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn3) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn3)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn4) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn4)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn5) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn5)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn6) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn6)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn7) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn7)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn8) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn8)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn9) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn9)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn10) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn10)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn11) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn11)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn12) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn12)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn13) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn13)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn14) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn14)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn15) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn15)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn16) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn16)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn17) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn17)) * assign2310_e3015), (((((assign2310_e3005 * var_t_dn18) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_dn18)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db0) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db0)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db1) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db1)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db2) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db2)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db3) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db3)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db4) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db4)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db5) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db5)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db6) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db6)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db7) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db7)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db8) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db8)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db9) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db9)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db10) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db10)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db11) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db11)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db12) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db12)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db13) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db13)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db14) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db14)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db15) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db15)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db16) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db16)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db17) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db17)) * assign2310_e3015), (((((assign2310_e3005 * var_t_db18) * p.p88) * var_cgs0_t) + (assign2310_e3009 * var_cgs0_t_db18)) * assign2310_e3015),)
    } else {
        (var_k, var_k_dn0, var_k_dn1, var_k_dn2, var_k_dn3, var_k_dn4, var_k_dn5, var_k_dn6, var_k_dn7, var_k_dn8, var_k_dn9, var_k_dn10, var_k_dn11, var_k_dn12, var_k_dn13, var_k_dn14, var_k_dn15, var_k_dn16, var_k_dn17, var_k_dn18, var_k_db0, var_k_db1, var_k_db2, var_k_db3, var_k_db4, var_k_db5, var_k_db6, var_k_db7, var_k_db8, var_k_db9, var_k_db10, var_k_db11, var_k_db12, var_k_db13, var_k_db14, var_k_db15, var_k_db16, var_k_db17, var_k_db18,)
    }
};
        var_k = assign2310_e3018;
        var_k_dn0 = assign2310_e3018_d_n0;
        var_k_dn1 = assign2310_e3018_d_n1;
        var_k_dn2 = assign2310_e3018_d_n2;
        var_k_dn3 = assign2310_e3018_d_n3;
        var_k_dn4 = assign2310_e3018_d_n4;
        var_k_dn5 = assign2310_e3018_d_n5;
        var_k_dn6 = assign2310_e3018_d_n6;
        var_k_dn7 = assign2310_e3018_d_n7;
        var_k_dn8 = assign2310_e3018_d_n8;
        var_k_dn9 = assign2310_e3018_d_n9;
        var_k_dn10 = assign2310_e3018_d_n10;
        var_k_dn11 = assign2310_e3018_d_n11;
        var_k_dn12 = assign2310_e3018_d_n12;
        var_k_dn13 = assign2310_e3018_d_n13;
        var_k_dn14 = assign2310_e3018_d_n14;
        var_k_dn15 = assign2310_e3018_d_n15;
        var_k_dn16 = assign2310_e3018_d_n16;
        var_k_dn17 = assign2310_e3018_d_n17;
        var_k_dn18 = assign2310_e3018_d_n18;
        var_k_db0 = assign2310_e3018_d_b0;
        var_k_db1 = assign2310_e3018_d_b1;
        var_k_db2 = assign2310_e3018_d_b2;
        var_k_db3 = assign2310_e3018_d_b3;
        var_k_db4 = assign2310_e3018_d_b4;
        var_k_db5 = assign2310_e3018_d_b5;
        var_k_db6 = assign2310_e3018_d_b6;
        var_k_db7 = assign2310_e3018_d_b7;
        var_k_db8 = assign2310_e3018_d_b8;
        var_k_db9 = assign2310_e3018_d_b9;
        var_k_db10 = assign2310_e3018_d_b10;
        var_k_db11 = assign2310_e3018_d_b11;
        var_k_db12 = assign2310_e3018_d_b12;
        var_k_db13 = assign2310_e3018_d_b13;
        var_k_db14 = assign2310_e3018_d_b14;
        var_k_db15 = assign2310_e3018_d_b15;
        var_k_db16 = assign2310_e3018_d_b16;
        var_k_db17 = assign2310_e3018_d_b17;
        var_k_db18 = assign2310_e3018_d_b18;
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
        var_k_rdn16 = 0.0;
        var_k_rdn17 = 0.0;
        var_k_rdn18 = 0.0;
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

        let (assign2340_e3055, assign2340_e3055_d_n0, assign2340_e3055_d_n1, assign2340_e3055_d_n2, assign2340_e3055_d_n3, assign2340_e3055_d_n4, assign2340_e3055_d_n5, assign2340_e3055_d_n6, assign2340_e3055_d_n7, assign2340_e3055_d_n8, assign2340_e3055_d_n9, assign2340_e3055_d_n10, assign2340_e3055_d_n11, assign2340_e3055_d_n12, assign2340_e3055_d_n13, assign2340_e3055_d_n14, assign2340_e3055_d_n15, assign2340_e3055_d_n16, assign2340_e3055_d_n17, assign2340_e3055_d_n18, assign2340_e3055_d_b0, assign2340_e3055_d_b1, assign2340_e3055_d_b2, assign2340_e3055_d_b3, assign2340_e3055_d_b4, assign2340_e3055_d_b5, assign2340_e3055_d_b6, assign2340_e3055_d_b7, assign2340_e3055_d_b8, assign2340_e3055_d_b9, assign2340_e3055_d_b10, assign2340_e3055_d_b11, assign2340_e3055_d_b12, assign2340_e3055_d_b13, assign2340_e3055_d_b14, assign2340_e3055_d_b15, assign2340_e3055_d_b16, assign2340_e3055_d_b17, assign2340_e3055_d_b18,) = {
    if (((var_guard29 != 0.0) && (var_guard28 == 0.0)) && (p.p0 != 0.0)) {
        let assign2340_e3053: f64 = (var_k * 3.141592653589793);
        (assign2340_e3053, (var_k_dn0 * 3.141592653589793), (var_k_dn1 * 3.141592653589793), (var_k_dn2 * 3.141592653589793), (var_k_dn3 * 3.141592653589793), (var_k_dn4 * 3.141592653589793), (var_k_dn5 * 3.141592653589793), (var_k_dn6 * 3.141592653589793), (var_k_dn7 * 3.141592653589793), (var_k_dn8 * 3.141592653589793), (var_k_dn9 * 3.141592653589793), (var_k_dn10 * 3.141592653589793), (var_k_dn11 * 3.141592653589793), (var_k_dn12 * 3.141592653589793), (var_k_dn13 * 3.141592653589793), (var_k_dn14 * 3.141592653589793), (var_k_dn15 * 3.141592653589793), (var_k_dn16 * 3.141592653589793), (var_k_dn17 * 3.141592653589793), (var_k_dn18 * 3.141592653589793), (var_k_db0 * 3.141592653589793), (var_k_db1 * 3.141592653589793), (var_k_db2 * 3.141592653589793), (var_k_db3 * 3.141592653589793), (var_k_db4 * 3.141592653589793), (var_k_db5 * 3.141592653589793), (var_k_db6 * 3.141592653589793), (var_k_db7 * 3.141592653589793), (var_k_db8 * 3.141592653589793), (var_k_db9 * 3.141592653589793), (var_k_db10 * 3.141592653589793), (var_k_db11 * 3.141592653589793), (var_k_db12 * 3.141592653589793), (var_k_db13 * 3.141592653589793), (var_k_db14 * 3.141592653589793), (var_k_db15 * 3.141592653589793), (var_k_db16 * 3.141592653589793), (var_k_db17 * 3.141592653589793), (var_k_db18 * 3.141592653589793),)
    } else {
        (var_ci, var_ci_dn0, var_ci_dn1, var_ci_dn2, var_ci_dn3, var_ci_dn4, var_ci_dn5, var_ci_dn6, var_ci_dn7, var_ci_dn8, var_ci_dn9, var_ci_dn10, var_ci_dn11, var_ci_dn12, var_ci_dn13, var_ci_dn14, var_ci_dn15, var_ci_dn16, var_ci_dn17, var_ci_dn18, var_ci_db0, var_ci_db1, var_ci_db2, var_ci_db3, var_ci_db4, var_ci_db5, var_ci_db6, var_ci_db7, var_ci_db8, var_ci_db9, var_ci_db10, var_ci_db11, var_ci_db12, var_ci_db13, var_ci_db14, var_ci_db15, var_ci_db16, var_ci_db17, var_ci_db18,)
    }
};
        var_ci = assign2340_e3055;
        var_ci_dn0 = assign2340_e3055_d_n0;
        var_ci_dn1 = assign2340_e3055_d_n1;
        var_ci_dn2 = assign2340_e3055_d_n2;
        var_ci_dn3 = assign2340_e3055_d_n3;
        var_ci_dn4 = assign2340_e3055_d_n4;
        var_ci_dn5 = assign2340_e3055_d_n5;
        var_ci_dn6 = assign2340_e3055_d_n6;
        var_ci_dn7 = assign2340_e3055_d_n7;
        var_ci_dn8 = assign2340_e3055_d_n8;
        var_ci_dn9 = assign2340_e3055_d_n9;
        var_ci_dn10 = assign2340_e3055_d_n10;
        var_ci_dn11 = assign2340_e3055_d_n11;
        var_ci_dn12 = assign2340_e3055_d_n12;
        var_ci_dn13 = assign2340_e3055_d_n13;
        var_ci_dn14 = assign2340_e3055_d_n14;
        var_ci_dn15 = assign2340_e3055_d_n15;
        var_ci_dn16 = assign2340_e3055_d_n16;
        var_ci_dn17 = assign2340_e3055_d_n17;
        var_ci_dn18 = assign2340_e3055_d_n18;
        var_ci_db0 = assign2340_e3055_d_b0;
        var_ci_db1 = assign2340_e3055_d_b1;
        var_ci_db2 = assign2340_e3055_d_b2;
        var_ci_db3 = assign2340_e3055_d_b3;
        var_ci_db4 = assign2340_e3055_d_b4;
        var_ci_db5 = assign2340_e3055_d_b5;
        var_ci_db6 = assign2340_e3055_d_b6;
        var_ci_db7 = assign2340_e3055_d_b7;
        var_ci_db8 = assign2340_e3055_d_b8;
        var_ci_db9 = assign2340_e3055_d_b9;
        var_ci_db10 = assign2340_e3055_d_b10;
        var_ci_db11 = assign2340_e3055_d_b11;
        var_ci_db12 = assign2340_e3055_d_b12;
        var_ci_db13 = assign2340_e3055_d_b13;
        var_ci_db14 = assign2340_e3055_d_b14;
        var_ci_db15 = assign2340_e3055_d_b15;
        var_ci_db16 = assign2340_e3055_d_b16;
        var_ci_db17 = assign2340_e3055_d_b17;
        var_ci_db18 = assign2340_e3055_d_b18;
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
        var_ci_rdn16 = 0.0;
        var_ci_rdn17 = 0.0;
        var_ci_rdn18 = 0.0;
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

        let (assign2350_e3074, assign2350_e3074_d_n0, assign2350_e3074_d_n1, assign2350_e3074_d_n2, assign2350_e3074_d_n3, assign2350_e3074_d_n4, assign2350_e3074_d_n5, assign2350_e3074_d_n6, assign2350_e3074_d_n7, assign2350_e3074_d_n8, assign2350_e3074_d_n9, assign2350_e3074_d_n10, assign2350_e3074_d_n11, assign2350_e3074_d_n12, assign2350_e3074_d_n13, assign2350_e3074_d_n14, assign2350_e3074_d_n15, assign2350_e3074_d_n16, assign2350_e3074_d_n17, assign2350_e3074_d_n18, assign2350_e3074_d_b0, assign2350_e3074_d_b1, assign2350_e3074_d_b2, assign2350_e3074_d_b3, assign2350_e3074_d_b4, assign2350_e3074_d_b5, assign2350_e3074_d_b6, assign2350_e3074_d_b7, assign2350_e3074_d_b8, assign2350_e3074_d_b9, assign2350_e3074_d_b10, assign2350_e3074_d_b11, assign2350_e3074_d_b12, assign2350_e3074_d_b13, assign2350_e3074_d_b14, assign2350_e3074_d_b15, assign2350_e3074_d_b16, assign2350_e3074_d_b17, assign2350_e3074_d_b18,) = {
    if (((var_guard29 != 0.0) && (var_guard28 == 0.0)) && (p.p0 != 0.0)) {
        let assign2350_e3064: f64 = (4.0 * 1.3806503e-23);
        let assign2350_e3066: f64 = (assign2350_e3064 * var_t);
        let assign2350_e3068: f64 = (assign2350_e3066 * var_gm);
        let assign2350_e3070: f64 = (assign2350_e3068 * p.p87);
        let assign2350_e3072: f64 = (assign2350_e3070 * p.p89);
        (assign2350_e3072, (((((assign2350_e3064 * var_t_dn0) * var_gm) + (assign2350_e3066 * var_gm_dn0)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn1) * var_gm) + (assign2350_e3066 * var_gm_dn1)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn2) * var_gm) + (assign2350_e3066 * var_gm_dn2)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn3) * var_gm) + (assign2350_e3066 * var_gm_dn3)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn4) * var_gm) + (assign2350_e3066 * var_gm_dn4)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn5) * var_gm) + (assign2350_e3066 * var_gm_dn5)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn6) * var_gm) + (assign2350_e3066 * var_gm_dn6)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn7) * var_gm) + (assign2350_e3066 * var_gm_dn7)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn8) * var_gm) + (assign2350_e3066 * var_gm_dn8)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn9) * var_gm) + (assign2350_e3066 * var_gm_dn9)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn10) * var_gm) + (assign2350_e3066 * var_gm_dn10)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn11) * var_gm) + (assign2350_e3066 * var_gm_dn11)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn12) * var_gm) + (assign2350_e3066 * var_gm_dn12)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn13) * var_gm) + (assign2350_e3066 * var_gm_dn13)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn14) * var_gm) + (assign2350_e3066 * var_gm_dn14)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn15) * var_gm) + (assign2350_e3066 * var_gm_dn15)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn16) * var_gm) + (assign2350_e3066 * var_gm_dn16)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn17) * var_gm) + (assign2350_e3066 * var_gm_dn17)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_dn18) * var_gm) + (assign2350_e3066 * var_gm_dn18)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db0) * var_gm) + (assign2350_e3066 * var_gm_db0)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db1) * var_gm) + (assign2350_e3066 * var_gm_db1)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db2) * var_gm) + (assign2350_e3066 * var_gm_db2)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db3) * var_gm) + (assign2350_e3066 * var_gm_db3)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db4) * var_gm) + (assign2350_e3066 * var_gm_db4)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db5) * var_gm) + (assign2350_e3066 * var_gm_db5)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db6) * var_gm) + (assign2350_e3066 * var_gm_db6)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db7) * var_gm) + (assign2350_e3066 * var_gm_db7)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db8) * var_gm) + (assign2350_e3066 * var_gm_db8)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db9) * var_gm) + (assign2350_e3066 * var_gm_db9)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db10) * var_gm) + (assign2350_e3066 * var_gm_db10)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db11) * var_gm) + (assign2350_e3066 * var_gm_db11)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db12) * var_gm) + (assign2350_e3066 * var_gm_db12)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db13) * var_gm) + (assign2350_e3066 * var_gm_db13)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db14) * var_gm) + (assign2350_e3066 * var_gm_db14)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db15) * var_gm) + (assign2350_e3066 * var_gm_db15)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db16) * var_gm) + (assign2350_e3066 * var_gm_db16)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db17) * var_gm) + (assign2350_e3066 * var_gm_db17)) * p.p87) * p.p89), (((((assign2350_e3064 * var_t_db18) * var_gm) + (assign2350_e3066 * var_gm_db18)) * p.p87) * p.p89),)
    } else {
        (var_noisepwr__blk41, var_noisepwr__blk41_dn0, var_noisepwr__blk41_dn1, var_noisepwr__blk41_dn2, var_noisepwr__blk41_dn3, var_noisepwr__blk41_dn4, var_noisepwr__blk41_dn5, var_noisepwr__blk41_dn6, var_noisepwr__blk41_dn7, var_noisepwr__blk41_dn8, var_noisepwr__blk41_dn9, var_noisepwr__blk41_dn10, var_noisepwr__blk41_dn11, var_noisepwr__blk41_dn12, var_noisepwr__blk41_dn13, var_noisepwr__blk41_dn14, var_noisepwr__blk41_dn15, var_noisepwr__blk41_dn16, var_noisepwr__blk41_dn17, var_noisepwr__blk41_dn18, var_noisepwr__blk41_db0, var_noisepwr__blk41_db1, var_noisepwr__blk41_db2, var_noisepwr__blk41_db3, var_noisepwr__blk41_db4, var_noisepwr__blk41_db5, var_noisepwr__blk41_db6, var_noisepwr__blk41_db7, var_noisepwr__blk41_db8, var_noisepwr__blk41_db9, var_noisepwr__blk41_db10, var_noisepwr__blk41_db11, var_noisepwr__blk41_db12, var_noisepwr__blk41_db13, var_noisepwr__blk41_db14, var_noisepwr__blk41_db15, var_noisepwr__blk41_db16, var_noisepwr__blk41_db17, var_noisepwr__blk41_db18,)
    }
};
        var_noisepwr__blk41 = assign2350_e3074;
        var_noisepwr__blk41_dn0 = assign2350_e3074_d_n0;
        var_noisepwr__blk41_dn1 = assign2350_e3074_d_n1;
        var_noisepwr__blk41_dn2 = assign2350_e3074_d_n2;
        var_noisepwr__blk41_dn3 = assign2350_e3074_d_n3;
        var_noisepwr__blk41_dn4 = assign2350_e3074_d_n4;
        var_noisepwr__blk41_dn5 = assign2350_e3074_d_n5;
        var_noisepwr__blk41_dn6 = assign2350_e3074_d_n6;
        var_noisepwr__blk41_dn7 = assign2350_e3074_d_n7;
        var_noisepwr__blk41_dn8 = assign2350_e3074_d_n8;
        var_noisepwr__blk41_dn9 = assign2350_e3074_d_n9;
        var_noisepwr__blk41_dn10 = assign2350_e3074_d_n10;
        var_noisepwr__blk41_dn11 = assign2350_e3074_d_n11;
        var_noisepwr__blk41_dn12 = assign2350_e3074_d_n12;
        var_noisepwr__blk41_dn13 = assign2350_e3074_d_n13;
        var_noisepwr__blk41_dn14 = assign2350_e3074_d_n14;
        var_noisepwr__blk41_dn15 = assign2350_e3074_d_n15;
        var_noisepwr__blk41_dn16 = assign2350_e3074_d_n16;
        var_noisepwr__blk41_dn17 = assign2350_e3074_d_n17;
        var_noisepwr__blk41_dn18 = assign2350_e3074_d_n18;
        var_noisepwr__blk41_db0 = assign2350_e3074_d_b0;
        var_noisepwr__blk41_db1 = assign2350_e3074_d_b1;
        var_noisepwr__blk41_db2 = assign2350_e3074_d_b2;
        var_noisepwr__blk41_db3 = assign2350_e3074_d_b3;
        var_noisepwr__blk41_db4 = assign2350_e3074_d_b4;
        var_noisepwr__blk41_db5 = assign2350_e3074_d_b5;
        var_noisepwr__blk41_db6 = assign2350_e3074_d_b6;
        var_noisepwr__blk41_db7 = assign2350_e3074_d_b7;
        var_noisepwr__blk41_db8 = assign2350_e3074_d_b8;
        var_noisepwr__blk41_db9 = assign2350_e3074_d_b9;
        var_noisepwr__blk41_db10 = assign2350_e3074_d_b10;
        var_noisepwr__blk41_db11 = assign2350_e3074_d_b11;
        var_noisepwr__blk41_db12 = assign2350_e3074_d_b12;
        var_noisepwr__blk41_db13 = assign2350_e3074_d_b13;
        var_noisepwr__blk41_db14 = assign2350_e3074_d_b14;
        var_noisepwr__blk41_db15 = assign2350_e3074_d_b15;
        var_noisepwr__blk41_db16 = assign2350_e3074_d_b16;
        var_noisepwr__blk41_db17 = assign2350_e3074_d_b17;
        var_noisepwr__blk41_db18 = assign2350_e3074_d_b18;
        var_noisepwr__blk41_rv = 0.0;
        var_noisepwr__blk41_rdn0 = 0.0;
        var_noisepwr__blk41_rdn1 = 0.0;
        var_noisepwr__blk41_rdn2 = 0.0;
        var_noisepwr__blk41_rdn3 = 0.0;
        var_noisepwr__blk41_rdn4 = 0.0;
        var_noisepwr__blk41_rdn5 = 0.0;
        var_noisepwr__blk41_rdn6 = 0.0;
        var_noisepwr__blk41_rdn7 = 0.0;
        var_noisepwr__blk41_rdn8 = 0.0;
        var_noisepwr__blk41_rdn9 = 0.0;
        var_noisepwr__blk41_rdn10 = 0.0;
        var_noisepwr__blk41_rdn11 = 0.0;
        var_noisepwr__blk41_rdn12 = 0.0;
        var_noisepwr__blk41_rdn13 = 0.0;
        var_noisepwr__blk41_rdn14 = 0.0;
        var_noisepwr__blk41_rdn15 = 0.0;
        var_noisepwr__blk41_rdn16 = 0.0;
        var_noisepwr__blk41_rdn17 = 0.0;
        var_noisepwr__blk41_rdn18 = 0.0;
        var_noisepwr__blk41_rdb0 = 0.0;
        var_noisepwr__blk41_rdb1 = 0.0;
        var_noisepwr__blk41_rdb2 = 0.0;
        var_noisepwr__blk41_rdb3 = 0.0;
        var_noisepwr__blk41_rdb4 = 0.0;
        var_noisepwr__blk41_rdb5 = 0.0;
        var_noisepwr__blk41_rdb6 = 0.0;
        var_noisepwr__blk41_rdb7 = 0.0;
        var_noisepwr__blk41_rdb8 = 0.0;
        var_noisepwr__blk41_rdb9 = 0.0;
        var_noisepwr__blk41_rdb10 = 0.0;
        var_noisepwr__blk41_rdb11 = 0.0;
        var_noisepwr__blk41_rdb12 = 0.0;
        var_noisepwr__blk41_rdb13 = 0.0;
        var_noisepwr__blk41_rdb14 = 0.0;
        var_noisepwr__blk41_rdb15 = 0.0;
        var_noisepwr__blk41_rdb16 = 0.0;
        var_noisepwr__blk41_rdb17 = 0.0;
        var_noisepwr__blk41_rdb18 = 0.0;

        let assign2370_e3080: f64 = if p.p90 > 0.0 { 1.0 } else { 0.0 };
        var_guard43 = assign2370_e3080;
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
        var_guard43_dn16 = 0.0;
        var_guard43_dn17 = 0.0;
        var_guard43_dn18 = 0.0;
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
        var_guard43_rdn16 = 0.0;
        var_guard43_rdn17 = 0.0;
        var_guard43_rdn18 = 0.0;
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

        let assign2380_e3083: f64 = if p.p1 == 1.0 { 1.0 } else { 0.0 };
        var_guard44 = assign2380_e3083;
        var_guard44_dn0 = 0.0;
        var_guard44_dn1 = 0.0;
        var_guard44_dn2 = 0.0;
        var_guard44_dn3 = 0.0;
        var_guard44_dn4 = 0.0;
        var_guard44_dn5 = 0.0;
        var_guard44_dn6 = 0.0;
        var_guard44_dn7 = 0.0;
        var_guard44_dn8 = 0.0;
        var_guard44_dn9 = 0.0;
        var_guard44_dn10 = 0.0;
        var_guard44_dn11 = 0.0;
        var_guard44_dn12 = 0.0;
        var_guard44_dn13 = 0.0;
        var_guard44_dn14 = 0.0;
        var_guard44_dn15 = 0.0;
        var_guard44_dn16 = 0.0;
        var_guard44_dn17 = 0.0;
        var_guard44_dn18 = 0.0;
        var_guard44_db0 = 0.0;
        var_guard44_db1 = 0.0;
        var_guard44_db2 = 0.0;
        var_guard44_db3 = 0.0;
        var_guard44_db4 = 0.0;
        var_guard44_db5 = 0.0;
        var_guard44_db6 = 0.0;
        var_guard44_db7 = 0.0;
        var_guard44_db8 = 0.0;
        var_guard44_db9 = 0.0;
        var_guard44_db10 = 0.0;
        var_guard44_db11 = 0.0;
        var_guard44_db12 = 0.0;
        var_guard44_db13 = 0.0;
        var_guard44_db14 = 0.0;
        var_guard44_db15 = 0.0;
        var_guard44_db16 = 0.0;
        var_guard44_db17 = 0.0;
        var_guard44_db18 = 0.0;
        var_guard44_rv = 0.0;
        var_guard44_rdn0 = 0.0;
        var_guard44_rdn1 = 0.0;
        var_guard44_rdn2 = 0.0;
        var_guard44_rdn3 = 0.0;
        var_guard44_rdn4 = 0.0;
        var_guard44_rdn5 = 0.0;
        var_guard44_rdn6 = 0.0;
        var_guard44_rdn7 = 0.0;
        var_guard44_rdn8 = 0.0;
        var_guard44_rdn9 = 0.0;
        var_guard44_rdn10 = 0.0;
        var_guard44_rdn11 = 0.0;
        var_guard44_rdn12 = 0.0;
        var_guard44_rdn13 = 0.0;
        var_guard44_rdn14 = 0.0;
        var_guard44_rdn15 = 0.0;
        var_guard44_rdn16 = 0.0;
        var_guard44_rdn17 = 0.0;
        var_guard44_rdn18 = 0.0;
        var_guard44_rdb0 = 0.0;
        var_guard44_rdb1 = 0.0;
        var_guard44_rdb2 = 0.0;
        var_guard44_rdb3 = 0.0;
        var_guard44_rdb4 = 0.0;
        var_guard44_rdb5 = 0.0;
        var_guard44_rdb6 = 0.0;
        var_guard44_rdb7 = 0.0;
        var_guard44_rdb8 = 0.0;
        var_guard44_rdb9 = 0.0;
        var_guard44_rdb10 = 0.0;
        var_guard44_rdb11 = 0.0;
        var_guard44_rdb12 = 0.0;
        var_guard44_rdb13 = 0.0;
        var_guard44_rdb14 = 0.0;
        var_guard44_rdb15 = 0.0;
        var_guard44_rdb16 = 0.0;
        var_guard44_rdb17 = 0.0;
        var_guard44_rdb18 = 0.0;


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
        *var_ci_dn16_slot = var_ci_dn16;
        *var_ci_dn17_slot = var_ci_dn17;
        *var_ci_dn18_slot = var_ci_dn18;
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
        *var_ci_rdn16_slot = var_ci_rdn16;
        *var_ci_rdn17_slot = var_ci_rdn17;
        *var_ci_rdn18_slot = var_ci_rdn18;
        *var_ci_rdn2_slot = var_ci_rdn2;
        *var_ci_rdn3_slot = var_ci_rdn3;
        *var_ci_rdn4_slot = var_ci_rdn4;
        *var_ci_rdn5_slot = var_ci_rdn5;
        *var_ci_rdn6_slot = var_ci_rdn6;
        *var_ci_rdn7_slot = var_ci_rdn7;
        *var_ci_rdn8_slot = var_ci_rdn8;
        *var_ci_rdn9_slot = var_ci_rdn9;
        *var_ci_rv_slot = var_ci_rv;
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
        *var_guard43_dn16_slot = var_guard43_dn16;
        *var_guard43_dn17_slot = var_guard43_dn17;
        *var_guard43_dn18_slot = var_guard43_dn18;
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
        *var_guard43_rdn16_slot = var_guard43_rdn16;
        *var_guard43_rdn17_slot = var_guard43_rdn17;
        *var_guard43_rdn18_slot = var_guard43_rdn18;
        *var_guard43_rdn2_slot = var_guard43_rdn2;
        *var_guard43_rdn3_slot = var_guard43_rdn3;
        *var_guard43_rdn4_slot = var_guard43_rdn4;
        *var_guard43_rdn5_slot = var_guard43_rdn5;
        *var_guard43_rdn6_slot = var_guard43_rdn6;
        *var_guard43_rdn7_slot = var_guard43_rdn7;
        *var_guard43_rdn8_slot = var_guard43_rdn8;
        *var_guard43_rdn9_slot = var_guard43_rdn9;
        *var_guard43_rv_slot = var_guard43_rv;
        *var_guard44_slot = var_guard44;
        *var_guard44_db0_slot = var_guard44_db0;
        *var_guard44_db1_slot = var_guard44_db1;
        *var_guard44_db10_slot = var_guard44_db10;
        *var_guard44_db11_slot = var_guard44_db11;
        *var_guard44_db12_slot = var_guard44_db12;
        *var_guard44_db13_slot = var_guard44_db13;
        *var_guard44_db14_slot = var_guard44_db14;
        *var_guard44_db15_slot = var_guard44_db15;
        *var_guard44_db16_slot = var_guard44_db16;
        *var_guard44_db17_slot = var_guard44_db17;
        *var_guard44_db18_slot = var_guard44_db18;
        *var_guard44_db2_slot = var_guard44_db2;
        *var_guard44_db3_slot = var_guard44_db3;
        *var_guard44_db4_slot = var_guard44_db4;
        *var_guard44_db5_slot = var_guard44_db5;
        *var_guard44_db6_slot = var_guard44_db6;
        *var_guard44_db7_slot = var_guard44_db7;
        *var_guard44_db8_slot = var_guard44_db8;
        *var_guard44_db9_slot = var_guard44_db9;
        *var_guard44_dn0_slot = var_guard44_dn0;
        *var_guard44_dn1_slot = var_guard44_dn1;
        *var_guard44_dn10_slot = var_guard44_dn10;
        *var_guard44_dn11_slot = var_guard44_dn11;
        *var_guard44_dn12_slot = var_guard44_dn12;
        *var_guard44_dn13_slot = var_guard44_dn13;
        *var_guard44_dn14_slot = var_guard44_dn14;
        *var_guard44_dn15_slot = var_guard44_dn15;
        *var_guard44_dn16_slot = var_guard44_dn16;
        *var_guard44_dn17_slot = var_guard44_dn17;
        *var_guard44_dn18_slot = var_guard44_dn18;
        *var_guard44_dn2_slot = var_guard44_dn2;
        *var_guard44_dn3_slot = var_guard44_dn3;
        *var_guard44_dn4_slot = var_guard44_dn4;
        *var_guard44_dn5_slot = var_guard44_dn5;
        *var_guard44_dn6_slot = var_guard44_dn6;
        *var_guard44_dn7_slot = var_guard44_dn7;
        *var_guard44_dn8_slot = var_guard44_dn8;
        *var_guard44_dn9_slot = var_guard44_dn9;
        *var_guard44_rdb0_slot = var_guard44_rdb0;
        *var_guard44_rdb1_slot = var_guard44_rdb1;
        *var_guard44_rdb10_slot = var_guard44_rdb10;
        *var_guard44_rdb11_slot = var_guard44_rdb11;
        *var_guard44_rdb12_slot = var_guard44_rdb12;
        *var_guard44_rdb13_slot = var_guard44_rdb13;
        *var_guard44_rdb14_slot = var_guard44_rdb14;
        *var_guard44_rdb15_slot = var_guard44_rdb15;
        *var_guard44_rdb16_slot = var_guard44_rdb16;
        *var_guard44_rdb17_slot = var_guard44_rdb17;
        *var_guard44_rdb18_slot = var_guard44_rdb18;
        *var_guard44_rdb2_slot = var_guard44_rdb2;
        *var_guard44_rdb3_slot = var_guard44_rdb3;
        *var_guard44_rdb4_slot = var_guard44_rdb4;
        *var_guard44_rdb5_slot = var_guard44_rdb5;
        *var_guard44_rdb6_slot = var_guard44_rdb6;
        *var_guard44_rdb7_slot = var_guard44_rdb7;
        *var_guard44_rdb8_slot = var_guard44_rdb8;
        *var_guard44_rdb9_slot = var_guard44_rdb9;
        *var_guard44_rdn0_slot = var_guard44_rdn0;
        *var_guard44_rdn1_slot = var_guard44_rdn1;
        *var_guard44_rdn10_slot = var_guard44_rdn10;
        *var_guard44_rdn11_slot = var_guard44_rdn11;
        *var_guard44_rdn12_slot = var_guard44_rdn12;
        *var_guard44_rdn13_slot = var_guard44_rdn13;
        *var_guard44_rdn14_slot = var_guard44_rdn14;
        *var_guard44_rdn15_slot = var_guard44_rdn15;
        *var_guard44_rdn16_slot = var_guard44_rdn16;
        *var_guard44_rdn17_slot = var_guard44_rdn17;
        *var_guard44_rdn18_slot = var_guard44_rdn18;
        *var_guard44_rdn2_slot = var_guard44_rdn2;
        *var_guard44_rdn3_slot = var_guard44_rdn3;
        *var_guard44_rdn4_slot = var_guard44_rdn4;
        *var_guard44_rdn5_slot = var_guard44_rdn5;
        *var_guard44_rdn6_slot = var_guard44_rdn6;
        *var_guard44_rdn7_slot = var_guard44_rdn7;
        *var_guard44_rdn8_slot = var_guard44_rdn8;
        *var_guard44_rdn9_slot = var_guard44_rdn9;
        *var_guard44_rv_slot = var_guard44_rv;
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
        *var_k_dn16_slot = var_k_dn16;
        *var_k_dn17_slot = var_k_dn17;
        *var_k_dn18_slot = var_k_dn18;
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
        *var_k_rdn16_slot = var_k_rdn16;
        *var_k_rdn17_slot = var_k_rdn17;
        *var_k_rdn18_slot = var_k_rdn18;
        *var_k_rdn2_slot = var_k_rdn2;
        *var_k_rdn3_slot = var_k_rdn3;
        *var_k_rdn4_slot = var_k_rdn4;
        *var_k_rdn5_slot = var_k_rdn5;
        *var_k_rdn6_slot = var_k_rdn6;
        *var_k_rdn7_slot = var_k_rdn7;
        *var_k_rdn8_slot = var_k_rdn8;
        *var_k_rdn9_slot = var_k_rdn9;
        *var_k_rv_slot = var_k_rv;
        *var_noisepwr__blk41_slot = var_noisepwr__blk41;
        *var_noisepwr__blk41_db0_slot = var_noisepwr__blk41_db0;
        *var_noisepwr__blk41_db1_slot = var_noisepwr__blk41_db1;
        *var_noisepwr__blk41_db10_slot = var_noisepwr__blk41_db10;
        *var_noisepwr__blk41_db11_slot = var_noisepwr__blk41_db11;
        *var_noisepwr__blk41_db12_slot = var_noisepwr__blk41_db12;
        *var_noisepwr__blk41_db13_slot = var_noisepwr__blk41_db13;
        *var_noisepwr__blk41_db14_slot = var_noisepwr__blk41_db14;
        *var_noisepwr__blk41_db15_slot = var_noisepwr__blk41_db15;
        *var_noisepwr__blk41_db16_slot = var_noisepwr__blk41_db16;
        *var_noisepwr__blk41_db17_slot = var_noisepwr__blk41_db17;
        *var_noisepwr__blk41_db18_slot = var_noisepwr__blk41_db18;
        *var_noisepwr__blk41_db2_slot = var_noisepwr__blk41_db2;
        *var_noisepwr__blk41_db3_slot = var_noisepwr__blk41_db3;
        *var_noisepwr__blk41_db4_slot = var_noisepwr__blk41_db4;
        *var_noisepwr__blk41_db5_slot = var_noisepwr__blk41_db5;
        *var_noisepwr__blk41_db6_slot = var_noisepwr__blk41_db6;
        *var_noisepwr__blk41_db7_slot = var_noisepwr__blk41_db7;
        *var_noisepwr__blk41_db8_slot = var_noisepwr__blk41_db8;
        *var_noisepwr__blk41_db9_slot = var_noisepwr__blk41_db9;
        *var_noisepwr__blk41_dn0_slot = var_noisepwr__blk41_dn0;
        *var_noisepwr__blk41_dn1_slot = var_noisepwr__blk41_dn1;
        *var_noisepwr__blk41_dn10_slot = var_noisepwr__blk41_dn10;
        *var_noisepwr__blk41_dn11_slot = var_noisepwr__blk41_dn11;
        *var_noisepwr__blk41_dn12_slot = var_noisepwr__blk41_dn12;
        *var_noisepwr__blk41_dn13_slot = var_noisepwr__blk41_dn13;
        *var_noisepwr__blk41_dn14_slot = var_noisepwr__blk41_dn14;
        *var_noisepwr__blk41_dn15_slot = var_noisepwr__blk41_dn15;
        *var_noisepwr__blk41_dn16_slot = var_noisepwr__blk41_dn16;
        *var_noisepwr__blk41_dn17_slot = var_noisepwr__blk41_dn17;
        *var_noisepwr__blk41_dn18_slot = var_noisepwr__blk41_dn18;
        *var_noisepwr__blk41_dn2_slot = var_noisepwr__blk41_dn2;
        *var_noisepwr__blk41_dn3_slot = var_noisepwr__blk41_dn3;
        *var_noisepwr__blk41_dn4_slot = var_noisepwr__blk41_dn4;
        *var_noisepwr__blk41_dn5_slot = var_noisepwr__blk41_dn5;
        *var_noisepwr__blk41_dn6_slot = var_noisepwr__blk41_dn6;
        *var_noisepwr__blk41_dn7_slot = var_noisepwr__blk41_dn7;
        *var_noisepwr__blk41_dn8_slot = var_noisepwr__blk41_dn8;
        *var_noisepwr__blk41_dn9_slot = var_noisepwr__blk41_dn9;
        *var_noisepwr__blk41_rdb0_slot = var_noisepwr__blk41_rdb0;
        *var_noisepwr__blk41_rdb1_slot = var_noisepwr__blk41_rdb1;
        *var_noisepwr__blk41_rdb10_slot = var_noisepwr__blk41_rdb10;
        *var_noisepwr__blk41_rdb11_slot = var_noisepwr__blk41_rdb11;
        *var_noisepwr__blk41_rdb12_slot = var_noisepwr__blk41_rdb12;
        *var_noisepwr__blk41_rdb13_slot = var_noisepwr__blk41_rdb13;
        *var_noisepwr__blk41_rdb14_slot = var_noisepwr__blk41_rdb14;
        *var_noisepwr__blk41_rdb15_slot = var_noisepwr__blk41_rdb15;
        *var_noisepwr__blk41_rdb16_slot = var_noisepwr__blk41_rdb16;
        *var_noisepwr__blk41_rdb17_slot = var_noisepwr__blk41_rdb17;
        *var_noisepwr__blk41_rdb18_slot = var_noisepwr__blk41_rdb18;
        *var_noisepwr__blk41_rdb2_slot = var_noisepwr__blk41_rdb2;
        *var_noisepwr__blk41_rdb3_slot = var_noisepwr__blk41_rdb3;
        *var_noisepwr__blk41_rdb4_slot = var_noisepwr__blk41_rdb4;
        *var_noisepwr__blk41_rdb5_slot = var_noisepwr__blk41_rdb5;
        *var_noisepwr__blk41_rdb6_slot = var_noisepwr__blk41_rdb6;
        *var_noisepwr__blk41_rdb7_slot = var_noisepwr__blk41_rdb7;
        *var_noisepwr__blk41_rdb8_slot = var_noisepwr__blk41_rdb8;
        *var_noisepwr__blk41_rdb9_slot = var_noisepwr__blk41_rdb9;
        *var_noisepwr__blk41_rdn0_slot = var_noisepwr__blk41_rdn0;
        *var_noisepwr__blk41_rdn1_slot = var_noisepwr__blk41_rdn1;
        *var_noisepwr__blk41_rdn10_slot = var_noisepwr__blk41_rdn10;
        *var_noisepwr__blk41_rdn11_slot = var_noisepwr__blk41_rdn11;
        *var_noisepwr__blk41_rdn12_slot = var_noisepwr__blk41_rdn12;
        *var_noisepwr__blk41_rdn13_slot = var_noisepwr__blk41_rdn13;
        *var_noisepwr__blk41_rdn14_slot = var_noisepwr__blk41_rdn14;
        *var_noisepwr__blk41_rdn15_slot = var_noisepwr__blk41_rdn15;
        *var_noisepwr__blk41_rdn16_slot = var_noisepwr__blk41_rdn16;
        *var_noisepwr__blk41_rdn17_slot = var_noisepwr__blk41_rdn17;
        *var_noisepwr__blk41_rdn18_slot = var_noisepwr__blk41_rdn18;
        *var_noisepwr__blk41_rdn2_slot = var_noisepwr__blk41_rdn2;
        *var_noisepwr__blk41_rdn3_slot = var_noisepwr__blk41_rdn3;
        *var_noisepwr__blk41_rdn4_slot = var_noisepwr__blk41_rdn4;
        *var_noisepwr__blk41_rdn5_slot = var_noisepwr__blk41_rdn5;
        *var_noisepwr__blk41_rdn6_slot = var_noisepwr__blk41_rdn6;
        *var_noisepwr__blk41_rdn7_slot = var_noisepwr__blk41_rdn7;
        *var_noisepwr__blk41_rdn8_slot = var_noisepwr__blk41_rdn8;
        *var_noisepwr__blk41_rdn9_slot = var_noisepwr__blk41_rdn9;
        *var_noisepwr__blk41_rv_slot = var_noisepwr__blk41_rv;
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
        var_cdel_t: f64,
        var_cdel_t_db0: f64,
        var_cdel_t_db1: f64,
        var_cdel_t_db10: f64,
        var_cdel_t_db11: f64,
        var_cdel_t_db12: f64,
        var_cdel_t_db13: f64,
        var_cdel_t_db14: f64,
        var_cdel_t_db15: f64,
        var_cdel_t_db16: f64,
        var_cdel_t_db17: f64,
        var_cdel_t_db18: f64,
        var_cdel_t_db2: f64,
        var_cdel_t_db3: f64,
        var_cdel_t_db4: f64,
        var_cdel_t_db5: f64,
        var_cdel_t_db6: f64,
        var_cdel_t_db7: f64,
        var_cdel_t_db8: f64,
        var_cdel_t_db9: f64,
        var_cdel_t_dn0: f64,
        var_cdel_t_dn1: f64,
        var_cdel_t_dn10: f64,
        var_cdel_t_dn11: f64,
        var_cdel_t_dn12: f64,
        var_cdel_t_dn13: f64,
        var_cdel_t_dn14: f64,
        var_cdel_t_dn15: f64,
        var_cdel_t_dn16: f64,
        var_cdel_t_dn17: f64,
        var_cdel_t_dn18: f64,
        var_cdel_t_dn2: f64,
        var_cdel_t_dn3: f64,
        var_cdel_t_dn4: f64,
        var_cdel_t_dn5: f64,
        var_cdel_t_dn6: f64,
        var_cdel_t_dn7: f64,
        var_cdel_t_dn8: f64,
        var_cdel_t_dn9: f64,
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
        var_cgd_dn16: f64,
        var_cgd_dn17: f64,
        var_cgd_dn18: f64,
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
        var_cgs_dn16: f64,
        var_cgs_dn17: f64,
        var_cgs_dn18: f64,
        var_cgs_dn2: f64,
        var_cgs_dn3: f64,
        var_cgs_dn4: f64,
        var_cgs_dn5: f64,
        var_cgs_dn6: f64,
        var_cgs_dn7: f64,
        var_cgs_dn8: f64,
        var_cgs_dn9: f64,
        var_guard19: f64,
        var_guard20: f64,
        var_guard21: f64,
        var_guard25: f64,
        var_guard26: f64,
        var_guard27: f64,
        var_qgd: f64,
        var_qgd_db0: f64,
        var_qgd_db1: f64,
        var_qgd_db10: f64,
        var_qgd_db11: f64,
        var_qgd_db12: f64,
        var_qgd_db13: f64,
        var_qgd_db14: f64,
        var_qgd_db15: f64,
        var_qgd_db16: f64,
        var_qgd_db17: f64,
        var_qgd_db18: f64,
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
        var_qgd_dn16: f64,
        var_qgd_dn17: f64,
        var_qgd_dn18: f64,
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
        var_qgs_db15: f64,
        var_qgs_db16: f64,
        var_qgs_db17: f64,
        var_qgs_db18: f64,
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
        var_qgs_dn16: f64,
        var_qgs_dn17: f64,
        var_qgs_dn18: f64,
        var_qgs_dn2: f64,
        var_qgs_dn3: f64,
        var_qgs_dn4: f64,
        var_qgs_dn5: f64,
        var_qgs_dn6: f64,
        var_qgs_dn7: f64,
        var_qgs_dn8: f64,
        var_qgs_dn9: f64,
        var_rc1: f64,
        var_rc1_db0: f64,
        var_rc1_db1: f64,
        var_rc1_db10: f64,
        var_rc1_db11: f64,
        var_rc1_db12: f64,
        var_rc1_db13: f64,
        var_rc1_db14: f64,
        var_rc1_db15: f64,
        var_rc1_db16: f64,
        var_rc1_db17: f64,
        var_rc1_db18: f64,
        var_rc1_db2: f64,
        var_rc1_db3: f64,
        var_rc1_db4: f64,
        var_rc1_db5: f64,
        var_rc1_db6: f64,
        var_rc1_db7: f64,
        var_rc1_db8: f64,
        var_rc1_db9: f64,
        var_rc1_dn0: f64,
        var_rc1_dn1: f64,
        var_rc1_dn10: f64,
        var_rc1_dn11: f64,
        var_rc1_dn12: f64,
        var_rc1_dn13: f64,
        var_rc1_dn14: f64,
        var_rc1_dn15: f64,
        var_rc1_dn16: f64,
        var_rc1_dn17: f64,
        var_rc1_dn18: f64,
        var_rc1_dn2: f64,
        var_rc1_dn3: f64,
        var_rc1_dn4: f64,
        var_rc1_dn5: f64,
        var_rc1_dn6: f64,
        var_rc1_dn7: f64,
        var_rc1_dn8: f64,
        var_rc1_dn9: f64,
        var_rd1_t: f64,
        var_rd1_t_db0: f64,
        var_rd1_t_db1: f64,
        var_rd1_t_db10: f64,
        var_rd1_t_db11: f64,
        var_rd1_t_db12: f64,
        var_rd1_t_db13: f64,
        var_rd1_t_db14: f64,
        var_rd1_t_db15: f64,
        var_rd1_t_db16: f64,
        var_rd1_t_db17: f64,
        var_rd1_t_db18: f64,
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
        var_rd1_t_dn16: f64,
        var_rd1_t_dn17: f64,
        var_rd1_t_dn18: f64,
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
        var_rs_t_db15: f64,
        var_rs_t_db16: f64,
        var_rs_t_db17: f64,
        var_rs_t_db18: f64,
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
        var_rs_t_dn16: f64,
        var_rs_t_dn17: f64,
        var_rs_t_dn18: f64,
        var_rs_t_dn2: f64,
        var_rs_t_dn3: f64,
        var_rs_t_dn4: f64,
        var_rs_t_dn5: f64,
        var_rs_t_dn6: f64,
        var_rs_t_dn7: f64,
        var_rs_t_dn8: f64,
        var_rs_t_dn9: f64,
        var_t0: f64,
        var_t0_db0: f64,
        var_t0_db1: f64,
        var_t0_db10: f64,
        var_t0_db11: f64,
        var_t0_db12: f64,
        var_t0_db13: f64,
        var_t0_db14: f64,
        var_t0_db15: f64,
        var_t0_db16: f64,
        var_t0_db17: f64,
        var_t0_db18: f64,
        var_t0_db2: f64,
        var_t0_db3: f64,
        var_t0_db4: f64,
        var_t0_db5: f64,
        var_t0_db6: f64,
        var_t0_db7: f64,
        var_t0_db8: f64,
        var_t0_db9: f64,
        var_t0_dn0: f64,
        var_t0_dn1: f64,
        var_t0_dn10: f64,
        var_t0_dn11: f64,
        var_t0_dn12: f64,
        var_t0_dn13: f64,
        var_t0_dn14: f64,
        var_t0_dn15: f64,
        var_t0_dn16: f64,
        var_t0_dn17: f64,
        var_t0_dn18: f64,
        var_t0_dn2: f64,
        var_t0_dn3: f64,
        var_t0_dn4: f64,
        var_t0_dn5: f64,
        var_t0_dn6: f64,
        var_t0_dn7: f64,
        var_t0_dn8: f64,
        var_t0_dn9: f64,
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
        var_vgdc_dn16: f64,
        var_vgdc_dn17: f64,
        var_vgdc_dn18: f64,
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
        var_vgsc_dn16: f64,
        var_vgsc_dn17: f64,
        var_vgsc_dn18: f64,
        var_vgsc_dn2: f64,
        var_vgsc_dn3: f64,
        var_vgsc_dn4: f64,
        var_vgsc_dn5: f64,
        var_vgsc_dn6: f64,
        var_vgsc_dn7: f64,
        var_vgsc_dn8: f64,
        var_vgsc_dn9: f64,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let bi0 = ctx.branch_current(branches[0]);
        let bi1 = ctx.branch_current(branches[1]);
        let bi7 = ctx.branch_current(branches[7]);
        let bi10 = ctx.branch_current(branches[10]);
        let bi11 = ctx.branch_current(branches[11]);
        let bi14 = ctx.branch_current(branches[14]);
        let bi15 = ctx.branch_current(branches[15]);
        let bi18 = ctx.branch_current(branches[18]);
        let eq3_e114: f64 = (p.p56 / 3.0);
        let eq3_e116: f64 = (eq3_e114 * bi0);
        let eq3_e117: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq3_e116);
        let eq3_value: f64 = eq3_e117;
        stamper.stamp_potential_branch1_local(
            0,
            eq3_value,
            0,
            (eq3_e114 * ddt_scale),
        );
        let (eq7_e125, eq7_e125_d_n0, eq7_e125_d_n1, eq7_e125_d_n2, eq7_e125_d_n3, eq7_e125_d_n4, eq7_e125_d_n5, eq7_e125_d_n6, eq7_e125_d_n7, eq7_e125_d_n8, eq7_e125_d_n9, eq7_e125_d_n10, eq7_e125_d_n11, eq7_e125_d_n12, eq7_e125_d_n13, eq7_e125_d_n14, eq7_e125_d_n15, eq7_e125_d_n16, eq7_e125_d_n17, eq7_e125_d_n18, eq7_e125_d_b0, eq7_e125_d_b1, eq7_e125_d_b2, eq7_e125_d_b3, eq7_e125_d_b4, eq7_e125_d_b5, eq7_e125_d_b6, eq7_e125_d_b7, eq7_e125_d_b8, eq7_e125_d_b9, eq7_e125_d_b10, eq7_e125_d_b11, eq7_e125_d_b12, eq7_e125_d_b13, eq7_e125_d_b14, eq7_e125_d_b15, eq7_e125_d_b16, eq7_e125_d_b17, eq7_e125_d_b18,) = {
    if (var_guard19 != 0.0) {
        let eq7_e123: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, var_qgd);
        (eq7_e123, (var_qgd_dn0 * ddt_scale), (var_qgd_dn1 * ddt_scale), (var_qgd_dn2 * ddt_scale), (var_qgd_dn3 * ddt_scale), (var_qgd_dn4 * ddt_scale), (var_qgd_dn5 * ddt_scale), (var_qgd_dn6 * ddt_scale), (var_qgd_dn7 * ddt_scale), (var_qgd_dn8 * ddt_scale), (var_qgd_dn9 * ddt_scale), (var_qgd_dn10 * ddt_scale), (var_qgd_dn11 * ddt_scale), (var_qgd_dn12 * ddt_scale), (var_qgd_dn13 * ddt_scale), (var_qgd_dn14 * ddt_scale), (var_qgd_dn15 * ddt_scale), (var_qgd_dn16 * ddt_scale), (var_qgd_dn17 * ddt_scale), (var_qgd_dn18 * ddt_scale), (var_qgd_db0 * ddt_scale), (var_qgd_db1 * ddt_scale), (var_qgd_db2 * ddt_scale), (var_qgd_db3 * ddt_scale), (var_qgd_db4 * ddt_scale), (var_qgd_db5 * ddt_scale), (var_qgd_db6 * ddt_scale), (var_qgd_db7 * ddt_scale), (var_qgd_db8 * ddt_scale), (var_qgd_db9 * ddt_scale), (var_qgd_db10 * ddt_scale), (var_qgd_db11 * ddt_scale), (var_qgd_db12 * ddt_scale), (var_qgd_db13 * ddt_scale), (var_qgd_db14 * ddt_scale), (var_qgd_db15 * ddt_scale), (var_qgd_db16 * ddt_scale), (var_qgd_db17 * ddt_scale), (var_qgd_db18 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e125;
        let eq7_node_derivatives: [f64; 19] = [eq7_e125_d_n0, eq7_e125_d_n1, eq7_e125_d_n2, eq7_e125_d_n3, eq7_e125_d_n4, eq7_e125_d_n5, eq7_e125_d_n6, eq7_e125_d_n7, eq7_e125_d_n8, eq7_e125_d_n9, eq7_e125_d_n10, eq7_e125_d_n11, eq7_e125_d_n12, eq7_e125_d_n13, eq7_e125_d_n14, eq7_e125_d_n15, eq7_e125_d_n16, eq7_e125_d_n17, eq7_e125_d_n18];
        let eq7_branch_derivatives: [f64; 19] = [eq7_e125_d_b0, eq7_e125_d_b1, eq7_e125_d_b2, eq7_e125_d_b3, eq7_e125_d_b4, eq7_e125_d_b5, eq7_e125_d_b6, eq7_e125_d_b7, eq7_e125_d_b8, eq7_e125_d_b9, eq7_e125_d_b10, eq7_e125_d_b11, eq7_e125_d_b12, eq7_e125_d_b13, eq7_e125_d_b14, eq7_e125_d_b15, eq7_e125_d_b16, eq7_e125_d_b17, eq7_e125_d_b18];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e130, eq8_e130_d_n0, eq8_e130_d_n1, eq8_e130_d_n2, eq8_e130_d_n3, eq8_e130_d_n4, eq8_e130_d_n5, eq8_e130_d_n6, eq8_e130_d_n7, eq8_e130_d_n8, eq8_e130_d_n9, eq8_e130_d_n10, eq8_e130_d_n11, eq8_e130_d_n12, eq8_e130_d_n13, eq8_e130_d_n14, eq8_e130_d_n15, eq8_e130_d_n16, eq8_e130_d_n17, eq8_e130_d_n18, eq8_e130_d_b0, eq8_e130_d_b1, eq8_e130_d_b2, eq8_e130_d_b3, eq8_e130_d_b4, eq8_e130_d_b5, eq8_e130_d_b6, eq8_e130_d_b7, eq8_e130_d_b8, eq8_e130_d_b9, eq8_e130_d_b10, eq8_e130_d_b11, eq8_e130_d_b12, eq8_e130_d_b13, eq8_e130_d_b14, eq8_e130_d_b15, eq8_e130_d_b16, eq8_e130_d_b17, eq8_e130_d_b18,) = {
    if (var_guard19 != 0.0) {
        let eq8_e128: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, var_qgs);
        (eq8_e128, (var_qgs_dn0 * ddt_scale), (var_qgs_dn1 * ddt_scale), (var_qgs_dn2 * ddt_scale), (var_qgs_dn3 * ddt_scale), (var_qgs_dn4 * ddt_scale), (var_qgs_dn5 * ddt_scale), (var_qgs_dn6 * ddt_scale), (var_qgs_dn7 * ddt_scale), (var_qgs_dn8 * ddt_scale), (var_qgs_dn9 * ddt_scale), (var_qgs_dn10 * ddt_scale), (var_qgs_dn11 * ddt_scale), (var_qgs_dn12 * ddt_scale), (var_qgs_dn13 * ddt_scale), (var_qgs_dn14 * ddt_scale), (var_qgs_dn15 * ddt_scale), (var_qgs_dn16 * ddt_scale), (var_qgs_dn17 * ddt_scale), (var_qgs_dn18 * ddt_scale), (var_qgs_db0 * ddt_scale), (var_qgs_db1 * ddt_scale), (var_qgs_db2 * ddt_scale), (var_qgs_db3 * ddt_scale), (var_qgs_db4 * ddt_scale), (var_qgs_db5 * ddt_scale), (var_qgs_db6 * ddt_scale), (var_qgs_db7 * ddt_scale), (var_qgs_db8 * ddt_scale), (var_qgs_db9 * ddt_scale), (var_qgs_db10 * ddt_scale), (var_qgs_db11 * ddt_scale), (var_qgs_db12 * ddt_scale), (var_qgs_db13 * ddt_scale), (var_qgs_db14 * ddt_scale), (var_qgs_db15 * ddt_scale), (var_qgs_db16 * ddt_scale), (var_qgs_db17 * ddt_scale), (var_qgs_db18 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e130;
        let eq8_node_derivatives: [f64; 19] = [eq8_e130_d_n0, eq8_e130_d_n1, eq8_e130_d_n2, eq8_e130_d_n3, eq8_e130_d_n4, eq8_e130_d_n5, eq8_e130_d_n6, eq8_e130_d_n7, eq8_e130_d_n8, eq8_e130_d_n9, eq8_e130_d_n10, eq8_e130_d_n11, eq8_e130_d_n12, eq8_e130_d_n13, eq8_e130_d_n14, eq8_e130_d_n15, eq8_e130_d_n16, eq8_e130_d_n17, eq8_e130_d_n18];
        let eq8_branch_derivatives: [f64; 19] = [eq8_e130_d_b0, eq8_e130_d_b1, eq8_e130_d_b2, eq8_e130_d_b3, eq8_e130_d_b4, eq8_e130_d_b5, eq8_e130_d_b6, eq8_e130_d_b7, eq8_e130_d_b8, eq8_e130_d_b9, eq8_e130_d_b10, eq8_e130_d_b11, eq8_e130_d_b12, eq8_e130_d_b13, eq8_e130_d_b14, eq8_e130_d_b15, eq8_e130_d_b16, eq8_e130_d_b17, eq8_e130_d_b18];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e138, eq9_e138_d_n0, eq9_e138_d_n1, eq9_e138_d_n2, eq9_e138_d_n3, eq9_e138_d_n4, eq9_e138_d_n5, eq9_e138_d_n6, eq9_e138_d_n7, eq9_e138_d_n8, eq9_e138_d_n9, eq9_e138_d_n10, eq9_e138_d_n11, eq9_e138_d_n12, eq9_e138_d_n13, eq9_e138_d_n14, eq9_e138_d_n15, eq9_e138_d_n16, eq9_e138_d_n17, eq9_e138_d_n18, eq9_e138_d_b0, eq9_e138_d_b1, eq9_e138_d_b2, eq9_e138_d_b3, eq9_e138_d_b4, eq9_e138_d_b5, eq9_e138_d_b6, eq9_e138_d_b7, eq9_e138_d_b8, eq9_e138_d_b9, eq9_e138_d_b10, eq9_e138_d_b11, eq9_e138_d_b12, eq9_e138_d_b13, eq9_e138_d_b14, eq9_e138_d_b15, eq9_e138_d_b16, eq9_e138_d_b17, eq9_e138_d_b18,) = {
    if (var_guard19 == 0.0) {
        let eq9_e135: f64 = (var_cgd * var_vgdc);
        let eq9_e135_d_n0: f64 = ((var_cgd_dn0 * var_vgdc) + (var_cgd * var_vgdc_dn0));
        let eq9_e135_d_n1: f64 = ((var_cgd_dn1 * var_vgdc) + (var_cgd * var_vgdc_dn1));
        let eq9_e135_d_n2: f64 = ((var_cgd_dn2 * var_vgdc) + (var_cgd * var_vgdc_dn2));
        let eq9_e135_d_n3: f64 = ((var_cgd_dn3 * var_vgdc) + (var_cgd * var_vgdc_dn3));
        let eq9_e135_d_n4: f64 = ((var_cgd_dn4 * var_vgdc) + (var_cgd * var_vgdc_dn4));
        let eq9_e135_d_n5: f64 = ((var_cgd_dn5 * var_vgdc) + (var_cgd * var_vgdc_dn5));
        let eq9_e135_d_n6: f64 = ((var_cgd_dn6 * var_vgdc) + (var_cgd * var_vgdc_dn6));
        let eq9_e135_d_n7: f64 = ((var_cgd_dn7 * var_vgdc) + (var_cgd * var_vgdc_dn7));
        let eq9_e135_d_n8: f64 = ((var_cgd_dn8 * var_vgdc) + (var_cgd * var_vgdc_dn8));
        let eq9_e135_d_n9: f64 = ((var_cgd_dn9 * var_vgdc) + (var_cgd * var_vgdc_dn9));
        let eq9_e135_d_n10: f64 = ((var_cgd_dn10 * var_vgdc) + (var_cgd * var_vgdc_dn10));
        let eq9_e135_d_n11: f64 = ((var_cgd_dn11 * var_vgdc) + (var_cgd * var_vgdc_dn11));
        let eq9_e135_d_n12: f64 = ((var_cgd_dn12 * var_vgdc) + (var_cgd * var_vgdc_dn12));
        let eq9_e135_d_n13: f64 = ((var_cgd_dn13 * var_vgdc) + (var_cgd * var_vgdc_dn13));
        let eq9_e135_d_n14: f64 = ((var_cgd_dn14 * var_vgdc) + (var_cgd * var_vgdc_dn14));
        let eq9_e135_d_n15: f64 = ((var_cgd_dn15 * var_vgdc) + (var_cgd * var_vgdc_dn15));
        let eq9_e135_d_n16: f64 = ((var_cgd_dn16 * var_vgdc) + (var_cgd * var_vgdc_dn16));
        let eq9_e135_d_n17: f64 = ((var_cgd_dn17 * var_vgdc) + (var_cgd * var_vgdc_dn17));
        let eq9_e135_d_n18: f64 = ((var_cgd_dn18 * var_vgdc) + (var_cgd * var_vgdc_dn18));
        let eq9_e135_d_b0: f64 = ((var_cgd_db0 * var_vgdc) + (var_cgd * var_vgdc_db0));
        let eq9_e135_d_b1: f64 = ((var_cgd_db1 * var_vgdc) + (var_cgd * var_vgdc_db1));
        let eq9_e135_d_b2: f64 = ((var_cgd_db2 * var_vgdc) + (var_cgd * var_vgdc_db2));
        let eq9_e135_d_b3: f64 = ((var_cgd_db3 * var_vgdc) + (var_cgd * var_vgdc_db3));
        let eq9_e135_d_b4: f64 = ((var_cgd_db4 * var_vgdc) + (var_cgd * var_vgdc_db4));
        let eq9_e135_d_b5: f64 = ((var_cgd_db5 * var_vgdc) + (var_cgd * var_vgdc_db5));
        let eq9_e135_d_b6: f64 = ((var_cgd_db6 * var_vgdc) + (var_cgd * var_vgdc_db6));
        let eq9_e135_d_b7: f64 = ((var_cgd_db7 * var_vgdc) + (var_cgd * var_vgdc_db7));
        let eq9_e135_d_b8: f64 = ((var_cgd_db8 * var_vgdc) + (var_cgd * var_vgdc_db8));
        let eq9_e135_d_b9: f64 = ((var_cgd_db9 * var_vgdc) + (var_cgd * var_vgdc_db9));
        let eq9_e135_d_b10: f64 = ((var_cgd_db10 * var_vgdc) + (var_cgd * var_vgdc_db10));
        let eq9_e135_d_b11: f64 = ((var_cgd_db11 * var_vgdc) + (var_cgd * var_vgdc_db11));
        let eq9_e135_d_b12: f64 = ((var_cgd_db12 * var_vgdc) + (var_cgd * var_vgdc_db12));
        let eq9_e135_d_b13: f64 = ((var_cgd_db13 * var_vgdc) + (var_cgd * var_vgdc_db13));
        let eq9_e135_d_b14: f64 = ((var_cgd_db14 * var_vgdc) + (var_cgd * var_vgdc_db14));
        let eq9_e135_d_b15: f64 = ((var_cgd_db15 * var_vgdc) + (var_cgd * var_vgdc_db15));
        let eq9_e135_d_b16: f64 = ((var_cgd_db16 * var_vgdc) + (var_cgd * var_vgdc_db16));
        let eq9_e135_d_b17: f64 = ((var_cgd_db17 * var_vgdc) + (var_cgd * var_vgdc_db17));
        let eq9_e135_d_b18: f64 = ((var_cgd_db18 * var_vgdc) + (var_cgd * var_vgdc_db18));
        let eq9_e136: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq9_e135);
        (eq9_e136, (eq9_e135_d_n0 * ddt_scale), (eq9_e135_d_n1 * ddt_scale), (eq9_e135_d_n2 * ddt_scale), (eq9_e135_d_n3 * ddt_scale), (eq9_e135_d_n4 * ddt_scale), (eq9_e135_d_n5 * ddt_scale), (eq9_e135_d_n6 * ddt_scale), (eq9_e135_d_n7 * ddt_scale), (eq9_e135_d_n8 * ddt_scale), (eq9_e135_d_n9 * ddt_scale), (eq9_e135_d_n10 * ddt_scale), (eq9_e135_d_n11 * ddt_scale), (eq9_e135_d_n12 * ddt_scale), (eq9_e135_d_n13 * ddt_scale), (eq9_e135_d_n14 * ddt_scale), (eq9_e135_d_n15 * ddt_scale), (eq9_e135_d_n16 * ddt_scale), (eq9_e135_d_n17 * ddt_scale), (eq9_e135_d_n18 * ddt_scale), (eq9_e135_d_b0 * ddt_scale), (eq9_e135_d_b1 * ddt_scale), (eq9_e135_d_b2 * ddt_scale), (eq9_e135_d_b3 * ddt_scale), (eq9_e135_d_b4 * ddt_scale), (eq9_e135_d_b5 * ddt_scale), (eq9_e135_d_b6 * ddt_scale), (eq9_e135_d_b7 * ddt_scale), (eq9_e135_d_b8 * ddt_scale), (eq9_e135_d_b9 * ddt_scale), (eq9_e135_d_b10 * ddt_scale), (eq9_e135_d_b11 * ddt_scale), (eq9_e135_d_b12 * ddt_scale), (eq9_e135_d_b13 * ddt_scale), (eq9_e135_d_b14 * ddt_scale), (eq9_e135_d_b15 * ddt_scale), (eq9_e135_d_b16 * ddt_scale), (eq9_e135_d_b17 * ddt_scale), (eq9_e135_d_b18 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e138;
        let eq9_node_derivatives: [f64; 19] = [eq9_e138_d_n0, eq9_e138_d_n1, eq9_e138_d_n2, eq9_e138_d_n3, eq9_e138_d_n4, eq9_e138_d_n5, eq9_e138_d_n6, eq9_e138_d_n7, eq9_e138_d_n8, eq9_e138_d_n9, eq9_e138_d_n10, eq9_e138_d_n11, eq9_e138_d_n12, eq9_e138_d_n13, eq9_e138_d_n14, eq9_e138_d_n15, eq9_e138_d_n16, eq9_e138_d_n17, eq9_e138_d_n18];
        let eq9_branch_derivatives: [f64; 19] = [eq9_e138_d_b0, eq9_e138_d_b1, eq9_e138_d_b2, eq9_e138_d_b3, eq9_e138_d_b4, eq9_e138_d_b5, eq9_e138_d_b6, eq9_e138_d_b7, eq9_e138_d_b8, eq9_e138_d_b9, eq9_e138_d_b10, eq9_e138_d_b11, eq9_e138_d_b12, eq9_e138_d_b13, eq9_e138_d_b14, eq9_e138_d_b15, eq9_e138_d_b16, eq9_e138_d_b17, eq9_e138_d_b18];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let (eq10_e146, eq10_e146_d_n0, eq10_e146_d_n1, eq10_e146_d_n2, eq10_e146_d_n3, eq10_e146_d_n4, eq10_e146_d_n5, eq10_e146_d_n6, eq10_e146_d_n7, eq10_e146_d_n8, eq10_e146_d_n9, eq10_e146_d_n10, eq10_e146_d_n11, eq10_e146_d_n12, eq10_e146_d_n13, eq10_e146_d_n14, eq10_e146_d_n15, eq10_e146_d_n16, eq10_e146_d_n17, eq10_e146_d_n18, eq10_e146_d_b0, eq10_e146_d_b1, eq10_e146_d_b2, eq10_e146_d_b3, eq10_e146_d_b4, eq10_e146_d_b5, eq10_e146_d_b6, eq10_e146_d_b7, eq10_e146_d_b8, eq10_e146_d_b9, eq10_e146_d_b10, eq10_e146_d_b11, eq10_e146_d_b12, eq10_e146_d_b13, eq10_e146_d_b14, eq10_e146_d_b15, eq10_e146_d_b16, eq10_e146_d_b17, eq10_e146_d_b18,) = {
    if (var_guard19 == 0.0) {
        let eq10_e143: f64 = (var_cgs * var_vgsc);
        let eq10_e143_d_n0: f64 = ((var_cgs_dn0 * var_vgsc) + (var_cgs * var_vgsc_dn0));
        let eq10_e143_d_n1: f64 = ((var_cgs_dn1 * var_vgsc) + (var_cgs * var_vgsc_dn1));
        let eq10_e143_d_n2: f64 = ((var_cgs_dn2 * var_vgsc) + (var_cgs * var_vgsc_dn2));
        let eq10_e143_d_n3: f64 = ((var_cgs_dn3 * var_vgsc) + (var_cgs * var_vgsc_dn3));
        let eq10_e143_d_n4: f64 = ((var_cgs_dn4 * var_vgsc) + (var_cgs * var_vgsc_dn4));
        let eq10_e143_d_n5: f64 = ((var_cgs_dn5 * var_vgsc) + (var_cgs * var_vgsc_dn5));
        let eq10_e143_d_n6: f64 = ((var_cgs_dn6 * var_vgsc) + (var_cgs * var_vgsc_dn6));
        let eq10_e143_d_n7: f64 = ((var_cgs_dn7 * var_vgsc) + (var_cgs * var_vgsc_dn7));
        let eq10_e143_d_n8: f64 = ((var_cgs_dn8 * var_vgsc) + (var_cgs * var_vgsc_dn8));
        let eq10_e143_d_n9: f64 = ((var_cgs_dn9 * var_vgsc) + (var_cgs * var_vgsc_dn9));
        let eq10_e143_d_n10: f64 = ((var_cgs_dn10 * var_vgsc) + (var_cgs * var_vgsc_dn10));
        let eq10_e143_d_n11: f64 = ((var_cgs_dn11 * var_vgsc) + (var_cgs * var_vgsc_dn11));
        let eq10_e143_d_n12: f64 = ((var_cgs_dn12 * var_vgsc) + (var_cgs * var_vgsc_dn12));
        let eq10_e143_d_n13: f64 = ((var_cgs_dn13 * var_vgsc) + (var_cgs * var_vgsc_dn13));
        let eq10_e143_d_n14: f64 = ((var_cgs_dn14 * var_vgsc) + (var_cgs * var_vgsc_dn14));
        let eq10_e143_d_n15: f64 = ((var_cgs_dn15 * var_vgsc) + (var_cgs * var_vgsc_dn15));
        let eq10_e143_d_n16: f64 = ((var_cgs_dn16 * var_vgsc) + (var_cgs * var_vgsc_dn16));
        let eq10_e143_d_n17: f64 = ((var_cgs_dn17 * var_vgsc) + (var_cgs * var_vgsc_dn17));
        let eq10_e143_d_n18: f64 = ((var_cgs_dn18 * var_vgsc) + (var_cgs * var_vgsc_dn18));
        let eq10_e143_d_b0: f64 = ((var_cgs_db0 * var_vgsc) + (var_cgs * var_vgsc_db0));
        let eq10_e143_d_b1: f64 = ((var_cgs_db1 * var_vgsc) + (var_cgs * var_vgsc_db1));
        let eq10_e143_d_b2: f64 = ((var_cgs_db2 * var_vgsc) + (var_cgs * var_vgsc_db2));
        let eq10_e143_d_b3: f64 = ((var_cgs_db3 * var_vgsc) + (var_cgs * var_vgsc_db3));
        let eq10_e143_d_b4: f64 = ((var_cgs_db4 * var_vgsc) + (var_cgs * var_vgsc_db4));
        let eq10_e143_d_b5: f64 = ((var_cgs_db5 * var_vgsc) + (var_cgs * var_vgsc_db5));
        let eq10_e143_d_b6: f64 = ((var_cgs_db6 * var_vgsc) + (var_cgs * var_vgsc_db6));
        let eq10_e143_d_b7: f64 = ((var_cgs_db7 * var_vgsc) + (var_cgs * var_vgsc_db7));
        let eq10_e143_d_b8: f64 = ((var_cgs_db8 * var_vgsc) + (var_cgs * var_vgsc_db8));
        let eq10_e143_d_b9: f64 = ((var_cgs_db9 * var_vgsc) + (var_cgs * var_vgsc_db9));
        let eq10_e143_d_b10: f64 = ((var_cgs_db10 * var_vgsc) + (var_cgs * var_vgsc_db10));
        let eq10_e143_d_b11: f64 = ((var_cgs_db11 * var_vgsc) + (var_cgs * var_vgsc_db11));
        let eq10_e143_d_b12: f64 = ((var_cgs_db12 * var_vgsc) + (var_cgs * var_vgsc_db12));
        let eq10_e143_d_b13: f64 = ((var_cgs_db13 * var_vgsc) + (var_cgs * var_vgsc_db13));
        let eq10_e143_d_b14: f64 = ((var_cgs_db14 * var_vgsc) + (var_cgs * var_vgsc_db14));
        let eq10_e143_d_b15: f64 = ((var_cgs_db15 * var_vgsc) + (var_cgs * var_vgsc_db15));
        let eq10_e143_d_b16: f64 = ((var_cgs_db16 * var_vgsc) + (var_cgs * var_vgsc_db16));
        let eq10_e143_d_b17: f64 = ((var_cgs_db17 * var_vgsc) + (var_cgs * var_vgsc_db17));
        let eq10_e143_d_b18: f64 = ((var_cgs_db18 * var_vgsc) + (var_cgs * var_vgsc_db18));
        let eq10_e144: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq10_e143);
        (eq10_e144, (eq10_e143_d_n0 * ddt_scale), (eq10_e143_d_n1 * ddt_scale), (eq10_e143_d_n2 * ddt_scale), (eq10_e143_d_n3 * ddt_scale), (eq10_e143_d_n4 * ddt_scale), (eq10_e143_d_n5 * ddt_scale), (eq10_e143_d_n6 * ddt_scale), (eq10_e143_d_n7 * ddt_scale), (eq10_e143_d_n8 * ddt_scale), (eq10_e143_d_n9 * ddt_scale), (eq10_e143_d_n10 * ddt_scale), (eq10_e143_d_n11 * ddt_scale), (eq10_e143_d_n12 * ddt_scale), (eq10_e143_d_n13 * ddt_scale), (eq10_e143_d_n14 * ddt_scale), (eq10_e143_d_n15 * ddt_scale), (eq10_e143_d_n16 * ddt_scale), (eq10_e143_d_n17 * ddt_scale), (eq10_e143_d_n18 * ddt_scale), (eq10_e143_d_b0 * ddt_scale), (eq10_e143_d_b1 * ddt_scale), (eq10_e143_d_b2 * ddt_scale), (eq10_e143_d_b3 * ddt_scale), (eq10_e143_d_b4 * ddt_scale), (eq10_e143_d_b5 * ddt_scale), (eq10_e143_d_b6 * ddt_scale), (eq10_e143_d_b7 * ddt_scale), (eq10_e143_d_b8 * ddt_scale), (eq10_e143_d_b9 * ddt_scale), (eq10_e143_d_b10 * ddt_scale), (eq10_e143_d_b11 * ddt_scale), (eq10_e143_d_b12 * ddt_scale), (eq10_e143_d_b13 * ddt_scale), (eq10_e143_d_b14 * ddt_scale), (eq10_e143_d_b15 * ddt_scale), (eq10_e143_d_b16 * ddt_scale), (eq10_e143_d_b17 * ddt_scale), (eq10_e143_d_b18 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e146;
        let eq10_node_derivatives: [f64; 19] = [eq10_e146_d_n0, eq10_e146_d_n1, eq10_e146_d_n2, eq10_e146_d_n3, eq10_e146_d_n4, eq10_e146_d_n5, eq10_e146_d_n6, eq10_e146_d_n7, eq10_e146_d_n8, eq10_e146_d_n9, eq10_e146_d_n10, eq10_e146_d_n11, eq10_e146_d_n12, eq10_e146_d_n13, eq10_e146_d_n14, eq10_e146_d_n15, eq10_e146_d_n16, eq10_e146_d_n17, eq10_e146_d_n18];
        let eq10_branch_derivatives: [f64; 19] = [eq10_e146_d_b0, eq10_e146_d_b1, eq10_e146_d_b2, eq10_e146_d_b3, eq10_e146_d_b4, eq10_e146_d_b5, eq10_e146_d_b6, eq10_e146_d_b7, eq10_e146_d_b8, eq10_e146_d_b9, eq10_e146_d_b10, eq10_e146_d_b11, eq10_e146_d_b12, eq10_e146_d_b13, eq10_e146_d_b14, eq10_e146_d_b15, eq10_e146_d_b16, eq10_e146_d_b17, eq10_e146_d_b18];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let (eq15_e169, eq15_e169_d_n0, eq15_e169_d_n1, eq15_e169_d_n2, eq15_e169_d_n3, eq15_e169_d_n4, eq15_e169_d_n5, eq15_e169_d_n6, eq15_e169_d_n7, eq15_e169_d_n8, eq15_e169_d_n9, eq15_e169_d_n10, eq15_e169_d_n11, eq15_e169_d_n12, eq15_e169_d_n13, eq15_e169_d_n14, eq15_e169_d_n15, eq15_e169_d_n16, eq15_e169_d_n17, eq15_e169_d_n18, eq15_e169_d_b0, eq15_e169_d_b1, eq15_e169_d_b2, eq15_e169_d_b3, eq15_e169_d_b4, eq15_e169_d_b5, eq15_e169_d_b6, eq15_e169_d_b7, eq15_e169_d_b8, eq15_e169_d_b9, eq15_e169_d_b10, eq15_e169_d_b11, eq15_e169_d_b12, eq15_e169_d_b13, eq15_e169_d_b14, eq15_e169_d_b15, eq15_e169_d_b16, eq15_e169_d_b17, eq15_e169_d_b18,) = {
    if (var_guard20 != 0.0) {
        let eq15_e165: f64 = (bi1 * var_rc1);
        let eq15_e165_d_n0: f64 = (bi1 * var_rc1_dn0);
        let eq15_e165_d_n1: f64 = (bi1 * var_rc1_dn1);
        let eq15_e165_d_n2: f64 = (bi1 * var_rc1_dn2);
        let eq15_e165_d_n3: f64 = (bi1 * var_rc1_dn3);
        let eq15_e165_d_n4: f64 = (bi1 * var_rc1_dn4);
        let eq15_e165_d_n5: f64 = (bi1 * var_rc1_dn5);
        let eq15_e165_d_n6: f64 = (bi1 * var_rc1_dn6);
        let eq15_e165_d_n7: f64 = (bi1 * var_rc1_dn7);
        let eq15_e165_d_n8: f64 = (bi1 * var_rc1_dn8);
        let eq15_e165_d_n9: f64 = (bi1 * var_rc1_dn9);
        let eq15_e165_d_n10: f64 = (bi1 * var_rc1_dn10);
        let eq15_e165_d_n11: f64 = (bi1 * var_rc1_dn11);
        let eq15_e165_d_n12: f64 = (bi1 * var_rc1_dn12);
        let eq15_e165_d_n13: f64 = (bi1 * var_rc1_dn13);
        let eq15_e165_d_n14: f64 = (bi1 * var_rc1_dn14);
        let eq15_e165_d_n15: f64 = (bi1 * var_rc1_dn15);
        let eq15_e165_d_n16: f64 = (bi1 * var_rc1_dn16);
        let eq15_e165_d_n17: f64 = (bi1 * var_rc1_dn17);
        let eq15_e165_d_n18: f64 = (bi1 * var_rc1_dn18);
        let eq15_e165_d_b0: f64 = (bi1 * var_rc1_db0);
        let eq15_e165_d_b1: f64 = (var_rc1 + (bi1 * var_rc1_db1));
        let eq15_e165_d_b2: f64 = (bi1 * var_rc1_db2);
        let eq15_e165_d_b3: f64 = (bi1 * var_rc1_db3);
        let eq15_e165_d_b4: f64 = (bi1 * var_rc1_db4);
        let eq15_e165_d_b5: f64 = (bi1 * var_rc1_db5);
        let eq15_e165_d_b6: f64 = (bi1 * var_rc1_db6);
        let eq15_e165_d_b7: f64 = (bi1 * var_rc1_db7);
        let eq15_e165_d_b8: f64 = (bi1 * var_rc1_db8);
        let eq15_e165_d_b9: f64 = (bi1 * var_rc1_db9);
        let eq15_e165_d_b10: f64 = (bi1 * var_rc1_db10);
        let eq15_e165_d_b11: f64 = (bi1 * var_rc1_db11);
        let eq15_e165_d_b12: f64 = (bi1 * var_rc1_db12);
        let eq15_e165_d_b13: f64 = (bi1 * var_rc1_db13);
        let eq15_e165_d_b14: f64 = (bi1 * var_rc1_db14);
        let eq15_e165_d_b15: f64 = (bi1 * var_rc1_db15);
        let eq15_e165_d_b16: f64 = (bi1 * var_rc1_db16);
        let eq15_e165_d_b17: f64 = (bi1 * var_rc1_db17);
        let eq15_e165_d_b18: f64 = (bi1 * var_rc1_db18);
        let eq15_e167: f64 = (eq15_e165 + var_t0);
        let eq15_e167_d_n0: f64 = (eq15_e165_d_n0 + var_t0_dn0);
        let eq15_e167_d_n1: f64 = (eq15_e165_d_n1 + var_t0_dn1);
        let eq15_e167_d_n2: f64 = (eq15_e165_d_n2 + var_t0_dn2);
        let eq15_e167_d_n3: f64 = (eq15_e165_d_n3 + var_t0_dn3);
        let eq15_e167_d_n4: f64 = (eq15_e165_d_n4 + var_t0_dn4);
        let eq15_e167_d_n5: f64 = (eq15_e165_d_n5 + var_t0_dn5);
        let eq15_e167_d_n6: f64 = (eq15_e165_d_n6 + var_t0_dn6);
        let eq15_e167_d_n7: f64 = (eq15_e165_d_n7 + var_t0_dn7);
        let eq15_e167_d_n8: f64 = (eq15_e165_d_n8 + var_t0_dn8);
        let eq15_e167_d_n9: f64 = (eq15_e165_d_n9 + var_t0_dn9);
        let eq15_e167_d_n10: f64 = (eq15_e165_d_n10 + var_t0_dn10);
        let eq15_e167_d_n11: f64 = (eq15_e165_d_n11 + var_t0_dn11);
        let eq15_e167_d_n12: f64 = (eq15_e165_d_n12 + var_t0_dn12);
        let eq15_e167_d_n13: f64 = (eq15_e165_d_n13 + var_t0_dn13);
        let eq15_e167_d_n14: f64 = (eq15_e165_d_n14 + var_t0_dn14);
        let eq15_e167_d_n15: f64 = (eq15_e165_d_n15 + var_t0_dn15);
        let eq15_e167_d_n16: f64 = (eq15_e165_d_n16 + var_t0_dn16);
        let eq15_e167_d_n17: f64 = (eq15_e165_d_n17 + var_t0_dn17);
        let eq15_e167_d_n18: f64 = (eq15_e165_d_n18 + var_t0_dn18);
        let eq15_e167_d_b0: f64 = (eq15_e165_d_b0 + var_t0_db0);
        let eq15_e167_d_b1: f64 = (eq15_e165_d_b1 + var_t0_db1);
        let eq15_e167_d_b2: f64 = (eq15_e165_d_b2 + var_t0_db2);
        let eq15_e167_d_b3: f64 = (eq15_e165_d_b3 + var_t0_db3);
        let eq15_e167_d_b4: f64 = (eq15_e165_d_b4 + var_t0_db4);
        let eq15_e167_d_b5: f64 = (eq15_e165_d_b5 + var_t0_db5);
        let eq15_e167_d_b6: f64 = (eq15_e165_d_b6 + var_t0_db6);
        let eq15_e167_d_b7: f64 = (eq15_e165_d_b7 + var_t0_db7);
        let eq15_e167_d_b8: f64 = (eq15_e165_d_b8 + var_t0_db8);
        let eq15_e167_d_b9: f64 = (eq15_e165_d_b9 + var_t0_db9);
        let eq15_e167_d_b10: f64 = (eq15_e165_d_b10 + var_t0_db10);
        let eq15_e167_d_b11: f64 = (eq15_e165_d_b11 + var_t0_db11);
        let eq15_e167_d_b12: f64 = (eq15_e165_d_b12 + var_t0_db12);
        let eq15_e167_d_b13: f64 = (eq15_e165_d_b13 + var_t0_db13);
        let eq15_e167_d_b14: f64 = (eq15_e165_d_b14 + var_t0_db14);
        let eq15_e167_d_b15: f64 = (eq15_e165_d_b15 + var_t0_db15);
        let eq15_e167_d_b16: f64 = (eq15_e165_d_b16 + var_t0_db16);
        let eq15_e167_d_b17: f64 = (eq15_e165_d_b17 + var_t0_db17);
        let eq15_e167_d_b18: f64 = (eq15_e165_d_b18 + var_t0_db18);
        (eq15_e167, eq15_e167_d_n0, eq15_e167_d_n1, eq15_e167_d_n2, eq15_e167_d_n3, eq15_e167_d_n4, eq15_e167_d_n5, eq15_e167_d_n6, eq15_e167_d_n7, eq15_e167_d_n8, eq15_e167_d_n9, eq15_e167_d_n10, eq15_e167_d_n11, eq15_e167_d_n12, eq15_e167_d_n13, eq15_e167_d_n14, eq15_e167_d_n15, eq15_e167_d_n16, eq15_e167_d_n17, eq15_e167_d_n18, eq15_e167_d_b0, eq15_e167_d_b1, eq15_e167_d_b2, eq15_e167_d_b3, eq15_e167_d_b4, eq15_e167_d_b5, eq15_e167_d_b6, eq15_e167_d_b7, eq15_e167_d_b8, eq15_e167_d_b9, eq15_e167_d_b10, eq15_e167_d_b11, eq15_e167_d_b12, eq15_e167_d_b13, eq15_e167_d_b14, eq15_e167_d_b15, eq15_e167_d_b16, eq15_e167_d_b17, eq15_e167_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e169;
        let eq15_node_derivatives: [f64; 19] = [eq15_e169_d_n0, eq15_e169_d_n1, eq15_e169_d_n2, eq15_e169_d_n3, eq15_e169_d_n4, eq15_e169_d_n5, eq15_e169_d_n6, eq15_e169_d_n7, eq15_e169_d_n8, eq15_e169_d_n9, eq15_e169_d_n10, eq15_e169_d_n11, eq15_e169_d_n12, eq15_e169_d_n13, eq15_e169_d_n14, eq15_e169_d_n15, eq15_e169_d_n16, eq15_e169_d_n17, eq15_e169_d_n18];
        let eq15_branch_derivatives: [f64; 19] = [eq15_e169_d_b0, eq15_e169_d_b1, eq15_e169_d_b2, eq15_e169_d_b3, eq15_e169_d_b4, eq15_e169_d_b5, eq15_e169_d_b6, eq15_e169_d_b7, eq15_e169_d_b8, eq15_e169_d_b9, eq15_e169_d_b10, eq15_e169_d_b11, eq15_e169_d_b12, eq15_e169_d_b13, eq15_e169_d_b14, eq15_e169_d_b15, eq15_e169_d_b16, eq15_e169_d_b17, eq15_e169_d_b18];
        stamper.stamp_potential_dense_local(
            1,
            eq15_value,
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
        );
        let (eq18_e187, eq18_e187_d_n0, eq18_e187_d_n1, eq18_e187_d_n2, eq18_e187_d_n3, eq18_e187_d_n4, eq18_e187_d_n5, eq18_e187_d_n6, eq18_e187_d_n7, eq18_e187_d_n8, eq18_e187_d_n9, eq18_e187_d_n10, eq18_e187_d_n11, eq18_e187_d_n12, eq18_e187_d_n13, eq18_e187_d_n14, eq18_e187_d_n15, eq18_e187_d_n16, eq18_e187_d_n17, eq18_e187_d_n18, eq18_e187_d_b0, eq18_e187_d_b1, eq18_e187_d_b2, eq18_e187_d_b3, eq18_e187_d_b4, eq18_e187_d_b5, eq18_e187_d_b6, eq18_e187_d_b7, eq18_e187_d_b8, eq18_e187_d_b9, eq18_e187_d_b10, eq18_e187_d_b11, eq18_e187_d_b12, eq18_e187_d_b13, eq18_e187_d_b14, eq18_e187_d_b15, eq18_e187_d_b16, eq18_e187_d_b17, eq18_e187_d_b18,) = {
    if (var_guard21 != 0.0) {
        let eq18_e184: f64 = (var_cdel_t * (nv12 - nv8));
        let eq18_e184_d_n0: f64 = (var_cdel_t_dn0 * (nv12 - nv8));
        let eq18_e184_d_n1: f64 = (var_cdel_t_dn1 * (nv12 - nv8));
        let eq18_e184_d_n2: f64 = (var_cdel_t_dn2 * (nv12 - nv8));
        let eq18_e184_d_n3: f64 = (var_cdel_t_dn3 * (nv12 - nv8));
        let eq18_e184_d_n4: f64 = (var_cdel_t_dn4 * (nv12 - nv8));
        let eq18_e184_d_n5: f64 = (var_cdel_t_dn5 * (nv12 - nv8));
        let eq18_e184_d_n6: f64 = (var_cdel_t_dn6 * (nv12 - nv8));
        let eq18_e184_d_n7: f64 = (var_cdel_t_dn7 * (nv12 - nv8));
        let eq18_e184_d_n8: f64 = ((var_cdel_t_dn8 * (nv12 - nv8)) + (-var_cdel_t));
        let eq18_e184_d_n9: f64 = (var_cdel_t_dn9 * (nv12 - nv8));
        let eq18_e184_d_n10: f64 = (var_cdel_t_dn10 * (nv12 - nv8));
        let eq18_e184_d_n11: f64 = (var_cdel_t_dn11 * (nv12 - nv8));
        let eq18_e184_d_n12: f64 = ((var_cdel_t_dn12 * (nv12 - nv8)) + var_cdel_t);
        let eq18_e184_d_n13: f64 = (var_cdel_t_dn13 * (nv12 - nv8));
        let eq18_e184_d_n14: f64 = (var_cdel_t_dn14 * (nv12 - nv8));
        let eq18_e184_d_n15: f64 = (var_cdel_t_dn15 * (nv12 - nv8));
        let eq18_e184_d_n16: f64 = (var_cdel_t_dn16 * (nv12 - nv8));
        let eq18_e184_d_n17: f64 = (var_cdel_t_dn17 * (nv12 - nv8));
        let eq18_e184_d_n18: f64 = (var_cdel_t_dn18 * (nv12 - nv8));
        let eq18_e184_d_b0: f64 = (var_cdel_t_db0 * (nv12 - nv8));
        let eq18_e184_d_b1: f64 = (var_cdel_t_db1 * (nv12 - nv8));
        let eq18_e184_d_b2: f64 = (var_cdel_t_db2 * (nv12 - nv8));
        let eq18_e184_d_b3: f64 = (var_cdel_t_db3 * (nv12 - nv8));
        let eq18_e184_d_b4: f64 = (var_cdel_t_db4 * (nv12 - nv8));
        let eq18_e184_d_b5: f64 = (var_cdel_t_db5 * (nv12 - nv8));
        let eq18_e184_d_b6: f64 = (var_cdel_t_db6 * (nv12 - nv8));
        let eq18_e184_d_b7: f64 = (var_cdel_t_db7 * (nv12 - nv8));
        let eq18_e184_d_b8: f64 = (var_cdel_t_db8 * (nv12 - nv8));
        let eq18_e184_d_b9: f64 = (var_cdel_t_db9 * (nv12 - nv8));
        let eq18_e184_d_b10: f64 = (var_cdel_t_db10 * (nv12 - nv8));
        let eq18_e184_d_b11: f64 = (var_cdel_t_db11 * (nv12 - nv8));
        let eq18_e184_d_b12: f64 = (var_cdel_t_db12 * (nv12 - nv8));
        let eq18_e184_d_b13: f64 = (var_cdel_t_db13 * (nv12 - nv8));
        let eq18_e184_d_b14: f64 = (var_cdel_t_db14 * (nv12 - nv8));
        let eq18_e184_d_b15: f64 = (var_cdel_t_db15 * (nv12 - nv8));
        let eq18_e184_d_b16: f64 = (var_cdel_t_db16 * (nv12 - nv8));
        let eq18_e184_d_b17: f64 = (var_cdel_t_db17 * (nv12 - nv8));
        let eq18_e184_d_b18: f64 = (var_cdel_t_db18 * (nv12 - nv8));
        let eq18_e185: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq18_e184);
        (eq18_e185, (eq18_e184_d_n0 * ddt_scale), (eq18_e184_d_n1 * ddt_scale), (eq18_e184_d_n2 * ddt_scale), (eq18_e184_d_n3 * ddt_scale), (eq18_e184_d_n4 * ddt_scale), (eq18_e184_d_n5 * ddt_scale), (eq18_e184_d_n6 * ddt_scale), (eq18_e184_d_n7 * ddt_scale), (eq18_e184_d_n8 * ddt_scale), (eq18_e184_d_n9 * ddt_scale), (eq18_e184_d_n10 * ddt_scale), (eq18_e184_d_n11 * ddt_scale), (eq18_e184_d_n12 * ddt_scale), (eq18_e184_d_n13 * ddt_scale), (eq18_e184_d_n14 * ddt_scale), (eq18_e184_d_n15 * ddt_scale), (eq18_e184_d_n16 * ddt_scale), (eq18_e184_d_n17 * ddt_scale), (eq18_e184_d_n18 * ddt_scale), (eq18_e184_d_b0 * ddt_scale), (eq18_e184_d_b1 * ddt_scale), (eq18_e184_d_b2 * ddt_scale), (eq18_e184_d_b3 * ddt_scale), (eq18_e184_d_b4 * ddt_scale), (eq18_e184_d_b5 * ddt_scale), (eq18_e184_d_b6 * ddt_scale), (eq18_e184_d_b7 * ddt_scale), (eq18_e184_d_b8 * ddt_scale), (eq18_e184_d_b9 * ddt_scale), (eq18_e184_d_b10 * ddt_scale), (eq18_e184_d_b11 * ddt_scale), (eq18_e184_d_b12 * ddt_scale), (eq18_e184_d_b13 * ddt_scale), (eq18_e184_d_b14 * ddt_scale), (eq18_e184_d_b15 * ddt_scale), (eq18_e184_d_b16 * ddt_scale), (eq18_e184_d_b17 * ddt_scale), (eq18_e184_d_b18 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e187;
        let eq18_node_derivatives: [f64; 19] = [eq18_e187_d_n0, eq18_e187_d_n1, eq18_e187_d_n2, eq18_e187_d_n3, eq18_e187_d_n4, eq18_e187_d_n5, eq18_e187_d_n6, eq18_e187_d_n7, eq18_e187_d_n8, eq18_e187_d_n9, eq18_e187_d_n10, eq18_e187_d_n11, eq18_e187_d_n12, eq18_e187_d_n13, eq18_e187_d_n14, eq18_e187_d_n15, eq18_e187_d_n16, eq18_e187_d_n17, eq18_e187_d_n18];
        let eq18_branch_derivatives: [f64; 19] = [eq18_e187_d_b0, eq18_e187_d_b1, eq18_e187_d_b2, eq18_e187_d_b3, eq18_e187_d_b4, eq18_e187_d_b5, eq18_e187_d_b6, eq18_e187_d_b7, eq18_e187_d_b8, eq18_e187_d_b9, eq18_e187_d_b10, eq18_e187_d_b11, eq18_e187_d_b12, eq18_e187_d_b13, eq18_e187_d_b14, eq18_e187_d_b15, eq18_e187_d_b16, eq18_e187_d_b17, eq18_e187_d_b18];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(8),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq28_e247, eq28_e247_d_b7,) = {
    if (var_guard25 != 0.0) {
        let eq28_e245: f64 = (bi7 * p.p46);
        (eq28_e245, p.p46,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e247;
        stamper.stamp_potential_branch1_local(
            7,
            eq28_value,
            7,
            eq28_e247_d_b7,
        );
        let (eq29_e261,) = {
    if ((var_guard25 != 0.0) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e261;
        stamper.stamp_potential_const_local(
            8,
            eq29_value,
        );
        let eq31_e269: f64 = (p.p54 * bi10);
        let eq31_e270: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq31_e269);
        let eq31_value: f64 = eq31_e270;
        stamper.stamp_potential_branch1_local(
            10,
            eq31_value,
            10,
            (p.p54 * ddt_scale),
        );
        let (eq32_e276, eq32_e276_d_n0, eq32_e276_d_n1, eq32_e276_d_n2, eq32_e276_d_n3, eq32_e276_d_n4, eq32_e276_d_n5, eq32_e276_d_n6, eq32_e276_d_n7, eq32_e276_d_n8, eq32_e276_d_n9, eq32_e276_d_n10, eq32_e276_d_n11, eq32_e276_d_n12, eq32_e276_d_n13, eq32_e276_d_n14, eq32_e276_d_n15, eq32_e276_d_n16, eq32_e276_d_n17, eq32_e276_d_n18, eq32_e276_d_b0, eq32_e276_d_b1, eq32_e276_d_b2, eq32_e276_d_b3, eq32_e276_d_b4, eq32_e276_d_b5, eq32_e276_d_b6, eq32_e276_d_b7, eq32_e276_d_b8, eq32_e276_d_b9, eq32_e276_d_b10, eq32_e276_d_b11, eq32_e276_d_b12, eq32_e276_d_b13, eq32_e276_d_b14, eq32_e276_d_b15, eq32_e276_d_b16, eq32_e276_d_b17, eq32_e276_d_b18,) = {
    if (var_guard26 != 0.0) {
        let eq32_e274: f64 = (bi11 * var_rs_t);
        let eq32_e274_d_n0: f64 = (bi11 * var_rs_t_dn0);
        let eq32_e274_d_n1: f64 = (bi11 * var_rs_t_dn1);
        let eq32_e274_d_n2: f64 = (bi11 * var_rs_t_dn2);
        let eq32_e274_d_n3: f64 = (bi11 * var_rs_t_dn3);
        let eq32_e274_d_n4: f64 = (bi11 * var_rs_t_dn4);
        let eq32_e274_d_n5: f64 = (bi11 * var_rs_t_dn5);
        let eq32_e274_d_n6: f64 = (bi11 * var_rs_t_dn6);
        let eq32_e274_d_n7: f64 = (bi11 * var_rs_t_dn7);
        let eq32_e274_d_n8: f64 = (bi11 * var_rs_t_dn8);
        let eq32_e274_d_n9: f64 = (bi11 * var_rs_t_dn9);
        let eq32_e274_d_n10: f64 = (bi11 * var_rs_t_dn10);
        let eq32_e274_d_n11: f64 = (bi11 * var_rs_t_dn11);
        let eq32_e274_d_n12: f64 = (bi11 * var_rs_t_dn12);
        let eq32_e274_d_n13: f64 = (bi11 * var_rs_t_dn13);
        let eq32_e274_d_n14: f64 = (bi11 * var_rs_t_dn14);
        let eq32_e274_d_n15: f64 = (bi11 * var_rs_t_dn15);
        let eq32_e274_d_n16: f64 = (bi11 * var_rs_t_dn16);
        let eq32_e274_d_n17: f64 = (bi11 * var_rs_t_dn17);
        let eq32_e274_d_n18: f64 = (bi11 * var_rs_t_dn18);
        let eq32_e274_d_b0: f64 = (bi11 * var_rs_t_db0);
        let eq32_e274_d_b1: f64 = (bi11 * var_rs_t_db1);
        let eq32_e274_d_b2: f64 = (bi11 * var_rs_t_db2);
        let eq32_e274_d_b3: f64 = (bi11 * var_rs_t_db3);
        let eq32_e274_d_b4: f64 = (bi11 * var_rs_t_db4);
        let eq32_e274_d_b5: f64 = (bi11 * var_rs_t_db5);
        let eq32_e274_d_b6: f64 = (bi11 * var_rs_t_db6);
        let eq32_e274_d_b7: f64 = (bi11 * var_rs_t_db7);
        let eq32_e274_d_b8: f64 = (bi11 * var_rs_t_db8);
        let eq32_e274_d_b9: f64 = (bi11 * var_rs_t_db9);
        let eq32_e274_d_b10: f64 = (bi11 * var_rs_t_db10);
        let eq32_e274_d_b11: f64 = (var_rs_t + (bi11 * var_rs_t_db11));
        let eq32_e274_d_b12: f64 = (bi11 * var_rs_t_db12);
        let eq32_e274_d_b13: f64 = (bi11 * var_rs_t_db13);
        let eq32_e274_d_b14: f64 = (bi11 * var_rs_t_db14);
        let eq32_e274_d_b15: f64 = (bi11 * var_rs_t_db15);
        let eq32_e274_d_b16: f64 = (bi11 * var_rs_t_db16);
        let eq32_e274_d_b17: f64 = (bi11 * var_rs_t_db17);
        let eq32_e274_d_b18: f64 = (bi11 * var_rs_t_db18);
        (eq32_e274, eq32_e274_d_n0, eq32_e274_d_n1, eq32_e274_d_n2, eq32_e274_d_n3, eq32_e274_d_n4, eq32_e274_d_n5, eq32_e274_d_n6, eq32_e274_d_n7, eq32_e274_d_n8, eq32_e274_d_n9, eq32_e274_d_n10, eq32_e274_d_n11, eq32_e274_d_n12, eq32_e274_d_n13, eq32_e274_d_n14, eq32_e274_d_n15, eq32_e274_d_n16, eq32_e274_d_n17, eq32_e274_d_n18, eq32_e274_d_b0, eq32_e274_d_b1, eq32_e274_d_b2, eq32_e274_d_b3, eq32_e274_d_b4, eq32_e274_d_b5, eq32_e274_d_b6, eq32_e274_d_b7, eq32_e274_d_b8, eq32_e274_d_b9, eq32_e274_d_b10, eq32_e274_d_b11, eq32_e274_d_b12, eq32_e274_d_b13, eq32_e274_d_b14, eq32_e274_d_b15, eq32_e274_d_b16, eq32_e274_d_b17, eq32_e274_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e276;
        let eq32_node_derivatives: [f64; 19] = [eq32_e276_d_n0, eq32_e276_d_n1, eq32_e276_d_n2, eq32_e276_d_n3, eq32_e276_d_n4, eq32_e276_d_n5, eq32_e276_d_n6, eq32_e276_d_n7, eq32_e276_d_n8, eq32_e276_d_n9, eq32_e276_d_n10, eq32_e276_d_n11, eq32_e276_d_n12, eq32_e276_d_n13, eq32_e276_d_n14, eq32_e276_d_n15, eq32_e276_d_n16, eq32_e276_d_n17, eq32_e276_d_n18];
        let eq32_branch_derivatives: [f64; 19] = [eq32_e276_d_b0, eq32_e276_d_b1, eq32_e276_d_b2, eq32_e276_d_b3, eq32_e276_d_b4, eq32_e276_d_b5, eq32_e276_d_b6, eq32_e276_d_b7, eq32_e276_d_b8, eq32_e276_d_b9, eq32_e276_d_b10, eq32_e276_d_b11, eq32_e276_d_b12, eq32_e276_d_b13, eq32_e276_d_b14, eq32_e276_d_b15, eq32_e276_d_b16, eq32_e276_d_b17, eq32_e276_d_b18];
        stamper.stamp_potential_dense_local(
            11,
            eq32_value,
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
        );
        let (eq33_e290,) = {
    if ((var_guard26 != 0.0) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq33_value: f64 = eq33_e290;
        stamper.stamp_potential_const_local(
            12,
            eq33_value,
        );
        let eq35_e298: f64 = (p.p53 * bi14);
        let eq35_e299: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq35_e298);
        let eq35_value: f64 = eq35_e299;
        stamper.stamp_potential_branch1_local(
            14,
            eq35_value,
            14,
            (p.p53 * ddt_scale),
        );
        let (eq36_e305, eq36_e305_d_n0, eq36_e305_d_n1, eq36_e305_d_n2, eq36_e305_d_n3, eq36_e305_d_n4, eq36_e305_d_n5, eq36_e305_d_n6, eq36_e305_d_n7, eq36_e305_d_n8, eq36_e305_d_n9, eq36_e305_d_n10, eq36_e305_d_n11, eq36_e305_d_n12, eq36_e305_d_n13, eq36_e305_d_n14, eq36_e305_d_n15, eq36_e305_d_n16, eq36_e305_d_n17, eq36_e305_d_n18, eq36_e305_d_b0, eq36_e305_d_b1, eq36_e305_d_b2, eq36_e305_d_b3, eq36_e305_d_b4, eq36_e305_d_b5, eq36_e305_d_b6, eq36_e305_d_b7, eq36_e305_d_b8, eq36_e305_d_b9, eq36_e305_d_b10, eq36_e305_d_b11, eq36_e305_d_b12, eq36_e305_d_b13, eq36_e305_d_b14, eq36_e305_d_b15, eq36_e305_d_b16, eq36_e305_d_b17, eq36_e305_d_b18,) = {
    if (var_guard27 != 0.0) {
        let eq36_e303: f64 = (bi15 * var_rd1_t);
        let eq36_e303_d_n0: f64 = (bi15 * var_rd1_t_dn0);
        let eq36_e303_d_n1: f64 = (bi15 * var_rd1_t_dn1);
        let eq36_e303_d_n2: f64 = (bi15 * var_rd1_t_dn2);
        let eq36_e303_d_n3: f64 = (bi15 * var_rd1_t_dn3);
        let eq36_e303_d_n4: f64 = (bi15 * var_rd1_t_dn4);
        let eq36_e303_d_n5: f64 = (bi15 * var_rd1_t_dn5);
        let eq36_e303_d_n6: f64 = (bi15 * var_rd1_t_dn6);
        let eq36_e303_d_n7: f64 = (bi15 * var_rd1_t_dn7);
        let eq36_e303_d_n8: f64 = (bi15 * var_rd1_t_dn8);
        let eq36_e303_d_n9: f64 = (bi15 * var_rd1_t_dn9);
        let eq36_e303_d_n10: f64 = (bi15 * var_rd1_t_dn10);
        let eq36_e303_d_n11: f64 = (bi15 * var_rd1_t_dn11);
        let eq36_e303_d_n12: f64 = (bi15 * var_rd1_t_dn12);
        let eq36_e303_d_n13: f64 = (bi15 * var_rd1_t_dn13);
        let eq36_e303_d_n14: f64 = (bi15 * var_rd1_t_dn14);
        let eq36_e303_d_n15: f64 = (bi15 * var_rd1_t_dn15);
        let eq36_e303_d_n16: f64 = (bi15 * var_rd1_t_dn16);
        let eq36_e303_d_n17: f64 = (bi15 * var_rd1_t_dn17);
        let eq36_e303_d_n18: f64 = (bi15 * var_rd1_t_dn18);
        let eq36_e303_d_b0: f64 = (bi15 * var_rd1_t_db0);
        let eq36_e303_d_b1: f64 = (bi15 * var_rd1_t_db1);
        let eq36_e303_d_b2: f64 = (bi15 * var_rd1_t_db2);
        let eq36_e303_d_b3: f64 = (bi15 * var_rd1_t_db3);
        let eq36_e303_d_b4: f64 = (bi15 * var_rd1_t_db4);
        let eq36_e303_d_b5: f64 = (bi15 * var_rd1_t_db5);
        let eq36_e303_d_b6: f64 = (bi15 * var_rd1_t_db6);
        let eq36_e303_d_b7: f64 = (bi15 * var_rd1_t_db7);
        let eq36_e303_d_b8: f64 = (bi15 * var_rd1_t_db8);
        let eq36_e303_d_b9: f64 = (bi15 * var_rd1_t_db9);
        let eq36_e303_d_b10: f64 = (bi15 * var_rd1_t_db10);
        let eq36_e303_d_b11: f64 = (bi15 * var_rd1_t_db11);
        let eq36_e303_d_b12: f64 = (bi15 * var_rd1_t_db12);
        let eq36_e303_d_b13: f64 = (bi15 * var_rd1_t_db13);
        let eq36_e303_d_b14: f64 = (bi15 * var_rd1_t_db14);
        let eq36_e303_d_b15: f64 = (var_rd1_t + (bi15 * var_rd1_t_db15));
        let eq36_e303_d_b16: f64 = (bi15 * var_rd1_t_db16);
        let eq36_e303_d_b17: f64 = (bi15 * var_rd1_t_db17);
        let eq36_e303_d_b18: f64 = (bi15 * var_rd1_t_db18);
        (eq36_e303, eq36_e303_d_n0, eq36_e303_d_n1, eq36_e303_d_n2, eq36_e303_d_n3, eq36_e303_d_n4, eq36_e303_d_n5, eq36_e303_d_n6, eq36_e303_d_n7, eq36_e303_d_n8, eq36_e303_d_n9, eq36_e303_d_n10, eq36_e303_d_n11, eq36_e303_d_n12, eq36_e303_d_n13, eq36_e303_d_n14, eq36_e303_d_n15, eq36_e303_d_n16, eq36_e303_d_n17, eq36_e303_d_n18, eq36_e303_d_b0, eq36_e303_d_b1, eq36_e303_d_b2, eq36_e303_d_b3, eq36_e303_d_b4, eq36_e303_d_b5, eq36_e303_d_b6, eq36_e303_d_b7, eq36_e303_d_b8, eq36_e303_d_b9, eq36_e303_d_b10, eq36_e303_d_b11, eq36_e303_d_b12, eq36_e303_d_b13, eq36_e303_d_b14, eq36_e303_d_b15, eq36_e303_d_b16, eq36_e303_d_b17, eq36_e303_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e305;
        let eq36_node_derivatives: [f64; 19] = [eq36_e305_d_n0, eq36_e305_d_n1, eq36_e305_d_n2, eq36_e305_d_n3, eq36_e305_d_n4, eq36_e305_d_n5, eq36_e305_d_n6, eq36_e305_d_n7, eq36_e305_d_n8, eq36_e305_d_n9, eq36_e305_d_n10, eq36_e305_d_n11, eq36_e305_d_n12, eq36_e305_d_n13, eq36_e305_d_n14, eq36_e305_d_n15, eq36_e305_d_n16, eq36_e305_d_n17, eq36_e305_d_n18];
        let eq36_branch_derivatives: [f64; 19] = [eq36_e305_d_b0, eq36_e305_d_b1, eq36_e305_d_b2, eq36_e305_d_b3, eq36_e305_d_b4, eq36_e305_d_b5, eq36_e305_d_b6, eq36_e305_d_b7, eq36_e305_d_b8, eq36_e305_d_b9, eq36_e305_d_b10, eq36_e305_d_b11, eq36_e305_d_b12, eq36_e305_d_b13, eq36_e305_d_b14, eq36_e305_d_b15, eq36_e305_d_b16, eq36_e305_d_b17, eq36_e305_d_b18];
        stamper.stamp_potential_dense_local(
            15,
            eq36_value,
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
        );
        let (eq37_e319,) = {
    if ((var_guard27 != 0.0) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq37_value: f64 = eq37_e319;
        stamper.stamp_potential_const_local(
            16,
            eq37_value,
        );
        let eq39_e327: f64 = (p.p52 * bi18);
        let eq39_e328: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq39_e327);
        let eq39_value: f64 = eq39_e328;
        stamper.stamp_potential_branch1_local(
            18,
            eq39_value,
            18,
            (p.p52 * ddt_scale),
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
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
        var_ci: f64,
        var_ci_db0: f64,
        var_ci_db1: f64,
        var_ci_db10: f64,
        var_ci_db11: f64,
        var_ci_db12: f64,
        var_ci_db13: f64,
        var_ci_db14: f64,
        var_ci_db15: f64,
        var_ci_db16: f64,
        var_ci_db17: f64,
        var_ci_db18: f64,
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
        var_ci_dn16: f64,
        var_ci_dn17: f64,
        var_ci_dn18: f64,
        var_ci_dn2: f64,
        var_ci_dn3: f64,
        var_ci_dn4: f64,
        var_ci_dn5: f64,
        var_ci_dn6: f64,
        var_ci_dn7: f64,
        var_ci_dn8: f64,
        var_ci_dn9: f64,
        var_guard28: f64,
        var_guard29: f64,
        var_guard44: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq51_e429, eq51_e429_d_n0, eq51_e429_d_n1, eq51_e429_d_n2, eq51_e429_d_n3, eq51_e429_d_n4, eq51_e429_d_n5, eq51_e429_d_n6, eq51_e429_d_n7, eq51_e429_d_n8, eq51_e429_d_n9, eq51_e429_d_n10, eq51_e429_d_n11, eq51_e429_d_n12, eq51_e429_d_n13, eq51_e429_d_n14, eq51_e429_d_n15, eq51_e429_d_n16, eq51_e429_d_n17, eq51_e429_d_n18, eq51_e429_d_b0, eq51_e429_d_b1, eq51_e429_d_b2, eq51_e429_d_b3, eq51_e429_d_b4, eq51_e429_d_b5, eq51_e429_d_b6, eq51_e429_d_b7, eq51_e429_d_b8, eq51_e429_d_b9, eq51_e429_d_b10, eq51_e429_d_b11, eq51_e429_d_b12, eq51_e429_d_b13, eq51_e429_d_b14, eq51_e429_d_b15, eq51_e429_d_b16, eq51_e429_d_b17, eq51_e429_d_b18,) = {
    if (((var_guard29 != 0.0) && (var_guard28 == 0.0)) && (p.p0 != 0.0)) {
        let eq51_e424: f64 = (-var_ci);
        let eq51_e426: f64 = (eq51_e424 * (nv17 - 0.0));
        let eq51_e426_d_n0: f64 = ((-var_ci_dn0) * (nv17 - 0.0));
        let eq51_e426_d_n1: f64 = ((-var_ci_dn1) * (nv17 - 0.0));
        let eq51_e426_d_n2: f64 = ((-var_ci_dn2) * (nv17 - 0.0));
        let eq51_e426_d_n3: f64 = ((-var_ci_dn3) * (nv17 - 0.0));
        let eq51_e426_d_n4: f64 = ((-var_ci_dn4) * (nv17 - 0.0));
        let eq51_e426_d_n5: f64 = ((-var_ci_dn5) * (nv17 - 0.0));
        let eq51_e426_d_n6: f64 = ((-var_ci_dn6) * (nv17 - 0.0));
        let eq51_e426_d_n7: f64 = ((-var_ci_dn7) * (nv17 - 0.0));
        let eq51_e426_d_n8: f64 = ((-var_ci_dn8) * (nv17 - 0.0));
        let eq51_e426_d_n9: f64 = ((-var_ci_dn9) * (nv17 - 0.0));
        let eq51_e426_d_n10: f64 = ((-var_ci_dn10) * (nv17 - 0.0));
        let eq51_e426_d_n11: f64 = ((-var_ci_dn11) * (nv17 - 0.0));
        let eq51_e426_d_n12: f64 = ((-var_ci_dn12) * (nv17 - 0.0));
        let eq51_e426_d_n13: f64 = ((-var_ci_dn13) * (nv17 - 0.0));
        let eq51_e426_d_n14: f64 = ((-var_ci_dn14) * (nv17 - 0.0));
        let eq51_e426_d_n15: f64 = ((-var_ci_dn15) * (nv17 - 0.0));
        let eq51_e426_d_n16: f64 = ((-var_ci_dn16) * (nv17 - 0.0));
        let eq51_e426_d_n17: f64 = (((-var_ci_dn17) * (nv17 - 0.0)) + eq51_e424);
        let eq51_e426_d_n18: f64 = ((-var_ci_dn18) * (nv17 - 0.0));
        let eq51_e426_d_b0: f64 = ((-var_ci_db0) * (nv17 - 0.0));
        let eq51_e426_d_b1: f64 = ((-var_ci_db1) * (nv17 - 0.0));
        let eq51_e426_d_b2: f64 = ((-var_ci_db2) * (nv17 - 0.0));
        let eq51_e426_d_b3: f64 = ((-var_ci_db3) * (nv17 - 0.0));
        let eq51_e426_d_b4: f64 = ((-var_ci_db4) * (nv17 - 0.0));
        let eq51_e426_d_b5: f64 = ((-var_ci_db5) * (nv17 - 0.0));
        let eq51_e426_d_b6: f64 = ((-var_ci_db6) * (nv17 - 0.0));
        let eq51_e426_d_b7: f64 = ((-var_ci_db7) * (nv17 - 0.0));
        let eq51_e426_d_b8: f64 = ((-var_ci_db8) * (nv17 - 0.0));
        let eq51_e426_d_b9: f64 = ((-var_ci_db9) * (nv17 - 0.0));
        let eq51_e426_d_b10: f64 = ((-var_ci_db10) * (nv17 - 0.0));
        let eq51_e426_d_b11: f64 = ((-var_ci_db11) * (nv17 - 0.0));
        let eq51_e426_d_b12: f64 = ((-var_ci_db12) * (nv17 - 0.0));
        let eq51_e426_d_b13: f64 = ((-var_ci_db13) * (nv17 - 0.0));
        let eq51_e426_d_b14: f64 = ((-var_ci_db14) * (nv17 - 0.0));
        let eq51_e426_d_b15: f64 = ((-var_ci_db15) * (nv17 - 0.0));
        let eq51_e426_d_b16: f64 = ((-var_ci_db16) * (nv17 - 0.0));
        let eq51_e426_d_b17: f64 = ((-var_ci_db17) * (nv17 - 0.0));
        let eq51_e426_d_b18: f64 = ((-var_ci_db18) * (nv17 - 0.0));
        let eq51_e427: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq51_e426);
        (eq51_e427, (eq51_e426_d_n0 * ddt_scale), (eq51_e426_d_n1 * ddt_scale), (eq51_e426_d_n2 * ddt_scale), (eq51_e426_d_n3 * ddt_scale), (eq51_e426_d_n4 * ddt_scale), (eq51_e426_d_n5 * ddt_scale), (eq51_e426_d_n6 * ddt_scale), (eq51_e426_d_n7 * ddt_scale), (eq51_e426_d_n8 * ddt_scale), (eq51_e426_d_n9 * ddt_scale), (eq51_e426_d_n10 * ddt_scale), (eq51_e426_d_n11 * ddt_scale), (eq51_e426_d_n12 * ddt_scale), (eq51_e426_d_n13 * ddt_scale), (eq51_e426_d_n14 * ddt_scale), (eq51_e426_d_n15 * ddt_scale), (eq51_e426_d_n16 * ddt_scale), (eq51_e426_d_n17 * ddt_scale), (eq51_e426_d_n18 * ddt_scale), (eq51_e426_d_b0 * ddt_scale), (eq51_e426_d_b1 * ddt_scale), (eq51_e426_d_b2 * ddt_scale), (eq51_e426_d_b3 * ddt_scale), (eq51_e426_d_b4 * ddt_scale), (eq51_e426_d_b5 * ddt_scale), (eq51_e426_d_b6 * ddt_scale), (eq51_e426_d_b7 * ddt_scale), (eq51_e426_d_b8 * ddt_scale), (eq51_e426_d_b9 * ddt_scale), (eq51_e426_d_b10 * ddt_scale), (eq51_e426_d_b11 * ddt_scale), (eq51_e426_d_b12 * ddt_scale), (eq51_e426_d_b13 * ddt_scale), (eq51_e426_d_b14 * ddt_scale), (eq51_e426_d_b15 * ddt_scale), (eq51_e426_d_b16 * ddt_scale), (eq51_e426_d_b17 * ddt_scale), (eq51_e426_d_b18 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e429;
        let eq51_node_derivatives: [f64; 19] = [eq51_e429_d_n0, eq51_e429_d_n1, eq51_e429_d_n2, eq51_e429_d_n3, eq51_e429_d_n4, eq51_e429_d_n5, eq51_e429_d_n6, eq51_e429_d_n7, eq51_e429_d_n8, eq51_e429_d_n9, eq51_e429_d_n10, eq51_e429_d_n11, eq51_e429_d_n12, eq51_e429_d_n13, eq51_e429_d_n14, eq51_e429_d_n15, eq51_e429_d_n16, eq51_e429_d_n17, eq51_e429_d_n18];
        let eq51_branch_derivatives: [f64; 19] = [eq51_e429_d_b0, eq51_e429_d_b1, eq51_e429_d_b2, eq51_e429_d_b3, eq51_e429_d_b4, eq51_e429_d_b5, eq51_e429_d_b6, eq51_e429_d_b7, eq51_e429_d_b8, eq51_e429_d_b9, eq51_e429_d_b10, eq51_e429_d_b11, eq51_e429_d_b12, eq51_e429_d_b13, eq51_e429_d_b14, eq51_e429_d_b15, eq51_e429_d_b16, eq51_e429_d_b17, eq51_e429_d_b18];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq64_e562, eq64_e562_d_n3,) = {
    if (var_guard44 != 0.0) {
        let eq64_e559: f64 = (p.p67 * (nv3 - 0.0));
        let eq64_e560: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq64_e559);
        (eq64_e560, (p.p67 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e562;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq64_value),
            3,
            multiplicity * (eq64_e562_d_n3),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_cdel_t: f64,
        var_cdel_t_db0: f64,
        var_cdel_t_db1: f64,
        var_cdel_t_db10: f64,
        var_cdel_t_db11: f64,
        var_cdel_t_db12: f64,
        var_cdel_t_db13: f64,
        var_cdel_t_db14: f64,
        var_cdel_t_db15: f64,
        var_cdel_t_db16: f64,
        var_cdel_t_db17: f64,
        var_cdel_t_db18: f64,
        var_cdel_t_db2: f64,
        var_cdel_t_db3: f64,
        var_cdel_t_db4: f64,
        var_cdel_t_db5: f64,
        var_cdel_t_db6: f64,
        var_cdel_t_db7: f64,
        var_cdel_t_db8: f64,
        var_cdel_t_db9: f64,
        var_cdel_t_dn0: f64,
        var_cdel_t_dn1: f64,
        var_cdel_t_dn10: f64,
        var_cdel_t_dn11: f64,
        var_cdel_t_dn12: f64,
        var_cdel_t_dn13: f64,
        var_cdel_t_dn14: f64,
        var_cdel_t_dn15: f64,
        var_cdel_t_dn16: f64,
        var_cdel_t_dn17: f64,
        var_cdel_t_dn18: f64,
        var_cdel_t_dn2: f64,
        var_cdel_t_dn3: f64,
        var_cdel_t_dn4: f64,
        var_cdel_t_dn5: f64,
        var_cdel_t_dn6: f64,
        var_cdel_t_dn7: f64,
        var_cdel_t_dn8: f64,
        var_cdel_t_dn9: f64,
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
        var_cgd_dn16: f64,
        var_cgd_dn17: f64,
        var_cgd_dn18: f64,
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
        var_cgs_dn16: f64,
        var_cgs_dn17: f64,
        var_cgs_dn18: f64,
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
        var_ci_db15: f64,
        var_ci_db16: f64,
        var_ci_db17: f64,
        var_ci_db18: f64,
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
        var_ci_dn16: f64,
        var_ci_dn17: f64,
        var_ci_dn18: f64,
        var_ci_dn2: f64,
        var_ci_dn3: f64,
        var_ci_dn4: f64,
        var_ci_dn5: f64,
        var_ci_dn6: f64,
        var_ci_dn7: f64,
        var_ci_dn8: f64,
        var_ci_dn9: f64,
        var_guard19: f64,
        var_guard20: f64,
        var_guard21: f64,
        var_guard28: f64,
        var_guard29: f64,
        var_guard44: f64,
        var_qgd: f64,
        var_qgd_db0: f64,
        var_qgd_db1: f64,
        var_qgd_db10: f64,
        var_qgd_db11: f64,
        var_qgd_db12: f64,
        var_qgd_db13: f64,
        var_qgd_db14: f64,
        var_qgd_db15: f64,
        var_qgd_db16: f64,
        var_qgd_db17: f64,
        var_qgd_db18: f64,
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
        var_qgd_dn16: f64,
        var_qgd_dn17: f64,
        var_qgd_dn18: f64,
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
        var_qgs_db15: f64,
        var_qgs_db16: f64,
        var_qgs_db17: f64,
        var_qgs_db18: f64,
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
        var_qgs_dn16: f64,
        var_qgs_dn17: f64,
        var_qgs_dn18: f64,
        var_qgs_dn2: f64,
        var_qgs_dn3: f64,
        var_qgs_dn4: f64,
        var_qgs_dn5: f64,
        var_qgs_dn6: f64,
        var_qgs_dn7: f64,
        var_qgs_dn8: f64,
        var_qgs_dn9: f64,
        var_rc1: f64,
        var_rc1_db0: f64,
        var_rc1_db1: f64,
        var_rc1_db10: f64,
        var_rc1_db11: f64,
        var_rc1_db12: f64,
        var_rc1_db13: f64,
        var_rc1_db14: f64,
        var_rc1_db15: f64,
        var_rc1_db16: f64,
        var_rc1_db17: f64,
        var_rc1_db18: f64,
        var_rc1_db2: f64,
        var_rc1_db3: f64,
        var_rc1_db4: f64,
        var_rc1_db5: f64,
        var_rc1_db6: f64,
        var_rc1_db7: f64,
        var_rc1_db8: f64,
        var_rc1_db9: f64,
        var_rc1_dn0: f64,
        var_rc1_dn1: f64,
        var_rc1_dn10: f64,
        var_rc1_dn11: f64,
        var_rc1_dn12: f64,
        var_rc1_dn13: f64,
        var_rc1_dn14: f64,
        var_rc1_dn15: f64,
        var_rc1_dn16: f64,
        var_rc1_dn17: f64,
        var_rc1_dn18: f64,
        var_rc1_dn2: f64,
        var_rc1_dn3: f64,
        var_rc1_dn4: f64,
        var_rc1_dn5: f64,
        var_rc1_dn6: f64,
        var_rc1_dn7: f64,
        var_rc1_dn8: f64,
        var_rc1_dn9: f64,
        var_t0: f64,
        var_t0_db0: f64,
        var_t0_db1: f64,
        var_t0_db10: f64,
        var_t0_db11: f64,
        var_t0_db12: f64,
        var_t0_db13: f64,
        var_t0_db14: f64,
        var_t0_db15: f64,
        var_t0_db16: f64,
        var_t0_db17: f64,
        var_t0_db18: f64,
        var_t0_db2: f64,
        var_t0_db3: f64,
        var_t0_db4: f64,
        var_t0_db5: f64,
        var_t0_db6: f64,
        var_t0_db7: f64,
        var_t0_db8: f64,
        var_t0_db9: f64,
        var_t0_dn0: f64,
        var_t0_dn1: f64,
        var_t0_dn10: f64,
        var_t0_dn11: f64,
        var_t0_dn12: f64,
        var_t0_dn13: f64,
        var_t0_dn14: f64,
        var_t0_dn15: f64,
        var_t0_dn16: f64,
        var_t0_dn17: f64,
        var_t0_dn18: f64,
        var_t0_dn2: f64,
        var_t0_dn3: f64,
        var_t0_dn4: f64,
        var_t0_dn5: f64,
        var_t0_dn6: f64,
        var_t0_dn7: f64,
        var_t0_dn8: f64,
        var_t0_dn9: f64,
        var_t0_rdb0: f64,
        var_t0_rdb1: f64,
        var_t0_rdb10: f64,
        var_t0_rdb11: f64,
        var_t0_rdb12: f64,
        var_t0_rdb13: f64,
        var_t0_rdb14: f64,
        var_t0_rdb15: f64,
        var_t0_rdb16: f64,
        var_t0_rdb17: f64,
        var_t0_rdb18: f64,
        var_t0_rdb2: f64,
        var_t0_rdb3: f64,
        var_t0_rdb4: f64,
        var_t0_rdb5: f64,
        var_t0_rdb6: f64,
        var_t0_rdb7: f64,
        var_t0_rdb8: f64,
        var_t0_rdb9: f64,
        var_t0_rdn0: f64,
        var_t0_rdn1: f64,
        var_t0_rdn10: f64,
        var_t0_rdn11: f64,
        var_t0_rdn12: f64,
        var_t0_rdn13: f64,
        var_t0_rdn14: f64,
        var_t0_rdn15: f64,
        var_t0_rdn16: f64,
        var_t0_rdn17: f64,
        var_t0_rdn18: f64,
        var_t0_rdn2: f64,
        var_t0_rdn3: f64,
        var_t0_rdn4: f64,
        var_t0_rdn5: f64,
        var_t0_rdn6: f64,
        var_t0_rdn7: f64,
        var_t0_rdn8: f64,
        var_t0_rdn9: f64,
        var_t0_rv: f64,
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
        var_vgdc_dn16: f64,
        var_vgdc_dn17: f64,
        var_vgdc_dn18: f64,
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
        var_vgsc_dn16: f64,
        var_vgsc_dn17: f64,
        var_vgsc_dn18: f64,
        var_vgsc_dn2: f64,
        var_vgsc_dn3: f64,
        var_vgsc_dn4: f64,
        var_vgsc_dn5: f64,
        var_vgsc_dn6: f64,
        var_vgsc_dn7: f64,
        var_vgsc_dn8: f64,
        var_vgsc_dn9: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let bi0 = ctx.branch_current(branches[0]);
        let bi1 = ctx.branch_current(branches[1]);
        let bi10 = ctx.branch_current(branches[10]);
        let bi14 = ctx.branch_current(branches[14]);
        let bi18 = ctx.branch_current(branches[18]);
        let eq3_e114: f64 = (p.p56 / 3.0);
        let eq3_e116: f64 = (eq3_e114 * bi0);
        let eq3_e117_q: f64 = eq3_e116;
        stamper.stamp_potential_reactive_branch1(
            branches[0],
            branches[0],
            eq3_e114,
        );
        let (eq7_e125, eq7_e125_d_n0, eq7_e125_d_n1, eq7_e125_d_n2, eq7_e125_d_n3, eq7_e125_d_n4, eq7_e125_d_n5, eq7_e125_d_n6, eq7_e125_d_n7, eq7_e125_d_n8, eq7_e125_d_n9, eq7_e125_d_n10, eq7_e125_d_n11, eq7_e125_d_n12, eq7_e125_d_n13, eq7_e125_d_n14, eq7_e125_d_n15, eq7_e125_d_n16, eq7_e125_d_n17, eq7_e125_d_n18, eq7_e125_d_b0, eq7_e125_d_b1, eq7_e125_d_b2, eq7_e125_d_b3, eq7_e125_d_b4, eq7_e125_d_b5, eq7_e125_d_b6, eq7_e125_d_b7, eq7_e125_d_b8, eq7_e125_d_b9, eq7_e125_d_b10, eq7_e125_d_b11, eq7_e125_d_b12, eq7_e125_d_b13, eq7_e125_d_b14, eq7_e125_d_b15, eq7_e125_d_b16, eq7_e125_d_b17, eq7_e125_d_b18, eq7_e125_q,) = {
    if (var_guard19 != 0.0) {
        let eq7_e123_q: f64 = var_qgd;
        (var_qgd, var_qgd_dn0, var_qgd_dn1, var_qgd_dn2, var_qgd_dn3, var_qgd_dn4, var_qgd_dn5, var_qgd_dn6, var_qgd_dn7, var_qgd_dn8, var_qgd_dn9, var_qgd_dn10, var_qgd_dn11, var_qgd_dn12, var_qgd_dn13, var_qgd_dn14, var_qgd_dn15, var_qgd_dn16, var_qgd_dn17, var_qgd_dn18, var_qgd_db0, var_qgd_db1, var_qgd_db2, var_qgd_db3, var_qgd_db4, var_qgd_db5, var_qgd_db6, var_qgd_db7, var_qgd_db8, var_qgd_db9, var_qgd_db10, var_qgd_db11, var_qgd_db12, var_qgd_db13, var_qgd_db14, var_qgd_db15, var_qgd_db16, var_qgd_db17, var_qgd_db18, eq7_e123_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 19] = [eq7_e125_d_n0, eq7_e125_d_n1, eq7_e125_d_n2, eq7_e125_d_n3, eq7_e125_d_n4, eq7_e125_d_n5, eq7_e125_d_n6, eq7_e125_d_n7, eq7_e125_d_n8, eq7_e125_d_n9, eq7_e125_d_n10, eq7_e125_d_n11, eq7_e125_d_n12, eq7_e125_d_n13, eq7_e125_d_n14, eq7_e125_d_n15, eq7_e125_d_n16, eq7_e125_d_n17, eq7_e125_d_n18];
        let eq7_reactive_branch_derivatives: [f64; 19] = [eq7_e125_d_b0, eq7_e125_d_b1, eq7_e125_d_b2, eq7_e125_d_b3, eq7_e125_d_b4, eq7_e125_d_b5, eq7_e125_d_b6, eq7_e125_d_b7, eq7_e125_d_b8, eq7_e125_d_b9, eq7_e125_d_b10, eq7_e125_d_b11, eq7_e125_d_b12, eq7_e125_d_b13, eq7_e125_d_b14, eq7_e125_d_b15, eq7_e125_d_b16, eq7_e125_d_b17, eq7_e125_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq8_e130, eq8_e130_d_n0, eq8_e130_d_n1, eq8_e130_d_n2, eq8_e130_d_n3, eq8_e130_d_n4, eq8_e130_d_n5, eq8_e130_d_n6, eq8_e130_d_n7, eq8_e130_d_n8, eq8_e130_d_n9, eq8_e130_d_n10, eq8_e130_d_n11, eq8_e130_d_n12, eq8_e130_d_n13, eq8_e130_d_n14, eq8_e130_d_n15, eq8_e130_d_n16, eq8_e130_d_n17, eq8_e130_d_n18, eq8_e130_d_b0, eq8_e130_d_b1, eq8_e130_d_b2, eq8_e130_d_b3, eq8_e130_d_b4, eq8_e130_d_b5, eq8_e130_d_b6, eq8_e130_d_b7, eq8_e130_d_b8, eq8_e130_d_b9, eq8_e130_d_b10, eq8_e130_d_b11, eq8_e130_d_b12, eq8_e130_d_b13, eq8_e130_d_b14, eq8_e130_d_b15, eq8_e130_d_b16, eq8_e130_d_b17, eq8_e130_d_b18, eq8_e130_q,) = {
    if (var_guard19 != 0.0) {
        let eq8_e128_q: f64 = var_qgs;
        (var_qgs, var_qgs_dn0, var_qgs_dn1, var_qgs_dn2, var_qgs_dn3, var_qgs_dn4, var_qgs_dn5, var_qgs_dn6, var_qgs_dn7, var_qgs_dn8, var_qgs_dn9, var_qgs_dn10, var_qgs_dn11, var_qgs_dn12, var_qgs_dn13, var_qgs_dn14, var_qgs_dn15, var_qgs_dn16, var_qgs_dn17, var_qgs_dn18, var_qgs_db0, var_qgs_db1, var_qgs_db2, var_qgs_db3, var_qgs_db4, var_qgs_db5, var_qgs_db6, var_qgs_db7, var_qgs_db8, var_qgs_db9, var_qgs_db10, var_qgs_db11, var_qgs_db12, var_qgs_db13, var_qgs_db14, var_qgs_db15, var_qgs_db16, var_qgs_db17, var_qgs_db18, eq8_e128_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 19] = [eq8_e130_d_n0, eq8_e130_d_n1, eq8_e130_d_n2, eq8_e130_d_n3, eq8_e130_d_n4, eq8_e130_d_n5, eq8_e130_d_n6, eq8_e130_d_n7, eq8_e130_d_n8, eq8_e130_d_n9, eq8_e130_d_n10, eq8_e130_d_n11, eq8_e130_d_n12, eq8_e130_d_n13, eq8_e130_d_n14, eq8_e130_d_n15, eq8_e130_d_n16, eq8_e130_d_n17, eq8_e130_d_n18];
        let eq8_reactive_branch_derivatives: [f64; 19] = [eq8_e130_d_b0, eq8_e130_d_b1, eq8_e130_d_b2, eq8_e130_d_b3, eq8_e130_d_b4, eq8_e130_d_b5, eq8_e130_d_b6, eq8_e130_d_b7, eq8_e130_d_b8, eq8_e130_d_b9, eq8_e130_d_b10, eq8_e130_d_b11, eq8_e130_d_b12, eq8_e130_d_b13, eq8_e130_d_b14, eq8_e130_d_b15, eq8_e130_d_b16, eq8_e130_d_b17, eq8_e130_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq9_e138, eq9_e138_d_n0, eq9_e138_d_n1, eq9_e138_d_n2, eq9_e138_d_n3, eq9_e138_d_n4, eq9_e138_d_n5, eq9_e138_d_n6, eq9_e138_d_n7, eq9_e138_d_n8, eq9_e138_d_n9, eq9_e138_d_n10, eq9_e138_d_n11, eq9_e138_d_n12, eq9_e138_d_n13, eq9_e138_d_n14, eq9_e138_d_n15, eq9_e138_d_n16, eq9_e138_d_n17, eq9_e138_d_n18, eq9_e138_d_b0, eq9_e138_d_b1, eq9_e138_d_b2, eq9_e138_d_b3, eq9_e138_d_b4, eq9_e138_d_b5, eq9_e138_d_b6, eq9_e138_d_b7, eq9_e138_d_b8, eq9_e138_d_b9, eq9_e138_d_b10, eq9_e138_d_b11, eq9_e138_d_b12, eq9_e138_d_b13, eq9_e138_d_b14, eq9_e138_d_b15, eq9_e138_d_b16, eq9_e138_d_b17, eq9_e138_d_b18, eq9_e138_q,) = {
    if (var_guard19 == 0.0) {
        let eq9_e135: f64 = (var_cgd * var_vgdc);
        let eq9_e135_d_n0: f64 = ((var_cgd_dn0 * var_vgdc) + (var_cgd * var_vgdc_dn0));
        let eq9_e135_d_n1: f64 = ((var_cgd_dn1 * var_vgdc) + (var_cgd * var_vgdc_dn1));
        let eq9_e135_d_n2: f64 = ((var_cgd_dn2 * var_vgdc) + (var_cgd * var_vgdc_dn2));
        let eq9_e135_d_n3: f64 = ((var_cgd_dn3 * var_vgdc) + (var_cgd * var_vgdc_dn3));
        let eq9_e135_d_n4: f64 = ((var_cgd_dn4 * var_vgdc) + (var_cgd * var_vgdc_dn4));
        let eq9_e135_d_n5: f64 = ((var_cgd_dn5 * var_vgdc) + (var_cgd * var_vgdc_dn5));
        let eq9_e135_d_n6: f64 = ((var_cgd_dn6 * var_vgdc) + (var_cgd * var_vgdc_dn6));
        let eq9_e135_d_n7: f64 = ((var_cgd_dn7 * var_vgdc) + (var_cgd * var_vgdc_dn7));
        let eq9_e135_d_n8: f64 = ((var_cgd_dn8 * var_vgdc) + (var_cgd * var_vgdc_dn8));
        let eq9_e135_d_n9: f64 = ((var_cgd_dn9 * var_vgdc) + (var_cgd * var_vgdc_dn9));
        let eq9_e135_d_n10: f64 = ((var_cgd_dn10 * var_vgdc) + (var_cgd * var_vgdc_dn10));
        let eq9_e135_d_n11: f64 = ((var_cgd_dn11 * var_vgdc) + (var_cgd * var_vgdc_dn11));
        let eq9_e135_d_n12: f64 = ((var_cgd_dn12 * var_vgdc) + (var_cgd * var_vgdc_dn12));
        let eq9_e135_d_n13: f64 = ((var_cgd_dn13 * var_vgdc) + (var_cgd * var_vgdc_dn13));
        let eq9_e135_d_n14: f64 = ((var_cgd_dn14 * var_vgdc) + (var_cgd * var_vgdc_dn14));
        let eq9_e135_d_n15: f64 = ((var_cgd_dn15 * var_vgdc) + (var_cgd * var_vgdc_dn15));
        let eq9_e135_d_n16: f64 = ((var_cgd_dn16 * var_vgdc) + (var_cgd * var_vgdc_dn16));
        let eq9_e135_d_n17: f64 = ((var_cgd_dn17 * var_vgdc) + (var_cgd * var_vgdc_dn17));
        let eq9_e135_d_n18: f64 = ((var_cgd_dn18 * var_vgdc) + (var_cgd * var_vgdc_dn18));
        let eq9_e135_d_b0: f64 = ((var_cgd_db0 * var_vgdc) + (var_cgd * var_vgdc_db0));
        let eq9_e135_d_b1: f64 = ((var_cgd_db1 * var_vgdc) + (var_cgd * var_vgdc_db1));
        let eq9_e135_d_b2: f64 = ((var_cgd_db2 * var_vgdc) + (var_cgd * var_vgdc_db2));
        let eq9_e135_d_b3: f64 = ((var_cgd_db3 * var_vgdc) + (var_cgd * var_vgdc_db3));
        let eq9_e135_d_b4: f64 = ((var_cgd_db4 * var_vgdc) + (var_cgd * var_vgdc_db4));
        let eq9_e135_d_b5: f64 = ((var_cgd_db5 * var_vgdc) + (var_cgd * var_vgdc_db5));
        let eq9_e135_d_b6: f64 = ((var_cgd_db6 * var_vgdc) + (var_cgd * var_vgdc_db6));
        let eq9_e135_d_b7: f64 = ((var_cgd_db7 * var_vgdc) + (var_cgd * var_vgdc_db7));
        let eq9_e135_d_b8: f64 = ((var_cgd_db8 * var_vgdc) + (var_cgd * var_vgdc_db8));
        let eq9_e135_d_b9: f64 = ((var_cgd_db9 * var_vgdc) + (var_cgd * var_vgdc_db9));
        let eq9_e135_d_b10: f64 = ((var_cgd_db10 * var_vgdc) + (var_cgd * var_vgdc_db10));
        let eq9_e135_d_b11: f64 = ((var_cgd_db11 * var_vgdc) + (var_cgd * var_vgdc_db11));
        let eq9_e135_d_b12: f64 = ((var_cgd_db12 * var_vgdc) + (var_cgd * var_vgdc_db12));
        let eq9_e135_d_b13: f64 = ((var_cgd_db13 * var_vgdc) + (var_cgd * var_vgdc_db13));
        let eq9_e135_d_b14: f64 = ((var_cgd_db14 * var_vgdc) + (var_cgd * var_vgdc_db14));
        let eq9_e135_d_b15: f64 = ((var_cgd_db15 * var_vgdc) + (var_cgd * var_vgdc_db15));
        let eq9_e135_d_b16: f64 = ((var_cgd_db16 * var_vgdc) + (var_cgd * var_vgdc_db16));
        let eq9_e135_d_b17: f64 = ((var_cgd_db17 * var_vgdc) + (var_cgd * var_vgdc_db17));
        let eq9_e135_d_b18: f64 = ((var_cgd_db18 * var_vgdc) + (var_cgd * var_vgdc_db18));
        let eq9_e136_q: f64 = eq9_e135;
        (eq9_e135, eq9_e135_d_n0, eq9_e135_d_n1, eq9_e135_d_n2, eq9_e135_d_n3, eq9_e135_d_n4, eq9_e135_d_n5, eq9_e135_d_n6, eq9_e135_d_n7, eq9_e135_d_n8, eq9_e135_d_n9, eq9_e135_d_n10, eq9_e135_d_n11, eq9_e135_d_n12, eq9_e135_d_n13, eq9_e135_d_n14, eq9_e135_d_n15, eq9_e135_d_n16, eq9_e135_d_n17, eq9_e135_d_n18, eq9_e135_d_b0, eq9_e135_d_b1, eq9_e135_d_b2, eq9_e135_d_b3, eq9_e135_d_b4, eq9_e135_d_b5, eq9_e135_d_b6, eq9_e135_d_b7, eq9_e135_d_b8, eq9_e135_d_b9, eq9_e135_d_b10, eq9_e135_d_b11, eq9_e135_d_b12, eq9_e135_d_b13, eq9_e135_d_b14, eq9_e135_d_b15, eq9_e135_d_b16, eq9_e135_d_b17, eq9_e135_d_b18, eq9_e136_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_reactive_node_derivatives: [f64; 19] = [eq9_e138_d_n0, eq9_e138_d_n1, eq9_e138_d_n2, eq9_e138_d_n3, eq9_e138_d_n4, eq9_e138_d_n5, eq9_e138_d_n6, eq9_e138_d_n7, eq9_e138_d_n8, eq9_e138_d_n9, eq9_e138_d_n10, eq9_e138_d_n11, eq9_e138_d_n12, eq9_e138_d_n13, eq9_e138_d_n14, eq9_e138_d_n15, eq9_e138_d_n16, eq9_e138_d_n17, eq9_e138_d_n18];
        let eq9_reactive_branch_derivatives: [f64; 19] = [eq9_e138_d_b0, eq9_e138_d_b1, eq9_e138_d_b2, eq9_e138_d_b3, eq9_e138_d_b4, eq9_e138_d_b5, eq9_e138_d_b6, eq9_e138_d_b7, eq9_e138_d_b8, eq9_e138_d_b9, eq9_e138_d_b10, eq9_e138_d_b11, eq9_e138_d_b12, eq9_e138_d_b13, eq9_e138_d_b14, eq9_e138_d_b15, eq9_e138_d_b16, eq9_e138_d_b17, eq9_e138_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq9_reactive_node_derivatives,
            branches,
            &eq9_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq10_e146, eq10_e146_d_n0, eq10_e146_d_n1, eq10_e146_d_n2, eq10_e146_d_n3, eq10_e146_d_n4, eq10_e146_d_n5, eq10_e146_d_n6, eq10_e146_d_n7, eq10_e146_d_n8, eq10_e146_d_n9, eq10_e146_d_n10, eq10_e146_d_n11, eq10_e146_d_n12, eq10_e146_d_n13, eq10_e146_d_n14, eq10_e146_d_n15, eq10_e146_d_n16, eq10_e146_d_n17, eq10_e146_d_n18, eq10_e146_d_b0, eq10_e146_d_b1, eq10_e146_d_b2, eq10_e146_d_b3, eq10_e146_d_b4, eq10_e146_d_b5, eq10_e146_d_b6, eq10_e146_d_b7, eq10_e146_d_b8, eq10_e146_d_b9, eq10_e146_d_b10, eq10_e146_d_b11, eq10_e146_d_b12, eq10_e146_d_b13, eq10_e146_d_b14, eq10_e146_d_b15, eq10_e146_d_b16, eq10_e146_d_b17, eq10_e146_d_b18, eq10_e146_q,) = {
    if (var_guard19 == 0.0) {
        let eq10_e143: f64 = (var_cgs * var_vgsc);
        let eq10_e143_d_n0: f64 = ((var_cgs_dn0 * var_vgsc) + (var_cgs * var_vgsc_dn0));
        let eq10_e143_d_n1: f64 = ((var_cgs_dn1 * var_vgsc) + (var_cgs * var_vgsc_dn1));
        let eq10_e143_d_n2: f64 = ((var_cgs_dn2 * var_vgsc) + (var_cgs * var_vgsc_dn2));
        let eq10_e143_d_n3: f64 = ((var_cgs_dn3 * var_vgsc) + (var_cgs * var_vgsc_dn3));
        let eq10_e143_d_n4: f64 = ((var_cgs_dn4 * var_vgsc) + (var_cgs * var_vgsc_dn4));
        let eq10_e143_d_n5: f64 = ((var_cgs_dn5 * var_vgsc) + (var_cgs * var_vgsc_dn5));
        let eq10_e143_d_n6: f64 = ((var_cgs_dn6 * var_vgsc) + (var_cgs * var_vgsc_dn6));
        let eq10_e143_d_n7: f64 = ((var_cgs_dn7 * var_vgsc) + (var_cgs * var_vgsc_dn7));
        let eq10_e143_d_n8: f64 = ((var_cgs_dn8 * var_vgsc) + (var_cgs * var_vgsc_dn8));
        let eq10_e143_d_n9: f64 = ((var_cgs_dn9 * var_vgsc) + (var_cgs * var_vgsc_dn9));
        let eq10_e143_d_n10: f64 = ((var_cgs_dn10 * var_vgsc) + (var_cgs * var_vgsc_dn10));
        let eq10_e143_d_n11: f64 = ((var_cgs_dn11 * var_vgsc) + (var_cgs * var_vgsc_dn11));
        let eq10_e143_d_n12: f64 = ((var_cgs_dn12 * var_vgsc) + (var_cgs * var_vgsc_dn12));
        let eq10_e143_d_n13: f64 = ((var_cgs_dn13 * var_vgsc) + (var_cgs * var_vgsc_dn13));
        let eq10_e143_d_n14: f64 = ((var_cgs_dn14 * var_vgsc) + (var_cgs * var_vgsc_dn14));
        let eq10_e143_d_n15: f64 = ((var_cgs_dn15 * var_vgsc) + (var_cgs * var_vgsc_dn15));
        let eq10_e143_d_n16: f64 = ((var_cgs_dn16 * var_vgsc) + (var_cgs * var_vgsc_dn16));
        let eq10_e143_d_n17: f64 = ((var_cgs_dn17 * var_vgsc) + (var_cgs * var_vgsc_dn17));
        let eq10_e143_d_n18: f64 = ((var_cgs_dn18 * var_vgsc) + (var_cgs * var_vgsc_dn18));
        let eq10_e143_d_b0: f64 = ((var_cgs_db0 * var_vgsc) + (var_cgs * var_vgsc_db0));
        let eq10_e143_d_b1: f64 = ((var_cgs_db1 * var_vgsc) + (var_cgs * var_vgsc_db1));
        let eq10_e143_d_b2: f64 = ((var_cgs_db2 * var_vgsc) + (var_cgs * var_vgsc_db2));
        let eq10_e143_d_b3: f64 = ((var_cgs_db3 * var_vgsc) + (var_cgs * var_vgsc_db3));
        let eq10_e143_d_b4: f64 = ((var_cgs_db4 * var_vgsc) + (var_cgs * var_vgsc_db4));
        let eq10_e143_d_b5: f64 = ((var_cgs_db5 * var_vgsc) + (var_cgs * var_vgsc_db5));
        let eq10_e143_d_b6: f64 = ((var_cgs_db6 * var_vgsc) + (var_cgs * var_vgsc_db6));
        let eq10_e143_d_b7: f64 = ((var_cgs_db7 * var_vgsc) + (var_cgs * var_vgsc_db7));
        let eq10_e143_d_b8: f64 = ((var_cgs_db8 * var_vgsc) + (var_cgs * var_vgsc_db8));
        let eq10_e143_d_b9: f64 = ((var_cgs_db9 * var_vgsc) + (var_cgs * var_vgsc_db9));
        let eq10_e143_d_b10: f64 = ((var_cgs_db10 * var_vgsc) + (var_cgs * var_vgsc_db10));
        let eq10_e143_d_b11: f64 = ((var_cgs_db11 * var_vgsc) + (var_cgs * var_vgsc_db11));
        let eq10_e143_d_b12: f64 = ((var_cgs_db12 * var_vgsc) + (var_cgs * var_vgsc_db12));
        let eq10_e143_d_b13: f64 = ((var_cgs_db13 * var_vgsc) + (var_cgs * var_vgsc_db13));
        let eq10_e143_d_b14: f64 = ((var_cgs_db14 * var_vgsc) + (var_cgs * var_vgsc_db14));
        let eq10_e143_d_b15: f64 = ((var_cgs_db15 * var_vgsc) + (var_cgs * var_vgsc_db15));
        let eq10_e143_d_b16: f64 = ((var_cgs_db16 * var_vgsc) + (var_cgs * var_vgsc_db16));
        let eq10_e143_d_b17: f64 = ((var_cgs_db17 * var_vgsc) + (var_cgs * var_vgsc_db17));
        let eq10_e143_d_b18: f64 = ((var_cgs_db18 * var_vgsc) + (var_cgs * var_vgsc_db18));
        let eq10_e144_q: f64 = eq10_e143;
        (eq10_e143, eq10_e143_d_n0, eq10_e143_d_n1, eq10_e143_d_n2, eq10_e143_d_n3, eq10_e143_d_n4, eq10_e143_d_n5, eq10_e143_d_n6, eq10_e143_d_n7, eq10_e143_d_n8, eq10_e143_d_n9, eq10_e143_d_n10, eq10_e143_d_n11, eq10_e143_d_n12, eq10_e143_d_n13, eq10_e143_d_n14, eq10_e143_d_n15, eq10_e143_d_n16, eq10_e143_d_n17, eq10_e143_d_n18, eq10_e143_d_b0, eq10_e143_d_b1, eq10_e143_d_b2, eq10_e143_d_b3, eq10_e143_d_b4, eq10_e143_d_b5, eq10_e143_d_b6, eq10_e143_d_b7, eq10_e143_d_b8, eq10_e143_d_b9, eq10_e143_d_b10, eq10_e143_d_b11, eq10_e143_d_b12, eq10_e143_d_b13, eq10_e143_d_b14, eq10_e143_d_b15, eq10_e143_d_b16, eq10_e143_d_b17, eq10_e143_d_b18, eq10_e144_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_reactive_node_derivatives: [f64; 19] = [eq10_e146_d_n0, eq10_e146_d_n1, eq10_e146_d_n2, eq10_e146_d_n3, eq10_e146_d_n4, eq10_e146_d_n5, eq10_e146_d_n6, eq10_e146_d_n7, eq10_e146_d_n8, eq10_e146_d_n9, eq10_e146_d_n10, eq10_e146_d_n11, eq10_e146_d_n12, eq10_e146_d_n13, eq10_e146_d_n14, eq10_e146_d_n15, eq10_e146_d_n16, eq10_e146_d_n17, eq10_e146_d_n18];
        let eq10_reactive_branch_derivatives: [f64; 19] = [eq10_e146_d_b0, eq10_e146_d_b1, eq10_e146_d_b2, eq10_e146_d_b3, eq10_e146_d_b4, eq10_e146_d_b5, eq10_e146_d_b6, eq10_e146_d_b7, eq10_e146_d_b8, eq10_e146_d_b9, eq10_e146_d_b10, eq10_e146_d_b11, eq10_e146_d_b12, eq10_e146_d_b13, eq10_e146_d_b14, eq10_e146_d_b15, eq10_e146_d_b16, eq10_e146_d_b17, eq10_e146_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq15_e169, eq15_e169_d_n0, eq15_e169_d_n1, eq15_e169_d_n2, eq15_e169_d_n3, eq15_e169_d_n4, eq15_e169_d_n5, eq15_e169_d_n6, eq15_e169_d_n7, eq15_e169_d_n8, eq15_e169_d_n9, eq15_e169_d_n10, eq15_e169_d_n11, eq15_e169_d_n12, eq15_e169_d_n13, eq15_e169_d_n14, eq15_e169_d_n15, eq15_e169_d_n16, eq15_e169_d_n17, eq15_e169_d_n18, eq15_e169_d_b0, eq15_e169_d_b1, eq15_e169_d_b2, eq15_e169_d_b3, eq15_e169_d_b4, eq15_e169_d_b5, eq15_e169_d_b6, eq15_e169_d_b7, eq15_e169_d_b8, eq15_e169_d_b9, eq15_e169_d_b10, eq15_e169_d_b11, eq15_e169_d_b12, eq15_e169_d_b13, eq15_e169_d_b14, eq15_e169_d_b15, eq15_e169_d_b16, eq15_e169_d_b17, eq15_e169_d_b18, eq15_e169_q, eq15_e169_q_d_n0, eq15_e169_q_d_n1, eq15_e169_q_d_n2, eq15_e169_q_d_n3, eq15_e169_q_d_n4, eq15_e169_q_d_n5, eq15_e169_q_d_n6, eq15_e169_q_d_n7, eq15_e169_q_d_n8, eq15_e169_q_d_n9, eq15_e169_q_d_n10, eq15_e169_q_d_n11, eq15_e169_q_d_n12, eq15_e169_q_d_n13, eq15_e169_q_d_n14, eq15_e169_q_d_n15, eq15_e169_q_d_n16, eq15_e169_q_d_n17, eq15_e169_q_d_n18, eq15_e169_q_d_b0, eq15_e169_q_d_b1, eq15_e169_q_d_b2, eq15_e169_q_d_b3, eq15_e169_q_d_b4, eq15_e169_q_d_b5, eq15_e169_q_d_b6, eq15_e169_q_d_b7, eq15_e169_q_d_b8, eq15_e169_q_d_b9, eq15_e169_q_d_b10, eq15_e169_q_d_b11, eq15_e169_q_d_b12, eq15_e169_q_d_b13, eq15_e169_q_d_b14, eq15_e169_q_d_b15, eq15_e169_q_d_b16, eq15_e169_q_d_b17, eq15_e169_q_d_b18,) = {
    if (var_guard20 != 0.0) {
        let eq15_e165: f64 = (bi1 * var_rc1);
        let eq15_e165_d_n0: f64 = (bi1 * var_rc1_dn0);
        let eq15_e165_d_n1: f64 = (bi1 * var_rc1_dn1);
        let eq15_e165_d_n2: f64 = (bi1 * var_rc1_dn2);
        let eq15_e165_d_n3: f64 = (bi1 * var_rc1_dn3);
        let eq15_e165_d_n4: f64 = (bi1 * var_rc1_dn4);
        let eq15_e165_d_n5: f64 = (bi1 * var_rc1_dn5);
        let eq15_e165_d_n6: f64 = (bi1 * var_rc1_dn6);
        let eq15_e165_d_n7: f64 = (bi1 * var_rc1_dn7);
        let eq15_e165_d_n8: f64 = (bi1 * var_rc1_dn8);
        let eq15_e165_d_n9: f64 = (bi1 * var_rc1_dn9);
        let eq15_e165_d_n10: f64 = (bi1 * var_rc1_dn10);
        let eq15_e165_d_n11: f64 = (bi1 * var_rc1_dn11);
        let eq15_e165_d_n12: f64 = (bi1 * var_rc1_dn12);
        let eq15_e165_d_n13: f64 = (bi1 * var_rc1_dn13);
        let eq15_e165_d_n14: f64 = (bi1 * var_rc1_dn14);
        let eq15_e165_d_n15: f64 = (bi1 * var_rc1_dn15);
        let eq15_e165_d_n16: f64 = (bi1 * var_rc1_dn16);
        let eq15_e165_d_n17: f64 = (bi1 * var_rc1_dn17);
        let eq15_e165_d_n18: f64 = (bi1 * var_rc1_dn18);
        let eq15_e165_d_b0: f64 = (bi1 * var_rc1_db0);
        let eq15_e165_d_b1: f64 = (var_rc1 + (bi1 * var_rc1_db1));
        let eq15_e165_d_b2: f64 = (bi1 * var_rc1_db2);
        let eq15_e165_d_b3: f64 = (bi1 * var_rc1_db3);
        let eq15_e165_d_b4: f64 = (bi1 * var_rc1_db4);
        let eq15_e165_d_b5: f64 = (bi1 * var_rc1_db5);
        let eq15_e165_d_b6: f64 = (bi1 * var_rc1_db6);
        let eq15_e165_d_b7: f64 = (bi1 * var_rc1_db7);
        let eq15_e165_d_b8: f64 = (bi1 * var_rc1_db8);
        let eq15_e165_d_b9: f64 = (bi1 * var_rc1_db9);
        let eq15_e165_d_b10: f64 = (bi1 * var_rc1_db10);
        let eq15_e165_d_b11: f64 = (bi1 * var_rc1_db11);
        let eq15_e165_d_b12: f64 = (bi1 * var_rc1_db12);
        let eq15_e165_d_b13: f64 = (bi1 * var_rc1_db13);
        let eq15_e165_d_b14: f64 = (bi1 * var_rc1_db14);
        let eq15_e165_d_b15: f64 = (bi1 * var_rc1_db15);
        let eq15_e165_d_b16: f64 = (bi1 * var_rc1_db16);
        let eq15_e165_d_b17: f64 = (bi1 * var_rc1_db17);
        let eq15_e165_d_b18: f64 = (bi1 * var_rc1_db18);
        let eq15_e166_q: f64 = var_t0_rv;
        let eq15_e167: f64 = (eq15_e165 + var_t0);
        let eq15_e167_d_n0: f64 = (eq15_e165_d_n0 + var_t0_dn0);
        let eq15_e167_d_n1: f64 = (eq15_e165_d_n1 + var_t0_dn1);
        let eq15_e167_d_n2: f64 = (eq15_e165_d_n2 + var_t0_dn2);
        let eq15_e167_d_n3: f64 = (eq15_e165_d_n3 + var_t0_dn3);
        let eq15_e167_d_n4: f64 = (eq15_e165_d_n4 + var_t0_dn4);
        let eq15_e167_d_n5: f64 = (eq15_e165_d_n5 + var_t0_dn5);
        let eq15_e167_d_n6: f64 = (eq15_e165_d_n6 + var_t0_dn6);
        let eq15_e167_d_n7: f64 = (eq15_e165_d_n7 + var_t0_dn7);
        let eq15_e167_d_n8: f64 = (eq15_e165_d_n8 + var_t0_dn8);
        let eq15_e167_d_n9: f64 = (eq15_e165_d_n9 + var_t0_dn9);
        let eq15_e167_d_n10: f64 = (eq15_e165_d_n10 + var_t0_dn10);
        let eq15_e167_d_n11: f64 = (eq15_e165_d_n11 + var_t0_dn11);
        let eq15_e167_d_n12: f64 = (eq15_e165_d_n12 + var_t0_dn12);
        let eq15_e167_d_n13: f64 = (eq15_e165_d_n13 + var_t0_dn13);
        let eq15_e167_d_n14: f64 = (eq15_e165_d_n14 + var_t0_dn14);
        let eq15_e167_d_n15: f64 = (eq15_e165_d_n15 + var_t0_dn15);
        let eq15_e167_d_n16: f64 = (eq15_e165_d_n16 + var_t0_dn16);
        let eq15_e167_d_n17: f64 = (eq15_e165_d_n17 + var_t0_dn17);
        let eq15_e167_d_n18: f64 = (eq15_e165_d_n18 + var_t0_dn18);
        let eq15_e167_d_b0: f64 = (eq15_e165_d_b0 + var_t0_db0);
        let eq15_e167_d_b1: f64 = (eq15_e165_d_b1 + var_t0_db1);
        let eq15_e167_d_b2: f64 = (eq15_e165_d_b2 + var_t0_db2);
        let eq15_e167_d_b3: f64 = (eq15_e165_d_b3 + var_t0_db3);
        let eq15_e167_d_b4: f64 = (eq15_e165_d_b4 + var_t0_db4);
        let eq15_e167_d_b5: f64 = (eq15_e165_d_b5 + var_t0_db5);
        let eq15_e167_d_b6: f64 = (eq15_e165_d_b6 + var_t0_db6);
        let eq15_e167_d_b7: f64 = (eq15_e165_d_b7 + var_t0_db7);
        let eq15_e167_d_b8: f64 = (eq15_e165_d_b8 + var_t0_db8);
        let eq15_e167_d_b9: f64 = (eq15_e165_d_b9 + var_t0_db9);
        let eq15_e167_d_b10: f64 = (eq15_e165_d_b10 + var_t0_db10);
        let eq15_e167_d_b11: f64 = (eq15_e165_d_b11 + var_t0_db11);
        let eq15_e167_d_b12: f64 = (eq15_e165_d_b12 + var_t0_db12);
        let eq15_e167_d_b13: f64 = (eq15_e165_d_b13 + var_t0_db13);
        let eq15_e167_d_b14: f64 = (eq15_e165_d_b14 + var_t0_db14);
        let eq15_e167_d_b15: f64 = (eq15_e165_d_b15 + var_t0_db15);
        let eq15_e167_d_b16: f64 = (eq15_e165_d_b16 + var_t0_db16);
        let eq15_e167_d_b17: f64 = (eq15_e165_d_b17 + var_t0_db17);
        let eq15_e167_d_b18: f64 = (eq15_e165_d_b18 + var_t0_db18);
        let eq15_e167_q: f64 = eq15_e166_q;
        (eq15_e167, eq15_e167_d_n0, eq15_e167_d_n1, eq15_e167_d_n2, eq15_e167_d_n3, eq15_e167_d_n4, eq15_e167_d_n5, eq15_e167_d_n6, eq15_e167_d_n7, eq15_e167_d_n8, eq15_e167_d_n9, eq15_e167_d_n10, eq15_e167_d_n11, eq15_e167_d_n12, eq15_e167_d_n13, eq15_e167_d_n14, eq15_e167_d_n15, eq15_e167_d_n16, eq15_e167_d_n17, eq15_e167_d_n18, eq15_e167_d_b0, eq15_e167_d_b1, eq15_e167_d_b2, eq15_e167_d_b3, eq15_e167_d_b4, eq15_e167_d_b5, eq15_e167_d_b6, eq15_e167_d_b7, eq15_e167_d_b8, eq15_e167_d_b9, eq15_e167_d_b10, eq15_e167_d_b11, eq15_e167_d_b12, eq15_e167_d_b13, eq15_e167_d_b14, eq15_e167_d_b15, eq15_e167_d_b16, eq15_e167_d_b17, eq15_e167_d_b18, eq15_e167_q, var_t0_rdn0, var_t0_rdn1, var_t0_rdn2, var_t0_rdn3, var_t0_rdn4, var_t0_rdn5, var_t0_rdn6, var_t0_rdn7, var_t0_rdn8, var_t0_rdn9, var_t0_rdn10, var_t0_rdn11, var_t0_rdn12, var_t0_rdn13, var_t0_rdn14, var_t0_rdn15, var_t0_rdn16, var_t0_rdn17, var_t0_rdn18, var_t0_rdb0, var_t0_rdb1, var_t0_rdb2, var_t0_rdb3, var_t0_rdb4, var_t0_rdb5, var_t0_rdb6, var_t0_rdb7, var_t0_rdb8, var_t0_rdb9, var_t0_rdb10, var_t0_rdb11, var_t0_rdb12, var_t0_rdb13, var_t0_rdb14, var_t0_rdb15, var_t0_rdb16, var_t0_rdb17, var_t0_rdb18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_reactive_node_derivatives: [f64; 19] = [eq15_e169_q_d_n0, eq15_e169_q_d_n1, eq15_e169_q_d_n2, eq15_e169_q_d_n3, eq15_e169_q_d_n4, eq15_e169_q_d_n5, eq15_e169_q_d_n6, eq15_e169_q_d_n7, eq15_e169_q_d_n8, eq15_e169_q_d_n9, eq15_e169_q_d_n10, eq15_e169_q_d_n11, eq15_e169_q_d_n12, eq15_e169_q_d_n13, eq15_e169_q_d_n14, eq15_e169_q_d_n15, eq15_e169_q_d_n16, eq15_e169_q_d_n17, eq15_e169_q_d_n18];
        let eq15_reactive_branch_derivatives: [f64; 19] = [eq15_e169_q_d_b0, eq15_e169_q_d_b1, eq15_e169_q_d_b2, eq15_e169_q_d_b3, eq15_e169_q_d_b4, eq15_e169_q_d_b5, eq15_e169_q_d_b6, eq15_e169_q_d_b7, eq15_e169_q_d_b8, eq15_e169_q_d_b9, eq15_e169_q_d_b10, eq15_e169_q_d_b11, eq15_e169_q_d_b12, eq15_e169_q_d_b13, eq15_e169_q_d_b14, eq15_e169_q_d_b15, eq15_e169_q_d_b16, eq15_e169_q_d_b17, eq15_e169_q_d_b18];
        stamper.stamp_potential_reactive_dense(
            branches[1],
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
        );
        let (eq18_e187, eq18_e187_d_n0, eq18_e187_d_n1, eq18_e187_d_n2, eq18_e187_d_n3, eq18_e187_d_n4, eq18_e187_d_n5, eq18_e187_d_n6, eq18_e187_d_n7, eq18_e187_d_n8, eq18_e187_d_n9, eq18_e187_d_n10, eq18_e187_d_n11, eq18_e187_d_n12, eq18_e187_d_n13, eq18_e187_d_n14, eq18_e187_d_n15, eq18_e187_d_n16, eq18_e187_d_n17, eq18_e187_d_n18, eq18_e187_d_b0, eq18_e187_d_b1, eq18_e187_d_b2, eq18_e187_d_b3, eq18_e187_d_b4, eq18_e187_d_b5, eq18_e187_d_b6, eq18_e187_d_b7, eq18_e187_d_b8, eq18_e187_d_b9, eq18_e187_d_b10, eq18_e187_d_b11, eq18_e187_d_b12, eq18_e187_d_b13, eq18_e187_d_b14, eq18_e187_d_b15, eq18_e187_d_b16, eq18_e187_d_b17, eq18_e187_d_b18, eq18_e187_q,) = {
    if (var_guard21 != 0.0) {
        let eq18_e184: f64 = (var_cdel_t * (nv12 - nv8));
        let eq18_e184_d_n0: f64 = (var_cdel_t_dn0 * (nv12 - nv8));
        let eq18_e184_d_n1: f64 = (var_cdel_t_dn1 * (nv12 - nv8));
        let eq18_e184_d_n2: f64 = (var_cdel_t_dn2 * (nv12 - nv8));
        let eq18_e184_d_n3: f64 = (var_cdel_t_dn3 * (nv12 - nv8));
        let eq18_e184_d_n4: f64 = (var_cdel_t_dn4 * (nv12 - nv8));
        let eq18_e184_d_n5: f64 = (var_cdel_t_dn5 * (nv12 - nv8));
        let eq18_e184_d_n6: f64 = (var_cdel_t_dn6 * (nv12 - nv8));
        let eq18_e184_d_n7: f64 = (var_cdel_t_dn7 * (nv12 - nv8));
        let eq18_e184_d_n8: f64 = ((var_cdel_t_dn8 * (nv12 - nv8)) + (-var_cdel_t));
        let eq18_e184_d_n9: f64 = (var_cdel_t_dn9 * (nv12 - nv8));
        let eq18_e184_d_n10: f64 = (var_cdel_t_dn10 * (nv12 - nv8));
        let eq18_e184_d_n11: f64 = (var_cdel_t_dn11 * (nv12 - nv8));
        let eq18_e184_d_n12: f64 = ((var_cdel_t_dn12 * (nv12 - nv8)) + var_cdel_t);
        let eq18_e184_d_n13: f64 = (var_cdel_t_dn13 * (nv12 - nv8));
        let eq18_e184_d_n14: f64 = (var_cdel_t_dn14 * (nv12 - nv8));
        let eq18_e184_d_n15: f64 = (var_cdel_t_dn15 * (nv12 - nv8));
        let eq18_e184_d_n16: f64 = (var_cdel_t_dn16 * (nv12 - nv8));
        let eq18_e184_d_n17: f64 = (var_cdel_t_dn17 * (nv12 - nv8));
        let eq18_e184_d_n18: f64 = (var_cdel_t_dn18 * (nv12 - nv8));
        let eq18_e184_d_b0: f64 = (var_cdel_t_db0 * (nv12 - nv8));
        let eq18_e184_d_b1: f64 = (var_cdel_t_db1 * (nv12 - nv8));
        let eq18_e184_d_b2: f64 = (var_cdel_t_db2 * (nv12 - nv8));
        let eq18_e184_d_b3: f64 = (var_cdel_t_db3 * (nv12 - nv8));
        let eq18_e184_d_b4: f64 = (var_cdel_t_db4 * (nv12 - nv8));
        let eq18_e184_d_b5: f64 = (var_cdel_t_db5 * (nv12 - nv8));
        let eq18_e184_d_b6: f64 = (var_cdel_t_db6 * (nv12 - nv8));
        let eq18_e184_d_b7: f64 = (var_cdel_t_db7 * (nv12 - nv8));
        let eq18_e184_d_b8: f64 = (var_cdel_t_db8 * (nv12 - nv8));
        let eq18_e184_d_b9: f64 = (var_cdel_t_db9 * (nv12 - nv8));
        let eq18_e184_d_b10: f64 = (var_cdel_t_db10 * (nv12 - nv8));
        let eq18_e184_d_b11: f64 = (var_cdel_t_db11 * (nv12 - nv8));
        let eq18_e184_d_b12: f64 = (var_cdel_t_db12 * (nv12 - nv8));
        let eq18_e184_d_b13: f64 = (var_cdel_t_db13 * (nv12 - nv8));
        let eq18_e184_d_b14: f64 = (var_cdel_t_db14 * (nv12 - nv8));
        let eq18_e184_d_b15: f64 = (var_cdel_t_db15 * (nv12 - nv8));
        let eq18_e184_d_b16: f64 = (var_cdel_t_db16 * (nv12 - nv8));
        let eq18_e184_d_b17: f64 = (var_cdel_t_db17 * (nv12 - nv8));
        let eq18_e184_d_b18: f64 = (var_cdel_t_db18 * (nv12 - nv8));
        let eq18_e185_q: f64 = eq18_e184;
        (eq18_e184, eq18_e184_d_n0, eq18_e184_d_n1, eq18_e184_d_n2, eq18_e184_d_n3, eq18_e184_d_n4, eq18_e184_d_n5, eq18_e184_d_n6, eq18_e184_d_n7, eq18_e184_d_n8, eq18_e184_d_n9, eq18_e184_d_n10, eq18_e184_d_n11, eq18_e184_d_n12, eq18_e184_d_n13, eq18_e184_d_n14, eq18_e184_d_n15, eq18_e184_d_n16, eq18_e184_d_n17, eq18_e184_d_n18, eq18_e184_d_b0, eq18_e184_d_b1, eq18_e184_d_b2, eq18_e184_d_b3, eq18_e184_d_b4, eq18_e184_d_b5, eq18_e184_d_b6, eq18_e184_d_b7, eq18_e184_d_b8, eq18_e184_d_b9, eq18_e184_d_b10, eq18_e184_d_b11, eq18_e184_d_b12, eq18_e184_d_b13, eq18_e184_d_b14, eq18_e184_d_b15, eq18_e184_d_b16, eq18_e184_d_b17, eq18_e184_d_b18, eq18_e185_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_reactive_node_derivatives: [f64; 19] = [eq18_e187_d_n0, eq18_e187_d_n1, eq18_e187_d_n2, eq18_e187_d_n3, eq18_e187_d_n4, eq18_e187_d_n5, eq18_e187_d_n6, eq18_e187_d_n7, eq18_e187_d_n8, eq18_e187_d_n9, eq18_e187_d_n10, eq18_e187_d_n11, eq18_e187_d_n12, eq18_e187_d_n13, eq18_e187_d_n14, eq18_e187_d_n15, eq18_e187_d_n16, eq18_e187_d_n17, eq18_e187_d_n18];
        let eq18_reactive_branch_derivatives: [f64; 19] = [eq18_e187_d_b0, eq18_e187_d_b1, eq18_e187_d_b2, eq18_e187_d_b3, eq18_e187_d_b4, eq18_e187_d_b5, eq18_e187_d_b6, eq18_e187_d_b7, eq18_e187_d_b8, eq18_e187_d_b9, eq18_e187_d_b10, eq18_e187_d_b11, eq18_e187_d_b12, eq18_e187_d_b13, eq18_e187_d_b14, eq18_e187_d_b15, eq18_e187_d_b16, eq18_e187_d_b17, eq18_e187_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[8]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq31_e269: f64 = (p.p54 * bi10);
        let eq31_e270_q: f64 = eq31_e269;
        stamper.stamp_potential_reactive_branch1(
            branches[10],
            branches[10],
            p.p54,
        );
        let eq35_e298: f64 = (p.p53 * bi14);
        let eq35_e299_q: f64 = eq35_e298;
        stamper.stamp_potential_reactive_branch1(
            branches[14],
            branches[14],
            p.p53,
        );
        let eq39_e327: f64 = (p.p52 * bi18);
        let eq39_e328_q: f64 = eq39_e327;
        stamper.stamp_potential_reactive_branch1(
            branches[18],
            branches[18],
            p.p52,
        );
        let (eq51_e429, eq51_e429_d_n0, eq51_e429_d_n1, eq51_e429_d_n2, eq51_e429_d_n3, eq51_e429_d_n4, eq51_e429_d_n5, eq51_e429_d_n6, eq51_e429_d_n7, eq51_e429_d_n8, eq51_e429_d_n9, eq51_e429_d_n10, eq51_e429_d_n11, eq51_e429_d_n12, eq51_e429_d_n13, eq51_e429_d_n14, eq51_e429_d_n15, eq51_e429_d_n16, eq51_e429_d_n17, eq51_e429_d_n18, eq51_e429_d_b0, eq51_e429_d_b1, eq51_e429_d_b2, eq51_e429_d_b3, eq51_e429_d_b4, eq51_e429_d_b5, eq51_e429_d_b6, eq51_e429_d_b7, eq51_e429_d_b8, eq51_e429_d_b9, eq51_e429_d_b10, eq51_e429_d_b11, eq51_e429_d_b12, eq51_e429_d_b13, eq51_e429_d_b14, eq51_e429_d_b15, eq51_e429_d_b16, eq51_e429_d_b17, eq51_e429_d_b18, eq51_e429_q,) = {
    if (((var_guard29 != 0.0) && (var_guard28 == 0.0)) && (p.p0 != 0.0)) {
        let eq51_e424: f64 = (-var_ci);
        let eq51_e426: f64 = (eq51_e424 * (nv17 - 0.0));
        let eq51_e426_d_n0: f64 = ((-var_ci_dn0) * (nv17 - 0.0));
        let eq51_e426_d_n1: f64 = ((-var_ci_dn1) * (nv17 - 0.0));
        let eq51_e426_d_n2: f64 = ((-var_ci_dn2) * (nv17 - 0.0));
        let eq51_e426_d_n3: f64 = ((-var_ci_dn3) * (nv17 - 0.0));
        let eq51_e426_d_n4: f64 = ((-var_ci_dn4) * (nv17 - 0.0));
        let eq51_e426_d_n5: f64 = ((-var_ci_dn5) * (nv17 - 0.0));
        let eq51_e426_d_n6: f64 = ((-var_ci_dn6) * (nv17 - 0.0));
        let eq51_e426_d_n7: f64 = ((-var_ci_dn7) * (nv17 - 0.0));
        let eq51_e426_d_n8: f64 = ((-var_ci_dn8) * (nv17 - 0.0));
        let eq51_e426_d_n9: f64 = ((-var_ci_dn9) * (nv17 - 0.0));
        let eq51_e426_d_n10: f64 = ((-var_ci_dn10) * (nv17 - 0.0));
        let eq51_e426_d_n11: f64 = ((-var_ci_dn11) * (nv17 - 0.0));
        let eq51_e426_d_n12: f64 = ((-var_ci_dn12) * (nv17 - 0.0));
        let eq51_e426_d_n13: f64 = ((-var_ci_dn13) * (nv17 - 0.0));
        let eq51_e426_d_n14: f64 = ((-var_ci_dn14) * (nv17 - 0.0));
        let eq51_e426_d_n15: f64 = ((-var_ci_dn15) * (nv17 - 0.0));
        let eq51_e426_d_n16: f64 = ((-var_ci_dn16) * (nv17 - 0.0));
        let eq51_e426_d_n17: f64 = (((-var_ci_dn17) * (nv17 - 0.0)) + eq51_e424);
        let eq51_e426_d_n18: f64 = ((-var_ci_dn18) * (nv17 - 0.0));
        let eq51_e426_d_b0: f64 = ((-var_ci_db0) * (nv17 - 0.0));
        let eq51_e426_d_b1: f64 = ((-var_ci_db1) * (nv17 - 0.0));
        let eq51_e426_d_b2: f64 = ((-var_ci_db2) * (nv17 - 0.0));
        let eq51_e426_d_b3: f64 = ((-var_ci_db3) * (nv17 - 0.0));
        let eq51_e426_d_b4: f64 = ((-var_ci_db4) * (nv17 - 0.0));
        let eq51_e426_d_b5: f64 = ((-var_ci_db5) * (nv17 - 0.0));
        let eq51_e426_d_b6: f64 = ((-var_ci_db6) * (nv17 - 0.0));
        let eq51_e426_d_b7: f64 = ((-var_ci_db7) * (nv17 - 0.0));
        let eq51_e426_d_b8: f64 = ((-var_ci_db8) * (nv17 - 0.0));
        let eq51_e426_d_b9: f64 = ((-var_ci_db9) * (nv17 - 0.0));
        let eq51_e426_d_b10: f64 = ((-var_ci_db10) * (nv17 - 0.0));
        let eq51_e426_d_b11: f64 = ((-var_ci_db11) * (nv17 - 0.0));
        let eq51_e426_d_b12: f64 = ((-var_ci_db12) * (nv17 - 0.0));
        let eq51_e426_d_b13: f64 = ((-var_ci_db13) * (nv17 - 0.0));
        let eq51_e426_d_b14: f64 = ((-var_ci_db14) * (nv17 - 0.0));
        let eq51_e426_d_b15: f64 = ((-var_ci_db15) * (nv17 - 0.0));
        let eq51_e426_d_b16: f64 = ((-var_ci_db16) * (nv17 - 0.0));
        let eq51_e426_d_b17: f64 = ((-var_ci_db17) * (nv17 - 0.0));
        let eq51_e426_d_b18: f64 = ((-var_ci_db18) * (nv17 - 0.0));
        let eq51_e427_q: f64 = eq51_e426;
        (eq51_e426, eq51_e426_d_n0, eq51_e426_d_n1, eq51_e426_d_n2, eq51_e426_d_n3, eq51_e426_d_n4, eq51_e426_d_n5, eq51_e426_d_n6, eq51_e426_d_n7, eq51_e426_d_n8, eq51_e426_d_n9, eq51_e426_d_n10, eq51_e426_d_n11, eq51_e426_d_n12, eq51_e426_d_n13, eq51_e426_d_n14, eq51_e426_d_n15, eq51_e426_d_n16, eq51_e426_d_n17, eq51_e426_d_n18, eq51_e426_d_b0, eq51_e426_d_b1, eq51_e426_d_b2, eq51_e426_d_b3, eq51_e426_d_b4, eq51_e426_d_b5, eq51_e426_d_b6, eq51_e426_d_b7, eq51_e426_d_b8, eq51_e426_d_b9, eq51_e426_d_b10, eq51_e426_d_b11, eq51_e426_d_b12, eq51_e426_d_b13, eq51_e426_d_b14, eq51_e426_d_b15, eq51_e426_d_b16, eq51_e426_d_b17, eq51_e426_d_b18, eq51_e427_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 19] = [eq51_e429_d_n0, eq51_e429_d_n1, eq51_e429_d_n2, eq51_e429_d_n3, eq51_e429_d_n4, eq51_e429_d_n5, eq51_e429_d_n6, eq51_e429_d_n7, eq51_e429_d_n8, eq51_e429_d_n9, eq51_e429_d_n10, eq51_e429_d_n11, eq51_e429_d_n12, eq51_e429_d_n13, eq51_e429_d_n14, eq51_e429_d_n15, eq51_e429_d_n16, eq51_e429_d_n17, eq51_e429_d_n18];
        let eq51_reactive_branch_derivatives: [f64; 19] = [eq51_e429_d_b0, eq51_e429_d_b1, eq51_e429_d_b2, eq51_e429_d_b3, eq51_e429_d_b4, eq51_e429_d_b5, eq51_e429_d_b6, eq51_e429_d_b7, eq51_e429_d_b8, eq51_e429_d_b9, eq51_e429_d_b10, eq51_e429_d_b11, eq51_e429_d_b12, eq51_e429_d_b13, eq51_e429_d_b14, eq51_e429_d_b15, eq51_e429_d_b16, eq51_e429_d_b17, eq51_e429_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq64_e562, eq64_e562_d_n3, eq64_e562_q,) = {
    if (var_guard44 != 0.0) {
        let eq64_e559: f64 = (p.p67 * (nv3 - 0.0));
        let eq64_e560_q: f64 = eq64_e559;
        (eq64_e559, p.p67, eq64_e560_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (eq64_e562_d_n3),
        );
    }
}
