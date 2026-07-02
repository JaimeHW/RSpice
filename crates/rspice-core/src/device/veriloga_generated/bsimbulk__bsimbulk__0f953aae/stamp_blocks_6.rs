#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_96(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31830_e42091, assign31830_e42091_d_n0, assign31830_e42091_d_n2, assign31830_e42091_d_n3, assign31830_e42091_d_n4, assign31830_e42091_d_n5, assign31830_e42091_d_n6, assign31830_e42091_d_n7, assign31830_e42091_d_n8, assign31830_e42091_d_n9, assign31830_e42091_d_n10, assign31830_e42091_d_n11, assign31830_e42091_d_n12, assign31830_e42091_d_n13, assign31830_e42091_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31830_e42075: f64 = (locals.var_dvth_dibl_1 - locals.var_dvth_temp);
        let assign31830_e42077: f64 = (assign31830_e42075 + locals.var_dvth_sce);
        let assign31830_e42079: f64 = (assign31830_e42077 + p.p961);
        let assign31830_e42081: f64 = (assign31830_e42079 + locals.var_vth0_stress_edge);
        let assign31830_e42084: f64 = (locals.var_k2edge_i + locals.var_k2_well_edge);
        let assign31830_e42086: f64 = (assign31830_e42084 * locals.var_vbsx);
        let assign31830_e42087: f64 = (assign31830_e42081 - assign31830_e42086);
        let assign31830_e42089: f64 = (assign31830_e42087 + locals.var_vth0_well_edge);
        (assign31830_e42089, (((((locals.var_dvth_dibl_1_dn0 - locals.var_dvth_temp_dn0) + locals.var_dvth_sce_dn0) + locals.var_vth0_stress_edge_dn0) - (((locals.var_k2edge_i_dn0 + locals.var_k2_well_edge_dn0) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn0))) + locals.var_vth0_well_edge_dn0), (((((locals.var_dvth_dibl_1_dn2 - locals.var_dvth_temp_dn2) + locals.var_dvth_sce_dn2) + locals.var_vth0_stress_edge_dn2) - (((locals.var_k2edge_i_dn2 + locals.var_k2_well_edge_dn2) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn2))) + locals.var_vth0_well_edge_dn2), (((((locals.var_dvth_dibl_1_dn3 - locals.var_dvth_temp_dn3) + locals.var_dvth_sce_dn3) + locals.var_vth0_stress_edge_dn3) - (((locals.var_k2edge_i_dn3 + locals.var_k2_well_edge_dn3) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn3))) + locals.var_vth0_well_edge_dn3), (((((locals.var_dvth_dibl_1_dn4 - locals.var_dvth_temp_dn4) + locals.var_dvth_sce_dn4) + locals.var_vth0_stress_edge_dn4) - (((locals.var_k2edge_i_dn4 + locals.var_k2_well_edge_dn4) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn4))) + locals.var_vth0_well_edge_dn4), (((((locals.var_dvth_dibl_1_dn5 - locals.var_dvth_temp_dn5) + locals.var_dvth_sce_dn5) + locals.var_vth0_stress_edge_dn5) - (((locals.var_k2edge_i_dn5 + locals.var_k2_well_edge_dn5) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn5))) + locals.var_vth0_well_edge_dn5), (((((locals.var_dvth_dibl_1_dn6 - locals.var_dvth_temp_dn6) + locals.var_dvth_sce_dn6) + locals.var_vth0_stress_edge_dn6) - (((locals.var_k2edge_i_dn6 + locals.var_k2_well_edge_dn6) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn6))) + locals.var_vth0_well_edge_dn6), (((((locals.var_dvth_dibl_1_dn7 - locals.var_dvth_temp_dn7) + locals.var_dvth_sce_dn7) + locals.var_vth0_stress_edge_dn7) - (((locals.var_k2edge_i_dn7 + locals.var_k2_well_edge_dn7) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn7))) + locals.var_vth0_well_edge_dn7), (((((locals.var_dvth_dibl_1_dn8 - locals.var_dvth_temp_dn8) + locals.var_dvth_sce_dn8) + locals.var_vth0_stress_edge_dn8) - (((locals.var_k2edge_i_dn8 + locals.var_k2_well_edge_dn8) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn8))) + locals.var_vth0_well_edge_dn8), (((((locals.var_dvth_dibl_1_dn9 - locals.var_dvth_temp_dn9) + locals.var_dvth_sce_dn9) + locals.var_vth0_stress_edge_dn9) - (((locals.var_k2edge_i_dn9 + locals.var_k2_well_edge_dn9) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn9))) + locals.var_vth0_well_edge_dn9), (((((locals.var_dvth_dibl_1_dn10 - locals.var_dvth_temp_dn10) + locals.var_dvth_sce_dn10) + locals.var_vth0_stress_edge_dn10) - (((locals.var_k2edge_i_dn10 + locals.var_k2_well_edge_dn10) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn10))) + locals.var_vth0_well_edge_dn10), (((((locals.var_dvth_dibl_1_dn11 - locals.var_dvth_temp_dn11) + locals.var_dvth_sce_dn11) + locals.var_vth0_stress_edge_dn11) - (((locals.var_k2edge_i_dn11 + locals.var_k2_well_edge_dn11) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn11))) + locals.var_vth0_well_edge_dn11), (((((locals.var_dvth_dibl_1_dn12 - locals.var_dvth_temp_dn12) + locals.var_dvth_sce_dn12) + locals.var_vth0_stress_edge_dn12) - (((locals.var_k2edge_i_dn12 + locals.var_k2_well_edge_dn12) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn12))) + locals.var_vth0_well_edge_dn12), (((((locals.var_dvth_dibl_1_dn13 - locals.var_dvth_temp_dn13) + locals.var_dvth_sce_dn13) + locals.var_vth0_stress_edge_dn13) - (((locals.var_k2edge_i_dn13 + locals.var_k2_well_edge_dn13) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn13))) + locals.var_vth0_well_edge_dn13), (((((locals.var_dvth_dibl_1_dn14 - locals.var_dvth_temp_dn14) + locals.var_dvth_sce_dn14) + locals.var_vth0_stress_edge_dn14) - (((locals.var_k2edge_i_dn14 + locals.var_k2_well_edge_dn14) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn14))) + locals.var_vth0_well_edge_dn14),)
    } else {
        (locals.var_vth_shift, locals.var_vth_shift_dn0, locals.var_vth_shift_dn2, locals.var_vth_shift_dn3, locals.var_vth_shift_dn4, locals.var_vth_shift_dn5, locals.var_vth_shift_dn6, locals.var_vth_shift_dn7, locals.var_vth_shift_dn8, locals.var_vth_shift_dn9, locals.var_vth_shift_dn10, locals.var_vth_shift_dn11, locals.var_vth_shift_dn12, locals.var_vth_shift_dn13, locals.var_vth_shift_dn14,)
    }
};
        locals.var_vth_shift = assign31830_e42091;
        locals.var_vth_shift_dn0 = assign31830_e42091_d_n0;
        locals.var_vth_shift_dn2 = assign31830_e42091_d_n2;
        locals.var_vth_shift_dn3 = assign31830_e42091_d_n3;
        locals.var_vth_shift_dn4 = assign31830_e42091_d_n4;
        locals.var_vth_shift_dn5 = assign31830_e42091_d_n5;
        locals.var_vth_shift_dn6 = assign31830_e42091_d_n6;
        locals.var_vth_shift_dn7 = assign31830_e42091_d_n7;
        locals.var_vth_shift_dn8 = assign31830_e42091_d_n8;
        locals.var_vth_shift_dn9 = assign31830_e42091_d_n9;
        locals.var_vth_shift_dn10 = assign31830_e42091_d_n10;
        locals.var_vth_shift_dn11 = assign31830_e42091_d_n11;
        locals.var_vth_shift_dn12 = assign31830_e42091_d_n12;
        locals.var_vth_shift_dn13 = assign31830_e42091_d_n13;
        locals.var_vth_shift_dn14 = assign31830_e42091_d_n14;

        let (assign31840_e42101, assign31840_e42101_d_n0, assign31840_e42101_d_n2, assign31840_e42101_d_n3, assign31840_e42101_d_n4, assign31840_e42101_d_n5, assign31840_e42101_d_n6, assign31840_e42101_d_n7, assign31840_e42101_d_n8, assign31840_e42101_d_n9, assign31840_e42101_d_n10, assign31840_e42101_d_n11, assign31840_e42101_d_n12, assign31840_e42101_d_n13, assign31840_e42101_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31840_e42095: f64 = (locals.var_vg_1 - locals.var_vfb);
        let assign31840_e42098: f64 = (locals.var_vth_shift * locals.var_inv_nvt);
        let assign31840_e42099: f64 = (assign31840_e42095 - assign31840_e42098);
        (assign31840_e42099, ((locals.var_vg_1_dn0 - locals.var_vfb_dn0) - ((locals.var_vth_shift_dn0 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn0))), ((locals.var_vg_1_dn2 - locals.var_vfb_dn2) - ((locals.var_vth_shift_dn2 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn2))), ((locals.var_vg_1_dn3 - locals.var_vfb_dn3) - ((locals.var_vth_shift_dn3 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn3))), ((locals.var_vg_1_dn4 - locals.var_vfb_dn4) - ((locals.var_vth_shift_dn4 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn4))), ((locals.var_vg_1_dn5 - locals.var_vfb_dn5) - ((locals.var_vth_shift_dn5 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn5))), ((locals.var_vg_1_dn6 - locals.var_vfb_dn6) - ((locals.var_vth_shift_dn6 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn6))), ((locals.var_vg_1_dn7 - locals.var_vfb_dn7) - ((locals.var_vth_shift_dn7 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn7))), ((locals.var_vg_1_dn8 - locals.var_vfb_dn8) - ((locals.var_vth_shift_dn8 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn8))), ((locals.var_vg_1_dn9 - locals.var_vfb_dn9) - ((locals.var_vth_shift_dn9 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn9))), ((locals.var_vg_1_dn10 - locals.var_vfb_dn10) - ((locals.var_vth_shift_dn10 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn10))), ((locals.var_vg_1_dn11 - locals.var_vfb_dn11) - ((locals.var_vth_shift_dn11 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn11))), ((locals.var_vg_1_dn12 - locals.var_vfb_dn12) - ((locals.var_vth_shift_dn12 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn12))), ((locals.var_vg_1_dn13 - locals.var_vfb_dn13) - ((locals.var_vth_shift_dn13 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn13))), ((locals.var_vg_1_dn14 - locals.var_vfb_dn14) - ((locals.var_vth_shift_dn14 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn14))),)
    } else {
        (locals.var_vgfb, locals.var_vgfb_dn0, locals.var_vgfb_dn2, locals.var_vgfb_dn3, locals.var_vgfb_dn4, locals.var_vgfb_dn5, locals.var_vgfb_dn6, locals.var_vgfb_dn7, locals.var_vgfb_dn8, locals.var_vgfb_dn9, locals.var_vgfb_dn10, locals.var_vgfb_dn11, locals.var_vgfb_dn12, locals.var_vgfb_dn13, locals.var_vgfb_dn14,)
    }
};
        locals.var_vgfb = assign31840_e42101;
        locals.var_vgfb_dn0 = assign31840_e42101_d_n0;
        locals.var_vgfb_dn2 = assign31840_e42101_d_n2;
        locals.var_vgfb_dn3 = assign31840_e42101_d_n3;
        locals.var_vgfb_dn4 = assign31840_e42101_d_n4;
        locals.var_vgfb_dn5 = assign31840_e42101_d_n5;
        locals.var_vgfb_dn6 = assign31840_e42101_d_n6;
        locals.var_vgfb_dn7 = assign31840_e42101_d_n7;
        locals.var_vgfb_dn8 = assign31840_e42101_d_n8;
        locals.var_vgfb_dn9 = assign31840_e42101_d_n9;
        locals.var_vgfb_dn10 = assign31840_e42101_d_n10;
        locals.var_vgfb_dn11 = assign31840_e42101_d_n11;
        locals.var_vgfb_dn12 = assign31840_e42101_d_n12;
        locals.var_vgfb_dn13 = assign31840_e42101_d_n13;
        locals.var_vgfb_dn14 = assign31840_e42101_d_n14;

        let (assign31850_e42114,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31850_e42108: f64 = (-p.p960);
        let assign31850_e42109: f64 = (locals.var_leff).powf(assign31850_e42108);
        let assign31850_e42110: f64 = (p.p959 * assign31850_e42109);
        let assign31850_e42111: f64 = (1.0 + assign31850_e42110);
        let assign31850_e42112: f64 = (p.p958 * assign31850_e42111);
        (assign31850_e42112,)
    } else {
        (locals.var_dgammaedge_i,)
    }
};
        locals.var_dgammaedge_i = assign31850_e42114;

        let (assign31860_e42129, assign31860_e42129_d_n0, assign31860_e42129_d_n2, assign31860_e42129_d_n3, assign31860_e42129_d_n4, assign31860_e42129_d_n5, assign31860_e42129_d_n6, assign31860_e42129_d_n7, assign31860_e42129_d_n8, assign31860_e42129_d_n9, assign31860_e42129_d_n10, assign31860_e42129_d_n11, assign31860_e42129_d_n12, assign31860_e42129_d_n13, assign31860_e42129_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31860_e42118: f64 = (2.0 * 1.60219e-19);
        let assign31860_e42120: f64 = (assign31860_e42118 * locals.var_epssi);
        let assign31860_e42122: f64 = (assign31860_e42120 * locals.var_ndepedge_i);
        let assign31860_e42124: f64 = (assign31860_e42122 * locals.var_inv_nvt);
        let assign31860_e42125: f64 = (assign31860_e42124).sqrt();
        let assign31860_e42127: f64 = (assign31860_e42125 / locals.var_cox);
        (assign31860_e42127, (((assign31860_e42122 * locals.var_inv_nvt_dn0) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn2) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn3) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn4) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn5) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn6) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn7) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn8) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn9) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn10) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn11) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn12) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn13) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn14) / (2.0 * assign31860_e42125)) / locals.var_cox),)
    } else {
        (locals.var_gam_edge, locals.var_gam_edge_dn0, locals.var_gam_edge_dn2, locals.var_gam_edge_dn3, locals.var_gam_edge_dn4, locals.var_gam_edge_dn5, locals.var_gam_edge_dn6, locals.var_gam_edge_dn7, locals.var_gam_edge_dn8, locals.var_gam_edge_dn9, locals.var_gam_edge_dn10, locals.var_gam_edge_dn11, locals.var_gam_edge_dn12, locals.var_gam_edge_dn13, locals.var_gam_edge_dn14,)
    }
};
        locals.var_gam_edge = assign31860_e42129;
        locals.var_gam_edge_dn0 = assign31860_e42129_d_n0;
        locals.var_gam_edge_dn2 = assign31860_e42129_d_n2;
        locals.var_gam_edge_dn3 = assign31860_e42129_d_n3;
        locals.var_gam_edge_dn4 = assign31860_e42129_d_n4;
        locals.var_gam_edge_dn5 = assign31860_e42129_d_n5;
        locals.var_gam_edge_dn6 = assign31860_e42129_d_n6;
        locals.var_gam_edge_dn7 = assign31860_e42129_d_n7;
        locals.var_gam_edge_dn8 = assign31860_e42129_d_n8;
        locals.var_gam_edge_dn9 = assign31860_e42129_d_n9;
        locals.var_gam_edge_dn10 = assign31860_e42129_d_n10;
        locals.var_gam_edge_dn11 = assign31860_e42129_d_n11;
        locals.var_gam_edge_dn12 = assign31860_e42129_d_n12;
        locals.var_gam_edge_dn13 = assign31860_e42129_d_n13;
        locals.var_gam_edge_dn14 = assign31860_e42129_d_n14;

        let (assign31870_e42137, assign31870_e42137_d_n0, assign31870_e42137_d_n2, assign31870_e42137_d_n3, assign31870_e42137_d_n4, assign31870_e42137_d_n5, assign31870_e42137_d_n6, assign31870_e42137_d_n7, assign31870_e42137_d_n8, assign31870_e42137_d_n9, assign31870_e42137_d_n10, assign31870_e42137_d_n11, assign31870_e42137_d_n12, assign31870_e42137_d_n13, assign31870_e42137_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31870_e42134: f64 = (1.0 + locals.var_dgammaedge_i);
        let assign31870_e42135: f64 = (locals.var_gam_edge * assign31870_e42134);
        (assign31870_e42135, (locals.var_gam_edge_dn0 * assign31870_e42134), (locals.var_gam_edge_dn2 * assign31870_e42134), (locals.var_gam_edge_dn3 * assign31870_e42134), (locals.var_gam_edge_dn4 * assign31870_e42134), (locals.var_gam_edge_dn5 * assign31870_e42134), (locals.var_gam_edge_dn6 * assign31870_e42134), (locals.var_gam_edge_dn7 * assign31870_e42134), (locals.var_gam_edge_dn8 * assign31870_e42134), (locals.var_gam_edge_dn9 * assign31870_e42134), (locals.var_gam_edge_dn10 * assign31870_e42134), (locals.var_gam_edge_dn11 * assign31870_e42134), (locals.var_gam_edge_dn12 * assign31870_e42134), (locals.var_gam_edge_dn13 * assign31870_e42134), (locals.var_gam_edge_dn14 * assign31870_e42134),)
    } else {
        (locals.var_gam_edge, locals.var_gam_edge_dn0, locals.var_gam_edge_dn2, locals.var_gam_edge_dn3, locals.var_gam_edge_dn4, locals.var_gam_edge_dn5, locals.var_gam_edge_dn6, locals.var_gam_edge_dn7, locals.var_gam_edge_dn8, locals.var_gam_edge_dn9, locals.var_gam_edge_dn10, locals.var_gam_edge_dn11, locals.var_gam_edge_dn12, locals.var_gam_edge_dn13, locals.var_gam_edge_dn14,)
    }
};
        locals.var_gam_edge = assign31870_e42137;
        locals.var_gam_edge_dn0 = assign31870_e42137_d_n0;
        locals.var_gam_edge_dn2 = assign31870_e42137_d_n2;
        locals.var_gam_edge_dn3 = assign31870_e42137_d_n3;
        locals.var_gam_edge_dn4 = assign31870_e42137_d_n4;
        locals.var_gam_edge_dn5 = assign31870_e42137_d_n5;
        locals.var_gam_edge_dn6 = assign31870_e42137_d_n6;
        locals.var_gam_edge_dn7 = assign31870_e42137_d_n7;
        locals.var_gam_edge_dn8 = assign31870_e42137_d_n8;
        locals.var_gam_edge_dn9 = assign31870_e42137_d_n9;
        locals.var_gam_edge_dn10 = assign31870_e42137_d_n10;
        locals.var_gam_edge_dn11 = assign31870_e42137_d_n11;
        locals.var_gam_edge_dn12 = assign31870_e42137_d_n12;
        locals.var_gam_edge_dn13 = assign31870_e42137_d_n13;
        locals.var_gam_edge_dn14 = assign31870_e42137_d_n14;

        let (assign31880_e42143, assign31880_e42143_d_n0, assign31880_e42143_d_n2, assign31880_e42143_d_n3, assign31880_e42143_d_n4, assign31880_e42143_d_n5, assign31880_e42143_d_n6, assign31880_e42143_d_n7, assign31880_e42143_d_n8, assign31880_e42143_d_n9, assign31880_e42143_d_n10, assign31880_e42143_d_n11, assign31880_e42143_d_n12, assign31880_e42143_d_n13, assign31880_e42143_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31880_e42141: f64 = (locals.var_phib_edge / locals.var_n);
        (assign31880_e42141, (((locals.var_phib_edge_dn0 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn0)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn2 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn2)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn3 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn3)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn4 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn4)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn5 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn5)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn6 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn6)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn7 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn7)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn8 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn8)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn9 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn9)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn10 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn10)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn11 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn11)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn12 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn12)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn13 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn13)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn14 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn14)) / (locals.var_n * locals.var_n)),)
    } else {
        (locals.var_phib_n_edge, locals.var_phib_n_edge_dn0, locals.var_phib_n_edge_dn2, locals.var_phib_n_edge_dn3, locals.var_phib_n_edge_dn4, locals.var_phib_n_edge_dn5, locals.var_phib_n_edge_dn6, locals.var_phib_n_edge_dn7, locals.var_phib_n_edge_dn8, locals.var_phib_n_edge_dn9, locals.var_phib_n_edge_dn10, locals.var_phib_n_edge_dn11, locals.var_phib_n_edge_dn12, locals.var_phib_n_edge_dn13, locals.var_phib_n_edge_dn14,)
    }
};
        locals.var_phib_n_edge = assign31880_e42143;
        locals.var_phib_n_edge_dn0 = assign31880_e42143_d_n0;
        locals.var_phib_n_edge_dn2 = assign31880_e42143_d_n2;
        locals.var_phib_n_edge_dn3 = assign31880_e42143_d_n3;
        locals.var_phib_n_edge_dn4 = assign31880_e42143_d_n4;
        locals.var_phib_n_edge_dn5 = assign31880_e42143_d_n5;
        locals.var_phib_n_edge_dn6 = assign31880_e42143_d_n6;
        locals.var_phib_n_edge_dn7 = assign31880_e42143_d_n7;
        locals.var_phib_n_edge_dn8 = assign31880_e42143_d_n8;
        locals.var_phib_n_edge_dn9 = assign31880_e42143_d_n9;
        locals.var_phib_n_edge_dn10 = assign31880_e42143_d_n10;
        locals.var_phib_n_edge_dn11 = assign31880_e42143_d_n11;
        locals.var_phib_n_edge_dn12 = assign31880_e42143_d_n12;
        locals.var_phib_n_edge_dn13 = assign31880_e42143_d_n13;
        locals.var_phib_n_edge_dn14 = assign31880_e42143_d_n14;

        let (assign31890_e42149, assign31890_e42149_d_n0, assign31890_e42149_d_n2, assign31890_e42149_d_n3, assign31890_e42149_d_n4, assign31890_e42149_d_n5, assign31890_e42149_d_n6, assign31890_e42149_d_n7, assign31890_e42149_d_n8, assign31890_e42149_d_n9, assign31890_e42149_d_n10, assign31890_e42149_d_n11, assign31890_e42149_d_n12, assign31890_e42149_d_n13, assign31890_e42149_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31890_e42147: f64 = 1.0;
        (assign31890_e42147, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign31890_e42149;
        locals.var_t1_dn0 = assign31890_e42149_d_n0;
        locals.var_t1_dn2 = assign31890_e42149_d_n2;
        locals.var_t1_dn3 = assign31890_e42149_d_n3;
        locals.var_t1_dn4 = assign31890_e42149_d_n4;
        locals.var_t1_dn5 = assign31890_e42149_d_n5;
        locals.var_t1_dn6 = assign31890_e42149_d_n6;
        locals.var_t1_dn7 = assign31890_e42149_d_n7;
        locals.var_t1_dn8 = assign31890_e42149_d_n8;
        locals.var_t1_dn9 = assign31890_e42149_d_n9;
        locals.var_t1_dn10 = assign31890_e42149_d_n10;
        locals.var_t1_dn11 = assign31890_e42149_d_n11;
        locals.var_t1_dn12 = assign31890_e42149_d_n12;
        locals.var_t1_dn13 = assign31890_e42149_d_n13;
        locals.var_t1_dn14 = assign31890_e42149_d_n14;

        let (assign31900_e42155, assign31900_e42155_d_n0, assign31900_e42155_d_n2, assign31900_e42155_d_n3, assign31900_e42155_d_n4, assign31900_e42155_d_n5, assign31900_e42155_d_n6, assign31900_e42155_d_n7, assign31900_e42155_d_n8, assign31900_e42155_d_n9, assign31900_e42155_d_n10, assign31900_e42155_d_n11, assign31900_e42155_d_n12, assign31900_e42155_d_n13, assign31900_e42155_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31900_e42153: f64 = (locals.var_vgfb / locals.var_t1);
        (assign31900_e42153, (((locals.var_vgfb_dn0 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn2 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn3 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn4 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn5 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn6 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn7 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn8 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn9 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn10 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn11 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn12 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn13 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn13)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn14 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_vgfbpd, locals.var_vgfbpd_dn0, locals.var_vgfbpd_dn2, locals.var_vgfbpd_dn3, locals.var_vgfbpd_dn4, locals.var_vgfbpd_dn5, locals.var_vgfbpd_dn6, locals.var_vgfbpd_dn7, locals.var_vgfbpd_dn8, locals.var_vgfbpd_dn9, locals.var_vgfbpd_dn10, locals.var_vgfbpd_dn11, locals.var_vgfbpd_dn12, locals.var_vgfbpd_dn13, locals.var_vgfbpd_dn14,)
    }
};
        locals.var_vgfbpd = assign31900_e42155;
        locals.var_vgfbpd_dn0 = assign31900_e42155_d_n0;
        locals.var_vgfbpd_dn2 = assign31900_e42155_d_n2;
        locals.var_vgfbpd_dn3 = assign31900_e42155_d_n3;
        locals.var_vgfbpd_dn4 = assign31900_e42155_d_n4;
        locals.var_vgfbpd_dn5 = assign31900_e42155_d_n5;
        locals.var_vgfbpd_dn6 = assign31900_e42155_d_n6;
        locals.var_vgfbpd_dn7 = assign31900_e42155_d_n7;
        locals.var_vgfbpd_dn8 = assign31900_e42155_d_n8;
        locals.var_vgfbpd_dn9 = assign31900_e42155_d_n9;
        locals.var_vgfbpd_dn10 = assign31900_e42155_d_n10;
        locals.var_vgfbpd_dn11 = assign31900_e42155_d_n11;
        locals.var_vgfbpd_dn12 = assign31900_e42155_d_n12;
        locals.var_vgfbpd_dn13 = assign31900_e42155_d_n13;
        locals.var_vgfbpd_dn14 = assign31900_e42155_d_n14;

        let (assign31910_e42161, assign31910_e42161_d_n0, assign31910_e42161_d_n2, assign31910_e42161_d_n3, assign31910_e42161_d_n4, assign31910_e42161_d_n5, assign31910_e42161_d_n6, assign31910_e42161_d_n7, assign31910_e42161_d_n8, assign31910_e42161_d_n9, assign31910_e42161_d_n10, assign31910_e42161_d_n11, assign31910_e42161_d_n12, assign31910_e42161_d_n13, assign31910_e42161_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31910_e42159: f64 = (locals.var_gam_edge / locals.var_t1);
        (assign31910_e42159, (((locals.var_gam_edge_dn0 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn2 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn3 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn4 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn5 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn6 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn7 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn8 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn9 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn10 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn11 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn12 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn13 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn13)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn14 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_gammapd, locals.var_gammapd_dn0, locals.var_gammapd_dn2, locals.var_gammapd_dn3, locals.var_gammapd_dn4, locals.var_gammapd_dn5, locals.var_gammapd_dn6, locals.var_gammapd_dn7, locals.var_gammapd_dn8, locals.var_gammapd_dn9, locals.var_gammapd_dn10, locals.var_gammapd_dn11, locals.var_gammapd_dn12, locals.var_gammapd_dn13, locals.var_gammapd_dn14,)
    }
};
        locals.var_gammapd = assign31910_e42161;
        locals.var_gammapd_dn0 = assign31910_e42161_d_n0;
        locals.var_gammapd_dn2 = assign31910_e42161_d_n2;
        locals.var_gammapd_dn3 = assign31910_e42161_d_n3;
        locals.var_gammapd_dn4 = assign31910_e42161_d_n4;
        locals.var_gammapd_dn5 = assign31910_e42161_d_n5;
        locals.var_gammapd_dn6 = assign31910_e42161_d_n6;
        locals.var_gammapd_dn7 = assign31910_e42161_d_n7;
        locals.var_gammapd_dn8 = assign31910_e42161_d_n8;
        locals.var_gammapd_dn9 = assign31910_e42161_d_n9;
        locals.var_gammapd_dn10 = assign31910_e42161_d_n10;
        locals.var_gammapd_dn11 = assign31910_e42161_d_n11;
        locals.var_gammapd_dn12 = assign31910_e42161_d_n12;
        locals.var_gammapd_dn13 = assign31910_e42161_d_n13;
        locals.var_gammapd_dn14 = assign31910_e42161_d_n14;

        let (assign31920_e42175, assign31920_e42175_d_n0, assign31920_e42175_d_n2, assign31920_e42175_d_n3, assign31920_e42175_d_n4, assign31920_e42175_d_n5, assign31920_e42175_d_n6, assign31920_e42175_d_n7, assign31920_e42175_d_n8, assign31920_e42175_d_n9, assign31920_e42175_d_n10, assign31920_e42175_d_n11, assign31920_e42175_d_n12, assign31920_e42175_d_n13, assign31920_e42175_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31920_e42165: f64 = (0.5 * locals.var_vgfbpd);
        let assign31920_e42170: f64 = (locals.var_gammapd / 1.4142135623730951);
        let assign31920_e42171: f64 = (1.0 + assign31920_e42170);
        let assign31920_e42172: f64 = (3.0 * assign31920_e42171);
        let assign31920_e42173: f64 = (assign31920_e42165 - assign31920_e42172);
        (assign31920_e42173, ((0.5 * locals.var_vgfbpd_dn0) - (3.0 * (locals.var_gammapd_dn0 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn2) - (3.0 * (locals.var_gammapd_dn2 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn3) - (3.0 * (locals.var_gammapd_dn3 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn4) - (3.0 * (locals.var_gammapd_dn4 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn5) - (3.0 * (locals.var_gammapd_dn5 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn6) - (3.0 * (locals.var_gammapd_dn6 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn7) - (3.0 * (locals.var_gammapd_dn7 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn8) - (3.0 * (locals.var_gammapd_dn8 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn9) - (3.0 * (locals.var_gammapd_dn9 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn10) - (3.0 * (locals.var_gammapd_dn10 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn11) - (3.0 * (locals.var_gammapd_dn11 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn12) - (3.0 * (locals.var_gammapd_dn12 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn13) - (3.0 * (locals.var_gammapd_dn13 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn14) - (3.0 * (locals.var_gammapd_dn14 / 1.4142135623730951))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign31920_e42175;
        locals.var_t1_dn0 = assign31920_e42175_d_n0;
        locals.var_t1_dn2 = assign31920_e42175_d_n2;
        locals.var_t1_dn3 = assign31920_e42175_d_n3;
        locals.var_t1_dn4 = assign31920_e42175_d_n4;
        locals.var_t1_dn5 = assign31920_e42175_d_n5;
        locals.var_t1_dn6 = assign31920_e42175_d_n6;
        locals.var_t1_dn7 = assign31920_e42175_d_n7;
        locals.var_t1_dn8 = assign31920_e42175_d_n8;
        locals.var_t1_dn9 = assign31920_e42175_d_n9;
        locals.var_t1_dn10 = assign31920_e42175_d_n10;
        locals.var_t1_dn11 = assign31920_e42175_d_n11;
        locals.var_t1_dn12 = assign31920_e42175_d_n12;
        locals.var_t1_dn13 = assign31920_e42175_d_n13;
        locals.var_t1_dn14 = assign31920_e42175_d_n14;

        let (assign31930_e42188, assign31930_e42188_d_n0, assign31930_e42188_d_n2, assign31930_e42188_d_n3, assign31930_e42188_d_n4, assign31930_e42188_d_n5, assign31930_e42188_d_n6, assign31930_e42188_d_n7, assign31930_e42188_d_n8, assign31930_e42188_d_n9, assign31930_e42188_d_n10, assign31930_e42188_d_n11, assign31930_e42188_d_n12, assign31930_e42188_d_n13, assign31930_e42188_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31930_e42180: f64 = (locals.var_t1 * locals.var_t1);
        let assign31930_e42183: f64 = (6.0 * locals.var_vgfbpd);
        let assign31930_e42184: f64 = (assign31930_e42180 + assign31930_e42183);
        let assign31930_e42185: f64 = (assign31930_e42184).sqrt();
        let assign31930_e42186: f64 = (locals.var_t1 + assign31930_e42185);
        (assign31930_e42186, (locals.var_t1_dn0 + ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + (6.0 * locals.var_vgfbpd_dn0)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn2 + ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + (6.0 * locals.var_vgfbpd_dn2)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn3 + ((((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) + (6.0 * locals.var_vgfbpd_dn3)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn4 + ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (6.0 * locals.var_vgfbpd_dn4)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn5 + ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (6.0 * locals.var_vgfbpd_dn5)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn6 + ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (6.0 * locals.var_vgfbpd_dn6)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn7 + ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (6.0 * locals.var_vgfbpd_dn7)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn8 + ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (6.0 * locals.var_vgfbpd_dn8)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn9 + ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (6.0 * locals.var_vgfbpd_dn9)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn10 + ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (6.0 * locals.var_vgfbpd_dn10)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn11 + ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (6.0 * locals.var_vgfbpd_dn11)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn12 + ((((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) + (6.0 * locals.var_vgfbpd_dn12)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn13 + ((((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) + (6.0 * locals.var_vgfbpd_dn13)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn14 + ((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) + (6.0 * locals.var_vgfbpd_dn14)) / (2.0 * assign31930_e42185))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign31930_e42188;
        locals.var_t2_dn0 = assign31930_e42188_d_n0;
        locals.var_t2_dn2 = assign31930_e42188_d_n2;
        locals.var_t2_dn3 = assign31930_e42188_d_n3;
        locals.var_t2_dn4 = assign31930_e42188_d_n4;
        locals.var_t2_dn5 = assign31930_e42188_d_n5;
        locals.var_t2_dn6 = assign31930_e42188_d_n6;
        locals.var_t2_dn7 = assign31930_e42188_d_n7;
        locals.var_t2_dn8 = assign31930_e42188_d_n8;
        locals.var_t2_dn9 = assign31930_e42188_d_n9;
        locals.var_t2_dn10 = assign31930_e42188_d_n10;
        locals.var_t2_dn11 = assign31930_e42188_d_n11;
        locals.var_t2_dn12 = assign31930_e42188_d_n12;
        locals.var_t2_dn13 = assign31930_e42188_d_n13;
        locals.var_t2_dn14 = assign31930_e42188_d_n14;

        let assign31940_e42191: f64 = if locals.var_vgfbpd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard735 = assign31940_e42191;

        let (assign31950_e42201, assign31950_e42201_d_n0, assign31950_e42201_d_n2, assign31950_e42201_d_n3, assign31950_e42201_d_n4, assign31950_e42201_d_n5, assign31950_e42201_d_n6, assign31950_e42201_d_n7, assign31950_e42201_d_n8, assign31950_e42201_d_n9, assign31950_e42201_d_n10, assign31950_e42201_d_n11, assign31950_e42201_d_n12, assign31950_e42201_d_n13, assign31950_e42201_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 != 0.0)) {
        let assign31950_e42197: f64 = (locals.var_vgfbpd - locals.var_t2);
        let assign31950_e42199: f64 = (assign31950_e42197 / locals.var_gammapd);
        (assign31950_e42199, ((((locals.var_vgfbpd_dn0 - locals.var_t2_dn0) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn0)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn2 - locals.var_t2_dn2) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn2)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn3 - locals.var_t2_dn3) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn3)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn4 - locals.var_t2_dn4) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn4)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn5 - locals.var_t2_dn5) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn5)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn6 - locals.var_t2_dn6) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn6)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn7 - locals.var_t2_dn7) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn7)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn8 - locals.var_t2_dn8) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn8)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn9 - locals.var_t2_dn9) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn9)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn10 - locals.var_t2_dn10) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn10)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn11 - locals.var_t2_dn11) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn11)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn12 - locals.var_t2_dn12) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn12)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn13 - locals.var_t2_dn13) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn13)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn14 - locals.var_t2_dn14) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn14)) / (locals.var_gammapd * locals.var_gammapd)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign31950_e42201;
        locals.var_t3_dn0 = assign31950_e42201_d_n0;
        locals.var_t3_dn2 = assign31950_e42201_d_n2;
        locals.var_t3_dn3 = assign31950_e42201_d_n3;
        locals.var_t3_dn4 = assign31950_e42201_d_n4;
        locals.var_t3_dn5 = assign31950_e42201_d_n5;
        locals.var_t3_dn6 = assign31950_e42201_d_n6;
        locals.var_t3_dn7 = assign31950_e42201_d_n7;
        locals.var_t3_dn8 = assign31950_e42201_d_n8;
        locals.var_t3_dn9 = assign31950_e42201_d_n9;
        locals.var_t3_dn10 = assign31950_e42201_d_n10;
        locals.var_t3_dn11 = assign31950_e42201_d_n11;
        locals.var_t3_dn12 = assign31950_e42201_d_n12;
        locals.var_t3_dn13 = assign31950_e42201_d_n13;
        locals.var_t3_dn14 = assign31950_e42201_d_n14;

        let (assign31960_e42217, assign31960_e42217_d_n0, assign31960_e42217_d_n2, assign31960_e42217_d_n3, assign31960_e42217_d_n4, assign31960_e42217_d_n5, assign31960_e42217_d_n6, assign31960_e42217_d_n7, assign31960_e42217_d_n8, assign31960_e42217_d_n9, assign31960_e42217_d_n10, assign31960_e42217_d_n11, assign31960_e42217_d_n12, assign31960_e42217_d_n13, assign31960_e42217_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 != 0.0)) {
        let assign31960_e42207: f64 = (1.0 - locals.var_t2);
        let assign31960_e42210: f64 = (locals.var_t3 * locals.var_t3);
        let assign31960_e42211: f64 = (assign31960_e42207 + assign31960_e42210);
        let assign31960_e42213: f64 = (assign31960_e42211).max(1e-38);
        let assign31960_e42214: f64 = (assign31960_e42213).ln();
        let assign31960_e42215: f64 = (-assign31960_e42214);
        (assign31960_e42215, (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn0) + ((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn2) + ((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn3) + ((locals.var_t3_dn3 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn3))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn4) + ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn5) + ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn6) + ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn7) + ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn8) + ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn9) + ((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn10) + ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn11) + ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn12) + ((locals.var_t3_dn12 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn12))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn13) + ((locals.var_t3_dn13 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn13))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn14) + ((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14))) } else { 0.0 } / assign31960_e42213)),)
    } else {
        (locals.var_psip, locals.var_psip_dn0, locals.var_psip_dn2, locals.var_psip_dn3, locals.var_psip_dn4, locals.var_psip_dn5, locals.var_psip_dn6, locals.var_psip_dn7, locals.var_psip_dn8, locals.var_psip_dn9, locals.var_psip_dn10, locals.var_psip_dn11, locals.var_psip_dn12, locals.var_psip_dn13, locals.var_psip_dn14,)
    }
};
        locals.var_psip = assign31960_e42217;
        locals.var_psip_dn0 = assign31960_e42217_d_n0;
        locals.var_psip_dn2 = assign31960_e42217_d_n2;
        locals.var_psip_dn3 = assign31960_e42217_d_n3;
        locals.var_psip_dn4 = assign31960_e42217_d_n4;
        locals.var_psip_dn5 = assign31960_e42217_d_n5;
        locals.var_psip_dn6 = assign31960_e42217_d_n6;
        locals.var_psip_dn7 = assign31960_e42217_d_n7;
        locals.var_psip_dn8 = assign31960_e42217_d_n8;
        locals.var_psip_dn9 = assign31960_e42217_d_n9;
        locals.var_psip_dn10 = assign31960_e42217_d_n10;
        locals.var_psip_dn11 = assign31960_e42217_d_n11;
        locals.var_psip_dn12 = assign31960_e42217_d_n12;
        locals.var_psip_dn13 = assign31960_e42217_d_n13;
        locals.var_psip_dn14 = assign31960_e42217_d_n14;

        let (assign31970_e42226, assign31970_e42226_d_n0, assign31970_e42226_d_n2, assign31970_e42226_d_n3, assign31970_e42226_d_n4, assign31970_e42226_d_n5, assign31970_e42226_d_n6, assign31970_e42226_d_n7, assign31970_e42226_d_n8, assign31970_e42226_d_n9, assign31970_e42226_d_n10, assign31970_e42226_d_n11, assign31970_e42226_d_n12, assign31970_e42226_d_n13, assign31970_e42226_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 == 0.0)) {
        let assign31970_e42223: f64 = (-locals.var_t2);
        let assign31970_e42224: f64 = { let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign31970_e42224, ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn0)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn2)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn12)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn13)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign31970_e42226;
        locals.var_t3_dn0 = assign31970_e42226_d_n0;
        locals.var_t3_dn2 = assign31970_e42226_d_n2;
        locals.var_t3_dn3 = assign31970_e42226_d_n3;
        locals.var_t3_dn4 = assign31970_e42226_d_n4;
        locals.var_t3_dn5 = assign31970_e42226_d_n5;
        locals.var_t3_dn6 = assign31970_e42226_d_n6;
        locals.var_t3_dn7 = assign31970_e42226_d_n7;
        locals.var_t3_dn8 = assign31970_e42226_d_n8;
        locals.var_t3_dn9 = assign31970_e42226_d_n9;
        locals.var_t3_dn10 = assign31970_e42226_d_n10;
        locals.var_t3_dn11 = assign31970_e42226_d_n11;
        locals.var_t3_dn12 = assign31970_e42226_d_n12;
        locals.var_t3_dn13 = assign31970_e42226_d_n13;
        locals.var_t3_dn14 = assign31970_e42226_d_n14;

        let (assign31980_e42235, assign31980_e42235_d_n0, assign31980_e42235_d_n2, assign31980_e42235_d_n3, assign31980_e42235_d_n4, assign31980_e42235_d_n5, assign31980_e42235_d_n6, assign31980_e42235_d_n7, assign31980_e42235_d_n8, assign31980_e42235_d_n9, assign31980_e42235_d_n10, assign31980_e42235_d_n11, assign31980_e42235_d_n12, assign31980_e42235_d_n13, assign31980_e42235_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 == 0.0)) {
        let assign31980_e42233: f64 = (0.5 * locals.var_gammapd);
        (assign31980_e42233, (0.5 * locals.var_gammapd_dn0), (0.5 * locals.var_gammapd_dn2), (0.5 * locals.var_gammapd_dn3), (0.5 * locals.var_gammapd_dn4), (0.5 * locals.var_gammapd_dn5), (0.5 * locals.var_gammapd_dn6), (0.5 * locals.var_gammapd_dn7), (0.5 * locals.var_gammapd_dn8), (0.5 * locals.var_gammapd_dn9), (0.5 * locals.var_gammapd_dn10), (0.5 * locals.var_gammapd_dn11), (0.5 * locals.var_gammapd_dn12), (0.5 * locals.var_gammapd_dn13), (0.5 * locals.var_gammapd_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign31980_e42235;
        locals.var_t1_dn0 = assign31980_e42235_d_n0;
        locals.var_t1_dn2 = assign31980_e42235_d_n2;
        locals.var_t1_dn3 = assign31980_e42235_d_n3;
        locals.var_t1_dn4 = assign31980_e42235_d_n4;
        locals.var_t1_dn5 = assign31980_e42235_d_n5;
        locals.var_t1_dn6 = assign31980_e42235_d_n6;
        locals.var_t1_dn7 = assign31980_e42235_d_n7;
        locals.var_t1_dn8 = assign31980_e42235_d_n8;
        locals.var_t1_dn9 = assign31980_e42235_d_n9;
        locals.var_t1_dn10 = assign31980_e42235_d_n10;
        locals.var_t1_dn11 = assign31980_e42235_d_n11;
        locals.var_t1_dn12 = assign31980_e42235_d_n12;
        locals.var_t1_dn13 = assign31980_e42235_d_n13;
        locals.var_t1_dn14 = assign31980_e42235_d_n14;

        let (assign31990_e42253, assign31990_e42253_d_n0, assign31990_e42253_d_n2, assign31990_e42253_d_n3, assign31990_e42253_d_n4, assign31990_e42253_d_n5, assign31990_e42253_d_n6, assign31990_e42253_d_n7, assign31990_e42253_d_n8, assign31990_e42253_d_n9, assign31990_e42253_d_n10, assign31990_e42253_d_n11, assign31990_e42253_d_n12, assign31990_e42253_d_n13, assign31990_e42253_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 == 0.0)) {
        let assign31990_e42242: f64 = (locals.var_vgfbpd - 1.0);
        let assign31990_e42244: f64 = (assign31990_e42242 + locals.var_t3);
        let assign31990_e42247: f64 = (locals.var_t1 * locals.var_t1);
        let assign31990_e42248: f64 = (assign31990_e42244 + assign31990_e42247);
        let assign31990_e42249: f64 = (assign31990_e42248).sqrt();
        let assign31990_e42251: f64 = (assign31990_e42249 - locals.var_t1);
        (assign31990_e42251, ((((locals.var_vgfbpd_dn0 + locals.var_t3_dn0) + ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn0), ((((locals.var_vgfbpd_dn2 + locals.var_t3_dn2) + ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn2), ((((locals.var_vgfbpd_dn3 + locals.var_t3_dn3) + ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn3), ((((locals.var_vgfbpd_dn4 + locals.var_t3_dn4) + ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn4), ((((locals.var_vgfbpd_dn5 + locals.var_t3_dn5) + ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn5), ((((locals.var_vgfbpd_dn6 + locals.var_t3_dn6) + ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn6), ((((locals.var_vgfbpd_dn7 + locals.var_t3_dn7) + ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn7), ((((locals.var_vgfbpd_dn8 + locals.var_t3_dn8) + ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn8), ((((locals.var_vgfbpd_dn9 + locals.var_t3_dn9) + ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn9), ((((locals.var_vgfbpd_dn10 + locals.var_t3_dn10) + ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn10), ((((locals.var_vgfbpd_dn11 + locals.var_t3_dn11) + ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn11), ((((locals.var_vgfbpd_dn12 + locals.var_t3_dn12) + ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn12), ((((locals.var_vgfbpd_dn13 + locals.var_t3_dn13) + ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn13), ((((locals.var_vgfbpd_dn14 + locals.var_t3_dn14) + ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign31990_e42253;
        locals.var_t2_dn0 = assign31990_e42253_d_n0;
        locals.var_t2_dn2 = assign31990_e42253_d_n2;
        locals.var_t2_dn3 = assign31990_e42253_d_n3;
        locals.var_t2_dn4 = assign31990_e42253_d_n4;
        locals.var_t2_dn5 = assign31990_e42253_d_n5;
        locals.var_t2_dn6 = assign31990_e42253_d_n6;
        locals.var_t2_dn7 = assign31990_e42253_d_n7;
        locals.var_t2_dn8 = assign31990_e42253_d_n8;
        locals.var_t2_dn9 = assign31990_e42253_d_n9;
        locals.var_t2_dn10 = assign31990_e42253_d_n10;
        locals.var_t2_dn11 = assign31990_e42253_d_n11;
        locals.var_t2_dn12 = assign31990_e42253_d_n12;
        locals.var_t2_dn13 = assign31990_e42253_d_n13;
        locals.var_t2_dn14 = assign31990_e42253_d_n14;

        let (assign32000_e42266, assign32000_e42266_d_n0, assign32000_e42266_d_n2, assign32000_e42266_d_n3, assign32000_e42266_d_n4, assign32000_e42266_d_n5, assign32000_e42266_d_n6, assign32000_e42266_d_n7, assign32000_e42266_d_n8, assign32000_e42266_d_n9, assign32000_e42266_d_n10, assign32000_e42266_d_n11, assign32000_e42266_d_n12, assign32000_e42266_d_n13, assign32000_e42266_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 == 0.0)) {
        let assign32000_e42260: f64 = (locals.var_t2 * locals.var_t2);
        let assign32000_e42262: f64 = (assign32000_e42260 + 1.0);
        let assign32000_e42264: f64 = (assign32000_e42262 - locals.var_t3);
        (assign32000_e42264, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) - locals.var_t3_dn0), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) - locals.var_t3_dn2), (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) - locals.var_t3_dn3), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) - locals.var_t3_dn4), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) - locals.var_t3_dn5), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) - locals.var_t3_dn6), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) - locals.var_t3_dn7), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) - locals.var_t3_dn8), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) - locals.var_t3_dn9), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) - locals.var_t3_dn10), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) - locals.var_t3_dn11), (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) - locals.var_t3_dn12), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) - locals.var_t3_dn13), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) - locals.var_t3_dn14),)
    } else {
        (locals.var_psip, locals.var_psip_dn0, locals.var_psip_dn2, locals.var_psip_dn3, locals.var_psip_dn4, locals.var_psip_dn5, locals.var_psip_dn6, locals.var_psip_dn7, locals.var_psip_dn8, locals.var_psip_dn9, locals.var_psip_dn10, locals.var_psip_dn11, locals.var_psip_dn12, locals.var_psip_dn13, locals.var_psip_dn14,)
    }
};
        locals.var_psip = assign32000_e42266;
        locals.var_psip_dn0 = assign32000_e42266_d_n0;
        locals.var_psip_dn2 = assign32000_e42266_d_n2;
        locals.var_psip_dn3 = assign32000_e42266_d_n3;
        locals.var_psip_dn4 = assign32000_e42266_d_n4;
        locals.var_psip_dn5 = assign32000_e42266_d_n5;
        locals.var_psip_dn6 = assign32000_e42266_d_n6;
        locals.var_psip_dn7 = assign32000_e42266_d_n7;
        locals.var_psip_dn8 = assign32000_e42266_d_n8;
        locals.var_psip_dn9 = assign32000_e42266_d_n9;
        locals.var_psip_dn10 = assign32000_e42266_d_n10;
        locals.var_psip_dn11 = assign32000_e42266_d_n11;
        locals.var_psip_dn12 = assign32000_e42266_d_n12;
        locals.var_psip_dn13 = assign32000_e42266_d_n13;
        locals.var_psip_dn14 = assign32000_e42266_d_n14;

        let (assign32010_e42289, assign32010_e42289_d_n0, assign32010_e42289_d_n2, assign32010_e42289_d_n3, assign32010_e42289_d_n4, assign32010_e42289_d_n5, assign32010_e42289_d_n6, assign32010_e42289_d_n7, assign32010_e42289_d_n8, assign32010_e42289_d_n9, assign32010_e42289_d_n10, assign32010_e42289_d_n11, assign32010_e42289_d_n12, assign32010_e42289_d_n13, assign32010_e42289_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32010_e42271: f64 = (locals.var_psip + 1.0);
        let assign32010_e42274: f64 = (locals.var_psip - 1.0);
        let assign32010_e42277: f64 = (locals.var_psip - 1.0);
        let assign32010_e42278: f64 = (assign32010_e42274 * assign32010_e42277);
        let assign32010_e42281: f64 = (0.25 * 2.0);
        let assign32010_e42283: f64 = (assign32010_e42281 * 2.0);
        let assign32010_e42284: f64 = (assign32010_e42278 + assign32010_e42283);
        let assign32010_e42285: f64 = (assign32010_e42284).sqrt();
        let assign32010_e42286: f64 = (assign32010_e42271 + assign32010_e42285);
        let assign32010_e42287: f64 = (0.5 * assign32010_e42286);
        (assign32010_e42287, (0.5 * (locals.var_psip_dn0 + (((locals.var_psip_dn0 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn0)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn2 + (((locals.var_psip_dn2 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn2)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn3)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn4)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn5)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn6)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn7)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn8)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn9)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn10)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn11)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn12 + (((locals.var_psip_dn12 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn12)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn13 + (((locals.var_psip_dn13 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn13)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn14 + (((locals.var_psip_dn14 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn14)) / (2.0 * assign32010_e42285)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign32010_e42289;
        locals.var_t8_dn0 = assign32010_e42289_d_n0;
        locals.var_t8_dn2 = assign32010_e42289_d_n2;
        locals.var_t8_dn3 = assign32010_e42289_d_n3;
        locals.var_t8_dn4 = assign32010_e42289_d_n4;
        locals.var_t8_dn5 = assign32010_e42289_d_n5;
        locals.var_t8_dn6 = assign32010_e42289_d_n6;
        locals.var_t8_dn7 = assign32010_e42289_d_n7;
        locals.var_t8_dn8 = assign32010_e42289_d_n8;
        locals.var_t8_dn9 = assign32010_e42289_d_n9;
        locals.var_t8_dn10 = assign32010_e42289_d_n10;
        locals.var_t8_dn11 = assign32010_e42289_d_n11;
        locals.var_t8_dn12 = assign32010_e42289_d_n12;
        locals.var_t8_dn13 = assign32010_e42289_d_n13;
        locals.var_t8_dn14 = assign32010_e42289_d_n14;

        let (assign32020_e42294, assign32020_e42294_d_n0, assign32020_e42294_d_n2, assign32020_e42294_d_n3, assign32020_e42294_d_n4, assign32020_e42294_d_n5, assign32020_e42294_d_n6, assign32020_e42294_d_n7, assign32020_e42294_d_n8, assign32020_e42294_d_n9, assign32020_e42294_d_n10, assign32020_e42294_d_n11, assign32020_e42294_d_n12, assign32020_e42294_d_n13, assign32020_e42294_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32020_e42292: f64 = (locals.var_t8).sqrt();
        (assign32020_e42292, (locals.var_t8_dn0 / (2.0 * assign32020_e42292)), (locals.var_t8_dn2 / (2.0 * assign32020_e42292)), (locals.var_t8_dn3 / (2.0 * assign32020_e42292)), (locals.var_t8_dn4 / (2.0 * assign32020_e42292)), (locals.var_t8_dn5 / (2.0 * assign32020_e42292)), (locals.var_t8_dn6 / (2.0 * assign32020_e42292)), (locals.var_t8_dn7 / (2.0 * assign32020_e42292)), (locals.var_t8_dn8 / (2.0 * assign32020_e42292)), (locals.var_t8_dn9 / (2.0 * assign32020_e42292)), (locals.var_t8_dn10 / (2.0 * assign32020_e42292)), (locals.var_t8_dn11 / (2.0 * assign32020_e42292)), (locals.var_t8_dn12 / (2.0 * assign32020_e42292)), (locals.var_t8_dn13 / (2.0 * assign32020_e42292)), (locals.var_t8_dn14 / (2.0 * assign32020_e42292)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    }
};
        locals.var_sqrtpsip = assign32020_e42294;
        locals.var_sqrtpsip_dn0 = assign32020_e42294_d_n0;
        locals.var_sqrtpsip_dn2 = assign32020_e42294_d_n2;
        locals.var_sqrtpsip_dn3 = assign32020_e42294_d_n3;
        locals.var_sqrtpsip_dn4 = assign32020_e42294_d_n4;
        locals.var_sqrtpsip_dn5 = assign32020_e42294_d_n5;
        locals.var_sqrtpsip_dn6 = assign32020_e42294_d_n6;
        locals.var_sqrtpsip_dn7 = assign32020_e42294_d_n7;
        locals.var_sqrtpsip_dn8 = assign32020_e42294_d_n8;
        locals.var_sqrtpsip_dn9 = assign32020_e42294_d_n9;
        locals.var_sqrtpsip_dn10 = assign32020_e42294_d_n10;
        locals.var_sqrtpsip_dn11 = assign32020_e42294_d_n11;
        locals.var_sqrtpsip_dn12 = assign32020_e42294_d_n12;
        locals.var_sqrtpsip_dn13 = assign32020_e42294_d_n13;
        locals.var_sqrtpsip_dn14 = assign32020_e42294_d_n14;

    }

    pub(super) fn stamp_transient_block_97(
        locals: &mut StampLocals,
    ) {
        let (assign32030_e42306, assign32030_e42306_d_n0, assign32030_e42306_d_n2, assign32030_e42306_d_n3, assign32030_e42306_d_n4, assign32030_e42306_d_n5, assign32030_e42306_d_n6, assign32030_e42306_d_n7, assign32030_e42306_d_n8, assign32030_e42306_d_n9, assign32030_e42306_d_n10, assign32030_e42306_d_n11, assign32030_e42306_d_n12, assign32030_e42306_d_n13, assign32030_e42306_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32030_e42300: f64 = (2.0 * locals.var_sqrtpsip);
        let assign32030_e42301: f64 = (locals.var_gam_edge / assign32030_e42300);
        let assign32030_e42302: f64 = (1.0 + assign32030_e42301);
        let assign32030_e42304: f64 = (assign32030_e42302 / locals.var_gam_edge);
        (assign32030_e42304, ((((((locals.var_gam_edge_dn0 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn0))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn0)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn2 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn2))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn2)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn3 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn3))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn3)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn4 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn4))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn4)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn5 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn5))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn5)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn6 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn6))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn6)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn7 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn7))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn7)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn8 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn8))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn8)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn9 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn9))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn9)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn10 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn10))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn10)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn11 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn11))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn11)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn12 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn12))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn12)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn13 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn13))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn13)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn14 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn14))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn14)) / (locals.var_gam_edge * locals.var_gam_edge)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32030_e42306;
        locals.var_t0_dn0 = assign32030_e42306_d_n0;
        locals.var_t0_dn2 = assign32030_e42306_d_n2;
        locals.var_t0_dn3 = assign32030_e42306_d_n3;
        locals.var_t0_dn4 = assign32030_e42306_d_n4;
        locals.var_t0_dn5 = assign32030_e42306_d_n5;
        locals.var_t0_dn6 = assign32030_e42306_d_n6;
        locals.var_t0_dn7 = assign32030_e42306_d_n7;
        locals.var_t0_dn8 = assign32030_e42306_d_n8;
        locals.var_t0_dn9 = assign32030_e42306_d_n9;
        locals.var_t0_dn10 = assign32030_e42306_d_n10;
        locals.var_t0_dn11 = assign32030_e42306_d_n11;
        locals.var_t0_dn12 = assign32030_e42306_d_n12;
        locals.var_t0_dn13 = assign32030_e42306_d_n13;
        locals.var_t0_dn14 = assign32030_e42306_d_n14;

        let (assign32040_e42316, assign32040_e42316_d_n0, assign32040_e42316_d_n2, assign32040_e42316_d_n3, assign32040_e42316_d_n4, assign32040_e42316_d_n5, assign32040_e42316_d_n6, assign32040_e42316_d_n7, assign32040_e42316_d_n8, assign32040_e42316_d_n9, assign32040_e42316_d_n10, assign32040_e42316_d_n11, assign32040_e42316_d_n12, assign32040_e42316_d_n13, assign32040_e42316_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32040_e42311: f64 = (2.0 * locals.var_phib_n_edge);
        let assign32040_e42312: f64 = (locals.var_psip - assign32040_e42311);
        let assign32040_e42314: f64 = (assign32040_e42312 - locals.var_vs_1);
        (assign32040_e42314, ((locals.var_psip_dn0 - (2.0 * locals.var_phib_n_edge_dn0)) - locals.var_vs_1_dn0), ((locals.var_psip_dn2 - (2.0 * locals.var_phib_n_edge_dn2)) - locals.var_vs_1_dn2), ((locals.var_psip_dn3 - (2.0 * locals.var_phib_n_edge_dn3)) - locals.var_vs_1_dn3), ((locals.var_psip_dn4 - (2.0 * locals.var_phib_n_edge_dn4)) - locals.var_vs_1_dn4), ((locals.var_psip_dn5 - (2.0 * locals.var_phib_n_edge_dn5)) - locals.var_vs_1_dn5), ((locals.var_psip_dn6 - (2.0 * locals.var_phib_n_edge_dn6)) - locals.var_vs_1_dn6), ((locals.var_psip_dn7 - (2.0 * locals.var_phib_n_edge_dn7)) - locals.var_vs_1_dn7), ((locals.var_psip_dn8 - (2.0 * locals.var_phib_n_edge_dn8)) - locals.var_vs_1_dn8), ((locals.var_psip_dn9 - (2.0 * locals.var_phib_n_edge_dn9)) - locals.var_vs_1_dn9), ((locals.var_psip_dn10 - (2.0 * locals.var_phib_n_edge_dn10)) - locals.var_vs_1_dn10), ((locals.var_psip_dn11 - (2.0 * locals.var_phib_n_edge_dn11)) - locals.var_vs_1_dn11), ((locals.var_psip_dn12 - (2.0 * locals.var_phib_n_edge_dn12)) - locals.var_vs_1_dn12), ((locals.var_psip_dn13 - (2.0 * locals.var_phib_n_edge_dn13)) - locals.var_vs_1_dn13), ((locals.var_psip_dn14 - (2.0 * locals.var_phib_n_edge_dn14)) - locals.var_vs_1_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32040_e42316;
        locals.var_t1_dn0 = assign32040_e42316_d_n0;
        locals.var_t1_dn2 = assign32040_e42316_d_n2;
        locals.var_t1_dn3 = assign32040_e42316_d_n3;
        locals.var_t1_dn4 = assign32040_e42316_d_n4;
        locals.var_t1_dn5 = assign32040_e42316_d_n5;
        locals.var_t1_dn6 = assign32040_e42316_d_n6;
        locals.var_t1_dn7 = assign32040_e42316_d_n7;
        locals.var_t1_dn8 = assign32040_e42316_d_n8;
        locals.var_t1_dn9 = assign32040_e42316_d_n9;
        locals.var_t1_dn10 = assign32040_e42316_d_n10;
        locals.var_t1_dn11 = assign32040_e42316_d_n11;
        locals.var_t1_dn12 = assign32040_e42316_d_n12;
        locals.var_t1_dn13 = assign32040_e42316_d_n13;
        locals.var_t1_dn14 = assign32040_e42316_d_n14;

        let (assign32050_e42331, assign32050_e42331_d_n0, assign32050_e42331_d_n2, assign32050_e42331_d_n3, assign32050_e42331_d_n4, assign32050_e42331_d_n5, assign32050_e42331_d_n6, assign32050_e42331_d_n7, assign32050_e42331_d_n8, assign32050_e42331_d_n9, assign32050_e42331_d_n10, assign32050_e42331_d_n11, assign32050_e42331_d_n12, assign32050_e42331_d_n13, assign32050_e42331_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32050_e42320: f64 = locals.var_t1;
        let assign32050_e42323: f64 = (4.0 * locals.var_t0);
        let assign32050_e42325: f64 = (assign32050_e42323 * locals.var_sqrtpsip);
        let assign32050_e42327: f64 = (assign32050_e42325).max(1e-38);
        let assign32050_e42328: f64 = (assign32050_e42327).ln();
        let assign32050_e42329: f64 = (assign32050_e42320 - assign32050_e42328);
        (assign32050_e42329, (locals.var_t1_dn0 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn0) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn0)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn2 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn2) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn2)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn3 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn3) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn3)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn4 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn4) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn4)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn5 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn5) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn5)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn6 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn6) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn6)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn7 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn7) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn7)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn8 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn8) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn8)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn9 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn9) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn9)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn10 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn10) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn10)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn11 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn11) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn11)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn12 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn12) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn12)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn13 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn13) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn13)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn14 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn14) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn14)) } else { 0.0 } / assign32050_e42327)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32050_e42331;
        locals.var_t2_dn0 = assign32050_e42331_d_n0;
        locals.var_t2_dn2 = assign32050_e42331_d_n2;
        locals.var_t2_dn3 = assign32050_e42331_d_n3;
        locals.var_t2_dn4 = assign32050_e42331_d_n4;
        locals.var_t2_dn5 = assign32050_e42331_d_n5;
        locals.var_t2_dn6 = assign32050_e42331_d_n6;
        locals.var_t2_dn7 = assign32050_e42331_d_n7;
        locals.var_t2_dn8 = assign32050_e42331_d_n8;
        locals.var_t2_dn9 = assign32050_e42331_d_n9;
        locals.var_t2_dn10 = assign32050_e42331_d_n10;
        locals.var_t2_dn11 = assign32050_e42331_d_n11;
        locals.var_t2_dn12 = assign32050_e42331_d_n12;
        locals.var_t2_dn13 = assign32050_e42331_d_n13;
        locals.var_t2_dn14 = assign32050_e42331_d_n14;

        let (assign32060_e42348, assign32060_e42348_d_n0, assign32060_e42348_d_n2, assign32060_e42348_d_n3, assign32060_e42348_d_n4, assign32060_e42348_d_n5, assign32060_e42348_d_n6, assign32060_e42348_d_n7, assign32060_e42348_d_n8, assign32060_e42348_d_n9, assign32060_e42348_d_n10, assign32060_e42348_d_n11, assign32060_e42348_d_n12, assign32060_e42348_d_n13, assign32060_e42348_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32060_e42336: f64 = (locals.var_t2 - 0.201491);
        let assign32060_e42340: f64 = (locals.var_t2 + 0.402982);
        let assign32060_e42341: f64 = (locals.var_t2 * assign32060_e42340);
        let assign32060_e42343: f64 = (assign32060_e42341 + 2.446562);
        let assign32060_e42344: f64 = (assign32060_e42343).sqrt();
        let assign32060_e42345: f64 = (assign32060_e42336 - assign32060_e42344);
        let assign32060_e42346: f64 = (0.5 * assign32060_e42345);
        (assign32060_e42346, (0.5 * (locals.var_t2_dn0 - (((locals.var_t2_dn0 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn2 - (((locals.var_t2_dn2 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn12 - (((locals.var_t2_dn12 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn13 - (((locals.var_t2_dn13 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn14 - (((locals.var_t2_dn14 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign32060_e42344)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign32060_e42348;
        locals.var_t8_dn0 = assign32060_e42348_d_n0;
        locals.var_t8_dn2 = assign32060_e42348_d_n2;
        locals.var_t8_dn3 = assign32060_e42348_d_n3;
        locals.var_t8_dn4 = assign32060_e42348_d_n4;
        locals.var_t8_dn5 = assign32060_e42348_d_n5;
        locals.var_t8_dn6 = assign32060_e42348_d_n6;
        locals.var_t8_dn7 = assign32060_e42348_d_n7;
        locals.var_t8_dn8 = assign32060_e42348_d_n8;
        locals.var_t8_dn9 = assign32060_e42348_d_n9;
        locals.var_t8_dn10 = assign32060_e42348_d_n10;
        locals.var_t8_dn11 = assign32060_e42348_d_n11;
        locals.var_t8_dn12 = assign32060_e42348_d_n12;
        locals.var_t8_dn13 = assign32060_e42348_d_n13;
        locals.var_t8_dn14 = assign32060_e42348_d_n14;

        let (assign32070_e42352, assign32070_e42352_d_n0, assign32070_e42352_d_n2, assign32070_e42352_d_n3, assign32070_e42352_d_n4, assign32070_e42352_d_n5, assign32070_e42352_d_n6, assign32070_e42352_d_n7, assign32070_e42352_d_n8, assign32070_e42352_d_n9, assign32070_e42352_d_n10, assign32070_e42352_d_n11, assign32070_e42352_d_n12, assign32070_e42352_d_n13, assign32070_e42352_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn0, locals.var_sqrtpsisa_dn2, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11, locals.var_sqrtpsisa_dn12, locals.var_sqrtpsisa_dn13, locals.var_sqrtpsisa_dn14,)
    }
};
        locals.var_sqrtpsisa = assign32070_e42352;
        locals.var_sqrtpsisa_dn0 = assign32070_e42352_d_n0;
        locals.var_sqrtpsisa_dn2 = assign32070_e42352_d_n2;
        locals.var_sqrtpsisa_dn3 = assign32070_e42352_d_n3;
        locals.var_sqrtpsisa_dn4 = assign32070_e42352_d_n4;
        locals.var_sqrtpsisa_dn5 = assign32070_e42352_d_n5;
        locals.var_sqrtpsisa_dn6 = assign32070_e42352_d_n6;
        locals.var_sqrtpsisa_dn7 = assign32070_e42352_d_n7;
        locals.var_sqrtpsisa_dn8 = assign32070_e42352_d_n8;
        locals.var_sqrtpsisa_dn9 = assign32070_e42352_d_n9;
        locals.var_sqrtpsisa_dn10 = assign32070_e42352_d_n10;
        locals.var_sqrtpsisa_dn11 = assign32070_e42352_d_n11;
        locals.var_sqrtpsisa_dn12 = assign32070_e42352_d_n12;
        locals.var_sqrtpsisa_dn13 = assign32070_e42352_d_n13;
        locals.var_sqrtpsisa_dn14 = assign32070_e42352_d_n14;

        let assign32080_e42355: f64 = (-68.0);
        let assign32080_e42356: f64 = if locals.var_t8 <= assign32080_e42355 { 1.0 } else { 0.0 };
        locals.var_guard736 = assign32080_e42356;

        let (assign32090_e42363, assign32090_e42363_d_n0, assign32090_e42363_d_n2, assign32090_e42363_d_n3, assign32090_e42363_d_n4, assign32090_e42363_d_n5, assign32090_e42363_d_n6, assign32090_e42363_d_n7, assign32090_e42363_d_n8, assign32090_e42363_d_n9, assign32090_e42363_d_n10, assign32090_e42363_d_n11, assign32090_e42363_d_n12, assign32090_e42363_d_n13, assign32090_e42363_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) {
        let assign32090_e42361: f64 = (-100.0);
        (assign32090_e42361, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32090_e42363;
        locals.var_t4_dn0 = assign32090_e42363_d_n0;
        locals.var_t4_dn2 = assign32090_e42363_d_n2;
        locals.var_t4_dn3 = assign32090_e42363_d_n3;
        locals.var_t4_dn4 = assign32090_e42363_d_n4;
        locals.var_t4_dn5 = assign32090_e42363_d_n5;
        locals.var_t4_dn6 = assign32090_e42363_d_n6;
        locals.var_t4_dn7 = assign32090_e42363_d_n7;
        locals.var_t4_dn8 = assign32090_e42363_d_n8;
        locals.var_t4_dn9 = assign32090_e42363_d_n9;
        locals.var_t4_dn10 = assign32090_e42363_d_n10;
        locals.var_t4_dn11 = assign32090_e42363_d_n11;
        locals.var_t4_dn12 = assign32090_e42363_d_n12;
        locals.var_t4_dn13 = assign32090_e42363_d_n13;
        locals.var_t4_dn14 = assign32090_e42363_d_n14;

        let (assign32100_e42369, assign32100_e42369_d_n0, assign32100_e42369_d_n2, assign32100_e42369_d_n3, assign32100_e42369_d_n4, assign32100_e42369_d_n5, assign32100_e42369_d_n6, assign32100_e42369_d_n7, assign32100_e42369_d_n8, assign32100_e42369_d_n9, assign32100_e42369_d_n10, assign32100_e42369_d_n11, assign32100_e42369_d_n12, assign32100_e42369_d_n13, assign32100_e42369_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32100_e42369;
        locals.var_t5_dn0 = assign32100_e42369_d_n0;
        locals.var_t5_dn2 = assign32100_e42369_d_n2;
        locals.var_t5_dn3 = assign32100_e42369_d_n3;
        locals.var_t5_dn4 = assign32100_e42369_d_n4;
        locals.var_t5_dn5 = assign32100_e42369_d_n5;
        locals.var_t5_dn6 = assign32100_e42369_d_n6;
        locals.var_t5_dn7 = assign32100_e42369_d_n7;
        locals.var_t5_dn8 = assign32100_e42369_d_n8;
        locals.var_t5_dn9 = assign32100_e42369_d_n9;
        locals.var_t5_dn10 = assign32100_e42369_d_n10;
        locals.var_t5_dn11 = assign32100_e42369_d_n11;
        locals.var_t5_dn12 = assign32100_e42369_d_n12;
        locals.var_t5_dn13 = assign32100_e42369_d_n13;
        locals.var_t5_dn14 = assign32100_e42369_d_n14;

        let assign32110_e42374: f64 = (0.5 * locals.var_t5);
        let assign32110_e42375: f64 = (locals.var_t4 - assign32110_e42374);
        let assign32110_e42376: f64 = if locals.var_t8 < assign32110_e42375 { 1.0 } else { 0.0 };
        locals.var_guard737 = assign32110_e42376;

        let (assign32120_e42385, assign32120_e42385_d_n0, assign32120_e42385_d_n2, assign32120_e42385_d_n3, assign32120_e42385_d_n4, assign32120_e42385_d_n5, assign32120_e42385_d_n6, assign32120_e42385_d_n7, assign32120_e42385_d_n8, assign32120_e42385_d_n9, assign32120_e42385_d_n10, assign32120_e42385_d_n11, assign32120_e42385_d_n12, assign32120_e42385_d_n13, assign32120_e42385_d_n14,) = {
    if (((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) && (locals.var_guard737 != 0.0)) {
        let assign32120_e42383: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32120_e42383, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn0), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn2), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn12), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn13), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32120_e42385;
        locals.var_t3_dn0 = assign32120_e42385_d_n0;
        locals.var_t3_dn2 = assign32120_e42385_d_n2;
        locals.var_t3_dn3 = assign32120_e42385_d_n3;
        locals.var_t3_dn4 = assign32120_e42385_d_n4;
        locals.var_t3_dn5 = assign32120_e42385_d_n5;
        locals.var_t3_dn6 = assign32120_e42385_d_n6;
        locals.var_t3_dn7 = assign32120_e42385_d_n7;
        locals.var_t3_dn8 = assign32120_e42385_d_n8;
        locals.var_t3_dn9 = assign32120_e42385_d_n9;
        locals.var_t3_dn10 = assign32120_e42385_d_n10;
        locals.var_t3_dn11 = assign32120_e42385_d_n11;
        locals.var_t3_dn12 = assign32120_e42385_d_n12;
        locals.var_t3_dn13 = assign32120_e42385_d_n13;
        locals.var_t3_dn14 = assign32120_e42385_d_n14;

        let assign32130_e42390: f64 = (0.5 * locals.var_t5);
        let assign32130_e42391: f64 = (locals.var_t4 + assign32130_e42390);
        let assign32130_e42392: f64 = if locals.var_t8 > assign32130_e42391 { 1.0 } else { 0.0 };
        locals.var_guard738 = assign32130_e42392;

        let (assign32140_e42404, assign32140_e42404_d_n0, assign32140_e42404_d_n2, assign32140_e42404_d_n3, assign32140_e42404_d_n4, assign32140_e42404_d_n5, assign32140_e42404_d_n6, assign32140_e42404_d_n7, assign32140_e42404_d_n8, assign32140_e42404_d_n9, assign32140_e42404_d_n10, assign32140_e42404_d_n11, assign32140_e42404_d_n12, assign32140_e42404_d_n13, assign32140_e42404_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign32140_e42402: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32140_e42402, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn12), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32140_e42404;
        locals.var_t3_dn0 = assign32140_e42404_d_n0;
        locals.var_t3_dn2 = assign32140_e42404_d_n2;
        locals.var_t3_dn3 = assign32140_e42404_d_n3;
        locals.var_t3_dn4 = assign32140_e42404_d_n4;
        locals.var_t3_dn5 = assign32140_e42404_d_n5;
        locals.var_t3_dn6 = assign32140_e42404_d_n6;
        locals.var_t3_dn7 = assign32140_e42404_d_n7;
        locals.var_t3_dn8 = assign32140_e42404_d_n8;
        locals.var_t3_dn9 = assign32140_e42404_d_n9;
        locals.var_t3_dn10 = assign32140_e42404_d_n10;
        locals.var_t3_dn11 = assign32140_e42404_d_n11;
        locals.var_t3_dn12 = assign32140_e42404_d_n12;
        locals.var_t3_dn13 = assign32140_e42404_d_n13;
        locals.var_t3_dn14 = assign32140_e42404_d_n14;

        let (assign32150_e42420, assign32150_e42420_d_n0, assign32150_e42420_d_n2, assign32150_e42420_d_n3, assign32150_e42420_d_n4, assign32150_e42420_d_n5, assign32150_e42420_d_n6, assign32150_e42420_d_n7, assign32150_e42420_d_n8, assign32150_e42420_d_n9, assign32150_e42420_d_n10, assign32150_e42420_d_n11, assign32150_e42420_d_n12, assign32150_e42420_d_n13, assign32150_e42420_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 == 0.0)) {
        let assign32150_e42416: f64 = (locals.var_t8 - locals.var_t4);
        let assign32150_e42418: f64 = (assign32150_e42416 / locals.var_t5);
        (assign32150_e42418, ((((locals.var_t8_dn0 - locals.var_t4_dn0) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn2 - locals.var_t4_dn2) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn12 - locals.var_t4_dn12) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn13 - locals.var_t4_dn13) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn14 - locals.var_t4_dn14) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32150_e42420;
        locals.var_t2_dn0 = assign32150_e42420_d_n0;
        locals.var_t2_dn2 = assign32150_e42420_d_n2;
        locals.var_t2_dn3 = assign32150_e42420_d_n3;
        locals.var_t2_dn4 = assign32150_e42420_d_n4;
        locals.var_t2_dn5 = assign32150_e42420_d_n5;
        locals.var_t2_dn6 = assign32150_e42420_d_n6;
        locals.var_t2_dn7 = assign32150_e42420_d_n7;
        locals.var_t2_dn8 = assign32150_e42420_d_n8;
        locals.var_t2_dn9 = assign32150_e42420_d_n9;
        locals.var_t2_dn10 = assign32150_e42420_d_n10;
        locals.var_t2_dn11 = assign32150_e42420_d_n11;
        locals.var_t2_dn12 = assign32150_e42420_d_n12;
        locals.var_t2_dn13 = assign32150_e42420_d_n13;
        locals.var_t2_dn14 = assign32150_e42420_d_n14;

        let (assign32160_e42434, assign32160_e42434_d_n0, assign32160_e42434_d_n2, assign32160_e42434_d_n3, assign32160_e42434_d_n4, assign32160_e42434_d_n5, assign32160_e42434_d_n6, assign32160_e42434_d_n7, assign32160_e42434_d_n8, assign32160_e42434_d_n9, assign32160_e42434_d_n10, assign32160_e42434_d_n11, assign32160_e42434_d_n12, assign32160_e42434_d_n13, assign32160_e42434_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 == 0.0)) {
        let assign32160_e42432: f64 = (locals.var_t2 * locals.var_t2);
        (assign32160_e42432, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)), ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)), ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32160_e42434;
        locals.var_t6_dn0 = assign32160_e42434_d_n0;
        locals.var_t6_dn2 = assign32160_e42434_d_n2;
        locals.var_t6_dn3 = assign32160_e42434_d_n3;
        locals.var_t6_dn4 = assign32160_e42434_d_n4;
        locals.var_t6_dn5 = assign32160_e42434_d_n5;
        locals.var_t6_dn6 = assign32160_e42434_d_n6;
        locals.var_t6_dn7 = assign32160_e42434_d_n7;
        locals.var_t6_dn8 = assign32160_e42434_d_n8;
        locals.var_t6_dn9 = assign32160_e42434_d_n9;
        locals.var_t6_dn10 = assign32160_e42434_d_n10;
        locals.var_t6_dn11 = assign32160_e42434_d_n11;
        locals.var_t6_dn12 = assign32160_e42434_d_n12;
        locals.var_t6_dn13 = assign32160_e42434_d_n13;
        locals.var_t6_dn14 = assign32160_e42434_d_n14;

        let (assign32170_e42469, assign32170_e42469_d_n0, assign32170_e42469_d_n2, assign32170_e42469_d_n3, assign32170_e42469_d_n4, assign32170_e42469_d_n5, assign32170_e42469_d_n6, assign32170_e42469_d_n7, assign32170_e42469_d_n8, assign32170_e42469_d_n9, assign32170_e42469_d_n10, assign32170_e42469_d_n11, assign32170_e42469_d_n12, assign32170_e42469_d_n13, assign32170_e42469_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 == 0.0)) {
        let assign32170_e42448: f64 = (5.0 / 64.0);
        let assign32170_e42451: f64 = (0.5 * locals.var_t2);
        let assign32170_e42452: f64 = (assign32170_e42448 + assign32170_e42451);
        let assign32170_e42456: f64 = (15.0 / 16.0);
        let assign32170_e42460: f64 = (1.25 - locals.var_t6);
        let assign32170_e42461: f64 = (locals.var_t6 * assign32170_e42460);
        let assign32170_e42462: f64 = (assign32170_e42456 - assign32170_e42461);
        let assign32170_e42463: f64 = (locals.var_t6 * assign32170_e42462);
        let assign32170_e42464: f64 = (assign32170_e42452 + assign32170_e42463);
        let assign32170_e42465: f64 = (locals.var_t5 * assign32170_e42464);
        let assign32170_e42466: f64 = (locals.var_t4 + assign32170_e42465);
        let assign32170_e42467: f64 = { let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32170_e42467, ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn0 + ((locals.var_t5_dn0 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn0) + ((locals.var_t6_dn0 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn0 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn0))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn2 + ((locals.var_t5_dn2 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn2) + ((locals.var_t6_dn2 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn2 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn2))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn12 + ((locals.var_t5_dn12 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn12) + ((locals.var_t6_dn12 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn12 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn12))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn13 + ((locals.var_t5_dn13 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn13) + ((locals.var_t6_dn13 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn13 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn13))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn14 + ((locals.var_t5_dn14 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn14) + ((locals.var_t6_dn14 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn14 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn14))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32170_e42469;
        locals.var_t3_dn0 = assign32170_e42469_d_n0;
        locals.var_t3_dn2 = assign32170_e42469_d_n2;
        locals.var_t3_dn3 = assign32170_e42469_d_n3;
        locals.var_t3_dn4 = assign32170_e42469_d_n4;
        locals.var_t3_dn5 = assign32170_e42469_d_n5;
        locals.var_t3_dn6 = assign32170_e42469_d_n6;
        locals.var_t3_dn7 = assign32170_e42469_d_n7;
        locals.var_t3_dn8 = assign32170_e42469_d_n8;
        locals.var_t3_dn9 = assign32170_e42469_d_n9;
        locals.var_t3_dn10 = assign32170_e42469_d_n10;
        locals.var_t3_dn11 = assign32170_e42469_d_n11;
        locals.var_t3_dn12 = assign32170_e42469_d_n12;
        locals.var_t3_dn13 = assign32170_e42469_d_n13;
        locals.var_t3_dn14 = assign32170_e42469_d_n14;

        let (assign32180_e42502, assign32180_e42502_d_n0, assign32180_e42502_d_n2, assign32180_e42502_d_n3, assign32180_e42502_d_n4, assign32180_e42502_d_n5, assign32180_e42502_d_n6, assign32180_e42502_d_n7, assign32180_e42502_d_n8, assign32180_e42502_d_n9, assign32180_e42502_d_n10, assign32180_e42502_d_n11, assign32180_e42502_d_n12, assign32180_e42502_d_n13, assign32180_e42502_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) {
        let assign32180_e42476: f64 = (1.0 + locals.var_t1);
        let assign32180_e42479: f64 = locals.var_t8;
        let assign32180_e42480: f64 = (assign32180_e42476 - assign32180_e42479);
        let assign32180_e42484: f64 = (2.0 * locals.var_t0);
        let assign32180_e42487: f64 = (locals.var_t3 * 2.0);
        let assign32180_e42489: f64 = (assign32180_e42487 * locals.var_t0);
        let assign32180_e42492: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32180_e42493: f64 = (assign32180_e42489 + assign32180_e42492);
        let assign32180_e42494: f64 = (assign32180_e42484 * assign32180_e42493);
        let assign32180_e42496: f64 = (assign32180_e42494).max(1e-38);
        let assign32180_e42497: f64 = (assign32180_e42496).ln();
        let assign32180_e42498: f64 = assign32180_e42497;
        let assign32180_e42499: f64 = (assign32180_e42480 - assign32180_e42498);
        let assign32180_e42500: f64 = (locals.var_t3 * assign32180_e42499);
        (assign32180_e42500, ((locals.var_t3_dn0 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn0 - locals.var_t8_dn0) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn0) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn2 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn2 - locals.var_t8_dn2) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn2) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn3 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn4 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn5 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn6 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn7 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn8 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn9 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn10 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn11 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn12 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn12 - locals.var_t8_dn12) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn12) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn13 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn13 - locals.var_t8_dn13) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn13) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn14 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn14 - locals.var_t8_dn14) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn14) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32180_e42496)))),)
    } else {
        (locals.var_qs_edge, locals.var_qs_edge_dn0, locals.var_qs_edge_dn2, locals.var_qs_edge_dn3, locals.var_qs_edge_dn4, locals.var_qs_edge_dn5, locals.var_qs_edge_dn6, locals.var_qs_edge_dn7, locals.var_qs_edge_dn8, locals.var_qs_edge_dn9, locals.var_qs_edge_dn10, locals.var_qs_edge_dn11, locals.var_qs_edge_dn12, locals.var_qs_edge_dn13, locals.var_qs_edge_dn14,)
    }
};
        locals.var_qs_edge = assign32180_e42502;
        locals.var_qs_edge_dn0 = assign32180_e42502_d_n0;
        locals.var_qs_edge_dn2 = assign32180_e42502_d_n2;
        locals.var_qs_edge_dn3 = assign32180_e42502_d_n3;
        locals.var_qs_edge_dn4 = assign32180_e42502_d_n4;
        locals.var_qs_edge_dn5 = assign32180_e42502_d_n5;
        locals.var_qs_edge_dn6 = assign32180_e42502_d_n6;
        locals.var_qs_edge_dn7 = assign32180_e42502_d_n7;
        locals.var_qs_edge_dn8 = assign32180_e42502_d_n8;
        locals.var_qs_edge_dn9 = assign32180_e42502_d_n9;
        locals.var_qs_edge_dn10 = assign32180_e42502_d_n10;
        locals.var_qs_edge_dn11 = assign32180_e42502_d_n11;
        locals.var_qs_edge_dn12 = assign32180_e42502_d_n12;
        locals.var_qs_edge_dn13 = assign32180_e42502_d_n13;
        locals.var_qs_edge_dn14 = assign32180_e42502_d_n14;

        let (assign32190_e42510, assign32190_e42510_d_n0, assign32190_e42510_d_n2, assign32190_e42510_d_n3, assign32190_e42510_d_n4, assign32190_e42510_d_n5, assign32190_e42510_d_n6, assign32190_e42510_d_n7, assign32190_e42510_d_n8, assign32190_e42510_d_n9, assign32190_e42510_d_n10, assign32190_e42510_d_n11, assign32190_e42510_d_n12, assign32190_e42510_d_n13, assign32190_e42510_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32190_e42508: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32190_e42508, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn12), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32190_e42510;
        locals.var_t3_dn0 = assign32190_e42510_d_n0;
        locals.var_t3_dn2 = assign32190_e42510_d_n2;
        locals.var_t3_dn3 = assign32190_e42510_d_n3;
        locals.var_t3_dn4 = assign32190_e42510_d_n4;
        locals.var_t3_dn5 = assign32190_e42510_d_n5;
        locals.var_t3_dn6 = assign32190_e42510_d_n6;
        locals.var_t3_dn7 = assign32190_e42510_d_n7;
        locals.var_t3_dn8 = assign32190_e42510_d_n8;
        locals.var_t3_dn9 = assign32190_e42510_d_n9;
        locals.var_t3_dn10 = assign32190_e42510_d_n10;
        locals.var_t3_dn11 = assign32190_e42510_d_n11;
        locals.var_t3_dn12 = assign32190_e42510_d_n12;
        locals.var_t3_dn13 = assign32190_e42510_d_n13;
        locals.var_t3_dn14 = assign32190_e42510_d_n14;

        let (assign32200_e42519, assign32200_e42519_d_n0, assign32200_e42519_d_n2, assign32200_e42519_d_n3, assign32200_e42519_d_n4, assign32200_e42519_d_n5, assign32200_e42519_d_n6, assign32200_e42519_d_n7, assign32200_e42519_d_n8, assign32200_e42519_d_n9, assign32200_e42519_d_n10, assign32200_e42519_d_n11, assign32200_e42519_d_n12, assign32200_e42519_d_n13, assign32200_e42519_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32200_e42517: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign32200_e42517, (-(locals.var_sqrtpsisa_dn0 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn2 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn12 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn13 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn14 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn0, locals.var_sqrtpsisainv_dn2, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11, locals.var_sqrtpsisainv_dn12, locals.var_sqrtpsisainv_dn13, locals.var_sqrtpsisainv_dn14,)
    }
};
        locals.var_sqrtpsisainv = assign32200_e42519;
        locals.var_sqrtpsisainv_dn0 = assign32200_e42519_d_n0;
        locals.var_sqrtpsisainv_dn2 = assign32200_e42519_d_n2;
        locals.var_sqrtpsisainv_dn3 = assign32200_e42519_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign32200_e42519_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign32200_e42519_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign32200_e42519_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign32200_e42519_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign32200_e42519_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign32200_e42519_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign32200_e42519_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign32200_e42519_d_n11;
        locals.var_sqrtpsisainv_dn12 = assign32200_e42519_d_n12;
        locals.var_sqrtpsisainv_dn13 = assign32200_e42519_d_n13;
        locals.var_sqrtpsisainv_dn14 = assign32200_e42519_d_n14;

        let (assign32210_e42551, assign32210_e42551_d_n0, assign32210_e42551_d_n2, assign32210_e42551_d_n3, assign32210_e42551_d_n4, assign32210_e42551_d_n5, assign32210_e42551_d_n6, assign32210_e42551_d_n7, assign32210_e42551_d_n8, assign32210_e42551_d_n9, assign32210_e42551_d_n10, assign32210_e42551_d_n11, assign32210_e42551_d_n12, assign32210_e42551_d_n13, assign32210_e42551_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32210_e42526: f64 = (2.0 * locals.var_t3);
        let assign32210_e42530: f64 = (locals.var_t3 * 2.0);
        let assign32210_e42532: f64 = (assign32210_e42530 * locals.var_t0);
        let assign32210_e42535: f64 = (locals.var_t3 * 2.0);
        let assign32210_e42537: f64 = (assign32210_e42535 * locals.var_t0);
        let assign32210_e42540: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32210_e42541: f64 = (assign32210_e42537 + assign32210_e42540);
        let assign32210_e42542: f64 = (assign32210_e42532 * assign32210_e42541);
        let assign32210_e42544: f64 = (assign32210_e42542).max(1e-38);
        let assign32210_e42545: f64 = (assign32210_e42544).ln();
        let assign32210_e42546: f64 = assign32210_e42545;
        let assign32210_e42547: f64 = (assign32210_e42526 + assign32210_e42546);
        let assign32210_e42549: f64 = (assign32210_e42547 - locals.var_t1);
        (assign32210_e42549, (((2.0 * locals.var_t3_dn0) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn0)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn0), (((2.0 * locals.var_t3_dn2) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn2)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn2), (((2.0 * locals.var_t3_dn3) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn3)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn4)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn5)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn6)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn7)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn8)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn9)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn10)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn11)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn11), (((2.0 * locals.var_t3_dn12) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn12)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn12), (((2.0 * locals.var_t3_dn13) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn13)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn13), (((2.0 * locals.var_t3_dn14) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn14)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32210_e42551;
        locals.var_t4_dn0 = assign32210_e42551_d_n0;
        locals.var_t4_dn2 = assign32210_e42551_d_n2;
        locals.var_t4_dn3 = assign32210_e42551_d_n3;
        locals.var_t4_dn4 = assign32210_e42551_d_n4;
        locals.var_t4_dn5 = assign32210_e42551_d_n5;
        locals.var_t4_dn6 = assign32210_e42551_d_n6;
        locals.var_t4_dn7 = assign32210_e42551_d_n7;
        locals.var_t4_dn8 = assign32210_e42551_d_n8;
        locals.var_t4_dn9 = assign32210_e42551_d_n9;
        locals.var_t4_dn10 = assign32210_e42551_d_n10;
        locals.var_t4_dn11 = assign32210_e42551_d_n11;
        locals.var_t4_dn12 = assign32210_e42551_d_n12;
        locals.var_t4_dn13 = assign32210_e42551_d_n13;
        locals.var_t4_dn14 = assign32210_e42551_d_n14;

        let (assign32220_e42576, assign32220_e42576_d_n0, assign32220_e42576_d_n2, assign32220_e42576_d_n3, assign32220_e42576_d_n4, assign32220_e42576_d_n5, assign32220_e42576_d_n6, assign32220_e42576_d_n7, assign32220_e42576_d_n8, assign32220_e42576_d_n9, assign32220_e42576_d_n10, assign32220_e42576_d_n11, assign32220_e42576_d_n12, assign32220_e42576_d_n13, assign32220_e42576_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32220_e42559: f64 = 1.0;
        let assign32220_e42561: f64 = (assign32220_e42559 / locals.var_t3);
        let assign32220_e42562: f64 = (2.0 + assign32220_e42561);
        let assign32220_e42566: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32220_e42567: f64 = assign32220_e42566;
        let assign32220_e42570: f64 = (locals.var_t0 * locals.var_t3);
        let assign32220_e42572: f64 = (assign32220_e42570 + locals.var_sqrtpsisa);
        let assign32220_e42573: f64 = (assign32220_e42567 / assign32220_e42572);
        let assign32220_e42574: f64 = (assign32220_e42562 + assign32220_e42573);
        (assign32220_e42574, ((-((assign32220_e42559 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32220_e42572 * assign32220_e42572))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32220_e42576;
        locals.var_t5_dn0 = assign32220_e42576_d_n0;
        locals.var_t5_dn2 = assign32220_e42576_d_n2;
        locals.var_t5_dn3 = assign32220_e42576_d_n3;
        locals.var_t5_dn4 = assign32220_e42576_d_n4;
        locals.var_t5_dn5 = assign32220_e42576_d_n5;
        locals.var_t5_dn6 = assign32220_e42576_d_n6;
        locals.var_t5_dn7 = assign32220_e42576_d_n7;
        locals.var_t5_dn8 = assign32220_e42576_d_n8;
        locals.var_t5_dn9 = assign32220_e42576_d_n9;
        locals.var_t5_dn10 = assign32220_e42576_d_n10;
        locals.var_t5_dn11 = assign32220_e42576_d_n11;
        locals.var_t5_dn12 = assign32220_e42576_d_n12;
        locals.var_t5_dn13 = assign32220_e42576_d_n13;
        locals.var_t5_dn14 = assign32220_e42576_d_n14;

        let (assign32230_e42587, assign32230_e42587_d_n0, assign32230_e42587_d_n2, assign32230_e42587_d_n3, assign32230_e42587_d_n4, assign32230_e42587_d_n5, assign32230_e42587_d_n6, assign32230_e42587_d_n7, assign32230_e42587_d_n8, assign32230_e42587_d_n9, assign32230_e42587_d_n10, assign32230_e42587_d_n11, assign32230_e42587_d_n12, assign32230_e42587_d_n13, assign32230_e42587_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32230_e42584: f64 = (locals.var_t4 / locals.var_t5);
        let assign32230_e42585: f64 = (locals.var_t3 - assign32230_e42584);
        (assign32230_e42585, (locals.var_t3_dn0 - (((locals.var_t4_dn0 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn2 - (((locals.var_t4_dn2 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn12 - (((locals.var_t4_dn12 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn13 - (((locals.var_t4_dn13 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn14 - (((locals.var_t4_dn14 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32230_e42587;
        locals.var_t3_dn0 = assign32230_e42587_d_n0;
        locals.var_t3_dn2 = assign32230_e42587_d_n2;
        locals.var_t3_dn3 = assign32230_e42587_d_n3;
        locals.var_t3_dn4 = assign32230_e42587_d_n4;
        locals.var_t3_dn5 = assign32230_e42587_d_n5;
        locals.var_t3_dn6 = assign32230_e42587_d_n6;
        locals.var_t3_dn7 = assign32230_e42587_d_n7;
        locals.var_t3_dn8 = assign32230_e42587_d_n8;
        locals.var_t3_dn9 = assign32230_e42587_d_n9;
        locals.var_t3_dn10 = assign32230_e42587_d_n10;
        locals.var_t3_dn11 = assign32230_e42587_d_n11;
        locals.var_t3_dn12 = assign32230_e42587_d_n12;
        locals.var_t3_dn13 = assign32230_e42587_d_n13;
        locals.var_t3_dn14 = assign32230_e42587_d_n14;

    }

    pub(super) fn stamp_transient_block_98(
        locals: &mut StampLocals,
    ) {
        let (assign32240_e42619, assign32240_e42619_d_n0, assign32240_e42619_d_n2, assign32240_e42619_d_n3, assign32240_e42619_d_n4, assign32240_e42619_d_n5, assign32240_e42619_d_n6, assign32240_e42619_d_n7, assign32240_e42619_d_n8, assign32240_e42619_d_n9, assign32240_e42619_d_n10, assign32240_e42619_d_n11, assign32240_e42619_d_n12, assign32240_e42619_d_n13, assign32240_e42619_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32240_e42594: f64 = (2.0 * locals.var_t3);
        let assign32240_e42598: f64 = (locals.var_t3 * 2.0);
        let assign32240_e42600: f64 = (assign32240_e42598 * locals.var_t0);
        let assign32240_e42603: f64 = (locals.var_t3 * 2.0);
        let assign32240_e42605: f64 = (assign32240_e42603 * locals.var_t0);
        let assign32240_e42608: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32240_e42609: f64 = (assign32240_e42605 + assign32240_e42608);
        let assign32240_e42610: f64 = (assign32240_e42600 * assign32240_e42609);
        let assign32240_e42612: f64 = (assign32240_e42610).max(1e-38);
        let assign32240_e42613: f64 = (assign32240_e42612).ln();
        let assign32240_e42614: f64 = assign32240_e42613;
        let assign32240_e42615: f64 = (assign32240_e42594 + assign32240_e42614);
        let assign32240_e42617: f64 = (assign32240_e42615 - locals.var_t1);
        (assign32240_e42617, (((2.0 * locals.var_t3_dn0) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn0)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn0), (((2.0 * locals.var_t3_dn2) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn2)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn2), (((2.0 * locals.var_t3_dn3) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn3)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn4)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn5)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn6)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn7)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn8)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn9)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn10)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn11)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn11), (((2.0 * locals.var_t3_dn12) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn12)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn12), (((2.0 * locals.var_t3_dn13) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn13)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn13), (((2.0 * locals.var_t3_dn14) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn14)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32240_e42619;
        locals.var_t4_dn0 = assign32240_e42619_d_n0;
        locals.var_t4_dn2 = assign32240_e42619_d_n2;
        locals.var_t4_dn3 = assign32240_e42619_d_n3;
        locals.var_t4_dn4 = assign32240_e42619_d_n4;
        locals.var_t4_dn5 = assign32240_e42619_d_n5;
        locals.var_t4_dn6 = assign32240_e42619_d_n6;
        locals.var_t4_dn7 = assign32240_e42619_d_n7;
        locals.var_t4_dn8 = assign32240_e42619_d_n8;
        locals.var_t4_dn9 = assign32240_e42619_d_n9;
        locals.var_t4_dn10 = assign32240_e42619_d_n10;
        locals.var_t4_dn11 = assign32240_e42619_d_n11;
        locals.var_t4_dn12 = assign32240_e42619_d_n12;
        locals.var_t4_dn13 = assign32240_e42619_d_n13;
        locals.var_t4_dn14 = assign32240_e42619_d_n14;

        let (assign32250_e42644, assign32250_e42644_d_n0, assign32250_e42644_d_n2, assign32250_e42644_d_n3, assign32250_e42644_d_n4, assign32250_e42644_d_n5, assign32250_e42644_d_n6, assign32250_e42644_d_n7, assign32250_e42644_d_n8, assign32250_e42644_d_n9, assign32250_e42644_d_n10, assign32250_e42644_d_n11, assign32250_e42644_d_n12, assign32250_e42644_d_n13, assign32250_e42644_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32250_e42627: f64 = 1.0;
        let assign32250_e42629: f64 = (assign32250_e42627 / locals.var_t3);
        let assign32250_e42630: f64 = (2.0 + assign32250_e42629);
        let assign32250_e42634: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32250_e42635: f64 = assign32250_e42634;
        let assign32250_e42638: f64 = (locals.var_t0 * locals.var_t3);
        let assign32250_e42640: f64 = (assign32250_e42638 + locals.var_sqrtpsisa);
        let assign32250_e42641: f64 = (assign32250_e42635 / assign32250_e42640);
        let assign32250_e42642: f64 = (assign32250_e42630 + assign32250_e42641);
        (assign32250_e42642, ((-((assign32250_e42627 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32250_e42640 * assign32250_e42640))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32250_e42644;
        locals.var_t5_dn0 = assign32250_e42644_d_n0;
        locals.var_t5_dn2 = assign32250_e42644_d_n2;
        locals.var_t5_dn3 = assign32250_e42644_d_n3;
        locals.var_t5_dn4 = assign32250_e42644_d_n4;
        locals.var_t5_dn5 = assign32250_e42644_d_n5;
        locals.var_t5_dn6 = assign32250_e42644_d_n6;
        locals.var_t5_dn7 = assign32250_e42644_d_n7;
        locals.var_t5_dn8 = assign32250_e42644_d_n8;
        locals.var_t5_dn9 = assign32250_e42644_d_n9;
        locals.var_t5_dn10 = assign32250_e42644_d_n10;
        locals.var_t5_dn11 = assign32250_e42644_d_n11;
        locals.var_t5_dn12 = assign32250_e42644_d_n12;
        locals.var_t5_dn13 = assign32250_e42644_d_n13;
        locals.var_t5_dn14 = assign32250_e42644_d_n14;

        let (assign32260_e42671, assign32260_e42671_d_n0, assign32260_e42671_d_n2, assign32260_e42671_d_n3, assign32260_e42671_d_n4, assign32260_e42671_d_n5, assign32260_e42671_d_n6, assign32260_e42671_d_n7, assign32260_e42671_d_n8, assign32260_e42671_d_n9, assign32260_e42671_d_n10, assign32260_e42671_d_n11, assign32260_e42671_d_n12, assign32260_e42671_d_n13, assign32260_e42671_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32260_e42652: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32260_e42655: f64 = (locals.var_t0 * locals.var_t3);
        let assign32260_e42657: f64 = (assign32260_e42655 + locals.var_sqrtpsisa);
        let assign32260_e42658: f64 = (assign32260_e42652 / assign32260_e42657);
        let assign32260_e42659: f64 = assign32260_e42658;
        let assign32260_e42662: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32260_e42665: f64 = (locals.var_t0 * locals.var_t3);
        let assign32260_e42667: f64 = (assign32260_e42665 + locals.var_sqrtpsisa);
        let assign32260_e42668: f64 = (assign32260_e42662 / assign32260_e42667);
        let assign32260_e42669: f64 = (assign32260_e42659 * assign32260_e42668);
        (assign32260_e42669, ((((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32260_e42667 * assign32260_e42667)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32260_e42671;
        locals.var_t6_dn0 = assign32260_e42671_d_n0;
        locals.var_t6_dn2 = assign32260_e42671_d_n2;
        locals.var_t6_dn3 = assign32260_e42671_d_n3;
        locals.var_t6_dn4 = assign32260_e42671_d_n4;
        locals.var_t6_dn5 = assign32260_e42671_d_n5;
        locals.var_t6_dn6 = assign32260_e42671_d_n6;
        locals.var_t6_dn7 = assign32260_e42671_d_n7;
        locals.var_t6_dn8 = assign32260_e42671_d_n8;
        locals.var_t6_dn9 = assign32260_e42671_d_n9;
        locals.var_t6_dn10 = assign32260_e42671_d_n10;
        locals.var_t6_dn11 = assign32260_e42671_d_n11;
        locals.var_t6_dn12 = assign32260_e42671_d_n12;
        locals.var_t6_dn13 = assign32260_e42671_d_n13;
        locals.var_t6_dn14 = assign32260_e42671_d_n14;

        let (assign32270_e42705, assign32270_e42705_d_n0, assign32270_e42705_d_n2, assign32270_e42705_d_n3, assign32270_e42705_d_n4, assign32270_e42705_d_n5, assign32270_e42705_d_n6, assign32270_e42705_d_n7, assign32270_e42705_d_n8, assign32270_e42705_d_n9, assign32270_e42705_d_n10, assign32270_e42705_d_n11, assign32270_e42705_d_n12, assign32270_e42705_d_n13, assign32270_e42705_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32270_e42677: f64 = (-1.0);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign32270_e42680: f64 = (1.0 * __rspice_inv_cse_0);
        let assign32270_e42683: f64 = (1.0 * __rspice_inv_cse_0);
        let assign32270_e42684: f64 = (assign32270_e42680 * assign32270_e42683);
        let assign32270_e42685: f64 = (assign32270_e42677 * assign32270_e42684);
        let assign32270_e42688: f64 = 1.0;
        let assign32270_e42691: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign32270_e42693: f64 = (assign32270_e42691 * locals.var_sqrtpsisa);
        let assign32270_e42696: f64 = (locals.var_t0 * locals.var_t3);
        let assign32270_e42698: f64 = (assign32270_e42696 + locals.var_sqrtpsisa);
        let assign32270_e42699: f64 = (assign32270_e42693 * assign32270_e42698);
        let assign32270_e42700: f64 = (assign32270_e42688 / assign32270_e42699);
        let assign32270_e42701: f64 = (assign32270_e42685 - assign32270_e42700);
        let assign32270_e42703: f64 = (assign32270_e42701 - locals.var_t6);
        (assign32270_e42703, (((assign32270_e42677 * (((-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn0 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn0)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn0)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn0), (((assign32270_e42677 * (((-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn2 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn2)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn2)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn2), (((assign32270_e42677 * (((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn3)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn3), (((assign32270_e42677 * (((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn4)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn4), (((assign32270_e42677 * (((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn5)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn5), (((assign32270_e42677 * (((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn6)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn6), (((assign32270_e42677 * (((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn7)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn7), (((assign32270_e42677 * (((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn8)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn8), (((assign32270_e42677 * (((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn9)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn9), (((assign32270_e42677 * (((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn10)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn10), (((assign32270_e42677 * (((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn11)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn11), (((assign32270_e42677 * (((-(locals.var_t3_dn12 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn12 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn12 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn12)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn12)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn12), (((assign32270_e42677 * (((-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn13 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn13)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn13)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn13), (((assign32270_e42677 * (((-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn14 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn14)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn14)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign32270_e42705;
        locals.var_t7_dn0 = assign32270_e42705_d_n0;
        locals.var_t7_dn2 = assign32270_e42705_d_n2;
        locals.var_t7_dn3 = assign32270_e42705_d_n3;
        locals.var_t7_dn4 = assign32270_e42705_d_n4;
        locals.var_t7_dn5 = assign32270_e42705_d_n5;
        locals.var_t7_dn6 = assign32270_e42705_d_n6;
        locals.var_t7_dn7 = assign32270_e42705_d_n7;
        locals.var_t7_dn8 = assign32270_e42705_d_n8;
        locals.var_t7_dn9 = assign32270_e42705_d_n9;
        locals.var_t7_dn10 = assign32270_e42705_d_n10;
        locals.var_t7_dn11 = assign32270_e42705_d_n11;
        locals.var_t7_dn12 = assign32270_e42705_d_n12;
        locals.var_t7_dn13 = assign32270_e42705_d_n13;
        locals.var_t7_dn14 = assign32270_e42705_d_n14;

        let (assign32280_e42728, assign32280_e42728_d_n0, assign32280_e42728_d_n2, assign32280_e42728_d_n3, assign32280_e42728_d_n4, assign32280_e42728_d_n5, assign32280_e42728_d_n6, assign32280_e42728_d_n7, assign32280_e42728_d_n8, assign32280_e42728_d_n9, assign32280_e42728_d_n10, assign32280_e42728_d_n11, assign32280_e42728_d_n12, assign32280_e42728_d_n13, assign32280_e42728_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32280_e42713: f64 = (locals.var_t4 / locals.var_t5);
        let assign32280_e42717: f64 = (locals.var_t4 * locals.var_t7);
        let assign32280_e42720: f64 = (2.0 * locals.var_t5);
        let assign32280_e42722: f64 = (assign32280_e42720 * locals.var_t5);
        let assign32280_e42723: f64 = (assign32280_e42717 / assign32280_e42722);
        let assign32280_e42724: f64 = (1.0 + assign32280_e42723);
        let assign32280_e42725: f64 = (assign32280_e42713 * assign32280_e42724);
        let assign32280_e42726: f64 = (locals.var_t3 - assign32280_e42725);
        (assign32280_e42726, (locals.var_t3_dn0 - (((((locals.var_t4_dn0 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn0 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn0)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn0) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn0)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn2 - (((((locals.var_t4_dn2 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn2 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn2)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn2) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn2)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn3)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn4)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn5)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn6)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn7)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn8)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn9)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn10)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn11)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn12 - (((((locals.var_t4_dn12 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn12 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn12)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn12) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn12)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn13 - (((((locals.var_t4_dn13 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn13 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn13)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn13) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn13)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn14 - (((((locals.var_t4_dn14 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn14 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn14)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn14) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn14)))) / (assign32280_e42722 * assign32280_e42722))))),)
    } else {
        (locals.var_qs_edge, locals.var_qs_edge_dn0, locals.var_qs_edge_dn2, locals.var_qs_edge_dn3, locals.var_qs_edge_dn4, locals.var_qs_edge_dn5, locals.var_qs_edge_dn6, locals.var_qs_edge_dn7, locals.var_qs_edge_dn8, locals.var_qs_edge_dn9, locals.var_qs_edge_dn10, locals.var_qs_edge_dn11, locals.var_qs_edge_dn12, locals.var_qs_edge_dn13, locals.var_qs_edge_dn14,)
    }
};
        locals.var_qs_edge = assign32280_e42728;
        locals.var_qs_edge_dn0 = assign32280_e42728_d_n0;
        locals.var_qs_edge_dn2 = assign32280_e42728_d_n2;
        locals.var_qs_edge_dn3 = assign32280_e42728_d_n3;
        locals.var_qs_edge_dn4 = assign32280_e42728_d_n4;
        locals.var_qs_edge_dn5 = assign32280_e42728_d_n5;
        locals.var_qs_edge_dn6 = assign32280_e42728_d_n6;
        locals.var_qs_edge_dn7 = assign32280_e42728_d_n7;
        locals.var_qs_edge_dn8 = assign32280_e42728_d_n8;
        locals.var_qs_edge_dn9 = assign32280_e42728_d_n9;
        locals.var_qs_edge_dn10 = assign32280_e42728_d_n10;
        locals.var_qs_edge_dn11 = assign32280_e42728_d_n11;
        locals.var_qs_edge_dn12 = assign32280_e42728_d_n12;
        locals.var_qs_edge_dn13 = assign32280_e42728_d_n13;
        locals.var_qs_edge_dn14 = assign32280_e42728_d_n14;

        let (assign32290_e42740, assign32290_e42740_d_n0, assign32290_e42740_d_n2, assign32290_e42740_d_n3, assign32290_e42740_d_n4, assign32290_e42740_d_n5, assign32290_e42740_d_n6, assign32290_e42740_d_n7, assign32290_e42740_d_n8, assign32290_e42740_d_n9, assign32290_e42740_d_n10, assign32290_e42740_d_n11, assign32290_e42740_d_n12, assign32290_e42740_d_n13, assign32290_e42740_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32290_e42732: f64 = (2.0 * locals.var_nvt);
        let assign32290_e42734: f64 = (assign32290_e42732 * locals.var_qs_edge);
        let assign32290_e42737: f64 = (2.0 * locals.var_nvt);
        let assign32290_e42738: f64 = (assign32290_e42734 + assign32290_e42737);
        (assign32290_e42738, ((((2.0 * locals.var_nvt_dn0) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn0)) + (2.0 * locals.var_nvt_dn0)), ((((2.0 * locals.var_nvt_dn2) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn2)) + (2.0 * locals.var_nvt_dn2)), ((((2.0 * locals.var_nvt_dn3) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn3)) + (2.0 * locals.var_nvt_dn3)), ((((2.0 * locals.var_nvt_dn4) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn4)) + (2.0 * locals.var_nvt_dn4)), ((((2.0 * locals.var_nvt_dn5) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn5)) + (2.0 * locals.var_nvt_dn5)), ((((2.0 * locals.var_nvt_dn6) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn6)) + (2.0 * locals.var_nvt_dn6)), ((((2.0 * locals.var_nvt_dn7) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn7)) + (2.0 * locals.var_nvt_dn7)), ((((2.0 * locals.var_nvt_dn8) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn8)) + (2.0 * locals.var_nvt_dn8)), ((((2.0 * locals.var_nvt_dn9) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn9)) + (2.0 * locals.var_nvt_dn9)), ((((2.0 * locals.var_nvt_dn10) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn10)) + (2.0 * locals.var_nvt_dn10)), ((((2.0 * locals.var_nvt_dn11) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn11)) + (2.0 * locals.var_nvt_dn11)), ((((2.0 * locals.var_nvt_dn12) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn12)) + (2.0 * locals.var_nvt_dn12)), ((((2.0 * locals.var_nvt_dn13) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn13)) + (2.0 * locals.var_nvt_dn13)), ((((2.0 * locals.var_nvt_dn14) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn14)) + (2.0 * locals.var_nvt_dn14)),)
    } else {
        (locals.var_vdsatedge, locals.var_vdsatedge_dn0, locals.var_vdsatedge_dn2, locals.var_vdsatedge_dn3, locals.var_vdsatedge_dn4, locals.var_vdsatedge_dn5, locals.var_vdsatedge_dn6, locals.var_vdsatedge_dn7, locals.var_vdsatedge_dn8, locals.var_vdsatedge_dn9, locals.var_vdsatedge_dn10, locals.var_vdsatedge_dn11, locals.var_vdsatedge_dn12, locals.var_vdsatedge_dn13, locals.var_vdsatedge_dn14,)
    }
};
        locals.var_vdsatedge = assign32290_e42740;
        locals.var_vdsatedge_dn0 = assign32290_e42740_d_n0;
        locals.var_vdsatedge_dn2 = assign32290_e42740_d_n2;
        locals.var_vdsatedge_dn3 = assign32290_e42740_d_n3;
        locals.var_vdsatedge_dn4 = assign32290_e42740_d_n4;
        locals.var_vdsatedge_dn5 = assign32290_e42740_d_n5;
        locals.var_vdsatedge_dn6 = assign32290_e42740_d_n6;
        locals.var_vdsatedge_dn7 = assign32290_e42740_d_n7;
        locals.var_vdsatedge_dn8 = assign32290_e42740_d_n8;
        locals.var_vdsatedge_dn9 = assign32290_e42740_d_n9;
        locals.var_vdsatedge_dn10 = assign32290_e42740_d_n10;
        locals.var_vdsatedge_dn11 = assign32290_e42740_d_n11;
        locals.var_vdsatedge_dn12 = assign32290_e42740_d_n12;
        locals.var_vdsatedge_dn13 = assign32290_e42740_d_n13;
        locals.var_vdsatedge_dn14 = assign32290_e42740_d_n14;

        let (assign32300_e42744, assign32300_e42744_d_n0, assign32300_e42744_d_n2, assign32300_e42744_d_n3, assign32300_e42744_d_n4, assign32300_e42744_d_n5, assign32300_e42744_d_n6, assign32300_e42744_d_n7, assign32300_e42744_d_n8, assign32300_e42744_d_n9, assign32300_e42744_d_n10, assign32300_e42744_d_n11, assign32300_e42744_d_n12, assign32300_e42744_d_n13, assign32300_e42744_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        (locals.var_vdsatedge, locals.var_vdsatedge_dn0, locals.var_vdsatedge_dn2, locals.var_vdsatedge_dn3, locals.var_vdsatedge_dn4, locals.var_vdsatedge_dn5, locals.var_vdsatedge_dn6, locals.var_vdsatedge_dn7, locals.var_vdsatedge_dn8, locals.var_vdsatedge_dn9, locals.var_vdsatedge_dn10, locals.var_vdsatedge_dn11, locals.var_vdsatedge_dn12, locals.var_vdsatedge_dn13, locals.var_vdsatedge_dn14,)
    } else {
        (locals.var_vdsatedge_1, locals.var_vdsatedge_1_dn0, locals.var_vdsatedge_1_dn2, locals.var_vdsatedge_1_dn3, locals.var_vdsatedge_1_dn4, locals.var_vdsatedge_1_dn5, locals.var_vdsatedge_1_dn6, locals.var_vdsatedge_1_dn7, locals.var_vdsatedge_1_dn8, locals.var_vdsatedge_1_dn9, locals.var_vdsatedge_1_dn10, locals.var_vdsatedge_1_dn11, locals.var_vdsatedge_1_dn12, locals.var_vdsatedge_1_dn13, locals.var_vdsatedge_1_dn14,)
    }
};
        locals.var_vdsatedge_1 = assign32300_e42744;
        locals.var_vdsatedge_1_dn0 = assign32300_e42744_d_n0;
        locals.var_vdsatedge_1_dn2 = assign32300_e42744_d_n2;
        locals.var_vdsatedge_1_dn3 = assign32300_e42744_d_n3;
        locals.var_vdsatedge_1_dn4 = assign32300_e42744_d_n4;
        locals.var_vdsatedge_1_dn5 = assign32300_e42744_d_n5;
        locals.var_vdsatedge_1_dn6 = assign32300_e42744_d_n6;
        locals.var_vdsatedge_1_dn7 = assign32300_e42744_d_n7;
        locals.var_vdsatedge_1_dn8 = assign32300_e42744_d_n8;
        locals.var_vdsatedge_1_dn9 = assign32300_e42744_d_n9;
        locals.var_vdsatedge_1_dn10 = assign32300_e42744_d_n10;
        locals.var_vdsatedge_1_dn11 = assign32300_e42744_d_n11;
        locals.var_vdsatedge_1_dn12 = assign32300_e42744_d_n12;
        locals.var_vdsatedge_1_dn13 = assign32300_e42744_d_n13;
        locals.var_vdsatedge_1_dn14 = assign32300_e42744_d_n14;

        let (assign32310_e42750, assign32310_e42750_d_n0, assign32310_e42750_d_n2, assign32310_e42750_d_n3, assign32310_e42750_d_n4, assign32310_e42750_d_n5, assign32310_e42750_d_n6, assign32310_e42750_d_n7, assign32310_e42750_d_n8, assign32310_e42750_d_n9, assign32310_e42750_d_n10, assign32310_e42750_d_n11, assign32310_e42750_d_n12, assign32310_e42750_d_n13, assign32310_e42750_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32310_e42748: f64 = (locals.var_vdsatedge_1 + locals.var_vs);
        (assign32310_e42748, locals.var_vdsatedge_1_dn0, locals.var_vdsatedge_1_dn2, locals.var_vdsatedge_1_dn3, locals.var_vdsatedge_1_dn4, (locals.var_vdsatedge_1_dn5 + locals.var_vs_dn5), locals.var_vdsatedge_1_dn6, (locals.var_vdsatedge_1_dn7 + locals.var_vs_dn7), locals.var_vdsatedge_1_dn8, locals.var_vdsatedge_1_dn9, locals.var_vdsatedge_1_dn10, (locals.var_vdsatedge_1_dn11 + locals.var_vs_dn11), locals.var_vdsatedge_1_dn12, locals.var_vdsatedge_1_dn13, locals.var_vdsatedge_1_dn14,)
    } else {
        (locals.var_vdsatedge_1, locals.var_vdsatedge_1_dn0, locals.var_vdsatedge_1_dn2, locals.var_vdsatedge_1_dn3, locals.var_vdsatedge_1_dn4, locals.var_vdsatedge_1_dn5, locals.var_vdsatedge_1_dn6, locals.var_vdsatedge_1_dn7, locals.var_vdsatedge_1_dn8, locals.var_vdsatedge_1_dn9, locals.var_vdsatedge_1_dn10, locals.var_vdsatedge_1_dn11, locals.var_vdsatedge_1_dn12, locals.var_vdsatedge_1_dn13, locals.var_vdsatedge_1_dn14,)
    }
};
        locals.var_vdsatedge_1 = assign32310_e42750;
        locals.var_vdsatedge_1_dn0 = assign32310_e42750_d_n0;
        locals.var_vdsatedge_1_dn2 = assign32310_e42750_d_n2;
        locals.var_vdsatedge_1_dn3 = assign32310_e42750_d_n3;
        locals.var_vdsatedge_1_dn4 = assign32310_e42750_d_n4;
        locals.var_vdsatedge_1_dn5 = assign32310_e42750_d_n5;
        locals.var_vdsatedge_1_dn6 = assign32310_e42750_d_n6;
        locals.var_vdsatedge_1_dn7 = assign32310_e42750_d_n7;
        locals.var_vdsatedge_1_dn8 = assign32310_e42750_d_n8;
        locals.var_vdsatedge_1_dn9 = assign32310_e42750_d_n9;
        locals.var_vdsatedge_1_dn10 = assign32310_e42750_d_n10;
        locals.var_vdsatedge_1_dn11 = assign32310_e42750_d_n11;
        locals.var_vdsatedge_1_dn12 = assign32310_e42750_d_n12;
        locals.var_vdsatedge_1_dn13 = assign32310_e42750_d_n13;
        locals.var_vdsatedge_1_dn14 = assign32310_e42750_d_n14;

        let assign32320_e42756: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign32320_e42758: f64 = (-2500.0);
        let assign32320_e42760: f64 = (assign32320_e42758 * 0.001);
        let assign32320_e42762: f64 = if ((0.0 == 0.0) && (assign32320_e42756 < assign32320_e42760)) { 1.0 } else { 0.0 };
        locals.var_guard739 = assign32320_e42762;

        let (assign32330_e42777, assign32330_e42777_d_n0, assign32330_e42777_d_n2, assign32330_e42777_d_n3, assign32330_e42777_d_n4, assign32330_e42777_d_n5, assign32330_e42777_d_n6, assign32330_e42777_d_n7, assign32330_e42777_d_n8, assign32330_e42777_d_n9, assign32330_e42777_d_n10, assign32330_e42777_d_n11, assign32330_e42777_d_n12, assign32330_e42777_d_n13, assign32330_e42777_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign32330_e42767: f64 = (-0.001);
        let assign32330_e42769: f64 = (assign32330_e42767 * 0.001);
        let assign32330_e42773: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign32330_e42774: f64 = (16.0 * assign32330_e42773);
        let assign32330_e42775: f64 = (assign32330_e42769 / assign32330_e42774);
        (assign32330_e42775, (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn0)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn2)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn3)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn4)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * (locals.var_vdsatedge_1_dn5 - locals.var_vs_dn5))) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn6)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * (locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7))) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn8)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn9)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn10)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * (locals.var_vdsatedge_1_dn11 - locals.var_vs_dn11))) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn12)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn13)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn14)) / (assign32330_e42774 * assign32330_e42774))),)
    } else {
        (locals.var_vdssate, locals.var_vdssate_dn0, locals.var_vdssate_dn2, locals.var_vdssate_dn3, locals.var_vdssate_dn4, locals.var_vdssate_dn5, locals.var_vdssate_dn6, locals.var_vdssate_dn7, locals.var_vdssate_dn8, locals.var_vdssate_dn9, locals.var_vdssate_dn10, locals.var_vdssate_dn11, locals.var_vdssate_dn12, locals.var_vdssate_dn13, locals.var_vdssate_dn14,)
    }
};
        locals.var_vdssate = assign32330_e42777;
        locals.var_vdssate_dn0 = assign32330_e42777_d_n0;
        locals.var_vdssate_dn2 = assign32330_e42777_d_n2;
        locals.var_vdssate_dn3 = assign32330_e42777_d_n3;
        locals.var_vdssate_dn4 = assign32330_e42777_d_n4;
        locals.var_vdssate_dn5 = assign32330_e42777_d_n5;
        locals.var_vdssate_dn6 = assign32330_e42777_d_n6;
        locals.var_vdssate_dn7 = assign32330_e42777_d_n7;
        locals.var_vdssate_dn8 = assign32330_e42777_d_n8;
        locals.var_vdssate_dn9 = assign32330_e42777_d_n9;
        locals.var_vdssate_dn10 = assign32330_e42777_d_n10;
        locals.var_vdssate_dn11 = assign32330_e42777_d_n11;
        locals.var_vdssate_dn12 = assign32330_e42777_d_n12;
        locals.var_vdssate_dn13 = assign32330_e42777_d_n13;
        locals.var_vdssate_dn14 = assign32330_e42777_d_n14;

        let (assign32340_e42809, assign32340_e42809_d_n0, assign32340_e42809_d_n2, assign32340_e42809_d_n3, assign32340_e42809_d_n4, assign32340_e42809_d_n5, assign32340_e42809_d_n6, assign32340_e42809_d_n7, assign32340_e42809_d_n8, assign32340_e42809_d_n9, assign32340_e42809_d_n10, assign32340_e42809_d_n11, assign32340_e42809_d_n12, assign32340_e42809_d_n13, assign32340_e42809_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard739 == 0.0)) {
        let assign32340_e42785: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign32340_e42787: f64 = assign32340_e42785;
        let assign32340_e42790: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign32340_e42792: f64 = assign32340_e42790;
        let assign32340_e42795: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign32340_e42797: f64 = assign32340_e42795;
        let assign32340_e42798: f64 = (assign32340_e42792 * assign32340_e42797);
        let assign32340_e42801: f64 = (0.25 * 0.001);
        let assign32340_e42803: f64 = (assign32340_e42801 * 0.001);
        let assign32340_e42804: f64 = (assign32340_e42798 + assign32340_e42803);
        let assign32340_e42805: f64 = (assign32340_e42804).sqrt();
        let assign32340_e42806: f64 = (assign32340_e42787 + assign32340_e42805);
        let assign32340_e42807: f64 = (0.5 * assign32340_e42806);
        (assign32340_e42807, (0.5 * (locals.var_vdsatedge_1_dn0 + (((locals.var_vdsatedge_1_dn0 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn0)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn2 + (((locals.var_vdsatedge_1_dn2 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn2)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn3 + (((locals.var_vdsatedge_1_dn3 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn3)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn4 + (((locals.var_vdsatedge_1_dn4 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn4)) / (2.0 * assign32340_e42805)))), (0.5 * ((locals.var_vdsatedge_1_dn5 - locals.var_vs_dn5) + ((((locals.var_vdsatedge_1_dn5 - locals.var_vs_dn5) * assign32340_e42797) + (assign32340_e42792 * (locals.var_vdsatedge_1_dn5 - locals.var_vs_dn5))) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn6 + (((locals.var_vdsatedge_1_dn6 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn6)) / (2.0 * assign32340_e42805)))), (0.5 * ((locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7) + ((((locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7) * assign32340_e42797) + (assign32340_e42792 * (locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7))) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn8 + (((locals.var_vdsatedge_1_dn8 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn8)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn9 + (((locals.var_vdsatedge_1_dn9 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn9)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn10 + (((locals.var_vdsatedge_1_dn10 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn10)) / (2.0 * assign32340_e42805)))), (0.5 * ((locals.var_vdsatedge_1_dn11 - locals.var_vs_dn11) + ((((locals.var_vdsatedge_1_dn11 - locals.var_vs_dn11) * assign32340_e42797) + (assign32340_e42792 * (locals.var_vdsatedge_1_dn11 - locals.var_vs_dn11))) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn12 + (((locals.var_vdsatedge_1_dn12 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn12)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn13 + (((locals.var_vdsatedge_1_dn13 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn13)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn14 + (((locals.var_vdsatedge_1_dn14 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn14)) / (2.0 * assign32340_e42805)))),)
    } else {
        (locals.var_vdssate, locals.var_vdssate_dn0, locals.var_vdssate_dn2, locals.var_vdssate_dn3, locals.var_vdssate_dn4, locals.var_vdssate_dn5, locals.var_vdssate_dn6, locals.var_vdssate_dn7, locals.var_vdssate_dn8, locals.var_vdssate_dn9, locals.var_vdssate_dn10, locals.var_vdssate_dn11, locals.var_vdssate_dn12, locals.var_vdssate_dn13, locals.var_vdssate_dn14,)
    }
};
        locals.var_vdssate = assign32340_e42809;
        locals.var_vdssate_dn0 = assign32340_e42809_d_n0;
        locals.var_vdssate_dn2 = assign32340_e42809_d_n2;
        locals.var_vdssate_dn3 = assign32340_e42809_d_n3;
        locals.var_vdssate_dn4 = assign32340_e42809_d_n4;
        locals.var_vdssate_dn5 = assign32340_e42809_d_n5;
        locals.var_vdssate_dn6 = assign32340_e42809_d_n6;
        locals.var_vdssate_dn7 = assign32340_e42809_d_n7;
        locals.var_vdssate_dn8 = assign32340_e42809_d_n8;
        locals.var_vdssate_dn9 = assign32340_e42809_d_n9;
        locals.var_vdssate_dn10 = assign32340_e42809_d_n10;
        locals.var_vdssate_dn11 = assign32340_e42809_d_n11;
        locals.var_vdssate_dn12 = assign32340_e42809_d_n12;
        locals.var_vdssate_dn13 = assign32340_e42809_d_n13;
        locals.var_vdssate_dn14 = assign32340_e42809_d_n14;

        let (assign32350_e42819, assign32350_e42819_d_n0, assign32350_e42819_d_n2, assign32350_e42819_d_n3, assign32350_e42819_d_n4, assign32350_e42819_d_n5, assign32350_e42819_d_n6, assign32350_e42819_d_n7, assign32350_e42819_d_n8, assign32350_e42819_d_n9, assign32350_e42819_d_n10, assign32350_e42819_d_n11, assign32350_e42819_d_n12, assign32350_e42819_d_n13, assign32350_e42819_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32350_e42813: f64 = (locals.var_vds / locals.var_vdssate);
        let assign32350_e42816: f64 = (1.0 / locals.var_delta_t);
        let assign32350_e42817: f64 = (assign32350_e42813).powf(assign32350_e42816);
        (assign32350_e42817, if (-(locals.var_delta_t_dn0 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn0) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn0 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn0) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn2 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn2) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn2 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn2) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn3) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn3) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn4) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn4) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (((locals.var_vds_dn5 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn5)) / (locals.var_vdssate * locals.var_vdssate)))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((((locals.var_vds_dn5 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn5)) / (locals.var_vdssate * locals.var_vdssate)) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn6) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn6) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (((locals.var_vds_dn7 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn7)) / (locals.var_vdssate * locals.var_vdssate)))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((((locals.var_vds_dn7 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn7)) / (locals.var_vdssate * locals.var_vdssate)) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn8) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn8) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn9) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn9) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn10) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn10) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (((locals.var_vds_dn11 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn11)) / (locals.var_vdssate * locals.var_vdssate)))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((((locals.var_vds_dn11 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn11)) / (locals.var_vdssate * locals.var_vdssate)) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn12 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn12) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn12 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn12) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn13 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn13) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn13 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn13) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn14 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn14) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn14 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn14) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) },)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign32350_e42819;
        locals.var_t7_dn0 = assign32350_e42819_d_n0;
        locals.var_t7_dn2 = assign32350_e42819_d_n2;
        locals.var_t7_dn3 = assign32350_e42819_d_n3;
        locals.var_t7_dn4 = assign32350_e42819_d_n4;
        locals.var_t7_dn5 = assign32350_e42819_d_n5;
        locals.var_t7_dn6 = assign32350_e42819_d_n6;
        locals.var_t7_dn7 = assign32350_e42819_d_n7;
        locals.var_t7_dn8 = assign32350_e42819_d_n8;
        locals.var_t7_dn9 = assign32350_e42819_d_n9;
        locals.var_t7_dn10 = assign32350_e42819_d_n10;
        locals.var_t7_dn11 = assign32350_e42819_d_n11;
        locals.var_t7_dn12 = assign32350_e42819_d_n12;
        locals.var_t7_dn13 = assign32350_e42819_d_n13;
        locals.var_t7_dn14 = assign32350_e42819_d_n14;

        let (assign32360_e42828, assign32360_e42828_d_n0, assign32360_e42828_d_n2, assign32360_e42828_d_n3, assign32360_e42828_d_n4, assign32360_e42828_d_n5, assign32360_e42828_d_n6, assign32360_e42828_d_n7, assign32360_e42828_d_n8, assign32360_e42828_d_n9, assign32360_e42828_d_n10, assign32360_e42828_d_n11, assign32360_e42828_d_n12, assign32360_e42828_d_n13, assign32360_e42828_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32360_e42823: f64 = (1.0 + locals.var_t7);
        let assign32360_e42825: f64 = (-locals.var_delta_t);
        let assign32360_e42826: f64 = (assign32360_e42823).powf(assign32360_e42825);
        (assign32360_e42826, if (-locals.var_delta_t_dn0) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn0)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn0) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn0 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn2) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn2)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn2) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn2 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn3) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn3)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn3) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn3 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn4) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn4)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn4) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn4 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn5) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn5)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn5) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn5 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn6) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn6)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn6) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn6 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn7) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn7)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn7) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn7 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn8) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn8)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn8) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn8 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn9) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn9)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn9) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn9 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn10) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn10)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn10) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn10 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn11) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn11)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn11) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn11 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn12) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn12)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn12) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn12 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn13) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn13)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn13) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn13 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn14) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn14)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn14) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn14 / assign32360_e42823)))) },)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign32360_e42828;
        locals.var_t8_dn0 = assign32360_e42828_d_n0;
        locals.var_t8_dn2 = assign32360_e42828_d_n2;
        locals.var_t8_dn3 = assign32360_e42828_d_n3;
        locals.var_t8_dn4 = assign32360_e42828_d_n4;
        locals.var_t8_dn5 = assign32360_e42828_d_n5;
        locals.var_t8_dn6 = assign32360_e42828_d_n6;
        locals.var_t8_dn7 = assign32360_e42828_d_n7;
        locals.var_t8_dn8 = assign32360_e42828_d_n8;
        locals.var_t8_dn9 = assign32360_e42828_d_n9;
        locals.var_t8_dn10 = assign32360_e42828_d_n10;
        locals.var_t8_dn11 = assign32360_e42828_d_n11;
        locals.var_t8_dn12 = assign32360_e42828_d_n12;
        locals.var_t8_dn13 = assign32360_e42828_d_n13;
        locals.var_t8_dn14 = assign32360_e42828_d_n14;

        let (assign32370_e42834, assign32370_e42834_d_n0, assign32370_e42834_d_n2, assign32370_e42834_d_n3, assign32370_e42834_d_n4, assign32370_e42834_d_n5, assign32370_e42834_d_n6, assign32370_e42834_d_n7, assign32370_e42834_d_n8, assign32370_e42834_d_n9, assign32370_e42834_d_n10, assign32370_e42834_d_n11, assign32370_e42834_d_n12, assign32370_e42834_d_n13, assign32370_e42834_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32370_e42832: f64 = (locals.var_vds * locals.var_t8);
        (assign32370_e42832, (locals.var_vds * locals.var_t8_dn0), (locals.var_vds * locals.var_t8_dn2), (locals.var_vds * locals.var_t8_dn3), (locals.var_vds * locals.var_t8_dn4), ((locals.var_vds_dn5 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn5)), (locals.var_vds * locals.var_t8_dn6), ((locals.var_vds_dn7 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn7)), (locals.var_vds * locals.var_t8_dn8), (locals.var_vds * locals.var_t8_dn9), (locals.var_vds * locals.var_t8_dn10), ((locals.var_vds_dn11 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn11)), (locals.var_vds * locals.var_t8_dn12), (locals.var_vds * locals.var_t8_dn13), (locals.var_vds * locals.var_t8_dn14),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn3, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn13, locals.var_vdseff_dn14,)
    }
};
        locals.var_vdseff = assign32370_e42834;
        locals.var_vdseff_dn0 = assign32370_e42834_d_n0;
        locals.var_vdseff_dn2 = assign32370_e42834_d_n2;
        locals.var_vdseff_dn3 = assign32370_e42834_d_n3;
        locals.var_vdseff_dn4 = assign32370_e42834_d_n4;
        locals.var_vdseff_dn5 = assign32370_e42834_d_n5;
        locals.var_vdseff_dn6 = assign32370_e42834_d_n6;
        locals.var_vdseff_dn7 = assign32370_e42834_d_n7;
        locals.var_vdseff_dn8 = assign32370_e42834_d_n8;
        locals.var_vdseff_dn9 = assign32370_e42834_d_n9;
        locals.var_vdseff_dn10 = assign32370_e42834_d_n10;
        locals.var_vdseff_dn11 = assign32370_e42834_d_n11;
        locals.var_vdseff_dn12 = assign32370_e42834_d_n12;
        locals.var_vdseff_dn13 = assign32370_e42834_d_n13;
        locals.var_vdseff_dn14 = assign32370_e42834_d_n14;

        let (assign32380_e42842, assign32380_e42842_d_n0, assign32380_e42842_d_n2, assign32380_e42842_d_n3, assign32380_e42842_d_n4, assign32380_e42842_d_n5, assign32380_e42842_d_n6, assign32380_e42842_d_n7, assign32380_e42842_d_n8, assign32380_e42842_d_n9, assign32380_e42842_d_n10, assign32380_e42842_d_n11, assign32380_e42842_d_n12, assign32380_e42842_d_n13, assign32380_e42842_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32380_e42838: f64 = (locals.var_vdseff + locals.var_vs);
        let assign32380_e42840: f64 = (assign32380_e42838 * locals.var_inv_nvt);
        (assign32380_e42840, ((locals.var_vdseff_dn0 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn0)), ((locals.var_vdseff_dn2 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn2)), ((locals.var_vdseff_dn3 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn3)), ((locals.var_vdseff_dn4 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn4)), (((locals.var_vdseff_dn5 + locals.var_vs_dn5) * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn5)), ((locals.var_vdseff_dn6 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn6)), (((locals.var_vdseff_dn7 + locals.var_vs_dn7) * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn7)), ((locals.var_vdseff_dn8 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn8)), ((locals.var_vdseff_dn9 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn9)), ((locals.var_vdseff_dn10 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn10)), (((locals.var_vdseff_dn11 + locals.var_vs_dn11) * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn11)), ((locals.var_vdseff_dn12 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn12)), ((locals.var_vdseff_dn13 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn13)), ((locals.var_vdseff_dn14 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn14)),)
    } else {
        (locals.var_vdeff, locals.var_vdeff_dn0, locals.var_vdeff_dn2, locals.var_vdeff_dn3, locals.var_vdeff_dn4, locals.var_vdeff_dn5, locals.var_vdeff_dn6, locals.var_vdeff_dn7, locals.var_vdeff_dn8, locals.var_vdeff_dn9, locals.var_vdeff_dn10, locals.var_vdeff_dn11, locals.var_vdeff_dn12, locals.var_vdeff_dn13, locals.var_vdeff_dn14,)
    }
};
        locals.var_vdeff = assign32380_e42842;
        locals.var_vdeff_dn0 = assign32380_e42842_d_n0;
        locals.var_vdeff_dn2 = assign32380_e42842_d_n2;
        locals.var_vdeff_dn3 = assign32380_e42842_d_n3;
        locals.var_vdeff_dn4 = assign32380_e42842_d_n4;
        locals.var_vdeff_dn5 = assign32380_e42842_d_n5;
        locals.var_vdeff_dn6 = assign32380_e42842_d_n6;
        locals.var_vdeff_dn7 = assign32380_e42842_d_n7;
        locals.var_vdeff_dn8 = assign32380_e42842_d_n8;
        locals.var_vdeff_dn9 = assign32380_e42842_d_n9;
        locals.var_vdeff_dn10 = assign32380_e42842_d_n10;
        locals.var_vdeff_dn11 = assign32380_e42842_d_n11;
        locals.var_vdeff_dn12 = assign32380_e42842_d_n12;
        locals.var_vdeff_dn13 = assign32380_e42842_d_n13;
        locals.var_vdeff_dn14 = assign32380_e42842_d_n14;

        let (assign32390_e42865, assign32390_e42865_d_n0, assign32390_e42865_d_n2, assign32390_e42865_d_n3, assign32390_e42865_d_n4, assign32390_e42865_d_n5, assign32390_e42865_d_n6, assign32390_e42865_d_n7, assign32390_e42865_d_n8, assign32390_e42865_d_n9, assign32390_e42865_d_n10, assign32390_e42865_d_n11, assign32390_e42865_d_n12, assign32390_e42865_d_n13, assign32390_e42865_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32390_e42847: f64 = (locals.var_psip + 1.0);
        let assign32390_e42850: f64 = (locals.var_psip - 1.0);
        let assign32390_e42853: f64 = (locals.var_psip - 1.0);
        let assign32390_e42854: f64 = (assign32390_e42850 * assign32390_e42853);
        let assign32390_e42857: f64 = (0.25 * 2.0);
        let assign32390_e42859: f64 = (assign32390_e42857 * 2.0);
        let assign32390_e42860: f64 = (assign32390_e42854 + assign32390_e42859);
        let assign32390_e42861: f64 = (assign32390_e42860).sqrt();
        let assign32390_e42862: f64 = (assign32390_e42847 + assign32390_e42861);
        let assign32390_e42863: f64 = (0.5 * assign32390_e42862);
        (assign32390_e42863, (0.5 * (locals.var_psip_dn0 + (((locals.var_psip_dn0 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn0)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn2 + (((locals.var_psip_dn2 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn2)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn3)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn4)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn5)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn6)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn7)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn8)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn9)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn10)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn11)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn12 + (((locals.var_psip_dn12 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn12)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn13 + (((locals.var_psip_dn13 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn13)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn14 + (((locals.var_psip_dn14 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn14)) / (2.0 * assign32390_e42861)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign32390_e42865;
        locals.var_t8_dn0 = assign32390_e42865_d_n0;
        locals.var_t8_dn2 = assign32390_e42865_d_n2;
        locals.var_t8_dn3 = assign32390_e42865_d_n3;
        locals.var_t8_dn4 = assign32390_e42865_d_n4;
        locals.var_t8_dn5 = assign32390_e42865_d_n5;
        locals.var_t8_dn6 = assign32390_e42865_d_n6;
        locals.var_t8_dn7 = assign32390_e42865_d_n7;
        locals.var_t8_dn8 = assign32390_e42865_d_n8;
        locals.var_t8_dn9 = assign32390_e42865_d_n9;
        locals.var_t8_dn10 = assign32390_e42865_d_n10;
        locals.var_t8_dn11 = assign32390_e42865_d_n11;
        locals.var_t8_dn12 = assign32390_e42865_d_n12;
        locals.var_t8_dn13 = assign32390_e42865_d_n13;
        locals.var_t8_dn14 = assign32390_e42865_d_n14;

        let (assign32400_e42870, assign32400_e42870_d_n0, assign32400_e42870_d_n2, assign32400_e42870_d_n3, assign32400_e42870_d_n4, assign32400_e42870_d_n5, assign32400_e42870_d_n6, assign32400_e42870_d_n7, assign32400_e42870_d_n8, assign32400_e42870_d_n9, assign32400_e42870_d_n10, assign32400_e42870_d_n11, assign32400_e42870_d_n12, assign32400_e42870_d_n13, assign32400_e42870_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32400_e42868: f64 = (locals.var_t8).sqrt();
        (assign32400_e42868, (locals.var_t8_dn0 / (2.0 * assign32400_e42868)), (locals.var_t8_dn2 / (2.0 * assign32400_e42868)), (locals.var_t8_dn3 / (2.0 * assign32400_e42868)), (locals.var_t8_dn4 / (2.0 * assign32400_e42868)), (locals.var_t8_dn5 / (2.0 * assign32400_e42868)), (locals.var_t8_dn6 / (2.0 * assign32400_e42868)), (locals.var_t8_dn7 / (2.0 * assign32400_e42868)), (locals.var_t8_dn8 / (2.0 * assign32400_e42868)), (locals.var_t8_dn9 / (2.0 * assign32400_e42868)), (locals.var_t8_dn10 / (2.0 * assign32400_e42868)), (locals.var_t8_dn11 / (2.0 * assign32400_e42868)), (locals.var_t8_dn12 / (2.0 * assign32400_e42868)), (locals.var_t8_dn13 / (2.0 * assign32400_e42868)), (locals.var_t8_dn14 / (2.0 * assign32400_e42868)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    }
};
        locals.var_sqrtpsip = assign32400_e42870;
        locals.var_sqrtpsip_dn0 = assign32400_e42870_d_n0;
        locals.var_sqrtpsip_dn2 = assign32400_e42870_d_n2;
        locals.var_sqrtpsip_dn3 = assign32400_e42870_d_n3;
        locals.var_sqrtpsip_dn4 = assign32400_e42870_d_n4;
        locals.var_sqrtpsip_dn5 = assign32400_e42870_d_n5;
        locals.var_sqrtpsip_dn6 = assign32400_e42870_d_n6;
        locals.var_sqrtpsip_dn7 = assign32400_e42870_d_n7;
        locals.var_sqrtpsip_dn8 = assign32400_e42870_d_n8;
        locals.var_sqrtpsip_dn9 = assign32400_e42870_d_n9;
        locals.var_sqrtpsip_dn10 = assign32400_e42870_d_n10;
        locals.var_sqrtpsip_dn11 = assign32400_e42870_d_n11;
        locals.var_sqrtpsip_dn12 = assign32400_e42870_d_n12;
        locals.var_sqrtpsip_dn13 = assign32400_e42870_d_n13;
        locals.var_sqrtpsip_dn14 = assign32400_e42870_d_n14;

        let (assign32410_e42882, assign32410_e42882_d_n0, assign32410_e42882_d_n2, assign32410_e42882_d_n3, assign32410_e42882_d_n4, assign32410_e42882_d_n5, assign32410_e42882_d_n6, assign32410_e42882_d_n7, assign32410_e42882_d_n8, assign32410_e42882_d_n9, assign32410_e42882_d_n10, assign32410_e42882_d_n11, assign32410_e42882_d_n12, assign32410_e42882_d_n13, assign32410_e42882_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32410_e42876: f64 = (2.0 * locals.var_sqrtpsip);
        let assign32410_e42877: f64 = (locals.var_gam_edge / assign32410_e42876);
        let assign32410_e42878: f64 = (1.0 + assign32410_e42877);
        let assign32410_e42880: f64 = (assign32410_e42878 / locals.var_gam_edge);
        (assign32410_e42880, ((((((locals.var_gam_edge_dn0 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn0))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn0)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn2 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn2))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn2)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn3 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn3))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn3)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn4 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn4))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn4)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn5 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn5))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn5)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn6 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn6))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn6)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn7 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn7))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn7)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn8 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn8))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn8)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn9 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn9))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn9)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn10 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn10))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn10)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn11 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn11))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn11)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn12 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn12))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn12)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn13 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn13))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn13)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn14 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn14))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn14)) / (locals.var_gam_edge * locals.var_gam_edge)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32410_e42882;
        locals.var_t0_dn0 = assign32410_e42882_d_n0;
        locals.var_t0_dn2 = assign32410_e42882_d_n2;
        locals.var_t0_dn3 = assign32410_e42882_d_n3;
        locals.var_t0_dn4 = assign32410_e42882_d_n4;
        locals.var_t0_dn5 = assign32410_e42882_d_n5;
        locals.var_t0_dn6 = assign32410_e42882_d_n6;
        locals.var_t0_dn7 = assign32410_e42882_d_n7;
        locals.var_t0_dn8 = assign32410_e42882_d_n8;
        locals.var_t0_dn9 = assign32410_e42882_d_n9;
        locals.var_t0_dn10 = assign32410_e42882_d_n10;
        locals.var_t0_dn11 = assign32410_e42882_d_n11;
        locals.var_t0_dn12 = assign32410_e42882_d_n12;
        locals.var_t0_dn13 = assign32410_e42882_d_n13;
        locals.var_t0_dn14 = assign32410_e42882_d_n14;

    }

    pub(super) fn stamp_transient_block_99(
        locals: &mut StampLocals,
    ) {
        let (assign32420_e42892, assign32420_e42892_d_n0, assign32420_e42892_d_n2, assign32420_e42892_d_n3, assign32420_e42892_d_n4, assign32420_e42892_d_n5, assign32420_e42892_d_n6, assign32420_e42892_d_n7, assign32420_e42892_d_n8, assign32420_e42892_d_n9, assign32420_e42892_d_n10, assign32420_e42892_d_n11, assign32420_e42892_d_n12, assign32420_e42892_d_n13, assign32420_e42892_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32420_e42887: f64 = (2.0 * locals.var_phib_n_edge);
        let assign32420_e42888: f64 = (locals.var_psip - assign32420_e42887);
        let assign32420_e42890: f64 = (assign32420_e42888 - locals.var_vdeff);
        (assign32420_e42890, ((locals.var_psip_dn0 - (2.0 * locals.var_phib_n_edge_dn0)) - locals.var_vdeff_dn0), ((locals.var_psip_dn2 - (2.0 * locals.var_phib_n_edge_dn2)) - locals.var_vdeff_dn2), ((locals.var_psip_dn3 - (2.0 * locals.var_phib_n_edge_dn3)) - locals.var_vdeff_dn3), ((locals.var_psip_dn4 - (2.0 * locals.var_phib_n_edge_dn4)) - locals.var_vdeff_dn4), ((locals.var_psip_dn5 - (2.0 * locals.var_phib_n_edge_dn5)) - locals.var_vdeff_dn5), ((locals.var_psip_dn6 - (2.0 * locals.var_phib_n_edge_dn6)) - locals.var_vdeff_dn6), ((locals.var_psip_dn7 - (2.0 * locals.var_phib_n_edge_dn7)) - locals.var_vdeff_dn7), ((locals.var_psip_dn8 - (2.0 * locals.var_phib_n_edge_dn8)) - locals.var_vdeff_dn8), ((locals.var_psip_dn9 - (2.0 * locals.var_phib_n_edge_dn9)) - locals.var_vdeff_dn9), ((locals.var_psip_dn10 - (2.0 * locals.var_phib_n_edge_dn10)) - locals.var_vdeff_dn10), ((locals.var_psip_dn11 - (2.0 * locals.var_phib_n_edge_dn11)) - locals.var_vdeff_dn11), ((locals.var_psip_dn12 - (2.0 * locals.var_phib_n_edge_dn12)) - locals.var_vdeff_dn12), ((locals.var_psip_dn13 - (2.0 * locals.var_phib_n_edge_dn13)) - locals.var_vdeff_dn13), ((locals.var_psip_dn14 - (2.0 * locals.var_phib_n_edge_dn14)) - locals.var_vdeff_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32420_e42892;
        locals.var_t1_dn0 = assign32420_e42892_d_n0;
        locals.var_t1_dn2 = assign32420_e42892_d_n2;
        locals.var_t1_dn3 = assign32420_e42892_d_n3;
        locals.var_t1_dn4 = assign32420_e42892_d_n4;
        locals.var_t1_dn5 = assign32420_e42892_d_n5;
        locals.var_t1_dn6 = assign32420_e42892_d_n6;
        locals.var_t1_dn7 = assign32420_e42892_d_n7;
        locals.var_t1_dn8 = assign32420_e42892_d_n8;
        locals.var_t1_dn9 = assign32420_e42892_d_n9;
        locals.var_t1_dn10 = assign32420_e42892_d_n10;
        locals.var_t1_dn11 = assign32420_e42892_d_n11;
        locals.var_t1_dn12 = assign32420_e42892_d_n12;
        locals.var_t1_dn13 = assign32420_e42892_d_n13;
        locals.var_t1_dn14 = assign32420_e42892_d_n14;

        let (assign32430_e42907, assign32430_e42907_d_n0, assign32430_e42907_d_n2, assign32430_e42907_d_n3, assign32430_e42907_d_n4, assign32430_e42907_d_n5, assign32430_e42907_d_n6, assign32430_e42907_d_n7, assign32430_e42907_d_n8, assign32430_e42907_d_n9, assign32430_e42907_d_n10, assign32430_e42907_d_n11, assign32430_e42907_d_n12, assign32430_e42907_d_n13, assign32430_e42907_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32430_e42896: f64 = locals.var_t1;
        let assign32430_e42899: f64 = (4.0 * locals.var_t0);
        let assign32430_e42901: f64 = (assign32430_e42899 * locals.var_sqrtpsip);
        let assign32430_e42903: f64 = (assign32430_e42901).max(1e-38);
        let assign32430_e42904: f64 = (assign32430_e42903).ln();
        let assign32430_e42905: f64 = (assign32430_e42896 - assign32430_e42904);
        (assign32430_e42905, (locals.var_t1_dn0 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn0) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn0)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn2 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn2) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn2)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn3 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn3) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn3)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn4 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn4) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn4)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn5 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn5) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn5)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn6 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn6) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn6)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn7 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn7) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn7)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn8 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn8) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn8)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn9 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn9) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn9)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn10 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn10) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn10)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn11 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn11) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn11)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn12 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn12) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn12)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn13 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn13) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn13)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn14 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn14) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn14)) } else { 0.0 } / assign32430_e42903)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32430_e42907;
        locals.var_t2_dn0 = assign32430_e42907_d_n0;
        locals.var_t2_dn2 = assign32430_e42907_d_n2;
        locals.var_t2_dn3 = assign32430_e42907_d_n3;
        locals.var_t2_dn4 = assign32430_e42907_d_n4;
        locals.var_t2_dn5 = assign32430_e42907_d_n5;
        locals.var_t2_dn6 = assign32430_e42907_d_n6;
        locals.var_t2_dn7 = assign32430_e42907_d_n7;
        locals.var_t2_dn8 = assign32430_e42907_d_n8;
        locals.var_t2_dn9 = assign32430_e42907_d_n9;
        locals.var_t2_dn10 = assign32430_e42907_d_n10;
        locals.var_t2_dn11 = assign32430_e42907_d_n11;
        locals.var_t2_dn12 = assign32430_e42907_d_n12;
        locals.var_t2_dn13 = assign32430_e42907_d_n13;
        locals.var_t2_dn14 = assign32430_e42907_d_n14;

        let (assign32440_e42924, assign32440_e42924_d_n0, assign32440_e42924_d_n2, assign32440_e42924_d_n3, assign32440_e42924_d_n4, assign32440_e42924_d_n5, assign32440_e42924_d_n6, assign32440_e42924_d_n7, assign32440_e42924_d_n8, assign32440_e42924_d_n9, assign32440_e42924_d_n10, assign32440_e42924_d_n11, assign32440_e42924_d_n12, assign32440_e42924_d_n13, assign32440_e42924_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32440_e42912: f64 = (locals.var_t2 - 0.201491);
        let assign32440_e42916: f64 = (locals.var_t2 + 0.402982);
        let assign32440_e42917: f64 = (locals.var_t2 * assign32440_e42916);
        let assign32440_e42919: f64 = (assign32440_e42917 + 2.446562);
        let assign32440_e42920: f64 = (assign32440_e42919).sqrt();
        let assign32440_e42921: f64 = (assign32440_e42912 - assign32440_e42920);
        let assign32440_e42922: f64 = (0.5 * assign32440_e42921);
        (assign32440_e42922, (0.5 * (locals.var_t2_dn0 - (((locals.var_t2_dn0 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn2 - (((locals.var_t2_dn2 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn12 - (((locals.var_t2_dn12 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn13 - (((locals.var_t2_dn13 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn14 - (((locals.var_t2_dn14 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign32440_e42920)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign32440_e42924;
        locals.var_t8_dn0 = assign32440_e42924_d_n0;
        locals.var_t8_dn2 = assign32440_e42924_d_n2;
        locals.var_t8_dn3 = assign32440_e42924_d_n3;
        locals.var_t8_dn4 = assign32440_e42924_d_n4;
        locals.var_t8_dn5 = assign32440_e42924_d_n5;
        locals.var_t8_dn6 = assign32440_e42924_d_n6;
        locals.var_t8_dn7 = assign32440_e42924_d_n7;
        locals.var_t8_dn8 = assign32440_e42924_d_n8;
        locals.var_t8_dn9 = assign32440_e42924_d_n9;
        locals.var_t8_dn10 = assign32440_e42924_d_n10;
        locals.var_t8_dn11 = assign32440_e42924_d_n11;
        locals.var_t8_dn12 = assign32440_e42924_d_n12;
        locals.var_t8_dn13 = assign32440_e42924_d_n13;
        locals.var_t8_dn14 = assign32440_e42924_d_n14;

        let (assign32450_e42928, assign32450_e42928_d_n0, assign32450_e42928_d_n2, assign32450_e42928_d_n3, assign32450_e42928_d_n4, assign32450_e42928_d_n5, assign32450_e42928_d_n6, assign32450_e42928_d_n7, assign32450_e42928_d_n8, assign32450_e42928_d_n9, assign32450_e42928_d_n10, assign32450_e42928_d_n11, assign32450_e42928_d_n12, assign32450_e42928_d_n13, assign32450_e42928_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn0, locals.var_sqrtpsisa_dn2, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11, locals.var_sqrtpsisa_dn12, locals.var_sqrtpsisa_dn13, locals.var_sqrtpsisa_dn14,)
    }
};
        locals.var_sqrtpsisa = assign32450_e42928;
        locals.var_sqrtpsisa_dn0 = assign32450_e42928_d_n0;
        locals.var_sqrtpsisa_dn2 = assign32450_e42928_d_n2;
        locals.var_sqrtpsisa_dn3 = assign32450_e42928_d_n3;
        locals.var_sqrtpsisa_dn4 = assign32450_e42928_d_n4;
        locals.var_sqrtpsisa_dn5 = assign32450_e42928_d_n5;
        locals.var_sqrtpsisa_dn6 = assign32450_e42928_d_n6;
        locals.var_sqrtpsisa_dn7 = assign32450_e42928_d_n7;
        locals.var_sqrtpsisa_dn8 = assign32450_e42928_d_n8;
        locals.var_sqrtpsisa_dn9 = assign32450_e42928_d_n9;
        locals.var_sqrtpsisa_dn10 = assign32450_e42928_d_n10;
        locals.var_sqrtpsisa_dn11 = assign32450_e42928_d_n11;
        locals.var_sqrtpsisa_dn12 = assign32450_e42928_d_n12;
        locals.var_sqrtpsisa_dn13 = assign32450_e42928_d_n13;
        locals.var_sqrtpsisa_dn14 = assign32450_e42928_d_n14;

        let assign32460_e42931: f64 = (-68.0);
        let assign32460_e42932: f64 = if locals.var_t8 <= assign32460_e42931 { 1.0 } else { 0.0 };
        locals.var_guard740 = assign32460_e42932;

        let (assign32470_e42939, assign32470_e42939_d_n0, assign32470_e42939_d_n2, assign32470_e42939_d_n3, assign32470_e42939_d_n4, assign32470_e42939_d_n5, assign32470_e42939_d_n6, assign32470_e42939_d_n7, assign32470_e42939_d_n8, assign32470_e42939_d_n9, assign32470_e42939_d_n10, assign32470_e42939_d_n11, assign32470_e42939_d_n12, assign32470_e42939_d_n13, assign32470_e42939_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) {
        let assign32470_e42937: f64 = (-100.0);
        (assign32470_e42937, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32470_e42939;
        locals.var_t4_dn0 = assign32470_e42939_d_n0;
        locals.var_t4_dn2 = assign32470_e42939_d_n2;
        locals.var_t4_dn3 = assign32470_e42939_d_n3;
        locals.var_t4_dn4 = assign32470_e42939_d_n4;
        locals.var_t4_dn5 = assign32470_e42939_d_n5;
        locals.var_t4_dn6 = assign32470_e42939_d_n6;
        locals.var_t4_dn7 = assign32470_e42939_d_n7;
        locals.var_t4_dn8 = assign32470_e42939_d_n8;
        locals.var_t4_dn9 = assign32470_e42939_d_n9;
        locals.var_t4_dn10 = assign32470_e42939_d_n10;
        locals.var_t4_dn11 = assign32470_e42939_d_n11;
        locals.var_t4_dn12 = assign32470_e42939_d_n12;
        locals.var_t4_dn13 = assign32470_e42939_d_n13;
        locals.var_t4_dn14 = assign32470_e42939_d_n14;

        let (assign32480_e42945, assign32480_e42945_d_n0, assign32480_e42945_d_n2, assign32480_e42945_d_n3, assign32480_e42945_d_n4, assign32480_e42945_d_n5, assign32480_e42945_d_n6, assign32480_e42945_d_n7, assign32480_e42945_d_n8, assign32480_e42945_d_n9, assign32480_e42945_d_n10, assign32480_e42945_d_n11, assign32480_e42945_d_n12, assign32480_e42945_d_n13, assign32480_e42945_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32480_e42945;
        locals.var_t5_dn0 = assign32480_e42945_d_n0;
        locals.var_t5_dn2 = assign32480_e42945_d_n2;
        locals.var_t5_dn3 = assign32480_e42945_d_n3;
        locals.var_t5_dn4 = assign32480_e42945_d_n4;
        locals.var_t5_dn5 = assign32480_e42945_d_n5;
        locals.var_t5_dn6 = assign32480_e42945_d_n6;
        locals.var_t5_dn7 = assign32480_e42945_d_n7;
        locals.var_t5_dn8 = assign32480_e42945_d_n8;
        locals.var_t5_dn9 = assign32480_e42945_d_n9;
        locals.var_t5_dn10 = assign32480_e42945_d_n10;
        locals.var_t5_dn11 = assign32480_e42945_d_n11;
        locals.var_t5_dn12 = assign32480_e42945_d_n12;
        locals.var_t5_dn13 = assign32480_e42945_d_n13;
        locals.var_t5_dn14 = assign32480_e42945_d_n14;

        let assign32490_e42950: f64 = (0.5 * locals.var_t5);
        let assign32490_e42951: f64 = (locals.var_t4 - assign32490_e42950);
        let assign32490_e42952: f64 = if locals.var_t8 < assign32490_e42951 { 1.0 } else { 0.0 };
        locals.var_guard741 = assign32490_e42952;

        let (assign32500_e42961, assign32500_e42961_d_n0, assign32500_e42961_d_n2, assign32500_e42961_d_n3, assign32500_e42961_d_n4, assign32500_e42961_d_n5, assign32500_e42961_d_n6, assign32500_e42961_d_n7, assign32500_e42961_d_n8, assign32500_e42961_d_n9, assign32500_e42961_d_n10, assign32500_e42961_d_n11, assign32500_e42961_d_n12, assign32500_e42961_d_n13, assign32500_e42961_d_n14,) = {
    if (((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 != 0.0)) {
        let assign32500_e42959: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32500_e42959, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn0), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn2), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn12), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn13), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32500_e42961;
        locals.var_t3_dn0 = assign32500_e42961_d_n0;
        locals.var_t3_dn2 = assign32500_e42961_d_n2;
        locals.var_t3_dn3 = assign32500_e42961_d_n3;
        locals.var_t3_dn4 = assign32500_e42961_d_n4;
        locals.var_t3_dn5 = assign32500_e42961_d_n5;
        locals.var_t3_dn6 = assign32500_e42961_d_n6;
        locals.var_t3_dn7 = assign32500_e42961_d_n7;
        locals.var_t3_dn8 = assign32500_e42961_d_n8;
        locals.var_t3_dn9 = assign32500_e42961_d_n9;
        locals.var_t3_dn10 = assign32500_e42961_d_n10;
        locals.var_t3_dn11 = assign32500_e42961_d_n11;
        locals.var_t3_dn12 = assign32500_e42961_d_n12;
        locals.var_t3_dn13 = assign32500_e42961_d_n13;
        locals.var_t3_dn14 = assign32500_e42961_d_n14;

        let assign32510_e42966: f64 = (0.5 * locals.var_t5);
        let assign32510_e42967: f64 = (locals.var_t4 + assign32510_e42966);
        let assign32510_e42968: f64 = if locals.var_t8 > assign32510_e42967 { 1.0 } else { 0.0 };
        locals.var_guard742 = assign32510_e42968;

        let (assign32520_e42980, assign32520_e42980_d_n0, assign32520_e42980_d_n2, assign32520_e42980_d_n3, assign32520_e42980_d_n4, assign32520_e42980_d_n5, assign32520_e42980_d_n6, assign32520_e42980_d_n7, assign32520_e42980_d_n8, assign32520_e42980_d_n9, assign32520_e42980_d_n10, assign32520_e42980_d_n11, assign32520_e42980_d_n12, assign32520_e42980_d_n13, assign32520_e42980_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) && (locals.var_guard742 != 0.0)) {
        let assign32520_e42978: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32520_e42978, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn12), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32520_e42980;
        locals.var_t3_dn0 = assign32520_e42980_d_n0;
        locals.var_t3_dn2 = assign32520_e42980_d_n2;
        locals.var_t3_dn3 = assign32520_e42980_d_n3;
        locals.var_t3_dn4 = assign32520_e42980_d_n4;
        locals.var_t3_dn5 = assign32520_e42980_d_n5;
        locals.var_t3_dn6 = assign32520_e42980_d_n6;
        locals.var_t3_dn7 = assign32520_e42980_d_n7;
        locals.var_t3_dn8 = assign32520_e42980_d_n8;
        locals.var_t3_dn9 = assign32520_e42980_d_n9;
        locals.var_t3_dn10 = assign32520_e42980_d_n10;
        locals.var_t3_dn11 = assign32520_e42980_d_n11;
        locals.var_t3_dn12 = assign32520_e42980_d_n12;
        locals.var_t3_dn13 = assign32520_e42980_d_n13;
        locals.var_t3_dn14 = assign32520_e42980_d_n14;

        let (assign32530_e42996, assign32530_e42996_d_n0, assign32530_e42996_d_n2, assign32530_e42996_d_n3, assign32530_e42996_d_n4, assign32530_e42996_d_n5, assign32530_e42996_d_n6, assign32530_e42996_d_n7, assign32530_e42996_d_n8, assign32530_e42996_d_n9, assign32530_e42996_d_n10, assign32530_e42996_d_n11, assign32530_e42996_d_n12, assign32530_e42996_d_n13, assign32530_e42996_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign32530_e42992: f64 = (locals.var_t8 - locals.var_t4);
        let assign32530_e42994: f64 = (assign32530_e42992 / locals.var_t5);
        (assign32530_e42994, ((((locals.var_t8_dn0 - locals.var_t4_dn0) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn2 - locals.var_t4_dn2) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn12 - locals.var_t4_dn12) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn13 - locals.var_t4_dn13) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn14 - locals.var_t4_dn14) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32530_e42996;
        locals.var_t2_dn0 = assign32530_e42996_d_n0;
        locals.var_t2_dn2 = assign32530_e42996_d_n2;
        locals.var_t2_dn3 = assign32530_e42996_d_n3;
        locals.var_t2_dn4 = assign32530_e42996_d_n4;
        locals.var_t2_dn5 = assign32530_e42996_d_n5;
        locals.var_t2_dn6 = assign32530_e42996_d_n6;
        locals.var_t2_dn7 = assign32530_e42996_d_n7;
        locals.var_t2_dn8 = assign32530_e42996_d_n8;
        locals.var_t2_dn9 = assign32530_e42996_d_n9;
        locals.var_t2_dn10 = assign32530_e42996_d_n10;
        locals.var_t2_dn11 = assign32530_e42996_d_n11;
        locals.var_t2_dn12 = assign32530_e42996_d_n12;
        locals.var_t2_dn13 = assign32530_e42996_d_n13;
        locals.var_t2_dn14 = assign32530_e42996_d_n14;

        let (assign32540_e43010, assign32540_e43010_d_n0, assign32540_e43010_d_n2, assign32540_e43010_d_n3, assign32540_e43010_d_n4, assign32540_e43010_d_n5, assign32540_e43010_d_n6, assign32540_e43010_d_n7, assign32540_e43010_d_n8, assign32540_e43010_d_n9, assign32540_e43010_d_n10, assign32540_e43010_d_n11, assign32540_e43010_d_n12, assign32540_e43010_d_n13, assign32540_e43010_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign32540_e43008: f64 = (locals.var_t2 * locals.var_t2);
        (assign32540_e43008, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)), ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)), ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32540_e43010;
        locals.var_t6_dn0 = assign32540_e43010_d_n0;
        locals.var_t6_dn2 = assign32540_e43010_d_n2;
        locals.var_t6_dn3 = assign32540_e43010_d_n3;
        locals.var_t6_dn4 = assign32540_e43010_d_n4;
        locals.var_t6_dn5 = assign32540_e43010_d_n5;
        locals.var_t6_dn6 = assign32540_e43010_d_n6;
        locals.var_t6_dn7 = assign32540_e43010_d_n7;
        locals.var_t6_dn8 = assign32540_e43010_d_n8;
        locals.var_t6_dn9 = assign32540_e43010_d_n9;
        locals.var_t6_dn10 = assign32540_e43010_d_n10;
        locals.var_t6_dn11 = assign32540_e43010_d_n11;
        locals.var_t6_dn12 = assign32540_e43010_d_n12;
        locals.var_t6_dn13 = assign32540_e43010_d_n13;
        locals.var_t6_dn14 = assign32540_e43010_d_n14;

        let (assign32550_e43045, assign32550_e43045_d_n0, assign32550_e43045_d_n2, assign32550_e43045_d_n3, assign32550_e43045_d_n4, assign32550_e43045_d_n5, assign32550_e43045_d_n6, assign32550_e43045_d_n7, assign32550_e43045_d_n8, assign32550_e43045_d_n9, assign32550_e43045_d_n10, assign32550_e43045_d_n11, assign32550_e43045_d_n12, assign32550_e43045_d_n13, assign32550_e43045_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign32550_e43024: f64 = (5.0 / 64.0);
        let assign32550_e43027: f64 = (0.5 * locals.var_t2);
        let assign32550_e43028: f64 = (assign32550_e43024 + assign32550_e43027);
        let assign32550_e43032: f64 = (15.0 / 16.0);
        let assign32550_e43036: f64 = (1.25 - locals.var_t6);
        let assign32550_e43037: f64 = (locals.var_t6 * assign32550_e43036);
        let assign32550_e43038: f64 = (assign32550_e43032 - assign32550_e43037);
        let assign32550_e43039: f64 = (locals.var_t6 * assign32550_e43038);
        let assign32550_e43040: f64 = (assign32550_e43028 + assign32550_e43039);
        let assign32550_e43041: f64 = (locals.var_t5 * assign32550_e43040);
        let assign32550_e43042: f64 = (locals.var_t4 + assign32550_e43041);
        let assign32550_e43043: f64 = { let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32550_e43043, ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn0 + ((locals.var_t5_dn0 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn0) + ((locals.var_t6_dn0 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn0 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn0))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn2 + ((locals.var_t5_dn2 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn2) + ((locals.var_t6_dn2 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn2 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn2))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn12 + ((locals.var_t5_dn12 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn12) + ((locals.var_t6_dn12 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn12 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn12))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn13 + ((locals.var_t5_dn13 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn13) + ((locals.var_t6_dn13 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn13 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn13))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn14 + ((locals.var_t5_dn14 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn14) + ((locals.var_t6_dn14 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn14 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn14))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32550_e43045;
        locals.var_t3_dn0 = assign32550_e43045_d_n0;
        locals.var_t3_dn2 = assign32550_e43045_d_n2;
        locals.var_t3_dn3 = assign32550_e43045_d_n3;
        locals.var_t3_dn4 = assign32550_e43045_d_n4;
        locals.var_t3_dn5 = assign32550_e43045_d_n5;
        locals.var_t3_dn6 = assign32550_e43045_d_n6;
        locals.var_t3_dn7 = assign32550_e43045_d_n7;
        locals.var_t3_dn8 = assign32550_e43045_d_n8;
        locals.var_t3_dn9 = assign32550_e43045_d_n9;
        locals.var_t3_dn10 = assign32550_e43045_d_n10;
        locals.var_t3_dn11 = assign32550_e43045_d_n11;
        locals.var_t3_dn12 = assign32550_e43045_d_n12;
        locals.var_t3_dn13 = assign32550_e43045_d_n13;
        locals.var_t3_dn14 = assign32550_e43045_d_n14;

        let (assign32560_e43078, assign32560_e43078_d_n0, assign32560_e43078_d_n2, assign32560_e43078_d_n3, assign32560_e43078_d_n4, assign32560_e43078_d_n5, assign32560_e43078_d_n6, assign32560_e43078_d_n7, assign32560_e43078_d_n8, assign32560_e43078_d_n9, assign32560_e43078_d_n10, assign32560_e43078_d_n11, assign32560_e43078_d_n12, assign32560_e43078_d_n13, assign32560_e43078_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) {
        let assign32560_e43052: f64 = (1.0 + locals.var_t1);
        let assign32560_e43055: f64 = locals.var_t8;
        let assign32560_e43056: f64 = (assign32560_e43052 - assign32560_e43055);
        let assign32560_e43060: f64 = (2.0 * locals.var_t0);
        let assign32560_e43063: f64 = (locals.var_t3 * 2.0);
        let assign32560_e43065: f64 = (assign32560_e43063 * locals.var_t0);
        let assign32560_e43068: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32560_e43069: f64 = (assign32560_e43065 + assign32560_e43068);
        let assign32560_e43070: f64 = (assign32560_e43060 * assign32560_e43069);
        let assign32560_e43072: f64 = (assign32560_e43070).max(1e-38);
        let assign32560_e43073: f64 = (assign32560_e43072).ln();
        let assign32560_e43074: f64 = assign32560_e43073;
        let assign32560_e43075: f64 = (assign32560_e43056 - assign32560_e43074);
        let assign32560_e43076: f64 = (locals.var_t3 * assign32560_e43075);
        (assign32560_e43076, ((locals.var_t3_dn0 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn0 - locals.var_t8_dn0) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn0) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn2 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn2 - locals.var_t8_dn2) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn2) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn3 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn4 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn5 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn6 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn7 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn8 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn9 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn10 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn11 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn12 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn12 - locals.var_t8_dn12) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn12) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn13 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn13 - locals.var_t8_dn13) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn13) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn14 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn14 - locals.var_t8_dn14) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn14) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32560_e43072)))),)
    } else {
        (locals.var_qdeff_edge, locals.var_qdeff_edge_dn0, locals.var_qdeff_edge_dn2, locals.var_qdeff_edge_dn3, locals.var_qdeff_edge_dn4, locals.var_qdeff_edge_dn5, locals.var_qdeff_edge_dn6, locals.var_qdeff_edge_dn7, locals.var_qdeff_edge_dn8, locals.var_qdeff_edge_dn9, locals.var_qdeff_edge_dn10, locals.var_qdeff_edge_dn11, locals.var_qdeff_edge_dn12, locals.var_qdeff_edge_dn13, locals.var_qdeff_edge_dn14,)
    }
};
        locals.var_qdeff_edge = assign32560_e43078;
        locals.var_qdeff_edge_dn0 = assign32560_e43078_d_n0;
        locals.var_qdeff_edge_dn2 = assign32560_e43078_d_n2;
        locals.var_qdeff_edge_dn3 = assign32560_e43078_d_n3;
        locals.var_qdeff_edge_dn4 = assign32560_e43078_d_n4;
        locals.var_qdeff_edge_dn5 = assign32560_e43078_d_n5;
        locals.var_qdeff_edge_dn6 = assign32560_e43078_d_n6;
        locals.var_qdeff_edge_dn7 = assign32560_e43078_d_n7;
        locals.var_qdeff_edge_dn8 = assign32560_e43078_d_n8;
        locals.var_qdeff_edge_dn9 = assign32560_e43078_d_n9;
        locals.var_qdeff_edge_dn10 = assign32560_e43078_d_n10;
        locals.var_qdeff_edge_dn11 = assign32560_e43078_d_n11;
        locals.var_qdeff_edge_dn12 = assign32560_e43078_d_n12;
        locals.var_qdeff_edge_dn13 = assign32560_e43078_d_n13;
        locals.var_qdeff_edge_dn14 = assign32560_e43078_d_n14;

        let (assign32570_e43086, assign32570_e43086_d_n0, assign32570_e43086_d_n2, assign32570_e43086_d_n3, assign32570_e43086_d_n4, assign32570_e43086_d_n5, assign32570_e43086_d_n6, assign32570_e43086_d_n7, assign32570_e43086_d_n8, assign32570_e43086_d_n9, assign32570_e43086_d_n10, assign32570_e43086_d_n11, assign32570_e43086_d_n12, assign32570_e43086_d_n13, assign32570_e43086_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32570_e43084: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32570_e43084, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn12), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32570_e43086;
        locals.var_t3_dn0 = assign32570_e43086_d_n0;
        locals.var_t3_dn2 = assign32570_e43086_d_n2;
        locals.var_t3_dn3 = assign32570_e43086_d_n3;
        locals.var_t3_dn4 = assign32570_e43086_d_n4;
        locals.var_t3_dn5 = assign32570_e43086_d_n5;
        locals.var_t3_dn6 = assign32570_e43086_d_n6;
        locals.var_t3_dn7 = assign32570_e43086_d_n7;
        locals.var_t3_dn8 = assign32570_e43086_d_n8;
        locals.var_t3_dn9 = assign32570_e43086_d_n9;
        locals.var_t3_dn10 = assign32570_e43086_d_n10;
        locals.var_t3_dn11 = assign32570_e43086_d_n11;
        locals.var_t3_dn12 = assign32570_e43086_d_n12;
        locals.var_t3_dn13 = assign32570_e43086_d_n13;
        locals.var_t3_dn14 = assign32570_e43086_d_n14;

        let (assign32580_e43095, assign32580_e43095_d_n0, assign32580_e43095_d_n2, assign32580_e43095_d_n3, assign32580_e43095_d_n4, assign32580_e43095_d_n5, assign32580_e43095_d_n6, assign32580_e43095_d_n7, assign32580_e43095_d_n8, assign32580_e43095_d_n9, assign32580_e43095_d_n10, assign32580_e43095_d_n11, assign32580_e43095_d_n12, assign32580_e43095_d_n13, assign32580_e43095_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32580_e43093: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign32580_e43093, (-(locals.var_sqrtpsisa_dn0 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn2 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn12 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn13 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn14 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn0, locals.var_sqrtpsisainv_dn2, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11, locals.var_sqrtpsisainv_dn12, locals.var_sqrtpsisainv_dn13, locals.var_sqrtpsisainv_dn14,)
    }
};
        locals.var_sqrtpsisainv = assign32580_e43095;
        locals.var_sqrtpsisainv_dn0 = assign32580_e43095_d_n0;
        locals.var_sqrtpsisainv_dn2 = assign32580_e43095_d_n2;
        locals.var_sqrtpsisainv_dn3 = assign32580_e43095_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign32580_e43095_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign32580_e43095_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign32580_e43095_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign32580_e43095_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign32580_e43095_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign32580_e43095_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign32580_e43095_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign32580_e43095_d_n11;
        locals.var_sqrtpsisainv_dn12 = assign32580_e43095_d_n12;
        locals.var_sqrtpsisainv_dn13 = assign32580_e43095_d_n13;
        locals.var_sqrtpsisainv_dn14 = assign32580_e43095_d_n14;

        let (assign32590_e43127, assign32590_e43127_d_n0, assign32590_e43127_d_n2, assign32590_e43127_d_n3, assign32590_e43127_d_n4, assign32590_e43127_d_n5, assign32590_e43127_d_n6, assign32590_e43127_d_n7, assign32590_e43127_d_n8, assign32590_e43127_d_n9, assign32590_e43127_d_n10, assign32590_e43127_d_n11, assign32590_e43127_d_n12, assign32590_e43127_d_n13, assign32590_e43127_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32590_e43102: f64 = (2.0 * locals.var_t3);
        let assign32590_e43106: f64 = (locals.var_t3 * 2.0);
        let assign32590_e43108: f64 = (assign32590_e43106 * locals.var_t0);
        let assign32590_e43111: f64 = (locals.var_t3 * 2.0);
        let assign32590_e43113: f64 = (assign32590_e43111 * locals.var_t0);
        let assign32590_e43116: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32590_e43117: f64 = (assign32590_e43113 + assign32590_e43116);
        let assign32590_e43118: f64 = (assign32590_e43108 * assign32590_e43117);
        let assign32590_e43120: f64 = (assign32590_e43118).max(1e-38);
        let assign32590_e43121: f64 = (assign32590_e43120).ln();
        let assign32590_e43122: f64 = assign32590_e43121;
        let assign32590_e43123: f64 = (assign32590_e43102 + assign32590_e43122);
        let assign32590_e43125: f64 = (assign32590_e43123 - locals.var_t1);
        (assign32590_e43125, (((2.0 * locals.var_t3_dn0) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn0)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn0), (((2.0 * locals.var_t3_dn2) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn2)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn2), (((2.0 * locals.var_t3_dn3) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn3)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn4)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn5)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn6)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn7)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn8)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn9)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn10)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn11)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn11), (((2.0 * locals.var_t3_dn12) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn12)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn12), (((2.0 * locals.var_t3_dn13) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn13)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn13), (((2.0 * locals.var_t3_dn14) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn14)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32590_e43127;
        locals.var_t4_dn0 = assign32590_e43127_d_n0;
        locals.var_t4_dn2 = assign32590_e43127_d_n2;
        locals.var_t4_dn3 = assign32590_e43127_d_n3;
        locals.var_t4_dn4 = assign32590_e43127_d_n4;
        locals.var_t4_dn5 = assign32590_e43127_d_n5;
        locals.var_t4_dn6 = assign32590_e43127_d_n6;
        locals.var_t4_dn7 = assign32590_e43127_d_n7;
        locals.var_t4_dn8 = assign32590_e43127_d_n8;
        locals.var_t4_dn9 = assign32590_e43127_d_n9;
        locals.var_t4_dn10 = assign32590_e43127_d_n10;
        locals.var_t4_dn11 = assign32590_e43127_d_n11;
        locals.var_t4_dn12 = assign32590_e43127_d_n12;
        locals.var_t4_dn13 = assign32590_e43127_d_n13;
        locals.var_t4_dn14 = assign32590_e43127_d_n14;

        let (assign32600_e43152, assign32600_e43152_d_n0, assign32600_e43152_d_n2, assign32600_e43152_d_n3, assign32600_e43152_d_n4, assign32600_e43152_d_n5, assign32600_e43152_d_n6, assign32600_e43152_d_n7, assign32600_e43152_d_n8, assign32600_e43152_d_n9, assign32600_e43152_d_n10, assign32600_e43152_d_n11, assign32600_e43152_d_n12, assign32600_e43152_d_n13, assign32600_e43152_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32600_e43135: f64 = 1.0;
        let assign32600_e43137: f64 = (assign32600_e43135 / locals.var_t3);
        let assign32600_e43138: f64 = (2.0 + assign32600_e43137);
        let assign32600_e43142: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32600_e43143: f64 = assign32600_e43142;
        let assign32600_e43146: f64 = (locals.var_t0 * locals.var_t3);
        let assign32600_e43148: f64 = (assign32600_e43146 + locals.var_sqrtpsisa);
        let assign32600_e43149: f64 = (assign32600_e43143 / assign32600_e43148);
        let assign32600_e43150: f64 = (assign32600_e43138 + assign32600_e43149);
        (assign32600_e43150, ((-((assign32600_e43135 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32600_e43148 * assign32600_e43148))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32600_e43152;
        locals.var_t5_dn0 = assign32600_e43152_d_n0;
        locals.var_t5_dn2 = assign32600_e43152_d_n2;
        locals.var_t5_dn3 = assign32600_e43152_d_n3;
        locals.var_t5_dn4 = assign32600_e43152_d_n4;
        locals.var_t5_dn5 = assign32600_e43152_d_n5;
        locals.var_t5_dn6 = assign32600_e43152_d_n6;
        locals.var_t5_dn7 = assign32600_e43152_d_n7;
        locals.var_t5_dn8 = assign32600_e43152_d_n8;
        locals.var_t5_dn9 = assign32600_e43152_d_n9;
        locals.var_t5_dn10 = assign32600_e43152_d_n10;
        locals.var_t5_dn11 = assign32600_e43152_d_n11;
        locals.var_t5_dn12 = assign32600_e43152_d_n12;
        locals.var_t5_dn13 = assign32600_e43152_d_n13;
        locals.var_t5_dn14 = assign32600_e43152_d_n14;

        let (assign32610_e43163, assign32610_e43163_d_n0, assign32610_e43163_d_n2, assign32610_e43163_d_n3, assign32610_e43163_d_n4, assign32610_e43163_d_n5, assign32610_e43163_d_n6, assign32610_e43163_d_n7, assign32610_e43163_d_n8, assign32610_e43163_d_n9, assign32610_e43163_d_n10, assign32610_e43163_d_n11, assign32610_e43163_d_n12, assign32610_e43163_d_n13, assign32610_e43163_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32610_e43160: f64 = (locals.var_t4 / locals.var_t5);
        let assign32610_e43161: f64 = (locals.var_t3 - assign32610_e43160);
        (assign32610_e43161, (locals.var_t3_dn0 - (((locals.var_t4_dn0 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn2 - (((locals.var_t4_dn2 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn12 - (((locals.var_t4_dn12 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn13 - (((locals.var_t4_dn13 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn14 - (((locals.var_t4_dn14 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32610_e43163;
        locals.var_t3_dn0 = assign32610_e43163_d_n0;
        locals.var_t3_dn2 = assign32610_e43163_d_n2;
        locals.var_t3_dn3 = assign32610_e43163_d_n3;
        locals.var_t3_dn4 = assign32610_e43163_d_n4;
        locals.var_t3_dn5 = assign32610_e43163_d_n5;
        locals.var_t3_dn6 = assign32610_e43163_d_n6;
        locals.var_t3_dn7 = assign32610_e43163_d_n7;
        locals.var_t3_dn8 = assign32610_e43163_d_n8;
        locals.var_t3_dn9 = assign32610_e43163_d_n9;
        locals.var_t3_dn10 = assign32610_e43163_d_n10;
        locals.var_t3_dn11 = assign32610_e43163_d_n11;
        locals.var_t3_dn12 = assign32610_e43163_d_n12;
        locals.var_t3_dn13 = assign32610_e43163_d_n13;
        locals.var_t3_dn14 = assign32610_e43163_d_n14;

    }

    pub(super) fn stamp_transient_block_100(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32620_e43195, assign32620_e43195_d_n0, assign32620_e43195_d_n2, assign32620_e43195_d_n3, assign32620_e43195_d_n4, assign32620_e43195_d_n5, assign32620_e43195_d_n6, assign32620_e43195_d_n7, assign32620_e43195_d_n8, assign32620_e43195_d_n9, assign32620_e43195_d_n10, assign32620_e43195_d_n11, assign32620_e43195_d_n12, assign32620_e43195_d_n13, assign32620_e43195_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32620_e43170: f64 = (2.0 * locals.var_t3);
        let assign32620_e43174: f64 = (locals.var_t3 * 2.0);
        let assign32620_e43176: f64 = (assign32620_e43174 * locals.var_t0);
        let assign32620_e43179: f64 = (locals.var_t3 * 2.0);
        let assign32620_e43181: f64 = (assign32620_e43179 * locals.var_t0);
        let assign32620_e43184: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32620_e43185: f64 = (assign32620_e43181 + assign32620_e43184);
        let assign32620_e43186: f64 = (assign32620_e43176 * assign32620_e43185);
        let assign32620_e43188: f64 = (assign32620_e43186).max(1e-38);
        let assign32620_e43189: f64 = (assign32620_e43188).ln();
        let assign32620_e43190: f64 = assign32620_e43189;
        let assign32620_e43191: f64 = (assign32620_e43170 + assign32620_e43190);
        let assign32620_e43193: f64 = (assign32620_e43191 - locals.var_t1);
        (assign32620_e43193, (((2.0 * locals.var_t3_dn0) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn0)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn0), (((2.0 * locals.var_t3_dn2) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn2)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn2), (((2.0 * locals.var_t3_dn3) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn3)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn4)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn5)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn6)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn7)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn8)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn9)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn10)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn11)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn11), (((2.0 * locals.var_t3_dn12) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn12)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn12), (((2.0 * locals.var_t3_dn13) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn13)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn13), (((2.0 * locals.var_t3_dn14) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn14)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32620_e43195;
        locals.var_t4_dn0 = assign32620_e43195_d_n0;
        locals.var_t4_dn2 = assign32620_e43195_d_n2;
        locals.var_t4_dn3 = assign32620_e43195_d_n3;
        locals.var_t4_dn4 = assign32620_e43195_d_n4;
        locals.var_t4_dn5 = assign32620_e43195_d_n5;
        locals.var_t4_dn6 = assign32620_e43195_d_n6;
        locals.var_t4_dn7 = assign32620_e43195_d_n7;
        locals.var_t4_dn8 = assign32620_e43195_d_n8;
        locals.var_t4_dn9 = assign32620_e43195_d_n9;
        locals.var_t4_dn10 = assign32620_e43195_d_n10;
        locals.var_t4_dn11 = assign32620_e43195_d_n11;
        locals.var_t4_dn12 = assign32620_e43195_d_n12;
        locals.var_t4_dn13 = assign32620_e43195_d_n13;
        locals.var_t4_dn14 = assign32620_e43195_d_n14;

        let (assign32630_e43220, assign32630_e43220_d_n0, assign32630_e43220_d_n2, assign32630_e43220_d_n3, assign32630_e43220_d_n4, assign32630_e43220_d_n5, assign32630_e43220_d_n6, assign32630_e43220_d_n7, assign32630_e43220_d_n8, assign32630_e43220_d_n9, assign32630_e43220_d_n10, assign32630_e43220_d_n11, assign32630_e43220_d_n12, assign32630_e43220_d_n13, assign32630_e43220_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32630_e43203: f64 = 1.0;
        let assign32630_e43205: f64 = (assign32630_e43203 / locals.var_t3);
        let assign32630_e43206: f64 = (2.0 + assign32630_e43205);
        let assign32630_e43210: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32630_e43211: f64 = assign32630_e43210;
        let assign32630_e43214: f64 = (locals.var_t0 * locals.var_t3);
        let assign32630_e43216: f64 = (assign32630_e43214 + locals.var_sqrtpsisa);
        let assign32630_e43217: f64 = (assign32630_e43211 / assign32630_e43216);
        let assign32630_e43218: f64 = (assign32630_e43206 + assign32630_e43217);
        (assign32630_e43218, ((-((assign32630_e43203 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32630_e43216 * assign32630_e43216))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32630_e43220;
        locals.var_t5_dn0 = assign32630_e43220_d_n0;
        locals.var_t5_dn2 = assign32630_e43220_d_n2;
        locals.var_t5_dn3 = assign32630_e43220_d_n3;
        locals.var_t5_dn4 = assign32630_e43220_d_n4;
        locals.var_t5_dn5 = assign32630_e43220_d_n5;
        locals.var_t5_dn6 = assign32630_e43220_d_n6;
        locals.var_t5_dn7 = assign32630_e43220_d_n7;
        locals.var_t5_dn8 = assign32630_e43220_d_n8;
        locals.var_t5_dn9 = assign32630_e43220_d_n9;
        locals.var_t5_dn10 = assign32630_e43220_d_n10;
        locals.var_t5_dn11 = assign32630_e43220_d_n11;
        locals.var_t5_dn12 = assign32630_e43220_d_n12;
        locals.var_t5_dn13 = assign32630_e43220_d_n13;
        locals.var_t5_dn14 = assign32630_e43220_d_n14;

        let (assign32640_e43247, assign32640_e43247_d_n0, assign32640_e43247_d_n2, assign32640_e43247_d_n3, assign32640_e43247_d_n4, assign32640_e43247_d_n5, assign32640_e43247_d_n6, assign32640_e43247_d_n7, assign32640_e43247_d_n8, assign32640_e43247_d_n9, assign32640_e43247_d_n10, assign32640_e43247_d_n11, assign32640_e43247_d_n12, assign32640_e43247_d_n13, assign32640_e43247_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32640_e43228: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32640_e43231: f64 = (locals.var_t0 * locals.var_t3);
        let assign32640_e43233: f64 = (assign32640_e43231 + locals.var_sqrtpsisa);
        let assign32640_e43234: f64 = (assign32640_e43228 / assign32640_e43233);
        let assign32640_e43235: f64 = assign32640_e43234;
        let assign32640_e43238: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32640_e43241: f64 = (locals.var_t0 * locals.var_t3);
        let assign32640_e43243: f64 = (assign32640_e43241 + locals.var_sqrtpsisa);
        let assign32640_e43244: f64 = (assign32640_e43238 / assign32640_e43243);
        let assign32640_e43245: f64 = (assign32640_e43235 * assign32640_e43244);
        (assign32640_e43245, ((((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32640_e43243 * assign32640_e43243)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32640_e43247;
        locals.var_t6_dn0 = assign32640_e43247_d_n0;
        locals.var_t6_dn2 = assign32640_e43247_d_n2;
        locals.var_t6_dn3 = assign32640_e43247_d_n3;
        locals.var_t6_dn4 = assign32640_e43247_d_n4;
        locals.var_t6_dn5 = assign32640_e43247_d_n5;
        locals.var_t6_dn6 = assign32640_e43247_d_n6;
        locals.var_t6_dn7 = assign32640_e43247_d_n7;
        locals.var_t6_dn8 = assign32640_e43247_d_n8;
        locals.var_t6_dn9 = assign32640_e43247_d_n9;
        locals.var_t6_dn10 = assign32640_e43247_d_n10;
        locals.var_t6_dn11 = assign32640_e43247_d_n11;
        locals.var_t6_dn12 = assign32640_e43247_d_n12;
        locals.var_t6_dn13 = assign32640_e43247_d_n13;
        locals.var_t6_dn14 = assign32640_e43247_d_n14;

        let (assign32650_e43281, assign32650_e43281_d_n0, assign32650_e43281_d_n2, assign32650_e43281_d_n3, assign32650_e43281_d_n4, assign32650_e43281_d_n5, assign32650_e43281_d_n6, assign32650_e43281_d_n7, assign32650_e43281_d_n8, assign32650_e43281_d_n9, assign32650_e43281_d_n10, assign32650_e43281_d_n11, assign32650_e43281_d_n12, assign32650_e43281_d_n13, assign32650_e43281_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32650_e43253: f64 = (-1.0);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign32650_e43256: f64 = (1.0 * __rspice_inv_cse_0);
        let assign32650_e43259: f64 = (1.0 * __rspice_inv_cse_0);
        let assign32650_e43260: f64 = (assign32650_e43256 * assign32650_e43259);
        let assign32650_e43261: f64 = (assign32650_e43253 * assign32650_e43260);
        let assign32650_e43264: f64 = 1.0;
        let assign32650_e43267: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign32650_e43269: f64 = (assign32650_e43267 * locals.var_sqrtpsisa);
        let assign32650_e43272: f64 = (locals.var_t0 * locals.var_t3);
        let assign32650_e43274: f64 = (assign32650_e43272 + locals.var_sqrtpsisa);
        let assign32650_e43275: f64 = (assign32650_e43269 * assign32650_e43274);
        let assign32650_e43276: f64 = (assign32650_e43264 / assign32650_e43275);
        let assign32650_e43277: f64 = (assign32650_e43261 - assign32650_e43276);
        let assign32650_e43279: f64 = (assign32650_e43277 - locals.var_t6);
        (assign32650_e43279, (((assign32650_e43253 * (((-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn0 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn0)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn0)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn0), (((assign32650_e43253 * (((-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn2 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn2)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn2)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn2), (((assign32650_e43253 * (((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn3)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn3), (((assign32650_e43253 * (((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn4)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn4), (((assign32650_e43253 * (((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn5)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn5), (((assign32650_e43253 * (((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn6)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn6), (((assign32650_e43253 * (((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn7)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn7), (((assign32650_e43253 * (((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn8)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn8), (((assign32650_e43253 * (((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn9)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn9), (((assign32650_e43253 * (((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn10)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn10), (((assign32650_e43253 * (((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn11)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn11), (((assign32650_e43253 * (((-(locals.var_t3_dn12 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn12 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn12 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn12)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn12)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn12), (((assign32650_e43253 * (((-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn13 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn13)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn13)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn13), (((assign32650_e43253 * (((-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn14 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn14)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn14)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign32650_e43281;
        locals.var_t7_dn0 = assign32650_e43281_d_n0;
        locals.var_t7_dn2 = assign32650_e43281_d_n2;
        locals.var_t7_dn3 = assign32650_e43281_d_n3;
        locals.var_t7_dn4 = assign32650_e43281_d_n4;
        locals.var_t7_dn5 = assign32650_e43281_d_n5;
        locals.var_t7_dn6 = assign32650_e43281_d_n6;
        locals.var_t7_dn7 = assign32650_e43281_d_n7;
        locals.var_t7_dn8 = assign32650_e43281_d_n8;
        locals.var_t7_dn9 = assign32650_e43281_d_n9;
        locals.var_t7_dn10 = assign32650_e43281_d_n10;
        locals.var_t7_dn11 = assign32650_e43281_d_n11;
        locals.var_t7_dn12 = assign32650_e43281_d_n12;
        locals.var_t7_dn13 = assign32650_e43281_d_n13;
        locals.var_t7_dn14 = assign32650_e43281_d_n14;

        let (assign32660_e43304, assign32660_e43304_d_n0, assign32660_e43304_d_n2, assign32660_e43304_d_n3, assign32660_e43304_d_n4, assign32660_e43304_d_n5, assign32660_e43304_d_n6, assign32660_e43304_d_n7, assign32660_e43304_d_n8, assign32660_e43304_d_n9, assign32660_e43304_d_n10, assign32660_e43304_d_n11, assign32660_e43304_d_n12, assign32660_e43304_d_n13, assign32660_e43304_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32660_e43289: f64 = (locals.var_t4 / locals.var_t5);
        let assign32660_e43293: f64 = (locals.var_t4 * locals.var_t7);
        let assign32660_e43296: f64 = (2.0 * locals.var_t5);
        let assign32660_e43298: f64 = (assign32660_e43296 * locals.var_t5);
        let assign32660_e43299: f64 = (assign32660_e43293 / assign32660_e43298);
        let assign32660_e43300: f64 = (1.0 + assign32660_e43299);
        let assign32660_e43301: f64 = (assign32660_e43289 * assign32660_e43300);
        let assign32660_e43302: f64 = (locals.var_t3 - assign32660_e43301);
        (assign32660_e43302, (locals.var_t3_dn0 - (((((locals.var_t4_dn0 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn0 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn0)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn0) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn0)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn2 - (((((locals.var_t4_dn2 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn2 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn2)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn2) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn2)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn3)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn4)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn5)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn6)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn7)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn8)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn9)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn10)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn11)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn12 - (((((locals.var_t4_dn12 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn12 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn12)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn12) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn12)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn13 - (((((locals.var_t4_dn13 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn13 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn13)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn13) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn13)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn14 - (((((locals.var_t4_dn14 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn14 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn14)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn14) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn14)))) / (assign32660_e43298 * assign32660_e43298))))),)
    } else {
        (locals.var_qdeff_edge, locals.var_qdeff_edge_dn0, locals.var_qdeff_edge_dn2, locals.var_qdeff_edge_dn3, locals.var_qdeff_edge_dn4, locals.var_qdeff_edge_dn5, locals.var_qdeff_edge_dn6, locals.var_qdeff_edge_dn7, locals.var_qdeff_edge_dn8, locals.var_qdeff_edge_dn9, locals.var_qdeff_edge_dn10, locals.var_qdeff_edge_dn11, locals.var_qdeff_edge_dn12, locals.var_qdeff_edge_dn13, locals.var_qdeff_edge_dn14,)
    }
};
        locals.var_qdeff_edge = assign32660_e43304;
        locals.var_qdeff_edge_dn0 = assign32660_e43304_d_n0;
        locals.var_qdeff_edge_dn2 = assign32660_e43304_d_n2;
        locals.var_qdeff_edge_dn3 = assign32660_e43304_d_n3;
        locals.var_qdeff_edge_dn4 = assign32660_e43304_d_n4;
        locals.var_qdeff_edge_dn5 = assign32660_e43304_d_n5;
        locals.var_qdeff_edge_dn6 = assign32660_e43304_d_n6;
        locals.var_qdeff_edge_dn7 = assign32660_e43304_d_n7;
        locals.var_qdeff_edge_dn8 = assign32660_e43304_d_n8;
        locals.var_qdeff_edge_dn9 = assign32660_e43304_d_n9;
        locals.var_qdeff_edge_dn10 = assign32660_e43304_d_n10;
        locals.var_qdeff_edge_dn11 = assign32660_e43304_d_n11;
        locals.var_qdeff_edge_dn12 = assign32660_e43304_d_n12;
        locals.var_qdeff_edge_dn13 = assign32660_e43304_d_n13;
        locals.var_qdeff_edge_dn14 = assign32660_e43304_d_n14;

        let assign32670_e43310: f64 = (-2500.0);
        let assign32670_e43312: f64 = (assign32670_e43310 * 2.0);
        let assign32670_e43314: f64 = if ((1.0 == 0.0) && (locals.var_psip < assign32670_e43312)) { 1.0 } else { 0.0 };
        locals.var_guard743 = assign32670_e43314;

        let (assign32680_e43327, assign32680_e43327_d_n0, assign32680_e43327_d_n2, assign32680_e43327_d_n3, assign32680_e43327_d_n4, assign32680_e43327_d_n5, assign32680_e43327_d_n6, assign32680_e43327_d_n7, assign32680_e43327_d_n8, assign32680_e43327_d_n9, assign32680_e43327_d_n10, assign32680_e43327_d_n11, assign32680_e43327_d_n12, assign32680_e43327_d_n13, assign32680_e43327_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard743 != 0.0)) {
        let assign32680_e43319: f64 = (-2.0);
        let assign32680_e43321: f64 = (assign32680_e43319 * 2.0);
        let assign32680_e43324: f64 = (16.0 * locals.var_psip);
        let assign32680_e43325: f64 = (assign32680_e43321 / assign32680_e43324);
        (assign32680_e43325, (-((assign32680_e43321 * (16.0 * locals.var_psip_dn0)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn2)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn3)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn4)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn5)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn6)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn7)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn8)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn9)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn10)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn11)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn12)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn13)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn14)) / (assign32680_e43324 * assign32680_e43324))),)
    } else {
        (locals.var_psipclamp, locals.var_psipclamp_dn0, locals.var_psipclamp_dn2, locals.var_psipclamp_dn3, locals.var_psipclamp_dn4, locals.var_psipclamp_dn5, locals.var_psipclamp_dn6, locals.var_psipclamp_dn7, locals.var_psipclamp_dn8, locals.var_psipclamp_dn9, locals.var_psipclamp_dn10, locals.var_psipclamp_dn11, locals.var_psipclamp_dn12, locals.var_psipclamp_dn13, locals.var_psipclamp_dn14,)
    }
};
        locals.var_psipclamp = assign32680_e43327;
        locals.var_psipclamp_dn0 = assign32680_e43327_d_n0;
        locals.var_psipclamp_dn2 = assign32680_e43327_d_n2;
        locals.var_psipclamp_dn3 = assign32680_e43327_d_n3;
        locals.var_psipclamp_dn4 = assign32680_e43327_d_n4;
        locals.var_psipclamp_dn5 = assign32680_e43327_d_n5;
        locals.var_psipclamp_dn6 = assign32680_e43327_d_n6;
        locals.var_psipclamp_dn7 = assign32680_e43327_d_n7;
        locals.var_psipclamp_dn8 = assign32680_e43327_d_n8;
        locals.var_psipclamp_dn9 = assign32680_e43327_d_n9;
        locals.var_psipclamp_dn10 = assign32680_e43327_d_n10;
        locals.var_psipclamp_dn11 = assign32680_e43327_d_n11;
        locals.var_psipclamp_dn12 = assign32680_e43327_d_n12;
        locals.var_psipclamp_dn13 = assign32680_e43327_d_n13;
        locals.var_psipclamp_dn14 = assign32680_e43327_d_n14;

        let (assign32690_e43353, assign32690_e43353_d_n0, assign32690_e43353_d_n2, assign32690_e43353_d_n3, assign32690_e43353_d_n4, assign32690_e43353_d_n5, assign32690_e43353_d_n6, assign32690_e43353_d_n7, assign32690_e43353_d_n8, assign32690_e43353_d_n9, assign32690_e43353_d_n10, assign32690_e43353_d_n11, assign32690_e43353_d_n12, assign32690_e43353_d_n13, assign32690_e43353_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard743 == 0.0)) {
        let assign32690_e43335: f64 = (locals.var_psip + 1.0);
        let assign32690_e43338: f64 = (locals.var_psip - 1.0);
        let assign32690_e43341: f64 = (locals.var_psip - 1.0);
        let assign32690_e43342: f64 = (assign32690_e43338 * assign32690_e43341);
        let assign32690_e43345: f64 = (0.25 * 2.0);
        let assign32690_e43347: f64 = (assign32690_e43345 * 2.0);
        let assign32690_e43348: f64 = (assign32690_e43342 + assign32690_e43347);
        let assign32690_e43349: f64 = (assign32690_e43348).sqrt();
        let assign32690_e43350: f64 = (assign32690_e43335 + assign32690_e43349);
        let assign32690_e43351: f64 = (0.5 * assign32690_e43350);
        (assign32690_e43351, (0.5 * (locals.var_psip_dn0 + (((locals.var_psip_dn0 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn0)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn2 + (((locals.var_psip_dn2 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn2)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn3)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn4)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn5)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn6)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn7)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn8)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn9)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn10)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn11)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn12 + (((locals.var_psip_dn12 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn12)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn13 + (((locals.var_psip_dn13 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn13)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn14 + (((locals.var_psip_dn14 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn14)) / (2.0 * assign32690_e43349)))),)
    } else {
        (locals.var_psipclamp, locals.var_psipclamp_dn0, locals.var_psipclamp_dn2, locals.var_psipclamp_dn3, locals.var_psipclamp_dn4, locals.var_psipclamp_dn5, locals.var_psipclamp_dn6, locals.var_psipclamp_dn7, locals.var_psipclamp_dn8, locals.var_psipclamp_dn9, locals.var_psipclamp_dn10, locals.var_psipclamp_dn11, locals.var_psipclamp_dn12, locals.var_psipclamp_dn13, locals.var_psipclamp_dn14,)
    }
};
        locals.var_psipclamp = assign32690_e43353;
        locals.var_psipclamp_dn0 = assign32690_e43353_d_n0;
        locals.var_psipclamp_dn2 = assign32690_e43353_d_n2;
        locals.var_psipclamp_dn3 = assign32690_e43353_d_n3;
        locals.var_psipclamp_dn4 = assign32690_e43353_d_n4;
        locals.var_psipclamp_dn5 = assign32690_e43353_d_n5;
        locals.var_psipclamp_dn6 = assign32690_e43353_d_n6;
        locals.var_psipclamp_dn7 = assign32690_e43353_d_n7;
        locals.var_psipclamp_dn8 = assign32690_e43353_d_n8;
        locals.var_psipclamp_dn9 = assign32690_e43353_d_n9;
        locals.var_psipclamp_dn10 = assign32690_e43353_d_n10;
        locals.var_psipclamp_dn11 = assign32690_e43353_d_n11;
        locals.var_psipclamp_dn12 = assign32690_e43353_d_n12;
        locals.var_psipclamp_dn13 = assign32690_e43353_d_n13;
        locals.var_psipclamp_dn14 = assign32690_e43353_d_n14;

        let (assign32700_e43358, assign32700_e43358_d_n0, assign32700_e43358_d_n2, assign32700_e43358_d_n3, assign32700_e43358_d_n4, assign32700_e43358_d_n5, assign32700_e43358_d_n6, assign32700_e43358_d_n7, assign32700_e43358_d_n8, assign32700_e43358_d_n9, assign32700_e43358_d_n10, assign32700_e43358_d_n11, assign32700_e43358_d_n12, assign32700_e43358_d_n13, assign32700_e43358_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32700_e43356: f64 = (locals.var_psipclamp).sqrt();
        (assign32700_e43356, (locals.var_psipclamp_dn0 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn2 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn3 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn4 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn5 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn6 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn7 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn8 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn9 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn10 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn11 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn12 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn13 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn14 / (2.0 * assign32700_e43356)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    }
};
        locals.var_sqrtpsip = assign32700_e43358;
        locals.var_sqrtpsip_dn0 = assign32700_e43358_d_n0;
        locals.var_sqrtpsip_dn2 = assign32700_e43358_d_n2;
        locals.var_sqrtpsip_dn3 = assign32700_e43358_d_n3;
        locals.var_sqrtpsip_dn4 = assign32700_e43358_d_n4;
        locals.var_sqrtpsip_dn5 = assign32700_e43358_d_n5;
        locals.var_sqrtpsip_dn6 = assign32700_e43358_d_n6;
        locals.var_sqrtpsip_dn7 = assign32700_e43358_d_n7;
        locals.var_sqrtpsip_dn8 = assign32700_e43358_d_n8;
        locals.var_sqrtpsip_dn9 = assign32700_e43358_d_n9;
        locals.var_sqrtpsip_dn10 = assign32700_e43358_d_n10;
        locals.var_sqrtpsip_dn11 = assign32700_e43358_d_n11;
        locals.var_sqrtpsip_dn12 = assign32700_e43358_d_n12;
        locals.var_sqrtpsip_dn13 = assign32700_e43358_d_n13;
        locals.var_sqrtpsip_dn14 = assign32700_e43358_d_n14;

        let (assign32710_e43368, assign32710_e43368_d_n0, assign32710_e43368_d_n2, assign32710_e43368_d_n3, assign32710_e43368_d_n4, assign32710_e43368_d_n5, assign32710_e43368_d_n6, assign32710_e43368_d_n7, assign32710_e43368_d_n8, assign32710_e43368_d_n9, assign32710_e43368_d_n10, assign32710_e43368_d_n11, assign32710_e43368_d_n12, assign32710_e43368_d_n13, assign32710_e43368_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32710_e43362: f64 = (locals.var_psip - locals.var_qs_edge);
        let assign32710_e43364: f64 = (assign32710_e43362 - locals.var_qdeff_edge);
        let assign32710_e43366: f64 = (assign32710_e43364 - 1.0);
        (assign32710_e43366, ((locals.var_psip_dn0 - locals.var_qs_edge_dn0) - locals.var_qdeff_edge_dn0), ((locals.var_psip_dn2 - locals.var_qs_edge_dn2) - locals.var_qdeff_edge_dn2), ((locals.var_psip_dn3 - locals.var_qs_edge_dn3) - locals.var_qdeff_edge_dn3), ((locals.var_psip_dn4 - locals.var_qs_edge_dn4) - locals.var_qdeff_edge_dn4), ((locals.var_psip_dn5 - locals.var_qs_edge_dn5) - locals.var_qdeff_edge_dn5), ((locals.var_psip_dn6 - locals.var_qs_edge_dn6) - locals.var_qdeff_edge_dn6), ((locals.var_psip_dn7 - locals.var_qs_edge_dn7) - locals.var_qdeff_edge_dn7), ((locals.var_psip_dn8 - locals.var_qs_edge_dn8) - locals.var_qdeff_edge_dn8), ((locals.var_psip_dn9 - locals.var_qs_edge_dn9) - locals.var_qdeff_edge_dn9), ((locals.var_psip_dn10 - locals.var_qs_edge_dn10) - locals.var_qdeff_edge_dn10), ((locals.var_psip_dn11 - locals.var_qs_edge_dn11) - locals.var_qdeff_edge_dn11), ((locals.var_psip_dn12 - locals.var_qs_edge_dn12) - locals.var_qdeff_edge_dn12), ((locals.var_psip_dn13 - locals.var_qs_edge_dn13) - locals.var_qdeff_edge_dn13), ((locals.var_psip_dn14 - locals.var_qs_edge_dn14) - locals.var_qdeff_edge_dn14),)
    } else {
        (locals.var_psiavg, locals.var_psiavg_dn0, locals.var_psiavg_dn2, locals.var_psiavg_dn3, locals.var_psiavg_dn4, locals.var_psiavg_dn5, locals.var_psiavg_dn6, locals.var_psiavg_dn7, locals.var_psiavg_dn8, locals.var_psiavg_dn9, locals.var_psiavg_dn10, locals.var_psiavg_dn11, locals.var_psiavg_dn12, locals.var_psiavg_dn13, locals.var_psiavg_dn14,)
    }
};
        locals.var_psiavg = assign32710_e43368;
        locals.var_psiavg_dn0 = assign32710_e43368_d_n0;
        locals.var_psiavg_dn2 = assign32710_e43368_d_n2;
        locals.var_psiavg_dn3 = assign32710_e43368_d_n3;
        locals.var_psiavg_dn4 = assign32710_e43368_d_n4;
        locals.var_psiavg_dn5 = assign32710_e43368_d_n5;
        locals.var_psiavg_dn6 = assign32710_e43368_d_n6;
        locals.var_psiavg_dn7 = assign32710_e43368_d_n7;
        locals.var_psiavg_dn8 = assign32710_e43368_d_n8;
        locals.var_psiavg_dn9 = assign32710_e43368_d_n9;
        locals.var_psiavg_dn10 = assign32710_e43368_d_n10;
        locals.var_psiavg_dn11 = assign32710_e43368_d_n11;
        locals.var_psiavg_dn12 = assign32710_e43368_d_n12;
        locals.var_psiavg_dn13 = assign32710_e43368_d_n13;
        locals.var_psiavg_dn14 = assign32710_e43368_d_n14;

        let assign32720_e43374: f64 = (-2500.0);
        let assign32720_e43376: f64 = (assign32720_e43374 * 2.0);
        let assign32720_e43378: f64 = if ((1.0 == 0.0) && (locals.var_psiavg < assign32720_e43376)) { 1.0 } else { 0.0 };
        locals.var_guard744 = assign32720_e43378;

        let (assign32730_e43391, assign32730_e43391_d_n0, assign32730_e43391_d_n2, assign32730_e43391_d_n3, assign32730_e43391_d_n4, assign32730_e43391_d_n5, assign32730_e43391_d_n6, assign32730_e43391_d_n7, assign32730_e43391_d_n8, assign32730_e43391_d_n9, assign32730_e43391_d_n10, assign32730_e43391_d_n11, assign32730_e43391_d_n12, assign32730_e43391_d_n13, assign32730_e43391_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard744 != 0.0)) {
        let assign32730_e43383: f64 = (-2.0);
        let assign32730_e43385: f64 = (assign32730_e43383 * 2.0);
        let assign32730_e43388: f64 = (16.0 * locals.var_psiavg);
        let assign32730_e43389: f64 = (assign32730_e43385 / assign32730_e43388);
        (assign32730_e43389, (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn0)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn2)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn3)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn4)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn5)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn6)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn7)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn8)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn9)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn10)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn11)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn12)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn13)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn14)) / (assign32730_e43388 * assign32730_e43388))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32730_e43391;
        locals.var_t0_dn0 = assign32730_e43391_d_n0;
        locals.var_t0_dn2 = assign32730_e43391_d_n2;
        locals.var_t0_dn3 = assign32730_e43391_d_n3;
        locals.var_t0_dn4 = assign32730_e43391_d_n4;
        locals.var_t0_dn5 = assign32730_e43391_d_n5;
        locals.var_t0_dn6 = assign32730_e43391_d_n6;
        locals.var_t0_dn7 = assign32730_e43391_d_n7;
        locals.var_t0_dn8 = assign32730_e43391_d_n8;
        locals.var_t0_dn9 = assign32730_e43391_d_n9;
        locals.var_t0_dn10 = assign32730_e43391_d_n10;
        locals.var_t0_dn11 = assign32730_e43391_d_n11;
        locals.var_t0_dn12 = assign32730_e43391_d_n12;
        locals.var_t0_dn13 = assign32730_e43391_d_n13;
        locals.var_t0_dn14 = assign32730_e43391_d_n14;

        let (assign32740_e43417, assign32740_e43417_d_n0, assign32740_e43417_d_n2, assign32740_e43417_d_n3, assign32740_e43417_d_n4, assign32740_e43417_d_n5, assign32740_e43417_d_n6, assign32740_e43417_d_n7, assign32740_e43417_d_n8, assign32740_e43417_d_n9, assign32740_e43417_d_n10, assign32740_e43417_d_n11, assign32740_e43417_d_n12, assign32740_e43417_d_n13, assign32740_e43417_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard744 == 0.0)) {
        let assign32740_e43399: f64 = (locals.var_psiavg + 1.0);
        let assign32740_e43402: f64 = (locals.var_psiavg - 1.0);
        let assign32740_e43405: f64 = (locals.var_psiavg - 1.0);
        let assign32740_e43406: f64 = (assign32740_e43402 * assign32740_e43405);
        let assign32740_e43409: f64 = (0.25 * 2.0);
        let assign32740_e43411: f64 = (assign32740_e43409 * 2.0);
        let assign32740_e43412: f64 = (assign32740_e43406 + assign32740_e43411);
        let assign32740_e43413: f64 = (assign32740_e43412).sqrt();
        let assign32740_e43414: f64 = (assign32740_e43399 + assign32740_e43413);
        let assign32740_e43415: f64 = (0.5 * assign32740_e43414);
        (assign32740_e43415, (0.5 * (locals.var_psiavg_dn0 + (((locals.var_psiavg_dn0 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn0)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn2 + (((locals.var_psiavg_dn2 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn2)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn3 + (((locals.var_psiavg_dn3 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn3)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn4 + (((locals.var_psiavg_dn4 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn4)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn5 + (((locals.var_psiavg_dn5 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn5)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn6 + (((locals.var_psiavg_dn6 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn6)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn7 + (((locals.var_psiavg_dn7 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn7)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn8 + (((locals.var_psiavg_dn8 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn8)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn9 + (((locals.var_psiavg_dn9 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn9)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn10 + (((locals.var_psiavg_dn10 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn10)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn11 + (((locals.var_psiavg_dn11 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn11)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn12 + (((locals.var_psiavg_dn12 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn12)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn13 + (((locals.var_psiavg_dn13 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn13)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn14 + (((locals.var_psiavg_dn14 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn14)) / (2.0 * assign32740_e43413)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32740_e43417;
        locals.var_t0_dn0 = assign32740_e43417_d_n0;
        locals.var_t0_dn2 = assign32740_e43417_d_n2;
        locals.var_t0_dn3 = assign32740_e43417_d_n3;
        locals.var_t0_dn4 = assign32740_e43417_d_n4;
        locals.var_t0_dn5 = assign32740_e43417_d_n5;
        locals.var_t0_dn6 = assign32740_e43417_d_n6;
        locals.var_t0_dn7 = assign32740_e43417_d_n7;
        locals.var_t0_dn8 = assign32740_e43417_d_n8;
        locals.var_t0_dn9 = assign32740_e43417_d_n9;
        locals.var_t0_dn10 = assign32740_e43417_d_n10;
        locals.var_t0_dn11 = assign32740_e43417_d_n11;
        locals.var_t0_dn12 = assign32740_e43417_d_n12;
        locals.var_t0_dn13 = assign32740_e43417_d_n13;
        locals.var_t0_dn14 = assign32740_e43417_d_n14;

        let (assign32750_e43422, assign32750_e43422_d_n0, assign32750_e43422_d_n2, assign32750_e43422_d_n3, assign32750_e43422_d_n4, assign32750_e43422_d_n5, assign32750_e43422_d_n6, assign32750_e43422_d_n7, assign32750_e43422_d_n8, assign32750_e43422_d_n9, assign32750_e43422_d_n10, assign32750_e43422_d_n11, assign32750_e43422_d_n12, assign32750_e43422_d_n13, assign32750_e43422_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32750_e43420: f64 = (locals.var_t0).sqrt();
        (assign32750_e43420, (locals.var_t0_dn0 / (2.0 * assign32750_e43420)), (locals.var_t0_dn2 / (2.0 * assign32750_e43420)), (locals.var_t0_dn3 / (2.0 * assign32750_e43420)), (locals.var_t0_dn4 / (2.0 * assign32750_e43420)), (locals.var_t0_dn5 / (2.0 * assign32750_e43420)), (locals.var_t0_dn6 / (2.0 * assign32750_e43420)), (locals.var_t0_dn7 / (2.0 * assign32750_e43420)), (locals.var_t0_dn8 / (2.0 * assign32750_e43420)), (locals.var_t0_dn9 / (2.0 * assign32750_e43420)), (locals.var_t0_dn10 / (2.0 * assign32750_e43420)), (locals.var_t0_dn11 / (2.0 * assign32750_e43420)), (locals.var_t0_dn12 / (2.0 * assign32750_e43420)), (locals.var_t0_dn13 / (2.0 * assign32750_e43420)), (locals.var_t0_dn14 / (2.0 * assign32750_e43420)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32750_e43422;
        locals.var_t2_dn0 = assign32750_e43422_d_n0;
        locals.var_t2_dn2 = assign32750_e43422_d_n2;
        locals.var_t2_dn3 = assign32750_e43422_d_n3;
        locals.var_t2_dn4 = assign32750_e43422_d_n4;
        locals.var_t2_dn5 = assign32750_e43422_d_n5;
        locals.var_t2_dn6 = assign32750_e43422_d_n6;
        locals.var_t2_dn7 = assign32750_e43422_d_n7;
        locals.var_t2_dn8 = assign32750_e43422_d_n8;
        locals.var_t2_dn9 = assign32750_e43422_d_n9;
        locals.var_t2_dn10 = assign32750_e43422_d_n10;
        locals.var_t2_dn11 = assign32750_e43422_d_n11;
        locals.var_t2_dn12 = assign32750_e43422_d_n12;
        locals.var_t2_dn13 = assign32750_e43422_d_n13;
        locals.var_t2_dn14 = assign32750_e43422_d_n14;

        let (assign32760_e43432, assign32760_e43432_d_n0, assign32760_e43432_d_n2, assign32760_e43432_d_n3, assign32760_e43432_d_n4, assign32760_e43432_d_n5, assign32760_e43432_d_n6, assign32760_e43432_d_n7, assign32760_e43432_d_n8, assign32760_e43432_d_n9, assign32760_e43432_d_n10, assign32760_e43432_d_n11, assign32760_e43432_d_n12, assign32760_e43432_d_n13, assign32760_e43432_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32760_e43428: f64 = (locals.var_sqrtpsip + locals.var_t2);
        let assign32760_e43429: f64 = (locals.var_gam_edge / assign32760_e43428);
        let assign32760_e43430: f64 = (1.0 + assign32760_e43429);
        (assign32760_e43430, (((locals.var_gam_edge_dn0 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn0 + locals.var_t2_dn0))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn2 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn2 + locals.var_t2_dn2))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn3 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn3 + locals.var_t2_dn3))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn4 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn4 + locals.var_t2_dn4))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn5 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn5 + locals.var_t2_dn5))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn6 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn6 + locals.var_t2_dn6))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn7 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn7 + locals.var_t2_dn7))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn8 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn8 + locals.var_t2_dn8))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn9 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn9 + locals.var_t2_dn9))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn10 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn10 + locals.var_t2_dn10))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn11 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn11 + locals.var_t2_dn11))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn12 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn12 + locals.var_t2_dn12))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn13 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn13 + locals.var_t2_dn13))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn14 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn14 + locals.var_t2_dn14))) / (assign32760_e43428 * assign32760_e43428)),)
    } else {
        (locals.var_nq_edge, locals.var_nq_edge_dn0, locals.var_nq_edge_dn2, locals.var_nq_edge_dn3, locals.var_nq_edge_dn4, locals.var_nq_edge_dn5, locals.var_nq_edge_dn6, locals.var_nq_edge_dn7, locals.var_nq_edge_dn8, locals.var_nq_edge_dn9, locals.var_nq_edge_dn10, locals.var_nq_edge_dn11, locals.var_nq_edge_dn12, locals.var_nq_edge_dn13, locals.var_nq_edge_dn14,)
    }
};
        locals.var_nq_edge = assign32760_e43432;
        locals.var_nq_edge_dn0 = assign32760_e43432_d_n0;
        locals.var_nq_edge_dn2 = assign32760_e43432_d_n2;
        locals.var_nq_edge_dn3 = assign32760_e43432_d_n3;
        locals.var_nq_edge_dn4 = assign32760_e43432_d_n4;
        locals.var_nq_edge_dn5 = assign32760_e43432_d_n5;
        locals.var_nq_edge_dn6 = assign32760_e43432_d_n6;
        locals.var_nq_edge_dn7 = assign32760_e43432_d_n7;
        locals.var_nq_edge_dn8 = assign32760_e43432_d_n8;
        locals.var_nq_edge_dn9 = assign32760_e43432_d_n9;
        locals.var_nq_edge_dn10 = assign32760_e43432_d_n10;
        locals.var_nq_edge_dn11 = assign32760_e43432_d_n11;
        locals.var_nq_edge_dn12 = assign32760_e43432_d_n12;
        locals.var_nq_edge_dn13 = assign32760_e43432_d_n13;
        locals.var_nq_edge_dn14 = assign32760_e43432_d_n14;

        let (assign32770_e43464, assign32770_e43464_d_n0, assign32770_e43464_d_n2, assign32770_e43464_d_n3, assign32770_e43464_d_n4, assign32770_e43464_d_n5, assign32770_e43464_d_n6, assign32770_e43464_d_n7, assign32770_e43464_d_n8, assign32770_e43464_d_n9, assign32770_e43464_d_n10, assign32770_e43464_d_n11, assign32770_e43464_d_n12, assign32770_e43464_d_n13, assign32770_e43464_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32770_e43436: f64 = (2.0 * p.p2);
        let assign32770_e43438: f64 = (assign32770_e43436 * locals.var_nq_edge);
        let assign32770_e43440: f64 = (assign32770_e43438 * locals.var_ueff);
        let assign32770_e43442: f64 = (assign32770_e43440 * p.p957);
        let assign32770_e43444: f64 = (assign32770_e43442 / locals.var_leff);
        let assign32770_e43446: f64 = (assign32770_e43444 * locals.var_cox);
        let assign32770_e43448: f64 = (assign32770_e43446 * locals.var_nvt);
        let assign32770_e43450: f64 = (assign32770_e43448 * locals.var_nvt);
        let assign32770_e43453: f64 = (locals.var_qs_edge - locals.var_qdeff_edge);
        let assign32770_e43456: f64 = (1.0 + locals.var_qs_edge);
        let assign32770_e43458: f64 = (assign32770_e43456 + locals.var_qdeff_edge);
        let assign32770_e43459: f64 = (assign32770_e43453 * assign32770_e43458);
        let assign32770_e43460: f64 = (assign32770_e43450 * assign32770_e43459);
        let assign32770_e43462: f64 = (assign32770_e43460 * locals.var_moc);
        (assign32770_e43462, ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn0) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn0)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn0)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn0)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn0 - locals.var_qdeff_edge_dn0) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn0 + locals.var_qdeff_edge_dn0))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn0)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn2) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn2)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn2)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn2)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn2 - locals.var_qdeff_edge_dn2) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn2 + locals.var_qdeff_edge_dn2))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn2)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn3) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn3)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn3)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn3)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn3 + locals.var_qdeff_edge_dn3))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn3)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn4) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn4)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn4)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn4)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn4 + locals.var_qdeff_edge_dn4))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn4)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn5) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn5)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn5)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn5)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn5 + locals.var_qdeff_edge_dn5))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn5)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn6) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn6)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn6)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn6)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn6 + locals.var_qdeff_edge_dn6))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn6)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn7) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn7)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn7)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn7)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn7 + locals.var_qdeff_edge_dn7))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn7)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn8) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn8)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn8)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn8)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn8 + locals.var_qdeff_edge_dn8))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn8)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn9) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn9)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn9)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn9)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn9 + locals.var_qdeff_edge_dn9))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn9)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn10) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn10)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn10)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn10)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn10 + locals.var_qdeff_edge_dn10))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn10)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn11) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn11)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn11)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn11)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn11 + locals.var_qdeff_edge_dn11))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn11)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn12) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn12)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn12)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn12)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn12 - locals.var_qdeff_edge_dn12) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn12 + locals.var_qdeff_edge_dn12))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn12)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn13) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn13)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn13)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn13)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn13 - locals.var_qdeff_edge_dn13) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn13 + locals.var_qdeff_edge_dn13))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn13)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn14) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn14)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn14)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn14)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn14 - locals.var_qdeff_edge_dn14) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn14 + locals.var_qdeff_edge_dn14))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn14)),)
    } else {
        (locals.var_ids_edge, locals.var_ids_edge_dn0, locals.var_ids_edge_dn2, locals.var_ids_edge_dn3, locals.var_ids_edge_dn4, locals.var_ids_edge_dn5, locals.var_ids_edge_dn6, locals.var_ids_edge_dn7, locals.var_ids_edge_dn8, locals.var_ids_edge_dn9, locals.var_ids_edge_dn10, locals.var_ids_edge_dn11, locals.var_ids_edge_dn12, locals.var_ids_edge_dn13, locals.var_ids_edge_dn14,)
    }
};
        locals.var_ids_edge = assign32770_e43464;
        locals.var_ids_edge_dn0 = assign32770_e43464_d_n0;
        locals.var_ids_edge_dn2 = assign32770_e43464_d_n2;
        locals.var_ids_edge_dn3 = assign32770_e43464_d_n3;
        locals.var_ids_edge_dn4 = assign32770_e43464_d_n4;
        locals.var_ids_edge_dn5 = assign32770_e43464_d_n5;
        locals.var_ids_edge_dn6 = assign32770_e43464_d_n6;
        locals.var_ids_edge_dn7 = assign32770_e43464_d_n7;
        locals.var_ids_edge_dn8 = assign32770_e43464_d_n8;
        locals.var_ids_edge_dn9 = assign32770_e43464_d_n9;
        locals.var_ids_edge_dn10 = assign32770_e43464_d_n10;
        locals.var_ids_edge_dn11 = assign32770_e43464_d_n11;
        locals.var_ids_edge_dn12 = assign32770_e43464_d_n12;
        locals.var_ids_edge_dn13 = assign32770_e43464_d_n13;
        locals.var_ids_edge_dn14 = assign32770_e43464_d_n14;

        let (assign32780_e43470, assign32780_e43470_d_n0, assign32780_e43470_d_n2, assign32780_e43470_d_n3, assign32780_e43470_d_n4, assign32780_e43470_d_n5, assign32780_e43470_d_n6, assign32780_e43470_d_n7, assign32780_e43470_d_n8, assign32780_e43470_d_n9, assign32780_e43470_d_n10, assign32780_e43470_d_n11, assign32780_e43470_d_n12, assign32780_e43470_d_n13, assign32780_e43470_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32780_e43468: f64 = (locals.var_ids_edge + locals.var_ids);
        (assign32780_e43468, (locals.var_ids_edge_dn0 + locals.var_ids_dn0), (locals.var_ids_edge_dn2 + locals.var_ids_dn2), (locals.var_ids_edge_dn3 + locals.var_ids_dn3), (locals.var_ids_edge_dn4 + locals.var_ids_dn4), (locals.var_ids_edge_dn5 + locals.var_ids_dn5), (locals.var_ids_edge_dn6 + locals.var_ids_dn6), (locals.var_ids_edge_dn7 + locals.var_ids_dn7), (locals.var_ids_edge_dn8 + locals.var_ids_dn8), (locals.var_ids_edge_dn9 + locals.var_ids_dn9), (locals.var_ids_edge_dn10 + locals.var_ids_dn10), (locals.var_ids_edge_dn11 + locals.var_ids_dn11), (locals.var_ids_edge_dn12 + locals.var_ids_dn12), (locals.var_ids_edge_dn13 + locals.var_ids_dn13), (locals.var_ids_edge_dn14 + locals.var_ids_dn14),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn3, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn13, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign32780_e43470;
        locals.var_ids_dn0 = assign32780_e43470_d_n0;
        locals.var_ids_dn2 = assign32780_e43470_d_n2;
        locals.var_ids_dn3 = assign32780_e43470_d_n3;
        locals.var_ids_dn4 = assign32780_e43470_d_n4;
        locals.var_ids_dn5 = assign32780_e43470_d_n5;
        locals.var_ids_dn6 = assign32780_e43470_d_n6;
        locals.var_ids_dn7 = assign32780_e43470_d_n7;
        locals.var_ids_dn8 = assign32780_e43470_d_n8;
        locals.var_ids_dn9 = assign32780_e43470_d_n9;
        locals.var_ids_dn10 = assign32780_e43470_d_n10;
        locals.var_ids_dn11 = assign32780_e43470_d_n11;
        locals.var_ids_dn12 = assign32780_e43470_d_n12;
        locals.var_ids_dn13 = assign32780_e43470_d_n13;
        locals.var_ids_dn14 = assign32780_e43470_d_n14;

        let (assign32790_e43476,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32790_e43474: f64 = (p.p785 * p.p1062);
        (assign32790_e43474,)
    } else {
        (locals.var_noia_edge,)
    }
};
        locals.var_noia_edge = assign32790_e43476;

        let (assign32800_e43482,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32800_e43480: f64 = (p.p799 * p.p1062);
        (assign32800_e43480,)
    } else {
        (locals.var_noib_edge,)
    }
};
        locals.var_noib_edge = assign32800_e43482;

        let (assign32810_e43488,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32810_e43486: f64 = (p.p800 * p.p1062);
        (assign32810_e43486,)
    } else {
        (locals.var_noic_edge,)
    }
};
        locals.var_noic_edge = assign32810_e43488;

        let (assign32820_e43496,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32820_e43493: f64 = (2.0 * locals.var_lintnoi_i);
        let assign32820_e43494: f64 = (locals.var_leff - assign32820_e43493);
        (assign32820_e43494,)
    } else {
        (locals.var_leffnoi_edge,)
    }
};
        locals.var_leffnoi_edge = assign32820_e43496;

        let (assign32830_e43502,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32830_e43500: f64 = (locals.var_leffnoi_edge * locals.var_leffnoi_edge);
        (assign32830_e43500,)
    } else {
        (locals.var_leffnoisq_edge,)
    }
};
        locals.var_leffnoisq_edge = assign32830_e43502;

    }

    pub(super) fn stamp_transient_block_101(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32840_e43514, assign32840_e43514_d_n0, assign32840_e43514_d_n2, assign32840_e43514_d_n3, assign32840_e43514_d_n4, assign32840_e43514_d_n5, assign32840_e43514_d_n6, assign32840_e43514_d_n7, assign32840_e43514_d_n8, assign32840_e43514_d_n9, assign32840_e43514_d_n10, assign32840_e43514_d_n11, assign32840_e43514_d_n12, assign32840_e43514_d_n13, assign32840_e43514_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32840_e43506: f64 = (locals.var_vt / 1.60219e-19);
        let assign32840_e43509: f64 = (locals.var_cox + locals.var_cdep);
        let assign32840_e43511: f64 = (assign32840_e43509 + locals.var_citedge_i);
        let assign32840_e43512: f64 = (assign32840_e43506 * assign32840_e43511);
        (assign32840_e43512, (assign32840_e43506 * locals.var_cdep_dn0), (assign32840_e43506 * locals.var_cdep_dn2), (assign32840_e43506 * locals.var_cdep_dn3), (((locals.var_vt_dn4 / 1.60219e-19) * assign32840_e43511) + (assign32840_e43506 * locals.var_cdep_dn4)), (assign32840_e43506 * locals.var_cdep_dn5), (assign32840_e43506 * locals.var_cdep_dn6), (assign32840_e43506 * locals.var_cdep_dn7), (assign32840_e43506 * locals.var_cdep_dn8), (assign32840_e43506 * locals.var_cdep_dn9), (assign32840_e43506 * locals.var_cdep_dn10), (assign32840_e43506 * locals.var_cdep_dn11), (assign32840_e43506 * locals.var_cdep_dn12), (assign32840_e43506 * locals.var_cdep_dn13), (assign32840_e43506 * locals.var_cdep_dn14),)
    } else {
        (locals.var_nstar, locals.var_nstar_dn0, locals.var_nstar_dn2, locals.var_nstar_dn3, locals.var_nstar_dn4, locals.var_nstar_dn5, locals.var_nstar_dn6, locals.var_nstar_dn7, locals.var_nstar_dn8, locals.var_nstar_dn9, locals.var_nstar_dn10, locals.var_nstar_dn11, locals.var_nstar_dn12, locals.var_nstar_dn13, locals.var_nstar_dn14,)
    }
};
        locals.var_nstar = assign32840_e43514;
        locals.var_nstar_dn0 = assign32840_e43514_d_n0;
        locals.var_nstar_dn2 = assign32840_e43514_d_n2;
        locals.var_nstar_dn3 = assign32840_e43514_d_n3;
        locals.var_nstar_dn4 = assign32840_e43514_d_n4;
        locals.var_nstar_dn5 = assign32840_e43514_d_n5;
        locals.var_nstar_dn6 = assign32840_e43514_d_n6;
        locals.var_nstar_dn7 = assign32840_e43514_d_n7;
        locals.var_nstar_dn8 = assign32840_e43514_d_n8;
        locals.var_nstar_dn9 = assign32840_e43514_d_n9;
        locals.var_nstar_dn10 = assign32840_e43514_d_n10;
        locals.var_nstar_dn11 = assign32840_e43514_d_n11;
        locals.var_nstar_dn12 = assign32840_e43514_d_n12;
        locals.var_nstar_dn13 = assign32840_e43514_d_n13;
        locals.var_nstar_dn14 = assign32840_e43514_d_n14;

        let (assign32850_e43528, assign32850_e43528_d_n0, assign32850_e43528_d_n2, assign32850_e43528_d_n3, assign32850_e43528_d_n4, assign32850_e43528_d_n5, assign32850_e43528_d_n6, assign32850_e43528_d_n7, assign32850_e43528_d_n8, assign32850_e43528_d_n9, assign32850_e43528_d_n10, assign32850_e43528_d_n11, assign32850_e43528_d_n12, assign32850_e43528_d_n13, assign32850_e43528_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32850_e43518: f64 = (2.0 * locals.var_nq_edge);
        let assign32850_e43520: f64 = (assign32850_e43518 * locals.var_cox);
        let assign32850_e43522: f64 = (assign32850_e43520 * locals.var_vt);
        let assign32850_e43524: f64 = (assign32850_e43522 * locals.var_qdeff_edge);
        let assign32850_e43526: f64 = (assign32850_e43524 / 1.60219e-19);
        (assign32850_e43526, ((((((2.0 * locals.var_nq_edge_dn0) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn0)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn2) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn2)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn3)) / 1.60219e-19), (((((((2.0 * locals.var_nq_edge_dn4) * locals.var_cox) * locals.var_vt) + (assign32850_e43520 * locals.var_vt_dn4)) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn4)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn5) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn5)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn6)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn7)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn8)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn9)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn10)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn11)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn12) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn12)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn13) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn13)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn14) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn14)) / 1.60219e-19),)
    } else {
        (locals.var_nl, locals.var_nl_dn0, locals.var_nl_dn2, locals.var_nl_dn3, locals.var_nl_dn4, locals.var_nl_dn5, locals.var_nl_dn6, locals.var_nl_dn7, locals.var_nl_dn8, locals.var_nl_dn9, locals.var_nl_dn10, locals.var_nl_dn11, locals.var_nl_dn12, locals.var_nl_dn13, locals.var_nl_dn14,)
    }
};
        locals.var_nl = assign32850_e43528;
        locals.var_nl_dn0 = assign32850_e43528_d_n0;
        locals.var_nl_dn2 = assign32850_e43528_d_n2;
        locals.var_nl_dn3 = assign32850_e43528_d_n3;
        locals.var_nl_dn4 = assign32850_e43528_d_n4;
        locals.var_nl_dn5 = assign32850_e43528_d_n5;
        locals.var_nl_dn6 = assign32850_e43528_d_n6;
        locals.var_nl_dn7 = assign32850_e43528_d_n7;
        locals.var_nl_dn8 = assign32850_e43528_d_n8;
        locals.var_nl_dn9 = assign32850_e43528_d_n9;
        locals.var_nl_dn10 = assign32850_e43528_d_n10;
        locals.var_nl_dn11 = assign32850_e43528_d_n11;
        locals.var_nl_dn12 = assign32850_e43528_d_n12;
        locals.var_nl_dn13 = assign32850_e43528_d_n13;
        locals.var_nl_dn14 = assign32850_e43528_d_n14;

        let (assign32860_e43543, assign32860_e43543_d_n0, assign32860_e43543_d_n2, assign32860_e43543_d_n3, assign32860_e43543_d_n4, assign32860_e43543_d_n5, assign32860_e43543_d_n6, assign32860_e43543_d_n7, assign32860_e43543_d_n8, assign32860_e43543_d_n9, assign32860_e43543_d_n10, assign32860_e43543_d_n11, assign32860_e43543_d_n12, assign32860_e43543_d_n13, assign32860_e43543_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32860_e43532: f64 = (1.60219e-19 * 1.60219e-19);
        let assign32860_e43534: f64 = (assign32860_e43532 * 1.60219e-19);
        let assign32860_e43536: f64 = (assign32860_e43534 * locals.var_vt);
        let assign32860_e43538: f64 = (locals.var_ids_edge).abs();
        let assign32860_e43539: f64 = (assign32860_e43536 * assign32860_e43538);
        let assign32860_e43541: f64 = (assign32860_e43539 * locals.var_ueff);
        (assign32860_e43541, (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn0 } else { (-locals.var_ids_edge_dn0) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn0)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn2 } else { (-locals.var_ids_edge_dn2) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn2)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn3 } else { (-locals.var_ids_edge_dn3) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn3)), (((((assign32860_e43534 * locals.var_vt_dn4) * assign32860_e43538) + (assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn4 } else { (-locals.var_ids_edge_dn4) })) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn4)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn5 } else { (-locals.var_ids_edge_dn5) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn5)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn6 } else { (-locals.var_ids_edge_dn6) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn6)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn7 } else { (-locals.var_ids_edge_dn7) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn7)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn8 } else { (-locals.var_ids_edge_dn8) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn8)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn9 } else { (-locals.var_ids_edge_dn9) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn9)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn10 } else { (-locals.var_ids_edge_dn10) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn10)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn11 } else { (-locals.var_ids_edge_dn11) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn11)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn12 } else { (-locals.var_ids_edge_dn12) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn12)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn13 } else { (-locals.var_ids_edge_dn13) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn13)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn14 } else { (-locals.var_ids_edge_dn14) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn14)),)
    } else {
        (locals.var_t0a, locals.var_t0a_dn0, locals.var_t0a_dn2, locals.var_t0a_dn3, locals.var_t0a_dn4, locals.var_t0a_dn5, locals.var_t0a_dn6, locals.var_t0a_dn7, locals.var_t0a_dn8, locals.var_t0a_dn9, locals.var_t0a_dn10, locals.var_t0a_dn11, locals.var_t0a_dn12, locals.var_t0a_dn13, locals.var_t0a_dn14,)
    }
};
        locals.var_t0a = assign32860_e43543;
        locals.var_t0a_dn0 = assign32860_e43543_d_n0;
        locals.var_t0a_dn2 = assign32860_e43543_d_n2;
        locals.var_t0a_dn3 = assign32860_e43543_d_n3;
        locals.var_t0a_dn4 = assign32860_e43543_d_n4;
        locals.var_t0a_dn5 = assign32860_e43543_d_n5;
        locals.var_t0a_dn6 = assign32860_e43543_d_n6;
        locals.var_t0a_dn7 = assign32860_e43543_d_n7;
        locals.var_t0a_dn8 = assign32860_e43543_d_n8;
        locals.var_t0a_dn9 = assign32860_e43543_d_n9;
        locals.var_t0a_dn10 = assign32860_e43543_d_n10;
        locals.var_t0a_dn11 = assign32860_e43543_d_n11;
        locals.var_t0a_dn12 = assign32860_e43543_d_n12;
        locals.var_t0a_dn13 = assign32860_e43543_d_n13;
        locals.var_t0a_dn14 = assign32860_e43543_d_n14;

        let (assign32870_e43553, assign32870_e43553_d_n0, assign32870_e43553_d_n2, assign32870_e43553_d_n3, assign32870_e43553_d_n4, assign32870_e43553_d_n5, assign32870_e43553_d_n6, assign32870_e43553_d_n7, assign32870_e43553_d_n8, assign32870_e43553_d_n9, assign32870_e43553_d_n10, assign32870_e43553_d_n11, assign32870_e43553_d_n12, assign32870_e43553_d_n13, assign32870_e43553_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32870_e43547: f64 = (1.60219e-19 * locals.var_vt);
        let assign32870_e43549: f64 = (assign32870_e43547 * locals.var_ids_edge);
        let assign32870_e43551: f64 = (assign32870_e43549 * locals.var_ids_edge);
        (assign32870_e43551, (((assign32870_e43547 * locals.var_ids_edge_dn0) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn0)), (((assign32870_e43547 * locals.var_ids_edge_dn2) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn2)), (((assign32870_e43547 * locals.var_ids_edge_dn3) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn3)), (((((1.60219e-19 * locals.var_vt_dn4) * locals.var_ids_edge) + (assign32870_e43547 * locals.var_ids_edge_dn4)) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn4)), (((assign32870_e43547 * locals.var_ids_edge_dn5) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn5)), (((assign32870_e43547 * locals.var_ids_edge_dn6) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn6)), (((assign32870_e43547 * locals.var_ids_edge_dn7) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn7)), (((assign32870_e43547 * locals.var_ids_edge_dn8) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn8)), (((assign32870_e43547 * locals.var_ids_edge_dn9) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn9)), (((assign32870_e43547 * locals.var_ids_edge_dn10) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn10)), (((assign32870_e43547 * locals.var_ids_edge_dn11) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn11)), (((assign32870_e43547 * locals.var_ids_edge_dn12) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn12)), (((assign32870_e43547 * locals.var_ids_edge_dn13) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn13)), (((assign32870_e43547 * locals.var_ids_edge_dn14) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn14)),)
    } else {
        (locals.var_t0b, locals.var_t0b_dn0, locals.var_t0b_dn2, locals.var_t0b_dn3, locals.var_t0b_dn4, locals.var_t0b_dn5, locals.var_t0b_dn6, locals.var_t0b_dn7, locals.var_t0b_dn8, locals.var_t0b_dn9, locals.var_t0b_dn10, locals.var_t0b_dn11, locals.var_t0b_dn12, locals.var_t0b_dn13, locals.var_t0b_dn14,)
    }
};
        locals.var_t0b = assign32870_e43553;
        locals.var_t0b_dn0 = assign32870_e43553_d_n0;
        locals.var_t0b_dn2 = assign32870_e43553_d_n2;
        locals.var_t0b_dn3 = assign32870_e43553_d_n3;
        locals.var_t0b_dn4 = assign32870_e43553_d_n4;
        locals.var_t0b_dn5 = assign32870_e43553_d_n5;
        locals.var_t0b_dn6 = assign32870_e43553_d_n6;
        locals.var_t0b_dn7 = assign32870_e43553_d_n7;
        locals.var_t0b_dn8 = assign32870_e43553_d_n8;
        locals.var_t0b_dn9 = assign32870_e43553_d_n9;
        locals.var_t0b_dn10 = assign32870_e43553_d_n10;
        locals.var_t0b_dn11 = assign32870_e43553_d_n11;
        locals.var_t0b_dn12 = assign32870_e43553_d_n12;
        locals.var_t0b_dn13 = assign32870_e43553_d_n13;
        locals.var_t0b_dn14 = assign32870_e43553_d_n14;

        let (assign32880_e43567, assign32880_e43567_d_n0, assign32880_e43567_d_n2, assign32880_e43567_d_n3, assign32880_e43567_d_n4, assign32880_e43567_d_n5, assign32880_e43567_d_n6, assign32880_e43567_d_n7, assign32880_e43567_d_n8, assign32880_e43567_d_n9, assign32880_e43567_d_n10, assign32880_e43567_d_n11, assign32880_e43567_d_n12, assign32880_e43567_d_n13, assign32880_e43567_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32880_e43558: f64 = (locals.var_noib_edge * locals.var_nl);
        let assign32880_e43559: f64 = (locals.var_noia_edge + assign32880_e43558);
        let assign32880_e43562: f64 = (locals.var_noic_edge * locals.var_nl);
        let assign32880_e43564: f64 = (assign32880_e43562 * locals.var_nl);
        let assign32880_e43565: f64 = (assign32880_e43559 + assign32880_e43564);
        (assign32880_e43565, ((locals.var_noib_edge * locals.var_nl_dn0) + (((locals.var_noic_edge * locals.var_nl_dn0) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn0))), ((locals.var_noib_edge * locals.var_nl_dn2) + (((locals.var_noic_edge * locals.var_nl_dn2) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn2))), ((locals.var_noib_edge * locals.var_nl_dn3) + (((locals.var_noic_edge * locals.var_nl_dn3) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn3))), ((locals.var_noib_edge * locals.var_nl_dn4) + (((locals.var_noic_edge * locals.var_nl_dn4) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn4))), ((locals.var_noib_edge * locals.var_nl_dn5) + (((locals.var_noic_edge * locals.var_nl_dn5) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn5))), ((locals.var_noib_edge * locals.var_nl_dn6) + (((locals.var_noic_edge * locals.var_nl_dn6) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn6))), ((locals.var_noib_edge * locals.var_nl_dn7) + (((locals.var_noic_edge * locals.var_nl_dn7) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn7))), ((locals.var_noib_edge * locals.var_nl_dn8) + (((locals.var_noic_edge * locals.var_nl_dn8) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn8))), ((locals.var_noib_edge * locals.var_nl_dn9) + (((locals.var_noic_edge * locals.var_nl_dn9) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn9))), ((locals.var_noib_edge * locals.var_nl_dn10) + (((locals.var_noic_edge * locals.var_nl_dn10) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn10))), ((locals.var_noib_edge * locals.var_nl_dn11) + (((locals.var_noic_edge * locals.var_nl_dn11) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn11))), ((locals.var_noib_edge * locals.var_nl_dn12) + (((locals.var_noic_edge * locals.var_nl_dn12) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn12))), ((locals.var_noib_edge * locals.var_nl_dn13) + (((locals.var_noic_edge * locals.var_nl_dn13) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn13))), ((locals.var_noib_edge * locals.var_nl_dn14) + (((locals.var_noic_edge * locals.var_nl_dn14) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn14))),)
    } else {
        (locals.var_t0c, locals.var_t0c_dn0, locals.var_t0c_dn2, locals.var_t0c_dn3, locals.var_t0c_dn4, locals.var_t0c_dn5, locals.var_t0c_dn6, locals.var_t0c_dn7, locals.var_t0c_dn8, locals.var_t0c_dn9, locals.var_t0c_dn10, locals.var_t0c_dn11, locals.var_t0c_dn12, locals.var_t0c_dn13, locals.var_t0c_dn14,)
    }
};
        locals.var_t0c = assign32880_e43567;
        locals.var_t0c_dn0 = assign32880_e43567_d_n0;
        locals.var_t0c_dn2 = assign32880_e43567_d_n2;
        locals.var_t0c_dn3 = assign32880_e43567_d_n3;
        locals.var_t0c_dn4 = assign32880_e43567_d_n4;
        locals.var_t0c_dn5 = assign32880_e43567_d_n5;
        locals.var_t0c_dn6 = assign32880_e43567_d_n6;
        locals.var_t0c_dn7 = assign32880_e43567_d_n7;
        locals.var_t0c_dn8 = assign32880_e43567_d_n8;
        locals.var_t0c_dn9 = assign32880_e43567_d_n9;
        locals.var_t0c_dn10 = assign32880_e43567_d_n10;
        locals.var_t0c_dn11 = assign32880_e43567_d_n11;
        locals.var_t0c_dn12 = assign32880_e43567_d_n12;
        locals.var_t0c_dn13 = assign32880_e43567_d_n13;
        locals.var_t0c_dn14 = assign32880_e43567_d_n14;

        let (assign32890_e43577, assign32890_e43577_d_n0, assign32890_e43577_d_n2, assign32890_e43577_d_n3, assign32890_e43577_d_n4, assign32890_e43577_d_n5, assign32890_e43577_d_n6, assign32890_e43577_d_n7, assign32890_e43577_d_n8, assign32890_e43577_d_n9, assign32890_e43577_d_n10, assign32890_e43577_d_n11, assign32890_e43577_d_n12, assign32890_e43577_d_n13, assign32890_e43577_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32890_e43571: f64 = (locals.var_nl + locals.var_nstar);
        let assign32890_e43574: f64 = (locals.var_nl + locals.var_nstar);
        let assign32890_e43575: f64 = (assign32890_e43571 * assign32890_e43574);
        (assign32890_e43575, (((locals.var_nl_dn0 + locals.var_nstar_dn0) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn0 + locals.var_nstar_dn0))), (((locals.var_nl_dn2 + locals.var_nstar_dn2) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn2 + locals.var_nstar_dn2))), (((locals.var_nl_dn3 + locals.var_nstar_dn3) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn3 + locals.var_nstar_dn3))), (((locals.var_nl_dn4 + locals.var_nstar_dn4) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn4 + locals.var_nstar_dn4))), (((locals.var_nl_dn5 + locals.var_nstar_dn5) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn5 + locals.var_nstar_dn5))), (((locals.var_nl_dn6 + locals.var_nstar_dn6) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn6 + locals.var_nstar_dn6))), (((locals.var_nl_dn7 + locals.var_nstar_dn7) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn7 + locals.var_nstar_dn7))), (((locals.var_nl_dn8 + locals.var_nstar_dn8) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn8 + locals.var_nstar_dn8))), (((locals.var_nl_dn9 + locals.var_nstar_dn9) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn9 + locals.var_nstar_dn9))), (((locals.var_nl_dn10 + locals.var_nstar_dn10) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn10 + locals.var_nstar_dn10))), (((locals.var_nl_dn11 + locals.var_nstar_dn11) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn11 + locals.var_nstar_dn11))), (((locals.var_nl_dn12 + locals.var_nstar_dn12) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn12 + locals.var_nstar_dn12))), (((locals.var_nl_dn13 + locals.var_nstar_dn13) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn13 + locals.var_nstar_dn13))), (((locals.var_nl_dn14 + locals.var_nstar_dn14) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn14 + locals.var_nstar_dn14))),)
    } else {
        (locals.var_t0d, locals.var_t0d_dn0, locals.var_t0d_dn2, locals.var_t0d_dn3, locals.var_t0d_dn4, locals.var_t0d_dn5, locals.var_t0d_dn6, locals.var_t0d_dn7, locals.var_t0d_dn8, locals.var_t0d_dn9, locals.var_t0d_dn10, locals.var_t0d_dn11, locals.var_t0d_dn12, locals.var_t0d_dn13, locals.var_t0d_dn14,)
    }
};
        locals.var_t0d = assign32890_e43577;
        locals.var_t0d_dn0 = assign32890_e43577_d_n0;
        locals.var_t0d_dn2 = assign32890_e43577_d_n2;
        locals.var_t0d_dn3 = assign32890_e43577_d_n3;
        locals.var_t0d_dn4 = assign32890_e43577_d_n4;
        locals.var_t0d_dn5 = assign32890_e43577_d_n5;
        locals.var_t0d_dn6 = assign32890_e43577_d_n6;
        locals.var_t0d_dn7 = assign32890_e43577_d_n7;
        locals.var_t0d_dn8 = assign32890_e43577_d_n8;
        locals.var_t0d_dn9 = assign32890_e43577_d_n9;
        locals.var_t0d_dn10 = assign32890_e43577_d_n10;
        locals.var_t0d_dn11 = assign32890_e43577_d_n11;
        locals.var_t0d_dn12 = assign32890_e43577_d_n12;
        locals.var_t0d_dn13 = assign32890_e43577_d_n13;
        locals.var_t0d_dn14 = assign32890_e43577_d_n14;

        let (assign32900_e43585, assign32900_e43585_d_n0, assign32900_e43585_d_n2, assign32900_e43585_d_n3, assign32900_e43585_d_n4, assign32900_e43585_d_n5, assign32900_e43585_d_n6, assign32900_e43585_d_n7, assign32900_e43585_d_n8, assign32900_e43585_d_n9, assign32900_e43585_d_n10, assign32900_e43585_d_n11, assign32900_e43585_d_n12, assign32900_e43585_d_n13, assign32900_e43585_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32900_e43581: f64 = (locals.var_noia_edge * 1.60219e-19);
        let assign32900_e43583: f64 = (assign32900_e43581 * locals.var_vt);
        (assign32900_e43583, 0.0, 0.0, 0.0, (assign32900_e43581 * locals.var_vt_dn4), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0e, locals.var_t0e_dn0, locals.var_t0e_dn2, locals.var_t0e_dn3, locals.var_t0e_dn4, locals.var_t0e_dn5, locals.var_t0e_dn6, locals.var_t0e_dn7, locals.var_t0e_dn8, locals.var_t0e_dn9, locals.var_t0e_dn10, locals.var_t0e_dn11, locals.var_t0e_dn12, locals.var_t0e_dn13, locals.var_t0e_dn14,)
    }
};
        locals.var_t0e = assign32900_e43585;
        locals.var_t0e_dn0 = assign32900_e43585_d_n0;
        locals.var_t0e_dn2 = assign32900_e43585_d_n2;
        locals.var_t0e_dn3 = assign32900_e43585_d_n3;
        locals.var_t0e_dn4 = assign32900_e43585_d_n4;
        locals.var_t0e_dn5 = assign32900_e43585_d_n5;
        locals.var_t0e_dn6 = assign32900_e43585_d_n6;
        locals.var_t0e_dn7 = assign32900_e43585_d_n7;
        locals.var_t0e_dn8 = assign32900_e43585_d_n8;
        locals.var_t0e_dn9 = assign32900_e43585_d_n9;
        locals.var_t0e_dn10 = assign32900_e43585_d_n10;
        locals.var_t0e_dn11 = assign32900_e43585_d_n11;
        locals.var_t0e_dn12 = assign32900_e43585_d_n12;
        locals.var_t0e_dn13 = assign32900_e43585_d_n13;
        locals.var_t0e_dn14 = assign32900_e43585_d_n14;

        let (assign32910_e43599, assign32910_e43599_d_n0, assign32910_e43599_d_n2, assign32910_e43599_d_n3, assign32910_e43599_d_n4, assign32910_e43599_d_n5, assign32910_e43599_d_n6, assign32910_e43599_d_n7, assign32910_e43599_d_n8, assign32910_e43599_d_n9, assign32910_e43599_d_n10, assign32910_e43599_d_n11, assign32910_e43599_d_n12, assign32910_e43599_d_n13, assign32910_e43599_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32910_e43589: f64 = (2.0 * locals.var_nq_edge);
        let assign32910_e43591: f64 = (assign32910_e43589 * locals.var_cox);
        let assign32910_e43593: f64 = (assign32910_e43591 * locals.var_vt);
        let assign32910_e43595: f64 = (assign32910_e43593 * locals.var_qs_edge);
        let assign32910_e43597: f64 = (assign32910_e43595 / 1.60219e-19);
        (assign32910_e43597, ((((((2.0 * locals.var_nq_edge_dn0) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn0)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn2) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn2)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn3)) / 1.60219e-19), (((((((2.0 * locals.var_nq_edge_dn4) * locals.var_cox) * locals.var_vt) + (assign32910_e43591 * locals.var_vt_dn4)) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn4)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn5) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn5)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn6)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn7)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn8)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn9)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn10)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn11)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn12) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn12)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn13) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn13)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn14) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn14)) / 1.60219e-19),)
    } else {
        (locals.var_n0, locals.var_n0_dn0, locals.var_n0_dn2, locals.var_n0_dn3, locals.var_n0_dn4, locals.var_n0_dn5, locals.var_n0_dn6, locals.var_n0_dn7, locals.var_n0_dn8, locals.var_n0_dn9, locals.var_n0_dn10, locals.var_n0_dn11, locals.var_n0_dn12, locals.var_n0_dn13, locals.var_n0_dn14,)
    }
};
        locals.var_n0 = assign32910_e43599;
        locals.var_n0_dn0 = assign32910_e43599_d_n0;
        locals.var_n0_dn2 = assign32910_e43599_d_n2;
        locals.var_n0_dn3 = assign32910_e43599_d_n3;
        locals.var_n0_dn4 = assign32910_e43599_d_n4;
        locals.var_n0_dn5 = assign32910_e43599_d_n5;
        locals.var_n0_dn6 = assign32910_e43599_d_n6;
        locals.var_n0_dn7 = assign32910_e43599_d_n7;
        locals.var_n0_dn8 = assign32910_e43599_d_n8;
        locals.var_n0_dn9 = assign32910_e43599_d_n9;
        locals.var_n0_dn10 = assign32910_e43599_d_n10;
        locals.var_n0_dn11 = assign32910_e43599_d_n11;
        locals.var_n0_dn12 = assign32910_e43599_d_n12;
        locals.var_n0_dn13 = assign32910_e43599_d_n13;
        locals.var_n0_dn14 = assign32910_e43599_d_n14;

        let (assign32920_e43614, assign32920_e43614_d_n0, assign32920_e43614_d_n2, assign32920_e43614_d_n3, assign32920_e43614_d_n4, assign32920_e43614_d_n5, assign32920_e43614_d_n6, assign32920_e43614_d_n7, assign32920_e43614_d_n8, assign32920_e43614_d_n9, assign32920_e43614_d_n10, assign32920_e43614_d_n11, assign32920_e43614_d_n12, assign32920_e43614_d_n13, assign32920_e43614_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32920_e43604: f64 = (locals.var_n0 + locals.var_nstar);
        let assign32920_e43607: f64 = (locals.var_nl + locals.var_nstar);
        let assign32920_e43608: f64 = (assign32920_e43604 / assign32920_e43607);
        let assign32920_e43610: f64 = (assign32920_e43608).max(1e-38);
        let assign32920_e43611: f64 = (assign32920_e43610).ln();
        let assign32920_e43612: f64 = (locals.var_noia_edge * assign32920_e43611);
        (assign32920_e43612, (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn0 + locals.var_nstar_dn0) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn0 + locals.var_nstar_dn0))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn2 + locals.var_nstar_dn2) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn2 + locals.var_nstar_dn2))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn3 + locals.var_nstar_dn3) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn3 + locals.var_nstar_dn3))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn4 + locals.var_nstar_dn4) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn4 + locals.var_nstar_dn4))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn5 + locals.var_nstar_dn5) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn5 + locals.var_nstar_dn5))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn6 + locals.var_nstar_dn6) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn6 + locals.var_nstar_dn6))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn7 + locals.var_nstar_dn7) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn7 + locals.var_nstar_dn7))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn8 + locals.var_nstar_dn8) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn8 + locals.var_nstar_dn8))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn9 + locals.var_nstar_dn9) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn9 + locals.var_nstar_dn9))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn10 + locals.var_nstar_dn10) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn10 + locals.var_nstar_dn10))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn11 + locals.var_nstar_dn11) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn11 + locals.var_nstar_dn11))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn12 + locals.var_nstar_dn12) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn12 + locals.var_nstar_dn12))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn13 + locals.var_nstar_dn13) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn13 + locals.var_nstar_dn13))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn14 + locals.var_nstar_dn14) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn14 + locals.var_nstar_dn14))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32920_e43614;
        locals.var_t1_dn0 = assign32920_e43614_d_n0;
        locals.var_t1_dn2 = assign32920_e43614_d_n2;
        locals.var_t1_dn3 = assign32920_e43614_d_n3;
        locals.var_t1_dn4 = assign32920_e43614_d_n4;
        locals.var_t1_dn5 = assign32920_e43614_d_n5;
        locals.var_t1_dn6 = assign32920_e43614_d_n6;
        locals.var_t1_dn7 = assign32920_e43614_d_n7;
        locals.var_t1_dn8 = assign32920_e43614_d_n8;
        locals.var_t1_dn9 = assign32920_e43614_d_n9;
        locals.var_t1_dn10 = assign32920_e43614_d_n10;
        locals.var_t1_dn11 = assign32920_e43614_d_n11;
        locals.var_t1_dn12 = assign32920_e43614_d_n12;
        locals.var_t1_dn13 = assign32920_e43614_d_n13;
        locals.var_t1_dn14 = assign32920_e43614_d_n14;

        let (assign32930_e43622, assign32930_e43622_d_n0, assign32930_e43622_d_n2, assign32930_e43622_d_n3, assign32930_e43622_d_n4, assign32930_e43622_d_n5, assign32930_e43622_d_n6, assign32930_e43622_d_n7, assign32930_e43622_d_n8, assign32930_e43622_d_n9, assign32930_e43622_d_n10, assign32930_e43622_d_n11, assign32930_e43622_d_n12, assign32930_e43622_d_n13, assign32930_e43622_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32930_e43619: f64 = (locals.var_n0 - locals.var_nl);
        let assign32930_e43620: f64 = (locals.var_noib_edge * assign32930_e43619);
        (assign32930_e43620, (locals.var_noib_edge * (locals.var_n0_dn0 - locals.var_nl_dn0)), (locals.var_noib_edge * (locals.var_n0_dn2 - locals.var_nl_dn2)), (locals.var_noib_edge * (locals.var_n0_dn3 - locals.var_nl_dn3)), (locals.var_noib_edge * (locals.var_n0_dn4 - locals.var_nl_dn4)), (locals.var_noib_edge * (locals.var_n0_dn5 - locals.var_nl_dn5)), (locals.var_noib_edge * (locals.var_n0_dn6 - locals.var_nl_dn6)), (locals.var_noib_edge * (locals.var_n0_dn7 - locals.var_nl_dn7)), (locals.var_noib_edge * (locals.var_n0_dn8 - locals.var_nl_dn8)), (locals.var_noib_edge * (locals.var_n0_dn9 - locals.var_nl_dn9)), (locals.var_noib_edge * (locals.var_n0_dn10 - locals.var_nl_dn10)), (locals.var_noib_edge * (locals.var_n0_dn11 - locals.var_nl_dn11)), (locals.var_noib_edge * (locals.var_n0_dn12 - locals.var_nl_dn12)), (locals.var_noib_edge * (locals.var_n0_dn13 - locals.var_nl_dn13)), (locals.var_noib_edge * (locals.var_n0_dn14 - locals.var_nl_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32930_e43622;
        locals.var_t2_dn0 = assign32930_e43622_d_n0;
        locals.var_t2_dn2 = assign32930_e43622_d_n2;
        locals.var_t2_dn3 = assign32930_e43622_d_n3;
        locals.var_t2_dn4 = assign32930_e43622_d_n4;
        locals.var_t2_dn5 = assign32930_e43622_d_n5;
        locals.var_t2_dn6 = assign32930_e43622_d_n6;
        locals.var_t2_dn7 = assign32930_e43622_d_n7;
        locals.var_t2_dn8 = assign32930_e43622_d_n8;
        locals.var_t2_dn9 = assign32930_e43622_d_n9;
        locals.var_t2_dn10 = assign32930_e43622_d_n10;
        locals.var_t2_dn11 = assign32930_e43622_d_n11;
        locals.var_t2_dn12 = assign32930_e43622_d_n12;
        locals.var_t2_dn13 = assign32930_e43622_d_n13;
        locals.var_t2_dn14 = assign32930_e43622_d_n14;

        let (assign32940_e43636, assign32940_e43636_d_n0, assign32940_e43636_d_n2, assign32940_e43636_d_n3, assign32940_e43636_d_n4, assign32940_e43636_d_n5, assign32940_e43636_d_n6, assign32940_e43636_d_n7, assign32940_e43636_d_n8, assign32940_e43636_d_n9, assign32940_e43636_d_n10, assign32940_e43636_d_n11, assign32940_e43636_d_n12, assign32940_e43636_d_n13, assign32940_e43636_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32940_e43626: f64 = (0.5 * locals.var_noic_edge);
        let assign32940_e43629: f64 = (locals.var_n0 * locals.var_n0);
        let assign32940_e43632: f64 = (locals.var_nl * locals.var_nl);
        let assign32940_e43633: f64 = (assign32940_e43629 - assign32940_e43632);
        let assign32940_e43634: f64 = (assign32940_e43626 * assign32940_e43633);
        (assign32940_e43634, (assign32940_e43626 * (((locals.var_n0_dn0 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn0)) - ((locals.var_nl_dn0 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn0)))), (assign32940_e43626 * (((locals.var_n0_dn2 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn2)) - ((locals.var_nl_dn2 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn2)))), (assign32940_e43626 * (((locals.var_n0_dn3 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn3)) - ((locals.var_nl_dn3 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn3)))), (assign32940_e43626 * (((locals.var_n0_dn4 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn4)) - ((locals.var_nl_dn4 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn4)))), (assign32940_e43626 * (((locals.var_n0_dn5 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn5)) - ((locals.var_nl_dn5 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn5)))), (assign32940_e43626 * (((locals.var_n0_dn6 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn6)) - ((locals.var_nl_dn6 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn6)))), (assign32940_e43626 * (((locals.var_n0_dn7 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn7)) - ((locals.var_nl_dn7 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn7)))), (assign32940_e43626 * (((locals.var_n0_dn8 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn8)) - ((locals.var_nl_dn8 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn8)))), (assign32940_e43626 * (((locals.var_n0_dn9 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn9)) - ((locals.var_nl_dn9 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn9)))), (assign32940_e43626 * (((locals.var_n0_dn10 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn10)) - ((locals.var_nl_dn10 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn10)))), (assign32940_e43626 * (((locals.var_n0_dn11 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn11)) - ((locals.var_nl_dn11 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn11)))), (assign32940_e43626 * (((locals.var_n0_dn12 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn12)) - ((locals.var_nl_dn12 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn12)))), (assign32940_e43626 * (((locals.var_n0_dn13 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn13)) - ((locals.var_nl_dn13 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn13)))), (assign32940_e43626 * (((locals.var_n0_dn14 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn14)) - ((locals.var_nl_dn14 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn14)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32940_e43636;
        locals.var_t3_dn0 = assign32940_e43636_d_n0;
        locals.var_t3_dn2 = assign32940_e43636_d_n2;
        locals.var_t3_dn3 = assign32940_e43636_d_n3;
        locals.var_t3_dn4 = assign32940_e43636_d_n4;
        locals.var_t3_dn5 = assign32940_e43636_d_n5;
        locals.var_t3_dn6 = assign32940_e43636_d_n6;
        locals.var_t3_dn7 = assign32940_e43636_d_n7;
        locals.var_t3_dn8 = assign32940_e43636_d_n8;
        locals.var_t3_dn9 = assign32940_e43636_d_n9;
        locals.var_t3_dn10 = assign32940_e43636_d_n10;
        locals.var_t3_dn11 = assign32940_e43636_d_n11;
        locals.var_t3_dn12 = assign32940_e43636_d_n12;
        locals.var_t3_dn13 = assign32940_e43636_d_n13;
        locals.var_t3_dn14 = assign32940_e43636_d_n14;

        let (assign32950_e43646, assign32950_e43646_d_n0, assign32950_e43646_d_n2, assign32950_e43646_d_n3, assign32950_e43646_d_n4, assign32950_e43646_d_n5, assign32950_e43646_d_n6, assign32950_e43646_d_n7, assign32950_e43646_d_n8, assign32950_e43646_d_n9, assign32950_e43646_d_n10, assign32950_e43646_d_n11, assign32950_e43646_d_n12, assign32950_e43646_d_n13, assign32950_e43646_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32950_e43640: f64 = (10000000000.0 * locals.var_leffnoisq_edge);
        let assign32950_e43642: f64 = (assign32950_e43640 * p.p957);
        let assign32950_e43644: f64 = (assign32950_e43642 * p.p2);
        (assign32950_e43644, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32950_e43646;
        locals.var_t4_dn0 = assign32950_e43646_d_n0;
        locals.var_t4_dn2 = assign32950_e43646_d_n2;
        locals.var_t4_dn3 = assign32950_e43646_d_n3;
        locals.var_t4_dn4 = assign32950_e43646_d_n4;
        locals.var_t4_dn5 = assign32950_e43646_d_n5;
        locals.var_t4_dn6 = assign32950_e43646_d_n6;
        locals.var_t4_dn7 = assign32950_e43646_d_n7;
        locals.var_t4_dn8 = assign32950_e43646_d_n8;
        locals.var_t4_dn9 = assign32950_e43646_d_n9;
        locals.var_t4_dn10 = assign32950_e43646_d_n10;
        locals.var_t4_dn11 = assign32950_e43646_d_n11;
        locals.var_t4_dn12 = assign32950_e43646_d_n12;
        locals.var_t4_dn13 = assign32950_e43646_d_n13;
        locals.var_t4_dn14 = assign32950_e43646_d_n14;

        let (assign32960_e43668, assign32960_e43668_d_n0, assign32960_e43668_d_n2, assign32960_e43668_d_n3, assign32960_e43668_d_n4, assign32960_e43668_d_n5, assign32960_e43668_d_n6, assign32960_e43668_d_n7, assign32960_e43668_d_n8, assign32960_e43668_d_n9, assign32960_e43668_d_n10, assign32960_e43668_d_n11, assign32960_e43668_d_n12, assign32960_e43668_d_n13, assign32960_e43668_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32960_e43650: f64 = (locals.var_t0a / locals.var_t0);
        let assign32960_e43653: f64 = (locals.var_t1 + locals.var_t2);
        let assign32960_e43655: f64 = (assign32960_e43653 + locals.var_t3);
        let assign32960_e43656: f64 = (assign32960_e43650 * assign32960_e43655);
        let assign32960_e43659: f64 = (locals.var_t0b / locals.var_t4);
        let assign32960_e43661: f64 = (assign32960_e43659 * locals.var_delclm);
        let assign32960_e43663: f64 = (assign32960_e43661 * locals.var_t0c);
        let assign32960_e43665: f64 = (assign32960_e43663 / locals.var_t0d);
        let assign32960_e43666: f64 = (assign32960_e43656 + assign32960_e43665);
        (assign32960_e43666, ((((((locals.var_t0a_dn0 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn0 + locals.var_t2_dn0) + locals.var_t3_dn0))) + ((((((((((locals.var_t0b_dn0 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn0)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn0)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn0)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn2 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn2 + locals.var_t2_dn2) + locals.var_t3_dn2))) + ((((((((((locals.var_t0b_dn2 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn2)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn2)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn2)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn3 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn3 + locals.var_t2_dn3) + locals.var_t3_dn3))) + ((((((((((locals.var_t0b_dn3 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn3)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn3)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn3)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn4 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn4 + locals.var_t2_dn4) + locals.var_t3_dn4))) + ((((((((((locals.var_t0b_dn4 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn4)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn4)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn4)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn5 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn5 + locals.var_t2_dn5) + locals.var_t3_dn5))) + ((((((((((locals.var_t0b_dn5 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn5)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn5)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn5)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn6 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn6 + locals.var_t2_dn6) + locals.var_t3_dn6))) + ((((((((((locals.var_t0b_dn6 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn6)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn6)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn6)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn7 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn7 + locals.var_t2_dn7) + locals.var_t3_dn7))) + ((((((((((locals.var_t0b_dn7 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn7)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn7)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn7)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn8 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn8 + locals.var_t2_dn8) + locals.var_t3_dn8))) + ((((((((((locals.var_t0b_dn8 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn8)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn8)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn8)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn9 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn9 + locals.var_t2_dn9) + locals.var_t3_dn9))) + ((((((((((locals.var_t0b_dn9 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn9)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn9)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn9)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn10 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn10 + locals.var_t2_dn10) + locals.var_t3_dn10))) + ((((((((((locals.var_t0b_dn10 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn10)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn10)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn10)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn11 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn11 + locals.var_t2_dn11) + locals.var_t3_dn11))) + ((((((((((locals.var_t0b_dn11 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn11)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn11)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn11)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn12 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn12 + locals.var_t2_dn12) + locals.var_t3_dn12))) + ((((((((((locals.var_t0b_dn12 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn12)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn12)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn12)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn13 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn13 + locals.var_t2_dn13) + locals.var_t3_dn13))) + ((((((((((locals.var_t0b_dn13 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn13)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn13)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn13)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn14 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn14 + locals.var_t2_dn14) + locals.var_t3_dn14))) + ((((((((((locals.var_t0b_dn14 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn14)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn14)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn14)) / (locals.var_t0d * locals.var_t0d))),)
    } else {
        (locals.var_ssi, locals.var_ssi_dn0, locals.var_ssi_dn2, locals.var_ssi_dn3, locals.var_ssi_dn4, locals.var_ssi_dn5, locals.var_ssi_dn6, locals.var_ssi_dn7, locals.var_ssi_dn8, locals.var_ssi_dn9, locals.var_ssi_dn10, locals.var_ssi_dn11, locals.var_ssi_dn12, locals.var_ssi_dn13, locals.var_ssi_dn14,)
    }
};
        locals.var_ssi = assign32960_e43668;
        locals.var_ssi_dn0 = assign32960_e43668_d_n0;
        locals.var_ssi_dn2 = assign32960_e43668_d_n2;
        locals.var_ssi_dn3 = assign32960_e43668_d_n3;
        locals.var_ssi_dn4 = assign32960_e43668_d_n4;
        locals.var_ssi_dn5 = assign32960_e43668_d_n5;
        locals.var_ssi_dn6 = assign32960_e43668_d_n6;
        locals.var_ssi_dn7 = assign32960_e43668_d_n7;
        locals.var_ssi_dn8 = assign32960_e43668_d_n8;
        locals.var_ssi_dn9 = assign32960_e43668_d_n9;
        locals.var_ssi_dn10 = assign32960_e43668_d_n10;
        locals.var_ssi_dn11 = assign32960_e43668_d_n11;
        locals.var_ssi_dn12 = assign32960_e43668_d_n12;
        locals.var_ssi_dn13 = assign32960_e43668_d_n13;
        locals.var_ssi_dn14 = assign32960_e43668_d_n14;

        let (assign32970_e43682, assign32970_e43682_d_n0, assign32970_e43682_d_n2, assign32970_e43682_d_n3, assign32970_e43682_d_n4, assign32970_e43682_d_n5, assign32970_e43682_d_n6, assign32970_e43682_d_n7, assign32970_e43682_d_n8, assign32970_e43682_d_n9, assign32970_e43682_d_n10, assign32970_e43682_d_n11, assign32970_e43682_d_n12, assign32970_e43682_d_n13, assign32970_e43682_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32970_e43672: f64 = (p.p957 * p.p2);
        let assign32970_e43674: f64 = (assign32970_e43672 * locals.var_leffnoi_edge);
        let assign32970_e43676: f64 = (assign32970_e43674 * 10000000000.0);
        let assign32970_e43678: f64 = (assign32970_e43676 * locals.var_nstar);
        let assign32970_e43680: f64 = (assign32970_e43678 * locals.var_nstar);
        (assign32970_e43680, (((assign32970_e43676 * locals.var_nstar_dn0) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn0)), (((assign32970_e43676 * locals.var_nstar_dn2) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn2)), (((assign32970_e43676 * locals.var_nstar_dn3) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn3)), (((assign32970_e43676 * locals.var_nstar_dn4) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn4)), (((assign32970_e43676 * locals.var_nstar_dn5) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn5)), (((assign32970_e43676 * locals.var_nstar_dn6) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn6)), (((assign32970_e43676 * locals.var_nstar_dn7) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn7)), (((assign32970_e43676 * locals.var_nstar_dn8) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn8)), (((assign32970_e43676 * locals.var_nstar_dn9) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn9)), (((assign32970_e43676 * locals.var_nstar_dn10) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn10)), (((assign32970_e43676 * locals.var_nstar_dn11) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn11)), (((assign32970_e43676 * locals.var_nstar_dn12) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn12)), (((assign32970_e43676 * locals.var_nstar_dn13) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn13)), (((assign32970_e43676 * locals.var_nstar_dn14) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32970_e43682;
        locals.var_t5_dn0 = assign32970_e43682_d_n0;
        locals.var_t5_dn2 = assign32970_e43682_d_n2;
        locals.var_t5_dn3 = assign32970_e43682_d_n3;
        locals.var_t5_dn4 = assign32970_e43682_d_n4;
        locals.var_t5_dn5 = assign32970_e43682_d_n5;
        locals.var_t5_dn6 = assign32970_e43682_d_n6;
        locals.var_t5_dn7 = assign32970_e43682_d_n7;
        locals.var_t5_dn8 = assign32970_e43682_d_n8;
        locals.var_t5_dn9 = assign32970_e43682_d_n9;
        locals.var_t5_dn10 = assign32970_e43682_d_n10;
        locals.var_t5_dn11 = assign32970_e43682_d_n11;
        locals.var_t5_dn12 = assign32970_e43682_d_n12;
        locals.var_t5_dn13 = assign32970_e43682_d_n13;
        locals.var_t5_dn14 = assign32970_e43682_d_n14;

        let (assign32980_e43692, assign32980_e43692_d_n0, assign32980_e43692_d_n2, assign32980_e43692_d_n3, assign32980_e43692_d_n4, assign32980_e43692_d_n5, assign32980_e43692_d_n6, assign32980_e43692_d_n7, assign32980_e43692_d_n8, assign32980_e43692_d_n9, assign32980_e43692_d_n10, assign32980_e43692_d_n11, assign32980_e43692_d_n12, assign32980_e43692_d_n13, assign32980_e43692_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32980_e43686: f64 = (locals.var_t0e / locals.var_t5);
        let assign32980_e43688: f64 = (assign32980_e43686 * locals.var_ids_edge);
        let assign32980_e43690: f64 = (assign32980_e43688 * locals.var_ids_edge);
        (assign32980_e43690, (((((((locals.var_t0e_dn0 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn0)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn0)), (((((((locals.var_t0e_dn2 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn2)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn2)), (((((((locals.var_t0e_dn3 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn3)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn3)), (((((((locals.var_t0e_dn4 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn4)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn4)), (((((((locals.var_t0e_dn5 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn5)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn5)), (((((((locals.var_t0e_dn6 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn6)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn6)), (((((((locals.var_t0e_dn7 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn7)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn7)), (((((((locals.var_t0e_dn8 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn8)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn8)), (((((((locals.var_t0e_dn9 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn9)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn9)), (((((((locals.var_t0e_dn10 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn10)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn10)), (((((((locals.var_t0e_dn11 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn11)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn11)), (((((((locals.var_t0e_dn12 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn12)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn12)), (((((((locals.var_t0e_dn13 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn13)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn13)), (((((((locals.var_t0e_dn14 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn14)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn14)),)
    } else {
        (locals.var_swi, locals.var_swi_dn0, locals.var_swi_dn2, locals.var_swi_dn3, locals.var_swi_dn4, locals.var_swi_dn5, locals.var_swi_dn6, locals.var_swi_dn7, locals.var_swi_dn8, locals.var_swi_dn9, locals.var_swi_dn10, locals.var_swi_dn11, locals.var_swi_dn12, locals.var_swi_dn13, locals.var_swi_dn14,)
    }
};
        locals.var_swi = assign32980_e43692;
        locals.var_swi_dn0 = assign32980_e43692_d_n0;
        locals.var_swi_dn2 = assign32980_e43692_d_n2;
        locals.var_swi_dn3 = assign32980_e43692_d_n3;
        locals.var_swi_dn4 = assign32980_e43692_d_n4;
        locals.var_swi_dn5 = assign32980_e43692_d_n5;
        locals.var_swi_dn6 = assign32980_e43692_d_n6;
        locals.var_swi_dn7 = assign32980_e43692_d_n7;
        locals.var_swi_dn8 = assign32980_e43692_d_n8;
        locals.var_swi_dn9 = assign32980_e43692_d_n9;
        locals.var_swi_dn10 = assign32980_e43692_d_n10;
        locals.var_swi_dn11 = assign32980_e43692_d_n11;
        locals.var_swi_dn12 = assign32980_e43692_d_n12;
        locals.var_swi_dn13 = assign32980_e43692_d_n13;
        locals.var_swi_dn14 = assign32980_e43692_d_n14;

        let (assign32990_e43698, assign32990_e43698_d_n0, assign32990_e43698_d_n2, assign32990_e43698_d_n3, assign32990_e43698_d_n4, assign32990_e43698_d_n5, assign32990_e43698_d_n6, assign32990_e43698_d_n7, assign32990_e43698_d_n8, assign32990_e43698_d_n9, assign32990_e43698_d_n10, assign32990_e43698_d_n11, assign32990_e43698_d_n12, assign32990_e43698_d_n13, assign32990_e43698_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32990_e43696: f64 = (locals.var_swi + locals.var_ssi);
        (assign32990_e43696, (locals.var_swi_dn0 + locals.var_ssi_dn0), (locals.var_swi_dn2 + locals.var_ssi_dn2), (locals.var_swi_dn3 + locals.var_ssi_dn3), (locals.var_swi_dn4 + locals.var_ssi_dn4), (locals.var_swi_dn5 + locals.var_ssi_dn5), (locals.var_swi_dn6 + locals.var_ssi_dn6), (locals.var_swi_dn7 + locals.var_ssi_dn7), (locals.var_swi_dn8 + locals.var_ssi_dn8), (locals.var_swi_dn9 + locals.var_ssi_dn9), (locals.var_swi_dn10 + locals.var_ssi_dn10), (locals.var_swi_dn11 + locals.var_ssi_dn11), (locals.var_swi_dn12 + locals.var_ssi_dn12), (locals.var_swi_dn13 + locals.var_ssi_dn13), (locals.var_swi_dn14 + locals.var_ssi_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32990_e43698;
        locals.var_t6_dn0 = assign32990_e43698_d_n0;
        locals.var_t6_dn2 = assign32990_e43698_d_n2;
        locals.var_t6_dn3 = assign32990_e43698_d_n3;
        locals.var_t6_dn4 = assign32990_e43698_d_n4;
        locals.var_t6_dn5 = assign32990_e43698_d_n5;
        locals.var_t6_dn6 = assign32990_e43698_d_n6;
        locals.var_t6_dn7 = assign32990_e43698_d_n7;
        locals.var_t6_dn8 = assign32990_e43698_d_n8;
        locals.var_t6_dn9 = assign32990_e43698_d_n9;
        locals.var_t6_dn10 = assign32990_e43698_d_n10;
        locals.var_t6_dn11 = assign32990_e43698_d_n11;
        locals.var_t6_dn12 = assign32990_e43698_d_n12;
        locals.var_t6_dn13 = assign32990_e43698_d_n13;
        locals.var_t6_dn14 = assign32990_e43698_d_n14;

        let assign33000_e43701: f64 = if locals.var_t6 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard745 = assign33000_e43701;

        let (assign33010_e43711, assign33010_e43711_d_n0, assign33010_e43711_d_n2, assign33010_e43711_d_n3, assign33010_e43711_d_n4, assign33010_e43711_d_n5, assign33010_e43711_d_n6, assign33010_e43711_d_n7, assign33010_e43711_d_n8, assign33010_e43711_d_n9, assign33010_e43711_d_n10, assign33010_e43711_d_n11, assign33010_e43711_d_n12, assign33010_e43711_d_n13, assign33010_e43711_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard745 != 0.0)) {
        let assign33010_e43707: f64 = (locals.var_ssi * locals.var_swi);
        let assign33010_e43709: f64 = (assign33010_e43707 / locals.var_t6);
        (assign33010_e43709, (((((locals.var_ssi_dn0 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn0)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn2 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn2)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn3 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn3)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn3)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn4 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn4)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn5 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn5)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn6 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn6)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn7 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn7)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn8 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn8)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn9 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn9)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn10 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn10)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn11 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn11)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn12 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn12)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn12)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn13 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn13)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn13)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn14 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn14)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn14)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign33010_e43711;
        locals.var_t7_dn0 = assign33010_e43711_d_n0;
        locals.var_t7_dn2 = assign33010_e43711_d_n2;
        locals.var_t7_dn3 = assign33010_e43711_d_n3;
        locals.var_t7_dn4 = assign33010_e43711_d_n4;
        locals.var_t7_dn5 = assign33010_e43711_d_n5;
        locals.var_t7_dn6 = assign33010_e43711_d_n6;
        locals.var_t7_dn7 = assign33010_e43711_d_n7;
        locals.var_t7_dn8 = assign33010_e43711_d_n8;
        locals.var_t7_dn9 = assign33010_e43711_d_n9;
        locals.var_t7_dn10 = assign33010_e43711_d_n10;
        locals.var_t7_dn11 = assign33010_e43711_d_n11;
        locals.var_t7_dn12 = assign33010_e43711_d_n12;
        locals.var_t7_dn13 = assign33010_e43711_d_n13;
        locals.var_t7_dn14 = assign33010_e43711_d_n14;

        let (assign33020_e43725, assign33020_e43725_d_n0, assign33020_e43725_d_n2, assign33020_e43725_d_n3, assign33020_e43725_d_n4, assign33020_e43725_d_n5, assign33020_e43725_d_n6, assign33020_e43725_d_n7, assign33020_e43725_d_n8, assign33020_e43725_d_n9, assign33020_e43725_d_n10, assign33020_e43725_d_n11, assign33020_e43725_d_n12, assign33020_e43725_d_n13, assign33020_e43725_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard745 != 0.0)) {
        let assign33020_e43719: f64 = (locals.var_qs_edge - locals.var_qdeff_edge);
        let assign33020_e43721: f64 = (assign33020_e43719).powf(p.p1064);
        let assign33020_e43722: f64 = (p.p1063 * assign33020_e43721);
        let assign33020_e43723: f64 = (1.0 + assign33020_e43722);
        (assign33020_e43723, (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn0 - locals.var_qdeff_edge_dn0))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn0 - locals.var_qdeff_edge_dn0) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn2 - locals.var_qdeff_edge_dn2))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn2 - locals.var_qdeff_edge_dn2) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn12 - locals.var_qdeff_edge_dn12))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn12 - locals.var_qdeff_edge_dn12) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn13 - locals.var_qdeff_edge_dn13))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn13 - locals.var_qdeff_edge_dn13) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn14 - locals.var_qdeff_edge_dn14))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn14 - locals.var_qdeff_edge_dn14) / assign33020_e43719))) }),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign33020_e43725;
        locals.var_t8_dn0 = assign33020_e43725_d_n0;
        locals.var_t8_dn2 = assign33020_e43725_d_n2;
        locals.var_t8_dn3 = assign33020_e43725_d_n3;
        locals.var_t8_dn4 = assign33020_e43725_d_n4;
        locals.var_t8_dn5 = assign33020_e43725_d_n5;
        locals.var_t8_dn6 = assign33020_e43725_d_n6;
        locals.var_t8_dn7 = assign33020_e43725_d_n7;
        locals.var_t8_dn8 = assign33020_e43725_d_n8;
        locals.var_t8_dn9 = assign33020_e43725_d_n9;
        locals.var_t8_dn10 = assign33020_e43725_d_n10;
        locals.var_t8_dn11 = assign33020_e43725_d_n11;
        locals.var_t8_dn12 = assign33020_e43725_d_n12;
        locals.var_t8_dn13 = assign33020_e43725_d_n13;
        locals.var_t8_dn14 = assign33020_e43725_d_n14;

        let assign33060_e43756: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard746 = assign33060_e43756;

    }

    pub(super) fn stamp_transient_block_102(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (assign33070_e43764, assign33070_e43764_d_n0, assign33070_e43764_d_n2, assign33070_e43764_d_n3, assign33070_e43764_d_n4, assign33070_e43764_d_n5, assign33070_e43764_d_n6, assign33070_e43764_d_n7, assign33070_e43764_d_n8, assign33070_e43764_d_n9, assign33070_e43764_d_n10, assign33070_e43764_d_n11, assign33070_e43764_d_n12, assign33070_e43764_d_n13, assign33070_e43764_d_n14,) = {
    if (locals.var_guard746 != 0.0) {
        let assign33070_e43760: f64 = (locals.var_devsign * p.p29);
        let assign33070_e43762: f64 = (assign33070_e43760 * locals.var_qsi);
        (assign33070_e43762, (assign33070_e43760 * locals.var_qsi_dn0), (assign33070_e43760 * locals.var_qsi_dn2), (assign33070_e43760 * locals.var_qsi_dn3), (assign33070_e43760 * locals.var_qsi_dn4), (assign33070_e43760 * locals.var_qsi_dn5), (assign33070_e43760 * locals.var_qsi_dn6), (assign33070_e43760 * locals.var_qsi_dn7), (assign33070_e43760 * locals.var_qsi_dn8), (assign33070_e43760 * locals.var_qsi_dn9), (assign33070_e43760 * locals.var_qsi_dn10), (assign33070_e43760 * locals.var_qsi_dn11), (assign33070_e43760 * locals.var_qsi_dn12), (assign33070_e43760 * locals.var_qsi_dn13), (assign33070_e43760 * locals.var_qsi_dn14),)
    } else {
        (locals.var_qsi_1, locals.var_qsi_1_dn0, locals.var_qsi_1_dn2, locals.var_qsi_1_dn3, locals.var_qsi_1_dn4, locals.var_qsi_1_dn5, locals.var_qsi_1_dn6, locals.var_qsi_1_dn7, locals.var_qsi_1_dn8, locals.var_qsi_1_dn9, locals.var_qsi_1_dn10, locals.var_qsi_1_dn11, locals.var_qsi_1_dn12, locals.var_qsi_1_dn13, locals.var_qsi_1_dn14,)
    }
};
        locals.var_qsi_1 = assign33070_e43764;
        locals.var_qsi_1_dn0 = assign33070_e43764_d_n0;
        locals.var_qsi_1_dn2 = assign33070_e43764_d_n2;
        locals.var_qsi_1_dn3 = assign33070_e43764_d_n3;
        locals.var_qsi_1_dn4 = assign33070_e43764_d_n4;
        locals.var_qsi_1_dn5 = assign33070_e43764_d_n5;
        locals.var_qsi_1_dn6 = assign33070_e43764_d_n6;
        locals.var_qsi_1_dn7 = assign33070_e43764_d_n7;
        locals.var_qsi_1_dn8 = assign33070_e43764_d_n8;
        locals.var_qsi_1_dn9 = assign33070_e43764_d_n9;
        locals.var_qsi_1_dn10 = assign33070_e43764_d_n10;
        locals.var_qsi_1_dn11 = assign33070_e43764_d_n11;
        locals.var_qsi_1_dn12 = assign33070_e43764_d_n12;
        locals.var_qsi_1_dn13 = assign33070_e43764_d_n13;
        locals.var_qsi_1_dn14 = assign33070_e43764_d_n14;

        let (assign33080_e43772, assign33080_e43772_d_n0, assign33080_e43772_d_n2, assign33080_e43772_d_n3, assign33080_e43772_d_n4, assign33080_e43772_d_n5, assign33080_e43772_d_n6, assign33080_e43772_d_n7, assign33080_e43772_d_n8, assign33080_e43772_d_n9, assign33080_e43772_d_n10, assign33080_e43772_d_n11, assign33080_e43772_d_n12, assign33080_e43772_d_n13, assign33080_e43772_d_n14,) = {
    if (locals.var_guard746 != 0.0) {
        let assign33080_e43768: f64 = (locals.var_devsign * p.p29);
        let assign33080_e43770: f64 = (assign33080_e43768 * locals.var_qdi);
        (assign33080_e43770, (assign33080_e43768 * locals.var_qdi_dn0), (assign33080_e43768 * locals.var_qdi_dn2), (assign33080_e43768 * locals.var_qdi_dn3), (assign33080_e43768 * locals.var_qdi_dn4), (assign33080_e43768 * locals.var_qdi_dn5), (assign33080_e43768 * locals.var_qdi_dn6), (assign33080_e43768 * locals.var_qdi_dn7), (assign33080_e43768 * locals.var_qdi_dn8), (assign33080_e43768 * locals.var_qdi_dn9), (assign33080_e43768 * locals.var_qdi_dn10), (assign33080_e43768 * locals.var_qdi_dn11), (assign33080_e43768 * locals.var_qdi_dn12), (assign33080_e43768 * locals.var_qdi_dn13), (assign33080_e43768 * locals.var_qdi_dn14),)
    } else {
        (locals.var_qdi_1, locals.var_qdi_1_dn0, locals.var_qdi_1_dn2, locals.var_qdi_1_dn3, locals.var_qdi_1_dn4, locals.var_qdi_1_dn5, locals.var_qdi_1_dn6, locals.var_qdi_1_dn7, locals.var_qdi_1_dn8, locals.var_qdi_1_dn9, locals.var_qdi_1_dn10, locals.var_qdi_1_dn11, locals.var_qdi_1_dn12, locals.var_qdi_1_dn13, locals.var_qdi_1_dn14,)
    }
};
        locals.var_qdi_1 = assign33080_e43772;
        locals.var_qdi_1_dn0 = assign33080_e43772_d_n0;
        locals.var_qdi_1_dn2 = assign33080_e43772_d_n2;
        locals.var_qdi_1_dn3 = assign33080_e43772_d_n3;
        locals.var_qdi_1_dn4 = assign33080_e43772_d_n4;
        locals.var_qdi_1_dn5 = assign33080_e43772_d_n5;
        locals.var_qdi_1_dn6 = assign33080_e43772_d_n6;
        locals.var_qdi_1_dn7 = assign33080_e43772_d_n7;
        locals.var_qdi_1_dn8 = assign33080_e43772_d_n8;
        locals.var_qdi_1_dn9 = assign33080_e43772_d_n9;
        locals.var_qdi_1_dn10 = assign33080_e43772_d_n10;
        locals.var_qdi_1_dn11 = assign33080_e43772_d_n11;
        locals.var_qdi_1_dn12 = assign33080_e43772_d_n12;
        locals.var_qdi_1_dn13 = assign33080_e43772_d_n13;
        locals.var_qdi_1_dn14 = assign33080_e43772_d_n14;

        let (assign33110_e43807, assign33110_e43807_d_n0, assign33110_e43807_d_n2, assign33110_e43807_d_n3, assign33110_e43807_d_n4, assign33110_e43807_d_n5, assign33110_e43807_d_n6, assign33110_e43807_d_n7, assign33110_e43807_d_n8, assign33110_e43807_d_n9, assign33110_e43807_d_n10, assign33110_e43807_d_n11, assign33110_e43807_d_n12, assign33110_e43807_d_n13, assign33110_e43807_d_n14,) = {
    if (locals.var_guard746 == 0.0) {
        let assign33110_e43803: f64 = (locals.var_devsign * p.p29);
        let assign33110_e43805: f64 = (assign33110_e43803 * locals.var_qdi);
        (assign33110_e43805, (assign33110_e43803 * locals.var_qdi_dn0), (assign33110_e43803 * locals.var_qdi_dn2), (assign33110_e43803 * locals.var_qdi_dn3), (assign33110_e43803 * locals.var_qdi_dn4), (assign33110_e43803 * locals.var_qdi_dn5), (assign33110_e43803 * locals.var_qdi_dn6), (assign33110_e43803 * locals.var_qdi_dn7), (assign33110_e43803 * locals.var_qdi_dn8), (assign33110_e43803 * locals.var_qdi_dn9), (assign33110_e43803 * locals.var_qdi_dn10), (assign33110_e43803 * locals.var_qdi_dn11), (assign33110_e43803 * locals.var_qdi_dn12), (assign33110_e43803 * locals.var_qdi_dn13), (assign33110_e43803 * locals.var_qdi_dn14),)
    } else {
        (locals.var_qsi_1, locals.var_qsi_1_dn0, locals.var_qsi_1_dn2, locals.var_qsi_1_dn3, locals.var_qsi_1_dn4, locals.var_qsi_1_dn5, locals.var_qsi_1_dn6, locals.var_qsi_1_dn7, locals.var_qsi_1_dn8, locals.var_qsi_1_dn9, locals.var_qsi_1_dn10, locals.var_qsi_1_dn11, locals.var_qsi_1_dn12, locals.var_qsi_1_dn13, locals.var_qsi_1_dn14,)
    }
};
        locals.var_qsi_1 = assign33110_e43807;
        locals.var_qsi_1_dn0 = assign33110_e43807_d_n0;
        locals.var_qsi_1_dn2 = assign33110_e43807_d_n2;
        locals.var_qsi_1_dn3 = assign33110_e43807_d_n3;
        locals.var_qsi_1_dn4 = assign33110_e43807_d_n4;
        locals.var_qsi_1_dn5 = assign33110_e43807_d_n5;
        locals.var_qsi_1_dn6 = assign33110_e43807_d_n6;
        locals.var_qsi_1_dn7 = assign33110_e43807_d_n7;
        locals.var_qsi_1_dn8 = assign33110_e43807_d_n8;
        locals.var_qsi_1_dn9 = assign33110_e43807_d_n9;
        locals.var_qsi_1_dn10 = assign33110_e43807_d_n10;
        locals.var_qsi_1_dn11 = assign33110_e43807_d_n11;
        locals.var_qsi_1_dn12 = assign33110_e43807_d_n12;
        locals.var_qsi_1_dn13 = assign33110_e43807_d_n13;
        locals.var_qsi_1_dn14 = assign33110_e43807_d_n14;

        let (assign33120_e43816, assign33120_e43816_d_n0, assign33120_e43816_d_n2, assign33120_e43816_d_n3, assign33120_e43816_d_n4, assign33120_e43816_d_n5, assign33120_e43816_d_n6, assign33120_e43816_d_n7, assign33120_e43816_d_n8, assign33120_e43816_d_n9, assign33120_e43816_d_n10, assign33120_e43816_d_n11, assign33120_e43816_d_n12, assign33120_e43816_d_n13, assign33120_e43816_d_n14,) = {
    if (locals.var_guard746 == 0.0) {
        let assign33120_e43812: f64 = (locals.var_devsign * p.p29);
        let assign33120_e43814: f64 = (assign33120_e43812 * locals.var_qsi);
        (assign33120_e43814, (assign33120_e43812 * locals.var_qsi_dn0), (assign33120_e43812 * locals.var_qsi_dn2), (assign33120_e43812 * locals.var_qsi_dn3), (assign33120_e43812 * locals.var_qsi_dn4), (assign33120_e43812 * locals.var_qsi_dn5), (assign33120_e43812 * locals.var_qsi_dn6), (assign33120_e43812 * locals.var_qsi_dn7), (assign33120_e43812 * locals.var_qsi_dn8), (assign33120_e43812 * locals.var_qsi_dn9), (assign33120_e43812 * locals.var_qsi_dn10), (assign33120_e43812 * locals.var_qsi_dn11), (assign33120_e43812 * locals.var_qsi_dn12), (assign33120_e43812 * locals.var_qsi_dn13), (assign33120_e43812 * locals.var_qsi_dn14),)
    } else {
        (locals.var_qdi_1, locals.var_qdi_1_dn0, locals.var_qdi_1_dn2, locals.var_qdi_1_dn3, locals.var_qdi_1_dn4, locals.var_qdi_1_dn5, locals.var_qdi_1_dn6, locals.var_qdi_1_dn7, locals.var_qdi_1_dn8, locals.var_qdi_1_dn9, locals.var_qdi_1_dn10, locals.var_qdi_1_dn11, locals.var_qdi_1_dn12, locals.var_qdi_1_dn13, locals.var_qdi_1_dn14,)
    }
};
        locals.var_qdi_1 = assign33120_e43816;
        locals.var_qdi_1_dn0 = assign33120_e43816_d_n0;
        locals.var_qdi_1_dn2 = assign33120_e43816_d_n2;
        locals.var_qdi_1_dn3 = assign33120_e43816_d_n3;
        locals.var_qdi_1_dn4 = assign33120_e43816_d_n4;
        locals.var_qdi_1_dn5 = assign33120_e43816_d_n5;
        locals.var_qdi_1_dn6 = assign33120_e43816_d_n6;
        locals.var_qdi_1_dn7 = assign33120_e43816_d_n7;
        locals.var_qdi_1_dn8 = assign33120_e43816_d_n8;
        locals.var_qdi_1_dn9 = assign33120_e43816_d_n9;
        locals.var_qdi_1_dn10 = assign33120_e43816_d_n10;
        locals.var_qdi_1_dn11 = assign33120_e43816_d_n11;
        locals.var_qdi_1_dn12 = assign33120_e43816_d_n12;
        locals.var_qdi_1_dn13 = assign33120_e43816_d_n13;
        locals.var_qdi_1_dn14 = assign33120_e43816_d_n14;

        let assign33160_e43858: f64 = if ((p.p1094 == 1.0) && (p.p1095 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard747 = assign33160_e43858;

        let (assign33170_e43864, assign33170_e43864_d_n0, assign33170_e43864_d_n2, assign33170_e43864_d_n3, assign33170_e43864_d_n4, assign33170_e43864_d_n5, assign33170_e43864_d_n6, assign33170_e43864_d_n7, assign33170_e43864_d_n8, assign33170_e43864_d_n9, assign33170_e43864_d_n10, assign33170_e43864_d_n11, assign33170_e43864_d_n12, assign33170_e43864_d_n13, assign33170_e43864_d_n14,) = {
    if (locals.var_guard747 != 0.0) {
        let assign33170_e43862: f64 = (locals.var_qovb + locals.var_qiov);
        (assign33170_e43862, (locals.var_qovb_dn0 + locals.var_qiov_dn0), (locals.var_qovb_dn2 + locals.var_qiov_dn2), (locals.var_qovb_dn3 + locals.var_qiov_dn3), (locals.var_qovb_dn4 + locals.var_qiov_dn4), (locals.var_qovb_dn5 + locals.var_qiov_dn5), (locals.var_qovb_dn6 + locals.var_qiov_dn6), (locals.var_qovb_dn7 + locals.var_qiov_dn7), (locals.var_qovb_dn8 + locals.var_qiov_dn8), (locals.var_qovb_dn9 + locals.var_qiov_dn9), (locals.var_qovb_dn10 + locals.var_qiov_dn10), (locals.var_qovb_dn11 + locals.var_qiov_dn11), (locals.var_qovb_dn12 + locals.var_qiov_dn12), (locals.var_qovb_dn13 + locals.var_qiov_dn13), (locals.var_qovb_dn14 + locals.var_qiov_dn14),)
    } else {
        (locals.var_qovb, locals.var_qovb_dn0, locals.var_qovb_dn2, locals.var_qovb_dn3, locals.var_qovb_dn4, locals.var_qovb_dn5, locals.var_qovb_dn6, locals.var_qovb_dn7, locals.var_qovb_dn8, locals.var_qovb_dn9, locals.var_qovb_dn10, locals.var_qovb_dn11, locals.var_qovb_dn12, locals.var_qovb_dn13, locals.var_qovb_dn14,)
    }
};
        locals.var_qovb = assign33170_e43864;
        locals.var_qovb_dn0 = assign33170_e43864_d_n0;
        locals.var_qovb_dn2 = assign33170_e43864_d_n2;
        locals.var_qovb_dn3 = assign33170_e43864_d_n3;
        locals.var_qovb_dn4 = assign33170_e43864_d_n4;
        locals.var_qovb_dn5 = assign33170_e43864_d_n5;
        locals.var_qovb_dn6 = assign33170_e43864_d_n6;
        locals.var_qovb_dn7 = assign33170_e43864_d_n7;
        locals.var_qovb_dn8 = assign33170_e43864_d_n8;
        locals.var_qovb_dn9 = assign33170_e43864_d_n9;
        locals.var_qovb_dn10 = assign33170_e43864_d_n10;
        locals.var_qovb_dn11 = assign33170_e43864_d_n11;
        locals.var_qovb_dn12 = assign33170_e43864_d_n12;
        locals.var_qovb_dn13 = assign33170_e43864_d_n13;
        locals.var_qovb_dn14 = assign33170_e43864_d_n14;

        let (assign33180_e43870, assign33180_e43870_d_n0, assign33180_e43870_d_n2, assign33180_e43870_d_n3, assign33180_e43870_d_n4, assign33180_e43870_d_n5, assign33180_e43870_d_n6, assign33180_e43870_d_n7, assign33180_e43870_d_n8, assign33180_e43870_d_n9, assign33180_e43870_d_n10, assign33180_e43870_d_n11, assign33180_e43870_d_n12, assign33180_e43870_d_n13, assign33180_e43870_d_n14,) = {
    if (locals.var_guard747 != 0.0) {
        let assign33180_e43868: f64 = (locals.var_qovd + locals.var_qbov);
        (assign33180_e43868, (locals.var_qovd_dn0 + locals.var_qbov_dn0), (locals.var_qovd_dn2 + locals.var_qbov_dn2), (locals.var_qovd_dn3 + locals.var_qbov_dn3), (locals.var_qovd_dn4 + locals.var_qbov_dn4), (locals.var_qovd_dn5 + locals.var_qbov_dn5), (locals.var_qovd_dn6 + locals.var_qbov_dn6), (locals.var_qovd_dn7 + locals.var_qbov_dn7), (locals.var_qovd_dn8 + locals.var_qbov_dn8), (locals.var_qovd_dn9 + locals.var_qbov_dn9), (locals.var_qovd_dn10 + locals.var_qbov_dn10), (locals.var_qovd_dn11 + locals.var_qbov_dn11), (locals.var_qovd_dn12 + locals.var_qbov_dn12), (locals.var_qovd_dn13 + locals.var_qbov_dn13), (locals.var_qovd_dn14 + locals.var_qbov_dn14),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn3, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn13, locals.var_qovd_dn14,)
    }
};
        locals.var_qovd = assign33180_e43870;
        locals.var_qovd_dn0 = assign33180_e43870_d_n0;
        locals.var_qovd_dn2 = assign33180_e43870_d_n2;
        locals.var_qovd_dn3 = assign33180_e43870_d_n3;
        locals.var_qovd_dn4 = assign33180_e43870_d_n4;
        locals.var_qovd_dn5 = assign33180_e43870_d_n5;
        locals.var_qovd_dn6 = assign33180_e43870_d_n6;
        locals.var_qovd_dn7 = assign33180_e43870_d_n7;
        locals.var_qovd_dn8 = assign33180_e43870_d_n8;
        locals.var_qovd_dn9 = assign33180_e43870_d_n9;
        locals.var_qovd_dn10 = assign33180_e43870_d_n10;
        locals.var_qovd_dn11 = assign33180_e43870_d_n11;
        locals.var_qovd_dn12 = assign33180_e43870_d_n12;
        locals.var_qovd_dn13 = assign33180_e43870_d_n13;
        locals.var_qovd_dn14 = assign33180_e43870_d_n14;

        let assign33190_e43873: f64 = if p.p1096 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard748 = assign33190_e43873;

        let (assign33200_e43881, assign33200_e43881_d_n0, assign33200_e43881_d_n2, assign33200_e43881_d_n3, assign33200_e43881_d_n4, assign33200_e43881_d_n5, assign33200_e43881_d_n6, assign33200_e43881_d_n7, assign33200_e43881_d_n8, assign33200_e43881_d_n9, assign33200_e43881_d_n10, assign33200_e43881_d_n11, assign33200_e43881_d_n12, assign33200_e43881_d_n13, assign33200_e43881_d_n14,) = {
    if ((locals.var_guard747 != 0.0) && (locals.var_guard748 != 0.0)) {
        let assign33200_e43879: f64 = (locals.var_qovb + locals.var_qiovs);
        (assign33200_e43879, (locals.var_qovb_dn0 + locals.var_qiovs_dn0), (locals.var_qovb_dn2 + locals.var_qiovs_dn2), (locals.var_qovb_dn3 + locals.var_qiovs_dn3), (locals.var_qovb_dn4 + locals.var_qiovs_dn4), (locals.var_qovb_dn5 + locals.var_qiovs_dn5), (locals.var_qovb_dn6 + locals.var_qiovs_dn6), (locals.var_qovb_dn7 + locals.var_qiovs_dn7), (locals.var_qovb_dn8 + locals.var_qiovs_dn8), (locals.var_qovb_dn9 + locals.var_qiovs_dn9), (locals.var_qovb_dn10 + locals.var_qiovs_dn10), (locals.var_qovb_dn11 + locals.var_qiovs_dn11), (locals.var_qovb_dn12 + locals.var_qiovs_dn12), (locals.var_qovb_dn13 + locals.var_qiovs_dn13), (locals.var_qovb_dn14 + locals.var_qiovs_dn14),)
    } else {
        (locals.var_qovb, locals.var_qovb_dn0, locals.var_qovb_dn2, locals.var_qovb_dn3, locals.var_qovb_dn4, locals.var_qovb_dn5, locals.var_qovb_dn6, locals.var_qovb_dn7, locals.var_qovb_dn8, locals.var_qovb_dn9, locals.var_qovb_dn10, locals.var_qovb_dn11, locals.var_qovb_dn12, locals.var_qovb_dn13, locals.var_qovb_dn14,)
    }
};
        locals.var_qovb = assign33200_e43881;
        locals.var_qovb_dn0 = assign33200_e43881_d_n0;
        locals.var_qovb_dn2 = assign33200_e43881_d_n2;
        locals.var_qovb_dn3 = assign33200_e43881_d_n3;
        locals.var_qovb_dn4 = assign33200_e43881_d_n4;
        locals.var_qovb_dn5 = assign33200_e43881_d_n5;
        locals.var_qovb_dn6 = assign33200_e43881_d_n6;
        locals.var_qovb_dn7 = assign33200_e43881_d_n7;
        locals.var_qovb_dn8 = assign33200_e43881_d_n8;
        locals.var_qovb_dn9 = assign33200_e43881_d_n9;
        locals.var_qovb_dn10 = assign33200_e43881_d_n10;
        locals.var_qovb_dn11 = assign33200_e43881_d_n11;
        locals.var_qovb_dn12 = assign33200_e43881_d_n12;
        locals.var_qovb_dn13 = assign33200_e43881_d_n13;
        locals.var_qovb_dn14 = assign33200_e43881_d_n14;

        let (assign33210_e43889, assign33210_e43889_d_n0, assign33210_e43889_d_n2, assign33210_e43889_d_n3, assign33210_e43889_d_n4, assign33210_e43889_d_n5, assign33210_e43889_d_n6, assign33210_e43889_d_n7, assign33210_e43889_d_n8, assign33210_e43889_d_n9, assign33210_e43889_d_n10, assign33210_e43889_d_n11, assign33210_e43889_d_n12, assign33210_e43889_d_n13, assign33210_e43889_d_n14,) = {
    if ((locals.var_guard747 != 0.0) && (locals.var_guard748 != 0.0)) {
        let assign33210_e43887: f64 = (locals.var_qovs + locals.var_qbovs);
        (assign33210_e43887, (locals.var_qovs_dn0 + locals.var_qbovs_dn0), (locals.var_qovs_dn2 + locals.var_qbovs_dn2), (locals.var_qovs_dn3 + locals.var_qbovs_dn3), (locals.var_qovs_dn4 + locals.var_qbovs_dn4), (locals.var_qovs_dn5 + locals.var_qbovs_dn5), (locals.var_qovs_dn6 + locals.var_qbovs_dn6), (locals.var_qovs_dn7 + locals.var_qbovs_dn7), (locals.var_qovs_dn8 + locals.var_qbovs_dn8), (locals.var_qovs_dn9 + locals.var_qbovs_dn9), (locals.var_qovs_dn10 + locals.var_qbovs_dn10), (locals.var_qovs_dn11 + locals.var_qbovs_dn11), (locals.var_qovs_dn12 + locals.var_qbovs_dn12), (locals.var_qovs_dn13 + locals.var_qbovs_dn13), (locals.var_qovs_dn14 + locals.var_qbovs_dn14),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn3, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn13, locals.var_qovs_dn14,)
    }
};
        locals.var_qovs = assign33210_e43889;
        locals.var_qovs_dn0 = assign33210_e43889_d_n0;
        locals.var_qovs_dn2 = assign33210_e43889_d_n2;
        locals.var_qovs_dn3 = assign33210_e43889_d_n3;
        locals.var_qovs_dn4 = assign33210_e43889_d_n4;
        locals.var_qovs_dn5 = assign33210_e43889_d_n5;
        locals.var_qovs_dn6 = assign33210_e43889_d_n6;
        locals.var_qovs_dn7 = assign33210_e43889_d_n7;
        locals.var_qovs_dn8 = assign33210_e43889_d_n8;
        locals.var_qovs_dn9 = assign33210_e43889_d_n9;
        locals.var_qovs_dn10 = assign33210_e43889_d_n10;
        locals.var_qovs_dn11 = assign33210_e43889_d_n11;
        locals.var_qovs_dn12 = assign33210_e43889_d_n12;
        locals.var_qovs_dn13 = assign33210_e43889_d_n13;
        locals.var_qovs_dn14 = assign33210_e43889_d_n14;

        let assign33230_e43897: f64 = (locals.var_devsign * p.p29);
        let assign33230_e43899: f64 = (assign33230_e43897 * locals.var_qgi);
        locals.var_qgi_1 = assign33230_e43899;
        locals.var_qgi_1_dn0 = (assign33230_e43897 * locals.var_qgi_dn0);
        locals.var_qgi_1_dn2 = (assign33230_e43897 * locals.var_qgi_dn2);
        locals.var_qgi_1_dn3 = (assign33230_e43897 * locals.var_qgi_dn3);
        locals.var_qgi_1_dn4 = (assign33230_e43897 * locals.var_qgi_dn4);
        locals.var_qgi_1_dn5 = (assign33230_e43897 * locals.var_qgi_dn5);
        locals.var_qgi_1_dn6 = (assign33230_e43897 * locals.var_qgi_dn6);
        locals.var_qgi_1_dn7 = (assign33230_e43897 * locals.var_qgi_dn7);
        locals.var_qgi_1_dn8 = (assign33230_e43897 * locals.var_qgi_dn8);
        locals.var_qgi_1_dn9 = (assign33230_e43897 * locals.var_qgi_dn9);
        locals.var_qgi_1_dn10 = (assign33230_e43897 * locals.var_qgi_dn10);
        locals.var_qgi_1_dn11 = (assign33230_e43897 * locals.var_qgi_dn11);
        locals.var_qgi_1_dn12 = (assign33230_e43897 * locals.var_qgi_dn12);
        locals.var_qgi_1_dn13 = (assign33230_e43897 * locals.var_qgi_dn13);
        locals.var_qgi_1_dn14 = (assign33230_e43897 * locals.var_qgi_dn14);

        let assign33870_e44253: f64 = if ((p.p42 != 2.0) && (locals.var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard754 = assign33870_e44253;

        let (assign33880_e44259, assign33880_e44259_d_n0, assign33880_e44259_d_n2, assign33880_e44259_d_n3, assign33880_e44259_d_n4, assign33880_e44259_d_n5, assign33880_e44259_d_n6, assign33880_e44259_d_n7, assign33880_e44259_d_n8, assign33880_e44259_d_n9, assign33880_e44259_d_n10, assign33880_e44259_d_n11, assign33880_e44259_d_n12, assign33880_e44259_d_n13, assign33880_e44259_d_n14,) = {
    if (locals.var_guard754 != 0.0) {
        let assign33880_e44257: f64 = (1.0 / locals.var_rdrain);
        (assign33880_e44257, (-(locals.var_rdrain_dn0 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn2 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn3 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn4 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn5 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn6 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn7 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn8 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn9 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn10 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn11 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn12 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn13 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn14 / (locals.var_rdrain * locals.var_rdrain))),)
    } else {
        (locals.var_gdpr, locals.var_gdpr_dn0, locals.var_gdpr_dn2, locals.var_gdpr_dn3, locals.var_gdpr_dn4, locals.var_gdpr_dn5, locals.var_gdpr_dn6, locals.var_gdpr_dn7, locals.var_gdpr_dn8, locals.var_gdpr_dn9, locals.var_gdpr_dn10, locals.var_gdpr_dn11, locals.var_gdpr_dn12, locals.var_gdpr_dn13, locals.var_gdpr_dn14,)
    }
};
        locals.var_gdpr = assign33880_e44259;
        locals.var_gdpr_dn0 = assign33880_e44259_d_n0;
        locals.var_gdpr_dn2 = assign33880_e44259_d_n2;
        locals.var_gdpr_dn3 = assign33880_e44259_d_n3;
        locals.var_gdpr_dn4 = assign33880_e44259_d_n4;
        locals.var_gdpr_dn5 = assign33880_e44259_d_n5;
        locals.var_gdpr_dn6 = assign33880_e44259_d_n6;
        locals.var_gdpr_dn7 = assign33880_e44259_d_n7;
        locals.var_gdpr_dn8 = assign33880_e44259_d_n8;
        locals.var_gdpr_dn9 = assign33880_e44259_d_n9;
        locals.var_gdpr_dn10 = assign33880_e44259_d_n10;
        locals.var_gdpr_dn11 = assign33880_e44259_d_n11;
        locals.var_gdpr_dn12 = assign33880_e44259_d_n12;
        locals.var_gdpr_dn13 = assign33880_e44259_d_n13;
        locals.var_gdpr_dn14 = assign33880_e44259_d_n14;

        let assign33890_e44270: f64 = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard755 = assign33890_e44270;

        let (assign33900_e44278, assign33900_e44278_d_n0, assign33900_e44278_d_n2, assign33900_e44278_d_n3, assign33900_e44278_d_n4, assign33900_e44278_d_n5, assign33900_e44278_d_n6, assign33900_e44278_d_n7, assign33900_e44278_d_n8, assign33900_e44278_d_n9, assign33900_e44278_d_n10, assign33900_e44278_d_n11, assign33900_e44278_d_n12, assign33900_e44278_d_n13, assign33900_e44278_d_n14,) = {
    if ((locals.var_guard754 != 0.0) && (locals.var_guard755 != 0.0)) {
        let assign33900_e44276: f64 = (1.0 / locals.var_rdrift_d);
        (assign33900_e44276, (-(locals.var_rdrift_d_dn0 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn2 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn3 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn4 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn5 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn6 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn7 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn8 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn9 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn10 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn11 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn12 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn13 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn14 / (locals.var_rdrift_d * locals.var_rdrift_d))),)
    } else {
        (locals.var_gdrift_d, locals.var_gdrift_d_dn0, locals.var_gdrift_d_dn2, locals.var_gdrift_d_dn3, locals.var_gdrift_d_dn4, locals.var_gdrift_d_dn5, locals.var_gdrift_d_dn6, locals.var_gdrift_d_dn7, locals.var_gdrift_d_dn8, locals.var_gdrift_d_dn9, locals.var_gdrift_d_dn10, locals.var_gdrift_d_dn11, locals.var_gdrift_d_dn12, locals.var_gdrift_d_dn13, locals.var_gdrift_d_dn14,)
    }
};
        locals.var_gdrift_d = assign33900_e44278;
        locals.var_gdrift_d_dn0 = assign33900_e44278_d_n0;
        locals.var_gdrift_d_dn2 = assign33900_e44278_d_n2;
        locals.var_gdrift_d_dn3 = assign33900_e44278_d_n3;
        locals.var_gdrift_d_dn4 = assign33900_e44278_d_n4;
        locals.var_gdrift_d_dn5 = assign33900_e44278_d_n5;
        locals.var_gdrift_d_dn6 = assign33900_e44278_d_n6;
        locals.var_gdrift_d_dn7 = assign33900_e44278_d_n7;
        locals.var_gdrift_d_dn8 = assign33900_e44278_d_n8;
        locals.var_gdrift_d_dn9 = assign33900_e44278_d_n9;
        locals.var_gdrift_d_dn10 = assign33900_e44278_d_n10;
        locals.var_gdrift_d_dn11 = assign33900_e44278_d_n11;
        locals.var_gdrift_d_dn12 = assign33900_e44278_d_n12;
        locals.var_gdrift_d_dn13 = assign33900_e44278_d_n13;
        locals.var_gdrift_d_dn14 = assign33900_e44278_d_n14;

        let assign33910_e44285: f64 = if ((p.p42 != 2.0) && (locals.var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard756 = assign33910_e44285;

        let (assign33920_e44291, assign33920_e44291_d_n0, assign33920_e44291_d_n2, assign33920_e44291_d_n3, assign33920_e44291_d_n4, assign33920_e44291_d_n5, assign33920_e44291_d_n6, assign33920_e44291_d_n7, assign33920_e44291_d_n8, assign33920_e44291_d_n9, assign33920_e44291_d_n10, assign33920_e44291_d_n11, assign33920_e44291_d_n12, assign33920_e44291_d_n13, assign33920_e44291_d_n14,) = {
    if (locals.var_guard756 != 0.0) {
        let assign33920_e44289: f64 = (1.0 / locals.var_rsource);
        (assign33920_e44289, (-(locals.var_rsource_dn0 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn2 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn3 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn4 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn5 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn6 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn7 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn8 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn9 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn10 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn11 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn12 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn13 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn14 / (locals.var_rsource * locals.var_rsource))),)
    } else {
        (locals.var_gspr, locals.var_gspr_dn0, locals.var_gspr_dn2, locals.var_gspr_dn3, locals.var_gspr_dn4, locals.var_gspr_dn5, locals.var_gspr_dn6, locals.var_gspr_dn7, locals.var_gspr_dn8, locals.var_gspr_dn9, locals.var_gspr_dn10, locals.var_gspr_dn11, locals.var_gspr_dn12, locals.var_gspr_dn13, locals.var_gspr_dn14,)
    }
};
        locals.var_gspr = assign33920_e44291;
        locals.var_gspr_dn0 = assign33920_e44291_d_n0;
        locals.var_gspr_dn2 = assign33920_e44291_d_n2;
        locals.var_gspr_dn3 = assign33920_e44291_d_n3;
        locals.var_gspr_dn4 = assign33920_e44291_d_n4;
        locals.var_gspr_dn5 = assign33920_e44291_d_n5;
        locals.var_gspr_dn6 = assign33920_e44291_d_n6;
        locals.var_gspr_dn7 = assign33920_e44291_d_n7;
        locals.var_gspr_dn8 = assign33920_e44291_d_n8;
        locals.var_gspr_dn9 = assign33920_e44291_d_n9;
        locals.var_gspr_dn10 = assign33920_e44291_d_n10;
        locals.var_gspr_dn11 = assign33920_e44291_d_n11;
        locals.var_gspr_dn12 = assign33920_e44291_d_n12;
        locals.var_gspr_dn13 = assign33920_e44291_d_n13;
        locals.var_gspr_dn14 = assign33920_e44291_d_n14;

        let assign33930_e44302: f64 = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard757 = assign33930_e44302;

        let (assign33940_e44310, assign33940_e44310_d_n0, assign33940_e44310_d_n2, assign33940_e44310_d_n3, assign33940_e44310_d_n4, assign33940_e44310_d_n5, assign33940_e44310_d_n6, assign33940_e44310_d_n7, assign33940_e44310_d_n8, assign33940_e44310_d_n9, assign33940_e44310_d_n10, assign33940_e44310_d_n11, assign33940_e44310_d_n12, assign33940_e44310_d_n13, assign33940_e44310_d_n14,) = {
    if ((locals.var_guard756 != 0.0) && (locals.var_guard757 != 0.0)) {
        let assign33940_e44308: f64 = (1.0 / locals.var_rdrift_s);
        (assign33940_e44308, (-(locals.var_rdrift_s_dn0 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn2 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn3 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn4 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn5 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn6 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn7 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn8 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn9 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn10 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn11 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn12 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn13 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn14 / (locals.var_rdrift_s * locals.var_rdrift_s))),)
    } else {
        (locals.var_gdrift_s, locals.var_gdrift_s_dn0, locals.var_gdrift_s_dn2, locals.var_gdrift_s_dn3, locals.var_gdrift_s_dn4, locals.var_gdrift_s_dn5, locals.var_gdrift_s_dn6, locals.var_gdrift_s_dn7, locals.var_gdrift_s_dn8, locals.var_gdrift_s_dn9, locals.var_gdrift_s_dn10, locals.var_gdrift_s_dn11, locals.var_gdrift_s_dn12, locals.var_gdrift_s_dn13, locals.var_gdrift_s_dn14,)
    }
};
        locals.var_gdrift_s = assign33940_e44310;
        locals.var_gdrift_s_dn0 = assign33940_e44310_d_n0;
        locals.var_gdrift_s_dn2 = assign33940_e44310_d_n2;
        locals.var_gdrift_s_dn3 = assign33940_e44310_d_n3;
        locals.var_gdrift_s_dn4 = assign33940_e44310_d_n4;
        locals.var_gdrift_s_dn5 = assign33940_e44310_d_n5;
        locals.var_gdrift_s_dn6 = assign33940_e44310_d_n6;
        locals.var_gdrift_s_dn7 = assign33940_e44310_d_n7;
        locals.var_gdrift_s_dn8 = assign33940_e44310_d_n8;
        locals.var_gdrift_s_dn9 = assign33940_e44310_d_n9;
        locals.var_gdrift_s_dn10 = assign33940_e44310_d_n10;
        locals.var_gdrift_s_dn11 = assign33940_e44310_d_n11;
        locals.var_gdrift_s_dn12 = assign33940_e44310_d_n12;
        locals.var_gdrift_s_dn13 = assign33940_e44310_d_n13;
        locals.var_gdrift_s_dn14 = assign33940_e44310_d_n14;

        let assign34020_e44360: f64 = if ((p.p49 != 0.0) && (p.p909 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard763 = assign34020_e44360;

        let (assign34030_e44370, assign34030_e44370_d_n0, assign34030_e44370_d_n2, assign34030_e44370_d_n3, assign34030_e44370_d_n4, assign34030_e44370_d_n5, assign34030_e44370_d_n6, assign34030_e44370_d_n7, assign34030_e44370_d_n8, assign34030_e44370_d_n9, assign34030_e44370_d_n10, assign34030_e44370_d_n11, assign34030_e44370_d_n12, assign34030_e44370_d_n13, assign34030_e44370_d_n14,) = {
    if (locals.var_guard763 != 0.0) {
        let assign34030_e44364: f64 = (locals.var_devsign * locals.var_sigvds);
        let assign34030_e44366: f64 = (assign34030_e44364 * locals.var_ids);
        let assign34030_e44368: f64 = (assign34030_e44366 * (nv5 - nv7));
        (assign34030_e44368, ((assign34030_e44364 * locals.var_ids_dn0) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn2) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn3) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn4) * (nv5 - nv7)), (((assign34030_e44364 * locals.var_ids_dn5) * (nv5 - nv7)) + assign34030_e44366), ((assign34030_e44364 * locals.var_ids_dn6) * (nv5 - nv7)), (((assign34030_e44364 * locals.var_ids_dn7) * (nv5 - nv7)) + (-assign34030_e44366)), ((assign34030_e44364 * locals.var_ids_dn8) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn9) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn10) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn11) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn12) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn13) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn14) * (nv5 - nv7)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34030_e44370;
        locals.var_pdiss_dn0 = assign34030_e44370_d_n0;
        locals.var_pdiss_dn2 = assign34030_e44370_d_n2;
        locals.var_pdiss_dn3 = assign34030_e44370_d_n3;
        locals.var_pdiss_dn4 = assign34030_e44370_d_n4;
        locals.var_pdiss_dn5 = assign34030_e44370_d_n5;
        locals.var_pdiss_dn6 = assign34030_e44370_d_n6;
        locals.var_pdiss_dn7 = assign34030_e44370_d_n7;
        locals.var_pdiss_dn8 = assign34030_e44370_d_n8;
        locals.var_pdiss_dn9 = assign34030_e44370_d_n9;
        locals.var_pdiss_dn10 = assign34030_e44370_d_n10;
        locals.var_pdiss_dn11 = assign34030_e44370_d_n11;
        locals.var_pdiss_dn12 = assign34030_e44370_d_n12;
        locals.var_pdiss_dn13 = assign34030_e44370_d_n13;
        locals.var_pdiss_dn14 = assign34030_e44370_d_n14;

        let assign34040_e44377: f64 = if ((p.p42 != 2.0) && (locals.var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard764 = assign34040_e44377;

        let assign34050_e44388: f64 = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard765 = assign34050_e44388;

        let (assign34060_e44408, assign34060_e44408_d_n0, assign34060_e44408_d_n2, assign34060_e44408_d_n3, assign34060_e44408_d_n4, assign34060_e44408_d_n5, assign34060_e44408_d_n6, assign34060_e44408_d_n7, assign34060_e44408_d_n8, assign34060_e44408_d_n9, assign34060_e44408_d_n10, assign34060_e44408_d_n11, assign34060_e44408_d_n12, assign34060_e44408_d_n13, assign34060_e44408_d_n14,) = {
    if (((locals.var_guard763 != 0.0) && (locals.var_guard764 != 0.0)) && (locals.var_guard765 != 0.0)) {
        let assign34060_e44397: f64 = ((nv0 - nv6) * (nv0 - nv6));
        let assign34060_e44399: f64 = (assign34060_e44397 * locals.var_gdpr);
        let assign34060_e44400: f64 = (locals.var_pdiss + assign34060_e44399);
        let assign34060_e44403: f64 = ((nv6 - nv5) * (nv6 - nv5));
        let assign34060_e44405: f64 = (assign34060_e44403 * locals.var_gdrift_d);
        let assign34060_e44406: f64 = (assign34060_e44400 + assign34060_e44405);
        (assign34060_e44406, ((locals.var_pdiss_dn0 + ((((nv0 - nv6) + (nv0 - nv6)) * locals.var_gdpr) + (assign34060_e44397 * locals.var_gdpr_dn0))) + (assign34060_e44403 * locals.var_gdrift_d_dn0)), ((locals.var_pdiss_dn2 + (assign34060_e44397 * locals.var_gdpr_dn2)) + (assign34060_e44403 * locals.var_gdrift_d_dn2)), ((locals.var_pdiss_dn3 + (assign34060_e44397 * locals.var_gdpr_dn3)) + (assign34060_e44403 * locals.var_gdrift_d_dn3)), ((locals.var_pdiss_dn4 + (assign34060_e44397 * locals.var_gdpr_dn4)) + (assign34060_e44403 * locals.var_gdrift_d_dn4)), ((locals.var_pdiss_dn5 + (assign34060_e44397 * locals.var_gdpr_dn5)) + ((((-(nv6 - nv5)) + (-(nv6 - nv5))) * locals.var_gdrift_d) + (assign34060_e44403 * locals.var_gdrift_d_dn5))), ((locals.var_pdiss_dn6 + ((((-(nv0 - nv6)) + (-(nv0 - nv6))) * locals.var_gdpr) + (assign34060_e44397 * locals.var_gdpr_dn6))) + ((((nv6 - nv5) + (nv6 - nv5)) * locals.var_gdrift_d) + (assign34060_e44403 * locals.var_gdrift_d_dn6))), ((locals.var_pdiss_dn7 + (assign34060_e44397 * locals.var_gdpr_dn7)) + (assign34060_e44403 * locals.var_gdrift_d_dn7)), ((locals.var_pdiss_dn8 + (assign34060_e44397 * locals.var_gdpr_dn8)) + (assign34060_e44403 * locals.var_gdrift_d_dn8)), ((locals.var_pdiss_dn9 + (assign34060_e44397 * locals.var_gdpr_dn9)) + (assign34060_e44403 * locals.var_gdrift_d_dn9)), ((locals.var_pdiss_dn10 + (assign34060_e44397 * locals.var_gdpr_dn10)) + (assign34060_e44403 * locals.var_gdrift_d_dn10)), ((locals.var_pdiss_dn11 + (assign34060_e44397 * locals.var_gdpr_dn11)) + (assign34060_e44403 * locals.var_gdrift_d_dn11)), ((locals.var_pdiss_dn12 + (assign34060_e44397 * locals.var_gdpr_dn12)) + (assign34060_e44403 * locals.var_gdrift_d_dn12)), ((locals.var_pdiss_dn13 + (assign34060_e44397 * locals.var_gdpr_dn13)) + (assign34060_e44403 * locals.var_gdrift_d_dn13)), ((locals.var_pdiss_dn14 + (assign34060_e44397 * locals.var_gdpr_dn14)) + (assign34060_e44403 * locals.var_gdrift_d_dn14)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34060_e44408;
        locals.var_pdiss_dn0 = assign34060_e44408_d_n0;
        locals.var_pdiss_dn2 = assign34060_e44408_d_n2;
        locals.var_pdiss_dn3 = assign34060_e44408_d_n3;
        locals.var_pdiss_dn4 = assign34060_e44408_d_n4;
        locals.var_pdiss_dn5 = assign34060_e44408_d_n5;
        locals.var_pdiss_dn6 = assign34060_e44408_d_n6;
        locals.var_pdiss_dn7 = assign34060_e44408_d_n7;
        locals.var_pdiss_dn8 = assign34060_e44408_d_n8;
        locals.var_pdiss_dn9 = assign34060_e44408_d_n9;
        locals.var_pdiss_dn10 = assign34060_e44408_d_n10;
        locals.var_pdiss_dn11 = assign34060_e44408_d_n11;
        locals.var_pdiss_dn12 = assign34060_e44408_d_n12;
        locals.var_pdiss_dn13 = assign34060_e44408_d_n13;
        locals.var_pdiss_dn14 = assign34060_e44408_d_n14;

        let (assign34070_e44423, assign34070_e44423_d_n0, assign34070_e44423_d_n2, assign34070_e44423_d_n3, assign34070_e44423_d_n4, assign34070_e44423_d_n5, assign34070_e44423_d_n6, assign34070_e44423_d_n7, assign34070_e44423_d_n8, assign34070_e44423_d_n9, assign34070_e44423_d_n10, assign34070_e44423_d_n11, assign34070_e44423_d_n12, assign34070_e44423_d_n13, assign34070_e44423_d_n14,) = {
    if (((locals.var_guard763 != 0.0) && (locals.var_guard764 != 0.0)) && (locals.var_guard765 == 0.0)) {
        let assign34070_e44418: f64 = ((nv0 - nv6) * (nv0 - nv6));
        let assign34070_e44420: f64 = (assign34070_e44418 * locals.var_gdpr);
        let assign34070_e44421: f64 = (locals.var_pdiss + assign34070_e44420);
        (assign34070_e44421, (locals.var_pdiss_dn0 + ((((nv0 - nv6) + (nv0 - nv6)) * locals.var_gdpr) + (assign34070_e44418 * locals.var_gdpr_dn0))), (locals.var_pdiss_dn2 + (assign34070_e44418 * locals.var_gdpr_dn2)), (locals.var_pdiss_dn3 + (assign34070_e44418 * locals.var_gdpr_dn3)), (locals.var_pdiss_dn4 + (assign34070_e44418 * locals.var_gdpr_dn4)), (locals.var_pdiss_dn5 + (assign34070_e44418 * locals.var_gdpr_dn5)), (locals.var_pdiss_dn6 + ((((-(nv0 - nv6)) + (-(nv0 - nv6))) * locals.var_gdpr) + (assign34070_e44418 * locals.var_gdpr_dn6))), (locals.var_pdiss_dn7 + (assign34070_e44418 * locals.var_gdpr_dn7)), (locals.var_pdiss_dn8 + (assign34070_e44418 * locals.var_gdpr_dn8)), (locals.var_pdiss_dn9 + (assign34070_e44418 * locals.var_gdpr_dn9)), (locals.var_pdiss_dn10 + (assign34070_e44418 * locals.var_gdpr_dn10)), (locals.var_pdiss_dn11 + (assign34070_e44418 * locals.var_gdpr_dn11)), (locals.var_pdiss_dn12 + (assign34070_e44418 * locals.var_gdpr_dn12)), (locals.var_pdiss_dn13 + (assign34070_e44418 * locals.var_gdpr_dn13)), (locals.var_pdiss_dn14 + (assign34070_e44418 * locals.var_gdpr_dn14)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34070_e44423;
        locals.var_pdiss_dn0 = assign34070_e44423_d_n0;
        locals.var_pdiss_dn2 = assign34070_e44423_d_n2;
        locals.var_pdiss_dn3 = assign34070_e44423_d_n3;
        locals.var_pdiss_dn4 = assign34070_e44423_d_n4;
        locals.var_pdiss_dn5 = assign34070_e44423_d_n5;
        locals.var_pdiss_dn6 = assign34070_e44423_d_n6;
        locals.var_pdiss_dn7 = assign34070_e44423_d_n7;
        locals.var_pdiss_dn8 = assign34070_e44423_d_n8;
        locals.var_pdiss_dn9 = assign34070_e44423_d_n9;
        locals.var_pdiss_dn10 = assign34070_e44423_d_n10;
        locals.var_pdiss_dn11 = assign34070_e44423_d_n11;
        locals.var_pdiss_dn12 = assign34070_e44423_d_n12;
        locals.var_pdiss_dn13 = assign34070_e44423_d_n13;
        locals.var_pdiss_dn14 = assign34070_e44423_d_n14;

        let assign34080_e44430: f64 = if ((p.p42 != 2.0) && (locals.var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard766 = assign34080_e44430;

        let assign34090_e44441: f64 = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard767 = assign34090_e44441;

        let (assign34100_e44461, assign34100_e44461_d_n0, assign34100_e44461_d_n2, assign34100_e44461_d_n3, assign34100_e44461_d_n4, assign34100_e44461_d_n5, assign34100_e44461_d_n6, assign34100_e44461_d_n7, assign34100_e44461_d_n8, assign34100_e44461_d_n9, assign34100_e44461_d_n10, assign34100_e44461_d_n11, assign34100_e44461_d_n12, assign34100_e44461_d_n13, assign34100_e44461_d_n14,) = {
    if (((locals.var_guard763 != 0.0) && (locals.var_guard766 != 0.0)) && (locals.var_guard767 != 0.0)) {
        let assign34100_e44450: f64 = ((nv2 - nv8) * (nv2 - nv8));
        let assign34100_e44452: f64 = (assign34100_e44450 * locals.var_gspr);
        let assign34100_e44453: f64 = (locals.var_pdiss + assign34100_e44452);
        let assign34100_e44456: f64 = ((nv8 - nv7) * (nv8 - nv7));
        let assign34100_e44458: f64 = (assign34100_e44456 * locals.var_gdrift_s);
        let assign34100_e44459: f64 = (assign34100_e44453 + assign34100_e44458);
        (assign34100_e44459, ((locals.var_pdiss_dn0 + (assign34100_e44450 * locals.var_gspr_dn0)) + (assign34100_e44456 * locals.var_gdrift_s_dn0)), ((locals.var_pdiss_dn2 + ((((nv2 - nv8) + (nv2 - nv8)) * locals.var_gspr) + (assign34100_e44450 * locals.var_gspr_dn2))) + (assign34100_e44456 * locals.var_gdrift_s_dn2)), ((locals.var_pdiss_dn3 + (assign34100_e44450 * locals.var_gspr_dn3)) + (assign34100_e44456 * locals.var_gdrift_s_dn3)), ((locals.var_pdiss_dn4 + (assign34100_e44450 * locals.var_gspr_dn4)) + (assign34100_e44456 * locals.var_gdrift_s_dn4)), ((locals.var_pdiss_dn5 + (assign34100_e44450 * locals.var_gspr_dn5)) + (assign34100_e44456 * locals.var_gdrift_s_dn5)), ((locals.var_pdiss_dn6 + (assign34100_e44450 * locals.var_gspr_dn6)) + (assign34100_e44456 * locals.var_gdrift_s_dn6)), ((locals.var_pdiss_dn7 + (assign34100_e44450 * locals.var_gspr_dn7)) + ((((-(nv8 - nv7)) + (-(nv8 - nv7))) * locals.var_gdrift_s) + (assign34100_e44456 * locals.var_gdrift_s_dn7))), ((locals.var_pdiss_dn8 + ((((-(nv2 - nv8)) + (-(nv2 - nv8))) * locals.var_gspr) + (assign34100_e44450 * locals.var_gspr_dn8))) + ((((nv8 - nv7) + (nv8 - nv7)) * locals.var_gdrift_s) + (assign34100_e44456 * locals.var_gdrift_s_dn8))), ((locals.var_pdiss_dn9 + (assign34100_e44450 * locals.var_gspr_dn9)) + (assign34100_e44456 * locals.var_gdrift_s_dn9)), ((locals.var_pdiss_dn10 + (assign34100_e44450 * locals.var_gspr_dn10)) + (assign34100_e44456 * locals.var_gdrift_s_dn10)), ((locals.var_pdiss_dn11 + (assign34100_e44450 * locals.var_gspr_dn11)) + (assign34100_e44456 * locals.var_gdrift_s_dn11)), ((locals.var_pdiss_dn12 + (assign34100_e44450 * locals.var_gspr_dn12)) + (assign34100_e44456 * locals.var_gdrift_s_dn12)), ((locals.var_pdiss_dn13 + (assign34100_e44450 * locals.var_gspr_dn13)) + (assign34100_e44456 * locals.var_gdrift_s_dn13)), ((locals.var_pdiss_dn14 + (assign34100_e44450 * locals.var_gspr_dn14)) + (assign34100_e44456 * locals.var_gdrift_s_dn14)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34100_e44461;
        locals.var_pdiss_dn0 = assign34100_e44461_d_n0;
        locals.var_pdiss_dn2 = assign34100_e44461_d_n2;
        locals.var_pdiss_dn3 = assign34100_e44461_d_n3;
        locals.var_pdiss_dn4 = assign34100_e44461_d_n4;
        locals.var_pdiss_dn5 = assign34100_e44461_d_n5;
        locals.var_pdiss_dn6 = assign34100_e44461_d_n6;
        locals.var_pdiss_dn7 = assign34100_e44461_d_n7;
        locals.var_pdiss_dn8 = assign34100_e44461_d_n8;
        locals.var_pdiss_dn9 = assign34100_e44461_d_n9;
        locals.var_pdiss_dn10 = assign34100_e44461_d_n10;
        locals.var_pdiss_dn11 = assign34100_e44461_d_n11;
        locals.var_pdiss_dn12 = assign34100_e44461_d_n12;
        locals.var_pdiss_dn13 = assign34100_e44461_d_n13;
        locals.var_pdiss_dn14 = assign34100_e44461_d_n14;

        let (assign34110_e44476, assign34110_e44476_d_n0, assign34110_e44476_d_n2, assign34110_e44476_d_n3, assign34110_e44476_d_n4, assign34110_e44476_d_n5, assign34110_e44476_d_n6, assign34110_e44476_d_n7, assign34110_e44476_d_n8, assign34110_e44476_d_n9, assign34110_e44476_d_n10, assign34110_e44476_d_n11, assign34110_e44476_d_n12, assign34110_e44476_d_n13, assign34110_e44476_d_n14,) = {
    if (((locals.var_guard763 != 0.0) && (locals.var_guard766 != 0.0)) && (locals.var_guard767 == 0.0)) {
        let assign34110_e44471: f64 = ((nv2 - nv8) * (nv2 - nv8));
        let assign34110_e44473: f64 = (assign34110_e44471 * locals.var_gspr);
        let assign34110_e44474: f64 = (locals.var_pdiss + assign34110_e44473);
        (assign34110_e44474, (locals.var_pdiss_dn0 + (assign34110_e44471 * locals.var_gspr_dn0)), (locals.var_pdiss_dn2 + ((((nv2 - nv8) + (nv2 - nv8)) * locals.var_gspr) + (assign34110_e44471 * locals.var_gspr_dn2))), (locals.var_pdiss_dn3 + (assign34110_e44471 * locals.var_gspr_dn3)), (locals.var_pdiss_dn4 + (assign34110_e44471 * locals.var_gspr_dn4)), (locals.var_pdiss_dn5 + (assign34110_e44471 * locals.var_gspr_dn5)), (locals.var_pdiss_dn6 + (assign34110_e44471 * locals.var_gspr_dn6)), (locals.var_pdiss_dn7 + (assign34110_e44471 * locals.var_gspr_dn7)), (locals.var_pdiss_dn8 + ((((-(nv2 - nv8)) + (-(nv2 - nv8))) * locals.var_gspr) + (assign34110_e44471 * locals.var_gspr_dn8))), (locals.var_pdiss_dn9 + (assign34110_e44471 * locals.var_gspr_dn9)), (locals.var_pdiss_dn10 + (assign34110_e44471 * locals.var_gspr_dn10)), (locals.var_pdiss_dn11 + (assign34110_e44471 * locals.var_gspr_dn11)), (locals.var_pdiss_dn12 + (assign34110_e44471 * locals.var_gspr_dn12)), (locals.var_pdiss_dn13 + (assign34110_e44471 * locals.var_gspr_dn13)), (locals.var_pdiss_dn14 + (assign34110_e44471 * locals.var_gspr_dn14)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34110_e44476;
        locals.var_pdiss_dn0 = assign34110_e44476_d_n0;
        locals.var_pdiss_dn2 = assign34110_e44476_d_n2;
        locals.var_pdiss_dn3 = assign34110_e44476_d_n3;
        locals.var_pdiss_dn4 = assign34110_e44476_d_n4;
        locals.var_pdiss_dn5 = assign34110_e44476_d_n5;
        locals.var_pdiss_dn6 = assign34110_e44476_d_n6;
        locals.var_pdiss_dn7 = assign34110_e44476_d_n7;
        locals.var_pdiss_dn8 = assign34110_e44476_d_n8;
        locals.var_pdiss_dn9 = assign34110_e44476_d_n9;
        locals.var_pdiss_dn10 = assign34110_e44476_d_n10;
        locals.var_pdiss_dn11 = assign34110_e44476_d_n11;
        locals.var_pdiss_dn12 = assign34110_e44476_d_n12;
        locals.var_pdiss_dn13 = assign34110_e44476_d_n13;
        locals.var_pdiss_dn14 = assign34110_e44476_d_n14;

        let assign34130_e44482: f64 = if p.p8 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard769 = assign34130_e44482;

        let assign34140_e44485: f64 = if p.p1097 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard770 = assign34140_e44485;

        let assign34160_e44499: f64 = if ((p.p8 != 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard772 = assign34160_e44499;

    }

    pub(super) fn stamp_reactive_block_0(
        locals: &mut StampLocals,
    ) {
        locals.var_cdscdr_i = 0.0;
        locals.var_cdscdr_i_dn0 = 0.0;
        locals.var_cdscdr_i_dn2 = 0.0;
        locals.var_cdscdr_i_dn3 = 0.0;
        locals.var_cdscdr_i_dn4 = 0.0;
        locals.var_cdscdr_i_dn5 = 0.0;
        locals.var_cdscdr_i_dn6 = 0.0;
        locals.var_cdscdr_i_dn7 = 0.0;
        locals.var_cdscdr_i_dn8 = 0.0;
        locals.var_cdscdr_i_dn9 = 0.0;
        locals.var_cdscdr_i_dn10 = 0.0;
        locals.var_cdscdr_i_dn11 = 0.0;
        locals.var_cdscdr_i_dn12 = 0.0;
        locals.var_cdscdr_i_dn13 = 0.0;
        locals.var_cdscdr_i_dn14 = 0.0;
        locals.var_cdscdr_i_rv = 0.0;

        locals.var_l_wln1 = 0.0;
        locals.var_l_wln1_rv = 0.0;

        locals.var_ptwgr_i = 0.0;
        locals.var_ptwgr_i_dn0 = 0.0;
        locals.var_ptwgr_i_dn2 = 0.0;
        locals.var_ptwgr_i_dn3 = 0.0;
        locals.var_ptwgr_i_dn4 = 0.0;
        locals.var_ptwgr_i_dn5 = 0.0;
        locals.var_ptwgr_i_dn6 = 0.0;
        locals.var_ptwgr_i_dn7 = 0.0;
        locals.var_ptwgr_i_dn8 = 0.0;
        locals.var_ptwgr_i_dn9 = 0.0;
        locals.var_ptwgr_i_dn10 = 0.0;
        locals.var_ptwgr_i_dn11 = 0.0;
        locals.var_ptwgr_i_dn12 = 0.0;
        locals.var_ptwgr_i_dn13 = 0.0;
        locals.var_ptwgr_i_dn14 = 0.0;
        locals.var_ptwgr_i_rv = 0.0;

        locals.var_uar_i = 0.0;
        locals.var_uar_i_dn0 = 0.0;
        locals.var_uar_i_dn2 = 0.0;
        locals.var_uar_i_dn3 = 0.0;
        locals.var_uar_i_dn4 = 0.0;
        locals.var_uar_i_dn5 = 0.0;
        locals.var_uar_i_dn6 = 0.0;
        locals.var_uar_i_dn7 = 0.0;
        locals.var_uar_i_dn8 = 0.0;
        locals.var_uar_i_dn9 = 0.0;
        locals.var_uar_i_dn10 = 0.0;
        locals.var_uar_i_dn11 = 0.0;
        locals.var_uar_i_dn12 = 0.0;
        locals.var_uar_i_dn13 = 0.0;
        locals.var_uar_i_dn14 = 0.0;
        locals.var_uar_i_rv = 0.0;

        locals.var_ucsr_i = 0.0;
        locals.var_ucsr_i_rv = 0.0;

        locals.var_ud_a = 0.0;
        locals.var_ud_a_dn0 = 0.0;
        locals.var_ud_a_dn2 = 0.0;
        locals.var_ud_a_dn3 = 0.0;
        locals.var_ud_a_dn4 = 0.0;
        locals.var_ud_a_dn5 = 0.0;
        locals.var_ud_a_dn6 = 0.0;
        locals.var_ud_a_dn7 = 0.0;
        locals.var_ud_a_dn8 = 0.0;
        locals.var_ud_a_dn9 = 0.0;
        locals.var_ud_a_dn10 = 0.0;
        locals.var_ud_a_dn11 = 0.0;
        locals.var_ud_a_dn12 = 0.0;
        locals.var_ud_a_dn13 = 0.0;
        locals.var_ud_a_dn14 = 0.0;
        locals.var_ud_a_rv = 0.0;

        locals.var_w_wwn1 = 0.0;
        locals.var_w_wwn1_rv = 0.0;

        locals.var_inv_sa = 0.0;
        locals.var_inv_sa_dn0 = 0.0;
        locals.var_inv_sa_dn2 = 0.0;
        locals.var_inv_sa_dn3 = 0.0;
        locals.var_inv_sa_dn4 = 0.0;
        locals.var_inv_sa_dn5 = 0.0;
        locals.var_inv_sa_dn6 = 0.0;
        locals.var_inv_sa_dn7 = 0.0;
        locals.var_inv_sa_dn8 = 0.0;
        locals.var_inv_sa_dn9 = 0.0;
        locals.var_inv_sa_dn10 = 0.0;
        locals.var_inv_sa_dn11 = 0.0;
        locals.var_inv_sa_dn12 = 0.0;
        locals.var_inv_sa_dn13 = 0.0;
        locals.var_inv_sa_dn14 = 0.0;
        locals.var_inv_sa_rv = 0.0;

        locals.var_eta_stress = 0.0;
        locals.var_eta_stress_dn0 = 0.0;
        locals.var_eta_stress_dn2 = 0.0;
        locals.var_eta_stress_dn3 = 0.0;
        locals.var_eta_stress_dn4 = 0.0;
        locals.var_eta_stress_dn5 = 0.0;
        locals.var_eta_stress_dn6 = 0.0;
        locals.var_eta_stress_dn7 = 0.0;
        locals.var_eta_stress_dn8 = 0.0;
        locals.var_eta_stress_dn9 = 0.0;
        locals.var_eta_stress_dn10 = 0.0;
        locals.var_eta_stress_dn11 = 0.0;
        locals.var_eta_stress_dn12 = 0.0;
        locals.var_eta_stress_dn13 = 0.0;
        locals.var_eta_stress_dn14 = 0.0;
        locals.var_eta_stress_rv = 0.0;

        locals.var_local_sca = 0.0;
        locals.var_local_sca_dn0 = 0.0;
        locals.var_local_sca_dn2 = 0.0;
        locals.var_local_sca_dn3 = 0.0;
        locals.var_local_sca_dn4 = 0.0;
        locals.var_local_sca_dn5 = 0.0;
        locals.var_local_sca_dn6 = 0.0;
        locals.var_local_sca_dn7 = 0.0;
        locals.var_local_sca_dn8 = 0.0;
        locals.var_local_sca_dn9 = 0.0;
        locals.var_local_sca_dn10 = 0.0;
        locals.var_local_sca_dn11 = 0.0;
        locals.var_local_sca_dn12 = 0.0;
        locals.var_local_sca_dn13 = 0.0;
        locals.var_local_sca_dn14 = 0.0;
        locals.var_local_sca_rv = 0.0;

        locals.var_m0_i = 0.0;
        locals.var_m0_i_rv = 0.0;

        locals.var_m0_t = 0.0;
        locals.var_m0_t_dn4 = 0.0;
        locals.var_m0_t_rv = 0.0;

        locals.var_eta0edge_i = 0.0;
        locals.var_eta0edge_i_dn0 = 0.0;
        locals.var_eta0edge_i_dn2 = 0.0;
        locals.var_eta0edge_i_dn3 = 0.0;
        locals.var_eta0edge_i_dn4 = 0.0;
        locals.var_eta0edge_i_dn5 = 0.0;
        locals.var_eta0edge_i_dn6 = 0.0;
        locals.var_eta0edge_i_dn7 = 0.0;
        locals.var_eta0edge_i_dn8 = 0.0;
        locals.var_eta0edge_i_dn9 = 0.0;
        locals.var_eta0edge_i_dn10 = 0.0;
        locals.var_eta0edge_i_dn11 = 0.0;
        locals.var_eta0edge_i_dn12 = 0.0;
        locals.var_eta0edge_i_dn13 = 0.0;
        locals.var_eta0edge_i_dn14 = 0.0;
        locals.var_eta0edge_i_rv = 0.0;

        locals.var_kt2edge_i = 0.0;
        locals.var_kt2edge_i_rv = 0.0;

        locals.var_k2edge_i = 0.0;
        locals.var_k2edge_i_dn0 = 0.0;
        locals.var_k2edge_i_dn2 = 0.0;
        locals.var_k2edge_i_dn3 = 0.0;
        locals.var_k2edge_i_dn4 = 0.0;
        locals.var_k2edge_i_dn5 = 0.0;
        locals.var_k2edge_i_dn6 = 0.0;
        locals.var_k2edge_i_dn7 = 0.0;
        locals.var_k2edge_i_dn8 = 0.0;
        locals.var_k2edge_i_dn9 = 0.0;
        locals.var_k2edge_i_dn10 = 0.0;
        locals.var_k2edge_i_dn11 = 0.0;
        locals.var_k2edge_i_dn12 = 0.0;
        locals.var_k2edge_i_dn13 = 0.0;
        locals.var_k2edge_i_dn14 = 0.0;
        locals.var_k2edge_i_rv = 0.0;

        locals.var_mnud1 = 0.0;
        locals.var_mnud1_dn0 = 0.0;
        locals.var_mnud1_dn2 = 0.0;
        locals.var_mnud1_dn3 = 0.0;
        locals.var_mnud1_dn4 = 0.0;
        locals.var_mnud1_dn5 = 0.0;
        locals.var_mnud1_dn6 = 0.0;
        locals.var_mnud1_dn7 = 0.0;
        locals.var_mnud1_dn8 = 0.0;
        locals.var_mnud1_dn9 = 0.0;
        locals.var_mnud1_dn10 = 0.0;
        locals.var_mnud1_dn11 = 0.0;
        locals.var_mnud1_dn12 = 0.0;
        locals.var_mnud1_dn13 = 0.0;
        locals.var_mnud1_dn14 = 0.0;
        locals.var_mnud1_rv = 0.0;

        locals.var_c0si_i = 0.0;
        locals.var_c0si_i_rv = 0.0;

        locals.var_c0sisat1_i = 0.0;
        locals.var_c0sisat1_i_rv = 0.0;

        locals.var_eta0r_i = 0.0;
        locals.var_eta0r_i_dn0 = 0.0;
        locals.var_eta0r_i_dn2 = 0.0;
        locals.var_eta0r_i_dn3 = 0.0;
        locals.var_eta0r_i_dn4 = 0.0;
        locals.var_eta0r_i_dn5 = 0.0;
        locals.var_eta0r_i_dn6 = 0.0;
        locals.var_eta0r_i_dn7 = 0.0;
        locals.var_eta0r_i_dn8 = 0.0;
        locals.var_eta0r_i_dn9 = 0.0;
        locals.var_eta0r_i_dn10 = 0.0;
        locals.var_eta0r_i_dn11 = 0.0;
        locals.var_eta0r_i_dn12 = 0.0;
        locals.var_eta0r_i_dn13 = 0.0;
        locals.var_eta0r_i_dn14 = 0.0;
        locals.var_eta0r_i_rv = 0.0;

        locals.var_pclmr_i = 0.0;
        locals.var_pclmr_i_dn0 = 0.0;
        locals.var_pclmr_i_dn2 = 0.0;
        locals.var_pclmr_i_dn3 = 0.0;
        locals.var_pclmr_i_dn4 = 0.0;
        locals.var_pclmr_i_dn5 = 0.0;
        locals.var_pclmr_i_dn6 = 0.0;
        locals.var_pclmr_i_dn7 = 0.0;
        locals.var_pclmr_i_dn8 = 0.0;
        locals.var_pclmr_i_dn9 = 0.0;
        locals.var_pclmr_i_dn10 = 0.0;
        locals.var_pclmr_i_dn11 = 0.0;
        locals.var_pclmr_i_dn12 = 0.0;
        locals.var_pclmr_i_dn13 = 0.0;
        locals.var_pclmr_i_dn14 = 0.0;
        locals.var_pclmr_i_rv = 0.0;

        locals.var_ptwgr_t = 0.0;
        locals.var_ptwgr_t_dn0 = 0.0;
        locals.var_ptwgr_t_dn2 = 0.0;
        locals.var_ptwgr_t_dn3 = 0.0;
        locals.var_ptwgr_t_dn4 = 0.0;
        locals.var_ptwgr_t_dn5 = 0.0;
        locals.var_ptwgr_t_dn6 = 0.0;
        locals.var_ptwgr_t_dn7 = 0.0;
        locals.var_ptwgr_t_dn8 = 0.0;
        locals.var_ptwgr_t_dn9 = 0.0;
        locals.var_ptwgr_t_dn10 = 0.0;
        locals.var_ptwgr_t_dn11 = 0.0;
        locals.var_ptwgr_t_dn12 = 0.0;
        locals.var_ptwgr_t_dn13 = 0.0;
        locals.var_ptwgr_t_dn14 = 0.0;
        locals.var_ptwgr_t_rv = 0.0;

        locals.var_uar_t = 0.0;
        locals.var_uar_t_dn0 = 0.0;
        locals.var_uar_t_dn2 = 0.0;
        locals.var_uar_t_dn3 = 0.0;
        locals.var_uar_t_dn4 = 0.0;
        locals.var_uar_t_dn5 = 0.0;
        locals.var_uar_t_dn6 = 0.0;
        locals.var_uar_t_dn7 = 0.0;
        locals.var_uar_t_dn8 = 0.0;
        locals.var_uar_t_dn9 = 0.0;
        locals.var_uar_t_dn10 = 0.0;
        locals.var_uar_t_dn11 = 0.0;
        locals.var_uar_t_dn12 = 0.0;
        locals.var_uar_t_dn13 = 0.0;
        locals.var_uar_t_dn14 = 0.0;
        locals.var_uar_t_rv = 0.0;

        locals.var_ucsr_t = 0.0;
        locals.var_ucsr_t_dn4 = 0.0;
        locals.var_ucsr_t_rv = 0.0;

        locals.var_vsatr_i = 0.0;
        locals.var_vsatr_i_dn0 = 0.0;
        locals.var_vsatr_i_dn2 = 0.0;
        locals.var_vsatr_i_dn3 = 0.0;
        locals.var_vsatr_i_dn4 = 0.0;
        locals.var_vsatr_i_dn5 = 0.0;
        locals.var_vsatr_i_dn6 = 0.0;
        locals.var_vsatr_i_dn7 = 0.0;
        locals.var_vsatr_i_dn8 = 0.0;
        locals.var_vsatr_i_dn9 = 0.0;
        locals.var_vsatr_i_dn10 = 0.0;
        locals.var_vsatr_i_dn11 = 0.0;
        locals.var_vsatr_i_dn12 = 0.0;
        locals.var_vsatr_i_dn13 = 0.0;
        locals.var_vsatr_i_dn14 = 0.0;
        locals.var_vsatr_i_rv = 0.0;

        locals.var_inv_sb = 0.0;
        locals.var_inv_sb_dn0 = 0.0;
        locals.var_inv_sb_dn2 = 0.0;
        locals.var_inv_sb_dn3 = 0.0;
        locals.var_inv_sb_dn4 = 0.0;
        locals.var_inv_sb_dn5 = 0.0;
        locals.var_inv_sb_dn6 = 0.0;
        locals.var_inv_sb_dn7 = 0.0;
        locals.var_inv_sb_dn8 = 0.0;
        locals.var_inv_sb_dn9 = 0.0;
        locals.var_inv_sb_dn10 = 0.0;
        locals.var_inv_sb_dn11 = 0.0;
        locals.var_inv_sb_dn12 = 0.0;
        locals.var_inv_sb_dn13 = 0.0;
        locals.var_inv_sb_dn14 = 0.0;
        locals.var_inv_sb_rv = 0.0;

        locals.var_local_scb = 0.0;
        locals.var_local_scb_dn0 = 0.0;
        locals.var_local_scb_dn2 = 0.0;
        locals.var_local_scb_dn3 = 0.0;
        locals.var_local_scb_dn4 = 0.0;
        locals.var_local_scb_dn5 = 0.0;
        locals.var_local_scb_dn6 = 0.0;
        locals.var_local_scb_dn7 = 0.0;
        locals.var_local_scb_dn8 = 0.0;
        locals.var_local_scb_dn9 = 0.0;
        locals.var_local_scb_dn10 = 0.0;
        locals.var_local_scb_dn11 = 0.0;
        locals.var_local_scb_dn12 = 0.0;
        locals.var_local_scb_dn13 = 0.0;
        locals.var_local_scb_dn14 = 0.0;
        locals.var_local_scb_rv = 0.0;

        locals.var_k01_i = 0.0;
        locals.var_k01_i_rv = 0.0;

        locals.var_citedge_i = 0.0;
        locals.var_citedge_i_rv = 0.0;

        locals.var_etabedge_i = 0.0;
        locals.var_etabedge_i_rv = 0.0;

        locals.var_kt1expedge_i = 0.0;
        locals.var_kt1expedge_i_rv = 0.0;

        locals.var_kvth0edge_i = 0.0;
        locals.var_kvth0edge_i_rv = 0.0;

        locals.var_c0_i = 0.0;
        locals.var_c0_i_rv = 0.0;

        locals.var_c0si1_i = 0.0;
        locals.var_c0si1_i_rv = 0.0;

        locals.var_c0sisat_t = 0.0;
        locals.var_c0sisat_t_dn4 = 0.0;
        locals.var_c0sisat_t_rv = 0.0;

        locals.var_rdstemphv = 1.0;
        locals.var_rdstemphv_dn4 = 0.0;
        locals.var_rdstemphv_rv = 0.0;

        locals.var_eta0r_t = 0.0;
        locals.var_eta0r_t_dn0 = 0.0;
        locals.var_eta0r_t_dn2 = 0.0;
        locals.var_eta0r_t_dn3 = 0.0;
        locals.var_eta0r_t_dn4 = 0.0;
        locals.var_eta0r_t_dn5 = 0.0;
        locals.var_eta0r_t_dn6 = 0.0;
        locals.var_eta0r_t_dn7 = 0.0;
        locals.var_eta0r_t_dn8 = 0.0;
        locals.var_eta0r_t_dn9 = 0.0;
        locals.var_eta0r_t_dn10 = 0.0;
        locals.var_eta0r_t_dn11 = 0.0;
        locals.var_eta0r_t_dn12 = 0.0;
        locals.var_eta0r_t_dn13 = 0.0;
        locals.var_eta0r_t_dn14 = 0.0;
        locals.var_eta0r_t_rv = 0.0;

        locals.var_pdiblcr_i = 0.0;
        locals.var_pdiblcr_i_dn0 = 0.0;
        locals.var_pdiblcr_i_dn2 = 0.0;
        locals.var_pdiblcr_i_dn3 = 0.0;
        locals.var_pdiblcr_i_dn4 = 0.0;
        locals.var_pdiblcr_i_dn5 = 0.0;
        locals.var_pdiblcr_i_dn6 = 0.0;
        locals.var_pdiblcr_i_dn7 = 0.0;
        locals.var_pdiblcr_i_dn8 = 0.0;
        locals.var_pdiblcr_i_dn9 = 0.0;
        locals.var_pdiblcr_i_dn10 = 0.0;
        locals.var_pdiblcr_i_dn11 = 0.0;
        locals.var_pdiblcr_i_dn12 = 0.0;
        locals.var_pdiblcr_i_dn13 = 0.0;
        locals.var_pdiblcr_i_dn14 = 0.0;
        locals.var_pdiblcr_i_rv = 0.0;

        locals.var_u0r_i = 0.0;
        locals.var_u0r_i_rv = 0.0;

        locals.var_ucr_i = 0.0;
        locals.var_ucr_i_dn0 = 0.0;
        locals.var_ucr_i_dn2 = 0.0;
        locals.var_ucr_i_dn3 = 0.0;
        locals.var_ucr_i_dn4 = 0.0;
        locals.var_ucr_i_dn5 = 0.0;
        locals.var_ucr_i_dn6 = 0.0;
        locals.var_ucr_i_dn7 = 0.0;
        locals.var_ucr_i_dn8 = 0.0;
        locals.var_ucr_i_dn9 = 0.0;
        locals.var_ucr_i_dn10 = 0.0;
        locals.var_ucr_i_dn11 = 0.0;
        locals.var_ucr_i_dn12 = 0.0;
        locals.var_ucr_i_dn13 = 0.0;
        locals.var_ucr_i_dn14 = 0.0;
        locals.var_ucr_i_rv = 0.0;

        locals.var_udr_i = 0.0;
        locals.var_udr_i_dn0 = 0.0;
        locals.var_udr_i_dn2 = 0.0;
        locals.var_udr_i_dn3 = 0.0;
        locals.var_udr_i_dn4 = 0.0;
        locals.var_udr_i_dn5 = 0.0;
        locals.var_udr_i_dn6 = 0.0;
        locals.var_udr_i_dn7 = 0.0;
        locals.var_udr_i_dn8 = 0.0;
        locals.var_udr_i_dn9 = 0.0;
        locals.var_udr_i_dn10 = 0.0;
        locals.var_udr_i_dn11 = 0.0;
        locals.var_udr_i_dn12 = 0.0;
        locals.var_udr_i_dn13 = 0.0;
        locals.var_udr_i_dn14 = 0.0;
        locals.var_udr_i_rv = 0.0;

        locals.var_vsatr_t = 0.0;
        locals.var_vsatr_t_dn0 = 0.0;
        locals.var_vsatr_t_dn2 = 0.0;
        locals.var_vsatr_t_dn3 = 0.0;
        locals.var_vsatr_t_dn4 = 0.0;
        locals.var_vsatr_t_dn5 = 0.0;
        locals.var_vsatr_t_dn6 = 0.0;
        locals.var_vsatr_t_dn7 = 0.0;
        locals.var_vsatr_t_dn8 = 0.0;
        locals.var_vsatr_t_dn9 = 0.0;
        locals.var_vsatr_t_dn10 = 0.0;
        locals.var_vsatr_t_dn11 = 0.0;
        locals.var_vsatr_t_dn12 = 0.0;
        locals.var_vsatr_t_dn13 = 0.0;
        locals.var_vsatr_t_dn14 = 0.0;
        locals.var_vsatr_t_rv = 0.0;

        locals.var_vth0_stress_edge = 0.0;
        locals.var_vth0_stress_edge_dn0 = 0.0;
        locals.var_vth0_stress_edge_dn2 = 0.0;
        locals.var_vth0_stress_edge_dn3 = 0.0;
        locals.var_vth0_stress_edge_dn4 = 0.0;
        locals.var_vth0_stress_edge_dn5 = 0.0;
        locals.var_vth0_stress_edge_dn6 = 0.0;
        locals.var_vth0_stress_edge_dn7 = 0.0;
        locals.var_vth0_stress_edge_dn8 = 0.0;
        locals.var_vth0_stress_edge_dn9 = 0.0;
        locals.var_vth0_stress_edge_dn10 = 0.0;
        locals.var_vth0_stress_edge_dn11 = 0.0;
        locals.var_vth0_stress_edge_dn12 = 0.0;
        locals.var_vth0_stress_edge_dn13 = 0.0;
        locals.var_vth0_stress_edge_dn14 = 0.0;
        locals.var_vth0_stress_edge_rv = 0.0;

        locals.var_eta_stress_edge = 0.0;
        locals.var_eta_stress_edge_dn0 = 0.0;
        locals.var_eta_stress_edge_dn2 = 0.0;
        locals.var_eta_stress_edge_dn3 = 0.0;
        locals.var_eta_stress_edge_dn4 = 0.0;
        locals.var_eta_stress_edge_dn5 = 0.0;
        locals.var_eta_stress_edge_dn6 = 0.0;
        locals.var_eta_stress_edge_dn7 = 0.0;
        locals.var_eta_stress_edge_dn8 = 0.0;
        locals.var_eta_stress_edge_dn9 = 0.0;
        locals.var_eta_stress_edge_dn10 = 0.0;
        locals.var_eta_stress_edge_dn11 = 0.0;
        locals.var_eta_stress_edge_dn12 = 0.0;
        locals.var_eta_stress_edge_dn13 = 0.0;
        locals.var_eta_stress_edge_dn14 = 0.0;
        locals.var_eta_stress_edge_rv = 0.0;

        locals.var_local_scc = 0.0;
        locals.var_local_scc_dn0 = 0.0;
        locals.var_local_scc_dn2 = 0.0;
        locals.var_local_scc_dn3 = 0.0;
        locals.var_local_scc_dn4 = 0.0;
        locals.var_local_scc_dn5 = 0.0;
        locals.var_local_scc_dn6 = 0.0;
        locals.var_local_scc_dn7 = 0.0;
        locals.var_local_scc_dn8 = 0.0;
        locals.var_local_scc_dn9 = 0.0;
        locals.var_local_scc_dn10 = 0.0;
        locals.var_local_scc_dn11 = 0.0;
        locals.var_local_scc_dn12 = 0.0;
        locals.var_local_scc_dn13 = 0.0;
        locals.var_local_scc_dn14 = 0.0;
        locals.var_local_scc_rv = 0.0;

        locals.var_m01_i = 0.0;
        locals.var_m01_i_rv = 0.0;

        locals.var_cdscdedge_i = 0.0;
        locals.var_cdscdedge_i_rv = 0.0;

        locals.var_kt1edge_i = 0.0;
        locals.var_kt1edge_i_rv = 0.0;

        locals.var_tnfactoredge_i = 0.0;
        locals.var_tnfactoredge_i_rv = 0.0;

        locals.var_stk2edge_i = 0.0;
        locals.var_stk2edge_i_rv = 0.0;

        locals.var_c01_i = 0.0;
        locals.var_c01_i_rv = 0.0;

        locals.var_c0si_t = 0.0;
        locals.var_c0si_t_dn4 = 0.0;
        locals.var_c0si_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        locals.var_rdrift_d = 0.0;
        locals.var_rdrift_d_dn0 = 0.0;
        locals.var_rdrift_d_dn2 = 0.0;
        locals.var_rdrift_d_dn3 = 0.0;
        locals.var_rdrift_d_dn4 = 0.0;
        locals.var_rdrift_d_dn5 = 0.0;
        locals.var_rdrift_d_dn6 = 0.0;
        locals.var_rdrift_d_dn7 = 0.0;
        locals.var_rdrift_d_dn8 = 0.0;
        locals.var_rdrift_d_dn9 = 0.0;
        locals.var_rdrift_d_dn10 = 0.0;
        locals.var_rdrift_d_dn11 = 0.0;
        locals.var_rdrift_d_dn12 = 0.0;
        locals.var_rdrift_d_dn13 = 0.0;
        locals.var_rdrift_d_dn14 = 0.0;
        locals.var_rdrift_d_rv = 0.0;

        locals.var_vdrift_t = 1.0;
        locals.var_vdrift_t_dn4 = 0.0;
        locals.var_vdrift_t_rv = 0.0;

        locals.var_l_lln1 = 0.0;
        locals.var_l_lln1_rv = 0.0;

        locals.var_psatr_i = 0.0;
        locals.var_psatr_i_rv = 0.0;

        locals.var_u0r_t = 0.0;
        locals.var_u0r_t_dn4 = 0.0;
        locals.var_u0r_t_rv = 0.0;

        locals.var_ucr_t = 0.0;
        locals.var_ucr_t_dn0 = 0.0;
        locals.var_ucr_t_dn2 = 0.0;
        locals.var_ucr_t_dn3 = 0.0;
        locals.var_ucr_t_dn4 = 0.0;
        locals.var_ucr_t_dn5 = 0.0;
        locals.var_ucr_t_dn6 = 0.0;
        locals.var_ucr_t_dn7 = 0.0;
        locals.var_ucr_t_dn8 = 0.0;
        locals.var_ucr_t_dn9 = 0.0;
        locals.var_ucr_t_dn10 = 0.0;
        locals.var_ucr_t_dn11 = 0.0;
        locals.var_ucr_t_dn12 = 0.0;
        locals.var_ucr_t_dn13 = 0.0;
        locals.var_ucr_t_dn14 = 0.0;
        locals.var_ucr_t_rv = 0.0;

        locals.var_udr_t = 0.0;
        locals.var_udr_t_dn0 = 0.0;
        locals.var_udr_t_dn2 = 0.0;
        locals.var_udr_t_dn3 = 0.0;
        locals.var_udr_t_dn4 = 0.0;
        locals.var_udr_t_dn5 = 0.0;
        locals.var_udr_t_dn6 = 0.0;
        locals.var_udr_t_dn7 = 0.0;
        locals.var_udr_t_dn8 = 0.0;
        locals.var_udr_t_dn9 = 0.0;
        locals.var_udr_t_dn10 = 0.0;
        locals.var_udr_t_dn11 = 0.0;
        locals.var_udr_t_dn12 = 0.0;
        locals.var_udr_t_dn13 = 0.0;
        locals.var_udr_t_dn14 = 0.0;
        locals.var_udr_t_rv = 0.0;

        locals.var_w_lwn1 = 0.0;
        locals.var_w_lwn1_rv = 0.0;

        locals.var_k2_stress_edge = 0.0;
        locals.var_k2_stress_edge_dn0 = 0.0;
        locals.var_k2_stress_edge_dn2 = 0.0;
        locals.var_k2_stress_edge_dn3 = 0.0;
        locals.var_k2_stress_edge_dn4 = 0.0;
        locals.var_k2_stress_edge_dn5 = 0.0;
        locals.var_k2_stress_edge_dn6 = 0.0;
        locals.var_k2_stress_edge_dn7 = 0.0;
        locals.var_k2_stress_edge_dn8 = 0.0;
        locals.var_k2_stress_edge_dn9 = 0.0;
        locals.var_k2_stress_edge_dn10 = 0.0;
        locals.var_k2_stress_edge_dn11 = 0.0;
        locals.var_k2_stress_edge_dn12 = 0.0;
        locals.var_k2_stress_edge_dn13 = 0.0;
        locals.var_k2_stress_edge_dn14 = 0.0;
        locals.var_k2_stress_edge_rv = 0.0;

        locals.var_k0_i = 0.0;
        locals.var_k0_i_rv = 0.0;

        locals.var_k0_t = 0.0;
        locals.var_k0_t_dn4 = 0.0;
        locals.var_k0_t_rv = 0.0;

        locals.var_cdscbedge_i = 0.0;
        locals.var_cdscbedge_i_rv = 0.0;

        locals.var_kt1ledge_i = 0.0;
        locals.var_kt1ledge_i_rv = 0.0;

        locals.var_teta0edge_i = 0.0;
        locals.var_teta0edge_i_rv = 0.0;

        locals.var_steta0edge_i = 0.0;
        locals.var_steta0edge_i_rv = 0.0;

        locals.var_c0_t = 0.0;
        locals.var_c0_t_dn4 = 0.0;
        locals.var_c0_t_rv = 0.0;

        locals.var_c0sisat_i = 0.0;
        locals.var_c0sisat_i_rv = 0.0;

        locals.var_rdrift_s = 0.0;
        locals.var_rdrift_s_dn0 = 0.0;
        locals.var_rdrift_s_dn2 = 0.0;
        locals.var_rdrift_s_dn3 = 0.0;
        locals.var_rdrift_s_dn4 = 0.0;
        locals.var_rdrift_s_dn5 = 0.0;
        locals.var_rdrift_s_dn6 = 0.0;
        locals.var_rdrift_s_dn7 = 0.0;
        locals.var_rdrift_s_dn8 = 0.0;
        locals.var_rdrift_s_dn9 = 0.0;
        locals.var_rdrift_s_dn10 = 0.0;
        locals.var_rdrift_s_dn11 = 0.0;
        locals.var_rdrift_s_dn12 = 0.0;
        locals.var_rdrift_s_dn13 = 0.0;
        locals.var_rdrift_s_dn14 = 0.0;
        locals.var_rdrift_s_rv = 0.0;

        locals.var_k2edgewe_i = 0.0;
        locals.var_k2edgewe_i_rv = 0.0;

        locals.var_kvth0edgewe_i = 0.0;
        locals.var_kvth0edgewe_i_rv = 0.0;

        locals.var_temp_adeff = 0.0;
        locals.var_temp_adeff_dn0 = 0.0;
        locals.var_temp_adeff_dn2 = 0.0;
        locals.var_temp_adeff_dn3 = 0.0;
        locals.var_temp_adeff_dn4 = 0.0;
        locals.var_temp_adeff_dn5 = 0.0;
        locals.var_temp_adeff_dn6 = 0.0;
        locals.var_temp_adeff_dn7 = 0.0;
        locals.var_temp_adeff_dn8 = 0.0;
        locals.var_temp_adeff_dn9 = 0.0;
        locals.var_temp_adeff_dn10 = 0.0;
        locals.var_temp_adeff_dn11 = 0.0;
        locals.var_temp_adeff_dn12 = 0.0;
        locals.var_temp_adeff_dn13 = 0.0;
        locals.var_temp_adeff_dn14 = 0.0;
        locals.var_temp_adeff_rv = 0.0;

        locals.var_temp_aseff = 0.0;
        locals.var_temp_aseff_dn0 = 0.0;
        locals.var_temp_aseff_dn2 = 0.0;
        locals.var_temp_aseff_dn3 = 0.0;
        locals.var_temp_aseff_dn4 = 0.0;
        locals.var_temp_aseff_dn5 = 0.0;
        locals.var_temp_aseff_dn6 = 0.0;
        locals.var_temp_aseff_dn7 = 0.0;
        locals.var_temp_aseff_dn8 = 0.0;
        locals.var_temp_aseff_dn9 = 0.0;
        locals.var_temp_aseff_dn10 = 0.0;
        locals.var_temp_aseff_dn11 = 0.0;
        locals.var_temp_aseff_dn12 = 0.0;
        locals.var_temp_aseff_dn13 = 0.0;
        locals.var_temp_aseff_dn14 = 0.0;
        locals.var_temp_aseff_rv = 0.0;

        locals.var_temp_pdeff = 0.0;
        locals.var_temp_pdeff_dn0 = 0.0;
        locals.var_temp_pdeff_dn2 = 0.0;
        locals.var_temp_pdeff_dn3 = 0.0;
        locals.var_temp_pdeff_dn4 = 0.0;
        locals.var_temp_pdeff_dn5 = 0.0;
        locals.var_temp_pdeff_dn6 = 0.0;
        locals.var_temp_pdeff_dn7 = 0.0;
        locals.var_temp_pdeff_dn8 = 0.0;
        locals.var_temp_pdeff_dn9 = 0.0;
        locals.var_temp_pdeff_dn10 = 0.0;
        locals.var_temp_pdeff_dn11 = 0.0;
        locals.var_temp_pdeff_dn12 = 0.0;
        locals.var_temp_pdeff_dn13 = 0.0;
        locals.var_temp_pdeff_dn14 = 0.0;
        locals.var_temp_pdeff_rv = 0.0;

        locals.var_temp_pseff = 0.0;
        locals.var_temp_pseff_dn0 = 0.0;
        locals.var_temp_pseff_dn2 = 0.0;
        locals.var_temp_pseff_dn3 = 0.0;
        locals.var_temp_pseff_dn4 = 0.0;
        locals.var_temp_pseff_dn5 = 0.0;
        locals.var_temp_pseff_dn6 = 0.0;
        locals.var_temp_pseff_dn7 = 0.0;
        locals.var_temp_pseff_dn8 = 0.0;
        locals.var_temp_pseff_dn9 = 0.0;
        locals.var_temp_pseff_dn10 = 0.0;
        locals.var_temp_pseff_dn11 = 0.0;
        locals.var_temp_pseff_dn12 = 0.0;
        locals.var_temp_pseff_dn13 = 0.0;
        locals.var_temp_pseff_dn14 = 0.0;
        locals.var_temp_pseff_rv = 0.0;

        locals.var_abulkiv = 1.0;
        locals.var_abulkiv_dn0 = 0.0;
        locals.var_abulkiv_dn2 = 0.0;
        locals.var_abulkiv_dn3 = 0.0;
        locals.var_abulkiv_dn4 = 0.0;
        locals.var_abulkiv_dn5 = 0.0;
        locals.var_abulkiv_dn6 = 0.0;
        locals.var_abulkiv_dn7 = 0.0;
        locals.var_abulkiv_dn8 = 0.0;
        locals.var_abulkiv_dn9 = 0.0;
        locals.var_abulkiv_dn10 = 0.0;
        locals.var_abulkiv_dn11 = 0.0;
        locals.var_abulkiv_dn12 = 0.0;
        locals.var_abulkiv_dn13 = 0.0;
        locals.var_abulkiv_dn14 = 0.0;
        locals.var_abulkiv_rv = 0.0;

        locals.var_abulkcv = 1.0;
        locals.var_abulkcv_dn0 = 0.0;
        locals.var_abulkcv_dn2 = 0.0;
        locals.var_abulkcv_dn3 = 0.0;
        locals.var_abulkcv_dn4 = 0.0;
        locals.var_abulkcv_dn5 = 0.0;
        locals.var_abulkcv_dn6 = 0.0;
        locals.var_abulkcv_dn7 = 0.0;
        locals.var_abulkcv_dn8 = 0.0;
        locals.var_abulkcv_dn9 = 0.0;
        locals.var_abulkcv_dn10 = 0.0;
        locals.var_abulkcv_dn11 = 0.0;
        locals.var_abulkcv_dn12 = 0.0;
        locals.var_abulkcv_dn13 = 0.0;
        locals.var_abulkcv_dn14 = 0.0;
        locals.var_abulkcv_rv = 0.0;

        locals.var_gdpr = 0.0;
        locals.var_gdpr_dn0 = 0.0;
        locals.var_gdpr_dn2 = 0.0;
        locals.var_gdpr_dn3 = 0.0;
        locals.var_gdpr_dn4 = 0.0;
        locals.var_gdpr_dn5 = 0.0;
        locals.var_gdpr_dn6 = 0.0;
        locals.var_gdpr_dn7 = 0.0;
        locals.var_gdpr_dn8 = 0.0;
        locals.var_gdpr_dn9 = 0.0;
        locals.var_gdpr_dn10 = 0.0;
        locals.var_gdpr_dn11 = 0.0;
        locals.var_gdpr_dn12 = 0.0;
        locals.var_gdpr_dn13 = 0.0;
        locals.var_gdpr_dn14 = 0.0;
        locals.var_gdpr_rv = 0.0;

        locals.var_gspr = 0.0;
        locals.var_gspr_dn0 = 0.0;
        locals.var_gspr_dn2 = 0.0;
        locals.var_gspr_dn3 = 0.0;
        locals.var_gspr_dn4 = 0.0;
        locals.var_gspr_dn5 = 0.0;
        locals.var_gspr_dn6 = 0.0;
        locals.var_gspr_dn7 = 0.0;
        locals.var_gspr_dn8 = 0.0;
        locals.var_gspr_dn9 = 0.0;
        locals.var_gspr_dn10 = 0.0;
        locals.var_gspr_dn11 = 0.0;
        locals.var_gspr_dn12 = 0.0;
        locals.var_gspr_dn13 = 0.0;
        locals.var_gspr_dn14 = 0.0;
        locals.var_gspr_rv = 0.0;

        locals.var_gdrift_d = 0.0;
        locals.var_gdrift_d_dn0 = 0.0;
        locals.var_gdrift_d_dn2 = 0.0;
        locals.var_gdrift_d_dn3 = 0.0;
        locals.var_gdrift_d_dn4 = 0.0;
        locals.var_gdrift_d_dn5 = 0.0;
        locals.var_gdrift_d_dn6 = 0.0;
        locals.var_gdrift_d_dn7 = 0.0;
        locals.var_gdrift_d_dn8 = 0.0;
        locals.var_gdrift_d_dn9 = 0.0;
        locals.var_gdrift_d_dn10 = 0.0;
        locals.var_gdrift_d_dn11 = 0.0;
        locals.var_gdrift_d_dn12 = 0.0;
        locals.var_gdrift_d_dn13 = 0.0;
        locals.var_gdrift_d_dn14 = 0.0;
        locals.var_gdrift_d_rv = 0.0;

        locals.var_gdrift_s = 0.0;
        locals.var_gdrift_s_dn0 = 0.0;
        locals.var_gdrift_s_dn2 = 0.0;
        locals.var_gdrift_s_dn3 = 0.0;
        locals.var_gdrift_s_dn4 = 0.0;
        locals.var_gdrift_s_dn5 = 0.0;
        locals.var_gdrift_s_dn6 = 0.0;
        locals.var_gdrift_s_dn7 = 0.0;
        locals.var_gdrift_s_dn8 = 0.0;
        locals.var_gdrift_s_dn9 = 0.0;
        locals.var_gdrift_s_dn10 = 0.0;
        locals.var_gdrift_s_dn11 = 0.0;
        locals.var_gdrift_s_dn12 = 0.0;
        locals.var_gdrift_s_dn13 = 0.0;
        locals.var_gdrift_s_dn14 = 0.0;
        locals.var_gdrift_s_rv = 0.0;

        locals.var_vd1 = 0.0;
        locals.var_vd1_dn6 = 0.0;
        locals.var_vd1_dn11 = 0.0;
        locals.var_vd1_rv = 0.0;

        locals.var_vs1 = 0.0;
        locals.var_vs1_dn8 = 0.0;
        locals.var_vs1_dn11 = 0.0;
        locals.var_vs1_rv = 0.0;

        locals.var_idrift_sat_d = 0.0;
        locals.var_idrift_sat_d_dn0 = 0.0;
        locals.var_idrift_sat_d_dn2 = 0.0;
        locals.var_idrift_sat_d_dn3 = 0.0;
        locals.var_idrift_sat_d_dn4 = 0.0;
        locals.var_idrift_sat_d_dn5 = 0.0;
        locals.var_idrift_sat_d_dn6 = 0.0;
        locals.var_idrift_sat_d_dn7 = 0.0;
        locals.var_idrift_sat_d_dn8 = 0.0;
        locals.var_idrift_sat_d_dn9 = 0.0;
        locals.var_idrift_sat_d_dn10 = 0.0;
        locals.var_idrift_sat_d_dn11 = 0.0;
        locals.var_idrift_sat_d_dn12 = 0.0;
        locals.var_idrift_sat_d_dn13 = 0.0;
        locals.var_idrift_sat_d_dn14 = 0.0;
        locals.var_idrift_sat_d_rv = 0.0;

        locals.var_ln_t1_t2 = 0.0;
        locals.var_ln_t1_t2_dn0 = 0.0;
        locals.var_ln_t1_t2_dn2 = 0.0;
        locals.var_ln_t1_t2_dn3 = 0.0;
        locals.var_ln_t1_t2_dn4 = 0.0;
        locals.var_ln_t1_t2_dn5 = 0.0;
        locals.var_ln_t1_t2_dn6 = 0.0;
        locals.var_ln_t1_t2_dn7 = 0.0;
        locals.var_ln_t1_t2_dn8 = 0.0;
        locals.var_ln_t1_t2_dn9 = 0.0;
        locals.var_ln_t1_t2_dn10 = 0.0;
        locals.var_ln_t1_t2_dn11 = 0.0;
        locals.var_ln_t1_t2_dn12 = 0.0;
        locals.var_ln_t1_t2_dn13 = 0.0;
        locals.var_ln_t1_t2_dn14 = 0.0;
        locals.var_ln_t1_t2_rv = 0.0;

        locals.var_vdseffii = 0.0;
        locals.var_vdseffii_dn0 = 0.0;
        locals.var_vdseffii_dn2 = 0.0;
        locals.var_vdseffii_dn3 = 0.0;
        locals.var_vdseffii_dn4 = 0.0;
        locals.var_vdseffii_dn5 = 0.0;
        locals.var_vdseffii_dn6 = 0.0;
        locals.var_vdseffii_dn7 = 0.0;
        locals.var_vdseffii_dn8 = 0.0;
        locals.var_vdseffii_dn9 = 0.0;
        locals.var_vdseffii_dn10 = 0.0;
        locals.var_vdseffii_dn11 = 0.0;
        locals.var_vdseffii_dn12 = 0.0;
        locals.var_vdseffii_dn13 = 0.0;
        locals.var_vdseffii_dn14 = 0.0;
        locals.var_vdseffii_rv = 0.0;

        locals.var_beta0r_t = 0.0;
        locals.var_beta0r_t_dn4 = 0.0;
        locals.var_beta0r_t_rv = 0.0;

        locals.var_alpha0r_i = 0.0;
        locals.var_alpha0r_i_dn0 = 0.0;
        locals.var_alpha0r_i_dn2 = 0.0;
        locals.var_alpha0r_i_dn3 = 0.0;
        locals.var_alpha0r_i_dn4 = 0.0;
        locals.var_alpha0r_i_dn5 = 0.0;
        locals.var_alpha0r_i_dn6 = 0.0;
        locals.var_alpha0r_i_dn7 = 0.0;
        locals.var_alpha0r_i_dn8 = 0.0;
        locals.var_alpha0r_i_dn9 = 0.0;
        locals.var_alpha0r_i_dn10 = 0.0;
        locals.var_alpha0r_i_dn11 = 0.0;
        locals.var_alpha0r_i_dn12 = 0.0;
        locals.var_alpha0r_i_dn13 = 0.0;
        locals.var_alpha0r_i_dn14 = 0.0;
        locals.var_alpha0r_i_rv = 0.0;

        locals.var_beta0r_i = 0.0;
        locals.var_beta0r_i_rv = 0.0;

        locals.var_vb_cm = 0.0;
        locals.var_vb_cm_dn3 = 0.0;
        locals.var_vb_cm_dn11 = 0.0;
        locals.var_vb_cm_rv = 0.0;

        let assign940_e2092: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign940_e2092;
        locals.var_guard1_rv = 0.0;

        let (assign950_e2096,) = {
    if (locals.var_guard1 != 0.0) {
        (1.0,)
    } else {
        (locals.var_devsign,)
    }
};
        locals.var_devsign = assign950_e2096;
        locals.var_devsign_rv = 0.0;

        let (assign960_e2102,) = {
    if (locals.var_guard1 == 0.0) {
        let assign960_e2100: f64 = (-1.0);
        (assign960_e2100,)
    } else {
        (locals.var_devsign,)
    }
};
        locals.var_devsign = assign960_e2102;
        locals.var_devsign_rv = 0.0;

        let assign970_e2105: f64 = (p.p110 * 8.85418e-12);
        locals.var_epssi = assign970_e2105;
        locals.var_epssi_rv = 0.0;

        let assign980_e2108: f64 = (p.p111 * 8.85418e-12);
        locals.var_epsox = assign980_e2108;
        locals.var_epsox_rv = 0.0;

        let assign990_e2111: f64 = (p.p111 * 8.85418e-12);
        let assign990_e2113: f64 = (assign990_e2111 / p.p77);
        locals.var_cox = assign990_e2113;
        locals.var_cox_rv = 0.0;

        let assign1000_e2116: f64 = (p.p110 / p.p111);
        locals.var_epsratio = assign1000_e2116;
        locals.var_epsratio_rv = 0.0;

        let assign1010_e2119: f64 = if (!param_given[78]) { 1.0 } else { 0.0 };
        locals.var_guard2 = assign1010_e2119;
        locals.var_guard2_rv = 0.0;

        let (assign1020_e2129,) = {
    if (locals.var_guard2 != 0.0) {
        let assign1020_e2123: f64 = (p.p77 * p.p111);
        let assign1020_e2125: f64 = (assign1020_e2123 / 3.9);
        let assign1020_e2127: f64 = (assign1020_e2125 - p.p79);
        (assign1020_e2127,)
    } else {
        (locals.var_bsimbulktoxp,)
    }
};
        locals.var_bsimbulktoxp = assign1020_e2129;
        locals.var_bsimbulktoxp_rv = 0.0;

        let (assign1030_e2134,) = {
    if (locals.var_guard2 == 0.0) {
        (p.p78,)
    } else {
        (locals.var_bsimbulktoxp,)
    }
};
        locals.var_bsimbulktoxp = assign1030_e2134;
        locals.var_bsimbulktoxp_rv = 0.0;

        let assign1040_e2137: f64 = (p.p0 * p.p52);
        locals.var_l_mult = assign1040_e2137;
        locals.var_l_mult_rv = 0.0;

        let assign1050_e2140: f64 = (p.p1 * p.p53);
        locals.var_w_mult = assign1050_e2140;
        locals.var_w_mult_rv = 0.0;

        let assign1060_e2143: f64 = (locals.var_l_mult + p.p54);
        locals.var_lnew = assign1060_e2143;
        locals.var_lnew_rv = 0.0;

        let assign1080_e2149: f64 = (locals.var_w_mult / p.p2);
        locals.var_w_by_nf = assign1080_e2149;
        locals.var_w_by_nf_rv = 0.0;

        let assign1090_e2152: f64 = (locals.var_w_by_nf + p.p56);
        locals.var_wnew = assign1090_e2152;
        locals.var_wnew_rv = 0.0;

        let assign1110_e2158: f64 = (-p.p61);
        let assign1110_e2159: f64 = (locals.var_lnew).powf(assign1110_e2158);
        locals.var_l_lln = assign1110_e2159;
        locals.var_l_lln_rv = 0.0;

        let assign1120_e2162: f64 = (-p.p62);
        let assign1120_e2163: f64 = (locals.var_wnew).powf(assign1120_e2162);
        locals.var_w_lwn = assign1120_e2163;
        locals.var_w_lwn_rv = 0.0;

        let assign1130_e2166: f64 = (locals.var_l_lln * locals.var_w_lwn);
        locals.var_lw_lln_lwn = assign1130_e2166;
        locals.var_lw_lln_lwn_rv = 0.0;

        let assign1140_e2170: f64 = (p.p58 * locals.var_l_lln);
        let assign1140_e2171: f64 = (p.p57 + assign1140_e2170);
        let assign1140_e2174: f64 = (p.p59 * locals.var_w_lwn);
        let assign1140_e2175: f64 = (assign1140_e2171 + assign1140_e2174);
        let assign1140_e2178: f64 = (p.p60 * locals.var_lw_lln_lwn);
        let assign1140_e2179: f64 = (assign1140_e2175 + assign1140_e2178);
        locals.var_dliv = assign1140_e2179;
        locals.var_dliv_rv = 0.0;

        let assign1150_e2182: f64 = (-p.p67);
        let assign1150_e2183: f64 = (locals.var_lnew).powf(assign1150_e2182);
        locals.var_l_wln = assign1150_e2183;
        locals.var_l_wln_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign1160_e2186: f64 = (-p.p68);
        let assign1160_e2187: f64 = (locals.var_wnew).powf(assign1160_e2186);
        locals.var_w_wwn = assign1160_e2187;
        locals.var_w_wwn_rv = 0.0;

        let assign1170_e2190: f64 = (locals.var_l_wln * locals.var_w_wwn);
        locals.var_lw_wln_wwn = assign1170_e2190;
        locals.var_lw_wln_wwn_rv = 0.0;

        let assign1180_e2194: f64 = (p.p64 * locals.var_l_wln);
        let assign1180_e2195: f64 = (p.p63 + assign1180_e2194);
        let assign1180_e2198: f64 = (p.p65 * locals.var_w_wwn);
        let assign1180_e2199: f64 = (assign1180_e2195 + assign1180_e2198);
        let assign1180_e2202: f64 = (p.p66 * locals.var_lw_wln_wwn);
        let assign1180_e2203: f64 = (assign1180_e2199 + assign1180_e2202);
        locals.var_dwiv = assign1180_e2203;
        locals.var_dwiv_rv = 0.0;

        let assign1190_e2207: f64 = (2.0 * locals.var_dliv);
        let assign1190_e2208: f64 = (locals.var_lnew - assign1190_e2207);
        locals.var_leff = assign1190_e2208;
        locals.var_leff_rv = 0.0;

        let assign1220_e2218: f64 = (2.0 * locals.var_dwiv);
        let assign1220_e2219: f64 = (locals.var_wnew - assign1220_e2218);
        locals.var_weff = assign1220_e2219;
        locals.var_weff_rv = 0.0;

        let assign1250_e2229: f64 = (p.p70 * locals.var_l_lln);
        let assign1250_e2230: f64 = (p.p69 + assign1250_e2229);
        let assign1250_e2233: f64 = (p.p71 * locals.var_w_lwn);
        let assign1250_e2234: f64 = (assign1250_e2230 + assign1250_e2233);
        let assign1250_e2237: f64 = (p.p72 * locals.var_lw_lln_lwn);
        let assign1250_e2238: f64 = (assign1250_e2234 + assign1250_e2237);
        locals.var_dlcv = assign1250_e2238;
        locals.var_dlcv_rv = 0.0;

        let assign1260_e2242: f64 = (p.p74 * locals.var_l_wln);
        let assign1260_e2243: f64 = (p.p73 + assign1260_e2242);
        let assign1260_e2246: f64 = (p.p75 * locals.var_w_wwn);
        let assign1260_e2247: f64 = (assign1260_e2243 + assign1260_e2246);
        let assign1260_e2250: f64 = (p.p76 * locals.var_lw_wln_wwn);
        let assign1260_e2251: f64 = (assign1260_e2247 + assign1260_e2250);
        locals.var_dwcv = assign1260_e2251;
        locals.var_dwcv_rv = 0.0;

        let assign1270_e2255: f64 = (2.0 * locals.var_dlcv);
        let assign1270_e2256: f64 = (locals.var_lnew - assign1270_e2255);
        locals.var_lact = assign1270_e2256;
        locals.var_lact_rv = 0.0;

        let assign1300_e2266: f64 = (2.0 * locals.var_dwcv);
        let assign1300_e2267: f64 = (locals.var_wnew - assign1300_e2266);
        locals.var_wact = assign1300_e2267;
        locals.var_wact_rv = 0.0;

        let assign1330_e2278: f64 = (locals.var_lnew).powf(p.p67);
        let assign1330_e2279: f64 = (p.p74 / assign1330_e2278);
        let assign1330_e2280: f64 = (p.p138 + assign1330_e2279);
        let assign1330_e2284: f64 = (locals.var_wnew).powf(p.p68);
        let assign1330_e2285: f64 = (p.p75 / assign1330_e2284);
        let assign1330_e2286: f64 = (assign1330_e2280 + assign1330_e2285);
        let assign1330_e2290: f64 = (locals.var_lnew).powf(p.p67);
        let assign1330_e2291: f64 = (p.p76 / assign1330_e2290);
        let assign1330_e2294: f64 = (locals.var_wnew).powf(p.p68);
        let assign1330_e2295: f64 = (assign1330_e2291 / assign1330_e2294);
        let assign1330_e2296: f64 = (assign1330_e2286 + assign1330_e2295);
        locals.var_dwj = assign1330_e2296;
        locals.var_dwj_rv = 0.0;

        let assign1340_e2300: f64 = (2.0 * locals.var_dwj);
        let assign1340_e2301: f64 = (locals.var_wnew - assign1340_e2300);
        locals.var_weffcj = assign1340_e2301;
        locals.var_weffcj_rv = 0.0;

        let assign1360_e2307: f64 = (1e-6 / locals.var_leff);
        locals.var_inv_l = assign1360_e2307;
        locals.var_inv_l_rv = 0.0;

        let assign1370_e2310: f64 = (1e-6 / locals.var_weff);
        locals.var_inv_w = assign1370_e2310;
        locals.var_inv_w_rv = 0.0;

        let assign1380_e2313: f64 = (1e-6 / locals.var_lact);
        locals.var_inv_lact = assign1380_e2313;
        locals.var_inv_lact_rv = 0.0;

        let assign1390_e2316: f64 = (1e-6 / locals.var_wact);
        locals.var_inv_wact = assign1390_e2316;
        locals.var_inv_wact_rv = 0.0;

        let assign1400_e2319: f64 = (1e-6 / p.p51);
        locals.var_inv_llong = assign1400_e2319;
        locals.var_inv_llong_rv = 0.0;

        let assign1410_e2322: f64 = (1e-6 / p.p55);
        locals.var_inv_wwide = assign1410_e2322;
        locals.var_inv_wwide_rv = 0.0;

        let assign1420_e2325: f64 = (locals.var_inv_l * locals.var_inv_w);
        locals.var_inv_wl = assign1420_e2325;
        locals.var_inv_wl_rv = 0.0;

        locals.var_l_lln1 = locals.var_l_lln;
        locals.var_l_lln1_rv = 0.0;

        locals.var_l_wln1 = locals.var_l_wln;
        locals.var_l_wln1_rv = 0.0;

        let assign1450_e2330: f64 = if p.p818 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign1450_e2330;
        locals.var_guard14_rv = 0.0;

        let assign1460_e2333: f64 = (-locals.var_lnew);
        let assign1460_e2334: f64 = if p.p818 <= assign1460_e2333 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign1460_e2334;
        locals.var_guard15_rv = 0.0;

        let (assign1470_e2346,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard15 == 0.0)) {
        let assign1470_e2341: f64 = (locals.var_lnew + p.p818);
        let assign1470_e2343: f64 = (-p.p61);
        let assign1470_e2344: f64 = (assign1470_e2341).powf(assign1470_e2343);
        (assign1470_e2344,)
    } else {
        (locals.var_l_lln1,)
    }
};
        locals.var_l_lln1 = assign1470_e2346;
        locals.var_l_lln1_rv = 0.0;

        let (assign1480_e2358,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard15 == 0.0)) {
        let assign1480_e2353: f64 = (locals.var_lnew + p.p818);
        let assign1480_e2355: f64 = (-p.p67);
        let assign1480_e2356: f64 = (assign1480_e2353).powf(assign1480_e2355);
        (assign1480_e2356,)
    } else {
        (locals.var_l_wln1,)
    }
};
        locals.var_l_wln1 = assign1480_e2358;
        locals.var_l_wln1_rv = 0.0;

        locals.var_w_lwn1 = locals.var_w_lwn;
        locals.var_w_lwn1_rv = 0.0;

        locals.var_w_wwn1 = locals.var_w_wwn;
        locals.var_w_wwn1_rv = 0.0;

        let assign1510_e2363: f64 = if p.p819 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1510_e2363;
        locals.var_guard16_rv = 0.0;

        let assign1520_e2366: f64 = (-locals.var_wnew);
        let assign1520_e2367: f64 = if p.p819 <= assign1520_e2366 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign1520_e2367;
        locals.var_guard17_rv = 0.0;

        let (assign1530_e2379,) = {
    if ((locals.var_guard16 != 0.0) && (locals.var_guard17 == 0.0)) {
        let assign1530_e2374: f64 = (locals.var_wnew + p.p819);
        let assign1530_e2376: f64 = (-p.p62);
        let assign1530_e2377: f64 = (assign1530_e2374).powf(assign1530_e2376);
        (assign1530_e2377,)
    } else {
        (locals.var_w_lwn1,)
    }
};
        locals.var_w_lwn1 = assign1530_e2379;
        locals.var_w_lwn1_rv = 0.0;

        let (assign1540_e2391,) = {
    if ((locals.var_guard16 != 0.0) && (locals.var_guard17 == 0.0)) {
        let assign1540_e2386: f64 = (locals.var_wnew + p.p819);
        let assign1540_e2388: f64 = (-p.p68);
        let assign1540_e2389: f64 = (assign1540_e2386).powf(assign1540_e2388);
        (assign1540_e2389,)
    } else {
        (locals.var_w_wwn1,)
    }
};
        locals.var_w_wwn1 = assign1540_e2391;
        locals.var_w_wwn1_rv = 0.0;

        let assign1550_e2394: f64 = (locals.var_l_lln1 * locals.var_w_lwn1);
        locals.var_lw_lln_lwn1 = assign1550_e2394;
        locals.var_lw_lln_lwn1_rv = 0.0;

        let assign1560_e2398: f64 = (p.p58 * locals.var_l_lln1);
        let assign1560_e2399: f64 = (p.p57 + assign1560_e2398);
        let assign1560_e2402: f64 = (p.p59 * locals.var_w_lwn1);
        let assign1560_e2403: f64 = (assign1560_e2399 + assign1560_e2402);
        let assign1560_e2406: f64 = (p.p60 * locals.var_lw_lln_lwn1);
        let assign1560_e2407: f64 = (assign1560_e2403 + assign1560_e2406);
        locals.var_dlb = assign1560_e2407;
        locals.var_dlb_rv = 0.0;

        let assign1570_e2410: f64 = (locals.var_l_wln1 * locals.var_w_wwn1);
        locals.var_lw_wln_wwn1 = assign1570_e2410;
        locals.var_lw_wln_wwn1_rv = 0.0;

        let assign1580_e2414: f64 = (p.p64 * locals.var_l_wln1);
        let assign1580_e2415: f64 = (p.p63 + assign1580_e2414);
        let assign1580_e2418: f64 = (p.p65 * locals.var_w_wwn1);
        let assign1580_e2419: f64 = (assign1580_e2415 + assign1580_e2418);
        let assign1580_e2422: f64 = (p.p66 * locals.var_lw_wln_wwn1);
        let assign1580_e2423: f64 = (assign1580_e2419 + assign1580_e2422);
        locals.var_dwb = assign1580_e2423;
        locals.var_dwb_rv = 0.0;

        let assign1590_e2427: f64 = (2.0 * locals.var_dlb);
        let assign1590_e2428: f64 = (locals.var_lnew - assign1590_e2427);
        let assign1590_e2430: f64 = (assign1590_e2428 + p.p818);
        locals.var_leff1 = assign1590_e2430;
        locals.var_leff1_rv = 0.0;

        let assign1610_e2437: f64 = (2.0 * locals.var_dwb);
        let assign1610_e2438: f64 = (locals.var_wnew - assign1610_e2437);
        let assign1610_e2440: f64 = (assign1610_e2438 + p.p819);
        locals.var_weff1 = assign1610_e2440;
        locals.var_weff1_rv = 0.0;

        let assign1630_e2446: f64 = if p.p817 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard20 = assign1630_e2446;
        locals.var_guard20_rv = 0.0;

        let (assign1640_e2452,) = {
    if (locals.var_guard20 != 0.0) {
        let assign1640_e2450: f64 = (1e-6 / locals.var_leff1);
        (assign1640_e2450,)
    } else {
        (locals.var_bin_l,)
    }
};
        locals.var_bin_l = assign1640_e2452;
        locals.var_bin_l_rv = 0.0;

        let (assign1650_e2458,) = {
    if (locals.var_guard20 != 0.0) {
        let assign1650_e2456: f64 = (1e-6 / locals.var_weff1);
        (assign1650_e2456,)
    } else {
        (locals.var_bin_w,)
    }
};
        locals.var_bin_w = assign1650_e2458;
        locals.var_bin_w_rv = 0.0;

        let (assign1660_e2465,) = {
    if (locals.var_guard20 == 0.0) {
        let assign1660_e2463: f64 = (1.0 / locals.var_leff1);
        (assign1660_e2463,)
    } else {
        (locals.var_bin_l,)
    }
};
        locals.var_bin_l = assign1660_e2465;
        locals.var_bin_l_rv = 0.0;

        let (assign1670_e2472,) = {
    if (locals.var_guard20 == 0.0) {
        let assign1670_e2470: f64 = (1.0 / locals.var_weff1);
        (assign1670_e2470,)
    } else {
        (locals.var_bin_w,)
    }
};
        locals.var_bin_w = assign1670_e2472;
        locals.var_bin_w_rv = 0.0;

        let assign1680_e2475: f64 = (locals.var_bin_l * locals.var_bin_w);
        locals.var_bin_wl = assign1680_e2475;
        locals.var_bin_wl_rv = 0.0;

        let assign1690_e2479: f64 = (locals.var_bin_l * p.p117);
        let assign1690_e2480: f64 = (p.p116 + assign1690_e2479);
        let assign1690_e2483: f64 = (locals.var_bin_w * p.p118);
        let assign1690_e2484: f64 = (assign1690_e2480 + assign1690_e2483);
        let assign1690_e2487: f64 = (locals.var_bin_wl * p.p119);
        let assign1690_e2488: f64 = (assign1690_e2484 + assign1690_e2487);
        locals.var_vfb_i = assign1690_e2488;
        locals.var_vfb_i_dn0 = 0.0;
        locals.var_vfb_i_dn2 = 0.0;
        locals.var_vfb_i_dn3 = 0.0;
        locals.var_vfb_i_dn4 = 0.0;
        locals.var_vfb_i_dn5 = 0.0;
        locals.var_vfb_i_dn6 = 0.0;
        locals.var_vfb_i_dn7 = 0.0;
        locals.var_vfb_i_dn8 = 0.0;
        locals.var_vfb_i_dn9 = 0.0;
        locals.var_vfb_i_dn10 = 0.0;
        locals.var_vfb_i_dn11 = 0.0;
        locals.var_vfb_i_dn12 = 0.0;
        locals.var_vfb_i_dn13 = 0.0;
        locals.var_vfb_i_dn14 = 0.0;
        locals.var_vfb_i_rv = 0.0;

        let assign1700_e2492: f64 = (locals.var_bin_l * p.p127);
        let assign1700_e2493: f64 = (p.p126 + assign1700_e2492);
        let assign1700_e2496: f64 = (locals.var_bin_w * p.p128);
        let assign1700_e2497: f64 = (assign1700_e2493 + assign1700_e2496);
        let assign1700_e2500: f64 = (locals.var_bin_wl * p.p129);
        let assign1700_e2501: f64 = (assign1700_e2497 + assign1700_e2500);
        locals.var_vfbcv_i = assign1700_e2501;
        locals.var_vfbcv_i_dn0 = 0.0;
        locals.var_vfbcv_i_dn2 = 0.0;
        locals.var_vfbcv_i_dn3 = 0.0;
        locals.var_vfbcv_i_dn4 = 0.0;
        locals.var_vfbcv_i_dn5 = 0.0;
        locals.var_vfbcv_i_dn6 = 0.0;
        locals.var_vfbcv_i_dn7 = 0.0;
        locals.var_vfbcv_i_dn8 = 0.0;
        locals.var_vfbcv_i_dn9 = 0.0;
        locals.var_vfbcv_i_dn10 = 0.0;
        locals.var_vfbcv_i_dn11 = 0.0;
        locals.var_vfbcv_i_dn12 = 0.0;
        locals.var_vfbcv_i_dn13 = 0.0;
        locals.var_vfbcv_i_dn14 = 0.0;
        locals.var_vfbcv_i_rv = 0.0;

        let assign1710_e2505: f64 = (locals.var_bin_l * p.p140);
        let assign1710_e2506: f64 = (p.p139 + assign1710_e2505);
        let assign1710_e2509: f64 = (locals.var_bin_w * p.p141);
        let assign1710_e2510: f64 = (assign1710_e2506 + assign1710_e2509);
        let assign1710_e2513: f64 = (locals.var_bin_wl * p.p142);
        let assign1710_e2514: f64 = (assign1710_e2510 + assign1710_e2513);
        locals.var_nsd_i = assign1710_e2514;
        locals.var_nsd_i_rv = 0.0;

        let assign1720_e2518: f64 = (locals.var_bin_l * p.p89);
        let assign1720_e2519: f64 = (p.p80 + assign1720_e2518);
        let assign1720_e2522: f64 = (locals.var_bin_w * p.p90);
        let assign1720_e2523: f64 = (assign1720_e2519 + assign1720_e2522);
        let assign1720_e2526: f64 = (locals.var_bin_wl * p.p91);
        let assign1720_e2527: f64 = (assign1720_e2523 + assign1720_e2526);
        locals.var_ndep_i = assign1720_e2527;
        locals.var_ndep_i_dn0 = 0.0;
        locals.var_ndep_i_dn2 = 0.0;
        locals.var_ndep_i_dn3 = 0.0;
        locals.var_ndep_i_dn4 = 0.0;
        locals.var_ndep_i_dn5 = 0.0;
        locals.var_ndep_i_dn6 = 0.0;
        locals.var_ndep_i_dn7 = 0.0;
        locals.var_ndep_i_dn8 = 0.0;
        locals.var_ndep_i_dn9 = 0.0;
        locals.var_ndep_i_dn10 = 0.0;
        locals.var_ndep_i_dn11 = 0.0;
        locals.var_ndep_i_dn12 = 0.0;
        locals.var_ndep_i_dn13 = 0.0;
        locals.var_ndep_i_dn14 = 0.0;
        locals.var_ndep_i_rv = 0.0;

        let assign1730_e2531: f64 = (locals.var_bin_l * p.p101);
        let assign1730_e2532: f64 = (p.p92 + assign1730_e2531);
        let assign1730_e2535: f64 = (locals.var_bin_w * p.p102);
        let assign1730_e2536: f64 = (assign1730_e2532 + assign1730_e2535);
        let assign1730_e2539: f64 = (locals.var_bin_wl * p.p103);
        let assign1730_e2540: f64 = (assign1730_e2536 + assign1730_e2539);
        locals.var_ndepcv_i = assign1730_e2540;
        locals.var_ndepcv_i_dn0 = 0.0;
        locals.var_ndepcv_i_dn2 = 0.0;
        locals.var_ndepcv_i_dn3 = 0.0;
        locals.var_ndepcv_i_dn4 = 0.0;
        locals.var_ndepcv_i_dn5 = 0.0;
        locals.var_ndepcv_i_dn6 = 0.0;
        locals.var_ndepcv_i_dn7 = 0.0;
        locals.var_ndepcv_i_dn8 = 0.0;
        locals.var_ndepcv_i_dn9 = 0.0;
        locals.var_ndepcv_i_dn10 = 0.0;
        locals.var_ndepcv_i_dn11 = 0.0;
        locals.var_ndepcv_i_dn12 = 0.0;
        locals.var_ndepcv_i_dn13 = 0.0;
        locals.var_ndepcv_i_dn14 = 0.0;
        locals.var_ndepcv_i_rv = 0.0;

        let assign1740_e2544: f64 = (locals.var_bin_l * p.p105);
        let assign1740_e2545: f64 = (p.p104 + assign1740_e2544);
        let assign1740_e2548: f64 = (locals.var_bin_w * p.p106);
        let assign1740_e2549: f64 = (assign1740_e2545 + assign1740_e2548);
        let assign1740_e2552: f64 = (locals.var_bin_wl * p.p107);
        let assign1740_e2553: f64 = (assign1740_e2549 + assign1740_e2552);
        locals.var_ngate_i = assign1740_e2553;
        locals.var_ngate_i_rv = 0.0;

        let assign1750_e2557: f64 = (locals.var_bin_l * p.p210);
        let assign1750_e2558: f64 = (p.p209 + assign1750_e2557);
        let assign1750_e2561: f64 = (locals.var_bin_w * p.p211);
        let assign1750_e2562: f64 = (assign1750_e2558 + assign1750_e2561);
        let assign1750_e2565: f64 = (locals.var_bin_wl * p.p212);
        let assign1750_e2566: f64 = (assign1750_e2562 + assign1750_e2565);
        locals.var_cit_i = assign1750_e2566;
        locals.var_cit_i_rv = 0.0;

        let assign1760_e2570: f64 = (locals.var_bin_l * p.p220);
        let assign1760_e2571: f64 = (p.p213 + assign1760_e2570);
        let assign1760_e2574: f64 = (locals.var_bin_w * p.p221);
        let assign1760_e2575: f64 = (assign1760_e2571 + assign1760_e2574);
        let assign1760_e2578: f64 = (locals.var_bin_wl * p.p222);
        let assign1760_e2579: f64 = (assign1760_e2575 + assign1760_e2578);
        locals.var_nfactor_i = assign1760_e2579;
        locals.var_nfactor_i_dn0 = 0.0;
        locals.var_nfactor_i_dn2 = 0.0;
        locals.var_nfactor_i_dn3 = 0.0;
        locals.var_nfactor_i_dn4 = 0.0;
        locals.var_nfactor_i_dn5 = 0.0;
        locals.var_nfactor_i_dn6 = 0.0;
        locals.var_nfactor_i_dn7 = 0.0;
        locals.var_nfactor_i_dn8 = 0.0;
        locals.var_nfactor_i_dn9 = 0.0;
        locals.var_nfactor_i_dn10 = 0.0;
        locals.var_nfactor_i_dn11 = 0.0;
        locals.var_nfactor_i_dn12 = 0.0;
        locals.var_nfactor_i_dn13 = 0.0;
        locals.var_nfactor_i_dn14 = 0.0;
        locals.var_nfactor_i_rv = 0.0;

        let assign1770_e2583: f64 = (locals.var_bin_l * p.p226);
        let assign1770_e2584: f64 = (p.p223 + assign1770_e2583);
        let assign1770_e2587: f64 = (locals.var_bin_w * p.p227);
        let assign1770_e2588: f64 = (assign1770_e2584 + assign1770_e2587);
        let assign1770_e2591: f64 = (locals.var_bin_wl * p.p228);
        let assign1770_e2592: f64 = (assign1770_e2588 + assign1770_e2591);
        locals.var_cdscd_i = assign1770_e2592;
        locals.var_cdscd_i_dn0 = 0.0;
        locals.var_cdscd_i_dn2 = 0.0;
        locals.var_cdscd_i_dn3 = 0.0;
        locals.var_cdscd_i_dn4 = 0.0;
        locals.var_cdscd_i_dn5 = 0.0;
        locals.var_cdscd_i_dn6 = 0.0;
        locals.var_cdscd_i_dn7 = 0.0;
        locals.var_cdscd_i_dn8 = 0.0;
        locals.var_cdscd_i_dn9 = 0.0;
        locals.var_cdscd_i_dn10 = 0.0;
        locals.var_cdscd_i_dn11 = 0.0;
        locals.var_cdscd_i_dn12 = 0.0;
        locals.var_cdscd_i_dn13 = 0.0;
        locals.var_cdscd_i_dn14 = 0.0;
        locals.var_cdscd_i_rv = 0.0;

        let assign1780_e2596: f64 = (locals.var_bin_l * p.p236);
        let assign1780_e2597: f64 = (p.p233 + assign1780_e2596);
        let assign1780_e2600: f64 = (locals.var_bin_w * p.p237);
        let assign1780_e2601: f64 = (assign1780_e2597 + assign1780_e2600);
        let assign1780_e2604: f64 = (locals.var_bin_wl * p.p238);
        let assign1780_e2605: f64 = (assign1780_e2601 + assign1780_e2604);
        locals.var_cdscb_i = assign1780_e2605;
        locals.var_cdscb_i_rv = 0.0;

        let assign1790_e2609: f64 = (locals.var_bin_l * p.p144);
        let assign1790_e2610: f64 = (p.p143 + assign1790_e2609);
        let assign1790_e2613: f64 = (locals.var_bin_w * p.p145);
        let assign1790_e2614: f64 = (assign1790_e2610 + assign1790_e2613);
        let assign1790_e2617: f64 = (locals.var_bin_wl * p.p146);
        let assign1790_e2618: f64 = (assign1790_e2614 + assign1790_e2617);
        locals.var_dvtp0_i = assign1790_e2618;
        locals.var_dvtp0_i_rv = 0.0;

        let assign1800_e2622: f64 = (locals.var_bin_l * p.p148);
        let assign1800_e2623: f64 = (p.p147 + assign1800_e2622);
        let assign1800_e2626: f64 = (locals.var_bin_w * p.p149);
        let assign1800_e2627: f64 = (assign1800_e2623 + assign1800_e2626);
        let assign1800_e2630: f64 = (locals.var_bin_wl * p.p150);
        let assign1800_e2631: f64 = (assign1800_e2627 + assign1800_e2630);
        locals.var_dvtp1_i = assign1800_e2631;
        locals.var_dvtp1_i_rv = 0.0;

        let assign1810_e2635: f64 = (locals.var_bin_l * p.p152);
        let assign1810_e2636: f64 = (p.p151 + assign1810_e2635);
        let assign1810_e2639: f64 = (locals.var_bin_w * p.p153);
        let assign1810_e2640: f64 = (assign1810_e2636 + assign1810_e2639);
        let assign1810_e2643: f64 = (locals.var_bin_wl * p.p154);
        let assign1810_e2644: f64 = (assign1810_e2640 + assign1810_e2643);
        locals.var_dvtp2_i = assign1810_e2644;
        locals.var_dvtp2_i_rv = 0.0;

        let assign1820_e2648: f64 = (locals.var_bin_l * p.p156);
        let assign1820_e2649: f64 = (p.p155 + assign1820_e2648);
        let assign1820_e2652: f64 = (locals.var_bin_w * p.p157);
        let assign1820_e2653: f64 = (assign1820_e2649 + assign1820_e2652);
        let assign1820_e2656: f64 = (locals.var_bin_wl * p.p158);
        let assign1820_e2657: f64 = (assign1820_e2653 + assign1820_e2656);
        locals.var_dvtp3_i = assign1820_e2657;
        locals.var_dvtp3_i_rv = 0.0;

        let assign1830_e2661: f64 = (locals.var_bin_l * p.p160);
        let assign1830_e2662: f64 = (p.p159 + assign1830_e2661);
        let assign1830_e2665: f64 = (locals.var_bin_w * p.p161);
        let assign1830_e2666: f64 = (assign1830_e2662 + assign1830_e2665);
        let assign1830_e2669: f64 = (locals.var_bin_wl * p.p162);
        let assign1830_e2670: f64 = (assign1830_e2666 + assign1830_e2669);
        locals.var_dvtp4_i = assign1830_e2670;
        locals.var_dvtp4_i_rv = 0.0;

        let assign1840_e2674: f64 = (locals.var_bin_l * p.p164);
        let assign1840_e2675: f64 = (p.p163 + assign1840_e2674);
        let assign1840_e2678: f64 = (locals.var_bin_w * p.p165);
        let assign1840_e2679: f64 = (assign1840_e2675 + assign1840_e2678);
        let assign1840_e2682: f64 = (locals.var_bin_wl * p.p166);
        let assign1840_e2683: f64 = (assign1840_e2679 + assign1840_e2682);
        locals.var_dvtp5_i = assign1840_e2683;
        locals.var_dvtp5_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign1850_e2687: f64 = (locals.var_bin_l * p.p202);
        let assign1850_e2688: f64 = (p.p195 + assign1850_e2687);
        let assign1850_e2691: f64 = (locals.var_bin_w * p.p203);
        let assign1850_e2692: f64 = (assign1850_e2688 + assign1850_e2691);
        let assign1850_e2695: f64 = (locals.var_bin_wl * p.p204);
        let assign1850_e2696: f64 = (assign1850_e2692 + assign1850_e2695);
        locals.var_k2_i = assign1850_e2696;
        locals.var_k2_i_dn0 = 0.0;
        locals.var_k2_i_dn2 = 0.0;
        locals.var_k2_i_dn3 = 0.0;
        locals.var_k2_i_dn4 = 0.0;
        locals.var_k2_i_dn5 = 0.0;
        locals.var_k2_i_dn6 = 0.0;
        locals.var_k2_i_dn7 = 0.0;
        locals.var_k2_i_dn8 = 0.0;
        locals.var_k2_i_dn9 = 0.0;
        locals.var_k2_i_dn10 = 0.0;
        locals.var_k2_i_dn11 = 0.0;
        locals.var_k2_i_dn12 = 0.0;
        locals.var_k2_i_dn13 = 0.0;
        locals.var_k2_i_dn14 = 0.0;
        locals.var_k2_i_rv = 0.0;

        let assign1860_e2700: f64 = (locals.var_bin_l * p.p192);
        let assign1860_e2701: f64 = (p.p185 + assign1860_e2700);
        let assign1860_e2704: f64 = (locals.var_bin_w * p.p193);
        let assign1860_e2705: f64 = (assign1860_e2701 + assign1860_e2704);
        let assign1860_e2708: f64 = (locals.var_bin_wl * p.p194);
        let assign1860_e2709: f64 = (assign1860_e2705 + assign1860_e2708);
        locals.var_k1_i = assign1860_e2709;
        locals.var_k1_i_dn0 = 0.0;
        locals.var_k1_i_dn2 = 0.0;
        locals.var_k1_i_dn3 = 0.0;
        locals.var_k1_i_dn4 = 0.0;
        locals.var_k1_i_dn5 = 0.0;
        locals.var_k1_i_dn6 = 0.0;
        locals.var_k1_i_dn7 = 0.0;
        locals.var_k1_i_dn8 = 0.0;
        locals.var_k1_i_dn9 = 0.0;
        locals.var_k1_i_dn10 = 0.0;
        locals.var_k1_i_dn11 = 0.0;
        locals.var_k1_i_dn12 = 0.0;
        locals.var_k1_i_dn13 = 0.0;
        locals.var_k1_i_dn14 = 0.0;
        locals.var_k1_i_rv = 0.0;

        let assign1870_e2713: f64 = (locals.var_bin_l * p.p113);
        let assign1870_e2714: f64 = (p.p112 + assign1870_e2713);
        let assign1870_e2717: f64 = (locals.var_bin_w * p.p114);
        let assign1870_e2718: f64 = (assign1870_e2714 + assign1870_e2717);
        let assign1870_e2721: f64 = (locals.var_bin_wl * p.p115);
        let assign1870_e2722: f64 = (assign1870_e2718 + assign1870_e2721);
        locals.var_xj_i = assign1870_e2722;
        locals.var_xj_i_rv = 0.0;

        let assign1880_e2726: f64 = (locals.var_bin_l * p.p168);
        let assign1880_e2727: f64 = (p.p167 + assign1880_e2726);
        let assign1880_e2730: f64 = (locals.var_bin_w * p.p169);
        let assign1880_e2731: f64 = (assign1880_e2727 + assign1880_e2730);
        let assign1880_e2734: f64 = (locals.var_bin_wl * p.p170);
        let assign1880_e2735: f64 = (assign1880_e2731 + assign1880_e2734);
        locals.var_phin_i = assign1880_e2735;
        locals.var_phin_i_rv = 0.0;

        let assign1890_e2739: f64 = (locals.var_bin_l * p.p172);
        let assign1890_e2740: f64 = (p.p171 + assign1890_e2739);
        let assign1890_e2743: f64 = (locals.var_bin_w * p.p173);
        let assign1890_e2744: f64 = (assign1890_e2740 + assign1890_e2743);
        let assign1890_e2747: f64 = (locals.var_bin_wl * p.p174);
        let assign1890_e2748: f64 = (assign1890_e2744 + assign1890_e2747);
        locals.var_eta0_i = assign1890_e2748;
        locals.var_eta0_i_dn0 = 0.0;
        locals.var_eta0_i_dn2 = 0.0;
        locals.var_eta0_i_dn3 = 0.0;
        locals.var_eta0_i_dn4 = 0.0;
        locals.var_eta0_i_dn5 = 0.0;
        locals.var_eta0_i_dn6 = 0.0;
        locals.var_eta0_i_dn7 = 0.0;
        locals.var_eta0_i_dn8 = 0.0;
        locals.var_eta0_i_dn9 = 0.0;
        locals.var_eta0_i_dn10 = 0.0;
        locals.var_eta0_i_dn11 = 0.0;
        locals.var_eta0_i_dn12 = 0.0;
        locals.var_eta0_i_dn13 = 0.0;
        locals.var_eta0_i_dn14 = 0.0;
        locals.var_eta0_i_rv = 0.0;

        let assign1900_e2752: f64 = (locals.var_bin_l * p.p182);
        let assign1900_e2753: f64 = (p.p180 + assign1900_e2752);
        let assign1900_e2756: f64 = (locals.var_bin_w * p.p183);
        let assign1900_e2757: f64 = (assign1900_e2753 + assign1900_e2756);
        let assign1900_e2760: f64 = (locals.var_bin_wl * p.p184);
        let assign1900_e2761: f64 = (assign1900_e2757 + assign1900_e2760);
        locals.var_etab_i = assign1900_e2761;
        locals.var_etab_i_rv = 0.0;

        let assign1910_e2765: f64 = (locals.var_bin_l * p.p254);
        let assign1910_e2766: f64 = (p.p253 + assign1910_e2765);
        let assign1910_e2769: f64 = (locals.var_bin_w * p.p255);
        let assign1910_e2770: f64 = (assign1910_e2766 + assign1910_e2769);
        let assign1910_e2773: f64 = (locals.var_bin_wl * p.p256);
        let assign1910_e2774: f64 = (assign1910_e2770 + assign1910_e2773);
        locals.var_delta_i = assign1910_e2774;
        locals.var_delta_i_dn0 = 0.0;
        locals.var_delta_i_dn2 = 0.0;
        locals.var_delta_i_dn3 = 0.0;
        locals.var_delta_i_dn4 = 0.0;
        locals.var_delta_i_dn5 = 0.0;
        locals.var_delta_i_dn6 = 0.0;
        locals.var_delta_i_dn7 = 0.0;
        locals.var_delta_i_dn8 = 0.0;
        locals.var_delta_i_dn9 = 0.0;
        locals.var_delta_i_dn10 = 0.0;
        locals.var_delta_i_dn11 = 0.0;
        locals.var_delta_i_dn12 = 0.0;
        locals.var_delta_i_dn13 = 0.0;
        locals.var_delta_i_dn14 = 0.0;
        locals.var_delta_i_rv = 0.0;

        let assign1920_e2778: f64 = (locals.var_bin_l * p.p276);
        let assign1920_e2779: f64 = (p.p273 + assign1920_e2778);
        let assign1920_e2782: f64 = (locals.var_bin_w * p.p277);
        let assign1920_e2783: f64 = (assign1920_e2779 + assign1920_e2782);
        let assign1920_e2786: f64 = (locals.var_bin_wl * p.p278);
        let assign1920_e2787: f64 = (assign1920_e2783 + assign1920_e2786);
        locals.var_u0_i = assign1920_e2787;
        locals.var_u0_i_rv = 0.0;

        let assign1930_e2791: f64 = (locals.var_bin_l * p.p291);
        let assign1930_e2792: f64 = (p.p284 + assign1930_e2791);
        let assign1930_e2795: f64 = (locals.var_bin_w * p.p292);
        let assign1930_e2796: f64 = (assign1930_e2792 + assign1930_e2795);
        let assign1930_e2799: f64 = (locals.var_bin_wl * p.p293);
        let assign1930_e2800: f64 = (assign1930_e2796 + assign1930_e2799);
        locals.var_ua_i = assign1930_e2800;
        locals.var_ua_i_dn0 = 0.0;
        locals.var_ua_i_dn2 = 0.0;
        locals.var_ua_i_dn3 = 0.0;
        locals.var_ua_i_dn4 = 0.0;
        locals.var_ua_i_dn5 = 0.0;
        locals.var_ua_i_dn6 = 0.0;
        locals.var_ua_i_dn7 = 0.0;
        locals.var_ua_i_dn8 = 0.0;
        locals.var_ua_i_dn9 = 0.0;
        locals.var_ua_i_dn10 = 0.0;
        locals.var_ua_i_dn11 = 0.0;
        locals.var_ua_i_dn12 = 0.0;
        locals.var_ua_i_dn13 = 0.0;
        locals.var_ua_i_dn14 = 0.0;
        locals.var_ua_i_rv = 0.0;

        let assign1940_e2804: f64 = (locals.var_bin_l * p.p311);
        let assign1940_e2805: f64 = (p.p308 + assign1940_e2804);
        let assign1940_e2808: f64 = (locals.var_bin_w * p.p312);
        let assign1940_e2809: f64 = (assign1940_e2805 + assign1940_e2808);
        let assign1940_e2812: f64 = (locals.var_bin_wl * p.p313);
        let assign1940_e2813: f64 = (assign1940_e2809 + assign1940_e2812);
        locals.var_ud_i = assign1940_e2813;
        locals.var_ud_i_dn0 = 0.0;
        locals.var_ud_i_dn2 = 0.0;
        locals.var_ud_i_dn3 = 0.0;
        locals.var_ud_i_dn4 = 0.0;
        locals.var_ud_i_dn5 = 0.0;
        locals.var_ud_i_dn6 = 0.0;
        locals.var_ud_i_dn7 = 0.0;
        locals.var_ud_i_dn8 = 0.0;
        locals.var_ud_i_dn9 = 0.0;
        locals.var_ud_i_dn10 = 0.0;
        locals.var_ud_i_dn11 = 0.0;
        locals.var_ud_i_dn12 = 0.0;
        locals.var_ud_i_dn13 = 0.0;
        locals.var_ud_i_dn14 = 0.0;
        locals.var_ud_i_rv = 0.0;

        let assign1950_e2817: f64 = (locals.var_bin_l * p.p299);
        let assign1950_e2818: f64 = (p.p298 + assign1950_e2817);
        let assign1950_e2821: f64 = (locals.var_bin_w * p.p300);
        let assign1950_e2822: f64 = (assign1950_e2818 + assign1950_e2821);
        let assign1950_e2825: f64 = (locals.var_bin_wl * p.p301);
        let assign1950_e2826: f64 = (assign1950_e2822 + assign1950_e2825);
        locals.var_eu_i = assign1950_e2826;
        locals.var_eu_i_dn0 = 0.0;
        locals.var_eu_i_dn2 = 0.0;
        locals.var_eu_i_dn3 = 0.0;
        locals.var_eu_i_dn4 = 0.0;
        locals.var_eu_i_dn5 = 0.0;
        locals.var_eu_i_dn6 = 0.0;
        locals.var_eu_i_dn7 = 0.0;
        locals.var_eu_i_dn8 = 0.0;
        locals.var_eu_i_dn9 = 0.0;
        locals.var_eu_i_dn10 = 0.0;
        locals.var_eu_i_dn11 = 0.0;
        locals.var_eu_i_dn12 = 0.0;
        locals.var_eu_i_dn13 = 0.0;
        locals.var_eu_i_dn14 = 0.0;
        locals.var_eu_i_rv = 0.0;

        let assign1960_e2830: f64 = (locals.var_bin_l * p.p319);
        let assign1960_e2831: f64 = (p.p318 + assign1960_e2830);
        let assign1960_e2834: f64 = (locals.var_bin_w * p.p320);
        let assign1960_e2835: f64 = (assign1960_e2831 + assign1960_e2834);
        let assign1960_e2838: f64 = (locals.var_bin_wl * p.p321);
        let assign1960_e2839: f64 = (assign1960_e2835 + assign1960_e2838);
        locals.var_ucs_i = assign1960_e2839;
        locals.var_ucs_i_rv = 0.0;

        let assign1970_e2843: f64 = (locals.var_bin_l * p.p333);
        let assign1970_e2844: f64 = (p.p326 + assign1970_e2843);
        let assign1970_e2847: f64 = (locals.var_bin_w * p.p334);
        let assign1970_e2848: f64 = (assign1970_e2844 + assign1970_e2847);
        let assign1970_e2851: f64 = (locals.var_bin_wl * p.p335);
        let assign1970_e2852: f64 = (assign1970_e2848 + assign1970_e2851);
        locals.var_uc_i = assign1970_e2852;
        locals.var_uc_i_dn0 = 0.0;
        locals.var_uc_i_dn2 = 0.0;
        locals.var_uc_i_dn3 = 0.0;
        locals.var_uc_i_dn4 = 0.0;
        locals.var_uc_i_dn5 = 0.0;
        locals.var_uc_i_dn6 = 0.0;
        locals.var_uc_i_dn7 = 0.0;
        locals.var_uc_i_dn8 = 0.0;
        locals.var_uc_i_dn9 = 0.0;
        locals.var_uc_i_dn10 = 0.0;
        locals.var_uc_i_dn11 = 0.0;
        locals.var_uc_i_dn12 = 0.0;
        locals.var_uc_i_dn13 = 0.0;
        locals.var_uc_i_dn14 = 0.0;
        locals.var_uc_i_rv = 0.0;

        let assign1980_e2856: f64 = (locals.var_bin_l * p.p343);
        let assign1980_e2857: f64 = (p.p340 + assign1980_e2856);
        let assign1980_e2860: f64 = (locals.var_bin_w * p.p344);
        let assign1980_e2861: f64 = (assign1980_e2857 + assign1980_e2860);
        let assign1980_e2864: f64 = (locals.var_bin_wl * p.p345);
        let assign1980_e2865: f64 = (assign1980_e2861 + assign1980_e2864);
        locals.var_pclm_i = assign1980_e2865;
        locals.var_pclm_i_dn0 = 0.0;
        locals.var_pclm_i_dn2 = 0.0;
        locals.var_pclm_i_dn3 = 0.0;
        locals.var_pclm_i_dn4 = 0.0;
        locals.var_pclm_i_dn5 = 0.0;
        locals.var_pclm_i_dn6 = 0.0;
        locals.var_pclm_i_dn7 = 0.0;
        locals.var_pclm_i_dn8 = 0.0;
        locals.var_pclm_i_dn9 = 0.0;
        locals.var_pclm_i_dn10 = 0.0;
        locals.var_pclm_i_dn11 = 0.0;
        locals.var_pclm_i_dn12 = 0.0;
        locals.var_pclm_i_dn13 = 0.0;
        locals.var_pclm_i_dn14 = 0.0;
        locals.var_pclm_i_rv = 0.0;

        let assign1990_e2869: f64 = (locals.var_bin_l * p.p354);
        let assign1990_e2870: f64 = (p.p351 + assign1990_e2869);
        let assign1990_e2873: f64 = (locals.var_bin_w * p.p355);
        let assign1990_e2874: f64 = (assign1990_e2870 + assign1990_e2873);
        let assign1990_e2877: f64 = (locals.var_bin_wl * p.p356);
        let assign1990_e2878: f64 = (assign1990_e2874 + assign1990_e2877);
        locals.var_pclmcv_i = assign1990_e2878;
        locals.var_pclmcv_i_rv = 0.0;

        let assign2000_e2882: f64 = (locals.var_bin_l * p.p394);
        let assign2000_e2883: f64 = (p.p393 + assign2000_e2882);
        let assign2000_e2886: f64 = (locals.var_bin_w * p.p395);
        let assign2000_e2887: f64 = (assign2000_e2883 + assign2000_e2886);
        let assign2000_e2890: f64 = (locals.var_bin_wl * p.p396);
        let assign2000_e2891: f64 = (assign2000_e2887 + assign2000_e2890);
        locals.var_rsw_i = assign2000_e2891;
        locals.var_rsw_i_rv = 0.0;

        let assign2010_e2895: f64 = (locals.var_bin_l * p.p404);
        let assign2010_e2896: f64 = (p.p403 + assign2010_e2895);
        let assign2010_e2899: f64 = (locals.var_bin_w * p.p405);
        let assign2010_e2900: f64 = (assign2010_e2896 + assign2010_e2899);
        let assign2010_e2903: f64 = (locals.var_bin_wl * p.p406);
        let assign2010_e2904: f64 = (assign2010_e2900 + assign2010_e2903);
        locals.var_rdw_i = assign2010_e2904;
        locals.var_rdw_i_rv = 0.0;

        let assign2020_e2908: f64 = (locals.var_bin_l * p.p376);
        let assign2020_e2909: f64 = (p.p375 + assign2020_e2908);
        let assign2020_e2912: f64 = (locals.var_bin_w * p.p377);
        let assign2020_e2913: f64 = (assign2020_e2909 + assign2020_e2912);
        let assign2020_e2916: f64 = (locals.var_bin_wl * p.p378);
        let assign2020_e2917: f64 = (assign2020_e2913 + assign2020_e2916);
        locals.var_prwg_i = assign2020_e2917;
        locals.var_prwg_i_rv = 0.0;

        let assign2030_e2921: f64 = (locals.var_bin_l * p.p380);
        let assign2030_e2922: f64 = (p.p379 + assign2030_e2921);
        let assign2030_e2925: f64 = (locals.var_bin_w * p.p381);
        let assign2030_e2926: f64 = (assign2030_e2922 + assign2030_e2925);
        let assign2030_e2929: f64 = (locals.var_bin_wl * p.p382);
        let assign2030_e2930: f64 = (assign2030_e2926 + assign2030_e2929);
        locals.var_prwb_i = assign2030_e2930;
        locals.var_prwb_i_rv = 0.0;

        let assign2040_e2934: f64 = (locals.var_bin_l * p.p386);
        let assign2040_e2935: f64 = (p.p385 + assign2040_e2934);
        let assign2040_e2938: f64 = (locals.var_bin_w * p.p387);
        let assign2040_e2939: f64 = (assign2040_e2935 + assign2040_e2938);
        let assign2040_e2942: f64 = (locals.var_bin_wl * p.p388);
        let assign2040_e2943: f64 = (assign2040_e2939 + assign2040_e2942);
        locals.var_wr_i = assign2040_e2943;
        locals.var_wr_i_rv = 0.0;

        let assign2050_e2947: f64 = (locals.var_bin_l * p.p390);
        let assign2050_e2948: f64 = (p.p389 + assign2050_e2947);
        let assign2050_e2951: f64 = (locals.var_bin_w * p.p391);
        let assign2050_e2952: f64 = (assign2050_e2948 + assign2050_e2951);
        let assign2050_e2955: f64 = (locals.var_bin_wl * p.p392);
        let assign2050_e2956: f64 = (assign2050_e2952 + assign2050_e2955);
        locals.var_rswmin_i = assign2050_e2956;
        locals.var_rswmin_i_rv = 0.0;

        let assign2060_e2960: f64 = (locals.var_bin_l * p.p400);
        let assign2060_e2961: f64 = (p.p399 + assign2060_e2960);
        let assign2060_e2964: f64 = (locals.var_bin_w * p.p401);
        let assign2060_e2965: f64 = (assign2060_e2961 + assign2060_e2964);
        let assign2060_e2968: f64 = (locals.var_bin_wl * p.p402);
        let assign2060_e2969: f64 = (assign2060_e2965 + assign2060_e2968);
        locals.var_rdwmin_i = assign2060_e2969;
        locals.var_rdwmin_i_rv = 0.0;

        let assign2070_e2973: f64 = (locals.var_bin_l * p.p416);
        let assign2070_e2974: f64 = (p.p413 + assign2070_e2973);
        let assign2070_e2977: f64 = (locals.var_bin_w * p.p417);
        let assign2070_e2978: f64 = (assign2070_e2974 + assign2070_e2977);
        let assign2070_e2981: f64 = (locals.var_bin_wl * p.p418);
        let assign2070_e2982: f64 = (assign2070_e2978 + assign2070_e2981);
        locals.var_rdsw_i = assign2070_e2982;
        locals.var_rdsw_i_rv = 0.0;

        let assign2080_e2986: f64 = (locals.var_bin_l * p.p410);
        let assign2080_e2987: f64 = (p.p409 + assign2080_e2986);
        let assign2080_e2990: f64 = (locals.var_bin_w * p.p411);
        let assign2080_e2991: f64 = (assign2080_e2987 + assign2080_e2990);
        let assign2080_e2994: f64 = (locals.var_bin_wl * p.p412);
        let assign2080_e2995: f64 = (assign2080_e2991 + assign2080_e2994);
        locals.var_rdswmin_i = assign2080_e2995;
        locals.var_rdswmin_i_rv = 0.0;

        let assign2090_e2999: f64 = (locals.var_bin_l * p.p435);
        let assign2090_e3000: f64 = (p.p434 + assign2090_e2999);
        let assign2090_e3003: f64 = (locals.var_bin_w * p.p436);
        let assign2090_e3004: f64 = (assign2090_e3000 + assign2090_e3003);
        let assign2090_e3007: f64 = (locals.var_bin_wl * p.p437);
        let assign2090_e3008: f64 = (assign2090_e3004 + assign2090_e3007);
        locals.var_ptwg_i = assign2090_e3008;
        locals.var_ptwg_i_dn0 = 0.0;
        locals.var_ptwg_i_dn2 = 0.0;
        locals.var_ptwg_i_dn3 = 0.0;
        locals.var_ptwg_i_dn4 = 0.0;
        locals.var_ptwg_i_dn5 = 0.0;
        locals.var_ptwg_i_dn6 = 0.0;
        locals.var_ptwg_i_dn7 = 0.0;
        locals.var_ptwg_i_dn8 = 0.0;
        locals.var_ptwg_i_dn9 = 0.0;
        locals.var_ptwg_i_dn10 = 0.0;
        locals.var_ptwg_i_dn11 = 0.0;
        locals.var_ptwg_i_dn12 = 0.0;
        locals.var_ptwg_i_dn13 = 0.0;
        locals.var_ptwg_i_dn14 = 0.0;
        locals.var_ptwg_i_rv = 0.0;

        let assign2100_e3012: f64 = (locals.var_bin_l * p.p463);
        let assign2100_e3013: f64 = (p.p460 + assign2100_e3012);
        let assign2100_e3016: f64 = (locals.var_bin_w * p.p464);
        let assign2100_e3017: f64 = (assign2100_e3013 + assign2100_e3016);
        let assign2100_e3020: f64 = (locals.var_bin_wl * p.p465);
        let assign2100_e3021: f64 = (assign2100_e3017 + assign2100_e3020);
        locals.var_pdiblc_i = assign2100_e3021;
        locals.var_pdiblc_i_dn0 = 0.0;
        locals.var_pdiblc_i_dn2 = 0.0;
        locals.var_pdiblc_i_dn3 = 0.0;
        locals.var_pdiblc_i_dn4 = 0.0;
        locals.var_pdiblc_i_dn5 = 0.0;
        locals.var_pdiblc_i_dn6 = 0.0;
        locals.var_pdiblc_i_dn7 = 0.0;
        locals.var_pdiblc_i_dn8 = 0.0;
        locals.var_pdiblc_i_dn9 = 0.0;
        locals.var_pdiblc_i_dn10 = 0.0;
        locals.var_pdiblc_i_dn11 = 0.0;
        locals.var_pdiblc_i_dn12 = 0.0;
        locals.var_pdiblc_i_dn13 = 0.0;
        locals.var_pdiblc_i_dn14 = 0.0;
        locals.var_pdiblc_i_rv = 0.0;

        let assign2110_e3025: f64 = (locals.var_bin_l * p.p471);
        let assign2110_e3026: f64 = (p.p470 + assign2110_e3025);
        let assign2110_e3029: f64 = (locals.var_bin_w * p.p472);
        let assign2110_e3030: f64 = (assign2110_e3026 + assign2110_e3029);
        let assign2110_e3033: f64 = (locals.var_bin_wl * p.p473);
        let assign2110_e3034: f64 = (assign2110_e3030 + assign2110_e3033);
        locals.var_pdiblcb_i = assign2110_e3034;
        locals.var_pdiblcb_i_rv = 0.0;

        let assign2120_e3038: f64 = (locals.var_bin_l * p.p358);
        let assign2120_e3039: f64 = (p.p357 + assign2120_e3038);
        let assign2120_e3042: f64 = (locals.var_bin_w * p.p359);
        let assign2120_e3043: f64 = (assign2120_e3039 + assign2120_e3042);
        let assign2120_e3046: f64 = (locals.var_bin_wl * p.p360);
        let assign2120_e3047: f64 = (assign2120_e3043 + assign2120_e3046);
        locals.var_pscbe1_i = assign2120_e3047;
        locals.var_pscbe1_i_rv = 0.0;

        let assign2130_e3051: f64 = (locals.var_bin_l * p.p362);
        let assign2130_e3052: f64 = (p.p361 + assign2130_e3051);
        let assign2130_e3055: f64 = (locals.var_bin_w * p.p363);
        let assign2130_e3056: f64 = (assign2130_e3052 + assign2130_e3055);
        let assign2130_e3059: f64 = (locals.var_bin_wl * p.p364);
        let assign2130_e3060: f64 = (assign2130_e3056 + assign2130_e3059);
        locals.var_pscbe2_i = assign2130_e3060;
        locals.var_pscbe2_i_rv = 0.0;

        let assign2140_e3064: f64 = (locals.var_bin_l * p.p366);
        let assign2140_e3065: f64 = (p.p365 + assign2140_e3064);
        let assign2140_e3068: f64 = (locals.var_bin_w * p.p367);
        let assign2140_e3069: f64 = (assign2140_e3065 + assign2140_e3068);
        let assign2140_e3072: f64 = (locals.var_bin_wl * p.p368);
        let assign2140_e3073: f64 = (assign2140_e3069 + assign2140_e3072);
        locals.var_pdits_i = assign2140_e3073;
        locals.var_pdits_i_rv = 0.0;

        let assign2150_e3077: f64 = (locals.var_bin_l * p.p371);
        let assign2150_e3078: f64 = (p.p370 + assign2150_e3077);
        let assign2150_e3081: f64 = (locals.var_bin_w * p.p372);
        let assign2150_e3082: f64 = (assign2150_e3078 + assign2150_e3081);
        let assign2150_e3085: f64 = (locals.var_bin_wl * p.p373);
        let assign2150_e3086: f64 = (assign2150_e3082 + assign2150_e3085);
        locals.var_pditsd_i = assign2150_e3086;
        locals.var_pditsd_i_rv = 0.0;

        let assign2160_e3090: f64 = (locals.var_bin_l * p.p481);
        let assign2160_e3091: f64 = (p.p478 + assign2160_e3090);
        let assign2160_e3094: f64 = (locals.var_bin_w * p.p482);
        let assign2160_e3095: f64 = (assign2160_e3091 + assign2160_e3094);
        let assign2160_e3098: f64 = (locals.var_bin_wl * p.p483);
        let assign2160_e3099: f64 = (assign2160_e3095 + assign2160_e3098);
        locals.var_fprout_i = assign2160_e3099;
        locals.var_fprout_i_rv = 0.0;

        let assign2170_e3103: f64 = (locals.var_bin_l * p.p475);
        let assign2170_e3104: f64 = (p.p474 + assign2170_e3103);
        let assign2170_e3107: f64 = (locals.var_bin_w * p.p476);
        let assign2170_e3108: f64 = (assign2170_e3104 + assign2170_e3107);
        let assign2170_e3111: f64 = (locals.var_bin_wl * p.p477);
        let assign2170_e3112: f64 = (assign2170_e3108 + assign2170_e3111);
        locals.var_pvag_i = assign2170_e3112;
        locals.var_pvag_i_rv = 0.0;

        let assign2180_e3116: f64 = (locals.var_bin_l * p.p240);
        let assign2180_e3117: f64 = (p.p239 + assign2180_e3116);
        let assign2180_e3120: f64 = (locals.var_bin_w * p.p241);
        let assign2180_e3121: f64 = (assign2180_e3117 + assign2180_e3120);
        let assign2180_e3124: f64 = (locals.var_bin_wl * p.p242);
        let assign2180_e3125: f64 = (assign2180_e3121 + assign2180_e3124);
        locals.var_vsat_i = assign2180_e3125;
        locals.var_vsat_i_dn0 = 0.0;
        locals.var_vsat_i_dn2 = 0.0;
        locals.var_vsat_i_dn3 = 0.0;
        locals.var_vsat_i_dn4 = 0.0;
        locals.var_vsat_i_dn5 = 0.0;
        locals.var_vsat_i_dn6 = 0.0;
        locals.var_vsat_i_dn7 = 0.0;
        locals.var_vsat_i_dn8 = 0.0;
        locals.var_vsat_i_dn9 = 0.0;
        locals.var_vsat_i_dn10 = 0.0;
        locals.var_vsat_i_dn11 = 0.0;
        locals.var_vsat_i_dn12 = 0.0;
        locals.var_vsat_i_dn13 = 0.0;
        locals.var_vsat_i_dn14 = 0.0;
        locals.var_vsat_i_rv = 0.0;

        let assign2190_e3129: f64 = (locals.var_bin_l * p.p420);
        let assign2190_e3130: f64 = (p.p419 + assign2190_e3129);
        let assign2190_e3133: f64 = (locals.var_bin_w * p.p421);
        let assign2190_e3134: f64 = (assign2190_e3130 + assign2190_e3133);
        let assign2190_e3137: f64 = (locals.var_bin_wl * p.p422);
        let assign2190_e3138: f64 = (assign2190_e3134 + assign2190_e3137);
        locals.var_psat_i = assign2190_e3138;
        locals.var_psat_i_rv = 0.0;

        let assign2200_e3142: f64 = (locals.var_bin_l * p.p260);
        let assign2200_e3143: f64 = (p.p259 + assign2200_e3142);
        let assign2200_e3146: f64 = (locals.var_bin_w * p.p261);
        let assign2200_e3147: f64 = (assign2200_e3143 + assign2200_e3146);
        let assign2200_e3150: f64 = (locals.var_bin_wl * p.p262);
        let assign2200_e3151: f64 = (assign2200_e3147 + assign2200_e3150);
        locals.var_vsatcv_i = assign2200_e3151;
        locals.var_vsatcv_i_dn0 = 0.0;
        locals.var_vsatcv_i_dn2 = 0.0;
        locals.var_vsatcv_i_dn3 = 0.0;
        locals.var_vsatcv_i_dn4 = 0.0;
        locals.var_vsatcv_i_dn5 = 0.0;
        locals.var_vsatcv_i_dn6 = 0.0;
        locals.var_vsatcv_i_dn7 = 0.0;
        locals.var_vsatcv_i_dn8 = 0.0;
        locals.var_vsatcv_i_dn9 = 0.0;
        locals.var_vsatcv_i_dn10 = 0.0;
        locals.var_vsatcv_i_dn11 = 0.0;
        locals.var_vsatcv_i_dn12 = 0.0;
        locals.var_vsatcv_i_dn13 = 0.0;
        locals.var_vsatcv_i_dn14 = 0.0;
        locals.var_vsatcv_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign2210_e3155: f64 = (locals.var_bin_l * p.p667);
        let assign2210_e3156: f64 = (p.p666 + assign2210_e3155);
        let assign2210_e3159: f64 = (locals.var_bin_w * p.p668);
        let assign2210_e3160: f64 = (assign2210_e3156 + assign2210_e3159);
        let assign2210_e3163: f64 = (locals.var_bin_wl * p.p669);
        let assign2210_e3164: f64 = (assign2210_e3160 + assign2210_e3163);
        locals.var_cf_i = assign2210_e3164;
        locals.var_cf_i_rv = 0.0;

        let assign2220_e3168: f64 = (locals.var_bin_l * p.p675);
        let assign2220_e3169: f64 = (p.p674 + assign2220_e3168);
        let assign2220_e3172: f64 = (locals.var_bin_w * p.p676);
        let assign2220_e3173: f64 = (assign2220_e3169 + assign2220_e3172);
        let assign2220_e3176: f64 = (locals.var_bin_wl * p.p677);
        let assign2220_e3177: f64 = (assign2220_e3173 + assign2220_e3176);
        locals.var_cgsl_i = assign2220_e3177;
        locals.var_cgsl_i_rv = 0.0;

        let assign2230_e3181: f64 = (locals.var_bin_l * p.p679);
        let assign2230_e3182: f64 = (p.p678 + assign2230_e3181);
        let assign2230_e3185: f64 = (locals.var_bin_w * p.p680);
        let assign2230_e3186: f64 = (assign2230_e3182 + assign2230_e3185);
        let assign2230_e3189: f64 = (locals.var_bin_wl * p.p681);
        let assign2230_e3190: f64 = (assign2230_e3186 + assign2230_e3189);
        locals.var_cgdl_i = assign2230_e3190;
        locals.var_cgdl_i_rv = 0.0;

        let assign2240_e3194: f64 = (locals.var_bin_l * p.p683);
        let assign2240_e3195: f64 = (p.p682 + assign2240_e3194);
        let assign2240_e3198: f64 = (locals.var_bin_w * p.p684);
        let assign2240_e3199: f64 = (assign2240_e3195 + assign2240_e3198);
        let assign2240_e3202: f64 = (locals.var_bin_wl * p.p685);
        let assign2240_e3203: f64 = (assign2240_e3199 + assign2240_e3202);
        locals.var_ckappas_i = assign2240_e3203;
        locals.var_ckappas_i_rv = 0.0;

        let assign2250_e3207: f64 = (locals.var_bin_l * p.p687);
        let assign2250_e3208: f64 = (p.p686 + assign2250_e3207);
        let assign2250_e3211: f64 = (locals.var_bin_w * p.p688);
        let assign2250_e3212: f64 = (assign2250_e3208 + assign2250_e3211);
        let assign2250_e3215: f64 = (locals.var_bin_wl * p.p689);
        let assign2250_e3216: f64 = (assign2250_e3212 + assign2250_e3215);
        locals.var_ckappad_i = assign2250_e3216;
        locals.var_ckappad_i_rv = 0.0;

        let assign2260_e3220: f64 = (locals.var_bin_l * p.p489);
        let assign2260_e3221: f64 = (p.p484 + assign2260_e3220);
        let assign2260_e3224: f64 = (locals.var_bin_w * p.p490);
        let assign2260_e3225: f64 = (assign2260_e3221 + assign2260_e3224);
        let assign2260_e3228: f64 = (locals.var_bin_wl * p.p491);
        let assign2260_e3229: f64 = (assign2260_e3225 + assign2260_e3228);
        locals.var_alpha0_i = assign2260_e3229;
        locals.var_alpha0_i_dn0 = 0.0;
        locals.var_alpha0_i_dn2 = 0.0;
        locals.var_alpha0_i_dn3 = 0.0;
        locals.var_alpha0_i_dn4 = 0.0;
        locals.var_alpha0_i_dn5 = 0.0;
        locals.var_alpha0_i_dn6 = 0.0;
        locals.var_alpha0_i_dn7 = 0.0;
        locals.var_alpha0_i_dn8 = 0.0;
        locals.var_alpha0_i_dn9 = 0.0;
        locals.var_alpha0_i_dn10 = 0.0;
        locals.var_alpha0_i_dn11 = 0.0;
        locals.var_alpha0_i_dn12 = 0.0;
        locals.var_alpha0_i_dn13 = 0.0;
        locals.var_alpha0_i_dn14 = 0.0;
        locals.var_alpha0_i_rv = 0.0;

        let assign2270_e3233: f64 = (locals.var_bin_l * p.p497);
        let assign2270_e3234: f64 = (p.p494 + assign2270_e3233);
        let assign2270_e3237: f64 = (locals.var_bin_w * p.p498);
        let assign2270_e3238: f64 = (assign2270_e3234 + assign2270_e3237);
        let assign2270_e3241: f64 = (locals.var_bin_wl * p.p499);
        let assign2270_e3242: f64 = (assign2270_e3238 + assign2270_e3241);
        locals.var_beta0_i = assign2270_e3242;
        locals.var_beta0_i_dn0 = 0.0;
        locals.var_beta0_i_dn2 = 0.0;
        locals.var_beta0_i_dn3 = 0.0;
        locals.var_beta0_i_dn4 = 0.0;
        locals.var_beta0_i_dn5 = 0.0;
        locals.var_beta0_i_dn6 = 0.0;
        locals.var_beta0_i_dn7 = 0.0;
        locals.var_beta0_i_dn8 = 0.0;
        locals.var_beta0_i_dn9 = 0.0;
        locals.var_beta0_i_dn10 = 0.0;
        locals.var_beta0_i_dn11 = 0.0;
        locals.var_beta0_i_dn12 = 0.0;
        locals.var_beta0_i_dn13 = 0.0;
        locals.var_beta0_i_dn14 = 0.0;
        locals.var_beta0_i_rv = 0.0;

        let assign2280_e3246: f64 = (locals.var_bin_l * p.p936);
        let assign2280_e3247: f64 = (p.p935 + assign2280_e3246);
        let assign2280_e3250: f64 = (locals.var_bin_w * p.p937);
        let assign2280_e3251: f64 = (assign2280_e3247 + assign2280_e3250);
        let assign2280_e3254: f64 = (locals.var_bin_wl * p.p938);
        let assign2280_e3255: f64 = (assign2280_e3251 + assign2280_e3254);
        locals.var_kvth0we_i = assign2280_e3255;
        locals.var_kvth0we_i_rv = 0.0;

        let assign2290_e3259: f64 = (locals.var_bin_l * p.p940);
        let assign2290_e3260: f64 = (p.p939 + assign2290_e3259);
        let assign2290_e3263: f64 = (locals.var_bin_w * p.p941);
        let assign2290_e3264: f64 = (assign2290_e3260 + assign2290_e3263);
        let assign2290_e3267: f64 = (locals.var_bin_wl * p.p942);
        let assign2290_e3268: f64 = (assign2290_e3264 + assign2290_e3267);
        locals.var_k2we_i = assign2290_e3268;
        locals.var_k2we_i_rv = 0.0;

        let assign2300_e3272: f64 = (locals.var_bin_l * p.p944);
        let assign2300_e3273: f64 = (p.p943 + assign2300_e3272);
        let assign2300_e3276: f64 = (locals.var_bin_w * p.p945);
        let assign2300_e3277: f64 = (assign2300_e3273 + assign2300_e3276);
        let assign2300_e3280: f64 = (locals.var_bin_wl * p.p946);
        let assign2300_e3281: f64 = (assign2300_e3277 + assign2300_e3280);
        locals.var_ku0we_i = assign2300_e3281;
        locals.var_ku0we_i_rv = 0.0;

        let assign2310_e3285: f64 = (locals.var_bin_l * p.p633);
        let assign2310_e3286: f64 = (p.p630 + assign2310_e3285);
        let assign2310_e3289: f64 = (locals.var_bin_w * p.p634);
        let assign2310_e3290: f64 = (assign2310_e3286 + assign2310_e3289);
        let assign2310_e3293: f64 = (locals.var_bin_wl * p.p635);
        let assign2310_e3294: f64 = (assign2310_e3290 + assign2310_e3293);
        locals.var_agidl_i = assign2310_e3294;
        locals.var_agidl_i_rv = 0.0;

        let assign2320_e3298: f64 = (locals.var_bin_l * p.p637);
        let assign2320_e3299: f64 = (p.p636 + assign2320_e3298);
        let assign2320_e3302: f64 = (locals.var_bin_w * p.p638);
        let assign2320_e3303: f64 = (assign2320_e3299 + assign2320_e3302);
        let assign2320_e3306: f64 = (locals.var_bin_wl * p.p639);
        let assign2320_e3307: f64 = (assign2320_e3303 + assign2320_e3306);
        locals.var_bgidl_i = assign2320_e3307;
        locals.var_bgidl_i_rv = 0.0;

        let assign2330_e3311: f64 = (locals.var_bin_l * p.p641);
        let assign2330_e3312: f64 = (p.p640 + assign2330_e3311);
        let assign2330_e3315: f64 = (locals.var_bin_w * p.p642);
        let assign2330_e3316: f64 = (assign2330_e3312 + assign2330_e3315);
        let assign2330_e3319: f64 = (locals.var_bin_wl * p.p643);
        let assign2330_e3320: f64 = (assign2330_e3316 + assign2330_e3319);
        locals.var_cgidl_i = assign2330_e3320;
        locals.var_cgidl_i_rv = 0.0;

        let assign2340_e3324: f64 = (locals.var_bin_l * p.p645);
        let assign2340_e3325: f64 = (p.p644 + assign2340_e3324);
        let assign2340_e3328: f64 = (locals.var_bin_w * p.p646);
        let assign2340_e3329: f64 = (assign2340_e3325 + assign2340_e3328);
        let assign2340_e3332: f64 = (locals.var_bin_wl * p.p647);
        let assign2340_e3333: f64 = (assign2340_e3329 + assign2340_e3332);
        locals.var_egidl_i = assign2340_e3333;
        locals.var_egidl_i_rv = 0.0;

        let assign2350_e3337: f64 = (locals.var_bin_l * p.p651);
        let assign2350_e3338: f64 = (p.p648 + assign2350_e3337);
        let assign2350_e3341: f64 = (locals.var_bin_w * p.p652);
        let assign2350_e3342: f64 = (assign2350_e3338 + assign2350_e3341);
        let assign2350_e3345: f64 = (locals.var_bin_wl * p.p653);
        let assign2350_e3346: f64 = (assign2350_e3342 + assign2350_e3345);
        locals.var_agisl_i = assign2350_e3346;
        locals.var_agisl_i_rv = 0.0;

        let assign2360_e3350: f64 = (locals.var_bin_l * p.p655);
        let assign2360_e3351: f64 = (p.p654 + assign2360_e3350);
        let assign2360_e3354: f64 = (locals.var_bin_w * p.p656);
        let assign2360_e3355: f64 = (assign2360_e3351 + assign2360_e3354);
        let assign2360_e3358: f64 = (locals.var_bin_wl * p.p657);
        let assign2360_e3359: f64 = (assign2360_e3355 + assign2360_e3358);
        locals.var_bgisl_i = assign2360_e3359;
        locals.var_bgisl_i_rv = 0.0;

        let assign2370_e3363: f64 = (locals.var_bin_l * p.p659);
        let assign2370_e3364: f64 = (p.p658 + assign2370_e3363);
        let assign2370_e3367: f64 = (locals.var_bin_w * p.p660);
        let assign2370_e3368: f64 = (assign2370_e3364 + assign2370_e3367);
        let assign2370_e3371: f64 = (locals.var_bin_wl * p.p661);
        let assign2370_e3372: f64 = (assign2370_e3368 + assign2370_e3371);
        locals.var_cgisl_i = assign2370_e3372;
        locals.var_cgisl_i_rv = 0.0;

        let assign2380_e3376: f64 = (locals.var_bin_l * p.p663);
        let assign2380_e3377: f64 = (p.p662 + assign2380_e3376);
        let assign2380_e3380: f64 = (locals.var_bin_w * p.p664);
        let assign2380_e3381: f64 = (assign2380_e3377 + assign2380_e3380);
        let assign2380_e3384: f64 = (locals.var_bin_wl * p.p665);
        let assign2380_e3385: f64 = (assign2380_e3381 + assign2380_e3384);
        locals.var_egisl_i = assign2380_e3385;
        locals.var_egisl_i_rv = 0.0;

        let assign2390_e3389: f64 = (locals.var_bin_l * p.p825);
        let assign2390_e3390: f64 = (p.p824 + assign2390_e3389);
        let assign2390_e3393: f64 = (locals.var_bin_w * p.p826);
        let assign2390_e3394: f64 = (assign2390_e3390 + assign2390_e3393);
        let assign2390_e3397: f64 = (locals.var_bin_wl * p.p827);
        let assign2390_e3398: f64 = (assign2390_e3394 + assign2390_e3397);
        locals.var_ute_i = assign2390_e3398;
        locals.var_ute_i_rv = 0.0;

        let assign2400_e3402: f64 = (locals.var_bin_l * p.p830);
        let assign2400_e3403: f64 = (p.p829 + assign2400_e3402);
        let assign2400_e3406: f64 = (locals.var_bin_w * p.p831);
        let assign2400_e3407: f64 = (assign2400_e3403 + assign2400_e3406);
        let assign2400_e3410: f64 = (locals.var_bin_wl * p.p832);
        let assign2400_e3411: f64 = (assign2400_e3407 + assign2400_e3410);
        locals.var_ua1_i = assign2400_e3411;
        locals.var_ua1_i_rv = 0.0;

        let assign2410_e3415: f64 = (locals.var_bin_l * p.p835);
        let assign2410_e3416: f64 = (p.p834 + assign2410_e3415);
        let assign2410_e3419: f64 = (locals.var_bin_w * p.p836);
        let assign2410_e3420: f64 = (assign2410_e3416 + assign2410_e3419);
        let assign2410_e3423: f64 = (locals.var_bin_wl * p.p837);
        let assign2410_e3424: f64 = (assign2410_e3420 + assign2410_e3423);
        locals.var_uc1_i = assign2410_e3424;
        locals.var_uc1_i_rv = 0.0;

        let assign2420_e3428: f64 = (locals.var_bin_l * p.p839);
        let assign2420_e3429: f64 = (p.p838 + assign2420_e3428);
        let assign2420_e3432: f64 = (locals.var_bin_w * p.p840);
        let assign2420_e3433: f64 = (assign2420_e3429 + assign2420_e3432);
        let assign2420_e3436: f64 = (locals.var_bin_wl * p.p841);
        let assign2420_e3437: f64 = (assign2420_e3433 + assign2420_e3436);
        locals.var_ud1_i = assign2420_e3437;
        locals.var_ud1_i_rv = 0.0;

        let assign2430_e3441: f64 = (locals.var_bin_l * p.p844);
        let assign2430_e3442: f64 = (p.p843 + assign2430_e3441);
        let assign2430_e3445: f64 = (locals.var_bin_w * p.p845);
        let assign2430_e3446: f64 = (assign2430_e3442 + assign2430_e3445);
        let assign2430_e3449: f64 = (locals.var_bin_wl * p.p846);
        let assign2430_e3450: f64 = (assign2430_e3446 + assign2430_e3449);
        locals.var_eu1_i = assign2430_e3450;
        locals.var_eu1_i_rv = 0.0;

        let assign2440_e3454: f64 = (locals.var_bin_l * p.p848);
        let assign2440_e3455: f64 = (p.p847 + assign2440_e3454);
        let assign2440_e3458: f64 = (locals.var_bin_w * p.p849);
        let assign2440_e3459: f64 = (assign2440_e3455 + assign2440_e3458);
        let assign2440_e3462: f64 = (locals.var_bin_wl * p.p850);
        let assign2440_e3463: f64 = (assign2440_e3459 + assign2440_e3462);
        locals.var_ucste_i = assign2440_e3463;
        locals.var_ucste_i_rv = 0.0;

        let assign2450_e3467: f64 = (locals.var_bin_l * p.p853);
        let assign2450_e3468: f64 = (p.p852 + assign2450_e3467);
        let assign2450_e3471: f64 = (locals.var_bin_w * p.p854);
        let assign2450_e3472: f64 = (assign2450_e3468 + assign2450_e3471);
        let assign2450_e3475: f64 = (locals.var_bin_wl * p.p855);
        let assign2450_e3476: f64 = (assign2450_e3472 + assign2450_e3475);
        locals.var_prt_i = assign2450_e3476;
        locals.var_prt_i_rv = 0.0;

        let assign2460_e3480: f64 = (locals.var_bin_l * p.p857);
        let assign2460_e3481: f64 = (p.p856 + assign2460_e3480);
        let assign2460_e3484: f64 = (locals.var_bin_w * p.p858);
        let assign2460_e3485: f64 = (assign2460_e3481 + assign2460_e3484);
        let assign2460_e3488: f64 = (locals.var_bin_wl * p.p859);
        let assign2460_e3489: f64 = (assign2460_e3485 + assign2460_e3488);
        locals.var_at_i = assign2460_e3489;
        locals.var_at_i_rv = 0.0;

        let assign2470_e3493: f64 = (locals.var_bin_l * p.p863);
        let assign2470_e3494: f64 = (p.p862 + assign2470_e3493);
        let assign2470_e3497: f64 = (locals.var_bin_w * p.p864);
        let assign2470_e3498: f64 = (assign2470_e3494 + assign2470_e3497);
        let assign2470_e3501: f64 = (locals.var_bin_wl * p.p865);
        let assign2470_e3502: f64 = (assign2470_e3498 + assign2470_e3501);
        locals.var_ptwgt_i = assign2470_e3502;
        locals.var_ptwgt_i_rv = 0.0;

        let assign2480_e3506: f64 = (locals.var_bin_l * p.p878);
        let assign2480_e3507: f64 = (p.p877 + assign2480_e3506);
        let assign2480_e3510: f64 = (locals.var_bin_w * p.p879);
        let assign2480_e3511: f64 = (assign2480_e3507 + assign2480_e3510);
        let assign2480_e3514: f64 = (locals.var_bin_wl * p.p880);
        let assign2480_e3515: f64 = (assign2480_e3511 + assign2480_e3514);
        locals.var_iit_i = assign2480_e3515;
        locals.var_iit_i_rv = 0.0;

        let assign2490_e3519: f64 = (locals.var_bin_l * p.p886);
        let assign2490_e3520: f64 = (p.p885 + assign2490_e3519);
        let assign2490_e3523: f64 = (locals.var_bin_w * p.p887);
        let assign2490_e3524: f64 = (assign2490_e3520 + assign2490_e3523);
        let assign2490_e3527: f64 = (locals.var_bin_wl * p.p888);
        let assign2490_e3528: f64 = (assign2490_e3524 + assign2490_e3527);
        locals.var_tgidl_i = assign2490_e3528;
        locals.var_tgidl_i_rv = 0.0;

        let assign2510_e3545: f64 = (locals.var_bin_l * p.p564);
        let assign2510_e3546: f64 = (p.p537 + assign2510_e3545);
        let assign2510_e3549: f64 = (locals.var_bin_w * p.p565);
        let assign2510_e3550: f64 = (assign2510_e3546 + assign2510_e3549);
        let assign2510_e3553: f64 = (locals.var_bin_wl * p.p566);
        let assign2510_e3554: f64 = (assign2510_e3550 + assign2510_e3553);
        locals.var_aigbinv_i = assign2510_e3554;
        locals.var_aigbinv_i_rv = 0.0;

        let assign2520_e3558: f64 = (locals.var_bin_l * p.p567);
        let assign2520_e3559: f64 = (p.p538 + assign2520_e3558);
        let assign2520_e3562: f64 = (locals.var_bin_w * p.p568);
        let assign2520_e3563: f64 = (assign2520_e3559 + assign2520_e3562);
        let assign2520_e3566: f64 = (locals.var_bin_wl * p.p569);
        let assign2520_e3567: f64 = (assign2520_e3563 + assign2520_e3566);
        locals.var_bigbinv_i = assign2520_e3567;
        locals.var_bigbinv_i_rv = 0.0;

        let assign2530_e3571: f64 = (locals.var_bin_l * p.p570);
        let assign2530_e3572: f64 = (p.p539 + assign2530_e3571);
        let assign2530_e3575: f64 = (locals.var_bin_w * p.p571);
        let assign2530_e3576: f64 = (assign2530_e3572 + assign2530_e3575);
        let assign2530_e3579: f64 = (locals.var_bin_wl * p.p572);
        let assign2530_e3580: f64 = (assign2530_e3576 + assign2530_e3579);
        locals.var_cigbinv_i = assign2530_e3580;
        locals.var_cigbinv_i_rv = 0.0;

        let assign2540_e3584: f64 = (locals.var_bin_l * p.p573);
        let assign2540_e3585: f64 = (p.p540 + assign2540_e3584);
        let assign2540_e3588: f64 = (locals.var_bin_w * p.p574);
        let assign2540_e3589: f64 = (assign2540_e3585 + assign2540_e3588);
        let assign2540_e3592: f64 = (locals.var_bin_wl * p.p575);
        let assign2540_e3593: f64 = (assign2540_e3589 + assign2540_e3592);
        locals.var_eigbinv_i = assign2540_e3593;
        locals.var_eigbinv_i_rv = 0.0;

        let assign2550_e3597: f64 = (locals.var_bin_l * p.p576);
        let assign2550_e3598: f64 = (p.p541 + assign2550_e3597);
        let assign2550_e3601: f64 = (locals.var_bin_w * p.p577);
        let assign2550_e3602: f64 = (assign2550_e3598 + assign2550_e3601);
        let assign2550_e3605: f64 = (locals.var_bin_wl * p.p578);
        let assign2550_e3606: f64 = (assign2550_e3602 + assign2550_e3605);
        locals.var_nigbinv_i = assign2550_e3606;
        locals.var_nigbinv_i_rv = 0.0;

        let assign2560_e3610: f64 = (locals.var_bin_l * p.p579);
        let assign2560_e3611: f64 = (p.p533 + assign2560_e3610);
        let assign2560_e3614: f64 = (locals.var_bin_w * p.p580);
        let assign2560_e3615: f64 = (assign2560_e3611 + assign2560_e3614);
        let assign2560_e3618: f64 = (locals.var_bin_wl * p.p581);
        let assign2560_e3619: f64 = (assign2560_e3615 + assign2560_e3618);
        locals.var_aigbacc_i = assign2560_e3619;
        locals.var_aigbacc_i_rv = 0.0;

        let assign2570_e3623: f64 = (locals.var_bin_l * p.p582);
        let assign2570_e3624: f64 = (p.p534 + assign2570_e3623);
        let assign2570_e3627: f64 = (locals.var_bin_w * p.p583);
        let assign2570_e3628: f64 = (assign2570_e3624 + assign2570_e3627);
        let assign2570_e3631: f64 = (locals.var_bin_wl * p.p584);
        let assign2570_e3632: f64 = (assign2570_e3628 + assign2570_e3631);
        locals.var_bigbacc_i = assign2570_e3632;
        locals.var_bigbacc_i_rv = 0.0;

        let assign2580_e3636: f64 = (locals.var_bin_l * p.p585);
        let assign2580_e3637: f64 = (p.p535 + assign2580_e3636);
        let assign2580_e3640: f64 = (locals.var_bin_w * p.p586);
        let assign2580_e3641: f64 = (assign2580_e3637 + assign2580_e3640);
        let assign2580_e3644: f64 = (locals.var_bin_wl * p.p587);
        let assign2580_e3645: f64 = (assign2580_e3641 + assign2580_e3644);
        locals.var_cigbacc_i = assign2580_e3645;
        locals.var_cigbacc_i_rv = 0.0;

        let assign2590_e3649: f64 = (locals.var_bin_l * p.p588);
        let assign2590_e3650: f64 = (p.p536 + assign2590_e3649);
        let assign2590_e3653: f64 = (locals.var_bin_w * p.p589);
        let assign2590_e3654: f64 = (assign2590_e3650 + assign2590_e3653);
        let assign2590_e3657: f64 = (locals.var_bin_wl * p.p590);
        let assign2590_e3658: f64 = (assign2590_e3654 + assign2590_e3657);
        locals.var_nigbacc_i = assign2590_e3658;
        locals.var_nigbacc_i_rv = 0.0;

        let assign2600_e3662: f64 = (locals.var_bin_l * p.p591);
        let assign2600_e3663: f64 = (p.p542 + assign2600_e3662);
        let assign2600_e3666: f64 = (locals.var_bin_w * p.p592);
        let assign2600_e3667: f64 = (assign2600_e3663 + assign2600_e3666);
        let assign2600_e3670: f64 = (locals.var_bin_wl * p.p593);
        let assign2600_e3671: f64 = (assign2600_e3667 + assign2600_e3670);
        locals.var_aigc_i = assign2600_e3671;
        locals.var_aigc_i_rv = 0.0;

        let assign2610_e3675: f64 = (locals.var_bin_l * p.p594);
        let assign2610_e3676: f64 = (p.p543 + assign2610_e3675);
        let assign2610_e3679: f64 = (locals.var_bin_w * p.p595);
        let assign2610_e3680: f64 = (assign2610_e3676 + assign2610_e3679);
        let assign2610_e3683: f64 = (locals.var_bin_wl * p.p596);
        let assign2610_e3684: f64 = (assign2610_e3680 + assign2610_e3683);
        locals.var_bigc_i = assign2610_e3684;
        locals.var_bigc_i_rv = 0.0;

        let assign2620_e3688: f64 = (locals.var_bin_l * p.p597);
        let assign2620_e3689: f64 = (p.p544 + assign2620_e3688);
        let assign2620_e3692: f64 = (locals.var_bin_w * p.p598);
        let assign2620_e3693: f64 = (assign2620_e3689 + assign2620_e3692);
        let assign2620_e3696: f64 = (locals.var_bin_wl * p.p599);
        let assign2620_e3697: f64 = (assign2620_e3693 + assign2620_e3696);
        locals.var_cigc_i = assign2620_e3697;
        locals.var_cigc_i_rv = 0.0;

        let assign2630_e3701: f64 = (locals.var_bin_l * p.p600);
        let assign2630_e3702: f64 = (p.p545 + assign2630_e3701);
        let assign2630_e3705: f64 = (locals.var_bin_w * p.p601);
        let assign2630_e3706: f64 = (assign2630_e3702 + assign2630_e3705);
        let assign2630_e3709: f64 = (locals.var_bin_wl * p.p602);
        let assign2630_e3710: f64 = (assign2630_e3706 + assign2630_e3709);
        locals.var_aigs_i = assign2630_e3710;
        locals.var_aigs_i_rv = 0.0;

        let assign2640_e3714: f64 = (locals.var_bin_l * p.p603);
        let assign2640_e3715: f64 = (p.p546 + assign2640_e3714);
        let assign2640_e3718: f64 = (locals.var_bin_w * p.p604);
        let assign2640_e3719: f64 = (assign2640_e3715 + assign2640_e3718);
        let assign2640_e3722: f64 = (locals.var_bin_wl * p.p605);
        let assign2640_e3723: f64 = (assign2640_e3719 + assign2640_e3722);
        locals.var_bigs_i = assign2640_e3723;
        locals.var_bigs_i_rv = 0.0;

        let assign2650_e3727: f64 = (locals.var_bin_l * p.p606);
        let assign2650_e3728: f64 = (p.p547 + assign2650_e3727);
        let assign2650_e3731: f64 = (locals.var_bin_w * p.p607);
        let assign2650_e3732: f64 = (assign2650_e3728 + assign2650_e3731);
        let assign2650_e3735: f64 = (locals.var_bin_wl * p.p608);
        let assign2650_e3736: f64 = (assign2650_e3732 + assign2650_e3735);
        locals.var_cigs_i = assign2650_e3736;
        locals.var_cigs_i_rv = 0.0;

        let assign2660_e3740: f64 = (locals.var_bin_l * p.p609);
        let assign2660_e3741: f64 = (p.p548 + assign2660_e3740);
        let assign2660_e3744: f64 = (locals.var_bin_w * p.p610);
        let assign2660_e3745: f64 = (assign2660_e3741 + assign2660_e3744);
        let assign2660_e3748: f64 = (locals.var_bin_wl * p.p611);
        let assign2660_e3749: f64 = (assign2660_e3745 + assign2660_e3748);
        locals.var_aigd_i = assign2660_e3749;
        locals.var_aigd_i_rv = 0.0;

        let assign2670_e3753: f64 = (locals.var_bin_l * p.p612);
        let assign2670_e3754: f64 = (p.p549 + assign2670_e3753);
        let assign2670_e3757: f64 = (locals.var_bin_w * p.p613);
        let assign2670_e3758: f64 = (assign2670_e3754 + assign2670_e3757);
        let assign2670_e3761: f64 = (locals.var_bin_wl * p.p614);
        let assign2670_e3762: f64 = (assign2670_e3758 + assign2670_e3761);
        locals.var_bigd_i = assign2670_e3762;
        locals.var_bigd_i_rv = 0.0;

        let assign2680_e3766: f64 = (locals.var_bin_l * p.p615);
        let assign2680_e3767: f64 = (p.p550 + assign2680_e3766);
        let assign2680_e3770: f64 = (locals.var_bin_w * p.p616);
        let assign2680_e3771: f64 = (assign2680_e3767 + assign2680_e3770);
        let assign2680_e3774: f64 = (locals.var_bin_wl * p.p617);
        let assign2680_e3775: f64 = (assign2680_e3771 + assign2680_e3774);
        locals.var_cigd_i = assign2680_e3775;
        locals.var_cigd_i_rv = 0.0;

        let assign2690_e3779: f64 = (locals.var_bin_l * p.p618);
        let assign2690_e3780: f64 = (p.p553 + assign2690_e3779);
        let assign2690_e3783: f64 = (locals.var_bin_w * p.p619);
        let assign2690_e3784: f64 = (assign2690_e3780 + assign2690_e3783);
        let assign2690_e3787: f64 = (locals.var_bin_wl * p.p620);
        let assign2690_e3788: f64 = (assign2690_e3784 + assign2690_e3787);
        locals.var_poxedge_i = assign2690_e3788;
        locals.var_poxedge_i_rv = 0.0;

        let assign2730_e3831: f64 = (locals.var_bin_l * p.p870);
        let assign2730_e3832: f64 = (p.p867 + assign2730_e3831);
        let assign2730_e3835: f64 = (locals.var_bin_w * p.p871);
        let assign2730_e3836: f64 = (assign2730_e3832 + assign2730_e3835);
        let assign2730_e3839: f64 = (locals.var_bin_wl * p.p872);
        let assign2730_e3840: f64 = (assign2730_e3836 + assign2730_e3839);
        locals.var_kt1_i = assign2730_e3840;
        locals.var_kt1_i_rv = 0.0;

        let assign2740_e3844: f64 = (locals.var_bin_l * p.p874);
        let assign2740_e3845: f64 = (p.p873 + assign2740_e3844);
        let assign2740_e3848: f64 = (locals.var_bin_w * p.p875);
        let assign2740_e3849: f64 = (assign2740_e3845 + assign2740_e3848);
        let assign2740_e3852: f64 = (locals.var_bin_wl * p.p876);
        let assign2740_e3853: f64 = (assign2740_e3849 + assign2740_e3852);
        locals.var_kt2_i = assign2740_e3853;
        locals.var_kt2_i_rv = 0.0;

        let assign2750_e3857: f64 = (locals.var_bin_l * p.p430);
        let assign2750_e3858: f64 = (p.p425 + assign2750_e3857);
        let assign2750_e3861: f64 = (locals.var_bin_w * p.p431);
        let assign2750_e3862: f64 = (assign2750_e3858 + assign2750_e3861);
        let assign2750_e3865: f64 = (locals.var_bin_wl * p.p432);
        let assign2750_e3866: f64 = (assign2750_e3862 + assign2750_e3865);
        locals.var_psatb_i = assign2750_e3866;
        locals.var_psatb_i_rv = 0.0;

        let assign2760_e3870: f64 = (locals.var_bin_l * p.p445);
        let assign2760_e3871: f64 = (p.p444 + assign2760_e3870);
        let assign2760_e3874: f64 = (locals.var_bin_w * p.p446);
        let assign2760_e3875: f64 = (assign2760_e3871 + assign2760_e3874);
        let assign2760_e3878: f64 = (locals.var_bin_wl * p.p447);
        let assign2760_e3879: f64 = (assign2760_e3875 + assign2760_e3878);
        locals.var_a1_i = assign2760_e3879;
        locals.var_a1_i_rv = 0.0;

        let assign2770_e3883: f64 = (locals.var_bin_l * p.p449);
        let assign2770_e3884: f64 = (p.p448 + assign2770_e3883);
        let assign2770_e3887: f64 = (locals.var_bin_w * p.p450);
        let assign2770_e3888: f64 = (assign2770_e3884 + assign2770_e3887);
        let assign2770_e3891: f64 = (locals.var_bin_wl * p.p451);
        let assign2770_e3892: f64 = (assign2770_e3888 + assign2770_e3891);
        locals.var_a11_i = assign2770_e3892;
        locals.var_a11_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign2780_e3896: f64 = (locals.var_bin_l * p.p453);
        let assign2780_e3897: f64 = (p.p452 + assign2780_e3896);
        let assign2780_e3900: f64 = (locals.var_bin_w * p.p454);
        let assign2780_e3901: f64 = (assign2780_e3897 + assign2780_e3900);
        let assign2780_e3904: f64 = (locals.var_bin_wl * p.p455);
        let assign2780_e3905: f64 = (assign2780_e3901 + assign2780_e3904);
        locals.var_a2_i = assign2780_e3905;
        locals.var_a2_i_rv = 0.0;

        let assign2790_e3909: f64 = (locals.var_bin_l * p.p457);
        let assign2790_e3910: f64 = (p.p456 + assign2790_e3909);
        let assign2790_e3913: f64 = (locals.var_bin_w * p.p458);
        let assign2790_e3914: f64 = (assign2790_e3910 + assign2790_e3913);
        let assign2790_e3917: f64 = (locals.var_bin_wl * p.p459);
        let assign2790_e3918: f64 = (assign2790_e3914 + assign2790_e3917);
        locals.var_a21_i = assign2790_e3918;
        locals.var_a21_i_rv = 0.0;

        let assign2800_e3922: f64 = (locals.var_bin_l * p.p1047);
        let assign2800_e3923: f64 = (p.p1046 + assign2800_e3922);
        let assign2800_e3926: f64 = (locals.var_bin_w * p.p1048);
        let assign2800_e3927: f64 = (assign2800_e3923 + assign2800_e3926);
        let assign2800_e3930: f64 = (locals.var_bin_wl * p.p1049);
        let assign2800_e3931: f64 = (assign2800_e3927 + assign2800_e3930);
        locals.var_k0_i = assign2800_e3931;
        locals.var_k0_i_rv = 0.0;

        let assign2810_e3935: f64 = (locals.var_bin_l * p.p1055);
        let assign2810_e3936: f64 = (p.p1054 + assign2810_e3935);
        let assign2810_e3939: f64 = (locals.var_bin_w * p.p1056);
        let assign2810_e3940: f64 = (assign2810_e3936 + assign2810_e3939);
        let assign2810_e3943: f64 = (locals.var_bin_wl * p.p1057);
        let assign2810_e3944: f64 = (assign2810_e3940 + assign2810_e3943);
        locals.var_m0_i = assign2810_e3944;
        locals.var_m0_i_rv = 0.0;

        let assign2820_e3948: f64 = (locals.var_bin_l * p.p1051);
        let assign2820_e3949: f64 = (p.p1050 + assign2820_e3948);
        let assign2820_e3952: f64 = (locals.var_bin_w * p.p1052);
        let assign2820_e3953: f64 = (assign2820_e3949 + assign2820_e3952);
        let assign2820_e3956: f64 = (locals.var_bin_wl * p.p1053);
        let assign2820_e3957: f64 = (assign2820_e3953 + assign2820_e3956);
        locals.var_k01_i = assign2820_e3957;
        locals.var_k01_i_rv = 0.0;

        let assign2830_e3961: f64 = (locals.var_bin_l * p.p1059);
        let assign2830_e3962: f64 = (p.p1058 + assign2830_e3961);
        let assign2830_e3965: f64 = (locals.var_bin_w * p.p1060);
        let assign2830_e3966: f64 = (assign2830_e3962 + assign2830_e3965);
        let assign2830_e3969: f64 = (locals.var_bin_wl * p.p1061);
        let assign2830_e3970: f64 = (assign2830_e3966 + assign2830_e3969);
        locals.var_m01_i = assign2830_e3970;
        locals.var_m01_i_rv = 0.0;

        let assign2840_e3974: f64 = (locals.var_bin_l * p.p967);
        let assign2840_e3975: f64 = (p.p966 + assign2840_e3974);
        let assign2840_e3978: f64 = (locals.var_bin_w * p.p968);
        let assign2840_e3979: f64 = (assign2840_e3975 + assign2840_e3978);
        let assign2840_e3982: f64 = (locals.var_bin_wl * p.p969);
        let assign2840_e3983: f64 = (assign2840_e3979 + assign2840_e3982);
        locals.var_nfactoredge_i = assign2840_e3983;
        locals.var_nfactoredge_i_rv = 0.0;

        let assign2850_e3987: f64 = (locals.var_bin_l * p.p963);
        let assign2850_e3988: f64 = (p.p962 + assign2850_e3987);
        let assign2850_e3991: f64 = (locals.var_bin_w * p.p964);
        let assign2850_e3992: f64 = (assign2850_e3988 + assign2850_e3991);
        let assign2850_e3995: f64 = (locals.var_bin_wl * p.p965);
        let assign2850_e3996: f64 = (assign2850_e3992 + assign2850_e3995);
        locals.var_ndepedge_i = assign2850_e3996;
        locals.var_ndepedge_i_rv = 0.0;

        let assign2860_e4000: f64 = (locals.var_bin_l * p.p971);
        let assign2860_e4001: f64 = (p.p970 + assign2860_e4000);
        let assign2860_e4004: f64 = (locals.var_bin_w * p.p972);
        let assign2860_e4005: f64 = (assign2860_e4001 + assign2860_e4004);
        let assign2860_e4008: f64 = (locals.var_bin_wl * p.p973);
        let assign2860_e4009: f64 = (assign2860_e4005 + assign2860_e4008);
        locals.var_citedge_i = assign2860_e4009;
        locals.var_citedge_i_rv = 0.0;

        let assign2870_e4013: f64 = (locals.var_bin_l * p.p975);
        let assign2870_e4014: f64 = (p.p974 + assign2870_e4013);
        let assign2870_e4017: f64 = (locals.var_bin_w * p.p976);
        let assign2870_e4018: f64 = (assign2870_e4014 + assign2870_e4017);
        let assign2870_e4021: f64 = (locals.var_bin_wl * p.p977);
        let assign2870_e4022: f64 = (assign2870_e4018 + assign2870_e4021);
        locals.var_cdscdedge_i = assign2870_e4022;
        locals.var_cdscdedge_i_rv = 0.0;

        let assign2880_e4026: f64 = (locals.var_bin_l * p.p979);
        let assign2880_e4027: f64 = (p.p978 + assign2880_e4026);
        let assign2880_e4030: f64 = (locals.var_bin_w * p.p980);
        let assign2880_e4031: f64 = (assign2880_e4027 + assign2880_e4030);
        let assign2880_e4034: f64 = (locals.var_bin_wl * p.p981);
        let assign2880_e4035: f64 = (assign2880_e4031 + assign2880_e4034);
        locals.var_cdscbedge_i = assign2880_e4035;
        locals.var_cdscbedge_i_rv = 0.0;

        let assign2890_e4039: f64 = (locals.var_bin_l * p.p983);
        let assign2890_e4040: f64 = (p.p982 + assign2890_e4039);
        let assign2890_e4043: f64 = (locals.var_bin_w * p.p984);
        let assign2890_e4044: f64 = (assign2890_e4040 + assign2890_e4043);
        let assign2890_e4047: f64 = (locals.var_bin_wl * p.p985);
        let assign2890_e4048: f64 = (assign2890_e4044 + assign2890_e4047);
        locals.var_eta0edge_i = assign2890_e4048;
        locals.var_eta0edge_i_dn0 = 0.0;
        locals.var_eta0edge_i_dn2 = 0.0;
        locals.var_eta0edge_i_dn3 = 0.0;
        locals.var_eta0edge_i_dn4 = 0.0;
        locals.var_eta0edge_i_dn5 = 0.0;
        locals.var_eta0edge_i_dn6 = 0.0;
        locals.var_eta0edge_i_dn7 = 0.0;
        locals.var_eta0edge_i_dn8 = 0.0;
        locals.var_eta0edge_i_dn9 = 0.0;
        locals.var_eta0edge_i_dn10 = 0.0;
        locals.var_eta0edge_i_dn11 = 0.0;
        locals.var_eta0edge_i_dn12 = 0.0;
        locals.var_eta0edge_i_dn13 = 0.0;
        locals.var_eta0edge_i_dn14 = 0.0;
        locals.var_eta0edge_i_rv = 0.0;

        let assign2900_e4052: f64 = (locals.var_bin_l * p.p987);
        let assign2900_e4053: f64 = (p.p986 + assign2900_e4052);
        let assign2900_e4056: f64 = (locals.var_bin_w * p.p988);
        let assign2900_e4057: f64 = (assign2900_e4053 + assign2900_e4056);
        let assign2900_e4060: f64 = (locals.var_bin_wl * p.p989);
        let assign2900_e4061: f64 = (assign2900_e4057 + assign2900_e4060);
        locals.var_etabedge_i = assign2900_e4061;
        locals.var_etabedge_i_rv = 0.0;

        let assign2910_e4065: f64 = (locals.var_bin_l * p.p991);
        let assign2910_e4066: f64 = (p.p990 + assign2910_e4065);
        let assign2910_e4069: f64 = (locals.var_bin_w * p.p992);
        let assign2910_e4070: f64 = (assign2910_e4066 + assign2910_e4069);
        let assign2910_e4073: f64 = (locals.var_bin_wl * p.p993);
        let assign2910_e4074: f64 = (assign2910_e4070 + assign2910_e4073);
        locals.var_kt1edge_i = assign2910_e4074;
        locals.var_kt1edge_i_rv = 0.0;

        let assign2920_e4078: f64 = (locals.var_bin_l * p.p995);
        let assign2920_e4079: f64 = (p.p994 + assign2920_e4078);
        let assign2920_e4082: f64 = (locals.var_bin_w * p.p996);
        let assign2920_e4083: f64 = (assign2920_e4079 + assign2920_e4082);
        let assign2920_e4086: f64 = (locals.var_bin_wl * p.p997);
        let assign2920_e4087: f64 = (assign2920_e4083 + assign2920_e4086);
        locals.var_kt1ledge_i = assign2920_e4087;
        locals.var_kt1ledge_i_rv = 0.0;

        let assign2930_e4091: f64 = (locals.var_bin_l * p.p999);
        let assign2930_e4092: f64 = (p.p998 + assign2930_e4091);
        let assign2930_e4095: f64 = (locals.var_bin_w * p.p1000);
        let assign2930_e4096: f64 = (assign2930_e4092 + assign2930_e4095);
        let assign2930_e4099: f64 = (locals.var_bin_wl * p.p1001);
        let assign2930_e4100: f64 = (assign2930_e4096 + assign2930_e4099);
        locals.var_kt2edge_i = assign2930_e4100;
        locals.var_kt2edge_i_rv = 0.0;

        let assign2940_e4104: f64 = (locals.var_bin_l * p.p1003);
        let assign2940_e4105: f64 = (p.p1002 + assign2940_e4104);
        let assign2940_e4108: f64 = (locals.var_bin_w * p.p1004);
        let assign2940_e4109: f64 = (assign2940_e4105 + assign2940_e4108);
        let assign2940_e4112: f64 = (locals.var_bin_wl * p.p1005);
        let assign2940_e4113: f64 = (assign2940_e4109 + assign2940_e4112);
        locals.var_kt1expedge_i = assign2940_e4113;
        locals.var_kt1expedge_i_rv = 0.0;

        let assign2950_e4117: f64 = (locals.var_bin_l * p.p1007);
        let assign2950_e4118: f64 = (p.p1006 + assign2950_e4117);
        let assign2950_e4121: f64 = (locals.var_bin_w * p.p1008);
        let assign2950_e4122: f64 = (assign2950_e4118 + assign2950_e4121);
        let assign2950_e4125: f64 = (locals.var_bin_wl * p.p1009);
        let assign2950_e4126: f64 = (assign2950_e4122 + assign2950_e4125);
        locals.var_tnfactoredge_i = assign2950_e4126;
        locals.var_tnfactoredge_i_rv = 0.0;

        let assign2960_e4130: f64 = (locals.var_bin_l * p.p1011);
        let assign2960_e4131: f64 = (p.p1010 + assign2960_e4130);
        let assign2960_e4134: f64 = (locals.var_bin_w * p.p1012);
        let assign2960_e4135: f64 = (assign2960_e4131 + assign2960_e4134);
        let assign2960_e4138: f64 = (locals.var_bin_wl * p.p1013);
        let assign2960_e4139: f64 = (assign2960_e4135 + assign2960_e4138);
        locals.var_teta0edge_i = assign2960_e4139;
        locals.var_teta0edge_i_rv = 0.0;

        let assign2970_e4143: f64 = (locals.var_bin_l * p.p1018);
        let assign2970_e4144: f64 = (p.p1017 + assign2970_e4143);
        let assign2970_e4147: f64 = (locals.var_bin_w * p.p1019);
        let assign2970_e4148: f64 = (assign2970_e4144 + assign2970_e4147);
        let assign2970_e4151: f64 = (locals.var_bin_wl * p.p1020);
        let assign2970_e4152: f64 = (assign2970_e4148 + assign2970_e4151);
        locals.var_k2edge_i = assign2970_e4152;
        locals.var_k2edge_i_dn0 = 0.0;
        locals.var_k2edge_i_dn2 = 0.0;
        locals.var_k2edge_i_dn3 = 0.0;
        locals.var_k2edge_i_dn4 = 0.0;
        locals.var_k2edge_i_dn5 = 0.0;
        locals.var_k2edge_i_dn6 = 0.0;
        locals.var_k2edge_i_dn7 = 0.0;
        locals.var_k2edge_i_dn8 = 0.0;
        locals.var_k2edge_i_dn9 = 0.0;
        locals.var_k2edge_i_dn10 = 0.0;
        locals.var_k2edge_i_dn11 = 0.0;
        locals.var_k2edge_i_dn12 = 0.0;
        locals.var_k2edge_i_dn13 = 0.0;
        locals.var_k2edge_i_dn14 = 0.0;
        locals.var_k2edge_i_rv = 0.0;

        let assign2980_e4156: f64 = (locals.var_bin_l * p.p1022);
        let assign2980_e4157: f64 = (p.p1021 + assign2980_e4156);
        let assign2980_e4160: f64 = (locals.var_bin_w * p.p1023);
        let assign2980_e4161: f64 = (assign2980_e4157 + assign2980_e4160);
        let assign2980_e4164: f64 = (locals.var_bin_wl * p.p1024);
        let assign2980_e4165: f64 = (assign2980_e4161 + assign2980_e4164);
        locals.var_kvth0edge_i = assign2980_e4165;
        locals.var_kvth0edge_i_rv = 0.0;

        let assign2990_e4169: f64 = (locals.var_bin_l * p.p1030);
        let assign2990_e4170: f64 = (p.p1029 + assign2990_e4169);
        let assign2990_e4173: f64 = (locals.var_bin_w * p.p1031);
        let assign2990_e4174: f64 = (assign2990_e4170 + assign2990_e4173);
        let assign2990_e4177: f64 = (locals.var_bin_wl * p.p1032);
        let assign2990_e4178: f64 = (assign2990_e4174 + assign2990_e4177);
        locals.var_k2edgewe_i = assign2990_e4178;
        locals.var_k2edgewe_i_rv = 0.0;

        let assign3000_e4182: f64 = (locals.var_bin_l * p.p1026);
        let assign3000_e4183: f64 = (p.p1025 + assign3000_e4182);
        let assign3000_e4186: f64 = (locals.var_bin_w * p.p1027);
        let assign3000_e4187: f64 = (assign3000_e4183 + assign3000_e4186);
        let assign3000_e4190: f64 = (locals.var_bin_wl * p.p1028);
        let assign3000_e4191: f64 = (assign3000_e4187 + assign3000_e4190);
        locals.var_kvth0edgewe_i = assign3000_e4191;
        locals.var_kvth0edgewe_i_rv = 0.0;

        let assign3010_e4195: f64 = (locals.var_bin_l * p.p1034);
        let assign3010_e4196: f64 = (p.p1033 + assign3010_e4195);
        let assign3010_e4199: f64 = (locals.var_bin_w * p.p1035);
        let assign3010_e4200: f64 = (assign3010_e4196 + assign3010_e4199);
        let assign3010_e4203: f64 = (locals.var_bin_wl * p.p1036);
        let assign3010_e4204: f64 = (assign3010_e4200 + assign3010_e4203);
        locals.var_stk2edge_i = assign3010_e4204;
        locals.var_stk2edge_i_rv = 0.0;

        let assign3020_e4208: f64 = (locals.var_bin_l * p.p1038);
        let assign3020_e4209: f64 = (p.p1037 + assign3020_e4208);
        let assign3020_e4212: f64 = (locals.var_bin_w * p.p1039);
        let assign3020_e4213: f64 = (assign3020_e4209 + assign3020_e4212);
        let assign3020_e4216: f64 = (locals.var_bin_wl * p.p1040);
        let assign3020_e4217: f64 = (assign3020_e4213 + assign3020_e4216);
        locals.var_steta0edge_i = assign3020_e4217;
        locals.var_steta0edge_i_rv = 0.0;

        let assign3030_e4221: f64 = (locals.var_bin_l * p.p1070);
        let assign3030_e4222: f64 = (p.p1069 + assign3030_e4221);
        let assign3030_e4225: f64 = (locals.var_bin_w * p.p1071);
        let assign3030_e4226: f64 = (assign3030_e4222 + assign3030_e4225);
        let assign3030_e4229: f64 = (locals.var_bin_wl * p.p1072);
        let assign3030_e4230: f64 = (assign3030_e4226 + assign3030_e4229);
        locals.var_c0_i = assign3030_e4230;
        locals.var_c0_i_rv = 0.0;

        let assign3040_e4234: f64 = (locals.var_bin_l * p.p1074);
        let assign3040_e4235: f64 = (p.p1073 + assign3040_e4234);
        let assign3040_e4238: f64 = (locals.var_bin_w * p.p1075);
        let assign3040_e4239: f64 = (assign3040_e4235 + assign3040_e4238);
        let assign3040_e4242: f64 = (locals.var_bin_wl * p.p1076);
        let assign3040_e4243: f64 = (assign3040_e4239 + assign3040_e4242);
        locals.var_c01_i = assign3040_e4243;
        locals.var_c01_i_rv = 0.0;

        let assign3050_e4247: f64 = (locals.var_bin_l * p.p1078);
        let assign3050_e4248: f64 = (p.p1077 + assign3050_e4247);
        let assign3050_e4251: f64 = (locals.var_bin_w * p.p1079);
        let assign3050_e4252: f64 = (assign3050_e4248 + assign3050_e4251);
        let assign3050_e4255: f64 = (locals.var_bin_wl * p.p1080);
        let assign3050_e4256: f64 = (assign3050_e4252 + assign3050_e4255);
        locals.var_c0si_i = assign3050_e4256;
        locals.var_c0si_i_rv = 0.0;

        let assign3060_e4260: f64 = (locals.var_bin_l * p.p1082);
        let assign3060_e4261: f64 = (p.p1081 + assign3060_e4260);
        let assign3060_e4264: f64 = (locals.var_bin_w * p.p1083);
        let assign3060_e4265: f64 = (assign3060_e4261 + assign3060_e4264);
        let assign3060_e4268: f64 = (locals.var_bin_wl * p.p1084);
        let assign3060_e4269: f64 = (assign3060_e4265 + assign3060_e4268);
        locals.var_c0si1_i = assign3060_e4269;
        locals.var_c0si1_i_rv = 0.0;

        let assign3070_e4273: f64 = (locals.var_bin_l * p.p1086);
        let assign3070_e4274: f64 = (p.p1085 + assign3070_e4273);
        let assign3070_e4277: f64 = (locals.var_bin_w * p.p1087);
        let assign3070_e4278: f64 = (assign3070_e4274 + assign3070_e4277);
        let assign3070_e4281: f64 = (locals.var_bin_wl * p.p1088);
        let assign3070_e4282: f64 = (assign3070_e4278 + assign3070_e4281);
        locals.var_c0sisat_i = assign3070_e4282;
        locals.var_c0sisat_i_rv = 0.0;

        let assign3080_e4286: f64 = (locals.var_bin_l * p.p1090);
        let assign3080_e4287: f64 = (p.p1089 + assign3080_e4286);
        let assign3080_e4290: f64 = (locals.var_bin_w * p.p1091);
        let assign3080_e4291: f64 = (assign3080_e4287 + assign3080_e4290);
        let assign3080_e4294: f64 = (locals.var_bin_wl * p.p1092);
        let assign3080_e4295: f64 = (assign3080_e4291 + assign3080_e4294);
        locals.var_c0sisat1_i = assign3080_e4295;
        locals.var_c0sisat1_i_rv = 0.0;

        let assign3090_e4299: f64 = (locals.var_bin_l * p.p787);
        let assign3090_e4300: f64 = (p.p786 + assign3090_e4299);
        let assign3090_e4303: f64 = (locals.var_bin_w * p.p788);
        let assign3090_e4304: f64 = (assign3090_e4300 + assign3090_e4303);
        let assign3090_e4307: f64 = (locals.var_bin_wl * p.p789);
        let assign3090_e4308: f64 = (assign3090_e4304 + assign3090_e4307);
        locals.var_noia3_i = assign3090_e4308;
        locals.var_noia3_i_rv = 0.0;

        let assign3100_e4312: f64 = (locals.var_bin_l * p.p795);
        let assign3100_e4313: f64 = (p.p794 + assign3100_e4312);
        let assign3100_e4316: f64 = (locals.var_bin_w * p.p796);
        let assign3100_e4317: f64 = (assign3100_e4313 + assign3100_e4316);
        let assign3100_e4320: f64 = (locals.var_bin_wl * p.p797);
        let assign3100_e4321: f64 = (assign3100_e4317 + assign3100_e4320);
        locals.var_qsref_i = assign3100_e4321;
        locals.var_qsref_i_rv = 0.0;

        let assign3110_e4325: f64 = (locals.var_bin_l * p.p791);
        let assign3110_e4326: f64 = (p.p790 + assign3110_e4325);
        let assign3110_e4329: f64 = (locals.var_bin_w * p.p792);
        let assign3110_e4330: f64 = (assign3110_e4326 + assign3110_e4329);
        let assign3110_e4333: f64 = (locals.var_bin_wl * p.p793);
        let assign3110_e4334: f64 = (assign3110_e4330 + assign3110_e4333);
        locals.var_mpower_i = assign3110_e4334;
        locals.var_mpower_i_rv = 0.0;

        let assign3120_e4337: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard21 = assign3120_e4337;
        locals.var_guard21_rv = 0.0;

        let (assign3130_e4353, assign3130_e4353_d_n0, assign3130_e4353_d_n2, assign3130_e4353_d_n3, assign3130_e4353_d_n4, assign3130_e4353_d_n5, assign3130_e4353_d_n6, assign3130_e4353_d_n7, assign3130_e4353_d_n8, assign3130_e4353_d_n9, assign3130_e4353_d_n10, assign3130_e4353_d_n11, assign3130_e4353_d_n12, assign3130_e4353_d_n13, assign3130_e4353_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3130_e4342: f64 = (locals.var_bin_l * p.p230);
        let assign3130_e4343: f64 = (p.p229 + assign3130_e4342);
        let assign3130_e4346: f64 = (locals.var_bin_w * p.p231);
        let assign3130_e4347: f64 = (assign3130_e4343 + assign3130_e4346);
        let assign3130_e4350: f64 = (locals.var_bin_wl * p.p232);
        let assign3130_e4351: f64 = (assign3130_e4347 + assign3130_e4350);
        (assign3130_e4351, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cdscdr_i, locals.var_cdscdr_i_dn0, locals.var_cdscdr_i_dn2, locals.var_cdscdr_i_dn3, locals.var_cdscdr_i_dn4, locals.var_cdscdr_i_dn5, locals.var_cdscdr_i_dn6, locals.var_cdscdr_i_dn7, locals.var_cdscdr_i_dn8, locals.var_cdscdr_i_dn9, locals.var_cdscdr_i_dn10, locals.var_cdscdr_i_dn11, locals.var_cdscdr_i_dn12, locals.var_cdscdr_i_dn13, locals.var_cdscdr_i_dn14,)
    }
};
        locals.var_cdscdr_i = assign3130_e4353;
        locals.var_cdscdr_i_dn0 = assign3130_e4353_d_n0;
        locals.var_cdscdr_i_dn2 = assign3130_e4353_d_n2;
        locals.var_cdscdr_i_dn3 = assign3130_e4353_d_n3;
        locals.var_cdscdr_i_dn4 = assign3130_e4353_d_n4;
        locals.var_cdscdr_i_dn5 = assign3130_e4353_d_n5;
        locals.var_cdscdr_i_dn6 = assign3130_e4353_d_n6;
        locals.var_cdscdr_i_dn7 = assign3130_e4353_d_n7;
        locals.var_cdscdr_i_dn8 = assign3130_e4353_d_n8;
        locals.var_cdscdr_i_dn9 = assign3130_e4353_d_n9;
        locals.var_cdscdr_i_dn10 = assign3130_e4353_d_n10;
        locals.var_cdscdr_i_dn11 = assign3130_e4353_d_n11;
        locals.var_cdscdr_i_dn12 = assign3130_e4353_d_n12;
        locals.var_cdscdr_i_dn13 = assign3130_e4353_d_n13;
        locals.var_cdscdr_i_dn14 = assign3130_e4353_d_n14;
        locals.var_cdscdr_i_rv = 0.0;

        let (assign3140_e4369, assign3140_e4369_d_n0, assign3140_e4369_d_n2, assign3140_e4369_d_n3, assign3140_e4369_d_n4, assign3140_e4369_d_n5, assign3140_e4369_d_n6, assign3140_e4369_d_n7, assign3140_e4369_d_n8, assign3140_e4369_d_n9, assign3140_e4369_d_n10, assign3140_e4369_d_n11, assign3140_e4369_d_n12, assign3140_e4369_d_n13, assign3140_e4369_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3140_e4358: f64 = (locals.var_bin_l * p.p176);
        let assign3140_e4359: f64 = (p.p175 + assign3140_e4358);
        let assign3140_e4362: f64 = (locals.var_bin_w * p.p177);
        let assign3140_e4363: f64 = (assign3140_e4359 + assign3140_e4362);
        let assign3140_e4366: f64 = (locals.var_bin_wl * p.p178);
        let assign3140_e4367: f64 = (assign3140_e4363 + assign3140_e4366);
        (assign3140_e4367, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eta0r_i, locals.var_eta0r_i_dn0, locals.var_eta0r_i_dn2, locals.var_eta0r_i_dn3, locals.var_eta0r_i_dn4, locals.var_eta0r_i_dn5, locals.var_eta0r_i_dn6, locals.var_eta0r_i_dn7, locals.var_eta0r_i_dn8, locals.var_eta0r_i_dn9, locals.var_eta0r_i_dn10, locals.var_eta0r_i_dn11, locals.var_eta0r_i_dn12, locals.var_eta0r_i_dn13, locals.var_eta0r_i_dn14,)
    }
};
        locals.var_eta0r_i = assign3140_e4369;
        locals.var_eta0r_i_dn0 = assign3140_e4369_d_n0;
        locals.var_eta0r_i_dn2 = assign3140_e4369_d_n2;
        locals.var_eta0r_i_dn3 = assign3140_e4369_d_n3;
        locals.var_eta0r_i_dn4 = assign3140_e4369_d_n4;
        locals.var_eta0r_i_dn5 = assign3140_e4369_d_n5;
        locals.var_eta0r_i_dn6 = assign3140_e4369_d_n6;
        locals.var_eta0r_i_dn7 = assign3140_e4369_d_n7;
        locals.var_eta0r_i_dn8 = assign3140_e4369_d_n8;
        locals.var_eta0r_i_dn9 = assign3140_e4369_d_n9;
        locals.var_eta0r_i_dn10 = assign3140_e4369_d_n10;
        locals.var_eta0r_i_dn11 = assign3140_e4369_d_n11;
        locals.var_eta0r_i_dn12 = assign3140_e4369_d_n12;
        locals.var_eta0r_i_dn13 = assign3140_e4369_d_n13;
        locals.var_eta0r_i_dn14 = assign3140_e4369_d_n14;
        locals.var_eta0r_i_rv = 0.0;

        let (assign3150_e4385,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3150_e4374: f64 = (locals.var_bin_l * p.p280);
        let assign3150_e4375: f64 = (p.p279 + assign3150_e4374);
        let assign3150_e4378: f64 = (locals.var_bin_w * p.p281);
        let assign3150_e4379: f64 = (assign3150_e4375 + assign3150_e4378);
        let assign3150_e4382: f64 = (locals.var_bin_wl * p.p282);
        let assign3150_e4383: f64 = (assign3150_e4379 + assign3150_e4382);
        (assign3150_e4383,)
    } else {
        (locals.var_u0r_i,)
    }
};
        locals.var_u0r_i = assign3150_e4385;
        locals.var_u0r_i_rv = 0.0;

        let (assign3160_e4401, assign3160_e4401_d_n0, assign3160_e4401_d_n2, assign3160_e4401_d_n3, assign3160_e4401_d_n4, assign3160_e4401_d_n5, assign3160_e4401_d_n6, assign3160_e4401_d_n7, assign3160_e4401_d_n8, assign3160_e4401_d_n9, assign3160_e4401_d_n10, assign3160_e4401_d_n11, assign3160_e4401_d_n12, assign3160_e4401_d_n13, assign3160_e4401_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3160_e4390: f64 = (locals.var_bin_l * p.p295);
        let assign3160_e4391: f64 = (p.p294 + assign3160_e4390);
        let assign3160_e4394: f64 = (locals.var_bin_w * p.p296);
        let assign3160_e4395: f64 = (assign3160_e4391 + assign3160_e4394);
        let assign3160_e4398: f64 = (locals.var_bin_wl * p.p297);
        let assign3160_e4399: f64 = (assign3160_e4395 + assign3160_e4398);
        (assign3160_e4399, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uar_i, locals.var_uar_i_dn0, locals.var_uar_i_dn2, locals.var_uar_i_dn3, locals.var_uar_i_dn4, locals.var_uar_i_dn5, locals.var_uar_i_dn6, locals.var_uar_i_dn7, locals.var_uar_i_dn8, locals.var_uar_i_dn9, locals.var_uar_i_dn10, locals.var_uar_i_dn11, locals.var_uar_i_dn12, locals.var_uar_i_dn13, locals.var_uar_i_dn14,)
    }
};
        locals.var_uar_i = assign3160_e4401;
        locals.var_uar_i_dn0 = assign3160_e4401_d_n0;
        locals.var_uar_i_dn2 = assign3160_e4401_d_n2;
        locals.var_uar_i_dn3 = assign3160_e4401_d_n3;
        locals.var_uar_i_dn4 = assign3160_e4401_d_n4;
        locals.var_uar_i_dn5 = assign3160_e4401_d_n5;
        locals.var_uar_i_dn6 = assign3160_e4401_d_n6;
        locals.var_uar_i_dn7 = assign3160_e4401_d_n7;
        locals.var_uar_i_dn8 = assign3160_e4401_d_n8;
        locals.var_uar_i_dn9 = assign3160_e4401_d_n9;
        locals.var_uar_i_dn10 = assign3160_e4401_d_n10;
        locals.var_uar_i_dn11 = assign3160_e4401_d_n11;
        locals.var_uar_i_dn12 = assign3160_e4401_d_n12;
        locals.var_uar_i_dn13 = assign3160_e4401_d_n13;
        locals.var_uar_i_dn14 = assign3160_e4401_d_n14;
        locals.var_uar_i_rv = 0.0;

        let (assign3170_e4417, assign3170_e4417_d_n0, assign3170_e4417_d_n2, assign3170_e4417_d_n3, assign3170_e4417_d_n4, assign3170_e4417_d_n5, assign3170_e4417_d_n6, assign3170_e4417_d_n7, assign3170_e4417_d_n8, assign3170_e4417_d_n9, assign3170_e4417_d_n10, assign3170_e4417_d_n11, assign3170_e4417_d_n12, assign3170_e4417_d_n13, assign3170_e4417_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3170_e4406: f64 = (locals.var_bin_l * p.p315);
        let assign3170_e4407: f64 = (p.p314 + assign3170_e4406);
        let assign3170_e4410: f64 = (locals.var_bin_w * p.p316);
        let assign3170_e4411: f64 = (assign3170_e4407 + assign3170_e4410);
        let assign3170_e4414: f64 = (locals.var_bin_wl * p.p317);
        let assign3170_e4415: f64 = (assign3170_e4411 + assign3170_e4414);
        (assign3170_e4415, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_udr_i, locals.var_udr_i_dn0, locals.var_udr_i_dn2, locals.var_udr_i_dn3, locals.var_udr_i_dn4, locals.var_udr_i_dn5, locals.var_udr_i_dn6, locals.var_udr_i_dn7, locals.var_udr_i_dn8, locals.var_udr_i_dn9, locals.var_udr_i_dn10, locals.var_udr_i_dn11, locals.var_udr_i_dn12, locals.var_udr_i_dn13, locals.var_udr_i_dn14,)
    }
};
        locals.var_udr_i = assign3170_e4417;
        locals.var_udr_i_dn0 = assign3170_e4417_d_n0;
        locals.var_udr_i_dn2 = assign3170_e4417_d_n2;
        locals.var_udr_i_dn3 = assign3170_e4417_d_n3;
        locals.var_udr_i_dn4 = assign3170_e4417_d_n4;
        locals.var_udr_i_dn5 = assign3170_e4417_d_n5;
        locals.var_udr_i_dn6 = assign3170_e4417_d_n6;
        locals.var_udr_i_dn7 = assign3170_e4417_d_n7;
        locals.var_udr_i_dn8 = assign3170_e4417_d_n8;
        locals.var_udr_i_dn9 = assign3170_e4417_d_n9;
        locals.var_udr_i_dn10 = assign3170_e4417_d_n10;
        locals.var_udr_i_dn11 = assign3170_e4417_d_n11;
        locals.var_udr_i_dn12 = assign3170_e4417_d_n12;
        locals.var_udr_i_dn13 = assign3170_e4417_d_n13;
        locals.var_udr_i_dn14 = assign3170_e4417_d_n14;
        locals.var_udr_i_rv = 0.0;

        let (assign3180_e4433,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3180_e4422: f64 = (locals.var_bin_l * p.p323);
        let assign3180_e4423: f64 = (p.p322 + assign3180_e4422);
        let assign3180_e4426: f64 = (locals.var_bin_w * p.p324);
        let assign3180_e4427: f64 = (assign3180_e4423 + assign3180_e4426);
        let assign3180_e4430: f64 = (locals.var_bin_wl * p.p325);
        let assign3180_e4431: f64 = (assign3180_e4427 + assign3180_e4430);
        (assign3180_e4431,)
    } else {
        (locals.var_ucsr_i,)
    }
};
        locals.var_ucsr_i = assign3180_e4433;
        locals.var_ucsr_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3190_e4449, assign3190_e4449_d_n0, assign3190_e4449_d_n2, assign3190_e4449_d_n3, assign3190_e4449_d_n4, assign3190_e4449_d_n5, assign3190_e4449_d_n6, assign3190_e4449_d_n7, assign3190_e4449_d_n8, assign3190_e4449_d_n9, assign3190_e4449_d_n10, assign3190_e4449_d_n11, assign3190_e4449_d_n12, assign3190_e4449_d_n13, assign3190_e4449_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3190_e4438: f64 = (locals.var_bin_l * p.p337);
        let assign3190_e4439: f64 = (p.p336 + assign3190_e4438);
        let assign3190_e4442: f64 = (locals.var_bin_w * p.p338);
        let assign3190_e4443: f64 = (assign3190_e4439 + assign3190_e4442);
        let assign3190_e4446: f64 = (locals.var_bin_wl * p.p339);
        let assign3190_e4447: f64 = (assign3190_e4443 + assign3190_e4446);
        (assign3190_e4447, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ucr_i, locals.var_ucr_i_dn0, locals.var_ucr_i_dn2, locals.var_ucr_i_dn3, locals.var_ucr_i_dn4, locals.var_ucr_i_dn5, locals.var_ucr_i_dn6, locals.var_ucr_i_dn7, locals.var_ucr_i_dn8, locals.var_ucr_i_dn9, locals.var_ucr_i_dn10, locals.var_ucr_i_dn11, locals.var_ucr_i_dn12, locals.var_ucr_i_dn13, locals.var_ucr_i_dn14,)
    }
};
        locals.var_ucr_i = assign3190_e4449;
        locals.var_ucr_i_dn0 = assign3190_e4449_d_n0;
        locals.var_ucr_i_dn2 = assign3190_e4449_d_n2;
        locals.var_ucr_i_dn3 = assign3190_e4449_d_n3;
        locals.var_ucr_i_dn4 = assign3190_e4449_d_n4;
        locals.var_ucr_i_dn5 = assign3190_e4449_d_n5;
        locals.var_ucr_i_dn6 = assign3190_e4449_d_n6;
        locals.var_ucr_i_dn7 = assign3190_e4449_d_n7;
        locals.var_ucr_i_dn8 = assign3190_e4449_d_n8;
        locals.var_ucr_i_dn9 = assign3190_e4449_d_n9;
        locals.var_ucr_i_dn10 = assign3190_e4449_d_n10;
        locals.var_ucr_i_dn11 = assign3190_e4449_d_n11;
        locals.var_ucr_i_dn12 = assign3190_e4449_d_n12;
        locals.var_ucr_i_dn13 = assign3190_e4449_d_n13;
        locals.var_ucr_i_dn14 = assign3190_e4449_d_n14;
        locals.var_ucr_i_rv = 0.0;

        let (assign3200_e4465, assign3200_e4465_d_n0, assign3200_e4465_d_n2, assign3200_e4465_d_n3, assign3200_e4465_d_n4, assign3200_e4465_d_n5, assign3200_e4465_d_n6, assign3200_e4465_d_n7, assign3200_e4465_d_n8, assign3200_e4465_d_n9, assign3200_e4465_d_n10, assign3200_e4465_d_n11, assign3200_e4465_d_n12, assign3200_e4465_d_n13, assign3200_e4465_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3200_e4454: f64 = (locals.var_bin_l * p.p347);
        let assign3200_e4455: f64 = (p.p346 + assign3200_e4454);
        let assign3200_e4458: f64 = (locals.var_bin_w * p.p348);
        let assign3200_e4459: f64 = (assign3200_e4455 + assign3200_e4458);
        let assign3200_e4462: f64 = (locals.var_bin_wl * p.p349);
        let assign3200_e4463: f64 = (assign3200_e4459 + assign3200_e4462);
        (assign3200_e4463, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pclmr_i, locals.var_pclmr_i_dn0, locals.var_pclmr_i_dn2, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11, locals.var_pclmr_i_dn12, locals.var_pclmr_i_dn13, locals.var_pclmr_i_dn14,)
    }
};
        locals.var_pclmr_i = assign3200_e4465;
        locals.var_pclmr_i_dn0 = assign3200_e4465_d_n0;
        locals.var_pclmr_i_dn2 = assign3200_e4465_d_n2;
        locals.var_pclmr_i_dn3 = assign3200_e4465_d_n3;
        locals.var_pclmr_i_dn4 = assign3200_e4465_d_n4;
        locals.var_pclmr_i_dn5 = assign3200_e4465_d_n5;
        locals.var_pclmr_i_dn6 = assign3200_e4465_d_n6;
        locals.var_pclmr_i_dn7 = assign3200_e4465_d_n7;
        locals.var_pclmr_i_dn8 = assign3200_e4465_d_n8;
        locals.var_pclmr_i_dn9 = assign3200_e4465_d_n9;
        locals.var_pclmr_i_dn10 = assign3200_e4465_d_n10;
        locals.var_pclmr_i_dn11 = assign3200_e4465_d_n11;
        locals.var_pclmr_i_dn12 = assign3200_e4465_d_n12;
        locals.var_pclmr_i_dn13 = assign3200_e4465_d_n13;
        locals.var_pclmr_i_dn14 = assign3200_e4465_d_n14;
        locals.var_pclmr_i_rv = 0.0;

        let (assign3210_e4481, assign3210_e4481_d_n0, assign3210_e4481_d_n2, assign3210_e4481_d_n3, assign3210_e4481_d_n4, assign3210_e4481_d_n5, assign3210_e4481_d_n6, assign3210_e4481_d_n7, assign3210_e4481_d_n8, assign3210_e4481_d_n9, assign3210_e4481_d_n10, assign3210_e4481_d_n11, assign3210_e4481_d_n12, assign3210_e4481_d_n13, assign3210_e4481_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3210_e4470: f64 = (locals.var_bin_l * p.p467);
        let assign3210_e4471: f64 = (p.p466 + assign3210_e4470);
        let assign3210_e4474: f64 = (locals.var_bin_w * p.p468);
        let assign3210_e4475: f64 = (assign3210_e4471 + assign3210_e4474);
        let assign3210_e4478: f64 = (locals.var_bin_wl * p.p469);
        let assign3210_e4479: f64 = (assign3210_e4475 + assign3210_e4478);
        (assign3210_e4479, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdiblcr_i, locals.var_pdiblcr_i_dn0, locals.var_pdiblcr_i_dn2, locals.var_pdiblcr_i_dn3, locals.var_pdiblcr_i_dn4, locals.var_pdiblcr_i_dn5, locals.var_pdiblcr_i_dn6, locals.var_pdiblcr_i_dn7, locals.var_pdiblcr_i_dn8, locals.var_pdiblcr_i_dn9, locals.var_pdiblcr_i_dn10, locals.var_pdiblcr_i_dn11, locals.var_pdiblcr_i_dn12, locals.var_pdiblcr_i_dn13, locals.var_pdiblcr_i_dn14,)
    }
};
        locals.var_pdiblcr_i = assign3210_e4481;
        locals.var_pdiblcr_i_dn0 = assign3210_e4481_d_n0;
        locals.var_pdiblcr_i_dn2 = assign3210_e4481_d_n2;
        locals.var_pdiblcr_i_dn3 = assign3210_e4481_d_n3;
        locals.var_pdiblcr_i_dn4 = assign3210_e4481_d_n4;
        locals.var_pdiblcr_i_dn5 = assign3210_e4481_d_n5;
        locals.var_pdiblcr_i_dn6 = assign3210_e4481_d_n6;
        locals.var_pdiblcr_i_dn7 = assign3210_e4481_d_n7;
        locals.var_pdiblcr_i_dn8 = assign3210_e4481_d_n8;
        locals.var_pdiblcr_i_dn9 = assign3210_e4481_d_n9;
        locals.var_pdiblcr_i_dn10 = assign3210_e4481_d_n10;
        locals.var_pdiblcr_i_dn11 = assign3210_e4481_d_n11;
        locals.var_pdiblcr_i_dn12 = assign3210_e4481_d_n12;
        locals.var_pdiblcr_i_dn13 = assign3210_e4481_d_n13;
        locals.var_pdiblcr_i_dn14 = assign3210_e4481_d_n14;
        locals.var_pdiblcr_i_rv = 0.0;

        let (assign3220_e4497, assign3220_e4497_d_n0, assign3220_e4497_d_n2, assign3220_e4497_d_n3, assign3220_e4497_d_n4, assign3220_e4497_d_n5, assign3220_e4497_d_n6, assign3220_e4497_d_n7, assign3220_e4497_d_n8, assign3220_e4497_d_n9, assign3220_e4497_d_n10, assign3220_e4497_d_n11, assign3220_e4497_d_n12, assign3220_e4497_d_n13, assign3220_e4497_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3220_e4486: f64 = (locals.var_bin_l * p.p250);
        let assign3220_e4487: f64 = (p.p249 + assign3220_e4486);
        let assign3220_e4490: f64 = (locals.var_bin_w * p.p251);
        let assign3220_e4491: f64 = (assign3220_e4487 + assign3220_e4490);
        let assign3220_e4494: f64 = (locals.var_bin_wl * p.p252);
        let assign3220_e4495: f64 = (assign3220_e4491 + assign3220_e4494);
        (assign3220_e4495, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsatr_i, locals.var_vsatr_i_dn0, locals.var_vsatr_i_dn2, locals.var_vsatr_i_dn3, locals.var_vsatr_i_dn4, locals.var_vsatr_i_dn5, locals.var_vsatr_i_dn6, locals.var_vsatr_i_dn7, locals.var_vsatr_i_dn8, locals.var_vsatr_i_dn9, locals.var_vsatr_i_dn10, locals.var_vsatr_i_dn11, locals.var_vsatr_i_dn12, locals.var_vsatr_i_dn13, locals.var_vsatr_i_dn14,)
    }
};
        locals.var_vsatr_i = assign3220_e4497;
        locals.var_vsatr_i_dn0 = assign3220_e4497_d_n0;
        locals.var_vsatr_i_dn2 = assign3220_e4497_d_n2;
        locals.var_vsatr_i_dn3 = assign3220_e4497_d_n3;
        locals.var_vsatr_i_dn4 = assign3220_e4497_d_n4;
        locals.var_vsatr_i_dn5 = assign3220_e4497_d_n5;
        locals.var_vsatr_i_dn6 = assign3220_e4497_d_n6;
        locals.var_vsatr_i_dn7 = assign3220_e4497_d_n7;
        locals.var_vsatr_i_dn8 = assign3220_e4497_d_n8;
        locals.var_vsatr_i_dn9 = assign3220_e4497_d_n9;
        locals.var_vsatr_i_dn10 = assign3220_e4497_d_n10;
        locals.var_vsatr_i_dn11 = assign3220_e4497_d_n11;
        locals.var_vsatr_i_dn12 = assign3220_e4497_d_n12;
        locals.var_vsatr_i_dn13 = assign3220_e4497_d_n13;
        locals.var_vsatr_i_dn14 = assign3220_e4497_d_n14;
        locals.var_vsatr_i_rv = 0.0;

        let (assign3230_e4513,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3230_e4502: f64 = (locals.var_bin_l * p.p427);
        let assign3230_e4503: f64 = (p.p426 + assign3230_e4502);
        let assign3230_e4506: f64 = (locals.var_bin_w * p.p428);
        let assign3230_e4507: f64 = (assign3230_e4503 + assign3230_e4506);
        let assign3230_e4510: f64 = (locals.var_bin_wl * p.p429);
        let assign3230_e4511: f64 = (assign3230_e4507 + assign3230_e4510);
        (assign3230_e4511,)
    } else {
        (locals.var_psatr_i,)
    }
};
        locals.var_psatr_i = assign3230_e4513;
        locals.var_psatr_i_rv = 0.0;

        let (assign3240_e4529, assign3240_e4529_d_n0, assign3240_e4529_d_n2, assign3240_e4529_d_n3, assign3240_e4529_d_n4, assign3240_e4529_d_n5, assign3240_e4529_d_n6, assign3240_e4529_d_n7, assign3240_e4529_d_n8, assign3240_e4529_d_n9, assign3240_e4529_d_n10, assign3240_e4529_d_n11, assign3240_e4529_d_n12, assign3240_e4529_d_n13, assign3240_e4529_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3240_e4518: f64 = (locals.var_bin_l * p.p441);
        let assign3240_e4519: f64 = (p.p440 + assign3240_e4518);
        let assign3240_e4522: f64 = (locals.var_bin_w * p.p442);
        let assign3240_e4523: f64 = (assign3240_e4519 + assign3240_e4522);
        let assign3240_e4526: f64 = (locals.var_bin_wl * p.p443);
        let assign3240_e4527: f64 = (assign3240_e4523 + assign3240_e4526);
        (assign3240_e4527, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ptwgr_i, locals.var_ptwgr_i_dn0, locals.var_ptwgr_i_dn2, locals.var_ptwgr_i_dn3, locals.var_ptwgr_i_dn4, locals.var_ptwgr_i_dn5, locals.var_ptwgr_i_dn6, locals.var_ptwgr_i_dn7, locals.var_ptwgr_i_dn8, locals.var_ptwgr_i_dn9, locals.var_ptwgr_i_dn10, locals.var_ptwgr_i_dn11, locals.var_ptwgr_i_dn12, locals.var_ptwgr_i_dn13, locals.var_ptwgr_i_dn14,)
    }
};
        locals.var_ptwgr_i = assign3240_e4529;
        locals.var_ptwgr_i_dn0 = assign3240_e4529_d_n0;
        locals.var_ptwgr_i_dn2 = assign3240_e4529_d_n2;
        locals.var_ptwgr_i_dn3 = assign3240_e4529_d_n3;
        locals.var_ptwgr_i_dn4 = assign3240_e4529_d_n4;
        locals.var_ptwgr_i_dn5 = assign3240_e4529_d_n5;
        locals.var_ptwgr_i_dn6 = assign3240_e4529_d_n6;
        locals.var_ptwgr_i_dn7 = assign3240_e4529_d_n7;
        locals.var_ptwgr_i_dn8 = assign3240_e4529_d_n8;
        locals.var_ptwgr_i_dn9 = assign3240_e4529_d_n9;
        locals.var_ptwgr_i_dn10 = assign3240_e4529_d_n10;
        locals.var_ptwgr_i_dn11 = assign3240_e4529_d_n11;
        locals.var_ptwgr_i_dn12 = assign3240_e4529_d_n12;
        locals.var_ptwgr_i_dn13 = assign3240_e4529_d_n13;
        locals.var_ptwgr_i_dn14 = assign3240_e4529_d_n14;
        locals.var_ptwgr_i_rv = 0.0;

        let (assign3250_e4545, assign3250_e4545_d_n0, assign3250_e4545_d_n2, assign3250_e4545_d_n3, assign3250_e4545_d_n4, assign3250_e4545_d_n5, assign3250_e4545_d_n6, assign3250_e4545_d_n7, assign3250_e4545_d_n8, assign3250_e4545_d_n9, assign3250_e4545_d_n10, assign3250_e4545_d_n11, assign3250_e4545_d_n12, assign3250_e4545_d_n13, assign3250_e4545_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3250_e4534: f64 = (locals.var_bin_l * p.p526);
        let assign3250_e4535: f64 = (p.p525 + assign3250_e4534);
        let assign3250_e4538: f64 = (locals.var_bin_w * p.p527);
        let assign3250_e4539: f64 = (assign3250_e4535 + assign3250_e4538);
        let assign3250_e4542: f64 = (locals.var_bin_wl * p.p528);
        let assign3250_e4543: f64 = (assign3250_e4539 + assign3250_e4542);
        (assign3250_e4543, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_alpha0r_i, locals.var_alpha0r_i_dn0, locals.var_alpha0r_i_dn2, locals.var_alpha0r_i_dn3, locals.var_alpha0r_i_dn4, locals.var_alpha0r_i_dn5, locals.var_alpha0r_i_dn6, locals.var_alpha0r_i_dn7, locals.var_alpha0r_i_dn8, locals.var_alpha0r_i_dn9, locals.var_alpha0r_i_dn10, locals.var_alpha0r_i_dn11, locals.var_alpha0r_i_dn12, locals.var_alpha0r_i_dn13, locals.var_alpha0r_i_dn14,)
    }
};
        locals.var_alpha0r_i = assign3250_e4545;
        locals.var_alpha0r_i_dn0 = assign3250_e4545_d_n0;
        locals.var_alpha0r_i_dn2 = assign3250_e4545_d_n2;
        locals.var_alpha0r_i_dn3 = assign3250_e4545_d_n3;
        locals.var_alpha0r_i_dn4 = assign3250_e4545_d_n4;
        locals.var_alpha0r_i_dn5 = assign3250_e4545_d_n5;
        locals.var_alpha0r_i_dn6 = assign3250_e4545_d_n6;
        locals.var_alpha0r_i_dn7 = assign3250_e4545_d_n7;
        locals.var_alpha0r_i_dn8 = assign3250_e4545_d_n8;
        locals.var_alpha0r_i_dn9 = assign3250_e4545_d_n9;
        locals.var_alpha0r_i_dn10 = assign3250_e4545_d_n10;
        locals.var_alpha0r_i_dn11 = assign3250_e4545_d_n11;
        locals.var_alpha0r_i_dn12 = assign3250_e4545_d_n12;
        locals.var_alpha0r_i_dn13 = assign3250_e4545_d_n13;
        locals.var_alpha0r_i_dn14 = assign3250_e4545_d_n14;
        locals.var_alpha0r_i_rv = 0.0;

        let (assign3260_e4561,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3260_e4550: f64 = (locals.var_bin_l * p.p530);
        let assign3260_e4551: f64 = (p.p529 + assign3260_e4550);
        let assign3260_e4554: f64 = (locals.var_bin_w * p.p531);
        let assign3260_e4555: f64 = (assign3260_e4551 + assign3260_e4554);
        let assign3260_e4558: f64 = (locals.var_bin_wl * p.p532);
        let assign3260_e4559: f64 = (assign3260_e4555 + assign3260_e4558);
        (assign3260_e4559,)
    } else {
        (locals.var_beta0r_i,)
    }
};
        locals.var_beta0r_i = assign3260_e4561;
        locals.var_beta0r_i_rv = 0.0;

        let assign3270_e4565: f64 = (locals.var_inv_l).powf(p.p82);
        let assign3270_e4568: f64 = (locals.var_inv_llong).powf(p.p82);
        let assign3270_e4569: f64 = (assign3270_e4565 - assign3270_e4568);
        let assign3270_e4571: f64 = (assign3270_e4569).max(0.0);
        let assign3270_e4572: f64 = (p.p81 * assign3270_e4571);
        let assign3270_e4576: f64 = (locals.var_inv_l).powf(p.p84);
        let assign3270_e4579: f64 = (locals.var_inv_llong).powf(p.p84);
        let assign3270_e4580: f64 = (assign3270_e4576 - assign3270_e4579);
        let assign3270_e4582: f64 = (assign3270_e4580).max(0.0);
        let assign3270_e4583: f64 = (p.p83 * assign3270_e4582);
        let assign3270_e4584: f64 = (assign3270_e4572 + assign3270_e4583);
        locals.var_t0 = assign3270_e4584;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign3280_e4588: f64 = (locals.var_inv_w).powf(p.p86);
        let assign3280_e4591: f64 = (locals.var_inv_wwide).powf(p.p86);
        let assign3280_e4592: f64 = (assign3280_e4588 - assign3280_e4591);
        let assign3280_e4594: f64 = (assign3280_e4592).max(0.0);
        let assign3280_e4595: f64 = (p.p85 * assign3280_e4594);
        let assign3280_e4599: f64 = (locals.var_inv_w * locals.var_inv_l);
        let assign3280_e4601: f64 = (assign3280_e4599).powf(p.p88);
        let assign3280_e4602: f64 = (p.p87 * assign3280_e4601);
        let assign3280_e4603: f64 = (assign3280_e4595 + assign3280_e4602);
        locals.var_t1 = assign3280_e4603;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign3290_e4607: f64 = (1.0 + locals.var_t0);
        let assign3290_e4609: f64 = (assign3290_e4607 + locals.var_t1);
        let assign3290_e4610: f64 = (locals.var_ndep_i * assign3290_e4609);
        locals.var_ndep_i = assign3290_e4610;
        locals.var_ndep_i_dn0 = ((locals.var_ndep_i_dn0 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_ndep_i_dn2 = ((locals.var_ndep_i_dn2 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_ndep_i_dn3 = ((locals.var_ndep_i_dn3 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_ndep_i_dn4 = ((locals.var_ndep_i_dn4 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_ndep_i_dn5 = ((locals.var_ndep_i_dn5 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_ndep_i_dn6 = ((locals.var_ndep_i_dn6 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_ndep_i_dn7 = ((locals.var_ndep_i_dn7 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_ndep_i_dn8 = ((locals.var_ndep_i_dn8 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_ndep_i_dn9 = ((locals.var_ndep_i_dn9 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_ndep_i_dn10 = ((locals.var_ndep_i_dn10 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_ndep_i_dn11 = ((locals.var_ndep_i_dn11 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_ndep_i_dn12 = ((locals.var_ndep_i_dn12 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_ndep_i_dn13 = ((locals.var_ndep_i_dn13 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_ndep_i_dn14 = ((locals.var_ndep_i_dn14 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));
        locals.var_ndep_i_rv = 0.0;

        let assign3300_e4614: f64 = (locals.var_inv_l).powf(p.p215);
        let assign3300_e4617: f64 = (locals.var_inv_llong).powf(p.p215);
        let assign3300_e4618: f64 = (assign3300_e4614 - assign3300_e4617);
        let assign3300_e4620: f64 = (assign3300_e4618).max(0.0);
        let assign3300_e4621: f64 = (p.p214 * assign3300_e4620);
        locals.var_t0 = assign3300_e4621;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign3310_e4625: f64 = (locals.var_inv_w).powf(p.p217);
        let assign3310_e4628: f64 = (locals.var_inv_wwide).powf(p.p217);
        let assign3310_e4629: f64 = (assign3310_e4625 - assign3310_e4628);
        let assign3310_e4631: f64 = (assign3310_e4629).max(0.0);
        let assign3310_e4632: f64 = (p.p216 * assign3310_e4631);
        let assign3310_e4636: f64 = (locals.var_inv_wl).powf(p.p219);
        let assign3310_e4637: f64 = (p.p218 * assign3310_e4636);
        let assign3310_e4638: f64 = (assign3310_e4632 + assign3310_e4637);
        locals.var_t1 = assign3310_e4638;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign3320_e4642: f64 = (1.0 + locals.var_t0);
        let assign3320_e4644: f64 = (assign3320_e4642 + locals.var_t1);
        let assign3320_e4645: f64 = (locals.var_nfactor_i * assign3320_e4644);
        locals.var_nfactor_i = assign3320_e4645;
        locals.var_nfactor_i_dn0 = ((locals.var_nfactor_i_dn0 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_nfactor_i_dn2 = ((locals.var_nfactor_i_dn2 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_nfactor_i_dn3 = ((locals.var_nfactor_i_dn3 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_nfactor_i_dn4 = ((locals.var_nfactor_i_dn4 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_nfactor_i_dn5 = ((locals.var_nfactor_i_dn5 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_nfactor_i_dn6 = ((locals.var_nfactor_i_dn6 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_nfactor_i_dn7 = ((locals.var_nfactor_i_dn7 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_nfactor_i_dn8 = ((locals.var_nfactor_i_dn8 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_nfactor_i_dn9 = ((locals.var_nfactor_i_dn9 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_nfactor_i_dn10 = ((locals.var_nfactor_i_dn10 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_nfactor_i_dn11 = ((locals.var_nfactor_i_dn11 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_nfactor_i_dn12 = ((locals.var_nfactor_i_dn12 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_nfactor_i_dn13 = ((locals.var_nfactor_i_dn13 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_nfactor_i_dn14 = ((locals.var_nfactor_i_dn14 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));
        locals.var_nfactor_i_rv = 0.0;

        let assign3330_e4650: f64 = (locals.var_inv_l).powf(p.p225);
        let assign3330_e4653: f64 = (locals.var_inv_llong).powf(p.p225);
        let assign3330_e4654: f64 = (assign3330_e4650 - assign3330_e4653);
        let assign3330_e4656: f64 = (assign3330_e4654).max(0.0);
        let assign3330_e4657: f64 = (p.p224 * assign3330_e4656);
        let assign3330_e4658: f64 = (1.0 + assign3330_e4657);
        locals.var_t0 = assign3330_e4658;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign3340_e4661: f64 = (locals.var_cdscd_i * locals.var_t0);
        locals.var_cdscd_i = assign3340_e4661;
        locals.var_cdscd_i_dn0 = ((locals.var_cdscd_i_dn0 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn0));
        locals.var_cdscd_i_dn2 = ((locals.var_cdscd_i_dn2 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn2));
        locals.var_cdscd_i_dn3 = ((locals.var_cdscd_i_dn3 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn3));
        locals.var_cdscd_i_dn4 = ((locals.var_cdscd_i_dn4 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn4));
        locals.var_cdscd_i_dn5 = ((locals.var_cdscd_i_dn5 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn5));
        locals.var_cdscd_i_dn6 = ((locals.var_cdscd_i_dn6 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn6));
        locals.var_cdscd_i_dn7 = ((locals.var_cdscd_i_dn7 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn7));
        locals.var_cdscd_i_dn8 = ((locals.var_cdscd_i_dn8 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn8));
        locals.var_cdscd_i_dn9 = ((locals.var_cdscd_i_dn9 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn9));
        locals.var_cdscd_i_dn10 = ((locals.var_cdscd_i_dn10 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn10));
        locals.var_cdscd_i_dn11 = ((locals.var_cdscd_i_dn11 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn11));
        locals.var_cdscd_i_dn12 = ((locals.var_cdscd_i_dn12 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn12));
        locals.var_cdscd_i_dn13 = ((locals.var_cdscd_i_dn13 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn13));
        locals.var_cdscd_i_dn14 = ((locals.var_cdscd_i_dn14 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn14));
        locals.var_cdscd_i_rv = 0.0;

        let assign3350_e4664: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard22 = assign3350_e4664;
        locals.var_guard22_rv = 0.0;

        let (assign3360_e4670, assign3360_e4670_d_n0, assign3360_e4670_d_n2, assign3360_e4670_d_n3, assign3360_e4670_d_n4, assign3360_e4670_d_n5, assign3360_e4670_d_n6, assign3360_e4670_d_n7, assign3360_e4670_d_n8, assign3360_e4670_d_n9, assign3360_e4670_d_n10, assign3360_e4670_d_n11, assign3360_e4670_d_n12, assign3360_e4670_d_n13, assign3360_e4670_d_n14,) = {
    if (locals.var_guard22 != 0.0) {
        let assign3360_e4668: f64 = (locals.var_cdscdr_i * locals.var_t0);
        (assign3360_e4668, ((locals.var_cdscdr_i_dn0 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn0)), ((locals.var_cdscdr_i_dn2 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn2)), ((locals.var_cdscdr_i_dn3 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn3)), ((locals.var_cdscdr_i_dn4 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn4)), ((locals.var_cdscdr_i_dn5 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn5)), ((locals.var_cdscdr_i_dn6 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn6)), ((locals.var_cdscdr_i_dn7 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn7)), ((locals.var_cdscdr_i_dn8 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn8)), ((locals.var_cdscdr_i_dn9 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn9)), ((locals.var_cdscdr_i_dn10 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn10)), ((locals.var_cdscdr_i_dn11 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn11)), ((locals.var_cdscdr_i_dn12 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn12)), ((locals.var_cdscdr_i_dn13 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn13)), ((locals.var_cdscdr_i_dn14 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn14)),)
    } else {
        (locals.var_cdscdr_i, locals.var_cdscdr_i_dn0, locals.var_cdscdr_i_dn2, locals.var_cdscdr_i_dn3, locals.var_cdscdr_i_dn4, locals.var_cdscdr_i_dn5, locals.var_cdscdr_i_dn6, locals.var_cdscdr_i_dn7, locals.var_cdscdr_i_dn8, locals.var_cdscdr_i_dn9, locals.var_cdscdr_i_dn10, locals.var_cdscdr_i_dn11, locals.var_cdscdr_i_dn12, locals.var_cdscdr_i_dn13, locals.var_cdscdr_i_dn14,)
    }
};
        locals.var_cdscdr_i = assign3360_e4670;
        locals.var_cdscdr_i_dn0 = assign3360_e4670_d_n0;
        locals.var_cdscdr_i_dn2 = assign3360_e4670_d_n2;
        locals.var_cdscdr_i_dn3 = assign3360_e4670_d_n3;
        locals.var_cdscdr_i_dn4 = assign3360_e4670_d_n4;
        locals.var_cdscdr_i_dn5 = assign3360_e4670_d_n5;
        locals.var_cdscdr_i_dn6 = assign3360_e4670_d_n6;
        locals.var_cdscdr_i_dn7 = assign3360_e4670_d_n7;
        locals.var_cdscdr_i_dn8 = assign3360_e4670_d_n8;
        locals.var_cdscdr_i_dn9 = assign3360_e4670_d_n9;
        locals.var_cdscdr_i_dn10 = assign3360_e4670_d_n10;
        locals.var_cdscdr_i_dn11 = assign3360_e4670_d_n11;
        locals.var_cdscdr_i_dn12 = assign3360_e4670_d_n12;
        locals.var_cdscdr_i_dn13 = assign3360_e4670_d_n13;
        locals.var_cdscdr_i_dn14 = assign3360_e4670_d_n14;
        locals.var_cdscdr_i_rv = 0.0;

        let assign3370_e4676: f64 = (locals.var_inv_l).powf(p.p235);
        let assign3370_e4679: f64 = (locals.var_inv_llong).powf(p.p235);
        let assign3370_e4680: f64 = (assign3370_e4676 - assign3370_e4679);
        let assign3370_e4682: f64 = (assign3370_e4680).max(0.0);
        let assign3370_e4683: f64 = (p.p234 * assign3370_e4682);
        let assign3370_e4684: f64 = (1.0 + assign3370_e4683);
        let assign3370_e4685: f64 = (locals.var_cdscb_i * assign3370_e4684);
        locals.var_cdscb_i = assign3370_e4685;
        locals.var_cdscb_i_rv = 0.0;

        let assign3380_e4688: f64 = (p.p34 * locals.var_u0_i);
        locals.var_u0_i = assign3380_e4688;
        locals.var_u0_i_rv = 0.0;

        let assign3390_e4691: f64 = if p.p50 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard23 = assign3390_e4691;
        locals.var_guard23_rv = 0.0;

        let assign3400_e4694: f64 = if p.p275 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard24 = assign3400_e4694;
        locals.var_guard24_rv = 0.0;

        let (assign3410_e4714,) = {
    if ((locals.var_guard23 != 0.0) && (locals.var_guard24 != 0.0)) {
        let assign3410_e4703: f64 = (locals.var_inv_l).powf(p.p275);
        let assign3410_e4706: f64 = (locals.var_inv_llong).powf(p.p275);
        let assign3410_e4707: f64 = (assign3410_e4703 - assign3410_e4706);
        let assign3410_e4709: f64 = (assign3410_e4707).max(0.0);
        let assign3410_e4710: f64 = (p.p274 * assign3410_e4709);
        let assign3410_e4711: f64 = (1.0 - assign3410_e4710);
        let assign3410_e4712: f64 = (locals.var_u0_i * assign3410_e4711);
        (assign3410_e4712,)
    } else {
        (locals.var_u0_i,)
    }
};
        locals.var_u0_i = assign3410_e4714;
        locals.var_u0_i_rv = 0.0;

        let assign3420_e4717: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard25 = assign3420_e4717;
        locals.var_guard25_rv = 0.0;

        let (assign3430_e4739,) = {
    if (((locals.var_guard23 != 0.0) && (locals.var_guard24 != 0.0)) && (locals.var_guard25 != 0.0)) {
        let assign3430_e4728: f64 = (locals.var_inv_l).powf(p.p275);
        let assign3430_e4731: f64 = (locals.var_inv_llong).powf(p.p275);
        let assign3430_e4732: f64 = (assign3430_e4728 - assign3430_e4731);
        let assign3430_e4734: f64 = (assign3430_e4732).max(0.0);
        let assign3430_e4735: f64 = (p.p274 * assign3430_e4734);
        let assign3430_e4736: f64 = (1.0 - assign3430_e4735);
        let assign3430_e4737: f64 = (locals.var_u0r_i * assign3430_e4736);
        (assign3430_e4737,)
    } else {
        (locals.var_u0r_i,)
    }
};
        locals.var_u0r_i = assign3430_e4739;
        locals.var_u0r_i_rv = 0.0;

        let (assign3440_e4750,) = {
    if ((locals.var_guard23 != 0.0) && (locals.var_guard24 == 0.0)) {
        let assign3440_e4747: f64 = (1.0 - p.p274);
        let assign3440_e4748: f64 = (locals.var_u0_i * assign3440_e4747);
        (assign3440_e4748,)
    } else {
        (locals.var_u0_i,)
    }
};
        locals.var_u0_i = assign3440_e4750;
        locals.var_u0_i_rv = 0.0;

        let assign3450_e4753: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign3450_e4753;
        locals.var_guard26_rv = 0.0;

        let (assign3460_e4766,) = {
    if (((locals.var_guard23 != 0.0) && (locals.var_guard24 == 0.0)) && (locals.var_guard26 != 0.0)) {
        let assign3460_e4763: f64 = (1.0 - p.p274);
        let assign3460_e4764: f64 = (locals.var_u0r_i * assign3460_e4763);
        (assign3460_e4764,)
    } else {
        (locals.var_u0r_i,)
    }
};
        locals.var_u0r_i = assign3460_e4766;
        locals.var_u0r_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3470_e4789,) = {
    if (locals.var_guard23 == 0.0) {
        let assign3470_e4773: f64 = (-locals.var_leff);
        let assign3470_e4775: f64 = (assign3470_e4773 / p.p270);
        let assign3470_e4776: f64 = { let limited_exp_arg = assign3470_e4775; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign3470_e4777: f64 = (p.p269 * assign3470_e4776);
        let assign3470_e4778: f64 = (1.0 - assign3470_e4777);
        let assign3470_e4781: f64 = (-locals.var_leff);
        let assign3470_e4783: f64 = (assign3470_e4781 / p.p272);
        let assign3470_e4784: f64 = { let limited_exp_arg = assign3470_e4783; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign3470_e4785: f64 = (p.p271 * assign3470_e4784);
        let assign3470_e4786: f64 = (assign3470_e4778 - assign3470_e4785);
        let assign3470_e4787: f64 = (locals.var_u0_i * assign3470_e4786);
        (assign3470_e4787,)
    } else {
        (locals.var_u0_i,)
    }
};
        locals.var_u0_i = assign3470_e4789;
        locals.var_u0_i_rv = 0.0;

        let assign3480_e4792: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard27 = assign3480_e4792;
        locals.var_guard27_rv = 0.0;

        let (assign3490_e4817,) = {
    if ((locals.var_guard23 == 0.0) && (locals.var_guard27 != 0.0)) {
        let assign3490_e4801: f64 = (-locals.var_leff);
        let assign3490_e4803: f64 = (assign3490_e4801 / p.p270);
        let assign3490_e4804: f64 = { let limited_exp_arg = assign3490_e4803; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign3490_e4805: f64 = (p.p269 * assign3490_e4804);
        let assign3490_e4806: f64 = (1.0 - assign3490_e4805);
        let assign3490_e4809: f64 = (-locals.var_leff);
        let assign3490_e4811: f64 = (assign3490_e4809 / p.p272);
        let assign3490_e4812: f64 = { let limited_exp_arg = assign3490_e4811; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign3490_e4813: f64 = (p.p271 * assign3490_e4812);
        let assign3490_e4814: f64 = (assign3490_e4806 - assign3490_e4813);
        let assign3490_e4815: f64 = (locals.var_u0r_i * assign3490_e4814);
        (assign3490_e4815,)
    } else {
        (locals.var_u0r_i,)
    }
};
        locals.var_u0r_i = assign3490_e4817;
        locals.var_u0r_i_rv = 0.0;

        let assign3500_e4821: f64 = (locals.var_inv_l).powf(p.p286);
        let assign3500_e4824: f64 = (locals.var_inv_llong).powf(p.p286);
        let assign3500_e4825: f64 = (assign3500_e4821 - assign3500_e4824);
        let assign3500_e4827: f64 = (assign3500_e4825).max(0.0);
        let assign3500_e4828: f64 = (p.p285 * assign3500_e4827);
        locals.var_t0 = assign3500_e4828;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign3510_e4832: f64 = (locals.var_inv_w).powf(p.p288);
        let assign3510_e4835: f64 = (locals.var_inv_wwide).powf(p.p288);
        let assign3510_e4836: f64 = (assign3510_e4832 - assign3510_e4835);
        let assign3510_e4838: f64 = (assign3510_e4836).max(0.0);
        let assign3510_e4839: f64 = (p.p287 * assign3510_e4838);
        let assign3510_e4843: f64 = (locals.var_inv_wl).powf(p.p290);
        let assign3510_e4844: f64 = (p.p289 * assign3510_e4843);
        let assign3510_e4845: f64 = (assign3510_e4839 + assign3510_e4844);
        locals.var_t1 = assign3510_e4845;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign3520_e4849: f64 = (1.0 + locals.var_t0);
        let assign3520_e4851: f64 = (assign3520_e4849 + locals.var_t1);
        let assign3520_e4852: f64 = (locals.var_ua_i * assign3520_e4851);
        locals.var_ua_i = assign3520_e4852;
        locals.var_ua_i_dn0 = ((locals.var_ua_i_dn0 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_ua_i_dn2 = ((locals.var_ua_i_dn2 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_ua_i_dn3 = ((locals.var_ua_i_dn3 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_ua_i_dn4 = ((locals.var_ua_i_dn4 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_ua_i_dn5 = ((locals.var_ua_i_dn5 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_ua_i_dn6 = ((locals.var_ua_i_dn6 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_ua_i_dn7 = ((locals.var_ua_i_dn7 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_ua_i_dn8 = ((locals.var_ua_i_dn8 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_ua_i_dn9 = ((locals.var_ua_i_dn9 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_ua_i_dn10 = ((locals.var_ua_i_dn10 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_ua_i_dn11 = ((locals.var_ua_i_dn11 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_ua_i_dn12 = ((locals.var_ua_i_dn12 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_ua_i_dn13 = ((locals.var_ua_i_dn13 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_ua_i_dn14 = ((locals.var_ua_i_dn14 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));
        locals.var_ua_i_rv = 0.0;

        let assign3530_e4855: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard28 = assign3530_e4855;
        locals.var_guard28_rv = 0.0;

        let (assign3540_e4865, assign3540_e4865_d_n0, assign3540_e4865_d_n2, assign3540_e4865_d_n3, assign3540_e4865_d_n4, assign3540_e4865_d_n5, assign3540_e4865_d_n6, assign3540_e4865_d_n7, assign3540_e4865_d_n8, assign3540_e4865_d_n9, assign3540_e4865_d_n10, assign3540_e4865_d_n11, assign3540_e4865_d_n12, assign3540_e4865_d_n13, assign3540_e4865_d_n14,) = {
    if (locals.var_guard28 != 0.0) {
        let assign3540_e4860: f64 = (1.0 + locals.var_t0);
        let assign3540_e4862: f64 = (assign3540_e4860 + locals.var_t1);
        let assign3540_e4863: f64 = (locals.var_uar_i * assign3540_e4862);
        (assign3540_e4863, ((locals.var_uar_i_dn0 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_uar_i_dn2 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_uar_i_dn3 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_uar_i_dn4 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_uar_i_dn5 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_uar_i_dn6 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_uar_i_dn7 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_uar_i_dn8 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_uar_i_dn9 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_uar_i_dn10 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_uar_i_dn11 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_uar_i_dn12 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_uar_i_dn13 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_uar_i_dn14 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn14 + locals.var_t1_dn14))),)
    } else {
        (locals.var_uar_i, locals.var_uar_i_dn0, locals.var_uar_i_dn2, locals.var_uar_i_dn3, locals.var_uar_i_dn4, locals.var_uar_i_dn5, locals.var_uar_i_dn6, locals.var_uar_i_dn7, locals.var_uar_i_dn8, locals.var_uar_i_dn9, locals.var_uar_i_dn10, locals.var_uar_i_dn11, locals.var_uar_i_dn12, locals.var_uar_i_dn13, locals.var_uar_i_dn14,)
    }
};
        locals.var_uar_i = assign3540_e4865;
        locals.var_uar_i_dn0 = assign3540_e4865_d_n0;
        locals.var_uar_i_dn2 = assign3540_e4865_d_n2;
        locals.var_uar_i_dn3 = assign3540_e4865_d_n3;
        locals.var_uar_i_dn4 = assign3540_e4865_d_n4;
        locals.var_uar_i_dn5 = assign3540_e4865_d_n5;
        locals.var_uar_i_dn6 = assign3540_e4865_d_n6;
        locals.var_uar_i_dn7 = assign3540_e4865_d_n7;
        locals.var_uar_i_dn8 = assign3540_e4865_d_n8;
        locals.var_uar_i_dn9 = assign3540_e4865_d_n9;
        locals.var_uar_i_dn10 = assign3540_e4865_d_n10;
        locals.var_uar_i_dn11 = assign3540_e4865_d_n11;
        locals.var_uar_i_dn12 = assign3540_e4865_d_n12;
        locals.var_uar_i_dn13 = assign3540_e4865_d_n13;
        locals.var_uar_i_dn14 = assign3540_e4865_d_n14;
        locals.var_uar_i_rv = 0.0;

        let assign3550_e4869: f64 = (locals.var_inv_l).powf(p.p303);
        let assign3550_e4872: f64 = (locals.var_inv_llong).powf(p.p303);
        let assign3550_e4873: f64 = (assign3550_e4869 - assign3550_e4872);
        let assign3550_e4875: f64 = (assign3550_e4873).max(0.0);
        let assign3550_e4876: f64 = (p.p302 * assign3550_e4875);
        locals.var_t0 = assign3550_e4876;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign3560_e4880: f64 = (locals.var_inv_w).powf(p.p305);
        let assign3560_e4883: f64 = (locals.var_inv_wwide).powf(p.p305);
        let assign3560_e4884: f64 = (assign3560_e4880 - assign3560_e4883);
        let assign3560_e4886: f64 = (assign3560_e4884).max(0.0);
        let assign3560_e4887: f64 = (p.p304 * assign3560_e4886);
        let assign3560_e4891: f64 = (locals.var_inv_wl).powf(p.p307);
        let assign3560_e4892: f64 = (p.p306 * assign3560_e4891);
        let assign3560_e4893: f64 = (assign3560_e4887 + assign3560_e4892);
        locals.var_t1 = assign3560_e4893;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign3570_e4897: f64 = (1.0 + locals.var_t0);
        let assign3570_e4899: f64 = (assign3570_e4897 + locals.var_t1);
        let assign3570_e4900: f64 = (locals.var_eu_i * assign3570_e4899);
        locals.var_eu_i = assign3570_e4900;
        locals.var_eu_i_dn0 = ((locals.var_eu_i_dn0 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_eu_i_dn2 = ((locals.var_eu_i_dn2 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_eu_i_dn3 = ((locals.var_eu_i_dn3 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_eu_i_dn4 = ((locals.var_eu_i_dn4 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_eu_i_dn5 = ((locals.var_eu_i_dn5 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_eu_i_dn6 = ((locals.var_eu_i_dn6 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_eu_i_dn7 = ((locals.var_eu_i_dn7 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_eu_i_dn8 = ((locals.var_eu_i_dn8 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_eu_i_dn9 = ((locals.var_eu_i_dn9 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_eu_i_dn10 = ((locals.var_eu_i_dn10 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_eu_i_dn11 = ((locals.var_eu_i_dn11 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_eu_i_dn12 = ((locals.var_eu_i_dn12 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_eu_i_dn13 = ((locals.var_eu_i_dn13 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_eu_i_dn14 = ((locals.var_eu_i_dn14 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));
        locals.var_eu_i_rv = 0.0;

        let assign3580_e4905: f64 = (locals.var_inv_l).powf(p.p310);
        let assign3580_e4908: f64 = (locals.var_inv_llong).powf(p.p310);
        let assign3580_e4909: f64 = (assign3580_e4905 - assign3580_e4908);
        let assign3580_e4911: f64 = (assign3580_e4909).max(0.0);
        let assign3580_e4912: f64 = (p.p309 * assign3580_e4911);
        let assign3580_e4913: f64 = (1.0 + assign3580_e4912);
        locals.var_t0 = assign3580_e4913;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign3590_e4916: f64 = (locals.var_ud_i * locals.var_t0);
        locals.var_ud_i = assign3590_e4916;
        locals.var_ud_i_dn0 = ((locals.var_ud_i_dn0 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn0));
        locals.var_ud_i_dn2 = ((locals.var_ud_i_dn2 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn2));
        locals.var_ud_i_dn3 = ((locals.var_ud_i_dn3 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn3));
        locals.var_ud_i_dn4 = ((locals.var_ud_i_dn4 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn4));
        locals.var_ud_i_dn5 = ((locals.var_ud_i_dn5 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn5));
        locals.var_ud_i_dn6 = ((locals.var_ud_i_dn6 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn6));
        locals.var_ud_i_dn7 = ((locals.var_ud_i_dn7 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn7));
        locals.var_ud_i_dn8 = ((locals.var_ud_i_dn8 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn8));
        locals.var_ud_i_dn9 = ((locals.var_ud_i_dn9 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn9));
        locals.var_ud_i_dn10 = ((locals.var_ud_i_dn10 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn10));
        locals.var_ud_i_dn11 = ((locals.var_ud_i_dn11 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn11));
        locals.var_ud_i_dn12 = ((locals.var_ud_i_dn12 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn12));
        locals.var_ud_i_dn13 = ((locals.var_ud_i_dn13 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn13));
        locals.var_ud_i_dn14 = ((locals.var_ud_i_dn14 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn14));
        locals.var_ud_i_rv = 0.0;

        let assign3600_e4919: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign3600_e4919;
        locals.var_guard29_rv = 0.0;

        let (assign3610_e4925, assign3610_e4925_d_n0, assign3610_e4925_d_n2, assign3610_e4925_d_n3, assign3610_e4925_d_n4, assign3610_e4925_d_n5, assign3610_e4925_d_n6, assign3610_e4925_d_n7, assign3610_e4925_d_n8, assign3610_e4925_d_n9, assign3610_e4925_d_n10, assign3610_e4925_d_n11, assign3610_e4925_d_n12, assign3610_e4925_d_n13, assign3610_e4925_d_n14,) = {
    if (locals.var_guard29 != 0.0) {
        let assign3610_e4923: f64 = (locals.var_udr_i * locals.var_t0);
        (assign3610_e4923, ((locals.var_udr_i_dn0 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn0)), ((locals.var_udr_i_dn2 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn2)), ((locals.var_udr_i_dn3 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn3)), ((locals.var_udr_i_dn4 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn4)), ((locals.var_udr_i_dn5 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn5)), ((locals.var_udr_i_dn6 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn6)), ((locals.var_udr_i_dn7 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn7)), ((locals.var_udr_i_dn8 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn8)), ((locals.var_udr_i_dn9 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn9)), ((locals.var_udr_i_dn10 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn10)), ((locals.var_udr_i_dn11 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn11)), ((locals.var_udr_i_dn12 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn12)), ((locals.var_udr_i_dn13 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn13)), ((locals.var_udr_i_dn14 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn14)),)
    } else {
        (locals.var_udr_i, locals.var_udr_i_dn0, locals.var_udr_i_dn2, locals.var_udr_i_dn3, locals.var_udr_i_dn4, locals.var_udr_i_dn5, locals.var_udr_i_dn6, locals.var_udr_i_dn7, locals.var_udr_i_dn8, locals.var_udr_i_dn9, locals.var_udr_i_dn10, locals.var_udr_i_dn11, locals.var_udr_i_dn12, locals.var_udr_i_dn13, locals.var_udr_i_dn14,)
    }
};
        locals.var_udr_i = assign3610_e4925;
        locals.var_udr_i_dn0 = assign3610_e4925_d_n0;
        locals.var_udr_i_dn2 = assign3610_e4925_d_n2;
        locals.var_udr_i_dn3 = assign3610_e4925_d_n3;
        locals.var_udr_i_dn4 = assign3610_e4925_d_n4;
        locals.var_udr_i_dn5 = assign3610_e4925_d_n5;
        locals.var_udr_i_dn6 = assign3610_e4925_d_n6;
        locals.var_udr_i_dn7 = assign3610_e4925_d_n7;
        locals.var_udr_i_dn8 = assign3610_e4925_d_n8;
        locals.var_udr_i_dn9 = assign3610_e4925_d_n9;
        locals.var_udr_i_dn10 = assign3610_e4925_d_n10;
        locals.var_udr_i_dn11 = assign3610_e4925_d_n11;
        locals.var_udr_i_dn12 = assign3610_e4925_d_n12;
        locals.var_udr_i_dn13 = assign3610_e4925_d_n13;
        locals.var_udr_i_dn14 = assign3610_e4925_d_n14;
        locals.var_udr_i_rv = 0.0;

        let assign3620_e4929: f64 = (locals.var_inv_l).powf(p.p328);
        let assign3620_e4932: f64 = (locals.var_inv_llong).powf(p.p328);
        let assign3620_e4933: f64 = (assign3620_e4929 - assign3620_e4932);
        let assign3620_e4935: f64 = (assign3620_e4933).max(0.0);
        let assign3620_e4936: f64 = (p.p327 * assign3620_e4935);
        locals.var_t0 = assign3620_e4936;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign3630_e4940: f64 = (locals.var_inv_w).powf(p.p330);
        let assign3630_e4943: f64 = (locals.var_inv_wwide).powf(p.p330);
        let assign3630_e4944: f64 = (assign3630_e4940 - assign3630_e4943);
        let assign3630_e4946: f64 = (assign3630_e4944).max(0.0);
        let assign3630_e4947: f64 = (p.p329 * assign3630_e4946);
        let assign3630_e4951: f64 = (locals.var_inv_wl).powf(p.p332);
        let assign3630_e4952: f64 = (p.p331 * assign3630_e4951);
        let assign3630_e4953: f64 = (assign3630_e4947 + assign3630_e4952);
        locals.var_t1 = assign3630_e4953;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign3640_e4957: f64 = (1.0 + locals.var_t0);
        let assign3640_e4959: f64 = (assign3640_e4957 + locals.var_t1);
        let assign3640_e4960: f64 = (locals.var_uc_i * assign3640_e4959);
        locals.var_uc_i = assign3640_e4960;
        locals.var_uc_i_dn0 = ((locals.var_uc_i_dn0 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_uc_i_dn2 = ((locals.var_uc_i_dn2 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_uc_i_dn3 = ((locals.var_uc_i_dn3 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_uc_i_dn4 = ((locals.var_uc_i_dn4 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_uc_i_dn5 = ((locals.var_uc_i_dn5 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_uc_i_dn6 = ((locals.var_uc_i_dn6 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_uc_i_dn7 = ((locals.var_uc_i_dn7 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_uc_i_dn8 = ((locals.var_uc_i_dn8 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_uc_i_dn9 = ((locals.var_uc_i_dn9 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_uc_i_dn10 = ((locals.var_uc_i_dn10 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_uc_i_dn11 = ((locals.var_uc_i_dn11 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_uc_i_dn12 = ((locals.var_uc_i_dn12 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_uc_i_dn13 = ((locals.var_uc_i_dn13 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_uc_i_dn14 = ((locals.var_uc_i_dn14 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));
        locals.var_uc_i_rv = 0.0;

        let assign3650_e4963: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard30 = assign3650_e4963;
        locals.var_guard30_rv = 0.0;

        let (assign3660_e4973, assign3660_e4973_d_n0, assign3660_e4973_d_n2, assign3660_e4973_d_n3, assign3660_e4973_d_n4, assign3660_e4973_d_n5, assign3660_e4973_d_n6, assign3660_e4973_d_n7, assign3660_e4973_d_n8, assign3660_e4973_d_n9, assign3660_e4973_d_n10, assign3660_e4973_d_n11, assign3660_e4973_d_n12, assign3660_e4973_d_n13, assign3660_e4973_d_n14,) = {
    if (locals.var_guard30 != 0.0) {
        let assign3660_e4968: f64 = (1.0 + locals.var_t0);
        let assign3660_e4970: f64 = (assign3660_e4968 + locals.var_t1);
        let assign3660_e4971: f64 = (locals.var_ucr_i * assign3660_e4970);
        (assign3660_e4971, ((locals.var_ucr_i_dn0 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_ucr_i_dn2 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_ucr_i_dn3 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_ucr_i_dn4 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_ucr_i_dn5 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_ucr_i_dn6 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_ucr_i_dn7 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_ucr_i_dn8 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_ucr_i_dn9 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_ucr_i_dn10 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_ucr_i_dn11 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_ucr_i_dn12 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_ucr_i_dn13 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_ucr_i_dn14 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn14 + locals.var_t1_dn14))),)
    } else {
        (locals.var_ucr_i, locals.var_ucr_i_dn0, locals.var_ucr_i_dn2, locals.var_ucr_i_dn3, locals.var_ucr_i_dn4, locals.var_ucr_i_dn5, locals.var_ucr_i_dn6, locals.var_ucr_i_dn7, locals.var_ucr_i_dn8, locals.var_ucr_i_dn9, locals.var_ucr_i_dn10, locals.var_ucr_i_dn11, locals.var_ucr_i_dn12, locals.var_ucr_i_dn13, locals.var_ucr_i_dn14,)
    }
};
        locals.var_ucr_i = assign3660_e4973;
        locals.var_ucr_i_dn0 = assign3660_e4973_d_n0;
        locals.var_ucr_i_dn2 = assign3660_e4973_d_n2;
        locals.var_ucr_i_dn3 = assign3660_e4973_d_n3;
        locals.var_ucr_i_dn4 = assign3660_e4973_d_n4;
        locals.var_ucr_i_dn5 = assign3660_e4973_d_n5;
        locals.var_ucr_i_dn6 = assign3660_e4973_d_n6;
        locals.var_ucr_i_dn7 = assign3660_e4973_d_n7;
        locals.var_ucr_i_dn8 = assign3660_e4973_d_n8;
        locals.var_ucr_i_dn9 = assign3660_e4973_d_n9;
        locals.var_ucr_i_dn10 = assign3660_e4973_d_n10;
        locals.var_ucr_i_dn11 = assign3660_e4973_d_n11;
        locals.var_ucr_i_dn12 = assign3660_e4973_d_n12;
        locals.var_ucr_i_dn13 = assign3660_e4973_d_n13;
        locals.var_ucr_i_dn14 = assign3660_e4973_d_n14;
        locals.var_ucr_i_rv = 0.0;

        let assign3670_e4976: f64 = (locals.var_inv_l).powf(p.p179);
        let assign3670_e4979: f64 = (locals.var_inv_llong).powf(p.p179);
        let assign3670_e4980: f64 = (assign3670_e4976 - assign3670_e4979);
        let assign3670_e4982: f64 = (assign3670_e4980).max(0.0);
        locals.var_t0 = assign3670_e4982;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign3680_e4985: f64 = (locals.var_eta0_i * locals.var_t0);
        locals.var_eta0_i = assign3680_e4985;
        locals.var_eta0_i_dn0 = ((locals.var_eta0_i_dn0 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn0));
        locals.var_eta0_i_dn2 = ((locals.var_eta0_i_dn2 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn2));
        locals.var_eta0_i_dn3 = ((locals.var_eta0_i_dn3 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn3));
        locals.var_eta0_i_dn4 = ((locals.var_eta0_i_dn4 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn4));
        locals.var_eta0_i_dn5 = ((locals.var_eta0_i_dn5 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn5));
        locals.var_eta0_i_dn6 = ((locals.var_eta0_i_dn6 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn6));
        locals.var_eta0_i_dn7 = ((locals.var_eta0_i_dn7 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn7));
        locals.var_eta0_i_dn8 = ((locals.var_eta0_i_dn8 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn8));
        locals.var_eta0_i_dn9 = ((locals.var_eta0_i_dn9 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn9));
        locals.var_eta0_i_dn10 = ((locals.var_eta0_i_dn10 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn10));
        locals.var_eta0_i_dn11 = ((locals.var_eta0_i_dn11 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn11));
        locals.var_eta0_i_dn12 = ((locals.var_eta0_i_dn12 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn12));
        locals.var_eta0_i_dn13 = ((locals.var_eta0_i_dn13 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn13));
        locals.var_eta0_i_dn14 = ((locals.var_eta0_i_dn14 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn14));
        locals.var_eta0_i_rv = 0.0;

        let assign3690_e4988: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign3690_e4988;
        locals.var_guard31_rv = 0.0;

        let (assign3700_e4994, assign3700_e4994_d_n0, assign3700_e4994_d_n2, assign3700_e4994_d_n3, assign3700_e4994_d_n4, assign3700_e4994_d_n5, assign3700_e4994_d_n6, assign3700_e4994_d_n7, assign3700_e4994_d_n8, assign3700_e4994_d_n9, assign3700_e4994_d_n10, assign3700_e4994_d_n11, assign3700_e4994_d_n12, assign3700_e4994_d_n13, assign3700_e4994_d_n14,) = {
    if (locals.var_guard31 != 0.0) {
        let assign3700_e4992: f64 = (locals.var_eta0r_i * locals.var_t0);
        (assign3700_e4992, ((locals.var_eta0r_i_dn0 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn0)), ((locals.var_eta0r_i_dn2 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn2)), ((locals.var_eta0r_i_dn3 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn3)), ((locals.var_eta0r_i_dn4 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn4)), ((locals.var_eta0r_i_dn5 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn5)), ((locals.var_eta0r_i_dn6 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn6)), ((locals.var_eta0r_i_dn7 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn7)), ((locals.var_eta0r_i_dn8 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn8)), ((locals.var_eta0r_i_dn9 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn9)), ((locals.var_eta0r_i_dn10 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn10)), ((locals.var_eta0r_i_dn11 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn11)), ((locals.var_eta0r_i_dn12 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn12)), ((locals.var_eta0r_i_dn13 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn13)), ((locals.var_eta0r_i_dn14 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn14)),)
    } else {
        (locals.var_eta0r_i, locals.var_eta0r_i_dn0, locals.var_eta0r_i_dn2, locals.var_eta0r_i_dn3, locals.var_eta0r_i_dn4, locals.var_eta0r_i_dn5, locals.var_eta0r_i_dn6, locals.var_eta0r_i_dn7, locals.var_eta0r_i_dn8, locals.var_eta0r_i_dn9, locals.var_eta0r_i_dn10, locals.var_eta0r_i_dn11, locals.var_eta0r_i_dn12, locals.var_eta0r_i_dn13, locals.var_eta0r_i_dn14,)
    }
};
        locals.var_eta0r_i = assign3700_e4994;
        locals.var_eta0r_i_dn0 = assign3700_e4994_d_n0;
        locals.var_eta0r_i_dn2 = assign3700_e4994_d_n2;
        locals.var_eta0r_i_dn3 = assign3700_e4994_d_n3;
        locals.var_eta0r_i_dn4 = assign3700_e4994_d_n4;
        locals.var_eta0r_i_dn5 = assign3700_e4994_d_n5;
        locals.var_eta0r_i_dn6 = assign3700_e4994_d_n6;
        locals.var_eta0r_i_dn7 = assign3700_e4994_d_n7;
        locals.var_eta0r_i_dn8 = assign3700_e4994_d_n8;
        locals.var_eta0r_i_dn9 = assign3700_e4994_d_n9;
        locals.var_eta0r_i_dn10 = assign3700_e4994_d_n10;
        locals.var_eta0r_i_dn11 = assign3700_e4994_d_n11;
        locals.var_eta0r_i_dn12 = assign3700_e4994_d_n12;
        locals.var_eta0r_i_dn13 = assign3700_e4994_d_n13;
        locals.var_eta0r_i_dn14 = assign3700_e4994_d_n14;
        locals.var_eta0r_i_rv = 0.0;

        let assign3710_e4998: f64 = (locals.var_inv_l).powf(p.p181);
        let assign3710_e5001: f64 = (locals.var_inv_llong).powf(p.p181);
        let assign3710_e5002: f64 = (assign3710_e4998 - assign3710_e5001);
        let assign3710_e5004: f64 = (assign3710_e5002).max(0.0);
        let assign3710_e5005: f64 = (locals.var_etab_i * assign3710_e5004);
        locals.var_etab_i = assign3710_e5005;
        locals.var_etab_i_rv = 0.0;

        let assign3720_e5010: f64 = (locals.var_inv_l).powf(p.p462);
        let assign3720_e5013: f64 = (locals.var_inv_llong).powf(p.p462);
        let assign3720_e5014: f64 = (assign3720_e5010 - assign3720_e5013);
        let assign3720_e5016: f64 = (assign3720_e5014).max(0.0);
        let assign3720_e5017: f64 = (p.p461 * assign3720_e5016);
        let assign3720_e5018: f64 = (1.0 + assign3720_e5017);
        locals.var_t0 = assign3720_e5018;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign3730_e5021: f64 = (locals.var_pdiblc_i * locals.var_t0);
        locals.var_pdiblc_i = assign3730_e5021;
        locals.var_pdiblc_i_dn0 = ((locals.var_pdiblc_i_dn0 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn0));
        locals.var_pdiblc_i_dn2 = ((locals.var_pdiblc_i_dn2 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn2));
        locals.var_pdiblc_i_dn3 = ((locals.var_pdiblc_i_dn3 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn3));
        locals.var_pdiblc_i_dn4 = ((locals.var_pdiblc_i_dn4 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn4));
        locals.var_pdiblc_i_dn5 = ((locals.var_pdiblc_i_dn5 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn5));
        locals.var_pdiblc_i_dn6 = ((locals.var_pdiblc_i_dn6 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn6));
        locals.var_pdiblc_i_dn7 = ((locals.var_pdiblc_i_dn7 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn7));
        locals.var_pdiblc_i_dn8 = ((locals.var_pdiblc_i_dn8 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn8));
        locals.var_pdiblc_i_dn9 = ((locals.var_pdiblc_i_dn9 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn9));
        locals.var_pdiblc_i_dn10 = ((locals.var_pdiblc_i_dn10 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn10));
        locals.var_pdiblc_i_dn11 = ((locals.var_pdiblc_i_dn11 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn11));
        locals.var_pdiblc_i_dn12 = ((locals.var_pdiblc_i_dn12 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn12));
        locals.var_pdiblc_i_dn13 = ((locals.var_pdiblc_i_dn13 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn13));
        locals.var_pdiblc_i_dn14 = ((locals.var_pdiblc_i_dn14 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn14));
        locals.var_pdiblc_i_rv = 0.0;

        let assign3740_e5024: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign3740_e5024;
        locals.var_guard32_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3750_e5030, assign3750_e5030_d_n0, assign3750_e5030_d_n2, assign3750_e5030_d_n3, assign3750_e5030_d_n4, assign3750_e5030_d_n5, assign3750_e5030_d_n6, assign3750_e5030_d_n7, assign3750_e5030_d_n8, assign3750_e5030_d_n9, assign3750_e5030_d_n10, assign3750_e5030_d_n11, assign3750_e5030_d_n12, assign3750_e5030_d_n13, assign3750_e5030_d_n14,) = {
    if (locals.var_guard32 != 0.0) {
        let assign3750_e5028: f64 = (locals.var_pdiblcr_i * locals.var_t0);
        (assign3750_e5028, ((locals.var_pdiblcr_i_dn0 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn0)), ((locals.var_pdiblcr_i_dn2 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn2)), ((locals.var_pdiblcr_i_dn3 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn3)), ((locals.var_pdiblcr_i_dn4 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn4)), ((locals.var_pdiblcr_i_dn5 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn5)), ((locals.var_pdiblcr_i_dn6 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn6)), ((locals.var_pdiblcr_i_dn7 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn7)), ((locals.var_pdiblcr_i_dn8 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn8)), ((locals.var_pdiblcr_i_dn9 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn9)), ((locals.var_pdiblcr_i_dn10 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn10)), ((locals.var_pdiblcr_i_dn11 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn11)), ((locals.var_pdiblcr_i_dn12 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn12)), ((locals.var_pdiblcr_i_dn13 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn13)), ((locals.var_pdiblcr_i_dn14 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn14)),)
    } else {
        (locals.var_pdiblcr_i, locals.var_pdiblcr_i_dn0, locals.var_pdiblcr_i_dn2, locals.var_pdiblcr_i_dn3, locals.var_pdiblcr_i_dn4, locals.var_pdiblcr_i_dn5, locals.var_pdiblcr_i_dn6, locals.var_pdiblcr_i_dn7, locals.var_pdiblcr_i_dn8, locals.var_pdiblcr_i_dn9, locals.var_pdiblcr_i_dn10, locals.var_pdiblcr_i_dn11, locals.var_pdiblcr_i_dn12, locals.var_pdiblcr_i_dn13, locals.var_pdiblcr_i_dn14,)
    }
};
        locals.var_pdiblcr_i = assign3750_e5030;
        locals.var_pdiblcr_i_dn0 = assign3750_e5030_d_n0;
        locals.var_pdiblcr_i_dn2 = assign3750_e5030_d_n2;
        locals.var_pdiblcr_i_dn3 = assign3750_e5030_d_n3;
        locals.var_pdiblcr_i_dn4 = assign3750_e5030_d_n4;
        locals.var_pdiblcr_i_dn5 = assign3750_e5030_d_n5;
        locals.var_pdiblcr_i_dn6 = assign3750_e5030_d_n6;
        locals.var_pdiblcr_i_dn7 = assign3750_e5030_d_n7;
        locals.var_pdiblcr_i_dn8 = assign3750_e5030_d_n8;
        locals.var_pdiblcr_i_dn9 = assign3750_e5030_d_n9;
        locals.var_pdiblcr_i_dn10 = assign3750_e5030_d_n10;
        locals.var_pdiblcr_i_dn11 = assign3750_e5030_d_n11;
        locals.var_pdiblcr_i_dn12 = assign3750_e5030_d_n12;
        locals.var_pdiblcr_i_dn13 = assign3750_e5030_d_n13;
        locals.var_pdiblcr_i_dn14 = assign3750_e5030_d_n14;
        locals.var_pdiblcr_i_rv = 0.0;

        let assign3760_e5036: f64 = (locals.var_inv_l).powf(p.p258);
        let assign3760_e5039: f64 = (locals.var_inv_llong).powf(p.p258);
        let assign3760_e5040: f64 = (assign3760_e5036 - assign3760_e5039);
        let assign3760_e5042: f64 = (assign3760_e5040).max(0.0);
        let assign3760_e5043: f64 = (p.p257 * assign3760_e5042);
        let assign3760_e5044: f64 = (1.0 + assign3760_e5043);
        let assign3760_e5045: f64 = (locals.var_delta_i * assign3760_e5044);
        locals.var_t0 = assign3760_e5045;
        locals.var_t0_dn0 = (locals.var_delta_i_dn0 * assign3760_e5044);
        locals.var_t0_dn2 = (locals.var_delta_i_dn2 * assign3760_e5044);
        locals.var_t0_dn3 = (locals.var_delta_i_dn3 * assign3760_e5044);
        locals.var_t0_dn4 = (locals.var_delta_i_dn4 * assign3760_e5044);
        locals.var_t0_dn5 = (locals.var_delta_i_dn5 * assign3760_e5044);
        locals.var_t0_dn6 = (locals.var_delta_i_dn6 * assign3760_e5044);
        locals.var_t0_dn7 = (locals.var_delta_i_dn7 * assign3760_e5044);
        locals.var_t0_dn8 = (locals.var_delta_i_dn8 * assign3760_e5044);
        locals.var_t0_dn9 = (locals.var_delta_i_dn9 * assign3760_e5044);
        locals.var_t0_dn10 = (locals.var_delta_i_dn10 * assign3760_e5044);
        locals.var_t0_dn11 = (locals.var_delta_i_dn11 * assign3760_e5044);
        locals.var_t0_dn12 = (locals.var_delta_i_dn12 * assign3760_e5044);
        locals.var_t0_dn13 = (locals.var_delta_i_dn13 * assign3760_e5044);
        locals.var_t0_dn14 = (locals.var_delta_i_dn14 * assign3760_e5044);
        locals.var_t0_rv = 0.0;

        let assign3770_e5048: f64 = (locals.var_t0).min(0.5);
        locals.var_delta_i = assign3770_e5048;
        locals.var_delta_i_dn0 = if locals.var_t0 <= 0.5 { locals.var_t0_dn0 } else { 0.0 };
        locals.var_delta_i_dn2 = if locals.var_t0 <= 0.5 { locals.var_t0_dn2 } else { 0.0 };
        locals.var_delta_i_dn3 = if locals.var_t0 <= 0.5 { locals.var_t0_dn3 } else { 0.0 };
        locals.var_delta_i_dn4 = if locals.var_t0 <= 0.5 { locals.var_t0_dn4 } else { 0.0 };
        locals.var_delta_i_dn5 = if locals.var_t0 <= 0.5 { locals.var_t0_dn5 } else { 0.0 };
        locals.var_delta_i_dn6 = if locals.var_t0 <= 0.5 { locals.var_t0_dn6 } else { 0.0 };
        locals.var_delta_i_dn7 = if locals.var_t0 <= 0.5 { locals.var_t0_dn7 } else { 0.0 };
        locals.var_delta_i_dn8 = if locals.var_t0 <= 0.5 { locals.var_t0_dn8 } else { 0.0 };
        locals.var_delta_i_dn9 = if locals.var_t0 <= 0.5 { locals.var_t0_dn9 } else { 0.0 };
        locals.var_delta_i_dn10 = if locals.var_t0 <= 0.5 { locals.var_t0_dn10 } else { 0.0 };
        locals.var_delta_i_dn11 = if locals.var_t0 <= 0.5 { locals.var_t0_dn11 } else { 0.0 };
        locals.var_delta_i_dn12 = if locals.var_t0 <= 0.5 { locals.var_t0_dn12 } else { 0.0 };
        locals.var_delta_i_dn13 = if locals.var_t0 <= 0.5 { locals.var_t0_dn13 } else { 0.0 };
        locals.var_delta_i_dn14 = if locals.var_t0 <= 0.5 { locals.var_t0_dn14 } else { 0.0 };
        locals.var_delta_i_rv = 0.0;

        let assign3780_e5054: f64 = (locals.var_inv_l).powf(p.p480);
        let assign3780_e5057: f64 = (locals.var_inv_llong).powf(p.p480);
        let assign3780_e5058: f64 = (assign3780_e5054 - assign3780_e5057);
        let assign3780_e5060: f64 = (assign3780_e5058).max(0.0);
        let assign3780_e5061: f64 = (p.p479 * assign3780_e5060);
        let assign3780_e5062: f64 = (1.0 + assign3780_e5061);
        let assign3780_e5063: f64 = (locals.var_fprout_i * assign3780_e5062);
        locals.var_fprout_i = assign3780_e5063;
        locals.var_fprout_i_rv = 0.0;

        let assign3790_e5068: f64 = (locals.var_inv_l).powf(p.p342);
        let assign3790_e5071: f64 = (locals.var_inv_llong).powf(p.p342);
        let assign3790_e5072: f64 = (assign3790_e5068 - assign3790_e5071);
        let assign3790_e5074: f64 = (assign3790_e5072).max(0.0);
        let assign3790_e5075: f64 = (p.p341 * assign3790_e5074);
        let assign3790_e5076: f64 = (1.0 + assign3790_e5075);
        locals.var_t0 = assign3790_e5076;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign3800_e5079: f64 = (locals.var_pclm_i * locals.var_t0);
        locals.var_pclm_i = assign3800_e5079;
        locals.var_pclm_i_dn0 = ((locals.var_pclm_i_dn0 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn0));
        locals.var_pclm_i_dn2 = ((locals.var_pclm_i_dn2 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn2));
        locals.var_pclm_i_dn3 = ((locals.var_pclm_i_dn3 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn3));
        locals.var_pclm_i_dn4 = ((locals.var_pclm_i_dn4 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn4));
        locals.var_pclm_i_dn5 = ((locals.var_pclm_i_dn5 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn5));
        locals.var_pclm_i_dn6 = ((locals.var_pclm_i_dn6 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn6));
        locals.var_pclm_i_dn7 = ((locals.var_pclm_i_dn7 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn7));
        locals.var_pclm_i_dn8 = ((locals.var_pclm_i_dn8 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn8));
        locals.var_pclm_i_dn9 = ((locals.var_pclm_i_dn9 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn9));
        locals.var_pclm_i_dn10 = ((locals.var_pclm_i_dn10 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn10));
        locals.var_pclm_i_dn11 = ((locals.var_pclm_i_dn11 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn11));
        locals.var_pclm_i_dn12 = ((locals.var_pclm_i_dn12 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn12));
        locals.var_pclm_i_dn13 = ((locals.var_pclm_i_dn13 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn13));
        locals.var_pclm_i_dn14 = ((locals.var_pclm_i_dn14 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn14));
        locals.var_pclm_i_rv = 0.0;

        let assign3810_e5082: f64 = (locals.var_pclm_i).max(0.0);
        locals.var_pclm_i = assign3810_e5082;
        locals.var_pclm_i_dn0 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn0 } else { 0.0 };
        locals.var_pclm_i_dn2 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn2 } else { 0.0 };
        locals.var_pclm_i_dn3 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn3 } else { 0.0 };
        locals.var_pclm_i_dn4 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn4 } else { 0.0 };
        locals.var_pclm_i_dn5 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn5 } else { 0.0 };
        locals.var_pclm_i_dn6 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn6 } else { 0.0 };
        locals.var_pclm_i_dn7 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn7 } else { 0.0 };
        locals.var_pclm_i_dn8 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn8 } else { 0.0 };
        locals.var_pclm_i_dn9 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn9 } else { 0.0 };
        locals.var_pclm_i_dn10 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn10 } else { 0.0 };
        locals.var_pclm_i_dn11 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn11 } else { 0.0 };
        locals.var_pclm_i_dn12 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn12 } else { 0.0 };
        locals.var_pclm_i_dn13 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn13 } else { 0.0 };
        locals.var_pclm_i_dn14 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn14 } else { 0.0 };
        locals.var_pclm_i_rv = 0.0;

        let assign3820_e5085: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign3820_e5085;
        locals.var_guard33_rv = 0.0;

        let (assign3830_e5091, assign3830_e5091_d_n0, assign3830_e5091_d_n2, assign3830_e5091_d_n3, assign3830_e5091_d_n4, assign3830_e5091_d_n5, assign3830_e5091_d_n6, assign3830_e5091_d_n7, assign3830_e5091_d_n8, assign3830_e5091_d_n9, assign3830_e5091_d_n10, assign3830_e5091_d_n11, assign3830_e5091_d_n12, assign3830_e5091_d_n13, assign3830_e5091_d_n14,) = {
    if (locals.var_guard33 != 0.0) {
        let assign3830_e5089: f64 = (locals.var_pclmr_i * locals.var_t0);
        (assign3830_e5089, ((locals.var_pclmr_i_dn0 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn0)), ((locals.var_pclmr_i_dn2 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn2)), ((locals.var_pclmr_i_dn3 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn3)), ((locals.var_pclmr_i_dn4 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn4)), ((locals.var_pclmr_i_dn5 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn5)), ((locals.var_pclmr_i_dn6 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn6)), ((locals.var_pclmr_i_dn7 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn7)), ((locals.var_pclmr_i_dn8 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn8)), ((locals.var_pclmr_i_dn9 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn9)), ((locals.var_pclmr_i_dn10 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn10)), ((locals.var_pclmr_i_dn11 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn11)), ((locals.var_pclmr_i_dn12 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn12)), ((locals.var_pclmr_i_dn13 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn13)), ((locals.var_pclmr_i_dn14 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn14)),)
    } else {
        (locals.var_pclmr_i, locals.var_pclmr_i_dn0, locals.var_pclmr_i_dn2, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11, locals.var_pclmr_i_dn12, locals.var_pclmr_i_dn13, locals.var_pclmr_i_dn14,)
    }
};
        locals.var_pclmr_i = assign3830_e5091;
        locals.var_pclmr_i_dn0 = assign3830_e5091_d_n0;
        locals.var_pclmr_i_dn2 = assign3830_e5091_d_n2;
        locals.var_pclmr_i_dn3 = assign3830_e5091_d_n3;
        locals.var_pclmr_i_dn4 = assign3830_e5091_d_n4;
        locals.var_pclmr_i_dn5 = assign3830_e5091_d_n5;
        locals.var_pclmr_i_dn6 = assign3830_e5091_d_n6;
        locals.var_pclmr_i_dn7 = assign3830_e5091_d_n7;
        locals.var_pclmr_i_dn8 = assign3830_e5091_d_n8;
        locals.var_pclmr_i_dn9 = assign3830_e5091_d_n9;
        locals.var_pclmr_i_dn10 = assign3830_e5091_d_n10;
        locals.var_pclmr_i_dn11 = assign3830_e5091_d_n11;
        locals.var_pclmr_i_dn12 = assign3830_e5091_d_n12;
        locals.var_pclmr_i_dn13 = assign3830_e5091_d_n13;
        locals.var_pclmr_i_dn14 = assign3830_e5091_d_n14;
        locals.var_pclmr_i_rv = 0.0;

        let (assign3840_e5097, assign3840_e5097_d_n0, assign3840_e5097_d_n2, assign3840_e5097_d_n3, assign3840_e5097_d_n4, assign3840_e5097_d_n5, assign3840_e5097_d_n6, assign3840_e5097_d_n7, assign3840_e5097_d_n8, assign3840_e5097_d_n9, assign3840_e5097_d_n10, assign3840_e5097_d_n11, assign3840_e5097_d_n12, assign3840_e5097_d_n13, assign3840_e5097_d_n14,) = {
    if (locals.var_guard33 != 0.0) {
        let assign3840_e5095: f64 = (locals.var_pclmr_i).max(0.0);
        (assign3840_e5095, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn0 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn2 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn3 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn4 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn5 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn6 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn7 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn8 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn9 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn10 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn11 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn12 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn13 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn14 } else { 0.0 },)
    } else {
        (locals.var_pclmr_i, locals.var_pclmr_i_dn0, locals.var_pclmr_i_dn2, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11, locals.var_pclmr_i_dn12, locals.var_pclmr_i_dn13, locals.var_pclmr_i_dn14,)
    }
};
        locals.var_pclmr_i = assign3840_e5097;
        locals.var_pclmr_i_dn0 = assign3840_e5097_d_n0;
        locals.var_pclmr_i_dn2 = assign3840_e5097_d_n2;
        locals.var_pclmr_i_dn3 = assign3840_e5097_d_n3;
        locals.var_pclmr_i_dn4 = assign3840_e5097_d_n4;
        locals.var_pclmr_i_dn5 = assign3840_e5097_d_n5;
        locals.var_pclmr_i_dn6 = assign3840_e5097_d_n6;
        locals.var_pclmr_i_dn7 = assign3840_e5097_d_n7;
        locals.var_pclmr_i_dn8 = assign3840_e5097_d_n8;
        locals.var_pclmr_i_dn9 = assign3840_e5097_d_n9;
        locals.var_pclmr_i_dn10 = assign3840_e5097_d_n10;
        locals.var_pclmr_i_dn11 = assign3840_e5097_d_n11;
        locals.var_pclmr_i_dn12 = assign3840_e5097_d_n12;
        locals.var_pclmr_i_dn13 = assign3840_e5097_d_n13;
        locals.var_pclmr_i_dn14 = assign3840_e5097_d_n14;
        locals.var_pclmr_i_rv = 0.0;

        let assign3850_e5101: f64 = (locals.var_inv_l).powf(p.p244);
        let assign3850_e5104: f64 = (locals.var_inv_llong).powf(p.p244);
        let assign3850_e5105: f64 = (assign3850_e5101 - assign3850_e5104);
        let assign3850_e5107: f64 = (assign3850_e5105).max(0.0);
        let assign3850_e5108: f64 = (p.p243 * assign3850_e5107);
        locals.var_t0 = assign3850_e5108;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign3860_e5112: f64 = (locals.var_inv_w).powf(p.p246);
        let assign3860_e5115: f64 = (locals.var_inv_wwide).powf(p.p246);
        let assign3860_e5116: f64 = (assign3860_e5112 - assign3860_e5115);
        let assign3860_e5118: f64 = (assign3860_e5116).max(0.0);
        let assign3860_e5119: f64 = (p.p245 * assign3860_e5118);
        let assign3860_e5123: f64 = (locals.var_inv_wl).powf(p.p248);
        let assign3860_e5124: f64 = (p.p247 * assign3860_e5123);
        let assign3860_e5125: f64 = (assign3860_e5119 + assign3860_e5124);
        locals.var_t1 = assign3860_e5125;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign3870_e5129: f64 = (1.0 + locals.var_t0);
        let assign3870_e5131: f64 = (assign3870_e5129 + locals.var_t1);
        let assign3870_e5132: f64 = (locals.var_vsat_i * assign3870_e5131);
        locals.var_vsat_i = assign3870_e5132;
        locals.var_vsat_i_dn0 = ((locals.var_vsat_i_dn0 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_vsat_i_dn2 = ((locals.var_vsat_i_dn2 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_vsat_i_dn3 = ((locals.var_vsat_i_dn3 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_vsat_i_dn4 = ((locals.var_vsat_i_dn4 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_vsat_i_dn5 = ((locals.var_vsat_i_dn5 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_vsat_i_dn6 = ((locals.var_vsat_i_dn6 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_vsat_i_dn7 = ((locals.var_vsat_i_dn7 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_vsat_i_dn8 = ((locals.var_vsat_i_dn8 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_vsat_i_dn9 = ((locals.var_vsat_i_dn9 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_vsat_i_dn10 = ((locals.var_vsat_i_dn10 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_vsat_i_dn11 = ((locals.var_vsat_i_dn11 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_vsat_i_dn12 = ((locals.var_vsat_i_dn12 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_vsat_i_dn13 = ((locals.var_vsat_i_dn13 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_vsat_i_dn14 = ((locals.var_vsat_i_dn14 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));
        locals.var_vsat_i_rv = 0.0;

        let assign3880_e5135: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign3880_e5135;
        locals.var_guard34_rv = 0.0;

        let (assign3890_e5145, assign3890_e5145_d_n0, assign3890_e5145_d_n2, assign3890_e5145_d_n3, assign3890_e5145_d_n4, assign3890_e5145_d_n5, assign3890_e5145_d_n6, assign3890_e5145_d_n7, assign3890_e5145_d_n8, assign3890_e5145_d_n9, assign3890_e5145_d_n10, assign3890_e5145_d_n11, assign3890_e5145_d_n12, assign3890_e5145_d_n13, assign3890_e5145_d_n14,) = {
    if (locals.var_guard34 != 0.0) {
        let assign3890_e5140: f64 = (1.0 + locals.var_t0);
        let assign3890_e5142: f64 = (assign3890_e5140 + locals.var_t1);
        let assign3890_e5143: f64 = (locals.var_vsatr_i * assign3890_e5142);
        (assign3890_e5143, ((locals.var_vsatr_i_dn0 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_vsatr_i_dn2 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_vsatr_i_dn3 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_vsatr_i_dn4 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_vsatr_i_dn5 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_vsatr_i_dn6 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_vsatr_i_dn7 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_vsatr_i_dn8 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_vsatr_i_dn9 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_vsatr_i_dn10 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_vsatr_i_dn11 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_vsatr_i_dn12 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_vsatr_i_dn13 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_vsatr_i_dn14 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn14 + locals.var_t1_dn14))),)
    } else {
        (locals.var_vsatr_i, locals.var_vsatr_i_dn0, locals.var_vsatr_i_dn2, locals.var_vsatr_i_dn3, locals.var_vsatr_i_dn4, locals.var_vsatr_i_dn5, locals.var_vsatr_i_dn6, locals.var_vsatr_i_dn7, locals.var_vsatr_i_dn8, locals.var_vsatr_i_dn9, locals.var_vsatr_i_dn10, locals.var_vsatr_i_dn11, locals.var_vsatr_i_dn12, locals.var_vsatr_i_dn13, locals.var_vsatr_i_dn14,)
    }
};
        locals.var_vsatr_i = assign3890_e5145;
        locals.var_vsatr_i_dn0 = assign3890_e5145_d_n0;
        locals.var_vsatr_i_dn2 = assign3890_e5145_d_n2;
        locals.var_vsatr_i_dn3 = assign3890_e5145_d_n3;
        locals.var_vsatr_i_dn4 = assign3890_e5145_d_n4;
        locals.var_vsatr_i_dn5 = assign3890_e5145_d_n5;
        locals.var_vsatr_i_dn6 = assign3890_e5145_d_n6;
        locals.var_vsatr_i_dn7 = assign3890_e5145_d_n7;
        locals.var_vsatr_i_dn8 = assign3890_e5145_d_n8;
        locals.var_vsatr_i_dn9 = assign3890_e5145_d_n9;
        locals.var_vsatr_i_dn10 = assign3890_e5145_d_n10;
        locals.var_vsatr_i_dn11 = assign3890_e5145_d_n11;
        locals.var_vsatr_i_dn12 = assign3890_e5145_d_n12;
        locals.var_vsatr_i_dn13 = assign3890_e5145_d_n13;
        locals.var_vsatr_i_dn14 = assign3890_e5145_d_n14;
        locals.var_vsatr_i_rv = 0.0;

        let assign3900_e5151: f64 = (locals.var_inv_l).powf(p.p424);
        let assign3900_e5154: f64 = (locals.var_inv_llong).powf(p.p424);
        let assign3900_e5155: f64 = (assign3900_e5151 - assign3900_e5154);
        let assign3900_e5157: f64 = (assign3900_e5155).max(0.0);
        let assign3900_e5158: f64 = (p.p423 * assign3900_e5157);
        let assign3900_e5159: f64 = (1.0 + assign3900_e5158);
        let assign3900_e5160: f64 = (locals.var_psat_i * assign3900_e5159);
        let assign3900_e5162: f64 = (assign3900_e5160).max(0.25);
        locals.var_psat_i = assign3900_e5162;
        locals.var_psat_i_rv = 0.0;

        let assign3910_e5165: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign3910_e5165;
        locals.var_guard35_rv = 0.0;

        let (assign3920_e5185,) = {
    if (locals.var_guard35 != 0.0) {
        let assign3920_e5172: f64 = (locals.var_inv_l).powf(p.p424);
        let assign3920_e5175: f64 = (locals.var_inv_llong).powf(p.p424);
        let assign3920_e5176: f64 = (assign3920_e5172 - assign3920_e5175);
        let assign3920_e5178: f64 = (assign3920_e5176).max(0.0);
        let assign3920_e5179: f64 = (p.p423 * assign3920_e5178);
        let assign3920_e5180: f64 = (1.0 + assign3920_e5179);
        let assign3920_e5181: f64 = (locals.var_psatr_i * assign3920_e5180);
        let assign3920_e5183: f64 = (assign3920_e5181).max(0.25);
        (assign3920_e5183,)
    } else {
        (locals.var_psatr_i,)
    }
};
        locals.var_psatr_i = assign3920_e5185;
        locals.var_psatr_i_rv = 0.0;

        let assign3930_e5190: f64 = (locals.var_inv_l).powf(p.p439);
        let assign3930_e5193: f64 = (locals.var_inv_llong).powf(p.p439);
        let assign3930_e5194: f64 = (assign3930_e5190 - assign3930_e5193);
        let assign3930_e5196: f64 = (assign3930_e5194).max(0.0);
        let assign3930_e5197: f64 = (p.p438 * assign3930_e5196);
        let assign3930_e5198: f64 = (1.0 + assign3930_e5197);
        locals.var_t0 = assign3930_e5198;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign3940_e5201: f64 = (locals.var_ptwg_i * locals.var_t0);
        locals.var_ptwg_i = assign3940_e5201;
        locals.var_ptwg_i_dn0 = ((locals.var_ptwg_i_dn0 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn0));
        locals.var_ptwg_i_dn2 = ((locals.var_ptwg_i_dn2 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn2));
        locals.var_ptwg_i_dn3 = ((locals.var_ptwg_i_dn3 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn3));
        locals.var_ptwg_i_dn4 = ((locals.var_ptwg_i_dn4 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn4));
        locals.var_ptwg_i_dn5 = ((locals.var_ptwg_i_dn5 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn5));
        locals.var_ptwg_i_dn6 = ((locals.var_ptwg_i_dn6 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn6));
        locals.var_ptwg_i_dn7 = ((locals.var_ptwg_i_dn7 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn7));
        locals.var_ptwg_i_dn8 = ((locals.var_ptwg_i_dn8 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn8));
        locals.var_ptwg_i_dn9 = ((locals.var_ptwg_i_dn9 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn9));
        locals.var_ptwg_i_dn10 = ((locals.var_ptwg_i_dn10 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn10));
        locals.var_ptwg_i_dn11 = ((locals.var_ptwg_i_dn11 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn11));
        locals.var_ptwg_i_dn12 = ((locals.var_ptwg_i_dn12 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn12));
        locals.var_ptwg_i_dn13 = ((locals.var_ptwg_i_dn13 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn13));
        locals.var_ptwg_i_dn14 = ((locals.var_ptwg_i_dn14 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn14));
        locals.var_ptwg_i_rv = 0.0;

        let assign3950_e5204: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign3950_e5204;
        locals.var_guard36_rv = 0.0;

        let (assign3960_e5210, assign3960_e5210_d_n0, assign3960_e5210_d_n2, assign3960_e5210_d_n3, assign3960_e5210_d_n4, assign3960_e5210_d_n5, assign3960_e5210_d_n6, assign3960_e5210_d_n7, assign3960_e5210_d_n8, assign3960_e5210_d_n9, assign3960_e5210_d_n10, assign3960_e5210_d_n11, assign3960_e5210_d_n12, assign3960_e5210_d_n13, assign3960_e5210_d_n14,) = {
    if (locals.var_guard36 != 0.0) {
        let assign3960_e5208: f64 = (locals.var_ptwgr_i * locals.var_t0);
        (assign3960_e5208, ((locals.var_ptwgr_i_dn0 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn0)), ((locals.var_ptwgr_i_dn2 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn2)), ((locals.var_ptwgr_i_dn3 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn3)), ((locals.var_ptwgr_i_dn4 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn4)), ((locals.var_ptwgr_i_dn5 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn5)), ((locals.var_ptwgr_i_dn6 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn6)), ((locals.var_ptwgr_i_dn7 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn7)), ((locals.var_ptwgr_i_dn8 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn8)), ((locals.var_ptwgr_i_dn9 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn9)), ((locals.var_ptwgr_i_dn10 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn10)), ((locals.var_ptwgr_i_dn11 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn11)), ((locals.var_ptwgr_i_dn12 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn12)), ((locals.var_ptwgr_i_dn13 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn13)), ((locals.var_ptwgr_i_dn14 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn14)),)
    } else {
        (locals.var_ptwgr_i, locals.var_ptwgr_i_dn0, locals.var_ptwgr_i_dn2, locals.var_ptwgr_i_dn3, locals.var_ptwgr_i_dn4, locals.var_ptwgr_i_dn5, locals.var_ptwgr_i_dn6, locals.var_ptwgr_i_dn7, locals.var_ptwgr_i_dn8, locals.var_ptwgr_i_dn9, locals.var_ptwgr_i_dn10, locals.var_ptwgr_i_dn11, locals.var_ptwgr_i_dn12, locals.var_ptwgr_i_dn13, locals.var_ptwgr_i_dn14,)
    }
};
        locals.var_ptwgr_i = assign3960_e5210;
        locals.var_ptwgr_i_dn0 = assign3960_e5210_d_n0;
        locals.var_ptwgr_i_dn2 = assign3960_e5210_d_n2;
        locals.var_ptwgr_i_dn3 = assign3960_e5210_d_n3;
        locals.var_ptwgr_i_dn4 = assign3960_e5210_d_n4;
        locals.var_ptwgr_i_dn5 = assign3960_e5210_d_n5;
        locals.var_ptwgr_i_dn6 = assign3960_e5210_d_n6;
        locals.var_ptwgr_i_dn7 = assign3960_e5210_d_n7;
        locals.var_ptwgr_i_dn8 = assign3960_e5210_d_n8;
        locals.var_ptwgr_i_dn9 = assign3960_e5210_d_n9;
        locals.var_ptwgr_i_dn10 = assign3960_e5210_d_n10;
        locals.var_ptwgr_i_dn11 = assign3960_e5210_d_n11;
        locals.var_ptwgr_i_dn12 = assign3960_e5210_d_n12;
        locals.var_ptwgr_i_dn13 = assign3960_e5210_d_n13;
        locals.var_ptwgr_i_dn14 = assign3960_e5210_d_n14;
        locals.var_ptwgr_i_rv = 0.0;

        let assign3970_e5214: f64 = (locals.var_inv_l).powf(p.p486);
        let assign3970_e5217: f64 = (locals.var_inv_llong).powf(p.p486);
        let assign3970_e5218: f64 = (assign3970_e5214 - assign3970_e5217);
        let assign3970_e5220: f64 = (assign3970_e5218).max(0.0);
        let assign3970_e5221: f64 = (p.p485 * assign3970_e5220);
        locals.var_t0 = assign3970_e5221;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign3980_e5225: f64 = (locals.var_inv_w).powf(p.p488);
        let assign3980_e5228: f64 = (locals.var_inv_wwide).powf(p.p488);
        let assign3980_e5229: f64 = (assign3980_e5225 - assign3980_e5228);
        let assign3980_e5231: f64 = (assign3980_e5229).max(0.0);
        let assign3980_e5232: f64 = (p.p487 * assign3980_e5231);
        locals.var_t1 = assign3980_e5232;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign3990_e5236: f64 = (1.0 + locals.var_t0);
        let assign3990_e5238: f64 = (assign3990_e5236 + locals.var_t1);
        let assign3990_e5239: f64 = (locals.var_alpha0_i * assign3990_e5238);
        locals.var_alpha0_i = assign3990_e5239;
        locals.var_alpha0_i_dn0 = ((locals.var_alpha0_i_dn0 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_alpha0_i_dn2 = ((locals.var_alpha0_i_dn2 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_alpha0_i_dn3 = ((locals.var_alpha0_i_dn3 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_alpha0_i_dn4 = ((locals.var_alpha0_i_dn4 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_alpha0_i_dn5 = ((locals.var_alpha0_i_dn5 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_alpha0_i_dn6 = ((locals.var_alpha0_i_dn6 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_alpha0_i_dn7 = ((locals.var_alpha0_i_dn7 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_alpha0_i_dn8 = ((locals.var_alpha0_i_dn8 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_alpha0_i_dn9 = ((locals.var_alpha0_i_dn9 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_alpha0_i_dn10 = ((locals.var_alpha0_i_dn10 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_alpha0_i_dn11 = ((locals.var_alpha0_i_dn11 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_alpha0_i_dn12 = ((locals.var_alpha0_i_dn12 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_alpha0_i_dn13 = ((locals.var_alpha0_i_dn13 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_alpha0_i_dn14 = ((locals.var_alpha0_i_dn14 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));
        locals.var_alpha0_i_rv = 0.0;

        let assign4000_e5242: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign4000_e5242;
        locals.var_guard37_rv = 0.0;

        let (assign4010_e5252, assign4010_e5252_d_n0, assign4010_e5252_d_n2, assign4010_e5252_d_n3, assign4010_e5252_d_n4, assign4010_e5252_d_n5, assign4010_e5252_d_n6, assign4010_e5252_d_n7, assign4010_e5252_d_n8, assign4010_e5252_d_n9, assign4010_e5252_d_n10, assign4010_e5252_d_n11, assign4010_e5252_d_n12, assign4010_e5252_d_n13, assign4010_e5252_d_n14,) = {
    if (locals.var_guard37 != 0.0) {
        let assign4010_e5247: f64 = (1.0 + locals.var_t0);
        let assign4010_e5249: f64 = (assign4010_e5247 + locals.var_t1);
        let assign4010_e5250: f64 = (locals.var_alpha0r_i * assign4010_e5249);
        (assign4010_e5250, ((locals.var_alpha0r_i_dn0 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_alpha0r_i_dn2 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_alpha0r_i_dn3 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_alpha0r_i_dn4 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_alpha0r_i_dn5 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_alpha0r_i_dn6 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_alpha0r_i_dn7 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_alpha0r_i_dn8 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_alpha0r_i_dn9 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_alpha0r_i_dn10 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_alpha0r_i_dn11 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_alpha0r_i_dn12 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_alpha0r_i_dn13 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_alpha0r_i_dn14 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn14 + locals.var_t1_dn14))),)
    } else {
        (locals.var_alpha0r_i, locals.var_alpha0r_i_dn0, locals.var_alpha0r_i_dn2, locals.var_alpha0r_i_dn3, locals.var_alpha0r_i_dn4, locals.var_alpha0r_i_dn5, locals.var_alpha0r_i_dn6, locals.var_alpha0r_i_dn7, locals.var_alpha0r_i_dn8, locals.var_alpha0r_i_dn9, locals.var_alpha0r_i_dn10, locals.var_alpha0r_i_dn11, locals.var_alpha0r_i_dn12, locals.var_alpha0r_i_dn13, locals.var_alpha0r_i_dn14,)
    }
};
        locals.var_alpha0r_i = assign4010_e5252;
        locals.var_alpha0r_i_dn0 = assign4010_e5252_d_n0;
        locals.var_alpha0r_i_dn2 = assign4010_e5252_d_n2;
        locals.var_alpha0r_i_dn3 = assign4010_e5252_d_n3;
        locals.var_alpha0r_i_dn4 = assign4010_e5252_d_n4;
        locals.var_alpha0r_i_dn5 = assign4010_e5252_d_n5;
        locals.var_alpha0r_i_dn6 = assign4010_e5252_d_n6;
        locals.var_alpha0r_i_dn7 = assign4010_e5252_d_n7;
        locals.var_alpha0r_i_dn8 = assign4010_e5252_d_n8;
        locals.var_alpha0r_i_dn9 = assign4010_e5252_d_n9;
        locals.var_alpha0r_i_dn10 = assign4010_e5252_d_n10;
        locals.var_alpha0r_i_dn11 = assign4010_e5252_d_n11;
        locals.var_alpha0r_i_dn12 = assign4010_e5252_d_n12;
        locals.var_alpha0r_i_dn13 = assign4010_e5252_d_n13;
        locals.var_alpha0r_i_dn14 = assign4010_e5252_d_n14;
        locals.var_alpha0r_i_rv = 0.0;

        let assign4020_e5256: f64 = (locals.var_inv_w).powf(p.p496);
        let assign4020_e5259: f64 = (locals.var_inv_wwide).powf(p.p496);
        let assign4020_e5260: f64 = (assign4020_e5256 - assign4020_e5259);
        let assign4020_e5262: f64 = (assign4020_e5260).max(0.0);
        let assign4020_e5263: f64 = (p.p495 * assign4020_e5262);
        locals.var_t1 = assign4020_e5263;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

    }
}
