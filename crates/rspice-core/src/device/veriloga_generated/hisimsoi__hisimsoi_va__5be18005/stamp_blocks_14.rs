#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

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

    pub(super) fn stamp_reactive_block_96(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26900_e37313, assign26900_e37313_d_n0, assign26900_e37313_d_n2, assign26900_e37313_d_n6, assign26900_e37313_d_n7, assign26900_e37313_d_n10, assign26900_e37313_d_n11, assign26900_e37313_d_n12, assign26900_e37313_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign26900_e37311: f64 = (locals.var_eg - locals.var_pb2over);
        (assign26900_e37311, (locals.var_eg_dn0 - locals.var_pb2over_dn0), (locals.var_eg_dn2 - locals.var_pb2over_dn2), (locals.var_eg_dn6 - locals.var_pb2over_dn6), (locals.var_eg_dn7 - locals.var_pb2over_dn7), (locals.var_eg_dn10 - locals.var_pb2over_dn10), (locals.var_eg_dn11 - locals.var_pb2over_dn11), (locals.var_eg_dn12 - locals.var_pb2over_dn12), (locals.var_eg_dn17 - locals.var_pb2over_dn17),)
    } else {
        (locals.var_ps0_min__blk811, locals.var_ps0_min__blk811_dn0, locals.var_ps0_min__blk811_dn2, locals.var_ps0_min__blk811_dn6, locals.var_ps0_min__blk811_dn7, locals.var_ps0_min__blk811_dn10, locals.var_ps0_min__blk811_dn11, locals.var_ps0_min__blk811_dn12, locals.var_ps0_min__blk811_dn17,)
    }
};
        locals.var_ps0_min__blk811 = assign26900_e37313;
        locals.var_ps0_min__blk811_dn0 = assign26900_e37313_d_n0;
        locals.var_ps0_min__blk811_dn2 = assign26900_e37313_d_n2;
        locals.var_ps0_min__blk811_dn6 = assign26900_e37313_d_n6;
        locals.var_ps0_min__blk811_dn7 = assign26900_e37313_d_n7;
        locals.var_ps0_min__blk811_dn10 = assign26900_e37313_d_n10;
        locals.var_ps0_min__blk811_dn11 = assign26900_e37313_d_n11;
        locals.var_ps0_min__blk811_dn12 = assign26900_e37313_d_n12;
        locals.var_ps0_min__blk811_dn17 = assign26900_e37313_d_n17;
        locals.var_ps0_min__blk811_rv = 0.0;

        let (assign26910_e37327, assign26910_e37327_d_n0, assign26910_e37327_d_n2, assign26910_e37327_d_n6, assign26910_e37327_d_n7, assign26910_e37327_d_n10, assign26910_e37327_d_n11, assign26910_e37327_d_n12, assign26910_e37327_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign26910_e37324: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign26910_e37325: f64 = (locals.var_beta * assign26910_e37324);
        (assign26910_e37325, (locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign26910_e37324) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_tx__blk781, locals.var_tx__blk781_dn0, locals.var_tx__blk781_dn2, locals.var_tx__blk781_dn6, locals.var_tx__blk781_dn7, locals.var_tx__blk781_dn10, locals.var_tx__blk781_dn11, locals.var_tx__blk781_dn12, locals.var_tx__blk781_dn17,)
    }
};
        locals.var_tx__blk781 = assign26910_e37327;
        locals.var_tx__blk781_dn0 = assign26910_e37327_d_n0;
        locals.var_tx__blk781_dn2 = assign26910_e37327_d_n2;
        locals.var_tx__blk781_dn6 = assign26910_e37327_d_n6;
        locals.var_tx__blk781_dn7 = assign26910_e37327_d_n7;
        locals.var_tx__blk781_dn10 = assign26910_e37327_d_n10;
        locals.var_tx__blk781_dn11 = assign26910_e37327_d_n11;
        locals.var_tx__blk781_dn12 = assign26910_e37327_d_n12;
        locals.var_tx__blk781_dn17 = assign26910_e37327_d_n17;
        locals.var_tx__blk781_rv = 0.0;

        let (assign26920_e37347, assign26920_e37347_d_n0, assign26920_e37347_d_n2, assign26920_e37347_d_n6, assign26920_e37347_d_n7, assign26920_e37347_d_n10, assign26920_e37347_d_n11, assign26920_e37347_d_n12, assign26920_e37347_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign26920_e37337: f64 = (7.0 * 1.414213562373095);
        let assign26920_e37340: f64 = (9.0 * locals.var_ty__blk782);
        let assign26920_e37343: f64 = (locals.var_tx__blk781 - 2.0);
        let assign26920_e37344: f64 = (assign26920_e37340 * assign26920_e37343);
        let assign26920_e37345: f64 = (assign26920_e37337 - assign26920_e37344);
        (assign26920_e37345, (-(((9.0 * locals.var_ty__blk782_dn0) * assign26920_e37343) + (assign26920_e37340 * locals.var_tx__blk781_dn0))), (-(((9.0 * locals.var_ty__blk782_dn2) * assign26920_e37343) + (assign26920_e37340 * locals.var_tx__blk781_dn2))), (-(((9.0 * locals.var_ty__blk782_dn6) * assign26920_e37343) + (assign26920_e37340 * locals.var_tx__blk781_dn6))), (-(((9.0 * locals.var_ty__blk782_dn7) * assign26920_e37343) + (assign26920_e37340 * locals.var_tx__blk781_dn7))), (-(((9.0 * locals.var_ty__blk782_dn10) * assign26920_e37343) + (assign26920_e37340 * locals.var_tx__blk781_dn10))), (-(((9.0 * locals.var_ty__blk782_dn11) * assign26920_e37343) + (assign26920_e37340 * locals.var_tx__blk781_dn11))), (-(((9.0 * locals.var_ty__blk782_dn12) * assign26920_e37343) + (assign26920_e37340 * locals.var_tx__blk781_dn12))), (-(((9.0 * locals.var_ty__blk782_dn17) * assign26920_e37343) + (assign26920_e37340 * locals.var_tx__blk781_dn17))),)
    } else {
        (locals.var_ac31__blk812, locals.var_ac31__blk812_dn0, locals.var_ac31__blk812_dn2, locals.var_ac31__blk812_dn6, locals.var_ac31__blk812_dn7, locals.var_ac31__blk812_dn10, locals.var_ac31__blk812_dn11, locals.var_ac31__blk812_dn12, locals.var_ac31__blk812_dn17,)
    }
};
        locals.var_ac31__blk812 = assign26920_e37347;
        locals.var_ac31__blk812_dn0 = assign26920_e37347_d_n0;
        locals.var_ac31__blk812_dn2 = assign26920_e37347_d_n2;
        locals.var_ac31__blk812_dn6 = assign26920_e37347_d_n6;
        locals.var_ac31__blk812_dn7 = assign26920_e37347_d_n7;
        locals.var_ac31__blk812_dn10 = assign26920_e37347_d_n10;
        locals.var_ac31__blk812_dn11 = assign26920_e37347_d_n11;
        locals.var_ac31__blk812_dn12 = assign26920_e37347_d_n12;
        locals.var_ac31__blk812_dn17 = assign26920_e37347_d_n17;
        locals.var_ac31__blk812_rv = 0.0;

        let (assign26930_e37359, assign26930_e37359_d_n0, assign26930_e37359_d_n2, assign26930_e37359_d_n6, assign26930_e37359_d_n7, assign26930_e37359_d_n10, assign26930_e37359_d_n11, assign26930_e37359_d_n12, assign26930_e37359_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign26930_e37357: f64 = (locals.var_ac31__blk812 * locals.var_ac31__blk812);
        (assign26930_e37357, ((locals.var_ac31__blk812_dn0 * locals.var_ac31__blk812) + (locals.var_ac31__blk812 * locals.var_ac31__blk812_dn0)), ((locals.var_ac31__blk812_dn2 * locals.var_ac31__blk812) + (locals.var_ac31__blk812 * locals.var_ac31__blk812_dn2)), ((locals.var_ac31__blk812_dn6 * locals.var_ac31__blk812) + (locals.var_ac31__blk812 * locals.var_ac31__blk812_dn6)), ((locals.var_ac31__blk812_dn7 * locals.var_ac31__blk812) + (locals.var_ac31__blk812 * locals.var_ac31__blk812_dn7)), ((locals.var_ac31__blk812_dn10 * locals.var_ac31__blk812) + (locals.var_ac31__blk812 * locals.var_ac31__blk812_dn10)), ((locals.var_ac31__blk812_dn11 * locals.var_ac31__blk812) + (locals.var_ac31__blk812 * locals.var_ac31__blk812_dn11)), ((locals.var_ac31__blk812_dn12 * locals.var_ac31__blk812) + (locals.var_ac31__blk812 * locals.var_ac31__blk812_dn12)), ((locals.var_ac31__blk812_dn17 * locals.var_ac31__blk812) + (locals.var_ac31__blk812 * locals.var_ac31__blk812_dn17)),)
    } else {
        (locals.var_ac3__blk813, locals.var_ac3__blk813_dn0, locals.var_ac3__blk813_dn2, locals.var_ac3__blk813_dn6, locals.var_ac3__blk813_dn7, locals.var_ac3__blk813_dn10, locals.var_ac3__blk813_dn11, locals.var_ac3__blk813_dn12, locals.var_ac3__blk813_dn17,)
    }
};
        locals.var_ac3__blk813 = assign26930_e37359;
        locals.var_ac3__blk813_dn0 = assign26930_e37359_d_n0;
        locals.var_ac3__blk813_dn2 = assign26930_e37359_d_n2;
        locals.var_ac3__blk813_dn6 = assign26930_e37359_d_n6;
        locals.var_ac3__blk813_dn7 = assign26930_e37359_d_n7;
        locals.var_ac3__blk813_dn10 = assign26930_e37359_d_n10;
        locals.var_ac3__blk813_dn11 = assign26930_e37359_d_n11;
        locals.var_ac3__blk813_dn12 = assign26930_e37359_d_n12;
        locals.var_ac3__blk813_dn17 = assign26930_e37359_d_n17;
        locals.var_ac3__blk813_rv = 0.0;

        let assign26940_e37363: f64 = (locals.var_ac3__blk813 * 1e-8);
        let assign26940_e37364: f64 = if locals.var_ac4__blk810 < assign26940_e37363 { 1.0 } else { 0.0 };
        locals.var_guard876 = assign26940_e37364;
        locals.var_guard876_rv = 0.0;

        let (assign26950_e37395, assign26950_e37395_d_n0, assign26950_e37395_d_n2, assign26950_e37395_d_n6, assign26950_e37395_d_n7, assign26950_e37395_d_n10, assign26950_e37395_d_n11, assign26950_e37395_d_n12, assign26950_e37395_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) && (locals.var_guard876 != 0.0)) {
        let assign26950_e37375: f64 = (-7.0);
        let assign26950_e37377: f64 = (assign26950_e37375 * 1.414213562373095);
        let assign26950_e37379: f64 = (assign26950_e37377 + locals.var_ac31__blk812);
        let assign26950_e37382: f64 = (0.5 * locals.var_ac4__blk810);
        let assign26950_e37384: f64 = (assign26950_e37382 / locals.var_ac31__blk812);
        let assign26950_e37385: f64 = (assign26950_e37379 + assign26950_e37384);
        let assign26950_e37388: f64 = (9.0 * locals.var_ty__blk782);
        let assign26950_e37391: f64 = (locals.var_tx__blk781 - 2.0);
        let assign26950_e37392: f64 = (assign26950_e37388 * assign26950_e37391);
        let assign26950_e37393: f64 = (assign26950_e37385 + assign26950_e37392);
        (assign26950_e37393, ((locals.var_ac31__blk812_dn0 + ((((0.5 * locals.var_ac4__blk810_dn0) * locals.var_ac31__blk812) - (assign26950_e37382 * locals.var_ac31__blk812_dn0)) / (locals.var_ac31__blk812 * locals.var_ac31__blk812))) + (((9.0 * locals.var_ty__blk782_dn0) * assign26950_e37391) + (assign26950_e37388 * locals.var_tx__blk781_dn0))), ((locals.var_ac31__blk812_dn2 + ((((0.5 * locals.var_ac4__blk810_dn2) * locals.var_ac31__blk812) - (assign26950_e37382 * locals.var_ac31__blk812_dn2)) / (locals.var_ac31__blk812 * locals.var_ac31__blk812))) + (((9.0 * locals.var_ty__blk782_dn2) * assign26950_e37391) + (assign26950_e37388 * locals.var_tx__blk781_dn2))), ((locals.var_ac31__blk812_dn6 + ((((0.5 * locals.var_ac4__blk810_dn6) * locals.var_ac31__blk812) - (assign26950_e37382 * locals.var_ac31__blk812_dn6)) / (locals.var_ac31__blk812 * locals.var_ac31__blk812))) + (((9.0 * locals.var_ty__blk782_dn6) * assign26950_e37391) + (assign26950_e37388 * locals.var_tx__blk781_dn6))), ((locals.var_ac31__blk812_dn7 + ((((0.5 * locals.var_ac4__blk810_dn7) * locals.var_ac31__blk812) - (assign26950_e37382 * locals.var_ac31__blk812_dn7)) / (locals.var_ac31__blk812 * locals.var_ac31__blk812))) + (((9.0 * locals.var_ty__blk782_dn7) * assign26950_e37391) + (assign26950_e37388 * locals.var_tx__blk781_dn7))), ((locals.var_ac31__blk812_dn10 + ((((0.5 * locals.var_ac4__blk810_dn10) * locals.var_ac31__blk812) - (assign26950_e37382 * locals.var_ac31__blk812_dn10)) / (locals.var_ac31__blk812 * locals.var_ac31__blk812))) + (((9.0 * locals.var_ty__blk782_dn10) * assign26950_e37391) + (assign26950_e37388 * locals.var_tx__blk781_dn10))), ((locals.var_ac31__blk812_dn11 + ((((0.5 * locals.var_ac4__blk810_dn11) * locals.var_ac31__blk812) - (assign26950_e37382 * locals.var_ac31__blk812_dn11)) / (locals.var_ac31__blk812 * locals.var_ac31__blk812))) + (((9.0 * locals.var_ty__blk782_dn11) * assign26950_e37391) + (assign26950_e37388 * locals.var_tx__blk781_dn11))), ((locals.var_ac31__blk812_dn12 + ((((0.5 * locals.var_ac4__blk810_dn12) * locals.var_ac31__blk812) - (assign26950_e37382 * locals.var_ac31__blk812_dn12)) / (locals.var_ac31__blk812 * locals.var_ac31__blk812))) + (((9.0 * locals.var_ty__blk782_dn12) * assign26950_e37391) + (assign26950_e37388 * locals.var_tx__blk781_dn12))), ((locals.var_ac31__blk812_dn17 + ((((0.5 * locals.var_ac4__blk810_dn17) * locals.var_ac31__blk812) - (assign26950_e37382 * locals.var_ac31__blk812_dn17)) / (locals.var_ac31__blk812 * locals.var_ac31__blk812))) + (((9.0 * locals.var_ty__blk782_dn17) * assign26950_e37391) + (assign26950_e37388 * locals.var_tx__blk781_dn17))),)
    } else {
        (locals.var_ac1__blk815, locals.var_ac1__blk815_dn0, locals.var_ac1__blk815_dn2, locals.var_ac1__blk815_dn6, locals.var_ac1__blk815_dn7, locals.var_ac1__blk815_dn10, locals.var_ac1__blk815_dn11, locals.var_ac1__blk815_dn12, locals.var_ac1__blk815_dn17,)
    }
};
        locals.var_ac1__blk815 = assign26950_e37395;
        locals.var_ac1__blk815_dn0 = assign26950_e37395_d_n0;
        locals.var_ac1__blk815_dn2 = assign26950_e37395_d_n2;
        locals.var_ac1__blk815_dn6 = assign26950_e37395_d_n6;
        locals.var_ac1__blk815_dn7 = assign26950_e37395_d_n7;
        locals.var_ac1__blk815_dn10 = assign26950_e37395_d_n10;
        locals.var_ac1__blk815_dn11 = assign26950_e37395_d_n11;
        locals.var_ac1__blk815_dn12 = assign26950_e37395_d_n12;
        locals.var_ac1__blk815_dn17 = assign26950_e37395_d_n17;
        locals.var_ac1__blk815_rv = 0.0;

        let (assign26960_e37411, assign26960_e37411_d_n0, assign26960_e37411_d_n2, assign26960_e37411_d_n6, assign26960_e37411_d_n7, assign26960_e37411_d_n10, assign26960_e37411_d_n11, assign26960_e37411_d_n12, assign26960_e37411_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) && (locals.var_guard876 == 0.0)) {
        let assign26960_e37408: f64 = (locals.var_ac4__blk810 + locals.var_ac3__blk813);
        let assign26960_e37409: f64 = (assign26960_e37408).sqrt();
        (assign26960_e37409, ((locals.var_ac4__blk810_dn0 + locals.var_ac3__blk813_dn0) / (2.0 * assign26960_e37409)), ((locals.var_ac4__blk810_dn2 + locals.var_ac3__blk813_dn2) / (2.0 * assign26960_e37409)), ((locals.var_ac4__blk810_dn6 + locals.var_ac3__blk813_dn6) / (2.0 * assign26960_e37409)), ((locals.var_ac4__blk810_dn7 + locals.var_ac3__blk813_dn7) / (2.0 * assign26960_e37409)), ((locals.var_ac4__blk810_dn10 + locals.var_ac3__blk813_dn10) / (2.0 * assign26960_e37409)), ((locals.var_ac4__blk810_dn11 + locals.var_ac3__blk813_dn11) / (2.0 * assign26960_e37409)), ((locals.var_ac4__blk810_dn12 + locals.var_ac3__blk813_dn12) / (2.0 * assign26960_e37409)), ((locals.var_ac4__blk810_dn17 + locals.var_ac3__blk813_dn17) / (2.0 * assign26960_e37409)),)
    } else {
        (locals.var_ac2__blk814, locals.var_ac2__blk814_dn0, locals.var_ac2__blk814_dn2, locals.var_ac2__blk814_dn6, locals.var_ac2__blk814_dn7, locals.var_ac2__blk814_dn10, locals.var_ac2__blk814_dn11, locals.var_ac2__blk814_dn12, locals.var_ac2__blk814_dn17,)
    }
};
        locals.var_ac2__blk814 = assign26960_e37411;
        locals.var_ac2__blk814_dn0 = assign26960_e37411_d_n0;
        locals.var_ac2__blk814_dn2 = assign26960_e37411_d_n2;
        locals.var_ac2__blk814_dn6 = assign26960_e37411_d_n6;
        locals.var_ac2__blk814_dn7 = assign26960_e37411_d_n7;
        locals.var_ac2__blk814_dn10 = assign26960_e37411_d_n10;
        locals.var_ac2__blk814_dn11 = assign26960_e37411_d_n11;
        locals.var_ac2__blk814_dn12 = assign26960_e37411_d_n12;
        locals.var_ac2__blk814_dn17 = assign26960_e37411_d_n17;
        locals.var_ac2__blk814_rv = 0.0;

        let (assign26970_e37437, assign26970_e37437_d_n0, assign26970_e37437_d_n2, assign26970_e37437_d_n6, assign26970_e37437_d_n7, assign26970_e37437_d_n10, assign26970_e37437_d_n11, assign26970_e37437_d_n12, assign26970_e37437_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) && (locals.var_guard876 == 0.0)) {
        let assign26970_e37423: f64 = (-7.0);
        let assign26970_e37425: f64 = (assign26970_e37423 * 1.414213562373095);
        let assign26970_e37427: f64 = (assign26970_e37425 + locals.var_ac2__blk814);
        let assign26970_e37430: f64 = (9.0 * locals.var_ty__blk782);
        let assign26970_e37433: f64 = (locals.var_tx__blk781 - 2.0);
        let assign26970_e37434: f64 = (assign26970_e37430 * assign26970_e37433);
        let assign26970_e37435: f64 = (assign26970_e37427 + assign26970_e37434);
        (assign26970_e37435, (locals.var_ac2__blk814_dn0 + (((9.0 * locals.var_ty__blk782_dn0) * assign26970_e37433) + (assign26970_e37430 * locals.var_tx__blk781_dn0))), (locals.var_ac2__blk814_dn2 + (((9.0 * locals.var_ty__blk782_dn2) * assign26970_e37433) + (assign26970_e37430 * locals.var_tx__blk781_dn2))), (locals.var_ac2__blk814_dn6 + (((9.0 * locals.var_ty__blk782_dn6) * assign26970_e37433) + (assign26970_e37430 * locals.var_tx__blk781_dn6))), (locals.var_ac2__blk814_dn7 + (((9.0 * locals.var_ty__blk782_dn7) * assign26970_e37433) + (assign26970_e37430 * locals.var_tx__blk781_dn7))), (locals.var_ac2__blk814_dn10 + (((9.0 * locals.var_ty__blk782_dn10) * assign26970_e37433) + (assign26970_e37430 * locals.var_tx__blk781_dn10))), (locals.var_ac2__blk814_dn11 + (((9.0 * locals.var_ty__blk782_dn11) * assign26970_e37433) + (assign26970_e37430 * locals.var_tx__blk781_dn11))), (locals.var_ac2__blk814_dn12 + (((9.0 * locals.var_ty__blk782_dn12) * assign26970_e37433) + (assign26970_e37430 * locals.var_tx__blk781_dn12))), (locals.var_ac2__blk814_dn17 + (((9.0 * locals.var_ty__blk782_dn17) * assign26970_e37433) + (assign26970_e37430 * locals.var_tx__blk781_dn17))),)
    } else {
        (locals.var_ac1__blk815, locals.var_ac1__blk815_dn0, locals.var_ac1__blk815_dn2, locals.var_ac1__blk815_dn6, locals.var_ac1__blk815_dn7, locals.var_ac1__blk815_dn10, locals.var_ac1__blk815_dn11, locals.var_ac1__blk815_dn12, locals.var_ac1__blk815_dn17,)
    }
};
        locals.var_ac1__blk815 = assign26970_e37437;
        locals.var_ac1__blk815_dn0 = assign26970_e37437_d_n0;
        locals.var_ac1__blk815_dn2 = assign26970_e37437_d_n2;
        locals.var_ac1__blk815_dn6 = assign26970_e37437_d_n6;
        locals.var_ac1__blk815_dn7 = assign26970_e37437_d_n7;
        locals.var_ac1__blk815_dn10 = assign26970_e37437_d_n10;
        locals.var_ac1__blk815_dn11 = assign26970_e37437_d_n11;
        locals.var_ac1__blk815_dn12 = assign26970_e37437_d_n12;
        locals.var_ac1__blk815_dn17 = assign26970_e37437_d_n17;
        locals.var_ac1__blk815_rv = 0.0;

        let (assign26980_e37449, assign26980_e37449_d_n0, assign26980_e37449_d_n2, assign26980_e37449_d_n6, assign26980_e37449_d_n7, assign26980_e37449_d_n10, assign26980_e37449_d_n11, assign26980_e37449_d_n12, assign26980_e37449_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign26980_e37447: f64 = (locals.var_ac1__blk815).powf(0.3333333333333333);
        (assign26980_e37447, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk815).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk815_dn0)) } } else { (assign26980_e37447 * (0.3333333333333333 * (locals.var_ac1__blk815_dn0 / locals.var_ac1__blk815))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk815).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk815_dn2)) } } else { (assign26980_e37447 * (0.3333333333333333 * (locals.var_ac1__blk815_dn2 / locals.var_ac1__blk815))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk815).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk815_dn6)) } } else { (assign26980_e37447 * (0.3333333333333333 * (locals.var_ac1__blk815_dn6 / locals.var_ac1__blk815))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk815).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk815_dn7)) } } else { (assign26980_e37447 * (0.3333333333333333 * (locals.var_ac1__blk815_dn7 / locals.var_ac1__blk815))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk815).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk815_dn10)) } } else { (assign26980_e37447 * (0.3333333333333333 * (locals.var_ac1__blk815_dn10 / locals.var_ac1__blk815))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk815).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk815_dn11)) } } else { (assign26980_e37447 * (0.3333333333333333 * (locals.var_ac1__blk815_dn11 / locals.var_ac1__blk815))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk815).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk815_dn12)) } } else { (assign26980_e37447 * (0.3333333333333333 * (locals.var_ac1__blk815_dn12 / locals.var_ac1__blk815))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk815).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk815_dn17)) } } else { (assign26980_e37447 * (0.3333333333333333 * (locals.var_ac1__blk815_dn17 / locals.var_ac1__blk815))) },)
    } else {
        (locals.var_acd__blk816, locals.var_acd__blk816_dn0, locals.var_acd__blk816_dn2, locals.var_acd__blk816_dn6, locals.var_acd__blk816_dn7, locals.var_acd__blk816_dn10, locals.var_acd__blk816_dn11, locals.var_acd__blk816_dn12, locals.var_acd__blk816_dn17,)
    }
};
        locals.var_acd__blk816 = assign26980_e37449;
        locals.var_acd__blk816_dn0 = assign26980_e37449_d_n0;
        locals.var_acd__blk816_dn2 = assign26980_e37449_d_n2;
        locals.var_acd__blk816_dn6 = assign26980_e37449_d_n6;
        locals.var_acd__blk816_dn7 = assign26980_e37449_d_n7;
        locals.var_acd__blk816_dn10 = assign26980_e37449_d_n10;
        locals.var_acd__blk816_dn11 = assign26980_e37449_d_n11;
        locals.var_acd__blk816_dn12 = assign26980_e37449_d_n12;
        locals.var_acd__blk816_dn17 = assign26980_e37449_d_n17;
        locals.var_acd__blk816_rv = 0.0;

        let (assign26990_e37476, assign26990_e37476_d_n0, assign26990_e37476_d_n2, assign26990_e37476_d_n6, assign26990_e37476_d_n7, assign26990_e37476_d_n10, assign26990_e37476_d_n11, assign26990_e37476_d_n12, assign26990_e37476_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign26990_e37458: f64 = (-4.0);
        let assign26990_e37460: f64 = (assign26990_e37458 * 1.414213562373095);
        let assign26990_e37463: f64 = (12.0 * locals.var_ty__blk782);
        let assign26990_e37464: f64 = (assign26990_e37460 - assign26990_e37463);
        let assign26990_e37467: f64 = (2.0 * locals.var_acd__blk816);
        let assign26990_e37468: f64 = (assign26990_e37464 + assign26990_e37467);
        let assign26990_e37471: f64 = (1.414213562373095 * locals.var_acd__blk816);
        let assign26990_e37473: f64 = (assign26990_e37471 * locals.var_acd__blk816);
        let assign26990_e37474: f64 = (assign26990_e37468 + assign26990_e37473);
        (assign26990_e37474, (((-(12.0 * locals.var_ty__blk782_dn0)) + (2.0 * locals.var_acd__blk816_dn0)) + (((1.414213562373095 * locals.var_acd__blk816_dn0) * locals.var_acd__blk816) + (assign26990_e37471 * locals.var_acd__blk816_dn0))), (((-(12.0 * locals.var_ty__blk782_dn2)) + (2.0 * locals.var_acd__blk816_dn2)) + (((1.414213562373095 * locals.var_acd__blk816_dn2) * locals.var_acd__blk816) + (assign26990_e37471 * locals.var_acd__blk816_dn2))), (((-(12.0 * locals.var_ty__blk782_dn6)) + (2.0 * locals.var_acd__blk816_dn6)) + (((1.414213562373095 * locals.var_acd__blk816_dn6) * locals.var_acd__blk816) + (assign26990_e37471 * locals.var_acd__blk816_dn6))), (((-(12.0 * locals.var_ty__blk782_dn7)) + (2.0 * locals.var_acd__blk816_dn7)) + (((1.414213562373095 * locals.var_acd__blk816_dn7) * locals.var_acd__blk816) + (assign26990_e37471 * locals.var_acd__blk816_dn7))), (((-(12.0 * locals.var_ty__blk782_dn10)) + (2.0 * locals.var_acd__blk816_dn10)) + (((1.414213562373095 * locals.var_acd__blk816_dn10) * locals.var_acd__blk816) + (assign26990_e37471 * locals.var_acd__blk816_dn10))), (((-(12.0 * locals.var_ty__blk782_dn11)) + (2.0 * locals.var_acd__blk816_dn11)) + (((1.414213562373095 * locals.var_acd__blk816_dn11) * locals.var_acd__blk816) + (assign26990_e37471 * locals.var_acd__blk816_dn11))), (((-(12.0 * locals.var_ty__blk782_dn12)) + (2.0 * locals.var_acd__blk816_dn12)) + (((1.414213562373095 * locals.var_acd__blk816_dn12) * locals.var_acd__blk816) + (assign26990_e37471 * locals.var_acd__blk816_dn12))), (((-(12.0 * locals.var_ty__blk782_dn17)) + (2.0 * locals.var_acd__blk816_dn17)) + (((1.414213562373095 * locals.var_acd__blk816_dn17) * locals.var_acd__blk816) + (assign26990_e37471 * locals.var_acd__blk816_dn17))),)
    } else {
        (locals.var_acn__blk817, locals.var_acn__blk817_dn0, locals.var_acn__blk817_dn2, locals.var_acn__blk817_dn6, locals.var_acn__blk817_dn7, locals.var_acn__blk817_dn10, locals.var_acn__blk817_dn11, locals.var_acn__blk817_dn12, locals.var_acn__blk817_dn17,)
    }
};
        locals.var_acn__blk817 = assign26990_e37476;
        locals.var_acn__blk817_dn0 = assign26990_e37476_d_n0;
        locals.var_acn__blk817_dn2 = assign26990_e37476_d_n2;
        locals.var_acn__blk817_dn6 = assign26990_e37476_d_n6;
        locals.var_acn__blk817_dn7 = assign26990_e37476_d_n7;
        locals.var_acn__blk817_dn10 = assign26990_e37476_d_n10;
        locals.var_acn__blk817_dn11 = assign26990_e37476_d_n11;
        locals.var_acn__blk817_dn12 = assign26990_e37476_d_n12;
        locals.var_acn__blk817_dn17 = assign26990_e37476_d_n17;
        locals.var_acn__blk817_rv = 0.0;

        let (assign27000_e37488, assign27000_e37488_d_n0, assign27000_e37488_d_n2, assign27000_e37488_d_n6, assign27000_e37488_d_n7, assign27000_e37488_d_n10, assign27000_e37488_d_n11, assign27000_e37488_d_n12, assign27000_e37488_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27000_e37486: f64 = (locals.var_acn__blk817 / locals.var_acd__blk816);
        (assign27000_e37486, (((locals.var_acn__blk817_dn0 * locals.var_acd__blk816) - (locals.var_acn__blk817 * locals.var_acd__blk816_dn0)) / (locals.var_acd__blk816 * locals.var_acd__blk816)), (((locals.var_acn__blk817_dn2 * locals.var_acd__blk816) - (locals.var_acn__blk817 * locals.var_acd__blk816_dn2)) / (locals.var_acd__blk816 * locals.var_acd__blk816)), (((locals.var_acn__blk817_dn6 * locals.var_acd__blk816) - (locals.var_acn__blk817 * locals.var_acd__blk816_dn6)) / (locals.var_acd__blk816 * locals.var_acd__blk816)), (((locals.var_acn__blk817_dn7 * locals.var_acd__blk816) - (locals.var_acn__blk817 * locals.var_acd__blk816_dn7)) / (locals.var_acd__blk816 * locals.var_acd__blk816)), (((locals.var_acn__blk817_dn10 * locals.var_acd__blk816) - (locals.var_acn__blk817 * locals.var_acd__blk816_dn10)) / (locals.var_acd__blk816 * locals.var_acd__blk816)), (((locals.var_acn__blk817_dn11 * locals.var_acd__blk816) - (locals.var_acn__blk817 * locals.var_acd__blk816_dn11)) / (locals.var_acd__blk816 * locals.var_acd__blk816)), (((locals.var_acn__blk817_dn12 * locals.var_acd__blk816) - (locals.var_acn__blk817 * locals.var_acd__blk816_dn12)) / (locals.var_acd__blk816 * locals.var_acd__blk816)), (((locals.var_acn__blk817_dn17 * locals.var_acd__blk816) - (locals.var_acn__blk817 * locals.var_acd__blk816_dn17)) / (locals.var_acd__blk816 * locals.var_acd__blk816)),)
    } else {
        (locals.var_chi__blk818, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    }
};
        locals.var_chi__blk818 = assign27000_e37488;
        locals.var_chi__blk818_dn0 = assign27000_e37488_d_n0;
        locals.var_chi__blk818_dn2 = assign27000_e37488_d_n2;
        locals.var_chi__blk818_dn6 = assign27000_e37488_d_n6;
        locals.var_chi__blk818_dn7 = assign27000_e37488_d_n7;
        locals.var_chi__blk818_dn10 = assign27000_e37488_d_n10;
        locals.var_chi__blk818_dn11 = assign27000_e37488_d_n11;
        locals.var_chi__blk818_dn12 = assign27000_e37488_d_n12;
        locals.var_chi__blk818_dn17 = assign27000_e37488_d_n17;
        locals.var_chi__blk818_rv = 0.0;

        let (assign27010_e37502, assign27010_e37502_d_n0, assign27010_e37502_d_n2, assign27010_e37502_d_n6, assign27010_e37502_d_n7, assign27010_e37502_d_n10, assign27010_e37502_d_n11, assign27010_e37502_d_n12, assign27010_e37502_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27010_e37498: f64 = (locals.var_chi__blk818 * locals.var_beta_inv);
        let assign27010_e37500: f64 = (assign27010_e37498 - locals.var_vxbgmtcl);
        (assign27010_e37500, ((locals.var_chi__blk818_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk818_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk818_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk818_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn7), (((locals.var_chi__blk818_dn10 * locals.var_beta_inv) + (locals.var_chi__blk818 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk818_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk818_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk818_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_psa__blk819, locals.var_psa__blk819_dn0, locals.var_psa__blk819_dn2, locals.var_psa__blk819_dn6, locals.var_psa__blk819_dn7, locals.var_psa__blk819_dn10, locals.var_psa__blk819_dn11, locals.var_psa__blk819_dn12, locals.var_psa__blk819_dn17,)
    }
};
        locals.var_psa__blk819 = assign27010_e37502;
        locals.var_psa__blk819_dn0 = assign27010_e37502_d_n0;
        locals.var_psa__blk819_dn2 = assign27010_e37502_d_n2;
        locals.var_psa__blk819_dn6 = assign27010_e37502_d_n6;
        locals.var_psa__blk819_dn7 = assign27010_e37502_d_n7;
        locals.var_psa__blk819_dn10 = assign27010_e37502_d_n10;
        locals.var_psa__blk819_dn11 = assign27010_e37502_d_n11;
        locals.var_psa__blk819_dn12 = assign27010_e37502_d_n12;
        locals.var_psa__blk819_dn17 = assign27010_e37502_d_n17;
        locals.var_psa__blk819_rv = 0.0;

        let (assign27020_e37514, assign27020_e37514_d_n0, assign27020_e37514_d_n2, assign27020_e37514_d_n6, assign27020_e37514_d_n7, assign27020_e37514_d_n10, assign27020_e37514_d_n11, assign27020_e37514_d_n12, assign27020_e37514_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27020_e37512: f64 = (locals.var_psa__blk819 + locals.var_vxbgmtcl);
        (assign27020_e37512, (locals.var_psa__blk819_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_psa__blk819_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_psa__blk819_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_psa__blk819_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_psa__blk819_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_psa__blk819_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_psa__blk819_dn12 + locals.var_vxbgmtcl_dn12), (locals.var_psa__blk819_dn17 + locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_t1__blk775, locals.var_t1__blk775_dn0, locals.var_t1__blk775_dn2, locals.var_t1__blk775_dn6, locals.var_t1__blk775_dn7, locals.var_t1__blk775_dn10, locals.var_t1__blk775_dn11, locals.var_t1__blk775_dn12, locals.var_t1__blk775_dn17,)
    }
};
        locals.var_t1__blk775 = assign27020_e37514;
        locals.var_t1__blk775_dn0 = assign27020_e37514_d_n0;
        locals.var_t1__blk775_dn2 = assign27020_e37514_d_n2;
        locals.var_t1__blk775_dn6 = assign27020_e37514_d_n6;
        locals.var_t1__blk775_dn7 = assign27020_e37514_d_n7;
        locals.var_t1__blk775_dn10 = assign27020_e37514_d_n10;
        locals.var_t1__blk775_dn11 = assign27020_e37514_d_n11;
        locals.var_t1__blk775_dn12 = assign27020_e37514_d_n12;
        locals.var_t1__blk775_dn17 = assign27020_e37514_d_n17;
        locals.var_t1__blk775_rv = 0.0;

        let (assign27030_e37526, assign27030_e37526_d_n0, assign27030_e37526_d_n2, assign27030_e37526_d_n6, assign27030_e37526_d_n7, assign27030_e37526_d_n10, assign27030_e37526_d_n11, assign27030_e37526_d_n12, assign27030_e37526_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27030_e37524: f64 = (locals.var_t1__blk775 / locals.var_ps0_min__blk811);
        (assign27030_e37524, (((locals.var_t1__blk775_dn0 * locals.var_ps0_min__blk811) - (locals.var_t1__blk775 * locals.var_ps0_min__blk811_dn0)) / (locals.var_ps0_min__blk811 * locals.var_ps0_min__blk811)), (((locals.var_t1__blk775_dn2 * locals.var_ps0_min__blk811) - (locals.var_t1__blk775 * locals.var_ps0_min__blk811_dn2)) / (locals.var_ps0_min__blk811 * locals.var_ps0_min__blk811)), (((locals.var_t1__blk775_dn6 * locals.var_ps0_min__blk811) - (locals.var_t1__blk775 * locals.var_ps0_min__blk811_dn6)) / (locals.var_ps0_min__blk811 * locals.var_ps0_min__blk811)), (((locals.var_t1__blk775_dn7 * locals.var_ps0_min__blk811) - (locals.var_t1__blk775 * locals.var_ps0_min__blk811_dn7)) / (locals.var_ps0_min__blk811 * locals.var_ps0_min__blk811)), (((locals.var_t1__blk775_dn10 * locals.var_ps0_min__blk811) - (locals.var_t1__blk775 * locals.var_ps0_min__blk811_dn10)) / (locals.var_ps0_min__blk811 * locals.var_ps0_min__blk811)), (((locals.var_t1__blk775_dn11 * locals.var_ps0_min__blk811) - (locals.var_t1__blk775 * locals.var_ps0_min__blk811_dn11)) / (locals.var_ps0_min__blk811 * locals.var_ps0_min__blk811)), (((locals.var_t1__blk775_dn12 * locals.var_ps0_min__blk811) - (locals.var_t1__blk775 * locals.var_ps0_min__blk811_dn12)) / (locals.var_ps0_min__blk811 * locals.var_ps0_min__blk811)), (((locals.var_t1__blk775_dn17 * locals.var_ps0_min__blk811) - (locals.var_t1__blk775 * locals.var_ps0_min__blk811_dn17)) / (locals.var_ps0_min__blk811 * locals.var_ps0_min__blk811)),)
    } else {
        (locals.var_t2__blk776, locals.var_t2__blk776_dn0, locals.var_t2__blk776_dn2, locals.var_t2__blk776_dn6, locals.var_t2__blk776_dn7, locals.var_t2__blk776_dn10, locals.var_t2__blk776_dn11, locals.var_t2__blk776_dn12, locals.var_t2__blk776_dn17,)
    }
};
        locals.var_t2__blk776 = assign27030_e37526;
        locals.var_t2__blk776_dn0 = assign27030_e37526_d_n0;
        locals.var_t2__blk776_dn2 = assign27030_e37526_d_n2;
        locals.var_t2__blk776_dn6 = assign27030_e37526_d_n6;
        locals.var_t2__blk776_dn7 = assign27030_e37526_d_n7;
        locals.var_t2__blk776_dn10 = assign27030_e37526_d_n10;
        locals.var_t2__blk776_dn11 = assign27030_e37526_d_n11;
        locals.var_t2__blk776_dn12 = assign27030_e37526_d_n12;
        locals.var_t2__blk776_dn17 = assign27030_e37526_d_n17;
        locals.var_t2__blk776_rv = 0.0;

        let (assign27040_e37541, assign27040_e37541_d_n0, assign27040_e37541_d_n2, assign27040_e37541_d_n6, assign27040_e37541_d_n7, assign27040_e37541_d_n10, assign27040_e37541_d_n11, assign27040_e37541_d_n12, assign27040_e37541_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27040_e37537: f64 = (locals.var_t2__blk776 * locals.var_t2__blk776);
        let assign27040_e37538: f64 = (1.0 + assign27040_e37537);
        let assign27040_e37539: f64 = (assign27040_e37538).sqrt();
        (assign27040_e37539, (((locals.var_t2__blk776_dn0 * locals.var_t2__blk776) + (locals.var_t2__blk776 * locals.var_t2__blk776_dn0)) / (2.0 * assign27040_e37539)), (((locals.var_t2__blk776_dn2 * locals.var_t2__blk776) + (locals.var_t2__blk776 * locals.var_t2__blk776_dn2)) / (2.0 * assign27040_e37539)), (((locals.var_t2__blk776_dn6 * locals.var_t2__blk776) + (locals.var_t2__blk776 * locals.var_t2__blk776_dn6)) / (2.0 * assign27040_e37539)), (((locals.var_t2__blk776_dn7 * locals.var_t2__blk776) + (locals.var_t2__blk776 * locals.var_t2__blk776_dn7)) / (2.0 * assign27040_e37539)), (((locals.var_t2__blk776_dn10 * locals.var_t2__blk776) + (locals.var_t2__blk776 * locals.var_t2__blk776_dn10)) / (2.0 * assign27040_e37539)), (((locals.var_t2__blk776_dn11 * locals.var_t2__blk776) + (locals.var_t2__blk776 * locals.var_t2__blk776_dn11)) / (2.0 * assign27040_e37539)), (((locals.var_t2__blk776_dn12 * locals.var_t2__blk776) + (locals.var_t2__blk776 * locals.var_t2__blk776_dn12)) / (2.0 * assign27040_e37539)), (((locals.var_t2__blk776_dn17 * locals.var_t2__blk776) + (locals.var_t2__blk776 * locals.var_t2__blk776_dn17)) / (2.0 * assign27040_e37539)),)
    } else {
        (locals.var_t3__blk777, locals.var_t3__blk777_dn0, locals.var_t3__blk777_dn2, locals.var_t3__blk777_dn6, locals.var_t3__blk777_dn7, locals.var_t3__blk777_dn10, locals.var_t3__blk777_dn11, locals.var_t3__blk777_dn12, locals.var_t3__blk777_dn17,)
    }
};
        locals.var_t3__blk777 = assign27040_e37541;
        locals.var_t3__blk777_dn0 = assign27040_e37541_d_n0;
        locals.var_t3__blk777_dn2 = assign27040_e37541_d_n2;
        locals.var_t3__blk777_dn6 = assign27040_e37541_d_n6;
        locals.var_t3__blk777_dn7 = assign27040_e37541_d_n7;
        locals.var_t3__blk777_dn10 = assign27040_e37541_d_n10;
        locals.var_t3__blk777_dn11 = assign27040_e37541_d_n11;
        locals.var_t3__blk777_dn12 = assign27040_e37541_d_n12;
        locals.var_t3__blk777_dn17 = assign27040_e37541_d_n17;
        locals.var_t3__blk777_rv = 0.0;

        let (assign27050_e37555, assign27050_e37555_d_n0, assign27050_e37555_d_n2, assign27050_e37555_d_n6, assign27050_e37555_d_n7, assign27050_e37555_d_n10, assign27050_e37555_d_n11, assign27050_e37555_d_n12, assign27050_e37555_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27050_e37551: f64 = (locals.var_t1__blk775 / locals.var_t3__blk777);
        let assign27050_e37553: f64 = (assign27050_e37551 - locals.var_vxbgmtcl);
        (assign27050_e37553, ((((locals.var_t1__blk775_dn0 * locals.var_t3__blk777) - (locals.var_t1__blk775 * locals.var_t3__blk777_dn0)) / (locals.var_t3__blk777 * locals.var_t3__blk777)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1__blk775_dn2 * locals.var_t3__blk777) - (locals.var_t1__blk775 * locals.var_t3__blk777_dn2)) / (locals.var_t3__blk777 * locals.var_t3__blk777)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1__blk775_dn6 * locals.var_t3__blk777) - (locals.var_t1__blk775 * locals.var_t3__blk777_dn6)) / (locals.var_t3__blk777 * locals.var_t3__blk777)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1__blk775_dn7 * locals.var_t3__blk777) - (locals.var_t1__blk775 * locals.var_t3__blk777_dn7)) / (locals.var_t3__blk777 * locals.var_t3__blk777)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1__blk775_dn10 * locals.var_t3__blk777) - (locals.var_t1__blk775 * locals.var_t3__blk777_dn10)) / (locals.var_t3__blk777 * locals.var_t3__blk777)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1__blk775_dn11 * locals.var_t3__blk777) - (locals.var_t1__blk775 * locals.var_t3__blk777_dn11)) / (locals.var_t3__blk777 * locals.var_t3__blk777)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1__blk775_dn12 * locals.var_t3__blk777) - (locals.var_t1__blk775 * locals.var_t3__blk777_dn12)) / (locals.var_t3__blk777 * locals.var_t3__blk777)) - locals.var_vxbgmtcl_dn12), ((((locals.var_t1__blk775_dn17 * locals.var_t3__blk777) - (locals.var_t1__blk775 * locals.var_t3__blk777_dn17)) / (locals.var_t3__blk777 * locals.var_t3__blk777)) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
        locals.var_ps0ld = assign27050_e37555;
        locals.var_ps0ld_dn0 = assign27050_e37555_d_n0;
        locals.var_ps0ld_dn2 = assign27050_e37555_d_n2;
        locals.var_ps0ld_dn6 = assign27050_e37555_d_n6;
        locals.var_ps0ld_dn7 = assign27050_e37555_d_n7;
        locals.var_ps0ld_dn10 = assign27050_e37555_d_n10;
        locals.var_ps0ld_dn11 = assign27050_e37555_d_n11;
        locals.var_ps0ld_dn12 = assign27050_e37555_d_n12;
        locals.var_ps0ld_dn17 = assign27050_e37555_d_n17;
        locals.var_ps0ld_rv = 0.0;

        let (assign27060_e37567, assign27060_e37567_d_n0, assign27060_e37567_d_n2, assign27060_e37567_d_n6, assign27060_e37567_d_n7, assign27060_e37567_d_n10, assign27060_e37567_d_n11, assign27060_e37567_d_n12, assign27060_e37567_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27060_e37565: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign27060_e37565, (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn10 - locals.var_ps0ld_dn10), (locals.var_vgpld_dn11 - locals.var_ps0ld_dn11), (locals.var_vgpld_dn12 - locals.var_ps0ld_dn12), (locals.var_vgpld_dn17 - locals.var_ps0ld_dn17),)
    } else {
        (locals.var_t2__blk776, locals.var_t2__blk776_dn0, locals.var_t2__blk776_dn2, locals.var_t2__blk776_dn6, locals.var_t2__blk776_dn7, locals.var_t2__blk776_dn10, locals.var_t2__blk776_dn11, locals.var_t2__blk776_dn12, locals.var_t2__blk776_dn17,)
    }
};
        locals.var_t2__blk776 = assign27060_e37567;
        locals.var_t2__blk776_dn0 = assign27060_e37567_d_n0;
        locals.var_t2__blk776_dn2 = assign27060_e37567_d_n2;
        locals.var_t2__blk776_dn6 = assign27060_e37567_d_n6;
        locals.var_t2__blk776_dn7 = assign27060_e37567_d_n7;
        locals.var_t2__blk776_dn10 = assign27060_e37567_d_n10;
        locals.var_t2__blk776_dn11 = assign27060_e37567_d_n11;
        locals.var_t2__blk776_dn12 = assign27060_e37567_d_n12;
        locals.var_t2__blk776_dn17 = assign27060_e37567_d_n17;
        locals.var_t2__blk776_rv = 0.0;

        let (assign27070_e37579, assign27070_e37579_d_n0, assign27070_e37579_d_n2, assign27070_e37579_d_n6, assign27070_e37579_d_n7, assign27070_e37579_d_n10, assign27070_e37579_d_n11, assign27070_e37579_d_n12, assign27070_e37579_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27070_e37577: f64 = (locals.var_cox0 * locals.var_t2__blk776);
        (assign27070_e37577, (locals.var_cox0 * locals.var_t2__blk776_dn0), (locals.var_cox0 * locals.var_t2__blk776_dn2), (locals.var_cox0 * locals.var_t2__blk776_dn6), (locals.var_cox0 * locals.var_t2__blk776_dn7), (locals.var_cox0 * locals.var_t2__blk776_dn10), (locals.var_cox0 * locals.var_t2__blk776_dn11), (locals.var_cox0 * locals.var_t2__blk776_dn12), (locals.var_cox0 * locals.var_t2__blk776_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign27070_e37579;
        locals.var_qsuld_dn0 = assign27070_e37579_d_n0;
        locals.var_qsuld_dn2 = assign27070_e37579_d_n2;
        locals.var_qsuld_dn6 = assign27070_e37579_d_n6;
        locals.var_qsuld_dn7 = assign27070_e37579_d_n7;
        locals.var_qsuld_dn10 = assign27070_e37579_d_n10;
        locals.var_qsuld_dn11 = assign27070_e37579_d_n11;
        locals.var_qsuld_dn12 = assign27070_e37579_d_n12;
        locals.var_qsuld_dn17 = assign27070_e37579_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign27080_e37589, assign27080_e37589_d_n0, assign27080_e37589_d_n2, assign27080_e37589_d_n6, assign27080_e37589_d_n7, assign27080_e37589_d_n10, assign27080_e37589_d_n11, assign27080_e37589_d_n12, assign27080_e37589_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign27080_e37589;
        locals.var_qbuld_dn0 = assign27080_e37589_d_n0;
        locals.var_qbuld_dn2 = assign27080_e37589_d_n2;
        locals.var_qbuld_dn6 = assign27080_e37589_d_n6;
        locals.var_qbuld_dn7 = assign27080_e37589_d_n7;
        locals.var_qbuld_dn10 = assign27080_e37589_d_n10;
        locals.var_qbuld_dn11 = assign27080_e37589_d_n11;
        locals.var_qbuld_dn12 = assign27080_e37589_d_n12;
        locals.var_qbuld_dn17 = assign27080_e37589_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign27100_e37611, assign27100_e37611_d_n0, assign27100_e37611_d_n2, assign27100_e37611_d_n6, assign27100_e37611_d_n7, assign27100_e37611_d_n10, assign27100_e37611_d_n11, assign27100_e37611_d_n12, assign27100_e37611_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        (3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi__blk818, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    }
};
        locals.var_chi__blk818 = assign27100_e37611;
        locals.var_chi__blk818_dn0 = assign27100_e37611_d_n0;
        locals.var_chi__blk818_dn2 = assign27100_e37611_d_n2;
        locals.var_chi__blk818_dn6 = assign27100_e37611_d_n6;
        locals.var_chi__blk818_dn7 = assign27100_e37611_d_n7;
        locals.var_chi__blk818_dn10 = assign27100_e37611_d_n10;
        locals.var_chi__blk818_dn11 = assign27100_e37611_d_n11;
        locals.var_chi__blk818_dn12 = assign27100_e37611_d_n12;
        locals.var_chi__blk818_dn17 = assign27100_e37611_d_n17;
        locals.var_chi__blk818_rv = 0.0;

        let (assign27110_e37626, assign27110_e37626_d_n0, assign27110_e37626_d_n2, assign27110_e37626_d_n6, assign27110_e37626_d_n7, assign27110_e37626_d_n10, assign27110_e37626_d_n11, assign27110_e37626_d_n12, assign27110_e37626_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27110_e37622: f64 = (locals.var_chi__blk818 / locals.var_beta);
        let assign27110_e37624: f64 = (assign27110_e37622 - locals.var_vxbgmtcl);
        (assign27110_e37624, ((locals.var_chi__blk818_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk818_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk818_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk818_dn7 / locals.var_beta) - locals.var_vxbgmtcl_dn7), ((((locals.var_chi__blk818_dn10 * locals.var_beta) - (locals.var_chi__blk818 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk818_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk818_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk818_dn17 / locals.var_beta) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0_inia__blk821, locals.var_ps0_inia__blk821_dn0, locals.var_ps0_inia__blk821_dn2, locals.var_ps0_inia__blk821_dn6, locals.var_ps0_inia__blk821_dn7, locals.var_ps0_inia__blk821_dn10, locals.var_ps0_inia__blk821_dn11, locals.var_ps0_inia__blk821_dn12, locals.var_ps0_inia__blk821_dn17,)
    }
};
        locals.var_ps0_inia__blk821 = assign27110_e37626;
        locals.var_ps0_inia__blk821_dn0 = assign27110_e37626_d_n0;
        locals.var_ps0_inia__blk821_dn2 = assign27110_e37626_d_n2;
        locals.var_ps0_inia__blk821_dn6 = assign27110_e37626_d_n6;
        locals.var_ps0_inia__blk821_dn7 = assign27110_e37626_d_n7;
        locals.var_ps0_inia__blk821_dn10 = assign27110_e37626_d_n10;
        locals.var_ps0_inia__blk821_dn11 = assign27110_e37626_d_n11;
        locals.var_ps0_inia__blk821_dn12 = assign27110_e37626_d_n12;
        locals.var_ps0_inia__blk821_dn17 = assign27110_e37626_d_n17;
        locals.var_ps0_inia__blk821_rv = 0.0;

        let (assign27120_e37639, assign27120_e37639_d_n0, assign27120_e37639_d_n2, assign27120_e37639_d_n6, assign27120_e37639_d_n7, assign27120_e37639_d_n10, assign27120_e37639_d_n11, assign27120_e37639_d_n12, assign27120_e37639_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27120_e37636: f64 = (-locals.var_chi__blk818);
        let assign27120_e37637: f64 = (assign27120_e37636).exp();
        (assign27120_e37637, (assign27120_e37637 * (-locals.var_chi__blk818_dn0)), (assign27120_e37637 * (-locals.var_chi__blk818_dn2)), (assign27120_e37637 * (-locals.var_chi__blk818_dn6)), (assign27120_e37637 * (-locals.var_chi__blk818_dn7)), (assign27120_e37637 * (-locals.var_chi__blk818_dn10)), (assign27120_e37637 * (-locals.var_chi__blk818_dn11)), (assign27120_e37637 * (-locals.var_chi__blk818_dn12)), (assign27120_e37637 * (-locals.var_chi__blk818_dn17)),)
    } else {
        (locals.var_ty__blk782, locals.var_ty__blk782_dn0, locals.var_ty__blk782_dn2, locals.var_ty__blk782_dn6, locals.var_ty__blk782_dn7, locals.var_ty__blk782_dn10, locals.var_ty__blk782_dn11, locals.var_ty__blk782_dn12, locals.var_ty__blk782_dn17,)
    }
};
        locals.var_ty__blk782 = assign27120_e37639;
        locals.var_ty__blk782_dn0 = assign27120_e37639_d_n0;
        locals.var_ty__blk782_dn2 = assign27120_e37639_d_n2;
        locals.var_ty__blk782_dn6 = assign27120_e37639_d_n6;
        locals.var_ty__blk782_dn7 = assign27120_e37639_d_n7;
        locals.var_ty__blk782_dn10 = assign27120_e37639_d_n10;
        locals.var_ty__blk782_dn11 = assign27120_e37639_d_n11;
        locals.var_ty__blk782_dn12 = assign27120_e37639_d_n12;
        locals.var_ty__blk782_dn17 = assign27120_e37639_d_n17;
        locals.var_ty__blk782_rv = 0.0;

        let (assign27130_e37666, assign27130_e37666_d_n0, assign27130_e37666_d_n2, assign27130_e37666_d_n6, assign27130_e37666_d_n7, assign27130_e37666_d_n10, assign27130_e37666_d_n11, assign27130_e37666_d_n12, assign27130_e37666_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27130_e37653: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign27130_e37654: f64 = (locals.var_beta * assign27130_e37653);
        let assign27130_e37656: f64 = (assign27130_e37654 - 1.0);
        let assign27130_e37658: f64 = (assign27130_e37656 + locals.var_ty__blk782);
        let assign27130_e37659: f64 = (4.0 * assign27130_e37658);
        let assign27130_e37662: f64 = (locals.var_fac1p2__blk805 * locals.var_beta2);
        let assign27130_e37663: f64 = (assign27130_e37659 / assign27130_e37662);
        let assign27130_e37664: f64 = (1.0 + assign27130_e37663);
        (assign27130_e37664, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + locals.var_ty__blk782_dn0)) * assign27130_e37662) - (assign27130_e37659 * (locals.var_fac1p2__blk805_dn0 * locals.var_beta2))) / (assign27130_e37662 * assign27130_e37662)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + locals.var_ty__blk782_dn2)) * assign27130_e37662) - (assign27130_e37659 * (locals.var_fac1p2__blk805_dn2 * locals.var_beta2))) / (assign27130_e37662 * assign27130_e37662)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) + locals.var_ty__blk782_dn6)) * assign27130_e37662) - (assign27130_e37659 * (locals.var_fac1p2__blk805_dn6 * locals.var_beta2))) / (assign27130_e37662 * assign27130_e37662)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) + locals.var_ty__blk782_dn7)) * assign27130_e37662) - (assign27130_e37659 * (locals.var_fac1p2__blk805_dn7 * locals.var_beta2))) / (assign27130_e37662 * assign27130_e37662)), ((((4.0 * (((locals.var_beta_dn10 * assign27130_e37653) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))) + locals.var_ty__blk782_dn10)) * assign27130_e37662) - (assign27130_e37659 * ((locals.var_fac1p2__blk805_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk805 * locals.var_beta2_dn10)))) / (assign27130_e37662 * assign27130_e37662)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) + locals.var_ty__blk782_dn11)) * assign27130_e37662) - (assign27130_e37659 * (locals.var_fac1p2__blk805_dn11 * locals.var_beta2))) / (assign27130_e37662 * assign27130_e37662)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) + locals.var_ty__blk782_dn12)) * assign27130_e37662) - (assign27130_e37659 * (locals.var_fac1p2__blk805_dn12 * locals.var_beta2))) / (assign27130_e37662 * assign27130_e37662)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) + locals.var_ty__blk782_dn17)) * assign27130_e37662) - (assign27130_e37659 * (locals.var_fac1p2__blk805_dn17 * locals.var_beta2))) / (assign27130_e37662 * assign27130_e37662)),)
    } else {
        (locals.var_tx__blk781, locals.var_tx__blk781_dn0, locals.var_tx__blk781_dn2, locals.var_tx__blk781_dn6, locals.var_tx__blk781_dn7, locals.var_tx__blk781_dn10, locals.var_tx__blk781_dn11, locals.var_tx__blk781_dn12, locals.var_tx__blk781_dn17,)
    }
};
        locals.var_tx__blk781 = assign27130_e37666;
        locals.var_tx__blk781_dn0 = assign27130_e37666_d_n0;
        locals.var_tx__blk781_dn2 = assign27130_e37666_d_n2;
        locals.var_tx__blk781_dn6 = assign27130_e37666_d_n6;
        locals.var_tx__blk781_dn7 = assign27130_e37666_d_n7;
        locals.var_tx__blk781_dn10 = assign27130_e37666_d_n10;
        locals.var_tx__blk781_dn11 = assign27130_e37666_d_n11;
        locals.var_tx__blk781_dn12 = assign27130_e37666_d_n12;
        locals.var_tx__blk781_dn17 = assign27130_e37666_d_n17;
        locals.var_tx__blk781_rv = 0.0;

        let assign27140_e37670: f64 = (10.0 * 2.220446049250313e-16);
        let assign27140_e37671: f64 = if locals.var_tx__blk781 < assign27140_e37670 { 1.0 } else { 0.0 };
        locals.var_guard877 = assign27140_e37671;
        locals.var_guard877_rv = 0.0;

        let (assign27150_e37686, assign27150_e37686_d_n0, assign27150_e37686_d_n2, assign27150_e37686_d_n6, assign27150_e37686_d_n7, assign27150_e37686_d_n10, assign27150_e37686_d_n11, assign27150_e37686_d_n12, assign27150_e37686_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27150_e37684: f64 = (10.0 * 2.220446049250313e-16);
        (assign27150_e37684, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk781, locals.var_tx__blk781_dn0, locals.var_tx__blk781_dn2, locals.var_tx__blk781_dn6, locals.var_tx__blk781_dn7, locals.var_tx__blk781_dn10, locals.var_tx__blk781_dn11, locals.var_tx__blk781_dn12, locals.var_tx__blk781_dn17,)
    }
};
        locals.var_tx__blk781 = assign27150_e37686;
        locals.var_tx__blk781_dn0 = assign27150_e37686_d_n0;
        locals.var_tx__blk781_dn2 = assign27150_e37686_d_n2;
        locals.var_tx__blk781_dn6 = assign27150_e37686_d_n6;
        locals.var_tx__blk781_dn7 = assign27150_e37686_d_n7;
        locals.var_tx__blk781_dn10 = assign27150_e37686_d_n10;
        locals.var_tx__blk781_dn11 = assign27150_e37686_d_n11;
        locals.var_tx__blk781_dn12 = assign27150_e37686_d_n12;
        locals.var_tx__blk781_dn17 = assign27150_e37686_d_n17;
        locals.var_tx__blk781_rv = 0.0;

        let (assign27160_e37708, assign27160_e37708_d_n0, assign27160_e37708_d_n2, assign27160_e37708_d_n6, assign27160_e37708_d_n7, assign27160_e37708_d_n10, assign27160_e37708_d_n11, assign27160_e37708_d_n12, assign27160_e37708_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27160_e37698: f64 = (locals.var_fac1p2__blk805 * locals.var_beta);
        let assign27160_e37700: f64 = (assign27160_e37698 / 2.0);
        let assign27160_e37703: f64 = (locals.var_tx__blk781).sqrt();
        let assign27160_e37704: f64 = (1.0 - assign27160_e37703);
        let assign27160_e37705: f64 = (assign27160_e37700 * assign27160_e37704);
        let assign27160_e37706: f64 = (locals.var_vgpld + assign27160_e37705);
        (assign27160_e37706, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2__blk805_dn0 * locals.var_beta) / 2.0) * assign27160_e37704) + (assign27160_e37700 * (-(locals.var_tx__blk781_dn0 / (2.0 * assign27160_e37703)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2__blk805_dn2 * locals.var_beta) / 2.0) * assign27160_e37704) + (assign27160_e37700 * (-(locals.var_tx__blk781_dn2 / (2.0 * assign27160_e37703)))))), (locals.var_vgpld_dn6 + ((((locals.var_fac1p2__blk805_dn6 * locals.var_beta) / 2.0) * assign27160_e37704) + (assign27160_e37700 * (-(locals.var_tx__blk781_dn6 / (2.0 * assign27160_e37703)))))), (locals.var_vgpld_dn7 + ((((locals.var_fac1p2__blk805_dn7 * locals.var_beta) / 2.0) * assign27160_e37704) + (assign27160_e37700 * (-(locals.var_tx__blk781_dn7 / (2.0 * assign27160_e37703)))))), (locals.var_vgpld_dn10 + (((((locals.var_fac1p2__blk805_dn10 * locals.var_beta) + (locals.var_fac1p2__blk805 * locals.var_beta_dn10)) / 2.0) * assign27160_e37704) + (assign27160_e37700 * (-(locals.var_tx__blk781_dn10 / (2.0 * assign27160_e37703)))))), (locals.var_vgpld_dn11 + ((((locals.var_fac1p2__blk805_dn11 * locals.var_beta) / 2.0) * assign27160_e37704) + (assign27160_e37700 * (-(locals.var_tx__blk781_dn11 / (2.0 * assign27160_e37703)))))), (locals.var_vgpld_dn12 + ((((locals.var_fac1p2__blk805_dn12 * locals.var_beta) / 2.0) * assign27160_e37704) + (assign27160_e37700 * (-(locals.var_tx__blk781_dn12 / (2.0 * assign27160_e37703)))))), (locals.var_vgpld_dn17 + ((((locals.var_fac1p2__blk805_dn17 * locals.var_beta) / 2.0) * assign27160_e37704) + (assign27160_e37700 * (-(locals.var_tx__blk781_dn17 / (2.0 * assign27160_e37703)))))),)
    } else {
        (locals.var_ps0_inia__blk821, locals.var_ps0_inia__blk821_dn0, locals.var_ps0_inia__blk821_dn2, locals.var_ps0_inia__blk821_dn6, locals.var_ps0_inia__blk821_dn7, locals.var_ps0_inia__blk821_dn10, locals.var_ps0_inia__blk821_dn11, locals.var_ps0_inia__blk821_dn12, locals.var_ps0_inia__blk821_dn17,)
    }
};
        locals.var_ps0_inia__blk821 = assign27160_e37708;
        locals.var_ps0_inia__blk821_dn0 = assign27160_e37708_d_n0;
        locals.var_ps0_inia__blk821_dn2 = assign27160_e37708_d_n2;
        locals.var_ps0_inia__blk821_dn6 = assign27160_e37708_d_n6;
        locals.var_ps0_inia__blk821_dn7 = assign27160_e37708_d_n7;
        locals.var_ps0_inia__blk821_dn10 = assign27160_e37708_d_n10;
        locals.var_ps0_inia__blk821_dn11 = assign27160_e37708_d_n11;
        locals.var_ps0_inia__blk821_dn12 = assign27160_e37708_d_n12;
        locals.var_ps0_inia__blk821_dn17 = assign27160_e37708_d_n17;
        locals.var_ps0_inia__blk821_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_97(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27170_e37723, assign27170_e37723_d_n0, assign27170_e37723_d_n2, assign27170_e37723_d_n6, assign27170_e37723_d_n7, assign27170_e37723_d_n10, assign27170_e37723_d_n11, assign27170_e37723_d_n12, assign27170_e37723_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27170_e37720: f64 = (locals.var_ps0_inia__blk821 + locals.var_vxbgmtcl);
        let assign27170_e37721: f64 = (locals.var_beta * assign27170_e37720);
        (assign27170_e37721, (locals.var_beta * (locals.var_ps0_inia__blk821_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign27170_e37720) + (locals.var_beta * (locals.var_ps0_inia__blk821_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk821_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk818, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    }
};
        locals.var_chi__blk818 = assign27170_e37723;
        locals.var_chi__blk818_dn0 = assign27170_e37723_d_n0;
        locals.var_chi__blk818_dn2 = assign27170_e37723_d_n2;
        locals.var_chi__blk818_dn6 = assign27170_e37723_d_n6;
        locals.var_chi__blk818_dn7 = assign27170_e37723_d_n7;
        locals.var_chi__blk818_dn10 = assign27170_e37723_d_n10;
        locals.var_chi__blk818_dn11 = assign27170_e37723_d_n11;
        locals.var_chi__blk818_dn12 = assign27170_e37723_d_n12;
        locals.var_chi__blk818_dn17 = assign27170_e37723_d_n17;
        locals.var_chi__blk818_rv = 0.0;

        let (assign27180_e37736, assign27180_e37736_d_n0, assign27180_e37736_d_n2, assign27180_e37736_d_n6, assign27180_e37736_d_n7, assign27180_e37736_d_n10, assign27180_e37736_d_n11, assign27180_e37736_d_n12, assign27180_e37736_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27180_e37733: f64 = (-locals.var_chi__blk818);
        let assign27180_e37734: f64 = (assign27180_e37733).exp();
        (assign27180_e37734, (assign27180_e37734 * (-locals.var_chi__blk818_dn0)), (assign27180_e37734 * (-locals.var_chi__blk818_dn2)), (assign27180_e37734 * (-locals.var_chi__blk818_dn6)), (assign27180_e37734 * (-locals.var_chi__blk818_dn7)), (assign27180_e37734 * (-locals.var_chi__blk818_dn10)), (assign27180_e37734 * (-locals.var_chi__blk818_dn11)), (assign27180_e37734 * (-locals.var_chi__blk818_dn12)), (assign27180_e37734 * (-locals.var_chi__blk818_dn17)),)
    } else {
        (locals.var_ty__blk782, locals.var_ty__blk782_dn0, locals.var_ty__blk782_dn2, locals.var_ty__blk782_dn6, locals.var_ty__blk782_dn7, locals.var_ty__blk782_dn10, locals.var_ty__blk782_dn11, locals.var_ty__blk782_dn12, locals.var_ty__blk782_dn17,)
    }
};
        locals.var_ty__blk782 = assign27180_e37736;
        locals.var_ty__blk782_dn0 = assign27180_e37736_d_n0;
        locals.var_ty__blk782_dn2 = assign27180_e37736_d_n2;
        locals.var_ty__blk782_dn6 = assign27180_e37736_d_n6;
        locals.var_ty__blk782_dn7 = assign27180_e37736_d_n7;
        locals.var_ty__blk782_dn10 = assign27180_e37736_d_n10;
        locals.var_ty__blk782_dn11 = assign27180_e37736_d_n11;
        locals.var_ty__blk782_dn12 = assign27180_e37736_d_n12;
        locals.var_ty__blk782_dn17 = assign27180_e37736_d_n17;
        locals.var_ty__blk782_rv = 0.0;

        let (assign27190_e37763, assign27190_e37763_d_n0, assign27190_e37763_d_n2, assign27190_e37763_d_n6, assign27190_e37763_d_n7, assign27190_e37763_d_n10, assign27190_e37763_d_n11, assign27190_e37763_d_n12, assign27190_e37763_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27190_e37750: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign27190_e37751: f64 = (locals.var_beta * assign27190_e37750);
        let assign27190_e37753: f64 = (assign27190_e37751 - 1.0);
        let assign27190_e37755: f64 = (assign27190_e37753 + locals.var_ty__blk782);
        let assign27190_e37756: f64 = (4.0 * assign27190_e37755);
        let assign27190_e37759: f64 = (locals.var_fac1p2__blk805 * locals.var_beta2);
        let assign27190_e37760: f64 = (assign27190_e37756 / assign27190_e37759);
        let assign27190_e37761: f64 = (1.0 + assign27190_e37760);
        (assign27190_e37761, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + locals.var_ty__blk782_dn0)) * assign27190_e37759) - (assign27190_e37756 * (locals.var_fac1p2__blk805_dn0 * locals.var_beta2))) / (assign27190_e37759 * assign27190_e37759)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + locals.var_ty__blk782_dn2)) * assign27190_e37759) - (assign27190_e37756 * (locals.var_fac1p2__blk805_dn2 * locals.var_beta2))) / (assign27190_e37759 * assign27190_e37759)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) + locals.var_ty__blk782_dn6)) * assign27190_e37759) - (assign27190_e37756 * (locals.var_fac1p2__blk805_dn6 * locals.var_beta2))) / (assign27190_e37759 * assign27190_e37759)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) + locals.var_ty__blk782_dn7)) * assign27190_e37759) - (assign27190_e37756 * (locals.var_fac1p2__blk805_dn7 * locals.var_beta2))) / (assign27190_e37759 * assign27190_e37759)), ((((4.0 * (((locals.var_beta_dn10 * assign27190_e37750) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))) + locals.var_ty__blk782_dn10)) * assign27190_e37759) - (assign27190_e37756 * ((locals.var_fac1p2__blk805_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk805 * locals.var_beta2_dn10)))) / (assign27190_e37759 * assign27190_e37759)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) + locals.var_ty__blk782_dn11)) * assign27190_e37759) - (assign27190_e37756 * (locals.var_fac1p2__blk805_dn11 * locals.var_beta2))) / (assign27190_e37759 * assign27190_e37759)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) + locals.var_ty__blk782_dn12)) * assign27190_e37759) - (assign27190_e37756 * (locals.var_fac1p2__blk805_dn12 * locals.var_beta2))) / (assign27190_e37759 * assign27190_e37759)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) + locals.var_ty__blk782_dn17)) * assign27190_e37759) - (assign27190_e37756 * (locals.var_fac1p2__blk805_dn17 * locals.var_beta2))) / (assign27190_e37759 * assign27190_e37759)),)
    } else {
        (locals.var_tx__blk781, locals.var_tx__blk781_dn0, locals.var_tx__blk781_dn2, locals.var_tx__blk781_dn6, locals.var_tx__blk781_dn7, locals.var_tx__blk781_dn10, locals.var_tx__blk781_dn11, locals.var_tx__blk781_dn12, locals.var_tx__blk781_dn17,)
    }
};
        locals.var_tx__blk781 = assign27190_e37763;
        locals.var_tx__blk781_dn0 = assign27190_e37763_d_n0;
        locals.var_tx__blk781_dn2 = assign27190_e37763_d_n2;
        locals.var_tx__blk781_dn6 = assign27190_e37763_d_n6;
        locals.var_tx__blk781_dn7 = assign27190_e37763_d_n7;
        locals.var_tx__blk781_dn10 = assign27190_e37763_d_n10;
        locals.var_tx__blk781_dn11 = assign27190_e37763_d_n11;
        locals.var_tx__blk781_dn12 = assign27190_e37763_d_n12;
        locals.var_tx__blk781_dn17 = assign27190_e37763_d_n17;
        locals.var_tx__blk781_rv = 0.0;

        let assign27200_e37767: f64 = (10.0 * 2.220446049250313e-16);
        let assign27200_e37768: f64 = if locals.var_tx__blk781 < assign27200_e37767 { 1.0 } else { 0.0 };
        locals.var_guard878 = assign27200_e37768;
        locals.var_guard878_rv = 0.0;

        let (assign27210_e37783, assign27210_e37783_d_n0, assign27210_e37783_d_n2, assign27210_e37783_d_n6, assign27210_e37783_d_n7, assign27210_e37783_d_n10, assign27210_e37783_d_n11, assign27210_e37783_d_n12, assign27210_e37783_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard878 != 0.0)) {
        let assign27210_e37781: f64 = (10.0 * 2.220446049250313e-16);
        (assign27210_e37781, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk781, locals.var_tx__blk781_dn0, locals.var_tx__blk781_dn2, locals.var_tx__blk781_dn6, locals.var_tx__blk781_dn7, locals.var_tx__blk781_dn10, locals.var_tx__blk781_dn11, locals.var_tx__blk781_dn12, locals.var_tx__blk781_dn17,)
    }
};
        locals.var_tx__blk781 = assign27210_e37783;
        locals.var_tx__blk781_dn0 = assign27210_e37783_d_n0;
        locals.var_tx__blk781_dn2 = assign27210_e37783_d_n2;
        locals.var_tx__blk781_dn6 = assign27210_e37783_d_n6;
        locals.var_tx__blk781_dn7 = assign27210_e37783_d_n7;
        locals.var_tx__blk781_dn10 = assign27210_e37783_d_n10;
        locals.var_tx__blk781_dn11 = assign27210_e37783_d_n11;
        locals.var_tx__blk781_dn12 = assign27210_e37783_d_n12;
        locals.var_tx__blk781_dn17 = assign27210_e37783_d_n17;
        locals.var_tx__blk781_rv = 0.0;

        let (assign27220_e37805, assign27220_e37805_d_n0, assign27220_e37805_d_n2, assign27220_e37805_d_n6, assign27220_e37805_d_n7, assign27220_e37805_d_n10, assign27220_e37805_d_n11, assign27220_e37805_d_n12, assign27220_e37805_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27220_e37795: f64 = (locals.var_fac1p2__blk805 * locals.var_beta);
        let assign27220_e37797: f64 = (assign27220_e37795 / 2.0);
        let assign27220_e37800: f64 = (locals.var_tx__blk781).sqrt();
        let assign27220_e37801: f64 = (1.0 - assign27220_e37800);
        let assign27220_e37802: f64 = (assign27220_e37797 * assign27220_e37801);
        let assign27220_e37803: f64 = (locals.var_vgpld + assign27220_e37802);
        (assign27220_e37803, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2__blk805_dn0 * locals.var_beta) / 2.0) * assign27220_e37801) + (assign27220_e37797 * (-(locals.var_tx__blk781_dn0 / (2.0 * assign27220_e37800)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2__blk805_dn2 * locals.var_beta) / 2.0) * assign27220_e37801) + (assign27220_e37797 * (-(locals.var_tx__blk781_dn2 / (2.0 * assign27220_e37800)))))), (locals.var_vgpld_dn6 + ((((locals.var_fac1p2__blk805_dn6 * locals.var_beta) / 2.0) * assign27220_e37801) + (assign27220_e37797 * (-(locals.var_tx__blk781_dn6 / (2.0 * assign27220_e37800)))))), (locals.var_vgpld_dn7 + ((((locals.var_fac1p2__blk805_dn7 * locals.var_beta) / 2.0) * assign27220_e37801) + (assign27220_e37797 * (-(locals.var_tx__blk781_dn7 / (2.0 * assign27220_e37800)))))), (locals.var_vgpld_dn10 + (((((locals.var_fac1p2__blk805_dn10 * locals.var_beta) + (locals.var_fac1p2__blk805 * locals.var_beta_dn10)) / 2.0) * assign27220_e37801) + (assign27220_e37797 * (-(locals.var_tx__blk781_dn10 / (2.0 * assign27220_e37800)))))), (locals.var_vgpld_dn11 + ((((locals.var_fac1p2__blk805_dn11 * locals.var_beta) / 2.0) * assign27220_e37801) + (assign27220_e37797 * (-(locals.var_tx__blk781_dn11 / (2.0 * assign27220_e37800)))))), (locals.var_vgpld_dn12 + ((((locals.var_fac1p2__blk805_dn12 * locals.var_beta) / 2.0) * assign27220_e37801) + (assign27220_e37797 * (-(locals.var_tx__blk781_dn12 / (2.0 * assign27220_e37800)))))), (locals.var_vgpld_dn17 + ((((locals.var_fac1p2__blk805_dn17 * locals.var_beta) / 2.0) * assign27220_e37801) + (assign27220_e37797 * (-(locals.var_tx__blk781_dn17 / (2.0 * assign27220_e37800)))))),)
    } else {
        (locals.var_ps0_inia__blk821, locals.var_ps0_inia__blk821_dn0, locals.var_ps0_inia__blk821_dn2, locals.var_ps0_inia__blk821_dn6, locals.var_ps0_inia__blk821_dn7, locals.var_ps0_inia__blk821_dn10, locals.var_ps0_inia__blk821_dn11, locals.var_ps0_inia__blk821_dn12, locals.var_ps0_inia__blk821_dn17,)
    }
};
        locals.var_ps0_inia__blk821 = assign27220_e37805;
        locals.var_ps0_inia__blk821_dn0 = assign27220_e37805_d_n0;
        locals.var_ps0_inia__blk821_dn2 = assign27220_e37805_d_n2;
        locals.var_ps0_inia__blk821_dn6 = assign27220_e37805_d_n6;
        locals.var_ps0_inia__blk821_dn7 = assign27220_e37805_d_n7;
        locals.var_ps0_inia__blk821_dn10 = assign27220_e37805_d_n10;
        locals.var_ps0_inia__blk821_dn11 = assign27220_e37805_d_n11;
        locals.var_ps0_inia__blk821_dn12 = assign27220_e37805_d_n12;
        locals.var_ps0_inia__blk821_dn17 = assign27220_e37805_d_n17;
        locals.var_ps0_inia__blk821_rv = 0.0;

        let (assign27230_e37820, assign27230_e37820_d_n0, assign27230_e37820_d_n2, assign27230_e37820_d_n6, assign27230_e37820_d_n7, assign27230_e37820_d_n10, assign27230_e37820_d_n11, assign27230_e37820_d_n12, assign27230_e37820_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27230_e37817: f64 = (locals.var_ps0_inia__blk821 + locals.var_vxbgmtcl);
        let assign27230_e37818: f64 = (locals.var_beta * assign27230_e37817);
        (assign27230_e37818, (locals.var_beta * (locals.var_ps0_inia__blk821_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign27230_e37817) + (locals.var_beta * (locals.var_ps0_inia__blk821_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk821_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk818, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    }
};
        locals.var_chi__blk818 = assign27230_e37820;
        locals.var_chi__blk818_dn0 = assign27230_e37820_d_n0;
        locals.var_chi__blk818_dn2 = assign27230_e37820_d_n2;
        locals.var_chi__blk818_dn6 = assign27230_e37820_d_n6;
        locals.var_chi__blk818_dn7 = assign27230_e37820_d_n7;
        locals.var_chi__blk818_dn10 = assign27230_e37820_d_n10;
        locals.var_chi__blk818_dn11 = assign27230_e37820_d_n11;
        locals.var_chi__blk818_dn12 = assign27230_e37820_d_n12;
        locals.var_chi__blk818_dn17 = assign27230_e37820_d_n17;
        locals.var_chi__blk818_rv = 0.0;

        let assign27240_e37823: f64 = if locals.var_chi__blk818 < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard879 = assign27240_e37823;
        locals.var_guard879_rv = 0.0;

        let (assign27260_e37866,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27260_e37850: f64 = (9.0 * 1.414213562373095);
        let assign27260_e37851: f64 = (1.0 / assign27260_e37850);
        let assign27260_e37855: f64 = (7.0 * 0.049787068367863944);
        let assign27260_e37856: f64 = (5.0 + assign27260_e37855);
        let assign27260_e37860: f64 = (2.0 + 0.049787068367863944);
        let assign27260_e37861: f64 = (assign27260_e37860).sqrt();
        let assign27260_e37862: f64 = (54.0 * assign27260_e37861);
        let assign27260_e37863: f64 = (assign27260_e37856 / assign27260_e37862);
        let assign27260_e37864: f64 = (assign27260_e37851 - assign27260_e37863);
        (assign27260_e37864,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign27260_e37866;
        locals.var_ta_rv = 0.0;

        let (assign27270_e37892,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27270_e37879: f64 = (1.0 + 0.049787068367863944);
        let assign27270_e37883: f64 = (2.0 + 0.049787068367863944);
        let assign27270_e37884: f64 = (assign27270_e37883).sqrt();
        let assign27270_e37885: f64 = (2.0 * assign27270_e37884);
        let assign27270_e37886: f64 = (assign27270_e37879 / assign27270_e37885);
        let assign27270_e37889: f64 = (1.414213562373095 / 3.0);
        let assign27270_e37890: f64 = (assign27270_e37886 - assign27270_e37889);
        (assign27270_e37890,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign27270_e37892;
        locals.var_tb_rv = 0.0;

        let (assign27280_e37913, assign27280_e37913_d_n0, assign27280_e37913_d_n2, assign27280_e37913_d_n6, assign27280_e37913_d_n7, assign27280_e37913_d_n10, assign27280_e37913_d_n11, assign27280_e37913_d_n12, assign27280_e37913_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27280_e37905: f64 = (1.0 / 1.414213562373095);
        let assign27280_e37909: f64 = (locals.var_beta * locals.var_fac1__blk804);
        let assign27280_e37910: f64 = (1.0 / assign27280_e37909);
        let assign27280_e37911: f64 = (assign27280_e37905 + assign27280_e37910);
        (assign27280_e37911, (-((locals.var_beta * locals.var_fac1__blk804_dn0) / (assign27280_e37909 * assign27280_e37909))), (-((locals.var_beta * locals.var_fac1__blk804_dn2) / (assign27280_e37909 * assign27280_e37909))), (-((locals.var_beta * locals.var_fac1__blk804_dn6) / (assign27280_e37909 * assign27280_e37909))), (-((locals.var_beta * locals.var_fac1__blk804_dn7) / (assign27280_e37909 * assign27280_e37909))), (-(((locals.var_beta_dn10 * locals.var_fac1__blk804) + (locals.var_beta * locals.var_fac1__blk804_dn10)) / (assign27280_e37909 * assign27280_e37909))), (-((locals.var_beta * locals.var_fac1__blk804_dn11) / (assign27280_e37909 * assign27280_e37909))), (-((locals.var_beta * locals.var_fac1__blk804_dn12) / (assign27280_e37909 * assign27280_e37909))), (-((locals.var_beta * locals.var_fac1__blk804_dn17) / (assign27280_e37909 * assign27280_e37909))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn12, locals.var_tc_dn17,)
    }
};
        locals.var_tc = assign27280_e37913;
        locals.var_tc_dn0 = assign27280_e37913_d_n0;
        locals.var_tc_dn2 = assign27280_e37913_d_n2;
        locals.var_tc_dn6 = assign27280_e37913_d_n6;
        locals.var_tc_dn7 = assign27280_e37913_d_n7;
        locals.var_tc_dn10 = assign27280_e37913_d_n10;
        locals.var_tc_dn11 = assign27280_e37913_d_n11;
        locals.var_tc_dn12 = assign27280_e37913_d_n12;
        locals.var_tc_dn17 = assign27280_e37913_d_n17;
        locals.var_tc_rv = 0.0;

        let (assign27290_e37931, assign27290_e37931_d_n0, assign27290_e37931_d_n2, assign27290_e37931_d_n6, assign27290_e37931_d_n7, assign27290_e37931_d_n10, assign27290_e37931_d_n11, assign27290_e37931_d_n12, assign27290_e37931_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27290_e37926: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign27290_e37927: f64 = (-assign27290_e37926);
        let assign27290_e37929: f64 = (assign27290_e37927 / locals.var_fac1__blk804);
        (assign27290_e37929, ((((-(locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) * locals.var_fac1__blk804) - (assign27290_e37927 * locals.var_fac1__blk804_dn0)) / (locals.var_fac1__blk804 * locals.var_fac1__blk804)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1__blk804) - (assign27290_e37927 * locals.var_fac1__blk804_dn2)) / (locals.var_fac1__blk804 * locals.var_fac1__blk804)), ((((-(locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) * locals.var_fac1__blk804) - (assign27290_e37927 * locals.var_fac1__blk804_dn6)) / (locals.var_fac1__blk804 * locals.var_fac1__blk804)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1__blk804) - (assign27290_e37927 * locals.var_fac1__blk804_dn7)) / (locals.var_fac1__blk804 * locals.var_fac1__blk804)), ((((-(locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10)) * locals.var_fac1__blk804) - (assign27290_e37927 * locals.var_fac1__blk804_dn10)) / (locals.var_fac1__blk804 * locals.var_fac1__blk804)), ((((-(locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) * locals.var_fac1__blk804) - (assign27290_e37927 * locals.var_fac1__blk804_dn11)) / (locals.var_fac1__blk804 * locals.var_fac1__blk804)), ((((-(locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) * locals.var_fac1__blk804) - (assign27290_e37927 * locals.var_fac1__blk804_dn12)) / (locals.var_fac1__blk804 * locals.var_fac1__blk804)), ((((-(locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) * locals.var_fac1__blk804) - (assign27290_e37927 * locals.var_fac1__blk804_dn17)) / (locals.var_fac1__blk804 * locals.var_fac1__blk804)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn12, locals.var_td_dn17,)
    }
};
        locals.var_td = assign27290_e37931;
        locals.var_td_dn0 = assign27290_e37931_d_n0;
        locals.var_td_dn2 = assign27290_e37931_d_n2;
        locals.var_td_dn6 = assign27290_e37931_d_n6;
        locals.var_td_dn7 = assign27290_e37931_d_n7;
        locals.var_td_dn10 = assign27290_e37931_d_n10;
        locals.var_td_dn11 = assign27290_e37931_d_n11;
        locals.var_td_dn12 = assign27290_e37931_d_n12;
        locals.var_td_dn17 = assign27290_e37931_d_n17;
        locals.var_td_rv = 0.0;

        let (assign27300_e37972, assign27300_e37972_d_n0, assign27300_e37972_d_n2, assign27300_e37972_d_n6, assign27300_e37972_d_n7, assign27300_e37972_d_n10, assign27300_e37972_d_n11, assign27300_e37972_d_n12, assign27300_e37972_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27300_e37944: f64 = (locals.var_tb * locals.var_tb);
        let assign27300_e37946: f64 = (assign27300_e37944 * locals.var_tb);
        let assign27300_e37949: f64 = (27.0 * locals.var_ta);
        let assign27300_e37951: f64 = (assign27300_e37949 * locals.var_ta);
        let assign27300_e37953: f64 = (assign27300_e37951 * locals.var_ta);
        let assign27300_e37954: f64 = (assign27300_e37946 / assign27300_e37953);
        let assign27300_e37957: f64 = (locals.var_tb * locals.var_tc);
        let assign27300_e37960: f64 = (6.0 * locals.var_ta);
        let assign27300_e37962: f64 = (assign27300_e37960 * locals.var_ta);
        let assign27300_e37963: f64 = (assign27300_e37957 / assign27300_e37962);
        let assign27300_e37964: f64 = (assign27300_e37954 - assign27300_e37963);
        let assign27300_e37968: f64 = (2.0 * locals.var_ta);
        let assign27300_e37969: f64 = (locals.var_td / assign27300_e37968);
        let assign27300_e37970: f64 = (assign27300_e37964 + assign27300_e37969);
        (assign27300_e37970, ((-((locals.var_tb * locals.var_tc_dn0) / assign27300_e37962)) + (locals.var_td_dn0 / assign27300_e37968)), ((-((locals.var_tb * locals.var_tc_dn2) / assign27300_e37962)) + (locals.var_td_dn2 / assign27300_e37968)), ((-((locals.var_tb * locals.var_tc_dn6) / assign27300_e37962)) + (locals.var_td_dn6 / assign27300_e37968)), ((-((locals.var_tb * locals.var_tc_dn7) / assign27300_e37962)) + (locals.var_td_dn7 / assign27300_e37968)), ((-((locals.var_tb * locals.var_tc_dn10) / assign27300_e37962)) + (locals.var_td_dn10 / assign27300_e37968)), ((-((locals.var_tb * locals.var_tc_dn11) / assign27300_e37962)) + (locals.var_td_dn11 / assign27300_e37968)), ((-((locals.var_tb * locals.var_tc_dn12) / assign27300_e37962)) + (locals.var_td_dn12 / assign27300_e37968)), ((-((locals.var_tb * locals.var_tc_dn17) / assign27300_e37962)) + (locals.var_td_dn17 / assign27300_e37968)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn12, locals.var_tq_dn17,)
    }
};
        locals.var_tq = assign27300_e37972;
        locals.var_tq_dn0 = assign27300_e37972_d_n0;
        locals.var_tq_dn2 = assign27300_e37972_d_n2;
        locals.var_tq_dn6 = assign27300_e37972_d_n6;
        locals.var_tq_dn7 = assign27300_e37972_d_n7;
        locals.var_tq_dn10 = assign27300_e37972_d_n10;
        locals.var_tq_dn11 = assign27300_e37972_d_n11;
        locals.var_tq_dn12 = assign27300_e37972_d_n12;
        locals.var_tq_dn17 = assign27300_e37972_d_n17;
        locals.var_tq_rv = 0.0;

        let (assign27310_e37999, assign27310_e37999_d_n0, assign27310_e37999_d_n2, assign27310_e37999_d_n6, assign27310_e37999_d_n7, assign27310_e37999_d_n10, assign27310_e37999_d_n11, assign27310_e37999_d_n12, assign27310_e37999_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27310_e37985: f64 = (3.0 * locals.var_ta);
        let assign27310_e37987: f64 = (assign27310_e37985 * locals.var_tc);
        let assign27310_e37990: f64 = (locals.var_tb * locals.var_tb);
        let assign27310_e37991: f64 = (assign27310_e37987 - assign27310_e37990);
        let assign27310_e37994: f64 = (9.0 * locals.var_ta);
        let assign27310_e37996: f64 = (assign27310_e37994 * locals.var_ta);
        let assign27310_e37997: f64 = (assign27310_e37991 / assign27310_e37996);
        (assign27310_e37997, ((assign27310_e37985 * locals.var_tc_dn0) / assign27310_e37996), ((assign27310_e37985 * locals.var_tc_dn2) / assign27310_e37996), ((assign27310_e37985 * locals.var_tc_dn6) / assign27310_e37996), ((assign27310_e37985 * locals.var_tc_dn7) / assign27310_e37996), ((assign27310_e37985 * locals.var_tc_dn10) / assign27310_e37996), ((assign27310_e37985 * locals.var_tc_dn11) / assign27310_e37996), ((assign27310_e37985 * locals.var_tc_dn12) / assign27310_e37996), ((assign27310_e37985 * locals.var_tc_dn17) / assign27310_e37996),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn12, locals.var_tp_dn17,)
    }
};
        locals.var_tp = assign27310_e37999;
        locals.var_tp_dn0 = assign27310_e37999_d_n0;
        locals.var_tp_dn2 = assign27310_e37999_d_n2;
        locals.var_tp_dn6 = assign27310_e37999_d_n6;
        locals.var_tp_dn7 = assign27310_e37999_d_n7;
        locals.var_tp_dn10 = assign27310_e37999_d_n10;
        locals.var_tp_dn11 = assign27310_e37999_d_n11;
        locals.var_tp_dn12 = assign27310_e37999_d_n12;
        locals.var_tp_dn17 = assign27310_e37999_d_n17;
        locals.var_tp_rv = 0.0;

        let (assign27320_e38021, assign27320_e38021_d_n0, assign27320_e38021_d_n2, assign27320_e38021_d_n6, assign27320_e38021_d_n7, assign27320_e38021_d_n10, assign27320_e38021_d_n11, assign27320_e38021_d_n12, assign27320_e38021_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27320_e38012: f64 = (locals.var_tq * locals.var_tq);
        let assign27320_e38015: f64 = (locals.var_tp * locals.var_tp);
        let assign27320_e38017: f64 = (assign27320_e38015 * locals.var_tp);
        let assign27320_e38018: f64 = (assign27320_e38012 + assign27320_e38017);
        let assign27320_e38019: f64 = (assign27320_e38018).sqrt();
        (assign27320_e38019, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign27320_e38015 * locals.var_tp_dn0))) / (2.0 * assign27320_e38019)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign27320_e38015 * locals.var_tp_dn2))) / (2.0 * assign27320_e38019)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign27320_e38015 * locals.var_tp_dn6))) / (2.0 * assign27320_e38019)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign27320_e38015 * locals.var_tp_dn7))) / (2.0 * assign27320_e38019)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign27320_e38015 * locals.var_tp_dn10))) / (2.0 * assign27320_e38019)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign27320_e38015 * locals.var_tp_dn11))) / (2.0 * assign27320_e38019)), ((((locals.var_tq_dn12 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn12)) + ((((locals.var_tp_dn12 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn12)) * locals.var_tp) + (assign27320_e38015 * locals.var_tp_dn12))) / (2.0 * assign27320_e38019)), ((((locals.var_tq_dn17 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn17)) + ((((locals.var_tp_dn17 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn17)) * locals.var_tp) + (assign27320_e38015 * locals.var_tp_dn17))) / (2.0 * assign27320_e38019)),)
    } else {
        (locals.var_t5__blk778, locals.var_t5__blk778_dn0, locals.var_t5__blk778_dn2, locals.var_t5__blk778_dn6, locals.var_t5__blk778_dn7, locals.var_t5__blk778_dn10, locals.var_t5__blk778_dn11, locals.var_t5__blk778_dn12, locals.var_t5__blk778_dn17,)
    }
};
        locals.var_t5__blk778 = assign27320_e38021;
        locals.var_t5__blk778_dn0 = assign27320_e38021_d_n0;
        locals.var_t5__blk778_dn2 = assign27320_e38021_d_n2;
        locals.var_t5__blk778_dn6 = assign27320_e38021_d_n6;
        locals.var_t5__blk778_dn7 = assign27320_e38021_d_n7;
        locals.var_t5__blk778_dn10 = assign27320_e38021_d_n10;
        locals.var_t5__blk778_dn11 = assign27320_e38021_d_n11;
        locals.var_t5__blk778_dn12 = assign27320_e38021_d_n12;
        locals.var_t5__blk778_dn17 = assign27320_e38021_d_n17;
        locals.var_t5__blk778_rv = 0.0;

        let (assign27330_e38039, assign27330_e38039_d_n0, assign27330_e38039_d_n2, assign27330_e38039_d_n6, assign27330_e38039_d_n7, assign27330_e38039_d_n10, assign27330_e38039_d_n11, assign27330_e38039_d_n12, assign27330_e38039_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27330_e38033: f64 = (-locals.var_tq);
        let assign27330_e38035: f64 = (assign27330_e38033 + locals.var_t5__blk778);
        let assign27330_e38037: f64 = (assign27330_e38035).powf(0.3333333333333333);
        (assign27330_e38037, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27330_e38035).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5__blk778_dn0))) } } else { (assign27330_e38037 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5__blk778_dn0) / assign27330_e38035))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27330_e38035).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5__blk778_dn2))) } } else { (assign27330_e38037 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5__blk778_dn2) / assign27330_e38035))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27330_e38035).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5__blk778_dn6))) } } else { (assign27330_e38037 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5__blk778_dn6) / assign27330_e38035))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27330_e38035).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5__blk778_dn7))) } } else { (assign27330_e38037 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5__blk778_dn7) / assign27330_e38035))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27330_e38035).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5__blk778_dn10))) } } else { (assign27330_e38037 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5__blk778_dn10) / assign27330_e38035))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27330_e38035).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5__blk778_dn11))) } } else { (assign27330_e38037 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5__blk778_dn11) / assign27330_e38035))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27330_e38035).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn12) + locals.var_t5__blk778_dn12))) } } else { (assign27330_e38037 * (0.3333333333333333 * (((-locals.var_tq_dn12) + locals.var_t5__blk778_dn12) / assign27330_e38035))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27330_e38035).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn17) + locals.var_t5__blk778_dn17))) } } else { (assign27330_e38037 * (0.3333333333333333 * (((-locals.var_tq_dn17) + locals.var_t5__blk778_dn17) / assign27330_e38035))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn12, locals.var_tu_dn17,)
    }
};
        locals.var_tu = assign27330_e38039;
        locals.var_tu_dn0 = assign27330_e38039_d_n0;
        locals.var_tu_dn2 = assign27330_e38039_d_n2;
        locals.var_tu_dn6 = assign27330_e38039_d_n6;
        locals.var_tu_dn7 = assign27330_e38039_d_n7;
        locals.var_tu_dn10 = assign27330_e38039_d_n10;
        locals.var_tu_dn11 = assign27330_e38039_d_n11;
        locals.var_tu_dn12 = assign27330_e38039_d_n12;
        locals.var_tu_dn17 = assign27330_e38039_d_n17;
        locals.var_tu_rv = 0.0;

        let (assign27340_e38057, assign27340_e38057_d_n0, assign27340_e38057_d_n2, assign27340_e38057_d_n6, assign27340_e38057_d_n7, assign27340_e38057_d_n10, assign27340_e38057_d_n11, assign27340_e38057_d_n12, assign27340_e38057_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27340_e38052: f64 = (locals.var_tq + locals.var_t5__blk778);
        let assign27340_e38054: f64 = (assign27340_e38052).powf(0.3333333333333333);
        let assign27340_e38055: f64 = (-assign27340_e38054);
        (assign27340_e38055, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27340_e38052).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5__blk778_dn0))) } } else { (assign27340_e38054 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5__blk778_dn0) / assign27340_e38052))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27340_e38052).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5__blk778_dn2))) } } else { (assign27340_e38054 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5__blk778_dn2) / assign27340_e38052))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27340_e38052).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5__blk778_dn6))) } } else { (assign27340_e38054 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5__blk778_dn6) / assign27340_e38052))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27340_e38052).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5__blk778_dn7))) } } else { (assign27340_e38054 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5__blk778_dn7) / assign27340_e38052))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27340_e38052).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5__blk778_dn10))) } } else { (assign27340_e38054 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5__blk778_dn10) / assign27340_e38052))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27340_e38052).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5__blk778_dn11))) } } else { (assign27340_e38054 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5__blk778_dn11) / assign27340_e38052))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27340_e38052).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn12 + locals.var_t5__blk778_dn12))) } } else { (assign27340_e38054 * (0.3333333333333333 * ((locals.var_tq_dn12 + locals.var_t5__blk778_dn12) / assign27340_e38052))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27340_e38052).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn17 + locals.var_t5__blk778_dn17))) } } else { (assign27340_e38054 * (0.3333333333333333 * ((locals.var_tq_dn17 + locals.var_t5__blk778_dn17) / assign27340_e38052))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn12, locals.var_tv_dn17,)
    }
};
        locals.var_tv = assign27340_e38057;
        locals.var_tv_dn0 = assign27340_e38057_d_n0;
        locals.var_tv_dn2 = assign27340_e38057_d_n2;
        locals.var_tv_dn6 = assign27340_e38057_d_n6;
        locals.var_tv_dn7 = assign27340_e38057_d_n7;
        locals.var_tv_dn10 = assign27340_e38057_d_n10;
        locals.var_tv_dn11 = assign27340_e38057_d_n11;
        locals.var_tv_dn12 = assign27340_e38057_d_n12;
        locals.var_tv_dn17 = assign27340_e38057_d_n17;
        locals.var_tv_rv = 0.0;

        let (assign27350_e38078, assign27350_e38078_d_n0, assign27350_e38078_d_n2, assign27350_e38078_d_n6, assign27350_e38078_d_n7, assign27350_e38078_d_n10, assign27350_e38078_d_n11, assign27350_e38078_d_n12, assign27350_e38078_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27350_e38070: f64 = (locals.var_tu + locals.var_tv);
        let assign27350_e38074: f64 = (3.0 * locals.var_ta);
        let assign27350_e38075: f64 = (locals.var_tb / assign27350_e38074);
        let assign27350_e38076: f64 = (assign27350_e38070 - assign27350_e38075);
        (assign27350_e38076, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn12 + locals.var_tv_dn12), (locals.var_tu_dn17 + locals.var_tv_dn17),)
    } else {
        (locals.var_tx__blk781, locals.var_tx__blk781_dn0, locals.var_tx__blk781_dn2, locals.var_tx__blk781_dn6, locals.var_tx__blk781_dn7, locals.var_tx__blk781_dn10, locals.var_tx__blk781_dn11, locals.var_tx__blk781_dn12, locals.var_tx__blk781_dn17,)
    }
};
        locals.var_tx__blk781 = assign27350_e38078;
        locals.var_tx__blk781_dn0 = assign27350_e38078_d_n0;
        locals.var_tx__blk781_dn2 = assign27350_e38078_d_n2;
        locals.var_tx__blk781_dn6 = assign27350_e38078_d_n6;
        locals.var_tx__blk781_dn7 = assign27350_e38078_d_n7;
        locals.var_tx__blk781_dn10 = assign27350_e38078_d_n10;
        locals.var_tx__blk781_dn11 = assign27350_e38078_d_n11;
        locals.var_tx__blk781_dn12 = assign27350_e38078_d_n12;
        locals.var_tx__blk781_dn17 = assign27350_e38078_d_n17;
        locals.var_tx__blk781_rv = 0.0;

        let (assign27360_e38095, assign27360_e38095_d_n0, assign27360_e38095_d_n2, assign27360_e38095_d_n6, assign27360_e38095_d_n7, assign27360_e38095_d_n10, assign27360_e38095_d_n11, assign27360_e38095_d_n12, assign27360_e38095_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27360_e38091: f64 = (locals.var_tx__blk781 * locals.var_beta_inv);
        let assign27360_e38093: f64 = (assign27360_e38091 - locals.var_vxbgmtcl);
        (assign27360_e38093, ((locals.var_tx__blk781_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_tx__blk781_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), ((locals.var_tx__blk781_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_tx__blk781_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn7), (((locals.var_tx__blk781_dn10 * locals.var_beta_inv) + (locals.var_tx__blk781 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), ((locals.var_tx__blk781_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_tx__blk781_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12), ((locals.var_tx__blk781_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0_inia__blk821, locals.var_ps0_inia__blk821_dn0, locals.var_ps0_inia__blk821_dn2, locals.var_ps0_inia__blk821_dn6, locals.var_ps0_inia__blk821_dn7, locals.var_ps0_inia__blk821_dn10, locals.var_ps0_inia__blk821_dn11, locals.var_ps0_inia__blk821_dn12, locals.var_ps0_inia__blk821_dn17,)
    }
};
        locals.var_ps0_inia__blk821 = assign27360_e38095;
        locals.var_ps0_inia__blk821_dn0 = assign27360_e38095_d_n0;
        locals.var_ps0_inia__blk821_dn2 = assign27360_e38095_d_n2;
        locals.var_ps0_inia__blk821_dn6 = assign27360_e38095_d_n6;
        locals.var_ps0_inia__blk821_dn7 = assign27360_e38095_d_n7;
        locals.var_ps0_inia__blk821_dn10 = assign27360_e38095_d_n10;
        locals.var_ps0_inia__blk821_dn11 = assign27360_e38095_d_n11;
        locals.var_ps0_inia__blk821_dn12 = assign27360_e38095_d_n12;
        locals.var_ps0_inia__blk821_dn17 = assign27360_e38095_d_n17;
        locals.var_ps0_inia__blk821_rv = 0.0;

        let (assign27370_e38112, assign27370_e38112_d_n0, assign27370_e38112_d_n2, assign27370_e38112_d_n6, assign27370_e38112_d_n7, assign27370_e38112_d_n10, assign27370_e38112_d_n11, assign27370_e38112_d_n12, assign27370_e38112_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27370_e38109: f64 = (locals.var_ps0_inia__blk821 + locals.var_vxbgmtcl);
        let assign27370_e38110: f64 = (locals.var_beta * assign27370_e38109);
        (assign27370_e38110, (locals.var_beta * (locals.var_ps0_inia__blk821_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign27370_e38109) + (locals.var_beta * (locals.var_ps0_inia__blk821_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk821_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk821_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk818, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    }
};
        locals.var_chi__blk818 = assign27370_e38112;
        locals.var_chi__blk818_dn0 = assign27370_e38112_d_n0;
        locals.var_chi__blk818_dn2 = assign27370_e38112_d_n2;
        locals.var_chi__blk818_dn6 = assign27370_e38112_d_n6;
        locals.var_chi__blk818_dn7 = assign27370_e38112_d_n7;
        locals.var_chi__blk818_dn10 = assign27370_e38112_d_n10;
        locals.var_chi__blk818_dn11 = assign27370_e38112_d_n11;
        locals.var_chi__blk818_dn12 = assign27370_e38112_d_n12;
        locals.var_chi__blk818_dn17 = assign27370_e38112_d_n17;
        locals.var_chi__blk818_rv = 0.0;

        let (assign27390_e38140, assign27390_e38140_d_n0, assign27390_e38140_d_n2, assign27390_e38140_d_n6, assign27390_e38140_d_n7, assign27390_e38140_d_n10, assign27390_e38140_d_n11, assign27390_e38140_d_n12, assign27390_e38140_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27390_e38136: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign27390_e38138: f64 = (assign27390_e38136 + 0.1);
        (assign27390_e38138, (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12), (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn12, locals.var_vgpld_shift_dn17,)
    }
};
        locals.var_vgpld_shift = assign27390_e38140;
        locals.var_vgpld_shift_dn0 = assign27390_e38140_d_n0;
        locals.var_vgpld_shift_dn2 = assign27390_e38140_d_n2;
        locals.var_vgpld_shift_dn6 = assign27390_e38140_d_n6;
        locals.var_vgpld_shift_dn7 = assign27390_e38140_d_n7;
        locals.var_vgpld_shift_dn10 = assign27390_e38140_d_n10;
        locals.var_vgpld_shift_dn11 = assign27390_e38140_d_n11;
        locals.var_vgpld_shift_dn12 = assign27390_e38140_d_n12;
        locals.var_vgpld_shift_dn17 = assign27390_e38140_d_n17;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign27400_e38157, assign27400_e38157_d_n0, assign27400_e38157_d_n2, assign27400_e38157_d_n6, assign27400_e38157_d_n7, assign27400_e38157_d_n10, assign27400_e38157_d_n11, assign27400_e38157_d_n12, assign27400_e38157_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27400_e38151: f64 = (-locals.var_vxbgmtcl);
        let assign27400_e38152: f64 = (locals.var_beta * assign27400_e38151);
        let assign27400_e38153: f64 = (assign27400_e38152).exp();
        let assign27400_e38155: f64 = (assign27400_e38153 + 1e-50);
        (assign27400_e38155, (assign27400_e38153 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign27400_e38153 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign27400_e38153 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign27400_e38153 * (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), (assign27400_e38153 * ((locals.var_beta_dn10 * assign27400_e38151) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign27400_e38153 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign27400_e38153 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))), (assign27400_e38153 * (locals.var_beta * (-locals.var_vxbgmtcl_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk837, locals.var_exp_bvbs__blk837_dn0, locals.var_exp_bvbs__blk837_dn2, locals.var_exp_bvbs__blk837_dn6, locals.var_exp_bvbs__blk837_dn7, locals.var_exp_bvbs__blk837_dn10, locals.var_exp_bvbs__blk837_dn11, locals.var_exp_bvbs__blk837_dn12, locals.var_exp_bvbs__blk837_dn17,)
    }
};
        locals.var_exp_bvbs__blk837 = assign27400_e38157;
        locals.var_exp_bvbs__blk837_dn0 = assign27400_e38157_d_n0;
        locals.var_exp_bvbs__blk837_dn2 = assign27400_e38157_d_n2;
        locals.var_exp_bvbs__blk837_dn6 = assign27400_e38157_d_n6;
        locals.var_exp_bvbs__blk837_dn7 = assign27400_e38157_d_n7;
        locals.var_exp_bvbs__blk837_dn10 = assign27400_e38157_d_n10;
        locals.var_exp_bvbs__blk837_dn11 = assign27400_e38157_d_n11;
        locals.var_exp_bvbs__blk837_dn12 = assign27400_e38157_d_n12;
        locals.var_exp_bvbs__blk837_dn17 = assign27400_e38157_d_n17;
        locals.var_exp_bvbs__blk837_rv = 0.0;

        let (assign27410_e38170, assign27410_e38170_d_n0, assign27410_e38170_d_n2, assign27410_e38170_d_n6, assign27410_e38170_d_n7, assign27410_e38170_d_n10, assign27410_e38170_d_n11, assign27410_e38170_d_n12, assign27410_e38170_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27410_e38168: f64 = (locals.var_nin / locals.var_uc_nsubbttub);
        (assign27410_e38168, (((locals.var_nin_dn0 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn0)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn2 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn2)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn6 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn6)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn7 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn7)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn10 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn10)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn11 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn11)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn12 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn12)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn17 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn17)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)),)
    } else {
        (locals.var_t0__blk774, locals.var_t0__blk774_dn0, locals.var_t0__blk774_dn2, locals.var_t0__blk774_dn6, locals.var_t0__blk774_dn7, locals.var_t0__blk774_dn10, locals.var_t0__blk774_dn11, locals.var_t0__blk774_dn12, locals.var_t0__blk774_dn17,)
    }
};
        locals.var_t0__blk774 = assign27410_e38170;
        locals.var_t0__blk774_dn0 = assign27410_e38170_d_n0;
        locals.var_t0__blk774_dn2 = assign27410_e38170_d_n2;
        locals.var_t0__blk774_dn6 = assign27410_e38170_d_n6;
        locals.var_t0__blk774_dn7 = assign27410_e38170_d_n7;
        locals.var_t0__blk774_dn10 = assign27410_e38170_d_n10;
        locals.var_t0__blk774_dn11 = assign27410_e38170_d_n11;
        locals.var_t0__blk774_dn12 = assign27410_e38170_d_n12;
        locals.var_t0__blk774_dn17 = assign27410_e38170_d_n17;
        locals.var_t0__blk774_rv = 0.0;

        let (assign27420_e38183, assign27420_e38183_d_n0, assign27420_e38183_d_n2, assign27420_e38183_d_n6, assign27420_e38183_d_n7, assign27420_e38183_d_n10, assign27420_e38183_d_n11, assign27420_e38183_d_n12, assign27420_e38183_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27420_e38181: f64 = (locals.var_t0__blk774 * locals.var_t0__blk774);
        (assign27420_e38181, ((locals.var_t0__blk774_dn0 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn0)), ((locals.var_t0__blk774_dn2 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn2)), ((locals.var_t0__blk774_dn6 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn6)), ((locals.var_t0__blk774_dn7 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn7)), ((locals.var_t0__blk774_dn10 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn10)), ((locals.var_t0__blk774_dn11 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn11)), ((locals.var_t0__blk774_dn12 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn12)), ((locals.var_t0__blk774_dn17 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn17)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12, locals.var_cnst1over_dn17,)
    }
};
        locals.var_cnst1over = assign27420_e38183;
        locals.var_cnst1over_dn0 = assign27420_e38183_d_n0;
        locals.var_cnst1over_dn2 = assign27420_e38183_d_n2;
        locals.var_cnst1over_dn6 = assign27420_e38183_d_n6;
        locals.var_cnst1over_dn7 = assign27420_e38183_d_n7;
        locals.var_cnst1over_dn10 = assign27420_e38183_d_n10;
        locals.var_cnst1over_dn11 = assign27420_e38183_d_n11;
        locals.var_cnst1over_dn12 = assign27420_e38183_d_n12;
        locals.var_cnst1over_dn17 = assign27420_e38183_d_n17;
        locals.var_cnst1over_rv = 0.0;

        let (assign27430_e38196, assign27430_e38196_d_n0, assign27430_e38196_d_n2, assign27430_e38196_d_n6, assign27430_e38196_d_n7, assign27430_e38196_d_n10, assign27430_e38196_d_n11, assign27430_e38196_d_n12, assign27430_e38196_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27430_e38194: f64 = (locals.var_cnst1over * locals.var_exp_bvbs__blk837);
        (assign27430_e38194, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn2)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn7)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn12)), ((locals.var_cnst1over_dn17 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn17)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn12, locals.var_gammachi_dn17,)
    }
};
        locals.var_gammachi = assign27430_e38196;
        locals.var_gammachi_dn0 = assign27430_e38196_d_n0;
        locals.var_gammachi_dn2 = assign27430_e38196_d_n2;
        locals.var_gammachi_dn6 = assign27430_e38196_d_n6;
        locals.var_gammachi_dn7 = assign27430_e38196_d_n7;
        locals.var_gammachi_dn10 = assign27430_e38196_d_n10;
        locals.var_gammachi_dn11 = assign27430_e38196_d_n11;
        locals.var_gammachi_dn12 = assign27430_e38196_d_n12;
        locals.var_gammachi_dn17 = assign27430_e38196_d_n17;
        locals.var_gammachi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_98(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27440_e38209, assign27440_e38209_d_n0, assign27440_e38209_d_n2, assign27440_e38209_d_n6, assign27440_e38209_d_n7, assign27440_e38209_d_n10, assign27440_e38209_d_n11, assign27440_e38209_d_n12, assign27440_e38209_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27440_e38207: f64 = (locals.var_beta2 * locals.var_fac1p2__blk805);
        (assign27440_e38207, (locals.var_beta2 * locals.var_fac1p2__blk805_dn0), (locals.var_beta2 * locals.var_fac1p2__blk805_dn2), (locals.var_beta2 * locals.var_fac1p2__blk805_dn6), (locals.var_beta2 * locals.var_fac1p2__blk805_dn7), ((locals.var_beta2_dn10 * locals.var_fac1p2__blk805) + (locals.var_beta2 * locals.var_fac1p2__blk805_dn10)), (locals.var_beta2 * locals.var_fac1p2__blk805_dn11), (locals.var_beta2 * locals.var_fac1p2__blk805_dn12), (locals.var_beta2 * locals.var_fac1p2__blk805_dn17),)
    } else {
        (locals.var_t0__blk774, locals.var_t0__blk774_dn0, locals.var_t0__blk774_dn2, locals.var_t0__blk774_dn6, locals.var_t0__blk774_dn7, locals.var_t0__blk774_dn10, locals.var_t0__blk774_dn11, locals.var_t0__blk774_dn12, locals.var_t0__blk774_dn17,)
    }
};
        locals.var_t0__blk774 = assign27440_e38209;
        locals.var_t0__blk774_dn0 = assign27440_e38209_d_n0;
        locals.var_t0__blk774_dn2 = assign27440_e38209_d_n2;
        locals.var_t0__blk774_dn6 = assign27440_e38209_d_n6;
        locals.var_t0__blk774_dn7 = assign27440_e38209_d_n7;
        locals.var_t0__blk774_dn10 = assign27440_e38209_d_n10;
        locals.var_t0__blk774_dn11 = assign27440_e38209_d_n11;
        locals.var_t0__blk774_dn12 = assign27440_e38209_d_n12;
        locals.var_t0__blk774_dn17 = assign27440_e38209_d_n17;
        locals.var_t0__blk774_rv = 0.0;

        let (assign27450_e38222, assign27450_e38222_d_n0, assign27450_e38222_d_n2, assign27450_e38222_d_n6, assign27450_e38222_d_n7, assign27450_e38222_d_n10, assign27450_e38222_d_n11, assign27450_e38222_d_n12, assign27450_e38222_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27450_e38220: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign27450_e38220, (locals.var_beta * locals.var_vgpld_shift_dn0), (locals.var_beta * locals.var_vgpld_shift_dn2), (locals.var_beta * locals.var_vgpld_shift_dn6), (locals.var_beta * locals.var_vgpld_shift_dn7), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), (locals.var_beta * locals.var_vgpld_shift_dn11), (locals.var_beta * locals.var_vgpld_shift_dn12), (locals.var_beta * locals.var_vgpld_shift_dn17),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign27450_e38222;
        locals.var_psi_dn0 = assign27450_e38222_d_n0;
        locals.var_psi_dn2 = assign27450_e38222_d_n2;
        locals.var_psi_dn6 = assign27450_e38222_d_n6;
        locals.var_psi_dn7 = assign27450_e38222_d_n7;
        locals.var_psi_dn10 = assign27450_e38222_d_n10;
        locals.var_psi_dn11 = assign27450_e38222_d_n11;
        locals.var_psi_dn12 = assign27450_e38222_d_n12;
        locals.var_psi_dn17 = assign27450_e38222_d_n17;
        locals.var_psi_rv = 0.0;

        let (assign27460_e38249, assign27460_e38249_d_n0, assign27460_e38249_d_n2, assign27460_e38249_d_n6, assign27460_e38249_d_n7, assign27460_e38249_d_n10, assign27460_e38249_d_n11, assign27460_e38249_d_n12, assign27460_e38249_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27460_e38233: f64 = (locals.var_gammachi * locals.var_t0__blk774);
        let assign27460_e38236: f64 = (locals.var_psi * locals.var_psi);
        let assign27460_e38237: f64 = (assign27460_e38233 + assign27460_e38236);
        let assign27460_e38238: f64 = (assign27460_e38237).ln();
        let assign27460_e38241: f64 = (locals.var_cnst1over * locals.var_t0__blk774);
        let assign27460_e38242: f64 = (assign27460_e38241).ln();
        let assign27460_e38243: f64 = (assign27460_e38238 - assign27460_e38242);
        let assign27460_e38246: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign27460_e38247: f64 = (assign27460_e38243 + assign27460_e38246);
        (assign27460_e38247, ((((((locals.var_gammachi_dn0 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign27460_e38237) - (((locals.var_cnst1over_dn0 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn0)) / assign27460_e38241)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign27460_e38237) - (((locals.var_cnst1over_dn2 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn2)) / assign27460_e38241)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn6 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign27460_e38237) - (((locals.var_cnst1over_dn6 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn6)) / assign27460_e38241)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn7 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign27460_e38237) - (((locals.var_cnst1over_dn7 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn7)) / assign27460_e38241)) + (locals.var_beta * locals.var_vxbgmtcl_dn7)), ((((((locals.var_gammachi_dn10 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign27460_e38237) - (((locals.var_cnst1over_dn10 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn10)) / assign27460_e38241)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign27460_e38237) - (((locals.var_cnst1over_dn11 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn11)) / assign27460_e38241)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign27460_e38237) - (((locals.var_cnst1over_dn12 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn12)) / assign27460_e38241)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)), ((((((locals.var_gammachi_dn17 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn17)) + ((locals.var_psi_dn17 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn17))) / assign27460_e38237) - (((locals.var_cnst1over_dn17 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn17)) / assign27460_e38241)) + (locals.var_beta * locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12, locals.var_chi_1_dn17,)
    }
};
        locals.var_chi_1 = assign27460_e38249;
        locals.var_chi_1_dn0 = assign27460_e38249_d_n0;
        locals.var_chi_1_dn2 = assign27460_e38249_d_n2;
        locals.var_chi_1_dn6 = assign27460_e38249_d_n6;
        locals.var_chi_1_dn7 = assign27460_e38249_d_n7;
        locals.var_chi_1_dn10 = assign27460_e38249_d_n10;
        locals.var_chi_1_dn11 = assign27460_e38249_d_n11;
        locals.var_chi_1_dn12 = assign27460_e38249_d_n12;
        locals.var_chi_1_dn17 = assign27460_e38249_d_n17;
        locals.var_chi_1_rv = 0.0;

        let (assign27470_e38264, assign27470_e38264_d_n0, assign27470_e38264_d_n2, assign27470_e38264_d_n6, assign27470_e38264_d_n7, assign27470_e38264_d_n10, assign27470_e38264_d_n11, assign27470_e38264_d_n12, assign27470_e38264_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27470_e38260: f64 = (locals.var_psi - locals.var_chi_1);
        let assign27470_e38262: f64 = (assign27470_e38260 - 1.0);
        (assign27470_e38262, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12), (locals.var_psi_dn17 - locals.var_chi_1_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign27470_e38264;
        locals.var_tmf1_dn0 = assign27470_e38264_d_n0;
        locals.var_tmf1_dn2 = assign27470_e38264_d_n2;
        locals.var_tmf1_dn6 = assign27470_e38264_d_n6;
        locals.var_tmf1_dn7 = assign27470_e38264_d_n7;
        locals.var_tmf1_dn10 = assign27470_e38264_d_n10;
        locals.var_tmf1_dn11 = assign27470_e38264_d_n11;
        locals.var_tmf1_dn12 = assign27470_e38264_d_n12;
        locals.var_tmf1_dn17 = assign27470_e38264_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign27480_e38279, assign27480_e38279_d_n0, assign27480_e38279_d_n2, assign27480_e38279_d_n6, assign27480_e38279_d_n7, assign27480_e38279_d_n10, assign27480_e38279_d_n11, assign27480_e38279_d_n12, assign27480_e38279_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27480_e38275: f64 = (4.0 * locals.var_psi);
        let assign27480_e38277: f64 = assign27480_e38275;
        (assign27480_e38277, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn12), (4.0 * locals.var_psi_dn17),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27480_e38279;
        locals.var_tmf2_dn0 = assign27480_e38279_d_n0;
        locals.var_tmf2_dn2 = assign27480_e38279_d_n2;
        locals.var_tmf2_dn6 = assign27480_e38279_d_n6;
        locals.var_tmf2_dn7 = assign27480_e38279_d_n7;
        locals.var_tmf2_dn10 = assign27480_e38279_d_n10;
        locals.var_tmf2_dn11 = assign27480_e38279_d_n11;
        locals.var_tmf2_dn12 = assign27480_e38279_d_n12;
        locals.var_tmf2_dn17 = assign27480_e38279_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27490_e38296, assign27490_e38296_d_n0, assign27490_e38296_d_n2, assign27490_e38296_d_n6, assign27490_e38296_d_n7, assign27490_e38296_d_n10, assign27490_e38296_d_n11, assign27490_e38296_d_n12, assign27490_e38296_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let (assign27490_e38294, assign27490_e38294_d_n0, assign27490_e38294_d_n2, assign27490_e38294_d_n6, assign27490_e38294_d_n7, assign27490_e38294_d_n10, assign27490_e38294_d_n11, assign27490_e38294_d_n12, assign27490_e38294_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign27490_e38293: f64 = (-locals.var_tmf2);
                (assign27490_e38293, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign27490_e38294, assign27490_e38294_d_n0, assign27490_e38294_d_n2, assign27490_e38294_d_n6, assign27490_e38294_d_n7, assign27490_e38294_d_n10, assign27490_e38294_d_n11, assign27490_e38294_d_n12, assign27490_e38294_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27490_e38296;
        locals.var_tmf2_dn0 = assign27490_e38296_d_n0;
        locals.var_tmf2_dn2 = assign27490_e38296_d_n2;
        locals.var_tmf2_dn6 = assign27490_e38296_d_n6;
        locals.var_tmf2_dn7 = assign27490_e38296_d_n7;
        locals.var_tmf2_dn10 = assign27490_e38296_d_n10;
        locals.var_tmf2_dn11 = assign27490_e38296_d_n11;
        locals.var_tmf2_dn12 = assign27490_e38296_d_n12;
        locals.var_tmf2_dn17 = assign27490_e38296_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27500_e38312, assign27500_e38312_d_n0, assign27500_e38312_d_n2, assign27500_e38312_d_n6, assign27500_e38312_d_n7, assign27500_e38312_d_n10, assign27500_e38312_d_n11, assign27500_e38312_d_n12, assign27500_e38312_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27500_e38307: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign27500_e38309: f64 = (assign27500_e38307 + locals.var_tmf2);
        let assign27500_e38310: f64 = (assign27500_e38309).sqrt();
        (assign27500_e38310, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign27500_e38310)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign27500_e38310)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign27500_e38310)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign27500_e38310)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign27500_e38310)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign27500_e38310)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign27500_e38310)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign27500_e38310)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27500_e38312;
        locals.var_tmf2_dn0 = assign27500_e38312_d_n0;
        locals.var_tmf2_dn2 = assign27500_e38312_d_n2;
        locals.var_tmf2_dn6 = assign27500_e38312_d_n6;
        locals.var_tmf2_dn7 = assign27500_e38312_d_n7;
        locals.var_tmf2_dn10 = assign27500_e38312_d_n10;
        locals.var_tmf2_dn11 = assign27500_e38312_d_n11;
        locals.var_tmf2_dn12 = assign27500_e38312_d_n12;
        locals.var_tmf2_dn17 = assign27500_e38312_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27510_e38329, assign27510_e38329_d_n0, assign27510_e38329_d_n2, assign27510_e38329_d_n6, assign27510_e38329_d_n7, assign27510_e38329_d_n10, assign27510_e38329_d_n11, assign27510_e38329_d_n12, assign27510_e38329_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27510_e38325: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign27510_e38326: f64 = (1.0 + assign27510_e38325);
        let assign27510_e38327: f64 = (0.5 * assign27510_e38326);
        (assign27510_e38327, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk775, locals.var_t1__blk775_dn0, locals.var_t1__blk775_dn2, locals.var_t1__blk775_dn6, locals.var_t1__blk775_dn7, locals.var_t1__blk775_dn10, locals.var_t1__blk775_dn11, locals.var_t1__blk775_dn12, locals.var_t1__blk775_dn17,)
    }
};
        locals.var_t1__blk775 = assign27510_e38329;
        locals.var_t1__blk775_dn0 = assign27510_e38329_d_n0;
        locals.var_t1__blk775_dn2 = assign27510_e38329_d_n2;
        locals.var_t1__blk775_dn6 = assign27510_e38329_d_n6;
        locals.var_t1__blk775_dn7 = assign27510_e38329_d_n7;
        locals.var_t1__blk775_dn10 = assign27510_e38329_d_n10;
        locals.var_t1__blk775_dn11 = assign27510_e38329_d_n11;
        locals.var_t1__blk775_dn12 = assign27510_e38329_d_n12;
        locals.var_t1__blk775_dn17 = assign27510_e38329_d_n17;
        locals.var_t1__blk775_rv = 0.0;

        let (assign27520_e38350, assign27520_e38350_d_n0, assign27520_e38350_d_n2, assign27520_e38350_d_n6, assign27520_e38350_d_n7, assign27520_e38350_d_n10, assign27520_e38350_d_n11, assign27520_e38350_d_n12, assign27520_e38350_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27520_e38343: f64 = 2.0;
        let assign27520_e38344: f64 = (locals.var_tmf1 + assign27520_e38343);
        let assign27520_e38346: f64 = (assign27520_e38344 / locals.var_tmf2);
        let assign27520_e38347: f64 = (1.0 - assign27520_e38346);
        let assign27520_e38348: f64 = (0.5 * assign27520_e38347);
        (assign27520_e38348, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign27520_e38344 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign27520_e38344 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign27520_e38344 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign27520_e38344 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign27520_e38344 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign27520_e38344 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign27520_e38344 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign27520_e38344 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk776, locals.var_t2__blk776_dn0, locals.var_t2__blk776_dn2, locals.var_t2__blk776_dn6, locals.var_t2__blk776_dn7, locals.var_t2__blk776_dn10, locals.var_t2__blk776_dn11, locals.var_t2__blk776_dn12, locals.var_t2__blk776_dn17,)
    }
};
        locals.var_t2__blk776 = assign27520_e38350;
        locals.var_t2__blk776_dn0 = assign27520_e38350_d_n0;
        locals.var_t2__blk776_dn2 = assign27520_e38350_d_n2;
        locals.var_t2__blk776_dn6 = assign27520_e38350_d_n6;
        locals.var_t2__blk776_dn7 = assign27520_e38350_d_n7;
        locals.var_t2__blk776_dn10 = assign27520_e38350_d_n10;
        locals.var_t2__blk776_dn11 = assign27520_e38350_d_n11;
        locals.var_t2__blk776_dn12 = assign27520_e38350_d_n12;
        locals.var_t2__blk776_dn17 = assign27520_e38350_d_n17;
        locals.var_t2__blk776_rv = 0.0;

        let (assign27530_e38367, assign27530_e38367_d_n0, assign27530_e38367_d_n2, assign27530_e38367_d_n6, assign27530_e38367_d_n7, assign27530_e38367_d_n10, assign27530_e38367_d_n11, assign27530_e38367_d_n12, assign27530_e38367_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27530_e38363: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign27530_e38364: f64 = (0.5 * assign27530_e38363);
        let assign27530_e38365: f64 = (locals.var_psi - assign27530_e38364);
        (assign27530_e38365, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psi_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12, locals.var_chi_1_dn17,)
    }
};
        locals.var_chi_1 = assign27530_e38367;
        locals.var_chi_1_dn0 = assign27530_e38367_d_n0;
        locals.var_chi_1_dn2 = assign27530_e38367_d_n2;
        locals.var_chi_1_dn6 = assign27530_e38367_d_n6;
        locals.var_chi_1_dn7 = assign27530_e38367_d_n7;
        locals.var_chi_1_dn10 = assign27530_e38367_d_n10;
        locals.var_chi_1_dn11 = assign27530_e38367_d_n11;
        locals.var_chi_1_dn12 = assign27530_e38367_d_n12;
        locals.var_chi_1_dn17 = assign27530_e38367_d_n17;
        locals.var_chi_1_rv = 0.0;

        let (assign27540_e38380, assign27540_e38380_d_n0, assign27540_e38380_d_n2, assign27540_e38380_d_n6, assign27540_e38380_d_n7, assign27540_e38380_d_n10, assign27540_e38380_d_n11, assign27540_e38380_d_n12, assign27540_e38380_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27540_e38378: f64 = (locals.var_psi - locals.var_chi_1);
        (assign27540_e38378, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12), (locals.var_psi_dn17 - locals.var_chi_1_dn17),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign27540_e38380;
        locals.var_psi_dn0 = assign27540_e38380_d_n0;
        locals.var_psi_dn2 = assign27540_e38380_d_n2;
        locals.var_psi_dn6 = assign27540_e38380_d_n6;
        locals.var_psi_dn7 = assign27540_e38380_d_n7;
        locals.var_psi_dn10 = assign27540_e38380_d_n10;
        locals.var_psi_dn11 = assign27540_e38380_d_n11;
        locals.var_psi_dn12 = assign27540_e38380_d_n12;
        locals.var_psi_dn17 = assign27540_e38380_d_n17;
        locals.var_psi_rv = 0.0;

        let (assign27550_e38395, assign27550_e38395_d_n0, assign27550_e38395_d_n2, assign27550_e38395_d_n6, assign27550_e38395_d_n7, assign27550_e38395_d_n10, assign27550_e38395_d_n11, assign27550_e38395_d_n12, assign27550_e38395_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27550_e38392: f64 = (locals.var_beta * 0.1);
        let assign27550_e38393: f64 = (locals.var_psi + assign27550_e38392);
        (assign27550_e38393, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign27550_e38395;
        locals.var_psi_dn0 = assign27550_e38395_d_n0;
        locals.var_psi_dn2 = assign27550_e38395_d_n2;
        locals.var_psi_dn6 = assign27550_e38395_d_n6;
        locals.var_psi_dn7 = assign27550_e38395_d_n7;
        locals.var_psi_dn10 = assign27550_e38395_d_n10;
        locals.var_psi_dn11 = assign27550_e38395_d_n11;
        locals.var_psi_dn12 = assign27550_e38395_d_n12;
        locals.var_psi_dn17 = assign27550_e38395_d_n17;
        locals.var_psi_rv = 0.0;

        let (assign27560_e38422, assign27560_e38422_d_n0, assign27560_e38422_d_n2, assign27560_e38422_d_n6, assign27560_e38422_d_n7, assign27560_e38422_d_n10, assign27560_e38422_d_n11, assign27560_e38422_d_n12, assign27560_e38422_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27560_e38406: f64 = (locals.var_gammachi * locals.var_t0__blk774);
        let assign27560_e38409: f64 = (locals.var_psi * locals.var_psi);
        let assign27560_e38410: f64 = (assign27560_e38406 + assign27560_e38409);
        let assign27560_e38411: f64 = (assign27560_e38410).ln();
        let assign27560_e38414: f64 = (locals.var_cnst1over * locals.var_t0__blk774);
        let assign27560_e38415: f64 = (assign27560_e38414).ln();
        let assign27560_e38416: f64 = (assign27560_e38411 - assign27560_e38415);
        let assign27560_e38419: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign27560_e38420: f64 = (assign27560_e38416 + assign27560_e38419);
        (assign27560_e38420, ((((((locals.var_gammachi_dn0 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign27560_e38410) - (((locals.var_cnst1over_dn0 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn0)) / assign27560_e38414)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign27560_e38410) - (((locals.var_cnst1over_dn2 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn2)) / assign27560_e38414)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn6 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign27560_e38410) - (((locals.var_cnst1over_dn6 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn6)) / assign27560_e38414)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn7 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign27560_e38410) - (((locals.var_cnst1over_dn7 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn7)) / assign27560_e38414)) + (locals.var_beta * locals.var_vxbgmtcl_dn7)), ((((((locals.var_gammachi_dn10 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign27560_e38410) - (((locals.var_cnst1over_dn10 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn10)) / assign27560_e38414)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign27560_e38410) - (((locals.var_cnst1over_dn11 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn11)) / assign27560_e38414)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign27560_e38410) - (((locals.var_cnst1over_dn12 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn12)) / assign27560_e38414)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)), ((((((locals.var_gammachi_dn17 * locals.var_t0__blk774) + (locals.var_gammachi * locals.var_t0__blk774_dn17)) + ((locals.var_psi_dn17 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn17))) / assign27560_e38410) - (((locals.var_cnst1over_dn17 * locals.var_t0__blk774) + (locals.var_cnst1over * locals.var_t0__blk774_dn17)) / assign27560_e38414)) + (locals.var_beta * locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn12, locals.var_chi_b_dn17,)
    }
};
        locals.var_chi_b = assign27560_e38422;
        locals.var_chi_b_dn0 = assign27560_e38422_d_n0;
        locals.var_chi_b_dn2 = assign27560_e38422_d_n2;
        locals.var_chi_b_dn6 = assign27560_e38422_d_n6;
        locals.var_chi_b_dn7 = assign27560_e38422_d_n7;
        locals.var_chi_b_dn10 = assign27560_e38422_d_n10;
        locals.var_chi_b_dn11 = assign27560_e38422_d_n11;
        locals.var_chi_b_dn12 = assign27560_e38422_d_n12;
        locals.var_chi_b_dn17 = assign27560_e38422_d_n17;
        locals.var_chi_b_rv = 0.0;

        let (assign27570_e38433, assign27570_e38433_d_n0, assign27570_e38433_d_n2, assign27570_e38433_d_n6, assign27570_e38433_d_n7, assign27570_e38433_d_n10, assign27570_e38433_d_n11, assign27570_e38433_d_n12, assign27570_e38433_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        (locals.var_chi__blk818, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn12, locals.var_chi_a_dn17,)
    }
};
        locals.var_chi_a = assign27570_e38433;
        locals.var_chi_a_dn0 = assign27570_e38433_d_n0;
        locals.var_chi_a_dn2 = assign27570_e38433_d_n2;
        locals.var_chi_a_dn6 = assign27570_e38433_d_n6;
        locals.var_chi_a_dn7 = assign27570_e38433_d_n7;
        locals.var_chi_a_dn10 = assign27570_e38433_d_n10;
        locals.var_chi_a_dn11 = assign27570_e38433_d_n11;
        locals.var_chi_a_dn12 = assign27570_e38433_d_n12;
        locals.var_chi_a_dn17 = assign27570_e38433_d_n17;
        locals.var_chi_a_rv = 0.0;

        let (assign27580_e38450, assign27580_e38450_d_n0, assign27580_e38450_d_n2, assign27580_e38450_d_n6, assign27580_e38450_d_n7, assign27580_e38450_d_n10, assign27580_e38450_d_n11, assign27580_e38450_d_n12, assign27580_e38450_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27580_e38444: f64 = (locals.var_chi_b - locals.var_chi_a);
        let assign27580_e38447: f64 = (0.0008 * 75.0);
        let assign27580_e38448: f64 = (assign27580_e38444 - assign27580_e38447);
        (assign27580_e38448, (locals.var_chi_b_dn0 - locals.var_chi_a_dn0), (locals.var_chi_b_dn2 - locals.var_chi_a_dn2), (locals.var_chi_b_dn6 - locals.var_chi_a_dn6), (locals.var_chi_b_dn7 - locals.var_chi_a_dn7), (locals.var_chi_b_dn10 - locals.var_chi_a_dn10), (locals.var_chi_b_dn11 - locals.var_chi_a_dn11), (locals.var_chi_b_dn12 - locals.var_chi_a_dn12), (locals.var_chi_b_dn17 - locals.var_chi_a_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign27580_e38450;
        locals.var_tmf1_dn0 = assign27580_e38450_d_n0;
        locals.var_tmf1_dn2 = assign27580_e38450_d_n2;
        locals.var_tmf1_dn6 = assign27580_e38450_d_n6;
        locals.var_tmf1_dn7 = assign27580_e38450_d_n7;
        locals.var_tmf1_dn10 = assign27580_e38450_d_n10;
        locals.var_tmf1_dn11 = assign27580_e38450_d_n11;
        locals.var_tmf1_dn12 = assign27580_e38450_d_n12;
        locals.var_tmf1_dn17 = assign27580_e38450_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign27590_e38467, assign27590_e38467_d_n0, assign27590_e38467_d_n2, assign27590_e38467_d_n6, assign27590_e38467_d_n7, assign27590_e38467_d_n10, assign27590_e38467_d_n11, assign27590_e38467_d_n12, assign27590_e38467_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27590_e38461: f64 = (4.0 * locals.var_chi_b);
        let assign27590_e38464: f64 = (0.0008 * 75.0);
        let assign27590_e38465: f64 = (assign27590_e38461 * assign27590_e38464);
        (assign27590_e38465, ((4.0 * locals.var_chi_b_dn0) * assign27590_e38464), ((4.0 * locals.var_chi_b_dn2) * assign27590_e38464), ((4.0 * locals.var_chi_b_dn6) * assign27590_e38464), ((4.0 * locals.var_chi_b_dn7) * assign27590_e38464), ((4.0 * locals.var_chi_b_dn10) * assign27590_e38464), ((4.0 * locals.var_chi_b_dn11) * assign27590_e38464), ((4.0 * locals.var_chi_b_dn12) * assign27590_e38464), ((4.0 * locals.var_chi_b_dn17) * assign27590_e38464),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27590_e38467;
        locals.var_tmf2_dn0 = assign27590_e38467_d_n0;
        locals.var_tmf2_dn2 = assign27590_e38467_d_n2;
        locals.var_tmf2_dn6 = assign27590_e38467_d_n6;
        locals.var_tmf2_dn7 = assign27590_e38467_d_n7;
        locals.var_tmf2_dn10 = assign27590_e38467_d_n10;
        locals.var_tmf2_dn11 = assign27590_e38467_d_n11;
        locals.var_tmf2_dn12 = assign27590_e38467_d_n12;
        locals.var_tmf2_dn17 = assign27590_e38467_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27600_e38484, assign27600_e38484_d_n0, assign27600_e38484_d_n2, assign27600_e38484_d_n6, assign27600_e38484_d_n7, assign27600_e38484_d_n10, assign27600_e38484_d_n11, assign27600_e38484_d_n12, assign27600_e38484_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let (assign27600_e38482, assign27600_e38482_d_n0, assign27600_e38482_d_n2, assign27600_e38482_d_n6, assign27600_e38482_d_n7, assign27600_e38482_d_n10, assign27600_e38482_d_n11, assign27600_e38482_d_n12, assign27600_e38482_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign27600_e38481: f64 = (-locals.var_tmf2);
                (assign27600_e38481, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign27600_e38482, assign27600_e38482_d_n0, assign27600_e38482_d_n2, assign27600_e38482_d_n6, assign27600_e38482_d_n7, assign27600_e38482_d_n10, assign27600_e38482_d_n11, assign27600_e38482_d_n12, assign27600_e38482_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27600_e38484;
        locals.var_tmf2_dn0 = assign27600_e38484_d_n0;
        locals.var_tmf2_dn2 = assign27600_e38484_d_n2;
        locals.var_tmf2_dn6 = assign27600_e38484_d_n6;
        locals.var_tmf2_dn7 = assign27600_e38484_d_n7;
        locals.var_tmf2_dn10 = assign27600_e38484_d_n10;
        locals.var_tmf2_dn11 = assign27600_e38484_d_n11;
        locals.var_tmf2_dn12 = assign27600_e38484_d_n12;
        locals.var_tmf2_dn17 = assign27600_e38484_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27610_e38500, assign27610_e38500_d_n0, assign27610_e38500_d_n2, assign27610_e38500_d_n6, assign27610_e38500_d_n7, assign27610_e38500_d_n10, assign27610_e38500_d_n11, assign27610_e38500_d_n12, assign27610_e38500_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27610_e38495: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign27610_e38497: f64 = (assign27610_e38495 + locals.var_tmf2);
        let assign27610_e38498: f64 = (assign27610_e38497).sqrt();
        (assign27610_e38498, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign27610_e38498)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign27610_e38498)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign27610_e38498)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign27610_e38498)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign27610_e38498)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign27610_e38498)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign27610_e38498)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign27610_e38498)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27610_e38500;
        locals.var_tmf2_dn0 = assign27610_e38500_d_n0;
        locals.var_tmf2_dn2 = assign27610_e38500_d_n2;
        locals.var_tmf2_dn6 = assign27610_e38500_d_n6;
        locals.var_tmf2_dn7 = assign27610_e38500_d_n7;
        locals.var_tmf2_dn10 = assign27610_e38500_d_n10;
        locals.var_tmf2_dn11 = assign27610_e38500_d_n11;
        locals.var_tmf2_dn12 = assign27610_e38500_d_n12;
        locals.var_tmf2_dn17 = assign27610_e38500_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27620_e38517, assign27620_e38517_d_n0, assign27620_e38517_d_n2, assign27620_e38517_d_n6, assign27620_e38517_d_n7, assign27620_e38517_d_n10, assign27620_e38517_d_n11, assign27620_e38517_d_n12, assign27620_e38517_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27620_e38513: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign27620_e38514: f64 = (1.0 + assign27620_e38513);
        let assign27620_e38515: f64 = (0.5 * assign27620_e38514);
        (assign27620_e38515, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk775, locals.var_t1__blk775_dn0, locals.var_t1__blk775_dn2, locals.var_t1__blk775_dn6, locals.var_t1__blk775_dn7, locals.var_t1__blk775_dn10, locals.var_t1__blk775_dn11, locals.var_t1__blk775_dn12, locals.var_t1__blk775_dn17,)
    }
};
        locals.var_t1__blk775 = assign27620_e38517;
        locals.var_t1__blk775_dn0 = assign27620_e38517_d_n0;
        locals.var_t1__blk775_dn2 = assign27620_e38517_d_n2;
        locals.var_t1__blk775_dn6 = assign27620_e38517_d_n6;
        locals.var_t1__blk775_dn7 = assign27620_e38517_d_n7;
        locals.var_t1__blk775_dn10 = assign27620_e38517_d_n10;
        locals.var_t1__blk775_dn11 = assign27620_e38517_d_n11;
        locals.var_t1__blk775_dn12 = assign27620_e38517_d_n12;
        locals.var_t1__blk775_dn17 = assign27620_e38517_d_n17;
        locals.var_t1__blk775_rv = 0.0;

        let (assign27630_e38540, assign27630_e38540_d_n0, assign27630_e38540_d_n2, assign27630_e38540_d_n6, assign27630_e38540_d_n7, assign27630_e38540_d_n10, assign27630_e38540_d_n11, assign27630_e38540_d_n12, assign27630_e38540_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27630_e38531: f64 = (2.0 * 0.0008);
        let assign27630_e38533: f64 = (assign27630_e38531 * 75.0);
        let assign27630_e38534: f64 = (locals.var_tmf1 + assign27630_e38533);
        let assign27630_e38536: f64 = (assign27630_e38534 / locals.var_tmf2);
        let assign27630_e38537: f64 = (1.0 - assign27630_e38536);
        let assign27630_e38538: f64 = (0.5 * assign27630_e38537);
        (assign27630_e38538, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign27630_e38534 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign27630_e38534 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign27630_e38534 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign27630_e38534 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign27630_e38534 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign27630_e38534 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign27630_e38534 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign27630_e38534 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk776, locals.var_t2__blk776_dn0, locals.var_t2__blk776_dn2, locals.var_t2__blk776_dn6, locals.var_t2__blk776_dn7, locals.var_t2__blk776_dn10, locals.var_t2__blk776_dn11, locals.var_t2__blk776_dn12, locals.var_t2__blk776_dn17,)
    }
};
        locals.var_t2__blk776 = assign27630_e38540;
        locals.var_t2__blk776_dn0 = assign27630_e38540_d_n0;
        locals.var_t2__blk776_dn2 = assign27630_e38540_d_n2;
        locals.var_t2__blk776_dn6 = assign27630_e38540_d_n6;
        locals.var_t2__blk776_dn7 = assign27630_e38540_d_n7;
        locals.var_t2__blk776_dn10 = assign27630_e38540_d_n10;
        locals.var_t2__blk776_dn11 = assign27630_e38540_d_n11;
        locals.var_t2__blk776_dn12 = assign27630_e38540_d_n12;
        locals.var_t2__blk776_dn17 = assign27630_e38540_d_n17;
        locals.var_t2__blk776_rv = 0.0;

        let (assign27640_e38557, assign27640_e38557_d_n0, assign27640_e38557_d_n2, assign27640_e38557_d_n6, assign27640_e38557_d_n7, assign27640_e38557_d_n10, assign27640_e38557_d_n11, assign27640_e38557_d_n12, assign27640_e38557_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27640_e38553: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign27640_e38554: f64 = (0.5 * assign27640_e38553);
        let assign27640_e38555: f64 = (locals.var_chi_b - assign27640_e38554);
        (assign27640_e38555, (locals.var_chi_b_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_chi_b_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_chi_b_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi__blk818, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    }
};
        locals.var_chi__blk818 = assign27640_e38557;
        locals.var_chi__blk818_dn0 = assign27640_e38557_d_n0;
        locals.var_chi__blk818_dn2 = assign27640_e38557_d_n2;
        locals.var_chi__blk818_dn6 = assign27640_e38557_d_n6;
        locals.var_chi__blk818_dn7 = assign27640_e38557_d_n7;
        locals.var_chi__blk818_dn10 = assign27640_e38557_d_n10;
        locals.var_chi__blk818_dn11 = assign27640_e38557_d_n11;
        locals.var_chi__blk818_dn12 = assign27640_e38557_d_n12;
        locals.var_chi__blk818_dn17 = assign27640_e38557_d_n17;
        locals.var_chi__blk818_rv = 0.0;

        let (assign27650_e38572, assign27650_e38572_d_n0, assign27650_e38572_d_n2, assign27650_e38572_d_n6, assign27650_e38572_d_n7, assign27650_e38572_d_n10, assign27650_e38572_d_n11, assign27650_e38572_d_n12, assign27650_e38572_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27650_e38568: f64 = (locals.var_chi__blk818 / locals.var_beta);
        let assign27650_e38570: f64 = (assign27650_e38568 - locals.var_vxbgmtcl);
        (assign27650_e38570, ((locals.var_chi__blk818_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk818_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk818_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk818_dn7 / locals.var_beta) - locals.var_vxbgmtcl_dn7), ((((locals.var_chi__blk818_dn10 * locals.var_beta) - (locals.var_chi__blk818 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk818_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk818_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk818_dn17 / locals.var_beta) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
        locals.var_ps0ld = assign27650_e38572;
        locals.var_ps0ld_dn0 = assign27650_e38572_d_n0;
        locals.var_ps0ld_dn2 = assign27650_e38572_d_n2;
        locals.var_ps0ld_dn6 = assign27650_e38572_d_n6;
        locals.var_ps0ld_dn7 = assign27650_e38572_d_n7;
        locals.var_ps0ld_dn10 = assign27650_e38572_d_n10;
        locals.var_ps0ld_dn11 = assign27650_e38572_d_n11;
        locals.var_ps0ld_dn12 = assign27650_e38572_d_n12;
        locals.var_ps0ld_dn17 = assign27650_e38572_d_n17;
        locals.var_ps0ld_rv = 0.0;

        let (assign27660_e38589, assign27660_e38589_d_n0, assign27660_e38589_d_n2, assign27660_e38589_d_n6, assign27660_e38589_d_n7, assign27660_e38589_d_n10, assign27660_e38589_d_n11, assign27660_e38589_d_n12, assign27660_e38589_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27660_e38583: f64 = (locals.var_chi__blk818 - 1.0);
        let assign27660_e38585: f64 = (-locals.var_chi__blk818);
        let assign27660_e38586: f64 = (assign27660_e38585).exp();
        let assign27660_e38587: f64 = (assign27660_e38583 + assign27660_e38586);
        (assign27660_e38587, (locals.var_chi__blk818_dn0 + (assign27660_e38586 * (-locals.var_chi__blk818_dn0))), (locals.var_chi__blk818_dn2 + (assign27660_e38586 * (-locals.var_chi__blk818_dn2))), (locals.var_chi__blk818_dn6 + (assign27660_e38586 * (-locals.var_chi__blk818_dn6))), (locals.var_chi__blk818_dn7 + (assign27660_e38586 * (-locals.var_chi__blk818_dn7))), (locals.var_chi__blk818_dn10 + (assign27660_e38586 * (-locals.var_chi__blk818_dn10))), (locals.var_chi__blk818_dn11 + (assign27660_e38586 * (-locals.var_chi__blk818_dn11))), (locals.var_chi__blk818_dn12 + (assign27660_e38586 * (-locals.var_chi__blk818_dn12))), (locals.var_chi__blk818_dn17 + (assign27660_e38586 * (-locals.var_chi__blk818_dn17))),)
    } else {
        (locals.var_t1__blk775, locals.var_t1__blk775_dn0, locals.var_t1__blk775_dn2, locals.var_t1__blk775_dn6, locals.var_t1__blk775_dn7, locals.var_t1__blk775_dn10, locals.var_t1__blk775_dn11, locals.var_t1__blk775_dn12, locals.var_t1__blk775_dn17,)
    }
};
        locals.var_t1__blk775 = assign27660_e38589;
        locals.var_t1__blk775_dn0 = assign27660_e38589_d_n0;
        locals.var_t1__blk775_dn2 = assign27660_e38589_d_n2;
        locals.var_t1__blk775_dn6 = assign27660_e38589_d_n6;
        locals.var_t1__blk775_dn7 = assign27660_e38589_d_n7;
        locals.var_t1__blk775_dn10 = assign27660_e38589_d_n10;
        locals.var_t1__blk775_dn11 = assign27660_e38589_d_n11;
        locals.var_t1__blk775_dn12 = assign27660_e38589_d_n12;
        locals.var_t1__blk775_dn17 = assign27660_e38589_d_n17;
        locals.var_t1__blk775_rv = 0.0;

        let assign27670_e38593: f64 = (10.0 * 2.220446049250313e-16);
        let assign27670_e38594: f64 = if locals.var_t1__blk775 < assign27670_e38593 { 1.0 } else { 0.0 };
        locals.var_guard880 = assign27670_e38594;
        locals.var_guard880_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_99(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27680_e38609, assign27680_e38609_d_n0, assign27680_e38609_d_n2, assign27680_e38609_d_n6, assign27680_e38609_d_n7, assign27680_e38609_d_n10, assign27680_e38609_d_n11, assign27680_e38609_d_n12, assign27680_e38609_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard880 != 0.0)) {
        let assign27680_e38607: f64 = (10.0 * 2.220446049250313e-16);
        (assign27680_e38607, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk775, locals.var_t1__blk775_dn0, locals.var_t1__blk775_dn2, locals.var_t1__blk775_dn6, locals.var_t1__blk775_dn7, locals.var_t1__blk775_dn10, locals.var_t1__blk775_dn11, locals.var_t1__blk775_dn12, locals.var_t1__blk775_dn17,)
    }
};
        locals.var_t1__blk775 = assign27680_e38609;
        locals.var_t1__blk775_dn0 = assign27680_e38609_d_n0;
        locals.var_t1__blk775_dn2 = assign27680_e38609_d_n2;
        locals.var_t1__blk775_dn6 = assign27680_e38609_d_n6;
        locals.var_t1__blk775_dn7 = assign27680_e38609_d_n7;
        locals.var_t1__blk775_dn10 = assign27680_e38609_d_n10;
        locals.var_t1__blk775_dn11 = assign27680_e38609_d_n11;
        locals.var_t1__blk775_dn12 = assign27680_e38609_d_n12;
        locals.var_t1__blk775_dn17 = assign27680_e38609_d_n17;
        locals.var_t1__blk775_rv = 0.0;

        let (assign27690_e38621, assign27690_e38621_d_n0, assign27690_e38621_d_n2, assign27690_e38621_d_n6, assign27690_e38621_d_n7, assign27690_e38621_d_n10, assign27690_e38621_d_n11, assign27690_e38621_d_n12, assign27690_e38621_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27690_e38619: f64 = (locals.var_t1__blk775).sqrt();
        (assign27690_e38619, (locals.var_t1__blk775_dn0 / (2.0 * assign27690_e38619)), (locals.var_t1__blk775_dn2 / (2.0 * assign27690_e38619)), (locals.var_t1__blk775_dn6 / (2.0 * assign27690_e38619)), (locals.var_t1__blk775_dn7 / (2.0 * assign27690_e38619)), (locals.var_t1__blk775_dn10 / (2.0 * assign27690_e38619)), (locals.var_t1__blk775_dn11 / (2.0 * assign27690_e38619)), (locals.var_t1__blk775_dn12 / (2.0 * assign27690_e38619)), (locals.var_t1__blk775_dn17 / (2.0 * assign27690_e38619)),)
    } else {
        (locals.var_t2__blk776, locals.var_t2__blk776_dn0, locals.var_t2__blk776_dn2, locals.var_t2__blk776_dn6, locals.var_t2__blk776_dn7, locals.var_t2__blk776_dn10, locals.var_t2__blk776_dn11, locals.var_t2__blk776_dn12, locals.var_t2__blk776_dn17,)
    }
};
        locals.var_t2__blk776 = assign27690_e38621;
        locals.var_t2__blk776_dn0 = assign27690_e38621_d_n0;
        locals.var_t2__blk776_dn2 = assign27690_e38621_d_n2;
        locals.var_t2__blk776_dn6 = assign27690_e38621_d_n6;
        locals.var_t2__blk776_dn7 = assign27690_e38621_d_n7;
        locals.var_t2__blk776_dn10 = assign27690_e38621_d_n10;
        locals.var_t2__blk776_dn11 = assign27690_e38621_d_n11;
        locals.var_t2__blk776_dn12 = assign27690_e38621_d_n12;
        locals.var_t2__blk776_dn17 = assign27690_e38621_d_n17;
        locals.var_t2__blk776_rv = 0.0;

        let (assign27700_e38634, assign27700_e38634_d_n0, assign27700_e38634_d_n2, assign27700_e38634_d_n6, assign27700_e38634_d_n7, assign27700_e38634_d_n10, assign27700_e38634_d_n11, assign27700_e38634_d_n12, assign27700_e38634_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27700_e38632: f64 = (locals.var_cnst0over * locals.var_t2__blk776);
        (assign27700_e38632, ((locals.var_cnst0over_dn0 * locals.var_t2__blk776) + (locals.var_cnst0over * locals.var_t2__blk776_dn0)), ((locals.var_cnst0over_dn2 * locals.var_t2__blk776) + (locals.var_cnst0over * locals.var_t2__blk776_dn2)), ((locals.var_cnst0over_dn6 * locals.var_t2__blk776) + (locals.var_cnst0over * locals.var_t2__blk776_dn6)), ((locals.var_cnst0over_dn7 * locals.var_t2__blk776) + (locals.var_cnst0over * locals.var_t2__blk776_dn7)), ((locals.var_cnst0over_dn10 * locals.var_t2__blk776) + (locals.var_cnst0over * locals.var_t2__blk776_dn10)), ((locals.var_cnst0over_dn11 * locals.var_t2__blk776) + (locals.var_cnst0over * locals.var_t2__blk776_dn11)), ((locals.var_cnst0over_dn12 * locals.var_t2__blk776) + (locals.var_cnst0over * locals.var_t2__blk776_dn12)), ((locals.var_cnst0over_dn17 * locals.var_t2__blk776) + (locals.var_cnst0over * locals.var_t2__blk776_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign27700_e38634;
        locals.var_qbuld_dn0 = assign27700_e38634_d_n0;
        locals.var_qbuld_dn2 = assign27700_e38634_d_n2;
        locals.var_qbuld_dn6 = assign27700_e38634_d_n6;
        locals.var_qbuld_dn7 = assign27700_e38634_d_n7;
        locals.var_qbuld_dn10 = assign27700_e38634_d_n10;
        locals.var_qbuld_dn11 = assign27700_e38634_d_n11;
        locals.var_qbuld_dn12 = assign27700_e38634_d_n12;
        locals.var_qbuld_dn17 = assign27700_e38634_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign27710_e38649, assign27710_e38649_d_n0, assign27710_e38649_d_n2, assign27710_e38649_d_n6, assign27710_e38649_d_n7, assign27710_e38649_d_n10, assign27710_e38649_d_n11, assign27710_e38649_d_n12, assign27710_e38649_d_n17,) = {
    if ((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) {
        let assign27710_e38646: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign27710_e38647: f64 = (locals.var_cox0 * assign27710_e38646);
        (assign27710_e38647, (locals.var_cox0 * (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0)), (locals.var_cox0 * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0 * (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6)), (locals.var_cox0 * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0 * (locals.var_vgpld_dn10 - locals.var_ps0ld_dn10)), (locals.var_cox0 * (locals.var_vgpld_dn11 - locals.var_ps0ld_dn11)), (locals.var_cox0 * (locals.var_vgpld_dn12 - locals.var_ps0ld_dn12)), (locals.var_cox0 * (locals.var_vgpld_dn17 - locals.var_ps0ld_dn17)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign27710_e38649;
        locals.var_qsuld_dn0 = assign27710_e38649_d_n0;
        locals.var_qsuld_dn2 = assign27710_e38649_d_n2;
        locals.var_qsuld_dn6 = assign27710_e38649_d_n6;
        locals.var_qsuld_dn7 = assign27710_e38649_d_n7;
        locals.var_qsuld_dn10 = assign27710_e38649_d_n10;
        locals.var_qsuld_dn11 = assign27710_e38649_d_n11;
        locals.var_qsuld_dn12 = assign27710_e38649_d_n12;
        locals.var_qsuld_dn17 = assign27710_e38649_d_n17;
        locals.var_qsuld_rv = 0.0;

        let assign27720_e38652: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard881 = assign27720_e38652;
        locals.var_guard881_rv = 0.0;

        let (assign27730_e38669, assign27730_e38669_d_n0, assign27730_e38669_d_n2, assign27730_e38669_d_n6, assign27730_e38669_d_n7, assign27730_e38669_d_n10, assign27730_e38669_d_n11, assign27730_e38669_d_n12, assign27730_e38669_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        let assign27730_e38665: f64 = (-locals.var_vxbgmtcl);
        let assign27730_e38666: f64 = (locals.var_beta * assign27730_e38665);
        let assign27730_e38667: f64 = (assign27730_e38666).exp();
        (assign27730_e38667, (assign27730_e38667 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign27730_e38667 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign27730_e38667 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign27730_e38667 * (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), (assign27730_e38667 * ((locals.var_beta_dn10 * assign27730_e38665) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign27730_e38667 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign27730_e38667 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))), (assign27730_e38667 * (locals.var_beta * (-locals.var_vxbgmtcl_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk837, locals.var_exp_bvbs__blk837_dn0, locals.var_exp_bvbs__blk837_dn2, locals.var_exp_bvbs__blk837_dn6, locals.var_exp_bvbs__blk837_dn7, locals.var_exp_bvbs__blk837_dn10, locals.var_exp_bvbs__blk837_dn11, locals.var_exp_bvbs__blk837_dn12, locals.var_exp_bvbs__blk837_dn17,)
    }
};
        locals.var_exp_bvbs__blk837 = assign27730_e38669;
        locals.var_exp_bvbs__blk837_dn0 = assign27730_e38669_d_n0;
        locals.var_exp_bvbs__blk837_dn2 = assign27730_e38669_d_n2;
        locals.var_exp_bvbs__blk837_dn6 = assign27730_e38669_d_n6;
        locals.var_exp_bvbs__blk837_dn7 = assign27730_e38669_d_n7;
        locals.var_exp_bvbs__blk837_dn10 = assign27730_e38669_d_n10;
        locals.var_exp_bvbs__blk837_dn11 = assign27730_e38669_d_n11;
        locals.var_exp_bvbs__blk837_dn12 = assign27730_e38669_d_n12;
        locals.var_exp_bvbs__blk837_dn17 = assign27730_e38669_d_n17;
        locals.var_exp_bvbs__blk837_rv = 0.0;

        let (assign27740_e38684, assign27740_e38684_d_n0, assign27740_e38684_d_n2, assign27740_e38684_d_n6, assign27740_e38684_d_n7, assign27740_e38684_d_n10, assign27740_e38684_d_n11, assign27740_e38684_d_n12, assign27740_e38684_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        let assign27740_e38682: f64 = (locals.var_nin / locals.var_uc_nsubbttub);
        (assign27740_e38682, (((locals.var_nin_dn0 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn0)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn2 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn2)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn6 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn6)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn7 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn7)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn10 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn10)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn11 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn11)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn12 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn12)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn17 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn17)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)),)
    } else {
        (locals.var_t0__blk774, locals.var_t0__blk774_dn0, locals.var_t0__blk774_dn2, locals.var_t0__blk774_dn6, locals.var_t0__blk774_dn7, locals.var_t0__blk774_dn10, locals.var_t0__blk774_dn11, locals.var_t0__blk774_dn12, locals.var_t0__blk774_dn17,)
    }
};
        locals.var_t0__blk774 = assign27740_e38684;
        locals.var_t0__blk774_dn0 = assign27740_e38684_d_n0;
        locals.var_t0__blk774_dn2 = assign27740_e38684_d_n2;
        locals.var_t0__blk774_dn6 = assign27740_e38684_d_n6;
        locals.var_t0__blk774_dn7 = assign27740_e38684_d_n7;
        locals.var_t0__blk774_dn10 = assign27740_e38684_d_n10;
        locals.var_t0__blk774_dn11 = assign27740_e38684_d_n11;
        locals.var_t0__blk774_dn12 = assign27740_e38684_d_n12;
        locals.var_t0__blk774_dn17 = assign27740_e38684_d_n17;
        locals.var_t0__blk774_rv = 0.0;

        let (assign27750_e38699, assign27750_e38699_d_n0, assign27750_e38699_d_n2, assign27750_e38699_d_n6, assign27750_e38699_d_n7, assign27750_e38699_d_n10, assign27750_e38699_d_n11, assign27750_e38699_d_n12, assign27750_e38699_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        let assign27750_e38697: f64 = (locals.var_t0__blk774 * locals.var_t0__blk774);
        (assign27750_e38697, ((locals.var_t0__blk774_dn0 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn0)), ((locals.var_t0__blk774_dn2 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn2)), ((locals.var_t0__blk774_dn6 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn6)), ((locals.var_t0__blk774_dn7 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn7)), ((locals.var_t0__blk774_dn10 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn10)), ((locals.var_t0__blk774_dn11 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn11)), ((locals.var_t0__blk774_dn12 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn12)), ((locals.var_t0__blk774_dn17 * locals.var_t0__blk774) + (locals.var_t0__blk774 * locals.var_t0__blk774_dn17)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12, locals.var_cnst1over_dn17,)
    }
};
        locals.var_cnst1over = assign27750_e38699;
        locals.var_cnst1over_dn0 = assign27750_e38699_d_n0;
        locals.var_cnst1over_dn2 = assign27750_e38699_d_n2;
        locals.var_cnst1over_dn6 = assign27750_e38699_d_n6;
        locals.var_cnst1over_dn7 = assign27750_e38699_d_n7;
        locals.var_cnst1over_dn10 = assign27750_e38699_d_n10;
        locals.var_cnst1over_dn11 = assign27750_e38699_d_n11;
        locals.var_cnst1over_dn12 = assign27750_e38699_d_n12;
        locals.var_cnst1over_dn17 = assign27750_e38699_d_n17;
        locals.var_cnst1over_rv = 0.0;

        let (assign27760_e38714, assign27760_e38714_d_n0, assign27760_e38714_d_n2, assign27760_e38714_d_n6, assign27760_e38714_d_n7, assign27760_e38714_d_n10, assign27760_e38714_d_n11, assign27760_e38714_d_n12, assign27760_e38714_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        let assign27760_e38712: f64 = (locals.var_cnst1over * locals.var_exp_bvbs__blk837);
        (assign27760_e38712, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn2)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn7)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn12)), ((locals.var_cnst1over_dn17 * locals.var_exp_bvbs__blk837) + (locals.var_cnst1over * locals.var_exp_bvbs__blk837_dn17)),)
    } else {
        (locals.var_cfs1__blk846, locals.var_cfs1__blk846_dn0, locals.var_cfs1__blk846_dn2, locals.var_cfs1__blk846_dn6, locals.var_cfs1__blk846_dn7, locals.var_cfs1__blk846_dn10, locals.var_cfs1__blk846_dn11, locals.var_cfs1__blk846_dn12, locals.var_cfs1__blk846_dn17,)
    }
};
        locals.var_cfs1__blk846 = assign27760_e38714;
        locals.var_cfs1__blk846_dn0 = assign27760_e38714_d_n0;
        locals.var_cfs1__blk846_dn2 = assign27760_e38714_d_n2;
        locals.var_cfs1__blk846_dn6 = assign27760_e38714_d_n6;
        locals.var_cfs1__blk846_dn7 = assign27760_e38714_d_n7;
        locals.var_cfs1__blk846_dn10 = assign27760_e38714_d_n10;
        locals.var_cfs1__blk846_dn11 = assign27760_e38714_d_n11;
        locals.var_cfs1__blk846_dn12 = assign27760_e38714_d_n12;
        locals.var_cfs1__blk846_dn17 = assign27760_e38714_d_n17;
        locals.var_cfs1__blk846_rv = 0.0;

        let (assign27770_e38727,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv__blk791,)
    }
};
        locals.var_flg_conv__blk791 = assign27770_e38727;
        locals.var_flg_conv__blk791_rv = 0.0;

        let (assign27780_e38740,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign27780_e38740;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_100(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign27790_loop_guard: usize = 0;
        while {
            let assign27790_cond_e38754: f64 = (2.0 * 20.0);
            let assign27790_cond_e38756: f64 = (assign27790_cond_e38754 + 1.0);
            let assign27790_cond_e38758: f64 = if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_lp_s0 <= assign27790_cond_e38756)) { 1.0 } else { 0.0 };
            assign27790_cond_e38758 != 0.0
        } {
            assign27790_loop_guard += 1;
            assert!(assign27790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign27790_body0_e38771, assign27790_body0_e38771_d_n0, assign27790_body0_e38771_d_n2, assign27790_body0_e38771_d_n6, assign27790_body0_e38771_d_n7, assign27790_body0_e38771_d_n10, assign27790_body0_e38771_d_n11, assign27790_body0_e38771_d_n12, assign27790_body0_e38771_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb__blk842, locals.var_fb__blk842_dn0, locals.var_fb__blk842_dn2, locals.var_fb__blk842_dn6, locals.var_fb__blk842_dn7, locals.var_fb__blk842_dn10, locals.var_fb__blk842_dn11, locals.var_fb__blk842_dn12, locals.var_fb__blk842_dn17,)
    }
};
            locals.var_fb__blk842 = assign27790_body0_e38771;
            locals.var_fb__blk842_dn0 = assign27790_body0_e38771_d_n0;
            locals.var_fb__blk842_dn2 = assign27790_body0_e38771_d_n2;
            locals.var_fb__blk842_dn6 = assign27790_body0_e38771_d_n6;
            locals.var_fb__blk842_dn7 = assign27790_body0_e38771_d_n7;
            locals.var_fb__blk842_dn10 = assign27790_body0_e38771_d_n10;
            locals.var_fb__blk842_dn11 = assign27790_body0_e38771_d_n11;
            locals.var_fb__blk842_dn12 = assign27790_body0_e38771_d_n12;
            locals.var_fb__blk842_dn17 = assign27790_body0_e38771_d_n17;
            locals.var_fb__blk842_rv = 0.0;
            let (assign27790_body1_e38788, assign27790_body1_e38788_d_n0, assign27790_body1_e38788_d_n2, assign27790_body1_e38788_d_n6, assign27790_body1_e38788_d_n7, assign27790_body1_e38788_d_n10, assign27790_body1_e38788_d_n11, assign27790_body1_e38788_d_n12, assign27790_body1_e38788_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        let assign27790_body1_e38785: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        let assign27790_body1_e38786: f64 = (locals.var_beta * assign27790_body1_e38785);
        (assign27790_body1_e38786, (locals.var_beta * (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign27790_body1_e38785) + (locals.var_beta * (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0ld_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0ld_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk818, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    }
};
            locals.var_chi__blk818 = assign27790_body1_e38788;
            locals.var_chi__blk818_dn0 = assign27790_body1_e38788_d_n0;
            locals.var_chi__blk818_dn2 = assign27790_body1_e38788_d_n2;
            locals.var_chi__blk818_dn6 = assign27790_body1_e38788_d_n6;
            locals.var_chi__blk818_dn7 = assign27790_body1_e38788_d_n7;
            locals.var_chi__blk818_dn10 = assign27790_body1_e38788_d_n10;
            locals.var_chi__blk818_dn11 = assign27790_body1_e38788_d_n11;
            locals.var_chi__blk818_dn12 = assign27790_body1_e38788_d_n12;
            locals.var_chi__blk818_dn17 = assign27790_body1_e38788_d_n17;
            locals.var_chi__blk818_rv = 0.0;
            let assign27790_body2_e38791: f64 = if locals.var_chi__blk818 < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard882 = assign27790_body2_e38791;
            locals.var_guard882_rv = 0.0;
            let (assign27790_body3_e38821, assign27790_body3_e38821_d_n0, assign27790_body3_e38821_d_n2, assign27790_body3_e38821_d_n6, assign27790_body3_e38821_d_n7, assign27790_body3_e38821_d_n10, assign27790_body3_e38821_d_n11, assign27790_body3_e38821_d_n12, assign27790_body3_e38821_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard882 != 0.0)) {
        let assign27790_body3_e38806: f64 = (locals.var_chi__blk818 * locals.var_chi__blk818);
        let assign27790_body3_e38808: f64 = (assign27790_body3_e38806 * locals.var_chi__blk818);
        let assign27790_body3_e38812: f64 = (-0.07053654284009761);
        let assign27790_body3_e38815: f64 = (locals.var_chi__blk818 * 0.006115288895133179);
        let assign27790_body3_e38816: f64 = (assign27790_body3_e38812 + assign27790_body3_e38815);
        let assign27790_body3_e38817: f64 = (locals.var_chi__blk818 * assign27790_body3_e38816);
        let assign27790_body3_e38818: f64 = (0.29693154855771 + assign27790_body3_e38817);
        let assign27790_body3_e38819: f64 = (assign27790_body3_e38808 * assign27790_body3_e38818);
        (assign27790_body3_e38819, ((((((locals.var_chi__blk818_dn0 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn0)) * locals.var_chi__blk818) + (assign27790_body3_e38806 * locals.var_chi__blk818_dn0)) * assign27790_body3_e38818) + (assign27790_body3_e38808 * ((locals.var_chi__blk818_dn0 * assign27790_body3_e38816) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn0 * 0.006115288895133179))))), ((((((locals.var_chi__blk818_dn2 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn2)) * locals.var_chi__blk818) + (assign27790_body3_e38806 * locals.var_chi__blk818_dn2)) * assign27790_body3_e38818) + (assign27790_body3_e38808 * ((locals.var_chi__blk818_dn2 * assign27790_body3_e38816) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn2 * 0.006115288895133179))))), ((((((locals.var_chi__blk818_dn6 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn6)) * locals.var_chi__blk818) + (assign27790_body3_e38806 * locals.var_chi__blk818_dn6)) * assign27790_body3_e38818) + (assign27790_body3_e38808 * ((locals.var_chi__blk818_dn6 * assign27790_body3_e38816) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn6 * 0.006115288895133179))))), ((((((locals.var_chi__blk818_dn7 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn7)) * locals.var_chi__blk818) + (assign27790_body3_e38806 * locals.var_chi__blk818_dn7)) * assign27790_body3_e38818) + (assign27790_body3_e38808 * ((locals.var_chi__blk818_dn7 * assign27790_body3_e38816) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn7 * 0.006115288895133179))))), ((((((locals.var_chi__blk818_dn10 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn10)) * locals.var_chi__blk818) + (assign27790_body3_e38806 * locals.var_chi__blk818_dn10)) * assign27790_body3_e38818) + (assign27790_body3_e38808 * ((locals.var_chi__blk818_dn10 * assign27790_body3_e38816) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn10 * 0.006115288895133179))))), ((((((locals.var_chi__blk818_dn11 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn11)) * locals.var_chi__blk818) + (assign27790_body3_e38806 * locals.var_chi__blk818_dn11)) * assign27790_body3_e38818) + (assign27790_body3_e38808 * ((locals.var_chi__blk818_dn11 * assign27790_body3_e38816) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn11 * 0.006115288895133179))))), ((((((locals.var_chi__blk818_dn12 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn12)) * locals.var_chi__blk818) + (assign27790_body3_e38806 * locals.var_chi__blk818_dn12)) * assign27790_body3_e38818) + (assign27790_body3_e38808 * ((locals.var_chi__blk818_dn12 * assign27790_body3_e38816) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn12 * 0.006115288895133179))))), ((((((locals.var_chi__blk818_dn17 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn17)) * locals.var_chi__blk818) + (assign27790_body3_e38806 * locals.var_chi__blk818_dn17)) * assign27790_body3_e38818) + (assign27790_body3_e38808 * ((locals.var_chi__blk818_dn17 * assign27790_body3_e38816) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn17 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi, locals.var_fi_dn0, locals.var_fi_dn2, locals.var_fi_dn6, locals.var_fi_dn7, locals.var_fi_dn10, locals.var_fi_dn11, locals.var_fi_dn12, locals.var_fi_dn17,)
    }
};
            locals.var_fi = assign27790_body3_e38821;
            locals.var_fi_dn0 = assign27790_body3_e38821_d_n0;
            locals.var_fi_dn2 = assign27790_body3_e38821_d_n2;
            locals.var_fi_dn6 = assign27790_body3_e38821_d_n6;
            locals.var_fi_dn7 = assign27790_body3_e38821_d_n7;
            locals.var_fi_dn10 = assign27790_body3_e38821_d_n10;
            locals.var_fi_dn11 = assign27790_body3_e38821_d_n11;
            locals.var_fi_dn12 = assign27790_body3_e38821_d_n12;
            locals.var_fi_dn17 = assign27790_body3_e38821_d_n17;
            locals.var_fi_rv = 0.0;
            let (assign27790_body4_e38855, assign27790_body4_e38855_d_n0, assign27790_body4_e38855_d_n2, assign27790_body4_e38855_d_n6, assign27790_body4_e38855_d_n7, assign27790_body4_e38855_d_n10, assign27790_body4_e38855_d_n11, assign27790_body4_e38855_d_n12, assign27790_body4_e38855_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard882 != 0.0)) {
        let assign27790_body4_e38836: f64 = (locals.var_chi__blk818 * locals.var_chi__blk818);
        let assign27790_body4_e38839: f64 = (3.0 * 0.29693154855771);
        let assign27790_body4_e38843: f64 = (-0.07053654284009761);
        let assign27790_body4_e38844: f64 = (4.0 * assign27790_body4_e38843);
        let assign27790_body4_e38847: f64 = (locals.var_chi__blk818 * 5.0);
        let assign27790_body4_e38849: f64 = (assign27790_body4_e38847 * 0.006115288895133179);
        let assign27790_body4_e38850: f64 = (assign27790_body4_e38844 + assign27790_body4_e38849);
        let assign27790_body4_e38851: f64 = (locals.var_chi__blk818 * assign27790_body4_e38850);
        let assign27790_body4_e38852: f64 = (assign27790_body4_e38839 + assign27790_body4_e38851);
        let assign27790_body4_e38853: f64 = (assign27790_body4_e38836 * assign27790_body4_e38852);
        (assign27790_body4_e38853, ((((locals.var_chi__blk818_dn0 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn0)) * assign27790_body4_e38852) + (assign27790_body4_e38836 * ((locals.var_chi__blk818_dn0 * assign27790_body4_e38850) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk818_dn2 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn2)) * assign27790_body4_e38852) + (assign27790_body4_e38836 * ((locals.var_chi__blk818_dn2 * assign27790_body4_e38850) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk818_dn6 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn6)) * assign27790_body4_e38852) + (assign27790_body4_e38836 * ((locals.var_chi__blk818_dn6 * assign27790_body4_e38850) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk818_dn7 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn7)) * assign27790_body4_e38852) + (assign27790_body4_e38836 * ((locals.var_chi__blk818_dn7 * assign27790_body4_e38850) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk818_dn10 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn10)) * assign27790_body4_e38852) + (assign27790_body4_e38836 * ((locals.var_chi__blk818_dn10 * assign27790_body4_e38850) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk818_dn11 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn11)) * assign27790_body4_e38852) + (assign27790_body4_e38836 * ((locals.var_chi__blk818_dn11 * assign27790_body4_e38850) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk818_dn12 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn12)) * assign27790_body4_e38852) + (assign27790_body4_e38836 * ((locals.var_chi__blk818_dn12 * assign27790_body4_e38850) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn12 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk818_dn17 * locals.var_chi__blk818) + (locals.var_chi__blk818 * locals.var_chi__blk818_dn17)) * assign27790_body4_e38852) + (assign27790_body4_e38836 * ((locals.var_chi__blk818_dn17 * assign27790_body4_e38850) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn17 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi, locals.var_fi_dchi_dn0, locals.var_fi_dchi_dn2, locals.var_fi_dchi_dn6, locals.var_fi_dchi_dn7, locals.var_fi_dchi_dn10, locals.var_fi_dchi_dn11, locals.var_fi_dchi_dn12, locals.var_fi_dchi_dn17,)
    }
};
            locals.var_fi_dchi = assign27790_body4_e38855;
            locals.var_fi_dchi_dn0 = assign27790_body4_e38855_d_n0;
            locals.var_fi_dchi_dn2 = assign27790_body4_e38855_d_n2;
            locals.var_fi_dchi_dn6 = assign27790_body4_e38855_d_n6;
            locals.var_fi_dchi_dn7 = assign27790_body4_e38855_d_n7;
            locals.var_fi_dchi_dn10 = assign27790_body4_e38855_d_n10;
            locals.var_fi_dchi_dn11 = assign27790_body4_e38855_d_n11;
            locals.var_fi_dchi_dn12 = assign27790_body4_e38855_d_n12;
            locals.var_fi_dchi_dn17 = assign27790_body4_e38855_d_n17;
            locals.var_fi_dchi_rv = 0.0;
            let (assign27790_body5_e38874, assign27790_body5_e38874_d_n0, assign27790_body5_e38874_d_n2, assign27790_body5_e38874_d_n6, assign27790_body5_e38874_d_n7, assign27790_body5_e38874_d_n10, assign27790_body5_e38874_d_n11, assign27790_body5_e38874_d_n12, assign27790_body5_e38874_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard882 != 0.0)) {
        let assign27790_body5_e38870: f64 = (locals.var_cfs1__blk846 * locals.var_fi);
        let assign27790_body5_e38872: f64 = (assign27790_body5_e38870 * locals.var_fi);
        (assign27790_body5_e38872, ((((locals.var_cfs1__blk846_dn0 * locals.var_fi) + (locals.var_cfs1__blk846 * locals.var_fi_dn0)) * locals.var_fi) + (assign27790_body5_e38870 * locals.var_fi_dn0)), ((((locals.var_cfs1__blk846_dn2 * locals.var_fi) + (locals.var_cfs1__blk846 * locals.var_fi_dn2)) * locals.var_fi) + (assign27790_body5_e38870 * locals.var_fi_dn2)), ((((locals.var_cfs1__blk846_dn6 * locals.var_fi) + (locals.var_cfs1__blk846 * locals.var_fi_dn6)) * locals.var_fi) + (assign27790_body5_e38870 * locals.var_fi_dn6)), ((((locals.var_cfs1__blk846_dn7 * locals.var_fi) + (locals.var_cfs1__blk846 * locals.var_fi_dn7)) * locals.var_fi) + (assign27790_body5_e38870 * locals.var_fi_dn7)), ((((locals.var_cfs1__blk846_dn10 * locals.var_fi) + (locals.var_cfs1__blk846 * locals.var_fi_dn10)) * locals.var_fi) + (assign27790_body5_e38870 * locals.var_fi_dn10)), ((((locals.var_cfs1__blk846_dn11 * locals.var_fi) + (locals.var_cfs1__blk846 * locals.var_fi_dn11)) * locals.var_fi) + (assign27790_body5_e38870 * locals.var_fi_dn11)), ((((locals.var_cfs1__blk846_dn12 * locals.var_fi) + (locals.var_cfs1__blk846 * locals.var_fi_dn12)) * locals.var_fi) + (assign27790_body5_e38870 * locals.var_fi_dn12)), ((((locals.var_cfs1__blk846_dn17 * locals.var_fi) + (locals.var_cfs1__blk846 * locals.var_fi_dn17)) * locals.var_fi) + (assign27790_body5_e38870 * locals.var_fi_dn17)),)
    } else {
        (locals.var_fs01__blk840, locals.var_fs01__blk840_dn0, locals.var_fs01__blk840_dn2, locals.var_fs01__blk840_dn6, locals.var_fs01__blk840_dn7, locals.var_fs01__blk840_dn10, locals.var_fs01__blk840_dn11, locals.var_fs01__blk840_dn12, locals.var_fs01__blk840_dn17,)
    }
};
            locals.var_fs01__blk840 = assign27790_body5_e38874;
            locals.var_fs01__blk840_dn0 = assign27790_body5_e38874_d_n0;
            locals.var_fs01__blk840_dn2 = assign27790_body5_e38874_d_n2;
            locals.var_fs01__blk840_dn6 = assign27790_body5_e38874_d_n6;
            locals.var_fs01__blk840_dn7 = assign27790_body5_e38874_d_n7;
            locals.var_fs01__blk840_dn10 = assign27790_body5_e38874_d_n10;
            locals.var_fs01__blk840_dn11 = assign27790_body5_e38874_d_n11;
            locals.var_fs01__blk840_dn12 = assign27790_body5_e38874_d_n12;
            locals.var_fs01__blk840_dn17 = assign27790_body5_e38874_d_n17;
            locals.var_fs01__blk840_rv = 0.0;
            let (assign27790_body6_e38897, assign27790_body6_e38897_d_n0, assign27790_body6_e38897_d_n2, assign27790_body6_e38897_d_n6, assign27790_body6_e38897_d_n7, assign27790_body6_e38897_d_n10, assign27790_body6_e38897_d_n11, assign27790_body6_e38897_d_n12, assign27790_body6_e38897_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard882 != 0.0)) {
        let assign27790_body6_e38889: f64 = (locals.var_cfs1__blk846 * locals.var_beta);
        let assign27790_body6_e38891: f64 = (assign27790_body6_e38889 * 2.0);
        let assign27790_body6_e38893: f64 = (assign27790_body6_e38891 * locals.var_fi);
        let assign27790_body6_e38895: f64 = (assign27790_body6_e38893 * locals.var_fi_dchi);
        (assign27790_body6_e38895, ((((((locals.var_cfs1__blk846_dn0 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27790_body6_e38891 * locals.var_fi_dn0)) * locals.var_fi_dchi) + (assign27790_body6_e38893 * locals.var_fi_dchi_dn0)), ((((((locals.var_cfs1__blk846_dn2 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27790_body6_e38891 * locals.var_fi_dn2)) * locals.var_fi_dchi) + (assign27790_body6_e38893 * locals.var_fi_dchi_dn2)), ((((((locals.var_cfs1__blk846_dn6 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27790_body6_e38891 * locals.var_fi_dn6)) * locals.var_fi_dchi) + (assign27790_body6_e38893 * locals.var_fi_dchi_dn6)), ((((((locals.var_cfs1__blk846_dn7 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27790_body6_e38891 * locals.var_fi_dn7)) * locals.var_fi_dchi) + (assign27790_body6_e38893 * locals.var_fi_dchi_dn7)), (((((((locals.var_cfs1__blk846_dn10 * locals.var_beta) + (locals.var_cfs1__blk846 * locals.var_beta_dn10)) * 2.0) * locals.var_fi) + (assign27790_body6_e38891 * locals.var_fi_dn10)) * locals.var_fi_dchi) + (assign27790_body6_e38893 * locals.var_fi_dchi_dn10)), ((((((locals.var_cfs1__blk846_dn11 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27790_body6_e38891 * locals.var_fi_dn11)) * locals.var_fi_dchi) + (assign27790_body6_e38893 * locals.var_fi_dchi_dn11)), ((((((locals.var_cfs1__blk846_dn12 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27790_body6_e38891 * locals.var_fi_dn12)) * locals.var_fi_dchi) + (assign27790_body6_e38893 * locals.var_fi_dchi_dn12)), ((((((locals.var_cfs1__blk846_dn17 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27790_body6_e38891 * locals.var_fi_dn17)) * locals.var_fi_dchi) + (assign27790_body6_e38893 * locals.var_fi_dchi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk841, locals.var_fs01_dps0__blk841_dn0, locals.var_fs01_dps0__blk841_dn2, locals.var_fs01_dps0__blk841_dn6, locals.var_fs01_dps0__blk841_dn7, locals.var_fs01_dps0__blk841_dn10, locals.var_fs01_dps0__blk841_dn11, locals.var_fs01_dps0__blk841_dn12, locals.var_fs01_dps0__blk841_dn17,)
    }
};
            locals.var_fs01_dps0__blk841 = assign27790_body6_e38897;
            locals.var_fs01_dps0__blk841_dn0 = assign27790_body6_e38897_d_n0;
            locals.var_fs01_dps0__blk841_dn2 = assign27790_body6_e38897_d_n2;
            locals.var_fs01_dps0__blk841_dn6 = assign27790_body6_e38897_d_n6;
            locals.var_fs01_dps0__blk841_dn7 = assign27790_body6_e38897_d_n7;
            locals.var_fs01_dps0__blk841_dn10 = assign27790_body6_e38897_d_n10;
            locals.var_fs01_dps0__blk841_dn11 = assign27790_body6_e38897_d_n11;
            locals.var_fs01_dps0__blk841_dn12 = assign27790_body6_e38897_d_n12;
            locals.var_fs01_dps0__blk841_dn17 = assign27790_body6_e38897_d_n17;
            locals.var_fs01_dps0__blk841_rv = 0.0;
            let (assign27790_body7_e38932, assign27790_body7_e38932_d_n0, assign27790_body7_e38932_d_n2, assign27790_body7_e38932_d_n6, assign27790_body7_e38932_d_n7, assign27790_body7_e38932_d_n10, assign27790_body7_e38932_d_n11, assign27790_body7_e38932_d_n12, assign27790_body7_e38932_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard882 != 0.0)) {
        let assign27790_body7_e38914: f64 = (-0.117851130197758);
        let assign27790_body7_e38919: f64 = (-0.00163730162779191);
        let assign27790_body7_e38922: f64 = (locals.var_chi__blk818 * 6.36964918866352e-5);
        let assign27790_body7_e38923: f64 = (assign27790_body7_e38919 + assign27790_body7_e38922);
        let assign27790_body7_e38924: f64 = (locals.var_chi__blk818 * assign27790_body7_e38923);
        let assign27790_body7_e38925: f64 = (0.0178800506338833 + assign27790_body7_e38924);
        let assign27790_body7_e38926: f64 = (locals.var_chi__blk818 * assign27790_body7_e38925);
        let assign27790_body7_e38927: f64 = (assign27790_body7_e38914 + assign27790_body7_e38926);
        let assign27790_body7_e38928: f64 = (locals.var_chi__blk818 * assign27790_body7_e38927);
        let assign27790_body7_e38929: f64 = (0.707106781186548 + assign27790_body7_e38928);
        let assign27790_body7_e38930: f64 = (locals.var_chi__blk818 * assign27790_body7_e38929);
        (assign27790_body7_e38930, ((locals.var_chi__blk818_dn0 * assign27790_body7_e38929) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn0 * assign27790_body7_e38927) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn0 * assign27790_body7_e38925) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn0 * assign27790_body7_e38923) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk818_dn2 * assign27790_body7_e38929) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn2 * assign27790_body7_e38927) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn2 * assign27790_body7_e38925) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn2 * assign27790_body7_e38923) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk818_dn6 * assign27790_body7_e38929) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn6 * assign27790_body7_e38927) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn6 * assign27790_body7_e38925) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn6 * assign27790_body7_e38923) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk818_dn7 * assign27790_body7_e38929) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn7 * assign27790_body7_e38927) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn7 * assign27790_body7_e38925) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn7 * assign27790_body7_e38923) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk818_dn10 * assign27790_body7_e38929) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn10 * assign27790_body7_e38927) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn10 * assign27790_body7_e38925) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn10 * assign27790_body7_e38923) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk818_dn11 * assign27790_body7_e38929) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn11 * assign27790_body7_e38927) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn11 * assign27790_body7_e38925) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn11 * assign27790_body7_e38923) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk818_dn12 * assign27790_body7_e38929) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn12 * assign27790_body7_e38927) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn12 * assign27790_body7_e38925) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn12 * assign27790_body7_e38923) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn12 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk818_dn17 * assign27790_body7_e38929) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn17 * assign27790_body7_e38927) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn17 * assign27790_body7_e38925) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn17 * assign27790_body7_e38923) + (locals.var_chi__blk818 * (locals.var_chi__blk818_dn17 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb__blk842, locals.var_fb__blk842_dn0, locals.var_fb__blk842_dn2, locals.var_fb__blk842_dn6, locals.var_fb__blk842_dn7, locals.var_fb__blk842_dn10, locals.var_fb__blk842_dn11, locals.var_fb__blk842_dn12, locals.var_fb__blk842_dn17,)
    }
};
            locals.var_fb__blk842 = assign27790_body7_e38932;
            locals.var_fb__blk842_dn0 = assign27790_body7_e38932_d_n0;
            locals.var_fb__blk842_dn2 = assign27790_body7_e38932_d_n2;
            locals.var_fb__blk842_dn6 = assign27790_body7_e38932_d_n6;
            locals.var_fb__blk842_dn7 = assign27790_body7_e38932_d_n7;
            locals.var_fb__blk842_dn10 = assign27790_body7_e38932_d_n10;
            locals.var_fb__blk842_dn11 = assign27790_body7_e38932_d_n11;
            locals.var_fb__blk842_dn12 = assign27790_body7_e38932_d_n12;
            locals.var_fb__blk842_dn17 = assign27790_body7_e38932_d_n17;
            locals.var_fb__blk842_rv = 0.0;
            let (assign27790_body8_e38973, assign27790_body8_e38973_d_n0, assign27790_body8_e38973_d_n2, assign27790_body8_e38973_d_n6, assign27790_body8_e38973_d_n7, assign27790_body8_e38973_d_n10, assign27790_body8_e38973_d_n11, assign27790_body8_e38973_d_n12, assign27790_body8_e38973_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard882 != 0.0)) {
        let assign27790_body8_e38949: f64 = (-0.117851130197758);
        let assign27790_body8_e38950: f64 = (2.0 * assign27790_body8_e38949);
        let assign27790_body8_e38954: f64 = (3.0 * 0.0178800506338833);
        let assign27790_body8_e38958: f64 = (-0.00163730162779191);
        let assign27790_body8_e38959: f64 = (4.0 * assign27790_body8_e38958);
        let assign27790_body8_e38962: f64 = (locals.var_chi__blk818 * 5.0);
        let assign27790_body8_e38964: f64 = (assign27790_body8_e38962 * 6.36964918866352e-5);
        let assign27790_body8_e38965: f64 = (assign27790_body8_e38959 + assign27790_body8_e38964);
        let assign27790_body8_e38966: f64 = (locals.var_chi__blk818 * assign27790_body8_e38965);
        let assign27790_body8_e38967: f64 = (assign27790_body8_e38954 + assign27790_body8_e38966);
        let assign27790_body8_e38968: f64 = (locals.var_chi__blk818 * assign27790_body8_e38967);
        let assign27790_body8_e38969: f64 = (assign27790_body8_e38950 + assign27790_body8_e38968);
        let assign27790_body8_e38970: f64 = (locals.var_chi__blk818 * assign27790_body8_e38969);
        let assign27790_body8_e38971: f64 = (0.707106781186548 + assign27790_body8_e38970);
        (assign27790_body8_e38971, ((locals.var_chi__blk818_dn0 * assign27790_body8_e38969) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn0 * assign27790_body8_e38967) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn0 * assign27790_body8_e38965) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk818_dn2 * assign27790_body8_e38969) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn2 * assign27790_body8_e38967) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn2 * assign27790_body8_e38965) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk818_dn6 * assign27790_body8_e38969) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn6 * assign27790_body8_e38967) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn6 * assign27790_body8_e38965) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk818_dn7 * assign27790_body8_e38969) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn7 * assign27790_body8_e38967) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn7 * assign27790_body8_e38965) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk818_dn10 * assign27790_body8_e38969) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn10 * assign27790_body8_e38967) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn10 * assign27790_body8_e38965) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk818_dn11 * assign27790_body8_e38969) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn11 * assign27790_body8_e38967) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn11 * assign27790_body8_e38965) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk818_dn12 * assign27790_body8_e38969) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn12 * assign27790_body8_e38967) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn12 * assign27790_body8_e38965) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn12 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk818_dn17 * assign27790_body8_e38969) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn17 * assign27790_body8_e38967) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn17 * assign27790_body8_e38965) + (locals.var_chi__blk818 * ((locals.var_chi__blk818_dn17 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi, locals.var_fb_dchi_dn0, locals.var_fb_dchi_dn2, locals.var_fb_dchi_dn6, locals.var_fb_dchi_dn7, locals.var_fb_dchi_dn10, locals.var_fb_dchi_dn11, locals.var_fb_dchi_dn12, locals.var_fb_dchi_dn17,)
    }
};
            locals.var_fb_dchi = assign27790_body8_e38973;
            locals.var_fb_dchi_dn0 = assign27790_body8_e38973_d_n0;
            locals.var_fb_dchi_dn2 = assign27790_body8_e38973_d_n2;
            locals.var_fb_dchi_dn6 = assign27790_body8_e38973_d_n6;
            locals.var_fb_dchi_dn7 = assign27790_body8_e38973_d_n7;
            locals.var_fb_dchi_dn10 = assign27790_body8_e38973_d_n10;
            locals.var_fb_dchi_dn11 = assign27790_body8_e38973_d_n11;
            locals.var_fb_dchi_dn12 = assign27790_body8_e38973_d_n12;
            locals.var_fb_dchi_dn17 = assign27790_body8_e38973_d_n17;
            locals.var_fb_dchi_rv = 0.0;
            let (assign27790_body9_e38995, assign27790_body9_e38995_d_n0, assign27790_body9_e38995_d_n2, assign27790_body9_e38995_d_n6, assign27790_body9_e38995_d_n7, assign27790_body9_e38995_d_n10, assign27790_body9_e38995_d_n11, assign27790_body9_e38995_d_n12, assign27790_body9_e38995_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard882 != 0.0)) {
        let assign27790_body9_e38988: f64 = (locals.var_fb__blk842 * locals.var_fb__blk842);
        let assign27790_body9_e38990: f64 = (assign27790_body9_e38988 + locals.var_fs01__blk840);
        let assign27790_body9_e38992: f64 = (assign27790_body9_e38990 + 1e-50);
        let assign27790_body9_e38993: f64 = (assign27790_body9_e38992).sqrt();
        (assign27790_body9_e38993, ((((locals.var_fb__blk842_dn0 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn0)) + locals.var_fs01__blk840_dn0) / (2.0 * assign27790_body9_e38993)), ((((locals.var_fb__blk842_dn2 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn2)) + locals.var_fs01__blk840_dn2) / (2.0 * assign27790_body9_e38993)), ((((locals.var_fb__blk842_dn6 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn6)) + locals.var_fs01__blk840_dn6) / (2.0 * assign27790_body9_e38993)), ((((locals.var_fb__blk842_dn7 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn7)) + locals.var_fs01__blk840_dn7) / (2.0 * assign27790_body9_e38993)), ((((locals.var_fb__blk842_dn10 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn10)) + locals.var_fs01__blk840_dn10) / (2.0 * assign27790_body9_e38993)), ((((locals.var_fb__blk842_dn11 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn11)) + locals.var_fs01__blk840_dn11) / (2.0 * assign27790_body9_e38993)), ((((locals.var_fb__blk842_dn12 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn12)) + locals.var_fs01__blk840_dn12) / (2.0 * assign27790_body9_e38993)), ((((locals.var_fb__blk842_dn17 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn17)) + locals.var_fs01__blk840_dn17) / (2.0 * assign27790_body9_e38993)),)
    } else {
        (locals.var_fs02__blk844, locals.var_fs02__blk844_dn0, locals.var_fs02__blk844_dn2, locals.var_fs02__blk844_dn6, locals.var_fs02__blk844_dn7, locals.var_fs02__blk844_dn10, locals.var_fs02__blk844_dn11, locals.var_fs02__blk844_dn12, locals.var_fs02__blk844_dn17,)
    }
};
            locals.var_fs02__blk844 = assign27790_body9_e38995;
            locals.var_fs02__blk844_dn0 = assign27790_body9_e38995_d_n0;
            locals.var_fs02__blk844_dn2 = assign27790_body9_e38995_d_n2;
            locals.var_fs02__blk844_dn6 = assign27790_body9_e38995_d_n6;
            locals.var_fs02__blk844_dn7 = assign27790_body9_e38995_d_n7;
            locals.var_fs02__blk844_dn10 = assign27790_body9_e38995_d_n10;
            locals.var_fs02__blk844_dn11 = assign27790_body9_e38995_d_n11;
            locals.var_fs02__blk844_dn12 = assign27790_body9_e38995_d_n12;
            locals.var_fs02__blk844_dn17 = assign27790_body9_e38995_d_n17;
            locals.var_fs02__blk844_rv = 0.0;
            let (assign27790_body10_e39022, assign27790_body10_e39022_d_n0, assign27790_body10_e39022_d_n2, assign27790_body10_e39022_d_n6, assign27790_body10_e39022_d_n7, assign27790_body10_e39022_d_n10, assign27790_body10_e39022_d_n11, assign27790_body10_e39022_d_n12, assign27790_body10_e39022_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard882 != 0.0)) {
        let assign27790_body10_e39010: f64 = (locals.var_beta * locals.var_fb_dchi);
        let assign27790_body10_e39012: f64 = (assign27790_body10_e39010 * 2.0);
        let assign27790_body10_e39014: f64 = (assign27790_body10_e39012 * locals.var_fb__blk842);
        let assign27790_body10_e39016: f64 = (assign27790_body10_e39014 + locals.var_fs01_dps0__blk841);
        let assign27790_body10_e39019: f64 = (locals.var_fs02__blk844 + locals.var_fs02__blk844);
        let assign27790_body10_e39020: f64 = (assign27790_body10_e39016 / assign27790_body10_e39019);
        (assign27790_body10_e39020, ((((((((locals.var_beta * locals.var_fb_dchi_dn0) * 2.0) * locals.var_fb__blk842) + (assign27790_body10_e39012 * locals.var_fb__blk842_dn0)) + locals.var_fs01_dps0__blk841_dn0) * assign27790_body10_e39019) - (assign27790_body10_e39016 * (locals.var_fs02__blk844_dn0 + locals.var_fs02__blk844_dn0))) / (assign27790_body10_e39019 * assign27790_body10_e39019)), ((((((((locals.var_beta * locals.var_fb_dchi_dn2) * 2.0) * locals.var_fb__blk842) + (assign27790_body10_e39012 * locals.var_fb__blk842_dn2)) + locals.var_fs01_dps0__blk841_dn2) * assign27790_body10_e39019) - (assign27790_body10_e39016 * (locals.var_fs02__blk844_dn2 + locals.var_fs02__blk844_dn2))) / (assign27790_body10_e39019 * assign27790_body10_e39019)), ((((((((locals.var_beta * locals.var_fb_dchi_dn6) * 2.0) * locals.var_fb__blk842) + (assign27790_body10_e39012 * locals.var_fb__blk842_dn6)) + locals.var_fs01_dps0__blk841_dn6) * assign27790_body10_e39019) - (assign27790_body10_e39016 * (locals.var_fs02__blk844_dn6 + locals.var_fs02__blk844_dn6))) / (assign27790_body10_e39019 * assign27790_body10_e39019)), ((((((((locals.var_beta * locals.var_fb_dchi_dn7) * 2.0) * locals.var_fb__blk842) + (assign27790_body10_e39012 * locals.var_fb__blk842_dn7)) + locals.var_fs01_dps0__blk841_dn7) * assign27790_body10_e39019) - (assign27790_body10_e39016 * (locals.var_fs02__blk844_dn7 + locals.var_fs02__blk844_dn7))) / (assign27790_body10_e39019 * assign27790_body10_e39019)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn10)) * 2.0) * locals.var_fb__blk842) + (assign27790_body10_e39012 * locals.var_fb__blk842_dn10)) + locals.var_fs01_dps0__blk841_dn10) * assign27790_body10_e39019) - (assign27790_body10_e39016 * (locals.var_fs02__blk844_dn10 + locals.var_fs02__blk844_dn10))) / (assign27790_body10_e39019 * assign27790_body10_e39019)), ((((((((locals.var_beta * locals.var_fb_dchi_dn11) * 2.0) * locals.var_fb__blk842) + (assign27790_body10_e39012 * locals.var_fb__blk842_dn11)) + locals.var_fs01_dps0__blk841_dn11) * assign27790_body10_e39019) - (assign27790_body10_e39016 * (locals.var_fs02__blk844_dn11 + locals.var_fs02__blk844_dn11))) / (assign27790_body10_e39019 * assign27790_body10_e39019)), ((((((((locals.var_beta * locals.var_fb_dchi_dn12) * 2.0) * locals.var_fb__blk842) + (assign27790_body10_e39012 * locals.var_fb__blk842_dn12)) + locals.var_fs01_dps0__blk841_dn12) * assign27790_body10_e39019) - (assign27790_body10_e39016 * (locals.var_fs02__blk844_dn12 + locals.var_fs02__blk844_dn12))) / (assign27790_body10_e39019 * assign27790_body10_e39019)), ((((((((locals.var_beta * locals.var_fb_dchi_dn17) * 2.0) * locals.var_fb__blk842) + (assign27790_body10_e39012 * locals.var_fb__blk842_dn17)) + locals.var_fs01_dps0__blk841_dn17) * assign27790_body10_e39019) - (assign27790_body10_e39016 * (locals.var_fs02__blk844_dn17 + locals.var_fs02__blk844_dn17))) / (assign27790_body10_e39019 * assign27790_body10_e39019)),)
    } else {
        (locals.var_fs02_dps0__blk845, locals.var_fs02_dps0__blk845_dn0, locals.var_fs02_dps0__blk845_dn2, locals.var_fs02_dps0__blk845_dn6, locals.var_fs02_dps0__blk845_dn7, locals.var_fs02_dps0__blk845_dn10, locals.var_fs02_dps0__blk845_dn11, locals.var_fs02_dps0__blk845_dn12, locals.var_fs02_dps0__blk845_dn17,)
    }
};
            locals.var_fs02_dps0__blk845 = assign27790_body10_e39022;
            locals.var_fs02_dps0__blk845_dn0 = assign27790_body10_e39022_d_n0;
            locals.var_fs02_dps0__blk845_dn2 = assign27790_body10_e39022_d_n2;
            locals.var_fs02_dps0__blk845_dn6 = assign27790_body10_e39022_d_n6;
            locals.var_fs02_dps0__blk845_dn7 = assign27790_body10_e39022_d_n7;
            locals.var_fs02_dps0__blk845_dn10 = assign27790_body10_e39022_d_n10;
            locals.var_fs02_dps0__blk845_dn11 = assign27790_body10_e39022_d_n11;
            locals.var_fs02_dps0__blk845_dn12 = assign27790_body10_e39022_d_n12;
            locals.var_fs02_dps0__blk845_dn17 = assign27790_body10_e39022_d_n17;
            locals.var_fs02_dps0__blk845_rv = 0.0;
            let assign27790_body11_e39025: f64 = if locals.var_chi__blk818 < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard883 = assign27790_body11_e39025;
            locals.var_guard883_rv = 0.0;
            let (assign27790_body12_e39044, assign27790_body12_e39044_d_n0, assign27790_body12_e39044_d_n2, assign27790_body12_e39044_d_n6, assign27790_body12_e39044_d_n7, assign27790_body12_e39044_d_n10, assign27790_body12_e39044_d_n11, assign27790_body12_e39044_d_n12, assign27790_body12_e39044_d_n17,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard882 == 0.0)) && (locals.var_guard883 != 0.0)) {
        let assign27790_body12_e39042: f64 = (locals.var_chi__blk818).exp();
        (assign27790_body12_e39042, (assign27790_body12_e39042 * locals.var_chi__blk818_dn0), (assign27790_body12_e39042 * locals.var_chi__blk818_dn2), (assign27790_body12_e39042 * locals.var_chi__blk818_dn6), (assign27790_body12_e39042 * locals.var_chi__blk818_dn7), (assign27790_body12_e39042 * locals.var_chi__blk818_dn10), (assign27790_body12_e39042 * locals.var_chi__blk818_dn11), (assign27790_body12_e39042 * locals.var_chi__blk818_dn12), (assign27790_body12_e39042 * locals.var_chi__blk818_dn17),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17,)
    }
};
            locals.var_exp_chi = assign27790_body12_e39044;
            locals.var_exp_chi_dn0 = assign27790_body12_e39044_d_n0;
            locals.var_exp_chi_dn2 = assign27790_body12_e39044_d_n2;
            locals.var_exp_chi_dn6 = assign27790_body12_e39044_d_n6;
            locals.var_exp_chi_dn7 = assign27790_body12_e39044_d_n7;
            locals.var_exp_chi_dn10 = assign27790_body12_e39044_d_n10;
            locals.var_exp_chi_dn11 = assign27790_body12_e39044_d_n11;
            locals.var_exp_chi_dn12 = assign27790_body12_e39044_d_n12;
            locals.var_exp_chi_dn17 = assign27790_body12_e39044_d_n17;
            locals.var_exp_chi_rv = 0.0;
            let (assign27790_body13_e39066, assign27790_body13_e39066_d_n0, assign27790_body13_e39066_d_n2, assign27790_body13_e39066_d_n6, assign27790_body13_e39066_d_n7, assign27790_body13_e39066_d_n10, assign27790_body13_e39066_d_n11, assign27790_body13_e39066_d_n12, assign27790_body13_e39066_d_n17,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard882 == 0.0)) && (locals.var_guard883 != 0.0)) {
        let assign27790_body13_e39063: f64 = (locals.var_exp_chi - 1.0);
        let assign27790_body13_e39064: f64 = (locals.var_cfs1__blk846 * assign27790_body13_e39063);
        (assign27790_body13_e39064, ((locals.var_cfs1__blk846_dn0 * assign27790_body13_e39063) + (locals.var_cfs1__blk846 * locals.var_exp_chi_dn0)), ((locals.var_cfs1__blk846_dn2 * assign27790_body13_e39063) + (locals.var_cfs1__blk846 * locals.var_exp_chi_dn2)), ((locals.var_cfs1__blk846_dn6 * assign27790_body13_e39063) + (locals.var_cfs1__blk846 * locals.var_exp_chi_dn6)), ((locals.var_cfs1__blk846_dn7 * assign27790_body13_e39063) + (locals.var_cfs1__blk846 * locals.var_exp_chi_dn7)), ((locals.var_cfs1__blk846_dn10 * assign27790_body13_e39063) + (locals.var_cfs1__blk846 * locals.var_exp_chi_dn10)), ((locals.var_cfs1__blk846_dn11 * assign27790_body13_e39063) + (locals.var_cfs1__blk846 * locals.var_exp_chi_dn11)), ((locals.var_cfs1__blk846_dn12 * assign27790_body13_e39063) + (locals.var_cfs1__blk846 * locals.var_exp_chi_dn12)), ((locals.var_cfs1__blk846_dn17 * assign27790_body13_e39063) + (locals.var_cfs1__blk846 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01__blk840, locals.var_fs01__blk840_dn0, locals.var_fs01__blk840_dn2, locals.var_fs01__blk840_dn6, locals.var_fs01__blk840_dn7, locals.var_fs01__blk840_dn10, locals.var_fs01__blk840_dn11, locals.var_fs01__blk840_dn12, locals.var_fs01__blk840_dn17,)
    }
};
            locals.var_fs01__blk840 = assign27790_body13_e39066;
            locals.var_fs01__blk840_dn0 = assign27790_body13_e39066_d_n0;
            locals.var_fs01__blk840_dn2 = assign27790_body13_e39066_d_n2;
            locals.var_fs01__blk840_dn6 = assign27790_body13_e39066_d_n6;
            locals.var_fs01__blk840_dn7 = assign27790_body13_e39066_d_n7;
            locals.var_fs01__blk840_dn10 = assign27790_body13_e39066_d_n10;
            locals.var_fs01__blk840_dn11 = assign27790_body13_e39066_d_n11;
            locals.var_fs01__blk840_dn12 = assign27790_body13_e39066_d_n12;
            locals.var_fs01__blk840_dn17 = assign27790_body13_e39066_d_n17;
            locals.var_fs01__blk840_rv = 0.0;
            let (assign27790_body14_e39088, assign27790_body14_e39088_d_n0, assign27790_body14_e39088_d_n2, assign27790_body14_e39088_d_n6, assign27790_body14_e39088_d_n7, assign27790_body14_e39088_d_n10, assign27790_body14_e39088_d_n11, assign27790_body14_e39088_d_n12, assign27790_body14_e39088_d_n17,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard882 == 0.0)) && (locals.var_guard883 != 0.0)) {
        let assign27790_body14_e39084: f64 = (locals.var_cfs1__blk846 * locals.var_beta);
        let assign27790_body14_e39086: f64 = (assign27790_body14_e39084 * locals.var_exp_chi);
        (assign27790_body14_e39086, (((locals.var_cfs1__blk846_dn0 * locals.var_beta) * locals.var_exp_chi) + (assign27790_body14_e39084 * locals.var_exp_chi_dn0)), (((locals.var_cfs1__blk846_dn2 * locals.var_beta) * locals.var_exp_chi) + (assign27790_body14_e39084 * locals.var_exp_chi_dn2)), (((locals.var_cfs1__blk846_dn6 * locals.var_beta) * locals.var_exp_chi) + (assign27790_body14_e39084 * locals.var_exp_chi_dn6)), (((locals.var_cfs1__blk846_dn7 * locals.var_beta) * locals.var_exp_chi) + (assign27790_body14_e39084 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1__blk846_dn10 * locals.var_beta) + (locals.var_cfs1__blk846 * locals.var_beta_dn10)) * locals.var_exp_chi) + (assign27790_body14_e39084 * locals.var_exp_chi_dn10)), (((locals.var_cfs1__blk846_dn11 * locals.var_beta) * locals.var_exp_chi) + (assign27790_body14_e39084 * locals.var_exp_chi_dn11)), (((locals.var_cfs1__blk846_dn12 * locals.var_beta) * locals.var_exp_chi) + (assign27790_body14_e39084 * locals.var_exp_chi_dn12)), (((locals.var_cfs1__blk846_dn17 * locals.var_beta) * locals.var_exp_chi) + (assign27790_body14_e39084 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk841, locals.var_fs01_dps0__blk841_dn0, locals.var_fs01_dps0__blk841_dn2, locals.var_fs01_dps0__blk841_dn6, locals.var_fs01_dps0__blk841_dn7, locals.var_fs01_dps0__blk841_dn10, locals.var_fs01_dps0__blk841_dn11, locals.var_fs01_dps0__blk841_dn12, locals.var_fs01_dps0__blk841_dn17,)
    }
};
            locals.var_fs01_dps0__blk841 = assign27790_body14_e39088;
            locals.var_fs01_dps0__blk841_dn0 = assign27790_body14_e39088_d_n0;
            locals.var_fs01_dps0__blk841_dn2 = assign27790_body14_e39088_d_n2;
            locals.var_fs01_dps0__blk841_dn6 = assign27790_body14_e39088_d_n6;
            locals.var_fs01_dps0__blk841_dn7 = assign27790_body14_e39088_d_n7;
            locals.var_fs01_dps0__blk841_dn10 = assign27790_body14_e39088_d_n10;
            locals.var_fs01_dps0__blk841_dn11 = assign27790_body14_e39088_d_n11;
            locals.var_fs01_dps0__blk841_dn12 = assign27790_body14_e39088_d_n12;
            locals.var_fs01_dps0__blk841_dn17 = assign27790_body14_e39088_d_n17;
            locals.var_fs01_dps0__blk841_rv = 0.0;
            let (assign27790_body15_e39110, assign27790_body15_e39110_d_n0, assign27790_body15_e39110_d_n2, assign27790_body15_e39110_d_n6, assign27790_body15_e39110_d_n7, assign27790_body15_e39110_d_n10, assign27790_body15_e39110_d_n11, assign27790_body15_e39110_d_n12, assign27790_body15_e39110_d_n17,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard882 == 0.0)) && (locals.var_guard883 == 0.0)) {
        let assign27790_body15_e39107: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign27790_body15_e39108: f64 = (assign27790_body15_e39107).exp();
        (assign27790_body15_e39108, (assign27790_body15_e39108 * (locals.var_beta * locals.var_ps0ld_dn0)), (assign27790_body15_e39108 * (locals.var_beta * locals.var_ps0ld_dn2)), (assign27790_body15_e39108 * (locals.var_beta * locals.var_ps0ld_dn6)), (assign27790_body15_e39108 * (locals.var_beta * locals.var_ps0ld_dn7)), (assign27790_body15_e39108 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign27790_body15_e39108 * (locals.var_beta * locals.var_ps0ld_dn11)), (assign27790_body15_e39108 * (locals.var_beta * locals.var_ps0ld_dn12)), (assign27790_body15_e39108 * (locals.var_beta * locals.var_ps0ld_dn17)),)
    } else {
        (locals.var_exp_bps0__blk847, locals.var_exp_bps0__blk847_dn0, locals.var_exp_bps0__blk847_dn2, locals.var_exp_bps0__blk847_dn6, locals.var_exp_bps0__blk847_dn7, locals.var_exp_bps0__blk847_dn10, locals.var_exp_bps0__blk847_dn11, locals.var_exp_bps0__blk847_dn12, locals.var_exp_bps0__blk847_dn17,)
    }
};
            locals.var_exp_bps0__blk847 = assign27790_body15_e39110;
            locals.var_exp_bps0__blk847_dn0 = assign27790_body15_e39110_d_n0;
            locals.var_exp_bps0__blk847_dn2 = assign27790_body15_e39110_d_n2;
            locals.var_exp_bps0__blk847_dn6 = assign27790_body15_e39110_d_n6;
            locals.var_exp_bps0__blk847_dn7 = assign27790_body15_e39110_d_n7;
            locals.var_exp_bps0__blk847_dn10 = assign27790_body15_e39110_d_n10;
            locals.var_exp_bps0__blk847_dn11 = assign27790_body15_e39110_d_n11;
            locals.var_exp_bps0__blk847_dn12 = assign27790_body15_e39110_d_n12;
            locals.var_exp_bps0__blk847_dn17 = assign27790_body15_e39110_d_n17;
            locals.var_exp_bps0__blk847_rv = 0.0;
            let (assign27790_body16_e39133, assign27790_body16_e39133_d_n0, assign27790_body16_e39133_d_n2, assign27790_body16_e39133_d_n6, assign27790_body16_e39133_d_n7, assign27790_body16_e39133_d_n10, assign27790_body16_e39133_d_n11, assign27790_body16_e39133_d_n12, assign27790_body16_e39133_d_n17,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard882 == 0.0)) && (locals.var_guard883 == 0.0)) {
        let assign27790_body16_e39130: f64 = (locals.var_exp_bps0__blk847 - locals.var_exp_bvbs__blk837);
        let assign27790_body16_e39131: f64 = (locals.var_cnst1over * assign27790_body16_e39130);
        (assign27790_body16_e39131, ((locals.var_cnst1over_dn0 * assign27790_body16_e39130) + (locals.var_cnst1over * (locals.var_exp_bps0__blk847_dn0 - locals.var_exp_bvbs__blk837_dn0))), ((locals.var_cnst1over_dn2 * assign27790_body16_e39130) + (locals.var_cnst1over * (locals.var_exp_bps0__blk847_dn2 - locals.var_exp_bvbs__blk837_dn2))), ((locals.var_cnst1over_dn6 * assign27790_body16_e39130) + (locals.var_cnst1over * (locals.var_exp_bps0__blk847_dn6 - locals.var_exp_bvbs__blk837_dn6))), ((locals.var_cnst1over_dn7 * assign27790_body16_e39130) + (locals.var_cnst1over * (locals.var_exp_bps0__blk847_dn7 - locals.var_exp_bvbs__blk837_dn7))), ((locals.var_cnst1over_dn10 * assign27790_body16_e39130) + (locals.var_cnst1over * (locals.var_exp_bps0__blk847_dn10 - locals.var_exp_bvbs__blk837_dn10))), ((locals.var_cnst1over_dn11 * assign27790_body16_e39130) + (locals.var_cnst1over * (locals.var_exp_bps0__blk847_dn11 - locals.var_exp_bvbs__blk837_dn11))), ((locals.var_cnst1over_dn12 * assign27790_body16_e39130) + (locals.var_cnst1over * (locals.var_exp_bps0__blk847_dn12 - locals.var_exp_bvbs__blk837_dn12))), ((locals.var_cnst1over_dn17 * assign27790_body16_e39130) + (locals.var_cnst1over * (locals.var_exp_bps0__blk847_dn17 - locals.var_exp_bvbs__blk837_dn17))),)
    } else {
        (locals.var_fs01__blk840, locals.var_fs01__blk840_dn0, locals.var_fs01__blk840_dn2, locals.var_fs01__blk840_dn6, locals.var_fs01__blk840_dn7, locals.var_fs01__blk840_dn10, locals.var_fs01__blk840_dn11, locals.var_fs01__blk840_dn12, locals.var_fs01__blk840_dn17,)
    }
};
            locals.var_fs01__blk840 = assign27790_body16_e39133;
            locals.var_fs01__blk840_dn0 = assign27790_body16_e39133_d_n0;
            locals.var_fs01__blk840_dn2 = assign27790_body16_e39133_d_n2;
            locals.var_fs01__blk840_dn6 = assign27790_body16_e39133_d_n6;
            locals.var_fs01__blk840_dn7 = assign27790_body16_e39133_d_n7;
            locals.var_fs01__blk840_dn10 = assign27790_body16_e39133_d_n10;
            locals.var_fs01__blk840_dn11 = assign27790_body16_e39133_d_n11;
            locals.var_fs01__blk840_dn12 = assign27790_body16_e39133_d_n12;
            locals.var_fs01__blk840_dn17 = assign27790_body16_e39133_d_n17;
            locals.var_fs01__blk840_rv = 0.0;
            let (assign27790_body17_e39156, assign27790_body17_e39156_d_n0, assign27790_body17_e39156_d_n2, assign27790_body17_e39156_d_n6, assign27790_body17_e39156_d_n7, assign27790_body17_e39156_d_n10, assign27790_body17_e39156_d_n11, assign27790_body17_e39156_d_n12, assign27790_body17_e39156_d_n17,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard882 == 0.0)) && (locals.var_guard883 == 0.0)) {
        let assign27790_body17_e39152: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign27790_body17_e39154: f64 = (assign27790_body17_e39152 * locals.var_exp_bps0__blk847);
        (assign27790_body17_e39154, (((locals.var_cnst1over_dn0 * locals.var_beta) * locals.var_exp_bps0__blk847) + (assign27790_body17_e39152 * locals.var_exp_bps0__blk847_dn0)), (((locals.var_cnst1over_dn2 * locals.var_beta) * locals.var_exp_bps0__blk847) + (assign27790_body17_e39152 * locals.var_exp_bps0__blk847_dn2)), (((locals.var_cnst1over_dn6 * locals.var_beta) * locals.var_exp_bps0__blk847) + (assign27790_body17_e39152 * locals.var_exp_bps0__blk847_dn6)), (((locals.var_cnst1over_dn7 * locals.var_beta) * locals.var_exp_bps0__blk847) + (assign27790_body17_e39152 * locals.var_exp_bps0__blk847_dn7)), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * locals.var_exp_bps0__blk847) + (assign27790_body17_e39152 * locals.var_exp_bps0__blk847_dn10)), (((locals.var_cnst1over_dn11 * locals.var_beta) * locals.var_exp_bps0__blk847) + (assign27790_body17_e39152 * locals.var_exp_bps0__blk847_dn11)), (((locals.var_cnst1over_dn12 * locals.var_beta) * locals.var_exp_bps0__blk847) + (assign27790_body17_e39152 * locals.var_exp_bps0__blk847_dn12)), (((locals.var_cnst1over_dn17 * locals.var_beta) * locals.var_exp_bps0__blk847) + (assign27790_body17_e39152 * locals.var_exp_bps0__blk847_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk841, locals.var_fs01_dps0__blk841_dn0, locals.var_fs01_dps0__blk841_dn2, locals.var_fs01_dps0__blk841_dn6, locals.var_fs01_dps0__blk841_dn7, locals.var_fs01_dps0__blk841_dn10, locals.var_fs01_dps0__blk841_dn11, locals.var_fs01_dps0__blk841_dn12, locals.var_fs01_dps0__blk841_dn17,)
    }
};
            locals.var_fs01_dps0__blk841 = assign27790_body17_e39156;
            locals.var_fs01_dps0__blk841_dn0 = assign27790_body17_e39156_d_n0;
            locals.var_fs01_dps0__blk841_dn2 = assign27790_body17_e39156_d_n2;
            locals.var_fs01_dps0__blk841_dn6 = assign27790_body17_e39156_d_n6;
            locals.var_fs01_dps0__blk841_dn7 = assign27790_body17_e39156_d_n7;
            locals.var_fs01_dps0__blk841_dn10 = assign27790_body17_e39156_d_n10;
            locals.var_fs01_dps0__blk841_dn11 = assign27790_body17_e39156_d_n11;
            locals.var_fs01_dps0__blk841_dn12 = assign27790_body17_e39156_d_n12;
            locals.var_fs01_dps0__blk841_dn17 = assign27790_body17_e39156_d_n17;
            locals.var_fs01_dps0__blk841_rv = 0.0;
            let (assign27790_body18_e39177, assign27790_body18_e39177_d_n0, assign27790_body18_e39177_d_n2, assign27790_body18_e39177_d_n6, assign27790_body18_e39177_d_n7, assign27790_body18_e39177_d_n10, assign27790_body18_e39177_d_n11, assign27790_body18_e39177_d_n12, assign27790_body18_e39177_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard882 == 0.0)) {
        let assign27790_body18_e39172: f64 = (locals.var_chi__blk818 - 1.0);
        let assign27790_body18_e39174: f64 = (assign27790_body18_e39172 + locals.var_fs01__blk840);
        let assign27790_body18_e39175: f64 = (assign27790_body18_e39174).sqrt();
        (assign27790_body18_e39175, ((locals.var_chi__blk818_dn0 + locals.var_fs01__blk840_dn0) / (2.0 * assign27790_body18_e39175)), ((locals.var_chi__blk818_dn2 + locals.var_fs01__blk840_dn2) / (2.0 * assign27790_body18_e39175)), ((locals.var_chi__blk818_dn6 + locals.var_fs01__blk840_dn6) / (2.0 * assign27790_body18_e39175)), ((locals.var_chi__blk818_dn7 + locals.var_fs01__blk840_dn7) / (2.0 * assign27790_body18_e39175)), ((locals.var_chi__blk818_dn10 + locals.var_fs01__blk840_dn10) / (2.0 * assign27790_body18_e39175)), ((locals.var_chi__blk818_dn11 + locals.var_fs01__blk840_dn11) / (2.0 * assign27790_body18_e39175)), ((locals.var_chi__blk818_dn12 + locals.var_fs01__blk840_dn12) / (2.0 * assign27790_body18_e39175)), ((locals.var_chi__blk818_dn17 + locals.var_fs01__blk840_dn17) / (2.0 * assign27790_body18_e39175)),)
    } else {
        (locals.var_fs02__blk844, locals.var_fs02__blk844_dn0, locals.var_fs02__blk844_dn2, locals.var_fs02__blk844_dn6, locals.var_fs02__blk844_dn7, locals.var_fs02__blk844_dn10, locals.var_fs02__blk844_dn11, locals.var_fs02__blk844_dn12, locals.var_fs02__blk844_dn17,)
    }
};
            locals.var_fs02__blk844 = assign27790_body18_e39177;
            locals.var_fs02__blk844_dn0 = assign27790_body18_e39177_d_n0;
            locals.var_fs02__blk844_dn2 = assign27790_body18_e39177_d_n2;
            locals.var_fs02__blk844_dn6 = assign27790_body18_e39177_d_n6;
            locals.var_fs02__blk844_dn7 = assign27790_body18_e39177_d_n7;
            locals.var_fs02__blk844_dn10 = assign27790_body18_e39177_d_n10;
            locals.var_fs02__blk844_dn11 = assign27790_body18_e39177_d_n11;
            locals.var_fs02__blk844_dn12 = assign27790_body18_e39177_d_n12;
            locals.var_fs02__blk844_dn17 = assign27790_body18_e39177_d_n17;
            locals.var_fs02__blk844_rv = 0.0;
            let (assign27790_body19_e39199, assign27790_body19_e39199_d_n0, assign27790_body19_e39199_d_n2, assign27790_body19_e39199_d_n6, assign27790_body19_e39199_d_n7, assign27790_body19_e39199_d_n10, assign27790_body19_e39199_d_n11, assign27790_body19_e39199_d_n12, assign27790_body19_e39199_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard882 == 0.0)) {
        let assign27790_body19_e39193: f64 = (locals.var_beta + locals.var_fs01_dps0__blk841);
        let assign27790_body19_e39195: f64 = (assign27790_body19_e39193 / locals.var_fs02__blk844);
        let assign27790_body19_e39197: f64 = (assign27790_body19_e39195 * 0.5);
        (assign27790_body19_e39197, ((((locals.var_fs01_dps0__blk841_dn0 * locals.var_fs02__blk844) - (assign27790_body19_e39193 * locals.var_fs02__blk844_dn0)) / (locals.var_fs02__blk844 * locals.var_fs02__blk844)) * 0.5), ((((locals.var_fs01_dps0__blk841_dn2 * locals.var_fs02__blk844) - (assign27790_body19_e39193 * locals.var_fs02__blk844_dn2)) / (locals.var_fs02__blk844 * locals.var_fs02__blk844)) * 0.5), ((((locals.var_fs01_dps0__blk841_dn6 * locals.var_fs02__blk844) - (assign27790_body19_e39193 * locals.var_fs02__blk844_dn6)) / (locals.var_fs02__blk844 * locals.var_fs02__blk844)) * 0.5), ((((locals.var_fs01_dps0__blk841_dn7 * locals.var_fs02__blk844) - (assign27790_body19_e39193 * locals.var_fs02__blk844_dn7)) / (locals.var_fs02__blk844 * locals.var_fs02__blk844)) * 0.5), (((((locals.var_beta_dn10 + locals.var_fs01_dps0__blk841_dn10) * locals.var_fs02__blk844) - (assign27790_body19_e39193 * locals.var_fs02__blk844_dn10)) / (locals.var_fs02__blk844 * locals.var_fs02__blk844)) * 0.5), ((((locals.var_fs01_dps0__blk841_dn11 * locals.var_fs02__blk844) - (assign27790_body19_e39193 * locals.var_fs02__blk844_dn11)) / (locals.var_fs02__blk844 * locals.var_fs02__blk844)) * 0.5), ((((locals.var_fs01_dps0__blk841_dn12 * locals.var_fs02__blk844) - (assign27790_body19_e39193 * locals.var_fs02__blk844_dn12)) / (locals.var_fs02__blk844 * locals.var_fs02__blk844)) * 0.5), ((((locals.var_fs01_dps0__blk841_dn17 * locals.var_fs02__blk844) - (assign27790_body19_e39193 * locals.var_fs02__blk844_dn17)) / (locals.var_fs02__blk844 * locals.var_fs02__blk844)) * 0.5),)
    } else {
        (locals.var_fs02_dps0__blk845, locals.var_fs02_dps0__blk845_dn0, locals.var_fs02_dps0__blk845_dn2, locals.var_fs02_dps0__blk845_dn6, locals.var_fs02_dps0__blk845_dn7, locals.var_fs02_dps0__blk845_dn10, locals.var_fs02_dps0__blk845_dn11, locals.var_fs02_dps0__blk845_dn12, locals.var_fs02_dps0__blk845_dn17,)
    }
};
            locals.var_fs02_dps0__blk845 = assign27790_body19_e39199;
            locals.var_fs02_dps0__blk845_dn0 = assign27790_body19_e39199_d_n0;
            locals.var_fs02_dps0__blk845_dn2 = assign27790_body19_e39199_d_n2;
            locals.var_fs02_dps0__blk845_dn6 = assign27790_body19_e39199_d_n6;
            locals.var_fs02_dps0__blk845_dn7 = assign27790_body19_e39199_d_n7;
            locals.var_fs02_dps0__blk845_dn10 = assign27790_body19_e39199_d_n10;
            locals.var_fs02_dps0__blk845_dn11 = assign27790_body19_e39199_d_n11;
            locals.var_fs02_dps0__blk845_dn12 = assign27790_body19_e39199_d_n12;
            locals.var_fs02_dps0__blk845_dn17 = assign27790_body19_e39199_d_n17;
            locals.var_fs02_dps0__blk845_rv = 0.0;
            let (assign27790_body20_e39218, assign27790_body20_e39218_d_n0, assign27790_body20_e39218_d_n2, assign27790_body20_e39218_d_n6, assign27790_body20_e39218_d_n7, assign27790_body20_e39218_d_n10, assign27790_body20_e39218_d_n11, assign27790_body20_e39218_d_n12, assign27790_body20_e39218_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        let assign27790_body20_e39212: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign27790_body20_e39215: f64 = (locals.var_fac1__blk804 * locals.var_fs02__blk844);
        let assign27790_body20_e39216: f64 = (assign27790_body20_e39212 - assign27790_body20_e39215);
        (assign27790_body20_e39216, ((locals.var_vgpld_dn0 - locals.var_ps0ld_dn0) - ((locals.var_fac1__blk804_dn0 * locals.var_fs02__blk844) + (locals.var_fac1__blk804 * locals.var_fs02__blk844_dn0))), ((locals.var_vgpld_dn2 - locals.var_ps0ld_dn2) - ((locals.var_fac1__blk804_dn2 * locals.var_fs02__blk844) + (locals.var_fac1__blk804 * locals.var_fs02__blk844_dn2))), ((locals.var_vgpld_dn6 - locals.var_ps0ld_dn6) - ((locals.var_fac1__blk804_dn6 * locals.var_fs02__blk844) + (locals.var_fac1__blk804 * locals.var_fs02__blk844_dn6))), ((locals.var_vgpld_dn7 - locals.var_ps0ld_dn7) - ((locals.var_fac1__blk804_dn7 * locals.var_fs02__blk844) + (locals.var_fac1__blk804 * locals.var_fs02__blk844_dn7))), ((locals.var_vgpld_dn10 - locals.var_ps0ld_dn10) - ((locals.var_fac1__blk804_dn10 * locals.var_fs02__blk844) + (locals.var_fac1__blk804 * locals.var_fs02__blk844_dn10))), ((locals.var_vgpld_dn11 - locals.var_ps0ld_dn11) - ((locals.var_fac1__blk804_dn11 * locals.var_fs02__blk844) + (locals.var_fac1__blk804 * locals.var_fs02__blk844_dn11))), ((locals.var_vgpld_dn12 - locals.var_ps0ld_dn12) - ((locals.var_fac1__blk804_dn12 * locals.var_fs02__blk844) + (locals.var_fac1__blk804 * locals.var_fs02__blk844_dn12))), ((locals.var_vgpld_dn17 - locals.var_ps0ld_dn17) - ((locals.var_fac1__blk804_dn17 * locals.var_fs02__blk844) + (locals.var_fac1__blk804 * locals.var_fs02__blk844_dn17))),)
    } else {
        (locals.var_fs0__blk848, locals.var_fs0__blk848_dn0, locals.var_fs0__blk848_dn2, locals.var_fs0__blk848_dn6, locals.var_fs0__blk848_dn7, locals.var_fs0__blk848_dn10, locals.var_fs0__blk848_dn11, locals.var_fs0__blk848_dn12, locals.var_fs0__blk848_dn17,)
    }
};
            locals.var_fs0__blk848 = assign27790_body20_e39218;
            locals.var_fs0__blk848_dn0 = assign27790_body20_e39218_d_n0;
            locals.var_fs0__blk848_dn2 = assign27790_body20_e39218_d_n2;
            locals.var_fs0__blk848_dn6 = assign27790_body20_e39218_d_n6;
            locals.var_fs0__blk848_dn7 = assign27790_body20_e39218_d_n7;
            locals.var_fs0__blk848_dn10 = assign27790_body20_e39218_d_n10;
            locals.var_fs0__blk848_dn11 = assign27790_body20_e39218_d_n11;
            locals.var_fs0__blk848_dn12 = assign27790_body20_e39218_d_n12;
            locals.var_fs0__blk848_dn17 = assign27790_body20_e39218_d_n17;
            locals.var_fs0__blk848_rv = 0.0;
            let (assign27790_body21_e39236, assign27790_body21_e39236_d_n0, assign27790_body21_e39236_d_n2, assign27790_body21_e39236_d_n6, assign27790_body21_e39236_d_n7, assign27790_body21_e39236_d_n10, assign27790_body21_e39236_d_n11, assign27790_body21_e39236_d_n12, assign27790_body21_e39236_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        let assign27790_body21_e39230: f64 = (-1.0);
        let assign27790_body21_e39233: f64 = (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845);
        let assign27790_body21_e39234: f64 = (assign27790_body21_e39230 - assign27790_body21_e39233);
        (assign27790_body21_e39234, (-((locals.var_fac1__blk804_dn0 * locals.var_fs02_dps0__blk845) + (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845_dn0))), (-((locals.var_fac1__blk804_dn2 * locals.var_fs02_dps0__blk845) + (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845_dn2))), (-((locals.var_fac1__blk804_dn6 * locals.var_fs02_dps0__blk845) + (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845_dn6))), (-((locals.var_fac1__blk804_dn7 * locals.var_fs02_dps0__blk845) + (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845_dn7))), (-((locals.var_fac1__blk804_dn10 * locals.var_fs02_dps0__blk845) + (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845_dn10))), (-((locals.var_fac1__blk804_dn11 * locals.var_fs02_dps0__blk845) + (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845_dn11))), (-((locals.var_fac1__blk804_dn12 * locals.var_fs02_dps0__blk845) + (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845_dn12))), (-((locals.var_fac1__blk804_dn17 * locals.var_fs02_dps0__blk845) + (locals.var_fac1__blk804 * locals.var_fs02_dps0__blk845_dn17))),)
    } else {
        (locals.var_fs0_dps0__blk849, locals.var_fs0_dps0__blk849_dn0, locals.var_fs0_dps0__blk849_dn2, locals.var_fs0_dps0__blk849_dn6, locals.var_fs0_dps0__blk849_dn7, locals.var_fs0_dps0__blk849_dn10, locals.var_fs0_dps0__blk849_dn11, locals.var_fs0_dps0__blk849_dn12, locals.var_fs0_dps0__blk849_dn17,)
    }
};
            locals.var_fs0_dps0__blk849 = assign27790_body21_e39236;
            locals.var_fs0_dps0__blk849_dn0 = assign27790_body21_e39236_d_n0;
            locals.var_fs0_dps0__blk849_dn2 = assign27790_body21_e39236_d_n2;
            locals.var_fs0_dps0__blk849_dn6 = assign27790_body21_e39236_d_n6;
            locals.var_fs0_dps0__blk849_dn7 = assign27790_body21_e39236_d_n7;
            locals.var_fs0_dps0__blk849_dn10 = assign27790_body21_e39236_d_n10;
            locals.var_fs0_dps0__blk849_dn11 = assign27790_body21_e39236_d_n11;
            locals.var_fs0_dps0__blk849_dn12 = assign27790_body21_e39236_d_n12;
            locals.var_fs0_dps0__blk849_dn17 = assign27790_body21_e39236_d_n17;
            locals.var_fs0_dps0__blk849_rv = 0.0;
            let assign27790_body22_e39239: f64 = if locals.var_flg_conv__blk791 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard884 = assign27790_body22_e39239;
            locals.var_guard884_rv = 0.0;
            let (assign27790_body23_e39258,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard884 != 0.0)) {
        let assign27790_body23_e39254: f64 = (2.0 * 20.0);
        let assign27790_body23_e39256: f64 = (assign27790_body23_e39254 + 1.0);
        (assign27790_body23_e39256,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign27790_body23_e39258;
            locals.var_lp_s0_rv = 0.0;
            let (assign27790_body24_e39277, assign27790_body24_e39277_d_n0, assign27790_body24_e39277_d_n2, assign27790_body24_e39277_d_n6, assign27790_body24_e39277_d_n7, assign27790_body24_e39277_d_n10, assign27790_body24_e39277_d_n11, assign27790_body24_e39277_d_n12, assign27790_body24_e39277_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard884 == 0.0)) {
        let assign27790_body24_e39273: f64 = (-locals.var_fs0__blk848);
        let assign27790_body24_e39275: f64 = (assign27790_body24_e39273 / locals.var_fs0_dps0__blk849);
        (assign27790_body24_e39275, ((((-locals.var_fs0__blk848_dn0) * locals.var_fs0_dps0__blk849) - (assign27790_body24_e39273 * locals.var_fs0_dps0__blk849_dn0)) / (locals.var_fs0_dps0__blk849 * locals.var_fs0_dps0__blk849)), ((((-locals.var_fs0__blk848_dn2) * locals.var_fs0_dps0__blk849) - (assign27790_body24_e39273 * locals.var_fs0_dps0__blk849_dn2)) / (locals.var_fs0_dps0__blk849 * locals.var_fs0_dps0__blk849)), ((((-locals.var_fs0__blk848_dn6) * locals.var_fs0_dps0__blk849) - (assign27790_body24_e39273 * locals.var_fs0_dps0__blk849_dn6)) / (locals.var_fs0_dps0__blk849 * locals.var_fs0_dps0__blk849)), ((((-locals.var_fs0__blk848_dn7) * locals.var_fs0_dps0__blk849) - (assign27790_body24_e39273 * locals.var_fs0_dps0__blk849_dn7)) / (locals.var_fs0_dps0__blk849 * locals.var_fs0_dps0__blk849)), ((((-locals.var_fs0__blk848_dn10) * locals.var_fs0_dps0__blk849) - (assign27790_body24_e39273 * locals.var_fs0_dps0__blk849_dn10)) / (locals.var_fs0_dps0__blk849 * locals.var_fs0_dps0__blk849)), ((((-locals.var_fs0__blk848_dn11) * locals.var_fs0_dps0__blk849) - (assign27790_body24_e39273 * locals.var_fs0_dps0__blk849_dn11)) / (locals.var_fs0_dps0__blk849 * locals.var_fs0_dps0__blk849)), ((((-locals.var_fs0__blk848_dn12) * locals.var_fs0_dps0__blk849) - (assign27790_body24_e39273 * locals.var_fs0_dps0__blk849_dn12)) / (locals.var_fs0_dps0__blk849 * locals.var_fs0_dps0__blk849)), ((((-locals.var_fs0__blk848_dn17) * locals.var_fs0_dps0__blk849) - (assign27790_body24_e39273 * locals.var_fs0_dps0__blk849_dn17)) / (locals.var_fs0_dps0__blk849 * locals.var_fs0_dps0__blk849)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign27790_body24_e39277;
            locals.var_dps0_dn0 = assign27790_body24_e39277_d_n0;
            locals.var_dps0_dn2 = assign27790_body24_e39277_d_n2;
            locals.var_dps0_dn6 = assign27790_body24_e39277_d_n6;
            locals.var_dps0_dn7 = assign27790_body24_e39277_d_n7;
            locals.var_dps0_dn10 = assign27790_body24_e39277_d_n10;
            locals.var_dps0_dn11 = assign27790_body24_e39277_d_n11;
            locals.var_dps0_dn12 = assign27790_body24_e39277_d_n12;
            locals.var_dps0_dn17 = assign27790_body24_e39277_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign27790_body25_e39306, assign27790_body25_e39306_d_n0, assign27790_body25_e39306_d_n2, assign27790_body25_e39306_d_n6, assign27790_body25_e39306_d_n7, assign27790_body25_e39306_d_n10, assign27790_body25_e39306_d_n11, assign27790_body25_e39306_d_n12, assign27790_body25_e39306_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard884 == 0.0)) {
        let assign27790_body25_e39293: f64 = (0.5 * 0.1);
        let assign27790_body25_e39297: f64 = (locals.var_ps0ld).abs();
        let (assign27790_body25_e39302, assign27790_body25_e39302_d_n0, assign27790_body25_e39302_d_n2, assign27790_body25_e39302_d_n6, assign27790_body25_e39302_d_n7, assign27790_body25_e39302_d_n10, assign27790_body25_e39302_d_n11, assign27790_body25_e39302_d_n12, assign27790_body25_e39302_d_n17,) = {
            if (1.0 >= assign27790_body25_e39297) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign27790_body25_e39301: f64 = (locals.var_ps0ld).abs();
                (assign27790_body25_e39301, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn12 } else { (-locals.var_ps0ld_dn12) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn17 } else { (-locals.var_ps0ld_dn17) },)
            }
        };
        let assign27790_body25_e39303: f64 = (1.0 + assign27790_body25_e39302);
        let assign27790_body25_e39304: f64 = (assign27790_body25_e39293 * assign27790_body25_e39303);
        (assign27790_body25_e39304, (assign27790_body25_e39293 * assign27790_body25_e39302_d_n0), (assign27790_body25_e39293 * assign27790_body25_e39302_d_n2), (assign27790_body25_e39293 * assign27790_body25_e39302_d_n6), (assign27790_body25_e39293 * assign27790_body25_e39302_d_n7), (assign27790_body25_e39293 * assign27790_body25_e39302_d_n10), (assign27790_body25_e39293 * assign27790_body25_e39302_d_n11), (assign27790_body25_e39293 * assign27790_body25_e39302_d_n12), (assign27790_body25_e39293 * assign27790_body25_e39302_d_n17),)
    } else {
        (locals.var_dplim__blk850, locals.var_dplim__blk850_dn0, locals.var_dplim__blk850_dn2, locals.var_dplim__blk850_dn6, locals.var_dplim__blk850_dn7, locals.var_dplim__blk850_dn10, locals.var_dplim__blk850_dn11, locals.var_dplim__blk850_dn12, locals.var_dplim__blk850_dn17,)
    }
};
            locals.var_dplim__blk850 = assign27790_body25_e39306;
            locals.var_dplim__blk850_dn0 = assign27790_body25_e39306_d_n0;
            locals.var_dplim__blk850_dn2 = assign27790_body25_e39306_d_n2;
            locals.var_dplim__blk850_dn6 = assign27790_body25_e39306_d_n6;
            locals.var_dplim__blk850_dn7 = assign27790_body25_e39306_d_n7;
            locals.var_dplim__blk850_dn10 = assign27790_body25_e39306_d_n10;
            locals.var_dplim__blk850_dn11 = assign27790_body25_e39306_d_n11;
            locals.var_dplim__blk850_dn12 = assign27790_body25_e39306_d_n12;
            locals.var_dplim__blk850_dn17 = assign27790_body25_e39306_d_n17;
            locals.var_dplim__blk850_rv = 0.0;
            let assign27790_body26_e39308: f64 = (locals.var_dps0).abs();
            let assign27790_body26_e39310: f64 = if assign27790_body26_e39308 > locals.var_dplim__blk850 { 1.0 } else { 0.0 };
            locals.var_guard885 = assign27790_body26_e39310;
            locals.var_guard885_rv = 0.0;
            let (assign27790_body27_e39336, assign27790_body27_e39336_d_n0, assign27790_body27_e39336_d_n2, assign27790_body27_e39336_d_n6, assign27790_body27_e39336_d_n7, assign27790_body27_e39336_d_n10, assign27790_body27_e39336_d_n11, assign27790_body27_e39336_d_n12, assign27790_body27_e39336_d_n17,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard884 == 0.0)) && (locals.var_guard885 != 0.0)) {
        let (assign27790_body27_e39333,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign27790_body27_e39332: f64 = (-1.0);
                (assign27790_body27_e39332,)
            }
        };
        let assign27790_body27_e39334: f64 = (locals.var_dplim__blk850 * assign27790_body27_e39333);
        (assign27790_body27_e39334, (locals.var_dplim__blk850_dn0 * assign27790_body27_e39333), (locals.var_dplim__blk850_dn2 * assign27790_body27_e39333), (locals.var_dplim__blk850_dn6 * assign27790_body27_e39333), (locals.var_dplim__blk850_dn7 * assign27790_body27_e39333), (locals.var_dplim__blk850_dn10 * assign27790_body27_e39333), (locals.var_dplim__blk850_dn11 * assign27790_body27_e39333), (locals.var_dplim__blk850_dn12 * assign27790_body27_e39333), (locals.var_dplim__blk850_dn17 * assign27790_body27_e39333),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign27790_body27_e39336;
            locals.var_dps0_dn0 = assign27790_body27_e39336_d_n0;
            locals.var_dps0_dn2 = assign27790_body27_e39336_d_n2;
            locals.var_dps0_dn6 = assign27790_body27_e39336_d_n6;
            locals.var_dps0_dn7 = assign27790_body27_e39336_d_n7;
            locals.var_dps0_dn10 = assign27790_body27_e39336_d_n10;
            locals.var_dps0_dn11 = assign27790_body27_e39336_d_n11;
            locals.var_dps0_dn12 = assign27790_body27_e39336_d_n12;
            locals.var_dps0_dn17 = assign27790_body27_e39336_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign27790_body28_e39354, assign27790_body28_e39354_d_n0, assign27790_body28_e39354_d_n2, assign27790_body28_e39354_d_n6, assign27790_body28_e39354_d_n7, assign27790_body28_e39354_d_n10, assign27790_body28_e39354_d_n11, assign27790_body28_e39354_d_n12, assign27790_body28_e39354_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard884 == 0.0)) {
        let assign27790_body28_e39352: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign27790_body28_e39352, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn12 + locals.var_dps0_dn12), (locals.var_ps0ld_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
            locals.var_ps0ld = assign27790_body28_e39354;
            locals.var_ps0ld_dn0 = assign27790_body28_e39354_d_n0;
            locals.var_ps0ld_dn2 = assign27790_body28_e39354_d_n2;
            locals.var_ps0ld_dn6 = assign27790_body28_e39354_d_n6;
            locals.var_ps0ld_dn7 = assign27790_body28_e39354_d_n7;
            locals.var_ps0ld_dn10 = assign27790_body28_e39354_d_n10;
            locals.var_ps0ld_dn11 = assign27790_body28_e39354_d_n11;
            locals.var_ps0ld_dn12 = assign27790_body28_e39354_d_n12;
            locals.var_ps0ld_dn17 = assign27790_body28_e39354_d_n17;
            locals.var_ps0ld_rv = 0.0;
            let assign27790_body29_e39356: f64 = (locals.var_dps0).abs();
            let assign27790_body29_e39360: f64 = (locals.var_fs0__blk848).abs();
            let assign27790_body29_e39363: f64 = if ((assign27790_body29_e39356 <= 5e-12) && (assign27790_body29_e39360 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard886 = assign27790_body29_e39363;
            locals.var_guard886_rv = 0.0;
            let (assign27790_body30_e39381,) = {
    if (((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard884 == 0.0)) && (locals.var_guard886 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv__blk791,)
    }
};
            locals.var_flg_conv__blk791 = assign27790_body30_e39381;
            locals.var_flg_conv__blk791_rv = 0.0;
            let (assign27790_body31_e39396,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        let assign27790_body31_e39394: f64 = (locals.var_lp_s0 + 1.0);
        (assign27790_body31_e39394,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign27790_body31_e39396;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_101(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign27810_e39402: f64 = if locals.var_chi__blk818 < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard888 = assign27810_e39402;
        locals.var_guard888_rv = 0.0;

        let (assign27850_e39461, assign27850_e39461_d_n0, assign27850_e39461_d_n2, assign27850_e39461_d_n6, assign27850_e39461_d_n7, assign27850_e39461_d_n10, assign27850_e39461_d_n11, assign27850_e39461_d_n12, assign27850_e39461_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard888 != 0.0)) {
        let assign27850_e39455: f64 = (locals.var_fb__blk842 * locals.var_fb__blk842);
        let assign27850_e39458: f64 = (10.0 * 2.220446049250313e-16);
        let assign27850_e39459: f64 = (assign27850_e39455 + assign27850_e39458);
        (assign27850_e39459, ((locals.var_fb__blk842_dn0 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn0)), ((locals.var_fb__blk842_dn2 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn2)), ((locals.var_fb__blk842_dn6 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn6)), ((locals.var_fb__blk842_dn7 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn7)), ((locals.var_fb__blk842_dn10 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn10)), ((locals.var_fb__blk842_dn11 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn11)), ((locals.var_fb__blk842_dn12 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn12)), ((locals.var_fb__blk842_dn17 * locals.var_fb__blk842) + (locals.var_fb__blk842 * locals.var_fb__blk842_dn17)),)
    } else {
        (locals.var_xi0__blk851, locals.var_xi0__blk851_dn0, locals.var_xi0__blk851_dn2, locals.var_xi0__blk851_dn6, locals.var_xi0__blk851_dn7, locals.var_xi0__blk851_dn10, locals.var_xi0__blk851_dn11, locals.var_xi0__blk851_dn12, locals.var_xi0__blk851_dn17,)
    }
};
        locals.var_xi0__blk851 = assign27850_e39461;
        locals.var_xi0__blk851_dn0 = assign27850_e39461_d_n0;
        locals.var_xi0__blk851_dn2 = assign27850_e39461_d_n2;
        locals.var_xi0__blk851_dn6 = assign27850_e39461_d_n6;
        locals.var_xi0__blk851_dn7 = assign27850_e39461_d_n7;
        locals.var_xi0__blk851_dn10 = assign27850_e39461_d_n10;
        locals.var_xi0__blk851_dn11 = assign27850_e39461_d_n11;
        locals.var_xi0__blk851_dn12 = assign27850_e39461_d_n12;
        locals.var_xi0__blk851_dn17 = assign27850_e39461_d_n17;
        locals.var_xi0__blk851_rv = 0.0;

        let (assign27860_e39480, assign27860_e39480_d_n0, assign27860_e39480_d_n2, assign27860_e39480_d_n6, assign27860_e39480_d_n7, assign27860_e39480_d_n10, assign27860_e39480_d_n11, assign27860_e39480_d_n12, assign27860_e39480_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard888 != 0.0)) {
        let assign27860_e39477: f64 = (10.0 * 2.220446049250313e-16);
        let assign27860_e39478: f64 = (locals.var_fb__blk842 + assign27860_e39477);
        (assign27860_e39478, locals.var_fb__blk842_dn0, locals.var_fb__blk842_dn2, locals.var_fb__blk842_dn6, locals.var_fb__blk842_dn7, locals.var_fb__blk842_dn10, locals.var_fb__blk842_dn11, locals.var_fb__blk842_dn12, locals.var_fb__blk842_dn17,)
    } else {
        (locals.var_xi0p12__blk852, locals.var_xi0p12__blk852_dn0, locals.var_xi0p12__blk852_dn2, locals.var_xi0p12__blk852_dn6, locals.var_xi0p12__blk852_dn7, locals.var_xi0p12__blk852_dn10, locals.var_xi0p12__blk852_dn11, locals.var_xi0p12__blk852_dn12, locals.var_xi0p12__blk852_dn17,)
    }
};
        locals.var_xi0p12__blk852 = assign27860_e39480;
        locals.var_xi0p12__blk852_dn0 = assign27860_e39480_d_n0;
        locals.var_xi0p12__blk852_dn2 = assign27860_e39480_d_n2;
        locals.var_xi0p12__blk852_dn6 = assign27860_e39480_d_n6;
        locals.var_xi0p12__blk852_dn7 = assign27860_e39480_d_n7;
        locals.var_xi0p12__blk852_dn10 = assign27860_e39480_d_n10;
        locals.var_xi0p12__blk852_dn11 = assign27860_e39480_d_n11;
        locals.var_xi0p12__blk852_dn12 = assign27860_e39480_d_n12;
        locals.var_xi0p12__blk852_dn17 = assign27860_e39480_d_n17;
        locals.var_xi0p12__blk852_rv = 0.0;

        let (assign27880_e39514, assign27880_e39514_d_n0, assign27880_e39514_d_n2, assign27880_e39514_d_n6, assign27880_e39514_d_n7, assign27880_e39514_d_n10, assign27880_e39514_d_n11, assign27880_e39514_d_n12, assign27880_e39514_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard888 == 0.0)) {
        let assign27880_e39512: f64 = (locals.var_chi__blk818 - 1.0);
        (assign27880_e39512, locals.var_chi__blk818_dn0, locals.var_chi__blk818_dn2, locals.var_chi__blk818_dn6, locals.var_chi__blk818_dn7, locals.var_chi__blk818_dn10, locals.var_chi__blk818_dn11, locals.var_chi__blk818_dn12, locals.var_chi__blk818_dn17,)
    } else {
        (locals.var_xi0__blk851, locals.var_xi0__blk851_dn0, locals.var_xi0__blk851_dn2, locals.var_xi0__blk851_dn6, locals.var_xi0__blk851_dn7, locals.var_xi0__blk851_dn10, locals.var_xi0__blk851_dn11, locals.var_xi0__blk851_dn12, locals.var_xi0__blk851_dn17,)
    }
};
        locals.var_xi0__blk851 = assign27880_e39514;
        locals.var_xi0__blk851_dn0 = assign27880_e39514_d_n0;
        locals.var_xi0__blk851_dn2 = assign27880_e39514_d_n2;
        locals.var_xi0__blk851_dn6 = assign27880_e39514_d_n6;
        locals.var_xi0__blk851_dn7 = assign27880_e39514_d_n7;
        locals.var_xi0__blk851_dn10 = assign27880_e39514_d_n10;
        locals.var_xi0__blk851_dn11 = assign27880_e39514_d_n11;
        locals.var_xi0__blk851_dn12 = assign27880_e39514_d_n12;
        locals.var_xi0__blk851_dn17 = assign27880_e39514_d_n17;
        locals.var_xi0__blk851_rv = 0.0;

        let (assign27890_e39531, assign27890_e39531_d_n0, assign27890_e39531_d_n2, assign27890_e39531_d_n6, assign27890_e39531_d_n7, assign27890_e39531_d_n10, assign27890_e39531_d_n11, assign27890_e39531_d_n12, assign27890_e39531_d_n17,) = {
    if ((((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) && (locals.var_guard888 == 0.0)) {
        let assign27890_e39529: f64 = (locals.var_xi0__blk851).sqrt();
        (assign27890_e39529, (locals.var_xi0__blk851_dn0 / (2.0 * assign27890_e39529)), (locals.var_xi0__blk851_dn2 / (2.0 * assign27890_e39529)), (locals.var_xi0__blk851_dn6 / (2.0 * assign27890_e39529)), (locals.var_xi0__blk851_dn7 / (2.0 * assign27890_e39529)), (locals.var_xi0__blk851_dn10 / (2.0 * assign27890_e39529)), (locals.var_xi0__blk851_dn11 / (2.0 * assign27890_e39529)), (locals.var_xi0__blk851_dn12 / (2.0 * assign27890_e39529)), (locals.var_xi0__blk851_dn17 / (2.0 * assign27890_e39529)),)
    } else {
        (locals.var_xi0p12__blk852, locals.var_xi0p12__blk852_dn0, locals.var_xi0p12__blk852_dn2, locals.var_xi0p12__blk852_dn6, locals.var_xi0p12__blk852_dn7, locals.var_xi0p12__blk852_dn10, locals.var_xi0p12__blk852_dn11, locals.var_xi0p12__blk852_dn12, locals.var_xi0p12__blk852_dn17,)
    }
};
        locals.var_xi0p12__blk852 = assign27890_e39531;
        locals.var_xi0p12__blk852_dn0 = assign27890_e39531_d_n0;
        locals.var_xi0p12__blk852_dn2 = assign27890_e39531_d_n2;
        locals.var_xi0p12__blk852_dn6 = assign27890_e39531_d_n6;
        locals.var_xi0p12__blk852_dn7 = assign27890_e39531_d_n7;
        locals.var_xi0p12__blk852_dn10 = assign27890_e39531_d_n10;
        locals.var_xi0p12__blk852_dn11 = assign27890_e39531_d_n11;
        locals.var_xi0p12__blk852_dn12 = assign27890_e39531_d_n12;
        locals.var_xi0p12__blk852_dn17 = assign27890_e39531_d_n17;
        locals.var_xi0p12__blk852_rv = 0.0;

        let (assign27900_e39546, assign27900_e39546_d_n0, assign27900_e39546_d_n2, assign27900_e39546_d_n6, assign27900_e39546_d_n7, assign27900_e39546_d_n10, assign27900_e39546_d_n11, assign27900_e39546_d_n12, assign27900_e39546_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        let assign27900_e39544: f64 = (locals.var_cnst0over * locals.var_xi0p12__blk852);
        (assign27900_e39544, ((locals.var_cnst0over_dn0 * locals.var_xi0p12__blk852) + (locals.var_cnst0over * locals.var_xi0p12__blk852_dn0)), ((locals.var_cnst0over_dn2 * locals.var_xi0p12__blk852) + (locals.var_cnst0over * locals.var_xi0p12__blk852_dn2)), ((locals.var_cnst0over_dn6 * locals.var_xi0p12__blk852) + (locals.var_cnst0over * locals.var_xi0p12__blk852_dn6)), ((locals.var_cnst0over_dn7 * locals.var_xi0p12__blk852) + (locals.var_cnst0over * locals.var_xi0p12__blk852_dn7)), ((locals.var_cnst0over_dn10 * locals.var_xi0p12__blk852) + (locals.var_cnst0over * locals.var_xi0p12__blk852_dn10)), ((locals.var_cnst0over_dn11 * locals.var_xi0p12__blk852) + (locals.var_cnst0over * locals.var_xi0p12__blk852_dn11)), ((locals.var_cnst0over_dn12 * locals.var_xi0p12__blk852) + (locals.var_cnst0over * locals.var_xi0p12__blk852_dn12)), ((locals.var_cnst0over_dn17 * locals.var_xi0p12__blk852) + (locals.var_cnst0over * locals.var_xi0p12__blk852_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign27900_e39546;
        locals.var_qbuld_dn0 = assign27900_e39546_d_n0;
        locals.var_qbuld_dn2 = assign27900_e39546_d_n2;
        locals.var_qbuld_dn6 = assign27900_e39546_d_n6;
        locals.var_qbuld_dn7 = assign27900_e39546_d_n7;
        locals.var_qbuld_dn10 = assign27900_e39546_d_n10;
        locals.var_qbuld_dn11 = assign27900_e39546_d_n11;
        locals.var_qbuld_dn12 = assign27900_e39546_d_n12;
        locals.var_qbuld_dn17 = assign27900_e39546_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign27910_e39563, assign27910_e39563_d_n0, assign27910_e39563_d_n2, assign27910_e39563_d_n6, assign27910_e39563_d_n7, assign27910_e39563_d_n10, assign27910_e39563_d_n11, assign27910_e39563_d_n12, assign27910_e39563_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        let assign27910_e39560: f64 = (locals.var_fs02__blk844 + locals.var_xi0p12__blk852);
        let assign27910_e39561: f64 = (1.0 / assign27910_e39560);
        (assign27910_e39561, (-((locals.var_fs02__blk844_dn0 + locals.var_xi0p12__blk852_dn0) / (assign27910_e39560 * assign27910_e39560))), (-((locals.var_fs02__blk844_dn2 + locals.var_xi0p12__blk852_dn2) / (assign27910_e39560 * assign27910_e39560))), (-((locals.var_fs02__blk844_dn6 + locals.var_xi0p12__blk852_dn6) / (assign27910_e39560 * assign27910_e39560))), (-((locals.var_fs02__blk844_dn7 + locals.var_xi0p12__blk852_dn7) / (assign27910_e39560 * assign27910_e39560))), (-((locals.var_fs02__blk844_dn10 + locals.var_xi0p12__blk852_dn10) / (assign27910_e39560 * assign27910_e39560))), (-((locals.var_fs02__blk844_dn11 + locals.var_xi0p12__blk852_dn11) / (assign27910_e39560 * assign27910_e39560))), (-((locals.var_fs02__blk844_dn12 + locals.var_xi0p12__blk852_dn12) / (assign27910_e39560 * assign27910_e39560))), (-((locals.var_fs02__blk844_dn17 + locals.var_xi0p12__blk852_dn17) / (assign27910_e39560 * assign27910_e39560))),)
    } else {
        (locals.var_t1__blk775, locals.var_t1__blk775_dn0, locals.var_t1__blk775_dn2, locals.var_t1__blk775_dn6, locals.var_t1__blk775_dn7, locals.var_t1__blk775_dn10, locals.var_t1__blk775_dn11, locals.var_t1__blk775_dn12, locals.var_t1__blk775_dn17,)
    }
};
        locals.var_t1__blk775 = assign27910_e39563;
        locals.var_t1__blk775_dn0 = assign27910_e39563_d_n0;
        locals.var_t1__blk775_dn2 = assign27910_e39563_d_n2;
        locals.var_t1__blk775_dn6 = assign27910_e39563_d_n6;
        locals.var_t1__blk775_dn7 = assign27910_e39563_d_n7;
        locals.var_t1__blk775_dn10 = assign27910_e39563_d_n10;
        locals.var_t1__blk775_dn11 = assign27910_e39563_d_n11;
        locals.var_t1__blk775_dn12 = assign27910_e39563_d_n12;
        locals.var_t1__blk775_dn17 = assign27910_e39563_d_n17;
        locals.var_t1__blk775_rv = 0.0;

        let (assign27920_e39580, assign27920_e39580_d_n0, assign27920_e39580_d_n2, assign27920_e39580_d_n6, assign27920_e39580_d_n7, assign27920_e39580_d_n10, assign27920_e39580_d_n11, assign27920_e39580_d_n12, assign27920_e39580_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        let assign27920_e39576: f64 = (locals.var_cnst0over * locals.var_fs01__blk840);
        let assign27920_e39578: f64 = (assign27920_e39576 * locals.var_t1__blk775);
        (assign27920_e39578, ((((locals.var_cnst0over_dn0 * locals.var_fs01__blk840) + (locals.var_cnst0over * locals.var_fs01__blk840_dn0)) * locals.var_t1__blk775) + (assign27920_e39576 * locals.var_t1__blk775_dn0)), ((((locals.var_cnst0over_dn2 * locals.var_fs01__blk840) + (locals.var_cnst0over * locals.var_fs01__blk840_dn2)) * locals.var_t1__blk775) + (assign27920_e39576 * locals.var_t1__blk775_dn2)), ((((locals.var_cnst0over_dn6 * locals.var_fs01__blk840) + (locals.var_cnst0over * locals.var_fs01__blk840_dn6)) * locals.var_t1__blk775) + (assign27920_e39576 * locals.var_t1__blk775_dn6)), ((((locals.var_cnst0over_dn7 * locals.var_fs01__blk840) + (locals.var_cnst0over * locals.var_fs01__blk840_dn7)) * locals.var_t1__blk775) + (assign27920_e39576 * locals.var_t1__blk775_dn7)), ((((locals.var_cnst0over_dn10 * locals.var_fs01__blk840) + (locals.var_cnst0over * locals.var_fs01__blk840_dn10)) * locals.var_t1__blk775) + (assign27920_e39576 * locals.var_t1__blk775_dn10)), ((((locals.var_cnst0over_dn11 * locals.var_fs01__blk840) + (locals.var_cnst0over * locals.var_fs01__blk840_dn11)) * locals.var_t1__blk775) + (assign27920_e39576 * locals.var_t1__blk775_dn11)), ((((locals.var_cnst0over_dn12 * locals.var_fs01__blk840) + (locals.var_cnst0over * locals.var_fs01__blk840_dn12)) * locals.var_t1__blk775) + (assign27920_e39576 * locals.var_t1__blk775_dn12)), ((((locals.var_cnst0over_dn17 * locals.var_fs01__blk840) + (locals.var_cnst0over * locals.var_fs01__blk840_dn17)) * locals.var_t1__blk775) + (assign27920_e39576 * locals.var_t1__blk775_dn17)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign27920_e39580;
        locals.var_qiuld_dn0 = assign27920_e39580_d_n0;
        locals.var_qiuld_dn2 = assign27920_e39580_d_n2;
        locals.var_qiuld_dn6 = assign27920_e39580_d_n6;
        locals.var_qiuld_dn7 = assign27920_e39580_d_n7;
        locals.var_qiuld_dn10 = assign27920_e39580_d_n10;
        locals.var_qiuld_dn11 = assign27920_e39580_d_n11;
        locals.var_qiuld_dn12 = assign27920_e39580_d_n12;
        locals.var_qiuld_dn17 = assign27920_e39580_d_n17;
        locals.var_qiuld_rv = 0.0;

        let (assign27930_e39595, assign27930_e39595_d_n0, assign27930_e39595_d_n2, assign27930_e39595_d_n6, assign27930_e39595_d_n7, assign27930_e39595_d_n10, assign27930_e39595_d_n11, assign27930_e39595_d_n12, assign27930_e39595_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard881 != 0.0)) {
        let assign27930_e39593: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign27930_e39593, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), (locals.var_qbuld_dn17 + locals.var_qiuld_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign27930_e39595;
        locals.var_qsuld_dn0 = assign27930_e39595_d_n0;
        locals.var_qsuld_dn2 = assign27930_e39595_d_n2;
        locals.var_qsuld_dn6 = assign27930_e39595_d_n6;
        locals.var_qsuld_dn7 = assign27930_e39595_d_n7;
        locals.var_qsuld_dn10 = assign27930_e39595_d_n10;
        locals.var_qsuld_dn11 = assign27930_e39595_d_n11;
        locals.var_qsuld_dn12 = assign27930_e39595_d_n12;
        locals.var_qsuld_dn17 = assign27930_e39595_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign27940_e39605, assign27940_e39605_d_n0, assign27940_e39605_d_n2, assign27940_e39605_d_n6, assign27940_e39605_d_n7, assign27940_e39605_d_n10, assign27940_e39605_d_n11, assign27940_e39605_d_n12, assign27940_e39605_d_n17,) = {
    if (((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign27940_e39603: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign27940_e39603, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), (locals.var_qsuld_dn17 - locals.var_qbuld_dn17),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign27940_e39605;
        locals.var_qiuld_dn0 = assign27940_e39605_d_n0;
        locals.var_qiuld_dn2 = assign27940_e39605_d_n2;
        locals.var_qiuld_dn6 = assign27940_e39605_d_n6;
        locals.var_qiuld_dn7 = assign27940_e39605_d_n7;
        locals.var_qiuld_dn10 = assign27940_e39605_d_n10;
        locals.var_qiuld_dn11 = assign27940_e39605_d_n11;
        locals.var_qiuld_dn12 = assign27940_e39605_d_n12;
        locals.var_qiuld_dn17 = assign27940_e39605_d_n17;
        locals.var_qiuld_rv = 0.0;

        let assign27950_e39608: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard890 = assign27950_e39608;
        locals.var_guard890_rv = 0.0;

        let assign27960_e39611: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard891 = assign27960_e39611;
        locals.var_guard891_rv = 0.0;

        let (assign27970_e39626, assign27970_e39626_d_n0, assign27970_e39626_d_n2, assign27970_e39626_d_n6, assign27970_e39626_d_n7, assign27970_e39626_d_n10, assign27970_e39626_d_n11, assign27970_e39626_d_n12, assign27970_e39626_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard890 != 0.0)) && (locals.var_flg_ovloops != 0.0)) {
        let assign27970_e39622: f64 = (-locals.var_uc_areabt);
        let assign27970_e39624: f64 = (assign27970_e39622 * locals.var_qsuld);
        (assign27970_e39624, (assign27970_e39622 * locals.var_qsuld_dn0), (assign27970_e39622 * locals.var_qsuld_dn2), (assign27970_e39622 * locals.var_qsuld_dn6), (assign27970_e39622 * locals.var_qsuld_dn7), (assign27970_e39622 * locals.var_qsuld_dn10), (assign27970_e39622 * locals.var_qsuld_dn11), (assign27970_e39622 * locals.var_qsuld_dn12), (assign27970_e39622 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_sus, locals.var_qbody_bt_p_sus_dn0, locals.var_qbody_bt_p_sus_dn2, locals.var_qbody_bt_p_sus_dn6, locals.var_qbody_bt_p_sus_dn7, locals.var_qbody_bt_p_sus_dn10, locals.var_qbody_bt_p_sus_dn11, locals.var_qbody_bt_p_sus_dn12, locals.var_qbody_bt_p_sus_dn17,)
    }
};
        locals.var_qbody_bt_p_sus = assign27970_e39626;
        locals.var_qbody_bt_p_sus_dn0 = assign27970_e39626_d_n0;
        locals.var_qbody_bt_p_sus_dn2 = assign27970_e39626_d_n2;
        locals.var_qbody_bt_p_sus_dn6 = assign27970_e39626_d_n6;
        locals.var_qbody_bt_p_sus_dn7 = assign27970_e39626_d_n7;
        locals.var_qbody_bt_p_sus_dn10 = assign27970_e39626_d_n10;
        locals.var_qbody_bt_p_sus_dn11 = assign27970_e39626_d_n11;
        locals.var_qbody_bt_p_sus_dn12 = assign27970_e39626_d_n12;
        locals.var_qbody_bt_p_sus_dn17 = assign27970_e39626_d_n17;
        locals.var_qbody_bt_p_sus_rv = 0.0;

        let (assign27980_e39641, assign27980_e39641_d_n0, assign27980_e39641_d_n2, assign27980_e39641_d_n6, assign27980_e39641_d_n7, assign27980_e39641_d_n10, assign27980_e39641_d_n11, assign27980_e39641_d_n12, assign27980_e39641_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard890 != 0.0)) && (locals.var_flg_ovloops != 0.0)) {
        let assign27980_e39637: f64 = (-locals.var_uc_areabt);
        let assign27980_e39639: f64 = (assign27980_e39637 * locals.var_qiuld);
        (assign27980_e39639, (assign27980_e39637 * locals.var_qiuld_dn0), (assign27980_e39637 * locals.var_qiuld_dn2), (assign27980_e39637 * locals.var_qiuld_dn6), (assign27980_e39637 * locals.var_qiuld_dn7), (assign27980_e39637 * locals.var_qiuld_dn10), (assign27980_e39637 * locals.var_qiuld_dn11), (assign27980_e39637 * locals.var_qiuld_dn12), (assign27980_e39637 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_ius, locals.var_qbody_bt_p_ius_dn0, locals.var_qbody_bt_p_ius_dn2, locals.var_qbody_bt_p_ius_dn6, locals.var_qbody_bt_p_ius_dn7, locals.var_qbody_bt_p_ius_dn10, locals.var_qbody_bt_p_ius_dn11, locals.var_qbody_bt_p_ius_dn12, locals.var_qbody_bt_p_ius_dn17,)
    }
};
        locals.var_qbody_bt_p_ius = assign27980_e39641;
        locals.var_qbody_bt_p_ius_dn0 = assign27980_e39641_d_n0;
        locals.var_qbody_bt_p_ius_dn2 = assign27980_e39641_d_n2;
        locals.var_qbody_bt_p_ius_dn6 = assign27980_e39641_d_n6;
        locals.var_qbody_bt_p_ius_dn7 = assign27980_e39641_d_n7;
        locals.var_qbody_bt_p_ius_dn10 = assign27980_e39641_d_n10;
        locals.var_qbody_bt_p_ius_dn11 = assign27980_e39641_d_n11;
        locals.var_qbody_bt_p_ius_dn12 = assign27980_e39641_d_n12;
        locals.var_qbody_bt_p_ius_dn17 = assign27980_e39641_d_n17;
        locals.var_qbody_bt_p_ius_rv = 0.0;

        let (assign27990_e39656, assign27990_e39656_d_n0, assign27990_e39656_d_n2, assign27990_e39656_d_n6, assign27990_e39656_d_n7, assign27990_e39656_d_n10, assign27990_e39656_d_n11, assign27990_e39656_d_n12, assign27990_e39656_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard890 != 0.0)) && (locals.var_flg_ovloopd != 0.0)) {
        let assign27990_e39652: f64 = (-locals.var_uc_areabt);
        let assign27990_e39654: f64 = (assign27990_e39652 * locals.var_qsuld);
        (assign27990_e39654, (assign27990_e39652 * locals.var_qsuld_dn0), (assign27990_e39652 * locals.var_qsuld_dn2), (assign27990_e39652 * locals.var_qsuld_dn6), (assign27990_e39652 * locals.var_qsuld_dn7), (assign27990_e39652 * locals.var_qsuld_dn10), (assign27990_e39652 * locals.var_qsuld_dn11), (assign27990_e39652 * locals.var_qsuld_dn12), (assign27990_e39652 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_sud, locals.var_qbody_bt_p_sud_dn0, locals.var_qbody_bt_p_sud_dn2, locals.var_qbody_bt_p_sud_dn6, locals.var_qbody_bt_p_sud_dn7, locals.var_qbody_bt_p_sud_dn10, locals.var_qbody_bt_p_sud_dn11, locals.var_qbody_bt_p_sud_dn12, locals.var_qbody_bt_p_sud_dn17,)
    }
};
        locals.var_qbody_bt_p_sud = assign27990_e39656;
        locals.var_qbody_bt_p_sud_dn0 = assign27990_e39656_d_n0;
        locals.var_qbody_bt_p_sud_dn2 = assign27990_e39656_d_n2;
        locals.var_qbody_bt_p_sud_dn6 = assign27990_e39656_d_n6;
        locals.var_qbody_bt_p_sud_dn7 = assign27990_e39656_d_n7;
        locals.var_qbody_bt_p_sud_dn10 = assign27990_e39656_d_n10;
        locals.var_qbody_bt_p_sud_dn11 = assign27990_e39656_d_n11;
        locals.var_qbody_bt_p_sud_dn12 = assign27990_e39656_d_n12;
        locals.var_qbody_bt_p_sud_dn17 = assign27990_e39656_d_n17;
        locals.var_qbody_bt_p_sud_rv = 0.0;

        let (assign28000_e39671, assign28000_e39671_d_n0, assign28000_e39671_d_n2, assign28000_e39671_d_n6, assign28000_e39671_d_n7, assign28000_e39671_d_n10, assign28000_e39671_d_n11, assign28000_e39671_d_n12, assign28000_e39671_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard890 != 0.0)) && (locals.var_flg_ovloopd != 0.0)) {
        let assign28000_e39667: f64 = (-locals.var_uc_areabt);
        let assign28000_e39669: f64 = (assign28000_e39667 * locals.var_qiuld);
        (assign28000_e39669, (assign28000_e39667 * locals.var_qiuld_dn0), (assign28000_e39667 * locals.var_qiuld_dn2), (assign28000_e39667 * locals.var_qiuld_dn6), (assign28000_e39667 * locals.var_qiuld_dn7), (assign28000_e39667 * locals.var_qiuld_dn10), (assign28000_e39667 * locals.var_qiuld_dn11), (assign28000_e39667 * locals.var_qiuld_dn12), (assign28000_e39667 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_iud, locals.var_qbody_bt_p_iud_dn0, locals.var_qbody_bt_p_iud_dn2, locals.var_qbody_bt_p_iud_dn6, locals.var_qbody_bt_p_iud_dn7, locals.var_qbody_bt_p_iud_dn10, locals.var_qbody_bt_p_iud_dn11, locals.var_qbody_bt_p_iud_dn12, locals.var_qbody_bt_p_iud_dn17,)
    }
};
        locals.var_qbody_bt_p_iud = assign28000_e39671;
        locals.var_qbody_bt_p_iud_dn0 = assign28000_e39671_d_n0;
        locals.var_qbody_bt_p_iud_dn2 = assign28000_e39671_d_n2;
        locals.var_qbody_bt_p_iud_dn6 = assign28000_e39671_d_n6;
        locals.var_qbody_bt_p_iud_dn7 = assign28000_e39671_d_n7;
        locals.var_qbody_bt_p_iud_dn10 = assign28000_e39671_d_n10;
        locals.var_qbody_bt_p_iud_dn11 = assign28000_e39671_d_n11;
        locals.var_qbody_bt_p_iud_dn12 = assign28000_e39671_d_n12;
        locals.var_qbody_bt_p_iud_dn17 = assign28000_e39671_d_n17;
        locals.var_qbody_bt_p_iud_rv = 0.0;

        let (assign28010_e39689, assign28010_e39689_d_n0, assign28010_e39689_d_n2, assign28010_e39689_d_n6, assign28010_e39689_d_n7, assign28010_e39689_d_n10, assign28010_e39689_d_n11, assign28010_e39689_d_n12, assign28010_e39689_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && ((locals.var_guard891 != 0.0) && (locals.var_guard890 == 0.0))) && (locals.var_flg_ovloops != 0.0)) {
        let assign28010_e39685: f64 = (-locals.var_uc_areabt);
        let assign28010_e39687: f64 = (assign28010_e39685 * locals.var_qsuld);
        (assign28010_e39687, (assign28010_e39685 * locals.var_qsuld_dn0), (assign28010_e39685 * locals.var_qsuld_dn2), (assign28010_e39685 * locals.var_qsuld_dn6), (assign28010_e39685 * locals.var_qsuld_dn7), (assign28010_e39685 * locals.var_qsuld_dn10), (assign28010_e39685 * locals.var_qsuld_dn11), (assign28010_e39685 * locals.var_qsuld_dn12), (assign28010_e39685 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_sus, locals.var_qbody_bt_n_sus_dn0, locals.var_qbody_bt_n_sus_dn2, locals.var_qbody_bt_n_sus_dn6, locals.var_qbody_bt_n_sus_dn7, locals.var_qbody_bt_n_sus_dn10, locals.var_qbody_bt_n_sus_dn11, locals.var_qbody_bt_n_sus_dn12, locals.var_qbody_bt_n_sus_dn17,)
    }
};
        locals.var_qbody_bt_n_sus = assign28010_e39689;
        locals.var_qbody_bt_n_sus_dn0 = assign28010_e39689_d_n0;
        locals.var_qbody_bt_n_sus_dn2 = assign28010_e39689_d_n2;
        locals.var_qbody_bt_n_sus_dn6 = assign28010_e39689_d_n6;
        locals.var_qbody_bt_n_sus_dn7 = assign28010_e39689_d_n7;
        locals.var_qbody_bt_n_sus_dn10 = assign28010_e39689_d_n10;
        locals.var_qbody_bt_n_sus_dn11 = assign28010_e39689_d_n11;
        locals.var_qbody_bt_n_sus_dn12 = assign28010_e39689_d_n12;
        locals.var_qbody_bt_n_sus_dn17 = assign28010_e39689_d_n17;
        locals.var_qbody_bt_n_sus_rv = 0.0;

        let (assign28020_e39707, assign28020_e39707_d_n0, assign28020_e39707_d_n2, assign28020_e39707_d_n6, assign28020_e39707_d_n7, assign28020_e39707_d_n10, assign28020_e39707_d_n11, assign28020_e39707_d_n12, assign28020_e39707_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && ((locals.var_guard891 != 0.0) && (locals.var_guard890 == 0.0))) && (locals.var_flg_ovloops != 0.0)) {
        let assign28020_e39703: f64 = (-locals.var_uc_areabt);
        let assign28020_e39705: f64 = (assign28020_e39703 * locals.var_qiuld);
        (assign28020_e39705, (assign28020_e39703 * locals.var_qiuld_dn0), (assign28020_e39703 * locals.var_qiuld_dn2), (assign28020_e39703 * locals.var_qiuld_dn6), (assign28020_e39703 * locals.var_qiuld_dn7), (assign28020_e39703 * locals.var_qiuld_dn10), (assign28020_e39703 * locals.var_qiuld_dn11), (assign28020_e39703 * locals.var_qiuld_dn12), (assign28020_e39703 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_ius, locals.var_qbody_bt_n_ius_dn0, locals.var_qbody_bt_n_ius_dn2, locals.var_qbody_bt_n_ius_dn6, locals.var_qbody_bt_n_ius_dn7, locals.var_qbody_bt_n_ius_dn10, locals.var_qbody_bt_n_ius_dn11, locals.var_qbody_bt_n_ius_dn12, locals.var_qbody_bt_n_ius_dn17,)
    }
};
        locals.var_qbody_bt_n_ius = assign28020_e39707;
        locals.var_qbody_bt_n_ius_dn0 = assign28020_e39707_d_n0;
        locals.var_qbody_bt_n_ius_dn2 = assign28020_e39707_d_n2;
        locals.var_qbody_bt_n_ius_dn6 = assign28020_e39707_d_n6;
        locals.var_qbody_bt_n_ius_dn7 = assign28020_e39707_d_n7;
        locals.var_qbody_bt_n_ius_dn10 = assign28020_e39707_d_n10;
        locals.var_qbody_bt_n_ius_dn11 = assign28020_e39707_d_n11;
        locals.var_qbody_bt_n_ius_dn12 = assign28020_e39707_d_n12;
        locals.var_qbody_bt_n_ius_dn17 = assign28020_e39707_d_n17;
        locals.var_qbody_bt_n_ius_rv = 0.0;

        let (assign28030_e39725, assign28030_e39725_d_n0, assign28030_e39725_d_n2, assign28030_e39725_d_n6, assign28030_e39725_d_n7, assign28030_e39725_d_n10, assign28030_e39725_d_n11, assign28030_e39725_d_n12, assign28030_e39725_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && ((locals.var_guard891 != 0.0) && (locals.var_guard890 == 0.0))) && (locals.var_flg_ovloopd != 0.0)) {
        let assign28030_e39721: f64 = (-locals.var_uc_areabt);
        let assign28030_e39723: f64 = (assign28030_e39721 * locals.var_qsuld);
        (assign28030_e39723, (assign28030_e39721 * locals.var_qsuld_dn0), (assign28030_e39721 * locals.var_qsuld_dn2), (assign28030_e39721 * locals.var_qsuld_dn6), (assign28030_e39721 * locals.var_qsuld_dn7), (assign28030_e39721 * locals.var_qsuld_dn10), (assign28030_e39721 * locals.var_qsuld_dn11), (assign28030_e39721 * locals.var_qsuld_dn12), (assign28030_e39721 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_sud, locals.var_qbody_bt_n_sud_dn0, locals.var_qbody_bt_n_sud_dn2, locals.var_qbody_bt_n_sud_dn6, locals.var_qbody_bt_n_sud_dn7, locals.var_qbody_bt_n_sud_dn10, locals.var_qbody_bt_n_sud_dn11, locals.var_qbody_bt_n_sud_dn12, locals.var_qbody_bt_n_sud_dn17,)
    }
};
        locals.var_qbody_bt_n_sud = assign28030_e39725;
        locals.var_qbody_bt_n_sud_dn0 = assign28030_e39725_d_n0;
        locals.var_qbody_bt_n_sud_dn2 = assign28030_e39725_d_n2;
        locals.var_qbody_bt_n_sud_dn6 = assign28030_e39725_d_n6;
        locals.var_qbody_bt_n_sud_dn7 = assign28030_e39725_d_n7;
        locals.var_qbody_bt_n_sud_dn10 = assign28030_e39725_d_n10;
        locals.var_qbody_bt_n_sud_dn11 = assign28030_e39725_d_n11;
        locals.var_qbody_bt_n_sud_dn12 = assign28030_e39725_d_n12;
        locals.var_qbody_bt_n_sud_dn17 = assign28030_e39725_d_n17;
        locals.var_qbody_bt_n_sud_rv = 0.0;

        let (assign28040_e39743, assign28040_e39743_d_n0, assign28040_e39743_d_n2, assign28040_e39743_d_n6, assign28040_e39743_d_n7, assign28040_e39743_d_n10, assign28040_e39743_d_n11, assign28040_e39743_d_n12, assign28040_e39743_d_n17,) = {
    if (((((locals.var_guard773 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard855 != 0.0)) && ((locals.var_guard891 != 0.0) && (locals.var_guard890 == 0.0))) && (locals.var_flg_ovloopd != 0.0)) {
        let assign28040_e39739: f64 = (-locals.var_uc_areabt);
        let assign28040_e39741: f64 = (assign28040_e39739 * locals.var_qiuld);
        (assign28040_e39741, (assign28040_e39739 * locals.var_qiuld_dn0), (assign28040_e39739 * locals.var_qiuld_dn2), (assign28040_e39739 * locals.var_qiuld_dn6), (assign28040_e39739 * locals.var_qiuld_dn7), (assign28040_e39739 * locals.var_qiuld_dn10), (assign28040_e39739 * locals.var_qiuld_dn11), (assign28040_e39739 * locals.var_qiuld_dn12), (assign28040_e39739 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_iud, locals.var_qbody_bt_n_iud_dn0, locals.var_qbody_bt_n_iud_dn2, locals.var_qbody_bt_n_iud_dn6, locals.var_qbody_bt_n_iud_dn7, locals.var_qbody_bt_n_iud_dn10, locals.var_qbody_bt_n_iud_dn11, locals.var_qbody_bt_n_iud_dn12, locals.var_qbody_bt_n_iud_dn17,)
    }
};
        locals.var_qbody_bt_n_iud = assign28040_e39743;
        locals.var_qbody_bt_n_iud_dn0 = assign28040_e39743_d_n0;
        locals.var_qbody_bt_n_iud_dn2 = assign28040_e39743_d_n2;
        locals.var_qbody_bt_n_iud_dn6 = assign28040_e39743_d_n6;
        locals.var_qbody_bt_n_iud_dn7 = assign28040_e39743_d_n7;
        locals.var_qbody_bt_n_iud_dn10 = assign28040_e39743_d_n10;
        locals.var_qbody_bt_n_iud_dn11 = assign28040_e39743_d_n11;
        locals.var_qbody_bt_n_iud_dn12 = assign28040_e39743_d_n12;
        locals.var_qbody_bt_n_iud_dn17 = assign28040_e39743_d_n17;
        locals.var_qbody_bt_n_iud_rv = 0.0;

        locals.var_aclm = p.p189;
        locals.var_aclm_rv = 0.0;

        let assign28060_e39747: f64 = if locals.var_flg_noqi != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard894 = assign28060_e39747;
        locals.var_guard894_rv = 0.0;

        let (assign28070_e39753, assign28070_e39753_d_n0, assign28070_e39753_d_n2, assign28070_e39753_d_n6, assign28070_e39753_d_n7, assign28070_e39753_d_n10, assign28070_e39753_d_n11, assign28070_e39753_d_n12, assign28070_e39753_d_n17,) = {
    if (locals.var_guard894 != 0.0) {
        let assign28070_e39751: f64 = (locals.var_vds + locals.var_ps0);
        (assign28070_e39751, (locals.var_vds_dn0 + locals.var_ps0_dn0), (locals.var_vds_dn2 + locals.var_ps0_dn2), (locals.var_vds_dn6 + locals.var_ps0_dn6), (locals.var_vds_dn7 + locals.var_ps0_dn7), (locals.var_vds_dn10 + locals.var_ps0_dn10), (locals.var_vds_dn11 + locals.var_ps0_dn11), (locals.var_vds_dn12 + locals.var_ps0_dn12), (locals.var_vds_dn17 + locals.var_ps0_dn17),)
    } else {
        (locals.var_t2__blk893, locals.var_t2__blk893_dn0, locals.var_t2__blk893_dn2, locals.var_t2__blk893_dn6, locals.var_t2__blk893_dn7, locals.var_t2__blk893_dn10, locals.var_t2__blk893_dn11, locals.var_t2__blk893_dn12, locals.var_t2__blk893_dn17,)
    }
};
        locals.var_t2__blk893 = assign28070_e39753;
        locals.var_t2__blk893_dn0 = assign28070_e39753_d_n0;
        locals.var_t2__blk893_dn2 = assign28070_e39753_d_n2;
        locals.var_t2__blk893_dn6 = assign28070_e39753_d_n6;
        locals.var_t2__blk893_dn7 = assign28070_e39753_d_n7;
        locals.var_t2__blk893_dn10 = assign28070_e39753_d_n10;
        locals.var_t2__blk893_dn11 = assign28070_e39753_d_n11;
        locals.var_t2__blk893_dn12 = assign28070_e39753_d_n12;
        locals.var_t2__blk893_dn17 = assign28070_e39753_d_n17;
        locals.var_t2__blk893_rv = 0.0;

        let (assign28080_e39765, assign28080_e39765_d_n0, assign28080_e39765_d_n2, assign28080_e39765_d_n6, assign28080_e39765_d_n7, assign28080_e39765_d_n10, assign28080_e39765_d_n11, assign28080_e39765_d_n12, assign28080_e39765_d_n17,) = {
    if (locals.var_guard894 != 0.0) {
        let assign28080_e39757: f64 = (locals.var_aclm * locals.var_t2__blk893);
        let assign28080_e39760: f64 = (1.0 - locals.var_aclm);
        let assign28080_e39762: f64 = (assign28080_e39760 * locals.var_psl);
        let assign28080_e39763: f64 = (assign28080_e39757 + assign28080_e39762);
        (assign28080_e39763, ((locals.var_aclm * locals.var_t2__blk893_dn0) + (assign28080_e39760 * locals.var_psl_dn0)), ((locals.var_aclm * locals.var_t2__blk893_dn2) + (assign28080_e39760 * locals.var_psl_dn2)), ((locals.var_aclm * locals.var_t2__blk893_dn6) + (assign28080_e39760 * locals.var_psl_dn6)), ((locals.var_aclm * locals.var_t2__blk893_dn7) + (assign28080_e39760 * locals.var_psl_dn7)), ((locals.var_aclm * locals.var_t2__blk893_dn10) + (assign28080_e39760 * locals.var_psl_dn10)), ((locals.var_aclm * locals.var_t2__blk893_dn11) + (assign28080_e39760 * locals.var_psl_dn11)), ((locals.var_aclm * locals.var_t2__blk893_dn12) + (assign28080_e39760 * locals.var_psl_dn12)), ((locals.var_aclm * locals.var_t2__blk893_dn17) + (assign28080_e39760 * locals.var_psl_dn17)),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign28080_e39765;
        locals.var_psdl_dn0 = assign28080_e39765_d_n0;
        locals.var_psdl_dn2 = assign28080_e39765_d_n2;
        locals.var_psdl_dn6 = assign28080_e39765_d_n6;
        locals.var_psdl_dn7 = assign28080_e39765_d_n7;
        locals.var_psdl_dn10 = assign28080_e39765_d_n10;
        locals.var_psdl_dn11 = assign28080_e39765_d_n11;
        locals.var_psdl_dn12 = assign28080_e39765_d_n12;
        locals.var_psdl_dn17 = assign28080_e39765_d_n17;
        locals.var_psdl_rv = 0.0;

        let assign28090_e39768: f64 = if p.p64 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard895 = assign28090_e39768;
        locals.var_guard895_rv = 0.0;

        let (assign28100_e39774, assign28100_e39774_d_n0, assign28100_e39774_d_n2, assign28100_e39774_d_n6, assign28100_e39774_d_n7, assign28100_e39774_d_n10, assign28100_e39774_d_n11, assign28100_e39774_d_n12, assign28100_e39774_d_n17,) = {
    if ((locals.var_guard894 != 0.0) && (locals.var_guard895 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn12, locals.var_ec_dn17,)
    }
};
        locals.var_ec = assign28100_e39774;
        locals.var_ec_dn0 = assign28100_e39774_d_n0;
        locals.var_ec_dn2 = assign28100_e39774_d_n2;
        locals.var_ec_dn6 = assign28100_e39774_d_n6;
        locals.var_ec_dn7 = assign28100_e39774_d_n7;
        locals.var_ec_dn10 = assign28100_e39774_d_n10;
        locals.var_ec_dn11 = assign28100_e39774_d_n11;
        locals.var_ec_dn12 = assign28100_e39774_d_n12;
        locals.var_ec_dn17 = assign28100_e39774_d_n17;
        locals.var_ec_rv = 0.0;

        let assign28110_e39778: f64 = (locals.var_ps0 + locals.var_vds);
        let assign28110_e39781: f64 = (10.0 * 2.220446049250313e-16);
        let assign28110_e39782: f64 = (assign28110_e39778 - assign28110_e39781);
        let assign28110_e39783: f64 = if locals.var_psdl > assign28110_e39782 { 1.0 } else { 0.0 };
        locals.var_guard896 = assign28110_e39783;
        locals.var_guard896_rv = 0.0;

        let (assign28120_e39795, assign28120_e39795_d_n0, assign28120_e39795_d_n2, assign28120_e39795_d_n6, assign28120_e39795_d_n7, assign28120_e39795_d_n10, assign28120_e39795_d_n11, assign28120_e39795_d_n12, assign28120_e39795_d_n17,) = {
    if ((locals.var_guard894 != 0.0) && (locals.var_guard896 != 0.0)) {
        let assign28120_e39789: f64 = (locals.var_ps0 + locals.var_vds);
        let assign28120_e39792: f64 = (10.0 * 2.220446049250313e-16);
        let assign28120_e39793: f64 = (assign28120_e39789 - assign28120_e39792);
        (assign28120_e39793, (locals.var_ps0_dn0 + locals.var_vds_dn0), (locals.var_ps0_dn2 + locals.var_vds_dn2), (locals.var_ps0_dn6 + locals.var_vds_dn6), (locals.var_ps0_dn7 + locals.var_vds_dn7), (locals.var_ps0_dn10 + locals.var_vds_dn10), (locals.var_ps0_dn11 + locals.var_vds_dn11), (locals.var_ps0_dn12 + locals.var_vds_dn12), (locals.var_ps0_dn17 + locals.var_vds_dn17),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign28120_e39795;
        locals.var_psdl_dn0 = assign28120_e39795_d_n0;
        locals.var_psdl_dn2 = assign28120_e39795_d_n2;
        locals.var_psdl_dn6 = assign28120_e39795_d_n6;
        locals.var_psdl_dn7 = assign28120_e39795_d_n7;
        locals.var_psdl_dn10 = assign28120_e39795_d_n10;
        locals.var_psdl_dn11 = assign28120_e39795_d_n11;
        locals.var_psdl_dn12 = assign28120_e39795_d_n12;
        locals.var_psdl_dn17 = assign28120_e39795_d_n17;
        locals.var_psdl_rv = 0.0;

        let assign28130_e39798: f64 = if p.p64 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard897 = assign28130_e39798;
        locals.var_guard897_rv = 0.0;

        let assign28140_e39801: f64 = if locals.var_idd < 1e-15 { 1.0 } else { 0.0 };
        locals.var_guard898 = assign28140_e39801;
        locals.var_guard898_rv = 0.0;

        let (assign28150_e39810, assign28150_e39810_d_n0, assign28150_e39810_d_n2, assign28150_e39810_d_n6, assign28150_e39810_d_n7, assign28150_e39810_d_n10, assign28150_e39810_d_n11, assign28150_e39810_d_n12, assign28150_e39810_d_n17,) = {
    if (((locals.var_guard894 == 0.0) && (locals.var_guard897 != 0.0)) && (locals.var_guard898 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn12, locals.var_ec_dn17,)
    }
};
        locals.var_ec = assign28150_e39810;
        locals.var_ec_dn0 = assign28150_e39810_d_n0;
        locals.var_ec_dn2 = assign28150_e39810_d_n2;
        locals.var_ec_dn6 = assign28150_e39810_d_n6;
        locals.var_ec_dn7 = assign28150_e39810_d_n7;
        locals.var_ec_dn10 = assign28150_e39810_d_n10;
        locals.var_ec_dn11 = assign28150_e39810_d_n11;
        locals.var_ec_dn12 = assign28150_e39810_d_n12;
        locals.var_ec_dn17 = assign28150_e39810_d_n17;
        locals.var_ec_rv = 0.0;

        let (assign28160_e39822, assign28160_e39822_d_n10,) = {
    if (((locals.var_guard894 == 0.0) && (locals.var_guard897 != 0.0)) && (locals.var_guard898 == 0.0)) {
        let assign28160_e39820: f64 = (locals.var_beta_inv / locals.var_leff);
        (assign28160_e39820, (locals.var_beta_inv_dn10 / locals.var_leff),)
    } else {
        (locals.var_t1__blk892, locals.var_t1__blk892_dn10,)
    }
};
        locals.var_t1__blk892 = assign28160_e39822;
        locals.var_t1__blk892_dn10 = assign28160_e39822_d_n10;
        locals.var_t1__blk892_rv = 0.0;

        let (assign28170_e39834, assign28170_e39834_d_n0, assign28170_e39834_d_n2, assign28170_e39834_d_n6, assign28170_e39834_d_n7, assign28170_e39834_d_n10, assign28170_e39834_d_n11, assign28170_e39834_d_n12, assign28170_e39834_d_n17,) = {
    if (((locals.var_guard894 == 0.0) && (locals.var_guard897 != 0.0)) && (locals.var_guard898 == 0.0)) {
        let assign28170_e39832: f64 = (1.0 / locals.var_qn0);
        (assign28170_e39832, (-(locals.var_qn0_dn0 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn2 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn6 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn7 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn10 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn11 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn12 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn17 / (locals.var_qn0 * locals.var_qn0))),)
    } else {
        (locals.var_t2__blk893, locals.var_t2__blk893_dn0, locals.var_t2__blk893_dn2, locals.var_t2__blk893_dn6, locals.var_t2__blk893_dn7, locals.var_t2__blk893_dn10, locals.var_t2__blk893_dn11, locals.var_t2__blk893_dn12, locals.var_t2__blk893_dn17,)
    }
};
        locals.var_t2__blk893 = assign28170_e39834;
        locals.var_t2__blk893_dn0 = assign28170_e39834_d_n0;
        locals.var_t2__blk893_dn2 = assign28170_e39834_d_n2;
        locals.var_t2__blk893_dn6 = assign28170_e39834_d_n6;
        locals.var_t2__blk893_dn7 = assign28170_e39834_d_n7;
        locals.var_t2__blk893_dn10 = assign28170_e39834_d_n10;
        locals.var_t2__blk893_dn11 = assign28170_e39834_d_n11;
        locals.var_t2__blk893_dn12 = assign28170_e39834_d_n12;
        locals.var_t2__blk893_dn17 = assign28170_e39834_d_n17;
        locals.var_t2__blk893_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_102(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28180_e39848, assign28180_e39848_d_n0, assign28180_e39848_d_n2, assign28180_e39848_d_n6, assign28180_e39848_d_n7, assign28180_e39848_d_n10, assign28180_e39848_d_n11, assign28180_e39848_d_n12, assign28180_e39848_d_n17,) = {
    if (((locals.var_guard894 == 0.0) && (locals.var_guard897 != 0.0)) && (locals.var_guard898 == 0.0)) {
        let assign28180_e39844: f64 = (locals.var_idd * locals.var_t1__blk892);
        let assign28180_e39846: f64 = (assign28180_e39844 * locals.var_t2__blk893);
        (assign28180_e39846, (((locals.var_idd_dn0 * locals.var_t1__blk892) * locals.var_t2__blk893) + (assign28180_e39844 * locals.var_t2__blk893_dn0)), (((locals.var_idd_dn2 * locals.var_t1__blk892) * locals.var_t2__blk893) + (assign28180_e39844 * locals.var_t2__blk893_dn2)), (((locals.var_idd_dn6 * locals.var_t1__blk892) * locals.var_t2__blk893) + (assign28180_e39844 * locals.var_t2__blk893_dn6)), (((locals.var_idd_dn7 * locals.var_t1__blk892) * locals.var_t2__blk893) + (assign28180_e39844 * locals.var_t2__blk893_dn7)), ((((locals.var_idd_dn10 * locals.var_t1__blk892) + (locals.var_idd * locals.var_t1__blk892_dn10)) * locals.var_t2__blk893) + (assign28180_e39844 * locals.var_t2__blk893_dn10)), (((locals.var_idd_dn11 * locals.var_t1__blk892) * locals.var_t2__blk893) + (assign28180_e39844 * locals.var_t2__blk893_dn11)), (((locals.var_idd_dn12 * locals.var_t1__blk892) * locals.var_t2__blk893) + (assign28180_e39844 * locals.var_t2__blk893_dn12)), (((locals.var_idd_dn17 * locals.var_t1__blk892) * locals.var_t2__blk893) + (assign28180_e39844 * locals.var_t2__blk893_dn17)),)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn12, locals.var_ec_dn17,)
    }
};
        locals.var_ec = assign28180_e39848;
        locals.var_ec_dn0 = assign28180_e39848_d_n0;
        locals.var_ec_dn2 = assign28180_e39848_d_n2;
        locals.var_ec_dn6 = assign28180_e39848_d_n6;
        locals.var_ec_dn7 = assign28180_e39848_d_n7;
        locals.var_ec_dn10 = assign28180_e39848_d_n10;
        locals.var_ec_dn11 = assign28180_e39848_d_n11;
        locals.var_ec_dn12 = assign28180_e39848_d_n12;
        locals.var_ec_dn17 = assign28180_e39848_d_n17;
        locals.var_ec_rv = 0.0;

        locals.var_cox0__blk910 = locals.var_c_fox0;
        locals.var_cox0__blk910_rv = 0.0;

        let assign28200_e39852: f64 = (1.0 / locals.var_cox0__blk910);
        locals.var_cox0_inv__blk911 = assign28200_e39852;
        locals.var_cox0_inv__blk911_rv = 0.0;

        locals.var_vgbgmt__blk931 = 0.0;
        locals.var_vgbgmt__blk931_dn0 = 0.0;
        locals.var_vgbgmt__blk931_dn2 = 0.0;
        locals.var_vgbgmt__blk931_dn6 = 0.0;
        locals.var_vgbgmt__blk931_dn7 = 0.0;
        locals.var_vgbgmt__blk931_dn10 = 0.0;
        locals.var_vgbgmt__blk931_dn11 = 0.0;
        locals.var_vgbgmt__blk931_dn12 = 0.0;
        locals.var_vgbgmt__blk931_dn17 = 0.0;
        locals.var_vgbgmt__blk931_rv = 0.0;

        locals.var_fb__blk971 = 0.0;
        locals.var_fb__blk971_dn0 = 0.0;
        locals.var_fb__blk971_dn2 = 0.0;
        locals.var_fb__blk971_dn6 = 0.0;
        locals.var_fb__blk971_dn7 = 0.0;
        locals.var_fb__blk971_dn10 = 0.0;
        locals.var_fb__blk971_dn11 = 0.0;
        locals.var_fb__blk971_dn12 = 0.0;
        locals.var_fb__blk971_dn17 = 0.0;
        locals.var_fb__blk971_rv = 0.0;

        locals.var_fs01__blk969 = 0.0;
        locals.var_fs01__blk969_dn0 = 0.0;
        locals.var_fs01__blk969_dn2 = 0.0;
        locals.var_fs01__blk969_dn6 = 0.0;
        locals.var_fs01__blk969_dn7 = 0.0;
        locals.var_fs01__blk969_dn10 = 0.0;
        locals.var_fs01__blk969_dn11 = 0.0;
        locals.var_fs01__blk969_dn12 = 0.0;
        locals.var_fs01__blk969_dn17 = 0.0;
        locals.var_fs01__blk969_rv = 0.0;

        locals.var_fs02__blk973 = 0.0;
        locals.var_fs02__blk973_dn0 = 0.0;
        locals.var_fs02__blk973_dn2 = 0.0;
        locals.var_fs02__blk973_dn6 = 0.0;
        locals.var_fs02__blk973_dn7 = 0.0;
        locals.var_fs02__blk973_dn10 = 0.0;
        locals.var_fs02__blk973_dn11 = 0.0;
        locals.var_fs02__blk973_dn12 = 0.0;
        locals.var_fs02__blk973_dn17 = 0.0;
        locals.var_fs02__blk973_rv = 0.0;

        let assign28250_e39863: f64 = if ((p.p29 >= 1.0) && (p.p188 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard982 = assign28250_e39863;
        locals.var_guard982_rv = 0.0;

        let (assign28260_e39869,) = {
    if ((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) {
        (p.p171,)
    } else {
        (locals.var_cov_slp,)
    }
};
        locals.var_cov_slp = assign28260_e39869;
        locals.var_cov_slp_rv = 0.0;

        let (assign28270_e39875,) = {
    if ((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) {
        (p.p172,)
    } else {
        (locals.var_cov_mag,)
    }
};
        locals.var_cov_mag = assign28270_e39875;
        locals.var_cov_mag_rv = 0.0;

        let (assign28280_e39881, assign28280_e39881_d_n6, assign28280_e39881_d_n7, assign28280_e39881_d_n11,) = {
    if ((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn11,)
    } else {
        (locals.var_covvg, locals.var_covvg_dn6, locals.var_covvg_dn7, locals.var_covvg_dn11,)
    }
};
        locals.var_covvg = assign28280_e39881;
        locals.var_covvg_dn6 = assign28280_e39881_d_n6;
        locals.var_covvg_dn7 = assign28280_e39881_d_n7;
        locals.var_covvg_dn11 = assign28280_e39881_d_n11;
        locals.var_covvg_rv = 0.0;

        let (assign28290_e39887,) = {
    if ((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) {
        (p.p188,)
    } else {
        (locals.var_lov,)
    }
};
        locals.var_lov = assign28290_e39887;
        locals.var_lov_rv = 0.0;

        let assign28300_e39894: f64 = if ((locals.var_mks_nover == 0.0) && (p.p188 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard983 = assign28300_e39894;
        locals.var_guard983_rv = 0.0;

        let (assign28310_e39911, assign28310_e39911_d_n0, assign28310_e39911_d_n2, assign28310_e39911_d_n6, assign28310_e39911_d_n7, assign28310_e39911_d_n10, assign28310_e39911_d_n11, assign28310_e39911_d_n12, assign28310_e39911_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 != 0.0)) {
        let (assign28310_e39909,) = {
            if (p.p43 == 1.0) {
                let assign28310_e39905: f64 = (locals.var_w_dioscv * locals.var_cox0__blk910);
                (assign28310_e39905,)
            } else {
                let assign28310_e39908: f64 = (locals.var_weffcv_nf * locals.var_cox0__blk910);
                (assign28310_e39908,)
            }
        };
        (assign28310_e39909, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign28310_e39911;
        locals.var_t1__blk900_dn0 = assign28310_e39911_d_n0;
        locals.var_t1__blk900_dn2 = assign28310_e39911_d_n2;
        locals.var_t1__blk900_dn6 = assign28310_e39911_d_n6;
        locals.var_t1__blk900_dn7 = assign28310_e39911_d_n7;
        locals.var_t1__blk900_dn10 = assign28310_e39911_d_n10;
        locals.var_t1__blk900_dn11 = assign28310_e39911_d_n11;
        locals.var_t1__blk900_dn12 = assign28310_e39911_d_n12;
        locals.var_t1__blk900_dn17 = assign28310_e39911_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign28320_e39925, assign28320_e39925_d_n0, assign28320_e39925_d_n2, assign28320_e39925_d_n6, assign28320_e39925_d_n7, assign28320_e39925_d_n10, assign28320_e39925_d_n11, assign28320_e39925_d_n12, assign28320_e39925_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28320_e39919: f64 = (locals.var_cov_slp * locals.var_t1__blk900);
        let assign28320_e39922: f64 = (locals.var_cov_mag + locals.var_covvg);
        let assign28320_e39923: f64 = (assign28320_e39919 * assign28320_e39922);
        (assign28320_e39923, ((locals.var_cov_slp * locals.var_t1__blk900_dn0) * assign28320_e39922), ((locals.var_cov_slp * locals.var_t1__blk900_dn2) * assign28320_e39922), (((locals.var_cov_slp * locals.var_t1__blk900_dn6) * assign28320_e39922) + (assign28320_e39919 * locals.var_covvg_dn6)), (((locals.var_cov_slp * locals.var_t1__blk900_dn7) * assign28320_e39922) + (assign28320_e39919 * locals.var_covvg_dn7)), ((locals.var_cov_slp * locals.var_t1__blk900_dn10) * assign28320_e39922), (((locals.var_cov_slp * locals.var_t1__blk900_dn11) * assign28320_e39922) + (assign28320_e39919 * locals.var_covvg_dn11)), ((locals.var_cov_slp * locals.var_t1__blk900_dn12) * assign28320_e39922), ((locals.var_cov_slp * locals.var_t1__blk900_dn17) * assign28320_e39922),)
    } else {
        (locals.var_t4__blk903, locals.var_t4__blk903_dn0, locals.var_t4__blk903_dn2, locals.var_t4__blk903_dn6, locals.var_t4__blk903_dn7, locals.var_t4__blk903_dn10, locals.var_t4__blk903_dn11, locals.var_t4__blk903_dn12, locals.var_t4__blk903_dn17,)
    }
};
        locals.var_t4__blk903 = assign28320_e39925;
        locals.var_t4__blk903_dn0 = assign28320_e39925_d_n0;
        locals.var_t4__blk903_dn2 = assign28320_e39925_d_n2;
        locals.var_t4__blk903_dn6 = assign28320_e39925_d_n6;
        locals.var_t4__blk903_dn7 = assign28320_e39925_d_n7;
        locals.var_t4__blk903_dn10 = assign28320_e39925_d_n10;
        locals.var_t4__blk903_dn11 = assign28320_e39925_d_n11;
        locals.var_t4__blk903_dn12 = assign28320_e39925_d_n12;
        locals.var_t4__blk903_dn17 = assign28320_e39925_d_n17;
        locals.var_t4__blk903_rv = 0.0;

        let (assign28330_e39935, assign28330_e39935_d_n0, assign28330_e39935_d_n2, assign28330_e39935_d_n6, assign28330_e39935_d_n7, assign28330_e39935_d_n10, assign28330_e39935_d_n11, assign28330_e39935_d_n12, assign28330_e39935_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28330_e39933: f64 = (locals.var_lov * locals.var_t1__blk900);
        (assign28330_e39933, (locals.var_lov * locals.var_t1__blk900_dn0), (locals.var_lov * locals.var_t1__blk900_dn2), (locals.var_lov * locals.var_t1__blk900_dn6), (locals.var_lov * locals.var_t1__blk900_dn7), (locals.var_lov * locals.var_t1__blk900_dn10), (locals.var_lov * locals.var_t1__blk900_dn11), (locals.var_lov * locals.var_t1__blk900_dn12), (locals.var_lov * locals.var_t1__blk900_dn17),)
    } else {
        (locals.var_t5__blk904, locals.var_t5__blk904_dn0, locals.var_t5__blk904_dn2, locals.var_t5__blk904_dn6, locals.var_t5__blk904_dn7, locals.var_t5__blk904_dn10, locals.var_t5__blk904_dn11, locals.var_t5__blk904_dn12, locals.var_t5__blk904_dn17,)
    }
};
        locals.var_t5__blk904 = assign28330_e39935;
        locals.var_t5__blk904_dn0 = assign28330_e39935_d_n0;
        locals.var_t5__blk904_dn2 = assign28330_e39935_d_n2;
        locals.var_t5__blk904_dn6 = assign28330_e39935_d_n6;
        locals.var_t5__blk904_dn7 = assign28330_e39935_d_n7;
        locals.var_t5__blk904_dn10 = assign28330_e39935_d_n10;
        locals.var_t5__blk904_dn11 = assign28330_e39935_d_n11;
        locals.var_t5__blk904_dn12 = assign28330_e39935_d_n12;
        locals.var_t5__blk904_dn17 = assign28330_e39935_d_n17;
        locals.var_t5__blk904_rv = 0.0;

        let (assign28340_e39943, assign28340_e39943_d_n0, assign28340_e39943_d_n2, assign28340_e39943_d_n6, assign28340_e39943_d_n7, assign28340_e39943_d_n10, assign28340_e39943_d_n11, assign28340_e39943_d_n12, assign28340_e39943_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_tx__blk908, locals.var_tx__blk908_dn0, locals.var_tx__blk908_dn2, locals.var_tx__blk908_dn6, locals.var_tx__blk908_dn7, locals.var_tx__blk908_dn10, locals.var_tx__blk908_dn11, locals.var_tx__blk908_dn12, locals.var_tx__blk908_dn17,)
    }
};
        locals.var_tx__blk908 = assign28340_e39943;
        locals.var_tx__blk908_dn0 = assign28340_e39943_d_n0;
        locals.var_tx__blk908_dn2 = assign28340_e39943_d_n2;
        locals.var_tx__blk908_dn6 = assign28340_e39943_d_n6;
        locals.var_tx__blk908_dn7 = assign28340_e39943_d_n7;
        locals.var_tx__blk908_dn10 = assign28340_e39943_d_n10;
        locals.var_tx__blk908_dn11 = assign28340_e39943_d_n11;
        locals.var_tx__blk908_dn12 = assign28340_e39943_d_n12;
        locals.var_tx__blk908_dn17 = assign28340_e39943_d_n17;
        locals.var_tx__blk908_rv = 0.0;

        let (assign28350_e39953, assign28350_e39953_d_n0, assign28350_e39953_d_n2, assign28350_e39953_d_n6, assign28350_e39953_d_n7, assign28350_e39953_d_n10, assign28350_e39953_d_n11, assign28350_e39953_d_n12, assign28350_e39953_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28350_e39951: f64 = (1.2 - locals.var_tx__blk908);
        (assign28350_e39951, (-locals.var_tx__blk908_dn0), (-locals.var_tx__blk908_dn2), (-locals.var_tx__blk908_dn6), (-locals.var_tx__blk908_dn7), (-locals.var_tx__blk908_dn10), (-locals.var_tx__blk908_dn11), (-locals.var_tx__blk908_dn12), (-locals.var_tx__blk908_dn17),)
    } else {
        (locals.var_t9__blk905, locals.var_t9__blk905_dn0, locals.var_t9__blk905_dn2, locals.var_t9__blk905_dn6, locals.var_t9__blk905_dn7, locals.var_t9__blk905_dn10, locals.var_t9__blk905_dn11, locals.var_t9__blk905_dn12, locals.var_t9__blk905_dn17,)
    }
};
        locals.var_t9__blk905 = assign28350_e39953;
        locals.var_t9__blk905_dn0 = assign28350_e39953_d_n0;
        locals.var_t9__blk905_dn2 = assign28350_e39953_d_n2;
        locals.var_t9__blk905_dn6 = assign28350_e39953_d_n6;
        locals.var_t9__blk905_dn7 = assign28350_e39953_d_n7;
        locals.var_t9__blk905_dn10 = assign28350_e39953_d_n10;
        locals.var_t9__blk905_dn11 = assign28350_e39953_d_n11;
        locals.var_t9__blk905_dn12 = assign28350_e39953_d_n12;
        locals.var_t9__blk905_dn17 = assign28350_e39953_d_n17;
        locals.var_t9__blk905_rv = 0.0;

        let (assign28360_e39967, assign28360_e39967_d_n0, assign28360_e39967_d_n2, assign28360_e39967_d_n6, assign28360_e39967_d_n7, assign28360_e39967_d_n10, assign28360_e39967_d_n11, assign28360_e39967_d_n12, assign28360_e39967_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28360_e39961: f64 = (locals.var_vgs * locals.var_t5__blk904);
        let assign28360_e39964: f64 = (locals.var_t9__blk905 * locals.var_t4__blk903);
        let assign28360_e39965: f64 = (assign28360_e39961 - assign28360_e39964);
        (assign28360_e39965, ((locals.var_vgs * locals.var_t5__blk904_dn0) - ((locals.var_t9__blk905_dn0 * locals.var_t4__blk903) + (locals.var_t9__blk905 * locals.var_t4__blk903_dn0))), ((locals.var_vgs * locals.var_t5__blk904_dn2) - ((locals.var_t9__blk905_dn2 * locals.var_t4__blk903) + (locals.var_t9__blk905 * locals.var_t4__blk903_dn2))), (((locals.var_vgs_dn6 * locals.var_t5__blk904) + (locals.var_vgs * locals.var_t5__blk904_dn6)) - ((locals.var_t9__blk905_dn6 * locals.var_t4__blk903) + (locals.var_t9__blk905 * locals.var_t4__blk903_dn6))), (((locals.var_vgs_dn7 * locals.var_t5__blk904) + (locals.var_vgs * locals.var_t5__blk904_dn7)) - ((locals.var_t9__blk905_dn7 * locals.var_t4__blk903) + (locals.var_t9__blk905 * locals.var_t4__blk903_dn7))), ((locals.var_vgs * locals.var_t5__blk904_dn10) - ((locals.var_t9__blk905_dn10 * locals.var_t4__blk903) + (locals.var_t9__blk905 * locals.var_t4__blk903_dn10))), (((locals.var_vgs_dn11 * locals.var_t5__blk904) + (locals.var_vgs * locals.var_t5__blk904_dn11)) - ((locals.var_t9__blk905_dn11 * locals.var_t4__blk903) + (locals.var_t9__blk905 * locals.var_t4__blk903_dn11))), ((locals.var_vgs * locals.var_t5__blk904_dn12) - ((locals.var_t9__blk905_dn12 * locals.var_t4__blk903) + (locals.var_t9__blk905 * locals.var_t4__blk903_dn12))), ((locals.var_vgs * locals.var_t5__blk904_dn17) - ((locals.var_t9__blk905_dn17 * locals.var_t4__blk903) + (locals.var_t9__blk905 * locals.var_t4__blk903_dn17))),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign28360_e39967;
        locals.var_qgos_dn0 = assign28360_e39967_d_n0;
        locals.var_qgos_dn2 = assign28360_e39967_d_n2;
        locals.var_qgos_dn6 = assign28360_e39967_d_n6;
        locals.var_qgos_dn7 = assign28360_e39967_d_n7;
        locals.var_qgos_dn10 = assign28360_e39967_d_n10;
        locals.var_qgos_dn11 = assign28360_e39967_d_n11;
        locals.var_qgos_dn12 = assign28360_e39967_d_n12;
        locals.var_qgos_dn17 = assign28360_e39967_d_n17;
        locals.var_qgos_rv = 0.0;

        let (assign28370_e39983, assign28370_e39983_d_n0, assign28370_e39983_d_n2, assign28370_e39983_d_n6, assign28370_e39983_d_n7, assign28370_e39983_d_n10, assign28370_e39983_d_n11, assign28370_e39983_d_n12, assign28370_e39983_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28370_e39975: f64 = (locals.var_cov_slp * locals.var_t1__blk900);
        let assign28370_e39978: f64 = (locals.var_cov_mag + locals.var_covvg);
        let assign28370_e39980: f64 = (assign28370_e39978 - locals.var_vds);
        let assign28370_e39981: f64 = (assign28370_e39975 * assign28370_e39980);
        (assign28370_e39981, (((locals.var_cov_slp * locals.var_t1__blk900_dn0) * assign28370_e39980) + (assign28370_e39975 * (-locals.var_vds_dn0))), (((locals.var_cov_slp * locals.var_t1__blk900_dn2) * assign28370_e39980) + (assign28370_e39975 * (-locals.var_vds_dn2))), (((locals.var_cov_slp * locals.var_t1__blk900_dn6) * assign28370_e39980) + (assign28370_e39975 * (locals.var_covvg_dn6 - locals.var_vds_dn6))), (((locals.var_cov_slp * locals.var_t1__blk900_dn7) * assign28370_e39980) + (assign28370_e39975 * (locals.var_covvg_dn7 - locals.var_vds_dn7))), (((locals.var_cov_slp * locals.var_t1__blk900_dn10) * assign28370_e39980) + (assign28370_e39975 * (-locals.var_vds_dn10))), (((locals.var_cov_slp * locals.var_t1__blk900_dn11) * assign28370_e39980) + (assign28370_e39975 * (locals.var_covvg_dn11 - locals.var_vds_dn11))), (((locals.var_cov_slp * locals.var_t1__blk900_dn12) * assign28370_e39980) + (assign28370_e39975 * (-locals.var_vds_dn12))), (((locals.var_cov_slp * locals.var_t1__blk900_dn17) * assign28370_e39980) + (assign28370_e39975 * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_t4__blk903, locals.var_t4__blk903_dn0, locals.var_t4__blk903_dn2, locals.var_t4__blk903_dn6, locals.var_t4__blk903_dn7, locals.var_t4__blk903_dn10, locals.var_t4__blk903_dn11, locals.var_t4__blk903_dn12, locals.var_t4__blk903_dn17,)
    }
};
        locals.var_t4__blk903 = assign28370_e39983;
        locals.var_t4__blk903_dn0 = assign28370_e39983_d_n0;
        locals.var_t4__blk903_dn2 = assign28370_e39983_d_n2;
        locals.var_t4__blk903_dn6 = assign28370_e39983_d_n6;
        locals.var_t4__blk903_dn7 = assign28370_e39983_d_n7;
        locals.var_t4__blk903_dn10 = assign28370_e39983_d_n10;
        locals.var_t4__blk903_dn11 = assign28370_e39983_d_n11;
        locals.var_t4__blk903_dn12 = assign28370_e39983_d_n12;
        locals.var_t4__blk903_dn17 = assign28370_e39983_d_n17;
        locals.var_t4__blk903_rv = 0.0;

        let (assign28380_e39993, assign28380_e39993_d_n0, assign28380_e39993_d_n2, assign28380_e39993_d_n6, assign28380_e39993_d_n7, assign28380_e39993_d_n10, assign28380_e39993_d_n11, assign28380_e39993_d_n12, assign28380_e39993_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28380_e39991: f64 = (locals.var_psl - locals.var_vds);
        (assign28380_e39991, (locals.var_psl_dn0 - locals.var_vds_dn0), (locals.var_psl_dn2 - locals.var_vds_dn2), (locals.var_psl_dn6 - locals.var_vds_dn6), (locals.var_psl_dn7 - locals.var_vds_dn7), (locals.var_psl_dn10 - locals.var_vds_dn10), (locals.var_psl_dn11 - locals.var_vds_dn11), (locals.var_psl_dn12 - locals.var_vds_dn12), (locals.var_psl_dn17 - locals.var_vds_dn17),)
    } else {
        (locals.var_tx__blk908, locals.var_tx__blk908_dn0, locals.var_tx__blk908_dn2, locals.var_tx__blk908_dn6, locals.var_tx__blk908_dn7, locals.var_tx__blk908_dn10, locals.var_tx__blk908_dn11, locals.var_tx__blk908_dn12, locals.var_tx__blk908_dn17,)
    }
};
        locals.var_tx__blk908 = assign28380_e39993;
        locals.var_tx__blk908_dn0 = assign28380_e39993_d_n0;
        locals.var_tx__blk908_dn2 = assign28380_e39993_d_n2;
        locals.var_tx__blk908_dn6 = assign28380_e39993_d_n6;
        locals.var_tx__blk908_dn7 = assign28380_e39993_d_n7;
        locals.var_tx__blk908_dn10 = assign28380_e39993_d_n10;
        locals.var_tx__blk908_dn11 = assign28380_e39993_d_n11;
        locals.var_tx__blk908_dn12 = assign28380_e39993_d_n12;
        locals.var_tx__blk908_dn17 = assign28380_e39993_d_n17;
        locals.var_tx__blk908_rv = 0.0;

        let (assign28390_e40003, assign28390_e40003_d_n0, assign28390_e40003_d_n2, assign28390_e40003_d_n6, assign28390_e40003_d_n7, assign28390_e40003_d_n10, assign28390_e40003_d_n11, assign28390_e40003_d_n12, assign28390_e40003_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28390_e40001: f64 = (1.2 - locals.var_tx__blk908);
        (assign28390_e40001, (-locals.var_tx__blk908_dn0), (-locals.var_tx__blk908_dn2), (-locals.var_tx__blk908_dn6), (-locals.var_tx__blk908_dn7), (-locals.var_tx__blk908_dn10), (-locals.var_tx__blk908_dn11), (-locals.var_tx__blk908_dn12), (-locals.var_tx__blk908_dn17),)
    } else {
        (locals.var_t9__blk905, locals.var_t9__blk905_dn0, locals.var_t9__blk905_dn2, locals.var_t9__blk905_dn6, locals.var_t9__blk905_dn7, locals.var_t9__blk905_dn10, locals.var_t9__blk905_dn11, locals.var_t9__blk905_dn12, locals.var_t9__blk905_dn17,)
    }
};
        locals.var_t9__blk905 = assign28390_e40003;
        locals.var_t9__blk905_dn0 = assign28390_e40003_d_n0;
        locals.var_t9__blk905_dn2 = assign28390_e40003_d_n2;
        locals.var_t9__blk905_dn6 = assign28390_e40003_d_n6;
        locals.var_t9__blk905_dn7 = assign28390_e40003_d_n7;
        locals.var_t9__blk905_dn10 = assign28390_e40003_d_n10;
        locals.var_t9__blk905_dn11 = assign28390_e40003_d_n11;
        locals.var_t9__blk905_dn12 = assign28390_e40003_d_n12;
        locals.var_t9__blk905_dn17 = assign28390_e40003_d_n17;
        locals.var_t9__blk905_rv = 0.0;

        let (assign28400_e40019, assign28400_e40019_d_n0, assign28400_e40019_d_n2, assign28400_e40019_d_n6, assign28400_e40019_d_n7, assign28400_e40019_d_n10, assign28400_e40019_d_n11, assign28400_e40019_d_n12, assign28400_e40019_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28400_e40011: f64 = (locals.var_vgs - locals.var_vds);
        let assign28400_e40013: f64 = (assign28400_e40011 * locals.var_t5__blk904);
        let assign28400_e40016: f64 = (locals.var_t4__blk903 * locals.var_t9__blk905);
        let assign28400_e40017: f64 = (assign28400_e40013 - assign28400_e40016);
        (assign28400_e40017, ((((-locals.var_vds_dn0) * locals.var_t5__blk904) + (assign28400_e40011 * locals.var_t5__blk904_dn0)) - ((locals.var_t4__blk903_dn0 * locals.var_t9__blk905) + (locals.var_t4__blk903 * locals.var_t9__blk905_dn0))), ((((-locals.var_vds_dn2) * locals.var_t5__blk904) + (assign28400_e40011 * locals.var_t5__blk904_dn2)) - ((locals.var_t4__blk903_dn2 * locals.var_t9__blk905) + (locals.var_t4__blk903 * locals.var_t9__blk905_dn2))), ((((locals.var_vgs_dn6 - locals.var_vds_dn6) * locals.var_t5__blk904) + (assign28400_e40011 * locals.var_t5__blk904_dn6)) - ((locals.var_t4__blk903_dn6 * locals.var_t9__blk905) + (locals.var_t4__blk903 * locals.var_t9__blk905_dn6))), ((((locals.var_vgs_dn7 - locals.var_vds_dn7) * locals.var_t5__blk904) + (assign28400_e40011 * locals.var_t5__blk904_dn7)) - ((locals.var_t4__blk903_dn7 * locals.var_t9__blk905) + (locals.var_t4__blk903 * locals.var_t9__blk905_dn7))), ((((-locals.var_vds_dn10) * locals.var_t5__blk904) + (assign28400_e40011 * locals.var_t5__blk904_dn10)) - ((locals.var_t4__blk903_dn10 * locals.var_t9__blk905) + (locals.var_t4__blk903 * locals.var_t9__blk905_dn10))), ((((locals.var_vgs_dn11 - locals.var_vds_dn11) * locals.var_t5__blk904) + (assign28400_e40011 * locals.var_t5__blk904_dn11)) - ((locals.var_t4__blk903_dn11 * locals.var_t9__blk905) + (locals.var_t4__blk903 * locals.var_t9__blk905_dn11))), ((((-locals.var_vds_dn12) * locals.var_t5__blk904) + (assign28400_e40011 * locals.var_t5__blk904_dn12)) - ((locals.var_t4__blk903_dn12 * locals.var_t9__blk905) + (locals.var_t4__blk903 * locals.var_t9__blk905_dn12))), ((((-locals.var_vds_dn17) * locals.var_t5__blk904) + (assign28400_e40011 * locals.var_t5__blk904_dn17)) - ((locals.var_t4__blk903_dn17 * locals.var_t9__blk905) + (locals.var_t4__blk903 * locals.var_t9__blk905_dn17))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign28400_e40019;
        locals.var_qgod_dn0 = assign28400_e40019_d_n0;
        locals.var_qgod_dn2 = assign28400_e40019_d_n2;
        locals.var_qgod_dn6 = assign28400_e40019_d_n6;
        locals.var_qgod_dn7 = assign28400_e40019_d_n7;
        locals.var_qgod_dn10 = assign28400_e40019_d_n10;
        locals.var_qgod_dn11 = assign28400_e40019_d_n11;
        locals.var_qgod_dn12 = assign28400_e40019_d_n12;
        locals.var_qgod_dn17 = assign28400_e40019_d_n17;
        locals.var_qgod_rv = 0.0;

        let (assign28410_e40033, assign28410_e40033_d_n0, assign28410_e40033_d_n2, assign28410_e40033_d_n6, assign28410_e40033_d_n7, assign28410_e40033_d_n10, assign28410_e40033_d_n11, assign28410_e40033_d_n12, assign28410_e40033_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign28410_e40029: f64 = (locals.var_mks_nover / locals.var_nsub);
        let assign28410_e40030: f64 = (assign28410_e40029).sqrt();
        let assign28410_e40031: f64 = (locals.var_cnst0soi * assign28410_e40030);
        (assign28410_e40031, ((locals.var_cnst0soi_dn0 * assign28410_e40030) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28410_e40030)))), ((locals.var_cnst0soi_dn2 * assign28410_e40030) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28410_e40030)))), ((locals.var_cnst0soi_dn6 * assign28410_e40030) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28410_e40030)))), ((locals.var_cnst0soi_dn7 * assign28410_e40030) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28410_e40030)))), ((locals.var_cnst0soi_dn10 * assign28410_e40030) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28410_e40030)))), ((locals.var_cnst0soi_dn11 * assign28410_e40030) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn11) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28410_e40030)))), ((locals.var_cnst0soi_dn12 * assign28410_e40030) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn12) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28410_e40030)))), ((locals.var_cnst0soi_dn17 * assign28410_e40030) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn17) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28410_e40030)))),)
    } else {
        (locals.var_cnst0over__blk932, locals.var_cnst0over__blk932_dn0, locals.var_cnst0over__blk932_dn2, locals.var_cnst0over__blk932_dn6, locals.var_cnst0over__blk932_dn7, locals.var_cnst0over__blk932_dn10, locals.var_cnst0over__blk932_dn11, locals.var_cnst0over__blk932_dn12, locals.var_cnst0over__blk932_dn17,)
    }
};
        locals.var_cnst0over__blk932 = assign28410_e40033;
        locals.var_cnst0over__blk932_dn0 = assign28410_e40033_d_n0;
        locals.var_cnst0over__blk932_dn2 = assign28410_e40033_d_n2;
        locals.var_cnst0over__blk932_dn6 = assign28410_e40033_d_n6;
        locals.var_cnst0over__blk932_dn7 = assign28410_e40033_d_n7;
        locals.var_cnst0over__blk932_dn10 = assign28410_e40033_d_n10;
        locals.var_cnst0over__blk932_dn11 = assign28410_e40033_d_n11;
        locals.var_cnst0over__blk932_dn12 = assign28410_e40033_d_n12;
        locals.var_cnst0over__blk932_dn17 = assign28410_e40033_d_n17;
        locals.var_cnst0over__blk932_rv = 0.0;

        let (assign28420_e40046,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign28420_e40042: f64 = (1.0 - -1.0);
        let assign28420_e40044: f64 = (assign28420_e40042 / 2.0);
        (assign28420_e40044,)
    } else {
        (locals.var_flg_ovloops__blk916,)
    }
};
        locals.var_flg_ovloops__blk916 = assign28420_e40046;
        locals.var_flg_ovloops__blk916_rv = 0.0;

        let (assign28430_e40059,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign28430_e40055: f64 = (1.0 + -1.0);
        let assign28430_e40057: f64 = (assign28430_e40055 / 2.0);
        (assign28430_e40057,)
    } else {
        (locals.var_flg_ovloopd__blk917,)
    }
};
        locals.var_flg_ovloopd__blk917 = assign28430_e40059;
        locals.var_flg_ovloopd__blk917_rv = 0.0;

        let assign28440_e40062: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard984 = assign28440_e40062;
        locals.var_guard984_rv = 0.0;

        let (assign28450_e40081, assign28450_e40081_d_n0, assign28450_e40081_d_n2, assign28450_e40081_d_n6, assign28450_e40081_d_n7, assign28450_e40081_d_n10, assign28450_e40081_d_n11, assign28450_e40081_d_n12, assign28450_e40081_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28450_e40073: f64 = (locals.var_modenml * locals.var_vbs);
        let assign28450_e40077: f64 = (locals.var_vbs - locals.var_vds);
        let assign28450_e40078: f64 = (locals.var_modervs * assign28450_e40077);
        let assign28450_e40079: f64 = (assign28450_e40073 + assign28450_e40078);
        (assign28450_e40079, ((locals.var_modenml * locals.var_vbs_dn0) + (locals.var_modervs * (locals.var_vbs_dn0 - locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vbs_dn2) + (locals.var_modervs * (locals.var_vbs_dn2 - locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vbs_dn6) + (locals.var_modervs * (locals.var_vbs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vbs_dn7) + (locals.var_modervs * (locals.var_vbs_dn7 - locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vbs_dn10) + (locals.var_modervs * (locals.var_vbs_dn10 - locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vbs_dn11) + (locals.var_modervs * (locals.var_vbs_dn11 - locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vbs_dn12) + (locals.var_modervs * (locals.var_vbs_dn12 - locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vbs_dn17) + (locals.var_modervs * (locals.var_vbs_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_vbsgmt__blk926, locals.var_vbsgmt__blk926_dn0, locals.var_vbsgmt__blk926_dn2, locals.var_vbsgmt__blk926_dn6, locals.var_vbsgmt__blk926_dn7, locals.var_vbsgmt__blk926_dn10, locals.var_vbsgmt__blk926_dn11, locals.var_vbsgmt__blk926_dn12, locals.var_vbsgmt__blk926_dn17,)
    }
};
        locals.var_vbsgmt__blk926 = assign28450_e40081;
        locals.var_vbsgmt__blk926_dn0 = assign28450_e40081_d_n0;
        locals.var_vbsgmt__blk926_dn2 = assign28450_e40081_d_n2;
        locals.var_vbsgmt__blk926_dn6 = assign28450_e40081_d_n6;
        locals.var_vbsgmt__blk926_dn7 = assign28450_e40081_d_n7;
        locals.var_vbsgmt__blk926_dn10 = assign28450_e40081_d_n10;
        locals.var_vbsgmt__blk926_dn11 = assign28450_e40081_d_n11;
        locals.var_vbsgmt__blk926_dn12 = assign28450_e40081_d_n12;
        locals.var_vbsgmt__blk926_dn17 = assign28450_e40081_d_n17;
        locals.var_vbsgmt__blk926_rv = 0.0;

        let (assign28460_e40099, assign28460_e40099_d_n0, assign28460_e40099_d_n2, assign28460_e40099_d_n6, assign28460_e40099_d_n7, assign28460_e40099_d_n10, assign28460_e40099_d_n11, assign28460_e40099_d_n12, assign28460_e40099_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28460_e40092: f64 = (locals.var_modenml * locals.var_vds);
        let assign28460_e40095: f64 = (-locals.var_vds);
        let assign28460_e40096: f64 = (locals.var_modervs * assign28460_e40095);
        let assign28460_e40097: f64 = (assign28460_e40092 + assign28460_e40096);
        (assign28460_e40097, ((locals.var_modenml * locals.var_vds_dn0) + (locals.var_modervs * (-locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vds_dn2) + (locals.var_modervs * (-locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vds_dn6) + (locals.var_modervs * (-locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vds_dn7) + (locals.var_modervs * (-locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vds_dn10) + (locals.var_modervs * (-locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vds_dn11) + (locals.var_modervs * (-locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vds_dn12) + (locals.var_modervs * (-locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vds_dn17) + (locals.var_modervs * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_vdsgmt__blk927, locals.var_vdsgmt__blk927_dn0, locals.var_vdsgmt__blk927_dn2, locals.var_vdsgmt__blk927_dn6, locals.var_vdsgmt__blk927_dn7, locals.var_vdsgmt__blk927_dn10, locals.var_vdsgmt__blk927_dn11, locals.var_vdsgmt__blk927_dn12, locals.var_vdsgmt__blk927_dn17,)
    }
};
        locals.var_vdsgmt__blk927 = assign28460_e40099;
        locals.var_vdsgmt__blk927_dn0 = assign28460_e40099_d_n0;
        locals.var_vdsgmt__blk927_dn2 = assign28460_e40099_d_n2;
        locals.var_vdsgmt__blk927_dn6 = assign28460_e40099_d_n6;
        locals.var_vdsgmt__blk927_dn7 = assign28460_e40099_d_n7;
        locals.var_vdsgmt__blk927_dn10 = assign28460_e40099_d_n10;
        locals.var_vdsgmt__blk927_dn11 = assign28460_e40099_d_n11;
        locals.var_vdsgmt__blk927_dn12 = assign28460_e40099_d_n12;
        locals.var_vdsgmt__blk927_dn17 = assign28460_e40099_d_n17;
        locals.var_vdsgmt__blk927_rv = 0.0;

        let (assign28470_e40118, assign28470_e40118_d_n0, assign28470_e40118_d_n2, assign28470_e40118_d_n6, assign28470_e40118_d_n7, assign28470_e40118_d_n10, assign28470_e40118_d_n11, assign28470_e40118_d_n12, assign28470_e40118_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28470_e40110: f64 = (locals.var_modenml * locals.var_vgs);
        let assign28470_e40114: f64 = (locals.var_vgs - locals.var_vds);
        let assign28470_e40115: f64 = (locals.var_modervs * assign28470_e40114);
        let assign28470_e40116: f64 = (assign28470_e40110 + assign28470_e40115);
        (assign28470_e40116, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgsgmt__blk928, locals.var_vgsgmt__blk928_dn0, locals.var_vgsgmt__blk928_dn2, locals.var_vgsgmt__blk928_dn6, locals.var_vgsgmt__blk928_dn7, locals.var_vgsgmt__blk928_dn10, locals.var_vgsgmt__blk928_dn11, locals.var_vgsgmt__blk928_dn12, locals.var_vgsgmt__blk928_dn17,)
    }
};
        locals.var_vgsgmt__blk928 = assign28470_e40118;
        locals.var_vgsgmt__blk928_dn0 = assign28470_e40118_d_n0;
        locals.var_vgsgmt__blk928_dn2 = assign28470_e40118_d_n2;
        locals.var_vgsgmt__blk928_dn6 = assign28470_e40118_d_n6;
        locals.var_vgsgmt__blk928_dn7 = assign28470_e40118_d_n7;
        locals.var_vgsgmt__blk928_dn10 = assign28470_e40118_d_n10;
        locals.var_vgsgmt__blk928_dn11 = assign28470_e40118_d_n11;
        locals.var_vgsgmt__blk928_dn12 = assign28470_e40118_d_n12;
        locals.var_vgsgmt__blk928_dn17 = assign28470_e40118_d_n17;
        locals.var_vgsgmt__blk928_rv = 0.0;

        let (assign28480_e40131, assign28480_e40131_d_n0, assign28480_e40131_d_n2, assign28480_e40131_d_n6, assign28480_e40131_d_n7, assign28480_e40131_d_n10, assign28480_e40131_d_n11, assign28480_e40131_d_n12, assign28480_e40131_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28480_e40129: f64 = (locals.var_vdsgmt__blk927 - locals.var_vbsgmt__blk926);
        (assign28480_e40129, (locals.var_vdsgmt__blk927_dn0 - locals.var_vbsgmt__blk926_dn0), (locals.var_vdsgmt__blk927_dn2 - locals.var_vbsgmt__blk926_dn2), (locals.var_vdsgmt__blk927_dn6 - locals.var_vbsgmt__blk926_dn6), (locals.var_vdsgmt__blk927_dn7 - locals.var_vbsgmt__blk926_dn7), (locals.var_vdsgmt__blk927_dn10 - locals.var_vbsgmt__blk926_dn10), (locals.var_vdsgmt__blk927_dn11 - locals.var_vbsgmt__blk926_dn11), (locals.var_vdsgmt__blk927_dn12 - locals.var_vbsgmt__blk926_dn12), (locals.var_vdsgmt__blk927_dn17 - locals.var_vbsgmt__blk926_dn17),)
    } else {
        (locals.var_vdbgmt__blk929, locals.var_vdbgmt__blk929_dn0, locals.var_vdbgmt__blk929_dn2, locals.var_vdbgmt__blk929_dn6, locals.var_vdbgmt__blk929_dn7, locals.var_vdbgmt__blk929_dn10, locals.var_vdbgmt__blk929_dn11, locals.var_vdbgmt__blk929_dn12, locals.var_vdbgmt__blk929_dn17,)
    }
};
        locals.var_vdbgmt__blk929 = assign28480_e40131;
        locals.var_vdbgmt__blk929_dn0 = assign28480_e40131_d_n0;
        locals.var_vdbgmt__blk929_dn2 = assign28480_e40131_d_n2;
        locals.var_vdbgmt__blk929_dn6 = assign28480_e40131_d_n6;
        locals.var_vdbgmt__blk929_dn7 = assign28480_e40131_d_n7;
        locals.var_vdbgmt__blk929_dn10 = assign28480_e40131_d_n10;
        locals.var_vdbgmt__blk929_dn11 = assign28480_e40131_d_n11;
        locals.var_vdbgmt__blk929_dn12 = assign28480_e40131_d_n12;
        locals.var_vdbgmt__blk929_dn17 = assign28480_e40131_d_n17;
        locals.var_vdbgmt__blk929_rv = 0.0;

        let (assign28490_e40144, assign28490_e40144_d_n0, assign28490_e40144_d_n2, assign28490_e40144_d_n6, assign28490_e40144_d_n7, assign28490_e40144_d_n10, assign28490_e40144_d_n11, assign28490_e40144_d_n12, assign28490_e40144_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28490_e40142: f64 = (locals.var_vgsgmt__blk928 - locals.var_vbsgmt__blk926);
        (assign28490_e40142, (locals.var_vgsgmt__blk928_dn0 - locals.var_vbsgmt__blk926_dn0), (locals.var_vgsgmt__blk928_dn2 - locals.var_vbsgmt__blk926_dn2), (locals.var_vgsgmt__blk928_dn6 - locals.var_vbsgmt__blk926_dn6), (locals.var_vgsgmt__blk928_dn7 - locals.var_vbsgmt__blk926_dn7), (locals.var_vgsgmt__blk928_dn10 - locals.var_vbsgmt__blk926_dn10), (locals.var_vgsgmt__blk928_dn11 - locals.var_vbsgmt__blk926_dn11), (locals.var_vgsgmt__blk928_dn12 - locals.var_vbsgmt__blk926_dn12), (locals.var_vgsgmt__blk928_dn17 - locals.var_vbsgmt__blk926_dn17),)
    } else {
        (locals.var_vgbgmt__blk931, locals.var_vgbgmt__blk931_dn0, locals.var_vgbgmt__blk931_dn2, locals.var_vgbgmt__blk931_dn6, locals.var_vgbgmt__blk931_dn7, locals.var_vgbgmt__blk931_dn10, locals.var_vgbgmt__blk931_dn11, locals.var_vgbgmt__blk931_dn12, locals.var_vgbgmt__blk931_dn17,)
    }
};
        locals.var_vgbgmt__blk931 = assign28490_e40144;
        locals.var_vgbgmt__blk931_dn0 = assign28490_e40144_d_n0;
        locals.var_vgbgmt__blk931_dn2 = assign28490_e40144_d_n2;
        locals.var_vgbgmt__blk931_dn6 = assign28490_e40144_d_n6;
        locals.var_vgbgmt__blk931_dn7 = assign28490_e40144_d_n7;
        locals.var_vgbgmt__blk931_dn10 = assign28490_e40144_d_n10;
        locals.var_vgbgmt__blk931_dn11 = assign28490_e40144_d_n11;
        locals.var_vgbgmt__blk931_dn12 = assign28490_e40144_d_n12;
        locals.var_vgbgmt__blk931_dn17 = assign28490_e40144_d_n17;
        locals.var_vgbgmt__blk931_rv = 0.0;

        let (assign28500_e40156, assign28500_e40156_d_n0, assign28500_e40156_d_n2, assign28500_e40156_d_n6, assign28500_e40156_d_n7, assign28500_e40156_d_n10, assign28500_e40156_d_n11, assign28500_e40156_d_n12, assign28500_e40156_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28500_e40154: f64 = (-locals.var_vbsgmt__blk926);
        (assign28500_e40154, (-locals.var_vbsgmt__blk926_dn0), (-locals.var_vbsgmt__blk926_dn2), (-locals.var_vbsgmt__blk926_dn6), (-locals.var_vbsgmt__blk926_dn7), (-locals.var_vbsgmt__blk926_dn10), (-locals.var_vbsgmt__blk926_dn11), (-locals.var_vbsgmt__blk926_dn12), (-locals.var_vbsgmt__blk926_dn17),)
    } else {
        (locals.var_vsbgmt__blk930, locals.var_vsbgmt__blk930_dn0, locals.var_vsbgmt__blk930_dn2, locals.var_vsbgmt__blk930_dn6, locals.var_vsbgmt__blk930_dn7, locals.var_vsbgmt__blk930_dn10, locals.var_vsbgmt__blk930_dn11, locals.var_vsbgmt__blk930_dn12, locals.var_vsbgmt__blk930_dn17,)
    }
};
        locals.var_vsbgmt__blk930 = assign28500_e40156;
        locals.var_vsbgmt__blk930_dn0 = assign28500_e40156_d_n0;
        locals.var_vsbgmt__blk930_dn2 = assign28500_e40156_d_n2;
        locals.var_vsbgmt__blk930_dn6 = assign28500_e40156_d_n6;
        locals.var_vsbgmt__blk930_dn7 = assign28500_e40156_d_n7;
        locals.var_vsbgmt__blk930_dn10 = assign28500_e40156_d_n10;
        locals.var_vsbgmt__blk930_dn11 = assign28500_e40156_d_n11;
        locals.var_vsbgmt__blk930_dn12 = assign28500_e40156_d_n12;
        locals.var_vsbgmt__blk930_dn17 = assign28500_e40156_d_n17;
        locals.var_vsbgmt__blk930_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_103(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28510_e40173,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28510_e40167: f64 = (locals.var_flg_ovloops__blk916 * locals.var_modenml);
        let assign28510_e40170: f64 = (locals.var_flg_ovloopd__blk917 * locals.var_modervs);
        let assign28510_e40171: f64 = (assign28510_e40167 + assign28510_e40170);
        (assign28510_e40171,)
    } else {
        (locals.var_flg_overs__blk918,)
    }
};
        locals.var_flg_overs__blk918 = assign28510_e40173;
        locals.var_flg_overs__blk918_rv = 0.0;

        let (assign28520_e40190,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28520_e40184: f64 = (locals.var_flg_ovloops__blk916 * locals.var_modervs);
        let assign28520_e40187: f64 = (locals.var_flg_ovloopd__blk917 * locals.var_modenml);
        let assign28520_e40188: f64 = (assign28520_e40184 + assign28520_e40187);
        (assign28520_e40188,)
    } else {
        (locals.var_flg_overd__blk919,)
    }
};
        locals.var_flg_overd__blk919 = assign28520_e40190;
        locals.var_flg_overd__blk919_rv = 0.0;

        let (assign28530_e40211, assign28530_e40211_d_n0, assign28530_e40211_d_n2, assign28530_e40211_d_n6, assign28530_e40211_d_n7, assign28530_e40211_d_n10, assign28530_e40211_d_n11, assign28530_e40211_d_n12, assign28530_e40211_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28530_e40201: f64 = (locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930);
        let assign28530_e40204: f64 = (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929);
        let assign28530_e40205: f64 = (assign28530_e40201 + assign28530_e40204);
        let assign28530_e40208: f64 = (10.0 * 2.220446049250313e-16);
        let assign28530_e40209: f64 = (assign28530_e40205 + assign28530_e40208);
        (assign28530_e40209, ((locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930_dn0) + (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929_dn0)), ((locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930_dn2) + (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929_dn2)), ((locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930_dn6) + (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929_dn6)), ((locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930_dn7) + (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929_dn7)), ((locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930_dn10) + (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929_dn10)), ((locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930_dn11) + (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929_dn11)), ((locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930_dn12) + (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929_dn12)), ((locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930_dn17) + (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929_dn17)),)
    } else {
        (locals.var_vxbgmt__blk924, locals.var_vxbgmt__blk924_dn0, locals.var_vxbgmt__blk924_dn2, locals.var_vxbgmt__blk924_dn6, locals.var_vxbgmt__blk924_dn7, locals.var_vxbgmt__blk924_dn10, locals.var_vxbgmt__blk924_dn11, locals.var_vxbgmt__blk924_dn12, locals.var_vxbgmt__blk924_dn17,)
    }
};
        locals.var_vxbgmt__blk924 = assign28530_e40211;
        locals.var_vxbgmt__blk924_dn0 = assign28530_e40211_d_n0;
        locals.var_vxbgmt__blk924_dn2 = assign28530_e40211_d_n2;
        locals.var_vxbgmt__blk924_dn6 = assign28530_e40211_d_n6;
        locals.var_vxbgmt__blk924_dn7 = assign28530_e40211_d_n7;
        locals.var_vxbgmt__blk924_dn10 = assign28530_e40211_d_n10;
        locals.var_vxbgmt__blk924_dn11 = assign28530_e40211_d_n11;
        locals.var_vxbgmt__blk924_dn12 = assign28530_e40211_d_n12;
        locals.var_vxbgmt__blk924_dn17 = assign28530_e40211_d_n17;
        locals.var_vxbgmt__blk924_rv = 0.0;

        let (assign28540_e40229,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard984 == 0.0)) {
        let assign28540_e40223: f64 = (locals.var_flg_ovloops__blk916 * locals.var_modenml);
        let assign28540_e40226: f64 = (locals.var_flg_ovloopd__blk917 * locals.var_modervs);
        let assign28540_e40227: f64 = (assign28540_e40223 + assign28540_e40226);
        (assign28540_e40227,)
    } else {
        (locals.var_flg_overs__blk918,)
    }
};
        locals.var_flg_overs__blk918 = assign28540_e40229;
        locals.var_flg_overs__blk918_rv = 0.0;

        let (assign28550_e40247,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard984 == 0.0)) {
        let assign28550_e40241: f64 = (locals.var_flg_ovloops__blk916 * locals.var_modervs);
        let assign28550_e40244: f64 = (locals.var_flg_ovloopd__blk917 * locals.var_modenml);
        let assign28550_e40245: f64 = (assign28550_e40241 + assign28550_e40244);
        (assign28550_e40245,)
    } else {
        (locals.var_flg_overd__blk919,)
    }
};
        locals.var_flg_overd__blk919 = assign28550_e40247;
        locals.var_flg_overd__blk919_rv = 0.0;

        let (assign28560_e40269, assign28560_e40269_d_n0, assign28560_e40269_d_n2, assign28560_e40269_d_n6, assign28560_e40269_d_n7, assign28560_e40269_d_n10, assign28560_e40269_d_n11, assign28560_e40269_d_n12, assign28560_e40269_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_flg_ovloops__blk916 != 0.0)) {
        let assign28560_e40261: f64 = (locals.var_modenml * locals.var_vgs);
        let assign28560_e40265: f64 = (locals.var_vgs - locals.var_vds);
        let assign28560_e40266: f64 = (locals.var_modervs * assign28560_e40265);
        let assign28560_e40267: f64 = (assign28560_e40261 + assign28560_e40266);
        (assign28560_e40267, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgbgmt__blk931, locals.var_vgbgmt__blk931_dn0, locals.var_vgbgmt__blk931_dn2, locals.var_vgbgmt__blk931_dn6, locals.var_vgbgmt__blk931_dn7, locals.var_vgbgmt__blk931_dn10, locals.var_vgbgmt__blk931_dn11, locals.var_vgbgmt__blk931_dn12, locals.var_vgbgmt__blk931_dn17,)
    }
};
        locals.var_vgbgmt__blk931 = assign28560_e40269;
        locals.var_vgbgmt__blk931_dn0 = assign28560_e40269_d_n0;
        locals.var_vgbgmt__blk931_dn2 = assign28560_e40269_d_n2;
        locals.var_vgbgmt__blk931_dn6 = assign28560_e40269_d_n6;
        locals.var_vgbgmt__blk931_dn7 = assign28560_e40269_d_n7;
        locals.var_vgbgmt__blk931_dn10 = assign28560_e40269_d_n10;
        locals.var_vgbgmt__blk931_dn11 = assign28560_e40269_d_n11;
        locals.var_vgbgmt__blk931_dn12 = assign28560_e40269_d_n12;
        locals.var_vgbgmt__blk931_dn17 = assign28560_e40269_d_n17;
        locals.var_vgbgmt__blk931_rv = 0.0;

        let (assign28570_e40291, assign28570_e40291_d_n0, assign28570_e40291_d_n2, assign28570_e40291_d_n6, assign28570_e40291_d_n7, assign28570_e40291_d_n10, assign28570_e40291_d_n11, assign28570_e40291_d_n12, assign28570_e40291_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_flg_ovloopd__blk917 != 0.0)) {
        let assign28570_e40283: f64 = (locals.var_modervs * locals.var_vgs);
        let assign28570_e40287: f64 = (locals.var_vgs - locals.var_vds);
        let assign28570_e40288: f64 = (locals.var_modenml * assign28570_e40287);
        let assign28570_e40289: f64 = (assign28570_e40283 + assign28570_e40288);
        (assign28570_e40289, (locals.var_modenml * (-locals.var_vds_dn0)), (locals.var_modenml * (-locals.var_vds_dn2)), ((locals.var_modervs * locals.var_vgs_dn6) + (locals.var_modenml * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modervs * locals.var_vgs_dn7) + (locals.var_modenml * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modenml * (-locals.var_vds_dn10)), ((locals.var_modervs * locals.var_vgs_dn11) + (locals.var_modenml * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modenml * (-locals.var_vds_dn12)), (locals.var_modenml * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgbgmt__blk931, locals.var_vgbgmt__blk931_dn0, locals.var_vgbgmt__blk931_dn2, locals.var_vgbgmt__blk931_dn6, locals.var_vgbgmt__blk931_dn7, locals.var_vgbgmt__blk931_dn10, locals.var_vgbgmt__blk931_dn11, locals.var_vgbgmt__blk931_dn12, locals.var_vgbgmt__blk931_dn17,)
    }
};
        locals.var_vgbgmt__blk931 = assign28570_e40291;
        locals.var_vgbgmt__blk931_dn0 = assign28570_e40291_d_n0;
        locals.var_vgbgmt__blk931_dn2 = assign28570_e40291_d_n2;
        locals.var_vgbgmt__blk931_dn6 = assign28570_e40291_d_n6;
        locals.var_vgbgmt__blk931_dn7 = assign28570_e40291_d_n7;
        locals.var_vgbgmt__blk931_dn10 = assign28570_e40291_d_n10;
        locals.var_vgbgmt__blk931_dn11 = assign28570_e40291_d_n11;
        locals.var_vgbgmt__blk931_dn12 = assign28570_e40291_d_n12;
        locals.var_vgbgmt__blk931_dn17 = assign28570_e40291_d_n17;
        locals.var_vgbgmt__blk931_rv = 0.0;

        let (assign28580_e40303, assign28580_e40303_d_n0, assign28580_e40303_d_n2, assign28580_e40303_d_n6, assign28580_e40303_d_n7, assign28580_e40303_d_n10, assign28580_e40303_d_n11, assign28580_e40303_d_n12, assign28580_e40303_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard984 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt__blk924, locals.var_vxbgmt__blk924_dn0, locals.var_vxbgmt__blk924_dn2, locals.var_vxbgmt__blk924_dn6, locals.var_vxbgmt__blk924_dn7, locals.var_vxbgmt__blk924_dn10, locals.var_vxbgmt__blk924_dn11, locals.var_vxbgmt__blk924_dn12, locals.var_vxbgmt__blk924_dn17,)
    }
};
        locals.var_vxbgmt__blk924 = assign28580_e40303;
        locals.var_vxbgmt__blk924_dn0 = assign28580_e40303_d_n0;
        locals.var_vxbgmt__blk924_dn2 = assign28580_e40303_d_n2;
        locals.var_vxbgmt__blk924_dn6 = assign28580_e40303_d_n6;
        locals.var_vxbgmt__blk924_dn7 = assign28580_e40303_d_n7;
        locals.var_vxbgmt__blk924_dn10 = assign28580_e40303_d_n10;
        locals.var_vxbgmt__blk924_dn11 = assign28580_e40303_d_n11;
        locals.var_vxbgmt__blk924_dn12 = assign28580_e40303_d_n12;
        locals.var_vxbgmt__blk924_dn17 = assign28580_e40303_d_n17;
        locals.var_vxbgmt__blk924_rv = 0.0;

        let (assign28590_e40313, assign28590_e40313_d_n0, assign28590_e40313_d_n2, assign28590_e40313_d_n6, assign28590_e40313_d_n7, assign28590_e40313_d_n10, assign28590_e40313_d_n11, assign28590_e40313_d_n12, assign28590_e40313_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign28590_e40311: f64 = (-locals.var_vxbgmt__blk924);
        (assign28590_e40311, (-locals.var_vxbgmt__blk924_dn0), (-locals.var_vxbgmt__blk924_dn2), (-locals.var_vxbgmt__blk924_dn6), (-locals.var_vxbgmt__blk924_dn7), (-locals.var_vxbgmt__blk924_dn10), (-locals.var_vxbgmt__blk924_dn11), (-locals.var_vxbgmt__blk924_dn12), (-locals.var_vxbgmt__blk924_dn17),)
    } else {
        (locals.var_t0__blk899, locals.var_t0__blk899_dn0, locals.var_t0__blk899_dn2, locals.var_t0__blk899_dn6, locals.var_t0__blk899_dn7, locals.var_t0__blk899_dn10, locals.var_t0__blk899_dn11, locals.var_t0__blk899_dn12, locals.var_t0__blk899_dn17,)
    }
};
        locals.var_t0__blk899 = assign28590_e40313;
        locals.var_t0__blk899_dn0 = assign28590_e40313_d_n0;
        locals.var_t0__blk899_dn2 = assign28590_e40313_d_n2;
        locals.var_t0__blk899_dn6 = assign28590_e40313_d_n6;
        locals.var_t0__blk899_dn7 = assign28590_e40313_d_n7;
        locals.var_t0__blk899_dn10 = assign28590_e40313_d_n10;
        locals.var_t0__blk899_dn11 = assign28590_e40313_d_n11;
        locals.var_t0__blk899_dn12 = assign28590_e40313_d_n12;
        locals.var_t0__blk899_dn17 = assign28590_e40313_d_n17;
        locals.var_t0__blk899_rv = 0.0;

        let assign28600_e40316: f64 = if locals.var_t0__blk899 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard985 = assign28600_e40316;
        locals.var_guard985_rv = 0.0;

        let (assign28610_e40329, assign28610_e40329_d_n0, assign28610_e40329_d_n2, assign28610_e40329_d_n6, assign28610_e40329_d_n7, assign28610_e40329_d_n10, assign28610_e40329_d_n11, assign28610_e40329_d_n12, assign28610_e40329_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard985 != 0.0)) {
        let assign28610_e40327: f64 = (locals.var_t0__blk899 - locals.var_vbs_bnd);
        (assign28610_e40327, locals.var_t0__blk899_dn0, locals.var_t0__blk899_dn2, locals.var_t0__blk899_dn6, locals.var_t0__blk899_dn7, locals.var_t0__blk899_dn10, locals.var_t0__blk899_dn11, locals.var_t0__blk899_dn12, locals.var_t0__blk899_dn17,)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign28610_e40329;
        locals.var_t1__blk900_dn0 = assign28610_e40329_d_n0;
        locals.var_t1__blk900_dn2 = assign28610_e40329_d_n2;
        locals.var_t1__blk900_dn6 = assign28610_e40329_d_n6;
        locals.var_t1__blk900_dn7 = assign28610_e40329_d_n7;
        locals.var_t1__blk900_dn10 = assign28610_e40329_d_n10;
        locals.var_t1__blk900_dn11 = assign28610_e40329_d_n11;
        locals.var_t1__blk900_dn12 = assign28610_e40329_d_n12;
        locals.var_t1__blk900_dn17 = assign28610_e40329_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign28620_e40342, assign28620_e40342_d_n0, assign28620_e40342_d_n2, assign28620_e40342_d_n6, assign28620_e40342_d_n7, assign28620_e40342_d_n10, assign28620_e40342_d_n11, assign28620_e40342_d_n12, assign28620_e40342_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard985 != 0.0)) {
        let assign28620_e40340: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign28620_e40340, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk901, locals.var_t2__blk901_dn0, locals.var_t2__blk901_dn2, locals.var_t2__blk901_dn6, locals.var_t2__blk901_dn7, locals.var_t2__blk901_dn10, locals.var_t2__blk901_dn11, locals.var_t2__blk901_dn12, locals.var_t2__blk901_dn17,)
    }
};
        locals.var_t2__blk901 = assign28620_e40342;
        locals.var_t2__blk901_dn0 = assign28620_e40342_d_n0;
        locals.var_t2__blk901_dn2 = assign28620_e40342_d_n2;
        locals.var_t2__blk901_dn6 = assign28620_e40342_d_n6;
        locals.var_t2__blk901_dn7 = assign28620_e40342_d_n7;
        locals.var_t2__blk901_dn10 = assign28620_e40342_d_n10;
        locals.var_t2__blk901_dn11 = assign28620_e40342_d_n11;
        locals.var_t2__blk901_dn12 = assign28620_e40342_d_n12;
        locals.var_t2__blk901_dn17 = assign28620_e40342_d_n17;
        locals.var_t2__blk901_rv = 0.0;

        let (assign28630_e40355, assign28630_e40355_d_n0, assign28630_e40355_d_n2, assign28630_e40355_d_n6, assign28630_e40355_d_n7, assign28630_e40355_d_n10, assign28630_e40355_d_n11, assign28630_e40355_d_n12, assign28630_e40355_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard985 != 0.0)) {
        let assign28630_e40353: f64 = (locals.var_t1__blk900 / locals.var_t2__blk901);
        (assign28630_e40353, (((locals.var_t1__blk900_dn0 * locals.var_t2__blk901) - (locals.var_t1__blk900 * locals.var_t2__blk901_dn0)) / (locals.var_t2__blk901 * locals.var_t2__blk901)), (((locals.var_t1__blk900_dn2 * locals.var_t2__blk901) - (locals.var_t1__blk900 * locals.var_t2__blk901_dn2)) / (locals.var_t2__blk901 * locals.var_t2__blk901)), (((locals.var_t1__blk900_dn6 * locals.var_t2__blk901) - (locals.var_t1__blk900 * locals.var_t2__blk901_dn6)) / (locals.var_t2__blk901 * locals.var_t2__blk901)), (((locals.var_t1__blk900_dn7 * locals.var_t2__blk901) - (locals.var_t1__blk900 * locals.var_t2__blk901_dn7)) / (locals.var_t2__blk901 * locals.var_t2__blk901)), (((locals.var_t1__blk900_dn10 * locals.var_t2__blk901) - (locals.var_t1__blk900 * locals.var_t2__blk901_dn10)) / (locals.var_t2__blk901 * locals.var_t2__blk901)), (((locals.var_t1__blk900_dn11 * locals.var_t2__blk901) - (locals.var_t1__blk900 * locals.var_t2__blk901_dn11)) / (locals.var_t2__blk901 * locals.var_t2__blk901)), (((locals.var_t1__blk900_dn12 * locals.var_t2__blk901) - (locals.var_t1__blk900 * locals.var_t2__blk901_dn12)) / (locals.var_t2__blk901 * locals.var_t2__blk901)), (((locals.var_t1__blk900_dn17 * locals.var_t2__blk901) - (locals.var_t1__blk900 * locals.var_t2__blk901_dn17)) / (locals.var_t2__blk901 * locals.var_t2__blk901)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign28630_e40355;
        locals.var_tmf1_dn0 = assign28630_e40355_d_n0;
        locals.var_tmf1_dn2 = assign28630_e40355_d_n2;
        locals.var_tmf1_dn6 = assign28630_e40355_d_n6;
        locals.var_tmf1_dn7 = assign28630_e40355_d_n7;
        locals.var_tmf1_dn10 = assign28630_e40355_d_n10;
        locals.var_tmf1_dn11 = assign28630_e40355_d_n11;
        locals.var_tmf1_dn12 = assign28630_e40355_d_n12;
        locals.var_tmf1_dn17 = assign28630_e40355_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign28640_e40368, assign28640_e40368_d_n0, assign28640_e40368_d_n2, assign28640_e40368_d_n6, assign28640_e40368_d_n7, assign28640_e40368_d_n10, assign28640_e40368_d_n11, assign28640_e40368_d_n12, assign28640_e40368_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard985 != 0.0)) {
        let assign28640_e40366: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign28640_e40366, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign28640_e40368;
        locals.var_tmf2_dn0 = assign28640_e40368_d_n0;
        locals.var_tmf2_dn2 = assign28640_e40368_d_n2;
        locals.var_tmf2_dn6 = assign28640_e40368_d_n6;
        locals.var_tmf2_dn7 = assign28640_e40368_d_n7;
        locals.var_tmf2_dn10 = assign28640_e40368_d_n10;
        locals.var_tmf2_dn11 = assign28640_e40368_d_n11;
        locals.var_tmf2_dn12 = assign28640_e40368_d_n12;
        locals.var_tmf2_dn17 = assign28640_e40368_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign28650_e40381, assign28650_e40381_d_n0, assign28650_e40381_d_n2, assign28650_e40381_d_n6, assign28650_e40381_d_n7, assign28650_e40381_d_n10, assign28650_e40381_d_n11, assign28650_e40381_d_n12, assign28650_e40381_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard985 != 0.0)) {
        let assign28650_e40379: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign28650_e40379, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12, locals.var_tmf3_dn17,)
    }
};
        locals.var_tmf3 = assign28650_e40381;
        locals.var_tmf3_dn0 = assign28650_e40381_d_n0;
        locals.var_tmf3_dn2 = assign28650_e40381_d_n2;
        locals.var_tmf3_dn6 = assign28650_e40381_d_n6;
        locals.var_tmf3_dn7 = assign28650_e40381_d_n7;
        locals.var_tmf3_dn10 = assign28650_e40381_d_n10;
        locals.var_tmf3_dn11 = assign28650_e40381_d_n11;
        locals.var_tmf3_dn12 = assign28650_e40381_d_n12;
        locals.var_tmf3_dn17 = assign28650_e40381_d_n17;
        locals.var_tmf3_rv = 0.0;

        let (assign28660_e40394, assign28660_e40394_d_n0, assign28660_e40394_d_n2, assign28660_e40394_d_n6, assign28660_e40394_d_n7, assign28660_e40394_d_n10, assign28660_e40394_d_n11, assign28660_e40394_d_n12, assign28660_e40394_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard985 != 0.0)) {
        let assign28660_e40392: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign28660_e40392, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn17)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn12, locals.var_tmf4_dn17,)
    }
};
        locals.var_tmf4 = assign28660_e40394;
        locals.var_tmf4_dn0 = assign28660_e40394_d_n0;
        locals.var_tmf4_dn2 = assign28660_e40394_d_n2;
        locals.var_tmf4_dn6 = assign28660_e40394_d_n6;
        locals.var_tmf4_dn7 = assign28660_e40394_d_n7;
        locals.var_tmf4_dn10 = assign28660_e40394_d_n10;
        locals.var_tmf4_dn11 = assign28660_e40394_d_n11;
        locals.var_tmf4_dn12 = assign28660_e40394_d_n12;
        locals.var_tmf4_dn17 = assign28660_e40394_d_n17;
        locals.var_tmf4_rv = 0.0;

        let (assign28670_e40415, assign28670_e40415_d_n0, assign28670_e40415_d_n2, assign28670_e40415_d_n6, assign28670_e40415_d_n7, assign28670_e40415_d_n10, assign28670_e40415_d_n11, assign28670_e40415_d_n12, assign28670_e40415_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard985 != 0.0)) {
        let assign28670_e40406: f64 = (1.0 + locals.var_tmf1);
        let assign28670_e40408: f64 = (assign28670_e40406 + locals.var_tmf2);
        let assign28670_e40410: f64 = (assign28670_e40408 + locals.var_tmf3);
        let assign28670_e40412: f64 = (assign28670_e40410 + locals.var_tmf4);
        let assign28670_e40413: f64 = (1.0 / assign28670_e40412);
        (assign28670_e40413, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign28670_e40412 * assign28670_e40412))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign28670_e40412 * assign28670_e40412))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign28670_e40412 * assign28670_e40412))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign28670_e40412 * assign28670_e40412))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign28670_e40412 * assign28670_e40412))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign28670_e40412 * assign28670_e40412))), (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign28670_e40412 * assign28670_e40412))), (-((((locals.var_tmf1_dn17 + locals.var_tmf2_dn17) + locals.var_tmf3_dn17) + locals.var_tmf4_dn17) / (assign28670_e40412 * assign28670_e40412))),)
    } else {
        (locals.var_ty__blk909, locals.var_ty__blk909_dn0, locals.var_ty__blk909_dn2, locals.var_ty__blk909_dn6, locals.var_ty__blk909_dn7, locals.var_ty__blk909_dn10, locals.var_ty__blk909_dn11, locals.var_ty__blk909_dn12, locals.var_ty__blk909_dn17,)
    }
};
        locals.var_ty__blk909 = assign28670_e40415;
        locals.var_ty__blk909_dn0 = assign28670_e40415_d_n0;
        locals.var_ty__blk909_dn2 = assign28670_e40415_d_n2;
        locals.var_ty__blk909_dn6 = assign28670_e40415_d_n6;
        locals.var_ty__blk909_dn7 = assign28670_e40415_d_n7;
        locals.var_ty__blk909_dn10 = assign28670_e40415_d_n10;
        locals.var_ty__blk909_dn11 = assign28670_e40415_d_n11;
        locals.var_ty__blk909_dn12 = assign28670_e40415_d_n12;
        locals.var_ty__blk909_dn17 = assign28670_e40415_d_n17;
        locals.var_ty__blk909_rv = 0.0;

        let (assign28690_e40458, assign28690_e40458_d_n0, assign28690_e40458_d_n2, assign28690_e40458_d_n6, assign28690_e40458_d_n7, assign28690_e40458_d_n10, assign28690_e40458_d_n11, assign28690_e40458_d_n12, assign28690_e40458_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard985 != 0.0)) {
        let assign28690_e40455: f64 = (1.0 - locals.var_ty__blk909);
        let assign28690_e40456: f64 = (locals.var_t2__blk901 * assign28690_e40455);
        (assign28690_e40456, ((locals.var_t2__blk901_dn0 * assign28690_e40455) + (locals.var_t2__blk901 * (-locals.var_ty__blk909_dn0))), ((locals.var_t2__blk901_dn2 * assign28690_e40455) + (locals.var_t2__blk901 * (-locals.var_ty__blk909_dn2))), ((locals.var_t2__blk901_dn6 * assign28690_e40455) + (locals.var_t2__blk901 * (-locals.var_ty__blk909_dn6))), ((locals.var_t2__blk901_dn7 * assign28690_e40455) + (locals.var_t2__blk901 * (-locals.var_ty__blk909_dn7))), ((locals.var_t2__blk901_dn10 * assign28690_e40455) + (locals.var_t2__blk901 * (-locals.var_ty__blk909_dn10))), ((locals.var_t2__blk901_dn11 * assign28690_e40455) + (locals.var_t2__blk901 * (-locals.var_ty__blk909_dn11))), ((locals.var_t2__blk901_dn12 * assign28690_e40455) + (locals.var_t2__blk901 * (-locals.var_ty__blk909_dn12))), ((locals.var_t2__blk901_dn17 * assign28690_e40455) + (locals.var_t2__blk901 * (-locals.var_ty__blk909_dn17))),)
    } else {
        (locals.var_ty__blk909, locals.var_ty__blk909_dn0, locals.var_ty__blk909_dn2, locals.var_ty__blk909_dn6, locals.var_ty__blk909_dn7, locals.var_ty__blk909_dn10, locals.var_ty__blk909_dn11, locals.var_ty__blk909_dn12, locals.var_ty__blk909_dn17,)
    }
};
        locals.var_ty__blk909 = assign28690_e40458;
        locals.var_ty__blk909_dn0 = assign28690_e40458_d_n0;
        locals.var_ty__blk909_dn2 = assign28690_e40458_d_n2;
        locals.var_ty__blk909_dn6 = assign28690_e40458_d_n6;
        locals.var_ty__blk909_dn7 = assign28690_e40458_d_n7;
        locals.var_ty__blk909_dn10 = assign28690_e40458_d_n10;
        locals.var_ty__blk909_dn11 = assign28690_e40458_d_n11;
        locals.var_ty__blk909_dn12 = assign28690_e40458_d_n12;
        locals.var_ty__blk909_dn17 = assign28690_e40458_d_n17;
        locals.var_ty__blk909_rv = 0.0;

        let (assign28710_e40483, assign28710_e40483_d_n0, assign28710_e40483_d_n2, assign28710_e40483_d_n6, assign28710_e40483_d_n7, assign28710_e40483_d_n10, assign28710_e40483_d_n11, assign28710_e40483_d_n12, assign28710_e40483_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard985 != 0.0)) {
        let assign28710_e40481: f64 = (locals.var_vbs_bnd + locals.var_ty__blk909);
        (assign28710_e40481, locals.var_ty__blk909_dn0, locals.var_ty__blk909_dn2, locals.var_ty__blk909_dn6, locals.var_ty__blk909_dn7, locals.var_ty__blk909_dn10, locals.var_ty__blk909_dn11, locals.var_ty__blk909_dn12, locals.var_ty__blk909_dn17,)
    } else {
        (locals.var_t10__blk906, locals.var_t10__blk906_dn0, locals.var_t10__blk906_dn2, locals.var_t10__blk906_dn6, locals.var_t10__blk906_dn7, locals.var_t10__blk906_dn10, locals.var_t10__blk906_dn11, locals.var_t10__blk906_dn12, locals.var_t10__blk906_dn17,)
    }
};
        locals.var_t10__blk906 = assign28710_e40483;
        locals.var_t10__blk906_dn0 = assign28710_e40483_d_n0;
        locals.var_t10__blk906_dn2 = assign28710_e40483_d_n2;
        locals.var_t10__blk906_dn6 = assign28710_e40483_d_n6;
        locals.var_t10__blk906_dn7 = assign28710_e40483_d_n7;
        locals.var_t10__blk906_dn10 = assign28710_e40483_d_n10;
        locals.var_t10__blk906_dn11 = assign28710_e40483_d_n11;
        locals.var_t10__blk906_dn12 = assign28710_e40483_d_n12;
        locals.var_t10__blk906_dn17 = assign28710_e40483_d_n17;
        locals.var_t10__blk906_rv = 0.0;

        let (assign28720_e40495, assign28720_e40495_d_n0, assign28720_e40495_d_n2, assign28720_e40495_d_n6, assign28720_e40495_d_n7, assign28720_e40495_d_n10, assign28720_e40495_d_n11, assign28720_e40495_d_n12, assign28720_e40495_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard985 == 0.0)) {
        (locals.var_t0__blk899, locals.var_t0__blk899_dn0, locals.var_t0__blk899_dn2, locals.var_t0__blk899_dn6, locals.var_t0__blk899_dn7, locals.var_t0__blk899_dn10, locals.var_t0__blk899_dn11, locals.var_t0__blk899_dn12, locals.var_t0__blk899_dn17,)
    } else {
        (locals.var_t10__blk906, locals.var_t10__blk906_dn0, locals.var_t10__blk906_dn2, locals.var_t10__blk906_dn6, locals.var_t10__blk906_dn7, locals.var_t10__blk906_dn10, locals.var_t10__blk906_dn11, locals.var_t10__blk906_dn12, locals.var_t10__blk906_dn17,)
    }
};
        locals.var_t10__blk906 = assign28720_e40495;
        locals.var_t10__blk906_dn0 = assign28720_e40495_d_n0;
        locals.var_t10__blk906_dn2 = assign28720_e40495_d_n2;
        locals.var_t10__blk906_dn6 = assign28720_e40495_d_n6;
        locals.var_t10__blk906_dn7 = assign28720_e40495_d_n7;
        locals.var_t10__blk906_dn10 = assign28720_e40495_d_n10;
        locals.var_t10__blk906_dn11 = assign28720_e40495_d_n11;
        locals.var_t10__blk906_dn12 = assign28720_e40495_d_n12;
        locals.var_t10__blk906_dn17 = assign28720_e40495_d_n17;
        locals.var_t10__blk906_rv = 0.0;

        let (assign28740_e40519, assign28740_e40519_d_n0, assign28740_e40519_d_n2, assign28740_e40519_d_n6, assign28740_e40519_d_n7, assign28740_e40519_d_n10, assign28740_e40519_d_n11, assign28740_e40519_d_n12, assign28740_e40519_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign28740_e40515: f64 = (-locals.var_t10__blk906);
        let assign28740_e40517: f64 = (assign28740_e40515 - 1e-12);
        (assign28740_e40517, (-locals.var_t10__blk906_dn0), (-locals.var_t10__blk906_dn2), (-locals.var_t10__blk906_dn6), (-locals.var_t10__blk906_dn7), (-locals.var_t10__blk906_dn10), (-locals.var_t10__blk906_dn11), (-locals.var_t10__blk906_dn12), (-locals.var_t10__blk906_dn17),)
    } else {
        (locals.var_vxbgmtcl__blk925, locals.var_vxbgmtcl__blk925_dn0, locals.var_vxbgmtcl__blk925_dn2, locals.var_vxbgmtcl__blk925_dn6, locals.var_vxbgmtcl__blk925_dn7, locals.var_vxbgmtcl__blk925_dn10, locals.var_vxbgmtcl__blk925_dn11, locals.var_vxbgmtcl__blk925_dn12, locals.var_vxbgmtcl__blk925_dn17,)
    }
};
        locals.var_vxbgmtcl__blk925 = assign28740_e40519;
        locals.var_vxbgmtcl__blk925_dn0 = assign28740_e40519_d_n0;
        locals.var_vxbgmtcl__blk925_dn2 = assign28740_e40519_d_n2;
        locals.var_vxbgmtcl__blk925_dn6 = assign28740_e40519_d_n6;
        locals.var_vxbgmtcl__blk925_dn7 = assign28740_e40519_d_n7;
        locals.var_vxbgmtcl__blk925_dn10 = assign28740_e40519_d_n10;
        locals.var_vxbgmtcl__blk925_dn11 = assign28740_e40519_d_n11;
        locals.var_vxbgmtcl__blk925_dn12 = assign28740_e40519_d_n12;
        locals.var_vxbgmtcl__blk925_dn17 = assign28740_e40519_d_n17;
        locals.var_vxbgmtcl__blk925_rv = 0.0;

        let (assign28750_e40530, assign28750_e40530_d_n0, assign28750_e40530_d_n2, assign28750_e40530_d_n6, assign28750_e40530_d_n7, assign28750_e40530_d_n10, assign28750_e40530_d_n11, assign28750_e40530_d_n12, assign28750_e40530_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign28750_e40528: f64 = (locals.var_cnst0over__blk932 * locals.var_cox0_inv__blk911);
        (assign28750_e40528, (locals.var_cnst0over__blk932_dn0 * locals.var_cox0_inv__blk911), (locals.var_cnst0over__blk932_dn2 * locals.var_cox0_inv__blk911), (locals.var_cnst0over__blk932_dn6 * locals.var_cox0_inv__blk911), (locals.var_cnst0over__blk932_dn7 * locals.var_cox0_inv__blk911), (locals.var_cnst0over__blk932_dn10 * locals.var_cox0_inv__blk911), (locals.var_cnst0over__blk932_dn11 * locals.var_cox0_inv__blk911), (locals.var_cnst0over__blk932_dn12 * locals.var_cox0_inv__blk911), (locals.var_cnst0over__blk932_dn17 * locals.var_cox0_inv__blk911),)
    } else {
        (locals.var_fac1__blk933, locals.var_fac1__blk933_dn0, locals.var_fac1__blk933_dn2, locals.var_fac1__blk933_dn6, locals.var_fac1__blk933_dn7, locals.var_fac1__blk933_dn10, locals.var_fac1__blk933_dn11, locals.var_fac1__blk933_dn12, locals.var_fac1__blk933_dn17,)
    }
};
        locals.var_fac1__blk933 = assign28750_e40530;
        locals.var_fac1__blk933_dn0 = assign28750_e40530_d_n0;
        locals.var_fac1__blk933_dn2 = assign28750_e40530_d_n2;
        locals.var_fac1__blk933_dn6 = assign28750_e40530_d_n6;
        locals.var_fac1__blk933_dn7 = assign28750_e40530_d_n7;
        locals.var_fac1__blk933_dn10 = assign28750_e40530_d_n10;
        locals.var_fac1__blk933_dn11 = assign28750_e40530_d_n11;
        locals.var_fac1__blk933_dn12 = assign28750_e40530_d_n12;
        locals.var_fac1__blk933_dn17 = assign28750_e40530_d_n17;
        locals.var_fac1__blk933_rv = 0.0;

        let (assign28760_e40541, assign28760_e40541_d_n0, assign28760_e40541_d_n2, assign28760_e40541_d_n6, assign28760_e40541_d_n7, assign28760_e40541_d_n10, assign28760_e40541_d_n11, assign28760_e40541_d_n12, assign28760_e40541_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign28760_e40539: f64 = (locals.var_fac1__blk933 * locals.var_fac1__blk933);
        (assign28760_e40539, ((locals.var_fac1__blk933_dn0 * locals.var_fac1__blk933) + (locals.var_fac1__blk933 * locals.var_fac1__blk933_dn0)), ((locals.var_fac1__blk933_dn2 * locals.var_fac1__blk933) + (locals.var_fac1__blk933 * locals.var_fac1__blk933_dn2)), ((locals.var_fac1__blk933_dn6 * locals.var_fac1__blk933) + (locals.var_fac1__blk933 * locals.var_fac1__blk933_dn6)), ((locals.var_fac1__blk933_dn7 * locals.var_fac1__blk933) + (locals.var_fac1__blk933 * locals.var_fac1__blk933_dn7)), ((locals.var_fac1__blk933_dn10 * locals.var_fac1__blk933) + (locals.var_fac1__blk933 * locals.var_fac1__blk933_dn10)), ((locals.var_fac1__blk933_dn11 * locals.var_fac1__blk933) + (locals.var_fac1__blk933 * locals.var_fac1__blk933_dn11)), ((locals.var_fac1__blk933_dn12 * locals.var_fac1__blk933) + (locals.var_fac1__blk933 * locals.var_fac1__blk933_dn12)), ((locals.var_fac1__blk933_dn17 * locals.var_fac1__blk933) + (locals.var_fac1__blk933 * locals.var_fac1__blk933_dn17)),)
    } else {
        (locals.var_fac1p2__blk934, locals.var_fac1p2__blk934_dn0, locals.var_fac1p2__blk934_dn2, locals.var_fac1p2__blk934_dn6, locals.var_fac1p2__blk934_dn7, locals.var_fac1p2__blk934_dn10, locals.var_fac1p2__blk934_dn11, locals.var_fac1p2__blk934_dn12, locals.var_fac1p2__blk934_dn17,)
    }
};
        locals.var_fac1p2__blk934 = assign28760_e40541;
        locals.var_fac1p2__blk934_dn0 = assign28760_e40541_d_n0;
        locals.var_fac1p2__blk934_dn2 = assign28760_e40541_d_n2;
        locals.var_fac1p2__blk934_dn6 = assign28760_e40541_d_n6;
        locals.var_fac1p2__blk934_dn7 = assign28760_e40541_d_n7;
        locals.var_fac1p2__blk934_dn10 = assign28760_e40541_d_n10;
        locals.var_fac1p2__blk934_dn11 = assign28760_e40541_d_n11;
        locals.var_fac1p2__blk934_dn12 = assign28760_e40541_d_n12;
        locals.var_fac1p2__blk934_dn17 = assign28760_e40541_d_n17;
        locals.var_fac1p2__blk934_rv = 0.0;

        let (assign28770_e40553, assign28770_e40553_d_n0, assign28770_e40553_d_n2, assign28770_e40553_d_n6, assign28770_e40553_d_n7, assign28770_e40553_d_n10, assign28770_e40553_d_n11, assign28770_e40553_d_n12, assign28770_e40553_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign28770_e40549: f64 = (-locals.var_vgbgmt__blk931);
        let assign28770_e40551: f64 = (assign28770_e40549 + locals.var_uc_vfbover);
        (assign28770_e40551, (-locals.var_vgbgmt__blk931_dn0), (-locals.var_vgbgmt__blk931_dn2), (-locals.var_vgbgmt__blk931_dn6), (-locals.var_vgbgmt__blk931_dn7), (-locals.var_vgbgmt__blk931_dn10), (-locals.var_vgbgmt__blk931_dn11), (-locals.var_vgbgmt__blk931_dn12), (-locals.var_vgbgmt__blk931_dn17),)
    } else {
        (locals.var_vgpld__blk935, locals.var_vgpld__blk935_dn0, locals.var_vgpld__blk935_dn2, locals.var_vgpld__blk935_dn6, locals.var_vgpld__blk935_dn7, locals.var_vgpld__blk935_dn10, locals.var_vgpld__blk935_dn11, locals.var_vgpld__blk935_dn12, locals.var_vgpld__blk935_dn17,)
    }
};
        locals.var_vgpld__blk935 = assign28770_e40553;
        locals.var_vgpld__blk935_dn0 = assign28770_e40553_d_n0;
        locals.var_vgpld__blk935_dn2 = assign28770_e40553_d_n2;
        locals.var_vgpld__blk935_dn6 = assign28770_e40553_d_n6;
        locals.var_vgpld__blk935_dn7 = assign28770_e40553_d_n7;
        locals.var_vgpld__blk935_dn10 = assign28770_e40553_d_n10;
        locals.var_vgpld__blk935_dn11 = assign28770_e40553_d_n11;
        locals.var_vgpld__blk935_dn12 = assign28770_e40553_d_n12;
        locals.var_vgpld__blk935_dn17 = assign28770_e40553_d_n17;
        locals.var_vgpld__blk935_rv = 0.0;

        let (assign28780_e40564, assign28780_e40564_d_n0, assign28780_e40564_d_n2, assign28780_e40564_d_n6, assign28780_e40564_d_n7, assign28780_e40564_d_n10, assign28780_e40564_d_n11, assign28780_e40564_d_n12, assign28780_e40564_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign28780_e40562: f64 = (locals.var_mks_nover / locals.var_nin);
        (assign28780_e40562, (-((locals.var_mks_nover * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn12) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn17) / (locals.var_nin * locals.var_nin))),)
    } else {
        (locals.var_t0__blk899, locals.var_t0__blk899_dn0, locals.var_t0__blk899_dn2, locals.var_t0__blk899_dn6, locals.var_t0__blk899_dn7, locals.var_t0__blk899_dn10, locals.var_t0__blk899_dn11, locals.var_t0__blk899_dn12, locals.var_t0__blk899_dn17,)
    }
};
        locals.var_t0__blk899 = assign28780_e40564;
        locals.var_t0__blk899_dn0 = assign28780_e40564_d_n0;
        locals.var_t0__blk899_dn2 = assign28780_e40564_d_n2;
        locals.var_t0__blk899_dn6 = assign28780_e40564_d_n6;
        locals.var_t0__blk899_dn7 = assign28780_e40564_d_n7;
        locals.var_t0__blk899_dn10 = assign28780_e40564_d_n10;
        locals.var_t0__blk899_dn11 = assign28780_e40564_d_n11;
        locals.var_t0__blk899_dn12 = assign28780_e40564_d_n12;
        locals.var_t0__blk899_dn17 = assign28780_e40564_d_n17;
        locals.var_t0__blk899_rv = 0.0;

        let (assign28790_e40578, assign28790_e40578_d_n0, assign28790_e40578_d_n2, assign28790_e40578_d_n6, assign28790_e40578_d_n7, assign28790_e40578_d_n10, assign28790_e40578_d_n11, assign28790_e40578_d_n12, assign28790_e40578_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign28790_e40573: f64 = (2.0 / locals.var_beta);
        let assign28790_e40575: f64 = (locals.var_t0__blk899).ln();
        let assign28790_e40576: f64 = (assign28790_e40573 * assign28790_e40575);
        (assign28790_e40576, (assign28790_e40573 * (locals.var_t0__blk899_dn0 / locals.var_t0__blk899)), (assign28790_e40573 * (locals.var_t0__blk899_dn2 / locals.var_t0__blk899)), (assign28790_e40573 * (locals.var_t0__blk899_dn6 / locals.var_t0__blk899)), (assign28790_e40573 * (locals.var_t0__blk899_dn7 / locals.var_t0__blk899)), (((-((2.0 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign28790_e40575) + (assign28790_e40573 * (locals.var_t0__blk899_dn10 / locals.var_t0__blk899))), (assign28790_e40573 * (locals.var_t0__blk899_dn11 / locals.var_t0__blk899)), (assign28790_e40573 * (locals.var_t0__blk899_dn12 / locals.var_t0__blk899)), (assign28790_e40573 * (locals.var_t0__blk899_dn17 / locals.var_t0__blk899)),)
    } else {
        (locals.var_pb2over__blk936, locals.var_pb2over__blk936_dn0, locals.var_pb2over__blk936_dn2, locals.var_pb2over__blk936_dn6, locals.var_pb2over__blk936_dn7, locals.var_pb2over__blk936_dn10, locals.var_pb2over__blk936_dn11, locals.var_pb2over__blk936_dn12, locals.var_pb2over__blk936_dn17,)
    }
};
        locals.var_pb2over__blk936 = assign28790_e40578;
        locals.var_pb2over__blk936_dn0 = assign28790_e40578_d_n0;
        locals.var_pb2over__blk936_dn2 = assign28790_e40578_d_n2;
        locals.var_pb2over__blk936_dn6 = assign28790_e40578_d_n6;
        locals.var_pb2over__blk936_dn7 = assign28790_e40578_d_n7;
        locals.var_pb2over__blk936_dn10 = assign28790_e40578_d_n10;
        locals.var_pb2over__blk936_dn11 = assign28790_e40578_d_n11;
        locals.var_pb2over__blk936_dn12 = assign28790_e40578_d_n12;
        locals.var_pb2over__blk936_dn17 = assign28790_e40578_d_n17;
        locals.var_pb2over__blk936_rv = 0.0;

        let (assign28800_e40588, assign28800_e40588_d_n0, assign28800_e40588_d_n2, assign28800_e40588_d_n6, assign28800_e40588_d_n7, assign28800_e40588_d_n10, assign28800_e40588_d_n11, assign28800_e40588_d_n12, assign28800_e40588_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign28800_e40586: f64 = (-locals.var_vxbgmtcl__blk925);
        (assign28800_e40586, (-locals.var_vxbgmtcl__blk925_dn0), (-locals.var_vxbgmtcl__blk925_dn2), (-locals.var_vxbgmtcl__blk925_dn6), (-locals.var_vxbgmtcl__blk925_dn7), (-locals.var_vxbgmtcl__blk925_dn10), (-locals.var_vxbgmtcl__blk925_dn11), (-locals.var_vxbgmtcl__blk925_dn12), (-locals.var_vxbgmtcl__blk925_dn17),)
    } else {
        (locals.var_vgb_fb_ld__blk937, locals.var_vgb_fb_ld__blk937_dn0, locals.var_vgb_fb_ld__blk937_dn2, locals.var_vgb_fb_ld__blk937_dn6, locals.var_vgb_fb_ld__blk937_dn7, locals.var_vgb_fb_ld__blk937_dn10, locals.var_vgb_fb_ld__blk937_dn11, locals.var_vgb_fb_ld__blk937_dn12, locals.var_vgb_fb_ld__blk937_dn17,)
    }
};
        locals.var_vgb_fb_ld__blk937 = assign28800_e40588;
        locals.var_vgb_fb_ld__blk937_dn0 = assign28800_e40588_d_n0;
        locals.var_vgb_fb_ld__blk937_dn2 = assign28800_e40588_d_n2;
        locals.var_vgb_fb_ld__blk937_dn6 = assign28800_e40588_d_n6;
        locals.var_vgb_fb_ld__blk937_dn7 = assign28800_e40588_d_n7;
        locals.var_vgb_fb_ld__blk937_dn10 = assign28800_e40588_d_n10;
        locals.var_vgb_fb_ld__blk937_dn11 = assign28800_e40588_d_n11;
        locals.var_vgb_fb_ld__blk937_dn12 = assign28800_e40588_d_n12;
        locals.var_vgb_fb_ld__blk937_dn17 = assign28800_e40588_d_n17;
        locals.var_vgb_fb_ld__blk937_rv = 0.0;

        let assign28810_e40591: f64 = if locals.var_vgpld__blk935 < locals.var_vgb_fb_ld__blk937 { 1.0 } else { 0.0 };
        locals.var_guard986 = assign28810_e40591;
        locals.var_guard986_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_104(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28830_e40618, assign28830_e40618_d_n0, assign28830_e40618_d_n2, assign28830_e40618_d_n6, assign28830_e40618_d_n7, assign28830_e40618_d_n10, assign28830_e40618_d_n11, assign28830_e40618_d_n12, assign28830_e40618_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign28830_e40615: f64 = (locals.var_beta * locals.var_cnst0over__blk932);
        let assign28830_e40616: f64 = (1.0 / assign28830_e40615);
        (assign28830_e40616, (-((locals.var_beta * locals.var_cnst0over__blk932_dn0) / (assign28830_e40615 * assign28830_e40615))), (-((locals.var_beta * locals.var_cnst0over__blk932_dn2) / (assign28830_e40615 * assign28830_e40615))), (-((locals.var_beta * locals.var_cnst0over__blk932_dn6) / (assign28830_e40615 * assign28830_e40615))), (-((locals.var_beta * locals.var_cnst0over__blk932_dn7) / (assign28830_e40615 * assign28830_e40615))), (-(((locals.var_beta_dn10 * locals.var_cnst0over__blk932) + (locals.var_beta * locals.var_cnst0over__blk932_dn10)) / (assign28830_e40615 * assign28830_e40615))), (-((locals.var_beta * locals.var_cnst0over__blk932_dn11) / (assign28830_e40615 * assign28830_e40615))), (-((locals.var_beta * locals.var_cnst0over__blk932_dn12) / (assign28830_e40615 * assign28830_e40615))), (-((locals.var_beta * locals.var_cnst0over__blk932_dn17) / (assign28830_e40615 * assign28830_e40615))),)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign28830_e40618;
        locals.var_t1__blk900_dn0 = assign28830_e40618_d_n0;
        locals.var_t1__blk900_dn2 = assign28830_e40618_d_n2;
        locals.var_t1__blk900_dn6 = assign28830_e40618_d_n6;
        locals.var_t1__blk900_dn7 = assign28830_e40618_d_n7;
        locals.var_t1__blk900_dn10 = assign28830_e40618_d_n10;
        locals.var_t1__blk900_dn11 = assign28830_e40618_d_n11;
        locals.var_t1__blk900_dn12 = assign28830_e40618_d_n12;
        locals.var_t1__blk900_dn17 = assign28830_e40618_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign28840_e40631, assign28840_e40631_d_n0, assign28840_e40631_d_n2, assign28840_e40631_d_n6, assign28840_e40631_d_n7, assign28840_e40631_d_n10, assign28840_e40631_d_n11, assign28840_e40631_d_n12, assign28840_e40631_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign28840_e40629: f64 = (locals.var_t1__blk900 * locals.var_cox0__blk910);
        (assign28840_e40629, (locals.var_t1__blk900_dn0 * locals.var_cox0__blk910), (locals.var_t1__blk900_dn2 * locals.var_cox0__blk910), (locals.var_t1__blk900_dn6 * locals.var_cox0__blk910), (locals.var_t1__blk900_dn7 * locals.var_cox0__blk910), (locals.var_t1__blk900_dn10 * locals.var_cox0__blk910), (locals.var_t1__blk900_dn11 * locals.var_cox0__blk910), (locals.var_t1__blk900_dn12 * locals.var_cox0__blk910), (locals.var_t1__blk900_dn17 * locals.var_cox0__blk910),)
    } else {
        (locals.var_ty__blk909, locals.var_ty__blk909_dn0, locals.var_ty__blk909_dn2, locals.var_ty__blk909_dn6, locals.var_ty__blk909_dn7, locals.var_ty__blk909_dn10, locals.var_ty__blk909_dn11, locals.var_ty__blk909_dn12, locals.var_ty__blk909_dn17,)
    }
};
        locals.var_ty__blk909 = assign28840_e40631;
        locals.var_ty__blk909_dn0 = assign28840_e40631_d_n0;
        locals.var_ty__blk909_dn2 = assign28840_e40631_d_n2;
        locals.var_ty__blk909_dn6 = assign28840_e40631_d_n6;
        locals.var_ty__blk909_dn7 = assign28840_e40631_d_n7;
        locals.var_ty__blk909_dn10 = assign28840_e40631_d_n10;
        locals.var_ty__blk909_dn11 = assign28840_e40631_d_n11;
        locals.var_ty__blk909_dn12 = assign28840_e40631_d_n12;
        locals.var_ty__blk909_dn17 = assign28840_e40631_d_n17;
        locals.var_ty__blk909_rv = 0.0;

        let (assign28850_e40648, assign28850_e40648_d_n0, assign28850_e40648_d_n2, assign28850_e40648_d_n6, assign28850_e40648_d_n7, assign28850_e40648_d_n10, assign28850_e40648_d_n11, assign28850_e40648_d_n12, assign28850_e40648_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign28850_e40643: f64 = (3.0 * 1.414213562373095);
        let assign28850_e40645: f64 = (assign28850_e40643 * locals.var_ty__blk909);
        let assign28850_e40646: f64 = (2.0 + assign28850_e40645);
        (assign28850_e40646, (assign28850_e40643 * locals.var_ty__blk909_dn0), (assign28850_e40643 * locals.var_ty__blk909_dn2), (assign28850_e40643 * locals.var_ty__blk909_dn6), (assign28850_e40643 * locals.var_ty__blk909_dn7), (assign28850_e40643 * locals.var_ty__blk909_dn10), (assign28850_e40643 * locals.var_ty__blk909_dn11), (assign28850_e40643 * locals.var_ty__blk909_dn12), (assign28850_e40643 * locals.var_ty__blk909_dn17),)
    } else {
        (locals.var_ac41__blk938, locals.var_ac41__blk938_dn0, locals.var_ac41__blk938_dn2, locals.var_ac41__blk938_dn6, locals.var_ac41__blk938_dn7, locals.var_ac41__blk938_dn10, locals.var_ac41__blk938_dn11, locals.var_ac41__blk938_dn12, locals.var_ac41__blk938_dn17,)
    }
};
        locals.var_ac41__blk938 = assign28850_e40648;
        locals.var_ac41__blk938_dn0 = assign28850_e40648_d_n0;
        locals.var_ac41__blk938_dn2 = assign28850_e40648_d_n2;
        locals.var_ac41__blk938_dn6 = assign28850_e40648_d_n6;
        locals.var_ac41__blk938_dn7 = assign28850_e40648_d_n7;
        locals.var_ac41__blk938_dn10 = assign28850_e40648_d_n10;
        locals.var_ac41__blk938_dn11 = assign28850_e40648_d_n11;
        locals.var_ac41__blk938_dn12 = assign28850_e40648_d_n12;
        locals.var_ac41__blk938_dn17 = assign28850_e40648_d_n17;
        locals.var_ac41__blk938_rv = 0.0;

        let (assign28860_e40665, assign28860_e40665_d_n0, assign28860_e40665_d_n2, assign28860_e40665_d_n6, assign28860_e40665_d_n7, assign28860_e40665_d_n10, assign28860_e40665_d_n11, assign28860_e40665_d_n12, assign28860_e40665_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign28860_e40659: f64 = (8.0 * locals.var_ac41__blk938);
        let assign28860_e40661: f64 = (assign28860_e40659 * locals.var_ac41__blk938);
        let assign28860_e40663: f64 = (assign28860_e40661 * locals.var_ac41__blk938);
        (assign28860_e40663, (((((8.0 * locals.var_ac41__blk938_dn0) * locals.var_ac41__blk938) + (assign28860_e40659 * locals.var_ac41__blk938_dn0)) * locals.var_ac41__blk938) + (assign28860_e40661 * locals.var_ac41__blk938_dn0)), (((((8.0 * locals.var_ac41__blk938_dn2) * locals.var_ac41__blk938) + (assign28860_e40659 * locals.var_ac41__blk938_dn2)) * locals.var_ac41__blk938) + (assign28860_e40661 * locals.var_ac41__blk938_dn2)), (((((8.0 * locals.var_ac41__blk938_dn6) * locals.var_ac41__blk938) + (assign28860_e40659 * locals.var_ac41__blk938_dn6)) * locals.var_ac41__blk938) + (assign28860_e40661 * locals.var_ac41__blk938_dn6)), (((((8.0 * locals.var_ac41__blk938_dn7) * locals.var_ac41__blk938) + (assign28860_e40659 * locals.var_ac41__blk938_dn7)) * locals.var_ac41__blk938) + (assign28860_e40661 * locals.var_ac41__blk938_dn7)), (((((8.0 * locals.var_ac41__blk938_dn10) * locals.var_ac41__blk938) + (assign28860_e40659 * locals.var_ac41__blk938_dn10)) * locals.var_ac41__blk938) + (assign28860_e40661 * locals.var_ac41__blk938_dn10)), (((((8.0 * locals.var_ac41__blk938_dn11) * locals.var_ac41__blk938) + (assign28860_e40659 * locals.var_ac41__blk938_dn11)) * locals.var_ac41__blk938) + (assign28860_e40661 * locals.var_ac41__blk938_dn11)), (((((8.0 * locals.var_ac41__blk938_dn12) * locals.var_ac41__blk938) + (assign28860_e40659 * locals.var_ac41__blk938_dn12)) * locals.var_ac41__blk938) + (assign28860_e40661 * locals.var_ac41__blk938_dn12)), (((((8.0 * locals.var_ac41__blk938_dn17) * locals.var_ac41__blk938) + (assign28860_e40659 * locals.var_ac41__blk938_dn17)) * locals.var_ac41__blk938) + (assign28860_e40661 * locals.var_ac41__blk938_dn17)),)
    } else {
        (locals.var_ac4__blk939, locals.var_ac4__blk939_dn0, locals.var_ac4__blk939_dn2, locals.var_ac4__blk939_dn6, locals.var_ac4__blk939_dn7, locals.var_ac4__blk939_dn10, locals.var_ac4__blk939_dn11, locals.var_ac4__blk939_dn12, locals.var_ac4__blk939_dn17,)
    }
};
        locals.var_ac4__blk939 = assign28860_e40665;
        locals.var_ac4__blk939_dn0 = assign28860_e40665_d_n0;
        locals.var_ac4__blk939_dn2 = assign28860_e40665_d_n2;
        locals.var_ac4__blk939_dn6 = assign28860_e40665_d_n6;
        locals.var_ac4__blk939_dn7 = assign28860_e40665_d_n7;
        locals.var_ac4__blk939_dn10 = assign28860_e40665_d_n10;
        locals.var_ac4__blk939_dn11 = assign28860_e40665_d_n11;
        locals.var_ac4__blk939_dn12 = assign28860_e40665_d_n12;
        locals.var_ac4__blk939_dn17 = assign28860_e40665_d_n17;
        locals.var_ac4__blk939_rv = 0.0;

        let (assign28870_e40678, assign28870_e40678_d_n0, assign28870_e40678_d_n2, assign28870_e40678_d_n6, assign28870_e40678_d_n7, assign28870_e40678_d_n10, assign28870_e40678_d_n11, assign28870_e40678_d_n12, assign28870_e40678_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign28870_e40676: f64 = (locals.var_eg - locals.var_pb2over__blk936);
        (assign28870_e40676, (locals.var_eg_dn0 - locals.var_pb2over__blk936_dn0), (locals.var_eg_dn2 - locals.var_pb2over__blk936_dn2), (locals.var_eg_dn6 - locals.var_pb2over__blk936_dn6), (locals.var_eg_dn7 - locals.var_pb2over__blk936_dn7), (locals.var_eg_dn10 - locals.var_pb2over__blk936_dn10), (locals.var_eg_dn11 - locals.var_pb2over__blk936_dn11), (locals.var_eg_dn12 - locals.var_pb2over__blk936_dn12), (locals.var_eg_dn17 - locals.var_pb2over__blk936_dn17),)
    } else {
        (locals.var_ps0_min__blk940, locals.var_ps0_min__blk940_dn0, locals.var_ps0_min__blk940_dn2, locals.var_ps0_min__blk940_dn6, locals.var_ps0_min__blk940_dn7, locals.var_ps0_min__blk940_dn10, locals.var_ps0_min__blk940_dn11, locals.var_ps0_min__blk940_dn12, locals.var_ps0_min__blk940_dn17,)
    }
};
        locals.var_ps0_min__blk940 = assign28870_e40678;
        locals.var_ps0_min__blk940_dn0 = assign28870_e40678_d_n0;
        locals.var_ps0_min__blk940_dn2 = assign28870_e40678_d_n2;
        locals.var_ps0_min__blk940_dn6 = assign28870_e40678_d_n6;
        locals.var_ps0_min__blk940_dn7 = assign28870_e40678_d_n7;
        locals.var_ps0_min__blk940_dn10 = assign28870_e40678_d_n10;
        locals.var_ps0_min__blk940_dn11 = assign28870_e40678_d_n11;
        locals.var_ps0_min__blk940_dn12 = assign28870_e40678_d_n12;
        locals.var_ps0_min__blk940_dn17 = assign28870_e40678_d_n17;
        locals.var_ps0_min__blk940_rv = 0.0;

        let (assign28880_e40693, assign28880_e40693_d_n0, assign28880_e40693_d_n2, assign28880_e40693_d_n6, assign28880_e40693_d_n7, assign28880_e40693_d_n10, assign28880_e40693_d_n11, assign28880_e40693_d_n12, assign28880_e40693_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign28880_e40690: f64 = (locals.var_vgpld__blk935 + locals.var_vxbgmtcl__blk925);
        let assign28880_e40691: f64 = (locals.var_beta * assign28880_e40690);
        (assign28880_e40691, (locals.var_beta * (locals.var_vgpld__blk935_dn0 + locals.var_vxbgmtcl__blk925_dn0)), (locals.var_beta * (locals.var_vgpld__blk935_dn2 + locals.var_vxbgmtcl__blk925_dn2)), (locals.var_beta * (locals.var_vgpld__blk935_dn6 + locals.var_vxbgmtcl__blk925_dn6)), (locals.var_beta * (locals.var_vgpld__blk935_dn7 + locals.var_vxbgmtcl__blk925_dn7)), ((locals.var_beta_dn10 * assign28880_e40690) + (locals.var_beta * (locals.var_vgpld__blk935_dn10 + locals.var_vxbgmtcl__blk925_dn10))), (locals.var_beta * (locals.var_vgpld__blk935_dn11 + locals.var_vxbgmtcl__blk925_dn11)), (locals.var_beta * (locals.var_vgpld__blk935_dn12 + locals.var_vxbgmtcl__blk925_dn12)), (locals.var_beta * (locals.var_vgpld__blk935_dn17 + locals.var_vxbgmtcl__blk925_dn17)),)
    } else {
        (locals.var_tx__blk908, locals.var_tx__blk908_dn0, locals.var_tx__blk908_dn2, locals.var_tx__blk908_dn6, locals.var_tx__blk908_dn7, locals.var_tx__blk908_dn10, locals.var_tx__blk908_dn11, locals.var_tx__blk908_dn12, locals.var_tx__blk908_dn17,)
    }
};
        locals.var_tx__blk908 = assign28880_e40693;
        locals.var_tx__blk908_dn0 = assign28880_e40693_d_n0;
        locals.var_tx__blk908_dn2 = assign28880_e40693_d_n2;
        locals.var_tx__blk908_dn6 = assign28880_e40693_d_n6;
        locals.var_tx__blk908_dn7 = assign28880_e40693_d_n7;
        locals.var_tx__blk908_dn10 = assign28880_e40693_d_n10;
        locals.var_tx__blk908_dn11 = assign28880_e40693_d_n11;
        locals.var_tx__blk908_dn12 = assign28880_e40693_d_n12;
        locals.var_tx__blk908_dn17 = assign28880_e40693_d_n17;
        locals.var_tx__blk908_rv = 0.0;

        let (assign28890_e40714, assign28890_e40714_d_n0, assign28890_e40714_d_n2, assign28890_e40714_d_n6, assign28890_e40714_d_n7, assign28890_e40714_d_n10, assign28890_e40714_d_n11, assign28890_e40714_d_n12, assign28890_e40714_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign28890_e40704: f64 = (7.0 * 1.414213562373095);
        let assign28890_e40707: f64 = (9.0 * locals.var_ty__blk909);
        let assign28890_e40710: f64 = (locals.var_tx__blk908 - 2.0);
        let assign28890_e40711: f64 = (assign28890_e40707 * assign28890_e40710);
        let assign28890_e40712: f64 = (assign28890_e40704 - assign28890_e40711);
        (assign28890_e40712, (-(((9.0 * locals.var_ty__blk909_dn0) * assign28890_e40710) + (assign28890_e40707 * locals.var_tx__blk908_dn0))), (-(((9.0 * locals.var_ty__blk909_dn2) * assign28890_e40710) + (assign28890_e40707 * locals.var_tx__blk908_dn2))), (-(((9.0 * locals.var_ty__blk909_dn6) * assign28890_e40710) + (assign28890_e40707 * locals.var_tx__blk908_dn6))), (-(((9.0 * locals.var_ty__blk909_dn7) * assign28890_e40710) + (assign28890_e40707 * locals.var_tx__blk908_dn7))), (-(((9.0 * locals.var_ty__blk909_dn10) * assign28890_e40710) + (assign28890_e40707 * locals.var_tx__blk908_dn10))), (-(((9.0 * locals.var_ty__blk909_dn11) * assign28890_e40710) + (assign28890_e40707 * locals.var_tx__blk908_dn11))), (-(((9.0 * locals.var_ty__blk909_dn12) * assign28890_e40710) + (assign28890_e40707 * locals.var_tx__blk908_dn12))), (-(((9.0 * locals.var_ty__blk909_dn17) * assign28890_e40710) + (assign28890_e40707 * locals.var_tx__blk908_dn17))),)
    } else {
        (locals.var_ac31__blk941, locals.var_ac31__blk941_dn0, locals.var_ac31__blk941_dn2, locals.var_ac31__blk941_dn6, locals.var_ac31__blk941_dn7, locals.var_ac31__blk941_dn10, locals.var_ac31__blk941_dn11, locals.var_ac31__blk941_dn12, locals.var_ac31__blk941_dn17,)
    }
};
        locals.var_ac31__blk941 = assign28890_e40714;
        locals.var_ac31__blk941_dn0 = assign28890_e40714_d_n0;
        locals.var_ac31__blk941_dn2 = assign28890_e40714_d_n2;
        locals.var_ac31__blk941_dn6 = assign28890_e40714_d_n6;
        locals.var_ac31__blk941_dn7 = assign28890_e40714_d_n7;
        locals.var_ac31__blk941_dn10 = assign28890_e40714_d_n10;
        locals.var_ac31__blk941_dn11 = assign28890_e40714_d_n11;
        locals.var_ac31__blk941_dn12 = assign28890_e40714_d_n12;
        locals.var_ac31__blk941_dn17 = assign28890_e40714_d_n17;
        locals.var_ac31__blk941_rv = 0.0;

        let (assign28900_e40727, assign28900_e40727_d_n0, assign28900_e40727_d_n2, assign28900_e40727_d_n6, assign28900_e40727_d_n7, assign28900_e40727_d_n10, assign28900_e40727_d_n11, assign28900_e40727_d_n12, assign28900_e40727_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign28900_e40725: f64 = (locals.var_ac31__blk941 * locals.var_ac31__blk941);
        (assign28900_e40725, ((locals.var_ac31__blk941_dn0 * locals.var_ac31__blk941) + (locals.var_ac31__blk941 * locals.var_ac31__blk941_dn0)), ((locals.var_ac31__blk941_dn2 * locals.var_ac31__blk941) + (locals.var_ac31__blk941 * locals.var_ac31__blk941_dn2)), ((locals.var_ac31__blk941_dn6 * locals.var_ac31__blk941) + (locals.var_ac31__blk941 * locals.var_ac31__blk941_dn6)), ((locals.var_ac31__blk941_dn7 * locals.var_ac31__blk941) + (locals.var_ac31__blk941 * locals.var_ac31__blk941_dn7)), ((locals.var_ac31__blk941_dn10 * locals.var_ac31__blk941) + (locals.var_ac31__blk941 * locals.var_ac31__blk941_dn10)), ((locals.var_ac31__blk941_dn11 * locals.var_ac31__blk941) + (locals.var_ac31__blk941 * locals.var_ac31__blk941_dn11)), ((locals.var_ac31__blk941_dn12 * locals.var_ac31__blk941) + (locals.var_ac31__blk941 * locals.var_ac31__blk941_dn12)), ((locals.var_ac31__blk941_dn17 * locals.var_ac31__blk941) + (locals.var_ac31__blk941 * locals.var_ac31__blk941_dn17)),)
    } else {
        (locals.var_ac3__blk942, locals.var_ac3__blk942_dn0, locals.var_ac3__blk942_dn2, locals.var_ac3__blk942_dn6, locals.var_ac3__blk942_dn7, locals.var_ac3__blk942_dn10, locals.var_ac3__blk942_dn11, locals.var_ac3__blk942_dn12, locals.var_ac3__blk942_dn17,)
    }
};
        locals.var_ac3__blk942 = assign28900_e40727;
        locals.var_ac3__blk942_dn0 = assign28900_e40727_d_n0;
        locals.var_ac3__blk942_dn2 = assign28900_e40727_d_n2;
        locals.var_ac3__blk942_dn6 = assign28900_e40727_d_n6;
        locals.var_ac3__blk942_dn7 = assign28900_e40727_d_n7;
        locals.var_ac3__blk942_dn10 = assign28900_e40727_d_n10;
        locals.var_ac3__blk942_dn11 = assign28900_e40727_d_n11;
        locals.var_ac3__blk942_dn12 = assign28900_e40727_d_n12;
        locals.var_ac3__blk942_dn17 = assign28900_e40727_d_n17;
        locals.var_ac3__blk942_rv = 0.0;

        let assign28910_e40731: f64 = (locals.var_ac3__blk942 * 1e-8);
        let assign28910_e40732: f64 = if locals.var_ac4__blk939 < assign28910_e40731 { 1.0 } else { 0.0 };
        locals.var_guard987 = assign28910_e40732;
        locals.var_guard987_rv = 0.0;

        let (assign28920_e40764, assign28920_e40764_d_n0, assign28920_e40764_d_n2, assign28920_e40764_d_n6, assign28920_e40764_d_n7, assign28920_e40764_d_n10, assign28920_e40764_d_n11, assign28920_e40764_d_n12, assign28920_e40764_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign28920_e40744: f64 = (-7.0);
        let assign28920_e40746: f64 = (assign28920_e40744 * 1.414213562373095);
        let assign28920_e40748: f64 = (assign28920_e40746 + locals.var_ac31__blk941);
        let assign28920_e40751: f64 = (0.5 * locals.var_ac4__blk939);
        let assign28920_e40753: f64 = (assign28920_e40751 / locals.var_ac31__blk941);
        let assign28920_e40754: f64 = (assign28920_e40748 + assign28920_e40753);
        let assign28920_e40757: f64 = (9.0 * locals.var_ty__blk909);
        let assign28920_e40760: f64 = (locals.var_tx__blk908 - 2.0);
        let assign28920_e40761: f64 = (assign28920_e40757 * assign28920_e40760);
        let assign28920_e40762: f64 = (assign28920_e40754 + assign28920_e40761);
        (assign28920_e40762, ((locals.var_ac31__blk941_dn0 + ((((0.5 * locals.var_ac4__blk939_dn0) * locals.var_ac31__blk941) - (assign28920_e40751 * locals.var_ac31__blk941_dn0)) / (locals.var_ac31__blk941 * locals.var_ac31__blk941))) + (((9.0 * locals.var_ty__blk909_dn0) * assign28920_e40760) + (assign28920_e40757 * locals.var_tx__blk908_dn0))), ((locals.var_ac31__blk941_dn2 + ((((0.5 * locals.var_ac4__blk939_dn2) * locals.var_ac31__blk941) - (assign28920_e40751 * locals.var_ac31__blk941_dn2)) / (locals.var_ac31__blk941 * locals.var_ac31__blk941))) + (((9.0 * locals.var_ty__blk909_dn2) * assign28920_e40760) + (assign28920_e40757 * locals.var_tx__blk908_dn2))), ((locals.var_ac31__blk941_dn6 + ((((0.5 * locals.var_ac4__blk939_dn6) * locals.var_ac31__blk941) - (assign28920_e40751 * locals.var_ac31__blk941_dn6)) / (locals.var_ac31__blk941 * locals.var_ac31__blk941))) + (((9.0 * locals.var_ty__blk909_dn6) * assign28920_e40760) + (assign28920_e40757 * locals.var_tx__blk908_dn6))), ((locals.var_ac31__blk941_dn7 + ((((0.5 * locals.var_ac4__blk939_dn7) * locals.var_ac31__blk941) - (assign28920_e40751 * locals.var_ac31__blk941_dn7)) / (locals.var_ac31__blk941 * locals.var_ac31__blk941))) + (((9.0 * locals.var_ty__blk909_dn7) * assign28920_e40760) + (assign28920_e40757 * locals.var_tx__blk908_dn7))), ((locals.var_ac31__blk941_dn10 + ((((0.5 * locals.var_ac4__blk939_dn10) * locals.var_ac31__blk941) - (assign28920_e40751 * locals.var_ac31__blk941_dn10)) / (locals.var_ac31__blk941 * locals.var_ac31__blk941))) + (((9.0 * locals.var_ty__blk909_dn10) * assign28920_e40760) + (assign28920_e40757 * locals.var_tx__blk908_dn10))), ((locals.var_ac31__blk941_dn11 + ((((0.5 * locals.var_ac4__blk939_dn11) * locals.var_ac31__blk941) - (assign28920_e40751 * locals.var_ac31__blk941_dn11)) / (locals.var_ac31__blk941 * locals.var_ac31__blk941))) + (((9.0 * locals.var_ty__blk909_dn11) * assign28920_e40760) + (assign28920_e40757 * locals.var_tx__blk908_dn11))), ((locals.var_ac31__blk941_dn12 + ((((0.5 * locals.var_ac4__blk939_dn12) * locals.var_ac31__blk941) - (assign28920_e40751 * locals.var_ac31__blk941_dn12)) / (locals.var_ac31__blk941 * locals.var_ac31__blk941))) + (((9.0 * locals.var_ty__blk909_dn12) * assign28920_e40760) + (assign28920_e40757 * locals.var_tx__blk908_dn12))), ((locals.var_ac31__blk941_dn17 + ((((0.5 * locals.var_ac4__blk939_dn17) * locals.var_ac31__blk941) - (assign28920_e40751 * locals.var_ac31__blk941_dn17)) / (locals.var_ac31__blk941 * locals.var_ac31__blk941))) + (((9.0 * locals.var_ty__blk909_dn17) * assign28920_e40760) + (assign28920_e40757 * locals.var_tx__blk908_dn17))),)
    } else {
        (locals.var_ac1__blk944, locals.var_ac1__blk944_dn0, locals.var_ac1__blk944_dn2, locals.var_ac1__blk944_dn6, locals.var_ac1__blk944_dn7, locals.var_ac1__blk944_dn10, locals.var_ac1__blk944_dn11, locals.var_ac1__blk944_dn12, locals.var_ac1__blk944_dn17,)
    }
};
        locals.var_ac1__blk944 = assign28920_e40764;
        locals.var_ac1__blk944_dn0 = assign28920_e40764_d_n0;
        locals.var_ac1__blk944_dn2 = assign28920_e40764_d_n2;
        locals.var_ac1__blk944_dn6 = assign28920_e40764_d_n6;
        locals.var_ac1__blk944_dn7 = assign28920_e40764_d_n7;
        locals.var_ac1__blk944_dn10 = assign28920_e40764_d_n10;
        locals.var_ac1__blk944_dn11 = assign28920_e40764_d_n11;
        locals.var_ac1__blk944_dn12 = assign28920_e40764_d_n12;
        locals.var_ac1__blk944_dn17 = assign28920_e40764_d_n17;
        locals.var_ac1__blk944_rv = 0.0;

        let (assign28930_e40781, assign28930_e40781_d_n0, assign28930_e40781_d_n2, assign28930_e40781_d_n6, assign28930_e40781_d_n7, assign28930_e40781_d_n10, assign28930_e40781_d_n11, assign28930_e40781_d_n12, assign28930_e40781_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) && (locals.var_guard987 == 0.0)) {
        let assign28930_e40778: f64 = (locals.var_ac4__blk939 + locals.var_ac3__blk942);
        let assign28930_e40779: f64 = (assign28930_e40778).sqrt();
        (assign28930_e40779, ((locals.var_ac4__blk939_dn0 + locals.var_ac3__blk942_dn0) / (2.0 * assign28930_e40779)), ((locals.var_ac4__blk939_dn2 + locals.var_ac3__blk942_dn2) / (2.0 * assign28930_e40779)), ((locals.var_ac4__blk939_dn6 + locals.var_ac3__blk942_dn6) / (2.0 * assign28930_e40779)), ((locals.var_ac4__blk939_dn7 + locals.var_ac3__blk942_dn7) / (2.0 * assign28930_e40779)), ((locals.var_ac4__blk939_dn10 + locals.var_ac3__blk942_dn10) / (2.0 * assign28930_e40779)), ((locals.var_ac4__blk939_dn11 + locals.var_ac3__blk942_dn11) / (2.0 * assign28930_e40779)), ((locals.var_ac4__blk939_dn12 + locals.var_ac3__blk942_dn12) / (2.0 * assign28930_e40779)), ((locals.var_ac4__blk939_dn17 + locals.var_ac3__blk942_dn17) / (2.0 * assign28930_e40779)),)
    } else {
        (locals.var_ac2__blk943, locals.var_ac2__blk943_dn0, locals.var_ac2__blk943_dn2, locals.var_ac2__blk943_dn6, locals.var_ac2__blk943_dn7, locals.var_ac2__blk943_dn10, locals.var_ac2__blk943_dn11, locals.var_ac2__blk943_dn12, locals.var_ac2__blk943_dn17,)
    }
};
        locals.var_ac2__blk943 = assign28930_e40781;
        locals.var_ac2__blk943_dn0 = assign28930_e40781_d_n0;
        locals.var_ac2__blk943_dn2 = assign28930_e40781_d_n2;
        locals.var_ac2__blk943_dn6 = assign28930_e40781_d_n6;
        locals.var_ac2__blk943_dn7 = assign28930_e40781_d_n7;
        locals.var_ac2__blk943_dn10 = assign28930_e40781_d_n10;
        locals.var_ac2__blk943_dn11 = assign28930_e40781_d_n11;
        locals.var_ac2__blk943_dn12 = assign28930_e40781_d_n12;
        locals.var_ac2__blk943_dn17 = assign28930_e40781_d_n17;
        locals.var_ac2__blk943_rv = 0.0;

        let (assign28940_e40808, assign28940_e40808_d_n0, assign28940_e40808_d_n2, assign28940_e40808_d_n6, assign28940_e40808_d_n7, assign28940_e40808_d_n10, assign28940_e40808_d_n11, assign28940_e40808_d_n12, assign28940_e40808_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) && (locals.var_guard987 == 0.0)) {
        let assign28940_e40794: f64 = (-7.0);
        let assign28940_e40796: f64 = (assign28940_e40794 * 1.414213562373095);
        let assign28940_e40798: f64 = (assign28940_e40796 + locals.var_ac2__blk943);
        let assign28940_e40801: f64 = (9.0 * locals.var_ty__blk909);
        let assign28940_e40804: f64 = (locals.var_tx__blk908 - 2.0);
        let assign28940_e40805: f64 = (assign28940_e40801 * assign28940_e40804);
        let assign28940_e40806: f64 = (assign28940_e40798 + assign28940_e40805);
        (assign28940_e40806, (locals.var_ac2__blk943_dn0 + (((9.0 * locals.var_ty__blk909_dn0) * assign28940_e40804) + (assign28940_e40801 * locals.var_tx__blk908_dn0))), (locals.var_ac2__blk943_dn2 + (((9.0 * locals.var_ty__blk909_dn2) * assign28940_e40804) + (assign28940_e40801 * locals.var_tx__blk908_dn2))), (locals.var_ac2__blk943_dn6 + (((9.0 * locals.var_ty__blk909_dn6) * assign28940_e40804) + (assign28940_e40801 * locals.var_tx__blk908_dn6))), (locals.var_ac2__blk943_dn7 + (((9.0 * locals.var_ty__blk909_dn7) * assign28940_e40804) + (assign28940_e40801 * locals.var_tx__blk908_dn7))), (locals.var_ac2__blk943_dn10 + (((9.0 * locals.var_ty__blk909_dn10) * assign28940_e40804) + (assign28940_e40801 * locals.var_tx__blk908_dn10))), (locals.var_ac2__blk943_dn11 + (((9.0 * locals.var_ty__blk909_dn11) * assign28940_e40804) + (assign28940_e40801 * locals.var_tx__blk908_dn11))), (locals.var_ac2__blk943_dn12 + (((9.0 * locals.var_ty__blk909_dn12) * assign28940_e40804) + (assign28940_e40801 * locals.var_tx__blk908_dn12))), (locals.var_ac2__blk943_dn17 + (((9.0 * locals.var_ty__blk909_dn17) * assign28940_e40804) + (assign28940_e40801 * locals.var_tx__blk908_dn17))),)
    } else {
        (locals.var_ac1__blk944, locals.var_ac1__blk944_dn0, locals.var_ac1__blk944_dn2, locals.var_ac1__blk944_dn6, locals.var_ac1__blk944_dn7, locals.var_ac1__blk944_dn10, locals.var_ac1__blk944_dn11, locals.var_ac1__blk944_dn12, locals.var_ac1__blk944_dn17,)
    }
};
        locals.var_ac1__blk944 = assign28940_e40808;
        locals.var_ac1__blk944_dn0 = assign28940_e40808_d_n0;
        locals.var_ac1__blk944_dn2 = assign28940_e40808_d_n2;
        locals.var_ac1__blk944_dn6 = assign28940_e40808_d_n6;
        locals.var_ac1__blk944_dn7 = assign28940_e40808_d_n7;
        locals.var_ac1__blk944_dn10 = assign28940_e40808_d_n10;
        locals.var_ac1__blk944_dn11 = assign28940_e40808_d_n11;
        locals.var_ac1__blk944_dn12 = assign28940_e40808_d_n12;
        locals.var_ac1__blk944_dn17 = assign28940_e40808_d_n17;
        locals.var_ac1__blk944_rv = 0.0;

        let (assign28950_e40821, assign28950_e40821_d_n0, assign28950_e40821_d_n2, assign28950_e40821_d_n6, assign28950_e40821_d_n7, assign28950_e40821_d_n10, assign28950_e40821_d_n11, assign28950_e40821_d_n12, assign28950_e40821_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign28950_e40819: f64 = (locals.var_ac1__blk944).powf(0.3333333333333333);
        (assign28950_e40819, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk944).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk944_dn0)) } } else { (assign28950_e40819 * (0.3333333333333333 * (locals.var_ac1__blk944_dn0 / locals.var_ac1__blk944))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk944).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk944_dn2)) } } else { (assign28950_e40819 * (0.3333333333333333 * (locals.var_ac1__blk944_dn2 / locals.var_ac1__blk944))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk944).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk944_dn6)) } } else { (assign28950_e40819 * (0.3333333333333333 * (locals.var_ac1__blk944_dn6 / locals.var_ac1__blk944))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk944).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk944_dn7)) } } else { (assign28950_e40819 * (0.3333333333333333 * (locals.var_ac1__blk944_dn7 / locals.var_ac1__blk944))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk944).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk944_dn10)) } } else { (assign28950_e40819 * (0.3333333333333333 * (locals.var_ac1__blk944_dn10 / locals.var_ac1__blk944))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk944).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk944_dn11)) } } else { (assign28950_e40819 * (0.3333333333333333 * (locals.var_ac1__blk944_dn11 / locals.var_ac1__blk944))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk944).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk944_dn12)) } } else { (assign28950_e40819 * (0.3333333333333333 * (locals.var_ac1__blk944_dn12 / locals.var_ac1__blk944))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk944).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk944_dn17)) } } else { (assign28950_e40819 * (0.3333333333333333 * (locals.var_ac1__blk944_dn17 / locals.var_ac1__blk944))) },)
    } else {
        (locals.var_acd__blk945, locals.var_acd__blk945_dn0, locals.var_acd__blk945_dn2, locals.var_acd__blk945_dn6, locals.var_acd__blk945_dn7, locals.var_acd__blk945_dn10, locals.var_acd__blk945_dn11, locals.var_acd__blk945_dn12, locals.var_acd__blk945_dn17,)
    }
};
        locals.var_acd__blk945 = assign28950_e40821;
        locals.var_acd__blk945_dn0 = assign28950_e40821_d_n0;
        locals.var_acd__blk945_dn2 = assign28950_e40821_d_n2;
        locals.var_acd__blk945_dn6 = assign28950_e40821_d_n6;
        locals.var_acd__blk945_dn7 = assign28950_e40821_d_n7;
        locals.var_acd__blk945_dn10 = assign28950_e40821_d_n10;
        locals.var_acd__blk945_dn11 = assign28950_e40821_d_n11;
        locals.var_acd__blk945_dn12 = assign28950_e40821_d_n12;
        locals.var_acd__blk945_dn17 = assign28950_e40821_d_n17;
        locals.var_acd__blk945_rv = 0.0;

        let (assign28960_e40849, assign28960_e40849_d_n0, assign28960_e40849_d_n2, assign28960_e40849_d_n6, assign28960_e40849_d_n7, assign28960_e40849_d_n10, assign28960_e40849_d_n11, assign28960_e40849_d_n12, assign28960_e40849_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign28960_e40831: f64 = (-4.0);
        let assign28960_e40833: f64 = (assign28960_e40831 * 1.414213562373095);
        let assign28960_e40836: f64 = (12.0 * locals.var_ty__blk909);
        let assign28960_e40837: f64 = (assign28960_e40833 - assign28960_e40836);
        let assign28960_e40840: f64 = (2.0 * locals.var_acd__blk945);
        let assign28960_e40841: f64 = (assign28960_e40837 + assign28960_e40840);
        let assign28960_e40844: f64 = (1.414213562373095 * locals.var_acd__blk945);
        let assign28960_e40846: f64 = (assign28960_e40844 * locals.var_acd__blk945);
        let assign28960_e40847: f64 = (assign28960_e40841 + assign28960_e40846);
        (assign28960_e40847, (((-(12.0 * locals.var_ty__blk909_dn0)) + (2.0 * locals.var_acd__blk945_dn0)) + (((1.414213562373095 * locals.var_acd__blk945_dn0) * locals.var_acd__blk945) + (assign28960_e40844 * locals.var_acd__blk945_dn0))), (((-(12.0 * locals.var_ty__blk909_dn2)) + (2.0 * locals.var_acd__blk945_dn2)) + (((1.414213562373095 * locals.var_acd__blk945_dn2) * locals.var_acd__blk945) + (assign28960_e40844 * locals.var_acd__blk945_dn2))), (((-(12.0 * locals.var_ty__blk909_dn6)) + (2.0 * locals.var_acd__blk945_dn6)) + (((1.414213562373095 * locals.var_acd__blk945_dn6) * locals.var_acd__blk945) + (assign28960_e40844 * locals.var_acd__blk945_dn6))), (((-(12.0 * locals.var_ty__blk909_dn7)) + (2.0 * locals.var_acd__blk945_dn7)) + (((1.414213562373095 * locals.var_acd__blk945_dn7) * locals.var_acd__blk945) + (assign28960_e40844 * locals.var_acd__blk945_dn7))), (((-(12.0 * locals.var_ty__blk909_dn10)) + (2.0 * locals.var_acd__blk945_dn10)) + (((1.414213562373095 * locals.var_acd__blk945_dn10) * locals.var_acd__blk945) + (assign28960_e40844 * locals.var_acd__blk945_dn10))), (((-(12.0 * locals.var_ty__blk909_dn11)) + (2.0 * locals.var_acd__blk945_dn11)) + (((1.414213562373095 * locals.var_acd__blk945_dn11) * locals.var_acd__blk945) + (assign28960_e40844 * locals.var_acd__blk945_dn11))), (((-(12.0 * locals.var_ty__blk909_dn12)) + (2.0 * locals.var_acd__blk945_dn12)) + (((1.414213562373095 * locals.var_acd__blk945_dn12) * locals.var_acd__blk945) + (assign28960_e40844 * locals.var_acd__blk945_dn12))), (((-(12.0 * locals.var_ty__blk909_dn17)) + (2.0 * locals.var_acd__blk945_dn17)) + (((1.414213562373095 * locals.var_acd__blk945_dn17) * locals.var_acd__blk945) + (assign28960_e40844 * locals.var_acd__blk945_dn17))),)
    } else {
        (locals.var_acn__blk946, locals.var_acn__blk946_dn0, locals.var_acn__blk946_dn2, locals.var_acn__blk946_dn6, locals.var_acn__blk946_dn7, locals.var_acn__blk946_dn10, locals.var_acn__blk946_dn11, locals.var_acn__blk946_dn12, locals.var_acn__blk946_dn17,)
    }
};
        locals.var_acn__blk946 = assign28960_e40849;
        locals.var_acn__blk946_dn0 = assign28960_e40849_d_n0;
        locals.var_acn__blk946_dn2 = assign28960_e40849_d_n2;
        locals.var_acn__blk946_dn6 = assign28960_e40849_d_n6;
        locals.var_acn__blk946_dn7 = assign28960_e40849_d_n7;
        locals.var_acn__blk946_dn10 = assign28960_e40849_d_n10;
        locals.var_acn__blk946_dn11 = assign28960_e40849_d_n11;
        locals.var_acn__blk946_dn12 = assign28960_e40849_d_n12;
        locals.var_acn__blk946_dn17 = assign28960_e40849_d_n17;
        locals.var_acn__blk946_rv = 0.0;

        let (assign28970_e40862, assign28970_e40862_d_n0, assign28970_e40862_d_n2, assign28970_e40862_d_n6, assign28970_e40862_d_n7, assign28970_e40862_d_n10, assign28970_e40862_d_n11, assign28970_e40862_d_n12, assign28970_e40862_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign28970_e40860: f64 = (locals.var_acn__blk946 / locals.var_acd__blk945);
        (assign28970_e40860, (((locals.var_acn__blk946_dn0 * locals.var_acd__blk945) - (locals.var_acn__blk946 * locals.var_acd__blk945_dn0)) / (locals.var_acd__blk945 * locals.var_acd__blk945)), (((locals.var_acn__blk946_dn2 * locals.var_acd__blk945) - (locals.var_acn__blk946 * locals.var_acd__blk945_dn2)) / (locals.var_acd__blk945 * locals.var_acd__blk945)), (((locals.var_acn__blk946_dn6 * locals.var_acd__blk945) - (locals.var_acn__blk946 * locals.var_acd__blk945_dn6)) / (locals.var_acd__blk945 * locals.var_acd__blk945)), (((locals.var_acn__blk946_dn7 * locals.var_acd__blk945) - (locals.var_acn__blk946 * locals.var_acd__blk945_dn7)) / (locals.var_acd__blk945 * locals.var_acd__blk945)), (((locals.var_acn__blk946_dn10 * locals.var_acd__blk945) - (locals.var_acn__blk946 * locals.var_acd__blk945_dn10)) / (locals.var_acd__blk945 * locals.var_acd__blk945)), (((locals.var_acn__blk946_dn11 * locals.var_acd__blk945) - (locals.var_acn__blk946 * locals.var_acd__blk945_dn11)) / (locals.var_acd__blk945 * locals.var_acd__blk945)), (((locals.var_acn__blk946_dn12 * locals.var_acd__blk945) - (locals.var_acn__blk946 * locals.var_acd__blk945_dn12)) / (locals.var_acd__blk945 * locals.var_acd__blk945)), (((locals.var_acn__blk946_dn17 * locals.var_acd__blk945) - (locals.var_acn__blk946 * locals.var_acd__blk945_dn17)) / (locals.var_acd__blk945 * locals.var_acd__blk945)),)
    } else {
        (locals.var_chi__blk947, locals.var_chi__blk947_dn0, locals.var_chi__blk947_dn2, locals.var_chi__blk947_dn6, locals.var_chi__blk947_dn7, locals.var_chi__blk947_dn10, locals.var_chi__blk947_dn11, locals.var_chi__blk947_dn12, locals.var_chi__blk947_dn17,)
    }
};
        locals.var_chi__blk947 = assign28970_e40862;
        locals.var_chi__blk947_dn0 = assign28970_e40862_d_n0;
        locals.var_chi__blk947_dn2 = assign28970_e40862_d_n2;
        locals.var_chi__blk947_dn6 = assign28970_e40862_d_n6;
        locals.var_chi__blk947_dn7 = assign28970_e40862_d_n7;
        locals.var_chi__blk947_dn10 = assign28970_e40862_d_n10;
        locals.var_chi__blk947_dn11 = assign28970_e40862_d_n11;
        locals.var_chi__blk947_dn12 = assign28970_e40862_d_n12;
        locals.var_chi__blk947_dn17 = assign28970_e40862_d_n17;
        locals.var_chi__blk947_rv = 0.0;

        let (assign28980_e40877, assign28980_e40877_d_n0, assign28980_e40877_d_n2, assign28980_e40877_d_n6, assign28980_e40877_d_n7, assign28980_e40877_d_n10, assign28980_e40877_d_n11, assign28980_e40877_d_n12, assign28980_e40877_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign28980_e40873: f64 = (locals.var_chi__blk947 * locals.var_beta_inv);
        let assign28980_e40875: f64 = (assign28980_e40873 - locals.var_vxbgmtcl__blk925);
        (assign28980_e40875, ((locals.var_chi__blk947_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn0), ((locals.var_chi__blk947_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn2), ((locals.var_chi__blk947_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn6), ((locals.var_chi__blk947_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn7), (((locals.var_chi__blk947_dn10 * locals.var_beta_inv) + (locals.var_chi__blk947 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl__blk925_dn10), ((locals.var_chi__blk947_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn11), ((locals.var_chi__blk947_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn12), ((locals.var_chi__blk947_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn17),)
    } else {
        (locals.var_psa__blk948, locals.var_psa__blk948_dn0, locals.var_psa__blk948_dn2, locals.var_psa__blk948_dn6, locals.var_psa__blk948_dn7, locals.var_psa__blk948_dn10, locals.var_psa__blk948_dn11, locals.var_psa__blk948_dn12, locals.var_psa__blk948_dn17,)
    }
};
        locals.var_psa__blk948 = assign28980_e40877;
        locals.var_psa__blk948_dn0 = assign28980_e40877_d_n0;
        locals.var_psa__blk948_dn2 = assign28980_e40877_d_n2;
        locals.var_psa__blk948_dn6 = assign28980_e40877_d_n6;
        locals.var_psa__blk948_dn7 = assign28980_e40877_d_n7;
        locals.var_psa__blk948_dn10 = assign28980_e40877_d_n10;
        locals.var_psa__blk948_dn11 = assign28980_e40877_d_n11;
        locals.var_psa__blk948_dn12 = assign28980_e40877_d_n12;
        locals.var_psa__blk948_dn17 = assign28980_e40877_d_n17;
        locals.var_psa__blk948_rv = 0.0;

        let (assign28990_e40890, assign28990_e40890_d_n0, assign28990_e40890_d_n2, assign28990_e40890_d_n6, assign28990_e40890_d_n7, assign28990_e40890_d_n10, assign28990_e40890_d_n11, assign28990_e40890_d_n12, assign28990_e40890_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign28990_e40888: f64 = (locals.var_psa__blk948 + locals.var_vxbgmtcl__blk925);
        (assign28990_e40888, (locals.var_psa__blk948_dn0 + locals.var_vxbgmtcl__blk925_dn0), (locals.var_psa__blk948_dn2 + locals.var_vxbgmtcl__blk925_dn2), (locals.var_psa__blk948_dn6 + locals.var_vxbgmtcl__blk925_dn6), (locals.var_psa__blk948_dn7 + locals.var_vxbgmtcl__blk925_dn7), (locals.var_psa__blk948_dn10 + locals.var_vxbgmtcl__blk925_dn10), (locals.var_psa__blk948_dn11 + locals.var_vxbgmtcl__blk925_dn11), (locals.var_psa__blk948_dn12 + locals.var_vxbgmtcl__blk925_dn12), (locals.var_psa__blk948_dn17 + locals.var_vxbgmtcl__blk925_dn17),)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign28990_e40890;
        locals.var_t1__blk900_dn0 = assign28990_e40890_d_n0;
        locals.var_t1__blk900_dn2 = assign28990_e40890_d_n2;
        locals.var_t1__blk900_dn6 = assign28990_e40890_d_n6;
        locals.var_t1__blk900_dn7 = assign28990_e40890_d_n7;
        locals.var_t1__blk900_dn10 = assign28990_e40890_d_n10;
        locals.var_t1__blk900_dn11 = assign28990_e40890_d_n11;
        locals.var_t1__blk900_dn12 = assign28990_e40890_d_n12;
        locals.var_t1__blk900_dn17 = assign28990_e40890_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign29000_e40903, assign29000_e40903_d_n0, assign29000_e40903_d_n2, assign29000_e40903_d_n6, assign29000_e40903_d_n7, assign29000_e40903_d_n10, assign29000_e40903_d_n11, assign29000_e40903_d_n12, assign29000_e40903_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29000_e40901: f64 = (locals.var_t1__blk900 / locals.var_ps0_min__blk940);
        (assign29000_e40901, (((locals.var_t1__blk900_dn0 * locals.var_ps0_min__blk940) - (locals.var_t1__blk900 * locals.var_ps0_min__blk940_dn0)) / (locals.var_ps0_min__blk940 * locals.var_ps0_min__blk940)), (((locals.var_t1__blk900_dn2 * locals.var_ps0_min__blk940) - (locals.var_t1__blk900 * locals.var_ps0_min__blk940_dn2)) / (locals.var_ps0_min__blk940 * locals.var_ps0_min__blk940)), (((locals.var_t1__blk900_dn6 * locals.var_ps0_min__blk940) - (locals.var_t1__blk900 * locals.var_ps0_min__blk940_dn6)) / (locals.var_ps0_min__blk940 * locals.var_ps0_min__blk940)), (((locals.var_t1__blk900_dn7 * locals.var_ps0_min__blk940) - (locals.var_t1__blk900 * locals.var_ps0_min__blk940_dn7)) / (locals.var_ps0_min__blk940 * locals.var_ps0_min__blk940)), (((locals.var_t1__blk900_dn10 * locals.var_ps0_min__blk940) - (locals.var_t1__blk900 * locals.var_ps0_min__blk940_dn10)) / (locals.var_ps0_min__blk940 * locals.var_ps0_min__blk940)), (((locals.var_t1__blk900_dn11 * locals.var_ps0_min__blk940) - (locals.var_t1__blk900 * locals.var_ps0_min__blk940_dn11)) / (locals.var_ps0_min__blk940 * locals.var_ps0_min__blk940)), (((locals.var_t1__blk900_dn12 * locals.var_ps0_min__blk940) - (locals.var_t1__blk900 * locals.var_ps0_min__blk940_dn12)) / (locals.var_ps0_min__blk940 * locals.var_ps0_min__blk940)), (((locals.var_t1__blk900_dn17 * locals.var_ps0_min__blk940) - (locals.var_t1__blk900 * locals.var_ps0_min__blk940_dn17)) / (locals.var_ps0_min__blk940 * locals.var_ps0_min__blk940)),)
    } else {
        (locals.var_t2__blk901, locals.var_t2__blk901_dn0, locals.var_t2__blk901_dn2, locals.var_t2__blk901_dn6, locals.var_t2__blk901_dn7, locals.var_t2__blk901_dn10, locals.var_t2__blk901_dn11, locals.var_t2__blk901_dn12, locals.var_t2__blk901_dn17,)
    }
};
        locals.var_t2__blk901 = assign29000_e40903;
        locals.var_t2__blk901_dn0 = assign29000_e40903_d_n0;
        locals.var_t2__blk901_dn2 = assign29000_e40903_d_n2;
        locals.var_t2__blk901_dn6 = assign29000_e40903_d_n6;
        locals.var_t2__blk901_dn7 = assign29000_e40903_d_n7;
        locals.var_t2__blk901_dn10 = assign29000_e40903_d_n10;
        locals.var_t2__blk901_dn11 = assign29000_e40903_d_n11;
        locals.var_t2__blk901_dn12 = assign29000_e40903_d_n12;
        locals.var_t2__blk901_dn17 = assign29000_e40903_d_n17;
        locals.var_t2__blk901_rv = 0.0;

        let (assign29010_e40919, assign29010_e40919_d_n0, assign29010_e40919_d_n2, assign29010_e40919_d_n6, assign29010_e40919_d_n7, assign29010_e40919_d_n10, assign29010_e40919_d_n11, assign29010_e40919_d_n12, assign29010_e40919_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29010_e40915: f64 = (locals.var_t2__blk901 * locals.var_t2__blk901);
        let assign29010_e40916: f64 = (1.0 + assign29010_e40915);
        let assign29010_e40917: f64 = (assign29010_e40916).sqrt();
        (assign29010_e40917, (((locals.var_t2__blk901_dn0 * locals.var_t2__blk901) + (locals.var_t2__blk901 * locals.var_t2__blk901_dn0)) / (2.0 * assign29010_e40917)), (((locals.var_t2__blk901_dn2 * locals.var_t2__blk901) + (locals.var_t2__blk901 * locals.var_t2__blk901_dn2)) / (2.0 * assign29010_e40917)), (((locals.var_t2__blk901_dn6 * locals.var_t2__blk901) + (locals.var_t2__blk901 * locals.var_t2__blk901_dn6)) / (2.0 * assign29010_e40917)), (((locals.var_t2__blk901_dn7 * locals.var_t2__blk901) + (locals.var_t2__blk901 * locals.var_t2__blk901_dn7)) / (2.0 * assign29010_e40917)), (((locals.var_t2__blk901_dn10 * locals.var_t2__blk901) + (locals.var_t2__blk901 * locals.var_t2__blk901_dn10)) / (2.0 * assign29010_e40917)), (((locals.var_t2__blk901_dn11 * locals.var_t2__blk901) + (locals.var_t2__blk901 * locals.var_t2__blk901_dn11)) / (2.0 * assign29010_e40917)), (((locals.var_t2__blk901_dn12 * locals.var_t2__blk901) + (locals.var_t2__blk901 * locals.var_t2__blk901_dn12)) / (2.0 * assign29010_e40917)), (((locals.var_t2__blk901_dn17 * locals.var_t2__blk901) + (locals.var_t2__blk901 * locals.var_t2__blk901_dn17)) / (2.0 * assign29010_e40917)),)
    } else {
        (locals.var_t3__blk902, locals.var_t3__blk902_dn0, locals.var_t3__blk902_dn2, locals.var_t3__blk902_dn6, locals.var_t3__blk902_dn7, locals.var_t3__blk902_dn10, locals.var_t3__blk902_dn11, locals.var_t3__blk902_dn12, locals.var_t3__blk902_dn17,)
    }
};
        locals.var_t3__blk902 = assign29010_e40919;
        locals.var_t3__blk902_dn0 = assign29010_e40919_d_n0;
        locals.var_t3__blk902_dn2 = assign29010_e40919_d_n2;
        locals.var_t3__blk902_dn6 = assign29010_e40919_d_n6;
        locals.var_t3__blk902_dn7 = assign29010_e40919_d_n7;
        locals.var_t3__blk902_dn10 = assign29010_e40919_d_n10;
        locals.var_t3__blk902_dn11 = assign29010_e40919_d_n11;
        locals.var_t3__blk902_dn12 = assign29010_e40919_d_n12;
        locals.var_t3__blk902_dn17 = assign29010_e40919_d_n17;
        locals.var_t3__blk902_rv = 0.0;

        let (assign29020_e40934, assign29020_e40934_d_n0, assign29020_e40934_d_n2, assign29020_e40934_d_n6, assign29020_e40934_d_n7, assign29020_e40934_d_n10, assign29020_e40934_d_n11, assign29020_e40934_d_n12, assign29020_e40934_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29020_e40930: f64 = (locals.var_t1__blk900 / locals.var_t3__blk902);
        let assign29020_e40932: f64 = (assign29020_e40930 - locals.var_vxbgmtcl__blk925);
        (assign29020_e40932, ((((locals.var_t1__blk900_dn0 * locals.var_t3__blk902) - (locals.var_t1__blk900 * locals.var_t3__blk902_dn0)) / (locals.var_t3__blk902 * locals.var_t3__blk902)) - locals.var_vxbgmtcl__blk925_dn0), ((((locals.var_t1__blk900_dn2 * locals.var_t3__blk902) - (locals.var_t1__blk900 * locals.var_t3__blk902_dn2)) / (locals.var_t3__blk902 * locals.var_t3__blk902)) - locals.var_vxbgmtcl__blk925_dn2), ((((locals.var_t1__blk900_dn6 * locals.var_t3__blk902) - (locals.var_t1__blk900 * locals.var_t3__blk902_dn6)) / (locals.var_t3__blk902 * locals.var_t3__blk902)) - locals.var_vxbgmtcl__blk925_dn6), ((((locals.var_t1__blk900_dn7 * locals.var_t3__blk902) - (locals.var_t1__blk900 * locals.var_t3__blk902_dn7)) / (locals.var_t3__blk902 * locals.var_t3__blk902)) - locals.var_vxbgmtcl__blk925_dn7), ((((locals.var_t1__blk900_dn10 * locals.var_t3__blk902) - (locals.var_t1__blk900 * locals.var_t3__blk902_dn10)) / (locals.var_t3__blk902 * locals.var_t3__blk902)) - locals.var_vxbgmtcl__blk925_dn10), ((((locals.var_t1__blk900_dn11 * locals.var_t3__blk902) - (locals.var_t1__blk900 * locals.var_t3__blk902_dn11)) / (locals.var_t3__blk902 * locals.var_t3__blk902)) - locals.var_vxbgmtcl__blk925_dn11), ((((locals.var_t1__blk900_dn12 * locals.var_t3__blk902) - (locals.var_t1__blk900 * locals.var_t3__blk902_dn12)) / (locals.var_t3__blk902 * locals.var_t3__blk902)) - locals.var_vxbgmtcl__blk925_dn12), ((((locals.var_t1__blk900_dn17 * locals.var_t3__blk902) - (locals.var_t1__blk900 * locals.var_t3__blk902_dn17)) / (locals.var_t3__blk902 * locals.var_t3__blk902)) - locals.var_vxbgmtcl__blk925_dn17),)
    } else {
        (locals.var_ps0ld__blk949, locals.var_ps0ld__blk949_dn0, locals.var_ps0ld__blk949_dn2, locals.var_ps0ld__blk949_dn6, locals.var_ps0ld__blk949_dn7, locals.var_ps0ld__blk949_dn10, locals.var_ps0ld__blk949_dn11, locals.var_ps0ld__blk949_dn12, locals.var_ps0ld__blk949_dn17,)
    }
};
        locals.var_ps0ld__blk949 = assign29020_e40934;
        locals.var_ps0ld__blk949_dn0 = assign29020_e40934_d_n0;
        locals.var_ps0ld__blk949_dn2 = assign29020_e40934_d_n2;
        locals.var_ps0ld__blk949_dn6 = assign29020_e40934_d_n6;
        locals.var_ps0ld__blk949_dn7 = assign29020_e40934_d_n7;
        locals.var_ps0ld__blk949_dn10 = assign29020_e40934_d_n10;
        locals.var_ps0ld__blk949_dn11 = assign29020_e40934_d_n11;
        locals.var_ps0ld__blk949_dn12 = assign29020_e40934_d_n12;
        locals.var_ps0ld__blk949_dn17 = assign29020_e40934_d_n17;
        locals.var_ps0ld__blk949_rv = 0.0;

        let (assign29030_e40947, assign29030_e40947_d_n0, assign29030_e40947_d_n2, assign29030_e40947_d_n6, assign29030_e40947_d_n7, assign29030_e40947_d_n10, assign29030_e40947_d_n11, assign29030_e40947_d_n12, assign29030_e40947_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29030_e40945: f64 = (locals.var_vgpld__blk935 - locals.var_ps0ld__blk949);
        (assign29030_e40945, (locals.var_vgpld__blk935_dn0 - locals.var_ps0ld__blk949_dn0), (locals.var_vgpld__blk935_dn2 - locals.var_ps0ld__blk949_dn2), (locals.var_vgpld__blk935_dn6 - locals.var_ps0ld__blk949_dn6), (locals.var_vgpld__blk935_dn7 - locals.var_ps0ld__blk949_dn7), (locals.var_vgpld__blk935_dn10 - locals.var_ps0ld__blk949_dn10), (locals.var_vgpld__blk935_dn11 - locals.var_ps0ld__blk949_dn11), (locals.var_vgpld__blk935_dn12 - locals.var_ps0ld__blk949_dn12), (locals.var_vgpld__blk935_dn17 - locals.var_ps0ld__blk949_dn17),)
    } else {
        (locals.var_t2__blk901, locals.var_t2__blk901_dn0, locals.var_t2__blk901_dn2, locals.var_t2__blk901_dn6, locals.var_t2__blk901_dn7, locals.var_t2__blk901_dn10, locals.var_t2__blk901_dn11, locals.var_t2__blk901_dn12, locals.var_t2__blk901_dn17,)
    }
};
        locals.var_t2__blk901 = assign29030_e40947;
        locals.var_t2__blk901_dn0 = assign29030_e40947_d_n0;
        locals.var_t2__blk901_dn2 = assign29030_e40947_d_n2;
        locals.var_t2__blk901_dn6 = assign29030_e40947_d_n6;
        locals.var_t2__blk901_dn7 = assign29030_e40947_d_n7;
        locals.var_t2__blk901_dn10 = assign29030_e40947_d_n10;
        locals.var_t2__blk901_dn11 = assign29030_e40947_d_n11;
        locals.var_t2__blk901_dn12 = assign29030_e40947_d_n12;
        locals.var_t2__blk901_dn17 = assign29030_e40947_d_n17;
        locals.var_t2__blk901_rv = 0.0;

        let (assign29040_e40960, assign29040_e40960_d_n0, assign29040_e40960_d_n2, assign29040_e40960_d_n6, assign29040_e40960_d_n7, assign29040_e40960_d_n10, assign29040_e40960_d_n11, assign29040_e40960_d_n12, assign29040_e40960_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29040_e40958: f64 = (locals.var_cox0__blk910 * locals.var_t2__blk901);
        (assign29040_e40958, (locals.var_cox0__blk910 * locals.var_t2__blk901_dn0), (locals.var_cox0__blk910 * locals.var_t2__blk901_dn2), (locals.var_cox0__blk910 * locals.var_t2__blk901_dn6), (locals.var_cox0__blk910 * locals.var_t2__blk901_dn7), (locals.var_cox0__blk910 * locals.var_t2__blk901_dn10), (locals.var_cox0__blk910 * locals.var_t2__blk901_dn11), (locals.var_cox0__blk910 * locals.var_t2__blk901_dn12), (locals.var_cox0__blk910 * locals.var_t2__blk901_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign29040_e40960;
        locals.var_qsuld_dn0 = assign29040_e40960_d_n0;
        locals.var_qsuld_dn2 = assign29040_e40960_d_n2;
        locals.var_qsuld_dn6 = assign29040_e40960_d_n6;
        locals.var_qsuld_dn7 = assign29040_e40960_d_n7;
        locals.var_qsuld_dn10 = assign29040_e40960_d_n10;
        locals.var_qsuld_dn11 = assign29040_e40960_d_n11;
        locals.var_qsuld_dn12 = assign29040_e40960_d_n12;
        locals.var_qsuld_dn17 = assign29040_e40960_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign29050_e40971, assign29050_e40971_d_n0, assign29050_e40971_d_n2, assign29050_e40971_d_n6, assign29050_e40971_d_n7, assign29050_e40971_d_n10, assign29050_e40971_d_n11, assign29050_e40971_d_n12, assign29050_e40971_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign29050_e40971;
        locals.var_qbuld_dn0 = assign29050_e40971_d_n0;
        locals.var_qbuld_dn2 = assign29050_e40971_d_n2;
        locals.var_qbuld_dn6 = assign29050_e40971_d_n6;
        locals.var_qbuld_dn7 = assign29050_e40971_d_n7;
        locals.var_qbuld_dn10 = assign29050_e40971_d_n10;
        locals.var_qbuld_dn11 = assign29050_e40971_d_n11;
        locals.var_qbuld_dn12 = assign29050_e40971_d_n12;
        locals.var_qbuld_dn17 = assign29050_e40971_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign29070_e40995, assign29070_e40995_d_n0, assign29070_e40995_d_n2, assign29070_e40995_d_n6, assign29070_e40995_d_n7, assign29070_e40995_d_n10, assign29070_e40995_d_n11, assign29070_e40995_d_n12, assign29070_e40995_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) {
        (3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi__blk947, locals.var_chi__blk947_dn0, locals.var_chi__blk947_dn2, locals.var_chi__blk947_dn6, locals.var_chi__blk947_dn7, locals.var_chi__blk947_dn10, locals.var_chi__blk947_dn11, locals.var_chi__blk947_dn12, locals.var_chi__blk947_dn17,)
    }
};
        locals.var_chi__blk947 = assign29070_e40995;
        locals.var_chi__blk947_dn0 = assign29070_e40995_d_n0;
        locals.var_chi__blk947_dn2 = assign29070_e40995_d_n2;
        locals.var_chi__blk947_dn6 = assign29070_e40995_d_n6;
        locals.var_chi__blk947_dn7 = assign29070_e40995_d_n7;
        locals.var_chi__blk947_dn10 = assign29070_e40995_d_n10;
        locals.var_chi__blk947_dn11 = assign29070_e40995_d_n11;
        locals.var_chi__blk947_dn12 = assign29070_e40995_d_n12;
        locals.var_chi__blk947_dn17 = assign29070_e40995_d_n17;
        locals.var_chi__blk947_rv = 0.0;

        let (assign29080_e41011, assign29080_e41011_d_n0, assign29080_e41011_d_n2, assign29080_e41011_d_n6, assign29080_e41011_d_n7, assign29080_e41011_d_n10, assign29080_e41011_d_n11, assign29080_e41011_d_n12, assign29080_e41011_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) {
        let assign29080_e41007: f64 = (locals.var_chi__blk947 / locals.var_beta);
        let assign29080_e41009: f64 = (assign29080_e41007 - locals.var_vxbgmtcl__blk925);
        (assign29080_e41009, ((locals.var_chi__blk947_dn0 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn0), ((locals.var_chi__blk947_dn2 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn2), ((locals.var_chi__blk947_dn6 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn6), ((locals.var_chi__blk947_dn7 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn7), ((((locals.var_chi__blk947_dn10 * locals.var_beta) - (locals.var_chi__blk947 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl__blk925_dn10), ((locals.var_chi__blk947_dn11 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn11), ((locals.var_chi__blk947_dn12 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn12), ((locals.var_chi__blk947_dn17 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn17),)
    } else {
        (locals.var_ps0_inia__blk950, locals.var_ps0_inia__blk950_dn0, locals.var_ps0_inia__blk950_dn2, locals.var_ps0_inia__blk950_dn6, locals.var_ps0_inia__blk950_dn7, locals.var_ps0_inia__blk950_dn10, locals.var_ps0_inia__blk950_dn11, locals.var_ps0_inia__blk950_dn12, locals.var_ps0_inia__blk950_dn17,)
    }
};
        locals.var_ps0_inia__blk950 = assign29080_e41011;
        locals.var_ps0_inia__blk950_dn0 = assign29080_e41011_d_n0;
        locals.var_ps0_inia__blk950_dn2 = assign29080_e41011_d_n2;
        locals.var_ps0_inia__blk950_dn6 = assign29080_e41011_d_n6;
        locals.var_ps0_inia__blk950_dn7 = assign29080_e41011_d_n7;
        locals.var_ps0_inia__blk950_dn10 = assign29080_e41011_d_n10;
        locals.var_ps0_inia__blk950_dn11 = assign29080_e41011_d_n11;
        locals.var_ps0_inia__blk950_dn12 = assign29080_e41011_d_n12;
        locals.var_ps0_inia__blk950_dn17 = assign29080_e41011_d_n17;
        locals.var_ps0_inia__blk950_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_105(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29090_e41025, assign29090_e41025_d_n0, assign29090_e41025_d_n2, assign29090_e41025_d_n6, assign29090_e41025_d_n7, assign29090_e41025_d_n10, assign29090_e41025_d_n11, assign29090_e41025_d_n12, assign29090_e41025_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) {
        let assign29090_e41022: f64 = (-locals.var_chi__blk947);
        let assign29090_e41023: f64 = (assign29090_e41022).exp();
        (assign29090_e41023, (assign29090_e41023 * (-locals.var_chi__blk947_dn0)), (assign29090_e41023 * (-locals.var_chi__blk947_dn2)), (assign29090_e41023 * (-locals.var_chi__blk947_dn6)), (assign29090_e41023 * (-locals.var_chi__blk947_dn7)), (assign29090_e41023 * (-locals.var_chi__blk947_dn10)), (assign29090_e41023 * (-locals.var_chi__blk947_dn11)), (assign29090_e41023 * (-locals.var_chi__blk947_dn12)), (assign29090_e41023 * (-locals.var_chi__blk947_dn17)),)
    } else {
        (locals.var_ty__blk909, locals.var_ty__blk909_dn0, locals.var_ty__blk909_dn2, locals.var_ty__blk909_dn6, locals.var_ty__blk909_dn7, locals.var_ty__blk909_dn10, locals.var_ty__blk909_dn11, locals.var_ty__blk909_dn12, locals.var_ty__blk909_dn17,)
    }
};
        locals.var_ty__blk909 = assign29090_e41025;
        locals.var_ty__blk909_dn0 = assign29090_e41025_d_n0;
        locals.var_ty__blk909_dn2 = assign29090_e41025_d_n2;
        locals.var_ty__blk909_dn6 = assign29090_e41025_d_n6;
        locals.var_ty__blk909_dn7 = assign29090_e41025_d_n7;
        locals.var_ty__blk909_dn10 = assign29090_e41025_d_n10;
        locals.var_ty__blk909_dn11 = assign29090_e41025_d_n11;
        locals.var_ty__blk909_dn12 = assign29090_e41025_d_n12;
        locals.var_ty__blk909_dn17 = assign29090_e41025_d_n17;
        locals.var_ty__blk909_rv = 0.0;

        let (assign29100_e41053, assign29100_e41053_d_n0, assign29100_e41053_d_n2, assign29100_e41053_d_n6, assign29100_e41053_d_n7, assign29100_e41053_d_n10, assign29100_e41053_d_n11, assign29100_e41053_d_n12, assign29100_e41053_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) {
        let assign29100_e41040: f64 = (locals.var_vgpld__blk935 + locals.var_vxbgmtcl__blk925);
        let assign29100_e41041: f64 = (locals.var_beta * assign29100_e41040);
        let assign29100_e41043: f64 = (assign29100_e41041 - 1.0);
        let assign29100_e41045: f64 = (assign29100_e41043 + locals.var_ty__blk909);
        let assign29100_e41046: f64 = (4.0 * assign29100_e41045);
        let assign29100_e41049: f64 = (locals.var_fac1p2__blk934 * locals.var_beta2);
        let assign29100_e41050: f64 = (assign29100_e41046 / assign29100_e41049);
        let assign29100_e41051: f64 = (1.0 + assign29100_e41050);
        (assign29100_e41051, ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn0 + locals.var_vxbgmtcl__blk925_dn0)) + locals.var_ty__blk909_dn0)) * assign29100_e41049) - (assign29100_e41046 * (locals.var_fac1p2__blk934_dn0 * locals.var_beta2))) / (assign29100_e41049 * assign29100_e41049)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn2 + locals.var_vxbgmtcl__blk925_dn2)) + locals.var_ty__blk909_dn2)) * assign29100_e41049) - (assign29100_e41046 * (locals.var_fac1p2__blk934_dn2 * locals.var_beta2))) / (assign29100_e41049 * assign29100_e41049)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn6 + locals.var_vxbgmtcl__blk925_dn6)) + locals.var_ty__blk909_dn6)) * assign29100_e41049) - (assign29100_e41046 * (locals.var_fac1p2__blk934_dn6 * locals.var_beta2))) / (assign29100_e41049 * assign29100_e41049)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn7 + locals.var_vxbgmtcl__blk925_dn7)) + locals.var_ty__blk909_dn7)) * assign29100_e41049) - (assign29100_e41046 * (locals.var_fac1p2__blk934_dn7 * locals.var_beta2))) / (assign29100_e41049 * assign29100_e41049)), ((((4.0 * (((locals.var_beta_dn10 * assign29100_e41040) + (locals.var_beta * (locals.var_vgpld__blk935_dn10 + locals.var_vxbgmtcl__blk925_dn10))) + locals.var_ty__blk909_dn10)) * assign29100_e41049) - (assign29100_e41046 * ((locals.var_fac1p2__blk934_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk934 * locals.var_beta2_dn10)))) / (assign29100_e41049 * assign29100_e41049)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn11 + locals.var_vxbgmtcl__blk925_dn11)) + locals.var_ty__blk909_dn11)) * assign29100_e41049) - (assign29100_e41046 * (locals.var_fac1p2__blk934_dn11 * locals.var_beta2))) / (assign29100_e41049 * assign29100_e41049)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn12 + locals.var_vxbgmtcl__blk925_dn12)) + locals.var_ty__blk909_dn12)) * assign29100_e41049) - (assign29100_e41046 * (locals.var_fac1p2__blk934_dn12 * locals.var_beta2))) / (assign29100_e41049 * assign29100_e41049)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn17 + locals.var_vxbgmtcl__blk925_dn17)) + locals.var_ty__blk909_dn17)) * assign29100_e41049) - (assign29100_e41046 * (locals.var_fac1p2__blk934_dn17 * locals.var_beta2))) / (assign29100_e41049 * assign29100_e41049)),)
    } else {
        (locals.var_tx__blk908, locals.var_tx__blk908_dn0, locals.var_tx__blk908_dn2, locals.var_tx__blk908_dn6, locals.var_tx__blk908_dn7, locals.var_tx__blk908_dn10, locals.var_tx__blk908_dn11, locals.var_tx__blk908_dn12, locals.var_tx__blk908_dn17,)
    }
};
        locals.var_tx__blk908 = assign29100_e41053;
        locals.var_tx__blk908_dn0 = assign29100_e41053_d_n0;
        locals.var_tx__blk908_dn2 = assign29100_e41053_d_n2;
        locals.var_tx__blk908_dn6 = assign29100_e41053_d_n6;
        locals.var_tx__blk908_dn7 = assign29100_e41053_d_n7;
        locals.var_tx__blk908_dn10 = assign29100_e41053_d_n10;
        locals.var_tx__blk908_dn11 = assign29100_e41053_d_n11;
        locals.var_tx__blk908_dn12 = assign29100_e41053_d_n12;
        locals.var_tx__blk908_dn17 = assign29100_e41053_d_n17;
        locals.var_tx__blk908_rv = 0.0;

        let assign29110_e41057: f64 = (10.0 * 2.220446049250313e-16);
        let assign29110_e41058: f64 = if locals.var_tx__blk908 < assign29110_e41057 { 1.0 } else { 0.0 };
        locals.var_guard988 = assign29110_e41058;
        locals.var_guard988_rv = 0.0;

        let (assign29120_e41074, assign29120_e41074_d_n0, assign29120_e41074_d_n2, assign29120_e41074_d_n6, assign29120_e41074_d_n7, assign29120_e41074_d_n10, assign29120_e41074_d_n11, assign29120_e41074_d_n12, assign29120_e41074_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard988 != 0.0)) {
        let assign29120_e41072: f64 = (10.0 * 2.220446049250313e-16);
        (assign29120_e41072, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk908, locals.var_tx__blk908_dn0, locals.var_tx__blk908_dn2, locals.var_tx__blk908_dn6, locals.var_tx__blk908_dn7, locals.var_tx__blk908_dn10, locals.var_tx__blk908_dn11, locals.var_tx__blk908_dn12, locals.var_tx__blk908_dn17,)
    }
};
        locals.var_tx__blk908 = assign29120_e41074;
        locals.var_tx__blk908_dn0 = assign29120_e41074_d_n0;
        locals.var_tx__blk908_dn2 = assign29120_e41074_d_n2;
        locals.var_tx__blk908_dn6 = assign29120_e41074_d_n6;
        locals.var_tx__blk908_dn7 = assign29120_e41074_d_n7;
        locals.var_tx__blk908_dn10 = assign29120_e41074_d_n10;
        locals.var_tx__blk908_dn11 = assign29120_e41074_d_n11;
        locals.var_tx__blk908_dn12 = assign29120_e41074_d_n12;
        locals.var_tx__blk908_dn17 = assign29120_e41074_d_n17;
        locals.var_tx__blk908_rv = 0.0;

        let (assign29130_e41097, assign29130_e41097_d_n0, assign29130_e41097_d_n2, assign29130_e41097_d_n6, assign29130_e41097_d_n7, assign29130_e41097_d_n10, assign29130_e41097_d_n11, assign29130_e41097_d_n12, assign29130_e41097_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) {
        let assign29130_e41087: f64 = (locals.var_fac1p2__blk934 * locals.var_beta);
        let assign29130_e41089: f64 = (assign29130_e41087 / 2.0);
        let assign29130_e41092: f64 = (locals.var_tx__blk908).sqrt();
        let assign29130_e41093: f64 = (1.0 - assign29130_e41092);
        let assign29130_e41094: f64 = (assign29130_e41089 * assign29130_e41093);
        let assign29130_e41095: f64 = (locals.var_vgpld__blk935 + assign29130_e41094);
        (assign29130_e41095, (locals.var_vgpld__blk935_dn0 + ((((locals.var_fac1p2__blk934_dn0 * locals.var_beta) / 2.0) * assign29130_e41093) + (assign29130_e41089 * (-(locals.var_tx__blk908_dn0 / (2.0 * assign29130_e41092)))))), (locals.var_vgpld__blk935_dn2 + ((((locals.var_fac1p2__blk934_dn2 * locals.var_beta) / 2.0) * assign29130_e41093) + (assign29130_e41089 * (-(locals.var_tx__blk908_dn2 / (2.0 * assign29130_e41092)))))), (locals.var_vgpld__blk935_dn6 + ((((locals.var_fac1p2__blk934_dn6 * locals.var_beta) / 2.0) * assign29130_e41093) + (assign29130_e41089 * (-(locals.var_tx__blk908_dn6 / (2.0 * assign29130_e41092)))))), (locals.var_vgpld__blk935_dn7 + ((((locals.var_fac1p2__blk934_dn7 * locals.var_beta) / 2.0) * assign29130_e41093) + (assign29130_e41089 * (-(locals.var_tx__blk908_dn7 / (2.0 * assign29130_e41092)))))), (locals.var_vgpld__blk935_dn10 + (((((locals.var_fac1p2__blk934_dn10 * locals.var_beta) + (locals.var_fac1p2__blk934 * locals.var_beta_dn10)) / 2.0) * assign29130_e41093) + (assign29130_e41089 * (-(locals.var_tx__blk908_dn10 / (2.0 * assign29130_e41092)))))), (locals.var_vgpld__blk935_dn11 + ((((locals.var_fac1p2__blk934_dn11 * locals.var_beta) / 2.0) * assign29130_e41093) + (assign29130_e41089 * (-(locals.var_tx__blk908_dn11 / (2.0 * assign29130_e41092)))))), (locals.var_vgpld__blk935_dn12 + ((((locals.var_fac1p2__blk934_dn12 * locals.var_beta) / 2.0) * assign29130_e41093) + (assign29130_e41089 * (-(locals.var_tx__blk908_dn12 / (2.0 * assign29130_e41092)))))), (locals.var_vgpld__blk935_dn17 + ((((locals.var_fac1p2__blk934_dn17 * locals.var_beta) / 2.0) * assign29130_e41093) + (assign29130_e41089 * (-(locals.var_tx__blk908_dn17 / (2.0 * assign29130_e41092)))))),)
    } else {
        (locals.var_ps0_inia__blk950, locals.var_ps0_inia__blk950_dn0, locals.var_ps0_inia__blk950_dn2, locals.var_ps0_inia__blk950_dn6, locals.var_ps0_inia__blk950_dn7, locals.var_ps0_inia__blk950_dn10, locals.var_ps0_inia__blk950_dn11, locals.var_ps0_inia__blk950_dn12, locals.var_ps0_inia__blk950_dn17,)
    }
};
        locals.var_ps0_inia__blk950 = assign29130_e41097;
        locals.var_ps0_inia__blk950_dn0 = assign29130_e41097_d_n0;
        locals.var_ps0_inia__blk950_dn2 = assign29130_e41097_d_n2;
        locals.var_ps0_inia__blk950_dn6 = assign29130_e41097_d_n6;
        locals.var_ps0_inia__blk950_dn7 = assign29130_e41097_d_n7;
        locals.var_ps0_inia__blk950_dn10 = assign29130_e41097_d_n10;
        locals.var_ps0_inia__blk950_dn11 = assign29130_e41097_d_n11;
        locals.var_ps0_inia__blk950_dn12 = assign29130_e41097_d_n12;
        locals.var_ps0_inia__blk950_dn17 = assign29130_e41097_d_n17;
        locals.var_ps0_inia__blk950_rv = 0.0;

        let (assign29140_e41113, assign29140_e41113_d_n0, assign29140_e41113_d_n2, assign29140_e41113_d_n6, assign29140_e41113_d_n7, assign29140_e41113_d_n10, assign29140_e41113_d_n11, assign29140_e41113_d_n12, assign29140_e41113_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) {
        let assign29140_e41110: f64 = (locals.var_ps0_inia__blk950 + locals.var_vxbgmtcl__blk925);
        let assign29140_e41111: f64 = (locals.var_beta * assign29140_e41110);
        (assign29140_e41111, (locals.var_beta * (locals.var_ps0_inia__blk950_dn0 + locals.var_vxbgmtcl__blk925_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn2 + locals.var_vxbgmtcl__blk925_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn6 + locals.var_vxbgmtcl__blk925_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn7 + locals.var_vxbgmtcl__blk925_dn7)), ((locals.var_beta_dn10 * assign29140_e41110) + (locals.var_beta * (locals.var_ps0_inia__blk950_dn10 + locals.var_vxbgmtcl__blk925_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk950_dn11 + locals.var_vxbgmtcl__blk925_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn12 + locals.var_vxbgmtcl__blk925_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn17 + locals.var_vxbgmtcl__blk925_dn17)),)
    } else {
        (locals.var_chi__blk947, locals.var_chi__blk947_dn0, locals.var_chi__blk947_dn2, locals.var_chi__blk947_dn6, locals.var_chi__blk947_dn7, locals.var_chi__blk947_dn10, locals.var_chi__blk947_dn11, locals.var_chi__blk947_dn12, locals.var_chi__blk947_dn17,)
    }
};
        locals.var_chi__blk947 = assign29140_e41113;
        locals.var_chi__blk947_dn0 = assign29140_e41113_d_n0;
        locals.var_chi__blk947_dn2 = assign29140_e41113_d_n2;
        locals.var_chi__blk947_dn6 = assign29140_e41113_d_n6;
        locals.var_chi__blk947_dn7 = assign29140_e41113_d_n7;
        locals.var_chi__blk947_dn10 = assign29140_e41113_d_n10;
        locals.var_chi__blk947_dn11 = assign29140_e41113_d_n11;
        locals.var_chi__blk947_dn12 = assign29140_e41113_d_n12;
        locals.var_chi__blk947_dn17 = assign29140_e41113_d_n17;
        locals.var_chi__blk947_rv = 0.0;

        let (assign29150_e41127, assign29150_e41127_d_n0, assign29150_e41127_d_n2, assign29150_e41127_d_n6, assign29150_e41127_d_n7, assign29150_e41127_d_n10, assign29150_e41127_d_n11, assign29150_e41127_d_n12, assign29150_e41127_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) {
        let assign29150_e41124: f64 = (-locals.var_chi__blk947);
        let assign29150_e41125: f64 = (assign29150_e41124).exp();
        (assign29150_e41125, (assign29150_e41125 * (-locals.var_chi__blk947_dn0)), (assign29150_e41125 * (-locals.var_chi__blk947_dn2)), (assign29150_e41125 * (-locals.var_chi__blk947_dn6)), (assign29150_e41125 * (-locals.var_chi__blk947_dn7)), (assign29150_e41125 * (-locals.var_chi__blk947_dn10)), (assign29150_e41125 * (-locals.var_chi__blk947_dn11)), (assign29150_e41125 * (-locals.var_chi__blk947_dn12)), (assign29150_e41125 * (-locals.var_chi__blk947_dn17)),)
    } else {
        (locals.var_ty__blk909, locals.var_ty__blk909_dn0, locals.var_ty__blk909_dn2, locals.var_ty__blk909_dn6, locals.var_ty__blk909_dn7, locals.var_ty__blk909_dn10, locals.var_ty__blk909_dn11, locals.var_ty__blk909_dn12, locals.var_ty__blk909_dn17,)
    }
};
        locals.var_ty__blk909 = assign29150_e41127;
        locals.var_ty__blk909_dn0 = assign29150_e41127_d_n0;
        locals.var_ty__blk909_dn2 = assign29150_e41127_d_n2;
        locals.var_ty__blk909_dn6 = assign29150_e41127_d_n6;
        locals.var_ty__blk909_dn7 = assign29150_e41127_d_n7;
        locals.var_ty__blk909_dn10 = assign29150_e41127_d_n10;
        locals.var_ty__blk909_dn11 = assign29150_e41127_d_n11;
        locals.var_ty__blk909_dn12 = assign29150_e41127_d_n12;
        locals.var_ty__blk909_dn17 = assign29150_e41127_d_n17;
        locals.var_ty__blk909_rv = 0.0;

        let (assign29160_e41155, assign29160_e41155_d_n0, assign29160_e41155_d_n2, assign29160_e41155_d_n6, assign29160_e41155_d_n7, assign29160_e41155_d_n10, assign29160_e41155_d_n11, assign29160_e41155_d_n12, assign29160_e41155_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) {
        let assign29160_e41142: f64 = (locals.var_vgpld__blk935 + locals.var_vxbgmtcl__blk925);
        let assign29160_e41143: f64 = (locals.var_beta * assign29160_e41142);
        let assign29160_e41145: f64 = (assign29160_e41143 - 1.0);
        let assign29160_e41147: f64 = (assign29160_e41145 + locals.var_ty__blk909);
        let assign29160_e41148: f64 = (4.0 * assign29160_e41147);
        let assign29160_e41151: f64 = (locals.var_fac1p2__blk934 * locals.var_beta2);
        let assign29160_e41152: f64 = (assign29160_e41148 / assign29160_e41151);
        let assign29160_e41153: f64 = (1.0 + assign29160_e41152);
        (assign29160_e41153, ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn0 + locals.var_vxbgmtcl__blk925_dn0)) + locals.var_ty__blk909_dn0)) * assign29160_e41151) - (assign29160_e41148 * (locals.var_fac1p2__blk934_dn0 * locals.var_beta2))) / (assign29160_e41151 * assign29160_e41151)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn2 + locals.var_vxbgmtcl__blk925_dn2)) + locals.var_ty__blk909_dn2)) * assign29160_e41151) - (assign29160_e41148 * (locals.var_fac1p2__blk934_dn2 * locals.var_beta2))) / (assign29160_e41151 * assign29160_e41151)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn6 + locals.var_vxbgmtcl__blk925_dn6)) + locals.var_ty__blk909_dn6)) * assign29160_e41151) - (assign29160_e41148 * (locals.var_fac1p2__blk934_dn6 * locals.var_beta2))) / (assign29160_e41151 * assign29160_e41151)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn7 + locals.var_vxbgmtcl__blk925_dn7)) + locals.var_ty__blk909_dn7)) * assign29160_e41151) - (assign29160_e41148 * (locals.var_fac1p2__blk934_dn7 * locals.var_beta2))) / (assign29160_e41151 * assign29160_e41151)), ((((4.0 * (((locals.var_beta_dn10 * assign29160_e41142) + (locals.var_beta * (locals.var_vgpld__blk935_dn10 + locals.var_vxbgmtcl__blk925_dn10))) + locals.var_ty__blk909_dn10)) * assign29160_e41151) - (assign29160_e41148 * ((locals.var_fac1p2__blk934_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk934 * locals.var_beta2_dn10)))) / (assign29160_e41151 * assign29160_e41151)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn11 + locals.var_vxbgmtcl__blk925_dn11)) + locals.var_ty__blk909_dn11)) * assign29160_e41151) - (assign29160_e41148 * (locals.var_fac1p2__blk934_dn11 * locals.var_beta2))) / (assign29160_e41151 * assign29160_e41151)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn12 + locals.var_vxbgmtcl__blk925_dn12)) + locals.var_ty__blk909_dn12)) * assign29160_e41151) - (assign29160_e41148 * (locals.var_fac1p2__blk934_dn12 * locals.var_beta2))) / (assign29160_e41151 * assign29160_e41151)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk935_dn17 + locals.var_vxbgmtcl__blk925_dn17)) + locals.var_ty__blk909_dn17)) * assign29160_e41151) - (assign29160_e41148 * (locals.var_fac1p2__blk934_dn17 * locals.var_beta2))) / (assign29160_e41151 * assign29160_e41151)),)
    } else {
        (locals.var_tx__blk908, locals.var_tx__blk908_dn0, locals.var_tx__blk908_dn2, locals.var_tx__blk908_dn6, locals.var_tx__blk908_dn7, locals.var_tx__blk908_dn10, locals.var_tx__blk908_dn11, locals.var_tx__blk908_dn12, locals.var_tx__blk908_dn17,)
    }
};
        locals.var_tx__blk908 = assign29160_e41155;
        locals.var_tx__blk908_dn0 = assign29160_e41155_d_n0;
        locals.var_tx__blk908_dn2 = assign29160_e41155_d_n2;
        locals.var_tx__blk908_dn6 = assign29160_e41155_d_n6;
        locals.var_tx__blk908_dn7 = assign29160_e41155_d_n7;
        locals.var_tx__blk908_dn10 = assign29160_e41155_d_n10;
        locals.var_tx__blk908_dn11 = assign29160_e41155_d_n11;
        locals.var_tx__blk908_dn12 = assign29160_e41155_d_n12;
        locals.var_tx__blk908_dn17 = assign29160_e41155_d_n17;
        locals.var_tx__blk908_rv = 0.0;

        let assign29170_e41159: f64 = (10.0 * 2.220446049250313e-16);
        let assign29170_e41160: f64 = if locals.var_tx__blk908 < assign29170_e41159 { 1.0 } else { 0.0 };
        locals.var_guard989 = assign29170_e41160;
        locals.var_guard989_rv = 0.0;

        let (assign29180_e41176, assign29180_e41176_d_n0, assign29180_e41176_d_n2, assign29180_e41176_d_n6, assign29180_e41176_d_n7, assign29180_e41176_d_n10, assign29180_e41176_d_n11, assign29180_e41176_d_n12, assign29180_e41176_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29180_e41174: f64 = (10.0 * 2.220446049250313e-16);
        (assign29180_e41174, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk908, locals.var_tx__blk908_dn0, locals.var_tx__blk908_dn2, locals.var_tx__blk908_dn6, locals.var_tx__blk908_dn7, locals.var_tx__blk908_dn10, locals.var_tx__blk908_dn11, locals.var_tx__blk908_dn12, locals.var_tx__blk908_dn17,)
    }
};
        locals.var_tx__blk908 = assign29180_e41176;
        locals.var_tx__blk908_dn0 = assign29180_e41176_d_n0;
        locals.var_tx__blk908_dn2 = assign29180_e41176_d_n2;
        locals.var_tx__blk908_dn6 = assign29180_e41176_d_n6;
        locals.var_tx__blk908_dn7 = assign29180_e41176_d_n7;
        locals.var_tx__blk908_dn10 = assign29180_e41176_d_n10;
        locals.var_tx__blk908_dn11 = assign29180_e41176_d_n11;
        locals.var_tx__blk908_dn12 = assign29180_e41176_d_n12;
        locals.var_tx__blk908_dn17 = assign29180_e41176_d_n17;
        locals.var_tx__blk908_rv = 0.0;

        let (assign29190_e41199, assign29190_e41199_d_n0, assign29190_e41199_d_n2, assign29190_e41199_d_n6, assign29190_e41199_d_n7, assign29190_e41199_d_n10, assign29190_e41199_d_n11, assign29190_e41199_d_n12, assign29190_e41199_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) {
        let assign29190_e41189: f64 = (locals.var_fac1p2__blk934 * locals.var_beta);
        let assign29190_e41191: f64 = (assign29190_e41189 / 2.0);
        let assign29190_e41194: f64 = (locals.var_tx__blk908).sqrt();
        let assign29190_e41195: f64 = (1.0 - assign29190_e41194);
        let assign29190_e41196: f64 = (assign29190_e41191 * assign29190_e41195);
        let assign29190_e41197: f64 = (locals.var_vgpld__blk935 + assign29190_e41196);
        (assign29190_e41197, (locals.var_vgpld__blk935_dn0 + ((((locals.var_fac1p2__blk934_dn0 * locals.var_beta) / 2.0) * assign29190_e41195) + (assign29190_e41191 * (-(locals.var_tx__blk908_dn0 / (2.0 * assign29190_e41194)))))), (locals.var_vgpld__blk935_dn2 + ((((locals.var_fac1p2__blk934_dn2 * locals.var_beta) / 2.0) * assign29190_e41195) + (assign29190_e41191 * (-(locals.var_tx__blk908_dn2 / (2.0 * assign29190_e41194)))))), (locals.var_vgpld__blk935_dn6 + ((((locals.var_fac1p2__blk934_dn6 * locals.var_beta) / 2.0) * assign29190_e41195) + (assign29190_e41191 * (-(locals.var_tx__blk908_dn6 / (2.0 * assign29190_e41194)))))), (locals.var_vgpld__blk935_dn7 + ((((locals.var_fac1p2__blk934_dn7 * locals.var_beta) / 2.0) * assign29190_e41195) + (assign29190_e41191 * (-(locals.var_tx__blk908_dn7 / (2.0 * assign29190_e41194)))))), (locals.var_vgpld__blk935_dn10 + (((((locals.var_fac1p2__blk934_dn10 * locals.var_beta) + (locals.var_fac1p2__blk934 * locals.var_beta_dn10)) / 2.0) * assign29190_e41195) + (assign29190_e41191 * (-(locals.var_tx__blk908_dn10 / (2.0 * assign29190_e41194)))))), (locals.var_vgpld__blk935_dn11 + ((((locals.var_fac1p2__blk934_dn11 * locals.var_beta) / 2.0) * assign29190_e41195) + (assign29190_e41191 * (-(locals.var_tx__blk908_dn11 / (2.0 * assign29190_e41194)))))), (locals.var_vgpld__blk935_dn12 + ((((locals.var_fac1p2__blk934_dn12 * locals.var_beta) / 2.0) * assign29190_e41195) + (assign29190_e41191 * (-(locals.var_tx__blk908_dn12 / (2.0 * assign29190_e41194)))))), (locals.var_vgpld__blk935_dn17 + ((((locals.var_fac1p2__blk934_dn17 * locals.var_beta) / 2.0) * assign29190_e41195) + (assign29190_e41191 * (-(locals.var_tx__blk908_dn17 / (2.0 * assign29190_e41194)))))),)
    } else {
        (locals.var_ps0_inia__blk950, locals.var_ps0_inia__blk950_dn0, locals.var_ps0_inia__blk950_dn2, locals.var_ps0_inia__blk950_dn6, locals.var_ps0_inia__blk950_dn7, locals.var_ps0_inia__blk950_dn10, locals.var_ps0_inia__blk950_dn11, locals.var_ps0_inia__blk950_dn12, locals.var_ps0_inia__blk950_dn17,)
    }
};
        locals.var_ps0_inia__blk950 = assign29190_e41199;
        locals.var_ps0_inia__blk950_dn0 = assign29190_e41199_d_n0;
        locals.var_ps0_inia__blk950_dn2 = assign29190_e41199_d_n2;
        locals.var_ps0_inia__blk950_dn6 = assign29190_e41199_d_n6;
        locals.var_ps0_inia__blk950_dn7 = assign29190_e41199_d_n7;
        locals.var_ps0_inia__blk950_dn10 = assign29190_e41199_d_n10;
        locals.var_ps0_inia__blk950_dn11 = assign29190_e41199_d_n11;
        locals.var_ps0_inia__blk950_dn12 = assign29190_e41199_d_n12;
        locals.var_ps0_inia__blk950_dn17 = assign29190_e41199_d_n17;
        locals.var_ps0_inia__blk950_rv = 0.0;

        let (assign29200_e41215, assign29200_e41215_d_n0, assign29200_e41215_d_n2, assign29200_e41215_d_n6, assign29200_e41215_d_n7, assign29200_e41215_d_n10, assign29200_e41215_d_n11, assign29200_e41215_d_n12, assign29200_e41215_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) {
        let assign29200_e41212: f64 = (locals.var_ps0_inia__blk950 + locals.var_vxbgmtcl__blk925);
        let assign29200_e41213: f64 = (locals.var_beta * assign29200_e41212);
        (assign29200_e41213, (locals.var_beta * (locals.var_ps0_inia__blk950_dn0 + locals.var_vxbgmtcl__blk925_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn2 + locals.var_vxbgmtcl__blk925_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn6 + locals.var_vxbgmtcl__blk925_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn7 + locals.var_vxbgmtcl__blk925_dn7)), ((locals.var_beta_dn10 * assign29200_e41212) + (locals.var_beta * (locals.var_ps0_inia__blk950_dn10 + locals.var_vxbgmtcl__blk925_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk950_dn11 + locals.var_vxbgmtcl__blk925_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn12 + locals.var_vxbgmtcl__blk925_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn17 + locals.var_vxbgmtcl__blk925_dn17)),)
    } else {
        (locals.var_chi__blk947, locals.var_chi__blk947_dn0, locals.var_chi__blk947_dn2, locals.var_chi__blk947_dn6, locals.var_chi__blk947_dn7, locals.var_chi__blk947_dn10, locals.var_chi__blk947_dn11, locals.var_chi__blk947_dn12, locals.var_chi__blk947_dn17,)
    }
};
        locals.var_chi__blk947 = assign29200_e41215;
        locals.var_chi__blk947_dn0 = assign29200_e41215_d_n0;
        locals.var_chi__blk947_dn2 = assign29200_e41215_d_n2;
        locals.var_chi__blk947_dn6 = assign29200_e41215_d_n6;
        locals.var_chi__blk947_dn7 = assign29200_e41215_d_n7;
        locals.var_chi__blk947_dn10 = assign29200_e41215_d_n10;
        locals.var_chi__blk947_dn11 = assign29200_e41215_d_n11;
        locals.var_chi__blk947_dn12 = assign29200_e41215_d_n12;
        locals.var_chi__blk947_dn17 = assign29200_e41215_d_n17;
        locals.var_chi__blk947_rv = 0.0;

        let assign29210_e41218: f64 = if locals.var_chi__blk947 < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard990 = assign29210_e41218;
        locals.var_guard990_rv = 0.0;

        let (assign29230_e41263,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29230_e41247: f64 = (9.0 * 1.414213562373095);
        let assign29230_e41248: f64 = (1.0 / assign29230_e41247);
        let assign29230_e41252: f64 = (7.0 * 0.049787068367863944);
        let assign29230_e41253: f64 = (5.0 + assign29230_e41252);
        let assign29230_e41257: f64 = (2.0 + 0.049787068367863944);
        let assign29230_e41258: f64 = (assign29230_e41257).sqrt();
        let assign29230_e41259: f64 = (54.0 * assign29230_e41258);
        let assign29230_e41260: f64 = (assign29230_e41253 / assign29230_e41259);
        let assign29230_e41261: f64 = (assign29230_e41248 - assign29230_e41260);
        (assign29230_e41261,)
    } else {
        (locals.var_ta__blk951,)
    }
};
        locals.var_ta__blk951 = assign29230_e41263;
        locals.var_ta__blk951_rv = 0.0;

        let (assign29240_e41290,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29240_e41277: f64 = (1.0 + 0.049787068367863944);
        let assign29240_e41281: f64 = (2.0 + 0.049787068367863944);
        let assign29240_e41282: f64 = (assign29240_e41281).sqrt();
        let assign29240_e41283: f64 = (2.0 * assign29240_e41282);
        let assign29240_e41284: f64 = (assign29240_e41277 / assign29240_e41283);
        let assign29240_e41287: f64 = (1.414213562373095 / 3.0);
        let assign29240_e41288: f64 = (assign29240_e41284 - assign29240_e41287);
        (assign29240_e41288,)
    } else {
        (locals.var_tb__blk952,)
    }
};
        locals.var_tb__blk952 = assign29240_e41290;
        locals.var_tb__blk952_rv = 0.0;

        let (assign29250_e41312, assign29250_e41312_d_n0, assign29250_e41312_d_n2, assign29250_e41312_d_n6, assign29250_e41312_d_n7, assign29250_e41312_d_n10, assign29250_e41312_d_n11, assign29250_e41312_d_n12, assign29250_e41312_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29250_e41304: f64 = (1.0 / 1.414213562373095);
        let assign29250_e41308: f64 = (locals.var_beta * locals.var_fac1__blk933);
        let assign29250_e41309: f64 = (1.0 / assign29250_e41308);
        let assign29250_e41310: f64 = (assign29250_e41304 + assign29250_e41309);
        (assign29250_e41310, (-((locals.var_beta * locals.var_fac1__blk933_dn0) / (assign29250_e41308 * assign29250_e41308))), (-((locals.var_beta * locals.var_fac1__blk933_dn2) / (assign29250_e41308 * assign29250_e41308))), (-((locals.var_beta * locals.var_fac1__blk933_dn6) / (assign29250_e41308 * assign29250_e41308))), (-((locals.var_beta * locals.var_fac1__blk933_dn7) / (assign29250_e41308 * assign29250_e41308))), (-(((locals.var_beta_dn10 * locals.var_fac1__blk933) + (locals.var_beta * locals.var_fac1__blk933_dn10)) / (assign29250_e41308 * assign29250_e41308))), (-((locals.var_beta * locals.var_fac1__blk933_dn11) / (assign29250_e41308 * assign29250_e41308))), (-((locals.var_beta * locals.var_fac1__blk933_dn12) / (assign29250_e41308 * assign29250_e41308))), (-((locals.var_beta * locals.var_fac1__blk933_dn17) / (assign29250_e41308 * assign29250_e41308))),)
    } else {
        (locals.var_tc__blk953, locals.var_tc__blk953_dn0, locals.var_tc__blk953_dn2, locals.var_tc__blk953_dn6, locals.var_tc__blk953_dn7, locals.var_tc__blk953_dn10, locals.var_tc__blk953_dn11, locals.var_tc__blk953_dn12, locals.var_tc__blk953_dn17,)
    }
};
        locals.var_tc__blk953 = assign29250_e41312;
        locals.var_tc__blk953_dn0 = assign29250_e41312_d_n0;
        locals.var_tc__blk953_dn2 = assign29250_e41312_d_n2;
        locals.var_tc__blk953_dn6 = assign29250_e41312_d_n6;
        locals.var_tc__blk953_dn7 = assign29250_e41312_d_n7;
        locals.var_tc__blk953_dn10 = assign29250_e41312_d_n10;
        locals.var_tc__blk953_dn11 = assign29250_e41312_d_n11;
        locals.var_tc__blk953_dn12 = assign29250_e41312_d_n12;
        locals.var_tc__blk953_dn17 = assign29250_e41312_d_n17;
        locals.var_tc__blk953_rv = 0.0;

        let (assign29260_e41331, assign29260_e41331_d_n0, assign29260_e41331_d_n2, assign29260_e41331_d_n6, assign29260_e41331_d_n7, assign29260_e41331_d_n10, assign29260_e41331_d_n11, assign29260_e41331_d_n12, assign29260_e41331_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29260_e41326: f64 = (locals.var_vgpld__blk935 + locals.var_vxbgmtcl__blk925);
        let assign29260_e41327: f64 = (-assign29260_e41326);
        let assign29260_e41329: f64 = (assign29260_e41327 / locals.var_fac1__blk933);
        (assign29260_e41329, ((((-(locals.var_vgpld__blk935_dn0 + locals.var_vxbgmtcl__blk925_dn0)) * locals.var_fac1__blk933) - (assign29260_e41327 * locals.var_fac1__blk933_dn0)) / (locals.var_fac1__blk933 * locals.var_fac1__blk933)), ((((-(locals.var_vgpld__blk935_dn2 + locals.var_vxbgmtcl__blk925_dn2)) * locals.var_fac1__blk933) - (assign29260_e41327 * locals.var_fac1__blk933_dn2)) / (locals.var_fac1__blk933 * locals.var_fac1__blk933)), ((((-(locals.var_vgpld__blk935_dn6 + locals.var_vxbgmtcl__blk925_dn6)) * locals.var_fac1__blk933) - (assign29260_e41327 * locals.var_fac1__blk933_dn6)) / (locals.var_fac1__blk933 * locals.var_fac1__blk933)), ((((-(locals.var_vgpld__blk935_dn7 + locals.var_vxbgmtcl__blk925_dn7)) * locals.var_fac1__blk933) - (assign29260_e41327 * locals.var_fac1__blk933_dn7)) / (locals.var_fac1__blk933 * locals.var_fac1__blk933)), ((((-(locals.var_vgpld__blk935_dn10 + locals.var_vxbgmtcl__blk925_dn10)) * locals.var_fac1__blk933) - (assign29260_e41327 * locals.var_fac1__blk933_dn10)) / (locals.var_fac1__blk933 * locals.var_fac1__blk933)), ((((-(locals.var_vgpld__blk935_dn11 + locals.var_vxbgmtcl__blk925_dn11)) * locals.var_fac1__blk933) - (assign29260_e41327 * locals.var_fac1__blk933_dn11)) / (locals.var_fac1__blk933 * locals.var_fac1__blk933)), ((((-(locals.var_vgpld__blk935_dn12 + locals.var_vxbgmtcl__blk925_dn12)) * locals.var_fac1__blk933) - (assign29260_e41327 * locals.var_fac1__blk933_dn12)) / (locals.var_fac1__blk933 * locals.var_fac1__blk933)), ((((-(locals.var_vgpld__blk935_dn17 + locals.var_vxbgmtcl__blk925_dn17)) * locals.var_fac1__blk933) - (assign29260_e41327 * locals.var_fac1__blk933_dn17)) / (locals.var_fac1__blk933 * locals.var_fac1__blk933)),)
    } else {
        (locals.var_td__blk954, locals.var_td__blk954_dn0, locals.var_td__blk954_dn2, locals.var_td__blk954_dn6, locals.var_td__blk954_dn7, locals.var_td__blk954_dn10, locals.var_td__blk954_dn11, locals.var_td__blk954_dn12, locals.var_td__blk954_dn17,)
    }
};
        locals.var_td__blk954 = assign29260_e41331;
        locals.var_td__blk954_dn0 = assign29260_e41331_d_n0;
        locals.var_td__blk954_dn2 = assign29260_e41331_d_n2;
        locals.var_td__blk954_dn6 = assign29260_e41331_d_n6;
        locals.var_td__blk954_dn7 = assign29260_e41331_d_n7;
        locals.var_td__blk954_dn10 = assign29260_e41331_d_n10;
        locals.var_td__blk954_dn11 = assign29260_e41331_d_n11;
        locals.var_td__blk954_dn12 = assign29260_e41331_d_n12;
        locals.var_td__blk954_dn17 = assign29260_e41331_d_n17;
        locals.var_td__blk954_rv = 0.0;

        let (assign29270_e41373, assign29270_e41373_d_n0, assign29270_e41373_d_n2, assign29270_e41373_d_n6, assign29270_e41373_d_n7, assign29270_e41373_d_n10, assign29270_e41373_d_n11, assign29270_e41373_d_n12, assign29270_e41373_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29270_e41345: f64 = (locals.var_tb__blk952 * locals.var_tb__blk952);
        let assign29270_e41347: f64 = (assign29270_e41345 * locals.var_tb__blk952);
        let assign29270_e41350: f64 = (27.0 * locals.var_ta__blk951);
        let assign29270_e41352: f64 = (assign29270_e41350 * locals.var_ta__blk951);
        let assign29270_e41354: f64 = (assign29270_e41352 * locals.var_ta__blk951);
        let assign29270_e41355: f64 = (assign29270_e41347 / assign29270_e41354);
        let assign29270_e41358: f64 = (locals.var_tb__blk952 * locals.var_tc__blk953);
        let assign29270_e41361: f64 = (6.0 * locals.var_ta__blk951);
        let assign29270_e41363: f64 = (assign29270_e41361 * locals.var_ta__blk951);
        let assign29270_e41364: f64 = (assign29270_e41358 / assign29270_e41363);
        let assign29270_e41365: f64 = (assign29270_e41355 - assign29270_e41364);
        let assign29270_e41369: f64 = (2.0 * locals.var_ta__blk951);
        let assign29270_e41370: f64 = (locals.var_td__blk954 / assign29270_e41369);
        let assign29270_e41371: f64 = (assign29270_e41365 + assign29270_e41370);
        (assign29270_e41371, ((-((locals.var_tb__blk952 * locals.var_tc__blk953_dn0) / assign29270_e41363)) + (locals.var_td__blk954_dn0 / assign29270_e41369)), ((-((locals.var_tb__blk952 * locals.var_tc__blk953_dn2) / assign29270_e41363)) + (locals.var_td__blk954_dn2 / assign29270_e41369)), ((-((locals.var_tb__blk952 * locals.var_tc__blk953_dn6) / assign29270_e41363)) + (locals.var_td__blk954_dn6 / assign29270_e41369)), ((-((locals.var_tb__blk952 * locals.var_tc__blk953_dn7) / assign29270_e41363)) + (locals.var_td__blk954_dn7 / assign29270_e41369)), ((-((locals.var_tb__blk952 * locals.var_tc__blk953_dn10) / assign29270_e41363)) + (locals.var_td__blk954_dn10 / assign29270_e41369)), ((-((locals.var_tb__blk952 * locals.var_tc__blk953_dn11) / assign29270_e41363)) + (locals.var_td__blk954_dn11 / assign29270_e41369)), ((-((locals.var_tb__blk952 * locals.var_tc__blk953_dn12) / assign29270_e41363)) + (locals.var_td__blk954_dn12 / assign29270_e41369)), ((-((locals.var_tb__blk952 * locals.var_tc__blk953_dn17) / assign29270_e41363)) + (locals.var_td__blk954_dn17 / assign29270_e41369)),)
    } else {
        (locals.var_tq__blk955, locals.var_tq__blk955_dn0, locals.var_tq__blk955_dn2, locals.var_tq__blk955_dn6, locals.var_tq__blk955_dn7, locals.var_tq__blk955_dn10, locals.var_tq__blk955_dn11, locals.var_tq__blk955_dn12, locals.var_tq__blk955_dn17,)
    }
};
        locals.var_tq__blk955 = assign29270_e41373;
        locals.var_tq__blk955_dn0 = assign29270_e41373_d_n0;
        locals.var_tq__blk955_dn2 = assign29270_e41373_d_n2;
        locals.var_tq__blk955_dn6 = assign29270_e41373_d_n6;
        locals.var_tq__blk955_dn7 = assign29270_e41373_d_n7;
        locals.var_tq__blk955_dn10 = assign29270_e41373_d_n10;
        locals.var_tq__blk955_dn11 = assign29270_e41373_d_n11;
        locals.var_tq__blk955_dn12 = assign29270_e41373_d_n12;
        locals.var_tq__blk955_dn17 = assign29270_e41373_d_n17;
        locals.var_tq__blk955_rv = 0.0;

        let (assign29280_e41401, assign29280_e41401_d_n0, assign29280_e41401_d_n2, assign29280_e41401_d_n6, assign29280_e41401_d_n7, assign29280_e41401_d_n10, assign29280_e41401_d_n11, assign29280_e41401_d_n12, assign29280_e41401_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29280_e41387: f64 = (3.0 * locals.var_ta__blk951);
        let assign29280_e41389: f64 = (assign29280_e41387 * locals.var_tc__blk953);
        let assign29280_e41392: f64 = (locals.var_tb__blk952 * locals.var_tb__blk952);
        let assign29280_e41393: f64 = (assign29280_e41389 - assign29280_e41392);
        let assign29280_e41396: f64 = (9.0 * locals.var_ta__blk951);
        let assign29280_e41398: f64 = (assign29280_e41396 * locals.var_ta__blk951);
        let assign29280_e41399: f64 = (assign29280_e41393 / assign29280_e41398);
        (assign29280_e41399, ((assign29280_e41387 * locals.var_tc__blk953_dn0) / assign29280_e41398), ((assign29280_e41387 * locals.var_tc__blk953_dn2) / assign29280_e41398), ((assign29280_e41387 * locals.var_tc__blk953_dn6) / assign29280_e41398), ((assign29280_e41387 * locals.var_tc__blk953_dn7) / assign29280_e41398), ((assign29280_e41387 * locals.var_tc__blk953_dn10) / assign29280_e41398), ((assign29280_e41387 * locals.var_tc__blk953_dn11) / assign29280_e41398), ((assign29280_e41387 * locals.var_tc__blk953_dn12) / assign29280_e41398), ((assign29280_e41387 * locals.var_tc__blk953_dn17) / assign29280_e41398),)
    } else {
        (locals.var_tp__blk956, locals.var_tp__blk956_dn0, locals.var_tp__blk956_dn2, locals.var_tp__blk956_dn6, locals.var_tp__blk956_dn7, locals.var_tp__blk956_dn10, locals.var_tp__blk956_dn11, locals.var_tp__blk956_dn12, locals.var_tp__blk956_dn17,)
    }
};
        locals.var_tp__blk956 = assign29280_e41401;
        locals.var_tp__blk956_dn0 = assign29280_e41401_d_n0;
        locals.var_tp__blk956_dn2 = assign29280_e41401_d_n2;
        locals.var_tp__blk956_dn6 = assign29280_e41401_d_n6;
        locals.var_tp__blk956_dn7 = assign29280_e41401_d_n7;
        locals.var_tp__blk956_dn10 = assign29280_e41401_d_n10;
        locals.var_tp__blk956_dn11 = assign29280_e41401_d_n11;
        locals.var_tp__blk956_dn12 = assign29280_e41401_d_n12;
        locals.var_tp__blk956_dn17 = assign29280_e41401_d_n17;
        locals.var_tp__blk956_rv = 0.0;

        let (assign29290_e41424, assign29290_e41424_d_n0, assign29290_e41424_d_n2, assign29290_e41424_d_n6, assign29290_e41424_d_n7, assign29290_e41424_d_n10, assign29290_e41424_d_n11, assign29290_e41424_d_n12, assign29290_e41424_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29290_e41415: f64 = (locals.var_tq__blk955 * locals.var_tq__blk955);
        let assign29290_e41418: f64 = (locals.var_tp__blk956 * locals.var_tp__blk956);
        let assign29290_e41420: f64 = (assign29290_e41418 * locals.var_tp__blk956);
        let assign29290_e41421: f64 = (assign29290_e41415 + assign29290_e41420);
        let assign29290_e41422: f64 = (assign29290_e41421).sqrt();
        (assign29290_e41422, ((((locals.var_tq__blk955_dn0 * locals.var_tq__blk955) + (locals.var_tq__blk955 * locals.var_tq__blk955_dn0)) + ((((locals.var_tp__blk956_dn0 * locals.var_tp__blk956) + (locals.var_tp__blk956 * locals.var_tp__blk956_dn0)) * locals.var_tp__blk956) + (assign29290_e41418 * locals.var_tp__blk956_dn0))) / (2.0 * assign29290_e41422)), ((((locals.var_tq__blk955_dn2 * locals.var_tq__blk955) + (locals.var_tq__blk955 * locals.var_tq__blk955_dn2)) + ((((locals.var_tp__blk956_dn2 * locals.var_tp__blk956) + (locals.var_tp__blk956 * locals.var_tp__blk956_dn2)) * locals.var_tp__blk956) + (assign29290_e41418 * locals.var_tp__blk956_dn2))) / (2.0 * assign29290_e41422)), ((((locals.var_tq__blk955_dn6 * locals.var_tq__blk955) + (locals.var_tq__blk955 * locals.var_tq__blk955_dn6)) + ((((locals.var_tp__blk956_dn6 * locals.var_tp__blk956) + (locals.var_tp__blk956 * locals.var_tp__blk956_dn6)) * locals.var_tp__blk956) + (assign29290_e41418 * locals.var_tp__blk956_dn6))) / (2.0 * assign29290_e41422)), ((((locals.var_tq__blk955_dn7 * locals.var_tq__blk955) + (locals.var_tq__blk955 * locals.var_tq__blk955_dn7)) + ((((locals.var_tp__blk956_dn7 * locals.var_tp__blk956) + (locals.var_tp__blk956 * locals.var_tp__blk956_dn7)) * locals.var_tp__blk956) + (assign29290_e41418 * locals.var_tp__blk956_dn7))) / (2.0 * assign29290_e41422)), ((((locals.var_tq__blk955_dn10 * locals.var_tq__blk955) + (locals.var_tq__blk955 * locals.var_tq__blk955_dn10)) + ((((locals.var_tp__blk956_dn10 * locals.var_tp__blk956) + (locals.var_tp__blk956 * locals.var_tp__blk956_dn10)) * locals.var_tp__blk956) + (assign29290_e41418 * locals.var_tp__blk956_dn10))) / (2.0 * assign29290_e41422)), ((((locals.var_tq__blk955_dn11 * locals.var_tq__blk955) + (locals.var_tq__blk955 * locals.var_tq__blk955_dn11)) + ((((locals.var_tp__blk956_dn11 * locals.var_tp__blk956) + (locals.var_tp__blk956 * locals.var_tp__blk956_dn11)) * locals.var_tp__blk956) + (assign29290_e41418 * locals.var_tp__blk956_dn11))) / (2.0 * assign29290_e41422)), ((((locals.var_tq__blk955_dn12 * locals.var_tq__blk955) + (locals.var_tq__blk955 * locals.var_tq__blk955_dn12)) + ((((locals.var_tp__blk956_dn12 * locals.var_tp__blk956) + (locals.var_tp__blk956 * locals.var_tp__blk956_dn12)) * locals.var_tp__blk956) + (assign29290_e41418 * locals.var_tp__blk956_dn12))) / (2.0 * assign29290_e41422)), ((((locals.var_tq__blk955_dn17 * locals.var_tq__blk955) + (locals.var_tq__blk955 * locals.var_tq__blk955_dn17)) + ((((locals.var_tp__blk956_dn17 * locals.var_tp__blk956) + (locals.var_tp__blk956 * locals.var_tp__blk956_dn17)) * locals.var_tp__blk956) + (assign29290_e41418 * locals.var_tp__blk956_dn17))) / (2.0 * assign29290_e41422)),)
    } else {
        (locals.var_t5__blk904, locals.var_t5__blk904_dn0, locals.var_t5__blk904_dn2, locals.var_t5__blk904_dn6, locals.var_t5__blk904_dn7, locals.var_t5__blk904_dn10, locals.var_t5__blk904_dn11, locals.var_t5__blk904_dn12, locals.var_t5__blk904_dn17,)
    }
};
        locals.var_t5__blk904 = assign29290_e41424;
        locals.var_t5__blk904_dn0 = assign29290_e41424_d_n0;
        locals.var_t5__blk904_dn2 = assign29290_e41424_d_n2;
        locals.var_t5__blk904_dn6 = assign29290_e41424_d_n6;
        locals.var_t5__blk904_dn7 = assign29290_e41424_d_n7;
        locals.var_t5__blk904_dn10 = assign29290_e41424_d_n10;
        locals.var_t5__blk904_dn11 = assign29290_e41424_d_n11;
        locals.var_t5__blk904_dn12 = assign29290_e41424_d_n12;
        locals.var_t5__blk904_dn17 = assign29290_e41424_d_n17;
        locals.var_t5__blk904_rv = 0.0;

        let (assign29300_e41443, assign29300_e41443_d_n0, assign29300_e41443_d_n2, assign29300_e41443_d_n6, assign29300_e41443_d_n7, assign29300_e41443_d_n10, assign29300_e41443_d_n11, assign29300_e41443_d_n12, assign29300_e41443_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29300_e41437: f64 = (-locals.var_tq__blk955);
        let assign29300_e41439: f64 = (assign29300_e41437 + locals.var_t5__blk904);
        let assign29300_e41441: f64 = (assign29300_e41439).powf(0.3333333333333333);
        (assign29300_e41441, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29300_e41439).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk955_dn0) + locals.var_t5__blk904_dn0))) } } else { (assign29300_e41441 * (0.3333333333333333 * (((-locals.var_tq__blk955_dn0) + locals.var_t5__blk904_dn0) / assign29300_e41439))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29300_e41439).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk955_dn2) + locals.var_t5__blk904_dn2))) } } else { (assign29300_e41441 * (0.3333333333333333 * (((-locals.var_tq__blk955_dn2) + locals.var_t5__blk904_dn2) / assign29300_e41439))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29300_e41439).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk955_dn6) + locals.var_t5__blk904_dn6))) } } else { (assign29300_e41441 * (0.3333333333333333 * (((-locals.var_tq__blk955_dn6) + locals.var_t5__blk904_dn6) / assign29300_e41439))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29300_e41439).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk955_dn7) + locals.var_t5__blk904_dn7))) } } else { (assign29300_e41441 * (0.3333333333333333 * (((-locals.var_tq__blk955_dn7) + locals.var_t5__blk904_dn7) / assign29300_e41439))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29300_e41439).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk955_dn10) + locals.var_t5__blk904_dn10))) } } else { (assign29300_e41441 * (0.3333333333333333 * (((-locals.var_tq__blk955_dn10) + locals.var_t5__blk904_dn10) / assign29300_e41439))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29300_e41439).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk955_dn11) + locals.var_t5__blk904_dn11))) } } else { (assign29300_e41441 * (0.3333333333333333 * (((-locals.var_tq__blk955_dn11) + locals.var_t5__blk904_dn11) / assign29300_e41439))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29300_e41439).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk955_dn12) + locals.var_t5__blk904_dn12))) } } else { (assign29300_e41441 * (0.3333333333333333 * (((-locals.var_tq__blk955_dn12) + locals.var_t5__blk904_dn12) / assign29300_e41439))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29300_e41439).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk955_dn17) + locals.var_t5__blk904_dn17))) } } else { (assign29300_e41441 * (0.3333333333333333 * (((-locals.var_tq__blk955_dn17) + locals.var_t5__blk904_dn17) / assign29300_e41439))) },)
    } else {
        (locals.var_tu__blk957, locals.var_tu__blk957_dn0, locals.var_tu__blk957_dn2, locals.var_tu__blk957_dn6, locals.var_tu__blk957_dn7, locals.var_tu__blk957_dn10, locals.var_tu__blk957_dn11, locals.var_tu__blk957_dn12, locals.var_tu__blk957_dn17,)
    }
};
        locals.var_tu__blk957 = assign29300_e41443;
        locals.var_tu__blk957_dn0 = assign29300_e41443_d_n0;
        locals.var_tu__blk957_dn2 = assign29300_e41443_d_n2;
        locals.var_tu__blk957_dn6 = assign29300_e41443_d_n6;
        locals.var_tu__blk957_dn7 = assign29300_e41443_d_n7;
        locals.var_tu__blk957_dn10 = assign29300_e41443_d_n10;
        locals.var_tu__blk957_dn11 = assign29300_e41443_d_n11;
        locals.var_tu__blk957_dn12 = assign29300_e41443_d_n12;
        locals.var_tu__blk957_dn17 = assign29300_e41443_d_n17;
        locals.var_tu__blk957_rv = 0.0;

        let (assign29310_e41462, assign29310_e41462_d_n0, assign29310_e41462_d_n2, assign29310_e41462_d_n6, assign29310_e41462_d_n7, assign29310_e41462_d_n10, assign29310_e41462_d_n11, assign29310_e41462_d_n12, assign29310_e41462_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29310_e41457: f64 = (locals.var_tq__blk955 + locals.var_t5__blk904);
        let assign29310_e41459: f64 = (assign29310_e41457).powf(0.3333333333333333);
        let assign29310_e41460: f64 = (-assign29310_e41459);
        (assign29310_e41460, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29310_e41457).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk955_dn0 + locals.var_t5__blk904_dn0))) } } else { (assign29310_e41459 * (0.3333333333333333 * ((locals.var_tq__blk955_dn0 + locals.var_t5__blk904_dn0) / assign29310_e41457))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29310_e41457).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk955_dn2 + locals.var_t5__blk904_dn2))) } } else { (assign29310_e41459 * (0.3333333333333333 * ((locals.var_tq__blk955_dn2 + locals.var_t5__blk904_dn2) / assign29310_e41457))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29310_e41457).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk955_dn6 + locals.var_t5__blk904_dn6))) } } else { (assign29310_e41459 * (0.3333333333333333 * ((locals.var_tq__blk955_dn6 + locals.var_t5__blk904_dn6) / assign29310_e41457))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29310_e41457).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk955_dn7 + locals.var_t5__blk904_dn7))) } } else { (assign29310_e41459 * (0.3333333333333333 * ((locals.var_tq__blk955_dn7 + locals.var_t5__blk904_dn7) / assign29310_e41457))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29310_e41457).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk955_dn10 + locals.var_t5__blk904_dn10))) } } else { (assign29310_e41459 * (0.3333333333333333 * ((locals.var_tq__blk955_dn10 + locals.var_t5__blk904_dn10) / assign29310_e41457))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29310_e41457).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk955_dn11 + locals.var_t5__blk904_dn11))) } } else { (assign29310_e41459 * (0.3333333333333333 * ((locals.var_tq__blk955_dn11 + locals.var_t5__blk904_dn11) / assign29310_e41457))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29310_e41457).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk955_dn12 + locals.var_t5__blk904_dn12))) } } else { (assign29310_e41459 * (0.3333333333333333 * ((locals.var_tq__blk955_dn12 + locals.var_t5__blk904_dn12) / assign29310_e41457))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29310_e41457).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk955_dn17 + locals.var_t5__blk904_dn17))) } } else { (assign29310_e41459 * (0.3333333333333333 * ((locals.var_tq__blk955_dn17 + locals.var_t5__blk904_dn17) / assign29310_e41457))) }),)
    } else {
        (locals.var_tv__blk958, locals.var_tv__blk958_dn0, locals.var_tv__blk958_dn2, locals.var_tv__blk958_dn6, locals.var_tv__blk958_dn7, locals.var_tv__blk958_dn10, locals.var_tv__blk958_dn11, locals.var_tv__blk958_dn12, locals.var_tv__blk958_dn17,)
    }
};
        locals.var_tv__blk958 = assign29310_e41462;
        locals.var_tv__blk958_dn0 = assign29310_e41462_d_n0;
        locals.var_tv__blk958_dn2 = assign29310_e41462_d_n2;
        locals.var_tv__blk958_dn6 = assign29310_e41462_d_n6;
        locals.var_tv__blk958_dn7 = assign29310_e41462_d_n7;
        locals.var_tv__blk958_dn10 = assign29310_e41462_d_n10;
        locals.var_tv__blk958_dn11 = assign29310_e41462_d_n11;
        locals.var_tv__blk958_dn12 = assign29310_e41462_d_n12;
        locals.var_tv__blk958_dn17 = assign29310_e41462_d_n17;
        locals.var_tv__blk958_rv = 0.0;

        let (assign29320_e41484, assign29320_e41484_d_n0, assign29320_e41484_d_n2, assign29320_e41484_d_n6, assign29320_e41484_d_n7, assign29320_e41484_d_n10, assign29320_e41484_d_n11, assign29320_e41484_d_n12, assign29320_e41484_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29320_e41476: f64 = (locals.var_tu__blk957 + locals.var_tv__blk958);
        let assign29320_e41480: f64 = (3.0 * locals.var_ta__blk951);
        let assign29320_e41481: f64 = (locals.var_tb__blk952 / assign29320_e41480);
        let assign29320_e41482: f64 = (assign29320_e41476 - assign29320_e41481);
        (assign29320_e41482, (locals.var_tu__blk957_dn0 + locals.var_tv__blk958_dn0), (locals.var_tu__blk957_dn2 + locals.var_tv__blk958_dn2), (locals.var_tu__blk957_dn6 + locals.var_tv__blk958_dn6), (locals.var_tu__blk957_dn7 + locals.var_tv__blk958_dn7), (locals.var_tu__blk957_dn10 + locals.var_tv__blk958_dn10), (locals.var_tu__blk957_dn11 + locals.var_tv__blk958_dn11), (locals.var_tu__blk957_dn12 + locals.var_tv__blk958_dn12), (locals.var_tu__blk957_dn17 + locals.var_tv__blk958_dn17),)
    } else {
        (locals.var_tx__blk908, locals.var_tx__blk908_dn0, locals.var_tx__blk908_dn2, locals.var_tx__blk908_dn6, locals.var_tx__blk908_dn7, locals.var_tx__blk908_dn10, locals.var_tx__blk908_dn11, locals.var_tx__blk908_dn12, locals.var_tx__blk908_dn17,)
    }
};
        locals.var_tx__blk908 = assign29320_e41484;
        locals.var_tx__blk908_dn0 = assign29320_e41484_d_n0;
        locals.var_tx__blk908_dn2 = assign29320_e41484_d_n2;
        locals.var_tx__blk908_dn6 = assign29320_e41484_d_n6;
        locals.var_tx__blk908_dn7 = assign29320_e41484_d_n7;
        locals.var_tx__blk908_dn10 = assign29320_e41484_d_n10;
        locals.var_tx__blk908_dn11 = assign29320_e41484_d_n11;
        locals.var_tx__blk908_dn12 = assign29320_e41484_d_n12;
        locals.var_tx__blk908_dn17 = assign29320_e41484_d_n17;
        locals.var_tx__blk908_rv = 0.0;

        let (assign29330_e41502, assign29330_e41502_d_n0, assign29330_e41502_d_n2, assign29330_e41502_d_n6, assign29330_e41502_d_n7, assign29330_e41502_d_n10, assign29330_e41502_d_n11, assign29330_e41502_d_n12, assign29330_e41502_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29330_e41498: f64 = (locals.var_tx__blk908 * locals.var_beta_inv);
        let assign29330_e41500: f64 = (assign29330_e41498 - locals.var_vxbgmtcl__blk925);
        (assign29330_e41500, ((locals.var_tx__blk908_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn0), ((locals.var_tx__blk908_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn2), ((locals.var_tx__blk908_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn6), ((locals.var_tx__blk908_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn7), (((locals.var_tx__blk908_dn10 * locals.var_beta_inv) + (locals.var_tx__blk908 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl__blk925_dn10), ((locals.var_tx__blk908_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn11), ((locals.var_tx__blk908_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn12), ((locals.var_tx__blk908_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk925_dn17),)
    } else {
        (locals.var_ps0_inia__blk950, locals.var_ps0_inia__blk950_dn0, locals.var_ps0_inia__blk950_dn2, locals.var_ps0_inia__blk950_dn6, locals.var_ps0_inia__blk950_dn7, locals.var_ps0_inia__blk950_dn10, locals.var_ps0_inia__blk950_dn11, locals.var_ps0_inia__blk950_dn12, locals.var_ps0_inia__blk950_dn17,)
    }
};
        locals.var_ps0_inia__blk950 = assign29330_e41502;
        locals.var_ps0_inia__blk950_dn0 = assign29330_e41502_d_n0;
        locals.var_ps0_inia__blk950_dn2 = assign29330_e41502_d_n2;
        locals.var_ps0_inia__blk950_dn6 = assign29330_e41502_d_n6;
        locals.var_ps0_inia__blk950_dn7 = assign29330_e41502_d_n7;
        locals.var_ps0_inia__blk950_dn10 = assign29330_e41502_d_n10;
        locals.var_ps0_inia__blk950_dn11 = assign29330_e41502_d_n11;
        locals.var_ps0_inia__blk950_dn12 = assign29330_e41502_d_n12;
        locals.var_ps0_inia__blk950_dn17 = assign29330_e41502_d_n17;
        locals.var_ps0_inia__blk950_rv = 0.0;

        let (assign29340_e41520, assign29340_e41520_d_n0, assign29340_e41520_d_n2, assign29340_e41520_d_n6, assign29340_e41520_d_n7, assign29340_e41520_d_n10, assign29340_e41520_d_n11, assign29340_e41520_d_n12, assign29340_e41520_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29340_e41517: f64 = (locals.var_ps0_inia__blk950 + locals.var_vxbgmtcl__blk925);
        let assign29340_e41518: f64 = (locals.var_beta * assign29340_e41517);
        (assign29340_e41518, (locals.var_beta * (locals.var_ps0_inia__blk950_dn0 + locals.var_vxbgmtcl__blk925_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn2 + locals.var_vxbgmtcl__blk925_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn6 + locals.var_vxbgmtcl__blk925_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn7 + locals.var_vxbgmtcl__blk925_dn7)), ((locals.var_beta_dn10 * assign29340_e41517) + (locals.var_beta * (locals.var_ps0_inia__blk950_dn10 + locals.var_vxbgmtcl__blk925_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk950_dn11 + locals.var_vxbgmtcl__blk925_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn12 + locals.var_vxbgmtcl__blk925_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk950_dn17 + locals.var_vxbgmtcl__blk925_dn17)),)
    } else {
        (locals.var_chi__blk947, locals.var_chi__blk947_dn0, locals.var_chi__blk947_dn2, locals.var_chi__blk947_dn6, locals.var_chi__blk947_dn7, locals.var_chi__blk947_dn10, locals.var_chi__blk947_dn11, locals.var_chi__blk947_dn12, locals.var_chi__blk947_dn17,)
    }
};
        locals.var_chi__blk947 = assign29340_e41520;
        locals.var_chi__blk947_dn0 = assign29340_e41520_d_n0;
        locals.var_chi__blk947_dn2 = assign29340_e41520_d_n2;
        locals.var_chi__blk947_dn6 = assign29340_e41520_d_n6;
        locals.var_chi__blk947_dn7 = assign29340_e41520_d_n7;
        locals.var_chi__blk947_dn10 = assign29340_e41520_d_n10;
        locals.var_chi__blk947_dn11 = assign29340_e41520_d_n11;
        locals.var_chi__blk947_dn12 = assign29340_e41520_d_n12;
        locals.var_chi__blk947_dn17 = assign29340_e41520_d_n17;
        locals.var_chi__blk947_rv = 0.0;

        let assign29350_e41523: f64 = if p.p41 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard991 = assign29350_e41523;
        locals.var_guard991_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_106(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29370_e41557, assign29370_e41557_d_n0, assign29370_e41557_d_n2, assign29370_e41557_d_n6, assign29370_e41557_d_n7, assign29370_e41557_d_n10, assign29370_e41557_d_n11, assign29370_e41557_d_n12, assign29370_e41557_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29370_e41553: f64 = (locals.var_vgpld__blk935 + locals.var_vxbgmtcl__blk925);
        let assign29370_e41555: f64 = (assign29370_e41553 + 0.1);
        (assign29370_e41555, (locals.var_vgpld__blk935_dn0 + locals.var_vxbgmtcl__blk925_dn0), (locals.var_vgpld__blk935_dn2 + locals.var_vxbgmtcl__blk925_dn2), (locals.var_vgpld__blk935_dn6 + locals.var_vxbgmtcl__blk925_dn6), (locals.var_vgpld__blk935_dn7 + locals.var_vxbgmtcl__blk925_dn7), (locals.var_vgpld__blk935_dn10 + locals.var_vxbgmtcl__blk925_dn10), (locals.var_vgpld__blk935_dn11 + locals.var_vxbgmtcl__blk925_dn11), (locals.var_vgpld__blk935_dn12 + locals.var_vxbgmtcl__blk925_dn12), (locals.var_vgpld__blk935_dn17 + locals.var_vxbgmtcl__blk925_dn17),)
    } else {
        (locals.var_vgpld_shift__blk959, locals.var_vgpld_shift__blk959_dn0, locals.var_vgpld_shift__blk959_dn2, locals.var_vgpld_shift__blk959_dn6, locals.var_vgpld_shift__blk959_dn7, locals.var_vgpld_shift__blk959_dn10, locals.var_vgpld_shift__blk959_dn11, locals.var_vgpld_shift__blk959_dn12, locals.var_vgpld_shift__blk959_dn17,)
    }
};
        locals.var_vgpld_shift__blk959 = assign29370_e41557;
        locals.var_vgpld_shift__blk959_dn0 = assign29370_e41557_d_n0;
        locals.var_vgpld_shift__blk959_dn2 = assign29370_e41557_d_n2;
        locals.var_vgpld_shift__blk959_dn6 = assign29370_e41557_d_n6;
        locals.var_vgpld_shift__blk959_dn7 = assign29370_e41557_d_n7;
        locals.var_vgpld_shift__blk959_dn10 = assign29370_e41557_d_n10;
        locals.var_vgpld_shift__blk959_dn11 = assign29370_e41557_d_n11;
        locals.var_vgpld_shift__blk959_dn12 = assign29370_e41557_d_n12;
        locals.var_vgpld_shift__blk959_dn17 = assign29370_e41557_d_n17;
        locals.var_vgpld_shift__blk959_rv = 0.0;

        let (assign29380_e41577, assign29380_e41577_d_n0, assign29380_e41577_d_n2, assign29380_e41577_d_n6, assign29380_e41577_d_n7, assign29380_e41577_d_n10, assign29380_e41577_d_n11, assign29380_e41577_d_n12, assign29380_e41577_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29380_e41571: f64 = (-locals.var_vxbgmtcl__blk925);
        let assign29380_e41572: f64 = (locals.var_beta * assign29380_e41571);
        let assign29380_e41573: f64 = (assign29380_e41572).exp();
        let assign29380_e41575: f64 = (assign29380_e41573 + 1e-50);
        (assign29380_e41575, (assign29380_e41573 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn0))), (assign29380_e41573 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn2))), (assign29380_e41573 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn6))), (assign29380_e41573 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn7))), (assign29380_e41573 * ((locals.var_beta_dn10 * assign29380_e41571) + (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn10)))), (assign29380_e41573 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn11))), (assign29380_e41573 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn12))), (assign29380_e41573 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk966, locals.var_exp_bvbs__blk966_dn0, locals.var_exp_bvbs__blk966_dn2, locals.var_exp_bvbs__blk966_dn6, locals.var_exp_bvbs__blk966_dn7, locals.var_exp_bvbs__blk966_dn10, locals.var_exp_bvbs__blk966_dn11, locals.var_exp_bvbs__blk966_dn12, locals.var_exp_bvbs__blk966_dn17,)
    }
};
        locals.var_exp_bvbs__blk966 = assign29380_e41577;
        locals.var_exp_bvbs__blk966_dn0 = assign29380_e41577_d_n0;
        locals.var_exp_bvbs__blk966_dn2 = assign29380_e41577_d_n2;
        locals.var_exp_bvbs__blk966_dn6 = assign29380_e41577_d_n6;
        locals.var_exp_bvbs__blk966_dn7 = assign29380_e41577_d_n7;
        locals.var_exp_bvbs__blk966_dn10 = assign29380_e41577_d_n10;
        locals.var_exp_bvbs__blk966_dn11 = assign29380_e41577_d_n11;
        locals.var_exp_bvbs__blk966_dn12 = assign29380_e41577_d_n12;
        locals.var_exp_bvbs__blk966_dn17 = assign29380_e41577_d_n17;
        locals.var_exp_bvbs__blk966_rv = 0.0;

        let (assign29390_e41593, assign29390_e41593_d_n0, assign29390_e41593_d_n2, assign29390_e41593_d_n6, assign29390_e41593_d_n7, assign29390_e41593_d_n10, assign29390_e41593_d_n11, assign29390_e41593_d_n12, assign29390_e41593_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29390_e41591: f64 = (locals.var_nin / locals.var_mks_nover);
        (assign29390_e41591, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn7 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover), (locals.var_nin_dn17 / locals.var_mks_nover),)
    } else {
        (locals.var_t0__blk899, locals.var_t0__blk899_dn0, locals.var_t0__blk899_dn2, locals.var_t0__blk899_dn6, locals.var_t0__blk899_dn7, locals.var_t0__blk899_dn10, locals.var_t0__blk899_dn11, locals.var_t0__blk899_dn12, locals.var_t0__blk899_dn17,)
    }
};
        locals.var_t0__blk899 = assign29390_e41593;
        locals.var_t0__blk899_dn0 = assign29390_e41593_d_n0;
        locals.var_t0__blk899_dn2 = assign29390_e41593_d_n2;
        locals.var_t0__blk899_dn6 = assign29390_e41593_d_n6;
        locals.var_t0__blk899_dn7 = assign29390_e41593_d_n7;
        locals.var_t0__blk899_dn10 = assign29390_e41593_d_n10;
        locals.var_t0__blk899_dn11 = assign29390_e41593_d_n11;
        locals.var_t0__blk899_dn12 = assign29390_e41593_d_n12;
        locals.var_t0__blk899_dn17 = assign29390_e41593_d_n17;
        locals.var_t0__blk899_rv = 0.0;

        let (assign29400_e41609, assign29400_e41609_d_n0, assign29400_e41609_d_n2, assign29400_e41609_d_n6, assign29400_e41609_d_n7, assign29400_e41609_d_n10, assign29400_e41609_d_n11, assign29400_e41609_d_n12, assign29400_e41609_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29400_e41607: f64 = (locals.var_t0__blk899 * locals.var_t0__blk899);
        (assign29400_e41607, ((locals.var_t0__blk899_dn0 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn0)), ((locals.var_t0__blk899_dn2 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn2)), ((locals.var_t0__blk899_dn6 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn6)), ((locals.var_t0__blk899_dn7 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn7)), ((locals.var_t0__blk899_dn10 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn10)), ((locals.var_t0__blk899_dn11 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn11)), ((locals.var_t0__blk899_dn12 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn12)), ((locals.var_t0__blk899_dn17 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn17)),)
    } else {
        (locals.var_cnst1over__blk960, locals.var_cnst1over__blk960_dn0, locals.var_cnst1over__blk960_dn2, locals.var_cnst1over__blk960_dn6, locals.var_cnst1over__blk960_dn7, locals.var_cnst1over__blk960_dn10, locals.var_cnst1over__blk960_dn11, locals.var_cnst1over__blk960_dn12, locals.var_cnst1over__blk960_dn17,)
    }
};
        locals.var_cnst1over__blk960 = assign29400_e41609;
        locals.var_cnst1over__blk960_dn0 = assign29400_e41609_d_n0;
        locals.var_cnst1over__blk960_dn2 = assign29400_e41609_d_n2;
        locals.var_cnst1over__blk960_dn6 = assign29400_e41609_d_n6;
        locals.var_cnst1over__blk960_dn7 = assign29400_e41609_d_n7;
        locals.var_cnst1over__blk960_dn10 = assign29400_e41609_d_n10;
        locals.var_cnst1over__blk960_dn11 = assign29400_e41609_d_n11;
        locals.var_cnst1over__blk960_dn12 = assign29400_e41609_d_n12;
        locals.var_cnst1over__blk960_dn17 = assign29400_e41609_d_n17;
        locals.var_cnst1over__blk960_rv = 0.0;

        let (assign29410_e41625, assign29410_e41625_d_n0, assign29410_e41625_d_n2, assign29410_e41625_d_n6, assign29410_e41625_d_n7, assign29410_e41625_d_n10, assign29410_e41625_d_n11, assign29410_e41625_d_n12, assign29410_e41625_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29410_e41623: f64 = (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966);
        (assign29410_e41623, ((locals.var_cnst1over__blk960_dn0 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn0)), ((locals.var_cnst1over__blk960_dn2 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn2)), ((locals.var_cnst1over__blk960_dn6 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn6)), ((locals.var_cnst1over__blk960_dn7 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn7)), ((locals.var_cnst1over__blk960_dn10 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn10)), ((locals.var_cnst1over__blk960_dn11 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn11)), ((locals.var_cnst1over__blk960_dn12 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn12)), ((locals.var_cnst1over__blk960_dn17 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn17)),)
    } else {
        (locals.var_gammachi__blk961, locals.var_gammachi__blk961_dn0, locals.var_gammachi__blk961_dn2, locals.var_gammachi__blk961_dn6, locals.var_gammachi__blk961_dn7, locals.var_gammachi__blk961_dn10, locals.var_gammachi__blk961_dn11, locals.var_gammachi__blk961_dn12, locals.var_gammachi__blk961_dn17,)
    }
};
        locals.var_gammachi__blk961 = assign29410_e41625;
        locals.var_gammachi__blk961_dn0 = assign29410_e41625_d_n0;
        locals.var_gammachi__blk961_dn2 = assign29410_e41625_d_n2;
        locals.var_gammachi__blk961_dn6 = assign29410_e41625_d_n6;
        locals.var_gammachi__blk961_dn7 = assign29410_e41625_d_n7;
        locals.var_gammachi__blk961_dn10 = assign29410_e41625_d_n10;
        locals.var_gammachi__blk961_dn11 = assign29410_e41625_d_n11;
        locals.var_gammachi__blk961_dn12 = assign29410_e41625_d_n12;
        locals.var_gammachi__blk961_dn17 = assign29410_e41625_d_n17;
        locals.var_gammachi__blk961_rv = 0.0;

        let (assign29420_e41641, assign29420_e41641_d_n0, assign29420_e41641_d_n2, assign29420_e41641_d_n6, assign29420_e41641_d_n7, assign29420_e41641_d_n10, assign29420_e41641_d_n11, assign29420_e41641_d_n12, assign29420_e41641_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29420_e41639: f64 = (locals.var_beta2 * locals.var_fac1p2__blk934);
        (assign29420_e41639, (locals.var_beta2 * locals.var_fac1p2__blk934_dn0), (locals.var_beta2 * locals.var_fac1p2__blk934_dn2), (locals.var_beta2 * locals.var_fac1p2__blk934_dn6), (locals.var_beta2 * locals.var_fac1p2__blk934_dn7), ((locals.var_beta2_dn10 * locals.var_fac1p2__blk934) + (locals.var_beta2 * locals.var_fac1p2__blk934_dn10)), (locals.var_beta2 * locals.var_fac1p2__blk934_dn11), (locals.var_beta2 * locals.var_fac1p2__blk934_dn12), (locals.var_beta2 * locals.var_fac1p2__blk934_dn17),)
    } else {
        (locals.var_t0__blk899, locals.var_t0__blk899_dn0, locals.var_t0__blk899_dn2, locals.var_t0__blk899_dn6, locals.var_t0__blk899_dn7, locals.var_t0__blk899_dn10, locals.var_t0__blk899_dn11, locals.var_t0__blk899_dn12, locals.var_t0__blk899_dn17,)
    }
};
        locals.var_t0__blk899 = assign29420_e41641;
        locals.var_t0__blk899_dn0 = assign29420_e41641_d_n0;
        locals.var_t0__blk899_dn2 = assign29420_e41641_d_n2;
        locals.var_t0__blk899_dn6 = assign29420_e41641_d_n6;
        locals.var_t0__blk899_dn7 = assign29420_e41641_d_n7;
        locals.var_t0__blk899_dn10 = assign29420_e41641_d_n10;
        locals.var_t0__blk899_dn11 = assign29420_e41641_d_n11;
        locals.var_t0__blk899_dn12 = assign29420_e41641_d_n12;
        locals.var_t0__blk899_dn17 = assign29420_e41641_d_n17;
        locals.var_t0__blk899_rv = 0.0;

        let (assign29430_e41657, assign29430_e41657_d_n0, assign29430_e41657_d_n2, assign29430_e41657_d_n6, assign29430_e41657_d_n7, assign29430_e41657_d_n10, assign29430_e41657_d_n11, assign29430_e41657_d_n12, assign29430_e41657_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29430_e41655: f64 = (locals.var_beta * locals.var_vgpld_shift__blk959);
        (assign29430_e41655, (locals.var_beta * locals.var_vgpld_shift__blk959_dn0), (locals.var_beta * locals.var_vgpld_shift__blk959_dn2), (locals.var_beta * locals.var_vgpld_shift__blk959_dn6), (locals.var_beta * locals.var_vgpld_shift__blk959_dn7), ((locals.var_beta_dn10 * locals.var_vgpld_shift__blk959) + (locals.var_beta * locals.var_vgpld_shift__blk959_dn10)), (locals.var_beta * locals.var_vgpld_shift__blk959_dn11), (locals.var_beta * locals.var_vgpld_shift__blk959_dn12), (locals.var_beta * locals.var_vgpld_shift__blk959_dn17),)
    } else {
        (locals.var_psi__blk962, locals.var_psi__blk962_dn0, locals.var_psi__blk962_dn2, locals.var_psi__blk962_dn6, locals.var_psi__blk962_dn7, locals.var_psi__blk962_dn10, locals.var_psi__blk962_dn11, locals.var_psi__blk962_dn12, locals.var_psi__blk962_dn17,)
    }
};
        locals.var_psi__blk962 = assign29430_e41657;
        locals.var_psi__blk962_dn0 = assign29430_e41657_d_n0;
        locals.var_psi__blk962_dn2 = assign29430_e41657_d_n2;
        locals.var_psi__blk962_dn6 = assign29430_e41657_d_n6;
        locals.var_psi__blk962_dn7 = assign29430_e41657_d_n7;
        locals.var_psi__blk962_dn10 = assign29430_e41657_d_n10;
        locals.var_psi__blk962_dn11 = assign29430_e41657_d_n11;
        locals.var_psi__blk962_dn12 = assign29430_e41657_d_n12;
        locals.var_psi__blk962_dn17 = assign29430_e41657_d_n17;
        locals.var_psi__blk962_rv = 0.0;

        let (assign29440_e41687, assign29440_e41687_d_n0, assign29440_e41687_d_n2, assign29440_e41687_d_n6, assign29440_e41687_d_n7, assign29440_e41687_d_n10, assign29440_e41687_d_n11, assign29440_e41687_d_n12, assign29440_e41687_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29440_e41671: f64 = (locals.var_gammachi__blk961 * locals.var_t0__blk899);
        let assign29440_e41674: f64 = (locals.var_psi__blk962 * locals.var_psi__blk962);
        let assign29440_e41675: f64 = (assign29440_e41671 + assign29440_e41674);
        let assign29440_e41676: f64 = (assign29440_e41675).ln();
        let assign29440_e41679: f64 = (locals.var_cnst1over__blk960 * locals.var_t0__blk899);
        let assign29440_e41680: f64 = (assign29440_e41679).ln();
        let assign29440_e41681: f64 = (assign29440_e41676 - assign29440_e41680);
        let assign29440_e41684: f64 = (locals.var_beta * locals.var_vxbgmtcl__blk925);
        let assign29440_e41685: f64 = (assign29440_e41681 + assign29440_e41684);
        (assign29440_e41685, ((((((locals.var_gammachi__blk961_dn0 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn0)) + ((locals.var_psi__blk962_dn0 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn0))) / assign29440_e41675) - (((locals.var_cnst1over__blk960_dn0 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn0)) / assign29440_e41679)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn0)), ((((((locals.var_gammachi__blk961_dn2 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn2)) + ((locals.var_psi__blk962_dn2 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn2))) / assign29440_e41675) - (((locals.var_cnst1over__blk960_dn2 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn2)) / assign29440_e41679)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn2)), ((((((locals.var_gammachi__blk961_dn6 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn6)) + ((locals.var_psi__blk962_dn6 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn6))) / assign29440_e41675) - (((locals.var_cnst1over__blk960_dn6 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn6)) / assign29440_e41679)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn6)), ((((((locals.var_gammachi__blk961_dn7 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn7)) + ((locals.var_psi__blk962_dn7 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn7))) / assign29440_e41675) - (((locals.var_cnst1over__blk960_dn7 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn7)) / assign29440_e41679)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn7)), ((((((locals.var_gammachi__blk961_dn10 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn10)) + ((locals.var_psi__blk962_dn10 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn10))) / assign29440_e41675) - (((locals.var_cnst1over__blk960_dn10 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn10)) / assign29440_e41679)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl__blk925) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn10))), ((((((locals.var_gammachi__blk961_dn11 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn11)) + ((locals.var_psi__blk962_dn11 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn11))) / assign29440_e41675) - (((locals.var_cnst1over__blk960_dn11 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn11)) / assign29440_e41679)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn11)), ((((((locals.var_gammachi__blk961_dn12 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn12)) + ((locals.var_psi__blk962_dn12 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn12))) / assign29440_e41675) - (((locals.var_cnst1over__blk960_dn12 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn12)) / assign29440_e41679)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn12)), ((((((locals.var_gammachi__blk961_dn17 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn17)) + ((locals.var_psi__blk962_dn17 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn17))) / assign29440_e41675) - (((locals.var_cnst1over__blk960_dn17 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn17)) / assign29440_e41679)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn17)),)
    } else {
        (locals.var_chi_1__blk963, locals.var_chi_1__blk963_dn0, locals.var_chi_1__blk963_dn2, locals.var_chi_1__blk963_dn6, locals.var_chi_1__blk963_dn7, locals.var_chi_1__blk963_dn10, locals.var_chi_1__blk963_dn11, locals.var_chi_1__blk963_dn12, locals.var_chi_1__blk963_dn17,)
    }
};
        locals.var_chi_1__blk963 = assign29440_e41687;
        locals.var_chi_1__blk963_dn0 = assign29440_e41687_d_n0;
        locals.var_chi_1__blk963_dn2 = assign29440_e41687_d_n2;
        locals.var_chi_1__blk963_dn6 = assign29440_e41687_d_n6;
        locals.var_chi_1__blk963_dn7 = assign29440_e41687_d_n7;
        locals.var_chi_1__blk963_dn10 = assign29440_e41687_d_n10;
        locals.var_chi_1__blk963_dn11 = assign29440_e41687_d_n11;
        locals.var_chi_1__blk963_dn12 = assign29440_e41687_d_n12;
        locals.var_chi_1__blk963_dn17 = assign29440_e41687_d_n17;
        locals.var_chi_1__blk963_rv = 0.0;

        let (assign29450_e41705, assign29450_e41705_d_n0, assign29450_e41705_d_n2, assign29450_e41705_d_n6, assign29450_e41705_d_n7, assign29450_e41705_d_n10, assign29450_e41705_d_n11, assign29450_e41705_d_n12, assign29450_e41705_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29450_e41701: f64 = (locals.var_psi__blk962 - locals.var_chi_1__blk963);
        let assign29450_e41703: f64 = (assign29450_e41701 - 1.0);
        (assign29450_e41703, (locals.var_psi__blk962_dn0 - locals.var_chi_1__blk963_dn0), (locals.var_psi__blk962_dn2 - locals.var_chi_1__blk963_dn2), (locals.var_psi__blk962_dn6 - locals.var_chi_1__blk963_dn6), (locals.var_psi__blk962_dn7 - locals.var_chi_1__blk963_dn7), (locals.var_psi__blk962_dn10 - locals.var_chi_1__blk963_dn10), (locals.var_psi__blk962_dn11 - locals.var_chi_1__blk963_dn11), (locals.var_psi__blk962_dn12 - locals.var_chi_1__blk963_dn12), (locals.var_psi__blk962_dn17 - locals.var_chi_1__blk963_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign29450_e41705;
        locals.var_tmf1_dn0 = assign29450_e41705_d_n0;
        locals.var_tmf1_dn2 = assign29450_e41705_d_n2;
        locals.var_tmf1_dn6 = assign29450_e41705_d_n6;
        locals.var_tmf1_dn7 = assign29450_e41705_d_n7;
        locals.var_tmf1_dn10 = assign29450_e41705_d_n10;
        locals.var_tmf1_dn11 = assign29450_e41705_d_n11;
        locals.var_tmf1_dn12 = assign29450_e41705_d_n12;
        locals.var_tmf1_dn17 = assign29450_e41705_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign29460_e41723, assign29460_e41723_d_n0, assign29460_e41723_d_n2, assign29460_e41723_d_n6, assign29460_e41723_d_n7, assign29460_e41723_d_n10, assign29460_e41723_d_n11, assign29460_e41723_d_n12, assign29460_e41723_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29460_e41719: f64 = (4.0 * locals.var_psi__blk962);
        let assign29460_e41721: f64 = assign29460_e41719;
        (assign29460_e41721, (4.0 * locals.var_psi__blk962_dn0), (4.0 * locals.var_psi__blk962_dn2), (4.0 * locals.var_psi__blk962_dn6), (4.0 * locals.var_psi__blk962_dn7), (4.0 * locals.var_psi__blk962_dn10), (4.0 * locals.var_psi__blk962_dn11), (4.0 * locals.var_psi__blk962_dn12), (4.0 * locals.var_psi__blk962_dn17),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29460_e41723;
        locals.var_tmf2_dn0 = assign29460_e41723_d_n0;
        locals.var_tmf2_dn2 = assign29460_e41723_d_n2;
        locals.var_tmf2_dn6 = assign29460_e41723_d_n6;
        locals.var_tmf2_dn7 = assign29460_e41723_d_n7;
        locals.var_tmf2_dn10 = assign29460_e41723_d_n10;
        locals.var_tmf2_dn11 = assign29460_e41723_d_n11;
        locals.var_tmf2_dn12 = assign29460_e41723_d_n12;
        locals.var_tmf2_dn17 = assign29460_e41723_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29470_e41743, assign29470_e41743_d_n0, assign29470_e41743_d_n2, assign29470_e41743_d_n6, assign29470_e41743_d_n7, assign29470_e41743_d_n10, assign29470_e41743_d_n11, assign29470_e41743_d_n12, assign29470_e41743_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let (assign29470_e41741, assign29470_e41741_d_n0, assign29470_e41741_d_n2, assign29470_e41741_d_n6, assign29470_e41741_d_n7, assign29470_e41741_d_n10, assign29470_e41741_d_n11, assign29470_e41741_d_n12, assign29470_e41741_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign29470_e41740: f64 = (-locals.var_tmf2);
                (assign29470_e41740, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign29470_e41741, assign29470_e41741_d_n0, assign29470_e41741_d_n2, assign29470_e41741_d_n6, assign29470_e41741_d_n7, assign29470_e41741_d_n10, assign29470_e41741_d_n11, assign29470_e41741_d_n12, assign29470_e41741_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29470_e41743;
        locals.var_tmf2_dn0 = assign29470_e41743_d_n0;
        locals.var_tmf2_dn2 = assign29470_e41743_d_n2;
        locals.var_tmf2_dn6 = assign29470_e41743_d_n6;
        locals.var_tmf2_dn7 = assign29470_e41743_d_n7;
        locals.var_tmf2_dn10 = assign29470_e41743_d_n10;
        locals.var_tmf2_dn11 = assign29470_e41743_d_n11;
        locals.var_tmf2_dn12 = assign29470_e41743_d_n12;
        locals.var_tmf2_dn17 = assign29470_e41743_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29480_e41762, assign29480_e41762_d_n0, assign29480_e41762_d_n2, assign29480_e41762_d_n6, assign29480_e41762_d_n7, assign29480_e41762_d_n10, assign29480_e41762_d_n11, assign29480_e41762_d_n12, assign29480_e41762_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29480_e41757: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign29480_e41759: f64 = (assign29480_e41757 + locals.var_tmf2);
        let assign29480_e41760: f64 = (assign29480_e41759).sqrt();
        (assign29480_e41760, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign29480_e41760)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign29480_e41760)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign29480_e41760)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign29480_e41760)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign29480_e41760)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign29480_e41760)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign29480_e41760)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign29480_e41760)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29480_e41762;
        locals.var_tmf2_dn0 = assign29480_e41762_d_n0;
        locals.var_tmf2_dn2 = assign29480_e41762_d_n2;
        locals.var_tmf2_dn6 = assign29480_e41762_d_n6;
        locals.var_tmf2_dn7 = assign29480_e41762_d_n7;
        locals.var_tmf2_dn10 = assign29480_e41762_d_n10;
        locals.var_tmf2_dn11 = assign29480_e41762_d_n11;
        locals.var_tmf2_dn12 = assign29480_e41762_d_n12;
        locals.var_tmf2_dn17 = assign29480_e41762_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29490_e41782, assign29490_e41782_d_n0, assign29490_e41782_d_n2, assign29490_e41782_d_n6, assign29490_e41782_d_n7, assign29490_e41782_d_n10, assign29490_e41782_d_n11, assign29490_e41782_d_n12, assign29490_e41782_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29490_e41778: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign29490_e41779: f64 = (1.0 + assign29490_e41778);
        let assign29490_e41780: f64 = (0.5 * assign29490_e41779);
        (assign29490_e41780, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign29490_e41782;
        locals.var_t1__blk900_dn0 = assign29490_e41782_d_n0;
        locals.var_t1__blk900_dn2 = assign29490_e41782_d_n2;
        locals.var_t1__blk900_dn6 = assign29490_e41782_d_n6;
        locals.var_t1__blk900_dn7 = assign29490_e41782_d_n7;
        locals.var_t1__blk900_dn10 = assign29490_e41782_d_n10;
        locals.var_t1__blk900_dn11 = assign29490_e41782_d_n11;
        locals.var_t1__blk900_dn12 = assign29490_e41782_d_n12;
        locals.var_t1__blk900_dn17 = assign29490_e41782_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign29500_e41806, assign29500_e41806_d_n0, assign29500_e41806_d_n2, assign29500_e41806_d_n6, assign29500_e41806_d_n7, assign29500_e41806_d_n10, assign29500_e41806_d_n11, assign29500_e41806_d_n12, assign29500_e41806_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29500_e41799: f64 = 2.0;
        let assign29500_e41800: f64 = (locals.var_tmf1 + assign29500_e41799);
        let assign29500_e41802: f64 = (assign29500_e41800 / locals.var_tmf2);
        let assign29500_e41803: f64 = (1.0 - assign29500_e41802);
        let assign29500_e41804: f64 = (0.5 * assign29500_e41803);
        (assign29500_e41804, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign29500_e41800 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign29500_e41800 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign29500_e41800 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign29500_e41800 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign29500_e41800 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign29500_e41800 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign29500_e41800 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign29500_e41800 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk901, locals.var_t2__blk901_dn0, locals.var_t2__blk901_dn2, locals.var_t2__blk901_dn6, locals.var_t2__blk901_dn7, locals.var_t2__blk901_dn10, locals.var_t2__blk901_dn11, locals.var_t2__blk901_dn12, locals.var_t2__blk901_dn17,)
    }
};
        locals.var_t2__blk901 = assign29500_e41806;
        locals.var_t2__blk901_dn0 = assign29500_e41806_d_n0;
        locals.var_t2__blk901_dn2 = assign29500_e41806_d_n2;
        locals.var_t2__blk901_dn6 = assign29500_e41806_d_n6;
        locals.var_t2__blk901_dn7 = assign29500_e41806_d_n7;
        locals.var_t2__blk901_dn10 = assign29500_e41806_d_n10;
        locals.var_t2__blk901_dn11 = assign29500_e41806_d_n11;
        locals.var_t2__blk901_dn12 = assign29500_e41806_d_n12;
        locals.var_t2__blk901_dn17 = assign29500_e41806_d_n17;
        locals.var_t2__blk901_rv = 0.0;

        let (assign29510_e41826, assign29510_e41826_d_n0, assign29510_e41826_d_n2, assign29510_e41826_d_n6, assign29510_e41826_d_n7, assign29510_e41826_d_n10, assign29510_e41826_d_n11, assign29510_e41826_d_n12, assign29510_e41826_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29510_e41822: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign29510_e41823: f64 = (0.5 * assign29510_e41822);
        let assign29510_e41824: f64 = (locals.var_psi__blk962 - assign29510_e41823);
        (assign29510_e41824, (locals.var_psi__blk962_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi__blk962_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi__blk962_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi__blk962_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi__blk962_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi__blk962_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi__blk962_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psi__blk962_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi_1__blk963, locals.var_chi_1__blk963_dn0, locals.var_chi_1__blk963_dn2, locals.var_chi_1__blk963_dn6, locals.var_chi_1__blk963_dn7, locals.var_chi_1__blk963_dn10, locals.var_chi_1__blk963_dn11, locals.var_chi_1__blk963_dn12, locals.var_chi_1__blk963_dn17,)
    }
};
        locals.var_chi_1__blk963 = assign29510_e41826;
        locals.var_chi_1__blk963_dn0 = assign29510_e41826_d_n0;
        locals.var_chi_1__blk963_dn2 = assign29510_e41826_d_n2;
        locals.var_chi_1__blk963_dn6 = assign29510_e41826_d_n6;
        locals.var_chi_1__blk963_dn7 = assign29510_e41826_d_n7;
        locals.var_chi_1__blk963_dn10 = assign29510_e41826_d_n10;
        locals.var_chi_1__blk963_dn11 = assign29510_e41826_d_n11;
        locals.var_chi_1__blk963_dn12 = assign29510_e41826_d_n12;
        locals.var_chi_1__blk963_dn17 = assign29510_e41826_d_n17;
        locals.var_chi_1__blk963_rv = 0.0;

        let (assign29520_e41842, assign29520_e41842_d_n0, assign29520_e41842_d_n2, assign29520_e41842_d_n6, assign29520_e41842_d_n7, assign29520_e41842_d_n10, assign29520_e41842_d_n11, assign29520_e41842_d_n12, assign29520_e41842_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29520_e41840: f64 = (locals.var_psi__blk962 - locals.var_chi_1__blk963);
        (assign29520_e41840, (locals.var_psi__blk962_dn0 - locals.var_chi_1__blk963_dn0), (locals.var_psi__blk962_dn2 - locals.var_chi_1__blk963_dn2), (locals.var_psi__blk962_dn6 - locals.var_chi_1__blk963_dn6), (locals.var_psi__blk962_dn7 - locals.var_chi_1__blk963_dn7), (locals.var_psi__blk962_dn10 - locals.var_chi_1__blk963_dn10), (locals.var_psi__blk962_dn11 - locals.var_chi_1__blk963_dn11), (locals.var_psi__blk962_dn12 - locals.var_chi_1__blk963_dn12), (locals.var_psi__blk962_dn17 - locals.var_chi_1__blk963_dn17),)
    } else {
        (locals.var_psi__blk962, locals.var_psi__blk962_dn0, locals.var_psi__blk962_dn2, locals.var_psi__blk962_dn6, locals.var_psi__blk962_dn7, locals.var_psi__blk962_dn10, locals.var_psi__blk962_dn11, locals.var_psi__blk962_dn12, locals.var_psi__blk962_dn17,)
    }
};
        locals.var_psi__blk962 = assign29520_e41842;
        locals.var_psi__blk962_dn0 = assign29520_e41842_d_n0;
        locals.var_psi__blk962_dn2 = assign29520_e41842_d_n2;
        locals.var_psi__blk962_dn6 = assign29520_e41842_d_n6;
        locals.var_psi__blk962_dn7 = assign29520_e41842_d_n7;
        locals.var_psi__blk962_dn10 = assign29520_e41842_d_n10;
        locals.var_psi__blk962_dn11 = assign29520_e41842_d_n11;
        locals.var_psi__blk962_dn12 = assign29520_e41842_d_n12;
        locals.var_psi__blk962_dn17 = assign29520_e41842_d_n17;
        locals.var_psi__blk962_rv = 0.0;

        let (assign29530_e41860, assign29530_e41860_d_n0, assign29530_e41860_d_n2, assign29530_e41860_d_n6, assign29530_e41860_d_n7, assign29530_e41860_d_n10, assign29530_e41860_d_n11, assign29530_e41860_d_n12, assign29530_e41860_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29530_e41857: f64 = (locals.var_beta * 0.1);
        let assign29530_e41858: f64 = (locals.var_psi__blk962 + assign29530_e41857);
        (assign29530_e41858, locals.var_psi__blk962_dn0, locals.var_psi__blk962_dn2, locals.var_psi__blk962_dn6, locals.var_psi__blk962_dn7, (locals.var_psi__blk962_dn10 + (locals.var_beta_dn10 * 0.1)), locals.var_psi__blk962_dn11, locals.var_psi__blk962_dn12, locals.var_psi__blk962_dn17,)
    } else {
        (locals.var_psi__blk962, locals.var_psi__blk962_dn0, locals.var_psi__blk962_dn2, locals.var_psi__blk962_dn6, locals.var_psi__blk962_dn7, locals.var_psi__blk962_dn10, locals.var_psi__blk962_dn11, locals.var_psi__blk962_dn12, locals.var_psi__blk962_dn17,)
    }
};
        locals.var_psi__blk962 = assign29530_e41860;
        locals.var_psi__blk962_dn0 = assign29530_e41860_d_n0;
        locals.var_psi__blk962_dn2 = assign29530_e41860_d_n2;
        locals.var_psi__blk962_dn6 = assign29530_e41860_d_n6;
        locals.var_psi__blk962_dn7 = assign29530_e41860_d_n7;
        locals.var_psi__blk962_dn10 = assign29530_e41860_d_n10;
        locals.var_psi__blk962_dn11 = assign29530_e41860_d_n11;
        locals.var_psi__blk962_dn12 = assign29530_e41860_d_n12;
        locals.var_psi__blk962_dn17 = assign29530_e41860_d_n17;
        locals.var_psi__blk962_rv = 0.0;

        let (assign29540_e41890, assign29540_e41890_d_n0, assign29540_e41890_d_n2, assign29540_e41890_d_n6, assign29540_e41890_d_n7, assign29540_e41890_d_n10, assign29540_e41890_d_n11, assign29540_e41890_d_n12, assign29540_e41890_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29540_e41874: f64 = (locals.var_gammachi__blk961 * locals.var_t0__blk899);
        let assign29540_e41877: f64 = (locals.var_psi__blk962 * locals.var_psi__blk962);
        let assign29540_e41878: f64 = (assign29540_e41874 + assign29540_e41877);
        let assign29540_e41879: f64 = (assign29540_e41878).ln();
        let assign29540_e41882: f64 = (locals.var_cnst1over__blk960 * locals.var_t0__blk899);
        let assign29540_e41883: f64 = (assign29540_e41882).ln();
        let assign29540_e41884: f64 = (assign29540_e41879 - assign29540_e41883);
        let assign29540_e41887: f64 = (locals.var_beta * locals.var_vxbgmtcl__blk925);
        let assign29540_e41888: f64 = (assign29540_e41884 + assign29540_e41887);
        (assign29540_e41888, ((((((locals.var_gammachi__blk961_dn0 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn0)) + ((locals.var_psi__blk962_dn0 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn0))) / assign29540_e41878) - (((locals.var_cnst1over__blk960_dn0 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn0)) / assign29540_e41882)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn0)), ((((((locals.var_gammachi__blk961_dn2 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn2)) + ((locals.var_psi__blk962_dn2 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn2))) / assign29540_e41878) - (((locals.var_cnst1over__blk960_dn2 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn2)) / assign29540_e41882)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn2)), ((((((locals.var_gammachi__blk961_dn6 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn6)) + ((locals.var_psi__blk962_dn6 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn6))) / assign29540_e41878) - (((locals.var_cnst1over__blk960_dn6 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn6)) / assign29540_e41882)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn6)), ((((((locals.var_gammachi__blk961_dn7 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn7)) + ((locals.var_psi__blk962_dn7 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn7))) / assign29540_e41878) - (((locals.var_cnst1over__blk960_dn7 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn7)) / assign29540_e41882)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn7)), ((((((locals.var_gammachi__blk961_dn10 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn10)) + ((locals.var_psi__blk962_dn10 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn10))) / assign29540_e41878) - (((locals.var_cnst1over__blk960_dn10 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn10)) / assign29540_e41882)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl__blk925) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn10))), ((((((locals.var_gammachi__blk961_dn11 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn11)) + ((locals.var_psi__blk962_dn11 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn11))) / assign29540_e41878) - (((locals.var_cnst1over__blk960_dn11 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn11)) / assign29540_e41882)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn11)), ((((((locals.var_gammachi__blk961_dn12 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn12)) + ((locals.var_psi__blk962_dn12 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn12))) / assign29540_e41878) - (((locals.var_cnst1over__blk960_dn12 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn12)) / assign29540_e41882)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn12)), ((((((locals.var_gammachi__blk961_dn17 * locals.var_t0__blk899) + (locals.var_gammachi__blk961 * locals.var_t0__blk899_dn17)) + ((locals.var_psi__blk962_dn17 * locals.var_psi__blk962) + (locals.var_psi__blk962 * locals.var_psi__blk962_dn17))) / assign29540_e41878) - (((locals.var_cnst1over__blk960_dn17 * locals.var_t0__blk899) + (locals.var_cnst1over__blk960 * locals.var_t0__blk899_dn17)) / assign29540_e41882)) + (locals.var_beta * locals.var_vxbgmtcl__blk925_dn17)),)
    } else {
        (locals.var_chi_b__blk964, locals.var_chi_b__blk964_dn0, locals.var_chi_b__blk964_dn2, locals.var_chi_b__blk964_dn6, locals.var_chi_b__blk964_dn7, locals.var_chi_b__blk964_dn10, locals.var_chi_b__blk964_dn11, locals.var_chi_b__blk964_dn12, locals.var_chi_b__blk964_dn17,)
    }
};
        locals.var_chi_b__blk964 = assign29540_e41890;
        locals.var_chi_b__blk964_dn0 = assign29540_e41890_d_n0;
        locals.var_chi_b__blk964_dn2 = assign29540_e41890_d_n2;
        locals.var_chi_b__blk964_dn6 = assign29540_e41890_d_n6;
        locals.var_chi_b__blk964_dn7 = assign29540_e41890_d_n7;
        locals.var_chi_b__blk964_dn10 = assign29540_e41890_d_n10;
        locals.var_chi_b__blk964_dn11 = assign29540_e41890_d_n11;
        locals.var_chi_b__blk964_dn12 = assign29540_e41890_d_n12;
        locals.var_chi_b__blk964_dn17 = assign29540_e41890_d_n17;
        locals.var_chi_b__blk964_rv = 0.0;

        let (assign29550_e41904, assign29550_e41904_d_n0, assign29550_e41904_d_n2, assign29550_e41904_d_n6, assign29550_e41904_d_n7, assign29550_e41904_d_n10, assign29550_e41904_d_n11, assign29550_e41904_d_n12, assign29550_e41904_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        (locals.var_chi__blk947, locals.var_chi__blk947_dn0, locals.var_chi__blk947_dn2, locals.var_chi__blk947_dn6, locals.var_chi__blk947_dn7, locals.var_chi__blk947_dn10, locals.var_chi__blk947_dn11, locals.var_chi__blk947_dn12, locals.var_chi__blk947_dn17,)
    } else {
        (locals.var_chi_a__blk965, locals.var_chi_a__blk965_dn0, locals.var_chi_a__blk965_dn2, locals.var_chi_a__blk965_dn6, locals.var_chi_a__blk965_dn7, locals.var_chi_a__blk965_dn10, locals.var_chi_a__blk965_dn11, locals.var_chi_a__blk965_dn12, locals.var_chi_a__blk965_dn17,)
    }
};
        locals.var_chi_a__blk965 = assign29550_e41904;
        locals.var_chi_a__blk965_dn0 = assign29550_e41904_d_n0;
        locals.var_chi_a__blk965_dn2 = assign29550_e41904_d_n2;
        locals.var_chi_a__blk965_dn6 = assign29550_e41904_d_n6;
        locals.var_chi_a__blk965_dn7 = assign29550_e41904_d_n7;
        locals.var_chi_a__blk965_dn10 = assign29550_e41904_d_n10;
        locals.var_chi_a__blk965_dn11 = assign29550_e41904_d_n11;
        locals.var_chi_a__blk965_dn12 = assign29550_e41904_d_n12;
        locals.var_chi_a__blk965_dn17 = assign29550_e41904_d_n17;
        locals.var_chi_a__blk965_rv = 0.0;

        let (assign29560_e41924, assign29560_e41924_d_n0, assign29560_e41924_d_n2, assign29560_e41924_d_n6, assign29560_e41924_d_n7, assign29560_e41924_d_n10, assign29560_e41924_d_n11, assign29560_e41924_d_n12, assign29560_e41924_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29560_e41918: f64 = (locals.var_chi_b__blk964 - locals.var_chi_a__blk965);
        let assign29560_e41921: f64 = (0.0008 * 75.0);
        let assign29560_e41922: f64 = (assign29560_e41918 - assign29560_e41921);
        (assign29560_e41922, (locals.var_chi_b__blk964_dn0 - locals.var_chi_a__blk965_dn0), (locals.var_chi_b__blk964_dn2 - locals.var_chi_a__blk965_dn2), (locals.var_chi_b__blk964_dn6 - locals.var_chi_a__blk965_dn6), (locals.var_chi_b__blk964_dn7 - locals.var_chi_a__blk965_dn7), (locals.var_chi_b__blk964_dn10 - locals.var_chi_a__blk965_dn10), (locals.var_chi_b__blk964_dn11 - locals.var_chi_a__blk965_dn11), (locals.var_chi_b__blk964_dn12 - locals.var_chi_a__blk965_dn12), (locals.var_chi_b__blk964_dn17 - locals.var_chi_a__blk965_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign29560_e41924;
        locals.var_tmf1_dn0 = assign29560_e41924_d_n0;
        locals.var_tmf1_dn2 = assign29560_e41924_d_n2;
        locals.var_tmf1_dn6 = assign29560_e41924_d_n6;
        locals.var_tmf1_dn7 = assign29560_e41924_d_n7;
        locals.var_tmf1_dn10 = assign29560_e41924_d_n10;
        locals.var_tmf1_dn11 = assign29560_e41924_d_n11;
        locals.var_tmf1_dn12 = assign29560_e41924_d_n12;
        locals.var_tmf1_dn17 = assign29560_e41924_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign29570_e41944, assign29570_e41944_d_n0, assign29570_e41944_d_n2, assign29570_e41944_d_n6, assign29570_e41944_d_n7, assign29570_e41944_d_n10, assign29570_e41944_d_n11, assign29570_e41944_d_n12, assign29570_e41944_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29570_e41938: f64 = (4.0 * locals.var_chi_b__blk964);
        let assign29570_e41941: f64 = (0.0008 * 75.0);
        let assign29570_e41942: f64 = (assign29570_e41938 * assign29570_e41941);
        (assign29570_e41942, ((4.0 * locals.var_chi_b__blk964_dn0) * assign29570_e41941), ((4.0 * locals.var_chi_b__blk964_dn2) * assign29570_e41941), ((4.0 * locals.var_chi_b__blk964_dn6) * assign29570_e41941), ((4.0 * locals.var_chi_b__blk964_dn7) * assign29570_e41941), ((4.0 * locals.var_chi_b__blk964_dn10) * assign29570_e41941), ((4.0 * locals.var_chi_b__blk964_dn11) * assign29570_e41941), ((4.0 * locals.var_chi_b__blk964_dn12) * assign29570_e41941), ((4.0 * locals.var_chi_b__blk964_dn17) * assign29570_e41941),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29570_e41944;
        locals.var_tmf2_dn0 = assign29570_e41944_d_n0;
        locals.var_tmf2_dn2 = assign29570_e41944_d_n2;
        locals.var_tmf2_dn6 = assign29570_e41944_d_n6;
        locals.var_tmf2_dn7 = assign29570_e41944_d_n7;
        locals.var_tmf2_dn10 = assign29570_e41944_d_n10;
        locals.var_tmf2_dn11 = assign29570_e41944_d_n11;
        locals.var_tmf2_dn12 = assign29570_e41944_d_n12;
        locals.var_tmf2_dn17 = assign29570_e41944_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29580_e41964, assign29580_e41964_d_n0, assign29580_e41964_d_n2, assign29580_e41964_d_n6, assign29580_e41964_d_n7, assign29580_e41964_d_n10, assign29580_e41964_d_n11, assign29580_e41964_d_n12, assign29580_e41964_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let (assign29580_e41962, assign29580_e41962_d_n0, assign29580_e41962_d_n2, assign29580_e41962_d_n6, assign29580_e41962_d_n7, assign29580_e41962_d_n10, assign29580_e41962_d_n11, assign29580_e41962_d_n12, assign29580_e41962_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign29580_e41961: f64 = (-locals.var_tmf2);
                (assign29580_e41961, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign29580_e41962, assign29580_e41962_d_n0, assign29580_e41962_d_n2, assign29580_e41962_d_n6, assign29580_e41962_d_n7, assign29580_e41962_d_n10, assign29580_e41962_d_n11, assign29580_e41962_d_n12, assign29580_e41962_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29580_e41964;
        locals.var_tmf2_dn0 = assign29580_e41964_d_n0;
        locals.var_tmf2_dn2 = assign29580_e41964_d_n2;
        locals.var_tmf2_dn6 = assign29580_e41964_d_n6;
        locals.var_tmf2_dn7 = assign29580_e41964_d_n7;
        locals.var_tmf2_dn10 = assign29580_e41964_d_n10;
        locals.var_tmf2_dn11 = assign29580_e41964_d_n11;
        locals.var_tmf2_dn12 = assign29580_e41964_d_n12;
        locals.var_tmf2_dn17 = assign29580_e41964_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29590_e41983, assign29590_e41983_d_n0, assign29590_e41983_d_n2, assign29590_e41983_d_n6, assign29590_e41983_d_n7, assign29590_e41983_d_n10, assign29590_e41983_d_n11, assign29590_e41983_d_n12, assign29590_e41983_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29590_e41978: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign29590_e41980: f64 = (assign29590_e41978 + locals.var_tmf2);
        let assign29590_e41981: f64 = (assign29590_e41980).sqrt();
        (assign29590_e41981, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign29590_e41981)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign29590_e41981)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign29590_e41981)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign29590_e41981)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign29590_e41981)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign29590_e41981)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign29590_e41981)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign29590_e41981)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29590_e41983;
        locals.var_tmf2_dn0 = assign29590_e41983_d_n0;
        locals.var_tmf2_dn2 = assign29590_e41983_d_n2;
        locals.var_tmf2_dn6 = assign29590_e41983_d_n6;
        locals.var_tmf2_dn7 = assign29590_e41983_d_n7;
        locals.var_tmf2_dn10 = assign29590_e41983_d_n10;
        locals.var_tmf2_dn11 = assign29590_e41983_d_n11;
        locals.var_tmf2_dn12 = assign29590_e41983_d_n12;
        locals.var_tmf2_dn17 = assign29590_e41983_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29600_e42003, assign29600_e42003_d_n0, assign29600_e42003_d_n2, assign29600_e42003_d_n6, assign29600_e42003_d_n7, assign29600_e42003_d_n10, assign29600_e42003_d_n11, assign29600_e42003_d_n12, assign29600_e42003_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29600_e41999: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign29600_e42000: f64 = (1.0 + assign29600_e41999);
        let assign29600_e42001: f64 = (0.5 * assign29600_e42000);
        (assign29600_e42001, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign29600_e42003;
        locals.var_t1__blk900_dn0 = assign29600_e42003_d_n0;
        locals.var_t1__blk900_dn2 = assign29600_e42003_d_n2;
        locals.var_t1__blk900_dn6 = assign29600_e42003_d_n6;
        locals.var_t1__blk900_dn7 = assign29600_e42003_d_n7;
        locals.var_t1__blk900_dn10 = assign29600_e42003_d_n10;
        locals.var_t1__blk900_dn11 = assign29600_e42003_d_n11;
        locals.var_t1__blk900_dn12 = assign29600_e42003_d_n12;
        locals.var_t1__blk900_dn17 = assign29600_e42003_d_n17;
        locals.var_t1__blk900_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_107(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29610_e42029, assign29610_e42029_d_n0, assign29610_e42029_d_n2, assign29610_e42029_d_n6, assign29610_e42029_d_n7, assign29610_e42029_d_n10, assign29610_e42029_d_n11, assign29610_e42029_d_n12, assign29610_e42029_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29610_e42020: f64 = (2.0 * 0.0008);
        let assign29610_e42022: f64 = (assign29610_e42020 * 75.0);
        let assign29610_e42023: f64 = (locals.var_tmf1 + assign29610_e42022);
        let assign29610_e42025: f64 = (assign29610_e42023 / locals.var_tmf2);
        let assign29610_e42026: f64 = (1.0 - assign29610_e42025);
        let assign29610_e42027: f64 = (0.5 * assign29610_e42026);
        (assign29610_e42027, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign29610_e42023 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign29610_e42023 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign29610_e42023 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign29610_e42023 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign29610_e42023 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign29610_e42023 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign29610_e42023 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign29610_e42023 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk901, locals.var_t2__blk901_dn0, locals.var_t2__blk901_dn2, locals.var_t2__blk901_dn6, locals.var_t2__blk901_dn7, locals.var_t2__blk901_dn10, locals.var_t2__blk901_dn11, locals.var_t2__blk901_dn12, locals.var_t2__blk901_dn17,)
    }
};
        locals.var_t2__blk901 = assign29610_e42029;
        locals.var_t2__blk901_dn0 = assign29610_e42029_d_n0;
        locals.var_t2__blk901_dn2 = assign29610_e42029_d_n2;
        locals.var_t2__blk901_dn6 = assign29610_e42029_d_n6;
        locals.var_t2__blk901_dn7 = assign29610_e42029_d_n7;
        locals.var_t2__blk901_dn10 = assign29610_e42029_d_n10;
        locals.var_t2__blk901_dn11 = assign29610_e42029_d_n11;
        locals.var_t2__blk901_dn12 = assign29610_e42029_d_n12;
        locals.var_t2__blk901_dn17 = assign29610_e42029_d_n17;
        locals.var_t2__blk901_rv = 0.0;

        let (assign29620_e42049, assign29620_e42049_d_n0, assign29620_e42049_d_n2, assign29620_e42049_d_n6, assign29620_e42049_d_n7, assign29620_e42049_d_n10, assign29620_e42049_d_n11, assign29620_e42049_d_n12, assign29620_e42049_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29620_e42045: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign29620_e42046: f64 = (0.5 * assign29620_e42045);
        let assign29620_e42047: f64 = (locals.var_chi_b__blk964 - assign29620_e42046);
        (assign29620_e42047, (locals.var_chi_b__blk964_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b__blk964_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b__blk964_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b__blk964_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_chi_b__blk964_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b__blk964_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b__blk964_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_chi_b__blk964_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi__blk947, locals.var_chi__blk947_dn0, locals.var_chi__blk947_dn2, locals.var_chi__blk947_dn6, locals.var_chi__blk947_dn7, locals.var_chi__blk947_dn10, locals.var_chi__blk947_dn11, locals.var_chi__blk947_dn12, locals.var_chi__blk947_dn17,)
    }
};
        locals.var_chi__blk947 = assign29620_e42049;
        locals.var_chi__blk947_dn0 = assign29620_e42049_d_n0;
        locals.var_chi__blk947_dn2 = assign29620_e42049_d_n2;
        locals.var_chi__blk947_dn6 = assign29620_e42049_d_n6;
        locals.var_chi__blk947_dn7 = assign29620_e42049_d_n7;
        locals.var_chi__blk947_dn10 = assign29620_e42049_d_n10;
        locals.var_chi__blk947_dn11 = assign29620_e42049_d_n11;
        locals.var_chi__blk947_dn12 = assign29620_e42049_d_n12;
        locals.var_chi__blk947_dn17 = assign29620_e42049_d_n17;
        locals.var_chi__blk947_rv = 0.0;

        let (assign29630_e42065, assign29630_e42065_d_n0, assign29630_e42065_d_n2, assign29630_e42065_d_n6, assign29630_e42065_d_n7, assign29630_e42065_d_n10, assign29630_e42065_d_n11, assign29630_e42065_d_n12, assign29630_e42065_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) {
        let assign29630_e42061: f64 = (locals.var_chi__blk947 / locals.var_beta);
        let assign29630_e42063: f64 = (assign29630_e42061 - locals.var_vxbgmtcl__blk925);
        (assign29630_e42063, ((locals.var_chi__blk947_dn0 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn0), ((locals.var_chi__blk947_dn2 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn2), ((locals.var_chi__blk947_dn6 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn6), ((locals.var_chi__blk947_dn7 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn7), ((((locals.var_chi__blk947_dn10 * locals.var_beta) - (locals.var_chi__blk947 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl__blk925_dn10), ((locals.var_chi__blk947_dn11 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn11), ((locals.var_chi__blk947_dn12 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn12), ((locals.var_chi__blk947_dn17 / locals.var_beta) - locals.var_vxbgmtcl__blk925_dn17),)
    } else {
        (locals.var_ps0ld__blk949, locals.var_ps0ld__blk949_dn0, locals.var_ps0ld__blk949_dn2, locals.var_ps0ld__blk949_dn6, locals.var_ps0ld__blk949_dn7, locals.var_ps0ld__blk949_dn10, locals.var_ps0ld__blk949_dn11, locals.var_ps0ld__blk949_dn12, locals.var_ps0ld__blk949_dn17,)
    }
};
        locals.var_ps0ld__blk949 = assign29630_e42065;
        locals.var_ps0ld__blk949_dn0 = assign29630_e42065_d_n0;
        locals.var_ps0ld__blk949_dn2 = assign29630_e42065_d_n2;
        locals.var_ps0ld__blk949_dn6 = assign29630_e42065_d_n6;
        locals.var_ps0ld__blk949_dn7 = assign29630_e42065_d_n7;
        locals.var_ps0ld__blk949_dn10 = assign29630_e42065_d_n10;
        locals.var_ps0ld__blk949_dn11 = assign29630_e42065_d_n11;
        locals.var_ps0ld__blk949_dn12 = assign29630_e42065_d_n12;
        locals.var_ps0ld__blk949_dn17 = assign29630_e42065_d_n17;
        locals.var_ps0ld__blk949_rv = 0.0;

        let (assign29640_e42083, assign29640_e42083_d_n0, assign29640_e42083_d_n2, assign29640_e42083_d_n6, assign29640_e42083_d_n7, assign29640_e42083_d_n10, assign29640_e42083_d_n11, assign29640_e42083_d_n12, assign29640_e42083_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) {
        let assign29640_e42077: f64 = (locals.var_chi__blk947 - 1.0);
        let assign29640_e42079: f64 = (-locals.var_chi__blk947);
        let assign29640_e42080: f64 = (assign29640_e42079).exp();
        let assign29640_e42081: f64 = (assign29640_e42077 + assign29640_e42080);
        (assign29640_e42081, (locals.var_chi__blk947_dn0 + (assign29640_e42080 * (-locals.var_chi__blk947_dn0))), (locals.var_chi__blk947_dn2 + (assign29640_e42080 * (-locals.var_chi__blk947_dn2))), (locals.var_chi__blk947_dn6 + (assign29640_e42080 * (-locals.var_chi__blk947_dn6))), (locals.var_chi__blk947_dn7 + (assign29640_e42080 * (-locals.var_chi__blk947_dn7))), (locals.var_chi__blk947_dn10 + (assign29640_e42080 * (-locals.var_chi__blk947_dn10))), (locals.var_chi__blk947_dn11 + (assign29640_e42080 * (-locals.var_chi__blk947_dn11))), (locals.var_chi__blk947_dn12 + (assign29640_e42080 * (-locals.var_chi__blk947_dn12))), (locals.var_chi__blk947_dn17 + (assign29640_e42080 * (-locals.var_chi__blk947_dn17))),)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign29640_e42083;
        locals.var_t1__blk900_dn0 = assign29640_e42083_d_n0;
        locals.var_t1__blk900_dn2 = assign29640_e42083_d_n2;
        locals.var_t1__blk900_dn6 = assign29640_e42083_d_n6;
        locals.var_t1__blk900_dn7 = assign29640_e42083_d_n7;
        locals.var_t1__blk900_dn10 = assign29640_e42083_d_n10;
        locals.var_t1__blk900_dn11 = assign29640_e42083_d_n11;
        locals.var_t1__blk900_dn12 = assign29640_e42083_d_n12;
        locals.var_t1__blk900_dn17 = assign29640_e42083_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let assign29650_e42087: f64 = (10.0 * 2.220446049250313e-16);
        let assign29650_e42088: f64 = if locals.var_t1__blk900 < assign29650_e42087 { 1.0 } else { 0.0 };
        locals.var_guard992 = assign29650_e42088;
        locals.var_guard992_rv = 0.0;

        let (assign29660_e42104, assign29660_e42104_d_n0, assign29660_e42104_d_n2, assign29660_e42104_d_n6, assign29660_e42104_d_n7, assign29660_e42104_d_n10, assign29660_e42104_d_n11, assign29660_e42104_d_n12, assign29660_e42104_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard992 != 0.0)) {
        let assign29660_e42102: f64 = (10.0 * 2.220446049250313e-16);
        (assign29660_e42102, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign29660_e42104;
        locals.var_t1__blk900_dn0 = assign29660_e42104_d_n0;
        locals.var_t1__blk900_dn2 = assign29660_e42104_d_n2;
        locals.var_t1__blk900_dn6 = assign29660_e42104_d_n6;
        locals.var_t1__blk900_dn7 = assign29660_e42104_d_n7;
        locals.var_t1__blk900_dn10 = assign29660_e42104_d_n10;
        locals.var_t1__blk900_dn11 = assign29660_e42104_d_n11;
        locals.var_t1__blk900_dn12 = assign29660_e42104_d_n12;
        locals.var_t1__blk900_dn17 = assign29660_e42104_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign29670_e42117, assign29670_e42117_d_n0, assign29670_e42117_d_n2, assign29670_e42117_d_n6, assign29670_e42117_d_n7, assign29670_e42117_d_n10, assign29670_e42117_d_n11, assign29670_e42117_d_n12, assign29670_e42117_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) {
        let assign29670_e42115: f64 = (locals.var_t1__blk900).sqrt();
        (assign29670_e42115, (locals.var_t1__blk900_dn0 / (2.0 * assign29670_e42115)), (locals.var_t1__blk900_dn2 / (2.0 * assign29670_e42115)), (locals.var_t1__blk900_dn6 / (2.0 * assign29670_e42115)), (locals.var_t1__blk900_dn7 / (2.0 * assign29670_e42115)), (locals.var_t1__blk900_dn10 / (2.0 * assign29670_e42115)), (locals.var_t1__blk900_dn11 / (2.0 * assign29670_e42115)), (locals.var_t1__blk900_dn12 / (2.0 * assign29670_e42115)), (locals.var_t1__blk900_dn17 / (2.0 * assign29670_e42115)),)
    } else {
        (locals.var_t2__blk901, locals.var_t2__blk901_dn0, locals.var_t2__blk901_dn2, locals.var_t2__blk901_dn6, locals.var_t2__blk901_dn7, locals.var_t2__blk901_dn10, locals.var_t2__blk901_dn11, locals.var_t2__blk901_dn12, locals.var_t2__blk901_dn17,)
    }
};
        locals.var_t2__blk901 = assign29670_e42117;
        locals.var_t2__blk901_dn0 = assign29670_e42117_d_n0;
        locals.var_t2__blk901_dn2 = assign29670_e42117_d_n2;
        locals.var_t2__blk901_dn6 = assign29670_e42117_d_n6;
        locals.var_t2__blk901_dn7 = assign29670_e42117_d_n7;
        locals.var_t2__blk901_dn10 = assign29670_e42117_d_n10;
        locals.var_t2__blk901_dn11 = assign29670_e42117_d_n11;
        locals.var_t2__blk901_dn12 = assign29670_e42117_d_n12;
        locals.var_t2__blk901_dn17 = assign29670_e42117_d_n17;
        locals.var_t2__blk901_rv = 0.0;

        let (assign29680_e42131, assign29680_e42131_d_n0, assign29680_e42131_d_n2, assign29680_e42131_d_n6, assign29680_e42131_d_n7, assign29680_e42131_d_n10, assign29680_e42131_d_n11, assign29680_e42131_d_n12, assign29680_e42131_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) {
        let assign29680_e42129: f64 = (locals.var_cnst0over__blk932 * locals.var_t2__blk901);
        (assign29680_e42129, ((locals.var_cnst0over__blk932_dn0 * locals.var_t2__blk901) + (locals.var_cnst0over__blk932 * locals.var_t2__blk901_dn0)), ((locals.var_cnst0over__blk932_dn2 * locals.var_t2__blk901) + (locals.var_cnst0over__blk932 * locals.var_t2__blk901_dn2)), ((locals.var_cnst0over__blk932_dn6 * locals.var_t2__blk901) + (locals.var_cnst0over__blk932 * locals.var_t2__blk901_dn6)), ((locals.var_cnst0over__blk932_dn7 * locals.var_t2__blk901) + (locals.var_cnst0over__blk932 * locals.var_t2__blk901_dn7)), ((locals.var_cnst0over__blk932_dn10 * locals.var_t2__blk901) + (locals.var_cnst0over__blk932 * locals.var_t2__blk901_dn10)), ((locals.var_cnst0over__blk932_dn11 * locals.var_t2__blk901) + (locals.var_cnst0over__blk932 * locals.var_t2__blk901_dn11)), ((locals.var_cnst0over__blk932_dn12 * locals.var_t2__blk901) + (locals.var_cnst0over__blk932 * locals.var_t2__blk901_dn12)), ((locals.var_cnst0over__blk932_dn17 * locals.var_t2__blk901) + (locals.var_cnst0over__blk932 * locals.var_t2__blk901_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign29680_e42131;
        locals.var_qbuld_dn0 = assign29680_e42131_d_n0;
        locals.var_qbuld_dn2 = assign29680_e42131_d_n2;
        locals.var_qbuld_dn6 = assign29680_e42131_d_n6;
        locals.var_qbuld_dn7 = assign29680_e42131_d_n7;
        locals.var_qbuld_dn10 = assign29680_e42131_d_n10;
        locals.var_qbuld_dn11 = assign29680_e42131_d_n11;
        locals.var_qbuld_dn12 = assign29680_e42131_d_n12;
        locals.var_qbuld_dn17 = assign29680_e42131_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign29690_e42147, assign29690_e42147_d_n0, assign29690_e42147_d_n2, assign29690_e42147_d_n6, assign29690_e42147_d_n7, assign29690_e42147_d_n10, assign29690_e42147_d_n11, assign29690_e42147_d_n12, assign29690_e42147_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) {
        let assign29690_e42144: f64 = (locals.var_vgpld__blk935 - locals.var_ps0ld__blk949);
        let assign29690_e42145: f64 = (locals.var_cox0__blk910 * assign29690_e42144);
        (assign29690_e42145, (locals.var_cox0__blk910 * (locals.var_vgpld__blk935_dn0 - locals.var_ps0ld__blk949_dn0)), (locals.var_cox0__blk910 * (locals.var_vgpld__blk935_dn2 - locals.var_ps0ld__blk949_dn2)), (locals.var_cox0__blk910 * (locals.var_vgpld__blk935_dn6 - locals.var_ps0ld__blk949_dn6)), (locals.var_cox0__blk910 * (locals.var_vgpld__blk935_dn7 - locals.var_ps0ld__blk949_dn7)), (locals.var_cox0__blk910 * (locals.var_vgpld__blk935_dn10 - locals.var_ps0ld__blk949_dn10)), (locals.var_cox0__blk910 * (locals.var_vgpld__blk935_dn11 - locals.var_ps0ld__blk949_dn11)), (locals.var_cox0__blk910 * (locals.var_vgpld__blk935_dn12 - locals.var_ps0ld__blk949_dn12)), (locals.var_cox0__blk910 * (locals.var_vgpld__blk935_dn17 - locals.var_ps0ld__blk949_dn17)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign29690_e42147;
        locals.var_qsuld_dn0 = assign29690_e42147_d_n0;
        locals.var_qsuld_dn2 = assign29690_e42147_d_n2;
        locals.var_qsuld_dn6 = assign29690_e42147_d_n6;
        locals.var_qsuld_dn7 = assign29690_e42147_d_n7;
        locals.var_qsuld_dn10 = assign29690_e42147_d_n10;
        locals.var_qsuld_dn11 = assign29690_e42147_d_n11;
        locals.var_qsuld_dn12 = assign29690_e42147_d_n12;
        locals.var_qsuld_dn17 = assign29690_e42147_d_n17;
        locals.var_qsuld_rv = 0.0;

        let assign29700_e42150: f64 = if p.p41 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard993 = assign29700_e42150;
        locals.var_guard993_rv = 0.0;

        let (assign29710_e42168, assign29710_e42168_d_n0, assign29710_e42168_d_n2, assign29710_e42168_d_n6, assign29710_e42168_d_n7, assign29710_e42168_d_n10, assign29710_e42168_d_n11, assign29710_e42168_d_n12, assign29710_e42168_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        let assign29710_e42164: f64 = (-locals.var_vxbgmtcl__blk925);
        let assign29710_e42165: f64 = (locals.var_beta * assign29710_e42164);
        let assign29710_e42166: f64 = (assign29710_e42165).exp();
        (assign29710_e42166, (assign29710_e42166 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn0))), (assign29710_e42166 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn2))), (assign29710_e42166 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn6))), (assign29710_e42166 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn7))), (assign29710_e42166 * ((locals.var_beta_dn10 * assign29710_e42164) + (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn10)))), (assign29710_e42166 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn11))), (assign29710_e42166 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn12))), (assign29710_e42166 * (locals.var_beta * (-locals.var_vxbgmtcl__blk925_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk966, locals.var_exp_bvbs__blk966_dn0, locals.var_exp_bvbs__blk966_dn2, locals.var_exp_bvbs__blk966_dn6, locals.var_exp_bvbs__blk966_dn7, locals.var_exp_bvbs__blk966_dn10, locals.var_exp_bvbs__blk966_dn11, locals.var_exp_bvbs__blk966_dn12, locals.var_exp_bvbs__blk966_dn17,)
    }
};
        locals.var_exp_bvbs__blk966 = assign29710_e42168;
        locals.var_exp_bvbs__blk966_dn0 = assign29710_e42168_d_n0;
        locals.var_exp_bvbs__blk966_dn2 = assign29710_e42168_d_n2;
        locals.var_exp_bvbs__blk966_dn6 = assign29710_e42168_d_n6;
        locals.var_exp_bvbs__blk966_dn7 = assign29710_e42168_d_n7;
        locals.var_exp_bvbs__blk966_dn10 = assign29710_e42168_d_n10;
        locals.var_exp_bvbs__blk966_dn11 = assign29710_e42168_d_n11;
        locals.var_exp_bvbs__blk966_dn12 = assign29710_e42168_d_n12;
        locals.var_exp_bvbs__blk966_dn17 = assign29710_e42168_d_n17;
        locals.var_exp_bvbs__blk966_rv = 0.0;

        let (assign29720_e42184, assign29720_e42184_d_n0, assign29720_e42184_d_n2, assign29720_e42184_d_n6, assign29720_e42184_d_n7, assign29720_e42184_d_n10, assign29720_e42184_d_n11, assign29720_e42184_d_n12, assign29720_e42184_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        let assign29720_e42182: f64 = (locals.var_nin / locals.var_mks_nover);
        (assign29720_e42182, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn7 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover), (locals.var_nin_dn17 / locals.var_mks_nover),)
    } else {
        (locals.var_t0__blk899, locals.var_t0__blk899_dn0, locals.var_t0__blk899_dn2, locals.var_t0__blk899_dn6, locals.var_t0__blk899_dn7, locals.var_t0__blk899_dn10, locals.var_t0__blk899_dn11, locals.var_t0__blk899_dn12, locals.var_t0__blk899_dn17,)
    }
};
        locals.var_t0__blk899 = assign29720_e42184;
        locals.var_t0__blk899_dn0 = assign29720_e42184_d_n0;
        locals.var_t0__blk899_dn2 = assign29720_e42184_d_n2;
        locals.var_t0__blk899_dn6 = assign29720_e42184_d_n6;
        locals.var_t0__blk899_dn7 = assign29720_e42184_d_n7;
        locals.var_t0__blk899_dn10 = assign29720_e42184_d_n10;
        locals.var_t0__blk899_dn11 = assign29720_e42184_d_n11;
        locals.var_t0__blk899_dn12 = assign29720_e42184_d_n12;
        locals.var_t0__blk899_dn17 = assign29720_e42184_d_n17;
        locals.var_t0__blk899_rv = 0.0;

        let (assign29730_e42200, assign29730_e42200_d_n0, assign29730_e42200_d_n2, assign29730_e42200_d_n6, assign29730_e42200_d_n7, assign29730_e42200_d_n10, assign29730_e42200_d_n11, assign29730_e42200_d_n12, assign29730_e42200_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        let assign29730_e42198: f64 = (locals.var_t0__blk899 * locals.var_t0__blk899);
        (assign29730_e42198, ((locals.var_t0__blk899_dn0 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn0)), ((locals.var_t0__blk899_dn2 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn2)), ((locals.var_t0__blk899_dn6 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn6)), ((locals.var_t0__blk899_dn7 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn7)), ((locals.var_t0__blk899_dn10 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn10)), ((locals.var_t0__blk899_dn11 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn11)), ((locals.var_t0__blk899_dn12 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn12)), ((locals.var_t0__blk899_dn17 * locals.var_t0__blk899) + (locals.var_t0__blk899 * locals.var_t0__blk899_dn17)),)
    } else {
        (locals.var_cnst1over__blk960, locals.var_cnst1over__blk960_dn0, locals.var_cnst1over__blk960_dn2, locals.var_cnst1over__blk960_dn6, locals.var_cnst1over__blk960_dn7, locals.var_cnst1over__blk960_dn10, locals.var_cnst1over__blk960_dn11, locals.var_cnst1over__blk960_dn12, locals.var_cnst1over__blk960_dn17,)
    }
};
        locals.var_cnst1over__blk960 = assign29730_e42200;
        locals.var_cnst1over__blk960_dn0 = assign29730_e42200_d_n0;
        locals.var_cnst1over__blk960_dn2 = assign29730_e42200_d_n2;
        locals.var_cnst1over__blk960_dn6 = assign29730_e42200_d_n6;
        locals.var_cnst1over__blk960_dn7 = assign29730_e42200_d_n7;
        locals.var_cnst1over__blk960_dn10 = assign29730_e42200_d_n10;
        locals.var_cnst1over__blk960_dn11 = assign29730_e42200_d_n11;
        locals.var_cnst1over__blk960_dn12 = assign29730_e42200_d_n12;
        locals.var_cnst1over__blk960_dn17 = assign29730_e42200_d_n17;
        locals.var_cnst1over__blk960_rv = 0.0;

        let (assign29740_e42216, assign29740_e42216_d_n0, assign29740_e42216_d_n2, assign29740_e42216_d_n6, assign29740_e42216_d_n7, assign29740_e42216_d_n10, assign29740_e42216_d_n11, assign29740_e42216_d_n12, assign29740_e42216_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        let assign29740_e42214: f64 = (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966);
        (assign29740_e42214, ((locals.var_cnst1over__blk960_dn0 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn0)), ((locals.var_cnst1over__blk960_dn2 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn2)), ((locals.var_cnst1over__blk960_dn6 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn6)), ((locals.var_cnst1over__blk960_dn7 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn7)), ((locals.var_cnst1over__blk960_dn10 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn10)), ((locals.var_cnst1over__blk960_dn11 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn11)), ((locals.var_cnst1over__blk960_dn12 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn12)), ((locals.var_cnst1over__blk960_dn17 * locals.var_exp_bvbs__blk966) + (locals.var_cnst1over__blk960 * locals.var_exp_bvbs__blk966_dn17)),)
    } else {
        (locals.var_cfs1__blk975, locals.var_cfs1__blk975_dn0, locals.var_cfs1__blk975_dn2, locals.var_cfs1__blk975_dn6, locals.var_cfs1__blk975_dn7, locals.var_cfs1__blk975_dn10, locals.var_cfs1__blk975_dn11, locals.var_cfs1__blk975_dn12, locals.var_cfs1__blk975_dn17,)
    }
};
        locals.var_cfs1__blk975 = assign29740_e42216;
        locals.var_cfs1__blk975_dn0 = assign29740_e42216_d_n0;
        locals.var_cfs1__blk975_dn2 = assign29740_e42216_d_n2;
        locals.var_cfs1__blk975_dn6 = assign29740_e42216_d_n6;
        locals.var_cfs1__blk975_dn7 = assign29740_e42216_d_n7;
        locals.var_cfs1__blk975_dn10 = assign29740_e42216_d_n10;
        locals.var_cfs1__blk975_dn11 = assign29740_e42216_d_n11;
        locals.var_cfs1__blk975_dn12 = assign29740_e42216_d_n12;
        locals.var_cfs1__blk975_dn17 = assign29740_e42216_d_n17;
        locals.var_cfs1__blk975_rv = 0.0;

        let (assign29750_e42230,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv__blk922,)
    }
};
        locals.var_flg_conv__blk922 = assign29750_e42230;
        locals.var_flg_conv__blk922_rv = 0.0;

        let (assign29760_e42244, assign29760_e42244_d_n0, assign29760_e42244_d_n2, assign29760_e42244_d_n6, assign29760_e42244_d_n7, assign29760_e42244_d_n10, assign29760_e42244_d_n11, assign29760_e42244_d_n12, assign29760_e42244_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01__blk969, locals.var_fs01__blk969_dn0, locals.var_fs01__blk969_dn2, locals.var_fs01__blk969_dn6, locals.var_fs01__blk969_dn7, locals.var_fs01__blk969_dn10, locals.var_fs01__blk969_dn11, locals.var_fs01__blk969_dn12, locals.var_fs01__blk969_dn17,)
    }
};
        locals.var_fs01__blk969 = assign29760_e42244;
        locals.var_fs01__blk969_dn0 = assign29760_e42244_d_n0;
        locals.var_fs01__blk969_dn2 = assign29760_e42244_d_n2;
        locals.var_fs01__blk969_dn6 = assign29760_e42244_d_n6;
        locals.var_fs01__blk969_dn7 = assign29760_e42244_d_n7;
        locals.var_fs01__blk969_dn10 = assign29760_e42244_d_n10;
        locals.var_fs01__blk969_dn11 = assign29760_e42244_d_n11;
        locals.var_fs01__blk969_dn12 = assign29760_e42244_d_n12;
        locals.var_fs01__blk969_dn17 = assign29760_e42244_d_n17;
        locals.var_fs01__blk969_rv = 0.0;

        let (assign29770_e42258, assign29770_e42258_d_n0, assign29770_e42258_d_n2, assign29770_e42258_d_n6, assign29770_e42258_d_n7, assign29770_e42258_d_n10, assign29770_e42258_d_n11, assign29770_e42258_d_n12, assign29770_e42258_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02__blk973, locals.var_fs02__blk973_dn0, locals.var_fs02__blk973_dn2, locals.var_fs02__blk973_dn6, locals.var_fs02__blk973_dn7, locals.var_fs02__blk973_dn10, locals.var_fs02__blk973_dn11, locals.var_fs02__blk973_dn12, locals.var_fs02__blk973_dn17,)
    }
};
        locals.var_fs02__blk973 = assign29770_e42258;
        locals.var_fs02__blk973_dn0 = assign29770_e42258_d_n0;
        locals.var_fs02__blk973_dn2 = assign29770_e42258_d_n2;
        locals.var_fs02__blk973_dn6 = assign29770_e42258_d_n6;
        locals.var_fs02__blk973_dn7 = assign29770_e42258_d_n7;
        locals.var_fs02__blk973_dn10 = assign29770_e42258_d_n10;
        locals.var_fs02__blk973_dn11 = assign29770_e42258_d_n11;
        locals.var_fs02__blk973_dn12 = assign29770_e42258_d_n12;
        locals.var_fs02__blk973_dn17 = assign29770_e42258_d_n17;
        locals.var_fs02__blk973_rv = 0.0;

        let (assign29780_e42272,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign29780_e42272;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_108(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign29790_loop_guard: usize = 0;
        while {
            let assign29790_cond_e42287: f64 = (2.0 * 20.0);
            let assign29790_cond_e42289: f64 = (assign29790_cond_e42287 + 1.0);
            let assign29790_cond_e42291: f64 = if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_lp_s0 <= assign29790_cond_e42289)) { 1.0 } else { 0.0 };
            assign29790_cond_e42291 != 0.0
        } {
            assign29790_loop_guard += 1;
            assert!(assign29790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign29790_body0_e42305, assign29790_body0_e42305_d_n0, assign29790_body0_e42305_d_n2, assign29790_body0_e42305_d_n6, assign29790_body0_e42305_d_n7, assign29790_body0_e42305_d_n10, assign29790_body0_e42305_d_n11, assign29790_body0_e42305_d_n12, assign29790_body0_e42305_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb__blk971, locals.var_fb__blk971_dn0, locals.var_fb__blk971_dn2, locals.var_fb__blk971_dn6, locals.var_fb__blk971_dn7, locals.var_fb__blk971_dn10, locals.var_fb__blk971_dn11, locals.var_fb__blk971_dn12, locals.var_fb__blk971_dn17,)
    }
};
            locals.var_fb__blk971 = assign29790_body0_e42305;
            locals.var_fb__blk971_dn0 = assign29790_body0_e42305_d_n0;
            locals.var_fb__blk971_dn2 = assign29790_body0_e42305_d_n2;
            locals.var_fb__blk971_dn6 = assign29790_body0_e42305_d_n6;
            locals.var_fb__blk971_dn7 = assign29790_body0_e42305_d_n7;
            locals.var_fb__blk971_dn10 = assign29790_body0_e42305_d_n10;
            locals.var_fb__blk971_dn11 = assign29790_body0_e42305_d_n11;
            locals.var_fb__blk971_dn12 = assign29790_body0_e42305_d_n12;
            locals.var_fb__blk971_dn17 = assign29790_body0_e42305_d_n17;
            locals.var_fb__blk971_rv = 0.0;
            let (assign29790_body1_e42323, assign29790_body1_e42323_d_n0, assign29790_body1_e42323_d_n2, assign29790_body1_e42323_d_n6, assign29790_body1_e42323_d_n7, assign29790_body1_e42323_d_n10, assign29790_body1_e42323_d_n11, assign29790_body1_e42323_d_n12, assign29790_body1_e42323_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        let assign29790_body1_e42320: f64 = (locals.var_ps0ld__blk949 + locals.var_vxbgmtcl__blk925);
        let assign29790_body1_e42321: f64 = (locals.var_beta * assign29790_body1_e42320);
        (assign29790_body1_e42321, (locals.var_beta * (locals.var_ps0ld__blk949_dn0 + locals.var_vxbgmtcl__blk925_dn0)), (locals.var_beta * (locals.var_ps0ld__blk949_dn2 + locals.var_vxbgmtcl__blk925_dn2)), (locals.var_beta * (locals.var_ps0ld__blk949_dn6 + locals.var_vxbgmtcl__blk925_dn6)), (locals.var_beta * (locals.var_ps0ld__blk949_dn7 + locals.var_vxbgmtcl__blk925_dn7)), ((locals.var_beta_dn10 * assign29790_body1_e42320) + (locals.var_beta * (locals.var_ps0ld__blk949_dn10 + locals.var_vxbgmtcl__blk925_dn10))), (locals.var_beta * (locals.var_ps0ld__blk949_dn11 + locals.var_vxbgmtcl__blk925_dn11)), (locals.var_beta * (locals.var_ps0ld__blk949_dn12 + locals.var_vxbgmtcl__blk925_dn12)), (locals.var_beta * (locals.var_ps0ld__blk949_dn17 + locals.var_vxbgmtcl__blk925_dn17)),)
    } else {
        (locals.var_chi__blk947, locals.var_chi__blk947_dn0, locals.var_chi__blk947_dn2, locals.var_chi__blk947_dn6, locals.var_chi__blk947_dn7, locals.var_chi__blk947_dn10, locals.var_chi__blk947_dn11, locals.var_chi__blk947_dn12, locals.var_chi__blk947_dn17,)
    }
};
            locals.var_chi__blk947 = assign29790_body1_e42323;
            locals.var_chi__blk947_dn0 = assign29790_body1_e42323_d_n0;
            locals.var_chi__blk947_dn2 = assign29790_body1_e42323_d_n2;
            locals.var_chi__blk947_dn6 = assign29790_body1_e42323_d_n6;
            locals.var_chi__blk947_dn7 = assign29790_body1_e42323_d_n7;
            locals.var_chi__blk947_dn10 = assign29790_body1_e42323_d_n10;
            locals.var_chi__blk947_dn11 = assign29790_body1_e42323_d_n11;
            locals.var_chi__blk947_dn12 = assign29790_body1_e42323_d_n12;
            locals.var_chi__blk947_dn17 = assign29790_body1_e42323_d_n17;
            locals.var_chi__blk947_rv = 0.0;
            let assign29790_body2_e42326: f64 = if locals.var_chi__blk947 < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard994 = assign29790_body2_e42326;
            locals.var_guard994_rv = 0.0;
            let (assign29790_body3_e42357, assign29790_body3_e42357_d_n0, assign29790_body3_e42357_d_n2, assign29790_body3_e42357_d_n6, assign29790_body3_e42357_d_n7, assign29790_body3_e42357_d_n10, assign29790_body3_e42357_d_n11, assign29790_body3_e42357_d_n12, assign29790_body3_e42357_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard994 != 0.0)) {
        let assign29790_body3_e42342: f64 = (locals.var_chi__blk947 * locals.var_chi__blk947);
        let assign29790_body3_e42344: f64 = (assign29790_body3_e42342 * locals.var_chi__blk947);
        let assign29790_body3_e42348: f64 = (-0.07053654284009761);
        let assign29790_body3_e42351: f64 = (locals.var_chi__blk947 * 0.006115288895133179);
        let assign29790_body3_e42352: f64 = (assign29790_body3_e42348 + assign29790_body3_e42351);
        let assign29790_body3_e42353: f64 = (locals.var_chi__blk947 * assign29790_body3_e42352);
        let assign29790_body3_e42354: f64 = (0.29693154855771 + assign29790_body3_e42353);
        let assign29790_body3_e42355: f64 = (assign29790_body3_e42344 * assign29790_body3_e42354);
        (assign29790_body3_e42355, ((((((locals.var_chi__blk947_dn0 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn0)) * locals.var_chi__blk947) + (assign29790_body3_e42342 * locals.var_chi__blk947_dn0)) * assign29790_body3_e42354) + (assign29790_body3_e42344 * ((locals.var_chi__blk947_dn0 * assign29790_body3_e42352) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn0 * 0.006115288895133179))))), ((((((locals.var_chi__blk947_dn2 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn2)) * locals.var_chi__blk947) + (assign29790_body3_e42342 * locals.var_chi__blk947_dn2)) * assign29790_body3_e42354) + (assign29790_body3_e42344 * ((locals.var_chi__blk947_dn2 * assign29790_body3_e42352) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn2 * 0.006115288895133179))))), ((((((locals.var_chi__blk947_dn6 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn6)) * locals.var_chi__blk947) + (assign29790_body3_e42342 * locals.var_chi__blk947_dn6)) * assign29790_body3_e42354) + (assign29790_body3_e42344 * ((locals.var_chi__blk947_dn6 * assign29790_body3_e42352) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn6 * 0.006115288895133179))))), ((((((locals.var_chi__blk947_dn7 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn7)) * locals.var_chi__blk947) + (assign29790_body3_e42342 * locals.var_chi__blk947_dn7)) * assign29790_body3_e42354) + (assign29790_body3_e42344 * ((locals.var_chi__blk947_dn7 * assign29790_body3_e42352) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn7 * 0.006115288895133179))))), ((((((locals.var_chi__blk947_dn10 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn10)) * locals.var_chi__blk947) + (assign29790_body3_e42342 * locals.var_chi__blk947_dn10)) * assign29790_body3_e42354) + (assign29790_body3_e42344 * ((locals.var_chi__blk947_dn10 * assign29790_body3_e42352) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn10 * 0.006115288895133179))))), ((((((locals.var_chi__blk947_dn11 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn11)) * locals.var_chi__blk947) + (assign29790_body3_e42342 * locals.var_chi__blk947_dn11)) * assign29790_body3_e42354) + (assign29790_body3_e42344 * ((locals.var_chi__blk947_dn11 * assign29790_body3_e42352) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn11 * 0.006115288895133179))))), ((((((locals.var_chi__blk947_dn12 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn12)) * locals.var_chi__blk947) + (assign29790_body3_e42342 * locals.var_chi__blk947_dn12)) * assign29790_body3_e42354) + (assign29790_body3_e42344 * ((locals.var_chi__blk947_dn12 * assign29790_body3_e42352) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn12 * 0.006115288895133179))))), ((((((locals.var_chi__blk947_dn17 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn17)) * locals.var_chi__blk947) + (assign29790_body3_e42342 * locals.var_chi__blk947_dn17)) * assign29790_body3_e42354) + (assign29790_body3_e42344 * ((locals.var_chi__blk947_dn17 * assign29790_body3_e42352) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn17 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi__blk967, locals.var_fi__blk967_dn0, locals.var_fi__blk967_dn2, locals.var_fi__blk967_dn6, locals.var_fi__blk967_dn7, locals.var_fi__blk967_dn10, locals.var_fi__blk967_dn11, locals.var_fi__blk967_dn12, locals.var_fi__blk967_dn17,)
    }
};
            locals.var_fi__blk967 = assign29790_body3_e42357;
            locals.var_fi__blk967_dn0 = assign29790_body3_e42357_d_n0;
            locals.var_fi__blk967_dn2 = assign29790_body3_e42357_d_n2;
            locals.var_fi__blk967_dn6 = assign29790_body3_e42357_d_n6;
            locals.var_fi__blk967_dn7 = assign29790_body3_e42357_d_n7;
            locals.var_fi__blk967_dn10 = assign29790_body3_e42357_d_n10;
            locals.var_fi__blk967_dn11 = assign29790_body3_e42357_d_n11;
            locals.var_fi__blk967_dn12 = assign29790_body3_e42357_d_n12;
            locals.var_fi__blk967_dn17 = assign29790_body3_e42357_d_n17;
            locals.var_fi__blk967_rv = 0.0;
            let (assign29790_body4_e42392, assign29790_body4_e42392_d_n0, assign29790_body4_e42392_d_n2, assign29790_body4_e42392_d_n6, assign29790_body4_e42392_d_n7, assign29790_body4_e42392_d_n10, assign29790_body4_e42392_d_n11, assign29790_body4_e42392_d_n12, assign29790_body4_e42392_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard994 != 0.0)) {
        let assign29790_body4_e42373: f64 = (locals.var_chi__blk947 * locals.var_chi__blk947);
        let assign29790_body4_e42376: f64 = (3.0 * 0.29693154855771);
        let assign29790_body4_e42380: f64 = (-0.07053654284009761);
        let assign29790_body4_e42381: f64 = (4.0 * assign29790_body4_e42380);
        let assign29790_body4_e42384: f64 = (locals.var_chi__blk947 * 5.0);
        let assign29790_body4_e42386: f64 = (assign29790_body4_e42384 * 0.006115288895133179);
        let assign29790_body4_e42387: f64 = (assign29790_body4_e42381 + assign29790_body4_e42386);
        let assign29790_body4_e42388: f64 = (locals.var_chi__blk947 * assign29790_body4_e42387);
        let assign29790_body4_e42389: f64 = (assign29790_body4_e42376 + assign29790_body4_e42388);
        let assign29790_body4_e42390: f64 = (assign29790_body4_e42373 * assign29790_body4_e42389);
        (assign29790_body4_e42390, ((((locals.var_chi__blk947_dn0 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn0)) * assign29790_body4_e42389) + (assign29790_body4_e42373 * ((locals.var_chi__blk947_dn0 * assign29790_body4_e42387) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk947_dn2 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn2)) * assign29790_body4_e42389) + (assign29790_body4_e42373 * ((locals.var_chi__blk947_dn2 * assign29790_body4_e42387) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk947_dn6 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn6)) * assign29790_body4_e42389) + (assign29790_body4_e42373 * ((locals.var_chi__blk947_dn6 * assign29790_body4_e42387) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk947_dn7 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn7)) * assign29790_body4_e42389) + (assign29790_body4_e42373 * ((locals.var_chi__blk947_dn7 * assign29790_body4_e42387) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk947_dn10 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn10)) * assign29790_body4_e42389) + (assign29790_body4_e42373 * ((locals.var_chi__blk947_dn10 * assign29790_body4_e42387) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk947_dn11 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn11)) * assign29790_body4_e42389) + (assign29790_body4_e42373 * ((locals.var_chi__blk947_dn11 * assign29790_body4_e42387) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk947_dn12 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn12)) * assign29790_body4_e42389) + (assign29790_body4_e42373 * ((locals.var_chi__blk947_dn12 * assign29790_body4_e42387) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn12 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk947_dn17 * locals.var_chi__blk947) + (locals.var_chi__blk947 * locals.var_chi__blk947_dn17)) * assign29790_body4_e42389) + (assign29790_body4_e42373 * ((locals.var_chi__blk947_dn17 * assign29790_body4_e42387) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn17 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi__blk968, locals.var_fi_dchi__blk968_dn0, locals.var_fi_dchi__blk968_dn2, locals.var_fi_dchi__blk968_dn6, locals.var_fi_dchi__blk968_dn7, locals.var_fi_dchi__blk968_dn10, locals.var_fi_dchi__blk968_dn11, locals.var_fi_dchi__blk968_dn12, locals.var_fi_dchi__blk968_dn17,)
    }
};
            locals.var_fi_dchi__blk968 = assign29790_body4_e42392;
            locals.var_fi_dchi__blk968_dn0 = assign29790_body4_e42392_d_n0;
            locals.var_fi_dchi__blk968_dn2 = assign29790_body4_e42392_d_n2;
            locals.var_fi_dchi__blk968_dn6 = assign29790_body4_e42392_d_n6;
            locals.var_fi_dchi__blk968_dn7 = assign29790_body4_e42392_d_n7;
            locals.var_fi_dchi__blk968_dn10 = assign29790_body4_e42392_d_n10;
            locals.var_fi_dchi__blk968_dn11 = assign29790_body4_e42392_d_n11;
            locals.var_fi_dchi__blk968_dn12 = assign29790_body4_e42392_d_n12;
            locals.var_fi_dchi__blk968_dn17 = assign29790_body4_e42392_d_n17;
            locals.var_fi_dchi__blk968_rv = 0.0;
            let (assign29790_body5_e42412, assign29790_body5_e42412_d_n0, assign29790_body5_e42412_d_n2, assign29790_body5_e42412_d_n6, assign29790_body5_e42412_d_n7, assign29790_body5_e42412_d_n10, assign29790_body5_e42412_d_n11, assign29790_body5_e42412_d_n12, assign29790_body5_e42412_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard994 != 0.0)) {
        let assign29790_body5_e42408: f64 = (locals.var_cfs1__blk975 * locals.var_fi__blk967);
        let assign29790_body5_e42410: f64 = (assign29790_body5_e42408 * locals.var_fi__blk967);
        (assign29790_body5_e42410, ((((locals.var_cfs1__blk975_dn0 * locals.var_fi__blk967) + (locals.var_cfs1__blk975 * locals.var_fi__blk967_dn0)) * locals.var_fi__blk967) + (assign29790_body5_e42408 * locals.var_fi__blk967_dn0)), ((((locals.var_cfs1__blk975_dn2 * locals.var_fi__blk967) + (locals.var_cfs1__blk975 * locals.var_fi__blk967_dn2)) * locals.var_fi__blk967) + (assign29790_body5_e42408 * locals.var_fi__blk967_dn2)), ((((locals.var_cfs1__blk975_dn6 * locals.var_fi__blk967) + (locals.var_cfs1__blk975 * locals.var_fi__blk967_dn6)) * locals.var_fi__blk967) + (assign29790_body5_e42408 * locals.var_fi__blk967_dn6)), ((((locals.var_cfs1__blk975_dn7 * locals.var_fi__blk967) + (locals.var_cfs1__blk975 * locals.var_fi__blk967_dn7)) * locals.var_fi__blk967) + (assign29790_body5_e42408 * locals.var_fi__blk967_dn7)), ((((locals.var_cfs1__blk975_dn10 * locals.var_fi__blk967) + (locals.var_cfs1__blk975 * locals.var_fi__blk967_dn10)) * locals.var_fi__blk967) + (assign29790_body5_e42408 * locals.var_fi__blk967_dn10)), ((((locals.var_cfs1__blk975_dn11 * locals.var_fi__blk967) + (locals.var_cfs1__blk975 * locals.var_fi__blk967_dn11)) * locals.var_fi__blk967) + (assign29790_body5_e42408 * locals.var_fi__blk967_dn11)), ((((locals.var_cfs1__blk975_dn12 * locals.var_fi__blk967) + (locals.var_cfs1__blk975 * locals.var_fi__blk967_dn12)) * locals.var_fi__blk967) + (assign29790_body5_e42408 * locals.var_fi__blk967_dn12)), ((((locals.var_cfs1__blk975_dn17 * locals.var_fi__blk967) + (locals.var_cfs1__blk975 * locals.var_fi__blk967_dn17)) * locals.var_fi__blk967) + (assign29790_body5_e42408 * locals.var_fi__blk967_dn17)),)
    } else {
        (locals.var_fs01__blk969, locals.var_fs01__blk969_dn0, locals.var_fs01__blk969_dn2, locals.var_fs01__blk969_dn6, locals.var_fs01__blk969_dn7, locals.var_fs01__blk969_dn10, locals.var_fs01__blk969_dn11, locals.var_fs01__blk969_dn12, locals.var_fs01__blk969_dn17,)
    }
};
            locals.var_fs01__blk969 = assign29790_body5_e42412;
            locals.var_fs01__blk969_dn0 = assign29790_body5_e42412_d_n0;
            locals.var_fs01__blk969_dn2 = assign29790_body5_e42412_d_n2;
            locals.var_fs01__blk969_dn6 = assign29790_body5_e42412_d_n6;
            locals.var_fs01__blk969_dn7 = assign29790_body5_e42412_d_n7;
            locals.var_fs01__blk969_dn10 = assign29790_body5_e42412_d_n10;
            locals.var_fs01__blk969_dn11 = assign29790_body5_e42412_d_n11;
            locals.var_fs01__blk969_dn12 = assign29790_body5_e42412_d_n12;
            locals.var_fs01__blk969_dn17 = assign29790_body5_e42412_d_n17;
            locals.var_fs01__blk969_rv = 0.0;
            let (assign29790_body6_e42436, assign29790_body6_e42436_d_n0, assign29790_body6_e42436_d_n2, assign29790_body6_e42436_d_n6, assign29790_body6_e42436_d_n7, assign29790_body6_e42436_d_n10, assign29790_body6_e42436_d_n11, assign29790_body6_e42436_d_n12, assign29790_body6_e42436_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard994 != 0.0)) {
        let assign29790_body6_e42428: f64 = (locals.var_cfs1__blk975 * locals.var_beta);
        let assign29790_body6_e42430: f64 = (assign29790_body6_e42428 * 2.0);
        let assign29790_body6_e42432: f64 = (assign29790_body6_e42430 * locals.var_fi__blk967);
        let assign29790_body6_e42434: f64 = (assign29790_body6_e42432 * locals.var_fi_dchi__blk968);
        (assign29790_body6_e42434, ((((((locals.var_cfs1__blk975_dn0 * locals.var_beta) * 2.0) * locals.var_fi__blk967) + (assign29790_body6_e42430 * locals.var_fi__blk967_dn0)) * locals.var_fi_dchi__blk968) + (assign29790_body6_e42432 * locals.var_fi_dchi__blk968_dn0)), ((((((locals.var_cfs1__blk975_dn2 * locals.var_beta) * 2.0) * locals.var_fi__blk967) + (assign29790_body6_e42430 * locals.var_fi__blk967_dn2)) * locals.var_fi_dchi__blk968) + (assign29790_body6_e42432 * locals.var_fi_dchi__blk968_dn2)), ((((((locals.var_cfs1__blk975_dn6 * locals.var_beta) * 2.0) * locals.var_fi__blk967) + (assign29790_body6_e42430 * locals.var_fi__blk967_dn6)) * locals.var_fi_dchi__blk968) + (assign29790_body6_e42432 * locals.var_fi_dchi__blk968_dn6)), ((((((locals.var_cfs1__blk975_dn7 * locals.var_beta) * 2.0) * locals.var_fi__blk967) + (assign29790_body6_e42430 * locals.var_fi__blk967_dn7)) * locals.var_fi_dchi__blk968) + (assign29790_body6_e42432 * locals.var_fi_dchi__blk968_dn7)), (((((((locals.var_cfs1__blk975_dn10 * locals.var_beta) + (locals.var_cfs1__blk975 * locals.var_beta_dn10)) * 2.0) * locals.var_fi__blk967) + (assign29790_body6_e42430 * locals.var_fi__blk967_dn10)) * locals.var_fi_dchi__blk968) + (assign29790_body6_e42432 * locals.var_fi_dchi__blk968_dn10)), ((((((locals.var_cfs1__blk975_dn11 * locals.var_beta) * 2.0) * locals.var_fi__blk967) + (assign29790_body6_e42430 * locals.var_fi__blk967_dn11)) * locals.var_fi_dchi__blk968) + (assign29790_body6_e42432 * locals.var_fi_dchi__blk968_dn11)), ((((((locals.var_cfs1__blk975_dn12 * locals.var_beta) * 2.0) * locals.var_fi__blk967) + (assign29790_body6_e42430 * locals.var_fi__blk967_dn12)) * locals.var_fi_dchi__blk968) + (assign29790_body6_e42432 * locals.var_fi_dchi__blk968_dn12)), ((((((locals.var_cfs1__blk975_dn17 * locals.var_beta) * 2.0) * locals.var_fi__blk967) + (assign29790_body6_e42430 * locals.var_fi__blk967_dn17)) * locals.var_fi_dchi__blk968) + (assign29790_body6_e42432 * locals.var_fi_dchi__blk968_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk970, locals.var_fs01_dps0__blk970_dn0, locals.var_fs01_dps0__blk970_dn2, locals.var_fs01_dps0__blk970_dn6, locals.var_fs01_dps0__blk970_dn7, locals.var_fs01_dps0__blk970_dn10, locals.var_fs01_dps0__blk970_dn11, locals.var_fs01_dps0__blk970_dn12, locals.var_fs01_dps0__blk970_dn17,)
    }
};
            locals.var_fs01_dps0__blk970 = assign29790_body6_e42436;
            locals.var_fs01_dps0__blk970_dn0 = assign29790_body6_e42436_d_n0;
            locals.var_fs01_dps0__blk970_dn2 = assign29790_body6_e42436_d_n2;
            locals.var_fs01_dps0__blk970_dn6 = assign29790_body6_e42436_d_n6;
            locals.var_fs01_dps0__blk970_dn7 = assign29790_body6_e42436_d_n7;
            locals.var_fs01_dps0__blk970_dn10 = assign29790_body6_e42436_d_n10;
            locals.var_fs01_dps0__blk970_dn11 = assign29790_body6_e42436_d_n11;
            locals.var_fs01_dps0__blk970_dn12 = assign29790_body6_e42436_d_n12;
            locals.var_fs01_dps0__blk970_dn17 = assign29790_body6_e42436_d_n17;
            locals.var_fs01_dps0__blk970_rv = 0.0;
            let (assign29790_body7_e42472, assign29790_body7_e42472_d_n0, assign29790_body7_e42472_d_n2, assign29790_body7_e42472_d_n6, assign29790_body7_e42472_d_n7, assign29790_body7_e42472_d_n10, assign29790_body7_e42472_d_n11, assign29790_body7_e42472_d_n12, assign29790_body7_e42472_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard994 != 0.0)) {
        let assign29790_body7_e42454: f64 = (-0.117851130197758);
        let assign29790_body7_e42459: f64 = (-0.00163730162779191);
        let assign29790_body7_e42462: f64 = (locals.var_chi__blk947 * 6.36964918866352e-5);
        let assign29790_body7_e42463: f64 = (assign29790_body7_e42459 + assign29790_body7_e42462);
        let assign29790_body7_e42464: f64 = (locals.var_chi__blk947 * assign29790_body7_e42463);
        let assign29790_body7_e42465: f64 = (0.0178800506338833 + assign29790_body7_e42464);
        let assign29790_body7_e42466: f64 = (locals.var_chi__blk947 * assign29790_body7_e42465);
        let assign29790_body7_e42467: f64 = (assign29790_body7_e42454 + assign29790_body7_e42466);
        let assign29790_body7_e42468: f64 = (locals.var_chi__blk947 * assign29790_body7_e42467);
        let assign29790_body7_e42469: f64 = (0.707106781186548 + assign29790_body7_e42468);
        let assign29790_body7_e42470: f64 = (locals.var_chi__blk947 * assign29790_body7_e42469);
        (assign29790_body7_e42470, ((locals.var_chi__blk947_dn0 * assign29790_body7_e42469) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn0 * assign29790_body7_e42467) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn0 * assign29790_body7_e42465) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn0 * assign29790_body7_e42463) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk947_dn2 * assign29790_body7_e42469) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn2 * assign29790_body7_e42467) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn2 * assign29790_body7_e42465) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn2 * assign29790_body7_e42463) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk947_dn6 * assign29790_body7_e42469) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn6 * assign29790_body7_e42467) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn6 * assign29790_body7_e42465) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn6 * assign29790_body7_e42463) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk947_dn7 * assign29790_body7_e42469) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn7 * assign29790_body7_e42467) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn7 * assign29790_body7_e42465) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn7 * assign29790_body7_e42463) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk947_dn10 * assign29790_body7_e42469) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn10 * assign29790_body7_e42467) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn10 * assign29790_body7_e42465) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn10 * assign29790_body7_e42463) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk947_dn11 * assign29790_body7_e42469) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn11 * assign29790_body7_e42467) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn11 * assign29790_body7_e42465) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn11 * assign29790_body7_e42463) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk947_dn12 * assign29790_body7_e42469) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn12 * assign29790_body7_e42467) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn12 * assign29790_body7_e42465) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn12 * assign29790_body7_e42463) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn12 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk947_dn17 * assign29790_body7_e42469) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn17 * assign29790_body7_e42467) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn17 * assign29790_body7_e42465) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn17 * assign29790_body7_e42463) + (locals.var_chi__blk947 * (locals.var_chi__blk947_dn17 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb__blk971, locals.var_fb__blk971_dn0, locals.var_fb__blk971_dn2, locals.var_fb__blk971_dn6, locals.var_fb__blk971_dn7, locals.var_fb__blk971_dn10, locals.var_fb__blk971_dn11, locals.var_fb__blk971_dn12, locals.var_fb__blk971_dn17,)
    }
};
            locals.var_fb__blk971 = assign29790_body7_e42472;
            locals.var_fb__blk971_dn0 = assign29790_body7_e42472_d_n0;
            locals.var_fb__blk971_dn2 = assign29790_body7_e42472_d_n2;
            locals.var_fb__blk971_dn6 = assign29790_body7_e42472_d_n6;
            locals.var_fb__blk971_dn7 = assign29790_body7_e42472_d_n7;
            locals.var_fb__blk971_dn10 = assign29790_body7_e42472_d_n10;
            locals.var_fb__blk971_dn11 = assign29790_body7_e42472_d_n11;
            locals.var_fb__blk971_dn12 = assign29790_body7_e42472_d_n12;
            locals.var_fb__blk971_dn17 = assign29790_body7_e42472_d_n17;
            locals.var_fb__blk971_rv = 0.0;
            let (assign29790_body8_e42514, assign29790_body8_e42514_d_n0, assign29790_body8_e42514_d_n2, assign29790_body8_e42514_d_n6, assign29790_body8_e42514_d_n7, assign29790_body8_e42514_d_n10, assign29790_body8_e42514_d_n11, assign29790_body8_e42514_d_n12, assign29790_body8_e42514_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard994 != 0.0)) {
        let assign29790_body8_e42490: f64 = (-0.117851130197758);
        let assign29790_body8_e42491: f64 = (2.0 * assign29790_body8_e42490);
        let assign29790_body8_e42495: f64 = (3.0 * 0.0178800506338833);
        let assign29790_body8_e42499: f64 = (-0.00163730162779191);
        let assign29790_body8_e42500: f64 = (4.0 * assign29790_body8_e42499);
        let assign29790_body8_e42503: f64 = (locals.var_chi__blk947 * 5.0);
        let assign29790_body8_e42505: f64 = (assign29790_body8_e42503 * 6.36964918866352e-5);
        let assign29790_body8_e42506: f64 = (assign29790_body8_e42500 + assign29790_body8_e42505);
        let assign29790_body8_e42507: f64 = (locals.var_chi__blk947 * assign29790_body8_e42506);
        let assign29790_body8_e42508: f64 = (assign29790_body8_e42495 + assign29790_body8_e42507);
        let assign29790_body8_e42509: f64 = (locals.var_chi__blk947 * assign29790_body8_e42508);
        let assign29790_body8_e42510: f64 = (assign29790_body8_e42491 + assign29790_body8_e42509);
        let assign29790_body8_e42511: f64 = (locals.var_chi__blk947 * assign29790_body8_e42510);
        let assign29790_body8_e42512: f64 = (0.707106781186548 + assign29790_body8_e42511);
        (assign29790_body8_e42512, ((locals.var_chi__blk947_dn0 * assign29790_body8_e42510) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn0 * assign29790_body8_e42508) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn0 * assign29790_body8_e42506) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk947_dn2 * assign29790_body8_e42510) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn2 * assign29790_body8_e42508) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn2 * assign29790_body8_e42506) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk947_dn6 * assign29790_body8_e42510) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn6 * assign29790_body8_e42508) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn6 * assign29790_body8_e42506) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk947_dn7 * assign29790_body8_e42510) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn7 * assign29790_body8_e42508) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn7 * assign29790_body8_e42506) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk947_dn10 * assign29790_body8_e42510) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn10 * assign29790_body8_e42508) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn10 * assign29790_body8_e42506) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk947_dn11 * assign29790_body8_e42510) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn11 * assign29790_body8_e42508) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn11 * assign29790_body8_e42506) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk947_dn12 * assign29790_body8_e42510) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn12 * assign29790_body8_e42508) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn12 * assign29790_body8_e42506) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn12 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk947_dn17 * assign29790_body8_e42510) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn17 * assign29790_body8_e42508) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn17 * assign29790_body8_e42506) + (locals.var_chi__blk947 * ((locals.var_chi__blk947_dn17 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi__blk972, locals.var_fb_dchi__blk972_dn0, locals.var_fb_dchi__blk972_dn2, locals.var_fb_dchi__blk972_dn6, locals.var_fb_dchi__blk972_dn7, locals.var_fb_dchi__blk972_dn10, locals.var_fb_dchi__blk972_dn11, locals.var_fb_dchi__blk972_dn12, locals.var_fb_dchi__blk972_dn17,)
    }
};
            locals.var_fb_dchi__blk972 = assign29790_body8_e42514;
            locals.var_fb_dchi__blk972_dn0 = assign29790_body8_e42514_d_n0;
            locals.var_fb_dchi__blk972_dn2 = assign29790_body8_e42514_d_n2;
            locals.var_fb_dchi__blk972_dn6 = assign29790_body8_e42514_d_n6;
            locals.var_fb_dchi__blk972_dn7 = assign29790_body8_e42514_d_n7;
            locals.var_fb_dchi__blk972_dn10 = assign29790_body8_e42514_d_n10;
            locals.var_fb_dchi__blk972_dn11 = assign29790_body8_e42514_d_n11;
            locals.var_fb_dchi__blk972_dn12 = assign29790_body8_e42514_d_n12;
            locals.var_fb_dchi__blk972_dn17 = assign29790_body8_e42514_d_n17;
            locals.var_fb_dchi__blk972_rv = 0.0;
            let (assign29790_body9_e42537, assign29790_body9_e42537_d_n0, assign29790_body9_e42537_d_n2, assign29790_body9_e42537_d_n6, assign29790_body9_e42537_d_n7, assign29790_body9_e42537_d_n10, assign29790_body9_e42537_d_n11, assign29790_body9_e42537_d_n12, assign29790_body9_e42537_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard994 != 0.0)) {
        let assign29790_body9_e42530: f64 = (locals.var_fb__blk971 * locals.var_fb__blk971);
        let assign29790_body9_e42532: f64 = (assign29790_body9_e42530 + locals.var_fs01__blk969);
        let assign29790_body9_e42534: f64 = (assign29790_body9_e42532 + 1e-50);
        let assign29790_body9_e42535: f64 = (assign29790_body9_e42534).sqrt();
        (assign29790_body9_e42535, ((((locals.var_fb__blk971_dn0 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn0)) + locals.var_fs01__blk969_dn0) / (2.0 * assign29790_body9_e42535)), ((((locals.var_fb__blk971_dn2 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn2)) + locals.var_fs01__blk969_dn2) / (2.0 * assign29790_body9_e42535)), ((((locals.var_fb__blk971_dn6 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn6)) + locals.var_fs01__blk969_dn6) / (2.0 * assign29790_body9_e42535)), ((((locals.var_fb__blk971_dn7 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn7)) + locals.var_fs01__blk969_dn7) / (2.0 * assign29790_body9_e42535)), ((((locals.var_fb__blk971_dn10 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn10)) + locals.var_fs01__blk969_dn10) / (2.0 * assign29790_body9_e42535)), ((((locals.var_fb__blk971_dn11 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn11)) + locals.var_fs01__blk969_dn11) / (2.0 * assign29790_body9_e42535)), ((((locals.var_fb__blk971_dn12 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn12)) + locals.var_fs01__blk969_dn12) / (2.0 * assign29790_body9_e42535)), ((((locals.var_fb__blk971_dn17 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn17)) + locals.var_fs01__blk969_dn17) / (2.0 * assign29790_body9_e42535)),)
    } else {
        (locals.var_fs02__blk973, locals.var_fs02__blk973_dn0, locals.var_fs02__blk973_dn2, locals.var_fs02__blk973_dn6, locals.var_fs02__blk973_dn7, locals.var_fs02__blk973_dn10, locals.var_fs02__blk973_dn11, locals.var_fs02__blk973_dn12, locals.var_fs02__blk973_dn17,)
    }
};
            locals.var_fs02__blk973 = assign29790_body9_e42537;
            locals.var_fs02__blk973_dn0 = assign29790_body9_e42537_d_n0;
            locals.var_fs02__blk973_dn2 = assign29790_body9_e42537_d_n2;
            locals.var_fs02__blk973_dn6 = assign29790_body9_e42537_d_n6;
            locals.var_fs02__blk973_dn7 = assign29790_body9_e42537_d_n7;
            locals.var_fs02__blk973_dn10 = assign29790_body9_e42537_d_n10;
            locals.var_fs02__blk973_dn11 = assign29790_body9_e42537_d_n11;
            locals.var_fs02__blk973_dn12 = assign29790_body9_e42537_d_n12;
            locals.var_fs02__blk973_dn17 = assign29790_body9_e42537_d_n17;
            locals.var_fs02__blk973_rv = 0.0;
            let (assign29790_body10_e42565, assign29790_body10_e42565_d_n0, assign29790_body10_e42565_d_n2, assign29790_body10_e42565_d_n6, assign29790_body10_e42565_d_n7, assign29790_body10_e42565_d_n10, assign29790_body10_e42565_d_n11, assign29790_body10_e42565_d_n12, assign29790_body10_e42565_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard994 != 0.0)) {
        let assign29790_body10_e42553: f64 = (locals.var_beta * locals.var_fb_dchi__blk972);
        let assign29790_body10_e42555: f64 = (assign29790_body10_e42553 * 2.0);
        let assign29790_body10_e42557: f64 = (assign29790_body10_e42555 * locals.var_fb__blk971);
        let assign29790_body10_e42559: f64 = (assign29790_body10_e42557 + locals.var_fs01_dps0__blk970);
        let assign29790_body10_e42562: f64 = (locals.var_fs02__blk973 + locals.var_fs02__blk973);
        let assign29790_body10_e42563: f64 = (assign29790_body10_e42559 / assign29790_body10_e42562);
        (assign29790_body10_e42563, ((((((((locals.var_beta * locals.var_fb_dchi__blk972_dn0) * 2.0) * locals.var_fb__blk971) + (assign29790_body10_e42555 * locals.var_fb__blk971_dn0)) + locals.var_fs01_dps0__blk970_dn0) * assign29790_body10_e42562) - (assign29790_body10_e42559 * (locals.var_fs02__blk973_dn0 + locals.var_fs02__blk973_dn0))) / (assign29790_body10_e42562 * assign29790_body10_e42562)), ((((((((locals.var_beta * locals.var_fb_dchi__blk972_dn2) * 2.0) * locals.var_fb__blk971) + (assign29790_body10_e42555 * locals.var_fb__blk971_dn2)) + locals.var_fs01_dps0__blk970_dn2) * assign29790_body10_e42562) - (assign29790_body10_e42559 * (locals.var_fs02__blk973_dn2 + locals.var_fs02__blk973_dn2))) / (assign29790_body10_e42562 * assign29790_body10_e42562)), ((((((((locals.var_beta * locals.var_fb_dchi__blk972_dn6) * 2.0) * locals.var_fb__blk971) + (assign29790_body10_e42555 * locals.var_fb__blk971_dn6)) + locals.var_fs01_dps0__blk970_dn6) * assign29790_body10_e42562) - (assign29790_body10_e42559 * (locals.var_fs02__blk973_dn6 + locals.var_fs02__blk973_dn6))) / (assign29790_body10_e42562 * assign29790_body10_e42562)), ((((((((locals.var_beta * locals.var_fb_dchi__blk972_dn7) * 2.0) * locals.var_fb__blk971) + (assign29790_body10_e42555 * locals.var_fb__blk971_dn7)) + locals.var_fs01_dps0__blk970_dn7) * assign29790_body10_e42562) - (assign29790_body10_e42559 * (locals.var_fs02__blk973_dn7 + locals.var_fs02__blk973_dn7))) / (assign29790_body10_e42562 * assign29790_body10_e42562)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi__blk972) + (locals.var_beta * locals.var_fb_dchi__blk972_dn10)) * 2.0) * locals.var_fb__blk971) + (assign29790_body10_e42555 * locals.var_fb__blk971_dn10)) + locals.var_fs01_dps0__blk970_dn10) * assign29790_body10_e42562) - (assign29790_body10_e42559 * (locals.var_fs02__blk973_dn10 + locals.var_fs02__blk973_dn10))) / (assign29790_body10_e42562 * assign29790_body10_e42562)), ((((((((locals.var_beta * locals.var_fb_dchi__blk972_dn11) * 2.0) * locals.var_fb__blk971) + (assign29790_body10_e42555 * locals.var_fb__blk971_dn11)) + locals.var_fs01_dps0__blk970_dn11) * assign29790_body10_e42562) - (assign29790_body10_e42559 * (locals.var_fs02__blk973_dn11 + locals.var_fs02__blk973_dn11))) / (assign29790_body10_e42562 * assign29790_body10_e42562)), ((((((((locals.var_beta * locals.var_fb_dchi__blk972_dn12) * 2.0) * locals.var_fb__blk971) + (assign29790_body10_e42555 * locals.var_fb__blk971_dn12)) + locals.var_fs01_dps0__blk970_dn12) * assign29790_body10_e42562) - (assign29790_body10_e42559 * (locals.var_fs02__blk973_dn12 + locals.var_fs02__blk973_dn12))) / (assign29790_body10_e42562 * assign29790_body10_e42562)), ((((((((locals.var_beta * locals.var_fb_dchi__blk972_dn17) * 2.0) * locals.var_fb__blk971) + (assign29790_body10_e42555 * locals.var_fb__blk971_dn17)) + locals.var_fs01_dps0__blk970_dn17) * assign29790_body10_e42562) - (assign29790_body10_e42559 * (locals.var_fs02__blk973_dn17 + locals.var_fs02__blk973_dn17))) / (assign29790_body10_e42562 * assign29790_body10_e42562)),)
    } else {
        (locals.var_fs02_dps0__blk974, locals.var_fs02_dps0__blk974_dn0, locals.var_fs02_dps0__blk974_dn2, locals.var_fs02_dps0__blk974_dn6, locals.var_fs02_dps0__blk974_dn7, locals.var_fs02_dps0__blk974_dn10, locals.var_fs02_dps0__blk974_dn11, locals.var_fs02_dps0__blk974_dn12, locals.var_fs02_dps0__blk974_dn17,)
    }
};
            locals.var_fs02_dps0__blk974 = assign29790_body10_e42565;
            locals.var_fs02_dps0__blk974_dn0 = assign29790_body10_e42565_d_n0;
            locals.var_fs02_dps0__blk974_dn2 = assign29790_body10_e42565_d_n2;
            locals.var_fs02_dps0__blk974_dn6 = assign29790_body10_e42565_d_n6;
            locals.var_fs02_dps0__blk974_dn7 = assign29790_body10_e42565_d_n7;
            locals.var_fs02_dps0__blk974_dn10 = assign29790_body10_e42565_d_n10;
            locals.var_fs02_dps0__blk974_dn11 = assign29790_body10_e42565_d_n11;
            locals.var_fs02_dps0__blk974_dn12 = assign29790_body10_e42565_d_n12;
            locals.var_fs02_dps0__blk974_dn17 = assign29790_body10_e42565_d_n17;
            locals.var_fs02_dps0__blk974_rv = 0.0;
            let assign29790_body11_e42568: f64 = if locals.var_chi__blk947 < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard995 = assign29790_body11_e42568;
            locals.var_guard995_rv = 0.0;
            let (assign29790_body12_e42588, assign29790_body12_e42588_d_n0, assign29790_body12_e42588_d_n2, assign29790_body12_e42588_d_n6, assign29790_body12_e42588_d_n7, assign29790_body12_e42588_d_n10, assign29790_body12_e42588_d_n11, assign29790_body12_e42588_d_n12, assign29790_body12_e42588_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard994 == 0.0)) && (locals.var_guard995 != 0.0)) {
        let assign29790_body12_e42586: f64 = (locals.var_chi__blk947).exp();
        (assign29790_body12_e42586, (assign29790_body12_e42586 * locals.var_chi__blk947_dn0), (assign29790_body12_e42586 * locals.var_chi__blk947_dn2), (assign29790_body12_e42586 * locals.var_chi__blk947_dn6), (assign29790_body12_e42586 * locals.var_chi__blk947_dn7), (assign29790_body12_e42586 * locals.var_chi__blk947_dn10), (assign29790_body12_e42586 * locals.var_chi__blk947_dn11), (assign29790_body12_e42586 * locals.var_chi__blk947_dn12), (assign29790_body12_e42586 * locals.var_chi__blk947_dn17),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17,)
    }
};
            locals.var_exp_chi = assign29790_body12_e42588;
            locals.var_exp_chi_dn0 = assign29790_body12_e42588_d_n0;
            locals.var_exp_chi_dn2 = assign29790_body12_e42588_d_n2;
            locals.var_exp_chi_dn6 = assign29790_body12_e42588_d_n6;
            locals.var_exp_chi_dn7 = assign29790_body12_e42588_d_n7;
            locals.var_exp_chi_dn10 = assign29790_body12_e42588_d_n10;
            locals.var_exp_chi_dn11 = assign29790_body12_e42588_d_n11;
            locals.var_exp_chi_dn12 = assign29790_body12_e42588_d_n12;
            locals.var_exp_chi_dn17 = assign29790_body12_e42588_d_n17;
            locals.var_exp_chi_rv = 0.0;
            let (assign29790_body13_e42611, assign29790_body13_e42611_d_n0, assign29790_body13_e42611_d_n2, assign29790_body13_e42611_d_n6, assign29790_body13_e42611_d_n7, assign29790_body13_e42611_d_n10, assign29790_body13_e42611_d_n11, assign29790_body13_e42611_d_n12, assign29790_body13_e42611_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard994 == 0.0)) && (locals.var_guard995 != 0.0)) {
        let assign29790_body13_e42608: f64 = (locals.var_exp_chi - 1.0);
        let assign29790_body13_e42609: f64 = (locals.var_cfs1__blk975 * assign29790_body13_e42608);
        (assign29790_body13_e42609, ((locals.var_cfs1__blk975_dn0 * assign29790_body13_e42608) + (locals.var_cfs1__blk975 * locals.var_exp_chi_dn0)), ((locals.var_cfs1__blk975_dn2 * assign29790_body13_e42608) + (locals.var_cfs1__blk975 * locals.var_exp_chi_dn2)), ((locals.var_cfs1__blk975_dn6 * assign29790_body13_e42608) + (locals.var_cfs1__blk975 * locals.var_exp_chi_dn6)), ((locals.var_cfs1__blk975_dn7 * assign29790_body13_e42608) + (locals.var_cfs1__blk975 * locals.var_exp_chi_dn7)), ((locals.var_cfs1__blk975_dn10 * assign29790_body13_e42608) + (locals.var_cfs1__blk975 * locals.var_exp_chi_dn10)), ((locals.var_cfs1__blk975_dn11 * assign29790_body13_e42608) + (locals.var_cfs1__blk975 * locals.var_exp_chi_dn11)), ((locals.var_cfs1__blk975_dn12 * assign29790_body13_e42608) + (locals.var_cfs1__blk975 * locals.var_exp_chi_dn12)), ((locals.var_cfs1__blk975_dn17 * assign29790_body13_e42608) + (locals.var_cfs1__blk975 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01__blk969, locals.var_fs01__blk969_dn0, locals.var_fs01__blk969_dn2, locals.var_fs01__blk969_dn6, locals.var_fs01__blk969_dn7, locals.var_fs01__blk969_dn10, locals.var_fs01__blk969_dn11, locals.var_fs01__blk969_dn12, locals.var_fs01__blk969_dn17,)
    }
};
            locals.var_fs01__blk969 = assign29790_body13_e42611;
            locals.var_fs01__blk969_dn0 = assign29790_body13_e42611_d_n0;
            locals.var_fs01__blk969_dn2 = assign29790_body13_e42611_d_n2;
            locals.var_fs01__blk969_dn6 = assign29790_body13_e42611_d_n6;
            locals.var_fs01__blk969_dn7 = assign29790_body13_e42611_d_n7;
            locals.var_fs01__blk969_dn10 = assign29790_body13_e42611_d_n10;
            locals.var_fs01__blk969_dn11 = assign29790_body13_e42611_d_n11;
            locals.var_fs01__blk969_dn12 = assign29790_body13_e42611_d_n12;
            locals.var_fs01__blk969_dn17 = assign29790_body13_e42611_d_n17;
            locals.var_fs01__blk969_rv = 0.0;
            let (assign29790_body14_e42634, assign29790_body14_e42634_d_n0, assign29790_body14_e42634_d_n2, assign29790_body14_e42634_d_n6, assign29790_body14_e42634_d_n7, assign29790_body14_e42634_d_n10, assign29790_body14_e42634_d_n11, assign29790_body14_e42634_d_n12, assign29790_body14_e42634_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard994 == 0.0)) && (locals.var_guard995 != 0.0)) {
        let assign29790_body14_e42630: f64 = (locals.var_cfs1__blk975 * locals.var_beta);
        let assign29790_body14_e42632: f64 = (assign29790_body14_e42630 * locals.var_exp_chi);
        (assign29790_body14_e42632, (((locals.var_cfs1__blk975_dn0 * locals.var_beta) * locals.var_exp_chi) + (assign29790_body14_e42630 * locals.var_exp_chi_dn0)), (((locals.var_cfs1__blk975_dn2 * locals.var_beta) * locals.var_exp_chi) + (assign29790_body14_e42630 * locals.var_exp_chi_dn2)), (((locals.var_cfs1__blk975_dn6 * locals.var_beta) * locals.var_exp_chi) + (assign29790_body14_e42630 * locals.var_exp_chi_dn6)), (((locals.var_cfs1__blk975_dn7 * locals.var_beta) * locals.var_exp_chi) + (assign29790_body14_e42630 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1__blk975_dn10 * locals.var_beta) + (locals.var_cfs1__blk975 * locals.var_beta_dn10)) * locals.var_exp_chi) + (assign29790_body14_e42630 * locals.var_exp_chi_dn10)), (((locals.var_cfs1__blk975_dn11 * locals.var_beta) * locals.var_exp_chi) + (assign29790_body14_e42630 * locals.var_exp_chi_dn11)), (((locals.var_cfs1__blk975_dn12 * locals.var_beta) * locals.var_exp_chi) + (assign29790_body14_e42630 * locals.var_exp_chi_dn12)), (((locals.var_cfs1__blk975_dn17 * locals.var_beta) * locals.var_exp_chi) + (assign29790_body14_e42630 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk970, locals.var_fs01_dps0__blk970_dn0, locals.var_fs01_dps0__blk970_dn2, locals.var_fs01_dps0__blk970_dn6, locals.var_fs01_dps0__blk970_dn7, locals.var_fs01_dps0__blk970_dn10, locals.var_fs01_dps0__blk970_dn11, locals.var_fs01_dps0__blk970_dn12, locals.var_fs01_dps0__blk970_dn17,)
    }
};
            locals.var_fs01_dps0__blk970 = assign29790_body14_e42634;
            locals.var_fs01_dps0__blk970_dn0 = assign29790_body14_e42634_d_n0;
            locals.var_fs01_dps0__blk970_dn2 = assign29790_body14_e42634_d_n2;
            locals.var_fs01_dps0__blk970_dn6 = assign29790_body14_e42634_d_n6;
            locals.var_fs01_dps0__blk970_dn7 = assign29790_body14_e42634_d_n7;
            locals.var_fs01_dps0__blk970_dn10 = assign29790_body14_e42634_d_n10;
            locals.var_fs01_dps0__blk970_dn11 = assign29790_body14_e42634_d_n11;
            locals.var_fs01_dps0__blk970_dn12 = assign29790_body14_e42634_d_n12;
            locals.var_fs01_dps0__blk970_dn17 = assign29790_body14_e42634_d_n17;
            locals.var_fs01_dps0__blk970_rv = 0.0;
            let (assign29790_body15_e42657, assign29790_body15_e42657_d_n0, assign29790_body15_e42657_d_n2, assign29790_body15_e42657_d_n6, assign29790_body15_e42657_d_n7, assign29790_body15_e42657_d_n10, assign29790_body15_e42657_d_n11, assign29790_body15_e42657_d_n12, assign29790_body15_e42657_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard994 == 0.0)) && (locals.var_guard995 == 0.0)) {
        let assign29790_body15_e42654: f64 = (locals.var_beta * locals.var_ps0ld__blk949);
        let assign29790_body15_e42655: f64 = (assign29790_body15_e42654).exp();
        (assign29790_body15_e42655, (assign29790_body15_e42655 * (locals.var_beta * locals.var_ps0ld__blk949_dn0)), (assign29790_body15_e42655 * (locals.var_beta * locals.var_ps0ld__blk949_dn2)), (assign29790_body15_e42655 * (locals.var_beta * locals.var_ps0ld__blk949_dn6)), (assign29790_body15_e42655 * (locals.var_beta * locals.var_ps0ld__blk949_dn7)), (assign29790_body15_e42655 * ((locals.var_beta_dn10 * locals.var_ps0ld__blk949) + (locals.var_beta * locals.var_ps0ld__blk949_dn10))), (assign29790_body15_e42655 * (locals.var_beta * locals.var_ps0ld__blk949_dn11)), (assign29790_body15_e42655 * (locals.var_beta * locals.var_ps0ld__blk949_dn12)), (assign29790_body15_e42655 * (locals.var_beta * locals.var_ps0ld__blk949_dn17)),)
    } else {
        (locals.var_exp_bps0__blk976, locals.var_exp_bps0__blk976_dn0, locals.var_exp_bps0__blk976_dn2, locals.var_exp_bps0__blk976_dn6, locals.var_exp_bps0__blk976_dn7, locals.var_exp_bps0__blk976_dn10, locals.var_exp_bps0__blk976_dn11, locals.var_exp_bps0__blk976_dn12, locals.var_exp_bps0__blk976_dn17,)
    }
};
            locals.var_exp_bps0__blk976 = assign29790_body15_e42657;
            locals.var_exp_bps0__blk976_dn0 = assign29790_body15_e42657_d_n0;
            locals.var_exp_bps0__blk976_dn2 = assign29790_body15_e42657_d_n2;
            locals.var_exp_bps0__blk976_dn6 = assign29790_body15_e42657_d_n6;
            locals.var_exp_bps0__blk976_dn7 = assign29790_body15_e42657_d_n7;
            locals.var_exp_bps0__blk976_dn10 = assign29790_body15_e42657_d_n10;
            locals.var_exp_bps0__blk976_dn11 = assign29790_body15_e42657_d_n11;
            locals.var_exp_bps0__blk976_dn12 = assign29790_body15_e42657_d_n12;
            locals.var_exp_bps0__blk976_dn17 = assign29790_body15_e42657_d_n17;
            locals.var_exp_bps0__blk976_rv = 0.0;
            let (assign29790_body16_e42681, assign29790_body16_e42681_d_n0, assign29790_body16_e42681_d_n2, assign29790_body16_e42681_d_n6, assign29790_body16_e42681_d_n7, assign29790_body16_e42681_d_n10, assign29790_body16_e42681_d_n11, assign29790_body16_e42681_d_n12, assign29790_body16_e42681_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard994 == 0.0)) && (locals.var_guard995 == 0.0)) {
        let assign29790_body16_e42678: f64 = (locals.var_exp_bps0__blk976 - locals.var_exp_bvbs__blk966);
        let assign29790_body16_e42679: f64 = (locals.var_cnst1over__blk960 * assign29790_body16_e42678);
        (assign29790_body16_e42679, ((locals.var_cnst1over__blk960_dn0 * assign29790_body16_e42678) + (locals.var_cnst1over__blk960 * (locals.var_exp_bps0__blk976_dn0 - locals.var_exp_bvbs__blk966_dn0))), ((locals.var_cnst1over__blk960_dn2 * assign29790_body16_e42678) + (locals.var_cnst1over__blk960 * (locals.var_exp_bps0__blk976_dn2 - locals.var_exp_bvbs__blk966_dn2))), ((locals.var_cnst1over__blk960_dn6 * assign29790_body16_e42678) + (locals.var_cnst1over__blk960 * (locals.var_exp_bps0__blk976_dn6 - locals.var_exp_bvbs__blk966_dn6))), ((locals.var_cnst1over__blk960_dn7 * assign29790_body16_e42678) + (locals.var_cnst1over__blk960 * (locals.var_exp_bps0__blk976_dn7 - locals.var_exp_bvbs__blk966_dn7))), ((locals.var_cnst1over__blk960_dn10 * assign29790_body16_e42678) + (locals.var_cnst1over__blk960 * (locals.var_exp_bps0__blk976_dn10 - locals.var_exp_bvbs__blk966_dn10))), ((locals.var_cnst1over__blk960_dn11 * assign29790_body16_e42678) + (locals.var_cnst1over__blk960 * (locals.var_exp_bps0__blk976_dn11 - locals.var_exp_bvbs__blk966_dn11))), ((locals.var_cnst1over__blk960_dn12 * assign29790_body16_e42678) + (locals.var_cnst1over__blk960 * (locals.var_exp_bps0__blk976_dn12 - locals.var_exp_bvbs__blk966_dn12))), ((locals.var_cnst1over__blk960_dn17 * assign29790_body16_e42678) + (locals.var_cnst1over__blk960 * (locals.var_exp_bps0__blk976_dn17 - locals.var_exp_bvbs__blk966_dn17))),)
    } else {
        (locals.var_fs01__blk969, locals.var_fs01__blk969_dn0, locals.var_fs01__blk969_dn2, locals.var_fs01__blk969_dn6, locals.var_fs01__blk969_dn7, locals.var_fs01__blk969_dn10, locals.var_fs01__blk969_dn11, locals.var_fs01__blk969_dn12, locals.var_fs01__blk969_dn17,)
    }
};
            locals.var_fs01__blk969 = assign29790_body16_e42681;
            locals.var_fs01__blk969_dn0 = assign29790_body16_e42681_d_n0;
            locals.var_fs01__blk969_dn2 = assign29790_body16_e42681_d_n2;
            locals.var_fs01__blk969_dn6 = assign29790_body16_e42681_d_n6;
            locals.var_fs01__blk969_dn7 = assign29790_body16_e42681_d_n7;
            locals.var_fs01__blk969_dn10 = assign29790_body16_e42681_d_n10;
            locals.var_fs01__blk969_dn11 = assign29790_body16_e42681_d_n11;
            locals.var_fs01__blk969_dn12 = assign29790_body16_e42681_d_n12;
            locals.var_fs01__blk969_dn17 = assign29790_body16_e42681_d_n17;
            locals.var_fs01__blk969_rv = 0.0;
            let (assign29790_body17_e42705, assign29790_body17_e42705_d_n0, assign29790_body17_e42705_d_n2, assign29790_body17_e42705_d_n6, assign29790_body17_e42705_d_n7, assign29790_body17_e42705_d_n10, assign29790_body17_e42705_d_n11, assign29790_body17_e42705_d_n12, assign29790_body17_e42705_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard994 == 0.0)) && (locals.var_guard995 == 0.0)) {
        let assign29790_body17_e42701: f64 = (locals.var_cnst1over__blk960 * locals.var_beta);
        let assign29790_body17_e42703: f64 = (assign29790_body17_e42701 * locals.var_exp_bps0__blk976);
        (assign29790_body17_e42703, (((locals.var_cnst1over__blk960_dn0 * locals.var_beta) * locals.var_exp_bps0__blk976) + (assign29790_body17_e42701 * locals.var_exp_bps0__blk976_dn0)), (((locals.var_cnst1over__blk960_dn2 * locals.var_beta) * locals.var_exp_bps0__blk976) + (assign29790_body17_e42701 * locals.var_exp_bps0__blk976_dn2)), (((locals.var_cnst1over__blk960_dn6 * locals.var_beta) * locals.var_exp_bps0__blk976) + (assign29790_body17_e42701 * locals.var_exp_bps0__blk976_dn6)), (((locals.var_cnst1over__blk960_dn7 * locals.var_beta) * locals.var_exp_bps0__blk976) + (assign29790_body17_e42701 * locals.var_exp_bps0__blk976_dn7)), ((((locals.var_cnst1over__blk960_dn10 * locals.var_beta) + (locals.var_cnst1over__blk960 * locals.var_beta_dn10)) * locals.var_exp_bps0__blk976) + (assign29790_body17_e42701 * locals.var_exp_bps0__blk976_dn10)), (((locals.var_cnst1over__blk960_dn11 * locals.var_beta) * locals.var_exp_bps0__blk976) + (assign29790_body17_e42701 * locals.var_exp_bps0__blk976_dn11)), (((locals.var_cnst1over__blk960_dn12 * locals.var_beta) * locals.var_exp_bps0__blk976) + (assign29790_body17_e42701 * locals.var_exp_bps0__blk976_dn12)), (((locals.var_cnst1over__blk960_dn17 * locals.var_beta) * locals.var_exp_bps0__blk976) + (assign29790_body17_e42701 * locals.var_exp_bps0__blk976_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk970, locals.var_fs01_dps0__blk970_dn0, locals.var_fs01_dps0__blk970_dn2, locals.var_fs01_dps0__blk970_dn6, locals.var_fs01_dps0__blk970_dn7, locals.var_fs01_dps0__blk970_dn10, locals.var_fs01_dps0__blk970_dn11, locals.var_fs01_dps0__blk970_dn12, locals.var_fs01_dps0__blk970_dn17,)
    }
};
            locals.var_fs01_dps0__blk970 = assign29790_body17_e42705;
            locals.var_fs01_dps0__blk970_dn0 = assign29790_body17_e42705_d_n0;
            locals.var_fs01_dps0__blk970_dn2 = assign29790_body17_e42705_d_n2;
            locals.var_fs01_dps0__blk970_dn6 = assign29790_body17_e42705_d_n6;
            locals.var_fs01_dps0__blk970_dn7 = assign29790_body17_e42705_d_n7;
            locals.var_fs01_dps0__blk970_dn10 = assign29790_body17_e42705_d_n10;
            locals.var_fs01_dps0__blk970_dn11 = assign29790_body17_e42705_d_n11;
            locals.var_fs01_dps0__blk970_dn12 = assign29790_body17_e42705_d_n12;
            locals.var_fs01_dps0__blk970_dn17 = assign29790_body17_e42705_d_n17;
            locals.var_fs01_dps0__blk970_rv = 0.0;
            let (assign29790_body18_e42727, assign29790_body18_e42727_d_n0, assign29790_body18_e42727_d_n2, assign29790_body18_e42727_d_n6, assign29790_body18_e42727_d_n7, assign29790_body18_e42727_d_n10, assign29790_body18_e42727_d_n11, assign29790_body18_e42727_d_n12, assign29790_body18_e42727_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard994 == 0.0)) {
        let assign29790_body18_e42722: f64 = (locals.var_chi__blk947 - 1.0);
        let assign29790_body18_e42724: f64 = (assign29790_body18_e42722 + locals.var_fs01__blk969);
        let assign29790_body18_e42725: f64 = (assign29790_body18_e42724).sqrt();
        (assign29790_body18_e42725, ((locals.var_chi__blk947_dn0 + locals.var_fs01__blk969_dn0) / (2.0 * assign29790_body18_e42725)), ((locals.var_chi__blk947_dn2 + locals.var_fs01__blk969_dn2) / (2.0 * assign29790_body18_e42725)), ((locals.var_chi__blk947_dn6 + locals.var_fs01__blk969_dn6) / (2.0 * assign29790_body18_e42725)), ((locals.var_chi__blk947_dn7 + locals.var_fs01__blk969_dn7) / (2.0 * assign29790_body18_e42725)), ((locals.var_chi__blk947_dn10 + locals.var_fs01__blk969_dn10) / (2.0 * assign29790_body18_e42725)), ((locals.var_chi__blk947_dn11 + locals.var_fs01__blk969_dn11) / (2.0 * assign29790_body18_e42725)), ((locals.var_chi__blk947_dn12 + locals.var_fs01__blk969_dn12) / (2.0 * assign29790_body18_e42725)), ((locals.var_chi__blk947_dn17 + locals.var_fs01__blk969_dn17) / (2.0 * assign29790_body18_e42725)),)
    } else {
        (locals.var_fs02__blk973, locals.var_fs02__blk973_dn0, locals.var_fs02__blk973_dn2, locals.var_fs02__blk973_dn6, locals.var_fs02__blk973_dn7, locals.var_fs02__blk973_dn10, locals.var_fs02__blk973_dn11, locals.var_fs02__blk973_dn12, locals.var_fs02__blk973_dn17,)
    }
};
            locals.var_fs02__blk973 = assign29790_body18_e42727;
            locals.var_fs02__blk973_dn0 = assign29790_body18_e42727_d_n0;
            locals.var_fs02__blk973_dn2 = assign29790_body18_e42727_d_n2;
            locals.var_fs02__blk973_dn6 = assign29790_body18_e42727_d_n6;
            locals.var_fs02__blk973_dn7 = assign29790_body18_e42727_d_n7;
            locals.var_fs02__blk973_dn10 = assign29790_body18_e42727_d_n10;
            locals.var_fs02__blk973_dn11 = assign29790_body18_e42727_d_n11;
            locals.var_fs02__blk973_dn12 = assign29790_body18_e42727_d_n12;
            locals.var_fs02__blk973_dn17 = assign29790_body18_e42727_d_n17;
            locals.var_fs02__blk973_rv = 0.0;
            let (assign29790_body19_e42750, assign29790_body19_e42750_d_n0, assign29790_body19_e42750_d_n2, assign29790_body19_e42750_d_n6, assign29790_body19_e42750_d_n7, assign29790_body19_e42750_d_n10, assign29790_body19_e42750_d_n11, assign29790_body19_e42750_d_n12, assign29790_body19_e42750_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard994 == 0.0)) {
        let assign29790_body19_e42744: f64 = (locals.var_beta + locals.var_fs01_dps0__blk970);
        let assign29790_body19_e42746: f64 = (assign29790_body19_e42744 / locals.var_fs02__blk973);
        let assign29790_body19_e42748: f64 = (assign29790_body19_e42746 * 0.5);
        (assign29790_body19_e42748, ((((locals.var_fs01_dps0__blk970_dn0 * locals.var_fs02__blk973) - (assign29790_body19_e42744 * locals.var_fs02__blk973_dn0)) / (locals.var_fs02__blk973 * locals.var_fs02__blk973)) * 0.5), ((((locals.var_fs01_dps0__blk970_dn2 * locals.var_fs02__blk973) - (assign29790_body19_e42744 * locals.var_fs02__blk973_dn2)) / (locals.var_fs02__blk973 * locals.var_fs02__blk973)) * 0.5), ((((locals.var_fs01_dps0__blk970_dn6 * locals.var_fs02__blk973) - (assign29790_body19_e42744 * locals.var_fs02__blk973_dn6)) / (locals.var_fs02__blk973 * locals.var_fs02__blk973)) * 0.5), ((((locals.var_fs01_dps0__blk970_dn7 * locals.var_fs02__blk973) - (assign29790_body19_e42744 * locals.var_fs02__blk973_dn7)) / (locals.var_fs02__blk973 * locals.var_fs02__blk973)) * 0.5), (((((locals.var_beta_dn10 + locals.var_fs01_dps0__blk970_dn10) * locals.var_fs02__blk973) - (assign29790_body19_e42744 * locals.var_fs02__blk973_dn10)) / (locals.var_fs02__blk973 * locals.var_fs02__blk973)) * 0.5), ((((locals.var_fs01_dps0__blk970_dn11 * locals.var_fs02__blk973) - (assign29790_body19_e42744 * locals.var_fs02__blk973_dn11)) / (locals.var_fs02__blk973 * locals.var_fs02__blk973)) * 0.5), ((((locals.var_fs01_dps0__blk970_dn12 * locals.var_fs02__blk973) - (assign29790_body19_e42744 * locals.var_fs02__blk973_dn12)) / (locals.var_fs02__blk973 * locals.var_fs02__blk973)) * 0.5), ((((locals.var_fs01_dps0__blk970_dn17 * locals.var_fs02__blk973) - (assign29790_body19_e42744 * locals.var_fs02__blk973_dn17)) / (locals.var_fs02__blk973 * locals.var_fs02__blk973)) * 0.5),)
    } else {
        (locals.var_fs02_dps0__blk974, locals.var_fs02_dps0__blk974_dn0, locals.var_fs02_dps0__blk974_dn2, locals.var_fs02_dps0__blk974_dn6, locals.var_fs02_dps0__blk974_dn7, locals.var_fs02_dps0__blk974_dn10, locals.var_fs02_dps0__blk974_dn11, locals.var_fs02_dps0__blk974_dn12, locals.var_fs02_dps0__blk974_dn17,)
    }
};
            locals.var_fs02_dps0__blk974 = assign29790_body19_e42750;
            locals.var_fs02_dps0__blk974_dn0 = assign29790_body19_e42750_d_n0;
            locals.var_fs02_dps0__blk974_dn2 = assign29790_body19_e42750_d_n2;
            locals.var_fs02_dps0__blk974_dn6 = assign29790_body19_e42750_d_n6;
            locals.var_fs02_dps0__blk974_dn7 = assign29790_body19_e42750_d_n7;
            locals.var_fs02_dps0__blk974_dn10 = assign29790_body19_e42750_d_n10;
            locals.var_fs02_dps0__blk974_dn11 = assign29790_body19_e42750_d_n11;
            locals.var_fs02_dps0__blk974_dn12 = assign29790_body19_e42750_d_n12;
            locals.var_fs02_dps0__blk974_dn17 = assign29790_body19_e42750_d_n17;
            locals.var_fs02_dps0__blk974_rv = 0.0;
            let (assign29790_body20_e42770, assign29790_body20_e42770_d_n0, assign29790_body20_e42770_d_n2, assign29790_body20_e42770_d_n6, assign29790_body20_e42770_d_n7, assign29790_body20_e42770_d_n10, assign29790_body20_e42770_d_n11, assign29790_body20_e42770_d_n12, assign29790_body20_e42770_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        let assign29790_body20_e42764: f64 = (locals.var_vgpld__blk935 - locals.var_ps0ld__blk949);
        let assign29790_body20_e42767: f64 = (locals.var_fac1__blk933 * locals.var_fs02__blk973);
        let assign29790_body20_e42768: f64 = (assign29790_body20_e42764 - assign29790_body20_e42767);
        (assign29790_body20_e42768, ((locals.var_vgpld__blk935_dn0 - locals.var_ps0ld__blk949_dn0) - ((locals.var_fac1__blk933_dn0 * locals.var_fs02__blk973) + (locals.var_fac1__blk933 * locals.var_fs02__blk973_dn0))), ((locals.var_vgpld__blk935_dn2 - locals.var_ps0ld__blk949_dn2) - ((locals.var_fac1__blk933_dn2 * locals.var_fs02__blk973) + (locals.var_fac1__blk933 * locals.var_fs02__blk973_dn2))), ((locals.var_vgpld__blk935_dn6 - locals.var_ps0ld__blk949_dn6) - ((locals.var_fac1__blk933_dn6 * locals.var_fs02__blk973) + (locals.var_fac1__blk933 * locals.var_fs02__blk973_dn6))), ((locals.var_vgpld__blk935_dn7 - locals.var_ps0ld__blk949_dn7) - ((locals.var_fac1__blk933_dn7 * locals.var_fs02__blk973) + (locals.var_fac1__blk933 * locals.var_fs02__blk973_dn7))), ((locals.var_vgpld__blk935_dn10 - locals.var_ps0ld__blk949_dn10) - ((locals.var_fac1__blk933_dn10 * locals.var_fs02__blk973) + (locals.var_fac1__blk933 * locals.var_fs02__blk973_dn10))), ((locals.var_vgpld__blk935_dn11 - locals.var_ps0ld__blk949_dn11) - ((locals.var_fac1__blk933_dn11 * locals.var_fs02__blk973) + (locals.var_fac1__blk933 * locals.var_fs02__blk973_dn11))), ((locals.var_vgpld__blk935_dn12 - locals.var_ps0ld__blk949_dn12) - ((locals.var_fac1__blk933_dn12 * locals.var_fs02__blk973) + (locals.var_fac1__blk933 * locals.var_fs02__blk973_dn12))), ((locals.var_vgpld__blk935_dn17 - locals.var_ps0ld__blk949_dn17) - ((locals.var_fac1__blk933_dn17 * locals.var_fs02__blk973) + (locals.var_fac1__blk933 * locals.var_fs02__blk973_dn17))),)
    } else {
        (locals.var_fs0__blk977, locals.var_fs0__blk977_dn0, locals.var_fs0__blk977_dn2, locals.var_fs0__blk977_dn6, locals.var_fs0__blk977_dn7, locals.var_fs0__blk977_dn10, locals.var_fs0__blk977_dn11, locals.var_fs0__blk977_dn12, locals.var_fs0__blk977_dn17,)
    }
};
            locals.var_fs0__blk977 = assign29790_body20_e42770;
            locals.var_fs0__blk977_dn0 = assign29790_body20_e42770_d_n0;
            locals.var_fs0__blk977_dn2 = assign29790_body20_e42770_d_n2;
            locals.var_fs0__blk977_dn6 = assign29790_body20_e42770_d_n6;
            locals.var_fs0__blk977_dn7 = assign29790_body20_e42770_d_n7;
            locals.var_fs0__blk977_dn10 = assign29790_body20_e42770_d_n10;
            locals.var_fs0__blk977_dn11 = assign29790_body20_e42770_d_n11;
            locals.var_fs0__blk977_dn12 = assign29790_body20_e42770_d_n12;
            locals.var_fs0__blk977_dn17 = assign29790_body20_e42770_d_n17;
            locals.var_fs0__blk977_rv = 0.0;
            let (assign29790_body21_e42789, assign29790_body21_e42789_d_n0, assign29790_body21_e42789_d_n2, assign29790_body21_e42789_d_n6, assign29790_body21_e42789_d_n7, assign29790_body21_e42789_d_n10, assign29790_body21_e42789_d_n11, assign29790_body21_e42789_d_n12, assign29790_body21_e42789_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        let assign29790_body21_e42783: f64 = (-1.0);
        let assign29790_body21_e42786: f64 = (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974);
        let assign29790_body21_e42787: f64 = (assign29790_body21_e42783 - assign29790_body21_e42786);
        (assign29790_body21_e42787, (-((locals.var_fac1__blk933_dn0 * locals.var_fs02_dps0__blk974) + (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974_dn0))), (-((locals.var_fac1__blk933_dn2 * locals.var_fs02_dps0__blk974) + (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974_dn2))), (-((locals.var_fac1__blk933_dn6 * locals.var_fs02_dps0__blk974) + (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974_dn6))), (-((locals.var_fac1__blk933_dn7 * locals.var_fs02_dps0__blk974) + (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974_dn7))), (-((locals.var_fac1__blk933_dn10 * locals.var_fs02_dps0__blk974) + (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974_dn10))), (-((locals.var_fac1__blk933_dn11 * locals.var_fs02_dps0__blk974) + (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974_dn11))), (-((locals.var_fac1__blk933_dn12 * locals.var_fs02_dps0__blk974) + (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974_dn12))), (-((locals.var_fac1__blk933_dn17 * locals.var_fs02_dps0__blk974) + (locals.var_fac1__blk933 * locals.var_fs02_dps0__blk974_dn17))),)
    } else {
        (locals.var_fs0_dps0__blk978, locals.var_fs0_dps0__blk978_dn0, locals.var_fs0_dps0__blk978_dn2, locals.var_fs0_dps0__blk978_dn6, locals.var_fs0_dps0__blk978_dn7, locals.var_fs0_dps0__blk978_dn10, locals.var_fs0_dps0__blk978_dn11, locals.var_fs0_dps0__blk978_dn12, locals.var_fs0_dps0__blk978_dn17,)
    }
};
            locals.var_fs0_dps0__blk978 = assign29790_body21_e42789;
            locals.var_fs0_dps0__blk978_dn0 = assign29790_body21_e42789_d_n0;
            locals.var_fs0_dps0__blk978_dn2 = assign29790_body21_e42789_d_n2;
            locals.var_fs0_dps0__blk978_dn6 = assign29790_body21_e42789_d_n6;
            locals.var_fs0_dps0__blk978_dn7 = assign29790_body21_e42789_d_n7;
            locals.var_fs0_dps0__blk978_dn10 = assign29790_body21_e42789_d_n10;
            locals.var_fs0_dps0__blk978_dn11 = assign29790_body21_e42789_d_n11;
            locals.var_fs0_dps0__blk978_dn12 = assign29790_body21_e42789_d_n12;
            locals.var_fs0_dps0__blk978_dn17 = assign29790_body21_e42789_d_n17;
            locals.var_fs0_dps0__blk978_rv = 0.0;
            let assign29790_body22_e42792: f64 = if locals.var_flg_conv__blk922 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard996 = assign29790_body22_e42792;
            locals.var_guard996_rv = 0.0;
            let (assign29790_body23_e42812,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard996 != 0.0)) {
        let assign29790_body23_e42808: f64 = (2.0 * 20.0);
        let assign29790_body23_e42810: f64 = (assign29790_body23_e42808 + 1.0);
        (assign29790_body23_e42810,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign29790_body23_e42812;
            locals.var_lp_s0_rv = 0.0;
            let (assign29790_body24_e42832, assign29790_body24_e42832_d_n0, assign29790_body24_e42832_d_n2, assign29790_body24_e42832_d_n6, assign29790_body24_e42832_d_n7, assign29790_body24_e42832_d_n10, assign29790_body24_e42832_d_n11, assign29790_body24_e42832_d_n12, assign29790_body24_e42832_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard996 == 0.0)) {
        let assign29790_body24_e42828: f64 = (-locals.var_fs0__blk977);
        let assign29790_body24_e42830: f64 = (assign29790_body24_e42828 / locals.var_fs0_dps0__blk978);
        (assign29790_body24_e42830, ((((-locals.var_fs0__blk977_dn0) * locals.var_fs0_dps0__blk978) - (assign29790_body24_e42828 * locals.var_fs0_dps0__blk978_dn0)) / (locals.var_fs0_dps0__blk978 * locals.var_fs0_dps0__blk978)), ((((-locals.var_fs0__blk977_dn2) * locals.var_fs0_dps0__blk978) - (assign29790_body24_e42828 * locals.var_fs0_dps0__blk978_dn2)) / (locals.var_fs0_dps0__blk978 * locals.var_fs0_dps0__blk978)), ((((-locals.var_fs0__blk977_dn6) * locals.var_fs0_dps0__blk978) - (assign29790_body24_e42828 * locals.var_fs0_dps0__blk978_dn6)) / (locals.var_fs0_dps0__blk978 * locals.var_fs0_dps0__blk978)), ((((-locals.var_fs0__blk977_dn7) * locals.var_fs0_dps0__blk978) - (assign29790_body24_e42828 * locals.var_fs0_dps0__blk978_dn7)) / (locals.var_fs0_dps0__blk978 * locals.var_fs0_dps0__blk978)), ((((-locals.var_fs0__blk977_dn10) * locals.var_fs0_dps0__blk978) - (assign29790_body24_e42828 * locals.var_fs0_dps0__blk978_dn10)) / (locals.var_fs0_dps0__blk978 * locals.var_fs0_dps0__blk978)), ((((-locals.var_fs0__blk977_dn11) * locals.var_fs0_dps0__blk978) - (assign29790_body24_e42828 * locals.var_fs0_dps0__blk978_dn11)) / (locals.var_fs0_dps0__blk978 * locals.var_fs0_dps0__blk978)), ((((-locals.var_fs0__blk977_dn12) * locals.var_fs0_dps0__blk978) - (assign29790_body24_e42828 * locals.var_fs0_dps0__blk978_dn12)) / (locals.var_fs0_dps0__blk978 * locals.var_fs0_dps0__blk978)), ((((-locals.var_fs0__blk977_dn17) * locals.var_fs0_dps0__blk978) - (assign29790_body24_e42828 * locals.var_fs0_dps0__blk978_dn17)) / (locals.var_fs0_dps0__blk978 * locals.var_fs0_dps0__blk978)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign29790_body24_e42832;
            locals.var_dps0_dn0 = assign29790_body24_e42832_d_n0;
            locals.var_dps0_dn2 = assign29790_body24_e42832_d_n2;
            locals.var_dps0_dn6 = assign29790_body24_e42832_d_n6;
            locals.var_dps0_dn7 = assign29790_body24_e42832_d_n7;
            locals.var_dps0_dn10 = assign29790_body24_e42832_d_n10;
            locals.var_dps0_dn11 = assign29790_body24_e42832_d_n11;
            locals.var_dps0_dn12 = assign29790_body24_e42832_d_n12;
            locals.var_dps0_dn17 = assign29790_body24_e42832_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign29790_body25_e42862, assign29790_body25_e42862_d_n0, assign29790_body25_e42862_d_n2, assign29790_body25_e42862_d_n6, assign29790_body25_e42862_d_n7, assign29790_body25_e42862_d_n10, assign29790_body25_e42862_d_n11, assign29790_body25_e42862_d_n12, assign29790_body25_e42862_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard996 == 0.0)) {
        let assign29790_body25_e42849: f64 = (0.5 * 0.1);
        let assign29790_body25_e42853: f64 = (locals.var_ps0ld__blk949).abs();
        let (assign29790_body25_e42858, assign29790_body25_e42858_d_n0, assign29790_body25_e42858_d_n2, assign29790_body25_e42858_d_n6, assign29790_body25_e42858_d_n7, assign29790_body25_e42858_d_n10, assign29790_body25_e42858_d_n11, assign29790_body25_e42858_d_n12, assign29790_body25_e42858_d_n17,) = {
            if (1.0 >= assign29790_body25_e42853) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign29790_body25_e42857: f64 = (locals.var_ps0ld__blk949).abs();
                (assign29790_body25_e42857, if locals.var_ps0ld__blk949 >= 0.0 { locals.var_ps0ld__blk949_dn0 } else { (-locals.var_ps0ld__blk949_dn0) }, if locals.var_ps0ld__blk949 >= 0.0 { locals.var_ps0ld__blk949_dn2 } else { (-locals.var_ps0ld__blk949_dn2) }, if locals.var_ps0ld__blk949 >= 0.0 { locals.var_ps0ld__blk949_dn6 } else { (-locals.var_ps0ld__blk949_dn6) }, if locals.var_ps0ld__blk949 >= 0.0 { locals.var_ps0ld__blk949_dn7 } else { (-locals.var_ps0ld__blk949_dn7) }, if locals.var_ps0ld__blk949 >= 0.0 { locals.var_ps0ld__blk949_dn10 } else { (-locals.var_ps0ld__blk949_dn10) }, if locals.var_ps0ld__blk949 >= 0.0 { locals.var_ps0ld__blk949_dn11 } else { (-locals.var_ps0ld__blk949_dn11) }, if locals.var_ps0ld__blk949 >= 0.0 { locals.var_ps0ld__blk949_dn12 } else { (-locals.var_ps0ld__blk949_dn12) }, if locals.var_ps0ld__blk949 >= 0.0 { locals.var_ps0ld__blk949_dn17 } else { (-locals.var_ps0ld__blk949_dn17) },)
            }
        };
        let assign29790_body25_e42859: f64 = (1.0 + assign29790_body25_e42858);
        let assign29790_body25_e42860: f64 = (assign29790_body25_e42849 * assign29790_body25_e42859);
        (assign29790_body25_e42860, (assign29790_body25_e42849 * assign29790_body25_e42858_d_n0), (assign29790_body25_e42849 * assign29790_body25_e42858_d_n2), (assign29790_body25_e42849 * assign29790_body25_e42858_d_n6), (assign29790_body25_e42849 * assign29790_body25_e42858_d_n7), (assign29790_body25_e42849 * assign29790_body25_e42858_d_n10), (assign29790_body25_e42849 * assign29790_body25_e42858_d_n11), (assign29790_body25_e42849 * assign29790_body25_e42858_d_n12), (assign29790_body25_e42849 * assign29790_body25_e42858_d_n17),)
    } else {
        (locals.var_dplim__blk979, locals.var_dplim__blk979_dn0, locals.var_dplim__blk979_dn2, locals.var_dplim__blk979_dn6, locals.var_dplim__blk979_dn7, locals.var_dplim__blk979_dn10, locals.var_dplim__blk979_dn11, locals.var_dplim__blk979_dn12, locals.var_dplim__blk979_dn17,)
    }
};
            locals.var_dplim__blk979 = assign29790_body25_e42862;
            locals.var_dplim__blk979_dn0 = assign29790_body25_e42862_d_n0;
            locals.var_dplim__blk979_dn2 = assign29790_body25_e42862_d_n2;
            locals.var_dplim__blk979_dn6 = assign29790_body25_e42862_d_n6;
            locals.var_dplim__blk979_dn7 = assign29790_body25_e42862_d_n7;
            locals.var_dplim__blk979_dn10 = assign29790_body25_e42862_d_n10;
            locals.var_dplim__blk979_dn11 = assign29790_body25_e42862_d_n11;
            locals.var_dplim__blk979_dn12 = assign29790_body25_e42862_d_n12;
            locals.var_dplim__blk979_dn17 = assign29790_body25_e42862_d_n17;
            locals.var_dplim__blk979_rv = 0.0;
            let assign29790_body26_e42864: f64 = (locals.var_dps0).abs();
            let assign29790_body26_e42866: f64 = if assign29790_body26_e42864 > locals.var_dplim__blk979 { 1.0 } else { 0.0 };
            locals.var_guard997 = assign29790_body26_e42866;
            locals.var_guard997_rv = 0.0;
            let (assign29790_body27_e42893, assign29790_body27_e42893_d_n0, assign29790_body27_e42893_d_n2, assign29790_body27_e42893_d_n6, assign29790_body27_e42893_d_n7, assign29790_body27_e42893_d_n10, assign29790_body27_e42893_d_n11, assign29790_body27_e42893_d_n12, assign29790_body27_e42893_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard996 == 0.0)) && (locals.var_guard997 != 0.0)) {
        let (assign29790_body27_e42890,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign29790_body27_e42889: f64 = (-1.0);
                (assign29790_body27_e42889,)
            }
        };
        let assign29790_body27_e42891: f64 = (locals.var_dplim__blk979 * assign29790_body27_e42890);
        (assign29790_body27_e42891, (locals.var_dplim__blk979_dn0 * assign29790_body27_e42890), (locals.var_dplim__blk979_dn2 * assign29790_body27_e42890), (locals.var_dplim__blk979_dn6 * assign29790_body27_e42890), (locals.var_dplim__blk979_dn7 * assign29790_body27_e42890), (locals.var_dplim__blk979_dn10 * assign29790_body27_e42890), (locals.var_dplim__blk979_dn11 * assign29790_body27_e42890), (locals.var_dplim__blk979_dn12 * assign29790_body27_e42890), (locals.var_dplim__blk979_dn17 * assign29790_body27_e42890),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign29790_body27_e42893;
            locals.var_dps0_dn0 = assign29790_body27_e42893_d_n0;
            locals.var_dps0_dn2 = assign29790_body27_e42893_d_n2;
            locals.var_dps0_dn6 = assign29790_body27_e42893_d_n6;
            locals.var_dps0_dn7 = assign29790_body27_e42893_d_n7;
            locals.var_dps0_dn10 = assign29790_body27_e42893_d_n10;
            locals.var_dps0_dn11 = assign29790_body27_e42893_d_n11;
            locals.var_dps0_dn12 = assign29790_body27_e42893_d_n12;
            locals.var_dps0_dn17 = assign29790_body27_e42893_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign29790_body28_e42912, assign29790_body28_e42912_d_n0, assign29790_body28_e42912_d_n2, assign29790_body28_e42912_d_n6, assign29790_body28_e42912_d_n7, assign29790_body28_e42912_d_n10, assign29790_body28_e42912_d_n11, assign29790_body28_e42912_d_n12, assign29790_body28_e42912_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard996 == 0.0)) {
        let assign29790_body28_e42910: f64 = (locals.var_ps0ld__blk949 + locals.var_dps0);
        (assign29790_body28_e42910, (locals.var_ps0ld__blk949_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld__blk949_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld__blk949_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld__blk949_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld__blk949_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld__blk949_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld__blk949_dn12 + locals.var_dps0_dn12), (locals.var_ps0ld__blk949_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_ps0ld__blk949, locals.var_ps0ld__blk949_dn0, locals.var_ps0ld__blk949_dn2, locals.var_ps0ld__blk949_dn6, locals.var_ps0ld__blk949_dn7, locals.var_ps0ld__blk949_dn10, locals.var_ps0ld__blk949_dn11, locals.var_ps0ld__blk949_dn12, locals.var_ps0ld__blk949_dn17,)
    }
};
            locals.var_ps0ld__blk949 = assign29790_body28_e42912;
            locals.var_ps0ld__blk949_dn0 = assign29790_body28_e42912_d_n0;
            locals.var_ps0ld__blk949_dn2 = assign29790_body28_e42912_d_n2;
            locals.var_ps0ld__blk949_dn6 = assign29790_body28_e42912_d_n6;
            locals.var_ps0ld__blk949_dn7 = assign29790_body28_e42912_d_n7;
            locals.var_ps0ld__blk949_dn10 = assign29790_body28_e42912_d_n10;
            locals.var_ps0ld__blk949_dn11 = assign29790_body28_e42912_d_n11;
            locals.var_ps0ld__blk949_dn12 = assign29790_body28_e42912_d_n12;
            locals.var_ps0ld__blk949_dn17 = assign29790_body28_e42912_d_n17;
            locals.var_ps0ld__blk949_rv = 0.0;
            let assign29790_body29_e42914: f64 = (locals.var_dps0).abs();
            let assign29790_body29_e42918: f64 = (locals.var_fs0__blk977).abs();
            let assign29790_body29_e42921: f64 = if ((assign29790_body29_e42914 <= 5e-12) && (assign29790_body29_e42918 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard998 = assign29790_body29_e42921;
            locals.var_guard998_rv = 0.0;
            let (assign29790_body30_e42940,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard996 == 0.0)) && (locals.var_guard998 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv__blk922,)
    }
};
            locals.var_flg_conv__blk922 = assign29790_body30_e42940;
            locals.var_flg_conv__blk922_rv = 0.0;
            let (assign29790_body31_e42956,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        let assign29790_body31_e42954: f64 = (locals.var_lp_s0 + 1.0);
        (assign29790_body31_e42954,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign29790_body31_e42956;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_109(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign29810_e42962: f64 = if locals.var_chi__blk947 < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard1000 = assign29810_e42962;
        locals.var_guard1000_rv = 0.0;

        let (assign29850_e43024, assign29850_e43024_d_n0, assign29850_e43024_d_n2, assign29850_e43024_d_n6, assign29850_e43024_d_n7, assign29850_e43024_d_n10, assign29850_e43024_d_n11, assign29850_e43024_d_n12, assign29850_e43024_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard1000 != 0.0)) {
        let assign29850_e43018: f64 = (locals.var_fb__blk971 * locals.var_fb__blk971);
        let assign29850_e43021: f64 = (10.0 * 2.220446049250313e-16);
        let assign29850_e43022: f64 = (assign29850_e43018 + assign29850_e43021);
        (assign29850_e43022, ((locals.var_fb__blk971_dn0 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn0)), ((locals.var_fb__blk971_dn2 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn2)), ((locals.var_fb__blk971_dn6 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn6)), ((locals.var_fb__blk971_dn7 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn7)), ((locals.var_fb__blk971_dn10 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn10)), ((locals.var_fb__blk971_dn11 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn11)), ((locals.var_fb__blk971_dn12 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn12)), ((locals.var_fb__blk971_dn17 * locals.var_fb__blk971) + (locals.var_fb__blk971 * locals.var_fb__blk971_dn17)),)
    } else {
        (locals.var_xi0__blk980, locals.var_xi0__blk980_dn0, locals.var_xi0__blk980_dn2, locals.var_xi0__blk980_dn6, locals.var_xi0__blk980_dn7, locals.var_xi0__blk980_dn10, locals.var_xi0__blk980_dn11, locals.var_xi0__blk980_dn12, locals.var_xi0__blk980_dn17,)
    }
};
        locals.var_xi0__blk980 = assign29850_e43024;
        locals.var_xi0__blk980_dn0 = assign29850_e43024_d_n0;
        locals.var_xi0__blk980_dn2 = assign29850_e43024_d_n2;
        locals.var_xi0__blk980_dn6 = assign29850_e43024_d_n6;
        locals.var_xi0__blk980_dn7 = assign29850_e43024_d_n7;
        locals.var_xi0__blk980_dn10 = assign29850_e43024_d_n10;
        locals.var_xi0__blk980_dn11 = assign29850_e43024_d_n11;
        locals.var_xi0__blk980_dn12 = assign29850_e43024_d_n12;
        locals.var_xi0__blk980_dn17 = assign29850_e43024_d_n17;
        locals.var_xi0__blk980_rv = 0.0;

        let (assign29860_e43044, assign29860_e43044_d_n0, assign29860_e43044_d_n2, assign29860_e43044_d_n6, assign29860_e43044_d_n7, assign29860_e43044_d_n10, assign29860_e43044_d_n11, assign29860_e43044_d_n12, assign29860_e43044_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard1000 != 0.0)) {
        let assign29860_e43041: f64 = (10.0 * 2.220446049250313e-16);
        let assign29860_e43042: f64 = (locals.var_fb__blk971 + assign29860_e43041);
        (assign29860_e43042, locals.var_fb__blk971_dn0, locals.var_fb__blk971_dn2, locals.var_fb__blk971_dn6, locals.var_fb__blk971_dn7, locals.var_fb__blk971_dn10, locals.var_fb__blk971_dn11, locals.var_fb__blk971_dn12, locals.var_fb__blk971_dn17,)
    } else {
        (locals.var_xi0p12__blk981, locals.var_xi0p12__blk981_dn0, locals.var_xi0p12__blk981_dn2, locals.var_xi0p12__blk981_dn6, locals.var_xi0p12__blk981_dn7, locals.var_xi0p12__blk981_dn10, locals.var_xi0p12__blk981_dn11, locals.var_xi0p12__blk981_dn12, locals.var_xi0p12__blk981_dn17,)
    }
};
        locals.var_xi0p12__blk981 = assign29860_e43044;
        locals.var_xi0p12__blk981_dn0 = assign29860_e43044_d_n0;
        locals.var_xi0p12__blk981_dn2 = assign29860_e43044_d_n2;
        locals.var_xi0p12__blk981_dn6 = assign29860_e43044_d_n6;
        locals.var_xi0p12__blk981_dn7 = assign29860_e43044_d_n7;
        locals.var_xi0p12__blk981_dn10 = assign29860_e43044_d_n10;
        locals.var_xi0p12__blk981_dn11 = assign29860_e43044_d_n11;
        locals.var_xi0p12__blk981_dn12 = assign29860_e43044_d_n12;
        locals.var_xi0p12__blk981_dn17 = assign29860_e43044_d_n17;
        locals.var_xi0p12__blk981_rv = 0.0;

        let (assign29880_e43080, assign29880_e43080_d_n0, assign29880_e43080_d_n2, assign29880_e43080_d_n6, assign29880_e43080_d_n7, assign29880_e43080_d_n10, assign29880_e43080_d_n11, assign29880_e43080_d_n12, assign29880_e43080_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard1000 == 0.0)) {
        let assign29880_e43078: f64 = (locals.var_chi__blk947 - 1.0);
        (assign29880_e43078, locals.var_chi__blk947_dn0, locals.var_chi__blk947_dn2, locals.var_chi__blk947_dn6, locals.var_chi__blk947_dn7, locals.var_chi__blk947_dn10, locals.var_chi__blk947_dn11, locals.var_chi__blk947_dn12, locals.var_chi__blk947_dn17,)
    } else {
        (locals.var_xi0__blk980, locals.var_xi0__blk980_dn0, locals.var_xi0__blk980_dn2, locals.var_xi0__blk980_dn6, locals.var_xi0__blk980_dn7, locals.var_xi0__blk980_dn10, locals.var_xi0__blk980_dn11, locals.var_xi0__blk980_dn12, locals.var_xi0__blk980_dn17,)
    }
};
        locals.var_xi0__blk980 = assign29880_e43080;
        locals.var_xi0__blk980_dn0 = assign29880_e43080_d_n0;
        locals.var_xi0__blk980_dn2 = assign29880_e43080_d_n2;
        locals.var_xi0__blk980_dn6 = assign29880_e43080_d_n6;
        locals.var_xi0__blk980_dn7 = assign29880_e43080_d_n7;
        locals.var_xi0__blk980_dn10 = assign29880_e43080_d_n10;
        locals.var_xi0__blk980_dn11 = assign29880_e43080_d_n11;
        locals.var_xi0__blk980_dn12 = assign29880_e43080_d_n12;
        locals.var_xi0__blk980_dn17 = assign29880_e43080_d_n17;
        locals.var_xi0__blk980_rv = 0.0;

        let (assign29890_e43098, assign29890_e43098_d_n0, assign29890_e43098_d_n2, assign29890_e43098_d_n6, assign29890_e43098_d_n7, assign29890_e43098_d_n10, assign29890_e43098_d_n11, assign29890_e43098_d_n12, assign29890_e43098_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) && (locals.var_guard1000 == 0.0)) {
        let assign29890_e43096: f64 = (locals.var_xi0__blk980).sqrt();
        (assign29890_e43096, (locals.var_xi0__blk980_dn0 / (2.0 * assign29890_e43096)), (locals.var_xi0__blk980_dn2 / (2.0 * assign29890_e43096)), (locals.var_xi0__blk980_dn6 / (2.0 * assign29890_e43096)), (locals.var_xi0__blk980_dn7 / (2.0 * assign29890_e43096)), (locals.var_xi0__blk980_dn10 / (2.0 * assign29890_e43096)), (locals.var_xi0__blk980_dn11 / (2.0 * assign29890_e43096)), (locals.var_xi0__blk980_dn12 / (2.0 * assign29890_e43096)), (locals.var_xi0__blk980_dn17 / (2.0 * assign29890_e43096)),)
    } else {
        (locals.var_xi0p12__blk981, locals.var_xi0p12__blk981_dn0, locals.var_xi0p12__blk981_dn2, locals.var_xi0p12__blk981_dn6, locals.var_xi0p12__blk981_dn7, locals.var_xi0p12__blk981_dn10, locals.var_xi0p12__blk981_dn11, locals.var_xi0p12__blk981_dn12, locals.var_xi0p12__blk981_dn17,)
    }
};
        locals.var_xi0p12__blk981 = assign29890_e43098;
        locals.var_xi0p12__blk981_dn0 = assign29890_e43098_d_n0;
        locals.var_xi0p12__blk981_dn2 = assign29890_e43098_d_n2;
        locals.var_xi0p12__blk981_dn6 = assign29890_e43098_d_n6;
        locals.var_xi0p12__blk981_dn7 = assign29890_e43098_d_n7;
        locals.var_xi0p12__blk981_dn10 = assign29890_e43098_d_n10;
        locals.var_xi0p12__blk981_dn11 = assign29890_e43098_d_n11;
        locals.var_xi0p12__blk981_dn12 = assign29890_e43098_d_n12;
        locals.var_xi0p12__blk981_dn17 = assign29890_e43098_d_n17;
        locals.var_xi0p12__blk981_rv = 0.0;

        let (assign29900_e43114, assign29900_e43114_d_n0, assign29900_e43114_d_n2, assign29900_e43114_d_n6, assign29900_e43114_d_n7, assign29900_e43114_d_n10, assign29900_e43114_d_n11, assign29900_e43114_d_n12, assign29900_e43114_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        let assign29900_e43112: f64 = (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981);
        (assign29900_e43112, ((locals.var_cnst0over__blk932_dn0 * locals.var_xi0p12__blk981) + (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981_dn0)), ((locals.var_cnst0over__blk932_dn2 * locals.var_xi0p12__blk981) + (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981_dn2)), ((locals.var_cnst0over__blk932_dn6 * locals.var_xi0p12__blk981) + (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981_dn6)), ((locals.var_cnst0over__blk932_dn7 * locals.var_xi0p12__blk981) + (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981_dn7)), ((locals.var_cnst0over__blk932_dn10 * locals.var_xi0p12__blk981) + (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981_dn10)), ((locals.var_cnst0over__blk932_dn11 * locals.var_xi0p12__blk981) + (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981_dn11)), ((locals.var_cnst0over__blk932_dn12 * locals.var_xi0p12__blk981) + (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981_dn12)), ((locals.var_cnst0over__blk932_dn17 * locals.var_xi0p12__blk981) + (locals.var_cnst0over__blk932 * locals.var_xi0p12__blk981_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign29900_e43114;
        locals.var_qbuld_dn0 = assign29900_e43114_d_n0;
        locals.var_qbuld_dn2 = assign29900_e43114_d_n2;
        locals.var_qbuld_dn6 = assign29900_e43114_d_n6;
        locals.var_qbuld_dn7 = assign29900_e43114_d_n7;
        locals.var_qbuld_dn10 = assign29900_e43114_d_n10;
        locals.var_qbuld_dn11 = assign29900_e43114_d_n11;
        locals.var_qbuld_dn12 = assign29900_e43114_d_n12;
        locals.var_qbuld_dn17 = assign29900_e43114_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign29910_e43132, assign29910_e43132_d_n0, assign29910_e43132_d_n2, assign29910_e43132_d_n6, assign29910_e43132_d_n7, assign29910_e43132_d_n10, assign29910_e43132_d_n11, assign29910_e43132_d_n12, assign29910_e43132_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        let assign29910_e43129: f64 = (locals.var_fs02__blk973 + locals.var_xi0p12__blk981);
        let assign29910_e43130: f64 = (1.0 / assign29910_e43129);
        (assign29910_e43130, (-((locals.var_fs02__blk973_dn0 + locals.var_xi0p12__blk981_dn0) / (assign29910_e43129 * assign29910_e43129))), (-((locals.var_fs02__blk973_dn2 + locals.var_xi0p12__blk981_dn2) / (assign29910_e43129 * assign29910_e43129))), (-((locals.var_fs02__blk973_dn6 + locals.var_xi0p12__blk981_dn6) / (assign29910_e43129 * assign29910_e43129))), (-((locals.var_fs02__blk973_dn7 + locals.var_xi0p12__blk981_dn7) / (assign29910_e43129 * assign29910_e43129))), (-((locals.var_fs02__blk973_dn10 + locals.var_xi0p12__blk981_dn10) / (assign29910_e43129 * assign29910_e43129))), (-((locals.var_fs02__blk973_dn11 + locals.var_xi0p12__blk981_dn11) / (assign29910_e43129 * assign29910_e43129))), (-((locals.var_fs02__blk973_dn12 + locals.var_xi0p12__blk981_dn12) / (assign29910_e43129 * assign29910_e43129))), (-((locals.var_fs02__blk973_dn17 + locals.var_xi0p12__blk981_dn17) / (assign29910_e43129 * assign29910_e43129))),)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign29910_e43132;
        locals.var_t1__blk900_dn0 = assign29910_e43132_d_n0;
        locals.var_t1__blk900_dn2 = assign29910_e43132_d_n2;
        locals.var_t1__blk900_dn6 = assign29910_e43132_d_n6;
        locals.var_t1__blk900_dn7 = assign29910_e43132_d_n7;
        locals.var_t1__blk900_dn10 = assign29910_e43132_d_n10;
        locals.var_t1__blk900_dn11 = assign29910_e43132_d_n11;
        locals.var_t1__blk900_dn12 = assign29910_e43132_d_n12;
        locals.var_t1__blk900_dn17 = assign29910_e43132_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign29920_e43150, assign29920_e43150_d_n0, assign29920_e43150_d_n2, assign29920_e43150_d_n6, assign29920_e43150_d_n7, assign29920_e43150_d_n10, assign29920_e43150_d_n11, assign29920_e43150_d_n12, assign29920_e43150_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        let assign29920_e43146: f64 = (locals.var_cnst0over__blk932 * locals.var_fs01__blk969);
        let assign29920_e43148: f64 = (assign29920_e43146 * locals.var_t1__blk900);
        (assign29920_e43148, ((((locals.var_cnst0over__blk932_dn0 * locals.var_fs01__blk969) + (locals.var_cnst0over__blk932 * locals.var_fs01__blk969_dn0)) * locals.var_t1__blk900) + (assign29920_e43146 * locals.var_t1__blk900_dn0)), ((((locals.var_cnst0over__blk932_dn2 * locals.var_fs01__blk969) + (locals.var_cnst0over__blk932 * locals.var_fs01__blk969_dn2)) * locals.var_t1__blk900) + (assign29920_e43146 * locals.var_t1__blk900_dn2)), ((((locals.var_cnst0over__blk932_dn6 * locals.var_fs01__blk969) + (locals.var_cnst0over__blk932 * locals.var_fs01__blk969_dn6)) * locals.var_t1__blk900) + (assign29920_e43146 * locals.var_t1__blk900_dn6)), ((((locals.var_cnst0over__blk932_dn7 * locals.var_fs01__blk969) + (locals.var_cnst0over__blk932 * locals.var_fs01__blk969_dn7)) * locals.var_t1__blk900) + (assign29920_e43146 * locals.var_t1__blk900_dn7)), ((((locals.var_cnst0over__blk932_dn10 * locals.var_fs01__blk969) + (locals.var_cnst0over__blk932 * locals.var_fs01__blk969_dn10)) * locals.var_t1__blk900) + (assign29920_e43146 * locals.var_t1__blk900_dn10)), ((((locals.var_cnst0over__blk932_dn11 * locals.var_fs01__blk969) + (locals.var_cnst0over__blk932 * locals.var_fs01__blk969_dn11)) * locals.var_t1__blk900) + (assign29920_e43146 * locals.var_t1__blk900_dn11)), ((((locals.var_cnst0over__blk932_dn12 * locals.var_fs01__blk969) + (locals.var_cnst0over__blk932 * locals.var_fs01__blk969_dn12)) * locals.var_t1__blk900) + (assign29920_e43146 * locals.var_t1__blk900_dn12)), ((((locals.var_cnst0over__blk932_dn17 * locals.var_fs01__blk969) + (locals.var_cnst0over__blk932 * locals.var_fs01__blk969_dn17)) * locals.var_t1__blk900) + (assign29920_e43146 * locals.var_t1__blk900_dn17)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign29920_e43150;
        locals.var_qiuld_dn0 = assign29920_e43150_d_n0;
        locals.var_qiuld_dn2 = assign29920_e43150_d_n2;
        locals.var_qiuld_dn6 = assign29920_e43150_d_n6;
        locals.var_qiuld_dn7 = assign29920_e43150_d_n7;
        locals.var_qiuld_dn10 = assign29920_e43150_d_n10;
        locals.var_qiuld_dn11 = assign29920_e43150_d_n11;
        locals.var_qiuld_dn12 = assign29920_e43150_d_n12;
        locals.var_qiuld_dn17 = assign29920_e43150_d_n17;
        locals.var_qiuld_rv = 0.0;

        let (assign29930_e43166, assign29930_e43166_d_n0, assign29930_e43166_d_n2, assign29930_e43166_d_n6, assign29930_e43166_d_n7, assign29930_e43166_d_n10, assign29930_e43166_d_n11, assign29930_e43166_d_n12, assign29930_e43166_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard986 == 0.0)) && (locals.var_guard993 != 0.0)) {
        let assign29930_e43164: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign29930_e43164, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), (locals.var_qbuld_dn17 + locals.var_qiuld_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign29930_e43166;
        locals.var_qsuld_dn0 = assign29930_e43166_d_n0;
        locals.var_qsuld_dn2 = assign29930_e43166_d_n2;
        locals.var_qsuld_dn6 = assign29930_e43166_d_n6;
        locals.var_qsuld_dn7 = assign29930_e43166_d_n7;
        locals.var_qsuld_dn10 = assign29930_e43166_d_n10;
        locals.var_qsuld_dn11 = assign29930_e43166_d_n11;
        locals.var_qsuld_dn12 = assign29930_e43166_d_n12;
        locals.var_qsuld_dn17 = assign29930_e43166_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign29940_e43177, assign29940_e43177_d_n0, assign29940_e43177_d_n2, assign29940_e43177_d_n6, assign29940_e43177_d_n7, assign29940_e43177_d_n10, assign29940_e43177_d_n11, assign29940_e43177_d_n12, assign29940_e43177_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign29940_e43175: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign29940_e43175, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), (locals.var_qsuld_dn17 - locals.var_qbuld_dn17),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign29940_e43177;
        locals.var_qiuld_dn0 = assign29940_e43177_d_n0;
        locals.var_qiuld_dn2 = assign29940_e43177_d_n2;
        locals.var_qiuld_dn6 = assign29940_e43177_d_n6;
        locals.var_qiuld_dn7 = assign29940_e43177_d_n7;
        locals.var_qiuld_dn10 = assign29940_e43177_d_n10;
        locals.var_qiuld_dn11 = assign29940_e43177_d_n11;
        locals.var_qiuld_dn12 = assign29940_e43177_d_n12;
        locals.var_qiuld_dn17 = assign29940_e43177_d_n17;
        locals.var_qiuld_rv = 0.0;

        let (assign29950_e43195, assign29950_e43195_d_n0, assign29950_e43195_d_n2, assign29950_e43195_d_n6, assign29950_e43195_d_n7, assign29950_e43195_d_n10, assign29950_e43195_d_n11, assign29950_e43195_d_n12, assign29950_e43195_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let (assign29950_e43193,) = {
            if (p.p43 == 1.0) {
                let assign29950_e43189: f64 = (locals.var_w_dioscv * locals.var_lov);
                (assign29950_e43189,)
            } else {
                let assign29950_e43192: f64 = (locals.var_weffcv_nf * locals.var_lov);
                (assign29950_e43192,)
            }
        };
        (assign29950_e43193, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk903, locals.var_t4__blk903_dn0, locals.var_t4__blk903_dn2, locals.var_t4__blk903_dn6, locals.var_t4__blk903_dn7, locals.var_t4__blk903_dn10, locals.var_t4__blk903_dn11, locals.var_t4__blk903_dn12, locals.var_t4__blk903_dn17,)
    }
};
        locals.var_t4__blk903 = assign29950_e43195;
        locals.var_t4__blk903_dn0 = assign29950_e43195_d_n0;
        locals.var_t4__blk903_dn2 = assign29950_e43195_d_n2;
        locals.var_t4__blk903_dn6 = assign29950_e43195_d_n6;
        locals.var_t4__blk903_dn7 = assign29950_e43195_d_n7;
        locals.var_t4__blk903_dn10 = assign29950_e43195_d_n10;
        locals.var_t4__blk903_dn11 = assign29950_e43195_d_n11;
        locals.var_t4__blk903_dn12 = assign29950_e43195_d_n12;
        locals.var_t4__blk903_dn17 = assign29950_e43195_d_n17;
        locals.var_t4__blk903_rv = 0.0;

        let assign29960_e43206: f64 = if (((locals.var_flg_overs__blk918 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloops__blk916 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard1002 = assign29960_e43206;
        locals.var_guard1002_rv = 0.0;

        let (assign29970_e43219, assign29970_e43219_d_n0, assign29970_e43219_d_n2, assign29970_e43219_d_n6, assign29970_e43219_d_n7, assign29970_e43219_d_n10, assign29970_e43219_d_n11, assign29970_e43219_d_n12, assign29970_e43219_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign29970_e43217: f64 = (locals.var_t4__blk903 * locals.var_qsuld);
        (assign29970_e43217, ((locals.var_t4__blk903_dn0 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn0)), ((locals.var_t4__blk903_dn2 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn2)), ((locals.var_t4__blk903_dn6 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn6)), ((locals.var_t4__blk903_dn7 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn7)), ((locals.var_t4__blk903_dn10 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn10)), ((locals.var_t4__blk903_dn11 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn11)), ((locals.var_t4__blk903_dn12 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn12)), ((locals.var_t4__blk903_dn17 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn17)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn17,)
    }
};
        locals.var_qovs = assign29970_e43219;
        locals.var_qovs_dn0 = assign29970_e43219_d_n0;
        locals.var_qovs_dn2 = assign29970_e43219_d_n2;
        locals.var_qovs_dn6 = assign29970_e43219_d_n6;
        locals.var_qovs_dn7 = assign29970_e43219_d_n7;
        locals.var_qovs_dn10 = assign29970_e43219_d_n10;
        locals.var_qovs_dn11 = assign29970_e43219_d_n11;
        locals.var_qovs_dn12 = assign29970_e43219_d_n12;
        locals.var_qovs_dn17 = assign29970_e43219_d_n17;
        locals.var_qovs_rv = 0.0;

        let (assign29980_e43232, assign29980_e43232_d_n0, assign29980_e43232_d_n2, assign29980_e43232_d_n6, assign29980_e43232_d_n7, assign29980_e43232_d_n10, assign29980_e43232_d_n11, assign29980_e43232_d_n12, assign29980_e43232_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign29980_e43230: f64 = (locals.var_t4__blk903 * locals.var_qbuld);
        (assign29980_e43230, ((locals.var_t4__blk903_dn0 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn0)), ((locals.var_t4__blk903_dn2 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn2)), ((locals.var_t4__blk903_dn6 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn6)), ((locals.var_t4__blk903_dn7 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn7)), ((locals.var_t4__blk903_dn10 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn10)), ((locals.var_t4__blk903_dn11 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn11)), ((locals.var_t4__blk903_dn12 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn12)), ((locals.var_t4__blk903_dn17 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn17)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn10, locals.var_qbsld_dn11, locals.var_qbsld_dn12, locals.var_qbsld_dn17,)
    }
};
        locals.var_qbsld = assign29980_e43232;
        locals.var_qbsld_dn0 = assign29980_e43232_d_n0;
        locals.var_qbsld_dn2 = assign29980_e43232_d_n2;
        locals.var_qbsld_dn6 = assign29980_e43232_d_n6;
        locals.var_qbsld_dn7 = assign29980_e43232_d_n7;
        locals.var_qbsld_dn10 = assign29980_e43232_d_n10;
        locals.var_qbsld_dn11 = assign29980_e43232_d_n11;
        locals.var_qbsld_dn12 = assign29980_e43232_d_n12;
        locals.var_qbsld_dn17 = assign29980_e43232_d_n17;
        locals.var_qbsld_rv = 0.0;

        let assign29990_e43243: f64 = if (((locals.var_flg_overd__blk919 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloopd__blk917 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard1003 = assign29990_e43243;
        locals.var_guard1003_rv = 0.0;

        let (assign30000_e43256, assign30000_e43256_d_n0, assign30000_e43256_d_n2, assign30000_e43256_d_n6, assign30000_e43256_d_n7, assign30000_e43256_d_n10, assign30000_e43256_d_n11, assign30000_e43256_d_n12, assign30000_e43256_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1003 != 0.0)) {
        let assign30000_e43254: f64 = (locals.var_t4__blk903 * locals.var_qsuld);
        (assign30000_e43254, ((locals.var_t4__blk903_dn0 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn0)), ((locals.var_t4__blk903_dn2 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn2)), ((locals.var_t4__blk903_dn6 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn6)), ((locals.var_t4__blk903_dn7 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn7)), ((locals.var_t4__blk903_dn10 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn10)), ((locals.var_t4__blk903_dn11 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn11)), ((locals.var_t4__blk903_dn12 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn12)), ((locals.var_t4__blk903_dn17 * locals.var_qsuld) + (locals.var_t4__blk903 * locals.var_qsuld_dn17)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn17,)
    }
};
        locals.var_qovd = assign30000_e43256;
        locals.var_qovd_dn0 = assign30000_e43256_d_n0;
        locals.var_qovd_dn2 = assign30000_e43256_d_n2;
        locals.var_qovd_dn6 = assign30000_e43256_d_n6;
        locals.var_qovd_dn7 = assign30000_e43256_d_n7;
        locals.var_qovd_dn10 = assign30000_e43256_d_n10;
        locals.var_qovd_dn11 = assign30000_e43256_d_n11;
        locals.var_qovd_dn12 = assign30000_e43256_d_n12;
        locals.var_qovd_dn17 = assign30000_e43256_d_n17;
        locals.var_qovd_rv = 0.0;

        let (assign30010_e43269, assign30010_e43269_d_n0, assign30010_e43269_d_n2, assign30010_e43269_d_n6, assign30010_e43269_d_n7, assign30010_e43269_d_n10, assign30010_e43269_d_n11, assign30010_e43269_d_n12, assign30010_e43269_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1003 != 0.0)) {
        let assign30010_e43267: f64 = (locals.var_t4__blk903 * locals.var_qbuld);
        (assign30010_e43267, ((locals.var_t4__blk903_dn0 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn0)), ((locals.var_t4__blk903_dn2 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn2)), ((locals.var_t4__blk903_dn6 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn6)), ((locals.var_t4__blk903_dn7 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn7)), ((locals.var_t4__blk903_dn10 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn10)), ((locals.var_t4__blk903_dn11 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn11)), ((locals.var_t4__blk903_dn12 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn12)), ((locals.var_t4__blk903_dn17 * locals.var_qbuld) + (locals.var_t4__blk903 * locals.var_qbuld_dn17)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn12, locals.var_qbdld_dn17,)
    }
};
        locals.var_qbdld = assign30010_e43269;
        locals.var_qbdld_dn0 = assign30010_e43269_d_n0;
        locals.var_qbdld_dn2 = assign30010_e43269_d_n2;
        locals.var_qbdld_dn6 = assign30010_e43269_d_n6;
        locals.var_qbdld_dn7 = assign30010_e43269_d_n7;
        locals.var_qbdld_dn10 = assign30010_e43269_d_n10;
        locals.var_qbdld_dn11 = assign30010_e43269_d_n11;
        locals.var_qbdld_dn12 = assign30010_e43269_d_n12;
        locals.var_qbdld_dn17 = assign30010_e43269_d_n17;
        locals.var_qbdld_rv = 0.0;

        let (assign30020_e43282,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign30020_e43278: f64 = (1.0 - 1.0);
        let assign30020_e43280: f64 = (assign30020_e43278 / 2.0);
        (assign30020_e43280,)
    } else {
        (locals.var_flg_ovloops__blk916,)
    }
};
        locals.var_flg_ovloops__blk916 = assign30020_e43282;
        locals.var_flg_ovloops__blk916_rv = 0.0;

        let (assign30030_e43295,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign30030_e43291: f64 = (1.0 + 1.0);
        let assign30030_e43293: f64 = (assign30030_e43291 / 2.0);
        (assign30030_e43293,)
    } else {
        (locals.var_flg_ovloopd__blk917,)
    }
};
        locals.var_flg_ovloopd__blk917 = assign30030_e43295;
        locals.var_flg_ovloopd__blk917_rv = 0.0;

        let assign30040_e43298: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1004 = assign30040_e43298;
        locals.var_guard1004_rv = 0.0;

        let (assign30050_e43317, assign30050_e43317_d_n0, assign30050_e43317_d_n2, assign30050_e43317_d_n6, assign30050_e43317_d_n7, assign30050_e43317_d_n10, assign30050_e43317_d_n11, assign30050_e43317_d_n12, assign30050_e43317_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1004 != 0.0)) {
        let assign30050_e43309: f64 = (locals.var_modenml * locals.var_vbs);
        let assign30050_e43313: f64 = (locals.var_vbs - locals.var_vds);
        let assign30050_e43314: f64 = (locals.var_modervs * assign30050_e43313);
        let assign30050_e43315: f64 = (assign30050_e43309 + assign30050_e43314);
        (assign30050_e43315, ((locals.var_modenml * locals.var_vbs_dn0) + (locals.var_modervs * (locals.var_vbs_dn0 - locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vbs_dn2) + (locals.var_modervs * (locals.var_vbs_dn2 - locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vbs_dn6) + (locals.var_modervs * (locals.var_vbs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vbs_dn7) + (locals.var_modervs * (locals.var_vbs_dn7 - locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vbs_dn10) + (locals.var_modervs * (locals.var_vbs_dn10 - locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vbs_dn11) + (locals.var_modervs * (locals.var_vbs_dn11 - locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vbs_dn12) + (locals.var_modervs * (locals.var_vbs_dn12 - locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vbs_dn17) + (locals.var_modervs * (locals.var_vbs_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_vbsgmt__blk926, locals.var_vbsgmt__blk926_dn0, locals.var_vbsgmt__blk926_dn2, locals.var_vbsgmt__blk926_dn6, locals.var_vbsgmt__blk926_dn7, locals.var_vbsgmt__blk926_dn10, locals.var_vbsgmt__blk926_dn11, locals.var_vbsgmt__blk926_dn12, locals.var_vbsgmt__blk926_dn17,)
    }
};
        locals.var_vbsgmt__blk926 = assign30050_e43317;
        locals.var_vbsgmt__blk926_dn0 = assign30050_e43317_d_n0;
        locals.var_vbsgmt__blk926_dn2 = assign30050_e43317_d_n2;
        locals.var_vbsgmt__blk926_dn6 = assign30050_e43317_d_n6;
        locals.var_vbsgmt__blk926_dn7 = assign30050_e43317_d_n7;
        locals.var_vbsgmt__blk926_dn10 = assign30050_e43317_d_n10;
        locals.var_vbsgmt__blk926_dn11 = assign30050_e43317_d_n11;
        locals.var_vbsgmt__blk926_dn12 = assign30050_e43317_d_n12;
        locals.var_vbsgmt__blk926_dn17 = assign30050_e43317_d_n17;
        locals.var_vbsgmt__blk926_rv = 0.0;

        let (assign30060_e43335, assign30060_e43335_d_n0, assign30060_e43335_d_n2, assign30060_e43335_d_n6, assign30060_e43335_d_n7, assign30060_e43335_d_n10, assign30060_e43335_d_n11, assign30060_e43335_d_n12, assign30060_e43335_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1004 != 0.0)) {
        let assign30060_e43328: f64 = (locals.var_modenml * locals.var_vds);
        let assign30060_e43331: f64 = (-locals.var_vds);
        let assign30060_e43332: f64 = (locals.var_modervs * assign30060_e43331);
        let assign30060_e43333: f64 = (assign30060_e43328 + assign30060_e43332);
        (assign30060_e43333, ((locals.var_modenml * locals.var_vds_dn0) + (locals.var_modervs * (-locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vds_dn2) + (locals.var_modervs * (-locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vds_dn6) + (locals.var_modervs * (-locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vds_dn7) + (locals.var_modervs * (-locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vds_dn10) + (locals.var_modervs * (-locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vds_dn11) + (locals.var_modervs * (-locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vds_dn12) + (locals.var_modervs * (-locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vds_dn17) + (locals.var_modervs * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_vdsgmt__blk927, locals.var_vdsgmt__blk927_dn0, locals.var_vdsgmt__blk927_dn2, locals.var_vdsgmt__blk927_dn6, locals.var_vdsgmt__blk927_dn7, locals.var_vdsgmt__blk927_dn10, locals.var_vdsgmt__blk927_dn11, locals.var_vdsgmt__blk927_dn12, locals.var_vdsgmt__blk927_dn17,)
    }
};
        locals.var_vdsgmt__blk927 = assign30060_e43335;
        locals.var_vdsgmt__blk927_dn0 = assign30060_e43335_d_n0;
        locals.var_vdsgmt__blk927_dn2 = assign30060_e43335_d_n2;
        locals.var_vdsgmt__blk927_dn6 = assign30060_e43335_d_n6;
        locals.var_vdsgmt__blk927_dn7 = assign30060_e43335_d_n7;
        locals.var_vdsgmt__blk927_dn10 = assign30060_e43335_d_n10;
        locals.var_vdsgmt__blk927_dn11 = assign30060_e43335_d_n11;
        locals.var_vdsgmt__blk927_dn12 = assign30060_e43335_d_n12;
        locals.var_vdsgmt__blk927_dn17 = assign30060_e43335_d_n17;
        locals.var_vdsgmt__blk927_rv = 0.0;

        let (assign30070_e43354, assign30070_e43354_d_n0, assign30070_e43354_d_n2, assign30070_e43354_d_n6, assign30070_e43354_d_n7, assign30070_e43354_d_n10, assign30070_e43354_d_n11, assign30070_e43354_d_n12, assign30070_e43354_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1004 != 0.0)) {
        let assign30070_e43346: f64 = (locals.var_modenml * locals.var_vgs);
        let assign30070_e43350: f64 = (locals.var_vgs - locals.var_vds);
        let assign30070_e43351: f64 = (locals.var_modervs * assign30070_e43350);
        let assign30070_e43352: f64 = (assign30070_e43346 + assign30070_e43351);
        (assign30070_e43352, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgsgmt__blk928, locals.var_vgsgmt__blk928_dn0, locals.var_vgsgmt__blk928_dn2, locals.var_vgsgmt__blk928_dn6, locals.var_vgsgmt__blk928_dn7, locals.var_vgsgmt__blk928_dn10, locals.var_vgsgmt__blk928_dn11, locals.var_vgsgmt__blk928_dn12, locals.var_vgsgmt__blk928_dn17,)
    }
};
        locals.var_vgsgmt__blk928 = assign30070_e43354;
        locals.var_vgsgmt__blk928_dn0 = assign30070_e43354_d_n0;
        locals.var_vgsgmt__blk928_dn2 = assign30070_e43354_d_n2;
        locals.var_vgsgmt__blk928_dn6 = assign30070_e43354_d_n6;
        locals.var_vgsgmt__blk928_dn7 = assign30070_e43354_d_n7;
        locals.var_vgsgmt__blk928_dn10 = assign30070_e43354_d_n10;
        locals.var_vgsgmt__blk928_dn11 = assign30070_e43354_d_n11;
        locals.var_vgsgmt__blk928_dn12 = assign30070_e43354_d_n12;
        locals.var_vgsgmt__blk928_dn17 = assign30070_e43354_d_n17;
        locals.var_vgsgmt__blk928_rv = 0.0;

        let (assign30080_e43367, assign30080_e43367_d_n0, assign30080_e43367_d_n2, assign30080_e43367_d_n6, assign30080_e43367_d_n7, assign30080_e43367_d_n10, assign30080_e43367_d_n11, assign30080_e43367_d_n12, assign30080_e43367_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1004 != 0.0)) {
        let assign30080_e43365: f64 = (locals.var_vdsgmt__blk927 - locals.var_vbsgmt__blk926);
        (assign30080_e43365, (locals.var_vdsgmt__blk927_dn0 - locals.var_vbsgmt__blk926_dn0), (locals.var_vdsgmt__blk927_dn2 - locals.var_vbsgmt__blk926_dn2), (locals.var_vdsgmt__blk927_dn6 - locals.var_vbsgmt__blk926_dn6), (locals.var_vdsgmt__blk927_dn7 - locals.var_vbsgmt__blk926_dn7), (locals.var_vdsgmt__blk927_dn10 - locals.var_vbsgmt__blk926_dn10), (locals.var_vdsgmt__blk927_dn11 - locals.var_vbsgmt__blk926_dn11), (locals.var_vdsgmt__blk927_dn12 - locals.var_vbsgmt__blk926_dn12), (locals.var_vdsgmt__blk927_dn17 - locals.var_vbsgmt__blk926_dn17),)
    } else {
        (locals.var_vdbgmt__blk929, locals.var_vdbgmt__blk929_dn0, locals.var_vdbgmt__blk929_dn2, locals.var_vdbgmt__blk929_dn6, locals.var_vdbgmt__blk929_dn7, locals.var_vdbgmt__blk929_dn10, locals.var_vdbgmt__blk929_dn11, locals.var_vdbgmt__blk929_dn12, locals.var_vdbgmt__blk929_dn17,)
    }
};
        locals.var_vdbgmt__blk929 = assign30080_e43367;
        locals.var_vdbgmt__blk929_dn0 = assign30080_e43367_d_n0;
        locals.var_vdbgmt__blk929_dn2 = assign30080_e43367_d_n2;
        locals.var_vdbgmt__blk929_dn6 = assign30080_e43367_d_n6;
        locals.var_vdbgmt__blk929_dn7 = assign30080_e43367_d_n7;
        locals.var_vdbgmt__blk929_dn10 = assign30080_e43367_d_n10;
        locals.var_vdbgmt__blk929_dn11 = assign30080_e43367_d_n11;
        locals.var_vdbgmt__blk929_dn12 = assign30080_e43367_d_n12;
        locals.var_vdbgmt__blk929_dn17 = assign30080_e43367_d_n17;
        locals.var_vdbgmt__blk929_rv = 0.0;

        let (assign30090_e43380, assign30090_e43380_d_n0, assign30090_e43380_d_n2, assign30090_e43380_d_n6, assign30090_e43380_d_n7, assign30090_e43380_d_n10, assign30090_e43380_d_n11, assign30090_e43380_d_n12, assign30090_e43380_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1004 != 0.0)) {
        let assign30090_e43378: f64 = (locals.var_vgsgmt__blk928 - locals.var_vbsgmt__blk926);
        (assign30090_e43378, (locals.var_vgsgmt__blk928_dn0 - locals.var_vbsgmt__blk926_dn0), (locals.var_vgsgmt__blk928_dn2 - locals.var_vbsgmt__blk926_dn2), (locals.var_vgsgmt__blk928_dn6 - locals.var_vbsgmt__blk926_dn6), (locals.var_vgsgmt__blk928_dn7 - locals.var_vbsgmt__blk926_dn7), (locals.var_vgsgmt__blk928_dn10 - locals.var_vbsgmt__blk926_dn10), (locals.var_vgsgmt__blk928_dn11 - locals.var_vbsgmt__blk926_dn11), (locals.var_vgsgmt__blk928_dn12 - locals.var_vbsgmt__blk926_dn12), (locals.var_vgsgmt__blk928_dn17 - locals.var_vbsgmt__blk926_dn17),)
    } else {
        (locals.var_vgbgmt__blk931, locals.var_vgbgmt__blk931_dn0, locals.var_vgbgmt__blk931_dn2, locals.var_vgbgmt__blk931_dn6, locals.var_vgbgmt__blk931_dn7, locals.var_vgbgmt__blk931_dn10, locals.var_vgbgmt__blk931_dn11, locals.var_vgbgmt__blk931_dn12, locals.var_vgbgmt__blk931_dn17,)
    }
};
        locals.var_vgbgmt__blk931 = assign30090_e43380;
        locals.var_vgbgmt__blk931_dn0 = assign30090_e43380_d_n0;
        locals.var_vgbgmt__blk931_dn2 = assign30090_e43380_d_n2;
        locals.var_vgbgmt__blk931_dn6 = assign30090_e43380_d_n6;
        locals.var_vgbgmt__blk931_dn7 = assign30090_e43380_d_n7;
        locals.var_vgbgmt__blk931_dn10 = assign30090_e43380_d_n10;
        locals.var_vgbgmt__blk931_dn11 = assign30090_e43380_d_n11;
        locals.var_vgbgmt__blk931_dn12 = assign30090_e43380_d_n12;
        locals.var_vgbgmt__blk931_dn17 = assign30090_e43380_d_n17;
        locals.var_vgbgmt__blk931_rv = 0.0;

        let (assign30100_e43392, assign30100_e43392_d_n0, assign30100_e43392_d_n2, assign30100_e43392_d_n6, assign30100_e43392_d_n7, assign30100_e43392_d_n10, assign30100_e43392_d_n11, assign30100_e43392_d_n12, assign30100_e43392_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1004 != 0.0)) {
        let assign30100_e43390: f64 = (-locals.var_vbsgmt__blk926);
        (assign30100_e43390, (-locals.var_vbsgmt__blk926_dn0), (-locals.var_vbsgmt__blk926_dn2), (-locals.var_vbsgmt__blk926_dn6), (-locals.var_vbsgmt__blk926_dn7), (-locals.var_vbsgmt__blk926_dn10), (-locals.var_vbsgmt__blk926_dn11), (-locals.var_vbsgmt__blk926_dn12), (-locals.var_vbsgmt__blk926_dn17),)
    } else {
        (locals.var_vsbgmt__blk930, locals.var_vsbgmt__blk930_dn0, locals.var_vsbgmt__blk930_dn2, locals.var_vsbgmt__blk930_dn6, locals.var_vsbgmt__blk930_dn7, locals.var_vsbgmt__blk930_dn10, locals.var_vsbgmt__blk930_dn11, locals.var_vsbgmt__blk930_dn12, locals.var_vsbgmt__blk930_dn17,)
    }
};
        locals.var_vsbgmt__blk930 = assign30100_e43392;
        locals.var_vsbgmt__blk930_dn0 = assign30100_e43392_d_n0;
        locals.var_vsbgmt__blk930_dn2 = assign30100_e43392_d_n2;
        locals.var_vsbgmt__blk930_dn6 = assign30100_e43392_d_n6;
        locals.var_vsbgmt__blk930_dn7 = assign30100_e43392_d_n7;
        locals.var_vsbgmt__blk930_dn10 = assign30100_e43392_d_n10;
        locals.var_vsbgmt__blk930_dn11 = assign30100_e43392_d_n11;
        locals.var_vsbgmt__blk930_dn12 = assign30100_e43392_d_n12;
        locals.var_vsbgmt__blk930_dn17 = assign30100_e43392_d_n17;
        locals.var_vsbgmt__blk930_rv = 0.0;

        let (assign30110_e43409,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1004 != 0.0)) {
        let assign30110_e43403: f64 = (locals.var_flg_ovloops__blk916 * locals.var_modenml);
        let assign30110_e43406: f64 = (locals.var_flg_ovloopd__blk917 * locals.var_modervs);
        let assign30110_e43407: f64 = (assign30110_e43403 + assign30110_e43406);
        (assign30110_e43407,)
    } else {
        (locals.var_flg_overs__blk918,)
    }
};
        locals.var_flg_overs__blk918 = assign30110_e43409;
        locals.var_flg_overs__blk918_rv = 0.0;

        let (assign30120_e43426,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1004 != 0.0)) {
        let assign30120_e43420: f64 = (locals.var_flg_ovloops__blk916 * locals.var_modervs);
        let assign30120_e43423: f64 = (locals.var_flg_ovloopd__blk917 * locals.var_modenml);
        let assign30120_e43424: f64 = (assign30120_e43420 + assign30120_e43423);
        (assign30120_e43424,)
    } else {
        (locals.var_flg_overd__blk919,)
    }
};
        locals.var_flg_overd__blk919 = assign30120_e43426;
        locals.var_flg_overd__blk919_rv = 0.0;

        let (assign30130_e43447, assign30130_e43447_d_n0, assign30130_e43447_d_n2, assign30130_e43447_d_n6, assign30130_e43447_d_n7, assign30130_e43447_d_n10, assign30130_e43447_d_n11, assign30130_e43447_d_n12, assign30130_e43447_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1004 != 0.0)) {
        let assign30130_e43437: f64 = (locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930);
        let assign30130_e43440: f64 = (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929);
        let assign30130_e43441: f64 = (assign30130_e43437 + assign30130_e43440);
        let assign30130_e43444: f64 = (10.0 * 2.220446049250313e-16);
        let assign30130_e43445: f64 = (assign30130_e43441 + assign30130_e43444);
        (assign30130_e43445, ((locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930_dn0) + (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929_dn0)), ((locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930_dn2) + (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929_dn2)), ((locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930_dn6) + (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929_dn6)), ((locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930_dn7) + (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929_dn7)), ((locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930_dn10) + (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929_dn10)), ((locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930_dn11) + (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929_dn11)), ((locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930_dn12) + (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929_dn12)), ((locals.var_flg_overs__blk918 * locals.var_vsbgmt__blk930_dn17) + (locals.var_flg_overd__blk919 * locals.var_vdbgmt__blk929_dn17)),)
    } else {
        (locals.var_vxbgmt__blk924, locals.var_vxbgmt__blk924_dn0, locals.var_vxbgmt__blk924_dn2, locals.var_vxbgmt__blk924_dn6, locals.var_vxbgmt__blk924_dn7, locals.var_vxbgmt__blk924_dn10, locals.var_vxbgmt__blk924_dn11, locals.var_vxbgmt__blk924_dn12, locals.var_vxbgmt__blk924_dn17,)
    }
};
        locals.var_vxbgmt__blk924 = assign30130_e43447;
        locals.var_vxbgmt__blk924_dn0 = assign30130_e43447_d_n0;
        locals.var_vxbgmt__blk924_dn2 = assign30130_e43447_d_n2;
        locals.var_vxbgmt__blk924_dn6 = assign30130_e43447_d_n6;
        locals.var_vxbgmt__blk924_dn7 = assign30130_e43447_d_n7;
        locals.var_vxbgmt__blk924_dn10 = assign30130_e43447_d_n10;
        locals.var_vxbgmt__blk924_dn11 = assign30130_e43447_d_n11;
        locals.var_vxbgmt__blk924_dn12 = assign30130_e43447_d_n12;
        locals.var_vxbgmt__blk924_dn17 = assign30130_e43447_d_n17;
        locals.var_vxbgmt__blk924_rv = 0.0;

        let (assign30140_e43465,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1004 == 0.0)) {
        let assign30140_e43459: f64 = (locals.var_flg_ovloops__blk916 * locals.var_modenml);
        let assign30140_e43462: f64 = (locals.var_flg_ovloopd__blk917 * locals.var_modervs);
        let assign30140_e43463: f64 = (assign30140_e43459 + assign30140_e43462);
        (assign30140_e43463,)
    } else {
        (locals.var_flg_overs__blk918,)
    }
};
        locals.var_flg_overs__blk918 = assign30140_e43465;
        locals.var_flg_overs__blk918_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_110(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30150_e43483,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1004 == 0.0)) {
        let assign30150_e43477: f64 = (locals.var_flg_ovloops__blk916 * locals.var_modervs);
        let assign30150_e43480: f64 = (locals.var_flg_ovloopd__blk917 * locals.var_modenml);
        let assign30150_e43481: f64 = (assign30150_e43477 + assign30150_e43480);
        (assign30150_e43481,)
    } else {
        (locals.var_flg_overd__blk919,)
    }
};
        locals.var_flg_overd__blk919 = assign30150_e43483;
        locals.var_flg_overd__blk919_rv = 0.0;

        let (assign30160_e43505, assign30160_e43505_d_n0, assign30160_e43505_d_n2, assign30160_e43505_d_n6, assign30160_e43505_d_n7, assign30160_e43505_d_n10, assign30160_e43505_d_n11, assign30160_e43505_d_n12, assign30160_e43505_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_flg_ovloops__blk916 != 0.0)) {
        let assign30160_e43497: f64 = (locals.var_modenml * locals.var_vgs);
        let assign30160_e43501: f64 = (locals.var_vgs - locals.var_vds);
        let assign30160_e43502: f64 = (locals.var_modervs * assign30160_e43501);
        let assign30160_e43503: f64 = (assign30160_e43497 + assign30160_e43502);
        (assign30160_e43503, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgbgmt__blk931, locals.var_vgbgmt__blk931_dn0, locals.var_vgbgmt__blk931_dn2, locals.var_vgbgmt__blk931_dn6, locals.var_vgbgmt__blk931_dn7, locals.var_vgbgmt__blk931_dn10, locals.var_vgbgmt__blk931_dn11, locals.var_vgbgmt__blk931_dn12, locals.var_vgbgmt__blk931_dn17,)
    }
};
        locals.var_vgbgmt__blk931 = assign30160_e43505;
        locals.var_vgbgmt__blk931_dn0 = assign30160_e43505_d_n0;
        locals.var_vgbgmt__blk931_dn2 = assign30160_e43505_d_n2;
        locals.var_vgbgmt__blk931_dn6 = assign30160_e43505_d_n6;
        locals.var_vgbgmt__blk931_dn7 = assign30160_e43505_d_n7;
        locals.var_vgbgmt__blk931_dn10 = assign30160_e43505_d_n10;
        locals.var_vgbgmt__blk931_dn11 = assign30160_e43505_d_n11;
        locals.var_vgbgmt__blk931_dn12 = assign30160_e43505_d_n12;
        locals.var_vgbgmt__blk931_dn17 = assign30160_e43505_d_n17;
        locals.var_vgbgmt__blk931_rv = 0.0;

        let (assign30170_e43527, assign30170_e43527_d_n0, assign30170_e43527_d_n2, assign30170_e43527_d_n6, assign30170_e43527_d_n7, assign30170_e43527_d_n10, assign30170_e43527_d_n11, assign30170_e43527_d_n12, assign30170_e43527_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1004 == 0.0)) && (locals.var_flg_ovloopd__blk917 != 0.0)) {
        let assign30170_e43519: f64 = (locals.var_modervs * locals.var_vgs);
        let assign30170_e43523: f64 = (locals.var_vgs - locals.var_vds);
        let assign30170_e43524: f64 = (locals.var_modenml * assign30170_e43523);
        let assign30170_e43525: f64 = (assign30170_e43519 + assign30170_e43524);
        (assign30170_e43525, (locals.var_modenml * (-locals.var_vds_dn0)), (locals.var_modenml * (-locals.var_vds_dn2)), ((locals.var_modervs * locals.var_vgs_dn6) + (locals.var_modenml * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modervs * locals.var_vgs_dn7) + (locals.var_modenml * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modenml * (-locals.var_vds_dn10)), ((locals.var_modervs * locals.var_vgs_dn11) + (locals.var_modenml * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modenml * (-locals.var_vds_dn12)), (locals.var_modenml * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgbgmt__blk931, locals.var_vgbgmt__blk931_dn0, locals.var_vgbgmt__blk931_dn2, locals.var_vgbgmt__blk931_dn6, locals.var_vgbgmt__blk931_dn7, locals.var_vgbgmt__blk931_dn10, locals.var_vgbgmt__blk931_dn11, locals.var_vgbgmt__blk931_dn12, locals.var_vgbgmt__blk931_dn17,)
    }
};
        locals.var_vgbgmt__blk931 = assign30170_e43527;
        locals.var_vgbgmt__blk931_dn0 = assign30170_e43527_d_n0;
        locals.var_vgbgmt__blk931_dn2 = assign30170_e43527_d_n2;
        locals.var_vgbgmt__blk931_dn6 = assign30170_e43527_d_n6;
        locals.var_vgbgmt__blk931_dn7 = assign30170_e43527_d_n7;
        locals.var_vgbgmt__blk931_dn10 = assign30170_e43527_d_n10;
        locals.var_vgbgmt__blk931_dn11 = assign30170_e43527_d_n11;
        locals.var_vgbgmt__blk931_dn12 = assign30170_e43527_d_n12;
        locals.var_vgbgmt__blk931_dn17 = assign30170_e43527_d_n17;
        locals.var_vgbgmt__blk931_rv = 0.0;

        let (assign30180_e43539, assign30180_e43539_d_n0, assign30180_e43539_d_n2, assign30180_e43539_d_n6, assign30180_e43539_d_n7, assign30180_e43539_d_n10, assign30180_e43539_d_n11, assign30180_e43539_d_n12, assign30180_e43539_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1004 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt__blk924, locals.var_vxbgmt__blk924_dn0, locals.var_vxbgmt__blk924_dn2, locals.var_vxbgmt__blk924_dn6, locals.var_vxbgmt__blk924_dn7, locals.var_vxbgmt__blk924_dn10, locals.var_vxbgmt__blk924_dn11, locals.var_vxbgmt__blk924_dn12, locals.var_vxbgmt__blk924_dn17,)
    }
};
        locals.var_vxbgmt__blk924 = assign30180_e43539;
        locals.var_vxbgmt__blk924_dn0 = assign30180_e43539_d_n0;
        locals.var_vxbgmt__blk924_dn2 = assign30180_e43539_d_n2;
        locals.var_vxbgmt__blk924_dn6 = assign30180_e43539_d_n6;
        locals.var_vxbgmt__blk924_dn7 = assign30180_e43539_d_n7;
        locals.var_vxbgmt__blk924_dn10 = assign30180_e43539_d_n10;
        locals.var_vxbgmt__blk924_dn11 = assign30180_e43539_d_n11;
        locals.var_vxbgmt__blk924_dn12 = assign30180_e43539_d_n12;
        locals.var_vxbgmt__blk924_dn17 = assign30180_e43539_d_n17;
        locals.var_vxbgmt__blk924_rv = 0.0;

        let (assign30190_e43549, assign30190_e43549_d_n0, assign30190_e43549_d_n2, assign30190_e43549_d_n6, assign30190_e43549_d_n7, assign30190_e43549_d_n10, assign30190_e43549_d_n11, assign30190_e43549_d_n12, assign30190_e43549_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign30190_e43547: f64 = (-locals.var_vxbgmt__blk924);
        (assign30190_e43547, (-locals.var_vxbgmt__blk924_dn0), (-locals.var_vxbgmt__blk924_dn2), (-locals.var_vxbgmt__blk924_dn6), (-locals.var_vxbgmt__blk924_dn7), (-locals.var_vxbgmt__blk924_dn10), (-locals.var_vxbgmt__blk924_dn11), (-locals.var_vxbgmt__blk924_dn12), (-locals.var_vxbgmt__blk924_dn17),)
    } else {
        (locals.var_t0__blk899, locals.var_t0__blk899_dn0, locals.var_t0__blk899_dn2, locals.var_t0__blk899_dn6, locals.var_t0__blk899_dn7, locals.var_t0__blk899_dn10, locals.var_t0__blk899_dn11, locals.var_t0__blk899_dn12, locals.var_t0__blk899_dn17,)
    }
};
        locals.var_t0__blk899 = assign30190_e43549;
        locals.var_t0__blk899_dn0 = assign30190_e43549_d_n0;
        locals.var_t0__blk899_dn2 = assign30190_e43549_d_n2;
        locals.var_t0__blk899_dn6 = assign30190_e43549_d_n6;
        locals.var_t0__blk899_dn7 = assign30190_e43549_d_n7;
        locals.var_t0__blk899_dn10 = assign30190_e43549_d_n10;
        locals.var_t0__blk899_dn11 = assign30190_e43549_d_n11;
        locals.var_t0__blk899_dn12 = assign30190_e43549_d_n12;
        locals.var_t0__blk899_dn17 = assign30190_e43549_d_n17;
        locals.var_t0__blk899_rv = 0.0;

        let assign30200_e43552: f64 = if locals.var_t0__blk899 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard1005 = assign30200_e43552;
        locals.var_guard1005_rv = 0.0;

        let (assign30210_e43565, assign30210_e43565_d_n0, assign30210_e43565_d_n2, assign30210_e43565_d_n6, assign30210_e43565_d_n7, assign30210_e43565_d_n10, assign30210_e43565_d_n11, assign30210_e43565_d_n12, assign30210_e43565_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1005 != 0.0)) {
        let assign30210_e43563: f64 = (locals.var_t0__blk899 - locals.var_vbs_bnd);
        (assign30210_e43563, locals.var_t0__blk899_dn0, locals.var_t0__blk899_dn2, locals.var_t0__blk899_dn6, locals.var_t0__blk899_dn7, locals.var_t0__blk899_dn10, locals.var_t0__blk899_dn11, locals.var_t0__blk899_dn12, locals.var_t0__blk899_dn17,)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign30210_e43565;
        locals.var_t1__blk900_dn0 = assign30210_e43565_d_n0;
        locals.var_t1__blk900_dn2 = assign30210_e43565_d_n2;
        locals.var_t1__blk900_dn6 = assign30210_e43565_d_n6;
        locals.var_t1__blk900_dn7 = assign30210_e43565_d_n7;
        locals.var_t1__blk900_dn10 = assign30210_e43565_d_n10;
        locals.var_t1__blk900_dn11 = assign30210_e43565_d_n11;
        locals.var_t1__blk900_dn12 = assign30210_e43565_d_n12;
        locals.var_t1__blk900_dn17 = assign30210_e43565_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign30220_e43578, assign30220_e43578_d_n0, assign30220_e43578_d_n2, assign30220_e43578_d_n6, assign30220_e43578_d_n7, assign30220_e43578_d_n10, assign30220_e43578_d_n11, assign30220_e43578_d_n12, assign30220_e43578_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1005 != 0.0)) {
        let assign30220_e43576: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign30220_e43576, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk901, locals.var_t2__blk901_dn0, locals.var_t2__blk901_dn2, locals.var_t2__blk901_dn6, locals.var_t2__blk901_dn7, locals.var_t2__blk901_dn10, locals.var_t2__blk901_dn11, locals.var_t2__blk901_dn12, locals.var_t2__blk901_dn17,)
    }
};
        locals.var_t2__blk901 = assign30220_e43578;
        locals.var_t2__blk901_dn0 = assign30220_e43578_d_n0;
        locals.var_t2__blk901_dn2 = assign30220_e43578_d_n2;
        locals.var_t2__blk901_dn6 = assign30220_e43578_d_n6;
        locals.var_t2__blk901_dn7 = assign30220_e43578_d_n7;
        locals.var_t2__blk901_dn10 = assign30220_e43578_d_n10;
        locals.var_t2__blk901_dn11 = assign30220_e43578_d_n11;
        locals.var_t2__blk901_dn12 = assign30220_e43578_d_n12;
        locals.var_t2__blk901_dn17 = assign30220_e43578_d_n17;
        locals.var_t2__blk901_rv = 0.0;

        let (assign30230_e43591, assign30230_e43591_d_n0, assign30230_e43591_d_n2, assign30230_e43591_d_n6, assign30230_e43591_d_n7, assign30230_e43591_d_n10, assign30230_e43591_d_n11, assign30230_e43591_d_n12, assign30230_e43591_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1005 != 0.0)) {
        let assign30230_e43589: f64 = (locals.var_t1__blk900 / locals.var_t2__blk901);
        (assign30230_e43589, (((locals.var_t1__blk900_dn0 * locals.var_t2__blk901) - (locals.var_t1__blk900 * locals.var_t2__blk901_dn0)) / (locals.var_t2__blk901 * locals.var_t2__blk901)), (((locals.var_t1__blk900_dn2 * locals.var_t2__blk901) - (locals.var_t1__blk900 * locals.var_t2__blk901_dn2)) / (locals.var_t2__blk901 * locals.var_t2__blk901)), (((locals.var_t1__blk900_dn6 * locals.var_t2__blk901) - (locals.var_t1__blk900 * locals.var_t2__blk901_dn6)) / (locals.var_t2__blk901 * locals.var_t2__blk901)), (((locals.var_t1__blk900_dn7 * locals.var_t2__blk901) - (locals.var_t1__blk900 * locals.var_t2__blk901_dn7)) / (locals.var_t2__blk901 * locals.var_t2__blk901)), (((locals.var_t1__blk900_dn10 * locals.var_t2__blk901) - (locals.var_t1__blk900 * locals.var_t2__blk901_dn10)) / (locals.var_t2__blk901 * locals.var_t2__blk901)), (((locals.var_t1__blk900_dn11 * locals.var_t2__blk901) - (locals.var_t1__blk900 * locals.var_t2__blk901_dn11)) / (locals.var_t2__blk901 * locals.var_t2__blk901)), (((locals.var_t1__blk900_dn12 * locals.var_t2__blk901) - (locals.var_t1__blk900 * locals.var_t2__blk901_dn12)) / (locals.var_t2__blk901 * locals.var_t2__blk901)), (((locals.var_t1__blk900_dn17 * locals.var_t2__blk901) - (locals.var_t1__blk900 * locals.var_t2__blk901_dn17)) / (locals.var_t2__blk901 * locals.var_t2__blk901)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign30230_e43591;
        locals.var_tmf1_dn0 = assign30230_e43591_d_n0;
        locals.var_tmf1_dn2 = assign30230_e43591_d_n2;
        locals.var_tmf1_dn6 = assign30230_e43591_d_n6;
        locals.var_tmf1_dn7 = assign30230_e43591_d_n7;
        locals.var_tmf1_dn10 = assign30230_e43591_d_n10;
        locals.var_tmf1_dn11 = assign30230_e43591_d_n11;
        locals.var_tmf1_dn12 = assign30230_e43591_d_n12;
        locals.var_tmf1_dn17 = assign30230_e43591_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign30240_e43604, assign30240_e43604_d_n0, assign30240_e43604_d_n2, assign30240_e43604_d_n6, assign30240_e43604_d_n7, assign30240_e43604_d_n10, assign30240_e43604_d_n11, assign30240_e43604_d_n12, assign30240_e43604_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1005 != 0.0)) {
        let assign30240_e43602: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign30240_e43602, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign30240_e43604;
        locals.var_tmf2_dn0 = assign30240_e43604_d_n0;
        locals.var_tmf2_dn2 = assign30240_e43604_d_n2;
        locals.var_tmf2_dn6 = assign30240_e43604_d_n6;
        locals.var_tmf2_dn7 = assign30240_e43604_d_n7;
        locals.var_tmf2_dn10 = assign30240_e43604_d_n10;
        locals.var_tmf2_dn11 = assign30240_e43604_d_n11;
        locals.var_tmf2_dn12 = assign30240_e43604_d_n12;
        locals.var_tmf2_dn17 = assign30240_e43604_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign30250_e43617, assign30250_e43617_d_n0, assign30250_e43617_d_n2, assign30250_e43617_d_n6, assign30250_e43617_d_n7, assign30250_e43617_d_n10, assign30250_e43617_d_n11, assign30250_e43617_d_n12, assign30250_e43617_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1005 != 0.0)) {
        let assign30250_e43615: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign30250_e43615, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12, locals.var_tmf3_dn17,)
    }
};
        locals.var_tmf3 = assign30250_e43617;
        locals.var_tmf3_dn0 = assign30250_e43617_d_n0;
        locals.var_tmf3_dn2 = assign30250_e43617_d_n2;
        locals.var_tmf3_dn6 = assign30250_e43617_d_n6;
        locals.var_tmf3_dn7 = assign30250_e43617_d_n7;
        locals.var_tmf3_dn10 = assign30250_e43617_d_n10;
        locals.var_tmf3_dn11 = assign30250_e43617_d_n11;
        locals.var_tmf3_dn12 = assign30250_e43617_d_n12;
        locals.var_tmf3_dn17 = assign30250_e43617_d_n17;
        locals.var_tmf3_rv = 0.0;

        let (assign30260_e43630, assign30260_e43630_d_n0, assign30260_e43630_d_n2, assign30260_e43630_d_n6, assign30260_e43630_d_n7, assign30260_e43630_d_n10, assign30260_e43630_d_n11, assign30260_e43630_d_n12, assign30260_e43630_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1005 != 0.0)) {
        let assign30260_e43628: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign30260_e43628, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn17)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn12, locals.var_tmf4_dn17,)
    }
};
        locals.var_tmf4 = assign30260_e43630;
        locals.var_tmf4_dn0 = assign30260_e43630_d_n0;
        locals.var_tmf4_dn2 = assign30260_e43630_d_n2;
        locals.var_tmf4_dn6 = assign30260_e43630_d_n6;
        locals.var_tmf4_dn7 = assign30260_e43630_d_n7;
        locals.var_tmf4_dn10 = assign30260_e43630_d_n10;
        locals.var_tmf4_dn11 = assign30260_e43630_d_n11;
        locals.var_tmf4_dn12 = assign30260_e43630_d_n12;
        locals.var_tmf4_dn17 = assign30260_e43630_d_n17;
        locals.var_tmf4_rv = 0.0;

        let (assign30270_e43651, assign30270_e43651_d_n0, assign30270_e43651_d_n2, assign30270_e43651_d_n6, assign30270_e43651_d_n7, assign30270_e43651_d_n10, assign30270_e43651_d_n11, assign30270_e43651_d_n12, assign30270_e43651_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1005 != 0.0)) {
        let assign30270_e43642: f64 = (1.0 + locals.var_tmf1);
        let assign30270_e43644: f64 = (assign30270_e43642 + locals.var_tmf2);
        let assign30270_e43646: f64 = (assign30270_e43644 + locals.var_tmf3);
        let assign30270_e43648: f64 = (assign30270_e43646 + locals.var_tmf4);
        let assign30270_e43649: f64 = (1.0 / assign30270_e43648);
        (assign30270_e43649, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign30270_e43648 * assign30270_e43648))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign30270_e43648 * assign30270_e43648))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign30270_e43648 * assign30270_e43648))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign30270_e43648 * assign30270_e43648))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign30270_e43648 * assign30270_e43648))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign30270_e43648 * assign30270_e43648))), (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign30270_e43648 * assign30270_e43648))), (-((((locals.var_tmf1_dn17 + locals.var_tmf2_dn17) + locals.var_tmf3_dn17) + locals.var_tmf4_dn17) / (assign30270_e43648 * assign30270_e43648))),)
    } else {
        (locals.var_ty__blk909, locals.var_ty__blk909_dn0, locals.var_ty__blk909_dn2, locals.var_ty__blk909_dn6, locals.var_ty__blk909_dn7, locals.var_ty__blk909_dn10, locals.var_ty__blk909_dn11, locals.var_ty__blk909_dn12, locals.var_ty__blk909_dn17,)
    }
};
        locals.var_ty__blk909 = assign30270_e43651;
        locals.var_ty__blk909_dn0 = assign30270_e43651_d_n0;
        locals.var_ty__blk909_dn2 = assign30270_e43651_d_n2;
        locals.var_ty__blk909_dn6 = assign30270_e43651_d_n6;
        locals.var_ty__blk909_dn7 = assign30270_e43651_d_n7;
        locals.var_ty__blk909_dn10 = assign30270_e43651_d_n10;
        locals.var_ty__blk909_dn11 = assign30270_e43651_d_n11;
        locals.var_ty__blk909_dn12 = assign30270_e43651_d_n12;
        locals.var_ty__blk909_dn17 = assign30270_e43651_d_n17;
        locals.var_ty__blk909_rv = 0.0;

        let (assign30290_e43694, assign30290_e43694_d_n0, assign30290_e43694_d_n2, assign30290_e43694_d_n6, assign30290_e43694_d_n7, assign30290_e43694_d_n10, assign30290_e43694_d_n11, assign30290_e43694_d_n12, assign30290_e43694_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1005 != 0.0)) {
        let assign30290_e43691: f64 = (1.0 - locals.var_ty__blk909);
        let assign30290_e43692: f64 = (locals.var_t2__blk901 * assign30290_e43691);
        (assign30290_e43692, ((locals.var_t2__blk901_dn0 * assign30290_e43691) + (locals.var_t2__blk901 * (-locals.var_ty__blk909_dn0))), ((locals.var_t2__blk901_dn2 * assign30290_e43691) + (locals.var_t2__blk901 * (-locals.var_ty__blk909_dn2))), ((locals.var_t2__blk901_dn6 * assign30290_e43691) + (locals.var_t2__blk901 * (-locals.var_ty__blk909_dn6))), ((locals.var_t2__blk901_dn7 * assign30290_e43691) + (locals.var_t2__blk901 * (-locals.var_ty__blk909_dn7))), ((locals.var_t2__blk901_dn10 * assign30290_e43691) + (locals.var_t2__blk901 * (-locals.var_ty__blk909_dn10))), ((locals.var_t2__blk901_dn11 * assign30290_e43691) + (locals.var_t2__blk901 * (-locals.var_ty__blk909_dn11))), ((locals.var_t2__blk901_dn12 * assign30290_e43691) + (locals.var_t2__blk901 * (-locals.var_ty__blk909_dn12))), ((locals.var_t2__blk901_dn17 * assign30290_e43691) + (locals.var_t2__blk901 * (-locals.var_ty__blk909_dn17))),)
    } else {
        (locals.var_ty__blk909, locals.var_ty__blk909_dn0, locals.var_ty__blk909_dn2, locals.var_ty__blk909_dn6, locals.var_ty__blk909_dn7, locals.var_ty__blk909_dn10, locals.var_ty__blk909_dn11, locals.var_ty__blk909_dn12, locals.var_ty__blk909_dn17,)
    }
};
        locals.var_ty__blk909 = assign30290_e43694;
        locals.var_ty__blk909_dn0 = assign30290_e43694_d_n0;
        locals.var_ty__blk909_dn2 = assign30290_e43694_d_n2;
        locals.var_ty__blk909_dn6 = assign30290_e43694_d_n6;
        locals.var_ty__blk909_dn7 = assign30290_e43694_d_n7;
        locals.var_ty__blk909_dn10 = assign30290_e43694_d_n10;
        locals.var_ty__blk909_dn11 = assign30290_e43694_d_n11;
        locals.var_ty__blk909_dn12 = assign30290_e43694_d_n12;
        locals.var_ty__blk909_dn17 = assign30290_e43694_d_n17;
        locals.var_ty__blk909_rv = 0.0;

        let (assign30310_e43719, assign30310_e43719_d_n0, assign30310_e43719_d_n2, assign30310_e43719_d_n6, assign30310_e43719_d_n7, assign30310_e43719_d_n10, assign30310_e43719_d_n11, assign30310_e43719_d_n12, assign30310_e43719_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1005 != 0.0)) {
        let assign30310_e43717: f64 = (locals.var_vbs_bnd + locals.var_ty__blk909);
        (assign30310_e43717, locals.var_ty__blk909_dn0, locals.var_ty__blk909_dn2, locals.var_ty__blk909_dn6, locals.var_ty__blk909_dn7, locals.var_ty__blk909_dn10, locals.var_ty__blk909_dn11, locals.var_ty__blk909_dn12, locals.var_ty__blk909_dn17,)
    } else {
        (locals.var_t10__blk906, locals.var_t10__blk906_dn0, locals.var_t10__blk906_dn2, locals.var_t10__blk906_dn6, locals.var_t10__blk906_dn7, locals.var_t10__blk906_dn10, locals.var_t10__blk906_dn11, locals.var_t10__blk906_dn12, locals.var_t10__blk906_dn17,)
    }
};
        locals.var_t10__blk906 = assign30310_e43719;
        locals.var_t10__blk906_dn0 = assign30310_e43719_d_n0;
        locals.var_t10__blk906_dn2 = assign30310_e43719_d_n2;
        locals.var_t10__blk906_dn6 = assign30310_e43719_d_n6;
        locals.var_t10__blk906_dn7 = assign30310_e43719_d_n7;
        locals.var_t10__blk906_dn10 = assign30310_e43719_d_n10;
        locals.var_t10__blk906_dn11 = assign30310_e43719_d_n11;
        locals.var_t10__blk906_dn12 = assign30310_e43719_d_n12;
        locals.var_t10__blk906_dn17 = assign30310_e43719_d_n17;
        locals.var_t10__blk906_rv = 0.0;

        let (assign30320_e43731, assign30320_e43731_d_n0, assign30320_e43731_d_n2, assign30320_e43731_d_n6, assign30320_e43731_d_n7, assign30320_e43731_d_n10, assign30320_e43731_d_n11, assign30320_e43731_d_n12, assign30320_e43731_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1005 == 0.0)) {
        (locals.var_t0__blk899, locals.var_t0__blk899_dn0, locals.var_t0__blk899_dn2, locals.var_t0__blk899_dn6, locals.var_t0__blk899_dn7, locals.var_t0__blk899_dn10, locals.var_t0__blk899_dn11, locals.var_t0__blk899_dn12, locals.var_t0__blk899_dn17,)
    } else {
        (locals.var_t10__blk906, locals.var_t10__blk906_dn0, locals.var_t10__blk906_dn2, locals.var_t10__blk906_dn6, locals.var_t10__blk906_dn7, locals.var_t10__blk906_dn10, locals.var_t10__blk906_dn11, locals.var_t10__blk906_dn12, locals.var_t10__blk906_dn17,)
    }
};
        locals.var_t10__blk906 = assign30320_e43731;
        locals.var_t10__blk906_dn0 = assign30320_e43731_d_n0;
        locals.var_t10__blk906_dn2 = assign30320_e43731_d_n2;
        locals.var_t10__blk906_dn6 = assign30320_e43731_d_n6;
        locals.var_t10__blk906_dn7 = assign30320_e43731_d_n7;
        locals.var_t10__blk906_dn10 = assign30320_e43731_d_n10;
        locals.var_t10__blk906_dn11 = assign30320_e43731_d_n11;
        locals.var_t10__blk906_dn12 = assign30320_e43731_d_n12;
        locals.var_t10__blk906_dn17 = assign30320_e43731_d_n17;
        locals.var_t10__blk906_rv = 0.0;

        let (assign30340_e43755, assign30340_e43755_d_n0, assign30340_e43755_d_n2, assign30340_e43755_d_n6, assign30340_e43755_d_n7, assign30340_e43755_d_n10, assign30340_e43755_d_n11, assign30340_e43755_d_n12, assign30340_e43755_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign30340_e43751: f64 = (-locals.var_t10__blk906);
        let assign30340_e43753: f64 = (assign30340_e43751 - 1e-12);
        (assign30340_e43753, (-locals.var_t10__blk906_dn0), (-locals.var_t10__blk906_dn2), (-locals.var_t10__blk906_dn6), (-locals.var_t10__blk906_dn7), (-locals.var_t10__blk906_dn10), (-locals.var_t10__blk906_dn11), (-locals.var_t10__blk906_dn12), (-locals.var_t10__blk906_dn17),)
    } else {
        (locals.var_vxbgmtcl__blk925, locals.var_vxbgmtcl__blk925_dn0, locals.var_vxbgmtcl__blk925_dn2, locals.var_vxbgmtcl__blk925_dn6, locals.var_vxbgmtcl__blk925_dn7, locals.var_vxbgmtcl__blk925_dn10, locals.var_vxbgmtcl__blk925_dn11, locals.var_vxbgmtcl__blk925_dn12, locals.var_vxbgmtcl__blk925_dn17,)
    }
};
        locals.var_vxbgmtcl__blk925 = assign30340_e43755;
        locals.var_vxbgmtcl__blk925_dn0 = assign30340_e43755_d_n0;
        locals.var_vxbgmtcl__blk925_dn2 = assign30340_e43755_d_n2;
        locals.var_vxbgmtcl__blk925_dn6 = assign30340_e43755_d_n6;
        locals.var_vxbgmtcl__blk925_dn7 = assign30340_e43755_d_n7;
        locals.var_vxbgmtcl__blk925_dn10 = assign30340_e43755_d_n10;
        locals.var_vxbgmtcl__blk925_dn11 = assign30340_e43755_d_n11;
        locals.var_vxbgmtcl__blk925_dn12 = assign30340_e43755_d_n12;
        locals.var_vxbgmtcl__blk925_dn17 = assign30340_e43755_d_n17;
        locals.var_vxbgmtcl__blk925_rv = 0.0;

        let (assign30350_e43766, assign30350_e43766_d_n0, assign30350_e43766_d_n2, assign30350_e43766_d_n6, assign30350_e43766_d_n7, assign30350_e43766_d_n10, assign30350_e43766_d_n11, assign30350_e43766_d_n12, assign30350_e43766_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign30350_e43764: f64 = (locals.var_cnst0over__blk932 * locals.var_cox0_inv__blk911);
        (assign30350_e43764, (locals.var_cnst0over__blk932_dn0 * locals.var_cox0_inv__blk911), (locals.var_cnst0over__blk932_dn2 * locals.var_cox0_inv__blk911), (locals.var_cnst0over__blk932_dn6 * locals.var_cox0_inv__blk911), (locals.var_cnst0over__blk932_dn7 * locals.var_cox0_inv__blk911), (locals.var_cnst0over__blk932_dn10 * locals.var_cox0_inv__blk911), (locals.var_cnst0over__blk932_dn11 * locals.var_cox0_inv__blk911), (locals.var_cnst0over__blk932_dn12 * locals.var_cox0_inv__blk911), (locals.var_cnst0over__blk932_dn17 * locals.var_cox0_inv__blk911),)
    } else {
        (locals.var_fac1__blk933, locals.var_fac1__blk933_dn0, locals.var_fac1__blk933_dn2, locals.var_fac1__blk933_dn6, locals.var_fac1__blk933_dn7, locals.var_fac1__blk933_dn10, locals.var_fac1__blk933_dn11, locals.var_fac1__blk933_dn12, locals.var_fac1__blk933_dn17,)
    }
};
        locals.var_fac1__blk933 = assign30350_e43766;
        locals.var_fac1__blk933_dn0 = assign30350_e43766_d_n0;
        locals.var_fac1__blk933_dn2 = assign30350_e43766_d_n2;
        locals.var_fac1__blk933_dn6 = assign30350_e43766_d_n6;
        locals.var_fac1__blk933_dn7 = assign30350_e43766_d_n7;
        locals.var_fac1__blk933_dn10 = assign30350_e43766_d_n10;
        locals.var_fac1__blk933_dn11 = assign30350_e43766_d_n11;
        locals.var_fac1__blk933_dn12 = assign30350_e43766_d_n12;
        locals.var_fac1__blk933_dn17 = assign30350_e43766_d_n17;
        locals.var_fac1__blk933_rv = 0.0;

        let (assign30360_e43777, assign30360_e43777_d_n0, assign30360_e43777_d_n2, assign30360_e43777_d_n6, assign30360_e43777_d_n7, assign30360_e43777_d_n10, assign30360_e43777_d_n11, assign30360_e43777_d_n12, assign30360_e43777_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign30360_e43775: f64 = (locals.var_fac1__blk933 * locals.var_fac1__blk933);
        (assign30360_e43775, ((locals.var_fac1__blk933_dn0 * locals.var_fac1__blk933) + (locals.var_fac1__blk933 * locals.var_fac1__blk933_dn0)), ((locals.var_fac1__blk933_dn2 * locals.var_fac1__blk933) + (locals.var_fac1__blk933 * locals.var_fac1__blk933_dn2)), ((locals.var_fac1__blk933_dn6 * locals.var_fac1__blk933) + (locals.var_fac1__blk933 * locals.var_fac1__blk933_dn6)), ((locals.var_fac1__blk933_dn7 * locals.var_fac1__blk933) + (locals.var_fac1__blk933 * locals.var_fac1__blk933_dn7)), ((locals.var_fac1__blk933_dn10 * locals.var_fac1__blk933) + (locals.var_fac1__blk933 * locals.var_fac1__blk933_dn10)), ((locals.var_fac1__blk933_dn11 * locals.var_fac1__blk933) + (locals.var_fac1__blk933 * locals.var_fac1__blk933_dn11)), ((locals.var_fac1__blk933_dn12 * locals.var_fac1__blk933) + (locals.var_fac1__blk933 * locals.var_fac1__blk933_dn12)), ((locals.var_fac1__blk933_dn17 * locals.var_fac1__blk933) + (locals.var_fac1__blk933 * locals.var_fac1__blk933_dn17)),)
    } else {
        (locals.var_fac1p2__blk934, locals.var_fac1p2__blk934_dn0, locals.var_fac1p2__blk934_dn2, locals.var_fac1p2__blk934_dn6, locals.var_fac1p2__blk934_dn7, locals.var_fac1p2__blk934_dn10, locals.var_fac1p2__blk934_dn11, locals.var_fac1p2__blk934_dn12, locals.var_fac1p2__blk934_dn17,)
    }
};
        locals.var_fac1p2__blk934 = assign30360_e43777;
        locals.var_fac1p2__blk934_dn0 = assign30360_e43777_d_n0;
        locals.var_fac1p2__blk934_dn2 = assign30360_e43777_d_n2;
        locals.var_fac1p2__blk934_dn6 = assign30360_e43777_d_n6;
        locals.var_fac1p2__blk934_dn7 = assign30360_e43777_d_n7;
        locals.var_fac1p2__blk934_dn10 = assign30360_e43777_d_n10;
        locals.var_fac1p2__blk934_dn11 = assign30360_e43777_d_n11;
        locals.var_fac1p2__blk934_dn12 = assign30360_e43777_d_n12;
        locals.var_fac1p2__blk934_dn17 = assign30360_e43777_d_n17;
        locals.var_fac1p2__blk934_rv = 0.0;

        let (assign30370_e43789, assign30370_e43789_d_n0, assign30370_e43789_d_n2, assign30370_e43789_d_n6, assign30370_e43789_d_n7, assign30370_e43789_d_n10, assign30370_e43789_d_n11, assign30370_e43789_d_n12, assign30370_e43789_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign30370_e43785: f64 = (-locals.var_vgbgmt__blk931);
        let assign30370_e43787: f64 = (assign30370_e43785 + locals.var_uc_vfbover);
        (assign30370_e43787, (-locals.var_vgbgmt__blk931_dn0), (-locals.var_vgbgmt__blk931_dn2), (-locals.var_vgbgmt__blk931_dn6), (-locals.var_vgbgmt__blk931_dn7), (-locals.var_vgbgmt__blk931_dn10), (-locals.var_vgbgmt__blk931_dn11), (-locals.var_vgbgmt__blk931_dn12), (-locals.var_vgbgmt__blk931_dn17),)
    } else {
        (locals.var_vgpld__blk935, locals.var_vgpld__blk935_dn0, locals.var_vgpld__blk935_dn2, locals.var_vgpld__blk935_dn6, locals.var_vgpld__blk935_dn7, locals.var_vgpld__blk935_dn10, locals.var_vgpld__blk935_dn11, locals.var_vgpld__blk935_dn12, locals.var_vgpld__blk935_dn17,)
    }
};
        locals.var_vgpld__blk935 = assign30370_e43789;
        locals.var_vgpld__blk935_dn0 = assign30370_e43789_d_n0;
        locals.var_vgpld__blk935_dn2 = assign30370_e43789_d_n2;
        locals.var_vgpld__blk935_dn6 = assign30370_e43789_d_n6;
        locals.var_vgpld__blk935_dn7 = assign30370_e43789_d_n7;
        locals.var_vgpld__blk935_dn10 = assign30370_e43789_d_n10;
        locals.var_vgpld__blk935_dn11 = assign30370_e43789_d_n11;
        locals.var_vgpld__blk935_dn12 = assign30370_e43789_d_n12;
        locals.var_vgpld__blk935_dn17 = assign30370_e43789_d_n17;
        locals.var_vgpld__blk935_rv = 0.0;

        let (assign30380_e43800, assign30380_e43800_d_n0, assign30380_e43800_d_n2, assign30380_e43800_d_n6, assign30380_e43800_d_n7, assign30380_e43800_d_n10, assign30380_e43800_d_n11, assign30380_e43800_d_n12, assign30380_e43800_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign30380_e43798: f64 = (locals.var_mks_nover / locals.var_nin);
        (assign30380_e43798, (-((locals.var_mks_nover * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn12) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn17) / (locals.var_nin * locals.var_nin))),)
    } else {
        (locals.var_t0__blk899, locals.var_t0__blk899_dn0, locals.var_t0__blk899_dn2, locals.var_t0__blk899_dn6, locals.var_t0__blk899_dn7, locals.var_t0__blk899_dn10, locals.var_t0__blk899_dn11, locals.var_t0__blk899_dn12, locals.var_t0__blk899_dn17,)
    }
};
        locals.var_t0__blk899 = assign30380_e43800;
        locals.var_t0__blk899_dn0 = assign30380_e43800_d_n0;
        locals.var_t0__blk899_dn2 = assign30380_e43800_d_n2;
        locals.var_t0__blk899_dn6 = assign30380_e43800_d_n6;
        locals.var_t0__blk899_dn7 = assign30380_e43800_d_n7;
        locals.var_t0__blk899_dn10 = assign30380_e43800_d_n10;
        locals.var_t0__blk899_dn11 = assign30380_e43800_d_n11;
        locals.var_t0__blk899_dn12 = assign30380_e43800_d_n12;
        locals.var_t0__blk899_dn17 = assign30380_e43800_d_n17;
        locals.var_t0__blk899_rv = 0.0;

        let (assign30390_e43814, assign30390_e43814_d_n0, assign30390_e43814_d_n2, assign30390_e43814_d_n6, assign30390_e43814_d_n7, assign30390_e43814_d_n10, assign30390_e43814_d_n11, assign30390_e43814_d_n12, assign30390_e43814_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign30390_e43809: f64 = (2.0 / locals.var_beta);
        let assign30390_e43811: f64 = (locals.var_t0__blk899).ln();
        let assign30390_e43812: f64 = (assign30390_e43809 * assign30390_e43811);
        (assign30390_e43812, (assign30390_e43809 * (locals.var_t0__blk899_dn0 / locals.var_t0__blk899)), (assign30390_e43809 * (locals.var_t0__blk899_dn2 / locals.var_t0__blk899)), (assign30390_e43809 * (locals.var_t0__blk899_dn6 / locals.var_t0__blk899)), (assign30390_e43809 * (locals.var_t0__blk899_dn7 / locals.var_t0__blk899)), (((-((2.0 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign30390_e43811) + (assign30390_e43809 * (locals.var_t0__blk899_dn10 / locals.var_t0__blk899))), (assign30390_e43809 * (locals.var_t0__blk899_dn11 / locals.var_t0__blk899)), (assign30390_e43809 * (locals.var_t0__blk899_dn12 / locals.var_t0__blk899)), (assign30390_e43809 * (locals.var_t0__blk899_dn17 / locals.var_t0__blk899)),)
    } else {
        (locals.var_pb2over__blk936, locals.var_pb2over__blk936_dn0, locals.var_pb2over__blk936_dn2, locals.var_pb2over__blk936_dn6, locals.var_pb2over__blk936_dn7, locals.var_pb2over__blk936_dn10, locals.var_pb2over__blk936_dn11, locals.var_pb2over__blk936_dn12, locals.var_pb2over__blk936_dn17,)
    }
};
        locals.var_pb2over__blk936 = assign30390_e43814;
        locals.var_pb2over__blk936_dn0 = assign30390_e43814_d_n0;
        locals.var_pb2over__blk936_dn2 = assign30390_e43814_d_n2;
        locals.var_pb2over__blk936_dn6 = assign30390_e43814_d_n6;
        locals.var_pb2over__blk936_dn7 = assign30390_e43814_d_n7;
        locals.var_pb2over__blk936_dn10 = assign30390_e43814_d_n10;
        locals.var_pb2over__blk936_dn11 = assign30390_e43814_d_n11;
        locals.var_pb2over__blk936_dn12 = assign30390_e43814_d_n12;
        locals.var_pb2over__blk936_dn17 = assign30390_e43814_d_n17;
        locals.var_pb2over__blk936_rv = 0.0;

        let (assign30400_e43824, assign30400_e43824_d_n0, assign30400_e43824_d_n2, assign30400_e43824_d_n6, assign30400_e43824_d_n7, assign30400_e43824_d_n10, assign30400_e43824_d_n11, assign30400_e43824_d_n12, assign30400_e43824_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign30400_e43822: f64 = (-locals.var_vxbgmtcl__blk925);
        (assign30400_e43822, (-locals.var_vxbgmtcl__blk925_dn0), (-locals.var_vxbgmtcl__blk925_dn2), (-locals.var_vxbgmtcl__blk925_dn6), (-locals.var_vxbgmtcl__blk925_dn7), (-locals.var_vxbgmtcl__blk925_dn10), (-locals.var_vxbgmtcl__blk925_dn11), (-locals.var_vxbgmtcl__blk925_dn12), (-locals.var_vxbgmtcl__blk925_dn17),)
    } else {
        (locals.var_vgb_fb_ld__blk937, locals.var_vgb_fb_ld__blk937_dn0, locals.var_vgb_fb_ld__blk937_dn2, locals.var_vgb_fb_ld__blk937_dn6, locals.var_vgb_fb_ld__blk937_dn7, locals.var_vgb_fb_ld__blk937_dn10, locals.var_vgb_fb_ld__blk937_dn11, locals.var_vgb_fb_ld__blk937_dn12, locals.var_vgb_fb_ld__blk937_dn17,)
    }
};
        locals.var_vgb_fb_ld__blk937 = assign30400_e43824;
        locals.var_vgb_fb_ld__blk937_dn0 = assign30400_e43824_d_n0;
        locals.var_vgb_fb_ld__blk937_dn2 = assign30400_e43824_d_n2;
        locals.var_vgb_fb_ld__blk937_dn6 = assign30400_e43824_d_n6;
        locals.var_vgb_fb_ld__blk937_dn7 = assign30400_e43824_d_n7;
        locals.var_vgb_fb_ld__blk937_dn10 = assign30400_e43824_d_n10;
        locals.var_vgb_fb_ld__blk937_dn11 = assign30400_e43824_d_n11;
        locals.var_vgb_fb_ld__blk937_dn12 = assign30400_e43824_d_n12;
        locals.var_vgb_fb_ld__blk937_dn17 = assign30400_e43824_d_n17;
        locals.var_vgb_fb_ld__blk937_rv = 0.0;

        let assign30410_e43827: f64 = if locals.var_vgpld__blk935 < locals.var_vgb_fb_ld__blk937 { 1.0 } else { 0.0 };
        locals.var_guard1006 = assign30410_e43827;
        locals.var_guard1006_rv = 0.0;

        let (assign30430_e43854, assign30430_e43854_d_n0, assign30430_e43854_d_n2, assign30430_e43854_d_n6, assign30430_e43854_d_n7, assign30430_e43854_d_n10, assign30430_e43854_d_n11, assign30430_e43854_d_n12, assign30430_e43854_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 != 0.0)) {
        let assign30430_e43851: f64 = (locals.var_beta * locals.var_cnst0over__blk932);
        let assign30430_e43852: f64 = (1.0 / assign30430_e43851);
        (assign30430_e43852, (-((locals.var_beta * locals.var_cnst0over__blk932_dn0) / (assign30430_e43851 * assign30430_e43851))), (-((locals.var_beta * locals.var_cnst0over__blk932_dn2) / (assign30430_e43851 * assign30430_e43851))), (-((locals.var_beta * locals.var_cnst0over__blk932_dn6) / (assign30430_e43851 * assign30430_e43851))), (-((locals.var_beta * locals.var_cnst0over__blk932_dn7) / (assign30430_e43851 * assign30430_e43851))), (-(((locals.var_beta_dn10 * locals.var_cnst0over__blk932) + (locals.var_beta * locals.var_cnst0over__blk932_dn10)) / (assign30430_e43851 * assign30430_e43851))), (-((locals.var_beta * locals.var_cnst0over__blk932_dn11) / (assign30430_e43851 * assign30430_e43851))), (-((locals.var_beta * locals.var_cnst0over__blk932_dn12) / (assign30430_e43851 * assign30430_e43851))), (-((locals.var_beta * locals.var_cnst0over__blk932_dn17) / (assign30430_e43851 * assign30430_e43851))),)
    } else {
        (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17,)
    }
};
        locals.var_t1__blk900 = assign30430_e43854;
        locals.var_t1__blk900_dn0 = assign30430_e43854_d_n0;
        locals.var_t1__blk900_dn2 = assign30430_e43854_d_n2;
        locals.var_t1__blk900_dn6 = assign30430_e43854_d_n6;
        locals.var_t1__blk900_dn7 = assign30430_e43854_d_n7;
        locals.var_t1__blk900_dn10 = assign30430_e43854_d_n10;
        locals.var_t1__blk900_dn11 = assign30430_e43854_d_n11;
        locals.var_t1__blk900_dn12 = assign30430_e43854_d_n12;
        locals.var_t1__blk900_dn17 = assign30430_e43854_d_n17;
        locals.var_t1__blk900_rv = 0.0;

        let (assign30440_e43867, assign30440_e43867_d_n0, assign30440_e43867_d_n2, assign30440_e43867_d_n6, assign30440_e43867_d_n7, assign30440_e43867_d_n10, assign30440_e43867_d_n11, assign30440_e43867_d_n12, assign30440_e43867_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 != 0.0)) {
        let assign30440_e43865: f64 = (locals.var_t1__blk900 * locals.var_cox0__blk910);
        (assign30440_e43865, (locals.var_t1__blk900_dn0 * locals.var_cox0__blk910), (locals.var_t1__blk900_dn2 * locals.var_cox0__blk910), (locals.var_t1__blk900_dn6 * locals.var_cox0__blk910), (locals.var_t1__blk900_dn7 * locals.var_cox0__blk910), (locals.var_t1__blk900_dn10 * locals.var_cox0__blk910), (locals.var_t1__blk900_dn11 * locals.var_cox0__blk910), (locals.var_t1__blk900_dn12 * locals.var_cox0__blk910), (locals.var_t1__blk900_dn17 * locals.var_cox0__blk910),)
    } else {
        (locals.var_ty__blk909, locals.var_ty__blk909_dn0, locals.var_ty__blk909_dn2, locals.var_ty__blk909_dn6, locals.var_ty__blk909_dn7, locals.var_ty__blk909_dn10, locals.var_ty__blk909_dn11, locals.var_ty__blk909_dn12, locals.var_ty__blk909_dn17,)
    }
};
        locals.var_ty__blk909 = assign30440_e43867;
        locals.var_ty__blk909_dn0 = assign30440_e43867_d_n0;
        locals.var_ty__blk909_dn2 = assign30440_e43867_d_n2;
        locals.var_ty__blk909_dn6 = assign30440_e43867_d_n6;
        locals.var_ty__blk909_dn7 = assign30440_e43867_d_n7;
        locals.var_ty__blk909_dn10 = assign30440_e43867_d_n10;
        locals.var_ty__blk909_dn11 = assign30440_e43867_d_n11;
        locals.var_ty__blk909_dn12 = assign30440_e43867_d_n12;
        locals.var_ty__blk909_dn17 = assign30440_e43867_d_n17;
        locals.var_ty__blk909_rv = 0.0;

        let (assign30450_e43884, assign30450_e43884_d_n0, assign30450_e43884_d_n2, assign30450_e43884_d_n6, assign30450_e43884_d_n7, assign30450_e43884_d_n10, assign30450_e43884_d_n11, assign30450_e43884_d_n12, assign30450_e43884_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) && (locals.var_guard1006 != 0.0)) {
        let assign30450_e43879: f64 = (3.0 * 1.414213562373095);
        let assign30450_e43881: f64 = (assign30450_e43879 * locals.var_ty__blk909);
        let assign30450_e43882: f64 = (2.0 + assign30450_e43881);
        (assign30450_e43882, (assign30450_e43879 * locals.var_ty__blk909_dn0), (assign30450_e43879 * locals.var_ty__blk909_dn2), (assign30450_e43879 * locals.var_ty__blk909_dn6), (assign30450_e43879 * locals.var_ty__blk909_dn7), (assign30450_e43879 * locals.var_ty__blk909_dn10), (assign30450_e43879 * locals.var_ty__blk909_dn11), (assign30450_e43879 * locals.var_ty__blk909_dn12), (assign30450_e43879 * locals.var_ty__blk909_dn17),)
    } else {
        (locals.var_ac41__blk938, locals.var_ac41__blk938_dn0, locals.var_ac41__blk938_dn2, locals.var_ac41__blk938_dn6, locals.var_ac41__blk938_dn7, locals.var_ac41__blk938_dn10, locals.var_ac41__blk938_dn11, locals.var_ac41__blk938_dn12, locals.var_ac41__blk938_dn17,)
    }
};
        locals.var_ac41__blk938 = assign30450_e43884;
        locals.var_ac41__blk938_dn0 = assign30450_e43884_d_n0;
        locals.var_ac41__blk938_dn2 = assign30450_e43884_d_n2;
        locals.var_ac41__blk938_dn6 = assign30450_e43884_d_n6;
        locals.var_ac41__blk938_dn7 = assign30450_e43884_d_n7;
        locals.var_ac41__blk938_dn10 = assign30450_e43884_d_n10;
        locals.var_ac41__blk938_dn11 = assign30450_e43884_d_n11;
        locals.var_ac41__blk938_dn12 = assign30450_e43884_d_n12;
        locals.var_ac41__blk938_dn17 = assign30450_e43884_d_n17;
        locals.var_ac41__blk938_rv = 0.0;

    }
}
