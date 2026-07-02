#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_80(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22230_e31089, assign22230_e31089_d_n0, assign22230_e31089_d_n2, assign22230_e31089_d_n6, assign22230_e31089_d_n7, assign22230_e31089_d_n10, assign22230_e31089_d_n11, assign22230_e31089_d_n12, assign22230_e31089_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) {
        let assign22230_e31082: f64 = (locals.var_psisubsat__blk683 + locals.var_tmf1);
        let assign22230_e31083: f64 = (0.5 * assign22230_e31082);
        let assign22230_e31086: f64 = (1e-10 * 0.01);
        let assign22230_e31087: f64 = (assign22230_e31083 + assign22230_e31086);
        (assign22230_e31087, (0.5 * (locals.var_psisubsat__blk683_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_psisubsat__blk683_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_psisubsat__blk683_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_psisubsat__blk683_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_psisubsat__blk683_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_psisubsat__blk683_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_psisubsat__blk683_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_psisubsat__blk683_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_psisubsat__blk683, locals.var_psisubsat__blk683_dn0, locals.var_psisubsat__blk683_dn2, locals.var_psisubsat__blk683_dn6, locals.var_psisubsat__blk683_dn7, locals.var_psisubsat__blk683_dn10, locals.var_psisubsat__blk683_dn11, locals.var_psisubsat__blk683_dn12, locals.var_psisubsat__blk683_dn17,)
    }
};
        locals.var_psisubsat__blk683 = assign22230_e31089;
        locals.var_psisubsat__blk683_dn0 = assign22230_e31089_d_n0;
        locals.var_psisubsat__blk683_dn2 = assign22230_e31089_d_n2;
        locals.var_psisubsat__blk683_dn6 = assign22230_e31089_d_n6;
        locals.var_psisubsat__blk683_dn7 = assign22230_e31089_d_n7;
        locals.var_psisubsat__blk683_dn10 = assign22230_e31089_d_n10;
        locals.var_psisubsat__blk683_dn11 = assign22230_e31089_d_n11;
        locals.var_psisubsat__blk683_dn12 = assign22230_e31089_d_n12;
        locals.var_psisubsat__blk683_dn17 = assign22230_e31089_d_n17;
        locals.var_psisubsat__blk683_rv = 0.0;

        let assign22240_e31092: f64 = if locals.var_psisubsat__blk683 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard688 = assign22240_e31092;
        locals.var_guard688_rv = 0.0;

        let (assign22250_e31106, assign22250_e31106_d_n0, assign22250_e31106_d_n2, assign22250_e31106_d_n6, assign22250_e31106_d_n7, assign22250_e31106_d_n10, assign22250_e31106_d_n11, assign22250_e31106_d_n12, assign22250_e31106_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 != 0.0)) && (locals.var_guard688 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat__blk683, locals.var_psisubsat__blk683_dn0, locals.var_psisubsat__blk683_dn2, locals.var_psisubsat__blk683_dn6, locals.var_psisubsat__blk683_dn7, locals.var_psisubsat__blk683_dn10, locals.var_psisubsat__blk683_dn11, locals.var_psisubsat__blk683_dn12, locals.var_psisubsat__blk683_dn17,)
    }
};
        locals.var_psisubsat__blk683 = assign22250_e31106;
        locals.var_psisubsat__blk683_dn0 = assign22250_e31106_d_n0;
        locals.var_psisubsat__blk683_dn2 = assign22250_e31106_d_n2;
        locals.var_psisubsat__blk683_dn6 = assign22250_e31106_d_n6;
        locals.var_psisubsat__blk683_dn7 = assign22250_e31106_d_n7;
        locals.var_psisubsat__blk683_dn10 = assign22250_e31106_d_n10;
        locals.var_psisubsat__blk683_dn11 = assign22250_e31106_d_n11;
        locals.var_psisubsat__blk683_dn12 = assign22250_e31106_d_n12;
        locals.var_psisubsat__blk683_dn17 = assign22250_e31106_d_n17;
        locals.var_psisubsat__blk683_rv = 0.0;

        let (assign22260_e31121, assign22260_e31121_d_n0, assign22260_e31121_d_n2, assign22260_e31121_d_n6, assign22260_e31121_d_n7, assign22260_e31121_d_n10, assign22260_e31121_d_n11, assign22260_e31121_d_n12, assign22260_e31121_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) {
        let assign22260_e31119: f64 = (locals.var_vg2const * locals.var_vgpsub);
        (assign22260_e31119, ((locals.var_vg2const_dn0 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn0)), ((locals.var_vg2const_dn2 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn2)), ((locals.var_vg2const_dn6 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn6)), ((locals.var_vg2const_dn7 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn7)), ((locals.var_vg2const_dn10 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn10)), ((locals.var_vg2const_dn11 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn11)), ((locals.var_vg2const_dn12 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn12)), ((locals.var_vg2const_dn17 * locals.var_vgpsub) + (locals.var_vg2const * locals.var_vgpsub_dn17)),)
    } else {
        (locals.var_t1__blk672, locals.var_t1__blk672_dn0, locals.var_t1__blk672_dn2, locals.var_t1__blk672_dn6, locals.var_t1__blk672_dn7, locals.var_t1__blk672_dn10, locals.var_t1__blk672_dn11, locals.var_t1__blk672_dn12, locals.var_t1__blk672_dn17,)
    }
};
        locals.var_t1__blk672 = assign22260_e31121;
        locals.var_t1__blk672_dn0 = assign22260_e31121_d_n0;
        locals.var_t1__blk672_dn2 = assign22260_e31121_d_n2;
        locals.var_t1__blk672_dn6 = assign22260_e31121_d_n6;
        locals.var_t1__blk672_dn7 = assign22260_e31121_d_n7;
        locals.var_t1__blk672_dn10 = assign22260_e31121_d_n10;
        locals.var_t1__blk672_dn11 = assign22260_e31121_d_n11;
        locals.var_t1__blk672_dn12 = assign22260_e31121_d_n12;
        locals.var_t1__blk672_dn17 = assign22260_e31121_d_n17;
        locals.var_t1__blk672_rv = 0.0;

        let (assign22270_e31138, assign22270_e31138_d_n0, assign22270_e31138_d_n2, assign22270_e31138_d_n6, assign22270_e31138_d_n7, assign22270_e31138_d_n10, assign22270_e31138_d_n11, assign22270_e31138_d_n12, assign22270_e31138_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) {
        let assign22270_e31135: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign22270_e31136: f64 = (locals.var_qnsub_esi / assign22270_e31135);
        (assign22270_e31136, (((locals.var_qnsub_esi_dn0 * assign22270_e31135) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))) / (assign22270_e31135 * assign22270_e31135)), (((locals.var_qnsub_esi_dn2 * assign22270_e31135) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))) / (assign22270_e31135 * assign22270_e31135)), (((locals.var_qnsub_esi_dn6 * assign22270_e31135) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))) / (assign22270_e31135 * assign22270_e31135)), (((locals.var_qnsub_esi_dn7 * assign22270_e31135) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))) / (assign22270_e31135 * assign22270_e31135)), (((locals.var_qnsub_esi_dn10 * assign22270_e31135) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))) / (assign22270_e31135 * assign22270_e31135)), (((locals.var_qnsub_esi_dn11 * assign22270_e31135) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))) / (assign22270_e31135 * assign22270_e31135)), (((locals.var_qnsub_esi_dn12 * assign22270_e31135) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))) / (assign22270_e31135 * assign22270_e31135)), (((locals.var_qnsub_esi_dn17 * assign22270_e31135) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))) / (assign22270_e31135 * assign22270_e31135)),)
    } else {
        (locals.var_t3__blk674, locals.var_t3__blk674_dn0, locals.var_t3__blk674_dn2, locals.var_t3__blk674_dn6, locals.var_t3__blk674_dn7, locals.var_t3__blk674_dn10, locals.var_t3__blk674_dn11, locals.var_t3__blk674_dn12, locals.var_t3__blk674_dn17,)
    }
};
        locals.var_t3__blk674 = assign22270_e31138;
        locals.var_t3__blk674_dn0 = assign22270_e31138_d_n0;
        locals.var_t3__blk674_dn2 = assign22270_e31138_d_n2;
        locals.var_t3__blk674_dn6 = assign22270_e31138_d_n6;
        locals.var_t3__blk674_dn7 = assign22270_e31138_d_n7;
        locals.var_t3__blk674_dn10 = assign22270_e31138_d_n10;
        locals.var_t3__blk674_dn11 = assign22270_e31138_d_n11;
        locals.var_t3__blk674_dn12 = assign22270_e31138_d_n12;
        locals.var_t3__blk674_dn17 = assign22270_e31138_d_n17;
        locals.var_t3__blk674_rv = 0.0;

        let (assign22280_e31157, assign22280_e31157_d_n0, assign22280_e31157_d_n2, assign22280_e31157_d_n6, assign22280_e31157_d_n7, assign22280_e31157_d_n10, assign22280_e31157_d_n11, assign22280_e31157_d_n12, assign22280_e31157_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) {
        let assign22280_e31151: f64 = (2.0 / locals.var_qnsub_esi);
        let assign22280_e31154: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign22280_e31155: f64 = (assign22280_e31151 * assign22280_e31154);
        (assign22280_e31155, (((-((2.0 * locals.var_qnsub_esi_dn0) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22280_e31154) + (assign22280_e31151 * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))), (((-((2.0 * locals.var_qnsub_esi_dn2) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22280_e31154) + (assign22280_e31151 * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))), (((-((2.0 * locals.var_qnsub_esi_dn6) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22280_e31154) + (assign22280_e31151 * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))), (((-((2.0 * locals.var_qnsub_esi_dn7) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22280_e31154) + (assign22280_e31151 * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))), (((-((2.0 * locals.var_qnsub_esi_dn10) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22280_e31154) + (assign22280_e31151 * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))), (((-((2.0 * locals.var_qnsub_esi_dn11) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22280_e31154) + (assign22280_e31151 * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))), (((-((2.0 * locals.var_qnsub_esi_dn12) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22280_e31154) + (assign22280_e31151 * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))), (((-((2.0 * locals.var_qnsub_esi_dn17) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign22280_e31154) + (assign22280_e31151 * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))),)
    } else {
        (locals.var_t4__blk675, locals.var_t4__blk675_dn0, locals.var_t4__blk675_dn2, locals.var_t4__blk675_dn6, locals.var_t4__blk675_dn7, locals.var_t4__blk675_dn10, locals.var_t4__blk675_dn11, locals.var_t4__blk675_dn12, locals.var_t4__blk675_dn17,)
    }
};
        locals.var_t4__blk675 = assign22280_e31157;
        locals.var_t4__blk675_dn0 = assign22280_e31157_d_n0;
        locals.var_t4__blk675_dn2 = assign22280_e31157_d_n2;
        locals.var_t4__blk675_dn6 = assign22280_e31157_d_n6;
        locals.var_t4__blk675_dn7 = assign22280_e31157_d_n7;
        locals.var_t4__blk675_dn10 = assign22280_e31157_d_n10;
        locals.var_t4__blk675_dn11 = assign22280_e31157_d_n11;
        locals.var_t4__blk675_dn12 = assign22280_e31157_d_n12;
        locals.var_t4__blk675_dn17 = assign22280_e31157_d_n17;
        locals.var_t4__blk675_rv = 0.0;

        let (assign22290_e31176, assign22290_e31176_d_n0, assign22290_e31176_d_n2, assign22290_e31176_d_n6, assign22290_e31176_d_n7, assign22290_e31176_d_n10, assign22290_e31176_d_n11, assign22290_e31176_d_n12, assign22290_e31176_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) {
        let assign22290_e31170: f64 = (locals.var_t1__blk672 - locals.var_beta_inv);
        let assign22290_e31173: f64 = (locals.var_xvbs * locals.var_vbspz);
        let assign22290_e31174: f64 = (assign22290_e31170 - assign22290_e31173);
        (assign22290_e31174, (locals.var_t1__blk672_dn0 - (locals.var_xvbs * locals.var_vbspz_dn0)), (locals.var_t1__blk672_dn2 - (locals.var_xvbs * locals.var_vbspz_dn2)), (locals.var_t1__blk672_dn6 - (locals.var_xvbs * locals.var_vbspz_dn6)), (locals.var_t1__blk672_dn7 - (locals.var_xvbs * locals.var_vbspz_dn7)), ((locals.var_t1__blk672_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs * locals.var_vbspz_dn10)), (locals.var_t1__blk672_dn11 - (locals.var_xvbs * locals.var_vbspz_dn11)), (locals.var_t1__blk672_dn12 - (locals.var_xvbs * locals.var_vbspz_dn12)), (locals.var_t1__blk672_dn17 - (locals.var_xvbs * locals.var_vbspz_dn17)),)
    } else {
        (locals.var_t5__blk676, locals.var_t5__blk676_dn0, locals.var_t5__blk676_dn2, locals.var_t5__blk676_dn6, locals.var_t5__blk676_dn7, locals.var_t5__blk676_dn10, locals.var_t5__blk676_dn11, locals.var_t5__blk676_dn12, locals.var_t5__blk676_dn17,)
    }
};
        locals.var_t5__blk676 = assign22290_e31176;
        locals.var_t5__blk676_dn0 = assign22290_e31176_d_n0;
        locals.var_t5__blk676_dn2 = assign22290_e31176_d_n2;
        locals.var_t5__blk676_dn6 = assign22290_e31176_d_n6;
        locals.var_t5__blk676_dn7 = assign22290_e31176_d_n7;
        locals.var_t5__blk676_dn10 = assign22290_e31176_d_n10;
        locals.var_t5__blk676_dn11 = assign22290_e31176_d_n11;
        locals.var_t5__blk676_dn12 = assign22290_e31176_d_n12;
        locals.var_t5__blk676_dn17 = assign22290_e31176_d_n17;
        locals.var_t5__blk676_rv = 0.0;

        let (assign22300_e31193, assign22300_e31193_d_n0, assign22300_e31193_d_n2, assign22300_e31193_d_n6, assign22300_e31193_d_n7, assign22300_e31193_d_n10, assign22300_e31193_d_n11, assign22300_e31193_d_n12, assign22300_e31193_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) {
        let assign22300_e31189: f64 = (p.p49 * locals.var_qhs);
        let assign22300_e31191: f64 = (assign22300_e31189 / locals.var_c_soi);
        (assign22300_e31191, ((p.p49 * locals.var_qhs_dn0) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn2) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn6) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn7) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn10) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn11) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn12) / locals.var_c_soi), ((p.p49 * locals.var_qhs_dn17) / locals.var_c_soi),)
    } else {
        (locals.var_dvbssub, locals.var_dvbssub_dn0, locals.var_dvbssub_dn2, locals.var_dvbssub_dn6, locals.var_dvbssub_dn7, locals.var_dvbssub_dn10, locals.var_dvbssub_dn11, locals.var_dvbssub_dn12, locals.var_dvbssub_dn17,)
    }
};
        locals.var_dvbssub = assign22300_e31193;
        locals.var_dvbssub_dn0 = assign22300_e31193_d_n0;
        locals.var_dvbssub_dn2 = assign22300_e31193_d_n2;
        locals.var_dvbssub_dn6 = assign22300_e31193_d_n6;
        locals.var_dvbssub_dn7 = assign22300_e31193_d_n7;
        locals.var_dvbssub_dn10 = assign22300_e31193_d_n10;
        locals.var_dvbssub_dn11 = assign22300_e31193_d_n11;
        locals.var_dvbssub_dn12 = assign22300_e31193_d_n12;
        locals.var_dvbssub_dn17 = assign22300_e31193_d_n17;
        locals.var_dvbssub_rv = 0.0;

        let (assign22310_e31210, assign22310_e31210_d_n0, assign22310_e31210_d_n2, assign22310_e31210_d_n6, assign22310_e31210_d_n7, assign22310_e31210_d_n10, assign22310_e31210_d_n11, assign22310_e31210_d_n12, assign22310_e31210_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) {
        let assign22310_e31207: f64 = (locals.var_xvbs * locals.var_dvbssub);
        let assign22310_e31208: f64 = (locals.var_t5__blk676 - assign22310_e31207);
        (assign22310_e31208, (locals.var_t5__blk676_dn0 - (locals.var_xvbs * locals.var_dvbssub_dn0)), (locals.var_t5__blk676_dn2 - (locals.var_xvbs * locals.var_dvbssub_dn2)), (locals.var_t5__blk676_dn6 - (locals.var_xvbs * locals.var_dvbssub_dn6)), (locals.var_t5__blk676_dn7 - (locals.var_xvbs * locals.var_dvbssub_dn7)), (locals.var_t5__blk676_dn10 - (locals.var_xvbs * locals.var_dvbssub_dn10)), (locals.var_t5__blk676_dn11 - (locals.var_xvbs * locals.var_dvbssub_dn11)), (locals.var_t5__blk676_dn12 - (locals.var_xvbs * locals.var_dvbssub_dn12)), (locals.var_t5__blk676_dn17 - (locals.var_xvbs * locals.var_dvbssub_dn17)),)
    } else {
        (locals.var_t5__blk676, locals.var_t5__blk676_dn0, locals.var_t5__blk676_dn2, locals.var_t5__blk676_dn6, locals.var_t5__blk676_dn7, locals.var_t5__blk676_dn10, locals.var_t5__blk676_dn11, locals.var_t5__blk676_dn12, locals.var_t5__blk676_dn17,)
    }
};
        locals.var_t5__blk676 = assign22310_e31210;
        locals.var_t5__blk676_dn0 = assign22310_e31210_d_n0;
        locals.var_t5__blk676_dn2 = assign22310_e31210_d_n2;
        locals.var_t5__blk676_dn6 = assign22310_e31210_d_n6;
        locals.var_t5__blk676_dn7 = assign22310_e31210_d_n7;
        locals.var_t5__blk676_dn10 = assign22310_e31210_d_n10;
        locals.var_t5__blk676_dn11 = assign22310_e31210_d_n11;
        locals.var_t5__blk676_dn12 = assign22310_e31210_d_n12;
        locals.var_t5__blk676_dn17 = assign22310_e31210_d_n17;
        locals.var_t5__blk676_rv = 0.0;

        let (assign22320_e31227, assign22320_e31227_d_n0, assign22320_e31227_d_n2, assign22320_e31227_d_n6, assign22320_e31227_d_n7, assign22320_e31227_d_n10, assign22320_e31227_d_n11, assign22320_e31227_d_n12, assign22320_e31227_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) {
        let assign22320_e31224: f64 = (locals.var_t4__blk675 * locals.var_t5__blk676);
        let assign22320_e31225: f64 = (1.0 + assign22320_e31224);
        (assign22320_e31225, ((locals.var_t4__blk675_dn0 * locals.var_t5__blk676) + (locals.var_t4__blk675 * locals.var_t5__blk676_dn0)), ((locals.var_t4__blk675_dn2 * locals.var_t5__blk676) + (locals.var_t4__blk675 * locals.var_t5__blk676_dn2)), ((locals.var_t4__blk675_dn6 * locals.var_t5__blk676) + (locals.var_t4__blk675 * locals.var_t5__blk676_dn6)), ((locals.var_t4__blk675_dn7 * locals.var_t5__blk676) + (locals.var_t4__blk675 * locals.var_t5__blk676_dn7)), ((locals.var_t4__blk675_dn10 * locals.var_t5__blk676) + (locals.var_t4__blk675 * locals.var_t5__blk676_dn10)), ((locals.var_t4__blk675_dn11 * locals.var_t5__blk676) + (locals.var_t4__blk675 * locals.var_t5__blk676_dn11)), ((locals.var_t4__blk675_dn12 * locals.var_t5__blk676) + (locals.var_t4__blk675 * locals.var_t5__blk676_dn12)), ((locals.var_t4__blk675_dn17 * locals.var_t5__blk676) + (locals.var_t4__blk675 * locals.var_t5__blk676_dn17)),)
    } else {
        (locals.var_t6__blk677, locals.var_t6__blk677_dn0, locals.var_t6__blk677_dn2, locals.var_t6__blk677_dn6, locals.var_t6__blk677_dn7, locals.var_t6__blk677_dn10, locals.var_t6__blk677_dn11, locals.var_t6__blk677_dn12, locals.var_t6__blk677_dn17,)
    }
};
        locals.var_t6__blk677 = assign22320_e31227;
        locals.var_t6__blk677_dn0 = assign22320_e31227_d_n0;
        locals.var_t6__blk677_dn2 = assign22320_e31227_d_n2;
        locals.var_t6__blk677_dn6 = assign22320_e31227_d_n6;
        locals.var_t6__blk677_dn7 = assign22320_e31227_d_n7;
        locals.var_t6__blk677_dn10 = assign22320_e31227_d_n10;
        locals.var_t6__blk677_dn11 = assign22320_e31227_d_n11;
        locals.var_t6__blk677_dn12 = assign22320_e31227_d_n12;
        locals.var_t6__blk677_dn17 = assign22320_e31227_d_n17;
        locals.var_t6__blk677_rv = 0.0;

        let (assign22330_e31244, assign22330_e31244_d_n0, assign22330_e31244_d_n2, assign22330_e31244_d_n6, assign22330_e31244_d_n7, assign22330_e31244_d_n10, assign22330_e31244_d_n11, assign22330_e31244_d_n12, assign22330_e31244_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) {
        let assign22330_e31241: f64 = (1.0 + locals.var_t4__blk675);
        let assign22330_e31242: f64 = (2.0 * assign22330_e31241);
        (assign22330_e31242, (2.0 * locals.var_t4__blk675_dn0), (2.0 * locals.var_t4__blk675_dn2), (2.0 * locals.var_t4__blk675_dn6), (2.0 * locals.var_t4__blk675_dn7), (2.0 * locals.var_t4__blk675_dn10), (2.0 * locals.var_t4__blk675_dn11), (2.0 * locals.var_t4__blk675_dn12), (2.0 * locals.var_t4__blk675_dn17),)
    } else {
        (locals.var_t7__blk679, locals.var_t7__blk679_dn0, locals.var_t7__blk679_dn2, locals.var_t7__blk679_dn6, locals.var_t7__blk679_dn7, locals.var_t7__blk679_dn10, locals.var_t7__blk679_dn11, locals.var_t7__blk679_dn12, locals.var_t7__blk679_dn17,)
    }
};
        locals.var_t7__blk679 = assign22330_e31244;
        locals.var_t7__blk679_dn0 = assign22330_e31244_d_n0;
        locals.var_t7__blk679_dn2 = assign22330_e31244_d_n2;
        locals.var_t7__blk679_dn6 = assign22330_e31244_d_n6;
        locals.var_t7__blk679_dn7 = assign22330_e31244_d_n7;
        locals.var_t7__blk679_dn10 = assign22330_e31244_d_n10;
        locals.var_t7__blk679_dn11 = assign22330_e31244_d_n11;
        locals.var_t7__blk679_dn12 = assign22330_e31244_d_n12;
        locals.var_t7__blk679_dn17 = assign22330_e31244_d_n17;
        locals.var_t7__blk679_rv = 0.0;

        let assign22340_e31248: f64 = (1e-50 + locals.var_t7__blk679);
        let assign22340_e31253: f64 = if ((locals.var_t6__blk677 < assign22340_e31248) && (locals.var_t7__blk679 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard689 = assign22340_e31253;
        locals.var_guard689_rv = 0.0;

        let (assign22350_e31272, assign22350_e31272_d_n0, assign22350_e31272_d_n2, assign22350_e31272_d_n6, assign22350_e31272_d_n7, assign22350_e31272_d_n10, assign22350_e31272_d_n11, assign22350_e31272_d_n12, assign22350_e31272_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign22350_e31268: f64 = (1e-50 + locals.var_t7__blk679);
        let assign22350_e31270: f64 = (assign22350_e31268 - locals.var_t6__blk677);
        (assign22350_e31270, (locals.var_t7__blk679_dn0 - locals.var_t6__blk677_dn0), (locals.var_t7__blk679_dn2 - locals.var_t6__blk677_dn2), (locals.var_t7__blk679_dn6 - locals.var_t6__blk677_dn6), (locals.var_t7__blk679_dn7 - locals.var_t6__blk677_dn7), (locals.var_t7__blk679_dn10 - locals.var_t6__blk677_dn10), (locals.var_t7__blk679_dn11 - locals.var_t6__blk677_dn11), (locals.var_t7__blk679_dn12 - locals.var_t6__blk677_dn12), (locals.var_t7__blk679_dn17 - locals.var_t6__blk677_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22350_e31272;
        locals.var_tmf1_dn0 = assign22350_e31272_d_n0;
        locals.var_tmf1_dn2 = assign22350_e31272_d_n2;
        locals.var_tmf1_dn6 = assign22350_e31272_d_n6;
        locals.var_tmf1_dn7 = assign22350_e31272_d_n7;
        locals.var_tmf1_dn10 = assign22350_e31272_d_n10;
        locals.var_tmf1_dn11 = assign22350_e31272_d_n11;
        locals.var_tmf1_dn12 = assign22350_e31272_d_n12;
        locals.var_tmf1_dn17 = assign22350_e31272_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign22360_e31289, assign22360_e31289_d_n0, assign22360_e31289_d_n2, assign22360_e31289_d_n6, assign22360_e31289_d_n7, assign22360_e31289_d_n10, assign22360_e31289_d_n11, assign22360_e31289_d_n12, assign22360_e31289_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign22360_e31287: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign22360_e31287, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign22360_e31289;
        locals.var_x2_dn0 = assign22360_e31289_d_n0;
        locals.var_x2_dn2 = assign22360_e31289_d_n2;
        locals.var_x2_dn6 = assign22360_e31289_d_n6;
        locals.var_x2_dn7 = assign22360_e31289_d_n7;
        locals.var_x2_dn10 = assign22360_e31289_d_n10;
        locals.var_x2_dn11 = assign22360_e31289_d_n11;
        locals.var_x2_dn12 = assign22360_e31289_d_n12;
        locals.var_x2_dn17 = assign22360_e31289_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign22370_e31306, assign22370_e31306_d_n0, assign22370_e31306_d_n2, assign22370_e31306_d_n6, assign22370_e31306_d_n7, assign22370_e31306_d_n10, assign22370_e31306_d_n11, assign22370_e31306_d_n12, assign22370_e31306_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign22370_e31304: f64 = (locals.var_t7__blk679 * locals.var_t7__blk679);
        (assign22370_e31304, ((locals.var_t7__blk679_dn0 * locals.var_t7__blk679) + (locals.var_t7__blk679 * locals.var_t7__blk679_dn0)), ((locals.var_t7__blk679_dn2 * locals.var_t7__blk679) + (locals.var_t7__blk679 * locals.var_t7__blk679_dn2)), ((locals.var_t7__blk679_dn6 * locals.var_t7__blk679) + (locals.var_t7__blk679 * locals.var_t7__blk679_dn6)), ((locals.var_t7__blk679_dn7 * locals.var_t7__blk679) + (locals.var_t7__blk679 * locals.var_t7__blk679_dn7)), ((locals.var_t7__blk679_dn10 * locals.var_t7__blk679) + (locals.var_t7__blk679 * locals.var_t7__blk679_dn10)), ((locals.var_t7__blk679_dn11 * locals.var_t7__blk679) + (locals.var_t7__blk679 * locals.var_t7__blk679_dn11)), ((locals.var_t7__blk679_dn12 * locals.var_t7__blk679) + (locals.var_t7__blk679 * locals.var_t7__blk679_dn12)), ((locals.var_t7__blk679_dn17 * locals.var_t7__blk679) + (locals.var_t7__blk679 * locals.var_t7__blk679_dn17)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign22370_e31306;
        locals.var_xmax2_dn0 = assign22370_e31306_d_n0;
        locals.var_xmax2_dn2 = assign22370_e31306_d_n2;
        locals.var_xmax2_dn6 = assign22370_e31306_d_n6;
        locals.var_xmax2_dn7 = assign22370_e31306_d_n7;
        locals.var_xmax2_dn10 = assign22370_e31306_d_n10;
        locals.var_xmax2_dn11 = assign22370_e31306_d_n11;
        locals.var_xmax2_dn12 = assign22370_e31306_d_n12;
        locals.var_xmax2_dn17 = assign22370_e31306_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign22380_e31321, assign22380_e31321_d_n0, assign22380_e31321_d_n2, assign22380_e31321_d_n6, assign22380_e31321_d_n7, assign22380_e31321_d_n10, assign22380_e31321_d_n11, assign22380_e31321_d_n12, assign22380_e31321_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign22380_e31321;
        locals.var_xp_dn0 = assign22380_e31321_d_n0;
        locals.var_xp_dn2 = assign22380_e31321_d_n2;
        locals.var_xp_dn6 = assign22380_e31321_d_n6;
        locals.var_xp_dn7 = assign22380_e31321_d_n7;
        locals.var_xp_dn10 = assign22380_e31321_d_n10;
        locals.var_xp_dn11 = assign22380_e31321_d_n11;
        locals.var_xp_dn12 = assign22380_e31321_d_n12;
        locals.var_xp_dn17 = assign22380_e31321_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign22390_e31336, assign22390_e31336_d_n0, assign22390_e31336_d_n2, assign22390_e31336_d_n6, assign22390_e31336_d_n7, assign22390_e31336_d_n10, assign22390_e31336_d_n11, assign22390_e31336_d_n12, assign22390_e31336_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign22390_e31336;
        locals.var_xmp_dn0 = assign22390_e31336_d_n0;
        locals.var_xmp_dn2 = assign22390_e31336_d_n2;
        locals.var_xmp_dn6 = assign22390_e31336_d_n6;
        locals.var_xmp_dn7 = assign22390_e31336_d_n7;
        locals.var_xmp_dn10 = assign22390_e31336_d_n10;
        locals.var_xmp_dn11 = assign22390_e31336_d_n11;
        locals.var_xmp_dn12 = assign22390_e31336_d_n12;
        locals.var_xmp_dn17 = assign22390_e31336_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign22400_e31351,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign22400_e31351;
        locals.var_m0_rv = 0.0;

        let (assign22410_e31366,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign22410_e31366;
        locals.var_mm_rv = 0.0;

        let (assign22420_e31381, assign22420_e31381_d_n0, assign22420_e31381_d_n2, assign22420_e31381_d_n6, assign22420_e31381_d_n7, assign22420_e31381_d_n10, assign22420_e31381_d_n11, assign22420_e31381_d_n12, assign22420_e31381_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign22420_e31381;
        locals.var_arg_dn0 = assign22420_e31381_d_n0;
        locals.var_arg_dn2 = assign22420_e31381_d_n2;
        locals.var_arg_dn6 = assign22420_e31381_d_n6;
        locals.var_arg_dn7 = assign22420_e31381_d_n7;
        locals.var_arg_dn10 = assign22420_e31381_d_n10;
        locals.var_arg_dn11 = assign22420_e31381_d_n11;
        locals.var_arg_dn12 = assign22420_e31381_d_n12;
        locals.var_arg_dn17 = assign22420_e31381_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign22430_e31396, assign22430_e31396_d_n0, assign22430_e31396_d_n2, assign22430_e31396_d_n6, assign22430_e31396_d_n7, assign22430_e31396_d_n10, assign22430_e31396_d_n11, assign22430_e31396_d_n12, assign22430_e31396_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign22430_e31396;
        locals.var_dnm_dn0 = assign22430_e31396_d_n0;
        locals.var_dnm_dn2 = assign22430_e31396_d_n2;
        locals.var_dnm_dn6 = assign22430_e31396_d_n6;
        locals.var_dnm_dn7 = assign22430_e31396_d_n7;
        locals.var_dnm_dn10 = assign22430_e31396_d_n10;
        locals.var_dnm_dn11 = assign22430_e31396_d_n11;
        locals.var_dnm_dn12 = assign22430_e31396_d_n12;
        locals.var_dnm_dn17 = assign22430_e31396_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign22440_e31413, assign22440_e31413_d_n0, assign22440_e31413_d_n2, assign22440_e31413_d_n6, assign22440_e31413_d_n7, assign22440_e31413_d_n10, assign22440_e31413_d_n11, assign22440_e31413_d_n12, assign22440_e31413_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign22440_e31411: f64 = (locals.var_xp * locals.var_x2);
        (assign22440_e31411, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign22440_e31413;
        locals.var_xp_dn0 = assign22440_e31413_d_n0;
        locals.var_xp_dn2 = assign22440_e31413_d_n2;
        locals.var_xp_dn6 = assign22440_e31413_d_n6;
        locals.var_xp_dn7 = assign22440_e31413_d_n7;
        locals.var_xp_dn10 = assign22440_e31413_d_n10;
        locals.var_xp_dn11 = assign22440_e31413_d_n11;
        locals.var_xp_dn12 = assign22440_e31413_d_n12;
        locals.var_xp_dn17 = assign22440_e31413_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign22450_e31430, assign22450_e31430_d_n0, assign22450_e31430_d_n2, assign22450_e31430_d_n6, assign22450_e31430_d_n7, assign22450_e31430_d_n10, assign22450_e31430_d_n11, assign22450_e31430_d_n12, assign22450_e31430_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign22450_e31428: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign22450_e31428, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign22450_e31430;
        locals.var_xmp_dn0 = assign22450_e31430_d_n0;
        locals.var_xmp_dn2 = assign22450_e31430_d_n2;
        locals.var_xmp_dn6 = assign22450_e31430_d_n6;
        locals.var_xmp_dn7 = assign22450_e31430_d_n7;
        locals.var_xmp_dn10 = assign22450_e31430_d_n10;
        locals.var_xmp_dn11 = assign22450_e31430_d_n11;
        locals.var_xmp_dn12 = assign22450_e31430_d_n12;
        locals.var_xmp_dn17 = assign22450_e31430_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign22460_e31447, assign22460_e31447_d_n0, assign22460_e31447_d_n2, assign22460_e31447_d_n6, assign22460_e31447_d_n7, assign22460_e31447_d_n10, assign22460_e31447_d_n11, assign22460_e31447_d_n12, assign22460_e31447_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign22460_e31445: f64 = (locals.var_xp * locals.var_x2);
        (assign22460_e31445, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign22460_e31447;
        locals.var_xp_dn0 = assign22460_e31447_d_n0;
        locals.var_xp_dn2 = assign22460_e31447_d_n2;
        locals.var_xp_dn6 = assign22460_e31447_d_n6;
        locals.var_xp_dn7 = assign22460_e31447_d_n7;
        locals.var_xp_dn10 = assign22460_e31447_d_n10;
        locals.var_xp_dn11 = assign22460_e31447_d_n11;
        locals.var_xp_dn12 = assign22460_e31447_d_n12;
        locals.var_xp_dn17 = assign22460_e31447_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign22470_e31464, assign22470_e31464_d_n0, assign22470_e31464_d_n2, assign22470_e31464_d_n6, assign22470_e31464_d_n7, assign22470_e31464_d_n10, assign22470_e31464_d_n11, assign22470_e31464_d_n12, assign22470_e31464_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign22470_e31462: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign22470_e31462, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign22470_e31464;
        locals.var_xmp_dn0 = assign22470_e31464_d_n0;
        locals.var_xmp_dn2 = assign22470_e31464_d_n2;
        locals.var_xmp_dn6 = assign22470_e31464_d_n6;
        locals.var_xmp_dn7 = assign22470_e31464_d_n7;
        locals.var_xmp_dn10 = assign22470_e31464_d_n10;
        locals.var_xmp_dn11 = assign22470_e31464_d_n11;
        locals.var_xmp_dn12 = assign22470_e31464_d_n12;
        locals.var_xmp_dn17 = assign22470_e31464_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign22480_e31481, assign22480_e31481_d_n0, assign22480_e31481_d_n2, assign22480_e31481_d_n6, assign22480_e31481_d_n7, assign22480_e31481_d_n10, assign22480_e31481_d_n11, assign22480_e31481_d_n12, assign22480_e31481_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign22480_e31479: f64 = (locals.var_xp * locals.var_x2);
        (assign22480_e31479, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign22480_e31481;
        locals.var_xp_dn0 = assign22480_e31481_d_n0;
        locals.var_xp_dn2 = assign22480_e31481_d_n2;
        locals.var_xp_dn6 = assign22480_e31481_d_n6;
        locals.var_xp_dn7 = assign22480_e31481_d_n7;
        locals.var_xp_dn10 = assign22480_e31481_d_n10;
        locals.var_xp_dn11 = assign22480_e31481_d_n11;
        locals.var_xp_dn12 = assign22480_e31481_d_n12;
        locals.var_xp_dn17 = assign22480_e31481_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign22490_e31498, assign22490_e31498_d_n0, assign22490_e31498_d_n2, assign22490_e31498_d_n6, assign22490_e31498_d_n7, assign22490_e31498_d_n10, assign22490_e31498_d_n11, assign22490_e31498_d_n12, assign22490_e31498_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign22490_e31496: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign22490_e31496, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign22490_e31498;
        locals.var_xmp_dn0 = assign22490_e31498_d_n0;
        locals.var_xmp_dn2 = assign22490_e31498_d_n2;
        locals.var_xmp_dn6 = assign22490_e31498_d_n6;
        locals.var_xmp_dn7 = assign22490_e31498_d_n7;
        locals.var_xmp_dn10 = assign22490_e31498_d_n10;
        locals.var_xmp_dn11 = assign22490_e31498_d_n11;
        locals.var_xmp_dn12 = assign22490_e31498_d_n12;
        locals.var_xmp_dn17 = assign22490_e31498_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign22500_e31515, assign22500_e31515_d_n0, assign22500_e31515_d_n2, assign22500_e31515_d_n6, assign22500_e31515_d_n7, assign22500_e31515_d_n10, assign22500_e31515_d_n11, assign22500_e31515_d_n12, assign22500_e31515_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign22500_e31513: f64 = (locals.var_xp * locals.var_x2);
        (assign22500_e31513, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign22500_e31515;
        locals.var_xp_dn0 = assign22500_e31515_d_n0;
        locals.var_xp_dn2 = assign22500_e31515_d_n2;
        locals.var_xp_dn6 = assign22500_e31515_d_n6;
        locals.var_xp_dn7 = assign22500_e31515_d_n7;
        locals.var_xp_dn10 = assign22500_e31515_d_n10;
        locals.var_xp_dn11 = assign22500_e31515_d_n11;
        locals.var_xp_dn12 = assign22500_e31515_d_n12;
        locals.var_xp_dn17 = assign22500_e31515_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign22510_e31532, assign22510_e31532_d_n0, assign22510_e31532_d_n2, assign22510_e31532_d_n6, assign22510_e31532_d_n7, assign22510_e31532_d_n10, assign22510_e31532_d_n11, assign22510_e31532_d_n12, assign22510_e31532_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign22510_e31530: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign22510_e31530, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign22510_e31532;
        locals.var_xmp_dn0 = assign22510_e31532_d_n0;
        locals.var_xmp_dn2 = assign22510_e31532_d_n2;
        locals.var_xmp_dn6 = assign22510_e31532_d_n6;
        locals.var_xmp_dn7 = assign22510_e31532_d_n7;
        locals.var_xmp_dn10 = assign22510_e31532_d_n10;
        locals.var_xmp_dn11 = assign22510_e31532_d_n11;
        locals.var_xmp_dn12 = assign22510_e31532_d_n12;
        locals.var_xmp_dn17 = assign22510_e31532_d_n17;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_81(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22520_e31549, assign22520_e31549_d_n0, assign22520_e31549_d_n2, assign22520_e31549_d_n6, assign22520_e31549_d_n7, assign22520_e31549_d_n10, assign22520_e31549_d_n11, assign22520_e31549_d_n12, assign22520_e31549_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign22520_e31547: f64 = (locals.var_xp + locals.var_xmp);
        (assign22520_e31547, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign22520_e31549;
        locals.var_arg_dn0 = assign22520_e31549_d_n0;
        locals.var_arg_dn2 = assign22520_e31549_d_n2;
        locals.var_arg_dn6 = assign22520_e31549_d_n6;
        locals.var_arg_dn7 = assign22520_e31549_d_n7;
        locals.var_arg_dn10 = assign22520_e31549_d_n10;
        locals.var_arg_dn11 = assign22520_e31549_d_n11;
        locals.var_arg_dn12 = assign22520_e31549_d_n12;
        locals.var_arg_dn17 = assign22520_e31549_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign22530_e31564, assign22530_e31564_d_n0, assign22530_e31564_d_n2, assign22530_e31564_d_n6, assign22530_e31564_d_n7, assign22530_e31564_d_n10, assign22530_e31564_d_n11, assign22530_e31564_d_n12, assign22530_e31564_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign22530_e31564;
        locals.var_dnm_dn0 = assign22530_e31564_d_n0;
        locals.var_dnm_dn2 = assign22530_e31564_d_n2;
        locals.var_dnm_dn6 = assign22530_e31564_d_n6;
        locals.var_dnm_dn7 = assign22530_e31564_d_n7;
        locals.var_dnm_dn10 = assign22530_e31564_d_n10;
        locals.var_dnm_dn11 = assign22530_e31564_d_n11;
        locals.var_dnm_dn12 = assign22530_e31564_d_n12;
        locals.var_dnm_dn17 = assign22530_e31564_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign22540_e31579: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard690 = assign22540_e31579;
        locals.var_guard690_rv = 0.0;

        let assign22550_e31582: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard691 = assign22550_e31582;
        locals.var_guard691_rv = 0.0;

        let (assign22560_e31601,) = {
    if (((((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign22560_e31601;
        locals.var_mm_rv = 0.0;

        let assign22570_e31604: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard692 = assign22570_e31604;
        locals.var_guard692_rv = 0.0;

        let (assign22580_e31626,) = {
    if ((((((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 == 0.0)) && (locals.var_guard692 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign22580_e31626;
        locals.var_mm_rv = 0.0;

        let assign22590_e31629: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard693 = assign22590_e31629;
        locals.var_guard693_rv = 0.0;

        let (assign22600_e31654,) = {
    if (((((((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 == 0.0)) && (locals.var_guard692 == 0.0)) && (locals.var_guard693 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign22600_e31654;
        locals.var_mm_rv = 0.0;

        let assign22610_e31657: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard694 = assign22610_e31657;
        locals.var_guard694_rv = 0.0;

        let (assign22620_e31685,) = {
    if ((((((((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_guard691 == 0.0)) && (locals.var_guard692 == 0.0)) && (locals.var_guard693 == 0.0)) && (locals.var_guard694 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign22620_e31685;
        locals.var_mm_rv = 0.0;

        let (assign22630_e31702,) = {
    if ((((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign22630_e31702;
        locals.var_m0_rv = 0.0;

        let mut assign22640_loop_guard: usize = 0;
        while {
            let assign22640_cond_e31720: f64 = if (((((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign22640_cond_e31720 != 0.0
        } {
            assign22640_loop_guard += 1;
            assert!(assign22640_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign22640_body0_e31738, assign22640_body0_e31738_d_n0, assign22640_body0_e31738_d_n2, assign22640_body0_e31738_d_n6, assign22640_body0_e31738_d_n7, assign22640_body0_e31738_d_n10, assign22640_body0_e31738_d_n11, assign22640_body0_e31738_d_n12, assign22640_body0_e31738_d_n17,) = {
    if ((((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign22640_body0_e31736: f64 = (locals.var_dnm).sqrt();
        (assign22640_body0_e31736, (locals.var_dnm_dn0 / (2.0 * assign22640_body0_e31736)), (locals.var_dnm_dn2 / (2.0 * assign22640_body0_e31736)), (locals.var_dnm_dn6 / (2.0 * assign22640_body0_e31736)), (locals.var_dnm_dn7 / (2.0 * assign22640_body0_e31736)), (locals.var_dnm_dn10 / (2.0 * assign22640_body0_e31736)), (locals.var_dnm_dn11 / (2.0 * assign22640_body0_e31736)), (locals.var_dnm_dn12 / (2.0 * assign22640_body0_e31736)), (locals.var_dnm_dn17 / (2.0 * assign22640_body0_e31736)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign22640_body0_e31738;
            locals.var_dnm_dn0 = assign22640_body0_e31738_d_n0;
            locals.var_dnm_dn2 = assign22640_body0_e31738_d_n2;
            locals.var_dnm_dn6 = assign22640_body0_e31738_d_n6;
            locals.var_dnm_dn7 = assign22640_body0_e31738_d_n7;
            locals.var_dnm_dn10 = assign22640_body0_e31738_d_n10;
            locals.var_dnm_dn11 = assign22640_body0_e31738_d_n11;
            locals.var_dnm_dn12 = assign22640_body0_e31738_d_n12;
            locals.var_dnm_dn17 = assign22640_body0_e31738_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign22640_body1_e31757,) = {
    if ((((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign22640_body1_e31755: f64 = (locals.var_m0 + 1.0);
        (assign22640_body1_e31755,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign22640_body1_e31757;
            locals.var_m0_rv = 0.0;
        }

        let (assign22650_e31781, assign22650_e31781_d_n0, assign22650_e31781_d_n2, assign22650_e31781_d_n6, assign22650_e31781_d_n7, assign22650_e31781_d_n10, assign22650_e31781_d_n11, assign22650_e31781_d_n12, assign22650_e31781_d_n17,) = {
    if ((((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) && (locals.var_guard690 == 0.0)) {
        let assign22650_e31777: f64 = (2.0 * 4.0);
        let assign22650_e31778: f64 = (1.0 / assign22650_e31777);
        let assign22650_e31779: f64 = (locals.var_dnm).powf(assign22650_e31778);
        (assign22650_e31779, if 0.0 == 0.0 && ((assign22650_e31778) as f64).is_finite() && ((assign22650_e31778) as f64).fract() == 0.0 { if assign22650_e31778 == 0.0 { 0.0 } else { (assign22650_e31778 * ((locals.var_dnm).powf(assign22650_e31778 - 1.0) * locals.var_dnm_dn0)) } } else { (assign22650_e31779 * (assign22650_e31778 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign22650_e31778) as f64).is_finite() && ((assign22650_e31778) as f64).fract() == 0.0 { if assign22650_e31778 == 0.0 { 0.0 } else { (assign22650_e31778 * ((locals.var_dnm).powf(assign22650_e31778 - 1.0) * locals.var_dnm_dn2)) } } else { (assign22650_e31779 * (assign22650_e31778 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign22650_e31778) as f64).is_finite() && ((assign22650_e31778) as f64).fract() == 0.0 { if assign22650_e31778 == 0.0 { 0.0 } else { (assign22650_e31778 * ((locals.var_dnm).powf(assign22650_e31778 - 1.0) * locals.var_dnm_dn6)) } } else { (assign22650_e31779 * (assign22650_e31778 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign22650_e31778) as f64).is_finite() && ((assign22650_e31778) as f64).fract() == 0.0 { if assign22650_e31778 == 0.0 { 0.0 } else { (assign22650_e31778 * ((locals.var_dnm).powf(assign22650_e31778 - 1.0) * locals.var_dnm_dn7)) } } else { (assign22650_e31779 * (assign22650_e31778 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign22650_e31778) as f64).is_finite() && ((assign22650_e31778) as f64).fract() == 0.0 { if assign22650_e31778 == 0.0 { 0.0 } else { (assign22650_e31778 * ((locals.var_dnm).powf(assign22650_e31778 - 1.0) * locals.var_dnm_dn10)) } } else { (assign22650_e31779 * (assign22650_e31778 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign22650_e31778) as f64).is_finite() && ((assign22650_e31778) as f64).fract() == 0.0 { if assign22650_e31778 == 0.0 { 0.0 } else { (assign22650_e31778 * ((locals.var_dnm).powf(assign22650_e31778 - 1.0) * locals.var_dnm_dn11)) } } else { (assign22650_e31779 * (assign22650_e31778 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign22650_e31778) as f64).is_finite() && ((assign22650_e31778) as f64).fract() == 0.0 { if assign22650_e31778 == 0.0 { 0.0 } else { (assign22650_e31778 * ((locals.var_dnm).powf(assign22650_e31778 - 1.0) * locals.var_dnm_dn12)) } } else { (assign22650_e31779 * (assign22650_e31778 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign22650_e31778) as f64).is_finite() && ((assign22650_e31778) as f64).fract() == 0.0 { if assign22650_e31778 == 0.0 { 0.0 } else { (assign22650_e31778 * ((locals.var_dnm).powf(assign22650_e31778 - 1.0) * locals.var_dnm_dn17)) } } else { (assign22650_e31779 * (assign22650_e31778 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign22650_e31781;
        locals.var_dnm_dn0 = assign22650_e31781_d_n0;
        locals.var_dnm_dn2 = assign22650_e31781_d_n2;
        locals.var_dnm_dn6 = assign22650_e31781_d_n6;
        locals.var_dnm_dn7 = assign22650_e31781_d_n7;
        locals.var_dnm_dn10 = assign22650_e31781_d_n10;
        locals.var_dnm_dn11 = assign22650_e31781_d_n11;
        locals.var_dnm_dn12 = assign22650_e31781_d_n12;
        locals.var_dnm_dn17 = assign22650_e31781_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign22660_e31798, assign22660_e31798_d_n0, assign22660_e31798_d_n2, assign22660_e31798_d_n6, assign22660_e31798_d_n7, assign22660_e31798_d_n10, assign22660_e31798_d_n11, assign22660_e31798_d_n12, assign22660_e31798_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign22660_e31796: f64 = (1.0 / locals.var_dnm);
        (assign22660_e31796, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign22660_e31798;
        locals.var_dnm_dn0 = assign22660_e31798_d_n0;
        locals.var_dnm_dn2 = assign22660_e31798_d_n2;
        locals.var_dnm_dn6 = assign22660_e31798_d_n6;
        locals.var_dnm_dn7 = assign22660_e31798_d_n7;
        locals.var_dnm_dn10 = assign22660_e31798_d_n10;
        locals.var_dnm_dn11 = assign22660_e31798_d_n11;
        locals.var_dnm_dn12 = assign22660_e31798_d_n12;
        locals.var_dnm_dn17 = assign22660_e31798_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign22670_e31817, assign22670_e31817_d_n0, assign22670_e31817_d_n2, assign22670_e31817_d_n6, assign22670_e31817_d_n7, assign22670_e31817_d_n10, assign22670_e31817_d_n11, assign22670_e31817_d_n12, assign22670_e31817_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign22670_e31813: f64 = (locals.var_tmf1 * locals.var_t7__blk679);
        let assign22670_e31815: f64 = (assign22670_e31813 * locals.var_dnm);
        (assign22670_e31815, ((((locals.var_tmf1_dn0 * locals.var_t7__blk679) + (locals.var_tmf1 * locals.var_t7__blk679_dn0)) * locals.var_dnm) + (assign22670_e31813 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t7__blk679) + (locals.var_tmf1 * locals.var_t7__blk679_dn2)) * locals.var_dnm) + (assign22670_e31813 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * locals.var_t7__blk679) + (locals.var_tmf1 * locals.var_t7__blk679_dn6)) * locals.var_dnm) + (assign22670_e31813 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t7__blk679) + (locals.var_tmf1 * locals.var_t7__blk679_dn7)) * locals.var_dnm) + (assign22670_e31813 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * locals.var_t7__blk679) + (locals.var_tmf1 * locals.var_t7__blk679_dn10)) * locals.var_dnm) + (assign22670_e31813 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t7__blk679) + (locals.var_tmf1 * locals.var_t7__blk679_dn11)) * locals.var_dnm) + (assign22670_e31813 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * locals.var_t7__blk679) + (locals.var_tmf1 * locals.var_t7__blk679_dn12)) * locals.var_dnm) + (assign22670_e31813 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * locals.var_t7__blk679) + (locals.var_tmf1 * locals.var_t7__blk679_dn17)) * locals.var_dnm) + (assign22670_e31813 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign22670_e31817;
        locals.var_tmf0_dn0 = assign22670_e31817_d_n0;
        locals.var_tmf0_dn2 = assign22670_e31817_d_n2;
        locals.var_tmf0_dn6 = assign22670_e31817_d_n6;
        locals.var_tmf0_dn7 = assign22670_e31817_d_n7;
        locals.var_tmf0_dn10 = assign22670_e31817_d_n10;
        locals.var_tmf0_dn11 = assign22670_e31817_d_n11;
        locals.var_tmf0_dn12 = assign22670_e31817_d_n12;
        locals.var_tmf0_dn17 = assign22670_e31817_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign22680_e31836, assign22680_e31836_d_n0, assign22680_e31836_d_n2, assign22680_e31836_d_n6, assign22680_e31836_d_n7, assign22680_e31836_d_n10, assign22680_e31836_d_n11, assign22680_e31836_d_n12, assign22680_e31836_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign22680_e31832: f64 = (1e-50 + locals.var_t7__blk679);
        let assign22680_e31834: f64 = (assign22680_e31832 - locals.var_tmf0);
        (assign22680_e31834, (locals.var_t7__blk679_dn0 - locals.var_tmf0_dn0), (locals.var_t7__blk679_dn2 - locals.var_tmf0_dn2), (locals.var_t7__blk679_dn6 - locals.var_tmf0_dn6), (locals.var_t7__blk679_dn7 - locals.var_tmf0_dn7), (locals.var_t7__blk679_dn10 - locals.var_tmf0_dn10), (locals.var_t7__blk679_dn11 - locals.var_tmf0_dn11), (locals.var_t7__blk679_dn12 - locals.var_tmf0_dn12), (locals.var_t7__blk679_dn17 - locals.var_tmf0_dn17),)
    } else {
        (locals.var_t6__blk677, locals.var_t6__blk677_dn0, locals.var_t6__blk677_dn2, locals.var_t6__blk677_dn6, locals.var_t6__blk677_dn7, locals.var_t6__blk677_dn10, locals.var_t6__blk677_dn11, locals.var_t6__blk677_dn12, locals.var_t6__blk677_dn17,)
    }
};
        locals.var_t6__blk677 = assign22680_e31836;
        locals.var_t6__blk677_dn0 = assign22680_e31836_d_n0;
        locals.var_t6__blk677_dn2 = assign22680_e31836_d_n2;
        locals.var_t6__blk677_dn6 = assign22680_e31836_d_n6;
        locals.var_t6__blk677_dn7 = assign22680_e31836_d_n7;
        locals.var_t6__blk677_dn10 = assign22680_e31836_d_n10;
        locals.var_t6__blk677_dn11 = assign22680_e31836_d_n11;
        locals.var_t6__blk677_dn12 = assign22680_e31836_d_n12;
        locals.var_t6__blk677_dn17 = assign22680_e31836_d_n17;
        locals.var_t6__blk677_rv = 0.0;

        let (assign22690_e31852, assign22690_e31852_d_n0, assign22690_e31852_d_n2, assign22690_e31852_d_n6, assign22690_e31852_d_n7, assign22690_e31852_d_n10, assign22690_e31852_d_n11, assign22690_e31852_d_n12, assign22690_e31852_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard689 == 0.0)) {
        (locals.var_t6__blk677, locals.var_t6__blk677_dn0, locals.var_t6__blk677_dn2, locals.var_t6__blk677_dn6, locals.var_t6__blk677_dn7, locals.var_t6__blk677_dn10, locals.var_t6__blk677_dn11, locals.var_t6__blk677_dn12, locals.var_t6__blk677_dn17,)
    } else {
        (locals.var_t6__blk677, locals.var_t6__blk677_dn0, locals.var_t6__blk677_dn2, locals.var_t6__blk677_dn6, locals.var_t6__blk677_dn7, locals.var_t6__blk677_dn10, locals.var_t6__blk677_dn11, locals.var_t6__blk677_dn12, locals.var_t6__blk677_dn17,)
    }
};
        locals.var_t6__blk677 = assign22690_e31852;
        locals.var_t6__blk677_dn0 = assign22690_e31852_d_n0;
        locals.var_t6__blk677_dn2 = assign22690_e31852_d_n2;
        locals.var_t6__blk677_dn6 = assign22690_e31852_d_n6;
        locals.var_t6__blk677_dn7 = assign22690_e31852_d_n7;
        locals.var_t6__blk677_dn10 = assign22690_e31852_d_n10;
        locals.var_t6__blk677_dn11 = assign22690_e31852_d_n11;
        locals.var_t6__blk677_dn12 = assign22690_e31852_d_n12;
        locals.var_t6__blk677_dn17 = assign22690_e31852_d_n17;
        locals.var_t6__blk677_rv = 0.0;

        let (assign22700_e31871, assign22700_e31871_d_n0, assign22700_e31871_d_n2, assign22700_e31871_d_n6, assign22700_e31871_d_n7, assign22700_e31871_d_n10, assign22700_e31871_d_n11, assign22700_e31871_d_n12, assign22700_e31871_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) {
        let (assign22700_e31869, assign22700_e31869_d_n0, assign22700_e31869_d_n2, assign22700_e31869_d_n6, assign22700_e31869_d_n7, assign22700_e31869_d_n10, assign22700_e31869_d_n11, assign22700_e31869_d_n12, assign22700_e31869_d_n17,) = {
            if (locals.var_t6__blk677 <= 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign22700_e31868: f64 = (locals.var_t6__blk677).sqrt();
                (assign22700_e31868, (locals.var_t6__blk677_dn0 / (2.0 * assign22700_e31868)), (locals.var_t6__blk677_dn2 / (2.0 * assign22700_e31868)), (locals.var_t6__blk677_dn6 / (2.0 * assign22700_e31868)), (locals.var_t6__blk677_dn7 / (2.0 * assign22700_e31868)), (locals.var_t6__blk677_dn10 / (2.0 * assign22700_e31868)), (locals.var_t6__blk677_dn11 / (2.0 * assign22700_e31868)), (locals.var_t6__blk677_dn12 / (2.0 * assign22700_e31868)), (locals.var_t6__blk677_dn17 / (2.0 * assign22700_e31868)),)
            }
        };
        (assign22700_e31869, assign22700_e31869_d_n0, assign22700_e31869_d_n2, assign22700_e31869_d_n6, assign22700_e31869_d_n7, assign22700_e31869_d_n10, assign22700_e31869_d_n11, assign22700_e31869_d_n12, assign22700_e31869_d_n17,)
    } else {
        (locals.var_t6__blk677, locals.var_t6__blk677_dn0, locals.var_t6__blk677_dn2, locals.var_t6__blk677_dn6, locals.var_t6__blk677_dn7, locals.var_t6__blk677_dn10, locals.var_t6__blk677_dn11, locals.var_t6__blk677_dn12, locals.var_t6__blk677_dn17,)
    }
};
        locals.var_t6__blk677 = assign22700_e31871;
        locals.var_t6__blk677_dn0 = assign22700_e31871_d_n0;
        locals.var_t6__blk677_dn2 = assign22700_e31871_d_n2;
        locals.var_t6__blk677_dn6 = assign22700_e31871_d_n6;
        locals.var_t6__blk677_dn7 = assign22700_e31871_d_n7;
        locals.var_t6__blk677_dn10 = assign22700_e31871_d_n10;
        locals.var_t6__blk677_dn11 = assign22700_e31871_d_n11;
        locals.var_t6__blk677_dn12 = assign22700_e31871_d_n12;
        locals.var_t6__blk677_dn17 = assign22700_e31871_d_n17;
        locals.var_t6__blk677_rv = 0.0;

        let (assign22710_e31890, assign22710_e31890_d_n0, assign22710_e31890_d_n2, assign22710_e31890_d_n6, assign22710_e31890_d_n7, assign22710_e31890_d_n10, assign22710_e31890_d_n11, assign22710_e31890_d_n12, assign22710_e31890_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) {
        let assign22710_e31886: f64 = (1.0 - locals.var_t6__blk677);
        let assign22710_e31887: f64 = (locals.var_t3__blk674 * assign22710_e31886);
        let assign22710_e31888: f64 = (locals.var_t1__blk672 + assign22710_e31887);
        (assign22710_e31888, (locals.var_t1__blk672_dn0 + ((locals.var_t3__blk674_dn0 * assign22710_e31886) + (locals.var_t3__blk674 * (-locals.var_t6__blk677_dn0)))), (locals.var_t1__blk672_dn2 + ((locals.var_t3__blk674_dn2 * assign22710_e31886) + (locals.var_t3__blk674 * (-locals.var_t6__blk677_dn2)))), (locals.var_t1__blk672_dn6 + ((locals.var_t3__blk674_dn6 * assign22710_e31886) + (locals.var_t3__blk674 * (-locals.var_t6__blk677_dn6)))), (locals.var_t1__blk672_dn7 + ((locals.var_t3__blk674_dn7 * assign22710_e31886) + (locals.var_t3__blk674 * (-locals.var_t6__blk677_dn7)))), (locals.var_t1__blk672_dn10 + ((locals.var_t3__blk674_dn10 * assign22710_e31886) + (locals.var_t3__blk674 * (-locals.var_t6__blk677_dn10)))), (locals.var_t1__blk672_dn11 + ((locals.var_t3__blk674_dn11 * assign22710_e31886) + (locals.var_t3__blk674 * (-locals.var_t6__blk677_dn11)))), (locals.var_t1__blk672_dn12 + ((locals.var_t3__blk674_dn12 * assign22710_e31886) + (locals.var_t3__blk674 * (-locals.var_t6__blk677_dn12)))), (locals.var_t1__blk672_dn17 + ((locals.var_t3__blk674_dn17 * assign22710_e31886) + (locals.var_t3__blk674 * (-locals.var_t6__blk677_dn17)))),)
    } else {
        (locals.var_psislsat__blk682, locals.var_psislsat__blk682_dn0, locals.var_psislsat__blk682_dn2, locals.var_psislsat__blk682_dn6, locals.var_psislsat__blk682_dn7, locals.var_psislsat__blk682_dn10, locals.var_psislsat__blk682_dn11, locals.var_psislsat__blk682_dn12, locals.var_psislsat__blk682_dn17,)
    }
};
        locals.var_psislsat__blk682 = assign22710_e31890;
        locals.var_psislsat__blk682_dn0 = assign22710_e31890_d_n0;
        locals.var_psislsat__blk682_dn2 = assign22710_e31890_d_n2;
        locals.var_psislsat__blk682_dn6 = assign22710_e31890_d_n6;
        locals.var_psislsat__blk682_dn7 = assign22710_e31890_d_n7;
        locals.var_psislsat__blk682_dn10 = assign22710_e31890_d_n10;
        locals.var_psislsat__blk682_dn11 = assign22710_e31890_d_n11;
        locals.var_psislsat__blk682_dn12 = assign22710_e31890_d_n12;
        locals.var_psislsat__blk682_dn17 = assign22710_e31890_d_n17;
        locals.var_psislsat__blk682_rv = 0.0;

        let (assign22720_e31907, assign22720_e31907_d_n0, assign22720_e31907_d_n2, assign22720_e31907_d_n6, assign22720_e31907_d_n7, assign22720_e31907_d_n10, assign22720_e31907_d_n11, assign22720_e31907_d_n12, assign22720_e31907_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) {
        let assign22720_e31904: f64 = (locals.var_xgate + locals.var_lgle);
        let assign22720_e31905: f64 = (locals.var_lgle / assign22720_e31904);
        (assign22720_e31905, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk673, locals.var_t2__blk673_dn0, locals.var_t2__blk673_dn2, locals.var_t2__blk673_dn6, locals.var_t2__blk673_dn7, locals.var_t2__blk673_dn10, locals.var_t2__blk673_dn11, locals.var_t2__blk673_dn12, locals.var_t2__blk673_dn17,)
    }
};
        locals.var_t2__blk673 = assign22720_e31907;
        locals.var_t2__blk673_dn0 = assign22720_e31907_d_n0;
        locals.var_t2__blk673_dn2 = assign22720_e31907_d_n2;
        locals.var_t2__blk673_dn6 = assign22720_e31907_d_n6;
        locals.var_t2__blk673_dn7 = assign22720_e31907_d_n7;
        locals.var_t2__blk673_dn10 = assign22720_e31907_d_n10;
        locals.var_t2__blk673_dn11 = assign22720_e31907_d_n11;
        locals.var_t2__blk673_dn12 = assign22720_e31907_d_n12;
        locals.var_t2__blk673_dn17 = assign22720_e31907_d_n17;
        locals.var_t2__blk673_rv = 0.0;

        let (assign22730_e31928, assign22730_e31928_d_n0, assign22730_e31928_d_n2, assign22730_e31928_d_n6, assign22730_e31928_d_n7, assign22730_e31928_d_n10, assign22730_e31928_d_n11, assign22730_e31928_d_n12, assign22730_e31928_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) {
        let assign22730_e31920: f64 = (p.p122 * locals.var_vdsz);
        let assign22730_e31922: f64 = (assign22730_e31920 + locals.var_ps0z);
        let assign22730_e31925: f64 = (locals.var_t2__blk673 * locals.var_psislsat__blk682);
        let assign22730_e31926: f64 = (assign22730_e31922 - assign22730_e31925);
        (assign22730_e31926, (((p.p122 * locals.var_vdsz_dn0) + locals.var_ps0z_dn0) - ((locals.var_t2__blk673_dn0 * locals.var_psislsat__blk682) + (locals.var_t2__blk673 * locals.var_psislsat__blk682_dn0))), (((p.p122 * locals.var_vdsz_dn2) + locals.var_ps0z_dn2) - ((locals.var_t2__blk673_dn2 * locals.var_psislsat__blk682) + (locals.var_t2__blk673 * locals.var_psislsat__blk682_dn2))), (((p.p122 * locals.var_vdsz_dn6) + locals.var_ps0z_dn6) - ((locals.var_t2__blk673_dn6 * locals.var_psislsat__blk682) + (locals.var_t2__blk673 * locals.var_psislsat__blk682_dn6))), (((p.p122 * locals.var_vdsz_dn7) + locals.var_ps0z_dn7) - ((locals.var_t2__blk673_dn7 * locals.var_psislsat__blk682) + (locals.var_t2__blk673 * locals.var_psislsat__blk682_dn7))), (((p.p122 * locals.var_vdsz_dn10) + locals.var_ps0z_dn10) - ((locals.var_t2__blk673_dn10 * locals.var_psislsat__blk682) + (locals.var_t2__blk673 * locals.var_psislsat__blk682_dn10))), (((p.p122 * locals.var_vdsz_dn11) + locals.var_ps0z_dn11) - ((locals.var_t2__blk673_dn11 * locals.var_psislsat__blk682) + (locals.var_t2__blk673 * locals.var_psislsat__blk682_dn11))), (((p.p122 * locals.var_vdsz_dn12) + locals.var_ps0z_dn12) - ((locals.var_t2__blk673_dn12 * locals.var_psislsat__blk682) + (locals.var_t2__blk673 * locals.var_psislsat__blk682_dn12))), (((p.p122 * locals.var_vdsz_dn17) + locals.var_ps0z_dn17) - ((locals.var_t2__blk673_dn17 * locals.var_psislsat__blk682) + (locals.var_t2__blk673 * locals.var_psislsat__blk682_dn17))),)
    } else {
        (locals.var_psisubsat__blk683, locals.var_psisubsat__blk683_dn0, locals.var_psisubsat__blk683_dn2, locals.var_psisubsat__blk683_dn6, locals.var_psisubsat__blk683_dn7, locals.var_psisubsat__blk683_dn10, locals.var_psisubsat__blk683_dn11, locals.var_psisubsat__blk683_dn12, locals.var_psisubsat__blk683_dn17,)
    }
};
        locals.var_psisubsat__blk683 = assign22730_e31928;
        locals.var_psisubsat__blk683_dn0 = assign22730_e31928_d_n0;
        locals.var_psisubsat__blk683_dn2 = assign22730_e31928_d_n2;
        locals.var_psisubsat__blk683_dn6 = assign22730_e31928_d_n6;
        locals.var_psisubsat__blk683_dn7 = assign22730_e31928_d_n7;
        locals.var_psisubsat__blk683_dn10 = assign22730_e31928_d_n10;
        locals.var_psisubsat__blk683_dn11 = assign22730_e31928_d_n11;
        locals.var_psisubsat__blk683_dn12 = assign22730_e31928_d_n12;
        locals.var_psisubsat__blk683_dn17 = assign22730_e31928_d_n17;
        locals.var_psisubsat__blk683_rv = 0.0;

        let (assign22740_e31950, assign22740_e31950_d_n0, assign22740_e31950_d_n2, assign22740_e31950_d_n6, assign22740_e31950_d_n7, assign22740_e31950_d_n10, assign22740_e31950_d_n11, assign22740_e31950_d_n12, assign22740_e31950_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) {
        let assign22740_e31941: f64 = (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683);
        let assign22740_e31944: f64 = (4.0 * 0.001);
        let assign22740_e31946: f64 = (assign22740_e31944 * 0.001);
        let assign22740_e31947: f64 = (assign22740_e31941 + assign22740_e31946);
        let assign22740_e31948: f64 = (assign22740_e31947).sqrt();
        (assign22740_e31948, (((locals.var_psisubsat__blk683_dn0 * locals.var_psisubsat__blk683) + (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683_dn0)) / (2.0 * assign22740_e31948)), (((locals.var_psisubsat__blk683_dn2 * locals.var_psisubsat__blk683) + (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683_dn2)) / (2.0 * assign22740_e31948)), (((locals.var_psisubsat__blk683_dn6 * locals.var_psisubsat__blk683) + (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683_dn6)) / (2.0 * assign22740_e31948)), (((locals.var_psisubsat__blk683_dn7 * locals.var_psisubsat__blk683) + (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683_dn7)) / (2.0 * assign22740_e31948)), (((locals.var_psisubsat__blk683_dn10 * locals.var_psisubsat__blk683) + (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683_dn10)) / (2.0 * assign22740_e31948)), (((locals.var_psisubsat__blk683_dn11 * locals.var_psisubsat__blk683) + (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683_dn11)) / (2.0 * assign22740_e31948)), (((locals.var_psisubsat__blk683_dn12 * locals.var_psisubsat__blk683) + (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683_dn12)) / (2.0 * assign22740_e31948)), (((locals.var_psisubsat__blk683_dn17 * locals.var_psisubsat__blk683) + (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683_dn17)) / (2.0 * assign22740_e31948)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22740_e31950;
        locals.var_tmf1_dn0 = assign22740_e31950_d_n0;
        locals.var_tmf1_dn2 = assign22740_e31950_d_n2;
        locals.var_tmf1_dn6 = assign22740_e31950_d_n6;
        locals.var_tmf1_dn7 = assign22740_e31950_d_n7;
        locals.var_tmf1_dn10 = assign22740_e31950_d_n10;
        locals.var_tmf1_dn11 = assign22740_e31950_d_n11;
        locals.var_tmf1_dn12 = assign22740_e31950_d_n12;
        locals.var_tmf1_dn17 = assign22740_e31950_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign22750_e31971, assign22750_e31971_d_n0, assign22750_e31971_d_n2, assign22750_e31971_d_n6, assign22750_e31971_d_n7, assign22750_e31971_d_n10, assign22750_e31971_d_n11, assign22750_e31971_d_n12, assign22750_e31971_d_n17,) = {
    if ((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) {
        let assign22750_e31964: f64 = (locals.var_psisubsat__blk683 + locals.var_tmf1);
        let assign22750_e31965: f64 = (0.5 * assign22750_e31964);
        let assign22750_e31968: f64 = (1e-10 * 0.001);
        let assign22750_e31969: f64 = (assign22750_e31965 + assign22750_e31968);
        (assign22750_e31969, (0.5 * (locals.var_psisubsat__blk683_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_psisubsat__blk683_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_psisubsat__blk683_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_psisubsat__blk683_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_psisubsat__blk683_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_psisubsat__blk683_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_psisubsat__blk683_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_psisubsat__blk683_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_psisubsat__blk683, locals.var_psisubsat__blk683_dn0, locals.var_psisubsat__blk683_dn2, locals.var_psisubsat__blk683_dn6, locals.var_psisubsat__blk683_dn7, locals.var_psisubsat__blk683_dn10, locals.var_psisubsat__blk683_dn11, locals.var_psisubsat__blk683_dn12, locals.var_psisubsat__blk683_dn17,)
    }
};
        locals.var_psisubsat__blk683 = assign22750_e31971;
        locals.var_psisubsat__blk683_dn0 = assign22750_e31971_d_n0;
        locals.var_psisubsat__blk683_dn2 = assign22750_e31971_d_n2;
        locals.var_psisubsat__blk683_dn6 = assign22750_e31971_d_n6;
        locals.var_psisubsat__blk683_dn7 = assign22750_e31971_d_n7;
        locals.var_psisubsat__blk683_dn10 = assign22750_e31971_d_n10;
        locals.var_psisubsat__blk683_dn11 = assign22750_e31971_d_n11;
        locals.var_psisubsat__blk683_dn12 = assign22750_e31971_d_n12;
        locals.var_psisubsat__blk683_dn17 = assign22750_e31971_d_n17;
        locals.var_psisubsat__blk683_rv = 0.0;

        let assign22760_e31974: f64 = if locals.var_psisubsat__blk683 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard695 = assign22760_e31974;
        locals.var_guard695_rv = 0.0;

        let (assign22770_e31989, assign22770_e31989_d_n0, assign22770_e31989_d_n2, assign22770_e31989_d_n6, assign22770_e31989_d_n7, assign22770_e31989_d_n10, assign22770_e31989_d_n11, assign22770_e31989_d_n12, assign22770_e31989_d_n17,) = {
    if (((((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) && (locals.var_guard686 == 0.0)) && (locals.var_guard695 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat__blk683, locals.var_psisubsat__blk683_dn0, locals.var_psisubsat__blk683_dn2, locals.var_psisubsat__blk683_dn6, locals.var_psisubsat__blk683_dn7, locals.var_psisubsat__blk683_dn10, locals.var_psisubsat__blk683_dn11, locals.var_psisubsat__blk683_dn12, locals.var_psisubsat__blk683_dn17,)
    }
};
        locals.var_psisubsat__blk683 = assign22770_e31989;
        locals.var_psisubsat__blk683_dn0 = assign22770_e31989_d_n0;
        locals.var_psisubsat__blk683_dn2 = assign22770_e31989_d_n2;
        locals.var_psisubsat__blk683_dn6 = assign22770_e31989_d_n6;
        locals.var_psisubsat__blk683_dn7 = assign22770_e31989_d_n7;
        locals.var_psisubsat__blk683_dn10 = assign22770_e31989_d_n10;
        locals.var_psisubsat__blk683_dn11 = assign22770_e31989_d_n11;
        locals.var_psisubsat__blk683_dn12 = assign22770_e31989_d_n12;
        locals.var_psisubsat__blk683_dn17 = assign22770_e31989_d_n17;
        locals.var_psisubsat__blk683_rv = 0.0;

        let (assign22780_e32001, assign22780_e32001_d_n0, assign22780_e32001_d_n2, assign22780_e32001_d_n6, assign22780_e32001_d_n7, assign22780_e32001_d_n10, assign22780_e32001_d_n11, assign22780_e32001_d_n12, assign22780_e32001_d_n17,) = {
    if (((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) {
        let assign22780_e31999: f64 = (locals.var_psisubsat__blk683 + 1e-50);
        (assign22780_e31999, locals.var_psisubsat__blk683_dn0, locals.var_psisubsat__blk683_dn2, locals.var_psisubsat__blk683_dn6, locals.var_psisubsat__blk683_dn7, locals.var_psisubsat__blk683_dn10, locals.var_psisubsat__blk683_dn11, locals.var_psisubsat__blk683_dn12, locals.var_psisubsat__blk683_dn17,)
    } else {
        (locals.var_psisubsat__blk683, locals.var_psisubsat__blk683_dn0, locals.var_psisubsat__blk683_dn2, locals.var_psisubsat__blk683_dn6, locals.var_psisubsat__blk683_dn7, locals.var_psisubsat__blk683_dn10, locals.var_psisubsat__blk683_dn11, locals.var_psisubsat__blk683_dn12, locals.var_psisubsat__blk683_dn17,)
    }
};
        locals.var_psisubsat__blk683 = assign22780_e32001;
        locals.var_psisubsat__blk683_dn0 = assign22780_e32001_d_n0;
        locals.var_psisubsat__blk683_dn2 = assign22780_e32001_d_n2;
        locals.var_psisubsat__blk683_dn6 = assign22780_e32001_d_n6;
        locals.var_psisubsat__blk683_dn7 = assign22780_e32001_d_n7;
        locals.var_psisubsat__blk683_dn10 = assign22780_e32001_d_n10;
        locals.var_psisubsat__blk683_dn11 = assign22780_e32001_d_n11;
        locals.var_psisubsat__blk683_dn12 = assign22780_e32001_d_n12;
        locals.var_psisubsat__blk683_dn17 = assign22780_e32001_d_n17;
        locals.var_psisubsat__blk683_rv = 0.0;

        let (assign22790_e32015, assign22790_e32015_d_n0, assign22790_e32015_d_n2, assign22790_e32015_d_n6, assign22790_e32015_d_n7, assign22790_e32015_d_n10, assign22790_e32015_d_n11, assign22790_e32015_d_n12, assign22790_e32015_d_n17,) = {
    if (((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) {
        let assign22790_e32010: f64 = (-locals.var_xsub2);
        let assign22790_e32012: f64 = (assign22790_e32010 / locals.var_psisubsat__blk683);
        let assign22790_e32013: f64 = (assign22790_e32012).exp();
        (assign22790_e32013, (assign22790_e32013 * (-((assign22790_e32010 * locals.var_psisubsat__blk683_dn0) / (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683)))), (assign22790_e32013 * (-((assign22790_e32010 * locals.var_psisubsat__blk683_dn2) / (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683)))), (assign22790_e32013 * (-((assign22790_e32010 * locals.var_psisubsat__blk683_dn6) / (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683)))), (assign22790_e32013 * (-((assign22790_e32010 * locals.var_psisubsat__blk683_dn7) / (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683)))), (assign22790_e32013 * (-((assign22790_e32010 * locals.var_psisubsat__blk683_dn10) / (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683)))), (assign22790_e32013 * (-((assign22790_e32010 * locals.var_psisubsat__blk683_dn11) / (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683)))), (assign22790_e32013 * (-((assign22790_e32010 * locals.var_psisubsat__blk683_dn12) / (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683)))), (assign22790_e32013 * (-((assign22790_e32010 * locals.var_psisubsat__blk683_dn17) / (locals.var_psisubsat__blk683 * locals.var_psisubsat__blk683)))),)
    } else {
        (locals.var_t2__blk673, locals.var_t2__blk673_dn0, locals.var_t2__blk673_dn2, locals.var_t2__blk673_dn6, locals.var_t2__blk673_dn7, locals.var_t2__blk673_dn10, locals.var_t2__blk673_dn11, locals.var_t2__blk673_dn12, locals.var_t2__blk673_dn17,)
    }
};
        locals.var_t2__blk673 = assign22790_e32015;
        locals.var_t2__blk673_dn0 = assign22790_e32015_d_n0;
        locals.var_t2__blk673_dn2 = assign22790_e32015_d_n2;
        locals.var_t2__blk673_dn6 = assign22790_e32015_d_n6;
        locals.var_t2__blk673_dn7 = assign22790_e32015_d_n7;
        locals.var_t2__blk673_dn10 = assign22790_e32015_d_n10;
        locals.var_t2__blk673_dn11 = assign22790_e32015_d_n11;
        locals.var_t2__blk673_dn12 = assign22790_e32015_d_n12;
        locals.var_t2__blk673_dn17 = assign22790_e32015_d_n17;
        locals.var_t2__blk673_rv = 0.0;

        let (assign22800_e32031, assign22800_e32031_d_n0, assign22800_e32031_d_n2, assign22800_e32031_d_n6, assign22800_e32031_d_n7, assign22800_e32031_d_n10, assign22800_e32031_d_n11, assign22800_e32031_d_n12, assign22800_e32031_d_n17,) = {
    if (((locals.var_guard671 != 0.0) && (locals.var_guard684 == 0.0)) && (locals.var_guard685 == 0.0)) {
        let assign22800_e32025: f64 = (locals.var_xsub1 * locals.var_psisubsat__blk683);
        let assign22800_e32027: f64 = (assign22800_e32025 * locals.var_ids);
        let assign22800_e32029: f64 = (assign22800_e32027 * locals.var_t2__blk673);
        (assign22800_e32029, (((((locals.var_xsub1 * locals.var_psisubsat__blk683_dn0) * locals.var_ids) + (assign22800_e32025 * locals.var_ids_dn0)) * locals.var_t2__blk673) + (assign22800_e32027 * locals.var_t2__blk673_dn0)), (((((locals.var_xsub1 * locals.var_psisubsat__blk683_dn2) * locals.var_ids) + (assign22800_e32025 * locals.var_ids_dn2)) * locals.var_t2__blk673) + (assign22800_e32027 * locals.var_t2__blk673_dn2)), (((((locals.var_xsub1 * locals.var_psisubsat__blk683_dn6) * locals.var_ids) + (assign22800_e32025 * locals.var_ids_dn6)) * locals.var_t2__blk673) + (assign22800_e32027 * locals.var_t2__blk673_dn6)), (((((locals.var_xsub1 * locals.var_psisubsat__blk683_dn7) * locals.var_ids) + (assign22800_e32025 * locals.var_ids_dn7)) * locals.var_t2__blk673) + (assign22800_e32027 * locals.var_t2__blk673_dn7)), (((((locals.var_xsub1 * locals.var_psisubsat__blk683_dn10) * locals.var_ids) + (assign22800_e32025 * locals.var_ids_dn10)) * locals.var_t2__blk673) + (assign22800_e32027 * locals.var_t2__blk673_dn10)), (((((locals.var_xsub1 * locals.var_psisubsat__blk683_dn11) * locals.var_ids) + (assign22800_e32025 * locals.var_ids_dn11)) * locals.var_t2__blk673) + (assign22800_e32027 * locals.var_t2__blk673_dn11)), (((((locals.var_xsub1 * locals.var_psisubsat__blk683_dn12) * locals.var_ids) + (assign22800_e32025 * locals.var_ids_dn12)) * locals.var_t2__blk673) + (assign22800_e32027 * locals.var_t2__blk673_dn12)), (((((locals.var_xsub1 * locals.var_psisubsat__blk683_dn17) * locals.var_ids) + (assign22800_e32025 * locals.var_ids_dn17)) * locals.var_t2__blk673) + (assign22800_e32027 * locals.var_t2__blk673_dn17)),)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign22800_e32031;
        locals.var_isub_dn0 = assign22800_e32031_d_n0;
        locals.var_isub_dn2 = assign22800_e32031_d_n2;
        locals.var_isub_dn6 = assign22800_e32031_d_n6;
        locals.var_isub_dn7 = assign22800_e32031_d_n7;
        locals.var_isub_dn10 = assign22800_e32031_d_n10;
        locals.var_isub_dn11 = assign22800_e32031_d_n11;
        locals.var_isub_dn12 = assign22800_e32031_d_n12;
        locals.var_isub_dn17 = assign22800_e32031_d_n17;
        locals.var_isub_rv = 0.0;

        let assign22810_e32042: f64 = if (((p.p25 == 1.0) && (p.p26 == 2.0)) && (p.p43 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard696 = assign22810_e32042;
        locals.var_guard696_rv = 0.0;

        let (assign22860_e32116, assign22860_e32116_d_n0, assign22860_e32116_d_n2, assign22860_e32116_d_n6, assign22860_e32116_d_n7, assign22860_e32116_d_n10, assign22860_e32116_d_n11, assign22860_e32116_d_n12, assign22860_e32116_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk700, locals.var_t5__blk700_dn0, locals.var_t5__blk700_dn2, locals.var_t5__blk700_dn6, locals.var_t5__blk700_dn7, locals.var_t5__blk700_dn10, locals.var_t5__blk700_dn11, locals.var_t5__blk700_dn12, locals.var_t5__blk700_dn17,)
    }
};
        locals.var_t5__blk700 = assign22860_e32116;
        locals.var_t5__blk700_dn0 = assign22860_e32116_d_n0;
        locals.var_t5__blk700_dn2 = assign22860_e32116_d_n2;
        locals.var_t5__blk700_dn6 = assign22860_e32116_d_n6;
        locals.var_t5__blk700_dn7 = assign22860_e32116_d_n7;
        locals.var_t5__blk700_dn10 = assign22860_e32116_d_n10;
        locals.var_t5__blk700_dn11 = assign22860_e32116_d_n11;
        locals.var_t5__blk700_dn12 = assign22860_e32116_d_n12;
        locals.var_t5__blk700_dn17 = assign22860_e32116_d_n17;
        locals.var_t5__blk700_rv = 0.0;

        let (assign22870_e32126, assign22870_e32126_d_n0, assign22870_e32126_d_n2, assign22870_e32126_d_n6, assign22870_e32126_d_n7, assign22870_e32126_d_n10, assign22870_e32126_d_n11, assign22870_e32126_d_n12, assign22870_e32126_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        let assign22870_e32120: f64 = (locals.var_pb2 - locals.var_t5__blk700);
        let assign22870_e32123: f64 = (locals.var_pb2 * 0.01);
        let assign22870_e32124: f64 = (assign22870_e32120 - assign22870_e32123);
        (assign22870_e32124, ((locals.var_pb2_dn0 - locals.var_t5__blk700_dn0) - (locals.var_pb2_dn0 * 0.01)), ((locals.var_pb2_dn2 - locals.var_t5__blk700_dn2) - (locals.var_pb2_dn2 * 0.01)), ((locals.var_pb2_dn6 - locals.var_t5__blk700_dn6) - (locals.var_pb2_dn6 * 0.01)), ((locals.var_pb2_dn7 - locals.var_t5__blk700_dn7) - (locals.var_pb2_dn7 * 0.01)), ((locals.var_pb2_dn10 - locals.var_t5__blk700_dn10) - (locals.var_pb2_dn10 * 0.01)), ((locals.var_pb2_dn11 - locals.var_t5__blk700_dn11) - (locals.var_pb2_dn11 * 0.01)), ((locals.var_pb2_dn12 - locals.var_t5__blk700_dn12) - (locals.var_pb2_dn12 * 0.01)), ((locals.var_pb2_dn17 - locals.var_t5__blk700_dn17) - (locals.var_pb2_dn17 * 0.01)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22870_e32126;
        locals.var_tmf1_dn0 = assign22870_e32126_d_n0;
        locals.var_tmf1_dn2 = assign22870_e32126_d_n2;
        locals.var_tmf1_dn6 = assign22870_e32126_d_n6;
        locals.var_tmf1_dn7 = assign22870_e32126_d_n7;
        locals.var_tmf1_dn10 = assign22870_e32126_d_n10;
        locals.var_tmf1_dn11 = assign22870_e32126_d_n11;
        locals.var_tmf1_dn12 = assign22870_e32126_d_n12;
        locals.var_tmf1_dn17 = assign22870_e32126_d_n17;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_82(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv17 = ctx.node_voltage(nodes[17]);
        let (assign22880_e32136, assign22880_e32136_d_n0, assign22880_e32136_d_n2, assign22880_e32136_d_n6, assign22880_e32136_d_n7, assign22880_e32136_d_n10, assign22880_e32136_d_n11, assign22880_e32136_d_n12, assign22880_e32136_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        let assign22880_e32130: f64 = (4.0 * locals.var_pb2);
        let assign22880_e32133: f64 = (locals.var_pb2 * 0.01);
        let assign22880_e32134: f64 = (assign22880_e32130 * assign22880_e32133);
        (assign22880_e32134, (((4.0 * locals.var_pb2_dn0) * assign22880_e32133) + (assign22880_e32130 * (locals.var_pb2_dn0 * 0.01))), (((4.0 * locals.var_pb2_dn2) * assign22880_e32133) + (assign22880_e32130 * (locals.var_pb2_dn2 * 0.01))), (((4.0 * locals.var_pb2_dn6) * assign22880_e32133) + (assign22880_e32130 * (locals.var_pb2_dn6 * 0.01))), (((4.0 * locals.var_pb2_dn7) * assign22880_e32133) + (assign22880_e32130 * (locals.var_pb2_dn7 * 0.01))), (((4.0 * locals.var_pb2_dn10) * assign22880_e32133) + (assign22880_e32130 * (locals.var_pb2_dn10 * 0.01))), (((4.0 * locals.var_pb2_dn11) * assign22880_e32133) + (assign22880_e32130 * (locals.var_pb2_dn11 * 0.01))), (((4.0 * locals.var_pb2_dn12) * assign22880_e32133) + (assign22880_e32130 * (locals.var_pb2_dn12 * 0.01))), (((4.0 * locals.var_pb2_dn17) * assign22880_e32133) + (assign22880_e32130 * (locals.var_pb2_dn17 * 0.01))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign22880_e32136;
        locals.var_tmf2_dn0 = assign22880_e32136_d_n0;
        locals.var_tmf2_dn2 = assign22880_e32136_d_n2;
        locals.var_tmf2_dn6 = assign22880_e32136_d_n6;
        locals.var_tmf2_dn7 = assign22880_e32136_d_n7;
        locals.var_tmf2_dn10 = assign22880_e32136_d_n10;
        locals.var_tmf2_dn11 = assign22880_e32136_d_n11;
        locals.var_tmf2_dn12 = assign22880_e32136_d_n12;
        locals.var_tmf2_dn17 = assign22880_e32136_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign22890_e32146, assign22890_e32146_d_n0, assign22890_e32146_d_n2, assign22890_e32146_d_n6, assign22890_e32146_d_n7, assign22890_e32146_d_n10, assign22890_e32146_d_n11, assign22890_e32146_d_n12, assign22890_e32146_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        let (assign22890_e32144, assign22890_e32144_d_n0, assign22890_e32144_d_n2, assign22890_e32144_d_n6, assign22890_e32144_d_n7, assign22890_e32144_d_n10, assign22890_e32144_d_n11, assign22890_e32144_d_n12, assign22890_e32144_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign22890_e32143: f64 = (-locals.var_tmf2);
                (assign22890_e32143, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign22890_e32144, assign22890_e32144_d_n0, assign22890_e32144_d_n2, assign22890_e32144_d_n6, assign22890_e32144_d_n7, assign22890_e32144_d_n10, assign22890_e32144_d_n11, assign22890_e32144_d_n12, assign22890_e32144_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign22890_e32146;
        locals.var_tmf2_dn0 = assign22890_e32146_d_n0;
        locals.var_tmf2_dn2 = assign22890_e32146_d_n2;
        locals.var_tmf2_dn6 = assign22890_e32146_d_n6;
        locals.var_tmf2_dn7 = assign22890_e32146_d_n7;
        locals.var_tmf2_dn10 = assign22890_e32146_d_n10;
        locals.var_tmf2_dn11 = assign22890_e32146_d_n11;
        locals.var_tmf2_dn12 = assign22890_e32146_d_n12;
        locals.var_tmf2_dn17 = assign22890_e32146_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign22900_e32155, assign22900_e32155_d_n0, assign22900_e32155_d_n2, assign22900_e32155_d_n6, assign22900_e32155_d_n7, assign22900_e32155_d_n10, assign22900_e32155_d_n11, assign22900_e32155_d_n12, assign22900_e32155_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        let assign22900_e32150: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign22900_e32152: f64 = (assign22900_e32150 + locals.var_tmf2);
        let assign22900_e32153: f64 = (assign22900_e32152).sqrt();
        (assign22900_e32153, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign22900_e32153)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign22900_e32153)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign22900_e32153)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign22900_e32153)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign22900_e32153)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign22900_e32153)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign22900_e32153)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign22900_e32153)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign22900_e32155;
        locals.var_tmf2_dn0 = assign22900_e32155_d_n0;
        locals.var_tmf2_dn2 = assign22900_e32155_d_n2;
        locals.var_tmf2_dn6 = assign22900_e32155_d_n6;
        locals.var_tmf2_dn7 = assign22900_e32155_d_n7;
        locals.var_tmf2_dn10 = assign22900_e32155_d_n10;
        locals.var_tmf2_dn11 = assign22900_e32155_d_n11;
        locals.var_tmf2_dn12 = assign22900_e32155_d_n12;
        locals.var_tmf2_dn17 = assign22900_e32155_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign22910_e32165, assign22910_e32165_d_n0, assign22910_e32165_d_n2, assign22910_e32165_d_n6, assign22910_e32165_d_n7, assign22910_e32165_d_n10, assign22910_e32165_d_n11, assign22910_e32165_d_n12, assign22910_e32165_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        let assign22910_e32161: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign22910_e32162: f64 = (0.5 * assign22910_e32161);
        let assign22910_e32163: f64 = (locals.var_pb2 - assign22910_e32162);
        (assign22910_e32163, (locals.var_pb2_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_pb2_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_pb2_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_pb2_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_pb2_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_pb2_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_pb2_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_pb2_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_t5__blk700, locals.var_t5__blk700_dn0, locals.var_t5__blk700_dn2, locals.var_t5__blk700_dn6, locals.var_t5__blk700_dn7, locals.var_t5__blk700_dn10, locals.var_t5__blk700_dn11, locals.var_t5__blk700_dn12, locals.var_t5__blk700_dn17,)
    }
};
        locals.var_t5__blk700 = assign22910_e32165;
        locals.var_t5__blk700_dn0 = assign22910_e32165_d_n0;
        locals.var_t5__blk700_dn2 = assign22910_e32165_d_n2;
        locals.var_t5__blk700_dn6 = assign22910_e32165_d_n6;
        locals.var_t5__blk700_dn7 = assign22910_e32165_d_n7;
        locals.var_t5__blk700_dn10 = assign22910_e32165_d_n10;
        locals.var_t5__blk700_dn11 = assign22910_e32165_d_n11;
        locals.var_t5__blk700_dn12 = assign22910_e32165_d_n12;
        locals.var_t5__blk700_dn17 = assign22910_e32165_d_n17;
        locals.var_t5__blk700_rv = 0.0;

        let (assign22930_e32182, assign22930_e32182_d_n0, assign22930_e32182_d_n2, assign22930_e32182_d_n6, assign22930_e32182_d_n7, assign22930_e32182_d_n10, assign22930_e32182_d_n11, assign22930_e32182_d_n12, assign22930_e32182_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        let assign22930_e32173: f64 = (2.0 * 1.034943e-10);
        let assign22930_e32175: f64 = (assign22930_e32173 * 1.6021918e-19);
        let assign22930_e32177: f64 = (assign22930_e32175 * locals.var_uc_nsubs);
        let assign22930_e32179: f64 = (assign22930_e32177 * locals.var_beta_inv);
        let assign22930_e32180: f64 = (assign22930_e32179).sqrt();
        (assign22930_e32180, (((assign22930_e32175 * locals.var_uc_nsubs_dn0) * locals.var_beta_inv) / (2.0 * assign22930_e32180)), (((assign22930_e32175 * locals.var_uc_nsubs_dn2) * locals.var_beta_inv) / (2.0 * assign22930_e32180)), (((assign22930_e32175 * locals.var_uc_nsubs_dn6) * locals.var_beta_inv) / (2.0 * assign22930_e32180)), (((assign22930_e32175 * locals.var_uc_nsubs_dn7) * locals.var_beta_inv) / (2.0 * assign22930_e32180)), ((((assign22930_e32175 * locals.var_uc_nsubs_dn10) * locals.var_beta_inv) + (assign22930_e32177 * locals.var_beta_inv_dn10)) / (2.0 * assign22930_e32180)), (((assign22930_e32175 * locals.var_uc_nsubs_dn11) * locals.var_beta_inv) / (2.0 * assign22930_e32180)), (((assign22930_e32175 * locals.var_uc_nsubs_dn12) * locals.var_beta_inv) / (2.0 * assign22930_e32180)), (((assign22930_e32175 * locals.var_uc_nsubs_dn17) * locals.var_beta_inv) / (2.0 * assign22930_e32180)),)
    } else {
        (locals.var_t6__blk701, locals.var_t6__blk701_dn0, locals.var_t6__blk701_dn2, locals.var_t6__blk701_dn6, locals.var_t6__blk701_dn7, locals.var_t6__blk701_dn10, locals.var_t6__blk701_dn11, locals.var_t6__blk701_dn12, locals.var_t6__blk701_dn17,)
    }
};
        locals.var_t6__blk701 = assign22930_e32182;
        locals.var_t6__blk701_dn0 = assign22930_e32182_d_n0;
        locals.var_t6__blk701_dn2 = assign22930_e32182_d_n2;
        locals.var_t6__blk701_dn6 = assign22930_e32182_d_n6;
        locals.var_t6__blk701_dn7 = assign22930_e32182_d_n7;
        locals.var_t6__blk701_dn10 = assign22930_e32182_d_n10;
        locals.var_t6__blk701_dn11 = assign22930_e32182_d_n11;
        locals.var_t6__blk701_dn12 = assign22930_e32182_d_n12;
        locals.var_t6__blk701_dn17 = assign22930_e32182_d_n17;
        locals.var_t6__blk701_rv = 0.0;

        let (assign22940_e32192, assign22940_e32192_d_n0, assign22940_e32192_d_n2, assign22940_e32192_d_n6, assign22940_e32192_d_n7, assign22940_e32192_d_n10, assign22940_e32192_d_n11, assign22940_e32192_d_n12, assign22940_e32192_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        let assign22940_e32188: f64 = (locals.var_ps0z - locals.var_t5__blk700);
        let assign22940_e32189: f64 = (locals.var_beta * assign22940_e32188);
        let assign22940_e32190: f64 = assign22940_e32189;
        (assign22940_e32190, (locals.var_beta * (locals.var_ps0z_dn0 - locals.var_t5__blk700_dn0)), (locals.var_beta * (locals.var_ps0z_dn2 - locals.var_t5__blk700_dn2)), (locals.var_beta * (locals.var_ps0z_dn6 - locals.var_t5__blk700_dn6)), (locals.var_beta * (locals.var_ps0z_dn7 - locals.var_t5__blk700_dn7)), ((locals.var_beta_dn10 * assign22940_e32188) + (locals.var_beta * (locals.var_ps0z_dn10 - locals.var_t5__blk700_dn10))), (locals.var_beta * (locals.var_ps0z_dn11 - locals.var_t5__blk700_dn11)), (locals.var_beta * (locals.var_ps0z_dn12 - locals.var_t5__blk700_dn12)), (locals.var_beta * (locals.var_ps0z_dn17 - locals.var_t5__blk700_dn17)),)
    } else {
        (locals.var_t7__blk702, locals.var_t7__blk702_dn0, locals.var_t7__blk702_dn2, locals.var_t7__blk702_dn6, locals.var_t7__blk702_dn7, locals.var_t7__blk702_dn10, locals.var_t7__blk702_dn11, locals.var_t7__blk702_dn12, locals.var_t7__blk702_dn17,)
    }
};
        locals.var_t7__blk702 = assign22940_e32192;
        locals.var_t7__blk702_dn0 = assign22940_e32192_d_n0;
        locals.var_t7__blk702_dn2 = assign22940_e32192_d_n2;
        locals.var_t7__blk702_dn6 = assign22940_e32192_d_n6;
        locals.var_t7__blk702_dn7 = assign22940_e32192_d_n7;
        locals.var_t7__blk702_dn10 = assign22940_e32192_d_n10;
        locals.var_t7__blk702_dn11 = assign22940_e32192_d_n11;
        locals.var_t7__blk702_dn12 = assign22940_e32192_d_n12;
        locals.var_t7__blk702_dn17 = assign22940_e32192_d_n17;
        locals.var_t7__blk702_rv = 0.0;

        let (assign22950_e32205, assign22950_e32205_d_n0, assign22950_e32205_d_n2, assign22950_e32205_d_n6, assign22950_e32205_d_n7, assign22950_e32205_d_n10, assign22950_e32205_d_n11, assign22950_e32205_d_n12, assign22950_e32205_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        let (assign22950_e32203, assign22950_e32203_d_n0, assign22950_e32203_d_n2, assign22950_e32203_d_n6, assign22950_e32203_d_n7, assign22950_e32203_d_n10, assign22950_e32203_d_n11, assign22950_e32203_d_n12, assign22950_e32203_d_n17,) = {
            if (locals.var_t7__blk702 > 0.0) {
                let assign22950_e32198: f64 = (locals.var_t7__blk702).sqrt();
                (assign22950_e32198, (locals.var_t7__blk702_dn0 / (2.0 * assign22950_e32198)), (locals.var_t7__blk702_dn2 / (2.0 * assign22950_e32198)), (locals.var_t7__blk702_dn6 / (2.0 * assign22950_e32198)), (locals.var_t7__blk702_dn7 / (2.0 * assign22950_e32198)), (locals.var_t7__blk702_dn10 / (2.0 * assign22950_e32198)), (locals.var_t7__blk702_dn11 / (2.0 * assign22950_e32198)), (locals.var_t7__blk702_dn12 / (2.0 * assign22950_e32198)), (locals.var_t7__blk702_dn17 / (2.0 * assign22950_e32198)),)
            } else {
                let assign22950_e32200: f64 = (-locals.var_t7__blk702);
                let assign22950_e32201: f64 = (assign22950_e32200).sqrt();
                let assign22950_e32202: f64 = (-assign22950_e32201);
                (assign22950_e32202, (-((-locals.var_t7__blk702_dn0) / (2.0 * assign22950_e32201))), (-((-locals.var_t7__blk702_dn2) / (2.0 * assign22950_e32201))), (-((-locals.var_t7__blk702_dn6) / (2.0 * assign22950_e32201))), (-((-locals.var_t7__blk702_dn7) / (2.0 * assign22950_e32201))), (-((-locals.var_t7__blk702_dn10) / (2.0 * assign22950_e32201))), (-((-locals.var_t7__blk702_dn11) / (2.0 * assign22950_e32201))), (-((-locals.var_t7__blk702_dn12) / (2.0 * assign22950_e32201))), (-((-locals.var_t7__blk702_dn17) / (2.0 * assign22950_e32201))),)
            }
        };
        (assign22950_e32203, assign22950_e32203_d_n0, assign22950_e32203_d_n2, assign22950_e32203_d_n6, assign22950_e32203_d_n7, assign22950_e32203_d_n10, assign22950_e32203_d_n11, assign22950_e32203_d_n12, assign22950_e32203_d_n17,)
    } else {
        (locals.var_t7__blk702, locals.var_t7__blk702_dn0, locals.var_t7__blk702_dn2, locals.var_t7__blk702_dn6, locals.var_t7__blk702_dn7, locals.var_t7__blk702_dn10, locals.var_t7__blk702_dn11, locals.var_t7__blk702_dn12, locals.var_t7__blk702_dn17,)
    }
};
        locals.var_t7__blk702 = assign22950_e32205;
        locals.var_t7__blk702_dn0 = assign22950_e32205_d_n0;
        locals.var_t7__blk702_dn2 = assign22950_e32205_d_n2;
        locals.var_t7__blk702_dn6 = assign22950_e32205_d_n6;
        locals.var_t7__blk702_dn7 = assign22950_e32205_d_n7;
        locals.var_t7__blk702_dn10 = assign22950_e32205_d_n10;
        locals.var_t7__blk702_dn11 = assign22950_e32205_d_n11;
        locals.var_t7__blk702_dn12 = assign22950_e32205_d_n12;
        locals.var_t7__blk702_dn17 = assign22950_e32205_d_n17;
        locals.var_t7__blk702_rv = 0.0;

        let (assign22960_e32214, assign22960_e32214_d_n0, assign22960_e32214_d_n2, assign22960_e32214_d_n6, assign22960_e32214_d_n7, assign22960_e32214_d_n10, assign22960_e32214_d_n11, assign22960_e32214_d_n12, assign22960_e32214_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        let assign22960_e32210: f64 = (locals.var_beta * locals.var_ps0z);
        let assign22960_e32211: f64 = assign22960_e32210;
        let assign22960_e32212: f64 = (assign22960_e32211).sqrt();
        (assign22960_e32212, ((locals.var_beta * locals.var_ps0z_dn0) / (2.0 * assign22960_e32212)), ((locals.var_beta * locals.var_ps0z_dn2) / (2.0 * assign22960_e32212)), ((locals.var_beta * locals.var_ps0z_dn6) / (2.0 * assign22960_e32212)), ((locals.var_beta * locals.var_ps0z_dn7) / (2.0 * assign22960_e32212)), (((locals.var_beta_dn10 * locals.var_ps0z) + (locals.var_beta * locals.var_ps0z_dn10)) / (2.0 * assign22960_e32212)), ((locals.var_beta * locals.var_ps0z_dn11) / (2.0 * assign22960_e32212)), ((locals.var_beta * locals.var_ps0z_dn12) / (2.0 * assign22960_e32212)), ((locals.var_beta * locals.var_ps0z_dn17) / (2.0 * assign22960_e32212)),)
    } else {
        (locals.var_t8__blk703, locals.var_t8__blk703_dn0, locals.var_t8__blk703_dn2, locals.var_t8__blk703_dn6, locals.var_t8__blk703_dn7, locals.var_t8__blk703_dn10, locals.var_t8__blk703_dn11, locals.var_t8__blk703_dn12, locals.var_t8__blk703_dn17,)
    }
};
        locals.var_t8__blk703 = assign22960_e32214;
        locals.var_t8__blk703_dn0 = assign22960_e32214_d_n0;
        locals.var_t8__blk703_dn2 = assign22960_e32214_d_n2;
        locals.var_t8__blk703_dn6 = assign22960_e32214_d_n6;
        locals.var_t8__blk703_dn7 = assign22960_e32214_d_n7;
        locals.var_t8__blk703_dn10 = assign22960_e32214_d_n10;
        locals.var_t8__blk703_dn11 = assign22960_e32214_d_n11;
        locals.var_t8__blk703_dn12 = assign22960_e32214_d_n12;
        locals.var_t8__blk703_dn17 = assign22960_e32214_d_n17;
        locals.var_t8__blk703_rv = 0.0;

        let (assign22970_e32223, assign22970_e32223_d_n0, assign22970_e32223_d_n2, assign22970_e32223_d_n6, assign22970_e32223_d_n7, assign22970_e32223_d_n10, assign22970_e32223_d_n11, assign22970_e32223_d_n12, assign22970_e32223_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        let assign22970_e32217: f64 = (-locals.var_t6__blk701);
        let assign22970_e32220: f64 = (locals.var_t7__blk702 - locals.var_t8__blk703);
        let assign22970_e32221: f64 = (assign22970_e32217 * assign22970_e32220);
        (assign22970_e32221, (((-locals.var_t6__blk701_dn0) * assign22970_e32220) + (assign22970_e32217 * (locals.var_t7__blk702_dn0 - locals.var_t8__blk703_dn0))), (((-locals.var_t6__blk701_dn2) * assign22970_e32220) + (assign22970_e32217 * (locals.var_t7__blk702_dn2 - locals.var_t8__blk703_dn2))), (((-locals.var_t6__blk701_dn6) * assign22970_e32220) + (assign22970_e32217 * (locals.var_t7__blk702_dn6 - locals.var_t8__blk703_dn6))), (((-locals.var_t6__blk701_dn7) * assign22970_e32220) + (assign22970_e32217 * (locals.var_t7__blk702_dn7 - locals.var_t8__blk703_dn7))), (((-locals.var_t6__blk701_dn10) * assign22970_e32220) + (assign22970_e32217 * (locals.var_t7__blk702_dn10 - locals.var_t8__blk703_dn10))), (((-locals.var_t6__blk701_dn11) * assign22970_e32220) + (assign22970_e32217 * (locals.var_t7__blk702_dn11 - locals.var_t8__blk703_dn11))), (((-locals.var_t6__blk701_dn12) * assign22970_e32220) + (assign22970_e32217 * (locals.var_t7__blk702_dn12 - locals.var_t8__blk703_dn12))), (((-locals.var_t6__blk701_dn17) * assign22970_e32220) + (assign22970_e32217 * (locals.var_t7__blk702_dn17 - locals.var_t8__blk703_dn17))),)
    } else {
        (locals.var_t9__blk704, locals.var_t9__blk704_dn0, locals.var_t9__blk704_dn2, locals.var_t9__blk704_dn6, locals.var_t9__blk704_dn7, locals.var_t9__blk704_dn10, locals.var_t9__blk704_dn11, locals.var_t9__blk704_dn12, locals.var_t9__blk704_dn17,)
    }
};
        locals.var_t9__blk704 = assign22970_e32223;
        locals.var_t9__blk704_dn0 = assign22970_e32223_d_n0;
        locals.var_t9__blk704_dn2 = assign22970_e32223_d_n2;
        locals.var_t9__blk704_dn6 = assign22970_e32223_d_n6;
        locals.var_t9__blk704_dn7 = assign22970_e32223_d_n7;
        locals.var_t9__blk704_dn10 = assign22970_e32223_d_n10;
        locals.var_t9__blk704_dn11 = assign22970_e32223_d_n11;
        locals.var_t9__blk704_dn12 = assign22970_e32223_d_n12;
        locals.var_t9__blk704_dn17 = assign22970_e32223_d_n17;
        locals.var_t9__blk704_rv = 0.0;

        let (assign22980_e32233, assign22980_e32233_d_n0, assign22980_e32233_d_n2, assign22980_e32233_d_n6, assign22980_e32233_d_n7, assign22980_e32233_d_n10, assign22980_e32233_d_n11, assign22980_e32233_d_n12, assign22980_e32233_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        let assign22980_e32227: f64 = (p.p47 - locals.var_t9__blk704);
        let assign22980_e32230: f64 = (p.p47 * 0.01);
        let assign22980_e32231: f64 = (assign22980_e32227 - assign22980_e32230);
        (assign22980_e32231, (-locals.var_t9__blk704_dn0), (-locals.var_t9__blk704_dn2), (-locals.var_t9__blk704_dn6), (-locals.var_t9__blk704_dn7), (-locals.var_t9__blk704_dn10), (-locals.var_t9__blk704_dn11), (-locals.var_t9__blk704_dn12), (-locals.var_t9__blk704_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign22980_e32233;
        locals.var_tmf1_dn0 = assign22980_e32233_d_n0;
        locals.var_tmf1_dn2 = assign22980_e32233_d_n2;
        locals.var_tmf1_dn6 = assign22980_e32233_d_n6;
        locals.var_tmf1_dn7 = assign22980_e32233_d_n7;
        locals.var_tmf1_dn10 = assign22980_e32233_d_n10;
        locals.var_tmf1_dn11 = assign22980_e32233_d_n11;
        locals.var_tmf1_dn12 = assign22980_e32233_d_n12;
        locals.var_tmf1_dn17 = assign22980_e32233_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign22990_e32243, assign22990_e32243_d_n0, assign22990_e32243_d_n2, assign22990_e32243_d_n6, assign22990_e32243_d_n7, assign22990_e32243_d_n10, assign22990_e32243_d_n11, assign22990_e32243_d_n12, assign22990_e32243_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        let assign22990_e32237: f64 = (4.0 * p.p47);
        let assign22990_e32240: f64 = (p.p47 * 0.01);
        let assign22990_e32241: f64 = (assign22990_e32237 * assign22990_e32240);
        (assign22990_e32241, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign22990_e32243;
        locals.var_tmf2_dn0 = assign22990_e32243_d_n0;
        locals.var_tmf2_dn2 = assign22990_e32243_d_n2;
        locals.var_tmf2_dn6 = assign22990_e32243_d_n6;
        locals.var_tmf2_dn7 = assign22990_e32243_d_n7;
        locals.var_tmf2_dn10 = assign22990_e32243_d_n10;
        locals.var_tmf2_dn11 = assign22990_e32243_d_n11;
        locals.var_tmf2_dn12 = assign22990_e32243_d_n12;
        locals.var_tmf2_dn17 = assign22990_e32243_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign23000_e32253, assign23000_e32253_d_n0, assign23000_e32253_d_n2, assign23000_e32253_d_n6, assign23000_e32253_d_n7, assign23000_e32253_d_n10, assign23000_e32253_d_n11, assign23000_e32253_d_n12, assign23000_e32253_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        let (assign23000_e32251, assign23000_e32251_d_n0, assign23000_e32251_d_n2, assign23000_e32251_d_n6, assign23000_e32251_d_n7, assign23000_e32251_d_n10, assign23000_e32251_d_n11, assign23000_e32251_d_n12, assign23000_e32251_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign23000_e32250: f64 = (-locals.var_tmf2);
                (assign23000_e32250, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign23000_e32251, assign23000_e32251_d_n0, assign23000_e32251_d_n2, assign23000_e32251_d_n6, assign23000_e32251_d_n7, assign23000_e32251_d_n10, assign23000_e32251_d_n11, assign23000_e32251_d_n12, assign23000_e32251_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign23000_e32253;
        locals.var_tmf2_dn0 = assign23000_e32253_d_n0;
        locals.var_tmf2_dn2 = assign23000_e32253_d_n2;
        locals.var_tmf2_dn6 = assign23000_e32253_d_n6;
        locals.var_tmf2_dn7 = assign23000_e32253_d_n7;
        locals.var_tmf2_dn10 = assign23000_e32253_d_n10;
        locals.var_tmf2_dn11 = assign23000_e32253_d_n11;
        locals.var_tmf2_dn12 = assign23000_e32253_d_n12;
        locals.var_tmf2_dn17 = assign23000_e32253_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign23010_e32262, assign23010_e32262_d_n0, assign23010_e32262_d_n2, assign23010_e32262_d_n6, assign23010_e32262_d_n7, assign23010_e32262_d_n10, assign23010_e32262_d_n11, assign23010_e32262_d_n12, assign23010_e32262_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        let assign23010_e32257: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign23010_e32259: f64 = (assign23010_e32257 + locals.var_tmf2);
        let assign23010_e32260: f64 = (assign23010_e32259).sqrt();
        (assign23010_e32260, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign23010_e32260)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign23010_e32260)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign23010_e32260)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign23010_e32260)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign23010_e32260)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign23010_e32260)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign23010_e32260)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign23010_e32260)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign23010_e32262;
        locals.var_tmf2_dn0 = assign23010_e32262_d_n0;
        locals.var_tmf2_dn2 = assign23010_e32262_d_n2;
        locals.var_tmf2_dn6 = assign23010_e32262_d_n6;
        locals.var_tmf2_dn7 = assign23010_e32262_d_n7;
        locals.var_tmf2_dn10 = assign23010_e32262_d_n10;
        locals.var_tmf2_dn11 = assign23010_e32262_d_n11;
        locals.var_tmf2_dn12 = assign23010_e32262_d_n12;
        locals.var_tmf2_dn17 = assign23010_e32262_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign23020_e32272, assign23020_e32272_d_n0, assign23020_e32272_d_n2, assign23020_e32272_d_n6, assign23020_e32272_d_n7, assign23020_e32272_d_n10, assign23020_e32272_d_n11, assign23020_e32272_d_n12, assign23020_e32272_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        let assign23020_e32268: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign23020_e32269: f64 = (0.5 * assign23020_e32268);
        let assign23020_e32270: f64 = (p.p47 - assign23020_e32269);
        (assign23020_e32270, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (-(0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_qhs, locals.var_qhs_dn0, locals.var_qhs_dn2, locals.var_qhs_dn6, locals.var_qhs_dn7, locals.var_qhs_dn10, locals.var_qhs_dn11, locals.var_qhs_dn12, locals.var_qhs_dn17,)
    }
};
        locals.var_qhs = assign23020_e32272;
        locals.var_qhs_dn0 = assign23020_e32272_d_n0;
        locals.var_qhs_dn2 = assign23020_e32272_d_n2;
        locals.var_qhs_dn6 = assign23020_e32272_d_n6;
        locals.var_qhs_dn7 = assign23020_e32272_d_n7;
        locals.var_qhs_dn10 = assign23020_e32272_d_n10;
        locals.var_qhs_dn11 = assign23020_e32272_d_n11;
        locals.var_qhs_dn12 = assign23020_e32272_d_n12;
        locals.var_qhs_dn17 = assign23020_e32272_d_n17;
        locals.var_qhs_rv = 0.0;

        let (assign23070_e32307, assign23070_e32307_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        let assign23070_e32303: f64 = (1e-9 / 0.0001);
        let assign23070_e32305: f64 = (assign23070_e32303 * (nv17 - 0.0));
        (assign23070_e32305, assign23070_e32303,)
    } else {
        (locals.var_qhs_hist, locals.var_qhs_hist_dn17,)
    }
};
        locals.var_qhs_hist = assign23070_e32307;
        locals.var_qhs_hist_dn17 = assign23070_e32307_d_n17;
        locals.var_qhs_hist_rv = 0.0;

        let (assign23080_e32311, assign23080_e32311_d_n0, assign23080_e32311_d_n2, assign23080_e32311_d_n6, assign23080_e32311_d_n7, assign23080_e32311_d_n10, assign23080_e32311_d_n11, assign23080_e32311_d_n12, assign23080_e32311_d_n17,) = {
    if (locals.var_guard696 != 0.0) {
        (locals.var_qhs_hist, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_qhs_hist_dn17,)
    } else {
        (locals.var_qhs, locals.var_qhs_dn0, locals.var_qhs_dn2, locals.var_qhs_dn6, locals.var_qhs_dn7, locals.var_qhs_dn10, locals.var_qhs_dn11, locals.var_qhs_dn12, locals.var_qhs_dn17,)
    }
};
        locals.var_qhs = assign23080_e32311;
        locals.var_qhs_dn0 = assign23080_e32311_d_n0;
        locals.var_qhs_dn2 = assign23080_e32311_d_n2;
        locals.var_qhs_dn6 = assign23080_e32311_d_n6;
        locals.var_qhs_dn7 = assign23080_e32311_d_n7;
        locals.var_qhs_dn10 = assign23080_e32311_d_n10;
        locals.var_qhs_dn11 = assign23080_e32311_d_n11;
        locals.var_qhs_dn12 = assign23080_e32311_d_n12;
        locals.var_qhs_dn17 = assign23080_e32311_d_n17;
        locals.var_qhs_rv = 0.0;

        let assign23100_e32330: f64 = if (((locals.var_flg_noqi == 0.0) && (locals.var_isub > 0.0)) && (p.p146 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard718 = assign23100_e32330;
        locals.var_guard718_rv = 0.0;

        let assign23110_e32333: f64 = if locals.var_subversion < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard719 = assign23110_e32333;
        locals.var_guard719_rv = 0.0;

        let (assign23120_e32339, assign23120_e32339_d_n0, assign23120_e32339_d_n2, assign23120_e32339_d_n6, assign23120_e32339_d_n7, assign23120_e32339_d_n10, assign23120_e32339_d_n11, assign23120_e32339_d_n12, assign23120_e32339_d_n17,) = {
    if ((locals.var_guard718 != 0.0) && (locals.var_guard719 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs0, locals.var_vbs0_dn0, locals.var_vbs0_dn2, locals.var_vbs0_dn6, locals.var_vbs0_dn7, locals.var_vbs0_dn10, locals.var_vbs0_dn11, locals.var_vbs0_dn12, locals.var_vbs0_dn17,)
    }
};
        locals.var_vbs0 = assign23120_e32339;
        locals.var_vbs0_dn0 = assign23120_e32339_d_n0;
        locals.var_vbs0_dn2 = assign23120_e32339_d_n2;
        locals.var_vbs0_dn6 = assign23120_e32339_d_n6;
        locals.var_vbs0_dn7 = assign23120_e32339_d_n7;
        locals.var_vbs0_dn10 = assign23120_e32339_d_n10;
        locals.var_vbs0_dn11 = assign23120_e32339_d_n11;
        locals.var_vbs0_dn12 = assign23120_e32339_d_n12;
        locals.var_vbs0_dn17 = assign23120_e32339_d_n17;
        locals.var_vbs0_rv = 0.0;

        let (assign23130_e32345, assign23130_e32345_d_n0, assign23130_e32345_d_n2, assign23130_e32345_d_n6, assign23130_e32345_d_n7, assign23130_e32345_d_n10, assign23130_e32345_d_n11, assign23130_e32345_d_n12, assign23130_e32345_d_n17,) = {
    if ((locals.var_guard718 != 0.0) && (locals.var_guard719 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbsl, locals.var_vbsl_dn0, locals.var_vbsl_dn2, locals.var_vbsl_dn6, locals.var_vbsl_dn7, locals.var_vbsl_dn10, locals.var_vbsl_dn11, locals.var_vbsl_dn12, locals.var_vbsl_dn17,)
    }
};
        locals.var_vbsl = assign23130_e32345;
        locals.var_vbsl_dn0 = assign23130_e32345_d_n0;
        locals.var_vbsl_dn2 = assign23130_e32345_d_n2;
        locals.var_vbsl_dn6 = assign23130_e32345_d_n6;
        locals.var_vbsl_dn7 = assign23130_e32345_d_n7;
        locals.var_vbsl_dn10 = assign23130_e32345_d_n10;
        locals.var_vbsl_dn11 = assign23130_e32345_d_n11;
        locals.var_vbsl_dn12 = assign23130_e32345_d_n12;
        locals.var_vbsl_dn17 = assign23130_e32345_d_n17;
        locals.var_vbsl_rv = 0.0;

        let (assign23140_e32357, assign23140_e32357_d_n0, assign23140_e32357_d_n2, assign23140_e32357_d_n6, assign23140_e32357_d_n7, assign23140_e32357_d_n10, assign23140_e32357_d_n11, assign23140_e32357_d_n12, assign23140_e32357_d_n17,) = {
    if ((locals.var_guard718 != 0.0) && (locals.var_guard719 == 0.0)) {
        let (assign23140_e32355, assign23140_e32355_d_n0, assign23140_e32355_d_n2, assign23140_e32355_d_n6, assign23140_e32355_d_n7, assign23140_e32355_d_n10, assign23140_e32355_d_n11, assign23140_e32355_d_n12, assign23140_e32355_d_n17,) = {
            if (p.p43 == 1.0) {
                (locals.var_vbs, locals.var_vbs_dn0, locals.var_vbs_dn2, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10, locals.var_vbs_dn11, locals.var_vbs_dn12, locals.var_vbs_dn17,)
            } else {
                (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
            }
        };
        (assign23140_e32355, assign23140_e32355_d_n0, assign23140_e32355_d_n2, assign23140_e32355_d_n6, assign23140_e32355_d_n7, assign23140_e32355_d_n10, assign23140_e32355_d_n11, assign23140_e32355_d_n12, assign23140_e32355_d_n17,)
    } else {
        (locals.var_vbs0, locals.var_vbs0_dn0, locals.var_vbs0_dn2, locals.var_vbs0_dn6, locals.var_vbs0_dn7, locals.var_vbs0_dn10, locals.var_vbs0_dn11, locals.var_vbs0_dn12, locals.var_vbs0_dn17,)
    }
};
        locals.var_vbs0 = assign23140_e32357;
        locals.var_vbs0_dn0 = assign23140_e32357_d_n0;
        locals.var_vbs0_dn2 = assign23140_e32357_d_n2;
        locals.var_vbs0_dn6 = assign23140_e32357_d_n6;
        locals.var_vbs0_dn7 = assign23140_e32357_d_n7;
        locals.var_vbs0_dn10 = assign23140_e32357_d_n10;
        locals.var_vbs0_dn11 = assign23140_e32357_d_n11;
        locals.var_vbs0_dn12 = assign23140_e32357_d_n12;
        locals.var_vbs0_dn17 = assign23140_e32357_d_n17;
        locals.var_vbs0_rv = 0.0;

        let (assign23150_e32369, assign23150_e32369_d_n0, assign23150_e32369_d_n2, assign23150_e32369_d_n6, assign23150_e32369_d_n7, assign23150_e32369_d_n10, assign23150_e32369_d_n11, assign23150_e32369_d_n12, assign23150_e32369_d_n17,) = {
    if ((locals.var_guard718 != 0.0) && (locals.var_guard719 == 0.0)) {
        let (assign23150_e32367, assign23150_e32367_d_n0, assign23150_e32367_d_n2, assign23150_e32367_d_n6, assign23150_e32367_d_n7, assign23150_e32367_d_n10, assign23150_e32367_d_n11, assign23150_e32367_d_n12, assign23150_e32367_d_n17,) = {
            if (p.p43 == 1.0) {
                (locals.var_vbs, locals.var_vbs_dn0, locals.var_vbs_dn2, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10, locals.var_vbs_dn11, locals.var_vbs_dn12, locals.var_vbs_dn17,)
            } else {
                (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
            }
        };
        (assign23150_e32367, assign23150_e32367_d_n0, assign23150_e32367_d_n2, assign23150_e32367_d_n6, assign23150_e32367_d_n7, assign23150_e32367_d_n10, assign23150_e32367_d_n11, assign23150_e32367_d_n12, assign23150_e32367_d_n17,)
    } else {
        (locals.var_vbsl, locals.var_vbsl_dn0, locals.var_vbsl_dn2, locals.var_vbsl_dn6, locals.var_vbsl_dn7, locals.var_vbsl_dn10, locals.var_vbsl_dn11, locals.var_vbsl_dn12, locals.var_vbsl_dn17,)
    }
};
        locals.var_vbsl = assign23150_e32369;
        locals.var_vbsl_dn0 = assign23150_e32369_d_n0;
        locals.var_vbsl_dn2 = assign23150_e32369_d_n2;
        locals.var_vbsl_dn6 = assign23150_e32369_d_n6;
        locals.var_vbsl_dn7 = assign23150_e32369_d_n7;
        locals.var_vbsl_dn10 = assign23150_e32369_d_n10;
        locals.var_vbsl_dn11 = assign23150_e32369_d_n11;
        locals.var_vbsl_dn12 = assign23150_e32369_d_n12;
        locals.var_vbsl_dn17 = assign23150_e32369_d_n17;
        locals.var_vbsl_rv = 0.0;

        let (assign23160_e32377, assign23160_e32377_d_n0, assign23160_e32377_d_n2, assign23160_e32377_d_n6, assign23160_e32377_d_n7, assign23160_e32377_d_n10, assign23160_e32377_d_n11, assign23160_e32377_d_n12, assign23160_e32377_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23160_e32374: f64 = (p.p147 * locals.var_dvth);
        let assign23160_e32375: f64 = (1.0 + assign23160_e32374);
        (assign23160_e32375, (p.p147 * locals.var_dvth_dn0), (p.p147 * locals.var_dvth_dn2), (p.p147 * locals.var_dvth_dn6), (p.p147 * locals.var_dvth_dn7), (p.p147 * locals.var_dvth_dn10), (p.p147 * locals.var_dvth_dn11), (p.p147 * locals.var_dvth_dn12), (p.p147 * locals.var_dvth_dn17),)
    } else {
        (locals.var_t0__blk705, locals.var_t0__blk705_dn0, locals.var_t0__blk705_dn2, locals.var_t0__blk705_dn6, locals.var_t0__blk705_dn7, locals.var_t0__blk705_dn10, locals.var_t0__blk705_dn11, locals.var_t0__blk705_dn12, locals.var_t0__blk705_dn17,)
    }
};
        locals.var_t0__blk705 = assign23160_e32377;
        locals.var_t0__blk705_dn0 = assign23160_e32377_d_n0;
        locals.var_t0__blk705_dn2 = assign23160_e32377_d_n2;
        locals.var_t0__blk705_dn6 = assign23160_e32377_d_n6;
        locals.var_t0__blk705_dn7 = assign23160_e32377_d_n7;
        locals.var_t0__blk705_dn10 = assign23160_e32377_d_n10;
        locals.var_t0__blk705_dn11 = assign23160_e32377_d_n11;
        locals.var_t0__blk705_dn12 = assign23160_e32377_d_n12;
        locals.var_t0__blk705_dn17 = assign23160_e32377_d_n17;
        locals.var_t0__blk705_rv = 0.0;

        let (assign23170_e32385, assign23170_e32385_d_n0, assign23170_e32385_d_n2, assign23170_e32385_d_n6, assign23170_e32385_d_n7, assign23170_e32385_d_n10, assign23170_e32385_d_n11, assign23170_e32385_d_n12, assign23170_e32385_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23170_e32381: f64 = (p.p146 * locals.var_t0__blk705);
        let assign23170_e32383: f64 = (assign23170_e32381 * locals.var_isub);
        (assign23170_e32383, (((p.p146 * locals.var_t0__blk705_dn0) * locals.var_isub) + (assign23170_e32381 * locals.var_isub_dn0)), (((p.p146 * locals.var_t0__blk705_dn2) * locals.var_isub) + (assign23170_e32381 * locals.var_isub_dn2)), (((p.p146 * locals.var_t0__blk705_dn6) * locals.var_isub) + (assign23170_e32381 * locals.var_isub_dn6)), (((p.p146 * locals.var_t0__blk705_dn7) * locals.var_isub) + (assign23170_e32381 * locals.var_isub_dn7)), (((p.p146 * locals.var_t0__blk705_dn10) * locals.var_isub) + (assign23170_e32381 * locals.var_isub_dn10)), (((p.p146 * locals.var_t0__blk705_dn11) * locals.var_isub) + (assign23170_e32381 * locals.var_isub_dn11)), (((p.p146 * locals.var_t0__blk705_dn12) * locals.var_isub) + (assign23170_e32381 * locals.var_isub_dn12)), (((p.p146 * locals.var_t0__blk705_dn17) * locals.var_isub) + (assign23170_e32381 * locals.var_isub_dn17)),)
    } else {
        (locals.var_dvbsibpc, locals.var_dvbsibpc_dn0, locals.var_dvbsibpc_dn2, locals.var_dvbsibpc_dn6, locals.var_dvbsibpc_dn7, locals.var_dvbsibpc_dn10, locals.var_dvbsibpc_dn11, locals.var_dvbsibpc_dn12, locals.var_dvbsibpc_dn17,)
    }
};
        locals.var_dvbsibpc = assign23170_e32385;
        locals.var_dvbsibpc_dn0 = assign23170_e32385_d_n0;
        locals.var_dvbsibpc_dn2 = assign23170_e32385_d_n2;
        locals.var_dvbsibpc_dn6 = assign23170_e32385_d_n6;
        locals.var_dvbsibpc_dn7 = assign23170_e32385_d_n7;
        locals.var_dvbsibpc_dn10 = assign23170_e32385_d_n10;
        locals.var_dvbsibpc_dn11 = assign23170_e32385_d_n11;
        locals.var_dvbsibpc_dn12 = assign23170_e32385_d_n12;
        locals.var_dvbsibpc_dn17 = assign23170_e32385_d_n17;
        locals.var_dvbsibpc_rv = 0.0;

        let (assign23180_e32395, assign23180_e32395_d_n0, assign23180_e32395_d_n2, assign23180_e32395_d_n6, assign23180_e32395_d_n7, assign23180_e32395_d_n10, assign23180_e32395_d_n11, assign23180_e32395_d_n12, assign23180_e32395_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23180_e32390: f64 = (locals.var_ps0 - locals.var_vbs0);
        let assign23180_e32391: f64 = (locals.var_beta * assign23180_e32390);
        let assign23180_e32393: f64 = (assign23180_e32391 - 1.0);
        (assign23180_e32393, (locals.var_beta * (locals.var_ps0_dn0 - locals.var_vbs0_dn0)), (locals.var_beta * (locals.var_ps0_dn2 - locals.var_vbs0_dn2)), (locals.var_beta * (locals.var_ps0_dn6 - locals.var_vbs0_dn6)), (locals.var_beta * (locals.var_ps0_dn7 - locals.var_vbs0_dn7)), ((locals.var_beta_dn10 * assign23180_e32390) + (locals.var_beta * (locals.var_ps0_dn10 - locals.var_vbs0_dn10))), (locals.var_beta * (locals.var_ps0_dn11 - locals.var_vbs0_dn11)), (locals.var_beta * (locals.var_ps0_dn12 - locals.var_vbs0_dn12)), (locals.var_beta * (locals.var_ps0_dn17 - locals.var_vbs0_dn17)),)
    } else {
        (locals.var_xi0__blk707, locals.var_xi0__blk707_dn0, locals.var_xi0__blk707_dn2, locals.var_xi0__blk707_dn6, locals.var_xi0__blk707_dn7, locals.var_xi0__blk707_dn10, locals.var_xi0__blk707_dn11, locals.var_xi0__blk707_dn12, locals.var_xi0__blk707_dn17,)
    }
};
        locals.var_xi0__blk707 = assign23180_e32395;
        locals.var_xi0__blk707_dn0 = assign23180_e32395_d_n0;
        locals.var_xi0__blk707_dn2 = assign23180_e32395_d_n2;
        locals.var_xi0__blk707_dn6 = assign23180_e32395_d_n6;
        locals.var_xi0__blk707_dn7 = assign23180_e32395_d_n7;
        locals.var_xi0__blk707_dn10 = assign23180_e32395_d_n10;
        locals.var_xi0__blk707_dn11 = assign23180_e32395_d_n11;
        locals.var_xi0__blk707_dn12 = assign23180_e32395_d_n12;
        locals.var_xi0__blk707_dn17 = assign23180_e32395_d_n17;
        locals.var_xi0__blk707_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_83(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23190_e32408, assign23190_e32408_d_n0, assign23190_e32408_d_n2, assign23190_e32408_d_n6, assign23190_e32408_d_n7, assign23190_e32408_d_n10, assign23190_e32408_d_n11, assign23190_e32408_d_n12, assign23190_e32408_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23190_e32399: f64 = (locals.var_xi0__blk707 * locals.var_xi0__blk707);
        let assign23190_e32402: f64 = (4.0 * 0.1);
        let assign23190_e32404: f64 = (assign23190_e32402 * 0.1);
        let assign23190_e32405: f64 = (assign23190_e32399 + assign23190_e32404);
        let assign23190_e32406: f64 = (assign23190_e32405).sqrt();
        (assign23190_e32406, (((locals.var_xi0__blk707_dn0 * locals.var_xi0__blk707) + (locals.var_xi0__blk707 * locals.var_xi0__blk707_dn0)) / (2.0 * assign23190_e32406)), (((locals.var_xi0__blk707_dn2 * locals.var_xi0__blk707) + (locals.var_xi0__blk707 * locals.var_xi0__blk707_dn2)) / (2.0 * assign23190_e32406)), (((locals.var_xi0__blk707_dn6 * locals.var_xi0__blk707) + (locals.var_xi0__blk707 * locals.var_xi0__blk707_dn6)) / (2.0 * assign23190_e32406)), (((locals.var_xi0__blk707_dn7 * locals.var_xi0__blk707) + (locals.var_xi0__blk707 * locals.var_xi0__blk707_dn7)) / (2.0 * assign23190_e32406)), (((locals.var_xi0__blk707_dn10 * locals.var_xi0__blk707) + (locals.var_xi0__blk707 * locals.var_xi0__blk707_dn10)) / (2.0 * assign23190_e32406)), (((locals.var_xi0__blk707_dn11 * locals.var_xi0__blk707) + (locals.var_xi0__blk707 * locals.var_xi0__blk707_dn11)) / (2.0 * assign23190_e32406)), (((locals.var_xi0__blk707_dn12 * locals.var_xi0__blk707) + (locals.var_xi0__blk707 * locals.var_xi0__blk707_dn12)) / (2.0 * assign23190_e32406)), (((locals.var_xi0__blk707_dn17 * locals.var_xi0__blk707) + (locals.var_xi0__blk707 * locals.var_xi0__blk707_dn17)) / (2.0 * assign23190_e32406)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign23190_e32408;
        locals.var_tmf1_dn0 = assign23190_e32408_d_n0;
        locals.var_tmf1_dn2 = assign23190_e32408_d_n2;
        locals.var_tmf1_dn6 = assign23190_e32408_d_n6;
        locals.var_tmf1_dn7 = assign23190_e32408_d_n7;
        locals.var_tmf1_dn10 = assign23190_e32408_d_n10;
        locals.var_tmf1_dn11 = assign23190_e32408_d_n11;
        locals.var_tmf1_dn12 = assign23190_e32408_d_n12;
        locals.var_tmf1_dn17 = assign23190_e32408_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign23200_e32420, assign23200_e32420_d_n0, assign23200_e32420_d_n2, assign23200_e32420_d_n6, assign23200_e32420_d_n7, assign23200_e32420_d_n10, assign23200_e32420_d_n11, assign23200_e32420_d_n12, assign23200_e32420_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23200_e32413: f64 = (locals.var_xi0__blk707 + locals.var_tmf1);
        let assign23200_e32414: f64 = (0.5 * assign23200_e32413);
        let assign23200_e32417: f64 = (1e-10 * 0.1);
        let assign23200_e32418: f64 = (assign23200_e32414 + assign23200_e32417);
        (assign23200_e32418, (0.5 * (locals.var_xi0__blk707_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_xi0__blk707_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_xi0__blk707_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_xi0__blk707_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_xi0__blk707_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_xi0__blk707_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_xi0__blk707_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_xi0__blk707_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_xi0__blk707, locals.var_xi0__blk707_dn0, locals.var_xi0__blk707_dn2, locals.var_xi0__blk707_dn6, locals.var_xi0__blk707_dn7, locals.var_xi0__blk707_dn10, locals.var_xi0__blk707_dn11, locals.var_xi0__blk707_dn12, locals.var_xi0__blk707_dn17,)
    }
};
        locals.var_xi0__blk707 = assign23200_e32420;
        locals.var_xi0__blk707_dn0 = assign23200_e32420_d_n0;
        locals.var_xi0__blk707_dn2 = assign23200_e32420_d_n2;
        locals.var_xi0__blk707_dn6 = assign23200_e32420_d_n6;
        locals.var_xi0__blk707_dn7 = assign23200_e32420_d_n7;
        locals.var_xi0__blk707_dn10 = assign23200_e32420_d_n10;
        locals.var_xi0__blk707_dn11 = assign23200_e32420_d_n11;
        locals.var_xi0__blk707_dn12 = assign23200_e32420_d_n12;
        locals.var_xi0__blk707_dn17 = assign23200_e32420_d_n17;
        locals.var_xi0__blk707_rv = 0.0;

        let assign23210_e32423: f64 = if locals.var_xi0__blk707 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard720 = assign23210_e32423;
        locals.var_guard720_rv = 0.0;

        let (assign23220_e32429, assign23220_e32429_d_n0, assign23220_e32429_d_n2, assign23220_e32429_d_n6, assign23220_e32429_d_n7, assign23220_e32429_d_n10, assign23220_e32429_d_n11, assign23220_e32429_d_n12, assign23220_e32429_d_n17,) = {
    if ((locals.var_guard718 != 0.0) && (locals.var_guard720 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xi0__blk707, locals.var_xi0__blk707_dn0, locals.var_xi0__blk707_dn2, locals.var_xi0__blk707_dn6, locals.var_xi0__blk707_dn7, locals.var_xi0__blk707_dn10, locals.var_xi0__blk707_dn11, locals.var_xi0__blk707_dn12, locals.var_xi0__blk707_dn17,)
    }
};
        locals.var_xi0__blk707 = assign23220_e32429;
        locals.var_xi0__blk707_dn0 = assign23220_e32429_d_n0;
        locals.var_xi0__blk707_dn2 = assign23220_e32429_d_n2;
        locals.var_xi0__blk707_dn6 = assign23220_e32429_d_n6;
        locals.var_xi0__blk707_dn7 = assign23220_e32429_d_n7;
        locals.var_xi0__blk707_dn10 = assign23220_e32429_d_n10;
        locals.var_xi0__blk707_dn11 = assign23220_e32429_d_n11;
        locals.var_xi0__blk707_dn12 = assign23220_e32429_d_n12;
        locals.var_xi0__blk707_dn17 = assign23220_e32429_d_n17;
        locals.var_xi0__blk707_rv = 0.0;

        let (assign23230_e32434, assign23230_e32434_d_n0, assign23230_e32434_d_n2, assign23230_e32434_d_n6, assign23230_e32434_d_n7, assign23230_e32434_d_n10, assign23230_e32434_d_n11, assign23230_e32434_d_n12, assign23230_e32434_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23230_e32432: f64 = (locals.var_xi0__blk707).sqrt();
        (assign23230_e32432, (locals.var_xi0__blk707_dn0 / (2.0 * assign23230_e32432)), (locals.var_xi0__blk707_dn2 / (2.0 * assign23230_e32432)), (locals.var_xi0__blk707_dn6 / (2.0 * assign23230_e32432)), (locals.var_xi0__blk707_dn7 / (2.0 * assign23230_e32432)), (locals.var_xi0__blk707_dn10 / (2.0 * assign23230_e32432)), (locals.var_xi0__blk707_dn11 / (2.0 * assign23230_e32432)), (locals.var_xi0__blk707_dn12 / (2.0 * assign23230_e32432)), (locals.var_xi0__blk707_dn17 / (2.0 * assign23230_e32432)),)
    } else {
        (locals.var_xi0p12__blk708, locals.var_xi0p12__blk708_dn0, locals.var_xi0p12__blk708_dn2, locals.var_xi0p12__blk708_dn6, locals.var_xi0p12__blk708_dn7, locals.var_xi0p12__blk708_dn10, locals.var_xi0p12__blk708_dn11, locals.var_xi0p12__blk708_dn12, locals.var_xi0p12__blk708_dn17,)
    }
};
        locals.var_xi0p12__blk708 = assign23230_e32434;
        locals.var_xi0p12__blk708_dn0 = assign23230_e32434_d_n0;
        locals.var_xi0p12__blk708_dn2 = assign23230_e32434_d_n2;
        locals.var_xi0p12__blk708_dn6 = assign23230_e32434_d_n6;
        locals.var_xi0p12__blk708_dn7 = assign23230_e32434_d_n7;
        locals.var_xi0p12__blk708_dn10 = assign23230_e32434_d_n10;
        locals.var_xi0p12__blk708_dn11 = assign23230_e32434_d_n11;
        locals.var_xi0p12__blk708_dn12 = assign23230_e32434_d_n12;
        locals.var_xi0p12__blk708_dn17 = assign23230_e32434_d_n17;
        locals.var_xi0p12__blk708_rv = 0.0;

        let (assign23240_e32440, assign23240_e32440_d_n0, assign23240_e32440_d_n2, assign23240_e32440_d_n6, assign23240_e32440_d_n7, assign23240_e32440_d_n10, assign23240_e32440_d_n11, assign23240_e32440_d_n12, assign23240_e32440_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23240_e32438: f64 = (locals.var_xi0__blk707 * locals.var_xi0p12__blk708);
        (assign23240_e32438, ((locals.var_xi0__blk707_dn0 * locals.var_xi0p12__blk708) + (locals.var_xi0__blk707 * locals.var_xi0p12__blk708_dn0)), ((locals.var_xi0__blk707_dn2 * locals.var_xi0p12__blk708) + (locals.var_xi0__blk707 * locals.var_xi0p12__blk708_dn2)), ((locals.var_xi0__blk707_dn6 * locals.var_xi0p12__blk708) + (locals.var_xi0__blk707 * locals.var_xi0p12__blk708_dn6)), ((locals.var_xi0__blk707_dn7 * locals.var_xi0p12__blk708) + (locals.var_xi0__blk707 * locals.var_xi0p12__blk708_dn7)), ((locals.var_xi0__blk707_dn10 * locals.var_xi0p12__blk708) + (locals.var_xi0__blk707 * locals.var_xi0p12__blk708_dn10)), ((locals.var_xi0__blk707_dn11 * locals.var_xi0p12__blk708) + (locals.var_xi0__blk707 * locals.var_xi0p12__blk708_dn11)), ((locals.var_xi0__blk707_dn12 * locals.var_xi0p12__blk708) + (locals.var_xi0__blk707 * locals.var_xi0p12__blk708_dn12)), ((locals.var_xi0__blk707_dn17 * locals.var_xi0p12__blk708) + (locals.var_xi0__blk707 * locals.var_xi0p12__blk708_dn17)),)
    } else {
        (locals.var_xi0p32, locals.var_xi0p32_dn0, locals.var_xi0p32_dn2, locals.var_xi0p32_dn6, locals.var_xi0p32_dn7, locals.var_xi0p32_dn10, locals.var_xi0p32_dn11, locals.var_xi0p32_dn12, locals.var_xi0p32_dn17,)
    }
};
        locals.var_xi0p32 = assign23240_e32440;
        locals.var_xi0p32_dn0 = assign23240_e32440_d_n0;
        locals.var_xi0p32_dn2 = assign23240_e32440_d_n2;
        locals.var_xi0p32_dn6 = assign23240_e32440_d_n6;
        locals.var_xi0p32_dn7 = assign23240_e32440_d_n7;
        locals.var_xi0p32_dn10 = assign23240_e32440_d_n10;
        locals.var_xi0p32_dn11 = assign23240_e32440_d_n11;
        locals.var_xi0p32_dn12 = assign23240_e32440_d_n12;
        locals.var_xi0p32_dn17 = assign23240_e32440_d_n17;
        locals.var_xi0p32_rv = 0.0;

        let (assign23250_e32450, assign23250_e32450_d_n0, assign23250_e32450_d_n2, assign23250_e32450_d_n6, assign23250_e32450_d_n7, assign23250_e32450_d_n10, assign23250_e32450_d_n11, assign23250_e32450_d_n12, assign23250_e32450_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23250_e32445: f64 = (locals.var_psl - locals.var_vbsl);
        let assign23250_e32446: f64 = (locals.var_beta * assign23250_e32445);
        let assign23250_e32448: f64 = (assign23250_e32446 - 1.0);
        (assign23250_e32448, (locals.var_beta * (locals.var_psl_dn0 - locals.var_vbsl_dn0)), (locals.var_beta * (locals.var_psl_dn2 - locals.var_vbsl_dn2)), (locals.var_beta * (locals.var_psl_dn6 - locals.var_vbsl_dn6)), (locals.var_beta * (locals.var_psl_dn7 - locals.var_vbsl_dn7)), ((locals.var_beta_dn10 * assign23250_e32445) + (locals.var_beta * (locals.var_psl_dn10 - locals.var_vbsl_dn10))), (locals.var_beta * (locals.var_psl_dn11 - locals.var_vbsl_dn11)), (locals.var_beta * (locals.var_psl_dn12 - locals.var_vbsl_dn12)), (locals.var_beta * (locals.var_psl_dn17 - locals.var_vbsl_dn17)),)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn6, locals.var_xil_dn7, locals.var_xil_dn10, locals.var_xil_dn11, locals.var_xil_dn12, locals.var_xil_dn17,)
    }
};
        locals.var_xil = assign23250_e32450;
        locals.var_xil_dn0 = assign23250_e32450_d_n0;
        locals.var_xil_dn2 = assign23250_e32450_d_n2;
        locals.var_xil_dn6 = assign23250_e32450_d_n6;
        locals.var_xil_dn7 = assign23250_e32450_d_n7;
        locals.var_xil_dn10 = assign23250_e32450_d_n10;
        locals.var_xil_dn11 = assign23250_e32450_d_n11;
        locals.var_xil_dn12 = assign23250_e32450_d_n12;
        locals.var_xil_dn17 = assign23250_e32450_d_n17;
        locals.var_xil_rv = 0.0;

        let (assign23260_e32463, assign23260_e32463_d_n0, assign23260_e32463_d_n2, assign23260_e32463_d_n6, assign23260_e32463_d_n7, assign23260_e32463_d_n10, assign23260_e32463_d_n11, assign23260_e32463_d_n12, assign23260_e32463_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23260_e32454: f64 = (locals.var_xil * locals.var_xil);
        let assign23260_e32457: f64 = (4.0 * 0.1);
        let assign23260_e32459: f64 = (assign23260_e32457 * 0.1);
        let assign23260_e32460: f64 = (assign23260_e32454 + assign23260_e32459);
        let assign23260_e32461: f64 = (assign23260_e32460).sqrt();
        (assign23260_e32461, (((locals.var_xil_dn0 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn0)) / (2.0 * assign23260_e32461)), (((locals.var_xil_dn2 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn2)) / (2.0 * assign23260_e32461)), (((locals.var_xil_dn6 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn6)) / (2.0 * assign23260_e32461)), (((locals.var_xil_dn7 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn7)) / (2.0 * assign23260_e32461)), (((locals.var_xil_dn10 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn10)) / (2.0 * assign23260_e32461)), (((locals.var_xil_dn11 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn11)) / (2.0 * assign23260_e32461)), (((locals.var_xil_dn12 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn12)) / (2.0 * assign23260_e32461)), (((locals.var_xil_dn17 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn17)) / (2.0 * assign23260_e32461)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign23260_e32463;
        locals.var_tmf1_dn0 = assign23260_e32463_d_n0;
        locals.var_tmf1_dn2 = assign23260_e32463_d_n2;
        locals.var_tmf1_dn6 = assign23260_e32463_d_n6;
        locals.var_tmf1_dn7 = assign23260_e32463_d_n7;
        locals.var_tmf1_dn10 = assign23260_e32463_d_n10;
        locals.var_tmf1_dn11 = assign23260_e32463_d_n11;
        locals.var_tmf1_dn12 = assign23260_e32463_d_n12;
        locals.var_tmf1_dn17 = assign23260_e32463_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign23270_e32475, assign23270_e32475_d_n0, assign23270_e32475_d_n2, assign23270_e32475_d_n6, assign23270_e32475_d_n7, assign23270_e32475_d_n10, assign23270_e32475_d_n11, assign23270_e32475_d_n12, assign23270_e32475_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23270_e32468: f64 = (locals.var_xil + locals.var_tmf1);
        let assign23270_e32469: f64 = (0.5 * assign23270_e32468);
        let assign23270_e32472: f64 = (1e-10 * 0.1);
        let assign23270_e32473: f64 = (assign23270_e32469 + assign23270_e32472);
        (assign23270_e32473, (0.5 * (locals.var_xil_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_xil_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_xil_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_xil_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_xil_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_xil_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_xil_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_xil_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn6, locals.var_xil_dn7, locals.var_xil_dn10, locals.var_xil_dn11, locals.var_xil_dn12, locals.var_xil_dn17,)
    }
};
        locals.var_xil = assign23270_e32475;
        locals.var_xil_dn0 = assign23270_e32475_d_n0;
        locals.var_xil_dn2 = assign23270_e32475_d_n2;
        locals.var_xil_dn6 = assign23270_e32475_d_n6;
        locals.var_xil_dn7 = assign23270_e32475_d_n7;
        locals.var_xil_dn10 = assign23270_e32475_d_n10;
        locals.var_xil_dn11 = assign23270_e32475_d_n11;
        locals.var_xil_dn12 = assign23270_e32475_d_n12;
        locals.var_xil_dn17 = assign23270_e32475_d_n17;
        locals.var_xil_rv = 0.0;

        let assign23280_e32478: f64 = if locals.var_xil < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard721 = assign23280_e32478;
        locals.var_guard721_rv = 0.0;

        let (assign23290_e32484, assign23290_e32484_d_n0, assign23290_e32484_d_n2, assign23290_e32484_d_n6, assign23290_e32484_d_n7, assign23290_e32484_d_n10, assign23290_e32484_d_n11, assign23290_e32484_d_n12, assign23290_e32484_d_n17,) = {
    if ((locals.var_guard718 != 0.0) && (locals.var_guard721 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn6, locals.var_xil_dn7, locals.var_xil_dn10, locals.var_xil_dn11, locals.var_xil_dn12, locals.var_xil_dn17,)
    }
};
        locals.var_xil = assign23290_e32484;
        locals.var_xil_dn0 = assign23290_e32484_d_n0;
        locals.var_xil_dn2 = assign23290_e32484_d_n2;
        locals.var_xil_dn6 = assign23290_e32484_d_n6;
        locals.var_xil_dn7 = assign23290_e32484_d_n7;
        locals.var_xil_dn10 = assign23290_e32484_d_n10;
        locals.var_xil_dn11 = assign23290_e32484_d_n11;
        locals.var_xil_dn12 = assign23290_e32484_d_n12;
        locals.var_xil_dn17 = assign23290_e32484_d_n17;
        locals.var_xil_rv = 0.0;

        let (assign23300_e32489, assign23300_e32489_d_n0, assign23300_e32489_d_n2, assign23300_e32489_d_n6, assign23300_e32489_d_n7, assign23300_e32489_d_n10, assign23300_e32489_d_n11, assign23300_e32489_d_n12, assign23300_e32489_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23300_e32487: f64 = (locals.var_xil).sqrt();
        (assign23300_e32487, (locals.var_xil_dn0 / (2.0 * assign23300_e32487)), (locals.var_xil_dn2 / (2.0 * assign23300_e32487)), (locals.var_xil_dn6 / (2.0 * assign23300_e32487)), (locals.var_xil_dn7 / (2.0 * assign23300_e32487)), (locals.var_xil_dn10 / (2.0 * assign23300_e32487)), (locals.var_xil_dn11 / (2.0 * assign23300_e32487)), (locals.var_xil_dn12 / (2.0 * assign23300_e32487)), (locals.var_xil_dn17 / (2.0 * assign23300_e32487)),)
    } else {
        (locals.var_xilp12__blk711, locals.var_xilp12__blk711_dn0, locals.var_xilp12__blk711_dn2, locals.var_xilp12__blk711_dn6, locals.var_xilp12__blk711_dn7, locals.var_xilp12__blk711_dn10, locals.var_xilp12__blk711_dn11, locals.var_xilp12__blk711_dn12, locals.var_xilp12__blk711_dn17,)
    }
};
        locals.var_xilp12__blk711 = assign23300_e32489;
        locals.var_xilp12__blk711_dn0 = assign23300_e32489_d_n0;
        locals.var_xilp12__blk711_dn2 = assign23300_e32489_d_n2;
        locals.var_xilp12__blk711_dn6 = assign23300_e32489_d_n6;
        locals.var_xilp12__blk711_dn7 = assign23300_e32489_d_n7;
        locals.var_xilp12__blk711_dn10 = assign23300_e32489_d_n10;
        locals.var_xilp12__blk711_dn11 = assign23300_e32489_d_n11;
        locals.var_xilp12__blk711_dn12 = assign23300_e32489_d_n12;
        locals.var_xilp12__blk711_dn17 = assign23300_e32489_d_n17;
        locals.var_xilp12__blk711_rv = 0.0;

        let (assign23310_e32495, assign23310_e32495_d_n0, assign23310_e32495_d_n2, assign23310_e32495_d_n6, assign23310_e32495_d_n7, assign23310_e32495_d_n10, assign23310_e32495_d_n11, assign23310_e32495_d_n12, assign23310_e32495_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23310_e32493: f64 = (locals.var_xil * locals.var_xilp12__blk711);
        (assign23310_e32493, ((locals.var_xil_dn0 * locals.var_xilp12__blk711) + (locals.var_xil * locals.var_xilp12__blk711_dn0)), ((locals.var_xil_dn2 * locals.var_xilp12__blk711) + (locals.var_xil * locals.var_xilp12__blk711_dn2)), ((locals.var_xil_dn6 * locals.var_xilp12__blk711) + (locals.var_xil * locals.var_xilp12__blk711_dn6)), ((locals.var_xil_dn7 * locals.var_xilp12__blk711) + (locals.var_xil * locals.var_xilp12__blk711_dn7)), ((locals.var_xil_dn10 * locals.var_xilp12__blk711) + (locals.var_xil * locals.var_xilp12__blk711_dn10)), ((locals.var_xil_dn11 * locals.var_xilp12__blk711) + (locals.var_xil * locals.var_xilp12__blk711_dn11)), ((locals.var_xil_dn12 * locals.var_xilp12__blk711) + (locals.var_xil * locals.var_xilp12__blk711_dn12)), ((locals.var_xil_dn17 * locals.var_xilp12__blk711) + (locals.var_xil * locals.var_xilp12__blk711_dn17)),)
    } else {
        (locals.var_xilp32, locals.var_xilp32_dn0, locals.var_xilp32_dn2, locals.var_xilp32_dn6, locals.var_xilp32_dn7, locals.var_xilp32_dn10, locals.var_xilp32_dn11, locals.var_xilp32_dn12, locals.var_xilp32_dn17,)
    }
};
        locals.var_xilp32 = assign23310_e32495;
        locals.var_xilp32_dn0 = assign23310_e32495_d_n0;
        locals.var_xilp32_dn2 = assign23310_e32495_d_n2;
        locals.var_xilp32_dn6 = assign23310_e32495_d_n6;
        locals.var_xilp32_dn7 = assign23310_e32495_d_n7;
        locals.var_xilp32_dn10 = assign23310_e32495_d_n10;
        locals.var_xilp32_dn11 = assign23310_e32495_d_n11;
        locals.var_xilp32_dn12 = assign23310_e32495_d_n12;
        locals.var_xilp32_dn17 = assign23310_e32495_d_n17;
        locals.var_xilp32_rv = 0.0;

        let (assign23320_e32501, assign23320_e32501_d_n0, assign23320_e32501_d_n2, assign23320_e32501_d_n6, assign23320_e32501_d_n7, assign23320_e32501_d_n10, assign23320_e32501_d_n11, assign23320_e32501_d_n12, assign23320_e32501_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23320_e32499: f64 = (1.0 / locals.var_xi0__blk707);
        (assign23320_e32499, (-(locals.var_xi0__blk707_dn0 / (locals.var_xi0__blk707 * locals.var_xi0__blk707))), (-(locals.var_xi0__blk707_dn2 / (locals.var_xi0__blk707 * locals.var_xi0__blk707))), (-(locals.var_xi0__blk707_dn6 / (locals.var_xi0__blk707 * locals.var_xi0__blk707))), (-(locals.var_xi0__blk707_dn7 / (locals.var_xi0__blk707 * locals.var_xi0__blk707))), (-(locals.var_xi0__blk707_dn10 / (locals.var_xi0__blk707 * locals.var_xi0__blk707))), (-(locals.var_xi0__blk707_dn11 / (locals.var_xi0__blk707 * locals.var_xi0__blk707))), (-(locals.var_xi0__blk707_dn12 / (locals.var_xi0__blk707 * locals.var_xi0__blk707))), (-(locals.var_xi0__blk707_dn17 / (locals.var_xi0__blk707 * locals.var_xi0__blk707))),)
    } else {
        (locals.var_t10__blk713, locals.var_t10__blk713_dn0, locals.var_t10__blk713_dn2, locals.var_t10__blk713_dn6, locals.var_t10__blk713_dn7, locals.var_t10__blk713_dn10, locals.var_t10__blk713_dn11, locals.var_t10__blk713_dn12, locals.var_t10__blk713_dn17,)
    }
};
        locals.var_t10__blk713 = assign23320_e32501;
        locals.var_t10__blk713_dn0 = assign23320_e32501_d_n0;
        locals.var_t10__blk713_dn2 = assign23320_e32501_d_n2;
        locals.var_t10__blk713_dn6 = assign23320_e32501_d_n6;
        locals.var_t10__blk713_dn7 = assign23320_e32501_d_n7;
        locals.var_t10__blk713_dn10 = assign23320_e32501_d_n10;
        locals.var_t10__blk713_dn11 = assign23320_e32501_d_n11;
        locals.var_t10__blk713_dn12 = assign23320_e32501_d_n12;
        locals.var_t10__blk713_dn17 = assign23320_e32501_d_n17;
        locals.var_t10__blk713_rv = 0.0;

        let (assign23330_e32509, assign23330_e32509_d_n0, assign23330_e32509_d_n2, assign23330_e32509_d_n6, assign23330_e32509_d_n7, assign23330_e32509_d_n10, assign23330_e32509_d_n11, assign23330_e32509_d_n12, assign23330_e32509_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23330_e32505: f64 = (locals.var_beta * locals.var_dvbsibpc);
        let assign23330_e32507: f64 = (assign23330_e32505 * locals.var_t10__blk713);
        (assign23330_e32507, (((locals.var_beta * locals.var_dvbsibpc_dn0) * locals.var_t10__blk713) + (assign23330_e32505 * locals.var_t10__blk713_dn0)), (((locals.var_beta * locals.var_dvbsibpc_dn2) * locals.var_t10__blk713) + (assign23330_e32505 * locals.var_t10__blk713_dn2)), (((locals.var_beta * locals.var_dvbsibpc_dn6) * locals.var_t10__blk713) + (assign23330_e32505 * locals.var_t10__blk713_dn6)), (((locals.var_beta * locals.var_dvbsibpc_dn7) * locals.var_t10__blk713) + (assign23330_e32505 * locals.var_t10__blk713_dn7)), ((((locals.var_beta_dn10 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn10)) * locals.var_t10__blk713) + (assign23330_e32505 * locals.var_t10__blk713_dn10)), (((locals.var_beta * locals.var_dvbsibpc_dn11) * locals.var_t10__blk713) + (assign23330_e32505 * locals.var_t10__blk713_dn11)), (((locals.var_beta * locals.var_dvbsibpc_dn12) * locals.var_t10__blk713) + (assign23330_e32505 * locals.var_t10__blk713_dn12)), (((locals.var_beta * locals.var_dvbsibpc_dn17) * locals.var_t10__blk713) + (assign23330_e32505 * locals.var_t10__blk713_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign23330_e32509;
        locals.var_t1_dn0 = assign23330_e32509_d_n0;
        locals.var_t1_dn2 = assign23330_e32509_d_n2;
        locals.var_t1_dn6 = assign23330_e32509_d_n6;
        locals.var_t1_dn7 = assign23330_e32509_d_n7;
        locals.var_t1_dn10 = assign23330_e32509_d_n10;
        locals.var_t1_dn11 = assign23330_e32509_d_n11;
        locals.var_t1_dn12 = assign23330_e32509_d_n12;
        locals.var_t1_dn17 = assign23330_e32509_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign23340_e32515, assign23340_e32515_d_n0, assign23340_e32515_d_n2, assign23340_e32515_d_n6, assign23340_e32515_d_n7, assign23340_e32515_d_n10, assign23340_e32515_d_n11, assign23340_e32515_d_n12, assign23340_e32515_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23340_e32513: f64 = (1.0 / locals.var_xil);
        (assign23340_e32513, (-(locals.var_xil_dn0 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn2 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn6 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn7 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn10 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn11 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn12 / (locals.var_xil * locals.var_xil))), (-(locals.var_xil_dn17 / (locals.var_xil * locals.var_xil))),)
    } else {
        (locals.var_t10__blk713, locals.var_t10__blk713_dn0, locals.var_t10__blk713_dn2, locals.var_t10__blk713_dn6, locals.var_t10__blk713_dn7, locals.var_t10__blk713_dn10, locals.var_t10__blk713_dn11, locals.var_t10__blk713_dn12, locals.var_t10__blk713_dn17,)
    }
};
        locals.var_t10__blk713 = assign23340_e32515;
        locals.var_t10__blk713_dn0 = assign23340_e32515_d_n0;
        locals.var_t10__blk713_dn2 = assign23340_e32515_d_n2;
        locals.var_t10__blk713_dn6 = assign23340_e32515_d_n6;
        locals.var_t10__blk713_dn7 = assign23340_e32515_d_n7;
        locals.var_t10__blk713_dn10 = assign23340_e32515_d_n10;
        locals.var_t10__blk713_dn11 = assign23340_e32515_d_n11;
        locals.var_t10__blk713_dn12 = assign23340_e32515_d_n12;
        locals.var_t10__blk713_dn17 = assign23340_e32515_d_n17;
        locals.var_t10__blk713_rv = 0.0;

        let (assign23350_e32523, assign23350_e32523_d_n0, assign23350_e32523_d_n2, assign23350_e32523_d_n6, assign23350_e32523_d_n7, assign23350_e32523_d_n10, assign23350_e32523_d_n11, assign23350_e32523_d_n12, assign23350_e32523_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23350_e32519: f64 = (locals.var_beta * locals.var_dvbsibpc);
        let assign23350_e32521: f64 = (assign23350_e32519 * locals.var_t10__blk713);
        (assign23350_e32521, (((locals.var_beta * locals.var_dvbsibpc_dn0) * locals.var_t10__blk713) + (assign23350_e32519 * locals.var_t10__blk713_dn0)), (((locals.var_beta * locals.var_dvbsibpc_dn2) * locals.var_t10__blk713) + (assign23350_e32519 * locals.var_t10__blk713_dn2)), (((locals.var_beta * locals.var_dvbsibpc_dn6) * locals.var_t10__blk713) + (assign23350_e32519 * locals.var_t10__blk713_dn6)), (((locals.var_beta * locals.var_dvbsibpc_dn7) * locals.var_t10__blk713) + (assign23350_e32519 * locals.var_t10__blk713_dn7)), ((((locals.var_beta_dn10 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn10)) * locals.var_t10__blk713) + (assign23350_e32519 * locals.var_t10__blk713_dn10)), (((locals.var_beta * locals.var_dvbsibpc_dn11) * locals.var_t10__blk713) + (assign23350_e32519 * locals.var_t10__blk713_dn11)), (((locals.var_beta * locals.var_dvbsibpc_dn12) * locals.var_t10__blk713) + (assign23350_e32519 * locals.var_t10__blk713_dn12)), (((locals.var_beta * locals.var_dvbsibpc_dn17) * locals.var_t10__blk713) + (assign23350_e32519 * locals.var_t10__blk713_dn17)),)
    } else {
        (locals.var_t2__blk714, locals.var_t2__blk714_dn0, locals.var_t2__blk714_dn2, locals.var_t2__blk714_dn6, locals.var_t2__blk714_dn7, locals.var_t2__blk714_dn10, locals.var_t2__blk714_dn11, locals.var_t2__blk714_dn12, locals.var_t2__blk714_dn17,)
    }
};
        locals.var_t2__blk714 = assign23350_e32523;
        locals.var_t2__blk714_dn0 = assign23350_e32523_d_n0;
        locals.var_t2__blk714_dn2 = assign23350_e32523_d_n2;
        locals.var_t2__blk714_dn6 = assign23350_e32523_d_n6;
        locals.var_t2__blk714_dn7 = assign23350_e32523_d_n7;
        locals.var_t2__blk714_dn10 = assign23350_e32523_d_n10;
        locals.var_t2__blk714_dn11 = assign23350_e32523_d_n11;
        locals.var_t2__blk714_dn12 = assign23350_e32523_d_n12;
        locals.var_t2__blk714_dn17 = assign23350_e32523_d_n17;
        locals.var_t2__blk714_rv = 0.0;

        let (assign23360_e32535, assign23360_e32535_d_n0, assign23360_e32535_d_n2, assign23360_e32535_d_n6, assign23360_e32535_d_n7, assign23360_e32535_d_n10, assign23360_e32535_d_n11, assign23360_e32535_d_n12, assign23360_e32535_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23360_e32528: f64 = (locals.var_xilp32 * locals.var_t2__blk714);
        let assign23360_e32531: f64 = (locals.var_xi0p32 * locals.var_t1);
        let assign23360_e32532: f64 = (assign23360_e32528 - assign23360_e32531);
        let assign23360_e32533: f64 = (locals.var_cnst0soi * assign23360_e32532);
        (assign23360_e32533, ((locals.var_cnst0soi_dn0 * assign23360_e32532) + (locals.var_cnst0soi * (((locals.var_xilp32_dn0 * locals.var_t2__blk714) + (locals.var_xilp32 * locals.var_t2__blk714_dn0)) - ((locals.var_xi0p32_dn0 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn0))))), ((locals.var_cnst0soi_dn2 * assign23360_e32532) + (locals.var_cnst0soi * (((locals.var_xilp32_dn2 * locals.var_t2__blk714) + (locals.var_xilp32 * locals.var_t2__blk714_dn2)) - ((locals.var_xi0p32_dn2 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn2))))), ((locals.var_cnst0soi_dn6 * assign23360_e32532) + (locals.var_cnst0soi * (((locals.var_xilp32_dn6 * locals.var_t2__blk714) + (locals.var_xilp32 * locals.var_t2__blk714_dn6)) - ((locals.var_xi0p32_dn6 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn6))))), ((locals.var_cnst0soi_dn7 * assign23360_e32532) + (locals.var_cnst0soi * (((locals.var_xilp32_dn7 * locals.var_t2__blk714) + (locals.var_xilp32 * locals.var_t2__blk714_dn7)) - ((locals.var_xi0p32_dn7 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn7))))), ((locals.var_cnst0soi_dn10 * assign23360_e32532) + (locals.var_cnst0soi * (((locals.var_xilp32_dn10 * locals.var_t2__blk714) + (locals.var_xilp32 * locals.var_t2__blk714_dn10)) - ((locals.var_xi0p32_dn10 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn10))))), ((locals.var_cnst0soi_dn11 * assign23360_e32532) + (locals.var_cnst0soi * (((locals.var_xilp32_dn11 * locals.var_t2__blk714) + (locals.var_xilp32 * locals.var_t2__blk714_dn11)) - ((locals.var_xi0p32_dn11 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn11))))), ((locals.var_cnst0soi_dn12 * assign23360_e32532) + (locals.var_cnst0soi * (((locals.var_xilp32_dn12 * locals.var_t2__blk714) + (locals.var_xilp32 * locals.var_t2__blk714_dn12)) - ((locals.var_xi0p32_dn12 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn12))))), ((locals.var_cnst0soi_dn17 * assign23360_e32532) + (locals.var_cnst0soi * (((locals.var_xilp32_dn17 * locals.var_t2__blk714) + (locals.var_xilp32 * locals.var_t2__blk714_dn17)) - ((locals.var_xi0p32_dn17 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn17))))),)
    } else {
        (locals.var_dg3, locals.var_dg3_dn0, locals.var_dg3_dn2, locals.var_dg3_dn6, locals.var_dg3_dn7, locals.var_dg3_dn10, locals.var_dg3_dn11, locals.var_dg3_dn12, locals.var_dg3_dn17,)
    }
};
        locals.var_dg3 = assign23360_e32535;
        locals.var_dg3_dn0 = assign23360_e32535_d_n0;
        locals.var_dg3_dn2 = assign23360_e32535_d_n2;
        locals.var_dg3_dn6 = assign23360_e32535_d_n6;
        locals.var_dg3_dn7 = assign23360_e32535_d_n7;
        locals.var_dg3_dn10 = assign23360_e32535_d_n10;
        locals.var_dg3_dn11 = assign23360_e32535_d_n11;
        locals.var_dg3_dn12 = assign23360_e32535_d_n12;
        locals.var_dg3_dn17 = assign23360_e32535_d_n17;
        locals.var_dg3_rv = 0.0;

        let (assign23370_e32550, assign23370_e32550_d_n0, assign23370_e32550_d_n2, assign23370_e32550_d_n6, assign23370_e32550_d_n7, assign23370_e32550_d_n10, assign23370_e32550_d_n11, assign23370_e32550_d_n12, assign23370_e32550_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23370_e32539: f64 = (locals.var_cnst0soi * 0.5);
        let assign23370_e32541: f64 = (-locals.var_xilp12__blk711);
        let assign23370_e32543: f64 = (assign23370_e32541 * locals.var_t2__blk714);
        let assign23370_e32546: f64 = (locals.var_xi0p12__blk708 * locals.var_t1);
        let assign23370_e32547: f64 = (assign23370_e32543 + assign23370_e32546);
        let assign23370_e32548: f64 = (assign23370_e32539 * assign23370_e32547);
        (assign23370_e32548, (((locals.var_cnst0soi_dn0 * 0.5) * assign23370_e32547) + (assign23370_e32539 * ((((-locals.var_xilp12__blk711_dn0) * locals.var_t2__blk714) + (assign23370_e32541 * locals.var_t2__blk714_dn0)) + ((locals.var_xi0p12__blk708_dn0 * locals.var_t1) + (locals.var_xi0p12__blk708 * locals.var_t1_dn0))))), (((locals.var_cnst0soi_dn2 * 0.5) * assign23370_e32547) + (assign23370_e32539 * ((((-locals.var_xilp12__blk711_dn2) * locals.var_t2__blk714) + (assign23370_e32541 * locals.var_t2__blk714_dn2)) + ((locals.var_xi0p12__blk708_dn2 * locals.var_t1) + (locals.var_xi0p12__blk708 * locals.var_t1_dn2))))), (((locals.var_cnst0soi_dn6 * 0.5) * assign23370_e32547) + (assign23370_e32539 * ((((-locals.var_xilp12__blk711_dn6) * locals.var_t2__blk714) + (assign23370_e32541 * locals.var_t2__blk714_dn6)) + ((locals.var_xi0p12__blk708_dn6 * locals.var_t1) + (locals.var_xi0p12__blk708 * locals.var_t1_dn6))))), (((locals.var_cnst0soi_dn7 * 0.5) * assign23370_e32547) + (assign23370_e32539 * ((((-locals.var_xilp12__blk711_dn7) * locals.var_t2__blk714) + (assign23370_e32541 * locals.var_t2__blk714_dn7)) + ((locals.var_xi0p12__blk708_dn7 * locals.var_t1) + (locals.var_xi0p12__blk708 * locals.var_t1_dn7))))), (((locals.var_cnst0soi_dn10 * 0.5) * assign23370_e32547) + (assign23370_e32539 * ((((-locals.var_xilp12__blk711_dn10) * locals.var_t2__blk714) + (assign23370_e32541 * locals.var_t2__blk714_dn10)) + ((locals.var_xi0p12__blk708_dn10 * locals.var_t1) + (locals.var_xi0p12__blk708 * locals.var_t1_dn10))))), (((locals.var_cnst0soi_dn11 * 0.5) * assign23370_e32547) + (assign23370_e32539 * ((((-locals.var_xilp12__blk711_dn11) * locals.var_t2__blk714) + (assign23370_e32541 * locals.var_t2__blk714_dn11)) + ((locals.var_xi0p12__blk708_dn11 * locals.var_t1) + (locals.var_xi0p12__blk708 * locals.var_t1_dn11))))), (((locals.var_cnst0soi_dn12 * 0.5) * assign23370_e32547) + (assign23370_e32539 * ((((-locals.var_xilp12__blk711_dn12) * locals.var_t2__blk714) + (assign23370_e32541 * locals.var_t2__blk714_dn12)) + ((locals.var_xi0p12__blk708_dn12 * locals.var_t1) + (locals.var_xi0p12__blk708 * locals.var_t1_dn12))))), (((locals.var_cnst0soi_dn17 * 0.5) * assign23370_e32547) + (assign23370_e32539 * ((((-locals.var_xilp12__blk711_dn17) * locals.var_t2__blk714) + (assign23370_e32541 * locals.var_t2__blk714_dn17)) + ((locals.var_xi0p12__blk708_dn17 * locals.var_t1) + (locals.var_xi0p12__blk708 * locals.var_t1_dn17))))),)
    } else {
        (locals.var_dg4, locals.var_dg4_dn0, locals.var_dg4_dn2, locals.var_dg4_dn6, locals.var_dg4_dn7, locals.var_dg4_dn10, locals.var_dg4_dn11, locals.var_dg4_dn12, locals.var_dg4_dn17,)
    }
};
        locals.var_dg4 = assign23370_e32550;
        locals.var_dg4_dn0 = assign23370_e32550_d_n0;
        locals.var_dg4_dn2 = assign23370_e32550_d_n2;
        locals.var_dg4_dn6 = assign23370_e32550_d_n6;
        locals.var_dg4_dn7 = assign23370_e32550_d_n7;
        locals.var_dg4_dn10 = assign23370_e32550_d_n10;
        locals.var_dg4_dn11 = assign23370_e32550_d_n11;
        locals.var_dg4_dn12 = assign23370_e32550_d_n12;
        locals.var_dg4_dn17 = assign23370_e32550_d_n17;
        locals.var_dg4_rv = 0.0;

        let (assign23380_e32556, assign23380_e32556_d_n0, assign23380_e32556_d_n2, assign23380_e32556_d_n6, assign23380_e32556_d_n7, assign23380_e32556_d_n10, assign23380_e32556_d_n11, assign23380_e32556_d_n12, assign23380_e32556_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23380_e32554: f64 = (locals.var_dg3 + locals.var_dg4);
        (assign23380_e32554, (locals.var_dg3_dn0 + locals.var_dg4_dn0), (locals.var_dg3_dn2 + locals.var_dg4_dn2), (locals.var_dg3_dn6 + locals.var_dg4_dn6), (locals.var_dg3_dn7 + locals.var_dg4_dn7), (locals.var_dg3_dn10 + locals.var_dg4_dn10), (locals.var_dg3_dn11 + locals.var_dg4_dn11), (locals.var_dg3_dn12 + locals.var_dg4_dn12), (locals.var_dg3_dn17 + locals.var_dg4_dn17),)
    } else {
        (locals.var_didd, locals.var_didd_dn0, locals.var_didd_dn2, locals.var_didd_dn6, locals.var_didd_dn7, locals.var_didd_dn10, locals.var_didd_dn11, locals.var_didd_dn12, locals.var_didd_dn17,)
    }
};
        locals.var_didd = assign23380_e32556;
        locals.var_didd_dn0 = assign23380_e32556_d_n0;
        locals.var_didd_dn2 = assign23380_e32556_d_n2;
        locals.var_didd_dn6 = assign23380_e32556_d_n6;
        locals.var_didd_dn7 = assign23380_e32556_d_n7;
        locals.var_didd_dn10 = assign23380_e32556_d_n10;
        locals.var_didd_dn11 = assign23380_e32556_d_n11;
        locals.var_didd_dn12 = assign23380_e32556_d_n12;
        locals.var_didd_dn17 = assign23380_e32556_d_n17;
        locals.var_didd_rv = 0.0;

        let (assign23390_e32564, assign23390_e32564_d_n0, assign23390_e32564_d_n2, assign23390_e32564_d_n6, assign23390_e32564_d_n7, assign23390_e32564_d_n10, assign23390_e32564_d_n11, assign23390_e32564_d_n12, assign23390_e32564_d_n17,) = {
    if (locals.var_guard718 != 0.0) {
        let assign23390_e32560: f64 = (locals.var_betawl * locals.var_didd);
        let assign23390_e32562: f64 = (assign23390_e32560 * locals.var_mu);
        (assign23390_e32562, ((((locals.var_betawl_dn0 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn0)) * locals.var_mu) + (assign23390_e32560 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn2)) * locals.var_mu) + (assign23390_e32560 * locals.var_mu_dn2)), ((((locals.var_betawl_dn6 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn6)) * locals.var_mu) + (assign23390_e32560 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn7)) * locals.var_mu) + (assign23390_e32560 * locals.var_mu_dn7)), ((((locals.var_betawl_dn10 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn10)) * locals.var_mu) + (assign23390_e32560 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn11)) * locals.var_mu) + (assign23390_e32560 * locals.var_mu_dn11)), ((((locals.var_betawl_dn12 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn12)) * locals.var_mu) + (assign23390_e32560 * locals.var_mu_dn12)), ((((locals.var_betawl_dn17 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn17)) * locals.var_mu) + (assign23390_e32560 * locals.var_mu_dn17)),)
    } else {
        (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn10, locals.var_idsibpc_dn11, locals.var_idsibpc_dn12, locals.var_idsibpc_dn17,)
    }
};
        locals.var_idsibpc = assign23390_e32564;
        locals.var_idsibpc_dn0 = assign23390_e32564_d_n0;
        locals.var_idsibpc_dn2 = assign23390_e32564_d_n2;
        locals.var_idsibpc_dn6 = assign23390_e32564_d_n6;
        locals.var_idsibpc_dn7 = assign23390_e32564_d_n7;
        locals.var_idsibpc_dn10 = assign23390_e32564_d_n10;
        locals.var_idsibpc_dn11 = assign23390_e32564_d_n11;
        locals.var_idsibpc_dn12 = assign23390_e32564_d_n12;
        locals.var_idsibpc_dn17 = assign23390_e32564_d_n17;
        locals.var_idsibpc_rv = 0.0;

        let assign23400_e32567: f64 = (locals.var_tfox0 * 100.0);
        locals.var_cgs_tfox0__blk735 = assign23400_e32567;
        locals.var_cgs_tfox0__blk735_rv = 0.0;

        let assign23410_e32570: f64 = (locals.var_c_fox / 10000.0);
        locals.var_cgs_c_fox = assign23410_e32570;
        locals.var_cgs_c_fox_dn0 = (locals.var_c_fox_dn0 / 10000.0);
        locals.var_cgs_c_fox_dn2 = (locals.var_c_fox_dn2 / 10000.0);
        locals.var_cgs_c_fox_dn6 = (locals.var_c_fox_dn6 / 10000.0);
        locals.var_cgs_c_fox_dn7 = (locals.var_c_fox_dn7 / 10000.0);
        locals.var_cgs_c_fox_dn10 = (locals.var_c_fox_dn10 / 10000.0);
        locals.var_cgs_c_fox_dn11 = (locals.var_c_fox_dn11 / 10000.0);
        locals.var_cgs_c_fox_dn12 = (locals.var_c_fox_dn12 / 10000.0);
        locals.var_cgs_c_fox_dn17 = (locals.var_c_fox_dn17 / 10000.0);
        locals.var_cgs_c_fox_rv = 0.0;

        let assign23420_e32573: f64 = (locals.var_leff * 100.0);
        locals.var_cgs_leff__blk737 = assign23420_e32573;
        locals.var_cgs_leff__blk737_rv = 0.0;

        let assign23430_e32576: f64 = (locals.var_weff_nf * 100.0);
        locals.var_cgs_weff_nf__blk738 = assign23430_e32576;
        locals.var_cgs_weff_nf__blk738_rv = 0.0;

        let assign23440_e32579: f64 = (locals.var_ey / 100.0);
        locals.var_cgs_ey = assign23440_e32579;
        locals.var_cgs_ey_dn0 = (locals.var_ey_dn0 / 100.0);
        locals.var_cgs_ey_dn2 = (locals.var_ey_dn2 / 100.0);
        locals.var_cgs_ey_dn6 = (locals.var_ey_dn6 / 100.0);
        locals.var_cgs_ey_dn7 = (locals.var_ey_dn7 / 100.0);
        locals.var_cgs_ey_dn10 = (locals.var_ey_dn10 / 100.0);
        locals.var_cgs_ey_dn11 = (locals.var_ey_dn11 / 100.0);
        locals.var_cgs_ey_dn12 = (locals.var_ey_dn12 / 100.0);
        locals.var_cgs_ey_dn17 = (locals.var_ey_dn17 / 100.0);
        locals.var_cgs_ey_rv = 0.0;

        let assign23450_e32582: f64 = (locals.var_qiu / 10000.0);
        locals.var_cgs_qiu__blk740 = assign23450_e32582;
        locals.var_cgs_qiu__blk740_dn0 = (locals.var_qiu_dn0 / 10000.0);
        locals.var_cgs_qiu__blk740_dn2 = (locals.var_qiu_dn2 / 10000.0);
        locals.var_cgs_qiu__blk740_dn6 = (locals.var_qiu_dn6 / 10000.0);
        locals.var_cgs_qiu__blk740_dn7 = (locals.var_qiu_dn7 / 10000.0);
        locals.var_cgs_qiu__blk740_dn10 = (locals.var_qiu_dn10 / 10000.0);
        locals.var_cgs_qiu__blk740_dn11 = (locals.var_qiu_dn11 / 10000.0);
        locals.var_cgs_qiu__blk740_dn12 = (locals.var_qiu_dn12 / 10000.0);
        locals.var_cgs_qiu__blk740_dn17 = (locals.var_qiu_dn17 / 10000.0);
        locals.var_cgs_qiu__blk740_rv = 0.0;

        let assign23460_e32585: f64 = (locals.var_cnst0soi / 10000.0);
        locals.var_cgs_cnst0soi = assign23460_e32585;
        locals.var_cgs_cnst0soi_dn0 = (locals.var_cnst0soi_dn0 / 10000.0);
        locals.var_cgs_cnst0soi_dn2 = (locals.var_cnst0soi_dn2 / 10000.0);
        locals.var_cgs_cnst0soi_dn6 = (locals.var_cnst0soi_dn6 / 10000.0);
        locals.var_cgs_cnst0soi_dn7 = (locals.var_cnst0soi_dn7 / 10000.0);
        locals.var_cgs_cnst0soi_dn10 = (locals.var_cnst0soi_dn10 / 10000.0);
        locals.var_cgs_cnst0soi_dn11 = (locals.var_cnst0soi_dn11 / 10000.0);
        locals.var_cgs_cnst0soi_dn12 = (locals.var_cnst0soi_dn12 / 10000.0);
        locals.var_cgs_cnst0soi_dn17 = (locals.var_cnst0soi_dn17 / 10000.0);
        locals.var_cgs_cnst0soi_rv = 0.0;

        let assign23470_e32588: f64 = if p.p27 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard742 = assign23470_e32588;
        locals.var_guard742_rv = 0.0;

        let assign23530_e32611: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard743 = assign23530_e32611;
        locals.var_guard743_rv = 0.0;

        let (assign23540_e32624, assign23540_e32624_d_n0, assign23540_e32624_d_n2, assign23540_e32624_d_n6, assign23540_e32624_d_n7, assign23540_e32624_d_n10, assign23540_e32624_d_n11, assign23540_e32624_d_n12, assign23540_e32624_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23540_e32618: f64 = (locals.var_ps0z + locals.var_vdsz);
        let assign23540_e32621: f64 = (10.0 * 2.220446049250313e-16);
        let assign23540_e32622: f64 = (assign23540_e32618 - assign23540_e32621);
        (assign23540_e32622, (locals.var_ps0z_dn0 + locals.var_vdsz_dn0), (locals.var_ps0z_dn2 + locals.var_vdsz_dn2), (locals.var_ps0z_dn6 + locals.var_vdsz_dn6), (locals.var_ps0z_dn7 + locals.var_vdsz_dn7), (locals.var_ps0z_dn10 + locals.var_vdsz_dn10), (locals.var_ps0z_dn11 + locals.var_vdsz_dn11), (locals.var_ps0z_dn12 + locals.var_vdsz_dn12), (locals.var_ps0z_dn17 + locals.var_vdsz_dn17),)
    } else {
        (locals.var_psdlz, locals.var_psdlz_dn0, locals.var_psdlz_dn2, locals.var_psdlz_dn6, locals.var_psdlz_dn7, locals.var_psdlz_dn10, locals.var_psdlz_dn11, locals.var_psdlz_dn12, locals.var_psdlz_dn17,)
    }
};
        locals.var_psdlz = assign23540_e32624;
        locals.var_psdlz_dn0 = assign23540_e32624_d_n0;
        locals.var_psdlz_dn2 = assign23540_e32624_d_n2;
        locals.var_psdlz_dn6 = assign23540_e32624_d_n6;
        locals.var_psdlz_dn7 = assign23540_e32624_d_n7;
        locals.var_psdlz_dn10 = assign23540_e32624_d_n10;
        locals.var_psdlz_dn11 = assign23540_e32624_d_n11;
        locals.var_psdlz_dn12 = assign23540_e32624_d_n12;
        locals.var_psdlz_dn17 = assign23540_e32624_d_n17;
        locals.var_psdlz_rv = 0.0;

        let (assign23550_e32645, assign23550_e32645_d_n0, assign23550_e32645_d_n2, assign23550_e32645_d_n6, assign23550_e32645_d_n7, assign23550_e32645_d_n10, assign23550_e32645_d_n11, assign23550_e32645_d_n12, assign23550_e32645_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23550_e32631: f64 = (locals.var_vgsz - locals.var_vfb);
        let assign23550_e32635: f64 = (locals.var_dvth - locals.var_dppg);
        let assign23550_e32636: f64 = (p.p216 * assign23550_e32635);
        let assign23550_e32638: f64 = (assign23550_e32636 * locals.var_cgs_leff__blk737);
        let assign23550_e32639: f64 = (assign23550_e32631 + assign23550_e32638);
        let assign23550_e32642: f64 = (locals.var_psdlz * p.p215);
        let assign23550_e32643: f64 = (assign23550_e32639 - assign23550_e32642);
        (assign23550_e32643, ((locals.var_vgsz_dn0 + ((p.p216 * (locals.var_dvth_dn0 - locals.var_dppg_dn0)) * locals.var_cgs_leff__blk737)) - (locals.var_psdlz_dn0 * p.p215)), ((locals.var_vgsz_dn2 + ((p.p216 * (locals.var_dvth_dn2 - locals.var_dppg_dn2)) * locals.var_cgs_leff__blk737)) - (locals.var_psdlz_dn2 * p.p215)), ((locals.var_vgsz_dn6 + ((p.p216 * (locals.var_dvth_dn6 - locals.var_dppg_dn6)) * locals.var_cgs_leff__blk737)) - (locals.var_psdlz_dn6 * p.p215)), ((locals.var_vgsz_dn7 + ((p.p216 * (locals.var_dvth_dn7 - locals.var_dppg_dn7)) * locals.var_cgs_leff__blk737)) - (locals.var_psdlz_dn7 * p.p215)), ((locals.var_vgsz_dn10 + ((p.p216 * (locals.var_dvth_dn10 - locals.var_dppg_dn10)) * locals.var_cgs_leff__blk737)) - (locals.var_psdlz_dn10 * p.p215)), ((locals.var_vgsz_dn11 + ((p.p216 * (locals.var_dvth_dn11 - locals.var_dppg_dn11)) * locals.var_cgs_leff__blk737)) - (locals.var_psdlz_dn11 * p.p215)), ((locals.var_vgsz_dn12 + ((p.p216 * (locals.var_dvth_dn12 - locals.var_dppg_dn12)) * locals.var_cgs_leff__blk737)) - (locals.var_psdlz_dn12 * p.p215)), ((locals.var_vgsz_dn17 + ((p.p216 * (locals.var_dvth_dn17 - locals.var_dppg_dn17)) * locals.var_cgs_leff__blk737)) - (locals.var_psdlz_dn17 * p.p215)),)
    } else {
        (locals.var_t1__blk724, locals.var_t1__blk724_dn0, locals.var_t1__blk724_dn2, locals.var_t1__blk724_dn6, locals.var_t1__blk724_dn7, locals.var_t1__blk724_dn10, locals.var_t1__blk724_dn11, locals.var_t1__blk724_dn12, locals.var_t1__blk724_dn17,)
    }
};
        locals.var_t1__blk724 = assign23550_e32645;
        locals.var_t1__blk724_dn0 = assign23550_e32645_d_n0;
        locals.var_t1__blk724_dn2 = assign23550_e32645_d_n2;
        locals.var_t1__blk724_dn6 = assign23550_e32645_d_n6;
        locals.var_t1__blk724_dn7 = assign23550_e32645_d_n7;
        locals.var_t1__blk724_dn10 = assign23550_e32645_d_n10;
        locals.var_t1__blk724_dn11 = assign23550_e32645_d_n11;
        locals.var_t1__blk724_dn12 = assign23550_e32645_d_n12;
        locals.var_t1__blk724_dn17 = assign23550_e32645_d_n17;
        locals.var_t1__blk724_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_84(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23560_e32654, assign23560_e32654_d_n0, assign23560_e32654_d_n2, assign23560_e32654_d_n6, assign23560_e32654_d_n7, assign23560_e32654_d_n10, assign23560_e32654_d_n11, assign23560_e32654_d_n12, assign23560_e32654_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23560_e32652: f64 = (1.0 / locals.var_cgs_tfox0__blk735);
        (assign23560_e32652, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk726, locals.var_t3__blk726_dn0, locals.var_t3__blk726_dn2, locals.var_t3__blk726_dn6, locals.var_t3__blk726_dn7, locals.var_t3__blk726_dn10, locals.var_t3__blk726_dn11, locals.var_t3__blk726_dn12, locals.var_t3__blk726_dn17,)
    }
};
        locals.var_t3__blk726 = assign23560_e32654;
        locals.var_t3__blk726_dn0 = assign23560_e32654_d_n0;
        locals.var_t3__blk726_dn2 = assign23560_e32654_d_n2;
        locals.var_t3__blk726_dn6 = assign23560_e32654_d_n6;
        locals.var_t3__blk726_dn7 = assign23560_e32654_d_n7;
        locals.var_t3__blk726_dn10 = assign23560_e32654_d_n10;
        locals.var_t3__blk726_dn11 = assign23560_e32654_d_n11;
        locals.var_t3__blk726_dn12 = assign23560_e32654_d_n12;
        locals.var_t3__blk726_dn17 = assign23560_e32654_d_n17;
        locals.var_t3__blk726_rv = 0.0;

        let (assign23570_e32663, assign23570_e32663_d_n0, assign23570_e32663_d_n2, assign23570_e32663_d_n6, assign23570_e32663_d_n7, assign23570_e32663_d_n10, assign23570_e32663_d_n11, assign23570_e32663_d_n12, assign23570_e32663_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23570_e32661: f64 = (locals.var_t1__blk724 * locals.var_t3__blk726);
        (assign23570_e32661, ((locals.var_t1__blk724_dn0 * locals.var_t3__blk726) + (locals.var_t1__blk724 * locals.var_t3__blk726_dn0)), ((locals.var_t1__blk724_dn2 * locals.var_t3__blk726) + (locals.var_t1__blk724 * locals.var_t3__blk726_dn2)), ((locals.var_t1__blk724_dn6 * locals.var_t3__blk726) + (locals.var_t1__blk724 * locals.var_t3__blk726_dn6)), ((locals.var_t1__blk724_dn7 * locals.var_t3__blk726) + (locals.var_t1__blk724 * locals.var_t3__blk726_dn7)), ((locals.var_t1__blk724_dn10 * locals.var_t3__blk726) + (locals.var_t1__blk724 * locals.var_t3__blk726_dn10)), ((locals.var_t1__blk724_dn11 * locals.var_t3__blk726) + (locals.var_t1__blk724 * locals.var_t3__blk726_dn11)), ((locals.var_t1__blk724_dn12 * locals.var_t3__blk726) + (locals.var_t1__blk724 * locals.var_t3__blk726_dn12)), ((locals.var_t1__blk724_dn17 * locals.var_t3__blk726) + (locals.var_t1__blk724 * locals.var_t3__blk726_dn17)),)
    } else {
        (locals.var_t2__blk725, locals.var_t2__blk725_dn0, locals.var_t2__blk725_dn2, locals.var_t2__blk725_dn6, locals.var_t2__blk725_dn7, locals.var_t2__blk725_dn10, locals.var_t2__blk725_dn11, locals.var_t2__blk725_dn12, locals.var_t2__blk725_dn17,)
    }
};
        locals.var_t2__blk725 = assign23570_e32663;
        locals.var_t2__blk725_dn0 = assign23570_e32663_d_n0;
        locals.var_t2__blk725_dn2 = assign23570_e32663_d_n2;
        locals.var_t2__blk725_dn6 = assign23570_e32663_d_n6;
        locals.var_t2__blk725_dn7 = assign23570_e32663_d_n7;
        locals.var_t2__blk725_dn10 = assign23570_e32663_d_n10;
        locals.var_t2__blk725_dn11 = assign23570_e32663_d_n11;
        locals.var_t2__blk725_dn12 = assign23570_e32663_d_n12;
        locals.var_t2__blk725_dn17 = assign23570_e32663_d_n17;
        locals.var_t2__blk725_rv = 0.0;

        let (assign23580_e32672, assign23580_e32672_d_n0, assign23580_e32672_d_n2, assign23580_e32672_d_n6, assign23580_e32672_d_n7, assign23580_e32672_d_n10, assign23580_e32672_d_n11, assign23580_e32672_d_n12, assign23580_e32672_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23580_e32670: f64 = (1.0 / p.p217);
        (assign23580_e32670, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk726, locals.var_t3__blk726_dn0, locals.var_t3__blk726_dn2, locals.var_t3__blk726_dn6, locals.var_t3__blk726_dn7, locals.var_t3__blk726_dn10, locals.var_t3__blk726_dn11, locals.var_t3__blk726_dn12, locals.var_t3__blk726_dn17,)
    }
};
        locals.var_t3__blk726 = assign23580_e32672;
        locals.var_t3__blk726_dn0 = assign23580_e32672_d_n0;
        locals.var_t3__blk726_dn2 = assign23580_e32672_d_n2;
        locals.var_t3__blk726_dn6 = assign23580_e32672_d_n6;
        locals.var_t3__blk726_dn7 = assign23580_e32672_d_n7;
        locals.var_t3__blk726_dn10 = assign23580_e32672_d_n10;
        locals.var_t3__blk726_dn11 = assign23580_e32672_d_n11;
        locals.var_t3__blk726_dn12 = assign23580_e32672_d_n12;
        locals.var_t3__blk726_dn17 = assign23580_e32672_d_n17;
        locals.var_t3__blk726_rv = 0.0;

        let (assign23590_e32683, assign23590_e32683_d_n0, assign23590_e32683_d_n2, assign23590_e32683_d_n6, assign23590_e32683_d_n7, assign23590_e32683_d_n10, assign23590_e32683_d_n11, assign23590_e32683_d_n12, assign23590_e32683_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23590_e32680: f64 = (locals.var_cgs_ey * locals.var_t3__blk726);
        let assign23590_e32681: f64 = (1.0 + assign23590_e32680);
        (assign23590_e32681, ((locals.var_cgs_ey_dn0 * locals.var_t3__blk726) + (locals.var_cgs_ey * locals.var_t3__blk726_dn0)), ((locals.var_cgs_ey_dn2 * locals.var_t3__blk726) + (locals.var_cgs_ey * locals.var_t3__blk726_dn2)), ((locals.var_cgs_ey_dn6 * locals.var_t3__blk726) + (locals.var_cgs_ey * locals.var_t3__blk726_dn6)), ((locals.var_cgs_ey_dn7 * locals.var_t3__blk726) + (locals.var_cgs_ey * locals.var_t3__blk726_dn7)), ((locals.var_cgs_ey_dn10 * locals.var_t3__blk726) + (locals.var_cgs_ey * locals.var_t3__blk726_dn10)), ((locals.var_cgs_ey_dn11 * locals.var_t3__blk726) + (locals.var_cgs_ey * locals.var_t3__blk726_dn11)), ((locals.var_cgs_ey_dn12 * locals.var_t3__blk726) + (locals.var_cgs_ey * locals.var_t3__blk726_dn12)), ((locals.var_cgs_ey_dn17 * locals.var_t3__blk726) + (locals.var_cgs_ey * locals.var_t3__blk726_dn17)),)
    } else {
        (locals.var_t7__blk730, locals.var_t7__blk730_dn0, locals.var_t7__blk730_dn2, locals.var_t7__blk730_dn6, locals.var_t7__blk730_dn7, locals.var_t7__blk730_dn10, locals.var_t7__blk730_dn11, locals.var_t7__blk730_dn12, locals.var_t7__blk730_dn17,)
    }
};
        locals.var_t7__blk730 = assign23590_e32683;
        locals.var_t7__blk730_dn0 = assign23590_e32683_d_n0;
        locals.var_t7__blk730_dn2 = assign23590_e32683_d_n2;
        locals.var_t7__blk730_dn6 = assign23590_e32683_d_n6;
        locals.var_t7__blk730_dn7 = assign23590_e32683_d_n7;
        locals.var_t7__blk730_dn10 = assign23590_e32683_d_n10;
        locals.var_t7__blk730_dn11 = assign23590_e32683_d_n11;
        locals.var_t7__blk730_dn12 = assign23590_e32683_d_n12;
        locals.var_t7__blk730_dn17 = assign23590_e32683_d_n17;
        locals.var_t7__blk730_rv = 0.0;

        let (assign23600_e32692, assign23600_e32692_d_n0, assign23600_e32692_d_n2, assign23600_e32692_d_n6, assign23600_e32692_d_n7, assign23600_e32692_d_n10, assign23600_e32692_d_n11, assign23600_e32692_d_n12, assign23600_e32692_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23600_e32690: f64 = (locals.var_t2__blk725 * locals.var_t7__blk730);
        (assign23600_e32690, ((locals.var_t2__blk725_dn0 * locals.var_t7__blk730) + (locals.var_t2__blk725 * locals.var_t7__blk730_dn0)), ((locals.var_t2__blk725_dn2 * locals.var_t7__blk730) + (locals.var_t2__blk725 * locals.var_t7__blk730_dn2)), ((locals.var_t2__blk725_dn6 * locals.var_t7__blk730) + (locals.var_t2__blk725 * locals.var_t7__blk730_dn6)), ((locals.var_t2__blk725_dn7 * locals.var_t7__blk730) + (locals.var_t2__blk725 * locals.var_t7__blk730_dn7)), ((locals.var_t2__blk725_dn10 * locals.var_t7__blk730) + (locals.var_t2__blk725 * locals.var_t7__blk730_dn10)), ((locals.var_t2__blk725_dn11 * locals.var_t7__blk730) + (locals.var_t2__blk725 * locals.var_t7__blk730_dn11)), ((locals.var_t2__blk725_dn12 * locals.var_t7__blk730) + (locals.var_t2__blk725 * locals.var_t7__blk730_dn12)), ((locals.var_t2__blk725_dn17 * locals.var_t7__blk730) + (locals.var_t2__blk725 * locals.var_t7__blk730_dn17)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign23600_e32692;
        locals.var_etun_dn0 = assign23600_e32692_d_n0;
        locals.var_etun_dn2 = assign23600_e32692_d_n2;
        locals.var_etun_dn6 = assign23600_e32692_d_n6;
        locals.var_etun_dn7 = assign23600_e32692_d_n7;
        locals.var_etun_dn10 = assign23600_e32692_d_n10;
        locals.var_etun_dn11 = assign23600_e32692_d_n11;
        locals.var_etun_dn12 = assign23600_e32692_d_n12;
        locals.var_etun_dn17 = assign23600_e32692_d_n17;
        locals.var_etun_rv = 0.0;

        let (assign23610_e32708, assign23610_e32708_d_n0, assign23610_e32708_d_n2, assign23610_e32708_d_n6, assign23610_e32708_d_n7, assign23610_e32708_d_n10, assign23610_e32708_d_n11, assign23610_e32708_d_n12, assign23610_e32708_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23610_e32699: f64 = (locals.var_etun * locals.var_etun);
        let assign23610_e32702: f64 = (4.0 * 0.01);
        let assign23610_e32704: f64 = (assign23610_e32702 * 0.01);
        let assign23610_e32705: f64 = (assign23610_e32699 + assign23610_e32704);
        let assign23610_e32706: f64 = (assign23610_e32705).sqrt();
        (assign23610_e32706, (((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0)) / (2.0 * assign23610_e32706)), (((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2)) / (2.0 * assign23610_e32706)), (((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6)) / (2.0 * assign23610_e32706)), (((locals.var_etun_dn7 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn7)) / (2.0 * assign23610_e32706)), (((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10)) / (2.0 * assign23610_e32706)), (((locals.var_etun_dn11 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn11)) / (2.0 * assign23610_e32706)), (((locals.var_etun_dn12 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn12)) / (2.0 * assign23610_e32706)), (((locals.var_etun_dn17 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn17)) / (2.0 * assign23610_e32706)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign23610_e32708;
        locals.var_tmf1_dn0 = assign23610_e32708_d_n0;
        locals.var_tmf1_dn2 = assign23610_e32708_d_n2;
        locals.var_tmf1_dn6 = assign23610_e32708_d_n6;
        locals.var_tmf1_dn7 = assign23610_e32708_d_n7;
        locals.var_tmf1_dn10 = assign23610_e32708_d_n10;
        locals.var_tmf1_dn11 = assign23610_e32708_d_n11;
        locals.var_tmf1_dn12 = assign23610_e32708_d_n12;
        locals.var_tmf1_dn17 = assign23610_e32708_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign23620_e32723, assign23620_e32723_d_n0, assign23620_e32723_d_n2, assign23620_e32723_d_n6, assign23620_e32723_d_n7, assign23620_e32723_d_n10, assign23620_e32723_d_n11, assign23620_e32723_d_n12, assign23620_e32723_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23620_e32716: f64 = (locals.var_etun + locals.var_tmf1);
        let assign23620_e32717: f64 = (0.5 * assign23620_e32716);
        let assign23620_e32720: f64 = (1e-10 * 0.01);
        let assign23620_e32721: f64 = (assign23620_e32717 + assign23620_e32720);
        (assign23620_e32721, (0.5 * (locals.var_etun_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_etun_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_etun_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_etun_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_etun_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_etun_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_etun_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_etun_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign23620_e32723;
        locals.var_etun_dn0 = assign23620_e32723_d_n0;
        locals.var_etun_dn2 = assign23620_e32723_d_n2;
        locals.var_etun_dn6 = assign23620_e32723_d_n6;
        locals.var_etun_dn7 = assign23620_e32723_d_n7;
        locals.var_etun_dn10 = assign23620_e32723_d_n10;
        locals.var_etun_dn11 = assign23620_e32723_d_n11;
        locals.var_etun_dn12 = assign23620_e32723_d_n12;
        locals.var_etun_dn17 = assign23620_e32723_d_n17;
        locals.var_etun_rv = 0.0;

        let assign23630_e32726: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard744 = assign23630_e32726;
        locals.var_guard744_rv = 0.0;

        let (assign23640_e32735, assign23640_e32735_d_n0, assign23640_e32735_d_n2, assign23640_e32735_d_n6, assign23640_e32735_d_n7, assign23640_e32735_d_n10, assign23640_e32735_d_n11, assign23640_e32735_d_n12, assign23640_e32735_d_n17,) = {
    if (((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) && (locals.var_guard744 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign23640_e32735;
        locals.var_etun_dn0 = assign23640_e32735_d_n0;
        locals.var_etun_dn2 = assign23640_e32735_d_n2;
        locals.var_etun_dn6 = assign23640_e32735_d_n6;
        locals.var_etun_dn7 = assign23640_e32735_d_n7;
        locals.var_etun_dn10 = assign23640_e32735_d_n10;
        locals.var_etun_dn11 = assign23640_e32735_d_n11;
        locals.var_etun_dn12 = assign23640_e32735_d_n12;
        locals.var_etun_dn17 = assign23640_e32735_d_n17;
        locals.var_etun_rv = 0.0;

        let (assign23650_e32751, assign23650_e32751_d_n0, assign23650_e32751_d_n2, assign23650_e32751_d_n6, assign23650_e32751_d_n7, assign23650_e32751_d_n10, assign23650_e32751_d_n11, assign23650_e32751_d_n12, assign23650_e32751_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23650_e32742: f64 = (locals.var_vgsz * locals.var_vgsz);
        let assign23650_e32745: f64 = (4.0 * 0.001);
        let assign23650_e32747: f64 = (assign23650_e32745 * 0.001);
        let assign23650_e32748: f64 = (assign23650_e32742 + assign23650_e32747);
        let assign23650_e32749: f64 = (assign23650_e32748).sqrt();
        (assign23650_e32749, (((locals.var_vgsz_dn0 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn0)) / (2.0 * assign23650_e32749)), (((locals.var_vgsz_dn2 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn2)) / (2.0 * assign23650_e32749)), (((locals.var_vgsz_dn6 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn6)) / (2.0 * assign23650_e32749)), (((locals.var_vgsz_dn7 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn7)) / (2.0 * assign23650_e32749)), (((locals.var_vgsz_dn10 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn10)) / (2.0 * assign23650_e32749)), (((locals.var_vgsz_dn11 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn11)) / (2.0 * assign23650_e32749)), (((locals.var_vgsz_dn12 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn12)) / (2.0 * assign23650_e32749)), (((locals.var_vgsz_dn17 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn17)) / (2.0 * assign23650_e32749)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign23650_e32751;
        locals.var_tmf1_dn0 = assign23650_e32751_d_n0;
        locals.var_tmf1_dn2 = assign23650_e32751_d_n2;
        locals.var_tmf1_dn6 = assign23650_e32751_d_n6;
        locals.var_tmf1_dn7 = assign23650_e32751_d_n7;
        locals.var_tmf1_dn10 = assign23650_e32751_d_n10;
        locals.var_tmf1_dn11 = assign23650_e32751_d_n11;
        locals.var_tmf1_dn12 = assign23650_e32751_d_n12;
        locals.var_tmf1_dn17 = assign23650_e32751_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign23660_e32766, assign23660_e32766_d_n0, assign23660_e32766_d_n2, assign23660_e32766_d_n6, assign23660_e32766_d_n7, assign23660_e32766_d_n10, assign23660_e32766_d_n11, assign23660_e32766_d_n12, assign23660_e32766_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23660_e32759: f64 = (locals.var_vgsz + locals.var_tmf1);
        let assign23660_e32760: f64 = (0.5 * assign23660_e32759);
        let assign23660_e32763: f64 = (1e-10 * 0.001);
        let assign23660_e32764: f64 = (assign23660_e32760 + assign23660_e32763);
        (assign23660_e32764, (0.5 * (locals.var_vgsz_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_vgsz_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_vgsz_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_vgsz_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_vgsz_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_vgsz_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_vgsz_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_vgsz_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t3__blk726, locals.var_t3__blk726_dn0, locals.var_t3__blk726_dn2, locals.var_t3__blk726_dn6, locals.var_t3__blk726_dn7, locals.var_t3__blk726_dn10, locals.var_t3__blk726_dn11, locals.var_t3__blk726_dn12, locals.var_t3__blk726_dn17,)
    }
};
        locals.var_t3__blk726 = assign23660_e32766;
        locals.var_t3__blk726_dn0 = assign23660_e32766_d_n0;
        locals.var_t3__blk726_dn2 = assign23660_e32766_d_n2;
        locals.var_t3__blk726_dn6 = assign23660_e32766_d_n6;
        locals.var_t3__blk726_dn7 = assign23660_e32766_d_n7;
        locals.var_t3__blk726_dn10 = assign23660_e32766_d_n10;
        locals.var_t3__blk726_dn11 = assign23660_e32766_d_n11;
        locals.var_t3__blk726_dn12 = assign23660_e32766_d_n12;
        locals.var_t3__blk726_dn17 = assign23660_e32766_d_n17;
        locals.var_t3__blk726_rv = 0.0;

        let assign23670_e32769: f64 = if locals.var_t3__blk726 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard745 = assign23670_e32769;
        locals.var_guard745_rv = 0.0;

        let (assign23680_e32778, assign23680_e32778_d_n0, assign23680_e32778_d_n2, assign23680_e32778_d_n6, assign23680_e32778_d_n7, assign23680_e32778_d_n10, assign23680_e32778_d_n11, assign23680_e32778_d_n12, assign23680_e32778_d_n17,) = {
    if (((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) && (locals.var_guard745 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk726, locals.var_t3__blk726_dn0, locals.var_t3__blk726_dn2, locals.var_t3__blk726_dn6, locals.var_t3__blk726_dn7, locals.var_t3__blk726_dn10, locals.var_t3__blk726_dn11, locals.var_t3__blk726_dn12, locals.var_t3__blk726_dn17,)
    }
};
        locals.var_t3__blk726 = assign23680_e32778;
        locals.var_t3__blk726_dn0 = assign23680_e32778_d_n0;
        locals.var_t3__blk726_dn2 = assign23680_e32778_d_n2;
        locals.var_t3__blk726_dn6 = assign23680_e32778_d_n6;
        locals.var_t3__blk726_dn7 = assign23680_e32778_d_n7;
        locals.var_t3__blk726_dn10 = assign23680_e32778_d_n10;
        locals.var_t3__blk726_dn11 = assign23680_e32778_d_n11;
        locals.var_t3__blk726_dn12 = assign23680_e32778_d_n12;
        locals.var_t3__blk726_dn17 = assign23680_e32778_d_n17;
        locals.var_t3__blk726_rv = 0.0;

        let (assign23690_e32787, assign23690_e32787_d_n0, assign23690_e32787_d_n2, assign23690_e32787_d_n6, assign23690_e32787_d_n7, assign23690_e32787_d_n10, assign23690_e32787_d_n11, assign23690_e32787_d_n12, assign23690_e32787_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23690_e32785: f64 = (locals.var_t3__blk726 - p.p226);
        (assign23690_e32785, locals.var_t3__blk726_dn0, locals.var_t3__blk726_dn2, locals.var_t3__blk726_dn6, locals.var_t3__blk726_dn7, locals.var_t3__blk726_dn10, locals.var_t3__blk726_dn11, locals.var_t3__blk726_dn12, locals.var_t3__blk726_dn17,)
    } else {
        (locals.var_t3__blk726, locals.var_t3__blk726_dn0, locals.var_t3__blk726_dn2, locals.var_t3__blk726_dn6, locals.var_t3__blk726_dn7, locals.var_t3__blk726_dn10, locals.var_t3__blk726_dn11, locals.var_t3__blk726_dn12, locals.var_t3__blk726_dn17,)
    }
};
        locals.var_t3__blk726 = assign23690_e32787;
        locals.var_t3__blk726_dn0 = assign23690_e32787_d_n0;
        locals.var_t3__blk726_dn2 = assign23690_e32787_d_n2;
        locals.var_t3__blk726_dn6 = assign23690_e32787_d_n6;
        locals.var_t3__blk726_dn7 = assign23690_e32787_d_n7;
        locals.var_t3__blk726_dn10 = assign23690_e32787_d_n10;
        locals.var_t3__blk726_dn11 = assign23690_e32787_d_n11;
        locals.var_t3__blk726_dn12 = assign23690_e32787_d_n12;
        locals.var_t3__blk726_dn17 = assign23690_e32787_d_n17;
        locals.var_t3__blk726_rv = 0.0;

        let (assign23700_e32796, assign23700_e32796_d_n0, assign23700_e32796_d_n2, assign23700_e32796_d_n6, assign23700_e32796_d_n7, assign23700_e32796_d_n10, assign23700_e32796_d_n11, assign23700_e32796_d_n12, assign23700_e32796_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23700_e32794: f64 = (locals.var_t3__blk726 / 0.1);
        (assign23700_e32794, (locals.var_t3__blk726_dn0 / 0.1), (locals.var_t3__blk726_dn2 / 0.1), (locals.var_t3__blk726_dn6 / 0.1), (locals.var_t3__blk726_dn7 / 0.1), (locals.var_t3__blk726_dn10 / 0.1), (locals.var_t3__blk726_dn11 / 0.1), (locals.var_t3__blk726_dn12 / 0.1), (locals.var_t3__blk726_dn17 / 0.1),)
    } else {
        (locals.var_tx__blk722, locals.var_tx__blk722_dn0, locals.var_tx__blk722_dn2, locals.var_tx__blk722_dn6, locals.var_tx__blk722_dn7, locals.var_tx__blk722_dn10, locals.var_tx__blk722_dn11, locals.var_tx__blk722_dn12, locals.var_tx__blk722_dn17,)
    }
};
        locals.var_tx__blk722 = assign23700_e32796;
        locals.var_tx__blk722_dn0 = assign23700_e32796_d_n0;
        locals.var_tx__blk722_dn2 = assign23700_e32796_d_n2;
        locals.var_tx__blk722_dn6 = assign23700_e32796_d_n6;
        locals.var_tx__blk722_dn7 = assign23700_e32796_d_n7;
        locals.var_tx__blk722_dn10 = assign23700_e32796_d_n10;
        locals.var_tx__blk722_dn11 = assign23700_e32796_d_n11;
        locals.var_tx__blk722_dn12 = assign23700_e32796_d_n12;
        locals.var_tx__blk722_dn17 = assign23700_e32796_d_n17;
        locals.var_tx__blk722_rv = 0.0;

        let (assign23710_e32807, assign23710_e32807_d_n0, assign23710_e32807_d_n2, assign23710_e32807_d_n6, assign23710_e32807_d_n7, assign23710_e32807_d_n10, assign23710_e32807_d_n11, assign23710_e32807_d_n12, assign23710_e32807_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23710_e32804: f64 = (locals.var_tx__blk722 * locals.var_tx__blk722);
        let assign23710_e32805: f64 = (1.0 + assign23710_e32804);
        (assign23710_e32805, ((locals.var_tx__blk722_dn0 * locals.var_tx__blk722) + (locals.var_tx__blk722 * locals.var_tx__blk722_dn0)), ((locals.var_tx__blk722_dn2 * locals.var_tx__blk722) + (locals.var_tx__blk722 * locals.var_tx__blk722_dn2)), ((locals.var_tx__blk722_dn6 * locals.var_tx__blk722) + (locals.var_tx__blk722 * locals.var_tx__blk722_dn6)), ((locals.var_tx__blk722_dn7 * locals.var_tx__blk722) + (locals.var_tx__blk722 * locals.var_tx__blk722_dn7)), ((locals.var_tx__blk722_dn10 * locals.var_tx__blk722) + (locals.var_tx__blk722 * locals.var_tx__blk722_dn10)), ((locals.var_tx__blk722_dn11 * locals.var_tx__blk722) + (locals.var_tx__blk722 * locals.var_tx__blk722_dn11)), ((locals.var_tx__blk722_dn12 * locals.var_tx__blk722) + (locals.var_tx__blk722 * locals.var_tx__blk722_dn12)), ((locals.var_tx__blk722_dn17 * locals.var_tx__blk722) + (locals.var_tx__blk722 * locals.var_tx__blk722_dn17)),)
    } else {
        (locals.var_t2__blk725, locals.var_t2__blk725_dn0, locals.var_t2__blk725_dn2, locals.var_t2__blk725_dn6, locals.var_t2__blk725_dn7, locals.var_t2__blk725_dn10, locals.var_t2__blk725_dn11, locals.var_t2__blk725_dn12, locals.var_t2__blk725_dn17,)
    }
};
        locals.var_t2__blk725 = assign23710_e32807;
        locals.var_t2__blk725_dn0 = assign23710_e32807_d_n0;
        locals.var_t2__blk725_dn2 = assign23710_e32807_d_n2;
        locals.var_t2__blk725_dn6 = assign23710_e32807_d_n6;
        locals.var_t2__blk725_dn7 = assign23710_e32807_d_n7;
        locals.var_t2__blk725_dn10 = assign23710_e32807_d_n10;
        locals.var_t2__blk725_dn11 = assign23710_e32807_d_n11;
        locals.var_t2__blk725_dn12 = assign23710_e32807_d_n12;
        locals.var_t2__blk725_dn17 = assign23710_e32807_d_n17;
        locals.var_t2__blk725_rv = 0.0;

        let (assign23720_e32818, assign23720_e32818_d_n0, assign23720_e32818_d_n2, assign23720_e32818_d_n6, assign23720_e32818_d_n7, assign23720_e32818_d_n10, assign23720_e32818_d_n11, assign23720_e32818_d_n12, assign23720_e32818_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23720_e32815: f64 = (1.0 / locals.var_t2__blk725);
        let assign23720_e32816: f64 = (1.0 - assign23720_e32815);
        (assign23720_e32816, (-(-(locals.var_t2__blk725_dn0 / (locals.var_t2__blk725 * locals.var_t2__blk725)))), (-(-(locals.var_t2__blk725_dn2 / (locals.var_t2__blk725 * locals.var_t2__blk725)))), (-(-(locals.var_t2__blk725_dn6 / (locals.var_t2__blk725 * locals.var_t2__blk725)))), (-(-(locals.var_t2__blk725_dn7 / (locals.var_t2__blk725 * locals.var_t2__blk725)))), (-(-(locals.var_t2__blk725_dn10 / (locals.var_t2__blk725 * locals.var_t2__blk725)))), (-(-(locals.var_t2__blk725_dn11 / (locals.var_t2__blk725 * locals.var_t2__blk725)))), (-(-(locals.var_t2__blk725_dn12 / (locals.var_t2__blk725 * locals.var_t2__blk725)))), (-(-(locals.var_t2__blk725_dn17 / (locals.var_t2__blk725 * locals.var_t2__blk725)))),)
    } else {
        (locals.var_t1__blk724, locals.var_t1__blk724_dn0, locals.var_t1__blk724_dn2, locals.var_t1__blk724_dn6, locals.var_t1__blk724_dn7, locals.var_t1__blk724_dn10, locals.var_t1__blk724_dn11, locals.var_t1__blk724_dn12, locals.var_t1__blk724_dn17,)
    }
};
        locals.var_t1__blk724 = assign23720_e32818;
        locals.var_t1__blk724_dn0 = assign23720_e32818_d_n0;
        locals.var_t1__blk724_dn2 = assign23720_e32818_d_n2;
        locals.var_t1__blk724_dn6 = assign23720_e32818_d_n6;
        locals.var_t1__blk724_dn7 = assign23720_e32818_d_n7;
        locals.var_t1__blk724_dn10 = assign23720_e32818_d_n10;
        locals.var_t1__blk724_dn11 = assign23720_e32818_d_n11;
        locals.var_t1__blk724_dn12 = assign23720_e32818_d_n12;
        locals.var_t1__blk724_dn17 = assign23720_e32818_d_n17;
        locals.var_t1__blk724_rv = 0.0;

        let (assign23730_e32827, assign23730_e32827_d_n0, assign23730_e32827_d_n2, assign23730_e32827_d_n6, assign23730_e32827_d_n7, assign23730_e32827_d_n10, assign23730_e32827_d_n11, assign23730_e32827_d_n12, assign23730_e32827_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23730_e32825: f64 = (locals.var_etun * locals.var_t1__blk724);
        (assign23730_e32825, ((locals.var_etun_dn0 * locals.var_t1__blk724) + (locals.var_etun * locals.var_t1__blk724_dn0)), ((locals.var_etun_dn2 * locals.var_t1__blk724) + (locals.var_etun * locals.var_t1__blk724_dn2)), ((locals.var_etun_dn6 * locals.var_t1__blk724) + (locals.var_etun * locals.var_t1__blk724_dn6)), ((locals.var_etun_dn7 * locals.var_t1__blk724) + (locals.var_etun * locals.var_t1__blk724_dn7)), ((locals.var_etun_dn10 * locals.var_t1__blk724) + (locals.var_etun * locals.var_t1__blk724_dn10)), ((locals.var_etun_dn11 * locals.var_t1__blk724) + (locals.var_etun * locals.var_t1__blk724_dn11)), ((locals.var_etun_dn12 * locals.var_t1__blk724) + (locals.var_etun * locals.var_t1__blk724_dn12)), ((locals.var_etun_dn17 * locals.var_t1__blk724) + (locals.var_etun * locals.var_t1__blk724_dn17)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign23730_e32827;
        locals.var_etun_dn0 = assign23730_e32827_d_n0;
        locals.var_etun_dn2 = assign23730_e32827_d_n2;
        locals.var_etun_dn6 = assign23730_e32827_d_n6;
        locals.var_etun_dn7 = assign23730_e32827_d_n7;
        locals.var_etun_dn10 = assign23730_e32827_d_n10;
        locals.var_etun_dn11 = assign23730_e32827_d_n11;
        locals.var_etun_dn12 = assign23730_e32827_d_n12;
        locals.var_etun_dn17 = assign23730_e32827_d_n17;
        locals.var_etun_rv = 0.0;

        let (assign23740_e32836, assign23740_e32836_d_n0, assign23740_e32836_d_n2, assign23740_e32836_d_n6, assign23740_e32836_d_n7, assign23740_e32836_d_n10, assign23740_e32836_d_n11, assign23740_e32836_d_n12, assign23740_e32836_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23740_e32834: f64 = (locals.var_cgs_leff__blk737 * locals.var_cgs_weff_nf__blk738);
        (assign23740_e32834, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk723, locals.var_t0__blk723_dn0, locals.var_t0__blk723_dn2, locals.var_t0__blk723_dn6, locals.var_t0__blk723_dn7, locals.var_t0__blk723_dn10, locals.var_t0__blk723_dn11, locals.var_t0__blk723_dn12, locals.var_t0__blk723_dn17,)
    }
};
        locals.var_t0__blk723 = assign23740_e32836;
        locals.var_t0__blk723_dn0 = assign23740_e32836_d_n0;
        locals.var_t0__blk723_dn2 = assign23740_e32836_d_n2;
        locals.var_t0__blk723_dn6 = assign23740_e32836_d_n6;
        locals.var_t0__blk723_dn7 = assign23740_e32836_d_n7;
        locals.var_t0__blk723_dn10 = assign23740_e32836_d_n10;
        locals.var_t0__blk723_dn11 = assign23740_e32836_d_n11;
        locals.var_t0__blk723_dn12 = assign23740_e32836_d_n12;
        locals.var_t0__blk723_dn17 = assign23740_e32836_d_n17;
        locals.var_t0__blk723_rv = 0.0;

        let (assign23750_e32847, assign23750_e32847_d_n0, assign23750_e32847_d_n2, assign23750_e32847_d_n6, assign23750_e32847_d_n7, assign23750_e32847_d_n10, assign23750_e32847_d_n11, assign23750_e32847_d_n12, assign23750_e32847_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23750_e32844: f64 = (p.p219 + locals.var_t0__blk723);
        let assign23750_e32845: f64 = (p.p219 / assign23750_e32844);
        (assign23750_e32845, (-((p.p219 * locals.var_t0__blk723_dn0) / (assign23750_e32844 * assign23750_e32844))), (-((p.p219 * locals.var_t0__blk723_dn2) / (assign23750_e32844 * assign23750_e32844))), (-((p.p219 * locals.var_t0__blk723_dn6) / (assign23750_e32844 * assign23750_e32844))), (-((p.p219 * locals.var_t0__blk723_dn7) / (assign23750_e32844 * assign23750_e32844))), (-((p.p219 * locals.var_t0__blk723_dn10) / (assign23750_e32844 * assign23750_e32844))), (-((p.p219 * locals.var_t0__blk723_dn11) / (assign23750_e32844 * assign23750_e32844))), (-((p.p219 * locals.var_t0__blk723_dn12) / (assign23750_e32844 * assign23750_e32844))), (-((p.p219 * locals.var_t0__blk723_dn17) / (assign23750_e32844 * assign23750_e32844))),)
    } else {
        (locals.var_t7__blk730, locals.var_t7__blk730_dn0, locals.var_t7__blk730_dn2, locals.var_t7__blk730_dn6, locals.var_t7__blk730_dn7, locals.var_t7__blk730_dn10, locals.var_t7__blk730_dn11, locals.var_t7__blk730_dn12, locals.var_t7__blk730_dn17,)
    }
};
        locals.var_t7__blk730 = assign23750_e32847;
        locals.var_t7__blk730_dn0 = assign23750_e32847_d_n0;
        locals.var_t7__blk730_dn2 = assign23750_e32847_d_n2;
        locals.var_t7__blk730_dn6 = assign23750_e32847_d_n6;
        locals.var_t7__blk730_dn7 = assign23750_e32847_d_n7;
        locals.var_t7__blk730_dn10 = assign23750_e32847_d_n10;
        locals.var_t7__blk730_dn11 = assign23750_e32847_d_n11;
        locals.var_t7__blk730_dn12 = assign23750_e32847_d_n12;
        locals.var_t7__blk730_dn17 = assign23750_e32847_d_n17;
        locals.var_t7__blk730_rv = 0.0;

        let (assign23760_e32854, assign23760_e32854_d_n0, assign23760_e32854_d_n2, assign23760_e32854_d_n6, assign23760_e32854_d_n7, assign23760_e32854_d_n10, assign23760_e32854_d_n11, assign23760_e32854_d_n12, assign23760_e32854_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        (p.p218, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk729, locals.var_t6__blk729_dn0, locals.var_t6__blk729_dn2, locals.var_t6__blk729_dn6, locals.var_t6__blk729_dn7, locals.var_t6__blk729_dn10, locals.var_t6__blk729_dn11, locals.var_t6__blk729_dn12, locals.var_t6__blk729_dn17,)
    }
};
        locals.var_t6__blk729 = assign23760_e32854;
        locals.var_t6__blk729_dn0 = assign23760_e32854_d_n0;
        locals.var_t6__blk729_dn2 = assign23760_e32854_d_n2;
        locals.var_t6__blk729_dn6 = assign23760_e32854_d_n6;
        locals.var_t6__blk729_dn7 = assign23760_e32854_d_n7;
        locals.var_t6__blk729_dn10 = assign23760_e32854_d_n10;
        locals.var_t6__blk729_dn11 = assign23760_e32854_d_n11;
        locals.var_t6__blk729_dn12 = assign23760_e32854_d_n12;
        locals.var_t6__blk729_dn17 = assign23760_e32854_d_n17;
        locals.var_t6__blk729_rv = 0.0;

        let (assign23780_e32876, assign23780_e32876_d_n0, assign23780_e32876_d_n2, assign23780_e32876_d_n6, assign23780_e32876_d_n7, assign23780_e32876_d_n10, assign23780_e32876_d_n11, assign23780_e32876_d_n12, assign23780_e32876_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23780_e32873: f64 = (locals.var_etun + 1e-50);
        let assign23780_e32874: f64 = (1.0 / assign23780_e32873);
        (assign23780_e32874, (-(locals.var_etun_dn0 / (assign23780_e32873 * assign23780_e32873))), (-(locals.var_etun_dn2 / (assign23780_e32873 * assign23780_e32873))), (-(locals.var_etun_dn6 / (assign23780_e32873 * assign23780_e32873))), (-(locals.var_etun_dn7 / (assign23780_e32873 * assign23780_e32873))), (-(locals.var_etun_dn10 / (assign23780_e32873 * assign23780_e32873))), (-(locals.var_etun_dn11 / (assign23780_e32873 * assign23780_e32873))), (-(locals.var_etun_dn12 / (assign23780_e32873 * assign23780_e32873))), (-(locals.var_etun_dn17 / (assign23780_e32873 * assign23780_e32873))),)
    } else {
        (locals.var_t4__blk727, locals.var_t4__blk727_dn0, locals.var_t4__blk727_dn2, locals.var_t4__blk727_dn6, locals.var_t4__blk727_dn7, locals.var_t4__blk727_dn10, locals.var_t4__blk727_dn11, locals.var_t4__blk727_dn12, locals.var_t4__blk727_dn17,)
    }
};
        locals.var_t4__blk727 = assign23780_e32876;
        locals.var_t4__blk727_dn0 = assign23780_e32876_d_n0;
        locals.var_t4__blk727_dn2 = assign23780_e32876_d_n2;
        locals.var_t4__blk727_dn6 = assign23780_e32876_d_n6;
        locals.var_t4__blk727_dn7 = assign23780_e32876_d_n7;
        locals.var_t4__blk727_dn10 = assign23780_e32876_d_n10;
        locals.var_t4__blk727_dn11 = assign23780_e32876_d_n11;
        locals.var_t4__blk727_dn12 = assign23780_e32876_d_n12;
        locals.var_t4__blk727_dn17 = assign23780_e32876_d_n17;
        locals.var_t4__blk727_rv = 0.0;

        let (assign23790_e32888, assign23790_e32888_d_n0, assign23790_e32888_d_n2, assign23790_e32888_d_n6, assign23790_e32888_d_n7, assign23790_e32888_d_n10, assign23790_e32888_d_n11, assign23790_e32888_d_n12, assign23790_e32888_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) {
        let assign23790_e32882: f64 = (-p.p214);
        let assign23790_e32884: f64 = (assign23790_e32882 * locals.var_egp32);
        let assign23790_e32886: f64 = (assign23790_e32884 * locals.var_t4__blk727);
        (assign23790_e32886, (((assign23790_e32882 * locals.var_egp32_dn0) * locals.var_t4__blk727) + (assign23790_e32884 * locals.var_t4__blk727_dn0)), (((assign23790_e32882 * locals.var_egp32_dn2) * locals.var_t4__blk727) + (assign23790_e32884 * locals.var_t4__blk727_dn2)), (((assign23790_e32882 * locals.var_egp32_dn6) * locals.var_t4__blk727) + (assign23790_e32884 * locals.var_t4__blk727_dn6)), (((assign23790_e32882 * locals.var_egp32_dn7) * locals.var_t4__blk727) + (assign23790_e32884 * locals.var_t4__blk727_dn7)), (((assign23790_e32882 * locals.var_egp32_dn10) * locals.var_t4__blk727) + (assign23790_e32884 * locals.var_t4__blk727_dn10)), (((assign23790_e32882 * locals.var_egp32_dn11) * locals.var_t4__blk727) + (assign23790_e32884 * locals.var_t4__blk727_dn11)), (((assign23790_e32882 * locals.var_egp32_dn12) * locals.var_t4__blk727) + (assign23790_e32884 * locals.var_t4__blk727_dn12)), (((assign23790_e32882 * locals.var_egp32_dn17) * locals.var_t4__blk727) + (assign23790_e32884 * locals.var_t4__blk727_dn17)),)
    } else {
        (locals.var_t1__blk724, locals.var_t1__blk724_dn0, locals.var_t1__blk724_dn2, locals.var_t1__blk724_dn6, locals.var_t1__blk724_dn7, locals.var_t1__blk724_dn10, locals.var_t1__blk724_dn11, locals.var_t1__blk724_dn12, locals.var_t1__blk724_dn17,)
    }
};
        locals.var_t1__blk724 = assign23790_e32888;
        locals.var_t1__blk724_dn0 = assign23790_e32888_d_n0;
        locals.var_t1__blk724_dn2 = assign23790_e32888_d_n2;
        locals.var_t1__blk724_dn6 = assign23790_e32888_d_n6;
        locals.var_t1__blk724_dn7 = assign23790_e32888_d_n7;
        locals.var_t1__blk724_dn10 = assign23790_e32888_d_n10;
        locals.var_t1__blk724_dn11 = assign23790_e32888_d_n11;
        locals.var_t1__blk724_dn12 = assign23790_e32888_d_n12;
        locals.var_t1__blk724_dn17 = assign23790_e32888_d_n17;
        locals.var_t1__blk724_rv = 0.0;

        let assign23800_e32891: f64 = (-34.0);
        let assign23800_e32892: f64 = if locals.var_t1__blk724 < assign23800_e32891 { 1.0 } else { 0.0 };
        locals.var_guard746 = assign23800_e32892;
        locals.var_guard746_rv = 0.0;

        let (assign23820_e32912, assign23820_e32912_d_n0, assign23820_e32912_d_n2, assign23820_e32912_d_n6, assign23820_e32912_d_n7, assign23820_e32912_d_n10, assign23820_e32912_d_n11, assign23820_e32912_d_n12, assign23820_e32912_d_n17,) = {
    if (((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) && (locals.var_guard746 == 0.0)) {
        let assign23820_e32910: f64 = (locals.var_t1__blk724).exp();
        (assign23820_e32910, (assign23820_e32910 * locals.var_t1__blk724_dn0), (assign23820_e32910 * locals.var_t1__blk724_dn2), (assign23820_e32910 * locals.var_t1__blk724_dn6), (assign23820_e32910 * locals.var_t1__blk724_dn7), (assign23820_e32910 * locals.var_t1__blk724_dn10), (assign23820_e32910 * locals.var_t1__blk724_dn11), (assign23820_e32910 * locals.var_t1__blk724_dn12), (assign23820_e32910 * locals.var_t1__blk724_dn17),)
    } else {
        (locals.var_t2__blk725, locals.var_t2__blk725_dn0, locals.var_t2__blk725_dn2, locals.var_t2__blk725_dn6, locals.var_t2__blk725_dn7, locals.var_t2__blk725_dn10, locals.var_t2__blk725_dn11, locals.var_t2__blk725_dn12, locals.var_t2__blk725_dn17,)
    }
};
        locals.var_t2__blk725 = assign23820_e32912;
        locals.var_t2__blk725_dn0 = assign23820_e32912_d_n0;
        locals.var_t2__blk725_dn2 = assign23820_e32912_d_n2;
        locals.var_t2__blk725_dn6 = assign23820_e32912_d_n6;
        locals.var_t2__blk725_dn7 = assign23820_e32912_d_n7;
        locals.var_t2__blk725_dn10 = assign23820_e32912_d_n10;
        locals.var_t2__blk725_dn11 = assign23820_e32912_d_n11;
        locals.var_t2__blk725_dn12 = assign23820_e32912_d_n12;
        locals.var_t2__blk725_dn17 = assign23820_e32912_d_n17;
        locals.var_t2__blk725_rv = 0.0;

        let (assign23830_e32928, assign23830_e32928_d_n0, assign23830_e32928_d_n2, assign23830_e32928_d_n6, assign23830_e32928_d_n7, assign23830_e32928_d_n10, assign23830_e32928_d_n11, assign23830_e32928_d_n12, assign23830_e32928_d_n17,) = {
    if (((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) && (locals.var_guard746 == 0.0)) {
        let assign23830_e32922: f64 = (p.p213 / locals.var_egp12);
        let assign23830_e32924: f64 = (assign23830_e32922 * 1.6021918e-19);
        let assign23830_e32926: f64 = (assign23830_e32924 * locals.var_t0__blk723);
        (assign23830_e32926, ((((-((p.p213 * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk723) + (assign23830_e32924 * locals.var_t0__blk723_dn0)), ((((-((p.p213 * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk723) + (assign23830_e32924 * locals.var_t0__blk723_dn2)), ((((-((p.p213 * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk723) + (assign23830_e32924 * locals.var_t0__blk723_dn6)), ((((-((p.p213 * locals.var_egp12_dn7) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk723) + (assign23830_e32924 * locals.var_t0__blk723_dn7)), ((((-((p.p213 * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk723) + (assign23830_e32924 * locals.var_t0__blk723_dn10)), ((((-((p.p213 * locals.var_egp12_dn11) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk723) + (assign23830_e32924 * locals.var_t0__blk723_dn11)), ((((-((p.p213 * locals.var_egp12_dn12) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk723) + (assign23830_e32924 * locals.var_t0__blk723_dn12)), ((((-((p.p213 * locals.var_egp12_dn17) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk723) + (assign23830_e32924 * locals.var_t0__blk723_dn17)),)
    } else {
        (locals.var_t3__blk726, locals.var_t3__blk726_dn0, locals.var_t3__blk726_dn2, locals.var_t3__blk726_dn6, locals.var_t3__blk726_dn7, locals.var_t3__blk726_dn10, locals.var_t3__blk726_dn11, locals.var_t3__blk726_dn12, locals.var_t3__blk726_dn17,)
    }
};
        locals.var_t3__blk726 = assign23830_e32928;
        locals.var_t3__blk726_dn0 = assign23830_e32928_d_n0;
        locals.var_t3__blk726_dn2 = assign23830_e32928_d_n2;
        locals.var_t3__blk726_dn6 = assign23830_e32928_d_n6;
        locals.var_t3__blk726_dn7 = assign23830_e32928_d_n7;
        locals.var_t3__blk726_dn10 = assign23830_e32928_d_n10;
        locals.var_t3__blk726_dn11 = assign23830_e32928_d_n11;
        locals.var_t3__blk726_dn12 = assign23830_e32928_d_n12;
        locals.var_t3__blk726_dn17 = assign23830_e32928_d_n17;
        locals.var_t3__blk726_rv = 0.0;

        let (assign23840_e32940, assign23840_e32940_d_n0, assign23840_e32940_d_n2, assign23840_e32940_d_n6, assign23840_e32940_d_n7, assign23840_e32940_d_n10, assign23840_e32940_d_n11, assign23840_e32940_d_n12, assign23840_e32940_d_n17,) = {
    if (((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) && (locals.var_guard746 == 0.0)) {
        let assign23840_e32938: f64 = (1.0 / locals.var_cgs_cnst0soi);
        (assign23840_e32938, (-(locals.var_cgs_cnst0soi_dn0 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn2 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn6 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn7 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn10 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn11 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn12 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))), (-(locals.var_cgs_cnst0soi_dn17 / (locals.var_cgs_cnst0soi * locals.var_cgs_cnst0soi))),)
    } else {
        (locals.var_t5__blk728, locals.var_t5__blk728_dn0, locals.var_t5__blk728_dn2, locals.var_t5__blk728_dn6, locals.var_t5__blk728_dn7, locals.var_t5__blk728_dn10, locals.var_t5__blk728_dn11, locals.var_t5__blk728_dn12, locals.var_t5__blk728_dn17,)
    }
};
        locals.var_t5__blk728 = assign23840_e32940;
        locals.var_t5__blk728_dn0 = assign23840_e32940_d_n0;
        locals.var_t5__blk728_dn2 = assign23840_e32940_d_n2;
        locals.var_t5__blk728_dn6 = assign23840_e32940_d_n6;
        locals.var_t5__blk728_dn7 = assign23840_e32940_d_n7;
        locals.var_t5__blk728_dn10 = assign23840_e32940_d_n10;
        locals.var_t5__blk728_dn11 = assign23840_e32940_d_n11;
        locals.var_t5__blk728_dn12 = assign23840_e32940_d_n12;
        locals.var_t5__blk728_dn17 = assign23840_e32940_d_n17;
        locals.var_t5__blk728_rv = 0.0;

        let (assign23850_e32957, assign23850_e32957_d_n0, assign23850_e32957_d_n2, assign23850_e32957_d_n6, assign23850_e32957_d_n7, assign23850_e32957_d_n10, assign23850_e32957_d_n11, assign23850_e32957_d_n12, assign23850_e32957_d_n17,) = {
    if (((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) && (locals.var_guard746 == 0.0)) {
        let assign23850_e32951: f64 = (locals.var_cgs_c_fox * 1e-12);
        let assign23850_e32952: f64 = (locals.var_cgs_qiu__blk740 + assign23850_e32951);
        let assign23850_e32954: f64 = (assign23850_e32952 * locals.var_t5__blk728);
        let assign23850_e32955: f64 = (assign23850_e32954).sqrt();
        (assign23850_e32955, ((((locals.var_cgs_qiu__blk740_dn0 + (locals.var_cgs_c_fox_dn0 * 1e-12)) * locals.var_t5__blk728) + (assign23850_e32952 * locals.var_t5__blk728_dn0)) / (2.0 * assign23850_e32955)), ((((locals.var_cgs_qiu__blk740_dn2 + (locals.var_cgs_c_fox_dn2 * 1e-12)) * locals.var_t5__blk728) + (assign23850_e32952 * locals.var_t5__blk728_dn2)) / (2.0 * assign23850_e32955)), ((((locals.var_cgs_qiu__blk740_dn6 + (locals.var_cgs_c_fox_dn6 * 1e-12)) * locals.var_t5__blk728) + (assign23850_e32952 * locals.var_t5__blk728_dn6)) / (2.0 * assign23850_e32955)), ((((locals.var_cgs_qiu__blk740_dn7 + (locals.var_cgs_c_fox_dn7 * 1e-12)) * locals.var_t5__blk728) + (assign23850_e32952 * locals.var_t5__blk728_dn7)) / (2.0 * assign23850_e32955)), ((((locals.var_cgs_qiu__blk740_dn10 + (locals.var_cgs_c_fox_dn10 * 1e-12)) * locals.var_t5__blk728) + (assign23850_e32952 * locals.var_t5__blk728_dn10)) / (2.0 * assign23850_e32955)), ((((locals.var_cgs_qiu__blk740_dn11 + (locals.var_cgs_c_fox_dn11 * 1e-12)) * locals.var_t5__blk728) + (assign23850_e32952 * locals.var_t5__blk728_dn11)) / (2.0 * assign23850_e32955)), ((((locals.var_cgs_qiu__blk740_dn12 + (locals.var_cgs_c_fox_dn12 * 1e-12)) * locals.var_t5__blk728) + (assign23850_e32952 * locals.var_t5__blk728_dn12)) / (2.0 * assign23850_e32955)), ((((locals.var_cgs_qiu__blk740_dn17 + (locals.var_cgs_c_fox_dn17 * 1e-12)) * locals.var_t5__blk728) + (assign23850_e32952 * locals.var_t5__blk728_dn17)) / (2.0 * assign23850_e32955)),)
    } else {
        (locals.var_t6__blk729, locals.var_t6__blk729_dn0, locals.var_t6__blk729_dn2, locals.var_t6__blk729_dn6, locals.var_t6__blk729_dn7, locals.var_t6__blk729_dn10, locals.var_t6__blk729_dn11, locals.var_t6__blk729_dn12, locals.var_t6__blk729_dn17,)
    }
};
        locals.var_t6__blk729 = assign23850_e32957;
        locals.var_t6__blk729_dn0 = assign23850_e32957_d_n0;
        locals.var_t6__blk729_dn2 = assign23850_e32957_d_n2;
        locals.var_t6__blk729_dn6 = assign23850_e32957_d_n6;
        locals.var_t6__blk729_dn7 = assign23850_e32957_d_n7;
        locals.var_t6__blk729_dn10 = assign23850_e32957_d_n10;
        locals.var_t6__blk729_dn11 = assign23850_e32957_d_n11;
        locals.var_t6__blk729_dn12 = assign23850_e32957_d_n12;
        locals.var_t6__blk729_dn17 = assign23850_e32957_d_n17;
        locals.var_t6__blk729_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_85(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23860_e32971, assign23860_e32971_d_n0, assign23860_e32971_d_n2, assign23860_e32971_d_n6, assign23860_e32971_d_n7, assign23860_e32971_d_n10, assign23860_e32971_d_n11, assign23860_e32971_d_n12, assign23860_e32971_d_n17,) = {
    if (((locals.var_guard742 == 0.0) && (locals.var_guard743 != 0.0)) && (locals.var_guard746 == 0.0)) {
        let assign23860_e32967: f64 = (locals.var_t2__blk725 * locals.var_t3__blk726);
        let assign23860_e32969: f64 = (assign23860_e32967 * locals.var_t6__blk729);
        (assign23860_e32969, ((((locals.var_t2__blk725_dn0 * locals.var_t3__blk726) + (locals.var_t2__blk725 * locals.var_t3__blk726_dn0)) * locals.var_t6__blk729) + (assign23860_e32967 * locals.var_t6__blk729_dn0)), ((((locals.var_t2__blk725_dn2 * locals.var_t3__blk726) + (locals.var_t2__blk725 * locals.var_t3__blk726_dn2)) * locals.var_t6__blk729) + (assign23860_e32967 * locals.var_t6__blk729_dn2)), ((((locals.var_t2__blk725_dn6 * locals.var_t3__blk726) + (locals.var_t2__blk725 * locals.var_t3__blk726_dn6)) * locals.var_t6__blk729) + (assign23860_e32967 * locals.var_t6__blk729_dn6)), ((((locals.var_t2__blk725_dn7 * locals.var_t3__blk726) + (locals.var_t2__blk725 * locals.var_t3__blk726_dn7)) * locals.var_t6__blk729) + (assign23860_e32967 * locals.var_t6__blk729_dn7)), ((((locals.var_t2__blk725_dn10 * locals.var_t3__blk726) + (locals.var_t2__blk725 * locals.var_t3__blk726_dn10)) * locals.var_t6__blk729) + (assign23860_e32967 * locals.var_t6__blk729_dn10)), ((((locals.var_t2__blk725_dn11 * locals.var_t3__blk726) + (locals.var_t2__blk725 * locals.var_t3__blk726_dn11)) * locals.var_t6__blk729) + (assign23860_e32967 * locals.var_t6__blk729_dn11)), ((((locals.var_t2__blk725_dn12 * locals.var_t3__blk726) + (locals.var_t2__blk725 * locals.var_t3__blk726_dn12)) * locals.var_t6__blk729) + (assign23860_e32967 * locals.var_t6__blk729_dn12)), ((((locals.var_t2__blk725_dn17 * locals.var_t3__blk726) + (locals.var_t2__blk725 * locals.var_t3__blk726_dn17)) * locals.var_t6__blk729) + (assign23860_e32967 * locals.var_t6__blk729_dn17)),)
    } else {
        (locals.var_t4__blk727, locals.var_t4__blk727_dn0, locals.var_t4__blk727_dn2, locals.var_t4__blk727_dn6, locals.var_t4__blk727_dn7, locals.var_t4__blk727_dn10, locals.var_t4__blk727_dn11, locals.var_t4__blk727_dn12, locals.var_t4__blk727_dn17,)
    }
};
        locals.var_t4__blk727 = assign23860_e32971;
        locals.var_t4__blk727_dn0 = assign23860_e32971_d_n0;
        locals.var_t4__blk727_dn2 = assign23860_e32971_d_n2;
        locals.var_t4__blk727_dn6 = assign23860_e32971_d_n6;
        locals.var_t4__blk727_dn7 = assign23860_e32971_d_n7;
        locals.var_t4__blk727_dn10 = assign23860_e32971_d_n10;
        locals.var_t4__blk727_dn11 = assign23860_e32971_d_n11;
        locals.var_t4__blk727_dn12 = assign23860_e32971_d_n12;
        locals.var_t4__blk727_dn17 = assign23860_e32971_d_n17;
        locals.var_t4__blk727_rv = 0.0;

        let (assign23900_e33017, assign23900_e33017_d_n0, assign23900_e33017_d_n2, assign23900_e33017_d_n6, assign23900_e33017_d_n7, assign23900_e33017_d_n10, assign23900_e33017_d_n11, assign23900_e33017_d_n12, assign23900_e33017_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let assign23900_e33011: f64 = (-p.p221);
        let assign23900_e33013: f64 = (assign23900_e33011 * locals.var_vgs);
        let assign23900_e33015: f64 = (assign23900_e33013 + p.p222);
        (assign23900_e33015, 0.0, 0.0, (assign23900_e33011 * locals.var_vgs_dn6), (assign23900_e33011 * locals.var_vgs_dn7), 0.0, (assign23900_e33011 * locals.var_vgs_dn11), 0.0, 0.0,)
    } else {
        (locals.var_t0__blk723, locals.var_t0__blk723_dn0, locals.var_t0__blk723_dn2, locals.var_t0__blk723_dn6, locals.var_t0__blk723_dn7, locals.var_t0__blk723_dn10, locals.var_t0__blk723_dn11, locals.var_t0__blk723_dn12, locals.var_t0__blk723_dn17,)
    }
};
        locals.var_t0__blk723 = assign23900_e33017;
        locals.var_t0__blk723_dn0 = assign23900_e33017_d_n0;
        locals.var_t0__blk723_dn2 = assign23900_e33017_d_n2;
        locals.var_t0__blk723_dn6 = assign23900_e33017_d_n6;
        locals.var_t0__blk723_dn7 = assign23900_e33017_d_n7;
        locals.var_t0__blk723_dn10 = assign23900_e33017_d_n10;
        locals.var_t0__blk723_dn11 = assign23900_e33017_d_n11;
        locals.var_t0__blk723_dn12 = assign23900_e33017_d_n12;
        locals.var_t0__blk723_dn17 = assign23900_e33017_d_n17;
        locals.var_t0__blk723_rv = 0.0;

        let (assign23910_e33025, assign23910_e33025_d_n0, assign23910_e33025_d_n2, assign23910_e33025_d_n6, assign23910_e33025_d_n7, assign23910_e33025_d_n10, assign23910_e33025_d_n11, assign23910_e33025_d_n12, assign23910_e33025_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let assign23910_e33022: f64 = (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723);
        let assign23910_e33023: f64 = (assign23910_e33022).exp();
        (assign23910_e33023, (assign23910_e33023 * (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723_dn0)), (assign23910_e33023 * (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723_dn2)), (assign23910_e33023 * (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723_dn6)), (assign23910_e33023 * (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723_dn7)), (assign23910_e33023 * (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723_dn10)), (assign23910_e33023 * (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723_dn11)), (assign23910_e33023 * (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723_dn12)), (assign23910_e33023 * (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723_dn17)),)
    } else {
        (locals.var_t2__blk725, locals.var_t2__blk725_dn0, locals.var_t2__blk725_dn2, locals.var_t2__blk725_dn6, locals.var_t2__blk725_dn7, locals.var_t2__blk725_dn10, locals.var_t2__blk725_dn11, locals.var_t2__blk725_dn12, locals.var_t2__blk725_dn17,)
    }
};
        locals.var_t2__blk725 = assign23910_e33025;
        locals.var_t2__blk725_dn0 = assign23910_e33025_d_n0;
        locals.var_t2__blk725_dn2 = assign23910_e33025_d_n2;
        locals.var_t2__blk725_dn6 = assign23910_e33025_d_n6;
        locals.var_t2__blk725_dn7 = assign23910_e33025_d_n7;
        locals.var_t2__blk725_dn10 = assign23910_e33025_d_n10;
        locals.var_t2__blk725_dn11 = assign23910_e33025_d_n11;
        locals.var_t2__blk725_dn12 = assign23910_e33025_d_n12;
        locals.var_t2__blk725_dn17 = assign23910_e33025_d_n17;
        locals.var_t2__blk725_rv = 0.0;

        let (assign23920_e33034, assign23920_e33034_d_n0, assign23920_e33034_d_n2, assign23920_e33034_d_n6, assign23920_e33034_d_n7, assign23920_e33034_d_n10, assign23920_e33034_d_n11, assign23920_e33034_d_n12, assign23920_e33034_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_cgs_tfox0__blk735;
        let assign23920_e33030: f64 = (locals.var_vgs * __rspice_inv_cse_0);
        let assign23920_e33032: f64 = (assign23920_e33030 * __rspice_inv_cse_0);
        (assign23920_e33032, 0.0, 0.0, ((locals.var_vgs_dn6 / locals.var_cgs_tfox0__blk735) / locals.var_cgs_tfox0__blk735), ((locals.var_vgs_dn7 / locals.var_cgs_tfox0__blk735) / locals.var_cgs_tfox0__blk735), 0.0, ((locals.var_vgs_dn11 / locals.var_cgs_tfox0__blk735) / locals.var_cgs_tfox0__blk735), 0.0, 0.0,)
    } else {
        (locals.var_t0__blk723, locals.var_t0__blk723_dn0, locals.var_t0__blk723_dn2, locals.var_t0__blk723_dn6, locals.var_t0__blk723_dn7, locals.var_t0__blk723_dn10, locals.var_t0__blk723_dn11, locals.var_t0__blk723_dn12, locals.var_t0__blk723_dn17,)
    }
};
        locals.var_t0__blk723 = assign23920_e33034;
        locals.var_t0__blk723_dn0 = assign23920_e33034_d_n0;
        locals.var_t0__blk723_dn2 = assign23920_e33034_d_n2;
        locals.var_t0__blk723_dn6 = assign23920_e33034_d_n6;
        locals.var_t0__blk723_dn7 = assign23920_e33034_d_n7;
        locals.var_t0__blk723_dn10 = assign23920_e33034_d_n10;
        locals.var_t0__blk723_dn11 = assign23920_e33034_d_n11;
        locals.var_t0__blk723_dn12 = assign23920_e33034_d_n12;
        locals.var_t0__blk723_dn17 = assign23920_e33034_d_n17;
        locals.var_t0__blk723_rv = 0.0;

        let (assign23930_e33041, assign23930_e33041_d_n0, assign23930_e33041_d_n2, assign23930_e33041_d_n6, assign23930_e33041_d_n7, assign23930_e33041_d_n10, assign23930_e33041_d_n11, assign23930_e33041_d_n12, assign23930_e33041_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let assign23930_e33039: f64 = (locals.var_vgs * locals.var_t0__blk723);
        (assign23930_e33039, (locals.var_vgs * locals.var_t0__blk723_dn0), (locals.var_vgs * locals.var_t0__blk723_dn2), ((locals.var_vgs_dn6 * locals.var_t0__blk723) + (locals.var_vgs * locals.var_t0__blk723_dn6)), ((locals.var_vgs_dn7 * locals.var_t0__blk723) + (locals.var_vgs * locals.var_t0__blk723_dn7)), (locals.var_vgs * locals.var_t0__blk723_dn10), ((locals.var_vgs_dn11 * locals.var_t0__blk723) + (locals.var_vgs * locals.var_t0__blk723_dn11)), (locals.var_vgs * locals.var_t0__blk723_dn12), (locals.var_vgs * locals.var_t0__blk723_dn17),)
    } else {
        (locals.var_t3__blk726, locals.var_t3__blk726_dn0, locals.var_t3__blk726_dn2, locals.var_t3__blk726_dn6, locals.var_t3__blk726_dn7, locals.var_t3__blk726_dn10, locals.var_t3__blk726_dn11, locals.var_t3__blk726_dn12, locals.var_t3__blk726_dn17,)
    }
};
        locals.var_t3__blk726 = assign23930_e33041;
        locals.var_t3__blk726_dn0 = assign23930_e33041_d_n0;
        locals.var_t3__blk726_dn2 = assign23930_e33041_d_n2;
        locals.var_t3__blk726_dn6 = assign23930_e33041_d_n6;
        locals.var_t3__blk726_dn7 = assign23930_e33041_d_n7;
        locals.var_t3__blk726_dn10 = assign23930_e33041_d_n10;
        locals.var_t3__blk726_dn11 = assign23930_e33041_d_n11;
        locals.var_t3__blk726_dn12 = assign23930_e33041_d_n12;
        locals.var_t3__blk726_dn17 = assign23930_e33041_d_n17;
        locals.var_t3__blk726_rv = 0.0;

        let (assign23940_e33050, assign23940_e33050_d_n0, assign23940_e33050_d_n2, assign23940_e33050_d_n6, assign23940_e33050_d_n7, assign23940_e33050_d_n10, assign23940_e33050_d_n11, assign23940_e33050_d_n12, assign23940_e33050_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let assign23940_e33046: f64 = (p.p220 / 1000000.0);
        let assign23940_e33048: f64 = (assign23940_e33046 * locals.var_cgs_weff_nf__blk738);
        (assign23940_e33048, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk727, locals.var_t4__blk727_dn0, locals.var_t4__blk727_dn2, locals.var_t4__blk727_dn6, locals.var_t4__blk727_dn7, locals.var_t4__blk727_dn10, locals.var_t4__blk727_dn11, locals.var_t4__blk727_dn12, locals.var_t4__blk727_dn17,)
    }
};
        locals.var_t4__blk727 = assign23940_e33050;
        locals.var_t4__blk727_dn0 = assign23940_e33050_d_n0;
        locals.var_t4__blk727_dn2 = assign23940_e33050_d_n2;
        locals.var_t4__blk727_dn6 = assign23940_e33050_d_n6;
        locals.var_t4__blk727_dn7 = assign23940_e33050_d_n7;
        locals.var_t4__blk727_dn10 = assign23940_e33050_d_n10;
        locals.var_t4__blk727_dn11 = assign23940_e33050_d_n11;
        locals.var_t4__blk727_dn12 = assign23940_e33050_d_n12;
        locals.var_t4__blk727_dn17 = assign23940_e33050_d_n17;
        locals.var_t4__blk727_rv = 0.0;

        let (assign23980_e33079, assign23980_e33079_d_n0, assign23980_e33079_d_n2, assign23980_e33079_d_n6, assign23980_e33079_d_n7, assign23980_e33079_d_n10, assign23980_e33079_d_n11, assign23980_e33079_d_n12, assign23980_e33079_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let assign23980_e33077: f64 = (locals.var_vgs - locals.var_vds);
        (assign23980_e33077, (-locals.var_vds_dn0), (-locals.var_vds_dn2), (locals.var_vgs_dn6 - locals.var_vds_dn6), (locals.var_vgs_dn7 - locals.var_vds_dn7), (-locals.var_vds_dn10), (locals.var_vgs_dn11 - locals.var_vds_dn11), (-locals.var_vds_dn12), (-locals.var_vds_dn17),)
    } else {
        (locals.var_t1__blk724, locals.var_t1__blk724_dn0, locals.var_t1__blk724_dn2, locals.var_t1__blk724_dn6, locals.var_t1__blk724_dn7, locals.var_t1__blk724_dn10, locals.var_t1__blk724_dn11, locals.var_t1__blk724_dn12, locals.var_t1__blk724_dn17,)
    }
};
        locals.var_t1__blk724 = assign23980_e33079;
        locals.var_t1__blk724_dn0 = assign23980_e33079_d_n0;
        locals.var_t1__blk724_dn2 = assign23980_e33079_d_n2;
        locals.var_t1__blk724_dn6 = assign23980_e33079_d_n6;
        locals.var_t1__blk724_dn7 = assign23980_e33079_d_n7;
        locals.var_t1__blk724_dn10 = assign23980_e33079_d_n10;
        locals.var_t1__blk724_dn11 = assign23980_e33079_d_n11;
        locals.var_t1__blk724_dn12 = assign23980_e33079_d_n12;
        locals.var_t1__blk724_dn17 = assign23980_e33079_d_n17;
        locals.var_t1__blk724_rv = 0.0;

        let (assign23990_e33089, assign23990_e33089_d_n0, assign23990_e33089_d_n2, assign23990_e33089_d_n6, assign23990_e33089_d_n7, assign23990_e33089_d_n10, assign23990_e33089_d_n11, assign23990_e33089_d_n12, assign23990_e33089_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let assign23990_e33083: f64 = (-p.p221);
        let assign23990_e33085: f64 = (assign23990_e33083 * locals.var_t1__blk724);
        let assign23990_e33087: f64 = (assign23990_e33085 + p.p222);
        (assign23990_e33087, (assign23990_e33083 * locals.var_t1__blk724_dn0), (assign23990_e33083 * locals.var_t1__blk724_dn2), (assign23990_e33083 * locals.var_t1__blk724_dn6), (assign23990_e33083 * locals.var_t1__blk724_dn7), (assign23990_e33083 * locals.var_t1__blk724_dn10), (assign23990_e33083 * locals.var_t1__blk724_dn11), (assign23990_e33083 * locals.var_t1__blk724_dn12), (assign23990_e33083 * locals.var_t1__blk724_dn17),)
    } else {
        (locals.var_t0__blk723, locals.var_t0__blk723_dn0, locals.var_t0__blk723_dn2, locals.var_t0__blk723_dn6, locals.var_t0__blk723_dn7, locals.var_t0__blk723_dn10, locals.var_t0__blk723_dn11, locals.var_t0__blk723_dn12, locals.var_t0__blk723_dn17,)
    }
};
        locals.var_t0__blk723 = assign23990_e33089;
        locals.var_t0__blk723_dn0 = assign23990_e33089_d_n0;
        locals.var_t0__blk723_dn2 = assign23990_e33089_d_n2;
        locals.var_t0__blk723_dn6 = assign23990_e33089_d_n6;
        locals.var_t0__blk723_dn7 = assign23990_e33089_d_n7;
        locals.var_t0__blk723_dn10 = assign23990_e33089_d_n10;
        locals.var_t0__blk723_dn11 = assign23990_e33089_d_n11;
        locals.var_t0__blk723_dn12 = assign23990_e33089_d_n12;
        locals.var_t0__blk723_dn17 = assign23990_e33089_d_n17;
        locals.var_t0__blk723_rv = 0.0;

        let (assign24000_e33097, assign24000_e33097_d_n0, assign24000_e33097_d_n2, assign24000_e33097_d_n6, assign24000_e33097_d_n7, assign24000_e33097_d_n10, assign24000_e33097_d_n11, assign24000_e33097_d_n12, assign24000_e33097_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let assign24000_e33094: f64 = (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723);
        let assign24000_e33095: f64 = (assign24000_e33094).exp();
        (assign24000_e33095, (assign24000_e33095 * (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723_dn0)), (assign24000_e33095 * (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723_dn2)), (assign24000_e33095 * (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723_dn6)), (assign24000_e33095 * (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723_dn7)), (assign24000_e33095 * (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723_dn10)), (assign24000_e33095 * (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723_dn11)), (assign24000_e33095 * (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723_dn12)), (assign24000_e33095 * (locals.var_cgs_tfox0__blk735 * locals.var_t0__blk723_dn17)),)
    } else {
        (locals.var_t2__blk725, locals.var_t2__blk725_dn0, locals.var_t2__blk725_dn2, locals.var_t2__blk725_dn6, locals.var_t2__blk725_dn7, locals.var_t2__blk725_dn10, locals.var_t2__blk725_dn11, locals.var_t2__blk725_dn12, locals.var_t2__blk725_dn17,)
    }
};
        locals.var_t2__blk725 = assign24000_e33097;
        locals.var_t2__blk725_dn0 = assign24000_e33097_d_n0;
        locals.var_t2__blk725_dn2 = assign24000_e33097_d_n2;
        locals.var_t2__blk725_dn6 = assign24000_e33097_d_n6;
        locals.var_t2__blk725_dn7 = assign24000_e33097_d_n7;
        locals.var_t2__blk725_dn10 = assign24000_e33097_d_n10;
        locals.var_t2__blk725_dn11 = assign24000_e33097_d_n11;
        locals.var_t2__blk725_dn12 = assign24000_e33097_d_n12;
        locals.var_t2__blk725_dn17 = assign24000_e33097_d_n17;
        locals.var_t2__blk725_rv = 0.0;

        let (assign24010_e33106, assign24010_e33106_d_n0, assign24010_e33106_d_n2, assign24010_e33106_d_n6, assign24010_e33106_d_n7, assign24010_e33106_d_n10, assign24010_e33106_d_n11, assign24010_e33106_d_n12, assign24010_e33106_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_cgs_tfox0__blk735;
        let assign24010_e33102: f64 = (locals.var_t1__blk724 * __rspice_inv_cse_1);
        let assign24010_e33104: f64 = (assign24010_e33102 * __rspice_inv_cse_1);
        (assign24010_e33104, ((locals.var_t1__blk724_dn0 / locals.var_cgs_tfox0__blk735) / locals.var_cgs_tfox0__blk735), ((locals.var_t1__blk724_dn2 / locals.var_cgs_tfox0__blk735) / locals.var_cgs_tfox0__blk735), ((locals.var_t1__blk724_dn6 / locals.var_cgs_tfox0__blk735) / locals.var_cgs_tfox0__blk735), ((locals.var_t1__blk724_dn7 / locals.var_cgs_tfox0__blk735) / locals.var_cgs_tfox0__blk735), ((locals.var_t1__blk724_dn10 / locals.var_cgs_tfox0__blk735) / locals.var_cgs_tfox0__blk735), ((locals.var_t1__blk724_dn11 / locals.var_cgs_tfox0__blk735) / locals.var_cgs_tfox0__blk735), ((locals.var_t1__blk724_dn12 / locals.var_cgs_tfox0__blk735) / locals.var_cgs_tfox0__blk735), ((locals.var_t1__blk724_dn17 / locals.var_cgs_tfox0__blk735) / locals.var_cgs_tfox0__blk735),)
    } else {
        (locals.var_t0__blk723, locals.var_t0__blk723_dn0, locals.var_t0__blk723_dn2, locals.var_t0__blk723_dn6, locals.var_t0__blk723_dn7, locals.var_t0__blk723_dn10, locals.var_t0__blk723_dn11, locals.var_t0__blk723_dn12, locals.var_t0__blk723_dn17,)
    }
};
        locals.var_t0__blk723 = assign24010_e33106;
        locals.var_t0__blk723_dn0 = assign24010_e33106_d_n0;
        locals.var_t0__blk723_dn2 = assign24010_e33106_d_n2;
        locals.var_t0__blk723_dn6 = assign24010_e33106_d_n6;
        locals.var_t0__blk723_dn7 = assign24010_e33106_d_n7;
        locals.var_t0__blk723_dn10 = assign24010_e33106_d_n10;
        locals.var_t0__blk723_dn11 = assign24010_e33106_d_n11;
        locals.var_t0__blk723_dn12 = assign24010_e33106_d_n12;
        locals.var_t0__blk723_dn17 = assign24010_e33106_d_n17;
        locals.var_t0__blk723_rv = 0.0;

        let (assign24020_e33113, assign24020_e33113_d_n0, assign24020_e33113_d_n2, assign24020_e33113_d_n6, assign24020_e33113_d_n7, assign24020_e33113_d_n10, assign24020_e33113_d_n11, assign24020_e33113_d_n12, assign24020_e33113_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let assign24020_e33111: f64 = (locals.var_t1__blk724 * locals.var_t0__blk723);
        (assign24020_e33111, ((locals.var_t1__blk724_dn0 * locals.var_t0__blk723) + (locals.var_t1__blk724 * locals.var_t0__blk723_dn0)), ((locals.var_t1__blk724_dn2 * locals.var_t0__blk723) + (locals.var_t1__blk724 * locals.var_t0__blk723_dn2)), ((locals.var_t1__blk724_dn6 * locals.var_t0__blk723) + (locals.var_t1__blk724 * locals.var_t0__blk723_dn6)), ((locals.var_t1__blk724_dn7 * locals.var_t0__blk723) + (locals.var_t1__blk724 * locals.var_t0__blk723_dn7)), ((locals.var_t1__blk724_dn10 * locals.var_t0__blk723) + (locals.var_t1__blk724 * locals.var_t0__blk723_dn10)), ((locals.var_t1__blk724_dn11 * locals.var_t0__blk723) + (locals.var_t1__blk724 * locals.var_t0__blk723_dn11)), ((locals.var_t1__blk724_dn12 * locals.var_t0__blk723) + (locals.var_t1__blk724 * locals.var_t0__blk723_dn12)), ((locals.var_t1__blk724_dn17 * locals.var_t0__blk723) + (locals.var_t1__blk724 * locals.var_t0__blk723_dn17)),)
    } else {
        (locals.var_t3__blk726, locals.var_t3__blk726_dn0, locals.var_t3__blk726_dn2, locals.var_t3__blk726_dn6, locals.var_t3__blk726_dn7, locals.var_t3__blk726_dn10, locals.var_t3__blk726_dn11, locals.var_t3__blk726_dn12, locals.var_t3__blk726_dn17,)
    }
};
        locals.var_t3__blk726 = assign24020_e33113;
        locals.var_t3__blk726_dn0 = assign24020_e33113_d_n0;
        locals.var_t3__blk726_dn2 = assign24020_e33113_d_n2;
        locals.var_t3__blk726_dn6 = assign24020_e33113_d_n6;
        locals.var_t3__blk726_dn7 = assign24020_e33113_d_n7;
        locals.var_t3__blk726_dn10 = assign24020_e33113_d_n10;
        locals.var_t3__blk726_dn11 = assign24020_e33113_d_n11;
        locals.var_t3__blk726_dn12 = assign24020_e33113_d_n12;
        locals.var_t3__blk726_dn17 = assign24020_e33113_d_n17;
        locals.var_t3__blk726_rv = 0.0;

        let (assign24030_e33122, assign24030_e33122_d_n0, assign24030_e33122_d_n2, assign24030_e33122_d_n6, assign24030_e33122_d_n7, assign24030_e33122_d_n10, assign24030_e33122_d_n11, assign24030_e33122_d_n12, assign24030_e33122_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let assign24030_e33118: f64 = (p.p220 / 1000000.0);
        let assign24030_e33120: f64 = (assign24030_e33118 * locals.var_cgs_weff_nf__blk738);
        (assign24030_e33120, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk727, locals.var_t4__blk727_dn0, locals.var_t4__blk727_dn2, locals.var_t4__blk727_dn6, locals.var_t4__blk727_dn7, locals.var_t4__blk727_dn10, locals.var_t4__blk727_dn11, locals.var_t4__blk727_dn12, locals.var_t4__blk727_dn17,)
    }
};
        locals.var_t4__blk727 = assign24030_e33122;
        locals.var_t4__blk727_dn0 = assign24030_e33122_d_n0;
        locals.var_t4__blk727_dn2 = assign24030_e33122_d_n2;
        locals.var_t4__blk727_dn6 = assign24030_e33122_d_n6;
        locals.var_t4__blk727_dn7 = assign24030_e33122_d_n7;
        locals.var_t4__blk727_dn10 = assign24030_e33122_d_n10;
        locals.var_t4__blk727_dn11 = assign24030_e33122_d_n11;
        locals.var_t4__blk727_dn12 = assign24030_e33122_d_n12;
        locals.var_t4__blk727_dn17 = assign24030_e33122_d_n17;
        locals.var_t4__blk727_rv = 0.0;

        let (assign24070_e33158, assign24070_e33158_d_n0, assign24070_e33158_d_n2, assign24070_e33158_d_n6, assign24070_e33158_d_n7, assign24070_e33158_d_n10, assign24070_e33158_d_n11, assign24070_e33158_d_n12, assign24070_e33158_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let assign24070_e33148: f64 = (-locals.var_vgs);
        let assign24070_e33150: f64 = (assign24070_e33148 + locals.var_vbsp);
        let assign24070_e33152: f64 = (assign24070_e33150 + locals.var_vfb);
        let assign24070_e33154: f64 = (assign24070_e33152 + p.p225);
        let assign24070_e33156: f64 = (assign24070_e33154 / locals.var_cgs_tfox0__blk735);
        (assign24070_e33156, (locals.var_vbsp_dn0 / locals.var_cgs_tfox0__blk735), (locals.var_vbsp_dn2 / locals.var_cgs_tfox0__blk735), (((-locals.var_vgs_dn6) + locals.var_vbsp_dn6) / locals.var_cgs_tfox0__blk735), (((-locals.var_vgs_dn7) + locals.var_vbsp_dn7) / locals.var_cgs_tfox0__blk735), (locals.var_vbsp_dn10 / locals.var_cgs_tfox0__blk735), (((-locals.var_vgs_dn11) + locals.var_vbsp_dn11) / locals.var_cgs_tfox0__blk735), (locals.var_vbsp_dn12 / locals.var_cgs_tfox0__blk735), (locals.var_vbsp_dn17 / locals.var_cgs_tfox0__blk735),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign24070_e33158;
        locals.var_etun_dn0 = assign24070_e33158_d_n0;
        locals.var_etun_dn2 = assign24070_e33158_d_n2;
        locals.var_etun_dn6 = assign24070_e33158_d_n6;
        locals.var_etun_dn7 = assign24070_e33158_d_n7;
        locals.var_etun_dn10 = assign24070_e33158_d_n10;
        locals.var_etun_dn11 = assign24070_e33158_d_n11;
        locals.var_etun_dn12 = assign24070_e33158_d_n12;
        locals.var_etun_dn17 = assign24070_e33158_d_n17;
        locals.var_etun_rv = 0.0;

        let (assign24080_e33172, assign24080_e33172_d_n0, assign24080_e33172_d_n2, assign24080_e33172_d_n6, assign24080_e33172_d_n7, assign24080_e33172_d_n10, assign24080_e33172_d_n11, assign24080_e33172_d_n12, assign24080_e33172_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let assign24080_e33163: f64 = (locals.var_etun * locals.var_etun);
        let assign24080_e33166: f64 = (4.0 * 0.01);
        let assign24080_e33168: f64 = (assign24080_e33166 * 0.01);
        let assign24080_e33169: f64 = (assign24080_e33163 + assign24080_e33168);
        let assign24080_e33170: f64 = (assign24080_e33169).sqrt();
        (assign24080_e33170, (((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0)) / (2.0 * assign24080_e33170)), (((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2)) / (2.0 * assign24080_e33170)), (((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6)) / (2.0 * assign24080_e33170)), (((locals.var_etun_dn7 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn7)) / (2.0 * assign24080_e33170)), (((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10)) / (2.0 * assign24080_e33170)), (((locals.var_etun_dn11 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn11)) / (2.0 * assign24080_e33170)), (((locals.var_etun_dn12 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn12)) / (2.0 * assign24080_e33170)), (((locals.var_etun_dn17 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn17)) / (2.0 * assign24080_e33170)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign24080_e33172;
        locals.var_tmf1_dn0 = assign24080_e33172_d_n0;
        locals.var_tmf1_dn2 = assign24080_e33172_d_n2;
        locals.var_tmf1_dn6 = assign24080_e33172_d_n6;
        locals.var_tmf1_dn7 = assign24080_e33172_d_n7;
        locals.var_tmf1_dn10 = assign24080_e33172_d_n10;
        locals.var_tmf1_dn11 = assign24080_e33172_d_n11;
        locals.var_tmf1_dn12 = assign24080_e33172_d_n12;
        locals.var_tmf1_dn17 = assign24080_e33172_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign24090_e33185, assign24090_e33185_d_n0, assign24090_e33185_d_n2, assign24090_e33185_d_n6, assign24090_e33185_d_n7, assign24090_e33185_d_n10, assign24090_e33185_d_n11, assign24090_e33185_d_n12, assign24090_e33185_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let assign24090_e33178: f64 = (locals.var_etun + locals.var_tmf1);
        let assign24090_e33179: f64 = (0.5 * assign24090_e33178);
        let assign24090_e33182: f64 = (1e-10 * 0.01);
        let assign24090_e33183: f64 = (assign24090_e33179 + assign24090_e33182);
        (assign24090_e33183, (0.5 * (locals.var_etun_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_etun_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_etun_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_etun_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_etun_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_etun_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_etun_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_etun_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign24090_e33185;
        locals.var_etun_dn0 = assign24090_e33185_d_n0;
        locals.var_etun_dn2 = assign24090_e33185_d_n2;
        locals.var_etun_dn6 = assign24090_e33185_d_n6;
        locals.var_etun_dn7 = assign24090_e33185_d_n7;
        locals.var_etun_dn10 = assign24090_e33185_d_n10;
        locals.var_etun_dn11 = assign24090_e33185_d_n11;
        locals.var_etun_dn12 = assign24090_e33185_d_n12;
        locals.var_etun_dn17 = assign24090_e33185_d_n17;
        locals.var_etun_rv = 0.0;

        let assign24100_e33188: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard749 = assign24100_e33188;
        locals.var_guard749_rv = 0.0;

        let (assign24110_e33195, assign24110_e33195_d_n0, assign24110_e33195_d_n2, assign24110_e33195_d_n6, assign24110_e33195_d_n7, assign24110_e33195_d_n10, assign24110_e33195_d_n11, assign24110_e33195_d_n12, assign24110_e33195_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard749 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign24110_e33195;
        locals.var_etun_dn0 = assign24110_e33195_d_n0;
        locals.var_etun_dn2 = assign24110_e33195_d_n2;
        locals.var_etun_dn6 = assign24110_e33195_d_n6;
        locals.var_etun_dn7 = assign24110_e33195_d_n7;
        locals.var_etun_dn10 = assign24110_e33195_d_n10;
        locals.var_etun_dn11 = assign24110_e33195_d_n11;
        locals.var_etun_dn12 = assign24110_e33195_d_n12;
        locals.var_etun_dn17 = assign24110_e33195_d_n17;
        locals.var_etun_rv = 0.0;

        let (assign24120_e33202, assign24120_e33202_d_n0, assign24120_e33202_d_n2, assign24120_e33202_d_n6, assign24120_e33202_d_n7, assign24120_e33202_d_n10, assign24120_e33202_d_n11, assign24120_e33202_d_n12, assign24120_e33202_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let assign24120_e33200: f64 = (locals.var_etun + 1e-50);
        (assign24120_e33200, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn6, locals.var_etun_dn7, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12, locals.var_etun_dn17,)
    }
};
        locals.var_etun = assign24120_e33202;
        locals.var_etun_dn0 = assign24120_e33202_d_n0;
        locals.var_etun_dn2 = assign24120_e33202_d_n2;
        locals.var_etun_dn6 = assign24120_e33202_d_n6;
        locals.var_etun_dn7 = assign24120_e33202_d_n7;
        locals.var_etun_dn10 = assign24120_e33202_d_n10;
        locals.var_etun_dn11 = assign24120_e33202_d_n11;
        locals.var_etun_dn12 = assign24120_e33202_d_n12;
        locals.var_etun_dn17 = assign24120_e33202_d_n17;
        locals.var_etun_rv = 0.0;

        let (assign24130_e33210, assign24130_e33210_d_n0, assign24130_e33210_d_n2, assign24130_e33210_d_n6, assign24130_e33210_d_n7, assign24130_e33210_d_n10, assign24130_e33210_d_n11, assign24130_e33210_d_n12, assign24130_e33210_d_n17,) = {
    if (locals.var_guard742 == 0.0) {
        let assign24130_e33206: f64 = (-p.p224);
        let assign24130_e33208: f64 = (assign24130_e33206 / locals.var_etun);
        (assign24130_e33208, (-((assign24130_e33206 * locals.var_etun_dn0) / (locals.var_etun * locals.var_etun))), (-((assign24130_e33206 * locals.var_etun_dn2) / (locals.var_etun * locals.var_etun))), (-((assign24130_e33206 * locals.var_etun_dn6) / (locals.var_etun * locals.var_etun))), (-((assign24130_e33206 * locals.var_etun_dn7) / (locals.var_etun * locals.var_etun))), (-((assign24130_e33206 * locals.var_etun_dn10) / (locals.var_etun * locals.var_etun))), (-((assign24130_e33206 * locals.var_etun_dn11) / (locals.var_etun * locals.var_etun))), (-((assign24130_e33206 * locals.var_etun_dn12) / (locals.var_etun * locals.var_etun))), (-((assign24130_e33206 * locals.var_etun_dn17) / (locals.var_etun * locals.var_etun))),)
    } else {
        (locals.var_t1__blk724, locals.var_t1__blk724_dn0, locals.var_t1__blk724_dn2, locals.var_t1__blk724_dn6, locals.var_t1__blk724_dn7, locals.var_t1__blk724_dn10, locals.var_t1__blk724_dn11, locals.var_t1__blk724_dn12, locals.var_t1__blk724_dn17,)
    }
};
        locals.var_t1__blk724 = assign24130_e33210;
        locals.var_t1__blk724_dn0 = assign24130_e33210_d_n0;
        locals.var_t1__blk724_dn2 = assign24130_e33210_d_n2;
        locals.var_t1__blk724_dn6 = assign24130_e33210_d_n6;
        locals.var_t1__blk724_dn7 = assign24130_e33210_d_n7;
        locals.var_t1__blk724_dn10 = assign24130_e33210_d_n10;
        locals.var_t1__blk724_dn11 = assign24130_e33210_d_n11;
        locals.var_t1__blk724_dn12 = assign24130_e33210_d_n12;
        locals.var_t1__blk724_dn17 = assign24130_e33210_d_n17;
        locals.var_t1__blk724_rv = 0.0;

        let assign24140_e33213: f64 = (-34.0);
        let assign24140_e33214: f64 = if locals.var_t1__blk724 < assign24140_e33213 { 1.0 } else { 0.0 };
        locals.var_guard750 = assign24140_e33214;
        locals.var_guard750_rv = 0.0;

        let (assign24160_e33230, assign24160_e33230_d_n0, assign24160_e33230_d_n2, assign24160_e33230_d_n6, assign24160_e33230_d_n7, assign24160_e33230_d_n10, assign24160_e33230_d_n11, assign24160_e33230_d_n12, assign24160_e33230_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign24160_e33228: f64 = (locals.var_t1__blk724).exp();
        (assign24160_e33228, (assign24160_e33228 * locals.var_t1__blk724_dn0), (assign24160_e33228 * locals.var_t1__blk724_dn2), (assign24160_e33228 * locals.var_t1__blk724_dn6), (assign24160_e33228 * locals.var_t1__blk724_dn7), (assign24160_e33228 * locals.var_t1__blk724_dn10), (assign24160_e33228 * locals.var_t1__blk724_dn11), (assign24160_e33228 * locals.var_t1__blk724_dn12), (assign24160_e33228 * locals.var_t1__blk724_dn17),)
    } else {
        (locals.var_t2__blk725, locals.var_t2__blk725_dn0, locals.var_t2__blk725_dn2, locals.var_t2__blk725_dn6, locals.var_t2__blk725_dn7, locals.var_t2__blk725_dn10, locals.var_t2__blk725_dn11, locals.var_t2__blk725_dn12, locals.var_t2__blk725_dn17,)
    }
};
        locals.var_t2__blk725 = assign24160_e33230;
        locals.var_t2__blk725_dn0 = assign24160_e33230_d_n0;
        locals.var_t2__blk725_dn2 = assign24160_e33230_d_n2;
        locals.var_t2__blk725_dn6 = assign24160_e33230_d_n6;
        locals.var_t2__blk725_dn7 = assign24160_e33230_d_n7;
        locals.var_t2__blk725_dn10 = assign24160_e33230_d_n10;
        locals.var_t2__blk725_dn11 = assign24160_e33230_d_n11;
        locals.var_t2__blk725_dn12 = assign24160_e33230_d_n12;
        locals.var_t2__blk725_dn17 = assign24160_e33230_d_n17;
        locals.var_t2__blk725_rv = 0.0;

        let (assign24170_e33242, assign24170_e33242_d_n0, assign24170_e33242_d_n2, assign24170_e33242_d_n6, assign24170_e33242_d_n7, assign24170_e33242_d_n10, assign24170_e33242_d_n11, assign24170_e33242_d_n12, assign24170_e33242_d_n17,) = {
    if ((locals.var_guard742 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign24170_e33238: f64 = (p.p223 * locals.var_cgs_weff_nf__blk738);
        let assign24170_e33240: f64 = (assign24170_e33238 * locals.var_cgs_leff__blk737);
        (assign24170_e33240, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk726, locals.var_t3__blk726_dn0, locals.var_t3__blk726_dn2, locals.var_t3__blk726_dn6, locals.var_t3__blk726_dn7, locals.var_t3__blk726_dn10, locals.var_t3__blk726_dn11, locals.var_t3__blk726_dn12, locals.var_t3__blk726_dn17,)
    }
};
        locals.var_t3__blk726 = assign24170_e33242;
        locals.var_t3__blk726_dn0 = assign24170_e33242_d_n0;
        locals.var_t3__blk726_dn2 = assign24170_e33242_d_n2;
        locals.var_t3__blk726_dn6 = assign24170_e33242_d_n6;
        locals.var_t3__blk726_dn7 = assign24170_e33242_d_n7;
        locals.var_t3__blk726_dn10 = assign24170_e33242_d_n10;
        locals.var_t3__blk726_dn11 = assign24170_e33242_d_n11;
        locals.var_t3__blk726_dn12 = assign24170_e33242_d_n12;
        locals.var_t3__blk726_dn17 = assign24170_e33242_d_n17;
        locals.var_t3__blk726_rv = 0.0;

        let assign24200_e33264: f64 = if p.p28 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard758 = assign24200_e33264;
        locals.var_guard758_rv = 0.0;

        let (assign24220_e33285, assign24220_e33285_d_n0, assign24220_e33285_d_n2, assign24220_e33285_d_n6, assign24220_e33285_d_n7, assign24220_e33285_d_n10, assign24220_e33285_d_n11, assign24220_e33285_d_n12, assign24220_e33285_d_n17,) = {
    if (locals.var_guard758 == 0.0) {
        let assign24220_e33274: f64 = (locals.var_vds + p.p210);
        let assign24220_e33275: f64 = (p.p209 * assign24220_e33274);
        let assign24220_e33277: f64 = (assign24220_e33275 - locals.var_vgs);
        let assign24220_e33280: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign24220_e33282: f64 = (assign24220_e33280 * p.p211);
        let assign24220_e33283: f64 = (assign24220_e33277 + assign24220_e33282);
        (assign24220_e33283, ((p.p209 * locals.var_vds_dn0) + ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) * p.p211)), ((p.p209 * locals.var_vds_dn2) + ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) * p.p211)), (((p.p209 * locals.var_vds_dn6) - locals.var_vgs_dn6) + ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) * p.p211)), (((p.p209 * locals.var_vds_dn7) - locals.var_vgs_dn7) + ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) * p.p211)), ((p.p209 * locals.var_vds_dn10) + ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) * p.p211)), (((p.p209 * locals.var_vds_dn11) - locals.var_vgs_dn11) + ((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) * p.p211)), ((p.p209 * locals.var_vds_dn12) + ((locals.var_dvthsc_dn12 + locals.var_dvthlp_dn12) * p.p211)), ((p.p209 * locals.var_vds_dn17) + ((locals.var_dvthsc_dn17 + locals.var_dvthlp_dn17) * p.p211)),)
    } else {
        (locals.var_t1__blk751, locals.var_t1__blk751_dn0, locals.var_t1__blk751_dn2, locals.var_t1__blk751_dn6, locals.var_t1__blk751_dn7, locals.var_t1__blk751_dn10, locals.var_t1__blk751_dn11, locals.var_t1__blk751_dn12, locals.var_t1__blk751_dn17,)
    }
};
        locals.var_t1__blk751 = assign24220_e33285;
        locals.var_t1__blk751_dn0 = assign24220_e33285_d_n0;
        locals.var_t1__blk751_dn2 = assign24220_e33285_d_n2;
        locals.var_t1__blk751_dn6 = assign24220_e33285_d_n6;
        locals.var_t1__blk751_dn7 = assign24220_e33285_d_n7;
        locals.var_t1__blk751_dn10 = assign24220_e33285_d_n10;
        locals.var_t1__blk751_dn11 = assign24220_e33285_d_n11;
        locals.var_t1__blk751_dn12 = assign24220_e33285_d_n12;
        locals.var_t1__blk751_dn17 = assign24220_e33285_d_n17;
        locals.var_t1__blk751_rv = 0.0;

        let (assign24230_e33292, assign24230_e33292_d_n0, assign24230_e33292_d_n2, assign24230_e33292_d_n6, assign24230_e33292_d_n7, assign24230_e33292_d_n10, assign24230_e33292_d_n11, assign24230_e33292_d_n12, assign24230_e33292_d_n17,) = {
    if (locals.var_guard758 == 0.0) {
        let assign24230_e33290: f64 = (1.0 / locals.var_tfox0);
        (assign24230_e33290, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk752, locals.var_t2__blk752_dn0, locals.var_t2__blk752_dn2, locals.var_t2__blk752_dn6, locals.var_t2__blk752_dn7, locals.var_t2__blk752_dn10, locals.var_t2__blk752_dn11, locals.var_t2__blk752_dn12, locals.var_t2__blk752_dn17,)
    }
};
        locals.var_t2__blk752 = assign24230_e33292;
        locals.var_t2__blk752_dn0 = assign24230_e33292_d_n0;
        locals.var_t2__blk752_dn2 = assign24230_e33292_d_n2;
        locals.var_t2__blk752_dn6 = assign24230_e33292_d_n6;
        locals.var_t2__blk752_dn7 = assign24230_e33292_d_n7;
        locals.var_t2__blk752_dn10 = assign24230_e33292_d_n10;
        locals.var_t2__blk752_dn11 = assign24230_e33292_d_n11;
        locals.var_t2__blk752_dn12 = assign24230_e33292_d_n12;
        locals.var_t2__blk752_dn17 = assign24230_e33292_d_n17;
        locals.var_t2__blk752_rv = 0.0;

        let (assign24240_e33299, assign24240_e33299_d_n0, assign24240_e33299_d_n2, assign24240_e33299_d_n6, assign24240_e33299_d_n7, assign24240_e33299_d_n10, assign24240_e33299_d_n11, assign24240_e33299_d_n12, assign24240_e33299_d_n17,) = {
    if (locals.var_guard758 == 0.0) {
        let assign24240_e33297: f64 = (locals.var_t1__blk751 * locals.var_t2__blk752);
        (assign24240_e33297, ((locals.var_t1__blk751_dn0 * locals.var_t2__blk752) + (locals.var_t1__blk751 * locals.var_t2__blk752_dn0)), ((locals.var_t1__blk751_dn2 * locals.var_t2__blk752) + (locals.var_t1__blk751 * locals.var_t2__blk752_dn2)), ((locals.var_t1__blk751_dn6 * locals.var_t2__blk752) + (locals.var_t1__blk751 * locals.var_t2__blk752_dn6)), ((locals.var_t1__blk751_dn7 * locals.var_t2__blk752) + (locals.var_t1__blk751 * locals.var_t2__blk752_dn7)), ((locals.var_t1__blk751_dn10 * locals.var_t2__blk752) + (locals.var_t1__blk751 * locals.var_t2__blk752_dn10)), ((locals.var_t1__blk751_dn11 * locals.var_t2__blk752) + (locals.var_t1__blk751 * locals.var_t2__blk752_dn11)), ((locals.var_t1__blk751_dn12 * locals.var_t2__blk752) + (locals.var_t1__blk751 * locals.var_t2__blk752_dn12)), ((locals.var_t1__blk751_dn17 * locals.var_t2__blk752) + (locals.var_t1__blk751 * locals.var_t2__blk752_dn17)),)
    } else {
        (locals.var_e1, locals.var_e1_dn0, locals.var_e1_dn2, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn10, locals.var_e1_dn11, locals.var_e1_dn12, locals.var_e1_dn17,)
    }
};
        locals.var_e1 = assign24240_e33299;
        locals.var_e1_dn0 = assign24240_e33299_d_n0;
        locals.var_e1_dn2 = assign24240_e33299_d_n2;
        locals.var_e1_dn6 = assign24240_e33299_d_n6;
        locals.var_e1_dn7 = assign24240_e33299_d_n7;
        locals.var_e1_dn10 = assign24240_e33299_d_n10;
        locals.var_e1_dn11 = assign24240_e33299_d_n11;
        locals.var_e1_dn12 = assign24240_e33299_d_n12;
        locals.var_e1_dn17 = assign24240_e33299_d_n17;
        locals.var_e1_rv = 0.0;

        let (assign24250_e33313, assign24250_e33313_d_n0, assign24250_e33313_d_n2, assign24250_e33313_d_n6, assign24250_e33313_d_n7, assign24250_e33313_d_n10, assign24250_e33313_d_n11, assign24250_e33313_d_n12, assign24250_e33313_d_n17,) = {
    if (locals.var_guard758 == 0.0) {
        let assign24250_e33304: f64 = (locals.var_e1 * locals.var_e1);
        let assign24250_e33307: f64 = (4.0 * 0.01);
        let assign24250_e33309: f64 = (assign24250_e33307 * 0.01);
        let assign24250_e33310: f64 = (assign24250_e33304 + assign24250_e33309);
        let assign24250_e33311: f64 = (assign24250_e33310).sqrt();
        (assign24250_e33311, (((locals.var_e1_dn0 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn0)) / (2.0 * assign24250_e33311)), (((locals.var_e1_dn2 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn2)) / (2.0 * assign24250_e33311)), (((locals.var_e1_dn6 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn6)) / (2.0 * assign24250_e33311)), (((locals.var_e1_dn7 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn7)) / (2.0 * assign24250_e33311)), (((locals.var_e1_dn10 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn10)) / (2.0 * assign24250_e33311)), (((locals.var_e1_dn11 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn11)) / (2.0 * assign24250_e33311)), (((locals.var_e1_dn12 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn12)) / (2.0 * assign24250_e33311)), (((locals.var_e1_dn17 * locals.var_e1) + (locals.var_e1 * locals.var_e1_dn17)) / (2.0 * assign24250_e33311)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign24250_e33313;
        locals.var_tmf1_dn0 = assign24250_e33313_d_n0;
        locals.var_tmf1_dn2 = assign24250_e33313_d_n2;
        locals.var_tmf1_dn6 = assign24250_e33313_d_n6;
        locals.var_tmf1_dn7 = assign24250_e33313_d_n7;
        locals.var_tmf1_dn10 = assign24250_e33313_d_n10;
        locals.var_tmf1_dn11 = assign24250_e33313_d_n11;
        locals.var_tmf1_dn12 = assign24250_e33313_d_n12;
        locals.var_tmf1_dn17 = assign24250_e33313_d_n17;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_86(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24260_e33326, assign24260_e33326_d_n0, assign24260_e33326_d_n2, assign24260_e33326_d_n6, assign24260_e33326_d_n7, assign24260_e33326_d_n10, assign24260_e33326_d_n11, assign24260_e33326_d_n12, assign24260_e33326_d_n17,) = {
    if (locals.var_guard758 == 0.0) {
        let assign24260_e33319: f64 = (locals.var_e1 + locals.var_tmf1);
        let assign24260_e33320: f64 = (0.5 * assign24260_e33319);
        let assign24260_e33323: f64 = (1e-10 * 0.01);
        let assign24260_e33324: f64 = (assign24260_e33320 + assign24260_e33323);
        (assign24260_e33324, (0.5 * (locals.var_e1_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_e1_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_e1_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_e1_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_e1_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_e1_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_e1_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_e1_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_egidl, locals.var_egidl_dn0, locals.var_egidl_dn2, locals.var_egidl_dn6, locals.var_egidl_dn7, locals.var_egidl_dn10, locals.var_egidl_dn11, locals.var_egidl_dn12, locals.var_egidl_dn17,)
    }
};
        locals.var_egidl = assign24260_e33326;
        locals.var_egidl_dn0 = assign24260_e33326_d_n0;
        locals.var_egidl_dn2 = assign24260_e33326_d_n2;
        locals.var_egidl_dn6 = assign24260_e33326_d_n6;
        locals.var_egidl_dn7 = assign24260_e33326_d_n7;
        locals.var_egidl_dn10 = assign24260_e33326_d_n10;
        locals.var_egidl_dn11 = assign24260_e33326_d_n11;
        locals.var_egidl_dn12 = assign24260_e33326_d_n12;
        locals.var_egidl_dn17 = assign24260_e33326_d_n17;
        locals.var_egidl_rv = 0.0;

        let assign24270_e33329: f64 = if locals.var_egidl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard759 = assign24270_e33329;
        locals.var_guard759_rv = 0.0;

        let (assign24280_e33336, assign24280_e33336_d_n0, assign24280_e33336_d_n2, assign24280_e33336_d_n6, assign24280_e33336_d_n7, assign24280_e33336_d_n10, assign24280_e33336_d_n11, assign24280_e33336_d_n12, assign24280_e33336_d_n17,) = {
    if ((locals.var_guard758 == 0.0) && (locals.var_guard759 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_egidl, locals.var_egidl_dn0, locals.var_egidl_dn2, locals.var_egidl_dn6, locals.var_egidl_dn7, locals.var_egidl_dn10, locals.var_egidl_dn11, locals.var_egidl_dn12, locals.var_egidl_dn17,)
    }
};
        locals.var_egidl = assign24280_e33336;
        locals.var_egidl_dn0 = assign24280_e33336_d_n0;
        locals.var_egidl_dn2 = assign24280_e33336_d_n2;
        locals.var_egidl_dn6 = assign24280_e33336_d_n6;
        locals.var_egidl_dn7 = assign24280_e33336_d_n7;
        locals.var_egidl_dn10 = assign24280_e33336_d_n10;
        locals.var_egidl_dn11 = assign24280_e33336_d_n11;
        locals.var_egidl_dn12 = assign24280_e33336_d_n12;
        locals.var_egidl_dn17 = assign24280_e33336_d_n17;
        locals.var_egidl_rv = 0.0;

        let (assign24290_e33345, assign24290_e33345_d_n0, assign24290_e33345_d_n2, assign24290_e33345_d_n6, assign24290_e33345_d_n7, assign24290_e33345_d_n10, assign24290_e33345_d_n11, assign24290_e33345_d_n12, assign24290_e33345_d_n17,) = {
    if (locals.var_guard758 == 0.0) {
        let assign24290_e33342: f64 = (locals.var_egidl + 1e-50);
        let assign24290_e33343: f64 = (1.0 / assign24290_e33342);
        (assign24290_e33343, (-(locals.var_egidl_dn0 / (assign24290_e33342 * assign24290_e33342))), (-(locals.var_egidl_dn2 / (assign24290_e33342 * assign24290_e33342))), (-(locals.var_egidl_dn6 / (assign24290_e33342 * assign24290_e33342))), (-(locals.var_egidl_dn7 / (assign24290_e33342 * assign24290_e33342))), (-(locals.var_egidl_dn10 / (assign24290_e33342 * assign24290_e33342))), (-(locals.var_egidl_dn11 / (assign24290_e33342 * assign24290_e33342))), (-(locals.var_egidl_dn12 / (assign24290_e33342 * assign24290_e33342))), (-(locals.var_egidl_dn17 / (assign24290_e33342 * assign24290_e33342))),)
    } else {
        (locals.var_t3__blk754, locals.var_t3__blk754_dn0, locals.var_t3__blk754_dn2, locals.var_t3__blk754_dn6, locals.var_t3__blk754_dn7, locals.var_t3__blk754_dn10, locals.var_t3__blk754_dn11, locals.var_t3__blk754_dn12, locals.var_t3__blk754_dn17,)
    }
};
        locals.var_t3__blk754 = assign24290_e33345;
        locals.var_t3__blk754_dn0 = assign24290_e33345_d_n0;
        locals.var_t3__blk754_dn2 = assign24290_e33345_d_n2;
        locals.var_t3__blk754_dn6 = assign24290_e33345_d_n6;
        locals.var_t3__blk754_dn7 = assign24290_e33345_d_n7;
        locals.var_t3__blk754_dn10 = assign24290_e33345_d_n10;
        locals.var_t3__blk754_dn11 = assign24290_e33345_d_n11;
        locals.var_t3__blk754_dn12 = assign24290_e33345_d_n12;
        locals.var_t3__blk754_dn17 = assign24290_e33345_d_n17;
        locals.var_t3__blk754_rv = 0.0;

        let (assign24300_e33355, assign24300_e33355_d_n0, assign24300_e33355_d_n2, assign24300_e33355_d_n6, assign24300_e33355_d_n7, assign24300_e33355_d_n10, assign24300_e33355_d_n11, assign24300_e33355_d_n12, assign24300_e33355_d_n17,) = {
    if (locals.var_guard758 == 0.0) {
        let assign24300_e33349: f64 = (-p.p208);
        let assign24300_e33351: f64 = (assign24300_e33349 * locals.var_egp32);
        let assign24300_e33353: f64 = (assign24300_e33351 * locals.var_t3__blk754);
        (assign24300_e33353, (((assign24300_e33349 * locals.var_egp32_dn0) * locals.var_t3__blk754) + (assign24300_e33351 * locals.var_t3__blk754_dn0)), (((assign24300_e33349 * locals.var_egp32_dn2) * locals.var_t3__blk754) + (assign24300_e33351 * locals.var_t3__blk754_dn2)), (((assign24300_e33349 * locals.var_egp32_dn6) * locals.var_t3__blk754) + (assign24300_e33351 * locals.var_t3__blk754_dn6)), (((assign24300_e33349 * locals.var_egp32_dn7) * locals.var_t3__blk754) + (assign24300_e33351 * locals.var_t3__blk754_dn7)), (((assign24300_e33349 * locals.var_egp32_dn10) * locals.var_t3__blk754) + (assign24300_e33351 * locals.var_t3__blk754_dn10)), (((assign24300_e33349 * locals.var_egp32_dn11) * locals.var_t3__blk754) + (assign24300_e33351 * locals.var_t3__blk754_dn11)), (((assign24300_e33349 * locals.var_egp32_dn12) * locals.var_t3__blk754) + (assign24300_e33351 * locals.var_t3__blk754_dn12)), (((assign24300_e33349 * locals.var_egp32_dn17) * locals.var_t3__blk754) + (assign24300_e33351 * locals.var_t3__blk754_dn17)),)
    } else {
        (locals.var_t0__blk755, locals.var_t0__blk755_dn0, locals.var_t0__blk755_dn2, locals.var_t0__blk755_dn6, locals.var_t0__blk755_dn7, locals.var_t0__blk755_dn10, locals.var_t0__blk755_dn11, locals.var_t0__blk755_dn12, locals.var_t0__blk755_dn17,)
    }
};
        locals.var_t0__blk755 = assign24300_e33355;
        locals.var_t0__blk755_dn0 = assign24300_e33355_d_n0;
        locals.var_t0__blk755_dn2 = assign24300_e33355_d_n2;
        locals.var_t0__blk755_dn6 = assign24300_e33355_d_n6;
        locals.var_t0__blk755_dn7 = assign24300_e33355_d_n7;
        locals.var_t0__blk755_dn10 = assign24300_e33355_d_n10;
        locals.var_t0__blk755_dn11 = assign24300_e33355_d_n11;
        locals.var_t0__blk755_dn12 = assign24300_e33355_d_n12;
        locals.var_t0__blk755_dn17 = assign24300_e33355_d_n17;
        locals.var_t0__blk755_rv = 0.0;

        let assign24310_e33358: f64 = (-34.0);
        let assign24310_e33359: f64 = if locals.var_t0__blk755 < assign24310_e33358 { 1.0 } else { 0.0 };
        locals.var_guard760 = assign24310_e33359;
        locals.var_guard760_rv = 0.0;

        let (assign24330_e33375, assign24330_e33375_d_n0, assign24330_e33375_d_n2, assign24330_e33375_d_n6, assign24330_e33375_d_n7, assign24330_e33375_d_n10, assign24330_e33375_d_n11, assign24330_e33375_d_n12, assign24330_e33375_d_n17,) = {
    if ((locals.var_guard758 == 0.0) && (locals.var_guard760 == 0.0)) {
        let assign24330_e33373: f64 = (locals.var_t0__blk755).exp();
        (assign24330_e33373, (assign24330_e33373 * locals.var_t0__blk755_dn0), (assign24330_e33373 * locals.var_t0__blk755_dn2), (assign24330_e33373 * locals.var_t0__blk755_dn6), (assign24330_e33373 * locals.var_t0__blk755_dn7), (assign24330_e33373 * locals.var_t0__blk755_dn10), (assign24330_e33373 * locals.var_t0__blk755_dn11), (assign24330_e33373 * locals.var_t0__blk755_dn12), (assign24330_e33373 * locals.var_t0__blk755_dn17),)
    } else {
        (locals.var_t1__blk751, locals.var_t1__blk751_dn0, locals.var_t1__blk751_dn2, locals.var_t1__blk751_dn6, locals.var_t1__blk751_dn7, locals.var_t1__blk751_dn10, locals.var_t1__blk751_dn11, locals.var_t1__blk751_dn12, locals.var_t1__blk751_dn17,)
    }
};
        locals.var_t1__blk751 = assign24330_e33375;
        locals.var_t1__blk751_dn0 = assign24330_e33375_d_n0;
        locals.var_t1__blk751_dn2 = assign24330_e33375_d_n2;
        locals.var_t1__blk751_dn6 = assign24330_e33375_d_n6;
        locals.var_t1__blk751_dn7 = assign24330_e33375_d_n7;
        locals.var_t1__blk751_dn10 = assign24330_e33375_d_n10;
        locals.var_t1__blk751_dn11 = assign24330_e33375_d_n11;
        locals.var_t1__blk751_dn12 = assign24330_e33375_d_n12;
        locals.var_t1__blk751_dn17 = assign24330_e33375_d_n17;
        locals.var_t1__blk751_rv = 0.0;

        let (assign24340_e33389, assign24340_e33389_d_n0, assign24340_e33389_d_n2, assign24340_e33389_d_n6, assign24340_e33389_d_n7, assign24340_e33389_d_n10, assign24340_e33389_d_n11, assign24340_e33389_d_n12, assign24340_e33389_d_n17,) = {
    if ((locals.var_guard758 == 0.0) && (locals.var_guard760 == 0.0)) {
        let assign24340_e33383: f64 = (p.p207 / locals.var_egp12);
        let assign24340_e33385: f64 = (assign24340_e33383 * 1.6021918e-19);
        let assign24340_e33387: f64 = (assign24340_e33385 * locals.var_weff_nf);
        (assign24340_e33387, (((-((p.p207 * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn7) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn11) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn12) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf), (((-((p.p207 * locals.var_egp12_dn17) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_weff_nf),)
    } else {
        (locals.var_t2__blk752, locals.var_t2__blk752_dn0, locals.var_t2__blk752_dn2, locals.var_t2__blk752_dn6, locals.var_t2__blk752_dn7, locals.var_t2__blk752_dn10, locals.var_t2__blk752_dn11, locals.var_t2__blk752_dn12, locals.var_t2__blk752_dn17,)
    }
};
        locals.var_t2__blk752 = assign24340_e33389;
        locals.var_t2__blk752_dn0 = assign24340_e33389_d_n0;
        locals.var_t2__blk752_dn2 = assign24340_e33389_d_n2;
        locals.var_t2__blk752_dn6 = assign24340_e33389_d_n6;
        locals.var_t2__blk752_dn7 = assign24340_e33389_d_n7;
        locals.var_t2__blk752_dn10 = assign24340_e33389_d_n10;
        locals.var_t2__blk752_dn11 = assign24340_e33389_d_n11;
        locals.var_t2__blk752_dn12 = assign24340_e33389_d_n12;
        locals.var_t2__blk752_dn17 = assign24340_e33389_d_n17;
        locals.var_t2__blk752_rv = 0.0;

        let (assign24360_e33410, assign24360_e33410_d_n0, assign24360_e33410_d_n2, assign24360_e33410_d_n6, assign24360_e33410_d_n7, assign24360_e33410_d_n10, assign24360_e33410_d_n11, assign24360_e33410_d_n12, assign24360_e33410_d_n17,) = {
    if (locals.var_guard758 == 0.0) {
        let assign24360_e33408: f64 = (locals.var_vds - locals.var_vbsp);
        (assign24360_e33408, (locals.var_vds_dn0 - locals.var_vbsp_dn0), (locals.var_vds_dn2 - locals.var_vbsp_dn2), (locals.var_vds_dn6 - locals.var_vbsp_dn6), (locals.var_vds_dn7 - locals.var_vbsp_dn7), (locals.var_vds_dn10 - locals.var_vbsp_dn10), (locals.var_vds_dn11 - locals.var_vbsp_dn11), (locals.var_vds_dn12 - locals.var_vbsp_dn12), (locals.var_vds_dn17 - locals.var_vbsp_dn17),)
    } else {
        (locals.var_vdb, locals.var_vdb_dn0, locals.var_vdb_dn2, locals.var_vdb_dn6, locals.var_vdb_dn7, locals.var_vdb_dn10, locals.var_vdb_dn11, locals.var_vdb_dn12, locals.var_vdb_dn17,)
    }
};
        locals.var_vdb = assign24360_e33410;
        locals.var_vdb_dn0 = assign24360_e33410_d_n0;
        locals.var_vdb_dn2 = assign24360_e33410_d_n2;
        locals.var_vdb_dn6 = assign24360_e33410_d_n6;
        locals.var_vdb_dn7 = assign24360_e33410_d_n7;
        locals.var_vdb_dn10 = assign24360_e33410_d_n10;
        locals.var_vdb_dn11 = assign24360_e33410_d_n11;
        locals.var_vdb_dn12 = assign24360_e33410_d_n12;
        locals.var_vdb_dn17 = assign24360_e33410_d_n17;
        locals.var_vdb_rv = 0.0;

        let assign24370_e33413: f64 = if locals.var_vdb > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard761 = assign24370_e33413;
        locals.var_guard761_rv = 0.0;

        let (assign24380_e33422, assign24380_e33422_d_n0, assign24380_e33422_d_n2, assign24380_e33422_d_n6, assign24380_e33422_d_n7, assign24380_e33422_d_n10, assign24380_e33422_d_n11, assign24380_e33422_d_n12, assign24380_e33422_d_n17,) = {
    if ((locals.var_guard758 == 0.0) && (locals.var_guard761 != 0.0)) {
        let assign24380_e33420: f64 = (locals.var_vdb * locals.var_vdb);
        (assign24380_e33420, ((locals.var_vdb_dn0 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn0)), ((locals.var_vdb_dn2 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn2)), ((locals.var_vdb_dn6 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn6)), ((locals.var_vdb_dn7 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn7)), ((locals.var_vdb_dn10 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn10)), ((locals.var_vdb_dn11 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn11)), ((locals.var_vdb_dn12 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn12)), ((locals.var_vdb_dn17 * locals.var_vdb) + (locals.var_vdb * locals.var_vdb_dn17)),)
    } else {
        (locals.var_t2__blk752, locals.var_t2__blk752_dn0, locals.var_t2__blk752_dn2, locals.var_t2__blk752_dn6, locals.var_t2__blk752_dn7, locals.var_t2__blk752_dn10, locals.var_t2__blk752_dn11, locals.var_t2__blk752_dn12, locals.var_t2__blk752_dn17,)
    }
};
        locals.var_t2__blk752 = assign24380_e33422;
        locals.var_t2__blk752_dn0 = assign24380_e33422_d_n0;
        locals.var_t2__blk752_dn2 = assign24380_e33422_d_n2;
        locals.var_t2__blk752_dn6 = assign24380_e33422_d_n6;
        locals.var_t2__blk752_dn7 = assign24380_e33422_d_n7;
        locals.var_t2__blk752_dn10 = assign24380_e33422_d_n10;
        locals.var_t2__blk752_dn11 = assign24380_e33422_d_n11;
        locals.var_t2__blk752_dn12 = assign24380_e33422_d_n12;
        locals.var_t2__blk752_dn17 = assign24380_e33422_d_n17;
        locals.var_t2__blk752_rv = 0.0;

        let (assign24390_e33431, assign24390_e33431_d_n0, assign24390_e33431_d_n2, assign24390_e33431_d_n6, assign24390_e33431_d_n7, assign24390_e33431_d_n10, assign24390_e33431_d_n11, assign24390_e33431_d_n12, assign24390_e33431_d_n17,) = {
    if ((locals.var_guard758 == 0.0) && (locals.var_guard761 != 0.0)) {
        let assign24390_e33429: f64 = (locals.var_t2__blk752 * locals.var_vdb);
        (assign24390_e33429, ((locals.var_t2__blk752_dn0 * locals.var_vdb) + (locals.var_t2__blk752 * locals.var_vdb_dn0)), ((locals.var_t2__blk752_dn2 * locals.var_vdb) + (locals.var_t2__blk752 * locals.var_vdb_dn2)), ((locals.var_t2__blk752_dn6 * locals.var_vdb) + (locals.var_t2__blk752 * locals.var_vdb_dn6)), ((locals.var_t2__blk752_dn7 * locals.var_vdb) + (locals.var_t2__blk752 * locals.var_vdb_dn7)), ((locals.var_t2__blk752_dn10 * locals.var_vdb) + (locals.var_t2__blk752 * locals.var_vdb_dn10)), ((locals.var_t2__blk752_dn11 * locals.var_vdb) + (locals.var_t2__blk752 * locals.var_vdb_dn11)), ((locals.var_t2__blk752_dn12 * locals.var_vdb) + (locals.var_t2__blk752 * locals.var_vdb_dn12)), ((locals.var_t2__blk752_dn17 * locals.var_vdb) + (locals.var_t2__blk752 * locals.var_vdb_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign24390_e33431;
        locals.var_t4_dn0 = assign24390_e33431_d_n0;
        locals.var_t4_dn2 = assign24390_e33431_d_n2;
        locals.var_t4_dn6 = assign24390_e33431_d_n6;
        locals.var_t4_dn7 = assign24390_e33431_d_n7;
        locals.var_t4_dn10 = assign24390_e33431_d_n10;
        locals.var_t4_dn11 = assign24390_e33431_d_n11;
        locals.var_t4_dn12 = assign24390_e33431_d_n12;
        locals.var_t4_dn17 = assign24390_e33431_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign24400_e33440, assign24400_e33440_d_n0, assign24400_e33440_d_n2, assign24400_e33440_d_n6, assign24400_e33440_d_n7, assign24400_e33440_d_n10, assign24400_e33440_d_n11, assign24400_e33440_d_n12, assign24400_e33440_d_n17,) = {
    if ((locals.var_guard758 == 0.0) && (locals.var_guard761 != 0.0)) {
        let assign24400_e33438: f64 = (locals.var_t4 + p.p212);
        (assign24400_e33438, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    } else {
        (locals.var_t0__blk755, locals.var_t0__blk755_dn0, locals.var_t0__blk755_dn2, locals.var_t0__blk755_dn6, locals.var_t0__blk755_dn7, locals.var_t0__blk755_dn10, locals.var_t0__blk755_dn11, locals.var_t0__blk755_dn12, locals.var_t0__blk755_dn17,)
    }
};
        locals.var_t0__blk755 = assign24400_e33440;
        locals.var_t0__blk755_dn0 = assign24400_e33440_d_n0;
        locals.var_t0__blk755_dn2 = assign24400_e33440_d_n2;
        locals.var_t0__blk755_dn6 = assign24400_e33440_d_n6;
        locals.var_t0__blk755_dn7 = assign24400_e33440_d_n7;
        locals.var_t0__blk755_dn10 = assign24400_e33440_d_n10;
        locals.var_t0__blk755_dn11 = assign24400_e33440_d_n11;
        locals.var_t0__blk755_dn12 = assign24400_e33440_d_n12;
        locals.var_t0__blk755_dn17 = assign24400_e33440_d_n17;
        locals.var_t0__blk755_rv = 0.0;

        let assign24440_e33469: f64 = if p.p28 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard769 = assign24440_e33469;
        locals.var_guard769_rv = 0.0;

        let (assign24460_e33493, assign24460_e33493_d_n0, assign24460_e33493_d_n2, assign24460_e33493_d_n6, assign24460_e33493_d_n7, assign24460_e33493_d_n10, assign24460_e33493_d_n11, assign24460_e33493_d_n12, assign24460_e33493_d_n17,) = {
    if (locals.var_guard769 == 0.0) {
        let assign24460_e33478: f64 = (-locals.var_vds);
        let assign24460_e33480: f64 = (assign24460_e33478 + p.p210);
        let assign24460_e33481: f64 = (p.p209 * assign24460_e33480);
        let assign24460_e33484: f64 = (locals.var_vgs - locals.var_vds);
        let assign24460_e33485: f64 = (assign24460_e33481 - assign24460_e33484);
        let assign24460_e33488: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign24460_e33490: f64 = (assign24460_e33488 * p.p211);
        let assign24460_e33491: f64 = (assign24460_e33485 + assign24460_e33490);
        (assign24460_e33491, (((p.p209 * (-locals.var_vds_dn0)) - (-locals.var_vds_dn0)) + ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) * p.p211)), (((p.p209 * (-locals.var_vds_dn2)) - (-locals.var_vds_dn2)) + ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) * p.p211)), (((p.p209 * (-locals.var_vds_dn6)) - (locals.var_vgs_dn6 - locals.var_vds_dn6)) + ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) * p.p211)), (((p.p209 * (-locals.var_vds_dn7)) - (locals.var_vgs_dn7 - locals.var_vds_dn7)) + ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) * p.p211)), (((p.p209 * (-locals.var_vds_dn10)) - (-locals.var_vds_dn10)) + ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) * p.p211)), (((p.p209 * (-locals.var_vds_dn11)) - (locals.var_vgs_dn11 - locals.var_vds_dn11)) + ((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) * p.p211)), (((p.p209 * (-locals.var_vds_dn12)) - (-locals.var_vds_dn12)) + ((locals.var_dvthsc_dn12 + locals.var_dvthlp_dn12) * p.p211)), (((p.p209 * (-locals.var_vds_dn17)) - (-locals.var_vds_dn17)) + ((locals.var_dvthsc_dn17 + locals.var_dvthlp_dn17) * p.p211)),)
    } else {
        (locals.var_t1__blk762, locals.var_t1__blk762_dn0, locals.var_t1__blk762_dn2, locals.var_t1__blk762_dn6, locals.var_t1__blk762_dn7, locals.var_t1__blk762_dn10, locals.var_t1__blk762_dn11, locals.var_t1__blk762_dn12, locals.var_t1__blk762_dn17,)
    }
};
        locals.var_t1__blk762 = assign24460_e33493;
        locals.var_t1__blk762_dn0 = assign24460_e33493_d_n0;
        locals.var_t1__blk762_dn2 = assign24460_e33493_d_n2;
        locals.var_t1__blk762_dn6 = assign24460_e33493_d_n6;
        locals.var_t1__blk762_dn7 = assign24460_e33493_d_n7;
        locals.var_t1__blk762_dn10 = assign24460_e33493_d_n10;
        locals.var_t1__blk762_dn11 = assign24460_e33493_d_n11;
        locals.var_t1__blk762_dn12 = assign24460_e33493_d_n12;
        locals.var_t1__blk762_dn17 = assign24460_e33493_d_n17;
        locals.var_t1__blk762_rv = 0.0;

        let (assign24470_e33500, assign24470_e33500_d_n0, assign24470_e33500_d_n2, assign24470_e33500_d_n6, assign24470_e33500_d_n7, assign24470_e33500_d_n10, assign24470_e33500_d_n11, assign24470_e33500_d_n12, assign24470_e33500_d_n17,) = {
    if (locals.var_guard769 == 0.0) {
        let assign24470_e33498: f64 = (1.0 / locals.var_tfox0);
        (assign24470_e33498, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk763, locals.var_t2__blk763_dn0, locals.var_t2__blk763_dn2, locals.var_t2__blk763_dn6, locals.var_t2__blk763_dn7, locals.var_t2__blk763_dn10, locals.var_t2__blk763_dn11, locals.var_t2__blk763_dn12, locals.var_t2__blk763_dn17,)
    }
};
        locals.var_t2__blk763 = assign24470_e33500;
        locals.var_t2__blk763_dn0 = assign24470_e33500_d_n0;
        locals.var_t2__blk763_dn2 = assign24470_e33500_d_n2;
        locals.var_t2__blk763_dn6 = assign24470_e33500_d_n6;
        locals.var_t2__blk763_dn7 = assign24470_e33500_d_n7;
        locals.var_t2__blk763_dn10 = assign24470_e33500_d_n10;
        locals.var_t2__blk763_dn11 = assign24470_e33500_d_n11;
        locals.var_t2__blk763_dn12 = assign24470_e33500_d_n12;
        locals.var_t2__blk763_dn17 = assign24470_e33500_d_n17;
        locals.var_t2__blk763_rv = 0.0;

        let (assign24480_e33507, assign24480_e33507_d_n0, assign24480_e33507_d_n2, assign24480_e33507_d_n6, assign24480_e33507_d_n7, assign24480_e33507_d_n10, assign24480_e33507_d_n11, assign24480_e33507_d_n12, assign24480_e33507_d_n17,) = {
    if (locals.var_guard769 == 0.0) {
        let assign24480_e33505: f64 = (locals.var_t1__blk762 * locals.var_t2__blk763);
        (assign24480_e33505, ((locals.var_t1__blk762_dn0 * locals.var_t2__blk763) + (locals.var_t1__blk762 * locals.var_t2__blk763_dn0)), ((locals.var_t1__blk762_dn2 * locals.var_t2__blk763) + (locals.var_t1__blk762 * locals.var_t2__blk763_dn2)), ((locals.var_t1__blk762_dn6 * locals.var_t2__blk763) + (locals.var_t1__blk762 * locals.var_t2__blk763_dn6)), ((locals.var_t1__blk762_dn7 * locals.var_t2__blk763) + (locals.var_t1__blk762 * locals.var_t2__blk763_dn7)), ((locals.var_t1__blk762_dn10 * locals.var_t2__blk763) + (locals.var_t1__blk762 * locals.var_t2__blk763_dn10)), ((locals.var_t1__blk762_dn11 * locals.var_t2__blk763) + (locals.var_t1__blk762 * locals.var_t2__blk763_dn11)), ((locals.var_t1__blk762_dn12 * locals.var_t2__blk763) + (locals.var_t1__blk762 * locals.var_t2__blk763_dn12)), ((locals.var_t1__blk762_dn17 * locals.var_t2__blk763) + (locals.var_t1__blk762 * locals.var_t2__blk763_dn17)),)
    } else {
        (locals.var_e1__blk764, locals.var_e1__blk764_dn0, locals.var_e1__blk764_dn2, locals.var_e1__blk764_dn6, locals.var_e1__blk764_dn7, locals.var_e1__blk764_dn10, locals.var_e1__blk764_dn11, locals.var_e1__blk764_dn12, locals.var_e1__blk764_dn17,)
    }
};
        locals.var_e1__blk764 = assign24480_e33507;
        locals.var_e1__blk764_dn0 = assign24480_e33507_d_n0;
        locals.var_e1__blk764_dn2 = assign24480_e33507_d_n2;
        locals.var_e1__blk764_dn6 = assign24480_e33507_d_n6;
        locals.var_e1__blk764_dn7 = assign24480_e33507_d_n7;
        locals.var_e1__blk764_dn10 = assign24480_e33507_d_n10;
        locals.var_e1__blk764_dn11 = assign24480_e33507_d_n11;
        locals.var_e1__blk764_dn12 = assign24480_e33507_d_n12;
        locals.var_e1__blk764_dn17 = assign24480_e33507_d_n17;
        locals.var_e1__blk764_rv = 0.0;

        let (assign24490_e33521, assign24490_e33521_d_n0, assign24490_e33521_d_n2, assign24490_e33521_d_n6, assign24490_e33521_d_n7, assign24490_e33521_d_n10, assign24490_e33521_d_n11, assign24490_e33521_d_n12, assign24490_e33521_d_n17,) = {
    if (locals.var_guard769 == 0.0) {
        let assign24490_e33512: f64 = (locals.var_e1__blk764 * locals.var_e1__blk764);
        let assign24490_e33515: f64 = (4.0 * 0.01);
        let assign24490_e33517: f64 = (assign24490_e33515 * 0.01);
        let assign24490_e33518: f64 = (assign24490_e33512 + assign24490_e33517);
        let assign24490_e33519: f64 = (assign24490_e33518).sqrt();
        (assign24490_e33519, (((locals.var_e1__blk764_dn0 * locals.var_e1__blk764) + (locals.var_e1__blk764 * locals.var_e1__blk764_dn0)) / (2.0 * assign24490_e33519)), (((locals.var_e1__blk764_dn2 * locals.var_e1__blk764) + (locals.var_e1__blk764 * locals.var_e1__blk764_dn2)) / (2.0 * assign24490_e33519)), (((locals.var_e1__blk764_dn6 * locals.var_e1__blk764) + (locals.var_e1__blk764 * locals.var_e1__blk764_dn6)) / (2.0 * assign24490_e33519)), (((locals.var_e1__blk764_dn7 * locals.var_e1__blk764) + (locals.var_e1__blk764 * locals.var_e1__blk764_dn7)) / (2.0 * assign24490_e33519)), (((locals.var_e1__blk764_dn10 * locals.var_e1__blk764) + (locals.var_e1__blk764 * locals.var_e1__blk764_dn10)) / (2.0 * assign24490_e33519)), (((locals.var_e1__blk764_dn11 * locals.var_e1__blk764) + (locals.var_e1__blk764 * locals.var_e1__blk764_dn11)) / (2.0 * assign24490_e33519)), (((locals.var_e1__blk764_dn12 * locals.var_e1__blk764) + (locals.var_e1__blk764 * locals.var_e1__blk764_dn12)) / (2.0 * assign24490_e33519)), (((locals.var_e1__blk764_dn17 * locals.var_e1__blk764) + (locals.var_e1__blk764 * locals.var_e1__blk764_dn17)) / (2.0 * assign24490_e33519)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign24490_e33521;
        locals.var_tmf1_dn0 = assign24490_e33521_d_n0;
        locals.var_tmf1_dn2 = assign24490_e33521_d_n2;
        locals.var_tmf1_dn6 = assign24490_e33521_d_n6;
        locals.var_tmf1_dn7 = assign24490_e33521_d_n7;
        locals.var_tmf1_dn10 = assign24490_e33521_d_n10;
        locals.var_tmf1_dn11 = assign24490_e33521_d_n11;
        locals.var_tmf1_dn12 = assign24490_e33521_d_n12;
        locals.var_tmf1_dn17 = assign24490_e33521_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign24500_e33534, assign24500_e33534_d_n0, assign24500_e33534_d_n2, assign24500_e33534_d_n6, assign24500_e33534_d_n7, assign24500_e33534_d_n10, assign24500_e33534_d_n11, assign24500_e33534_d_n12, assign24500_e33534_d_n17,) = {
    if (locals.var_guard769 == 0.0) {
        let assign24500_e33527: f64 = (locals.var_e1__blk764 + locals.var_tmf1);
        let assign24500_e33528: f64 = (0.5 * assign24500_e33527);
        let assign24500_e33531: f64 = (1e-10 * 0.01);
        let assign24500_e33532: f64 = (assign24500_e33528 + assign24500_e33531);
        (assign24500_e33532, (0.5 * (locals.var_e1__blk764_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_e1__blk764_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_e1__blk764_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_e1__blk764_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_e1__blk764_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_e1__blk764_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_e1__blk764_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_e1__blk764_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_egisl, locals.var_egisl_dn0, locals.var_egisl_dn2, locals.var_egisl_dn6, locals.var_egisl_dn7, locals.var_egisl_dn10, locals.var_egisl_dn11, locals.var_egisl_dn12, locals.var_egisl_dn17,)
    }
};
        locals.var_egisl = assign24500_e33534;
        locals.var_egisl_dn0 = assign24500_e33534_d_n0;
        locals.var_egisl_dn2 = assign24500_e33534_d_n2;
        locals.var_egisl_dn6 = assign24500_e33534_d_n6;
        locals.var_egisl_dn7 = assign24500_e33534_d_n7;
        locals.var_egisl_dn10 = assign24500_e33534_d_n10;
        locals.var_egisl_dn11 = assign24500_e33534_d_n11;
        locals.var_egisl_dn12 = assign24500_e33534_d_n12;
        locals.var_egisl_dn17 = assign24500_e33534_d_n17;
        locals.var_egisl_rv = 0.0;

        let assign24510_e33537: f64 = if locals.var_egisl < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard770 = assign24510_e33537;
        locals.var_guard770_rv = 0.0;

        let (assign24520_e33544, assign24520_e33544_d_n0, assign24520_e33544_d_n2, assign24520_e33544_d_n6, assign24520_e33544_d_n7, assign24520_e33544_d_n10, assign24520_e33544_d_n11, assign24520_e33544_d_n12, assign24520_e33544_d_n17,) = {
    if ((locals.var_guard769 == 0.0) && (locals.var_guard770 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_egisl, locals.var_egisl_dn0, locals.var_egisl_dn2, locals.var_egisl_dn6, locals.var_egisl_dn7, locals.var_egisl_dn10, locals.var_egisl_dn11, locals.var_egisl_dn12, locals.var_egisl_dn17,)
    }
};
        locals.var_egisl = assign24520_e33544;
        locals.var_egisl_dn0 = assign24520_e33544_d_n0;
        locals.var_egisl_dn2 = assign24520_e33544_d_n2;
        locals.var_egisl_dn6 = assign24520_e33544_d_n6;
        locals.var_egisl_dn7 = assign24520_e33544_d_n7;
        locals.var_egisl_dn10 = assign24520_e33544_d_n10;
        locals.var_egisl_dn11 = assign24520_e33544_d_n11;
        locals.var_egisl_dn12 = assign24520_e33544_d_n12;
        locals.var_egisl_dn17 = assign24520_e33544_d_n17;
        locals.var_egisl_rv = 0.0;

        let (assign24530_e33553, assign24530_e33553_d_n0, assign24530_e33553_d_n2, assign24530_e33553_d_n6, assign24530_e33553_d_n7, assign24530_e33553_d_n10, assign24530_e33553_d_n11, assign24530_e33553_d_n12, assign24530_e33553_d_n17,) = {
    if (locals.var_guard769 == 0.0) {
        let assign24530_e33550: f64 = (locals.var_egisl + 1e-50);
        let assign24530_e33551: f64 = (1.0 / assign24530_e33550);
        (assign24530_e33551, (-(locals.var_egisl_dn0 / (assign24530_e33550 * assign24530_e33550))), (-(locals.var_egisl_dn2 / (assign24530_e33550 * assign24530_e33550))), (-(locals.var_egisl_dn6 / (assign24530_e33550 * assign24530_e33550))), (-(locals.var_egisl_dn7 / (assign24530_e33550 * assign24530_e33550))), (-(locals.var_egisl_dn10 / (assign24530_e33550 * assign24530_e33550))), (-(locals.var_egisl_dn11 / (assign24530_e33550 * assign24530_e33550))), (-(locals.var_egisl_dn12 / (assign24530_e33550 * assign24530_e33550))), (-(locals.var_egisl_dn17 / (assign24530_e33550 * assign24530_e33550))),)
    } else {
        (locals.var_t3__blk765, locals.var_t3__blk765_dn0, locals.var_t3__blk765_dn2, locals.var_t3__blk765_dn6, locals.var_t3__blk765_dn7, locals.var_t3__blk765_dn10, locals.var_t3__blk765_dn11, locals.var_t3__blk765_dn12, locals.var_t3__blk765_dn17,)
    }
};
        locals.var_t3__blk765 = assign24530_e33553;
        locals.var_t3__blk765_dn0 = assign24530_e33553_d_n0;
        locals.var_t3__blk765_dn2 = assign24530_e33553_d_n2;
        locals.var_t3__blk765_dn6 = assign24530_e33553_d_n6;
        locals.var_t3__blk765_dn7 = assign24530_e33553_d_n7;
        locals.var_t3__blk765_dn10 = assign24530_e33553_d_n10;
        locals.var_t3__blk765_dn11 = assign24530_e33553_d_n11;
        locals.var_t3__blk765_dn12 = assign24530_e33553_d_n12;
        locals.var_t3__blk765_dn17 = assign24530_e33553_d_n17;
        locals.var_t3__blk765_rv = 0.0;

        let (assign24540_e33563, assign24540_e33563_d_n0, assign24540_e33563_d_n2, assign24540_e33563_d_n6, assign24540_e33563_d_n7, assign24540_e33563_d_n10, assign24540_e33563_d_n11, assign24540_e33563_d_n12, assign24540_e33563_d_n17,) = {
    if (locals.var_guard769 == 0.0) {
        let assign24540_e33557: f64 = (-p.p208);
        let assign24540_e33559: f64 = (assign24540_e33557 * locals.var_egp32);
        let assign24540_e33561: f64 = (assign24540_e33559 * locals.var_t3__blk765);
        (assign24540_e33561, (((assign24540_e33557 * locals.var_egp32_dn0) * locals.var_t3__blk765) + (assign24540_e33559 * locals.var_t3__blk765_dn0)), (((assign24540_e33557 * locals.var_egp32_dn2) * locals.var_t3__blk765) + (assign24540_e33559 * locals.var_t3__blk765_dn2)), (((assign24540_e33557 * locals.var_egp32_dn6) * locals.var_t3__blk765) + (assign24540_e33559 * locals.var_t3__blk765_dn6)), (((assign24540_e33557 * locals.var_egp32_dn7) * locals.var_t3__blk765) + (assign24540_e33559 * locals.var_t3__blk765_dn7)), (((assign24540_e33557 * locals.var_egp32_dn10) * locals.var_t3__blk765) + (assign24540_e33559 * locals.var_t3__blk765_dn10)), (((assign24540_e33557 * locals.var_egp32_dn11) * locals.var_t3__blk765) + (assign24540_e33559 * locals.var_t3__blk765_dn11)), (((assign24540_e33557 * locals.var_egp32_dn12) * locals.var_t3__blk765) + (assign24540_e33559 * locals.var_t3__blk765_dn12)), (((assign24540_e33557 * locals.var_egp32_dn17) * locals.var_t3__blk765) + (assign24540_e33559 * locals.var_t3__blk765_dn17)),)
    } else {
        (locals.var_t0__blk766, locals.var_t0__blk766_dn0, locals.var_t0__blk766_dn2, locals.var_t0__blk766_dn6, locals.var_t0__blk766_dn7, locals.var_t0__blk766_dn10, locals.var_t0__blk766_dn11, locals.var_t0__blk766_dn12, locals.var_t0__blk766_dn17,)
    }
};
        locals.var_t0__blk766 = assign24540_e33563;
        locals.var_t0__blk766_dn0 = assign24540_e33563_d_n0;
        locals.var_t0__blk766_dn2 = assign24540_e33563_d_n2;
        locals.var_t0__blk766_dn6 = assign24540_e33563_d_n6;
        locals.var_t0__blk766_dn7 = assign24540_e33563_d_n7;
        locals.var_t0__blk766_dn10 = assign24540_e33563_d_n10;
        locals.var_t0__blk766_dn11 = assign24540_e33563_d_n11;
        locals.var_t0__blk766_dn12 = assign24540_e33563_d_n12;
        locals.var_t0__blk766_dn17 = assign24540_e33563_d_n17;
        locals.var_t0__blk766_rv = 0.0;

        let assign24550_e33566: f64 = (-34.0);
        let assign24550_e33567: f64 = if locals.var_t0__blk766 < assign24550_e33566 { 1.0 } else { 0.0 };
        locals.var_guard771 = assign24550_e33567;
        locals.var_guard771_rv = 0.0;

        let (assign24570_e33583, assign24570_e33583_d_n0, assign24570_e33583_d_n2, assign24570_e33583_d_n6, assign24570_e33583_d_n7, assign24570_e33583_d_n10, assign24570_e33583_d_n11, assign24570_e33583_d_n12, assign24570_e33583_d_n17,) = {
    if ((locals.var_guard769 == 0.0) && (locals.var_guard771 == 0.0)) {
        let assign24570_e33581: f64 = (locals.var_t0__blk766).exp();
        (assign24570_e33581, (assign24570_e33581 * locals.var_t0__blk766_dn0), (assign24570_e33581 * locals.var_t0__blk766_dn2), (assign24570_e33581 * locals.var_t0__blk766_dn6), (assign24570_e33581 * locals.var_t0__blk766_dn7), (assign24570_e33581 * locals.var_t0__blk766_dn10), (assign24570_e33581 * locals.var_t0__blk766_dn11), (assign24570_e33581 * locals.var_t0__blk766_dn12), (assign24570_e33581 * locals.var_t0__blk766_dn17),)
    } else {
        (locals.var_t1__blk762, locals.var_t1__blk762_dn0, locals.var_t1__blk762_dn2, locals.var_t1__blk762_dn6, locals.var_t1__blk762_dn7, locals.var_t1__blk762_dn10, locals.var_t1__blk762_dn11, locals.var_t1__blk762_dn12, locals.var_t1__blk762_dn17,)
    }
};
        locals.var_t1__blk762 = assign24570_e33583;
        locals.var_t1__blk762_dn0 = assign24570_e33583_d_n0;
        locals.var_t1__blk762_dn2 = assign24570_e33583_d_n2;
        locals.var_t1__blk762_dn6 = assign24570_e33583_d_n6;
        locals.var_t1__blk762_dn7 = assign24570_e33583_d_n7;
        locals.var_t1__blk762_dn10 = assign24570_e33583_d_n10;
        locals.var_t1__blk762_dn11 = assign24570_e33583_d_n11;
        locals.var_t1__blk762_dn12 = assign24570_e33583_d_n12;
        locals.var_t1__blk762_dn17 = assign24570_e33583_d_n17;
        locals.var_t1__blk762_rv = 0.0;

        let (assign24580_e33593, assign24580_e33593_d_n0, assign24580_e33593_d_n2, assign24580_e33593_d_n6, assign24580_e33593_d_n7, assign24580_e33593_d_n10, assign24580_e33593_d_n11, assign24580_e33593_d_n12, assign24580_e33593_d_n17,) = {
    if ((locals.var_guard769 == 0.0) && (locals.var_guard771 == 0.0)) {
        let assign24580_e33591: f64 = (1.0 / locals.var_egp12);
        (assign24580_e33591, (-(locals.var_egp12_dn0 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn2 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn6 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn7 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn10 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn11 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn12 / (locals.var_egp12 * locals.var_egp12))), (-(locals.var_egp12_dn17 / (locals.var_egp12 * locals.var_egp12))),)
    } else {
        (locals.var_t3__blk765, locals.var_t3__blk765_dn0, locals.var_t3__blk765_dn2, locals.var_t3__blk765_dn6, locals.var_t3__blk765_dn7, locals.var_t3__blk765_dn10, locals.var_t3__blk765_dn11, locals.var_t3__blk765_dn12, locals.var_t3__blk765_dn17,)
    }
};
        locals.var_t3__blk765 = assign24580_e33593;
        locals.var_t3__blk765_dn0 = assign24580_e33593_d_n0;
        locals.var_t3__blk765_dn2 = assign24580_e33593_d_n2;
        locals.var_t3__blk765_dn6 = assign24580_e33593_d_n6;
        locals.var_t3__blk765_dn7 = assign24580_e33593_d_n7;
        locals.var_t3__blk765_dn10 = assign24580_e33593_d_n10;
        locals.var_t3__blk765_dn11 = assign24580_e33593_d_n11;
        locals.var_t3__blk765_dn12 = assign24580_e33593_d_n12;
        locals.var_t3__blk765_dn17 = assign24580_e33593_d_n17;
        locals.var_t3__blk765_rv = 0.0;

        let (assign24590_e33607, assign24590_e33607_d_n0, assign24590_e33607_d_n2, assign24590_e33607_d_n6, assign24590_e33607_d_n7, assign24590_e33607_d_n10, assign24590_e33607_d_n11, assign24590_e33607_d_n12, assign24590_e33607_d_n17,) = {
    if ((locals.var_guard769 == 0.0) && (locals.var_guard771 == 0.0)) {
        let assign24590_e33601: f64 = (p.p207 * locals.var_t3__blk765);
        let assign24590_e33603: f64 = (assign24590_e33601 * 1.6021918e-19);
        let assign24590_e33605: f64 = (assign24590_e33603 * locals.var_weff_nf);
        (assign24590_e33605, (((p.p207 * locals.var_t3__blk765_dn0) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk765_dn2) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk765_dn6) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk765_dn7) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk765_dn10) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk765_dn11) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk765_dn12) * 1.6021918e-19) * locals.var_weff_nf), (((p.p207 * locals.var_t3__blk765_dn17) * 1.6021918e-19) * locals.var_weff_nf),)
    } else {
        (locals.var_t2__blk763, locals.var_t2__blk763_dn0, locals.var_t2__blk763_dn2, locals.var_t2__blk763_dn6, locals.var_t2__blk763_dn7, locals.var_t2__blk763_dn10, locals.var_t2__blk763_dn11, locals.var_t2__blk763_dn12, locals.var_t2__blk763_dn17,)
    }
};
        locals.var_t2__blk763 = assign24590_e33607;
        locals.var_t2__blk763_dn0 = assign24590_e33607_d_n0;
        locals.var_t2__blk763_dn2 = assign24590_e33607_d_n2;
        locals.var_t2__blk763_dn6 = assign24590_e33607_d_n6;
        locals.var_t2__blk763_dn7 = assign24590_e33607_d_n7;
        locals.var_t2__blk763_dn10 = assign24590_e33607_d_n10;
        locals.var_t2__blk763_dn11 = assign24590_e33607_d_n11;
        locals.var_t2__blk763_dn12 = assign24590_e33607_d_n12;
        locals.var_t2__blk763_dn17 = assign24590_e33607_d_n17;
        locals.var_t2__blk763_rv = 0.0;

        let (assign24610_e33627, assign24610_e33627_d_n0, assign24610_e33627_d_n2, assign24610_e33627_d_n6, assign24610_e33627_d_n7, assign24610_e33627_d_n10, assign24610_e33627_d_n11, assign24610_e33627_d_n12, assign24610_e33627_d_n17,) = {
    if (locals.var_guard769 == 0.0) {
        let assign24610_e33625: f64 = (-locals.var_vbsp);
        (assign24610_e33625, (-locals.var_vbsp_dn0), (-locals.var_vbsp_dn2), (-locals.var_vbsp_dn6), (-locals.var_vbsp_dn7), (-locals.var_vbsp_dn10), (-locals.var_vbsp_dn11), (-locals.var_vbsp_dn12), (-locals.var_vbsp_dn17),)
    } else {
        (locals.var_vsb, locals.var_vsb_dn0, locals.var_vsb_dn2, locals.var_vsb_dn6, locals.var_vsb_dn7, locals.var_vsb_dn10, locals.var_vsb_dn11, locals.var_vsb_dn12, locals.var_vsb_dn17,)
    }
};
        locals.var_vsb = assign24610_e33627;
        locals.var_vsb_dn0 = assign24610_e33627_d_n0;
        locals.var_vsb_dn2 = assign24610_e33627_d_n2;
        locals.var_vsb_dn6 = assign24610_e33627_d_n6;
        locals.var_vsb_dn7 = assign24610_e33627_d_n7;
        locals.var_vsb_dn10 = assign24610_e33627_d_n10;
        locals.var_vsb_dn11 = assign24610_e33627_d_n11;
        locals.var_vsb_dn12 = assign24610_e33627_d_n12;
        locals.var_vsb_dn17 = assign24610_e33627_d_n17;
        locals.var_vsb_rv = 0.0;

        let assign24620_e33630: f64 = if locals.var_vsb > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard772 = assign24620_e33630;
        locals.var_guard772_rv = 0.0;

        let (assign24630_e33639, assign24630_e33639_d_n0, assign24630_e33639_d_n2, assign24630_e33639_d_n6, assign24630_e33639_d_n7, assign24630_e33639_d_n10, assign24630_e33639_d_n11, assign24630_e33639_d_n12, assign24630_e33639_d_n17,) = {
    if ((locals.var_guard769 == 0.0) && (locals.var_guard772 != 0.0)) {
        let assign24630_e33637: f64 = (locals.var_vsb * locals.var_vsb);
        (assign24630_e33637, ((locals.var_vsb_dn0 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn0)), ((locals.var_vsb_dn2 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn2)), ((locals.var_vsb_dn6 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn6)), ((locals.var_vsb_dn7 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn7)), ((locals.var_vsb_dn10 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn10)), ((locals.var_vsb_dn11 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn11)), ((locals.var_vsb_dn12 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn12)), ((locals.var_vsb_dn17 * locals.var_vsb) + (locals.var_vsb * locals.var_vsb_dn17)),)
    } else {
        (locals.var_t2__blk763, locals.var_t2__blk763_dn0, locals.var_t2__blk763_dn2, locals.var_t2__blk763_dn6, locals.var_t2__blk763_dn7, locals.var_t2__blk763_dn10, locals.var_t2__blk763_dn11, locals.var_t2__blk763_dn12, locals.var_t2__blk763_dn17,)
    }
};
        locals.var_t2__blk763 = assign24630_e33639;
        locals.var_t2__blk763_dn0 = assign24630_e33639_d_n0;
        locals.var_t2__blk763_dn2 = assign24630_e33639_d_n2;
        locals.var_t2__blk763_dn6 = assign24630_e33639_d_n6;
        locals.var_t2__blk763_dn7 = assign24630_e33639_d_n7;
        locals.var_t2__blk763_dn10 = assign24630_e33639_d_n10;
        locals.var_t2__blk763_dn11 = assign24630_e33639_d_n11;
        locals.var_t2__blk763_dn12 = assign24630_e33639_d_n12;
        locals.var_t2__blk763_dn17 = assign24630_e33639_d_n17;
        locals.var_t2__blk763_rv = 0.0;

        let (assign24640_e33648, assign24640_e33648_d_n0, assign24640_e33648_d_n2, assign24640_e33648_d_n6, assign24640_e33648_d_n7, assign24640_e33648_d_n10, assign24640_e33648_d_n11, assign24640_e33648_d_n12, assign24640_e33648_d_n17,) = {
    if ((locals.var_guard769 == 0.0) && (locals.var_guard772 != 0.0)) {
        let assign24640_e33646: f64 = (locals.var_t2__blk763 * locals.var_vsb);
        (assign24640_e33646, ((locals.var_t2__blk763_dn0 * locals.var_vsb) + (locals.var_t2__blk763 * locals.var_vsb_dn0)), ((locals.var_t2__blk763_dn2 * locals.var_vsb) + (locals.var_t2__blk763 * locals.var_vsb_dn2)), ((locals.var_t2__blk763_dn6 * locals.var_vsb) + (locals.var_t2__blk763 * locals.var_vsb_dn6)), ((locals.var_t2__blk763_dn7 * locals.var_vsb) + (locals.var_t2__blk763 * locals.var_vsb_dn7)), ((locals.var_t2__blk763_dn10 * locals.var_vsb) + (locals.var_t2__blk763 * locals.var_vsb_dn10)), ((locals.var_t2__blk763_dn11 * locals.var_vsb) + (locals.var_t2__blk763 * locals.var_vsb_dn11)), ((locals.var_t2__blk763_dn12 * locals.var_vsb) + (locals.var_t2__blk763 * locals.var_vsb_dn12)), ((locals.var_t2__blk763_dn17 * locals.var_vsb) + (locals.var_t2__blk763 * locals.var_vsb_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign24640_e33648;
        locals.var_t4_dn0 = assign24640_e33648_d_n0;
        locals.var_t4_dn2 = assign24640_e33648_d_n2;
        locals.var_t4_dn6 = assign24640_e33648_d_n6;
        locals.var_t4_dn7 = assign24640_e33648_d_n7;
        locals.var_t4_dn10 = assign24640_e33648_d_n10;
        locals.var_t4_dn11 = assign24640_e33648_d_n11;
        locals.var_t4_dn12 = assign24640_e33648_d_n12;
        locals.var_t4_dn17 = assign24640_e33648_d_n17;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_87(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24650_e33657, assign24650_e33657_d_n0, assign24650_e33657_d_n2, assign24650_e33657_d_n6, assign24650_e33657_d_n7, assign24650_e33657_d_n10, assign24650_e33657_d_n11, assign24650_e33657_d_n12, assign24650_e33657_d_n17,) = {
    if ((locals.var_guard769 == 0.0) && (locals.var_guard772 != 0.0)) {
        let assign24650_e33655: f64 = (locals.var_t4 + p.p212);
        (assign24650_e33655, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    } else {
        (locals.var_t0__blk766, locals.var_t0__blk766_dn0, locals.var_t0__blk766_dn2, locals.var_t0__blk766_dn6, locals.var_t0__blk766_dn7, locals.var_t0__blk766_dn10, locals.var_t0__blk766_dn11, locals.var_t0__blk766_dn12, locals.var_t0__blk766_dn17,)
    }
};
        locals.var_t0__blk766 = assign24650_e33657;
        locals.var_t0__blk766_dn0 = assign24650_e33657_d_n0;
        locals.var_t0__blk766_dn2 = assign24650_e33657_d_n2;
        locals.var_t0__blk766_dn6 = assign24650_e33657_d_n6;
        locals.var_t0__blk766_dn7 = assign24650_e33657_d_n7;
        locals.var_t0__blk766_dn10 = assign24650_e33657_d_n10;
        locals.var_t0__blk766_dn11 = assign24650_e33657_d_n11;
        locals.var_t0__blk766_dn12 = assign24650_e33657_d_n12;
        locals.var_t0__blk766_dn17 = assign24650_e33657_d_n17;
        locals.var_t0__blk766_rv = 0.0;

        let assign24690_e33686: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard773 = assign24690_e33686;
        locals.var_guard773_rv = 0.0;

        let (assign24700_e33690,) = {
    if (locals.var_guard773 != 0.0) {
        (locals.var_c_fox0,)
    } else {
        (locals.var_cox0,)
    }
};
        locals.var_cox0 = assign24700_e33690;
        locals.var_cox0_rv = 0.0;

        let (assign24710_e33696,) = {
    if (locals.var_guard773 != 0.0) {
        let assign24710_e33694: f64 = (1.0 / locals.var_cox0);
        (assign24710_e33694,)
    } else {
        (locals.var_cox0_inv,)
    }
};
        locals.var_cox0_inv = assign24710_e33696;
        locals.var_cox0_inv_rv = 0.0;

        let (assign24720_e33700, assign24720_e33700_d_n0, assign24720_e33700_d_n2, assign24720_e33700_d_n6, assign24720_e33700_d_n7, assign24720_e33700_d_n10, assign24720_e33700_d_n11, assign24720_e33700_d_n12, assign24720_e33700_d_n17,) = {
    if (locals.var_guard773 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01__blk840, locals.var_fs01__blk840_dn0, locals.var_fs01__blk840_dn2, locals.var_fs01__blk840_dn6, locals.var_fs01__blk840_dn7, locals.var_fs01__blk840_dn10, locals.var_fs01__blk840_dn11, locals.var_fs01__blk840_dn12, locals.var_fs01__blk840_dn17,)
    }
};
        locals.var_fs01__blk840 = assign24720_e33700;
        locals.var_fs01__blk840_dn0 = assign24720_e33700_d_n0;
        locals.var_fs01__blk840_dn2 = assign24720_e33700_d_n2;
        locals.var_fs01__blk840_dn6 = assign24720_e33700_d_n6;
        locals.var_fs01__blk840_dn7 = assign24720_e33700_d_n7;
        locals.var_fs01__blk840_dn10 = assign24720_e33700_d_n10;
        locals.var_fs01__blk840_dn11 = assign24720_e33700_d_n11;
        locals.var_fs01__blk840_dn12 = assign24720_e33700_d_n12;
        locals.var_fs01__blk840_dn17 = assign24720_e33700_d_n17;
        locals.var_fs01__blk840_rv = 0.0;

        let (assign24730_e33704, assign24730_e33704_d_n0, assign24730_e33704_d_n2, assign24730_e33704_d_n6, assign24730_e33704_d_n7, assign24730_e33704_d_n10, assign24730_e33704_d_n11, assign24730_e33704_d_n12, assign24730_e33704_d_n17,) = {
    if (locals.var_guard773 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb__blk842, locals.var_fb__blk842_dn0, locals.var_fb__blk842_dn2, locals.var_fb__blk842_dn6, locals.var_fb__blk842_dn7, locals.var_fb__blk842_dn10, locals.var_fb__blk842_dn11, locals.var_fb__blk842_dn12, locals.var_fb__blk842_dn17,)
    }
};
        locals.var_fb__blk842 = assign24730_e33704;
        locals.var_fb__blk842_dn0 = assign24730_e33704_d_n0;
        locals.var_fb__blk842_dn2 = assign24730_e33704_d_n2;
        locals.var_fb__blk842_dn6 = assign24730_e33704_d_n6;
        locals.var_fb__blk842_dn7 = assign24730_e33704_d_n7;
        locals.var_fb__blk842_dn10 = assign24730_e33704_d_n10;
        locals.var_fb__blk842_dn11 = assign24730_e33704_d_n11;
        locals.var_fb__blk842_dn12 = assign24730_e33704_d_n12;
        locals.var_fb__blk842_dn17 = assign24730_e33704_d_n17;
        locals.var_fb__blk842_rv = 0.0;

        let (assign24740_e33708, assign24740_e33708_d_n0, assign24740_e33708_d_n2, assign24740_e33708_d_n6, assign24740_e33708_d_n7, assign24740_e33708_d_n10, assign24740_e33708_d_n11, assign24740_e33708_d_n12, assign24740_e33708_d_n17,) = {
    if (locals.var_guard773 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02__blk844, locals.var_fs02__blk844_dn0, locals.var_fs02__blk844_dn2, locals.var_fs02__blk844_dn6, locals.var_fs02__blk844_dn7, locals.var_fs02__blk844_dn10, locals.var_fs02__blk844_dn11, locals.var_fs02__blk844_dn12, locals.var_fs02__blk844_dn17,)
    }
};
        locals.var_fs02__blk844 = assign24740_e33708;
        locals.var_fs02__blk844_dn0 = assign24740_e33708_d_n0;
        locals.var_fs02__blk844_dn2 = assign24740_e33708_d_n2;
        locals.var_fs02__blk844_dn6 = assign24740_e33708_d_n6;
        locals.var_fs02__blk844_dn7 = assign24740_e33708_d_n7;
        locals.var_fs02__blk844_dn10 = assign24740_e33708_d_n10;
        locals.var_fs02__blk844_dn11 = assign24740_e33708_d_n11;
        locals.var_fs02__blk844_dn12 = assign24740_e33708_d_n12;
        locals.var_fs02__blk844_dn17 = assign24740_e33708_d_n17;
        locals.var_fs02__blk844_rv = 0.0;

        let (assign24750_e33713, assign24750_e33713_d_n0, assign24750_e33713_d_n2, assign24750_e33713_d_n6, assign24750_e33713_d_n7, assign24750_e33713_d_n10, assign24750_e33713_d_n11, assign24750_e33713_d_n12, assign24750_e33713_d_n17,) = {
    if (locals.var_guard773 != 0.0) {
        let assign24750_e33711: f64 = (-locals.var_area_bt_n);
        (assign24750_e33711, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk776, locals.var_t2__blk776_dn0, locals.var_t2__blk776_dn2, locals.var_t2__blk776_dn6, locals.var_t2__blk776_dn7, locals.var_t2__blk776_dn10, locals.var_t2__blk776_dn11, locals.var_t2__blk776_dn12, locals.var_t2__blk776_dn17,)
    }
};
        locals.var_t2__blk776 = assign24750_e33713;
        locals.var_t2__blk776_dn0 = assign24750_e33713_d_n0;
        locals.var_t2__blk776_dn2 = assign24750_e33713_d_n2;
        locals.var_t2__blk776_dn6 = assign24750_e33713_d_n6;
        locals.var_t2__blk776_dn7 = assign24750_e33713_d_n7;
        locals.var_t2__blk776_dn10 = assign24750_e33713_d_n10;
        locals.var_t2__blk776_dn11 = assign24750_e33713_d_n11;
        locals.var_t2__blk776_dn12 = assign24750_e33713_d_n12;
        locals.var_t2__blk776_dn17 = assign24750_e33713_d_n17;
        locals.var_t2__blk776_rv = 0.0;

        let (assign24760_e33719, assign24760_e33719_d_n0, assign24760_e33719_d_n2, assign24760_e33719_d_n6, assign24760_e33719_d_n7, assign24760_e33719_d_n10, assign24760_e33719_d_n11, assign24760_e33719_d_n12, assign24760_e33719_d_n17,) = {
    if (locals.var_guard773 != 0.0) {
        let assign24760_e33717: f64 = (locals.var_t2__blk776 * locals.var_qiu);
        (assign24760_e33717, ((locals.var_t2__blk776_dn0 * locals.var_qiu) + (locals.var_t2__blk776 * locals.var_qiu_dn0)), ((locals.var_t2__blk776_dn2 * locals.var_qiu) + (locals.var_t2__blk776 * locals.var_qiu_dn2)), ((locals.var_t2__blk776_dn6 * locals.var_qiu) + (locals.var_t2__blk776 * locals.var_qiu_dn6)), ((locals.var_t2__blk776_dn7 * locals.var_qiu) + (locals.var_t2__blk776 * locals.var_qiu_dn7)), ((locals.var_t2__blk776_dn10 * locals.var_qiu) + (locals.var_t2__blk776 * locals.var_qiu_dn10)), ((locals.var_t2__blk776_dn11 * locals.var_qiu) + (locals.var_t2__blk776 * locals.var_qiu_dn11)), ((locals.var_t2__blk776_dn12 * locals.var_qiu) + (locals.var_t2__blk776 * locals.var_qiu_dn12)), ((locals.var_t2__blk776_dn17 * locals.var_qiu) + (locals.var_t2__blk776 * locals.var_qiu_dn17)),)
    } else {
        (locals.var_t3__blk777, locals.var_t3__blk777_dn0, locals.var_t3__blk777_dn2, locals.var_t3__blk777_dn6, locals.var_t3__blk777_dn7, locals.var_t3__blk777_dn10, locals.var_t3__blk777_dn11, locals.var_t3__blk777_dn12, locals.var_t3__blk777_dn17,)
    }
};
        locals.var_t3__blk777 = assign24760_e33719;
        locals.var_t3__blk777_dn0 = assign24760_e33719_d_n0;
        locals.var_t3__blk777_dn2 = assign24760_e33719_d_n2;
        locals.var_t3__blk777_dn6 = assign24760_e33719_d_n6;
        locals.var_t3__blk777_dn7 = assign24760_e33719_d_n7;
        locals.var_t3__blk777_dn10 = assign24760_e33719_d_n10;
        locals.var_t3__blk777_dn11 = assign24760_e33719_d_n11;
        locals.var_t3__blk777_dn12 = assign24760_e33719_d_n12;
        locals.var_t3__blk777_dn17 = assign24760_e33719_d_n17;
        locals.var_t3__blk777_rv = 0.0;

        let (assign24770_e33727, assign24770_e33727_d_n0, assign24770_e33727_d_n2, assign24770_e33727_d_n6, assign24770_e33727_d_n7, assign24770_e33727_d_n10, assign24770_e33727_d_n11, assign24770_e33727_d_n12, assign24770_e33727_d_n17,) = {
    if (locals.var_guard773 != 0.0) {
        let assign24770_e33724: f64 = (locals.var_t2__blk776 * locals.var_qbu);
        let assign24770_e33725: f64 = (locals.var_t3__blk777 + assign24770_e33724);
        (assign24770_e33725, (locals.var_t3__blk777_dn0 + ((locals.var_t2__blk776_dn0 * locals.var_qbu) + (locals.var_t2__blk776 * locals.var_qbu_dn0))), (locals.var_t3__blk777_dn2 + ((locals.var_t2__blk776_dn2 * locals.var_qbu) + (locals.var_t2__blk776 * locals.var_qbu_dn2))), (locals.var_t3__blk777_dn6 + ((locals.var_t2__blk776_dn6 * locals.var_qbu) + (locals.var_t2__blk776 * locals.var_qbu_dn6))), (locals.var_t3__blk777_dn7 + ((locals.var_t2__blk776_dn7 * locals.var_qbu) + (locals.var_t2__blk776 * locals.var_qbu_dn7))), (locals.var_t3__blk777_dn10 + ((locals.var_t2__blk776_dn10 * locals.var_qbu) + (locals.var_t2__blk776 * locals.var_qbu_dn10))), (locals.var_t3__blk777_dn11 + ((locals.var_t2__blk776_dn11 * locals.var_qbu) + (locals.var_t2__blk776 * locals.var_qbu_dn11))), (locals.var_t3__blk777_dn12 + ((locals.var_t2__blk776_dn12 * locals.var_qbu) + (locals.var_t2__blk776 * locals.var_qbu_dn12))), (locals.var_t3__blk777_dn17 + ((locals.var_t2__blk776_dn17 * locals.var_qbu) + (locals.var_t2__blk776 * locals.var_qbu_dn17))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign24770_e33727;
        locals.var_t4_dn0 = assign24770_e33727_d_n0;
        locals.var_t4_dn2 = assign24770_e33727_d_n2;
        locals.var_t4_dn6 = assign24770_e33727_d_n6;
        locals.var_t4_dn7 = assign24770_e33727_d_n7;
        locals.var_t4_dn10 = assign24770_e33727_d_n10;
        locals.var_t4_dn11 = assign24770_e33727_d_n11;
        locals.var_t4_dn12 = assign24770_e33727_d_n12;
        locals.var_t4_dn17 = assign24770_e33727_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign24780_e33733, assign24780_e33733_d_n0, assign24780_e33733_d_n2, assign24780_e33733_d_n6, assign24780_e33733_d_n7, assign24780_e33733_d_n10, assign24780_e33733_d_n11, assign24780_e33733_d_n12, assign24780_e33733_d_n17,) = {
    if (locals.var_guard773 != 0.0) {
        let assign24780_e33731: f64 = (locals.var_t3__blk777 * locals.var_qdrat);
        (assign24780_e33731, ((locals.var_t3__blk777_dn0 * locals.var_qdrat) + (locals.var_t3__blk777 * locals.var_qdrat_dn0)), ((locals.var_t3__blk777_dn2 * locals.var_qdrat) + (locals.var_t3__blk777 * locals.var_qdrat_dn2)), ((locals.var_t3__blk777_dn6 * locals.var_qdrat) + (locals.var_t3__blk777 * locals.var_qdrat_dn6)), ((locals.var_t3__blk777_dn7 * locals.var_qdrat) + (locals.var_t3__blk777 * locals.var_qdrat_dn7)), ((locals.var_t3__blk777_dn10 * locals.var_qdrat) + (locals.var_t3__blk777 * locals.var_qdrat_dn10)), ((locals.var_t3__blk777_dn11 * locals.var_qdrat) + (locals.var_t3__blk777 * locals.var_qdrat_dn11)), ((locals.var_t3__blk777_dn12 * locals.var_qdrat) + (locals.var_t3__blk777 * locals.var_qdrat_dn12)), ((locals.var_t3__blk777_dn17 * locals.var_qdrat) + (locals.var_t3__blk777 * locals.var_qdrat_dn17)),)
    } else {
        (locals.var_qbody_bt_n_iud, locals.var_qbody_bt_n_iud_dn0, locals.var_qbody_bt_n_iud_dn2, locals.var_qbody_bt_n_iud_dn6, locals.var_qbody_bt_n_iud_dn7, locals.var_qbody_bt_n_iud_dn10, locals.var_qbody_bt_n_iud_dn11, locals.var_qbody_bt_n_iud_dn12, locals.var_qbody_bt_n_iud_dn17,)
    }
};
        locals.var_qbody_bt_n_iud = assign24780_e33733;
        locals.var_qbody_bt_n_iud_dn0 = assign24780_e33733_d_n0;
        locals.var_qbody_bt_n_iud_dn2 = assign24780_e33733_d_n2;
        locals.var_qbody_bt_n_iud_dn6 = assign24780_e33733_d_n6;
        locals.var_qbody_bt_n_iud_dn7 = assign24780_e33733_d_n7;
        locals.var_qbody_bt_n_iud_dn10 = assign24780_e33733_d_n10;
        locals.var_qbody_bt_n_iud_dn11 = assign24780_e33733_d_n11;
        locals.var_qbody_bt_n_iud_dn12 = assign24780_e33733_d_n12;
        locals.var_qbody_bt_n_iud_dn17 = assign24780_e33733_d_n17;
        locals.var_qbody_bt_n_iud_rv = 0.0;

        let (assign24790_e33739, assign24790_e33739_d_n0, assign24790_e33739_d_n2, assign24790_e33739_d_n6, assign24790_e33739_d_n7, assign24790_e33739_d_n10, assign24790_e33739_d_n11, assign24790_e33739_d_n12, assign24790_e33739_d_n17,) = {
    if (locals.var_guard773 != 0.0) {
        let assign24790_e33737: f64 = (locals.var_t3__blk777 - locals.var_qbody_bt_n_iud);
        (assign24790_e33737, (locals.var_t3__blk777_dn0 - locals.var_qbody_bt_n_iud_dn0), (locals.var_t3__blk777_dn2 - locals.var_qbody_bt_n_iud_dn2), (locals.var_t3__blk777_dn6 - locals.var_qbody_bt_n_iud_dn6), (locals.var_t3__blk777_dn7 - locals.var_qbody_bt_n_iud_dn7), (locals.var_t3__blk777_dn10 - locals.var_qbody_bt_n_iud_dn10), (locals.var_t3__blk777_dn11 - locals.var_qbody_bt_n_iud_dn11), (locals.var_t3__blk777_dn12 - locals.var_qbody_bt_n_iud_dn12), (locals.var_t3__blk777_dn17 - locals.var_qbody_bt_n_iud_dn17),)
    } else {
        (locals.var_qbody_bt_n_ius, locals.var_qbody_bt_n_ius_dn0, locals.var_qbody_bt_n_ius_dn2, locals.var_qbody_bt_n_ius_dn6, locals.var_qbody_bt_n_ius_dn7, locals.var_qbody_bt_n_ius_dn10, locals.var_qbody_bt_n_ius_dn11, locals.var_qbody_bt_n_ius_dn12, locals.var_qbody_bt_n_ius_dn17,)
    }
};
        locals.var_qbody_bt_n_ius = assign24790_e33739;
        locals.var_qbody_bt_n_ius_dn0 = assign24790_e33739_d_n0;
        locals.var_qbody_bt_n_ius_dn2 = assign24790_e33739_d_n2;
        locals.var_qbody_bt_n_ius_dn6 = assign24790_e33739_d_n6;
        locals.var_qbody_bt_n_ius_dn7 = assign24790_e33739_d_n7;
        locals.var_qbody_bt_n_ius_dn10 = assign24790_e33739_d_n10;
        locals.var_qbody_bt_n_ius_dn11 = assign24790_e33739_d_n11;
        locals.var_qbody_bt_n_ius_dn12 = assign24790_e33739_d_n12;
        locals.var_qbody_bt_n_ius_dn17 = assign24790_e33739_d_n17;
        locals.var_qbody_bt_n_ius_rv = 0.0;

        let (assign24800_e33745, assign24800_e33745_d_n0, assign24800_e33745_d_n2, assign24800_e33745_d_n6, assign24800_e33745_d_n7, assign24800_e33745_d_n10, assign24800_e33745_d_n11, assign24800_e33745_d_n12, assign24800_e33745_d_n17,) = {
    if (locals.var_guard773 != 0.0) {
        let assign24800_e33743: f64 = (locals.var_t4 * locals.var_qdrat);
        (assign24800_e33743, ((locals.var_t4_dn0 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn0)), ((locals.var_t4_dn2 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn2)), ((locals.var_t4_dn6 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn6)), ((locals.var_t4_dn7 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn7)), ((locals.var_t4_dn10 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn10)), ((locals.var_t4_dn11 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn11)), ((locals.var_t4_dn12 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn12)), ((locals.var_t4_dn17 * locals.var_qdrat) + (locals.var_t4 * locals.var_qdrat_dn17)),)
    } else {
        (locals.var_qbody_bt_n_sud, locals.var_qbody_bt_n_sud_dn0, locals.var_qbody_bt_n_sud_dn2, locals.var_qbody_bt_n_sud_dn6, locals.var_qbody_bt_n_sud_dn7, locals.var_qbody_bt_n_sud_dn10, locals.var_qbody_bt_n_sud_dn11, locals.var_qbody_bt_n_sud_dn12, locals.var_qbody_bt_n_sud_dn17,)
    }
};
        locals.var_qbody_bt_n_sud = assign24800_e33745;
        locals.var_qbody_bt_n_sud_dn0 = assign24800_e33745_d_n0;
        locals.var_qbody_bt_n_sud_dn2 = assign24800_e33745_d_n2;
        locals.var_qbody_bt_n_sud_dn6 = assign24800_e33745_d_n6;
        locals.var_qbody_bt_n_sud_dn7 = assign24800_e33745_d_n7;
        locals.var_qbody_bt_n_sud_dn10 = assign24800_e33745_d_n10;
        locals.var_qbody_bt_n_sud_dn11 = assign24800_e33745_d_n11;
        locals.var_qbody_bt_n_sud_dn12 = assign24800_e33745_d_n12;
        locals.var_qbody_bt_n_sud_dn17 = assign24800_e33745_d_n17;
        locals.var_qbody_bt_n_sud_rv = 0.0;

        let (assign24810_e33751, assign24810_e33751_d_n0, assign24810_e33751_d_n2, assign24810_e33751_d_n6, assign24810_e33751_d_n7, assign24810_e33751_d_n10, assign24810_e33751_d_n11, assign24810_e33751_d_n12, assign24810_e33751_d_n17,) = {
    if (locals.var_guard773 != 0.0) {
        let assign24810_e33749: f64 = (locals.var_t4 - locals.var_qbody_bt_n_sud);
        (assign24810_e33749, (locals.var_t4_dn0 - locals.var_qbody_bt_n_sud_dn0), (locals.var_t4_dn2 - locals.var_qbody_bt_n_sud_dn2), (locals.var_t4_dn6 - locals.var_qbody_bt_n_sud_dn6), (locals.var_t4_dn7 - locals.var_qbody_bt_n_sud_dn7), (locals.var_t4_dn10 - locals.var_qbody_bt_n_sud_dn10), (locals.var_t4_dn11 - locals.var_qbody_bt_n_sud_dn11), (locals.var_t4_dn12 - locals.var_qbody_bt_n_sud_dn12), (locals.var_t4_dn17 - locals.var_qbody_bt_n_sud_dn17),)
    } else {
        (locals.var_qbody_bt_n_sus, locals.var_qbody_bt_n_sus_dn0, locals.var_qbody_bt_n_sus_dn2, locals.var_qbody_bt_n_sus_dn6, locals.var_qbody_bt_n_sus_dn7, locals.var_qbody_bt_n_sus_dn10, locals.var_qbody_bt_n_sus_dn11, locals.var_qbody_bt_n_sus_dn12, locals.var_qbody_bt_n_sus_dn17,)
    }
};
        locals.var_qbody_bt_n_sus = assign24810_e33751;
        locals.var_qbody_bt_n_sus_dn0 = assign24810_e33751_d_n0;
        locals.var_qbody_bt_n_sus_dn2 = assign24810_e33751_d_n2;
        locals.var_qbody_bt_n_sus_dn6 = assign24810_e33751_d_n6;
        locals.var_qbody_bt_n_sus_dn7 = assign24810_e33751_d_n7;
        locals.var_qbody_bt_n_sus_dn10 = assign24810_e33751_d_n10;
        locals.var_qbody_bt_n_sus_dn11 = assign24810_e33751_d_n11;
        locals.var_qbody_bt_n_sus_dn12 = assign24810_e33751_d_n12;
        locals.var_qbody_bt_n_sus_dn17 = assign24810_e33751_d_n17;
        locals.var_qbody_bt_n_sus_rv = 0.0;

        let (assign24820_e33757, assign24820_e33757_d_n0, assign24820_e33757_d_n2, assign24820_e33757_d_n6, assign24820_e33757_d_n7, assign24820_e33757_d_n10, assign24820_e33757_d_n11, assign24820_e33757_d_n12, assign24820_e33757_d_n17,) = {
    if ((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn10, locals.var_nsub_dn11, locals.var_nsub_dn12, locals.var_nsub_dn17,)
    } else {
        (locals.var_uc_nsubbttub, locals.var_uc_nsubbttub_dn0, locals.var_uc_nsubbttub_dn2, locals.var_uc_nsubbttub_dn6, locals.var_uc_nsubbttub_dn7, locals.var_uc_nsubbttub_dn10, locals.var_uc_nsubbttub_dn11, locals.var_uc_nsubbttub_dn12, locals.var_uc_nsubbttub_dn17,)
    }
};
        locals.var_uc_nsubbttub = assign24820_e33757;
        locals.var_uc_nsubbttub_dn0 = assign24820_e33757_d_n0;
        locals.var_uc_nsubbttub_dn2 = assign24820_e33757_d_n2;
        locals.var_uc_nsubbttub_dn6 = assign24820_e33757_d_n6;
        locals.var_uc_nsubbttub_dn7 = assign24820_e33757_d_n7;
        locals.var_uc_nsubbttub_dn10 = assign24820_e33757_d_n10;
        locals.var_uc_nsubbttub_dn11 = assign24820_e33757_d_n11;
        locals.var_uc_nsubbttub_dn12 = assign24820_e33757_d_n12;
        locals.var_uc_nsubbttub_dn17 = assign24820_e33757_d_n17;
        locals.var_uc_nsubbttub_rv = 0.0;

        let (assign24830_e33763,) = {
    if ((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_cbtb_given,)
    }
};
        locals.var_cbtb_given = assign24830_e33763;
        locals.var_cbtb_given_rv = 0.0;

        let assign24840_e33766: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard853 = assign24840_e33766;
        locals.var_guard853_rv = 0.0;

        let assign24850_e33769: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard854 = assign24850_e33769;
        locals.var_guard854_rv = 0.0;

        let (assign24860_e33779,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign24860_e33777: f64 = (locals.var_area_bt_p * 0.5);
        (assign24860_e33777,)
    } else {
        (locals.var_uc_areabt,)
    }
};
        locals.var_uc_areabt = assign24860_e33779;
        locals.var_uc_areabt_rv = 0.0;

        let (assign24870_e33787,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        (p.p292,)
    } else {
        (locals.var_uc_vfbbt,)
    }
};
        locals.var_uc_vfbbt = assign24870_e33787;
        locals.var_uc_vfbbt_rv = 0.0;

        let (assign24880_e33795,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        (locals.var_cbtbp_given,)
    } else {
        (locals.var_cbtb_given,)
    }
};
        locals.var_cbtb_given = assign24880_e33795;
        locals.var_cbtb_given_rv = 0.0;

        let (assign24890_e33808,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && ((locals.var_guard854 != 0.0) && (locals.var_guard853 == 0.0))) {
        let assign24890_e33806: f64 = (locals.var_area_bt_n * 0.5);
        (assign24890_e33806,)
    } else {
        (locals.var_uc_areabt,)
    }
};
        locals.var_uc_areabt = assign24890_e33808;
        locals.var_uc_areabt_rv = 0.0;

        let (assign24900_e33819,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && ((locals.var_guard854 != 0.0) && (locals.var_guard853 == 0.0))) {
        (p.p68,)
    } else {
        (locals.var_uc_vfbbt,)
    }
};
        locals.var_uc_vfbbt = assign24900_e33819;
        locals.var_uc_vfbbt_rv = 0.0;

        let (assign24910_e33830,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && ((locals.var_guard854 != 0.0) && (locals.var_guard853 == 0.0))) {
        (locals.var_cbtbn_given,)
    } else {
        (locals.var_cbtb_given,)
    }
};
        locals.var_cbtb_given = assign24910_e33830;
        locals.var_cbtb_given_rv = 0.0;

        let (assign24920_e33841,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && ((locals.var_guard854 != 0.0) && (locals.var_guard853 == 0.0))) {
        (1.0,)
    } else {
        (locals.var_cbtb_given,)
    }
};
        locals.var_cbtb_given = assign24920_e33841;
        locals.var_cbtb_given_rv = 0.0;

        let assign24930_e33844: f64 = if locals.var_cbtb_given == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard855 = assign24930_e33844;
        locals.var_guard855_rv = 0.0;

        let (assign24940_e33857, assign24940_e33857_d_n0, assign24940_e33857_d_n2, assign24940_e33857_d_n6, assign24940_e33857_d_n7, assign24940_e33857_d_n10, assign24940_e33857_d_n11, assign24940_e33857_d_n12, assign24940_e33857_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign24940_e33853: f64 = (locals.var_uc_nsubbttub / locals.var_nsub);
        let assign24940_e33854: f64 = (assign24940_e33853).sqrt();
        let assign24940_e33855: f64 = (locals.var_cnst0soi * assign24940_e33854);
        (assign24940_e33855, ((locals.var_cnst0soi_dn0 * assign24940_e33854) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn0 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24940_e33854)))), ((locals.var_cnst0soi_dn2 * assign24940_e33854) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn2 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24940_e33854)))), ((locals.var_cnst0soi_dn6 * assign24940_e33854) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn6 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24940_e33854)))), ((locals.var_cnst0soi_dn7 * assign24940_e33854) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn7 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24940_e33854)))), ((locals.var_cnst0soi_dn10 * assign24940_e33854) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn10 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24940_e33854)))), ((locals.var_cnst0soi_dn11 * assign24940_e33854) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn11 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn11)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24940_e33854)))), ((locals.var_cnst0soi_dn12 * assign24940_e33854) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn12 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn12)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24940_e33854)))), ((locals.var_cnst0soi_dn17 * assign24940_e33854) + (locals.var_cnst0soi * ((((locals.var_uc_nsubbttub_dn17 * locals.var_nsub) - (locals.var_uc_nsubbttub * locals.var_nsub_dn17)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign24940_e33854)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn12, locals.var_cnst0over_dn17,)
    }
};
        locals.var_cnst0over = assign24940_e33857;
        locals.var_cnst0over_dn0 = assign24940_e33857_d_n0;
        locals.var_cnst0over_dn2 = assign24940_e33857_d_n2;
        locals.var_cnst0over_dn6 = assign24940_e33857_d_n6;
        locals.var_cnst0over_dn7 = assign24940_e33857_d_n7;
        locals.var_cnst0over_dn10 = assign24940_e33857_d_n10;
        locals.var_cnst0over_dn11 = assign24940_e33857_d_n11;
        locals.var_cnst0over_dn12 = assign24940_e33857_d_n12;
        locals.var_cnst0over_dn17 = assign24940_e33857_d_n17;
        locals.var_cnst0over_rv = 0.0;

        let (assign24950_e33869,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign24950_e33865: f64 = (1.0 - -1.0);
        let assign24950_e33867: f64 = (assign24950_e33865 / 2.0);
        (assign24950_e33867,)
    } else {
        (locals.var_flg_ovloops,)
    }
};
        locals.var_flg_ovloops = assign24950_e33869;
        locals.var_flg_ovloops_rv = 0.0;

        let (assign24960_e33881,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign24960_e33877: f64 = (1.0 + -1.0);
        let assign24960_e33879: f64 = (assign24960_e33877 / 2.0);
        (assign24960_e33879,)
    } else {
        (locals.var_flg_ovloopd,)
    }
};
        locals.var_flg_ovloopd = assign24960_e33881;
        locals.var_flg_ovloopd_rv = 0.0;

        let (assign24970_e33897, assign24970_e33897_d_n0, assign24970_e33897_d_n2, assign24970_e33897_d_n6, assign24970_e33897_d_n7, assign24970_e33897_d_n10, assign24970_e33897_d_n11, assign24970_e33897_d_n12, assign24970_e33897_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign24970_e33889: f64 = (locals.var_modenml * locals.var_vbs);
        let assign24970_e33893: f64 = (locals.var_vbs - locals.var_vds);
        let assign24970_e33894: f64 = (locals.var_modervs * assign24970_e33893);
        let assign24970_e33895: f64 = (assign24970_e33889 + assign24970_e33894);
        (assign24970_e33895, ((locals.var_modenml * locals.var_vbs_dn0) + (locals.var_modervs * (locals.var_vbs_dn0 - locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vbs_dn2) + (locals.var_modervs * (locals.var_vbs_dn2 - locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vbs_dn6) + (locals.var_modervs * (locals.var_vbs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vbs_dn7) + (locals.var_modervs * (locals.var_vbs_dn7 - locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vbs_dn10) + (locals.var_modervs * (locals.var_vbs_dn10 - locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vbs_dn11) + (locals.var_modervs * (locals.var_vbs_dn11 - locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vbs_dn12) + (locals.var_modervs * (locals.var_vbs_dn12 - locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vbs_dn17) + (locals.var_modervs * (locals.var_vbs_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_vbsgmt, locals.var_vbsgmt_dn0, locals.var_vbsgmt_dn2, locals.var_vbsgmt_dn6, locals.var_vbsgmt_dn7, locals.var_vbsgmt_dn10, locals.var_vbsgmt_dn11, locals.var_vbsgmt_dn12, locals.var_vbsgmt_dn17,)
    }
};
        locals.var_vbsgmt = assign24970_e33897;
        locals.var_vbsgmt_dn0 = assign24970_e33897_d_n0;
        locals.var_vbsgmt_dn2 = assign24970_e33897_d_n2;
        locals.var_vbsgmt_dn6 = assign24970_e33897_d_n6;
        locals.var_vbsgmt_dn7 = assign24970_e33897_d_n7;
        locals.var_vbsgmt_dn10 = assign24970_e33897_d_n10;
        locals.var_vbsgmt_dn11 = assign24970_e33897_d_n11;
        locals.var_vbsgmt_dn12 = assign24970_e33897_d_n12;
        locals.var_vbsgmt_dn17 = assign24970_e33897_d_n17;
        locals.var_vbsgmt_rv = 0.0;

        let (assign24980_e33912, assign24980_e33912_d_n0, assign24980_e33912_d_n2, assign24980_e33912_d_n6, assign24980_e33912_d_n7, assign24980_e33912_d_n10, assign24980_e33912_d_n11, assign24980_e33912_d_n12, assign24980_e33912_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign24980_e33905: f64 = (locals.var_modenml * locals.var_vds);
        let assign24980_e33908: f64 = (-locals.var_vds);
        let assign24980_e33909: f64 = (locals.var_modervs * assign24980_e33908);
        let assign24980_e33910: f64 = (assign24980_e33905 + assign24980_e33909);
        (assign24980_e33910, ((locals.var_modenml * locals.var_vds_dn0) + (locals.var_modervs * (-locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vds_dn2) + (locals.var_modervs * (-locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vds_dn6) + (locals.var_modervs * (-locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vds_dn7) + (locals.var_modervs * (-locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vds_dn10) + (locals.var_modervs * (-locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vds_dn11) + (locals.var_modervs * (-locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vds_dn12) + (locals.var_modervs * (-locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vds_dn17) + (locals.var_modervs * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_vdsgmt, locals.var_vdsgmt_dn0, locals.var_vdsgmt_dn2, locals.var_vdsgmt_dn6, locals.var_vdsgmt_dn7, locals.var_vdsgmt_dn10, locals.var_vdsgmt_dn11, locals.var_vdsgmt_dn12, locals.var_vdsgmt_dn17,)
    }
};
        locals.var_vdsgmt = assign24980_e33912;
        locals.var_vdsgmt_dn0 = assign24980_e33912_d_n0;
        locals.var_vdsgmt_dn2 = assign24980_e33912_d_n2;
        locals.var_vdsgmt_dn6 = assign24980_e33912_d_n6;
        locals.var_vdsgmt_dn7 = assign24980_e33912_d_n7;
        locals.var_vdsgmt_dn10 = assign24980_e33912_d_n10;
        locals.var_vdsgmt_dn11 = assign24980_e33912_d_n11;
        locals.var_vdsgmt_dn12 = assign24980_e33912_d_n12;
        locals.var_vdsgmt_dn17 = assign24980_e33912_d_n17;
        locals.var_vdsgmt_rv = 0.0;

        let (assign24990_e33928, assign24990_e33928_d_n0, assign24990_e33928_d_n2, assign24990_e33928_d_n6, assign24990_e33928_d_n7, assign24990_e33928_d_n10, assign24990_e33928_d_n11, assign24990_e33928_d_n12, assign24990_e33928_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign24990_e33920: f64 = (locals.var_modenml * locals.var_vgs);
        let assign24990_e33924: f64 = (locals.var_vgs - locals.var_vds);
        let assign24990_e33925: f64 = (locals.var_modervs * assign24990_e33924);
        let assign24990_e33926: f64 = (assign24990_e33920 + assign24990_e33925);
        (assign24990_e33926, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgsgmt, locals.var_vgsgmt_dn0, locals.var_vgsgmt_dn2, locals.var_vgsgmt_dn6, locals.var_vgsgmt_dn7, locals.var_vgsgmt_dn10, locals.var_vgsgmt_dn11, locals.var_vgsgmt_dn12, locals.var_vgsgmt_dn17,)
    }
};
        locals.var_vgsgmt = assign24990_e33928;
        locals.var_vgsgmt_dn0 = assign24990_e33928_d_n0;
        locals.var_vgsgmt_dn2 = assign24990_e33928_d_n2;
        locals.var_vgsgmt_dn6 = assign24990_e33928_d_n6;
        locals.var_vgsgmt_dn7 = assign24990_e33928_d_n7;
        locals.var_vgsgmt_dn10 = assign24990_e33928_d_n10;
        locals.var_vgsgmt_dn11 = assign24990_e33928_d_n11;
        locals.var_vgsgmt_dn12 = assign24990_e33928_d_n12;
        locals.var_vgsgmt_dn17 = assign24990_e33928_d_n17;
        locals.var_vgsgmt_rv = 0.0;

        let (assign25000_e33944, assign25000_e33944_d_n0, assign25000_e33944_d_n2, assign25000_e33944_d_n6, assign25000_e33944_d_n7, assign25000_e33944_d_n10, assign25000_e33944_d_n11, assign25000_e33944_d_n12, assign25000_e33944_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25000_e33936: f64 = (locals.var_modervs * locals.var_vgs);
        let assign25000_e33940: f64 = (locals.var_vgs - locals.var_vds);
        let assign25000_e33941: f64 = (locals.var_modenml * assign25000_e33940);
        let assign25000_e33942: f64 = (assign25000_e33936 + assign25000_e33941);
        (assign25000_e33942, (locals.var_modenml * (-locals.var_vds_dn0)), (locals.var_modenml * (-locals.var_vds_dn2)), ((locals.var_modervs * locals.var_vgs_dn6) + (locals.var_modenml * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modervs * locals.var_vgs_dn7) + (locals.var_modenml * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modenml * (-locals.var_vds_dn10)), ((locals.var_modervs * locals.var_vgs_dn11) + (locals.var_modenml * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modenml * (-locals.var_vds_dn12)), (locals.var_modenml * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgdgmt, locals.var_vgdgmt_dn0, locals.var_vgdgmt_dn2, locals.var_vgdgmt_dn6, locals.var_vgdgmt_dn7, locals.var_vgdgmt_dn10, locals.var_vgdgmt_dn11, locals.var_vgdgmt_dn12, locals.var_vgdgmt_dn17,)
    }
};
        locals.var_vgdgmt = assign25000_e33944;
        locals.var_vgdgmt_dn0 = assign25000_e33944_d_n0;
        locals.var_vgdgmt_dn2 = assign25000_e33944_d_n2;
        locals.var_vgdgmt_dn6 = assign25000_e33944_d_n6;
        locals.var_vgdgmt_dn7 = assign25000_e33944_d_n7;
        locals.var_vgdgmt_dn10 = assign25000_e33944_d_n10;
        locals.var_vgdgmt_dn11 = assign25000_e33944_d_n11;
        locals.var_vgdgmt_dn12 = assign25000_e33944_d_n12;
        locals.var_vgdgmt_dn17 = assign25000_e33944_d_n17;
        locals.var_vgdgmt_rv = 0.0;

        let (assign25010_e33954, assign25010_e33954_d_n0, assign25010_e33954_d_n2, assign25010_e33954_d_n6, assign25010_e33954_d_n7, assign25010_e33954_d_n10, assign25010_e33954_d_n11, assign25010_e33954_d_n12, assign25010_e33954_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25010_e33952: f64 = (locals.var_vdsgmt - locals.var_vbsgmt);
        (assign25010_e33952, (locals.var_vdsgmt_dn0 - locals.var_vbsgmt_dn0), (locals.var_vdsgmt_dn2 - locals.var_vbsgmt_dn2), (locals.var_vdsgmt_dn6 - locals.var_vbsgmt_dn6), (locals.var_vdsgmt_dn7 - locals.var_vbsgmt_dn7), (locals.var_vdsgmt_dn10 - locals.var_vbsgmt_dn10), (locals.var_vdsgmt_dn11 - locals.var_vbsgmt_dn11), (locals.var_vdsgmt_dn12 - locals.var_vbsgmt_dn12), (locals.var_vdsgmt_dn17 - locals.var_vbsgmt_dn17),)
    } else {
        (locals.var_vdbgmt, locals.var_vdbgmt_dn0, locals.var_vdbgmt_dn2, locals.var_vdbgmt_dn6, locals.var_vdbgmt_dn7, locals.var_vdbgmt_dn10, locals.var_vdbgmt_dn11, locals.var_vdbgmt_dn12, locals.var_vdbgmt_dn17,)
    }
};
        locals.var_vdbgmt = assign25010_e33954;
        locals.var_vdbgmt_dn0 = assign25010_e33954_d_n0;
        locals.var_vdbgmt_dn2 = assign25010_e33954_d_n2;
        locals.var_vdbgmt_dn6 = assign25010_e33954_d_n6;
        locals.var_vdbgmt_dn7 = assign25010_e33954_d_n7;
        locals.var_vdbgmt_dn10 = assign25010_e33954_d_n10;
        locals.var_vdbgmt_dn11 = assign25010_e33954_d_n11;
        locals.var_vdbgmt_dn12 = assign25010_e33954_d_n12;
        locals.var_vdbgmt_dn17 = assign25010_e33954_d_n17;
        locals.var_vdbgmt_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_88(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25020_e33963, assign25020_e33963_d_n0, assign25020_e33963_d_n2, assign25020_e33963_d_n6, assign25020_e33963_d_n7, assign25020_e33963_d_n10, assign25020_e33963_d_n11, assign25020_e33963_d_n12, assign25020_e33963_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25020_e33961: f64 = (-locals.var_vbsgmt);
        (assign25020_e33961, (-locals.var_vbsgmt_dn0), (-locals.var_vbsgmt_dn2), (-locals.var_vbsgmt_dn6), (-locals.var_vbsgmt_dn7), (-locals.var_vbsgmt_dn10), (-locals.var_vbsgmt_dn11), (-locals.var_vbsgmt_dn12), (-locals.var_vbsgmt_dn17),)
    } else {
        (locals.var_vsbgmt, locals.var_vsbgmt_dn0, locals.var_vsbgmt_dn2, locals.var_vsbgmt_dn6, locals.var_vsbgmt_dn7, locals.var_vsbgmt_dn10, locals.var_vsbgmt_dn11, locals.var_vsbgmt_dn12, locals.var_vsbgmt_dn17,)
    }
};
        locals.var_vsbgmt = assign25020_e33963;
        locals.var_vsbgmt_dn0 = assign25020_e33963_d_n0;
        locals.var_vsbgmt_dn2 = assign25020_e33963_d_n2;
        locals.var_vsbgmt_dn6 = assign25020_e33963_d_n6;
        locals.var_vsbgmt_dn7 = assign25020_e33963_d_n7;
        locals.var_vsbgmt_dn10 = assign25020_e33963_d_n10;
        locals.var_vsbgmt_dn11 = assign25020_e33963_d_n11;
        locals.var_vsbgmt_dn12 = assign25020_e33963_d_n12;
        locals.var_vsbgmt_dn17 = assign25020_e33963_d_n17;
        locals.var_vsbgmt_rv = 0.0;

        let (assign25030_e33977,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25030_e33971: f64 = (locals.var_flg_ovloops * locals.var_modenml);
        let assign25030_e33974: f64 = (locals.var_flg_ovloopd * locals.var_modervs);
        let assign25030_e33975: f64 = (assign25030_e33971 + assign25030_e33974);
        (assign25030_e33975,)
    } else {
        (locals.var_flg_overs,)
    }
};
        locals.var_flg_overs = assign25030_e33977;
        locals.var_flg_overs_rv = 0.0;

        let (assign25040_e33991,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25040_e33985: f64 = (locals.var_flg_ovloops * locals.var_modervs);
        let assign25040_e33988: f64 = (locals.var_flg_ovloopd * locals.var_modenml);
        let assign25040_e33989: f64 = (assign25040_e33985 + assign25040_e33988);
        (assign25040_e33989,)
    } else {
        (locals.var_flg_overd,)
    }
};
        locals.var_flg_overd = assign25040_e33991;
        locals.var_flg_overd_rv = 0.0;

        let (assign25050_e34005, assign25050_e34005_d_n0, assign25050_e34005_d_n2, assign25050_e34005_d_n6, assign25050_e34005_d_n7, assign25050_e34005_d_n10, assign25050_e34005_d_n11, assign25050_e34005_d_n12, assign25050_e34005_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25050_e33999: f64 = (locals.var_flg_overs * locals.var_vgsgmt);
        let assign25050_e34002: f64 = (locals.var_flg_overd * locals.var_vgdgmt);
        let assign25050_e34003: f64 = (assign25050_e33999 + assign25050_e34002);
        (assign25050_e34003, ((locals.var_flg_overs * locals.var_vgsgmt_dn0) + (locals.var_flg_overd * locals.var_vgdgmt_dn0)), ((locals.var_flg_overs * locals.var_vgsgmt_dn2) + (locals.var_flg_overd * locals.var_vgdgmt_dn2)), ((locals.var_flg_overs * locals.var_vgsgmt_dn6) + (locals.var_flg_overd * locals.var_vgdgmt_dn6)), ((locals.var_flg_overs * locals.var_vgsgmt_dn7) + (locals.var_flg_overd * locals.var_vgdgmt_dn7)), ((locals.var_flg_overs * locals.var_vgsgmt_dn10) + (locals.var_flg_overd * locals.var_vgdgmt_dn10)), ((locals.var_flg_overs * locals.var_vgsgmt_dn11) + (locals.var_flg_overd * locals.var_vgdgmt_dn11)), ((locals.var_flg_overs * locals.var_vgsgmt_dn12) + (locals.var_flg_overd * locals.var_vgdgmt_dn12)), ((locals.var_flg_overs * locals.var_vgsgmt_dn17) + (locals.var_flg_overd * locals.var_vgdgmt_dn17)),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn10, locals.var_vgbgmt_dn11, locals.var_vgbgmt_dn12, locals.var_vgbgmt_dn17,)
    }
};
        locals.var_vgbgmt = assign25050_e34005;
        locals.var_vgbgmt_dn0 = assign25050_e34005_d_n0;
        locals.var_vgbgmt_dn2 = assign25050_e34005_d_n2;
        locals.var_vgbgmt_dn6 = assign25050_e34005_d_n6;
        locals.var_vgbgmt_dn7 = assign25050_e34005_d_n7;
        locals.var_vgbgmt_dn10 = assign25050_e34005_d_n10;
        locals.var_vgbgmt_dn11 = assign25050_e34005_d_n11;
        locals.var_vgbgmt_dn12 = assign25050_e34005_d_n12;
        locals.var_vgbgmt_dn17 = assign25050_e34005_d_n17;
        locals.var_vgbgmt_rv = 0.0;

        let (assign25060_e34023, assign25060_e34023_d_n0, assign25060_e34023_d_n2, assign25060_e34023_d_n6, assign25060_e34023_d_n7, assign25060_e34023_d_n10, assign25060_e34023_d_n11, assign25060_e34023_d_n12, assign25060_e34023_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25060_e34013: f64 = (locals.var_flg_overs * locals.var_vsbgmt);
        let assign25060_e34016: f64 = (locals.var_flg_overd * locals.var_vdbgmt);
        let assign25060_e34017: f64 = (assign25060_e34013 + assign25060_e34016);
        let assign25060_e34020: f64 = (10.0 * 2.220446049250313e-16);
        let assign25060_e34021: f64 = (assign25060_e34017 + assign25060_e34020);
        (assign25060_e34021, ((locals.var_flg_overs * locals.var_vsbgmt_dn0) + (locals.var_flg_overd * locals.var_vdbgmt_dn0)), ((locals.var_flg_overs * locals.var_vsbgmt_dn2) + (locals.var_flg_overd * locals.var_vdbgmt_dn2)), ((locals.var_flg_overs * locals.var_vsbgmt_dn6) + (locals.var_flg_overd * locals.var_vdbgmt_dn6)), ((locals.var_flg_overs * locals.var_vsbgmt_dn7) + (locals.var_flg_overd * locals.var_vdbgmt_dn7)), ((locals.var_flg_overs * locals.var_vsbgmt_dn10) + (locals.var_flg_overd * locals.var_vdbgmt_dn10)), ((locals.var_flg_overs * locals.var_vsbgmt_dn11) + (locals.var_flg_overd * locals.var_vdbgmt_dn11)), ((locals.var_flg_overs * locals.var_vsbgmt_dn12) + (locals.var_flg_overd * locals.var_vdbgmt_dn12)), ((locals.var_flg_overs * locals.var_vsbgmt_dn17) + (locals.var_flg_overd * locals.var_vdbgmt_dn17)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn12, locals.var_vxbgmt_dn17,)
    }
};
        locals.var_vxbgmt = assign25060_e34023;
        locals.var_vxbgmt_dn0 = assign25060_e34023_d_n0;
        locals.var_vxbgmt_dn2 = assign25060_e34023_d_n2;
        locals.var_vxbgmt_dn6 = assign25060_e34023_d_n6;
        locals.var_vxbgmt_dn7 = assign25060_e34023_d_n7;
        locals.var_vxbgmt_dn10 = assign25060_e34023_d_n10;
        locals.var_vxbgmt_dn11 = assign25060_e34023_d_n11;
        locals.var_vxbgmt_dn12 = assign25060_e34023_d_n12;
        locals.var_vxbgmt_dn17 = assign25060_e34023_d_n17;
        locals.var_vxbgmt_rv = 0.0;

        let (assign25070_e34032, assign25070_e34032_d_n0, assign25070_e34032_d_n2, assign25070_e34032_d_n6, assign25070_e34032_d_n7, assign25070_e34032_d_n10, assign25070_e34032_d_n11, assign25070_e34032_d_n12, assign25070_e34032_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25070_e34030: f64 = (-locals.var_vxbgmt);
        (assign25070_e34030, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn11), (-locals.var_vxbgmt_dn12), (-locals.var_vxbgmt_dn17),)
    } else {
        (locals.var_t0__blk774, locals.var_t0__blk774_dn0, locals.var_t0__blk774_dn2, locals.var_t0__blk774_dn6, locals.var_t0__blk774_dn7, locals.var_t0__blk774_dn10, locals.var_t0__blk774_dn11, locals.var_t0__blk774_dn12, locals.var_t0__blk774_dn17,)
    }
};
        locals.var_t0__blk774 = assign25070_e34032;
        locals.var_t0__blk774_dn0 = assign25070_e34032_d_n0;
        locals.var_t0__blk774_dn2 = assign25070_e34032_d_n2;
        locals.var_t0__blk774_dn6 = assign25070_e34032_d_n6;
        locals.var_t0__blk774_dn7 = assign25070_e34032_d_n7;
        locals.var_t0__blk774_dn10 = assign25070_e34032_d_n10;
        locals.var_t0__blk774_dn11 = assign25070_e34032_d_n11;
        locals.var_t0__blk774_dn12 = assign25070_e34032_d_n12;
        locals.var_t0__blk774_dn17 = assign25070_e34032_d_n17;
        locals.var_t0__blk774_rv = 0.0;

        let assign25080_e34035: f64 = if locals.var_t0__blk774 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard856 = assign25080_e34035;
        locals.var_guard856_rv = 0.0;

        let (assign25090_e34047, assign25090_e34047_d_n0, assign25090_e34047_d_n2, assign25090_e34047_d_n6, assign25090_e34047_d_n7, assign25090_e34047_d_n10, assign25090_e34047_d_n11, assign25090_e34047_d_n12, assign25090_e34047_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 != 0.0)) {
        let assign25090_e34045: f64 = (locals.var_t0__blk774 - locals.var_vbs_bnd);
        (assign25090_e34045, locals.var_t0__blk774_dn0, locals.var_t0__blk774_dn2, locals.var_t0__blk774_dn6, locals.var_t0__blk774_dn7, locals.var_t0__blk774_dn10, locals.var_t0__blk774_dn11, locals.var_t0__blk774_dn12, locals.var_t0__blk774_dn17,)
    } else {
        (locals.var_t1__blk775, locals.var_t1__blk775_dn0, locals.var_t1__blk775_dn2, locals.var_t1__blk775_dn6, locals.var_t1__blk775_dn7, locals.var_t1__blk775_dn10, locals.var_t1__blk775_dn11, locals.var_t1__blk775_dn12, locals.var_t1__blk775_dn17,)
    }
};
        locals.var_t1__blk775 = assign25090_e34047;
        locals.var_t1__blk775_dn0 = assign25090_e34047_d_n0;
        locals.var_t1__blk775_dn2 = assign25090_e34047_d_n2;
        locals.var_t1__blk775_dn6 = assign25090_e34047_d_n6;
        locals.var_t1__blk775_dn7 = assign25090_e34047_d_n7;
        locals.var_t1__blk775_dn10 = assign25090_e34047_d_n10;
        locals.var_t1__blk775_dn11 = assign25090_e34047_d_n11;
        locals.var_t1__blk775_dn12 = assign25090_e34047_d_n12;
        locals.var_t1__blk775_dn17 = assign25090_e34047_d_n17;
        locals.var_t1__blk775_rv = 0.0;

        let (assign25100_e34059, assign25100_e34059_d_n0, assign25100_e34059_d_n2, assign25100_e34059_d_n6, assign25100_e34059_d_n7, assign25100_e34059_d_n10, assign25100_e34059_d_n11, assign25100_e34059_d_n12, assign25100_e34059_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 != 0.0)) {
        let assign25100_e34057: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign25100_e34057, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk776, locals.var_t2__blk776_dn0, locals.var_t2__blk776_dn2, locals.var_t2__blk776_dn6, locals.var_t2__blk776_dn7, locals.var_t2__blk776_dn10, locals.var_t2__blk776_dn11, locals.var_t2__blk776_dn12, locals.var_t2__blk776_dn17,)
    }
};
        locals.var_t2__blk776 = assign25100_e34059;
        locals.var_t2__blk776_dn0 = assign25100_e34059_d_n0;
        locals.var_t2__blk776_dn2 = assign25100_e34059_d_n2;
        locals.var_t2__blk776_dn6 = assign25100_e34059_d_n6;
        locals.var_t2__blk776_dn7 = assign25100_e34059_d_n7;
        locals.var_t2__blk776_dn10 = assign25100_e34059_d_n10;
        locals.var_t2__blk776_dn11 = assign25100_e34059_d_n11;
        locals.var_t2__blk776_dn12 = assign25100_e34059_d_n12;
        locals.var_t2__blk776_dn17 = assign25100_e34059_d_n17;
        locals.var_t2__blk776_rv = 0.0;

        let (assign25110_e34071, assign25110_e34071_d_n0, assign25110_e34071_d_n2, assign25110_e34071_d_n6, assign25110_e34071_d_n7, assign25110_e34071_d_n10, assign25110_e34071_d_n11, assign25110_e34071_d_n12, assign25110_e34071_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 != 0.0)) {
        let assign25110_e34069: f64 = (locals.var_t1__blk775 / locals.var_t2__blk776);
        (assign25110_e34069, (((locals.var_t1__blk775_dn0 * locals.var_t2__blk776) - (locals.var_t1__blk775 * locals.var_t2__blk776_dn0)) / (locals.var_t2__blk776 * locals.var_t2__blk776)), (((locals.var_t1__blk775_dn2 * locals.var_t2__blk776) - (locals.var_t1__blk775 * locals.var_t2__blk776_dn2)) / (locals.var_t2__blk776 * locals.var_t2__blk776)), (((locals.var_t1__blk775_dn6 * locals.var_t2__blk776) - (locals.var_t1__blk775 * locals.var_t2__blk776_dn6)) / (locals.var_t2__blk776 * locals.var_t2__blk776)), (((locals.var_t1__blk775_dn7 * locals.var_t2__blk776) - (locals.var_t1__blk775 * locals.var_t2__blk776_dn7)) / (locals.var_t2__blk776 * locals.var_t2__blk776)), (((locals.var_t1__blk775_dn10 * locals.var_t2__blk776) - (locals.var_t1__blk775 * locals.var_t2__blk776_dn10)) / (locals.var_t2__blk776 * locals.var_t2__blk776)), (((locals.var_t1__blk775_dn11 * locals.var_t2__blk776) - (locals.var_t1__blk775 * locals.var_t2__blk776_dn11)) / (locals.var_t2__blk776 * locals.var_t2__blk776)), (((locals.var_t1__blk775_dn12 * locals.var_t2__blk776) - (locals.var_t1__blk775 * locals.var_t2__blk776_dn12)) / (locals.var_t2__blk776 * locals.var_t2__blk776)), (((locals.var_t1__blk775_dn17 * locals.var_t2__blk776) - (locals.var_t1__blk775 * locals.var_t2__blk776_dn17)) / (locals.var_t2__blk776 * locals.var_t2__blk776)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign25110_e34071;
        locals.var_tmf1_dn0 = assign25110_e34071_d_n0;
        locals.var_tmf1_dn2 = assign25110_e34071_d_n2;
        locals.var_tmf1_dn6 = assign25110_e34071_d_n6;
        locals.var_tmf1_dn7 = assign25110_e34071_d_n7;
        locals.var_tmf1_dn10 = assign25110_e34071_d_n10;
        locals.var_tmf1_dn11 = assign25110_e34071_d_n11;
        locals.var_tmf1_dn12 = assign25110_e34071_d_n12;
        locals.var_tmf1_dn17 = assign25110_e34071_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign25120_e34083, assign25120_e34083_d_n0, assign25120_e34083_d_n2, assign25120_e34083_d_n6, assign25120_e34083_d_n7, assign25120_e34083_d_n10, assign25120_e34083_d_n11, assign25120_e34083_d_n12, assign25120_e34083_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 != 0.0)) {
        let assign25120_e34081: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign25120_e34081, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign25120_e34083;
        locals.var_tmf2_dn0 = assign25120_e34083_d_n0;
        locals.var_tmf2_dn2 = assign25120_e34083_d_n2;
        locals.var_tmf2_dn6 = assign25120_e34083_d_n6;
        locals.var_tmf2_dn7 = assign25120_e34083_d_n7;
        locals.var_tmf2_dn10 = assign25120_e34083_d_n10;
        locals.var_tmf2_dn11 = assign25120_e34083_d_n11;
        locals.var_tmf2_dn12 = assign25120_e34083_d_n12;
        locals.var_tmf2_dn17 = assign25120_e34083_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign25130_e34095, assign25130_e34095_d_n0, assign25130_e34095_d_n2, assign25130_e34095_d_n6, assign25130_e34095_d_n7, assign25130_e34095_d_n10, assign25130_e34095_d_n11, assign25130_e34095_d_n12, assign25130_e34095_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 != 0.0)) {
        let assign25130_e34093: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign25130_e34093, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12, locals.var_tmf3_dn17,)
    }
};
        locals.var_tmf3 = assign25130_e34095;
        locals.var_tmf3_dn0 = assign25130_e34095_d_n0;
        locals.var_tmf3_dn2 = assign25130_e34095_d_n2;
        locals.var_tmf3_dn6 = assign25130_e34095_d_n6;
        locals.var_tmf3_dn7 = assign25130_e34095_d_n7;
        locals.var_tmf3_dn10 = assign25130_e34095_d_n10;
        locals.var_tmf3_dn11 = assign25130_e34095_d_n11;
        locals.var_tmf3_dn12 = assign25130_e34095_d_n12;
        locals.var_tmf3_dn17 = assign25130_e34095_d_n17;
        locals.var_tmf3_rv = 0.0;

        let (assign25140_e34107, assign25140_e34107_d_n0, assign25140_e34107_d_n2, assign25140_e34107_d_n6, assign25140_e34107_d_n7, assign25140_e34107_d_n10, assign25140_e34107_d_n11, assign25140_e34107_d_n12, assign25140_e34107_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 != 0.0)) {
        let assign25140_e34105: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign25140_e34105, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn17)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn12, locals.var_tmf4_dn17,)
    }
};
        locals.var_tmf4 = assign25140_e34107;
        locals.var_tmf4_dn0 = assign25140_e34107_d_n0;
        locals.var_tmf4_dn2 = assign25140_e34107_d_n2;
        locals.var_tmf4_dn6 = assign25140_e34107_d_n6;
        locals.var_tmf4_dn7 = assign25140_e34107_d_n7;
        locals.var_tmf4_dn10 = assign25140_e34107_d_n10;
        locals.var_tmf4_dn11 = assign25140_e34107_d_n11;
        locals.var_tmf4_dn12 = assign25140_e34107_d_n12;
        locals.var_tmf4_dn17 = assign25140_e34107_d_n17;
        locals.var_tmf4_rv = 0.0;

        let (assign25150_e34127, assign25150_e34127_d_n0, assign25150_e34127_d_n2, assign25150_e34127_d_n6, assign25150_e34127_d_n7, assign25150_e34127_d_n10, assign25150_e34127_d_n11, assign25150_e34127_d_n12, assign25150_e34127_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 != 0.0)) {
        let assign25150_e34118: f64 = (1.0 + locals.var_tmf1);
        let assign25150_e34120: f64 = (assign25150_e34118 + locals.var_tmf2);
        let assign25150_e34122: f64 = (assign25150_e34120 + locals.var_tmf3);
        let assign25150_e34124: f64 = (assign25150_e34122 + locals.var_tmf4);
        let assign25150_e34125: f64 = (1.0 / assign25150_e34124);
        (assign25150_e34125, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign25150_e34124 * assign25150_e34124))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign25150_e34124 * assign25150_e34124))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign25150_e34124 * assign25150_e34124))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign25150_e34124 * assign25150_e34124))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign25150_e34124 * assign25150_e34124))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign25150_e34124 * assign25150_e34124))), (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign25150_e34124 * assign25150_e34124))), (-((((locals.var_tmf1_dn17 + locals.var_tmf2_dn17) + locals.var_tmf3_dn17) + locals.var_tmf4_dn17) / (assign25150_e34124 * assign25150_e34124))),)
    } else {
        (locals.var_ty__blk782, locals.var_ty__blk782_dn0, locals.var_ty__blk782_dn2, locals.var_ty__blk782_dn6, locals.var_ty__blk782_dn7, locals.var_ty__blk782_dn10, locals.var_ty__blk782_dn11, locals.var_ty__blk782_dn12, locals.var_ty__blk782_dn17,)
    }
};
        locals.var_ty__blk782 = assign25150_e34127;
        locals.var_ty__blk782_dn0 = assign25150_e34127_d_n0;
        locals.var_ty__blk782_dn2 = assign25150_e34127_d_n2;
        locals.var_ty__blk782_dn6 = assign25150_e34127_d_n6;
        locals.var_ty__blk782_dn7 = assign25150_e34127_d_n7;
        locals.var_ty__blk782_dn10 = assign25150_e34127_d_n10;
        locals.var_ty__blk782_dn11 = assign25150_e34127_d_n11;
        locals.var_ty__blk782_dn12 = assign25150_e34127_d_n12;
        locals.var_ty__blk782_dn17 = assign25150_e34127_d_n17;
        locals.var_ty__blk782_rv = 0.0;

        let (assign25170_e34168, assign25170_e34168_d_n0, assign25170_e34168_d_n2, assign25170_e34168_d_n6, assign25170_e34168_d_n7, assign25170_e34168_d_n10, assign25170_e34168_d_n11, assign25170_e34168_d_n12, assign25170_e34168_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 != 0.0)) {
        let assign25170_e34165: f64 = (1.0 - locals.var_ty__blk782);
        let assign25170_e34166: f64 = (locals.var_t2__blk776 * assign25170_e34165);
        (assign25170_e34166, ((locals.var_t2__blk776_dn0 * assign25170_e34165) + (locals.var_t2__blk776 * (-locals.var_ty__blk782_dn0))), ((locals.var_t2__blk776_dn2 * assign25170_e34165) + (locals.var_t2__blk776 * (-locals.var_ty__blk782_dn2))), ((locals.var_t2__blk776_dn6 * assign25170_e34165) + (locals.var_t2__blk776 * (-locals.var_ty__blk782_dn6))), ((locals.var_t2__blk776_dn7 * assign25170_e34165) + (locals.var_t2__blk776 * (-locals.var_ty__blk782_dn7))), ((locals.var_t2__blk776_dn10 * assign25170_e34165) + (locals.var_t2__blk776 * (-locals.var_ty__blk782_dn10))), ((locals.var_t2__blk776_dn11 * assign25170_e34165) + (locals.var_t2__blk776 * (-locals.var_ty__blk782_dn11))), ((locals.var_t2__blk776_dn12 * assign25170_e34165) + (locals.var_t2__blk776 * (-locals.var_ty__blk782_dn12))), ((locals.var_t2__blk776_dn17 * assign25170_e34165) + (locals.var_t2__blk776 * (-locals.var_ty__blk782_dn17))),)
    } else {
        (locals.var_ty__blk782, locals.var_ty__blk782_dn0, locals.var_ty__blk782_dn2, locals.var_ty__blk782_dn6, locals.var_ty__blk782_dn7, locals.var_ty__blk782_dn10, locals.var_ty__blk782_dn11, locals.var_ty__blk782_dn12, locals.var_ty__blk782_dn17,)
    }
};
        locals.var_ty__blk782 = assign25170_e34168;
        locals.var_ty__blk782_dn0 = assign25170_e34168_d_n0;
        locals.var_ty__blk782_dn2 = assign25170_e34168_d_n2;
        locals.var_ty__blk782_dn6 = assign25170_e34168_d_n6;
        locals.var_ty__blk782_dn7 = assign25170_e34168_d_n7;
        locals.var_ty__blk782_dn10 = assign25170_e34168_d_n10;
        locals.var_ty__blk782_dn11 = assign25170_e34168_d_n11;
        locals.var_ty__blk782_dn12 = assign25170_e34168_d_n12;
        locals.var_ty__blk782_dn17 = assign25170_e34168_d_n17;
        locals.var_ty__blk782_rv = 0.0;

        let (assign25190_e34191, assign25190_e34191_d_n0, assign25190_e34191_d_n2, assign25190_e34191_d_n6, assign25190_e34191_d_n7, assign25190_e34191_d_n10, assign25190_e34191_d_n11, assign25190_e34191_d_n12, assign25190_e34191_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 != 0.0)) {
        let assign25190_e34189: f64 = (locals.var_vbs_bnd + locals.var_ty__blk782);
        (assign25190_e34189, locals.var_ty__blk782_dn0, locals.var_ty__blk782_dn2, locals.var_ty__blk782_dn6, locals.var_ty__blk782_dn7, locals.var_ty__blk782_dn10, locals.var_ty__blk782_dn11, locals.var_ty__blk782_dn12, locals.var_ty__blk782_dn17,)
    } else {
        (locals.var_t10__blk779, locals.var_t10__blk779_dn0, locals.var_t10__blk779_dn2, locals.var_t10__blk779_dn6, locals.var_t10__blk779_dn7, locals.var_t10__blk779_dn10, locals.var_t10__blk779_dn11, locals.var_t10__blk779_dn12, locals.var_t10__blk779_dn17,)
    }
};
        locals.var_t10__blk779 = assign25190_e34191;
        locals.var_t10__blk779_dn0 = assign25190_e34191_d_n0;
        locals.var_t10__blk779_dn2 = assign25190_e34191_d_n2;
        locals.var_t10__blk779_dn6 = assign25190_e34191_d_n6;
        locals.var_t10__blk779_dn7 = assign25190_e34191_d_n7;
        locals.var_t10__blk779_dn10 = assign25190_e34191_d_n10;
        locals.var_t10__blk779_dn11 = assign25190_e34191_d_n11;
        locals.var_t10__blk779_dn12 = assign25190_e34191_d_n12;
        locals.var_t10__blk779_dn17 = assign25190_e34191_d_n17;
        locals.var_t10__blk779_rv = 0.0;

        let (assign25200_e34202, assign25200_e34202_d_n0, assign25200_e34202_d_n2, assign25200_e34202_d_n6, assign25200_e34202_d_n7, assign25200_e34202_d_n10, assign25200_e34202_d_n11, assign25200_e34202_d_n12, assign25200_e34202_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 == 0.0)) {
        (locals.var_t0__blk774, locals.var_t0__blk774_dn0, locals.var_t0__blk774_dn2, locals.var_t0__blk774_dn6, locals.var_t0__blk774_dn7, locals.var_t0__blk774_dn10, locals.var_t0__blk774_dn11, locals.var_t0__blk774_dn12, locals.var_t0__blk774_dn17,)
    } else {
        (locals.var_t10__blk779, locals.var_t10__blk779_dn0, locals.var_t10__blk779_dn2, locals.var_t10__blk779_dn6, locals.var_t10__blk779_dn7, locals.var_t10__blk779_dn10, locals.var_t10__blk779_dn11, locals.var_t10__blk779_dn12, locals.var_t10__blk779_dn17,)
    }
};
        locals.var_t10__blk779 = assign25200_e34202;
        locals.var_t10__blk779_dn0 = assign25200_e34202_d_n0;
        locals.var_t10__blk779_dn2 = assign25200_e34202_d_n2;
        locals.var_t10__blk779_dn6 = assign25200_e34202_d_n6;
        locals.var_t10__blk779_dn7 = assign25200_e34202_d_n7;
        locals.var_t10__blk779_dn10 = assign25200_e34202_d_n10;
        locals.var_t10__blk779_dn11 = assign25200_e34202_d_n11;
        locals.var_t10__blk779_dn12 = assign25200_e34202_d_n12;
        locals.var_t10__blk779_dn17 = assign25200_e34202_d_n17;
        locals.var_t10__blk779_rv = 0.0;

        let (assign25220_e34224, assign25220_e34224_d_n0, assign25220_e34224_d_n2, assign25220_e34224_d_n6, assign25220_e34224_d_n7, assign25220_e34224_d_n10, assign25220_e34224_d_n11, assign25220_e34224_d_n12, assign25220_e34224_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25220_e34220: f64 = (-locals.var_t10__blk779);
        let assign25220_e34222: f64 = (assign25220_e34220 - 1e-12);
        (assign25220_e34222, (-locals.var_t10__blk779_dn0), (-locals.var_t10__blk779_dn2), (-locals.var_t10__blk779_dn6), (-locals.var_t10__blk779_dn7), (-locals.var_t10__blk779_dn10), (-locals.var_t10__blk779_dn11), (-locals.var_t10__blk779_dn12), (-locals.var_t10__blk779_dn17),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn12, locals.var_vxbgmtcl_dn17,)
    }
};
        locals.var_vxbgmtcl = assign25220_e34224;
        locals.var_vxbgmtcl_dn0 = assign25220_e34224_d_n0;
        locals.var_vxbgmtcl_dn2 = assign25220_e34224_d_n2;
        locals.var_vxbgmtcl_dn6 = assign25220_e34224_d_n6;
        locals.var_vxbgmtcl_dn7 = assign25220_e34224_d_n7;
        locals.var_vxbgmtcl_dn10 = assign25220_e34224_d_n10;
        locals.var_vxbgmtcl_dn11 = assign25220_e34224_d_n11;
        locals.var_vxbgmtcl_dn12 = assign25220_e34224_d_n12;
        locals.var_vxbgmtcl_dn17 = assign25220_e34224_d_n17;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign25230_e34234, assign25230_e34234_d_n0, assign25230_e34234_d_n2, assign25230_e34234_d_n6, assign25230_e34234_d_n7, assign25230_e34234_d_n10, assign25230_e34234_d_n11, assign25230_e34234_d_n12, assign25230_e34234_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25230_e34232: f64 = (locals.var_cnst0over * locals.var_cox0_inv);
        (assign25230_e34232, (locals.var_cnst0over_dn0 * locals.var_cox0_inv), (locals.var_cnst0over_dn2 * locals.var_cox0_inv), (locals.var_cnst0over_dn6 * locals.var_cox0_inv), (locals.var_cnst0over_dn7 * locals.var_cox0_inv), (locals.var_cnst0over_dn10 * locals.var_cox0_inv), (locals.var_cnst0over_dn11 * locals.var_cox0_inv), (locals.var_cnst0over_dn12 * locals.var_cox0_inv), (locals.var_cnst0over_dn17 * locals.var_cox0_inv),)
    } else {
        (locals.var_fac1__blk804, locals.var_fac1__blk804_dn0, locals.var_fac1__blk804_dn2, locals.var_fac1__blk804_dn6, locals.var_fac1__blk804_dn7, locals.var_fac1__blk804_dn10, locals.var_fac1__blk804_dn11, locals.var_fac1__blk804_dn12, locals.var_fac1__blk804_dn17,)
    }
};
        locals.var_fac1__blk804 = assign25230_e34234;
        locals.var_fac1__blk804_dn0 = assign25230_e34234_d_n0;
        locals.var_fac1__blk804_dn2 = assign25230_e34234_d_n2;
        locals.var_fac1__blk804_dn6 = assign25230_e34234_d_n6;
        locals.var_fac1__blk804_dn7 = assign25230_e34234_d_n7;
        locals.var_fac1__blk804_dn10 = assign25230_e34234_d_n10;
        locals.var_fac1__blk804_dn11 = assign25230_e34234_d_n11;
        locals.var_fac1__blk804_dn12 = assign25230_e34234_d_n12;
        locals.var_fac1__blk804_dn17 = assign25230_e34234_d_n17;
        locals.var_fac1__blk804_rv = 0.0;

        let (assign25240_e34244, assign25240_e34244_d_n0, assign25240_e34244_d_n2, assign25240_e34244_d_n6, assign25240_e34244_d_n7, assign25240_e34244_d_n10, assign25240_e34244_d_n11, assign25240_e34244_d_n12, assign25240_e34244_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25240_e34242: f64 = (locals.var_fac1__blk804 * locals.var_fac1__blk804);
        (assign25240_e34242, ((locals.var_fac1__blk804_dn0 * locals.var_fac1__blk804) + (locals.var_fac1__blk804 * locals.var_fac1__blk804_dn0)), ((locals.var_fac1__blk804_dn2 * locals.var_fac1__blk804) + (locals.var_fac1__blk804 * locals.var_fac1__blk804_dn2)), ((locals.var_fac1__blk804_dn6 * locals.var_fac1__blk804) + (locals.var_fac1__blk804 * locals.var_fac1__blk804_dn6)), ((locals.var_fac1__blk804_dn7 * locals.var_fac1__blk804) + (locals.var_fac1__blk804 * locals.var_fac1__blk804_dn7)), ((locals.var_fac1__blk804_dn10 * locals.var_fac1__blk804) + (locals.var_fac1__blk804 * locals.var_fac1__blk804_dn10)), ((locals.var_fac1__blk804_dn11 * locals.var_fac1__blk804) + (locals.var_fac1__blk804 * locals.var_fac1__blk804_dn11)), ((locals.var_fac1__blk804_dn12 * locals.var_fac1__blk804) + (locals.var_fac1__blk804 * locals.var_fac1__blk804_dn12)), ((locals.var_fac1__blk804_dn17 * locals.var_fac1__blk804) + (locals.var_fac1__blk804 * locals.var_fac1__blk804_dn17)),)
    } else {
        (locals.var_fac1p2__blk805, locals.var_fac1p2__blk805_dn0, locals.var_fac1p2__blk805_dn2, locals.var_fac1p2__blk805_dn6, locals.var_fac1p2__blk805_dn7, locals.var_fac1p2__blk805_dn10, locals.var_fac1p2__blk805_dn11, locals.var_fac1p2__blk805_dn12, locals.var_fac1p2__blk805_dn17,)
    }
};
        locals.var_fac1p2__blk805 = assign25240_e34244;
        locals.var_fac1p2__blk805_dn0 = assign25240_e34244_d_n0;
        locals.var_fac1p2__blk805_dn2 = assign25240_e34244_d_n2;
        locals.var_fac1p2__blk805_dn6 = assign25240_e34244_d_n6;
        locals.var_fac1p2__blk805_dn7 = assign25240_e34244_d_n7;
        locals.var_fac1p2__blk805_dn10 = assign25240_e34244_d_n10;
        locals.var_fac1p2__blk805_dn11 = assign25240_e34244_d_n11;
        locals.var_fac1p2__blk805_dn12 = assign25240_e34244_d_n12;
        locals.var_fac1p2__blk805_dn17 = assign25240_e34244_d_n17;
        locals.var_fac1p2__blk805_rv = 0.0;

        let (assign25250_e34254, assign25250_e34254_d_n0, assign25250_e34254_d_n2, assign25250_e34254_d_n6, assign25250_e34254_d_n7, assign25250_e34254_d_n10, assign25250_e34254_d_n11, assign25250_e34254_d_n12, assign25250_e34254_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25250_e34252: f64 = (locals.var_vgbgmt - locals.var_uc_vfbbt);
        (assign25250_e34252, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn10, locals.var_vgbgmt_dn11, locals.var_vgbgmt_dn12, locals.var_vgbgmt_dn17,)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn0, locals.var_vgpld_dn2, locals.var_vgpld_dn6, locals.var_vgpld_dn7, locals.var_vgpld_dn10, locals.var_vgpld_dn11, locals.var_vgpld_dn12, locals.var_vgpld_dn17,)
    }
};
        locals.var_vgpld = assign25250_e34254;
        locals.var_vgpld_dn0 = assign25250_e34254_d_n0;
        locals.var_vgpld_dn2 = assign25250_e34254_d_n2;
        locals.var_vgpld_dn6 = assign25250_e34254_d_n6;
        locals.var_vgpld_dn7 = assign25250_e34254_d_n7;
        locals.var_vgpld_dn10 = assign25250_e34254_d_n10;
        locals.var_vgpld_dn11 = assign25250_e34254_d_n11;
        locals.var_vgpld_dn12 = assign25250_e34254_d_n12;
        locals.var_vgpld_dn17 = assign25250_e34254_d_n17;
        locals.var_vgpld_rv = 0.0;

        let (assign25260_e34264, assign25260_e34264_d_n0, assign25260_e34264_d_n2, assign25260_e34264_d_n6, assign25260_e34264_d_n7, assign25260_e34264_d_n10, assign25260_e34264_d_n11, assign25260_e34264_d_n12, assign25260_e34264_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25260_e34262: f64 = (locals.var_uc_nsubbttub / locals.var_nin);
        (assign25260_e34262, (((locals.var_uc_nsubbttub_dn0 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn2 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn6 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn7 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn10 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn11 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn12 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn12)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn17 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn17)) / (locals.var_nin * locals.var_nin)),)
    } else {
        (locals.var_t0__blk774, locals.var_t0__blk774_dn0, locals.var_t0__blk774_dn2, locals.var_t0__blk774_dn6, locals.var_t0__blk774_dn7, locals.var_t0__blk774_dn10, locals.var_t0__blk774_dn11, locals.var_t0__blk774_dn12, locals.var_t0__blk774_dn17,)
    }
};
        locals.var_t0__blk774 = assign25260_e34264;
        locals.var_t0__blk774_dn0 = assign25260_e34264_d_n0;
        locals.var_t0__blk774_dn2 = assign25260_e34264_d_n2;
        locals.var_t0__blk774_dn6 = assign25260_e34264_d_n6;
        locals.var_t0__blk774_dn7 = assign25260_e34264_d_n7;
        locals.var_t0__blk774_dn10 = assign25260_e34264_d_n10;
        locals.var_t0__blk774_dn11 = assign25260_e34264_d_n11;
        locals.var_t0__blk774_dn12 = assign25260_e34264_d_n12;
        locals.var_t0__blk774_dn17 = assign25260_e34264_d_n17;
        locals.var_t0__blk774_rv = 0.0;

        let (assign25270_e34277, assign25270_e34277_d_n0, assign25270_e34277_d_n2, assign25270_e34277_d_n6, assign25270_e34277_d_n7, assign25270_e34277_d_n10, assign25270_e34277_d_n11, assign25270_e34277_d_n12, assign25270_e34277_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25270_e34272: f64 = (2.0 / locals.var_beta);
        let assign25270_e34274: f64 = (locals.var_t0__blk774).ln();
        let assign25270_e34275: f64 = (assign25270_e34272 * assign25270_e34274);
        (assign25270_e34275, (assign25270_e34272 * (locals.var_t0__blk774_dn0 / locals.var_t0__blk774)), (assign25270_e34272 * (locals.var_t0__blk774_dn2 / locals.var_t0__blk774)), (assign25270_e34272 * (locals.var_t0__blk774_dn6 / locals.var_t0__blk774)), (assign25270_e34272 * (locals.var_t0__blk774_dn7 / locals.var_t0__blk774)), (((-((2.0 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign25270_e34274) + (assign25270_e34272 * (locals.var_t0__blk774_dn10 / locals.var_t0__blk774))), (assign25270_e34272 * (locals.var_t0__blk774_dn11 / locals.var_t0__blk774)), (assign25270_e34272 * (locals.var_t0__blk774_dn12 / locals.var_t0__blk774)), (assign25270_e34272 * (locals.var_t0__blk774_dn17 / locals.var_t0__blk774)),)
    } else {
        (locals.var_pb2over, locals.var_pb2over_dn0, locals.var_pb2over_dn2, locals.var_pb2over_dn6, locals.var_pb2over_dn7, locals.var_pb2over_dn10, locals.var_pb2over_dn11, locals.var_pb2over_dn12, locals.var_pb2over_dn17,)
    }
};
        locals.var_pb2over = assign25270_e34277;
        locals.var_pb2over_dn0 = assign25270_e34277_d_n0;
        locals.var_pb2over_dn2 = assign25270_e34277_d_n2;
        locals.var_pb2over_dn6 = assign25270_e34277_d_n6;
        locals.var_pb2over_dn7 = assign25270_e34277_d_n7;
        locals.var_pb2over_dn10 = assign25270_e34277_d_n10;
        locals.var_pb2over_dn11 = assign25270_e34277_d_n11;
        locals.var_pb2over_dn12 = assign25270_e34277_d_n12;
        locals.var_pb2over_dn17 = assign25270_e34277_d_n17;
        locals.var_pb2over_rv = 0.0;

        let (assign25280_e34286, assign25280_e34286_d_n0, assign25280_e34286_d_n2, assign25280_e34286_d_n6, assign25280_e34286_d_n7, assign25280_e34286_d_n10, assign25280_e34286_d_n11, assign25280_e34286_d_n12, assign25280_e34286_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign25280_e34284: f64 = (-locals.var_vxbgmtcl);
        (assign25280_e34284, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn12), (-locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn12, locals.var_vgb_fb_ld_dn17,)
    }
};
        locals.var_vgb_fb_ld = assign25280_e34286;
        locals.var_vgb_fb_ld_dn0 = assign25280_e34286_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign25280_e34286_d_n2;
        locals.var_vgb_fb_ld_dn6 = assign25280_e34286_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign25280_e34286_d_n7;
        locals.var_vgb_fb_ld_dn10 = assign25280_e34286_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign25280_e34286_d_n11;
        locals.var_vgb_fb_ld_dn12 = assign25280_e34286_d_n12;
        locals.var_vgb_fb_ld_dn17 = assign25280_e34286_d_n17;
        locals.var_vgb_fb_ld_rv = 0.0;

        let assign25290_e34289: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard857 = assign25290_e34289;
        locals.var_guard857_rv = 0.0;

        let (assign25310_e34314, assign25310_e34314_d_n0, assign25310_e34314_d_n2, assign25310_e34314_d_n6, assign25310_e34314_d_n7, assign25310_e34314_d_n10, assign25310_e34314_d_n11, assign25310_e34314_d_n12, assign25310_e34314_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25310_e34311: f64 = (locals.var_beta * locals.var_cnst0over);
        let assign25310_e34312: f64 = (1.0 / assign25310_e34311);
        (assign25310_e34312, (-((locals.var_beta * locals.var_cnst0over_dn0) / (assign25310_e34311 * assign25310_e34311))), (-((locals.var_beta * locals.var_cnst0over_dn2) / (assign25310_e34311 * assign25310_e34311))), (-((locals.var_beta * locals.var_cnst0over_dn6) / (assign25310_e34311 * assign25310_e34311))), (-((locals.var_beta * locals.var_cnst0over_dn7) / (assign25310_e34311 * assign25310_e34311))), (-(((locals.var_beta_dn10 * locals.var_cnst0over) + (locals.var_beta * locals.var_cnst0over_dn10)) / (assign25310_e34311 * assign25310_e34311))), (-((locals.var_beta * locals.var_cnst0over_dn11) / (assign25310_e34311 * assign25310_e34311))), (-((locals.var_beta * locals.var_cnst0over_dn12) / (assign25310_e34311 * assign25310_e34311))), (-((locals.var_beta * locals.var_cnst0over_dn17) / (assign25310_e34311 * assign25310_e34311))),)
    } else {
        (locals.var_t1__blk775, locals.var_t1__blk775_dn0, locals.var_t1__blk775_dn2, locals.var_t1__blk775_dn6, locals.var_t1__blk775_dn7, locals.var_t1__blk775_dn10, locals.var_t1__blk775_dn11, locals.var_t1__blk775_dn12, locals.var_t1__blk775_dn17,)
    }
};
        locals.var_t1__blk775 = assign25310_e34314;
        locals.var_t1__blk775_dn0 = assign25310_e34314_d_n0;
        locals.var_t1__blk775_dn2 = assign25310_e34314_d_n2;
        locals.var_t1__blk775_dn6 = assign25310_e34314_d_n6;
        locals.var_t1__blk775_dn7 = assign25310_e34314_d_n7;
        locals.var_t1__blk775_dn10 = assign25310_e34314_d_n10;
        locals.var_t1__blk775_dn11 = assign25310_e34314_d_n11;
        locals.var_t1__blk775_dn12 = assign25310_e34314_d_n12;
        locals.var_t1__blk775_dn17 = assign25310_e34314_d_n17;
        locals.var_t1__blk775_rv = 0.0;

        let (assign25320_e34326, assign25320_e34326_d_n0, assign25320_e34326_d_n2, assign25320_e34326_d_n6, assign25320_e34326_d_n7, assign25320_e34326_d_n10, assign25320_e34326_d_n11, assign25320_e34326_d_n12, assign25320_e34326_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25320_e34324: f64 = (locals.var_t1__blk775 * locals.var_cox0);
        (assign25320_e34324, (locals.var_t1__blk775_dn0 * locals.var_cox0), (locals.var_t1__blk775_dn2 * locals.var_cox0), (locals.var_t1__blk775_dn6 * locals.var_cox0), (locals.var_t1__blk775_dn7 * locals.var_cox0), (locals.var_t1__blk775_dn10 * locals.var_cox0), (locals.var_t1__blk775_dn11 * locals.var_cox0), (locals.var_t1__blk775_dn12 * locals.var_cox0), (locals.var_t1__blk775_dn17 * locals.var_cox0),)
    } else {
        (locals.var_ty__blk782, locals.var_ty__blk782_dn0, locals.var_ty__blk782_dn2, locals.var_ty__blk782_dn6, locals.var_ty__blk782_dn7, locals.var_ty__blk782_dn10, locals.var_ty__blk782_dn11, locals.var_ty__blk782_dn12, locals.var_ty__blk782_dn17,)
    }
};
        locals.var_ty__blk782 = assign25320_e34326;
        locals.var_ty__blk782_dn0 = assign25320_e34326_d_n0;
        locals.var_ty__blk782_dn2 = assign25320_e34326_d_n2;
        locals.var_ty__blk782_dn6 = assign25320_e34326_d_n6;
        locals.var_ty__blk782_dn7 = assign25320_e34326_d_n7;
        locals.var_ty__blk782_dn10 = assign25320_e34326_d_n10;
        locals.var_ty__blk782_dn11 = assign25320_e34326_d_n11;
        locals.var_ty__blk782_dn12 = assign25320_e34326_d_n12;
        locals.var_ty__blk782_dn17 = assign25320_e34326_d_n17;
        locals.var_ty__blk782_rv = 0.0;

        let (assign25330_e34342, assign25330_e34342_d_n0, assign25330_e34342_d_n2, assign25330_e34342_d_n6, assign25330_e34342_d_n7, assign25330_e34342_d_n10, assign25330_e34342_d_n11, assign25330_e34342_d_n12, assign25330_e34342_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25330_e34337: f64 = (3.0 * 1.414213562373095);
        let assign25330_e34339: f64 = (assign25330_e34337 * locals.var_ty__blk782);
        let assign25330_e34340: f64 = (2.0 + assign25330_e34339);
        (assign25330_e34340, (assign25330_e34337 * locals.var_ty__blk782_dn0), (assign25330_e34337 * locals.var_ty__blk782_dn2), (assign25330_e34337 * locals.var_ty__blk782_dn6), (assign25330_e34337 * locals.var_ty__blk782_dn7), (assign25330_e34337 * locals.var_ty__blk782_dn10), (assign25330_e34337 * locals.var_ty__blk782_dn11), (assign25330_e34337 * locals.var_ty__blk782_dn12), (assign25330_e34337 * locals.var_ty__blk782_dn17),)
    } else {
        (locals.var_ac41__blk809, locals.var_ac41__blk809_dn0, locals.var_ac41__blk809_dn2, locals.var_ac41__blk809_dn6, locals.var_ac41__blk809_dn7, locals.var_ac41__blk809_dn10, locals.var_ac41__blk809_dn11, locals.var_ac41__blk809_dn12, locals.var_ac41__blk809_dn17,)
    }
};
        locals.var_ac41__blk809 = assign25330_e34342;
        locals.var_ac41__blk809_dn0 = assign25330_e34342_d_n0;
        locals.var_ac41__blk809_dn2 = assign25330_e34342_d_n2;
        locals.var_ac41__blk809_dn6 = assign25330_e34342_d_n6;
        locals.var_ac41__blk809_dn7 = assign25330_e34342_d_n7;
        locals.var_ac41__blk809_dn10 = assign25330_e34342_d_n10;
        locals.var_ac41__blk809_dn11 = assign25330_e34342_d_n11;
        locals.var_ac41__blk809_dn12 = assign25330_e34342_d_n12;
        locals.var_ac41__blk809_dn17 = assign25330_e34342_d_n17;
        locals.var_ac41__blk809_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_89(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25340_e34358, assign25340_e34358_d_n0, assign25340_e34358_d_n2, assign25340_e34358_d_n6, assign25340_e34358_d_n7, assign25340_e34358_d_n10, assign25340_e34358_d_n11, assign25340_e34358_d_n12, assign25340_e34358_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25340_e34352: f64 = (8.0 * locals.var_ac41__blk809);
        let assign25340_e34354: f64 = (assign25340_e34352 * locals.var_ac41__blk809);
        let assign25340_e34356: f64 = (assign25340_e34354 * locals.var_ac41__blk809);
        (assign25340_e34356, (((((8.0 * locals.var_ac41__blk809_dn0) * locals.var_ac41__blk809) + (assign25340_e34352 * locals.var_ac41__blk809_dn0)) * locals.var_ac41__blk809) + (assign25340_e34354 * locals.var_ac41__blk809_dn0)), (((((8.0 * locals.var_ac41__blk809_dn2) * locals.var_ac41__blk809) + (assign25340_e34352 * locals.var_ac41__blk809_dn2)) * locals.var_ac41__blk809) + (assign25340_e34354 * locals.var_ac41__blk809_dn2)), (((((8.0 * locals.var_ac41__blk809_dn6) * locals.var_ac41__blk809) + (assign25340_e34352 * locals.var_ac41__blk809_dn6)) * locals.var_ac41__blk809) + (assign25340_e34354 * locals.var_ac41__blk809_dn6)), (((((8.0 * locals.var_ac41__blk809_dn7) * locals.var_ac41__blk809) + (assign25340_e34352 * locals.var_ac41__blk809_dn7)) * locals.var_ac41__blk809) + (assign25340_e34354 * locals.var_ac41__blk809_dn7)), (((((8.0 * locals.var_ac41__blk809_dn10) * locals.var_ac41__blk809) + (assign25340_e34352 * locals.var_ac41__blk809_dn10)) * locals.var_ac41__blk809) + (assign25340_e34354 * locals.var_ac41__blk809_dn10)), (((((8.0 * locals.var_ac41__blk809_dn11) * locals.var_ac41__blk809) + (assign25340_e34352 * locals.var_ac41__blk809_dn11)) * locals.var_ac41__blk809) + (assign25340_e34354 * locals.var_ac41__blk809_dn11)), (((((8.0 * locals.var_ac41__blk809_dn12) * locals.var_ac41__blk809) + (assign25340_e34352 * locals.var_ac41__blk809_dn12)) * locals.var_ac41__blk809) + (assign25340_e34354 * locals.var_ac41__blk809_dn12)), (((((8.0 * locals.var_ac41__blk809_dn17) * locals.var_ac41__blk809) + (assign25340_e34352 * locals.var_ac41__blk809_dn17)) * locals.var_ac41__blk809) + (assign25340_e34354 * locals.var_ac41__blk809_dn17)),)
    } else {
        (locals.var_ac4__blk810, locals.var_ac4__blk810_dn0, locals.var_ac4__blk810_dn2, locals.var_ac4__blk810_dn6, locals.var_ac4__blk810_dn7, locals.var_ac4__blk810_dn10, locals.var_ac4__blk810_dn11, locals.var_ac4__blk810_dn12, locals.var_ac4__blk810_dn17,)
    }
};
        locals.var_ac4__blk810 = assign25340_e34358;
        locals.var_ac4__blk810_dn0 = assign25340_e34358_d_n0;
        locals.var_ac4__blk810_dn2 = assign25340_e34358_d_n2;
        locals.var_ac4__blk810_dn6 = assign25340_e34358_d_n6;
        locals.var_ac4__blk810_dn7 = assign25340_e34358_d_n7;
        locals.var_ac4__blk810_dn10 = assign25340_e34358_d_n10;
        locals.var_ac4__blk810_dn11 = assign25340_e34358_d_n11;
        locals.var_ac4__blk810_dn12 = assign25340_e34358_d_n12;
        locals.var_ac4__blk810_dn17 = assign25340_e34358_d_n17;
        locals.var_ac4__blk810_rv = 0.0;

        let (assign25350_e34370, assign25350_e34370_d_n0, assign25350_e34370_d_n2, assign25350_e34370_d_n6, assign25350_e34370_d_n7, assign25350_e34370_d_n10, assign25350_e34370_d_n11, assign25350_e34370_d_n12, assign25350_e34370_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25350_e34368: f64 = (locals.var_eg - locals.var_pb2over);
        (assign25350_e34368, (locals.var_eg_dn0 - locals.var_pb2over_dn0), (locals.var_eg_dn2 - locals.var_pb2over_dn2), (locals.var_eg_dn6 - locals.var_pb2over_dn6), (locals.var_eg_dn7 - locals.var_pb2over_dn7), (locals.var_eg_dn10 - locals.var_pb2over_dn10), (locals.var_eg_dn11 - locals.var_pb2over_dn11), (locals.var_eg_dn12 - locals.var_pb2over_dn12), (locals.var_eg_dn17 - locals.var_pb2over_dn17),)
    } else {
        (locals.var_ps0_min__blk811, locals.var_ps0_min__blk811_dn0, locals.var_ps0_min__blk811_dn2, locals.var_ps0_min__blk811_dn6, locals.var_ps0_min__blk811_dn7, locals.var_ps0_min__blk811_dn10, locals.var_ps0_min__blk811_dn11, locals.var_ps0_min__blk811_dn12, locals.var_ps0_min__blk811_dn17,)
    }
};
        locals.var_ps0_min__blk811 = assign25350_e34370;
        locals.var_ps0_min__blk811_dn0 = assign25350_e34370_d_n0;
        locals.var_ps0_min__blk811_dn2 = assign25350_e34370_d_n2;
        locals.var_ps0_min__blk811_dn6 = assign25350_e34370_d_n6;
        locals.var_ps0_min__blk811_dn7 = assign25350_e34370_d_n7;
        locals.var_ps0_min__blk811_dn10 = assign25350_e34370_d_n10;
        locals.var_ps0_min__blk811_dn11 = assign25350_e34370_d_n11;
        locals.var_ps0_min__blk811_dn12 = assign25350_e34370_d_n12;
        locals.var_ps0_min__blk811_dn17 = assign25350_e34370_d_n17;
        locals.var_ps0_min__blk811_rv = 0.0;

        let (assign25360_e34384, assign25360_e34384_d_n0, assign25360_e34384_d_n2, assign25360_e34384_d_n6, assign25360_e34384_d_n7, assign25360_e34384_d_n10, assign25360_e34384_d_n11, assign25360_e34384_d_n12, assign25360_e34384_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25360_e34381: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign25360_e34382: f64 = (locals.var_beta * assign25360_e34381);
        (assign25360_e34382, (locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign25360_e34381) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_tx__blk781, locals.var_tx__blk781_dn0, locals.var_tx__blk781_dn2, locals.var_tx__blk781_dn6, locals.var_tx__blk781_dn7, locals.var_tx__blk781_dn10, locals.var_tx__blk781_dn11, locals.var_tx__blk781_dn12, locals.var_tx__blk781_dn17,)
    }
};
        locals.var_tx__blk781 = assign25360_e34384;
        locals.var_tx__blk781_dn0 = assign25360_e34384_d_n0;
        locals.var_tx__blk781_dn2 = assign25360_e34384_d_n2;
        locals.var_tx__blk781_dn6 = assign25360_e34384_d_n6;
        locals.var_tx__blk781_dn7 = assign25360_e34384_d_n7;
        locals.var_tx__blk781_dn10 = assign25360_e34384_d_n10;
        locals.var_tx__blk781_dn11 = assign25360_e34384_d_n11;
        locals.var_tx__blk781_dn12 = assign25360_e34384_d_n12;
        locals.var_tx__blk781_dn17 = assign25360_e34384_d_n17;
        locals.var_tx__blk781_rv = 0.0;

        let (assign25370_e34404, assign25370_e34404_d_n0, assign25370_e34404_d_n2, assign25370_e34404_d_n6, assign25370_e34404_d_n7, assign25370_e34404_d_n10, assign25370_e34404_d_n11, assign25370_e34404_d_n12, assign25370_e34404_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25370_e34394: f64 = (7.0 * 1.414213562373095);
        let assign25370_e34397: f64 = (9.0 * locals.var_ty__blk782);
        let assign25370_e34400: f64 = (locals.var_tx__blk781 - 2.0);
        let assign25370_e34401: f64 = (assign25370_e34397 * assign25370_e34400);
        let assign25370_e34402: f64 = (assign25370_e34394 - assign25370_e34401);
        (assign25370_e34402, (-(((9.0 * locals.var_ty__blk782_dn0) * assign25370_e34400) + (assign25370_e34397 * locals.var_tx__blk781_dn0))), (-(((9.0 * locals.var_ty__blk782_dn2) * assign25370_e34400) + (assign25370_e34397 * locals.var_tx__blk781_dn2))), (-(((9.0 * locals.var_ty__blk782_dn6) * assign25370_e34400) + (assign25370_e34397 * locals.var_tx__blk781_dn6))), (-(((9.0 * locals.var_ty__blk782_dn7) * assign25370_e34400) + (assign25370_e34397 * locals.var_tx__blk781_dn7))), (-(((9.0 * locals.var_ty__blk782_dn10) * assign25370_e34400) + (assign25370_e34397 * locals.var_tx__blk781_dn10))), (-(((9.0 * locals.var_ty__blk782_dn11) * assign25370_e34400) + (assign25370_e34397 * locals.var_tx__blk781_dn11))), (-(((9.0 * locals.var_ty__blk782_dn12) * assign25370_e34400) + (assign25370_e34397 * locals.var_tx__blk781_dn12))), (-(((9.0 * locals.var_ty__blk782_dn17) * assign25370_e34400) + (assign25370_e34397 * locals.var_tx__blk781_dn17))),)
    } else {
        (locals.var_ac31__blk812, locals.var_ac31__blk812_dn0, locals.var_ac31__blk812_dn2, locals.var_ac31__blk812_dn6, locals.var_ac31__blk812_dn7, locals.var_ac31__blk812_dn10, locals.var_ac31__blk812_dn11, locals.var_ac31__blk812_dn12, locals.var_ac31__blk812_dn17,)
    }
};
        locals.var_ac31__blk812 = assign25370_e34404;
        locals.var_ac31__blk812_dn0 = assign25370_e34404_d_n0;
        locals.var_ac31__blk812_dn2 = assign25370_e34404_d_n2;
        locals.var_ac31__blk812_dn6 = assign25370_e34404_d_n6;
        locals.var_ac31__blk812_dn7 = assign25370_e34404_d_n7;
        locals.var_ac31__blk812_dn10 = assign25370_e34404_d_n10;
        locals.var_ac31__blk812_dn11 = assign25370_e34404_d_n11;
        locals.var_ac31__blk812_dn12 = assign25370_e34404_d_n12;
        locals.var_ac31__blk812_dn17 = assign25370_e34404_d_n17;
        locals.var_ac31__blk812_rv = 0.0;

        let (assign25380_e34416, assign25380_e34416_d_n0, assign25380_e34416_d_n2, assign25380_e34416_d_n6, assign25380_e34416_d_n7, assign25380_e34416_d_n10, assign25380_e34416_d_n11, assign25380_e34416_d_n12, assign25380_e34416_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25380_e34414: f64 = (locals.var_ac31__blk812 * locals.var_ac31__blk812);
        (assign25380_e34414, ((locals.var_ac31__blk812_dn0 * locals.var_ac31__blk812) + (locals.var_ac31__blk812 * locals.var_ac31__blk812_dn0)), ((locals.var_ac31__blk812_dn2 * locals.var_ac31__blk812) + (locals.var_ac31__blk812 * locals.var_ac31__blk812_dn2)), ((locals.var_ac31__blk812_dn6 * locals.var_ac31__blk812) + (locals.var_ac31__blk812 * locals.var_ac31__blk812_dn6)), ((locals.var_ac31__blk812_dn7 * locals.var_ac31__blk812) + (locals.var_ac31__blk812 * locals.var_ac31__blk812_dn7)), ((locals.var_ac31__blk812_dn10 * locals.var_ac31__blk812) + (locals.var_ac31__blk812 * locals.var_ac31__blk812_dn10)), ((locals.var_ac31__blk812_dn11 * locals.var_ac31__blk812) + (locals.var_ac31__blk812 * locals.var_ac31__blk812_dn11)), ((locals.var_ac31__blk812_dn12 * locals.var_ac31__blk812) + (locals.var_ac31__blk812 * locals.var_ac31__blk812_dn12)), ((locals.var_ac31__blk812_dn17 * locals.var_ac31__blk812) + (locals.var_ac31__blk812 * locals.var_ac31__blk812_dn17)),)
    } else {
        (locals.var_ac3__blk813, locals.var_ac3__blk813_dn0, locals.var_ac3__blk813_dn2, locals.var_ac3__blk813_dn6, locals.var_ac3__blk813_dn7, locals.var_ac3__blk813_dn10, locals.var_ac3__blk813_dn11, locals.var_ac3__blk813_dn12, locals.var_ac3__blk813_dn17,)
    }
};
        locals.var_ac3__blk813 = assign25380_e34416;
        locals.var_ac3__blk813_dn0 = assign25380_e34416_d_n0;
        locals.var_ac3__blk813_dn2 = assign25380_e34416_d_n2;
        locals.var_ac3__blk813_dn6 = assign25380_e34416_d_n6;
        locals.var_ac3__blk813_dn7 = assign25380_e34416_d_n7;
        locals.var_ac3__blk813_dn10 = assign25380_e34416_d_n10;
        locals.var_ac3__blk813_dn11 = assign25380_e34416_d_n11;
        locals.var_ac3__blk813_dn12 = assign25380_e34416_d_n12;
        locals.var_ac3__blk813_dn17 = assign25380_e34416_d_n17;
        locals.var_ac3__blk813_rv = 0.0;

        let assign25390_e34420: f64 = (locals.var_ac3__blk813 * 1e-8);
        let assign25390_e34421: f64 = if locals.var_ac4__blk810 < assign25390_e34420 { 1.0 } else { 0.0 };
        locals.var_guard858 = assign25390_e34421;
        locals.var_guard858_rv = 0.0;

        let (assign25400_e34452, assign25400_e34452_d_n0, assign25400_e34452_d_n2, assign25400_e34452_d_n6, assign25400_e34452_d_n7, assign25400_e34452_d_n10, assign25400_e34452_d_n11, assign25400_e34452_d_n12, assign25400_e34452_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) && (locals.var_guard858 != 0.0)) {
        let assign25400_e34432: f64 = (-7.0);
        let assign25400_e34434: f64 = (assign25400_e34432 * 1.414213562373095);
        let assign25400_e34436: f64 = (assign25400_e34434 + locals.var_ac31__blk812);
        let assign25400_e34439: f64 = (0.5 * locals.var_ac4__blk810);
        let assign25400_e34441: f64 = (assign25400_e34439 / locals.var_ac31__blk812);
        let assign25400_e34442: f64 = (assign25400_e34436 + assign25400_e34441);
        let assign25400_e34445: f64 = (9.0 * locals.var_ty__blk782);
        let assign25400_e34448: f64 = (locals.var_tx__blk781 - 2.0);
        let assign25400_e34449: f64 = (assign25400_e34445 * assign25400_e34448);
        let assign25400_e34450: f64 = (assign25400_e34442 + assign25400_e34449);
        (assign25400_e34450, ((locals.var_ac31__blk812_dn0 + ((((0.5 * locals.var_ac4__blk810_dn0) * locals.var_ac31__blk812) - (assign25400_e34439 * locals.var_ac31__blk812_dn0)) / (locals.var_ac31__blk812 * locals.var_ac31__blk812))) + (((9.0 * locals.var_ty__blk782_dn0) * assign25400_e34448) + (assign25400_e34445 * locals.var_tx__blk781_dn0))), ((locals.var_ac31__blk812_dn2 + ((((0.5 * locals.var_ac4__blk810_dn2) * locals.var_ac31__blk812) - (assign25400_e34439 * locals.var_ac31__blk812_dn2)) / (locals.var_ac31__blk812 * locals.var_ac31__blk812))) + (((9.0 * locals.var_ty__blk782_dn2) * assign25400_e34448) + (assign25400_e34445 * locals.var_tx__blk781_dn2))), ((locals.var_ac31__blk812_dn6 + ((((0.5 * locals.var_ac4__blk810_dn6) * locals.var_ac31__blk812) - (assign25400_e34439 * locals.var_ac31__blk812_dn6)) / (locals.var_ac31__blk812 * locals.var_ac31__blk812))) + (((9.0 * locals.var_ty__blk782_dn6) * assign25400_e34448) + (assign25400_e34445 * locals.var_tx__blk781_dn6))), ((locals.var_ac31__blk812_dn7 + ((((0.5 * locals.var_ac4__blk810_dn7) * locals.var_ac31__blk812) - (assign25400_e34439 * locals.var_ac31__blk812_dn7)) / (locals.var_ac31__blk812 * locals.var_ac31__blk812))) + (((9.0 * locals.var_ty__blk782_dn7) * assign25400_e34448) + (assign25400_e34445 * locals.var_tx__blk781_dn7))), ((locals.var_ac31__blk812_dn10 + ((((0.5 * locals.var_ac4__blk810_dn10) * locals.var_ac31__blk812) - (assign25400_e34439 * locals.var_ac31__blk812_dn10)) / (locals.var_ac31__blk812 * locals.var_ac31__blk812))) + (((9.0 * locals.var_ty__blk782_dn10) * assign25400_e34448) + (assign25400_e34445 * locals.var_tx__blk781_dn10))), ((locals.var_ac31__blk812_dn11 + ((((0.5 * locals.var_ac4__blk810_dn11) * locals.var_ac31__blk812) - (assign25400_e34439 * locals.var_ac31__blk812_dn11)) / (locals.var_ac31__blk812 * locals.var_ac31__blk812))) + (((9.0 * locals.var_ty__blk782_dn11) * assign25400_e34448) + (assign25400_e34445 * locals.var_tx__blk781_dn11))), ((locals.var_ac31__blk812_dn12 + ((((0.5 * locals.var_ac4__blk810_dn12) * locals.var_ac31__blk812) - (assign25400_e34439 * locals.var_ac31__blk812_dn12)) / (locals.var_ac31__blk812 * locals.var_ac31__blk812))) + (((9.0 * locals.var_ty__blk782_dn12) * assign25400_e34448) + (assign25400_e34445 * locals.var_tx__blk781_dn12))), ((locals.var_ac31__blk812_dn17 + ((((0.5 * locals.var_ac4__blk810_dn17) * locals.var_ac31__blk812) - (assign25400_e34439 * locals.var_ac31__blk812_dn17)) / (locals.var_ac31__blk812 * locals.var_ac31__blk812))) + (((9.0 * locals.var_ty__blk782_dn17) * assign25400_e34448) + (assign25400_e34445 * locals.var_tx__blk781_dn17))),)
    } else {
        (locals.var_ac1__blk815, locals.var_ac1__blk815_dn0, locals.var_ac1__blk815_dn2, locals.var_ac1__blk815_dn6, locals.var_ac1__blk815_dn7, locals.var_ac1__blk815_dn10, locals.var_ac1__blk815_dn11, locals.var_ac1__blk815_dn12, locals.var_ac1__blk815_dn17,)
    }
};
        locals.var_ac1__blk815 = assign25400_e34452;
        locals.var_ac1__blk815_dn0 = assign25400_e34452_d_n0;
        locals.var_ac1__blk815_dn2 = assign25400_e34452_d_n2;
        locals.var_ac1__blk815_dn6 = assign25400_e34452_d_n6;
        locals.var_ac1__blk815_dn7 = assign25400_e34452_d_n7;
        locals.var_ac1__blk815_dn10 = assign25400_e34452_d_n10;
        locals.var_ac1__blk815_dn11 = assign25400_e34452_d_n11;
        locals.var_ac1__blk815_dn12 = assign25400_e34452_d_n12;
        locals.var_ac1__blk815_dn17 = assign25400_e34452_d_n17;
        locals.var_ac1__blk815_rv = 0.0;

        let (assign25410_e34468, assign25410_e34468_d_n0, assign25410_e34468_d_n2, assign25410_e34468_d_n6, assign25410_e34468_d_n7, assign25410_e34468_d_n10, assign25410_e34468_d_n11, assign25410_e34468_d_n12, assign25410_e34468_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) && (locals.var_guard858 == 0.0)) {
        let assign25410_e34465: f64 = (locals.var_ac4__blk810 + locals.var_ac3__blk813);
        let assign25410_e34466: f64 = (assign25410_e34465).sqrt();
        (assign25410_e34466, ((locals.var_ac4__blk810_dn0 + locals.var_ac3__blk813_dn0) / (2.0 * assign25410_e34466)), ((locals.var_ac4__blk810_dn2 + locals.var_ac3__blk813_dn2) / (2.0 * assign25410_e34466)), ((locals.var_ac4__blk810_dn6 + locals.var_ac3__blk813_dn6) / (2.0 * assign25410_e34466)), ((locals.var_ac4__blk810_dn7 + locals.var_ac3__blk813_dn7) / (2.0 * assign25410_e34466)), ((locals.var_ac4__blk810_dn10 + locals.var_ac3__blk813_dn10) / (2.0 * assign25410_e34466)), ((locals.var_ac4__blk810_dn11 + locals.var_ac3__blk813_dn11) / (2.0 * assign25410_e34466)), ((locals.var_ac4__blk810_dn12 + locals.var_ac3__blk813_dn12) / (2.0 * assign25410_e34466)), ((locals.var_ac4__blk810_dn17 + locals.var_ac3__blk813_dn17) / (2.0 * assign25410_e34466)),)
    } else {
        (locals.var_ac2__blk814, locals.var_ac2__blk814_dn0, locals.var_ac2__blk814_dn2, locals.var_ac2__blk814_dn6, locals.var_ac2__blk814_dn7, locals.var_ac2__blk814_dn10, locals.var_ac2__blk814_dn11, locals.var_ac2__blk814_dn12, locals.var_ac2__blk814_dn17,)
    }
};
        locals.var_ac2__blk814 = assign25410_e34468;
        locals.var_ac2__blk814_dn0 = assign25410_e34468_d_n0;
        locals.var_ac2__blk814_dn2 = assign25410_e34468_d_n2;
        locals.var_ac2__blk814_dn6 = assign25410_e34468_d_n6;
        locals.var_ac2__blk814_dn7 = assign25410_e34468_d_n7;
        locals.var_ac2__blk814_dn10 = assign25410_e34468_d_n10;
        locals.var_ac2__blk814_dn11 = assign25410_e34468_d_n11;
        locals.var_ac2__blk814_dn12 = assign25410_e34468_d_n12;
        locals.var_ac2__blk814_dn17 = assign25410_e34468_d_n17;
        locals.var_ac2__blk814_rv = 0.0;

        let (assign25420_e34494, assign25420_e34494_d_n0, assign25420_e34494_d_n2, assign25420_e34494_d_n6, assign25420_e34494_d_n7, assign25420_e34494_d_n10, assign25420_e34494_d_n11, assign25420_e34494_d_n12, assign25420_e34494_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) && (locals.var_guard858 == 0.0)) {
        let assign25420_e34480: f64 = (-7.0);
        let assign25420_e34482: f64 = (assign25420_e34480 * 1.414213562373095);
        let assign25420_e34484: f64 = (assign25420_e34482 + locals.var_ac2__blk814);
        let assign25420_e34487: f64 = (9.0 * locals.var_ty__blk782);
        let assign25420_e34490: f64 = (locals.var_tx__blk781 - 2.0);
        let assign25420_e34491: f64 = (assign25420_e34487 * assign25420_e34490);
        let assign25420_e34492: f64 = (assign25420_e34484 + assign25420_e34491);
        (assign25420_e34492, (locals.var_ac2__blk814_dn0 + (((9.0 * locals.var_ty__blk782_dn0) * assign25420_e34490) + (assign25420_e34487 * locals.var_tx__blk781_dn0))), (locals.var_ac2__blk814_dn2 + (((9.0 * locals.var_ty__blk782_dn2) * assign25420_e34490) + (assign25420_e34487 * locals.var_tx__blk781_dn2))), (locals.var_ac2__blk814_dn6 + (((9.0 * locals.var_ty__blk782_dn6) * assign25420_e34490) + (assign25420_e34487 * locals.var_tx__blk781_dn6))), (locals.var_ac2__blk814_dn7 + (((9.0 * locals.var_ty__blk782_dn7) * assign25420_e34490) + (assign25420_e34487 * locals.var_tx__blk781_dn7))), (locals.var_ac2__blk814_dn10 + (((9.0 * locals.var_ty__blk782_dn10) * assign25420_e34490) + (assign25420_e34487 * locals.var_tx__blk781_dn10))), (locals.var_ac2__blk814_dn11 + (((9.0 * locals.var_ty__blk782_dn11) * assign25420_e34490) + (assign25420_e34487 * locals.var_tx__blk781_dn11))), (locals.var_ac2__blk814_dn12 + (((9.0 * locals.var_ty__blk782_dn12) * assign25420_e34490) + (assign25420_e34487 * locals.var_tx__blk781_dn12))), (locals.var_ac2__blk814_dn17 + (((9.0 * locals.var_ty__blk782_dn17) * assign25420_e34490) + (assign25420_e34487 * locals.var_tx__blk781_dn17))),)
    } else {
        (locals.var_ac1__blk815, locals.var_ac1__blk815_dn0, locals.var_ac1__blk815_dn2, locals.var_ac1__blk815_dn6, locals.var_ac1__blk815_dn7, locals.var_ac1__blk815_dn10, locals.var_ac1__blk815_dn11, locals.var_ac1__blk815_dn12, locals.var_ac1__blk815_dn17,)
    }
};
        locals.var_ac1__blk815 = assign25420_e34494;
        locals.var_ac1__blk815_dn0 = assign25420_e34494_d_n0;
        locals.var_ac1__blk815_dn2 = assign25420_e34494_d_n2;
        locals.var_ac1__blk815_dn6 = assign25420_e34494_d_n6;
        locals.var_ac1__blk815_dn7 = assign25420_e34494_d_n7;
        locals.var_ac1__blk815_dn10 = assign25420_e34494_d_n10;
        locals.var_ac1__blk815_dn11 = assign25420_e34494_d_n11;
        locals.var_ac1__blk815_dn12 = assign25420_e34494_d_n12;
        locals.var_ac1__blk815_dn17 = assign25420_e34494_d_n17;
        locals.var_ac1__blk815_rv = 0.0;

        let (assign25430_e34506, assign25430_e34506_d_n0, assign25430_e34506_d_n2, assign25430_e34506_d_n6, assign25430_e34506_d_n7, assign25430_e34506_d_n10, assign25430_e34506_d_n11, assign25430_e34506_d_n12, assign25430_e34506_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25430_e34504: f64 = (locals.var_ac1__blk815).powf(0.3333333333333333);
        (assign25430_e34504, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk815).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk815_dn0)) } } else { (assign25430_e34504 * (0.3333333333333333 * (locals.var_ac1__blk815_dn0 / locals.var_ac1__blk815))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk815).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk815_dn2)) } } else { (assign25430_e34504 * (0.3333333333333333 * (locals.var_ac1__blk815_dn2 / locals.var_ac1__blk815))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk815).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk815_dn6)) } } else { (assign25430_e34504 * (0.3333333333333333 * (locals.var_ac1__blk815_dn6 / locals.var_ac1__blk815))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk815).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk815_dn7)) } } else { (assign25430_e34504 * (0.3333333333333333 * (locals.var_ac1__blk815_dn7 / locals.var_ac1__blk815))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk815).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk815_dn10)) } } else { (assign25430_e34504 * (0.3333333333333333 * (locals.var_ac1__blk815_dn10 / locals.var_ac1__blk815))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk815).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk815_dn11)) } } else { (assign25430_e34504 * (0.3333333333333333 * (locals.var_ac1__blk815_dn11 / locals.var_ac1__blk815))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk815).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk815_dn12)) } } else { (assign25430_e34504 * (0.3333333333333333 * (locals.var_ac1__blk815_dn12 / locals.var_ac1__blk815))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk815).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk815_dn17)) } } else { (assign25430_e34504 * (0.3333333333333333 * (locals.var_ac1__blk815_dn17 / locals.var_ac1__blk815))) },)
    } else {
        (locals.var_acd__blk816, locals.var_acd__blk816_dn0, locals.var_acd__blk816_dn2, locals.var_acd__blk816_dn6, locals.var_acd__blk816_dn7, locals.var_acd__blk816_dn10, locals.var_acd__blk816_dn11, locals.var_acd__blk816_dn12, locals.var_acd__blk816_dn17,)
    }
};
        locals.var_acd__blk816 = assign25430_e34506;
        locals.var_acd__blk816_dn0 = assign25430_e34506_d_n0;
        locals.var_acd__blk816_dn2 = assign25430_e34506_d_n2;
        locals.var_acd__blk816_dn6 = assign25430_e34506_d_n6;
        locals.var_acd__blk816_dn7 = assign25430_e34506_d_n7;
        locals.var_acd__blk816_dn10 = assign25430_e34506_d_n10;
        locals.var_acd__blk816_dn11 = assign25430_e34506_d_n11;
        locals.var_acd__blk816_dn12 = assign25430_e34506_d_n12;
        locals.var_acd__blk816_dn17 = assign25430_e34506_d_n17;
        locals.var_acd__blk816_rv = 0.0;

        let (assign25440_e34533, assign25440_e34533_d_n0, assign25440_e34533_d_n2, assign25440_e34533_d_n6, assign25440_e34533_d_n7, assign25440_e34533_d_n10, assign25440_e34533_d_n11, assign25440_e34533_d_n12, assign25440_e34533_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25440_e34515: f64 = (-4.0);
        let assign25440_e34517: f64 = (assign25440_e34515 * 1.414213562373095);
        let assign25440_e34520: f64 = (12.0 * locals.var_ty__blk782);
        let assign25440_e34521: f64 = (assign25440_e34517 - assign25440_e34520);
        let assign25440_e34524: f64 = (2.0 * locals.var_acd__blk816);
        let assign25440_e34525: f64 = (assign25440_e34521 + assign25440_e34524);
        let assign25440_e34528: f64 = (1.414213562373095 * locals.var_acd__blk816);
        let assign25440_e34530: f64 = (assign25440_e34528 * locals.var_acd__blk816);
        let assign25440_e34531: f64 = (assign25440_e34525 + assign25440_e34530);
        (assign25440_e34531, (((-(12.0 * locals.var_ty__blk782_dn0)) + (2.0 * locals.var_acd__blk816_dn0)) + (((1.414213562373095 * locals.var_acd__blk816_dn0) * locals.var_acd__blk816) + (assign25440_e34528 * locals.var_acd__blk816_dn0))), (((-(12.0 * locals.var_ty__blk782_dn2)) + (2.0 * locals.var_acd__blk816_dn2)) + (((1.414213562373095 * locals.var_acd__blk816_dn2) * locals.var_acd__blk816) + (assign25440_e34528 * locals.var_acd__blk816_dn2))), (((-(12.0 * locals.var_ty__blk782_dn6)) + (2.0 * locals.var_acd__blk816_dn6)) + (((1.414213562373095 * locals.var_acd__blk816_dn6) * locals.var_acd__blk816) + (assign25440_e34528 * locals.var_acd__blk816_dn6))), (((-(12.0 * locals.var_ty__blk782_dn7)) + (2.0 * locals.var_acd__blk816_dn7)) + (((1.414213562373095 * locals.var_acd__blk816_dn7) * locals.var_acd__blk816) + (assign25440_e34528 * locals.var_acd__blk816_dn7))), (((-(12.0 * locals.var_ty__blk782_dn10)) + (2.0 * locals.var_acd__blk816_dn10)) + (((1.414213562373095 * locals.var_acd__blk816_dn10) * locals.var_acd__blk816) + (assign25440_e34528 * locals.var_acd__blk816_dn10))), (((-(12.0 * locals.var_ty__blk782_dn11)) + (2.0 * locals.var_acd__blk816_dn11)) + (((1.414213562373095 * locals.var_acd__blk816_dn11) * locals.var_acd__blk816) + (assign25440_e34528 * locals.var_acd__blk816_dn11))), (((-(12.0 * locals.var_ty__blk782_dn12)) + (2.0 * locals.var_acd__blk816_dn12)) + (((1.414213562373095 * locals.var_acd__blk816_dn12) * locals.var_acd__blk816) + (assign25440_e34528 * locals.var_acd__blk816_dn12))), (((-(12.0 * locals.var_ty__blk782_dn17)) + (2.0 * locals.var_acd__blk816_dn17)) + (((1.414213562373095 * locals.var_acd__blk816_dn17) * locals.var_acd__blk816) + (assign25440_e34528 * locals.var_acd__blk816_dn17))),)
    } else {
        (locals.var_acn__blk817, locals.var_acn__blk817_dn0, locals.var_acn__blk817_dn2, locals.var_acn__blk817_dn6, locals.var_acn__blk817_dn7, locals.var_acn__blk817_dn10, locals.var_acn__blk817_dn11, locals.var_acn__blk817_dn12, locals.var_acn__blk817_dn17,)
    }
};
        locals.var_acn__blk817 = assign25440_e34533;
        locals.var_acn__blk817_dn0 = assign25440_e34533_d_n0;
        locals.var_acn__blk817_dn2 = assign25440_e34533_d_n2;
        locals.var_acn__blk817_dn6 = assign25440_e34533_d_n6;
        locals.var_acn__blk817_dn7 = assign25440_e34533_d_n7;
        locals.var_acn__blk817_dn10 = assign25440_e34533_d_n10;
        locals.var_acn__blk817_dn11 = assign25440_e34533_d_n11;
        locals.var_acn__blk817_dn12 = assign25440_e34533_d_n12;
        locals.var_acn__blk817_dn17 = assign25440_e34533_d_n17;
        locals.var_acn__blk817_rv = 0.0;

        let (assign25450_e34545, assign25450_e34545_d_n0, assign25450_e34545_d_n2, assign25450_e34545_d_n6, assign25450_e34545_d_n7, assign25450_e34545_d_n10, assign25450_e34545_d_n11, assign25450_e34545_d_n12, assign25450_e34545_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25450_e34543: f64 = (locals.var_acn__blk817 / locals.var_acd__blk816);
        (assign25450_e34543, (((locals.var_acn__blk817_dn0 * locals.var_acd__blk816) - (locals.var_acn__blk817 * locals.var_acd__blk816_dn0)) / (locals.var_acd__blk816 * locals.var_acd__blk816)), (((locals.var_acn__blk817_dn2 * locals.var_acd__blk816) - (locals.var_acn__blk817 * locals.var_acd__blk816_dn2)) / (locals.var_acd__blk816 * locals.var_acd__blk816)), (((locals.var_acn__blk817_dn6 * locals.var_acd__blk816) - (locals.var_acn__blk817 * locals.var_acd__blk816_dn6)) / (locals.var_acd__blk816 * locals.var_acd__blk816)), (((locals.var_acn__blk817_dn7 * locals.var_acd__blk816) - (locals.var_acn__blk817 * locals.var_acd__blk816_dn7)) / (locals.var_acd__blk816 * locals.var_acd__blk816)), (((locals.var_acn__blk817_dn10 * locals.var_acd__blk816) - (locals.var_acn__blk817 * locals.var_acd__blk816_dn10)) / (locals.var_acd__blk816 * locals.var_acd__blk816)), (((locals.var_acn__blk817_dn11 * locals.var_acd__blk816) - (locals.var_acn__blk817 * locals.var_acd__blk816_dn11)) / (locals.var_acd__blk816 * locals.var_acd__blk816)), (((locals.var_acn__blk817_dn12 * locals.var_acd__blk816) - (locals.var_acn__blk817 * locals.var_acd__blk816_dn12)) / (locals.var_acd__blk816 * locals.var_acd__blk816)), (((locals.var_acn__blk817_dn17 * locals.var_acd__blk816) - (locals.var_acn__blk817 * locals.var_acd__blk816_dn17)) / (locals.var_acd__blk816 * locals.var_acd__blk816)),)
    } else {
        (locals.var_chi__blk818, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    }
};
        locals.var_chi__blk818 = assign25450_e34545;
        locals.var_chi__blk818_dn0 = assign25450_e34545_d_n0;
        locals.var_chi__blk818_dn2 = assign25450_e34545_d_n2;
        locals.var_chi__blk818_dn6 = assign25450_e34545_d_n6;
        locals.var_chi__blk818_dn7 = assign25450_e34545_d_n7;
        locals.var_chi__blk818_dn10 = assign25450_e34545_d_n10;
        locals.var_chi__blk818_dn11 = assign25450_e34545_d_n11;
        locals.var_chi__blk818_dn12 = assign25450_e34545_d_n12;
        locals.var_chi__blk818_dn17 = assign25450_e34545_d_n17;
        locals.var_chi__blk818_rv = 0.0;

        let (assign25460_e34559, assign25460_e34559_d_n0, assign25460_e34559_d_n2, assign25460_e34559_d_n6, assign25460_e34559_d_n7, assign25460_e34559_d_n10, assign25460_e34559_d_n11, assign25460_e34559_d_n12, assign25460_e34559_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25460_e34555: f64 = (locals.var_chi__blk818 * locals.var_beta_inv);
        let assign25460_e34557: f64 = (assign25460_e34555 - locals.var_vxbgmtcl);
        (assign25460_e34557, ((locals.var_chi__blk818_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk818_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk818_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk818_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn7), (((locals.var_chi__blk818_dn10 * locals.var_beta_inv) + (locals.var_chi__blk818 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk818_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk818_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk818_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_psa__blk819, locals.var_psa__blk819_dn0, locals.var_psa__blk819_dn2, locals.var_psa__blk819_dn6, locals.var_psa__blk819_dn7, locals.var_psa__blk819_dn10, locals.var_psa__blk819_dn11, locals.var_psa__blk819_dn12, locals.var_psa__blk819_dn17,)
    }
};
        locals.var_psa__blk819 = assign25460_e34559;
        locals.var_psa__blk819_dn0 = assign25460_e34559_d_n0;
        locals.var_psa__blk819_dn2 = assign25460_e34559_d_n2;
        locals.var_psa__blk819_dn6 = assign25460_e34559_d_n6;
        locals.var_psa__blk819_dn7 = assign25460_e34559_d_n7;
        locals.var_psa__blk819_dn10 = assign25460_e34559_d_n10;
        locals.var_psa__blk819_dn11 = assign25460_e34559_d_n11;
        locals.var_psa__blk819_dn12 = assign25460_e34559_d_n12;
        locals.var_psa__blk819_dn17 = assign25460_e34559_d_n17;
        locals.var_psa__blk819_rv = 0.0;

        let (assign25470_e34571, assign25470_e34571_d_n0, assign25470_e34571_d_n2, assign25470_e34571_d_n6, assign25470_e34571_d_n7, assign25470_e34571_d_n10, assign25470_e34571_d_n11, assign25470_e34571_d_n12, assign25470_e34571_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25470_e34569: f64 = (locals.var_psa__blk819 + locals.var_vxbgmtcl);
        (assign25470_e34569, (locals.var_psa__blk819_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_psa__blk819_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_psa__blk819_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_psa__blk819_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_psa__blk819_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_psa__blk819_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_psa__blk819_dn12 + locals.var_vxbgmtcl_dn12), (locals.var_psa__blk819_dn17 + locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_t1__blk775, locals.var_t1__blk775_dn0, locals.var_t1__blk775_dn2, locals.var_t1__blk775_dn6, locals.var_t1__blk775_dn7, locals.var_t1__blk775_dn10, locals.var_t1__blk775_dn11, locals.var_t1__blk775_dn12, locals.var_t1__blk775_dn17,)
    }
};
        locals.var_t1__blk775 = assign25470_e34571;
        locals.var_t1__blk775_dn0 = assign25470_e34571_d_n0;
        locals.var_t1__blk775_dn2 = assign25470_e34571_d_n2;
        locals.var_t1__blk775_dn6 = assign25470_e34571_d_n6;
        locals.var_t1__blk775_dn7 = assign25470_e34571_d_n7;
        locals.var_t1__blk775_dn10 = assign25470_e34571_d_n10;
        locals.var_t1__blk775_dn11 = assign25470_e34571_d_n11;
        locals.var_t1__blk775_dn12 = assign25470_e34571_d_n12;
        locals.var_t1__blk775_dn17 = assign25470_e34571_d_n17;
        locals.var_t1__blk775_rv = 0.0;

        let (assign25480_e34583, assign25480_e34583_d_n0, assign25480_e34583_d_n2, assign25480_e34583_d_n6, assign25480_e34583_d_n7, assign25480_e34583_d_n10, assign25480_e34583_d_n11, assign25480_e34583_d_n12, assign25480_e34583_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25480_e34581: f64 = (locals.var_t1__blk775 / locals.var_ps0_min__blk811);
        (assign25480_e34581, (((locals.var_t1__blk775_dn0 * locals.var_ps0_min__blk811) - (locals.var_t1__blk775 * locals.var_ps0_min__blk811_dn0)) / (locals.var_ps0_min__blk811 * locals.var_ps0_min__blk811)), (((locals.var_t1__blk775_dn2 * locals.var_ps0_min__blk811) - (locals.var_t1__blk775 * locals.var_ps0_min__blk811_dn2)) / (locals.var_ps0_min__blk811 * locals.var_ps0_min__blk811)), (((locals.var_t1__blk775_dn6 * locals.var_ps0_min__blk811) - (locals.var_t1__blk775 * locals.var_ps0_min__blk811_dn6)) / (locals.var_ps0_min__blk811 * locals.var_ps0_min__blk811)), (((locals.var_t1__blk775_dn7 * locals.var_ps0_min__blk811) - (locals.var_t1__blk775 * locals.var_ps0_min__blk811_dn7)) / (locals.var_ps0_min__blk811 * locals.var_ps0_min__blk811)), (((locals.var_t1__blk775_dn10 * locals.var_ps0_min__blk811) - (locals.var_t1__blk775 * locals.var_ps0_min__blk811_dn10)) / (locals.var_ps0_min__blk811 * locals.var_ps0_min__blk811)), (((locals.var_t1__blk775_dn11 * locals.var_ps0_min__blk811) - (locals.var_t1__blk775 * locals.var_ps0_min__blk811_dn11)) / (locals.var_ps0_min__blk811 * locals.var_ps0_min__blk811)), (((locals.var_t1__blk775_dn12 * locals.var_ps0_min__blk811) - (locals.var_t1__blk775 * locals.var_ps0_min__blk811_dn12)) / (locals.var_ps0_min__blk811 * locals.var_ps0_min__blk811)), (((locals.var_t1__blk775_dn17 * locals.var_ps0_min__blk811) - (locals.var_t1__blk775 * locals.var_ps0_min__blk811_dn17)) / (locals.var_ps0_min__blk811 * locals.var_ps0_min__blk811)),)
    } else {
        (locals.var_t2__blk776, locals.var_t2__blk776_dn0, locals.var_t2__blk776_dn2, locals.var_t2__blk776_dn6, locals.var_t2__blk776_dn7, locals.var_t2__blk776_dn10, locals.var_t2__blk776_dn11, locals.var_t2__blk776_dn12, locals.var_t2__blk776_dn17,)
    }
};
        locals.var_t2__blk776 = assign25480_e34583;
        locals.var_t2__blk776_dn0 = assign25480_e34583_d_n0;
        locals.var_t2__blk776_dn2 = assign25480_e34583_d_n2;
        locals.var_t2__blk776_dn6 = assign25480_e34583_d_n6;
        locals.var_t2__blk776_dn7 = assign25480_e34583_d_n7;
        locals.var_t2__blk776_dn10 = assign25480_e34583_d_n10;
        locals.var_t2__blk776_dn11 = assign25480_e34583_d_n11;
        locals.var_t2__blk776_dn12 = assign25480_e34583_d_n12;
        locals.var_t2__blk776_dn17 = assign25480_e34583_d_n17;
        locals.var_t2__blk776_rv = 0.0;

        let (assign25490_e34598, assign25490_e34598_d_n0, assign25490_e34598_d_n2, assign25490_e34598_d_n6, assign25490_e34598_d_n7, assign25490_e34598_d_n10, assign25490_e34598_d_n11, assign25490_e34598_d_n12, assign25490_e34598_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25490_e34594: f64 = (locals.var_t2__blk776 * locals.var_t2__blk776);
        let assign25490_e34595: f64 = (1.0 + assign25490_e34594);
        let assign25490_e34596: f64 = (assign25490_e34595).sqrt();
        (assign25490_e34596, (((locals.var_t2__blk776_dn0 * locals.var_t2__blk776) + (locals.var_t2__blk776 * locals.var_t2__blk776_dn0)) / (2.0 * assign25490_e34596)), (((locals.var_t2__blk776_dn2 * locals.var_t2__blk776) + (locals.var_t2__blk776 * locals.var_t2__blk776_dn2)) / (2.0 * assign25490_e34596)), (((locals.var_t2__blk776_dn6 * locals.var_t2__blk776) + (locals.var_t2__blk776 * locals.var_t2__blk776_dn6)) / (2.0 * assign25490_e34596)), (((locals.var_t2__blk776_dn7 * locals.var_t2__blk776) + (locals.var_t2__blk776 * locals.var_t2__blk776_dn7)) / (2.0 * assign25490_e34596)), (((locals.var_t2__blk776_dn10 * locals.var_t2__blk776) + (locals.var_t2__blk776 * locals.var_t2__blk776_dn10)) / (2.0 * assign25490_e34596)), (((locals.var_t2__blk776_dn11 * locals.var_t2__blk776) + (locals.var_t2__blk776 * locals.var_t2__blk776_dn11)) / (2.0 * assign25490_e34596)), (((locals.var_t2__blk776_dn12 * locals.var_t2__blk776) + (locals.var_t2__blk776 * locals.var_t2__blk776_dn12)) / (2.0 * assign25490_e34596)), (((locals.var_t2__blk776_dn17 * locals.var_t2__blk776) + (locals.var_t2__blk776 * locals.var_t2__blk776_dn17)) / (2.0 * assign25490_e34596)),)
    } else {
        (locals.var_t3__blk777, locals.var_t3__blk777_dn0, locals.var_t3__blk777_dn2, locals.var_t3__blk777_dn6, locals.var_t3__blk777_dn7, locals.var_t3__blk777_dn10, locals.var_t3__blk777_dn11, locals.var_t3__blk777_dn12, locals.var_t3__blk777_dn17,)
    }
};
        locals.var_t3__blk777 = assign25490_e34598;
        locals.var_t3__blk777_dn0 = assign25490_e34598_d_n0;
        locals.var_t3__blk777_dn2 = assign25490_e34598_d_n2;
        locals.var_t3__blk777_dn6 = assign25490_e34598_d_n6;
        locals.var_t3__blk777_dn7 = assign25490_e34598_d_n7;
        locals.var_t3__blk777_dn10 = assign25490_e34598_d_n10;
        locals.var_t3__blk777_dn11 = assign25490_e34598_d_n11;
        locals.var_t3__blk777_dn12 = assign25490_e34598_d_n12;
        locals.var_t3__blk777_dn17 = assign25490_e34598_d_n17;
        locals.var_t3__blk777_rv = 0.0;

        let (assign25500_e34612, assign25500_e34612_d_n0, assign25500_e34612_d_n2, assign25500_e34612_d_n6, assign25500_e34612_d_n7, assign25500_e34612_d_n10, assign25500_e34612_d_n11, assign25500_e34612_d_n12, assign25500_e34612_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25500_e34608: f64 = (locals.var_t1__blk775 / locals.var_t3__blk777);
        let assign25500_e34610: f64 = (assign25500_e34608 - locals.var_vxbgmtcl);
        (assign25500_e34610, ((((locals.var_t1__blk775_dn0 * locals.var_t3__blk777) - (locals.var_t1__blk775 * locals.var_t3__blk777_dn0)) / (locals.var_t3__blk777 * locals.var_t3__blk777)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1__blk775_dn2 * locals.var_t3__blk777) - (locals.var_t1__blk775 * locals.var_t3__blk777_dn2)) / (locals.var_t3__blk777 * locals.var_t3__blk777)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1__blk775_dn6 * locals.var_t3__blk777) - (locals.var_t1__blk775 * locals.var_t3__blk777_dn6)) / (locals.var_t3__blk777 * locals.var_t3__blk777)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1__blk775_dn7 * locals.var_t3__blk777) - (locals.var_t1__blk775 * locals.var_t3__blk777_dn7)) / (locals.var_t3__blk777 * locals.var_t3__blk777)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1__blk775_dn10 * locals.var_t3__blk777) - (locals.var_t1__blk775 * locals.var_t3__blk777_dn10)) / (locals.var_t3__blk777 * locals.var_t3__blk777)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1__blk775_dn11 * locals.var_t3__blk777) - (locals.var_t1__blk775 * locals.var_t3__blk777_dn11)) / (locals.var_t3__blk777 * locals.var_t3__blk777)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1__blk775_dn12 * locals.var_t3__blk777) - (locals.var_t1__blk775 * locals.var_t3__blk777_dn12)) / (locals.var_t3__blk777 * locals.var_t3__blk777)) - locals.var_vxbgmtcl_dn12), ((((locals.var_t1__blk775_dn17 * locals.var_t3__blk777) - (locals.var_t1__blk775 * locals.var_t3__blk777_dn17)) / (locals.var_t3__blk777 * locals.var_t3__blk777)) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
        locals.var_ps0ld = assign25500_e34612;
        locals.var_ps0ld_dn0 = assign25500_e34612_d_n0;
        locals.var_ps0ld_dn2 = assign25500_e34612_d_n2;
        locals.var_ps0ld_dn6 = assign25500_e34612_d_n6;
        locals.var_ps0ld_dn7 = assign25500_e34612_d_n7;
        locals.var_ps0ld_dn10 = assign25500_e34612_d_n10;
        locals.var_ps0ld_dn11 = assign25500_e34612_d_n11;
        locals.var_ps0ld_dn12 = assign25500_e34612_d_n12;
        locals.var_ps0ld_dn17 = assign25500_e34612_d_n17;
        locals.var_ps0ld_rv = 0.0;

        let (assign25510_e34624, assign25510_e34624_d_n0, assign25510_e34624_d_n2, assign25510_e34624_d_n6, assign25510_e34624_d_n7, assign25510_e34624_d_n10, assign25510_e34624_d_n11, assign25510_e34624_d_n12, assign25510_e34624_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25510_e34622: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign25510_e34622, (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn10 - locals.var_ps0ld_dn10), (locals.var_vgpld_dn11 - locals.var_ps0ld_dn11), (locals.var_vgpld_dn12 - locals.var_ps0ld_dn12), (locals.var_vgpld_dn17 - locals.var_ps0ld_dn17),)
    } else {
        (locals.var_t2__blk776, locals.var_t2__blk776_dn0, locals.var_t2__blk776_dn2, locals.var_t2__blk776_dn6, locals.var_t2__blk776_dn7, locals.var_t2__blk776_dn10, locals.var_t2__blk776_dn11, locals.var_t2__blk776_dn12, locals.var_t2__blk776_dn17,)
    }
};
        locals.var_t2__blk776 = assign25510_e34624;
        locals.var_t2__blk776_dn0 = assign25510_e34624_d_n0;
        locals.var_t2__blk776_dn2 = assign25510_e34624_d_n2;
        locals.var_t2__blk776_dn6 = assign25510_e34624_d_n6;
        locals.var_t2__blk776_dn7 = assign25510_e34624_d_n7;
        locals.var_t2__blk776_dn10 = assign25510_e34624_d_n10;
        locals.var_t2__blk776_dn11 = assign25510_e34624_d_n11;
        locals.var_t2__blk776_dn12 = assign25510_e34624_d_n12;
        locals.var_t2__blk776_dn17 = assign25510_e34624_d_n17;
        locals.var_t2__blk776_rv = 0.0;

        let (assign25520_e34636, assign25520_e34636_d_n0, assign25520_e34636_d_n2, assign25520_e34636_d_n6, assign25520_e34636_d_n7, assign25520_e34636_d_n10, assign25520_e34636_d_n11, assign25520_e34636_d_n12, assign25520_e34636_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign25520_e34634: f64 = (locals.var_cox0 * locals.var_t2__blk776);
        (assign25520_e34634, (locals.var_cox0 * locals.var_t2__blk776_dn0), (locals.var_cox0 * locals.var_t2__blk776_dn2), (locals.var_cox0 * locals.var_t2__blk776_dn6), (locals.var_cox0 * locals.var_t2__blk776_dn7), (locals.var_cox0 * locals.var_t2__blk776_dn10), (locals.var_cox0 * locals.var_t2__blk776_dn11), (locals.var_cox0 * locals.var_t2__blk776_dn12), (locals.var_cox0 * locals.var_t2__blk776_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign25520_e34636;
        locals.var_qsuld_dn0 = assign25520_e34636_d_n0;
        locals.var_qsuld_dn2 = assign25520_e34636_d_n2;
        locals.var_qsuld_dn6 = assign25520_e34636_d_n6;
        locals.var_qsuld_dn7 = assign25520_e34636_d_n7;
        locals.var_qsuld_dn10 = assign25520_e34636_d_n10;
        locals.var_qsuld_dn11 = assign25520_e34636_d_n11;
        locals.var_qsuld_dn12 = assign25520_e34636_d_n12;
        locals.var_qsuld_dn17 = assign25520_e34636_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign25530_e34646, assign25530_e34646_d_n0, assign25530_e34646_d_n2, assign25530_e34646_d_n6, assign25530_e34646_d_n7, assign25530_e34646_d_n10, assign25530_e34646_d_n11, assign25530_e34646_d_n12, assign25530_e34646_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign25530_e34646;
        locals.var_qbuld_dn0 = assign25530_e34646_d_n0;
        locals.var_qbuld_dn2 = assign25530_e34646_d_n2;
        locals.var_qbuld_dn6 = assign25530_e34646_d_n6;
        locals.var_qbuld_dn7 = assign25530_e34646_d_n7;
        locals.var_qbuld_dn10 = assign25530_e34646_d_n10;
        locals.var_qbuld_dn11 = assign25530_e34646_d_n11;
        locals.var_qbuld_dn12 = assign25530_e34646_d_n12;
        locals.var_qbuld_dn17 = assign25530_e34646_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign25550_e34668, assign25550_e34668_d_n0, assign25550_e34668_d_n2, assign25550_e34668_d_n6, assign25550_e34668_d_n7, assign25550_e34668_d_n10, assign25550_e34668_d_n11, assign25550_e34668_d_n12, assign25550_e34668_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        (3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi__blk818, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    }
};
        locals.var_chi__blk818 = assign25550_e34668;
        locals.var_chi__blk818_dn0 = assign25550_e34668_d_n0;
        locals.var_chi__blk818_dn2 = assign25550_e34668_d_n2;
        locals.var_chi__blk818_dn6 = assign25550_e34668_d_n6;
        locals.var_chi__blk818_dn7 = assign25550_e34668_d_n7;
        locals.var_chi__blk818_dn10 = assign25550_e34668_d_n10;
        locals.var_chi__blk818_dn11 = assign25550_e34668_d_n11;
        locals.var_chi__blk818_dn12 = assign25550_e34668_d_n12;
        locals.var_chi__blk818_dn17 = assign25550_e34668_d_n17;
        locals.var_chi__blk818_rv = 0.0;

        let (assign25560_e34683, assign25560_e34683_d_n0, assign25560_e34683_d_n2, assign25560_e34683_d_n6, assign25560_e34683_d_n7, assign25560_e34683_d_n10, assign25560_e34683_d_n11, assign25560_e34683_d_n12, assign25560_e34683_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25560_e34679: f64 = (locals.var_chi__blk818 / locals.var_beta);
        let assign25560_e34681: f64 = (assign25560_e34679 - locals.var_vxbgmtcl);
        (assign25560_e34681, ((locals.var_chi__blk818_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk818_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk818_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk818_dn7 / locals.var_beta) - locals.var_vxbgmtcl_dn7), ((((locals.var_chi__blk818_dn10 * locals.var_beta) - (locals.var_chi__blk818 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk818_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk818_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk818_dn17 / locals.var_beta) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0_inia__blk821, locals.var_ps0_inia__blk821_dn0, locals.var_ps0_inia__blk821_dn2, locals.var_ps0_inia__blk821_dn6, locals.var_ps0_inia__blk821_dn7, locals.var_ps0_inia__blk821_dn10, locals.var_ps0_inia__blk821_dn11, locals.var_ps0_inia__blk821_dn12, locals.var_ps0_inia__blk821_dn17,)
    }
};
        locals.var_ps0_inia__blk821 = assign25560_e34683;
        locals.var_ps0_inia__blk821_dn0 = assign25560_e34683_d_n0;
        locals.var_ps0_inia__blk821_dn2 = assign25560_e34683_d_n2;
        locals.var_ps0_inia__blk821_dn6 = assign25560_e34683_d_n6;
        locals.var_ps0_inia__blk821_dn7 = assign25560_e34683_d_n7;
        locals.var_ps0_inia__blk821_dn10 = assign25560_e34683_d_n10;
        locals.var_ps0_inia__blk821_dn11 = assign25560_e34683_d_n11;
        locals.var_ps0_inia__blk821_dn12 = assign25560_e34683_d_n12;
        locals.var_ps0_inia__blk821_dn17 = assign25560_e34683_d_n17;
        locals.var_ps0_inia__blk821_rv = 0.0;

        let (assign25570_e34696, assign25570_e34696_d_n0, assign25570_e34696_d_n2, assign25570_e34696_d_n6, assign25570_e34696_d_n7, assign25570_e34696_d_n10, assign25570_e34696_d_n11, assign25570_e34696_d_n12, assign25570_e34696_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25570_e34693: f64 = (-locals.var_chi__blk818);
        let assign25570_e34694: f64 = (assign25570_e34693).exp();
        (assign25570_e34694, (assign25570_e34694 * (-locals.var_chi__blk818_dn0)), (assign25570_e34694 * (-locals.var_chi__blk818_dn2)), (assign25570_e34694 * (-locals.var_chi__blk818_dn6)), (assign25570_e34694 * (-locals.var_chi__blk818_dn7)), (assign25570_e34694 * (-locals.var_chi__blk818_dn10)), (assign25570_e34694 * (-locals.var_chi__blk818_dn11)), (assign25570_e34694 * (-locals.var_chi__blk818_dn12)), (assign25570_e34694 * (-locals.var_chi__blk818_dn17)),)
    } else {
        (locals.var_ty__blk782, locals.var_ty__blk782_dn0, locals.var_ty__blk782_dn2, locals.var_ty__blk782_dn6, locals.var_ty__blk782_dn7, locals.var_ty__blk782_dn10, locals.var_ty__blk782_dn11, locals.var_ty__blk782_dn12, locals.var_ty__blk782_dn17,)
    }
};
        locals.var_ty__blk782 = assign25570_e34696;
        locals.var_ty__blk782_dn0 = assign25570_e34696_d_n0;
        locals.var_ty__blk782_dn2 = assign25570_e34696_d_n2;
        locals.var_ty__blk782_dn6 = assign25570_e34696_d_n6;
        locals.var_ty__blk782_dn7 = assign25570_e34696_d_n7;
        locals.var_ty__blk782_dn10 = assign25570_e34696_d_n10;
        locals.var_ty__blk782_dn11 = assign25570_e34696_d_n11;
        locals.var_ty__blk782_dn12 = assign25570_e34696_d_n12;
        locals.var_ty__blk782_dn17 = assign25570_e34696_d_n17;
        locals.var_ty__blk782_rv = 0.0;

        let (assign25580_e34723, assign25580_e34723_d_n0, assign25580_e34723_d_n2, assign25580_e34723_d_n6, assign25580_e34723_d_n7, assign25580_e34723_d_n10, assign25580_e34723_d_n11, assign25580_e34723_d_n12, assign25580_e34723_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25580_e34710: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign25580_e34711: f64 = (locals.var_beta * assign25580_e34710);
        let assign25580_e34713: f64 = (assign25580_e34711 - 1.0);
        let assign25580_e34715: f64 = (assign25580_e34713 + locals.var_ty__blk782);
        let assign25580_e34716: f64 = (4.0 * assign25580_e34715);
        let assign25580_e34719: f64 = (locals.var_fac1p2__blk805 * locals.var_beta2);
        let assign25580_e34720: f64 = (assign25580_e34716 / assign25580_e34719);
        let assign25580_e34721: f64 = (1.0 + assign25580_e34720);
        (assign25580_e34721, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + locals.var_ty__blk782_dn0)) * assign25580_e34719) - (assign25580_e34716 * (locals.var_fac1p2__blk805_dn0 * locals.var_beta2))) / (assign25580_e34719 * assign25580_e34719)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + locals.var_ty__blk782_dn2)) * assign25580_e34719) - (assign25580_e34716 * (locals.var_fac1p2__blk805_dn2 * locals.var_beta2))) / (assign25580_e34719 * assign25580_e34719)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) + locals.var_ty__blk782_dn6)) * assign25580_e34719) - (assign25580_e34716 * (locals.var_fac1p2__blk805_dn6 * locals.var_beta2))) / (assign25580_e34719 * assign25580_e34719)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) + locals.var_ty__blk782_dn7)) * assign25580_e34719) - (assign25580_e34716 * (locals.var_fac1p2__blk805_dn7 * locals.var_beta2))) / (assign25580_e34719 * assign25580_e34719)), ((((4.0 * (((locals.var_beta_dn10 * assign25580_e34710) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))) + locals.var_ty__blk782_dn10)) * assign25580_e34719) - (assign25580_e34716 * ((locals.var_fac1p2__blk805_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk805 * locals.var_beta2_dn10)))) / (assign25580_e34719 * assign25580_e34719)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) + locals.var_ty__blk782_dn11)) * assign25580_e34719) - (assign25580_e34716 * (locals.var_fac1p2__blk805_dn11 * locals.var_beta2))) / (assign25580_e34719 * assign25580_e34719)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) + locals.var_ty__blk782_dn12)) * assign25580_e34719) - (assign25580_e34716 * (locals.var_fac1p2__blk805_dn12 * locals.var_beta2))) / (assign25580_e34719 * assign25580_e34719)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) + locals.var_ty__blk782_dn17)) * assign25580_e34719) - (assign25580_e34716 * (locals.var_fac1p2__blk805_dn17 * locals.var_beta2))) / (assign25580_e34719 * assign25580_e34719)),)
    } else {
        (locals.var_tx__blk781, locals.var_tx__blk781_dn0, locals.var_tx__blk781_dn2, locals.var_tx__blk781_dn6, locals.var_tx__blk781_dn7, locals.var_tx__blk781_dn10, locals.var_tx__blk781_dn11, locals.var_tx__blk781_dn12, locals.var_tx__blk781_dn17,)
    }
};
        locals.var_tx__blk781 = assign25580_e34723;
        locals.var_tx__blk781_dn0 = assign25580_e34723_d_n0;
        locals.var_tx__blk781_dn2 = assign25580_e34723_d_n2;
        locals.var_tx__blk781_dn6 = assign25580_e34723_d_n6;
        locals.var_tx__blk781_dn7 = assign25580_e34723_d_n7;
        locals.var_tx__blk781_dn10 = assign25580_e34723_d_n10;
        locals.var_tx__blk781_dn11 = assign25580_e34723_d_n11;
        locals.var_tx__blk781_dn12 = assign25580_e34723_d_n12;
        locals.var_tx__blk781_dn17 = assign25580_e34723_d_n17;
        locals.var_tx__blk781_rv = 0.0;

        let assign25590_e34727: f64 = (10.0 * 2.220446049250313e-16);
        let assign25590_e34728: f64 = if locals.var_tx__blk781 < assign25590_e34727 { 1.0 } else { 0.0 };
        locals.var_guard859 = assign25590_e34728;
        locals.var_guard859_rv = 0.0;

        let (assign25600_e34743, assign25600_e34743_d_n0, assign25600_e34743_d_n2, assign25600_e34743_d_n6, assign25600_e34743_d_n7, assign25600_e34743_d_n10, assign25600_e34743_d_n11, assign25600_e34743_d_n12, assign25600_e34743_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign25600_e34741: f64 = (10.0 * 2.220446049250313e-16);
        (assign25600_e34741, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk781, locals.var_tx__blk781_dn0, locals.var_tx__blk781_dn2, locals.var_tx__blk781_dn6, locals.var_tx__blk781_dn7, locals.var_tx__blk781_dn10, locals.var_tx__blk781_dn11, locals.var_tx__blk781_dn12, locals.var_tx__blk781_dn17,)
    }
};
        locals.var_tx__blk781 = assign25600_e34743;
        locals.var_tx__blk781_dn0 = assign25600_e34743_d_n0;
        locals.var_tx__blk781_dn2 = assign25600_e34743_d_n2;
        locals.var_tx__blk781_dn6 = assign25600_e34743_d_n6;
        locals.var_tx__blk781_dn7 = assign25600_e34743_d_n7;
        locals.var_tx__blk781_dn10 = assign25600_e34743_d_n10;
        locals.var_tx__blk781_dn11 = assign25600_e34743_d_n11;
        locals.var_tx__blk781_dn12 = assign25600_e34743_d_n12;
        locals.var_tx__blk781_dn17 = assign25600_e34743_d_n17;
        locals.var_tx__blk781_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_90(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25610_e34765, assign25610_e34765_d_n0, assign25610_e34765_d_n2, assign25610_e34765_d_n6, assign25610_e34765_d_n7, assign25610_e34765_d_n10, assign25610_e34765_d_n11, assign25610_e34765_d_n12, assign25610_e34765_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25610_e34755: f64 = (locals.var_fac1p2__blk805 * locals.var_beta);
        let assign25610_e34757: f64 = (assign25610_e34755 / 2.0);
        let assign25610_e34760: f64 = (locals.var_tx__blk781).sqrt();
        let assign25610_e34761: f64 = (1.0 - assign25610_e34760);
        let assign25610_e34762: f64 = (assign25610_e34757 * assign25610_e34761);
        let assign25610_e34763: f64 = (locals.var_vgpld + assign25610_e34762);
        (assign25610_e34763, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2__blk805_dn0 * locals.var_beta) / 2.0) * assign25610_e34761) + (assign25610_e34757 * (-(locals.var_tx__blk781_dn0 / (2.0 * assign25610_e34760)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2__blk805_dn2 * locals.var_beta) / 2.0) * assign25610_e34761) + (assign25610_e34757 * (-(locals.var_tx__blk781_dn2 / (2.0 * assign25610_e34760)))))), (locals.var_vgpld_dn6 + ((((locals.var_fac1p2__blk805_dn6 * locals.var_beta) / 2.0) * assign25610_e34761) + (assign25610_e34757 * (-(locals.var_tx__blk781_dn6 / (2.0 * assign25610_e34760)))))), (locals.var_vgpld_dn7 + ((((locals.var_fac1p2__blk805_dn7 * locals.var_beta) / 2.0) * assign25610_e34761) + (assign25610_e34757 * (-(locals.var_tx__blk781_dn7 / (2.0 * assign25610_e34760)))))), (locals.var_vgpld_dn10 + (((((locals.var_fac1p2__blk805_dn10 * locals.var_beta) + (locals.var_fac1p2__blk805 * locals.var_beta_dn10)) / 2.0) * assign25610_e34761) + (assign25610_e34757 * (-(locals.var_tx__blk781_dn10 / (2.0 * assign25610_e34760)))))), (locals.var_vgpld_dn11 + ((((locals.var_fac1p2__blk805_dn11 * locals.var_beta) / 2.0) * assign25610_e34761) + (assign25610_e34757 * (-(locals.var_tx__blk781_dn11 / (2.0 * assign25610_e34760)))))), (locals.var_vgpld_dn12 + ((((locals.var_fac1p2__blk805_dn12 * locals.var_beta) / 2.0) * assign25610_e34761) + (assign25610_e34757 * (-(locals.var_tx__blk781_dn12 / (2.0 * assign25610_e34760)))))), (locals.var_vgpld_dn17 + ((((locals.var_fac1p2__blk805_dn17 * locals.var_beta) / 2.0) * assign25610_e34761) + (assign25610_e34757 * (-(locals.var_tx__blk781_dn17 / (2.0 * assign25610_e34760)))))),)
    } else {
        (locals.var_ps0_inia__blk821, locals.var_ps0_inia__blk821_dn0, locals.var_ps0_inia__blk821_dn2, locals.var_ps0_inia__blk821_dn6, locals.var_ps0_inia__blk821_dn7, locals.var_ps0_inia__blk821_dn10, locals.var_ps0_inia__blk821_dn11, locals.var_ps0_inia__blk821_dn12, locals.var_ps0_inia__blk821_dn17,)
    }
};
        locals.var_ps0_inia__blk821 = assign25610_e34765;
        locals.var_ps0_inia__blk821_dn0 = assign25610_e34765_d_n0;
        locals.var_ps0_inia__blk821_dn2 = assign25610_e34765_d_n2;
        locals.var_ps0_inia__blk821_dn6 = assign25610_e34765_d_n6;
        locals.var_ps0_inia__blk821_dn7 = assign25610_e34765_d_n7;
        locals.var_ps0_inia__blk821_dn10 = assign25610_e34765_d_n10;
        locals.var_ps0_inia__blk821_dn11 = assign25610_e34765_d_n11;
        locals.var_ps0_inia__blk821_dn12 = assign25610_e34765_d_n12;
        locals.var_ps0_inia__blk821_dn17 = assign25610_e34765_d_n17;
        locals.var_ps0_inia__blk821_rv = 0.0;

        let (assign25620_e34780, assign25620_e34780_d_n0, assign25620_e34780_d_n2, assign25620_e34780_d_n6, assign25620_e34780_d_n7, assign25620_e34780_d_n10, assign25620_e34780_d_n11, assign25620_e34780_d_n12, assign25620_e34780_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25620_e34777: f64 = (locals.var_ps0_inia__blk821 + locals.var_vxbgmtcl);
        let assign25620_e34778: f64 = (locals.var_beta * assign25620_e34777);
        (assign25620_e34778, (locals.var_beta * (locals.var_ps0_inia__blk821_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign25620_e34777) + (locals.var_beta * (locals.var_ps0_inia__blk821_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk821_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk818, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    }
};
        locals.var_chi__blk818 = assign25620_e34780;
        locals.var_chi__blk818_dn0 = assign25620_e34780_d_n0;
        locals.var_chi__blk818_dn2 = assign25620_e34780_d_n2;
        locals.var_chi__blk818_dn6 = assign25620_e34780_d_n6;
        locals.var_chi__blk818_dn7 = assign25620_e34780_d_n7;
        locals.var_chi__blk818_dn10 = assign25620_e34780_d_n10;
        locals.var_chi__blk818_dn11 = assign25620_e34780_d_n11;
        locals.var_chi__blk818_dn12 = assign25620_e34780_d_n12;
        locals.var_chi__blk818_dn17 = assign25620_e34780_d_n17;
        locals.var_chi__blk818_rv = 0.0;

        let (assign25630_e34793, assign25630_e34793_d_n0, assign25630_e34793_d_n2, assign25630_e34793_d_n6, assign25630_e34793_d_n7, assign25630_e34793_d_n10, assign25630_e34793_d_n11, assign25630_e34793_d_n12, assign25630_e34793_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25630_e34790: f64 = (-locals.var_chi__blk818);
        let assign25630_e34791: f64 = (assign25630_e34790).exp();
        (assign25630_e34791, (assign25630_e34791 * (-locals.var_chi__blk818_dn0)), (assign25630_e34791 * (-locals.var_chi__blk818_dn2)), (assign25630_e34791 * (-locals.var_chi__blk818_dn6)), (assign25630_e34791 * (-locals.var_chi__blk818_dn7)), (assign25630_e34791 * (-locals.var_chi__blk818_dn10)), (assign25630_e34791 * (-locals.var_chi__blk818_dn11)), (assign25630_e34791 * (-locals.var_chi__blk818_dn12)), (assign25630_e34791 * (-locals.var_chi__blk818_dn17)),)
    } else {
        (locals.var_ty__blk782, locals.var_ty__blk782_dn0, locals.var_ty__blk782_dn2, locals.var_ty__blk782_dn6, locals.var_ty__blk782_dn7, locals.var_ty__blk782_dn10, locals.var_ty__blk782_dn11, locals.var_ty__blk782_dn12, locals.var_ty__blk782_dn17,)
    }
};
        locals.var_ty__blk782 = assign25630_e34793;
        locals.var_ty__blk782_dn0 = assign25630_e34793_d_n0;
        locals.var_ty__blk782_dn2 = assign25630_e34793_d_n2;
        locals.var_ty__blk782_dn6 = assign25630_e34793_d_n6;
        locals.var_ty__blk782_dn7 = assign25630_e34793_d_n7;
        locals.var_ty__blk782_dn10 = assign25630_e34793_d_n10;
        locals.var_ty__blk782_dn11 = assign25630_e34793_d_n11;
        locals.var_ty__blk782_dn12 = assign25630_e34793_d_n12;
        locals.var_ty__blk782_dn17 = assign25630_e34793_d_n17;
        locals.var_ty__blk782_rv = 0.0;

        let (assign25640_e34820, assign25640_e34820_d_n0, assign25640_e34820_d_n2, assign25640_e34820_d_n6, assign25640_e34820_d_n7, assign25640_e34820_d_n10, assign25640_e34820_d_n11, assign25640_e34820_d_n12, assign25640_e34820_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25640_e34807: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign25640_e34808: f64 = (locals.var_beta * assign25640_e34807);
        let assign25640_e34810: f64 = (assign25640_e34808 - 1.0);
        let assign25640_e34812: f64 = (assign25640_e34810 + locals.var_ty__blk782);
        let assign25640_e34813: f64 = (4.0 * assign25640_e34812);
        let assign25640_e34816: f64 = (locals.var_fac1p2__blk805 * locals.var_beta2);
        let assign25640_e34817: f64 = (assign25640_e34813 / assign25640_e34816);
        let assign25640_e34818: f64 = (1.0 + assign25640_e34817);
        (assign25640_e34818, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + locals.var_ty__blk782_dn0)) * assign25640_e34816) - (assign25640_e34813 * (locals.var_fac1p2__blk805_dn0 * locals.var_beta2))) / (assign25640_e34816 * assign25640_e34816)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + locals.var_ty__blk782_dn2)) * assign25640_e34816) - (assign25640_e34813 * (locals.var_fac1p2__blk805_dn2 * locals.var_beta2))) / (assign25640_e34816 * assign25640_e34816)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) + locals.var_ty__blk782_dn6)) * assign25640_e34816) - (assign25640_e34813 * (locals.var_fac1p2__blk805_dn6 * locals.var_beta2))) / (assign25640_e34816 * assign25640_e34816)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) + locals.var_ty__blk782_dn7)) * assign25640_e34816) - (assign25640_e34813 * (locals.var_fac1p2__blk805_dn7 * locals.var_beta2))) / (assign25640_e34816 * assign25640_e34816)), ((((4.0 * (((locals.var_beta_dn10 * assign25640_e34807) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))) + locals.var_ty__blk782_dn10)) * assign25640_e34816) - (assign25640_e34813 * ((locals.var_fac1p2__blk805_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk805 * locals.var_beta2_dn10)))) / (assign25640_e34816 * assign25640_e34816)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) + locals.var_ty__blk782_dn11)) * assign25640_e34816) - (assign25640_e34813 * (locals.var_fac1p2__blk805_dn11 * locals.var_beta2))) / (assign25640_e34816 * assign25640_e34816)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) + locals.var_ty__blk782_dn12)) * assign25640_e34816) - (assign25640_e34813 * (locals.var_fac1p2__blk805_dn12 * locals.var_beta2))) / (assign25640_e34816 * assign25640_e34816)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) + locals.var_ty__blk782_dn17)) * assign25640_e34816) - (assign25640_e34813 * (locals.var_fac1p2__blk805_dn17 * locals.var_beta2))) / (assign25640_e34816 * assign25640_e34816)),)
    } else {
        (locals.var_tx__blk781, locals.var_tx__blk781_dn0, locals.var_tx__blk781_dn2, locals.var_tx__blk781_dn6, locals.var_tx__blk781_dn7, locals.var_tx__blk781_dn10, locals.var_tx__blk781_dn11, locals.var_tx__blk781_dn12, locals.var_tx__blk781_dn17,)
    }
};
        locals.var_tx__blk781 = assign25640_e34820;
        locals.var_tx__blk781_dn0 = assign25640_e34820_d_n0;
        locals.var_tx__blk781_dn2 = assign25640_e34820_d_n2;
        locals.var_tx__blk781_dn6 = assign25640_e34820_d_n6;
        locals.var_tx__blk781_dn7 = assign25640_e34820_d_n7;
        locals.var_tx__blk781_dn10 = assign25640_e34820_d_n10;
        locals.var_tx__blk781_dn11 = assign25640_e34820_d_n11;
        locals.var_tx__blk781_dn12 = assign25640_e34820_d_n12;
        locals.var_tx__blk781_dn17 = assign25640_e34820_d_n17;
        locals.var_tx__blk781_rv = 0.0;

        let assign25650_e34824: f64 = (10.0 * 2.220446049250313e-16);
        let assign25650_e34825: f64 = if locals.var_tx__blk781 < assign25650_e34824 { 1.0 } else { 0.0 };
        locals.var_guard860 = assign25650_e34825;
        locals.var_guard860_rv = 0.0;

        let (assign25660_e34840, assign25660_e34840_d_n0, assign25660_e34840_d_n2, assign25660_e34840_d_n6, assign25660_e34840_d_n7, assign25660_e34840_d_n10, assign25660_e34840_d_n11, assign25660_e34840_d_n12, assign25660_e34840_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard860 != 0.0)) {
        let assign25660_e34838: f64 = (10.0 * 2.220446049250313e-16);
        (assign25660_e34838, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk781, locals.var_tx__blk781_dn0, locals.var_tx__blk781_dn2, locals.var_tx__blk781_dn6, locals.var_tx__blk781_dn7, locals.var_tx__blk781_dn10, locals.var_tx__blk781_dn11, locals.var_tx__blk781_dn12, locals.var_tx__blk781_dn17,)
    }
};
        locals.var_tx__blk781 = assign25660_e34840;
        locals.var_tx__blk781_dn0 = assign25660_e34840_d_n0;
        locals.var_tx__blk781_dn2 = assign25660_e34840_d_n2;
        locals.var_tx__blk781_dn6 = assign25660_e34840_d_n6;
        locals.var_tx__blk781_dn7 = assign25660_e34840_d_n7;
        locals.var_tx__blk781_dn10 = assign25660_e34840_d_n10;
        locals.var_tx__blk781_dn11 = assign25660_e34840_d_n11;
        locals.var_tx__blk781_dn12 = assign25660_e34840_d_n12;
        locals.var_tx__blk781_dn17 = assign25660_e34840_d_n17;
        locals.var_tx__blk781_rv = 0.0;

        let (assign25670_e34862, assign25670_e34862_d_n0, assign25670_e34862_d_n2, assign25670_e34862_d_n6, assign25670_e34862_d_n7, assign25670_e34862_d_n10, assign25670_e34862_d_n11, assign25670_e34862_d_n12, assign25670_e34862_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25670_e34852: f64 = (locals.var_fac1p2__blk805 * locals.var_beta);
        let assign25670_e34854: f64 = (assign25670_e34852 / 2.0);
        let assign25670_e34857: f64 = (locals.var_tx__blk781).sqrt();
        let assign25670_e34858: f64 = (1.0 - assign25670_e34857);
        let assign25670_e34859: f64 = (assign25670_e34854 * assign25670_e34858);
        let assign25670_e34860: f64 = (locals.var_vgpld + assign25670_e34859);
        (assign25670_e34860, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2__blk805_dn0 * locals.var_beta) / 2.0) * assign25670_e34858) + (assign25670_e34854 * (-(locals.var_tx__blk781_dn0 / (2.0 * assign25670_e34857)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2__blk805_dn2 * locals.var_beta) / 2.0) * assign25670_e34858) + (assign25670_e34854 * (-(locals.var_tx__blk781_dn2 / (2.0 * assign25670_e34857)))))), (locals.var_vgpld_dn6 + ((((locals.var_fac1p2__blk805_dn6 * locals.var_beta) / 2.0) * assign25670_e34858) + (assign25670_e34854 * (-(locals.var_tx__blk781_dn6 / (2.0 * assign25670_e34857)))))), (locals.var_vgpld_dn7 + ((((locals.var_fac1p2__blk805_dn7 * locals.var_beta) / 2.0) * assign25670_e34858) + (assign25670_e34854 * (-(locals.var_tx__blk781_dn7 / (2.0 * assign25670_e34857)))))), (locals.var_vgpld_dn10 + (((((locals.var_fac1p2__blk805_dn10 * locals.var_beta) + (locals.var_fac1p2__blk805 * locals.var_beta_dn10)) / 2.0) * assign25670_e34858) + (assign25670_e34854 * (-(locals.var_tx__blk781_dn10 / (2.0 * assign25670_e34857)))))), (locals.var_vgpld_dn11 + ((((locals.var_fac1p2__blk805_dn11 * locals.var_beta) / 2.0) * assign25670_e34858) + (assign25670_e34854 * (-(locals.var_tx__blk781_dn11 / (2.0 * assign25670_e34857)))))), (locals.var_vgpld_dn12 + ((((locals.var_fac1p2__blk805_dn12 * locals.var_beta) / 2.0) * assign25670_e34858) + (assign25670_e34854 * (-(locals.var_tx__blk781_dn12 / (2.0 * assign25670_e34857)))))), (locals.var_vgpld_dn17 + ((((locals.var_fac1p2__blk805_dn17 * locals.var_beta) / 2.0) * assign25670_e34858) + (assign25670_e34854 * (-(locals.var_tx__blk781_dn17 / (2.0 * assign25670_e34857)))))),)
    } else {
        (locals.var_ps0_inia__blk821, locals.var_ps0_inia__blk821_dn0, locals.var_ps0_inia__blk821_dn2, locals.var_ps0_inia__blk821_dn6, locals.var_ps0_inia__blk821_dn7, locals.var_ps0_inia__blk821_dn10, locals.var_ps0_inia__blk821_dn11, locals.var_ps0_inia__blk821_dn12, locals.var_ps0_inia__blk821_dn17,)
    }
};
        locals.var_ps0_inia__blk821 = assign25670_e34862;
        locals.var_ps0_inia__blk821_dn0 = assign25670_e34862_d_n0;
        locals.var_ps0_inia__blk821_dn2 = assign25670_e34862_d_n2;
        locals.var_ps0_inia__blk821_dn6 = assign25670_e34862_d_n6;
        locals.var_ps0_inia__blk821_dn7 = assign25670_e34862_d_n7;
        locals.var_ps0_inia__blk821_dn10 = assign25670_e34862_d_n10;
        locals.var_ps0_inia__blk821_dn11 = assign25670_e34862_d_n11;
        locals.var_ps0_inia__blk821_dn12 = assign25670_e34862_d_n12;
        locals.var_ps0_inia__blk821_dn17 = assign25670_e34862_d_n17;
        locals.var_ps0_inia__blk821_rv = 0.0;

        let (assign25680_e34877, assign25680_e34877_d_n0, assign25680_e34877_d_n2, assign25680_e34877_d_n6, assign25680_e34877_d_n7, assign25680_e34877_d_n10, assign25680_e34877_d_n11, assign25680_e34877_d_n12, assign25680_e34877_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25680_e34874: f64 = (locals.var_ps0_inia__blk821 + locals.var_vxbgmtcl);
        let assign25680_e34875: f64 = (locals.var_beta * assign25680_e34874);
        (assign25680_e34875, (locals.var_beta * (locals.var_ps0_inia__blk821_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign25680_e34874) + (locals.var_beta * (locals.var_ps0_inia__blk821_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk821_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk818, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    }
};
        locals.var_chi__blk818 = assign25680_e34877;
        locals.var_chi__blk818_dn0 = assign25680_e34877_d_n0;
        locals.var_chi__blk818_dn2 = assign25680_e34877_d_n2;
        locals.var_chi__blk818_dn6 = assign25680_e34877_d_n6;
        locals.var_chi__blk818_dn7 = assign25680_e34877_d_n7;
        locals.var_chi__blk818_dn10 = assign25680_e34877_d_n10;
        locals.var_chi__blk818_dn11 = assign25680_e34877_d_n11;
        locals.var_chi__blk818_dn12 = assign25680_e34877_d_n12;
        locals.var_chi__blk818_dn17 = assign25680_e34877_d_n17;
        locals.var_chi__blk818_rv = 0.0;

        let assign25690_e34880: f64 = if locals.var_chi__blk818 < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard861 = assign25690_e34880;
        locals.var_guard861_rv = 0.0;

        let (assign25710_e34923,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign25710_e34907: f64 = (9.0 * 1.414213562373095);
        let assign25710_e34908: f64 = (1.0 / assign25710_e34907);
        let assign25710_e34912: f64 = (7.0 * 0.049787068367863944);
        let assign25710_e34913: f64 = (5.0 + assign25710_e34912);
        let assign25710_e34917: f64 = (2.0 + 0.049787068367863944);
        let assign25710_e34918: f64 = (assign25710_e34917).sqrt();
        let assign25710_e34919: f64 = (54.0 * assign25710_e34918);
        let assign25710_e34920: f64 = (assign25710_e34913 / assign25710_e34919);
        let assign25710_e34921: f64 = (assign25710_e34908 - assign25710_e34920);
        (assign25710_e34921,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign25710_e34923;
        locals.var_ta_rv = 0.0;

        let (assign25720_e34949,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign25720_e34936: f64 = (1.0 + 0.049787068367863944);
        let assign25720_e34940: f64 = (2.0 + 0.049787068367863944);
        let assign25720_e34941: f64 = (assign25720_e34940).sqrt();
        let assign25720_e34942: f64 = (2.0 * assign25720_e34941);
        let assign25720_e34943: f64 = (assign25720_e34936 / assign25720_e34942);
        let assign25720_e34946: f64 = (1.414213562373095 / 3.0);
        let assign25720_e34947: f64 = (assign25720_e34943 - assign25720_e34946);
        (assign25720_e34947,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign25720_e34949;
        locals.var_tb_rv = 0.0;

        let (assign25730_e34970, assign25730_e34970_d_n0, assign25730_e34970_d_n2, assign25730_e34970_d_n6, assign25730_e34970_d_n7, assign25730_e34970_d_n10, assign25730_e34970_d_n11, assign25730_e34970_d_n12, assign25730_e34970_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign25730_e34962: f64 = (1.0 / 1.414213562373095);
        let assign25730_e34966: f64 = (locals.var_beta * locals.var_fac1__blk804);
        let assign25730_e34967: f64 = (1.0 / assign25730_e34966);
        let assign25730_e34968: f64 = (assign25730_e34962 + assign25730_e34967);
        (assign25730_e34968, (-((locals.var_beta * locals.var_fac1__blk804_dn0) / (assign25730_e34966 * assign25730_e34966))), (-((locals.var_beta * locals.var_fac1__blk804_dn2) / (assign25730_e34966 * assign25730_e34966))), (-((locals.var_beta * locals.var_fac1__blk804_dn6) / (assign25730_e34966 * assign25730_e34966))), (-((locals.var_beta * locals.var_fac1__blk804_dn7) / (assign25730_e34966 * assign25730_e34966))), (-(((locals.var_beta_dn10 * locals.var_fac1__blk804) + (locals.var_beta * locals.var_fac1__blk804_dn10)) / (assign25730_e34966 * assign25730_e34966))), (-((locals.var_beta * locals.var_fac1__blk804_dn11) / (assign25730_e34966 * assign25730_e34966))), (-((locals.var_beta * locals.var_fac1__blk804_dn12) / (assign25730_e34966 * assign25730_e34966))), (-((locals.var_beta * locals.var_fac1__blk804_dn17) / (assign25730_e34966 * assign25730_e34966))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn12, locals.var_tc_dn17,)
    }
};
        locals.var_tc = assign25730_e34970;
        locals.var_tc_dn0 = assign25730_e34970_d_n0;
        locals.var_tc_dn2 = assign25730_e34970_d_n2;
        locals.var_tc_dn6 = assign25730_e34970_d_n6;
        locals.var_tc_dn7 = assign25730_e34970_d_n7;
        locals.var_tc_dn10 = assign25730_e34970_d_n10;
        locals.var_tc_dn11 = assign25730_e34970_d_n11;
        locals.var_tc_dn12 = assign25730_e34970_d_n12;
        locals.var_tc_dn17 = assign25730_e34970_d_n17;
        locals.var_tc_rv = 0.0;

        let (assign25740_e34988, assign25740_e34988_d_n0, assign25740_e34988_d_n2, assign25740_e34988_d_n6, assign25740_e34988_d_n7, assign25740_e34988_d_n10, assign25740_e34988_d_n11, assign25740_e34988_d_n12, assign25740_e34988_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign25740_e34983: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign25740_e34984: f64 = (-assign25740_e34983);
        let assign25740_e34986: f64 = (assign25740_e34984 / locals.var_fac1__blk804);
        (assign25740_e34986, ((((-(locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) * locals.var_fac1__blk804) - (assign25740_e34984 * locals.var_fac1__blk804_dn0)) / (locals.var_fac1__blk804 * locals.var_fac1__blk804)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1__blk804) - (assign25740_e34984 * locals.var_fac1__blk804_dn2)) / (locals.var_fac1__blk804 * locals.var_fac1__blk804)), ((((-(locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) * locals.var_fac1__blk804) - (assign25740_e34984 * locals.var_fac1__blk804_dn6)) / (locals.var_fac1__blk804 * locals.var_fac1__blk804)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1__blk804) - (assign25740_e34984 * locals.var_fac1__blk804_dn7)) / (locals.var_fac1__blk804 * locals.var_fac1__blk804)), ((((-(locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10)) * locals.var_fac1__blk804) - (assign25740_e34984 * locals.var_fac1__blk804_dn10)) / (locals.var_fac1__blk804 * locals.var_fac1__blk804)), ((((-(locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) * locals.var_fac1__blk804) - (assign25740_e34984 * locals.var_fac1__blk804_dn11)) / (locals.var_fac1__blk804 * locals.var_fac1__blk804)), ((((-(locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) * locals.var_fac1__blk804) - (assign25740_e34984 * locals.var_fac1__blk804_dn12)) / (locals.var_fac1__blk804 * locals.var_fac1__blk804)), ((((-(locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) * locals.var_fac1__blk804) - (assign25740_e34984 * locals.var_fac1__blk804_dn17)) / (locals.var_fac1__blk804 * locals.var_fac1__blk804)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn12, locals.var_td_dn17,)
    }
};
        locals.var_td = assign25740_e34988;
        locals.var_td_dn0 = assign25740_e34988_d_n0;
        locals.var_td_dn2 = assign25740_e34988_d_n2;
        locals.var_td_dn6 = assign25740_e34988_d_n6;
        locals.var_td_dn7 = assign25740_e34988_d_n7;
        locals.var_td_dn10 = assign25740_e34988_d_n10;
        locals.var_td_dn11 = assign25740_e34988_d_n11;
        locals.var_td_dn12 = assign25740_e34988_d_n12;
        locals.var_td_dn17 = assign25740_e34988_d_n17;
        locals.var_td_rv = 0.0;

        let (assign25750_e35029, assign25750_e35029_d_n0, assign25750_e35029_d_n2, assign25750_e35029_d_n6, assign25750_e35029_d_n7, assign25750_e35029_d_n10, assign25750_e35029_d_n11, assign25750_e35029_d_n12, assign25750_e35029_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign25750_e35001: f64 = (locals.var_tb * locals.var_tb);
        let assign25750_e35003: f64 = (assign25750_e35001 * locals.var_tb);
        let assign25750_e35006: f64 = (27.0 * locals.var_ta);
        let assign25750_e35008: f64 = (assign25750_e35006 * locals.var_ta);
        let assign25750_e35010: f64 = (assign25750_e35008 * locals.var_ta);
        let assign25750_e35011: f64 = (assign25750_e35003 / assign25750_e35010);
        let assign25750_e35014: f64 = (locals.var_tb * locals.var_tc);
        let assign25750_e35017: f64 = (6.0 * locals.var_ta);
        let assign25750_e35019: f64 = (assign25750_e35017 * locals.var_ta);
        let assign25750_e35020: f64 = (assign25750_e35014 / assign25750_e35019);
        let assign25750_e35021: f64 = (assign25750_e35011 - assign25750_e35020);
        let assign25750_e35025: f64 = (2.0 * locals.var_ta);
        let assign25750_e35026: f64 = (locals.var_td / assign25750_e35025);
        let assign25750_e35027: f64 = (assign25750_e35021 + assign25750_e35026);
        (assign25750_e35027, ((-((locals.var_tb * locals.var_tc_dn0) / assign25750_e35019)) + (locals.var_td_dn0 / assign25750_e35025)), ((-((locals.var_tb * locals.var_tc_dn2) / assign25750_e35019)) + (locals.var_td_dn2 / assign25750_e35025)), ((-((locals.var_tb * locals.var_tc_dn6) / assign25750_e35019)) + (locals.var_td_dn6 / assign25750_e35025)), ((-((locals.var_tb * locals.var_tc_dn7) / assign25750_e35019)) + (locals.var_td_dn7 / assign25750_e35025)), ((-((locals.var_tb * locals.var_tc_dn10) / assign25750_e35019)) + (locals.var_td_dn10 / assign25750_e35025)), ((-((locals.var_tb * locals.var_tc_dn11) / assign25750_e35019)) + (locals.var_td_dn11 / assign25750_e35025)), ((-((locals.var_tb * locals.var_tc_dn12) / assign25750_e35019)) + (locals.var_td_dn12 / assign25750_e35025)), ((-((locals.var_tb * locals.var_tc_dn17) / assign25750_e35019)) + (locals.var_td_dn17 / assign25750_e35025)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn12, locals.var_tq_dn17,)
    }
};
        locals.var_tq = assign25750_e35029;
        locals.var_tq_dn0 = assign25750_e35029_d_n0;
        locals.var_tq_dn2 = assign25750_e35029_d_n2;
        locals.var_tq_dn6 = assign25750_e35029_d_n6;
        locals.var_tq_dn7 = assign25750_e35029_d_n7;
        locals.var_tq_dn10 = assign25750_e35029_d_n10;
        locals.var_tq_dn11 = assign25750_e35029_d_n11;
        locals.var_tq_dn12 = assign25750_e35029_d_n12;
        locals.var_tq_dn17 = assign25750_e35029_d_n17;
        locals.var_tq_rv = 0.0;

        let (assign25760_e35056, assign25760_e35056_d_n0, assign25760_e35056_d_n2, assign25760_e35056_d_n6, assign25760_e35056_d_n7, assign25760_e35056_d_n10, assign25760_e35056_d_n11, assign25760_e35056_d_n12, assign25760_e35056_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign25760_e35042: f64 = (3.0 * locals.var_ta);
        let assign25760_e35044: f64 = (assign25760_e35042 * locals.var_tc);
        let assign25760_e35047: f64 = (locals.var_tb * locals.var_tb);
        let assign25760_e35048: f64 = (assign25760_e35044 - assign25760_e35047);
        let assign25760_e35051: f64 = (9.0 * locals.var_ta);
        let assign25760_e35053: f64 = (assign25760_e35051 * locals.var_ta);
        let assign25760_e35054: f64 = (assign25760_e35048 / assign25760_e35053);
        (assign25760_e35054, ((assign25760_e35042 * locals.var_tc_dn0) / assign25760_e35053), ((assign25760_e35042 * locals.var_tc_dn2) / assign25760_e35053), ((assign25760_e35042 * locals.var_tc_dn6) / assign25760_e35053), ((assign25760_e35042 * locals.var_tc_dn7) / assign25760_e35053), ((assign25760_e35042 * locals.var_tc_dn10) / assign25760_e35053), ((assign25760_e35042 * locals.var_tc_dn11) / assign25760_e35053), ((assign25760_e35042 * locals.var_tc_dn12) / assign25760_e35053), ((assign25760_e35042 * locals.var_tc_dn17) / assign25760_e35053),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn12, locals.var_tp_dn17,)
    }
};
        locals.var_tp = assign25760_e35056;
        locals.var_tp_dn0 = assign25760_e35056_d_n0;
        locals.var_tp_dn2 = assign25760_e35056_d_n2;
        locals.var_tp_dn6 = assign25760_e35056_d_n6;
        locals.var_tp_dn7 = assign25760_e35056_d_n7;
        locals.var_tp_dn10 = assign25760_e35056_d_n10;
        locals.var_tp_dn11 = assign25760_e35056_d_n11;
        locals.var_tp_dn12 = assign25760_e35056_d_n12;
        locals.var_tp_dn17 = assign25760_e35056_d_n17;
        locals.var_tp_rv = 0.0;

        let (assign25770_e35078, assign25770_e35078_d_n0, assign25770_e35078_d_n2, assign25770_e35078_d_n6, assign25770_e35078_d_n7, assign25770_e35078_d_n10, assign25770_e35078_d_n11, assign25770_e35078_d_n12, assign25770_e35078_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign25770_e35069: f64 = (locals.var_tq * locals.var_tq);
        let assign25770_e35072: f64 = (locals.var_tp * locals.var_tp);
        let assign25770_e35074: f64 = (assign25770_e35072 * locals.var_tp);
        let assign25770_e35075: f64 = (assign25770_e35069 + assign25770_e35074);
        let assign25770_e35076: f64 = (assign25770_e35075).sqrt();
        (assign25770_e35076, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign25770_e35072 * locals.var_tp_dn0))) / (2.0 * assign25770_e35076)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign25770_e35072 * locals.var_tp_dn2))) / (2.0 * assign25770_e35076)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign25770_e35072 * locals.var_tp_dn6))) / (2.0 * assign25770_e35076)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign25770_e35072 * locals.var_tp_dn7))) / (2.0 * assign25770_e35076)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign25770_e35072 * locals.var_tp_dn10))) / (2.0 * assign25770_e35076)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign25770_e35072 * locals.var_tp_dn11))) / (2.0 * assign25770_e35076)), ((((locals.var_tq_dn12 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn12)) + ((((locals.var_tp_dn12 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn12)) * locals.var_tp) + (assign25770_e35072 * locals.var_tp_dn12))) / (2.0 * assign25770_e35076)), ((((locals.var_tq_dn17 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn17)) + ((((locals.var_tp_dn17 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn17)) * locals.var_tp) + (assign25770_e35072 * locals.var_tp_dn17))) / (2.0 * assign25770_e35076)),)
    } else {
        (locals.var_t5__blk778, locals.var_t5__blk778_dn0, locals.var_t5__blk778_dn2, locals.var_t5__blk778_dn6, locals.var_t5__blk778_dn7, locals.var_t5__blk778_dn10, locals.var_t5__blk778_dn11, locals.var_t5__blk778_dn12, locals.var_t5__blk778_dn17,)
    }
};
        locals.var_t5__blk778 = assign25770_e35078;
        locals.var_t5__blk778_dn0 = assign25770_e35078_d_n0;
        locals.var_t5__blk778_dn2 = assign25770_e35078_d_n2;
        locals.var_t5__blk778_dn6 = assign25770_e35078_d_n6;
        locals.var_t5__blk778_dn7 = assign25770_e35078_d_n7;
        locals.var_t5__blk778_dn10 = assign25770_e35078_d_n10;
        locals.var_t5__blk778_dn11 = assign25770_e35078_d_n11;
        locals.var_t5__blk778_dn12 = assign25770_e35078_d_n12;
        locals.var_t5__blk778_dn17 = assign25770_e35078_d_n17;
        locals.var_t5__blk778_rv = 0.0;

        let (assign25780_e35096, assign25780_e35096_d_n0, assign25780_e35096_d_n2, assign25780_e35096_d_n6, assign25780_e35096_d_n7, assign25780_e35096_d_n10, assign25780_e35096_d_n11, assign25780_e35096_d_n12, assign25780_e35096_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign25780_e35090: f64 = (-locals.var_tq);
        let assign25780_e35092: f64 = (assign25780_e35090 + locals.var_t5__blk778);
        let assign25780_e35094: f64 = (assign25780_e35092).powf(0.3333333333333333);
        (assign25780_e35094, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25780_e35092).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5__blk778_dn0))) } } else { (assign25780_e35094 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5__blk778_dn0) / assign25780_e35092))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25780_e35092).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5__blk778_dn2))) } } else { (assign25780_e35094 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5__blk778_dn2) / assign25780_e35092))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25780_e35092).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5__blk778_dn6))) } } else { (assign25780_e35094 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5__blk778_dn6) / assign25780_e35092))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25780_e35092).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5__blk778_dn7))) } } else { (assign25780_e35094 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5__blk778_dn7) / assign25780_e35092))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25780_e35092).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5__blk778_dn10))) } } else { (assign25780_e35094 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5__blk778_dn10) / assign25780_e35092))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25780_e35092).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5__blk778_dn11))) } } else { (assign25780_e35094 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5__blk778_dn11) / assign25780_e35092))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25780_e35092).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn12) + locals.var_t5__blk778_dn12))) } } else { (assign25780_e35094 * (0.3333333333333333 * (((-locals.var_tq_dn12) + locals.var_t5__blk778_dn12) / assign25780_e35092))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25780_e35092).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn17) + locals.var_t5__blk778_dn17))) } } else { (assign25780_e35094 * (0.3333333333333333 * (((-locals.var_tq_dn17) + locals.var_t5__blk778_dn17) / assign25780_e35092))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn12, locals.var_tu_dn17,)
    }
};
        locals.var_tu = assign25780_e35096;
        locals.var_tu_dn0 = assign25780_e35096_d_n0;
        locals.var_tu_dn2 = assign25780_e35096_d_n2;
        locals.var_tu_dn6 = assign25780_e35096_d_n6;
        locals.var_tu_dn7 = assign25780_e35096_d_n7;
        locals.var_tu_dn10 = assign25780_e35096_d_n10;
        locals.var_tu_dn11 = assign25780_e35096_d_n11;
        locals.var_tu_dn12 = assign25780_e35096_d_n12;
        locals.var_tu_dn17 = assign25780_e35096_d_n17;
        locals.var_tu_rv = 0.0;

        let (assign25790_e35114, assign25790_e35114_d_n0, assign25790_e35114_d_n2, assign25790_e35114_d_n6, assign25790_e35114_d_n7, assign25790_e35114_d_n10, assign25790_e35114_d_n11, assign25790_e35114_d_n12, assign25790_e35114_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign25790_e35109: f64 = (locals.var_tq + locals.var_t5__blk778);
        let assign25790_e35111: f64 = (assign25790_e35109).powf(0.3333333333333333);
        let assign25790_e35112: f64 = (-assign25790_e35111);
        (assign25790_e35112, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25790_e35109).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5__blk778_dn0))) } } else { (assign25790_e35111 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5__blk778_dn0) / assign25790_e35109))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25790_e35109).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5__blk778_dn2))) } } else { (assign25790_e35111 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5__blk778_dn2) / assign25790_e35109))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25790_e35109).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5__blk778_dn6))) } } else { (assign25790_e35111 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5__blk778_dn6) / assign25790_e35109))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25790_e35109).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5__blk778_dn7))) } } else { (assign25790_e35111 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5__blk778_dn7) / assign25790_e35109))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25790_e35109).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5__blk778_dn10))) } } else { (assign25790_e35111 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5__blk778_dn10) / assign25790_e35109))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25790_e35109).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5__blk778_dn11))) } } else { (assign25790_e35111 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5__blk778_dn11) / assign25790_e35109))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25790_e35109).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn12 + locals.var_t5__blk778_dn12))) } } else { (assign25790_e35111 * (0.3333333333333333 * ((locals.var_tq_dn12 + locals.var_t5__blk778_dn12) / assign25790_e35109))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign25790_e35109).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn17 + locals.var_t5__blk778_dn17))) } } else { (assign25790_e35111 * (0.3333333333333333 * ((locals.var_tq_dn17 + locals.var_t5__blk778_dn17) / assign25790_e35109))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn12, locals.var_tv_dn17,)
    }
};
        locals.var_tv = assign25790_e35114;
        locals.var_tv_dn0 = assign25790_e35114_d_n0;
        locals.var_tv_dn2 = assign25790_e35114_d_n2;
        locals.var_tv_dn6 = assign25790_e35114_d_n6;
        locals.var_tv_dn7 = assign25790_e35114_d_n7;
        locals.var_tv_dn10 = assign25790_e35114_d_n10;
        locals.var_tv_dn11 = assign25790_e35114_d_n11;
        locals.var_tv_dn12 = assign25790_e35114_d_n12;
        locals.var_tv_dn17 = assign25790_e35114_d_n17;
        locals.var_tv_rv = 0.0;

        let (assign25800_e35135, assign25800_e35135_d_n0, assign25800_e35135_d_n2, assign25800_e35135_d_n6, assign25800_e35135_d_n7, assign25800_e35135_d_n10, assign25800_e35135_d_n11, assign25800_e35135_d_n12, assign25800_e35135_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign25800_e35127: f64 = (locals.var_tu + locals.var_tv);
        let assign25800_e35131: f64 = (3.0 * locals.var_ta);
        let assign25800_e35132: f64 = (locals.var_tb / assign25800_e35131);
        let assign25800_e35133: f64 = (assign25800_e35127 - assign25800_e35132);
        (assign25800_e35133, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn12 + locals.var_tv_dn12), (locals.var_tu_dn17 + locals.var_tv_dn17),)
    } else {
        (locals.var_tx__blk781, locals.var_tx__blk781_dn0, locals.var_tx__blk781_dn2, locals.var_tx__blk781_dn6, locals.var_tx__blk781_dn7, locals.var_tx__blk781_dn10, locals.var_tx__blk781_dn11, locals.var_tx__blk781_dn12, locals.var_tx__blk781_dn17,)
    }
};
        locals.var_tx__blk781 = assign25800_e35135;
        locals.var_tx__blk781_dn0 = assign25800_e35135_d_n0;
        locals.var_tx__blk781_dn2 = assign25800_e35135_d_n2;
        locals.var_tx__blk781_dn6 = assign25800_e35135_d_n6;
        locals.var_tx__blk781_dn7 = assign25800_e35135_d_n7;
        locals.var_tx__blk781_dn10 = assign25800_e35135_d_n10;
        locals.var_tx__blk781_dn11 = assign25800_e35135_d_n11;
        locals.var_tx__blk781_dn12 = assign25800_e35135_d_n12;
        locals.var_tx__blk781_dn17 = assign25800_e35135_d_n17;
        locals.var_tx__blk781_rv = 0.0;

        let (assign25810_e35152, assign25810_e35152_d_n0, assign25810_e35152_d_n2, assign25810_e35152_d_n6, assign25810_e35152_d_n7, assign25810_e35152_d_n10, assign25810_e35152_d_n11, assign25810_e35152_d_n12, assign25810_e35152_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign25810_e35148: f64 = (locals.var_tx__blk781 * locals.var_beta_inv);
        let assign25810_e35150: f64 = (assign25810_e35148 - locals.var_vxbgmtcl);
        (assign25810_e35150, ((locals.var_tx__blk781_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_tx__blk781_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), ((locals.var_tx__blk781_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_tx__blk781_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn7), (((locals.var_tx__blk781_dn10 * locals.var_beta_inv) + (locals.var_tx__blk781 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), ((locals.var_tx__blk781_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_tx__blk781_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12), ((locals.var_tx__blk781_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0_inia__blk821, locals.var_ps0_inia__blk821_dn0, locals.var_ps0_inia__blk821_dn2, locals.var_ps0_inia__blk821_dn6, locals.var_ps0_inia__blk821_dn7, locals.var_ps0_inia__blk821_dn10, locals.var_ps0_inia__blk821_dn11, locals.var_ps0_inia__blk821_dn12, locals.var_ps0_inia__blk821_dn17,)
    }
};
        locals.var_ps0_inia__blk821 = assign25810_e35152;
        locals.var_ps0_inia__blk821_dn0 = assign25810_e35152_d_n0;
        locals.var_ps0_inia__blk821_dn2 = assign25810_e35152_d_n2;
        locals.var_ps0_inia__blk821_dn6 = assign25810_e35152_d_n6;
        locals.var_ps0_inia__blk821_dn7 = assign25810_e35152_d_n7;
        locals.var_ps0_inia__blk821_dn10 = assign25810_e35152_d_n10;
        locals.var_ps0_inia__blk821_dn11 = assign25810_e35152_d_n11;
        locals.var_ps0_inia__blk821_dn12 = assign25810_e35152_d_n12;
        locals.var_ps0_inia__blk821_dn17 = assign25810_e35152_d_n17;
        locals.var_ps0_inia__blk821_rv = 0.0;

        let (assign25820_e35169, assign25820_e35169_d_n0, assign25820_e35169_d_n2, assign25820_e35169_d_n6, assign25820_e35169_d_n7, assign25820_e35169_d_n10, assign25820_e35169_d_n11, assign25820_e35169_d_n12, assign25820_e35169_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard861 != 0.0)) {
        let assign25820_e35166: f64 = (locals.var_ps0_inia__blk821 + locals.var_vxbgmtcl);
        let assign25820_e35167: f64 = (locals.var_beta * assign25820_e35166);
        (assign25820_e35167, (locals.var_beta * (locals.var_ps0_inia__blk821_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign25820_e35166) + (locals.var_beta * (locals.var_ps0_inia__blk821_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk821_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk818, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    }
};
        locals.var_chi__blk818 = assign25820_e35169;
        locals.var_chi__blk818_dn0 = assign25820_e35169_d_n0;
        locals.var_chi__blk818_dn2 = assign25820_e35169_d_n2;
        locals.var_chi__blk818_dn6 = assign25820_e35169_d_n6;
        locals.var_chi__blk818_dn7 = assign25820_e35169_d_n7;
        locals.var_chi__blk818_dn10 = assign25820_e35169_d_n10;
        locals.var_chi__blk818_dn11 = assign25820_e35169_d_n11;
        locals.var_chi__blk818_dn12 = assign25820_e35169_d_n12;
        locals.var_chi__blk818_dn17 = assign25820_e35169_d_n17;
        locals.var_chi__blk818_rv = 0.0;

        let (assign25840_e35197, assign25840_e35197_d_n0, assign25840_e35197_d_n2, assign25840_e35197_d_n6, assign25840_e35197_d_n7, assign25840_e35197_d_n10, assign25840_e35197_d_n11, assign25840_e35197_d_n12, assign25840_e35197_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25840_e35193: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign25840_e35195: f64 = (assign25840_e35193 + 0.1);
        (assign25840_e35195, (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12), (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn12, locals.var_vgpld_shift_dn17,)
    }
};
        locals.var_vgpld_shift = assign25840_e35197;
        locals.var_vgpld_shift_dn0 = assign25840_e35197_d_n0;
        locals.var_vgpld_shift_dn2 = assign25840_e35197_d_n2;
        locals.var_vgpld_shift_dn6 = assign25840_e35197_d_n6;
        locals.var_vgpld_shift_dn7 = assign25840_e35197_d_n7;
        locals.var_vgpld_shift_dn10 = assign25840_e35197_d_n10;
        locals.var_vgpld_shift_dn11 = assign25840_e35197_d_n11;
        locals.var_vgpld_shift_dn12 = assign25840_e35197_d_n12;
        locals.var_vgpld_shift_dn17 = assign25840_e35197_d_n17;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign25850_e35214, assign25850_e35214_d_n0, assign25850_e35214_d_n2, assign25850_e35214_d_n6, assign25850_e35214_d_n7, assign25850_e35214_d_n10, assign25850_e35214_d_n11, assign25850_e35214_d_n12, assign25850_e35214_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25850_e35208: f64 = (-locals.var_vxbgmtcl);
        let assign25850_e35209: f64 = (locals.var_beta * assign25850_e35208);
        let assign25850_e35210: f64 = (assign25850_e35209).exp();
        let assign25850_e35212: f64 = (assign25850_e35210 + 1e-50);
        (assign25850_e35212, (assign25850_e35210 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign25850_e35210 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign25850_e35210 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign25850_e35210 * (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), (assign25850_e35210 * ((locals.var_beta_dn10 * assign25850_e35208) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign25850_e35210 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign25850_e35210 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))), (assign25850_e35210 * (locals.var_beta * (-locals.var_vxbgmtcl_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk837, locals.var_exp_bvbs__blk837_dn0, locals.var_exp_bvbs__blk837_dn2, locals.var_exp_bvbs__blk837_dn6, locals.var_exp_bvbs__blk837_dn7, locals.var_exp_bvbs__blk837_dn10, locals.var_exp_bvbs__blk837_dn11, locals.var_exp_bvbs__blk837_dn12, locals.var_exp_bvbs__blk837_dn17,)
    }
};
        locals.var_exp_bvbs__blk837 = assign25850_e35214;
        locals.var_exp_bvbs__blk837_dn0 = assign25850_e35214_d_n0;
        locals.var_exp_bvbs__blk837_dn2 = assign25850_e35214_d_n2;
        locals.var_exp_bvbs__blk837_dn6 = assign25850_e35214_d_n6;
        locals.var_exp_bvbs__blk837_dn7 = assign25850_e35214_d_n7;
        locals.var_exp_bvbs__blk837_dn10 = assign25850_e35214_d_n10;
        locals.var_exp_bvbs__blk837_dn11 = assign25850_e35214_d_n11;
        locals.var_exp_bvbs__blk837_dn12 = assign25850_e35214_d_n12;
        locals.var_exp_bvbs__blk837_dn17 = assign25850_e35214_d_n17;
        locals.var_exp_bvbs__blk837_rv = 0.0;

        let (assign25860_e35227, assign25860_e35227_d_n0, assign25860_e35227_d_n2, assign25860_e35227_d_n6, assign25860_e35227_d_n7, assign25860_e35227_d_n10, assign25860_e35227_d_n11, assign25860_e35227_d_n12, assign25860_e35227_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25860_e35225: f64 = (locals.var_nin / locals.var_uc_nsubbttub);
        (assign25860_e35225, (((locals.var_nin_dn0 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn0)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn2 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn2)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn6 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn6)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn7 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn7)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn10 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn10)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn11 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn11)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn12 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn12)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn17 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn17)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)),)
    } else {
        (locals.var_t0__blk774, locals.var_t0__blk774_dn0, locals.var_t0__blk774_dn2, locals.var_t0__blk774_dn6, locals.var_t0__blk774_dn7, locals.var_t0__blk774_dn10, locals.var_t0__blk774_dn11, locals.var_t0__blk774_dn12, locals.var_t0__blk774_dn17,)
    }
};
        locals.var_t0__blk774 = assign25860_e35227;
        locals.var_t0__blk774_dn0 = assign25860_e35227_d_n0;
        locals.var_t0__blk774_dn2 = assign25860_e35227_d_n2;
        locals.var_t0__blk774_dn6 = assign25860_e35227_d_n6;
        locals.var_t0__blk774_dn7 = assign25860_e35227_d_n7;
        locals.var_t0__blk774_dn10 = assign25860_e35227_d_n10;
        locals.var_t0__blk774_dn11 = assign25860_e35227_d_n11;
        locals.var_t0__blk774_dn12 = assign25860_e35227_d_n12;
        locals.var_t0__blk774_dn17 = assign25860_e35227_d_n17;
        locals.var_t0__blk774_rv = 0.0;

        let (assign25870_e35240, assign25870_e35240_d_n0, assign25870_e35240_d_n2, assign25870_e35240_d_n6, assign25870_e35240_d_n7, assign25870_e35240_d_n10, assign25870_e35240_d_n11, assign25870_e35240_d_n12, assign25870_e35240_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25870_e35238: f64 = (locals.var_t0__blk774 * locals.var_t0__blk774);
        (assign25870_e35238, ((locals.var_t0__blk774_dn0 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn0)), ((locals.var_t0__blk774_dn2 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn2)), ((locals.var_t0__blk774_dn6 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn6)), ((locals.var_t0__blk774_dn7 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn7)), ((locals.var_t0__blk774_dn10 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn10)), ((locals.var_t0__blk774_dn11 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn11)), ((locals.var_t0__blk774_dn12 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn12)), ((locals.var_t0__blk774_dn17 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn17)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12, locals.var_cnst1over_dn17,)
    }
};
        locals.var_cnst1over = assign25870_e35240;
        locals.var_cnst1over_dn0 = assign25870_e35240_d_n0;
        locals.var_cnst1over_dn2 = assign25870_e35240_d_n2;
        locals.var_cnst1over_dn6 = assign25870_e35240_d_n6;
        locals.var_cnst1over_dn7 = assign25870_e35240_d_n7;
        locals.var_cnst1over_dn10 = assign25870_e35240_d_n10;
        locals.var_cnst1over_dn11 = assign25870_e35240_d_n11;
        locals.var_cnst1over_dn12 = assign25870_e35240_d_n12;
        locals.var_cnst1over_dn17 = assign25870_e35240_d_n17;
        locals.var_cnst1over_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_91(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25880_e35253, assign25880_e35253_d_n0, assign25880_e35253_d_n2, assign25880_e35253_d_n6, assign25880_e35253_d_n7, assign25880_e35253_d_n10, assign25880_e35253_d_n11, assign25880_e35253_d_n12, assign25880_e35253_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25880_e35251: f64 = (locals.var_cnst1over * locals.var_exp_bvbs__blk837);
        (assign25880_e35251, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn2)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn7)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn12)), ((locals.var_cnst1over_dn17 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn17)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn12, locals.var_gammachi_dn17,)
    }
};
        locals.var_gammachi = assign25880_e35253;
        locals.var_gammachi_dn0 = assign25880_e35253_d_n0;
        locals.var_gammachi_dn2 = assign25880_e35253_d_n2;
        locals.var_gammachi_dn6 = assign25880_e35253_d_n6;
        locals.var_gammachi_dn7 = assign25880_e35253_d_n7;
        locals.var_gammachi_dn10 = assign25880_e35253_d_n10;
        locals.var_gammachi_dn11 = assign25880_e35253_d_n11;
        locals.var_gammachi_dn12 = assign25880_e35253_d_n12;
        locals.var_gammachi_dn17 = assign25880_e35253_d_n17;
        locals.var_gammachi_rv = 0.0;

        let (assign25890_e35266, assign25890_e35266_d_n0, assign25890_e35266_d_n2, assign25890_e35266_d_n6, assign25890_e35266_d_n7, assign25890_e35266_d_n10, assign25890_e35266_d_n11, assign25890_e35266_d_n12, assign25890_e35266_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25890_e35264: f64 = (locals.var_beta2 * locals.var_fac1p2__blk805);
        (assign25890_e35264, (locals.var_beta2 * locals.var_fac1p2__blk805_dn0), (locals.var_beta2 * locals.var_fac1p2__blk805_dn2), (locals.var_beta2 * locals.var_fac1p2__blk805_dn6), (locals.var_beta2 * locals.var_fac1p2__blk805_dn7), ((locals.var_beta2_dn10 * locals.var_fac1p2__blk805) + (locals.var_beta2 * locals.var_fac1p2__blk805_dn10)), (locals.var_beta2 * locals.var_fac1p2__blk805_dn11), (locals.var_beta2 * locals.var_fac1p2__blk805_dn12), (locals.var_beta2 * locals.var_fac1p2__blk805_dn17),)
    } else {
        (locals.var_t0__blk774, locals.var_t0__blk774_dn0, locals.var_t0__blk774_dn2, locals.var_t0__blk774_dn6, locals.var_t0__blk774_dn7, locals.var_t0__blk774_dn10, locals.var_t0__blk774_dn11, locals.var_t0__blk774_dn12, locals.var_t0__blk774_dn17,)
    }
};
        locals.var_t0__blk774 = assign25890_e35266;
        locals.var_t0__blk774_dn0 = assign25890_e35266_d_n0;
        locals.var_t0__blk774_dn2 = assign25890_e35266_d_n2;
        locals.var_t0__blk774_dn6 = assign25890_e35266_d_n6;
        locals.var_t0__blk774_dn7 = assign25890_e35266_d_n7;
        locals.var_t0__blk774_dn10 = assign25890_e35266_d_n10;
        locals.var_t0__blk774_dn11 = assign25890_e35266_d_n11;
        locals.var_t0__blk774_dn12 = assign25890_e35266_d_n12;
        locals.var_t0__blk774_dn17 = assign25890_e35266_d_n17;
        locals.var_t0__blk774_rv = 0.0;

        let (assign25900_e35279, assign25900_e35279_d_n0, assign25900_e35279_d_n2, assign25900_e35279_d_n6, assign25900_e35279_d_n7, assign25900_e35279_d_n10, assign25900_e35279_d_n11, assign25900_e35279_d_n12, assign25900_e35279_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25900_e35277: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign25900_e35277, (locals.var_beta * locals.var_vgpld_shift_dn0), (locals.var_beta * locals.var_vgpld_shift_dn2), (locals.var_beta * locals.var_vgpld_shift_dn6), (locals.var_beta * locals.var_vgpld_shift_dn7), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), (locals.var_beta * locals.var_vgpld_shift_dn11), (locals.var_beta * locals.var_vgpld_shift_dn12), (locals.var_beta * locals.var_vgpld_shift_dn17),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign25900_e35279;
        locals.var_psi_dn0 = assign25900_e35279_d_n0;
        locals.var_psi_dn2 = assign25900_e35279_d_n2;
        locals.var_psi_dn6 = assign25900_e35279_d_n6;
        locals.var_psi_dn7 = assign25900_e35279_d_n7;
        locals.var_psi_dn10 = assign25900_e35279_d_n10;
        locals.var_psi_dn11 = assign25900_e35279_d_n11;
        locals.var_psi_dn12 = assign25900_e35279_d_n12;
        locals.var_psi_dn17 = assign25900_e35279_d_n17;
        locals.var_psi_rv = 0.0;

        let (assign25910_e35306, assign25910_e35306_d_n0, assign25910_e35306_d_n2, assign25910_e35306_d_n6, assign25910_e35306_d_n7, assign25910_e35306_d_n10, assign25910_e35306_d_n11, assign25910_e35306_d_n12, assign25910_e35306_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25910_e35290: f64 = (locals.var_gammachi * locals.var_t0__blk774);
        let assign25910_e35293: f64 = (locals.var_psi * locals.var_psi);
        let assign25910_e35294: f64 = (assign25910_e35290 + assign25910_e35293);
        let assign25910_e35295: f64 = (assign25910_e35294).ln();
        let assign25910_e35298: f64 = (locals.var_cnst1over * locals.var_t0__blk774);
        let assign25910_e35299: f64 = (assign25910_e35298).ln();
        let assign25910_e35300: f64 = (assign25910_e35295 - assign25910_e35299);
        let assign25910_e35303: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign25910_e35304: f64 = (assign25910_e35300 + assign25910_e35303);
        (assign25910_e35304, ((((((locals.var_gammachi_dn0 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign25910_e35294) - (((locals.var_cnst1over_dn0 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn0)) / assign25910_e35298)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign25910_e35294) - (((locals.var_cnst1over_dn2 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn2)) / assign25910_e35298)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn6 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign25910_e35294) - (((locals.var_cnst1over_dn6 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn6)) / assign25910_e35298)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn7 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign25910_e35294) - (((locals.var_cnst1over_dn7 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn7)) / assign25910_e35298)) + (locals.var_beta * locals.var_vxbgmtcl_dn7)), ((((((locals.var_gammachi_dn10 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign25910_e35294) - (((locals.var_cnst1over_dn10 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn10)) / assign25910_e35298)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign25910_e35294) - (((locals.var_cnst1over_dn11 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn11)) / assign25910_e35298)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign25910_e35294) - (((locals.var_cnst1over_dn12 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn12)) / assign25910_e35298)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)), ((((((locals.var_gammachi_dn17 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn17)) + ((locals.var_psi_dn17 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn17))) / assign25910_e35294) - (((locals.var_cnst1over_dn17 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn17)) / assign25910_e35298)) + (locals.var_beta * locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12, locals.var_chi_1_dn17,)
    }
};
        locals.var_chi_1 = assign25910_e35306;
        locals.var_chi_1_dn0 = assign25910_e35306_d_n0;
        locals.var_chi_1_dn2 = assign25910_e35306_d_n2;
        locals.var_chi_1_dn6 = assign25910_e35306_d_n6;
        locals.var_chi_1_dn7 = assign25910_e35306_d_n7;
        locals.var_chi_1_dn10 = assign25910_e35306_d_n10;
        locals.var_chi_1_dn11 = assign25910_e35306_d_n11;
        locals.var_chi_1_dn12 = assign25910_e35306_d_n12;
        locals.var_chi_1_dn17 = assign25910_e35306_d_n17;
        locals.var_chi_1_rv = 0.0;

        let (assign25920_e35321, assign25920_e35321_d_n0, assign25920_e35321_d_n2, assign25920_e35321_d_n6, assign25920_e35321_d_n7, assign25920_e35321_d_n10, assign25920_e35321_d_n11, assign25920_e35321_d_n12, assign25920_e35321_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25920_e35317: f64 = (locals.var_psi - locals.var_chi_1);
        let assign25920_e35319: f64 = (assign25920_e35317 - 1.0);
        (assign25920_e35319, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12), (locals.var_psi_dn17 - locals.var_chi_1_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign25920_e35321;
        locals.var_tmf1_dn0 = assign25920_e35321_d_n0;
        locals.var_tmf1_dn2 = assign25920_e35321_d_n2;
        locals.var_tmf1_dn6 = assign25920_e35321_d_n6;
        locals.var_tmf1_dn7 = assign25920_e35321_d_n7;
        locals.var_tmf1_dn10 = assign25920_e35321_d_n10;
        locals.var_tmf1_dn11 = assign25920_e35321_d_n11;
        locals.var_tmf1_dn12 = assign25920_e35321_d_n12;
        locals.var_tmf1_dn17 = assign25920_e35321_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign25930_e35336, assign25930_e35336_d_n0, assign25930_e35336_d_n2, assign25930_e35336_d_n6, assign25930_e35336_d_n7, assign25930_e35336_d_n10, assign25930_e35336_d_n11, assign25930_e35336_d_n12, assign25930_e35336_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25930_e35332: f64 = (4.0 * locals.var_psi);
        let assign25930_e35334: f64 = assign25930_e35332;
        (assign25930_e35334, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn12), (4.0 * locals.var_psi_dn17),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign25930_e35336;
        locals.var_tmf2_dn0 = assign25930_e35336_d_n0;
        locals.var_tmf2_dn2 = assign25930_e35336_d_n2;
        locals.var_tmf2_dn6 = assign25930_e35336_d_n6;
        locals.var_tmf2_dn7 = assign25930_e35336_d_n7;
        locals.var_tmf2_dn10 = assign25930_e35336_d_n10;
        locals.var_tmf2_dn11 = assign25930_e35336_d_n11;
        locals.var_tmf2_dn12 = assign25930_e35336_d_n12;
        locals.var_tmf2_dn17 = assign25930_e35336_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign25940_e35353, assign25940_e35353_d_n0, assign25940_e35353_d_n2, assign25940_e35353_d_n6, assign25940_e35353_d_n7, assign25940_e35353_d_n10, assign25940_e35353_d_n11, assign25940_e35353_d_n12, assign25940_e35353_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let (assign25940_e35351, assign25940_e35351_d_n0, assign25940_e35351_d_n2, assign25940_e35351_d_n6, assign25940_e35351_d_n7, assign25940_e35351_d_n10, assign25940_e35351_d_n11, assign25940_e35351_d_n12, assign25940_e35351_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign25940_e35350: f64 = (-locals.var_tmf2);
                (assign25940_e35350, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign25940_e35351, assign25940_e35351_d_n0, assign25940_e35351_d_n2, assign25940_e35351_d_n6, assign25940_e35351_d_n7, assign25940_e35351_d_n10, assign25940_e35351_d_n11, assign25940_e35351_d_n12, assign25940_e35351_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign25940_e35353;
        locals.var_tmf2_dn0 = assign25940_e35353_d_n0;
        locals.var_tmf2_dn2 = assign25940_e35353_d_n2;
        locals.var_tmf2_dn6 = assign25940_e35353_d_n6;
        locals.var_tmf2_dn7 = assign25940_e35353_d_n7;
        locals.var_tmf2_dn10 = assign25940_e35353_d_n10;
        locals.var_tmf2_dn11 = assign25940_e35353_d_n11;
        locals.var_tmf2_dn12 = assign25940_e35353_d_n12;
        locals.var_tmf2_dn17 = assign25940_e35353_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign25950_e35369, assign25950_e35369_d_n0, assign25950_e35369_d_n2, assign25950_e35369_d_n6, assign25950_e35369_d_n7, assign25950_e35369_d_n10, assign25950_e35369_d_n11, assign25950_e35369_d_n12, assign25950_e35369_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25950_e35364: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign25950_e35366: f64 = (assign25950_e35364 + locals.var_tmf2);
        let assign25950_e35367: f64 = (assign25950_e35366).sqrt();
        (assign25950_e35367, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign25950_e35367)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign25950_e35367)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign25950_e35367)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign25950_e35367)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign25950_e35367)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign25950_e35367)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign25950_e35367)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign25950_e35367)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign25950_e35369;
        locals.var_tmf2_dn0 = assign25950_e35369_d_n0;
        locals.var_tmf2_dn2 = assign25950_e35369_d_n2;
        locals.var_tmf2_dn6 = assign25950_e35369_d_n6;
        locals.var_tmf2_dn7 = assign25950_e35369_d_n7;
        locals.var_tmf2_dn10 = assign25950_e35369_d_n10;
        locals.var_tmf2_dn11 = assign25950_e35369_d_n11;
        locals.var_tmf2_dn12 = assign25950_e35369_d_n12;
        locals.var_tmf2_dn17 = assign25950_e35369_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign25960_e35386, assign25960_e35386_d_n0, assign25960_e35386_d_n2, assign25960_e35386_d_n6, assign25960_e35386_d_n7, assign25960_e35386_d_n10, assign25960_e35386_d_n11, assign25960_e35386_d_n12, assign25960_e35386_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25960_e35382: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign25960_e35383: f64 = (1.0 + assign25960_e35382);
        let assign25960_e35384: f64 = (0.5 * assign25960_e35383);
        (assign25960_e35384, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk775, locals.var_t1__blk775_dn0, locals.var_t1__blk775_dn2, locals.var_t1__blk775_dn6, locals.var_t1__blk775_dn7, locals.var_t1__blk775_dn10, locals.var_t1__blk775_dn11, locals.var_t1__blk775_dn12, locals.var_t1__blk775_dn17,)
    }
};
        locals.var_t1__blk775 = assign25960_e35386;
        locals.var_t1__blk775_dn0 = assign25960_e35386_d_n0;
        locals.var_t1__blk775_dn2 = assign25960_e35386_d_n2;
        locals.var_t1__blk775_dn6 = assign25960_e35386_d_n6;
        locals.var_t1__blk775_dn7 = assign25960_e35386_d_n7;
        locals.var_t1__blk775_dn10 = assign25960_e35386_d_n10;
        locals.var_t1__blk775_dn11 = assign25960_e35386_d_n11;
        locals.var_t1__blk775_dn12 = assign25960_e35386_d_n12;
        locals.var_t1__blk775_dn17 = assign25960_e35386_d_n17;
        locals.var_t1__blk775_rv = 0.0;

        let (assign25970_e35407, assign25970_e35407_d_n0, assign25970_e35407_d_n2, assign25970_e35407_d_n6, assign25970_e35407_d_n7, assign25970_e35407_d_n10, assign25970_e35407_d_n11, assign25970_e35407_d_n12, assign25970_e35407_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25970_e35400: f64 = 2.0;
        let assign25970_e35401: f64 = (locals.var_tmf1 + assign25970_e35400);
        let assign25970_e35403: f64 = (assign25970_e35401 / locals.var_tmf2);
        let assign25970_e35404: f64 = (1.0 - assign25970_e35403);
        let assign25970_e35405: f64 = (0.5 * assign25970_e35404);
        (assign25970_e35405, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign25970_e35401 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign25970_e35401 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign25970_e35401 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign25970_e35401 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign25970_e35401 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign25970_e35401 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign25970_e35401 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign25970_e35401 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk776, locals.var_t2__blk776_dn0, locals.var_t2__blk776_dn2, locals.var_t2__blk776_dn6, locals.var_t2__blk776_dn7, locals.var_t2__blk776_dn10, locals.var_t2__blk776_dn11, locals.var_t2__blk776_dn12, locals.var_t2__blk776_dn17,)
    }
};
        locals.var_t2__blk776 = assign25970_e35407;
        locals.var_t2__blk776_dn0 = assign25970_e35407_d_n0;
        locals.var_t2__blk776_dn2 = assign25970_e35407_d_n2;
        locals.var_t2__blk776_dn6 = assign25970_e35407_d_n6;
        locals.var_t2__blk776_dn7 = assign25970_e35407_d_n7;
        locals.var_t2__blk776_dn10 = assign25970_e35407_d_n10;
        locals.var_t2__blk776_dn11 = assign25970_e35407_d_n11;
        locals.var_t2__blk776_dn12 = assign25970_e35407_d_n12;
        locals.var_t2__blk776_dn17 = assign25970_e35407_d_n17;
        locals.var_t2__blk776_rv = 0.0;

        let (assign25980_e35424, assign25980_e35424_d_n0, assign25980_e35424_d_n2, assign25980_e35424_d_n6, assign25980_e35424_d_n7, assign25980_e35424_d_n10, assign25980_e35424_d_n11, assign25980_e35424_d_n12, assign25980_e35424_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25980_e35420: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign25980_e35421: f64 = (0.5 * assign25980_e35420);
        let assign25980_e35422: f64 = (locals.var_psi - assign25980_e35421);
        (assign25980_e35422, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psi_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12, locals.var_chi_1_dn17,)
    }
};
        locals.var_chi_1 = assign25980_e35424;
        locals.var_chi_1_dn0 = assign25980_e35424_d_n0;
        locals.var_chi_1_dn2 = assign25980_e35424_d_n2;
        locals.var_chi_1_dn6 = assign25980_e35424_d_n6;
        locals.var_chi_1_dn7 = assign25980_e35424_d_n7;
        locals.var_chi_1_dn10 = assign25980_e35424_d_n10;
        locals.var_chi_1_dn11 = assign25980_e35424_d_n11;
        locals.var_chi_1_dn12 = assign25980_e35424_d_n12;
        locals.var_chi_1_dn17 = assign25980_e35424_d_n17;
        locals.var_chi_1_rv = 0.0;

        let (assign25990_e35437, assign25990_e35437_d_n0, assign25990_e35437_d_n2, assign25990_e35437_d_n6, assign25990_e35437_d_n7, assign25990_e35437_d_n10, assign25990_e35437_d_n11, assign25990_e35437_d_n12, assign25990_e35437_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign25990_e35435: f64 = (locals.var_psi - locals.var_chi_1);
        (assign25990_e35435, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12), (locals.var_psi_dn17 - locals.var_chi_1_dn17),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign25990_e35437;
        locals.var_psi_dn0 = assign25990_e35437_d_n0;
        locals.var_psi_dn2 = assign25990_e35437_d_n2;
        locals.var_psi_dn6 = assign25990_e35437_d_n6;
        locals.var_psi_dn7 = assign25990_e35437_d_n7;
        locals.var_psi_dn10 = assign25990_e35437_d_n10;
        locals.var_psi_dn11 = assign25990_e35437_d_n11;
        locals.var_psi_dn12 = assign25990_e35437_d_n12;
        locals.var_psi_dn17 = assign25990_e35437_d_n17;
        locals.var_psi_rv = 0.0;

        let (assign26000_e35452, assign26000_e35452_d_n0, assign26000_e35452_d_n2, assign26000_e35452_d_n6, assign26000_e35452_d_n7, assign26000_e35452_d_n10, assign26000_e35452_d_n11, assign26000_e35452_d_n12, assign26000_e35452_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign26000_e35449: f64 = (locals.var_beta * 0.1);
        let assign26000_e35450: f64 = (locals.var_psi + assign26000_e35449);
        (assign26000_e35450, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign26000_e35452;
        locals.var_psi_dn0 = assign26000_e35452_d_n0;
        locals.var_psi_dn2 = assign26000_e35452_d_n2;
        locals.var_psi_dn6 = assign26000_e35452_d_n6;
        locals.var_psi_dn7 = assign26000_e35452_d_n7;
        locals.var_psi_dn10 = assign26000_e35452_d_n10;
        locals.var_psi_dn11 = assign26000_e35452_d_n11;
        locals.var_psi_dn12 = assign26000_e35452_d_n12;
        locals.var_psi_dn17 = assign26000_e35452_d_n17;
        locals.var_psi_rv = 0.0;

        let (assign26010_e35479, assign26010_e35479_d_n0, assign26010_e35479_d_n2, assign26010_e35479_d_n6, assign26010_e35479_d_n7, assign26010_e35479_d_n10, assign26010_e35479_d_n11, assign26010_e35479_d_n12, assign26010_e35479_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign26010_e35463: f64 = (locals.var_gammachi * locals.var_t0__blk774);
        let assign26010_e35466: f64 = (locals.var_psi * locals.var_psi);
        let assign26010_e35467: f64 = (assign26010_e35463 + assign26010_e35466);
        let assign26010_e35468: f64 = (assign26010_e35467).ln();
        let assign26010_e35471: f64 = (locals.var_cnst1over * locals.var_t0__blk774);
        let assign26010_e35472: f64 = (assign26010_e35471).ln();
        let assign26010_e35473: f64 = (assign26010_e35468 - assign26010_e35472);
        let assign26010_e35476: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign26010_e35477: f64 = (assign26010_e35473 + assign26010_e35476);
        (assign26010_e35477, ((((((locals.var_gammachi_dn0 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign26010_e35467) - (((locals.var_cnst1over_dn0 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn0)) / assign26010_e35471)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign26010_e35467) - (((locals.var_cnst1over_dn2 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn2)) / assign26010_e35471)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn6 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign26010_e35467) - (((locals.var_cnst1over_dn6 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn6)) / assign26010_e35471)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn7 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign26010_e35467) - (((locals.var_cnst1over_dn7 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn7)) / assign26010_e35471)) + (locals.var_beta * locals.var_vxbgmtcl_dn7)), ((((((locals.var_gammachi_dn10 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign26010_e35467) - (((locals.var_cnst1over_dn10 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn10)) / assign26010_e35471)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign26010_e35467) - (((locals.var_cnst1over_dn11 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn11)) / assign26010_e35471)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign26010_e35467) - (((locals.var_cnst1over_dn12 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn12)) / assign26010_e35471)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)), ((((((locals.var_gammachi_dn17 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn17)) + ((locals.var_psi_dn17 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn17))) / assign26010_e35467) - (((locals.var_cnst1over_dn17 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn17)) / assign26010_e35471)) + (locals.var_beta * locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn12, locals.var_chi_b_dn17,)
    }
};
        locals.var_chi_b = assign26010_e35479;
        locals.var_chi_b_dn0 = assign26010_e35479_d_n0;
        locals.var_chi_b_dn2 = assign26010_e35479_d_n2;
        locals.var_chi_b_dn6 = assign26010_e35479_d_n6;
        locals.var_chi_b_dn7 = assign26010_e35479_d_n7;
        locals.var_chi_b_dn10 = assign26010_e35479_d_n10;
        locals.var_chi_b_dn11 = assign26010_e35479_d_n11;
        locals.var_chi_b_dn12 = assign26010_e35479_d_n12;
        locals.var_chi_b_dn17 = assign26010_e35479_d_n17;
        locals.var_chi_b_rv = 0.0;

        let (assign26020_e35490, assign26020_e35490_d_n0, assign26020_e35490_d_n2, assign26020_e35490_d_n6, assign26020_e35490_d_n7, assign26020_e35490_d_n10, assign26020_e35490_d_n11, assign26020_e35490_d_n12, assign26020_e35490_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        (locals.var_chi__blk818, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn12, locals.var_chi_a_dn17,)
    }
};
        locals.var_chi_a = assign26020_e35490;
        locals.var_chi_a_dn0 = assign26020_e35490_d_n0;
        locals.var_chi_a_dn2 = assign26020_e35490_d_n2;
        locals.var_chi_a_dn6 = assign26020_e35490_d_n6;
        locals.var_chi_a_dn7 = assign26020_e35490_d_n7;
        locals.var_chi_a_dn10 = assign26020_e35490_d_n10;
        locals.var_chi_a_dn11 = assign26020_e35490_d_n11;
        locals.var_chi_a_dn12 = assign26020_e35490_d_n12;
        locals.var_chi_a_dn17 = assign26020_e35490_d_n17;
        locals.var_chi_a_rv = 0.0;

        let (assign26030_e35507, assign26030_e35507_d_n0, assign26030_e35507_d_n2, assign26030_e35507_d_n6, assign26030_e35507_d_n7, assign26030_e35507_d_n10, assign26030_e35507_d_n11, assign26030_e35507_d_n12, assign26030_e35507_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign26030_e35501: f64 = (locals.var_chi_b - locals.var_chi_a);
        let assign26030_e35504: f64 = (0.0008 * 75.0);
        let assign26030_e35505: f64 = (assign26030_e35501 - assign26030_e35504);
        (assign26030_e35505, (locals.var_chi_b_dn0 - locals.var_chi_a_dn0), (locals.var_chi_b_dn2 - locals.var_chi_a_dn2), (locals.var_chi_b_dn6 - locals.var_chi_a_dn6), (locals.var_chi_b_dn7 - locals.var_chi_a_dn7), (locals.var_chi_b_dn10 - locals.var_chi_a_dn10), (locals.var_chi_b_dn11 - locals.var_chi_a_dn11), (locals.var_chi_b_dn12 - locals.var_chi_a_dn12), (locals.var_chi_b_dn17 - locals.var_chi_a_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign26030_e35507;
        locals.var_tmf1_dn0 = assign26030_e35507_d_n0;
        locals.var_tmf1_dn2 = assign26030_e35507_d_n2;
        locals.var_tmf1_dn6 = assign26030_e35507_d_n6;
        locals.var_tmf1_dn7 = assign26030_e35507_d_n7;
        locals.var_tmf1_dn10 = assign26030_e35507_d_n10;
        locals.var_tmf1_dn11 = assign26030_e35507_d_n11;
        locals.var_tmf1_dn12 = assign26030_e35507_d_n12;
        locals.var_tmf1_dn17 = assign26030_e35507_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign26040_e35524, assign26040_e35524_d_n0, assign26040_e35524_d_n2, assign26040_e35524_d_n6, assign26040_e35524_d_n7, assign26040_e35524_d_n10, assign26040_e35524_d_n11, assign26040_e35524_d_n12, assign26040_e35524_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign26040_e35518: f64 = (4.0 * locals.var_chi_b);
        let assign26040_e35521: f64 = (0.0008 * 75.0);
        let assign26040_e35522: f64 = (assign26040_e35518 * assign26040_e35521);
        (assign26040_e35522, ((4.0 * locals.var_chi_b_dn0) * assign26040_e35521), ((4.0 * locals.var_chi_b_dn2) * assign26040_e35521), ((4.0 * locals.var_chi_b_dn6) * assign26040_e35521), ((4.0 * locals.var_chi_b_dn7) * assign26040_e35521), ((4.0 * locals.var_chi_b_dn10) * assign26040_e35521), ((4.0 * locals.var_chi_b_dn11) * assign26040_e35521), ((4.0 * locals.var_chi_b_dn12) * assign26040_e35521), ((4.0 * locals.var_chi_b_dn17) * assign26040_e35521),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign26040_e35524;
        locals.var_tmf2_dn0 = assign26040_e35524_d_n0;
        locals.var_tmf2_dn2 = assign26040_e35524_d_n2;
        locals.var_tmf2_dn6 = assign26040_e35524_d_n6;
        locals.var_tmf2_dn7 = assign26040_e35524_d_n7;
        locals.var_tmf2_dn10 = assign26040_e35524_d_n10;
        locals.var_tmf2_dn11 = assign26040_e35524_d_n11;
        locals.var_tmf2_dn12 = assign26040_e35524_d_n12;
        locals.var_tmf2_dn17 = assign26040_e35524_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign26050_e35541, assign26050_e35541_d_n0, assign26050_e35541_d_n2, assign26050_e35541_d_n6, assign26050_e35541_d_n7, assign26050_e35541_d_n10, assign26050_e35541_d_n11, assign26050_e35541_d_n12, assign26050_e35541_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let (assign26050_e35539, assign26050_e35539_d_n0, assign26050_e35539_d_n2, assign26050_e35539_d_n6, assign26050_e35539_d_n7, assign26050_e35539_d_n10, assign26050_e35539_d_n11, assign26050_e35539_d_n12, assign26050_e35539_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign26050_e35538: f64 = (-locals.var_tmf2);
                (assign26050_e35538, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign26050_e35539, assign26050_e35539_d_n0, assign26050_e35539_d_n2, assign26050_e35539_d_n6, assign26050_e35539_d_n7, assign26050_e35539_d_n10, assign26050_e35539_d_n11, assign26050_e35539_d_n12, assign26050_e35539_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign26050_e35541;
        locals.var_tmf2_dn0 = assign26050_e35541_d_n0;
        locals.var_tmf2_dn2 = assign26050_e35541_d_n2;
        locals.var_tmf2_dn6 = assign26050_e35541_d_n6;
        locals.var_tmf2_dn7 = assign26050_e35541_d_n7;
        locals.var_tmf2_dn10 = assign26050_e35541_d_n10;
        locals.var_tmf2_dn11 = assign26050_e35541_d_n11;
        locals.var_tmf2_dn12 = assign26050_e35541_d_n12;
        locals.var_tmf2_dn17 = assign26050_e35541_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign26060_e35557, assign26060_e35557_d_n0, assign26060_e35557_d_n2, assign26060_e35557_d_n6, assign26060_e35557_d_n7, assign26060_e35557_d_n10, assign26060_e35557_d_n11, assign26060_e35557_d_n12, assign26060_e35557_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign26060_e35552: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign26060_e35554: f64 = (assign26060_e35552 + locals.var_tmf2);
        let assign26060_e35555: f64 = (assign26060_e35554).sqrt();
        (assign26060_e35555, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign26060_e35555)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign26060_e35555)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign26060_e35555)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign26060_e35555)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign26060_e35555)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign26060_e35555)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign26060_e35555)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign26060_e35555)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign26060_e35557;
        locals.var_tmf2_dn0 = assign26060_e35557_d_n0;
        locals.var_tmf2_dn2 = assign26060_e35557_d_n2;
        locals.var_tmf2_dn6 = assign26060_e35557_d_n6;
        locals.var_tmf2_dn7 = assign26060_e35557_d_n7;
        locals.var_tmf2_dn10 = assign26060_e35557_d_n10;
        locals.var_tmf2_dn11 = assign26060_e35557_d_n11;
        locals.var_tmf2_dn12 = assign26060_e35557_d_n12;
        locals.var_tmf2_dn17 = assign26060_e35557_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign26070_e35574, assign26070_e35574_d_n0, assign26070_e35574_d_n2, assign26070_e35574_d_n6, assign26070_e35574_d_n7, assign26070_e35574_d_n10, assign26070_e35574_d_n11, assign26070_e35574_d_n12, assign26070_e35574_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign26070_e35570: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign26070_e35571: f64 = (1.0 + assign26070_e35570);
        let assign26070_e35572: f64 = (0.5 * assign26070_e35571);
        (assign26070_e35572, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk775, locals.var_t1__blk775_dn0, locals.var_t1__blk775_dn2, locals.var_t1__blk775_dn6, locals.var_t1__blk775_dn7, locals.var_t1__blk775_dn10, locals.var_t1__blk775_dn11, locals.var_t1__blk775_dn12, locals.var_t1__blk775_dn17,)
    }
};
        locals.var_t1__blk775 = assign26070_e35574;
        locals.var_t1__blk775_dn0 = assign26070_e35574_d_n0;
        locals.var_t1__blk775_dn2 = assign26070_e35574_d_n2;
        locals.var_t1__blk775_dn6 = assign26070_e35574_d_n6;
        locals.var_t1__blk775_dn7 = assign26070_e35574_d_n7;
        locals.var_t1__blk775_dn10 = assign26070_e35574_d_n10;
        locals.var_t1__blk775_dn11 = assign26070_e35574_d_n11;
        locals.var_t1__blk775_dn12 = assign26070_e35574_d_n12;
        locals.var_t1__blk775_dn17 = assign26070_e35574_d_n17;
        locals.var_t1__blk775_rv = 0.0;

        let (assign26080_e35597, assign26080_e35597_d_n0, assign26080_e35597_d_n2, assign26080_e35597_d_n6, assign26080_e35597_d_n7, assign26080_e35597_d_n10, assign26080_e35597_d_n11, assign26080_e35597_d_n12, assign26080_e35597_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign26080_e35588: f64 = (2.0 * 0.0008);
        let assign26080_e35590: f64 = (assign26080_e35588 * 75.0);
        let assign26080_e35591: f64 = (locals.var_tmf1 + assign26080_e35590);
        let assign26080_e35593: f64 = (assign26080_e35591 / locals.var_tmf2);
        let assign26080_e35594: f64 = (1.0 - assign26080_e35593);
        let assign26080_e35595: f64 = (0.5 * assign26080_e35594);
        (assign26080_e35595, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign26080_e35591 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign26080_e35591 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign26080_e35591 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign26080_e35591 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign26080_e35591 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign26080_e35591 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign26080_e35591 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign26080_e35591 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk776, locals.var_t2__blk776_dn0, locals.var_t2__blk776_dn2, locals.var_t2__blk776_dn6, locals.var_t2__blk776_dn7, locals.var_t2__blk776_dn10, locals.var_t2__blk776_dn11, locals.var_t2__blk776_dn12, locals.var_t2__blk776_dn17,)
    }
};
        locals.var_t2__blk776 = assign26080_e35597;
        locals.var_t2__blk776_dn0 = assign26080_e35597_d_n0;
        locals.var_t2__blk776_dn2 = assign26080_e35597_d_n2;
        locals.var_t2__blk776_dn6 = assign26080_e35597_d_n6;
        locals.var_t2__blk776_dn7 = assign26080_e35597_d_n7;
        locals.var_t2__blk776_dn10 = assign26080_e35597_d_n10;
        locals.var_t2__blk776_dn11 = assign26080_e35597_d_n11;
        locals.var_t2__blk776_dn12 = assign26080_e35597_d_n12;
        locals.var_t2__blk776_dn17 = assign26080_e35597_d_n17;
        locals.var_t2__blk776_rv = 0.0;

        let (assign26090_e35614, assign26090_e35614_d_n0, assign26090_e35614_d_n2, assign26090_e35614_d_n6, assign26090_e35614_d_n7, assign26090_e35614_d_n10, assign26090_e35614_d_n11, assign26090_e35614_d_n12, assign26090_e35614_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign26090_e35610: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign26090_e35611: f64 = (0.5 * assign26090_e35610);
        let assign26090_e35612: f64 = (locals.var_chi_b - assign26090_e35611);
        (assign26090_e35612, (locals.var_chi_b_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_chi_b_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_chi_b_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi__blk818, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    }
};
        locals.var_chi__blk818 = assign26090_e35614;
        locals.var_chi__blk818_dn0 = assign26090_e35614_d_n0;
        locals.var_chi__blk818_dn2 = assign26090_e35614_d_n2;
        locals.var_chi__blk818_dn6 = assign26090_e35614_d_n6;
        locals.var_chi__blk818_dn7 = assign26090_e35614_d_n7;
        locals.var_chi__blk818_dn10 = assign26090_e35614_d_n10;
        locals.var_chi__blk818_dn11 = assign26090_e35614_d_n11;
        locals.var_chi__blk818_dn12 = assign26090_e35614_d_n12;
        locals.var_chi__blk818_dn17 = assign26090_e35614_d_n17;
        locals.var_chi__blk818_rv = 0.0;

        let (assign26100_e35629, assign26100_e35629_d_n0, assign26100_e35629_d_n2, assign26100_e35629_d_n6, assign26100_e35629_d_n7, assign26100_e35629_d_n10, assign26100_e35629_d_n11, assign26100_e35629_d_n12, assign26100_e35629_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign26100_e35625: f64 = (locals.var_chi__blk818 / locals.var_beta);
        let assign26100_e35627: f64 = (assign26100_e35625 - locals.var_vxbgmtcl);
        (assign26100_e35627, ((locals.var_chi__blk818_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk818_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk818_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk818_dn7 / locals.var_beta) - locals.var_vxbgmtcl_dn7), ((((locals.var_chi__blk818_dn10 * locals.var_beta) - (locals.var_chi__blk818 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk818_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk818_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk818_dn17 / locals.var_beta) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
        locals.var_ps0ld = assign26100_e35629;
        locals.var_ps0ld_dn0 = assign26100_e35629_d_n0;
        locals.var_ps0ld_dn2 = assign26100_e35629_d_n2;
        locals.var_ps0ld_dn6 = assign26100_e35629_d_n6;
        locals.var_ps0ld_dn7 = assign26100_e35629_d_n7;
        locals.var_ps0ld_dn10 = assign26100_e35629_d_n10;
        locals.var_ps0ld_dn11 = assign26100_e35629_d_n11;
        locals.var_ps0ld_dn12 = assign26100_e35629_d_n12;
        locals.var_ps0ld_dn17 = assign26100_e35629_d_n17;
        locals.var_ps0ld_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_92(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26110_e35646, assign26110_e35646_d_n0, assign26110_e35646_d_n2, assign26110_e35646_d_n6, assign26110_e35646_d_n7, assign26110_e35646_d_n10, assign26110_e35646_d_n11, assign26110_e35646_d_n12, assign26110_e35646_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign26110_e35640: f64 = (locals.var_chi__blk818 - 1.0);
        let assign26110_e35642: f64 = (-locals.var_chi__blk818);
        let assign26110_e35643: f64 = (assign26110_e35642).exp();
        let assign26110_e35644: f64 = (assign26110_e35640 + assign26110_e35643);
        (assign26110_e35644, (locals.var_chi__blk818_dn0 + (assign26110_e35643 * (-locals.var_chi__blk818_dn0))), (locals.var_chi__blk818_dn2 + (assign26110_e35643 * (-locals.var_chi__blk818_dn2))), (locals.var_chi__blk818_dn6 + (assign26110_e35643 * (-locals.var_chi__blk818_dn6))), (locals.var_chi__blk818_dn7 + (assign26110_e35643 * (-locals.var_chi__blk818_dn7))), (locals.var_chi__blk818_dn10 + (assign26110_e35643 * (-locals.var_chi__blk818_dn10))), (locals.var_chi__blk818_dn11 + (assign26110_e35643 * (-locals.var_chi__blk818_dn11))), (locals.var_chi__blk818_dn12 + (assign26110_e35643 * (-locals.var_chi__blk818_dn12))), (locals.var_chi__blk818_dn17 + (assign26110_e35643 * (-locals.var_chi__blk818_dn17))),)
    } else {
        (locals.var_t1__blk775, locals.var_t1__blk775_dn0, locals.var_t1__blk775_dn2, locals.var_t1__blk775_dn6, locals.var_t1__blk775_dn7, locals.var_t1__blk775_dn10, locals.var_t1__blk775_dn11, locals.var_t1__blk775_dn12, locals.var_t1__blk775_dn17,)
    }
};
        locals.var_t1__blk775 = assign26110_e35646;
        locals.var_t1__blk775_dn0 = assign26110_e35646_d_n0;
        locals.var_t1__blk775_dn2 = assign26110_e35646_d_n2;
        locals.var_t1__blk775_dn6 = assign26110_e35646_d_n6;
        locals.var_t1__blk775_dn7 = assign26110_e35646_d_n7;
        locals.var_t1__blk775_dn10 = assign26110_e35646_d_n10;
        locals.var_t1__blk775_dn11 = assign26110_e35646_d_n11;
        locals.var_t1__blk775_dn12 = assign26110_e35646_d_n12;
        locals.var_t1__blk775_dn17 = assign26110_e35646_d_n17;
        locals.var_t1__blk775_rv = 0.0;

        let assign26120_e35650: f64 = (10.0 * 2.220446049250313e-16);
        let assign26120_e35651: f64 = if locals.var_t1__blk775 < assign26120_e35650 { 1.0 } else { 0.0 };
        locals.var_guard862 = assign26120_e35651;
        locals.var_guard862_rv = 0.0;

        let (assign26130_e35666, assign26130_e35666_d_n0, assign26130_e35666_d_n2, assign26130_e35666_d_n6, assign26130_e35666_d_n7, assign26130_e35666_d_n10, assign26130_e35666_d_n11, assign26130_e35666_d_n12, assign26130_e35666_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard862 != 0.0)) {
        let assign26130_e35664: f64 = (10.0 * 2.220446049250313e-16);
        (assign26130_e35664, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk775, locals.var_t1__blk775_dn0, locals.var_t1__blk775_dn2, locals.var_t1__blk775_dn6, locals.var_t1__blk775_dn7, locals.var_t1__blk775_dn10, locals.var_t1__blk775_dn11, locals.var_t1__blk775_dn12, locals.var_t1__blk775_dn17,)
    }
};
        locals.var_t1__blk775 = assign26130_e35666;
        locals.var_t1__blk775_dn0 = assign26130_e35666_d_n0;
        locals.var_t1__blk775_dn2 = assign26130_e35666_d_n2;
        locals.var_t1__blk775_dn6 = assign26130_e35666_d_n6;
        locals.var_t1__blk775_dn7 = assign26130_e35666_d_n7;
        locals.var_t1__blk775_dn10 = assign26130_e35666_d_n10;
        locals.var_t1__blk775_dn11 = assign26130_e35666_d_n11;
        locals.var_t1__blk775_dn12 = assign26130_e35666_d_n12;
        locals.var_t1__blk775_dn17 = assign26130_e35666_d_n17;
        locals.var_t1__blk775_rv = 0.0;

        let (assign26140_e35678, assign26140_e35678_d_n0, assign26140_e35678_d_n2, assign26140_e35678_d_n6, assign26140_e35678_d_n7, assign26140_e35678_d_n10, assign26140_e35678_d_n11, assign26140_e35678_d_n12, assign26140_e35678_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign26140_e35676: f64 = (locals.var_t1__blk775).sqrt();
        (assign26140_e35676, (locals.var_t1__blk775_dn0 / (2.0 * assign26140_e35676)), (locals.var_t1__blk775_dn2 / (2.0 * assign26140_e35676)), (locals.var_t1__blk775_dn6 / (2.0 * assign26140_e35676)), (locals.var_t1__blk775_dn7 / (2.0 * assign26140_e35676)), (locals.var_t1__blk775_dn10 / (2.0 * assign26140_e35676)), (locals.var_t1__blk775_dn11 / (2.0 * assign26140_e35676)), (locals.var_t1__blk775_dn12 / (2.0 * assign26140_e35676)), (locals.var_t1__blk775_dn17 / (2.0 * assign26140_e35676)),)
    } else {
        (locals.var_t2__blk776, locals.var_t2__blk776_dn0, locals.var_t2__blk776_dn2, locals.var_t2__blk776_dn6, locals.var_t2__blk776_dn7, locals.var_t2__blk776_dn10, locals.var_t2__blk776_dn11, locals.var_t2__blk776_dn12, locals.var_t2__blk776_dn17,)
    }
};
        locals.var_t2__blk776 = assign26140_e35678;
        locals.var_t2__blk776_dn0 = assign26140_e35678_d_n0;
        locals.var_t2__blk776_dn2 = assign26140_e35678_d_n2;
        locals.var_t2__blk776_dn6 = assign26140_e35678_d_n6;
        locals.var_t2__blk776_dn7 = assign26140_e35678_d_n7;
        locals.var_t2__blk776_dn10 = assign26140_e35678_d_n10;
        locals.var_t2__blk776_dn11 = assign26140_e35678_d_n11;
        locals.var_t2__blk776_dn12 = assign26140_e35678_d_n12;
        locals.var_t2__blk776_dn17 = assign26140_e35678_d_n17;
        locals.var_t2__blk776_rv = 0.0;

        let (assign26150_e35691, assign26150_e35691_d_n0, assign26150_e35691_d_n2, assign26150_e35691_d_n6, assign26150_e35691_d_n7, assign26150_e35691_d_n10, assign26150_e35691_d_n11, assign26150_e35691_d_n12, assign26150_e35691_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign26150_e35689: f64 = (locals.var_cnst0over * locals.var_t2__blk776);
        (assign26150_e35689, ((locals.var_cnst0over_dn0 * locals.var_t2__blk776) + (locals.var_cnst0over * locals.var_t2__blk776_dn0)), ((locals.var_cnst0over_dn2 * locals.var_t2__blk776) + (locals.var_cnst0over * locals.var_t2__blk776_dn2)), ((locals.var_cnst0over_dn6 * locals.var_t2__blk776) + (locals.var_cnst0over * locals.var_t2__blk776_dn6)), ((locals.var_cnst0over_dn7 * locals.var_t2__blk776) + (locals.var_cnst0over * locals.var_t2__blk776_dn7)), ((locals.var_cnst0over_dn10 * locals.var_t2__blk776) + (locals.var_cnst0over * locals.var_t2__blk776_dn10)), ((locals.var_cnst0over_dn11 * locals.var_t2__blk776) + (locals.var_cnst0over * locals.var_t2__blk776_dn11)), ((locals.var_cnst0over_dn12 * locals.var_t2__blk776) + (locals.var_cnst0over * locals.var_t2__blk776_dn12)), ((locals.var_cnst0over_dn17 * locals.var_t2__blk776) + (locals.var_cnst0over * locals.var_t2__blk776_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign26150_e35691;
        locals.var_qbuld_dn0 = assign26150_e35691_d_n0;
        locals.var_qbuld_dn2 = assign26150_e35691_d_n2;
        locals.var_qbuld_dn6 = assign26150_e35691_d_n6;
        locals.var_qbuld_dn7 = assign26150_e35691_d_n7;
        locals.var_qbuld_dn10 = assign26150_e35691_d_n10;
        locals.var_qbuld_dn11 = assign26150_e35691_d_n11;
        locals.var_qbuld_dn12 = assign26150_e35691_d_n12;
        locals.var_qbuld_dn17 = assign26150_e35691_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign26160_e35706, assign26160_e35706_d_n0, assign26160_e35706_d_n2, assign26160_e35706_d_n6, assign26160_e35706_d_n7, assign26160_e35706_d_n10, assign26160_e35706_d_n11, assign26160_e35706_d_n12, assign26160_e35706_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign26160_e35703: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign26160_e35704: f64 = (locals.var_cox0 * assign26160_e35703);
        (assign26160_e35704, (locals.var_cox0 * (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0)), (locals.var_cox0 * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0 * (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6)), (locals.var_cox0 * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0 * (locals.var_vgpld_dn10 - locals.var_ps0ld_dn10)), (locals.var_cox0 * (locals.var_vgpld_dn11 - locals.var_ps0ld_dn11)), (locals.var_cox0 * (locals.var_vgpld_dn12 - locals.var_ps0ld_dn12)), (locals.var_cox0 * (locals.var_vgpld_dn17 - locals.var_ps0ld_dn17)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign26160_e35706;
        locals.var_qsuld_dn0 = assign26160_e35706_d_n0;
        locals.var_qsuld_dn2 = assign26160_e35706_d_n2;
        locals.var_qsuld_dn6 = assign26160_e35706_d_n6;
        locals.var_qsuld_dn7 = assign26160_e35706_d_n7;
        locals.var_qsuld_dn10 = assign26160_e35706_d_n10;
        locals.var_qsuld_dn11 = assign26160_e35706_d_n11;
        locals.var_qsuld_dn12 = assign26160_e35706_d_n12;
        locals.var_qsuld_dn17 = assign26160_e35706_d_n17;
        locals.var_qsuld_rv = 0.0;

        let assign26170_e35709: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard863 = assign26170_e35709;
        locals.var_guard863_rv = 0.0;

        let (assign26180_e35726, assign26180_e35726_d_n0, assign26180_e35726_d_n2, assign26180_e35726_d_n6, assign26180_e35726_d_n7, assign26180_e35726_d_n10, assign26180_e35726_d_n11, assign26180_e35726_d_n12, assign26180_e35726_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let assign26180_e35722: f64 = (-locals.var_vxbgmtcl);
        let assign26180_e35723: f64 = (locals.var_beta * assign26180_e35722);
        let assign26180_e35724: f64 = (assign26180_e35723).exp();
        (assign26180_e35724, (assign26180_e35724 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign26180_e35724 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign26180_e35724 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign26180_e35724 * (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), (assign26180_e35724 * ((locals.var_beta_dn10 * assign26180_e35722) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign26180_e35724 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign26180_e35724 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))), (assign26180_e35724 * (locals.var_beta * (-locals.var_vxbgmtcl_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk837, locals.var_exp_bvbs__blk837_dn0, locals.var_exp_bvbs__blk837_dn2, locals.var_exp_bvbs__blk837_dn6, locals.var_exp_bvbs__blk837_dn7, locals.var_exp_bvbs__blk837_dn10, locals.var_exp_bvbs__blk837_dn11, locals.var_exp_bvbs__blk837_dn12, locals.var_exp_bvbs__blk837_dn17,)
    }
};
        locals.var_exp_bvbs__blk837 = assign26180_e35726;
        locals.var_exp_bvbs__blk837_dn0 = assign26180_e35726_d_n0;
        locals.var_exp_bvbs__blk837_dn2 = assign26180_e35726_d_n2;
        locals.var_exp_bvbs__blk837_dn6 = assign26180_e35726_d_n6;
        locals.var_exp_bvbs__blk837_dn7 = assign26180_e35726_d_n7;
        locals.var_exp_bvbs__blk837_dn10 = assign26180_e35726_d_n10;
        locals.var_exp_bvbs__blk837_dn11 = assign26180_e35726_d_n11;
        locals.var_exp_bvbs__blk837_dn12 = assign26180_e35726_d_n12;
        locals.var_exp_bvbs__blk837_dn17 = assign26180_e35726_d_n17;
        locals.var_exp_bvbs__blk837_rv = 0.0;

        let (assign26190_e35741, assign26190_e35741_d_n0, assign26190_e35741_d_n2, assign26190_e35741_d_n6, assign26190_e35741_d_n7, assign26190_e35741_d_n10, assign26190_e35741_d_n11, assign26190_e35741_d_n12, assign26190_e35741_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let assign26190_e35739: f64 = (locals.var_nin / locals.var_uc_nsubbttub);
        (assign26190_e35739, (((locals.var_nin_dn0 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn0)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn2 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn2)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn6 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn6)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn7 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn7)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn10 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn10)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn11 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn11)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn12 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn12)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn17 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn17)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)),)
    } else {
        (locals.var_t0__blk774, locals.var_t0__blk774_dn0, locals.var_t0__blk774_dn2, locals.var_t0__blk774_dn6, locals.var_t0__blk774_dn7, locals.var_t0__blk774_dn10, locals.var_t0__blk774_dn11, locals.var_t0__blk774_dn12, locals.var_t0__blk774_dn17,)
    }
};
        locals.var_t0__blk774 = assign26190_e35741;
        locals.var_t0__blk774_dn0 = assign26190_e35741_d_n0;
        locals.var_t0__blk774_dn2 = assign26190_e35741_d_n2;
        locals.var_t0__blk774_dn6 = assign26190_e35741_d_n6;
        locals.var_t0__blk774_dn7 = assign26190_e35741_d_n7;
        locals.var_t0__blk774_dn10 = assign26190_e35741_d_n10;
        locals.var_t0__blk774_dn11 = assign26190_e35741_d_n11;
        locals.var_t0__blk774_dn12 = assign26190_e35741_d_n12;
        locals.var_t0__blk774_dn17 = assign26190_e35741_d_n17;
        locals.var_t0__blk774_rv = 0.0;

        let (assign26200_e35756, assign26200_e35756_d_n0, assign26200_e35756_d_n2, assign26200_e35756_d_n6, assign26200_e35756_d_n7, assign26200_e35756_d_n10, assign26200_e35756_d_n11, assign26200_e35756_d_n12, assign26200_e35756_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let assign26200_e35754: f64 = (locals.var_t0__blk774 * locals.var_t0__blk774);
        (assign26200_e35754, ((locals.var_t0__blk774_dn0 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn0)), ((locals.var_t0__blk774_dn2 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn2)), ((locals.var_t0__blk774_dn6 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn6)), ((locals.var_t0__blk774_dn7 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn7)), ((locals.var_t0__blk774_dn10 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn10)), ((locals.var_t0__blk774_dn11 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn11)), ((locals.var_t0__blk774_dn12 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn12)), ((locals.var_t0__blk774_dn17 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn17)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12, locals.var_cnst1over_dn17,)
    }
};
        locals.var_cnst1over = assign26200_e35756;
        locals.var_cnst1over_dn0 = assign26200_e35756_d_n0;
        locals.var_cnst1over_dn2 = assign26200_e35756_d_n2;
        locals.var_cnst1over_dn6 = assign26200_e35756_d_n6;
        locals.var_cnst1over_dn7 = assign26200_e35756_d_n7;
        locals.var_cnst1over_dn10 = assign26200_e35756_d_n10;
        locals.var_cnst1over_dn11 = assign26200_e35756_d_n11;
        locals.var_cnst1over_dn12 = assign26200_e35756_d_n12;
        locals.var_cnst1over_dn17 = assign26200_e35756_d_n17;
        locals.var_cnst1over_rv = 0.0;

        let (assign26210_e35771, assign26210_e35771_d_n0, assign26210_e35771_d_n2, assign26210_e35771_d_n6, assign26210_e35771_d_n7, assign26210_e35771_d_n10, assign26210_e35771_d_n11, assign26210_e35771_d_n12, assign26210_e35771_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let assign26210_e35769: f64 = (locals.var_cnst1over * locals.var_exp_bvbs__blk837);
        (assign26210_e35769, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn2)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn7)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn12)), ((locals.var_cnst1over_dn17 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn17)),)
    } else {
        (locals.var_cfs1__blk846, locals.var_cfs1__blk846_dn0, locals.var_cfs1__blk846_dn2, locals.var_cfs1__blk846_dn6, locals.var_cfs1__blk846_dn7, locals.var_cfs1__blk846_dn10, locals.var_cfs1__blk846_dn11, locals.var_cfs1__blk846_dn12, locals.var_cfs1__blk846_dn17,)
    }
};
        locals.var_cfs1__blk846 = assign26210_e35771;
        locals.var_cfs1__blk846_dn0 = assign26210_e35771_d_n0;
        locals.var_cfs1__blk846_dn2 = assign26210_e35771_d_n2;
        locals.var_cfs1__blk846_dn6 = assign26210_e35771_d_n6;
        locals.var_cfs1__blk846_dn7 = assign26210_e35771_d_n7;
        locals.var_cfs1__blk846_dn10 = assign26210_e35771_d_n10;
        locals.var_cfs1__blk846_dn11 = assign26210_e35771_d_n11;
        locals.var_cfs1__blk846_dn12 = assign26210_e35771_d_n12;
        locals.var_cfs1__blk846_dn17 = assign26210_e35771_d_n17;
        locals.var_cfs1__blk846_rv = 0.0;

        let (assign26220_e35784,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv__blk791,)
    }
};
        locals.var_flg_conv__blk791 = assign26220_e35784;
        locals.var_flg_conv__blk791_rv = 0.0;

        let (assign26230_e35797,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign26230_e35797;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_93(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign26240_loop_guard: usize = 0;
        while {
            let assign26240_cond_e35811: f64 = (2.0 * 20.0);
            let assign26240_cond_e35813: f64 = (assign26240_cond_e35811 + 1.0);
            let assign26240_cond_e35815: f64 = if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_lp_s0 <= assign26240_cond_e35813)) { 1.0 } else { 0.0 };
            assign26240_cond_e35815 != 0.0
        } {
            assign26240_loop_guard += 1;
            assert!(assign26240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign26240_body0_e35828, assign26240_body0_e35828_d_n0, assign26240_body0_e35828_d_n2, assign26240_body0_e35828_d_n6, assign26240_body0_e35828_d_n7, assign26240_body0_e35828_d_n10, assign26240_body0_e35828_d_n11, assign26240_body0_e35828_d_n12, assign26240_body0_e35828_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb__blk842, locals.var_fb__blk842_dn0, locals.var_fb__blk842_dn2, locals.var_fb__blk842_dn6, locals.var_fb__blk842_dn7, locals.var_fb__blk842_dn10, locals.var_fb__blk842_dn11, locals.var_fb__blk842_dn12, locals.var_fb__blk842_dn17,)
    }
};
            locals.var_fb__blk842 = assign26240_body0_e35828;
            locals.var_fb__blk842_dn0 = assign26240_body0_e35828_d_n0;
            locals.var_fb__blk842_dn2 = assign26240_body0_e35828_d_n2;
            locals.var_fb__blk842_dn6 = assign26240_body0_e35828_d_n6;
            locals.var_fb__blk842_dn7 = assign26240_body0_e35828_d_n7;
            locals.var_fb__blk842_dn10 = assign26240_body0_e35828_d_n10;
            locals.var_fb__blk842_dn11 = assign26240_body0_e35828_d_n11;
            locals.var_fb__blk842_dn12 = assign26240_body0_e35828_d_n12;
            locals.var_fb__blk842_dn17 = assign26240_body0_e35828_d_n17;
            locals.var_fb__blk842_rv = 0.0;
            let (assign26240_body1_e35845, assign26240_body1_e35845_d_n0, assign26240_body1_e35845_d_n2, assign26240_body1_e35845_d_n6, assign26240_body1_e35845_d_n7, assign26240_body1_e35845_d_n10, assign26240_body1_e35845_d_n11, assign26240_body1_e35845_d_n12, assign26240_body1_e35845_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let assign26240_body1_e35842: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        let assign26240_body1_e35843: f64 = (locals.var_beta * assign26240_body1_e35842);
        (assign26240_body1_e35843, (locals.var_beta * (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign26240_body1_e35842) + (locals.var_beta * (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0ld_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0ld_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk818, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    }
};
            locals.var_chi__blk818 = assign26240_body1_e35845;
            locals.var_chi__blk818_dn0 = assign26240_body1_e35845_d_n0;
            locals.var_chi__blk818_dn2 = assign26240_body1_e35845_d_n2;
            locals.var_chi__blk818_dn6 = assign26240_body1_e35845_d_n6;
            locals.var_chi__blk818_dn7 = assign26240_body1_e35845_d_n7;
            locals.var_chi__blk818_dn10 = assign26240_body1_e35845_d_n10;
            locals.var_chi__blk818_dn11 = assign26240_body1_e35845_d_n11;
            locals.var_chi__blk818_dn12 = assign26240_body1_e35845_d_n12;
            locals.var_chi__blk818_dn17 = assign26240_body1_e35845_d_n17;
            locals.var_chi__blk818_rv = 0.0;
            let assign26240_body2_e35848: f64 = if locals.var_chi__blk818 < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard864 = assign26240_body2_e35848;
            locals.var_guard864_rv = 0.0;
            let (assign26240_body3_e35878, assign26240_body3_e35878_d_n0, assign26240_body3_e35878_d_n2, assign26240_body3_e35878_d_n6, assign26240_body3_e35878_d_n7, assign26240_body3_e35878_d_n10, assign26240_body3_e35878_d_n11, assign26240_body3_e35878_d_n12, assign26240_body3_e35878_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard864 != 0.0)) {
        let assign26240_body3_e35863: f64 = (locals.var_chi__blk818 * locals.var_chi__blk818);
        let assign26240_body3_e35865: f64 = (assign26240_body3_e35863 * locals.var_chi__blk818);
        let assign26240_body3_e35869: f64 = (-0.07053654284009761);
        let assign26240_body3_e35872: f64 = (locals.var_chi__blk818 * 0.006115288895133179);
        let assign26240_body3_e35873: f64 = (assign26240_body3_e35869 + assign26240_body3_e35872);
        let assign26240_body3_e35874: f64 = (locals.var_chi__blk818 * assign26240_body3_e35873);
        let assign26240_body3_e35875: f64 = (0.29693154855771 + assign26240_body3_e35874);
        let assign26240_body3_e35876: f64 = (assign26240_body3_e35865 * assign26240_body3_e35875);
        (assign26240_body3_e35876, ((((((locals.var_chi__blk818_dn0 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn0)) * locals.var_chi__blk818) + (assign26240_body3_e35863 * locals.var_chi__blk818_dn0)) * assign26240_body3_e35875) + (assign26240_body3_e35865 * ((locals.var_chi__blk818_dn0 * assign26240_body3_e35873) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn0 * 0.006115288895133179))))), ((((((locals.var_chi__blk818_dn2 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn2)) * locals.var_chi__blk818) + (assign26240_body3_e35863 * locals.var_chi__blk818_dn2)) * assign26240_body3_e35875) + (assign26240_body3_e35865 * ((locals.var_chi__blk818_dn2 * assign26240_body3_e35873) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn2 * 0.006115288895133179))))), ((((((locals.var_chi__blk818_dn6 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn6)) * locals.var_chi__blk818) + (assign26240_body3_e35863 * locals.var_chi__blk818_dn6)) * assign26240_body3_e35875) + (assign26240_body3_e35865 * ((locals.var_chi__blk818_dn6 * assign26240_body3_e35873) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn6 * 0.006115288895133179))))), ((((((locals.var_chi__blk818_dn7 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn7)) * locals.var_chi__blk818) + (assign26240_body3_e35863 * locals.var_chi__blk818_dn7)) * assign26240_body3_e35875) + (assign26240_body3_e35865 * ((locals.var_chi__blk818_dn7 * assign26240_body3_e35873) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn7 * 0.006115288895133179))))), ((((((locals.var_chi__blk818_dn10 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn10)) * locals.var_chi__blk818) + (assign26240_body3_e35863 * locals.var_chi__blk818_dn10)) * assign26240_body3_e35875) + (assign26240_body3_e35865 * ((locals.var_chi__blk818_dn10 * assign26240_body3_e35873) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn10 * 0.006115288895133179))))), ((((((locals.var_chi__blk818_dn11 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn11)) * locals.var_chi__blk818) + (assign26240_body3_e35863 * locals.var_chi__blk818_dn11)) * assign26240_body3_e35875) + (assign26240_body3_e35865 * ((locals.var_chi__blk818_dn11 * assign26240_body3_e35873) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn11 * 0.006115288895133179))))), ((((((locals.var_chi__blk818_dn12 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn12)) * locals.var_chi__blk818) + (assign26240_body3_e35863 * locals.var_chi__blk818_dn12)) * assign26240_body3_e35875) + (assign26240_body3_e35865 * ((locals.var_chi__blk818_dn12 * assign26240_body3_e35873) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn12 * 0.006115288895133179))))), ((((((locals.var_chi__blk818_dn17 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn17)) * locals.var_chi__blk818) + (assign26240_body3_e35863 * locals.var_chi__blk818_dn17)) * assign26240_body3_e35875) + (assign26240_body3_e35865 * ((locals.var_chi__blk818_dn17 * assign26240_body3_e35873) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn17 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi, locals.var_fi_dn0, locals.var_fi_dn2, locals.var_fi_dn6, locals.var_fi_dn7, locals.var_fi_dn10, locals.var_fi_dn11, locals.var_fi_dn12, locals.var_fi_dn17,)
    }
};
            locals.var_fi = assign26240_body3_e35878;
            locals.var_fi_dn0 = assign26240_body3_e35878_d_n0;
            locals.var_fi_dn2 = assign26240_body3_e35878_d_n2;
            locals.var_fi_dn6 = assign26240_body3_e35878_d_n6;
            locals.var_fi_dn7 = assign26240_body3_e35878_d_n7;
            locals.var_fi_dn10 = assign26240_body3_e35878_d_n10;
            locals.var_fi_dn11 = assign26240_body3_e35878_d_n11;
            locals.var_fi_dn12 = assign26240_body3_e35878_d_n12;
            locals.var_fi_dn17 = assign26240_body3_e35878_d_n17;
            locals.var_fi_rv = 0.0;
            let (assign26240_body4_e35912, assign26240_body4_e35912_d_n0, assign26240_body4_e35912_d_n2, assign26240_body4_e35912_d_n6, assign26240_body4_e35912_d_n7, assign26240_body4_e35912_d_n10, assign26240_body4_e35912_d_n11, assign26240_body4_e35912_d_n12, assign26240_body4_e35912_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard864 != 0.0)) {
        let assign26240_body4_e35893: f64 = (locals.var_chi__blk818 * locals.var_chi__blk818);
        let assign26240_body4_e35896: f64 = (3.0 * 0.29693154855771);
        let assign26240_body4_e35900: f64 = (-0.07053654284009761);
        let assign26240_body4_e35901: f64 = (4.0 * assign26240_body4_e35900);
        let assign26240_body4_e35904: f64 = (locals.var_chi__blk818 * 5.0);
        let assign26240_body4_e35906: f64 = (assign26240_body4_e35904 * 0.006115288895133179);
        let assign26240_body4_e35907: f64 = (assign26240_body4_e35901 + assign26240_body4_e35906);
        let assign26240_body4_e35908: f64 = (locals.var_chi__blk818 * assign26240_body4_e35907);
        let assign26240_body4_e35909: f64 = (assign26240_body4_e35896 + assign26240_body4_e35908);
        let assign26240_body4_e35910: f64 = (assign26240_body4_e35893 * assign26240_body4_e35909);
        (assign26240_body4_e35910, ((((locals.var_chi__blk818_dn0 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn0)) * assign26240_body4_e35909) + (assign26240_body4_e35893 * ((locals.var_chi__blk818_dn0 * assign26240_body4_e35907) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk818_dn2 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn2)) * assign26240_body4_e35909) + (assign26240_body4_e35893 * ((locals.var_chi__blk818_dn2 * assign26240_body4_e35907) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk818_dn6 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn6)) * assign26240_body4_e35909) + (assign26240_body4_e35893 * ((locals.var_chi__blk818_dn6 * assign26240_body4_e35907) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk818_dn7 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn7)) * assign26240_body4_e35909) + (assign26240_body4_e35893 * ((locals.var_chi__blk818_dn7 * assign26240_body4_e35907) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk818_dn10 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn10)) * assign26240_body4_e35909) + (assign26240_body4_e35893 * ((locals.var_chi__blk818_dn10 * assign26240_body4_e35907) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk818_dn11 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn11)) * assign26240_body4_e35909) + (assign26240_body4_e35893 * ((locals.var_chi__blk818_dn11 * assign26240_body4_e35907) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk818_dn12 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn12)) * assign26240_body4_e35909) + (assign26240_body4_e35893 * ((locals.var_chi__blk818_dn12 * assign26240_body4_e35907) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn12 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk818_dn17 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn17)) * assign26240_body4_e35909) + (assign26240_body4_e35893 * ((locals.var_chi__blk818_dn17 * assign26240_body4_e35907) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn17 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi, locals.var_fi_dchi_dn0, locals.var_fi_dchi_dn2, locals.var_fi_dchi_dn6, locals.var_fi_dchi_dn7, locals.var_fi_dchi_dn10, locals.var_fi_dchi_dn11, locals.var_fi_dchi_dn12, locals.var_fi_dchi_dn17,)
    }
};
            locals.var_fi_dchi = assign26240_body4_e35912;
            locals.var_fi_dchi_dn0 = assign26240_body4_e35912_d_n0;
            locals.var_fi_dchi_dn2 = assign26240_body4_e35912_d_n2;
            locals.var_fi_dchi_dn6 = assign26240_body4_e35912_d_n6;
            locals.var_fi_dchi_dn7 = assign26240_body4_e35912_d_n7;
            locals.var_fi_dchi_dn10 = assign26240_body4_e35912_d_n10;
            locals.var_fi_dchi_dn11 = assign26240_body4_e35912_d_n11;
            locals.var_fi_dchi_dn12 = assign26240_body4_e35912_d_n12;
            locals.var_fi_dchi_dn17 = assign26240_body4_e35912_d_n17;
            locals.var_fi_dchi_rv = 0.0;
            let (assign26240_body5_e35931, assign26240_body5_e35931_d_n0, assign26240_body5_e35931_d_n2, assign26240_body5_e35931_d_n6, assign26240_body5_e35931_d_n7, assign26240_body5_e35931_d_n10, assign26240_body5_e35931_d_n11, assign26240_body5_e35931_d_n12, assign26240_body5_e35931_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard864 != 0.0)) {
        let assign26240_body5_e35927: f64 = (locals.var_cfs1__blk846 * locals.var_fi);
        let assign26240_body5_e35929: f64 = (assign26240_body5_e35927 * locals.var_fi);
        (assign26240_body5_e35929, ((((locals.var_cfs1__blk846_dn0 * locals.var_fi) + (locals.var_cfs1__blk846 * locals.var_fi_dn0)) * locals.var_fi) + (assign26240_body5_e35927 * locals.var_fi_dn0)), ((((locals.var_cfs1__blk846_dn2 * locals.var_fi) + (locals.var_cfs1__blk846 * locals.var_fi_dn2)) * locals.var_fi) + (assign26240_body5_e35927 * locals.var_fi_dn2)), ((((locals.var_cfs1__blk846_dn6 * locals.var_fi) + (locals.var_cfs1__blk846 * locals.var_fi_dn6)) * locals.var_fi) + (assign26240_body5_e35927 * locals.var_fi_dn6)), ((((locals.var_cfs1__blk846_dn7 * locals.var_fi) + (locals.var_cfs1__blk846 * locals.var_fi_dn7)) * locals.var_fi) + (assign26240_body5_e35927 * locals.var_fi_dn7)), ((((locals.var_cfs1__blk846_dn10 * locals.var_fi) + (locals.var_cfs1__blk846 * locals.var_fi_dn10)) * locals.var_fi) + (assign26240_body5_e35927 * locals.var_fi_dn10)), ((((locals.var_cfs1__blk846_dn11 * locals.var_fi) + (locals.var_cfs1__blk846 * locals.var_fi_dn11)) * locals.var_fi) + (assign26240_body5_e35927 * locals.var_fi_dn11)), ((((locals.var_cfs1__blk846_dn12 * locals.var_fi) + (locals.var_cfs1__blk846 * locals.var_fi_dn12)) * locals.var_fi) + (assign26240_body5_e35927 * locals.var_fi_dn12)), ((((locals.var_cfs1__blk846_dn17 * locals.var_fi) + (locals.var_cfs1__blk846 * locals.var_fi_dn17)) * locals.var_fi) + (assign26240_body5_e35927 * locals.var_fi_dn17)),)
    } else {
        (locals.var_fs01__blk840, locals.var_fs01__blk840_dn0, locals.var_fs01__blk840_dn2, locals.var_fs01__blk840_dn6, locals.var_fs01__blk840_dn7, locals.var_fs01__blk840_dn10, locals.var_fs01__blk840_dn11, locals.var_fs01__blk840_dn12, locals.var_fs01__blk840_dn17,)
    }
};
            locals.var_fs01__blk840 = assign26240_body5_e35931;
            locals.var_fs01__blk840_dn0 = assign26240_body5_e35931_d_n0;
            locals.var_fs01__blk840_dn2 = assign26240_body5_e35931_d_n2;
            locals.var_fs01__blk840_dn6 = assign26240_body5_e35931_d_n6;
            locals.var_fs01__blk840_dn7 = assign26240_body5_e35931_d_n7;
            locals.var_fs01__blk840_dn10 = assign26240_body5_e35931_d_n10;
            locals.var_fs01__blk840_dn11 = assign26240_body5_e35931_d_n11;
            locals.var_fs01__blk840_dn12 = assign26240_body5_e35931_d_n12;
            locals.var_fs01__blk840_dn17 = assign26240_body5_e35931_d_n17;
            locals.var_fs01__blk840_rv = 0.0;
            let (assign26240_body6_e35954, assign26240_body6_e35954_d_n0, assign26240_body6_e35954_d_n2, assign26240_body6_e35954_d_n6, assign26240_body6_e35954_d_n7, assign26240_body6_e35954_d_n10, assign26240_body6_e35954_d_n11, assign26240_body6_e35954_d_n12, assign26240_body6_e35954_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard864 != 0.0)) {
        let assign26240_body6_e35946: f64 = (locals.var_cfs1__blk846 * locals.var_beta);
        let assign26240_body6_e35948: f64 = (assign26240_body6_e35946 * 2.0);
        let assign26240_body6_e35950: f64 = (assign26240_body6_e35948 * locals.var_fi);
        let assign26240_body6_e35952: f64 = (assign26240_body6_e35950 * locals.var_fi_dchi);
        (assign26240_body6_e35952, ((((((locals.var_cfs1__blk846_dn0 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26240_body6_e35948 * locals.var_fi_dn0)) * locals.var_fi_dchi) + (assign26240_body6_e35950 * locals.var_fi_dchi_dn0)), ((((((locals.var_cfs1__blk846_dn2 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26240_body6_e35948 * locals.var_fi_dn2)) * locals.var_fi_dchi) + (assign26240_body6_e35950 * locals.var_fi_dchi_dn2)), ((((((locals.var_cfs1__blk846_dn6 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26240_body6_e35948 * locals.var_fi_dn6)) * locals.var_fi_dchi) + (assign26240_body6_e35950 * locals.var_fi_dchi_dn6)), ((((((locals.var_cfs1__blk846_dn7 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26240_body6_e35948 * locals.var_fi_dn7)) * locals.var_fi_dchi) + (assign26240_body6_e35950 * locals.var_fi_dchi_dn7)), (((((((locals.var_cfs1__blk846_dn10 * locals.var_beta) + (locals.var_cfs1__blk846 * locals.var_beta_dn10)) * 2.0) * locals.var_fi) + (assign26240_body6_e35948 * locals.var_fi_dn10)) * locals.var_fi_dchi) + (assign26240_body6_e35950 * locals.var_fi_dchi_dn10)), ((((((locals.var_cfs1__blk846_dn11 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26240_body6_e35948 * locals.var_fi_dn11)) * locals.var_fi_dchi) + (assign26240_body6_e35950 * locals.var_fi_dchi_dn11)), ((((((locals.var_cfs1__blk846_dn12 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26240_body6_e35948 * locals.var_fi_dn12)) * locals.var_fi_dchi) + (assign26240_body6_e35950 * locals.var_fi_dchi_dn12)), ((((((locals.var_cfs1__blk846_dn17 * locals.var_beta) * 2.0) * locals.var_fi) + (assign26240_body6_e35948 * locals.var_fi_dn17)) * locals.var_fi_dchi) + (assign26240_body6_e35950 * locals.var_fi_dchi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk841, locals.var_fs01_dps0__blk841_dn0, locals.var_fs01_dps0__blk841_dn2, locals.var_fs01_dps0__blk841_dn6, locals.var_fs01_dps0__blk841_dn7, locals.var_fs01_dps0__blk841_dn10, locals.var_fs01_dps0__blk841_dn11, locals.var_fs01_dps0__blk841_dn12, locals.var_fs01_dps0__blk841_dn17,)
    }
};
            locals.var_fs01_dps0__blk841 = assign26240_body6_e35954;
            locals.var_fs01_dps0__blk841_dn0 = assign26240_body6_e35954_d_n0;
            locals.var_fs01_dps0__blk841_dn2 = assign26240_body6_e35954_d_n2;
            locals.var_fs01_dps0__blk841_dn6 = assign26240_body6_e35954_d_n6;
            locals.var_fs01_dps0__blk841_dn7 = assign26240_body6_e35954_d_n7;
            locals.var_fs01_dps0__blk841_dn10 = assign26240_body6_e35954_d_n10;
            locals.var_fs01_dps0__blk841_dn11 = assign26240_body6_e35954_d_n11;
            locals.var_fs01_dps0__blk841_dn12 = assign26240_body6_e35954_d_n12;
            locals.var_fs01_dps0__blk841_dn17 = assign26240_body6_e35954_d_n17;
            locals.var_fs01_dps0__blk841_rv = 0.0;
            let (assign26240_body7_e35989, assign26240_body7_e35989_d_n0, assign26240_body7_e35989_d_n2, assign26240_body7_e35989_d_n6, assign26240_body7_e35989_d_n7, assign26240_body7_e35989_d_n10, assign26240_body7_e35989_d_n11, assign26240_body7_e35989_d_n12, assign26240_body7_e35989_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard864 != 0.0)) {
        let assign26240_body7_e35971: f64 = (-0.117851130197758);
        let assign26240_body7_e35976: f64 = (-0.00163730162779191);
        let assign26240_body7_e35979: f64 = (locals.var_chi__blk818 * 6.36964918866352e-5);
        let assign26240_body7_e35980: f64 = (assign26240_body7_e35976 + assign26240_body7_e35979);
        let assign26240_body7_e35981: f64 = (locals.var_chi__blk818 * assign26240_body7_e35980);
        let assign26240_body7_e35982: f64 = (0.0178800506338833 + assign26240_body7_e35981);
        let assign26240_body7_e35983: f64 = (locals.var_chi__blk818 * assign26240_body7_e35982);
        let assign26240_body7_e35984: f64 = (assign26240_body7_e35971 + assign26240_body7_e35983);
        let assign26240_body7_e35985: f64 = (locals.var_chi__blk818 * assign26240_body7_e35984);
        let assign26240_body7_e35986: f64 = (0.707106781186548 + assign26240_body7_e35985);
        let assign26240_body7_e35987: f64 = (locals.var_chi__blk818 * assign26240_body7_e35986);
        (assign26240_body7_e35987, ((locals.var_chi__blk818_dn0 * assign26240_body7_e35986) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn0 * assign26240_body7_e35984) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn0 * assign26240_body7_e35982) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn0 * assign26240_body7_e35980) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk818_dn2 * assign26240_body7_e35986) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn2 * assign26240_body7_e35984) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn2 * assign26240_body7_e35982) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn2 * assign26240_body7_e35980) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk818_dn6 * assign26240_body7_e35986) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn6 * assign26240_body7_e35984) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn6 * assign26240_body7_e35982) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn6 * assign26240_body7_e35980) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk818_dn7 * assign26240_body7_e35986) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn7 * assign26240_body7_e35984) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn7 * assign26240_body7_e35982) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn7 * assign26240_body7_e35980) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk818_dn10 * assign26240_body7_e35986) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn10 * assign26240_body7_e35984) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn10 * assign26240_body7_e35982) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn10 * assign26240_body7_e35980) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk818_dn11 * assign26240_body7_e35986) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn11 * assign26240_body7_e35984) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn11 * assign26240_body7_e35982) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn11 * assign26240_body7_e35980) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk818_dn12 * assign26240_body7_e35986) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn12 * assign26240_body7_e35984) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn12 * assign26240_body7_e35982) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn12 * assign26240_body7_e35980) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn12 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk818_dn17 * assign26240_body7_e35986) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn17 * assign26240_body7_e35984) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn17 * assign26240_body7_e35982) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn17 * assign26240_body7_e35980) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn17 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb__blk842, locals.var_fb__blk842_dn0, locals.var_fb__blk842_dn2, locals.var_fb__blk842_dn6, locals.var_fb__blk842_dn7, locals.var_fb__blk842_dn10, locals.var_fb__blk842_dn11, locals.var_fb__blk842_dn12, locals.var_fb__blk842_dn17,)
    }
};
            locals.var_fb__blk842 = assign26240_body7_e35989;
            locals.var_fb__blk842_dn0 = assign26240_body7_e35989_d_n0;
            locals.var_fb__blk842_dn2 = assign26240_body7_e35989_d_n2;
            locals.var_fb__blk842_dn6 = assign26240_body7_e35989_d_n6;
            locals.var_fb__blk842_dn7 = assign26240_body7_e35989_d_n7;
            locals.var_fb__blk842_dn10 = assign26240_body7_e35989_d_n10;
            locals.var_fb__blk842_dn11 = assign26240_body7_e35989_d_n11;
            locals.var_fb__blk842_dn12 = assign26240_body7_e35989_d_n12;
            locals.var_fb__blk842_dn17 = assign26240_body7_e35989_d_n17;
            locals.var_fb__blk842_rv = 0.0;
            let (assign26240_body8_e36030, assign26240_body8_e36030_d_n0, assign26240_body8_e36030_d_n2, assign26240_body8_e36030_d_n6, assign26240_body8_e36030_d_n7, assign26240_body8_e36030_d_n10, assign26240_body8_e36030_d_n11, assign26240_body8_e36030_d_n12, assign26240_body8_e36030_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard864 != 0.0)) {
        let assign26240_body8_e36006: f64 = (-0.117851130197758);
        let assign26240_body8_e36007: f64 = (2.0 * assign26240_body8_e36006);
        let assign26240_body8_e36011: f64 = (3.0 * 0.0178800506338833);
        let assign26240_body8_e36015: f64 = (-0.00163730162779191);
        let assign26240_body8_e36016: f64 = (4.0 * assign26240_body8_e36015);
        let assign26240_body8_e36019: f64 = (locals.var_chi__blk818 * 5.0);
        let assign26240_body8_e36021: f64 = (assign26240_body8_e36019 * 6.36964918866352e-5);
        let assign26240_body8_e36022: f64 = (assign26240_body8_e36016 + assign26240_body8_e36021);
        let assign26240_body8_e36023: f64 = (locals.var_chi__blk818 * assign26240_body8_e36022);
        let assign26240_body8_e36024: f64 = (assign26240_body8_e36011 + assign26240_body8_e36023);
        let assign26240_body8_e36025: f64 = (locals.var_chi__blk818 * assign26240_body8_e36024);
        let assign26240_body8_e36026: f64 = (assign26240_body8_e36007 + assign26240_body8_e36025);
        let assign26240_body8_e36027: f64 = (locals.var_chi__blk818 * assign26240_body8_e36026);
        let assign26240_body8_e36028: f64 = (0.707106781186548 + assign26240_body8_e36027);
        (assign26240_body8_e36028, ((locals.var_chi__blk818_dn0 * assign26240_body8_e36026) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn0 * assign26240_body8_e36024) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn0 * assign26240_body8_e36022) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk818_dn2 * assign26240_body8_e36026) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn2 * assign26240_body8_e36024) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn2 * assign26240_body8_e36022) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk818_dn6 * assign26240_body8_e36026) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn6 * assign26240_body8_e36024) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn6 * assign26240_body8_e36022) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk818_dn7 * assign26240_body8_e36026) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn7 * assign26240_body8_e36024) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn7 * assign26240_body8_e36022) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk818_dn10 * assign26240_body8_e36026) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn10 * assign26240_body8_e36024) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn10 * assign26240_body8_e36022) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk818_dn11 * assign26240_body8_e36026) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn11 * assign26240_body8_e36024) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn11 * assign26240_body8_e36022) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk818_dn12 * assign26240_body8_e36026) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn12 * assign26240_body8_e36024) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn12 * assign26240_body8_e36022) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn12 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk818_dn17 * assign26240_body8_e36026) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn17 * assign26240_body8_e36024) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn17 * assign26240_body8_e36022) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn17 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi, locals.var_fb_dchi_dn0, locals.var_fb_dchi_dn2, locals.var_fb_dchi_dn6, locals.var_fb_dchi_dn7, locals.var_fb_dchi_dn10, locals.var_fb_dchi_dn11, locals.var_fb_dchi_dn12, locals.var_fb_dchi_dn17,)
    }
};
            locals.var_fb_dchi = assign26240_body8_e36030;
            locals.var_fb_dchi_dn0 = assign26240_body8_e36030_d_n0;
            locals.var_fb_dchi_dn2 = assign26240_body8_e36030_d_n2;
            locals.var_fb_dchi_dn6 = assign26240_body8_e36030_d_n6;
            locals.var_fb_dchi_dn7 = assign26240_body8_e36030_d_n7;
            locals.var_fb_dchi_dn10 = assign26240_body8_e36030_d_n10;
            locals.var_fb_dchi_dn11 = assign26240_body8_e36030_d_n11;
            locals.var_fb_dchi_dn12 = assign26240_body8_e36030_d_n12;
            locals.var_fb_dchi_dn17 = assign26240_body8_e36030_d_n17;
            locals.var_fb_dchi_rv = 0.0;
            let (assign26240_body9_e36052, assign26240_body9_e36052_d_n0, assign26240_body9_e36052_d_n2, assign26240_body9_e36052_d_n6, assign26240_body9_e36052_d_n7, assign26240_body9_e36052_d_n10, assign26240_body9_e36052_d_n11, assign26240_body9_e36052_d_n12, assign26240_body9_e36052_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard864 != 0.0)) {
        let assign26240_body9_e36045: f64 = (locals.var_fb__blk842 * locals.var_fb__blk842);
        let assign26240_body9_e36047: f64 = (assign26240_body9_e36045 + locals.var_fs01__blk840);
        let assign26240_body9_e36049: f64 = (assign26240_body9_e36047 + 1e-50);
        let assign26240_body9_e36050: f64 = (assign26240_body9_e36049).sqrt();
        (assign26240_body9_e36050, ((((locals.var_fb__blk842_dn0 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn0)) + locals.var_fs01__blk840_dn0) / (2.0 * assign26240_body9_e36050)), ((((locals.var_fb__blk842_dn2 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn2)) + locals.var_fs01__blk840_dn2) / (2.0 * assign26240_body9_e36050)), ((((locals.var_fb__blk842_dn6 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn6)) + locals.var_fs01__blk840_dn6) / (2.0 * assign26240_body9_e36050)), ((((locals.var_fb__blk842_dn7 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn7)) + locals.var_fs01__blk840_dn7) / (2.0 * assign26240_body9_e36050)), ((((locals.var_fb__blk842_dn10 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn10)) + locals.var_fs01__blk840_dn10) / (2.0 * assign26240_body9_e36050)), ((((locals.var_fb__blk842_dn11 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn11)) + locals.var_fs01__blk840_dn11) / (2.0 * assign26240_body9_e36050)), ((((locals.var_fb__blk842_dn12 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn12)) + locals.var_fs01__blk840_dn12) / (2.0 * assign26240_body9_e36050)), ((((locals.var_fb__blk842_dn17 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn17)) + locals.var_fs01__blk840_dn17) / (2.0 * assign26240_body9_e36050)),)
    } else {
        (locals.var_fs02__blk844, locals.var_fs02__blk844_dn0, locals.var_fs02__blk844_dn2, locals.var_fs02__blk844_dn6, locals.var_fs02__blk844_dn7, locals.var_fs02__blk844_dn10, locals.var_fs02__blk844_dn11, locals.var_fs02__blk844_dn12, locals.var_fs02__blk844_dn17,)
    }
};
            locals.var_fs02__blk844 = assign26240_body9_e36052;
            locals.var_fs02__blk844_dn0 = assign26240_body9_e36052_d_n0;
            locals.var_fs02__blk844_dn2 = assign26240_body9_e36052_d_n2;
            locals.var_fs02__blk844_dn6 = assign26240_body9_e36052_d_n6;
            locals.var_fs02__blk844_dn7 = assign26240_body9_e36052_d_n7;
            locals.var_fs02__blk844_dn10 = assign26240_body9_e36052_d_n10;
            locals.var_fs02__blk844_dn11 = assign26240_body9_e36052_d_n11;
            locals.var_fs02__blk844_dn12 = assign26240_body9_e36052_d_n12;
            locals.var_fs02__blk844_dn17 = assign26240_body9_e36052_d_n17;
            locals.var_fs02__blk844_rv = 0.0;
            let (assign26240_body10_e36079, assign26240_body10_e36079_d_n0, assign26240_body10_e36079_d_n2, assign26240_body10_e36079_d_n6, assign26240_body10_e36079_d_n7, assign26240_body10_e36079_d_n10, assign26240_body10_e36079_d_n11, assign26240_body10_e36079_d_n12, assign26240_body10_e36079_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard864 != 0.0)) {
        let assign26240_body10_e36067: f64 = (locals.var_beta * locals.var_fb_dchi);
        let assign26240_body10_e36069: f64 = (assign26240_body10_e36067 * 2.0);
        let assign26240_body10_e36071: f64 = (assign26240_body10_e36069 * locals.var_fb__blk842);
        let assign26240_body10_e36073: f64 = (assign26240_body10_e36071 + locals.var_fs01_dps0__blk841);
        let assign26240_body10_e36076: f64 = (locals.var_fs02__blk844 + locals.var_fs02__blk844);
        let assign26240_body10_e36077: f64 = (assign26240_body10_e36073 / assign26240_body10_e36076);
        (assign26240_body10_e36077, ((((((((locals.var_beta * locals.var_fb_dchi_dn0) * 2.0) * locals.var_fb__blk842) + (assign26240_body10_e36069 * locals.var_fb__blk842_dn0)) + locals.var_fs01_dps0__blk841_dn0) * assign26240_body10_e36076) - (assign26240_body10_e36073 * (locals.var_fs02__blk844_dn0 + locals.var_fs02__blk844_dn0))) / (assign26240_body10_e36076 * assign26240_body10_e36076)), ((((((((locals.var_beta * locals.var_fb_dchi_dn2) * 2.0) * locals.var_fb__blk842) + (assign26240_body10_e36069 * locals.var_fb__blk842_dn2)) + locals.var_fs01_dps0__blk841_dn2) * assign26240_body10_e36076) - (assign26240_body10_e36073 * (locals.var_fs02__blk844_dn2 + locals.var_fs02__blk844_dn2))) / (assign26240_body10_e36076 * assign26240_body10_e36076)), ((((((((locals.var_beta * locals.var_fb_dchi_dn6) * 2.0) * locals.var_fb__blk842) + (assign26240_body10_e36069 * locals.var_fb__blk842_dn6)) + locals.var_fs01_dps0__blk841_dn6) * assign26240_body10_e36076) - (assign26240_body10_e36073 * (locals.var_fs02__blk844_dn6 + locals.var_fs02__blk844_dn6))) / (assign26240_body10_e36076 * assign26240_body10_e36076)), ((((((((locals.var_beta * locals.var_fb_dchi_dn7) * 2.0) * locals.var_fb__blk842) + (assign26240_body10_e36069 * locals.var_fb__blk842_dn7)) + locals.var_fs01_dps0__blk841_dn7) * assign26240_body10_e36076) - (assign26240_body10_e36073 * (locals.var_fs02__blk844_dn7 + locals.var_fs02__blk844_dn7))) / (assign26240_body10_e36076 * assign26240_body10_e36076)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn10)) * 2.0) * locals.var_fb__blk842) + (assign26240_body10_e36069 * locals.var_fb__blk842_dn10)) + locals.var_fs01_dps0__blk841_dn10) * assign26240_body10_e36076) - (assign26240_body10_e36073 * (locals.var_fs02__blk844_dn10 + locals.var_fs02__blk844_dn10))) / (assign26240_body10_e36076 * assign26240_body10_e36076)), ((((((((locals.var_beta * locals.var_fb_dchi_dn11) * 2.0) * locals.var_fb__blk842) + (assign26240_body10_e36069 * locals.var_fb__blk842_dn11)) + locals.var_fs01_dps0__blk841_dn11) * assign26240_body10_e36076) - (assign26240_body10_e36073 * (locals.var_fs02__blk844_dn11 + locals.var_fs02__blk844_dn11))) / (assign26240_body10_e36076 * assign26240_body10_e36076)), ((((((((locals.var_beta * locals.var_fb_dchi_dn12) * 2.0) * locals.var_fb__blk842) + (assign26240_body10_e36069 * locals.var_fb__blk842_dn12)) + locals.var_fs01_dps0__blk841_dn12) * assign26240_body10_e36076) - (assign26240_body10_e36073 * (locals.var_fs02__blk844_dn12 + locals.var_fs02__blk844_dn12))) / (assign26240_body10_e36076 * assign26240_body10_e36076)), ((((((((locals.var_beta * locals.var_fb_dchi_dn17) * 2.0) * locals.var_fb__blk842) + (assign26240_body10_e36069 * locals.var_fb__blk842_dn17)) + locals.var_fs01_dps0__blk841_dn17) * assign26240_body10_e36076) - (assign26240_body10_e36073 * (locals.var_fs02__blk844_dn17 + locals.var_fs02__blk844_dn17))) / (assign26240_body10_e36076 * assign26240_body10_e36076)),)
    } else {
        (locals.var_fs02_dps0__blk845, locals.var_fs02_dps0__blk845_dn0, locals.var_fs02_dps0__blk845_dn2, locals.var_fs02_dps0__blk845_dn6, locals.var_fs02_dps0__blk845_dn7, locals.var_fs02_dps0__blk845_dn10, locals.var_fs02_dps0__blk845_dn11, locals.var_fs02_dps0__blk845_dn12, locals.var_fs02_dps0__blk845_dn17,)
    }
};
            locals.var_fs02_dps0__blk845 = assign26240_body10_e36079;
            locals.var_fs02_dps0__blk845_dn0 = assign26240_body10_e36079_d_n0;
            locals.var_fs02_dps0__blk845_dn2 = assign26240_body10_e36079_d_n2;
            locals.var_fs02_dps0__blk845_dn6 = assign26240_body10_e36079_d_n6;
            locals.var_fs02_dps0__blk845_dn7 = assign26240_body10_e36079_d_n7;
            locals.var_fs02_dps0__blk845_dn10 = assign26240_body10_e36079_d_n10;
            locals.var_fs02_dps0__blk845_dn11 = assign26240_body10_e36079_d_n11;
            locals.var_fs02_dps0__blk845_dn12 = assign26240_body10_e36079_d_n12;
            locals.var_fs02_dps0__blk845_dn17 = assign26240_body10_e36079_d_n17;
            locals.var_fs02_dps0__blk845_rv = 0.0;
            let assign26240_body11_e36082: f64 = if locals.var_chi__blk818 < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard865 = assign26240_body11_e36082;
            locals.var_guard865_rv = 0.0;
            let (assign26240_body12_e36101, assign26240_body12_e36101_d_n0, assign26240_body12_e36101_d_n2, assign26240_body12_e36101_d_n6, assign26240_body12_e36101_d_n7, assign26240_body12_e36101_d_n10, assign26240_body12_e36101_d_n11, assign26240_body12_e36101_d_n12, assign26240_body12_e36101_d_n17,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard864 == 0.0)) && (locals.var_guard865 != 0.0)) {
        let assign26240_body12_e36099: f64 = (locals.var_chi__blk818).exp();
        (assign26240_body12_e36099, (assign26240_body12_e36099 * locals.var_chi__blk818_dn0), (assign26240_body12_e36099 * locals.var_chi__blk818_dn2), (assign26240_body12_e36099 * locals.var_chi__blk818_dn6), (assign26240_body12_e36099 * locals.var_chi__blk818_dn7), (assign26240_body12_e36099 * locals.var_chi__blk818_dn10), (assign26240_body12_e36099 * locals.var_chi__blk818_dn11), (assign26240_body12_e36099 * locals.var_chi__blk818_dn12), (assign26240_body12_e36099 * locals.var_chi__blk818_dn17),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17,)
    }
};
            locals.var_exp_chi = assign26240_body12_e36101;
            locals.var_exp_chi_dn0 = assign26240_body12_e36101_d_n0;
            locals.var_exp_chi_dn2 = assign26240_body12_e36101_d_n2;
            locals.var_exp_chi_dn6 = assign26240_body12_e36101_d_n6;
            locals.var_exp_chi_dn7 = assign26240_body12_e36101_d_n7;
            locals.var_exp_chi_dn10 = assign26240_body12_e36101_d_n10;
            locals.var_exp_chi_dn11 = assign26240_body12_e36101_d_n11;
            locals.var_exp_chi_dn12 = assign26240_body12_e36101_d_n12;
            locals.var_exp_chi_dn17 = assign26240_body12_e36101_d_n17;
            locals.var_exp_chi_rv = 0.0;
            let (assign26240_body13_e36123, assign26240_body13_e36123_d_n0, assign26240_body13_e36123_d_n2, assign26240_body13_e36123_d_n6, assign26240_body13_e36123_d_n7, assign26240_body13_e36123_d_n10, assign26240_body13_e36123_d_n11, assign26240_body13_e36123_d_n12, assign26240_body13_e36123_d_n17,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard864 == 0.0)) && (locals.var_guard865 != 0.0)) {
        let assign26240_body13_e36120: f64 = (locals.var_exp_chi - 1.0);
        let assign26240_body13_e36121: f64 = (locals.var_cfs1__blk846 * assign26240_body13_e36120);
        (assign26240_body13_e36121, ((locals.var_cfs1__blk846_dn0 * assign26240_body13_e36120) + (locals.var_cfs1__blk846 * locals.var_exp_chi_dn0)), ((locals.var_cfs1__blk846_dn2 * assign26240_body13_e36120) + (locals.var_cfs1__blk846 * locals.var_exp_chi_dn2)), ((locals.var_cfs1__blk846_dn6 * assign26240_body13_e36120) + (locals.var_cfs1__blk846 * locals.var_exp_chi_dn6)), ((locals.var_cfs1__blk846_dn7 * assign26240_body13_e36120) + (locals.var_cfs1__blk846 * locals.var_exp_chi_dn7)), ((locals.var_cfs1__blk846_dn10 * assign26240_body13_e36120) + (locals.var_cfs1__blk846 * locals.var_exp_chi_dn10)), ((locals.var_cfs1__blk846_dn11 * assign26240_body13_e36120) + (locals.var_cfs1__blk846 * locals.var_exp_chi_dn11)), ((locals.var_cfs1__blk846_dn12 * assign26240_body13_e36120) + (locals.var_cfs1__blk846 * locals.var_exp_chi_dn12)), ((locals.var_cfs1__blk846_dn17 * assign26240_body13_e36120) + (locals.var_cfs1__blk846 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01__blk840, locals.var_fs01__blk840_dn0, locals.var_fs01__blk840_dn2, locals.var_fs01__blk840_dn6, locals.var_fs01__blk840_dn7, locals.var_fs01__blk840_dn10, locals.var_fs01__blk840_dn11, locals.var_fs01__blk840_dn12, locals.var_fs01__blk840_dn17,)
    }
};
            locals.var_fs01__blk840 = assign26240_body13_e36123;
            locals.var_fs01__blk840_dn0 = assign26240_body13_e36123_d_n0;
            locals.var_fs01__blk840_dn2 = assign26240_body13_e36123_d_n2;
            locals.var_fs01__blk840_dn6 = assign26240_body13_e36123_d_n6;
            locals.var_fs01__blk840_dn7 = assign26240_body13_e36123_d_n7;
            locals.var_fs01__blk840_dn10 = assign26240_body13_e36123_d_n10;
            locals.var_fs01__blk840_dn11 = assign26240_body13_e36123_d_n11;
            locals.var_fs01__blk840_dn12 = assign26240_body13_e36123_d_n12;
            locals.var_fs01__blk840_dn17 = assign26240_body13_e36123_d_n17;
            locals.var_fs01__blk840_rv = 0.0;
            let (assign26240_body14_e36145, assign26240_body14_e36145_d_n0, assign26240_body14_e36145_d_n2, assign26240_body14_e36145_d_n6, assign26240_body14_e36145_d_n7, assign26240_body14_e36145_d_n10, assign26240_body14_e36145_d_n11, assign26240_body14_e36145_d_n12, assign26240_body14_e36145_d_n17,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard864 == 0.0)) && (locals.var_guard865 != 0.0)) {
        let assign26240_body14_e36141: f64 = (locals.var_cfs1__blk846 * locals.var_beta);
        let assign26240_body14_e36143: f64 = (assign26240_body14_e36141 * locals.var_exp_chi);
        (assign26240_body14_e36143, (((locals.var_cfs1__blk846_dn0 * locals.var_beta) * locals.var_exp_chi) + (assign26240_body14_e36141 * locals.var_exp_chi_dn0)), (((locals.var_cfs1__blk846_dn2 * locals.var_beta) * locals.var_exp_chi) + (assign26240_body14_e36141 * locals.var_exp_chi_dn2)), (((locals.var_cfs1__blk846_dn6 * locals.var_beta) * locals.var_exp_chi) + (assign26240_body14_e36141 * locals.var_exp_chi_dn6)), (((locals.var_cfs1__blk846_dn7 * locals.var_beta) * locals.var_exp_chi) + (assign26240_body14_e36141 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1__blk846_dn10 * locals.var_beta) + (locals.var_cfs1__blk846 * locals.var_beta_dn10)) * locals.var_exp_chi) + (assign26240_body14_e36141 * locals.var_exp_chi_dn10)), (((locals.var_cfs1__blk846_dn11 * locals.var_beta) * locals.var_exp_chi) + (assign26240_body14_e36141 * locals.var_exp_chi_dn11)), (((locals.var_cfs1__blk846_dn12 * locals.var_beta) * locals.var_exp_chi) + (assign26240_body14_e36141 * locals.var_exp_chi_dn12)), (((locals.var_cfs1__blk846_dn17 * locals.var_beta) * locals.var_exp_chi) + (assign26240_body14_e36141 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk841, locals.var_fs01_dps0__blk841_dn0, locals.var_fs01_dps0__blk841_dn2, locals.var_fs01_dps0__blk841_dn6, locals.var_fs01_dps0__blk841_dn7, locals.var_fs01_dps0__blk841_dn10, locals.var_fs01_dps0__blk841_dn11, locals.var_fs01_dps0__blk841_dn12, locals.var_fs01_dps0__blk841_dn17,)
    }
};
            locals.var_fs01_dps0__blk841 = assign26240_body14_e36145;
            locals.var_fs01_dps0__blk841_dn0 = assign26240_body14_e36145_d_n0;
            locals.var_fs01_dps0__blk841_dn2 = assign26240_body14_e36145_d_n2;
            locals.var_fs01_dps0__blk841_dn6 = assign26240_body14_e36145_d_n6;
            locals.var_fs01_dps0__blk841_dn7 = assign26240_body14_e36145_d_n7;
            locals.var_fs01_dps0__blk841_dn10 = assign26240_body14_e36145_d_n10;
            locals.var_fs01_dps0__blk841_dn11 = assign26240_body14_e36145_d_n11;
            locals.var_fs01_dps0__blk841_dn12 = assign26240_body14_e36145_d_n12;
            locals.var_fs01_dps0__blk841_dn17 = assign26240_body14_e36145_d_n17;
            locals.var_fs01_dps0__blk841_rv = 0.0;
            let (assign26240_body15_e36167, assign26240_body15_e36167_d_n0, assign26240_body15_e36167_d_n2, assign26240_body15_e36167_d_n6, assign26240_body15_e36167_d_n7, assign26240_body15_e36167_d_n10, assign26240_body15_e36167_d_n11, assign26240_body15_e36167_d_n12, assign26240_body15_e36167_d_n17,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard864 == 0.0)) && (locals.var_guard865 == 0.0)) {
        let assign26240_body15_e36164: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign26240_body15_e36165: f64 = (assign26240_body15_e36164).exp();
        (assign26240_body15_e36165, (assign26240_body15_e36165 * (locals.var_beta * locals.var_ps0ld_dn0)), (assign26240_body15_e36165 * (locals.var_beta * locals.var_ps0ld_dn2)), (assign26240_body15_e36165 * (locals.var_beta * locals.var_ps0ld_dn6)), (assign26240_body15_e36165 * (locals.var_beta * locals.var_ps0ld_dn7)), (assign26240_body15_e36165 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign26240_body15_e36165 * (locals.var_beta * locals.var_ps0ld_dn11)), (assign26240_body15_e36165 * (locals.var_beta * locals.var_ps0ld_dn12)), (assign26240_body15_e36165 * (locals.var_beta * locals.var_ps0ld_dn17)),)
    } else {
        (locals.var_exp_bps0__blk847, locals.var_exp_bps0__blk847_dn0, locals.var_exp_bps0__blk847_dn2, locals.var_exp_bps0__blk847_dn6, locals.var_exp_bps0__blk847_dn7, locals.var_exp_bps0__blk847_dn10, locals.var_exp_bps0__blk847_dn11, locals.var_exp_bps0__blk847_dn12, locals.var_exp_bps0__blk847_dn17,)
    }
};
            locals.var_exp_bps0__blk847 = assign26240_body15_e36167;
            locals.var_exp_bps0__blk847_dn0 = assign26240_body15_e36167_d_n0;
            locals.var_exp_bps0__blk847_dn2 = assign26240_body15_e36167_d_n2;
            locals.var_exp_bps0__blk847_dn6 = assign26240_body15_e36167_d_n6;
            locals.var_exp_bps0__blk847_dn7 = assign26240_body15_e36167_d_n7;
            locals.var_exp_bps0__blk847_dn10 = assign26240_body15_e36167_d_n10;
            locals.var_exp_bps0__blk847_dn11 = assign26240_body15_e36167_d_n11;
            locals.var_exp_bps0__blk847_dn12 = assign26240_body15_e36167_d_n12;
            locals.var_exp_bps0__blk847_dn17 = assign26240_body15_e36167_d_n17;
            locals.var_exp_bps0__blk847_rv = 0.0;
            let (assign26240_body16_e36190, assign26240_body16_e36190_d_n0, assign26240_body16_e36190_d_n2, assign26240_body16_e36190_d_n6, assign26240_body16_e36190_d_n7, assign26240_body16_e36190_d_n10, assign26240_body16_e36190_d_n11, assign26240_body16_e36190_d_n12, assign26240_body16_e36190_d_n17,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard864 == 0.0)) && (locals.var_guard865 == 0.0)) {
        let assign26240_body16_e36187: f64 = (locals.var_exp_bps0__blk847 - locals.var_exp_bvbs__blk837);
        let assign26240_body16_e36188: f64 = (locals.var_cnst1over * assign26240_body16_e36187);
        (assign26240_body16_e36188, ((locals.var_cnst1over_dn0 * assign26240_body16_e36187) + (locals.var_cnst1over * (locals.var_exp_bps0__blk847_dn0 - locals.var_exp_bvbs__blk837_dn0))), ((locals.var_cnst1over_dn2 * assign26240_body16_e36187) + (locals.var_cnst1over * (locals.var_exp_bps0__blk847_dn2 - locals.var_exp_bvbs__blk837_dn2))), ((locals.var_cnst1over_dn6 * assign26240_body16_e36187) + (locals.var_cnst1over * (locals.var_exp_bps0__blk847_dn6 - locals.var_exp_bvbs__blk837_dn6))), ((locals.var_cnst1over_dn7 * assign26240_body16_e36187) + (locals.var_cnst1over * (locals.var_exp_bps0__blk847_dn7 - locals.var_exp_bvbs__blk837_dn7))), ((locals.var_cnst1over_dn10 * assign26240_body16_e36187) + (locals.var_cnst1over * (locals.var_exp_bps0__blk847_dn10 - locals.var_exp_bvbs__blk837_dn10))), ((locals.var_cnst1over_dn11 * assign26240_body16_e36187) + (locals.var_cnst1over * (locals.var_exp_bps0__blk847_dn11 - locals.var_exp_bvbs__blk837_dn11))), ((locals.var_cnst1over_dn12 * assign26240_body16_e36187) + (locals.var_cnst1over * (locals.var_exp_bps0__blk847_dn12 - locals.var_exp_bvbs__blk837_dn12))), ((locals.var_cnst1over_dn17 * assign26240_body16_e36187) + (locals.var_cnst1over * (locals.var_exp_bps0__blk847_dn17 - locals.var_exp_bvbs__blk837_dn17))),)
    } else {
        (locals.var_fs01__blk840, locals.var_fs01__blk840_dn0, locals.var_fs01__blk840_dn2, locals.var_fs01__blk840_dn6, locals.var_fs01__blk840_dn7, locals.var_fs01__blk840_dn10, locals.var_fs01__blk840_dn11, locals.var_fs01__blk840_dn12, locals.var_fs01__blk840_dn17,)
    }
};
            locals.var_fs01__blk840 = assign26240_body16_e36190;
            locals.var_fs01__blk840_dn0 = assign26240_body16_e36190_d_n0;
            locals.var_fs01__blk840_dn2 = assign26240_body16_e36190_d_n2;
            locals.var_fs01__blk840_dn6 = assign26240_body16_e36190_d_n6;
            locals.var_fs01__blk840_dn7 = assign26240_body16_e36190_d_n7;
            locals.var_fs01__blk840_dn10 = assign26240_body16_e36190_d_n10;
            locals.var_fs01__blk840_dn11 = assign26240_body16_e36190_d_n11;
            locals.var_fs01__blk840_dn12 = assign26240_body16_e36190_d_n12;
            locals.var_fs01__blk840_dn17 = assign26240_body16_e36190_d_n17;
            locals.var_fs01__blk840_rv = 0.0;
            let (assign26240_body17_e36213, assign26240_body17_e36213_d_n0, assign26240_body17_e36213_d_n2, assign26240_body17_e36213_d_n6, assign26240_body17_e36213_d_n7, assign26240_body17_e36213_d_n10, assign26240_body17_e36213_d_n11, assign26240_body17_e36213_d_n12, assign26240_body17_e36213_d_n17,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard864 == 0.0)) && (locals.var_guard865 == 0.0)) {
        let assign26240_body17_e36209: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign26240_body17_e36211: f64 = (assign26240_body17_e36209 * locals.var_exp_bps0__blk847);
        (assign26240_body17_e36211, (((locals.var_cnst1over_dn0 * locals.var_beta) * locals.var_exp_bps0__blk847) + (assign26240_body17_e36209 * locals.var_exp_bps0__blk847_dn0)), (((locals.var_cnst1over_dn2 * locals.var_beta) * locals.var_exp_bps0__blk847) + (assign26240_body17_e36209 * locals.var_exp_bps0__blk847_dn2)), (((locals.var_cnst1over_dn6 * locals.var_beta) * locals.var_exp_bps0__blk847) + (assign26240_body17_e36209 * locals.var_exp_bps0__blk847_dn6)), (((locals.var_cnst1over_dn7 * locals.var_beta) * locals.var_exp_bps0__blk847) + (assign26240_body17_e36209 * locals.var_exp_bps0__blk847_dn7)), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * locals.var_exp_bps0__blk847) + (assign26240_body17_e36209 * locals.var_exp_bps0__blk847_dn10)), (((locals.var_cnst1over_dn11 * locals.var_beta) * locals.var_exp_bps0__blk847) + (assign26240_body17_e36209 * locals.var_exp_bps0__blk847_dn11)), (((locals.var_cnst1over_dn12 * locals.var_beta) * locals.var_exp_bps0__blk847) + (assign26240_body17_e36209 * locals.var_exp_bps0__blk847_dn12)), (((locals.var_cnst1over_dn17 * locals.var_beta) * locals.var_exp_bps0__blk847) + (assign26240_body17_e36209 * locals.var_exp_bps0__blk847_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk841, locals.var_fs01_dps0__blk841_dn0, locals.var_fs01_dps0__blk841_dn2, locals.var_fs01_dps0__blk841_dn6, locals.var_fs01_dps0__blk841_dn7, locals.var_fs01_dps0__blk841_dn10, locals.var_fs01_dps0__blk841_dn11, locals.var_fs01_dps0__blk841_dn12, locals.var_fs01_dps0__blk841_dn17,)
    }
};
            locals.var_fs01_dps0__blk841 = assign26240_body17_e36213;
            locals.var_fs01_dps0__blk841_dn0 = assign26240_body17_e36213_d_n0;
            locals.var_fs01_dps0__blk841_dn2 = assign26240_body17_e36213_d_n2;
            locals.var_fs01_dps0__blk841_dn6 = assign26240_body17_e36213_d_n6;
            locals.var_fs01_dps0__blk841_dn7 = assign26240_body17_e36213_d_n7;
            locals.var_fs01_dps0__blk841_dn10 = assign26240_body17_e36213_d_n10;
            locals.var_fs01_dps0__blk841_dn11 = assign26240_body17_e36213_d_n11;
            locals.var_fs01_dps0__blk841_dn12 = assign26240_body17_e36213_d_n12;
            locals.var_fs01_dps0__blk841_dn17 = assign26240_body17_e36213_d_n17;
            locals.var_fs01_dps0__blk841_rv = 0.0;
            let (assign26240_body18_e36234, assign26240_body18_e36234_d_n0, assign26240_body18_e36234_d_n2, assign26240_body18_e36234_d_n6, assign26240_body18_e36234_d_n7, assign26240_body18_e36234_d_n10, assign26240_body18_e36234_d_n11, assign26240_body18_e36234_d_n12, assign26240_body18_e36234_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard864 == 0.0)) {
        let assign26240_body18_e36229: f64 = (locals.var_chi__blk818 - 1.0);
        let assign26240_body18_e36231: f64 = (assign26240_body18_e36229 + locals.var_fs01__blk840);
        let assign26240_body18_e36232: f64 = (assign26240_body18_e36231).sqrt();
        (assign26240_body18_e36232, ((locals.var_chi__blk818_dn0 + locals.var_fs01__blk840_dn0) / (2.0 * assign26240_body18_e36232)), ((locals.var_chi__blk818_dn2 + locals.var_fs01__blk840_dn2) / (2.0 * assign26240_body18_e36232)), ((locals.var_chi__blk818_dn6 + locals.var_fs01__blk840_dn6) / (2.0 * assign26240_body18_e36232)), ((locals.var_chi__blk818_dn7 + locals.var_fs01__blk840_dn7) / (2.0 * assign26240_body18_e36232)), ((locals.var_chi__blk818_dn10 + locals.var_fs01__blk840_dn10) / (2.0 * assign26240_body18_e36232)), ((locals.var_chi__blk818_dn11 + locals.var_fs01__blk840_dn11) / (2.0 * assign26240_body18_e36232)), ((locals.var_chi__blk818_dn12 + locals.var_fs01__blk840_dn12) / (2.0 * assign26240_body18_e36232)), ((locals.var_chi__blk818_dn17 + locals.var_fs01__blk840_dn17) / (2.0 * assign26240_body18_e36232)),)
    } else {
        (locals.var_fs02__blk844, locals.var_fs02__blk844_dn0, locals.var_fs02__blk844_dn2, locals.var_fs02__blk844_dn6, locals.var_fs02__blk844_dn7, locals.var_fs02__blk844_dn10, locals.var_fs02__blk844_dn11, locals.var_fs02__blk844_dn12, locals.var_fs02__blk844_dn17,)
    }
};
            locals.var_fs02__blk844 = assign26240_body18_e36234;
            locals.var_fs02__blk844_dn0 = assign26240_body18_e36234_d_n0;
            locals.var_fs02__blk844_dn2 = assign26240_body18_e36234_d_n2;
            locals.var_fs02__blk844_dn6 = assign26240_body18_e36234_d_n6;
            locals.var_fs02__blk844_dn7 = assign26240_body18_e36234_d_n7;
            locals.var_fs02__blk844_dn10 = assign26240_body18_e36234_d_n10;
            locals.var_fs02__blk844_dn11 = assign26240_body18_e36234_d_n11;
            locals.var_fs02__blk844_dn12 = assign26240_body18_e36234_d_n12;
            locals.var_fs02__blk844_dn17 = assign26240_body18_e36234_d_n17;
            locals.var_fs02__blk844_rv = 0.0;
            let (assign26240_body19_e36256, assign26240_body19_e36256_d_n0, assign26240_body19_e36256_d_n2, assign26240_body19_e36256_d_n6, assign26240_body19_e36256_d_n7, assign26240_body19_e36256_d_n10, assign26240_body19_e36256_d_n11, assign26240_body19_e36256_d_n12, assign26240_body19_e36256_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard864 == 0.0)) {
        let assign26240_body19_e36250: f64 = (locals.var_beta + locals.var_fs01_dps0__blk841);
        let assign26240_body19_e36252: f64 = (assign26240_body19_e36250 / locals.var_fs02__blk844);
        let assign26240_body19_e36254: f64 = (assign26240_body19_e36252 * 0.5);
        (assign26240_body19_e36254, ((((locals.var_fs01_dps0__blk841_dn0 * locals.var_fs02__blk844) - (assign26240_body19_e36250 * locals.var_fs02__blk844_dn0)) / (locals.var_fs02__blk844 * locals.var_fs02__blk844)) * 0.5), ((((locals.var_fs01_dps0__blk841_dn2 * locals.var_fs02__blk844) - (assign26240_body19_e36250 * locals.var_fs02__blk844_dn2)) / (locals.var_fs02__blk844 * locals.var_fs02__blk844)) * 0.5), ((((locals.var_fs01_dps0__blk841_dn6 * locals.var_fs02__blk844) - (assign26240_body19_e36250 * locals.var_fs02__blk844_dn6)) / (locals.var_fs02__blk844 * locals.var_fs02__blk844)) * 0.5), ((((locals.var_fs01_dps0__blk841_dn7 * locals.var_fs02__blk844) - (assign26240_body19_e36250 * locals.var_fs02__blk844_dn7)) / (locals.var_fs02__blk844 * locals.var_fs02__blk844)) * 0.5), (((((locals.var_beta_dn10 + locals.var_fs01_dps0__blk841_dn10) * locals.var_fs02__blk844) - (assign26240_body19_e36250 * locals.var_fs02__blk844_dn10)) / (locals.var_fs02__blk844 * locals.var_fs02__blk844)) * 0.5), ((((locals.var_fs01_dps0__blk841_dn11 * locals.var_fs02__blk844) - (assign26240_body19_e36250 * locals.var_fs02__blk844_dn11)) / (locals.var_fs02__blk844 * locals.var_fs02__blk844)) * 0.5), ((((locals.var_fs01_dps0__blk841_dn12 * locals.var_fs02__blk844) - (assign26240_body19_e36250 * locals.var_fs02__blk844_dn12)) / (locals.var_fs02__blk844 * locals.var_fs02__blk844)) * 0.5), ((((locals.var_fs01_dps0__blk841_dn17 * locals.var_fs02__blk844) - (assign26240_body19_e36250 * locals.var_fs02__blk844_dn17)) / (locals.var_fs02__blk844 * locals.var_fs02__blk844)) * 0.5),)
    } else {
        (locals.var_fs02_dps0__blk845, locals.var_fs02_dps0__blk845_dn0, locals.var_fs02_dps0__blk845_dn2, locals.var_fs02_dps0__blk845_dn6, locals.var_fs02_dps0__blk845_dn7, locals.var_fs02_dps0__blk845_dn10, locals.var_fs02_dps0__blk845_dn11, locals.var_fs02_dps0__blk845_dn12, locals.var_fs02_dps0__blk845_dn17,)
    }
};
            locals.var_fs02_dps0__blk845 = assign26240_body19_e36256;
            locals.var_fs02_dps0__blk845_dn0 = assign26240_body19_e36256_d_n0;
            locals.var_fs02_dps0__blk845_dn2 = assign26240_body19_e36256_d_n2;
            locals.var_fs02_dps0__blk845_dn6 = assign26240_body19_e36256_d_n6;
            locals.var_fs02_dps0__blk845_dn7 = assign26240_body19_e36256_d_n7;
            locals.var_fs02_dps0__blk845_dn10 = assign26240_body19_e36256_d_n10;
            locals.var_fs02_dps0__blk845_dn11 = assign26240_body19_e36256_d_n11;
            locals.var_fs02_dps0__blk845_dn12 = assign26240_body19_e36256_d_n12;
            locals.var_fs02_dps0__blk845_dn17 = assign26240_body19_e36256_d_n17;
            locals.var_fs02_dps0__blk845_rv = 0.0;
            let (assign26240_body20_e36275, assign26240_body20_e36275_d_n0, assign26240_body20_e36275_d_n2, assign26240_body20_e36275_d_n6, assign26240_body20_e36275_d_n7, assign26240_body20_e36275_d_n10, assign26240_body20_e36275_d_n11, assign26240_body20_e36275_d_n12, assign26240_body20_e36275_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let assign26240_body20_e36269: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign26240_body20_e36272: f64 = (locals.var_fac1__blk804 * locals.var_fs02__blk844);
        let assign26240_body20_e36273: f64 = (assign26240_body20_e36269 - assign26240_body20_e36272);
        (assign26240_body20_e36273, ((locals.var_vgpld_dn0 - locals.var_ps0ld_dn0) - ((locals.var_fac1__blk804_dn0 * locals.var_fs02__blk844) + (locals.var_fac1__blk804 * locals.var_fs02__blk844_dn0))), ((locals.var_vgpld_dn2 - locals.var_ps0ld_dn2) - ((locals.var_fac1__blk804_dn2 * locals.var_fs02__blk844) + (locals.var_fac1__blk804 * locals.var_fs02__blk844_dn2))), ((locals.var_vgpld_dn6 - locals.var_ps0ld_dn6) - ((locals.var_fac1__blk804_dn6 * locals.var_fs02__blk844) + (locals.var_fac1__blk804 * locals.var_fs02__blk844_dn6))), ((locals.var_vgpld_dn7 - locals.var_ps0ld_dn7) - ((locals.var_fac1__blk804_dn7 * locals.var_fs02__blk844) + (locals.var_fac1__blk804 * locals.var_fs02__blk844_dn7))), ((locals.var_vgpld_dn10 - locals.var_ps0ld_dn10) - ((locals.var_fac1__blk804_dn10 * locals.var_fs02__blk844) + (locals.var_fac1__blk804 * locals.var_fs02__blk844_dn10))), ((locals.var_vgpld_dn11 - locals.var_ps0ld_dn11) - ((locals.var_fac1__blk804_dn11 * locals.var_fs02__blk844) + (locals.var_fac1__blk804 * locals.var_fs02__blk844_dn11))), ((locals.var_vgpld_dn12 - locals.var_ps0ld_dn12) - ((locals.var_fac1__blk804_dn12 * locals.var_fs02__blk844) + (locals.var_fac1__blk804 * locals.var_fs02__blk844_dn12))), ((locals.var_vgpld_dn17 - locals.var_ps0ld_dn17) - ((locals.var_fac1__blk804_dn17 * locals.var_fs02__blk844) + (locals.var_fac1__blk804 * locals.var_fs02__blk844_dn17))),)
    } else {
        (locals.var_fs0__blk848, locals.var_fs0__blk848_dn0, locals.var_fs0__blk848_dn2, locals.var_fs0__blk848_dn6, locals.var_fs0__blk848_dn7, locals.var_fs0__blk848_dn10, locals.var_fs0__blk848_dn11, locals.var_fs0__blk848_dn12, locals.var_fs0__blk848_dn17,)
    }
};
            locals.var_fs0__blk848 = assign26240_body20_e36275;
            locals.var_fs0__blk848_dn0 = assign26240_body20_e36275_d_n0;
            locals.var_fs0__blk848_dn2 = assign26240_body20_e36275_d_n2;
            locals.var_fs0__blk848_dn6 = assign26240_body20_e36275_d_n6;
            locals.var_fs0__blk848_dn7 = assign26240_body20_e36275_d_n7;
            locals.var_fs0__blk848_dn10 = assign26240_body20_e36275_d_n10;
            locals.var_fs0__blk848_dn11 = assign26240_body20_e36275_d_n11;
            locals.var_fs0__blk848_dn12 = assign26240_body20_e36275_d_n12;
            locals.var_fs0__blk848_dn17 = assign26240_body20_e36275_d_n17;
            locals.var_fs0__blk848_rv = 0.0;
            let (assign26240_body21_e36293, assign26240_body21_e36293_d_n0, assign26240_body21_e36293_d_n2, assign26240_body21_e36293_d_n6, assign26240_body21_e36293_d_n7, assign26240_body21_e36293_d_n10, assign26240_body21_e36293_d_n11, assign26240_body21_e36293_d_n12, assign26240_body21_e36293_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let assign26240_body21_e36287: f64 = (-1.0);
        let assign26240_body21_e36290: f64 = (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845);
        let assign26240_body21_e36291: f64 = (assign26240_body21_e36287 - assign26240_body21_e36290);
        (assign26240_body21_e36291, (-((locals.var_fac1__blk804_dn0 * locals.var_fs02_dps0__blk845) + (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845_dn0))), (-((locals.var_fac1__blk804_dn2 * locals.var_fs02_dps0__blk845) + (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845_dn2))), (-((locals.var_fac1__blk804_dn6 * locals.var_fs02_dps0__blk845) + (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845_dn6))), (-((locals.var_fac1__blk804_dn7 * locals.var_fs02_dps0__blk845) + (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845_dn7))), (-((locals.var_fac1__blk804_dn10 * locals.var_fs02_dps0__blk845) + (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845_dn10))), (-((locals.var_fac1__blk804_dn11 * locals.var_fs02_dps0__blk845) + (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845_dn11))), (-((locals.var_fac1__blk804_dn12 * locals.var_fs02_dps0__blk845) + (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845_dn12))), (-((locals.var_fac1__blk804_dn17 * locals.var_fs02_dps0__blk845) + (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845_dn17))),)
    } else {
        (locals.var_fs0_dps0__blk849, locals.var_fs0_dps0__blk849_dn0, locals.var_fs0_dps0__blk849_dn2, locals.var_fs0_dps0__blk849_dn6, locals.var_fs0_dps0__blk849_dn7, locals.var_fs0_dps0__blk849_dn10, locals.var_fs0_dps0__blk849_dn11, locals.var_fs0_dps0__blk849_dn12, locals.var_fs0_dps0__blk849_dn17,)
    }
};
            locals.var_fs0_dps0__blk849 = assign26240_body21_e36293;
            locals.var_fs0_dps0__blk849_dn0 = assign26240_body21_e36293_d_n0;
            locals.var_fs0_dps0__blk849_dn2 = assign26240_body21_e36293_d_n2;
            locals.var_fs0_dps0__blk849_dn6 = assign26240_body21_e36293_d_n6;
            locals.var_fs0_dps0__blk849_dn7 = assign26240_body21_e36293_d_n7;
            locals.var_fs0_dps0__blk849_dn10 = assign26240_body21_e36293_d_n10;
            locals.var_fs0_dps0__blk849_dn11 = assign26240_body21_e36293_d_n11;
            locals.var_fs0_dps0__blk849_dn12 = assign26240_body21_e36293_d_n12;
            locals.var_fs0_dps0__blk849_dn17 = assign26240_body21_e36293_d_n17;
            locals.var_fs0_dps0__blk849_rv = 0.0;
            let assign26240_body22_e36296: f64 = if locals.var_flg_conv__blk791 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard866 = assign26240_body22_e36296;
            locals.var_guard866_rv = 0.0;
            let (assign26240_body23_e36315,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard866 != 0.0)) {
        let assign26240_body23_e36311: f64 = (2.0 * 20.0);
        let assign26240_body23_e36313: f64 = (assign26240_body23_e36311 + 1.0);
        (assign26240_body23_e36313,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign26240_body23_e36315;
            locals.var_lp_s0_rv = 0.0;
            let (assign26240_body24_e36334, assign26240_body24_e36334_d_n0, assign26240_body24_e36334_d_n2, assign26240_body24_e36334_d_n6, assign26240_body24_e36334_d_n7, assign26240_body24_e36334_d_n10, assign26240_body24_e36334_d_n11, assign26240_body24_e36334_d_n12, assign26240_body24_e36334_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard866 == 0.0)) {
        let assign26240_body24_e36330: f64 = (-locals.var_fs0__blk848);
        let assign26240_body24_e36332: f64 = (assign26240_body24_e36330 / locals.var_fs0_dps0__blk849);
        (assign26240_body24_e36332, ((((-locals.var_fs0__blk848_dn0) * locals.var_fs0_dps0__blk849) - (assign26240_body24_e36330 * locals.var_fs0_dps0__blk849_dn0)) / (locals.var_fs0_dps0__blk849 * locals.var_fs0_dps0__blk849)), ((((-locals.var_fs0__blk848_dn2) * locals.var_fs0_dps0__blk849) - (assign26240_body24_e36330 * locals.var_fs0_dps0__blk849_dn2)) / (locals.var_fs0_dps0__blk849 * locals.var_fs0_dps0__blk849)), ((((-locals.var_fs0__blk848_dn6) * locals.var_fs0_dps0__blk849) - (assign26240_body24_e36330 * locals.var_fs0_dps0__blk849_dn6)) / (locals.var_fs0_dps0__blk849 * locals.var_fs0_dps0__blk849)), ((((-locals.var_fs0__blk848_dn7) * locals.var_fs0_dps0__blk849) - (assign26240_body24_e36330 * locals.var_fs0_dps0__blk849_dn7)) / (locals.var_fs0_dps0__blk849 * locals.var_fs0_dps0__blk849)), ((((-locals.var_fs0__blk848_dn10) * locals.var_fs0_dps0__blk849) - (assign26240_body24_e36330 * locals.var_fs0_dps0__blk849_dn10)) / (locals.var_fs0_dps0__blk849 * locals.var_fs0_dps0__blk849)), ((((-locals.var_fs0__blk848_dn11) * locals.var_fs0_dps0__blk849) - (assign26240_body24_e36330 * locals.var_fs0_dps0__blk849_dn11)) / (locals.var_fs0_dps0__blk849 * locals.var_fs0_dps0__blk849)), ((((-locals.var_fs0__blk848_dn12) * locals.var_fs0_dps0__blk849) - (assign26240_body24_e36330 * locals.var_fs0_dps0__blk849_dn12)) / (locals.var_fs0_dps0__blk849 * locals.var_fs0_dps0__blk849)), ((((-locals.var_fs0__blk848_dn17) * locals.var_fs0_dps0__blk849) - (assign26240_body24_e36330 * locals.var_fs0_dps0__blk849_dn17)) / (locals.var_fs0_dps0__blk849 * locals.var_fs0_dps0__blk849)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign26240_body24_e36334;
            locals.var_dps0_dn0 = assign26240_body24_e36334_d_n0;
            locals.var_dps0_dn2 = assign26240_body24_e36334_d_n2;
            locals.var_dps0_dn6 = assign26240_body24_e36334_d_n6;
            locals.var_dps0_dn7 = assign26240_body24_e36334_d_n7;
            locals.var_dps0_dn10 = assign26240_body24_e36334_d_n10;
            locals.var_dps0_dn11 = assign26240_body24_e36334_d_n11;
            locals.var_dps0_dn12 = assign26240_body24_e36334_d_n12;
            locals.var_dps0_dn17 = assign26240_body24_e36334_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign26240_body25_e36363, assign26240_body25_e36363_d_n0, assign26240_body25_e36363_d_n2, assign26240_body25_e36363_d_n6, assign26240_body25_e36363_d_n7, assign26240_body25_e36363_d_n10, assign26240_body25_e36363_d_n11, assign26240_body25_e36363_d_n12, assign26240_body25_e36363_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard866 == 0.0)) {
        let assign26240_body25_e36350: f64 = (0.5 * 0.1);
        let assign26240_body25_e36354: f64 = (locals.var_ps0ld).abs();
        let (assign26240_body25_e36359, assign26240_body25_e36359_d_n0, assign26240_body25_e36359_d_n2, assign26240_body25_e36359_d_n6, assign26240_body25_e36359_d_n7, assign26240_body25_e36359_d_n10, assign26240_body25_e36359_d_n11, assign26240_body25_e36359_d_n12, assign26240_body25_e36359_d_n17,) = {
            if (1.0 >= assign26240_body25_e36354) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign26240_body25_e36358: f64 = (locals.var_ps0ld).abs();
                (assign26240_body25_e36358, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn12 } else { (-locals.var_ps0ld_dn12) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn17 } else { (-locals.var_ps0ld_dn17) },)
            }
        };
        let assign26240_body25_e36360: f64 = (1.0 + assign26240_body25_e36359);
        let assign26240_body25_e36361: f64 = (assign26240_body25_e36350 * assign26240_body25_e36360);
        (assign26240_body25_e36361, (assign26240_body25_e36350 * assign26240_body25_e36359_d_n0), (assign26240_body25_e36350 * assign26240_body25_e36359_d_n2), (assign26240_body25_e36350 * assign26240_body25_e36359_d_n6), (assign26240_body25_e36350 * assign26240_body25_e36359_d_n7), (assign26240_body25_e36350 * assign26240_body25_e36359_d_n10), (assign26240_body25_e36350 * assign26240_body25_e36359_d_n11), (assign26240_body25_e36350 * assign26240_body25_e36359_d_n12), (assign26240_body25_e36350 * assign26240_body25_e36359_d_n17),)
    } else {
        (locals.var_dplim__blk850, locals.var_dplim__blk850_dn0, locals.var_dplim__blk850_dn2, locals.var_dplim__blk850_dn6, locals.var_dplim__blk850_dn7, locals.var_dplim__blk850_dn10, locals.var_dplim__blk850_dn11, locals.var_dplim__blk850_dn12, locals.var_dplim__blk850_dn17,)
    }
};
            locals.var_dplim__blk850 = assign26240_body25_e36363;
            locals.var_dplim__blk850_dn0 = assign26240_body25_e36363_d_n0;
            locals.var_dplim__blk850_dn2 = assign26240_body25_e36363_d_n2;
            locals.var_dplim__blk850_dn6 = assign26240_body25_e36363_d_n6;
            locals.var_dplim__blk850_dn7 = assign26240_body25_e36363_d_n7;
            locals.var_dplim__blk850_dn10 = assign26240_body25_e36363_d_n10;
            locals.var_dplim__blk850_dn11 = assign26240_body25_e36363_d_n11;
            locals.var_dplim__blk850_dn12 = assign26240_body25_e36363_d_n12;
            locals.var_dplim__blk850_dn17 = assign26240_body25_e36363_d_n17;
            locals.var_dplim__blk850_rv = 0.0;
            let assign26240_body26_e36365: f64 = (locals.var_dps0).abs();
            let assign26240_body26_e36367: f64 = if assign26240_body26_e36365 > locals.var_dplim__blk850 { 1.0 } else { 0.0 };
            locals.var_guard867 = assign26240_body26_e36367;
            locals.var_guard867_rv = 0.0;
            let (assign26240_body27_e36393, assign26240_body27_e36393_d_n0, assign26240_body27_e36393_d_n2, assign26240_body27_e36393_d_n6, assign26240_body27_e36393_d_n7, assign26240_body27_e36393_d_n10, assign26240_body27_e36393_d_n11, assign26240_body27_e36393_d_n12, assign26240_body27_e36393_d_n17,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard866 == 0.0)) && (locals.var_guard867 != 0.0)) {
        let (assign26240_body27_e36390,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign26240_body27_e36389: f64 = (-1.0);
                (assign26240_body27_e36389,)
            }
        };
        let assign26240_body27_e36391: f64 = (locals.var_dplim__blk850 * assign26240_body27_e36390);
        (assign26240_body27_e36391, (locals.var_dplim__blk850_dn0 * assign26240_body27_e36390), (locals.var_dplim__blk850_dn2 * assign26240_body27_e36390), (locals.var_dplim__blk850_dn6 * assign26240_body27_e36390), (locals.var_dplim__blk850_dn7 * assign26240_body27_e36390), (locals.var_dplim__blk850_dn10 * assign26240_body27_e36390), (locals.var_dplim__blk850_dn11 * assign26240_body27_e36390), (locals.var_dplim__blk850_dn12 * assign26240_body27_e36390), (locals.var_dplim__blk850_dn17 * assign26240_body27_e36390),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign26240_body27_e36393;
            locals.var_dps0_dn0 = assign26240_body27_e36393_d_n0;
            locals.var_dps0_dn2 = assign26240_body27_e36393_d_n2;
            locals.var_dps0_dn6 = assign26240_body27_e36393_d_n6;
            locals.var_dps0_dn7 = assign26240_body27_e36393_d_n7;
            locals.var_dps0_dn10 = assign26240_body27_e36393_d_n10;
            locals.var_dps0_dn11 = assign26240_body27_e36393_d_n11;
            locals.var_dps0_dn12 = assign26240_body27_e36393_d_n12;
            locals.var_dps0_dn17 = assign26240_body27_e36393_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign26240_body28_e36411, assign26240_body28_e36411_d_n0, assign26240_body28_e36411_d_n2, assign26240_body28_e36411_d_n6, assign26240_body28_e36411_d_n7, assign26240_body28_e36411_d_n10, assign26240_body28_e36411_d_n11, assign26240_body28_e36411_d_n12, assign26240_body28_e36411_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard866 == 0.0)) {
        let assign26240_body28_e36409: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign26240_body28_e36409, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn12 + locals.var_dps0_dn12), (locals.var_ps0ld_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
            locals.var_ps0ld = assign26240_body28_e36411;
            locals.var_ps0ld_dn0 = assign26240_body28_e36411_d_n0;
            locals.var_ps0ld_dn2 = assign26240_body28_e36411_d_n2;
            locals.var_ps0ld_dn6 = assign26240_body28_e36411_d_n6;
            locals.var_ps0ld_dn7 = assign26240_body28_e36411_d_n7;
            locals.var_ps0ld_dn10 = assign26240_body28_e36411_d_n10;
            locals.var_ps0ld_dn11 = assign26240_body28_e36411_d_n11;
            locals.var_ps0ld_dn12 = assign26240_body28_e36411_d_n12;
            locals.var_ps0ld_dn17 = assign26240_body28_e36411_d_n17;
            locals.var_ps0ld_rv = 0.0;
            let assign26240_body29_e36413: f64 = (locals.var_dps0).abs();
            let assign26240_body29_e36417: f64 = (locals.var_fs0__blk848).abs();
            let assign26240_body29_e36420: f64 = if ((assign26240_body29_e36413 <= 5e-12) && (assign26240_body29_e36417 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard868 = assign26240_body29_e36420;
            locals.var_guard868_rv = 0.0;
            let (assign26240_body30_e36438,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard866 == 0.0)) && (locals.var_guard868 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv__blk791,)
    }
};
            locals.var_flg_conv__blk791 = assign26240_body30_e36438;
            locals.var_flg_conv__blk791_rv = 0.0;
            let (assign26240_body31_e36453,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let assign26240_body31_e36451: f64 = (locals.var_lp_s0 + 1.0);
        (assign26240_body31_e36451,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign26240_body31_e36453;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_94(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign26260_e36459: f64 = if locals.var_chi__blk818 < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard870 = assign26260_e36459;
        locals.var_guard870_rv = 0.0;

        let (assign26300_e36518, assign26300_e36518_d_n0, assign26300_e36518_d_n2, assign26300_e36518_d_n6, assign26300_e36518_d_n7, assign26300_e36518_d_n10, assign26300_e36518_d_n11, assign26300_e36518_d_n12, assign26300_e36518_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard870 != 0.0)) {
        let assign26300_e36512: f64 = (locals.var_fb__blk842 * locals.var_fb__blk842);
        let assign26300_e36515: f64 = (10.0 * 2.220446049250313e-16);
        let assign26300_e36516: f64 = (assign26300_e36512 + assign26300_e36515);
        (assign26300_e36516, ((locals.var_fb__blk842_dn0 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn0)), ((locals.var_fb__blk842_dn2 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn2)), ((locals.var_fb__blk842_dn6 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn6)), ((locals.var_fb__blk842_dn7 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn7)), ((locals.var_fb__blk842_dn10 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn10)), ((locals.var_fb__blk842_dn11 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn11)), ((locals.var_fb__blk842_dn12 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn12)), ((locals.var_fb__blk842_dn17 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn17)),)
    } else {
        (locals.var_xi0__blk851, locals.var_xi0__blk851_dn0, locals.var_xi0__blk851_dn2, locals.var_xi0__blk851_dn6, locals.var_xi0__blk851_dn7, locals.var_xi0__blk851_dn10, locals.var_xi0__blk851_dn11, locals.var_xi0__blk851_dn12, locals.var_xi0__blk851_dn17,)
    }
};
        locals.var_xi0__blk851 = assign26300_e36518;
        locals.var_xi0__blk851_dn0 = assign26300_e36518_d_n0;
        locals.var_xi0__blk851_dn2 = assign26300_e36518_d_n2;
        locals.var_xi0__blk851_dn6 = assign26300_e36518_d_n6;
        locals.var_xi0__blk851_dn7 = assign26300_e36518_d_n7;
        locals.var_xi0__blk851_dn10 = assign26300_e36518_d_n10;
        locals.var_xi0__blk851_dn11 = assign26300_e36518_d_n11;
        locals.var_xi0__blk851_dn12 = assign26300_e36518_d_n12;
        locals.var_xi0__blk851_dn17 = assign26300_e36518_d_n17;
        locals.var_xi0__blk851_rv = 0.0;

        let (assign26310_e36537, assign26310_e36537_d_n0, assign26310_e36537_d_n2, assign26310_e36537_d_n6, assign26310_e36537_d_n7, assign26310_e36537_d_n10, assign26310_e36537_d_n11, assign26310_e36537_d_n12, assign26310_e36537_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard870 != 0.0)) {
        let assign26310_e36534: f64 = (10.0 * 2.220446049250313e-16);
        let assign26310_e36535: f64 = (locals.var_fb__blk842 + assign26310_e36534);
        (assign26310_e36535, locals.var_fb__blk842_dn0, locals.var_fb__blk842_dn2, locals.var_fb__blk842_dn6, locals.var_fb__blk842_dn7, locals.var_fb__blk842_dn10, locals.var_fb__blk842_dn11, locals.var_fb__blk842_dn12, locals.var_fb__blk842_dn17,)
    } else {
        (locals.var_xi0p12__blk852, locals.var_xi0p12__blk852_dn0, locals.var_xi0p12__blk852_dn2, locals.var_xi0p12__blk852_dn6, locals.var_xi0p12__blk852_dn7, locals.var_xi0p12__blk852_dn10, locals.var_xi0p12__blk852_dn11, locals.var_xi0p12__blk852_dn12, locals.var_xi0p12__blk852_dn17,)
    }
};
        locals.var_xi0p12__blk852 = assign26310_e36537;
        locals.var_xi0p12__blk852_dn0 = assign26310_e36537_d_n0;
        locals.var_xi0p12__blk852_dn2 = assign26310_e36537_d_n2;
        locals.var_xi0p12__blk852_dn6 = assign26310_e36537_d_n6;
        locals.var_xi0p12__blk852_dn7 = assign26310_e36537_d_n7;
        locals.var_xi0p12__blk852_dn10 = assign26310_e36537_d_n10;
        locals.var_xi0p12__blk852_dn11 = assign26310_e36537_d_n11;
        locals.var_xi0p12__blk852_dn12 = assign26310_e36537_d_n12;
        locals.var_xi0p12__blk852_dn17 = assign26310_e36537_d_n17;
        locals.var_xi0p12__blk852_rv = 0.0;

        let (assign26330_e36571, assign26330_e36571_d_n0, assign26330_e36571_d_n2, assign26330_e36571_d_n6, assign26330_e36571_d_n7, assign26330_e36571_d_n10, assign26330_e36571_d_n11, assign26330_e36571_d_n12, assign26330_e36571_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard870 == 0.0)) {
        let assign26330_e36569: f64 = (locals.var_chi__blk818 - 1.0);
        (assign26330_e36569, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    } else {
        (locals.var_xi0__blk851, locals.var_xi0__blk851_dn0, locals.var_xi0__blk851_dn2, locals.var_xi0__blk851_dn6, locals.var_xi0__blk851_dn7, locals.var_xi0__blk851_dn10, locals.var_xi0__blk851_dn11, locals.var_xi0__blk851_dn12, locals.var_xi0__blk851_dn17,)
    }
};
        locals.var_xi0__blk851 = assign26330_e36571;
        locals.var_xi0__blk851_dn0 = assign26330_e36571_d_n0;
        locals.var_xi0__blk851_dn2 = assign26330_e36571_d_n2;
        locals.var_xi0__blk851_dn6 = assign26330_e36571_d_n6;
        locals.var_xi0__blk851_dn7 = assign26330_e36571_d_n7;
        locals.var_xi0__blk851_dn10 = assign26330_e36571_d_n10;
        locals.var_xi0__blk851_dn11 = assign26330_e36571_d_n11;
        locals.var_xi0__blk851_dn12 = assign26330_e36571_d_n12;
        locals.var_xi0__blk851_dn17 = assign26330_e36571_d_n17;
        locals.var_xi0__blk851_rv = 0.0;

        let (assign26340_e36588, assign26340_e36588_d_n0, assign26340_e36588_d_n2, assign26340_e36588_d_n6, assign26340_e36588_d_n7, assign26340_e36588_d_n10, assign26340_e36588_d_n11, assign26340_e36588_d_n12, assign26340_e36588_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) && (locals.var_guard870 == 0.0)) {
        let assign26340_e36586: f64 = (locals.var_xi0__blk851).sqrt();
        (assign26340_e36586, (locals.var_xi0__blk851_dn0 / (2.0 * assign26340_e36586)), (locals.var_xi0__blk851_dn2 / (2.0 * assign26340_e36586)), (locals.var_xi0__blk851_dn6 / (2.0 * assign26340_e36586)), (locals.var_xi0__blk851_dn7 / (2.0 * assign26340_e36586)), (locals.var_xi0__blk851_dn10 / (2.0 * assign26340_e36586)), (locals.var_xi0__blk851_dn11 / (2.0 * assign26340_e36586)), (locals.var_xi0__blk851_dn12 / (2.0 * assign26340_e36586)), (locals.var_xi0__blk851_dn17 / (2.0 * assign26340_e36586)),)
    } else {
        (locals.var_xi0p12__blk852, locals.var_xi0p12__blk852_dn0, locals.var_xi0p12__blk852_dn2, locals.var_xi0p12__blk852_dn6, locals.var_xi0p12__blk852_dn7, locals.var_xi0p12__blk852_dn10, locals.var_xi0p12__blk852_dn11, locals.var_xi0p12__blk852_dn12, locals.var_xi0p12__blk852_dn17,)
    }
};
        locals.var_xi0p12__blk852 = assign26340_e36588;
        locals.var_xi0p12__blk852_dn0 = assign26340_e36588_d_n0;
        locals.var_xi0p12__blk852_dn2 = assign26340_e36588_d_n2;
        locals.var_xi0p12__blk852_dn6 = assign26340_e36588_d_n6;
        locals.var_xi0p12__blk852_dn7 = assign26340_e36588_d_n7;
        locals.var_xi0p12__blk852_dn10 = assign26340_e36588_d_n10;
        locals.var_xi0p12__blk852_dn11 = assign26340_e36588_d_n11;
        locals.var_xi0p12__blk852_dn12 = assign26340_e36588_d_n12;
        locals.var_xi0p12__blk852_dn17 = assign26340_e36588_d_n17;
        locals.var_xi0p12__blk852_rv = 0.0;

        let (assign26350_e36603, assign26350_e36603_d_n0, assign26350_e36603_d_n2, assign26350_e36603_d_n6, assign26350_e36603_d_n7, assign26350_e36603_d_n10, assign26350_e36603_d_n11, assign26350_e36603_d_n12, assign26350_e36603_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let assign26350_e36601: f64 = (locals.var_cnst0over * locals.var_xi0p12__blk852);
        (assign26350_e36601, ((locals.var_cnst0over_dn0 * locals.var_xi0p12__blk852) + (locals.var_cnst0over * locals.var_xi0p12__blk852_dn0)), ((locals.var_cnst0over_dn2 * locals.var_xi0p12__blk852) + (locals.var_cnst0over * locals.var_xi0p12__blk852_dn2)), ((locals.var_cnst0over_dn6 * locals.var_xi0p12__blk852) + (locals.var_cnst0over * locals.var_xi0p12__blk852_dn6)), ((locals.var_cnst0over_dn7 * locals.var_xi0p12__blk852) + (locals.var_cnst0over * locals.var_xi0p12__blk852_dn7)), ((locals.var_cnst0over_dn10 * locals.var_xi0p12__blk852) + (locals.var_cnst0over * locals.var_xi0p12__blk852_dn10)), ((locals.var_cnst0over_dn11 * locals.var_xi0p12__blk852) + (locals.var_cnst0over * locals.var_xi0p12__blk852_dn11)), ((locals.var_cnst0over_dn12 * locals.var_xi0p12__blk852) + (locals.var_cnst0over * locals.var_xi0p12__blk852_dn12)), ((locals.var_cnst0over_dn17 * locals.var_xi0p12__blk852) + (locals.var_cnst0over * locals.var_xi0p12__blk852_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign26350_e36603;
        locals.var_qbuld_dn0 = assign26350_e36603_d_n0;
        locals.var_qbuld_dn2 = assign26350_e36603_d_n2;
        locals.var_qbuld_dn6 = assign26350_e36603_d_n6;
        locals.var_qbuld_dn7 = assign26350_e36603_d_n7;
        locals.var_qbuld_dn10 = assign26350_e36603_d_n10;
        locals.var_qbuld_dn11 = assign26350_e36603_d_n11;
        locals.var_qbuld_dn12 = assign26350_e36603_d_n12;
        locals.var_qbuld_dn17 = assign26350_e36603_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign26360_e36620, assign26360_e36620_d_n0, assign26360_e36620_d_n2, assign26360_e36620_d_n6, assign26360_e36620_d_n7, assign26360_e36620_d_n10, assign26360_e36620_d_n11, assign26360_e36620_d_n12, assign26360_e36620_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let assign26360_e36617: f64 = (locals.var_fs02__blk844 + locals.var_xi0p12__blk852);
        let assign26360_e36618: f64 = (1.0 / assign26360_e36617);
        (assign26360_e36618, (-((locals.var_fs02__blk844_dn0 + locals.var_xi0p12__blk852_dn0) / (assign26360_e36617 * assign26360_e36617))), (-((locals.var_fs02__blk844_dn2 + locals.var_xi0p12__blk852_dn2) / (assign26360_e36617 * assign26360_e36617))), (-((locals.var_fs02__blk844_dn6 + locals.var_xi0p12__blk852_dn6) / (assign26360_e36617 * assign26360_e36617))), (-((locals.var_fs02__blk844_dn7 + locals.var_xi0p12__blk852_dn7) / (assign26360_e36617 * assign26360_e36617))), (-((locals.var_fs02__blk844_dn10 + locals.var_xi0p12__blk852_dn10) / (assign26360_e36617 * assign26360_e36617))), (-((locals.var_fs02__blk844_dn11 + locals.var_xi0p12__blk852_dn11) / (assign26360_e36617 * assign26360_e36617))), (-((locals.var_fs02__blk844_dn12 + locals.var_xi0p12__blk852_dn12) / (assign26360_e36617 * assign26360_e36617))), (-((locals.var_fs02__blk844_dn17 + locals.var_xi0p12__blk852_dn17) / (assign26360_e36617 * assign26360_e36617))),)
    } else {
        (locals.var_t1__blk775, locals.var_t1__blk775_dn0, locals.var_t1__blk775_dn2, locals.var_t1__blk775_dn6, locals.var_t1__blk775_dn7, locals.var_t1__blk775_dn10, locals.var_t1__blk775_dn11, locals.var_t1__blk775_dn12, locals.var_t1__blk775_dn17,)
    }
};
        locals.var_t1__blk775 = assign26360_e36620;
        locals.var_t1__blk775_dn0 = assign26360_e36620_d_n0;
        locals.var_t1__blk775_dn2 = assign26360_e36620_d_n2;
        locals.var_t1__blk775_dn6 = assign26360_e36620_d_n6;
        locals.var_t1__blk775_dn7 = assign26360_e36620_d_n7;
        locals.var_t1__blk775_dn10 = assign26360_e36620_d_n10;
        locals.var_t1__blk775_dn11 = assign26360_e36620_d_n11;
        locals.var_t1__blk775_dn12 = assign26360_e36620_d_n12;
        locals.var_t1__blk775_dn17 = assign26360_e36620_d_n17;
        locals.var_t1__blk775_rv = 0.0;

        let (assign26370_e36637, assign26370_e36637_d_n0, assign26370_e36637_d_n2, assign26370_e36637_d_n6, assign26370_e36637_d_n7, assign26370_e36637_d_n10, assign26370_e36637_d_n11, assign26370_e36637_d_n12, assign26370_e36637_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let assign26370_e36633: f64 = (locals.var_cnst0over * locals.var_fs01__blk840);
        let assign26370_e36635: f64 = (assign26370_e36633 * locals.var_t1__blk775);
        (assign26370_e36635, ((((locals.var_cnst0over_dn0 * locals.var_fs01__blk840) + (locals.var_cnst0over * locals.var_fs01__blk840_dn0)) * locals.var_t1__blk775) + (assign26370_e36633 * locals.var_t1__blk775_dn0)), ((((locals.var_cnst0over_dn2 * locals.var_fs01__blk840) + (locals.var_cnst0over * locals.var_fs01__blk840_dn2)) * locals.var_t1__blk775) + (assign26370_e36633 * locals.var_t1__blk775_dn2)), ((((locals.var_cnst0over_dn6 * locals.var_fs01__blk840) + (locals.var_cnst0over * locals.var_fs01__blk840_dn6)) * locals.var_t1__blk775) + (assign26370_e36633 * locals.var_t1__blk775_dn6)), ((((locals.var_cnst0over_dn7 * locals.var_fs01__blk840) + (locals.var_cnst0over * locals.var_fs01__blk840_dn7)) * locals.var_t1__blk775) + (assign26370_e36633 * locals.var_t1__blk775_dn7)), ((((locals.var_cnst0over_dn10 * locals.var_fs01__blk840) + (locals.var_cnst0over * locals.var_fs01__blk840_dn10)) * locals.var_t1__blk775) + (assign26370_e36633 * locals.var_t1__blk775_dn10)), ((((locals.var_cnst0over_dn11 * locals.var_fs01__blk840) + (locals.var_cnst0over * locals.var_fs01__blk840_dn11)) * locals.var_t1__blk775) + (assign26370_e36633 * locals.var_t1__blk775_dn11)), ((((locals.var_cnst0over_dn12 * locals.var_fs01__blk840) + (locals.var_cnst0over * locals.var_fs01__blk840_dn12)) * locals.var_t1__blk775) + (assign26370_e36633 * locals.var_t1__blk775_dn12)), ((((locals.var_cnst0over_dn17 * locals.var_fs01__blk840) + (locals.var_cnst0over * locals.var_fs01__blk840_dn17)) * locals.var_t1__blk775) + (assign26370_e36633 * locals.var_t1__blk775_dn17)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign26370_e36637;
        locals.var_qiuld_dn0 = assign26370_e36637_d_n0;
        locals.var_qiuld_dn2 = assign26370_e36637_d_n2;
        locals.var_qiuld_dn6 = assign26370_e36637_d_n6;
        locals.var_qiuld_dn7 = assign26370_e36637_d_n7;
        locals.var_qiuld_dn10 = assign26370_e36637_d_n10;
        locals.var_qiuld_dn11 = assign26370_e36637_d_n11;
        locals.var_qiuld_dn12 = assign26370_e36637_d_n12;
        locals.var_qiuld_dn17 = assign26370_e36637_d_n17;
        locals.var_qiuld_rv = 0.0;

        let (assign26380_e36652, assign26380_e36652_d_n0, assign26380_e36652_d_n2, assign26380_e36652_d_n6, assign26380_e36652_d_n7, assign26380_e36652_d_n10, assign26380_e36652_d_n11, assign26380_e36652_d_n12, assign26380_e36652_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard857 == 0.0)) && (locals.var_guard863 != 0.0)) {
        let assign26380_e36650: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign26380_e36650, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), (locals.var_qbuld_dn17 + locals.var_qiuld_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign26380_e36652;
        locals.var_qsuld_dn0 = assign26380_e36652_d_n0;
        locals.var_qsuld_dn2 = assign26380_e36652_d_n2;
        locals.var_qsuld_dn6 = assign26380_e36652_d_n6;
        locals.var_qsuld_dn7 = assign26380_e36652_d_n7;
        locals.var_qsuld_dn10 = assign26380_e36652_d_n10;
        locals.var_qsuld_dn11 = assign26380_e36652_d_n11;
        locals.var_qsuld_dn12 = assign26380_e36652_d_n12;
        locals.var_qsuld_dn17 = assign26380_e36652_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign26390_e36662, assign26390_e36662_d_n0, assign26390_e36662_d_n2, assign26390_e36662_d_n6, assign26390_e36662_d_n7, assign26390_e36662_d_n10, assign26390_e36662_d_n11, assign26390_e36662_d_n12, assign26390_e36662_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26390_e36660: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign26390_e36660, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), (locals.var_qsuld_dn17 - locals.var_qbuld_dn17),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign26390_e36662;
        locals.var_qiuld_dn0 = assign26390_e36662_d_n0;
        locals.var_qiuld_dn2 = assign26390_e36662_d_n2;
        locals.var_qiuld_dn6 = assign26390_e36662_d_n6;
        locals.var_qiuld_dn7 = assign26390_e36662_d_n7;
        locals.var_qiuld_dn10 = assign26390_e36662_d_n10;
        locals.var_qiuld_dn11 = assign26390_e36662_d_n11;
        locals.var_qiuld_dn12 = assign26390_e36662_d_n12;
        locals.var_qiuld_dn17 = assign26390_e36662_d_n17;
        locals.var_qiuld_rv = 0.0;

        let assign26400_e36665: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard872 = assign26400_e36665;
        locals.var_guard872_rv = 0.0;

        let assign26410_e36668: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard873 = assign26410_e36668;
        locals.var_guard873_rv = 0.0;

        let (assign26420_e36683, assign26420_e36683_d_n0, assign26420_e36683_d_n2, assign26420_e36683_d_n6, assign26420_e36683_d_n7, assign26420_e36683_d_n10, assign26420_e36683_d_n11, assign26420_e36683_d_n12, assign26420_e36683_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard872 != 0.0)) && (locals.var_flg_ovloops != 0.0)) {
        let assign26420_e36679: f64 = (-locals.var_uc_areabt);
        let assign26420_e36681: f64 = (assign26420_e36679 * locals.var_qsuld);
        (assign26420_e36681, (assign26420_e36679 * locals.var_qsuld_dn0), (assign26420_e36679 * locals.var_qsuld_dn2), (assign26420_e36679 * locals.var_qsuld_dn6), (assign26420_e36679 * locals.var_qsuld_dn7), (assign26420_e36679 * locals.var_qsuld_dn10), (assign26420_e36679 * locals.var_qsuld_dn11), (assign26420_e36679 * locals.var_qsuld_dn12), (assign26420_e36679 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_sus, locals.var_qbody_bt_p_sus_dn0, locals.var_qbody_bt_p_sus_dn2, locals.var_qbody_bt_p_sus_dn6, locals.var_qbody_bt_p_sus_dn7, locals.var_qbody_bt_p_sus_dn10, locals.var_qbody_bt_p_sus_dn11, locals.var_qbody_bt_p_sus_dn12, locals.var_qbody_bt_p_sus_dn17,)
    }
};
        locals.var_qbody_bt_p_sus = assign26420_e36683;
        locals.var_qbody_bt_p_sus_dn0 = assign26420_e36683_d_n0;
        locals.var_qbody_bt_p_sus_dn2 = assign26420_e36683_d_n2;
        locals.var_qbody_bt_p_sus_dn6 = assign26420_e36683_d_n6;
        locals.var_qbody_bt_p_sus_dn7 = assign26420_e36683_d_n7;
        locals.var_qbody_bt_p_sus_dn10 = assign26420_e36683_d_n10;
        locals.var_qbody_bt_p_sus_dn11 = assign26420_e36683_d_n11;
        locals.var_qbody_bt_p_sus_dn12 = assign26420_e36683_d_n12;
        locals.var_qbody_bt_p_sus_dn17 = assign26420_e36683_d_n17;
        locals.var_qbody_bt_p_sus_rv = 0.0;

        let (assign26430_e36698, assign26430_e36698_d_n0, assign26430_e36698_d_n2, assign26430_e36698_d_n6, assign26430_e36698_d_n7, assign26430_e36698_d_n10, assign26430_e36698_d_n11, assign26430_e36698_d_n12, assign26430_e36698_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard872 != 0.0)) && (locals.var_flg_ovloops != 0.0)) {
        let assign26430_e36694: f64 = (-locals.var_uc_areabt);
        let assign26430_e36696: f64 = (assign26430_e36694 * locals.var_qiuld);
        (assign26430_e36696, (assign26430_e36694 * locals.var_qiuld_dn0), (assign26430_e36694 * locals.var_qiuld_dn2), (assign26430_e36694 * locals.var_qiuld_dn6), (assign26430_e36694 * locals.var_qiuld_dn7), (assign26430_e36694 * locals.var_qiuld_dn10), (assign26430_e36694 * locals.var_qiuld_dn11), (assign26430_e36694 * locals.var_qiuld_dn12), (assign26430_e36694 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_ius, locals.var_qbody_bt_p_ius_dn0, locals.var_qbody_bt_p_ius_dn2, locals.var_qbody_bt_p_ius_dn6, locals.var_qbody_bt_p_ius_dn7, locals.var_qbody_bt_p_ius_dn10, locals.var_qbody_bt_p_ius_dn11, locals.var_qbody_bt_p_ius_dn12, locals.var_qbody_bt_p_ius_dn17,)
    }
};
        locals.var_qbody_bt_p_ius = assign26430_e36698;
        locals.var_qbody_bt_p_ius_dn0 = assign26430_e36698_d_n0;
        locals.var_qbody_bt_p_ius_dn2 = assign26430_e36698_d_n2;
        locals.var_qbody_bt_p_ius_dn6 = assign26430_e36698_d_n6;
        locals.var_qbody_bt_p_ius_dn7 = assign26430_e36698_d_n7;
        locals.var_qbody_bt_p_ius_dn10 = assign26430_e36698_d_n10;
        locals.var_qbody_bt_p_ius_dn11 = assign26430_e36698_d_n11;
        locals.var_qbody_bt_p_ius_dn12 = assign26430_e36698_d_n12;
        locals.var_qbody_bt_p_ius_dn17 = assign26430_e36698_d_n17;
        locals.var_qbody_bt_p_ius_rv = 0.0;

        let (assign26440_e36713, assign26440_e36713_d_n0, assign26440_e36713_d_n2, assign26440_e36713_d_n6, assign26440_e36713_d_n7, assign26440_e36713_d_n10, assign26440_e36713_d_n11, assign26440_e36713_d_n12, assign26440_e36713_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard872 != 0.0)) && (locals.var_flg_ovloopd != 0.0)) {
        let assign26440_e36709: f64 = (-locals.var_uc_areabt);
        let assign26440_e36711: f64 = (assign26440_e36709 * locals.var_qsuld);
        (assign26440_e36711, (assign26440_e36709 * locals.var_qsuld_dn0), (assign26440_e36709 * locals.var_qsuld_dn2), (assign26440_e36709 * locals.var_qsuld_dn6), (assign26440_e36709 * locals.var_qsuld_dn7), (assign26440_e36709 * locals.var_qsuld_dn10), (assign26440_e36709 * locals.var_qsuld_dn11), (assign26440_e36709 * locals.var_qsuld_dn12), (assign26440_e36709 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_sud, locals.var_qbody_bt_p_sud_dn0, locals.var_qbody_bt_p_sud_dn2, locals.var_qbody_bt_p_sud_dn6, locals.var_qbody_bt_p_sud_dn7, locals.var_qbody_bt_p_sud_dn10, locals.var_qbody_bt_p_sud_dn11, locals.var_qbody_bt_p_sud_dn12, locals.var_qbody_bt_p_sud_dn17,)
    }
};
        locals.var_qbody_bt_p_sud = assign26440_e36713;
        locals.var_qbody_bt_p_sud_dn0 = assign26440_e36713_d_n0;
        locals.var_qbody_bt_p_sud_dn2 = assign26440_e36713_d_n2;
        locals.var_qbody_bt_p_sud_dn6 = assign26440_e36713_d_n6;
        locals.var_qbody_bt_p_sud_dn7 = assign26440_e36713_d_n7;
        locals.var_qbody_bt_p_sud_dn10 = assign26440_e36713_d_n10;
        locals.var_qbody_bt_p_sud_dn11 = assign26440_e36713_d_n11;
        locals.var_qbody_bt_p_sud_dn12 = assign26440_e36713_d_n12;
        locals.var_qbody_bt_p_sud_dn17 = assign26440_e36713_d_n17;
        locals.var_qbody_bt_p_sud_rv = 0.0;

        let (assign26450_e36728, assign26450_e36728_d_n0, assign26450_e36728_d_n2, assign26450_e36728_d_n6, assign26450_e36728_d_n7, assign26450_e36728_d_n10, assign26450_e36728_d_n11, assign26450_e36728_d_n12, assign26450_e36728_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard872 != 0.0)) && (locals.var_flg_ovloopd != 0.0)) {
        let assign26450_e36724: f64 = (-locals.var_uc_areabt);
        let assign26450_e36726: f64 = (assign26450_e36724 * locals.var_qiuld);
        (assign26450_e36726, (assign26450_e36724 * locals.var_qiuld_dn0), (assign26450_e36724 * locals.var_qiuld_dn2), (assign26450_e36724 * locals.var_qiuld_dn6), (assign26450_e36724 * locals.var_qiuld_dn7), (assign26450_e36724 * locals.var_qiuld_dn10), (assign26450_e36724 * locals.var_qiuld_dn11), (assign26450_e36724 * locals.var_qiuld_dn12), (assign26450_e36724 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_iud, locals.var_qbody_bt_p_iud_dn0, locals.var_qbody_bt_p_iud_dn2, locals.var_qbody_bt_p_iud_dn6, locals.var_qbody_bt_p_iud_dn7, locals.var_qbody_bt_p_iud_dn10, locals.var_qbody_bt_p_iud_dn11, locals.var_qbody_bt_p_iud_dn12, locals.var_qbody_bt_p_iud_dn17,)
    }
};
        locals.var_qbody_bt_p_iud = assign26450_e36728;
        locals.var_qbody_bt_p_iud_dn0 = assign26450_e36728_d_n0;
        locals.var_qbody_bt_p_iud_dn2 = assign26450_e36728_d_n2;
        locals.var_qbody_bt_p_iud_dn6 = assign26450_e36728_d_n6;
        locals.var_qbody_bt_p_iud_dn7 = assign26450_e36728_d_n7;
        locals.var_qbody_bt_p_iud_dn10 = assign26450_e36728_d_n10;
        locals.var_qbody_bt_p_iud_dn11 = assign26450_e36728_d_n11;
        locals.var_qbody_bt_p_iud_dn12 = assign26450_e36728_d_n12;
        locals.var_qbody_bt_p_iud_dn17 = assign26450_e36728_d_n17;
        locals.var_qbody_bt_p_iud_rv = 0.0;

        let (assign26460_e36746, assign26460_e36746_d_n0, assign26460_e36746_d_n2, assign26460_e36746_d_n6, assign26460_e36746_d_n7, assign26460_e36746_d_n10, assign26460_e36746_d_n11, assign26460_e36746_d_n12, assign26460_e36746_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && ((locals.var_guard873 != 0.0) && (locals.var_guard872 == 0.0))) && (locals.var_flg_ovloops != 0.0)) {
        let assign26460_e36742: f64 = (-locals.var_uc_areabt);
        let assign26460_e36744: f64 = (assign26460_e36742 * locals.var_qsuld);
        (assign26460_e36744, (assign26460_e36742 * locals.var_qsuld_dn0), (assign26460_e36742 * locals.var_qsuld_dn2), (assign26460_e36742 * locals.var_qsuld_dn6), (assign26460_e36742 * locals.var_qsuld_dn7), (assign26460_e36742 * locals.var_qsuld_dn10), (assign26460_e36742 * locals.var_qsuld_dn11), (assign26460_e36742 * locals.var_qsuld_dn12), (assign26460_e36742 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_sus, locals.var_qbody_bt_n_sus_dn0, locals.var_qbody_bt_n_sus_dn2, locals.var_qbody_bt_n_sus_dn6, locals.var_qbody_bt_n_sus_dn7, locals.var_qbody_bt_n_sus_dn10, locals.var_qbody_bt_n_sus_dn11, locals.var_qbody_bt_n_sus_dn12, locals.var_qbody_bt_n_sus_dn17,)
    }
};
        locals.var_qbody_bt_n_sus = assign26460_e36746;
        locals.var_qbody_bt_n_sus_dn0 = assign26460_e36746_d_n0;
        locals.var_qbody_bt_n_sus_dn2 = assign26460_e36746_d_n2;
        locals.var_qbody_bt_n_sus_dn6 = assign26460_e36746_d_n6;
        locals.var_qbody_bt_n_sus_dn7 = assign26460_e36746_d_n7;
        locals.var_qbody_bt_n_sus_dn10 = assign26460_e36746_d_n10;
        locals.var_qbody_bt_n_sus_dn11 = assign26460_e36746_d_n11;
        locals.var_qbody_bt_n_sus_dn12 = assign26460_e36746_d_n12;
        locals.var_qbody_bt_n_sus_dn17 = assign26460_e36746_d_n17;
        locals.var_qbody_bt_n_sus_rv = 0.0;

        let (assign26470_e36764, assign26470_e36764_d_n0, assign26470_e36764_d_n2, assign26470_e36764_d_n6, assign26470_e36764_d_n7, assign26470_e36764_d_n10, assign26470_e36764_d_n11, assign26470_e36764_d_n12, assign26470_e36764_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && ((locals.var_guard873 != 0.0) && (locals.var_guard872 == 0.0))) && (locals.var_flg_ovloops != 0.0)) {
        let assign26470_e36760: f64 = (-locals.var_uc_areabt);
        let assign26470_e36762: f64 = (assign26470_e36760 * locals.var_qiuld);
        (assign26470_e36762, (assign26470_e36760 * locals.var_qiuld_dn0), (assign26470_e36760 * locals.var_qiuld_dn2), (assign26470_e36760 * locals.var_qiuld_dn6), (assign26470_e36760 * locals.var_qiuld_dn7), (assign26470_e36760 * locals.var_qiuld_dn10), (assign26470_e36760 * locals.var_qiuld_dn11), (assign26470_e36760 * locals.var_qiuld_dn12), (assign26470_e36760 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_ius, locals.var_qbody_bt_n_ius_dn0, locals.var_qbody_bt_n_ius_dn2, locals.var_qbody_bt_n_ius_dn6, locals.var_qbody_bt_n_ius_dn7, locals.var_qbody_bt_n_ius_dn10, locals.var_qbody_bt_n_ius_dn11, locals.var_qbody_bt_n_ius_dn12, locals.var_qbody_bt_n_ius_dn17,)
    }
};
        locals.var_qbody_bt_n_ius = assign26470_e36764;
        locals.var_qbody_bt_n_ius_dn0 = assign26470_e36764_d_n0;
        locals.var_qbody_bt_n_ius_dn2 = assign26470_e36764_d_n2;
        locals.var_qbody_bt_n_ius_dn6 = assign26470_e36764_d_n6;
        locals.var_qbody_bt_n_ius_dn7 = assign26470_e36764_d_n7;
        locals.var_qbody_bt_n_ius_dn10 = assign26470_e36764_d_n10;
        locals.var_qbody_bt_n_ius_dn11 = assign26470_e36764_d_n11;
        locals.var_qbody_bt_n_ius_dn12 = assign26470_e36764_d_n12;
        locals.var_qbody_bt_n_ius_dn17 = assign26470_e36764_d_n17;
        locals.var_qbody_bt_n_ius_rv = 0.0;

        let (assign26480_e36782, assign26480_e36782_d_n0, assign26480_e36782_d_n2, assign26480_e36782_d_n6, assign26480_e36782_d_n7, assign26480_e36782_d_n10, assign26480_e36782_d_n11, assign26480_e36782_d_n12, assign26480_e36782_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && ((locals.var_guard873 != 0.0) && (locals.var_guard872 == 0.0))) && (locals.var_flg_ovloopd != 0.0)) {
        let assign26480_e36778: f64 = (-locals.var_uc_areabt);
        let assign26480_e36780: f64 = (assign26480_e36778 * locals.var_qsuld);
        (assign26480_e36780, (assign26480_e36778 * locals.var_qsuld_dn0), (assign26480_e36778 * locals.var_qsuld_dn2), (assign26480_e36778 * locals.var_qsuld_dn6), (assign26480_e36778 * locals.var_qsuld_dn7), (assign26480_e36778 * locals.var_qsuld_dn10), (assign26480_e36778 * locals.var_qsuld_dn11), (assign26480_e36778 * locals.var_qsuld_dn12), (assign26480_e36778 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_sud, locals.var_qbody_bt_n_sud_dn0, locals.var_qbody_bt_n_sud_dn2, locals.var_qbody_bt_n_sud_dn6, locals.var_qbody_bt_n_sud_dn7, locals.var_qbody_bt_n_sud_dn10, locals.var_qbody_bt_n_sud_dn11, locals.var_qbody_bt_n_sud_dn12, locals.var_qbody_bt_n_sud_dn17,)
    }
};
        locals.var_qbody_bt_n_sud = assign26480_e36782;
        locals.var_qbody_bt_n_sud_dn0 = assign26480_e36782_d_n0;
        locals.var_qbody_bt_n_sud_dn2 = assign26480_e36782_d_n2;
        locals.var_qbody_bt_n_sud_dn6 = assign26480_e36782_d_n6;
        locals.var_qbody_bt_n_sud_dn7 = assign26480_e36782_d_n7;
        locals.var_qbody_bt_n_sud_dn10 = assign26480_e36782_d_n10;
        locals.var_qbody_bt_n_sud_dn11 = assign26480_e36782_d_n11;
        locals.var_qbody_bt_n_sud_dn12 = assign26480_e36782_d_n12;
        locals.var_qbody_bt_n_sud_dn17 = assign26480_e36782_d_n17;
        locals.var_qbody_bt_n_sud_rv = 0.0;

        let (assign26490_e36800, assign26490_e36800_d_n0, assign26490_e36800_d_n2, assign26490_e36800_d_n6, assign26490_e36800_d_n7, assign26490_e36800_d_n10, assign26490_e36800_d_n11, assign26490_e36800_d_n12, assign26490_e36800_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && ((locals.var_guard873 != 0.0) && (locals.var_guard872 == 0.0))) && (locals.var_flg_ovloopd != 0.0)) {
        let assign26490_e36796: f64 = (-locals.var_uc_areabt);
        let assign26490_e36798: f64 = (assign26490_e36796 * locals.var_qiuld);
        (assign26490_e36798, (assign26490_e36796 * locals.var_qiuld_dn0), (assign26490_e36796 * locals.var_qiuld_dn2), (assign26490_e36796 * locals.var_qiuld_dn6), (assign26490_e36796 * locals.var_qiuld_dn7), (assign26490_e36796 * locals.var_qiuld_dn10), (assign26490_e36796 * locals.var_qiuld_dn11), (assign26490_e36796 * locals.var_qiuld_dn12), (assign26490_e36796 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_iud, locals.var_qbody_bt_n_iud_dn0, locals.var_qbody_bt_n_iud_dn2, locals.var_qbody_bt_n_iud_dn6, locals.var_qbody_bt_n_iud_dn7, locals.var_qbody_bt_n_iud_dn10, locals.var_qbody_bt_n_iud_dn11, locals.var_qbody_bt_n_iud_dn12, locals.var_qbody_bt_n_iud_dn17,)
    }
};
        locals.var_qbody_bt_n_iud = assign26490_e36800;
        locals.var_qbody_bt_n_iud_dn0 = assign26490_e36800_d_n0;
        locals.var_qbody_bt_n_iud_dn2 = assign26490_e36800_d_n2;
        locals.var_qbody_bt_n_iud_dn6 = assign26490_e36800_d_n6;
        locals.var_qbody_bt_n_iud_dn7 = assign26490_e36800_d_n7;
        locals.var_qbody_bt_n_iud_dn10 = assign26490_e36800_d_n10;
        locals.var_qbody_bt_n_iud_dn11 = assign26490_e36800_d_n11;
        locals.var_qbody_bt_n_iud_dn12 = assign26490_e36800_d_n12;
        locals.var_qbody_bt_n_iud_dn17 = assign26490_e36800_d_n17;
        locals.var_qbody_bt_n_iud_rv = 0.0;

        let (assign26500_e36812,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26500_e36808: f64 = (1.0 - 1.0);
        let assign26500_e36810: f64 = (assign26500_e36808 / 2.0);
        (assign26500_e36810,)
    } else {
        (locals.var_flg_ovloops,)
    }
};
        locals.var_flg_ovloops = assign26500_e36812;
        locals.var_flg_ovloops_rv = 0.0;

        let (assign26510_e36824,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26510_e36820: f64 = (1.0 + 1.0);
        let assign26510_e36822: f64 = (assign26510_e36820 / 2.0);
        (assign26510_e36822,)
    } else {
        (locals.var_flg_ovloopd,)
    }
};
        locals.var_flg_ovloopd = assign26510_e36824;
        locals.var_flg_ovloopd_rv = 0.0;

        let (assign26520_e36840, assign26520_e36840_d_n0, assign26520_e36840_d_n2, assign26520_e36840_d_n6, assign26520_e36840_d_n7, assign26520_e36840_d_n10, assign26520_e36840_d_n11, assign26520_e36840_d_n12, assign26520_e36840_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26520_e36832: f64 = (locals.var_modenml * locals.var_vbs);
        let assign26520_e36836: f64 = (locals.var_vbs - locals.var_vds);
        let assign26520_e36837: f64 = (locals.var_modervs * assign26520_e36836);
        let assign26520_e36838: f64 = (assign26520_e36832 + assign26520_e36837);
        (assign26520_e36838, ((locals.var_modenml * locals.var_vbs_dn0) + (locals.var_modervs * (locals.var_vbs_dn0 - locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vbs_dn2) + (locals.var_modervs * (locals.var_vbs_dn2 - locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vbs_dn6) + (locals.var_modervs * (locals.var_vbs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vbs_dn7) + (locals.var_modervs * (locals.var_vbs_dn7 - locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vbs_dn10) + (locals.var_modervs * (locals.var_vbs_dn10 - locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vbs_dn11) + (locals.var_modervs * (locals.var_vbs_dn11 - locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vbs_dn12) + (locals.var_modervs * (locals.var_vbs_dn12 - locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vbs_dn17) + (locals.var_modervs * (locals.var_vbs_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_vbsgmt, locals.var_vbsgmt_dn0, locals.var_vbsgmt_dn2, locals.var_vbsgmt_dn6, locals.var_vbsgmt_dn7, locals.var_vbsgmt_dn10, locals.var_vbsgmt_dn11, locals.var_vbsgmt_dn12, locals.var_vbsgmt_dn17,)
    }
};
        locals.var_vbsgmt = assign26520_e36840;
        locals.var_vbsgmt_dn0 = assign26520_e36840_d_n0;
        locals.var_vbsgmt_dn2 = assign26520_e36840_d_n2;
        locals.var_vbsgmt_dn6 = assign26520_e36840_d_n6;
        locals.var_vbsgmt_dn7 = assign26520_e36840_d_n7;
        locals.var_vbsgmt_dn10 = assign26520_e36840_d_n10;
        locals.var_vbsgmt_dn11 = assign26520_e36840_d_n11;
        locals.var_vbsgmt_dn12 = assign26520_e36840_d_n12;
        locals.var_vbsgmt_dn17 = assign26520_e36840_d_n17;
        locals.var_vbsgmt_rv = 0.0;

        let (assign26530_e36855, assign26530_e36855_d_n0, assign26530_e36855_d_n2, assign26530_e36855_d_n6, assign26530_e36855_d_n7, assign26530_e36855_d_n10, assign26530_e36855_d_n11, assign26530_e36855_d_n12, assign26530_e36855_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26530_e36848: f64 = (locals.var_modenml * locals.var_vds);
        let assign26530_e36851: f64 = (-locals.var_vds);
        let assign26530_e36852: f64 = (locals.var_modervs * assign26530_e36851);
        let assign26530_e36853: f64 = (assign26530_e36848 + assign26530_e36852);
        (assign26530_e36853, ((locals.var_modenml * locals.var_vds_dn0) + (locals.var_modervs * (-locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vds_dn2) + (locals.var_modervs * (-locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vds_dn6) + (locals.var_modervs * (-locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vds_dn7) + (locals.var_modervs * (-locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vds_dn10) + (locals.var_modervs * (-locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vds_dn11) + (locals.var_modervs * (-locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vds_dn12) + (locals.var_modervs * (-locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vds_dn17) + (locals.var_modervs * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_vdsgmt, locals.var_vdsgmt_dn0, locals.var_vdsgmt_dn2, locals.var_vdsgmt_dn6, locals.var_vdsgmt_dn7, locals.var_vdsgmt_dn10, locals.var_vdsgmt_dn11, locals.var_vdsgmt_dn12, locals.var_vdsgmt_dn17,)
    }
};
        locals.var_vdsgmt = assign26530_e36855;
        locals.var_vdsgmt_dn0 = assign26530_e36855_d_n0;
        locals.var_vdsgmt_dn2 = assign26530_e36855_d_n2;
        locals.var_vdsgmt_dn6 = assign26530_e36855_d_n6;
        locals.var_vdsgmt_dn7 = assign26530_e36855_d_n7;
        locals.var_vdsgmt_dn10 = assign26530_e36855_d_n10;
        locals.var_vdsgmt_dn11 = assign26530_e36855_d_n11;
        locals.var_vdsgmt_dn12 = assign26530_e36855_d_n12;
        locals.var_vdsgmt_dn17 = assign26530_e36855_d_n17;
        locals.var_vdsgmt_rv = 0.0;

        let (assign26540_e36871, assign26540_e36871_d_n0, assign26540_e36871_d_n2, assign26540_e36871_d_n6, assign26540_e36871_d_n7, assign26540_e36871_d_n10, assign26540_e36871_d_n11, assign26540_e36871_d_n12, assign26540_e36871_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26540_e36863: f64 = (locals.var_modenml * locals.var_vgs);
        let assign26540_e36867: f64 = (locals.var_vgs - locals.var_vds);
        let assign26540_e36868: f64 = (locals.var_modervs * assign26540_e36867);
        let assign26540_e36869: f64 = (assign26540_e36863 + assign26540_e36868);
        (assign26540_e36869, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgsgmt, locals.var_vgsgmt_dn0, locals.var_vgsgmt_dn2, locals.var_vgsgmt_dn6, locals.var_vgsgmt_dn7, locals.var_vgsgmt_dn10, locals.var_vgsgmt_dn11, locals.var_vgsgmt_dn12, locals.var_vgsgmt_dn17,)
    }
};
        locals.var_vgsgmt = assign26540_e36871;
        locals.var_vgsgmt_dn0 = assign26540_e36871_d_n0;
        locals.var_vgsgmt_dn2 = assign26540_e36871_d_n2;
        locals.var_vgsgmt_dn6 = assign26540_e36871_d_n6;
        locals.var_vgsgmt_dn7 = assign26540_e36871_d_n7;
        locals.var_vgsgmt_dn10 = assign26540_e36871_d_n10;
        locals.var_vgsgmt_dn11 = assign26540_e36871_d_n11;
        locals.var_vgsgmt_dn12 = assign26540_e36871_d_n12;
        locals.var_vgsgmt_dn17 = assign26540_e36871_d_n17;
        locals.var_vgsgmt_rv = 0.0;

        let (assign26550_e36887, assign26550_e36887_d_n0, assign26550_e36887_d_n2, assign26550_e36887_d_n6, assign26550_e36887_d_n7, assign26550_e36887_d_n10, assign26550_e36887_d_n11, assign26550_e36887_d_n12, assign26550_e36887_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26550_e36879: f64 = (locals.var_modervs * locals.var_vgs);
        let assign26550_e36883: f64 = (locals.var_vgs - locals.var_vds);
        let assign26550_e36884: f64 = (locals.var_modenml * assign26550_e36883);
        let assign26550_e36885: f64 = (assign26550_e36879 + assign26550_e36884);
        (assign26550_e36885, (locals.var_modenml * (-locals.var_vds_dn0)), (locals.var_modenml * (-locals.var_vds_dn2)), ((locals.var_modervs * locals.var_vgs_dn6) + (locals.var_modenml * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modervs * locals.var_vgs_dn7) + (locals.var_modenml * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modenml * (-locals.var_vds_dn10)), ((locals.var_modervs * locals.var_vgs_dn11) + (locals.var_modenml * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modenml * (-locals.var_vds_dn12)), (locals.var_modenml * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgdgmt, locals.var_vgdgmt_dn0, locals.var_vgdgmt_dn2, locals.var_vgdgmt_dn6, locals.var_vgdgmt_dn7, locals.var_vgdgmt_dn10, locals.var_vgdgmt_dn11, locals.var_vgdgmt_dn12, locals.var_vgdgmt_dn17,)
    }
};
        locals.var_vgdgmt = assign26550_e36887;
        locals.var_vgdgmt_dn0 = assign26550_e36887_d_n0;
        locals.var_vgdgmt_dn2 = assign26550_e36887_d_n2;
        locals.var_vgdgmt_dn6 = assign26550_e36887_d_n6;
        locals.var_vgdgmt_dn7 = assign26550_e36887_d_n7;
        locals.var_vgdgmt_dn10 = assign26550_e36887_d_n10;
        locals.var_vgdgmt_dn11 = assign26550_e36887_d_n11;
        locals.var_vgdgmt_dn12 = assign26550_e36887_d_n12;
        locals.var_vgdgmt_dn17 = assign26550_e36887_d_n17;
        locals.var_vgdgmt_rv = 0.0;

        let (assign26560_e36897, assign26560_e36897_d_n0, assign26560_e36897_d_n2, assign26560_e36897_d_n6, assign26560_e36897_d_n7, assign26560_e36897_d_n10, assign26560_e36897_d_n11, assign26560_e36897_d_n12, assign26560_e36897_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26560_e36895: f64 = (locals.var_vdsgmt - locals.var_vbsgmt);
        (assign26560_e36895, (locals.var_vdsgmt_dn0 - locals.var_vbsgmt_dn0), (locals.var_vdsgmt_dn2 - locals.var_vbsgmt_dn2), (locals.var_vdsgmt_dn6 - locals.var_vbsgmt_dn6), (locals.var_vdsgmt_dn7 - locals.var_vbsgmt_dn7), (locals.var_vdsgmt_dn10 - locals.var_vbsgmt_dn10), (locals.var_vdsgmt_dn11 - locals.var_vbsgmt_dn11), (locals.var_vdsgmt_dn12 - locals.var_vbsgmt_dn12), (locals.var_vdsgmt_dn17 - locals.var_vbsgmt_dn17),)
    } else {
        (locals.var_vdbgmt, locals.var_vdbgmt_dn0, locals.var_vdbgmt_dn2, locals.var_vdbgmt_dn6, locals.var_vdbgmt_dn7, locals.var_vdbgmt_dn10, locals.var_vdbgmt_dn11, locals.var_vdbgmt_dn12, locals.var_vdbgmt_dn17,)
    }
};
        locals.var_vdbgmt = assign26560_e36897;
        locals.var_vdbgmt_dn0 = assign26560_e36897_d_n0;
        locals.var_vdbgmt_dn2 = assign26560_e36897_d_n2;
        locals.var_vdbgmt_dn6 = assign26560_e36897_d_n6;
        locals.var_vdbgmt_dn7 = assign26560_e36897_d_n7;
        locals.var_vdbgmt_dn10 = assign26560_e36897_d_n10;
        locals.var_vdbgmt_dn11 = assign26560_e36897_d_n11;
        locals.var_vdbgmt_dn12 = assign26560_e36897_d_n12;
        locals.var_vdbgmt_dn17 = assign26560_e36897_d_n17;
        locals.var_vdbgmt_rv = 0.0;

        let (assign26570_e36906, assign26570_e36906_d_n0, assign26570_e36906_d_n2, assign26570_e36906_d_n6, assign26570_e36906_d_n7, assign26570_e36906_d_n10, assign26570_e36906_d_n11, assign26570_e36906_d_n12, assign26570_e36906_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26570_e36904: f64 = (-locals.var_vbsgmt);
        (assign26570_e36904, (-locals.var_vbsgmt_dn0), (-locals.var_vbsgmt_dn2), (-locals.var_vbsgmt_dn6), (-locals.var_vbsgmt_dn7), (-locals.var_vbsgmt_dn10), (-locals.var_vbsgmt_dn11), (-locals.var_vbsgmt_dn12), (-locals.var_vbsgmt_dn17),)
    } else {
        (locals.var_vsbgmt, locals.var_vsbgmt_dn0, locals.var_vsbgmt_dn2, locals.var_vsbgmt_dn6, locals.var_vsbgmt_dn7, locals.var_vsbgmt_dn10, locals.var_vsbgmt_dn11, locals.var_vsbgmt_dn12, locals.var_vsbgmt_dn17,)
    }
};
        locals.var_vsbgmt = assign26570_e36906;
        locals.var_vsbgmt_dn0 = assign26570_e36906_d_n0;
        locals.var_vsbgmt_dn2 = assign26570_e36906_d_n2;
        locals.var_vsbgmt_dn6 = assign26570_e36906_d_n6;
        locals.var_vsbgmt_dn7 = assign26570_e36906_d_n7;
        locals.var_vsbgmt_dn10 = assign26570_e36906_d_n10;
        locals.var_vsbgmt_dn11 = assign26570_e36906_d_n11;
        locals.var_vsbgmt_dn12 = assign26570_e36906_d_n12;
        locals.var_vsbgmt_dn17 = assign26570_e36906_d_n17;
        locals.var_vsbgmt_rv = 0.0;

        let (assign26580_e36920,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26580_e36914: f64 = (locals.var_flg_ovloops * locals.var_modenml);
        let assign26580_e36917: f64 = (locals.var_flg_ovloopd * locals.var_modervs);
        let assign26580_e36918: f64 = (assign26580_e36914 + assign26580_e36917);
        (assign26580_e36918,)
    } else {
        (locals.var_flg_overs,)
    }
};
        locals.var_flg_overs = assign26580_e36920;
        locals.var_flg_overs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_95(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26590_e36934,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26590_e36928: f64 = (locals.var_flg_ovloops * locals.var_modervs);
        let assign26590_e36931: f64 = (locals.var_flg_ovloopd * locals.var_modenml);
        let assign26590_e36932: f64 = (assign26590_e36928 + assign26590_e36931);
        (assign26590_e36932,)
    } else {
        (locals.var_flg_overd,)
    }
};
        locals.var_flg_overd = assign26590_e36934;
        locals.var_flg_overd_rv = 0.0;

        let (assign26600_e36948, assign26600_e36948_d_n0, assign26600_e36948_d_n2, assign26600_e36948_d_n6, assign26600_e36948_d_n7, assign26600_e36948_d_n10, assign26600_e36948_d_n11, assign26600_e36948_d_n12, assign26600_e36948_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26600_e36942: f64 = (locals.var_flg_overs * locals.var_vgsgmt);
        let assign26600_e36945: f64 = (locals.var_flg_overd * locals.var_vgdgmt);
        let assign26600_e36946: f64 = (assign26600_e36942 + assign26600_e36945);
        (assign26600_e36946, ((locals.var_flg_overs * locals.var_vgsgmt_dn0) + (locals.var_flg_overd * locals.var_vgdgmt_dn0)), ((locals.var_flg_overs * locals.var_vgsgmt_dn2) + (locals.var_flg_overd * locals.var_vgdgmt_dn2)), ((locals.var_flg_overs * locals.var_vgsgmt_dn6) + (locals.var_flg_overd * locals.var_vgdgmt_dn6)), ((locals.var_flg_overs * locals.var_vgsgmt_dn7) + (locals.var_flg_overd * locals.var_vgdgmt_dn7)), ((locals.var_flg_overs * locals.var_vgsgmt_dn10) + (locals.var_flg_overd * locals.var_vgdgmt_dn10)), ((locals.var_flg_overs * locals.var_vgsgmt_dn11) + (locals.var_flg_overd * locals.var_vgdgmt_dn11)), ((locals.var_flg_overs * locals.var_vgsgmt_dn12) + (locals.var_flg_overd * locals.var_vgdgmt_dn12)), ((locals.var_flg_overs * locals.var_vgsgmt_dn17) + (locals.var_flg_overd * locals.var_vgdgmt_dn17)),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn10, locals.var_vgbgmt_dn11, locals.var_vgbgmt_dn12, locals.var_vgbgmt_dn17,)
    }
};
        locals.var_vgbgmt = assign26600_e36948;
        locals.var_vgbgmt_dn0 = assign26600_e36948_d_n0;
        locals.var_vgbgmt_dn2 = assign26600_e36948_d_n2;
        locals.var_vgbgmt_dn6 = assign26600_e36948_d_n6;
        locals.var_vgbgmt_dn7 = assign26600_e36948_d_n7;
        locals.var_vgbgmt_dn10 = assign26600_e36948_d_n10;
        locals.var_vgbgmt_dn11 = assign26600_e36948_d_n11;
        locals.var_vgbgmt_dn12 = assign26600_e36948_d_n12;
        locals.var_vgbgmt_dn17 = assign26600_e36948_d_n17;
        locals.var_vgbgmt_rv = 0.0;

        let (assign26610_e36966, assign26610_e36966_d_n0, assign26610_e36966_d_n2, assign26610_e36966_d_n6, assign26610_e36966_d_n7, assign26610_e36966_d_n10, assign26610_e36966_d_n11, assign26610_e36966_d_n12, assign26610_e36966_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26610_e36956: f64 = (locals.var_flg_overs * locals.var_vsbgmt);
        let assign26610_e36959: f64 = (locals.var_flg_overd * locals.var_vdbgmt);
        let assign26610_e36960: f64 = (assign26610_e36956 + assign26610_e36959);
        let assign26610_e36963: f64 = (10.0 * 2.220446049250313e-16);
        let assign26610_e36964: f64 = (assign26610_e36960 + assign26610_e36963);
        (assign26610_e36964, ((locals.var_flg_overs * locals.var_vsbgmt_dn0) + (locals.var_flg_overd * locals.var_vdbgmt_dn0)), ((locals.var_flg_overs * locals.var_vsbgmt_dn2) + (locals.var_flg_overd * locals.var_vdbgmt_dn2)), ((locals.var_flg_overs * locals.var_vsbgmt_dn6) + (locals.var_flg_overd * locals.var_vdbgmt_dn6)), ((locals.var_flg_overs * locals.var_vsbgmt_dn7) + (locals.var_flg_overd * locals.var_vdbgmt_dn7)), ((locals.var_flg_overs * locals.var_vsbgmt_dn10) + (locals.var_flg_overd * locals.var_vdbgmt_dn10)), ((locals.var_flg_overs * locals.var_vsbgmt_dn11) + (locals.var_flg_overd * locals.var_vdbgmt_dn11)), ((locals.var_flg_overs * locals.var_vsbgmt_dn12) + (locals.var_flg_overd * locals.var_vdbgmt_dn12)), ((locals.var_flg_overs * locals.var_vsbgmt_dn17) + (locals.var_flg_overd * locals.var_vdbgmt_dn17)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn12, locals.var_vxbgmt_dn17,)
    }
};
        locals.var_vxbgmt = assign26610_e36966;
        locals.var_vxbgmt_dn0 = assign26610_e36966_d_n0;
        locals.var_vxbgmt_dn2 = assign26610_e36966_d_n2;
        locals.var_vxbgmt_dn6 = assign26610_e36966_d_n6;
        locals.var_vxbgmt_dn7 = assign26610_e36966_d_n7;
        locals.var_vxbgmt_dn10 = assign26610_e36966_d_n10;
        locals.var_vxbgmt_dn11 = assign26610_e36966_d_n11;
        locals.var_vxbgmt_dn12 = assign26610_e36966_d_n12;
        locals.var_vxbgmt_dn17 = assign26610_e36966_d_n17;
        locals.var_vxbgmt_rv = 0.0;

        let (assign26620_e36975, assign26620_e36975_d_n0, assign26620_e36975_d_n2, assign26620_e36975_d_n6, assign26620_e36975_d_n7, assign26620_e36975_d_n10, assign26620_e36975_d_n11, assign26620_e36975_d_n12, assign26620_e36975_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26620_e36973: f64 = (-locals.var_vxbgmt);
        (assign26620_e36973, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn11), (-locals.var_vxbgmt_dn12), (-locals.var_vxbgmt_dn17),)
    } else {
        (locals.var_t0__blk774, locals.var_t0__blk774_dn0, locals.var_t0__blk774_dn2, locals.var_t0__blk774_dn6, locals.var_t0__blk774_dn7, locals.var_t0__blk774_dn10, locals.var_t0__blk774_dn11, locals.var_t0__blk774_dn12, locals.var_t0__blk774_dn17,)
    }
};
        locals.var_t0__blk774 = assign26620_e36975;
        locals.var_t0__blk774_dn0 = assign26620_e36975_d_n0;
        locals.var_t0__blk774_dn2 = assign26620_e36975_d_n2;
        locals.var_t0__blk774_dn6 = assign26620_e36975_d_n6;
        locals.var_t0__blk774_dn7 = assign26620_e36975_d_n7;
        locals.var_t0__blk774_dn10 = assign26620_e36975_d_n10;
        locals.var_t0__blk774_dn11 = assign26620_e36975_d_n11;
        locals.var_t0__blk774_dn12 = assign26620_e36975_d_n12;
        locals.var_t0__blk774_dn17 = assign26620_e36975_d_n17;
        locals.var_t0__blk774_rv = 0.0;

        let assign26630_e36978: f64 = if locals.var_t0__blk774 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard874 = assign26630_e36978;
        locals.var_guard874_rv = 0.0;

        let (assign26640_e36990, assign26640_e36990_d_n0, assign26640_e36990_d_n2, assign26640_e36990_d_n6, assign26640_e36990_d_n7, assign26640_e36990_d_n10, assign26640_e36990_d_n11, assign26640_e36990_d_n12, assign26640_e36990_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard874 != 0.0)) {
        let assign26640_e36988: f64 = (locals.var_t0__blk774 - locals.var_vbs_bnd);
        (assign26640_e36988, locals.var_t0__blk774_dn0, locals.var_t0__blk774_dn2, locals.var_t0__blk774_dn6, locals.var_t0__blk774_dn7, locals.var_t0__blk774_dn10, locals.var_t0__blk774_dn11, locals.var_t0__blk774_dn12, locals.var_t0__blk774_dn17,)
    } else {
        (locals.var_t1__blk775, locals.var_t1__blk775_dn0, locals.var_t1__blk775_dn2, locals.var_t1__blk775_dn6, locals.var_t1__blk775_dn7, locals.var_t1__blk775_dn10, locals.var_t1__blk775_dn11, locals.var_t1__blk775_dn12, locals.var_t1__blk775_dn17,)
    }
};
        locals.var_t1__blk775 = assign26640_e36990;
        locals.var_t1__blk775_dn0 = assign26640_e36990_d_n0;
        locals.var_t1__blk775_dn2 = assign26640_e36990_d_n2;
        locals.var_t1__blk775_dn6 = assign26640_e36990_d_n6;
        locals.var_t1__blk775_dn7 = assign26640_e36990_d_n7;
        locals.var_t1__blk775_dn10 = assign26640_e36990_d_n10;
        locals.var_t1__blk775_dn11 = assign26640_e36990_d_n11;
        locals.var_t1__blk775_dn12 = assign26640_e36990_d_n12;
        locals.var_t1__blk775_dn17 = assign26640_e36990_d_n17;
        locals.var_t1__blk775_rv = 0.0;

        let (assign26650_e37002, assign26650_e37002_d_n0, assign26650_e37002_d_n2, assign26650_e37002_d_n6, assign26650_e37002_d_n7, assign26650_e37002_d_n10, assign26650_e37002_d_n11, assign26650_e37002_d_n12, assign26650_e37002_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard874 != 0.0)) {
        let assign26650_e37000: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign26650_e37000, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk776, locals.var_t2__blk776_dn0, locals.var_t2__blk776_dn2, locals.var_t2__blk776_dn6, locals.var_t2__blk776_dn7, locals.var_t2__blk776_dn10, locals.var_t2__blk776_dn11, locals.var_t2__blk776_dn12, locals.var_t2__blk776_dn17,)
    }
};
        locals.var_t2__blk776 = assign26650_e37002;
        locals.var_t2__blk776_dn0 = assign26650_e37002_d_n0;
        locals.var_t2__blk776_dn2 = assign26650_e37002_d_n2;
        locals.var_t2__blk776_dn6 = assign26650_e37002_d_n6;
        locals.var_t2__blk776_dn7 = assign26650_e37002_d_n7;
        locals.var_t2__blk776_dn10 = assign26650_e37002_d_n10;
        locals.var_t2__blk776_dn11 = assign26650_e37002_d_n11;
        locals.var_t2__blk776_dn12 = assign26650_e37002_d_n12;
        locals.var_t2__blk776_dn17 = assign26650_e37002_d_n17;
        locals.var_t2__blk776_rv = 0.0;

        let (assign26660_e37014, assign26660_e37014_d_n0, assign26660_e37014_d_n2, assign26660_e37014_d_n6, assign26660_e37014_d_n7, assign26660_e37014_d_n10, assign26660_e37014_d_n11, assign26660_e37014_d_n12, assign26660_e37014_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard874 != 0.0)) {
        let assign26660_e37012: f64 = (locals.var_t1__blk775 / locals.var_t2__blk776);
        (assign26660_e37012, (((locals.var_t1__blk775_dn0 * locals.var_t2__blk776) - (locals.var_t1__blk775 * locals.var_t2__blk776_dn0)) / (locals.var_t2__blk776 * locals.var_t2__blk776)), (((locals.var_t1__blk775_dn2 * locals.var_t2__blk776) - (locals.var_t1__blk775 * locals.var_t2__blk776_dn2)) / (locals.var_t2__blk776 * locals.var_t2__blk776)), (((locals.var_t1__blk775_dn6 * locals.var_t2__blk776) - (locals.var_t1__blk775 * locals.var_t2__blk776_dn6)) / (locals.var_t2__blk776 * locals.var_t2__blk776)), (((locals.var_t1__blk775_dn7 * locals.var_t2__blk776) - (locals.var_t1__blk775 * locals.var_t2__blk776_dn7)) / (locals.var_t2__blk776 * locals.var_t2__blk776)), (((locals.var_t1__blk775_dn10 * locals.var_t2__blk776) - (locals.var_t1__blk775 * locals.var_t2__blk776_dn10)) / (locals.var_t2__blk776 * locals.var_t2__blk776)), (((locals.var_t1__blk775_dn11 * locals.var_t2__blk776) - (locals.var_t1__blk775 * locals.var_t2__blk776_dn11)) / (locals.var_t2__blk776 * locals.var_t2__blk776)), (((locals.var_t1__blk775_dn12 * locals.var_t2__blk776) - (locals.var_t1__blk775 * locals.var_t2__blk776_dn12)) / (locals.var_t2__blk776 * locals.var_t2__blk776)), (((locals.var_t1__blk775_dn17 * locals.var_t2__blk776) - (locals.var_t1__blk775 * locals.var_t2__blk776_dn17)) / (locals.var_t2__blk776 * locals.var_t2__blk776)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign26660_e37014;
        locals.var_tmf1_dn0 = assign26660_e37014_d_n0;
        locals.var_tmf1_dn2 = assign26660_e37014_d_n2;
        locals.var_tmf1_dn6 = assign26660_e37014_d_n6;
        locals.var_tmf1_dn7 = assign26660_e37014_d_n7;
        locals.var_tmf1_dn10 = assign26660_e37014_d_n10;
        locals.var_tmf1_dn11 = assign26660_e37014_d_n11;
        locals.var_tmf1_dn12 = assign26660_e37014_d_n12;
        locals.var_tmf1_dn17 = assign26660_e37014_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign26670_e37026, assign26670_e37026_d_n0, assign26670_e37026_d_n2, assign26670_e37026_d_n6, assign26670_e37026_d_n7, assign26670_e37026_d_n10, assign26670_e37026_d_n11, assign26670_e37026_d_n12, assign26670_e37026_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard874 != 0.0)) {
        let assign26670_e37024: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign26670_e37024, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign26670_e37026;
        locals.var_tmf2_dn0 = assign26670_e37026_d_n0;
        locals.var_tmf2_dn2 = assign26670_e37026_d_n2;
        locals.var_tmf2_dn6 = assign26670_e37026_d_n6;
        locals.var_tmf2_dn7 = assign26670_e37026_d_n7;
        locals.var_tmf2_dn10 = assign26670_e37026_d_n10;
        locals.var_tmf2_dn11 = assign26670_e37026_d_n11;
        locals.var_tmf2_dn12 = assign26670_e37026_d_n12;
        locals.var_tmf2_dn17 = assign26670_e37026_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign26680_e37038, assign26680_e37038_d_n0, assign26680_e37038_d_n2, assign26680_e37038_d_n6, assign26680_e37038_d_n7, assign26680_e37038_d_n10, assign26680_e37038_d_n11, assign26680_e37038_d_n12, assign26680_e37038_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard874 != 0.0)) {
        let assign26680_e37036: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign26680_e37036, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12, locals.var_tmf3_dn17,)
    }
};
        locals.var_tmf3 = assign26680_e37038;
        locals.var_tmf3_dn0 = assign26680_e37038_d_n0;
        locals.var_tmf3_dn2 = assign26680_e37038_d_n2;
        locals.var_tmf3_dn6 = assign26680_e37038_d_n6;
        locals.var_tmf3_dn7 = assign26680_e37038_d_n7;
        locals.var_tmf3_dn10 = assign26680_e37038_d_n10;
        locals.var_tmf3_dn11 = assign26680_e37038_d_n11;
        locals.var_tmf3_dn12 = assign26680_e37038_d_n12;
        locals.var_tmf3_dn17 = assign26680_e37038_d_n17;
        locals.var_tmf3_rv = 0.0;

        let (assign26690_e37050, assign26690_e37050_d_n0, assign26690_e37050_d_n2, assign26690_e37050_d_n6, assign26690_e37050_d_n7, assign26690_e37050_d_n10, assign26690_e37050_d_n11, assign26690_e37050_d_n12, assign26690_e37050_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard874 != 0.0)) {
        let assign26690_e37048: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign26690_e37048, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn17)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn12, locals.var_tmf4_dn17,)
    }
};
        locals.var_tmf4 = assign26690_e37050;
        locals.var_tmf4_dn0 = assign26690_e37050_d_n0;
        locals.var_tmf4_dn2 = assign26690_e37050_d_n2;
        locals.var_tmf4_dn6 = assign26690_e37050_d_n6;
        locals.var_tmf4_dn7 = assign26690_e37050_d_n7;
        locals.var_tmf4_dn10 = assign26690_e37050_d_n10;
        locals.var_tmf4_dn11 = assign26690_e37050_d_n11;
        locals.var_tmf4_dn12 = assign26690_e37050_d_n12;
        locals.var_tmf4_dn17 = assign26690_e37050_d_n17;
        locals.var_tmf4_rv = 0.0;

        let (assign26700_e37070, assign26700_e37070_d_n0, assign26700_e37070_d_n2, assign26700_e37070_d_n6, assign26700_e37070_d_n7, assign26700_e37070_d_n10, assign26700_e37070_d_n11, assign26700_e37070_d_n12, assign26700_e37070_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard874 != 0.0)) {
        let assign26700_e37061: f64 = (1.0 + locals.var_tmf1);
        let assign26700_e37063: f64 = (assign26700_e37061 + locals.var_tmf2);
        let assign26700_e37065: f64 = (assign26700_e37063 + locals.var_tmf3);
        let assign26700_e37067: f64 = (assign26700_e37065 + locals.var_tmf4);
        let assign26700_e37068: f64 = (1.0 / assign26700_e37067);
        (assign26700_e37068, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign26700_e37067 * assign26700_e37067))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign26700_e37067 * assign26700_e37067))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign26700_e37067 * assign26700_e37067))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign26700_e37067 * assign26700_e37067))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign26700_e37067 * assign26700_e37067))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign26700_e37067 * assign26700_e37067))), (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign26700_e37067 * assign26700_e37067))), (-((((locals.var_tmf1_dn17 + locals.var_tmf2_dn17) + locals.var_tmf3_dn17) + locals.var_tmf4_dn17) / (assign26700_e37067 * assign26700_e37067))),)
    } else {
        (locals.var_ty__blk782, locals.var_ty__blk782_dn0, locals.var_ty__blk782_dn2, locals.var_ty__blk782_dn6, locals.var_ty__blk782_dn7, locals.var_ty__blk782_dn10, locals.var_ty__blk782_dn11, locals.var_ty__blk782_dn12, locals.var_ty__blk782_dn17,)
    }
};
        locals.var_ty__blk782 = assign26700_e37070;
        locals.var_ty__blk782_dn0 = assign26700_e37070_d_n0;
        locals.var_ty__blk782_dn2 = assign26700_e37070_d_n2;
        locals.var_ty__blk782_dn6 = assign26700_e37070_d_n6;
        locals.var_ty__blk782_dn7 = assign26700_e37070_d_n7;
        locals.var_ty__blk782_dn10 = assign26700_e37070_d_n10;
        locals.var_ty__blk782_dn11 = assign26700_e37070_d_n11;
        locals.var_ty__blk782_dn12 = assign26700_e37070_d_n12;
        locals.var_ty__blk782_dn17 = assign26700_e37070_d_n17;
        locals.var_ty__blk782_rv = 0.0;

        let (assign26720_e37111, assign26720_e37111_d_n0, assign26720_e37111_d_n2, assign26720_e37111_d_n6, assign26720_e37111_d_n7, assign26720_e37111_d_n10, assign26720_e37111_d_n11, assign26720_e37111_d_n12, assign26720_e37111_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard874 != 0.0)) {
        let assign26720_e37108: f64 = (1.0 - locals.var_ty__blk782);
        let assign26720_e37109: f64 = (locals.var_t2__blk776 * assign26720_e37108);
        (assign26720_e37109, ((locals.var_t2__blk776_dn0 * assign26720_e37108) + (locals.var_t2__blk776 * (-locals.var_ty__blk782_dn0))), ((locals.var_t2__blk776_dn2 * assign26720_e37108) + (locals.var_t2__blk776 * (-locals.var_ty__blk782_dn2))), ((locals.var_t2__blk776_dn6 * assign26720_e37108) + (locals.var_t2__blk776 * (-locals.var_ty__blk782_dn6))), ((locals.var_t2__blk776_dn7 * assign26720_e37108) + (locals.var_t2__blk776 * (-locals.var_ty__blk782_dn7))), ((locals.var_t2__blk776_dn10 * assign26720_e37108) + (locals.var_t2__blk776 * (-locals.var_ty__blk782_dn10))), ((locals.var_t2__blk776_dn11 * assign26720_e37108) + (locals.var_t2__blk776 * (-locals.var_ty__blk782_dn11))), ((locals.var_t2__blk776_dn12 * assign26720_e37108) + (locals.var_t2__blk776 * (-locals.var_ty__blk782_dn12))), ((locals.var_t2__blk776_dn17 * assign26720_e37108) + (locals.var_t2__blk776 * (-locals.var_ty__blk782_dn17))),)
    } else {
        (locals.var_ty__blk782, locals.var_ty__blk782_dn0, locals.var_ty__blk782_dn2, locals.var_ty__blk782_dn6, locals.var_ty__blk782_dn7, locals.var_ty__blk782_dn10, locals.var_ty__blk782_dn11, locals.var_ty__blk782_dn12, locals.var_ty__blk782_dn17,)
    }
};
        locals.var_ty__blk782 = assign26720_e37111;
        locals.var_ty__blk782_dn0 = assign26720_e37111_d_n0;
        locals.var_ty__blk782_dn2 = assign26720_e37111_d_n2;
        locals.var_ty__blk782_dn6 = assign26720_e37111_d_n6;
        locals.var_ty__blk782_dn7 = assign26720_e37111_d_n7;
        locals.var_ty__blk782_dn10 = assign26720_e37111_d_n10;
        locals.var_ty__blk782_dn11 = assign26720_e37111_d_n11;
        locals.var_ty__blk782_dn12 = assign26720_e37111_d_n12;
        locals.var_ty__blk782_dn17 = assign26720_e37111_d_n17;
        locals.var_ty__blk782_rv = 0.0;

        let (assign26740_e37134, assign26740_e37134_d_n0, assign26740_e37134_d_n2, assign26740_e37134_d_n6, assign26740_e37134_d_n7, assign26740_e37134_d_n10, assign26740_e37134_d_n11, assign26740_e37134_d_n12, assign26740_e37134_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard874 != 0.0)) {
        let assign26740_e37132: f64 = (locals.var_vbs_bnd + locals.var_ty__blk782);
        (assign26740_e37132, locals.var_ty__blk782_dn0, locals.var_ty__blk782_dn2, locals.var_ty__blk782_dn6, locals.var_ty__blk782_dn7, locals.var_ty__blk782_dn10, locals.var_ty__blk782_dn11, locals.var_ty__blk782_dn12, locals.var_ty__blk782_dn17,)
    } else {
        (locals.var_t10__blk779, locals.var_t10__blk779_dn0, locals.var_t10__blk779_dn2, locals.var_t10__blk779_dn6, locals.var_t10__blk779_dn7, locals.var_t10__blk779_dn10, locals.var_t10__blk779_dn11, locals.var_t10__blk779_dn12, locals.var_t10__blk779_dn17,)
    }
};
        locals.var_t10__blk779 = assign26740_e37134;
        locals.var_t10__blk779_dn0 = assign26740_e37134_d_n0;
        locals.var_t10__blk779_dn2 = assign26740_e37134_d_n2;
        locals.var_t10__blk779_dn6 = assign26740_e37134_d_n6;
        locals.var_t10__blk779_dn7 = assign26740_e37134_d_n7;
        locals.var_t10__blk779_dn10 = assign26740_e37134_d_n10;
        locals.var_t10__blk779_dn11 = assign26740_e37134_d_n11;
        locals.var_t10__blk779_dn12 = assign26740_e37134_d_n12;
        locals.var_t10__blk779_dn17 = assign26740_e37134_d_n17;
        locals.var_t10__blk779_rv = 0.0;

        let (assign26750_e37145, assign26750_e37145_d_n0, assign26750_e37145_d_n2, assign26750_e37145_d_n6, assign26750_e37145_d_n7, assign26750_e37145_d_n10, assign26750_e37145_d_n11, assign26750_e37145_d_n12, assign26750_e37145_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard874 == 0.0)) {
        (locals.var_t0__blk774, locals.var_t0__blk774_dn0, locals.var_t0__blk774_dn2, locals.var_t0__blk774_dn6, locals.var_t0__blk774_dn7, locals.var_t0__blk774_dn10, locals.var_t0__blk774_dn11, locals.var_t0__blk774_dn12, locals.var_t0__blk774_dn17,)
    } else {
        (locals.var_t10__blk779, locals.var_t10__blk779_dn0, locals.var_t10__blk779_dn2, locals.var_t10__blk779_dn6, locals.var_t10__blk779_dn7, locals.var_t10__blk779_dn10, locals.var_t10__blk779_dn11, locals.var_t10__blk779_dn12, locals.var_t10__blk779_dn17,)
    }
};
        locals.var_t10__blk779 = assign26750_e37145;
        locals.var_t10__blk779_dn0 = assign26750_e37145_d_n0;
        locals.var_t10__blk779_dn2 = assign26750_e37145_d_n2;
        locals.var_t10__blk779_dn6 = assign26750_e37145_d_n6;
        locals.var_t10__blk779_dn7 = assign26750_e37145_d_n7;
        locals.var_t10__blk779_dn10 = assign26750_e37145_d_n10;
        locals.var_t10__blk779_dn11 = assign26750_e37145_d_n11;
        locals.var_t10__blk779_dn12 = assign26750_e37145_d_n12;
        locals.var_t10__blk779_dn17 = assign26750_e37145_d_n17;
        locals.var_t10__blk779_rv = 0.0;

        let (assign26770_e37167, assign26770_e37167_d_n0, assign26770_e37167_d_n2, assign26770_e37167_d_n6, assign26770_e37167_d_n7, assign26770_e37167_d_n10, assign26770_e37167_d_n11, assign26770_e37167_d_n12, assign26770_e37167_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26770_e37163: f64 = (-locals.var_t10__blk779);
        let assign26770_e37165: f64 = (assign26770_e37163 - 1e-12);
        (assign26770_e37165, (-locals.var_t10__blk779_dn0), (-locals.var_t10__blk779_dn2), (-locals.var_t10__blk779_dn6), (-locals.var_t10__blk779_dn7), (-locals.var_t10__blk779_dn10), (-locals.var_t10__blk779_dn11), (-locals.var_t10__blk779_dn12), (-locals.var_t10__blk779_dn17),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn12, locals.var_vxbgmtcl_dn17,)
    }
};
        locals.var_vxbgmtcl = assign26770_e37167;
        locals.var_vxbgmtcl_dn0 = assign26770_e37167_d_n0;
        locals.var_vxbgmtcl_dn2 = assign26770_e37167_d_n2;
        locals.var_vxbgmtcl_dn6 = assign26770_e37167_d_n6;
        locals.var_vxbgmtcl_dn7 = assign26770_e37167_d_n7;
        locals.var_vxbgmtcl_dn10 = assign26770_e37167_d_n10;
        locals.var_vxbgmtcl_dn11 = assign26770_e37167_d_n11;
        locals.var_vxbgmtcl_dn12 = assign26770_e37167_d_n12;
        locals.var_vxbgmtcl_dn17 = assign26770_e37167_d_n17;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign26780_e37177, assign26780_e37177_d_n0, assign26780_e37177_d_n2, assign26780_e37177_d_n6, assign26780_e37177_d_n7, assign26780_e37177_d_n10, assign26780_e37177_d_n11, assign26780_e37177_d_n12, assign26780_e37177_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26780_e37175: f64 = (locals.var_cnst0over * locals.var_cox0_inv);
        (assign26780_e37175, (locals.var_cnst0over_dn0 * locals.var_cox0_inv), (locals.var_cnst0over_dn2 * locals.var_cox0_inv), (locals.var_cnst0over_dn6 * locals.var_cox0_inv), (locals.var_cnst0over_dn7 * locals.var_cox0_inv), (locals.var_cnst0over_dn10 * locals.var_cox0_inv), (locals.var_cnst0over_dn11 * locals.var_cox0_inv), (locals.var_cnst0over_dn12 * locals.var_cox0_inv), (locals.var_cnst0over_dn17 * locals.var_cox0_inv),)
    } else {
        (locals.var_fac1__blk804, locals.var_fac1__blk804_dn0, locals.var_fac1__blk804_dn2, locals.var_fac1__blk804_dn6, locals.var_fac1__blk804_dn7, locals.var_fac1__blk804_dn10, locals.var_fac1__blk804_dn11, locals.var_fac1__blk804_dn12, locals.var_fac1__blk804_dn17,)
    }
};
        locals.var_fac1__blk804 = assign26780_e37177;
        locals.var_fac1__blk804_dn0 = assign26780_e37177_d_n0;
        locals.var_fac1__blk804_dn2 = assign26780_e37177_d_n2;
        locals.var_fac1__blk804_dn6 = assign26780_e37177_d_n6;
        locals.var_fac1__blk804_dn7 = assign26780_e37177_d_n7;
        locals.var_fac1__blk804_dn10 = assign26780_e37177_d_n10;
        locals.var_fac1__blk804_dn11 = assign26780_e37177_d_n11;
        locals.var_fac1__blk804_dn12 = assign26780_e37177_d_n12;
        locals.var_fac1__blk804_dn17 = assign26780_e37177_d_n17;
        locals.var_fac1__blk804_rv = 0.0;

        let (assign26790_e37187, assign26790_e37187_d_n0, assign26790_e37187_d_n2, assign26790_e37187_d_n6, assign26790_e37187_d_n7, assign26790_e37187_d_n10, assign26790_e37187_d_n11, assign26790_e37187_d_n12, assign26790_e37187_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26790_e37185: f64 = (locals.var_fac1__blk804 * locals.var_fac1__blk804);
        (assign26790_e37185, ((locals.var_fac1__blk804_dn0 * locals.var_fac1__blk804) + (locals.var_fac1__blk804 * locals.var_fac1__blk804_dn0)), ((locals.var_fac1__blk804_dn2 * locals.var_fac1__blk804) + (locals.var_fac1__blk804 * locals.var_fac1__blk804_dn2)), ((locals.var_fac1__blk804_dn6 * locals.var_fac1__blk804) + (locals.var_fac1__blk804 * locals.var_fac1__blk804_dn6)), ((locals.var_fac1__blk804_dn7 * locals.var_fac1__blk804) + (locals.var_fac1__blk804 * locals.var_fac1__blk804_dn7)), ((locals.var_fac1__blk804_dn10 * locals.var_fac1__blk804) + (locals.var_fac1__blk804 * locals.var_fac1__blk804_dn10)), ((locals.var_fac1__blk804_dn11 * locals.var_fac1__blk804) + (locals.var_fac1__blk804 * locals.var_fac1__blk804_dn11)), ((locals.var_fac1__blk804_dn12 * locals.var_fac1__blk804) + (locals.var_fac1__blk804 * locals.var_fac1__blk804_dn12)), ((locals.var_fac1__blk804_dn17 * locals.var_fac1__blk804) + (locals.var_fac1__blk804 * locals.var_fac1__blk804_dn17)),)
    } else {
        (locals.var_fac1p2__blk805, locals.var_fac1p2__blk805_dn0, locals.var_fac1p2__blk805_dn2, locals.var_fac1p2__blk805_dn6, locals.var_fac1p2__blk805_dn7, locals.var_fac1p2__blk805_dn10, locals.var_fac1p2__blk805_dn11, locals.var_fac1p2__blk805_dn12, locals.var_fac1p2__blk805_dn17,)
    }
};
        locals.var_fac1p2__blk805 = assign26790_e37187;
        locals.var_fac1p2__blk805_dn0 = assign26790_e37187_d_n0;
        locals.var_fac1p2__blk805_dn2 = assign26790_e37187_d_n2;
        locals.var_fac1p2__blk805_dn6 = assign26790_e37187_d_n6;
        locals.var_fac1p2__blk805_dn7 = assign26790_e37187_d_n7;
        locals.var_fac1p2__blk805_dn10 = assign26790_e37187_d_n10;
        locals.var_fac1p2__blk805_dn11 = assign26790_e37187_d_n11;
        locals.var_fac1p2__blk805_dn12 = assign26790_e37187_d_n12;
        locals.var_fac1p2__blk805_dn17 = assign26790_e37187_d_n17;
        locals.var_fac1p2__blk805_rv = 0.0;

        let (assign26800_e37197, assign26800_e37197_d_n0, assign26800_e37197_d_n2, assign26800_e37197_d_n6, assign26800_e37197_d_n7, assign26800_e37197_d_n10, assign26800_e37197_d_n11, assign26800_e37197_d_n12, assign26800_e37197_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26800_e37195: f64 = (locals.var_vgbgmt - locals.var_uc_vfbbt);
        (assign26800_e37195, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn10, locals.var_vgbgmt_dn11, locals.var_vgbgmt_dn12, locals.var_vgbgmt_dn17,)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn0, locals.var_vgpld_dn2, locals.var_vgpld_dn6, locals.var_vgpld_dn7, locals.var_vgpld_dn10, locals.var_vgpld_dn11, locals.var_vgpld_dn12, locals.var_vgpld_dn17,)
    }
};
        locals.var_vgpld = assign26800_e37197;
        locals.var_vgpld_dn0 = assign26800_e37197_d_n0;
        locals.var_vgpld_dn2 = assign26800_e37197_d_n2;
        locals.var_vgpld_dn6 = assign26800_e37197_d_n6;
        locals.var_vgpld_dn7 = assign26800_e37197_d_n7;
        locals.var_vgpld_dn10 = assign26800_e37197_d_n10;
        locals.var_vgpld_dn11 = assign26800_e37197_d_n11;
        locals.var_vgpld_dn12 = assign26800_e37197_d_n12;
        locals.var_vgpld_dn17 = assign26800_e37197_d_n17;
        locals.var_vgpld_rv = 0.0;

        let (assign26810_e37207, assign26810_e37207_d_n0, assign26810_e37207_d_n2, assign26810_e37207_d_n6, assign26810_e37207_d_n7, assign26810_e37207_d_n10, assign26810_e37207_d_n11, assign26810_e37207_d_n12, assign26810_e37207_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26810_e37205: f64 = (locals.var_uc_nsubbttub / locals.var_nin);
        (assign26810_e37205, (((locals.var_uc_nsubbttub_dn0 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn2 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn6 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn7 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn10 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn11 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn12 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn12)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn17 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn17)) / (locals.var_nin * locals.var_nin)),)
    } else {
        (locals.var_t0__blk774, locals.var_t0__blk774_dn0, locals.var_t0__blk774_dn2, locals.var_t0__blk774_dn6, locals.var_t0__blk774_dn7, locals.var_t0__blk774_dn10, locals.var_t0__blk774_dn11, locals.var_t0__blk774_dn12, locals.var_t0__blk774_dn17,)
    }
};
        locals.var_t0__blk774 = assign26810_e37207;
        locals.var_t0__blk774_dn0 = assign26810_e37207_d_n0;
        locals.var_t0__blk774_dn2 = assign26810_e37207_d_n2;
        locals.var_t0__blk774_dn6 = assign26810_e37207_d_n6;
        locals.var_t0__blk774_dn7 = assign26810_e37207_d_n7;
        locals.var_t0__blk774_dn10 = assign26810_e37207_d_n10;
        locals.var_t0__blk774_dn11 = assign26810_e37207_d_n11;
        locals.var_t0__blk774_dn12 = assign26810_e37207_d_n12;
        locals.var_t0__blk774_dn17 = assign26810_e37207_d_n17;
        locals.var_t0__blk774_rv = 0.0;

        let (assign26820_e37220, assign26820_e37220_d_n0, assign26820_e37220_d_n2, assign26820_e37220_d_n6, assign26820_e37220_d_n7, assign26820_e37220_d_n10, assign26820_e37220_d_n11, assign26820_e37220_d_n12, assign26820_e37220_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26820_e37215: f64 = (2.0 / locals.var_beta);
        let assign26820_e37217: f64 = (locals.var_t0__blk774).ln();
        let assign26820_e37218: f64 = (assign26820_e37215 * assign26820_e37217);
        (assign26820_e37218, (assign26820_e37215 * (locals.var_t0__blk774_dn0 / locals.var_t0__blk774)), (assign26820_e37215 * (locals.var_t0__blk774_dn2 / locals.var_t0__blk774)), (assign26820_e37215 * (locals.var_t0__blk774_dn6 / locals.var_t0__blk774)), (assign26820_e37215 * (locals.var_t0__blk774_dn7 / locals.var_t0__blk774)), (((-((2.0 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign26820_e37217) + (assign26820_e37215 * (locals.var_t0__blk774_dn10 / locals.var_t0__blk774))), (assign26820_e37215 * (locals.var_t0__blk774_dn11 / locals.var_t0__blk774)), (assign26820_e37215 * (locals.var_t0__blk774_dn12 / locals.var_t0__blk774)), (assign26820_e37215 * (locals.var_t0__blk774_dn17 / locals.var_t0__blk774)),)
    } else {
        (locals.var_pb2over, locals.var_pb2over_dn0, locals.var_pb2over_dn2, locals.var_pb2over_dn6, locals.var_pb2over_dn7, locals.var_pb2over_dn10, locals.var_pb2over_dn11, locals.var_pb2over_dn12, locals.var_pb2over_dn17,)
    }
};
        locals.var_pb2over = assign26820_e37220;
        locals.var_pb2over_dn0 = assign26820_e37220_d_n0;
        locals.var_pb2over_dn2 = assign26820_e37220_d_n2;
        locals.var_pb2over_dn6 = assign26820_e37220_d_n6;
        locals.var_pb2over_dn7 = assign26820_e37220_d_n7;
        locals.var_pb2over_dn10 = assign26820_e37220_d_n10;
        locals.var_pb2over_dn11 = assign26820_e37220_d_n11;
        locals.var_pb2over_dn12 = assign26820_e37220_d_n12;
        locals.var_pb2over_dn17 = assign26820_e37220_d_n17;
        locals.var_pb2over_rv = 0.0;

        let (assign26830_e37229, assign26830_e37229_d_n0, assign26830_e37229_d_n2, assign26830_e37229_d_n6, assign26830_e37229_d_n7, assign26830_e37229_d_n10, assign26830_e37229_d_n11, assign26830_e37229_d_n12, assign26830_e37229_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign26830_e37227: f64 = (-locals.var_vxbgmtcl);
        (assign26830_e37227, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn12), (-locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn12, locals.var_vgb_fb_ld_dn17,)
    }
};
        locals.var_vgb_fb_ld = assign26830_e37229;
        locals.var_vgb_fb_ld_dn0 = assign26830_e37229_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign26830_e37229_d_n2;
        locals.var_vgb_fb_ld_dn6 = assign26830_e37229_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign26830_e37229_d_n7;
        locals.var_vgb_fb_ld_dn10 = assign26830_e37229_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign26830_e37229_d_n11;
        locals.var_vgb_fb_ld_dn12 = assign26830_e37229_d_n12;
        locals.var_vgb_fb_ld_dn17 = assign26830_e37229_d_n17;
        locals.var_vgb_fb_ld_rv = 0.0;

        let assign26840_e37232: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard875 = assign26840_e37232;
        locals.var_guard875_rv = 0.0;

        let (assign26860_e37257, assign26860_e37257_d_n0, assign26860_e37257_d_n2, assign26860_e37257_d_n6, assign26860_e37257_d_n7, assign26860_e37257_d_n10, assign26860_e37257_d_n11, assign26860_e37257_d_n12, assign26860_e37257_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign26860_e37254: f64 = (locals.var_beta * locals.var_cnst0over);
        let assign26860_e37255: f64 = (1.0 / assign26860_e37254);
        (assign26860_e37255, (-((locals.var_beta * locals.var_cnst0over_dn0) / (assign26860_e37254 * assign26860_e37254))), (-((locals.var_beta * locals.var_cnst0over_dn2) / (assign26860_e37254 * assign26860_e37254))), (-((locals.var_beta * locals.var_cnst0over_dn6) / (assign26860_e37254 * assign26860_e37254))), (-((locals.var_beta * locals.var_cnst0over_dn7) / (assign26860_e37254 * assign26860_e37254))), (-(((locals.var_beta_dn10 * locals.var_cnst0over) + (locals.var_beta * locals.var_cnst0over_dn10)) / (assign26860_e37254 * assign26860_e37254))), (-((locals.var_beta * locals.var_cnst0over_dn11) / (assign26860_e37254 * assign26860_e37254))), (-((locals.var_beta * locals.var_cnst0over_dn12) / (assign26860_e37254 * assign26860_e37254))), (-((locals.var_beta * locals.var_cnst0over_dn17) / (assign26860_e37254 * assign26860_e37254))),)
    } else {
        (locals.var_t1__blk775, locals.var_t1__blk775_dn0, locals.var_t1__blk775_dn2, locals.var_t1__blk775_dn6, locals.var_t1__blk775_dn7, locals.var_t1__blk775_dn10, locals.var_t1__blk775_dn11, locals.var_t1__blk775_dn12, locals.var_t1__blk775_dn17,)
    }
};
        locals.var_t1__blk775 = assign26860_e37257;
        locals.var_t1__blk775_dn0 = assign26860_e37257_d_n0;
        locals.var_t1__blk775_dn2 = assign26860_e37257_d_n2;
        locals.var_t1__blk775_dn6 = assign26860_e37257_d_n6;
        locals.var_t1__blk775_dn7 = assign26860_e37257_d_n7;
        locals.var_t1__blk775_dn10 = assign26860_e37257_d_n10;
        locals.var_t1__blk775_dn11 = assign26860_e37257_d_n11;
        locals.var_t1__blk775_dn12 = assign26860_e37257_d_n12;
        locals.var_t1__blk775_dn17 = assign26860_e37257_d_n17;
        locals.var_t1__blk775_rv = 0.0;

        let (assign26870_e37269, assign26870_e37269_d_n0, assign26870_e37269_d_n2, assign26870_e37269_d_n6, assign26870_e37269_d_n7, assign26870_e37269_d_n10, assign26870_e37269_d_n11, assign26870_e37269_d_n12, assign26870_e37269_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign26870_e37267: f64 = (locals.var_t1__blk775 * locals.var_cox0);
        (assign26870_e37267, (locals.var_t1__blk775_dn0 * locals.var_cox0), (locals.var_t1__blk775_dn2 * locals.var_cox0), (locals.var_t1__blk775_dn6 * locals.var_cox0), (locals.var_t1__blk775_dn7 * locals.var_cox0), (locals.var_t1__blk775_dn10 * locals.var_cox0), (locals.var_t1__blk775_dn11 * locals.var_cox0), (locals.var_t1__blk775_dn12 * locals.var_cox0), (locals.var_t1__blk775_dn17 * locals.var_cox0),)
    } else {
        (locals.var_ty__blk782, locals.var_ty__blk782_dn0, locals.var_ty__blk782_dn2, locals.var_ty__blk782_dn6, locals.var_ty__blk782_dn7, locals.var_ty__blk782_dn10, locals.var_ty__blk782_dn11, locals.var_ty__blk782_dn12, locals.var_ty__blk782_dn17,)
    }
};
        locals.var_ty__blk782 = assign26870_e37269;
        locals.var_ty__blk782_dn0 = assign26870_e37269_d_n0;
        locals.var_ty__blk782_dn2 = assign26870_e37269_d_n2;
        locals.var_ty__blk782_dn6 = assign26870_e37269_d_n6;
        locals.var_ty__blk782_dn7 = assign26870_e37269_d_n7;
        locals.var_ty__blk782_dn10 = assign26870_e37269_d_n10;
        locals.var_ty__blk782_dn11 = assign26870_e37269_d_n11;
        locals.var_ty__blk782_dn12 = assign26870_e37269_d_n12;
        locals.var_ty__blk782_dn17 = assign26870_e37269_d_n17;
        locals.var_ty__blk782_rv = 0.0;

        let (assign26880_e37285, assign26880_e37285_d_n0, assign26880_e37285_d_n2, assign26880_e37285_d_n6, assign26880_e37285_d_n7, assign26880_e37285_d_n10, assign26880_e37285_d_n11, assign26880_e37285_d_n12, assign26880_e37285_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign26880_e37280: f64 = (3.0 * 1.414213562373095);
        let assign26880_e37282: f64 = (assign26880_e37280 * locals.var_ty__blk782);
        let assign26880_e37283: f64 = (2.0 + assign26880_e37282);
        (assign26880_e37283, (assign26880_e37280 * locals.var_ty__blk782_dn0), (assign26880_e37280 * locals.var_ty__blk782_dn2), (assign26880_e37280 * locals.var_ty__blk782_dn6), (assign26880_e37280 * locals.var_ty__blk782_dn7), (assign26880_e37280 * locals.var_ty__blk782_dn10), (assign26880_e37280 * locals.var_ty__blk782_dn11), (assign26880_e37280 * locals.var_ty__blk782_dn12), (assign26880_e37280 * locals.var_ty__blk782_dn17),)
    } else {
        (locals.var_ac41__blk809, locals.var_ac41__blk809_dn0, locals.var_ac41__blk809_dn2, locals.var_ac41__blk809_dn6, locals.var_ac41__blk809_dn7, locals.var_ac41__blk809_dn10, locals.var_ac41__blk809_dn11, locals.var_ac41__blk809_dn12, locals.var_ac41__blk809_dn17,)
    }
};
        locals.var_ac41__blk809 = assign26880_e37285;
        locals.var_ac41__blk809_dn0 = assign26880_e37285_d_n0;
        locals.var_ac41__blk809_dn2 = assign26880_e37285_d_n2;
        locals.var_ac41__blk809_dn6 = assign26880_e37285_d_n6;
        locals.var_ac41__blk809_dn7 = assign26880_e37285_d_n7;
        locals.var_ac41__blk809_dn10 = assign26880_e37285_d_n10;
        locals.var_ac41__blk809_dn11 = assign26880_e37285_d_n11;
        locals.var_ac41__blk809_dn12 = assign26880_e37285_d_n12;
        locals.var_ac41__blk809_dn17 = assign26880_e37285_d_n17;
        locals.var_ac41__blk809_rv = 0.0;

        let (assign26890_e37301, assign26890_e37301_d_n0, assign26890_e37301_d_n2, assign26890_e37301_d_n6, assign26890_e37301_d_n7, assign26890_e37301_d_n10, assign26890_e37301_d_n11, assign26890_e37301_d_n12, assign26890_e37301_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign26890_e37295: f64 = (8.0 * locals.var_ac41__blk809);
        let assign26890_e37297: f64 = (assign26890_e37295 * locals.var_ac41__blk809);
        let assign26890_e37299: f64 = (assign26890_e37297 * locals.var_ac41__blk809);
        (assign26890_e37299, (((((8.0 * locals.var_ac41__blk809_dn0) * locals.var_ac41__blk809) + (assign26890_e37295 * locals.var_ac41__blk809_dn0)) * locals.var_ac41__blk809) + (assign26890_e37297 * locals.var_ac41__blk809_dn0)), (((((8.0 * locals.var_ac41__blk809_dn2) * locals.var_ac41__blk809) + (assign26890_e37295 * locals.var_ac41__blk809_dn2)) * locals.var_ac41__blk809) + (assign26890_e37297 * locals.var_ac41__blk809_dn2)), (((((8.0 * locals.var_ac41__blk809_dn6) * locals.var_ac41__blk809) + (assign26890_e37295 * locals.var_ac41__blk809_dn6)) * locals.var_ac41__blk809) + (assign26890_e37297 * locals.var_ac41__blk809_dn6)), (((((8.0 * locals.var_ac41__blk809_dn7) * locals.var_ac41__blk809) + (assign26890_e37295 * locals.var_ac41__blk809_dn7)) * locals.var_ac41__blk809) + (assign26890_e37297 * locals.var_ac41__blk809_dn7)), (((((8.0 * locals.var_ac41__blk809_dn10) * locals.var_ac41__blk809) + (assign26890_e37295 * locals.var_ac41__blk809_dn10)) * locals.var_ac41__blk809) + (assign26890_e37297 * locals.var_ac41__blk809_dn10)), (((((8.0 * locals.var_ac41__blk809_dn11) * locals.var_ac41__blk809) + (assign26890_e37295 * locals.var_ac41__blk809_dn11)) * locals.var_ac41__blk809) + (assign26890_e37297 * locals.var_ac41__blk809_dn11)), (((((8.0 * locals.var_ac41__blk809_dn12) * locals.var_ac41__blk809) + (assign26890_e37295 * locals.var_ac41__blk809_dn12)) * locals.var_ac41__blk809) + (assign26890_e37297 * locals.var_ac41__blk809_dn12)), (((((8.0 * locals.var_ac41__blk809_dn17) * locals.var_ac41__blk809) + (assign26890_e37295 * locals.var_ac41__blk809_dn17)) * locals.var_ac41__blk809) + (assign26890_e37297 * locals.var_ac41__blk809_dn17)),)
    } else {
        (locals.var_ac4__blk810, locals.var_ac4__blk810_dn0, locals.var_ac4__blk810_dn2, locals.var_ac4__blk810_dn6, locals.var_ac4__blk810_dn7, locals.var_ac4__blk810_dn10, locals.var_ac4__blk810_dn11, locals.var_ac4__blk810_dn12, locals.var_ac4__blk810_dn17,)
    }
};
        locals.var_ac4__blk810 = assign26890_e37301;
        locals.var_ac4__blk810_dn0 = assign26890_e37301_d_n0;
        locals.var_ac4__blk810_dn2 = assign26890_e37301_d_n2;
        locals.var_ac4__blk810_dn6 = assign26890_e37301_d_n6;
        locals.var_ac4__blk810_dn7 = assign26890_e37301_d_n7;
        locals.var_ac4__blk810_dn10 = assign26890_e37301_d_n10;
        locals.var_ac4__blk810_dn11 = assign26890_e37301_d_n11;
        locals.var_ac4__blk810_dn12 = assign26890_e37301_d_n12;
        locals.var_ac4__blk810_dn17 = assign26890_e37301_d_n17;
        locals.var_ac4__blk810_rv = 0.0;

    }
}
