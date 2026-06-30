#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_25(
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
        var_dvh__blk163: f64,
        var_dvh__blk163_dn0: f64,
        var_dvh__blk163_dn1: f64,
        var_dvh__blk163_dn10: f64,
        var_dvh__blk163_dn11: f64,
        var_dvh__blk163_dn12: f64,
        var_dvh__blk163_dn13: f64,
        var_dvh__blk163_dn2: f64,
        var_dvh__blk163_dn3: f64,
        var_dvh__blk163_dn4: f64,
        var_dvh__blk163_dn5: f64,
        var_dvh__blk163_dn6: f64,
        var_dvh__blk163_dn7: f64,
        var_dvh__blk163_dn8: f64,
        var_dvh__blk163_dn9: f64,
        var_guard183: f64,
        var_guard184: f64,
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
        var_guard185_slot: &mut f64,
        var_guard185_dn0_slot: &mut f64,
        var_guard185_dn1_slot: &mut f64,
        var_guard185_dn10_slot: &mut f64,
        var_guard185_dn11_slot: &mut f64,
        var_guard185_dn12_slot: &mut f64,
        var_guard185_dn13_slot: &mut f64,
        var_guard185_dn2_slot: &mut f64,
        var_guard185_dn3_slot: &mut f64,
        var_guard185_dn4_slot: &mut f64,
        var_guard185_dn5_slot: &mut f64,
        var_guard185_dn6_slot: &mut f64,
        var_guard185_dn7_slot: &mut f64,
        var_guard185_dn8_slot: &mut f64,
        var_guard185_dn9_slot: &mut f64,
        var_guard185_rdn0_slot: &mut f64,
        var_guard185_rdn1_slot: &mut f64,
        var_guard185_rdn10_slot: &mut f64,
        var_guard185_rdn11_slot: &mut f64,
        var_guard185_rdn12_slot: &mut f64,
        var_guard185_rdn13_slot: &mut f64,
        var_guard185_rdn2_slot: &mut f64,
        var_guard185_rdn3_slot: &mut f64,
        var_guard185_rdn4_slot: &mut f64,
        var_guard185_rdn5_slot: &mut f64,
        var_guard185_rdn6_slot: &mut f64,
        var_guard185_rdn7_slot: &mut f64,
        var_guard185_rdn8_slot: &mut f64,
        var_guard185_rdn9_slot: &mut f64,
        var_guard185_rv_slot: &mut f64,
        var_guard186_slot: &mut f64,
        var_guard186_dn0_slot: &mut f64,
        var_guard186_dn1_slot: &mut f64,
        var_guard186_dn10_slot: &mut f64,
        var_guard186_dn11_slot: &mut f64,
        var_guard186_dn12_slot: &mut f64,
        var_guard186_dn13_slot: &mut f64,
        var_guard186_dn2_slot: &mut f64,
        var_guard186_dn3_slot: &mut f64,
        var_guard186_dn4_slot: &mut f64,
        var_guard186_dn5_slot: &mut f64,
        var_guard186_dn6_slot: &mut f64,
        var_guard186_dn7_slot: &mut f64,
        var_guard186_dn8_slot: &mut f64,
        var_guard186_dn9_slot: &mut f64,
        var_guard186_rdn0_slot: &mut f64,
        var_guard186_rdn1_slot: &mut f64,
        var_guard186_rdn10_slot: &mut f64,
        var_guard186_rdn11_slot: &mut f64,
        var_guard186_rdn12_slot: &mut f64,
        var_guard186_rdn13_slot: &mut f64,
        var_guard186_rdn2_slot: &mut f64,
        var_guard186_rdn3_slot: &mut f64,
        var_guard186_rdn4_slot: &mut f64,
        var_guard186_rdn5_slot: &mut f64,
        var_guard186_rdn6_slot: &mut f64,
        var_guard186_rdn7_slot: &mut f64,
        var_guard186_rdn8_slot: &mut f64,
        var_guard186_rdn9_slot: &mut f64,
        var_guard186_rv_slot: &mut f64,
        var_pwq__blk164_slot: &mut f64,
        var_pwq__blk164_dn0_slot: &mut f64,
        var_pwq__blk164_dn1_slot: &mut f64,
        var_pwq__blk164_dn10_slot: &mut f64,
        var_pwq__blk164_dn11_slot: &mut f64,
        var_pwq__blk164_dn12_slot: &mut f64,
        var_pwq__blk164_dn13_slot: &mut f64,
        var_pwq__blk164_dn2_slot: &mut f64,
        var_pwq__blk164_dn3_slot: &mut f64,
        var_pwq__blk164_dn4_slot: &mut f64,
        var_pwq__blk164_dn5_slot: &mut f64,
        var_pwq__blk164_dn6_slot: &mut f64,
        var_pwq__blk164_dn7_slot: &mut f64,
        var_pwq__blk164_dn8_slot: &mut f64,
        var_pwq__blk164_dn9_slot: &mut f64,
        var_pwq__blk164_rdn0_slot: &mut f64,
        var_pwq__blk164_rdn1_slot: &mut f64,
        var_pwq__blk164_rdn10_slot: &mut f64,
        var_pwq__blk164_rdn11_slot: &mut f64,
        var_pwq__blk164_rdn12_slot: &mut f64,
        var_pwq__blk164_rdn13_slot: &mut f64,
        var_pwq__blk164_rdn2_slot: &mut f64,
        var_pwq__blk164_rdn3_slot: &mut f64,
        var_pwq__blk164_rdn4_slot: &mut f64,
        var_pwq__blk164_rdn5_slot: &mut f64,
        var_pwq__blk164_rdn6_slot: &mut f64,
        var_pwq__blk164_rdn7_slot: &mut f64,
        var_pwq__blk164_rdn8_slot: &mut f64,
        var_pwq__blk164_rdn9_slot: &mut f64,
        var_pwq__blk164_rv_slot: &mut f64,
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
        var_qdbep_rdn0_slot: &mut f64,
        var_qdbep_rdn1_slot: &mut f64,
        var_qdbep_rdn10_slot: &mut f64,
        var_qdbep_rdn11_slot: &mut f64,
        var_qdbep_rdn12_slot: &mut f64,
        var_qdbep_rdn13_slot: &mut f64,
        var_qdbep_rdn2_slot: &mut f64,
        var_qdbep_rdn3_slot: &mut f64,
        var_qdbep_rdn4_slot: &mut f64,
        var_qdbep_rdn5_slot: &mut f64,
        var_qdbep_rdn6_slot: &mut f64,
        var_qdbep_rdn7_slot: &mut f64,
        var_qdbep_rdn8_slot: &mut f64,
        var_qdbep_rdn9_slot: &mut f64,
        var_qdbep_rv_slot: &mut f64,
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
        var_qhi__blk166_rdn0_slot: &mut f64,
        var_qhi__blk166_rdn1_slot: &mut f64,
        var_qhi__blk166_rdn10_slot: &mut f64,
        var_qhi__blk166_rdn11_slot: &mut f64,
        var_qhi__blk166_rdn12_slot: &mut f64,
        var_qhi__blk166_rdn13_slot: &mut f64,
        var_qhi__blk166_rdn2_slot: &mut f64,
        var_qhi__blk166_rdn3_slot: &mut f64,
        var_qhi__blk166_rdn4_slot: &mut f64,
        var_qhi__blk166_rdn5_slot: &mut f64,
        var_qhi__blk166_rdn6_slot: &mut f64,
        var_qhi__blk166_rdn7_slot: &mut f64,
        var_qhi__blk166_rdn8_slot: &mut f64,
        var_qhi__blk166_rdn9_slot: &mut f64,
        var_qhi__blk166_rv_slot: &mut f64,
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
        var_qlo__blk165_rdn0_slot: &mut f64,
        var_qlo__blk165_rdn1_slot: &mut f64,
        var_qlo__blk165_rdn10_slot: &mut f64,
        var_qlo__blk165_rdn11_slot: &mut f64,
        var_qlo__blk165_rdn12_slot: &mut f64,
        var_qlo__blk165_rdn13_slot: &mut f64,
        var_qlo__blk165_rdn2_slot: &mut f64,
        var_qlo__blk165_rdn3_slot: &mut f64,
        var_qlo__blk165_rdn4_slot: &mut f64,
        var_qlo__blk165_rdn5_slot: &mut f64,
        var_qlo__blk165_rdn6_slot: &mut f64,
        var_qlo__blk165_rdn7_slot: &mut f64,
        var_qlo__blk165_rdn8_slot: &mut f64,
        var_qlo__blk165_rdn9_slot: &mut f64,
        var_qlo__blk165_rv_slot: &mut f64,
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
        var_vn0__blk167_rdn0_slot: &mut f64,
        var_vn0__blk167_rdn1_slot: &mut f64,
        var_vn0__blk167_rdn10_slot: &mut f64,
        var_vn0__blk167_rdn11_slot: &mut f64,
        var_vn0__blk167_rdn12_slot: &mut f64,
        var_vn0__blk167_rdn13_slot: &mut f64,
        var_vn0__blk167_rdn2_slot: &mut f64,
        var_vn0__blk167_rdn3_slot: &mut f64,
        var_vn0__blk167_rdn4_slot: &mut f64,
        var_vn0__blk167_rdn5_slot: &mut f64,
        var_vn0__blk167_rdn6_slot: &mut f64,
        var_vn0__blk167_rdn7_slot: &mut f64,
        var_vn0__blk167_rdn8_slot: &mut f64,
        var_vn0__blk167_rdn9_slot: &mut f64,
        var_vn0__blk167_rv_slot: &mut f64,
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
        var_vnl0__blk168_rdn0_slot: &mut f64,
        var_vnl0__blk168_rdn1_slot: &mut f64,
        var_vnl0__blk168_rdn10_slot: &mut f64,
        var_vnl0__blk168_rdn11_slot: &mut f64,
        var_vnl0__blk168_rdn12_slot: &mut f64,
        var_vnl0__blk168_rdn13_slot: &mut f64,
        var_vnl0__blk168_rdn2_slot: &mut f64,
        var_vnl0__blk168_rdn3_slot: &mut f64,
        var_vnl0__blk168_rdn4_slot: &mut f64,
        var_vnl0__blk168_rdn5_slot: &mut f64,
        var_vnl0__blk168_rdn6_slot: &mut f64,
        var_vnl0__blk168_rdn7_slot: &mut f64,
        var_vnl0__blk168_rdn8_slot: &mut f64,
        var_vnl0__blk168_rdn9_slot: &mut f64,
        var_vnl0__blk168_rv_slot: &mut f64,
    ) {
        let mut var_guard185: f64 = *var_guard185_slot;
        let mut var_guard185_dn0: f64 = *var_guard185_dn0_slot;
        let mut var_guard185_dn1: f64 = *var_guard185_dn1_slot;
        let mut var_guard185_dn10: f64 = *var_guard185_dn10_slot;
        let mut var_guard185_dn11: f64 = *var_guard185_dn11_slot;
        let mut var_guard185_dn12: f64 = *var_guard185_dn12_slot;
        let mut var_guard185_dn13: f64 = *var_guard185_dn13_slot;
        let mut var_guard185_dn2: f64 = *var_guard185_dn2_slot;
        let mut var_guard185_dn3: f64 = *var_guard185_dn3_slot;
        let mut var_guard185_dn4: f64 = *var_guard185_dn4_slot;
        let mut var_guard185_dn5: f64 = *var_guard185_dn5_slot;
        let mut var_guard185_dn6: f64 = *var_guard185_dn6_slot;
        let mut var_guard185_dn7: f64 = *var_guard185_dn7_slot;
        let mut var_guard185_dn8: f64 = *var_guard185_dn8_slot;
        let mut var_guard185_dn9: f64 = *var_guard185_dn9_slot;
        let mut var_guard185_rdn0: f64 = *var_guard185_rdn0_slot;
        let mut var_guard185_rdn1: f64 = *var_guard185_rdn1_slot;
        let mut var_guard185_rdn10: f64 = *var_guard185_rdn10_slot;
        let mut var_guard185_rdn11: f64 = *var_guard185_rdn11_slot;
        let mut var_guard185_rdn12: f64 = *var_guard185_rdn12_slot;
        let mut var_guard185_rdn13: f64 = *var_guard185_rdn13_slot;
        let mut var_guard185_rdn2: f64 = *var_guard185_rdn2_slot;
        let mut var_guard185_rdn3: f64 = *var_guard185_rdn3_slot;
        let mut var_guard185_rdn4: f64 = *var_guard185_rdn4_slot;
        let mut var_guard185_rdn5: f64 = *var_guard185_rdn5_slot;
        let mut var_guard185_rdn6: f64 = *var_guard185_rdn6_slot;
        let mut var_guard185_rdn7: f64 = *var_guard185_rdn7_slot;
        let mut var_guard185_rdn8: f64 = *var_guard185_rdn8_slot;
        let mut var_guard185_rdn9: f64 = *var_guard185_rdn9_slot;
        let mut var_guard185_rv: f64 = *var_guard185_rv_slot;
        let mut var_guard186: f64 = *var_guard186_slot;
        let mut var_guard186_dn0: f64 = *var_guard186_dn0_slot;
        let mut var_guard186_dn1: f64 = *var_guard186_dn1_slot;
        let mut var_guard186_dn10: f64 = *var_guard186_dn10_slot;
        let mut var_guard186_dn11: f64 = *var_guard186_dn11_slot;
        let mut var_guard186_dn12: f64 = *var_guard186_dn12_slot;
        let mut var_guard186_dn13: f64 = *var_guard186_dn13_slot;
        let mut var_guard186_dn2: f64 = *var_guard186_dn2_slot;
        let mut var_guard186_dn3: f64 = *var_guard186_dn3_slot;
        let mut var_guard186_dn4: f64 = *var_guard186_dn4_slot;
        let mut var_guard186_dn5: f64 = *var_guard186_dn5_slot;
        let mut var_guard186_dn6: f64 = *var_guard186_dn6_slot;
        let mut var_guard186_dn7: f64 = *var_guard186_dn7_slot;
        let mut var_guard186_dn8: f64 = *var_guard186_dn8_slot;
        let mut var_guard186_dn9: f64 = *var_guard186_dn9_slot;
        let mut var_guard186_rdn0: f64 = *var_guard186_rdn0_slot;
        let mut var_guard186_rdn1: f64 = *var_guard186_rdn1_slot;
        let mut var_guard186_rdn10: f64 = *var_guard186_rdn10_slot;
        let mut var_guard186_rdn11: f64 = *var_guard186_rdn11_slot;
        let mut var_guard186_rdn12: f64 = *var_guard186_rdn12_slot;
        let mut var_guard186_rdn13: f64 = *var_guard186_rdn13_slot;
        let mut var_guard186_rdn2: f64 = *var_guard186_rdn2_slot;
        let mut var_guard186_rdn3: f64 = *var_guard186_rdn3_slot;
        let mut var_guard186_rdn4: f64 = *var_guard186_rdn4_slot;
        let mut var_guard186_rdn5: f64 = *var_guard186_rdn5_slot;
        let mut var_guard186_rdn6: f64 = *var_guard186_rdn6_slot;
        let mut var_guard186_rdn7: f64 = *var_guard186_rdn7_slot;
        let mut var_guard186_rdn8: f64 = *var_guard186_rdn8_slot;
        let mut var_guard186_rdn9: f64 = *var_guard186_rdn9_slot;
        let mut var_guard186_rv: f64 = *var_guard186_rv_slot;
        let mut var_pwq__blk164: f64 = *var_pwq__blk164_slot;
        let mut var_pwq__blk164_dn0: f64 = *var_pwq__blk164_dn0_slot;
        let mut var_pwq__blk164_dn1: f64 = *var_pwq__blk164_dn1_slot;
        let mut var_pwq__blk164_dn10: f64 = *var_pwq__blk164_dn10_slot;
        let mut var_pwq__blk164_dn11: f64 = *var_pwq__blk164_dn11_slot;
        let mut var_pwq__blk164_dn12: f64 = *var_pwq__blk164_dn12_slot;
        let mut var_pwq__blk164_dn13: f64 = *var_pwq__blk164_dn13_slot;
        let mut var_pwq__blk164_dn2: f64 = *var_pwq__blk164_dn2_slot;
        let mut var_pwq__blk164_dn3: f64 = *var_pwq__blk164_dn3_slot;
        let mut var_pwq__blk164_dn4: f64 = *var_pwq__blk164_dn4_slot;
        let mut var_pwq__blk164_dn5: f64 = *var_pwq__blk164_dn5_slot;
        let mut var_pwq__blk164_dn6: f64 = *var_pwq__blk164_dn6_slot;
        let mut var_pwq__blk164_dn7: f64 = *var_pwq__blk164_dn7_slot;
        let mut var_pwq__blk164_dn8: f64 = *var_pwq__blk164_dn8_slot;
        let mut var_pwq__blk164_dn9: f64 = *var_pwq__blk164_dn9_slot;
        let mut var_pwq__blk164_rdn0: f64 = *var_pwq__blk164_rdn0_slot;
        let mut var_pwq__blk164_rdn1: f64 = *var_pwq__blk164_rdn1_slot;
        let mut var_pwq__blk164_rdn10: f64 = *var_pwq__blk164_rdn10_slot;
        let mut var_pwq__blk164_rdn11: f64 = *var_pwq__blk164_rdn11_slot;
        let mut var_pwq__blk164_rdn12: f64 = *var_pwq__blk164_rdn12_slot;
        let mut var_pwq__blk164_rdn13: f64 = *var_pwq__blk164_rdn13_slot;
        let mut var_pwq__blk164_rdn2: f64 = *var_pwq__blk164_rdn2_slot;
        let mut var_pwq__blk164_rdn3: f64 = *var_pwq__blk164_rdn3_slot;
        let mut var_pwq__blk164_rdn4: f64 = *var_pwq__blk164_rdn4_slot;
        let mut var_pwq__blk164_rdn5: f64 = *var_pwq__blk164_rdn5_slot;
        let mut var_pwq__blk164_rdn6: f64 = *var_pwq__blk164_rdn6_slot;
        let mut var_pwq__blk164_rdn7: f64 = *var_pwq__blk164_rdn7_slot;
        let mut var_pwq__blk164_rdn8: f64 = *var_pwq__blk164_rdn8_slot;
        let mut var_pwq__blk164_rdn9: f64 = *var_pwq__blk164_rdn9_slot;
        let mut var_pwq__blk164_rv: f64 = *var_pwq__blk164_rv_slot;
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
        let mut var_qdbep_rdn0: f64 = *var_qdbep_rdn0_slot;
        let mut var_qdbep_rdn1: f64 = *var_qdbep_rdn1_slot;
        let mut var_qdbep_rdn10: f64 = *var_qdbep_rdn10_slot;
        let mut var_qdbep_rdn11: f64 = *var_qdbep_rdn11_slot;
        let mut var_qdbep_rdn12: f64 = *var_qdbep_rdn12_slot;
        let mut var_qdbep_rdn13: f64 = *var_qdbep_rdn13_slot;
        let mut var_qdbep_rdn2: f64 = *var_qdbep_rdn2_slot;
        let mut var_qdbep_rdn3: f64 = *var_qdbep_rdn3_slot;
        let mut var_qdbep_rdn4: f64 = *var_qdbep_rdn4_slot;
        let mut var_qdbep_rdn5: f64 = *var_qdbep_rdn5_slot;
        let mut var_qdbep_rdn6: f64 = *var_qdbep_rdn6_slot;
        let mut var_qdbep_rdn7: f64 = *var_qdbep_rdn7_slot;
        let mut var_qdbep_rdn8: f64 = *var_qdbep_rdn8_slot;
        let mut var_qdbep_rdn9: f64 = *var_qdbep_rdn9_slot;
        let mut var_qdbep_rv: f64 = *var_qdbep_rv_slot;
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
        let mut var_qhi__blk166_rdn0: f64 = *var_qhi__blk166_rdn0_slot;
        let mut var_qhi__blk166_rdn1: f64 = *var_qhi__blk166_rdn1_slot;
        let mut var_qhi__blk166_rdn10: f64 = *var_qhi__blk166_rdn10_slot;
        let mut var_qhi__blk166_rdn11: f64 = *var_qhi__blk166_rdn11_slot;
        let mut var_qhi__blk166_rdn12: f64 = *var_qhi__blk166_rdn12_slot;
        let mut var_qhi__blk166_rdn13: f64 = *var_qhi__blk166_rdn13_slot;
        let mut var_qhi__blk166_rdn2: f64 = *var_qhi__blk166_rdn2_slot;
        let mut var_qhi__blk166_rdn3: f64 = *var_qhi__blk166_rdn3_slot;
        let mut var_qhi__blk166_rdn4: f64 = *var_qhi__blk166_rdn4_slot;
        let mut var_qhi__blk166_rdn5: f64 = *var_qhi__blk166_rdn5_slot;
        let mut var_qhi__blk166_rdn6: f64 = *var_qhi__blk166_rdn6_slot;
        let mut var_qhi__blk166_rdn7: f64 = *var_qhi__blk166_rdn7_slot;
        let mut var_qhi__blk166_rdn8: f64 = *var_qhi__blk166_rdn8_slot;
        let mut var_qhi__blk166_rdn9: f64 = *var_qhi__blk166_rdn9_slot;
        let mut var_qhi__blk166_rv: f64 = *var_qhi__blk166_rv_slot;
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
        let mut var_qlo__blk165_rdn0: f64 = *var_qlo__blk165_rdn0_slot;
        let mut var_qlo__blk165_rdn1: f64 = *var_qlo__blk165_rdn1_slot;
        let mut var_qlo__blk165_rdn10: f64 = *var_qlo__blk165_rdn10_slot;
        let mut var_qlo__blk165_rdn11: f64 = *var_qlo__blk165_rdn11_slot;
        let mut var_qlo__blk165_rdn12: f64 = *var_qlo__blk165_rdn12_slot;
        let mut var_qlo__blk165_rdn13: f64 = *var_qlo__blk165_rdn13_slot;
        let mut var_qlo__blk165_rdn2: f64 = *var_qlo__blk165_rdn2_slot;
        let mut var_qlo__blk165_rdn3: f64 = *var_qlo__blk165_rdn3_slot;
        let mut var_qlo__blk165_rdn4: f64 = *var_qlo__blk165_rdn4_slot;
        let mut var_qlo__blk165_rdn5: f64 = *var_qlo__blk165_rdn5_slot;
        let mut var_qlo__blk165_rdn6: f64 = *var_qlo__blk165_rdn6_slot;
        let mut var_qlo__blk165_rdn7: f64 = *var_qlo__blk165_rdn7_slot;
        let mut var_qlo__blk165_rdn8: f64 = *var_qlo__blk165_rdn8_slot;
        let mut var_qlo__blk165_rdn9: f64 = *var_qlo__blk165_rdn9_slot;
        let mut var_qlo__blk165_rv: f64 = *var_qlo__blk165_rv_slot;
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
        let mut var_vn0__blk167_rdn0: f64 = *var_vn0__blk167_rdn0_slot;
        let mut var_vn0__blk167_rdn1: f64 = *var_vn0__blk167_rdn1_slot;
        let mut var_vn0__blk167_rdn10: f64 = *var_vn0__blk167_rdn10_slot;
        let mut var_vn0__blk167_rdn11: f64 = *var_vn0__blk167_rdn11_slot;
        let mut var_vn0__blk167_rdn12: f64 = *var_vn0__blk167_rdn12_slot;
        let mut var_vn0__blk167_rdn13: f64 = *var_vn0__blk167_rdn13_slot;
        let mut var_vn0__blk167_rdn2: f64 = *var_vn0__blk167_rdn2_slot;
        let mut var_vn0__blk167_rdn3: f64 = *var_vn0__blk167_rdn3_slot;
        let mut var_vn0__blk167_rdn4: f64 = *var_vn0__blk167_rdn4_slot;
        let mut var_vn0__blk167_rdn5: f64 = *var_vn0__blk167_rdn5_slot;
        let mut var_vn0__blk167_rdn6: f64 = *var_vn0__blk167_rdn6_slot;
        let mut var_vn0__blk167_rdn7: f64 = *var_vn0__blk167_rdn7_slot;
        let mut var_vn0__blk167_rdn8: f64 = *var_vn0__blk167_rdn8_slot;
        let mut var_vn0__blk167_rdn9: f64 = *var_vn0__blk167_rdn9_slot;
        let mut var_vn0__blk167_rv: f64 = *var_vn0__blk167_rv_slot;
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
        let mut var_vnl0__blk168_rdn0: f64 = *var_vnl0__blk168_rdn0_slot;
        let mut var_vnl0__blk168_rdn1: f64 = *var_vnl0__blk168_rdn1_slot;
        let mut var_vnl0__blk168_rdn10: f64 = *var_vnl0__blk168_rdn10_slot;
        let mut var_vnl0__blk168_rdn11: f64 = *var_vnl0__blk168_rdn11_slot;
        let mut var_vnl0__blk168_rdn12: f64 = *var_vnl0__blk168_rdn12_slot;
        let mut var_vnl0__blk168_rdn13: f64 = *var_vnl0__blk168_rdn13_slot;
        let mut var_vnl0__blk168_rdn2: f64 = *var_vnl0__blk168_rdn2_slot;
        let mut var_vnl0__blk168_rdn3: f64 = *var_vnl0__blk168_rdn3_slot;
        let mut var_vnl0__blk168_rdn4: f64 = *var_vnl0__blk168_rdn4_slot;
        let mut var_vnl0__blk168_rdn5: f64 = *var_vnl0__blk168_rdn5_slot;
        let mut var_vnl0__blk168_rdn6: f64 = *var_vnl0__blk168_rdn6_slot;
        let mut var_vnl0__blk168_rdn7: f64 = *var_vnl0__blk168_rdn7_slot;
        let mut var_vnl0__blk168_rdn8: f64 = *var_vnl0__blk168_rdn8_slot;
        let mut var_vnl0__blk168_rdn9: f64 = *var_vnl0__blk168_rdn9_slot;
        let mut var_vnl0__blk168_rv: f64 = *var_vnl0__blk168_rv_slot;

        let (assign5170_e5563, assign5170_e5563_d_n0, assign5170_e5563_d_n1, assign5170_e5563_d_n2, assign5170_e5563_d_n3, assign5170_e5563_d_n4, assign5170_e5563_d_n5, assign5170_e5563_d_n6, assign5170_e5563_d_n7, assign5170_e5563_d_n8, assign5170_e5563_d_n9, assign5170_e5563_d_n10, assign5170_e5563_d_n11, assign5170_e5563_d_n12, assign5170_e5563_d_n13,) = {
    if ((var_guard183 != 0.0) && (var_guard184 != 0.0)) {
        let assign5170_e5556: f64 = (1.0 - p.p34);
        let assign5170_e5558: f64 = (-1.0);
        let assign5170_e5560: f64 = (assign5170_e5558 - p.p43);
        let assign5170_e5561: f64 = (assign5170_e5556).powf(assign5170_e5560);
        (assign5170_e5561, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pwq__blk164, var_pwq__blk164_dn0, var_pwq__blk164_dn1, var_pwq__blk164_dn2, var_pwq__blk164_dn3, var_pwq__blk164_dn4, var_pwq__blk164_dn5, var_pwq__blk164_dn6, var_pwq__blk164_dn7, var_pwq__blk164_dn8, var_pwq__blk164_dn9, var_pwq__blk164_dn10, var_pwq__blk164_dn11, var_pwq__blk164_dn12, var_pwq__blk164_dn13,)
    }
};
        var_pwq__blk164 = assign5170_e5563;
        var_pwq__blk164_dn0 = assign5170_e5563_d_n0;
        var_pwq__blk164_dn1 = assign5170_e5563_d_n1;
        var_pwq__blk164_dn2 = assign5170_e5563_d_n2;
        var_pwq__blk164_dn3 = assign5170_e5563_d_n3;
        var_pwq__blk164_dn4 = assign5170_e5563_d_n4;
        var_pwq__blk164_dn5 = assign5170_e5563_d_n5;
        var_pwq__blk164_dn6 = assign5170_e5563_d_n6;
        var_pwq__blk164_dn7 = assign5170_e5563_d_n7;
        var_pwq__blk164_dn8 = assign5170_e5563_d_n8;
        var_pwq__blk164_dn9 = assign5170_e5563_d_n9;
        var_pwq__blk164_dn10 = assign5170_e5563_d_n10;
        var_pwq__blk164_dn11 = assign5170_e5563_d_n11;
        var_pwq__blk164_dn12 = assign5170_e5563_d_n12;
        var_pwq__blk164_dn13 = assign5170_e5563_d_n13;
        var_pwq__blk164_rv = 0.0;
        var_pwq__blk164_rdn0 = 0.0;
        var_pwq__blk164_rdn1 = 0.0;
        var_pwq__blk164_rdn2 = 0.0;
        var_pwq__blk164_rdn3 = 0.0;
        var_pwq__blk164_rdn4 = 0.0;
        var_pwq__blk164_rdn5 = 0.0;
        var_pwq__blk164_rdn6 = 0.0;
        var_pwq__blk164_rdn7 = 0.0;
        var_pwq__blk164_rdn8 = 0.0;
        var_pwq__blk164_rdn9 = 0.0;
        var_pwq__blk164_rdn10 = 0.0;
        var_pwq__blk164_rdn11 = 0.0;
        var_pwq__blk164_rdn12 = 0.0;
        var_pwq__blk164_rdn13 = 0.0;

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
        (assign5180_e5583, (((var_pc_t_dn0 * assign5180_e5578) + (var_pc_t * (-((var_pwq__blk164_dn0 * assign5180_e5572) * assign5180_e5576)))) / assign5180_e5582), (((var_pc_t_dn1 * assign5180_e5578) + (var_pc_t * (-((var_pwq__blk164_dn1 * assign5180_e5572) * assign5180_e5576)))) / assign5180_e5582), (((var_pc_t_dn2 * assign5180_e5578) + (var_pc_t * (-((var_pwq__blk164_dn2 * assign5180_e5572) * assign5180_e5576)))) / assign5180_e5582), (((var_pc_t_dn3 * assign5180_e5578) + (var_pc_t * (-((var_pwq__blk164_dn3 * assign5180_e5572) * assign5180_e5576)))) / assign5180_e5582), (((var_pc_t_dn4 * assign5180_e5578) + (var_pc_t * (-((var_pwq__blk164_dn4 * assign5180_e5572) * assign5180_e5576)))) / assign5180_e5582), (((var_pc_t_dn5 * assign5180_e5578) + (var_pc_t * (-((var_pwq__blk164_dn5 * assign5180_e5572) * assign5180_e5576)))) / assign5180_e5582), (((var_pc_t_dn6 * assign5180_e5578) + (var_pc_t * (-((var_pwq__blk164_dn6 * assign5180_e5572) * assign5180_e5576)))) / assign5180_e5582), (((var_pc_t_dn7 * assign5180_e5578) + (var_pc_t * (-((var_pwq__blk164_dn7 * assign5180_e5572) * assign5180_e5576)))) / assign5180_e5582), (((var_pc_t_dn8 * assign5180_e5578) + (var_pc_t * (-((var_pwq__blk164_dn8 * assign5180_e5572) * assign5180_e5576)))) / assign5180_e5582), (((var_pc_t_dn9 * assign5180_e5578) + (var_pc_t * (-((var_pwq__blk164_dn9 * assign5180_e5572) * assign5180_e5576)))) / assign5180_e5582), (((var_pc_t_dn10 * assign5180_e5578) + (var_pc_t * (-((var_pwq__blk164_dn10 * assign5180_e5572) * assign5180_e5576)))) / assign5180_e5582), (((var_pc_t_dn11 * assign5180_e5578) + (var_pc_t * (-((var_pwq__blk164_dn11 * assign5180_e5572) * assign5180_e5576)))) / assign5180_e5582), (((var_pc_t_dn12 * assign5180_e5578) + (var_pc_t * (-((var_pwq__blk164_dn12 * assign5180_e5572) * assign5180_e5576)))) / assign5180_e5582), (((var_pc_t_dn13 * assign5180_e5578) + (var_pc_t * (-((var_pwq__blk164_dn13 * assign5180_e5572) * assign5180_e5576)))) / assign5180_e5582),)
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
        var_qlo__blk165_rv = 0.0;
        var_qlo__blk165_rdn0 = 0.0;
        var_qlo__blk165_rdn1 = 0.0;
        var_qlo__blk165_rdn2 = 0.0;
        var_qlo__blk165_rdn3 = 0.0;
        var_qlo__blk165_rdn4 = 0.0;
        var_qlo__blk165_rdn5 = 0.0;
        var_qlo__blk165_rdn6 = 0.0;
        var_qlo__blk165_rdn7 = 0.0;
        var_qlo__blk165_rdn8 = 0.0;
        var_qlo__blk165_rdn9 = 0.0;
        var_qlo__blk165_rdn10 = 0.0;
        var_qlo__blk165_rdn11 = 0.0;
        var_qlo__blk165_rdn12 = 0.0;
        var_qlo__blk165_rdn13 = 0.0;

        let (assign5190_e5605, assign5190_e5605_d_n0, assign5190_e5605_d_n1, assign5190_e5605_d_n2, assign5190_e5605_d_n3, assign5190_e5605_d_n4, assign5190_e5605_d_n5, assign5190_e5605_d_n6, assign5190_e5605_d_n7, assign5190_e5605_d_n8, assign5190_e5605_d_n9, assign5190_e5605_d_n10, assign5190_e5605_d_n11, assign5190_e5605_d_n12, assign5190_e5605_d_n13,) = {
    if ((var_guard183 != 0.0) && (var_guard184 != 0.0)) {
        let assign5190_e5592: f64 = (1.0 - p.p34);
        let assign5190_e5595: f64 = (0.5 * p.p43);
        let assign5190_e5597: f64 = (assign5190_e5595 * var_dvh__blk163);
        let assign5190_e5599: f64 = (assign5190_e5597 / var_pc_t);
        let assign5190_e5600: f64 = (assign5190_e5592 + assign5190_e5599);
        let assign5190_e5601: f64 = (var_dvh__blk163 * assign5190_e5600);
        let assign5190_e5603: f64 = (assign5190_e5601 * var_pwq__blk164);
        (assign5190_e5603, ((((var_dvh__blk163_dn0 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn0) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn0)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164) + (assign5190_e5601 * var_pwq__blk164_dn0)), ((((var_dvh__blk163_dn1 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn1) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn1)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164) + (assign5190_e5601 * var_pwq__blk164_dn1)), ((((var_dvh__blk163_dn2 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn2) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn2)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164) + (assign5190_e5601 * var_pwq__blk164_dn2)), ((((var_dvh__blk163_dn3 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn3) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn3)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164) + (assign5190_e5601 * var_pwq__blk164_dn3)), ((((var_dvh__blk163_dn4 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn4) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn4)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164) + (assign5190_e5601 * var_pwq__blk164_dn4)), ((((var_dvh__blk163_dn5 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn5) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn5)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164) + (assign5190_e5601 * var_pwq__blk164_dn5)), ((((var_dvh__blk163_dn6 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn6) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn6)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164) + (assign5190_e5601 * var_pwq__blk164_dn6)), ((((var_dvh__blk163_dn7 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn7) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn7)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164) + (assign5190_e5601 * var_pwq__blk164_dn7)), ((((var_dvh__blk163_dn8 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn8) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn8)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164) + (assign5190_e5601 * var_pwq__blk164_dn8)), ((((var_dvh__blk163_dn9 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn9) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn9)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164) + (assign5190_e5601 * var_pwq__blk164_dn9)), ((((var_dvh__blk163_dn10 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn10) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn10)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164) + (assign5190_e5601 * var_pwq__blk164_dn10)), ((((var_dvh__blk163_dn11 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn11) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn11)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164) + (assign5190_e5601 * var_pwq__blk164_dn11)), ((((var_dvh__blk163_dn12 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn12) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn12)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164) + (assign5190_e5601 * var_pwq__blk164_dn12)), ((((var_dvh__blk163_dn13 * assign5190_e5600) + (var_dvh__blk163 * ((((assign5190_e5595 * var_dvh__blk163_dn13) * var_pc_t) - (assign5190_e5597 * var_pc_t_dn13)) / (var_pc_t * var_pc_t)))) * var_pwq__blk164) + (assign5190_e5601 * var_pwq__blk164_dn13)),)
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
        var_qhi__blk166_rv = 0.0;
        var_qhi__blk166_rdn0 = 0.0;
        var_qhi__blk166_rdn1 = 0.0;
        var_qhi__blk166_rdn2 = 0.0;
        var_qhi__blk166_rdn3 = 0.0;
        var_qhi__blk166_rdn4 = 0.0;
        var_qhi__blk166_rdn5 = 0.0;
        var_qhi__blk166_rdn6 = 0.0;
        var_qhi__blk166_rdn7 = 0.0;
        var_qhi__blk166_rdn8 = 0.0;
        var_qhi__blk166_rdn9 = 0.0;
        var_qhi__blk166_rdn10 = 0.0;
        var_qhi__blk166_rdn11 = 0.0;
        var_qhi__blk166_rdn12 = 0.0;
        var_qhi__blk166_rdn13 = 0.0;

        let assign5200_e5611: f64 = (-p.p45);
        let assign5200_e5613: f64 = if ((p.p45 > 0.0) && (var_vbep < assign5200_e5611)) { 1.0 } else { 0.0 };
        var_guard185 = assign5200_e5613;
        var_guard185_dn0 = 0.0;
        var_guard185_dn1 = 0.0;
        var_guard185_dn2 = 0.0;
        var_guard185_dn3 = 0.0;
        var_guard185_dn4 = 0.0;
        var_guard185_dn5 = 0.0;
        var_guard185_dn6 = 0.0;
        var_guard185_dn7 = 0.0;
        var_guard185_dn8 = 0.0;
        var_guard185_dn9 = 0.0;
        var_guard185_dn10 = 0.0;
        var_guard185_dn11 = 0.0;
        var_guard185_dn12 = 0.0;
        var_guard185_dn13 = 0.0;
        var_guard185_rv = 0.0;
        var_guard185_rdn0 = 0.0;
        var_guard185_rdn1 = 0.0;
        var_guard185_rdn2 = 0.0;
        var_guard185_rdn3 = 0.0;
        var_guard185_rdn4 = 0.0;
        var_guard185_rdn5 = 0.0;
        var_guard185_rdn6 = 0.0;
        var_guard185_rdn7 = 0.0;
        var_guard185_rdn8 = 0.0;
        var_guard185_rdn9 = 0.0;
        var_guard185_rdn10 = 0.0;
        var_guard185_rdn11 = 0.0;
        var_guard185_rdn12 = 0.0;
        var_guard185_rdn13 = 0.0;

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
        var_qlo__blk165_rv = 0.0;
        var_qlo__blk165_rdn0 = 0.0;
        var_qlo__blk165_rdn1 = 0.0;
        var_qlo__blk165_rdn2 = 0.0;
        var_qlo__blk165_rdn3 = 0.0;
        var_qlo__blk165_rdn4 = 0.0;
        var_qlo__blk165_rdn5 = 0.0;
        var_qlo__blk165_rdn6 = 0.0;
        var_qlo__blk165_rdn7 = 0.0;
        var_qlo__blk165_rdn8 = 0.0;
        var_qlo__blk165_rdn9 = 0.0;
        var_qlo__blk165_rdn10 = 0.0;
        var_qlo__blk165_rdn11 = 0.0;
        var_qlo__blk165_rdn12 = 0.0;
        var_qlo__blk165_rdn13 = 0.0;

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
        var_qlo__blk165_rv = 0.0;
        var_qlo__blk165_rdn0 = 0.0;
        var_qlo__blk165_rdn1 = 0.0;
        var_qlo__blk165_rdn2 = 0.0;
        var_qlo__blk165_rdn3 = 0.0;
        var_qlo__blk165_rdn4 = 0.0;
        var_qlo__blk165_rdn5 = 0.0;
        var_qlo__blk165_rdn6 = 0.0;
        var_qlo__blk165_rdn7 = 0.0;
        var_qlo__blk165_rdn8 = 0.0;
        var_qlo__blk165_rdn9 = 0.0;
        var_qlo__blk165_rdn10 = 0.0;
        var_qlo__blk165_rdn11 = 0.0;
        var_qlo__blk165_rdn12 = 0.0;
        var_qlo__blk165_rdn13 = 0.0;

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
        var_qhi__blk166_rv = 0.0;
        var_qhi__blk166_rdn0 = 0.0;
        var_qhi__blk166_rdn1 = 0.0;
        var_qhi__blk166_rdn2 = 0.0;
        var_qhi__blk166_rdn3 = 0.0;
        var_qhi__blk166_rdn4 = 0.0;
        var_qhi__blk166_rdn5 = 0.0;
        var_qhi__blk166_rdn6 = 0.0;
        var_qhi__blk166_rdn7 = 0.0;
        var_qhi__blk166_rdn8 = 0.0;
        var_qhi__blk166_rdn9 = 0.0;
        var_qhi__blk166_rdn10 = 0.0;
        var_qhi__blk166_rdn11 = 0.0;
        var_qhi__blk166_rdn12 = 0.0;
        var_qhi__blk166_rdn13 = 0.0;

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
        var_qdbep_rv = 0.0;
        var_qdbep_rdn0 = 0.0;
        var_qdbep_rdn1 = 0.0;
        var_qdbep_rdn2 = 0.0;
        var_qdbep_rdn3 = 0.0;
        var_qdbep_rdn4 = 0.0;
        var_qdbep_rdn5 = 0.0;
        var_qdbep_rdn6 = 0.0;
        var_qdbep_rdn7 = 0.0;
        var_qdbep_rdn8 = 0.0;
        var_qdbep_rdn9 = 0.0;
        var_qdbep_rdn10 = 0.0;
        var_qdbep_rdn11 = 0.0;
        var_qdbep_rdn12 = 0.0;
        var_qdbep_rdn13 = 0.0;

        let assign5250_e5698: f64 = if ((p.p45 > 0.0) && (p.p46 > 0.0)) { 1.0 } else { 0.0 };
        var_guard186 = assign5250_e5698;
        var_guard186_dn0 = 0.0;
        var_guard186_dn1 = 0.0;
        var_guard186_dn2 = 0.0;
        var_guard186_dn3 = 0.0;
        var_guard186_dn4 = 0.0;
        var_guard186_dn5 = 0.0;
        var_guard186_dn6 = 0.0;
        var_guard186_dn7 = 0.0;
        var_guard186_dn8 = 0.0;
        var_guard186_dn9 = 0.0;
        var_guard186_dn10 = 0.0;
        var_guard186_dn11 = 0.0;
        var_guard186_dn12 = 0.0;
        var_guard186_dn13 = 0.0;
        var_guard186_rv = 0.0;
        var_guard186_rdn0 = 0.0;
        var_guard186_rdn1 = 0.0;
        var_guard186_rdn2 = 0.0;
        var_guard186_rdn3 = 0.0;
        var_guard186_rdn4 = 0.0;
        var_guard186_rdn5 = 0.0;
        var_guard186_rdn6 = 0.0;
        var_guard186_rdn7 = 0.0;
        var_guard186_rdn8 = 0.0;
        var_guard186_rdn9 = 0.0;
        var_guard186_rdn10 = 0.0;
        var_guard186_rdn11 = 0.0;
        var_guard186_rdn12 = 0.0;
        var_guard186_rdn13 = 0.0;

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
        var_vn0__blk167_rv = 0.0;
        var_vn0__blk167_rdn0 = 0.0;
        var_vn0__blk167_rdn1 = 0.0;
        var_vn0__blk167_rdn2 = 0.0;
        var_vn0__blk167_rdn3 = 0.0;
        var_vn0__blk167_rdn4 = 0.0;
        var_vn0__blk167_rdn5 = 0.0;
        var_vn0__blk167_rdn6 = 0.0;
        var_vn0__blk167_rdn7 = 0.0;
        var_vn0__blk167_rdn8 = 0.0;
        var_vn0__blk167_rdn9 = 0.0;
        var_vn0__blk167_rdn10 = 0.0;
        var_vn0__blk167_rdn11 = 0.0;
        var_vn0__blk167_rdn12 = 0.0;
        var_vn0__blk167_rdn13 = 0.0;

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
        var_vnl0__blk168_rv = 0.0;
        var_vnl0__blk168_rdn0 = 0.0;
        var_vnl0__blk168_rdn1 = 0.0;
        var_vnl0__blk168_rdn2 = 0.0;
        var_vnl0__blk168_rdn3 = 0.0;
        var_vnl0__blk168_rdn4 = 0.0;
        var_vnl0__blk168_rdn5 = 0.0;
        var_vnl0__blk168_rdn6 = 0.0;
        var_vnl0__blk168_rdn7 = 0.0;
        var_vnl0__blk168_rdn8 = 0.0;
        var_vnl0__blk168_rdn9 = 0.0;
        var_vnl0__blk168_rdn10 = 0.0;
        var_vnl0__blk168_rdn11 = 0.0;
        var_vnl0__blk168_rdn12 = 0.0;
        var_vnl0__blk168_rdn13 = 0.0;


        *var_guard185_slot = var_guard185;
        *var_guard185_dn0_slot = var_guard185_dn0;
        *var_guard185_dn1_slot = var_guard185_dn1;
        *var_guard185_dn10_slot = var_guard185_dn10;
        *var_guard185_dn11_slot = var_guard185_dn11;
        *var_guard185_dn12_slot = var_guard185_dn12;
        *var_guard185_dn13_slot = var_guard185_dn13;
        *var_guard185_dn2_slot = var_guard185_dn2;
        *var_guard185_dn3_slot = var_guard185_dn3;
        *var_guard185_dn4_slot = var_guard185_dn4;
        *var_guard185_dn5_slot = var_guard185_dn5;
        *var_guard185_dn6_slot = var_guard185_dn6;
        *var_guard185_dn7_slot = var_guard185_dn7;
        *var_guard185_dn8_slot = var_guard185_dn8;
        *var_guard185_dn9_slot = var_guard185_dn9;
        *var_guard185_rdn0_slot = var_guard185_rdn0;
        *var_guard185_rdn1_slot = var_guard185_rdn1;
        *var_guard185_rdn10_slot = var_guard185_rdn10;
        *var_guard185_rdn11_slot = var_guard185_rdn11;
        *var_guard185_rdn12_slot = var_guard185_rdn12;
        *var_guard185_rdn13_slot = var_guard185_rdn13;
        *var_guard185_rdn2_slot = var_guard185_rdn2;
        *var_guard185_rdn3_slot = var_guard185_rdn3;
        *var_guard185_rdn4_slot = var_guard185_rdn4;
        *var_guard185_rdn5_slot = var_guard185_rdn5;
        *var_guard185_rdn6_slot = var_guard185_rdn6;
        *var_guard185_rdn7_slot = var_guard185_rdn7;
        *var_guard185_rdn8_slot = var_guard185_rdn8;
        *var_guard185_rdn9_slot = var_guard185_rdn9;
        *var_guard185_rv_slot = var_guard185_rv;
        *var_guard186_slot = var_guard186;
        *var_guard186_dn0_slot = var_guard186_dn0;
        *var_guard186_dn1_slot = var_guard186_dn1;
        *var_guard186_dn10_slot = var_guard186_dn10;
        *var_guard186_dn11_slot = var_guard186_dn11;
        *var_guard186_dn12_slot = var_guard186_dn12;
        *var_guard186_dn13_slot = var_guard186_dn13;
        *var_guard186_dn2_slot = var_guard186_dn2;
        *var_guard186_dn3_slot = var_guard186_dn3;
        *var_guard186_dn4_slot = var_guard186_dn4;
        *var_guard186_dn5_slot = var_guard186_dn5;
        *var_guard186_dn6_slot = var_guard186_dn6;
        *var_guard186_dn7_slot = var_guard186_dn7;
        *var_guard186_dn8_slot = var_guard186_dn8;
        *var_guard186_dn9_slot = var_guard186_dn9;
        *var_guard186_rdn0_slot = var_guard186_rdn0;
        *var_guard186_rdn1_slot = var_guard186_rdn1;
        *var_guard186_rdn10_slot = var_guard186_rdn10;
        *var_guard186_rdn11_slot = var_guard186_rdn11;
        *var_guard186_rdn12_slot = var_guard186_rdn12;
        *var_guard186_rdn13_slot = var_guard186_rdn13;
        *var_guard186_rdn2_slot = var_guard186_rdn2;
        *var_guard186_rdn3_slot = var_guard186_rdn3;
        *var_guard186_rdn4_slot = var_guard186_rdn4;
        *var_guard186_rdn5_slot = var_guard186_rdn5;
        *var_guard186_rdn6_slot = var_guard186_rdn6;
        *var_guard186_rdn7_slot = var_guard186_rdn7;
        *var_guard186_rdn8_slot = var_guard186_rdn8;
        *var_guard186_rdn9_slot = var_guard186_rdn9;
        *var_guard186_rv_slot = var_guard186_rv;
        *var_pwq__blk164_slot = var_pwq__blk164;
        *var_pwq__blk164_dn0_slot = var_pwq__blk164_dn0;
        *var_pwq__blk164_dn1_slot = var_pwq__blk164_dn1;
        *var_pwq__blk164_dn10_slot = var_pwq__blk164_dn10;
        *var_pwq__blk164_dn11_slot = var_pwq__blk164_dn11;
        *var_pwq__blk164_dn12_slot = var_pwq__blk164_dn12;
        *var_pwq__blk164_dn13_slot = var_pwq__blk164_dn13;
        *var_pwq__blk164_dn2_slot = var_pwq__blk164_dn2;
        *var_pwq__blk164_dn3_slot = var_pwq__blk164_dn3;
        *var_pwq__blk164_dn4_slot = var_pwq__blk164_dn4;
        *var_pwq__blk164_dn5_slot = var_pwq__blk164_dn5;
        *var_pwq__blk164_dn6_slot = var_pwq__blk164_dn6;
        *var_pwq__blk164_dn7_slot = var_pwq__blk164_dn7;
        *var_pwq__blk164_dn8_slot = var_pwq__blk164_dn8;
        *var_pwq__blk164_dn9_slot = var_pwq__blk164_dn9;
        *var_pwq__blk164_rdn0_slot = var_pwq__blk164_rdn0;
        *var_pwq__blk164_rdn1_slot = var_pwq__blk164_rdn1;
        *var_pwq__blk164_rdn10_slot = var_pwq__blk164_rdn10;
        *var_pwq__blk164_rdn11_slot = var_pwq__blk164_rdn11;
        *var_pwq__blk164_rdn12_slot = var_pwq__blk164_rdn12;
        *var_pwq__blk164_rdn13_slot = var_pwq__blk164_rdn13;
        *var_pwq__blk164_rdn2_slot = var_pwq__blk164_rdn2;
        *var_pwq__blk164_rdn3_slot = var_pwq__blk164_rdn3;
        *var_pwq__blk164_rdn4_slot = var_pwq__blk164_rdn4;
        *var_pwq__blk164_rdn5_slot = var_pwq__blk164_rdn5;
        *var_pwq__blk164_rdn6_slot = var_pwq__blk164_rdn6;
        *var_pwq__blk164_rdn7_slot = var_pwq__blk164_rdn7;
        *var_pwq__blk164_rdn8_slot = var_pwq__blk164_rdn8;
        *var_pwq__blk164_rdn9_slot = var_pwq__blk164_rdn9;
        *var_pwq__blk164_rv_slot = var_pwq__blk164_rv;
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
        *var_qdbep_rdn0_slot = var_qdbep_rdn0;
        *var_qdbep_rdn1_slot = var_qdbep_rdn1;
        *var_qdbep_rdn10_slot = var_qdbep_rdn10;
        *var_qdbep_rdn11_slot = var_qdbep_rdn11;
        *var_qdbep_rdn12_slot = var_qdbep_rdn12;
        *var_qdbep_rdn13_slot = var_qdbep_rdn13;
        *var_qdbep_rdn2_slot = var_qdbep_rdn2;
        *var_qdbep_rdn3_slot = var_qdbep_rdn3;
        *var_qdbep_rdn4_slot = var_qdbep_rdn4;
        *var_qdbep_rdn5_slot = var_qdbep_rdn5;
        *var_qdbep_rdn6_slot = var_qdbep_rdn6;
        *var_qdbep_rdn7_slot = var_qdbep_rdn7;
        *var_qdbep_rdn8_slot = var_qdbep_rdn8;
        *var_qdbep_rdn9_slot = var_qdbep_rdn9;
        *var_qdbep_rv_slot = var_qdbep_rv;
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
        *var_qhi__blk166_rdn0_slot = var_qhi__blk166_rdn0;
        *var_qhi__blk166_rdn1_slot = var_qhi__blk166_rdn1;
        *var_qhi__blk166_rdn10_slot = var_qhi__blk166_rdn10;
        *var_qhi__blk166_rdn11_slot = var_qhi__blk166_rdn11;
        *var_qhi__blk166_rdn12_slot = var_qhi__blk166_rdn12;
        *var_qhi__blk166_rdn13_slot = var_qhi__blk166_rdn13;
        *var_qhi__blk166_rdn2_slot = var_qhi__blk166_rdn2;
        *var_qhi__blk166_rdn3_slot = var_qhi__blk166_rdn3;
        *var_qhi__blk166_rdn4_slot = var_qhi__blk166_rdn4;
        *var_qhi__blk166_rdn5_slot = var_qhi__blk166_rdn5;
        *var_qhi__blk166_rdn6_slot = var_qhi__blk166_rdn6;
        *var_qhi__blk166_rdn7_slot = var_qhi__blk166_rdn7;
        *var_qhi__blk166_rdn8_slot = var_qhi__blk166_rdn8;
        *var_qhi__blk166_rdn9_slot = var_qhi__blk166_rdn9;
        *var_qhi__blk166_rv_slot = var_qhi__blk166_rv;
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
        *var_qlo__blk165_rdn0_slot = var_qlo__blk165_rdn0;
        *var_qlo__blk165_rdn1_slot = var_qlo__blk165_rdn1;
        *var_qlo__blk165_rdn10_slot = var_qlo__blk165_rdn10;
        *var_qlo__blk165_rdn11_slot = var_qlo__blk165_rdn11;
        *var_qlo__blk165_rdn12_slot = var_qlo__blk165_rdn12;
        *var_qlo__blk165_rdn13_slot = var_qlo__blk165_rdn13;
        *var_qlo__blk165_rdn2_slot = var_qlo__blk165_rdn2;
        *var_qlo__blk165_rdn3_slot = var_qlo__blk165_rdn3;
        *var_qlo__blk165_rdn4_slot = var_qlo__blk165_rdn4;
        *var_qlo__blk165_rdn5_slot = var_qlo__blk165_rdn5;
        *var_qlo__blk165_rdn6_slot = var_qlo__blk165_rdn6;
        *var_qlo__blk165_rdn7_slot = var_qlo__blk165_rdn7;
        *var_qlo__blk165_rdn8_slot = var_qlo__blk165_rdn8;
        *var_qlo__blk165_rdn9_slot = var_qlo__blk165_rdn9;
        *var_qlo__blk165_rv_slot = var_qlo__blk165_rv;
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
        *var_vn0__blk167_rdn0_slot = var_vn0__blk167_rdn0;
        *var_vn0__blk167_rdn1_slot = var_vn0__blk167_rdn1;
        *var_vn0__blk167_rdn10_slot = var_vn0__blk167_rdn10;
        *var_vn0__blk167_rdn11_slot = var_vn0__blk167_rdn11;
        *var_vn0__blk167_rdn12_slot = var_vn0__blk167_rdn12;
        *var_vn0__blk167_rdn13_slot = var_vn0__blk167_rdn13;
        *var_vn0__blk167_rdn2_slot = var_vn0__blk167_rdn2;
        *var_vn0__blk167_rdn3_slot = var_vn0__blk167_rdn3;
        *var_vn0__blk167_rdn4_slot = var_vn0__blk167_rdn4;
        *var_vn0__blk167_rdn5_slot = var_vn0__blk167_rdn5;
        *var_vn0__blk167_rdn6_slot = var_vn0__blk167_rdn6;
        *var_vn0__blk167_rdn7_slot = var_vn0__blk167_rdn7;
        *var_vn0__blk167_rdn8_slot = var_vn0__blk167_rdn8;
        *var_vn0__blk167_rdn9_slot = var_vn0__blk167_rdn9;
        *var_vn0__blk167_rv_slot = var_vn0__blk167_rv;
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
        *var_vnl0__blk168_rdn0_slot = var_vnl0__blk168_rdn0;
        *var_vnl0__blk168_rdn1_slot = var_vnl0__blk168_rdn1;
        *var_vnl0__blk168_rdn10_slot = var_vnl0__blk168_rdn10;
        *var_vnl0__blk168_rdn11_slot = var_vnl0__blk168_rdn11;
        *var_vnl0__blk168_rdn12_slot = var_vnl0__blk168_rdn12;
        *var_vnl0__blk168_rdn13_slot = var_vnl0__blk168_rdn13;
        *var_vnl0__blk168_rdn2_slot = var_vnl0__blk168_rdn2;
        *var_vnl0__blk168_rdn3_slot = var_vnl0__blk168_rdn3;
        *var_vnl0__blk168_rdn4_slot = var_vnl0__blk168_rdn4;
        *var_vnl0__blk168_rdn5_slot = var_vnl0__blk168_rdn5;
        *var_vnl0__blk168_rdn6_slot = var_vnl0__blk168_rdn6;
        *var_vnl0__blk168_rdn7_slot = var_vnl0__blk168_rdn7;
        *var_vnl0__blk168_rdn8_slot = var_vnl0__blk168_rdn8;
        *var_vnl0__blk168_rdn9_slot = var_vnl0__blk168_rdn9;
        *var_vnl0__blk168_rv_slot = var_vnl0__blk168_rv;
    }

    pub(super) fn stamp_reactive_block_26(
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
        var_vnl0__blk168: f64,
        var_vnl0__blk168_dn0: f64,
        var_vnl0__blk168_dn1: f64,
        var_vnl0__blk168_dn10: f64,
        var_vnl0__blk168_dn11: f64,
        var_vnl0__blk168_dn12: f64,
        var_vnl0__blk168_dn13: f64,
        var_vnl0__blk168_dn2: f64,
        var_vnl0__blk168_dn3: f64,
        var_vnl0__blk168_dn4: f64,
        var_vnl0__blk168_dn5: f64,
        var_vnl0__blk168_dn6: f64,
        var_vnl0__blk168_dn7: f64,
        var_vnl0__blk168_dn8: f64,
        var_vnl0__blk168_dn9: f64,
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
        var_cl__blk177_rdn0_slot: &mut f64,
        var_cl__blk177_rdn1_slot: &mut f64,
        var_cl__blk177_rdn10_slot: &mut f64,
        var_cl__blk177_rdn11_slot: &mut f64,
        var_cl__blk177_rdn12_slot: &mut f64,
        var_cl__blk177_rdn13_slot: &mut f64,
        var_cl__blk177_rdn2_slot: &mut f64,
        var_cl__blk177_rdn3_slot: &mut f64,
        var_cl__blk177_rdn4_slot: &mut f64,
        var_cl__blk177_rdn5_slot: &mut f64,
        var_cl__blk177_rdn6_slot: &mut f64,
        var_cl__blk177_rdn7_slot: &mut f64,
        var_cl__blk177_rdn8_slot: &mut f64,
        var_cl__blk177_rdn9_slot: &mut f64,
        var_cl__blk177_rv_slot: &mut f64,
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
        var_cmx__blk176_rdn0_slot: &mut f64,
        var_cmx__blk176_rdn1_slot: &mut f64,
        var_cmx__blk176_rdn10_slot: &mut f64,
        var_cmx__blk176_rdn11_slot: &mut f64,
        var_cmx__blk176_rdn12_slot: &mut f64,
        var_cmx__blk176_rdn13_slot: &mut f64,
        var_cmx__blk176_rdn2_slot: &mut f64,
        var_cmx__blk176_rdn3_slot: &mut f64,
        var_cmx__blk176_rdn4_slot: &mut f64,
        var_cmx__blk176_rdn5_slot: &mut f64,
        var_cmx__blk176_rdn6_slot: &mut f64,
        var_cmx__blk176_rdn7_slot: &mut f64,
        var_cmx__blk176_rdn8_slot: &mut f64,
        var_cmx__blk176_rdn9_slot: &mut f64,
        var_cmx__blk176_rv_slot: &mut f64,
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
        var_crt__blk175_rdn0_slot: &mut f64,
        var_crt__blk175_rdn1_slot: &mut f64,
        var_crt__blk175_rdn10_slot: &mut f64,
        var_crt__blk175_rdn11_slot: &mut f64,
        var_crt__blk175_rdn12_slot: &mut f64,
        var_crt__blk175_rdn13_slot: &mut f64,
        var_crt__blk175_rdn2_slot: &mut f64,
        var_crt__blk175_rdn3_slot: &mut f64,
        var_crt__blk175_rdn4_slot: &mut f64,
        var_crt__blk175_rdn5_slot: &mut f64,
        var_crt__blk175_rdn6_slot: &mut f64,
        var_crt__blk175_rdn7_slot: &mut f64,
        var_crt__blk175_rdn8_slot: &mut f64,
        var_crt__blk175_rdn9_slot: &mut f64,
        var_crt__blk175_rv_slot: &mut f64,
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
        var_ql__blk178_rdn0_slot: &mut f64,
        var_ql__blk178_rdn1_slot: &mut f64,
        var_ql__blk178_rdn10_slot: &mut f64,
        var_ql__blk178_rdn11_slot: &mut f64,
        var_ql__blk178_rdn12_slot: &mut f64,
        var_ql__blk178_rdn13_slot: &mut f64,
        var_ql__blk178_rdn2_slot: &mut f64,
        var_ql__blk178_rdn3_slot: &mut f64,
        var_ql__blk178_rdn4_slot: &mut f64,
        var_ql__blk178_rdn5_slot: &mut f64,
        var_ql__blk178_rdn6_slot: &mut f64,
        var_ql__blk178_rdn7_slot: &mut f64,
        var_ql__blk178_rdn8_slot: &mut f64,
        var_ql__blk178_rdn9_slot: &mut f64,
        var_ql__blk178_rv_slot: &mut f64,
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
        var_qlo0__blk170_rdn0_slot: &mut f64,
        var_qlo0__blk170_rdn1_slot: &mut f64,
        var_qlo0__blk170_rdn10_slot: &mut f64,
        var_qlo0__blk170_rdn11_slot: &mut f64,
        var_qlo0__blk170_rdn12_slot: &mut f64,
        var_qlo0__blk170_rdn13_slot: &mut f64,
        var_qlo0__blk170_rdn2_slot: &mut f64,
        var_qlo0__blk170_rdn3_slot: &mut f64,
        var_qlo0__blk170_rdn4_slot: &mut f64,
        var_qlo0__blk170_rdn5_slot: &mut f64,
        var_qlo0__blk170_rdn6_slot: &mut f64,
        var_qlo0__blk170_rdn7_slot: &mut f64,
        var_qlo0__blk170_rdn8_slot: &mut f64,
        var_qlo0__blk170_rdn9_slot: &mut f64,
        var_qlo0__blk170_rv_slot: &mut f64,
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
        var_qlo__blk165_rdn0_slot: &mut f64,
        var_qlo__blk165_rdn1_slot: &mut f64,
        var_qlo__blk165_rdn10_slot: &mut f64,
        var_qlo__blk165_rdn11_slot: &mut f64,
        var_qlo__blk165_rdn12_slot: &mut f64,
        var_qlo__blk165_rdn13_slot: &mut f64,
        var_qlo__blk165_rdn2_slot: &mut f64,
        var_qlo__blk165_rdn3_slot: &mut f64,
        var_qlo__blk165_rdn4_slot: &mut f64,
        var_qlo__blk165_rdn5_slot: &mut f64,
        var_qlo__blk165_rdn6_slot: &mut f64,
        var_qlo__blk165_rdn7_slot: &mut f64,
        var_qlo__blk165_rdn8_slot: &mut f64,
        var_qlo__blk165_rdn9_slot: &mut f64,
        var_qlo__blk165_rv_slot: &mut f64,
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
        var_sel__blk174_rdn0_slot: &mut f64,
        var_sel__blk174_rdn1_slot: &mut f64,
        var_sel__blk174_rdn10_slot: &mut f64,
        var_sel__blk174_rdn11_slot: &mut f64,
        var_sel__blk174_rdn12_slot: &mut f64,
        var_sel__blk174_rdn13_slot: &mut f64,
        var_sel__blk174_rdn2_slot: &mut f64,
        var_sel__blk174_rdn3_slot: &mut f64,
        var_sel__blk174_rdn4_slot: &mut f64,
        var_sel__blk174_rdn5_slot: &mut f64,
        var_sel__blk174_rdn6_slot: &mut f64,
        var_sel__blk174_rdn7_slot: &mut f64,
        var_sel__blk174_rdn8_slot: &mut f64,
        var_sel__blk174_rdn9_slot: &mut f64,
        var_sel__blk174_rv_slot: &mut f64,
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
        var_vl0__blk169_rdn0_slot: &mut f64,
        var_vl0__blk169_rdn1_slot: &mut f64,
        var_vl0__blk169_rdn10_slot: &mut f64,
        var_vl0__blk169_rdn11_slot: &mut f64,
        var_vl0__blk169_rdn12_slot: &mut f64,
        var_vl0__blk169_rdn13_slot: &mut f64,
        var_vl0__blk169_rdn2_slot: &mut f64,
        var_vl0__blk169_rdn3_slot: &mut f64,
        var_vl0__blk169_rdn4_slot: &mut f64,
        var_vl0__blk169_rdn5_slot: &mut f64,
        var_vl0__blk169_rdn6_slot: &mut f64,
        var_vl0__blk169_rdn7_slot: &mut f64,
        var_vl0__blk169_rdn8_slot: &mut f64,
        var_vl0__blk169_rdn9_slot: &mut f64,
        var_vl0__blk169_rv_slot: &mut f64,
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
        var_vl__blk173_rdn0_slot: &mut f64,
        var_vl__blk173_rdn1_slot: &mut f64,
        var_vl__blk173_rdn10_slot: &mut f64,
        var_vl__blk173_rdn11_slot: &mut f64,
        var_vl__blk173_rdn12_slot: &mut f64,
        var_vl__blk173_rdn13_slot: &mut f64,
        var_vl__blk173_rdn2_slot: &mut f64,
        var_vl__blk173_rdn3_slot: &mut f64,
        var_vl__blk173_rdn4_slot: &mut f64,
        var_vl__blk173_rdn5_slot: &mut f64,
        var_vl__blk173_rdn6_slot: &mut f64,
        var_vl__blk173_rdn7_slot: &mut f64,
        var_vl__blk173_rdn8_slot: &mut f64,
        var_vl__blk173_rdn9_slot: &mut f64,
        var_vl__blk173_rv_slot: &mut f64,
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
        var_vn__blk171_rdn0_slot: &mut f64,
        var_vn__blk171_rdn1_slot: &mut f64,
        var_vn__blk171_rdn10_slot: &mut f64,
        var_vn__blk171_rdn11_slot: &mut f64,
        var_vn__blk171_rdn12_slot: &mut f64,
        var_vn__blk171_rdn13_slot: &mut f64,
        var_vn__blk171_rdn2_slot: &mut f64,
        var_vn__blk171_rdn3_slot: &mut f64,
        var_vn__blk171_rdn4_slot: &mut f64,
        var_vn__blk171_rdn5_slot: &mut f64,
        var_vn__blk171_rdn6_slot: &mut f64,
        var_vn__blk171_rdn7_slot: &mut f64,
        var_vn__blk171_rdn8_slot: &mut f64,
        var_vn__blk171_rdn9_slot: &mut f64,
        var_vn__blk171_rv_slot: &mut f64,
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
        var_vnl__blk172_rdn0_slot: &mut f64,
        var_vnl__blk172_rdn1_slot: &mut f64,
        var_vnl__blk172_rdn10_slot: &mut f64,
        var_vnl__blk172_rdn11_slot: &mut f64,
        var_vnl__blk172_rdn12_slot: &mut f64,
        var_vnl__blk172_rdn13_slot: &mut f64,
        var_vnl__blk172_rdn2_slot: &mut f64,
        var_vnl__blk172_rdn3_slot: &mut f64,
        var_vnl__blk172_rdn4_slot: &mut f64,
        var_vnl__blk172_rdn5_slot: &mut f64,
        var_vnl__blk172_rdn6_slot: &mut f64,
        var_vnl__blk172_rdn7_slot: &mut f64,
        var_vnl__blk172_rdn8_slot: &mut f64,
        var_vnl__blk172_rdn9_slot: &mut f64,
        var_vnl__blk172_rv_slot: &mut f64,
    ) {
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
        let mut var_cl__blk177_rdn0: f64 = *var_cl__blk177_rdn0_slot;
        let mut var_cl__blk177_rdn1: f64 = *var_cl__blk177_rdn1_slot;
        let mut var_cl__blk177_rdn10: f64 = *var_cl__blk177_rdn10_slot;
        let mut var_cl__blk177_rdn11: f64 = *var_cl__blk177_rdn11_slot;
        let mut var_cl__blk177_rdn12: f64 = *var_cl__blk177_rdn12_slot;
        let mut var_cl__blk177_rdn13: f64 = *var_cl__blk177_rdn13_slot;
        let mut var_cl__blk177_rdn2: f64 = *var_cl__blk177_rdn2_slot;
        let mut var_cl__blk177_rdn3: f64 = *var_cl__blk177_rdn3_slot;
        let mut var_cl__blk177_rdn4: f64 = *var_cl__blk177_rdn4_slot;
        let mut var_cl__blk177_rdn5: f64 = *var_cl__blk177_rdn5_slot;
        let mut var_cl__blk177_rdn6: f64 = *var_cl__blk177_rdn6_slot;
        let mut var_cl__blk177_rdn7: f64 = *var_cl__blk177_rdn7_slot;
        let mut var_cl__blk177_rdn8: f64 = *var_cl__blk177_rdn8_slot;
        let mut var_cl__blk177_rdn9: f64 = *var_cl__blk177_rdn9_slot;
        let mut var_cl__blk177_rv: f64 = *var_cl__blk177_rv_slot;
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
        let mut var_cmx__blk176_rdn0: f64 = *var_cmx__blk176_rdn0_slot;
        let mut var_cmx__blk176_rdn1: f64 = *var_cmx__blk176_rdn1_slot;
        let mut var_cmx__blk176_rdn10: f64 = *var_cmx__blk176_rdn10_slot;
        let mut var_cmx__blk176_rdn11: f64 = *var_cmx__blk176_rdn11_slot;
        let mut var_cmx__blk176_rdn12: f64 = *var_cmx__blk176_rdn12_slot;
        let mut var_cmx__blk176_rdn13: f64 = *var_cmx__blk176_rdn13_slot;
        let mut var_cmx__blk176_rdn2: f64 = *var_cmx__blk176_rdn2_slot;
        let mut var_cmx__blk176_rdn3: f64 = *var_cmx__blk176_rdn3_slot;
        let mut var_cmx__blk176_rdn4: f64 = *var_cmx__blk176_rdn4_slot;
        let mut var_cmx__blk176_rdn5: f64 = *var_cmx__blk176_rdn5_slot;
        let mut var_cmx__blk176_rdn6: f64 = *var_cmx__blk176_rdn6_slot;
        let mut var_cmx__blk176_rdn7: f64 = *var_cmx__blk176_rdn7_slot;
        let mut var_cmx__blk176_rdn8: f64 = *var_cmx__blk176_rdn8_slot;
        let mut var_cmx__blk176_rdn9: f64 = *var_cmx__blk176_rdn9_slot;
        let mut var_cmx__blk176_rv: f64 = *var_cmx__blk176_rv_slot;
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
        let mut var_crt__blk175_rdn0: f64 = *var_crt__blk175_rdn0_slot;
        let mut var_crt__blk175_rdn1: f64 = *var_crt__blk175_rdn1_slot;
        let mut var_crt__blk175_rdn10: f64 = *var_crt__blk175_rdn10_slot;
        let mut var_crt__blk175_rdn11: f64 = *var_crt__blk175_rdn11_slot;
        let mut var_crt__blk175_rdn12: f64 = *var_crt__blk175_rdn12_slot;
        let mut var_crt__blk175_rdn13: f64 = *var_crt__blk175_rdn13_slot;
        let mut var_crt__blk175_rdn2: f64 = *var_crt__blk175_rdn2_slot;
        let mut var_crt__blk175_rdn3: f64 = *var_crt__blk175_rdn3_slot;
        let mut var_crt__blk175_rdn4: f64 = *var_crt__blk175_rdn4_slot;
        let mut var_crt__blk175_rdn5: f64 = *var_crt__blk175_rdn5_slot;
        let mut var_crt__blk175_rdn6: f64 = *var_crt__blk175_rdn6_slot;
        let mut var_crt__blk175_rdn7: f64 = *var_crt__blk175_rdn7_slot;
        let mut var_crt__blk175_rdn8: f64 = *var_crt__blk175_rdn8_slot;
        let mut var_crt__blk175_rdn9: f64 = *var_crt__blk175_rdn9_slot;
        let mut var_crt__blk175_rv: f64 = *var_crt__blk175_rv_slot;
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
        let mut var_ql__blk178_rdn0: f64 = *var_ql__blk178_rdn0_slot;
        let mut var_ql__blk178_rdn1: f64 = *var_ql__blk178_rdn1_slot;
        let mut var_ql__blk178_rdn10: f64 = *var_ql__blk178_rdn10_slot;
        let mut var_ql__blk178_rdn11: f64 = *var_ql__blk178_rdn11_slot;
        let mut var_ql__blk178_rdn12: f64 = *var_ql__blk178_rdn12_slot;
        let mut var_ql__blk178_rdn13: f64 = *var_ql__blk178_rdn13_slot;
        let mut var_ql__blk178_rdn2: f64 = *var_ql__blk178_rdn2_slot;
        let mut var_ql__blk178_rdn3: f64 = *var_ql__blk178_rdn3_slot;
        let mut var_ql__blk178_rdn4: f64 = *var_ql__blk178_rdn4_slot;
        let mut var_ql__blk178_rdn5: f64 = *var_ql__blk178_rdn5_slot;
        let mut var_ql__blk178_rdn6: f64 = *var_ql__blk178_rdn6_slot;
        let mut var_ql__blk178_rdn7: f64 = *var_ql__blk178_rdn7_slot;
        let mut var_ql__blk178_rdn8: f64 = *var_ql__blk178_rdn8_slot;
        let mut var_ql__blk178_rdn9: f64 = *var_ql__blk178_rdn9_slot;
        let mut var_ql__blk178_rv: f64 = *var_ql__blk178_rv_slot;
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
        let mut var_qlo0__blk170_rdn0: f64 = *var_qlo0__blk170_rdn0_slot;
        let mut var_qlo0__blk170_rdn1: f64 = *var_qlo0__blk170_rdn1_slot;
        let mut var_qlo0__blk170_rdn10: f64 = *var_qlo0__blk170_rdn10_slot;
        let mut var_qlo0__blk170_rdn11: f64 = *var_qlo0__blk170_rdn11_slot;
        let mut var_qlo0__blk170_rdn12: f64 = *var_qlo0__blk170_rdn12_slot;
        let mut var_qlo0__blk170_rdn13: f64 = *var_qlo0__blk170_rdn13_slot;
        let mut var_qlo0__blk170_rdn2: f64 = *var_qlo0__blk170_rdn2_slot;
        let mut var_qlo0__blk170_rdn3: f64 = *var_qlo0__blk170_rdn3_slot;
        let mut var_qlo0__blk170_rdn4: f64 = *var_qlo0__blk170_rdn4_slot;
        let mut var_qlo0__blk170_rdn5: f64 = *var_qlo0__blk170_rdn5_slot;
        let mut var_qlo0__blk170_rdn6: f64 = *var_qlo0__blk170_rdn6_slot;
        let mut var_qlo0__blk170_rdn7: f64 = *var_qlo0__blk170_rdn7_slot;
        let mut var_qlo0__blk170_rdn8: f64 = *var_qlo0__blk170_rdn8_slot;
        let mut var_qlo0__blk170_rdn9: f64 = *var_qlo0__blk170_rdn9_slot;
        let mut var_qlo0__blk170_rv: f64 = *var_qlo0__blk170_rv_slot;
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
        let mut var_qlo__blk165_rdn0: f64 = *var_qlo__blk165_rdn0_slot;
        let mut var_qlo__blk165_rdn1: f64 = *var_qlo__blk165_rdn1_slot;
        let mut var_qlo__blk165_rdn10: f64 = *var_qlo__blk165_rdn10_slot;
        let mut var_qlo__blk165_rdn11: f64 = *var_qlo__blk165_rdn11_slot;
        let mut var_qlo__blk165_rdn12: f64 = *var_qlo__blk165_rdn12_slot;
        let mut var_qlo__blk165_rdn13: f64 = *var_qlo__blk165_rdn13_slot;
        let mut var_qlo__blk165_rdn2: f64 = *var_qlo__blk165_rdn2_slot;
        let mut var_qlo__blk165_rdn3: f64 = *var_qlo__blk165_rdn3_slot;
        let mut var_qlo__blk165_rdn4: f64 = *var_qlo__blk165_rdn4_slot;
        let mut var_qlo__blk165_rdn5: f64 = *var_qlo__blk165_rdn5_slot;
        let mut var_qlo__blk165_rdn6: f64 = *var_qlo__blk165_rdn6_slot;
        let mut var_qlo__blk165_rdn7: f64 = *var_qlo__blk165_rdn7_slot;
        let mut var_qlo__blk165_rdn8: f64 = *var_qlo__blk165_rdn8_slot;
        let mut var_qlo__blk165_rdn9: f64 = *var_qlo__blk165_rdn9_slot;
        let mut var_qlo__blk165_rv: f64 = *var_qlo__blk165_rv_slot;
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
        let mut var_sel__blk174_rdn0: f64 = *var_sel__blk174_rdn0_slot;
        let mut var_sel__blk174_rdn1: f64 = *var_sel__blk174_rdn1_slot;
        let mut var_sel__blk174_rdn10: f64 = *var_sel__blk174_rdn10_slot;
        let mut var_sel__blk174_rdn11: f64 = *var_sel__blk174_rdn11_slot;
        let mut var_sel__blk174_rdn12: f64 = *var_sel__blk174_rdn12_slot;
        let mut var_sel__blk174_rdn13: f64 = *var_sel__blk174_rdn13_slot;
        let mut var_sel__blk174_rdn2: f64 = *var_sel__blk174_rdn2_slot;
        let mut var_sel__blk174_rdn3: f64 = *var_sel__blk174_rdn3_slot;
        let mut var_sel__blk174_rdn4: f64 = *var_sel__blk174_rdn4_slot;
        let mut var_sel__blk174_rdn5: f64 = *var_sel__blk174_rdn5_slot;
        let mut var_sel__blk174_rdn6: f64 = *var_sel__blk174_rdn6_slot;
        let mut var_sel__blk174_rdn7: f64 = *var_sel__blk174_rdn7_slot;
        let mut var_sel__blk174_rdn8: f64 = *var_sel__blk174_rdn8_slot;
        let mut var_sel__blk174_rdn9: f64 = *var_sel__blk174_rdn9_slot;
        let mut var_sel__blk174_rv: f64 = *var_sel__blk174_rv_slot;
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
        let mut var_vl0__blk169_rdn0: f64 = *var_vl0__blk169_rdn0_slot;
        let mut var_vl0__blk169_rdn1: f64 = *var_vl0__blk169_rdn1_slot;
        let mut var_vl0__blk169_rdn10: f64 = *var_vl0__blk169_rdn10_slot;
        let mut var_vl0__blk169_rdn11: f64 = *var_vl0__blk169_rdn11_slot;
        let mut var_vl0__blk169_rdn12: f64 = *var_vl0__blk169_rdn12_slot;
        let mut var_vl0__blk169_rdn13: f64 = *var_vl0__blk169_rdn13_slot;
        let mut var_vl0__blk169_rdn2: f64 = *var_vl0__blk169_rdn2_slot;
        let mut var_vl0__blk169_rdn3: f64 = *var_vl0__blk169_rdn3_slot;
        let mut var_vl0__blk169_rdn4: f64 = *var_vl0__blk169_rdn4_slot;
        let mut var_vl0__blk169_rdn5: f64 = *var_vl0__blk169_rdn5_slot;
        let mut var_vl0__blk169_rdn6: f64 = *var_vl0__blk169_rdn6_slot;
        let mut var_vl0__blk169_rdn7: f64 = *var_vl0__blk169_rdn7_slot;
        let mut var_vl0__blk169_rdn8: f64 = *var_vl0__blk169_rdn8_slot;
        let mut var_vl0__blk169_rdn9: f64 = *var_vl0__blk169_rdn9_slot;
        let mut var_vl0__blk169_rv: f64 = *var_vl0__blk169_rv_slot;
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
        let mut var_vl__blk173_rdn0: f64 = *var_vl__blk173_rdn0_slot;
        let mut var_vl__blk173_rdn1: f64 = *var_vl__blk173_rdn1_slot;
        let mut var_vl__blk173_rdn10: f64 = *var_vl__blk173_rdn10_slot;
        let mut var_vl__blk173_rdn11: f64 = *var_vl__blk173_rdn11_slot;
        let mut var_vl__blk173_rdn12: f64 = *var_vl__blk173_rdn12_slot;
        let mut var_vl__blk173_rdn13: f64 = *var_vl__blk173_rdn13_slot;
        let mut var_vl__blk173_rdn2: f64 = *var_vl__blk173_rdn2_slot;
        let mut var_vl__blk173_rdn3: f64 = *var_vl__blk173_rdn3_slot;
        let mut var_vl__blk173_rdn4: f64 = *var_vl__blk173_rdn4_slot;
        let mut var_vl__blk173_rdn5: f64 = *var_vl__blk173_rdn5_slot;
        let mut var_vl__blk173_rdn6: f64 = *var_vl__blk173_rdn6_slot;
        let mut var_vl__blk173_rdn7: f64 = *var_vl__blk173_rdn7_slot;
        let mut var_vl__blk173_rdn8: f64 = *var_vl__blk173_rdn8_slot;
        let mut var_vl__blk173_rdn9: f64 = *var_vl__blk173_rdn9_slot;
        let mut var_vl__blk173_rv: f64 = *var_vl__blk173_rv_slot;
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
        let mut var_vn__blk171_rdn0: f64 = *var_vn__blk171_rdn0_slot;
        let mut var_vn__blk171_rdn1: f64 = *var_vn__blk171_rdn1_slot;
        let mut var_vn__blk171_rdn10: f64 = *var_vn__blk171_rdn10_slot;
        let mut var_vn__blk171_rdn11: f64 = *var_vn__blk171_rdn11_slot;
        let mut var_vn__blk171_rdn12: f64 = *var_vn__blk171_rdn12_slot;
        let mut var_vn__blk171_rdn13: f64 = *var_vn__blk171_rdn13_slot;
        let mut var_vn__blk171_rdn2: f64 = *var_vn__blk171_rdn2_slot;
        let mut var_vn__blk171_rdn3: f64 = *var_vn__blk171_rdn3_slot;
        let mut var_vn__blk171_rdn4: f64 = *var_vn__blk171_rdn4_slot;
        let mut var_vn__blk171_rdn5: f64 = *var_vn__blk171_rdn5_slot;
        let mut var_vn__blk171_rdn6: f64 = *var_vn__blk171_rdn6_slot;
        let mut var_vn__blk171_rdn7: f64 = *var_vn__blk171_rdn7_slot;
        let mut var_vn__blk171_rdn8: f64 = *var_vn__blk171_rdn8_slot;
        let mut var_vn__blk171_rdn9: f64 = *var_vn__blk171_rdn9_slot;
        let mut var_vn__blk171_rv: f64 = *var_vn__blk171_rv_slot;
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
        let mut var_vnl__blk172_rdn0: f64 = *var_vnl__blk172_rdn0_slot;
        let mut var_vnl__blk172_rdn1: f64 = *var_vnl__blk172_rdn1_slot;
        let mut var_vnl__blk172_rdn10: f64 = *var_vnl__blk172_rdn10_slot;
        let mut var_vnl__blk172_rdn11: f64 = *var_vnl__blk172_rdn11_slot;
        let mut var_vnl__blk172_rdn12: f64 = *var_vnl__blk172_rdn12_slot;
        let mut var_vnl__blk172_rdn13: f64 = *var_vnl__blk172_rdn13_slot;
        let mut var_vnl__blk172_rdn2: f64 = *var_vnl__blk172_rdn2_slot;
        let mut var_vnl__blk172_rdn3: f64 = *var_vnl__blk172_rdn3_slot;
        let mut var_vnl__blk172_rdn4: f64 = *var_vnl__blk172_rdn4_slot;
        let mut var_vnl__blk172_rdn5: f64 = *var_vnl__blk172_rdn5_slot;
        let mut var_vnl__blk172_rdn6: f64 = *var_vnl__blk172_rdn6_slot;
        let mut var_vnl__blk172_rdn7: f64 = *var_vnl__blk172_rdn7_slot;
        let mut var_vnl__blk172_rdn8: f64 = *var_vnl__blk172_rdn8_slot;
        let mut var_vnl__blk172_rdn9: f64 = *var_vnl__blk172_rdn9_slot;
        let mut var_vnl__blk172_rv: f64 = *var_vnl__blk172_rv_slot;

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
        var_vl0__blk169_rv = 0.0;
        var_vl0__blk169_rdn0 = 0.0;
        var_vl0__blk169_rdn1 = 0.0;
        var_vl0__blk169_rdn2 = 0.0;
        var_vl0__blk169_rdn3 = 0.0;
        var_vl0__blk169_rdn4 = 0.0;
        var_vl0__blk169_rdn5 = 0.0;
        var_vl0__blk169_rdn6 = 0.0;
        var_vl0__blk169_rdn7 = 0.0;
        var_vl0__blk169_rdn8 = 0.0;
        var_vl0__blk169_rdn9 = 0.0;
        var_vl0__blk169_rdn10 = 0.0;
        var_vl0__blk169_rdn11 = 0.0;
        var_vl0__blk169_rdn12 = 0.0;
        var_vl0__blk169_rdn13 = 0.0;

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
        var_qlo0__blk170_rv = 0.0;
        var_qlo0__blk170_rdn0 = 0.0;
        var_qlo0__blk170_rdn1 = 0.0;
        var_qlo0__blk170_rdn2 = 0.0;
        var_qlo0__blk170_rdn3 = 0.0;
        var_qlo0__blk170_rdn4 = 0.0;
        var_qlo0__blk170_rdn5 = 0.0;
        var_qlo0__blk170_rdn6 = 0.0;
        var_qlo0__blk170_rdn7 = 0.0;
        var_qlo0__blk170_rdn8 = 0.0;
        var_qlo0__blk170_rdn9 = 0.0;
        var_qlo0__blk170_rdn10 = 0.0;
        var_qlo0__blk170_rdn11 = 0.0;
        var_qlo0__blk170_rdn12 = 0.0;
        var_qlo0__blk170_rdn13 = 0.0;

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
        var_vn__blk171_rv = 0.0;
        var_vn__blk171_rdn0 = 0.0;
        var_vn__blk171_rdn1 = 0.0;
        var_vn__blk171_rdn2 = 0.0;
        var_vn__blk171_rdn3 = 0.0;
        var_vn__blk171_rdn4 = 0.0;
        var_vn__blk171_rdn5 = 0.0;
        var_vn__blk171_rdn6 = 0.0;
        var_vn__blk171_rdn7 = 0.0;
        var_vn__blk171_rdn8 = 0.0;
        var_vn__blk171_rdn9 = 0.0;
        var_vn__blk171_rdn10 = 0.0;
        var_vn__blk171_rdn11 = 0.0;
        var_vn__blk171_rdn12 = 0.0;
        var_vn__blk171_rdn13 = 0.0;

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
        var_vnl__blk172_rv = 0.0;
        var_vnl__blk172_rdn0 = 0.0;
        var_vnl__blk172_rdn1 = 0.0;
        var_vnl__blk172_rdn2 = 0.0;
        var_vnl__blk172_rdn3 = 0.0;
        var_vnl__blk172_rdn4 = 0.0;
        var_vnl__blk172_rdn5 = 0.0;
        var_vnl__blk172_rdn6 = 0.0;
        var_vnl__blk172_rdn7 = 0.0;
        var_vnl__blk172_rdn8 = 0.0;
        var_vnl__blk172_rdn9 = 0.0;
        var_vnl__blk172_rdn10 = 0.0;
        var_vnl__blk172_rdn11 = 0.0;
        var_vnl__blk172_rdn12 = 0.0;
        var_vnl__blk172_rdn13 = 0.0;

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
        var_vl__blk173_rv = 0.0;
        var_vl__blk173_rdn0 = 0.0;
        var_vl__blk173_rdn1 = 0.0;
        var_vl__blk173_rdn2 = 0.0;
        var_vl__blk173_rdn3 = 0.0;
        var_vl__blk173_rdn4 = 0.0;
        var_vl__blk173_rdn5 = 0.0;
        var_vl__blk173_rdn6 = 0.0;
        var_vl__blk173_rdn7 = 0.0;
        var_vl__blk173_rdn8 = 0.0;
        var_vl__blk173_rdn9 = 0.0;
        var_vl__blk173_rdn10 = 0.0;
        var_vl__blk173_rdn11 = 0.0;
        var_vl__blk173_rdn12 = 0.0;
        var_vl__blk173_rdn13 = 0.0;

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
        var_qlo__blk165_rv = 0.0;
        var_qlo__blk165_rdn0 = 0.0;
        var_qlo__blk165_rdn1 = 0.0;
        var_qlo__blk165_rdn2 = 0.0;
        var_qlo__blk165_rdn3 = 0.0;
        var_qlo__blk165_rdn4 = 0.0;
        var_qlo__blk165_rdn5 = 0.0;
        var_qlo__blk165_rdn6 = 0.0;
        var_qlo__blk165_rdn7 = 0.0;
        var_qlo__blk165_rdn8 = 0.0;
        var_qlo__blk165_rdn9 = 0.0;
        var_qlo__blk165_rdn10 = 0.0;
        var_qlo__blk165_rdn11 = 0.0;
        var_qlo__blk165_rdn12 = 0.0;
        var_qlo__blk165_rdn13 = 0.0;

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
        var_sel__blk174_rv = 0.0;
        var_sel__blk174_rdn0 = 0.0;
        var_sel__blk174_rdn1 = 0.0;
        var_sel__blk174_rdn2 = 0.0;
        var_sel__blk174_rdn3 = 0.0;
        var_sel__blk174_rdn4 = 0.0;
        var_sel__blk174_rdn5 = 0.0;
        var_sel__blk174_rdn6 = 0.0;
        var_sel__blk174_rdn7 = 0.0;
        var_sel__blk174_rdn8 = 0.0;
        var_sel__blk174_rdn9 = 0.0;
        var_sel__blk174_rdn10 = 0.0;
        var_sel__blk174_rdn11 = 0.0;
        var_sel__blk174_rdn12 = 0.0;
        var_sel__blk174_rdn13 = 0.0;

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
        var_crt__blk175_rv = 0.0;
        var_crt__blk175_rdn0 = 0.0;
        var_crt__blk175_rdn1 = 0.0;
        var_crt__blk175_rdn2 = 0.0;
        var_crt__blk175_rdn3 = 0.0;
        var_crt__blk175_rdn4 = 0.0;
        var_crt__blk175_rdn5 = 0.0;
        var_crt__blk175_rdn6 = 0.0;
        var_crt__blk175_rdn7 = 0.0;
        var_crt__blk175_rdn8 = 0.0;
        var_crt__blk175_rdn9 = 0.0;
        var_crt__blk175_rdn10 = 0.0;
        var_crt__blk175_rdn11 = 0.0;
        var_crt__blk175_rdn12 = 0.0;
        var_crt__blk175_rdn13 = 0.0;

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
        var_cmx__blk176_rv = 0.0;
        var_cmx__blk176_rdn0 = 0.0;
        var_cmx__blk176_rdn1 = 0.0;
        var_cmx__blk176_rdn2 = 0.0;
        var_cmx__blk176_rdn3 = 0.0;
        var_cmx__blk176_rdn4 = 0.0;
        var_cmx__blk176_rdn5 = 0.0;
        var_cmx__blk176_rdn6 = 0.0;
        var_cmx__blk176_rdn7 = 0.0;
        var_cmx__blk176_rdn8 = 0.0;
        var_cmx__blk176_rdn9 = 0.0;
        var_cmx__blk176_rdn10 = 0.0;
        var_cmx__blk176_rdn11 = 0.0;
        var_cmx__blk176_rdn12 = 0.0;
        var_cmx__blk176_rdn13 = 0.0;

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
        var_cl__blk177_rv = 0.0;
        var_cl__blk177_rdn0 = 0.0;
        var_cl__blk177_rdn1 = 0.0;
        var_cl__blk177_rdn2 = 0.0;
        var_cl__blk177_rdn3 = 0.0;
        var_cl__blk177_rdn4 = 0.0;
        var_cl__blk177_rdn5 = 0.0;
        var_cl__blk177_rdn6 = 0.0;
        var_cl__blk177_rdn7 = 0.0;
        var_cl__blk177_rdn8 = 0.0;
        var_cl__blk177_rdn9 = 0.0;
        var_cl__blk177_rdn10 = 0.0;
        var_cl__blk177_rdn11 = 0.0;
        var_cl__blk177_rdn12 = 0.0;
        var_cl__blk177_rdn13 = 0.0;

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
        var_ql__blk178_rv = 0.0;
        var_ql__blk178_rdn0 = 0.0;
        var_ql__blk178_rdn1 = 0.0;
        var_ql__blk178_rdn2 = 0.0;
        var_ql__blk178_rdn3 = 0.0;
        var_ql__blk178_rdn4 = 0.0;
        var_ql__blk178_rdn5 = 0.0;
        var_ql__blk178_rdn6 = 0.0;
        var_ql__blk178_rdn7 = 0.0;
        var_ql__blk178_rdn8 = 0.0;
        var_ql__blk178_rdn9 = 0.0;
        var_ql__blk178_rdn10 = 0.0;
        var_ql__blk178_rdn11 = 0.0;
        var_ql__blk178_rdn12 = 0.0;
        var_ql__blk178_rdn13 = 0.0;


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
        *var_cl__blk177_rdn0_slot = var_cl__blk177_rdn0;
        *var_cl__blk177_rdn1_slot = var_cl__blk177_rdn1;
        *var_cl__blk177_rdn10_slot = var_cl__blk177_rdn10;
        *var_cl__blk177_rdn11_slot = var_cl__blk177_rdn11;
        *var_cl__blk177_rdn12_slot = var_cl__blk177_rdn12;
        *var_cl__blk177_rdn13_slot = var_cl__blk177_rdn13;
        *var_cl__blk177_rdn2_slot = var_cl__blk177_rdn2;
        *var_cl__blk177_rdn3_slot = var_cl__blk177_rdn3;
        *var_cl__blk177_rdn4_slot = var_cl__blk177_rdn4;
        *var_cl__blk177_rdn5_slot = var_cl__blk177_rdn5;
        *var_cl__blk177_rdn6_slot = var_cl__blk177_rdn6;
        *var_cl__blk177_rdn7_slot = var_cl__blk177_rdn7;
        *var_cl__blk177_rdn8_slot = var_cl__blk177_rdn8;
        *var_cl__blk177_rdn9_slot = var_cl__blk177_rdn9;
        *var_cl__blk177_rv_slot = var_cl__blk177_rv;
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
        *var_cmx__blk176_rdn0_slot = var_cmx__blk176_rdn0;
        *var_cmx__blk176_rdn1_slot = var_cmx__blk176_rdn1;
        *var_cmx__blk176_rdn10_slot = var_cmx__blk176_rdn10;
        *var_cmx__blk176_rdn11_slot = var_cmx__blk176_rdn11;
        *var_cmx__blk176_rdn12_slot = var_cmx__blk176_rdn12;
        *var_cmx__blk176_rdn13_slot = var_cmx__blk176_rdn13;
        *var_cmx__blk176_rdn2_slot = var_cmx__blk176_rdn2;
        *var_cmx__blk176_rdn3_slot = var_cmx__blk176_rdn3;
        *var_cmx__blk176_rdn4_slot = var_cmx__blk176_rdn4;
        *var_cmx__blk176_rdn5_slot = var_cmx__blk176_rdn5;
        *var_cmx__blk176_rdn6_slot = var_cmx__blk176_rdn6;
        *var_cmx__blk176_rdn7_slot = var_cmx__blk176_rdn7;
        *var_cmx__blk176_rdn8_slot = var_cmx__blk176_rdn8;
        *var_cmx__blk176_rdn9_slot = var_cmx__blk176_rdn9;
        *var_cmx__blk176_rv_slot = var_cmx__blk176_rv;
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
        *var_crt__blk175_rdn0_slot = var_crt__blk175_rdn0;
        *var_crt__blk175_rdn1_slot = var_crt__blk175_rdn1;
        *var_crt__blk175_rdn10_slot = var_crt__blk175_rdn10;
        *var_crt__blk175_rdn11_slot = var_crt__blk175_rdn11;
        *var_crt__blk175_rdn12_slot = var_crt__blk175_rdn12;
        *var_crt__blk175_rdn13_slot = var_crt__blk175_rdn13;
        *var_crt__blk175_rdn2_slot = var_crt__blk175_rdn2;
        *var_crt__blk175_rdn3_slot = var_crt__blk175_rdn3;
        *var_crt__blk175_rdn4_slot = var_crt__blk175_rdn4;
        *var_crt__blk175_rdn5_slot = var_crt__blk175_rdn5;
        *var_crt__blk175_rdn6_slot = var_crt__blk175_rdn6;
        *var_crt__blk175_rdn7_slot = var_crt__blk175_rdn7;
        *var_crt__blk175_rdn8_slot = var_crt__blk175_rdn8;
        *var_crt__blk175_rdn9_slot = var_crt__blk175_rdn9;
        *var_crt__blk175_rv_slot = var_crt__blk175_rv;
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
        *var_ql__blk178_rdn0_slot = var_ql__blk178_rdn0;
        *var_ql__blk178_rdn1_slot = var_ql__blk178_rdn1;
        *var_ql__blk178_rdn10_slot = var_ql__blk178_rdn10;
        *var_ql__blk178_rdn11_slot = var_ql__blk178_rdn11;
        *var_ql__blk178_rdn12_slot = var_ql__blk178_rdn12;
        *var_ql__blk178_rdn13_slot = var_ql__blk178_rdn13;
        *var_ql__blk178_rdn2_slot = var_ql__blk178_rdn2;
        *var_ql__blk178_rdn3_slot = var_ql__blk178_rdn3;
        *var_ql__blk178_rdn4_slot = var_ql__blk178_rdn4;
        *var_ql__blk178_rdn5_slot = var_ql__blk178_rdn5;
        *var_ql__blk178_rdn6_slot = var_ql__blk178_rdn6;
        *var_ql__blk178_rdn7_slot = var_ql__blk178_rdn7;
        *var_ql__blk178_rdn8_slot = var_ql__blk178_rdn8;
        *var_ql__blk178_rdn9_slot = var_ql__blk178_rdn9;
        *var_ql__blk178_rv_slot = var_ql__blk178_rv;
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
        *var_qlo0__blk170_rdn0_slot = var_qlo0__blk170_rdn0;
        *var_qlo0__blk170_rdn1_slot = var_qlo0__blk170_rdn1;
        *var_qlo0__blk170_rdn10_slot = var_qlo0__blk170_rdn10;
        *var_qlo0__blk170_rdn11_slot = var_qlo0__blk170_rdn11;
        *var_qlo0__blk170_rdn12_slot = var_qlo0__blk170_rdn12;
        *var_qlo0__blk170_rdn13_slot = var_qlo0__blk170_rdn13;
        *var_qlo0__blk170_rdn2_slot = var_qlo0__blk170_rdn2;
        *var_qlo0__blk170_rdn3_slot = var_qlo0__blk170_rdn3;
        *var_qlo0__blk170_rdn4_slot = var_qlo0__blk170_rdn4;
        *var_qlo0__blk170_rdn5_slot = var_qlo0__blk170_rdn5;
        *var_qlo0__blk170_rdn6_slot = var_qlo0__blk170_rdn6;
        *var_qlo0__blk170_rdn7_slot = var_qlo0__blk170_rdn7;
        *var_qlo0__blk170_rdn8_slot = var_qlo0__blk170_rdn8;
        *var_qlo0__blk170_rdn9_slot = var_qlo0__blk170_rdn9;
        *var_qlo0__blk170_rv_slot = var_qlo0__blk170_rv;
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
        *var_qlo__blk165_rdn0_slot = var_qlo__blk165_rdn0;
        *var_qlo__blk165_rdn1_slot = var_qlo__blk165_rdn1;
        *var_qlo__blk165_rdn10_slot = var_qlo__blk165_rdn10;
        *var_qlo__blk165_rdn11_slot = var_qlo__blk165_rdn11;
        *var_qlo__blk165_rdn12_slot = var_qlo__blk165_rdn12;
        *var_qlo__blk165_rdn13_slot = var_qlo__blk165_rdn13;
        *var_qlo__blk165_rdn2_slot = var_qlo__blk165_rdn2;
        *var_qlo__blk165_rdn3_slot = var_qlo__blk165_rdn3;
        *var_qlo__blk165_rdn4_slot = var_qlo__blk165_rdn4;
        *var_qlo__blk165_rdn5_slot = var_qlo__blk165_rdn5;
        *var_qlo__blk165_rdn6_slot = var_qlo__blk165_rdn6;
        *var_qlo__blk165_rdn7_slot = var_qlo__blk165_rdn7;
        *var_qlo__blk165_rdn8_slot = var_qlo__blk165_rdn8;
        *var_qlo__blk165_rdn9_slot = var_qlo__blk165_rdn9;
        *var_qlo__blk165_rv_slot = var_qlo__blk165_rv;
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
        *var_sel__blk174_rdn0_slot = var_sel__blk174_rdn0;
        *var_sel__blk174_rdn1_slot = var_sel__blk174_rdn1;
        *var_sel__blk174_rdn10_slot = var_sel__blk174_rdn10;
        *var_sel__blk174_rdn11_slot = var_sel__blk174_rdn11;
        *var_sel__blk174_rdn12_slot = var_sel__blk174_rdn12;
        *var_sel__blk174_rdn13_slot = var_sel__blk174_rdn13;
        *var_sel__blk174_rdn2_slot = var_sel__blk174_rdn2;
        *var_sel__blk174_rdn3_slot = var_sel__blk174_rdn3;
        *var_sel__blk174_rdn4_slot = var_sel__blk174_rdn4;
        *var_sel__blk174_rdn5_slot = var_sel__blk174_rdn5;
        *var_sel__blk174_rdn6_slot = var_sel__blk174_rdn6;
        *var_sel__blk174_rdn7_slot = var_sel__blk174_rdn7;
        *var_sel__blk174_rdn8_slot = var_sel__blk174_rdn8;
        *var_sel__blk174_rdn9_slot = var_sel__blk174_rdn9;
        *var_sel__blk174_rv_slot = var_sel__blk174_rv;
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
        *var_vl0__blk169_rdn0_slot = var_vl0__blk169_rdn0;
        *var_vl0__blk169_rdn1_slot = var_vl0__blk169_rdn1;
        *var_vl0__blk169_rdn10_slot = var_vl0__blk169_rdn10;
        *var_vl0__blk169_rdn11_slot = var_vl0__blk169_rdn11;
        *var_vl0__blk169_rdn12_slot = var_vl0__blk169_rdn12;
        *var_vl0__blk169_rdn13_slot = var_vl0__blk169_rdn13;
        *var_vl0__blk169_rdn2_slot = var_vl0__blk169_rdn2;
        *var_vl0__blk169_rdn3_slot = var_vl0__blk169_rdn3;
        *var_vl0__blk169_rdn4_slot = var_vl0__blk169_rdn4;
        *var_vl0__blk169_rdn5_slot = var_vl0__blk169_rdn5;
        *var_vl0__blk169_rdn6_slot = var_vl0__blk169_rdn6;
        *var_vl0__blk169_rdn7_slot = var_vl0__blk169_rdn7;
        *var_vl0__blk169_rdn8_slot = var_vl0__blk169_rdn8;
        *var_vl0__blk169_rdn9_slot = var_vl0__blk169_rdn9;
        *var_vl0__blk169_rv_slot = var_vl0__blk169_rv;
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
        *var_vl__blk173_rdn0_slot = var_vl__blk173_rdn0;
        *var_vl__blk173_rdn1_slot = var_vl__blk173_rdn1;
        *var_vl__blk173_rdn10_slot = var_vl__blk173_rdn10;
        *var_vl__blk173_rdn11_slot = var_vl__blk173_rdn11;
        *var_vl__blk173_rdn12_slot = var_vl__blk173_rdn12;
        *var_vl__blk173_rdn13_slot = var_vl__blk173_rdn13;
        *var_vl__blk173_rdn2_slot = var_vl__blk173_rdn2;
        *var_vl__blk173_rdn3_slot = var_vl__blk173_rdn3;
        *var_vl__blk173_rdn4_slot = var_vl__blk173_rdn4;
        *var_vl__blk173_rdn5_slot = var_vl__blk173_rdn5;
        *var_vl__blk173_rdn6_slot = var_vl__blk173_rdn6;
        *var_vl__blk173_rdn7_slot = var_vl__blk173_rdn7;
        *var_vl__blk173_rdn8_slot = var_vl__blk173_rdn8;
        *var_vl__blk173_rdn9_slot = var_vl__blk173_rdn9;
        *var_vl__blk173_rv_slot = var_vl__blk173_rv;
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
        *var_vn__blk171_rdn0_slot = var_vn__blk171_rdn0;
        *var_vn__blk171_rdn1_slot = var_vn__blk171_rdn1;
        *var_vn__blk171_rdn10_slot = var_vn__blk171_rdn10;
        *var_vn__blk171_rdn11_slot = var_vn__blk171_rdn11;
        *var_vn__blk171_rdn12_slot = var_vn__blk171_rdn12;
        *var_vn__blk171_rdn13_slot = var_vn__blk171_rdn13;
        *var_vn__blk171_rdn2_slot = var_vn__blk171_rdn2;
        *var_vn__blk171_rdn3_slot = var_vn__blk171_rdn3;
        *var_vn__blk171_rdn4_slot = var_vn__blk171_rdn4;
        *var_vn__blk171_rdn5_slot = var_vn__blk171_rdn5;
        *var_vn__blk171_rdn6_slot = var_vn__blk171_rdn6;
        *var_vn__blk171_rdn7_slot = var_vn__blk171_rdn7;
        *var_vn__blk171_rdn8_slot = var_vn__blk171_rdn8;
        *var_vn__blk171_rdn9_slot = var_vn__blk171_rdn9;
        *var_vn__blk171_rv_slot = var_vn__blk171_rv;
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
        *var_vnl__blk172_rdn0_slot = var_vnl__blk172_rdn0;
        *var_vnl__blk172_rdn1_slot = var_vnl__blk172_rdn1;
        *var_vnl__blk172_rdn10_slot = var_vnl__blk172_rdn10;
        *var_vnl__blk172_rdn11_slot = var_vnl__blk172_rdn11;
        *var_vnl__blk172_rdn12_slot = var_vnl__blk172_rdn12;
        *var_vnl__blk172_rdn13_slot = var_vnl__blk172_rdn13;
        *var_vnl__blk172_rdn2_slot = var_vnl__blk172_rdn2;
        *var_vnl__blk172_rdn3_slot = var_vnl__blk172_rdn3;
        *var_vnl__blk172_rdn4_slot = var_vnl__blk172_rdn4;
        *var_vnl__blk172_rdn5_slot = var_vnl__blk172_rdn5;
        *var_vnl__blk172_rdn6_slot = var_vnl__blk172_rdn6;
        *var_vnl__blk172_rdn7_slot = var_vnl__blk172_rdn7;
        *var_vnl__blk172_rdn8_slot = var_vnl__blk172_rdn8;
        *var_vnl__blk172_rdn9_slot = var_vnl__blk172_rdn9;
        *var_vnl__blk172_rv_slot = var_vnl__blk172_rv;
    }

    pub(super) fn stamp_reactive_block_27(
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
        var_iitf_dn0: f64,
        var_iitf_dn1: f64,
        var_iitf_dn10: f64,
        var_iitf_dn11: f64,
        var_iitf_dn12: f64,
        var_iitf_dn13: f64,
        var_iitf_dn2: f64,
        var_iitf_dn3: f64,
        var_iitf_dn4: f64,
        var_iitf_dn5: f64,
        var_iitf_dn6: f64,
        var_iitf_dn7: f64,
        var_iitf_dn8: f64,
        var_iitf_dn9: f64,
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
        var_ql__blk178: f64,
        var_ql__blk178_dn0: f64,
        var_ql__blk178_dn1: f64,
        var_ql__blk178_dn10: f64,
        var_ql__blk178_dn11: f64,
        var_ql__blk178_dn12: f64,
        var_ql__blk178_dn13: f64,
        var_ql__blk178_dn2: f64,
        var_ql__blk178_dn3: f64,
        var_ql__blk178_dn4: f64,
        var_ql__blk178_dn5: f64,
        var_ql__blk178_dn6: f64,
        var_ql__blk178_dn7: f64,
        var_ql__blk178_dn8: f64,
        var_ql__blk178_dn9: f64,
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
        var_dv__blk181_rdn0_slot: &mut f64,
        var_dv__blk181_rdn1_slot: &mut f64,
        var_dv__blk181_rdn10_slot: &mut f64,
        var_dv__blk181_rdn11_slot: &mut f64,
        var_dv__blk181_rdn12_slot: &mut f64,
        var_dv__blk181_rdn13_slot: &mut f64,
        var_dv__blk181_rdn2_slot: &mut f64,
        var_dv__blk181_rdn3_slot: &mut f64,
        var_dv__blk181_rdn4_slot: &mut f64,
        var_dv__blk181_rdn5_slot: &mut f64,
        var_dv__blk181_rdn6_slot: &mut f64,
        var_dv__blk181_rdn7_slot: &mut f64,
        var_dv__blk181_rdn8_slot: &mut f64,
        var_dv__blk181_rdn9_slot: &mut f64,
        var_dv__blk181_rv_slot: &mut f64,
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
        var_mif_rdn0_slot: &mut f64,
        var_mif_rdn1_slot: &mut f64,
        var_mif_rdn10_slot: &mut f64,
        var_mif_rdn11_slot: &mut f64,
        var_mif_rdn12_slot: &mut f64,
        var_mif_rdn13_slot: &mut f64,
        var_mif_rdn2_slot: &mut f64,
        var_mif_rdn3_slot: &mut f64,
        var_mif_rdn4_slot: &mut f64,
        var_mif_rdn5_slot: &mut f64,
        var_mif_rdn6_slot: &mut f64,
        var_mif_rdn7_slot: &mut f64,
        var_mif_rdn8_slot: &mut f64,
        var_mif_rdn9_slot: &mut f64,
        var_mif_rv_slot: &mut f64,
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
        var_mv0__blk179_rdn0_slot: &mut f64,
        var_mv0__blk179_rdn1_slot: &mut f64,
        var_mv0__blk179_rdn10_slot: &mut f64,
        var_mv0__blk179_rdn11_slot: &mut f64,
        var_mv0__blk179_rdn12_slot: &mut f64,
        var_mv0__blk179_rdn13_slot: &mut f64,
        var_mv0__blk179_rdn2_slot: &mut f64,
        var_mv0__blk179_rdn3_slot: &mut f64,
        var_mv0__blk179_rdn4_slot: &mut f64,
        var_mv0__blk179_rdn5_slot: &mut f64,
        var_mv0__blk179_rdn6_slot: &mut f64,
        var_mv0__blk179_rdn7_slot: &mut f64,
        var_mv0__blk179_rdn8_slot: &mut f64,
        var_mv0__blk179_rdn9_slot: &mut f64,
        var_mv0__blk179_rv_slot: &mut f64,
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
        var_mv__blk182_rdn0_slot: &mut f64,
        var_mv__blk182_rdn1_slot: &mut f64,
        var_mv__blk182_rdn10_slot: &mut f64,
        var_mv__blk182_rdn11_slot: &mut f64,
        var_mv__blk182_rdn12_slot: &mut f64,
        var_mv__blk182_rdn13_slot: &mut f64,
        var_mv__blk182_rdn2_slot: &mut f64,
        var_mv__blk182_rdn3_slot: &mut f64,
        var_mv__blk182_rdn4_slot: &mut f64,
        var_mv__blk182_rdn5_slot: &mut f64,
        var_mv__blk182_rdn6_slot: &mut f64,
        var_mv__blk182_rdn7_slot: &mut f64,
        var_mv__blk182_rdn8_slot: &mut f64,
        var_mv__blk182_rdn9_slot: &mut f64,
        var_mv__blk182_rv_slot: &mut f64,
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
        var_q0__blk180_rdn0_slot: &mut f64,
        var_q0__blk180_rdn1_slot: &mut f64,
        var_q0__blk180_rdn10_slot: &mut f64,
        var_q0__blk180_rdn11_slot: &mut f64,
        var_q0__blk180_rdn12_slot: &mut f64,
        var_q0__blk180_rdn13_slot: &mut f64,
        var_q0__blk180_rdn2_slot: &mut f64,
        var_q0__blk180_rdn3_slot: &mut f64,
        var_q0__blk180_rdn4_slot: &mut f64,
        var_q0__blk180_rdn5_slot: &mut f64,
        var_q0__blk180_rdn6_slot: &mut f64,
        var_q0__blk180_rdn7_slot: &mut f64,
        var_q0__blk180_rdn8_slot: &mut f64,
        var_q0__blk180_rdn9_slot: &mut f64,
        var_q0__blk180_rv_slot: &mut f64,
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
        var_qdbep_rdn0_slot: &mut f64,
        var_qdbep_rdn1_slot: &mut f64,
        var_qdbep_rdn10_slot: &mut f64,
        var_qdbep_rdn11_slot: &mut f64,
        var_qdbep_rdn12_slot: &mut f64,
        var_qdbep_rdn13_slot: &mut f64,
        var_qdbep_rdn2_slot: &mut f64,
        var_qdbep_rdn3_slot: &mut f64,
        var_qdbep_rdn4_slot: &mut f64,
        var_qdbep_rdn5_slot: &mut f64,
        var_qdbep_rdn6_slot: &mut f64,
        var_qdbep_rdn7_slot: &mut f64,
        var_qdbep_rdn8_slot: &mut f64,
        var_qdbep_rdn9_slot: &mut f64,
        var_qdbep_rv_slot: &mut f64,
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
        var_qlo__blk165_rdn0_slot: &mut f64,
        var_qlo__blk165_rdn1_slot: &mut f64,
        var_qlo__blk165_rdn10_slot: &mut f64,
        var_qlo__blk165_rdn11_slot: &mut f64,
        var_qlo__blk165_rdn12_slot: &mut f64,
        var_qlo__blk165_rdn13_slot: &mut f64,
        var_qlo__blk165_rdn2_slot: &mut f64,
        var_qlo__blk165_rdn3_slot: &mut f64,
        var_qlo__blk165_rdn4_slot: &mut f64,
        var_qlo__blk165_rdn5_slot: &mut f64,
        var_qlo__blk165_rdn6_slot: &mut f64,
        var_qlo__blk165_rdn7_slot: &mut f64,
        var_qlo__blk165_rdn8_slot: &mut f64,
        var_qlo__blk165_rdn9_slot: &mut f64,
        var_qlo__blk165_rv_slot: &mut f64,
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
        var_rif_rdn0_slot: &mut f64,
        var_rif_rdn1_slot: &mut f64,
        var_rif_rdn10_slot: &mut f64,
        var_rif_rdn11_slot: &mut f64,
        var_rif_rdn12_slot: &mut f64,
        var_rif_rdn13_slot: &mut f64,
        var_rif_rdn2_slot: &mut f64,
        var_rif_rdn3_slot: &mut f64,
        var_rif_rdn4_slot: &mut f64,
        var_rif_rdn5_slot: &mut f64,
        var_rif_rdn6_slot: &mut f64,
        var_rif_rdn7_slot: &mut f64,
        var_rif_rdn8_slot: &mut f64,
        var_rif_rdn9_slot: &mut f64,
        var_rif_rv_slot: &mut f64,
        var_sgif_slot: &mut f64,
        var_sgif_dn0_slot: &mut f64,
        var_sgif_dn1_slot: &mut f64,
        var_sgif_dn10_slot: &mut f64,
        var_sgif_dn11_slot: &mut f64,
        var_sgif_dn12_slot: &mut f64,
        var_sgif_dn13_slot: &mut f64,
        var_sgif_dn2_slot: &mut f64,
        var_sgif_dn3_slot: &mut f64,
        var_sgif_dn4_slot: &mut f64,
        var_sgif_dn5_slot: &mut f64,
        var_sgif_dn6_slot: &mut f64,
        var_sgif_dn7_slot: &mut f64,
        var_sgif_dn8_slot: &mut f64,
        var_sgif_dn9_slot: &mut f64,
        var_sgif_rdn0_slot: &mut f64,
        var_sgif_rdn1_slot: &mut f64,
        var_sgif_rdn10_slot: &mut f64,
        var_sgif_rdn11_slot: &mut f64,
        var_sgif_rdn12_slot: &mut f64,
        var_sgif_rdn13_slot: &mut f64,
        var_sgif_rdn2_slot: &mut f64,
        var_sgif_rdn3_slot: &mut f64,
        var_sgif_rdn4_slot: &mut f64,
        var_sgif_rdn5_slot: &mut f64,
        var_sgif_rdn6_slot: &mut f64,
        var_sgif_rdn7_slot: &mut f64,
        var_sgif_rdn8_slot: &mut f64,
        var_sgif_rdn9_slot: &mut f64,
        var_sgif_rv_slot: &mut f64,
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
        var_vl0__blk169_rdn0_slot: &mut f64,
        var_vl0__blk169_rdn1_slot: &mut f64,
        var_vl0__blk169_rdn10_slot: &mut f64,
        var_vl0__blk169_rdn11_slot: &mut f64,
        var_vl0__blk169_rdn12_slot: &mut f64,
        var_vl0__blk169_rdn13_slot: &mut f64,
        var_vl0__blk169_rdn2_slot: &mut f64,
        var_vl0__blk169_rdn3_slot: &mut f64,
        var_vl0__blk169_rdn4_slot: &mut f64,
        var_vl0__blk169_rdn5_slot: &mut f64,
        var_vl0__blk169_rdn6_slot: &mut f64,
        var_vl0__blk169_rdn7_slot: &mut f64,
        var_vl0__blk169_rdn8_slot: &mut f64,
        var_vl0__blk169_rdn9_slot: &mut f64,
        var_vl0__blk169_rv_slot: &mut f64,
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
        var_vl__blk173_rdn0_slot: &mut f64,
        var_vl__blk173_rdn1_slot: &mut f64,
        var_vl__blk173_rdn10_slot: &mut f64,
        var_vl__blk173_rdn11_slot: &mut f64,
        var_vl__blk173_rdn12_slot: &mut f64,
        var_vl__blk173_rdn13_slot: &mut f64,
        var_vl__blk173_rdn2_slot: &mut f64,
        var_vl__blk173_rdn3_slot: &mut f64,
        var_vl__blk173_rdn4_slot: &mut f64,
        var_vl__blk173_rdn5_slot: &mut f64,
        var_vl__blk173_rdn6_slot: &mut f64,
        var_vl__blk173_rdn7_slot: &mut f64,
        var_vl__blk173_rdn8_slot: &mut f64,
        var_vl__blk173_rdn9_slot: &mut f64,
        var_vl__blk173_rv_slot: &mut f64,
    ) {
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
        let mut var_dv__blk181_rdn0: f64 = *var_dv__blk181_rdn0_slot;
        let mut var_dv__blk181_rdn1: f64 = *var_dv__blk181_rdn1_slot;
        let mut var_dv__blk181_rdn10: f64 = *var_dv__blk181_rdn10_slot;
        let mut var_dv__blk181_rdn11: f64 = *var_dv__blk181_rdn11_slot;
        let mut var_dv__blk181_rdn12: f64 = *var_dv__blk181_rdn12_slot;
        let mut var_dv__blk181_rdn13: f64 = *var_dv__blk181_rdn13_slot;
        let mut var_dv__blk181_rdn2: f64 = *var_dv__blk181_rdn2_slot;
        let mut var_dv__blk181_rdn3: f64 = *var_dv__blk181_rdn3_slot;
        let mut var_dv__blk181_rdn4: f64 = *var_dv__blk181_rdn4_slot;
        let mut var_dv__blk181_rdn5: f64 = *var_dv__blk181_rdn5_slot;
        let mut var_dv__blk181_rdn6: f64 = *var_dv__blk181_rdn6_slot;
        let mut var_dv__blk181_rdn7: f64 = *var_dv__blk181_rdn7_slot;
        let mut var_dv__blk181_rdn8: f64 = *var_dv__blk181_rdn8_slot;
        let mut var_dv__blk181_rdn9: f64 = *var_dv__blk181_rdn9_slot;
        let mut var_dv__blk181_rv: f64 = *var_dv__blk181_rv_slot;
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
        let mut var_mif_rdn0: f64 = *var_mif_rdn0_slot;
        let mut var_mif_rdn1: f64 = *var_mif_rdn1_slot;
        let mut var_mif_rdn10: f64 = *var_mif_rdn10_slot;
        let mut var_mif_rdn11: f64 = *var_mif_rdn11_slot;
        let mut var_mif_rdn12: f64 = *var_mif_rdn12_slot;
        let mut var_mif_rdn13: f64 = *var_mif_rdn13_slot;
        let mut var_mif_rdn2: f64 = *var_mif_rdn2_slot;
        let mut var_mif_rdn3: f64 = *var_mif_rdn3_slot;
        let mut var_mif_rdn4: f64 = *var_mif_rdn4_slot;
        let mut var_mif_rdn5: f64 = *var_mif_rdn5_slot;
        let mut var_mif_rdn6: f64 = *var_mif_rdn6_slot;
        let mut var_mif_rdn7: f64 = *var_mif_rdn7_slot;
        let mut var_mif_rdn8: f64 = *var_mif_rdn8_slot;
        let mut var_mif_rdn9: f64 = *var_mif_rdn9_slot;
        let mut var_mif_rv: f64 = *var_mif_rv_slot;
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
        let mut var_mv0__blk179_rdn0: f64 = *var_mv0__blk179_rdn0_slot;
        let mut var_mv0__blk179_rdn1: f64 = *var_mv0__blk179_rdn1_slot;
        let mut var_mv0__blk179_rdn10: f64 = *var_mv0__blk179_rdn10_slot;
        let mut var_mv0__blk179_rdn11: f64 = *var_mv0__blk179_rdn11_slot;
        let mut var_mv0__blk179_rdn12: f64 = *var_mv0__blk179_rdn12_slot;
        let mut var_mv0__blk179_rdn13: f64 = *var_mv0__blk179_rdn13_slot;
        let mut var_mv0__blk179_rdn2: f64 = *var_mv0__blk179_rdn2_slot;
        let mut var_mv0__blk179_rdn3: f64 = *var_mv0__blk179_rdn3_slot;
        let mut var_mv0__blk179_rdn4: f64 = *var_mv0__blk179_rdn4_slot;
        let mut var_mv0__blk179_rdn5: f64 = *var_mv0__blk179_rdn5_slot;
        let mut var_mv0__blk179_rdn6: f64 = *var_mv0__blk179_rdn6_slot;
        let mut var_mv0__blk179_rdn7: f64 = *var_mv0__blk179_rdn7_slot;
        let mut var_mv0__blk179_rdn8: f64 = *var_mv0__blk179_rdn8_slot;
        let mut var_mv0__blk179_rdn9: f64 = *var_mv0__blk179_rdn9_slot;
        let mut var_mv0__blk179_rv: f64 = *var_mv0__blk179_rv_slot;
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
        let mut var_mv__blk182_rdn0: f64 = *var_mv__blk182_rdn0_slot;
        let mut var_mv__blk182_rdn1: f64 = *var_mv__blk182_rdn1_slot;
        let mut var_mv__blk182_rdn10: f64 = *var_mv__blk182_rdn10_slot;
        let mut var_mv__blk182_rdn11: f64 = *var_mv__blk182_rdn11_slot;
        let mut var_mv__blk182_rdn12: f64 = *var_mv__blk182_rdn12_slot;
        let mut var_mv__blk182_rdn13: f64 = *var_mv__blk182_rdn13_slot;
        let mut var_mv__blk182_rdn2: f64 = *var_mv__blk182_rdn2_slot;
        let mut var_mv__blk182_rdn3: f64 = *var_mv__blk182_rdn3_slot;
        let mut var_mv__blk182_rdn4: f64 = *var_mv__blk182_rdn4_slot;
        let mut var_mv__blk182_rdn5: f64 = *var_mv__blk182_rdn5_slot;
        let mut var_mv__blk182_rdn6: f64 = *var_mv__blk182_rdn6_slot;
        let mut var_mv__blk182_rdn7: f64 = *var_mv__blk182_rdn7_slot;
        let mut var_mv__blk182_rdn8: f64 = *var_mv__blk182_rdn8_slot;
        let mut var_mv__blk182_rdn9: f64 = *var_mv__blk182_rdn9_slot;
        let mut var_mv__blk182_rv: f64 = *var_mv__blk182_rv_slot;
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
        let mut var_q0__blk180_rdn0: f64 = *var_q0__blk180_rdn0_slot;
        let mut var_q0__blk180_rdn1: f64 = *var_q0__blk180_rdn1_slot;
        let mut var_q0__blk180_rdn10: f64 = *var_q0__blk180_rdn10_slot;
        let mut var_q0__blk180_rdn11: f64 = *var_q0__blk180_rdn11_slot;
        let mut var_q0__blk180_rdn12: f64 = *var_q0__blk180_rdn12_slot;
        let mut var_q0__blk180_rdn13: f64 = *var_q0__blk180_rdn13_slot;
        let mut var_q0__blk180_rdn2: f64 = *var_q0__blk180_rdn2_slot;
        let mut var_q0__blk180_rdn3: f64 = *var_q0__blk180_rdn3_slot;
        let mut var_q0__blk180_rdn4: f64 = *var_q0__blk180_rdn4_slot;
        let mut var_q0__blk180_rdn5: f64 = *var_q0__blk180_rdn5_slot;
        let mut var_q0__blk180_rdn6: f64 = *var_q0__blk180_rdn6_slot;
        let mut var_q0__blk180_rdn7: f64 = *var_q0__blk180_rdn7_slot;
        let mut var_q0__blk180_rdn8: f64 = *var_q0__blk180_rdn8_slot;
        let mut var_q0__blk180_rdn9: f64 = *var_q0__blk180_rdn9_slot;
        let mut var_q0__blk180_rv: f64 = *var_q0__blk180_rv_slot;
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
        let mut var_qdbep_rdn0: f64 = *var_qdbep_rdn0_slot;
        let mut var_qdbep_rdn1: f64 = *var_qdbep_rdn1_slot;
        let mut var_qdbep_rdn10: f64 = *var_qdbep_rdn10_slot;
        let mut var_qdbep_rdn11: f64 = *var_qdbep_rdn11_slot;
        let mut var_qdbep_rdn12: f64 = *var_qdbep_rdn12_slot;
        let mut var_qdbep_rdn13: f64 = *var_qdbep_rdn13_slot;
        let mut var_qdbep_rdn2: f64 = *var_qdbep_rdn2_slot;
        let mut var_qdbep_rdn3: f64 = *var_qdbep_rdn3_slot;
        let mut var_qdbep_rdn4: f64 = *var_qdbep_rdn4_slot;
        let mut var_qdbep_rdn5: f64 = *var_qdbep_rdn5_slot;
        let mut var_qdbep_rdn6: f64 = *var_qdbep_rdn6_slot;
        let mut var_qdbep_rdn7: f64 = *var_qdbep_rdn7_slot;
        let mut var_qdbep_rdn8: f64 = *var_qdbep_rdn8_slot;
        let mut var_qdbep_rdn9: f64 = *var_qdbep_rdn9_slot;
        let mut var_qdbep_rv: f64 = *var_qdbep_rv_slot;
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
        let mut var_qlo__blk165_rdn0: f64 = *var_qlo__blk165_rdn0_slot;
        let mut var_qlo__blk165_rdn1: f64 = *var_qlo__blk165_rdn1_slot;
        let mut var_qlo__blk165_rdn10: f64 = *var_qlo__blk165_rdn10_slot;
        let mut var_qlo__blk165_rdn11: f64 = *var_qlo__blk165_rdn11_slot;
        let mut var_qlo__blk165_rdn12: f64 = *var_qlo__blk165_rdn12_slot;
        let mut var_qlo__blk165_rdn13: f64 = *var_qlo__blk165_rdn13_slot;
        let mut var_qlo__blk165_rdn2: f64 = *var_qlo__blk165_rdn2_slot;
        let mut var_qlo__blk165_rdn3: f64 = *var_qlo__blk165_rdn3_slot;
        let mut var_qlo__blk165_rdn4: f64 = *var_qlo__blk165_rdn4_slot;
        let mut var_qlo__blk165_rdn5: f64 = *var_qlo__blk165_rdn5_slot;
        let mut var_qlo__blk165_rdn6: f64 = *var_qlo__blk165_rdn6_slot;
        let mut var_qlo__blk165_rdn7: f64 = *var_qlo__blk165_rdn7_slot;
        let mut var_qlo__blk165_rdn8: f64 = *var_qlo__blk165_rdn8_slot;
        let mut var_qlo__blk165_rdn9: f64 = *var_qlo__blk165_rdn9_slot;
        let mut var_qlo__blk165_rv: f64 = *var_qlo__blk165_rv_slot;
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
        let mut var_rif_rdn0: f64 = *var_rif_rdn0_slot;
        let mut var_rif_rdn1: f64 = *var_rif_rdn1_slot;
        let mut var_rif_rdn10: f64 = *var_rif_rdn10_slot;
        let mut var_rif_rdn11: f64 = *var_rif_rdn11_slot;
        let mut var_rif_rdn12: f64 = *var_rif_rdn12_slot;
        let mut var_rif_rdn13: f64 = *var_rif_rdn13_slot;
        let mut var_rif_rdn2: f64 = *var_rif_rdn2_slot;
        let mut var_rif_rdn3: f64 = *var_rif_rdn3_slot;
        let mut var_rif_rdn4: f64 = *var_rif_rdn4_slot;
        let mut var_rif_rdn5: f64 = *var_rif_rdn5_slot;
        let mut var_rif_rdn6: f64 = *var_rif_rdn6_slot;
        let mut var_rif_rdn7: f64 = *var_rif_rdn7_slot;
        let mut var_rif_rdn8: f64 = *var_rif_rdn8_slot;
        let mut var_rif_rdn9: f64 = *var_rif_rdn9_slot;
        let mut var_rif_rv: f64 = *var_rif_rv_slot;
        let mut var_sgif: f64 = *var_sgif_slot;
        let mut var_sgif_dn0: f64 = *var_sgif_dn0_slot;
        let mut var_sgif_dn1: f64 = *var_sgif_dn1_slot;
        let mut var_sgif_dn10: f64 = *var_sgif_dn10_slot;
        let mut var_sgif_dn11: f64 = *var_sgif_dn11_slot;
        let mut var_sgif_dn12: f64 = *var_sgif_dn12_slot;
        let mut var_sgif_dn13: f64 = *var_sgif_dn13_slot;
        let mut var_sgif_dn2: f64 = *var_sgif_dn2_slot;
        let mut var_sgif_dn3: f64 = *var_sgif_dn3_slot;
        let mut var_sgif_dn4: f64 = *var_sgif_dn4_slot;
        let mut var_sgif_dn5: f64 = *var_sgif_dn5_slot;
        let mut var_sgif_dn6: f64 = *var_sgif_dn6_slot;
        let mut var_sgif_dn7: f64 = *var_sgif_dn7_slot;
        let mut var_sgif_dn8: f64 = *var_sgif_dn8_slot;
        let mut var_sgif_dn9: f64 = *var_sgif_dn9_slot;
        let mut var_sgif_rdn0: f64 = *var_sgif_rdn0_slot;
        let mut var_sgif_rdn1: f64 = *var_sgif_rdn1_slot;
        let mut var_sgif_rdn10: f64 = *var_sgif_rdn10_slot;
        let mut var_sgif_rdn11: f64 = *var_sgif_rdn11_slot;
        let mut var_sgif_rdn12: f64 = *var_sgif_rdn12_slot;
        let mut var_sgif_rdn13: f64 = *var_sgif_rdn13_slot;
        let mut var_sgif_rdn2: f64 = *var_sgif_rdn2_slot;
        let mut var_sgif_rdn3: f64 = *var_sgif_rdn3_slot;
        let mut var_sgif_rdn4: f64 = *var_sgif_rdn4_slot;
        let mut var_sgif_rdn5: f64 = *var_sgif_rdn5_slot;
        let mut var_sgif_rdn6: f64 = *var_sgif_rdn6_slot;
        let mut var_sgif_rdn7: f64 = *var_sgif_rdn7_slot;
        let mut var_sgif_rdn8: f64 = *var_sgif_rdn8_slot;
        let mut var_sgif_rdn9: f64 = *var_sgif_rdn9_slot;
        let mut var_sgif_rv: f64 = *var_sgif_rv_slot;
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
        let mut var_vl0__blk169_rdn0: f64 = *var_vl0__blk169_rdn0_slot;
        let mut var_vl0__blk169_rdn1: f64 = *var_vl0__blk169_rdn1_slot;
        let mut var_vl0__blk169_rdn10: f64 = *var_vl0__blk169_rdn10_slot;
        let mut var_vl0__blk169_rdn11: f64 = *var_vl0__blk169_rdn11_slot;
        let mut var_vl0__blk169_rdn12: f64 = *var_vl0__blk169_rdn12_slot;
        let mut var_vl0__blk169_rdn13: f64 = *var_vl0__blk169_rdn13_slot;
        let mut var_vl0__blk169_rdn2: f64 = *var_vl0__blk169_rdn2_slot;
        let mut var_vl0__blk169_rdn3: f64 = *var_vl0__blk169_rdn3_slot;
        let mut var_vl0__blk169_rdn4: f64 = *var_vl0__blk169_rdn4_slot;
        let mut var_vl0__blk169_rdn5: f64 = *var_vl0__blk169_rdn5_slot;
        let mut var_vl0__blk169_rdn6: f64 = *var_vl0__blk169_rdn6_slot;
        let mut var_vl0__blk169_rdn7: f64 = *var_vl0__blk169_rdn7_slot;
        let mut var_vl0__blk169_rdn8: f64 = *var_vl0__blk169_rdn8_slot;
        let mut var_vl0__blk169_rdn9: f64 = *var_vl0__blk169_rdn9_slot;
        let mut var_vl0__blk169_rv: f64 = *var_vl0__blk169_rv_slot;
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
        let mut var_vl__blk173_rdn0: f64 = *var_vl__blk173_rdn0_slot;
        let mut var_vl__blk173_rdn1: f64 = *var_vl__blk173_rdn1_slot;
        let mut var_vl__blk173_rdn10: f64 = *var_vl__blk173_rdn10_slot;
        let mut var_vl__blk173_rdn11: f64 = *var_vl__blk173_rdn11_slot;
        let mut var_vl__blk173_rdn12: f64 = *var_vl__blk173_rdn12_slot;
        let mut var_vl__blk173_rdn13: f64 = *var_vl__blk173_rdn13_slot;
        let mut var_vl__blk173_rdn2: f64 = *var_vl__blk173_rdn2_slot;
        let mut var_vl__blk173_rdn3: f64 = *var_vl__blk173_rdn3_slot;
        let mut var_vl__blk173_rdn4: f64 = *var_vl__blk173_rdn4_slot;
        let mut var_vl__blk173_rdn5: f64 = *var_vl__blk173_rdn5_slot;
        let mut var_vl__blk173_rdn6: f64 = *var_vl__blk173_rdn6_slot;
        let mut var_vl__blk173_rdn7: f64 = *var_vl__blk173_rdn7_slot;
        let mut var_vl__blk173_rdn8: f64 = *var_vl__blk173_rdn8_slot;
        let mut var_vl__blk173_rdn9: f64 = *var_vl__blk173_rdn9_slot;
        let mut var_vl__blk173_rv: f64 = *var_vl__blk173_rv_slot;

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
        var_qdbep_rv = 0.0;
        var_qdbep_rdn0 = 0.0;
        var_qdbep_rdn1 = 0.0;
        var_qdbep_rdn2 = 0.0;
        var_qdbep_rdn3 = 0.0;
        var_qdbep_rdn4 = 0.0;
        var_qdbep_rdn5 = 0.0;
        var_qdbep_rdn6 = 0.0;
        var_qdbep_rdn7 = 0.0;
        var_qdbep_rdn8 = 0.0;
        var_qdbep_rdn9 = 0.0;
        var_qdbep_rdn10 = 0.0;
        var_qdbep_rdn11 = 0.0;
        var_qdbep_rdn12 = 0.0;
        var_qdbep_rdn13 = 0.0;

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
        var_mv0__blk179_rv = 0.0;
        var_mv0__blk179_rdn0 = 0.0;
        var_mv0__blk179_rdn1 = 0.0;
        var_mv0__blk179_rdn2 = 0.0;
        var_mv0__blk179_rdn3 = 0.0;
        var_mv0__blk179_rdn4 = 0.0;
        var_mv0__blk179_rdn5 = 0.0;
        var_mv0__blk179_rdn6 = 0.0;
        var_mv0__blk179_rdn7 = 0.0;
        var_mv0__blk179_rdn8 = 0.0;
        var_mv0__blk179_rdn9 = 0.0;
        var_mv0__blk179_rdn10 = 0.0;
        var_mv0__blk179_rdn11 = 0.0;
        var_mv0__blk179_rdn12 = 0.0;
        var_mv0__blk179_rdn13 = 0.0;

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
        var_vl0__blk169_rv = 0.0;
        var_vl0__blk169_rdn0 = 0.0;
        var_vl0__blk169_rdn1 = 0.0;
        var_vl0__blk169_rdn2 = 0.0;
        var_vl0__blk169_rdn3 = 0.0;
        var_vl0__blk169_rdn4 = 0.0;
        var_vl0__blk169_rdn5 = 0.0;
        var_vl0__blk169_rdn6 = 0.0;
        var_vl0__blk169_rdn7 = 0.0;
        var_vl0__blk169_rdn8 = 0.0;
        var_vl0__blk169_rdn9 = 0.0;
        var_vl0__blk169_rdn10 = 0.0;
        var_vl0__blk169_rdn11 = 0.0;
        var_vl0__blk169_rdn12 = 0.0;
        var_vl0__blk169_rdn13 = 0.0;

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
        var_q0__blk180_rv = 0.0;
        var_q0__blk180_rdn0 = 0.0;
        var_q0__blk180_rdn1 = 0.0;
        var_q0__blk180_rdn2 = 0.0;
        var_q0__blk180_rdn3 = 0.0;
        var_q0__blk180_rdn4 = 0.0;
        var_q0__blk180_rdn5 = 0.0;
        var_q0__blk180_rdn6 = 0.0;
        var_q0__blk180_rdn7 = 0.0;
        var_q0__blk180_rdn8 = 0.0;
        var_q0__blk180_rdn9 = 0.0;
        var_q0__blk180_rdn10 = 0.0;
        var_q0__blk180_rdn11 = 0.0;
        var_q0__blk180_rdn12 = 0.0;
        var_q0__blk180_rdn13 = 0.0;

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
        var_dv__blk181_rv = 0.0;
        var_dv__blk181_rdn0 = 0.0;
        var_dv__blk181_rdn1 = 0.0;
        var_dv__blk181_rdn2 = 0.0;
        var_dv__blk181_rdn3 = 0.0;
        var_dv__blk181_rdn4 = 0.0;
        var_dv__blk181_rdn5 = 0.0;
        var_dv__blk181_rdn6 = 0.0;
        var_dv__blk181_rdn7 = 0.0;
        var_dv__blk181_rdn8 = 0.0;
        var_dv__blk181_rdn9 = 0.0;
        var_dv__blk181_rdn10 = 0.0;
        var_dv__blk181_rdn11 = 0.0;
        var_dv__blk181_rdn12 = 0.0;
        var_dv__blk181_rdn13 = 0.0;

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
        var_mv__blk182_rv = 0.0;
        var_mv__blk182_rdn0 = 0.0;
        var_mv__blk182_rdn1 = 0.0;
        var_mv__blk182_rdn2 = 0.0;
        var_mv__blk182_rdn3 = 0.0;
        var_mv__blk182_rdn4 = 0.0;
        var_mv__blk182_rdn5 = 0.0;
        var_mv__blk182_rdn6 = 0.0;
        var_mv__blk182_rdn7 = 0.0;
        var_mv__blk182_rdn8 = 0.0;
        var_mv__blk182_rdn9 = 0.0;
        var_mv__blk182_rdn10 = 0.0;
        var_mv__blk182_rdn11 = 0.0;
        var_mv__blk182_rdn12 = 0.0;
        var_mv__blk182_rdn13 = 0.0;

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
        var_vl__blk173_rv = 0.0;
        var_vl__blk173_rdn0 = 0.0;
        var_vl__blk173_rdn1 = 0.0;
        var_vl__blk173_rdn2 = 0.0;
        var_vl__blk173_rdn3 = 0.0;
        var_vl__blk173_rdn4 = 0.0;
        var_vl__blk173_rdn5 = 0.0;
        var_vl__blk173_rdn6 = 0.0;
        var_vl__blk173_rdn7 = 0.0;
        var_vl__blk173_rdn8 = 0.0;
        var_vl__blk173_rdn9 = 0.0;
        var_vl__blk173_rdn10 = 0.0;
        var_vl__blk173_rdn11 = 0.0;
        var_vl__blk173_rdn12 = 0.0;
        var_vl__blk173_rdn13 = 0.0;

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
        var_qlo__blk165_rv = 0.0;
        var_qlo__blk165_rdn0 = 0.0;
        var_qlo__blk165_rdn1 = 0.0;
        var_qlo__blk165_rdn2 = 0.0;
        var_qlo__blk165_rdn3 = 0.0;
        var_qlo__blk165_rdn4 = 0.0;
        var_qlo__blk165_rdn5 = 0.0;
        var_qlo__blk165_rdn6 = 0.0;
        var_qlo__blk165_rdn7 = 0.0;
        var_qlo__blk165_rdn8 = 0.0;
        var_qlo__blk165_rdn9 = 0.0;
        var_qlo__blk165_rdn10 = 0.0;
        var_qlo__blk165_rdn11 = 0.0;
        var_qlo__blk165_rdn12 = 0.0;
        var_qlo__blk165_rdn13 = 0.0;

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
        var_qdbep_rv = 0.0;
        var_qdbep_rdn0 = 0.0;
        var_qdbep_rdn1 = 0.0;
        var_qdbep_rdn2 = 0.0;
        var_qdbep_rdn3 = 0.0;
        var_qdbep_rdn4 = 0.0;
        var_qdbep_rdn5 = 0.0;
        var_qdbep_rdn6 = 0.0;
        var_qdbep_rdn7 = 0.0;
        var_qdbep_rdn8 = 0.0;
        var_qdbep_rdn9 = 0.0;
        var_qdbep_rdn10 = 0.0;
        var_qdbep_rdn11 = 0.0;
        var_qdbep_rdn12 = 0.0;
        var_qdbep_rdn13 = 0.0;

        let (assign5480_e6110,) = {
    if (var_ifi > 0.0) {
        (1.0,)
    } else {
        (0.0,)
    }
};
        var_sgif = assign5480_e6110;
        var_sgif_dn0 = 0.0;
        var_sgif_dn1 = 0.0;
        var_sgif_dn2 = 0.0;
        var_sgif_dn3 = 0.0;
        var_sgif_dn4 = 0.0;
        var_sgif_dn5 = 0.0;
        var_sgif_dn6 = 0.0;
        var_sgif_dn7 = 0.0;
        var_sgif_dn8 = 0.0;
        var_sgif_dn9 = 0.0;
        var_sgif_dn10 = 0.0;
        var_sgif_dn11 = 0.0;
        var_sgif_dn12 = 0.0;
        var_sgif_dn13 = 0.0;
        var_sgif_rv = 0.0;
        var_sgif_rdn0 = 0.0;
        var_sgif_rdn1 = 0.0;
        var_sgif_rdn2 = 0.0;
        var_sgif_rdn3 = 0.0;
        var_sgif_rdn4 = 0.0;
        var_sgif_rdn5 = 0.0;
        var_sgif_rdn6 = 0.0;
        var_sgif_rdn7 = 0.0;
        var_sgif_rdn8 = 0.0;
        var_sgif_rdn9 = 0.0;
        var_sgif_rdn10 = 0.0;
        var_sgif_rdn11 = 0.0;
        var_sgif_rdn12 = 0.0;
        var_sgif_rdn13 = 0.0;

        let assign5490_e6113: f64 = (var_ifi * var_sgif);
        let assign5490_e6115: f64 = (assign5490_e6113 * var_iitf);
        var_rif = assign5490_e6115;
        var_rif_dn0 = ((((var_ifi_dn0 * var_sgif) + (var_ifi * var_sgif_dn0)) * var_iitf) + (assign5490_e6113 * var_iitf_dn0));
        var_rif_dn1 = ((((var_ifi_dn1 * var_sgif) + (var_ifi * var_sgif_dn1)) * var_iitf) + (assign5490_e6113 * var_iitf_dn1));
        var_rif_dn2 = ((((var_ifi_dn2 * var_sgif) + (var_ifi * var_sgif_dn2)) * var_iitf) + (assign5490_e6113 * var_iitf_dn2));
        var_rif_dn3 = ((((var_ifi_dn3 * var_sgif) + (var_ifi * var_sgif_dn3)) * var_iitf) + (assign5490_e6113 * var_iitf_dn3));
        var_rif_dn4 = ((((var_ifi_dn4 * var_sgif) + (var_ifi * var_sgif_dn4)) * var_iitf) + (assign5490_e6113 * var_iitf_dn4));
        var_rif_dn5 = ((((var_ifi_dn5 * var_sgif) + (var_ifi * var_sgif_dn5)) * var_iitf) + (assign5490_e6113 * var_iitf_dn5));
        var_rif_dn6 = ((((var_ifi_dn6 * var_sgif) + (var_ifi * var_sgif_dn6)) * var_iitf) + (assign5490_e6113 * var_iitf_dn6));
        var_rif_dn7 = ((((var_ifi_dn7 * var_sgif) + (var_ifi * var_sgif_dn7)) * var_iitf) + (assign5490_e6113 * var_iitf_dn7));
        var_rif_dn8 = ((((var_ifi_dn8 * var_sgif) + (var_ifi * var_sgif_dn8)) * var_iitf) + (assign5490_e6113 * var_iitf_dn8));
        var_rif_dn9 = ((((var_ifi_dn9 * var_sgif) + (var_ifi * var_sgif_dn9)) * var_iitf) + (assign5490_e6113 * var_iitf_dn9));
        var_rif_dn10 = ((((var_ifi_dn10 * var_sgif) + (var_ifi * var_sgif_dn10)) * var_iitf) + (assign5490_e6113 * var_iitf_dn10));
        var_rif_dn11 = ((((var_ifi_dn11 * var_sgif) + (var_ifi * var_sgif_dn11)) * var_iitf) + (assign5490_e6113 * var_iitf_dn11));
        var_rif_dn12 = ((((var_ifi_dn12 * var_sgif) + (var_ifi * var_sgif_dn12)) * var_iitf) + (assign5490_e6113 * var_iitf_dn12));
        var_rif_dn13 = ((((var_ifi_dn13 * var_sgif) + (var_ifi * var_sgif_dn13)) * var_iitf) + (assign5490_e6113 * var_iitf_dn13));
        var_rif_rv = 0.0;
        var_rif_rdn0 = 0.0;
        var_rif_rdn1 = 0.0;
        var_rif_rdn2 = 0.0;
        var_rif_rdn3 = 0.0;
        var_rif_rdn4 = 0.0;
        var_rif_rdn5 = 0.0;
        var_rif_rdn6 = 0.0;
        var_rif_rdn7 = 0.0;
        var_rif_rdn8 = 0.0;
        var_rif_rdn9 = 0.0;
        var_rif_rdn10 = 0.0;
        var_rif_rdn11 = 0.0;
        var_rif_rdn12 = 0.0;
        var_rif_rdn13 = 0.0;

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
        var_mif_rv = 0.0;
        var_mif_rdn0 = 0.0;
        var_mif_rdn1 = 0.0;
        var_mif_rdn2 = 0.0;
        var_mif_rdn3 = 0.0;
        var_mif_rdn4 = 0.0;
        var_mif_rdn5 = 0.0;
        var_mif_rdn6 = 0.0;
        var_mif_rdn7 = 0.0;
        var_mif_rdn8 = 0.0;
        var_mif_rdn9 = 0.0;
        var_mif_rdn10 = 0.0;
        var_mif_rdn11 = 0.0;
        var_mif_rdn12 = 0.0;
        var_mif_rdn13 = 0.0;


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
        *var_dv__blk181_rdn0_slot = var_dv__blk181_rdn0;
        *var_dv__blk181_rdn1_slot = var_dv__blk181_rdn1;
        *var_dv__blk181_rdn10_slot = var_dv__blk181_rdn10;
        *var_dv__blk181_rdn11_slot = var_dv__blk181_rdn11;
        *var_dv__blk181_rdn12_slot = var_dv__blk181_rdn12;
        *var_dv__blk181_rdn13_slot = var_dv__blk181_rdn13;
        *var_dv__blk181_rdn2_slot = var_dv__blk181_rdn2;
        *var_dv__blk181_rdn3_slot = var_dv__blk181_rdn3;
        *var_dv__blk181_rdn4_slot = var_dv__blk181_rdn4;
        *var_dv__blk181_rdn5_slot = var_dv__blk181_rdn5;
        *var_dv__blk181_rdn6_slot = var_dv__blk181_rdn6;
        *var_dv__blk181_rdn7_slot = var_dv__blk181_rdn7;
        *var_dv__blk181_rdn8_slot = var_dv__blk181_rdn8;
        *var_dv__blk181_rdn9_slot = var_dv__blk181_rdn9;
        *var_dv__blk181_rv_slot = var_dv__blk181_rv;
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
        *var_mif_rdn0_slot = var_mif_rdn0;
        *var_mif_rdn1_slot = var_mif_rdn1;
        *var_mif_rdn10_slot = var_mif_rdn10;
        *var_mif_rdn11_slot = var_mif_rdn11;
        *var_mif_rdn12_slot = var_mif_rdn12;
        *var_mif_rdn13_slot = var_mif_rdn13;
        *var_mif_rdn2_slot = var_mif_rdn2;
        *var_mif_rdn3_slot = var_mif_rdn3;
        *var_mif_rdn4_slot = var_mif_rdn4;
        *var_mif_rdn5_slot = var_mif_rdn5;
        *var_mif_rdn6_slot = var_mif_rdn6;
        *var_mif_rdn7_slot = var_mif_rdn7;
        *var_mif_rdn8_slot = var_mif_rdn8;
        *var_mif_rdn9_slot = var_mif_rdn9;
        *var_mif_rv_slot = var_mif_rv;
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
        *var_mv0__blk179_rdn0_slot = var_mv0__blk179_rdn0;
        *var_mv0__blk179_rdn1_slot = var_mv0__blk179_rdn1;
        *var_mv0__blk179_rdn10_slot = var_mv0__blk179_rdn10;
        *var_mv0__blk179_rdn11_slot = var_mv0__blk179_rdn11;
        *var_mv0__blk179_rdn12_slot = var_mv0__blk179_rdn12;
        *var_mv0__blk179_rdn13_slot = var_mv0__blk179_rdn13;
        *var_mv0__blk179_rdn2_slot = var_mv0__blk179_rdn2;
        *var_mv0__blk179_rdn3_slot = var_mv0__blk179_rdn3;
        *var_mv0__blk179_rdn4_slot = var_mv0__blk179_rdn4;
        *var_mv0__blk179_rdn5_slot = var_mv0__blk179_rdn5;
        *var_mv0__blk179_rdn6_slot = var_mv0__blk179_rdn6;
        *var_mv0__blk179_rdn7_slot = var_mv0__blk179_rdn7;
        *var_mv0__blk179_rdn8_slot = var_mv0__blk179_rdn8;
        *var_mv0__blk179_rdn9_slot = var_mv0__blk179_rdn9;
        *var_mv0__blk179_rv_slot = var_mv0__blk179_rv;
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
        *var_mv__blk182_rdn0_slot = var_mv__blk182_rdn0;
        *var_mv__blk182_rdn1_slot = var_mv__blk182_rdn1;
        *var_mv__blk182_rdn10_slot = var_mv__blk182_rdn10;
        *var_mv__blk182_rdn11_slot = var_mv__blk182_rdn11;
        *var_mv__blk182_rdn12_slot = var_mv__blk182_rdn12;
        *var_mv__blk182_rdn13_slot = var_mv__blk182_rdn13;
        *var_mv__blk182_rdn2_slot = var_mv__blk182_rdn2;
        *var_mv__blk182_rdn3_slot = var_mv__blk182_rdn3;
        *var_mv__blk182_rdn4_slot = var_mv__blk182_rdn4;
        *var_mv__blk182_rdn5_slot = var_mv__blk182_rdn5;
        *var_mv__blk182_rdn6_slot = var_mv__blk182_rdn6;
        *var_mv__blk182_rdn7_slot = var_mv__blk182_rdn7;
        *var_mv__blk182_rdn8_slot = var_mv__blk182_rdn8;
        *var_mv__blk182_rdn9_slot = var_mv__blk182_rdn9;
        *var_mv__blk182_rv_slot = var_mv__blk182_rv;
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
        *var_q0__blk180_rdn0_slot = var_q0__blk180_rdn0;
        *var_q0__blk180_rdn1_slot = var_q0__blk180_rdn1;
        *var_q0__blk180_rdn10_slot = var_q0__blk180_rdn10;
        *var_q0__blk180_rdn11_slot = var_q0__blk180_rdn11;
        *var_q0__blk180_rdn12_slot = var_q0__blk180_rdn12;
        *var_q0__blk180_rdn13_slot = var_q0__blk180_rdn13;
        *var_q0__blk180_rdn2_slot = var_q0__blk180_rdn2;
        *var_q0__blk180_rdn3_slot = var_q0__blk180_rdn3;
        *var_q0__blk180_rdn4_slot = var_q0__blk180_rdn4;
        *var_q0__blk180_rdn5_slot = var_q0__blk180_rdn5;
        *var_q0__blk180_rdn6_slot = var_q0__blk180_rdn6;
        *var_q0__blk180_rdn7_slot = var_q0__blk180_rdn7;
        *var_q0__blk180_rdn8_slot = var_q0__blk180_rdn8;
        *var_q0__blk180_rdn9_slot = var_q0__blk180_rdn9;
        *var_q0__blk180_rv_slot = var_q0__blk180_rv;
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
        *var_qdbep_rdn0_slot = var_qdbep_rdn0;
        *var_qdbep_rdn1_slot = var_qdbep_rdn1;
        *var_qdbep_rdn10_slot = var_qdbep_rdn10;
        *var_qdbep_rdn11_slot = var_qdbep_rdn11;
        *var_qdbep_rdn12_slot = var_qdbep_rdn12;
        *var_qdbep_rdn13_slot = var_qdbep_rdn13;
        *var_qdbep_rdn2_slot = var_qdbep_rdn2;
        *var_qdbep_rdn3_slot = var_qdbep_rdn3;
        *var_qdbep_rdn4_slot = var_qdbep_rdn4;
        *var_qdbep_rdn5_slot = var_qdbep_rdn5;
        *var_qdbep_rdn6_slot = var_qdbep_rdn6;
        *var_qdbep_rdn7_slot = var_qdbep_rdn7;
        *var_qdbep_rdn8_slot = var_qdbep_rdn8;
        *var_qdbep_rdn9_slot = var_qdbep_rdn9;
        *var_qdbep_rv_slot = var_qdbep_rv;
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
        *var_qlo__blk165_rdn0_slot = var_qlo__blk165_rdn0;
        *var_qlo__blk165_rdn1_slot = var_qlo__blk165_rdn1;
        *var_qlo__blk165_rdn10_slot = var_qlo__blk165_rdn10;
        *var_qlo__blk165_rdn11_slot = var_qlo__blk165_rdn11;
        *var_qlo__blk165_rdn12_slot = var_qlo__blk165_rdn12;
        *var_qlo__blk165_rdn13_slot = var_qlo__blk165_rdn13;
        *var_qlo__blk165_rdn2_slot = var_qlo__blk165_rdn2;
        *var_qlo__blk165_rdn3_slot = var_qlo__blk165_rdn3;
        *var_qlo__blk165_rdn4_slot = var_qlo__blk165_rdn4;
        *var_qlo__blk165_rdn5_slot = var_qlo__blk165_rdn5;
        *var_qlo__blk165_rdn6_slot = var_qlo__blk165_rdn6;
        *var_qlo__blk165_rdn7_slot = var_qlo__blk165_rdn7;
        *var_qlo__blk165_rdn8_slot = var_qlo__blk165_rdn8;
        *var_qlo__blk165_rdn9_slot = var_qlo__blk165_rdn9;
        *var_qlo__blk165_rv_slot = var_qlo__blk165_rv;
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
        *var_rif_rdn0_slot = var_rif_rdn0;
        *var_rif_rdn1_slot = var_rif_rdn1;
        *var_rif_rdn10_slot = var_rif_rdn10;
        *var_rif_rdn11_slot = var_rif_rdn11;
        *var_rif_rdn12_slot = var_rif_rdn12;
        *var_rif_rdn13_slot = var_rif_rdn13;
        *var_rif_rdn2_slot = var_rif_rdn2;
        *var_rif_rdn3_slot = var_rif_rdn3;
        *var_rif_rdn4_slot = var_rif_rdn4;
        *var_rif_rdn5_slot = var_rif_rdn5;
        *var_rif_rdn6_slot = var_rif_rdn6;
        *var_rif_rdn7_slot = var_rif_rdn7;
        *var_rif_rdn8_slot = var_rif_rdn8;
        *var_rif_rdn9_slot = var_rif_rdn9;
        *var_rif_rv_slot = var_rif_rv;
        *var_sgif_slot = var_sgif;
        *var_sgif_dn0_slot = var_sgif_dn0;
        *var_sgif_dn1_slot = var_sgif_dn1;
        *var_sgif_dn10_slot = var_sgif_dn10;
        *var_sgif_dn11_slot = var_sgif_dn11;
        *var_sgif_dn12_slot = var_sgif_dn12;
        *var_sgif_dn13_slot = var_sgif_dn13;
        *var_sgif_dn2_slot = var_sgif_dn2;
        *var_sgif_dn3_slot = var_sgif_dn3;
        *var_sgif_dn4_slot = var_sgif_dn4;
        *var_sgif_dn5_slot = var_sgif_dn5;
        *var_sgif_dn6_slot = var_sgif_dn6;
        *var_sgif_dn7_slot = var_sgif_dn7;
        *var_sgif_dn8_slot = var_sgif_dn8;
        *var_sgif_dn9_slot = var_sgif_dn9;
        *var_sgif_rdn0_slot = var_sgif_rdn0;
        *var_sgif_rdn1_slot = var_sgif_rdn1;
        *var_sgif_rdn10_slot = var_sgif_rdn10;
        *var_sgif_rdn11_slot = var_sgif_rdn11;
        *var_sgif_rdn12_slot = var_sgif_rdn12;
        *var_sgif_rdn13_slot = var_sgif_rdn13;
        *var_sgif_rdn2_slot = var_sgif_rdn2;
        *var_sgif_rdn3_slot = var_sgif_rdn3;
        *var_sgif_rdn4_slot = var_sgif_rdn4;
        *var_sgif_rdn5_slot = var_sgif_rdn5;
        *var_sgif_rdn6_slot = var_sgif_rdn6;
        *var_sgif_rdn7_slot = var_sgif_rdn7;
        *var_sgif_rdn8_slot = var_sgif_rdn8;
        *var_sgif_rdn9_slot = var_sgif_rdn9;
        *var_sgif_rv_slot = var_sgif_rv;
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
        *var_vl0__blk169_rdn0_slot = var_vl0__blk169_rdn0;
        *var_vl0__blk169_rdn1_slot = var_vl0__blk169_rdn1;
        *var_vl0__blk169_rdn10_slot = var_vl0__blk169_rdn10;
        *var_vl0__blk169_rdn11_slot = var_vl0__blk169_rdn11;
        *var_vl0__blk169_rdn12_slot = var_vl0__blk169_rdn12;
        *var_vl0__blk169_rdn13_slot = var_vl0__blk169_rdn13;
        *var_vl0__blk169_rdn2_slot = var_vl0__blk169_rdn2;
        *var_vl0__blk169_rdn3_slot = var_vl0__blk169_rdn3;
        *var_vl0__blk169_rdn4_slot = var_vl0__blk169_rdn4;
        *var_vl0__blk169_rdn5_slot = var_vl0__blk169_rdn5;
        *var_vl0__blk169_rdn6_slot = var_vl0__blk169_rdn6;
        *var_vl0__blk169_rdn7_slot = var_vl0__blk169_rdn7;
        *var_vl0__blk169_rdn8_slot = var_vl0__blk169_rdn8;
        *var_vl0__blk169_rdn9_slot = var_vl0__blk169_rdn9;
        *var_vl0__blk169_rv_slot = var_vl0__blk169_rv;
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
        *var_vl__blk173_rdn0_slot = var_vl__blk173_rdn0;
        *var_vl__blk173_rdn1_slot = var_vl__blk173_rdn1;
        *var_vl__blk173_rdn10_slot = var_vl__blk173_rdn10;
        *var_vl__blk173_rdn11_slot = var_vl__blk173_rdn11;
        *var_vl__blk173_rdn12_slot = var_vl__blk173_rdn12;
        *var_vl__blk173_rdn13_slot = var_vl__blk173_rdn13;
        *var_vl__blk173_rdn2_slot = var_vl__blk173_rdn2;
        *var_vl__blk173_rdn3_slot = var_vl__blk173_rdn3;
        *var_vl__blk173_rdn4_slot = var_vl__blk173_rdn4;
        *var_vl__blk173_rdn5_slot = var_vl__blk173_rdn5;
        *var_vl__blk173_rdn6_slot = var_vl__blk173_rdn6;
        *var_vl__blk173_rdn7_slot = var_vl__blk173_rdn7;
        *var_vl__blk173_rdn8_slot = var_vl__blk173_rdn8;
        *var_vl__blk173_rdn9_slot = var_vl__blk173_rdn9;
        *var_vl__blk173_rv_slot = var_vl__blk173_rv;
    }

    pub(super) fn stamp_reactive_block_28(
        p: &Parameters,
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
        var_ivtf: f64,
        var_ivtf_dn0: f64,
        var_ivtf_dn1: f64,
        var_ivtf_dn10: f64,
        var_ivtf_dn11: f64,
        var_ivtf_dn12: f64,
        var_ivtf_dn13: f64,
        var_ivtf_dn2: f64,
        var_ivtf_dn3: f64,
        var_ivtf_dn4: f64,
        var_ivtf_dn5: f64,
        var_ivtf_dn6: f64,
        var_ivtf_dn7: f64,
        var_ivtf_dn8: f64,
        var_ivtf_dn9: f64,
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
        var_sgif_dn0: f64,
        var_sgif_dn1: f64,
        var_sgif_dn10: f64,
        var_sgif_dn11: f64,
        var_sgif_dn12: f64,
        var_sgif_dn13: f64,
        var_sgif_dn2: f64,
        var_sgif_dn3: f64,
        var_sgif_dn4: f64,
        var_sgif_dn5: f64,
        var_sgif_dn6: f64,
        var_sgif_dn7: f64,
        var_sgif_dn8: f64,
        var_sgif_dn9: f64,
        var_sltf: f64,
        var_sltf_dn0: f64,
        var_sltf_dn1: f64,
        var_sltf_dn10: f64,
        var_sltf_dn11: f64,
        var_sltf_dn12: f64,
        var_sltf_dn13: f64,
        var_sltf_dn2: f64,
        var_sltf_dn3: f64,
        var_sltf_dn4: f64,
        var_sltf_dn5: f64,
        var_sltf_dn6: f64,
        var_sltf_dn7: f64,
        var_sltf_dn8: f64,
        var_sltf_dn9: f64,
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
        var_vbictype: f64,
        var_vbictype_dn0: f64,
        var_vbictype_dn1: f64,
        var_vbictype_dn10: f64,
        var_vbictype_dn11: f64,
        var_vbictype_dn12: f64,
        var_vbictype_dn13: f64,
        var_vbictype_dn2: f64,
        var_vbictype_dn3: f64,
        var_vbictype_dn4: f64,
        var_vbictype_dn5: f64,
        var_vbictype_dn6: f64,
        var_vbictype_dn7: f64,
        var_vbictype_dn8: f64,
        var_vbictype_dn9: f64,
        var_vmaxexp: f64,
        var_vmaxexp_dn0: f64,
        var_vmaxexp_dn1: f64,
        var_vmaxexp_dn10: f64,
        var_vmaxexp_dn11: f64,
        var_vmaxexp_dn12: f64,
        var_vmaxexp_dn13: f64,
        var_vmaxexp_dn2: f64,
        var_vmaxexp_dn3: f64,
        var_vmaxexp_dn4: f64,
        var_vmaxexp_dn5: f64,
        var_vmaxexp_dn6: f64,
        var_vmaxexp_dn7: f64,
        var_vmaxexp_dn8: f64,
        var_vmaxexp_dn9: f64,
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
        var_arg_rdn0_slot: &mut f64,
        var_arg_rdn1_slot: &mut f64,
        var_arg_rdn10_slot: &mut f64,
        var_arg_rdn11_slot: &mut f64,
        var_arg_rdn12_slot: &mut f64,
        var_arg_rdn13_slot: &mut f64,
        var_arg_rdn2_slot: &mut f64,
        var_arg_rdn3_slot: &mut f64,
        var_arg_rdn4_slot: &mut f64,
        var_arg_rdn5_slot: &mut f64,
        var_arg_rdn6_slot: &mut f64,
        var_arg_rdn7_slot: &mut f64,
        var_arg_rdn8_slot: &mut f64,
        var_arg_rdn9_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
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
        var_expi_rdn0_slot: &mut f64,
        var_expi_rdn1_slot: &mut f64,
        var_expi_rdn10_slot: &mut f64,
        var_expi_rdn11_slot: &mut f64,
        var_expi_rdn12_slot: &mut f64,
        var_expi_rdn13_slot: &mut f64,
        var_expi_rdn2_slot: &mut f64,
        var_expi_rdn3_slot: &mut f64,
        var_expi_rdn4_slot: &mut f64,
        var_expi_rdn5_slot: &mut f64,
        var_expi_rdn6_slot: &mut f64,
        var_expi_rdn7_slot: &mut f64,
        var_expi_rdn8_slot: &mut f64,
        var_expi_rdn9_slot: &mut f64,
        var_expi_rv_slot: &mut f64,
        var_guard187_slot: &mut f64,
        var_guard187_dn0_slot: &mut f64,
        var_guard187_dn1_slot: &mut f64,
        var_guard187_dn10_slot: &mut f64,
        var_guard187_dn11_slot: &mut f64,
        var_guard187_dn12_slot: &mut f64,
        var_guard187_dn13_slot: &mut f64,
        var_guard187_dn2_slot: &mut f64,
        var_guard187_dn3_slot: &mut f64,
        var_guard187_dn4_slot: &mut f64,
        var_guard187_dn5_slot: &mut f64,
        var_guard187_dn6_slot: &mut f64,
        var_guard187_dn7_slot: &mut f64,
        var_guard187_dn8_slot: &mut f64,
        var_guard187_dn9_slot: &mut f64,
        var_guard187_rdn0_slot: &mut f64,
        var_guard187_rdn1_slot: &mut f64,
        var_guard187_rdn10_slot: &mut f64,
        var_guard187_rdn11_slot: &mut f64,
        var_guard187_rdn12_slot: &mut f64,
        var_guard187_rdn13_slot: &mut f64,
        var_guard187_rdn2_slot: &mut f64,
        var_guard187_rdn3_slot: &mut f64,
        var_guard187_rdn4_slot: &mut f64,
        var_guard187_rdn5_slot: &mut f64,
        var_guard187_rdn6_slot: &mut f64,
        var_guard187_rdn7_slot: &mut f64,
        var_guard187_rdn8_slot: &mut f64,
        var_guard187_rdn9_slot: &mut f64,
        var_guard187_rv_slot: &mut f64,
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
        var_qbc_rdn0_slot: &mut f64,
        var_qbc_rdn1_slot: &mut f64,
        var_qbc_rdn10_slot: &mut f64,
        var_qbc_rdn11_slot: &mut f64,
        var_qbc_rdn12_slot: &mut f64,
        var_qbc_rdn13_slot: &mut f64,
        var_qbc_rdn2_slot: &mut f64,
        var_qbc_rdn3_slot: &mut f64,
        var_qbc_rdn4_slot: &mut f64,
        var_qbc_rdn5_slot: &mut f64,
        var_qbc_rdn6_slot: &mut f64,
        var_qbc_rdn7_slot: &mut f64,
        var_qbc_rdn8_slot: &mut f64,
        var_qbc_rdn9_slot: &mut f64,
        var_qbc_rv_slot: &mut f64,
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
        var_qbcp_rdn0_slot: &mut f64,
        var_qbcp_rdn1_slot: &mut f64,
        var_qbcp_rdn10_slot: &mut f64,
        var_qbcp_rdn11_slot: &mut f64,
        var_qbcp_rdn12_slot: &mut f64,
        var_qbcp_rdn13_slot: &mut f64,
        var_qbcp_rdn2_slot: &mut f64,
        var_qbcp_rdn3_slot: &mut f64,
        var_qbcp_rdn4_slot: &mut f64,
        var_qbcp_rdn5_slot: &mut f64,
        var_qbcp_rdn6_slot: &mut f64,
        var_qbcp_rdn7_slot: &mut f64,
        var_qbcp_rdn8_slot: &mut f64,
        var_qbcp_rdn9_slot: &mut f64,
        var_qbcp_rv_slot: &mut f64,
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
        var_qbcx_rdn0_slot: &mut f64,
        var_qbcx_rdn1_slot: &mut f64,
        var_qbcx_rdn10_slot: &mut f64,
        var_qbcx_rdn11_slot: &mut f64,
        var_qbcx_rdn12_slot: &mut f64,
        var_qbcx_rdn13_slot: &mut f64,
        var_qbcx_rdn2_slot: &mut f64,
        var_qbcx_rdn3_slot: &mut f64,
        var_qbcx_rdn4_slot: &mut f64,
        var_qbcx_rdn5_slot: &mut f64,
        var_qbcx_rdn6_slot: &mut f64,
        var_qbcx_rdn7_slot: &mut f64,
        var_qbcx_rdn8_slot: &mut f64,
        var_qbcx_rdn9_slot: &mut f64,
        var_qbcx_rv_slot: &mut f64,
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
        var_qbe_rdn0_slot: &mut f64,
        var_qbe_rdn1_slot: &mut f64,
        var_qbe_rdn10_slot: &mut f64,
        var_qbe_rdn11_slot: &mut f64,
        var_qbe_rdn12_slot: &mut f64,
        var_qbe_rdn13_slot: &mut f64,
        var_qbe_rdn2_slot: &mut f64,
        var_qbe_rdn3_slot: &mut f64,
        var_qbe_rdn4_slot: &mut f64,
        var_qbe_rdn5_slot: &mut f64,
        var_qbe_rdn6_slot: &mut f64,
        var_qbe_rdn7_slot: &mut f64,
        var_qbe_rdn8_slot: &mut f64,
        var_qbe_rdn9_slot: &mut f64,
        var_qbe_rv_slot: &mut f64,
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
        var_qbep_rdn0_slot: &mut f64,
        var_qbep_rdn1_slot: &mut f64,
        var_qbep_rdn10_slot: &mut f64,
        var_qbep_rdn11_slot: &mut f64,
        var_qbep_rdn12_slot: &mut f64,
        var_qbep_rdn13_slot: &mut f64,
        var_qbep_rdn2_slot: &mut f64,
        var_qbep_rdn3_slot: &mut f64,
        var_qbep_rdn4_slot: &mut f64,
        var_qbep_rdn5_slot: &mut f64,
        var_qbep_rdn6_slot: &mut f64,
        var_qbep_rdn7_slot: &mut f64,
        var_qbep_rdn8_slot: &mut f64,
        var_qbep_rdn9_slot: &mut f64,
        var_qbep_rv_slot: &mut f64,
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
        var_qbex_rdn0_slot: &mut f64,
        var_qbex_rdn1_slot: &mut f64,
        var_qbex_rdn10_slot: &mut f64,
        var_qbex_rdn11_slot: &mut f64,
        var_qbex_rdn12_slot: &mut f64,
        var_qbex_rdn13_slot: &mut f64,
        var_qbex_rdn2_slot: &mut f64,
        var_qbex_rdn3_slot: &mut f64,
        var_qbex_rdn4_slot: &mut f64,
        var_qbex_rdn5_slot: &mut f64,
        var_qbex_rdn6_slot: &mut f64,
        var_qbex_rdn7_slot: &mut f64,
        var_qbex_rdn8_slot: &mut f64,
        var_qbex_rdn9_slot: &mut f64,
        var_qbex_rv_slot: &mut f64,
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
        var_qcth_rdn0_slot: &mut f64,
        var_qcth_rdn1_slot: &mut f64,
        var_qcth_rdn10_slot: &mut f64,
        var_qcth_rdn11_slot: &mut f64,
        var_qcth_rdn12_slot: &mut f64,
        var_qcth_rdn13_slot: &mut f64,
        var_qcth_rdn2_slot: &mut f64,
        var_qcth_rdn3_slot: &mut f64,
        var_qcth_rdn4_slot: &mut f64,
        var_qcth_rdn5_slot: &mut f64,
        var_qcth_rdn6_slot: &mut f64,
        var_qcth_rdn7_slot: &mut f64,
        var_qcth_rdn8_slot: &mut f64,
        var_qcth_rdn9_slot: &mut f64,
        var_qcth_rv_slot: &mut f64,
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
        var_tff_rdn0_slot: &mut f64,
        var_tff_rdn1_slot: &mut f64,
        var_tff_rdn10_slot: &mut f64,
        var_tff_rdn11_slot: &mut f64,
        var_tff_rdn12_slot: &mut f64,
        var_tff_rdn13_slot: &mut f64,
        var_tff_rdn2_slot: &mut f64,
        var_tff_rdn3_slot: &mut f64,
        var_tff_rdn4_slot: &mut f64,
        var_tff_rdn5_slot: &mut f64,
        var_tff_rdn6_slot: &mut f64,
        var_tff_rdn7_slot: &mut f64,
        var_tff_rdn8_slot: &mut f64,
        var_tff_rdn9_slot: &mut f64,
        var_tff_rv_slot: &mut f64,
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
        let mut var_arg_rdn0: f64 = *var_arg_rdn0_slot;
        let mut var_arg_rdn1: f64 = *var_arg_rdn1_slot;
        let mut var_arg_rdn10: f64 = *var_arg_rdn10_slot;
        let mut var_arg_rdn11: f64 = *var_arg_rdn11_slot;
        let mut var_arg_rdn12: f64 = *var_arg_rdn12_slot;
        let mut var_arg_rdn13: f64 = *var_arg_rdn13_slot;
        let mut var_arg_rdn2: f64 = *var_arg_rdn2_slot;
        let mut var_arg_rdn3: f64 = *var_arg_rdn3_slot;
        let mut var_arg_rdn4: f64 = *var_arg_rdn4_slot;
        let mut var_arg_rdn5: f64 = *var_arg_rdn5_slot;
        let mut var_arg_rdn6: f64 = *var_arg_rdn6_slot;
        let mut var_arg_rdn7: f64 = *var_arg_rdn7_slot;
        let mut var_arg_rdn8: f64 = *var_arg_rdn8_slot;
        let mut var_arg_rdn9: f64 = *var_arg_rdn9_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
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
        let mut var_expi_rdn0: f64 = *var_expi_rdn0_slot;
        let mut var_expi_rdn1: f64 = *var_expi_rdn1_slot;
        let mut var_expi_rdn10: f64 = *var_expi_rdn10_slot;
        let mut var_expi_rdn11: f64 = *var_expi_rdn11_slot;
        let mut var_expi_rdn12: f64 = *var_expi_rdn12_slot;
        let mut var_expi_rdn13: f64 = *var_expi_rdn13_slot;
        let mut var_expi_rdn2: f64 = *var_expi_rdn2_slot;
        let mut var_expi_rdn3: f64 = *var_expi_rdn3_slot;
        let mut var_expi_rdn4: f64 = *var_expi_rdn4_slot;
        let mut var_expi_rdn5: f64 = *var_expi_rdn5_slot;
        let mut var_expi_rdn6: f64 = *var_expi_rdn6_slot;
        let mut var_expi_rdn7: f64 = *var_expi_rdn7_slot;
        let mut var_expi_rdn8: f64 = *var_expi_rdn8_slot;
        let mut var_expi_rdn9: f64 = *var_expi_rdn9_slot;
        let mut var_expi_rv: f64 = *var_expi_rv_slot;
        let mut var_guard187: f64 = *var_guard187_slot;
        let mut var_guard187_dn0: f64 = *var_guard187_dn0_slot;
        let mut var_guard187_dn1: f64 = *var_guard187_dn1_slot;
        let mut var_guard187_dn10: f64 = *var_guard187_dn10_slot;
        let mut var_guard187_dn11: f64 = *var_guard187_dn11_slot;
        let mut var_guard187_dn12: f64 = *var_guard187_dn12_slot;
        let mut var_guard187_dn13: f64 = *var_guard187_dn13_slot;
        let mut var_guard187_dn2: f64 = *var_guard187_dn2_slot;
        let mut var_guard187_dn3: f64 = *var_guard187_dn3_slot;
        let mut var_guard187_dn4: f64 = *var_guard187_dn4_slot;
        let mut var_guard187_dn5: f64 = *var_guard187_dn5_slot;
        let mut var_guard187_dn6: f64 = *var_guard187_dn6_slot;
        let mut var_guard187_dn7: f64 = *var_guard187_dn7_slot;
        let mut var_guard187_dn8: f64 = *var_guard187_dn8_slot;
        let mut var_guard187_dn9: f64 = *var_guard187_dn9_slot;
        let mut var_guard187_rdn0: f64 = *var_guard187_rdn0_slot;
        let mut var_guard187_rdn1: f64 = *var_guard187_rdn1_slot;
        let mut var_guard187_rdn10: f64 = *var_guard187_rdn10_slot;
        let mut var_guard187_rdn11: f64 = *var_guard187_rdn11_slot;
        let mut var_guard187_rdn12: f64 = *var_guard187_rdn12_slot;
        let mut var_guard187_rdn13: f64 = *var_guard187_rdn13_slot;
        let mut var_guard187_rdn2: f64 = *var_guard187_rdn2_slot;
        let mut var_guard187_rdn3: f64 = *var_guard187_rdn3_slot;
        let mut var_guard187_rdn4: f64 = *var_guard187_rdn4_slot;
        let mut var_guard187_rdn5: f64 = *var_guard187_rdn5_slot;
        let mut var_guard187_rdn6: f64 = *var_guard187_rdn6_slot;
        let mut var_guard187_rdn7: f64 = *var_guard187_rdn7_slot;
        let mut var_guard187_rdn8: f64 = *var_guard187_rdn8_slot;
        let mut var_guard187_rdn9: f64 = *var_guard187_rdn9_slot;
        let mut var_guard187_rv: f64 = *var_guard187_rv_slot;
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
        let mut var_qbc_rdn0: f64 = *var_qbc_rdn0_slot;
        let mut var_qbc_rdn1: f64 = *var_qbc_rdn1_slot;
        let mut var_qbc_rdn10: f64 = *var_qbc_rdn10_slot;
        let mut var_qbc_rdn11: f64 = *var_qbc_rdn11_slot;
        let mut var_qbc_rdn12: f64 = *var_qbc_rdn12_slot;
        let mut var_qbc_rdn13: f64 = *var_qbc_rdn13_slot;
        let mut var_qbc_rdn2: f64 = *var_qbc_rdn2_slot;
        let mut var_qbc_rdn3: f64 = *var_qbc_rdn3_slot;
        let mut var_qbc_rdn4: f64 = *var_qbc_rdn4_slot;
        let mut var_qbc_rdn5: f64 = *var_qbc_rdn5_slot;
        let mut var_qbc_rdn6: f64 = *var_qbc_rdn6_slot;
        let mut var_qbc_rdn7: f64 = *var_qbc_rdn7_slot;
        let mut var_qbc_rdn8: f64 = *var_qbc_rdn8_slot;
        let mut var_qbc_rdn9: f64 = *var_qbc_rdn9_slot;
        let mut var_qbc_rv: f64 = *var_qbc_rv_slot;
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
        let mut var_qbcp_rdn0: f64 = *var_qbcp_rdn0_slot;
        let mut var_qbcp_rdn1: f64 = *var_qbcp_rdn1_slot;
        let mut var_qbcp_rdn10: f64 = *var_qbcp_rdn10_slot;
        let mut var_qbcp_rdn11: f64 = *var_qbcp_rdn11_slot;
        let mut var_qbcp_rdn12: f64 = *var_qbcp_rdn12_slot;
        let mut var_qbcp_rdn13: f64 = *var_qbcp_rdn13_slot;
        let mut var_qbcp_rdn2: f64 = *var_qbcp_rdn2_slot;
        let mut var_qbcp_rdn3: f64 = *var_qbcp_rdn3_slot;
        let mut var_qbcp_rdn4: f64 = *var_qbcp_rdn4_slot;
        let mut var_qbcp_rdn5: f64 = *var_qbcp_rdn5_slot;
        let mut var_qbcp_rdn6: f64 = *var_qbcp_rdn6_slot;
        let mut var_qbcp_rdn7: f64 = *var_qbcp_rdn7_slot;
        let mut var_qbcp_rdn8: f64 = *var_qbcp_rdn8_slot;
        let mut var_qbcp_rdn9: f64 = *var_qbcp_rdn9_slot;
        let mut var_qbcp_rv: f64 = *var_qbcp_rv_slot;
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
        let mut var_qbcx_rdn0: f64 = *var_qbcx_rdn0_slot;
        let mut var_qbcx_rdn1: f64 = *var_qbcx_rdn1_slot;
        let mut var_qbcx_rdn10: f64 = *var_qbcx_rdn10_slot;
        let mut var_qbcx_rdn11: f64 = *var_qbcx_rdn11_slot;
        let mut var_qbcx_rdn12: f64 = *var_qbcx_rdn12_slot;
        let mut var_qbcx_rdn13: f64 = *var_qbcx_rdn13_slot;
        let mut var_qbcx_rdn2: f64 = *var_qbcx_rdn2_slot;
        let mut var_qbcx_rdn3: f64 = *var_qbcx_rdn3_slot;
        let mut var_qbcx_rdn4: f64 = *var_qbcx_rdn4_slot;
        let mut var_qbcx_rdn5: f64 = *var_qbcx_rdn5_slot;
        let mut var_qbcx_rdn6: f64 = *var_qbcx_rdn6_slot;
        let mut var_qbcx_rdn7: f64 = *var_qbcx_rdn7_slot;
        let mut var_qbcx_rdn8: f64 = *var_qbcx_rdn8_slot;
        let mut var_qbcx_rdn9: f64 = *var_qbcx_rdn9_slot;
        let mut var_qbcx_rv: f64 = *var_qbcx_rv_slot;
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
        let mut var_qbe_rdn0: f64 = *var_qbe_rdn0_slot;
        let mut var_qbe_rdn1: f64 = *var_qbe_rdn1_slot;
        let mut var_qbe_rdn10: f64 = *var_qbe_rdn10_slot;
        let mut var_qbe_rdn11: f64 = *var_qbe_rdn11_slot;
        let mut var_qbe_rdn12: f64 = *var_qbe_rdn12_slot;
        let mut var_qbe_rdn13: f64 = *var_qbe_rdn13_slot;
        let mut var_qbe_rdn2: f64 = *var_qbe_rdn2_slot;
        let mut var_qbe_rdn3: f64 = *var_qbe_rdn3_slot;
        let mut var_qbe_rdn4: f64 = *var_qbe_rdn4_slot;
        let mut var_qbe_rdn5: f64 = *var_qbe_rdn5_slot;
        let mut var_qbe_rdn6: f64 = *var_qbe_rdn6_slot;
        let mut var_qbe_rdn7: f64 = *var_qbe_rdn7_slot;
        let mut var_qbe_rdn8: f64 = *var_qbe_rdn8_slot;
        let mut var_qbe_rdn9: f64 = *var_qbe_rdn9_slot;
        let mut var_qbe_rv: f64 = *var_qbe_rv_slot;
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
        let mut var_qbep_rdn0: f64 = *var_qbep_rdn0_slot;
        let mut var_qbep_rdn1: f64 = *var_qbep_rdn1_slot;
        let mut var_qbep_rdn10: f64 = *var_qbep_rdn10_slot;
        let mut var_qbep_rdn11: f64 = *var_qbep_rdn11_slot;
        let mut var_qbep_rdn12: f64 = *var_qbep_rdn12_slot;
        let mut var_qbep_rdn13: f64 = *var_qbep_rdn13_slot;
        let mut var_qbep_rdn2: f64 = *var_qbep_rdn2_slot;
        let mut var_qbep_rdn3: f64 = *var_qbep_rdn3_slot;
        let mut var_qbep_rdn4: f64 = *var_qbep_rdn4_slot;
        let mut var_qbep_rdn5: f64 = *var_qbep_rdn5_slot;
        let mut var_qbep_rdn6: f64 = *var_qbep_rdn6_slot;
        let mut var_qbep_rdn7: f64 = *var_qbep_rdn7_slot;
        let mut var_qbep_rdn8: f64 = *var_qbep_rdn8_slot;
        let mut var_qbep_rdn9: f64 = *var_qbep_rdn9_slot;
        let mut var_qbep_rv: f64 = *var_qbep_rv_slot;
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
        let mut var_qbex_rdn0: f64 = *var_qbex_rdn0_slot;
        let mut var_qbex_rdn1: f64 = *var_qbex_rdn1_slot;
        let mut var_qbex_rdn10: f64 = *var_qbex_rdn10_slot;
        let mut var_qbex_rdn11: f64 = *var_qbex_rdn11_slot;
        let mut var_qbex_rdn12: f64 = *var_qbex_rdn12_slot;
        let mut var_qbex_rdn13: f64 = *var_qbex_rdn13_slot;
        let mut var_qbex_rdn2: f64 = *var_qbex_rdn2_slot;
        let mut var_qbex_rdn3: f64 = *var_qbex_rdn3_slot;
        let mut var_qbex_rdn4: f64 = *var_qbex_rdn4_slot;
        let mut var_qbex_rdn5: f64 = *var_qbex_rdn5_slot;
        let mut var_qbex_rdn6: f64 = *var_qbex_rdn6_slot;
        let mut var_qbex_rdn7: f64 = *var_qbex_rdn7_slot;
        let mut var_qbex_rdn8: f64 = *var_qbex_rdn8_slot;
        let mut var_qbex_rdn9: f64 = *var_qbex_rdn9_slot;
        let mut var_qbex_rv: f64 = *var_qbex_rv_slot;
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
        let mut var_qcth_rdn0: f64 = *var_qcth_rdn0_slot;
        let mut var_qcth_rdn1: f64 = *var_qcth_rdn1_slot;
        let mut var_qcth_rdn10: f64 = *var_qcth_rdn10_slot;
        let mut var_qcth_rdn11: f64 = *var_qcth_rdn11_slot;
        let mut var_qcth_rdn12: f64 = *var_qcth_rdn12_slot;
        let mut var_qcth_rdn13: f64 = *var_qcth_rdn13_slot;
        let mut var_qcth_rdn2: f64 = *var_qcth_rdn2_slot;
        let mut var_qcth_rdn3: f64 = *var_qcth_rdn3_slot;
        let mut var_qcth_rdn4: f64 = *var_qcth_rdn4_slot;
        let mut var_qcth_rdn5: f64 = *var_qcth_rdn5_slot;
        let mut var_qcth_rdn6: f64 = *var_qcth_rdn6_slot;
        let mut var_qcth_rdn7: f64 = *var_qcth_rdn7_slot;
        let mut var_qcth_rdn8: f64 = *var_qcth_rdn8_slot;
        let mut var_qcth_rdn9: f64 = *var_qcth_rdn9_slot;
        let mut var_qcth_rv: f64 = *var_qcth_rv_slot;
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
        let mut var_tff_rdn0: f64 = *var_tff_rdn0_slot;
        let mut var_tff_rdn1: f64 = *var_tff_rdn1_slot;
        let mut var_tff_rdn10: f64 = *var_tff_rdn10_slot;
        let mut var_tff_rdn11: f64 = *var_tff_rdn11_slot;
        let mut var_tff_rdn12: f64 = *var_tff_rdn12_slot;
        let mut var_tff_rdn13: f64 = *var_tff_rdn13_slot;
        let mut var_tff_rdn2: f64 = *var_tff_rdn2_slot;
        let mut var_tff_rdn3: f64 = *var_tff_rdn3_slot;
        let mut var_tff_rdn4: f64 = *var_tff_rdn4_slot;
        let mut var_tff_rdn5: f64 = *var_tff_rdn5_slot;
        let mut var_tff_rdn6: f64 = *var_tff_rdn6_slot;
        let mut var_tff_rdn7: f64 = *var_tff_rdn7_slot;
        let mut var_tff_rdn8: f64 = *var_tff_rdn8_slot;
        let mut var_tff_rdn9: f64 = *var_tff_rdn9_slot;
        let mut var_tff_rv: f64 = *var_tff_rv_slot;

        let assign5510_e6123: f64 = (var_vbci * var_ivtf);
        let assign5510_e6125: f64 = (assign5510_e6123 / 1.44);
        var_arg = assign5510_e6125;
        var_arg_dn0 = (((var_vbci_dn0 * var_ivtf) + (var_vbci * var_ivtf_dn0)) / 1.44);
        var_arg_dn1 = (((var_vbci_dn1 * var_ivtf) + (var_vbci * var_ivtf_dn1)) / 1.44);
        var_arg_dn2 = (((var_vbci_dn2 * var_ivtf) + (var_vbci * var_ivtf_dn2)) / 1.44);
        var_arg_dn3 = (((var_vbci_dn3 * var_ivtf) + (var_vbci * var_ivtf_dn3)) / 1.44);
        var_arg_dn4 = (((var_vbci_dn4 * var_ivtf) + (var_vbci * var_ivtf_dn4)) / 1.44);
        var_arg_dn5 = (((var_vbci_dn5 * var_ivtf) + (var_vbci * var_ivtf_dn5)) / 1.44);
        var_arg_dn6 = (((var_vbci_dn6 * var_ivtf) + (var_vbci * var_ivtf_dn6)) / 1.44);
        var_arg_dn7 = (((var_vbci_dn7 * var_ivtf) + (var_vbci * var_ivtf_dn7)) / 1.44);
        var_arg_dn8 = (((var_vbci_dn8 * var_ivtf) + (var_vbci * var_ivtf_dn8)) / 1.44);
        var_arg_dn9 = (((var_vbci_dn9 * var_ivtf) + (var_vbci * var_ivtf_dn9)) / 1.44);
        var_arg_dn10 = (((var_vbci_dn10 * var_ivtf) + (var_vbci * var_ivtf_dn10)) / 1.44);
        var_arg_dn11 = (((var_vbci_dn11 * var_ivtf) + (var_vbci * var_ivtf_dn11)) / 1.44);
        var_arg_dn12 = (((var_vbci_dn12 * var_ivtf) + (var_vbci * var_ivtf_dn12)) / 1.44);
        var_arg_dn13 = (((var_vbci_dn13 * var_ivtf) + (var_vbci * var_ivtf_dn13)) / 1.44);
        var_arg_rv = 0.0;
        var_arg_rdn0 = 0.0;
        var_arg_rdn1 = 0.0;
        var_arg_rdn2 = 0.0;
        var_arg_rdn3 = 0.0;
        var_arg_rdn4 = 0.0;
        var_arg_rdn5 = 0.0;
        var_arg_rdn6 = 0.0;
        var_arg_rdn7 = 0.0;
        var_arg_rdn8 = 0.0;
        var_arg_rdn9 = 0.0;
        var_arg_rdn10 = 0.0;
        var_arg_rdn11 = 0.0;
        var_arg_rdn12 = 0.0;
        var_arg_rdn13 = 0.0;

        let assign5520_e6128: f64 = if var_arg < var_vmaxexp { 1.0 } else { 0.0 };
        var_guard187 = assign5520_e6128;
        var_guard187_dn0 = 0.0;
        var_guard187_dn1 = 0.0;
        var_guard187_dn2 = 0.0;
        var_guard187_dn3 = 0.0;
        var_guard187_dn4 = 0.0;
        var_guard187_dn5 = 0.0;
        var_guard187_dn6 = 0.0;
        var_guard187_dn7 = 0.0;
        var_guard187_dn8 = 0.0;
        var_guard187_dn9 = 0.0;
        var_guard187_dn10 = 0.0;
        var_guard187_dn11 = 0.0;
        var_guard187_dn12 = 0.0;
        var_guard187_dn13 = 0.0;
        var_guard187_rv = 0.0;
        var_guard187_rdn0 = 0.0;
        var_guard187_rdn1 = 0.0;
        var_guard187_rdn2 = 0.0;
        var_guard187_rdn3 = 0.0;
        var_guard187_rdn4 = 0.0;
        var_guard187_rdn5 = 0.0;
        var_guard187_rdn6 = 0.0;
        var_guard187_rdn7 = 0.0;
        var_guard187_rdn8 = 0.0;
        var_guard187_rdn9 = 0.0;
        var_guard187_rdn10 = 0.0;
        var_guard187_rdn11 = 0.0;
        var_guard187_rdn12 = 0.0;
        var_guard187_rdn13 = 0.0;

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
        var_expi_rv = 0.0;
        var_expi_rdn0 = 0.0;
        var_expi_rdn1 = 0.0;
        var_expi_rdn2 = 0.0;
        var_expi_rdn3 = 0.0;
        var_expi_rdn4 = 0.0;
        var_expi_rdn5 = 0.0;
        var_expi_rdn6 = 0.0;
        var_expi_rdn7 = 0.0;
        var_expi_rdn8 = 0.0;
        var_expi_rdn9 = 0.0;
        var_expi_rdn10 = 0.0;
        var_expi_rdn11 = 0.0;
        var_expi_rdn12 = 0.0;
        var_expi_rdn13 = 0.0;

        let (assign5540_e6145, assign5540_e6145_d_n0, assign5540_e6145_d_n1, assign5540_e6145_d_n2, assign5540_e6145_d_n3, assign5540_e6145_d_n4, assign5540_e6145_d_n5, assign5540_e6145_d_n6, assign5540_e6145_d_n7, assign5540_e6145_d_n8, assign5540_e6145_d_n9, assign5540_e6145_d_n10, assign5540_e6145_d_n11, assign5540_e6145_d_n12, assign5540_e6145_d_n13,) = {
    if (var_guard187 == 0.0) {
        let assign5540_e6137: f64 = (var_vmaxexp).exp();
        let assign5540_e6141: f64 = (var_arg - var_vmaxexp);
        let assign5540_e6142: f64 = (1.0 + assign5540_e6141);
        let assign5540_e6143: f64 = (assign5540_e6137 * assign5540_e6142);
        (assign5540_e6143, (((assign5540_e6137 * var_vmaxexp_dn0) * assign5540_e6142) + (assign5540_e6137 * (var_arg_dn0 - var_vmaxexp_dn0))), (((assign5540_e6137 * var_vmaxexp_dn1) * assign5540_e6142) + (assign5540_e6137 * (var_arg_dn1 - var_vmaxexp_dn1))), (((assign5540_e6137 * var_vmaxexp_dn2) * assign5540_e6142) + (assign5540_e6137 * (var_arg_dn2 - var_vmaxexp_dn2))), (((assign5540_e6137 * var_vmaxexp_dn3) * assign5540_e6142) + (assign5540_e6137 * (var_arg_dn3 - var_vmaxexp_dn3))), (((assign5540_e6137 * var_vmaxexp_dn4) * assign5540_e6142) + (assign5540_e6137 * (var_arg_dn4 - var_vmaxexp_dn4))), (((assign5540_e6137 * var_vmaxexp_dn5) * assign5540_e6142) + (assign5540_e6137 * (var_arg_dn5 - var_vmaxexp_dn5))), (((assign5540_e6137 * var_vmaxexp_dn6) * assign5540_e6142) + (assign5540_e6137 * (var_arg_dn6 - var_vmaxexp_dn6))), (((assign5540_e6137 * var_vmaxexp_dn7) * assign5540_e6142) + (assign5540_e6137 * (var_arg_dn7 - var_vmaxexp_dn7))), (((assign5540_e6137 * var_vmaxexp_dn8) * assign5540_e6142) + (assign5540_e6137 * (var_arg_dn8 - var_vmaxexp_dn8))), (((assign5540_e6137 * var_vmaxexp_dn9) * assign5540_e6142) + (assign5540_e6137 * (var_arg_dn9 - var_vmaxexp_dn9))), (((assign5540_e6137 * var_vmaxexp_dn10) * assign5540_e6142) + (assign5540_e6137 * (var_arg_dn10 - var_vmaxexp_dn10))), (((assign5540_e6137 * var_vmaxexp_dn11) * assign5540_e6142) + (assign5540_e6137 * (var_arg_dn11 - var_vmaxexp_dn11))), (((assign5540_e6137 * var_vmaxexp_dn12) * assign5540_e6142) + (assign5540_e6137 * (var_arg_dn12 - var_vmaxexp_dn12))), (((assign5540_e6137 * var_vmaxexp_dn13) * assign5540_e6142) + (assign5540_e6137 * (var_arg_dn13 - var_vmaxexp_dn13))),)
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
        var_expi_rv = 0.0;
        var_expi_rdn0 = 0.0;
        var_expi_rdn1 = 0.0;
        var_expi_rdn2 = 0.0;
        var_expi_rdn3 = 0.0;
        var_expi_rdn4 = 0.0;
        var_expi_rdn5 = 0.0;
        var_expi_rdn6 = 0.0;
        var_expi_rdn7 = 0.0;
        var_expi_rdn8 = 0.0;
        var_expi_rdn9 = 0.0;
        var_expi_rdn10 = 0.0;
        var_expi_rdn11 = 0.0;
        var_expi_rdn12 = 0.0;
        var_expi_rdn13 = 0.0;

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
        var_tff_dn0 = (((p.p76 * (p.p77 * var_q1_dn0)) * assign5550_e6165) + (assign5550_e6152 * (((((p.p78 * var_expi_dn0) * assign5550_e6161) + (assign5550_e6156 * (var_sltf_dn0 + ((var_mif_dn0 * var_mif) + (var_mif * var_mif_dn0))))) * var_sgif) + (assign5550_e6162 * var_sgif_dn0))));
        var_tff_dn1 = (((p.p76 * (p.p77 * var_q1_dn1)) * assign5550_e6165) + (assign5550_e6152 * (((((p.p78 * var_expi_dn1) * assign5550_e6161) + (assign5550_e6156 * (var_sltf_dn1 + ((var_mif_dn1 * var_mif) + (var_mif * var_mif_dn1))))) * var_sgif) + (assign5550_e6162 * var_sgif_dn1))));
        var_tff_dn2 = (((p.p76 * (p.p77 * var_q1_dn2)) * assign5550_e6165) + (assign5550_e6152 * (((((p.p78 * var_expi_dn2) * assign5550_e6161) + (assign5550_e6156 * (var_sltf_dn2 + ((var_mif_dn2 * var_mif) + (var_mif * var_mif_dn2))))) * var_sgif) + (assign5550_e6162 * var_sgif_dn2))));
        var_tff_dn3 = (((p.p76 * (p.p77 * var_q1_dn3)) * assign5550_e6165) + (assign5550_e6152 * (((((p.p78 * var_expi_dn3) * assign5550_e6161) + (assign5550_e6156 * (var_sltf_dn3 + ((var_mif_dn3 * var_mif) + (var_mif * var_mif_dn3))))) * var_sgif) + (assign5550_e6162 * var_sgif_dn3))));
        var_tff_dn4 = (((p.p76 * (p.p77 * var_q1_dn4)) * assign5550_e6165) + (assign5550_e6152 * (((((p.p78 * var_expi_dn4) * assign5550_e6161) + (assign5550_e6156 * (var_sltf_dn4 + ((var_mif_dn4 * var_mif) + (var_mif * var_mif_dn4))))) * var_sgif) + (assign5550_e6162 * var_sgif_dn4))));
        var_tff_dn5 = (((p.p76 * (p.p77 * var_q1_dn5)) * assign5550_e6165) + (assign5550_e6152 * (((((p.p78 * var_expi_dn5) * assign5550_e6161) + (assign5550_e6156 * (var_sltf_dn5 + ((var_mif_dn5 * var_mif) + (var_mif * var_mif_dn5))))) * var_sgif) + (assign5550_e6162 * var_sgif_dn5))));
        var_tff_dn6 = (((p.p76 * (p.p77 * var_q1_dn6)) * assign5550_e6165) + (assign5550_e6152 * (((((p.p78 * var_expi_dn6) * assign5550_e6161) + (assign5550_e6156 * (var_sltf_dn6 + ((var_mif_dn6 * var_mif) + (var_mif * var_mif_dn6))))) * var_sgif) + (assign5550_e6162 * var_sgif_dn6))));
        var_tff_dn7 = (((p.p76 * (p.p77 * var_q1_dn7)) * assign5550_e6165) + (assign5550_e6152 * (((((p.p78 * var_expi_dn7) * assign5550_e6161) + (assign5550_e6156 * (var_sltf_dn7 + ((var_mif_dn7 * var_mif) + (var_mif * var_mif_dn7))))) * var_sgif) + (assign5550_e6162 * var_sgif_dn7))));
        var_tff_dn8 = (((p.p76 * (p.p77 * var_q1_dn8)) * assign5550_e6165) + (assign5550_e6152 * (((((p.p78 * var_expi_dn8) * assign5550_e6161) + (assign5550_e6156 * (var_sltf_dn8 + ((var_mif_dn8 * var_mif) + (var_mif * var_mif_dn8))))) * var_sgif) + (assign5550_e6162 * var_sgif_dn8))));
        var_tff_dn9 = (((p.p76 * (p.p77 * var_q1_dn9)) * assign5550_e6165) + (assign5550_e6152 * (((((p.p78 * var_expi_dn9) * assign5550_e6161) + (assign5550_e6156 * (var_sltf_dn9 + ((var_mif_dn9 * var_mif) + (var_mif * var_mif_dn9))))) * var_sgif) + (assign5550_e6162 * var_sgif_dn9))));
        var_tff_dn10 = (((p.p76 * (p.p77 * var_q1_dn10)) * assign5550_e6165) + (assign5550_e6152 * (((((p.p78 * var_expi_dn10) * assign5550_e6161) + (assign5550_e6156 * (var_sltf_dn10 + ((var_mif_dn10 * var_mif) + (var_mif * var_mif_dn10))))) * var_sgif) + (assign5550_e6162 * var_sgif_dn10))));
        var_tff_dn11 = (((p.p76 * (p.p77 * var_q1_dn11)) * assign5550_e6165) + (assign5550_e6152 * (((((p.p78 * var_expi_dn11) * assign5550_e6161) + (assign5550_e6156 * (var_sltf_dn11 + ((var_mif_dn11 * var_mif) + (var_mif * var_mif_dn11))))) * var_sgif) + (assign5550_e6162 * var_sgif_dn11))));
        var_tff_dn12 = (((p.p76 * (p.p77 * var_q1_dn12)) * assign5550_e6165) + (assign5550_e6152 * (((((p.p78 * var_expi_dn12) * assign5550_e6161) + (assign5550_e6156 * (var_sltf_dn12 + ((var_mif_dn12 * var_mif) + (var_mif * var_mif_dn12))))) * var_sgif) + (assign5550_e6162 * var_sgif_dn12))));
        var_tff_dn13 = (((p.p76 * (p.p77 * var_q1_dn13)) * assign5550_e6165) + (assign5550_e6152 * (((((p.p78 * var_expi_dn13) * assign5550_e6161) + (assign5550_e6156 * (var_sltf_dn13 + ((var_mif_dn13 * var_mif) + (var_mif * var_mif_dn13))))) * var_sgif) + (assign5550_e6162 * var_sgif_dn13))));
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
        var_tff_rdn10 = 0.0;
        var_tff_rdn11 = 0.0;
        var_tff_rdn12 = 0.0;
        var_tff_rdn13 = 0.0;

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
        var_qbe_rv = 0.0;
        var_qbe_rdn0 = 0.0;
        var_qbe_rdn1 = 0.0;
        var_qbe_rdn2 = 0.0;
        var_qbe_rdn3 = 0.0;
        var_qbe_rdn4 = 0.0;
        var_qbe_rdn5 = 0.0;
        var_qbe_rdn6 = 0.0;
        var_qbe_rdn7 = 0.0;
        var_qbe_rdn8 = 0.0;
        var_qbe_rdn9 = 0.0;
        var_qbe_rdn10 = 0.0;
        var_qbe_rdn11 = 0.0;
        var_qbe_rdn12 = 0.0;
        var_qbe_rdn13 = 0.0;

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
        var_qbex_rv = 0.0;
        var_qbex_rdn0 = 0.0;
        var_qbex_rdn1 = 0.0;
        var_qbex_rdn2 = 0.0;
        var_qbex_rdn3 = 0.0;
        var_qbex_rdn4 = 0.0;
        var_qbex_rdn5 = 0.0;
        var_qbex_rdn6 = 0.0;
        var_qbex_rdn7 = 0.0;
        var_qbex_rdn8 = 0.0;
        var_qbex_rdn9 = 0.0;
        var_qbex_rdn10 = 0.0;
        var_qbex_rdn11 = 0.0;
        var_qbex_rdn12 = 0.0;
        var_qbex_rdn13 = 0.0;

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
        var_qbc_rv = 0.0;
        var_qbc_rdn0 = 0.0;
        var_qbc_rdn1 = 0.0;
        var_qbc_rdn2 = 0.0;
        var_qbc_rdn3 = 0.0;
        var_qbc_rdn4 = 0.0;
        var_qbc_rdn5 = 0.0;
        var_qbc_rdn6 = 0.0;
        var_qbc_rdn7 = 0.0;
        var_qbc_rdn8 = 0.0;
        var_qbc_rdn9 = 0.0;
        var_qbc_rdn10 = 0.0;
        var_qbc_rdn11 = 0.0;
        var_qbc_rdn12 = 0.0;
        var_qbc_rdn13 = 0.0;

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
        var_qbcx_rv = 0.0;
        var_qbcx_rdn0 = 0.0;
        var_qbcx_rdn1 = 0.0;
        var_qbcx_rdn2 = 0.0;
        var_qbcx_rdn3 = 0.0;
        var_qbcx_rdn4 = 0.0;
        var_qbcx_rdn5 = 0.0;
        var_qbcx_rdn6 = 0.0;
        var_qbcx_rdn7 = 0.0;
        var_qbcx_rdn8 = 0.0;
        var_qbcx_rdn9 = 0.0;
        var_qbcx_rdn10 = 0.0;
        var_qbcx_rdn11 = 0.0;
        var_qbcx_rdn12 = 0.0;
        var_qbcx_rdn13 = 0.0;

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
        var_qbep_rv = 0.0;
        var_qbep_rdn0 = 0.0;
        var_qbep_rdn1 = 0.0;
        var_qbep_rdn2 = 0.0;
        var_qbep_rdn3 = 0.0;
        var_qbep_rdn4 = 0.0;
        var_qbep_rdn5 = 0.0;
        var_qbep_rdn6 = 0.0;
        var_qbep_rdn7 = 0.0;
        var_qbep_rdn8 = 0.0;
        var_qbep_rdn9 = 0.0;
        var_qbep_rdn10 = 0.0;
        var_qbep_rdn11 = 0.0;
        var_qbep_rdn12 = 0.0;
        var_qbep_rdn13 = 0.0;

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
        var_qbcp_rv = 0.0;
        var_qbcp_rdn0 = 0.0;
        var_qbcp_rdn1 = 0.0;
        var_qbcp_rdn2 = 0.0;
        var_qbcp_rdn3 = 0.0;
        var_qbcp_rdn4 = 0.0;
        var_qbcp_rdn5 = 0.0;
        var_qbcp_rdn6 = 0.0;
        var_qbcp_rdn7 = 0.0;
        var_qbcp_rdn8 = 0.0;
        var_qbcp_rdn9 = 0.0;
        var_qbcp_rdn10 = 0.0;
        var_qbcp_rdn11 = 0.0;
        var_qbcp_rdn12 = 0.0;
        var_qbcp_rdn13 = 0.0;

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
        var_qcth_rv = 0.0;
        var_qcth_rdn0 = 0.0;
        var_qcth_rdn1 = 0.0;
        var_qcth_rdn2 = 0.0;
        var_qcth_rdn3 = 0.0;
        var_qcth_rdn4 = 0.0;
        var_qcth_rdn5 = 0.0;
        var_qcth_rdn6 = 0.0;
        var_qcth_rdn7 = 0.0;
        var_qcth_rdn8 = 0.0;
        var_qcth_rdn9 = 0.0;
        var_qcth_rdn10 = 0.0;
        var_qcth_rdn11 = 0.0;
        var_qcth_rdn12 = 0.0;
        var_qcth_rdn13 = 0.0;

        let assign5670_e6232: f64 = var_vbictype;
        let assign5670_e6234: f64 = (assign5670_e6232 * var_qbe);
        var_qbe = assign5670_e6234;
        var_qbe_dn0 = ((var_vbictype_dn0 * var_qbe) + (assign5670_e6232 * var_qbe_dn0));
        var_qbe_dn1 = ((var_vbictype_dn1 * var_qbe) + (assign5670_e6232 * var_qbe_dn1));
        var_qbe_dn2 = ((var_vbictype_dn2 * var_qbe) + (assign5670_e6232 * var_qbe_dn2));
        var_qbe_dn3 = ((var_vbictype_dn3 * var_qbe) + (assign5670_e6232 * var_qbe_dn3));
        var_qbe_dn4 = ((var_vbictype_dn4 * var_qbe) + (assign5670_e6232 * var_qbe_dn4));
        var_qbe_dn5 = ((var_vbictype_dn5 * var_qbe) + (assign5670_e6232 * var_qbe_dn5));
        var_qbe_dn6 = ((var_vbictype_dn6 * var_qbe) + (assign5670_e6232 * var_qbe_dn6));
        var_qbe_dn7 = ((var_vbictype_dn7 * var_qbe) + (assign5670_e6232 * var_qbe_dn7));
        var_qbe_dn8 = ((var_vbictype_dn8 * var_qbe) + (assign5670_e6232 * var_qbe_dn8));
        var_qbe_dn9 = ((var_vbictype_dn9 * var_qbe) + (assign5670_e6232 * var_qbe_dn9));
        var_qbe_dn10 = ((var_vbictype_dn10 * var_qbe) + (assign5670_e6232 * var_qbe_dn10));
        var_qbe_dn11 = ((var_vbictype_dn11 * var_qbe) + (assign5670_e6232 * var_qbe_dn11));
        var_qbe_dn12 = ((var_vbictype_dn12 * var_qbe) + (assign5670_e6232 * var_qbe_dn12));
        var_qbe_dn13 = ((var_vbictype_dn13 * var_qbe) + (assign5670_e6232 * var_qbe_dn13));
        var_qbe_rv = 0.0;
        var_qbe_rdn0 = 0.0;
        var_qbe_rdn1 = 0.0;
        var_qbe_rdn2 = 0.0;
        var_qbe_rdn3 = 0.0;
        var_qbe_rdn4 = 0.0;
        var_qbe_rdn5 = 0.0;
        var_qbe_rdn6 = 0.0;
        var_qbe_rdn7 = 0.0;
        var_qbe_rdn8 = 0.0;
        var_qbe_rdn9 = 0.0;
        var_qbe_rdn10 = 0.0;
        var_qbe_rdn11 = 0.0;
        var_qbe_rdn12 = 0.0;
        var_qbe_rdn13 = 0.0;

        let assign5680_e6237: f64 = var_vbictype;
        let assign5680_e6239: f64 = (assign5680_e6237 * var_qbex);
        var_qbex = assign5680_e6239;
        var_qbex_dn0 = ((var_vbictype_dn0 * var_qbex) + (assign5680_e6237 * var_qbex_dn0));
        var_qbex_dn1 = ((var_vbictype_dn1 * var_qbex) + (assign5680_e6237 * var_qbex_dn1));
        var_qbex_dn2 = ((var_vbictype_dn2 * var_qbex) + (assign5680_e6237 * var_qbex_dn2));
        var_qbex_dn3 = ((var_vbictype_dn3 * var_qbex) + (assign5680_e6237 * var_qbex_dn3));
        var_qbex_dn4 = ((var_vbictype_dn4 * var_qbex) + (assign5680_e6237 * var_qbex_dn4));
        var_qbex_dn5 = ((var_vbictype_dn5 * var_qbex) + (assign5680_e6237 * var_qbex_dn5));
        var_qbex_dn6 = ((var_vbictype_dn6 * var_qbex) + (assign5680_e6237 * var_qbex_dn6));
        var_qbex_dn7 = ((var_vbictype_dn7 * var_qbex) + (assign5680_e6237 * var_qbex_dn7));
        var_qbex_dn8 = ((var_vbictype_dn8 * var_qbex) + (assign5680_e6237 * var_qbex_dn8));
        var_qbex_dn9 = ((var_vbictype_dn9 * var_qbex) + (assign5680_e6237 * var_qbex_dn9));
        var_qbex_dn10 = ((var_vbictype_dn10 * var_qbex) + (assign5680_e6237 * var_qbex_dn10));
        var_qbex_dn11 = ((var_vbictype_dn11 * var_qbex) + (assign5680_e6237 * var_qbex_dn11));
        var_qbex_dn12 = ((var_vbictype_dn12 * var_qbex) + (assign5680_e6237 * var_qbex_dn12));
        var_qbex_dn13 = ((var_vbictype_dn13 * var_qbex) + (assign5680_e6237 * var_qbex_dn13));
        var_qbex_rv = 0.0;
        var_qbex_rdn0 = 0.0;
        var_qbex_rdn1 = 0.0;
        var_qbex_rdn2 = 0.0;
        var_qbex_rdn3 = 0.0;
        var_qbex_rdn4 = 0.0;
        var_qbex_rdn5 = 0.0;
        var_qbex_rdn6 = 0.0;
        var_qbex_rdn7 = 0.0;
        var_qbex_rdn8 = 0.0;
        var_qbex_rdn9 = 0.0;
        var_qbex_rdn10 = 0.0;
        var_qbex_rdn11 = 0.0;
        var_qbex_rdn12 = 0.0;
        var_qbex_rdn13 = 0.0;


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
        *var_arg_rdn0_slot = var_arg_rdn0;
        *var_arg_rdn1_slot = var_arg_rdn1;
        *var_arg_rdn10_slot = var_arg_rdn10;
        *var_arg_rdn11_slot = var_arg_rdn11;
        *var_arg_rdn12_slot = var_arg_rdn12;
        *var_arg_rdn13_slot = var_arg_rdn13;
        *var_arg_rdn2_slot = var_arg_rdn2;
        *var_arg_rdn3_slot = var_arg_rdn3;
        *var_arg_rdn4_slot = var_arg_rdn4;
        *var_arg_rdn5_slot = var_arg_rdn5;
        *var_arg_rdn6_slot = var_arg_rdn6;
        *var_arg_rdn7_slot = var_arg_rdn7;
        *var_arg_rdn8_slot = var_arg_rdn8;
        *var_arg_rdn9_slot = var_arg_rdn9;
        *var_arg_rv_slot = var_arg_rv;
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
        *var_expi_rdn0_slot = var_expi_rdn0;
        *var_expi_rdn1_slot = var_expi_rdn1;
        *var_expi_rdn10_slot = var_expi_rdn10;
        *var_expi_rdn11_slot = var_expi_rdn11;
        *var_expi_rdn12_slot = var_expi_rdn12;
        *var_expi_rdn13_slot = var_expi_rdn13;
        *var_expi_rdn2_slot = var_expi_rdn2;
        *var_expi_rdn3_slot = var_expi_rdn3;
        *var_expi_rdn4_slot = var_expi_rdn4;
        *var_expi_rdn5_slot = var_expi_rdn5;
        *var_expi_rdn6_slot = var_expi_rdn6;
        *var_expi_rdn7_slot = var_expi_rdn7;
        *var_expi_rdn8_slot = var_expi_rdn8;
        *var_expi_rdn9_slot = var_expi_rdn9;
        *var_expi_rv_slot = var_expi_rv;
        *var_guard187_slot = var_guard187;
        *var_guard187_dn0_slot = var_guard187_dn0;
        *var_guard187_dn1_slot = var_guard187_dn1;
        *var_guard187_dn10_slot = var_guard187_dn10;
        *var_guard187_dn11_slot = var_guard187_dn11;
        *var_guard187_dn12_slot = var_guard187_dn12;
        *var_guard187_dn13_slot = var_guard187_dn13;
        *var_guard187_dn2_slot = var_guard187_dn2;
        *var_guard187_dn3_slot = var_guard187_dn3;
        *var_guard187_dn4_slot = var_guard187_dn4;
        *var_guard187_dn5_slot = var_guard187_dn5;
        *var_guard187_dn6_slot = var_guard187_dn6;
        *var_guard187_dn7_slot = var_guard187_dn7;
        *var_guard187_dn8_slot = var_guard187_dn8;
        *var_guard187_dn9_slot = var_guard187_dn9;
        *var_guard187_rdn0_slot = var_guard187_rdn0;
        *var_guard187_rdn1_slot = var_guard187_rdn1;
        *var_guard187_rdn10_slot = var_guard187_rdn10;
        *var_guard187_rdn11_slot = var_guard187_rdn11;
        *var_guard187_rdn12_slot = var_guard187_rdn12;
        *var_guard187_rdn13_slot = var_guard187_rdn13;
        *var_guard187_rdn2_slot = var_guard187_rdn2;
        *var_guard187_rdn3_slot = var_guard187_rdn3;
        *var_guard187_rdn4_slot = var_guard187_rdn4;
        *var_guard187_rdn5_slot = var_guard187_rdn5;
        *var_guard187_rdn6_slot = var_guard187_rdn6;
        *var_guard187_rdn7_slot = var_guard187_rdn7;
        *var_guard187_rdn8_slot = var_guard187_rdn8;
        *var_guard187_rdn9_slot = var_guard187_rdn9;
        *var_guard187_rv_slot = var_guard187_rv;
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
        *var_qbc_rdn0_slot = var_qbc_rdn0;
        *var_qbc_rdn1_slot = var_qbc_rdn1;
        *var_qbc_rdn10_slot = var_qbc_rdn10;
        *var_qbc_rdn11_slot = var_qbc_rdn11;
        *var_qbc_rdn12_slot = var_qbc_rdn12;
        *var_qbc_rdn13_slot = var_qbc_rdn13;
        *var_qbc_rdn2_slot = var_qbc_rdn2;
        *var_qbc_rdn3_slot = var_qbc_rdn3;
        *var_qbc_rdn4_slot = var_qbc_rdn4;
        *var_qbc_rdn5_slot = var_qbc_rdn5;
        *var_qbc_rdn6_slot = var_qbc_rdn6;
        *var_qbc_rdn7_slot = var_qbc_rdn7;
        *var_qbc_rdn8_slot = var_qbc_rdn8;
        *var_qbc_rdn9_slot = var_qbc_rdn9;
        *var_qbc_rv_slot = var_qbc_rv;
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
        *var_qbcp_rdn0_slot = var_qbcp_rdn0;
        *var_qbcp_rdn1_slot = var_qbcp_rdn1;
        *var_qbcp_rdn10_slot = var_qbcp_rdn10;
        *var_qbcp_rdn11_slot = var_qbcp_rdn11;
        *var_qbcp_rdn12_slot = var_qbcp_rdn12;
        *var_qbcp_rdn13_slot = var_qbcp_rdn13;
        *var_qbcp_rdn2_slot = var_qbcp_rdn2;
        *var_qbcp_rdn3_slot = var_qbcp_rdn3;
        *var_qbcp_rdn4_slot = var_qbcp_rdn4;
        *var_qbcp_rdn5_slot = var_qbcp_rdn5;
        *var_qbcp_rdn6_slot = var_qbcp_rdn6;
        *var_qbcp_rdn7_slot = var_qbcp_rdn7;
        *var_qbcp_rdn8_slot = var_qbcp_rdn8;
        *var_qbcp_rdn9_slot = var_qbcp_rdn9;
        *var_qbcp_rv_slot = var_qbcp_rv;
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
        *var_qbcx_rdn0_slot = var_qbcx_rdn0;
        *var_qbcx_rdn1_slot = var_qbcx_rdn1;
        *var_qbcx_rdn10_slot = var_qbcx_rdn10;
        *var_qbcx_rdn11_slot = var_qbcx_rdn11;
        *var_qbcx_rdn12_slot = var_qbcx_rdn12;
        *var_qbcx_rdn13_slot = var_qbcx_rdn13;
        *var_qbcx_rdn2_slot = var_qbcx_rdn2;
        *var_qbcx_rdn3_slot = var_qbcx_rdn3;
        *var_qbcx_rdn4_slot = var_qbcx_rdn4;
        *var_qbcx_rdn5_slot = var_qbcx_rdn5;
        *var_qbcx_rdn6_slot = var_qbcx_rdn6;
        *var_qbcx_rdn7_slot = var_qbcx_rdn7;
        *var_qbcx_rdn8_slot = var_qbcx_rdn8;
        *var_qbcx_rdn9_slot = var_qbcx_rdn9;
        *var_qbcx_rv_slot = var_qbcx_rv;
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
        *var_qbe_rdn0_slot = var_qbe_rdn0;
        *var_qbe_rdn1_slot = var_qbe_rdn1;
        *var_qbe_rdn10_slot = var_qbe_rdn10;
        *var_qbe_rdn11_slot = var_qbe_rdn11;
        *var_qbe_rdn12_slot = var_qbe_rdn12;
        *var_qbe_rdn13_slot = var_qbe_rdn13;
        *var_qbe_rdn2_slot = var_qbe_rdn2;
        *var_qbe_rdn3_slot = var_qbe_rdn3;
        *var_qbe_rdn4_slot = var_qbe_rdn4;
        *var_qbe_rdn5_slot = var_qbe_rdn5;
        *var_qbe_rdn6_slot = var_qbe_rdn6;
        *var_qbe_rdn7_slot = var_qbe_rdn7;
        *var_qbe_rdn8_slot = var_qbe_rdn8;
        *var_qbe_rdn9_slot = var_qbe_rdn9;
        *var_qbe_rv_slot = var_qbe_rv;
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
        *var_qbep_rdn0_slot = var_qbep_rdn0;
        *var_qbep_rdn1_slot = var_qbep_rdn1;
        *var_qbep_rdn10_slot = var_qbep_rdn10;
        *var_qbep_rdn11_slot = var_qbep_rdn11;
        *var_qbep_rdn12_slot = var_qbep_rdn12;
        *var_qbep_rdn13_slot = var_qbep_rdn13;
        *var_qbep_rdn2_slot = var_qbep_rdn2;
        *var_qbep_rdn3_slot = var_qbep_rdn3;
        *var_qbep_rdn4_slot = var_qbep_rdn4;
        *var_qbep_rdn5_slot = var_qbep_rdn5;
        *var_qbep_rdn6_slot = var_qbep_rdn6;
        *var_qbep_rdn7_slot = var_qbep_rdn7;
        *var_qbep_rdn8_slot = var_qbep_rdn8;
        *var_qbep_rdn9_slot = var_qbep_rdn9;
        *var_qbep_rv_slot = var_qbep_rv;
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
        *var_qbex_rdn0_slot = var_qbex_rdn0;
        *var_qbex_rdn1_slot = var_qbex_rdn1;
        *var_qbex_rdn10_slot = var_qbex_rdn10;
        *var_qbex_rdn11_slot = var_qbex_rdn11;
        *var_qbex_rdn12_slot = var_qbex_rdn12;
        *var_qbex_rdn13_slot = var_qbex_rdn13;
        *var_qbex_rdn2_slot = var_qbex_rdn2;
        *var_qbex_rdn3_slot = var_qbex_rdn3;
        *var_qbex_rdn4_slot = var_qbex_rdn4;
        *var_qbex_rdn5_slot = var_qbex_rdn5;
        *var_qbex_rdn6_slot = var_qbex_rdn6;
        *var_qbex_rdn7_slot = var_qbex_rdn7;
        *var_qbex_rdn8_slot = var_qbex_rdn8;
        *var_qbex_rdn9_slot = var_qbex_rdn9;
        *var_qbex_rv_slot = var_qbex_rv;
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
        *var_qcth_rdn0_slot = var_qcth_rdn0;
        *var_qcth_rdn1_slot = var_qcth_rdn1;
        *var_qcth_rdn10_slot = var_qcth_rdn10;
        *var_qcth_rdn11_slot = var_qcth_rdn11;
        *var_qcth_rdn12_slot = var_qcth_rdn12;
        *var_qcth_rdn13_slot = var_qcth_rdn13;
        *var_qcth_rdn2_slot = var_qcth_rdn2;
        *var_qcth_rdn3_slot = var_qcth_rdn3;
        *var_qcth_rdn4_slot = var_qcth_rdn4;
        *var_qcth_rdn5_slot = var_qcth_rdn5;
        *var_qcth_rdn6_slot = var_qcth_rdn6;
        *var_qcth_rdn7_slot = var_qcth_rdn7;
        *var_qcth_rdn8_slot = var_qcth_rdn8;
        *var_qcth_rdn9_slot = var_qcth_rdn9;
        *var_qcth_rv_slot = var_qcth_rv;
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
        *var_tff_rdn0_slot = var_tff_rdn0;
        *var_tff_rdn1_slot = var_tff_rdn1;
        *var_tff_rdn10_slot = var_tff_rdn10;
        *var_tff_rdn11_slot = var_tff_rdn11;
        *var_tff_rdn12_slot = var_tff_rdn12;
        *var_tff_rdn13_slot = var_tff_rdn13;
        *var_tff_rdn2_slot = var_tff_rdn2;
        *var_tff_rdn3_slot = var_tff_rdn3;
        *var_tff_rdn4_slot = var_tff_rdn4;
        *var_tff_rdn5_slot = var_tff_rdn5;
        *var_tff_rdn6_slot = var_tff_rdn6;
        *var_tff_rdn7_slot = var_tff_rdn7;
        *var_tff_rdn8_slot = var_tff_rdn8;
        *var_tff_rdn9_slot = var_tff_rdn9;
        *var_tff_rv_slot = var_tff_rv;
    }

    pub(super) fn stamp_reactive_block_29(
        var_vbictype: f64,
        var_vbictype_dn0: f64,
        var_vbictype_dn1: f64,
        var_vbictype_dn10: f64,
        var_vbictype_dn11: f64,
        var_vbictype_dn12: f64,
        var_vbictype_dn13: f64,
        var_vbictype_dn2: f64,
        var_vbictype_dn3: f64,
        var_vbictype_dn4: f64,
        var_vbictype_dn5: f64,
        var_vbictype_dn6: f64,
        var_vbictype_dn7: f64,
        var_vbictype_dn8: f64,
        var_vbictype_dn9: f64,
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
        var_qbc_rdn0_slot: &mut f64,
        var_qbc_rdn1_slot: &mut f64,
        var_qbc_rdn10_slot: &mut f64,
        var_qbc_rdn11_slot: &mut f64,
        var_qbc_rdn12_slot: &mut f64,
        var_qbc_rdn13_slot: &mut f64,
        var_qbc_rdn2_slot: &mut f64,
        var_qbc_rdn3_slot: &mut f64,
        var_qbc_rdn4_slot: &mut f64,
        var_qbc_rdn5_slot: &mut f64,
        var_qbc_rdn6_slot: &mut f64,
        var_qbc_rdn7_slot: &mut f64,
        var_qbc_rdn8_slot: &mut f64,
        var_qbc_rdn9_slot: &mut f64,
        var_qbc_rv_slot: &mut f64,
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
        var_qbcp_rdn0_slot: &mut f64,
        var_qbcp_rdn1_slot: &mut f64,
        var_qbcp_rdn10_slot: &mut f64,
        var_qbcp_rdn11_slot: &mut f64,
        var_qbcp_rdn12_slot: &mut f64,
        var_qbcp_rdn13_slot: &mut f64,
        var_qbcp_rdn2_slot: &mut f64,
        var_qbcp_rdn3_slot: &mut f64,
        var_qbcp_rdn4_slot: &mut f64,
        var_qbcp_rdn5_slot: &mut f64,
        var_qbcp_rdn6_slot: &mut f64,
        var_qbcp_rdn7_slot: &mut f64,
        var_qbcp_rdn8_slot: &mut f64,
        var_qbcp_rdn9_slot: &mut f64,
        var_qbcp_rv_slot: &mut f64,
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
        var_qbcx_rdn0_slot: &mut f64,
        var_qbcx_rdn1_slot: &mut f64,
        var_qbcx_rdn10_slot: &mut f64,
        var_qbcx_rdn11_slot: &mut f64,
        var_qbcx_rdn12_slot: &mut f64,
        var_qbcx_rdn13_slot: &mut f64,
        var_qbcx_rdn2_slot: &mut f64,
        var_qbcx_rdn3_slot: &mut f64,
        var_qbcx_rdn4_slot: &mut f64,
        var_qbcx_rdn5_slot: &mut f64,
        var_qbcx_rdn6_slot: &mut f64,
        var_qbcx_rdn7_slot: &mut f64,
        var_qbcx_rdn8_slot: &mut f64,
        var_qbcx_rdn9_slot: &mut f64,
        var_qbcx_rv_slot: &mut f64,
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
        var_qbep_rdn0_slot: &mut f64,
        var_qbep_rdn1_slot: &mut f64,
        var_qbep_rdn10_slot: &mut f64,
        var_qbep_rdn11_slot: &mut f64,
        var_qbep_rdn12_slot: &mut f64,
        var_qbep_rdn13_slot: &mut f64,
        var_qbep_rdn2_slot: &mut f64,
        var_qbep_rdn3_slot: &mut f64,
        var_qbep_rdn4_slot: &mut f64,
        var_qbep_rdn5_slot: &mut f64,
        var_qbep_rdn6_slot: &mut f64,
        var_qbep_rdn7_slot: &mut f64,
        var_qbep_rdn8_slot: &mut f64,
        var_qbep_rdn9_slot: &mut f64,
        var_qbep_rv_slot: &mut f64,
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
        var_qcth_rdn0_slot: &mut f64,
        var_qcth_rdn1_slot: &mut f64,
        var_qcth_rdn10_slot: &mut f64,
        var_qcth_rdn11_slot: &mut f64,
        var_qcth_rdn12_slot: &mut f64,
        var_qcth_rdn13_slot: &mut f64,
        var_qcth_rdn2_slot: &mut f64,
        var_qcth_rdn3_slot: &mut f64,
        var_qcth_rdn4_slot: &mut f64,
        var_qcth_rdn5_slot: &mut f64,
        var_qcth_rdn6_slot: &mut f64,
        var_qcth_rdn7_slot: &mut f64,
        var_qcth_rdn8_slot: &mut f64,
        var_qcth_rdn9_slot: &mut f64,
        var_qcth_rv_slot: &mut f64,
    ) {
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
        let mut var_qbc_rdn0: f64 = *var_qbc_rdn0_slot;
        let mut var_qbc_rdn1: f64 = *var_qbc_rdn1_slot;
        let mut var_qbc_rdn10: f64 = *var_qbc_rdn10_slot;
        let mut var_qbc_rdn11: f64 = *var_qbc_rdn11_slot;
        let mut var_qbc_rdn12: f64 = *var_qbc_rdn12_slot;
        let mut var_qbc_rdn13: f64 = *var_qbc_rdn13_slot;
        let mut var_qbc_rdn2: f64 = *var_qbc_rdn2_slot;
        let mut var_qbc_rdn3: f64 = *var_qbc_rdn3_slot;
        let mut var_qbc_rdn4: f64 = *var_qbc_rdn4_slot;
        let mut var_qbc_rdn5: f64 = *var_qbc_rdn5_slot;
        let mut var_qbc_rdn6: f64 = *var_qbc_rdn6_slot;
        let mut var_qbc_rdn7: f64 = *var_qbc_rdn7_slot;
        let mut var_qbc_rdn8: f64 = *var_qbc_rdn8_slot;
        let mut var_qbc_rdn9: f64 = *var_qbc_rdn9_slot;
        let mut var_qbc_rv: f64 = *var_qbc_rv_slot;
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
        let mut var_qbcp_rdn0: f64 = *var_qbcp_rdn0_slot;
        let mut var_qbcp_rdn1: f64 = *var_qbcp_rdn1_slot;
        let mut var_qbcp_rdn10: f64 = *var_qbcp_rdn10_slot;
        let mut var_qbcp_rdn11: f64 = *var_qbcp_rdn11_slot;
        let mut var_qbcp_rdn12: f64 = *var_qbcp_rdn12_slot;
        let mut var_qbcp_rdn13: f64 = *var_qbcp_rdn13_slot;
        let mut var_qbcp_rdn2: f64 = *var_qbcp_rdn2_slot;
        let mut var_qbcp_rdn3: f64 = *var_qbcp_rdn3_slot;
        let mut var_qbcp_rdn4: f64 = *var_qbcp_rdn4_slot;
        let mut var_qbcp_rdn5: f64 = *var_qbcp_rdn5_slot;
        let mut var_qbcp_rdn6: f64 = *var_qbcp_rdn6_slot;
        let mut var_qbcp_rdn7: f64 = *var_qbcp_rdn7_slot;
        let mut var_qbcp_rdn8: f64 = *var_qbcp_rdn8_slot;
        let mut var_qbcp_rdn9: f64 = *var_qbcp_rdn9_slot;
        let mut var_qbcp_rv: f64 = *var_qbcp_rv_slot;
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
        let mut var_qbcx_rdn0: f64 = *var_qbcx_rdn0_slot;
        let mut var_qbcx_rdn1: f64 = *var_qbcx_rdn1_slot;
        let mut var_qbcx_rdn10: f64 = *var_qbcx_rdn10_slot;
        let mut var_qbcx_rdn11: f64 = *var_qbcx_rdn11_slot;
        let mut var_qbcx_rdn12: f64 = *var_qbcx_rdn12_slot;
        let mut var_qbcx_rdn13: f64 = *var_qbcx_rdn13_slot;
        let mut var_qbcx_rdn2: f64 = *var_qbcx_rdn2_slot;
        let mut var_qbcx_rdn3: f64 = *var_qbcx_rdn3_slot;
        let mut var_qbcx_rdn4: f64 = *var_qbcx_rdn4_slot;
        let mut var_qbcx_rdn5: f64 = *var_qbcx_rdn5_slot;
        let mut var_qbcx_rdn6: f64 = *var_qbcx_rdn6_slot;
        let mut var_qbcx_rdn7: f64 = *var_qbcx_rdn7_slot;
        let mut var_qbcx_rdn8: f64 = *var_qbcx_rdn8_slot;
        let mut var_qbcx_rdn9: f64 = *var_qbcx_rdn9_slot;
        let mut var_qbcx_rv: f64 = *var_qbcx_rv_slot;
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
        let mut var_qbep_rdn0: f64 = *var_qbep_rdn0_slot;
        let mut var_qbep_rdn1: f64 = *var_qbep_rdn1_slot;
        let mut var_qbep_rdn10: f64 = *var_qbep_rdn10_slot;
        let mut var_qbep_rdn11: f64 = *var_qbep_rdn11_slot;
        let mut var_qbep_rdn12: f64 = *var_qbep_rdn12_slot;
        let mut var_qbep_rdn13: f64 = *var_qbep_rdn13_slot;
        let mut var_qbep_rdn2: f64 = *var_qbep_rdn2_slot;
        let mut var_qbep_rdn3: f64 = *var_qbep_rdn3_slot;
        let mut var_qbep_rdn4: f64 = *var_qbep_rdn4_slot;
        let mut var_qbep_rdn5: f64 = *var_qbep_rdn5_slot;
        let mut var_qbep_rdn6: f64 = *var_qbep_rdn6_slot;
        let mut var_qbep_rdn7: f64 = *var_qbep_rdn7_slot;
        let mut var_qbep_rdn8: f64 = *var_qbep_rdn8_slot;
        let mut var_qbep_rdn9: f64 = *var_qbep_rdn9_slot;
        let mut var_qbep_rv: f64 = *var_qbep_rv_slot;
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
        let mut var_qcth_rdn0: f64 = *var_qcth_rdn0_slot;
        let mut var_qcth_rdn1: f64 = *var_qcth_rdn1_slot;
        let mut var_qcth_rdn10: f64 = *var_qcth_rdn10_slot;
        let mut var_qcth_rdn11: f64 = *var_qcth_rdn11_slot;
        let mut var_qcth_rdn12: f64 = *var_qcth_rdn12_slot;
        let mut var_qcth_rdn13: f64 = *var_qcth_rdn13_slot;
        let mut var_qcth_rdn2: f64 = *var_qcth_rdn2_slot;
        let mut var_qcth_rdn3: f64 = *var_qcth_rdn3_slot;
        let mut var_qcth_rdn4: f64 = *var_qcth_rdn4_slot;
        let mut var_qcth_rdn5: f64 = *var_qcth_rdn5_slot;
        let mut var_qcth_rdn6: f64 = *var_qcth_rdn6_slot;
        let mut var_qcth_rdn7: f64 = *var_qcth_rdn7_slot;
        let mut var_qcth_rdn8: f64 = *var_qcth_rdn8_slot;
        let mut var_qcth_rdn9: f64 = *var_qcth_rdn9_slot;
        let mut var_qcth_rv: f64 = *var_qcth_rv_slot;

        let assign5690_e6242: f64 = var_vbictype;
        let assign5690_e6244: f64 = (assign5690_e6242 * var_qbc);
        var_qbc = assign5690_e6244;
        var_qbc_dn0 = ((var_vbictype_dn0 * var_qbc) + (assign5690_e6242 * var_qbc_dn0));
        var_qbc_dn1 = ((var_vbictype_dn1 * var_qbc) + (assign5690_e6242 * var_qbc_dn1));
        var_qbc_dn2 = ((var_vbictype_dn2 * var_qbc) + (assign5690_e6242 * var_qbc_dn2));
        var_qbc_dn3 = ((var_vbictype_dn3 * var_qbc) + (assign5690_e6242 * var_qbc_dn3));
        var_qbc_dn4 = ((var_vbictype_dn4 * var_qbc) + (assign5690_e6242 * var_qbc_dn4));
        var_qbc_dn5 = ((var_vbictype_dn5 * var_qbc) + (assign5690_e6242 * var_qbc_dn5));
        var_qbc_dn6 = ((var_vbictype_dn6 * var_qbc) + (assign5690_e6242 * var_qbc_dn6));
        var_qbc_dn7 = ((var_vbictype_dn7 * var_qbc) + (assign5690_e6242 * var_qbc_dn7));
        var_qbc_dn8 = ((var_vbictype_dn8 * var_qbc) + (assign5690_e6242 * var_qbc_dn8));
        var_qbc_dn9 = ((var_vbictype_dn9 * var_qbc) + (assign5690_e6242 * var_qbc_dn9));
        var_qbc_dn10 = ((var_vbictype_dn10 * var_qbc) + (assign5690_e6242 * var_qbc_dn10));
        var_qbc_dn11 = ((var_vbictype_dn11 * var_qbc) + (assign5690_e6242 * var_qbc_dn11));
        var_qbc_dn12 = ((var_vbictype_dn12 * var_qbc) + (assign5690_e6242 * var_qbc_dn12));
        var_qbc_dn13 = ((var_vbictype_dn13 * var_qbc) + (assign5690_e6242 * var_qbc_dn13));
        var_qbc_rv = 0.0;
        var_qbc_rdn0 = 0.0;
        var_qbc_rdn1 = 0.0;
        var_qbc_rdn2 = 0.0;
        var_qbc_rdn3 = 0.0;
        var_qbc_rdn4 = 0.0;
        var_qbc_rdn5 = 0.0;
        var_qbc_rdn6 = 0.0;
        var_qbc_rdn7 = 0.0;
        var_qbc_rdn8 = 0.0;
        var_qbc_rdn9 = 0.0;
        var_qbc_rdn10 = 0.0;
        var_qbc_rdn11 = 0.0;
        var_qbc_rdn12 = 0.0;
        var_qbc_rdn13 = 0.0;

        let assign5700_e6247: f64 = var_vbictype;
        let assign5700_e6249: f64 = (assign5700_e6247 * var_qbcx);
        var_qbcx = assign5700_e6249;
        var_qbcx_dn0 = ((var_vbictype_dn0 * var_qbcx) + (assign5700_e6247 * var_qbcx_dn0));
        var_qbcx_dn1 = ((var_vbictype_dn1 * var_qbcx) + (assign5700_e6247 * var_qbcx_dn1));
        var_qbcx_dn2 = ((var_vbictype_dn2 * var_qbcx) + (assign5700_e6247 * var_qbcx_dn2));
        var_qbcx_dn3 = ((var_vbictype_dn3 * var_qbcx) + (assign5700_e6247 * var_qbcx_dn3));
        var_qbcx_dn4 = ((var_vbictype_dn4 * var_qbcx) + (assign5700_e6247 * var_qbcx_dn4));
        var_qbcx_dn5 = ((var_vbictype_dn5 * var_qbcx) + (assign5700_e6247 * var_qbcx_dn5));
        var_qbcx_dn6 = ((var_vbictype_dn6 * var_qbcx) + (assign5700_e6247 * var_qbcx_dn6));
        var_qbcx_dn7 = ((var_vbictype_dn7 * var_qbcx) + (assign5700_e6247 * var_qbcx_dn7));
        var_qbcx_dn8 = ((var_vbictype_dn8 * var_qbcx) + (assign5700_e6247 * var_qbcx_dn8));
        var_qbcx_dn9 = ((var_vbictype_dn9 * var_qbcx) + (assign5700_e6247 * var_qbcx_dn9));
        var_qbcx_dn10 = ((var_vbictype_dn10 * var_qbcx) + (assign5700_e6247 * var_qbcx_dn10));
        var_qbcx_dn11 = ((var_vbictype_dn11 * var_qbcx) + (assign5700_e6247 * var_qbcx_dn11));
        var_qbcx_dn12 = ((var_vbictype_dn12 * var_qbcx) + (assign5700_e6247 * var_qbcx_dn12));
        var_qbcx_dn13 = ((var_vbictype_dn13 * var_qbcx) + (assign5700_e6247 * var_qbcx_dn13));
        var_qbcx_rv = 0.0;
        var_qbcx_rdn0 = 0.0;
        var_qbcx_rdn1 = 0.0;
        var_qbcx_rdn2 = 0.0;
        var_qbcx_rdn3 = 0.0;
        var_qbcx_rdn4 = 0.0;
        var_qbcx_rdn5 = 0.0;
        var_qbcx_rdn6 = 0.0;
        var_qbcx_rdn7 = 0.0;
        var_qbcx_rdn8 = 0.0;
        var_qbcx_rdn9 = 0.0;
        var_qbcx_rdn10 = 0.0;
        var_qbcx_rdn11 = 0.0;
        var_qbcx_rdn12 = 0.0;
        var_qbcx_rdn13 = 0.0;

        let assign5710_e6252: f64 = var_vbictype;
        let assign5710_e6254: f64 = (assign5710_e6252 * var_qbep);
        var_qbep = assign5710_e6254;
        var_qbep_dn0 = ((var_vbictype_dn0 * var_qbep) + (assign5710_e6252 * var_qbep_dn0));
        var_qbep_dn1 = ((var_vbictype_dn1 * var_qbep) + (assign5710_e6252 * var_qbep_dn1));
        var_qbep_dn2 = ((var_vbictype_dn2 * var_qbep) + (assign5710_e6252 * var_qbep_dn2));
        var_qbep_dn3 = ((var_vbictype_dn3 * var_qbep) + (assign5710_e6252 * var_qbep_dn3));
        var_qbep_dn4 = ((var_vbictype_dn4 * var_qbep) + (assign5710_e6252 * var_qbep_dn4));
        var_qbep_dn5 = ((var_vbictype_dn5 * var_qbep) + (assign5710_e6252 * var_qbep_dn5));
        var_qbep_dn6 = ((var_vbictype_dn6 * var_qbep) + (assign5710_e6252 * var_qbep_dn6));
        var_qbep_dn7 = ((var_vbictype_dn7 * var_qbep) + (assign5710_e6252 * var_qbep_dn7));
        var_qbep_dn8 = ((var_vbictype_dn8 * var_qbep) + (assign5710_e6252 * var_qbep_dn8));
        var_qbep_dn9 = ((var_vbictype_dn9 * var_qbep) + (assign5710_e6252 * var_qbep_dn9));
        var_qbep_dn10 = ((var_vbictype_dn10 * var_qbep) + (assign5710_e6252 * var_qbep_dn10));
        var_qbep_dn11 = ((var_vbictype_dn11 * var_qbep) + (assign5710_e6252 * var_qbep_dn11));
        var_qbep_dn12 = ((var_vbictype_dn12 * var_qbep) + (assign5710_e6252 * var_qbep_dn12));
        var_qbep_dn13 = ((var_vbictype_dn13 * var_qbep) + (assign5710_e6252 * var_qbep_dn13));
        var_qbep_rv = 0.0;
        var_qbep_rdn0 = 0.0;
        var_qbep_rdn1 = 0.0;
        var_qbep_rdn2 = 0.0;
        var_qbep_rdn3 = 0.0;
        var_qbep_rdn4 = 0.0;
        var_qbep_rdn5 = 0.0;
        var_qbep_rdn6 = 0.0;
        var_qbep_rdn7 = 0.0;
        var_qbep_rdn8 = 0.0;
        var_qbep_rdn9 = 0.0;
        var_qbep_rdn10 = 0.0;
        var_qbep_rdn11 = 0.0;
        var_qbep_rdn12 = 0.0;
        var_qbep_rdn13 = 0.0;

        let assign5740_e6263: f64 = var_vbictype;
        let assign5740_e6265: f64 = (assign5740_e6263 * var_qbcp);
        var_qbcp = assign5740_e6265;
        var_qbcp_dn0 = ((var_vbictype_dn0 * var_qbcp) + (assign5740_e6263 * var_qbcp_dn0));
        var_qbcp_dn1 = ((var_vbictype_dn1 * var_qbcp) + (assign5740_e6263 * var_qbcp_dn1));
        var_qbcp_dn2 = ((var_vbictype_dn2 * var_qbcp) + (assign5740_e6263 * var_qbcp_dn2));
        var_qbcp_dn3 = ((var_vbictype_dn3 * var_qbcp) + (assign5740_e6263 * var_qbcp_dn3));
        var_qbcp_dn4 = ((var_vbictype_dn4 * var_qbcp) + (assign5740_e6263 * var_qbcp_dn4));
        var_qbcp_dn5 = ((var_vbictype_dn5 * var_qbcp) + (assign5740_e6263 * var_qbcp_dn5));
        var_qbcp_dn6 = ((var_vbictype_dn6 * var_qbcp) + (assign5740_e6263 * var_qbcp_dn6));
        var_qbcp_dn7 = ((var_vbictype_dn7 * var_qbcp) + (assign5740_e6263 * var_qbcp_dn7));
        var_qbcp_dn8 = ((var_vbictype_dn8 * var_qbcp) + (assign5740_e6263 * var_qbcp_dn8));
        var_qbcp_dn9 = ((var_vbictype_dn9 * var_qbcp) + (assign5740_e6263 * var_qbcp_dn9));
        var_qbcp_dn10 = ((var_vbictype_dn10 * var_qbcp) + (assign5740_e6263 * var_qbcp_dn10));
        var_qbcp_dn11 = ((var_vbictype_dn11 * var_qbcp) + (assign5740_e6263 * var_qbcp_dn11));
        var_qbcp_dn12 = ((var_vbictype_dn12 * var_qbcp) + (assign5740_e6263 * var_qbcp_dn12));
        var_qbcp_dn13 = ((var_vbictype_dn13 * var_qbcp) + (assign5740_e6263 * var_qbcp_dn13));
        var_qbcp_rv = 0.0;
        var_qbcp_rdn0 = 0.0;
        var_qbcp_rdn1 = 0.0;
        var_qbcp_rdn2 = 0.0;
        var_qbcp_rdn3 = 0.0;
        var_qbcp_rdn4 = 0.0;
        var_qbcp_rdn5 = 0.0;
        var_qbcp_rdn6 = 0.0;
        var_qbcp_rdn7 = 0.0;
        var_qbcp_rdn8 = 0.0;
        var_qbcp_rdn9 = 0.0;
        var_qbcp_rdn10 = 0.0;
        var_qbcp_rdn11 = 0.0;
        var_qbcp_rdn12 = 0.0;
        var_qbcp_rdn13 = 0.0;

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
        var_qcth_rv = 0.0;
        var_qcth_rdn0 = 0.0;
        var_qcth_rdn1 = 0.0;
        var_qcth_rdn2 = 0.0;
        var_qcth_rdn3 = 0.0;
        var_qcth_rdn4 = 0.0;
        var_qcth_rdn5 = 0.0;
        var_qcth_rdn6 = 0.0;
        var_qcth_rdn7 = 0.0;
        var_qcth_rdn8 = 0.0;
        var_qcth_rdn9 = 0.0;
        var_qcth_rdn10 = 0.0;
        var_qcth_rdn11 = 0.0;
        var_qcth_rdn12 = 0.0;
        var_qcth_rdn13 = 0.0;


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
        *var_qbc_rdn0_slot = var_qbc_rdn0;
        *var_qbc_rdn1_slot = var_qbc_rdn1;
        *var_qbc_rdn10_slot = var_qbc_rdn10;
        *var_qbc_rdn11_slot = var_qbc_rdn11;
        *var_qbc_rdn12_slot = var_qbc_rdn12;
        *var_qbc_rdn13_slot = var_qbc_rdn13;
        *var_qbc_rdn2_slot = var_qbc_rdn2;
        *var_qbc_rdn3_slot = var_qbc_rdn3;
        *var_qbc_rdn4_slot = var_qbc_rdn4;
        *var_qbc_rdn5_slot = var_qbc_rdn5;
        *var_qbc_rdn6_slot = var_qbc_rdn6;
        *var_qbc_rdn7_slot = var_qbc_rdn7;
        *var_qbc_rdn8_slot = var_qbc_rdn8;
        *var_qbc_rdn9_slot = var_qbc_rdn9;
        *var_qbc_rv_slot = var_qbc_rv;
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
        *var_qbcp_rdn0_slot = var_qbcp_rdn0;
        *var_qbcp_rdn1_slot = var_qbcp_rdn1;
        *var_qbcp_rdn10_slot = var_qbcp_rdn10;
        *var_qbcp_rdn11_slot = var_qbcp_rdn11;
        *var_qbcp_rdn12_slot = var_qbcp_rdn12;
        *var_qbcp_rdn13_slot = var_qbcp_rdn13;
        *var_qbcp_rdn2_slot = var_qbcp_rdn2;
        *var_qbcp_rdn3_slot = var_qbcp_rdn3;
        *var_qbcp_rdn4_slot = var_qbcp_rdn4;
        *var_qbcp_rdn5_slot = var_qbcp_rdn5;
        *var_qbcp_rdn6_slot = var_qbcp_rdn6;
        *var_qbcp_rdn7_slot = var_qbcp_rdn7;
        *var_qbcp_rdn8_slot = var_qbcp_rdn8;
        *var_qbcp_rdn9_slot = var_qbcp_rdn9;
        *var_qbcp_rv_slot = var_qbcp_rv;
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
        *var_qbcx_rdn0_slot = var_qbcx_rdn0;
        *var_qbcx_rdn1_slot = var_qbcx_rdn1;
        *var_qbcx_rdn10_slot = var_qbcx_rdn10;
        *var_qbcx_rdn11_slot = var_qbcx_rdn11;
        *var_qbcx_rdn12_slot = var_qbcx_rdn12;
        *var_qbcx_rdn13_slot = var_qbcx_rdn13;
        *var_qbcx_rdn2_slot = var_qbcx_rdn2;
        *var_qbcx_rdn3_slot = var_qbcx_rdn3;
        *var_qbcx_rdn4_slot = var_qbcx_rdn4;
        *var_qbcx_rdn5_slot = var_qbcx_rdn5;
        *var_qbcx_rdn6_slot = var_qbcx_rdn6;
        *var_qbcx_rdn7_slot = var_qbcx_rdn7;
        *var_qbcx_rdn8_slot = var_qbcx_rdn8;
        *var_qbcx_rdn9_slot = var_qbcx_rdn9;
        *var_qbcx_rv_slot = var_qbcx_rv;
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
        *var_qbep_rdn0_slot = var_qbep_rdn0;
        *var_qbep_rdn1_slot = var_qbep_rdn1;
        *var_qbep_rdn10_slot = var_qbep_rdn10;
        *var_qbep_rdn11_slot = var_qbep_rdn11;
        *var_qbep_rdn12_slot = var_qbep_rdn12;
        *var_qbep_rdn13_slot = var_qbep_rdn13;
        *var_qbep_rdn2_slot = var_qbep_rdn2;
        *var_qbep_rdn3_slot = var_qbep_rdn3;
        *var_qbep_rdn4_slot = var_qbep_rdn4;
        *var_qbep_rdn5_slot = var_qbep_rdn5;
        *var_qbep_rdn6_slot = var_qbep_rdn6;
        *var_qbep_rdn7_slot = var_qbep_rdn7;
        *var_qbep_rdn8_slot = var_qbep_rdn8;
        *var_qbep_rdn9_slot = var_qbep_rdn9;
        *var_qbep_rv_slot = var_qbep_rv;
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
        *var_qcth_rdn0_slot = var_qcth_rdn0;
        *var_qcth_rdn1_slot = var_qcth_rdn1;
        *var_qcth_rdn10_slot = var_qcth_rdn10;
        *var_qcth_rdn11_slot = var_qcth_rdn11;
        *var_qcth_rdn12_slot = var_qcth_rdn12;
        *var_qcth_rdn13_slot = var_qcth_rdn13;
        *var_qcth_rdn2_slot = var_qcth_rdn2;
        *var_qcth_rdn3_slot = var_qcth_rdn3;
        *var_qcth_rdn4_slot = var_qcth_rdn4;
        *var_qcth_rdn5_slot = var_qcth_rdn5;
        *var_qcth_rdn6_slot = var_qcth_rdn6;
        *var_qcth_rdn7_slot = var_qcth_rdn7;
        *var_qcth_rdn8_slot = var_qcth_rdn8;
        *var_qcth_rdn9_slot = var_qcth_rdn9;
        *var_qcth_rv_slot = var_qcth_rv;
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
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
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
        let eq20_e159_q: f64 = var_qbe;
        let eq20_reactive_node_derivatives: [f64; 14] = [var_qbe_dn0, var_qbe_dn1, var_qbe_dn2, var_qbe_dn3, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn7, var_qbe_dn8, var_qbe_dn9, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, var_qbe_dn13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &[],
            multiplicity,
        );
        let eq21_e161_q: f64 = var_qbex;
        let eq21_reactive_node_derivatives: [f64; 14] = [var_qbex_dn0, var_qbex_dn1, var_qbex_dn2, var_qbex_dn3, var_qbex_dn4, var_qbex_dn5, var_qbex_dn6, var_qbex_dn7, var_qbex_dn8, var_qbex_dn9, var_qbex_dn10, var_qbex_dn11, var_qbex_dn12, var_qbex_dn13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq21_reactive_node_derivatives,
            branches,
            &[],
            multiplicity,
        );
        let eq22_e163_q: f64 = var_qbc;
        let eq22_reactive_node_derivatives: [f64; 14] = [var_qbc_dn0, var_qbc_dn1, var_qbc_dn2, var_qbc_dn3, var_qbc_dn4, var_qbc_dn5, var_qbc_dn6, var_qbc_dn7, var_qbc_dn8, var_qbc_dn9, var_qbc_dn10, var_qbc_dn11, var_qbc_dn12, var_qbc_dn13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq22_reactive_node_derivatives,
            branches,
            &[],
            multiplicity,
        );
        let eq23_e165_q: f64 = var_qbcx;
        let eq23_reactive_node_derivatives: [f64; 14] = [var_qbcx_dn0, var_qbcx_dn1, var_qbcx_dn2, var_qbcx_dn3, var_qbcx_dn4, var_qbcx_dn5, var_qbcx_dn6, var_qbcx_dn7, var_qbcx_dn8, var_qbcx_dn9, var_qbcx_dn10, var_qbcx_dn11, var_qbcx_dn12, var_qbcx_dn13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &[],
            multiplicity,
        );
        let eq24_e167_q: f64 = var_qbep;
        let eq24_reactive_node_derivatives: [f64; 14] = [var_qbep_dn0, var_qbep_dn1, var_qbep_dn2, var_qbep_dn3, var_qbep_dn4, var_qbep_dn5, var_qbep_dn6, var_qbep_dn7, var_qbep_dn8, var_qbep_dn9, var_qbep_dn10, var_qbep_dn11, var_qbep_dn12, var_qbep_dn13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            nodes,
            &eq24_reactive_node_derivatives,
            branches,
            &[],
            multiplicity,
        );
        let eq27_e173_q: f64 = var_qbcp;
        let eq27_reactive_node_derivatives: [f64; 14] = [var_qbcp_dn0, var_qbcp_dn1, var_qbcp_dn2, var_qbcp_dn3, var_qbcp_dn4, var_qbcp_dn5, var_qbcp_dn6, var_qbcp_dn7, var_qbcp_dn8, var_qbcp_dn9, var_qbcp_dn10, var_qbcp_dn11, var_qbcp_dn12, var_qbcp_dn13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[10]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &[],
            multiplicity,
        );
        let eq30_e179_q: f64 = var_qcth;
        let eq30_reactive_node_derivatives: [f64; 14] = [var_qcth_dn0, var_qcth_dn1, var_qcth_dn2, var_qcth_dn3, var_qcth_dn4, var_qcth_dn5, var_qcth_dn6, var_qcth_dn7, var_qcth_dn8, var_qcth_dn9, var_qcth_dn10, var_qcth_dn11, var_qcth_dn12, var_qcth_dn13];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &[],
            multiplicity,
        );
    }
}
