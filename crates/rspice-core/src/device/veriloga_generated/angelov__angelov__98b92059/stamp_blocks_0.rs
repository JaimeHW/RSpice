#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_cgd_slot: &mut f64,
        var_cgd_db0_slot: &mut f64,
        var_cgd_db1_slot: &mut f64,
        var_cgd_db10_slot: &mut f64,
        var_cgd_db11_slot: &mut f64,
        var_cgd_db12_slot: &mut f64,
        var_cgd_db13_slot: &mut f64,
        var_cgd_db14_slot: &mut f64,
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
        var_cgs_slot: &mut f64,
        var_cgs_db0_slot: &mut f64,
        var_cgs_db1_slot: &mut f64,
        var_cgs_db10_slot: &mut f64,
        var_cgs_db11_slot: &mut f64,
        var_cgs_db12_slot: &mut f64,
        var_cgs_db13_slot: &mut f64,
        var_cgs_db14_slot: &mut f64,
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
        var_guard1_slot: &mut f64,
        var_guard2_slot: &mut f64,
        var_qgd_slot: &mut f64,
        var_qgd_db0_slot: &mut f64,
        var_qgd_db1_slot: &mut f64,
        var_qgd_db10_slot: &mut f64,
        var_qgd_db11_slot: &mut f64,
        var_qgd_db12_slot: &mut f64,
        var_qgd_db13_slot: &mut f64,
        var_qgd_db14_slot: &mut f64,
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
        var_qgs_slot: &mut f64,
        var_qgs_db0_slot: &mut f64,
        var_qgs_db1_slot: &mut f64,
        var_qgs_db10_slot: &mut f64,
        var_qgs_db11_slot: &mut f64,
        var_qgs_db12_slot: &mut f64,
        var_qgs_db13_slot: &mut f64,
        var_qgs_db14_slot: &mut f64,
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
        var_t_slot: &mut f64,
        var_t_db0_slot: &mut f64,
        var_t_db1_slot: &mut f64,
        var_t_db10_slot: &mut f64,
        var_t_db11_slot: &mut f64,
        var_t_db12_slot: &mut f64,
        var_t_db13_slot: &mut f64,
        var_t_db14_slot: &mut f64,
        var_t_db2_slot: &mut f64,
        var_t_db3_slot: &mut f64,
        var_t_db4_slot: &mut f64,
        var_t_db5_slot: &mut f64,
        var_t_db6_slot: &mut f64,
        var_t_db7_slot: &mut f64,
        var_t_db8_slot: &mut f64,
        var_t_db9_slot: &mut f64,
        var_t_dn0_slot: &mut f64,
        var_t_dn1_slot: &mut f64,
        var_t_dn10_slot: &mut f64,
        var_t_dn11_slot: &mut f64,
        var_t_dn12_slot: &mut f64,
        var_t_dn13_slot: &mut f64,
        var_t_dn14_slot: &mut f64,
        var_t_dn15_slot: &mut f64,
        var_t_dn2_slot: &mut f64,
        var_t_dn3_slot: &mut f64,
        var_t_dn4_slot: &mut f64,
        var_t_dn5_slot: &mut f64,
        var_t_dn6_slot: &mut f64,
        var_t_dn7_slot: &mut f64,
        var_t_dn8_slot: &mut f64,
        var_t_dn9_slot: &mut f64,
        var_t_nom_slot: &mut f64,
        var_vdg_slot: &mut f64,
        var_vdg_db0_slot: &mut f64,
        var_vdg_db1_slot: &mut f64,
        var_vdg_db10_slot: &mut f64,
        var_vdg_db11_slot: &mut f64,
        var_vdg_db12_slot: &mut f64,
        var_vdg_db13_slot: &mut f64,
        var_vdg_db14_slot: &mut f64,
        var_vdg_db2_slot: &mut f64,
        var_vdg_db3_slot: &mut f64,
        var_vdg_db4_slot: &mut f64,
        var_vdg_db5_slot: &mut f64,
        var_vdg_db6_slot: &mut f64,
        var_vdg_db7_slot: &mut f64,
        var_vdg_db8_slot: &mut f64,
        var_vdg_db9_slot: &mut f64,
        var_vdg_dn0_slot: &mut f64,
        var_vdg_dn1_slot: &mut f64,
        var_vdg_dn10_slot: &mut f64,
        var_vdg_dn11_slot: &mut f64,
        var_vdg_dn12_slot: &mut f64,
        var_vdg_dn13_slot: &mut f64,
        var_vdg_dn14_slot: &mut f64,
        var_vdg_dn15_slot: &mut f64,
        var_vdg_dn2_slot: &mut f64,
        var_vdg_dn3_slot: &mut f64,
        var_vdg_dn4_slot: &mut f64,
        var_vdg_dn5_slot: &mut f64,
        var_vdg_dn6_slot: &mut f64,
        var_vdg_dn7_slot: &mut f64,
        var_vdg_dn8_slot: &mut f64,
        var_vdg_dn9_slot: &mut f64,
        var_vds_slot: &mut f64,
        var_vds_db0_slot: &mut f64,
        var_vds_db1_slot: &mut f64,
        var_vds_db10_slot: &mut f64,
        var_vds_db11_slot: &mut f64,
        var_vds_db12_slot: &mut f64,
        var_vds_db13_slot: &mut f64,
        var_vds_db14_slot: &mut f64,
        var_vds_db2_slot: &mut f64,
        var_vds_db3_slot: &mut f64,
        var_vds_db4_slot: &mut f64,
        var_vds_db5_slot: &mut f64,
        var_vds_db6_slot: &mut f64,
        var_vds_db7_slot: &mut f64,
        var_vds_db8_slot: &mut f64,
        var_vds_db9_slot: &mut f64,
        var_vds_dn0_slot: &mut f64,
        var_vds_dn1_slot: &mut f64,
        var_vds_dn10_slot: &mut f64,
        var_vds_dn11_slot: &mut f64,
        var_vds_dn12_slot: &mut f64,
        var_vds_dn13_slot: &mut f64,
        var_vds_dn14_slot: &mut f64,
        var_vds_dn15_slot: &mut f64,
        var_vds_dn2_slot: &mut f64,
        var_vds_dn3_slot: &mut f64,
        var_vds_dn4_slot: &mut f64,
        var_vds_dn5_slot: &mut f64,
        var_vds_dn6_slot: &mut f64,
        var_vds_dn7_slot: &mut f64,
        var_vds_dn8_slot: &mut f64,
        var_vds_dn9_slot: &mut f64,
        var_vgd_slot: &mut f64,
        var_vgd_db0_slot: &mut f64,
        var_vgd_db1_slot: &mut f64,
        var_vgd_db10_slot: &mut f64,
        var_vgd_db11_slot: &mut f64,
        var_vgd_db12_slot: &mut f64,
        var_vgd_db13_slot: &mut f64,
        var_vgd_db14_slot: &mut f64,
        var_vgd_db2_slot: &mut f64,
        var_vgd_db3_slot: &mut f64,
        var_vgd_db4_slot: &mut f64,
        var_vgd_db5_slot: &mut f64,
        var_vgd_db6_slot: &mut f64,
        var_vgd_db7_slot: &mut f64,
        var_vgd_db8_slot: &mut f64,
        var_vgd_db9_slot: &mut f64,
        var_vgd_dn0_slot: &mut f64,
        var_vgd_dn1_slot: &mut f64,
        var_vgd_dn10_slot: &mut f64,
        var_vgd_dn11_slot: &mut f64,
        var_vgd_dn12_slot: &mut f64,
        var_vgd_dn13_slot: &mut f64,
        var_vgd_dn14_slot: &mut f64,
        var_vgd_dn15_slot: &mut f64,
        var_vgd_dn2_slot: &mut f64,
        var_vgd_dn3_slot: &mut f64,
        var_vgd_dn4_slot: &mut f64,
        var_vgd_dn5_slot: &mut f64,
        var_vgd_dn6_slot: &mut f64,
        var_vgd_dn7_slot: &mut f64,
        var_vgd_dn8_slot: &mut f64,
        var_vgd_dn9_slot: &mut f64,
        var_vgdc_slot: &mut f64,
        var_vgdc_db0_slot: &mut f64,
        var_vgdc_db1_slot: &mut f64,
        var_vgdc_db10_slot: &mut f64,
        var_vgdc_db11_slot: &mut f64,
        var_vgdc_db12_slot: &mut f64,
        var_vgdc_db13_slot: &mut f64,
        var_vgdc_db14_slot: &mut f64,
        var_vgdc_db2_slot: &mut f64,
        var_vgdc_db3_slot: &mut f64,
        var_vgdc_db4_slot: &mut f64,
        var_vgdc_db5_slot: &mut f64,
        var_vgdc_db6_slot: &mut f64,
        var_vgdc_db7_slot: &mut f64,
        var_vgdc_db8_slot: &mut f64,
        var_vgdc_db9_slot: &mut f64,
        var_vgdc_dn0_slot: &mut f64,
        var_vgdc_dn1_slot: &mut f64,
        var_vgdc_dn10_slot: &mut f64,
        var_vgdc_dn11_slot: &mut f64,
        var_vgdc_dn12_slot: &mut f64,
        var_vgdc_dn13_slot: &mut f64,
        var_vgdc_dn14_slot: &mut f64,
        var_vgdc_dn15_slot: &mut f64,
        var_vgdc_dn2_slot: &mut f64,
        var_vgdc_dn3_slot: &mut f64,
        var_vgdc_dn4_slot: &mut f64,
        var_vgdc_dn5_slot: &mut f64,
        var_vgdc_dn6_slot: &mut f64,
        var_vgdc_dn7_slot: &mut f64,
        var_vgdc_dn8_slot: &mut f64,
        var_vgdc_dn9_slot: &mut f64,
        var_vgs_slot: &mut f64,
        var_vgs_db0_slot: &mut f64,
        var_vgs_db1_slot: &mut f64,
        var_vgs_db10_slot: &mut f64,
        var_vgs_db11_slot: &mut f64,
        var_vgs_db12_slot: &mut f64,
        var_vgs_db13_slot: &mut f64,
        var_vgs_db14_slot: &mut f64,
        var_vgs_db2_slot: &mut f64,
        var_vgs_db3_slot: &mut f64,
        var_vgs_db4_slot: &mut f64,
        var_vgs_db5_slot: &mut f64,
        var_vgs_db6_slot: &mut f64,
        var_vgs_db7_slot: &mut f64,
        var_vgs_db8_slot: &mut f64,
        var_vgs_db9_slot: &mut f64,
        var_vgs_dn0_slot: &mut f64,
        var_vgs_dn1_slot: &mut f64,
        var_vgs_dn10_slot: &mut f64,
        var_vgs_dn11_slot: &mut f64,
        var_vgs_dn12_slot: &mut f64,
        var_vgs_dn13_slot: &mut f64,
        var_vgs_dn14_slot: &mut f64,
        var_vgs_dn15_slot: &mut f64,
        var_vgs_dn2_slot: &mut f64,
        var_vgs_dn3_slot: &mut f64,
        var_vgs_dn4_slot: &mut f64,
        var_vgs_dn5_slot: &mut f64,
        var_vgs_dn6_slot: &mut f64,
        var_vgs_dn7_slot: &mut f64,
        var_vgs_dn8_slot: &mut f64,
        var_vgs_dn9_slot: &mut f64,
        var_vgsc_slot: &mut f64,
        var_vgsc_db0_slot: &mut f64,
        var_vgsc_db1_slot: &mut f64,
        var_vgsc_db10_slot: &mut f64,
        var_vgsc_db11_slot: &mut f64,
        var_vgsc_db12_slot: &mut f64,
        var_vgsc_db13_slot: &mut f64,
        var_vgsc_db14_slot: &mut f64,
        var_vgsc_db2_slot: &mut f64,
        var_vgsc_db3_slot: &mut f64,
        var_vgsc_db4_slot: &mut f64,
        var_vgsc_db5_slot: &mut f64,
        var_vgsc_db6_slot: &mut f64,
        var_vgsc_db7_slot: &mut f64,
        var_vgsc_db8_slot: &mut f64,
        var_vgsc_db9_slot: &mut f64,
        var_vgsc_dn0_slot: &mut f64,
        var_vgsc_dn1_slot: &mut f64,
        var_vgsc_dn10_slot: &mut f64,
        var_vgsc_dn11_slot: &mut f64,
        var_vgsc_dn12_slot: &mut f64,
        var_vgsc_dn13_slot: &mut f64,
        var_vgsc_dn14_slot: &mut f64,
        var_vgsc_dn15_slot: &mut f64,
        var_vgsc_dn2_slot: &mut f64,
        var_vgsc_dn3_slot: &mut f64,
        var_vgsc_dn4_slot: &mut f64,
        var_vgsc_dn5_slot: &mut f64,
        var_vgsc_dn6_slot: &mut f64,
        var_vgsc_dn7_slot: &mut f64,
        var_vgsc_dn8_slot: &mut f64,
        var_vgsc_dn9_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let mut var_cgd: f64 = *var_cgd_slot;
        let mut var_cgd_db0: f64 = *var_cgd_db0_slot;
        let mut var_cgd_db1: f64 = *var_cgd_db1_slot;
        let mut var_cgd_db10: f64 = *var_cgd_db10_slot;
        let mut var_cgd_db11: f64 = *var_cgd_db11_slot;
        let mut var_cgd_db12: f64 = *var_cgd_db12_slot;
        let mut var_cgd_db13: f64 = *var_cgd_db13_slot;
        let mut var_cgd_db14: f64 = *var_cgd_db14_slot;
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
        let mut var_cgs: f64 = *var_cgs_slot;
        let mut var_cgs_db0: f64 = *var_cgs_db0_slot;
        let mut var_cgs_db1: f64 = *var_cgs_db1_slot;
        let mut var_cgs_db10: f64 = *var_cgs_db10_slot;
        let mut var_cgs_db11: f64 = *var_cgs_db11_slot;
        let mut var_cgs_db12: f64 = *var_cgs_db12_slot;
        let mut var_cgs_db13: f64 = *var_cgs_db13_slot;
        let mut var_cgs_db14: f64 = *var_cgs_db14_slot;
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
        let mut var_guard1: f64 = *var_guard1_slot;
        let mut var_guard2: f64 = *var_guard2_slot;
        let mut var_qgd: f64 = *var_qgd_slot;
        let mut var_qgd_db0: f64 = *var_qgd_db0_slot;
        let mut var_qgd_db1: f64 = *var_qgd_db1_slot;
        let mut var_qgd_db10: f64 = *var_qgd_db10_slot;
        let mut var_qgd_db11: f64 = *var_qgd_db11_slot;
        let mut var_qgd_db12: f64 = *var_qgd_db12_slot;
        let mut var_qgd_db13: f64 = *var_qgd_db13_slot;
        let mut var_qgd_db14: f64 = *var_qgd_db14_slot;
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
        let mut var_qgs: f64 = *var_qgs_slot;
        let mut var_qgs_db0: f64 = *var_qgs_db0_slot;
        let mut var_qgs_db1: f64 = *var_qgs_db1_slot;
        let mut var_qgs_db10: f64 = *var_qgs_db10_slot;
        let mut var_qgs_db11: f64 = *var_qgs_db11_slot;
        let mut var_qgs_db12: f64 = *var_qgs_db12_slot;
        let mut var_qgs_db13: f64 = *var_qgs_db13_slot;
        let mut var_qgs_db14: f64 = *var_qgs_db14_slot;
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
        let mut var_t: f64 = *var_t_slot;
        let mut var_t_db0: f64 = *var_t_db0_slot;
        let mut var_t_db1: f64 = *var_t_db1_slot;
        let mut var_t_db10: f64 = *var_t_db10_slot;
        let mut var_t_db11: f64 = *var_t_db11_slot;
        let mut var_t_db12: f64 = *var_t_db12_slot;
        let mut var_t_db13: f64 = *var_t_db13_slot;
        let mut var_t_db14: f64 = *var_t_db14_slot;
        let mut var_t_db2: f64 = *var_t_db2_slot;
        let mut var_t_db3: f64 = *var_t_db3_slot;
        let mut var_t_db4: f64 = *var_t_db4_slot;
        let mut var_t_db5: f64 = *var_t_db5_slot;
        let mut var_t_db6: f64 = *var_t_db6_slot;
        let mut var_t_db7: f64 = *var_t_db7_slot;
        let mut var_t_db8: f64 = *var_t_db8_slot;
        let mut var_t_db9: f64 = *var_t_db9_slot;
        let mut var_t_dn0: f64 = *var_t_dn0_slot;
        let mut var_t_dn1: f64 = *var_t_dn1_slot;
        let mut var_t_dn10: f64 = *var_t_dn10_slot;
        let mut var_t_dn11: f64 = *var_t_dn11_slot;
        let mut var_t_dn12: f64 = *var_t_dn12_slot;
        let mut var_t_dn13: f64 = *var_t_dn13_slot;
        let mut var_t_dn14: f64 = *var_t_dn14_slot;
        let mut var_t_dn15: f64 = *var_t_dn15_slot;
        let mut var_t_dn2: f64 = *var_t_dn2_slot;
        let mut var_t_dn3: f64 = *var_t_dn3_slot;
        let mut var_t_dn4: f64 = *var_t_dn4_slot;
        let mut var_t_dn5: f64 = *var_t_dn5_slot;
        let mut var_t_dn6: f64 = *var_t_dn6_slot;
        let mut var_t_dn7: f64 = *var_t_dn7_slot;
        let mut var_t_dn8: f64 = *var_t_dn8_slot;
        let mut var_t_dn9: f64 = *var_t_dn9_slot;
        let mut var_t_nom: f64 = *var_t_nom_slot;
        let mut var_vdg: f64 = *var_vdg_slot;
        let mut var_vdg_db0: f64 = *var_vdg_db0_slot;
        let mut var_vdg_db1: f64 = *var_vdg_db1_slot;
        let mut var_vdg_db10: f64 = *var_vdg_db10_slot;
        let mut var_vdg_db11: f64 = *var_vdg_db11_slot;
        let mut var_vdg_db12: f64 = *var_vdg_db12_slot;
        let mut var_vdg_db13: f64 = *var_vdg_db13_slot;
        let mut var_vdg_db14: f64 = *var_vdg_db14_slot;
        let mut var_vdg_db2: f64 = *var_vdg_db2_slot;
        let mut var_vdg_db3: f64 = *var_vdg_db3_slot;
        let mut var_vdg_db4: f64 = *var_vdg_db4_slot;
        let mut var_vdg_db5: f64 = *var_vdg_db5_slot;
        let mut var_vdg_db6: f64 = *var_vdg_db6_slot;
        let mut var_vdg_db7: f64 = *var_vdg_db7_slot;
        let mut var_vdg_db8: f64 = *var_vdg_db8_slot;
        let mut var_vdg_db9: f64 = *var_vdg_db9_slot;
        let mut var_vdg_dn0: f64 = *var_vdg_dn0_slot;
        let mut var_vdg_dn1: f64 = *var_vdg_dn1_slot;
        let mut var_vdg_dn10: f64 = *var_vdg_dn10_slot;
        let mut var_vdg_dn11: f64 = *var_vdg_dn11_slot;
        let mut var_vdg_dn12: f64 = *var_vdg_dn12_slot;
        let mut var_vdg_dn13: f64 = *var_vdg_dn13_slot;
        let mut var_vdg_dn14: f64 = *var_vdg_dn14_slot;
        let mut var_vdg_dn15: f64 = *var_vdg_dn15_slot;
        let mut var_vdg_dn2: f64 = *var_vdg_dn2_slot;
        let mut var_vdg_dn3: f64 = *var_vdg_dn3_slot;
        let mut var_vdg_dn4: f64 = *var_vdg_dn4_slot;
        let mut var_vdg_dn5: f64 = *var_vdg_dn5_slot;
        let mut var_vdg_dn6: f64 = *var_vdg_dn6_slot;
        let mut var_vdg_dn7: f64 = *var_vdg_dn7_slot;
        let mut var_vdg_dn8: f64 = *var_vdg_dn8_slot;
        let mut var_vdg_dn9: f64 = *var_vdg_dn9_slot;
        let mut var_vds: f64 = *var_vds_slot;
        let mut var_vds_db0: f64 = *var_vds_db0_slot;
        let mut var_vds_db1: f64 = *var_vds_db1_slot;
        let mut var_vds_db10: f64 = *var_vds_db10_slot;
        let mut var_vds_db11: f64 = *var_vds_db11_slot;
        let mut var_vds_db12: f64 = *var_vds_db12_slot;
        let mut var_vds_db13: f64 = *var_vds_db13_slot;
        let mut var_vds_db14: f64 = *var_vds_db14_slot;
        let mut var_vds_db2: f64 = *var_vds_db2_slot;
        let mut var_vds_db3: f64 = *var_vds_db3_slot;
        let mut var_vds_db4: f64 = *var_vds_db4_slot;
        let mut var_vds_db5: f64 = *var_vds_db5_slot;
        let mut var_vds_db6: f64 = *var_vds_db6_slot;
        let mut var_vds_db7: f64 = *var_vds_db7_slot;
        let mut var_vds_db8: f64 = *var_vds_db8_slot;
        let mut var_vds_db9: f64 = *var_vds_db9_slot;
        let mut var_vds_dn0: f64 = *var_vds_dn0_slot;
        let mut var_vds_dn1: f64 = *var_vds_dn1_slot;
        let mut var_vds_dn10: f64 = *var_vds_dn10_slot;
        let mut var_vds_dn11: f64 = *var_vds_dn11_slot;
        let mut var_vds_dn12: f64 = *var_vds_dn12_slot;
        let mut var_vds_dn13: f64 = *var_vds_dn13_slot;
        let mut var_vds_dn14: f64 = *var_vds_dn14_slot;
        let mut var_vds_dn15: f64 = *var_vds_dn15_slot;
        let mut var_vds_dn2: f64 = *var_vds_dn2_slot;
        let mut var_vds_dn3: f64 = *var_vds_dn3_slot;
        let mut var_vds_dn4: f64 = *var_vds_dn4_slot;
        let mut var_vds_dn5: f64 = *var_vds_dn5_slot;
        let mut var_vds_dn6: f64 = *var_vds_dn6_slot;
        let mut var_vds_dn7: f64 = *var_vds_dn7_slot;
        let mut var_vds_dn8: f64 = *var_vds_dn8_slot;
        let mut var_vds_dn9: f64 = *var_vds_dn9_slot;
        let mut var_vgd: f64 = *var_vgd_slot;
        let mut var_vgd_db0: f64 = *var_vgd_db0_slot;
        let mut var_vgd_db1: f64 = *var_vgd_db1_slot;
        let mut var_vgd_db10: f64 = *var_vgd_db10_slot;
        let mut var_vgd_db11: f64 = *var_vgd_db11_slot;
        let mut var_vgd_db12: f64 = *var_vgd_db12_slot;
        let mut var_vgd_db13: f64 = *var_vgd_db13_slot;
        let mut var_vgd_db14: f64 = *var_vgd_db14_slot;
        let mut var_vgd_db2: f64 = *var_vgd_db2_slot;
        let mut var_vgd_db3: f64 = *var_vgd_db3_slot;
        let mut var_vgd_db4: f64 = *var_vgd_db4_slot;
        let mut var_vgd_db5: f64 = *var_vgd_db5_slot;
        let mut var_vgd_db6: f64 = *var_vgd_db6_slot;
        let mut var_vgd_db7: f64 = *var_vgd_db7_slot;
        let mut var_vgd_db8: f64 = *var_vgd_db8_slot;
        let mut var_vgd_db9: f64 = *var_vgd_db9_slot;
        let mut var_vgd_dn0: f64 = *var_vgd_dn0_slot;
        let mut var_vgd_dn1: f64 = *var_vgd_dn1_slot;
        let mut var_vgd_dn10: f64 = *var_vgd_dn10_slot;
        let mut var_vgd_dn11: f64 = *var_vgd_dn11_slot;
        let mut var_vgd_dn12: f64 = *var_vgd_dn12_slot;
        let mut var_vgd_dn13: f64 = *var_vgd_dn13_slot;
        let mut var_vgd_dn14: f64 = *var_vgd_dn14_slot;
        let mut var_vgd_dn15: f64 = *var_vgd_dn15_slot;
        let mut var_vgd_dn2: f64 = *var_vgd_dn2_slot;
        let mut var_vgd_dn3: f64 = *var_vgd_dn3_slot;
        let mut var_vgd_dn4: f64 = *var_vgd_dn4_slot;
        let mut var_vgd_dn5: f64 = *var_vgd_dn5_slot;
        let mut var_vgd_dn6: f64 = *var_vgd_dn6_slot;
        let mut var_vgd_dn7: f64 = *var_vgd_dn7_slot;
        let mut var_vgd_dn8: f64 = *var_vgd_dn8_slot;
        let mut var_vgd_dn9: f64 = *var_vgd_dn9_slot;
        let mut var_vgdc: f64 = *var_vgdc_slot;
        let mut var_vgdc_db0: f64 = *var_vgdc_db0_slot;
        let mut var_vgdc_db1: f64 = *var_vgdc_db1_slot;
        let mut var_vgdc_db10: f64 = *var_vgdc_db10_slot;
        let mut var_vgdc_db11: f64 = *var_vgdc_db11_slot;
        let mut var_vgdc_db12: f64 = *var_vgdc_db12_slot;
        let mut var_vgdc_db13: f64 = *var_vgdc_db13_slot;
        let mut var_vgdc_db14: f64 = *var_vgdc_db14_slot;
        let mut var_vgdc_db2: f64 = *var_vgdc_db2_slot;
        let mut var_vgdc_db3: f64 = *var_vgdc_db3_slot;
        let mut var_vgdc_db4: f64 = *var_vgdc_db4_slot;
        let mut var_vgdc_db5: f64 = *var_vgdc_db5_slot;
        let mut var_vgdc_db6: f64 = *var_vgdc_db6_slot;
        let mut var_vgdc_db7: f64 = *var_vgdc_db7_slot;
        let mut var_vgdc_db8: f64 = *var_vgdc_db8_slot;
        let mut var_vgdc_db9: f64 = *var_vgdc_db9_slot;
        let mut var_vgdc_dn0: f64 = *var_vgdc_dn0_slot;
        let mut var_vgdc_dn1: f64 = *var_vgdc_dn1_slot;
        let mut var_vgdc_dn10: f64 = *var_vgdc_dn10_slot;
        let mut var_vgdc_dn11: f64 = *var_vgdc_dn11_slot;
        let mut var_vgdc_dn12: f64 = *var_vgdc_dn12_slot;
        let mut var_vgdc_dn13: f64 = *var_vgdc_dn13_slot;
        let mut var_vgdc_dn14: f64 = *var_vgdc_dn14_slot;
        let mut var_vgdc_dn15: f64 = *var_vgdc_dn15_slot;
        let mut var_vgdc_dn2: f64 = *var_vgdc_dn2_slot;
        let mut var_vgdc_dn3: f64 = *var_vgdc_dn3_slot;
        let mut var_vgdc_dn4: f64 = *var_vgdc_dn4_slot;
        let mut var_vgdc_dn5: f64 = *var_vgdc_dn5_slot;
        let mut var_vgdc_dn6: f64 = *var_vgdc_dn6_slot;
        let mut var_vgdc_dn7: f64 = *var_vgdc_dn7_slot;
        let mut var_vgdc_dn8: f64 = *var_vgdc_dn8_slot;
        let mut var_vgdc_dn9: f64 = *var_vgdc_dn9_slot;
        let mut var_vgs: f64 = *var_vgs_slot;
        let mut var_vgs_db0: f64 = *var_vgs_db0_slot;
        let mut var_vgs_db1: f64 = *var_vgs_db1_slot;
        let mut var_vgs_db10: f64 = *var_vgs_db10_slot;
        let mut var_vgs_db11: f64 = *var_vgs_db11_slot;
        let mut var_vgs_db12: f64 = *var_vgs_db12_slot;
        let mut var_vgs_db13: f64 = *var_vgs_db13_slot;
        let mut var_vgs_db14: f64 = *var_vgs_db14_slot;
        let mut var_vgs_db2: f64 = *var_vgs_db2_slot;
        let mut var_vgs_db3: f64 = *var_vgs_db3_slot;
        let mut var_vgs_db4: f64 = *var_vgs_db4_slot;
        let mut var_vgs_db5: f64 = *var_vgs_db5_slot;
        let mut var_vgs_db6: f64 = *var_vgs_db6_slot;
        let mut var_vgs_db7: f64 = *var_vgs_db7_slot;
        let mut var_vgs_db8: f64 = *var_vgs_db8_slot;
        let mut var_vgs_db9: f64 = *var_vgs_db9_slot;
        let mut var_vgs_dn0: f64 = *var_vgs_dn0_slot;
        let mut var_vgs_dn1: f64 = *var_vgs_dn1_slot;
        let mut var_vgs_dn10: f64 = *var_vgs_dn10_slot;
        let mut var_vgs_dn11: f64 = *var_vgs_dn11_slot;
        let mut var_vgs_dn12: f64 = *var_vgs_dn12_slot;
        let mut var_vgs_dn13: f64 = *var_vgs_dn13_slot;
        let mut var_vgs_dn14: f64 = *var_vgs_dn14_slot;
        let mut var_vgs_dn15: f64 = *var_vgs_dn15_slot;
        let mut var_vgs_dn2: f64 = *var_vgs_dn2_slot;
        let mut var_vgs_dn3: f64 = *var_vgs_dn3_slot;
        let mut var_vgs_dn4: f64 = *var_vgs_dn4_slot;
        let mut var_vgs_dn5: f64 = *var_vgs_dn5_slot;
        let mut var_vgs_dn6: f64 = *var_vgs_dn6_slot;
        let mut var_vgs_dn7: f64 = *var_vgs_dn7_slot;
        let mut var_vgs_dn8: f64 = *var_vgs_dn8_slot;
        let mut var_vgs_dn9: f64 = *var_vgs_dn9_slot;
        let mut var_vgsc: f64 = *var_vgsc_slot;
        let mut var_vgsc_db0: f64 = *var_vgsc_db0_slot;
        let mut var_vgsc_db1: f64 = *var_vgsc_db1_slot;
        let mut var_vgsc_db10: f64 = *var_vgsc_db10_slot;
        let mut var_vgsc_db11: f64 = *var_vgsc_db11_slot;
        let mut var_vgsc_db12: f64 = *var_vgsc_db12_slot;
        let mut var_vgsc_db13: f64 = *var_vgsc_db13_slot;
        let mut var_vgsc_db14: f64 = *var_vgsc_db14_slot;
        let mut var_vgsc_db2: f64 = *var_vgsc_db2_slot;
        let mut var_vgsc_db3: f64 = *var_vgsc_db3_slot;
        let mut var_vgsc_db4: f64 = *var_vgsc_db4_slot;
        let mut var_vgsc_db5: f64 = *var_vgsc_db5_slot;
        let mut var_vgsc_db6: f64 = *var_vgsc_db6_slot;
        let mut var_vgsc_db7: f64 = *var_vgsc_db7_slot;
        let mut var_vgsc_db8: f64 = *var_vgsc_db8_slot;
        let mut var_vgsc_db9: f64 = *var_vgsc_db9_slot;
        let mut var_vgsc_dn0: f64 = *var_vgsc_dn0_slot;
        let mut var_vgsc_dn1: f64 = *var_vgsc_dn1_slot;
        let mut var_vgsc_dn10: f64 = *var_vgsc_dn10_slot;
        let mut var_vgsc_dn11: f64 = *var_vgsc_dn11_slot;
        let mut var_vgsc_dn12: f64 = *var_vgsc_dn12_slot;
        let mut var_vgsc_dn13: f64 = *var_vgsc_dn13_slot;
        let mut var_vgsc_dn14: f64 = *var_vgsc_dn14_slot;
        let mut var_vgsc_dn15: f64 = *var_vgsc_dn15_slot;
        let mut var_vgsc_dn2: f64 = *var_vgsc_dn2_slot;
        let mut var_vgsc_dn3: f64 = *var_vgsc_dn3_slot;
        let mut var_vgsc_dn4: f64 = *var_vgsc_dn4_slot;
        let mut var_vgsc_dn5: f64 = *var_vgsc_dn5_slot;
        let mut var_vgsc_dn6: f64 = *var_vgsc_dn6_slot;
        let mut var_vgsc_dn7: f64 = *var_vgsc_dn7_slot;
        let mut var_vgsc_dn8: f64 = *var_vgsc_dn8_slot;
        let mut var_vgsc_dn9: f64 = *var_vgsc_dn9_slot;

        var_vgs = (nv8 - nv5);
        var_vgs_dn0 = 0.0;
        var_vgs_dn1 = 0.0;
        var_vgs_dn2 = 0.0;
        var_vgs_dn3 = 0.0;
        var_vgs_dn4 = 0.0;
        var_vgs_dn5 = -1.0;
        var_vgs_dn6 = 0.0;
        var_vgs_dn7 = 0.0;
        var_vgs_dn8 = 1.0;
        var_vgs_dn9 = 0.0;
        var_vgs_dn10 = 0.0;
        var_vgs_dn11 = 0.0;
        var_vgs_dn12 = 0.0;
        var_vgs_dn13 = 0.0;
        var_vgs_dn14 = 0.0;
        var_vgs_dn15 = 0.0;
        var_vgs_db0 = 0.0;
        var_vgs_db1 = 0.0;
        var_vgs_db2 = 0.0;
        var_vgs_db3 = 0.0;
        var_vgs_db4 = 0.0;
        var_vgs_db5 = 0.0;
        var_vgs_db6 = 0.0;
        var_vgs_db7 = 0.0;
        var_vgs_db8 = 0.0;
        var_vgs_db9 = 0.0;
        var_vgs_db10 = 0.0;
        var_vgs_db11 = 0.0;
        var_vgs_db12 = 0.0;
        var_vgs_db13 = 0.0;
        var_vgs_db14 = 0.0;

        var_vgd = (nv4 - nv3);
        var_vgd_dn0 = 0.0;
        var_vgd_dn1 = 0.0;
        var_vgd_dn2 = 0.0;
        var_vgd_dn3 = -1.0;
        var_vgd_dn4 = 1.0;
        var_vgd_dn5 = 0.0;
        var_vgd_dn6 = 0.0;
        var_vgd_dn7 = 0.0;
        var_vgd_dn8 = 0.0;
        var_vgd_dn9 = 0.0;
        var_vgd_dn10 = 0.0;
        var_vgd_dn11 = 0.0;
        var_vgd_dn12 = 0.0;
        var_vgd_dn13 = 0.0;
        var_vgd_dn14 = 0.0;
        var_vgd_dn15 = 0.0;
        var_vgd_db0 = 0.0;
        var_vgd_db1 = 0.0;
        var_vgd_db2 = 0.0;
        var_vgd_db3 = 0.0;
        var_vgd_db4 = 0.0;
        var_vgd_db5 = 0.0;
        var_vgd_db6 = 0.0;
        var_vgd_db7 = 0.0;
        var_vgd_db8 = 0.0;
        var_vgd_db9 = 0.0;
        var_vgd_db10 = 0.0;
        var_vgd_db11 = 0.0;
        var_vgd_db12 = 0.0;
        var_vgd_db13 = 0.0;
        var_vgd_db14 = 0.0;

        let assign20_e564: f64 = (-var_vgd);
        var_vdg = assign20_e564;
        var_vdg_dn0 = (-var_vgd_dn0);
        var_vdg_dn1 = (-var_vgd_dn1);
        var_vdg_dn2 = (-var_vgd_dn2);
        var_vdg_dn3 = (-var_vgd_dn3);
        var_vdg_dn4 = (-var_vgd_dn4);
        var_vdg_dn5 = (-var_vgd_dn5);
        var_vdg_dn6 = (-var_vgd_dn6);
        var_vdg_dn7 = (-var_vgd_dn7);
        var_vdg_dn8 = (-var_vgd_dn8);
        var_vdg_dn9 = (-var_vgd_dn9);
        var_vdg_dn10 = (-var_vgd_dn10);
        var_vdg_dn11 = (-var_vgd_dn11);
        var_vdg_dn12 = (-var_vgd_dn12);
        var_vdg_dn13 = (-var_vgd_dn13);
        var_vdg_dn14 = (-var_vgd_dn14);
        var_vdg_dn15 = (-var_vgd_dn15);
        var_vdg_db0 = (-var_vgd_db0);
        var_vdg_db1 = (-var_vgd_db1);
        var_vdg_db2 = (-var_vgd_db2);
        var_vdg_db3 = (-var_vgd_db3);
        var_vdg_db4 = (-var_vgd_db4);
        var_vdg_db5 = (-var_vgd_db5);
        var_vdg_db6 = (-var_vgd_db6);
        var_vdg_db7 = (-var_vgd_db7);
        var_vdg_db8 = (-var_vgd_db8);
        var_vdg_db9 = (-var_vgd_db9);
        var_vdg_db10 = (-var_vgd_db10);
        var_vdg_db11 = (-var_vgd_db11);
        var_vdg_db12 = (-var_vgd_db12);
        var_vdg_db13 = (-var_vgd_db13);
        var_vdg_db14 = (-var_vgd_db14);

        var_vds = (nv3 - nv5);
        var_vds_dn0 = 0.0;
        var_vds_dn1 = 0.0;
        var_vds_dn2 = 0.0;
        var_vds_dn3 = 1.0;
        var_vds_dn4 = 0.0;
        var_vds_dn5 = -1.0;
        var_vds_dn6 = 0.0;
        var_vds_dn7 = 0.0;
        var_vds_dn8 = 0.0;
        var_vds_dn9 = 0.0;
        var_vds_dn10 = 0.0;
        var_vds_dn11 = 0.0;
        var_vds_dn12 = 0.0;
        var_vds_dn13 = 0.0;
        var_vds_dn14 = 0.0;
        var_vds_dn15 = 0.0;
        var_vds_db0 = 0.0;
        var_vds_db1 = 0.0;
        var_vds_db2 = 0.0;
        var_vds_db3 = 0.0;
        var_vds_db4 = 0.0;
        var_vds_db5 = 0.0;
        var_vds_db6 = 0.0;
        var_vds_db7 = 0.0;
        var_vds_db8 = 0.0;
        var_vds_db9 = 0.0;
        var_vds_db10 = 0.0;
        var_vds_db11 = 0.0;
        var_vds_db12 = 0.0;
        var_vds_db13 = 0.0;
        var_vds_db14 = 0.0;

        var_vgsc = var_vgs;
        var_vgsc_dn0 = var_vgs_dn0;
        var_vgsc_dn1 = var_vgs_dn1;
        var_vgsc_dn2 = var_vgs_dn2;
        var_vgsc_dn3 = var_vgs_dn3;
        var_vgsc_dn4 = var_vgs_dn4;
        var_vgsc_dn5 = var_vgs_dn5;
        var_vgsc_dn6 = var_vgs_dn6;
        var_vgsc_dn7 = var_vgs_dn7;
        var_vgsc_dn8 = var_vgs_dn8;
        var_vgsc_dn9 = var_vgs_dn9;
        var_vgsc_dn10 = var_vgs_dn10;
        var_vgsc_dn11 = var_vgs_dn11;
        var_vgsc_dn12 = var_vgs_dn12;
        var_vgsc_dn13 = var_vgs_dn13;
        var_vgsc_dn14 = var_vgs_dn14;
        var_vgsc_dn15 = var_vgs_dn15;
        var_vgsc_db0 = var_vgs_db0;
        var_vgsc_db1 = var_vgs_db1;
        var_vgsc_db2 = var_vgs_db2;
        var_vgsc_db3 = var_vgs_db3;
        var_vgsc_db4 = var_vgs_db4;
        var_vgsc_db5 = var_vgs_db5;
        var_vgsc_db6 = var_vgs_db6;
        var_vgsc_db7 = var_vgs_db7;
        var_vgsc_db8 = var_vgs_db8;
        var_vgsc_db9 = var_vgs_db9;
        var_vgsc_db10 = var_vgs_db10;
        var_vgsc_db11 = var_vgs_db11;
        var_vgsc_db12 = var_vgs_db12;
        var_vgsc_db13 = var_vgs_db13;
        var_vgsc_db14 = var_vgs_db14;

        var_vgdc = (nv7 - nv3);
        var_vgdc_dn0 = 0.0;
        var_vgdc_dn1 = 0.0;
        var_vgdc_dn2 = 0.0;
        var_vgdc_dn3 = -1.0;
        var_vgdc_dn4 = 0.0;
        var_vgdc_dn5 = 0.0;
        var_vgdc_dn6 = 0.0;
        var_vgdc_dn7 = 1.0;
        var_vgdc_dn8 = 0.0;
        var_vgdc_dn9 = 0.0;
        var_vgdc_dn10 = 0.0;
        var_vgdc_dn11 = 0.0;
        var_vgdc_dn12 = 0.0;
        var_vgdc_dn13 = 0.0;
        var_vgdc_dn14 = 0.0;
        var_vgdc_dn15 = 0.0;
        var_vgdc_db0 = 0.0;
        var_vgdc_db1 = 0.0;
        var_vgdc_db2 = 0.0;
        var_vgdc_db3 = 0.0;
        var_vgdc_db4 = 0.0;
        var_vgdc_db5 = 0.0;
        var_vgdc_db6 = 0.0;
        var_vgdc_db7 = 0.0;
        var_vgdc_db8 = 0.0;
        var_vgdc_db9 = 0.0;
        var_vgdc_db10 = 0.0;
        var_vgdc_db11 = 0.0;
        var_vgdc_db12 = 0.0;
        var_vgdc_db13 = 0.0;
        var_vgdc_db14 = 0.0;

        var_qgd = 0.0;
        var_qgd_dn0 = 0.0;
        var_qgd_dn1 = 0.0;
        var_qgd_dn2 = 0.0;
        var_qgd_dn3 = 0.0;
        var_qgd_dn4 = 0.0;
        var_qgd_dn5 = 0.0;
        var_qgd_dn6 = 0.0;
        var_qgd_dn7 = 0.0;
        var_qgd_dn8 = 0.0;
        var_qgd_dn9 = 0.0;
        var_qgd_dn10 = 0.0;
        var_qgd_dn11 = 0.0;
        var_qgd_dn12 = 0.0;
        var_qgd_dn13 = 0.0;
        var_qgd_dn14 = 0.0;
        var_qgd_dn15 = 0.0;
        var_qgd_db0 = 0.0;
        var_qgd_db1 = 0.0;
        var_qgd_db2 = 0.0;
        var_qgd_db3 = 0.0;
        var_qgd_db4 = 0.0;
        var_qgd_db5 = 0.0;
        var_qgd_db6 = 0.0;
        var_qgd_db7 = 0.0;
        var_qgd_db8 = 0.0;
        var_qgd_db9 = 0.0;
        var_qgd_db10 = 0.0;
        var_qgd_db11 = 0.0;
        var_qgd_db12 = 0.0;
        var_qgd_db13 = 0.0;
        var_qgd_db14 = 0.0;

        var_qgs = 0.0;
        var_qgs_dn0 = 0.0;
        var_qgs_dn1 = 0.0;
        var_qgs_dn2 = 0.0;
        var_qgs_dn3 = 0.0;
        var_qgs_dn4 = 0.0;
        var_qgs_dn5 = 0.0;
        var_qgs_dn6 = 0.0;
        var_qgs_dn7 = 0.0;
        var_qgs_dn8 = 0.0;
        var_qgs_dn9 = 0.0;
        var_qgs_dn10 = 0.0;
        var_qgs_dn11 = 0.0;
        var_qgs_dn12 = 0.0;
        var_qgs_dn13 = 0.0;
        var_qgs_dn14 = 0.0;
        var_qgs_dn15 = 0.0;
        var_qgs_db0 = 0.0;
        var_qgs_db1 = 0.0;
        var_qgs_db2 = 0.0;
        var_qgs_db3 = 0.0;
        var_qgs_db4 = 0.0;
        var_qgs_db5 = 0.0;
        var_qgs_db6 = 0.0;
        var_qgs_db7 = 0.0;
        var_qgs_db8 = 0.0;
        var_qgs_db9 = 0.0;
        var_qgs_db10 = 0.0;
        var_qgs_db11 = 0.0;
        var_qgs_db12 = 0.0;
        var_qgs_db13 = 0.0;
        var_qgs_db14 = 0.0;

        var_cgd = 0.0;
        var_cgd_dn0 = 0.0;
        var_cgd_dn1 = 0.0;
        var_cgd_dn2 = 0.0;
        var_cgd_dn3 = 0.0;
        var_cgd_dn4 = 0.0;
        var_cgd_dn5 = 0.0;
        var_cgd_dn6 = 0.0;
        var_cgd_dn7 = 0.0;
        var_cgd_dn8 = 0.0;
        var_cgd_dn9 = 0.0;
        var_cgd_dn10 = 0.0;
        var_cgd_dn11 = 0.0;
        var_cgd_dn12 = 0.0;
        var_cgd_dn13 = 0.0;
        var_cgd_dn14 = 0.0;
        var_cgd_dn15 = 0.0;
        var_cgd_db0 = 0.0;
        var_cgd_db1 = 0.0;
        var_cgd_db2 = 0.0;
        var_cgd_db3 = 0.0;
        var_cgd_db4 = 0.0;
        var_cgd_db5 = 0.0;
        var_cgd_db6 = 0.0;
        var_cgd_db7 = 0.0;
        var_cgd_db8 = 0.0;
        var_cgd_db9 = 0.0;
        var_cgd_db10 = 0.0;
        var_cgd_db11 = 0.0;
        var_cgd_db12 = 0.0;
        var_cgd_db13 = 0.0;
        var_cgd_db14 = 0.0;

        var_cgs = 0.0;
        var_cgs_dn0 = 0.0;
        var_cgs_dn1 = 0.0;
        var_cgs_dn2 = 0.0;
        var_cgs_dn3 = 0.0;
        var_cgs_dn4 = 0.0;
        var_cgs_dn5 = 0.0;
        var_cgs_dn6 = 0.0;
        var_cgs_dn7 = 0.0;
        var_cgs_dn8 = 0.0;
        var_cgs_dn9 = 0.0;
        var_cgs_dn10 = 0.0;
        var_cgs_dn11 = 0.0;
        var_cgs_dn12 = 0.0;
        var_cgs_dn13 = 0.0;
        var_cgs_dn14 = 0.0;
        var_cgs_dn15 = 0.0;
        var_cgs_db0 = 0.0;
        var_cgs_db1 = 0.0;
        var_cgs_db2 = 0.0;
        var_cgs_db3 = 0.0;
        var_cgs_db4 = 0.0;
        var_cgs_db5 = 0.0;
        var_cgs_db6 = 0.0;
        var_cgs_db7 = 0.0;
        var_cgs_db8 = 0.0;
        var_cgs_db9 = 0.0;
        var_cgs_db10 = 0.0;
        var_cgs_db11 = 0.0;
        var_cgs_db12 = 0.0;
        var_cgs_db13 = 0.0;
        var_cgs_db14 = 0.0;

        let assign120_e575: f64 = if param_given[3] { 1.0 } else { 0.0 };
        var_guard1 = assign120_e575;

        let (assign130_e581, assign130_e581_d_n0, assign130_e581_d_n1, assign130_e581_d_n2, assign130_e581_d_n3, assign130_e581_d_n4, assign130_e581_d_n5, assign130_e581_d_n6, assign130_e581_d_n7, assign130_e581_d_n8, assign130_e581_d_n9, assign130_e581_d_n10, assign130_e581_d_n11, assign130_e581_d_n12, assign130_e581_d_n13, assign130_e581_d_n14, assign130_e581_d_n15, assign130_e581_d_b0, assign130_e581_d_b1, assign130_e581_d_b2, assign130_e581_d_b3, assign130_e581_d_b4, assign130_e581_d_b5, assign130_e581_d_b6, assign130_e581_d_b7, assign130_e581_d_b8, assign130_e581_d_b9, assign130_e581_d_b10, assign130_e581_d_b11, assign130_e581_d_b12, assign130_e581_d_b13, assign130_e581_d_b14,) = {
    if (var_guard1 != 0.0) {
        let assign130_e579: f64 = (p.p3 + 273.15);
        (assign130_e579, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t, var_t_dn0, var_t_dn1, var_t_dn2, var_t_dn3, var_t_dn4, var_t_dn5, var_t_dn6, var_t_dn7, var_t_dn8, var_t_dn9, var_t_dn10, var_t_dn11, var_t_dn12, var_t_dn13, var_t_dn14, var_t_dn15, var_t_db0, var_t_db1, var_t_db2, var_t_db3, var_t_db4, var_t_db5, var_t_db6, var_t_db7, var_t_db8, var_t_db9, var_t_db10, var_t_db11, var_t_db12, var_t_db13, var_t_db14,)
    }
};
        var_t = assign130_e581;
        var_t_dn0 = assign130_e581_d_n0;
        var_t_dn1 = assign130_e581_d_n1;
        var_t_dn2 = assign130_e581_d_n2;
        var_t_dn3 = assign130_e581_d_n3;
        var_t_dn4 = assign130_e581_d_n4;
        var_t_dn5 = assign130_e581_d_n5;
        var_t_dn6 = assign130_e581_d_n6;
        var_t_dn7 = assign130_e581_d_n7;
        var_t_dn8 = assign130_e581_d_n8;
        var_t_dn9 = assign130_e581_d_n9;
        var_t_dn10 = assign130_e581_d_n10;
        var_t_dn11 = assign130_e581_d_n11;
        var_t_dn12 = assign130_e581_d_n12;
        var_t_dn13 = assign130_e581_d_n13;
        var_t_dn14 = assign130_e581_d_n14;
        var_t_dn15 = assign130_e581_d_n15;
        var_t_db0 = assign130_e581_d_b0;
        var_t_db1 = assign130_e581_d_b1;
        var_t_db2 = assign130_e581_d_b2;
        var_t_db3 = assign130_e581_d_b3;
        var_t_db4 = assign130_e581_d_b4;
        var_t_db5 = assign130_e581_d_b5;
        var_t_db6 = assign130_e581_d_b6;
        var_t_db7 = assign130_e581_d_b7;
        var_t_db8 = assign130_e581_d_b8;
        var_t_db9 = assign130_e581_d_b9;
        var_t_db10 = assign130_e581_d_b10;
        var_t_db11 = assign130_e581_d_b11;
        var_t_db12 = assign130_e581_d_b12;
        var_t_db13 = assign130_e581_d_b13;
        var_t_db14 = assign130_e581_d_b14;

        let (assign140_e588, assign140_e588_d_n0, assign140_e588_d_n1, assign140_e588_d_n2, assign140_e588_d_n3, assign140_e588_d_n4, assign140_e588_d_n5, assign140_e588_d_n6, assign140_e588_d_n7, assign140_e588_d_n8, assign140_e588_d_n9, assign140_e588_d_n10, assign140_e588_d_n11, assign140_e588_d_n12, assign140_e588_d_n13, assign140_e588_d_n14, assign140_e588_d_n15, assign140_e588_d_b0, assign140_e588_d_b1, assign140_e588_d_b2, assign140_e588_d_b3, assign140_e588_d_b4, assign140_e588_d_b5, assign140_e588_d_b6, assign140_e588_d_b7, assign140_e588_d_b8, assign140_e588_d_b9, assign140_e588_d_b10, assign140_e588_d_b11, assign140_e588_d_b12, assign140_e588_d_b13, assign140_e588_d_b14,) = {
    if (var_guard1 == 0.0) {
        let assign140_e584: f64 = ctx_temp;
        let assign140_e586: f64 = (assign140_e584 + p.p2);
        (assign140_e586, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t, var_t_dn0, var_t_dn1, var_t_dn2, var_t_dn3, var_t_dn4, var_t_dn5, var_t_dn6, var_t_dn7, var_t_dn8, var_t_dn9, var_t_dn10, var_t_dn11, var_t_dn12, var_t_dn13, var_t_dn14, var_t_dn15, var_t_db0, var_t_db1, var_t_db2, var_t_db3, var_t_db4, var_t_db5, var_t_db6, var_t_db7, var_t_db8, var_t_db9, var_t_db10, var_t_db11, var_t_db12, var_t_db13, var_t_db14,)
    }
};
        var_t = assign140_e588;
        var_t_dn0 = assign140_e588_d_n0;
        var_t_dn1 = assign140_e588_d_n1;
        var_t_dn2 = assign140_e588_d_n2;
        var_t_dn3 = assign140_e588_d_n3;
        var_t_dn4 = assign140_e588_d_n4;
        var_t_dn5 = assign140_e588_d_n5;
        var_t_dn6 = assign140_e588_d_n6;
        var_t_dn7 = assign140_e588_d_n7;
        var_t_dn8 = assign140_e588_d_n8;
        var_t_dn9 = assign140_e588_d_n9;
        var_t_dn10 = assign140_e588_d_n10;
        var_t_dn11 = assign140_e588_d_n11;
        var_t_dn12 = assign140_e588_d_n12;
        var_t_dn13 = assign140_e588_d_n13;
        var_t_dn14 = assign140_e588_d_n14;
        var_t_dn15 = assign140_e588_d_n15;
        var_t_db0 = assign140_e588_d_b0;
        var_t_db1 = assign140_e588_d_b1;
        var_t_db2 = assign140_e588_d_b2;
        var_t_db3 = assign140_e588_d_b3;
        var_t_db4 = assign140_e588_d_b4;
        var_t_db5 = assign140_e588_d_b5;
        var_t_db6 = assign140_e588_d_b6;
        var_t_db7 = assign140_e588_d_b7;
        var_t_db8 = assign140_e588_d_b8;
        var_t_db9 = assign140_e588_d_b9;
        var_t_db10 = assign140_e588_d_b10;
        var_t_db11 = assign140_e588_d_b11;
        var_t_db12 = assign140_e588_d_b12;
        var_t_db13 = assign140_e588_d_b13;
        var_t_db14 = assign140_e588_d_b14;

        let assign150_e590: f64 = if param_given[85] { 1.0 } else { 0.0 };
        var_guard2 = assign150_e590;

        let (assign160_e596,) = {
    if (var_guard2 != 0.0) {
        let assign160_e594: f64 = (p.p85 + 273.15);
        (assign160_e594,)
    } else {
        (var_t_nom,)
    }
};
        var_t_nom = assign160_e596;

        let (assign170_e603,) = {
    if (var_guard2 == 0.0) {
        let assign170_e601: f64 = (27.0 + 273.15);
        (assign170_e601,)
    } else {
        (var_t_nom,)
    }
};
        var_t_nom = assign170_e603;

        let (assign180_e610, assign180_e610_d_n0, assign180_e610_d_n1, assign180_e610_d_n2, assign180_e610_d_n3, assign180_e610_d_n4, assign180_e610_d_n5, assign180_e610_d_n6, assign180_e610_d_n7, assign180_e610_d_n8, assign180_e610_d_n9, assign180_e610_d_n10, assign180_e610_d_n11, assign180_e610_d_n12, assign180_e610_d_n13, assign180_e610_d_n14, assign180_e610_d_n15, assign180_e610_d_b0, assign180_e610_d_b1, assign180_e610_d_b2, assign180_e610_d_b3, assign180_e610_d_b4, assign180_e610_d_b5, assign180_e610_d_b6, assign180_e610_d_b7, assign180_e610_d_b8, assign180_e610_d_b9, assign180_e610_d_b10, assign180_e610_d_b11, assign180_e610_d_b12, assign180_e610_d_b13, assign180_e610_d_b14,) = {
    if (p.p1 != 0.0) {
        let assign180_e607: f64 = ((nv11 - 0.0)).abs();
        let assign180_e608: f64 = (var_t + assign180_e607);
        (assign180_e608, var_t_dn0, var_t_dn1, var_t_dn2, var_t_dn3, var_t_dn4, var_t_dn5, var_t_dn6, var_t_dn7, var_t_dn8, var_t_dn9, var_t_dn10, (var_t_dn11 + if (nv11 - 0.0) >= 0.0 { 1.0 } else { (-1.0) }), var_t_dn12, var_t_dn13, var_t_dn14, var_t_dn15, var_t_db0, var_t_db1, var_t_db2, var_t_db3, var_t_db4, var_t_db5, var_t_db6, var_t_db7, var_t_db8, var_t_db9, var_t_db10, var_t_db11, var_t_db12, var_t_db13, var_t_db14,)
    } else {
        (var_t, var_t_dn0, var_t_dn1, var_t_dn2, var_t_dn3, var_t_dn4, var_t_dn5, var_t_dn6, var_t_dn7, var_t_dn8, var_t_dn9, var_t_dn10, var_t_dn11, var_t_dn12, var_t_dn13, var_t_dn14, var_t_dn15, var_t_db0, var_t_db1, var_t_db2, var_t_db3, var_t_db4, var_t_db5, var_t_db6, var_t_db7, var_t_db8, var_t_db9, var_t_db10, var_t_db11, var_t_db12, var_t_db13, var_t_db14,)
    }
};
        var_t = assign180_e610;
        var_t_dn0 = assign180_e610_d_n0;
        var_t_dn1 = assign180_e610_d_n1;
        var_t_dn2 = assign180_e610_d_n2;
        var_t_dn3 = assign180_e610_d_n3;
        var_t_dn4 = assign180_e610_d_n4;
        var_t_dn5 = assign180_e610_d_n5;
        var_t_dn6 = assign180_e610_d_n6;
        var_t_dn7 = assign180_e610_d_n7;
        var_t_dn8 = assign180_e610_d_n8;
        var_t_dn9 = assign180_e610_d_n9;
        var_t_dn10 = assign180_e610_d_n10;
        var_t_dn11 = assign180_e610_d_n11;
        var_t_dn12 = assign180_e610_d_n12;
        var_t_dn13 = assign180_e610_d_n13;
        var_t_dn14 = assign180_e610_d_n14;
        var_t_dn15 = assign180_e610_d_n15;
        var_t_db0 = assign180_e610_d_b0;
        var_t_db1 = assign180_e610_d_b1;
        var_t_db2 = assign180_e610_d_b2;
        var_t_db3 = assign180_e610_d_b3;
        var_t_db4 = assign180_e610_d_b4;
        var_t_db5 = assign180_e610_d_b5;
        var_t_db6 = assign180_e610_d_b6;
        var_t_db7 = assign180_e610_d_b7;
        var_t_db8 = assign180_e610_d_b8;
        var_t_db9 = assign180_e610_d_b9;
        var_t_db10 = assign180_e610_d_b10;
        var_t_db11 = assign180_e610_d_b11;
        var_t_db12 = assign180_e610_d_b12;
        var_t_db13 = assign180_e610_d_b13;
        var_t_db14 = assign180_e610_d_b14;


        *var_cgd_slot = var_cgd;
        *var_cgd_db0_slot = var_cgd_db0;
        *var_cgd_db1_slot = var_cgd_db1;
        *var_cgd_db10_slot = var_cgd_db10;
        *var_cgd_db11_slot = var_cgd_db11;
        *var_cgd_db12_slot = var_cgd_db12;
        *var_cgd_db13_slot = var_cgd_db13;
        *var_cgd_db14_slot = var_cgd_db14;
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
        *var_cgs_slot = var_cgs;
        *var_cgs_db0_slot = var_cgs_db0;
        *var_cgs_db1_slot = var_cgs_db1;
        *var_cgs_db10_slot = var_cgs_db10;
        *var_cgs_db11_slot = var_cgs_db11;
        *var_cgs_db12_slot = var_cgs_db12;
        *var_cgs_db13_slot = var_cgs_db13;
        *var_cgs_db14_slot = var_cgs_db14;
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
        *var_guard1_slot = var_guard1;
        *var_guard2_slot = var_guard2;
        *var_qgd_slot = var_qgd;
        *var_qgd_db0_slot = var_qgd_db0;
        *var_qgd_db1_slot = var_qgd_db1;
        *var_qgd_db10_slot = var_qgd_db10;
        *var_qgd_db11_slot = var_qgd_db11;
        *var_qgd_db12_slot = var_qgd_db12;
        *var_qgd_db13_slot = var_qgd_db13;
        *var_qgd_db14_slot = var_qgd_db14;
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
        *var_qgs_slot = var_qgs;
        *var_qgs_db0_slot = var_qgs_db0;
        *var_qgs_db1_slot = var_qgs_db1;
        *var_qgs_db10_slot = var_qgs_db10;
        *var_qgs_db11_slot = var_qgs_db11;
        *var_qgs_db12_slot = var_qgs_db12;
        *var_qgs_db13_slot = var_qgs_db13;
        *var_qgs_db14_slot = var_qgs_db14;
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
        *var_t_slot = var_t;
        *var_t_db0_slot = var_t_db0;
        *var_t_db1_slot = var_t_db1;
        *var_t_db10_slot = var_t_db10;
        *var_t_db11_slot = var_t_db11;
        *var_t_db12_slot = var_t_db12;
        *var_t_db13_slot = var_t_db13;
        *var_t_db14_slot = var_t_db14;
        *var_t_db2_slot = var_t_db2;
        *var_t_db3_slot = var_t_db3;
        *var_t_db4_slot = var_t_db4;
        *var_t_db5_slot = var_t_db5;
        *var_t_db6_slot = var_t_db6;
        *var_t_db7_slot = var_t_db7;
        *var_t_db8_slot = var_t_db8;
        *var_t_db9_slot = var_t_db9;
        *var_t_dn0_slot = var_t_dn0;
        *var_t_dn1_slot = var_t_dn1;
        *var_t_dn10_slot = var_t_dn10;
        *var_t_dn11_slot = var_t_dn11;
        *var_t_dn12_slot = var_t_dn12;
        *var_t_dn13_slot = var_t_dn13;
        *var_t_dn14_slot = var_t_dn14;
        *var_t_dn15_slot = var_t_dn15;
        *var_t_dn2_slot = var_t_dn2;
        *var_t_dn3_slot = var_t_dn3;
        *var_t_dn4_slot = var_t_dn4;
        *var_t_dn5_slot = var_t_dn5;
        *var_t_dn6_slot = var_t_dn6;
        *var_t_dn7_slot = var_t_dn7;
        *var_t_dn8_slot = var_t_dn8;
        *var_t_dn9_slot = var_t_dn9;
        *var_t_nom_slot = var_t_nom;
        *var_vdg_slot = var_vdg;
        *var_vdg_db0_slot = var_vdg_db0;
        *var_vdg_db1_slot = var_vdg_db1;
        *var_vdg_db10_slot = var_vdg_db10;
        *var_vdg_db11_slot = var_vdg_db11;
        *var_vdg_db12_slot = var_vdg_db12;
        *var_vdg_db13_slot = var_vdg_db13;
        *var_vdg_db14_slot = var_vdg_db14;
        *var_vdg_db2_slot = var_vdg_db2;
        *var_vdg_db3_slot = var_vdg_db3;
        *var_vdg_db4_slot = var_vdg_db4;
        *var_vdg_db5_slot = var_vdg_db5;
        *var_vdg_db6_slot = var_vdg_db6;
        *var_vdg_db7_slot = var_vdg_db7;
        *var_vdg_db8_slot = var_vdg_db8;
        *var_vdg_db9_slot = var_vdg_db9;
        *var_vdg_dn0_slot = var_vdg_dn0;
        *var_vdg_dn1_slot = var_vdg_dn1;
        *var_vdg_dn10_slot = var_vdg_dn10;
        *var_vdg_dn11_slot = var_vdg_dn11;
        *var_vdg_dn12_slot = var_vdg_dn12;
        *var_vdg_dn13_slot = var_vdg_dn13;
        *var_vdg_dn14_slot = var_vdg_dn14;
        *var_vdg_dn15_slot = var_vdg_dn15;
        *var_vdg_dn2_slot = var_vdg_dn2;
        *var_vdg_dn3_slot = var_vdg_dn3;
        *var_vdg_dn4_slot = var_vdg_dn4;
        *var_vdg_dn5_slot = var_vdg_dn5;
        *var_vdg_dn6_slot = var_vdg_dn6;
        *var_vdg_dn7_slot = var_vdg_dn7;
        *var_vdg_dn8_slot = var_vdg_dn8;
        *var_vdg_dn9_slot = var_vdg_dn9;
        *var_vds_slot = var_vds;
        *var_vds_db0_slot = var_vds_db0;
        *var_vds_db1_slot = var_vds_db1;
        *var_vds_db10_slot = var_vds_db10;
        *var_vds_db11_slot = var_vds_db11;
        *var_vds_db12_slot = var_vds_db12;
        *var_vds_db13_slot = var_vds_db13;
        *var_vds_db14_slot = var_vds_db14;
        *var_vds_db2_slot = var_vds_db2;
        *var_vds_db3_slot = var_vds_db3;
        *var_vds_db4_slot = var_vds_db4;
        *var_vds_db5_slot = var_vds_db5;
        *var_vds_db6_slot = var_vds_db6;
        *var_vds_db7_slot = var_vds_db7;
        *var_vds_db8_slot = var_vds_db8;
        *var_vds_db9_slot = var_vds_db9;
        *var_vds_dn0_slot = var_vds_dn0;
        *var_vds_dn1_slot = var_vds_dn1;
        *var_vds_dn10_slot = var_vds_dn10;
        *var_vds_dn11_slot = var_vds_dn11;
        *var_vds_dn12_slot = var_vds_dn12;
        *var_vds_dn13_slot = var_vds_dn13;
        *var_vds_dn14_slot = var_vds_dn14;
        *var_vds_dn15_slot = var_vds_dn15;
        *var_vds_dn2_slot = var_vds_dn2;
        *var_vds_dn3_slot = var_vds_dn3;
        *var_vds_dn4_slot = var_vds_dn4;
        *var_vds_dn5_slot = var_vds_dn5;
        *var_vds_dn6_slot = var_vds_dn6;
        *var_vds_dn7_slot = var_vds_dn7;
        *var_vds_dn8_slot = var_vds_dn8;
        *var_vds_dn9_slot = var_vds_dn9;
        *var_vgd_slot = var_vgd;
        *var_vgd_db0_slot = var_vgd_db0;
        *var_vgd_db1_slot = var_vgd_db1;
        *var_vgd_db10_slot = var_vgd_db10;
        *var_vgd_db11_slot = var_vgd_db11;
        *var_vgd_db12_slot = var_vgd_db12;
        *var_vgd_db13_slot = var_vgd_db13;
        *var_vgd_db14_slot = var_vgd_db14;
        *var_vgd_db2_slot = var_vgd_db2;
        *var_vgd_db3_slot = var_vgd_db3;
        *var_vgd_db4_slot = var_vgd_db4;
        *var_vgd_db5_slot = var_vgd_db5;
        *var_vgd_db6_slot = var_vgd_db6;
        *var_vgd_db7_slot = var_vgd_db7;
        *var_vgd_db8_slot = var_vgd_db8;
        *var_vgd_db9_slot = var_vgd_db9;
        *var_vgd_dn0_slot = var_vgd_dn0;
        *var_vgd_dn1_slot = var_vgd_dn1;
        *var_vgd_dn10_slot = var_vgd_dn10;
        *var_vgd_dn11_slot = var_vgd_dn11;
        *var_vgd_dn12_slot = var_vgd_dn12;
        *var_vgd_dn13_slot = var_vgd_dn13;
        *var_vgd_dn14_slot = var_vgd_dn14;
        *var_vgd_dn15_slot = var_vgd_dn15;
        *var_vgd_dn2_slot = var_vgd_dn2;
        *var_vgd_dn3_slot = var_vgd_dn3;
        *var_vgd_dn4_slot = var_vgd_dn4;
        *var_vgd_dn5_slot = var_vgd_dn5;
        *var_vgd_dn6_slot = var_vgd_dn6;
        *var_vgd_dn7_slot = var_vgd_dn7;
        *var_vgd_dn8_slot = var_vgd_dn8;
        *var_vgd_dn9_slot = var_vgd_dn9;
        *var_vgdc_slot = var_vgdc;
        *var_vgdc_db0_slot = var_vgdc_db0;
        *var_vgdc_db1_slot = var_vgdc_db1;
        *var_vgdc_db10_slot = var_vgdc_db10;
        *var_vgdc_db11_slot = var_vgdc_db11;
        *var_vgdc_db12_slot = var_vgdc_db12;
        *var_vgdc_db13_slot = var_vgdc_db13;
        *var_vgdc_db14_slot = var_vgdc_db14;
        *var_vgdc_db2_slot = var_vgdc_db2;
        *var_vgdc_db3_slot = var_vgdc_db3;
        *var_vgdc_db4_slot = var_vgdc_db4;
        *var_vgdc_db5_slot = var_vgdc_db5;
        *var_vgdc_db6_slot = var_vgdc_db6;
        *var_vgdc_db7_slot = var_vgdc_db7;
        *var_vgdc_db8_slot = var_vgdc_db8;
        *var_vgdc_db9_slot = var_vgdc_db9;
        *var_vgdc_dn0_slot = var_vgdc_dn0;
        *var_vgdc_dn1_slot = var_vgdc_dn1;
        *var_vgdc_dn10_slot = var_vgdc_dn10;
        *var_vgdc_dn11_slot = var_vgdc_dn11;
        *var_vgdc_dn12_slot = var_vgdc_dn12;
        *var_vgdc_dn13_slot = var_vgdc_dn13;
        *var_vgdc_dn14_slot = var_vgdc_dn14;
        *var_vgdc_dn15_slot = var_vgdc_dn15;
        *var_vgdc_dn2_slot = var_vgdc_dn2;
        *var_vgdc_dn3_slot = var_vgdc_dn3;
        *var_vgdc_dn4_slot = var_vgdc_dn4;
        *var_vgdc_dn5_slot = var_vgdc_dn5;
        *var_vgdc_dn6_slot = var_vgdc_dn6;
        *var_vgdc_dn7_slot = var_vgdc_dn7;
        *var_vgdc_dn8_slot = var_vgdc_dn8;
        *var_vgdc_dn9_slot = var_vgdc_dn9;
        *var_vgs_slot = var_vgs;
        *var_vgs_db0_slot = var_vgs_db0;
        *var_vgs_db1_slot = var_vgs_db1;
        *var_vgs_db10_slot = var_vgs_db10;
        *var_vgs_db11_slot = var_vgs_db11;
        *var_vgs_db12_slot = var_vgs_db12;
        *var_vgs_db13_slot = var_vgs_db13;
        *var_vgs_db14_slot = var_vgs_db14;
        *var_vgs_db2_slot = var_vgs_db2;
        *var_vgs_db3_slot = var_vgs_db3;
        *var_vgs_db4_slot = var_vgs_db4;
        *var_vgs_db5_slot = var_vgs_db5;
        *var_vgs_db6_slot = var_vgs_db6;
        *var_vgs_db7_slot = var_vgs_db7;
        *var_vgs_db8_slot = var_vgs_db8;
        *var_vgs_db9_slot = var_vgs_db9;
        *var_vgs_dn0_slot = var_vgs_dn0;
        *var_vgs_dn1_slot = var_vgs_dn1;
        *var_vgs_dn10_slot = var_vgs_dn10;
        *var_vgs_dn11_slot = var_vgs_dn11;
        *var_vgs_dn12_slot = var_vgs_dn12;
        *var_vgs_dn13_slot = var_vgs_dn13;
        *var_vgs_dn14_slot = var_vgs_dn14;
        *var_vgs_dn15_slot = var_vgs_dn15;
        *var_vgs_dn2_slot = var_vgs_dn2;
        *var_vgs_dn3_slot = var_vgs_dn3;
        *var_vgs_dn4_slot = var_vgs_dn4;
        *var_vgs_dn5_slot = var_vgs_dn5;
        *var_vgs_dn6_slot = var_vgs_dn6;
        *var_vgs_dn7_slot = var_vgs_dn7;
        *var_vgs_dn8_slot = var_vgs_dn8;
        *var_vgs_dn9_slot = var_vgs_dn9;
        *var_vgsc_slot = var_vgsc;
        *var_vgsc_db0_slot = var_vgsc_db0;
        *var_vgsc_db1_slot = var_vgsc_db1;
        *var_vgsc_db10_slot = var_vgsc_db10;
        *var_vgsc_db11_slot = var_vgsc_db11;
        *var_vgsc_db12_slot = var_vgsc_db12;
        *var_vgsc_db13_slot = var_vgsc_db13;
        *var_vgsc_db14_slot = var_vgsc_db14;
        *var_vgsc_db2_slot = var_vgsc_db2;
        *var_vgsc_db3_slot = var_vgsc_db3;
        *var_vgsc_db4_slot = var_vgsc_db4;
        *var_vgsc_db5_slot = var_vgsc_db5;
        *var_vgsc_db6_slot = var_vgsc_db6;
        *var_vgsc_db7_slot = var_vgsc_db7;
        *var_vgsc_db8_slot = var_vgsc_db8;
        *var_vgsc_db9_slot = var_vgsc_db9;
        *var_vgsc_dn0_slot = var_vgsc_dn0;
        *var_vgsc_dn1_slot = var_vgsc_dn1;
        *var_vgsc_dn10_slot = var_vgsc_dn10;
        *var_vgsc_dn11_slot = var_vgsc_dn11;
        *var_vgsc_dn12_slot = var_vgsc_dn12;
        *var_vgsc_dn13_slot = var_vgsc_dn13;
        *var_vgsc_dn14_slot = var_vgsc_dn14;
        *var_vgsc_dn15_slot = var_vgsc_dn15;
        *var_vgsc_dn2_slot = var_vgsc_dn2;
        *var_vgsc_dn3_slot = var_vgsc_dn3;
        *var_vgsc_dn4_slot = var_vgsc_dn4;
        *var_vgsc_dn5_slot = var_vgsc_dn5;
        *var_vgsc_dn6_slot = var_vgsc_dn6;
        *var_vgsc_dn7_slot = var_vgsc_dn7;
        *var_vgsc_dn8_slot = var_vgsc_dn8;
        *var_vgsc_dn9_slot = var_vgsc_dn9;
    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
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
        var_t_nom: f64,
        var_cgd0_t_slot: &mut f64,
        var_cgd0_t_db0_slot: &mut f64,
        var_cgd0_t_db1_slot: &mut f64,
        var_cgd0_t_db10_slot: &mut f64,
        var_cgd0_t_db11_slot: &mut f64,
        var_cgd0_t_db12_slot: &mut f64,
        var_cgd0_t_db13_slot: &mut f64,
        var_cgd0_t_db14_slot: &mut f64,
        var_cgd0_t_db2_slot: &mut f64,
        var_cgd0_t_db3_slot: &mut f64,
        var_cgd0_t_db4_slot: &mut f64,
        var_cgd0_t_db5_slot: &mut f64,
        var_cgd0_t_db6_slot: &mut f64,
        var_cgd0_t_db7_slot: &mut f64,
        var_cgd0_t_db8_slot: &mut f64,
        var_cgd0_t_db9_slot: &mut f64,
        var_cgd0_t_dn0_slot: &mut f64,
        var_cgd0_t_dn1_slot: &mut f64,
        var_cgd0_t_dn10_slot: &mut f64,
        var_cgd0_t_dn11_slot: &mut f64,
        var_cgd0_t_dn12_slot: &mut f64,
        var_cgd0_t_dn13_slot: &mut f64,
        var_cgd0_t_dn14_slot: &mut f64,
        var_cgd0_t_dn15_slot: &mut f64,
        var_cgd0_t_dn2_slot: &mut f64,
        var_cgd0_t_dn3_slot: &mut f64,
        var_cgd0_t_dn4_slot: &mut f64,
        var_cgd0_t_dn5_slot: &mut f64,
        var_cgd0_t_dn6_slot: &mut f64,
        var_cgd0_t_dn7_slot: &mut f64,
        var_cgd0_t_dn8_slot: &mut f64,
        var_cgd0_t_dn9_slot: &mut f64,
        var_cgs0_t_slot: &mut f64,
        var_cgs0_t_db0_slot: &mut f64,
        var_cgs0_t_db1_slot: &mut f64,
        var_cgs0_t_db10_slot: &mut f64,
        var_cgs0_t_db11_slot: &mut f64,
        var_cgs0_t_db12_slot: &mut f64,
        var_cgs0_t_db13_slot: &mut f64,
        var_cgs0_t_db14_slot: &mut f64,
        var_cgs0_t_db2_slot: &mut f64,
        var_cgs0_t_db3_slot: &mut f64,
        var_cgs0_t_db4_slot: &mut f64,
        var_cgs0_t_db5_slot: &mut f64,
        var_cgs0_t_db6_slot: &mut f64,
        var_cgs0_t_db7_slot: &mut f64,
        var_cgs0_t_db8_slot: &mut f64,
        var_cgs0_t_db9_slot: &mut f64,
        var_cgs0_t_dn0_slot: &mut f64,
        var_cgs0_t_dn1_slot: &mut f64,
        var_cgs0_t_dn10_slot: &mut f64,
        var_cgs0_t_dn11_slot: &mut f64,
        var_cgs0_t_dn12_slot: &mut f64,
        var_cgs0_t_dn13_slot: &mut f64,
        var_cgs0_t_dn14_slot: &mut f64,
        var_cgs0_t_dn15_slot: &mut f64,
        var_cgs0_t_dn2_slot: &mut f64,
        var_cgs0_t_dn3_slot: &mut f64,
        var_cgs0_t_dn4_slot: &mut f64,
        var_cgs0_t_dn5_slot: &mut f64,
        var_cgs0_t_dn6_slot: &mut f64,
        var_cgs0_t_dn7_slot: &mut f64,
        var_cgs0_t_dn8_slot: &mut f64,
        var_cgs0_t_dn9_slot: &mut f64,
        var_delta_t_slot: &mut f64,
        var_delta_t_db0_slot: &mut f64,
        var_delta_t_db1_slot: &mut f64,
        var_delta_t_db10_slot: &mut f64,
        var_delta_t_db11_slot: &mut f64,
        var_delta_t_db12_slot: &mut f64,
        var_delta_t_db13_slot: &mut f64,
        var_delta_t_db14_slot: &mut f64,
        var_delta_t_db2_slot: &mut f64,
        var_delta_t_db3_slot: &mut f64,
        var_delta_t_db4_slot: &mut f64,
        var_delta_t_db5_slot: &mut f64,
        var_delta_t_db6_slot: &mut f64,
        var_delta_t_db7_slot: &mut f64,
        var_delta_t_db8_slot: &mut f64,
        var_delta_t_db9_slot: &mut f64,
        var_delta_t_dn0_slot: &mut f64,
        var_delta_t_dn1_slot: &mut f64,
        var_delta_t_dn10_slot: &mut f64,
        var_delta_t_dn11_slot: &mut f64,
        var_delta_t_dn12_slot: &mut f64,
        var_delta_t_dn13_slot: &mut f64,
        var_delta_t_dn14_slot: &mut f64,
        var_delta_t_dn15_slot: &mut f64,
        var_delta_t_dn2_slot: &mut f64,
        var_delta_t_dn3_slot: &mut f64,
        var_delta_t_dn4_slot: &mut f64,
        var_delta_t_dn5_slot: &mut f64,
        var_delta_t_dn6_slot: &mut f64,
        var_delta_t_dn7_slot: &mut f64,
        var_delta_t_dn8_slot: &mut f64,
        var_delta_t_dn9_slot: &mut f64,
        var_guard3_slot: &mut f64,
        var_p10_t_slot: &mut f64,
        var_p10_t_db0_slot: &mut f64,
        var_p10_t_db1_slot: &mut f64,
        var_p10_t_db10_slot: &mut f64,
        var_p10_t_db11_slot: &mut f64,
        var_p10_t_db12_slot: &mut f64,
        var_p10_t_db13_slot: &mut f64,
        var_p10_t_db14_slot: &mut f64,
        var_p10_t_db2_slot: &mut f64,
        var_p10_t_db3_slot: &mut f64,
        var_p10_t_db4_slot: &mut f64,
        var_p10_t_db5_slot: &mut f64,
        var_p10_t_db6_slot: &mut f64,
        var_p10_t_db7_slot: &mut f64,
        var_p10_t_db8_slot: &mut f64,
        var_p10_t_db9_slot: &mut f64,
        var_p10_t_dn0_slot: &mut f64,
        var_p10_t_dn1_slot: &mut f64,
        var_p10_t_dn10_slot: &mut f64,
        var_p10_t_dn11_slot: &mut f64,
        var_p10_t_dn12_slot: &mut f64,
        var_p10_t_dn13_slot: &mut f64,
        var_p10_t_dn14_slot: &mut f64,
        var_p10_t_dn15_slot: &mut f64,
        var_p10_t_dn2_slot: &mut f64,
        var_p10_t_dn3_slot: &mut f64,
        var_p10_t_dn4_slot: &mut f64,
        var_p10_t_dn5_slot: &mut f64,
        var_p10_t_dn6_slot: &mut f64,
        var_p10_t_dn7_slot: &mut f64,
        var_p10_t_dn8_slot: &mut f64,
        var_p10_t_dn9_slot: &mut f64,
        var_p1_t_slot: &mut f64,
        var_p1_t_db0_slot: &mut f64,
        var_p1_t_db1_slot: &mut f64,
        var_p1_t_db10_slot: &mut f64,
        var_p1_t_db11_slot: &mut f64,
        var_p1_t_db12_slot: &mut f64,
        var_p1_t_db13_slot: &mut f64,
        var_p1_t_db14_slot: &mut f64,
        var_p1_t_db2_slot: &mut f64,
        var_p1_t_db3_slot: &mut f64,
        var_p1_t_db4_slot: &mut f64,
        var_p1_t_db5_slot: &mut f64,
        var_p1_t_db6_slot: &mut f64,
        var_p1_t_db7_slot: &mut f64,
        var_p1_t_db8_slot: &mut f64,
        var_p1_t_db9_slot: &mut f64,
        var_p1_t_dn0_slot: &mut f64,
        var_p1_t_dn1_slot: &mut f64,
        var_p1_t_dn10_slot: &mut f64,
        var_p1_t_dn11_slot: &mut f64,
        var_p1_t_dn12_slot: &mut f64,
        var_p1_t_dn13_slot: &mut f64,
        var_p1_t_dn14_slot: &mut f64,
        var_p1_t_dn15_slot: &mut f64,
        var_p1_t_dn2_slot: &mut f64,
        var_p1_t_dn3_slot: &mut f64,
        var_p1_t_dn4_slot: &mut f64,
        var_p1_t_dn5_slot: &mut f64,
        var_p1_t_dn6_slot: &mut f64,
        var_p1_t_dn7_slot: &mut f64,
        var_p1_t_dn8_slot: &mut f64,
        var_p1_t_dn9_slot: &mut f64,
        var_p40_t_slot: &mut f64,
        var_p40_t_db0_slot: &mut f64,
        var_p40_t_db1_slot: &mut f64,
        var_p40_t_db10_slot: &mut f64,
        var_p40_t_db11_slot: &mut f64,
        var_p40_t_db12_slot: &mut f64,
        var_p40_t_db13_slot: &mut f64,
        var_p40_t_db14_slot: &mut f64,
        var_p40_t_db2_slot: &mut f64,
        var_p40_t_db3_slot: &mut f64,
        var_p40_t_db4_slot: &mut f64,
        var_p40_t_db5_slot: &mut f64,
        var_p40_t_db6_slot: &mut f64,
        var_p40_t_db7_slot: &mut f64,
        var_p40_t_db8_slot: &mut f64,
        var_p40_t_db9_slot: &mut f64,
        var_p40_t_dn0_slot: &mut f64,
        var_p40_t_dn1_slot: &mut f64,
        var_p40_t_dn10_slot: &mut f64,
        var_p40_t_dn11_slot: &mut f64,
        var_p40_t_dn12_slot: &mut f64,
        var_p40_t_dn13_slot: &mut f64,
        var_p40_t_dn14_slot: &mut f64,
        var_p40_t_dn15_slot: &mut f64,
        var_p40_t_dn2_slot: &mut f64,
        var_p40_t_dn3_slot: &mut f64,
        var_p40_t_dn4_slot: &mut f64,
        var_p40_t_dn5_slot: &mut f64,
        var_p40_t_dn6_slot: &mut f64,
        var_p40_t_dn7_slot: &mut f64,
        var_p40_t_dn8_slot: &mut f64,
        var_p40_t_dn9_slot: &mut f64,
        var_vjg_t_slot: &mut f64,
        var_vjg_t_db0_slot: &mut f64,
        var_vjg_t_db1_slot: &mut f64,
        var_vjg_t_db10_slot: &mut f64,
        var_vjg_t_db11_slot: &mut f64,
        var_vjg_t_db12_slot: &mut f64,
        var_vjg_t_db13_slot: &mut f64,
        var_vjg_t_db14_slot: &mut f64,
        var_vjg_t_db2_slot: &mut f64,
        var_vjg_t_db3_slot: &mut f64,
        var_vjg_t_db4_slot: &mut f64,
        var_vjg_t_db5_slot: &mut f64,
        var_vjg_t_db6_slot: &mut f64,
        var_vjg_t_db7_slot: &mut f64,
        var_vjg_t_db8_slot: &mut f64,
        var_vjg_t_db9_slot: &mut f64,
        var_vjg_t_dn0_slot: &mut f64,
        var_vjg_t_dn1_slot: &mut f64,
        var_vjg_t_dn10_slot: &mut f64,
        var_vjg_t_dn11_slot: &mut f64,
        var_vjg_t_dn12_slot: &mut f64,
        var_vjg_t_dn13_slot: &mut f64,
        var_vjg_t_dn14_slot: &mut f64,
        var_vjg_t_dn15_slot: &mut f64,
        var_vjg_t_dn2_slot: &mut f64,
        var_vjg_t_dn3_slot: &mut f64,
        var_vjg_t_dn4_slot: &mut f64,
        var_vjg_t_dn5_slot: &mut f64,
        var_vjg_t_dn6_slot: &mut f64,
        var_vjg_t_dn7_slot: &mut f64,
        var_vjg_t_dn8_slot: &mut f64,
        var_vjg_t_dn9_slot: &mut f64,
        var_vpks_t_slot: &mut f64,
        var_vpks_t_db0_slot: &mut f64,
        var_vpks_t_db1_slot: &mut f64,
        var_vpks_t_db10_slot: &mut f64,
        var_vpks_t_db11_slot: &mut f64,
        var_vpks_t_db12_slot: &mut f64,
        var_vpks_t_db13_slot: &mut f64,
        var_vpks_t_db14_slot: &mut f64,
        var_vpks_t_db2_slot: &mut f64,
        var_vpks_t_db3_slot: &mut f64,
        var_vpks_t_db4_slot: &mut f64,
        var_vpks_t_db5_slot: &mut f64,
        var_vpks_t_db6_slot: &mut f64,
        var_vpks_t_db7_slot: &mut f64,
        var_vpks_t_db8_slot: &mut f64,
        var_vpks_t_db9_slot: &mut f64,
        var_vpks_t_dn0_slot: &mut f64,
        var_vpks_t_dn1_slot: &mut f64,
        var_vpks_t_dn10_slot: &mut f64,
        var_vpks_t_dn11_slot: &mut f64,
        var_vpks_t_dn12_slot: &mut f64,
        var_vpks_t_dn13_slot: &mut f64,
        var_vpks_t_dn14_slot: &mut f64,
        var_vpks_t_dn15_slot: &mut f64,
        var_vpks_t_dn2_slot: &mut f64,
        var_vpks_t_dn3_slot: &mut f64,
        var_vpks_t_dn4_slot: &mut f64,
        var_vpks_t_dn5_slot: &mut f64,
        var_vpks_t_dn6_slot: &mut f64,
        var_vpks_t_dn7_slot: &mut f64,
        var_vpks_t_dn8_slot: &mut f64,
        var_vpks_t_dn9_slot: &mut f64,
        var_vth_slot: &mut f64,
        var_vth_db0_slot: &mut f64,
        var_vth_db1_slot: &mut f64,
        var_vth_db10_slot: &mut f64,
        var_vth_db11_slot: &mut f64,
        var_vth_db12_slot: &mut f64,
        var_vth_db13_slot: &mut f64,
        var_vth_db14_slot: &mut f64,
        var_vth_db2_slot: &mut f64,
        var_vth_db3_slot: &mut f64,
        var_vth_db4_slot: &mut f64,
        var_vth_db5_slot: &mut f64,
        var_vth_db6_slot: &mut f64,
        var_vth_db7_slot: &mut f64,
        var_vth_db8_slot: &mut f64,
        var_vth_db9_slot: &mut f64,
        var_vth_dn0_slot: &mut f64,
        var_vth_dn1_slot: &mut f64,
        var_vth_dn10_slot: &mut f64,
        var_vth_dn11_slot: &mut f64,
        var_vth_dn12_slot: &mut f64,
        var_vth_dn13_slot: &mut f64,
        var_vth_dn14_slot: &mut f64,
        var_vth_dn15_slot: &mut f64,
        var_vth_dn2_slot: &mut f64,
        var_vth_dn3_slot: &mut f64,
        var_vth_dn4_slot: &mut f64,
        var_vth_dn5_slot: &mut f64,
        var_vth_dn6_slot: &mut f64,
        var_vth_dn7_slot: &mut f64,
        var_vth_dn8_slot: &mut f64,
        var_vth_dn9_slot: &mut f64,
        var_vtr_t_slot: &mut f64,
        var_vtr_t_db0_slot: &mut f64,
        var_vtr_t_db1_slot: &mut f64,
        var_vtr_t_db10_slot: &mut f64,
        var_vtr_t_db11_slot: &mut f64,
        var_vtr_t_db12_slot: &mut f64,
        var_vtr_t_db13_slot: &mut f64,
        var_vtr_t_db14_slot: &mut f64,
        var_vtr_t_db2_slot: &mut f64,
        var_vtr_t_db3_slot: &mut f64,
        var_vtr_t_db4_slot: &mut f64,
        var_vtr_t_db5_slot: &mut f64,
        var_vtr_t_db6_slot: &mut f64,
        var_vtr_t_db7_slot: &mut f64,
        var_vtr_t_db8_slot: &mut f64,
        var_vtr_t_db9_slot: &mut f64,
        var_vtr_t_dn0_slot: &mut f64,
        var_vtr_t_dn1_slot: &mut f64,
        var_vtr_t_dn10_slot: &mut f64,
        var_vtr_t_dn11_slot: &mut f64,
        var_vtr_t_dn12_slot: &mut f64,
        var_vtr_t_dn13_slot: &mut f64,
        var_vtr_t_dn14_slot: &mut f64,
        var_vtr_t_dn15_slot: &mut f64,
        var_vtr_t_dn2_slot: &mut f64,
        var_vtr_t_dn3_slot: &mut f64,
        var_vtr_t_dn4_slot: &mut f64,
        var_vtr_t_dn5_slot: &mut f64,
        var_vtr_t_dn6_slot: &mut f64,
        var_vtr_t_dn7_slot: &mut f64,
        var_vtr_t_dn8_slot: &mut f64,
        var_vtr_t_dn9_slot: &mut f64,
    ) {
        let mut var_cgd0_t: f64 = *var_cgd0_t_slot;
        let mut var_cgd0_t_db0: f64 = *var_cgd0_t_db0_slot;
        let mut var_cgd0_t_db1: f64 = *var_cgd0_t_db1_slot;
        let mut var_cgd0_t_db10: f64 = *var_cgd0_t_db10_slot;
        let mut var_cgd0_t_db11: f64 = *var_cgd0_t_db11_slot;
        let mut var_cgd0_t_db12: f64 = *var_cgd0_t_db12_slot;
        let mut var_cgd0_t_db13: f64 = *var_cgd0_t_db13_slot;
        let mut var_cgd0_t_db14: f64 = *var_cgd0_t_db14_slot;
        let mut var_cgd0_t_db2: f64 = *var_cgd0_t_db2_slot;
        let mut var_cgd0_t_db3: f64 = *var_cgd0_t_db3_slot;
        let mut var_cgd0_t_db4: f64 = *var_cgd0_t_db4_slot;
        let mut var_cgd0_t_db5: f64 = *var_cgd0_t_db5_slot;
        let mut var_cgd0_t_db6: f64 = *var_cgd0_t_db6_slot;
        let mut var_cgd0_t_db7: f64 = *var_cgd0_t_db7_slot;
        let mut var_cgd0_t_db8: f64 = *var_cgd0_t_db8_slot;
        let mut var_cgd0_t_db9: f64 = *var_cgd0_t_db9_slot;
        let mut var_cgd0_t_dn0: f64 = *var_cgd0_t_dn0_slot;
        let mut var_cgd0_t_dn1: f64 = *var_cgd0_t_dn1_slot;
        let mut var_cgd0_t_dn10: f64 = *var_cgd0_t_dn10_slot;
        let mut var_cgd0_t_dn11: f64 = *var_cgd0_t_dn11_slot;
        let mut var_cgd0_t_dn12: f64 = *var_cgd0_t_dn12_slot;
        let mut var_cgd0_t_dn13: f64 = *var_cgd0_t_dn13_slot;
        let mut var_cgd0_t_dn14: f64 = *var_cgd0_t_dn14_slot;
        let mut var_cgd0_t_dn15: f64 = *var_cgd0_t_dn15_slot;
        let mut var_cgd0_t_dn2: f64 = *var_cgd0_t_dn2_slot;
        let mut var_cgd0_t_dn3: f64 = *var_cgd0_t_dn3_slot;
        let mut var_cgd0_t_dn4: f64 = *var_cgd0_t_dn4_slot;
        let mut var_cgd0_t_dn5: f64 = *var_cgd0_t_dn5_slot;
        let mut var_cgd0_t_dn6: f64 = *var_cgd0_t_dn6_slot;
        let mut var_cgd0_t_dn7: f64 = *var_cgd0_t_dn7_slot;
        let mut var_cgd0_t_dn8: f64 = *var_cgd0_t_dn8_slot;
        let mut var_cgd0_t_dn9: f64 = *var_cgd0_t_dn9_slot;
        let mut var_cgs0_t: f64 = *var_cgs0_t_slot;
        let mut var_cgs0_t_db0: f64 = *var_cgs0_t_db0_slot;
        let mut var_cgs0_t_db1: f64 = *var_cgs0_t_db1_slot;
        let mut var_cgs0_t_db10: f64 = *var_cgs0_t_db10_slot;
        let mut var_cgs0_t_db11: f64 = *var_cgs0_t_db11_slot;
        let mut var_cgs0_t_db12: f64 = *var_cgs0_t_db12_slot;
        let mut var_cgs0_t_db13: f64 = *var_cgs0_t_db13_slot;
        let mut var_cgs0_t_db14: f64 = *var_cgs0_t_db14_slot;
        let mut var_cgs0_t_db2: f64 = *var_cgs0_t_db2_slot;
        let mut var_cgs0_t_db3: f64 = *var_cgs0_t_db3_slot;
        let mut var_cgs0_t_db4: f64 = *var_cgs0_t_db4_slot;
        let mut var_cgs0_t_db5: f64 = *var_cgs0_t_db5_slot;
        let mut var_cgs0_t_db6: f64 = *var_cgs0_t_db6_slot;
        let mut var_cgs0_t_db7: f64 = *var_cgs0_t_db7_slot;
        let mut var_cgs0_t_db8: f64 = *var_cgs0_t_db8_slot;
        let mut var_cgs0_t_db9: f64 = *var_cgs0_t_db9_slot;
        let mut var_cgs0_t_dn0: f64 = *var_cgs0_t_dn0_slot;
        let mut var_cgs0_t_dn1: f64 = *var_cgs0_t_dn1_slot;
        let mut var_cgs0_t_dn10: f64 = *var_cgs0_t_dn10_slot;
        let mut var_cgs0_t_dn11: f64 = *var_cgs0_t_dn11_slot;
        let mut var_cgs0_t_dn12: f64 = *var_cgs0_t_dn12_slot;
        let mut var_cgs0_t_dn13: f64 = *var_cgs0_t_dn13_slot;
        let mut var_cgs0_t_dn14: f64 = *var_cgs0_t_dn14_slot;
        let mut var_cgs0_t_dn15: f64 = *var_cgs0_t_dn15_slot;
        let mut var_cgs0_t_dn2: f64 = *var_cgs0_t_dn2_slot;
        let mut var_cgs0_t_dn3: f64 = *var_cgs0_t_dn3_slot;
        let mut var_cgs0_t_dn4: f64 = *var_cgs0_t_dn4_slot;
        let mut var_cgs0_t_dn5: f64 = *var_cgs0_t_dn5_slot;
        let mut var_cgs0_t_dn6: f64 = *var_cgs0_t_dn6_slot;
        let mut var_cgs0_t_dn7: f64 = *var_cgs0_t_dn7_slot;
        let mut var_cgs0_t_dn8: f64 = *var_cgs0_t_dn8_slot;
        let mut var_cgs0_t_dn9: f64 = *var_cgs0_t_dn9_slot;
        let mut var_delta_t: f64 = *var_delta_t_slot;
        let mut var_delta_t_db0: f64 = *var_delta_t_db0_slot;
        let mut var_delta_t_db1: f64 = *var_delta_t_db1_slot;
        let mut var_delta_t_db10: f64 = *var_delta_t_db10_slot;
        let mut var_delta_t_db11: f64 = *var_delta_t_db11_slot;
        let mut var_delta_t_db12: f64 = *var_delta_t_db12_slot;
        let mut var_delta_t_db13: f64 = *var_delta_t_db13_slot;
        let mut var_delta_t_db14: f64 = *var_delta_t_db14_slot;
        let mut var_delta_t_db2: f64 = *var_delta_t_db2_slot;
        let mut var_delta_t_db3: f64 = *var_delta_t_db3_slot;
        let mut var_delta_t_db4: f64 = *var_delta_t_db4_slot;
        let mut var_delta_t_db5: f64 = *var_delta_t_db5_slot;
        let mut var_delta_t_db6: f64 = *var_delta_t_db6_slot;
        let mut var_delta_t_db7: f64 = *var_delta_t_db7_slot;
        let mut var_delta_t_db8: f64 = *var_delta_t_db8_slot;
        let mut var_delta_t_db9: f64 = *var_delta_t_db9_slot;
        let mut var_delta_t_dn0: f64 = *var_delta_t_dn0_slot;
        let mut var_delta_t_dn1: f64 = *var_delta_t_dn1_slot;
        let mut var_delta_t_dn10: f64 = *var_delta_t_dn10_slot;
        let mut var_delta_t_dn11: f64 = *var_delta_t_dn11_slot;
        let mut var_delta_t_dn12: f64 = *var_delta_t_dn12_slot;
        let mut var_delta_t_dn13: f64 = *var_delta_t_dn13_slot;
        let mut var_delta_t_dn14: f64 = *var_delta_t_dn14_slot;
        let mut var_delta_t_dn15: f64 = *var_delta_t_dn15_slot;
        let mut var_delta_t_dn2: f64 = *var_delta_t_dn2_slot;
        let mut var_delta_t_dn3: f64 = *var_delta_t_dn3_slot;
        let mut var_delta_t_dn4: f64 = *var_delta_t_dn4_slot;
        let mut var_delta_t_dn5: f64 = *var_delta_t_dn5_slot;
        let mut var_delta_t_dn6: f64 = *var_delta_t_dn6_slot;
        let mut var_delta_t_dn7: f64 = *var_delta_t_dn7_slot;
        let mut var_delta_t_dn8: f64 = *var_delta_t_dn8_slot;
        let mut var_delta_t_dn9: f64 = *var_delta_t_dn9_slot;
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_p10_t: f64 = *var_p10_t_slot;
        let mut var_p10_t_db0: f64 = *var_p10_t_db0_slot;
        let mut var_p10_t_db1: f64 = *var_p10_t_db1_slot;
        let mut var_p10_t_db10: f64 = *var_p10_t_db10_slot;
        let mut var_p10_t_db11: f64 = *var_p10_t_db11_slot;
        let mut var_p10_t_db12: f64 = *var_p10_t_db12_slot;
        let mut var_p10_t_db13: f64 = *var_p10_t_db13_slot;
        let mut var_p10_t_db14: f64 = *var_p10_t_db14_slot;
        let mut var_p10_t_db2: f64 = *var_p10_t_db2_slot;
        let mut var_p10_t_db3: f64 = *var_p10_t_db3_slot;
        let mut var_p10_t_db4: f64 = *var_p10_t_db4_slot;
        let mut var_p10_t_db5: f64 = *var_p10_t_db5_slot;
        let mut var_p10_t_db6: f64 = *var_p10_t_db6_slot;
        let mut var_p10_t_db7: f64 = *var_p10_t_db7_slot;
        let mut var_p10_t_db8: f64 = *var_p10_t_db8_slot;
        let mut var_p10_t_db9: f64 = *var_p10_t_db9_slot;
        let mut var_p10_t_dn0: f64 = *var_p10_t_dn0_slot;
        let mut var_p10_t_dn1: f64 = *var_p10_t_dn1_slot;
        let mut var_p10_t_dn10: f64 = *var_p10_t_dn10_slot;
        let mut var_p10_t_dn11: f64 = *var_p10_t_dn11_slot;
        let mut var_p10_t_dn12: f64 = *var_p10_t_dn12_slot;
        let mut var_p10_t_dn13: f64 = *var_p10_t_dn13_slot;
        let mut var_p10_t_dn14: f64 = *var_p10_t_dn14_slot;
        let mut var_p10_t_dn15: f64 = *var_p10_t_dn15_slot;
        let mut var_p10_t_dn2: f64 = *var_p10_t_dn2_slot;
        let mut var_p10_t_dn3: f64 = *var_p10_t_dn3_slot;
        let mut var_p10_t_dn4: f64 = *var_p10_t_dn4_slot;
        let mut var_p10_t_dn5: f64 = *var_p10_t_dn5_slot;
        let mut var_p10_t_dn6: f64 = *var_p10_t_dn6_slot;
        let mut var_p10_t_dn7: f64 = *var_p10_t_dn7_slot;
        let mut var_p10_t_dn8: f64 = *var_p10_t_dn8_slot;
        let mut var_p10_t_dn9: f64 = *var_p10_t_dn9_slot;
        let mut var_p1_t: f64 = *var_p1_t_slot;
        let mut var_p1_t_db0: f64 = *var_p1_t_db0_slot;
        let mut var_p1_t_db1: f64 = *var_p1_t_db1_slot;
        let mut var_p1_t_db10: f64 = *var_p1_t_db10_slot;
        let mut var_p1_t_db11: f64 = *var_p1_t_db11_slot;
        let mut var_p1_t_db12: f64 = *var_p1_t_db12_slot;
        let mut var_p1_t_db13: f64 = *var_p1_t_db13_slot;
        let mut var_p1_t_db14: f64 = *var_p1_t_db14_slot;
        let mut var_p1_t_db2: f64 = *var_p1_t_db2_slot;
        let mut var_p1_t_db3: f64 = *var_p1_t_db3_slot;
        let mut var_p1_t_db4: f64 = *var_p1_t_db4_slot;
        let mut var_p1_t_db5: f64 = *var_p1_t_db5_slot;
        let mut var_p1_t_db6: f64 = *var_p1_t_db6_slot;
        let mut var_p1_t_db7: f64 = *var_p1_t_db7_slot;
        let mut var_p1_t_db8: f64 = *var_p1_t_db8_slot;
        let mut var_p1_t_db9: f64 = *var_p1_t_db9_slot;
        let mut var_p1_t_dn0: f64 = *var_p1_t_dn0_slot;
        let mut var_p1_t_dn1: f64 = *var_p1_t_dn1_slot;
        let mut var_p1_t_dn10: f64 = *var_p1_t_dn10_slot;
        let mut var_p1_t_dn11: f64 = *var_p1_t_dn11_slot;
        let mut var_p1_t_dn12: f64 = *var_p1_t_dn12_slot;
        let mut var_p1_t_dn13: f64 = *var_p1_t_dn13_slot;
        let mut var_p1_t_dn14: f64 = *var_p1_t_dn14_slot;
        let mut var_p1_t_dn15: f64 = *var_p1_t_dn15_slot;
        let mut var_p1_t_dn2: f64 = *var_p1_t_dn2_slot;
        let mut var_p1_t_dn3: f64 = *var_p1_t_dn3_slot;
        let mut var_p1_t_dn4: f64 = *var_p1_t_dn4_slot;
        let mut var_p1_t_dn5: f64 = *var_p1_t_dn5_slot;
        let mut var_p1_t_dn6: f64 = *var_p1_t_dn6_slot;
        let mut var_p1_t_dn7: f64 = *var_p1_t_dn7_slot;
        let mut var_p1_t_dn8: f64 = *var_p1_t_dn8_slot;
        let mut var_p1_t_dn9: f64 = *var_p1_t_dn9_slot;
        let mut var_p40_t: f64 = *var_p40_t_slot;
        let mut var_p40_t_db0: f64 = *var_p40_t_db0_slot;
        let mut var_p40_t_db1: f64 = *var_p40_t_db1_slot;
        let mut var_p40_t_db10: f64 = *var_p40_t_db10_slot;
        let mut var_p40_t_db11: f64 = *var_p40_t_db11_slot;
        let mut var_p40_t_db12: f64 = *var_p40_t_db12_slot;
        let mut var_p40_t_db13: f64 = *var_p40_t_db13_slot;
        let mut var_p40_t_db14: f64 = *var_p40_t_db14_slot;
        let mut var_p40_t_db2: f64 = *var_p40_t_db2_slot;
        let mut var_p40_t_db3: f64 = *var_p40_t_db3_slot;
        let mut var_p40_t_db4: f64 = *var_p40_t_db4_slot;
        let mut var_p40_t_db5: f64 = *var_p40_t_db5_slot;
        let mut var_p40_t_db6: f64 = *var_p40_t_db6_slot;
        let mut var_p40_t_db7: f64 = *var_p40_t_db7_slot;
        let mut var_p40_t_db8: f64 = *var_p40_t_db8_slot;
        let mut var_p40_t_db9: f64 = *var_p40_t_db9_slot;
        let mut var_p40_t_dn0: f64 = *var_p40_t_dn0_slot;
        let mut var_p40_t_dn1: f64 = *var_p40_t_dn1_slot;
        let mut var_p40_t_dn10: f64 = *var_p40_t_dn10_slot;
        let mut var_p40_t_dn11: f64 = *var_p40_t_dn11_slot;
        let mut var_p40_t_dn12: f64 = *var_p40_t_dn12_slot;
        let mut var_p40_t_dn13: f64 = *var_p40_t_dn13_slot;
        let mut var_p40_t_dn14: f64 = *var_p40_t_dn14_slot;
        let mut var_p40_t_dn15: f64 = *var_p40_t_dn15_slot;
        let mut var_p40_t_dn2: f64 = *var_p40_t_dn2_slot;
        let mut var_p40_t_dn3: f64 = *var_p40_t_dn3_slot;
        let mut var_p40_t_dn4: f64 = *var_p40_t_dn4_slot;
        let mut var_p40_t_dn5: f64 = *var_p40_t_dn5_slot;
        let mut var_p40_t_dn6: f64 = *var_p40_t_dn6_slot;
        let mut var_p40_t_dn7: f64 = *var_p40_t_dn7_slot;
        let mut var_p40_t_dn8: f64 = *var_p40_t_dn8_slot;
        let mut var_p40_t_dn9: f64 = *var_p40_t_dn9_slot;
        let mut var_vjg_t: f64 = *var_vjg_t_slot;
        let mut var_vjg_t_db0: f64 = *var_vjg_t_db0_slot;
        let mut var_vjg_t_db1: f64 = *var_vjg_t_db1_slot;
        let mut var_vjg_t_db10: f64 = *var_vjg_t_db10_slot;
        let mut var_vjg_t_db11: f64 = *var_vjg_t_db11_slot;
        let mut var_vjg_t_db12: f64 = *var_vjg_t_db12_slot;
        let mut var_vjg_t_db13: f64 = *var_vjg_t_db13_slot;
        let mut var_vjg_t_db14: f64 = *var_vjg_t_db14_slot;
        let mut var_vjg_t_db2: f64 = *var_vjg_t_db2_slot;
        let mut var_vjg_t_db3: f64 = *var_vjg_t_db3_slot;
        let mut var_vjg_t_db4: f64 = *var_vjg_t_db4_slot;
        let mut var_vjg_t_db5: f64 = *var_vjg_t_db5_slot;
        let mut var_vjg_t_db6: f64 = *var_vjg_t_db6_slot;
        let mut var_vjg_t_db7: f64 = *var_vjg_t_db7_slot;
        let mut var_vjg_t_db8: f64 = *var_vjg_t_db8_slot;
        let mut var_vjg_t_db9: f64 = *var_vjg_t_db9_slot;
        let mut var_vjg_t_dn0: f64 = *var_vjg_t_dn0_slot;
        let mut var_vjg_t_dn1: f64 = *var_vjg_t_dn1_slot;
        let mut var_vjg_t_dn10: f64 = *var_vjg_t_dn10_slot;
        let mut var_vjg_t_dn11: f64 = *var_vjg_t_dn11_slot;
        let mut var_vjg_t_dn12: f64 = *var_vjg_t_dn12_slot;
        let mut var_vjg_t_dn13: f64 = *var_vjg_t_dn13_slot;
        let mut var_vjg_t_dn14: f64 = *var_vjg_t_dn14_slot;
        let mut var_vjg_t_dn15: f64 = *var_vjg_t_dn15_slot;
        let mut var_vjg_t_dn2: f64 = *var_vjg_t_dn2_slot;
        let mut var_vjg_t_dn3: f64 = *var_vjg_t_dn3_slot;
        let mut var_vjg_t_dn4: f64 = *var_vjg_t_dn4_slot;
        let mut var_vjg_t_dn5: f64 = *var_vjg_t_dn5_slot;
        let mut var_vjg_t_dn6: f64 = *var_vjg_t_dn6_slot;
        let mut var_vjg_t_dn7: f64 = *var_vjg_t_dn7_slot;
        let mut var_vjg_t_dn8: f64 = *var_vjg_t_dn8_slot;
        let mut var_vjg_t_dn9: f64 = *var_vjg_t_dn9_slot;
        let mut var_vpks_t: f64 = *var_vpks_t_slot;
        let mut var_vpks_t_db0: f64 = *var_vpks_t_db0_slot;
        let mut var_vpks_t_db1: f64 = *var_vpks_t_db1_slot;
        let mut var_vpks_t_db10: f64 = *var_vpks_t_db10_slot;
        let mut var_vpks_t_db11: f64 = *var_vpks_t_db11_slot;
        let mut var_vpks_t_db12: f64 = *var_vpks_t_db12_slot;
        let mut var_vpks_t_db13: f64 = *var_vpks_t_db13_slot;
        let mut var_vpks_t_db14: f64 = *var_vpks_t_db14_slot;
        let mut var_vpks_t_db2: f64 = *var_vpks_t_db2_slot;
        let mut var_vpks_t_db3: f64 = *var_vpks_t_db3_slot;
        let mut var_vpks_t_db4: f64 = *var_vpks_t_db4_slot;
        let mut var_vpks_t_db5: f64 = *var_vpks_t_db5_slot;
        let mut var_vpks_t_db6: f64 = *var_vpks_t_db6_slot;
        let mut var_vpks_t_db7: f64 = *var_vpks_t_db7_slot;
        let mut var_vpks_t_db8: f64 = *var_vpks_t_db8_slot;
        let mut var_vpks_t_db9: f64 = *var_vpks_t_db9_slot;
        let mut var_vpks_t_dn0: f64 = *var_vpks_t_dn0_slot;
        let mut var_vpks_t_dn1: f64 = *var_vpks_t_dn1_slot;
        let mut var_vpks_t_dn10: f64 = *var_vpks_t_dn10_slot;
        let mut var_vpks_t_dn11: f64 = *var_vpks_t_dn11_slot;
        let mut var_vpks_t_dn12: f64 = *var_vpks_t_dn12_slot;
        let mut var_vpks_t_dn13: f64 = *var_vpks_t_dn13_slot;
        let mut var_vpks_t_dn14: f64 = *var_vpks_t_dn14_slot;
        let mut var_vpks_t_dn15: f64 = *var_vpks_t_dn15_slot;
        let mut var_vpks_t_dn2: f64 = *var_vpks_t_dn2_slot;
        let mut var_vpks_t_dn3: f64 = *var_vpks_t_dn3_slot;
        let mut var_vpks_t_dn4: f64 = *var_vpks_t_dn4_slot;
        let mut var_vpks_t_dn5: f64 = *var_vpks_t_dn5_slot;
        let mut var_vpks_t_dn6: f64 = *var_vpks_t_dn6_slot;
        let mut var_vpks_t_dn7: f64 = *var_vpks_t_dn7_slot;
        let mut var_vpks_t_dn8: f64 = *var_vpks_t_dn8_slot;
        let mut var_vpks_t_dn9: f64 = *var_vpks_t_dn9_slot;
        let mut var_vth: f64 = *var_vth_slot;
        let mut var_vth_db0: f64 = *var_vth_db0_slot;
        let mut var_vth_db1: f64 = *var_vth_db1_slot;
        let mut var_vth_db10: f64 = *var_vth_db10_slot;
        let mut var_vth_db11: f64 = *var_vth_db11_slot;
        let mut var_vth_db12: f64 = *var_vth_db12_slot;
        let mut var_vth_db13: f64 = *var_vth_db13_slot;
        let mut var_vth_db14: f64 = *var_vth_db14_slot;
        let mut var_vth_db2: f64 = *var_vth_db2_slot;
        let mut var_vth_db3: f64 = *var_vth_db3_slot;
        let mut var_vth_db4: f64 = *var_vth_db4_slot;
        let mut var_vth_db5: f64 = *var_vth_db5_slot;
        let mut var_vth_db6: f64 = *var_vth_db6_slot;
        let mut var_vth_db7: f64 = *var_vth_db7_slot;
        let mut var_vth_db8: f64 = *var_vth_db8_slot;
        let mut var_vth_db9: f64 = *var_vth_db9_slot;
        let mut var_vth_dn0: f64 = *var_vth_dn0_slot;
        let mut var_vth_dn1: f64 = *var_vth_dn1_slot;
        let mut var_vth_dn10: f64 = *var_vth_dn10_slot;
        let mut var_vth_dn11: f64 = *var_vth_dn11_slot;
        let mut var_vth_dn12: f64 = *var_vth_dn12_slot;
        let mut var_vth_dn13: f64 = *var_vth_dn13_slot;
        let mut var_vth_dn14: f64 = *var_vth_dn14_slot;
        let mut var_vth_dn15: f64 = *var_vth_dn15_slot;
        let mut var_vth_dn2: f64 = *var_vth_dn2_slot;
        let mut var_vth_dn3: f64 = *var_vth_dn3_slot;
        let mut var_vth_dn4: f64 = *var_vth_dn4_slot;
        let mut var_vth_dn5: f64 = *var_vth_dn5_slot;
        let mut var_vth_dn6: f64 = *var_vth_dn6_slot;
        let mut var_vth_dn7: f64 = *var_vth_dn7_slot;
        let mut var_vth_dn8: f64 = *var_vth_dn8_slot;
        let mut var_vth_dn9: f64 = *var_vth_dn9_slot;
        let mut var_vtr_t: f64 = *var_vtr_t_slot;
        let mut var_vtr_t_db0: f64 = *var_vtr_t_db0_slot;
        let mut var_vtr_t_db1: f64 = *var_vtr_t_db1_slot;
        let mut var_vtr_t_db10: f64 = *var_vtr_t_db10_slot;
        let mut var_vtr_t_db11: f64 = *var_vtr_t_db11_slot;
        let mut var_vtr_t_db12: f64 = *var_vtr_t_db12_slot;
        let mut var_vtr_t_db13: f64 = *var_vtr_t_db13_slot;
        let mut var_vtr_t_db14: f64 = *var_vtr_t_db14_slot;
        let mut var_vtr_t_db2: f64 = *var_vtr_t_db2_slot;
        let mut var_vtr_t_db3: f64 = *var_vtr_t_db3_slot;
        let mut var_vtr_t_db4: f64 = *var_vtr_t_db4_slot;
        let mut var_vtr_t_db5: f64 = *var_vtr_t_db5_slot;
        let mut var_vtr_t_db6: f64 = *var_vtr_t_db6_slot;
        let mut var_vtr_t_db7: f64 = *var_vtr_t_db7_slot;
        let mut var_vtr_t_db8: f64 = *var_vtr_t_db8_slot;
        let mut var_vtr_t_db9: f64 = *var_vtr_t_db9_slot;
        let mut var_vtr_t_dn0: f64 = *var_vtr_t_dn0_slot;
        let mut var_vtr_t_dn1: f64 = *var_vtr_t_dn1_slot;
        let mut var_vtr_t_dn10: f64 = *var_vtr_t_dn10_slot;
        let mut var_vtr_t_dn11: f64 = *var_vtr_t_dn11_slot;
        let mut var_vtr_t_dn12: f64 = *var_vtr_t_dn12_slot;
        let mut var_vtr_t_dn13: f64 = *var_vtr_t_dn13_slot;
        let mut var_vtr_t_dn14: f64 = *var_vtr_t_dn14_slot;
        let mut var_vtr_t_dn15: f64 = *var_vtr_t_dn15_slot;
        let mut var_vtr_t_dn2: f64 = *var_vtr_t_dn2_slot;
        let mut var_vtr_t_dn3: f64 = *var_vtr_t_dn3_slot;
        let mut var_vtr_t_dn4: f64 = *var_vtr_t_dn4_slot;
        let mut var_vtr_t_dn5: f64 = *var_vtr_t_dn5_slot;
        let mut var_vtr_t_dn6: f64 = *var_vtr_t_dn6_slot;
        let mut var_vtr_t_dn7: f64 = *var_vtr_t_dn7_slot;
        let mut var_vtr_t_dn8: f64 = *var_vtr_t_dn8_slot;
        let mut var_vtr_t_dn9: f64 = *var_vtr_t_dn9_slot;

        let assign190_e612: f64 = (var_t * THERMAL_VOLTAGE_PER_K);
        var_vth = assign190_e612;
        var_vth_dn0 = (var_t_dn0 * THERMAL_VOLTAGE_PER_K);
        var_vth_dn1 = (var_t_dn1 * THERMAL_VOLTAGE_PER_K);
        var_vth_dn2 = (var_t_dn2 * THERMAL_VOLTAGE_PER_K);
        var_vth_dn3 = (var_t_dn3 * THERMAL_VOLTAGE_PER_K);
        var_vth_dn4 = (var_t_dn4 * THERMAL_VOLTAGE_PER_K);
        var_vth_dn5 = (var_t_dn5 * THERMAL_VOLTAGE_PER_K);
        var_vth_dn6 = (var_t_dn6 * THERMAL_VOLTAGE_PER_K);
        var_vth_dn7 = (var_t_dn7 * THERMAL_VOLTAGE_PER_K);
        var_vth_dn8 = (var_t_dn8 * THERMAL_VOLTAGE_PER_K);
        var_vth_dn9 = (var_t_dn9 * THERMAL_VOLTAGE_PER_K);
        var_vth_dn10 = (var_t_dn10 * THERMAL_VOLTAGE_PER_K);
        var_vth_dn11 = (var_t_dn11 * THERMAL_VOLTAGE_PER_K);
        var_vth_dn12 = (var_t_dn12 * THERMAL_VOLTAGE_PER_K);
        var_vth_dn13 = (var_t_dn13 * THERMAL_VOLTAGE_PER_K);
        var_vth_dn14 = (var_t_dn14 * THERMAL_VOLTAGE_PER_K);
        var_vth_dn15 = (var_t_dn15 * THERMAL_VOLTAGE_PER_K);
        var_vth_db0 = (var_t_db0 * THERMAL_VOLTAGE_PER_K);
        var_vth_db1 = (var_t_db1 * THERMAL_VOLTAGE_PER_K);
        var_vth_db2 = (var_t_db2 * THERMAL_VOLTAGE_PER_K);
        var_vth_db3 = (var_t_db3 * THERMAL_VOLTAGE_PER_K);
        var_vth_db4 = (var_t_db4 * THERMAL_VOLTAGE_PER_K);
        var_vth_db5 = (var_t_db5 * THERMAL_VOLTAGE_PER_K);
        var_vth_db6 = (var_t_db6 * THERMAL_VOLTAGE_PER_K);
        var_vth_db7 = (var_t_db7 * THERMAL_VOLTAGE_PER_K);
        var_vth_db8 = (var_t_db8 * THERMAL_VOLTAGE_PER_K);
        var_vth_db9 = (var_t_db9 * THERMAL_VOLTAGE_PER_K);
        var_vth_db10 = (var_t_db10 * THERMAL_VOLTAGE_PER_K);
        var_vth_db11 = (var_t_db11 * THERMAL_VOLTAGE_PER_K);
        var_vth_db12 = (var_t_db12 * THERMAL_VOLTAGE_PER_K);
        var_vth_db13 = (var_t_db13 * THERMAL_VOLTAGE_PER_K);
        var_vth_db14 = (var_t_db14 * THERMAL_VOLTAGE_PER_K);

        let assign200_e615: f64 = (var_t - var_t_nom);
        let assign200_e616: f64 = (assign200_e615).abs();
        var_delta_t = assign200_e616;
        var_delta_t_dn0 = if assign200_e615 >= 0.0 { var_t_dn0 } else { (-var_t_dn0) };
        var_delta_t_dn1 = if assign200_e615 >= 0.0 { var_t_dn1 } else { (-var_t_dn1) };
        var_delta_t_dn2 = if assign200_e615 >= 0.0 { var_t_dn2 } else { (-var_t_dn2) };
        var_delta_t_dn3 = if assign200_e615 >= 0.0 { var_t_dn3 } else { (-var_t_dn3) };
        var_delta_t_dn4 = if assign200_e615 >= 0.0 { var_t_dn4 } else { (-var_t_dn4) };
        var_delta_t_dn5 = if assign200_e615 >= 0.0 { var_t_dn5 } else { (-var_t_dn5) };
        var_delta_t_dn6 = if assign200_e615 >= 0.0 { var_t_dn6 } else { (-var_t_dn6) };
        var_delta_t_dn7 = if assign200_e615 >= 0.0 { var_t_dn7 } else { (-var_t_dn7) };
        var_delta_t_dn8 = if assign200_e615 >= 0.0 { var_t_dn8 } else { (-var_t_dn8) };
        var_delta_t_dn9 = if assign200_e615 >= 0.0 { var_t_dn9 } else { (-var_t_dn9) };
        var_delta_t_dn10 = if assign200_e615 >= 0.0 { var_t_dn10 } else { (-var_t_dn10) };
        var_delta_t_dn11 = if assign200_e615 >= 0.0 { var_t_dn11 } else { (-var_t_dn11) };
        var_delta_t_dn12 = if assign200_e615 >= 0.0 { var_t_dn12 } else { (-var_t_dn12) };
        var_delta_t_dn13 = if assign200_e615 >= 0.0 { var_t_dn13 } else { (-var_t_dn13) };
        var_delta_t_dn14 = if assign200_e615 >= 0.0 { var_t_dn14 } else { (-var_t_dn14) };
        var_delta_t_dn15 = if assign200_e615 >= 0.0 { var_t_dn15 } else { (-var_t_dn15) };
        var_delta_t_db0 = if assign200_e615 >= 0.0 { var_t_db0 } else { (-var_t_db0) };
        var_delta_t_db1 = if assign200_e615 >= 0.0 { var_t_db1 } else { (-var_t_db1) };
        var_delta_t_db2 = if assign200_e615 >= 0.0 { var_t_db2 } else { (-var_t_db2) };
        var_delta_t_db3 = if assign200_e615 >= 0.0 { var_t_db3 } else { (-var_t_db3) };
        var_delta_t_db4 = if assign200_e615 >= 0.0 { var_t_db4 } else { (-var_t_db4) };
        var_delta_t_db5 = if assign200_e615 >= 0.0 { var_t_db5 } else { (-var_t_db5) };
        var_delta_t_db6 = if assign200_e615 >= 0.0 { var_t_db6 } else { (-var_t_db6) };
        var_delta_t_db7 = if assign200_e615 >= 0.0 { var_t_db7 } else { (-var_t_db7) };
        var_delta_t_db8 = if assign200_e615 >= 0.0 { var_t_db8 } else { (-var_t_db8) };
        var_delta_t_db9 = if assign200_e615 >= 0.0 { var_t_db9 } else { (-var_t_db9) };
        var_delta_t_db10 = if assign200_e615 >= 0.0 { var_t_db10 } else { (-var_t_db10) };
        var_delta_t_db11 = if assign200_e615 >= 0.0 { var_t_db11 } else { (-var_t_db11) };
        var_delta_t_db12 = if assign200_e615 >= 0.0 { var_t_db12 } else { (-var_t_db12) };
        var_delta_t_db13 = if assign200_e615 >= 0.0 { var_t_db13 } else { (-var_t_db13) };
        var_delta_t_db14 = if assign200_e615 >= 0.0 { var_t_db14 } else { (-var_t_db14) };

        let assign210_e623: f64 = if ((var_delta_t > 0.0) || (p.p57 > 0.0)) { 1.0 } else { 0.0 };
        var_guard3 = assign210_e623;

        let (assign240_e653, assign240_e653_d_n0, assign240_e653_d_n1, assign240_e653_d_n2, assign240_e653_d_n3, assign240_e653_d_n4, assign240_e653_d_n5, assign240_e653_d_n6, assign240_e653_d_n7, assign240_e653_d_n8, assign240_e653_d_n9, assign240_e653_d_n10, assign240_e653_d_n11, assign240_e653_d_n12, assign240_e653_d_n13, assign240_e653_d_n14, assign240_e653_d_n15, assign240_e653_d_b0, assign240_e653_d_b1, assign240_e653_d_b2, assign240_e653_d_b3, assign240_e653_d_b4, assign240_e653_d_b5, assign240_e653_d_b6, assign240_e653_d_b7, assign240_e653_d_b8, assign240_e653_d_b9, assign240_e653_d_b10, assign240_e653_d_b11, assign240_e653_d_b12, assign240_e653_d_b13, assign240_e653_d_b14,) = {
    if (var_guard3 != 0.0) {
        let assign240_e649: f64 = (p.p60 * var_delta_t);
        let assign240_e650: f64 = (1.0 + assign240_e649);
        let assign240_e651: f64 = (p.p11 * assign240_e650);
        (assign240_e651, (p.p11 * (p.p60 * var_delta_t_dn0)), (p.p11 * (p.p60 * var_delta_t_dn1)), (p.p11 * (p.p60 * var_delta_t_dn2)), (p.p11 * (p.p60 * var_delta_t_dn3)), (p.p11 * (p.p60 * var_delta_t_dn4)), (p.p11 * (p.p60 * var_delta_t_dn5)), (p.p11 * (p.p60 * var_delta_t_dn6)), (p.p11 * (p.p60 * var_delta_t_dn7)), (p.p11 * (p.p60 * var_delta_t_dn8)), (p.p11 * (p.p60 * var_delta_t_dn9)), (p.p11 * (p.p60 * var_delta_t_dn10)), (p.p11 * (p.p60 * var_delta_t_dn11)), (p.p11 * (p.p60 * var_delta_t_dn12)), (p.p11 * (p.p60 * var_delta_t_dn13)), (p.p11 * (p.p60 * var_delta_t_dn14)), (p.p11 * (p.p60 * var_delta_t_dn15)), (p.p11 * (p.p60 * var_delta_t_db0)), (p.p11 * (p.p60 * var_delta_t_db1)), (p.p11 * (p.p60 * var_delta_t_db2)), (p.p11 * (p.p60 * var_delta_t_db3)), (p.p11 * (p.p60 * var_delta_t_db4)), (p.p11 * (p.p60 * var_delta_t_db5)), (p.p11 * (p.p60 * var_delta_t_db6)), (p.p11 * (p.p60 * var_delta_t_db7)), (p.p11 * (p.p60 * var_delta_t_db8)), (p.p11 * (p.p60 * var_delta_t_db9)), (p.p11 * (p.p60 * var_delta_t_db10)), (p.p11 * (p.p60 * var_delta_t_db11)), (p.p11 * (p.p60 * var_delta_t_db12)), (p.p11 * (p.p60 * var_delta_t_db13)), (p.p11 * (p.p60 * var_delta_t_db14)),)
    } else {
        (var_p1_t, var_p1_t_dn0, var_p1_t_dn1, var_p1_t_dn2, var_p1_t_dn3, var_p1_t_dn4, var_p1_t_dn5, var_p1_t_dn6, var_p1_t_dn7, var_p1_t_dn8, var_p1_t_dn9, var_p1_t_dn10, var_p1_t_dn11, var_p1_t_dn12, var_p1_t_dn13, var_p1_t_dn14, var_p1_t_dn15, var_p1_t_db0, var_p1_t_db1, var_p1_t_db2, var_p1_t_db3, var_p1_t_db4, var_p1_t_db5, var_p1_t_db6, var_p1_t_db7, var_p1_t_db8, var_p1_t_db9, var_p1_t_db10, var_p1_t_db11, var_p1_t_db12, var_p1_t_db13, var_p1_t_db14,)
    }
};
        var_p1_t = assign240_e653;
        var_p1_t_dn0 = assign240_e653_d_n0;
        var_p1_t_dn1 = assign240_e653_d_n1;
        var_p1_t_dn2 = assign240_e653_d_n2;
        var_p1_t_dn3 = assign240_e653_d_n3;
        var_p1_t_dn4 = assign240_e653_d_n4;
        var_p1_t_dn5 = assign240_e653_d_n5;
        var_p1_t_dn6 = assign240_e653_d_n6;
        var_p1_t_dn7 = assign240_e653_d_n7;
        var_p1_t_dn8 = assign240_e653_d_n8;
        var_p1_t_dn9 = assign240_e653_d_n9;
        var_p1_t_dn10 = assign240_e653_d_n10;
        var_p1_t_dn11 = assign240_e653_d_n11;
        var_p1_t_dn12 = assign240_e653_d_n12;
        var_p1_t_dn13 = assign240_e653_d_n13;
        var_p1_t_dn14 = assign240_e653_d_n14;
        var_p1_t_dn15 = assign240_e653_d_n15;
        var_p1_t_db0 = assign240_e653_d_b0;
        var_p1_t_db1 = assign240_e653_d_b1;
        var_p1_t_db2 = assign240_e653_d_b2;
        var_p1_t_db3 = assign240_e653_d_b3;
        var_p1_t_db4 = assign240_e653_d_b4;
        var_p1_t_db5 = assign240_e653_d_b5;
        var_p1_t_db6 = assign240_e653_d_b6;
        var_p1_t_db7 = assign240_e653_d_b7;
        var_p1_t_db8 = assign240_e653_d_b8;
        var_p1_t_db9 = assign240_e653_d_b9;
        var_p1_t_db10 = assign240_e653_d_b10;
        var_p1_t_db11 = assign240_e653_d_b11;
        var_p1_t_db12 = assign240_e653_d_b12;
        var_p1_t_db13 = assign240_e653_d_b13;
        var_p1_t_db14 = assign240_e653_d_b14;

        let (assign260_e673, assign260_e673_d_n0, assign260_e673_d_n1, assign260_e673_d_n2, assign260_e673_d_n3, assign260_e673_d_n4, assign260_e673_d_n5, assign260_e673_d_n6, assign260_e673_d_n7, assign260_e673_d_n8, assign260_e673_d_n9, assign260_e673_d_n10, assign260_e673_d_n11, assign260_e673_d_n12, assign260_e673_d_n13, assign260_e673_d_n14, assign260_e673_d_n15, assign260_e673_d_b0, assign260_e673_d_b1, assign260_e673_d_b2, assign260_e673_d_b3, assign260_e673_d_b4, assign260_e673_d_b5, assign260_e673_d_b6, assign260_e673_d_b7, assign260_e673_d_b8, assign260_e673_d_b9, assign260_e673_d_b10, assign260_e673_d_b11, assign260_e673_d_b12, assign260_e673_d_b13, assign260_e673_d_b14,) = {
    if (var_guard3 != 0.0) {
        let assign260_e669: f64 = (p.p61 * var_delta_t);
        let assign260_e670: f64 = (1.0 + assign260_e669);
        let assign260_e671: f64 = (p.p25 * assign260_e670);
        (assign260_e671, (p.p25 * (p.p61 * var_delta_t_dn0)), (p.p25 * (p.p61 * var_delta_t_dn1)), (p.p25 * (p.p61 * var_delta_t_dn2)), (p.p25 * (p.p61 * var_delta_t_dn3)), (p.p25 * (p.p61 * var_delta_t_dn4)), (p.p25 * (p.p61 * var_delta_t_dn5)), (p.p25 * (p.p61 * var_delta_t_dn6)), (p.p25 * (p.p61 * var_delta_t_dn7)), (p.p25 * (p.p61 * var_delta_t_dn8)), (p.p25 * (p.p61 * var_delta_t_dn9)), (p.p25 * (p.p61 * var_delta_t_dn10)), (p.p25 * (p.p61 * var_delta_t_dn11)), (p.p25 * (p.p61 * var_delta_t_dn12)), (p.p25 * (p.p61 * var_delta_t_dn13)), (p.p25 * (p.p61 * var_delta_t_dn14)), (p.p25 * (p.p61 * var_delta_t_dn15)), (p.p25 * (p.p61 * var_delta_t_db0)), (p.p25 * (p.p61 * var_delta_t_db1)), (p.p25 * (p.p61 * var_delta_t_db2)), (p.p25 * (p.p61 * var_delta_t_db3)), (p.p25 * (p.p61 * var_delta_t_db4)), (p.p25 * (p.p61 * var_delta_t_db5)), (p.p25 * (p.p61 * var_delta_t_db6)), (p.p25 * (p.p61 * var_delta_t_db7)), (p.p25 * (p.p61 * var_delta_t_db8)), (p.p25 * (p.p61 * var_delta_t_db9)), (p.p25 * (p.p61 * var_delta_t_db10)), (p.p25 * (p.p61 * var_delta_t_db11)), (p.p25 * (p.p61 * var_delta_t_db12)), (p.p25 * (p.p61 * var_delta_t_db13)), (p.p25 * (p.p61 * var_delta_t_db14)),)
    } else {
        (var_cgs0_t, var_cgs0_t_dn0, var_cgs0_t_dn1, var_cgs0_t_dn2, var_cgs0_t_dn3, var_cgs0_t_dn4, var_cgs0_t_dn5, var_cgs0_t_dn6, var_cgs0_t_dn7, var_cgs0_t_dn8, var_cgs0_t_dn9, var_cgs0_t_dn10, var_cgs0_t_dn11, var_cgs0_t_dn12, var_cgs0_t_dn13, var_cgs0_t_dn14, var_cgs0_t_dn15, var_cgs0_t_db0, var_cgs0_t_db1, var_cgs0_t_db2, var_cgs0_t_db3, var_cgs0_t_db4, var_cgs0_t_db5, var_cgs0_t_db6, var_cgs0_t_db7, var_cgs0_t_db8, var_cgs0_t_db9, var_cgs0_t_db10, var_cgs0_t_db11, var_cgs0_t_db12, var_cgs0_t_db13, var_cgs0_t_db14,)
    }
};
        var_cgs0_t = assign260_e673;
        var_cgs0_t_dn0 = assign260_e673_d_n0;
        var_cgs0_t_dn1 = assign260_e673_d_n1;
        var_cgs0_t_dn2 = assign260_e673_d_n2;
        var_cgs0_t_dn3 = assign260_e673_d_n3;
        var_cgs0_t_dn4 = assign260_e673_d_n4;
        var_cgs0_t_dn5 = assign260_e673_d_n5;
        var_cgs0_t_dn6 = assign260_e673_d_n6;
        var_cgs0_t_dn7 = assign260_e673_d_n7;
        var_cgs0_t_dn8 = assign260_e673_d_n8;
        var_cgs0_t_dn9 = assign260_e673_d_n9;
        var_cgs0_t_dn10 = assign260_e673_d_n10;
        var_cgs0_t_dn11 = assign260_e673_d_n11;
        var_cgs0_t_dn12 = assign260_e673_d_n12;
        var_cgs0_t_dn13 = assign260_e673_d_n13;
        var_cgs0_t_dn14 = assign260_e673_d_n14;
        var_cgs0_t_dn15 = assign260_e673_d_n15;
        var_cgs0_t_db0 = assign260_e673_d_b0;
        var_cgs0_t_db1 = assign260_e673_d_b1;
        var_cgs0_t_db2 = assign260_e673_d_b2;
        var_cgs0_t_db3 = assign260_e673_d_b3;
        var_cgs0_t_db4 = assign260_e673_d_b4;
        var_cgs0_t_db5 = assign260_e673_d_b5;
        var_cgs0_t_db6 = assign260_e673_d_b6;
        var_cgs0_t_db7 = assign260_e673_d_b7;
        var_cgs0_t_db8 = assign260_e673_d_b8;
        var_cgs0_t_db9 = assign260_e673_d_b9;
        var_cgs0_t_db10 = assign260_e673_d_b10;
        var_cgs0_t_db11 = assign260_e673_d_b11;
        var_cgs0_t_db12 = assign260_e673_d_b12;
        var_cgs0_t_db13 = assign260_e673_d_b13;
        var_cgs0_t_db14 = assign260_e673_d_b14;

        let (assign270_e683, assign270_e683_d_n0, assign270_e683_d_n1, assign270_e683_d_n2, assign270_e683_d_n3, assign270_e683_d_n4, assign270_e683_d_n5, assign270_e683_d_n6, assign270_e683_d_n7, assign270_e683_d_n8, assign270_e683_d_n9, assign270_e683_d_n10, assign270_e683_d_n11, assign270_e683_d_n12, assign270_e683_d_n13, assign270_e683_d_n14, assign270_e683_d_n15, assign270_e683_d_b0, assign270_e683_d_b1, assign270_e683_d_b2, assign270_e683_d_b3, assign270_e683_d_b4, assign270_e683_d_b5, assign270_e683_d_b6, assign270_e683_d_b7, assign270_e683_d_b8, assign270_e683_d_b9, assign270_e683_d_b10, assign270_e683_d_b11, assign270_e683_d_b12, assign270_e683_d_b13, assign270_e683_d_b14,) = {
    if (var_guard3 != 0.0) {
        let assign270_e679: f64 = (p.p62 * var_delta_t);
        let assign270_e680: f64 = (1.0 + assign270_e679);
        let assign270_e681: f64 = (p.p28 * assign270_e680);
        (assign270_e681, (p.p28 * (p.p62 * var_delta_t_dn0)), (p.p28 * (p.p62 * var_delta_t_dn1)), (p.p28 * (p.p62 * var_delta_t_dn2)), (p.p28 * (p.p62 * var_delta_t_dn3)), (p.p28 * (p.p62 * var_delta_t_dn4)), (p.p28 * (p.p62 * var_delta_t_dn5)), (p.p28 * (p.p62 * var_delta_t_dn6)), (p.p28 * (p.p62 * var_delta_t_dn7)), (p.p28 * (p.p62 * var_delta_t_dn8)), (p.p28 * (p.p62 * var_delta_t_dn9)), (p.p28 * (p.p62 * var_delta_t_dn10)), (p.p28 * (p.p62 * var_delta_t_dn11)), (p.p28 * (p.p62 * var_delta_t_dn12)), (p.p28 * (p.p62 * var_delta_t_dn13)), (p.p28 * (p.p62 * var_delta_t_dn14)), (p.p28 * (p.p62 * var_delta_t_dn15)), (p.p28 * (p.p62 * var_delta_t_db0)), (p.p28 * (p.p62 * var_delta_t_db1)), (p.p28 * (p.p62 * var_delta_t_db2)), (p.p28 * (p.p62 * var_delta_t_db3)), (p.p28 * (p.p62 * var_delta_t_db4)), (p.p28 * (p.p62 * var_delta_t_db5)), (p.p28 * (p.p62 * var_delta_t_db6)), (p.p28 * (p.p62 * var_delta_t_db7)), (p.p28 * (p.p62 * var_delta_t_db8)), (p.p28 * (p.p62 * var_delta_t_db9)), (p.p28 * (p.p62 * var_delta_t_db10)), (p.p28 * (p.p62 * var_delta_t_db11)), (p.p28 * (p.p62 * var_delta_t_db12)), (p.p28 * (p.p62 * var_delta_t_db13)), (p.p28 * (p.p62 * var_delta_t_db14)),)
    } else {
        (var_cgd0_t, var_cgd0_t_dn0, var_cgd0_t_dn1, var_cgd0_t_dn2, var_cgd0_t_dn3, var_cgd0_t_dn4, var_cgd0_t_dn5, var_cgd0_t_dn6, var_cgd0_t_dn7, var_cgd0_t_dn8, var_cgd0_t_dn9, var_cgd0_t_dn10, var_cgd0_t_dn11, var_cgd0_t_dn12, var_cgd0_t_dn13, var_cgd0_t_dn14, var_cgd0_t_dn15, var_cgd0_t_db0, var_cgd0_t_db1, var_cgd0_t_db2, var_cgd0_t_db3, var_cgd0_t_db4, var_cgd0_t_db5, var_cgd0_t_db6, var_cgd0_t_db7, var_cgd0_t_db8, var_cgd0_t_db9, var_cgd0_t_db10, var_cgd0_t_db11, var_cgd0_t_db12, var_cgd0_t_db13, var_cgd0_t_db14,)
    }
};
        var_cgd0_t = assign270_e683;
        var_cgd0_t_dn0 = assign270_e683_d_n0;
        var_cgd0_t_dn1 = assign270_e683_d_n1;
        var_cgd0_t_dn2 = assign270_e683_d_n2;
        var_cgd0_t_dn3 = assign270_e683_d_n3;
        var_cgd0_t_dn4 = assign270_e683_d_n4;
        var_cgd0_t_dn5 = assign270_e683_d_n5;
        var_cgd0_t_dn6 = assign270_e683_d_n6;
        var_cgd0_t_dn7 = assign270_e683_d_n7;
        var_cgd0_t_dn8 = assign270_e683_d_n8;
        var_cgd0_t_dn9 = assign270_e683_d_n9;
        var_cgd0_t_dn10 = assign270_e683_d_n10;
        var_cgd0_t_dn11 = assign270_e683_d_n11;
        var_cgd0_t_dn12 = assign270_e683_d_n12;
        var_cgd0_t_dn13 = assign270_e683_d_n13;
        var_cgd0_t_dn14 = assign270_e683_d_n14;
        var_cgd0_t_dn15 = assign270_e683_d_n15;
        var_cgd0_t_db0 = assign270_e683_d_b0;
        var_cgd0_t_db1 = assign270_e683_d_b1;
        var_cgd0_t_db2 = assign270_e683_d_b2;
        var_cgd0_t_db3 = assign270_e683_d_b3;
        var_cgd0_t_db4 = assign270_e683_d_b4;
        var_cgd0_t_db5 = assign270_e683_d_b5;
        var_cgd0_t_db6 = assign270_e683_d_b6;
        var_cgd0_t_db7 = assign270_e683_d_b7;
        var_cgd0_t_db8 = assign270_e683_d_b8;
        var_cgd0_t_db9 = assign270_e683_d_b9;
        var_cgd0_t_db10 = assign270_e683_d_b10;
        var_cgd0_t_db11 = assign270_e683_d_b11;
        var_cgd0_t_db12 = assign270_e683_d_b12;
        var_cgd0_t_db13 = assign270_e683_d_b13;
        var_cgd0_t_db14 = assign270_e683_d_b14;

        let (assign300_e711, assign300_e711_d_n0, assign300_e711_d_n1, assign300_e711_d_n2, assign300_e711_d_n3, assign300_e711_d_n4, assign300_e711_d_n5, assign300_e711_d_n6, assign300_e711_d_n7, assign300_e711_d_n8, assign300_e711_d_n9, assign300_e711_d_n10, assign300_e711_d_n11, assign300_e711_d_n12, assign300_e711_d_n13, assign300_e711_d_n14, assign300_e711_d_n15, assign300_e711_d_b0, assign300_e711_d_b1, assign300_e711_d_b2, assign300_e711_d_b3, assign300_e711_d_b4, assign300_e711_d_b5, assign300_e711_d_b6, assign300_e711_d_b7, assign300_e711_d_b8, assign300_e711_d_b9, assign300_e711_d_b10, assign300_e711_d_b11, assign300_e711_d_b12, assign300_e711_d_b13, assign300_e711_d_b14,) = {
    if (var_guard3 != 0.0) {
        let assign300_e708: f64 = (p.p68 * var_delta_t);
        let assign300_e709: f64 = (p.p9 + assign300_e708);
        (assign300_e709, (p.p68 * var_delta_t_dn0), (p.p68 * var_delta_t_dn1), (p.p68 * var_delta_t_dn2), (p.p68 * var_delta_t_dn3), (p.p68 * var_delta_t_dn4), (p.p68 * var_delta_t_dn5), (p.p68 * var_delta_t_dn6), (p.p68 * var_delta_t_dn7), (p.p68 * var_delta_t_dn8), (p.p68 * var_delta_t_dn9), (p.p68 * var_delta_t_dn10), (p.p68 * var_delta_t_dn11), (p.p68 * var_delta_t_dn12), (p.p68 * var_delta_t_dn13), (p.p68 * var_delta_t_dn14), (p.p68 * var_delta_t_dn15), (p.p68 * var_delta_t_db0), (p.p68 * var_delta_t_db1), (p.p68 * var_delta_t_db2), (p.p68 * var_delta_t_db3), (p.p68 * var_delta_t_db4), (p.p68 * var_delta_t_db5), (p.p68 * var_delta_t_db6), (p.p68 * var_delta_t_db7), (p.p68 * var_delta_t_db8), (p.p68 * var_delta_t_db9), (p.p68 * var_delta_t_db10), (p.p68 * var_delta_t_db11), (p.p68 * var_delta_t_db12), (p.p68 * var_delta_t_db13), (p.p68 * var_delta_t_db14),)
    } else {
        (var_vpks_t, var_vpks_t_dn0, var_vpks_t_dn1, var_vpks_t_dn2, var_vpks_t_dn3, var_vpks_t_dn4, var_vpks_t_dn5, var_vpks_t_dn6, var_vpks_t_dn7, var_vpks_t_dn8, var_vpks_t_dn9, var_vpks_t_dn10, var_vpks_t_dn11, var_vpks_t_dn12, var_vpks_t_dn13, var_vpks_t_dn14, var_vpks_t_dn15, var_vpks_t_db0, var_vpks_t_db1, var_vpks_t_db2, var_vpks_t_db3, var_vpks_t_db4, var_vpks_t_db5, var_vpks_t_db6, var_vpks_t_db7, var_vpks_t_db8, var_vpks_t_db9, var_vpks_t_db10, var_vpks_t_db11, var_vpks_t_db12, var_vpks_t_db13, var_vpks_t_db14,)
    }
};
        var_vpks_t = assign300_e711;
        var_vpks_t_dn0 = assign300_e711_d_n0;
        var_vpks_t_dn1 = assign300_e711_d_n1;
        var_vpks_t_dn2 = assign300_e711_d_n2;
        var_vpks_t_dn3 = assign300_e711_d_n3;
        var_vpks_t_dn4 = assign300_e711_d_n4;
        var_vpks_t_dn5 = assign300_e711_d_n5;
        var_vpks_t_dn6 = assign300_e711_d_n6;
        var_vpks_t_dn7 = assign300_e711_d_n7;
        var_vpks_t_dn8 = assign300_e711_d_n8;
        var_vpks_t_dn9 = assign300_e711_d_n9;
        var_vpks_t_dn10 = assign300_e711_d_n10;
        var_vpks_t_dn11 = assign300_e711_d_n11;
        var_vpks_t_dn12 = assign300_e711_d_n12;
        var_vpks_t_dn13 = assign300_e711_d_n13;
        var_vpks_t_dn14 = assign300_e711_d_n14;
        var_vpks_t_dn15 = assign300_e711_d_n15;
        var_vpks_t_db0 = assign300_e711_d_b0;
        var_vpks_t_db1 = assign300_e711_d_b1;
        var_vpks_t_db2 = assign300_e711_d_b2;
        var_vpks_t_db3 = assign300_e711_d_b3;
        var_vpks_t_db4 = assign300_e711_d_b4;
        var_vpks_t_db5 = assign300_e711_d_b5;
        var_vpks_t_db6 = assign300_e711_d_b6;
        var_vpks_t_db7 = assign300_e711_d_b7;
        var_vpks_t_db8 = assign300_e711_d_b8;
        var_vpks_t_db9 = assign300_e711_d_b9;
        var_vpks_t_db10 = assign300_e711_d_b10;
        var_vpks_t_db11 = assign300_e711_d_b11;
        var_vpks_t_db12 = assign300_e711_d_b12;
        var_vpks_t_db13 = assign300_e711_d_b13;
        var_vpks_t_db14 = assign300_e711_d_b14;

        let (assign310_e721, assign310_e721_d_n0, assign310_e721_d_n1, assign310_e721_d_n2, assign310_e721_d_n3, assign310_e721_d_n4, assign310_e721_d_n5, assign310_e721_d_n6, assign310_e721_d_n7, assign310_e721_d_n8, assign310_e721_d_n9, assign310_e721_d_n10, assign310_e721_d_n11, assign310_e721_d_n12, assign310_e721_d_n13, assign310_e721_d_n14, assign310_e721_d_n15, assign310_e721_d_b0, assign310_e721_d_b1, assign310_e721_d_b2, assign310_e721_d_b3, assign310_e721_d_b4, assign310_e721_d_b5, assign310_e721_d_b6, assign310_e721_d_b7, assign310_e721_d_b8, assign310_e721_d_b9, assign310_e721_d_b10, assign310_e721_d_b11, assign310_e721_d_b12, assign310_e721_d_b13, assign310_e721_d_b14,) = {
    if (var_guard3 != 0.0) {
        let assign310_e716: f64 = (p.p30 * p.p68);
        let assign310_e718: f64 = (assign310_e716 * var_delta_t);
        let assign310_e719: f64 = (p.p29 + assign310_e718);
        (assign310_e719, (assign310_e716 * var_delta_t_dn0), (assign310_e716 * var_delta_t_dn1), (assign310_e716 * var_delta_t_dn2), (assign310_e716 * var_delta_t_dn3), (assign310_e716 * var_delta_t_dn4), (assign310_e716 * var_delta_t_dn5), (assign310_e716 * var_delta_t_dn6), (assign310_e716 * var_delta_t_dn7), (assign310_e716 * var_delta_t_dn8), (assign310_e716 * var_delta_t_dn9), (assign310_e716 * var_delta_t_dn10), (assign310_e716 * var_delta_t_dn11), (assign310_e716 * var_delta_t_dn12), (assign310_e716 * var_delta_t_dn13), (assign310_e716 * var_delta_t_dn14), (assign310_e716 * var_delta_t_dn15), (assign310_e716 * var_delta_t_db0), (assign310_e716 * var_delta_t_db1), (assign310_e716 * var_delta_t_db2), (assign310_e716 * var_delta_t_db3), (assign310_e716 * var_delta_t_db4), (assign310_e716 * var_delta_t_db5), (assign310_e716 * var_delta_t_db6), (assign310_e716 * var_delta_t_db7), (assign310_e716 * var_delta_t_db8), (assign310_e716 * var_delta_t_db9), (assign310_e716 * var_delta_t_db10), (assign310_e716 * var_delta_t_db11), (assign310_e716 * var_delta_t_db12), (assign310_e716 * var_delta_t_db13), (assign310_e716 * var_delta_t_db14),)
    } else {
        (var_p10_t, var_p10_t_dn0, var_p10_t_dn1, var_p10_t_dn2, var_p10_t_dn3, var_p10_t_dn4, var_p10_t_dn5, var_p10_t_dn6, var_p10_t_dn7, var_p10_t_dn8, var_p10_t_dn9, var_p10_t_dn10, var_p10_t_dn11, var_p10_t_dn12, var_p10_t_dn13, var_p10_t_dn14, var_p10_t_dn15, var_p10_t_db0, var_p10_t_db1, var_p10_t_db2, var_p10_t_db3, var_p10_t_db4, var_p10_t_db5, var_p10_t_db6, var_p10_t_db7, var_p10_t_db8, var_p10_t_db9, var_p10_t_db10, var_p10_t_db11, var_p10_t_db12, var_p10_t_db13, var_p10_t_db14,)
    }
};
        var_p10_t = assign310_e721;
        var_p10_t_dn0 = assign310_e721_d_n0;
        var_p10_t_dn1 = assign310_e721_d_n1;
        var_p10_t_dn2 = assign310_e721_d_n2;
        var_p10_t_dn3 = assign310_e721_d_n3;
        var_p10_t_dn4 = assign310_e721_d_n4;
        var_p10_t_dn5 = assign310_e721_d_n5;
        var_p10_t_dn6 = assign310_e721_d_n6;
        var_p10_t_dn7 = assign310_e721_d_n7;
        var_p10_t_dn8 = assign310_e721_d_n8;
        var_p10_t_dn9 = assign310_e721_d_n9;
        var_p10_t_dn10 = assign310_e721_d_n10;
        var_p10_t_dn11 = assign310_e721_d_n11;
        var_p10_t_dn12 = assign310_e721_d_n12;
        var_p10_t_dn13 = assign310_e721_d_n13;
        var_p10_t_dn14 = assign310_e721_d_n14;
        var_p10_t_dn15 = assign310_e721_d_n15;
        var_p10_t_db0 = assign310_e721_d_b0;
        var_p10_t_db1 = assign310_e721_d_b1;
        var_p10_t_db2 = assign310_e721_d_b2;
        var_p10_t_db3 = assign310_e721_d_b3;
        var_p10_t_db4 = assign310_e721_d_b4;
        var_p10_t_db5 = assign310_e721_d_b5;
        var_p10_t_db6 = assign310_e721_d_b6;
        var_p10_t_db7 = assign310_e721_d_b7;
        var_p10_t_db8 = assign310_e721_d_b8;
        var_p10_t_db9 = assign310_e721_d_b9;
        var_p10_t_db10 = assign310_e721_d_b10;
        var_p10_t_db11 = assign310_e721_d_b11;
        var_p10_t_db12 = assign310_e721_d_b12;
        var_p10_t_db13 = assign310_e721_d_b13;
        var_p10_t_db14 = assign310_e721_d_b14;

        let (assign320_e731, assign320_e731_d_n0, assign320_e731_d_n1, assign320_e731_d_n2, assign320_e731_d_n3, assign320_e731_d_n4, assign320_e731_d_n5, assign320_e731_d_n6, assign320_e731_d_n7, assign320_e731_d_n8, assign320_e731_d_n9, assign320_e731_d_n10, assign320_e731_d_n11, assign320_e731_d_n12, assign320_e731_d_n13, assign320_e731_d_n14, assign320_e731_d_n15, assign320_e731_d_b0, assign320_e731_d_b1, assign320_e731_d_b2, assign320_e731_d_b3, assign320_e731_d_b4, assign320_e731_d_b5, assign320_e731_d_b6, assign320_e731_d_b7, assign320_e731_d_b8, assign320_e731_d_b9, assign320_e731_d_b10, assign320_e731_d_b11, assign320_e731_d_b12, assign320_e731_d_b13, assign320_e731_d_b14,) = {
    if (var_guard3 != 0.0) {
        let assign320_e726: f64 = (p.p36 * p.p68);
        let assign320_e728: f64 = (assign320_e726 * var_delta_t);
        let assign320_e729: f64 = (p.p35 + assign320_e728);
        (assign320_e729, (assign320_e726 * var_delta_t_dn0), (assign320_e726 * var_delta_t_dn1), (assign320_e726 * var_delta_t_dn2), (assign320_e726 * var_delta_t_dn3), (assign320_e726 * var_delta_t_dn4), (assign320_e726 * var_delta_t_dn5), (assign320_e726 * var_delta_t_dn6), (assign320_e726 * var_delta_t_dn7), (assign320_e726 * var_delta_t_dn8), (assign320_e726 * var_delta_t_dn9), (assign320_e726 * var_delta_t_dn10), (assign320_e726 * var_delta_t_dn11), (assign320_e726 * var_delta_t_dn12), (assign320_e726 * var_delta_t_dn13), (assign320_e726 * var_delta_t_dn14), (assign320_e726 * var_delta_t_dn15), (assign320_e726 * var_delta_t_db0), (assign320_e726 * var_delta_t_db1), (assign320_e726 * var_delta_t_db2), (assign320_e726 * var_delta_t_db3), (assign320_e726 * var_delta_t_db4), (assign320_e726 * var_delta_t_db5), (assign320_e726 * var_delta_t_db6), (assign320_e726 * var_delta_t_db7), (assign320_e726 * var_delta_t_db8), (assign320_e726 * var_delta_t_db9), (assign320_e726 * var_delta_t_db10), (assign320_e726 * var_delta_t_db11), (assign320_e726 * var_delta_t_db12), (assign320_e726 * var_delta_t_db13), (assign320_e726 * var_delta_t_db14),)
    } else {
        (var_p40_t, var_p40_t_dn0, var_p40_t_dn1, var_p40_t_dn2, var_p40_t_dn3, var_p40_t_dn4, var_p40_t_dn5, var_p40_t_dn6, var_p40_t_dn7, var_p40_t_dn8, var_p40_t_dn9, var_p40_t_dn10, var_p40_t_dn11, var_p40_t_dn12, var_p40_t_dn13, var_p40_t_dn14, var_p40_t_dn15, var_p40_t_db0, var_p40_t_db1, var_p40_t_db2, var_p40_t_db3, var_p40_t_db4, var_p40_t_db5, var_p40_t_db6, var_p40_t_db7, var_p40_t_db8, var_p40_t_db9, var_p40_t_db10, var_p40_t_db11, var_p40_t_db12, var_p40_t_db13, var_p40_t_db14,)
    }
};
        var_p40_t = assign320_e731;
        var_p40_t_dn0 = assign320_e731_d_n0;
        var_p40_t_dn1 = assign320_e731_d_n1;
        var_p40_t_dn2 = assign320_e731_d_n2;
        var_p40_t_dn3 = assign320_e731_d_n3;
        var_p40_t_dn4 = assign320_e731_d_n4;
        var_p40_t_dn5 = assign320_e731_d_n5;
        var_p40_t_dn6 = assign320_e731_d_n6;
        var_p40_t_dn7 = assign320_e731_d_n7;
        var_p40_t_dn8 = assign320_e731_d_n8;
        var_p40_t_dn9 = assign320_e731_d_n9;
        var_p40_t_dn10 = assign320_e731_d_n10;
        var_p40_t_dn11 = assign320_e731_d_n11;
        var_p40_t_dn12 = assign320_e731_d_n12;
        var_p40_t_dn13 = assign320_e731_d_n13;
        var_p40_t_dn14 = assign320_e731_d_n14;
        var_p40_t_dn15 = assign320_e731_d_n15;
        var_p40_t_db0 = assign320_e731_d_b0;
        var_p40_t_db1 = assign320_e731_d_b1;
        var_p40_t_db2 = assign320_e731_d_b2;
        var_p40_t_db3 = assign320_e731_d_b3;
        var_p40_t_db4 = assign320_e731_d_b4;
        var_p40_t_db5 = assign320_e731_d_b5;
        var_p40_t_db6 = assign320_e731_d_b6;
        var_p40_t_db7 = assign320_e731_d_b7;
        var_p40_t_db8 = assign320_e731_d_b8;
        var_p40_t_db9 = assign320_e731_d_b9;
        var_p40_t_db10 = assign320_e731_d_b10;
        var_p40_t_db11 = assign320_e731_d_b11;
        var_p40_t_db12 = assign320_e731_d_b12;
        var_p40_t_db13 = assign320_e731_d_b13;
        var_p40_t_db14 = assign320_e731_d_b14;

        let (assign330_e739, assign330_e739_d_n0, assign330_e739_d_n1, assign330_e739_d_n2, assign330_e739_d_n3, assign330_e739_d_n4, assign330_e739_d_n5, assign330_e739_d_n6, assign330_e739_d_n7, assign330_e739_d_n8, assign330_e739_d_n9, assign330_e739_d_n10, assign330_e739_d_n11, assign330_e739_d_n12, assign330_e739_d_n13, assign330_e739_d_n14, assign330_e739_d_n15, assign330_e739_d_b0, assign330_e739_d_b1, assign330_e739_d_b2, assign330_e739_d_b3, assign330_e739_d_b4, assign330_e739_d_b5, assign330_e739_d_b6, assign330_e739_d_b7, assign330_e739_d_b8, assign330_e739_d_b9, assign330_e739_d_b10, assign330_e739_d_b11, assign330_e739_d_b12, assign330_e739_d_b13, assign330_e739_d_b14,) = {
    if (var_guard3 != 0.0) {
        let assign330_e736: f64 = (p.p69 * var_delta_t);
        let assign330_e737: f64 = (p.p41 + assign330_e736);
        (assign330_e737, (p.p69 * var_delta_t_dn0), (p.p69 * var_delta_t_dn1), (p.p69 * var_delta_t_dn2), (p.p69 * var_delta_t_dn3), (p.p69 * var_delta_t_dn4), (p.p69 * var_delta_t_dn5), (p.p69 * var_delta_t_dn6), (p.p69 * var_delta_t_dn7), (p.p69 * var_delta_t_dn8), (p.p69 * var_delta_t_dn9), (p.p69 * var_delta_t_dn10), (p.p69 * var_delta_t_dn11), (p.p69 * var_delta_t_dn12), (p.p69 * var_delta_t_dn13), (p.p69 * var_delta_t_dn14), (p.p69 * var_delta_t_dn15), (p.p69 * var_delta_t_db0), (p.p69 * var_delta_t_db1), (p.p69 * var_delta_t_db2), (p.p69 * var_delta_t_db3), (p.p69 * var_delta_t_db4), (p.p69 * var_delta_t_db5), (p.p69 * var_delta_t_db6), (p.p69 * var_delta_t_db7), (p.p69 * var_delta_t_db8), (p.p69 * var_delta_t_db9), (p.p69 * var_delta_t_db10), (p.p69 * var_delta_t_db11), (p.p69 * var_delta_t_db12), (p.p69 * var_delta_t_db13), (p.p69 * var_delta_t_db14),)
    } else {
        (var_vjg_t, var_vjg_t_dn0, var_vjg_t_dn1, var_vjg_t_dn2, var_vjg_t_dn3, var_vjg_t_dn4, var_vjg_t_dn5, var_vjg_t_dn6, var_vjg_t_dn7, var_vjg_t_dn8, var_vjg_t_dn9, var_vjg_t_dn10, var_vjg_t_dn11, var_vjg_t_dn12, var_vjg_t_dn13, var_vjg_t_dn14, var_vjg_t_dn15, var_vjg_t_db0, var_vjg_t_db1, var_vjg_t_db2, var_vjg_t_db3, var_vjg_t_db4, var_vjg_t_db5, var_vjg_t_db6, var_vjg_t_db7, var_vjg_t_db8, var_vjg_t_db9, var_vjg_t_db10, var_vjg_t_db11, var_vjg_t_db12, var_vjg_t_db13, var_vjg_t_db14,)
    }
};
        var_vjg_t = assign330_e739;
        var_vjg_t_dn0 = assign330_e739_d_n0;
        var_vjg_t_dn1 = assign330_e739_d_n1;
        var_vjg_t_dn2 = assign330_e739_d_n2;
        var_vjg_t_dn3 = assign330_e739_d_n3;
        var_vjg_t_dn4 = assign330_e739_d_n4;
        var_vjg_t_dn5 = assign330_e739_d_n5;
        var_vjg_t_dn6 = assign330_e739_d_n6;
        var_vjg_t_dn7 = assign330_e739_d_n7;
        var_vjg_t_dn8 = assign330_e739_d_n8;
        var_vjg_t_dn9 = assign330_e739_d_n9;
        var_vjg_t_dn10 = assign330_e739_d_n10;
        var_vjg_t_dn11 = assign330_e739_d_n11;
        var_vjg_t_dn12 = assign330_e739_d_n12;
        var_vjg_t_dn13 = assign330_e739_d_n13;
        var_vjg_t_dn14 = assign330_e739_d_n14;
        var_vjg_t_dn15 = assign330_e739_d_n15;
        var_vjg_t_db0 = assign330_e739_d_b0;
        var_vjg_t_db1 = assign330_e739_d_b1;
        var_vjg_t_db2 = assign330_e739_d_b2;
        var_vjg_t_db3 = assign330_e739_d_b3;
        var_vjg_t_db4 = assign330_e739_d_b4;
        var_vjg_t_db5 = assign330_e739_d_b5;
        var_vjg_t_db6 = assign330_e739_d_b6;
        var_vjg_t_db7 = assign330_e739_d_b7;
        var_vjg_t_db8 = assign330_e739_d_b8;
        var_vjg_t_db9 = assign330_e739_d_b9;
        var_vjg_t_db10 = assign330_e739_d_b10;
        var_vjg_t_db11 = assign330_e739_d_b11;
        var_vjg_t_db12 = assign330_e739_d_b12;
        var_vjg_t_db13 = assign330_e739_d_b13;
        var_vjg_t_db14 = assign330_e739_d_b14;

        let (assign340_e747, assign340_e747_d_n0, assign340_e747_d_n1, assign340_e747_d_n2, assign340_e747_d_n3, assign340_e747_d_n4, assign340_e747_d_n5, assign340_e747_d_n6, assign340_e747_d_n7, assign340_e747_d_n8, assign340_e747_d_n9, assign340_e747_d_n10, assign340_e747_d_n11, assign340_e747_d_n12, assign340_e747_d_n13, assign340_e747_d_n14, assign340_e747_d_n15, assign340_e747_d_b0, assign340_e747_d_b1, assign340_e747_d_b2, assign340_e747_d_b3, assign340_e747_d_b4, assign340_e747_d_b5, assign340_e747_d_b6, assign340_e747_d_b7, assign340_e747_d_b8, assign340_e747_d_b9, assign340_e747_d_b10, assign340_e747_d_b11, assign340_e747_d_b12, assign340_e747_d_b13, assign340_e747_d_b14,) = {
    if (var_guard3 != 0.0) {
        let assign340_e744: f64 = (p.p70 * var_delta_t);
        let assign340_e745: f64 = (p.p21 + assign340_e744);
        (assign340_e745, (p.p70 * var_delta_t_dn0), (p.p70 * var_delta_t_dn1), (p.p70 * var_delta_t_dn2), (p.p70 * var_delta_t_dn3), (p.p70 * var_delta_t_dn4), (p.p70 * var_delta_t_dn5), (p.p70 * var_delta_t_dn6), (p.p70 * var_delta_t_dn7), (p.p70 * var_delta_t_dn8), (p.p70 * var_delta_t_dn9), (p.p70 * var_delta_t_dn10), (p.p70 * var_delta_t_dn11), (p.p70 * var_delta_t_dn12), (p.p70 * var_delta_t_dn13), (p.p70 * var_delta_t_dn14), (p.p70 * var_delta_t_dn15), (p.p70 * var_delta_t_db0), (p.p70 * var_delta_t_db1), (p.p70 * var_delta_t_db2), (p.p70 * var_delta_t_db3), (p.p70 * var_delta_t_db4), (p.p70 * var_delta_t_db5), (p.p70 * var_delta_t_db6), (p.p70 * var_delta_t_db7), (p.p70 * var_delta_t_db8), (p.p70 * var_delta_t_db9), (p.p70 * var_delta_t_db10), (p.p70 * var_delta_t_db11), (p.p70 * var_delta_t_db12), (p.p70 * var_delta_t_db13), (p.p70 * var_delta_t_db14),)
    } else {
        (var_vtr_t, var_vtr_t_dn0, var_vtr_t_dn1, var_vtr_t_dn2, var_vtr_t_dn3, var_vtr_t_dn4, var_vtr_t_dn5, var_vtr_t_dn6, var_vtr_t_dn7, var_vtr_t_dn8, var_vtr_t_dn9, var_vtr_t_dn10, var_vtr_t_dn11, var_vtr_t_dn12, var_vtr_t_dn13, var_vtr_t_dn14, var_vtr_t_dn15, var_vtr_t_db0, var_vtr_t_db1, var_vtr_t_db2, var_vtr_t_db3, var_vtr_t_db4, var_vtr_t_db5, var_vtr_t_db6, var_vtr_t_db7, var_vtr_t_db8, var_vtr_t_db9, var_vtr_t_db10, var_vtr_t_db11, var_vtr_t_db12, var_vtr_t_db13, var_vtr_t_db14,)
    }
};
        var_vtr_t = assign340_e747;
        var_vtr_t_dn0 = assign340_e747_d_n0;
        var_vtr_t_dn1 = assign340_e747_d_n1;
        var_vtr_t_dn2 = assign340_e747_d_n2;
        var_vtr_t_dn3 = assign340_e747_d_n3;
        var_vtr_t_dn4 = assign340_e747_d_n4;
        var_vtr_t_dn5 = assign340_e747_d_n5;
        var_vtr_t_dn6 = assign340_e747_d_n6;
        var_vtr_t_dn7 = assign340_e747_d_n7;
        var_vtr_t_dn8 = assign340_e747_d_n8;
        var_vtr_t_dn9 = assign340_e747_d_n9;
        var_vtr_t_dn10 = assign340_e747_d_n10;
        var_vtr_t_dn11 = assign340_e747_d_n11;
        var_vtr_t_dn12 = assign340_e747_d_n12;
        var_vtr_t_dn13 = assign340_e747_d_n13;
        var_vtr_t_dn14 = assign340_e747_d_n14;
        var_vtr_t_dn15 = assign340_e747_d_n15;
        var_vtr_t_db0 = assign340_e747_d_b0;
        var_vtr_t_db1 = assign340_e747_d_b1;
        var_vtr_t_db2 = assign340_e747_d_b2;
        var_vtr_t_db3 = assign340_e747_d_b3;
        var_vtr_t_db4 = assign340_e747_d_b4;
        var_vtr_t_db5 = assign340_e747_d_b5;
        var_vtr_t_db6 = assign340_e747_d_b6;
        var_vtr_t_db7 = assign340_e747_d_b7;
        var_vtr_t_db8 = assign340_e747_d_b8;
        var_vtr_t_db9 = assign340_e747_d_b9;
        var_vtr_t_db10 = assign340_e747_d_b10;
        var_vtr_t_db11 = assign340_e747_d_b11;
        var_vtr_t_db12 = assign340_e747_d_b12;
        var_vtr_t_db13 = assign340_e747_d_b13;
        var_vtr_t_db14 = assign340_e747_d_b14;

        let (assign360_e757, assign360_e757_d_n0, assign360_e757_d_n1, assign360_e757_d_n2, assign360_e757_d_n3, assign360_e757_d_n4, assign360_e757_d_n5, assign360_e757_d_n6, assign360_e757_d_n7, assign360_e757_d_n8, assign360_e757_d_n9, assign360_e757_d_n10, assign360_e757_d_n11, assign360_e757_d_n12, assign360_e757_d_n13, assign360_e757_d_n14, assign360_e757_d_n15, assign360_e757_d_b0, assign360_e757_d_b1, assign360_e757_d_b2, assign360_e757_d_b3, assign360_e757_d_b4, assign360_e757_d_b5, assign360_e757_d_b6, assign360_e757_d_b7, assign360_e757_d_b8, assign360_e757_d_b9, assign360_e757_d_b10, assign360_e757_d_b11, assign360_e757_d_b12, assign360_e757_d_b13, assign360_e757_d_b14,) = {
    if (var_guard3 == 0.0) {
        (p.p11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_p1_t, var_p1_t_dn0, var_p1_t_dn1, var_p1_t_dn2, var_p1_t_dn3, var_p1_t_dn4, var_p1_t_dn5, var_p1_t_dn6, var_p1_t_dn7, var_p1_t_dn8, var_p1_t_dn9, var_p1_t_dn10, var_p1_t_dn11, var_p1_t_dn12, var_p1_t_dn13, var_p1_t_dn14, var_p1_t_dn15, var_p1_t_db0, var_p1_t_db1, var_p1_t_db2, var_p1_t_db3, var_p1_t_db4, var_p1_t_db5, var_p1_t_db6, var_p1_t_db7, var_p1_t_db8, var_p1_t_db9, var_p1_t_db10, var_p1_t_db11, var_p1_t_db12, var_p1_t_db13, var_p1_t_db14,)
    }
};
        var_p1_t = assign360_e757;
        var_p1_t_dn0 = assign360_e757_d_n0;
        var_p1_t_dn1 = assign360_e757_d_n1;
        var_p1_t_dn2 = assign360_e757_d_n2;
        var_p1_t_dn3 = assign360_e757_d_n3;
        var_p1_t_dn4 = assign360_e757_d_n4;
        var_p1_t_dn5 = assign360_e757_d_n5;
        var_p1_t_dn6 = assign360_e757_d_n6;
        var_p1_t_dn7 = assign360_e757_d_n7;
        var_p1_t_dn8 = assign360_e757_d_n8;
        var_p1_t_dn9 = assign360_e757_d_n9;
        var_p1_t_dn10 = assign360_e757_d_n10;
        var_p1_t_dn11 = assign360_e757_d_n11;
        var_p1_t_dn12 = assign360_e757_d_n12;
        var_p1_t_dn13 = assign360_e757_d_n13;
        var_p1_t_dn14 = assign360_e757_d_n14;
        var_p1_t_dn15 = assign360_e757_d_n15;
        var_p1_t_db0 = assign360_e757_d_b0;
        var_p1_t_db1 = assign360_e757_d_b1;
        var_p1_t_db2 = assign360_e757_d_b2;
        var_p1_t_db3 = assign360_e757_d_b3;
        var_p1_t_db4 = assign360_e757_d_b4;
        var_p1_t_db5 = assign360_e757_d_b5;
        var_p1_t_db6 = assign360_e757_d_b6;
        var_p1_t_db7 = assign360_e757_d_b7;
        var_p1_t_db8 = assign360_e757_d_b8;
        var_p1_t_db9 = assign360_e757_d_b9;
        var_p1_t_db10 = assign360_e757_d_b10;
        var_p1_t_db11 = assign360_e757_d_b11;
        var_p1_t_db12 = assign360_e757_d_b12;
        var_p1_t_db13 = assign360_e757_d_b13;
        var_p1_t_db14 = assign360_e757_d_b14;

        let (assign380_e767, assign380_e767_d_n0, assign380_e767_d_n1, assign380_e767_d_n2, assign380_e767_d_n3, assign380_e767_d_n4, assign380_e767_d_n5, assign380_e767_d_n6, assign380_e767_d_n7, assign380_e767_d_n8, assign380_e767_d_n9, assign380_e767_d_n10, assign380_e767_d_n11, assign380_e767_d_n12, assign380_e767_d_n13, assign380_e767_d_n14, assign380_e767_d_n15, assign380_e767_d_b0, assign380_e767_d_b1, assign380_e767_d_b2, assign380_e767_d_b3, assign380_e767_d_b4, assign380_e767_d_b5, assign380_e767_d_b6, assign380_e767_d_b7, assign380_e767_d_b8, assign380_e767_d_b9, assign380_e767_d_b10, assign380_e767_d_b11, assign380_e767_d_b12, assign380_e767_d_b13, assign380_e767_d_b14,) = {
    if (var_guard3 == 0.0) {
        (p.p25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgs0_t, var_cgs0_t_dn0, var_cgs0_t_dn1, var_cgs0_t_dn2, var_cgs0_t_dn3, var_cgs0_t_dn4, var_cgs0_t_dn5, var_cgs0_t_dn6, var_cgs0_t_dn7, var_cgs0_t_dn8, var_cgs0_t_dn9, var_cgs0_t_dn10, var_cgs0_t_dn11, var_cgs0_t_dn12, var_cgs0_t_dn13, var_cgs0_t_dn14, var_cgs0_t_dn15, var_cgs0_t_db0, var_cgs0_t_db1, var_cgs0_t_db2, var_cgs0_t_db3, var_cgs0_t_db4, var_cgs0_t_db5, var_cgs0_t_db6, var_cgs0_t_db7, var_cgs0_t_db8, var_cgs0_t_db9, var_cgs0_t_db10, var_cgs0_t_db11, var_cgs0_t_db12, var_cgs0_t_db13, var_cgs0_t_db14,)
    }
};
        var_cgs0_t = assign380_e767;
        var_cgs0_t_dn0 = assign380_e767_d_n0;
        var_cgs0_t_dn1 = assign380_e767_d_n1;
        var_cgs0_t_dn2 = assign380_e767_d_n2;
        var_cgs0_t_dn3 = assign380_e767_d_n3;
        var_cgs0_t_dn4 = assign380_e767_d_n4;
        var_cgs0_t_dn5 = assign380_e767_d_n5;
        var_cgs0_t_dn6 = assign380_e767_d_n6;
        var_cgs0_t_dn7 = assign380_e767_d_n7;
        var_cgs0_t_dn8 = assign380_e767_d_n8;
        var_cgs0_t_dn9 = assign380_e767_d_n9;
        var_cgs0_t_dn10 = assign380_e767_d_n10;
        var_cgs0_t_dn11 = assign380_e767_d_n11;
        var_cgs0_t_dn12 = assign380_e767_d_n12;
        var_cgs0_t_dn13 = assign380_e767_d_n13;
        var_cgs0_t_dn14 = assign380_e767_d_n14;
        var_cgs0_t_dn15 = assign380_e767_d_n15;
        var_cgs0_t_db0 = assign380_e767_d_b0;
        var_cgs0_t_db1 = assign380_e767_d_b1;
        var_cgs0_t_db2 = assign380_e767_d_b2;
        var_cgs0_t_db3 = assign380_e767_d_b3;
        var_cgs0_t_db4 = assign380_e767_d_b4;
        var_cgs0_t_db5 = assign380_e767_d_b5;
        var_cgs0_t_db6 = assign380_e767_d_b6;
        var_cgs0_t_db7 = assign380_e767_d_b7;
        var_cgs0_t_db8 = assign380_e767_d_b8;
        var_cgs0_t_db9 = assign380_e767_d_b9;
        var_cgs0_t_db10 = assign380_e767_d_b10;
        var_cgs0_t_db11 = assign380_e767_d_b11;
        var_cgs0_t_db12 = assign380_e767_d_b12;
        var_cgs0_t_db13 = assign380_e767_d_b13;
        var_cgs0_t_db14 = assign380_e767_d_b14;


        *var_cgd0_t_slot = var_cgd0_t;
        *var_cgd0_t_db0_slot = var_cgd0_t_db0;
        *var_cgd0_t_db1_slot = var_cgd0_t_db1;
        *var_cgd0_t_db10_slot = var_cgd0_t_db10;
        *var_cgd0_t_db11_slot = var_cgd0_t_db11;
        *var_cgd0_t_db12_slot = var_cgd0_t_db12;
        *var_cgd0_t_db13_slot = var_cgd0_t_db13;
        *var_cgd0_t_db14_slot = var_cgd0_t_db14;
        *var_cgd0_t_db2_slot = var_cgd0_t_db2;
        *var_cgd0_t_db3_slot = var_cgd0_t_db3;
        *var_cgd0_t_db4_slot = var_cgd0_t_db4;
        *var_cgd0_t_db5_slot = var_cgd0_t_db5;
        *var_cgd0_t_db6_slot = var_cgd0_t_db6;
        *var_cgd0_t_db7_slot = var_cgd0_t_db7;
        *var_cgd0_t_db8_slot = var_cgd0_t_db8;
        *var_cgd0_t_db9_slot = var_cgd0_t_db9;
        *var_cgd0_t_dn0_slot = var_cgd0_t_dn0;
        *var_cgd0_t_dn1_slot = var_cgd0_t_dn1;
        *var_cgd0_t_dn10_slot = var_cgd0_t_dn10;
        *var_cgd0_t_dn11_slot = var_cgd0_t_dn11;
        *var_cgd0_t_dn12_slot = var_cgd0_t_dn12;
        *var_cgd0_t_dn13_slot = var_cgd0_t_dn13;
        *var_cgd0_t_dn14_slot = var_cgd0_t_dn14;
        *var_cgd0_t_dn15_slot = var_cgd0_t_dn15;
        *var_cgd0_t_dn2_slot = var_cgd0_t_dn2;
        *var_cgd0_t_dn3_slot = var_cgd0_t_dn3;
        *var_cgd0_t_dn4_slot = var_cgd0_t_dn4;
        *var_cgd0_t_dn5_slot = var_cgd0_t_dn5;
        *var_cgd0_t_dn6_slot = var_cgd0_t_dn6;
        *var_cgd0_t_dn7_slot = var_cgd0_t_dn7;
        *var_cgd0_t_dn8_slot = var_cgd0_t_dn8;
        *var_cgd0_t_dn9_slot = var_cgd0_t_dn9;
        *var_cgs0_t_slot = var_cgs0_t;
        *var_cgs0_t_db0_slot = var_cgs0_t_db0;
        *var_cgs0_t_db1_slot = var_cgs0_t_db1;
        *var_cgs0_t_db10_slot = var_cgs0_t_db10;
        *var_cgs0_t_db11_slot = var_cgs0_t_db11;
        *var_cgs0_t_db12_slot = var_cgs0_t_db12;
        *var_cgs0_t_db13_slot = var_cgs0_t_db13;
        *var_cgs0_t_db14_slot = var_cgs0_t_db14;
        *var_cgs0_t_db2_slot = var_cgs0_t_db2;
        *var_cgs0_t_db3_slot = var_cgs0_t_db3;
        *var_cgs0_t_db4_slot = var_cgs0_t_db4;
        *var_cgs0_t_db5_slot = var_cgs0_t_db5;
        *var_cgs0_t_db6_slot = var_cgs0_t_db6;
        *var_cgs0_t_db7_slot = var_cgs0_t_db7;
        *var_cgs0_t_db8_slot = var_cgs0_t_db8;
        *var_cgs0_t_db9_slot = var_cgs0_t_db9;
        *var_cgs0_t_dn0_slot = var_cgs0_t_dn0;
        *var_cgs0_t_dn1_slot = var_cgs0_t_dn1;
        *var_cgs0_t_dn10_slot = var_cgs0_t_dn10;
        *var_cgs0_t_dn11_slot = var_cgs0_t_dn11;
        *var_cgs0_t_dn12_slot = var_cgs0_t_dn12;
        *var_cgs0_t_dn13_slot = var_cgs0_t_dn13;
        *var_cgs0_t_dn14_slot = var_cgs0_t_dn14;
        *var_cgs0_t_dn15_slot = var_cgs0_t_dn15;
        *var_cgs0_t_dn2_slot = var_cgs0_t_dn2;
        *var_cgs0_t_dn3_slot = var_cgs0_t_dn3;
        *var_cgs0_t_dn4_slot = var_cgs0_t_dn4;
        *var_cgs0_t_dn5_slot = var_cgs0_t_dn5;
        *var_cgs0_t_dn6_slot = var_cgs0_t_dn6;
        *var_cgs0_t_dn7_slot = var_cgs0_t_dn7;
        *var_cgs0_t_dn8_slot = var_cgs0_t_dn8;
        *var_cgs0_t_dn9_slot = var_cgs0_t_dn9;
        *var_delta_t_slot = var_delta_t;
        *var_delta_t_db0_slot = var_delta_t_db0;
        *var_delta_t_db1_slot = var_delta_t_db1;
        *var_delta_t_db10_slot = var_delta_t_db10;
        *var_delta_t_db11_slot = var_delta_t_db11;
        *var_delta_t_db12_slot = var_delta_t_db12;
        *var_delta_t_db13_slot = var_delta_t_db13;
        *var_delta_t_db14_slot = var_delta_t_db14;
        *var_delta_t_db2_slot = var_delta_t_db2;
        *var_delta_t_db3_slot = var_delta_t_db3;
        *var_delta_t_db4_slot = var_delta_t_db4;
        *var_delta_t_db5_slot = var_delta_t_db5;
        *var_delta_t_db6_slot = var_delta_t_db6;
        *var_delta_t_db7_slot = var_delta_t_db7;
        *var_delta_t_db8_slot = var_delta_t_db8;
        *var_delta_t_db9_slot = var_delta_t_db9;
        *var_delta_t_dn0_slot = var_delta_t_dn0;
        *var_delta_t_dn1_slot = var_delta_t_dn1;
        *var_delta_t_dn10_slot = var_delta_t_dn10;
        *var_delta_t_dn11_slot = var_delta_t_dn11;
        *var_delta_t_dn12_slot = var_delta_t_dn12;
        *var_delta_t_dn13_slot = var_delta_t_dn13;
        *var_delta_t_dn14_slot = var_delta_t_dn14;
        *var_delta_t_dn15_slot = var_delta_t_dn15;
        *var_delta_t_dn2_slot = var_delta_t_dn2;
        *var_delta_t_dn3_slot = var_delta_t_dn3;
        *var_delta_t_dn4_slot = var_delta_t_dn4;
        *var_delta_t_dn5_slot = var_delta_t_dn5;
        *var_delta_t_dn6_slot = var_delta_t_dn6;
        *var_delta_t_dn7_slot = var_delta_t_dn7;
        *var_delta_t_dn8_slot = var_delta_t_dn8;
        *var_delta_t_dn9_slot = var_delta_t_dn9;
        *var_guard3_slot = var_guard3;
        *var_p10_t_slot = var_p10_t;
        *var_p10_t_db0_slot = var_p10_t_db0;
        *var_p10_t_db1_slot = var_p10_t_db1;
        *var_p10_t_db10_slot = var_p10_t_db10;
        *var_p10_t_db11_slot = var_p10_t_db11;
        *var_p10_t_db12_slot = var_p10_t_db12;
        *var_p10_t_db13_slot = var_p10_t_db13;
        *var_p10_t_db14_slot = var_p10_t_db14;
        *var_p10_t_db2_slot = var_p10_t_db2;
        *var_p10_t_db3_slot = var_p10_t_db3;
        *var_p10_t_db4_slot = var_p10_t_db4;
        *var_p10_t_db5_slot = var_p10_t_db5;
        *var_p10_t_db6_slot = var_p10_t_db6;
        *var_p10_t_db7_slot = var_p10_t_db7;
        *var_p10_t_db8_slot = var_p10_t_db8;
        *var_p10_t_db9_slot = var_p10_t_db9;
        *var_p10_t_dn0_slot = var_p10_t_dn0;
        *var_p10_t_dn1_slot = var_p10_t_dn1;
        *var_p10_t_dn10_slot = var_p10_t_dn10;
        *var_p10_t_dn11_slot = var_p10_t_dn11;
        *var_p10_t_dn12_slot = var_p10_t_dn12;
        *var_p10_t_dn13_slot = var_p10_t_dn13;
        *var_p10_t_dn14_slot = var_p10_t_dn14;
        *var_p10_t_dn15_slot = var_p10_t_dn15;
        *var_p10_t_dn2_slot = var_p10_t_dn2;
        *var_p10_t_dn3_slot = var_p10_t_dn3;
        *var_p10_t_dn4_slot = var_p10_t_dn4;
        *var_p10_t_dn5_slot = var_p10_t_dn5;
        *var_p10_t_dn6_slot = var_p10_t_dn6;
        *var_p10_t_dn7_slot = var_p10_t_dn7;
        *var_p10_t_dn8_slot = var_p10_t_dn8;
        *var_p10_t_dn9_slot = var_p10_t_dn9;
        *var_p1_t_slot = var_p1_t;
        *var_p1_t_db0_slot = var_p1_t_db0;
        *var_p1_t_db1_slot = var_p1_t_db1;
        *var_p1_t_db10_slot = var_p1_t_db10;
        *var_p1_t_db11_slot = var_p1_t_db11;
        *var_p1_t_db12_slot = var_p1_t_db12;
        *var_p1_t_db13_slot = var_p1_t_db13;
        *var_p1_t_db14_slot = var_p1_t_db14;
        *var_p1_t_db2_slot = var_p1_t_db2;
        *var_p1_t_db3_slot = var_p1_t_db3;
        *var_p1_t_db4_slot = var_p1_t_db4;
        *var_p1_t_db5_slot = var_p1_t_db5;
        *var_p1_t_db6_slot = var_p1_t_db6;
        *var_p1_t_db7_slot = var_p1_t_db7;
        *var_p1_t_db8_slot = var_p1_t_db8;
        *var_p1_t_db9_slot = var_p1_t_db9;
        *var_p1_t_dn0_slot = var_p1_t_dn0;
        *var_p1_t_dn1_slot = var_p1_t_dn1;
        *var_p1_t_dn10_slot = var_p1_t_dn10;
        *var_p1_t_dn11_slot = var_p1_t_dn11;
        *var_p1_t_dn12_slot = var_p1_t_dn12;
        *var_p1_t_dn13_slot = var_p1_t_dn13;
        *var_p1_t_dn14_slot = var_p1_t_dn14;
        *var_p1_t_dn15_slot = var_p1_t_dn15;
        *var_p1_t_dn2_slot = var_p1_t_dn2;
        *var_p1_t_dn3_slot = var_p1_t_dn3;
        *var_p1_t_dn4_slot = var_p1_t_dn4;
        *var_p1_t_dn5_slot = var_p1_t_dn5;
        *var_p1_t_dn6_slot = var_p1_t_dn6;
        *var_p1_t_dn7_slot = var_p1_t_dn7;
        *var_p1_t_dn8_slot = var_p1_t_dn8;
        *var_p1_t_dn9_slot = var_p1_t_dn9;
        *var_p40_t_slot = var_p40_t;
        *var_p40_t_db0_slot = var_p40_t_db0;
        *var_p40_t_db1_slot = var_p40_t_db1;
        *var_p40_t_db10_slot = var_p40_t_db10;
        *var_p40_t_db11_slot = var_p40_t_db11;
        *var_p40_t_db12_slot = var_p40_t_db12;
        *var_p40_t_db13_slot = var_p40_t_db13;
        *var_p40_t_db14_slot = var_p40_t_db14;
        *var_p40_t_db2_slot = var_p40_t_db2;
        *var_p40_t_db3_slot = var_p40_t_db3;
        *var_p40_t_db4_slot = var_p40_t_db4;
        *var_p40_t_db5_slot = var_p40_t_db5;
        *var_p40_t_db6_slot = var_p40_t_db6;
        *var_p40_t_db7_slot = var_p40_t_db7;
        *var_p40_t_db8_slot = var_p40_t_db8;
        *var_p40_t_db9_slot = var_p40_t_db9;
        *var_p40_t_dn0_slot = var_p40_t_dn0;
        *var_p40_t_dn1_slot = var_p40_t_dn1;
        *var_p40_t_dn10_slot = var_p40_t_dn10;
        *var_p40_t_dn11_slot = var_p40_t_dn11;
        *var_p40_t_dn12_slot = var_p40_t_dn12;
        *var_p40_t_dn13_slot = var_p40_t_dn13;
        *var_p40_t_dn14_slot = var_p40_t_dn14;
        *var_p40_t_dn15_slot = var_p40_t_dn15;
        *var_p40_t_dn2_slot = var_p40_t_dn2;
        *var_p40_t_dn3_slot = var_p40_t_dn3;
        *var_p40_t_dn4_slot = var_p40_t_dn4;
        *var_p40_t_dn5_slot = var_p40_t_dn5;
        *var_p40_t_dn6_slot = var_p40_t_dn6;
        *var_p40_t_dn7_slot = var_p40_t_dn7;
        *var_p40_t_dn8_slot = var_p40_t_dn8;
        *var_p40_t_dn9_slot = var_p40_t_dn9;
        *var_vjg_t_slot = var_vjg_t;
        *var_vjg_t_db0_slot = var_vjg_t_db0;
        *var_vjg_t_db1_slot = var_vjg_t_db1;
        *var_vjg_t_db10_slot = var_vjg_t_db10;
        *var_vjg_t_db11_slot = var_vjg_t_db11;
        *var_vjg_t_db12_slot = var_vjg_t_db12;
        *var_vjg_t_db13_slot = var_vjg_t_db13;
        *var_vjg_t_db14_slot = var_vjg_t_db14;
        *var_vjg_t_db2_slot = var_vjg_t_db2;
        *var_vjg_t_db3_slot = var_vjg_t_db3;
        *var_vjg_t_db4_slot = var_vjg_t_db4;
        *var_vjg_t_db5_slot = var_vjg_t_db5;
        *var_vjg_t_db6_slot = var_vjg_t_db6;
        *var_vjg_t_db7_slot = var_vjg_t_db7;
        *var_vjg_t_db8_slot = var_vjg_t_db8;
        *var_vjg_t_db9_slot = var_vjg_t_db9;
        *var_vjg_t_dn0_slot = var_vjg_t_dn0;
        *var_vjg_t_dn1_slot = var_vjg_t_dn1;
        *var_vjg_t_dn10_slot = var_vjg_t_dn10;
        *var_vjg_t_dn11_slot = var_vjg_t_dn11;
        *var_vjg_t_dn12_slot = var_vjg_t_dn12;
        *var_vjg_t_dn13_slot = var_vjg_t_dn13;
        *var_vjg_t_dn14_slot = var_vjg_t_dn14;
        *var_vjg_t_dn15_slot = var_vjg_t_dn15;
        *var_vjg_t_dn2_slot = var_vjg_t_dn2;
        *var_vjg_t_dn3_slot = var_vjg_t_dn3;
        *var_vjg_t_dn4_slot = var_vjg_t_dn4;
        *var_vjg_t_dn5_slot = var_vjg_t_dn5;
        *var_vjg_t_dn6_slot = var_vjg_t_dn6;
        *var_vjg_t_dn7_slot = var_vjg_t_dn7;
        *var_vjg_t_dn8_slot = var_vjg_t_dn8;
        *var_vjg_t_dn9_slot = var_vjg_t_dn9;
        *var_vpks_t_slot = var_vpks_t;
        *var_vpks_t_db0_slot = var_vpks_t_db0;
        *var_vpks_t_db1_slot = var_vpks_t_db1;
        *var_vpks_t_db10_slot = var_vpks_t_db10;
        *var_vpks_t_db11_slot = var_vpks_t_db11;
        *var_vpks_t_db12_slot = var_vpks_t_db12;
        *var_vpks_t_db13_slot = var_vpks_t_db13;
        *var_vpks_t_db14_slot = var_vpks_t_db14;
        *var_vpks_t_db2_slot = var_vpks_t_db2;
        *var_vpks_t_db3_slot = var_vpks_t_db3;
        *var_vpks_t_db4_slot = var_vpks_t_db4;
        *var_vpks_t_db5_slot = var_vpks_t_db5;
        *var_vpks_t_db6_slot = var_vpks_t_db6;
        *var_vpks_t_db7_slot = var_vpks_t_db7;
        *var_vpks_t_db8_slot = var_vpks_t_db8;
        *var_vpks_t_db9_slot = var_vpks_t_db9;
        *var_vpks_t_dn0_slot = var_vpks_t_dn0;
        *var_vpks_t_dn1_slot = var_vpks_t_dn1;
        *var_vpks_t_dn10_slot = var_vpks_t_dn10;
        *var_vpks_t_dn11_slot = var_vpks_t_dn11;
        *var_vpks_t_dn12_slot = var_vpks_t_dn12;
        *var_vpks_t_dn13_slot = var_vpks_t_dn13;
        *var_vpks_t_dn14_slot = var_vpks_t_dn14;
        *var_vpks_t_dn15_slot = var_vpks_t_dn15;
        *var_vpks_t_dn2_slot = var_vpks_t_dn2;
        *var_vpks_t_dn3_slot = var_vpks_t_dn3;
        *var_vpks_t_dn4_slot = var_vpks_t_dn4;
        *var_vpks_t_dn5_slot = var_vpks_t_dn5;
        *var_vpks_t_dn6_slot = var_vpks_t_dn6;
        *var_vpks_t_dn7_slot = var_vpks_t_dn7;
        *var_vpks_t_dn8_slot = var_vpks_t_dn8;
        *var_vpks_t_dn9_slot = var_vpks_t_dn9;
        *var_vth_slot = var_vth;
        *var_vth_db0_slot = var_vth_db0;
        *var_vth_db1_slot = var_vth_db1;
        *var_vth_db10_slot = var_vth_db10;
        *var_vth_db11_slot = var_vth_db11;
        *var_vth_db12_slot = var_vth_db12;
        *var_vth_db13_slot = var_vth_db13;
        *var_vth_db14_slot = var_vth_db14;
        *var_vth_db2_slot = var_vth_db2;
        *var_vth_db3_slot = var_vth_db3;
        *var_vth_db4_slot = var_vth_db4;
        *var_vth_db5_slot = var_vth_db5;
        *var_vth_db6_slot = var_vth_db6;
        *var_vth_db7_slot = var_vth_db7;
        *var_vth_db8_slot = var_vth_db8;
        *var_vth_db9_slot = var_vth_db9;
        *var_vth_dn0_slot = var_vth_dn0;
        *var_vth_dn1_slot = var_vth_dn1;
        *var_vth_dn10_slot = var_vth_dn10;
        *var_vth_dn11_slot = var_vth_dn11;
        *var_vth_dn12_slot = var_vth_dn12;
        *var_vth_dn13_slot = var_vth_dn13;
        *var_vth_dn14_slot = var_vth_dn14;
        *var_vth_dn15_slot = var_vth_dn15;
        *var_vth_dn2_slot = var_vth_dn2;
        *var_vth_dn3_slot = var_vth_dn3;
        *var_vth_dn4_slot = var_vth_dn4;
        *var_vth_dn5_slot = var_vth_dn5;
        *var_vth_dn6_slot = var_vth_dn6;
        *var_vth_dn7_slot = var_vth_dn7;
        *var_vth_dn8_slot = var_vth_dn8;
        *var_vth_dn9_slot = var_vth_dn9;
        *var_vtr_t_slot = var_vtr_t;
        *var_vtr_t_db0_slot = var_vtr_t_db0;
        *var_vtr_t_db1_slot = var_vtr_t_db1;
        *var_vtr_t_db10_slot = var_vtr_t_db10;
        *var_vtr_t_db11_slot = var_vtr_t_db11;
        *var_vtr_t_db12_slot = var_vtr_t_db12;
        *var_vtr_t_db13_slot = var_vtr_t_db13;
        *var_vtr_t_db14_slot = var_vtr_t_db14;
        *var_vtr_t_db2_slot = var_vtr_t_db2;
        *var_vtr_t_db3_slot = var_vtr_t_db3;
        *var_vtr_t_db4_slot = var_vtr_t_db4;
        *var_vtr_t_db5_slot = var_vtr_t_db5;
        *var_vtr_t_db6_slot = var_vtr_t_db6;
        *var_vtr_t_db7_slot = var_vtr_t_db7;
        *var_vtr_t_db8_slot = var_vtr_t_db8;
        *var_vtr_t_db9_slot = var_vtr_t_db9;
        *var_vtr_t_dn0_slot = var_vtr_t_dn0;
        *var_vtr_t_dn1_slot = var_vtr_t_dn1;
        *var_vtr_t_dn10_slot = var_vtr_t_dn10;
        *var_vtr_t_dn11_slot = var_vtr_t_dn11;
        *var_vtr_t_dn12_slot = var_vtr_t_dn12;
        *var_vtr_t_dn13_slot = var_vtr_t_dn13;
        *var_vtr_t_dn14_slot = var_vtr_t_dn14;
        *var_vtr_t_dn15_slot = var_vtr_t_dn15;
        *var_vtr_t_dn2_slot = var_vtr_t_dn2;
        *var_vtr_t_dn3_slot = var_vtr_t_dn3;
        *var_vtr_t_dn4_slot = var_vtr_t_dn4;
        *var_vtr_t_dn5_slot = var_vtr_t_dn5;
        *var_vtr_t_dn6_slot = var_vtr_t_dn6;
        *var_vtr_t_dn7_slot = var_vtr_t_dn7;
        *var_vtr_t_dn8_slot = var_vtr_t_dn8;
        *var_vtr_t_dn9_slot = var_vtr_t_dn9;
    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard3: f64,
        var_p1_t: f64,
        var_p1_t_db0: f64,
        var_p1_t_db1: f64,
        var_p1_t_db10: f64,
        var_p1_t_db11: f64,
        var_p1_t_db12: f64,
        var_p1_t_db13: f64,
        var_p1_t_db14: f64,
        var_p1_t_db2: f64,
        var_p1_t_db3: f64,
        var_p1_t_db4: f64,
        var_p1_t_db5: f64,
        var_p1_t_db6: f64,
        var_p1_t_db7: f64,
        var_p1_t_db8: f64,
        var_p1_t_db9: f64,
        var_p1_t_dn0: f64,
        var_p1_t_dn1: f64,
        var_p1_t_dn10: f64,
        var_p1_t_dn11: f64,
        var_p1_t_dn12: f64,
        var_p1_t_dn13: f64,
        var_p1_t_dn14: f64,
        var_p1_t_dn15: f64,
        var_p1_t_dn2: f64,
        var_p1_t_dn3: f64,
        var_p1_t_dn4: f64,
        var_p1_t_dn5: f64,
        var_p1_t_dn6: f64,
        var_p1_t_dn7: f64,
        var_p1_t_dn8: f64,
        var_p1_t_dn9: f64,
        var_vdg: f64,
        var_vdg_db0: f64,
        var_vdg_db1: f64,
        var_vdg_db10: f64,
        var_vdg_db11: f64,
        var_vdg_db12: f64,
        var_vdg_db13: f64,
        var_vdg_db14: f64,
        var_vdg_db2: f64,
        var_vdg_db3: f64,
        var_vdg_db4: f64,
        var_vdg_db5: f64,
        var_vdg_db6: f64,
        var_vdg_db7: f64,
        var_vdg_db8: f64,
        var_vdg_db9: f64,
        var_vdg_dn0: f64,
        var_vdg_dn1: f64,
        var_vdg_dn10: f64,
        var_vdg_dn11: f64,
        var_vdg_dn12: f64,
        var_vdg_dn13: f64,
        var_vdg_dn14: f64,
        var_vdg_dn15: f64,
        var_vdg_dn2: f64,
        var_vdg_dn3: f64,
        var_vdg_dn4: f64,
        var_vdg_dn5: f64,
        var_vdg_dn6: f64,
        var_vdg_dn7: f64,
        var_vdg_dn8: f64,
        var_vdg_dn9: f64,
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
        var_vgs: f64,
        var_vgs_db0: f64,
        var_vgs_db1: f64,
        var_vgs_db10: f64,
        var_vgs_db11: f64,
        var_vgs_db12: f64,
        var_vgs_db13: f64,
        var_vgs_db14: f64,
        var_vgs_db2: f64,
        var_vgs_db3: f64,
        var_vgs_db4: f64,
        var_vgs_db5: f64,
        var_vgs_db6: f64,
        var_vgs_db7: f64,
        var_vgs_db8: f64,
        var_vgs_db9: f64,
        var_vgs_dn0: f64,
        var_vgs_dn1: f64,
        var_vgs_dn10: f64,
        var_vgs_dn11: f64,
        var_vgs_dn12: f64,
        var_vgs_dn13: f64,
        var_vgs_dn14: f64,
        var_vgs_dn15: f64,
        var_vgs_dn2: f64,
        var_vgs_dn3: f64,
        var_vgs_dn4: f64,
        var_vgs_dn5: f64,
        var_vgs_dn6: f64,
        var_vgs_dn7: f64,
        var_vgs_dn8: f64,
        var_vgs_dn9: f64,
        var_vth: f64,
        var_vth_db0: f64,
        var_vth_db1: f64,
        var_vth_db10: f64,
        var_vth_db11: f64,
        var_vth_db12: f64,
        var_vth_db13: f64,
        var_vth_db14: f64,
        var_vth_db2: f64,
        var_vth_db3: f64,
        var_vth_db4: f64,
        var_vth_db5: f64,
        var_vth_db6: f64,
        var_vth_db7: f64,
        var_vth_db8: f64,
        var_vth_db9: f64,
        var_vth_dn0: f64,
        var_vth_dn1: f64,
        var_vth_dn10: f64,
        var_vth_dn11: f64,
        var_vth_dn12: f64,
        var_vth_dn13: f64,
        var_vth_dn14: f64,
        var_vth_dn15: f64,
        var_vth_dn2: f64,
        var_vth_dn3: f64,
        var_vth_dn4: f64,
        var_vth_dn5: f64,
        var_vth_dn6: f64,
        var_vth_dn7: f64,
        var_vth_dn8: f64,
        var_vth_dn9: f64,
        var_cgd0_t_slot: &mut f64,
        var_cgd0_t_db0_slot: &mut f64,
        var_cgd0_t_db1_slot: &mut f64,
        var_cgd0_t_db10_slot: &mut f64,
        var_cgd0_t_db11_slot: &mut f64,
        var_cgd0_t_db12_slot: &mut f64,
        var_cgd0_t_db13_slot: &mut f64,
        var_cgd0_t_db14_slot: &mut f64,
        var_cgd0_t_db2_slot: &mut f64,
        var_cgd0_t_db3_slot: &mut f64,
        var_cgd0_t_db4_slot: &mut f64,
        var_cgd0_t_db5_slot: &mut f64,
        var_cgd0_t_db6_slot: &mut f64,
        var_cgd0_t_db7_slot: &mut f64,
        var_cgd0_t_db8_slot: &mut f64,
        var_cgd0_t_db9_slot: &mut f64,
        var_cgd0_t_dn0_slot: &mut f64,
        var_cgd0_t_dn1_slot: &mut f64,
        var_cgd0_t_dn10_slot: &mut f64,
        var_cgd0_t_dn11_slot: &mut f64,
        var_cgd0_t_dn12_slot: &mut f64,
        var_cgd0_t_dn13_slot: &mut f64,
        var_cgd0_t_dn14_slot: &mut f64,
        var_cgd0_t_dn15_slot: &mut f64,
        var_cgd0_t_dn2_slot: &mut f64,
        var_cgd0_t_dn3_slot: &mut f64,
        var_cgd0_t_dn4_slot: &mut f64,
        var_cgd0_t_dn5_slot: &mut f64,
        var_cgd0_t_dn6_slot: &mut f64,
        var_cgd0_t_dn7_slot: &mut f64,
        var_cgd0_t_dn8_slot: &mut f64,
        var_cgd0_t_dn9_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_p10_t_slot: &mut f64,
        var_p10_t_db0_slot: &mut f64,
        var_p10_t_db1_slot: &mut f64,
        var_p10_t_db10_slot: &mut f64,
        var_p10_t_db11_slot: &mut f64,
        var_p10_t_db12_slot: &mut f64,
        var_p10_t_db13_slot: &mut f64,
        var_p10_t_db14_slot: &mut f64,
        var_p10_t_db2_slot: &mut f64,
        var_p10_t_db3_slot: &mut f64,
        var_p10_t_db4_slot: &mut f64,
        var_p10_t_db5_slot: &mut f64,
        var_p10_t_db6_slot: &mut f64,
        var_p10_t_db7_slot: &mut f64,
        var_p10_t_db8_slot: &mut f64,
        var_p10_t_db9_slot: &mut f64,
        var_p10_t_dn0_slot: &mut f64,
        var_p10_t_dn1_slot: &mut f64,
        var_p10_t_dn10_slot: &mut f64,
        var_p10_t_dn11_slot: &mut f64,
        var_p10_t_dn12_slot: &mut f64,
        var_p10_t_dn13_slot: &mut f64,
        var_p10_t_dn14_slot: &mut f64,
        var_p10_t_dn15_slot: &mut f64,
        var_p10_t_dn2_slot: &mut f64,
        var_p10_t_dn3_slot: &mut f64,
        var_p10_t_dn4_slot: &mut f64,
        var_p10_t_dn5_slot: &mut f64,
        var_p10_t_dn6_slot: &mut f64,
        var_p10_t_dn7_slot: &mut f64,
        var_p10_t_dn8_slot: &mut f64,
        var_p10_t_dn9_slot: &mut f64,
        var_p1m_slot: &mut f64,
        var_p1m_db0_slot: &mut f64,
        var_p1m_db1_slot: &mut f64,
        var_p1m_db10_slot: &mut f64,
        var_p1m_db11_slot: &mut f64,
        var_p1m_db12_slot: &mut f64,
        var_p1m_db13_slot: &mut f64,
        var_p1m_db14_slot: &mut f64,
        var_p1m_db2_slot: &mut f64,
        var_p1m_db3_slot: &mut f64,
        var_p1m_db4_slot: &mut f64,
        var_p1m_db5_slot: &mut f64,
        var_p1m_db6_slot: &mut f64,
        var_p1m_db7_slot: &mut f64,
        var_p1m_db8_slot: &mut f64,
        var_p1m_db9_slot: &mut f64,
        var_p1m_dn0_slot: &mut f64,
        var_p1m_dn1_slot: &mut f64,
        var_p1m_dn10_slot: &mut f64,
        var_p1m_dn11_slot: &mut f64,
        var_p1m_dn12_slot: &mut f64,
        var_p1m_dn13_slot: &mut f64,
        var_p1m_dn14_slot: &mut f64,
        var_p1m_dn15_slot: &mut f64,
        var_p1m_dn2_slot: &mut f64,
        var_p1m_dn3_slot: &mut f64,
        var_p1m_dn4_slot: &mut f64,
        var_p1m_dn5_slot: &mut f64,
        var_p1m_dn6_slot: &mut f64,
        var_p1m_dn7_slot: &mut f64,
        var_p1m_dn8_slot: &mut f64,
        var_p1m_dn9_slot: &mut f64,
        var_p40_t_slot: &mut f64,
        var_p40_t_db0_slot: &mut f64,
        var_p40_t_db1_slot: &mut f64,
        var_p40_t_db10_slot: &mut f64,
        var_p40_t_db11_slot: &mut f64,
        var_p40_t_db12_slot: &mut f64,
        var_p40_t_db13_slot: &mut f64,
        var_p40_t_db14_slot: &mut f64,
        var_p40_t_db2_slot: &mut f64,
        var_p40_t_db3_slot: &mut f64,
        var_p40_t_db4_slot: &mut f64,
        var_p40_t_db5_slot: &mut f64,
        var_p40_t_db6_slot: &mut f64,
        var_p40_t_db7_slot: &mut f64,
        var_p40_t_db8_slot: &mut f64,
        var_p40_t_db9_slot: &mut f64,
        var_p40_t_dn0_slot: &mut f64,
        var_p40_t_dn1_slot: &mut f64,
        var_p40_t_dn10_slot: &mut f64,
        var_p40_t_dn11_slot: &mut f64,
        var_p40_t_dn12_slot: &mut f64,
        var_p40_t_dn13_slot: &mut f64,
        var_p40_t_dn14_slot: &mut f64,
        var_p40_t_dn15_slot: &mut f64,
        var_p40_t_dn2_slot: &mut f64,
        var_p40_t_dn3_slot: &mut f64,
        var_p40_t_dn4_slot: &mut f64,
        var_p40_t_dn5_slot: &mut f64,
        var_p40_t_dn6_slot: &mut f64,
        var_p40_t_dn7_slot: &mut f64,
        var_p40_t_dn8_slot: &mut f64,
        var_p40_t_dn9_slot: &mut f64,
        var_pg_param_slot: &mut f64,
        var_pg_param_db0_slot: &mut f64,
        var_pg_param_db1_slot: &mut f64,
        var_pg_param_db10_slot: &mut f64,
        var_pg_param_db11_slot: &mut f64,
        var_pg_param_db12_slot: &mut f64,
        var_pg_param_db13_slot: &mut f64,
        var_pg_param_db14_slot: &mut f64,
        var_pg_param_db2_slot: &mut f64,
        var_pg_param_db3_slot: &mut f64,
        var_pg_param_db4_slot: &mut f64,
        var_pg_param_db5_slot: &mut f64,
        var_pg_param_db6_slot: &mut f64,
        var_pg_param_db7_slot: &mut f64,
        var_pg_param_db8_slot: &mut f64,
        var_pg_param_db9_slot: &mut f64,
        var_pg_param_dn0_slot: &mut f64,
        var_pg_param_dn1_slot: &mut f64,
        var_pg_param_dn10_slot: &mut f64,
        var_pg_param_dn11_slot: &mut f64,
        var_pg_param_dn12_slot: &mut f64,
        var_pg_param_dn13_slot: &mut f64,
        var_pg_param_dn14_slot: &mut f64,
        var_pg_param_dn15_slot: &mut f64,
        var_pg_param_dn2_slot: &mut f64,
        var_pg_param_dn3_slot: &mut f64,
        var_pg_param_dn4_slot: &mut f64,
        var_pg_param_dn5_slot: &mut f64,
        var_pg_param_dn6_slot: &mut f64,
        var_pg_param_dn7_slot: &mut f64,
        var_pg_param_dn8_slot: &mut f64,
        var_pg_param_dn9_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_db0_slot: &mut f64,
        var_t0_db1_slot: &mut f64,
        var_t0_db10_slot: &mut f64,
        var_t0_db11_slot: &mut f64,
        var_t0_db12_slot: &mut f64,
        var_t0_db13_slot: &mut f64,
        var_t0_db14_slot: &mut f64,
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
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_db0_slot: &mut f64,
        var_t1_db1_slot: &mut f64,
        var_t1_db10_slot: &mut f64,
        var_t1_db11_slot: &mut f64,
        var_t1_db12_slot: &mut f64,
        var_t1_db13_slot: &mut f64,
        var_t1_db14_slot: &mut f64,
        var_t1_db2_slot: &mut f64,
        var_t1_db3_slot: &mut f64,
        var_t1_db4_slot: &mut f64,
        var_t1_db5_slot: &mut f64,
        var_t1_db6_slot: &mut f64,
        var_t1_db7_slot: &mut f64,
        var_t1_db8_slot: &mut f64,
        var_t1_db9_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn15_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_db0_slot: &mut f64,
        var_t2_db1_slot: &mut f64,
        var_t2_db10_slot: &mut f64,
        var_t2_db11_slot: &mut f64,
        var_t2_db12_slot: &mut f64,
        var_t2_db13_slot: &mut f64,
        var_t2_db14_slot: &mut f64,
        var_t2_db2_slot: &mut f64,
        var_t2_db3_slot: &mut f64,
        var_t2_db4_slot: &mut f64,
        var_t2_db5_slot: &mut f64,
        var_t2_db6_slot: &mut f64,
        var_t2_db7_slot: &mut f64,
        var_t2_db8_slot: &mut f64,
        var_t2_db9_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn1_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn15_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_vjg_t_slot: &mut f64,
        var_vjg_t_db0_slot: &mut f64,
        var_vjg_t_db1_slot: &mut f64,
        var_vjg_t_db10_slot: &mut f64,
        var_vjg_t_db11_slot: &mut f64,
        var_vjg_t_db12_slot: &mut f64,
        var_vjg_t_db13_slot: &mut f64,
        var_vjg_t_db14_slot: &mut f64,
        var_vjg_t_db2_slot: &mut f64,
        var_vjg_t_db3_slot: &mut f64,
        var_vjg_t_db4_slot: &mut f64,
        var_vjg_t_db5_slot: &mut f64,
        var_vjg_t_db6_slot: &mut f64,
        var_vjg_t_db7_slot: &mut f64,
        var_vjg_t_db8_slot: &mut f64,
        var_vjg_t_db9_slot: &mut f64,
        var_vjg_t_dn0_slot: &mut f64,
        var_vjg_t_dn1_slot: &mut f64,
        var_vjg_t_dn10_slot: &mut f64,
        var_vjg_t_dn11_slot: &mut f64,
        var_vjg_t_dn12_slot: &mut f64,
        var_vjg_t_dn13_slot: &mut f64,
        var_vjg_t_dn14_slot: &mut f64,
        var_vjg_t_dn15_slot: &mut f64,
        var_vjg_t_dn2_slot: &mut f64,
        var_vjg_t_dn3_slot: &mut f64,
        var_vjg_t_dn4_slot: &mut f64,
        var_vjg_t_dn5_slot: &mut f64,
        var_vjg_t_dn6_slot: &mut f64,
        var_vjg_t_dn7_slot: &mut f64,
        var_vjg_t_dn8_slot: &mut f64,
        var_vjg_t_dn9_slot: &mut f64,
        var_vpkm_slot: &mut f64,
        var_vpkm_db0_slot: &mut f64,
        var_vpkm_db1_slot: &mut f64,
        var_vpkm_db10_slot: &mut f64,
        var_vpkm_db11_slot: &mut f64,
        var_vpkm_db12_slot: &mut f64,
        var_vpkm_db13_slot: &mut f64,
        var_vpkm_db14_slot: &mut f64,
        var_vpkm_db2_slot: &mut f64,
        var_vpkm_db3_slot: &mut f64,
        var_vpkm_db4_slot: &mut f64,
        var_vpkm_db5_slot: &mut f64,
        var_vpkm_db6_slot: &mut f64,
        var_vpkm_db7_slot: &mut f64,
        var_vpkm_db8_slot: &mut f64,
        var_vpkm_db9_slot: &mut f64,
        var_vpkm_dn0_slot: &mut f64,
        var_vpkm_dn1_slot: &mut f64,
        var_vpkm_dn10_slot: &mut f64,
        var_vpkm_dn11_slot: &mut f64,
        var_vpkm_dn12_slot: &mut f64,
        var_vpkm_dn13_slot: &mut f64,
        var_vpkm_dn14_slot: &mut f64,
        var_vpkm_dn15_slot: &mut f64,
        var_vpkm_dn2_slot: &mut f64,
        var_vpkm_dn3_slot: &mut f64,
        var_vpkm_dn4_slot: &mut f64,
        var_vpkm_dn5_slot: &mut f64,
        var_vpkm_dn6_slot: &mut f64,
        var_vpkm_dn7_slot: &mut f64,
        var_vpkm_dn8_slot: &mut f64,
        var_vpkm_dn9_slot: &mut f64,
        var_vpks_t_slot: &mut f64,
        var_vpks_t_db0_slot: &mut f64,
        var_vpks_t_db1_slot: &mut f64,
        var_vpks_t_db10_slot: &mut f64,
        var_vpks_t_db11_slot: &mut f64,
        var_vpks_t_db12_slot: &mut f64,
        var_vpks_t_db13_slot: &mut f64,
        var_vpks_t_db14_slot: &mut f64,
        var_vpks_t_db2_slot: &mut f64,
        var_vpks_t_db3_slot: &mut f64,
        var_vpks_t_db4_slot: &mut f64,
        var_vpks_t_db5_slot: &mut f64,
        var_vpks_t_db6_slot: &mut f64,
        var_vpks_t_db7_slot: &mut f64,
        var_vpks_t_db8_slot: &mut f64,
        var_vpks_t_db9_slot: &mut f64,
        var_vpks_t_dn0_slot: &mut f64,
        var_vpks_t_dn1_slot: &mut f64,
        var_vpks_t_dn10_slot: &mut f64,
        var_vpks_t_dn11_slot: &mut f64,
        var_vpks_t_dn12_slot: &mut f64,
        var_vpks_t_dn13_slot: &mut f64,
        var_vpks_t_dn14_slot: &mut f64,
        var_vpks_t_dn15_slot: &mut f64,
        var_vpks_t_dn2_slot: &mut f64,
        var_vpks_t_dn3_slot: &mut f64,
        var_vpks_t_dn4_slot: &mut f64,
        var_vpks_t_dn5_slot: &mut f64,
        var_vpks_t_dn6_slot: &mut f64,
        var_vpks_t_dn7_slot: &mut f64,
        var_vpks_t_dn8_slot: &mut f64,
        var_vpks_t_dn9_slot: &mut f64,
        var_vtr_t_slot: &mut f64,
        var_vtr_t_db0_slot: &mut f64,
        var_vtr_t_db1_slot: &mut f64,
        var_vtr_t_db10_slot: &mut f64,
        var_vtr_t_db11_slot: &mut f64,
        var_vtr_t_db12_slot: &mut f64,
        var_vtr_t_db13_slot: &mut f64,
        var_vtr_t_db14_slot: &mut f64,
        var_vtr_t_db2_slot: &mut f64,
        var_vtr_t_db3_slot: &mut f64,
        var_vtr_t_db4_slot: &mut f64,
        var_vtr_t_db5_slot: &mut f64,
        var_vtr_t_db6_slot: &mut f64,
        var_vtr_t_db7_slot: &mut f64,
        var_vtr_t_db8_slot: &mut f64,
        var_vtr_t_db9_slot: &mut f64,
        var_vtr_t_dn0_slot: &mut f64,
        var_vtr_t_dn1_slot: &mut f64,
        var_vtr_t_dn10_slot: &mut f64,
        var_vtr_t_dn11_slot: &mut f64,
        var_vtr_t_dn12_slot: &mut f64,
        var_vtr_t_dn13_slot: &mut f64,
        var_vtr_t_dn14_slot: &mut f64,
        var_vtr_t_dn15_slot: &mut f64,
        var_vtr_t_dn2_slot: &mut f64,
        var_vtr_t_dn3_slot: &mut f64,
        var_vtr_t_dn4_slot: &mut f64,
        var_vtr_t_dn5_slot: &mut f64,
        var_vtr_t_dn6_slot: &mut f64,
        var_vtr_t_dn7_slot: &mut f64,
        var_vtr_t_dn8_slot: &mut f64,
        var_vtr_t_dn9_slot: &mut f64,
    ) {
        let mut var_cgd0_t: f64 = *var_cgd0_t_slot;
        let mut var_cgd0_t_db0: f64 = *var_cgd0_t_db0_slot;
        let mut var_cgd0_t_db1: f64 = *var_cgd0_t_db1_slot;
        let mut var_cgd0_t_db10: f64 = *var_cgd0_t_db10_slot;
        let mut var_cgd0_t_db11: f64 = *var_cgd0_t_db11_slot;
        let mut var_cgd0_t_db12: f64 = *var_cgd0_t_db12_slot;
        let mut var_cgd0_t_db13: f64 = *var_cgd0_t_db13_slot;
        let mut var_cgd0_t_db14: f64 = *var_cgd0_t_db14_slot;
        let mut var_cgd0_t_db2: f64 = *var_cgd0_t_db2_slot;
        let mut var_cgd0_t_db3: f64 = *var_cgd0_t_db3_slot;
        let mut var_cgd0_t_db4: f64 = *var_cgd0_t_db4_slot;
        let mut var_cgd0_t_db5: f64 = *var_cgd0_t_db5_slot;
        let mut var_cgd0_t_db6: f64 = *var_cgd0_t_db6_slot;
        let mut var_cgd0_t_db7: f64 = *var_cgd0_t_db7_slot;
        let mut var_cgd0_t_db8: f64 = *var_cgd0_t_db8_slot;
        let mut var_cgd0_t_db9: f64 = *var_cgd0_t_db9_slot;
        let mut var_cgd0_t_dn0: f64 = *var_cgd0_t_dn0_slot;
        let mut var_cgd0_t_dn1: f64 = *var_cgd0_t_dn1_slot;
        let mut var_cgd0_t_dn10: f64 = *var_cgd0_t_dn10_slot;
        let mut var_cgd0_t_dn11: f64 = *var_cgd0_t_dn11_slot;
        let mut var_cgd0_t_dn12: f64 = *var_cgd0_t_dn12_slot;
        let mut var_cgd0_t_dn13: f64 = *var_cgd0_t_dn13_slot;
        let mut var_cgd0_t_dn14: f64 = *var_cgd0_t_dn14_slot;
        let mut var_cgd0_t_dn15: f64 = *var_cgd0_t_dn15_slot;
        let mut var_cgd0_t_dn2: f64 = *var_cgd0_t_dn2_slot;
        let mut var_cgd0_t_dn3: f64 = *var_cgd0_t_dn3_slot;
        let mut var_cgd0_t_dn4: f64 = *var_cgd0_t_dn4_slot;
        let mut var_cgd0_t_dn5: f64 = *var_cgd0_t_dn5_slot;
        let mut var_cgd0_t_dn6: f64 = *var_cgd0_t_dn6_slot;
        let mut var_cgd0_t_dn7: f64 = *var_cgd0_t_dn7_slot;
        let mut var_cgd0_t_dn8: f64 = *var_cgd0_t_dn8_slot;
        let mut var_cgd0_t_dn9: f64 = *var_cgd0_t_dn9_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_p10_t: f64 = *var_p10_t_slot;
        let mut var_p10_t_db0: f64 = *var_p10_t_db0_slot;
        let mut var_p10_t_db1: f64 = *var_p10_t_db1_slot;
        let mut var_p10_t_db10: f64 = *var_p10_t_db10_slot;
        let mut var_p10_t_db11: f64 = *var_p10_t_db11_slot;
        let mut var_p10_t_db12: f64 = *var_p10_t_db12_slot;
        let mut var_p10_t_db13: f64 = *var_p10_t_db13_slot;
        let mut var_p10_t_db14: f64 = *var_p10_t_db14_slot;
        let mut var_p10_t_db2: f64 = *var_p10_t_db2_slot;
        let mut var_p10_t_db3: f64 = *var_p10_t_db3_slot;
        let mut var_p10_t_db4: f64 = *var_p10_t_db4_slot;
        let mut var_p10_t_db5: f64 = *var_p10_t_db5_slot;
        let mut var_p10_t_db6: f64 = *var_p10_t_db6_slot;
        let mut var_p10_t_db7: f64 = *var_p10_t_db7_slot;
        let mut var_p10_t_db8: f64 = *var_p10_t_db8_slot;
        let mut var_p10_t_db9: f64 = *var_p10_t_db9_slot;
        let mut var_p10_t_dn0: f64 = *var_p10_t_dn0_slot;
        let mut var_p10_t_dn1: f64 = *var_p10_t_dn1_slot;
        let mut var_p10_t_dn10: f64 = *var_p10_t_dn10_slot;
        let mut var_p10_t_dn11: f64 = *var_p10_t_dn11_slot;
        let mut var_p10_t_dn12: f64 = *var_p10_t_dn12_slot;
        let mut var_p10_t_dn13: f64 = *var_p10_t_dn13_slot;
        let mut var_p10_t_dn14: f64 = *var_p10_t_dn14_slot;
        let mut var_p10_t_dn15: f64 = *var_p10_t_dn15_slot;
        let mut var_p10_t_dn2: f64 = *var_p10_t_dn2_slot;
        let mut var_p10_t_dn3: f64 = *var_p10_t_dn3_slot;
        let mut var_p10_t_dn4: f64 = *var_p10_t_dn4_slot;
        let mut var_p10_t_dn5: f64 = *var_p10_t_dn5_slot;
        let mut var_p10_t_dn6: f64 = *var_p10_t_dn6_slot;
        let mut var_p10_t_dn7: f64 = *var_p10_t_dn7_slot;
        let mut var_p10_t_dn8: f64 = *var_p10_t_dn8_slot;
        let mut var_p10_t_dn9: f64 = *var_p10_t_dn9_slot;
        let mut var_p1m: f64 = *var_p1m_slot;
        let mut var_p1m_db0: f64 = *var_p1m_db0_slot;
        let mut var_p1m_db1: f64 = *var_p1m_db1_slot;
        let mut var_p1m_db10: f64 = *var_p1m_db10_slot;
        let mut var_p1m_db11: f64 = *var_p1m_db11_slot;
        let mut var_p1m_db12: f64 = *var_p1m_db12_slot;
        let mut var_p1m_db13: f64 = *var_p1m_db13_slot;
        let mut var_p1m_db14: f64 = *var_p1m_db14_slot;
        let mut var_p1m_db2: f64 = *var_p1m_db2_slot;
        let mut var_p1m_db3: f64 = *var_p1m_db3_slot;
        let mut var_p1m_db4: f64 = *var_p1m_db4_slot;
        let mut var_p1m_db5: f64 = *var_p1m_db5_slot;
        let mut var_p1m_db6: f64 = *var_p1m_db6_slot;
        let mut var_p1m_db7: f64 = *var_p1m_db7_slot;
        let mut var_p1m_db8: f64 = *var_p1m_db8_slot;
        let mut var_p1m_db9: f64 = *var_p1m_db9_slot;
        let mut var_p1m_dn0: f64 = *var_p1m_dn0_slot;
        let mut var_p1m_dn1: f64 = *var_p1m_dn1_slot;
        let mut var_p1m_dn10: f64 = *var_p1m_dn10_slot;
        let mut var_p1m_dn11: f64 = *var_p1m_dn11_slot;
        let mut var_p1m_dn12: f64 = *var_p1m_dn12_slot;
        let mut var_p1m_dn13: f64 = *var_p1m_dn13_slot;
        let mut var_p1m_dn14: f64 = *var_p1m_dn14_slot;
        let mut var_p1m_dn15: f64 = *var_p1m_dn15_slot;
        let mut var_p1m_dn2: f64 = *var_p1m_dn2_slot;
        let mut var_p1m_dn3: f64 = *var_p1m_dn3_slot;
        let mut var_p1m_dn4: f64 = *var_p1m_dn4_slot;
        let mut var_p1m_dn5: f64 = *var_p1m_dn5_slot;
        let mut var_p1m_dn6: f64 = *var_p1m_dn6_slot;
        let mut var_p1m_dn7: f64 = *var_p1m_dn7_slot;
        let mut var_p1m_dn8: f64 = *var_p1m_dn8_slot;
        let mut var_p1m_dn9: f64 = *var_p1m_dn9_slot;
        let mut var_p40_t: f64 = *var_p40_t_slot;
        let mut var_p40_t_db0: f64 = *var_p40_t_db0_slot;
        let mut var_p40_t_db1: f64 = *var_p40_t_db1_slot;
        let mut var_p40_t_db10: f64 = *var_p40_t_db10_slot;
        let mut var_p40_t_db11: f64 = *var_p40_t_db11_slot;
        let mut var_p40_t_db12: f64 = *var_p40_t_db12_slot;
        let mut var_p40_t_db13: f64 = *var_p40_t_db13_slot;
        let mut var_p40_t_db14: f64 = *var_p40_t_db14_slot;
        let mut var_p40_t_db2: f64 = *var_p40_t_db2_slot;
        let mut var_p40_t_db3: f64 = *var_p40_t_db3_slot;
        let mut var_p40_t_db4: f64 = *var_p40_t_db4_slot;
        let mut var_p40_t_db5: f64 = *var_p40_t_db5_slot;
        let mut var_p40_t_db6: f64 = *var_p40_t_db6_slot;
        let mut var_p40_t_db7: f64 = *var_p40_t_db7_slot;
        let mut var_p40_t_db8: f64 = *var_p40_t_db8_slot;
        let mut var_p40_t_db9: f64 = *var_p40_t_db9_slot;
        let mut var_p40_t_dn0: f64 = *var_p40_t_dn0_slot;
        let mut var_p40_t_dn1: f64 = *var_p40_t_dn1_slot;
        let mut var_p40_t_dn10: f64 = *var_p40_t_dn10_slot;
        let mut var_p40_t_dn11: f64 = *var_p40_t_dn11_slot;
        let mut var_p40_t_dn12: f64 = *var_p40_t_dn12_slot;
        let mut var_p40_t_dn13: f64 = *var_p40_t_dn13_slot;
        let mut var_p40_t_dn14: f64 = *var_p40_t_dn14_slot;
        let mut var_p40_t_dn15: f64 = *var_p40_t_dn15_slot;
        let mut var_p40_t_dn2: f64 = *var_p40_t_dn2_slot;
        let mut var_p40_t_dn3: f64 = *var_p40_t_dn3_slot;
        let mut var_p40_t_dn4: f64 = *var_p40_t_dn4_slot;
        let mut var_p40_t_dn5: f64 = *var_p40_t_dn5_slot;
        let mut var_p40_t_dn6: f64 = *var_p40_t_dn6_slot;
        let mut var_p40_t_dn7: f64 = *var_p40_t_dn7_slot;
        let mut var_p40_t_dn8: f64 = *var_p40_t_dn8_slot;
        let mut var_p40_t_dn9: f64 = *var_p40_t_dn9_slot;
        let mut var_pg_param: f64 = *var_pg_param_slot;
        let mut var_pg_param_db0: f64 = *var_pg_param_db0_slot;
        let mut var_pg_param_db1: f64 = *var_pg_param_db1_slot;
        let mut var_pg_param_db10: f64 = *var_pg_param_db10_slot;
        let mut var_pg_param_db11: f64 = *var_pg_param_db11_slot;
        let mut var_pg_param_db12: f64 = *var_pg_param_db12_slot;
        let mut var_pg_param_db13: f64 = *var_pg_param_db13_slot;
        let mut var_pg_param_db14: f64 = *var_pg_param_db14_slot;
        let mut var_pg_param_db2: f64 = *var_pg_param_db2_slot;
        let mut var_pg_param_db3: f64 = *var_pg_param_db3_slot;
        let mut var_pg_param_db4: f64 = *var_pg_param_db4_slot;
        let mut var_pg_param_db5: f64 = *var_pg_param_db5_slot;
        let mut var_pg_param_db6: f64 = *var_pg_param_db6_slot;
        let mut var_pg_param_db7: f64 = *var_pg_param_db7_slot;
        let mut var_pg_param_db8: f64 = *var_pg_param_db8_slot;
        let mut var_pg_param_db9: f64 = *var_pg_param_db9_slot;
        let mut var_pg_param_dn0: f64 = *var_pg_param_dn0_slot;
        let mut var_pg_param_dn1: f64 = *var_pg_param_dn1_slot;
        let mut var_pg_param_dn10: f64 = *var_pg_param_dn10_slot;
        let mut var_pg_param_dn11: f64 = *var_pg_param_dn11_slot;
        let mut var_pg_param_dn12: f64 = *var_pg_param_dn12_slot;
        let mut var_pg_param_dn13: f64 = *var_pg_param_dn13_slot;
        let mut var_pg_param_dn14: f64 = *var_pg_param_dn14_slot;
        let mut var_pg_param_dn15: f64 = *var_pg_param_dn15_slot;
        let mut var_pg_param_dn2: f64 = *var_pg_param_dn2_slot;
        let mut var_pg_param_dn3: f64 = *var_pg_param_dn3_slot;
        let mut var_pg_param_dn4: f64 = *var_pg_param_dn4_slot;
        let mut var_pg_param_dn5: f64 = *var_pg_param_dn5_slot;
        let mut var_pg_param_dn6: f64 = *var_pg_param_dn6_slot;
        let mut var_pg_param_dn7: f64 = *var_pg_param_dn7_slot;
        let mut var_pg_param_dn8: f64 = *var_pg_param_dn8_slot;
        let mut var_pg_param_dn9: f64 = *var_pg_param_dn9_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_db0: f64 = *var_t0_db0_slot;
        let mut var_t0_db1: f64 = *var_t0_db1_slot;
        let mut var_t0_db10: f64 = *var_t0_db10_slot;
        let mut var_t0_db11: f64 = *var_t0_db11_slot;
        let mut var_t0_db12: f64 = *var_t0_db12_slot;
        let mut var_t0_db13: f64 = *var_t0_db13_slot;
        let mut var_t0_db14: f64 = *var_t0_db14_slot;
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
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_db0: f64 = *var_t1_db0_slot;
        let mut var_t1_db1: f64 = *var_t1_db1_slot;
        let mut var_t1_db10: f64 = *var_t1_db10_slot;
        let mut var_t1_db11: f64 = *var_t1_db11_slot;
        let mut var_t1_db12: f64 = *var_t1_db12_slot;
        let mut var_t1_db13: f64 = *var_t1_db13_slot;
        let mut var_t1_db14: f64 = *var_t1_db14_slot;
        let mut var_t1_db2: f64 = *var_t1_db2_slot;
        let mut var_t1_db3: f64 = *var_t1_db3_slot;
        let mut var_t1_db4: f64 = *var_t1_db4_slot;
        let mut var_t1_db5: f64 = *var_t1_db5_slot;
        let mut var_t1_db6: f64 = *var_t1_db6_slot;
        let mut var_t1_db7: f64 = *var_t1_db7_slot;
        let mut var_t1_db8: f64 = *var_t1_db8_slot;
        let mut var_t1_db9: f64 = *var_t1_db9_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn15: f64 = *var_t1_dn15_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_db0: f64 = *var_t2_db0_slot;
        let mut var_t2_db1: f64 = *var_t2_db1_slot;
        let mut var_t2_db10: f64 = *var_t2_db10_slot;
        let mut var_t2_db11: f64 = *var_t2_db11_slot;
        let mut var_t2_db12: f64 = *var_t2_db12_slot;
        let mut var_t2_db13: f64 = *var_t2_db13_slot;
        let mut var_t2_db14: f64 = *var_t2_db14_slot;
        let mut var_t2_db2: f64 = *var_t2_db2_slot;
        let mut var_t2_db3: f64 = *var_t2_db3_slot;
        let mut var_t2_db4: f64 = *var_t2_db4_slot;
        let mut var_t2_db5: f64 = *var_t2_db5_slot;
        let mut var_t2_db6: f64 = *var_t2_db6_slot;
        let mut var_t2_db7: f64 = *var_t2_db7_slot;
        let mut var_t2_db8: f64 = *var_t2_db8_slot;
        let mut var_t2_db9: f64 = *var_t2_db9_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn1: f64 = *var_t2_dn1_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn15: f64 = *var_t2_dn15_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_vjg_t: f64 = *var_vjg_t_slot;
        let mut var_vjg_t_db0: f64 = *var_vjg_t_db0_slot;
        let mut var_vjg_t_db1: f64 = *var_vjg_t_db1_slot;
        let mut var_vjg_t_db10: f64 = *var_vjg_t_db10_slot;
        let mut var_vjg_t_db11: f64 = *var_vjg_t_db11_slot;
        let mut var_vjg_t_db12: f64 = *var_vjg_t_db12_slot;
        let mut var_vjg_t_db13: f64 = *var_vjg_t_db13_slot;
        let mut var_vjg_t_db14: f64 = *var_vjg_t_db14_slot;
        let mut var_vjg_t_db2: f64 = *var_vjg_t_db2_slot;
        let mut var_vjg_t_db3: f64 = *var_vjg_t_db3_slot;
        let mut var_vjg_t_db4: f64 = *var_vjg_t_db4_slot;
        let mut var_vjg_t_db5: f64 = *var_vjg_t_db5_slot;
        let mut var_vjg_t_db6: f64 = *var_vjg_t_db6_slot;
        let mut var_vjg_t_db7: f64 = *var_vjg_t_db7_slot;
        let mut var_vjg_t_db8: f64 = *var_vjg_t_db8_slot;
        let mut var_vjg_t_db9: f64 = *var_vjg_t_db9_slot;
        let mut var_vjg_t_dn0: f64 = *var_vjg_t_dn0_slot;
        let mut var_vjg_t_dn1: f64 = *var_vjg_t_dn1_slot;
        let mut var_vjg_t_dn10: f64 = *var_vjg_t_dn10_slot;
        let mut var_vjg_t_dn11: f64 = *var_vjg_t_dn11_slot;
        let mut var_vjg_t_dn12: f64 = *var_vjg_t_dn12_slot;
        let mut var_vjg_t_dn13: f64 = *var_vjg_t_dn13_slot;
        let mut var_vjg_t_dn14: f64 = *var_vjg_t_dn14_slot;
        let mut var_vjg_t_dn15: f64 = *var_vjg_t_dn15_slot;
        let mut var_vjg_t_dn2: f64 = *var_vjg_t_dn2_slot;
        let mut var_vjg_t_dn3: f64 = *var_vjg_t_dn3_slot;
        let mut var_vjg_t_dn4: f64 = *var_vjg_t_dn4_slot;
        let mut var_vjg_t_dn5: f64 = *var_vjg_t_dn5_slot;
        let mut var_vjg_t_dn6: f64 = *var_vjg_t_dn6_slot;
        let mut var_vjg_t_dn7: f64 = *var_vjg_t_dn7_slot;
        let mut var_vjg_t_dn8: f64 = *var_vjg_t_dn8_slot;
        let mut var_vjg_t_dn9: f64 = *var_vjg_t_dn9_slot;
        let mut var_vpkm: f64 = *var_vpkm_slot;
        let mut var_vpkm_db0: f64 = *var_vpkm_db0_slot;
        let mut var_vpkm_db1: f64 = *var_vpkm_db1_slot;
        let mut var_vpkm_db10: f64 = *var_vpkm_db10_slot;
        let mut var_vpkm_db11: f64 = *var_vpkm_db11_slot;
        let mut var_vpkm_db12: f64 = *var_vpkm_db12_slot;
        let mut var_vpkm_db13: f64 = *var_vpkm_db13_slot;
        let mut var_vpkm_db14: f64 = *var_vpkm_db14_slot;
        let mut var_vpkm_db2: f64 = *var_vpkm_db2_slot;
        let mut var_vpkm_db3: f64 = *var_vpkm_db3_slot;
        let mut var_vpkm_db4: f64 = *var_vpkm_db4_slot;
        let mut var_vpkm_db5: f64 = *var_vpkm_db5_slot;
        let mut var_vpkm_db6: f64 = *var_vpkm_db6_slot;
        let mut var_vpkm_db7: f64 = *var_vpkm_db7_slot;
        let mut var_vpkm_db8: f64 = *var_vpkm_db8_slot;
        let mut var_vpkm_db9: f64 = *var_vpkm_db9_slot;
        let mut var_vpkm_dn0: f64 = *var_vpkm_dn0_slot;
        let mut var_vpkm_dn1: f64 = *var_vpkm_dn1_slot;
        let mut var_vpkm_dn10: f64 = *var_vpkm_dn10_slot;
        let mut var_vpkm_dn11: f64 = *var_vpkm_dn11_slot;
        let mut var_vpkm_dn12: f64 = *var_vpkm_dn12_slot;
        let mut var_vpkm_dn13: f64 = *var_vpkm_dn13_slot;
        let mut var_vpkm_dn14: f64 = *var_vpkm_dn14_slot;
        let mut var_vpkm_dn15: f64 = *var_vpkm_dn15_slot;
        let mut var_vpkm_dn2: f64 = *var_vpkm_dn2_slot;
        let mut var_vpkm_dn3: f64 = *var_vpkm_dn3_slot;
        let mut var_vpkm_dn4: f64 = *var_vpkm_dn4_slot;
        let mut var_vpkm_dn5: f64 = *var_vpkm_dn5_slot;
        let mut var_vpkm_dn6: f64 = *var_vpkm_dn6_slot;
        let mut var_vpkm_dn7: f64 = *var_vpkm_dn7_slot;
        let mut var_vpkm_dn8: f64 = *var_vpkm_dn8_slot;
        let mut var_vpkm_dn9: f64 = *var_vpkm_dn9_slot;
        let mut var_vpks_t: f64 = *var_vpks_t_slot;
        let mut var_vpks_t_db0: f64 = *var_vpks_t_db0_slot;
        let mut var_vpks_t_db1: f64 = *var_vpks_t_db1_slot;
        let mut var_vpks_t_db10: f64 = *var_vpks_t_db10_slot;
        let mut var_vpks_t_db11: f64 = *var_vpks_t_db11_slot;
        let mut var_vpks_t_db12: f64 = *var_vpks_t_db12_slot;
        let mut var_vpks_t_db13: f64 = *var_vpks_t_db13_slot;
        let mut var_vpks_t_db14: f64 = *var_vpks_t_db14_slot;
        let mut var_vpks_t_db2: f64 = *var_vpks_t_db2_slot;
        let mut var_vpks_t_db3: f64 = *var_vpks_t_db3_slot;
        let mut var_vpks_t_db4: f64 = *var_vpks_t_db4_slot;
        let mut var_vpks_t_db5: f64 = *var_vpks_t_db5_slot;
        let mut var_vpks_t_db6: f64 = *var_vpks_t_db6_slot;
        let mut var_vpks_t_db7: f64 = *var_vpks_t_db7_slot;
        let mut var_vpks_t_db8: f64 = *var_vpks_t_db8_slot;
        let mut var_vpks_t_db9: f64 = *var_vpks_t_db9_slot;
        let mut var_vpks_t_dn0: f64 = *var_vpks_t_dn0_slot;
        let mut var_vpks_t_dn1: f64 = *var_vpks_t_dn1_slot;
        let mut var_vpks_t_dn10: f64 = *var_vpks_t_dn10_slot;
        let mut var_vpks_t_dn11: f64 = *var_vpks_t_dn11_slot;
        let mut var_vpks_t_dn12: f64 = *var_vpks_t_dn12_slot;
        let mut var_vpks_t_dn13: f64 = *var_vpks_t_dn13_slot;
        let mut var_vpks_t_dn14: f64 = *var_vpks_t_dn14_slot;
        let mut var_vpks_t_dn15: f64 = *var_vpks_t_dn15_slot;
        let mut var_vpks_t_dn2: f64 = *var_vpks_t_dn2_slot;
        let mut var_vpks_t_dn3: f64 = *var_vpks_t_dn3_slot;
        let mut var_vpks_t_dn4: f64 = *var_vpks_t_dn4_slot;
        let mut var_vpks_t_dn5: f64 = *var_vpks_t_dn5_slot;
        let mut var_vpks_t_dn6: f64 = *var_vpks_t_dn6_slot;
        let mut var_vpks_t_dn7: f64 = *var_vpks_t_dn7_slot;
        let mut var_vpks_t_dn8: f64 = *var_vpks_t_dn8_slot;
        let mut var_vpks_t_dn9: f64 = *var_vpks_t_dn9_slot;
        let mut var_vtr_t: f64 = *var_vtr_t_slot;
        let mut var_vtr_t_db0: f64 = *var_vtr_t_db0_slot;
        let mut var_vtr_t_db1: f64 = *var_vtr_t_db1_slot;
        let mut var_vtr_t_db10: f64 = *var_vtr_t_db10_slot;
        let mut var_vtr_t_db11: f64 = *var_vtr_t_db11_slot;
        let mut var_vtr_t_db12: f64 = *var_vtr_t_db12_slot;
        let mut var_vtr_t_db13: f64 = *var_vtr_t_db13_slot;
        let mut var_vtr_t_db14: f64 = *var_vtr_t_db14_slot;
        let mut var_vtr_t_db2: f64 = *var_vtr_t_db2_slot;
        let mut var_vtr_t_db3: f64 = *var_vtr_t_db3_slot;
        let mut var_vtr_t_db4: f64 = *var_vtr_t_db4_slot;
        let mut var_vtr_t_db5: f64 = *var_vtr_t_db5_slot;
        let mut var_vtr_t_db6: f64 = *var_vtr_t_db6_slot;
        let mut var_vtr_t_db7: f64 = *var_vtr_t_db7_slot;
        let mut var_vtr_t_db8: f64 = *var_vtr_t_db8_slot;
        let mut var_vtr_t_db9: f64 = *var_vtr_t_db9_slot;
        let mut var_vtr_t_dn0: f64 = *var_vtr_t_dn0_slot;
        let mut var_vtr_t_dn1: f64 = *var_vtr_t_dn1_slot;
        let mut var_vtr_t_dn10: f64 = *var_vtr_t_dn10_slot;
        let mut var_vtr_t_dn11: f64 = *var_vtr_t_dn11_slot;
        let mut var_vtr_t_dn12: f64 = *var_vtr_t_dn12_slot;
        let mut var_vtr_t_dn13: f64 = *var_vtr_t_dn13_slot;
        let mut var_vtr_t_dn14: f64 = *var_vtr_t_dn14_slot;
        let mut var_vtr_t_dn15: f64 = *var_vtr_t_dn15_slot;
        let mut var_vtr_t_dn2: f64 = *var_vtr_t_dn2_slot;
        let mut var_vtr_t_dn3: f64 = *var_vtr_t_dn3_slot;
        let mut var_vtr_t_dn4: f64 = *var_vtr_t_dn4_slot;
        let mut var_vtr_t_dn5: f64 = *var_vtr_t_dn5_slot;
        let mut var_vtr_t_dn6: f64 = *var_vtr_t_dn6_slot;
        let mut var_vtr_t_dn7: f64 = *var_vtr_t_dn7_slot;
        let mut var_vtr_t_dn8: f64 = *var_vtr_t_dn8_slot;
        let mut var_vtr_t_dn9: f64 = *var_vtr_t_dn9_slot;

        let (assign390_e772, assign390_e772_d_n0, assign390_e772_d_n1, assign390_e772_d_n2, assign390_e772_d_n3, assign390_e772_d_n4, assign390_e772_d_n5, assign390_e772_d_n6, assign390_e772_d_n7, assign390_e772_d_n8, assign390_e772_d_n9, assign390_e772_d_n10, assign390_e772_d_n11, assign390_e772_d_n12, assign390_e772_d_n13, assign390_e772_d_n14, assign390_e772_d_n15, assign390_e772_d_b0, assign390_e772_d_b1, assign390_e772_d_b2, assign390_e772_d_b3, assign390_e772_d_b4, assign390_e772_d_b5, assign390_e772_d_b6, assign390_e772_d_b7, assign390_e772_d_b8, assign390_e772_d_b9, assign390_e772_d_b10, assign390_e772_d_b11, assign390_e772_d_b12, assign390_e772_d_b13, assign390_e772_d_b14,) = {
    if (var_guard3 == 0.0) {
        (p.p28, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgd0_t, var_cgd0_t_dn0, var_cgd0_t_dn1, var_cgd0_t_dn2, var_cgd0_t_dn3, var_cgd0_t_dn4, var_cgd0_t_dn5, var_cgd0_t_dn6, var_cgd0_t_dn7, var_cgd0_t_dn8, var_cgd0_t_dn9, var_cgd0_t_dn10, var_cgd0_t_dn11, var_cgd0_t_dn12, var_cgd0_t_dn13, var_cgd0_t_dn14, var_cgd0_t_dn15, var_cgd0_t_db0, var_cgd0_t_db1, var_cgd0_t_db2, var_cgd0_t_db3, var_cgd0_t_db4, var_cgd0_t_db5, var_cgd0_t_db6, var_cgd0_t_db7, var_cgd0_t_db8, var_cgd0_t_db9, var_cgd0_t_db10, var_cgd0_t_db11, var_cgd0_t_db12, var_cgd0_t_db13, var_cgd0_t_db14,)
    }
};
        var_cgd0_t = assign390_e772;
        var_cgd0_t_dn0 = assign390_e772_d_n0;
        var_cgd0_t_dn1 = assign390_e772_d_n1;
        var_cgd0_t_dn2 = assign390_e772_d_n2;
        var_cgd0_t_dn3 = assign390_e772_d_n3;
        var_cgd0_t_dn4 = assign390_e772_d_n4;
        var_cgd0_t_dn5 = assign390_e772_d_n5;
        var_cgd0_t_dn6 = assign390_e772_d_n6;
        var_cgd0_t_dn7 = assign390_e772_d_n7;
        var_cgd0_t_dn8 = assign390_e772_d_n8;
        var_cgd0_t_dn9 = assign390_e772_d_n9;
        var_cgd0_t_dn10 = assign390_e772_d_n10;
        var_cgd0_t_dn11 = assign390_e772_d_n11;
        var_cgd0_t_dn12 = assign390_e772_d_n12;
        var_cgd0_t_dn13 = assign390_e772_d_n13;
        var_cgd0_t_dn14 = assign390_e772_d_n14;
        var_cgd0_t_dn15 = assign390_e772_d_n15;
        var_cgd0_t_db0 = assign390_e772_d_b0;
        var_cgd0_t_db1 = assign390_e772_d_b1;
        var_cgd0_t_db2 = assign390_e772_d_b2;
        var_cgd0_t_db3 = assign390_e772_d_b3;
        var_cgd0_t_db4 = assign390_e772_d_b4;
        var_cgd0_t_db5 = assign390_e772_d_b5;
        var_cgd0_t_db6 = assign390_e772_d_b6;
        var_cgd0_t_db7 = assign390_e772_d_b7;
        var_cgd0_t_db8 = assign390_e772_d_b8;
        var_cgd0_t_db9 = assign390_e772_d_b9;
        var_cgd0_t_db10 = assign390_e772_d_b10;
        var_cgd0_t_db11 = assign390_e772_d_b11;
        var_cgd0_t_db12 = assign390_e772_d_b12;
        var_cgd0_t_db13 = assign390_e772_d_b13;
        var_cgd0_t_db14 = assign390_e772_d_b14;

        let (assign420_e787, assign420_e787_d_n0, assign420_e787_d_n1, assign420_e787_d_n2, assign420_e787_d_n3, assign420_e787_d_n4, assign420_e787_d_n5, assign420_e787_d_n6, assign420_e787_d_n7, assign420_e787_d_n8, assign420_e787_d_n9, assign420_e787_d_n10, assign420_e787_d_n11, assign420_e787_d_n12, assign420_e787_d_n13, assign420_e787_d_n14, assign420_e787_d_n15, assign420_e787_d_b0, assign420_e787_d_b1, assign420_e787_d_b2, assign420_e787_d_b3, assign420_e787_d_b4, assign420_e787_d_b5, assign420_e787_d_b6, assign420_e787_d_b7, assign420_e787_d_b8, assign420_e787_d_b9, assign420_e787_d_b10, assign420_e787_d_b11, assign420_e787_d_b12, assign420_e787_d_b13, assign420_e787_d_b14,) = {
    if (var_guard3 == 0.0) {
        (p.p9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vpks_t, var_vpks_t_dn0, var_vpks_t_dn1, var_vpks_t_dn2, var_vpks_t_dn3, var_vpks_t_dn4, var_vpks_t_dn5, var_vpks_t_dn6, var_vpks_t_dn7, var_vpks_t_dn8, var_vpks_t_dn9, var_vpks_t_dn10, var_vpks_t_dn11, var_vpks_t_dn12, var_vpks_t_dn13, var_vpks_t_dn14, var_vpks_t_dn15, var_vpks_t_db0, var_vpks_t_db1, var_vpks_t_db2, var_vpks_t_db3, var_vpks_t_db4, var_vpks_t_db5, var_vpks_t_db6, var_vpks_t_db7, var_vpks_t_db8, var_vpks_t_db9, var_vpks_t_db10, var_vpks_t_db11, var_vpks_t_db12, var_vpks_t_db13, var_vpks_t_db14,)
    }
};
        var_vpks_t = assign420_e787;
        var_vpks_t_dn0 = assign420_e787_d_n0;
        var_vpks_t_dn1 = assign420_e787_d_n1;
        var_vpks_t_dn2 = assign420_e787_d_n2;
        var_vpks_t_dn3 = assign420_e787_d_n3;
        var_vpks_t_dn4 = assign420_e787_d_n4;
        var_vpks_t_dn5 = assign420_e787_d_n5;
        var_vpks_t_dn6 = assign420_e787_d_n6;
        var_vpks_t_dn7 = assign420_e787_d_n7;
        var_vpks_t_dn8 = assign420_e787_d_n8;
        var_vpks_t_dn9 = assign420_e787_d_n9;
        var_vpks_t_dn10 = assign420_e787_d_n10;
        var_vpks_t_dn11 = assign420_e787_d_n11;
        var_vpks_t_dn12 = assign420_e787_d_n12;
        var_vpks_t_dn13 = assign420_e787_d_n13;
        var_vpks_t_dn14 = assign420_e787_d_n14;
        var_vpks_t_dn15 = assign420_e787_d_n15;
        var_vpks_t_db0 = assign420_e787_d_b0;
        var_vpks_t_db1 = assign420_e787_d_b1;
        var_vpks_t_db2 = assign420_e787_d_b2;
        var_vpks_t_db3 = assign420_e787_d_b3;
        var_vpks_t_db4 = assign420_e787_d_b4;
        var_vpks_t_db5 = assign420_e787_d_b5;
        var_vpks_t_db6 = assign420_e787_d_b6;
        var_vpks_t_db7 = assign420_e787_d_b7;
        var_vpks_t_db8 = assign420_e787_d_b8;
        var_vpks_t_db9 = assign420_e787_d_b9;
        var_vpks_t_db10 = assign420_e787_d_b10;
        var_vpks_t_db11 = assign420_e787_d_b11;
        var_vpks_t_db12 = assign420_e787_d_b12;
        var_vpks_t_db13 = assign420_e787_d_b13;
        var_vpks_t_db14 = assign420_e787_d_b14;

        let (assign430_e792, assign430_e792_d_n0, assign430_e792_d_n1, assign430_e792_d_n2, assign430_e792_d_n3, assign430_e792_d_n4, assign430_e792_d_n5, assign430_e792_d_n6, assign430_e792_d_n7, assign430_e792_d_n8, assign430_e792_d_n9, assign430_e792_d_n10, assign430_e792_d_n11, assign430_e792_d_n12, assign430_e792_d_n13, assign430_e792_d_n14, assign430_e792_d_n15, assign430_e792_d_b0, assign430_e792_d_b1, assign430_e792_d_b2, assign430_e792_d_b3, assign430_e792_d_b4, assign430_e792_d_b5, assign430_e792_d_b6, assign430_e792_d_b7, assign430_e792_d_b8, assign430_e792_d_b9, assign430_e792_d_b10, assign430_e792_d_b11, assign430_e792_d_b12, assign430_e792_d_b13, assign430_e792_d_b14,) = {
    if (var_guard3 == 0.0) {
        (p.p29, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_p10_t, var_p10_t_dn0, var_p10_t_dn1, var_p10_t_dn2, var_p10_t_dn3, var_p10_t_dn4, var_p10_t_dn5, var_p10_t_dn6, var_p10_t_dn7, var_p10_t_dn8, var_p10_t_dn9, var_p10_t_dn10, var_p10_t_dn11, var_p10_t_dn12, var_p10_t_dn13, var_p10_t_dn14, var_p10_t_dn15, var_p10_t_db0, var_p10_t_db1, var_p10_t_db2, var_p10_t_db3, var_p10_t_db4, var_p10_t_db5, var_p10_t_db6, var_p10_t_db7, var_p10_t_db8, var_p10_t_db9, var_p10_t_db10, var_p10_t_db11, var_p10_t_db12, var_p10_t_db13, var_p10_t_db14,)
    }
};
        var_p10_t = assign430_e792;
        var_p10_t_dn0 = assign430_e792_d_n0;
        var_p10_t_dn1 = assign430_e792_d_n1;
        var_p10_t_dn2 = assign430_e792_d_n2;
        var_p10_t_dn3 = assign430_e792_d_n3;
        var_p10_t_dn4 = assign430_e792_d_n4;
        var_p10_t_dn5 = assign430_e792_d_n5;
        var_p10_t_dn6 = assign430_e792_d_n6;
        var_p10_t_dn7 = assign430_e792_d_n7;
        var_p10_t_dn8 = assign430_e792_d_n8;
        var_p10_t_dn9 = assign430_e792_d_n9;
        var_p10_t_dn10 = assign430_e792_d_n10;
        var_p10_t_dn11 = assign430_e792_d_n11;
        var_p10_t_dn12 = assign430_e792_d_n12;
        var_p10_t_dn13 = assign430_e792_d_n13;
        var_p10_t_dn14 = assign430_e792_d_n14;
        var_p10_t_dn15 = assign430_e792_d_n15;
        var_p10_t_db0 = assign430_e792_d_b0;
        var_p10_t_db1 = assign430_e792_d_b1;
        var_p10_t_db2 = assign430_e792_d_b2;
        var_p10_t_db3 = assign430_e792_d_b3;
        var_p10_t_db4 = assign430_e792_d_b4;
        var_p10_t_db5 = assign430_e792_d_b5;
        var_p10_t_db6 = assign430_e792_d_b6;
        var_p10_t_db7 = assign430_e792_d_b7;
        var_p10_t_db8 = assign430_e792_d_b8;
        var_p10_t_db9 = assign430_e792_d_b9;
        var_p10_t_db10 = assign430_e792_d_b10;
        var_p10_t_db11 = assign430_e792_d_b11;
        var_p10_t_db12 = assign430_e792_d_b12;
        var_p10_t_db13 = assign430_e792_d_b13;
        var_p10_t_db14 = assign430_e792_d_b14;

        let (assign440_e797, assign440_e797_d_n0, assign440_e797_d_n1, assign440_e797_d_n2, assign440_e797_d_n3, assign440_e797_d_n4, assign440_e797_d_n5, assign440_e797_d_n6, assign440_e797_d_n7, assign440_e797_d_n8, assign440_e797_d_n9, assign440_e797_d_n10, assign440_e797_d_n11, assign440_e797_d_n12, assign440_e797_d_n13, assign440_e797_d_n14, assign440_e797_d_n15, assign440_e797_d_b0, assign440_e797_d_b1, assign440_e797_d_b2, assign440_e797_d_b3, assign440_e797_d_b4, assign440_e797_d_b5, assign440_e797_d_b6, assign440_e797_d_b7, assign440_e797_d_b8, assign440_e797_d_b9, assign440_e797_d_b10, assign440_e797_d_b11, assign440_e797_d_b12, assign440_e797_d_b13, assign440_e797_d_b14,) = {
    if (var_guard3 == 0.0) {
        (p.p35, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_p40_t, var_p40_t_dn0, var_p40_t_dn1, var_p40_t_dn2, var_p40_t_dn3, var_p40_t_dn4, var_p40_t_dn5, var_p40_t_dn6, var_p40_t_dn7, var_p40_t_dn8, var_p40_t_dn9, var_p40_t_dn10, var_p40_t_dn11, var_p40_t_dn12, var_p40_t_dn13, var_p40_t_dn14, var_p40_t_dn15, var_p40_t_db0, var_p40_t_db1, var_p40_t_db2, var_p40_t_db3, var_p40_t_db4, var_p40_t_db5, var_p40_t_db6, var_p40_t_db7, var_p40_t_db8, var_p40_t_db9, var_p40_t_db10, var_p40_t_db11, var_p40_t_db12, var_p40_t_db13, var_p40_t_db14,)
    }
};
        var_p40_t = assign440_e797;
        var_p40_t_dn0 = assign440_e797_d_n0;
        var_p40_t_dn1 = assign440_e797_d_n1;
        var_p40_t_dn2 = assign440_e797_d_n2;
        var_p40_t_dn3 = assign440_e797_d_n3;
        var_p40_t_dn4 = assign440_e797_d_n4;
        var_p40_t_dn5 = assign440_e797_d_n5;
        var_p40_t_dn6 = assign440_e797_d_n6;
        var_p40_t_dn7 = assign440_e797_d_n7;
        var_p40_t_dn8 = assign440_e797_d_n8;
        var_p40_t_dn9 = assign440_e797_d_n9;
        var_p40_t_dn10 = assign440_e797_d_n10;
        var_p40_t_dn11 = assign440_e797_d_n11;
        var_p40_t_dn12 = assign440_e797_d_n12;
        var_p40_t_dn13 = assign440_e797_d_n13;
        var_p40_t_dn14 = assign440_e797_d_n14;
        var_p40_t_dn15 = assign440_e797_d_n15;
        var_p40_t_db0 = assign440_e797_d_b0;
        var_p40_t_db1 = assign440_e797_d_b1;
        var_p40_t_db2 = assign440_e797_d_b2;
        var_p40_t_db3 = assign440_e797_d_b3;
        var_p40_t_db4 = assign440_e797_d_b4;
        var_p40_t_db5 = assign440_e797_d_b5;
        var_p40_t_db6 = assign440_e797_d_b6;
        var_p40_t_db7 = assign440_e797_d_b7;
        var_p40_t_db8 = assign440_e797_d_b8;
        var_p40_t_db9 = assign440_e797_d_b9;
        var_p40_t_db10 = assign440_e797_d_b10;
        var_p40_t_db11 = assign440_e797_d_b11;
        var_p40_t_db12 = assign440_e797_d_b12;
        var_p40_t_db13 = assign440_e797_d_b13;
        var_p40_t_db14 = assign440_e797_d_b14;

        let (assign450_e802, assign450_e802_d_n0, assign450_e802_d_n1, assign450_e802_d_n2, assign450_e802_d_n3, assign450_e802_d_n4, assign450_e802_d_n5, assign450_e802_d_n6, assign450_e802_d_n7, assign450_e802_d_n8, assign450_e802_d_n9, assign450_e802_d_n10, assign450_e802_d_n11, assign450_e802_d_n12, assign450_e802_d_n13, assign450_e802_d_n14, assign450_e802_d_n15, assign450_e802_d_b0, assign450_e802_d_b1, assign450_e802_d_b2, assign450_e802_d_b3, assign450_e802_d_b4, assign450_e802_d_b5, assign450_e802_d_b6, assign450_e802_d_b7, assign450_e802_d_b8, assign450_e802_d_b9, assign450_e802_d_b10, assign450_e802_d_b11, assign450_e802_d_b12, assign450_e802_d_b13, assign450_e802_d_b14,) = {
    if (var_guard3 == 0.0) {
        (p.p41, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vjg_t, var_vjg_t_dn0, var_vjg_t_dn1, var_vjg_t_dn2, var_vjg_t_dn3, var_vjg_t_dn4, var_vjg_t_dn5, var_vjg_t_dn6, var_vjg_t_dn7, var_vjg_t_dn8, var_vjg_t_dn9, var_vjg_t_dn10, var_vjg_t_dn11, var_vjg_t_dn12, var_vjg_t_dn13, var_vjg_t_dn14, var_vjg_t_dn15, var_vjg_t_db0, var_vjg_t_db1, var_vjg_t_db2, var_vjg_t_db3, var_vjg_t_db4, var_vjg_t_db5, var_vjg_t_db6, var_vjg_t_db7, var_vjg_t_db8, var_vjg_t_db9, var_vjg_t_db10, var_vjg_t_db11, var_vjg_t_db12, var_vjg_t_db13, var_vjg_t_db14,)
    }
};
        var_vjg_t = assign450_e802;
        var_vjg_t_dn0 = assign450_e802_d_n0;
        var_vjg_t_dn1 = assign450_e802_d_n1;
        var_vjg_t_dn2 = assign450_e802_d_n2;
        var_vjg_t_dn3 = assign450_e802_d_n3;
        var_vjg_t_dn4 = assign450_e802_d_n4;
        var_vjg_t_dn5 = assign450_e802_d_n5;
        var_vjg_t_dn6 = assign450_e802_d_n6;
        var_vjg_t_dn7 = assign450_e802_d_n7;
        var_vjg_t_dn8 = assign450_e802_d_n8;
        var_vjg_t_dn9 = assign450_e802_d_n9;
        var_vjg_t_dn10 = assign450_e802_d_n10;
        var_vjg_t_dn11 = assign450_e802_d_n11;
        var_vjg_t_dn12 = assign450_e802_d_n12;
        var_vjg_t_dn13 = assign450_e802_d_n13;
        var_vjg_t_dn14 = assign450_e802_d_n14;
        var_vjg_t_dn15 = assign450_e802_d_n15;
        var_vjg_t_db0 = assign450_e802_d_b0;
        var_vjg_t_db1 = assign450_e802_d_b1;
        var_vjg_t_db2 = assign450_e802_d_b2;
        var_vjg_t_db3 = assign450_e802_d_b3;
        var_vjg_t_db4 = assign450_e802_d_b4;
        var_vjg_t_db5 = assign450_e802_d_b5;
        var_vjg_t_db6 = assign450_e802_d_b6;
        var_vjg_t_db7 = assign450_e802_d_b7;
        var_vjg_t_db8 = assign450_e802_d_b8;
        var_vjg_t_db9 = assign450_e802_d_b9;
        var_vjg_t_db10 = assign450_e802_d_b10;
        var_vjg_t_db11 = assign450_e802_d_b11;
        var_vjg_t_db12 = assign450_e802_d_b12;
        var_vjg_t_db13 = assign450_e802_d_b13;
        var_vjg_t_db14 = assign450_e802_d_b14;

        let (assign460_e807, assign460_e807_d_n0, assign460_e807_d_n1, assign460_e807_d_n2, assign460_e807_d_n3, assign460_e807_d_n4, assign460_e807_d_n5, assign460_e807_d_n6, assign460_e807_d_n7, assign460_e807_d_n8, assign460_e807_d_n9, assign460_e807_d_n10, assign460_e807_d_n11, assign460_e807_d_n12, assign460_e807_d_n13, assign460_e807_d_n14, assign460_e807_d_n15, assign460_e807_d_b0, assign460_e807_d_b1, assign460_e807_d_b2, assign460_e807_d_b3, assign460_e807_d_b4, assign460_e807_d_b5, assign460_e807_d_b6, assign460_e807_d_b7, assign460_e807_d_b8, assign460_e807_d_b9, assign460_e807_d_b10, assign460_e807_d_b11, assign460_e807_d_b12, assign460_e807_d_b13, assign460_e807_d_b14,) = {
    if (var_guard3 == 0.0) {
        (p.p21, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vtr_t, var_vtr_t_dn0, var_vtr_t_dn1, var_vtr_t_dn2, var_vtr_t_dn3, var_vtr_t_dn4, var_vtr_t_dn5, var_vtr_t_dn6, var_vtr_t_dn7, var_vtr_t_dn8, var_vtr_t_dn9, var_vtr_t_dn10, var_vtr_t_dn11, var_vtr_t_dn12, var_vtr_t_dn13, var_vtr_t_dn14, var_vtr_t_dn15, var_vtr_t_db0, var_vtr_t_db1, var_vtr_t_db2, var_vtr_t_db3, var_vtr_t_db4, var_vtr_t_db5, var_vtr_t_db6, var_vtr_t_db7, var_vtr_t_db8, var_vtr_t_db9, var_vtr_t_db10, var_vtr_t_db11, var_vtr_t_db12, var_vtr_t_db13, var_vtr_t_db14,)
    }
};
        var_vtr_t = assign460_e807;
        var_vtr_t_dn0 = assign460_e807_d_n0;
        var_vtr_t_dn1 = assign460_e807_d_n1;
        var_vtr_t_dn2 = assign460_e807_d_n2;
        var_vtr_t_dn3 = assign460_e807_d_n3;
        var_vtr_t_dn4 = assign460_e807_d_n4;
        var_vtr_t_dn5 = assign460_e807_d_n5;
        var_vtr_t_dn6 = assign460_e807_d_n6;
        var_vtr_t_dn7 = assign460_e807_d_n7;
        var_vtr_t_dn8 = assign460_e807_d_n8;
        var_vtr_t_dn9 = assign460_e807_d_n9;
        var_vtr_t_dn10 = assign460_e807_d_n10;
        var_vtr_t_dn11 = assign460_e807_d_n11;
        var_vtr_t_dn12 = assign460_e807_d_n12;
        var_vtr_t_dn13 = assign460_e807_d_n13;
        var_vtr_t_dn14 = assign460_e807_d_n14;
        var_vtr_t_dn15 = assign460_e807_d_n15;
        var_vtr_t_db0 = assign460_e807_d_b0;
        var_vtr_t_db1 = assign460_e807_d_b1;
        var_vtr_t_db2 = assign460_e807_d_b2;
        var_vtr_t_db3 = assign460_e807_d_b3;
        var_vtr_t_db4 = assign460_e807_d_b4;
        var_vtr_t_db5 = assign460_e807_d_b5;
        var_vtr_t_db6 = assign460_e807_d_b6;
        var_vtr_t_db7 = assign460_e807_d_b7;
        var_vtr_t_db8 = assign460_e807_d_b8;
        var_vtr_t_db9 = assign460_e807_d_b9;
        var_vtr_t_db10 = assign460_e807_d_b10;
        var_vtr_t_db11 = assign460_e807_d_b11;
        var_vtr_t_db12 = assign460_e807_d_b12;
        var_vtr_t_db13 = assign460_e807_d_b13;
        var_vtr_t_db14 = assign460_e807_d_b14;

        let assign470_e813: f64 = if ((!param_given[39]) && param_given[40]) { 1.0 } else { 0.0 };
        var_guard4 = assign470_e813;

        let (assign480_e821, assign480_e821_d_n0, assign480_e821_d_n1, assign480_e821_d_n2, assign480_e821_d_n3, assign480_e821_d_n4, assign480_e821_d_n5, assign480_e821_d_n6, assign480_e821_d_n7, assign480_e821_d_n8, assign480_e821_d_n9, assign480_e821_d_n10, assign480_e821_d_n11, assign480_e821_d_n12, assign480_e821_d_n13, assign480_e821_d_n14, assign480_e821_d_n15, assign480_e821_d_b0, assign480_e821_d_b1, assign480_e821_d_b2, assign480_e821_d_b3, assign480_e821_d_b4, assign480_e821_d_b5, assign480_e821_d_b6, assign480_e821_d_b7, assign480_e821_d_b8, assign480_e821_d_b9, assign480_e821_d_b10, assign480_e821_d_b11, assign480_e821_d_b12, assign480_e821_d_b13, assign480_e821_d_b14,) = {
    if (var_guard4 != 0.0) {
        let assign480_e817: f64 = (0.5 / p.p40);
        let assign480_e819: f64 = (assign480_e817 / var_vth);
        (assign480_e819, (-((assign480_e817 * var_vth_dn0) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_dn1) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_dn2) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_dn3) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_dn4) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_dn5) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_dn6) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_dn7) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_dn8) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_dn9) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_dn10) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_dn11) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_dn12) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_dn13) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_dn14) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_dn15) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_db0) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_db1) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_db2) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_db3) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_db4) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_db5) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_db6) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_db7) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_db8) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_db9) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_db10) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_db11) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_db12) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_db13) / (var_vth * var_vth))), (-((assign480_e817 * var_vth_db14) / (var_vth * var_vth))),)
    } else {
        (var_pg_param, var_pg_param_dn0, var_pg_param_dn1, var_pg_param_dn2, var_pg_param_dn3, var_pg_param_dn4, var_pg_param_dn5, var_pg_param_dn6, var_pg_param_dn7, var_pg_param_dn8, var_pg_param_dn9, var_pg_param_dn10, var_pg_param_dn11, var_pg_param_dn12, var_pg_param_dn13, var_pg_param_dn14, var_pg_param_dn15, var_pg_param_db0, var_pg_param_db1, var_pg_param_db2, var_pg_param_db3, var_pg_param_db4, var_pg_param_db5, var_pg_param_db6, var_pg_param_db7, var_pg_param_db8, var_pg_param_db9, var_pg_param_db10, var_pg_param_db11, var_pg_param_db12, var_pg_param_db13, var_pg_param_db14,)
    }
};
        var_pg_param = assign480_e821;
        var_pg_param_dn0 = assign480_e821_d_n0;
        var_pg_param_dn1 = assign480_e821_d_n1;
        var_pg_param_dn2 = assign480_e821_d_n2;
        var_pg_param_dn3 = assign480_e821_d_n3;
        var_pg_param_dn4 = assign480_e821_d_n4;
        var_pg_param_dn5 = assign480_e821_d_n5;
        var_pg_param_dn6 = assign480_e821_d_n6;
        var_pg_param_dn7 = assign480_e821_d_n7;
        var_pg_param_dn8 = assign480_e821_d_n8;
        var_pg_param_dn9 = assign480_e821_d_n9;
        var_pg_param_dn10 = assign480_e821_d_n10;
        var_pg_param_dn11 = assign480_e821_d_n11;
        var_pg_param_dn12 = assign480_e821_d_n12;
        var_pg_param_dn13 = assign480_e821_d_n13;
        var_pg_param_dn14 = assign480_e821_d_n14;
        var_pg_param_dn15 = assign480_e821_d_n15;
        var_pg_param_db0 = assign480_e821_d_b0;
        var_pg_param_db1 = assign480_e821_d_b1;
        var_pg_param_db2 = assign480_e821_d_b2;
        var_pg_param_db3 = assign480_e821_d_b3;
        var_pg_param_db4 = assign480_e821_d_b4;
        var_pg_param_db5 = assign480_e821_d_b5;
        var_pg_param_db6 = assign480_e821_d_b6;
        var_pg_param_db7 = assign480_e821_d_b7;
        var_pg_param_db8 = assign480_e821_d_b8;
        var_pg_param_db9 = assign480_e821_d_b9;
        var_pg_param_db10 = assign480_e821_d_b10;
        var_pg_param_db11 = assign480_e821_d_b11;
        var_pg_param_db12 = assign480_e821_d_b12;
        var_pg_param_db13 = assign480_e821_d_b13;
        var_pg_param_db14 = assign480_e821_d_b14;

        let (assign490_e826, assign490_e826_d_n0, assign490_e826_d_n1, assign490_e826_d_n2, assign490_e826_d_n3, assign490_e826_d_n4, assign490_e826_d_n5, assign490_e826_d_n6, assign490_e826_d_n7, assign490_e826_d_n8, assign490_e826_d_n9, assign490_e826_d_n10, assign490_e826_d_n11, assign490_e826_d_n12, assign490_e826_d_n13, assign490_e826_d_n14, assign490_e826_d_n15, assign490_e826_d_b0, assign490_e826_d_b1, assign490_e826_d_b2, assign490_e826_d_b3, assign490_e826_d_b4, assign490_e826_d_b5, assign490_e826_d_b6, assign490_e826_d_b7, assign490_e826_d_b8, assign490_e826_d_b9, assign490_e826_d_b10, assign490_e826_d_b11, assign490_e826_d_b12, assign490_e826_d_b13, assign490_e826_d_b14,) = {
    if (var_guard4 == 0.0) {
        (p.p39, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pg_param, var_pg_param_dn0, var_pg_param_dn1, var_pg_param_dn2, var_pg_param_dn3, var_pg_param_dn4, var_pg_param_dn5, var_pg_param_dn6, var_pg_param_dn7, var_pg_param_dn8, var_pg_param_dn9, var_pg_param_dn10, var_pg_param_dn11, var_pg_param_dn12, var_pg_param_dn13, var_pg_param_dn14, var_pg_param_dn15, var_pg_param_db0, var_pg_param_db1, var_pg_param_db2, var_pg_param_db3, var_pg_param_db4, var_pg_param_db5, var_pg_param_db6, var_pg_param_db7, var_pg_param_db8, var_pg_param_db9, var_pg_param_db10, var_pg_param_db11, var_pg_param_db12, var_pg_param_db13, var_pg_param_db14,)
    }
};
        var_pg_param = assign490_e826;
        var_pg_param_dn0 = assign490_e826_d_n0;
        var_pg_param_dn1 = assign490_e826_d_n1;
        var_pg_param_dn2 = assign490_e826_d_n2;
        var_pg_param_dn3 = assign490_e826_d_n3;
        var_pg_param_dn4 = assign490_e826_d_n4;
        var_pg_param_dn5 = assign490_e826_d_n5;
        var_pg_param_dn6 = assign490_e826_d_n6;
        var_pg_param_dn7 = assign490_e826_d_n7;
        var_pg_param_dn8 = assign490_e826_d_n8;
        var_pg_param_dn9 = assign490_e826_d_n9;
        var_pg_param_dn10 = assign490_e826_d_n10;
        var_pg_param_dn11 = assign490_e826_d_n11;
        var_pg_param_dn12 = assign490_e826_d_n12;
        var_pg_param_dn13 = assign490_e826_d_n13;
        var_pg_param_dn14 = assign490_e826_d_n14;
        var_pg_param_dn15 = assign490_e826_d_n15;
        var_pg_param_db0 = assign490_e826_d_b0;
        var_pg_param_db1 = assign490_e826_d_b1;
        var_pg_param_db2 = assign490_e826_d_b2;
        var_pg_param_db3 = assign490_e826_d_b3;
        var_pg_param_db4 = assign490_e826_d_b4;
        var_pg_param_db5 = assign490_e826_d_b5;
        var_pg_param_db6 = assign490_e826_d_b6;
        var_pg_param_db7 = assign490_e826_d_b7;
        var_pg_param_db8 = assign490_e826_d_b8;
        var_pg_param_db9 = assign490_e826_d_b9;
        var_pg_param_db10 = assign490_e826_d_b10;
        var_pg_param_db11 = assign490_e826_d_b11;
        var_pg_param_db12 = assign490_e826_d_b12;
        var_pg_param_db13 = assign490_e826_d_b13;
        var_pg_param_db14 = assign490_e826_d_b14;

        let assign500_e829: f64 = (p.p19 * var_vds);
        let assign500_e830: f64 = (assign500_e829).cosh();
        var_t0 = assign500_e830;
        var_t0_dn0 = ((assign500_e829).sinh() * (p.p19 * var_vds_dn0));
        var_t0_dn1 = ((assign500_e829).sinh() * (p.p19 * var_vds_dn1));
        var_t0_dn2 = ((assign500_e829).sinh() * (p.p19 * var_vds_dn2));
        var_t0_dn3 = ((assign500_e829).sinh() * (p.p19 * var_vds_dn3));
        var_t0_dn4 = ((assign500_e829).sinh() * (p.p19 * var_vds_dn4));
        var_t0_dn5 = ((assign500_e829).sinh() * (p.p19 * var_vds_dn5));
        var_t0_dn6 = ((assign500_e829).sinh() * (p.p19 * var_vds_dn6));
        var_t0_dn7 = ((assign500_e829).sinh() * (p.p19 * var_vds_dn7));
        var_t0_dn8 = ((assign500_e829).sinh() * (p.p19 * var_vds_dn8));
        var_t0_dn9 = ((assign500_e829).sinh() * (p.p19 * var_vds_dn9));
        var_t0_dn10 = ((assign500_e829).sinh() * (p.p19 * var_vds_dn10));
        var_t0_dn11 = ((assign500_e829).sinh() * (p.p19 * var_vds_dn11));
        var_t0_dn12 = ((assign500_e829).sinh() * (p.p19 * var_vds_dn12));
        var_t0_dn13 = ((assign500_e829).sinh() * (p.p19 * var_vds_dn13));
        var_t0_dn14 = ((assign500_e829).sinh() * (p.p19 * var_vds_dn14));
        var_t0_dn15 = ((assign500_e829).sinh() * (p.p19 * var_vds_dn15));
        var_t0_db0 = ((assign500_e829).sinh() * (p.p19 * var_vds_db0));
        var_t0_db1 = ((assign500_e829).sinh() * (p.p19 * var_vds_db1));
        var_t0_db2 = ((assign500_e829).sinh() * (p.p19 * var_vds_db2));
        var_t0_db3 = ((assign500_e829).sinh() * (p.p19 * var_vds_db3));
        var_t0_db4 = ((assign500_e829).sinh() * (p.p19 * var_vds_db4));
        var_t0_db5 = ((assign500_e829).sinh() * (p.p19 * var_vds_db5));
        var_t0_db6 = ((assign500_e829).sinh() * (p.p19 * var_vds_db6));
        var_t0_db7 = ((assign500_e829).sinh() * (p.p19 * var_vds_db7));
        var_t0_db8 = ((assign500_e829).sinh() * (p.p19 * var_vds_db8));
        var_t0_db9 = ((assign500_e829).sinh() * (p.p19 * var_vds_db9));
        var_t0_db10 = ((assign500_e829).sinh() * (p.p19 * var_vds_db10));
        var_t0_db11 = ((assign500_e829).sinh() * (p.p19 * var_vds_db11));
        var_t0_db12 = ((assign500_e829).sinh() * (p.p19 * var_vds_db12));
        var_t0_db13 = ((assign500_e829).sinh() * (p.p19 * var_vds_db13));
        var_t0_db14 = ((assign500_e829).sinh() * (p.p19 * var_vds_db14));

        let assign510_e836: f64 = (var_t0 * var_t0);
        let assign510_e837: f64 = (p.p18 / assign510_e836);
        let assign510_e838: f64 = (1.0 + assign510_e837);
        let assign510_e839: f64 = (var_p1_t * assign510_e838);
        var_p1m = assign510_e839;
        var_p1m_dn0 = ((var_p1_t_dn0 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_dn0 * var_t0) + (var_t0 * var_t0_dn0))) / (assign510_e836 * assign510_e836)))));
        var_p1m_dn1 = ((var_p1_t_dn1 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_dn1 * var_t0) + (var_t0 * var_t0_dn1))) / (assign510_e836 * assign510_e836)))));
        var_p1m_dn2 = ((var_p1_t_dn2 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_dn2 * var_t0) + (var_t0 * var_t0_dn2))) / (assign510_e836 * assign510_e836)))));
        var_p1m_dn3 = ((var_p1_t_dn3 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3))) / (assign510_e836 * assign510_e836)))));
        var_p1m_dn4 = ((var_p1_t_dn4 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4))) / (assign510_e836 * assign510_e836)))));
        var_p1m_dn5 = ((var_p1_t_dn5 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5))) / (assign510_e836 * assign510_e836)))));
        var_p1m_dn6 = ((var_p1_t_dn6 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6))) / (assign510_e836 * assign510_e836)))));
        var_p1m_dn7 = ((var_p1_t_dn7 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7))) / (assign510_e836 * assign510_e836)))));
        var_p1m_dn8 = ((var_p1_t_dn8 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8))) / (assign510_e836 * assign510_e836)))));
        var_p1m_dn9 = ((var_p1_t_dn9 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_dn9 * var_t0) + (var_t0 * var_t0_dn9))) / (assign510_e836 * assign510_e836)))));
        var_p1m_dn10 = ((var_p1_t_dn10 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_dn10 * var_t0) + (var_t0 * var_t0_dn10))) / (assign510_e836 * assign510_e836)))));
        var_p1m_dn11 = ((var_p1_t_dn11 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_dn11 * var_t0) + (var_t0 * var_t0_dn11))) / (assign510_e836 * assign510_e836)))));
        var_p1m_dn12 = ((var_p1_t_dn12 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_dn12 * var_t0) + (var_t0 * var_t0_dn12))) / (assign510_e836 * assign510_e836)))));
        var_p1m_dn13 = ((var_p1_t_dn13 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_dn13 * var_t0) + (var_t0 * var_t0_dn13))) / (assign510_e836 * assign510_e836)))));
        var_p1m_dn14 = ((var_p1_t_dn14 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_dn14 * var_t0) + (var_t0 * var_t0_dn14))) / (assign510_e836 * assign510_e836)))));
        var_p1m_dn15 = ((var_p1_t_dn15 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_dn15 * var_t0) + (var_t0 * var_t0_dn15))) / (assign510_e836 * assign510_e836)))));
        var_p1m_db0 = ((var_p1_t_db0 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_db0 * var_t0) + (var_t0 * var_t0_db0))) / (assign510_e836 * assign510_e836)))));
        var_p1m_db1 = ((var_p1_t_db1 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_db1 * var_t0) + (var_t0 * var_t0_db1))) / (assign510_e836 * assign510_e836)))));
        var_p1m_db2 = ((var_p1_t_db2 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_db2 * var_t0) + (var_t0 * var_t0_db2))) / (assign510_e836 * assign510_e836)))));
        var_p1m_db3 = ((var_p1_t_db3 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_db3 * var_t0) + (var_t0 * var_t0_db3))) / (assign510_e836 * assign510_e836)))));
        var_p1m_db4 = ((var_p1_t_db4 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_db4 * var_t0) + (var_t0 * var_t0_db4))) / (assign510_e836 * assign510_e836)))));
        var_p1m_db5 = ((var_p1_t_db5 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_db5 * var_t0) + (var_t0 * var_t0_db5))) / (assign510_e836 * assign510_e836)))));
        var_p1m_db6 = ((var_p1_t_db6 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_db6 * var_t0) + (var_t0 * var_t0_db6))) / (assign510_e836 * assign510_e836)))));
        var_p1m_db7 = ((var_p1_t_db7 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_db7 * var_t0) + (var_t0 * var_t0_db7))) / (assign510_e836 * assign510_e836)))));
        var_p1m_db8 = ((var_p1_t_db8 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_db8 * var_t0) + (var_t0 * var_t0_db8))) / (assign510_e836 * assign510_e836)))));
        var_p1m_db9 = ((var_p1_t_db9 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_db9 * var_t0) + (var_t0 * var_t0_db9))) / (assign510_e836 * assign510_e836)))));
        var_p1m_db10 = ((var_p1_t_db10 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_db10 * var_t0) + (var_t0 * var_t0_db10))) / (assign510_e836 * assign510_e836)))));
        var_p1m_db11 = ((var_p1_t_db11 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_db11 * var_t0) + (var_t0 * var_t0_db11))) / (assign510_e836 * assign510_e836)))));
        var_p1m_db12 = ((var_p1_t_db12 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_db12 * var_t0) + (var_t0 * var_t0_db12))) / (assign510_e836 * assign510_e836)))));
        var_p1m_db13 = ((var_p1_t_db13 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_db13 * var_t0) + (var_t0 * var_t0_db13))) / (assign510_e836 * assign510_e836)))));
        var_p1m_db14 = ((var_p1_t_db14 * assign510_e838) + (var_p1_t * (-((p.p18 * ((var_t0_db14 * var_t0) + (var_t0 * var_t0_db14))) / (assign510_e836 * assign510_e836)))));

        let assign520_e842: f64 = (var_vpks_t - p.p10);
        let assign520_e846: f64 = (p.p15 * var_vds);
        let assign520_e847: f64 = (assign520_e846).tanh();
        let assign520_e848: f64 = (p.p10 * assign520_e847);
        let assign520_e849: f64 = (assign520_e842 + assign520_e848);
        let assign520_e853: f64 = (var_vdg - p.p21);
        let assign520_e854: f64 = (p.p22 * assign520_e853);
        let assign520_e857: f64 = (var_vdg - var_vtr_t);
        let assign520_e858: f64 = (assign520_e854 * assign520_e857);
        let assign520_e859: f64 = (assign520_e849 - assign520_e858);
        var_vpkm = assign520_e859;
        var_vpkm_dn0 = ((var_vpks_t_dn0 + (p.p10 * ((p.p15 * var_vds_dn0) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_dn0) * assign520_e857) + (assign520_e854 * (var_vdg_dn0 - var_vtr_t_dn0))));
        var_vpkm_dn1 = ((var_vpks_t_dn1 + (p.p10 * ((p.p15 * var_vds_dn1) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_dn1) * assign520_e857) + (assign520_e854 * (var_vdg_dn1 - var_vtr_t_dn1))));
        var_vpkm_dn2 = ((var_vpks_t_dn2 + (p.p10 * ((p.p15 * var_vds_dn2) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_dn2) * assign520_e857) + (assign520_e854 * (var_vdg_dn2 - var_vtr_t_dn2))));
        var_vpkm_dn3 = ((var_vpks_t_dn3 + (p.p10 * ((p.p15 * var_vds_dn3) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_dn3) * assign520_e857) + (assign520_e854 * (var_vdg_dn3 - var_vtr_t_dn3))));
        var_vpkm_dn4 = ((var_vpks_t_dn4 + (p.p10 * ((p.p15 * var_vds_dn4) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_dn4) * assign520_e857) + (assign520_e854 * (var_vdg_dn4 - var_vtr_t_dn4))));
        var_vpkm_dn5 = ((var_vpks_t_dn5 + (p.p10 * ((p.p15 * var_vds_dn5) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_dn5) * assign520_e857) + (assign520_e854 * (var_vdg_dn5 - var_vtr_t_dn5))));
        var_vpkm_dn6 = ((var_vpks_t_dn6 + (p.p10 * ((p.p15 * var_vds_dn6) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_dn6) * assign520_e857) + (assign520_e854 * (var_vdg_dn6 - var_vtr_t_dn6))));
        var_vpkm_dn7 = ((var_vpks_t_dn7 + (p.p10 * ((p.p15 * var_vds_dn7) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_dn7) * assign520_e857) + (assign520_e854 * (var_vdg_dn7 - var_vtr_t_dn7))));
        var_vpkm_dn8 = ((var_vpks_t_dn8 + (p.p10 * ((p.p15 * var_vds_dn8) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_dn8) * assign520_e857) + (assign520_e854 * (var_vdg_dn8 - var_vtr_t_dn8))));
        var_vpkm_dn9 = ((var_vpks_t_dn9 + (p.p10 * ((p.p15 * var_vds_dn9) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_dn9) * assign520_e857) + (assign520_e854 * (var_vdg_dn9 - var_vtr_t_dn9))));
        var_vpkm_dn10 = ((var_vpks_t_dn10 + (p.p10 * ((p.p15 * var_vds_dn10) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_dn10) * assign520_e857) + (assign520_e854 * (var_vdg_dn10 - var_vtr_t_dn10))));
        var_vpkm_dn11 = ((var_vpks_t_dn11 + (p.p10 * ((p.p15 * var_vds_dn11) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_dn11) * assign520_e857) + (assign520_e854 * (var_vdg_dn11 - var_vtr_t_dn11))));
        var_vpkm_dn12 = ((var_vpks_t_dn12 + (p.p10 * ((p.p15 * var_vds_dn12) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_dn12) * assign520_e857) + (assign520_e854 * (var_vdg_dn12 - var_vtr_t_dn12))));
        var_vpkm_dn13 = ((var_vpks_t_dn13 + (p.p10 * ((p.p15 * var_vds_dn13) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_dn13) * assign520_e857) + (assign520_e854 * (var_vdg_dn13 - var_vtr_t_dn13))));
        var_vpkm_dn14 = ((var_vpks_t_dn14 + (p.p10 * ((p.p15 * var_vds_dn14) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_dn14) * assign520_e857) + (assign520_e854 * (var_vdg_dn14 - var_vtr_t_dn14))));
        var_vpkm_dn15 = ((var_vpks_t_dn15 + (p.p10 * ((p.p15 * var_vds_dn15) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_dn15) * assign520_e857) + (assign520_e854 * (var_vdg_dn15 - var_vtr_t_dn15))));
        var_vpkm_db0 = ((var_vpks_t_db0 + (p.p10 * ((p.p15 * var_vds_db0) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_db0) * assign520_e857) + (assign520_e854 * (var_vdg_db0 - var_vtr_t_db0))));
        var_vpkm_db1 = ((var_vpks_t_db1 + (p.p10 * ((p.p15 * var_vds_db1) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_db1) * assign520_e857) + (assign520_e854 * (var_vdg_db1 - var_vtr_t_db1))));
        var_vpkm_db2 = ((var_vpks_t_db2 + (p.p10 * ((p.p15 * var_vds_db2) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_db2) * assign520_e857) + (assign520_e854 * (var_vdg_db2 - var_vtr_t_db2))));
        var_vpkm_db3 = ((var_vpks_t_db3 + (p.p10 * ((p.p15 * var_vds_db3) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_db3) * assign520_e857) + (assign520_e854 * (var_vdg_db3 - var_vtr_t_db3))));
        var_vpkm_db4 = ((var_vpks_t_db4 + (p.p10 * ((p.p15 * var_vds_db4) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_db4) * assign520_e857) + (assign520_e854 * (var_vdg_db4 - var_vtr_t_db4))));
        var_vpkm_db5 = ((var_vpks_t_db5 + (p.p10 * ((p.p15 * var_vds_db5) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_db5) * assign520_e857) + (assign520_e854 * (var_vdg_db5 - var_vtr_t_db5))));
        var_vpkm_db6 = ((var_vpks_t_db6 + (p.p10 * ((p.p15 * var_vds_db6) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_db6) * assign520_e857) + (assign520_e854 * (var_vdg_db6 - var_vtr_t_db6))));
        var_vpkm_db7 = ((var_vpks_t_db7 + (p.p10 * ((p.p15 * var_vds_db7) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_db7) * assign520_e857) + (assign520_e854 * (var_vdg_db7 - var_vtr_t_db7))));
        var_vpkm_db8 = ((var_vpks_t_db8 + (p.p10 * ((p.p15 * var_vds_db8) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_db8) * assign520_e857) + (assign520_e854 * (var_vdg_db8 - var_vtr_t_db8))));
        var_vpkm_db9 = ((var_vpks_t_db9 + (p.p10 * ((p.p15 * var_vds_db9) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_db9) * assign520_e857) + (assign520_e854 * (var_vdg_db9 - var_vtr_t_db9))));
        var_vpkm_db10 = ((var_vpks_t_db10 + (p.p10 * ((p.p15 * var_vds_db10) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_db10) * assign520_e857) + (assign520_e854 * (var_vdg_db10 - var_vtr_t_db10))));
        var_vpkm_db11 = ((var_vpks_t_db11 + (p.p10 * ((p.p15 * var_vds_db11) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_db11) * assign520_e857) + (assign520_e854 * (var_vdg_db11 - var_vtr_t_db11))));
        var_vpkm_db12 = ((var_vpks_t_db12 + (p.p10 * ((p.p15 * var_vds_db12) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_db12) * assign520_e857) + (assign520_e854 * (var_vdg_db12 - var_vtr_t_db12))));
        var_vpkm_db13 = ((var_vpks_t_db13 + (p.p10 * ((p.p15 * var_vds_db13) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_db13) * assign520_e857) + (assign520_e854 * (var_vdg_db13 - var_vtr_t_db13))));
        var_vpkm_db14 = ((var_vpks_t_db14 + (p.p10 * ((p.p15 * var_vds_db14) / ((assign520_e846).cosh() * (assign520_e846).cosh())))) - (((p.p22 * var_vdg_db14) * assign520_e857) + (assign520_e854 * (var_vdg_db14 - var_vtr_t_db14))));

        let assign530_e862: f64 = (var_vgs - var_vpkm);
        var_t1 = assign530_e862;
        var_t1_dn0 = (var_vgs_dn0 - var_vpkm_dn0);
        var_t1_dn1 = (var_vgs_dn1 - var_vpkm_dn1);
        var_t1_dn2 = (var_vgs_dn2 - var_vpkm_dn2);
        var_t1_dn3 = (var_vgs_dn3 - var_vpkm_dn3);
        var_t1_dn4 = (var_vgs_dn4 - var_vpkm_dn4);
        var_t1_dn5 = (var_vgs_dn5 - var_vpkm_dn5);
        var_t1_dn6 = (var_vgs_dn6 - var_vpkm_dn6);
        var_t1_dn7 = (var_vgs_dn7 - var_vpkm_dn7);
        var_t1_dn8 = (var_vgs_dn8 - var_vpkm_dn8);
        var_t1_dn9 = (var_vgs_dn9 - var_vpkm_dn9);
        var_t1_dn10 = (var_vgs_dn10 - var_vpkm_dn10);
        var_t1_dn11 = (var_vgs_dn11 - var_vpkm_dn11);
        var_t1_dn12 = (var_vgs_dn12 - var_vpkm_dn12);
        var_t1_dn13 = (var_vgs_dn13 - var_vpkm_dn13);
        var_t1_dn14 = (var_vgs_dn14 - var_vpkm_dn14);
        var_t1_dn15 = (var_vgs_dn15 - var_vpkm_dn15);
        var_t1_db0 = (var_vgs_db0 - var_vpkm_db0);
        var_t1_db1 = (var_vgs_db1 - var_vpkm_db1);
        var_t1_db2 = (var_vgs_db2 - var_vpkm_db2);
        var_t1_db3 = (var_vgs_db3 - var_vpkm_db3);
        var_t1_db4 = (var_vgs_db4 - var_vpkm_db4);
        var_t1_db5 = (var_vgs_db5 - var_vpkm_db5);
        var_t1_db6 = (var_vgs_db6 - var_vpkm_db6);
        var_t1_db7 = (var_vgs_db7 - var_vpkm_db7);
        var_t1_db8 = (var_vgs_db8 - var_vpkm_db8);
        var_t1_db9 = (var_vgs_db9 - var_vpkm_db9);
        var_t1_db10 = (var_vgs_db10 - var_vpkm_db10);
        var_t1_db11 = (var_vgs_db11 - var_vpkm_db11);
        var_t1_db12 = (var_vgs_db12 - var_vpkm_db12);
        var_t1_db13 = (var_vgs_db13 - var_vpkm_db13);
        var_t1_db14 = (var_vgs_db14 - var_vpkm_db14);

        let assign540_e865: f64 = (var_t1 * var_t1);
        var_t2 = assign540_e865;
        var_t2_dn0 = ((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0));
        var_t2_dn1 = ((var_t1_dn1 * var_t1) + (var_t1 * var_t1_dn1));
        var_t2_dn2 = ((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2));
        var_t2_dn3 = ((var_t1_dn3 * var_t1) + (var_t1 * var_t1_dn3));
        var_t2_dn4 = ((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4));
        var_t2_dn5 = ((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5));
        var_t2_dn6 = ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6));
        var_t2_dn7 = ((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7));
        var_t2_dn8 = ((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8));
        var_t2_dn9 = ((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9));
        var_t2_dn10 = ((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10));
        var_t2_dn11 = ((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11));
        var_t2_dn12 = ((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12));
        var_t2_dn13 = ((var_t1_dn13 * var_t1) + (var_t1 * var_t1_dn13));
        var_t2_dn14 = ((var_t1_dn14 * var_t1) + (var_t1 * var_t1_dn14));
        var_t2_dn15 = ((var_t1_dn15 * var_t1) + (var_t1 * var_t1_dn15));
        var_t2_db0 = ((var_t1_db0 * var_t1) + (var_t1 * var_t1_db0));
        var_t2_db1 = ((var_t1_db1 * var_t1) + (var_t1 * var_t1_db1));
        var_t2_db2 = ((var_t1_db2 * var_t1) + (var_t1 * var_t1_db2));
        var_t2_db3 = ((var_t1_db3 * var_t1) + (var_t1 * var_t1_db3));
        var_t2_db4 = ((var_t1_db4 * var_t1) + (var_t1 * var_t1_db4));
        var_t2_db5 = ((var_t1_db5 * var_t1) + (var_t1 * var_t1_db5));
        var_t2_db6 = ((var_t1_db6 * var_t1) + (var_t1 * var_t1_db6));
        var_t2_db7 = ((var_t1_db7 * var_t1) + (var_t1 * var_t1_db7));
        var_t2_db8 = ((var_t1_db8 * var_t1) + (var_t1 * var_t1_db8));
        var_t2_db9 = ((var_t1_db9 * var_t1) + (var_t1 * var_t1_db9));
        var_t2_db10 = ((var_t1_db10 * var_t1) + (var_t1 * var_t1_db10));
        var_t2_db11 = ((var_t1_db11 * var_t1) + (var_t1 * var_t1_db11));
        var_t2_db12 = ((var_t1_db12 * var_t1) + (var_t1 * var_t1_db12));
        var_t2_db13 = ((var_t1_db13 * var_t1) + (var_t1 * var_t1_db13));
        var_t2_db14 = ((var_t1_db14 * var_t1) + (var_t1 * var_t1_db14));


        *var_cgd0_t_slot = var_cgd0_t;
        *var_cgd0_t_db0_slot = var_cgd0_t_db0;
        *var_cgd0_t_db1_slot = var_cgd0_t_db1;
        *var_cgd0_t_db10_slot = var_cgd0_t_db10;
        *var_cgd0_t_db11_slot = var_cgd0_t_db11;
        *var_cgd0_t_db12_slot = var_cgd0_t_db12;
        *var_cgd0_t_db13_slot = var_cgd0_t_db13;
        *var_cgd0_t_db14_slot = var_cgd0_t_db14;
        *var_cgd0_t_db2_slot = var_cgd0_t_db2;
        *var_cgd0_t_db3_slot = var_cgd0_t_db3;
        *var_cgd0_t_db4_slot = var_cgd0_t_db4;
        *var_cgd0_t_db5_slot = var_cgd0_t_db5;
        *var_cgd0_t_db6_slot = var_cgd0_t_db6;
        *var_cgd0_t_db7_slot = var_cgd0_t_db7;
        *var_cgd0_t_db8_slot = var_cgd0_t_db8;
        *var_cgd0_t_db9_slot = var_cgd0_t_db9;
        *var_cgd0_t_dn0_slot = var_cgd0_t_dn0;
        *var_cgd0_t_dn1_slot = var_cgd0_t_dn1;
        *var_cgd0_t_dn10_slot = var_cgd0_t_dn10;
        *var_cgd0_t_dn11_slot = var_cgd0_t_dn11;
        *var_cgd0_t_dn12_slot = var_cgd0_t_dn12;
        *var_cgd0_t_dn13_slot = var_cgd0_t_dn13;
        *var_cgd0_t_dn14_slot = var_cgd0_t_dn14;
        *var_cgd0_t_dn15_slot = var_cgd0_t_dn15;
        *var_cgd0_t_dn2_slot = var_cgd0_t_dn2;
        *var_cgd0_t_dn3_slot = var_cgd0_t_dn3;
        *var_cgd0_t_dn4_slot = var_cgd0_t_dn4;
        *var_cgd0_t_dn5_slot = var_cgd0_t_dn5;
        *var_cgd0_t_dn6_slot = var_cgd0_t_dn6;
        *var_cgd0_t_dn7_slot = var_cgd0_t_dn7;
        *var_cgd0_t_dn8_slot = var_cgd0_t_dn8;
        *var_cgd0_t_dn9_slot = var_cgd0_t_dn9;
        *var_guard4_slot = var_guard4;
        *var_p10_t_slot = var_p10_t;
        *var_p10_t_db0_slot = var_p10_t_db0;
        *var_p10_t_db1_slot = var_p10_t_db1;
        *var_p10_t_db10_slot = var_p10_t_db10;
        *var_p10_t_db11_slot = var_p10_t_db11;
        *var_p10_t_db12_slot = var_p10_t_db12;
        *var_p10_t_db13_slot = var_p10_t_db13;
        *var_p10_t_db14_slot = var_p10_t_db14;
        *var_p10_t_db2_slot = var_p10_t_db2;
        *var_p10_t_db3_slot = var_p10_t_db3;
        *var_p10_t_db4_slot = var_p10_t_db4;
        *var_p10_t_db5_slot = var_p10_t_db5;
        *var_p10_t_db6_slot = var_p10_t_db6;
        *var_p10_t_db7_slot = var_p10_t_db7;
        *var_p10_t_db8_slot = var_p10_t_db8;
        *var_p10_t_db9_slot = var_p10_t_db9;
        *var_p10_t_dn0_slot = var_p10_t_dn0;
        *var_p10_t_dn1_slot = var_p10_t_dn1;
        *var_p10_t_dn10_slot = var_p10_t_dn10;
        *var_p10_t_dn11_slot = var_p10_t_dn11;
        *var_p10_t_dn12_slot = var_p10_t_dn12;
        *var_p10_t_dn13_slot = var_p10_t_dn13;
        *var_p10_t_dn14_slot = var_p10_t_dn14;
        *var_p10_t_dn15_slot = var_p10_t_dn15;
        *var_p10_t_dn2_slot = var_p10_t_dn2;
        *var_p10_t_dn3_slot = var_p10_t_dn3;
        *var_p10_t_dn4_slot = var_p10_t_dn4;
        *var_p10_t_dn5_slot = var_p10_t_dn5;
        *var_p10_t_dn6_slot = var_p10_t_dn6;
        *var_p10_t_dn7_slot = var_p10_t_dn7;
        *var_p10_t_dn8_slot = var_p10_t_dn8;
        *var_p10_t_dn9_slot = var_p10_t_dn9;
        *var_p1m_slot = var_p1m;
        *var_p1m_db0_slot = var_p1m_db0;
        *var_p1m_db1_slot = var_p1m_db1;
        *var_p1m_db10_slot = var_p1m_db10;
        *var_p1m_db11_slot = var_p1m_db11;
        *var_p1m_db12_slot = var_p1m_db12;
        *var_p1m_db13_slot = var_p1m_db13;
        *var_p1m_db14_slot = var_p1m_db14;
        *var_p1m_db2_slot = var_p1m_db2;
        *var_p1m_db3_slot = var_p1m_db3;
        *var_p1m_db4_slot = var_p1m_db4;
        *var_p1m_db5_slot = var_p1m_db5;
        *var_p1m_db6_slot = var_p1m_db6;
        *var_p1m_db7_slot = var_p1m_db7;
        *var_p1m_db8_slot = var_p1m_db8;
        *var_p1m_db9_slot = var_p1m_db9;
        *var_p1m_dn0_slot = var_p1m_dn0;
        *var_p1m_dn1_slot = var_p1m_dn1;
        *var_p1m_dn10_slot = var_p1m_dn10;
        *var_p1m_dn11_slot = var_p1m_dn11;
        *var_p1m_dn12_slot = var_p1m_dn12;
        *var_p1m_dn13_slot = var_p1m_dn13;
        *var_p1m_dn14_slot = var_p1m_dn14;
        *var_p1m_dn15_slot = var_p1m_dn15;
        *var_p1m_dn2_slot = var_p1m_dn2;
        *var_p1m_dn3_slot = var_p1m_dn3;
        *var_p1m_dn4_slot = var_p1m_dn4;
        *var_p1m_dn5_slot = var_p1m_dn5;
        *var_p1m_dn6_slot = var_p1m_dn6;
        *var_p1m_dn7_slot = var_p1m_dn7;
        *var_p1m_dn8_slot = var_p1m_dn8;
        *var_p1m_dn9_slot = var_p1m_dn9;
        *var_p40_t_slot = var_p40_t;
        *var_p40_t_db0_slot = var_p40_t_db0;
        *var_p40_t_db1_slot = var_p40_t_db1;
        *var_p40_t_db10_slot = var_p40_t_db10;
        *var_p40_t_db11_slot = var_p40_t_db11;
        *var_p40_t_db12_slot = var_p40_t_db12;
        *var_p40_t_db13_slot = var_p40_t_db13;
        *var_p40_t_db14_slot = var_p40_t_db14;
        *var_p40_t_db2_slot = var_p40_t_db2;
        *var_p40_t_db3_slot = var_p40_t_db3;
        *var_p40_t_db4_slot = var_p40_t_db4;
        *var_p40_t_db5_slot = var_p40_t_db5;
        *var_p40_t_db6_slot = var_p40_t_db6;
        *var_p40_t_db7_slot = var_p40_t_db7;
        *var_p40_t_db8_slot = var_p40_t_db8;
        *var_p40_t_db9_slot = var_p40_t_db9;
        *var_p40_t_dn0_slot = var_p40_t_dn0;
        *var_p40_t_dn1_slot = var_p40_t_dn1;
        *var_p40_t_dn10_slot = var_p40_t_dn10;
        *var_p40_t_dn11_slot = var_p40_t_dn11;
        *var_p40_t_dn12_slot = var_p40_t_dn12;
        *var_p40_t_dn13_slot = var_p40_t_dn13;
        *var_p40_t_dn14_slot = var_p40_t_dn14;
        *var_p40_t_dn15_slot = var_p40_t_dn15;
        *var_p40_t_dn2_slot = var_p40_t_dn2;
        *var_p40_t_dn3_slot = var_p40_t_dn3;
        *var_p40_t_dn4_slot = var_p40_t_dn4;
        *var_p40_t_dn5_slot = var_p40_t_dn5;
        *var_p40_t_dn6_slot = var_p40_t_dn6;
        *var_p40_t_dn7_slot = var_p40_t_dn7;
        *var_p40_t_dn8_slot = var_p40_t_dn8;
        *var_p40_t_dn9_slot = var_p40_t_dn9;
        *var_pg_param_slot = var_pg_param;
        *var_pg_param_db0_slot = var_pg_param_db0;
        *var_pg_param_db1_slot = var_pg_param_db1;
        *var_pg_param_db10_slot = var_pg_param_db10;
        *var_pg_param_db11_slot = var_pg_param_db11;
        *var_pg_param_db12_slot = var_pg_param_db12;
        *var_pg_param_db13_slot = var_pg_param_db13;
        *var_pg_param_db14_slot = var_pg_param_db14;
        *var_pg_param_db2_slot = var_pg_param_db2;
        *var_pg_param_db3_slot = var_pg_param_db3;
        *var_pg_param_db4_slot = var_pg_param_db4;
        *var_pg_param_db5_slot = var_pg_param_db5;
        *var_pg_param_db6_slot = var_pg_param_db6;
        *var_pg_param_db7_slot = var_pg_param_db7;
        *var_pg_param_db8_slot = var_pg_param_db8;
        *var_pg_param_db9_slot = var_pg_param_db9;
        *var_pg_param_dn0_slot = var_pg_param_dn0;
        *var_pg_param_dn1_slot = var_pg_param_dn1;
        *var_pg_param_dn10_slot = var_pg_param_dn10;
        *var_pg_param_dn11_slot = var_pg_param_dn11;
        *var_pg_param_dn12_slot = var_pg_param_dn12;
        *var_pg_param_dn13_slot = var_pg_param_dn13;
        *var_pg_param_dn14_slot = var_pg_param_dn14;
        *var_pg_param_dn15_slot = var_pg_param_dn15;
        *var_pg_param_dn2_slot = var_pg_param_dn2;
        *var_pg_param_dn3_slot = var_pg_param_dn3;
        *var_pg_param_dn4_slot = var_pg_param_dn4;
        *var_pg_param_dn5_slot = var_pg_param_dn5;
        *var_pg_param_dn6_slot = var_pg_param_dn6;
        *var_pg_param_dn7_slot = var_pg_param_dn7;
        *var_pg_param_dn8_slot = var_pg_param_dn8;
        *var_pg_param_dn9_slot = var_pg_param_dn9;
        *var_t0_slot = var_t0;
        *var_t0_db0_slot = var_t0_db0;
        *var_t0_db1_slot = var_t0_db1;
        *var_t0_db10_slot = var_t0_db10;
        *var_t0_db11_slot = var_t0_db11;
        *var_t0_db12_slot = var_t0_db12;
        *var_t0_db13_slot = var_t0_db13;
        *var_t0_db14_slot = var_t0_db14;
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
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t1_slot = var_t1;
        *var_t1_db0_slot = var_t1_db0;
        *var_t1_db1_slot = var_t1_db1;
        *var_t1_db10_slot = var_t1_db10;
        *var_t1_db11_slot = var_t1_db11;
        *var_t1_db12_slot = var_t1_db12;
        *var_t1_db13_slot = var_t1_db13;
        *var_t1_db14_slot = var_t1_db14;
        *var_t1_db2_slot = var_t1_db2;
        *var_t1_db3_slot = var_t1_db3;
        *var_t1_db4_slot = var_t1_db4;
        *var_t1_db5_slot = var_t1_db5;
        *var_t1_db6_slot = var_t1_db6;
        *var_t1_db7_slot = var_t1_db7;
        *var_t1_db8_slot = var_t1_db8;
        *var_t1_db9_slot = var_t1_db9;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn15_slot = var_t1_dn15;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_db0_slot = var_t2_db0;
        *var_t2_db1_slot = var_t2_db1;
        *var_t2_db10_slot = var_t2_db10;
        *var_t2_db11_slot = var_t2_db11;
        *var_t2_db12_slot = var_t2_db12;
        *var_t2_db13_slot = var_t2_db13;
        *var_t2_db14_slot = var_t2_db14;
        *var_t2_db2_slot = var_t2_db2;
        *var_t2_db3_slot = var_t2_db3;
        *var_t2_db4_slot = var_t2_db4;
        *var_t2_db5_slot = var_t2_db5;
        *var_t2_db6_slot = var_t2_db6;
        *var_t2_db7_slot = var_t2_db7;
        *var_t2_db8_slot = var_t2_db8;
        *var_t2_db9_slot = var_t2_db9;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn1_slot = var_t2_dn1;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn15_slot = var_t2_dn15;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_vjg_t_slot = var_vjg_t;
        *var_vjg_t_db0_slot = var_vjg_t_db0;
        *var_vjg_t_db1_slot = var_vjg_t_db1;
        *var_vjg_t_db10_slot = var_vjg_t_db10;
        *var_vjg_t_db11_slot = var_vjg_t_db11;
        *var_vjg_t_db12_slot = var_vjg_t_db12;
        *var_vjg_t_db13_slot = var_vjg_t_db13;
        *var_vjg_t_db14_slot = var_vjg_t_db14;
        *var_vjg_t_db2_slot = var_vjg_t_db2;
        *var_vjg_t_db3_slot = var_vjg_t_db3;
        *var_vjg_t_db4_slot = var_vjg_t_db4;
        *var_vjg_t_db5_slot = var_vjg_t_db5;
        *var_vjg_t_db6_slot = var_vjg_t_db6;
        *var_vjg_t_db7_slot = var_vjg_t_db7;
        *var_vjg_t_db8_slot = var_vjg_t_db8;
        *var_vjg_t_db9_slot = var_vjg_t_db9;
        *var_vjg_t_dn0_slot = var_vjg_t_dn0;
        *var_vjg_t_dn1_slot = var_vjg_t_dn1;
        *var_vjg_t_dn10_slot = var_vjg_t_dn10;
        *var_vjg_t_dn11_slot = var_vjg_t_dn11;
        *var_vjg_t_dn12_slot = var_vjg_t_dn12;
        *var_vjg_t_dn13_slot = var_vjg_t_dn13;
        *var_vjg_t_dn14_slot = var_vjg_t_dn14;
        *var_vjg_t_dn15_slot = var_vjg_t_dn15;
        *var_vjg_t_dn2_slot = var_vjg_t_dn2;
        *var_vjg_t_dn3_slot = var_vjg_t_dn3;
        *var_vjg_t_dn4_slot = var_vjg_t_dn4;
        *var_vjg_t_dn5_slot = var_vjg_t_dn5;
        *var_vjg_t_dn6_slot = var_vjg_t_dn6;
        *var_vjg_t_dn7_slot = var_vjg_t_dn7;
        *var_vjg_t_dn8_slot = var_vjg_t_dn8;
        *var_vjg_t_dn9_slot = var_vjg_t_dn9;
        *var_vpkm_slot = var_vpkm;
        *var_vpkm_db0_slot = var_vpkm_db0;
        *var_vpkm_db1_slot = var_vpkm_db1;
        *var_vpkm_db10_slot = var_vpkm_db10;
        *var_vpkm_db11_slot = var_vpkm_db11;
        *var_vpkm_db12_slot = var_vpkm_db12;
        *var_vpkm_db13_slot = var_vpkm_db13;
        *var_vpkm_db14_slot = var_vpkm_db14;
        *var_vpkm_db2_slot = var_vpkm_db2;
        *var_vpkm_db3_slot = var_vpkm_db3;
        *var_vpkm_db4_slot = var_vpkm_db4;
        *var_vpkm_db5_slot = var_vpkm_db5;
        *var_vpkm_db6_slot = var_vpkm_db6;
        *var_vpkm_db7_slot = var_vpkm_db7;
        *var_vpkm_db8_slot = var_vpkm_db8;
        *var_vpkm_db9_slot = var_vpkm_db9;
        *var_vpkm_dn0_slot = var_vpkm_dn0;
        *var_vpkm_dn1_slot = var_vpkm_dn1;
        *var_vpkm_dn10_slot = var_vpkm_dn10;
        *var_vpkm_dn11_slot = var_vpkm_dn11;
        *var_vpkm_dn12_slot = var_vpkm_dn12;
        *var_vpkm_dn13_slot = var_vpkm_dn13;
        *var_vpkm_dn14_slot = var_vpkm_dn14;
        *var_vpkm_dn15_slot = var_vpkm_dn15;
        *var_vpkm_dn2_slot = var_vpkm_dn2;
        *var_vpkm_dn3_slot = var_vpkm_dn3;
        *var_vpkm_dn4_slot = var_vpkm_dn4;
        *var_vpkm_dn5_slot = var_vpkm_dn5;
        *var_vpkm_dn6_slot = var_vpkm_dn6;
        *var_vpkm_dn7_slot = var_vpkm_dn7;
        *var_vpkm_dn8_slot = var_vpkm_dn8;
        *var_vpkm_dn9_slot = var_vpkm_dn9;
        *var_vpks_t_slot = var_vpks_t;
        *var_vpks_t_db0_slot = var_vpks_t_db0;
        *var_vpks_t_db1_slot = var_vpks_t_db1;
        *var_vpks_t_db10_slot = var_vpks_t_db10;
        *var_vpks_t_db11_slot = var_vpks_t_db11;
        *var_vpks_t_db12_slot = var_vpks_t_db12;
        *var_vpks_t_db13_slot = var_vpks_t_db13;
        *var_vpks_t_db14_slot = var_vpks_t_db14;
        *var_vpks_t_db2_slot = var_vpks_t_db2;
        *var_vpks_t_db3_slot = var_vpks_t_db3;
        *var_vpks_t_db4_slot = var_vpks_t_db4;
        *var_vpks_t_db5_slot = var_vpks_t_db5;
        *var_vpks_t_db6_slot = var_vpks_t_db6;
        *var_vpks_t_db7_slot = var_vpks_t_db7;
        *var_vpks_t_db8_slot = var_vpks_t_db8;
        *var_vpks_t_db9_slot = var_vpks_t_db9;
        *var_vpks_t_dn0_slot = var_vpks_t_dn0;
        *var_vpks_t_dn1_slot = var_vpks_t_dn1;
        *var_vpks_t_dn10_slot = var_vpks_t_dn10;
        *var_vpks_t_dn11_slot = var_vpks_t_dn11;
        *var_vpks_t_dn12_slot = var_vpks_t_dn12;
        *var_vpks_t_dn13_slot = var_vpks_t_dn13;
        *var_vpks_t_dn14_slot = var_vpks_t_dn14;
        *var_vpks_t_dn15_slot = var_vpks_t_dn15;
        *var_vpks_t_dn2_slot = var_vpks_t_dn2;
        *var_vpks_t_dn3_slot = var_vpks_t_dn3;
        *var_vpks_t_dn4_slot = var_vpks_t_dn4;
        *var_vpks_t_dn5_slot = var_vpks_t_dn5;
        *var_vpks_t_dn6_slot = var_vpks_t_dn6;
        *var_vpks_t_dn7_slot = var_vpks_t_dn7;
        *var_vpks_t_dn8_slot = var_vpks_t_dn8;
        *var_vpks_t_dn9_slot = var_vpks_t_dn9;
        *var_vtr_t_slot = var_vtr_t;
        *var_vtr_t_db0_slot = var_vtr_t_db0;
        *var_vtr_t_db1_slot = var_vtr_t_db1;
        *var_vtr_t_db10_slot = var_vtr_t_db10;
        *var_vtr_t_db11_slot = var_vtr_t_db11;
        *var_vtr_t_db12_slot = var_vtr_t_db12;
        *var_vtr_t_db13_slot = var_vtr_t_db13;
        *var_vtr_t_db14_slot = var_vtr_t_db14;
        *var_vtr_t_db2_slot = var_vtr_t_db2;
        *var_vtr_t_db3_slot = var_vtr_t_db3;
        *var_vtr_t_db4_slot = var_vtr_t_db4;
        *var_vtr_t_db5_slot = var_vtr_t_db5;
        *var_vtr_t_db6_slot = var_vtr_t_db6;
        *var_vtr_t_db7_slot = var_vtr_t_db7;
        *var_vtr_t_db8_slot = var_vtr_t_db8;
        *var_vtr_t_db9_slot = var_vtr_t_db9;
        *var_vtr_t_dn0_slot = var_vtr_t_dn0;
        *var_vtr_t_dn1_slot = var_vtr_t_dn1;
        *var_vtr_t_dn10_slot = var_vtr_t_dn10;
        *var_vtr_t_dn11_slot = var_vtr_t_dn11;
        *var_vtr_t_dn12_slot = var_vtr_t_dn12;
        *var_vtr_t_dn13_slot = var_vtr_t_dn13;
        *var_vtr_t_dn14_slot = var_vtr_t_dn14;
        *var_vtr_t_dn15_slot = var_vtr_t_dn15;
        *var_vtr_t_dn2_slot = var_vtr_t_dn2;
        *var_vtr_t_dn3_slot = var_vtr_t_dn3;
        *var_vtr_t_dn4_slot = var_vtr_t_dn4;
        *var_vtr_t_dn5_slot = var_vtr_t_dn5;
        *var_vtr_t_dn6_slot = var_vtr_t_dn6;
        *var_vtr_t_dn7_slot = var_vtr_t_dn7;
        *var_vtr_t_dn8_slot = var_vtr_t_dn8;
        *var_vtr_t_dn9_slot = var_vtr_t_dn9;
    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        var_p1m: f64,
        var_p1m_db0: f64,
        var_p1m_db1: f64,
        var_p1m_db10: f64,
        var_p1m_db11: f64,
        var_p1m_db12: f64,
        var_p1m_db13: f64,
        var_p1m_db14: f64,
        var_p1m_db2: f64,
        var_p1m_db3: f64,
        var_p1m_db4: f64,
        var_p1m_db5: f64,
        var_p1m_db6: f64,
        var_p1m_db7: f64,
        var_p1m_db8: f64,
        var_p1m_db9: f64,
        var_p1m_dn0: f64,
        var_p1m_dn1: f64,
        var_p1m_dn10: f64,
        var_p1m_dn11: f64,
        var_p1m_dn12: f64,
        var_p1m_dn13: f64,
        var_p1m_dn14: f64,
        var_p1m_dn15: f64,
        var_p1m_dn2: f64,
        var_p1m_dn3: f64,
        var_p1m_dn4: f64,
        var_p1m_dn5: f64,
        var_p1m_dn6: f64,
        var_p1m_dn7: f64,
        var_p1m_dn8: f64,
        var_p1m_dn9: f64,
        var_vgd: f64,
        var_vgd_db0: f64,
        var_vgd_db1: f64,
        var_vgd_db10: f64,
        var_vgd_db11: f64,
        var_vgd_db12: f64,
        var_vgd_db13: f64,
        var_vgd_db14: f64,
        var_vgd_db2: f64,
        var_vgd_db3: f64,
        var_vgd_db4: f64,
        var_vgd_db5: f64,
        var_vgd_db6: f64,
        var_vgd_db7: f64,
        var_vgd_db8: f64,
        var_vgd_db9: f64,
        var_vgd_dn0: f64,
        var_vgd_dn1: f64,
        var_vgd_dn10: f64,
        var_vgd_dn11: f64,
        var_vgd_dn12: f64,
        var_vgd_dn13: f64,
        var_vgd_dn14: f64,
        var_vgd_dn15: f64,
        var_vgd_dn2: f64,
        var_vgd_dn3: f64,
        var_vgd_dn4: f64,
        var_vgd_dn5: f64,
        var_vgd_dn6: f64,
        var_vgd_dn7: f64,
        var_vgd_dn8: f64,
        var_vgd_dn9: f64,
        var_vgs: f64,
        var_vgs_db0: f64,
        var_vgs_db1: f64,
        var_vgs_db10: f64,
        var_vgs_db11: f64,
        var_vgs_db12: f64,
        var_vgs_db13: f64,
        var_vgs_db14: f64,
        var_vgs_db2: f64,
        var_vgs_db3: f64,
        var_vgs_db4: f64,
        var_vgs_db5: f64,
        var_vgs_db6: f64,
        var_vgs_db7: f64,
        var_vgs_db8: f64,
        var_vgs_db9: f64,
        var_vgs_dn0: f64,
        var_vgs_dn1: f64,
        var_vgs_dn10: f64,
        var_vgs_dn11: f64,
        var_vgs_dn12: f64,
        var_vgs_dn13: f64,
        var_vgs_dn14: f64,
        var_vgs_dn15: f64,
        var_vgs_dn2: f64,
        var_vgs_dn3: f64,
        var_vgs_dn4: f64,
        var_vgs_dn5: f64,
        var_vgs_dn6: f64,
        var_vgs_dn7: f64,
        var_vgs_dn8: f64,
        var_vgs_dn9: f64,
        var_vpkm: f64,
        var_vpkm_db0: f64,
        var_vpkm_db1: f64,
        var_vpkm_db10: f64,
        var_vpkm_db11: f64,
        var_vpkm_db12: f64,
        var_vpkm_db13: f64,
        var_vpkm_db14: f64,
        var_vpkm_db2: f64,
        var_vpkm_db3: f64,
        var_vpkm_db4: f64,
        var_vpkm_db5: f64,
        var_vpkm_db6: f64,
        var_vpkm_db7: f64,
        var_vpkm_db8: f64,
        var_vpkm_db9: f64,
        var_vpkm_dn0: f64,
        var_vpkm_dn1: f64,
        var_vpkm_dn10: f64,
        var_vpkm_dn11: f64,
        var_vpkm_dn12: f64,
        var_vpkm_dn13: f64,
        var_vpkm_dn14: f64,
        var_vpkm_dn15: f64,
        var_vpkm_dn2: f64,
        var_vpkm_dn3: f64,
        var_vpkm_dn4: f64,
        var_vpkm_dn5: f64,
        var_vpkm_dn6: f64,
        var_vpkm_dn7: f64,
        var_vpkm_dn8: f64,
        var_vpkm_dn9: f64,
        var_guard5_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_guard8_slot: &mut f64,
        var_psi_slot: &mut f64,
        var_psi_db0_slot: &mut f64,
        var_psi_db1_slot: &mut f64,
        var_psi_db10_slot: &mut f64,
        var_psi_db11_slot: &mut f64,
        var_psi_db12_slot: &mut f64,
        var_psi_db13_slot: &mut f64,
        var_psi_db14_slot: &mut f64,
        var_psi_db2_slot: &mut f64,
        var_psi_db3_slot: &mut f64,
        var_psi_db4_slot: &mut f64,
        var_psi_db5_slot: &mut f64,
        var_psi_db6_slot: &mut f64,
        var_psi_db7_slot: &mut f64,
        var_psi_db8_slot: &mut f64,
        var_psi_db9_slot: &mut f64,
        var_psi_dn0_slot: &mut f64,
        var_psi_dn1_slot: &mut f64,
        var_psi_dn10_slot: &mut f64,
        var_psi_dn11_slot: &mut f64,
        var_psi_dn12_slot: &mut f64,
        var_psi_dn13_slot: &mut f64,
        var_psi_dn14_slot: &mut f64,
        var_psi_dn15_slot: &mut f64,
        var_psi_dn2_slot: &mut f64,
        var_psi_dn3_slot: &mut f64,
        var_psi_dn4_slot: &mut f64,
        var_psi_dn5_slot: &mut f64,
        var_psi_dn6_slot: &mut f64,
        var_psi_dn7_slot: &mut f64,
        var_psi_dn8_slot: &mut f64,
        var_psi_dn9_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_db0_slot: &mut f64,
        var_t0_db1_slot: &mut f64,
        var_t0_db10_slot: &mut f64,
        var_t0_db11_slot: &mut f64,
        var_t0_db12_slot: &mut f64,
        var_t0_db13_slot: &mut f64,
        var_t0_db14_slot: &mut f64,
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
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_db0_slot: &mut f64,
        var_t1_db1_slot: &mut f64,
        var_t1_db10_slot: &mut f64,
        var_t1_db11_slot: &mut f64,
        var_t1_db12_slot: &mut f64,
        var_t1_db13_slot: &mut f64,
        var_t1_db14_slot: &mut f64,
        var_t1_db2_slot: &mut f64,
        var_t1_db3_slot: &mut f64,
        var_t1_db4_slot: &mut f64,
        var_t1_db5_slot: &mut f64,
        var_t1_db6_slot: &mut f64,
        var_t1_db7_slot: &mut f64,
        var_t1_db8_slot: &mut f64,
        var_t1_db9_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn15_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_db0_slot: &mut f64,
        var_t2_db1_slot: &mut f64,
        var_t2_db10_slot: &mut f64,
        var_t2_db11_slot: &mut f64,
        var_t2_db12_slot: &mut f64,
        var_t2_db13_slot: &mut f64,
        var_t2_db14_slot: &mut f64,
        var_t2_db2_slot: &mut f64,
        var_t2_db3_slot: &mut f64,
        var_t2_db4_slot: &mut f64,
        var_t2_db5_slot: &mut f64,
        var_t2_db6_slot: &mut f64,
        var_t2_db7_slot: &mut f64,
        var_t2_db8_slot: &mut f64,
        var_t2_db9_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn1_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn15_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_tanh_psi_slot: &mut f64,
        var_tanh_psi1_slot: &mut f64,
        var_tanh_psi1_db0_slot: &mut f64,
        var_tanh_psi1_db1_slot: &mut f64,
        var_tanh_psi1_db10_slot: &mut f64,
        var_tanh_psi1_db11_slot: &mut f64,
        var_tanh_psi1_db12_slot: &mut f64,
        var_tanh_psi1_db13_slot: &mut f64,
        var_tanh_psi1_db14_slot: &mut f64,
        var_tanh_psi1_db2_slot: &mut f64,
        var_tanh_psi1_db3_slot: &mut f64,
        var_tanh_psi1_db4_slot: &mut f64,
        var_tanh_psi1_db5_slot: &mut f64,
        var_tanh_psi1_db6_slot: &mut f64,
        var_tanh_psi1_db7_slot: &mut f64,
        var_tanh_psi1_db8_slot: &mut f64,
        var_tanh_psi1_db9_slot: &mut f64,
        var_tanh_psi1_dn0_slot: &mut f64,
        var_tanh_psi1_dn1_slot: &mut f64,
        var_tanh_psi1_dn10_slot: &mut f64,
        var_tanh_psi1_dn11_slot: &mut f64,
        var_tanh_psi1_dn12_slot: &mut f64,
        var_tanh_psi1_dn13_slot: &mut f64,
        var_tanh_psi1_dn14_slot: &mut f64,
        var_tanh_psi1_dn15_slot: &mut f64,
        var_tanh_psi1_dn2_slot: &mut f64,
        var_tanh_psi1_dn3_slot: &mut f64,
        var_tanh_psi1_dn4_slot: &mut f64,
        var_tanh_psi1_dn5_slot: &mut f64,
        var_tanh_psi1_dn6_slot: &mut f64,
        var_tanh_psi1_dn7_slot: &mut f64,
        var_tanh_psi1_dn8_slot: &mut f64,
        var_tanh_psi1_dn9_slot: &mut f64,
        var_tanh_psi_db0_slot: &mut f64,
        var_tanh_psi_db1_slot: &mut f64,
        var_tanh_psi_db10_slot: &mut f64,
        var_tanh_psi_db11_slot: &mut f64,
        var_tanh_psi_db12_slot: &mut f64,
        var_tanh_psi_db13_slot: &mut f64,
        var_tanh_psi_db14_slot: &mut f64,
        var_tanh_psi_db2_slot: &mut f64,
        var_tanh_psi_db3_slot: &mut f64,
        var_tanh_psi_db4_slot: &mut f64,
        var_tanh_psi_db5_slot: &mut f64,
        var_tanh_psi_db6_slot: &mut f64,
        var_tanh_psi_db7_slot: &mut f64,
        var_tanh_psi_db8_slot: &mut f64,
        var_tanh_psi_db9_slot: &mut f64,
        var_tanh_psi_dn0_slot: &mut f64,
        var_tanh_psi_dn1_slot: &mut f64,
        var_tanh_psi_dn10_slot: &mut f64,
        var_tanh_psi_dn11_slot: &mut f64,
        var_tanh_psi_dn12_slot: &mut f64,
        var_tanh_psi_dn13_slot: &mut f64,
        var_tanh_psi_dn14_slot: &mut f64,
        var_tanh_psi_dn15_slot: &mut f64,
        var_tanh_psi_dn2_slot: &mut f64,
        var_tanh_psi_dn3_slot: &mut f64,
        var_tanh_psi_dn4_slot: &mut f64,
        var_tanh_psi_dn5_slot: &mut f64,
        var_tanh_psi_dn6_slot: &mut f64,
        var_tanh_psi_dn7_slot: &mut f64,
        var_tanh_psi_dn8_slot: &mut f64,
        var_tanh_psi_dn9_slot: &mut f64,
    ) {
        let mut var_guard5: f64 = *var_guard5_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_guard8: f64 = *var_guard8_slot;
        let mut var_psi: f64 = *var_psi_slot;
        let mut var_psi_db0: f64 = *var_psi_db0_slot;
        let mut var_psi_db1: f64 = *var_psi_db1_slot;
        let mut var_psi_db10: f64 = *var_psi_db10_slot;
        let mut var_psi_db11: f64 = *var_psi_db11_slot;
        let mut var_psi_db12: f64 = *var_psi_db12_slot;
        let mut var_psi_db13: f64 = *var_psi_db13_slot;
        let mut var_psi_db14: f64 = *var_psi_db14_slot;
        let mut var_psi_db2: f64 = *var_psi_db2_slot;
        let mut var_psi_db3: f64 = *var_psi_db3_slot;
        let mut var_psi_db4: f64 = *var_psi_db4_slot;
        let mut var_psi_db5: f64 = *var_psi_db5_slot;
        let mut var_psi_db6: f64 = *var_psi_db6_slot;
        let mut var_psi_db7: f64 = *var_psi_db7_slot;
        let mut var_psi_db8: f64 = *var_psi_db8_slot;
        let mut var_psi_db9: f64 = *var_psi_db9_slot;
        let mut var_psi_dn0: f64 = *var_psi_dn0_slot;
        let mut var_psi_dn1: f64 = *var_psi_dn1_slot;
        let mut var_psi_dn10: f64 = *var_psi_dn10_slot;
        let mut var_psi_dn11: f64 = *var_psi_dn11_slot;
        let mut var_psi_dn12: f64 = *var_psi_dn12_slot;
        let mut var_psi_dn13: f64 = *var_psi_dn13_slot;
        let mut var_psi_dn14: f64 = *var_psi_dn14_slot;
        let mut var_psi_dn15: f64 = *var_psi_dn15_slot;
        let mut var_psi_dn2: f64 = *var_psi_dn2_slot;
        let mut var_psi_dn3: f64 = *var_psi_dn3_slot;
        let mut var_psi_dn4: f64 = *var_psi_dn4_slot;
        let mut var_psi_dn5: f64 = *var_psi_dn5_slot;
        let mut var_psi_dn6: f64 = *var_psi_dn6_slot;
        let mut var_psi_dn7: f64 = *var_psi_dn7_slot;
        let mut var_psi_dn8: f64 = *var_psi_dn8_slot;
        let mut var_psi_dn9: f64 = *var_psi_dn9_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_db0: f64 = *var_t0_db0_slot;
        let mut var_t0_db1: f64 = *var_t0_db1_slot;
        let mut var_t0_db10: f64 = *var_t0_db10_slot;
        let mut var_t0_db11: f64 = *var_t0_db11_slot;
        let mut var_t0_db12: f64 = *var_t0_db12_slot;
        let mut var_t0_db13: f64 = *var_t0_db13_slot;
        let mut var_t0_db14: f64 = *var_t0_db14_slot;
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
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_db0: f64 = *var_t1_db0_slot;
        let mut var_t1_db1: f64 = *var_t1_db1_slot;
        let mut var_t1_db10: f64 = *var_t1_db10_slot;
        let mut var_t1_db11: f64 = *var_t1_db11_slot;
        let mut var_t1_db12: f64 = *var_t1_db12_slot;
        let mut var_t1_db13: f64 = *var_t1_db13_slot;
        let mut var_t1_db14: f64 = *var_t1_db14_slot;
        let mut var_t1_db2: f64 = *var_t1_db2_slot;
        let mut var_t1_db3: f64 = *var_t1_db3_slot;
        let mut var_t1_db4: f64 = *var_t1_db4_slot;
        let mut var_t1_db5: f64 = *var_t1_db5_slot;
        let mut var_t1_db6: f64 = *var_t1_db6_slot;
        let mut var_t1_db7: f64 = *var_t1_db7_slot;
        let mut var_t1_db8: f64 = *var_t1_db8_slot;
        let mut var_t1_db9: f64 = *var_t1_db9_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn15: f64 = *var_t1_dn15_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_db0: f64 = *var_t2_db0_slot;
        let mut var_t2_db1: f64 = *var_t2_db1_slot;
        let mut var_t2_db10: f64 = *var_t2_db10_slot;
        let mut var_t2_db11: f64 = *var_t2_db11_slot;
        let mut var_t2_db12: f64 = *var_t2_db12_slot;
        let mut var_t2_db13: f64 = *var_t2_db13_slot;
        let mut var_t2_db14: f64 = *var_t2_db14_slot;
        let mut var_t2_db2: f64 = *var_t2_db2_slot;
        let mut var_t2_db3: f64 = *var_t2_db3_slot;
        let mut var_t2_db4: f64 = *var_t2_db4_slot;
        let mut var_t2_db5: f64 = *var_t2_db5_slot;
        let mut var_t2_db6: f64 = *var_t2_db6_slot;
        let mut var_t2_db7: f64 = *var_t2_db7_slot;
        let mut var_t2_db8: f64 = *var_t2_db8_slot;
        let mut var_t2_db9: f64 = *var_t2_db9_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn1: f64 = *var_t2_dn1_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn15: f64 = *var_t2_dn15_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_tanh_psi: f64 = *var_tanh_psi_slot;
        let mut var_tanh_psi1: f64 = *var_tanh_psi1_slot;
        let mut var_tanh_psi1_db0: f64 = *var_tanh_psi1_db0_slot;
        let mut var_tanh_psi1_db1: f64 = *var_tanh_psi1_db1_slot;
        let mut var_tanh_psi1_db10: f64 = *var_tanh_psi1_db10_slot;
        let mut var_tanh_psi1_db11: f64 = *var_tanh_psi1_db11_slot;
        let mut var_tanh_psi1_db12: f64 = *var_tanh_psi1_db12_slot;
        let mut var_tanh_psi1_db13: f64 = *var_tanh_psi1_db13_slot;
        let mut var_tanh_psi1_db14: f64 = *var_tanh_psi1_db14_slot;
        let mut var_tanh_psi1_db2: f64 = *var_tanh_psi1_db2_slot;
        let mut var_tanh_psi1_db3: f64 = *var_tanh_psi1_db3_slot;
        let mut var_tanh_psi1_db4: f64 = *var_tanh_psi1_db4_slot;
        let mut var_tanh_psi1_db5: f64 = *var_tanh_psi1_db5_slot;
        let mut var_tanh_psi1_db6: f64 = *var_tanh_psi1_db6_slot;
        let mut var_tanh_psi1_db7: f64 = *var_tanh_psi1_db7_slot;
        let mut var_tanh_psi1_db8: f64 = *var_tanh_psi1_db8_slot;
        let mut var_tanh_psi1_db9: f64 = *var_tanh_psi1_db9_slot;
        let mut var_tanh_psi1_dn0: f64 = *var_tanh_psi1_dn0_slot;
        let mut var_tanh_psi1_dn1: f64 = *var_tanh_psi1_dn1_slot;
        let mut var_tanh_psi1_dn10: f64 = *var_tanh_psi1_dn10_slot;
        let mut var_tanh_psi1_dn11: f64 = *var_tanh_psi1_dn11_slot;
        let mut var_tanh_psi1_dn12: f64 = *var_tanh_psi1_dn12_slot;
        let mut var_tanh_psi1_dn13: f64 = *var_tanh_psi1_dn13_slot;
        let mut var_tanh_psi1_dn14: f64 = *var_tanh_psi1_dn14_slot;
        let mut var_tanh_psi1_dn15: f64 = *var_tanh_psi1_dn15_slot;
        let mut var_tanh_psi1_dn2: f64 = *var_tanh_psi1_dn2_slot;
        let mut var_tanh_psi1_dn3: f64 = *var_tanh_psi1_dn3_slot;
        let mut var_tanh_psi1_dn4: f64 = *var_tanh_psi1_dn4_slot;
        let mut var_tanh_psi1_dn5: f64 = *var_tanh_psi1_dn5_slot;
        let mut var_tanh_psi1_dn6: f64 = *var_tanh_psi1_dn6_slot;
        let mut var_tanh_psi1_dn7: f64 = *var_tanh_psi1_dn7_slot;
        let mut var_tanh_psi1_dn8: f64 = *var_tanh_psi1_dn8_slot;
        let mut var_tanh_psi1_dn9: f64 = *var_tanh_psi1_dn9_slot;
        let mut var_tanh_psi_db0: f64 = *var_tanh_psi_db0_slot;
        let mut var_tanh_psi_db1: f64 = *var_tanh_psi_db1_slot;
        let mut var_tanh_psi_db10: f64 = *var_tanh_psi_db10_slot;
        let mut var_tanh_psi_db11: f64 = *var_tanh_psi_db11_slot;
        let mut var_tanh_psi_db12: f64 = *var_tanh_psi_db12_slot;
        let mut var_tanh_psi_db13: f64 = *var_tanh_psi_db13_slot;
        let mut var_tanh_psi_db14: f64 = *var_tanh_psi_db14_slot;
        let mut var_tanh_psi_db2: f64 = *var_tanh_psi_db2_slot;
        let mut var_tanh_psi_db3: f64 = *var_tanh_psi_db3_slot;
        let mut var_tanh_psi_db4: f64 = *var_tanh_psi_db4_slot;
        let mut var_tanh_psi_db5: f64 = *var_tanh_psi_db5_slot;
        let mut var_tanh_psi_db6: f64 = *var_tanh_psi_db6_slot;
        let mut var_tanh_psi_db7: f64 = *var_tanh_psi_db7_slot;
        let mut var_tanh_psi_db8: f64 = *var_tanh_psi_db8_slot;
        let mut var_tanh_psi_db9: f64 = *var_tanh_psi_db9_slot;
        let mut var_tanh_psi_dn0: f64 = *var_tanh_psi_dn0_slot;
        let mut var_tanh_psi_dn1: f64 = *var_tanh_psi_dn1_slot;
        let mut var_tanh_psi_dn10: f64 = *var_tanh_psi_dn10_slot;
        let mut var_tanh_psi_dn11: f64 = *var_tanh_psi_dn11_slot;
        let mut var_tanh_psi_dn12: f64 = *var_tanh_psi_dn12_slot;
        let mut var_tanh_psi_dn13: f64 = *var_tanh_psi_dn13_slot;
        let mut var_tanh_psi_dn14: f64 = *var_tanh_psi_dn14_slot;
        let mut var_tanh_psi_dn15: f64 = *var_tanh_psi_dn15_slot;
        let mut var_tanh_psi_dn2: f64 = *var_tanh_psi_dn2_slot;
        let mut var_tanh_psi_dn3: f64 = *var_tanh_psi_dn3_slot;
        let mut var_tanh_psi_dn4: f64 = *var_tanh_psi_dn4_slot;
        let mut var_tanh_psi_dn5: f64 = *var_tanh_psi_dn5_slot;
        let mut var_tanh_psi_dn6: f64 = *var_tanh_psi_dn6_slot;
        let mut var_tanh_psi_dn7: f64 = *var_tanh_psi_dn7_slot;
        let mut var_tanh_psi_dn8: f64 = *var_tanh_psi_dn8_slot;
        let mut var_tanh_psi_dn9: f64 = *var_tanh_psi_dn9_slot;

        let assign550_e868: f64 = (var_p1m * var_t1);
        let assign550_e871: f64 = (p.p12 * var_t2);
        let assign550_e872: f64 = (assign550_e868 + assign550_e871);
        let assign550_e875: f64 = (p.p13 * var_t1);
        let assign550_e877: f64 = (assign550_e875 * var_t2);
        let assign550_e878: f64 = (assign550_e872 + assign550_e877);
        var_psi = assign550_e878;
        var_psi_dn0 = ((((var_p1m_dn0 * var_t1) + (var_p1m * var_t1_dn0)) + (p.p12 * var_t2_dn0)) + (((p.p13 * var_t1_dn0) * var_t2) + (assign550_e875 * var_t2_dn0)));
        var_psi_dn1 = ((((var_p1m_dn1 * var_t1) + (var_p1m * var_t1_dn1)) + (p.p12 * var_t2_dn1)) + (((p.p13 * var_t1_dn1) * var_t2) + (assign550_e875 * var_t2_dn1)));
        var_psi_dn2 = ((((var_p1m_dn2 * var_t1) + (var_p1m * var_t1_dn2)) + (p.p12 * var_t2_dn2)) + (((p.p13 * var_t1_dn2) * var_t2) + (assign550_e875 * var_t2_dn2)));
        var_psi_dn3 = ((((var_p1m_dn3 * var_t1) + (var_p1m * var_t1_dn3)) + (p.p12 * var_t2_dn3)) + (((p.p13 * var_t1_dn3) * var_t2) + (assign550_e875 * var_t2_dn3)));
        var_psi_dn4 = ((((var_p1m_dn4 * var_t1) + (var_p1m * var_t1_dn4)) + (p.p12 * var_t2_dn4)) + (((p.p13 * var_t1_dn4) * var_t2) + (assign550_e875 * var_t2_dn4)));
        var_psi_dn5 = ((((var_p1m_dn5 * var_t1) + (var_p1m * var_t1_dn5)) + (p.p12 * var_t2_dn5)) + (((p.p13 * var_t1_dn5) * var_t2) + (assign550_e875 * var_t2_dn5)));
        var_psi_dn6 = ((((var_p1m_dn6 * var_t1) + (var_p1m * var_t1_dn6)) + (p.p12 * var_t2_dn6)) + (((p.p13 * var_t1_dn6) * var_t2) + (assign550_e875 * var_t2_dn6)));
        var_psi_dn7 = ((((var_p1m_dn7 * var_t1) + (var_p1m * var_t1_dn7)) + (p.p12 * var_t2_dn7)) + (((p.p13 * var_t1_dn7) * var_t2) + (assign550_e875 * var_t2_dn7)));
        var_psi_dn8 = ((((var_p1m_dn8 * var_t1) + (var_p1m * var_t1_dn8)) + (p.p12 * var_t2_dn8)) + (((p.p13 * var_t1_dn8) * var_t2) + (assign550_e875 * var_t2_dn8)));
        var_psi_dn9 = ((((var_p1m_dn9 * var_t1) + (var_p1m * var_t1_dn9)) + (p.p12 * var_t2_dn9)) + (((p.p13 * var_t1_dn9) * var_t2) + (assign550_e875 * var_t2_dn9)));
        var_psi_dn10 = ((((var_p1m_dn10 * var_t1) + (var_p1m * var_t1_dn10)) + (p.p12 * var_t2_dn10)) + (((p.p13 * var_t1_dn10) * var_t2) + (assign550_e875 * var_t2_dn10)));
        var_psi_dn11 = ((((var_p1m_dn11 * var_t1) + (var_p1m * var_t1_dn11)) + (p.p12 * var_t2_dn11)) + (((p.p13 * var_t1_dn11) * var_t2) + (assign550_e875 * var_t2_dn11)));
        var_psi_dn12 = ((((var_p1m_dn12 * var_t1) + (var_p1m * var_t1_dn12)) + (p.p12 * var_t2_dn12)) + (((p.p13 * var_t1_dn12) * var_t2) + (assign550_e875 * var_t2_dn12)));
        var_psi_dn13 = ((((var_p1m_dn13 * var_t1) + (var_p1m * var_t1_dn13)) + (p.p12 * var_t2_dn13)) + (((p.p13 * var_t1_dn13) * var_t2) + (assign550_e875 * var_t2_dn13)));
        var_psi_dn14 = ((((var_p1m_dn14 * var_t1) + (var_p1m * var_t1_dn14)) + (p.p12 * var_t2_dn14)) + (((p.p13 * var_t1_dn14) * var_t2) + (assign550_e875 * var_t2_dn14)));
        var_psi_dn15 = ((((var_p1m_dn15 * var_t1) + (var_p1m * var_t1_dn15)) + (p.p12 * var_t2_dn15)) + (((p.p13 * var_t1_dn15) * var_t2) + (assign550_e875 * var_t2_dn15)));
        var_psi_db0 = ((((var_p1m_db0 * var_t1) + (var_p1m * var_t1_db0)) + (p.p12 * var_t2_db0)) + (((p.p13 * var_t1_db0) * var_t2) + (assign550_e875 * var_t2_db0)));
        var_psi_db1 = ((((var_p1m_db1 * var_t1) + (var_p1m * var_t1_db1)) + (p.p12 * var_t2_db1)) + (((p.p13 * var_t1_db1) * var_t2) + (assign550_e875 * var_t2_db1)));
        var_psi_db2 = ((((var_p1m_db2 * var_t1) + (var_p1m * var_t1_db2)) + (p.p12 * var_t2_db2)) + (((p.p13 * var_t1_db2) * var_t2) + (assign550_e875 * var_t2_db2)));
        var_psi_db3 = ((((var_p1m_db3 * var_t1) + (var_p1m * var_t1_db3)) + (p.p12 * var_t2_db3)) + (((p.p13 * var_t1_db3) * var_t2) + (assign550_e875 * var_t2_db3)));
        var_psi_db4 = ((((var_p1m_db4 * var_t1) + (var_p1m * var_t1_db4)) + (p.p12 * var_t2_db4)) + (((p.p13 * var_t1_db4) * var_t2) + (assign550_e875 * var_t2_db4)));
        var_psi_db5 = ((((var_p1m_db5 * var_t1) + (var_p1m * var_t1_db5)) + (p.p12 * var_t2_db5)) + (((p.p13 * var_t1_db5) * var_t2) + (assign550_e875 * var_t2_db5)));
        var_psi_db6 = ((((var_p1m_db6 * var_t1) + (var_p1m * var_t1_db6)) + (p.p12 * var_t2_db6)) + (((p.p13 * var_t1_db6) * var_t2) + (assign550_e875 * var_t2_db6)));
        var_psi_db7 = ((((var_p1m_db7 * var_t1) + (var_p1m * var_t1_db7)) + (p.p12 * var_t2_db7)) + (((p.p13 * var_t1_db7) * var_t2) + (assign550_e875 * var_t2_db7)));
        var_psi_db8 = ((((var_p1m_db8 * var_t1) + (var_p1m * var_t1_db8)) + (p.p12 * var_t2_db8)) + (((p.p13 * var_t1_db8) * var_t2) + (assign550_e875 * var_t2_db8)));
        var_psi_db9 = ((((var_p1m_db9 * var_t1) + (var_p1m * var_t1_db9)) + (p.p12 * var_t2_db9)) + (((p.p13 * var_t1_db9) * var_t2) + (assign550_e875 * var_t2_db9)));
        var_psi_db10 = ((((var_p1m_db10 * var_t1) + (var_p1m * var_t1_db10)) + (p.p12 * var_t2_db10)) + (((p.p13 * var_t1_db10) * var_t2) + (assign550_e875 * var_t2_db10)));
        var_psi_db11 = ((((var_p1m_db11 * var_t1) + (var_p1m * var_t1_db11)) + (p.p12 * var_t2_db11)) + (((p.p13 * var_t1_db11) * var_t2) + (assign550_e875 * var_t2_db11)));
        var_psi_db12 = ((((var_p1m_db12 * var_t1) + (var_p1m * var_t1_db12)) + (p.p12 * var_t2_db12)) + (((p.p13 * var_t1_db12) * var_t2) + (assign550_e875 * var_t2_db12)));
        var_psi_db13 = ((((var_p1m_db13 * var_t1) + (var_p1m * var_t1_db13)) + (p.p12 * var_t2_db13)) + (((p.p13 * var_t1_db13) * var_t2) + (assign550_e875 * var_t2_db13)));
        var_psi_db14 = ((((var_p1m_db14 * var_t1) + (var_p1m * var_t1_db14)) + (p.p12 * var_t2_db14)) + (((p.p13 * var_t1_db14) * var_t2) + (assign550_e875 * var_t2_db14)));

        let assign560_e881: f64 = (var_psi).tanh();
        let assign560_e882: f64 = (1.0 + assign560_e881);
        var_tanh_psi = assign560_e882;
        var_tanh_psi_dn0 = (var_psi_dn0 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_dn1 = (var_psi_dn1 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_dn2 = (var_psi_dn2 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_dn3 = (var_psi_dn3 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_dn4 = (var_psi_dn4 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_dn5 = (var_psi_dn5 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_dn6 = (var_psi_dn6 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_dn7 = (var_psi_dn7 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_dn8 = (var_psi_dn8 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_dn9 = (var_psi_dn9 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_dn10 = (var_psi_dn10 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_dn11 = (var_psi_dn11 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_dn12 = (var_psi_dn12 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_dn13 = (var_psi_dn13 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_dn14 = (var_psi_dn14 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_dn15 = (var_psi_dn15 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_db0 = (var_psi_db0 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_db1 = (var_psi_db1 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_db2 = (var_psi_db2 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_db3 = (var_psi_db3 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_db4 = (var_psi_db4 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_db5 = (var_psi_db5 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_db6 = (var_psi_db6 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_db7 = (var_psi_db7 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_db8 = (var_psi_db8 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_db9 = (var_psi_db9 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_db10 = (var_psi_db10 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_db11 = (var_psi_db11 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_db12 = (var_psi_db12 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_db13 = (var_psi_db13 / ((var_psi).cosh() * (var_psi).cosh()));
        var_tanh_psi_db14 = (var_psi_db14 / ((var_psi).cosh() * (var_psi).cosh()));

        let assign570_e886: f64 = { let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign570_e888: f64 = (-var_psi);
        let assign570_e889: f64 = { let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign570_e890: f64 = (assign570_e886 - assign570_e889);
        let assign570_e891: f64 = (0.5 * assign570_e890);
        let assign570_e892: f64 = (assign570_e891).tanh();
        let assign570_e893: f64 = (1.0 + assign570_e892);
        var_tanh_psi1 = assign570_e893;
        var_tanh_psi1_dn0 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn0) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn0)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_dn1 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn1) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn1)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_dn2 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn2) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn2)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_dn3 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn3) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn3)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_dn4 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn4) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn4)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_dn5 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn5) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn5)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_dn6 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn6) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn6)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_dn7 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn7) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn7)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_dn8 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn8) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn8)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_dn9 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn9) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn9)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_dn10 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn10) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn10)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_dn11 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn11) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn11)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_dn12 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn12) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn12)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_dn13 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn13) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn13)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_dn14 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn14) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn14)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_dn15 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn15) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn15)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_db0 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db0) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db0)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_db1 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db1) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db1)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_db2 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db2) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db2)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_db3 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db3) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db3)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_db4 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db4) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db4)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_db5 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db5) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db5)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_db6 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db6) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db6)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_db7 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db7) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db7)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_db8 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db8) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db8)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_db9 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db9) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db9)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_db10 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db10) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db10)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_db11 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db11) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db11)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_db12 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db12) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db12)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_db13 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db13) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db13)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        var_tanh_psi1_db14 = ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db14) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db14)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));

        let assign600_e905: f64 = if p.p4 == 0.0 { 1.0 } else { 0.0 };
        var_guard5 = assign600_e905;

        let assign610_e908: f64 = if p.p4 == 1.0 { 1.0 } else { 0.0 };
        var_guard6 = assign610_e908;

        let assign620_e911: f64 = if p.p4 == 2.0 { 1.0 } else { 0.0 };
        var_guard7 = assign620_e911;

        let assign630_e914: f64 = if p.p4 == 3.0 { 1.0 } else { 0.0 };
        var_guard8 = assign630_e914;

        let (assign650_e944, assign650_e944_d_n0, assign650_e944_d_n1, assign650_e944_d_n2, assign650_e944_d_n3, assign650_e944_d_n4, assign650_e944_d_n5, assign650_e944_d_n6, assign650_e944_d_n7, assign650_e944_d_n8, assign650_e944_d_n9, assign650_e944_d_n10, assign650_e944_d_n11, assign650_e944_d_n12, assign650_e944_d_n13, assign650_e944_d_n14, assign650_e944_d_n15, assign650_e944_d_b0, assign650_e944_d_b1, assign650_e944_d_b2, assign650_e944_d_b3, assign650_e944_d_b4, assign650_e944_d_b5, assign650_e944_d_b6, assign650_e944_d_b7, assign650_e944_d_b8, assign650_e944_d_b9, assign650_e944_d_b10, assign650_e944_d_b11, assign650_e944_d_b12, assign650_e944_d_b13, assign650_e944_d_b14,) = {
    if ((var_guard6 != 0.0) && (var_guard5 == 0.0)) {
        let assign650_e942: f64 = (var_vgd - var_vpkm);
        (assign650_e942, (var_vgd_dn0 - var_vpkm_dn0), (var_vgd_dn1 - var_vpkm_dn1), (var_vgd_dn2 - var_vpkm_dn2), (var_vgd_dn3 - var_vpkm_dn3), (var_vgd_dn4 - var_vpkm_dn4), (var_vgd_dn5 - var_vpkm_dn5), (var_vgd_dn6 - var_vpkm_dn6), (var_vgd_dn7 - var_vpkm_dn7), (var_vgd_dn8 - var_vpkm_dn8), (var_vgd_dn9 - var_vpkm_dn9), (var_vgd_dn10 - var_vpkm_dn10), (var_vgd_dn11 - var_vpkm_dn11), (var_vgd_dn12 - var_vpkm_dn12), (var_vgd_dn13 - var_vpkm_dn13), (var_vgd_dn14 - var_vpkm_dn14), (var_vgd_dn15 - var_vpkm_dn15), (var_vgd_db0 - var_vpkm_db0), (var_vgd_db1 - var_vpkm_db1), (var_vgd_db2 - var_vpkm_db2), (var_vgd_db3 - var_vpkm_db3), (var_vgd_db4 - var_vpkm_db4), (var_vgd_db5 - var_vpkm_db5), (var_vgd_db6 - var_vpkm_db6), (var_vgd_db7 - var_vpkm_db7), (var_vgd_db8 - var_vpkm_db8), (var_vgd_db9 - var_vpkm_db9), (var_vgd_db10 - var_vpkm_db10), (var_vgd_db11 - var_vpkm_db11), (var_vgd_db12 - var_vpkm_db12), (var_vgd_db13 - var_vpkm_db13), (var_vgd_db14 - var_vpkm_db14),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn12, var_t0_dn13, var_t0_dn14, var_t0_dn15, var_t0_db0, var_t0_db1, var_t0_db2, var_t0_db3, var_t0_db4, var_t0_db5, var_t0_db6, var_t0_db7, var_t0_db8, var_t0_db9, var_t0_db10, var_t0_db11, var_t0_db12, var_t0_db13, var_t0_db14,)
    }
};
        var_t0 = assign650_e944;
        var_t0_dn0 = assign650_e944_d_n0;
        var_t0_dn1 = assign650_e944_d_n1;
        var_t0_dn2 = assign650_e944_d_n2;
        var_t0_dn3 = assign650_e944_d_n3;
        var_t0_dn4 = assign650_e944_d_n4;
        var_t0_dn5 = assign650_e944_d_n5;
        var_t0_dn6 = assign650_e944_d_n6;
        var_t0_dn7 = assign650_e944_d_n7;
        var_t0_dn8 = assign650_e944_d_n8;
        var_t0_dn9 = assign650_e944_d_n9;
        var_t0_dn10 = assign650_e944_d_n10;
        var_t0_dn11 = assign650_e944_d_n11;
        var_t0_dn12 = assign650_e944_d_n12;
        var_t0_dn13 = assign650_e944_d_n13;
        var_t0_dn14 = assign650_e944_d_n14;
        var_t0_dn15 = assign650_e944_d_n15;
        var_t0_db0 = assign650_e944_d_b0;
        var_t0_db1 = assign650_e944_d_b1;
        var_t0_db2 = assign650_e944_d_b2;
        var_t0_db3 = assign650_e944_d_b3;
        var_t0_db4 = assign650_e944_d_b4;
        var_t0_db5 = assign650_e944_d_b5;
        var_t0_db6 = assign650_e944_d_b6;
        var_t0_db7 = assign650_e944_d_b7;
        var_t0_db8 = assign650_e944_d_b8;
        var_t0_db9 = assign650_e944_d_b9;
        var_t0_db10 = assign650_e944_d_b10;
        var_t0_db11 = assign650_e944_d_b11;
        var_t0_db12 = assign650_e944_d_b12;
        var_t0_db13 = assign650_e944_d_b13;
        var_t0_db14 = assign650_e944_d_b14;

        let (assign660_e953, assign660_e953_d_n0, assign660_e953_d_n1, assign660_e953_d_n2, assign660_e953_d_n3, assign660_e953_d_n4, assign660_e953_d_n5, assign660_e953_d_n6, assign660_e953_d_n7, assign660_e953_d_n8, assign660_e953_d_n9, assign660_e953_d_n10, assign660_e953_d_n11, assign660_e953_d_n12, assign660_e953_d_n13, assign660_e953_d_n14, assign660_e953_d_n15, assign660_e953_d_b0, assign660_e953_d_b1, assign660_e953_d_b2, assign660_e953_d_b3, assign660_e953_d_b4, assign660_e953_d_b5, assign660_e953_d_b6, assign660_e953_d_b7, assign660_e953_d_b8, assign660_e953_d_b9, assign660_e953_d_b10, assign660_e953_d_b11, assign660_e953_d_b12, assign660_e953_d_b13, assign660_e953_d_b14,) = {
    if ((var_guard6 != 0.0) && (var_guard5 == 0.0)) {
        let assign660_e951: f64 = (var_t0 * var_t0);
        (assign660_e951, ((var_t0_dn0 * var_t0) + (var_t0 * var_t0_dn0)), ((var_t0_dn1 * var_t0) + (var_t0 * var_t0_dn1)), ((var_t0_dn2 * var_t0) + (var_t0 * var_t0_dn2)), ((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)), ((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)), ((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)), ((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)), ((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)), ((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)), ((var_t0_dn9 * var_t0) + (var_t0 * var_t0_dn9)), ((var_t0_dn10 * var_t0) + (var_t0 * var_t0_dn10)), ((var_t0_dn11 * var_t0) + (var_t0 * var_t0_dn11)), ((var_t0_dn12 * var_t0) + (var_t0 * var_t0_dn12)), ((var_t0_dn13 * var_t0) + (var_t0 * var_t0_dn13)), ((var_t0_dn14 * var_t0) + (var_t0 * var_t0_dn14)), ((var_t0_dn15 * var_t0) + (var_t0 * var_t0_dn15)), ((var_t0_db0 * var_t0) + (var_t0 * var_t0_db0)), ((var_t0_db1 * var_t0) + (var_t0 * var_t0_db1)), ((var_t0_db2 * var_t0) + (var_t0 * var_t0_db2)), ((var_t0_db3 * var_t0) + (var_t0 * var_t0_db3)), ((var_t0_db4 * var_t0) + (var_t0 * var_t0_db4)), ((var_t0_db5 * var_t0) + (var_t0 * var_t0_db5)), ((var_t0_db6 * var_t0) + (var_t0 * var_t0_db6)), ((var_t0_db7 * var_t0) + (var_t0 * var_t0_db7)), ((var_t0_db8 * var_t0) + (var_t0 * var_t0_db8)), ((var_t0_db9 * var_t0) + (var_t0 * var_t0_db9)), ((var_t0_db10 * var_t0) + (var_t0 * var_t0_db10)), ((var_t0_db11 * var_t0) + (var_t0 * var_t0_db11)), ((var_t0_db12 * var_t0) + (var_t0 * var_t0_db12)), ((var_t0_db13 * var_t0) + (var_t0 * var_t0_db13)), ((var_t0_db14 * var_t0) + (var_t0 * var_t0_db14)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn13, var_t1_dn14, var_t1_dn15, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4, var_t1_db5, var_t1_db6, var_t1_db7, var_t1_db8, var_t1_db9, var_t1_db10, var_t1_db11, var_t1_db12, var_t1_db13, var_t1_db14,)
    }
};
        var_t1 = assign660_e953;
        var_t1_dn0 = assign660_e953_d_n0;
        var_t1_dn1 = assign660_e953_d_n1;
        var_t1_dn2 = assign660_e953_d_n2;
        var_t1_dn3 = assign660_e953_d_n3;
        var_t1_dn4 = assign660_e953_d_n4;
        var_t1_dn5 = assign660_e953_d_n5;
        var_t1_dn6 = assign660_e953_d_n6;
        var_t1_dn7 = assign660_e953_d_n7;
        var_t1_dn8 = assign660_e953_d_n8;
        var_t1_dn9 = assign660_e953_d_n9;
        var_t1_dn10 = assign660_e953_d_n10;
        var_t1_dn11 = assign660_e953_d_n11;
        var_t1_dn12 = assign660_e953_d_n12;
        var_t1_dn13 = assign660_e953_d_n13;
        var_t1_dn14 = assign660_e953_d_n14;
        var_t1_dn15 = assign660_e953_d_n15;
        var_t1_db0 = assign660_e953_d_b0;
        var_t1_db1 = assign660_e953_d_b1;
        var_t1_db2 = assign660_e953_d_b2;
        var_t1_db3 = assign660_e953_d_b3;
        var_t1_db4 = assign660_e953_d_b4;
        var_t1_db5 = assign660_e953_d_b5;
        var_t1_db6 = assign660_e953_d_b6;
        var_t1_db7 = assign660_e953_d_b7;
        var_t1_db8 = assign660_e953_d_b8;
        var_t1_db9 = assign660_e953_d_b9;
        var_t1_db10 = assign660_e953_d_b10;
        var_t1_db11 = assign660_e953_d_b11;
        var_t1_db12 = assign660_e953_d_b12;
        var_t1_db13 = assign660_e953_d_b13;
        var_t1_db14 = assign660_e953_d_b14;

        let (assign670_e962, assign670_e962_d_n0, assign670_e962_d_n1, assign670_e962_d_n2, assign670_e962_d_n3, assign670_e962_d_n4, assign670_e962_d_n5, assign670_e962_d_n6, assign670_e962_d_n7, assign670_e962_d_n8, assign670_e962_d_n9, assign670_e962_d_n10, assign670_e962_d_n11, assign670_e962_d_n12, assign670_e962_d_n13, assign670_e962_d_n14, assign670_e962_d_n15, assign670_e962_d_b0, assign670_e962_d_b1, assign670_e962_d_b2, assign670_e962_d_b3, assign670_e962_d_b4, assign670_e962_d_b5, assign670_e962_d_b6, assign670_e962_d_b7, assign670_e962_d_b8, assign670_e962_d_b9, assign670_e962_d_b10, assign670_e962_d_b11, assign670_e962_d_b12, assign670_e962_d_b13, assign670_e962_d_b14,) = {
    if ((var_guard6 != 0.0) && (var_guard5 == 0.0)) {
        let assign670_e960: f64 = (var_t1 * var_t0);
        (assign670_e960, ((var_t1_dn0 * var_t0) + (var_t1 * var_t0_dn0)), ((var_t1_dn1 * var_t0) + (var_t1 * var_t0_dn1)), ((var_t1_dn2 * var_t0) + (var_t1 * var_t0_dn2)), ((var_t1_dn3 * var_t0) + (var_t1 * var_t0_dn3)), ((var_t1_dn4 * var_t0) + (var_t1 * var_t0_dn4)), ((var_t1_dn5 * var_t0) + (var_t1 * var_t0_dn5)), ((var_t1_dn6 * var_t0) + (var_t1 * var_t0_dn6)), ((var_t1_dn7 * var_t0) + (var_t1 * var_t0_dn7)), ((var_t1_dn8 * var_t0) + (var_t1 * var_t0_dn8)), ((var_t1_dn9 * var_t0) + (var_t1 * var_t0_dn9)), ((var_t1_dn10 * var_t0) + (var_t1 * var_t0_dn10)), ((var_t1_dn11 * var_t0) + (var_t1 * var_t0_dn11)), ((var_t1_dn12 * var_t0) + (var_t1 * var_t0_dn12)), ((var_t1_dn13 * var_t0) + (var_t1 * var_t0_dn13)), ((var_t1_dn14 * var_t0) + (var_t1 * var_t0_dn14)), ((var_t1_dn15 * var_t0) + (var_t1 * var_t0_dn15)), ((var_t1_db0 * var_t0) + (var_t1 * var_t0_db0)), ((var_t1_db1 * var_t0) + (var_t1 * var_t0_db1)), ((var_t1_db2 * var_t0) + (var_t1 * var_t0_db2)), ((var_t1_db3 * var_t0) + (var_t1 * var_t0_db3)), ((var_t1_db4 * var_t0) + (var_t1 * var_t0_db4)), ((var_t1_db5 * var_t0) + (var_t1 * var_t0_db5)), ((var_t1_db6 * var_t0) + (var_t1 * var_t0_db6)), ((var_t1_db7 * var_t0) + (var_t1 * var_t0_db7)), ((var_t1_db8 * var_t0) + (var_t1 * var_t0_db8)), ((var_t1_db9 * var_t0) + (var_t1 * var_t0_db9)), ((var_t1_db10 * var_t0) + (var_t1 * var_t0_db10)), ((var_t1_db11 * var_t0) + (var_t1 * var_t0_db11)), ((var_t1_db12 * var_t0) + (var_t1 * var_t0_db12)), ((var_t1_db13 * var_t0) + (var_t1 * var_t0_db13)), ((var_t1_db14 * var_t0) + (var_t1 * var_t0_db14)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn1, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn13, var_t2_dn14, var_t2_dn15, var_t2_db0, var_t2_db1, var_t2_db2, var_t2_db3, var_t2_db4, var_t2_db5, var_t2_db6, var_t2_db7, var_t2_db8, var_t2_db9, var_t2_db10, var_t2_db11, var_t2_db12, var_t2_db13, var_t2_db14,)
    }
};
        var_t2 = assign670_e962;
        var_t2_dn0 = assign670_e962_d_n0;
        var_t2_dn1 = assign670_e962_d_n1;
        var_t2_dn2 = assign670_e962_d_n2;
        var_t2_dn3 = assign670_e962_d_n3;
        var_t2_dn4 = assign670_e962_d_n4;
        var_t2_dn5 = assign670_e962_d_n5;
        var_t2_dn6 = assign670_e962_d_n6;
        var_t2_dn7 = assign670_e962_d_n7;
        var_t2_dn8 = assign670_e962_d_n8;
        var_t2_dn9 = assign670_e962_d_n9;
        var_t2_dn10 = assign670_e962_d_n10;
        var_t2_dn11 = assign670_e962_d_n11;
        var_t2_dn12 = assign670_e962_d_n12;
        var_t2_dn13 = assign670_e962_d_n13;
        var_t2_dn14 = assign670_e962_d_n14;
        var_t2_dn15 = assign670_e962_d_n15;
        var_t2_db0 = assign670_e962_d_b0;
        var_t2_db1 = assign670_e962_d_b1;
        var_t2_db2 = assign670_e962_d_b2;
        var_t2_db3 = assign670_e962_d_b3;
        var_t2_db4 = assign670_e962_d_b4;
        var_t2_db5 = assign670_e962_d_b5;
        var_t2_db6 = assign670_e962_d_b6;
        var_t2_db7 = assign670_e962_d_b7;
        var_t2_db8 = assign670_e962_d_b8;
        var_t2_db9 = assign670_e962_d_b9;
        var_t2_db10 = assign670_e962_d_b10;
        var_t2_db11 = assign670_e962_d_b11;
        var_t2_db12 = assign670_e962_d_b12;
        var_t2_db13 = assign670_e962_d_b13;
        var_t2_db14 = assign670_e962_d_b14;

        let (assign770_e1099, assign770_e1099_d_n0, assign770_e1099_d_n1, assign770_e1099_d_n2, assign770_e1099_d_n3, assign770_e1099_d_n4, assign770_e1099_d_n5, assign770_e1099_d_n6, assign770_e1099_d_n7, assign770_e1099_d_n8, assign770_e1099_d_n9, assign770_e1099_d_n10, assign770_e1099_d_n11, assign770_e1099_d_n12, assign770_e1099_d_n13, assign770_e1099_d_n14, assign770_e1099_d_n15, assign770_e1099_d_b0, assign770_e1099_d_b1, assign770_e1099_d_b2, assign770_e1099_d_b3, assign770_e1099_d_b4, assign770_e1099_d_b5, assign770_e1099_d_b6, assign770_e1099_d_b7, assign770_e1099_d_b8, assign770_e1099_d_b9, assign770_e1099_d_b10, assign770_e1099_d_b11, assign770_e1099_d_b12, assign770_e1099_d_b13, assign770_e1099_d_b14,) = {
    if ((var_guard7 != 0.0) && (!((var_guard5 != 0.0) || (var_guard6 != 0.0)))) {
        let assign770_e1097: f64 = (var_vgs - var_vpkm);
        (assign770_e1097, (var_vgs_dn0 - var_vpkm_dn0), (var_vgs_dn1 - var_vpkm_dn1), (var_vgs_dn2 - var_vpkm_dn2), (var_vgs_dn3 - var_vpkm_dn3), (var_vgs_dn4 - var_vpkm_dn4), (var_vgs_dn5 - var_vpkm_dn5), (var_vgs_dn6 - var_vpkm_dn6), (var_vgs_dn7 - var_vpkm_dn7), (var_vgs_dn8 - var_vpkm_dn8), (var_vgs_dn9 - var_vpkm_dn9), (var_vgs_dn10 - var_vpkm_dn10), (var_vgs_dn11 - var_vpkm_dn11), (var_vgs_dn12 - var_vpkm_dn12), (var_vgs_dn13 - var_vpkm_dn13), (var_vgs_dn14 - var_vpkm_dn14), (var_vgs_dn15 - var_vpkm_dn15), (var_vgs_db0 - var_vpkm_db0), (var_vgs_db1 - var_vpkm_db1), (var_vgs_db2 - var_vpkm_db2), (var_vgs_db3 - var_vpkm_db3), (var_vgs_db4 - var_vpkm_db4), (var_vgs_db5 - var_vpkm_db5), (var_vgs_db6 - var_vpkm_db6), (var_vgs_db7 - var_vpkm_db7), (var_vgs_db8 - var_vpkm_db8), (var_vgs_db9 - var_vpkm_db9), (var_vgs_db10 - var_vpkm_db10), (var_vgs_db11 - var_vpkm_db11), (var_vgs_db12 - var_vpkm_db12), (var_vgs_db13 - var_vpkm_db13), (var_vgs_db14 - var_vpkm_db14),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn12, var_t0_dn13, var_t0_dn14, var_t0_dn15, var_t0_db0, var_t0_db1, var_t0_db2, var_t0_db3, var_t0_db4, var_t0_db5, var_t0_db6, var_t0_db7, var_t0_db8, var_t0_db9, var_t0_db10, var_t0_db11, var_t0_db12, var_t0_db13, var_t0_db14,)
    }
};
        var_t0 = assign770_e1099;
        var_t0_dn0 = assign770_e1099_d_n0;
        var_t0_dn1 = assign770_e1099_d_n1;
        var_t0_dn2 = assign770_e1099_d_n2;
        var_t0_dn3 = assign770_e1099_d_n3;
        var_t0_dn4 = assign770_e1099_d_n4;
        var_t0_dn5 = assign770_e1099_d_n5;
        var_t0_dn6 = assign770_e1099_d_n6;
        var_t0_dn7 = assign770_e1099_d_n7;
        var_t0_dn8 = assign770_e1099_d_n8;
        var_t0_dn9 = assign770_e1099_d_n9;
        var_t0_dn10 = assign770_e1099_d_n10;
        var_t0_dn11 = assign770_e1099_d_n11;
        var_t0_dn12 = assign770_e1099_d_n12;
        var_t0_dn13 = assign770_e1099_d_n13;
        var_t0_dn14 = assign770_e1099_d_n14;
        var_t0_dn15 = assign770_e1099_d_n15;
        var_t0_db0 = assign770_e1099_d_b0;
        var_t0_db1 = assign770_e1099_d_b1;
        var_t0_db2 = assign770_e1099_d_b2;
        var_t0_db3 = assign770_e1099_d_b3;
        var_t0_db4 = assign770_e1099_d_b4;
        var_t0_db5 = assign770_e1099_d_b5;
        var_t0_db6 = assign770_e1099_d_b6;
        var_t0_db7 = assign770_e1099_d_b7;
        var_t0_db8 = assign770_e1099_d_b8;
        var_t0_db9 = assign770_e1099_d_b9;
        var_t0_db10 = assign770_e1099_d_b10;
        var_t0_db11 = assign770_e1099_d_b11;
        var_t0_db12 = assign770_e1099_d_b12;
        var_t0_db13 = assign770_e1099_d_b13;
        var_t0_db14 = assign770_e1099_d_b14;

        let (assign780_e1110, assign780_e1110_d_n0, assign780_e1110_d_n1, assign780_e1110_d_n2, assign780_e1110_d_n3, assign780_e1110_d_n4, assign780_e1110_d_n5, assign780_e1110_d_n6, assign780_e1110_d_n7, assign780_e1110_d_n8, assign780_e1110_d_n9, assign780_e1110_d_n10, assign780_e1110_d_n11, assign780_e1110_d_n12, assign780_e1110_d_n13, assign780_e1110_d_n14, assign780_e1110_d_n15, assign780_e1110_d_b0, assign780_e1110_d_b1, assign780_e1110_d_b2, assign780_e1110_d_b3, assign780_e1110_d_b4, assign780_e1110_d_b5, assign780_e1110_d_b6, assign780_e1110_d_b7, assign780_e1110_d_b8, assign780_e1110_d_b9, assign780_e1110_d_b10, assign780_e1110_d_b11, assign780_e1110_d_b12, assign780_e1110_d_b13, assign780_e1110_d_b14,) = {
    if ((var_guard7 != 0.0) && (!((var_guard5 != 0.0) || (var_guard6 != 0.0)))) {
        let assign780_e1108: f64 = (var_t0 * var_t0);
        (assign780_e1108, ((var_t0_dn0 * var_t0) + (var_t0 * var_t0_dn0)), ((var_t0_dn1 * var_t0) + (var_t0 * var_t0_dn1)), ((var_t0_dn2 * var_t0) + (var_t0 * var_t0_dn2)), ((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)), ((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)), ((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)), ((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)), ((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)), ((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)), ((var_t0_dn9 * var_t0) + (var_t0 * var_t0_dn9)), ((var_t0_dn10 * var_t0) + (var_t0 * var_t0_dn10)), ((var_t0_dn11 * var_t0) + (var_t0 * var_t0_dn11)), ((var_t0_dn12 * var_t0) + (var_t0 * var_t0_dn12)), ((var_t0_dn13 * var_t0) + (var_t0 * var_t0_dn13)), ((var_t0_dn14 * var_t0) + (var_t0 * var_t0_dn14)), ((var_t0_dn15 * var_t0) + (var_t0 * var_t0_dn15)), ((var_t0_db0 * var_t0) + (var_t0 * var_t0_db0)), ((var_t0_db1 * var_t0) + (var_t0 * var_t0_db1)), ((var_t0_db2 * var_t0) + (var_t0 * var_t0_db2)), ((var_t0_db3 * var_t0) + (var_t0 * var_t0_db3)), ((var_t0_db4 * var_t0) + (var_t0 * var_t0_db4)), ((var_t0_db5 * var_t0) + (var_t0 * var_t0_db5)), ((var_t0_db6 * var_t0) + (var_t0 * var_t0_db6)), ((var_t0_db7 * var_t0) + (var_t0 * var_t0_db7)), ((var_t0_db8 * var_t0) + (var_t0 * var_t0_db8)), ((var_t0_db9 * var_t0) + (var_t0 * var_t0_db9)), ((var_t0_db10 * var_t0) + (var_t0 * var_t0_db10)), ((var_t0_db11 * var_t0) + (var_t0 * var_t0_db11)), ((var_t0_db12 * var_t0) + (var_t0 * var_t0_db12)), ((var_t0_db13 * var_t0) + (var_t0 * var_t0_db13)), ((var_t0_db14 * var_t0) + (var_t0 * var_t0_db14)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn13, var_t1_dn14, var_t1_dn15, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4, var_t1_db5, var_t1_db6, var_t1_db7, var_t1_db8, var_t1_db9, var_t1_db10, var_t1_db11, var_t1_db12, var_t1_db13, var_t1_db14,)
    }
};
        var_t1 = assign780_e1110;
        var_t1_dn0 = assign780_e1110_d_n0;
        var_t1_dn1 = assign780_e1110_d_n1;
        var_t1_dn2 = assign780_e1110_d_n2;
        var_t1_dn3 = assign780_e1110_d_n3;
        var_t1_dn4 = assign780_e1110_d_n4;
        var_t1_dn5 = assign780_e1110_d_n5;
        var_t1_dn6 = assign780_e1110_d_n6;
        var_t1_dn7 = assign780_e1110_d_n7;
        var_t1_dn8 = assign780_e1110_d_n8;
        var_t1_dn9 = assign780_e1110_d_n9;
        var_t1_dn10 = assign780_e1110_d_n10;
        var_t1_dn11 = assign780_e1110_d_n11;
        var_t1_dn12 = assign780_e1110_d_n12;
        var_t1_dn13 = assign780_e1110_d_n13;
        var_t1_dn14 = assign780_e1110_d_n14;
        var_t1_dn15 = assign780_e1110_d_n15;
        var_t1_db0 = assign780_e1110_d_b0;
        var_t1_db1 = assign780_e1110_d_b1;
        var_t1_db2 = assign780_e1110_d_b2;
        var_t1_db3 = assign780_e1110_d_b3;
        var_t1_db4 = assign780_e1110_d_b4;
        var_t1_db5 = assign780_e1110_d_b5;
        var_t1_db6 = assign780_e1110_d_b6;
        var_t1_db7 = assign780_e1110_d_b7;
        var_t1_db8 = assign780_e1110_d_b8;
        var_t1_db9 = assign780_e1110_d_b9;
        var_t1_db10 = assign780_e1110_d_b10;
        var_t1_db11 = assign780_e1110_d_b11;
        var_t1_db12 = assign780_e1110_d_b12;
        var_t1_db13 = assign780_e1110_d_b13;
        var_t1_db14 = assign780_e1110_d_b14;

        let (assign790_e1131, assign790_e1131_d_n0, assign790_e1131_d_n1, assign790_e1131_d_n2, assign790_e1131_d_n3, assign790_e1131_d_n4, assign790_e1131_d_n5, assign790_e1131_d_n6, assign790_e1131_d_n7, assign790_e1131_d_n8, assign790_e1131_d_n9, assign790_e1131_d_n10, assign790_e1131_d_n11, assign790_e1131_d_n12, assign790_e1131_d_n13, assign790_e1131_d_n14, assign790_e1131_d_n15, assign790_e1131_d_b0, assign790_e1131_d_b1, assign790_e1131_d_b2, assign790_e1131_d_b3, assign790_e1131_d_b4, assign790_e1131_d_b5, assign790_e1131_d_b6, assign790_e1131_d_b7, assign790_e1131_d_b8, assign790_e1131_d_b9, assign790_e1131_d_b10, assign790_e1131_d_b11, assign790_e1131_d_b12, assign790_e1131_d_b13, assign790_e1131_d_b14,) = {
    if ((var_guard7 != 0.0) && (!((var_guard5 != 0.0) || (var_guard6 != 0.0)))) {
        let assign790_e1121: f64 = (p.p12 * var_t1);
        let assign790_e1122: f64 = (var_t0 + assign790_e1121);
        let assign790_e1125: f64 = (p.p13 * var_t1);
        let assign790_e1127: f64 = (assign790_e1125 * var_t0);
        let assign790_e1128: f64 = (assign790_e1122 + assign790_e1127);
        let assign790_e1129: f64 = (var_p1m * assign790_e1128);
        (assign790_e1129, ((var_p1m_dn0 * assign790_e1128) + (var_p1m * ((var_t0_dn0 + (p.p12 * var_t1_dn0)) + (((p.p13 * var_t1_dn0) * var_t0) + (assign790_e1125 * var_t0_dn0))))), ((var_p1m_dn1 * assign790_e1128) + (var_p1m * ((var_t0_dn1 + (p.p12 * var_t1_dn1)) + (((p.p13 * var_t1_dn1) * var_t0) + (assign790_e1125 * var_t0_dn1))))), ((var_p1m_dn2 * assign790_e1128) + (var_p1m * ((var_t0_dn2 + (p.p12 * var_t1_dn2)) + (((p.p13 * var_t1_dn2) * var_t0) + (assign790_e1125 * var_t0_dn2))))), ((var_p1m_dn3 * assign790_e1128) + (var_p1m * ((var_t0_dn3 + (p.p12 * var_t1_dn3)) + (((p.p13 * var_t1_dn3) * var_t0) + (assign790_e1125 * var_t0_dn3))))), ((var_p1m_dn4 * assign790_e1128) + (var_p1m * ((var_t0_dn4 + (p.p12 * var_t1_dn4)) + (((p.p13 * var_t1_dn4) * var_t0) + (assign790_e1125 * var_t0_dn4))))), ((var_p1m_dn5 * assign790_e1128) + (var_p1m * ((var_t0_dn5 + (p.p12 * var_t1_dn5)) + (((p.p13 * var_t1_dn5) * var_t0) + (assign790_e1125 * var_t0_dn5))))), ((var_p1m_dn6 * assign790_e1128) + (var_p1m * ((var_t0_dn6 + (p.p12 * var_t1_dn6)) + (((p.p13 * var_t1_dn6) * var_t0) + (assign790_e1125 * var_t0_dn6))))), ((var_p1m_dn7 * assign790_e1128) + (var_p1m * ((var_t0_dn7 + (p.p12 * var_t1_dn7)) + (((p.p13 * var_t1_dn7) * var_t0) + (assign790_e1125 * var_t0_dn7))))), ((var_p1m_dn8 * assign790_e1128) + (var_p1m * ((var_t0_dn8 + (p.p12 * var_t1_dn8)) + (((p.p13 * var_t1_dn8) * var_t0) + (assign790_e1125 * var_t0_dn8))))), ((var_p1m_dn9 * assign790_e1128) + (var_p1m * ((var_t0_dn9 + (p.p12 * var_t1_dn9)) + (((p.p13 * var_t1_dn9) * var_t0) + (assign790_e1125 * var_t0_dn9))))), ((var_p1m_dn10 * assign790_e1128) + (var_p1m * ((var_t0_dn10 + (p.p12 * var_t1_dn10)) + (((p.p13 * var_t1_dn10) * var_t0) + (assign790_e1125 * var_t0_dn10))))), ((var_p1m_dn11 * assign790_e1128) + (var_p1m * ((var_t0_dn11 + (p.p12 * var_t1_dn11)) + (((p.p13 * var_t1_dn11) * var_t0) + (assign790_e1125 * var_t0_dn11))))), ((var_p1m_dn12 * assign790_e1128) + (var_p1m * ((var_t0_dn12 + (p.p12 * var_t1_dn12)) + (((p.p13 * var_t1_dn12) * var_t0) + (assign790_e1125 * var_t0_dn12))))), ((var_p1m_dn13 * assign790_e1128) + (var_p1m * ((var_t0_dn13 + (p.p12 * var_t1_dn13)) + (((p.p13 * var_t1_dn13) * var_t0) + (assign790_e1125 * var_t0_dn13))))), ((var_p1m_dn14 * assign790_e1128) + (var_p1m * ((var_t0_dn14 + (p.p12 * var_t1_dn14)) + (((p.p13 * var_t1_dn14) * var_t0) + (assign790_e1125 * var_t0_dn14))))), ((var_p1m_dn15 * assign790_e1128) + (var_p1m * ((var_t0_dn15 + (p.p12 * var_t1_dn15)) + (((p.p13 * var_t1_dn15) * var_t0) + (assign790_e1125 * var_t0_dn15))))), ((var_p1m_db0 * assign790_e1128) + (var_p1m * ((var_t0_db0 + (p.p12 * var_t1_db0)) + (((p.p13 * var_t1_db0) * var_t0) + (assign790_e1125 * var_t0_db0))))), ((var_p1m_db1 * assign790_e1128) + (var_p1m * ((var_t0_db1 + (p.p12 * var_t1_db1)) + (((p.p13 * var_t1_db1) * var_t0) + (assign790_e1125 * var_t0_db1))))), ((var_p1m_db2 * assign790_e1128) + (var_p1m * ((var_t0_db2 + (p.p12 * var_t1_db2)) + (((p.p13 * var_t1_db2) * var_t0) + (assign790_e1125 * var_t0_db2))))), ((var_p1m_db3 * assign790_e1128) + (var_p1m * ((var_t0_db3 + (p.p12 * var_t1_db3)) + (((p.p13 * var_t1_db3) * var_t0) + (assign790_e1125 * var_t0_db3))))), ((var_p1m_db4 * assign790_e1128) + (var_p1m * ((var_t0_db4 + (p.p12 * var_t1_db4)) + (((p.p13 * var_t1_db4) * var_t0) + (assign790_e1125 * var_t0_db4))))), ((var_p1m_db5 * assign790_e1128) + (var_p1m * ((var_t0_db5 + (p.p12 * var_t1_db5)) + (((p.p13 * var_t1_db5) * var_t0) + (assign790_e1125 * var_t0_db5))))), ((var_p1m_db6 * assign790_e1128) + (var_p1m * ((var_t0_db6 + (p.p12 * var_t1_db6)) + (((p.p13 * var_t1_db6) * var_t0) + (assign790_e1125 * var_t0_db6))))), ((var_p1m_db7 * assign790_e1128) + (var_p1m * ((var_t0_db7 + (p.p12 * var_t1_db7)) + (((p.p13 * var_t1_db7) * var_t0) + (assign790_e1125 * var_t0_db7))))), ((var_p1m_db8 * assign790_e1128) + (var_p1m * ((var_t0_db8 + (p.p12 * var_t1_db8)) + (((p.p13 * var_t1_db8) * var_t0) + (assign790_e1125 * var_t0_db8))))), ((var_p1m_db9 * assign790_e1128) + (var_p1m * ((var_t0_db9 + (p.p12 * var_t1_db9)) + (((p.p13 * var_t1_db9) * var_t0) + (assign790_e1125 * var_t0_db9))))), ((var_p1m_db10 * assign790_e1128) + (var_p1m * ((var_t0_db10 + (p.p12 * var_t1_db10)) + (((p.p13 * var_t1_db10) * var_t0) + (assign790_e1125 * var_t0_db10))))), ((var_p1m_db11 * assign790_e1128) + (var_p1m * ((var_t0_db11 + (p.p12 * var_t1_db11)) + (((p.p13 * var_t1_db11) * var_t0) + (assign790_e1125 * var_t0_db11))))), ((var_p1m_db12 * assign790_e1128) + (var_p1m * ((var_t0_db12 + (p.p12 * var_t1_db12)) + (((p.p13 * var_t1_db12) * var_t0) + (assign790_e1125 * var_t0_db12))))), ((var_p1m_db13 * assign790_e1128) + (var_p1m * ((var_t0_db13 + (p.p12 * var_t1_db13)) + (((p.p13 * var_t1_db13) * var_t0) + (assign790_e1125 * var_t0_db13))))), ((var_p1m_db14 * assign790_e1128) + (var_p1m * ((var_t0_db14 + (p.p12 * var_t1_db14)) + (((p.p13 * var_t1_db14) * var_t0) + (assign790_e1125 * var_t0_db14))))),)
    } else {
        (var_psi, var_psi_dn0, var_psi_dn1, var_psi_dn2, var_psi_dn3, var_psi_dn4, var_psi_dn5, var_psi_dn6, var_psi_dn7, var_psi_dn8, var_psi_dn9, var_psi_dn10, var_psi_dn11, var_psi_dn12, var_psi_dn13, var_psi_dn14, var_psi_dn15, var_psi_db0, var_psi_db1, var_psi_db2, var_psi_db3, var_psi_db4, var_psi_db5, var_psi_db6, var_psi_db7, var_psi_db8, var_psi_db9, var_psi_db10, var_psi_db11, var_psi_db12, var_psi_db13, var_psi_db14,)
    }
};
        var_psi = assign790_e1131;
        var_psi_dn0 = assign790_e1131_d_n0;
        var_psi_dn1 = assign790_e1131_d_n1;
        var_psi_dn2 = assign790_e1131_d_n2;
        var_psi_dn3 = assign790_e1131_d_n3;
        var_psi_dn4 = assign790_e1131_d_n4;
        var_psi_dn5 = assign790_e1131_d_n5;
        var_psi_dn6 = assign790_e1131_d_n6;
        var_psi_dn7 = assign790_e1131_d_n7;
        var_psi_dn8 = assign790_e1131_d_n8;
        var_psi_dn9 = assign790_e1131_d_n9;
        var_psi_dn10 = assign790_e1131_d_n10;
        var_psi_dn11 = assign790_e1131_d_n11;
        var_psi_dn12 = assign790_e1131_d_n12;
        var_psi_dn13 = assign790_e1131_d_n13;
        var_psi_dn14 = assign790_e1131_d_n14;
        var_psi_dn15 = assign790_e1131_d_n15;
        var_psi_db0 = assign790_e1131_d_b0;
        var_psi_db1 = assign790_e1131_d_b1;
        var_psi_db2 = assign790_e1131_d_b2;
        var_psi_db3 = assign790_e1131_d_b3;
        var_psi_db4 = assign790_e1131_d_b4;
        var_psi_db5 = assign790_e1131_d_b5;
        var_psi_db6 = assign790_e1131_d_b6;
        var_psi_db7 = assign790_e1131_d_b7;
        var_psi_db8 = assign790_e1131_d_b8;
        var_psi_db9 = assign790_e1131_d_b9;
        var_psi_db10 = assign790_e1131_d_b10;
        var_psi_db11 = assign790_e1131_d_b11;
        var_psi_db12 = assign790_e1131_d_b12;
        var_psi_db13 = assign790_e1131_d_b13;
        var_psi_db14 = assign790_e1131_d_b14;

        let (assign800_e1150, assign800_e1150_d_n0, assign800_e1150_d_n1, assign800_e1150_d_n2, assign800_e1150_d_n3, assign800_e1150_d_n4, assign800_e1150_d_n5, assign800_e1150_d_n6, assign800_e1150_d_n7, assign800_e1150_d_n8, assign800_e1150_d_n9, assign800_e1150_d_n10, assign800_e1150_d_n11, assign800_e1150_d_n12, assign800_e1150_d_n13, assign800_e1150_d_n14, assign800_e1150_d_n15, assign800_e1150_d_b0, assign800_e1150_d_b1, assign800_e1150_d_b2, assign800_e1150_d_b3, assign800_e1150_d_b4, assign800_e1150_d_b5, assign800_e1150_d_b6, assign800_e1150_d_b7, assign800_e1150_d_b8, assign800_e1150_d_b9, assign800_e1150_d_b10, assign800_e1150_d_b11, assign800_e1150_d_b12, assign800_e1150_d_b13, assign800_e1150_d_b14,) = {
    if ((var_guard7 != 0.0) && (!((var_guard5 != 0.0) || (var_guard6 != 0.0)))) {
        let assign800_e1141: f64 = { let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign800_e1143: f64 = (-var_psi);
        let assign800_e1144: f64 = { let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign800_e1145: f64 = (assign800_e1141 - assign800_e1144);
        let assign800_e1146: f64 = (0.5 * assign800_e1145);
        let assign800_e1147: f64 = (assign800_e1146).tanh();
        let assign800_e1148: f64 = (1.0 + assign800_e1147);
        (assign800_e1148, ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn0) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn0)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn1) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn1)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn2) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn2)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn3) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn3)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn4) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn4)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn5) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn5)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn6) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn6)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn7) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn7)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn8) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn8)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn9) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn9)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn10) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn10)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn11) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn11)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn12) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn12)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn13) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn13)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn14) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn14)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn15) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn15)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db0) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db0)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db1) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db1)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db2) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db2)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db3) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db3)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db4) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db4)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db5) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db5)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db6) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db6)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db7) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db7)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db8) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db8)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db9) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db9)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db10) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db10)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db11) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db11)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db12) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db12)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db13) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db13)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db14) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db14)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())),)
    } else {
        (var_tanh_psi1, var_tanh_psi1_dn0, var_tanh_psi1_dn1, var_tanh_psi1_dn2, var_tanh_psi1_dn3, var_tanh_psi1_dn4, var_tanh_psi1_dn5, var_tanh_psi1_dn6, var_tanh_psi1_dn7, var_tanh_psi1_dn8, var_tanh_psi1_dn9, var_tanh_psi1_dn10, var_tanh_psi1_dn11, var_tanh_psi1_dn12, var_tanh_psi1_dn13, var_tanh_psi1_dn14, var_tanh_psi1_dn15, var_tanh_psi1_db0, var_tanh_psi1_db1, var_tanh_psi1_db2, var_tanh_psi1_db3, var_tanh_psi1_db4, var_tanh_psi1_db5, var_tanh_psi1_db6, var_tanh_psi1_db7, var_tanh_psi1_db8, var_tanh_psi1_db9, var_tanh_psi1_db10, var_tanh_psi1_db11, var_tanh_psi1_db12, var_tanh_psi1_db13, var_tanh_psi1_db14,)
    }
};
        var_tanh_psi1 = assign800_e1150;
        var_tanh_psi1_dn0 = assign800_e1150_d_n0;
        var_tanh_psi1_dn1 = assign800_e1150_d_n1;
        var_tanh_psi1_dn2 = assign800_e1150_d_n2;
        var_tanh_psi1_dn3 = assign800_e1150_d_n3;
        var_tanh_psi1_dn4 = assign800_e1150_d_n4;
        var_tanh_psi1_dn5 = assign800_e1150_d_n5;
        var_tanh_psi1_dn6 = assign800_e1150_d_n6;
        var_tanh_psi1_dn7 = assign800_e1150_d_n7;
        var_tanh_psi1_dn8 = assign800_e1150_d_n8;
        var_tanh_psi1_dn9 = assign800_e1150_d_n9;
        var_tanh_psi1_dn10 = assign800_e1150_d_n10;
        var_tanh_psi1_dn11 = assign800_e1150_d_n11;
        var_tanh_psi1_dn12 = assign800_e1150_d_n12;
        var_tanh_psi1_dn13 = assign800_e1150_d_n13;
        var_tanh_psi1_dn14 = assign800_e1150_d_n14;
        var_tanh_psi1_dn15 = assign800_e1150_d_n15;
        var_tanh_psi1_db0 = assign800_e1150_d_b0;
        var_tanh_psi1_db1 = assign800_e1150_d_b1;
        var_tanh_psi1_db2 = assign800_e1150_d_b2;
        var_tanh_psi1_db3 = assign800_e1150_d_b3;
        var_tanh_psi1_db4 = assign800_e1150_d_b4;
        var_tanh_psi1_db5 = assign800_e1150_d_b5;
        var_tanh_psi1_db6 = assign800_e1150_d_b6;
        var_tanh_psi1_db7 = assign800_e1150_d_b7;
        var_tanh_psi1_db8 = assign800_e1150_d_b8;
        var_tanh_psi1_db9 = assign800_e1150_d_b9;
        var_tanh_psi1_db10 = assign800_e1150_d_b10;
        var_tanh_psi1_db11 = assign800_e1150_d_b11;
        var_tanh_psi1_db12 = assign800_e1150_d_b12;
        var_tanh_psi1_db13 = assign800_e1150_d_b13;
        var_tanh_psi1_db14 = assign800_e1150_d_b14;

        let (assign850_e1227, assign850_e1227_d_n0, assign850_e1227_d_n1, assign850_e1227_d_n2, assign850_e1227_d_n3, assign850_e1227_d_n4, assign850_e1227_d_n5, assign850_e1227_d_n6, assign850_e1227_d_n7, assign850_e1227_d_n8, assign850_e1227_d_n9, assign850_e1227_d_n10, assign850_e1227_d_n11, assign850_e1227_d_n12, assign850_e1227_d_n13, assign850_e1227_d_n14, assign850_e1227_d_n15, assign850_e1227_d_b0, assign850_e1227_d_b1, assign850_e1227_d_b2, assign850_e1227_d_b3, assign850_e1227_d_b4, assign850_e1227_d_b5, assign850_e1227_d_b6, assign850_e1227_d_b7, assign850_e1227_d_b8, assign850_e1227_d_b9, assign850_e1227_d_b10, assign850_e1227_d_b11, assign850_e1227_d_b12, assign850_e1227_d_b13, assign850_e1227_d_b14,) = {
    if ((var_guard8 != 0.0) && (!(((var_guard5 != 0.0) || (var_guard6 != 0.0)) || (var_guard7 != 0.0)))) {
        let assign850_e1225: f64 = (var_vgs - var_vpkm);
        (assign850_e1225, (var_vgs_dn0 - var_vpkm_dn0), (var_vgs_dn1 - var_vpkm_dn1), (var_vgs_dn2 - var_vpkm_dn2), (var_vgs_dn3 - var_vpkm_dn3), (var_vgs_dn4 - var_vpkm_dn4), (var_vgs_dn5 - var_vpkm_dn5), (var_vgs_dn6 - var_vpkm_dn6), (var_vgs_dn7 - var_vpkm_dn7), (var_vgs_dn8 - var_vpkm_dn8), (var_vgs_dn9 - var_vpkm_dn9), (var_vgs_dn10 - var_vpkm_dn10), (var_vgs_dn11 - var_vpkm_dn11), (var_vgs_dn12 - var_vpkm_dn12), (var_vgs_dn13 - var_vpkm_dn13), (var_vgs_dn14 - var_vpkm_dn14), (var_vgs_dn15 - var_vpkm_dn15), (var_vgs_db0 - var_vpkm_db0), (var_vgs_db1 - var_vpkm_db1), (var_vgs_db2 - var_vpkm_db2), (var_vgs_db3 - var_vpkm_db3), (var_vgs_db4 - var_vpkm_db4), (var_vgs_db5 - var_vpkm_db5), (var_vgs_db6 - var_vpkm_db6), (var_vgs_db7 - var_vpkm_db7), (var_vgs_db8 - var_vpkm_db8), (var_vgs_db9 - var_vpkm_db9), (var_vgs_db10 - var_vpkm_db10), (var_vgs_db11 - var_vpkm_db11), (var_vgs_db12 - var_vpkm_db12), (var_vgs_db13 - var_vpkm_db13), (var_vgs_db14 - var_vpkm_db14),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn12, var_t0_dn13, var_t0_dn14, var_t0_dn15, var_t0_db0, var_t0_db1, var_t0_db2, var_t0_db3, var_t0_db4, var_t0_db5, var_t0_db6, var_t0_db7, var_t0_db8, var_t0_db9, var_t0_db10, var_t0_db11, var_t0_db12, var_t0_db13, var_t0_db14,)
    }
};
        var_t0 = assign850_e1227;
        var_t0_dn0 = assign850_e1227_d_n0;
        var_t0_dn1 = assign850_e1227_d_n1;
        var_t0_dn2 = assign850_e1227_d_n2;
        var_t0_dn3 = assign850_e1227_d_n3;
        var_t0_dn4 = assign850_e1227_d_n4;
        var_t0_dn5 = assign850_e1227_d_n5;
        var_t0_dn6 = assign850_e1227_d_n6;
        var_t0_dn7 = assign850_e1227_d_n7;
        var_t0_dn8 = assign850_e1227_d_n8;
        var_t0_dn9 = assign850_e1227_d_n9;
        var_t0_dn10 = assign850_e1227_d_n10;
        var_t0_dn11 = assign850_e1227_d_n11;
        var_t0_dn12 = assign850_e1227_d_n12;
        var_t0_dn13 = assign850_e1227_d_n13;
        var_t0_dn14 = assign850_e1227_d_n14;
        var_t0_dn15 = assign850_e1227_d_n15;
        var_t0_db0 = assign850_e1227_d_b0;
        var_t0_db1 = assign850_e1227_d_b1;
        var_t0_db2 = assign850_e1227_d_b2;
        var_t0_db3 = assign850_e1227_d_b3;
        var_t0_db4 = assign850_e1227_d_b4;
        var_t0_db5 = assign850_e1227_d_b5;
        var_t0_db6 = assign850_e1227_d_b6;
        var_t0_db7 = assign850_e1227_d_b7;
        var_t0_db8 = assign850_e1227_d_b8;
        var_t0_db9 = assign850_e1227_d_b9;
        var_t0_db10 = assign850_e1227_d_b10;
        var_t0_db11 = assign850_e1227_d_b11;
        var_t0_db12 = assign850_e1227_d_b12;
        var_t0_db13 = assign850_e1227_d_b13;
        var_t0_db14 = assign850_e1227_d_b14;

        let (assign860_e1240, assign860_e1240_d_n0, assign860_e1240_d_n1, assign860_e1240_d_n2, assign860_e1240_d_n3, assign860_e1240_d_n4, assign860_e1240_d_n5, assign860_e1240_d_n6, assign860_e1240_d_n7, assign860_e1240_d_n8, assign860_e1240_d_n9, assign860_e1240_d_n10, assign860_e1240_d_n11, assign860_e1240_d_n12, assign860_e1240_d_n13, assign860_e1240_d_n14, assign860_e1240_d_n15, assign860_e1240_d_b0, assign860_e1240_d_b1, assign860_e1240_d_b2, assign860_e1240_d_b3, assign860_e1240_d_b4, assign860_e1240_d_b5, assign860_e1240_d_b6, assign860_e1240_d_b7, assign860_e1240_d_b8, assign860_e1240_d_b9, assign860_e1240_d_b10, assign860_e1240_d_b11, assign860_e1240_d_b12, assign860_e1240_d_b13, assign860_e1240_d_b14,) = {
    if ((var_guard8 != 0.0) && (!(((var_guard5 != 0.0) || (var_guard6 != 0.0)) || (var_guard7 != 0.0)))) {
        let assign860_e1238: f64 = (var_t0 * var_t0);
        (assign860_e1238, ((var_t0_dn0 * var_t0) + (var_t0 * var_t0_dn0)), ((var_t0_dn1 * var_t0) + (var_t0 * var_t0_dn1)), ((var_t0_dn2 * var_t0) + (var_t0 * var_t0_dn2)), ((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)), ((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)), ((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)), ((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)), ((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)), ((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)), ((var_t0_dn9 * var_t0) + (var_t0 * var_t0_dn9)), ((var_t0_dn10 * var_t0) + (var_t0 * var_t0_dn10)), ((var_t0_dn11 * var_t0) + (var_t0 * var_t0_dn11)), ((var_t0_dn12 * var_t0) + (var_t0 * var_t0_dn12)), ((var_t0_dn13 * var_t0) + (var_t0 * var_t0_dn13)), ((var_t0_dn14 * var_t0) + (var_t0 * var_t0_dn14)), ((var_t0_dn15 * var_t0) + (var_t0 * var_t0_dn15)), ((var_t0_db0 * var_t0) + (var_t0 * var_t0_db0)), ((var_t0_db1 * var_t0) + (var_t0 * var_t0_db1)), ((var_t0_db2 * var_t0) + (var_t0 * var_t0_db2)), ((var_t0_db3 * var_t0) + (var_t0 * var_t0_db3)), ((var_t0_db4 * var_t0) + (var_t0 * var_t0_db4)), ((var_t0_db5 * var_t0) + (var_t0 * var_t0_db5)), ((var_t0_db6 * var_t0) + (var_t0 * var_t0_db6)), ((var_t0_db7 * var_t0) + (var_t0 * var_t0_db7)), ((var_t0_db8 * var_t0) + (var_t0 * var_t0_db8)), ((var_t0_db9 * var_t0) + (var_t0 * var_t0_db9)), ((var_t0_db10 * var_t0) + (var_t0 * var_t0_db10)), ((var_t0_db11 * var_t0) + (var_t0 * var_t0_db11)), ((var_t0_db12 * var_t0) + (var_t0 * var_t0_db12)), ((var_t0_db13 * var_t0) + (var_t0 * var_t0_db13)), ((var_t0_db14 * var_t0) + (var_t0 * var_t0_db14)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn13, var_t1_dn14, var_t1_dn15, var_t1_db0, var_t1_db1, var_t1_db2, var_t1_db3, var_t1_db4, var_t1_db5, var_t1_db6, var_t1_db7, var_t1_db8, var_t1_db9, var_t1_db10, var_t1_db11, var_t1_db12, var_t1_db13, var_t1_db14,)
    }
};
        var_t1 = assign860_e1240;
        var_t1_dn0 = assign860_e1240_d_n0;
        var_t1_dn1 = assign860_e1240_d_n1;
        var_t1_dn2 = assign860_e1240_d_n2;
        var_t1_dn3 = assign860_e1240_d_n3;
        var_t1_dn4 = assign860_e1240_d_n4;
        var_t1_dn5 = assign860_e1240_d_n5;
        var_t1_dn6 = assign860_e1240_d_n6;
        var_t1_dn7 = assign860_e1240_d_n7;
        var_t1_dn8 = assign860_e1240_d_n8;
        var_t1_dn9 = assign860_e1240_d_n9;
        var_t1_dn10 = assign860_e1240_d_n10;
        var_t1_dn11 = assign860_e1240_d_n11;
        var_t1_dn12 = assign860_e1240_d_n12;
        var_t1_dn13 = assign860_e1240_d_n13;
        var_t1_dn14 = assign860_e1240_d_n14;
        var_t1_dn15 = assign860_e1240_d_n15;
        var_t1_db0 = assign860_e1240_d_b0;
        var_t1_db1 = assign860_e1240_d_b1;
        var_t1_db2 = assign860_e1240_d_b2;
        var_t1_db3 = assign860_e1240_d_b3;
        var_t1_db4 = assign860_e1240_d_b4;
        var_t1_db5 = assign860_e1240_d_b5;
        var_t1_db6 = assign860_e1240_d_b6;
        var_t1_db7 = assign860_e1240_d_b7;
        var_t1_db8 = assign860_e1240_d_b8;
        var_t1_db9 = assign860_e1240_d_b9;
        var_t1_db10 = assign860_e1240_d_b10;
        var_t1_db11 = assign860_e1240_d_b11;
        var_t1_db12 = assign860_e1240_d_b12;
        var_t1_db13 = assign860_e1240_d_b13;
        var_t1_db14 = assign860_e1240_d_b14;


        *var_guard5_slot = var_guard5;
        *var_guard6_slot = var_guard6;
        *var_guard7_slot = var_guard7;
        *var_guard8_slot = var_guard8;
        *var_psi_slot = var_psi;
        *var_psi_db0_slot = var_psi_db0;
        *var_psi_db1_slot = var_psi_db1;
        *var_psi_db10_slot = var_psi_db10;
        *var_psi_db11_slot = var_psi_db11;
        *var_psi_db12_slot = var_psi_db12;
        *var_psi_db13_slot = var_psi_db13;
        *var_psi_db14_slot = var_psi_db14;
        *var_psi_db2_slot = var_psi_db2;
        *var_psi_db3_slot = var_psi_db3;
        *var_psi_db4_slot = var_psi_db4;
        *var_psi_db5_slot = var_psi_db5;
        *var_psi_db6_slot = var_psi_db6;
        *var_psi_db7_slot = var_psi_db7;
        *var_psi_db8_slot = var_psi_db8;
        *var_psi_db9_slot = var_psi_db9;
        *var_psi_dn0_slot = var_psi_dn0;
        *var_psi_dn1_slot = var_psi_dn1;
        *var_psi_dn10_slot = var_psi_dn10;
        *var_psi_dn11_slot = var_psi_dn11;
        *var_psi_dn12_slot = var_psi_dn12;
        *var_psi_dn13_slot = var_psi_dn13;
        *var_psi_dn14_slot = var_psi_dn14;
        *var_psi_dn15_slot = var_psi_dn15;
        *var_psi_dn2_slot = var_psi_dn2;
        *var_psi_dn3_slot = var_psi_dn3;
        *var_psi_dn4_slot = var_psi_dn4;
        *var_psi_dn5_slot = var_psi_dn5;
        *var_psi_dn6_slot = var_psi_dn6;
        *var_psi_dn7_slot = var_psi_dn7;
        *var_psi_dn8_slot = var_psi_dn8;
        *var_psi_dn9_slot = var_psi_dn9;
        *var_t0_slot = var_t0;
        *var_t0_db0_slot = var_t0_db0;
        *var_t0_db1_slot = var_t0_db1;
        *var_t0_db10_slot = var_t0_db10;
        *var_t0_db11_slot = var_t0_db11;
        *var_t0_db12_slot = var_t0_db12;
        *var_t0_db13_slot = var_t0_db13;
        *var_t0_db14_slot = var_t0_db14;
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
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t1_slot = var_t1;
        *var_t1_db0_slot = var_t1_db0;
        *var_t1_db1_slot = var_t1_db1;
        *var_t1_db10_slot = var_t1_db10;
        *var_t1_db11_slot = var_t1_db11;
        *var_t1_db12_slot = var_t1_db12;
        *var_t1_db13_slot = var_t1_db13;
        *var_t1_db14_slot = var_t1_db14;
        *var_t1_db2_slot = var_t1_db2;
        *var_t1_db3_slot = var_t1_db3;
        *var_t1_db4_slot = var_t1_db4;
        *var_t1_db5_slot = var_t1_db5;
        *var_t1_db6_slot = var_t1_db6;
        *var_t1_db7_slot = var_t1_db7;
        *var_t1_db8_slot = var_t1_db8;
        *var_t1_db9_slot = var_t1_db9;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn15_slot = var_t1_dn15;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_db0_slot = var_t2_db0;
        *var_t2_db1_slot = var_t2_db1;
        *var_t2_db10_slot = var_t2_db10;
        *var_t2_db11_slot = var_t2_db11;
        *var_t2_db12_slot = var_t2_db12;
        *var_t2_db13_slot = var_t2_db13;
        *var_t2_db14_slot = var_t2_db14;
        *var_t2_db2_slot = var_t2_db2;
        *var_t2_db3_slot = var_t2_db3;
        *var_t2_db4_slot = var_t2_db4;
        *var_t2_db5_slot = var_t2_db5;
        *var_t2_db6_slot = var_t2_db6;
        *var_t2_db7_slot = var_t2_db7;
        *var_t2_db8_slot = var_t2_db8;
        *var_t2_db9_slot = var_t2_db9;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn1_slot = var_t2_dn1;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn15_slot = var_t2_dn15;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_tanh_psi_slot = var_tanh_psi;
        *var_tanh_psi1_slot = var_tanh_psi1;
        *var_tanh_psi1_db0_slot = var_tanh_psi1_db0;
        *var_tanh_psi1_db1_slot = var_tanh_psi1_db1;
        *var_tanh_psi1_db10_slot = var_tanh_psi1_db10;
        *var_tanh_psi1_db11_slot = var_tanh_psi1_db11;
        *var_tanh_psi1_db12_slot = var_tanh_psi1_db12;
        *var_tanh_psi1_db13_slot = var_tanh_psi1_db13;
        *var_tanh_psi1_db14_slot = var_tanh_psi1_db14;
        *var_tanh_psi1_db2_slot = var_tanh_psi1_db2;
        *var_tanh_psi1_db3_slot = var_tanh_psi1_db3;
        *var_tanh_psi1_db4_slot = var_tanh_psi1_db4;
        *var_tanh_psi1_db5_slot = var_tanh_psi1_db5;
        *var_tanh_psi1_db6_slot = var_tanh_psi1_db6;
        *var_tanh_psi1_db7_slot = var_tanh_psi1_db7;
        *var_tanh_psi1_db8_slot = var_tanh_psi1_db8;
        *var_tanh_psi1_db9_slot = var_tanh_psi1_db9;
        *var_tanh_psi1_dn0_slot = var_tanh_psi1_dn0;
        *var_tanh_psi1_dn1_slot = var_tanh_psi1_dn1;
        *var_tanh_psi1_dn10_slot = var_tanh_psi1_dn10;
        *var_tanh_psi1_dn11_slot = var_tanh_psi1_dn11;
        *var_tanh_psi1_dn12_slot = var_tanh_psi1_dn12;
        *var_tanh_psi1_dn13_slot = var_tanh_psi1_dn13;
        *var_tanh_psi1_dn14_slot = var_tanh_psi1_dn14;
        *var_tanh_psi1_dn15_slot = var_tanh_psi1_dn15;
        *var_tanh_psi1_dn2_slot = var_tanh_psi1_dn2;
        *var_tanh_psi1_dn3_slot = var_tanh_psi1_dn3;
        *var_tanh_psi1_dn4_slot = var_tanh_psi1_dn4;
        *var_tanh_psi1_dn5_slot = var_tanh_psi1_dn5;
        *var_tanh_psi1_dn6_slot = var_tanh_psi1_dn6;
        *var_tanh_psi1_dn7_slot = var_tanh_psi1_dn7;
        *var_tanh_psi1_dn8_slot = var_tanh_psi1_dn8;
        *var_tanh_psi1_dn9_slot = var_tanh_psi1_dn9;
        *var_tanh_psi_db0_slot = var_tanh_psi_db0;
        *var_tanh_psi_db1_slot = var_tanh_psi_db1;
        *var_tanh_psi_db10_slot = var_tanh_psi_db10;
        *var_tanh_psi_db11_slot = var_tanh_psi_db11;
        *var_tanh_psi_db12_slot = var_tanh_psi_db12;
        *var_tanh_psi_db13_slot = var_tanh_psi_db13;
        *var_tanh_psi_db14_slot = var_tanh_psi_db14;
        *var_tanh_psi_db2_slot = var_tanh_psi_db2;
        *var_tanh_psi_db3_slot = var_tanh_psi_db3;
        *var_tanh_psi_db4_slot = var_tanh_psi_db4;
        *var_tanh_psi_db5_slot = var_tanh_psi_db5;
        *var_tanh_psi_db6_slot = var_tanh_psi_db6;
        *var_tanh_psi_db7_slot = var_tanh_psi_db7;
        *var_tanh_psi_db8_slot = var_tanh_psi_db8;
        *var_tanh_psi_db9_slot = var_tanh_psi_db9;
        *var_tanh_psi_dn0_slot = var_tanh_psi_dn0;
        *var_tanh_psi_dn1_slot = var_tanh_psi_dn1;
        *var_tanh_psi_dn10_slot = var_tanh_psi_dn10;
        *var_tanh_psi_dn11_slot = var_tanh_psi_dn11;
        *var_tanh_psi_dn12_slot = var_tanh_psi_dn12;
        *var_tanh_psi_dn13_slot = var_tanh_psi_dn13;
        *var_tanh_psi_dn14_slot = var_tanh_psi_dn14;
        *var_tanh_psi_dn15_slot = var_tanh_psi_dn15;
        *var_tanh_psi_dn2_slot = var_tanh_psi_dn2;
        *var_tanh_psi_dn3_slot = var_tanh_psi_dn3;
        *var_tanh_psi_dn4_slot = var_tanh_psi_dn4;
        *var_tanh_psi_dn5_slot = var_tanh_psi_dn5;
        *var_tanh_psi_dn6_slot = var_tanh_psi_dn6;
        *var_tanh_psi_dn7_slot = var_tanh_psi_dn7;
        *var_tanh_psi_dn8_slot = var_tanh_psi_dn8;
        *var_tanh_psi_dn9_slot = var_tanh_psi_dn9;
    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        var_delta_t: f64,
        var_delta_t_db0: f64,
        var_delta_t_db1: f64,
        var_delta_t_db10: f64,
        var_delta_t_db11: f64,
        var_delta_t_db12: f64,
        var_delta_t_db13: f64,
        var_delta_t_db14: f64,
        var_delta_t_db2: f64,
        var_delta_t_db3: f64,
        var_delta_t_db4: f64,
        var_delta_t_db5: f64,
        var_delta_t_db6: f64,
        var_delta_t_db7: f64,
        var_delta_t_db8: f64,
        var_delta_t_db9: f64,
        var_delta_t_dn0: f64,
        var_delta_t_dn1: f64,
        var_delta_t_dn10: f64,
        var_delta_t_dn11: f64,
        var_delta_t_dn12: f64,
        var_delta_t_dn13: f64,
        var_delta_t_dn14: f64,
        var_delta_t_dn15: f64,
        var_delta_t_dn2: f64,
        var_delta_t_dn3: f64,
        var_delta_t_dn4: f64,
        var_delta_t_dn5: f64,
        var_delta_t_dn6: f64,
        var_delta_t_dn7: f64,
        var_delta_t_dn8: f64,
        var_delta_t_dn9: f64,
        var_guard5: f64,
        var_guard6: f64,
        var_guard7: f64,
        var_guard8: f64,
        var_p1m: f64,
        var_p1m_db0: f64,
        var_p1m_db1: f64,
        var_p1m_db10: f64,
        var_p1m_db11: f64,
        var_p1m_db12: f64,
        var_p1m_db13: f64,
        var_p1m_db14: f64,
        var_p1m_db2: f64,
        var_p1m_db3: f64,
        var_p1m_db4: f64,
        var_p1m_db5: f64,
        var_p1m_db6: f64,
        var_p1m_db7: f64,
        var_p1m_db8: f64,
        var_p1m_db9: f64,
        var_p1m_dn0: f64,
        var_p1m_dn1: f64,
        var_p1m_dn10: f64,
        var_p1m_dn11: f64,
        var_p1m_dn12: f64,
        var_p1m_dn13: f64,
        var_p1m_dn14: f64,
        var_p1m_dn15: f64,
        var_p1m_dn2: f64,
        var_p1m_dn3: f64,
        var_p1m_dn4: f64,
        var_p1m_dn5: f64,
        var_p1m_dn6: f64,
        var_p1m_dn7: f64,
        var_p1m_dn8: f64,
        var_p1m_dn9: f64,
        var_t0: f64,
        var_t0_db0: f64,
        var_t0_db1: f64,
        var_t0_db10: f64,
        var_t0_db11: f64,
        var_t0_db12: f64,
        var_t0_db13: f64,
        var_t0_db14: f64,
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
        var_t0_dn2: f64,
        var_t0_dn3: f64,
        var_t0_dn4: f64,
        var_t0_dn5: f64,
        var_t0_dn6: f64,
        var_t0_dn7: f64,
        var_t0_dn8: f64,
        var_t0_dn9: f64,
        var_t1: f64,
        var_t1_db0: f64,
        var_t1_db1: f64,
        var_t1_db10: f64,
        var_t1_db11: f64,
        var_t1_db12: f64,
        var_t1_db13: f64,
        var_t1_db14: f64,
        var_t1_db2: f64,
        var_t1_db3: f64,
        var_t1_db4: f64,
        var_t1_db5: f64,
        var_t1_db6: f64,
        var_t1_db7: f64,
        var_t1_db8: f64,
        var_t1_db9: f64,
        var_t1_dn0: f64,
        var_t1_dn1: f64,
        var_t1_dn10: f64,
        var_t1_dn11: f64,
        var_t1_dn12: f64,
        var_t1_dn13: f64,
        var_t1_dn14: f64,
        var_t1_dn15: f64,
        var_t1_dn2: f64,
        var_t1_dn3: f64,
        var_t1_dn4: f64,
        var_t1_dn5: f64,
        var_t1_dn6: f64,
        var_t1_dn7: f64,
        var_t1_dn8: f64,
        var_t1_dn9: f64,
        var_tanh_psi: f64,
        var_tanh_psi_db0: f64,
        var_tanh_psi_db1: f64,
        var_tanh_psi_db10: f64,
        var_tanh_psi_db11: f64,
        var_tanh_psi_db12: f64,
        var_tanh_psi_db13: f64,
        var_tanh_psi_db14: f64,
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
        var_tanh_psi_dn2: f64,
        var_tanh_psi_dn3: f64,
        var_tanh_psi_dn4: f64,
        var_tanh_psi_dn5: f64,
        var_tanh_psi_dn6: f64,
        var_tanh_psi_dn7: f64,
        var_tanh_psi_dn8: f64,
        var_tanh_psi_dn9: f64,
        var_vgd: f64,
        var_vgd_db0: f64,
        var_vgd_db1: f64,
        var_vgd_db10: f64,
        var_vgd_db11: f64,
        var_vgd_db12: f64,
        var_vgd_db13: f64,
        var_vgd_db14: f64,
        var_vgd_db2: f64,
        var_vgd_db3: f64,
        var_vgd_db4: f64,
        var_vgd_db5: f64,
        var_vgd_db6: f64,
        var_vgd_db7: f64,
        var_vgd_db8: f64,
        var_vgd_db9: f64,
        var_vgd_dn0: f64,
        var_vgd_dn1: f64,
        var_vgd_dn10: f64,
        var_vgd_dn11: f64,
        var_vgd_dn12: f64,
        var_vgd_dn13: f64,
        var_vgd_dn14: f64,
        var_vgd_dn15: f64,
        var_vgd_dn2: f64,
        var_vgd_dn3: f64,
        var_vgd_dn4: f64,
        var_vgd_dn5: f64,
        var_vgd_dn6: f64,
        var_vgd_dn7: f64,
        var_vgd_dn8: f64,
        var_vgd_dn9: f64,
        var_vpkm: f64,
        var_vpkm_db0: f64,
        var_vpkm_db1: f64,
        var_vpkm_db10: f64,
        var_vpkm_db11: f64,
        var_vpkm_db12: f64,
        var_vpkm_db13: f64,
        var_vpkm_db14: f64,
        var_vpkm_db2: f64,
        var_vpkm_db3: f64,
        var_vpkm_db4: f64,
        var_vpkm_db5: f64,
        var_vpkm_db6: f64,
        var_vpkm_db7: f64,
        var_vpkm_db8: f64,
        var_vpkm_db9: f64,
        var_vpkm_dn0: f64,
        var_vpkm_dn1: f64,
        var_vpkm_dn10: f64,
        var_vpkm_dn11: f64,
        var_vpkm_dn12: f64,
        var_vpkm_dn13: f64,
        var_vpkm_dn14: f64,
        var_vpkm_dn15: f64,
        var_vpkm_dn2: f64,
        var_vpkm_dn3: f64,
        var_vpkm_dn4: f64,
        var_vpkm_dn5: f64,
        var_vpkm_dn6: f64,
        var_vpkm_dn7: f64,
        var_vpkm_dn8: f64,
        var_vpkm_dn9: f64,
        var_guard10_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard9_slot: &mut f64,
        var_psi_slot: &mut f64,
        var_psi_db0_slot: &mut f64,
        var_psi_db1_slot: &mut f64,
        var_psi_db10_slot: &mut f64,
        var_psi_db11_slot: &mut f64,
        var_psi_db12_slot: &mut f64,
        var_psi_db13_slot: &mut f64,
        var_psi_db14_slot: &mut f64,
        var_psi_db2_slot: &mut f64,
        var_psi_db3_slot: &mut f64,
        var_psi_db4_slot: &mut f64,
        var_psi_db5_slot: &mut f64,
        var_psi_db6_slot: &mut f64,
        var_psi_db7_slot: &mut f64,
        var_psi_db8_slot: &mut f64,
        var_psi_db9_slot: &mut f64,
        var_psi_dn0_slot: &mut f64,
        var_psi_dn1_slot: &mut f64,
        var_psi_dn10_slot: &mut f64,
        var_psi_dn11_slot: &mut f64,
        var_psi_dn12_slot: &mut f64,
        var_psi_dn13_slot: &mut f64,
        var_psi_dn14_slot: &mut f64,
        var_psi_dn15_slot: &mut f64,
        var_psi_dn2_slot: &mut f64,
        var_psi_dn3_slot: &mut f64,
        var_psi_dn4_slot: &mut f64,
        var_psi_dn5_slot: &mut f64,
        var_psi_dn6_slot: &mut f64,
        var_psi_dn7_slot: &mut f64,
        var_psi_dn8_slot: &mut f64,
        var_psi_dn9_slot: &mut f64,
        var_rd1_slot: &mut f64,
        var_rd1_db0_slot: &mut f64,
        var_rd1_db1_slot: &mut f64,
        var_rd1_db10_slot: &mut f64,
        var_rd1_db11_slot: &mut f64,
        var_rd1_db12_slot: &mut f64,
        var_rd1_db13_slot: &mut f64,
        var_rd1_db14_slot: &mut f64,
        var_rd1_db2_slot: &mut f64,
        var_rd1_db3_slot: &mut f64,
        var_rd1_db4_slot: &mut f64,
        var_rd1_db5_slot: &mut f64,
        var_rd1_db6_slot: &mut f64,
        var_rd1_db7_slot: &mut f64,
        var_rd1_db8_slot: &mut f64,
        var_rd1_db9_slot: &mut f64,
        var_rd1_dn0_slot: &mut f64,
        var_rd1_dn1_slot: &mut f64,
        var_rd1_dn10_slot: &mut f64,
        var_rd1_dn11_slot: &mut f64,
        var_rd1_dn12_slot: &mut f64,
        var_rd1_dn13_slot: &mut f64,
        var_rd1_dn14_slot: &mut f64,
        var_rd1_dn15_slot: &mut f64,
        var_rd1_dn2_slot: &mut f64,
        var_rd1_dn3_slot: &mut f64,
        var_rd1_dn4_slot: &mut f64,
        var_rd1_dn5_slot: &mut f64,
        var_rd1_dn6_slot: &mut f64,
        var_rd1_dn7_slot: &mut f64,
        var_rd1_dn8_slot: &mut f64,
        var_rd1_dn9_slot: &mut f64,
        var_rd1_t_slot: &mut f64,
        var_rd1_t_db0_slot: &mut f64,
        var_rd1_t_db1_slot: &mut f64,
        var_rd1_t_db10_slot: &mut f64,
        var_rd1_t_db11_slot: &mut f64,
        var_rd1_t_db12_slot: &mut f64,
        var_rd1_t_db13_slot: &mut f64,
        var_rd1_t_db14_slot: &mut f64,
        var_rd1_t_db2_slot: &mut f64,
        var_rd1_t_db3_slot: &mut f64,
        var_rd1_t_db4_slot: &mut f64,
        var_rd1_t_db5_slot: &mut f64,
        var_rd1_t_db6_slot: &mut f64,
        var_rd1_t_db7_slot: &mut f64,
        var_rd1_t_db8_slot: &mut f64,
        var_rd1_t_db9_slot: &mut f64,
        var_rd1_t_dn0_slot: &mut f64,
        var_rd1_t_dn1_slot: &mut f64,
        var_rd1_t_dn10_slot: &mut f64,
        var_rd1_t_dn11_slot: &mut f64,
        var_rd1_t_dn12_slot: &mut f64,
        var_rd1_t_dn13_slot: &mut f64,
        var_rd1_t_dn14_slot: &mut f64,
        var_rd1_t_dn15_slot: &mut f64,
        var_rd1_t_dn2_slot: &mut f64,
        var_rd1_t_dn3_slot: &mut f64,
        var_rd1_t_dn4_slot: &mut f64,
        var_rd1_t_dn5_slot: &mut f64,
        var_rd1_t_dn6_slot: &mut f64,
        var_rd1_t_dn7_slot: &mut f64,
        var_rd1_t_dn8_slot: &mut f64,
        var_rd1_t_dn9_slot: &mut f64,
        var_rs1_slot: &mut f64,
        var_rs1_db0_slot: &mut f64,
        var_rs1_db1_slot: &mut f64,
        var_rs1_db10_slot: &mut f64,
        var_rs1_db11_slot: &mut f64,
        var_rs1_db12_slot: &mut f64,
        var_rs1_db13_slot: &mut f64,
        var_rs1_db14_slot: &mut f64,
        var_rs1_db2_slot: &mut f64,
        var_rs1_db3_slot: &mut f64,
        var_rs1_db4_slot: &mut f64,
        var_rs1_db5_slot: &mut f64,
        var_rs1_db6_slot: &mut f64,
        var_rs1_db7_slot: &mut f64,
        var_rs1_db8_slot: &mut f64,
        var_rs1_db9_slot: &mut f64,
        var_rs1_dn0_slot: &mut f64,
        var_rs1_dn1_slot: &mut f64,
        var_rs1_dn10_slot: &mut f64,
        var_rs1_dn11_slot: &mut f64,
        var_rs1_dn12_slot: &mut f64,
        var_rs1_dn13_slot: &mut f64,
        var_rs1_dn14_slot: &mut f64,
        var_rs1_dn15_slot: &mut f64,
        var_rs1_dn2_slot: &mut f64,
        var_rs1_dn3_slot: &mut f64,
        var_rs1_dn4_slot: &mut f64,
        var_rs1_dn5_slot: &mut f64,
        var_rs1_dn6_slot: &mut f64,
        var_rs1_dn7_slot: &mut f64,
        var_rs1_dn8_slot: &mut f64,
        var_rs1_dn9_slot: &mut f64,
        var_rs_t_slot: &mut f64,
        var_rs_t_db0_slot: &mut f64,
        var_rs_t_db1_slot: &mut f64,
        var_rs_t_db10_slot: &mut f64,
        var_rs_t_db11_slot: &mut f64,
        var_rs_t_db12_slot: &mut f64,
        var_rs_t_db13_slot: &mut f64,
        var_rs_t_db14_slot: &mut f64,
        var_rs_t_db2_slot: &mut f64,
        var_rs_t_db3_slot: &mut f64,
        var_rs_t_db4_slot: &mut f64,
        var_rs_t_db5_slot: &mut f64,
        var_rs_t_db6_slot: &mut f64,
        var_rs_t_db7_slot: &mut f64,
        var_rs_t_db8_slot: &mut f64,
        var_rs_t_db9_slot: &mut f64,
        var_rs_t_dn0_slot: &mut f64,
        var_rs_t_dn1_slot: &mut f64,
        var_rs_t_dn10_slot: &mut f64,
        var_rs_t_dn11_slot: &mut f64,
        var_rs_t_dn12_slot: &mut f64,
        var_rs_t_dn13_slot: &mut f64,
        var_rs_t_dn14_slot: &mut f64,
        var_rs_t_dn15_slot: &mut f64,
        var_rs_t_dn2_slot: &mut f64,
        var_rs_t_dn3_slot: &mut f64,
        var_rs_t_dn4_slot: &mut f64,
        var_rs_t_dn5_slot: &mut f64,
        var_rs_t_dn6_slot: &mut f64,
        var_rs_t_dn7_slot: &mut f64,
        var_rs_t_dn8_slot: &mut f64,
        var_rs_t_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_db0_slot: &mut f64,
        var_t2_db1_slot: &mut f64,
        var_t2_db10_slot: &mut f64,
        var_t2_db11_slot: &mut f64,
        var_t2_db12_slot: &mut f64,
        var_t2_db13_slot: &mut f64,
        var_t2_db14_slot: &mut f64,
        var_t2_db2_slot: &mut f64,
        var_t2_db3_slot: &mut f64,
        var_t2_db4_slot: &mut f64,
        var_t2_db5_slot: &mut f64,
        var_t2_db6_slot: &mut f64,
        var_t2_db7_slot: &mut f64,
        var_t2_db8_slot: &mut f64,
        var_t2_db9_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn1_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn15_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_tanh_psi1_slot: &mut f64,
        var_tanh_psi1_db0_slot: &mut f64,
        var_tanh_psi1_db1_slot: &mut f64,
        var_tanh_psi1_db10_slot: &mut f64,
        var_tanh_psi1_db11_slot: &mut f64,
        var_tanh_psi1_db12_slot: &mut f64,
        var_tanh_psi1_db13_slot: &mut f64,
        var_tanh_psi1_db14_slot: &mut f64,
        var_tanh_psi1_db2_slot: &mut f64,
        var_tanh_psi1_db3_slot: &mut f64,
        var_tanh_psi1_db4_slot: &mut f64,
        var_tanh_psi1_db5_slot: &mut f64,
        var_tanh_psi1_db6_slot: &mut f64,
        var_tanh_psi1_db7_slot: &mut f64,
        var_tanh_psi1_db8_slot: &mut f64,
        var_tanh_psi1_db9_slot: &mut f64,
        var_tanh_psi1_dn0_slot: &mut f64,
        var_tanh_psi1_dn1_slot: &mut f64,
        var_tanh_psi1_dn10_slot: &mut f64,
        var_tanh_psi1_dn11_slot: &mut f64,
        var_tanh_psi1_dn12_slot: &mut f64,
        var_tanh_psi1_dn13_slot: &mut f64,
        var_tanh_psi1_dn14_slot: &mut f64,
        var_tanh_psi1_dn15_slot: &mut f64,
        var_tanh_psi1_dn2_slot: &mut f64,
        var_tanh_psi1_dn3_slot: &mut f64,
        var_tanh_psi1_dn4_slot: &mut f64,
        var_tanh_psi1_dn5_slot: &mut f64,
        var_tanh_psi1_dn6_slot: &mut f64,
        var_tanh_psi1_dn7_slot: &mut f64,
        var_tanh_psi1_dn8_slot: &mut f64,
        var_tanh_psi1_dn9_slot: &mut f64,
    ) {
        let mut var_guard10: f64 = *var_guard10_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard9: f64 = *var_guard9_slot;
        let mut var_psi: f64 = *var_psi_slot;
        let mut var_psi_db0: f64 = *var_psi_db0_slot;
        let mut var_psi_db1: f64 = *var_psi_db1_slot;
        let mut var_psi_db10: f64 = *var_psi_db10_slot;
        let mut var_psi_db11: f64 = *var_psi_db11_slot;
        let mut var_psi_db12: f64 = *var_psi_db12_slot;
        let mut var_psi_db13: f64 = *var_psi_db13_slot;
        let mut var_psi_db14: f64 = *var_psi_db14_slot;
        let mut var_psi_db2: f64 = *var_psi_db2_slot;
        let mut var_psi_db3: f64 = *var_psi_db3_slot;
        let mut var_psi_db4: f64 = *var_psi_db4_slot;
        let mut var_psi_db5: f64 = *var_psi_db5_slot;
        let mut var_psi_db6: f64 = *var_psi_db6_slot;
        let mut var_psi_db7: f64 = *var_psi_db7_slot;
        let mut var_psi_db8: f64 = *var_psi_db8_slot;
        let mut var_psi_db9: f64 = *var_psi_db9_slot;
        let mut var_psi_dn0: f64 = *var_psi_dn0_slot;
        let mut var_psi_dn1: f64 = *var_psi_dn1_slot;
        let mut var_psi_dn10: f64 = *var_psi_dn10_slot;
        let mut var_psi_dn11: f64 = *var_psi_dn11_slot;
        let mut var_psi_dn12: f64 = *var_psi_dn12_slot;
        let mut var_psi_dn13: f64 = *var_psi_dn13_slot;
        let mut var_psi_dn14: f64 = *var_psi_dn14_slot;
        let mut var_psi_dn15: f64 = *var_psi_dn15_slot;
        let mut var_psi_dn2: f64 = *var_psi_dn2_slot;
        let mut var_psi_dn3: f64 = *var_psi_dn3_slot;
        let mut var_psi_dn4: f64 = *var_psi_dn4_slot;
        let mut var_psi_dn5: f64 = *var_psi_dn5_slot;
        let mut var_psi_dn6: f64 = *var_psi_dn6_slot;
        let mut var_psi_dn7: f64 = *var_psi_dn7_slot;
        let mut var_psi_dn8: f64 = *var_psi_dn8_slot;
        let mut var_psi_dn9: f64 = *var_psi_dn9_slot;
        let mut var_rd1: f64 = *var_rd1_slot;
        let mut var_rd1_db0: f64 = *var_rd1_db0_slot;
        let mut var_rd1_db1: f64 = *var_rd1_db1_slot;
        let mut var_rd1_db10: f64 = *var_rd1_db10_slot;
        let mut var_rd1_db11: f64 = *var_rd1_db11_slot;
        let mut var_rd1_db12: f64 = *var_rd1_db12_slot;
        let mut var_rd1_db13: f64 = *var_rd1_db13_slot;
        let mut var_rd1_db14: f64 = *var_rd1_db14_slot;
        let mut var_rd1_db2: f64 = *var_rd1_db2_slot;
        let mut var_rd1_db3: f64 = *var_rd1_db3_slot;
        let mut var_rd1_db4: f64 = *var_rd1_db4_slot;
        let mut var_rd1_db5: f64 = *var_rd1_db5_slot;
        let mut var_rd1_db6: f64 = *var_rd1_db6_slot;
        let mut var_rd1_db7: f64 = *var_rd1_db7_slot;
        let mut var_rd1_db8: f64 = *var_rd1_db8_slot;
        let mut var_rd1_db9: f64 = *var_rd1_db9_slot;
        let mut var_rd1_dn0: f64 = *var_rd1_dn0_slot;
        let mut var_rd1_dn1: f64 = *var_rd1_dn1_slot;
        let mut var_rd1_dn10: f64 = *var_rd1_dn10_slot;
        let mut var_rd1_dn11: f64 = *var_rd1_dn11_slot;
        let mut var_rd1_dn12: f64 = *var_rd1_dn12_slot;
        let mut var_rd1_dn13: f64 = *var_rd1_dn13_slot;
        let mut var_rd1_dn14: f64 = *var_rd1_dn14_slot;
        let mut var_rd1_dn15: f64 = *var_rd1_dn15_slot;
        let mut var_rd1_dn2: f64 = *var_rd1_dn2_slot;
        let mut var_rd1_dn3: f64 = *var_rd1_dn3_slot;
        let mut var_rd1_dn4: f64 = *var_rd1_dn4_slot;
        let mut var_rd1_dn5: f64 = *var_rd1_dn5_slot;
        let mut var_rd1_dn6: f64 = *var_rd1_dn6_slot;
        let mut var_rd1_dn7: f64 = *var_rd1_dn7_slot;
        let mut var_rd1_dn8: f64 = *var_rd1_dn8_slot;
        let mut var_rd1_dn9: f64 = *var_rd1_dn9_slot;
        let mut var_rd1_t: f64 = *var_rd1_t_slot;
        let mut var_rd1_t_db0: f64 = *var_rd1_t_db0_slot;
        let mut var_rd1_t_db1: f64 = *var_rd1_t_db1_slot;
        let mut var_rd1_t_db10: f64 = *var_rd1_t_db10_slot;
        let mut var_rd1_t_db11: f64 = *var_rd1_t_db11_slot;
        let mut var_rd1_t_db12: f64 = *var_rd1_t_db12_slot;
        let mut var_rd1_t_db13: f64 = *var_rd1_t_db13_slot;
        let mut var_rd1_t_db14: f64 = *var_rd1_t_db14_slot;
        let mut var_rd1_t_db2: f64 = *var_rd1_t_db2_slot;
        let mut var_rd1_t_db3: f64 = *var_rd1_t_db3_slot;
        let mut var_rd1_t_db4: f64 = *var_rd1_t_db4_slot;
        let mut var_rd1_t_db5: f64 = *var_rd1_t_db5_slot;
        let mut var_rd1_t_db6: f64 = *var_rd1_t_db6_slot;
        let mut var_rd1_t_db7: f64 = *var_rd1_t_db7_slot;
        let mut var_rd1_t_db8: f64 = *var_rd1_t_db8_slot;
        let mut var_rd1_t_db9: f64 = *var_rd1_t_db9_slot;
        let mut var_rd1_t_dn0: f64 = *var_rd1_t_dn0_slot;
        let mut var_rd1_t_dn1: f64 = *var_rd1_t_dn1_slot;
        let mut var_rd1_t_dn10: f64 = *var_rd1_t_dn10_slot;
        let mut var_rd1_t_dn11: f64 = *var_rd1_t_dn11_slot;
        let mut var_rd1_t_dn12: f64 = *var_rd1_t_dn12_slot;
        let mut var_rd1_t_dn13: f64 = *var_rd1_t_dn13_slot;
        let mut var_rd1_t_dn14: f64 = *var_rd1_t_dn14_slot;
        let mut var_rd1_t_dn15: f64 = *var_rd1_t_dn15_slot;
        let mut var_rd1_t_dn2: f64 = *var_rd1_t_dn2_slot;
        let mut var_rd1_t_dn3: f64 = *var_rd1_t_dn3_slot;
        let mut var_rd1_t_dn4: f64 = *var_rd1_t_dn4_slot;
        let mut var_rd1_t_dn5: f64 = *var_rd1_t_dn5_slot;
        let mut var_rd1_t_dn6: f64 = *var_rd1_t_dn6_slot;
        let mut var_rd1_t_dn7: f64 = *var_rd1_t_dn7_slot;
        let mut var_rd1_t_dn8: f64 = *var_rd1_t_dn8_slot;
        let mut var_rd1_t_dn9: f64 = *var_rd1_t_dn9_slot;
        let mut var_rs1: f64 = *var_rs1_slot;
        let mut var_rs1_db0: f64 = *var_rs1_db0_slot;
        let mut var_rs1_db1: f64 = *var_rs1_db1_slot;
        let mut var_rs1_db10: f64 = *var_rs1_db10_slot;
        let mut var_rs1_db11: f64 = *var_rs1_db11_slot;
        let mut var_rs1_db12: f64 = *var_rs1_db12_slot;
        let mut var_rs1_db13: f64 = *var_rs1_db13_slot;
        let mut var_rs1_db14: f64 = *var_rs1_db14_slot;
        let mut var_rs1_db2: f64 = *var_rs1_db2_slot;
        let mut var_rs1_db3: f64 = *var_rs1_db3_slot;
        let mut var_rs1_db4: f64 = *var_rs1_db4_slot;
        let mut var_rs1_db5: f64 = *var_rs1_db5_slot;
        let mut var_rs1_db6: f64 = *var_rs1_db6_slot;
        let mut var_rs1_db7: f64 = *var_rs1_db7_slot;
        let mut var_rs1_db8: f64 = *var_rs1_db8_slot;
        let mut var_rs1_db9: f64 = *var_rs1_db9_slot;
        let mut var_rs1_dn0: f64 = *var_rs1_dn0_slot;
        let mut var_rs1_dn1: f64 = *var_rs1_dn1_slot;
        let mut var_rs1_dn10: f64 = *var_rs1_dn10_slot;
        let mut var_rs1_dn11: f64 = *var_rs1_dn11_slot;
        let mut var_rs1_dn12: f64 = *var_rs1_dn12_slot;
        let mut var_rs1_dn13: f64 = *var_rs1_dn13_slot;
        let mut var_rs1_dn14: f64 = *var_rs1_dn14_slot;
        let mut var_rs1_dn15: f64 = *var_rs1_dn15_slot;
        let mut var_rs1_dn2: f64 = *var_rs1_dn2_slot;
        let mut var_rs1_dn3: f64 = *var_rs1_dn3_slot;
        let mut var_rs1_dn4: f64 = *var_rs1_dn4_slot;
        let mut var_rs1_dn5: f64 = *var_rs1_dn5_slot;
        let mut var_rs1_dn6: f64 = *var_rs1_dn6_slot;
        let mut var_rs1_dn7: f64 = *var_rs1_dn7_slot;
        let mut var_rs1_dn8: f64 = *var_rs1_dn8_slot;
        let mut var_rs1_dn9: f64 = *var_rs1_dn9_slot;
        let mut var_rs_t: f64 = *var_rs_t_slot;
        let mut var_rs_t_db0: f64 = *var_rs_t_db0_slot;
        let mut var_rs_t_db1: f64 = *var_rs_t_db1_slot;
        let mut var_rs_t_db10: f64 = *var_rs_t_db10_slot;
        let mut var_rs_t_db11: f64 = *var_rs_t_db11_slot;
        let mut var_rs_t_db12: f64 = *var_rs_t_db12_slot;
        let mut var_rs_t_db13: f64 = *var_rs_t_db13_slot;
        let mut var_rs_t_db14: f64 = *var_rs_t_db14_slot;
        let mut var_rs_t_db2: f64 = *var_rs_t_db2_slot;
        let mut var_rs_t_db3: f64 = *var_rs_t_db3_slot;
        let mut var_rs_t_db4: f64 = *var_rs_t_db4_slot;
        let mut var_rs_t_db5: f64 = *var_rs_t_db5_slot;
        let mut var_rs_t_db6: f64 = *var_rs_t_db6_slot;
        let mut var_rs_t_db7: f64 = *var_rs_t_db7_slot;
        let mut var_rs_t_db8: f64 = *var_rs_t_db8_slot;
        let mut var_rs_t_db9: f64 = *var_rs_t_db9_slot;
        let mut var_rs_t_dn0: f64 = *var_rs_t_dn0_slot;
        let mut var_rs_t_dn1: f64 = *var_rs_t_dn1_slot;
        let mut var_rs_t_dn10: f64 = *var_rs_t_dn10_slot;
        let mut var_rs_t_dn11: f64 = *var_rs_t_dn11_slot;
        let mut var_rs_t_dn12: f64 = *var_rs_t_dn12_slot;
        let mut var_rs_t_dn13: f64 = *var_rs_t_dn13_slot;
        let mut var_rs_t_dn14: f64 = *var_rs_t_dn14_slot;
        let mut var_rs_t_dn15: f64 = *var_rs_t_dn15_slot;
        let mut var_rs_t_dn2: f64 = *var_rs_t_dn2_slot;
        let mut var_rs_t_dn3: f64 = *var_rs_t_dn3_slot;
        let mut var_rs_t_dn4: f64 = *var_rs_t_dn4_slot;
        let mut var_rs_t_dn5: f64 = *var_rs_t_dn5_slot;
        let mut var_rs_t_dn6: f64 = *var_rs_t_dn6_slot;
        let mut var_rs_t_dn7: f64 = *var_rs_t_dn7_slot;
        let mut var_rs_t_dn8: f64 = *var_rs_t_dn8_slot;
        let mut var_rs_t_dn9: f64 = *var_rs_t_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_db0: f64 = *var_t2_db0_slot;
        let mut var_t2_db1: f64 = *var_t2_db1_slot;
        let mut var_t2_db10: f64 = *var_t2_db10_slot;
        let mut var_t2_db11: f64 = *var_t2_db11_slot;
        let mut var_t2_db12: f64 = *var_t2_db12_slot;
        let mut var_t2_db13: f64 = *var_t2_db13_slot;
        let mut var_t2_db14: f64 = *var_t2_db14_slot;
        let mut var_t2_db2: f64 = *var_t2_db2_slot;
        let mut var_t2_db3: f64 = *var_t2_db3_slot;
        let mut var_t2_db4: f64 = *var_t2_db4_slot;
        let mut var_t2_db5: f64 = *var_t2_db5_slot;
        let mut var_t2_db6: f64 = *var_t2_db6_slot;
        let mut var_t2_db7: f64 = *var_t2_db7_slot;
        let mut var_t2_db8: f64 = *var_t2_db8_slot;
        let mut var_t2_db9: f64 = *var_t2_db9_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn1: f64 = *var_t2_dn1_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn15: f64 = *var_t2_dn15_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_tanh_psi1: f64 = *var_tanh_psi1_slot;
        let mut var_tanh_psi1_db0: f64 = *var_tanh_psi1_db0_slot;
        let mut var_tanh_psi1_db1: f64 = *var_tanh_psi1_db1_slot;
        let mut var_tanh_psi1_db10: f64 = *var_tanh_psi1_db10_slot;
        let mut var_tanh_psi1_db11: f64 = *var_tanh_psi1_db11_slot;
        let mut var_tanh_psi1_db12: f64 = *var_tanh_psi1_db12_slot;
        let mut var_tanh_psi1_db13: f64 = *var_tanh_psi1_db13_slot;
        let mut var_tanh_psi1_db14: f64 = *var_tanh_psi1_db14_slot;
        let mut var_tanh_psi1_db2: f64 = *var_tanh_psi1_db2_slot;
        let mut var_tanh_psi1_db3: f64 = *var_tanh_psi1_db3_slot;
        let mut var_tanh_psi1_db4: f64 = *var_tanh_psi1_db4_slot;
        let mut var_tanh_psi1_db5: f64 = *var_tanh_psi1_db5_slot;
        let mut var_tanh_psi1_db6: f64 = *var_tanh_psi1_db6_slot;
        let mut var_tanh_psi1_db7: f64 = *var_tanh_psi1_db7_slot;
        let mut var_tanh_psi1_db8: f64 = *var_tanh_psi1_db8_slot;
        let mut var_tanh_psi1_db9: f64 = *var_tanh_psi1_db9_slot;
        let mut var_tanh_psi1_dn0: f64 = *var_tanh_psi1_dn0_slot;
        let mut var_tanh_psi1_dn1: f64 = *var_tanh_psi1_dn1_slot;
        let mut var_tanh_psi1_dn10: f64 = *var_tanh_psi1_dn10_slot;
        let mut var_tanh_psi1_dn11: f64 = *var_tanh_psi1_dn11_slot;
        let mut var_tanh_psi1_dn12: f64 = *var_tanh_psi1_dn12_slot;
        let mut var_tanh_psi1_dn13: f64 = *var_tanh_psi1_dn13_slot;
        let mut var_tanh_psi1_dn14: f64 = *var_tanh_psi1_dn14_slot;
        let mut var_tanh_psi1_dn15: f64 = *var_tanh_psi1_dn15_slot;
        let mut var_tanh_psi1_dn2: f64 = *var_tanh_psi1_dn2_slot;
        let mut var_tanh_psi1_dn3: f64 = *var_tanh_psi1_dn3_slot;
        let mut var_tanh_psi1_dn4: f64 = *var_tanh_psi1_dn4_slot;
        let mut var_tanh_psi1_dn5: f64 = *var_tanh_psi1_dn5_slot;
        let mut var_tanh_psi1_dn6: f64 = *var_tanh_psi1_dn6_slot;
        let mut var_tanh_psi1_dn7: f64 = *var_tanh_psi1_dn7_slot;
        let mut var_tanh_psi1_dn8: f64 = *var_tanh_psi1_dn8_slot;
        let mut var_tanh_psi1_dn9: f64 = *var_tanh_psi1_dn9_slot;

        let (assign870_e1263, assign870_e1263_d_n0, assign870_e1263_d_n1, assign870_e1263_d_n2, assign870_e1263_d_n3, assign870_e1263_d_n4, assign870_e1263_d_n5, assign870_e1263_d_n6, assign870_e1263_d_n7, assign870_e1263_d_n8, assign870_e1263_d_n9, assign870_e1263_d_n10, assign870_e1263_d_n11, assign870_e1263_d_n12, assign870_e1263_d_n13, assign870_e1263_d_n14, assign870_e1263_d_n15, assign870_e1263_d_b0, assign870_e1263_d_b1, assign870_e1263_d_b2, assign870_e1263_d_b3, assign870_e1263_d_b4, assign870_e1263_d_b5, assign870_e1263_d_b6, assign870_e1263_d_b7, assign870_e1263_d_b8, assign870_e1263_d_b9, assign870_e1263_d_b10, assign870_e1263_d_b11, assign870_e1263_d_b12, assign870_e1263_d_b13, assign870_e1263_d_b14,) = {
    if ((var_guard8 != 0.0) && (!(((var_guard5 != 0.0) || (var_guard6 != 0.0)) || (var_guard7 != 0.0)))) {
        let assign870_e1253: f64 = (p.p12 * var_t1);
        let assign870_e1254: f64 = (var_t0 + assign870_e1253);
        let assign870_e1257: f64 = (p.p13 * var_t1);
        let assign870_e1259: f64 = (assign870_e1257 * var_t0);
        let assign870_e1260: f64 = (assign870_e1254 + assign870_e1259);
        let assign870_e1261: f64 = (var_p1m * assign870_e1260);
        (assign870_e1261, ((var_p1m_dn0 * assign870_e1260) + (var_p1m * ((var_t0_dn0 + (p.p12 * var_t1_dn0)) + (((p.p13 * var_t1_dn0) * var_t0) + (assign870_e1257 * var_t0_dn0))))), ((var_p1m_dn1 * assign870_e1260) + (var_p1m * ((var_t0_dn1 + (p.p12 * var_t1_dn1)) + (((p.p13 * var_t1_dn1) * var_t0) + (assign870_e1257 * var_t0_dn1))))), ((var_p1m_dn2 * assign870_e1260) + (var_p1m * ((var_t0_dn2 + (p.p12 * var_t1_dn2)) + (((p.p13 * var_t1_dn2) * var_t0) + (assign870_e1257 * var_t0_dn2))))), ((var_p1m_dn3 * assign870_e1260) + (var_p1m * ((var_t0_dn3 + (p.p12 * var_t1_dn3)) + (((p.p13 * var_t1_dn3) * var_t0) + (assign870_e1257 * var_t0_dn3))))), ((var_p1m_dn4 * assign870_e1260) + (var_p1m * ((var_t0_dn4 + (p.p12 * var_t1_dn4)) + (((p.p13 * var_t1_dn4) * var_t0) + (assign870_e1257 * var_t0_dn4))))), ((var_p1m_dn5 * assign870_e1260) + (var_p1m * ((var_t0_dn5 + (p.p12 * var_t1_dn5)) + (((p.p13 * var_t1_dn5) * var_t0) + (assign870_e1257 * var_t0_dn5))))), ((var_p1m_dn6 * assign870_e1260) + (var_p1m * ((var_t0_dn6 + (p.p12 * var_t1_dn6)) + (((p.p13 * var_t1_dn6) * var_t0) + (assign870_e1257 * var_t0_dn6))))), ((var_p1m_dn7 * assign870_e1260) + (var_p1m * ((var_t0_dn7 + (p.p12 * var_t1_dn7)) + (((p.p13 * var_t1_dn7) * var_t0) + (assign870_e1257 * var_t0_dn7))))), ((var_p1m_dn8 * assign870_e1260) + (var_p1m * ((var_t0_dn8 + (p.p12 * var_t1_dn8)) + (((p.p13 * var_t1_dn8) * var_t0) + (assign870_e1257 * var_t0_dn8))))), ((var_p1m_dn9 * assign870_e1260) + (var_p1m * ((var_t0_dn9 + (p.p12 * var_t1_dn9)) + (((p.p13 * var_t1_dn9) * var_t0) + (assign870_e1257 * var_t0_dn9))))), ((var_p1m_dn10 * assign870_e1260) + (var_p1m * ((var_t0_dn10 + (p.p12 * var_t1_dn10)) + (((p.p13 * var_t1_dn10) * var_t0) + (assign870_e1257 * var_t0_dn10))))), ((var_p1m_dn11 * assign870_e1260) + (var_p1m * ((var_t0_dn11 + (p.p12 * var_t1_dn11)) + (((p.p13 * var_t1_dn11) * var_t0) + (assign870_e1257 * var_t0_dn11))))), ((var_p1m_dn12 * assign870_e1260) + (var_p1m * ((var_t0_dn12 + (p.p12 * var_t1_dn12)) + (((p.p13 * var_t1_dn12) * var_t0) + (assign870_e1257 * var_t0_dn12))))), ((var_p1m_dn13 * assign870_e1260) + (var_p1m * ((var_t0_dn13 + (p.p12 * var_t1_dn13)) + (((p.p13 * var_t1_dn13) * var_t0) + (assign870_e1257 * var_t0_dn13))))), ((var_p1m_dn14 * assign870_e1260) + (var_p1m * ((var_t0_dn14 + (p.p12 * var_t1_dn14)) + (((p.p13 * var_t1_dn14) * var_t0) + (assign870_e1257 * var_t0_dn14))))), ((var_p1m_dn15 * assign870_e1260) + (var_p1m * ((var_t0_dn15 + (p.p12 * var_t1_dn15)) + (((p.p13 * var_t1_dn15) * var_t0) + (assign870_e1257 * var_t0_dn15))))), ((var_p1m_db0 * assign870_e1260) + (var_p1m * ((var_t0_db0 + (p.p12 * var_t1_db0)) + (((p.p13 * var_t1_db0) * var_t0) + (assign870_e1257 * var_t0_db0))))), ((var_p1m_db1 * assign870_e1260) + (var_p1m * ((var_t0_db1 + (p.p12 * var_t1_db1)) + (((p.p13 * var_t1_db1) * var_t0) + (assign870_e1257 * var_t0_db1))))), ((var_p1m_db2 * assign870_e1260) + (var_p1m * ((var_t0_db2 + (p.p12 * var_t1_db2)) + (((p.p13 * var_t1_db2) * var_t0) + (assign870_e1257 * var_t0_db2))))), ((var_p1m_db3 * assign870_e1260) + (var_p1m * ((var_t0_db3 + (p.p12 * var_t1_db3)) + (((p.p13 * var_t1_db3) * var_t0) + (assign870_e1257 * var_t0_db3))))), ((var_p1m_db4 * assign870_e1260) + (var_p1m * ((var_t0_db4 + (p.p12 * var_t1_db4)) + (((p.p13 * var_t1_db4) * var_t0) + (assign870_e1257 * var_t0_db4))))), ((var_p1m_db5 * assign870_e1260) + (var_p1m * ((var_t0_db5 + (p.p12 * var_t1_db5)) + (((p.p13 * var_t1_db5) * var_t0) + (assign870_e1257 * var_t0_db5))))), ((var_p1m_db6 * assign870_e1260) + (var_p1m * ((var_t0_db6 + (p.p12 * var_t1_db6)) + (((p.p13 * var_t1_db6) * var_t0) + (assign870_e1257 * var_t0_db6))))), ((var_p1m_db7 * assign870_e1260) + (var_p1m * ((var_t0_db7 + (p.p12 * var_t1_db7)) + (((p.p13 * var_t1_db7) * var_t0) + (assign870_e1257 * var_t0_db7))))), ((var_p1m_db8 * assign870_e1260) + (var_p1m * ((var_t0_db8 + (p.p12 * var_t1_db8)) + (((p.p13 * var_t1_db8) * var_t0) + (assign870_e1257 * var_t0_db8))))), ((var_p1m_db9 * assign870_e1260) + (var_p1m * ((var_t0_db9 + (p.p12 * var_t1_db9)) + (((p.p13 * var_t1_db9) * var_t0) + (assign870_e1257 * var_t0_db9))))), ((var_p1m_db10 * assign870_e1260) + (var_p1m * ((var_t0_db10 + (p.p12 * var_t1_db10)) + (((p.p13 * var_t1_db10) * var_t0) + (assign870_e1257 * var_t0_db10))))), ((var_p1m_db11 * assign870_e1260) + (var_p1m * ((var_t0_db11 + (p.p12 * var_t1_db11)) + (((p.p13 * var_t1_db11) * var_t0) + (assign870_e1257 * var_t0_db11))))), ((var_p1m_db12 * assign870_e1260) + (var_p1m * ((var_t0_db12 + (p.p12 * var_t1_db12)) + (((p.p13 * var_t1_db12) * var_t0) + (assign870_e1257 * var_t0_db12))))), ((var_p1m_db13 * assign870_e1260) + (var_p1m * ((var_t0_db13 + (p.p12 * var_t1_db13)) + (((p.p13 * var_t1_db13) * var_t0) + (assign870_e1257 * var_t0_db13))))), ((var_p1m_db14 * assign870_e1260) + (var_p1m * ((var_t0_db14 + (p.p12 * var_t1_db14)) + (((p.p13 * var_t1_db14) * var_t0) + (assign870_e1257 * var_t0_db14))))),)
    } else {
        (var_psi, var_psi_dn0, var_psi_dn1, var_psi_dn2, var_psi_dn3, var_psi_dn4, var_psi_dn5, var_psi_dn6, var_psi_dn7, var_psi_dn8, var_psi_dn9, var_psi_dn10, var_psi_dn11, var_psi_dn12, var_psi_dn13, var_psi_dn14, var_psi_dn15, var_psi_db0, var_psi_db1, var_psi_db2, var_psi_db3, var_psi_db4, var_psi_db5, var_psi_db6, var_psi_db7, var_psi_db8, var_psi_db9, var_psi_db10, var_psi_db11, var_psi_db12, var_psi_db13, var_psi_db14,)
    }
};
        var_psi = assign870_e1263;
        var_psi_dn0 = assign870_e1263_d_n0;
        var_psi_dn1 = assign870_e1263_d_n1;
        var_psi_dn2 = assign870_e1263_d_n2;
        var_psi_dn3 = assign870_e1263_d_n3;
        var_psi_dn4 = assign870_e1263_d_n4;
        var_psi_dn5 = assign870_e1263_d_n5;
        var_psi_dn6 = assign870_e1263_d_n6;
        var_psi_dn7 = assign870_e1263_d_n7;
        var_psi_dn8 = assign870_e1263_d_n8;
        var_psi_dn9 = assign870_e1263_d_n9;
        var_psi_dn10 = assign870_e1263_d_n10;
        var_psi_dn11 = assign870_e1263_d_n11;
        var_psi_dn12 = assign870_e1263_d_n12;
        var_psi_dn13 = assign870_e1263_d_n13;
        var_psi_dn14 = assign870_e1263_d_n14;
        var_psi_dn15 = assign870_e1263_d_n15;
        var_psi_db0 = assign870_e1263_d_b0;
        var_psi_db1 = assign870_e1263_d_b1;
        var_psi_db2 = assign870_e1263_d_b2;
        var_psi_db3 = assign870_e1263_d_b3;
        var_psi_db4 = assign870_e1263_d_b4;
        var_psi_db5 = assign870_e1263_d_b5;
        var_psi_db6 = assign870_e1263_d_b6;
        var_psi_db7 = assign870_e1263_d_b7;
        var_psi_db8 = assign870_e1263_d_b8;
        var_psi_db9 = assign870_e1263_d_b9;
        var_psi_db10 = assign870_e1263_d_b10;
        var_psi_db11 = assign870_e1263_d_b11;
        var_psi_db12 = assign870_e1263_d_b12;
        var_psi_db13 = assign870_e1263_d_b13;
        var_psi_db14 = assign870_e1263_d_b14;

        let (assign880_e1276, assign880_e1276_d_n0, assign880_e1276_d_n1, assign880_e1276_d_n2, assign880_e1276_d_n3, assign880_e1276_d_n4, assign880_e1276_d_n5, assign880_e1276_d_n6, assign880_e1276_d_n7, assign880_e1276_d_n8, assign880_e1276_d_n9, assign880_e1276_d_n10, assign880_e1276_d_n11, assign880_e1276_d_n12, assign880_e1276_d_n13, assign880_e1276_d_n14, assign880_e1276_d_n15, assign880_e1276_d_b0, assign880_e1276_d_b1, assign880_e1276_d_b2, assign880_e1276_d_b3, assign880_e1276_d_b4, assign880_e1276_d_b5, assign880_e1276_d_b6, assign880_e1276_d_b7, assign880_e1276_d_b8, assign880_e1276_d_b9, assign880_e1276_d_b10, assign880_e1276_d_b11, assign880_e1276_d_b12, assign880_e1276_d_b13, assign880_e1276_d_b14,) = {
    if ((var_guard8 != 0.0) && (!(((var_guard5 != 0.0) || (var_guard6 != 0.0)) || (var_guard7 != 0.0)))) {
        let assign880_e1274: f64 = (var_vgd - var_vpkm);
        (assign880_e1274, (var_vgd_dn0 - var_vpkm_dn0), (var_vgd_dn1 - var_vpkm_dn1), (var_vgd_dn2 - var_vpkm_dn2), (var_vgd_dn3 - var_vpkm_dn3), (var_vgd_dn4 - var_vpkm_dn4), (var_vgd_dn5 - var_vpkm_dn5), (var_vgd_dn6 - var_vpkm_dn6), (var_vgd_dn7 - var_vpkm_dn7), (var_vgd_dn8 - var_vpkm_dn8), (var_vgd_dn9 - var_vpkm_dn9), (var_vgd_dn10 - var_vpkm_dn10), (var_vgd_dn11 - var_vpkm_dn11), (var_vgd_dn12 - var_vpkm_dn12), (var_vgd_dn13 - var_vpkm_dn13), (var_vgd_dn14 - var_vpkm_dn14), (var_vgd_dn15 - var_vpkm_dn15), (var_vgd_db0 - var_vpkm_db0), (var_vgd_db1 - var_vpkm_db1), (var_vgd_db2 - var_vpkm_db2), (var_vgd_db3 - var_vpkm_db3), (var_vgd_db4 - var_vpkm_db4), (var_vgd_db5 - var_vpkm_db5), (var_vgd_db6 - var_vpkm_db6), (var_vgd_db7 - var_vpkm_db7), (var_vgd_db8 - var_vpkm_db8), (var_vgd_db9 - var_vpkm_db9), (var_vgd_db10 - var_vpkm_db10), (var_vgd_db11 - var_vpkm_db11), (var_vgd_db12 - var_vpkm_db12), (var_vgd_db13 - var_vpkm_db13), (var_vgd_db14 - var_vpkm_db14),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn1, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn13, var_t2_dn14, var_t2_dn15, var_t2_db0, var_t2_db1, var_t2_db2, var_t2_db3, var_t2_db4, var_t2_db5, var_t2_db6, var_t2_db7, var_t2_db8, var_t2_db9, var_t2_db10, var_t2_db11, var_t2_db12, var_t2_db13, var_t2_db14,)
    }
};
        var_t2 = assign880_e1276;
        var_t2_dn0 = assign880_e1276_d_n0;
        var_t2_dn1 = assign880_e1276_d_n1;
        var_t2_dn2 = assign880_e1276_d_n2;
        var_t2_dn3 = assign880_e1276_d_n3;
        var_t2_dn4 = assign880_e1276_d_n4;
        var_t2_dn5 = assign880_e1276_d_n5;
        var_t2_dn6 = assign880_e1276_d_n6;
        var_t2_dn7 = assign880_e1276_d_n7;
        var_t2_dn8 = assign880_e1276_d_n8;
        var_t2_dn9 = assign880_e1276_d_n9;
        var_t2_dn10 = assign880_e1276_d_n10;
        var_t2_dn11 = assign880_e1276_d_n11;
        var_t2_dn12 = assign880_e1276_d_n12;
        var_t2_dn13 = assign880_e1276_d_n13;
        var_t2_dn14 = assign880_e1276_d_n14;
        var_t2_dn15 = assign880_e1276_d_n15;
        var_t2_db0 = assign880_e1276_d_b0;
        var_t2_db1 = assign880_e1276_d_b1;
        var_t2_db2 = assign880_e1276_d_b2;
        var_t2_db3 = assign880_e1276_d_b3;
        var_t2_db4 = assign880_e1276_d_b4;
        var_t2_db5 = assign880_e1276_d_b5;
        var_t2_db6 = assign880_e1276_d_b6;
        var_t2_db7 = assign880_e1276_d_b7;
        var_t2_db8 = assign880_e1276_d_b8;
        var_t2_db9 = assign880_e1276_d_b9;
        var_t2_db10 = assign880_e1276_d_b10;
        var_t2_db11 = assign880_e1276_d_b11;
        var_t2_db12 = assign880_e1276_d_b12;
        var_t2_db13 = assign880_e1276_d_b13;
        var_t2_db14 = assign880_e1276_d_b14;

        let (assign910_e1333, assign910_e1333_d_n0, assign910_e1333_d_n1, assign910_e1333_d_n2, assign910_e1333_d_n3, assign910_e1333_d_n4, assign910_e1333_d_n5, assign910_e1333_d_n6, assign910_e1333_d_n7, assign910_e1333_d_n8, assign910_e1333_d_n9, assign910_e1333_d_n10, assign910_e1333_d_n11, assign910_e1333_d_n12, assign910_e1333_d_n13, assign910_e1333_d_n14, assign910_e1333_d_n15, assign910_e1333_d_b0, assign910_e1333_d_b1, assign910_e1333_d_b2, assign910_e1333_d_b3, assign910_e1333_d_b4, assign910_e1333_d_b5, assign910_e1333_d_b6, assign910_e1333_d_b7, assign910_e1333_d_b8, assign910_e1333_d_b9, assign910_e1333_d_b10, assign910_e1333_d_b11, assign910_e1333_d_b12, assign910_e1333_d_b13, assign910_e1333_d_b14,) = {
    if ((var_guard8 != 0.0) && (!(((var_guard5 != 0.0) || (var_guard6 != 0.0)) || (var_guard7 != 0.0)))) {
        let assign910_e1324: f64 = { let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign910_e1326: f64 = (-var_psi);
        let assign910_e1327: f64 = { let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign910_e1328: f64 = (assign910_e1324 - assign910_e1327);
        let assign910_e1329: f64 = (0.5 * assign910_e1328);
        let assign910_e1330: f64 = (assign910_e1329).tanh();
        let assign910_e1331: f64 = (1.0 + assign910_e1330);
        (assign910_e1331, ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn0) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn0)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn1) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn1)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn2) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn2)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn3) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn3)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn4) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn4)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn5) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn5)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn6) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn6)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn7) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn7)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn8) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn8)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn9) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn9)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn10) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn10)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn11) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn11)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn12) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn12)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn13) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn13)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn14) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn14)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_dn15) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_dn15)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db0) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db0)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db1) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db1)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db2) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db2)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db3) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db3)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db4) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db4)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db5) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db5)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db6) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db6)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db7) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db7)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db8) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db8)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db9) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db9)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db10) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db10)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db11) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db11)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db12) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db12)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db13) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db13)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * var_psi_db14) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-var_psi_db14)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())),)
    } else {
        (var_tanh_psi1, var_tanh_psi1_dn0, var_tanh_psi1_dn1, var_tanh_psi1_dn2, var_tanh_psi1_dn3, var_tanh_psi1_dn4, var_tanh_psi1_dn5, var_tanh_psi1_dn6, var_tanh_psi1_dn7, var_tanh_psi1_dn8, var_tanh_psi1_dn9, var_tanh_psi1_dn10, var_tanh_psi1_dn11, var_tanh_psi1_dn12, var_tanh_psi1_dn13, var_tanh_psi1_dn14, var_tanh_psi1_dn15, var_tanh_psi1_db0, var_tanh_psi1_db1, var_tanh_psi1_db2, var_tanh_psi1_db3, var_tanh_psi1_db4, var_tanh_psi1_db5, var_tanh_psi1_db6, var_tanh_psi1_db7, var_tanh_psi1_db8, var_tanh_psi1_db9, var_tanh_psi1_db10, var_tanh_psi1_db11, var_tanh_psi1_db12, var_tanh_psi1_db13, var_tanh_psi1_db14,)
    }
};
        var_tanh_psi1 = assign910_e1333;
        var_tanh_psi1_dn0 = assign910_e1333_d_n0;
        var_tanh_psi1_dn1 = assign910_e1333_d_n1;
        var_tanh_psi1_dn2 = assign910_e1333_d_n2;
        var_tanh_psi1_dn3 = assign910_e1333_d_n3;
        var_tanh_psi1_dn4 = assign910_e1333_d_n4;
        var_tanh_psi1_dn5 = assign910_e1333_d_n5;
        var_tanh_psi1_dn6 = assign910_e1333_d_n6;
        var_tanh_psi1_dn7 = assign910_e1333_d_n7;
        var_tanh_psi1_dn8 = assign910_e1333_d_n8;
        var_tanh_psi1_dn9 = assign910_e1333_d_n9;
        var_tanh_psi1_dn10 = assign910_e1333_d_n10;
        var_tanh_psi1_dn11 = assign910_e1333_d_n11;
        var_tanh_psi1_dn12 = assign910_e1333_d_n12;
        var_tanh_psi1_dn13 = assign910_e1333_d_n13;
        var_tanh_psi1_dn14 = assign910_e1333_d_n14;
        var_tanh_psi1_dn15 = assign910_e1333_d_n15;
        var_tanh_psi1_db0 = assign910_e1333_d_b0;
        var_tanh_psi1_db1 = assign910_e1333_d_b1;
        var_tanh_psi1_db2 = assign910_e1333_d_b2;
        var_tanh_psi1_db3 = assign910_e1333_d_b3;
        var_tanh_psi1_db4 = assign910_e1333_d_b4;
        var_tanh_psi1_db5 = assign910_e1333_d_b5;
        var_tanh_psi1_db6 = assign910_e1333_d_b6;
        var_tanh_psi1_db7 = assign910_e1333_d_b7;
        var_tanh_psi1_db8 = assign910_e1333_d_b8;
        var_tanh_psi1_db9 = assign910_e1333_d_b9;
        var_tanh_psi1_db10 = assign910_e1333_d_b10;
        var_tanh_psi1_db11 = assign910_e1333_d_b11;
        var_tanh_psi1_db12 = assign910_e1333_d_b12;
        var_tanh_psi1_db13 = assign910_e1333_d_b13;
        var_tanh_psi1_db14 = assign910_e1333_d_b14;

        let assign1020_e1517: f64 = if ((p.p4 == 0.0) || (p.p4 == 1.0)) { 1.0 } else { 0.0 };
        var_guard9 = assign1020_e1517;

        let (assign1040_e1535, assign1040_e1535_d_n0, assign1040_e1535_d_n1, assign1040_e1535_d_n2, assign1040_e1535_d_n3, assign1040_e1535_d_n4, assign1040_e1535_d_n5, assign1040_e1535_d_n6, assign1040_e1535_d_n7, assign1040_e1535_d_n8, assign1040_e1535_d_n9, assign1040_e1535_d_n10, assign1040_e1535_d_n11, assign1040_e1535_d_n12, assign1040_e1535_d_n13, assign1040_e1535_d_n14, assign1040_e1535_d_n15, assign1040_e1535_d_b0, assign1040_e1535_d_b1, assign1040_e1535_d_b2, assign1040_e1535_d_b3, assign1040_e1535_d_b4, assign1040_e1535_d_b5, assign1040_e1535_d_b6, assign1040_e1535_d_b7, assign1040_e1535_d_b8, assign1040_e1535_d_b9, assign1040_e1535_d_b10, assign1040_e1535_d_b11, assign1040_e1535_d_b12, assign1040_e1535_d_b13, assign1040_e1535_d_b14,) = {
    if (var_guard9 != 0.0) {
        let assign1040_e1532: f64 = (p.p44 * var_tanh_psi);
        let assign1040_e1533: f64 = (p.p43 + assign1040_e1532);
        (assign1040_e1533, (p.p44 * var_tanh_psi_dn0), (p.p44 * var_tanh_psi_dn1), (p.p44 * var_tanh_psi_dn2), (p.p44 * var_tanh_psi_dn3), (p.p44 * var_tanh_psi_dn4), (p.p44 * var_tanh_psi_dn5), (p.p44 * var_tanh_psi_dn6), (p.p44 * var_tanh_psi_dn7), (p.p44 * var_tanh_psi_dn8), (p.p44 * var_tanh_psi_dn9), (p.p44 * var_tanh_psi_dn10), (p.p44 * var_tanh_psi_dn11), (p.p44 * var_tanh_psi_dn12), (p.p44 * var_tanh_psi_dn13), (p.p44 * var_tanh_psi_dn14), (p.p44 * var_tanh_psi_dn15), (p.p44 * var_tanh_psi_db0), (p.p44 * var_tanh_psi_db1), (p.p44 * var_tanh_psi_db2), (p.p44 * var_tanh_psi_db3), (p.p44 * var_tanh_psi_db4), (p.p44 * var_tanh_psi_db5), (p.p44 * var_tanh_psi_db6), (p.p44 * var_tanh_psi_db7), (p.p44 * var_tanh_psi_db8), (p.p44 * var_tanh_psi_db9), (p.p44 * var_tanh_psi_db10), (p.p44 * var_tanh_psi_db11), (p.p44 * var_tanh_psi_db12), (p.p44 * var_tanh_psi_db13), (p.p44 * var_tanh_psi_db14),)
    } else {
        (var_rd1, var_rd1_dn0, var_rd1_dn1, var_rd1_dn2, var_rd1_dn3, var_rd1_dn4, var_rd1_dn5, var_rd1_dn6, var_rd1_dn7, var_rd1_dn8, var_rd1_dn9, var_rd1_dn10, var_rd1_dn11, var_rd1_dn12, var_rd1_dn13, var_rd1_dn14, var_rd1_dn15, var_rd1_db0, var_rd1_db1, var_rd1_db2, var_rd1_db3, var_rd1_db4, var_rd1_db5, var_rd1_db6, var_rd1_db7, var_rd1_db8, var_rd1_db9, var_rd1_db10, var_rd1_db11, var_rd1_db12, var_rd1_db13, var_rd1_db14,)
    }
};
        var_rd1 = assign1040_e1535;
        var_rd1_dn0 = assign1040_e1535_d_n0;
        var_rd1_dn1 = assign1040_e1535_d_n1;
        var_rd1_dn2 = assign1040_e1535_d_n2;
        var_rd1_dn3 = assign1040_e1535_d_n3;
        var_rd1_dn4 = assign1040_e1535_d_n4;
        var_rd1_dn5 = assign1040_e1535_d_n5;
        var_rd1_dn6 = assign1040_e1535_d_n6;
        var_rd1_dn7 = assign1040_e1535_d_n7;
        var_rd1_dn8 = assign1040_e1535_d_n8;
        var_rd1_dn9 = assign1040_e1535_d_n9;
        var_rd1_dn10 = assign1040_e1535_d_n10;
        var_rd1_dn11 = assign1040_e1535_d_n11;
        var_rd1_dn12 = assign1040_e1535_d_n12;
        var_rd1_dn13 = assign1040_e1535_d_n13;
        var_rd1_dn14 = assign1040_e1535_d_n14;
        var_rd1_dn15 = assign1040_e1535_d_n15;
        var_rd1_db0 = assign1040_e1535_d_b0;
        var_rd1_db1 = assign1040_e1535_d_b1;
        var_rd1_db2 = assign1040_e1535_d_b2;
        var_rd1_db3 = assign1040_e1535_d_b3;
        var_rd1_db4 = assign1040_e1535_d_b4;
        var_rd1_db5 = assign1040_e1535_d_b5;
        var_rd1_db6 = assign1040_e1535_d_b6;
        var_rd1_db7 = assign1040_e1535_d_b7;
        var_rd1_db8 = assign1040_e1535_d_b8;
        var_rd1_db9 = assign1040_e1535_d_b9;
        var_rd1_db10 = assign1040_e1535_d_b10;
        var_rd1_db11 = assign1040_e1535_d_b11;
        var_rd1_db12 = assign1040_e1535_d_b12;
        var_rd1_db13 = assign1040_e1535_d_b13;
        var_rd1_db14 = assign1040_e1535_d_b14;

        let (assign1050_e1543, assign1050_e1543_d_n0, assign1050_e1543_d_n1, assign1050_e1543_d_n2, assign1050_e1543_d_n3, assign1050_e1543_d_n4, assign1050_e1543_d_n5, assign1050_e1543_d_n6, assign1050_e1543_d_n7, assign1050_e1543_d_n8, assign1050_e1543_d_n9, assign1050_e1543_d_n10, assign1050_e1543_d_n11, assign1050_e1543_d_n12, assign1050_e1543_d_n13, assign1050_e1543_d_n14, assign1050_e1543_d_n15, assign1050_e1543_d_b0, assign1050_e1543_d_b1, assign1050_e1543_d_b2, assign1050_e1543_d_b3, assign1050_e1543_d_b4, assign1050_e1543_d_b5, assign1050_e1543_d_b6, assign1050_e1543_d_b7, assign1050_e1543_d_b8, assign1050_e1543_d_b9, assign1050_e1543_d_b10, assign1050_e1543_d_b11, assign1050_e1543_d_b12, assign1050_e1543_d_b13, assign1050_e1543_d_b14,) = {
    if (var_guard9 != 0.0) {
        let assign1050_e1540: f64 = (p.p44 * var_tanh_psi);
        let assign1050_e1541: f64 = (p.p46 + assign1050_e1540);
        (assign1050_e1541, (p.p44 * var_tanh_psi_dn0), (p.p44 * var_tanh_psi_dn1), (p.p44 * var_tanh_psi_dn2), (p.p44 * var_tanh_psi_dn3), (p.p44 * var_tanh_psi_dn4), (p.p44 * var_tanh_psi_dn5), (p.p44 * var_tanh_psi_dn6), (p.p44 * var_tanh_psi_dn7), (p.p44 * var_tanh_psi_dn8), (p.p44 * var_tanh_psi_dn9), (p.p44 * var_tanh_psi_dn10), (p.p44 * var_tanh_psi_dn11), (p.p44 * var_tanh_psi_dn12), (p.p44 * var_tanh_psi_dn13), (p.p44 * var_tanh_psi_dn14), (p.p44 * var_tanh_psi_dn15), (p.p44 * var_tanh_psi_db0), (p.p44 * var_tanh_psi_db1), (p.p44 * var_tanh_psi_db2), (p.p44 * var_tanh_psi_db3), (p.p44 * var_tanh_psi_db4), (p.p44 * var_tanh_psi_db5), (p.p44 * var_tanh_psi_db6), (p.p44 * var_tanh_psi_db7), (p.p44 * var_tanh_psi_db8), (p.p44 * var_tanh_psi_db9), (p.p44 * var_tanh_psi_db10), (p.p44 * var_tanh_psi_db11), (p.p44 * var_tanh_psi_db12), (p.p44 * var_tanh_psi_db13), (p.p44 * var_tanh_psi_db14),)
    } else {
        (var_rs1, var_rs1_dn0, var_rs1_dn1, var_rs1_dn2, var_rs1_dn3, var_rs1_dn4, var_rs1_dn5, var_rs1_dn6, var_rs1_dn7, var_rs1_dn8, var_rs1_dn9, var_rs1_dn10, var_rs1_dn11, var_rs1_dn12, var_rs1_dn13, var_rs1_dn14, var_rs1_dn15, var_rs1_db0, var_rs1_db1, var_rs1_db2, var_rs1_db3, var_rs1_db4, var_rs1_db5, var_rs1_db6, var_rs1_db7, var_rs1_db8, var_rs1_db9, var_rs1_db10, var_rs1_db11, var_rs1_db12, var_rs1_db13, var_rs1_db14,)
    }
};
        var_rs1 = assign1050_e1543;
        var_rs1_dn0 = assign1050_e1543_d_n0;
        var_rs1_dn1 = assign1050_e1543_d_n1;
        var_rs1_dn2 = assign1050_e1543_d_n2;
        var_rs1_dn3 = assign1050_e1543_d_n3;
        var_rs1_dn4 = assign1050_e1543_d_n4;
        var_rs1_dn5 = assign1050_e1543_d_n5;
        var_rs1_dn6 = assign1050_e1543_d_n6;
        var_rs1_dn7 = assign1050_e1543_d_n7;
        var_rs1_dn8 = assign1050_e1543_d_n8;
        var_rs1_dn9 = assign1050_e1543_d_n9;
        var_rs1_dn10 = assign1050_e1543_d_n10;
        var_rs1_dn11 = assign1050_e1543_d_n11;
        var_rs1_dn12 = assign1050_e1543_d_n12;
        var_rs1_dn13 = assign1050_e1543_d_n13;
        var_rs1_dn14 = assign1050_e1543_d_n14;
        var_rs1_dn15 = assign1050_e1543_d_n15;
        var_rs1_db0 = assign1050_e1543_d_b0;
        var_rs1_db1 = assign1050_e1543_d_b1;
        var_rs1_db2 = assign1050_e1543_d_b2;
        var_rs1_db3 = assign1050_e1543_d_b3;
        var_rs1_db4 = assign1050_e1543_d_b4;
        var_rs1_db5 = assign1050_e1543_d_b5;
        var_rs1_db6 = assign1050_e1543_d_b6;
        var_rs1_db7 = assign1050_e1543_d_b7;
        var_rs1_db8 = assign1050_e1543_d_b8;
        var_rs1_db9 = assign1050_e1543_d_b9;
        var_rs1_db10 = assign1050_e1543_d_b10;
        var_rs1_db11 = assign1050_e1543_d_b11;
        var_rs1_db12 = assign1050_e1543_d_b12;
        var_rs1_db13 = assign1050_e1543_d_b13;
        var_rs1_db14 = assign1050_e1543_d_b14;

        let (assign1070_e1563, assign1070_e1563_d_n0, assign1070_e1563_d_n1, assign1070_e1563_d_n2, assign1070_e1563_d_n3, assign1070_e1563_d_n4, assign1070_e1563_d_n5, assign1070_e1563_d_n6, assign1070_e1563_d_n7, assign1070_e1563_d_n8, assign1070_e1563_d_n9, assign1070_e1563_d_n10, assign1070_e1563_d_n11, assign1070_e1563_d_n12, assign1070_e1563_d_n13, assign1070_e1563_d_n14, assign1070_e1563_d_n15, assign1070_e1563_d_b0, assign1070_e1563_d_b1, assign1070_e1563_d_b2, assign1070_e1563_d_b3, assign1070_e1563_d_b4, assign1070_e1563_d_b5, assign1070_e1563_d_b6, assign1070_e1563_d_b7, assign1070_e1563_d_b8, assign1070_e1563_d_b9, assign1070_e1563_d_b10, assign1070_e1563_d_b11, assign1070_e1563_d_b12, assign1070_e1563_d_b13, assign1070_e1563_d_b14,) = {
    if (var_guard9 == 0.0) {
        let assign1070_e1560: f64 = (p.p44 * var_tanh_psi1);
        let assign1070_e1561: f64 = (p.p43 + assign1070_e1560);
        (assign1070_e1561, (p.p44 * var_tanh_psi1_dn0), (p.p44 * var_tanh_psi1_dn1), (p.p44 * var_tanh_psi1_dn2), (p.p44 * var_tanh_psi1_dn3), (p.p44 * var_tanh_psi1_dn4), (p.p44 * var_tanh_psi1_dn5), (p.p44 * var_tanh_psi1_dn6), (p.p44 * var_tanh_psi1_dn7), (p.p44 * var_tanh_psi1_dn8), (p.p44 * var_tanh_psi1_dn9), (p.p44 * var_tanh_psi1_dn10), (p.p44 * var_tanh_psi1_dn11), (p.p44 * var_tanh_psi1_dn12), (p.p44 * var_tanh_psi1_dn13), (p.p44 * var_tanh_psi1_dn14), (p.p44 * var_tanh_psi1_dn15), (p.p44 * var_tanh_psi1_db0), (p.p44 * var_tanh_psi1_db1), (p.p44 * var_tanh_psi1_db2), (p.p44 * var_tanh_psi1_db3), (p.p44 * var_tanh_psi1_db4), (p.p44 * var_tanh_psi1_db5), (p.p44 * var_tanh_psi1_db6), (p.p44 * var_tanh_psi1_db7), (p.p44 * var_tanh_psi1_db8), (p.p44 * var_tanh_psi1_db9), (p.p44 * var_tanh_psi1_db10), (p.p44 * var_tanh_psi1_db11), (p.p44 * var_tanh_psi1_db12), (p.p44 * var_tanh_psi1_db13), (p.p44 * var_tanh_psi1_db14),)
    } else {
        (var_rd1, var_rd1_dn0, var_rd1_dn1, var_rd1_dn2, var_rd1_dn3, var_rd1_dn4, var_rd1_dn5, var_rd1_dn6, var_rd1_dn7, var_rd1_dn8, var_rd1_dn9, var_rd1_dn10, var_rd1_dn11, var_rd1_dn12, var_rd1_dn13, var_rd1_dn14, var_rd1_dn15, var_rd1_db0, var_rd1_db1, var_rd1_db2, var_rd1_db3, var_rd1_db4, var_rd1_db5, var_rd1_db6, var_rd1_db7, var_rd1_db8, var_rd1_db9, var_rd1_db10, var_rd1_db11, var_rd1_db12, var_rd1_db13, var_rd1_db14,)
    }
};
        var_rd1 = assign1070_e1563;
        var_rd1_dn0 = assign1070_e1563_d_n0;
        var_rd1_dn1 = assign1070_e1563_d_n1;
        var_rd1_dn2 = assign1070_e1563_d_n2;
        var_rd1_dn3 = assign1070_e1563_d_n3;
        var_rd1_dn4 = assign1070_e1563_d_n4;
        var_rd1_dn5 = assign1070_e1563_d_n5;
        var_rd1_dn6 = assign1070_e1563_d_n6;
        var_rd1_dn7 = assign1070_e1563_d_n7;
        var_rd1_dn8 = assign1070_e1563_d_n8;
        var_rd1_dn9 = assign1070_e1563_d_n9;
        var_rd1_dn10 = assign1070_e1563_d_n10;
        var_rd1_dn11 = assign1070_e1563_d_n11;
        var_rd1_dn12 = assign1070_e1563_d_n12;
        var_rd1_dn13 = assign1070_e1563_d_n13;
        var_rd1_dn14 = assign1070_e1563_d_n14;
        var_rd1_dn15 = assign1070_e1563_d_n15;
        var_rd1_db0 = assign1070_e1563_d_b0;
        var_rd1_db1 = assign1070_e1563_d_b1;
        var_rd1_db2 = assign1070_e1563_d_b2;
        var_rd1_db3 = assign1070_e1563_d_b3;
        var_rd1_db4 = assign1070_e1563_d_b4;
        var_rd1_db5 = assign1070_e1563_d_b5;
        var_rd1_db6 = assign1070_e1563_d_b6;
        var_rd1_db7 = assign1070_e1563_d_b7;
        var_rd1_db8 = assign1070_e1563_d_b8;
        var_rd1_db9 = assign1070_e1563_d_b9;
        var_rd1_db10 = assign1070_e1563_d_b10;
        var_rd1_db11 = assign1070_e1563_d_b11;
        var_rd1_db12 = assign1070_e1563_d_b12;
        var_rd1_db13 = assign1070_e1563_d_b13;
        var_rd1_db14 = assign1070_e1563_d_b14;

        let (assign1080_e1572, assign1080_e1572_d_n0, assign1080_e1572_d_n1, assign1080_e1572_d_n2, assign1080_e1572_d_n3, assign1080_e1572_d_n4, assign1080_e1572_d_n5, assign1080_e1572_d_n6, assign1080_e1572_d_n7, assign1080_e1572_d_n8, assign1080_e1572_d_n9, assign1080_e1572_d_n10, assign1080_e1572_d_n11, assign1080_e1572_d_n12, assign1080_e1572_d_n13, assign1080_e1572_d_n14, assign1080_e1572_d_n15, assign1080_e1572_d_b0, assign1080_e1572_d_b1, assign1080_e1572_d_b2, assign1080_e1572_d_b3, assign1080_e1572_d_b4, assign1080_e1572_d_b5, assign1080_e1572_d_b6, assign1080_e1572_d_b7, assign1080_e1572_d_b8, assign1080_e1572_d_b9, assign1080_e1572_d_b10, assign1080_e1572_d_b11, assign1080_e1572_d_b12, assign1080_e1572_d_b13, assign1080_e1572_d_b14,) = {
    if (var_guard9 == 0.0) {
        let assign1080_e1569: f64 = (p.p44 * var_tanh_psi1);
        let assign1080_e1570: f64 = (p.p46 + assign1080_e1569);
        (assign1080_e1570, (p.p44 * var_tanh_psi1_dn0), (p.p44 * var_tanh_psi1_dn1), (p.p44 * var_tanh_psi1_dn2), (p.p44 * var_tanh_psi1_dn3), (p.p44 * var_tanh_psi1_dn4), (p.p44 * var_tanh_psi1_dn5), (p.p44 * var_tanh_psi1_dn6), (p.p44 * var_tanh_psi1_dn7), (p.p44 * var_tanh_psi1_dn8), (p.p44 * var_tanh_psi1_dn9), (p.p44 * var_tanh_psi1_dn10), (p.p44 * var_tanh_psi1_dn11), (p.p44 * var_tanh_psi1_dn12), (p.p44 * var_tanh_psi1_dn13), (p.p44 * var_tanh_psi1_dn14), (p.p44 * var_tanh_psi1_dn15), (p.p44 * var_tanh_psi1_db0), (p.p44 * var_tanh_psi1_db1), (p.p44 * var_tanh_psi1_db2), (p.p44 * var_tanh_psi1_db3), (p.p44 * var_tanh_psi1_db4), (p.p44 * var_tanh_psi1_db5), (p.p44 * var_tanh_psi1_db6), (p.p44 * var_tanh_psi1_db7), (p.p44 * var_tanh_psi1_db8), (p.p44 * var_tanh_psi1_db9), (p.p44 * var_tanh_psi1_db10), (p.p44 * var_tanh_psi1_db11), (p.p44 * var_tanh_psi1_db12), (p.p44 * var_tanh_psi1_db13), (p.p44 * var_tanh_psi1_db14),)
    } else {
        (var_rs1, var_rs1_dn0, var_rs1_dn1, var_rs1_dn2, var_rs1_dn3, var_rs1_dn4, var_rs1_dn5, var_rs1_dn6, var_rs1_dn7, var_rs1_dn8, var_rs1_dn9, var_rs1_dn10, var_rs1_dn11, var_rs1_dn12, var_rs1_dn13, var_rs1_dn14, var_rs1_dn15, var_rs1_db0, var_rs1_db1, var_rs1_db2, var_rs1_db3, var_rs1_db4, var_rs1_db5, var_rs1_db6, var_rs1_db7, var_rs1_db8, var_rs1_db9, var_rs1_db10, var_rs1_db11, var_rs1_db12, var_rs1_db13, var_rs1_db14,)
    }
};
        var_rs1 = assign1080_e1572;
        var_rs1_dn0 = assign1080_e1572_d_n0;
        var_rs1_dn1 = assign1080_e1572_d_n1;
        var_rs1_dn2 = assign1080_e1572_d_n2;
        var_rs1_dn3 = assign1080_e1572_d_n3;
        var_rs1_dn4 = assign1080_e1572_d_n4;
        var_rs1_dn5 = assign1080_e1572_d_n5;
        var_rs1_dn6 = assign1080_e1572_d_n6;
        var_rs1_dn7 = assign1080_e1572_d_n7;
        var_rs1_dn8 = assign1080_e1572_d_n8;
        var_rs1_dn9 = assign1080_e1572_d_n9;
        var_rs1_dn10 = assign1080_e1572_d_n10;
        var_rs1_dn11 = assign1080_e1572_d_n11;
        var_rs1_dn12 = assign1080_e1572_d_n12;
        var_rs1_dn13 = assign1080_e1572_d_n13;
        var_rs1_dn14 = assign1080_e1572_d_n14;
        var_rs1_dn15 = assign1080_e1572_d_n15;
        var_rs1_db0 = assign1080_e1572_d_b0;
        var_rs1_db1 = assign1080_e1572_d_b1;
        var_rs1_db2 = assign1080_e1572_d_b2;
        var_rs1_db3 = assign1080_e1572_d_b3;
        var_rs1_db4 = assign1080_e1572_d_b4;
        var_rs1_db5 = assign1080_e1572_d_b5;
        var_rs1_db6 = assign1080_e1572_d_b6;
        var_rs1_db7 = assign1080_e1572_d_b7;
        var_rs1_db8 = assign1080_e1572_d_b8;
        var_rs1_db9 = assign1080_e1572_d_b9;
        var_rs1_db10 = assign1080_e1572_d_b10;
        var_rs1_db11 = assign1080_e1572_d_b11;
        var_rs1_db12 = assign1080_e1572_d_b12;
        var_rs1_db13 = assign1080_e1572_d_b13;
        var_rs1_db14 = assign1080_e1572_d_b14;

        let assign1090_e1577: f64 = if ((var_delta_t != 0.0) || (p.p57 > 0.0)) { 1.0 } else { 0.0 };
        var_guard10 = assign1090_e1577;

        let (assign1100_e1587, assign1100_e1587_d_n0, assign1100_e1587_d_n1, assign1100_e1587_d_n2, assign1100_e1587_d_n3, assign1100_e1587_d_n4, assign1100_e1587_d_n5, assign1100_e1587_d_n6, assign1100_e1587_d_n7, assign1100_e1587_d_n8, assign1100_e1587_d_n9, assign1100_e1587_d_n10, assign1100_e1587_d_n11, assign1100_e1587_d_n12, assign1100_e1587_d_n13, assign1100_e1587_d_n14, assign1100_e1587_d_n15, assign1100_e1587_d_b0, assign1100_e1587_d_b1, assign1100_e1587_d_b2, assign1100_e1587_d_b3, assign1100_e1587_d_b4, assign1100_e1587_d_b5, assign1100_e1587_d_b6, assign1100_e1587_d_b7, assign1100_e1587_d_b8, assign1100_e1587_d_b9, assign1100_e1587_d_b10, assign1100_e1587_d_b11, assign1100_e1587_d_b12, assign1100_e1587_d_b13, assign1100_e1587_d_b14,) = {
    if (var_guard10 != 0.0) {
        let assign1100_e1583: f64 = (p.p66 * var_delta_t);
        let assign1100_e1584: f64 = (1.0 + assign1100_e1583);
        let assign1100_e1585: f64 = (var_rs1 * assign1100_e1584);
        (assign1100_e1585, ((var_rs1_dn0 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_dn0))), ((var_rs1_dn1 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_dn1))), ((var_rs1_dn2 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_dn2))), ((var_rs1_dn3 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_dn3))), ((var_rs1_dn4 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_dn4))), ((var_rs1_dn5 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_dn5))), ((var_rs1_dn6 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_dn6))), ((var_rs1_dn7 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_dn7))), ((var_rs1_dn8 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_dn8))), ((var_rs1_dn9 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_dn9))), ((var_rs1_dn10 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_dn10))), ((var_rs1_dn11 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_dn11))), ((var_rs1_dn12 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_dn12))), ((var_rs1_dn13 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_dn13))), ((var_rs1_dn14 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_dn14))), ((var_rs1_dn15 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_dn15))), ((var_rs1_db0 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_db0))), ((var_rs1_db1 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_db1))), ((var_rs1_db2 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_db2))), ((var_rs1_db3 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_db3))), ((var_rs1_db4 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_db4))), ((var_rs1_db5 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_db5))), ((var_rs1_db6 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_db6))), ((var_rs1_db7 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_db7))), ((var_rs1_db8 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_db8))), ((var_rs1_db9 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_db9))), ((var_rs1_db10 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_db10))), ((var_rs1_db11 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_db11))), ((var_rs1_db12 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_db12))), ((var_rs1_db13 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_db13))), ((var_rs1_db14 * assign1100_e1584) + (var_rs1 * (p.p66 * var_delta_t_db14))),)
    } else {
        (var_rs_t, var_rs_t_dn0, var_rs_t_dn1, var_rs_t_dn2, var_rs_t_dn3, var_rs_t_dn4, var_rs_t_dn5, var_rs_t_dn6, var_rs_t_dn7, var_rs_t_dn8, var_rs_t_dn9, var_rs_t_dn10, var_rs_t_dn11, var_rs_t_dn12, var_rs_t_dn13, var_rs_t_dn14, var_rs_t_dn15, var_rs_t_db0, var_rs_t_db1, var_rs_t_db2, var_rs_t_db3, var_rs_t_db4, var_rs_t_db5, var_rs_t_db6, var_rs_t_db7, var_rs_t_db8, var_rs_t_db9, var_rs_t_db10, var_rs_t_db11, var_rs_t_db12, var_rs_t_db13, var_rs_t_db14,)
    }
};
        var_rs_t = assign1100_e1587;
        var_rs_t_dn0 = assign1100_e1587_d_n0;
        var_rs_t_dn1 = assign1100_e1587_d_n1;
        var_rs_t_dn2 = assign1100_e1587_d_n2;
        var_rs_t_dn3 = assign1100_e1587_d_n3;
        var_rs_t_dn4 = assign1100_e1587_d_n4;
        var_rs_t_dn5 = assign1100_e1587_d_n5;
        var_rs_t_dn6 = assign1100_e1587_d_n6;
        var_rs_t_dn7 = assign1100_e1587_d_n7;
        var_rs_t_dn8 = assign1100_e1587_d_n8;
        var_rs_t_dn9 = assign1100_e1587_d_n9;
        var_rs_t_dn10 = assign1100_e1587_d_n10;
        var_rs_t_dn11 = assign1100_e1587_d_n11;
        var_rs_t_dn12 = assign1100_e1587_d_n12;
        var_rs_t_dn13 = assign1100_e1587_d_n13;
        var_rs_t_dn14 = assign1100_e1587_d_n14;
        var_rs_t_dn15 = assign1100_e1587_d_n15;
        var_rs_t_db0 = assign1100_e1587_d_b0;
        var_rs_t_db1 = assign1100_e1587_d_b1;
        var_rs_t_db2 = assign1100_e1587_d_b2;
        var_rs_t_db3 = assign1100_e1587_d_b3;
        var_rs_t_db4 = assign1100_e1587_d_b4;
        var_rs_t_db5 = assign1100_e1587_d_b5;
        var_rs_t_db6 = assign1100_e1587_d_b6;
        var_rs_t_db7 = assign1100_e1587_d_b7;
        var_rs_t_db8 = assign1100_e1587_d_b8;
        var_rs_t_db9 = assign1100_e1587_d_b9;
        var_rs_t_db10 = assign1100_e1587_d_b10;
        var_rs_t_db11 = assign1100_e1587_d_b11;
        var_rs_t_db12 = assign1100_e1587_d_b12;
        var_rs_t_db13 = assign1100_e1587_d_b13;
        var_rs_t_db14 = assign1100_e1587_d_b14;

        let (assign1110_e1597, assign1110_e1597_d_n0, assign1110_e1597_d_n1, assign1110_e1597_d_n2, assign1110_e1597_d_n3, assign1110_e1597_d_n4, assign1110_e1597_d_n5, assign1110_e1597_d_n6, assign1110_e1597_d_n7, assign1110_e1597_d_n8, assign1110_e1597_d_n9, assign1110_e1597_d_n10, assign1110_e1597_d_n11, assign1110_e1597_d_n12, assign1110_e1597_d_n13, assign1110_e1597_d_n14, assign1110_e1597_d_n15, assign1110_e1597_d_b0, assign1110_e1597_d_b1, assign1110_e1597_d_b2, assign1110_e1597_d_b3, assign1110_e1597_d_b4, assign1110_e1597_d_b5, assign1110_e1597_d_b6, assign1110_e1597_d_b7, assign1110_e1597_d_b8, assign1110_e1597_d_b9, assign1110_e1597_d_b10, assign1110_e1597_d_b11, assign1110_e1597_d_b12, assign1110_e1597_d_b13, assign1110_e1597_d_b14,) = {
    if (var_guard10 != 0.0) {
        let assign1110_e1593: f64 = (p.p66 * var_delta_t);
        let assign1110_e1594: f64 = (1.0 + assign1110_e1593);
        let assign1110_e1595: f64 = (var_rd1 * assign1110_e1594);
        (assign1110_e1595, ((var_rd1_dn0 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_dn0))), ((var_rd1_dn1 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_dn1))), ((var_rd1_dn2 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_dn2))), ((var_rd1_dn3 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_dn3))), ((var_rd1_dn4 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_dn4))), ((var_rd1_dn5 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_dn5))), ((var_rd1_dn6 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_dn6))), ((var_rd1_dn7 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_dn7))), ((var_rd1_dn8 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_dn8))), ((var_rd1_dn9 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_dn9))), ((var_rd1_dn10 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_dn10))), ((var_rd1_dn11 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_dn11))), ((var_rd1_dn12 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_dn12))), ((var_rd1_dn13 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_dn13))), ((var_rd1_dn14 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_dn14))), ((var_rd1_dn15 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_dn15))), ((var_rd1_db0 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_db0))), ((var_rd1_db1 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_db1))), ((var_rd1_db2 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_db2))), ((var_rd1_db3 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_db3))), ((var_rd1_db4 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_db4))), ((var_rd1_db5 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_db5))), ((var_rd1_db6 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_db6))), ((var_rd1_db7 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_db7))), ((var_rd1_db8 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_db8))), ((var_rd1_db9 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_db9))), ((var_rd1_db10 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_db10))), ((var_rd1_db11 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_db11))), ((var_rd1_db12 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_db12))), ((var_rd1_db13 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_db13))), ((var_rd1_db14 * assign1110_e1594) + (var_rd1 * (p.p66 * var_delta_t_db14))),)
    } else {
        (var_rd1_t, var_rd1_t_dn0, var_rd1_t_dn1, var_rd1_t_dn2, var_rd1_t_dn3, var_rd1_t_dn4, var_rd1_t_dn5, var_rd1_t_dn6, var_rd1_t_dn7, var_rd1_t_dn8, var_rd1_t_dn9, var_rd1_t_dn10, var_rd1_t_dn11, var_rd1_t_dn12, var_rd1_t_dn13, var_rd1_t_dn14, var_rd1_t_dn15, var_rd1_t_db0, var_rd1_t_db1, var_rd1_t_db2, var_rd1_t_db3, var_rd1_t_db4, var_rd1_t_db5, var_rd1_t_db6, var_rd1_t_db7, var_rd1_t_db8, var_rd1_t_db9, var_rd1_t_db10, var_rd1_t_db11, var_rd1_t_db12, var_rd1_t_db13, var_rd1_t_db14,)
    }
};
        var_rd1_t = assign1110_e1597;
        var_rd1_t_dn0 = assign1110_e1597_d_n0;
        var_rd1_t_dn1 = assign1110_e1597_d_n1;
        var_rd1_t_dn2 = assign1110_e1597_d_n2;
        var_rd1_t_dn3 = assign1110_e1597_d_n3;
        var_rd1_t_dn4 = assign1110_e1597_d_n4;
        var_rd1_t_dn5 = assign1110_e1597_d_n5;
        var_rd1_t_dn6 = assign1110_e1597_d_n6;
        var_rd1_t_dn7 = assign1110_e1597_d_n7;
        var_rd1_t_dn8 = assign1110_e1597_d_n8;
        var_rd1_t_dn9 = assign1110_e1597_d_n9;
        var_rd1_t_dn10 = assign1110_e1597_d_n10;
        var_rd1_t_dn11 = assign1110_e1597_d_n11;
        var_rd1_t_dn12 = assign1110_e1597_d_n12;
        var_rd1_t_dn13 = assign1110_e1597_d_n13;
        var_rd1_t_dn14 = assign1110_e1597_d_n14;
        var_rd1_t_dn15 = assign1110_e1597_d_n15;
        var_rd1_t_db0 = assign1110_e1597_d_b0;
        var_rd1_t_db1 = assign1110_e1597_d_b1;
        var_rd1_t_db2 = assign1110_e1597_d_b2;
        var_rd1_t_db3 = assign1110_e1597_d_b3;
        var_rd1_t_db4 = assign1110_e1597_d_b4;
        var_rd1_t_db5 = assign1110_e1597_d_b5;
        var_rd1_t_db6 = assign1110_e1597_d_b6;
        var_rd1_t_db7 = assign1110_e1597_d_b7;
        var_rd1_t_db8 = assign1110_e1597_d_b8;
        var_rd1_t_db9 = assign1110_e1597_d_b9;
        var_rd1_t_db10 = assign1110_e1597_d_b10;
        var_rd1_t_db11 = assign1110_e1597_d_b11;
        var_rd1_t_db12 = assign1110_e1597_d_b12;
        var_rd1_t_db13 = assign1110_e1597_d_b13;
        var_rd1_t_db14 = assign1110_e1597_d_b14;

        let (assign1130_e1612, assign1130_e1612_d_n0, assign1130_e1612_d_n1, assign1130_e1612_d_n2, assign1130_e1612_d_n3, assign1130_e1612_d_n4, assign1130_e1612_d_n5, assign1130_e1612_d_n6, assign1130_e1612_d_n7, assign1130_e1612_d_n8, assign1130_e1612_d_n9, assign1130_e1612_d_n10, assign1130_e1612_d_n11, assign1130_e1612_d_n12, assign1130_e1612_d_n13, assign1130_e1612_d_n14, assign1130_e1612_d_n15, assign1130_e1612_d_b0, assign1130_e1612_d_b1, assign1130_e1612_d_b2, assign1130_e1612_d_b3, assign1130_e1612_d_b4, assign1130_e1612_d_b5, assign1130_e1612_d_b6, assign1130_e1612_d_b7, assign1130_e1612_d_b8, assign1130_e1612_d_b9, assign1130_e1612_d_b10, assign1130_e1612_d_b11, assign1130_e1612_d_b12, assign1130_e1612_d_b13, assign1130_e1612_d_b14,) = {
    if (var_guard10 == 0.0) {
        (var_rd1, var_rd1_dn0, var_rd1_dn1, var_rd1_dn2, var_rd1_dn3, var_rd1_dn4, var_rd1_dn5, var_rd1_dn6, var_rd1_dn7, var_rd1_dn8, var_rd1_dn9, var_rd1_dn10, var_rd1_dn11, var_rd1_dn12, var_rd1_dn13, var_rd1_dn14, var_rd1_dn15, var_rd1_db0, var_rd1_db1, var_rd1_db2, var_rd1_db3, var_rd1_db4, var_rd1_db5, var_rd1_db6, var_rd1_db7, var_rd1_db8, var_rd1_db9, var_rd1_db10, var_rd1_db11, var_rd1_db12, var_rd1_db13, var_rd1_db14,)
    } else {
        (var_rd1_t, var_rd1_t_dn0, var_rd1_t_dn1, var_rd1_t_dn2, var_rd1_t_dn3, var_rd1_t_dn4, var_rd1_t_dn5, var_rd1_t_dn6, var_rd1_t_dn7, var_rd1_t_dn8, var_rd1_t_dn9, var_rd1_t_dn10, var_rd1_t_dn11, var_rd1_t_dn12, var_rd1_t_dn13, var_rd1_t_dn14, var_rd1_t_dn15, var_rd1_t_db0, var_rd1_t_db1, var_rd1_t_db2, var_rd1_t_db3, var_rd1_t_db4, var_rd1_t_db5, var_rd1_t_db6, var_rd1_t_db7, var_rd1_t_db8, var_rd1_t_db9, var_rd1_t_db10, var_rd1_t_db11, var_rd1_t_db12, var_rd1_t_db13, var_rd1_t_db14,)
    }
};
        var_rd1_t = assign1130_e1612;
        var_rd1_t_dn0 = assign1130_e1612_d_n0;
        var_rd1_t_dn1 = assign1130_e1612_d_n1;
        var_rd1_t_dn2 = assign1130_e1612_d_n2;
        var_rd1_t_dn3 = assign1130_e1612_d_n3;
        var_rd1_t_dn4 = assign1130_e1612_d_n4;
        var_rd1_t_dn5 = assign1130_e1612_d_n5;
        var_rd1_t_dn6 = assign1130_e1612_d_n6;
        var_rd1_t_dn7 = assign1130_e1612_d_n7;
        var_rd1_t_dn8 = assign1130_e1612_d_n8;
        var_rd1_t_dn9 = assign1130_e1612_d_n9;
        var_rd1_t_dn10 = assign1130_e1612_d_n10;
        var_rd1_t_dn11 = assign1130_e1612_d_n11;
        var_rd1_t_dn12 = assign1130_e1612_d_n12;
        var_rd1_t_dn13 = assign1130_e1612_d_n13;
        var_rd1_t_dn14 = assign1130_e1612_d_n14;
        var_rd1_t_dn15 = assign1130_e1612_d_n15;
        var_rd1_t_db0 = assign1130_e1612_d_b0;
        var_rd1_t_db1 = assign1130_e1612_d_b1;
        var_rd1_t_db2 = assign1130_e1612_d_b2;
        var_rd1_t_db3 = assign1130_e1612_d_b3;
        var_rd1_t_db4 = assign1130_e1612_d_b4;
        var_rd1_t_db5 = assign1130_e1612_d_b5;
        var_rd1_t_db6 = assign1130_e1612_d_b6;
        var_rd1_t_db7 = assign1130_e1612_d_b7;
        var_rd1_t_db8 = assign1130_e1612_d_b8;
        var_rd1_t_db9 = assign1130_e1612_d_b9;
        var_rd1_t_db10 = assign1130_e1612_d_b10;
        var_rd1_t_db11 = assign1130_e1612_d_b11;
        var_rd1_t_db12 = assign1130_e1612_d_b12;
        var_rd1_t_db13 = assign1130_e1612_d_b13;
        var_rd1_t_db14 = assign1130_e1612_d_b14;

        let (assign1140_e1617, assign1140_e1617_d_n0, assign1140_e1617_d_n1, assign1140_e1617_d_n2, assign1140_e1617_d_n3, assign1140_e1617_d_n4, assign1140_e1617_d_n5, assign1140_e1617_d_n6, assign1140_e1617_d_n7, assign1140_e1617_d_n8, assign1140_e1617_d_n9, assign1140_e1617_d_n10, assign1140_e1617_d_n11, assign1140_e1617_d_n12, assign1140_e1617_d_n13, assign1140_e1617_d_n14, assign1140_e1617_d_n15, assign1140_e1617_d_b0, assign1140_e1617_d_b1, assign1140_e1617_d_b2, assign1140_e1617_d_b3, assign1140_e1617_d_b4, assign1140_e1617_d_b5, assign1140_e1617_d_b6, assign1140_e1617_d_b7, assign1140_e1617_d_b8, assign1140_e1617_d_b9, assign1140_e1617_d_b10, assign1140_e1617_d_b11, assign1140_e1617_d_b12, assign1140_e1617_d_b13, assign1140_e1617_d_b14,) = {
    if (var_guard10 == 0.0) {
        (var_rs1, var_rs1_dn0, var_rs1_dn1, var_rs1_dn2, var_rs1_dn3, var_rs1_dn4, var_rs1_dn5, var_rs1_dn6, var_rs1_dn7, var_rs1_dn8, var_rs1_dn9, var_rs1_dn10, var_rs1_dn11, var_rs1_dn12, var_rs1_dn13, var_rs1_dn14, var_rs1_dn15, var_rs1_db0, var_rs1_db1, var_rs1_db2, var_rs1_db3, var_rs1_db4, var_rs1_db5, var_rs1_db6, var_rs1_db7, var_rs1_db8, var_rs1_db9, var_rs1_db10, var_rs1_db11, var_rs1_db12, var_rs1_db13, var_rs1_db14,)
    } else {
        (var_rs_t, var_rs_t_dn0, var_rs_t_dn1, var_rs_t_dn2, var_rs_t_dn3, var_rs_t_dn4, var_rs_t_dn5, var_rs_t_dn6, var_rs_t_dn7, var_rs_t_dn8, var_rs_t_dn9, var_rs_t_dn10, var_rs_t_dn11, var_rs_t_dn12, var_rs_t_dn13, var_rs_t_dn14, var_rs_t_dn15, var_rs_t_db0, var_rs_t_db1, var_rs_t_db2, var_rs_t_db3, var_rs_t_db4, var_rs_t_db5, var_rs_t_db6, var_rs_t_db7, var_rs_t_db8, var_rs_t_db9, var_rs_t_db10, var_rs_t_db11, var_rs_t_db12, var_rs_t_db13, var_rs_t_db14,)
    }
};
        var_rs_t = assign1140_e1617;
        var_rs_t_dn0 = assign1140_e1617_d_n0;
        var_rs_t_dn1 = assign1140_e1617_d_n1;
        var_rs_t_dn2 = assign1140_e1617_d_n2;
        var_rs_t_dn3 = assign1140_e1617_d_n3;
        var_rs_t_dn4 = assign1140_e1617_d_n4;
        var_rs_t_dn5 = assign1140_e1617_d_n5;
        var_rs_t_dn6 = assign1140_e1617_d_n6;
        var_rs_t_dn7 = assign1140_e1617_d_n7;
        var_rs_t_dn8 = assign1140_e1617_d_n8;
        var_rs_t_dn9 = assign1140_e1617_d_n9;
        var_rs_t_dn10 = assign1140_e1617_d_n10;
        var_rs_t_dn11 = assign1140_e1617_d_n11;
        var_rs_t_dn12 = assign1140_e1617_d_n12;
        var_rs_t_dn13 = assign1140_e1617_d_n13;
        var_rs_t_dn14 = assign1140_e1617_d_n14;
        var_rs_t_dn15 = assign1140_e1617_d_n15;
        var_rs_t_db0 = assign1140_e1617_d_b0;
        var_rs_t_db1 = assign1140_e1617_d_b1;
        var_rs_t_db2 = assign1140_e1617_d_b2;
        var_rs_t_db3 = assign1140_e1617_d_b3;
        var_rs_t_db4 = assign1140_e1617_d_b4;
        var_rs_t_db5 = assign1140_e1617_d_b5;
        var_rs_t_db6 = assign1140_e1617_d_b6;
        var_rs_t_db7 = assign1140_e1617_d_b7;
        var_rs_t_db8 = assign1140_e1617_d_b8;
        var_rs_t_db9 = assign1140_e1617_d_b9;
        var_rs_t_db10 = assign1140_e1617_d_b10;
        var_rs_t_db11 = assign1140_e1617_d_b11;
        var_rs_t_db12 = assign1140_e1617_d_b12;
        var_rs_t_db13 = assign1140_e1617_d_b13;
        var_rs_t_db14 = assign1140_e1617_d_b14;

        let assign1160_e1625: f64 = if p.p5 == 0.0 { 1.0 } else { 0.0 };
        var_guard11 = assign1160_e1625;


        *var_guard10_slot = var_guard10;
        *var_guard11_slot = var_guard11;
        *var_guard9_slot = var_guard9;
        *var_psi_slot = var_psi;
        *var_psi_db0_slot = var_psi_db0;
        *var_psi_db1_slot = var_psi_db1;
        *var_psi_db10_slot = var_psi_db10;
        *var_psi_db11_slot = var_psi_db11;
        *var_psi_db12_slot = var_psi_db12;
        *var_psi_db13_slot = var_psi_db13;
        *var_psi_db14_slot = var_psi_db14;
        *var_psi_db2_slot = var_psi_db2;
        *var_psi_db3_slot = var_psi_db3;
        *var_psi_db4_slot = var_psi_db4;
        *var_psi_db5_slot = var_psi_db5;
        *var_psi_db6_slot = var_psi_db6;
        *var_psi_db7_slot = var_psi_db7;
        *var_psi_db8_slot = var_psi_db8;
        *var_psi_db9_slot = var_psi_db9;
        *var_psi_dn0_slot = var_psi_dn0;
        *var_psi_dn1_slot = var_psi_dn1;
        *var_psi_dn10_slot = var_psi_dn10;
        *var_psi_dn11_slot = var_psi_dn11;
        *var_psi_dn12_slot = var_psi_dn12;
        *var_psi_dn13_slot = var_psi_dn13;
        *var_psi_dn14_slot = var_psi_dn14;
        *var_psi_dn15_slot = var_psi_dn15;
        *var_psi_dn2_slot = var_psi_dn2;
        *var_psi_dn3_slot = var_psi_dn3;
        *var_psi_dn4_slot = var_psi_dn4;
        *var_psi_dn5_slot = var_psi_dn5;
        *var_psi_dn6_slot = var_psi_dn6;
        *var_psi_dn7_slot = var_psi_dn7;
        *var_psi_dn8_slot = var_psi_dn8;
        *var_psi_dn9_slot = var_psi_dn9;
        *var_rd1_slot = var_rd1;
        *var_rd1_db0_slot = var_rd1_db0;
        *var_rd1_db1_slot = var_rd1_db1;
        *var_rd1_db10_slot = var_rd1_db10;
        *var_rd1_db11_slot = var_rd1_db11;
        *var_rd1_db12_slot = var_rd1_db12;
        *var_rd1_db13_slot = var_rd1_db13;
        *var_rd1_db14_slot = var_rd1_db14;
        *var_rd1_db2_slot = var_rd1_db2;
        *var_rd1_db3_slot = var_rd1_db3;
        *var_rd1_db4_slot = var_rd1_db4;
        *var_rd1_db5_slot = var_rd1_db5;
        *var_rd1_db6_slot = var_rd1_db6;
        *var_rd1_db7_slot = var_rd1_db7;
        *var_rd1_db8_slot = var_rd1_db8;
        *var_rd1_db9_slot = var_rd1_db9;
        *var_rd1_dn0_slot = var_rd1_dn0;
        *var_rd1_dn1_slot = var_rd1_dn1;
        *var_rd1_dn10_slot = var_rd1_dn10;
        *var_rd1_dn11_slot = var_rd1_dn11;
        *var_rd1_dn12_slot = var_rd1_dn12;
        *var_rd1_dn13_slot = var_rd1_dn13;
        *var_rd1_dn14_slot = var_rd1_dn14;
        *var_rd1_dn15_slot = var_rd1_dn15;
        *var_rd1_dn2_slot = var_rd1_dn2;
        *var_rd1_dn3_slot = var_rd1_dn3;
        *var_rd1_dn4_slot = var_rd1_dn4;
        *var_rd1_dn5_slot = var_rd1_dn5;
        *var_rd1_dn6_slot = var_rd1_dn6;
        *var_rd1_dn7_slot = var_rd1_dn7;
        *var_rd1_dn8_slot = var_rd1_dn8;
        *var_rd1_dn9_slot = var_rd1_dn9;
        *var_rd1_t_slot = var_rd1_t;
        *var_rd1_t_db0_slot = var_rd1_t_db0;
        *var_rd1_t_db1_slot = var_rd1_t_db1;
        *var_rd1_t_db10_slot = var_rd1_t_db10;
        *var_rd1_t_db11_slot = var_rd1_t_db11;
        *var_rd1_t_db12_slot = var_rd1_t_db12;
        *var_rd1_t_db13_slot = var_rd1_t_db13;
        *var_rd1_t_db14_slot = var_rd1_t_db14;
        *var_rd1_t_db2_slot = var_rd1_t_db2;
        *var_rd1_t_db3_slot = var_rd1_t_db3;
        *var_rd1_t_db4_slot = var_rd1_t_db4;
        *var_rd1_t_db5_slot = var_rd1_t_db5;
        *var_rd1_t_db6_slot = var_rd1_t_db6;
        *var_rd1_t_db7_slot = var_rd1_t_db7;
        *var_rd1_t_db8_slot = var_rd1_t_db8;
        *var_rd1_t_db9_slot = var_rd1_t_db9;
        *var_rd1_t_dn0_slot = var_rd1_t_dn0;
        *var_rd1_t_dn1_slot = var_rd1_t_dn1;
        *var_rd1_t_dn10_slot = var_rd1_t_dn10;
        *var_rd1_t_dn11_slot = var_rd1_t_dn11;
        *var_rd1_t_dn12_slot = var_rd1_t_dn12;
        *var_rd1_t_dn13_slot = var_rd1_t_dn13;
        *var_rd1_t_dn14_slot = var_rd1_t_dn14;
        *var_rd1_t_dn15_slot = var_rd1_t_dn15;
        *var_rd1_t_dn2_slot = var_rd1_t_dn2;
        *var_rd1_t_dn3_slot = var_rd1_t_dn3;
        *var_rd1_t_dn4_slot = var_rd1_t_dn4;
        *var_rd1_t_dn5_slot = var_rd1_t_dn5;
        *var_rd1_t_dn6_slot = var_rd1_t_dn6;
        *var_rd1_t_dn7_slot = var_rd1_t_dn7;
        *var_rd1_t_dn8_slot = var_rd1_t_dn8;
        *var_rd1_t_dn9_slot = var_rd1_t_dn9;
        *var_rs1_slot = var_rs1;
        *var_rs1_db0_slot = var_rs1_db0;
        *var_rs1_db1_slot = var_rs1_db1;
        *var_rs1_db10_slot = var_rs1_db10;
        *var_rs1_db11_slot = var_rs1_db11;
        *var_rs1_db12_slot = var_rs1_db12;
        *var_rs1_db13_slot = var_rs1_db13;
        *var_rs1_db14_slot = var_rs1_db14;
        *var_rs1_db2_slot = var_rs1_db2;
        *var_rs1_db3_slot = var_rs1_db3;
        *var_rs1_db4_slot = var_rs1_db4;
        *var_rs1_db5_slot = var_rs1_db5;
        *var_rs1_db6_slot = var_rs1_db6;
        *var_rs1_db7_slot = var_rs1_db7;
        *var_rs1_db8_slot = var_rs1_db8;
        *var_rs1_db9_slot = var_rs1_db9;
        *var_rs1_dn0_slot = var_rs1_dn0;
        *var_rs1_dn1_slot = var_rs1_dn1;
        *var_rs1_dn10_slot = var_rs1_dn10;
        *var_rs1_dn11_slot = var_rs1_dn11;
        *var_rs1_dn12_slot = var_rs1_dn12;
        *var_rs1_dn13_slot = var_rs1_dn13;
        *var_rs1_dn14_slot = var_rs1_dn14;
        *var_rs1_dn15_slot = var_rs1_dn15;
        *var_rs1_dn2_slot = var_rs1_dn2;
        *var_rs1_dn3_slot = var_rs1_dn3;
        *var_rs1_dn4_slot = var_rs1_dn4;
        *var_rs1_dn5_slot = var_rs1_dn5;
        *var_rs1_dn6_slot = var_rs1_dn6;
        *var_rs1_dn7_slot = var_rs1_dn7;
        *var_rs1_dn8_slot = var_rs1_dn8;
        *var_rs1_dn9_slot = var_rs1_dn9;
        *var_rs_t_slot = var_rs_t;
        *var_rs_t_db0_slot = var_rs_t_db0;
        *var_rs_t_db1_slot = var_rs_t_db1;
        *var_rs_t_db10_slot = var_rs_t_db10;
        *var_rs_t_db11_slot = var_rs_t_db11;
        *var_rs_t_db12_slot = var_rs_t_db12;
        *var_rs_t_db13_slot = var_rs_t_db13;
        *var_rs_t_db14_slot = var_rs_t_db14;
        *var_rs_t_db2_slot = var_rs_t_db2;
        *var_rs_t_db3_slot = var_rs_t_db3;
        *var_rs_t_db4_slot = var_rs_t_db4;
        *var_rs_t_db5_slot = var_rs_t_db5;
        *var_rs_t_db6_slot = var_rs_t_db6;
        *var_rs_t_db7_slot = var_rs_t_db7;
        *var_rs_t_db8_slot = var_rs_t_db8;
        *var_rs_t_db9_slot = var_rs_t_db9;
        *var_rs_t_dn0_slot = var_rs_t_dn0;
        *var_rs_t_dn1_slot = var_rs_t_dn1;
        *var_rs_t_dn10_slot = var_rs_t_dn10;
        *var_rs_t_dn11_slot = var_rs_t_dn11;
        *var_rs_t_dn12_slot = var_rs_t_dn12;
        *var_rs_t_dn13_slot = var_rs_t_dn13;
        *var_rs_t_dn14_slot = var_rs_t_dn14;
        *var_rs_t_dn15_slot = var_rs_t_dn15;
        *var_rs_t_dn2_slot = var_rs_t_dn2;
        *var_rs_t_dn3_slot = var_rs_t_dn3;
        *var_rs_t_dn4_slot = var_rs_t_dn4;
        *var_rs_t_dn5_slot = var_rs_t_dn5;
        *var_rs_t_dn6_slot = var_rs_t_dn6;
        *var_rs_t_dn7_slot = var_rs_t_dn7;
        *var_rs_t_dn8_slot = var_rs_t_dn8;
        *var_rs_t_dn9_slot = var_rs_t_dn9;
        *var_t2_slot = var_t2;
        *var_t2_db0_slot = var_t2_db0;
        *var_t2_db1_slot = var_t2_db1;
        *var_t2_db10_slot = var_t2_db10;
        *var_t2_db11_slot = var_t2_db11;
        *var_t2_db12_slot = var_t2_db12;
        *var_t2_db13_slot = var_t2_db13;
        *var_t2_db14_slot = var_t2_db14;
        *var_t2_db2_slot = var_t2_db2;
        *var_t2_db3_slot = var_t2_db3;
        *var_t2_db4_slot = var_t2_db4;
        *var_t2_db5_slot = var_t2_db5;
        *var_t2_db6_slot = var_t2_db6;
        *var_t2_db7_slot = var_t2_db7;
        *var_t2_db8_slot = var_t2_db8;
        *var_t2_db9_slot = var_t2_db9;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn1_slot = var_t2_dn1;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn15_slot = var_t2_dn15;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_tanh_psi1_slot = var_tanh_psi1;
        *var_tanh_psi1_db0_slot = var_tanh_psi1_db0;
        *var_tanh_psi1_db1_slot = var_tanh_psi1_db1;
        *var_tanh_psi1_db10_slot = var_tanh_psi1_db10;
        *var_tanh_psi1_db11_slot = var_tanh_psi1_db11;
        *var_tanh_psi1_db12_slot = var_tanh_psi1_db12;
        *var_tanh_psi1_db13_slot = var_tanh_psi1_db13;
        *var_tanh_psi1_db14_slot = var_tanh_psi1_db14;
        *var_tanh_psi1_db2_slot = var_tanh_psi1_db2;
        *var_tanh_psi1_db3_slot = var_tanh_psi1_db3;
        *var_tanh_psi1_db4_slot = var_tanh_psi1_db4;
        *var_tanh_psi1_db5_slot = var_tanh_psi1_db5;
        *var_tanh_psi1_db6_slot = var_tanh_psi1_db6;
        *var_tanh_psi1_db7_slot = var_tanh_psi1_db7;
        *var_tanh_psi1_db8_slot = var_tanh_psi1_db8;
        *var_tanh_psi1_db9_slot = var_tanh_psi1_db9;
        *var_tanh_psi1_dn0_slot = var_tanh_psi1_dn0;
        *var_tanh_psi1_dn1_slot = var_tanh_psi1_dn1;
        *var_tanh_psi1_dn10_slot = var_tanh_psi1_dn10;
        *var_tanh_psi1_dn11_slot = var_tanh_psi1_dn11;
        *var_tanh_psi1_dn12_slot = var_tanh_psi1_dn12;
        *var_tanh_psi1_dn13_slot = var_tanh_psi1_dn13;
        *var_tanh_psi1_dn14_slot = var_tanh_psi1_dn14;
        *var_tanh_psi1_dn15_slot = var_tanh_psi1_dn15;
        *var_tanh_psi1_dn2_slot = var_tanh_psi1_dn2;
        *var_tanh_psi1_dn3_slot = var_tanh_psi1_dn3;
        *var_tanh_psi1_dn4_slot = var_tanh_psi1_dn4;
        *var_tanh_psi1_dn5_slot = var_tanh_psi1_dn5;
        *var_tanh_psi1_dn6_slot = var_tanh_psi1_dn6;
        *var_tanh_psi1_dn7_slot = var_tanh_psi1_dn7;
        *var_tanh_psi1_dn8_slot = var_tanh_psi1_dn8;
        *var_tanh_psi1_dn9_slot = var_tanh_psi1_dn9;
    }

    pub(super) fn stamp_transient_block_5(
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
        var_guard11: f64,
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
        var_pg_param: f64,
        var_pg_param_db0: f64,
        var_pg_param_db1: f64,
        var_pg_param_db10: f64,
        var_pg_param_db11: f64,
        var_pg_param_db12: f64,
        var_pg_param_db13: f64,
        var_pg_param_db14: f64,
        var_pg_param_db2: f64,
        var_pg_param_db3: f64,
        var_pg_param_db4: f64,
        var_pg_param_db5: f64,
        var_pg_param_db6: f64,
        var_pg_param_db7: f64,
        var_pg_param_db8: f64,
        var_pg_param_db9: f64,
        var_pg_param_dn0: f64,
        var_pg_param_dn1: f64,
        var_pg_param_dn10: f64,
        var_pg_param_dn11: f64,
        var_pg_param_dn12: f64,
        var_pg_param_dn13: f64,
        var_pg_param_dn14: f64,
        var_pg_param_dn15: f64,
        var_pg_param_dn2: f64,
        var_pg_param_dn3: f64,
        var_pg_param_dn4: f64,
        var_pg_param_dn5: f64,
        var_pg_param_dn6: f64,
        var_pg_param_dn7: f64,
        var_pg_param_dn8: f64,
        var_pg_param_dn9: f64,
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
        var_vjg_t: f64,
        var_vjg_t_db0: f64,
        var_vjg_t_db1: f64,
        var_vjg_t_db10: f64,
        var_vjg_t_db11: f64,
        var_vjg_t_db12: f64,
        var_vjg_t_db13: f64,
        var_vjg_t_db14: f64,
        var_vjg_t_db2: f64,
        var_vjg_t_db3: f64,
        var_vjg_t_db4: f64,
        var_vjg_t_db5: f64,
        var_vjg_t_db6: f64,
        var_vjg_t_db7: f64,
        var_vjg_t_db8: f64,
        var_vjg_t_db9: f64,
        var_vjg_t_dn0: f64,
        var_vjg_t_dn1: f64,
        var_vjg_t_dn10: f64,
        var_vjg_t_dn11: f64,
        var_vjg_t_dn12: f64,
        var_vjg_t_dn13: f64,
        var_vjg_t_dn14: f64,
        var_vjg_t_dn15: f64,
        var_vjg_t_dn2: f64,
        var_vjg_t_dn3: f64,
        var_vjg_t_dn4: f64,
        var_vjg_t_dn5: f64,
        var_vjg_t_dn6: f64,
        var_vjg_t_dn7: f64,
        var_vjg_t_dn8: f64,
        var_vjg_t_dn9: f64,
        var_cgd_slot: &mut f64,
        var_cgd_db0_slot: &mut f64,
        var_cgd_db1_slot: &mut f64,
        var_cgd_db10_slot: &mut f64,
        var_cgd_db11_slot: &mut f64,
        var_cgd_db12_slot: &mut f64,
        var_cgd_db13_slot: &mut f64,
        var_cgd_db14_slot: &mut f64,
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
        var_cgs_slot: &mut f64,
        var_cgs_db0_slot: &mut f64,
        var_cgs_db1_slot: &mut f64,
        var_cgs_db10_slot: &mut f64,
        var_cgs_db11_slot: &mut f64,
        var_cgs_db12_slot: &mut f64,
        var_cgs_db13_slot: &mut f64,
        var_cgs_db14_slot: &mut f64,
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
        var_guard13_slot: &mut f64,
        var_guard14_slot: &mut f64,
        var_guard15_slot: &mut f64,
        var_psi_1_slot: &mut f64,
        var_psi_1_db0_slot: &mut f64,
        var_psi_1_db1_slot: &mut f64,
        var_psi_1_db10_slot: &mut f64,
        var_psi_1_db11_slot: &mut f64,
        var_psi_1_db12_slot: &mut f64,
        var_psi_1_db13_slot: &mut f64,
        var_psi_1_db14_slot: &mut f64,
        var_psi_1_db2_slot: &mut f64,
        var_psi_1_db3_slot: &mut f64,
        var_psi_1_db4_slot: &mut f64,
        var_psi_1_db5_slot: &mut f64,
        var_psi_1_db6_slot: &mut f64,
        var_psi_1_db7_slot: &mut f64,
        var_psi_1_db8_slot: &mut f64,
        var_psi_1_db9_slot: &mut f64,
        var_psi_1_dn0_slot: &mut f64,
        var_psi_1_dn1_slot: &mut f64,
        var_psi_1_dn10_slot: &mut f64,
        var_psi_1_dn11_slot: &mut f64,
        var_psi_1_dn12_slot: &mut f64,
        var_psi_1_dn13_slot: &mut f64,
        var_psi_1_dn14_slot: &mut f64,
        var_psi_1_dn15_slot: &mut f64,
        var_psi_1_dn2_slot: &mut f64,
        var_psi_1_dn3_slot: &mut f64,
        var_psi_1_dn4_slot: &mut f64,
        var_psi_1_dn5_slot: &mut f64,
        var_psi_1_dn6_slot: &mut f64,
        var_psi_1_dn7_slot: &mut f64,
        var_psi_1_dn8_slot: &mut f64,
        var_psi_1_dn9_slot: &mut f64,
        var_psi_2_slot: &mut f64,
        var_psi_2_db0_slot: &mut f64,
        var_psi_2_db1_slot: &mut f64,
        var_psi_2_db10_slot: &mut f64,
        var_psi_2_db11_slot: &mut f64,
        var_psi_2_db12_slot: &mut f64,
        var_psi_2_db13_slot: &mut f64,
        var_psi_2_db14_slot: &mut f64,
        var_psi_2_db2_slot: &mut f64,
        var_psi_2_db3_slot: &mut f64,
        var_psi_2_db4_slot: &mut f64,
        var_psi_2_db5_slot: &mut f64,
        var_psi_2_db6_slot: &mut f64,
        var_psi_2_db7_slot: &mut f64,
        var_psi_2_db8_slot: &mut f64,
        var_psi_2_db9_slot: &mut f64,
        var_psi_2_dn0_slot: &mut f64,
        var_psi_2_dn1_slot: &mut f64,
        var_psi_2_dn10_slot: &mut f64,
        var_psi_2_dn11_slot: &mut f64,
        var_psi_2_dn12_slot: &mut f64,
        var_psi_2_dn13_slot: &mut f64,
        var_psi_2_dn14_slot: &mut f64,
        var_psi_2_dn15_slot: &mut f64,
        var_psi_2_dn2_slot: &mut f64,
        var_psi_2_dn3_slot: &mut f64,
        var_psi_2_dn4_slot: &mut f64,
        var_psi_2_dn5_slot: &mut f64,
        var_psi_2_dn6_slot: &mut f64,
        var_psi_2_dn7_slot: &mut f64,
        var_psi_2_dn8_slot: &mut f64,
        var_psi_2_dn9_slot: &mut f64,
        var_psi_3_slot: &mut f64,
        var_psi_3_db0_slot: &mut f64,
        var_psi_3_db1_slot: &mut f64,
        var_psi_3_db10_slot: &mut f64,
        var_psi_3_db11_slot: &mut f64,
        var_psi_3_db12_slot: &mut f64,
        var_psi_3_db13_slot: &mut f64,
        var_psi_3_db14_slot: &mut f64,
        var_psi_3_db2_slot: &mut f64,
        var_psi_3_db3_slot: &mut f64,
        var_psi_3_db4_slot: &mut f64,
        var_psi_3_db5_slot: &mut f64,
        var_psi_3_db6_slot: &mut f64,
        var_psi_3_db7_slot: &mut f64,
        var_psi_3_db8_slot: &mut f64,
        var_psi_3_db9_slot: &mut f64,
        var_psi_3_dn0_slot: &mut f64,
        var_psi_3_dn1_slot: &mut f64,
        var_psi_3_dn10_slot: &mut f64,
        var_psi_3_dn11_slot: &mut f64,
        var_psi_3_dn12_slot: &mut f64,
        var_psi_3_dn13_slot: &mut f64,
        var_psi_3_dn14_slot: &mut f64,
        var_psi_3_dn15_slot: &mut f64,
        var_psi_3_dn2_slot: &mut f64,
        var_psi_3_dn3_slot: &mut f64,
        var_psi_3_dn4_slot: &mut f64,
        var_psi_3_dn5_slot: &mut f64,
        var_psi_3_dn6_slot: &mut f64,
        var_psi_3_dn7_slot: &mut f64,
        var_psi_3_dn8_slot: &mut f64,
        var_psi_3_dn9_slot: &mut f64,
        var_psi_4_slot: &mut f64,
        var_psi_4_db0_slot: &mut f64,
        var_psi_4_db1_slot: &mut f64,
        var_psi_4_db10_slot: &mut f64,
        var_psi_4_db11_slot: &mut f64,
        var_psi_4_db12_slot: &mut f64,
        var_psi_4_db13_slot: &mut f64,
        var_psi_4_db14_slot: &mut f64,
        var_psi_4_db2_slot: &mut f64,
        var_psi_4_db3_slot: &mut f64,
        var_psi_4_db4_slot: &mut f64,
        var_psi_4_db5_slot: &mut f64,
        var_psi_4_db6_slot: &mut f64,
        var_psi_4_db7_slot: &mut f64,
        var_psi_4_db8_slot: &mut f64,
        var_psi_4_db9_slot: &mut f64,
        var_psi_4_dn0_slot: &mut f64,
        var_psi_4_dn1_slot: &mut f64,
        var_psi_4_dn10_slot: &mut f64,
        var_psi_4_dn11_slot: &mut f64,
        var_psi_4_dn12_slot: &mut f64,
        var_psi_4_dn13_slot: &mut f64,
        var_psi_4_dn14_slot: &mut f64,
        var_psi_4_dn15_slot: &mut f64,
        var_psi_4_dn2_slot: &mut f64,
        var_psi_4_dn3_slot: &mut f64,
        var_psi_4_dn4_slot: &mut f64,
        var_psi_4_dn5_slot: &mut f64,
        var_psi_4_dn6_slot: &mut f64,
        var_psi_4_dn7_slot: &mut f64,
        var_psi_4_dn8_slot: &mut f64,
        var_psi_4_dn9_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_db0_slot: &mut f64,
        var_t0_db1_slot: &mut f64,
        var_t0_db10_slot: &mut f64,
        var_t0_db11_slot: &mut f64,
        var_t0_db12_slot: &mut f64,
        var_t0_db13_slot: &mut f64,
        var_t0_db14_slot: &mut f64,
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
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_tanh1_slot: &mut f64,
        var_tanh1_db0_slot: &mut f64,
        var_tanh1_db1_slot: &mut f64,
        var_tanh1_db10_slot: &mut f64,
        var_tanh1_db11_slot: &mut f64,
        var_tanh1_db12_slot: &mut f64,
        var_tanh1_db13_slot: &mut f64,
        var_tanh1_db14_slot: &mut f64,
        var_tanh1_db2_slot: &mut f64,
        var_tanh1_db3_slot: &mut f64,
        var_tanh1_db4_slot: &mut f64,
        var_tanh1_db5_slot: &mut f64,
        var_tanh1_db6_slot: &mut f64,
        var_tanh1_db7_slot: &mut f64,
        var_tanh1_db8_slot: &mut f64,
        var_tanh1_db9_slot: &mut f64,
        var_tanh1_dn0_slot: &mut f64,
        var_tanh1_dn1_slot: &mut f64,
        var_tanh1_dn10_slot: &mut f64,
        var_tanh1_dn11_slot: &mut f64,
        var_tanh1_dn12_slot: &mut f64,
        var_tanh1_dn13_slot: &mut f64,
        var_tanh1_dn14_slot: &mut f64,
        var_tanh1_dn15_slot: &mut f64,
        var_tanh1_dn2_slot: &mut f64,
        var_tanh1_dn3_slot: &mut f64,
        var_tanh1_dn4_slot: &mut f64,
        var_tanh1_dn5_slot: &mut f64,
        var_tanh1_dn6_slot: &mut f64,
        var_tanh1_dn7_slot: &mut f64,
        var_tanh1_dn8_slot: &mut f64,
        var_tanh1_dn9_slot: &mut f64,
        var_tanh2_slot: &mut f64,
        var_tanh2_db0_slot: &mut f64,
        var_tanh2_db1_slot: &mut f64,
        var_tanh2_db10_slot: &mut f64,
        var_tanh2_db11_slot: &mut f64,
        var_tanh2_db12_slot: &mut f64,
        var_tanh2_db13_slot: &mut f64,
        var_tanh2_db14_slot: &mut f64,
        var_tanh2_db2_slot: &mut f64,
        var_tanh2_db3_slot: &mut f64,
        var_tanh2_db4_slot: &mut f64,
        var_tanh2_db5_slot: &mut f64,
        var_tanh2_db6_slot: &mut f64,
        var_tanh2_db7_slot: &mut f64,
        var_tanh2_db8_slot: &mut f64,
        var_tanh2_db9_slot: &mut f64,
        var_tanh2_dn0_slot: &mut f64,
        var_tanh2_dn1_slot: &mut f64,
        var_tanh2_dn10_slot: &mut f64,
        var_tanh2_dn11_slot: &mut f64,
        var_tanh2_dn12_slot: &mut f64,
        var_tanh2_dn13_slot: &mut f64,
        var_tanh2_dn14_slot: &mut f64,
        var_tanh2_dn15_slot: &mut f64,
        var_tanh2_dn2_slot: &mut f64,
        var_tanh2_dn3_slot: &mut f64,
        var_tanh2_dn4_slot: &mut f64,
        var_tanh2_dn5_slot: &mut f64,
        var_tanh2_dn6_slot: &mut f64,
        var_tanh2_dn7_slot: &mut f64,
        var_tanh2_dn8_slot: &mut f64,
        var_tanh2_dn9_slot: &mut f64,
        var_tanh3_slot: &mut f64,
        var_tanh3_db0_slot: &mut f64,
        var_tanh3_db1_slot: &mut f64,
        var_tanh3_db10_slot: &mut f64,
        var_tanh3_db11_slot: &mut f64,
        var_tanh3_db12_slot: &mut f64,
        var_tanh3_db13_slot: &mut f64,
        var_tanh3_db14_slot: &mut f64,
        var_tanh3_db2_slot: &mut f64,
        var_tanh3_db3_slot: &mut f64,
        var_tanh3_db4_slot: &mut f64,
        var_tanh3_db5_slot: &mut f64,
        var_tanh3_db6_slot: &mut f64,
        var_tanh3_db7_slot: &mut f64,
        var_tanh3_db8_slot: &mut f64,
        var_tanh3_db9_slot: &mut f64,
        var_tanh3_dn0_slot: &mut f64,
        var_tanh3_dn1_slot: &mut f64,
        var_tanh3_dn10_slot: &mut f64,
        var_tanh3_dn11_slot: &mut f64,
        var_tanh3_dn12_slot: &mut f64,
        var_tanh3_dn13_slot: &mut f64,
        var_tanh3_dn14_slot: &mut f64,
        var_tanh3_dn15_slot: &mut f64,
        var_tanh3_dn2_slot: &mut f64,
        var_tanh3_dn3_slot: &mut f64,
        var_tanh3_dn4_slot: &mut f64,
        var_tanh3_dn5_slot: &mut f64,
        var_tanh3_dn6_slot: &mut f64,
        var_tanh3_dn7_slot: &mut f64,
        var_tanh3_dn8_slot: &mut f64,
        var_tanh3_dn9_slot: &mut f64,
        var_tanh4_slot: &mut f64,
        var_tanh4_db0_slot: &mut f64,
        var_tanh4_db1_slot: &mut f64,
        var_tanh4_db10_slot: &mut f64,
        var_tanh4_db11_slot: &mut f64,
        var_tanh4_db12_slot: &mut f64,
        var_tanh4_db13_slot: &mut f64,
        var_tanh4_db14_slot: &mut f64,
        var_tanh4_db2_slot: &mut f64,
        var_tanh4_db3_slot: &mut f64,
        var_tanh4_db4_slot: &mut f64,
        var_tanh4_db5_slot: &mut f64,
        var_tanh4_db6_slot: &mut f64,
        var_tanh4_db7_slot: &mut f64,
        var_tanh4_db8_slot: &mut f64,
        var_tanh4_db9_slot: &mut f64,
        var_tanh4_dn0_slot: &mut f64,
        var_tanh4_dn1_slot: &mut f64,
        var_tanh4_dn10_slot: &mut f64,
        var_tanh4_dn11_slot: &mut f64,
        var_tanh4_dn12_slot: &mut f64,
        var_tanh4_dn13_slot: &mut f64,
        var_tanh4_dn14_slot: &mut f64,
        var_tanh4_dn15_slot: &mut f64,
        var_tanh4_dn2_slot: &mut f64,
        var_tanh4_dn3_slot: &mut f64,
        var_tanh4_dn4_slot: &mut f64,
        var_tanh4_dn5_slot: &mut f64,
        var_tanh4_dn6_slot: &mut f64,
        var_tanh4_dn7_slot: &mut f64,
        var_tanh4_dn8_slot: &mut f64,
        var_tanh4_dn9_slot: &mut f64,
    ) {
        let mut var_cgd: f64 = *var_cgd_slot;
        let mut var_cgd_db0: f64 = *var_cgd_db0_slot;
        let mut var_cgd_db1: f64 = *var_cgd_db1_slot;
        let mut var_cgd_db10: f64 = *var_cgd_db10_slot;
        let mut var_cgd_db11: f64 = *var_cgd_db11_slot;
        let mut var_cgd_db12: f64 = *var_cgd_db12_slot;
        let mut var_cgd_db13: f64 = *var_cgd_db13_slot;
        let mut var_cgd_db14: f64 = *var_cgd_db14_slot;
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
        let mut var_cgs: f64 = *var_cgs_slot;
        let mut var_cgs_db0: f64 = *var_cgs_db0_slot;
        let mut var_cgs_db1: f64 = *var_cgs_db1_slot;
        let mut var_cgs_db10: f64 = *var_cgs_db10_slot;
        let mut var_cgs_db11: f64 = *var_cgs_db11_slot;
        let mut var_cgs_db12: f64 = *var_cgs_db12_slot;
        let mut var_cgs_db13: f64 = *var_cgs_db13_slot;
        let mut var_cgs_db14: f64 = *var_cgs_db14_slot;
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
        let mut var_guard13: f64 = *var_guard13_slot;
        let mut var_guard14: f64 = *var_guard14_slot;
        let mut var_guard15: f64 = *var_guard15_slot;
        let mut var_psi_1: f64 = *var_psi_1_slot;
        let mut var_psi_1_db0: f64 = *var_psi_1_db0_slot;
        let mut var_psi_1_db1: f64 = *var_psi_1_db1_slot;
        let mut var_psi_1_db10: f64 = *var_psi_1_db10_slot;
        let mut var_psi_1_db11: f64 = *var_psi_1_db11_slot;
        let mut var_psi_1_db12: f64 = *var_psi_1_db12_slot;
        let mut var_psi_1_db13: f64 = *var_psi_1_db13_slot;
        let mut var_psi_1_db14: f64 = *var_psi_1_db14_slot;
        let mut var_psi_1_db2: f64 = *var_psi_1_db2_slot;
        let mut var_psi_1_db3: f64 = *var_psi_1_db3_slot;
        let mut var_psi_1_db4: f64 = *var_psi_1_db4_slot;
        let mut var_psi_1_db5: f64 = *var_psi_1_db5_slot;
        let mut var_psi_1_db6: f64 = *var_psi_1_db6_slot;
        let mut var_psi_1_db7: f64 = *var_psi_1_db7_slot;
        let mut var_psi_1_db8: f64 = *var_psi_1_db8_slot;
        let mut var_psi_1_db9: f64 = *var_psi_1_db9_slot;
        let mut var_psi_1_dn0: f64 = *var_psi_1_dn0_slot;
        let mut var_psi_1_dn1: f64 = *var_psi_1_dn1_slot;
        let mut var_psi_1_dn10: f64 = *var_psi_1_dn10_slot;
        let mut var_psi_1_dn11: f64 = *var_psi_1_dn11_slot;
        let mut var_psi_1_dn12: f64 = *var_psi_1_dn12_slot;
        let mut var_psi_1_dn13: f64 = *var_psi_1_dn13_slot;
        let mut var_psi_1_dn14: f64 = *var_psi_1_dn14_slot;
        let mut var_psi_1_dn15: f64 = *var_psi_1_dn15_slot;
        let mut var_psi_1_dn2: f64 = *var_psi_1_dn2_slot;
        let mut var_psi_1_dn3: f64 = *var_psi_1_dn3_slot;
        let mut var_psi_1_dn4: f64 = *var_psi_1_dn4_slot;
        let mut var_psi_1_dn5: f64 = *var_psi_1_dn5_slot;
        let mut var_psi_1_dn6: f64 = *var_psi_1_dn6_slot;
        let mut var_psi_1_dn7: f64 = *var_psi_1_dn7_slot;
        let mut var_psi_1_dn8: f64 = *var_psi_1_dn8_slot;
        let mut var_psi_1_dn9: f64 = *var_psi_1_dn9_slot;
        let mut var_psi_2: f64 = *var_psi_2_slot;
        let mut var_psi_2_db0: f64 = *var_psi_2_db0_slot;
        let mut var_psi_2_db1: f64 = *var_psi_2_db1_slot;
        let mut var_psi_2_db10: f64 = *var_psi_2_db10_slot;
        let mut var_psi_2_db11: f64 = *var_psi_2_db11_slot;
        let mut var_psi_2_db12: f64 = *var_psi_2_db12_slot;
        let mut var_psi_2_db13: f64 = *var_psi_2_db13_slot;
        let mut var_psi_2_db14: f64 = *var_psi_2_db14_slot;
        let mut var_psi_2_db2: f64 = *var_psi_2_db2_slot;
        let mut var_psi_2_db3: f64 = *var_psi_2_db3_slot;
        let mut var_psi_2_db4: f64 = *var_psi_2_db4_slot;
        let mut var_psi_2_db5: f64 = *var_psi_2_db5_slot;
        let mut var_psi_2_db6: f64 = *var_psi_2_db6_slot;
        let mut var_psi_2_db7: f64 = *var_psi_2_db7_slot;
        let mut var_psi_2_db8: f64 = *var_psi_2_db8_slot;
        let mut var_psi_2_db9: f64 = *var_psi_2_db9_slot;
        let mut var_psi_2_dn0: f64 = *var_psi_2_dn0_slot;
        let mut var_psi_2_dn1: f64 = *var_psi_2_dn1_slot;
        let mut var_psi_2_dn10: f64 = *var_psi_2_dn10_slot;
        let mut var_psi_2_dn11: f64 = *var_psi_2_dn11_slot;
        let mut var_psi_2_dn12: f64 = *var_psi_2_dn12_slot;
        let mut var_psi_2_dn13: f64 = *var_psi_2_dn13_slot;
        let mut var_psi_2_dn14: f64 = *var_psi_2_dn14_slot;
        let mut var_psi_2_dn15: f64 = *var_psi_2_dn15_slot;
        let mut var_psi_2_dn2: f64 = *var_psi_2_dn2_slot;
        let mut var_psi_2_dn3: f64 = *var_psi_2_dn3_slot;
        let mut var_psi_2_dn4: f64 = *var_psi_2_dn4_slot;
        let mut var_psi_2_dn5: f64 = *var_psi_2_dn5_slot;
        let mut var_psi_2_dn6: f64 = *var_psi_2_dn6_slot;
        let mut var_psi_2_dn7: f64 = *var_psi_2_dn7_slot;
        let mut var_psi_2_dn8: f64 = *var_psi_2_dn8_slot;
        let mut var_psi_2_dn9: f64 = *var_psi_2_dn9_slot;
        let mut var_psi_3: f64 = *var_psi_3_slot;
        let mut var_psi_3_db0: f64 = *var_psi_3_db0_slot;
        let mut var_psi_3_db1: f64 = *var_psi_3_db1_slot;
        let mut var_psi_3_db10: f64 = *var_psi_3_db10_slot;
        let mut var_psi_3_db11: f64 = *var_psi_3_db11_slot;
        let mut var_psi_3_db12: f64 = *var_psi_3_db12_slot;
        let mut var_psi_3_db13: f64 = *var_psi_3_db13_slot;
        let mut var_psi_3_db14: f64 = *var_psi_3_db14_slot;
        let mut var_psi_3_db2: f64 = *var_psi_3_db2_slot;
        let mut var_psi_3_db3: f64 = *var_psi_3_db3_slot;
        let mut var_psi_3_db4: f64 = *var_psi_3_db4_slot;
        let mut var_psi_3_db5: f64 = *var_psi_3_db5_slot;
        let mut var_psi_3_db6: f64 = *var_psi_3_db6_slot;
        let mut var_psi_3_db7: f64 = *var_psi_3_db7_slot;
        let mut var_psi_3_db8: f64 = *var_psi_3_db8_slot;
        let mut var_psi_3_db9: f64 = *var_psi_3_db9_slot;
        let mut var_psi_3_dn0: f64 = *var_psi_3_dn0_slot;
        let mut var_psi_3_dn1: f64 = *var_psi_3_dn1_slot;
        let mut var_psi_3_dn10: f64 = *var_psi_3_dn10_slot;
        let mut var_psi_3_dn11: f64 = *var_psi_3_dn11_slot;
        let mut var_psi_3_dn12: f64 = *var_psi_3_dn12_slot;
        let mut var_psi_3_dn13: f64 = *var_psi_3_dn13_slot;
        let mut var_psi_3_dn14: f64 = *var_psi_3_dn14_slot;
        let mut var_psi_3_dn15: f64 = *var_psi_3_dn15_slot;
        let mut var_psi_3_dn2: f64 = *var_psi_3_dn2_slot;
        let mut var_psi_3_dn3: f64 = *var_psi_3_dn3_slot;
        let mut var_psi_3_dn4: f64 = *var_psi_3_dn4_slot;
        let mut var_psi_3_dn5: f64 = *var_psi_3_dn5_slot;
        let mut var_psi_3_dn6: f64 = *var_psi_3_dn6_slot;
        let mut var_psi_3_dn7: f64 = *var_psi_3_dn7_slot;
        let mut var_psi_3_dn8: f64 = *var_psi_3_dn8_slot;
        let mut var_psi_3_dn9: f64 = *var_psi_3_dn9_slot;
        let mut var_psi_4: f64 = *var_psi_4_slot;
        let mut var_psi_4_db0: f64 = *var_psi_4_db0_slot;
        let mut var_psi_4_db1: f64 = *var_psi_4_db1_slot;
        let mut var_psi_4_db10: f64 = *var_psi_4_db10_slot;
        let mut var_psi_4_db11: f64 = *var_psi_4_db11_slot;
        let mut var_psi_4_db12: f64 = *var_psi_4_db12_slot;
        let mut var_psi_4_db13: f64 = *var_psi_4_db13_slot;
        let mut var_psi_4_db14: f64 = *var_psi_4_db14_slot;
        let mut var_psi_4_db2: f64 = *var_psi_4_db2_slot;
        let mut var_psi_4_db3: f64 = *var_psi_4_db3_slot;
        let mut var_psi_4_db4: f64 = *var_psi_4_db4_slot;
        let mut var_psi_4_db5: f64 = *var_psi_4_db5_slot;
        let mut var_psi_4_db6: f64 = *var_psi_4_db6_slot;
        let mut var_psi_4_db7: f64 = *var_psi_4_db7_slot;
        let mut var_psi_4_db8: f64 = *var_psi_4_db8_slot;
        let mut var_psi_4_db9: f64 = *var_psi_4_db9_slot;
        let mut var_psi_4_dn0: f64 = *var_psi_4_dn0_slot;
        let mut var_psi_4_dn1: f64 = *var_psi_4_dn1_slot;
        let mut var_psi_4_dn10: f64 = *var_psi_4_dn10_slot;
        let mut var_psi_4_dn11: f64 = *var_psi_4_dn11_slot;
        let mut var_psi_4_dn12: f64 = *var_psi_4_dn12_slot;
        let mut var_psi_4_dn13: f64 = *var_psi_4_dn13_slot;
        let mut var_psi_4_dn14: f64 = *var_psi_4_dn14_slot;
        let mut var_psi_4_dn15: f64 = *var_psi_4_dn15_slot;
        let mut var_psi_4_dn2: f64 = *var_psi_4_dn2_slot;
        let mut var_psi_4_dn3: f64 = *var_psi_4_dn3_slot;
        let mut var_psi_4_dn4: f64 = *var_psi_4_dn4_slot;
        let mut var_psi_4_dn5: f64 = *var_psi_4_dn5_slot;
        let mut var_psi_4_dn6: f64 = *var_psi_4_dn6_slot;
        let mut var_psi_4_dn7: f64 = *var_psi_4_dn7_slot;
        let mut var_psi_4_dn8: f64 = *var_psi_4_dn8_slot;
        let mut var_psi_4_dn9: f64 = *var_psi_4_dn9_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_db0: f64 = *var_t0_db0_slot;
        let mut var_t0_db1: f64 = *var_t0_db1_slot;
        let mut var_t0_db10: f64 = *var_t0_db10_slot;
        let mut var_t0_db11: f64 = *var_t0_db11_slot;
        let mut var_t0_db12: f64 = *var_t0_db12_slot;
        let mut var_t0_db13: f64 = *var_t0_db13_slot;
        let mut var_t0_db14: f64 = *var_t0_db14_slot;
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
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_tanh1: f64 = *var_tanh1_slot;
        let mut var_tanh1_db0: f64 = *var_tanh1_db0_slot;
        let mut var_tanh1_db1: f64 = *var_tanh1_db1_slot;
        let mut var_tanh1_db10: f64 = *var_tanh1_db10_slot;
        let mut var_tanh1_db11: f64 = *var_tanh1_db11_slot;
        let mut var_tanh1_db12: f64 = *var_tanh1_db12_slot;
        let mut var_tanh1_db13: f64 = *var_tanh1_db13_slot;
        let mut var_tanh1_db14: f64 = *var_tanh1_db14_slot;
        let mut var_tanh1_db2: f64 = *var_tanh1_db2_slot;
        let mut var_tanh1_db3: f64 = *var_tanh1_db3_slot;
        let mut var_tanh1_db4: f64 = *var_tanh1_db4_slot;
        let mut var_tanh1_db5: f64 = *var_tanh1_db5_slot;
        let mut var_tanh1_db6: f64 = *var_tanh1_db6_slot;
        let mut var_tanh1_db7: f64 = *var_tanh1_db7_slot;
        let mut var_tanh1_db8: f64 = *var_tanh1_db8_slot;
        let mut var_tanh1_db9: f64 = *var_tanh1_db9_slot;
        let mut var_tanh1_dn0: f64 = *var_tanh1_dn0_slot;
        let mut var_tanh1_dn1: f64 = *var_tanh1_dn1_slot;
        let mut var_tanh1_dn10: f64 = *var_tanh1_dn10_slot;
        let mut var_tanh1_dn11: f64 = *var_tanh1_dn11_slot;
        let mut var_tanh1_dn12: f64 = *var_tanh1_dn12_slot;
        let mut var_tanh1_dn13: f64 = *var_tanh1_dn13_slot;
        let mut var_tanh1_dn14: f64 = *var_tanh1_dn14_slot;
        let mut var_tanh1_dn15: f64 = *var_tanh1_dn15_slot;
        let mut var_tanh1_dn2: f64 = *var_tanh1_dn2_slot;
        let mut var_tanh1_dn3: f64 = *var_tanh1_dn3_slot;
        let mut var_tanh1_dn4: f64 = *var_tanh1_dn4_slot;
        let mut var_tanh1_dn5: f64 = *var_tanh1_dn5_slot;
        let mut var_tanh1_dn6: f64 = *var_tanh1_dn6_slot;
        let mut var_tanh1_dn7: f64 = *var_tanh1_dn7_slot;
        let mut var_tanh1_dn8: f64 = *var_tanh1_dn8_slot;
        let mut var_tanh1_dn9: f64 = *var_tanh1_dn9_slot;
        let mut var_tanh2: f64 = *var_tanh2_slot;
        let mut var_tanh2_db0: f64 = *var_tanh2_db0_slot;
        let mut var_tanh2_db1: f64 = *var_tanh2_db1_slot;
        let mut var_tanh2_db10: f64 = *var_tanh2_db10_slot;
        let mut var_tanh2_db11: f64 = *var_tanh2_db11_slot;
        let mut var_tanh2_db12: f64 = *var_tanh2_db12_slot;
        let mut var_tanh2_db13: f64 = *var_tanh2_db13_slot;
        let mut var_tanh2_db14: f64 = *var_tanh2_db14_slot;
        let mut var_tanh2_db2: f64 = *var_tanh2_db2_slot;
        let mut var_tanh2_db3: f64 = *var_tanh2_db3_slot;
        let mut var_tanh2_db4: f64 = *var_tanh2_db4_slot;
        let mut var_tanh2_db5: f64 = *var_tanh2_db5_slot;
        let mut var_tanh2_db6: f64 = *var_tanh2_db6_slot;
        let mut var_tanh2_db7: f64 = *var_tanh2_db7_slot;
        let mut var_tanh2_db8: f64 = *var_tanh2_db8_slot;
        let mut var_tanh2_db9: f64 = *var_tanh2_db9_slot;
        let mut var_tanh2_dn0: f64 = *var_tanh2_dn0_slot;
        let mut var_tanh2_dn1: f64 = *var_tanh2_dn1_slot;
        let mut var_tanh2_dn10: f64 = *var_tanh2_dn10_slot;
        let mut var_tanh2_dn11: f64 = *var_tanh2_dn11_slot;
        let mut var_tanh2_dn12: f64 = *var_tanh2_dn12_slot;
        let mut var_tanh2_dn13: f64 = *var_tanh2_dn13_slot;
        let mut var_tanh2_dn14: f64 = *var_tanh2_dn14_slot;
        let mut var_tanh2_dn15: f64 = *var_tanh2_dn15_slot;
        let mut var_tanh2_dn2: f64 = *var_tanh2_dn2_slot;
        let mut var_tanh2_dn3: f64 = *var_tanh2_dn3_slot;
        let mut var_tanh2_dn4: f64 = *var_tanh2_dn4_slot;
        let mut var_tanh2_dn5: f64 = *var_tanh2_dn5_slot;
        let mut var_tanh2_dn6: f64 = *var_tanh2_dn6_slot;
        let mut var_tanh2_dn7: f64 = *var_tanh2_dn7_slot;
        let mut var_tanh2_dn8: f64 = *var_tanh2_dn8_slot;
        let mut var_tanh2_dn9: f64 = *var_tanh2_dn9_slot;
        let mut var_tanh3: f64 = *var_tanh3_slot;
        let mut var_tanh3_db0: f64 = *var_tanh3_db0_slot;
        let mut var_tanh3_db1: f64 = *var_tanh3_db1_slot;
        let mut var_tanh3_db10: f64 = *var_tanh3_db10_slot;
        let mut var_tanh3_db11: f64 = *var_tanh3_db11_slot;
        let mut var_tanh3_db12: f64 = *var_tanh3_db12_slot;
        let mut var_tanh3_db13: f64 = *var_tanh3_db13_slot;
        let mut var_tanh3_db14: f64 = *var_tanh3_db14_slot;
        let mut var_tanh3_db2: f64 = *var_tanh3_db2_slot;
        let mut var_tanh3_db3: f64 = *var_tanh3_db3_slot;
        let mut var_tanh3_db4: f64 = *var_tanh3_db4_slot;
        let mut var_tanh3_db5: f64 = *var_tanh3_db5_slot;
        let mut var_tanh3_db6: f64 = *var_tanh3_db6_slot;
        let mut var_tanh3_db7: f64 = *var_tanh3_db7_slot;
        let mut var_tanh3_db8: f64 = *var_tanh3_db8_slot;
        let mut var_tanh3_db9: f64 = *var_tanh3_db9_slot;
        let mut var_tanh3_dn0: f64 = *var_tanh3_dn0_slot;
        let mut var_tanh3_dn1: f64 = *var_tanh3_dn1_slot;
        let mut var_tanh3_dn10: f64 = *var_tanh3_dn10_slot;
        let mut var_tanh3_dn11: f64 = *var_tanh3_dn11_slot;
        let mut var_tanh3_dn12: f64 = *var_tanh3_dn12_slot;
        let mut var_tanh3_dn13: f64 = *var_tanh3_dn13_slot;
        let mut var_tanh3_dn14: f64 = *var_tanh3_dn14_slot;
        let mut var_tanh3_dn15: f64 = *var_tanh3_dn15_slot;
        let mut var_tanh3_dn2: f64 = *var_tanh3_dn2_slot;
        let mut var_tanh3_dn3: f64 = *var_tanh3_dn3_slot;
        let mut var_tanh3_dn4: f64 = *var_tanh3_dn4_slot;
        let mut var_tanh3_dn5: f64 = *var_tanh3_dn5_slot;
        let mut var_tanh3_dn6: f64 = *var_tanh3_dn6_slot;
        let mut var_tanh3_dn7: f64 = *var_tanh3_dn7_slot;
        let mut var_tanh3_dn8: f64 = *var_tanh3_dn8_slot;
        let mut var_tanh3_dn9: f64 = *var_tanh3_dn9_slot;
        let mut var_tanh4: f64 = *var_tanh4_slot;
        let mut var_tanh4_db0: f64 = *var_tanh4_db0_slot;
        let mut var_tanh4_db1: f64 = *var_tanh4_db1_slot;
        let mut var_tanh4_db10: f64 = *var_tanh4_db10_slot;
        let mut var_tanh4_db11: f64 = *var_tanh4_db11_slot;
        let mut var_tanh4_db12: f64 = *var_tanh4_db12_slot;
        let mut var_tanh4_db13: f64 = *var_tanh4_db13_slot;
        let mut var_tanh4_db14: f64 = *var_tanh4_db14_slot;
        let mut var_tanh4_db2: f64 = *var_tanh4_db2_slot;
        let mut var_tanh4_db3: f64 = *var_tanh4_db3_slot;
        let mut var_tanh4_db4: f64 = *var_tanh4_db4_slot;
        let mut var_tanh4_db5: f64 = *var_tanh4_db5_slot;
        let mut var_tanh4_db6: f64 = *var_tanh4_db6_slot;
        let mut var_tanh4_db7: f64 = *var_tanh4_db7_slot;
        let mut var_tanh4_db8: f64 = *var_tanh4_db8_slot;
        let mut var_tanh4_db9: f64 = *var_tanh4_db9_slot;
        let mut var_tanh4_dn0: f64 = *var_tanh4_dn0_slot;
        let mut var_tanh4_dn1: f64 = *var_tanh4_dn1_slot;
        let mut var_tanh4_dn10: f64 = *var_tanh4_dn10_slot;
        let mut var_tanh4_dn11: f64 = *var_tanh4_dn11_slot;
        let mut var_tanh4_dn12: f64 = *var_tanh4_dn12_slot;
        let mut var_tanh4_dn13: f64 = *var_tanh4_dn13_slot;
        let mut var_tanh4_dn14: f64 = *var_tanh4_dn14_slot;
        let mut var_tanh4_dn15: f64 = *var_tanh4_dn15_slot;
        let mut var_tanh4_dn2: f64 = *var_tanh4_dn2_slot;
        let mut var_tanh4_dn3: f64 = *var_tanh4_dn3_slot;
        let mut var_tanh4_dn4: f64 = *var_tanh4_dn4_slot;
        let mut var_tanh4_dn5: f64 = *var_tanh4_dn5_slot;
        let mut var_tanh4_dn6: f64 = *var_tanh4_dn6_slot;
        let mut var_tanh4_dn7: f64 = *var_tanh4_dn7_slot;
        let mut var_tanh4_dn8: f64 = *var_tanh4_dn8_slot;
        let mut var_tanh4_dn9: f64 = *var_tanh4_dn9_slot;

        let (assign1170_e1636, assign1170_e1636_d_n0, assign1170_e1636_d_n1, assign1170_e1636_d_n2, assign1170_e1636_d_n3, assign1170_e1636_d_n4, assign1170_e1636_d_n5, assign1170_e1636_d_n6, assign1170_e1636_d_n7, assign1170_e1636_d_n8, assign1170_e1636_d_n9, assign1170_e1636_d_n10, assign1170_e1636_d_n11, assign1170_e1636_d_n12, assign1170_e1636_d_n13, assign1170_e1636_d_n14, assign1170_e1636_d_n15, assign1170_e1636_d_b0, assign1170_e1636_d_b1, assign1170_e1636_d_b2, assign1170_e1636_d_b3, assign1170_e1636_d_b4, assign1170_e1636_d_b5, assign1170_e1636_d_b6, assign1170_e1636_d_b7, assign1170_e1636_d_b8, assign1170_e1636_d_b9, assign1170_e1636_d_b10, assign1170_e1636_d_b11, assign1170_e1636_d_b12, assign1170_e1636_d_b13, assign1170_e1636_d_b14,) = {
    if (var_guard11 != 0.0) {
        let assign1170_e1629: f64 = (-1.0);
        let assign1170_e1631: f64 = (assign1170_e1629 * var_vjg_t);
        let assign1170_e1632: f64 = (assign1170_e1631).tanh();
        let assign1170_e1633: f64 = (var_pg_param * assign1170_e1632);
        let assign1170_e1634: f64 = { let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (assign1170_e1634, ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_dn0 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_dn0) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_dn1 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_dn1) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_dn2 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_dn2) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_dn3 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_dn3) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_dn4 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_dn4) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_dn5 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_dn5) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_dn6 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_dn6) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_dn7 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_dn7) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_dn8 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_dn8) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_dn9 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_dn9) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_dn10 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_dn10) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_dn11 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_dn11) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_dn12 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_dn12) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_dn13 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_dn13) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_dn14 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_dn14) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_dn15 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_dn15) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_db0 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_db0) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_db1 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_db1) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_db2 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_db2) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_db3 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_db3) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_db4 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_db4) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_db5 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_db5) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_db6 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_db6) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_db7 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_db7) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_db8 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_db8) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_db9 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_db9) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_db10 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_db10) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_db11 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_db11) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_db12 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_db12) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_db13 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_db13) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))), ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((var_pg_param_db14 * assign1170_e1632) + (var_pg_param * ((assign1170_e1629 * var_vjg_t_db14) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn12, var_t0_dn13, var_t0_dn14, var_t0_dn15, var_t0_db0, var_t0_db1, var_t0_db2, var_t0_db3, var_t0_db4, var_t0_db5, var_t0_db6, var_t0_db7, var_t0_db8, var_t0_db9, var_t0_db10, var_t0_db11, var_t0_db12, var_t0_db13, var_t0_db14,)
    }
};
        var_t0 = assign1170_e1636;
        var_t0_dn0 = assign1170_e1636_d_n0;
        var_t0_dn1 = assign1170_e1636_d_n1;
        var_t0_dn2 = assign1170_e1636_d_n2;
        var_t0_dn3 = assign1170_e1636_d_n3;
        var_t0_dn4 = assign1170_e1636_d_n4;
        var_t0_dn5 = assign1170_e1636_d_n5;
        var_t0_dn6 = assign1170_e1636_d_n6;
        var_t0_dn7 = assign1170_e1636_d_n7;
        var_t0_dn8 = assign1170_e1636_d_n8;
        var_t0_dn9 = assign1170_e1636_d_n9;
        var_t0_dn10 = assign1170_e1636_d_n10;
        var_t0_dn11 = assign1170_e1636_d_n11;
        var_t0_dn12 = assign1170_e1636_d_n12;
        var_t0_dn13 = assign1170_e1636_d_n13;
        var_t0_dn14 = assign1170_e1636_d_n14;
        var_t0_dn15 = assign1170_e1636_d_n15;
        var_t0_db0 = assign1170_e1636_d_b0;
        var_t0_db1 = assign1170_e1636_d_b1;
        var_t0_db2 = assign1170_e1636_d_b2;
        var_t0_db3 = assign1170_e1636_d_b3;
        var_t0_db4 = assign1170_e1636_d_b4;
        var_t0_db5 = assign1170_e1636_d_b5;
        var_t0_db6 = assign1170_e1636_d_b6;
        var_t0_db7 = assign1170_e1636_d_b7;
        var_t0_db8 = assign1170_e1636_d_b8;
        var_t0_db9 = assign1170_e1636_d_b9;
        var_t0_db10 = assign1170_e1636_d_b10;
        var_t0_db11 = assign1170_e1636_d_b11;
        var_t0_db12 = assign1170_e1636_d_b12;
        var_t0_db13 = assign1170_e1636_d_b13;
        var_t0_db14 = assign1170_e1636_d_b14;

        let (assign1200_e1661, assign1200_e1661_d_n0, assign1200_e1661_d_n1, assign1200_e1661_d_n2, assign1200_e1661_d_n3, assign1200_e1661_d_n4, assign1200_e1661_d_n5, assign1200_e1661_d_n6, assign1200_e1661_d_n7, assign1200_e1661_d_n8, assign1200_e1661_d_n9, assign1200_e1661_d_n10, assign1200_e1661_d_n11, assign1200_e1661_d_n12, assign1200_e1661_d_n13, assign1200_e1661_d_n14, assign1200_e1661_d_n15, assign1200_e1661_d_b0, assign1200_e1661_d_b1, assign1200_e1661_d_b2, assign1200_e1661_d_b3, assign1200_e1661_d_b4, assign1200_e1661_d_b5, assign1200_e1661_d_b6, assign1200_e1661_d_b7, assign1200_e1661_d_b8, assign1200_e1661_d_b9, assign1200_e1661_d_b10, assign1200_e1661_d_b11, assign1200_e1661_d_b12, assign1200_e1661_d_b13, assign1200_e1661_d_b14,) = {
    if (var_guard11 == 0.0) {
        let assign1200_e1656: f64 = (-var_pg_param);
        let assign1200_e1658: f64 = (assign1200_e1656 * var_vjg_t);
        let assign1200_e1659: f64 = { let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (assign1200_e1659, ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_dn0) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_dn0))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_dn1) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_dn1))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_dn2) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_dn2))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_dn3) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_dn3))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_dn4) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_dn4))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_dn5) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_dn5))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_dn6) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_dn6))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_dn7) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_dn7))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_dn8) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_dn8))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_dn9) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_dn9))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_dn10) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_dn10))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_dn11) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_dn11))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_dn12) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_dn12))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_dn13) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_dn13))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_dn14) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_dn14))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_dn15) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_dn15))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_db0) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_db0))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_db1) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_db1))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_db2) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_db2))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_db3) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_db3))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_db4) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_db4))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_db5) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_db5))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_db6) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_db6))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_db7) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_db7))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_db8) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_db8))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_db9) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_db9))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_db10) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_db10))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_db11) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_db11))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_db12) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_db12))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_db13) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_db13))), ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-var_pg_param_db14) * var_vjg_t) + (assign1200_e1656 * var_vjg_t_db14))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn12, var_t0_dn13, var_t0_dn14, var_t0_dn15, var_t0_db0, var_t0_db1, var_t0_db2, var_t0_db3, var_t0_db4, var_t0_db5, var_t0_db6, var_t0_db7, var_t0_db8, var_t0_db9, var_t0_db10, var_t0_db11, var_t0_db12, var_t0_db13, var_t0_db14,)
    }
};
        var_t0 = assign1200_e1661;
        var_t0_dn0 = assign1200_e1661_d_n0;
        var_t0_dn1 = assign1200_e1661_d_n1;
        var_t0_dn2 = assign1200_e1661_d_n2;
        var_t0_dn3 = assign1200_e1661_d_n3;
        var_t0_dn4 = assign1200_e1661_d_n4;
        var_t0_dn5 = assign1200_e1661_d_n5;
        var_t0_dn6 = assign1200_e1661_d_n6;
        var_t0_dn7 = assign1200_e1661_d_n7;
        var_t0_dn8 = assign1200_e1661_d_n8;
        var_t0_dn9 = assign1200_e1661_d_n9;
        var_t0_dn10 = assign1200_e1661_d_n10;
        var_t0_dn11 = assign1200_e1661_d_n11;
        var_t0_dn12 = assign1200_e1661_d_n12;
        var_t0_dn13 = assign1200_e1661_d_n13;
        var_t0_dn14 = assign1200_e1661_d_n14;
        var_t0_dn15 = assign1200_e1661_d_n15;
        var_t0_db0 = assign1200_e1661_d_b0;
        var_t0_db1 = assign1200_e1661_d_b1;
        var_t0_db2 = assign1200_e1661_d_b2;
        var_t0_db3 = assign1200_e1661_d_b3;
        var_t0_db4 = assign1200_e1661_d_b4;
        var_t0_db5 = assign1200_e1661_d_b5;
        var_t0_db6 = assign1200_e1661_d_b6;
        var_t0_db7 = assign1200_e1661_d_b7;
        var_t0_db8 = assign1200_e1661_d_b8;
        var_t0_db9 = assign1200_e1661_d_b9;
        var_t0_db10 = assign1200_e1661_d_b10;
        var_t0_db11 = assign1200_e1661_d_b11;
        var_t0_db12 = assign1200_e1661_d_b12;
        var_t0_db13 = assign1200_e1661_d_b13;
        var_t0_db14 = assign1200_e1661_d_b14;

        let assign1280_e1724: f64 = (p.p30 * var_vgsc);
        let assign1280_e1725: f64 = (var_p10_t + assign1280_e1724);
        let assign1280_e1728: f64 = (p.p37 * var_vds);
        let assign1280_e1729: f64 = (assign1280_e1725 + assign1280_e1728);
        var_psi_1 = assign1280_e1729;
        var_psi_1_dn0 = ((var_p10_t_dn0 + (p.p30 * var_vgsc_dn0)) + (p.p37 * var_vds_dn0));
        var_psi_1_dn1 = ((var_p10_t_dn1 + (p.p30 * var_vgsc_dn1)) + (p.p37 * var_vds_dn1));
        var_psi_1_dn2 = ((var_p10_t_dn2 + (p.p30 * var_vgsc_dn2)) + (p.p37 * var_vds_dn2));
        var_psi_1_dn3 = ((var_p10_t_dn3 + (p.p30 * var_vgsc_dn3)) + (p.p37 * var_vds_dn3));
        var_psi_1_dn4 = ((var_p10_t_dn4 + (p.p30 * var_vgsc_dn4)) + (p.p37 * var_vds_dn4));
        var_psi_1_dn5 = ((var_p10_t_dn5 + (p.p30 * var_vgsc_dn5)) + (p.p37 * var_vds_dn5));
        var_psi_1_dn6 = ((var_p10_t_dn6 + (p.p30 * var_vgsc_dn6)) + (p.p37 * var_vds_dn6));
        var_psi_1_dn7 = ((var_p10_t_dn7 + (p.p30 * var_vgsc_dn7)) + (p.p37 * var_vds_dn7));
        var_psi_1_dn8 = ((var_p10_t_dn8 + (p.p30 * var_vgsc_dn8)) + (p.p37 * var_vds_dn8));
        var_psi_1_dn9 = ((var_p10_t_dn9 + (p.p30 * var_vgsc_dn9)) + (p.p37 * var_vds_dn9));
        var_psi_1_dn10 = ((var_p10_t_dn10 + (p.p30 * var_vgsc_dn10)) + (p.p37 * var_vds_dn10));
        var_psi_1_dn11 = ((var_p10_t_dn11 + (p.p30 * var_vgsc_dn11)) + (p.p37 * var_vds_dn11));
        var_psi_1_dn12 = ((var_p10_t_dn12 + (p.p30 * var_vgsc_dn12)) + (p.p37 * var_vds_dn12));
        var_psi_1_dn13 = ((var_p10_t_dn13 + (p.p30 * var_vgsc_dn13)) + (p.p37 * var_vds_dn13));
        var_psi_1_dn14 = ((var_p10_t_dn14 + (p.p30 * var_vgsc_dn14)) + (p.p37 * var_vds_dn14));
        var_psi_1_dn15 = ((var_p10_t_dn15 + (p.p30 * var_vgsc_dn15)) + (p.p37 * var_vds_dn15));
        var_psi_1_db0 = ((var_p10_t_db0 + (p.p30 * var_vgsc_db0)) + (p.p37 * var_vds_db0));
        var_psi_1_db1 = ((var_p10_t_db1 + (p.p30 * var_vgsc_db1)) + (p.p37 * var_vds_db1));
        var_psi_1_db2 = ((var_p10_t_db2 + (p.p30 * var_vgsc_db2)) + (p.p37 * var_vds_db2));
        var_psi_1_db3 = ((var_p10_t_db3 + (p.p30 * var_vgsc_db3)) + (p.p37 * var_vds_db3));
        var_psi_1_db4 = ((var_p10_t_db4 + (p.p30 * var_vgsc_db4)) + (p.p37 * var_vds_db4));
        var_psi_1_db5 = ((var_p10_t_db5 + (p.p30 * var_vgsc_db5)) + (p.p37 * var_vds_db5));
        var_psi_1_db6 = ((var_p10_t_db6 + (p.p30 * var_vgsc_db6)) + (p.p37 * var_vds_db6));
        var_psi_1_db7 = ((var_p10_t_db7 + (p.p30 * var_vgsc_db7)) + (p.p37 * var_vds_db7));
        var_psi_1_db8 = ((var_p10_t_db8 + (p.p30 * var_vgsc_db8)) + (p.p37 * var_vds_db8));
        var_psi_1_db9 = ((var_p10_t_db9 + (p.p30 * var_vgsc_db9)) + (p.p37 * var_vds_db9));
        var_psi_1_db10 = ((var_p10_t_db10 + (p.p30 * var_vgsc_db10)) + (p.p37 * var_vds_db10));
        var_psi_1_db11 = ((var_p10_t_db11 + (p.p30 * var_vgsc_db11)) + (p.p37 * var_vds_db11));
        var_psi_1_db12 = ((var_p10_t_db12 + (p.p30 * var_vgsc_db12)) + (p.p37 * var_vds_db12));
        var_psi_1_db13 = ((var_p10_t_db13 + (p.p30 * var_vgsc_db13)) + (p.p37 * var_vds_db13));
        var_psi_1_db14 = ((var_p10_t_db14 + (p.p30 * var_vgsc_db14)) + (p.p37 * var_vds_db14));

        let assign1290_e1732: f64 = (var_psi_1).tanh();
        let assign1290_e1733: f64 = (1.0 + assign1290_e1732);
        var_tanh1 = assign1290_e1733;
        var_tanh1_dn0 = (var_psi_1_dn0 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_dn1 = (var_psi_1_dn1 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_dn2 = (var_psi_1_dn2 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_dn3 = (var_psi_1_dn3 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_dn4 = (var_psi_1_dn4 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_dn5 = (var_psi_1_dn5 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_dn6 = (var_psi_1_dn6 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_dn7 = (var_psi_1_dn7 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_dn8 = (var_psi_1_dn8 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_dn9 = (var_psi_1_dn9 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_dn10 = (var_psi_1_dn10 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_dn11 = (var_psi_1_dn11 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_dn12 = (var_psi_1_dn12 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_dn13 = (var_psi_1_dn13 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_dn14 = (var_psi_1_dn14 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_dn15 = (var_psi_1_dn15 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_db0 = (var_psi_1_db0 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_db1 = (var_psi_1_db1 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_db2 = (var_psi_1_db2 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_db3 = (var_psi_1_db3 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_db4 = (var_psi_1_db4 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_db5 = (var_psi_1_db5 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_db6 = (var_psi_1_db6 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_db7 = (var_psi_1_db7 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_db8 = (var_psi_1_db8 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_db9 = (var_psi_1_db9 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_db10 = (var_psi_1_db10 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_db11 = (var_psi_1_db11 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_db12 = (var_psi_1_db12 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_db13 = (var_psi_1_db13 / ((var_psi_1).cosh() * (var_psi_1).cosh()));
        var_tanh1_db14 = (var_psi_1_db14 / ((var_psi_1).cosh() * (var_psi_1).cosh()));

        let assign1300_e1737: f64 = (p.p32 * var_vds);
        let assign1300_e1738: f64 = (p.p31 + assign1300_e1737);
        var_psi_2 = assign1300_e1738;
        var_psi_2_dn0 = (p.p32 * var_vds_dn0);
        var_psi_2_dn1 = (p.p32 * var_vds_dn1);
        var_psi_2_dn2 = (p.p32 * var_vds_dn2);
        var_psi_2_dn3 = (p.p32 * var_vds_dn3);
        var_psi_2_dn4 = (p.p32 * var_vds_dn4);
        var_psi_2_dn5 = (p.p32 * var_vds_dn5);
        var_psi_2_dn6 = (p.p32 * var_vds_dn6);
        var_psi_2_dn7 = (p.p32 * var_vds_dn7);
        var_psi_2_dn8 = (p.p32 * var_vds_dn8);
        var_psi_2_dn9 = (p.p32 * var_vds_dn9);
        var_psi_2_dn10 = (p.p32 * var_vds_dn10);
        var_psi_2_dn11 = (p.p32 * var_vds_dn11);
        var_psi_2_dn12 = (p.p32 * var_vds_dn12);
        var_psi_2_dn13 = (p.p32 * var_vds_dn13);
        var_psi_2_dn14 = (p.p32 * var_vds_dn14);
        var_psi_2_dn15 = (p.p32 * var_vds_dn15);
        var_psi_2_db0 = (p.p32 * var_vds_db0);
        var_psi_2_db1 = (p.p32 * var_vds_db1);
        var_psi_2_db2 = (p.p32 * var_vds_db2);
        var_psi_2_db3 = (p.p32 * var_vds_db3);
        var_psi_2_db4 = (p.p32 * var_vds_db4);
        var_psi_2_db5 = (p.p32 * var_vds_db5);
        var_psi_2_db6 = (p.p32 * var_vds_db6);
        var_psi_2_db7 = (p.p32 * var_vds_db7);
        var_psi_2_db8 = (p.p32 * var_vds_db8);
        var_psi_2_db9 = (p.p32 * var_vds_db9);
        var_psi_2_db10 = (p.p32 * var_vds_db10);
        var_psi_2_db11 = (p.p32 * var_vds_db11);
        var_psi_2_db12 = (p.p32 * var_vds_db12);
        var_psi_2_db13 = (p.p32 * var_vds_db13);
        var_psi_2_db14 = (p.p32 * var_vds_db14);

        let assign1310_e1741: f64 = (var_psi_2).tanh();
        let assign1310_e1742: f64 = (1.0 + assign1310_e1741);
        var_tanh2 = assign1310_e1742;
        var_tanh2_dn0 = (var_psi_2_dn0 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_dn1 = (var_psi_2_dn1 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_dn2 = (var_psi_2_dn2 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_dn3 = (var_psi_2_dn3 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_dn4 = (var_psi_2_dn4 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_dn5 = (var_psi_2_dn5 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_dn6 = (var_psi_2_dn6 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_dn7 = (var_psi_2_dn7 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_dn8 = (var_psi_2_dn8 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_dn9 = (var_psi_2_dn9 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_dn10 = (var_psi_2_dn10 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_dn11 = (var_psi_2_dn11 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_dn12 = (var_psi_2_dn12 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_dn13 = (var_psi_2_dn13 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_dn14 = (var_psi_2_dn14 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_dn15 = (var_psi_2_dn15 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_db0 = (var_psi_2_db0 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_db1 = (var_psi_2_db1 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_db2 = (var_psi_2_db2 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_db3 = (var_psi_2_db3 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_db4 = (var_psi_2_db4 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_db5 = (var_psi_2_db5 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_db6 = (var_psi_2_db6 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_db7 = (var_psi_2_db7 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_db8 = (var_psi_2_db8 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_db9 = (var_psi_2_db9 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_db10 = (var_psi_2_db10 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_db11 = (var_psi_2_db11 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_db12 = (var_psi_2_db12 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_db13 = (var_psi_2_db13 / ((var_psi_2).cosh() * (var_psi_2).cosh()));
        var_tanh2_db14 = (var_psi_2_db14 / ((var_psi_2).cosh() * (var_psi_2).cosh()));

        let assign1320_e1746: f64 = (p.p34 * var_vds);
        let assign1320_e1747: f64 = (p.p33 - assign1320_e1746);
        var_psi_3 = assign1320_e1747;
        var_psi_3_dn0 = (-(p.p34 * var_vds_dn0));
        var_psi_3_dn1 = (-(p.p34 * var_vds_dn1));
        var_psi_3_dn2 = (-(p.p34 * var_vds_dn2));
        var_psi_3_dn3 = (-(p.p34 * var_vds_dn3));
        var_psi_3_dn4 = (-(p.p34 * var_vds_dn4));
        var_psi_3_dn5 = (-(p.p34 * var_vds_dn5));
        var_psi_3_dn6 = (-(p.p34 * var_vds_dn6));
        var_psi_3_dn7 = (-(p.p34 * var_vds_dn7));
        var_psi_3_dn8 = (-(p.p34 * var_vds_dn8));
        var_psi_3_dn9 = (-(p.p34 * var_vds_dn9));
        var_psi_3_dn10 = (-(p.p34 * var_vds_dn10));
        var_psi_3_dn11 = (-(p.p34 * var_vds_dn11));
        var_psi_3_dn12 = (-(p.p34 * var_vds_dn12));
        var_psi_3_dn13 = (-(p.p34 * var_vds_dn13));
        var_psi_3_dn14 = (-(p.p34 * var_vds_dn14));
        var_psi_3_dn15 = (-(p.p34 * var_vds_dn15));
        var_psi_3_db0 = (-(p.p34 * var_vds_db0));
        var_psi_3_db1 = (-(p.p34 * var_vds_db1));
        var_psi_3_db2 = (-(p.p34 * var_vds_db2));
        var_psi_3_db3 = (-(p.p34 * var_vds_db3));
        var_psi_3_db4 = (-(p.p34 * var_vds_db4));
        var_psi_3_db5 = (-(p.p34 * var_vds_db5));
        var_psi_3_db6 = (-(p.p34 * var_vds_db6));
        var_psi_3_db7 = (-(p.p34 * var_vds_db7));
        var_psi_3_db8 = (-(p.p34 * var_vds_db8));
        var_psi_3_db9 = (-(p.p34 * var_vds_db9));
        var_psi_3_db10 = (-(p.p34 * var_vds_db10));
        var_psi_3_db11 = (-(p.p34 * var_vds_db11));
        var_psi_3_db12 = (-(p.p34 * var_vds_db12));
        var_psi_3_db13 = (-(p.p34 * var_vds_db13));
        var_psi_3_db14 = (-(p.p34 * var_vds_db14));

        let assign1330_e1750: f64 = (var_psi_3).tanh();
        let assign1330_e1751: f64 = (1.0 + assign1330_e1750);
        let assign1330_e1753: f64 = (assign1330_e1751 - p.p37);
        var_tanh3 = assign1330_e1753;
        var_tanh3_dn0 = (var_psi_3_dn0 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_dn1 = (var_psi_3_dn1 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_dn2 = (var_psi_3_dn2 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_dn3 = (var_psi_3_dn3 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_dn4 = (var_psi_3_dn4 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_dn5 = (var_psi_3_dn5 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_dn6 = (var_psi_3_dn6 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_dn7 = (var_psi_3_dn7 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_dn8 = (var_psi_3_dn8 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_dn9 = (var_psi_3_dn9 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_dn10 = (var_psi_3_dn10 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_dn11 = (var_psi_3_dn11 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_dn12 = (var_psi_3_dn12 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_dn13 = (var_psi_3_dn13 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_dn14 = (var_psi_3_dn14 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_dn15 = (var_psi_3_dn15 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_db0 = (var_psi_3_db0 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_db1 = (var_psi_3_db1 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_db2 = (var_psi_3_db2 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_db3 = (var_psi_3_db3 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_db4 = (var_psi_3_db4 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_db5 = (var_psi_3_db5 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_db6 = (var_psi_3_db6 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_db7 = (var_psi_3_db7 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_db8 = (var_psi_3_db8 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_db9 = (var_psi_3_db9 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_db10 = (var_psi_3_db10 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_db11 = (var_psi_3_db11 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_db12 = (var_psi_3_db12 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_db13 = (var_psi_3_db13 / ((var_psi_3).cosh() * (var_psi_3).cosh()));
        var_tanh3_db14 = (var_psi_3_db14 / ((var_psi_3).cosh() * (var_psi_3).cosh()));

        let assign1340_e1757: f64 = (p.p36 * var_vgdc);
        let assign1340_e1758: f64 = (var_p40_t + assign1340_e1757);
        let assign1340_e1761: f64 = (p.p37 * var_vds);
        let assign1340_e1762: f64 = (assign1340_e1758 - assign1340_e1761);
        var_psi_4 = assign1340_e1762;
        var_psi_4_dn0 = ((var_p40_t_dn0 + (p.p36 * var_vgdc_dn0)) - (p.p37 * var_vds_dn0));
        var_psi_4_dn1 = ((var_p40_t_dn1 + (p.p36 * var_vgdc_dn1)) - (p.p37 * var_vds_dn1));
        var_psi_4_dn2 = ((var_p40_t_dn2 + (p.p36 * var_vgdc_dn2)) - (p.p37 * var_vds_dn2));
        var_psi_4_dn3 = ((var_p40_t_dn3 + (p.p36 * var_vgdc_dn3)) - (p.p37 * var_vds_dn3));
        var_psi_4_dn4 = ((var_p40_t_dn4 + (p.p36 * var_vgdc_dn4)) - (p.p37 * var_vds_dn4));
        var_psi_4_dn5 = ((var_p40_t_dn5 + (p.p36 * var_vgdc_dn5)) - (p.p37 * var_vds_dn5));
        var_psi_4_dn6 = ((var_p40_t_dn6 + (p.p36 * var_vgdc_dn6)) - (p.p37 * var_vds_dn6));
        var_psi_4_dn7 = ((var_p40_t_dn7 + (p.p36 * var_vgdc_dn7)) - (p.p37 * var_vds_dn7));
        var_psi_4_dn8 = ((var_p40_t_dn8 + (p.p36 * var_vgdc_dn8)) - (p.p37 * var_vds_dn8));
        var_psi_4_dn9 = ((var_p40_t_dn9 + (p.p36 * var_vgdc_dn9)) - (p.p37 * var_vds_dn9));
        var_psi_4_dn10 = ((var_p40_t_dn10 + (p.p36 * var_vgdc_dn10)) - (p.p37 * var_vds_dn10));
        var_psi_4_dn11 = ((var_p40_t_dn11 + (p.p36 * var_vgdc_dn11)) - (p.p37 * var_vds_dn11));
        var_psi_4_dn12 = ((var_p40_t_dn12 + (p.p36 * var_vgdc_dn12)) - (p.p37 * var_vds_dn12));
        var_psi_4_dn13 = ((var_p40_t_dn13 + (p.p36 * var_vgdc_dn13)) - (p.p37 * var_vds_dn13));
        var_psi_4_dn14 = ((var_p40_t_dn14 + (p.p36 * var_vgdc_dn14)) - (p.p37 * var_vds_dn14));
        var_psi_4_dn15 = ((var_p40_t_dn15 + (p.p36 * var_vgdc_dn15)) - (p.p37 * var_vds_dn15));
        var_psi_4_db0 = ((var_p40_t_db0 + (p.p36 * var_vgdc_db0)) - (p.p37 * var_vds_db0));
        var_psi_4_db1 = ((var_p40_t_db1 + (p.p36 * var_vgdc_db1)) - (p.p37 * var_vds_db1));
        var_psi_4_db2 = ((var_p40_t_db2 + (p.p36 * var_vgdc_db2)) - (p.p37 * var_vds_db2));
        var_psi_4_db3 = ((var_p40_t_db3 + (p.p36 * var_vgdc_db3)) - (p.p37 * var_vds_db3));
        var_psi_4_db4 = ((var_p40_t_db4 + (p.p36 * var_vgdc_db4)) - (p.p37 * var_vds_db4));
        var_psi_4_db5 = ((var_p40_t_db5 + (p.p36 * var_vgdc_db5)) - (p.p37 * var_vds_db5));
        var_psi_4_db6 = ((var_p40_t_db6 + (p.p36 * var_vgdc_db6)) - (p.p37 * var_vds_db6));
        var_psi_4_db7 = ((var_p40_t_db7 + (p.p36 * var_vgdc_db7)) - (p.p37 * var_vds_db7));
        var_psi_4_db8 = ((var_p40_t_db8 + (p.p36 * var_vgdc_db8)) - (p.p37 * var_vds_db8));
        var_psi_4_db9 = ((var_p40_t_db9 + (p.p36 * var_vgdc_db9)) - (p.p37 * var_vds_db9));
        var_psi_4_db10 = ((var_p40_t_db10 + (p.p36 * var_vgdc_db10)) - (p.p37 * var_vds_db10));
        var_psi_4_db11 = ((var_p40_t_db11 + (p.p36 * var_vgdc_db11)) - (p.p37 * var_vds_db11));
        var_psi_4_db12 = ((var_p40_t_db12 + (p.p36 * var_vgdc_db12)) - (p.p37 * var_vds_db12));
        var_psi_4_db13 = ((var_p40_t_db13 + (p.p36 * var_vgdc_db13)) - (p.p37 * var_vds_db13));
        var_psi_4_db14 = ((var_p40_t_db14 + (p.p36 * var_vgdc_db14)) - (p.p37 * var_vds_db14));

        let assign1350_e1765: f64 = (var_psi_4).tanh();
        let assign1350_e1766: f64 = (1.0 + assign1350_e1765);
        var_tanh4 = assign1350_e1766;
        var_tanh4_dn0 = (var_psi_4_dn0 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_dn1 = (var_psi_4_dn1 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_dn2 = (var_psi_4_dn2 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_dn3 = (var_psi_4_dn3 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_dn4 = (var_psi_4_dn4 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_dn5 = (var_psi_4_dn5 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_dn6 = (var_psi_4_dn6 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_dn7 = (var_psi_4_dn7 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_dn8 = (var_psi_4_dn8 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_dn9 = (var_psi_4_dn9 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_dn10 = (var_psi_4_dn10 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_dn11 = (var_psi_4_dn11 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_dn12 = (var_psi_4_dn12 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_dn13 = (var_psi_4_dn13 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_dn14 = (var_psi_4_dn14 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_dn15 = (var_psi_4_dn15 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_db0 = (var_psi_4_db0 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_db1 = (var_psi_4_db1 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_db2 = (var_psi_4_db2 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_db3 = (var_psi_4_db3 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_db4 = (var_psi_4_db4 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_db5 = (var_psi_4_db5 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_db6 = (var_psi_4_db6 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_db7 = (var_psi_4_db7 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_db8 = (var_psi_4_db8 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_db9 = (var_psi_4_db9 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_db10 = (var_psi_4_db10 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_db11 = (var_psi_4_db11 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_db12 = (var_psi_4_db12 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_db13 = (var_psi_4_db13 / ((var_psi_4).cosh() * (var_psi_4).cosh()));
        var_tanh4_db14 = (var_psi_4_db14 / ((var_psi_4).cosh() * (var_psi_4).cosh()));

        let assign1360_e1769: f64 = if p.p6 == 0.0 { 1.0 } else { 0.0 };
        var_guard13 = assign1360_e1769;

        let assign1370_e1772: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        var_guard14 = assign1370_e1772;

        let assign1380_e1775: f64 = if p.p6 == 2.0 { 1.0 } else { 0.0 };
        var_guard15 = assign1380_e1775;

        let (assign1390_e1779, assign1390_e1779_d_n0, assign1390_e1779_d_n1, assign1390_e1779_d_n2, assign1390_e1779_d_n3, assign1390_e1779_d_n4, assign1390_e1779_d_n5, assign1390_e1779_d_n6, assign1390_e1779_d_n7, assign1390_e1779_d_n8, assign1390_e1779_d_n9, assign1390_e1779_d_n10, assign1390_e1779_d_n11, assign1390_e1779_d_n12, assign1390_e1779_d_n13, assign1390_e1779_d_n14, assign1390_e1779_d_n15, assign1390_e1779_d_b0, assign1390_e1779_d_b1, assign1390_e1779_d_b2, assign1390_e1779_d_b3, assign1390_e1779_d_b4, assign1390_e1779_d_b5, assign1390_e1779_d_b6, assign1390_e1779_d_b7, assign1390_e1779_d_b8, assign1390_e1779_d_b9, assign1390_e1779_d_b10, assign1390_e1779_d_b11, assign1390_e1779_d_b12, assign1390_e1779_d_b13, assign1390_e1779_d_b14,) = {
    if (var_guard13 != 0.0) {
        (p.p24, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgs, var_cgs_dn0, var_cgs_dn1, var_cgs_dn2, var_cgs_dn3, var_cgs_dn4, var_cgs_dn5, var_cgs_dn6, var_cgs_dn7, var_cgs_dn8, var_cgs_dn9, var_cgs_dn10, var_cgs_dn11, var_cgs_dn12, var_cgs_dn13, var_cgs_dn14, var_cgs_dn15, var_cgs_db0, var_cgs_db1, var_cgs_db2, var_cgs_db3, var_cgs_db4, var_cgs_db5, var_cgs_db6, var_cgs_db7, var_cgs_db8, var_cgs_db9, var_cgs_db10, var_cgs_db11, var_cgs_db12, var_cgs_db13, var_cgs_db14,)
    }
};
        var_cgs = assign1390_e1779;
        var_cgs_dn0 = assign1390_e1779_d_n0;
        var_cgs_dn1 = assign1390_e1779_d_n1;
        var_cgs_dn2 = assign1390_e1779_d_n2;
        var_cgs_dn3 = assign1390_e1779_d_n3;
        var_cgs_dn4 = assign1390_e1779_d_n4;
        var_cgs_dn5 = assign1390_e1779_d_n5;
        var_cgs_dn6 = assign1390_e1779_d_n6;
        var_cgs_dn7 = assign1390_e1779_d_n7;
        var_cgs_dn8 = assign1390_e1779_d_n8;
        var_cgs_dn9 = assign1390_e1779_d_n9;
        var_cgs_dn10 = assign1390_e1779_d_n10;
        var_cgs_dn11 = assign1390_e1779_d_n11;
        var_cgs_dn12 = assign1390_e1779_d_n12;
        var_cgs_dn13 = assign1390_e1779_d_n13;
        var_cgs_dn14 = assign1390_e1779_d_n14;
        var_cgs_dn15 = assign1390_e1779_d_n15;
        var_cgs_db0 = assign1390_e1779_d_b0;
        var_cgs_db1 = assign1390_e1779_d_b1;
        var_cgs_db2 = assign1390_e1779_d_b2;
        var_cgs_db3 = assign1390_e1779_d_b3;
        var_cgs_db4 = assign1390_e1779_d_b4;
        var_cgs_db5 = assign1390_e1779_d_b5;
        var_cgs_db6 = assign1390_e1779_d_b6;
        var_cgs_db7 = assign1390_e1779_d_b7;
        var_cgs_db8 = assign1390_e1779_d_b8;
        var_cgs_db9 = assign1390_e1779_d_b9;
        var_cgs_db10 = assign1390_e1779_d_b10;
        var_cgs_db11 = assign1390_e1779_d_b11;
        var_cgs_db12 = assign1390_e1779_d_b12;
        var_cgs_db13 = assign1390_e1779_d_b13;
        var_cgs_db14 = assign1390_e1779_d_b14;

        let (assign1400_e1783, assign1400_e1783_d_n0, assign1400_e1783_d_n1, assign1400_e1783_d_n2, assign1400_e1783_d_n3, assign1400_e1783_d_n4, assign1400_e1783_d_n5, assign1400_e1783_d_n6, assign1400_e1783_d_n7, assign1400_e1783_d_n8, assign1400_e1783_d_n9, assign1400_e1783_d_n10, assign1400_e1783_d_n11, assign1400_e1783_d_n12, assign1400_e1783_d_n13, assign1400_e1783_d_n14, assign1400_e1783_d_n15, assign1400_e1783_d_b0, assign1400_e1783_d_b1, assign1400_e1783_d_b2, assign1400_e1783_d_b3, assign1400_e1783_d_b4, assign1400_e1783_d_b5, assign1400_e1783_d_b6, assign1400_e1783_d_b7, assign1400_e1783_d_b8, assign1400_e1783_d_b9, assign1400_e1783_d_b10, assign1400_e1783_d_b11, assign1400_e1783_d_b12, assign1400_e1783_d_b13, assign1400_e1783_d_b14,) = {
    if (var_guard13 != 0.0) {
        (p.p26, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgd, var_cgd_dn0, var_cgd_dn1, var_cgd_dn2, var_cgd_dn3, var_cgd_dn4, var_cgd_dn5, var_cgd_dn6, var_cgd_dn7, var_cgd_dn8, var_cgd_dn9, var_cgd_dn10, var_cgd_dn11, var_cgd_dn12, var_cgd_dn13, var_cgd_dn14, var_cgd_dn15, var_cgd_db0, var_cgd_db1, var_cgd_db2, var_cgd_db3, var_cgd_db4, var_cgd_db5, var_cgd_db6, var_cgd_db7, var_cgd_db8, var_cgd_db9, var_cgd_db10, var_cgd_db11, var_cgd_db12, var_cgd_db13, var_cgd_db14,)
    }
};
        var_cgd = assign1400_e1783;
        var_cgd_dn0 = assign1400_e1783_d_n0;
        var_cgd_dn1 = assign1400_e1783_d_n1;
        var_cgd_dn2 = assign1400_e1783_d_n2;
        var_cgd_dn3 = assign1400_e1783_d_n3;
        var_cgd_dn4 = assign1400_e1783_d_n4;
        var_cgd_dn5 = assign1400_e1783_d_n5;
        var_cgd_dn6 = assign1400_e1783_d_n6;
        var_cgd_dn7 = assign1400_e1783_d_n7;
        var_cgd_dn8 = assign1400_e1783_d_n8;
        var_cgd_dn9 = assign1400_e1783_d_n9;
        var_cgd_dn10 = assign1400_e1783_d_n10;
        var_cgd_dn11 = assign1400_e1783_d_n11;
        var_cgd_dn12 = assign1400_e1783_d_n12;
        var_cgd_dn13 = assign1400_e1783_d_n13;
        var_cgd_dn14 = assign1400_e1783_d_n14;
        var_cgd_dn15 = assign1400_e1783_d_n15;
        var_cgd_db0 = assign1400_e1783_d_b0;
        var_cgd_db1 = assign1400_e1783_d_b1;
        var_cgd_db2 = assign1400_e1783_d_b2;
        var_cgd_db3 = assign1400_e1783_d_b3;
        var_cgd_db4 = assign1400_e1783_d_b4;
        var_cgd_db5 = assign1400_e1783_d_b5;
        var_cgd_db6 = assign1400_e1783_d_b6;
        var_cgd_db7 = assign1400_e1783_d_b7;
        var_cgd_db8 = assign1400_e1783_d_b8;
        var_cgd_db9 = assign1400_e1783_d_b9;
        var_cgd_db10 = assign1400_e1783_d_b10;
        var_cgd_db11 = assign1400_e1783_d_b11;
        var_cgd_db12 = assign1400_e1783_d_b12;
        var_cgd_db13 = assign1400_e1783_d_b13;
        var_cgd_db14 = assign1400_e1783_d_b14;

        let (assign1410_e1796, assign1410_e1796_d_n0, assign1410_e1796_d_n1, assign1410_e1796_d_n2, assign1410_e1796_d_n3, assign1410_e1796_d_n4, assign1410_e1796_d_n5, assign1410_e1796_d_n6, assign1410_e1796_d_n7, assign1410_e1796_d_n8, assign1410_e1796_d_n9, assign1410_e1796_d_n10, assign1410_e1796_d_n11, assign1410_e1796_d_n12, assign1410_e1796_d_n13, assign1410_e1796_d_n14, assign1410_e1796_d_n15, assign1410_e1796_d_b0, assign1410_e1796_d_b1, assign1410_e1796_d_b2, assign1410_e1796_d_b3, assign1410_e1796_d_b4, assign1410_e1796_d_b5, assign1410_e1796_d_b6, assign1410_e1796_d_b7, assign1410_e1796_d_b8, assign1410_e1796_d_b9, assign1410_e1796_d_b10, assign1410_e1796_d_b11, assign1410_e1796_d_b12, assign1410_e1796_d_b13, assign1410_e1796_d_b14,) = {
    if ((var_guard14 != 0.0) && (var_guard13 == 0.0)) {
        let assign1410_e1791: f64 = (var_cgs0_t * var_tanh1);
        let assign1410_e1793: f64 = (assign1410_e1791 * var_tanh2);
        let assign1410_e1794: f64 = (p.p24 + assign1410_e1793);
        (assign1410_e1794, ((((var_cgs0_t_dn0 * var_tanh1) + (var_cgs0_t * var_tanh1_dn0)) * var_tanh2) + (assign1410_e1791 * var_tanh2_dn0)), ((((var_cgs0_t_dn1 * var_tanh1) + (var_cgs0_t * var_tanh1_dn1)) * var_tanh2) + (assign1410_e1791 * var_tanh2_dn1)), ((((var_cgs0_t_dn2 * var_tanh1) + (var_cgs0_t * var_tanh1_dn2)) * var_tanh2) + (assign1410_e1791 * var_tanh2_dn2)), ((((var_cgs0_t_dn3 * var_tanh1) + (var_cgs0_t * var_tanh1_dn3)) * var_tanh2) + (assign1410_e1791 * var_tanh2_dn3)), ((((var_cgs0_t_dn4 * var_tanh1) + (var_cgs0_t * var_tanh1_dn4)) * var_tanh2) + (assign1410_e1791 * var_tanh2_dn4)), ((((var_cgs0_t_dn5 * var_tanh1) + (var_cgs0_t * var_tanh1_dn5)) * var_tanh2) + (assign1410_e1791 * var_tanh2_dn5)), ((((var_cgs0_t_dn6 * var_tanh1) + (var_cgs0_t * var_tanh1_dn6)) * var_tanh2) + (assign1410_e1791 * var_tanh2_dn6)), ((((var_cgs0_t_dn7 * var_tanh1) + (var_cgs0_t * var_tanh1_dn7)) * var_tanh2) + (assign1410_e1791 * var_tanh2_dn7)), ((((var_cgs0_t_dn8 * var_tanh1) + (var_cgs0_t * var_tanh1_dn8)) * var_tanh2) + (assign1410_e1791 * var_tanh2_dn8)), ((((var_cgs0_t_dn9 * var_tanh1) + (var_cgs0_t * var_tanh1_dn9)) * var_tanh2) + (assign1410_e1791 * var_tanh2_dn9)), ((((var_cgs0_t_dn10 * var_tanh1) + (var_cgs0_t * var_tanh1_dn10)) * var_tanh2) + (assign1410_e1791 * var_tanh2_dn10)), ((((var_cgs0_t_dn11 * var_tanh1) + (var_cgs0_t * var_tanh1_dn11)) * var_tanh2) + (assign1410_e1791 * var_tanh2_dn11)), ((((var_cgs0_t_dn12 * var_tanh1) + (var_cgs0_t * var_tanh1_dn12)) * var_tanh2) + (assign1410_e1791 * var_tanh2_dn12)), ((((var_cgs0_t_dn13 * var_tanh1) + (var_cgs0_t * var_tanh1_dn13)) * var_tanh2) + (assign1410_e1791 * var_tanh2_dn13)), ((((var_cgs0_t_dn14 * var_tanh1) + (var_cgs0_t * var_tanh1_dn14)) * var_tanh2) + (assign1410_e1791 * var_tanh2_dn14)), ((((var_cgs0_t_dn15 * var_tanh1) + (var_cgs0_t * var_tanh1_dn15)) * var_tanh2) + (assign1410_e1791 * var_tanh2_dn15)), ((((var_cgs0_t_db0 * var_tanh1) + (var_cgs0_t * var_tanh1_db0)) * var_tanh2) + (assign1410_e1791 * var_tanh2_db0)), ((((var_cgs0_t_db1 * var_tanh1) + (var_cgs0_t * var_tanh1_db1)) * var_tanh2) + (assign1410_e1791 * var_tanh2_db1)), ((((var_cgs0_t_db2 * var_tanh1) + (var_cgs0_t * var_tanh1_db2)) * var_tanh2) + (assign1410_e1791 * var_tanh2_db2)), ((((var_cgs0_t_db3 * var_tanh1) + (var_cgs0_t * var_tanh1_db3)) * var_tanh2) + (assign1410_e1791 * var_tanh2_db3)), ((((var_cgs0_t_db4 * var_tanh1) + (var_cgs0_t * var_tanh1_db4)) * var_tanh2) + (assign1410_e1791 * var_tanh2_db4)), ((((var_cgs0_t_db5 * var_tanh1) + (var_cgs0_t * var_tanh1_db5)) * var_tanh2) + (assign1410_e1791 * var_tanh2_db5)), ((((var_cgs0_t_db6 * var_tanh1) + (var_cgs0_t * var_tanh1_db6)) * var_tanh2) + (assign1410_e1791 * var_tanh2_db6)), ((((var_cgs0_t_db7 * var_tanh1) + (var_cgs0_t * var_tanh1_db7)) * var_tanh2) + (assign1410_e1791 * var_tanh2_db7)), ((((var_cgs0_t_db8 * var_tanh1) + (var_cgs0_t * var_tanh1_db8)) * var_tanh2) + (assign1410_e1791 * var_tanh2_db8)), ((((var_cgs0_t_db9 * var_tanh1) + (var_cgs0_t * var_tanh1_db9)) * var_tanh2) + (assign1410_e1791 * var_tanh2_db9)), ((((var_cgs0_t_db10 * var_tanh1) + (var_cgs0_t * var_tanh1_db10)) * var_tanh2) + (assign1410_e1791 * var_tanh2_db10)), ((((var_cgs0_t_db11 * var_tanh1) + (var_cgs0_t * var_tanh1_db11)) * var_tanh2) + (assign1410_e1791 * var_tanh2_db11)), ((((var_cgs0_t_db12 * var_tanh1) + (var_cgs0_t * var_tanh1_db12)) * var_tanh2) + (assign1410_e1791 * var_tanh2_db12)), ((((var_cgs0_t_db13 * var_tanh1) + (var_cgs0_t * var_tanh1_db13)) * var_tanh2) + (assign1410_e1791 * var_tanh2_db13)), ((((var_cgs0_t_db14 * var_tanh1) + (var_cgs0_t * var_tanh1_db14)) * var_tanh2) + (assign1410_e1791 * var_tanh2_db14)),)
    } else {
        (var_cgs, var_cgs_dn0, var_cgs_dn1, var_cgs_dn2, var_cgs_dn3, var_cgs_dn4, var_cgs_dn5, var_cgs_dn6, var_cgs_dn7, var_cgs_dn8, var_cgs_dn9, var_cgs_dn10, var_cgs_dn11, var_cgs_dn12, var_cgs_dn13, var_cgs_dn14, var_cgs_dn15, var_cgs_db0, var_cgs_db1, var_cgs_db2, var_cgs_db3, var_cgs_db4, var_cgs_db5, var_cgs_db6, var_cgs_db7, var_cgs_db8, var_cgs_db9, var_cgs_db10, var_cgs_db11, var_cgs_db12, var_cgs_db13, var_cgs_db14,)
    }
};
        var_cgs = assign1410_e1796;
        var_cgs_dn0 = assign1410_e1796_d_n0;
        var_cgs_dn1 = assign1410_e1796_d_n1;
        var_cgs_dn2 = assign1410_e1796_d_n2;
        var_cgs_dn3 = assign1410_e1796_d_n3;
        var_cgs_dn4 = assign1410_e1796_d_n4;
        var_cgs_dn5 = assign1410_e1796_d_n5;
        var_cgs_dn6 = assign1410_e1796_d_n6;
        var_cgs_dn7 = assign1410_e1796_d_n7;
        var_cgs_dn8 = assign1410_e1796_d_n8;
        var_cgs_dn9 = assign1410_e1796_d_n9;
        var_cgs_dn10 = assign1410_e1796_d_n10;
        var_cgs_dn11 = assign1410_e1796_d_n11;
        var_cgs_dn12 = assign1410_e1796_d_n12;
        var_cgs_dn13 = assign1410_e1796_d_n13;
        var_cgs_dn14 = assign1410_e1796_d_n14;
        var_cgs_dn15 = assign1410_e1796_d_n15;
        var_cgs_db0 = assign1410_e1796_d_b0;
        var_cgs_db1 = assign1410_e1796_d_b1;
        var_cgs_db2 = assign1410_e1796_d_b2;
        var_cgs_db3 = assign1410_e1796_d_b3;
        var_cgs_db4 = assign1410_e1796_d_b4;
        var_cgs_db5 = assign1410_e1796_d_b5;
        var_cgs_db6 = assign1410_e1796_d_b6;
        var_cgs_db7 = assign1410_e1796_d_b7;
        var_cgs_db8 = assign1410_e1796_d_b8;
        var_cgs_db9 = assign1410_e1796_d_b9;
        var_cgs_db10 = assign1410_e1796_d_b10;
        var_cgs_db11 = assign1410_e1796_d_b11;
        var_cgs_db12 = assign1410_e1796_d_b12;
        var_cgs_db13 = assign1410_e1796_d_b13;
        var_cgs_db14 = assign1410_e1796_d_b14;


        *var_cgd_slot = var_cgd;
        *var_cgd_db0_slot = var_cgd_db0;
        *var_cgd_db1_slot = var_cgd_db1;
        *var_cgd_db10_slot = var_cgd_db10;
        *var_cgd_db11_slot = var_cgd_db11;
        *var_cgd_db12_slot = var_cgd_db12;
        *var_cgd_db13_slot = var_cgd_db13;
        *var_cgd_db14_slot = var_cgd_db14;
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
        *var_cgs_slot = var_cgs;
        *var_cgs_db0_slot = var_cgs_db0;
        *var_cgs_db1_slot = var_cgs_db1;
        *var_cgs_db10_slot = var_cgs_db10;
        *var_cgs_db11_slot = var_cgs_db11;
        *var_cgs_db12_slot = var_cgs_db12;
        *var_cgs_db13_slot = var_cgs_db13;
        *var_cgs_db14_slot = var_cgs_db14;
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
        *var_guard13_slot = var_guard13;
        *var_guard14_slot = var_guard14;
        *var_guard15_slot = var_guard15;
        *var_psi_1_slot = var_psi_1;
        *var_psi_1_db0_slot = var_psi_1_db0;
        *var_psi_1_db1_slot = var_psi_1_db1;
        *var_psi_1_db10_slot = var_psi_1_db10;
        *var_psi_1_db11_slot = var_psi_1_db11;
        *var_psi_1_db12_slot = var_psi_1_db12;
        *var_psi_1_db13_slot = var_psi_1_db13;
        *var_psi_1_db14_slot = var_psi_1_db14;
        *var_psi_1_db2_slot = var_psi_1_db2;
        *var_psi_1_db3_slot = var_psi_1_db3;
        *var_psi_1_db4_slot = var_psi_1_db4;
        *var_psi_1_db5_slot = var_psi_1_db5;
        *var_psi_1_db6_slot = var_psi_1_db6;
        *var_psi_1_db7_slot = var_psi_1_db7;
        *var_psi_1_db8_slot = var_psi_1_db8;
        *var_psi_1_db9_slot = var_psi_1_db9;
        *var_psi_1_dn0_slot = var_psi_1_dn0;
        *var_psi_1_dn1_slot = var_psi_1_dn1;
        *var_psi_1_dn10_slot = var_psi_1_dn10;
        *var_psi_1_dn11_slot = var_psi_1_dn11;
        *var_psi_1_dn12_slot = var_psi_1_dn12;
        *var_psi_1_dn13_slot = var_psi_1_dn13;
        *var_psi_1_dn14_slot = var_psi_1_dn14;
        *var_psi_1_dn15_slot = var_psi_1_dn15;
        *var_psi_1_dn2_slot = var_psi_1_dn2;
        *var_psi_1_dn3_slot = var_psi_1_dn3;
        *var_psi_1_dn4_slot = var_psi_1_dn4;
        *var_psi_1_dn5_slot = var_psi_1_dn5;
        *var_psi_1_dn6_slot = var_psi_1_dn6;
        *var_psi_1_dn7_slot = var_psi_1_dn7;
        *var_psi_1_dn8_slot = var_psi_1_dn8;
        *var_psi_1_dn9_slot = var_psi_1_dn9;
        *var_psi_2_slot = var_psi_2;
        *var_psi_2_db0_slot = var_psi_2_db0;
        *var_psi_2_db1_slot = var_psi_2_db1;
        *var_psi_2_db10_slot = var_psi_2_db10;
        *var_psi_2_db11_slot = var_psi_2_db11;
        *var_psi_2_db12_slot = var_psi_2_db12;
        *var_psi_2_db13_slot = var_psi_2_db13;
        *var_psi_2_db14_slot = var_psi_2_db14;
        *var_psi_2_db2_slot = var_psi_2_db2;
        *var_psi_2_db3_slot = var_psi_2_db3;
        *var_psi_2_db4_slot = var_psi_2_db4;
        *var_psi_2_db5_slot = var_psi_2_db5;
        *var_psi_2_db6_slot = var_psi_2_db6;
        *var_psi_2_db7_slot = var_psi_2_db7;
        *var_psi_2_db8_slot = var_psi_2_db8;
        *var_psi_2_db9_slot = var_psi_2_db9;
        *var_psi_2_dn0_slot = var_psi_2_dn0;
        *var_psi_2_dn1_slot = var_psi_2_dn1;
        *var_psi_2_dn10_slot = var_psi_2_dn10;
        *var_psi_2_dn11_slot = var_psi_2_dn11;
        *var_psi_2_dn12_slot = var_psi_2_dn12;
        *var_psi_2_dn13_slot = var_psi_2_dn13;
        *var_psi_2_dn14_slot = var_psi_2_dn14;
        *var_psi_2_dn15_slot = var_psi_2_dn15;
        *var_psi_2_dn2_slot = var_psi_2_dn2;
        *var_psi_2_dn3_slot = var_psi_2_dn3;
        *var_psi_2_dn4_slot = var_psi_2_dn4;
        *var_psi_2_dn5_slot = var_psi_2_dn5;
        *var_psi_2_dn6_slot = var_psi_2_dn6;
        *var_psi_2_dn7_slot = var_psi_2_dn7;
        *var_psi_2_dn8_slot = var_psi_2_dn8;
        *var_psi_2_dn9_slot = var_psi_2_dn9;
        *var_psi_3_slot = var_psi_3;
        *var_psi_3_db0_slot = var_psi_3_db0;
        *var_psi_3_db1_slot = var_psi_3_db1;
        *var_psi_3_db10_slot = var_psi_3_db10;
        *var_psi_3_db11_slot = var_psi_3_db11;
        *var_psi_3_db12_slot = var_psi_3_db12;
        *var_psi_3_db13_slot = var_psi_3_db13;
        *var_psi_3_db14_slot = var_psi_3_db14;
        *var_psi_3_db2_slot = var_psi_3_db2;
        *var_psi_3_db3_slot = var_psi_3_db3;
        *var_psi_3_db4_slot = var_psi_3_db4;
        *var_psi_3_db5_slot = var_psi_3_db5;
        *var_psi_3_db6_slot = var_psi_3_db6;
        *var_psi_3_db7_slot = var_psi_3_db7;
        *var_psi_3_db8_slot = var_psi_3_db8;
        *var_psi_3_db9_slot = var_psi_3_db9;
        *var_psi_3_dn0_slot = var_psi_3_dn0;
        *var_psi_3_dn1_slot = var_psi_3_dn1;
        *var_psi_3_dn10_slot = var_psi_3_dn10;
        *var_psi_3_dn11_slot = var_psi_3_dn11;
        *var_psi_3_dn12_slot = var_psi_3_dn12;
        *var_psi_3_dn13_slot = var_psi_3_dn13;
        *var_psi_3_dn14_slot = var_psi_3_dn14;
        *var_psi_3_dn15_slot = var_psi_3_dn15;
        *var_psi_3_dn2_slot = var_psi_3_dn2;
        *var_psi_3_dn3_slot = var_psi_3_dn3;
        *var_psi_3_dn4_slot = var_psi_3_dn4;
        *var_psi_3_dn5_slot = var_psi_3_dn5;
        *var_psi_3_dn6_slot = var_psi_3_dn6;
        *var_psi_3_dn7_slot = var_psi_3_dn7;
        *var_psi_3_dn8_slot = var_psi_3_dn8;
        *var_psi_3_dn9_slot = var_psi_3_dn9;
        *var_psi_4_slot = var_psi_4;
        *var_psi_4_db0_slot = var_psi_4_db0;
        *var_psi_4_db1_slot = var_psi_4_db1;
        *var_psi_4_db10_slot = var_psi_4_db10;
        *var_psi_4_db11_slot = var_psi_4_db11;
        *var_psi_4_db12_slot = var_psi_4_db12;
        *var_psi_4_db13_slot = var_psi_4_db13;
        *var_psi_4_db14_slot = var_psi_4_db14;
        *var_psi_4_db2_slot = var_psi_4_db2;
        *var_psi_4_db3_slot = var_psi_4_db3;
        *var_psi_4_db4_slot = var_psi_4_db4;
        *var_psi_4_db5_slot = var_psi_4_db5;
        *var_psi_4_db6_slot = var_psi_4_db6;
        *var_psi_4_db7_slot = var_psi_4_db7;
        *var_psi_4_db8_slot = var_psi_4_db8;
        *var_psi_4_db9_slot = var_psi_4_db9;
        *var_psi_4_dn0_slot = var_psi_4_dn0;
        *var_psi_4_dn1_slot = var_psi_4_dn1;
        *var_psi_4_dn10_slot = var_psi_4_dn10;
        *var_psi_4_dn11_slot = var_psi_4_dn11;
        *var_psi_4_dn12_slot = var_psi_4_dn12;
        *var_psi_4_dn13_slot = var_psi_4_dn13;
        *var_psi_4_dn14_slot = var_psi_4_dn14;
        *var_psi_4_dn15_slot = var_psi_4_dn15;
        *var_psi_4_dn2_slot = var_psi_4_dn2;
        *var_psi_4_dn3_slot = var_psi_4_dn3;
        *var_psi_4_dn4_slot = var_psi_4_dn4;
        *var_psi_4_dn5_slot = var_psi_4_dn5;
        *var_psi_4_dn6_slot = var_psi_4_dn6;
        *var_psi_4_dn7_slot = var_psi_4_dn7;
        *var_psi_4_dn8_slot = var_psi_4_dn8;
        *var_psi_4_dn9_slot = var_psi_4_dn9;
        *var_t0_slot = var_t0;
        *var_t0_db0_slot = var_t0_db0;
        *var_t0_db1_slot = var_t0_db1;
        *var_t0_db10_slot = var_t0_db10;
        *var_t0_db11_slot = var_t0_db11;
        *var_t0_db12_slot = var_t0_db12;
        *var_t0_db13_slot = var_t0_db13;
        *var_t0_db14_slot = var_t0_db14;
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
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_tanh1_slot = var_tanh1;
        *var_tanh1_db0_slot = var_tanh1_db0;
        *var_tanh1_db1_slot = var_tanh1_db1;
        *var_tanh1_db10_slot = var_tanh1_db10;
        *var_tanh1_db11_slot = var_tanh1_db11;
        *var_tanh1_db12_slot = var_tanh1_db12;
        *var_tanh1_db13_slot = var_tanh1_db13;
        *var_tanh1_db14_slot = var_tanh1_db14;
        *var_tanh1_db2_slot = var_tanh1_db2;
        *var_tanh1_db3_slot = var_tanh1_db3;
        *var_tanh1_db4_slot = var_tanh1_db4;
        *var_tanh1_db5_slot = var_tanh1_db5;
        *var_tanh1_db6_slot = var_tanh1_db6;
        *var_tanh1_db7_slot = var_tanh1_db7;
        *var_tanh1_db8_slot = var_tanh1_db8;
        *var_tanh1_db9_slot = var_tanh1_db9;
        *var_tanh1_dn0_slot = var_tanh1_dn0;
        *var_tanh1_dn1_slot = var_tanh1_dn1;
        *var_tanh1_dn10_slot = var_tanh1_dn10;
        *var_tanh1_dn11_slot = var_tanh1_dn11;
        *var_tanh1_dn12_slot = var_tanh1_dn12;
        *var_tanh1_dn13_slot = var_tanh1_dn13;
        *var_tanh1_dn14_slot = var_tanh1_dn14;
        *var_tanh1_dn15_slot = var_tanh1_dn15;
        *var_tanh1_dn2_slot = var_tanh1_dn2;
        *var_tanh1_dn3_slot = var_tanh1_dn3;
        *var_tanh1_dn4_slot = var_tanh1_dn4;
        *var_tanh1_dn5_slot = var_tanh1_dn5;
        *var_tanh1_dn6_slot = var_tanh1_dn6;
        *var_tanh1_dn7_slot = var_tanh1_dn7;
        *var_tanh1_dn8_slot = var_tanh1_dn8;
        *var_tanh1_dn9_slot = var_tanh1_dn9;
        *var_tanh2_slot = var_tanh2;
        *var_tanh2_db0_slot = var_tanh2_db0;
        *var_tanh2_db1_slot = var_tanh2_db1;
        *var_tanh2_db10_slot = var_tanh2_db10;
        *var_tanh2_db11_slot = var_tanh2_db11;
        *var_tanh2_db12_slot = var_tanh2_db12;
        *var_tanh2_db13_slot = var_tanh2_db13;
        *var_tanh2_db14_slot = var_tanh2_db14;
        *var_tanh2_db2_slot = var_tanh2_db2;
        *var_tanh2_db3_slot = var_tanh2_db3;
        *var_tanh2_db4_slot = var_tanh2_db4;
        *var_tanh2_db5_slot = var_tanh2_db5;
        *var_tanh2_db6_slot = var_tanh2_db6;
        *var_tanh2_db7_slot = var_tanh2_db7;
        *var_tanh2_db8_slot = var_tanh2_db8;
        *var_tanh2_db9_slot = var_tanh2_db9;
        *var_tanh2_dn0_slot = var_tanh2_dn0;
        *var_tanh2_dn1_slot = var_tanh2_dn1;
        *var_tanh2_dn10_slot = var_tanh2_dn10;
        *var_tanh2_dn11_slot = var_tanh2_dn11;
        *var_tanh2_dn12_slot = var_tanh2_dn12;
        *var_tanh2_dn13_slot = var_tanh2_dn13;
        *var_tanh2_dn14_slot = var_tanh2_dn14;
        *var_tanh2_dn15_slot = var_tanh2_dn15;
        *var_tanh2_dn2_slot = var_tanh2_dn2;
        *var_tanh2_dn3_slot = var_tanh2_dn3;
        *var_tanh2_dn4_slot = var_tanh2_dn4;
        *var_tanh2_dn5_slot = var_tanh2_dn5;
        *var_tanh2_dn6_slot = var_tanh2_dn6;
        *var_tanh2_dn7_slot = var_tanh2_dn7;
        *var_tanh2_dn8_slot = var_tanh2_dn8;
        *var_tanh2_dn9_slot = var_tanh2_dn9;
        *var_tanh3_slot = var_tanh3;
        *var_tanh3_db0_slot = var_tanh3_db0;
        *var_tanh3_db1_slot = var_tanh3_db1;
        *var_tanh3_db10_slot = var_tanh3_db10;
        *var_tanh3_db11_slot = var_tanh3_db11;
        *var_tanh3_db12_slot = var_tanh3_db12;
        *var_tanh3_db13_slot = var_tanh3_db13;
        *var_tanh3_db14_slot = var_tanh3_db14;
        *var_tanh3_db2_slot = var_tanh3_db2;
        *var_tanh3_db3_slot = var_tanh3_db3;
        *var_tanh3_db4_slot = var_tanh3_db4;
        *var_tanh3_db5_slot = var_tanh3_db5;
        *var_tanh3_db6_slot = var_tanh3_db6;
        *var_tanh3_db7_slot = var_tanh3_db7;
        *var_tanh3_db8_slot = var_tanh3_db8;
        *var_tanh3_db9_slot = var_tanh3_db9;
        *var_tanh3_dn0_slot = var_tanh3_dn0;
        *var_tanh3_dn1_slot = var_tanh3_dn1;
        *var_tanh3_dn10_slot = var_tanh3_dn10;
        *var_tanh3_dn11_slot = var_tanh3_dn11;
        *var_tanh3_dn12_slot = var_tanh3_dn12;
        *var_tanh3_dn13_slot = var_tanh3_dn13;
        *var_tanh3_dn14_slot = var_tanh3_dn14;
        *var_tanh3_dn15_slot = var_tanh3_dn15;
        *var_tanh3_dn2_slot = var_tanh3_dn2;
        *var_tanh3_dn3_slot = var_tanh3_dn3;
        *var_tanh3_dn4_slot = var_tanh3_dn4;
        *var_tanh3_dn5_slot = var_tanh3_dn5;
        *var_tanh3_dn6_slot = var_tanh3_dn6;
        *var_tanh3_dn7_slot = var_tanh3_dn7;
        *var_tanh3_dn8_slot = var_tanh3_dn8;
        *var_tanh3_dn9_slot = var_tanh3_dn9;
        *var_tanh4_slot = var_tanh4;
        *var_tanh4_db0_slot = var_tanh4_db0;
        *var_tanh4_db1_slot = var_tanh4_db1;
        *var_tanh4_db10_slot = var_tanh4_db10;
        *var_tanh4_db11_slot = var_tanh4_db11;
        *var_tanh4_db12_slot = var_tanh4_db12;
        *var_tanh4_db13_slot = var_tanh4_db13;
        *var_tanh4_db14_slot = var_tanh4_db14;
        *var_tanh4_db2_slot = var_tanh4_db2;
        *var_tanh4_db3_slot = var_tanh4_db3;
        *var_tanh4_db4_slot = var_tanh4_db4;
        *var_tanh4_db5_slot = var_tanh4_db5;
        *var_tanh4_db6_slot = var_tanh4_db6;
        *var_tanh4_db7_slot = var_tanh4_db7;
        *var_tanh4_db8_slot = var_tanh4_db8;
        *var_tanh4_db9_slot = var_tanh4_db9;
        *var_tanh4_dn0_slot = var_tanh4_dn0;
        *var_tanh4_dn1_slot = var_tanh4_dn1;
        *var_tanh4_dn10_slot = var_tanh4_dn10;
        *var_tanh4_dn11_slot = var_tanh4_dn11;
        *var_tanh4_dn12_slot = var_tanh4_dn12;
        *var_tanh4_dn13_slot = var_tanh4_dn13;
        *var_tanh4_dn14_slot = var_tanh4_dn14;
        *var_tanh4_dn15_slot = var_tanh4_dn15;
        *var_tanh4_dn2_slot = var_tanh4_dn2;
        *var_tanh4_dn3_slot = var_tanh4_dn3;
        *var_tanh4_dn4_slot = var_tanh4_dn4;
        *var_tanh4_dn5_slot = var_tanh4_dn5;
        *var_tanh4_dn6_slot = var_tanh4_dn6;
        *var_tanh4_dn7_slot = var_tanh4_dn7;
        *var_tanh4_dn8_slot = var_tanh4_dn8;
        *var_tanh4_dn9_slot = var_tanh4_dn9;
    }

    pub(super) fn stamp_transient_block_6(
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
        var_tanh4: f64,
        var_tanh4_db0: f64,
        var_tanh4_db1: f64,
        var_tanh4_db10: f64,
        var_tanh4_db11: f64,
        var_tanh4_db12: f64,
        var_tanh4_db13: f64,
        var_tanh4_db14: f64,
        var_tanh4_db2: f64,
        var_tanh4_db3: f64,
        var_tanh4_db4: f64,
        var_tanh4_db5: f64,
        var_tanh4_db6: f64,
        var_tanh4_db7: f64,
        var_tanh4_db8: f64,
        var_tanh4_db9: f64,
        var_tanh4_dn0: f64,
        var_tanh4_dn1: f64,
        var_tanh4_dn10: f64,
        var_tanh4_dn11: f64,
        var_tanh4_dn12: f64,
        var_tanh4_dn13: f64,
        var_tanh4_dn14: f64,
        var_tanh4_dn15: f64,
        var_tanh4_dn2: f64,
        var_tanh4_dn3: f64,
        var_tanh4_dn4: f64,
        var_tanh4_dn5: f64,
        var_tanh4_dn6: f64,
        var_tanh4_dn7: f64,
        var_tanh4_dn8: f64,
        var_tanh4_dn9: f64,
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
        var_cgd_slot: &mut f64,
        var_cgd_db0_slot: &mut f64,
        var_cgd_db1_slot: &mut f64,
        var_cgd_db10_slot: &mut f64,
        var_cgd_db11_slot: &mut f64,
        var_cgd_db12_slot: &mut f64,
        var_cgd_db13_slot: &mut f64,
        var_cgd_db14_slot: &mut f64,
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
        var_cosh0_slot: &mut f64,
        var_cosh0_db0_slot: &mut f64,
        var_cosh0_db1_slot: &mut f64,
        var_cosh0_db10_slot: &mut f64,
        var_cosh0_db11_slot: &mut f64,
        var_cosh0_db12_slot: &mut f64,
        var_cosh0_db13_slot: &mut f64,
        var_cosh0_db14_slot: &mut f64,
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
        var_cosh1_slot: &mut f64,
        var_cosh1_db0_slot: &mut f64,
        var_cosh1_db1_slot: &mut f64,
        var_cosh1_db10_slot: &mut f64,
        var_cosh1_db11_slot: &mut f64,
        var_cosh1_db12_slot: &mut f64,
        var_cosh1_db13_slot: &mut f64,
        var_cosh1_db14_slot: &mut f64,
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
        var_lc1_slot: &mut f64,
        var_lc10_slot: &mut f64,
        var_lc10_db0_slot: &mut f64,
        var_lc10_db1_slot: &mut f64,
        var_lc10_db10_slot: &mut f64,
        var_lc10_db11_slot: &mut f64,
        var_lc10_db12_slot: &mut f64,
        var_lc10_db13_slot: &mut f64,
        var_lc10_db14_slot: &mut f64,
        var_lc10_db2_slot: &mut f64,
        var_lc10_db3_slot: &mut f64,
        var_lc10_db4_slot: &mut f64,
        var_lc10_db5_slot: &mut f64,
        var_lc10_db6_slot: &mut f64,
        var_lc10_db7_slot: &mut f64,
        var_lc10_db8_slot: &mut f64,
        var_lc10_db9_slot: &mut f64,
        var_lc10_dn0_slot: &mut f64,
        var_lc10_dn1_slot: &mut f64,
        var_lc10_dn10_slot: &mut f64,
        var_lc10_dn11_slot: &mut f64,
        var_lc10_dn12_slot: &mut f64,
        var_lc10_dn13_slot: &mut f64,
        var_lc10_dn14_slot: &mut f64,
        var_lc10_dn15_slot: &mut f64,
        var_lc10_dn2_slot: &mut f64,
        var_lc10_dn3_slot: &mut f64,
        var_lc10_dn4_slot: &mut f64,
        var_lc10_dn5_slot: &mut f64,
        var_lc10_dn6_slot: &mut f64,
        var_lc10_dn7_slot: &mut f64,
        var_lc10_dn8_slot: &mut f64,
        var_lc10_dn9_slot: &mut f64,
        var_lc1_db0_slot: &mut f64,
        var_lc1_db1_slot: &mut f64,
        var_lc1_db10_slot: &mut f64,
        var_lc1_db11_slot: &mut f64,
        var_lc1_db12_slot: &mut f64,
        var_lc1_db13_slot: &mut f64,
        var_lc1_db14_slot: &mut f64,
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
        var_lc4_slot: &mut f64,
        var_lc40_slot: &mut f64,
        var_lc40_db0_slot: &mut f64,
        var_lc40_db1_slot: &mut f64,
        var_lc40_db10_slot: &mut f64,
        var_lc40_db11_slot: &mut f64,
        var_lc40_db12_slot: &mut f64,
        var_lc40_db13_slot: &mut f64,
        var_lc40_db14_slot: &mut f64,
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
        var_lc4_db0_slot: &mut f64,
        var_lc4_db1_slot: &mut f64,
        var_lc4_db10_slot: &mut f64,
        var_lc4_db11_slot: &mut f64,
        var_lc4_db12_slot: &mut f64,
        var_lc4_db13_slot: &mut f64,
        var_lc4_db14_slot: &mut f64,
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
        var_qgs_slot: &mut f64,
        var_qgs0_slot: &mut f64,
        var_qgs0_db0_slot: &mut f64,
        var_qgs0_db1_slot: &mut f64,
        var_qgs0_db10_slot: &mut f64,
        var_qgs0_db11_slot: &mut f64,
        var_qgs0_db12_slot: &mut f64,
        var_qgs0_db13_slot: &mut f64,
        var_qgs0_db14_slot: &mut f64,
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
        var_qgs_db0_slot: &mut f64,
        var_qgs_db1_slot: &mut f64,
        var_qgs_db10_slot: &mut f64,
        var_qgs_db11_slot: &mut f64,
        var_qgs_db12_slot: &mut f64,
        var_qgs_db13_slot: &mut f64,
        var_qgs_db14_slot: &mut f64,
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
        var_tanh2_slot: &mut f64,
        var_tanh2_db0_slot: &mut f64,
        var_tanh2_db1_slot: &mut f64,
        var_tanh2_db10_slot: &mut f64,
        var_tanh2_db11_slot: &mut f64,
        var_tanh2_db12_slot: &mut f64,
        var_tanh2_db13_slot: &mut f64,
        var_tanh2_db14_slot: &mut f64,
        var_tanh2_db2_slot: &mut f64,
        var_tanh2_db3_slot: &mut f64,
        var_tanh2_db4_slot: &mut f64,
        var_tanh2_db5_slot: &mut f64,
        var_tanh2_db6_slot: &mut f64,
        var_tanh2_db7_slot: &mut f64,
        var_tanh2_db8_slot: &mut f64,
        var_tanh2_db9_slot: &mut f64,
        var_tanh2_dn0_slot: &mut f64,
        var_tanh2_dn1_slot: &mut f64,
        var_tanh2_dn10_slot: &mut f64,
        var_tanh2_dn11_slot: &mut f64,
        var_tanh2_dn12_slot: &mut f64,
        var_tanh2_dn13_slot: &mut f64,
        var_tanh2_dn14_slot: &mut f64,
        var_tanh2_dn15_slot: &mut f64,
        var_tanh2_dn2_slot: &mut f64,
        var_tanh2_dn3_slot: &mut f64,
        var_tanh2_dn4_slot: &mut f64,
        var_tanh2_dn5_slot: &mut f64,
        var_tanh2_dn6_slot: &mut f64,
        var_tanh2_dn7_slot: &mut f64,
        var_tanh2_dn8_slot: &mut f64,
        var_tanh2_dn9_slot: &mut f64,
    ) {
        let mut var_cgd: f64 = *var_cgd_slot;
        let mut var_cgd_db0: f64 = *var_cgd_db0_slot;
        let mut var_cgd_db1: f64 = *var_cgd_db1_slot;
        let mut var_cgd_db10: f64 = *var_cgd_db10_slot;
        let mut var_cgd_db11: f64 = *var_cgd_db11_slot;
        let mut var_cgd_db12: f64 = *var_cgd_db12_slot;
        let mut var_cgd_db13: f64 = *var_cgd_db13_slot;
        let mut var_cgd_db14: f64 = *var_cgd_db14_slot;
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
        let mut var_cosh0: f64 = *var_cosh0_slot;
        let mut var_cosh0_db0: f64 = *var_cosh0_db0_slot;
        let mut var_cosh0_db1: f64 = *var_cosh0_db1_slot;
        let mut var_cosh0_db10: f64 = *var_cosh0_db10_slot;
        let mut var_cosh0_db11: f64 = *var_cosh0_db11_slot;
        let mut var_cosh0_db12: f64 = *var_cosh0_db12_slot;
        let mut var_cosh0_db13: f64 = *var_cosh0_db13_slot;
        let mut var_cosh0_db14: f64 = *var_cosh0_db14_slot;
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
        let mut var_cosh1: f64 = *var_cosh1_slot;
        let mut var_cosh1_db0: f64 = *var_cosh1_db0_slot;
        let mut var_cosh1_db1: f64 = *var_cosh1_db1_slot;
        let mut var_cosh1_db10: f64 = *var_cosh1_db10_slot;
        let mut var_cosh1_db11: f64 = *var_cosh1_db11_slot;
        let mut var_cosh1_db12: f64 = *var_cosh1_db12_slot;
        let mut var_cosh1_db13: f64 = *var_cosh1_db13_slot;
        let mut var_cosh1_db14: f64 = *var_cosh1_db14_slot;
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
        let mut var_lc1: f64 = *var_lc1_slot;
        let mut var_lc10: f64 = *var_lc10_slot;
        let mut var_lc10_db0: f64 = *var_lc10_db0_slot;
        let mut var_lc10_db1: f64 = *var_lc10_db1_slot;
        let mut var_lc10_db10: f64 = *var_lc10_db10_slot;
        let mut var_lc10_db11: f64 = *var_lc10_db11_slot;
        let mut var_lc10_db12: f64 = *var_lc10_db12_slot;
        let mut var_lc10_db13: f64 = *var_lc10_db13_slot;
        let mut var_lc10_db14: f64 = *var_lc10_db14_slot;
        let mut var_lc10_db2: f64 = *var_lc10_db2_slot;
        let mut var_lc10_db3: f64 = *var_lc10_db3_slot;
        let mut var_lc10_db4: f64 = *var_lc10_db4_slot;
        let mut var_lc10_db5: f64 = *var_lc10_db5_slot;
        let mut var_lc10_db6: f64 = *var_lc10_db6_slot;
        let mut var_lc10_db7: f64 = *var_lc10_db7_slot;
        let mut var_lc10_db8: f64 = *var_lc10_db8_slot;
        let mut var_lc10_db9: f64 = *var_lc10_db9_slot;
        let mut var_lc10_dn0: f64 = *var_lc10_dn0_slot;
        let mut var_lc10_dn1: f64 = *var_lc10_dn1_slot;
        let mut var_lc10_dn10: f64 = *var_lc10_dn10_slot;
        let mut var_lc10_dn11: f64 = *var_lc10_dn11_slot;
        let mut var_lc10_dn12: f64 = *var_lc10_dn12_slot;
        let mut var_lc10_dn13: f64 = *var_lc10_dn13_slot;
        let mut var_lc10_dn14: f64 = *var_lc10_dn14_slot;
        let mut var_lc10_dn15: f64 = *var_lc10_dn15_slot;
        let mut var_lc10_dn2: f64 = *var_lc10_dn2_slot;
        let mut var_lc10_dn3: f64 = *var_lc10_dn3_slot;
        let mut var_lc10_dn4: f64 = *var_lc10_dn4_slot;
        let mut var_lc10_dn5: f64 = *var_lc10_dn5_slot;
        let mut var_lc10_dn6: f64 = *var_lc10_dn6_slot;
        let mut var_lc10_dn7: f64 = *var_lc10_dn7_slot;
        let mut var_lc10_dn8: f64 = *var_lc10_dn8_slot;
        let mut var_lc10_dn9: f64 = *var_lc10_dn9_slot;
        let mut var_lc1_db0: f64 = *var_lc1_db0_slot;
        let mut var_lc1_db1: f64 = *var_lc1_db1_slot;
        let mut var_lc1_db10: f64 = *var_lc1_db10_slot;
        let mut var_lc1_db11: f64 = *var_lc1_db11_slot;
        let mut var_lc1_db12: f64 = *var_lc1_db12_slot;
        let mut var_lc1_db13: f64 = *var_lc1_db13_slot;
        let mut var_lc1_db14: f64 = *var_lc1_db14_slot;
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
        let mut var_lc4: f64 = *var_lc4_slot;
        let mut var_lc40: f64 = *var_lc40_slot;
        let mut var_lc40_db0: f64 = *var_lc40_db0_slot;
        let mut var_lc40_db1: f64 = *var_lc40_db1_slot;
        let mut var_lc40_db10: f64 = *var_lc40_db10_slot;
        let mut var_lc40_db11: f64 = *var_lc40_db11_slot;
        let mut var_lc40_db12: f64 = *var_lc40_db12_slot;
        let mut var_lc40_db13: f64 = *var_lc40_db13_slot;
        let mut var_lc40_db14: f64 = *var_lc40_db14_slot;
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
        let mut var_lc4_db0: f64 = *var_lc4_db0_slot;
        let mut var_lc4_db1: f64 = *var_lc4_db1_slot;
        let mut var_lc4_db10: f64 = *var_lc4_db10_slot;
        let mut var_lc4_db11: f64 = *var_lc4_db11_slot;
        let mut var_lc4_db12: f64 = *var_lc4_db12_slot;
        let mut var_lc4_db13: f64 = *var_lc4_db13_slot;
        let mut var_lc4_db14: f64 = *var_lc4_db14_slot;
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
        let mut var_qgs: f64 = *var_qgs_slot;
        let mut var_qgs0: f64 = *var_qgs0_slot;
        let mut var_qgs0_db0: f64 = *var_qgs0_db0_slot;
        let mut var_qgs0_db1: f64 = *var_qgs0_db1_slot;
        let mut var_qgs0_db10: f64 = *var_qgs0_db10_slot;
        let mut var_qgs0_db11: f64 = *var_qgs0_db11_slot;
        let mut var_qgs0_db12: f64 = *var_qgs0_db12_slot;
        let mut var_qgs0_db13: f64 = *var_qgs0_db13_slot;
        let mut var_qgs0_db14: f64 = *var_qgs0_db14_slot;
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
        let mut var_qgs_db0: f64 = *var_qgs_db0_slot;
        let mut var_qgs_db1: f64 = *var_qgs_db1_slot;
        let mut var_qgs_db10: f64 = *var_qgs_db10_slot;
        let mut var_qgs_db11: f64 = *var_qgs_db11_slot;
        let mut var_qgs_db12: f64 = *var_qgs_db12_slot;
        let mut var_qgs_db13: f64 = *var_qgs_db13_slot;
        let mut var_qgs_db14: f64 = *var_qgs_db14_slot;
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
        let mut var_tanh2: f64 = *var_tanh2_slot;
        let mut var_tanh2_db0: f64 = *var_tanh2_db0_slot;
        let mut var_tanh2_db1: f64 = *var_tanh2_db1_slot;
        let mut var_tanh2_db10: f64 = *var_tanh2_db10_slot;
        let mut var_tanh2_db11: f64 = *var_tanh2_db11_slot;
        let mut var_tanh2_db12: f64 = *var_tanh2_db12_slot;
        let mut var_tanh2_db13: f64 = *var_tanh2_db13_slot;
        let mut var_tanh2_db14: f64 = *var_tanh2_db14_slot;
        let mut var_tanh2_db2: f64 = *var_tanh2_db2_slot;
        let mut var_tanh2_db3: f64 = *var_tanh2_db3_slot;
        let mut var_tanh2_db4: f64 = *var_tanh2_db4_slot;
        let mut var_tanh2_db5: f64 = *var_tanh2_db5_slot;
        let mut var_tanh2_db6: f64 = *var_tanh2_db6_slot;
        let mut var_tanh2_db7: f64 = *var_tanh2_db7_slot;
        let mut var_tanh2_db8: f64 = *var_tanh2_db8_slot;
        let mut var_tanh2_db9: f64 = *var_tanh2_db9_slot;
        let mut var_tanh2_dn0: f64 = *var_tanh2_dn0_slot;
        let mut var_tanh2_dn1: f64 = *var_tanh2_dn1_slot;
        let mut var_tanh2_dn10: f64 = *var_tanh2_dn10_slot;
        let mut var_tanh2_dn11: f64 = *var_tanh2_dn11_slot;
        let mut var_tanh2_dn12: f64 = *var_tanh2_dn12_slot;
        let mut var_tanh2_dn13: f64 = *var_tanh2_dn13_slot;
        let mut var_tanh2_dn14: f64 = *var_tanh2_dn14_slot;
        let mut var_tanh2_dn15: f64 = *var_tanh2_dn15_slot;
        let mut var_tanh2_dn2: f64 = *var_tanh2_dn2_slot;
        let mut var_tanh2_dn3: f64 = *var_tanh2_dn3_slot;
        let mut var_tanh2_dn4: f64 = *var_tanh2_dn4_slot;
        let mut var_tanh2_dn5: f64 = *var_tanh2_dn5_slot;
        let mut var_tanh2_dn6: f64 = *var_tanh2_dn6_slot;
        let mut var_tanh2_dn7: f64 = *var_tanh2_dn7_slot;
        let mut var_tanh2_dn8: f64 = *var_tanh2_dn8_slot;
        let mut var_tanh2_dn9: f64 = *var_tanh2_dn9_slot;

        let (assign1420_e1813, assign1420_e1813_d_n0, assign1420_e1813_d_n1, assign1420_e1813_d_n2, assign1420_e1813_d_n3, assign1420_e1813_d_n4, assign1420_e1813_d_n5, assign1420_e1813_d_n6, assign1420_e1813_d_n7, assign1420_e1813_d_n8, assign1420_e1813_d_n9, assign1420_e1813_d_n10, assign1420_e1813_d_n11, assign1420_e1813_d_n12, assign1420_e1813_d_n13, assign1420_e1813_d_n14, assign1420_e1813_d_n15, assign1420_e1813_d_b0, assign1420_e1813_d_b1, assign1420_e1813_d_b2, assign1420_e1813_d_b3, assign1420_e1813_d_b4, assign1420_e1813_d_b5, assign1420_e1813_d_b6, assign1420_e1813_d_b7, assign1420_e1813_d_b8, assign1420_e1813_d_b9, assign1420_e1813_d_b10, assign1420_e1813_d_b11, assign1420_e1813_d_b12, assign1420_e1813_d_b13, assign1420_e1813_d_b14,) = {
    if ((var_guard14 != 0.0) && (var_guard13 == 0.0)) {
        let assign1420_e1805: f64 = (var_tanh3 * var_tanh4);
        let assign1420_e1808: f64 = (2.0 * p.p37);
        let assign1420_e1809: f64 = (assign1420_e1805 + assign1420_e1808);
        let assign1420_e1810: f64 = (var_cgd0_t * assign1420_e1809);
        let assign1420_e1811: f64 = (p.p26 + assign1420_e1810);
        (assign1420_e1811, ((var_cgd0_t_dn0 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_dn0 * var_tanh4) + (var_tanh3 * var_tanh4_dn0)))), ((var_cgd0_t_dn1 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_dn1 * var_tanh4) + (var_tanh3 * var_tanh4_dn1)))), ((var_cgd0_t_dn2 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_dn2 * var_tanh4) + (var_tanh3 * var_tanh4_dn2)))), ((var_cgd0_t_dn3 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_dn3 * var_tanh4) + (var_tanh3 * var_tanh4_dn3)))), ((var_cgd0_t_dn4 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_dn4 * var_tanh4) + (var_tanh3 * var_tanh4_dn4)))), ((var_cgd0_t_dn5 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_dn5 * var_tanh4) + (var_tanh3 * var_tanh4_dn5)))), ((var_cgd0_t_dn6 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_dn6 * var_tanh4) + (var_tanh3 * var_tanh4_dn6)))), ((var_cgd0_t_dn7 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_dn7 * var_tanh4) + (var_tanh3 * var_tanh4_dn7)))), ((var_cgd0_t_dn8 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_dn8 * var_tanh4) + (var_tanh3 * var_tanh4_dn8)))), ((var_cgd0_t_dn9 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_dn9 * var_tanh4) + (var_tanh3 * var_tanh4_dn9)))), ((var_cgd0_t_dn10 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_dn10 * var_tanh4) + (var_tanh3 * var_tanh4_dn10)))), ((var_cgd0_t_dn11 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_dn11 * var_tanh4) + (var_tanh3 * var_tanh4_dn11)))), ((var_cgd0_t_dn12 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_dn12 * var_tanh4) + (var_tanh3 * var_tanh4_dn12)))), ((var_cgd0_t_dn13 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_dn13 * var_tanh4) + (var_tanh3 * var_tanh4_dn13)))), ((var_cgd0_t_dn14 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_dn14 * var_tanh4) + (var_tanh3 * var_tanh4_dn14)))), ((var_cgd0_t_dn15 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_dn15 * var_tanh4) + (var_tanh3 * var_tanh4_dn15)))), ((var_cgd0_t_db0 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_db0 * var_tanh4) + (var_tanh3 * var_tanh4_db0)))), ((var_cgd0_t_db1 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_db1 * var_tanh4) + (var_tanh3 * var_tanh4_db1)))), ((var_cgd0_t_db2 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_db2 * var_tanh4) + (var_tanh3 * var_tanh4_db2)))), ((var_cgd0_t_db3 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_db3 * var_tanh4) + (var_tanh3 * var_tanh4_db3)))), ((var_cgd0_t_db4 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_db4 * var_tanh4) + (var_tanh3 * var_tanh4_db4)))), ((var_cgd0_t_db5 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_db5 * var_tanh4) + (var_tanh3 * var_tanh4_db5)))), ((var_cgd0_t_db6 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_db6 * var_tanh4) + (var_tanh3 * var_tanh4_db6)))), ((var_cgd0_t_db7 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_db7 * var_tanh4) + (var_tanh3 * var_tanh4_db7)))), ((var_cgd0_t_db8 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_db8 * var_tanh4) + (var_tanh3 * var_tanh4_db8)))), ((var_cgd0_t_db9 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_db9 * var_tanh4) + (var_tanh3 * var_tanh4_db9)))), ((var_cgd0_t_db10 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_db10 * var_tanh4) + (var_tanh3 * var_tanh4_db10)))), ((var_cgd0_t_db11 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_db11 * var_tanh4) + (var_tanh3 * var_tanh4_db11)))), ((var_cgd0_t_db12 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_db12 * var_tanh4) + (var_tanh3 * var_tanh4_db12)))), ((var_cgd0_t_db13 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_db13 * var_tanh4) + (var_tanh3 * var_tanh4_db13)))), ((var_cgd0_t_db14 * assign1420_e1809) + (var_cgd0_t * ((var_tanh3_db14 * var_tanh4) + (var_tanh3 * var_tanh4_db14)))),)
    } else {
        (var_cgd, var_cgd_dn0, var_cgd_dn1, var_cgd_dn2, var_cgd_dn3, var_cgd_dn4, var_cgd_dn5, var_cgd_dn6, var_cgd_dn7, var_cgd_dn8, var_cgd_dn9, var_cgd_dn10, var_cgd_dn11, var_cgd_dn12, var_cgd_dn13, var_cgd_dn14, var_cgd_dn15, var_cgd_db0, var_cgd_db1, var_cgd_db2, var_cgd_db3, var_cgd_db4, var_cgd_db5, var_cgd_db6, var_cgd_db7, var_cgd_db8, var_cgd_db9, var_cgd_db10, var_cgd_db11, var_cgd_db12, var_cgd_db13, var_cgd_db14,)
    }
};
        var_cgd = assign1420_e1813;
        var_cgd_dn0 = assign1420_e1813_d_n0;
        var_cgd_dn1 = assign1420_e1813_d_n1;
        var_cgd_dn2 = assign1420_e1813_d_n2;
        var_cgd_dn3 = assign1420_e1813_d_n3;
        var_cgd_dn4 = assign1420_e1813_d_n4;
        var_cgd_dn5 = assign1420_e1813_d_n5;
        var_cgd_dn6 = assign1420_e1813_d_n6;
        var_cgd_dn7 = assign1420_e1813_d_n7;
        var_cgd_dn8 = assign1420_e1813_d_n8;
        var_cgd_dn9 = assign1420_e1813_d_n9;
        var_cgd_dn10 = assign1420_e1813_d_n10;
        var_cgd_dn11 = assign1420_e1813_d_n11;
        var_cgd_dn12 = assign1420_e1813_d_n12;
        var_cgd_dn13 = assign1420_e1813_d_n13;
        var_cgd_dn14 = assign1420_e1813_d_n14;
        var_cgd_dn15 = assign1420_e1813_d_n15;
        var_cgd_db0 = assign1420_e1813_d_b0;
        var_cgd_db1 = assign1420_e1813_d_b1;
        var_cgd_db2 = assign1420_e1813_d_b2;
        var_cgd_db3 = assign1420_e1813_d_b3;
        var_cgd_db4 = assign1420_e1813_d_b4;
        var_cgd_db5 = assign1420_e1813_d_b5;
        var_cgd_db6 = assign1420_e1813_d_b6;
        var_cgd_db7 = assign1420_e1813_d_b7;
        var_cgd_db8 = assign1420_e1813_d_b8;
        var_cgd_db9 = assign1420_e1813_d_b9;
        var_cgd_db10 = assign1420_e1813_d_b10;
        var_cgd_db11 = assign1420_e1813_d_b11;
        var_cgd_db12 = assign1420_e1813_d_b12;
        var_cgd_db13 = assign1420_e1813_d_b13;
        var_cgd_db14 = assign1420_e1813_d_b14;

        let (assign1430_e1824, assign1430_e1824_d_n0, assign1430_e1824_d_n1, assign1430_e1824_d_n2, assign1430_e1824_d_n3, assign1430_e1824_d_n4, assign1430_e1824_d_n5, assign1430_e1824_d_n6, assign1430_e1824_d_n7, assign1430_e1824_d_n8, assign1430_e1824_d_n9, assign1430_e1824_d_n10, assign1430_e1824_d_n11, assign1430_e1824_d_n12, assign1430_e1824_d_n13, assign1430_e1824_d_n14, assign1430_e1824_d_n15, assign1430_e1824_d_b0, assign1430_e1824_d_b1, assign1430_e1824_d_b2, assign1430_e1824_d_b3, assign1430_e1824_d_b4, assign1430_e1824_d_b5, assign1430_e1824_d_b6, assign1430_e1824_d_b7, assign1430_e1824_d_b8, assign1430_e1824_d_b9, assign1430_e1824_d_b10, assign1430_e1824_d_b11, assign1430_e1824_d_b12, assign1430_e1824_d_b13, assign1430_e1824_d_b14,) = {
    if ((var_guard15 != 0.0) && (!((var_guard13 != 0.0) || (var_guard14 != 0.0)))) {
        let assign1430_e1822: f64 = (var_tanh2 - p.p37);
        (assign1430_e1822, var_tanh2_dn0, var_tanh2_dn1, var_tanh2_dn2, var_tanh2_dn3, var_tanh2_dn4, var_tanh2_dn5, var_tanh2_dn6, var_tanh2_dn7, var_tanh2_dn8, var_tanh2_dn9, var_tanh2_dn10, var_tanh2_dn11, var_tanh2_dn12, var_tanh2_dn13, var_tanh2_dn14, var_tanh2_dn15, var_tanh2_db0, var_tanh2_db1, var_tanh2_db2, var_tanh2_db3, var_tanh2_db4, var_tanh2_db5, var_tanh2_db6, var_tanh2_db7, var_tanh2_db8, var_tanh2_db9, var_tanh2_db10, var_tanh2_db11, var_tanh2_db12, var_tanh2_db13, var_tanh2_db14,)
    } else {
        (var_tanh2, var_tanh2_dn0, var_tanh2_dn1, var_tanh2_dn2, var_tanh2_dn3, var_tanh2_dn4, var_tanh2_dn5, var_tanh2_dn6, var_tanh2_dn7, var_tanh2_dn8, var_tanh2_dn9, var_tanh2_dn10, var_tanh2_dn11, var_tanh2_dn12, var_tanh2_dn13, var_tanh2_dn14, var_tanh2_dn15, var_tanh2_db0, var_tanh2_db1, var_tanh2_db2, var_tanh2_db3, var_tanh2_db4, var_tanh2_db5, var_tanh2_db6, var_tanh2_db7, var_tanh2_db8, var_tanh2_db9, var_tanh2_db10, var_tanh2_db11, var_tanh2_db12, var_tanh2_db13, var_tanh2_db14,)
    }
};
        var_tanh2 = assign1430_e1824;
        var_tanh2_dn0 = assign1430_e1824_d_n0;
        var_tanh2_dn1 = assign1430_e1824_d_n1;
        var_tanh2_dn2 = assign1430_e1824_d_n2;
        var_tanh2_dn3 = assign1430_e1824_d_n3;
        var_tanh2_dn4 = assign1430_e1824_d_n4;
        var_tanh2_dn5 = assign1430_e1824_d_n5;
        var_tanh2_dn6 = assign1430_e1824_d_n6;
        var_tanh2_dn7 = assign1430_e1824_d_n7;
        var_tanh2_dn8 = assign1430_e1824_d_n8;
        var_tanh2_dn9 = assign1430_e1824_d_n9;
        var_tanh2_dn10 = assign1430_e1824_d_n10;
        var_tanh2_dn11 = assign1430_e1824_d_n11;
        var_tanh2_dn12 = assign1430_e1824_d_n12;
        var_tanh2_dn13 = assign1430_e1824_d_n13;
        var_tanh2_dn14 = assign1430_e1824_d_n14;
        var_tanh2_dn15 = assign1430_e1824_d_n15;
        var_tanh2_db0 = assign1430_e1824_d_b0;
        var_tanh2_db1 = assign1430_e1824_d_b1;
        var_tanh2_db2 = assign1430_e1824_d_b2;
        var_tanh2_db3 = assign1430_e1824_d_b3;
        var_tanh2_db4 = assign1430_e1824_d_b4;
        var_tanh2_db5 = assign1430_e1824_d_b5;
        var_tanh2_db6 = assign1430_e1824_d_b6;
        var_tanh2_db7 = assign1430_e1824_d_b7;
        var_tanh2_db8 = assign1430_e1824_d_b8;
        var_tanh2_db9 = assign1430_e1824_d_b9;
        var_tanh2_db10 = assign1430_e1824_d_b10;
        var_tanh2_db11 = assign1430_e1824_d_b11;
        var_tanh2_db12 = assign1430_e1824_d_b12;
        var_tanh2_db13 = assign1430_e1824_d_b13;
        var_tanh2_db14 = assign1430_e1824_d_b14;

        let (assign1440_e1838, assign1440_e1838_d_n0, assign1440_e1838_d_n1, assign1440_e1838_d_n2, assign1440_e1838_d_n3, assign1440_e1838_d_n4, assign1440_e1838_d_n5, assign1440_e1838_d_n6, assign1440_e1838_d_n7, assign1440_e1838_d_n8, assign1440_e1838_d_n9, assign1440_e1838_d_n10, assign1440_e1838_d_n11, assign1440_e1838_d_n12, assign1440_e1838_d_n13, assign1440_e1838_d_n14, assign1440_e1838_d_n15, assign1440_e1838_d_b0, assign1440_e1838_d_b1, assign1440_e1838_d_b2, assign1440_e1838_d_b3, assign1440_e1838_d_b4, assign1440_e1838_d_b5, assign1440_e1838_d_b6, assign1440_e1838_d_b7, assign1440_e1838_d_b8, assign1440_e1838_d_b9, assign1440_e1838_d_b10, assign1440_e1838_d_b11, assign1440_e1838_d_b12, assign1440_e1838_d_b13, assign1440_e1838_d_b14,) = {
    if ((var_guard15 != 0.0) && (!((var_guard13 != 0.0) || (var_guard14 != 0.0)))) {
        let assign1440_e1834: f64 = (p.p37 * var_vds);
        let assign1440_e1835: f64 = (var_p10_t + assign1440_e1834);
        let assign1440_e1836: f64 = (assign1440_e1835).cosh();
        (assign1440_e1836, ((assign1440_e1835).sinh() * (var_p10_t_dn0 + (p.p37 * var_vds_dn0))), ((assign1440_e1835).sinh() * (var_p10_t_dn1 + (p.p37 * var_vds_dn1))), ((assign1440_e1835).sinh() * (var_p10_t_dn2 + (p.p37 * var_vds_dn2))), ((assign1440_e1835).sinh() * (var_p10_t_dn3 + (p.p37 * var_vds_dn3))), ((assign1440_e1835).sinh() * (var_p10_t_dn4 + (p.p37 * var_vds_dn4))), ((assign1440_e1835).sinh() * (var_p10_t_dn5 + (p.p37 * var_vds_dn5))), ((assign1440_e1835).sinh() * (var_p10_t_dn6 + (p.p37 * var_vds_dn6))), ((assign1440_e1835).sinh() * (var_p10_t_dn7 + (p.p37 * var_vds_dn7))), ((assign1440_e1835).sinh() * (var_p10_t_dn8 + (p.p37 * var_vds_dn8))), ((assign1440_e1835).sinh() * (var_p10_t_dn9 + (p.p37 * var_vds_dn9))), ((assign1440_e1835).sinh() * (var_p10_t_dn10 + (p.p37 * var_vds_dn10))), ((assign1440_e1835).sinh() * (var_p10_t_dn11 + (p.p37 * var_vds_dn11))), ((assign1440_e1835).sinh() * (var_p10_t_dn12 + (p.p37 * var_vds_dn12))), ((assign1440_e1835).sinh() * (var_p10_t_dn13 + (p.p37 * var_vds_dn13))), ((assign1440_e1835).sinh() * (var_p10_t_dn14 + (p.p37 * var_vds_dn14))), ((assign1440_e1835).sinh() * (var_p10_t_dn15 + (p.p37 * var_vds_dn15))), ((assign1440_e1835).sinh() * (var_p10_t_db0 + (p.p37 * var_vds_db0))), ((assign1440_e1835).sinh() * (var_p10_t_db1 + (p.p37 * var_vds_db1))), ((assign1440_e1835).sinh() * (var_p10_t_db2 + (p.p37 * var_vds_db2))), ((assign1440_e1835).sinh() * (var_p10_t_db3 + (p.p37 * var_vds_db3))), ((assign1440_e1835).sinh() * (var_p10_t_db4 + (p.p37 * var_vds_db4))), ((assign1440_e1835).sinh() * (var_p10_t_db5 + (p.p37 * var_vds_db5))), ((assign1440_e1835).sinh() * (var_p10_t_db6 + (p.p37 * var_vds_db6))), ((assign1440_e1835).sinh() * (var_p10_t_db7 + (p.p37 * var_vds_db7))), ((assign1440_e1835).sinh() * (var_p10_t_db8 + (p.p37 * var_vds_db8))), ((assign1440_e1835).sinh() * (var_p10_t_db9 + (p.p37 * var_vds_db9))), ((assign1440_e1835).sinh() * (var_p10_t_db10 + (p.p37 * var_vds_db10))), ((assign1440_e1835).sinh() * (var_p10_t_db11 + (p.p37 * var_vds_db11))), ((assign1440_e1835).sinh() * (var_p10_t_db12 + (p.p37 * var_vds_db12))), ((assign1440_e1835).sinh() * (var_p10_t_db13 + (p.p37 * var_vds_db13))), ((assign1440_e1835).sinh() * (var_p10_t_db14 + (p.p37 * var_vds_db14))),)
    } else {
        (var_cosh0, var_cosh0_dn0, var_cosh0_dn1, var_cosh0_dn2, var_cosh0_dn3, var_cosh0_dn4, var_cosh0_dn5, var_cosh0_dn6, var_cosh0_dn7, var_cosh0_dn8, var_cosh0_dn9, var_cosh0_dn10, var_cosh0_dn11, var_cosh0_dn12, var_cosh0_dn13, var_cosh0_dn14, var_cosh0_dn15, var_cosh0_db0, var_cosh0_db1, var_cosh0_db2, var_cosh0_db3, var_cosh0_db4, var_cosh0_db5, var_cosh0_db6, var_cosh0_db7, var_cosh0_db8, var_cosh0_db9, var_cosh0_db10, var_cosh0_db11, var_cosh0_db12, var_cosh0_db13, var_cosh0_db14,)
    }
};
        var_cosh0 = assign1440_e1838;
        var_cosh0_dn0 = assign1440_e1838_d_n0;
        var_cosh0_dn1 = assign1440_e1838_d_n1;
        var_cosh0_dn2 = assign1440_e1838_d_n2;
        var_cosh0_dn3 = assign1440_e1838_d_n3;
        var_cosh0_dn4 = assign1440_e1838_d_n4;
        var_cosh0_dn5 = assign1440_e1838_d_n5;
        var_cosh0_dn6 = assign1440_e1838_d_n6;
        var_cosh0_dn7 = assign1440_e1838_d_n7;
        var_cosh0_dn8 = assign1440_e1838_d_n8;
        var_cosh0_dn9 = assign1440_e1838_d_n9;
        var_cosh0_dn10 = assign1440_e1838_d_n10;
        var_cosh0_dn11 = assign1440_e1838_d_n11;
        var_cosh0_dn12 = assign1440_e1838_d_n12;
        var_cosh0_dn13 = assign1440_e1838_d_n13;
        var_cosh0_dn14 = assign1440_e1838_d_n14;
        var_cosh0_dn15 = assign1440_e1838_d_n15;
        var_cosh0_db0 = assign1440_e1838_d_b0;
        var_cosh0_db1 = assign1440_e1838_d_b1;
        var_cosh0_db2 = assign1440_e1838_d_b2;
        var_cosh0_db3 = assign1440_e1838_d_b3;
        var_cosh0_db4 = assign1440_e1838_d_b4;
        var_cosh0_db5 = assign1440_e1838_d_b5;
        var_cosh0_db6 = assign1440_e1838_d_b6;
        var_cosh0_db7 = assign1440_e1838_d_b7;
        var_cosh0_db8 = assign1440_e1838_d_b8;
        var_cosh0_db9 = assign1440_e1838_d_b9;
        var_cosh0_db10 = assign1440_e1838_d_b10;
        var_cosh0_db11 = assign1440_e1838_d_b11;
        var_cosh0_db12 = assign1440_e1838_d_b12;
        var_cosh0_db13 = assign1440_e1838_d_b13;
        var_cosh0_db14 = assign1440_e1838_d_b14;

        let (assign1450_e1848, assign1450_e1848_d_n0, assign1450_e1848_d_n1, assign1450_e1848_d_n2, assign1450_e1848_d_n3, assign1450_e1848_d_n4, assign1450_e1848_d_n5, assign1450_e1848_d_n6, assign1450_e1848_d_n7, assign1450_e1848_d_n8, assign1450_e1848_d_n9, assign1450_e1848_d_n10, assign1450_e1848_d_n11, assign1450_e1848_d_n12, assign1450_e1848_d_n13, assign1450_e1848_d_n14, assign1450_e1848_d_n15, assign1450_e1848_d_b0, assign1450_e1848_d_b1, assign1450_e1848_d_b2, assign1450_e1848_d_b3, assign1450_e1848_d_b4, assign1450_e1848_d_b5, assign1450_e1848_d_b6, assign1450_e1848_d_b7, assign1450_e1848_d_b8, assign1450_e1848_d_b9, assign1450_e1848_d_b10, assign1450_e1848_d_b11, assign1450_e1848_d_b12, assign1450_e1848_d_b13, assign1450_e1848_d_b14,) = {
    if ((var_guard15 != 0.0) && (!((var_guard13 != 0.0) || (var_guard14 != 0.0)))) {
        let assign1450_e1846: f64 = (var_cosh0).ln();
        (assign1450_e1846, (var_cosh0_dn0 / var_cosh0), (var_cosh0_dn1 / var_cosh0), (var_cosh0_dn2 / var_cosh0), (var_cosh0_dn3 / var_cosh0), (var_cosh0_dn4 / var_cosh0), (var_cosh0_dn5 / var_cosh0), (var_cosh0_dn6 / var_cosh0), (var_cosh0_dn7 / var_cosh0), (var_cosh0_dn8 / var_cosh0), (var_cosh0_dn9 / var_cosh0), (var_cosh0_dn10 / var_cosh0), (var_cosh0_dn11 / var_cosh0), (var_cosh0_dn12 / var_cosh0), (var_cosh0_dn13 / var_cosh0), (var_cosh0_dn14 / var_cosh0), (var_cosh0_dn15 / var_cosh0), (var_cosh0_db0 / var_cosh0), (var_cosh0_db1 / var_cosh0), (var_cosh0_db2 / var_cosh0), (var_cosh0_db3 / var_cosh0), (var_cosh0_db4 / var_cosh0), (var_cosh0_db5 / var_cosh0), (var_cosh0_db6 / var_cosh0), (var_cosh0_db7 / var_cosh0), (var_cosh0_db8 / var_cosh0), (var_cosh0_db9 / var_cosh0), (var_cosh0_db10 / var_cosh0), (var_cosh0_db11 / var_cosh0), (var_cosh0_db12 / var_cosh0), (var_cosh0_db13 / var_cosh0), (var_cosh0_db14 / var_cosh0),)
    } else {
        (var_lc10, var_lc10_dn0, var_lc10_dn1, var_lc10_dn2, var_lc10_dn3, var_lc10_dn4, var_lc10_dn5, var_lc10_dn6, var_lc10_dn7, var_lc10_dn8, var_lc10_dn9, var_lc10_dn10, var_lc10_dn11, var_lc10_dn12, var_lc10_dn13, var_lc10_dn14, var_lc10_dn15, var_lc10_db0, var_lc10_db1, var_lc10_db2, var_lc10_db3, var_lc10_db4, var_lc10_db5, var_lc10_db6, var_lc10_db7, var_lc10_db8, var_lc10_db9, var_lc10_db10, var_lc10_db11, var_lc10_db12, var_lc10_db13, var_lc10_db14,)
    }
};
        var_lc10 = assign1450_e1848;
        var_lc10_dn0 = assign1450_e1848_d_n0;
        var_lc10_dn1 = assign1450_e1848_d_n1;
        var_lc10_dn2 = assign1450_e1848_d_n2;
        var_lc10_dn3 = assign1450_e1848_d_n3;
        var_lc10_dn4 = assign1450_e1848_d_n4;
        var_lc10_dn5 = assign1450_e1848_d_n5;
        var_lc10_dn6 = assign1450_e1848_d_n6;
        var_lc10_dn7 = assign1450_e1848_d_n7;
        var_lc10_dn8 = assign1450_e1848_d_n8;
        var_lc10_dn9 = assign1450_e1848_d_n9;
        var_lc10_dn10 = assign1450_e1848_d_n10;
        var_lc10_dn11 = assign1450_e1848_d_n11;
        var_lc10_dn12 = assign1450_e1848_d_n12;
        var_lc10_dn13 = assign1450_e1848_d_n13;
        var_lc10_dn14 = assign1450_e1848_d_n14;
        var_lc10_dn15 = assign1450_e1848_d_n15;
        var_lc10_db0 = assign1450_e1848_d_b0;
        var_lc10_db1 = assign1450_e1848_d_b1;
        var_lc10_db2 = assign1450_e1848_d_b2;
        var_lc10_db3 = assign1450_e1848_d_b3;
        var_lc10_db4 = assign1450_e1848_d_b4;
        var_lc10_db5 = assign1450_e1848_d_b5;
        var_lc10_db6 = assign1450_e1848_d_b6;
        var_lc10_db7 = assign1450_e1848_d_b7;
        var_lc10_db8 = assign1450_e1848_d_b8;
        var_lc10_db9 = assign1450_e1848_d_b9;
        var_lc10_db10 = assign1450_e1848_d_b10;
        var_lc10_db11 = assign1450_e1848_d_b11;
        var_lc10_db12 = assign1450_e1848_d_b12;
        var_lc10_db13 = assign1450_e1848_d_b13;
        var_lc10_db14 = assign1450_e1848_d_b14;

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


        *var_cgd_slot = var_cgd;
        *var_cgd_db0_slot = var_cgd_db0;
        *var_cgd_db1_slot = var_cgd_db1;
        *var_cgd_db10_slot = var_cgd_db10;
        *var_cgd_db11_slot = var_cgd_db11;
        *var_cgd_db12_slot = var_cgd_db12;
        *var_cgd_db13_slot = var_cgd_db13;
        *var_cgd_db14_slot = var_cgd_db14;
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
        *var_cosh0_slot = var_cosh0;
        *var_cosh0_db0_slot = var_cosh0_db0;
        *var_cosh0_db1_slot = var_cosh0_db1;
        *var_cosh0_db10_slot = var_cosh0_db10;
        *var_cosh0_db11_slot = var_cosh0_db11;
        *var_cosh0_db12_slot = var_cosh0_db12;
        *var_cosh0_db13_slot = var_cosh0_db13;
        *var_cosh0_db14_slot = var_cosh0_db14;
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
        *var_cosh1_slot = var_cosh1;
        *var_cosh1_db0_slot = var_cosh1_db0;
        *var_cosh1_db1_slot = var_cosh1_db1;
        *var_cosh1_db10_slot = var_cosh1_db10;
        *var_cosh1_db11_slot = var_cosh1_db11;
        *var_cosh1_db12_slot = var_cosh1_db12;
        *var_cosh1_db13_slot = var_cosh1_db13;
        *var_cosh1_db14_slot = var_cosh1_db14;
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
        *var_lc1_slot = var_lc1;
        *var_lc10_slot = var_lc10;
        *var_lc10_db0_slot = var_lc10_db0;
        *var_lc10_db1_slot = var_lc10_db1;
        *var_lc10_db10_slot = var_lc10_db10;
        *var_lc10_db11_slot = var_lc10_db11;
        *var_lc10_db12_slot = var_lc10_db12;
        *var_lc10_db13_slot = var_lc10_db13;
        *var_lc10_db14_slot = var_lc10_db14;
        *var_lc10_db2_slot = var_lc10_db2;
        *var_lc10_db3_slot = var_lc10_db3;
        *var_lc10_db4_slot = var_lc10_db4;
        *var_lc10_db5_slot = var_lc10_db5;
        *var_lc10_db6_slot = var_lc10_db6;
        *var_lc10_db7_slot = var_lc10_db7;
        *var_lc10_db8_slot = var_lc10_db8;
        *var_lc10_db9_slot = var_lc10_db9;
        *var_lc10_dn0_slot = var_lc10_dn0;
        *var_lc10_dn1_slot = var_lc10_dn1;
        *var_lc10_dn10_slot = var_lc10_dn10;
        *var_lc10_dn11_slot = var_lc10_dn11;
        *var_lc10_dn12_slot = var_lc10_dn12;
        *var_lc10_dn13_slot = var_lc10_dn13;
        *var_lc10_dn14_slot = var_lc10_dn14;
        *var_lc10_dn15_slot = var_lc10_dn15;
        *var_lc10_dn2_slot = var_lc10_dn2;
        *var_lc10_dn3_slot = var_lc10_dn3;
        *var_lc10_dn4_slot = var_lc10_dn4;
        *var_lc10_dn5_slot = var_lc10_dn5;
        *var_lc10_dn6_slot = var_lc10_dn6;
        *var_lc10_dn7_slot = var_lc10_dn7;
        *var_lc10_dn8_slot = var_lc10_dn8;
        *var_lc10_dn9_slot = var_lc10_dn9;
        *var_lc1_db0_slot = var_lc1_db0;
        *var_lc1_db1_slot = var_lc1_db1;
        *var_lc1_db10_slot = var_lc1_db10;
        *var_lc1_db11_slot = var_lc1_db11;
        *var_lc1_db12_slot = var_lc1_db12;
        *var_lc1_db13_slot = var_lc1_db13;
        *var_lc1_db14_slot = var_lc1_db14;
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
        *var_lc4_slot = var_lc4;
        *var_lc40_slot = var_lc40;
        *var_lc40_db0_slot = var_lc40_db0;
        *var_lc40_db1_slot = var_lc40_db1;
        *var_lc40_db10_slot = var_lc40_db10;
        *var_lc40_db11_slot = var_lc40_db11;
        *var_lc40_db12_slot = var_lc40_db12;
        *var_lc40_db13_slot = var_lc40_db13;
        *var_lc40_db14_slot = var_lc40_db14;
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
        *var_lc4_db0_slot = var_lc4_db0;
        *var_lc4_db1_slot = var_lc4_db1;
        *var_lc4_db10_slot = var_lc4_db10;
        *var_lc4_db11_slot = var_lc4_db11;
        *var_lc4_db12_slot = var_lc4_db12;
        *var_lc4_db13_slot = var_lc4_db13;
        *var_lc4_db14_slot = var_lc4_db14;
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
        *var_qgs_slot = var_qgs;
        *var_qgs0_slot = var_qgs0;
        *var_qgs0_db0_slot = var_qgs0_db0;
        *var_qgs0_db1_slot = var_qgs0_db1;
        *var_qgs0_db10_slot = var_qgs0_db10;
        *var_qgs0_db11_slot = var_qgs0_db11;
        *var_qgs0_db12_slot = var_qgs0_db12;
        *var_qgs0_db13_slot = var_qgs0_db13;
        *var_qgs0_db14_slot = var_qgs0_db14;
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
        *var_qgs_db0_slot = var_qgs_db0;
        *var_qgs_db1_slot = var_qgs_db1;
        *var_qgs_db10_slot = var_qgs_db10;
        *var_qgs_db11_slot = var_qgs_db11;
        *var_qgs_db12_slot = var_qgs_db12;
        *var_qgs_db13_slot = var_qgs_db13;
        *var_qgs_db14_slot = var_qgs_db14;
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
        *var_tanh2_slot = var_tanh2;
        *var_tanh2_db0_slot = var_tanh2_db0;
        *var_tanh2_db1_slot = var_tanh2_db1;
        *var_tanh2_db10_slot = var_tanh2_db10;
        *var_tanh2_db11_slot = var_tanh2_db11;
        *var_tanh2_db12_slot = var_tanh2_db12;
        *var_tanh2_db13_slot = var_tanh2_db13;
        *var_tanh2_db14_slot = var_tanh2_db14;
        *var_tanh2_db2_slot = var_tanh2_db2;
        *var_tanh2_db3_slot = var_tanh2_db3;
        *var_tanh2_db4_slot = var_tanh2_db4;
        *var_tanh2_db5_slot = var_tanh2_db5;
        *var_tanh2_db6_slot = var_tanh2_db6;
        *var_tanh2_db7_slot = var_tanh2_db7;
        *var_tanh2_db8_slot = var_tanh2_db8;
        *var_tanh2_db9_slot = var_tanh2_db9;
        *var_tanh2_dn0_slot = var_tanh2_dn0;
        *var_tanh2_dn1_slot = var_tanh2_dn1;
        *var_tanh2_dn10_slot = var_tanh2_dn10;
        *var_tanh2_dn11_slot = var_tanh2_dn11;
        *var_tanh2_dn12_slot = var_tanh2_dn12;
        *var_tanh2_dn13_slot = var_tanh2_dn13;
        *var_tanh2_dn14_slot = var_tanh2_dn14;
        *var_tanh2_dn15_slot = var_tanh2_dn15;
        *var_tanh2_dn2_slot = var_tanh2_dn2;
        *var_tanh2_dn3_slot = var_tanh2_dn3;
        *var_tanh2_dn4_slot = var_tanh2_dn4;
        *var_tanh2_dn5_slot = var_tanh2_dn5;
        *var_tanh2_dn6_slot = var_tanh2_dn6;
        *var_tanh2_dn7_slot = var_tanh2_dn7;
        *var_tanh2_dn8_slot = var_tanh2_dn8;
        *var_tanh2_dn9_slot = var_tanh2_dn9;
    }

    pub(super) fn stamp_transient_block_7(
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
        var_lc4: f64,
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
        var_lc4_db0: f64,
        var_lc4_db1: f64,
        var_lc4_db10: f64,
        var_lc4_db11: f64,
        var_lc4_db12: f64,
        var_lc4_db13: f64,
        var_lc4_db14: f64,
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
        var_cgs_slot: &mut f64,
        var_cgs_db0_slot: &mut f64,
        var_cgs_db1_slot: &mut f64,
        var_cgs_db10_slot: &mut f64,
        var_cgs_db11_slot: &mut f64,
        var_cgs_db12_slot: &mut f64,
        var_cgs_db13_slot: &mut f64,
        var_cgs_db14_slot: &mut f64,
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
        var_ci_slot: &mut f64,
        var_ci_db0_slot: &mut f64,
        var_ci_db1_slot: &mut f64,
        var_ci_db10_slot: &mut f64,
        var_ci_db11_slot: &mut f64,
        var_ci_db12_slot: &mut f64,
        var_ci_db13_slot: &mut f64,
        var_ci_db14_slot: &mut f64,
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
        var_guard16_slot: &mut f64,
        var_guard21_slot: &mut f64,
        var_guard22_slot: &mut f64,
        var_guard23_slot: &mut f64,
        var_guard24_slot: &mut f64,
        var_guard25_slot: &mut f64,
        var_guard26_slot: &mut f64,
        var_guard27_slot: &mut f64,
        var_guard43_slot: &mut f64,
        var_k_slot: &mut f64,
        var_k_db0_slot: &mut f64,
        var_k_db1_slot: &mut f64,
        var_k_db10_slot: &mut f64,
        var_k_db11_slot: &mut f64,
        var_k_db12_slot: &mut f64,
        var_k_db13_slot: &mut f64,
        var_k_db14_slot: &mut f64,
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
        var_qgd_slot: &mut f64,
        var_qgd0_slot: &mut f64,
        var_qgd0_db0_slot: &mut f64,
        var_qgd0_db1_slot: &mut f64,
        var_qgd0_db10_slot: &mut f64,
        var_qgd0_db11_slot: &mut f64,
        var_qgd0_db12_slot: &mut f64,
        var_qgd0_db13_slot: &mut f64,
        var_qgd0_db14_slot: &mut f64,
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
        var_qgd_db0_slot: &mut f64,
        var_qgd_db1_slot: &mut f64,
        var_qgd_db10_slot: &mut f64,
        var_qgd_db11_slot: &mut f64,
        var_qgd_db12_slot: &mut f64,
        var_qgd_db13_slot: &mut f64,
        var_qgd_db14_slot: &mut f64,
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
    ) {
        let mut var_cgd: f64 = *var_cgd_slot;
        let mut var_cgd_db0: f64 = *var_cgd_db0_slot;
        let mut var_cgd_db1: f64 = *var_cgd_db1_slot;
        let mut var_cgd_db10: f64 = *var_cgd_db10_slot;
        let mut var_cgd_db11: f64 = *var_cgd_db11_slot;
        let mut var_cgd_db12: f64 = *var_cgd_db12_slot;
        let mut var_cgd_db13: f64 = *var_cgd_db13_slot;
        let mut var_cgd_db14: f64 = *var_cgd_db14_slot;
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
        let mut var_cgs: f64 = *var_cgs_slot;
        let mut var_cgs_db0: f64 = *var_cgs_db0_slot;
        let mut var_cgs_db1: f64 = *var_cgs_db1_slot;
        let mut var_cgs_db10: f64 = *var_cgs_db10_slot;
        let mut var_cgs_db11: f64 = *var_cgs_db11_slot;
        let mut var_cgs_db12: f64 = *var_cgs_db12_slot;
        let mut var_cgs_db13: f64 = *var_cgs_db13_slot;
        let mut var_cgs_db14: f64 = *var_cgs_db14_slot;
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
        let mut var_ci: f64 = *var_ci_slot;
        let mut var_ci_db0: f64 = *var_ci_db0_slot;
        let mut var_ci_db1: f64 = *var_ci_db1_slot;
        let mut var_ci_db10: f64 = *var_ci_db10_slot;
        let mut var_ci_db11: f64 = *var_ci_db11_slot;
        let mut var_ci_db12: f64 = *var_ci_db12_slot;
        let mut var_ci_db13: f64 = *var_ci_db13_slot;
        let mut var_ci_db14: f64 = *var_ci_db14_slot;
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
        let mut var_guard16: f64 = *var_guard16_slot;
        let mut var_guard21: f64 = *var_guard21_slot;
        let mut var_guard22: f64 = *var_guard22_slot;
        let mut var_guard23: f64 = *var_guard23_slot;
        let mut var_guard24: f64 = *var_guard24_slot;
        let mut var_guard25: f64 = *var_guard25_slot;
        let mut var_guard26: f64 = *var_guard26_slot;
        let mut var_guard27: f64 = *var_guard27_slot;
        let mut var_guard43: f64 = *var_guard43_slot;
        let mut var_k: f64 = *var_k_slot;
        let mut var_k_db0: f64 = *var_k_db0_slot;
        let mut var_k_db1: f64 = *var_k_db1_slot;
        let mut var_k_db10: f64 = *var_k_db10_slot;
        let mut var_k_db11: f64 = *var_k_db11_slot;
        let mut var_k_db12: f64 = *var_k_db12_slot;
        let mut var_k_db13: f64 = *var_k_db13_slot;
        let mut var_k_db14: f64 = *var_k_db14_slot;
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
        let mut var_qgd: f64 = *var_qgd_slot;
        let mut var_qgd0: f64 = *var_qgd0_slot;
        let mut var_qgd0_db0: f64 = *var_qgd0_db0_slot;
        let mut var_qgd0_db1: f64 = *var_qgd0_db1_slot;
        let mut var_qgd0_db10: f64 = *var_qgd0_db10_slot;
        let mut var_qgd0_db11: f64 = *var_qgd0_db11_slot;
        let mut var_qgd0_db12: f64 = *var_qgd0_db12_slot;
        let mut var_qgd0_db13: f64 = *var_qgd0_db13_slot;
        let mut var_qgd0_db14: f64 = *var_qgd0_db14_slot;
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
        let mut var_qgd_db0: f64 = *var_qgd_db0_slot;
        let mut var_qgd_db1: f64 = *var_qgd_db1_slot;
        let mut var_qgd_db10: f64 = *var_qgd_db10_slot;
        let mut var_qgd_db11: f64 = *var_qgd_db11_slot;
        let mut var_qgd_db12: f64 = *var_qgd_db12_slot;
        let mut var_qgd_db13: f64 = *var_qgd_db13_slot;
        let mut var_qgd_db14: f64 = *var_qgd_db14_slot;
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

        let assign1580_e2025: f64 = if p.p6 == 2.0 { 1.0 } else { 0.0 };
        var_guard16 = assign1580_e2025;

        let assign1630_e2040: f64 = if p.p42 > 0.0 { 1.0 } else { 0.0 };
        var_guard21 = assign1630_e2040;

        let assign1640_e2043: f64 = if p.p50 > 0.0 { 1.0 } else { 0.0 };
        var_guard22 = assign1640_e2043;

        let assign1650_e2046: f64 = if p.p46 > 0.0 { 1.0 } else { 0.0 };
        var_guard23 = assign1650_e2046;

        let assign1660_e2053: f64 = if ((p.p43 > 0.0) || (p.p44 > 0.0)) { 1.0 } else { 0.0 };
        var_guard24 = assign1660_e2053;

        let assign1670_e2056: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        var_guard25 = assign1670_e2056;

        let assign1680_e2059: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        var_guard26 = assign1680_e2059;

        let assign1690_e2062: f64 = if p.p7 == 1.0 { 1.0 } else { 0.0 };
        var_guard27 = assign1690_e2062;

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

        let assign1860_e2271: f64 = if ((p.p1 != 0.0) && (p.p57 != 0.0)) { 1.0 } else { 0.0 };
        var_guard43 = assign1860_e2271;


        *var_cgd_slot = var_cgd;
        *var_cgd_db0_slot = var_cgd_db0;
        *var_cgd_db1_slot = var_cgd_db1;
        *var_cgd_db10_slot = var_cgd_db10;
        *var_cgd_db11_slot = var_cgd_db11;
        *var_cgd_db12_slot = var_cgd_db12;
        *var_cgd_db13_slot = var_cgd_db13;
        *var_cgd_db14_slot = var_cgd_db14;
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
        *var_cgs_slot = var_cgs;
        *var_cgs_db0_slot = var_cgs_db0;
        *var_cgs_db1_slot = var_cgs_db1;
        *var_cgs_db10_slot = var_cgs_db10;
        *var_cgs_db11_slot = var_cgs_db11;
        *var_cgs_db12_slot = var_cgs_db12;
        *var_cgs_db13_slot = var_cgs_db13;
        *var_cgs_db14_slot = var_cgs_db14;
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
        *var_ci_slot = var_ci;
        *var_ci_db0_slot = var_ci_db0;
        *var_ci_db1_slot = var_ci_db1;
        *var_ci_db10_slot = var_ci_db10;
        *var_ci_db11_slot = var_ci_db11;
        *var_ci_db12_slot = var_ci_db12;
        *var_ci_db13_slot = var_ci_db13;
        *var_ci_db14_slot = var_ci_db14;
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
        *var_guard16_slot = var_guard16;
        *var_guard21_slot = var_guard21;
        *var_guard22_slot = var_guard22;
        *var_guard23_slot = var_guard23;
        *var_guard24_slot = var_guard24;
        *var_guard25_slot = var_guard25;
        *var_guard26_slot = var_guard26;
        *var_guard27_slot = var_guard27;
        *var_guard43_slot = var_guard43;
        *var_k_slot = var_k;
        *var_k_db0_slot = var_k_db0;
        *var_k_db1_slot = var_k_db1;
        *var_k_db10_slot = var_k_db10;
        *var_k_db11_slot = var_k_db11;
        *var_k_db12_slot = var_k_db12;
        *var_k_db13_slot = var_k_db13;
        *var_k_db14_slot = var_k_db14;
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
        *var_qgd_slot = var_qgd;
        *var_qgd0_slot = var_qgd0;
        *var_qgd0_db0_slot = var_qgd0_db0;
        *var_qgd0_db1_slot = var_qgd0_db1;
        *var_qgd0_db10_slot = var_qgd0_db10;
        *var_qgd0_db11_slot = var_qgd0_db11;
        *var_qgd0_db12_slot = var_qgd0_db12;
        *var_qgd0_db13_slot = var_qgd0_db13;
        *var_qgd0_db14_slot = var_qgd0_db14;
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
        *var_qgd_db0_slot = var_qgd_db0;
        *var_qgd_db1_slot = var_qgd_db1;
        *var_qgd_db10_slot = var_qgd_db10;
        *var_qgd_db11_slot = var_qgd_db11;
        *var_qgd_db12_slot = var_qgd_db12;
        *var_qgd_db13_slot = var_qgd_db13;
        *var_qgd_db14_slot = var_qgd_db14;
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
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
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
    if s.b[97] {
        let eq7_e108_q: f64 = s.v[21];
        (s.v[21], s.dn[21][0], s.dn[21][1], s.dn[21][2], s.dn[21][3], s.dn[21][4], s.dn[21][5], s.dn[21][6], s.dn[21][7], s.dn[21][8], s.dn[21][9], s.dn[21][10], s.dn[21][11], s.dn[21][12], s.dn[21][13], s.dn[21][14], s.dn[21][15], s.db[21][0], s.db[21][1], s.db[21][2], s.db[21][3], s.db[21][4], s.db[21][5], s.db[21][6], s.db[21][7], s.db[21][8], s.db[21][9], s.db[21][10], s.db[21][11], s.db[21][12], s.db[21][13], s.db[21][14], eq7_e108_q,)
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
    if s.b[97] {
        let eq8_e113_q: f64 = s.v[20];
        (s.v[20], s.dn[20][0], s.dn[20][1], s.dn[20][2], s.dn[20][3], s.dn[20][4], s.dn[20][5], s.dn[20][6], s.dn[20][7], s.dn[20][8], s.dn[20][9], s.dn[20][10], s.dn[20][11], s.dn[20][12], s.dn[20][13], s.dn[20][14], s.dn[20][15], s.db[20][0], s.db[20][1], s.db[20][2], s.db[20][3], s.db[20][4], s.db[20][5], s.db[20][6], s.db[20][7], s.db[20][8], s.db[20][9], s.db[20][10], s.db[20][11], s.db[20][12], s.db[20][13], s.db[20][14], eq8_e113_q,)
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
    if (!s.b[97]) {
        let eq9_e120: f64 = (s.v[19] * s.v[80]);
        let eq9_e120_d_n0: f64 = ((s.dn[19][0] * s.v[80]) + (s.v[19] * s.dn[80][0]));
        let eq9_e120_d_n1: f64 = ((s.dn[19][1] * s.v[80]) + (s.v[19] * s.dn[80][1]));
        let eq9_e120_d_n2: f64 = ((s.dn[19][2] * s.v[80]) + (s.v[19] * s.dn[80][2]));
        let eq9_e120_d_n3: f64 = ((s.dn[19][3] * s.v[80]) + (s.v[19] * s.dn[80][3]));
        let eq9_e120_d_n4: f64 = ((s.dn[19][4] * s.v[80]) + (s.v[19] * s.dn[80][4]));
        let eq9_e120_d_n5: f64 = ((s.dn[19][5] * s.v[80]) + (s.v[19] * s.dn[80][5]));
        let eq9_e120_d_n6: f64 = ((s.dn[19][6] * s.v[80]) + (s.v[19] * s.dn[80][6]));
        let eq9_e120_d_n7: f64 = ((s.dn[19][7] * s.v[80]) + (s.v[19] * s.dn[80][7]));
        let eq9_e120_d_n8: f64 = ((s.dn[19][8] * s.v[80]) + (s.v[19] * s.dn[80][8]));
        let eq9_e120_d_n9: f64 = ((s.dn[19][9] * s.v[80]) + (s.v[19] * s.dn[80][9]));
        let eq9_e120_d_n10: f64 = ((s.dn[19][10] * s.v[80]) + (s.v[19] * s.dn[80][10]));
        let eq9_e120_d_n11: f64 = ((s.dn[19][11] * s.v[80]) + (s.v[19] * s.dn[80][11]));
        let eq9_e120_d_n12: f64 = ((s.dn[19][12] * s.v[80]) + (s.v[19] * s.dn[80][12]));
        let eq9_e120_d_n13: f64 = ((s.dn[19][13] * s.v[80]) + (s.v[19] * s.dn[80][13]));
        let eq9_e120_d_n14: f64 = ((s.dn[19][14] * s.v[80]) + (s.v[19] * s.dn[80][14]));
        let eq9_e120_d_n15: f64 = ((s.dn[19][15] * s.v[80]) + (s.v[19] * s.dn[80][15]));
        let eq9_e120_d_b0: f64 = ((s.db[19][0] * s.v[80]) + (s.v[19] * s.db[80][0]));
        let eq9_e120_d_b1: f64 = ((s.db[19][1] * s.v[80]) + (s.v[19] * s.db[80][1]));
        let eq9_e120_d_b2: f64 = ((s.db[19][2] * s.v[80]) + (s.v[19] * s.db[80][2]));
        let eq9_e120_d_b3: f64 = ((s.db[19][3] * s.v[80]) + (s.v[19] * s.db[80][3]));
        let eq9_e120_d_b4: f64 = ((s.db[19][4] * s.v[80]) + (s.v[19] * s.db[80][4]));
        let eq9_e120_d_b5: f64 = ((s.db[19][5] * s.v[80]) + (s.v[19] * s.db[80][5]));
        let eq9_e120_d_b6: f64 = ((s.db[19][6] * s.v[80]) + (s.v[19] * s.db[80][6]));
        let eq9_e120_d_b7: f64 = ((s.db[19][7] * s.v[80]) + (s.v[19] * s.db[80][7]));
        let eq9_e120_d_b8: f64 = ((s.db[19][8] * s.v[80]) + (s.v[19] * s.db[80][8]));
        let eq9_e120_d_b9: f64 = ((s.db[19][9] * s.v[80]) + (s.v[19] * s.db[80][9]));
        let eq9_e120_d_b10: f64 = ((s.db[19][10] * s.v[80]) + (s.v[19] * s.db[80][10]));
        let eq9_e120_d_b11: f64 = ((s.db[19][11] * s.v[80]) + (s.v[19] * s.db[80][11]));
        let eq9_e120_d_b12: f64 = ((s.db[19][12] * s.v[80]) + (s.v[19] * s.db[80][12]));
        let eq9_e120_d_b13: f64 = ((s.db[19][13] * s.v[80]) + (s.v[19] * s.db[80][13]));
        let eq9_e120_d_b14: f64 = ((s.db[19][14] * s.v[80]) + (s.v[19] * s.db[80][14]));
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
    if (!s.b[97]) {
        let eq10_e128: f64 = (s.v[18] * s.v[79]);
        let eq10_e128_d_n0: f64 = ((s.dn[18][0] * s.v[79]) + (s.v[18] * s.dn[79][0]));
        let eq10_e128_d_n1: f64 = ((s.dn[18][1] * s.v[79]) + (s.v[18] * s.dn[79][1]));
        let eq10_e128_d_n2: f64 = ((s.dn[18][2] * s.v[79]) + (s.v[18] * s.dn[79][2]));
        let eq10_e128_d_n3: f64 = ((s.dn[18][3] * s.v[79]) + (s.v[18] * s.dn[79][3]));
        let eq10_e128_d_n4: f64 = ((s.dn[18][4] * s.v[79]) + (s.v[18] * s.dn[79][4]));
        let eq10_e128_d_n5: f64 = ((s.dn[18][5] * s.v[79]) + (s.v[18] * s.dn[79][5]));
        let eq10_e128_d_n6: f64 = ((s.dn[18][6] * s.v[79]) + (s.v[18] * s.dn[79][6]));
        let eq10_e128_d_n7: f64 = ((s.dn[18][7] * s.v[79]) + (s.v[18] * s.dn[79][7]));
        let eq10_e128_d_n8: f64 = ((s.dn[18][8] * s.v[79]) + (s.v[18] * s.dn[79][8]));
        let eq10_e128_d_n9: f64 = ((s.dn[18][9] * s.v[79]) + (s.v[18] * s.dn[79][9]));
        let eq10_e128_d_n10: f64 = ((s.dn[18][10] * s.v[79]) + (s.v[18] * s.dn[79][10]));
        let eq10_e128_d_n11: f64 = ((s.dn[18][11] * s.v[79]) + (s.v[18] * s.dn[79][11]));
        let eq10_e128_d_n12: f64 = ((s.dn[18][12] * s.v[79]) + (s.v[18] * s.dn[79][12]));
        let eq10_e128_d_n13: f64 = ((s.dn[18][13] * s.v[79]) + (s.v[18] * s.dn[79][13]));
        let eq10_e128_d_n14: f64 = ((s.dn[18][14] * s.v[79]) + (s.v[18] * s.dn[79][14]));
        let eq10_e128_d_n15: f64 = ((s.dn[18][15] * s.v[79]) + (s.v[18] * s.dn[79][15]));
        let eq10_e128_d_b0: f64 = ((s.db[18][0] * s.v[79]) + (s.v[18] * s.db[79][0]));
        let eq10_e128_d_b1: f64 = ((s.db[18][1] * s.v[79]) + (s.v[18] * s.db[79][1]));
        let eq10_e128_d_b2: f64 = ((s.db[18][2] * s.v[79]) + (s.v[18] * s.db[79][2]));
        let eq10_e128_d_b3: f64 = ((s.db[18][3] * s.v[79]) + (s.v[18] * s.db[79][3]));
        let eq10_e128_d_b4: f64 = ((s.db[18][4] * s.v[79]) + (s.v[18] * s.db[79][4]));
        let eq10_e128_d_b5: f64 = ((s.db[18][5] * s.v[79]) + (s.v[18] * s.db[79][5]));
        let eq10_e128_d_b6: f64 = ((s.db[18][6] * s.v[79]) + (s.v[18] * s.db[79][6]));
        let eq10_e128_d_b7: f64 = ((s.db[18][7] * s.v[79]) + (s.v[18] * s.db[79][7]));
        let eq10_e128_d_b8: f64 = ((s.db[18][8] * s.v[79]) + (s.v[18] * s.db[79][8]));
        let eq10_e128_d_b9: f64 = ((s.db[18][9] * s.v[79]) + (s.v[18] * s.db[79][9]));
        let eq10_e128_d_b10: f64 = ((s.db[18][10] * s.v[79]) + (s.v[18] * s.db[79][10]));
        let eq10_e128_d_b11: f64 = ((s.db[18][11] * s.v[79]) + (s.v[18] * s.db[79][11]));
        let eq10_e128_d_b12: f64 = ((s.db[18][12] * s.v[79]) + (s.v[18] * s.db[79][12]));
        let eq10_e128_d_b13: f64 = ((s.db[18][13] * s.v[79]) + (s.v[18] * s.db[79][13]));
        let eq10_e128_d_b14: f64 = ((s.db[18][14] * s.v[79]) + (s.v[18] * s.db[79][14]));
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
    if s.b[102] {
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
    if ((!s.b[102]) && s.b[103]) {
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
    if s.b[105] {
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
    if ((!s.b[105]) && s.b[106]) {
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
    if ((s.b[108] && (!s.b[107])) && (p.p0 != 0.0)) {
        let eq46_e415: f64 = (-s.v[118]);
        let eq46_e417: f64 = (eq46_e415 * (nv14 - 0.0));
        let eq46_e417_d_n0: f64 = ((-s.dn[118][0]) * (nv14 - 0.0));
        let eq46_e417_d_n1: f64 = ((-s.dn[118][1]) * (nv14 - 0.0));
        let eq46_e417_d_n2: f64 = ((-s.dn[118][2]) * (nv14 - 0.0));
        let eq46_e417_d_n3: f64 = ((-s.dn[118][3]) * (nv14 - 0.0));
        let eq46_e417_d_n4: f64 = ((-s.dn[118][4]) * (nv14 - 0.0));
        let eq46_e417_d_n5: f64 = ((-s.dn[118][5]) * (nv14 - 0.0));
        let eq46_e417_d_n6: f64 = ((-s.dn[118][6]) * (nv14 - 0.0));
        let eq46_e417_d_n7: f64 = ((-s.dn[118][7]) * (nv14 - 0.0));
        let eq46_e417_d_n8: f64 = ((-s.dn[118][8]) * (nv14 - 0.0));
        let eq46_e417_d_n9: f64 = ((-s.dn[118][9]) * (nv14 - 0.0));
        let eq46_e417_d_n10: f64 = ((-s.dn[118][10]) * (nv14 - 0.0));
        let eq46_e417_d_n11: f64 = ((-s.dn[118][11]) * (nv14 - 0.0));
        let eq46_e417_d_n12: f64 = ((-s.dn[118][12]) * (nv14 - 0.0));
        let eq46_e417_d_n13: f64 = ((-s.dn[118][13]) * (nv14 - 0.0));
        let eq46_e417_d_n14: f64 = (((-s.dn[118][14]) * (nv14 - 0.0)) + eq46_e415);
        let eq46_e417_d_n15: f64 = ((-s.dn[118][15]) * (nv14 - 0.0));
        let eq46_e417_d_b0: f64 = ((-s.db[118][0]) * (nv14 - 0.0));
        let eq46_e417_d_b1: f64 = ((-s.db[118][1]) * (nv14 - 0.0));
        let eq46_e417_d_b2: f64 = ((-s.db[118][2]) * (nv14 - 0.0));
        let eq46_e417_d_b3: f64 = ((-s.db[118][3]) * (nv14 - 0.0));
        let eq46_e417_d_b4: f64 = ((-s.db[118][4]) * (nv14 - 0.0));
        let eq46_e417_d_b5: f64 = ((-s.db[118][5]) * (nv14 - 0.0));
        let eq46_e417_d_b6: f64 = ((-s.db[118][6]) * (nv14 - 0.0));
        let eq46_e417_d_b7: f64 = ((-s.db[118][7]) * (nv14 - 0.0));
        let eq46_e417_d_b8: f64 = ((-s.db[118][8]) * (nv14 - 0.0));
        let eq46_e417_d_b9: f64 = ((-s.db[118][9]) * (nv14 - 0.0));
        let eq46_e417_d_b10: f64 = ((-s.db[118][10]) * (nv14 - 0.0));
        let eq46_e417_d_b11: f64 = ((-s.db[118][11]) * (nv14 - 0.0));
        let eq46_e417_d_b12: f64 = ((-s.db[118][12]) * (nv14 - 0.0));
        let eq46_e417_d_b13: f64 = ((-s.db[118][13]) * (nv14 - 0.0));
        let eq46_e417_d_b14: f64 = ((-s.db[118][14]) * (nv14 - 0.0));
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
    if s.b[124] {
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
