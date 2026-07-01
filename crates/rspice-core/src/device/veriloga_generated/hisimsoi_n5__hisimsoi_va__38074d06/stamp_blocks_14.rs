#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_95(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26570_e36919,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26570_e36913: f64 = (locals.var_flg_ovloops * locals.var_modervs);
        let assign26570_e36916: f64 = (locals.var_flg_ovloopd * locals.var_modenml);
        let assign26570_e36917: f64 = (assign26570_e36913 + assign26570_e36916);
        (assign26570_e36917,)
    } else {
        (locals.var_flg_overd,)
    }
};
        locals.var_flg_overd = assign26570_e36919;
        locals.var_flg_overd_rv = 0.0;

        let (assign26580_e36933, assign26580_e36933_d_n0, assign26580_e36933_d_n2, assign26580_e36933_d_n6, assign26580_e36933_d_n7, assign26580_e36933_d_n10, assign26580_e36933_d_n11, assign26580_e36933_d_n12, assign26580_e36933_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26580_e36927: f64 = (locals.var_flg_overs * locals.var_vgsgmt);
        let assign26580_e36930: f64 = (locals.var_flg_overd * locals.var_vgdgmt);
        let assign26580_e36931: f64 = (assign26580_e36927 + assign26580_e36930);
        (assign26580_e36931, ((locals.var_flg_overs * locals.var_vgsgmt_dn0) + (locals.var_flg_overd * locals.var_vgdgmt_dn0)), ((locals.var_flg_overs * locals.var_vgsgmt_dn2) + (locals.var_flg_overd * locals.var_vgdgmt_dn2)), ((locals.var_flg_overs * locals.var_vgsgmt_dn6) + (locals.var_flg_overd * locals.var_vgdgmt_dn6)), ((locals.var_flg_overs * locals.var_vgsgmt_dn7) + (locals.var_flg_overd * locals.var_vgdgmt_dn7)), ((locals.var_flg_overs * locals.var_vgsgmt_dn10) + (locals.var_flg_overd * locals.var_vgdgmt_dn10)), ((locals.var_flg_overs * locals.var_vgsgmt_dn11) + (locals.var_flg_overd * locals.var_vgdgmt_dn11)), ((locals.var_flg_overs * locals.var_vgsgmt_dn12) + (locals.var_flg_overd * locals.var_vgdgmt_dn12)), ((locals.var_flg_overs * locals.var_vgsgmt_dn17) + (locals.var_flg_overd * locals.var_vgdgmt_dn17)),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn10, locals.var_vgbgmt_dn11, locals.var_vgbgmt_dn12, locals.var_vgbgmt_dn17,)
    }
};
        locals.var_vgbgmt = assign26580_e36933;
        locals.var_vgbgmt_dn0 = assign26580_e36933_d_n0;
        locals.var_vgbgmt_dn2 = assign26580_e36933_d_n2;
        locals.var_vgbgmt_dn6 = assign26580_e36933_d_n6;
        locals.var_vgbgmt_dn7 = assign26580_e36933_d_n7;
        locals.var_vgbgmt_dn10 = assign26580_e36933_d_n10;
        locals.var_vgbgmt_dn11 = assign26580_e36933_d_n11;
        locals.var_vgbgmt_dn12 = assign26580_e36933_d_n12;
        locals.var_vgbgmt_dn17 = assign26580_e36933_d_n17;
        locals.var_vgbgmt_rv = 0.0;

        let (assign26590_e36951, assign26590_e36951_d_n0, assign26590_e36951_d_n2, assign26590_e36951_d_n6, assign26590_e36951_d_n7, assign26590_e36951_d_n10, assign26590_e36951_d_n11, assign26590_e36951_d_n12, assign26590_e36951_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26590_e36941: f64 = (locals.var_flg_overs * locals.var_vsbgmt);
        let assign26590_e36944: f64 = (locals.var_flg_overd * locals.var_vdbgmt);
        let assign26590_e36945: f64 = (assign26590_e36941 + assign26590_e36944);
        let assign26590_e36948: f64 = (10.0 * 2.220446049250313e-16);
        let assign26590_e36949: f64 = (assign26590_e36945 + assign26590_e36948);
        (assign26590_e36949, ((locals.var_flg_overs * locals.var_vsbgmt_dn0) + (locals.var_flg_overd * locals.var_vdbgmt_dn0)), ((locals.var_flg_overs * locals.var_vsbgmt_dn2) + (locals.var_flg_overd * locals.var_vdbgmt_dn2)), ((locals.var_flg_overs * locals.var_vsbgmt_dn6) + (locals.var_flg_overd * locals.var_vdbgmt_dn6)), ((locals.var_flg_overs * locals.var_vsbgmt_dn7) + (locals.var_flg_overd * locals.var_vdbgmt_dn7)), ((locals.var_flg_overs * locals.var_vsbgmt_dn10) + (locals.var_flg_overd * locals.var_vdbgmt_dn10)), ((locals.var_flg_overs * locals.var_vsbgmt_dn11) + (locals.var_flg_overd * locals.var_vdbgmt_dn11)), ((locals.var_flg_overs * locals.var_vsbgmt_dn12) + (locals.var_flg_overd * locals.var_vdbgmt_dn12)), ((locals.var_flg_overs * locals.var_vsbgmt_dn17) + (locals.var_flg_overd * locals.var_vdbgmt_dn17)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn12, locals.var_vxbgmt_dn17,)
    }
};
        locals.var_vxbgmt = assign26590_e36951;
        locals.var_vxbgmt_dn0 = assign26590_e36951_d_n0;
        locals.var_vxbgmt_dn2 = assign26590_e36951_d_n2;
        locals.var_vxbgmt_dn6 = assign26590_e36951_d_n6;
        locals.var_vxbgmt_dn7 = assign26590_e36951_d_n7;
        locals.var_vxbgmt_dn10 = assign26590_e36951_d_n10;
        locals.var_vxbgmt_dn11 = assign26590_e36951_d_n11;
        locals.var_vxbgmt_dn12 = assign26590_e36951_d_n12;
        locals.var_vxbgmt_dn17 = assign26590_e36951_d_n17;
        locals.var_vxbgmt_rv = 0.0;

        let (assign26600_e36960, assign26600_e36960_d_n0, assign26600_e36960_d_n2, assign26600_e36960_d_n6, assign26600_e36960_d_n7, assign26600_e36960_d_n10, assign26600_e36960_d_n11, assign26600_e36960_d_n12, assign26600_e36960_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26600_e36958: f64 = (-locals.var_vxbgmt);
        (assign26600_e36958, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn11), (-locals.var_vxbgmt_dn12), (-locals.var_vxbgmt_dn17),)
    } else {
        (locals.var_t0__blk772, locals.var_t0__blk772_dn0, locals.var_t0__blk772_dn2, locals.var_t0__blk772_dn6, locals.var_t0__blk772_dn7, locals.var_t0__blk772_dn10, locals.var_t0__blk772_dn11, locals.var_t0__blk772_dn12, locals.var_t0__blk772_dn17,)
    }
};
        locals.var_t0__blk772 = assign26600_e36960;
        locals.var_t0__blk772_dn0 = assign26600_e36960_d_n0;
        locals.var_t0__blk772_dn2 = assign26600_e36960_d_n2;
        locals.var_t0__blk772_dn6 = assign26600_e36960_d_n6;
        locals.var_t0__blk772_dn7 = assign26600_e36960_d_n7;
        locals.var_t0__blk772_dn10 = assign26600_e36960_d_n10;
        locals.var_t0__blk772_dn11 = assign26600_e36960_d_n11;
        locals.var_t0__blk772_dn12 = assign26600_e36960_d_n12;
        locals.var_t0__blk772_dn17 = assign26600_e36960_d_n17;
        locals.var_t0__blk772_rv = 0.0;

        let assign26610_e36963: f64 = if locals.var_t0__blk772 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard872 = assign26610_e36963;
        locals.var_guard872_rv = 0.0;

        let (assign26620_e36975, assign26620_e36975_d_n0, assign26620_e36975_d_n2, assign26620_e36975_d_n6, assign26620_e36975_d_n7, assign26620_e36975_d_n10, assign26620_e36975_d_n11, assign26620_e36975_d_n12, assign26620_e36975_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard872 != 0.0)) {
        let assign26620_e36973: f64 = (locals.var_t0__blk772 - locals.var_vbs_bnd);
        (assign26620_e36973, locals.var_t0__blk772_dn0, locals.var_t0__blk772_dn2, locals.var_t0__blk772_dn6, locals.var_t0__blk772_dn7, locals.var_t0__blk772_dn10, locals.var_t0__blk772_dn11, locals.var_t0__blk772_dn12, locals.var_t0__blk772_dn17,)
    } else {
        (locals.var_t1__blk773, locals.var_t1__blk773_dn0, locals.var_t1__blk773_dn2, locals.var_t1__blk773_dn6, locals.var_t1__blk773_dn7, locals.var_t1__blk773_dn10, locals.var_t1__blk773_dn11, locals.var_t1__blk773_dn12, locals.var_t1__blk773_dn17,)
    }
};
        locals.var_t1__blk773 = assign26620_e36975;
        locals.var_t1__blk773_dn0 = assign26620_e36975_d_n0;
        locals.var_t1__blk773_dn2 = assign26620_e36975_d_n2;
        locals.var_t1__blk773_dn6 = assign26620_e36975_d_n6;
        locals.var_t1__blk773_dn7 = assign26620_e36975_d_n7;
        locals.var_t1__blk773_dn10 = assign26620_e36975_d_n10;
        locals.var_t1__blk773_dn11 = assign26620_e36975_d_n11;
        locals.var_t1__blk773_dn12 = assign26620_e36975_d_n12;
        locals.var_t1__blk773_dn17 = assign26620_e36975_d_n17;
        locals.var_t1__blk773_rv = 0.0;

        let (assign26630_e36987, assign26630_e36987_d_n0, assign26630_e36987_d_n2, assign26630_e36987_d_n6, assign26630_e36987_d_n7, assign26630_e36987_d_n10, assign26630_e36987_d_n11, assign26630_e36987_d_n12, assign26630_e36987_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard872 != 0.0)) {
        let assign26630_e36985: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign26630_e36985, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk774, locals.var_t2__blk774_dn0, locals.var_t2__blk774_dn2, locals.var_t2__blk774_dn6, locals.var_t2__blk774_dn7, locals.var_t2__blk774_dn10, locals.var_t2__blk774_dn11, locals.var_t2__blk774_dn12, locals.var_t2__blk774_dn17,)
    }
};
        locals.var_t2__blk774 = assign26630_e36987;
        locals.var_t2__blk774_dn0 = assign26630_e36987_d_n0;
        locals.var_t2__blk774_dn2 = assign26630_e36987_d_n2;
        locals.var_t2__blk774_dn6 = assign26630_e36987_d_n6;
        locals.var_t2__blk774_dn7 = assign26630_e36987_d_n7;
        locals.var_t2__blk774_dn10 = assign26630_e36987_d_n10;
        locals.var_t2__blk774_dn11 = assign26630_e36987_d_n11;
        locals.var_t2__blk774_dn12 = assign26630_e36987_d_n12;
        locals.var_t2__blk774_dn17 = assign26630_e36987_d_n17;
        locals.var_t2__blk774_rv = 0.0;

        let (assign26640_e36999, assign26640_e36999_d_n0, assign26640_e36999_d_n2, assign26640_e36999_d_n6, assign26640_e36999_d_n7, assign26640_e36999_d_n10, assign26640_e36999_d_n11, assign26640_e36999_d_n12, assign26640_e36999_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard872 != 0.0)) {
        let assign26640_e36997: f64 = (locals.var_t1__blk773 / locals.var_t2__blk774);
        (assign26640_e36997, (((locals.var_t1__blk773_dn0 * locals.var_t2__blk774) - (locals.var_t1__blk773 * locals.var_t2__blk774_dn0)) / (locals.var_t2__blk774 * locals.var_t2__blk774)), (((locals.var_t1__blk773_dn2 * locals.var_t2__blk774) - (locals.var_t1__blk773 * locals.var_t2__blk774_dn2)) / (locals.var_t2__blk774 * locals.var_t2__blk774)), (((locals.var_t1__blk773_dn6 * locals.var_t2__blk774) - (locals.var_t1__blk773 * locals.var_t2__blk774_dn6)) / (locals.var_t2__blk774 * locals.var_t2__blk774)), (((locals.var_t1__blk773_dn7 * locals.var_t2__blk774) - (locals.var_t1__blk773 * locals.var_t2__blk774_dn7)) / (locals.var_t2__blk774 * locals.var_t2__blk774)), (((locals.var_t1__blk773_dn10 * locals.var_t2__blk774) - (locals.var_t1__blk773 * locals.var_t2__blk774_dn10)) / (locals.var_t2__blk774 * locals.var_t2__blk774)), (((locals.var_t1__blk773_dn11 * locals.var_t2__blk774) - (locals.var_t1__blk773 * locals.var_t2__blk774_dn11)) / (locals.var_t2__blk774 * locals.var_t2__blk774)), (((locals.var_t1__blk773_dn12 * locals.var_t2__blk774) - (locals.var_t1__blk773 * locals.var_t2__blk774_dn12)) / (locals.var_t2__blk774 * locals.var_t2__blk774)), (((locals.var_t1__blk773_dn17 * locals.var_t2__blk774) - (locals.var_t1__blk773 * locals.var_t2__blk774_dn17)) / (locals.var_t2__blk774 * locals.var_t2__blk774)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign26640_e36999;
        locals.var_tmf1_dn0 = assign26640_e36999_d_n0;
        locals.var_tmf1_dn2 = assign26640_e36999_d_n2;
        locals.var_tmf1_dn6 = assign26640_e36999_d_n6;
        locals.var_tmf1_dn7 = assign26640_e36999_d_n7;
        locals.var_tmf1_dn10 = assign26640_e36999_d_n10;
        locals.var_tmf1_dn11 = assign26640_e36999_d_n11;
        locals.var_tmf1_dn12 = assign26640_e36999_d_n12;
        locals.var_tmf1_dn17 = assign26640_e36999_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign26650_e37011, assign26650_e37011_d_n0, assign26650_e37011_d_n2, assign26650_e37011_d_n6, assign26650_e37011_d_n7, assign26650_e37011_d_n10, assign26650_e37011_d_n11, assign26650_e37011_d_n12, assign26650_e37011_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard872 != 0.0)) {
        let assign26650_e37009: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign26650_e37009, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign26650_e37011;
        locals.var_tmf2_dn0 = assign26650_e37011_d_n0;
        locals.var_tmf2_dn2 = assign26650_e37011_d_n2;
        locals.var_tmf2_dn6 = assign26650_e37011_d_n6;
        locals.var_tmf2_dn7 = assign26650_e37011_d_n7;
        locals.var_tmf2_dn10 = assign26650_e37011_d_n10;
        locals.var_tmf2_dn11 = assign26650_e37011_d_n11;
        locals.var_tmf2_dn12 = assign26650_e37011_d_n12;
        locals.var_tmf2_dn17 = assign26650_e37011_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign26660_e37023, assign26660_e37023_d_n0, assign26660_e37023_d_n2, assign26660_e37023_d_n6, assign26660_e37023_d_n7, assign26660_e37023_d_n10, assign26660_e37023_d_n11, assign26660_e37023_d_n12, assign26660_e37023_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard872 != 0.0)) {
        let assign26660_e37021: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign26660_e37021, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12, locals.var_tmf3_dn17,)
    }
};
        locals.var_tmf3 = assign26660_e37023;
        locals.var_tmf3_dn0 = assign26660_e37023_d_n0;
        locals.var_tmf3_dn2 = assign26660_e37023_d_n2;
        locals.var_tmf3_dn6 = assign26660_e37023_d_n6;
        locals.var_tmf3_dn7 = assign26660_e37023_d_n7;
        locals.var_tmf3_dn10 = assign26660_e37023_d_n10;
        locals.var_tmf3_dn11 = assign26660_e37023_d_n11;
        locals.var_tmf3_dn12 = assign26660_e37023_d_n12;
        locals.var_tmf3_dn17 = assign26660_e37023_d_n17;
        locals.var_tmf3_rv = 0.0;

        let (assign26670_e37035, assign26670_e37035_d_n0, assign26670_e37035_d_n2, assign26670_e37035_d_n6, assign26670_e37035_d_n7, assign26670_e37035_d_n10, assign26670_e37035_d_n11, assign26670_e37035_d_n12, assign26670_e37035_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard872 != 0.0)) {
        let assign26670_e37033: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign26670_e37033, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn17)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn12, locals.var_tmf4_dn17,)
    }
};
        locals.var_tmf4 = assign26670_e37035;
        locals.var_tmf4_dn0 = assign26670_e37035_d_n0;
        locals.var_tmf4_dn2 = assign26670_e37035_d_n2;
        locals.var_tmf4_dn6 = assign26670_e37035_d_n6;
        locals.var_tmf4_dn7 = assign26670_e37035_d_n7;
        locals.var_tmf4_dn10 = assign26670_e37035_d_n10;
        locals.var_tmf4_dn11 = assign26670_e37035_d_n11;
        locals.var_tmf4_dn12 = assign26670_e37035_d_n12;
        locals.var_tmf4_dn17 = assign26670_e37035_d_n17;
        locals.var_tmf4_rv = 0.0;

        let (assign26680_e37055, assign26680_e37055_d_n0, assign26680_e37055_d_n2, assign26680_e37055_d_n6, assign26680_e37055_d_n7, assign26680_e37055_d_n10, assign26680_e37055_d_n11, assign26680_e37055_d_n12, assign26680_e37055_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard872 != 0.0)) {
        let assign26680_e37046: f64 = (1.0 + locals.var_tmf1);
        let assign26680_e37048: f64 = (assign26680_e37046 + locals.var_tmf2);
        let assign26680_e37050: f64 = (assign26680_e37048 + locals.var_tmf3);
        let assign26680_e37052: f64 = (assign26680_e37050 + locals.var_tmf4);
        let assign26680_e37053: f64 = (1.0 / assign26680_e37052);
        (assign26680_e37053, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign26680_e37052 * assign26680_e37052))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign26680_e37052 * assign26680_e37052))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign26680_e37052 * assign26680_e37052))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign26680_e37052 * assign26680_e37052))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign26680_e37052 * assign26680_e37052))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign26680_e37052 * assign26680_e37052))), (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign26680_e37052 * assign26680_e37052))), (-((((locals.var_tmf1_dn17 + locals.var_tmf2_dn17) + locals.var_tmf3_dn17) + locals.var_tmf4_dn17) / (assign26680_e37052 * assign26680_e37052))),)
    } else {
        (locals.var_ty__blk780, locals.var_ty__blk780_dn0, locals.var_ty__blk780_dn2, locals.var_ty__blk780_dn6, locals.var_ty__blk780_dn7, locals.var_ty__blk780_dn10, locals.var_ty__blk780_dn11, locals.var_ty__blk780_dn12, locals.var_ty__blk780_dn17,)
    }
};
        locals.var_ty__blk780 = assign26680_e37055;
        locals.var_ty__blk780_dn0 = assign26680_e37055_d_n0;
        locals.var_ty__blk780_dn2 = assign26680_e37055_d_n2;
        locals.var_ty__blk780_dn6 = assign26680_e37055_d_n6;
        locals.var_ty__blk780_dn7 = assign26680_e37055_d_n7;
        locals.var_ty__blk780_dn10 = assign26680_e37055_d_n10;
        locals.var_ty__blk780_dn11 = assign26680_e37055_d_n11;
        locals.var_ty__blk780_dn12 = assign26680_e37055_d_n12;
        locals.var_ty__blk780_dn17 = assign26680_e37055_d_n17;
        locals.var_ty__blk780_rv = 0.0;

        let (assign26700_e37096, assign26700_e37096_d_n0, assign26700_e37096_d_n2, assign26700_e37096_d_n6, assign26700_e37096_d_n7, assign26700_e37096_d_n10, assign26700_e37096_d_n11, assign26700_e37096_d_n12, assign26700_e37096_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard872 != 0.0)) {
        let assign26700_e37093: f64 = (1.0 - locals.var_ty__blk780);
        let assign26700_e37094: f64 = (locals.var_t2__blk774 * assign26700_e37093);
        (assign26700_e37094, ((locals.var_t2__blk774_dn0 * assign26700_e37093) + (locals.var_t2__blk774 * (-locals.var_ty__blk780_dn0))), ((locals.var_t2__blk774_dn2 * assign26700_e37093) + (locals.var_t2__blk774 * (-locals.var_ty__blk780_dn2))), ((locals.var_t2__blk774_dn6 * assign26700_e37093) + (locals.var_t2__blk774 * (-locals.var_ty__blk780_dn6))), ((locals.var_t2__blk774_dn7 * assign26700_e37093) + (locals.var_t2__blk774 * (-locals.var_ty__blk780_dn7))), ((locals.var_t2__blk774_dn10 * assign26700_e37093) + (locals.var_t2__blk774 * (-locals.var_ty__blk780_dn10))), ((locals.var_t2__blk774_dn11 * assign26700_e37093) + (locals.var_t2__blk774 * (-locals.var_ty__blk780_dn11))), ((locals.var_t2__blk774_dn12 * assign26700_e37093) + (locals.var_t2__blk774 * (-locals.var_ty__blk780_dn12))), ((locals.var_t2__blk774_dn17 * assign26700_e37093) + (locals.var_t2__blk774 * (-locals.var_ty__blk780_dn17))),)
    } else {
        (locals.var_ty__blk780, locals.var_ty__blk780_dn0, locals.var_ty__blk780_dn2, locals.var_ty__blk780_dn6, locals.var_ty__blk780_dn7, locals.var_ty__blk780_dn10, locals.var_ty__blk780_dn11, locals.var_ty__blk780_dn12, locals.var_ty__blk780_dn17,)
    }
};
        locals.var_ty__blk780 = assign26700_e37096;
        locals.var_ty__blk780_dn0 = assign26700_e37096_d_n0;
        locals.var_ty__blk780_dn2 = assign26700_e37096_d_n2;
        locals.var_ty__blk780_dn6 = assign26700_e37096_d_n6;
        locals.var_ty__blk780_dn7 = assign26700_e37096_d_n7;
        locals.var_ty__blk780_dn10 = assign26700_e37096_d_n10;
        locals.var_ty__blk780_dn11 = assign26700_e37096_d_n11;
        locals.var_ty__blk780_dn12 = assign26700_e37096_d_n12;
        locals.var_ty__blk780_dn17 = assign26700_e37096_d_n17;
        locals.var_ty__blk780_rv = 0.0;

        let (assign26720_e37119, assign26720_e37119_d_n0, assign26720_e37119_d_n2, assign26720_e37119_d_n6, assign26720_e37119_d_n7, assign26720_e37119_d_n10, assign26720_e37119_d_n11, assign26720_e37119_d_n12, assign26720_e37119_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard872 != 0.0)) {
        let assign26720_e37117: f64 = (locals.var_vbs_bnd + locals.var_ty__blk780);
        (assign26720_e37117, locals.var_ty__blk780_dn0, locals.var_ty__blk780_dn2, locals.var_ty__blk780_dn6, locals.var_ty__blk780_dn7, locals.var_ty__blk780_dn10, locals.var_ty__blk780_dn11, locals.var_ty__blk780_dn12, locals.var_ty__blk780_dn17,)
    } else {
        (locals.var_t10__blk777, locals.var_t10__blk777_dn0, locals.var_t10__blk777_dn2, locals.var_t10__blk777_dn6, locals.var_t10__blk777_dn7, locals.var_t10__blk777_dn10, locals.var_t10__blk777_dn11, locals.var_t10__blk777_dn12, locals.var_t10__blk777_dn17,)
    }
};
        locals.var_t10__blk777 = assign26720_e37119;
        locals.var_t10__blk777_dn0 = assign26720_e37119_d_n0;
        locals.var_t10__blk777_dn2 = assign26720_e37119_d_n2;
        locals.var_t10__blk777_dn6 = assign26720_e37119_d_n6;
        locals.var_t10__blk777_dn7 = assign26720_e37119_d_n7;
        locals.var_t10__blk777_dn10 = assign26720_e37119_d_n10;
        locals.var_t10__blk777_dn11 = assign26720_e37119_d_n11;
        locals.var_t10__blk777_dn12 = assign26720_e37119_d_n12;
        locals.var_t10__blk777_dn17 = assign26720_e37119_d_n17;
        locals.var_t10__blk777_rv = 0.0;

        let (assign26730_e37130, assign26730_e37130_d_n0, assign26730_e37130_d_n2, assign26730_e37130_d_n6, assign26730_e37130_d_n7, assign26730_e37130_d_n10, assign26730_e37130_d_n11, assign26730_e37130_d_n12, assign26730_e37130_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard872 == 0.0)) {
        (locals.var_t0__blk772, locals.var_t0__blk772_dn0, locals.var_t0__blk772_dn2, locals.var_t0__blk772_dn6, locals.var_t0__blk772_dn7, locals.var_t0__blk772_dn10, locals.var_t0__blk772_dn11, locals.var_t0__blk772_dn12, locals.var_t0__blk772_dn17,)
    } else {
        (locals.var_t10__blk777, locals.var_t10__blk777_dn0, locals.var_t10__blk777_dn2, locals.var_t10__blk777_dn6, locals.var_t10__blk777_dn7, locals.var_t10__blk777_dn10, locals.var_t10__blk777_dn11, locals.var_t10__blk777_dn12, locals.var_t10__blk777_dn17,)
    }
};
        locals.var_t10__blk777 = assign26730_e37130;
        locals.var_t10__blk777_dn0 = assign26730_e37130_d_n0;
        locals.var_t10__blk777_dn2 = assign26730_e37130_d_n2;
        locals.var_t10__blk777_dn6 = assign26730_e37130_d_n6;
        locals.var_t10__blk777_dn7 = assign26730_e37130_d_n7;
        locals.var_t10__blk777_dn10 = assign26730_e37130_d_n10;
        locals.var_t10__blk777_dn11 = assign26730_e37130_d_n11;
        locals.var_t10__blk777_dn12 = assign26730_e37130_d_n12;
        locals.var_t10__blk777_dn17 = assign26730_e37130_d_n17;
        locals.var_t10__blk777_rv = 0.0;

        let (assign26750_e37152, assign26750_e37152_d_n0, assign26750_e37152_d_n2, assign26750_e37152_d_n6, assign26750_e37152_d_n7, assign26750_e37152_d_n10, assign26750_e37152_d_n11, assign26750_e37152_d_n12, assign26750_e37152_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26750_e37148: f64 = (-locals.var_t10__blk777);
        let assign26750_e37150: f64 = (assign26750_e37148 - 1e-12);
        (assign26750_e37150, (-locals.var_t10__blk777_dn0), (-locals.var_t10__blk777_dn2), (-locals.var_t10__blk777_dn6), (-locals.var_t10__blk777_dn7), (-locals.var_t10__blk777_dn10), (-locals.var_t10__blk777_dn11), (-locals.var_t10__blk777_dn12), (-locals.var_t10__blk777_dn17),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn12, locals.var_vxbgmtcl_dn17,)
    }
};
        locals.var_vxbgmtcl = assign26750_e37152;
        locals.var_vxbgmtcl_dn0 = assign26750_e37152_d_n0;
        locals.var_vxbgmtcl_dn2 = assign26750_e37152_d_n2;
        locals.var_vxbgmtcl_dn6 = assign26750_e37152_d_n6;
        locals.var_vxbgmtcl_dn7 = assign26750_e37152_d_n7;
        locals.var_vxbgmtcl_dn10 = assign26750_e37152_d_n10;
        locals.var_vxbgmtcl_dn11 = assign26750_e37152_d_n11;
        locals.var_vxbgmtcl_dn12 = assign26750_e37152_d_n12;
        locals.var_vxbgmtcl_dn17 = assign26750_e37152_d_n17;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign26760_e37162, assign26760_e37162_d_n0, assign26760_e37162_d_n2, assign26760_e37162_d_n6, assign26760_e37162_d_n7, assign26760_e37162_d_n10, assign26760_e37162_d_n11, assign26760_e37162_d_n12, assign26760_e37162_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26760_e37160: f64 = (locals.var_cnst0over * locals.var_cox0_inv);
        (assign26760_e37160, (locals.var_cnst0over_dn0 * locals.var_cox0_inv), (locals.var_cnst0over_dn2 * locals.var_cox0_inv), (locals.var_cnst0over_dn6 * locals.var_cox0_inv), (locals.var_cnst0over_dn7 * locals.var_cox0_inv), (locals.var_cnst0over_dn10 * locals.var_cox0_inv), (locals.var_cnst0over_dn11 * locals.var_cox0_inv), (locals.var_cnst0over_dn12 * locals.var_cox0_inv), (locals.var_cnst0over_dn17 * locals.var_cox0_inv),)
    } else {
        (locals.var_fac1__blk802, locals.var_fac1__blk802_dn0, locals.var_fac1__blk802_dn2, locals.var_fac1__blk802_dn6, locals.var_fac1__blk802_dn7, locals.var_fac1__blk802_dn10, locals.var_fac1__blk802_dn11, locals.var_fac1__blk802_dn12, locals.var_fac1__blk802_dn17,)
    }
};
        locals.var_fac1__blk802 = assign26760_e37162;
        locals.var_fac1__blk802_dn0 = assign26760_e37162_d_n0;
        locals.var_fac1__blk802_dn2 = assign26760_e37162_d_n2;
        locals.var_fac1__blk802_dn6 = assign26760_e37162_d_n6;
        locals.var_fac1__blk802_dn7 = assign26760_e37162_d_n7;
        locals.var_fac1__blk802_dn10 = assign26760_e37162_d_n10;
        locals.var_fac1__blk802_dn11 = assign26760_e37162_d_n11;
        locals.var_fac1__blk802_dn12 = assign26760_e37162_d_n12;
        locals.var_fac1__blk802_dn17 = assign26760_e37162_d_n17;
        locals.var_fac1__blk802_rv = 0.0;

        let (assign26770_e37172, assign26770_e37172_d_n0, assign26770_e37172_d_n2, assign26770_e37172_d_n6, assign26770_e37172_d_n7, assign26770_e37172_d_n10, assign26770_e37172_d_n11, assign26770_e37172_d_n12, assign26770_e37172_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26770_e37170: f64 = (locals.var_fac1__blk802 * locals.var_fac1__blk802);
        (assign26770_e37170, ((locals.var_fac1__blk802_dn0 * locals.var_fac1__blk802) + (locals.var_fac1__blk802 * locals.var_fac1__blk802_dn0)), ((locals.var_fac1__blk802_dn2 * locals.var_fac1__blk802) + (locals.var_fac1__blk802 * locals.var_fac1__blk802_dn2)), ((locals.var_fac1__blk802_dn6 * locals.var_fac1__blk802) + (locals.var_fac1__blk802 * locals.var_fac1__blk802_dn6)), ((locals.var_fac1__blk802_dn7 * locals.var_fac1__blk802) + (locals.var_fac1__blk802 * locals.var_fac1__blk802_dn7)), ((locals.var_fac1__blk802_dn10 * locals.var_fac1__blk802) + (locals.var_fac1__blk802 * locals.var_fac1__blk802_dn10)), ((locals.var_fac1__blk802_dn11 * locals.var_fac1__blk802) + (locals.var_fac1__blk802 * locals.var_fac1__blk802_dn11)), ((locals.var_fac1__blk802_dn12 * locals.var_fac1__blk802) + (locals.var_fac1__blk802 * locals.var_fac1__blk802_dn12)), ((locals.var_fac1__blk802_dn17 * locals.var_fac1__blk802) + (locals.var_fac1__blk802 * locals.var_fac1__blk802_dn17)),)
    } else {
        (locals.var_fac1p2__blk803, locals.var_fac1p2__blk803_dn0, locals.var_fac1p2__blk803_dn2, locals.var_fac1p2__blk803_dn6, locals.var_fac1p2__blk803_dn7, locals.var_fac1p2__blk803_dn10, locals.var_fac1p2__blk803_dn11, locals.var_fac1p2__blk803_dn12, locals.var_fac1p2__blk803_dn17,)
    }
};
        locals.var_fac1p2__blk803 = assign26770_e37172;
        locals.var_fac1p2__blk803_dn0 = assign26770_e37172_d_n0;
        locals.var_fac1p2__blk803_dn2 = assign26770_e37172_d_n2;
        locals.var_fac1p2__blk803_dn6 = assign26770_e37172_d_n6;
        locals.var_fac1p2__blk803_dn7 = assign26770_e37172_d_n7;
        locals.var_fac1p2__blk803_dn10 = assign26770_e37172_d_n10;
        locals.var_fac1p2__blk803_dn11 = assign26770_e37172_d_n11;
        locals.var_fac1p2__blk803_dn12 = assign26770_e37172_d_n12;
        locals.var_fac1p2__blk803_dn17 = assign26770_e37172_d_n17;
        locals.var_fac1p2__blk803_rv = 0.0;

        let (assign26780_e37182, assign26780_e37182_d_n0, assign26780_e37182_d_n2, assign26780_e37182_d_n6, assign26780_e37182_d_n7, assign26780_e37182_d_n10, assign26780_e37182_d_n11, assign26780_e37182_d_n12, assign26780_e37182_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26780_e37180: f64 = (locals.var_vgbgmt - locals.var_uc_vfbbt);
        (assign26780_e37180, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn10, locals.var_vgbgmt_dn11, locals.var_vgbgmt_dn12, locals.var_vgbgmt_dn17,)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn0, locals.var_vgpld_dn2, locals.var_vgpld_dn6, locals.var_vgpld_dn7, locals.var_vgpld_dn10, locals.var_vgpld_dn11, locals.var_vgpld_dn12, locals.var_vgpld_dn17,)
    }
};
        locals.var_vgpld = assign26780_e37182;
        locals.var_vgpld_dn0 = assign26780_e37182_d_n0;
        locals.var_vgpld_dn2 = assign26780_e37182_d_n2;
        locals.var_vgpld_dn6 = assign26780_e37182_d_n6;
        locals.var_vgpld_dn7 = assign26780_e37182_d_n7;
        locals.var_vgpld_dn10 = assign26780_e37182_d_n10;
        locals.var_vgpld_dn11 = assign26780_e37182_d_n11;
        locals.var_vgpld_dn12 = assign26780_e37182_d_n12;
        locals.var_vgpld_dn17 = assign26780_e37182_d_n17;
        locals.var_vgpld_rv = 0.0;

        let (assign26790_e37192, assign26790_e37192_d_n0, assign26790_e37192_d_n2, assign26790_e37192_d_n6, assign26790_e37192_d_n7, assign26790_e37192_d_n10, assign26790_e37192_d_n11, assign26790_e37192_d_n12, assign26790_e37192_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26790_e37190: f64 = (locals.var_uc_nsubbttub / locals.var_nin);
        (assign26790_e37190, (((locals.var_uc_nsubbttub_dn0 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn2 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn6 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn7 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn10 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn11 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn12 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn12)) / (locals.var_nin * locals.var_nin)), (((locals.var_uc_nsubbttub_dn17 * locals.var_nin) - (locals.var_uc_nsubbttub * locals.var_nin_dn17)) / (locals.var_nin * locals.var_nin)),)
    } else {
        (locals.var_t0__blk772, locals.var_t0__blk772_dn0, locals.var_t0__blk772_dn2, locals.var_t0__blk772_dn6, locals.var_t0__blk772_dn7, locals.var_t0__blk772_dn10, locals.var_t0__blk772_dn11, locals.var_t0__blk772_dn12, locals.var_t0__blk772_dn17,)
    }
};
        locals.var_t0__blk772 = assign26790_e37192;
        locals.var_t0__blk772_dn0 = assign26790_e37192_d_n0;
        locals.var_t0__blk772_dn2 = assign26790_e37192_d_n2;
        locals.var_t0__blk772_dn6 = assign26790_e37192_d_n6;
        locals.var_t0__blk772_dn7 = assign26790_e37192_d_n7;
        locals.var_t0__blk772_dn10 = assign26790_e37192_d_n10;
        locals.var_t0__blk772_dn11 = assign26790_e37192_d_n11;
        locals.var_t0__blk772_dn12 = assign26790_e37192_d_n12;
        locals.var_t0__blk772_dn17 = assign26790_e37192_d_n17;
        locals.var_t0__blk772_rv = 0.0;

        let (assign26800_e37205, assign26800_e37205_d_n0, assign26800_e37205_d_n2, assign26800_e37205_d_n6, assign26800_e37205_d_n7, assign26800_e37205_d_n10, assign26800_e37205_d_n11, assign26800_e37205_d_n12, assign26800_e37205_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26800_e37200: f64 = (2.0 / locals.var_beta);
        let assign26800_e37202: f64 = (locals.var_t0__blk772).ln();
        let assign26800_e37203: f64 = (assign26800_e37200 * assign26800_e37202);
        (assign26800_e37203, (assign26800_e37200 * (locals.var_t0__blk772_dn0 / locals.var_t0__blk772)), (assign26800_e37200 * (locals.var_t0__blk772_dn2 / locals.var_t0__blk772)), (assign26800_e37200 * (locals.var_t0__blk772_dn6 / locals.var_t0__blk772)), (assign26800_e37200 * (locals.var_t0__blk772_dn7 / locals.var_t0__blk772)), (((-((2.0 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign26800_e37202) + (assign26800_e37200 * (locals.var_t0__blk772_dn10 / locals.var_t0__blk772))), (assign26800_e37200 * (locals.var_t0__blk772_dn11 / locals.var_t0__blk772)), (assign26800_e37200 * (locals.var_t0__blk772_dn12 / locals.var_t0__blk772)), (assign26800_e37200 * (locals.var_t0__blk772_dn17 / locals.var_t0__blk772)),)
    } else {
        (locals.var_pb2over, locals.var_pb2over_dn0, locals.var_pb2over_dn2, locals.var_pb2over_dn6, locals.var_pb2over_dn7, locals.var_pb2over_dn10, locals.var_pb2over_dn11, locals.var_pb2over_dn12, locals.var_pb2over_dn17,)
    }
};
        locals.var_pb2over = assign26800_e37205;
        locals.var_pb2over_dn0 = assign26800_e37205_d_n0;
        locals.var_pb2over_dn2 = assign26800_e37205_d_n2;
        locals.var_pb2over_dn6 = assign26800_e37205_d_n6;
        locals.var_pb2over_dn7 = assign26800_e37205_d_n7;
        locals.var_pb2over_dn10 = assign26800_e37205_d_n10;
        locals.var_pb2over_dn11 = assign26800_e37205_d_n11;
        locals.var_pb2over_dn12 = assign26800_e37205_d_n12;
        locals.var_pb2over_dn17 = assign26800_e37205_d_n17;
        locals.var_pb2over_rv = 0.0;

        let (assign26810_e37214, assign26810_e37214_d_n0, assign26810_e37214_d_n2, assign26810_e37214_d_n6, assign26810_e37214_d_n7, assign26810_e37214_d_n10, assign26810_e37214_d_n11, assign26810_e37214_d_n12, assign26810_e37214_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign26810_e37212: f64 = (-locals.var_vxbgmtcl);
        (assign26810_e37212, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn12), (-locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn12, locals.var_vgb_fb_ld_dn17,)
    }
};
        locals.var_vgb_fb_ld = assign26810_e37214;
        locals.var_vgb_fb_ld_dn0 = assign26810_e37214_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign26810_e37214_d_n2;
        locals.var_vgb_fb_ld_dn6 = assign26810_e37214_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign26810_e37214_d_n7;
        locals.var_vgb_fb_ld_dn10 = assign26810_e37214_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign26810_e37214_d_n11;
        locals.var_vgb_fb_ld_dn12 = assign26810_e37214_d_n12;
        locals.var_vgb_fb_ld_dn17 = assign26810_e37214_d_n17;
        locals.var_vgb_fb_ld_rv = 0.0;

        let assign26820_e37217: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard873 = assign26820_e37217;
        locals.var_guard873_rv = 0.0;

        let (assign26840_e37242, assign26840_e37242_d_n0, assign26840_e37242_d_n2, assign26840_e37242_d_n6, assign26840_e37242_d_n7, assign26840_e37242_d_n10, assign26840_e37242_d_n11, assign26840_e37242_d_n12, assign26840_e37242_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign26840_e37239: f64 = (locals.var_beta * locals.var_cnst0over);
        let assign26840_e37240: f64 = (1.0 / assign26840_e37239);
        (assign26840_e37240, (-((locals.var_beta * locals.var_cnst0over_dn0) / (assign26840_e37239 * assign26840_e37239))), (-((locals.var_beta * locals.var_cnst0over_dn2) / (assign26840_e37239 * assign26840_e37239))), (-((locals.var_beta * locals.var_cnst0over_dn6) / (assign26840_e37239 * assign26840_e37239))), (-((locals.var_beta * locals.var_cnst0over_dn7) / (assign26840_e37239 * assign26840_e37239))), (-(((locals.var_beta_dn10 * locals.var_cnst0over) + (locals.var_beta * locals.var_cnst0over_dn10)) / (assign26840_e37239 * assign26840_e37239))), (-((locals.var_beta * locals.var_cnst0over_dn11) / (assign26840_e37239 * assign26840_e37239))), (-((locals.var_beta * locals.var_cnst0over_dn12) / (assign26840_e37239 * assign26840_e37239))), (-((locals.var_beta * locals.var_cnst0over_dn17) / (assign26840_e37239 * assign26840_e37239))),)
    } else {
        (locals.var_t1__blk773, locals.var_t1__blk773_dn0, locals.var_t1__blk773_dn2, locals.var_t1__blk773_dn6, locals.var_t1__blk773_dn7, locals.var_t1__blk773_dn10, locals.var_t1__blk773_dn11, locals.var_t1__blk773_dn12, locals.var_t1__blk773_dn17,)
    }
};
        locals.var_t1__blk773 = assign26840_e37242;
        locals.var_t1__blk773_dn0 = assign26840_e37242_d_n0;
        locals.var_t1__blk773_dn2 = assign26840_e37242_d_n2;
        locals.var_t1__blk773_dn6 = assign26840_e37242_d_n6;
        locals.var_t1__blk773_dn7 = assign26840_e37242_d_n7;
        locals.var_t1__blk773_dn10 = assign26840_e37242_d_n10;
        locals.var_t1__blk773_dn11 = assign26840_e37242_d_n11;
        locals.var_t1__blk773_dn12 = assign26840_e37242_d_n12;
        locals.var_t1__blk773_dn17 = assign26840_e37242_d_n17;
        locals.var_t1__blk773_rv = 0.0;

        let (assign26850_e37254, assign26850_e37254_d_n0, assign26850_e37254_d_n2, assign26850_e37254_d_n6, assign26850_e37254_d_n7, assign26850_e37254_d_n10, assign26850_e37254_d_n11, assign26850_e37254_d_n12, assign26850_e37254_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign26850_e37252: f64 = (locals.var_t1__blk773 * locals.var_cox0);
        (assign26850_e37252, (locals.var_t1__blk773_dn0 * locals.var_cox0), (locals.var_t1__blk773_dn2 * locals.var_cox0), (locals.var_t1__blk773_dn6 * locals.var_cox0), (locals.var_t1__blk773_dn7 * locals.var_cox0), (locals.var_t1__blk773_dn10 * locals.var_cox0), (locals.var_t1__blk773_dn11 * locals.var_cox0), (locals.var_t1__blk773_dn12 * locals.var_cox0), (locals.var_t1__blk773_dn17 * locals.var_cox0),)
    } else {
        (locals.var_ty__blk780, locals.var_ty__blk780_dn0, locals.var_ty__blk780_dn2, locals.var_ty__blk780_dn6, locals.var_ty__blk780_dn7, locals.var_ty__blk780_dn10, locals.var_ty__blk780_dn11, locals.var_ty__blk780_dn12, locals.var_ty__blk780_dn17,)
    }
};
        locals.var_ty__blk780 = assign26850_e37254;
        locals.var_ty__blk780_dn0 = assign26850_e37254_d_n0;
        locals.var_ty__blk780_dn2 = assign26850_e37254_d_n2;
        locals.var_ty__blk780_dn6 = assign26850_e37254_d_n6;
        locals.var_ty__blk780_dn7 = assign26850_e37254_d_n7;
        locals.var_ty__blk780_dn10 = assign26850_e37254_d_n10;
        locals.var_ty__blk780_dn11 = assign26850_e37254_d_n11;
        locals.var_ty__blk780_dn12 = assign26850_e37254_d_n12;
        locals.var_ty__blk780_dn17 = assign26850_e37254_d_n17;
        locals.var_ty__blk780_rv = 0.0;

        let (assign26860_e37270, assign26860_e37270_d_n0, assign26860_e37270_d_n2, assign26860_e37270_d_n6, assign26860_e37270_d_n7, assign26860_e37270_d_n10, assign26860_e37270_d_n11, assign26860_e37270_d_n12, assign26860_e37270_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign26860_e37265: f64 = (3.0 * 1.414213562373095);
        let assign26860_e37267: f64 = (assign26860_e37265 * locals.var_ty__blk780);
        let assign26860_e37268: f64 = (2.0 + assign26860_e37267);
        (assign26860_e37268, (assign26860_e37265 * locals.var_ty__blk780_dn0), (assign26860_e37265 * locals.var_ty__blk780_dn2), (assign26860_e37265 * locals.var_ty__blk780_dn6), (assign26860_e37265 * locals.var_ty__blk780_dn7), (assign26860_e37265 * locals.var_ty__blk780_dn10), (assign26860_e37265 * locals.var_ty__blk780_dn11), (assign26860_e37265 * locals.var_ty__blk780_dn12), (assign26860_e37265 * locals.var_ty__blk780_dn17),)
    } else {
        (locals.var_ac41__blk807, locals.var_ac41__blk807_dn0, locals.var_ac41__blk807_dn2, locals.var_ac41__blk807_dn6, locals.var_ac41__blk807_dn7, locals.var_ac41__blk807_dn10, locals.var_ac41__blk807_dn11, locals.var_ac41__blk807_dn12, locals.var_ac41__blk807_dn17,)
    }
};
        locals.var_ac41__blk807 = assign26860_e37270;
        locals.var_ac41__blk807_dn0 = assign26860_e37270_d_n0;
        locals.var_ac41__blk807_dn2 = assign26860_e37270_d_n2;
        locals.var_ac41__blk807_dn6 = assign26860_e37270_d_n6;
        locals.var_ac41__blk807_dn7 = assign26860_e37270_d_n7;
        locals.var_ac41__blk807_dn10 = assign26860_e37270_d_n10;
        locals.var_ac41__blk807_dn11 = assign26860_e37270_d_n11;
        locals.var_ac41__blk807_dn12 = assign26860_e37270_d_n12;
        locals.var_ac41__blk807_dn17 = assign26860_e37270_d_n17;
        locals.var_ac41__blk807_rv = 0.0;

        let (assign26870_e37286, assign26870_e37286_d_n0, assign26870_e37286_d_n2, assign26870_e37286_d_n6, assign26870_e37286_d_n7, assign26870_e37286_d_n10, assign26870_e37286_d_n11, assign26870_e37286_d_n12, assign26870_e37286_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign26870_e37280: f64 = (8.0 * locals.var_ac41__blk807);
        let assign26870_e37282: f64 = (assign26870_e37280 * locals.var_ac41__blk807);
        let assign26870_e37284: f64 = (assign26870_e37282 * locals.var_ac41__blk807);
        (assign26870_e37284, (((((8.0 * locals.var_ac41__blk807_dn0) * locals.var_ac41__blk807) + (assign26870_e37280 * locals.var_ac41__blk807_dn0)) * locals.var_ac41__blk807) + (assign26870_e37282 * locals.var_ac41__blk807_dn0)), (((((8.0 * locals.var_ac41__blk807_dn2) * locals.var_ac41__blk807) + (assign26870_e37280 * locals.var_ac41__blk807_dn2)) * locals.var_ac41__blk807) + (assign26870_e37282 * locals.var_ac41__blk807_dn2)), (((((8.0 * locals.var_ac41__blk807_dn6) * locals.var_ac41__blk807) + (assign26870_e37280 * locals.var_ac41__blk807_dn6)) * locals.var_ac41__blk807) + (assign26870_e37282 * locals.var_ac41__blk807_dn6)), (((((8.0 * locals.var_ac41__blk807_dn7) * locals.var_ac41__blk807) + (assign26870_e37280 * locals.var_ac41__blk807_dn7)) * locals.var_ac41__blk807) + (assign26870_e37282 * locals.var_ac41__blk807_dn7)), (((((8.0 * locals.var_ac41__blk807_dn10) * locals.var_ac41__blk807) + (assign26870_e37280 * locals.var_ac41__blk807_dn10)) * locals.var_ac41__blk807) + (assign26870_e37282 * locals.var_ac41__blk807_dn10)), (((((8.0 * locals.var_ac41__blk807_dn11) * locals.var_ac41__blk807) + (assign26870_e37280 * locals.var_ac41__blk807_dn11)) * locals.var_ac41__blk807) + (assign26870_e37282 * locals.var_ac41__blk807_dn11)), (((((8.0 * locals.var_ac41__blk807_dn12) * locals.var_ac41__blk807) + (assign26870_e37280 * locals.var_ac41__blk807_dn12)) * locals.var_ac41__blk807) + (assign26870_e37282 * locals.var_ac41__blk807_dn12)), (((((8.0 * locals.var_ac41__blk807_dn17) * locals.var_ac41__blk807) + (assign26870_e37280 * locals.var_ac41__blk807_dn17)) * locals.var_ac41__blk807) + (assign26870_e37282 * locals.var_ac41__blk807_dn17)),)
    } else {
        (locals.var_ac4__blk808, locals.var_ac4__blk808_dn0, locals.var_ac4__blk808_dn2, locals.var_ac4__blk808_dn6, locals.var_ac4__blk808_dn7, locals.var_ac4__blk808_dn10, locals.var_ac4__blk808_dn11, locals.var_ac4__blk808_dn12, locals.var_ac4__blk808_dn17,)
    }
};
        locals.var_ac4__blk808 = assign26870_e37286;
        locals.var_ac4__blk808_dn0 = assign26870_e37286_d_n0;
        locals.var_ac4__blk808_dn2 = assign26870_e37286_d_n2;
        locals.var_ac4__blk808_dn6 = assign26870_e37286_d_n6;
        locals.var_ac4__blk808_dn7 = assign26870_e37286_d_n7;
        locals.var_ac4__blk808_dn10 = assign26870_e37286_d_n10;
        locals.var_ac4__blk808_dn11 = assign26870_e37286_d_n11;
        locals.var_ac4__blk808_dn12 = assign26870_e37286_d_n12;
        locals.var_ac4__blk808_dn17 = assign26870_e37286_d_n17;
        locals.var_ac4__blk808_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_96(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26880_e37298, assign26880_e37298_d_n0, assign26880_e37298_d_n2, assign26880_e37298_d_n6, assign26880_e37298_d_n7, assign26880_e37298_d_n10, assign26880_e37298_d_n11, assign26880_e37298_d_n12, assign26880_e37298_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign26880_e37296: f64 = (locals.var_eg - locals.var_pb2over);
        (assign26880_e37296, (locals.var_eg_dn0 - locals.var_pb2over_dn0), (locals.var_eg_dn2 - locals.var_pb2over_dn2), (locals.var_eg_dn6 - locals.var_pb2over_dn6), (locals.var_eg_dn7 - locals.var_pb2over_dn7), (locals.var_eg_dn10 - locals.var_pb2over_dn10), (locals.var_eg_dn11 - locals.var_pb2over_dn11), (locals.var_eg_dn12 - locals.var_pb2over_dn12), (locals.var_eg_dn17 - locals.var_pb2over_dn17),)
    } else {
        (locals.var_ps0_min__blk809, locals.var_ps0_min__blk809_dn0, locals.var_ps0_min__blk809_dn2, locals.var_ps0_min__blk809_dn6, locals.var_ps0_min__blk809_dn7, locals.var_ps0_min__blk809_dn10, locals.var_ps0_min__blk809_dn11, locals.var_ps0_min__blk809_dn12, locals.var_ps0_min__blk809_dn17,)
    }
};
        locals.var_ps0_min__blk809 = assign26880_e37298;
        locals.var_ps0_min__blk809_dn0 = assign26880_e37298_d_n0;
        locals.var_ps0_min__blk809_dn2 = assign26880_e37298_d_n2;
        locals.var_ps0_min__blk809_dn6 = assign26880_e37298_d_n6;
        locals.var_ps0_min__blk809_dn7 = assign26880_e37298_d_n7;
        locals.var_ps0_min__blk809_dn10 = assign26880_e37298_d_n10;
        locals.var_ps0_min__blk809_dn11 = assign26880_e37298_d_n11;
        locals.var_ps0_min__blk809_dn12 = assign26880_e37298_d_n12;
        locals.var_ps0_min__blk809_dn17 = assign26880_e37298_d_n17;
        locals.var_ps0_min__blk809_rv = 0.0;

        let (assign26890_e37312, assign26890_e37312_d_n0, assign26890_e37312_d_n2, assign26890_e37312_d_n6, assign26890_e37312_d_n7, assign26890_e37312_d_n10, assign26890_e37312_d_n11, assign26890_e37312_d_n12, assign26890_e37312_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign26890_e37309: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign26890_e37310: f64 = (locals.var_beta * assign26890_e37309);
        (assign26890_e37310, (locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign26890_e37309) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_tx__blk779, locals.var_tx__blk779_dn0, locals.var_tx__blk779_dn2, locals.var_tx__blk779_dn6, locals.var_tx__blk779_dn7, locals.var_tx__blk779_dn10, locals.var_tx__blk779_dn11, locals.var_tx__blk779_dn12, locals.var_tx__blk779_dn17,)
    }
};
        locals.var_tx__blk779 = assign26890_e37312;
        locals.var_tx__blk779_dn0 = assign26890_e37312_d_n0;
        locals.var_tx__blk779_dn2 = assign26890_e37312_d_n2;
        locals.var_tx__blk779_dn6 = assign26890_e37312_d_n6;
        locals.var_tx__blk779_dn7 = assign26890_e37312_d_n7;
        locals.var_tx__blk779_dn10 = assign26890_e37312_d_n10;
        locals.var_tx__blk779_dn11 = assign26890_e37312_d_n11;
        locals.var_tx__blk779_dn12 = assign26890_e37312_d_n12;
        locals.var_tx__blk779_dn17 = assign26890_e37312_d_n17;
        locals.var_tx__blk779_rv = 0.0;

        let (assign26900_e37332, assign26900_e37332_d_n0, assign26900_e37332_d_n2, assign26900_e37332_d_n6, assign26900_e37332_d_n7, assign26900_e37332_d_n10, assign26900_e37332_d_n11, assign26900_e37332_d_n12, assign26900_e37332_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign26900_e37322: f64 = (7.0 * 1.414213562373095);
        let assign26900_e37325: f64 = (9.0 * locals.var_ty__blk780);
        let assign26900_e37328: f64 = (locals.var_tx__blk779 - 2.0);
        let assign26900_e37329: f64 = (assign26900_e37325 * assign26900_e37328);
        let assign26900_e37330: f64 = (assign26900_e37322 - assign26900_e37329);
        (assign26900_e37330, (-(((9.0 * locals.var_ty__blk780_dn0) * assign26900_e37328) + (assign26900_e37325 * locals.var_tx__blk779_dn0))), (-(((9.0 * locals.var_ty__blk780_dn2) * assign26900_e37328) + (assign26900_e37325 * locals.var_tx__blk779_dn2))), (-(((9.0 * locals.var_ty__blk780_dn6) * assign26900_e37328) + (assign26900_e37325 * locals.var_tx__blk779_dn6))), (-(((9.0 * locals.var_ty__blk780_dn7) * assign26900_e37328) + (assign26900_e37325 * locals.var_tx__blk779_dn7))), (-(((9.0 * locals.var_ty__blk780_dn10) * assign26900_e37328) + (assign26900_e37325 * locals.var_tx__blk779_dn10))), (-(((9.0 * locals.var_ty__blk780_dn11) * assign26900_e37328) + (assign26900_e37325 * locals.var_tx__blk779_dn11))), (-(((9.0 * locals.var_ty__blk780_dn12) * assign26900_e37328) + (assign26900_e37325 * locals.var_tx__blk779_dn12))), (-(((9.0 * locals.var_ty__blk780_dn17) * assign26900_e37328) + (assign26900_e37325 * locals.var_tx__blk779_dn17))),)
    } else {
        (locals.var_ac31__blk810, locals.var_ac31__blk810_dn0, locals.var_ac31__blk810_dn2, locals.var_ac31__blk810_dn6, locals.var_ac31__blk810_dn7, locals.var_ac31__blk810_dn10, locals.var_ac31__blk810_dn11, locals.var_ac31__blk810_dn12, locals.var_ac31__blk810_dn17,)
    }
};
        locals.var_ac31__blk810 = assign26900_e37332;
        locals.var_ac31__blk810_dn0 = assign26900_e37332_d_n0;
        locals.var_ac31__blk810_dn2 = assign26900_e37332_d_n2;
        locals.var_ac31__blk810_dn6 = assign26900_e37332_d_n6;
        locals.var_ac31__blk810_dn7 = assign26900_e37332_d_n7;
        locals.var_ac31__blk810_dn10 = assign26900_e37332_d_n10;
        locals.var_ac31__blk810_dn11 = assign26900_e37332_d_n11;
        locals.var_ac31__blk810_dn12 = assign26900_e37332_d_n12;
        locals.var_ac31__blk810_dn17 = assign26900_e37332_d_n17;
        locals.var_ac31__blk810_rv = 0.0;

        let (assign26910_e37344, assign26910_e37344_d_n0, assign26910_e37344_d_n2, assign26910_e37344_d_n6, assign26910_e37344_d_n7, assign26910_e37344_d_n10, assign26910_e37344_d_n11, assign26910_e37344_d_n12, assign26910_e37344_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign26910_e37342: f64 = (locals.var_ac31__blk810 * locals.var_ac31__blk810);
        (assign26910_e37342, ((locals.var_ac31__blk810_dn0 * locals.var_ac31__blk810) + (locals.var_ac31__blk810 * locals.var_ac31__blk810_dn0)), ((locals.var_ac31__blk810_dn2 * locals.var_ac31__blk810) + (locals.var_ac31__blk810 * locals.var_ac31__blk810_dn2)), ((locals.var_ac31__blk810_dn6 * locals.var_ac31__blk810) + (locals.var_ac31__blk810 * locals.var_ac31__blk810_dn6)), ((locals.var_ac31__blk810_dn7 * locals.var_ac31__blk810) + (locals.var_ac31__blk810 * locals.var_ac31__blk810_dn7)), ((locals.var_ac31__blk810_dn10 * locals.var_ac31__blk810) + (locals.var_ac31__blk810 * locals.var_ac31__blk810_dn10)), ((locals.var_ac31__blk810_dn11 * locals.var_ac31__blk810) + (locals.var_ac31__blk810 * locals.var_ac31__blk810_dn11)), ((locals.var_ac31__blk810_dn12 * locals.var_ac31__blk810) + (locals.var_ac31__blk810 * locals.var_ac31__blk810_dn12)), ((locals.var_ac31__blk810_dn17 * locals.var_ac31__blk810) + (locals.var_ac31__blk810 * locals.var_ac31__blk810_dn17)),)
    } else {
        (locals.var_ac3__blk811, locals.var_ac3__blk811_dn0, locals.var_ac3__blk811_dn2, locals.var_ac3__blk811_dn6, locals.var_ac3__blk811_dn7, locals.var_ac3__blk811_dn10, locals.var_ac3__blk811_dn11, locals.var_ac3__blk811_dn12, locals.var_ac3__blk811_dn17,)
    }
};
        locals.var_ac3__blk811 = assign26910_e37344;
        locals.var_ac3__blk811_dn0 = assign26910_e37344_d_n0;
        locals.var_ac3__blk811_dn2 = assign26910_e37344_d_n2;
        locals.var_ac3__blk811_dn6 = assign26910_e37344_d_n6;
        locals.var_ac3__blk811_dn7 = assign26910_e37344_d_n7;
        locals.var_ac3__blk811_dn10 = assign26910_e37344_d_n10;
        locals.var_ac3__blk811_dn11 = assign26910_e37344_d_n11;
        locals.var_ac3__blk811_dn12 = assign26910_e37344_d_n12;
        locals.var_ac3__blk811_dn17 = assign26910_e37344_d_n17;
        locals.var_ac3__blk811_rv = 0.0;

        let assign26920_e37348: f64 = (locals.var_ac3__blk811 * 1e-8);
        let assign26920_e37349: f64 = if locals.var_ac4__blk808 < assign26920_e37348 { 1.0 } else { 0.0 };
        locals.var_guard874 = assign26920_e37349;
        locals.var_guard874_rv = 0.0;

        let (assign26930_e37380, assign26930_e37380_d_n0, assign26930_e37380_d_n2, assign26930_e37380_d_n6, assign26930_e37380_d_n7, assign26930_e37380_d_n10, assign26930_e37380_d_n11, assign26930_e37380_d_n12, assign26930_e37380_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) && (locals.var_guard874 != 0.0)) {
        let assign26930_e37360: f64 = (-7.0);
        let assign26930_e37362: f64 = (assign26930_e37360 * 1.414213562373095);
        let assign26930_e37364: f64 = (assign26930_e37362 + locals.var_ac31__blk810);
        let assign26930_e37367: f64 = (0.5 * locals.var_ac4__blk808);
        let assign26930_e37369: f64 = (assign26930_e37367 / locals.var_ac31__blk810);
        let assign26930_e37370: f64 = (assign26930_e37364 + assign26930_e37369);
        let assign26930_e37373: f64 = (9.0 * locals.var_ty__blk780);
        let assign26930_e37376: f64 = (locals.var_tx__blk779 - 2.0);
        let assign26930_e37377: f64 = (assign26930_e37373 * assign26930_e37376);
        let assign26930_e37378: f64 = (assign26930_e37370 + assign26930_e37377);
        (assign26930_e37378, ((locals.var_ac31__blk810_dn0 + ((((0.5 * locals.var_ac4__blk808_dn0) * locals.var_ac31__blk810) - (assign26930_e37367 * locals.var_ac31__blk810_dn0)) / (locals.var_ac31__blk810 * locals.var_ac31__blk810))) + (((9.0 * locals.var_ty__blk780_dn0) * assign26930_e37376) + (assign26930_e37373 * locals.var_tx__blk779_dn0))), ((locals.var_ac31__blk810_dn2 + ((((0.5 * locals.var_ac4__blk808_dn2) * locals.var_ac31__blk810) - (assign26930_e37367 * locals.var_ac31__blk810_dn2)) / (locals.var_ac31__blk810 * locals.var_ac31__blk810))) + (((9.0 * locals.var_ty__blk780_dn2) * assign26930_e37376) + (assign26930_e37373 * locals.var_tx__blk779_dn2))), ((locals.var_ac31__blk810_dn6 + ((((0.5 * locals.var_ac4__blk808_dn6) * locals.var_ac31__blk810) - (assign26930_e37367 * locals.var_ac31__blk810_dn6)) / (locals.var_ac31__blk810 * locals.var_ac31__blk810))) + (((9.0 * locals.var_ty__blk780_dn6) * assign26930_e37376) + (assign26930_e37373 * locals.var_tx__blk779_dn6))), ((locals.var_ac31__blk810_dn7 + ((((0.5 * locals.var_ac4__blk808_dn7) * locals.var_ac31__blk810) - (assign26930_e37367 * locals.var_ac31__blk810_dn7)) / (locals.var_ac31__blk810 * locals.var_ac31__blk810))) + (((9.0 * locals.var_ty__blk780_dn7) * assign26930_e37376) + (assign26930_e37373 * locals.var_tx__blk779_dn7))), ((locals.var_ac31__blk810_dn10 + ((((0.5 * locals.var_ac4__blk808_dn10) * locals.var_ac31__blk810) - (assign26930_e37367 * locals.var_ac31__blk810_dn10)) / (locals.var_ac31__blk810 * locals.var_ac31__blk810))) + (((9.0 * locals.var_ty__blk780_dn10) * assign26930_e37376) + (assign26930_e37373 * locals.var_tx__blk779_dn10))), ((locals.var_ac31__blk810_dn11 + ((((0.5 * locals.var_ac4__blk808_dn11) * locals.var_ac31__blk810) - (assign26930_e37367 * locals.var_ac31__blk810_dn11)) / (locals.var_ac31__blk810 * locals.var_ac31__blk810))) + (((9.0 * locals.var_ty__blk780_dn11) * assign26930_e37376) + (assign26930_e37373 * locals.var_tx__blk779_dn11))), ((locals.var_ac31__blk810_dn12 + ((((0.5 * locals.var_ac4__blk808_dn12) * locals.var_ac31__blk810) - (assign26930_e37367 * locals.var_ac31__blk810_dn12)) / (locals.var_ac31__blk810 * locals.var_ac31__blk810))) + (((9.0 * locals.var_ty__blk780_dn12) * assign26930_e37376) + (assign26930_e37373 * locals.var_tx__blk779_dn12))), ((locals.var_ac31__blk810_dn17 + ((((0.5 * locals.var_ac4__blk808_dn17) * locals.var_ac31__blk810) - (assign26930_e37367 * locals.var_ac31__blk810_dn17)) / (locals.var_ac31__blk810 * locals.var_ac31__blk810))) + (((9.0 * locals.var_ty__blk780_dn17) * assign26930_e37376) + (assign26930_e37373 * locals.var_tx__blk779_dn17))),)
    } else {
        (locals.var_ac1__blk813, locals.var_ac1__blk813_dn0, locals.var_ac1__blk813_dn2, locals.var_ac1__blk813_dn6, locals.var_ac1__blk813_dn7, locals.var_ac1__blk813_dn10, locals.var_ac1__blk813_dn11, locals.var_ac1__blk813_dn12, locals.var_ac1__blk813_dn17,)
    }
};
        locals.var_ac1__blk813 = assign26930_e37380;
        locals.var_ac1__blk813_dn0 = assign26930_e37380_d_n0;
        locals.var_ac1__blk813_dn2 = assign26930_e37380_d_n2;
        locals.var_ac1__blk813_dn6 = assign26930_e37380_d_n6;
        locals.var_ac1__blk813_dn7 = assign26930_e37380_d_n7;
        locals.var_ac1__blk813_dn10 = assign26930_e37380_d_n10;
        locals.var_ac1__blk813_dn11 = assign26930_e37380_d_n11;
        locals.var_ac1__blk813_dn12 = assign26930_e37380_d_n12;
        locals.var_ac1__blk813_dn17 = assign26930_e37380_d_n17;
        locals.var_ac1__blk813_rv = 0.0;

        let (assign26940_e37396, assign26940_e37396_d_n0, assign26940_e37396_d_n2, assign26940_e37396_d_n6, assign26940_e37396_d_n7, assign26940_e37396_d_n10, assign26940_e37396_d_n11, assign26940_e37396_d_n12, assign26940_e37396_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign26940_e37393: f64 = (locals.var_ac4__blk808 + locals.var_ac3__blk811);
        let assign26940_e37394: f64 = (assign26940_e37393).sqrt();
        (assign26940_e37394, ((locals.var_ac4__blk808_dn0 + locals.var_ac3__blk811_dn0) / (2.0 * assign26940_e37394)), ((locals.var_ac4__blk808_dn2 + locals.var_ac3__blk811_dn2) / (2.0 * assign26940_e37394)), ((locals.var_ac4__blk808_dn6 + locals.var_ac3__blk811_dn6) / (2.0 * assign26940_e37394)), ((locals.var_ac4__blk808_dn7 + locals.var_ac3__blk811_dn7) / (2.0 * assign26940_e37394)), ((locals.var_ac4__blk808_dn10 + locals.var_ac3__blk811_dn10) / (2.0 * assign26940_e37394)), ((locals.var_ac4__blk808_dn11 + locals.var_ac3__blk811_dn11) / (2.0 * assign26940_e37394)), ((locals.var_ac4__blk808_dn12 + locals.var_ac3__blk811_dn12) / (2.0 * assign26940_e37394)), ((locals.var_ac4__blk808_dn17 + locals.var_ac3__blk811_dn17) / (2.0 * assign26940_e37394)),)
    } else {
        (locals.var_ac2__blk812, locals.var_ac2__blk812_dn0, locals.var_ac2__blk812_dn2, locals.var_ac2__blk812_dn6, locals.var_ac2__blk812_dn7, locals.var_ac2__blk812_dn10, locals.var_ac2__blk812_dn11, locals.var_ac2__blk812_dn12, locals.var_ac2__blk812_dn17,)
    }
};
        locals.var_ac2__blk812 = assign26940_e37396;
        locals.var_ac2__blk812_dn0 = assign26940_e37396_d_n0;
        locals.var_ac2__blk812_dn2 = assign26940_e37396_d_n2;
        locals.var_ac2__blk812_dn6 = assign26940_e37396_d_n6;
        locals.var_ac2__blk812_dn7 = assign26940_e37396_d_n7;
        locals.var_ac2__blk812_dn10 = assign26940_e37396_d_n10;
        locals.var_ac2__blk812_dn11 = assign26940_e37396_d_n11;
        locals.var_ac2__blk812_dn12 = assign26940_e37396_d_n12;
        locals.var_ac2__blk812_dn17 = assign26940_e37396_d_n17;
        locals.var_ac2__blk812_rv = 0.0;

        let (assign26950_e37422, assign26950_e37422_d_n0, assign26950_e37422_d_n2, assign26950_e37422_d_n6, assign26950_e37422_d_n7, assign26950_e37422_d_n10, assign26950_e37422_d_n11, assign26950_e37422_d_n12, assign26950_e37422_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign26950_e37408: f64 = (-7.0);
        let assign26950_e37410: f64 = (assign26950_e37408 * 1.414213562373095);
        let assign26950_e37412: f64 = (assign26950_e37410 + locals.var_ac2__blk812);
        let assign26950_e37415: f64 = (9.0 * locals.var_ty__blk780);
        let assign26950_e37418: f64 = (locals.var_tx__blk779 - 2.0);
        let assign26950_e37419: f64 = (assign26950_e37415 * assign26950_e37418);
        let assign26950_e37420: f64 = (assign26950_e37412 + assign26950_e37419);
        (assign26950_e37420, (locals.var_ac2__blk812_dn0 + (((9.0 * locals.var_ty__blk780_dn0) * assign26950_e37418) + (assign26950_e37415 * locals.var_tx__blk779_dn0))), (locals.var_ac2__blk812_dn2 + (((9.0 * locals.var_ty__blk780_dn2) * assign26950_e37418) + (assign26950_e37415 * locals.var_tx__blk779_dn2))), (locals.var_ac2__blk812_dn6 + (((9.0 * locals.var_ty__blk780_dn6) * assign26950_e37418) + (assign26950_e37415 * locals.var_tx__blk779_dn6))), (locals.var_ac2__blk812_dn7 + (((9.0 * locals.var_ty__blk780_dn7) * assign26950_e37418) + (assign26950_e37415 * locals.var_tx__blk779_dn7))), (locals.var_ac2__blk812_dn10 + (((9.0 * locals.var_ty__blk780_dn10) * assign26950_e37418) + (assign26950_e37415 * locals.var_tx__blk779_dn10))), (locals.var_ac2__blk812_dn11 + (((9.0 * locals.var_ty__blk780_dn11) * assign26950_e37418) + (assign26950_e37415 * locals.var_tx__blk779_dn11))), (locals.var_ac2__blk812_dn12 + (((9.0 * locals.var_ty__blk780_dn12) * assign26950_e37418) + (assign26950_e37415 * locals.var_tx__blk779_dn12))), (locals.var_ac2__blk812_dn17 + (((9.0 * locals.var_ty__blk780_dn17) * assign26950_e37418) + (assign26950_e37415 * locals.var_tx__blk779_dn17))),)
    } else {
        (locals.var_ac1__blk813, locals.var_ac1__blk813_dn0, locals.var_ac1__blk813_dn2, locals.var_ac1__blk813_dn6, locals.var_ac1__blk813_dn7, locals.var_ac1__blk813_dn10, locals.var_ac1__blk813_dn11, locals.var_ac1__blk813_dn12, locals.var_ac1__blk813_dn17,)
    }
};
        locals.var_ac1__blk813 = assign26950_e37422;
        locals.var_ac1__blk813_dn0 = assign26950_e37422_d_n0;
        locals.var_ac1__blk813_dn2 = assign26950_e37422_d_n2;
        locals.var_ac1__blk813_dn6 = assign26950_e37422_d_n6;
        locals.var_ac1__blk813_dn7 = assign26950_e37422_d_n7;
        locals.var_ac1__blk813_dn10 = assign26950_e37422_d_n10;
        locals.var_ac1__blk813_dn11 = assign26950_e37422_d_n11;
        locals.var_ac1__blk813_dn12 = assign26950_e37422_d_n12;
        locals.var_ac1__blk813_dn17 = assign26950_e37422_d_n17;
        locals.var_ac1__blk813_rv = 0.0;

        let (assign26960_e37434, assign26960_e37434_d_n0, assign26960_e37434_d_n2, assign26960_e37434_d_n6, assign26960_e37434_d_n7, assign26960_e37434_d_n10, assign26960_e37434_d_n11, assign26960_e37434_d_n12, assign26960_e37434_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign26960_e37432: f64 = (locals.var_ac1__blk813).powf(0.3333333333333333);
        (assign26960_e37432, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk813).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk813_dn0)) } } else { (assign26960_e37432 * (0.3333333333333333 * (locals.var_ac1__blk813_dn0 / locals.var_ac1__blk813))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk813).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk813_dn2)) } } else { (assign26960_e37432 * (0.3333333333333333 * (locals.var_ac1__blk813_dn2 / locals.var_ac1__blk813))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk813).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk813_dn6)) } } else { (assign26960_e37432 * (0.3333333333333333 * (locals.var_ac1__blk813_dn6 / locals.var_ac1__blk813))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk813).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk813_dn7)) } } else { (assign26960_e37432 * (0.3333333333333333 * (locals.var_ac1__blk813_dn7 / locals.var_ac1__blk813))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk813).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk813_dn10)) } } else { (assign26960_e37432 * (0.3333333333333333 * (locals.var_ac1__blk813_dn10 / locals.var_ac1__blk813))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk813).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk813_dn11)) } } else { (assign26960_e37432 * (0.3333333333333333 * (locals.var_ac1__blk813_dn11 / locals.var_ac1__blk813))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk813).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk813_dn12)) } } else { (assign26960_e37432 * (0.3333333333333333 * (locals.var_ac1__blk813_dn12 / locals.var_ac1__blk813))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk813).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk813_dn17)) } } else { (assign26960_e37432 * (0.3333333333333333 * (locals.var_ac1__blk813_dn17 / locals.var_ac1__blk813))) },)
    } else {
        (locals.var_acd__blk814, locals.var_acd__blk814_dn0, locals.var_acd__blk814_dn2, locals.var_acd__blk814_dn6, locals.var_acd__blk814_dn7, locals.var_acd__blk814_dn10, locals.var_acd__blk814_dn11, locals.var_acd__blk814_dn12, locals.var_acd__blk814_dn17,)
    }
};
        locals.var_acd__blk814 = assign26960_e37434;
        locals.var_acd__blk814_dn0 = assign26960_e37434_d_n0;
        locals.var_acd__blk814_dn2 = assign26960_e37434_d_n2;
        locals.var_acd__blk814_dn6 = assign26960_e37434_d_n6;
        locals.var_acd__blk814_dn7 = assign26960_e37434_d_n7;
        locals.var_acd__blk814_dn10 = assign26960_e37434_d_n10;
        locals.var_acd__blk814_dn11 = assign26960_e37434_d_n11;
        locals.var_acd__blk814_dn12 = assign26960_e37434_d_n12;
        locals.var_acd__blk814_dn17 = assign26960_e37434_d_n17;
        locals.var_acd__blk814_rv = 0.0;

        let (assign26970_e37461, assign26970_e37461_d_n0, assign26970_e37461_d_n2, assign26970_e37461_d_n6, assign26970_e37461_d_n7, assign26970_e37461_d_n10, assign26970_e37461_d_n11, assign26970_e37461_d_n12, assign26970_e37461_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign26970_e37443: f64 = (-4.0);
        let assign26970_e37445: f64 = (assign26970_e37443 * 1.414213562373095);
        let assign26970_e37448: f64 = (12.0 * locals.var_ty__blk780);
        let assign26970_e37449: f64 = (assign26970_e37445 - assign26970_e37448);
        let assign26970_e37452: f64 = (2.0 * locals.var_acd__blk814);
        let assign26970_e37453: f64 = (assign26970_e37449 + assign26970_e37452);
        let assign26970_e37456: f64 = (1.414213562373095 * locals.var_acd__blk814);
        let assign26970_e37458: f64 = (assign26970_e37456 * locals.var_acd__blk814);
        let assign26970_e37459: f64 = (assign26970_e37453 + assign26970_e37458);
        (assign26970_e37459, (((-(12.0 * locals.var_ty__blk780_dn0)) + (2.0 * locals.var_acd__blk814_dn0)) + (((1.414213562373095 * locals.var_acd__blk814_dn0) * locals.var_acd__blk814) + (assign26970_e37456 * locals.var_acd__blk814_dn0))), (((-(12.0 * locals.var_ty__blk780_dn2)) + (2.0 * locals.var_acd__blk814_dn2)) + (((1.414213562373095 * locals.var_acd__blk814_dn2) * locals.var_acd__blk814) + (assign26970_e37456 * locals.var_acd__blk814_dn2))), (((-(12.0 * locals.var_ty__blk780_dn6)) + (2.0 * locals.var_acd__blk814_dn6)) + (((1.414213562373095 * locals.var_acd__blk814_dn6) * locals.var_acd__blk814) + (assign26970_e37456 * locals.var_acd__blk814_dn6))), (((-(12.0 * locals.var_ty__blk780_dn7)) + (2.0 * locals.var_acd__blk814_dn7)) + (((1.414213562373095 * locals.var_acd__blk814_dn7) * locals.var_acd__blk814) + (assign26970_e37456 * locals.var_acd__blk814_dn7))), (((-(12.0 * locals.var_ty__blk780_dn10)) + (2.0 * locals.var_acd__blk814_dn10)) + (((1.414213562373095 * locals.var_acd__blk814_dn10) * locals.var_acd__blk814) + (assign26970_e37456 * locals.var_acd__blk814_dn10))), (((-(12.0 * locals.var_ty__blk780_dn11)) + (2.0 * locals.var_acd__blk814_dn11)) + (((1.414213562373095 * locals.var_acd__blk814_dn11) * locals.var_acd__blk814) + (assign26970_e37456 * locals.var_acd__blk814_dn11))), (((-(12.0 * locals.var_ty__blk780_dn12)) + (2.0 * locals.var_acd__blk814_dn12)) + (((1.414213562373095 * locals.var_acd__blk814_dn12) * locals.var_acd__blk814) + (assign26970_e37456 * locals.var_acd__blk814_dn12))), (((-(12.0 * locals.var_ty__blk780_dn17)) + (2.0 * locals.var_acd__blk814_dn17)) + (((1.414213562373095 * locals.var_acd__blk814_dn17) * locals.var_acd__blk814) + (assign26970_e37456 * locals.var_acd__blk814_dn17))),)
    } else {
        (locals.var_acn__blk815, locals.var_acn__blk815_dn0, locals.var_acn__blk815_dn2, locals.var_acn__blk815_dn6, locals.var_acn__blk815_dn7, locals.var_acn__blk815_dn10, locals.var_acn__blk815_dn11, locals.var_acn__blk815_dn12, locals.var_acn__blk815_dn17,)
    }
};
        locals.var_acn__blk815 = assign26970_e37461;
        locals.var_acn__blk815_dn0 = assign26970_e37461_d_n0;
        locals.var_acn__blk815_dn2 = assign26970_e37461_d_n2;
        locals.var_acn__blk815_dn6 = assign26970_e37461_d_n6;
        locals.var_acn__blk815_dn7 = assign26970_e37461_d_n7;
        locals.var_acn__blk815_dn10 = assign26970_e37461_d_n10;
        locals.var_acn__blk815_dn11 = assign26970_e37461_d_n11;
        locals.var_acn__blk815_dn12 = assign26970_e37461_d_n12;
        locals.var_acn__blk815_dn17 = assign26970_e37461_d_n17;
        locals.var_acn__blk815_rv = 0.0;

        let (assign26980_e37473, assign26980_e37473_d_n0, assign26980_e37473_d_n2, assign26980_e37473_d_n6, assign26980_e37473_d_n7, assign26980_e37473_d_n10, assign26980_e37473_d_n11, assign26980_e37473_d_n12, assign26980_e37473_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign26980_e37471: f64 = (locals.var_acn__blk815 / locals.var_acd__blk814);
        (assign26980_e37471, (((locals.var_acn__blk815_dn0 * locals.var_acd__blk814) - (locals.var_acn__blk815 * locals.var_acd__blk814_dn0)) / (locals.var_acd__blk814 * locals.var_acd__blk814)), (((locals.var_acn__blk815_dn2 * locals.var_acd__blk814) - (locals.var_acn__blk815 * locals.var_acd__blk814_dn2)) / (locals.var_acd__blk814 * locals.var_acd__blk814)), (((locals.var_acn__blk815_dn6 * locals.var_acd__blk814) - (locals.var_acn__blk815 * locals.var_acd__blk814_dn6)) / (locals.var_acd__blk814 * locals.var_acd__blk814)), (((locals.var_acn__blk815_dn7 * locals.var_acd__blk814) - (locals.var_acn__blk815 * locals.var_acd__blk814_dn7)) / (locals.var_acd__blk814 * locals.var_acd__blk814)), (((locals.var_acn__blk815_dn10 * locals.var_acd__blk814) - (locals.var_acn__blk815 * locals.var_acd__blk814_dn10)) / (locals.var_acd__blk814 * locals.var_acd__blk814)), (((locals.var_acn__blk815_dn11 * locals.var_acd__blk814) - (locals.var_acn__blk815 * locals.var_acd__blk814_dn11)) / (locals.var_acd__blk814 * locals.var_acd__blk814)), (((locals.var_acn__blk815_dn12 * locals.var_acd__blk814) - (locals.var_acn__blk815 * locals.var_acd__blk814_dn12)) / (locals.var_acd__blk814 * locals.var_acd__blk814)), (((locals.var_acn__blk815_dn17 * locals.var_acd__blk814) - (locals.var_acn__blk815 * locals.var_acd__blk814_dn17)) / (locals.var_acd__blk814 * locals.var_acd__blk814)),)
    } else {
        (locals.var_chi__blk816, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    }
};
        locals.var_chi__blk816 = assign26980_e37473;
        locals.var_chi__blk816_dn0 = assign26980_e37473_d_n0;
        locals.var_chi__blk816_dn2 = assign26980_e37473_d_n2;
        locals.var_chi__blk816_dn6 = assign26980_e37473_d_n6;
        locals.var_chi__blk816_dn7 = assign26980_e37473_d_n7;
        locals.var_chi__blk816_dn10 = assign26980_e37473_d_n10;
        locals.var_chi__blk816_dn11 = assign26980_e37473_d_n11;
        locals.var_chi__blk816_dn12 = assign26980_e37473_d_n12;
        locals.var_chi__blk816_dn17 = assign26980_e37473_d_n17;
        locals.var_chi__blk816_rv = 0.0;

        let (assign26990_e37487, assign26990_e37487_d_n0, assign26990_e37487_d_n2, assign26990_e37487_d_n6, assign26990_e37487_d_n7, assign26990_e37487_d_n10, assign26990_e37487_d_n11, assign26990_e37487_d_n12, assign26990_e37487_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign26990_e37483: f64 = (locals.var_chi__blk816 * locals.var_beta_inv);
        let assign26990_e37485: f64 = (assign26990_e37483 - locals.var_vxbgmtcl);
        (assign26990_e37485, ((locals.var_chi__blk816_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk816_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk816_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk816_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn7), (((locals.var_chi__blk816_dn10 * locals.var_beta_inv) + (locals.var_chi__blk816 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk816_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk816_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk816_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_psa__blk817, locals.var_psa__blk817_dn0, locals.var_psa__blk817_dn2, locals.var_psa__blk817_dn6, locals.var_psa__blk817_dn7, locals.var_psa__blk817_dn10, locals.var_psa__blk817_dn11, locals.var_psa__blk817_dn12, locals.var_psa__blk817_dn17,)
    }
};
        locals.var_psa__blk817 = assign26990_e37487;
        locals.var_psa__blk817_dn0 = assign26990_e37487_d_n0;
        locals.var_psa__blk817_dn2 = assign26990_e37487_d_n2;
        locals.var_psa__blk817_dn6 = assign26990_e37487_d_n6;
        locals.var_psa__blk817_dn7 = assign26990_e37487_d_n7;
        locals.var_psa__blk817_dn10 = assign26990_e37487_d_n10;
        locals.var_psa__blk817_dn11 = assign26990_e37487_d_n11;
        locals.var_psa__blk817_dn12 = assign26990_e37487_d_n12;
        locals.var_psa__blk817_dn17 = assign26990_e37487_d_n17;
        locals.var_psa__blk817_rv = 0.0;

        let (assign27000_e37499, assign27000_e37499_d_n0, assign27000_e37499_d_n2, assign27000_e37499_d_n6, assign27000_e37499_d_n7, assign27000_e37499_d_n10, assign27000_e37499_d_n11, assign27000_e37499_d_n12, assign27000_e37499_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign27000_e37497: f64 = (locals.var_psa__blk817 + locals.var_vxbgmtcl);
        (assign27000_e37497, (locals.var_psa__blk817_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_psa__blk817_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_psa__blk817_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_psa__blk817_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_psa__blk817_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_psa__blk817_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_psa__blk817_dn12 + locals.var_vxbgmtcl_dn12), (locals.var_psa__blk817_dn17 + locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_t1__blk773, locals.var_t1__blk773_dn0, locals.var_t1__blk773_dn2, locals.var_t1__blk773_dn6, locals.var_t1__blk773_dn7, locals.var_t1__blk773_dn10, locals.var_t1__blk773_dn11, locals.var_t1__blk773_dn12, locals.var_t1__blk773_dn17,)
    }
};
        locals.var_t1__blk773 = assign27000_e37499;
        locals.var_t1__blk773_dn0 = assign27000_e37499_d_n0;
        locals.var_t1__blk773_dn2 = assign27000_e37499_d_n2;
        locals.var_t1__blk773_dn6 = assign27000_e37499_d_n6;
        locals.var_t1__blk773_dn7 = assign27000_e37499_d_n7;
        locals.var_t1__blk773_dn10 = assign27000_e37499_d_n10;
        locals.var_t1__blk773_dn11 = assign27000_e37499_d_n11;
        locals.var_t1__blk773_dn12 = assign27000_e37499_d_n12;
        locals.var_t1__blk773_dn17 = assign27000_e37499_d_n17;
        locals.var_t1__blk773_rv = 0.0;

        let (assign27010_e37511, assign27010_e37511_d_n0, assign27010_e37511_d_n2, assign27010_e37511_d_n6, assign27010_e37511_d_n7, assign27010_e37511_d_n10, assign27010_e37511_d_n11, assign27010_e37511_d_n12, assign27010_e37511_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign27010_e37509: f64 = (locals.var_t1__blk773 / locals.var_ps0_min__blk809);
        (assign27010_e37509, (((locals.var_t1__blk773_dn0 * locals.var_ps0_min__blk809) - (locals.var_t1__blk773 * locals.var_ps0_min__blk809_dn0)) / (locals.var_ps0_min__blk809 * locals.var_ps0_min__blk809)), (((locals.var_t1__blk773_dn2 * locals.var_ps0_min__blk809) - (locals.var_t1__blk773 * locals.var_ps0_min__blk809_dn2)) / (locals.var_ps0_min__blk809 * locals.var_ps0_min__blk809)), (((locals.var_t1__blk773_dn6 * locals.var_ps0_min__blk809) - (locals.var_t1__blk773 * locals.var_ps0_min__blk809_dn6)) / (locals.var_ps0_min__blk809 * locals.var_ps0_min__blk809)), (((locals.var_t1__blk773_dn7 * locals.var_ps0_min__blk809) - (locals.var_t1__blk773 * locals.var_ps0_min__blk809_dn7)) / (locals.var_ps0_min__blk809 * locals.var_ps0_min__blk809)), (((locals.var_t1__blk773_dn10 * locals.var_ps0_min__blk809) - (locals.var_t1__blk773 * locals.var_ps0_min__blk809_dn10)) / (locals.var_ps0_min__blk809 * locals.var_ps0_min__blk809)), (((locals.var_t1__blk773_dn11 * locals.var_ps0_min__blk809) - (locals.var_t1__blk773 * locals.var_ps0_min__blk809_dn11)) / (locals.var_ps0_min__blk809 * locals.var_ps0_min__blk809)), (((locals.var_t1__blk773_dn12 * locals.var_ps0_min__blk809) - (locals.var_t1__blk773 * locals.var_ps0_min__blk809_dn12)) / (locals.var_ps0_min__blk809 * locals.var_ps0_min__blk809)), (((locals.var_t1__blk773_dn17 * locals.var_ps0_min__blk809) - (locals.var_t1__blk773 * locals.var_ps0_min__blk809_dn17)) / (locals.var_ps0_min__blk809 * locals.var_ps0_min__blk809)),)
    } else {
        (locals.var_t2__blk774, locals.var_t2__blk774_dn0, locals.var_t2__blk774_dn2, locals.var_t2__blk774_dn6, locals.var_t2__blk774_dn7, locals.var_t2__blk774_dn10, locals.var_t2__blk774_dn11, locals.var_t2__blk774_dn12, locals.var_t2__blk774_dn17,)
    }
};
        locals.var_t2__blk774 = assign27010_e37511;
        locals.var_t2__blk774_dn0 = assign27010_e37511_d_n0;
        locals.var_t2__blk774_dn2 = assign27010_e37511_d_n2;
        locals.var_t2__blk774_dn6 = assign27010_e37511_d_n6;
        locals.var_t2__blk774_dn7 = assign27010_e37511_d_n7;
        locals.var_t2__blk774_dn10 = assign27010_e37511_d_n10;
        locals.var_t2__blk774_dn11 = assign27010_e37511_d_n11;
        locals.var_t2__blk774_dn12 = assign27010_e37511_d_n12;
        locals.var_t2__blk774_dn17 = assign27010_e37511_d_n17;
        locals.var_t2__blk774_rv = 0.0;

        let (assign27020_e37526, assign27020_e37526_d_n0, assign27020_e37526_d_n2, assign27020_e37526_d_n6, assign27020_e37526_d_n7, assign27020_e37526_d_n10, assign27020_e37526_d_n11, assign27020_e37526_d_n12, assign27020_e37526_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign27020_e37522: f64 = (locals.var_t2__blk774 * locals.var_t2__blk774);
        let assign27020_e37523: f64 = (1.0 + assign27020_e37522);
        let assign27020_e37524: f64 = (assign27020_e37523).sqrt();
        (assign27020_e37524, (((locals.var_t2__blk774_dn0 * locals.var_t2__blk774) + (locals.var_t2__blk774 * locals.var_t2__blk774_dn0)) / (2.0 * assign27020_e37524)), (((locals.var_t2__blk774_dn2 * locals.var_t2__blk774) + (locals.var_t2__blk774 * locals.var_t2__blk774_dn2)) / (2.0 * assign27020_e37524)), (((locals.var_t2__blk774_dn6 * locals.var_t2__blk774) + (locals.var_t2__blk774 * locals.var_t2__blk774_dn6)) / (2.0 * assign27020_e37524)), (((locals.var_t2__blk774_dn7 * locals.var_t2__blk774) + (locals.var_t2__blk774 * locals.var_t2__blk774_dn7)) / (2.0 * assign27020_e37524)), (((locals.var_t2__blk774_dn10 * locals.var_t2__blk774) + (locals.var_t2__blk774 * locals.var_t2__blk774_dn10)) / (2.0 * assign27020_e37524)), (((locals.var_t2__blk774_dn11 * locals.var_t2__blk774) + (locals.var_t2__blk774 * locals.var_t2__blk774_dn11)) / (2.0 * assign27020_e37524)), (((locals.var_t2__blk774_dn12 * locals.var_t2__blk774) + (locals.var_t2__blk774 * locals.var_t2__blk774_dn12)) / (2.0 * assign27020_e37524)), (((locals.var_t2__blk774_dn17 * locals.var_t2__blk774) + (locals.var_t2__blk774 * locals.var_t2__blk774_dn17)) / (2.0 * assign27020_e37524)),)
    } else {
        (locals.var_t3__blk775, locals.var_t3__blk775_dn0, locals.var_t3__blk775_dn2, locals.var_t3__blk775_dn6, locals.var_t3__blk775_dn7, locals.var_t3__blk775_dn10, locals.var_t3__blk775_dn11, locals.var_t3__blk775_dn12, locals.var_t3__blk775_dn17,)
    }
};
        locals.var_t3__blk775 = assign27020_e37526;
        locals.var_t3__blk775_dn0 = assign27020_e37526_d_n0;
        locals.var_t3__blk775_dn2 = assign27020_e37526_d_n2;
        locals.var_t3__blk775_dn6 = assign27020_e37526_d_n6;
        locals.var_t3__blk775_dn7 = assign27020_e37526_d_n7;
        locals.var_t3__blk775_dn10 = assign27020_e37526_d_n10;
        locals.var_t3__blk775_dn11 = assign27020_e37526_d_n11;
        locals.var_t3__blk775_dn12 = assign27020_e37526_d_n12;
        locals.var_t3__blk775_dn17 = assign27020_e37526_d_n17;
        locals.var_t3__blk775_rv = 0.0;

        let (assign27030_e37540, assign27030_e37540_d_n0, assign27030_e37540_d_n2, assign27030_e37540_d_n6, assign27030_e37540_d_n7, assign27030_e37540_d_n10, assign27030_e37540_d_n11, assign27030_e37540_d_n12, assign27030_e37540_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign27030_e37536: f64 = (locals.var_t1__blk773 / locals.var_t3__blk775);
        let assign27030_e37538: f64 = (assign27030_e37536 - locals.var_vxbgmtcl);
        (assign27030_e37538, ((((locals.var_t1__blk773_dn0 * locals.var_t3__blk775) - (locals.var_t1__blk773 * locals.var_t3__blk775_dn0)) / (locals.var_t3__blk775 * locals.var_t3__blk775)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1__blk773_dn2 * locals.var_t3__blk775) - (locals.var_t1__blk773 * locals.var_t3__blk775_dn2)) / (locals.var_t3__blk775 * locals.var_t3__blk775)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1__blk773_dn6 * locals.var_t3__blk775) - (locals.var_t1__blk773 * locals.var_t3__blk775_dn6)) / (locals.var_t3__blk775 * locals.var_t3__blk775)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1__blk773_dn7 * locals.var_t3__blk775) - (locals.var_t1__blk773 * locals.var_t3__blk775_dn7)) / (locals.var_t3__blk775 * locals.var_t3__blk775)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1__blk773_dn10 * locals.var_t3__blk775) - (locals.var_t1__blk773 * locals.var_t3__blk775_dn10)) / (locals.var_t3__blk775 * locals.var_t3__blk775)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1__blk773_dn11 * locals.var_t3__blk775) - (locals.var_t1__blk773 * locals.var_t3__blk775_dn11)) / (locals.var_t3__blk775 * locals.var_t3__blk775)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1__blk773_dn12 * locals.var_t3__blk775) - (locals.var_t1__blk773 * locals.var_t3__blk775_dn12)) / (locals.var_t3__blk775 * locals.var_t3__blk775)) - locals.var_vxbgmtcl_dn12), ((((locals.var_t1__blk773_dn17 * locals.var_t3__blk775) - (locals.var_t1__blk773 * locals.var_t3__blk775_dn17)) / (locals.var_t3__blk775 * locals.var_t3__blk775)) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
        locals.var_ps0ld = assign27030_e37540;
        locals.var_ps0ld_dn0 = assign27030_e37540_d_n0;
        locals.var_ps0ld_dn2 = assign27030_e37540_d_n2;
        locals.var_ps0ld_dn6 = assign27030_e37540_d_n6;
        locals.var_ps0ld_dn7 = assign27030_e37540_d_n7;
        locals.var_ps0ld_dn10 = assign27030_e37540_d_n10;
        locals.var_ps0ld_dn11 = assign27030_e37540_d_n11;
        locals.var_ps0ld_dn12 = assign27030_e37540_d_n12;
        locals.var_ps0ld_dn17 = assign27030_e37540_d_n17;
        locals.var_ps0ld_rv = 0.0;

        let (assign27040_e37552, assign27040_e37552_d_n0, assign27040_e37552_d_n2, assign27040_e37552_d_n6, assign27040_e37552_d_n7, assign27040_e37552_d_n10, assign27040_e37552_d_n11, assign27040_e37552_d_n12, assign27040_e37552_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign27040_e37550: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign27040_e37550, (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn10 - locals.var_ps0ld_dn10), (locals.var_vgpld_dn11 - locals.var_ps0ld_dn11), (locals.var_vgpld_dn12 - locals.var_ps0ld_dn12), (locals.var_vgpld_dn17 - locals.var_ps0ld_dn17),)
    } else {
        (locals.var_t2__blk774, locals.var_t2__blk774_dn0, locals.var_t2__blk774_dn2, locals.var_t2__blk774_dn6, locals.var_t2__blk774_dn7, locals.var_t2__blk774_dn10, locals.var_t2__blk774_dn11, locals.var_t2__blk774_dn12, locals.var_t2__blk774_dn17,)
    }
};
        locals.var_t2__blk774 = assign27040_e37552;
        locals.var_t2__blk774_dn0 = assign27040_e37552_d_n0;
        locals.var_t2__blk774_dn2 = assign27040_e37552_d_n2;
        locals.var_t2__blk774_dn6 = assign27040_e37552_d_n6;
        locals.var_t2__blk774_dn7 = assign27040_e37552_d_n7;
        locals.var_t2__blk774_dn10 = assign27040_e37552_d_n10;
        locals.var_t2__blk774_dn11 = assign27040_e37552_d_n11;
        locals.var_t2__blk774_dn12 = assign27040_e37552_d_n12;
        locals.var_t2__blk774_dn17 = assign27040_e37552_d_n17;
        locals.var_t2__blk774_rv = 0.0;

        let (assign27050_e37564, assign27050_e37564_d_n0, assign27050_e37564_d_n2, assign27050_e37564_d_n6, assign27050_e37564_d_n7, assign27050_e37564_d_n10, assign27050_e37564_d_n11, assign27050_e37564_d_n12, assign27050_e37564_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign27050_e37562: f64 = (locals.var_cox0 * locals.var_t2__blk774);
        (assign27050_e37562, (locals.var_cox0 * locals.var_t2__blk774_dn0), (locals.var_cox0 * locals.var_t2__blk774_dn2), (locals.var_cox0 * locals.var_t2__blk774_dn6), (locals.var_cox0 * locals.var_t2__blk774_dn7), (locals.var_cox0 * locals.var_t2__blk774_dn10), (locals.var_cox0 * locals.var_t2__blk774_dn11), (locals.var_cox0 * locals.var_t2__blk774_dn12), (locals.var_cox0 * locals.var_t2__blk774_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign27050_e37564;
        locals.var_qsuld_dn0 = assign27050_e37564_d_n0;
        locals.var_qsuld_dn2 = assign27050_e37564_d_n2;
        locals.var_qsuld_dn6 = assign27050_e37564_d_n6;
        locals.var_qsuld_dn7 = assign27050_e37564_d_n7;
        locals.var_qsuld_dn10 = assign27050_e37564_d_n10;
        locals.var_qsuld_dn11 = assign27050_e37564_d_n11;
        locals.var_qsuld_dn12 = assign27050_e37564_d_n12;
        locals.var_qsuld_dn17 = assign27050_e37564_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign27060_e37574, assign27060_e37574_d_n0, assign27060_e37574_d_n2, assign27060_e37574_d_n6, assign27060_e37574_d_n7, assign27060_e37574_d_n10, assign27060_e37574_d_n11, assign27060_e37574_d_n12, assign27060_e37574_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign27060_e37574;
        locals.var_qbuld_dn0 = assign27060_e37574_d_n0;
        locals.var_qbuld_dn2 = assign27060_e37574_d_n2;
        locals.var_qbuld_dn6 = assign27060_e37574_d_n6;
        locals.var_qbuld_dn7 = assign27060_e37574_d_n7;
        locals.var_qbuld_dn10 = assign27060_e37574_d_n10;
        locals.var_qbuld_dn11 = assign27060_e37574_d_n11;
        locals.var_qbuld_dn12 = assign27060_e37574_d_n12;
        locals.var_qbuld_dn17 = assign27060_e37574_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign27080_e37596, assign27080_e37596_d_n0, assign27080_e37596_d_n2, assign27080_e37596_d_n6, assign27080_e37596_d_n7, assign27080_e37596_d_n10, assign27080_e37596_d_n11, assign27080_e37596_d_n12, assign27080_e37596_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        (3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi__blk816, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    }
};
        locals.var_chi__blk816 = assign27080_e37596;
        locals.var_chi__blk816_dn0 = assign27080_e37596_d_n0;
        locals.var_chi__blk816_dn2 = assign27080_e37596_d_n2;
        locals.var_chi__blk816_dn6 = assign27080_e37596_d_n6;
        locals.var_chi__blk816_dn7 = assign27080_e37596_d_n7;
        locals.var_chi__blk816_dn10 = assign27080_e37596_d_n10;
        locals.var_chi__blk816_dn11 = assign27080_e37596_d_n11;
        locals.var_chi__blk816_dn12 = assign27080_e37596_d_n12;
        locals.var_chi__blk816_dn17 = assign27080_e37596_d_n17;
        locals.var_chi__blk816_rv = 0.0;

        let (assign27090_e37611, assign27090_e37611_d_n0, assign27090_e37611_d_n2, assign27090_e37611_d_n6, assign27090_e37611_d_n7, assign27090_e37611_d_n10, assign27090_e37611_d_n11, assign27090_e37611_d_n12, assign27090_e37611_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27090_e37607: f64 = (locals.var_chi__blk816 / locals.var_beta);
        let assign27090_e37609: f64 = (assign27090_e37607 - locals.var_vxbgmtcl);
        (assign27090_e37609, ((locals.var_chi__blk816_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk816_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk816_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk816_dn7 / locals.var_beta) - locals.var_vxbgmtcl_dn7), ((((locals.var_chi__blk816_dn10 * locals.var_beta) - (locals.var_chi__blk816 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk816_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk816_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk816_dn17 / locals.var_beta) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0_inia__blk819, locals.var_ps0_inia__blk819_dn0, locals.var_ps0_inia__blk819_dn2, locals.var_ps0_inia__blk819_dn6, locals.var_ps0_inia__blk819_dn7, locals.var_ps0_inia__blk819_dn10, locals.var_ps0_inia__blk819_dn11, locals.var_ps0_inia__blk819_dn12, locals.var_ps0_inia__blk819_dn17,)
    }
};
        locals.var_ps0_inia__blk819 = assign27090_e37611;
        locals.var_ps0_inia__blk819_dn0 = assign27090_e37611_d_n0;
        locals.var_ps0_inia__blk819_dn2 = assign27090_e37611_d_n2;
        locals.var_ps0_inia__blk819_dn6 = assign27090_e37611_d_n6;
        locals.var_ps0_inia__blk819_dn7 = assign27090_e37611_d_n7;
        locals.var_ps0_inia__blk819_dn10 = assign27090_e37611_d_n10;
        locals.var_ps0_inia__blk819_dn11 = assign27090_e37611_d_n11;
        locals.var_ps0_inia__blk819_dn12 = assign27090_e37611_d_n12;
        locals.var_ps0_inia__blk819_dn17 = assign27090_e37611_d_n17;
        locals.var_ps0_inia__blk819_rv = 0.0;

        let (assign27100_e37624, assign27100_e37624_d_n0, assign27100_e37624_d_n2, assign27100_e37624_d_n6, assign27100_e37624_d_n7, assign27100_e37624_d_n10, assign27100_e37624_d_n11, assign27100_e37624_d_n12, assign27100_e37624_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27100_e37621: f64 = (-locals.var_chi__blk816);
        let assign27100_e37622: f64 = (assign27100_e37621).exp();
        (assign27100_e37622, (assign27100_e37622 * (-locals.var_chi__blk816_dn0)), (assign27100_e37622 * (-locals.var_chi__blk816_dn2)), (assign27100_e37622 * (-locals.var_chi__blk816_dn6)), (assign27100_e37622 * (-locals.var_chi__blk816_dn7)), (assign27100_e37622 * (-locals.var_chi__blk816_dn10)), (assign27100_e37622 * (-locals.var_chi__blk816_dn11)), (assign27100_e37622 * (-locals.var_chi__blk816_dn12)), (assign27100_e37622 * (-locals.var_chi__blk816_dn17)),)
    } else {
        (locals.var_ty__blk780, locals.var_ty__blk780_dn0, locals.var_ty__blk780_dn2, locals.var_ty__blk780_dn6, locals.var_ty__blk780_dn7, locals.var_ty__blk780_dn10, locals.var_ty__blk780_dn11, locals.var_ty__blk780_dn12, locals.var_ty__blk780_dn17,)
    }
};
        locals.var_ty__blk780 = assign27100_e37624;
        locals.var_ty__blk780_dn0 = assign27100_e37624_d_n0;
        locals.var_ty__blk780_dn2 = assign27100_e37624_d_n2;
        locals.var_ty__blk780_dn6 = assign27100_e37624_d_n6;
        locals.var_ty__blk780_dn7 = assign27100_e37624_d_n7;
        locals.var_ty__blk780_dn10 = assign27100_e37624_d_n10;
        locals.var_ty__blk780_dn11 = assign27100_e37624_d_n11;
        locals.var_ty__blk780_dn12 = assign27100_e37624_d_n12;
        locals.var_ty__blk780_dn17 = assign27100_e37624_d_n17;
        locals.var_ty__blk780_rv = 0.0;

        let (assign27110_e37651, assign27110_e37651_d_n0, assign27110_e37651_d_n2, assign27110_e37651_d_n6, assign27110_e37651_d_n7, assign27110_e37651_d_n10, assign27110_e37651_d_n11, assign27110_e37651_d_n12, assign27110_e37651_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27110_e37638: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign27110_e37639: f64 = (locals.var_beta * assign27110_e37638);
        let assign27110_e37641: f64 = (assign27110_e37639 - 1.0);
        let assign27110_e37643: f64 = (assign27110_e37641 + locals.var_ty__blk780);
        let assign27110_e37644: f64 = (4.0 * assign27110_e37643);
        let assign27110_e37647: f64 = (locals.var_fac1p2__blk803 * locals.var_beta2);
        let assign27110_e37648: f64 = (assign27110_e37644 / assign27110_e37647);
        let assign27110_e37649: f64 = (1.0 + assign27110_e37648);
        (assign27110_e37649, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + locals.var_ty__blk780_dn0)) * assign27110_e37647) - (assign27110_e37644 * (locals.var_fac1p2__blk803_dn0 * locals.var_beta2))) / (assign27110_e37647 * assign27110_e37647)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + locals.var_ty__blk780_dn2)) * assign27110_e37647) - (assign27110_e37644 * (locals.var_fac1p2__blk803_dn2 * locals.var_beta2))) / (assign27110_e37647 * assign27110_e37647)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) + locals.var_ty__blk780_dn6)) * assign27110_e37647) - (assign27110_e37644 * (locals.var_fac1p2__blk803_dn6 * locals.var_beta2))) / (assign27110_e37647 * assign27110_e37647)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) + locals.var_ty__blk780_dn7)) * assign27110_e37647) - (assign27110_e37644 * (locals.var_fac1p2__blk803_dn7 * locals.var_beta2))) / (assign27110_e37647 * assign27110_e37647)), ((((4.0 * (((locals.var_beta_dn10 * assign27110_e37638) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))) + locals.var_ty__blk780_dn10)) * assign27110_e37647) - (assign27110_e37644 * ((locals.var_fac1p2__blk803_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk803 * locals.var_beta2_dn10)))) / (assign27110_e37647 * assign27110_e37647)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) + locals.var_ty__blk780_dn11)) * assign27110_e37647) - (assign27110_e37644 * (locals.var_fac1p2__blk803_dn11 * locals.var_beta2))) / (assign27110_e37647 * assign27110_e37647)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) + locals.var_ty__blk780_dn12)) * assign27110_e37647) - (assign27110_e37644 * (locals.var_fac1p2__blk803_dn12 * locals.var_beta2))) / (assign27110_e37647 * assign27110_e37647)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) + locals.var_ty__blk780_dn17)) * assign27110_e37647) - (assign27110_e37644 * (locals.var_fac1p2__blk803_dn17 * locals.var_beta2))) / (assign27110_e37647 * assign27110_e37647)),)
    } else {
        (locals.var_tx__blk779, locals.var_tx__blk779_dn0, locals.var_tx__blk779_dn2, locals.var_tx__blk779_dn6, locals.var_tx__blk779_dn7, locals.var_tx__blk779_dn10, locals.var_tx__blk779_dn11, locals.var_tx__blk779_dn12, locals.var_tx__blk779_dn17,)
    }
};
        locals.var_tx__blk779 = assign27110_e37651;
        locals.var_tx__blk779_dn0 = assign27110_e37651_d_n0;
        locals.var_tx__blk779_dn2 = assign27110_e37651_d_n2;
        locals.var_tx__blk779_dn6 = assign27110_e37651_d_n6;
        locals.var_tx__blk779_dn7 = assign27110_e37651_d_n7;
        locals.var_tx__blk779_dn10 = assign27110_e37651_d_n10;
        locals.var_tx__blk779_dn11 = assign27110_e37651_d_n11;
        locals.var_tx__blk779_dn12 = assign27110_e37651_d_n12;
        locals.var_tx__blk779_dn17 = assign27110_e37651_d_n17;
        locals.var_tx__blk779_rv = 0.0;

        let assign27120_e37655: f64 = (10.0 * 2.220446049250313e-16);
        let assign27120_e37656: f64 = if locals.var_tx__blk779 < assign27120_e37655 { 1.0 } else { 0.0 };
        locals.var_guard875 = assign27120_e37656;
        locals.var_guard875_rv = 0.0;

        let (assign27130_e37671, assign27130_e37671_d_n0, assign27130_e37671_d_n2, assign27130_e37671_d_n6, assign27130_e37671_d_n7, assign27130_e37671_d_n10, assign27130_e37671_d_n11, assign27130_e37671_d_n12, assign27130_e37671_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27130_e37669: f64 = (10.0 * 2.220446049250313e-16);
        (assign27130_e37669, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk779, locals.var_tx__blk779_dn0, locals.var_tx__blk779_dn2, locals.var_tx__blk779_dn6, locals.var_tx__blk779_dn7, locals.var_tx__blk779_dn10, locals.var_tx__blk779_dn11, locals.var_tx__blk779_dn12, locals.var_tx__blk779_dn17,)
    }
};
        locals.var_tx__blk779 = assign27130_e37671;
        locals.var_tx__blk779_dn0 = assign27130_e37671_d_n0;
        locals.var_tx__blk779_dn2 = assign27130_e37671_d_n2;
        locals.var_tx__blk779_dn6 = assign27130_e37671_d_n6;
        locals.var_tx__blk779_dn7 = assign27130_e37671_d_n7;
        locals.var_tx__blk779_dn10 = assign27130_e37671_d_n10;
        locals.var_tx__blk779_dn11 = assign27130_e37671_d_n11;
        locals.var_tx__blk779_dn12 = assign27130_e37671_d_n12;
        locals.var_tx__blk779_dn17 = assign27130_e37671_d_n17;
        locals.var_tx__blk779_rv = 0.0;

        let (assign27140_e37693, assign27140_e37693_d_n0, assign27140_e37693_d_n2, assign27140_e37693_d_n6, assign27140_e37693_d_n7, assign27140_e37693_d_n10, assign27140_e37693_d_n11, assign27140_e37693_d_n12, assign27140_e37693_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27140_e37683: f64 = (locals.var_fac1p2__blk803 * locals.var_beta);
        let assign27140_e37685: f64 = (assign27140_e37683 / 2.0);
        let assign27140_e37688: f64 = (locals.var_tx__blk779).sqrt();
        let assign27140_e37689: f64 = (1.0 - assign27140_e37688);
        let assign27140_e37690: f64 = (assign27140_e37685 * assign27140_e37689);
        let assign27140_e37691: f64 = (locals.var_vgpld + assign27140_e37690);
        (assign27140_e37691, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2__blk803_dn0 * locals.var_beta) / 2.0) * assign27140_e37689) + (assign27140_e37685 * (-(locals.var_tx__blk779_dn0 / (2.0 * assign27140_e37688)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2__blk803_dn2 * locals.var_beta) / 2.0) * assign27140_e37689) + (assign27140_e37685 * (-(locals.var_tx__blk779_dn2 / (2.0 * assign27140_e37688)))))), (locals.var_vgpld_dn6 + ((((locals.var_fac1p2__blk803_dn6 * locals.var_beta) / 2.0) * assign27140_e37689) + (assign27140_e37685 * (-(locals.var_tx__blk779_dn6 / (2.0 * assign27140_e37688)))))), (locals.var_vgpld_dn7 + ((((locals.var_fac1p2__blk803_dn7 * locals.var_beta) / 2.0) * assign27140_e37689) + (assign27140_e37685 * (-(locals.var_tx__blk779_dn7 / (2.0 * assign27140_e37688)))))), (locals.var_vgpld_dn10 + (((((locals.var_fac1p2__blk803_dn10 * locals.var_beta) + (locals.var_fac1p2__blk803 * locals.var_beta_dn10)) / 2.0) * assign27140_e37689) + (assign27140_e37685 * (-(locals.var_tx__blk779_dn10 / (2.0 * assign27140_e37688)))))), (locals.var_vgpld_dn11 + ((((locals.var_fac1p2__blk803_dn11 * locals.var_beta) / 2.0) * assign27140_e37689) + (assign27140_e37685 * (-(locals.var_tx__blk779_dn11 / (2.0 * assign27140_e37688)))))), (locals.var_vgpld_dn12 + ((((locals.var_fac1p2__blk803_dn12 * locals.var_beta) / 2.0) * assign27140_e37689) + (assign27140_e37685 * (-(locals.var_tx__blk779_dn12 / (2.0 * assign27140_e37688)))))), (locals.var_vgpld_dn17 + ((((locals.var_fac1p2__blk803_dn17 * locals.var_beta) / 2.0) * assign27140_e37689) + (assign27140_e37685 * (-(locals.var_tx__blk779_dn17 / (2.0 * assign27140_e37688)))))),)
    } else {
        (locals.var_ps0_inia__blk819, locals.var_ps0_inia__blk819_dn0, locals.var_ps0_inia__blk819_dn2, locals.var_ps0_inia__blk819_dn6, locals.var_ps0_inia__blk819_dn7, locals.var_ps0_inia__blk819_dn10, locals.var_ps0_inia__blk819_dn11, locals.var_ps0_inia__blk819_dn12, locals.var_ps0_inia__blk819_dn17,)
    }
};
        locals.var_ps0_inia__blk819 = assign27140_e37693;
        locals.var_ps0_inia__blk819_dn0 = assign27140_e37693_d_n0;
        locals.var_ps0_inia__blk819_dn2 = assign27140_e37693_d_n2;
        locals.var_ps0_inia__blk819_dn6 = assign27140_e37693_d_n6;
        locals.var_ps0_inia__blk819_dn7 = assign27140_e37693_d_n7;
        locals.var_ps0_inia__blk819_dn10 = assign27140_e37693_d_n10;
        locals.var_ps0_inia__blk819_dn11 = assign27140_e37693_d_n11;
        locals.var_ps0_inia__blk819_dn12 = assign27140_e37693_d_n12;
        locals.var_ps0_inia__blk819_dn17 = assign27140_e37693_d_n17;
        locals.var_ps0_inia__blk819_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_97(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27150_e37708, assign27150_e37708_d_n0, assign27150_e37708_d_n2, assign27150_e37708_d_n6, assign27150_e37708_d_n7, assign27150_e37708_d_n10, assign27150_e37708_d_n11, assign27150_e37708_d_n12, assign27150_e37708_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27150_e37705: f64 = (locals.var_ps0_inia__blk819 + locals.var_vxbgmtcl);
        let assign27150_e37706: f64 = (locals.var_beta * assign27150_e37705);
        (assign27150_e37706, (locals.var_beta * (locals.var_ps0_inia__blk819_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign27150_e37705) + (locals.var_beta * (locals.var_ps0_inia__blk819_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk819_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk816, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    }
};
        locals.var_chi__blk816 = assign27150_e37708;
        locals.var_chi__blk816_dn0 = assign27150_e37708_d_n0;
        locals.var_chi__blk816_dn2 = assign27150_e37708_d_n2;
        locals.var_chi__blk816_dn6 = assign27150_e37708_d_n6;
        locals.var_chi__blk816_dn7 = assign27150_e37708_d_n7;
        locals.var_chi__blk816_dn10 = assign27150_e37708_d_n10;
        locals.var_chi__blk816_dn11 = assign27150_e37708_d_n11;
        locals.var_chi__blk816_dn12 = assign27150_e37708_d_n12;
        locals.var_chi__blk816_dn17 = assign27150_e37708_d_n17;
        locals.var_chi__blk816_rv = 0.0;

        let (assign27160_e37721, assign27160_e37721_d_n0, assign27160_e37721_d_n2, assign27160_e37721_d_n6, assign27160_e37721_d_n7, assign27160_e37721_d_n10, assign27160_e37721_d_n11, assign27160_e37721_d_n12, assign27160_e37721_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27160_e37718: f64 = (-locals.var_chi__blk816);
        let assign27160_e37719: f64 = (assign27160_e37718).exp();
        (assign27160_e37719, (assign27160_e37719 * (-locals.var_chi__blk816_dn0)), (assign27160_e37719 * (-locals.var_chi__blk816_dn2)), (assign27160_e37719 * (-locals.var_chi__blk816_dn6)), (assign27160_e37719 * (-locals.var_chi__blk816_dn7)), (assign27160_e37719 * (-locals.var_chi__blk816_dn10)), (assign27160_e37719 * (-locals.var_chi__blk816_dn11)), (assign27160_e37719 * (-locals.var_chi__blk816_dn12)), (assign27160_e37719 * (-locals.var_chi__blk816_dn17)),)
    } else {
        (locals.var_ty__blk780, locals.var_ty__blk780_dn0, locals.var_ty__blk780_dn2, locals.var_ty__blk780_dn6, locals.var_ty__blk780_dn7, locals.var_ty__blk780_dn10, locals.var_ty__blk780_dn11, locals.var_ty__blk780_dn12, locals.var_ty__blk780_dn17,)
    }
};
        locals.var_ty__blk780 = assign27160_e37721;
        locals.var_ty__blk780_dn0 = assign27160_e37721_d_n0;
        locals.var_ty__blk780_dn2 = assign27160_e37721_d_n2;
        locals.var_ty__blk780_dn6 = assign27160_e37721_d_n6;
        locals.var_ty__blk780_dn7 = assign27160_e37721_d_n7;
        locals.var_ty__blk780_dn10 = assign27160_e37721_d_n10;
        locals.var_ty__blk780_dn11 = assign27160_e37721_d_n11;
        locals.var_ty__blk780_dn12 = assign27160_e37721_d_n12;
        locals.var_ty__blk780_dn17 = assign27160_e37721_d_n17;
        locals.var_ty__blk780_rv = 0.0;

        let (assign27170_e37748, assign27170_e37748_d_n0, assign27170_e37748_d_n2, assign27170_e37748_d_n6, assign27170_e37748_d_n7, assign27170_e37748_d_n10, assign27170_e37748_d_n11, assign27170_e37748_d_n12, assign27170_e37748_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27170_e37735: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign27170_e37736: f64 = (locals.var_beta * assign27170_e37735);
        let assign27170_e37738: f64 = (assign27170_e37736 - 1.0);
        let assign27170_e37740: f64 = (assign27170_e37738 + locals.var_ty__blk780);
        let assign27170_e37741: f64 = (4.0 * assign27170_e37740);
        let assign27170_e37744: f64 = (locals.var_fac1p2__blk803 * locals.var_beta2);
        let assign27170_e37745: f64 = (assign27170_e37741 / assign27170_e37744);
        let assign27170_e37746: f64 = (1.0 + assign27170_e37745);
        (assign27170_e37746, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + locals.var_ty__blk780_dn0)) * assign27170_e37744) - (assign27170_e37741 * (locals.var_fac1p2__blk803_dn0 * locals.var_beta2))) / (assign27170_e37744 * assign27170_e37744)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + locals.var_ty__blk780_dn2)) * assign27170_e37744) - (assign27170_e37741 * (locals.var_fac1p2__blk803_dn2 * locals.var_beta2))) / (assign27170_e37744 * assign27170_e37744)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) + locals.var_ty__blk780_dn6)) * assign27170_e37744) - (assign27170_e37741 * (locals.var_fac1p2__blk803_dn6 * locals.var_beta2))) / (assign27170_e37744 * assign27170_e37744)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) + locals.var_ty__blk780_dn7)) * assign27170_e37744) - (assign27170_e37741 * (locals.var_fac1p2__blk803_dn7 * locals.var_beta2))) / (assign27170_e37744 * assign27170_e37744)), ((((4.0 * (((locals.var_beta_dn10 * assign27170_e37735) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))) + locals.var_ty__blk780_dn10)) * assign27170_e37744) - (assign27170_e37741 * ((locals.var_fac1p2__blk803_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk803 * locals.var_beta2_dn10)))) / (assign27170_e37744 * assign27170_e37744)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) + locals.var_ty__blk780_dn11)) * assign27170_e37744) - (assign27170_e37741 * (locals.var_fac1p2__blk803_dn11 * locals.var_beta2))) / (assign27170_e37744 * assign27170_e37744)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) + locals.var_ty__blk780_dn12)) * assign27170_e37744) - (assign27170_e37741 * (locals.var_fac1p2__blk803_dn12 * locals.var_beta2))) / (assign27170_e37744 * assign27170_e37744)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) + locals.var_ty__blk780_dn17)) * assign27170_e37744) - (assign27170_e37741 * (locals.var_fac1p2__blk803_dn17 * locals.var_beta2))) / (assign27170_e37744 * assign27170_e37744)),)
    } else {
        (locals.var_tx__blk779, locals.var_tx__blk779_dn0, locals.var_tx__blk779_dn2, locals.var_tx__blk779_dn6, locals.var_tx__blk779_dn7, locals.var_tx__blk779_dn10, locals.var_tx__blk779_dn11, locals.var_tx__blk779_dn12, locals.var_tx__blk779_dn17,)
    }
};
        locals.var_tx__blk779 = assign27170_e37748;
        locals.var_tx__blk779_dn0 = assign27170_e37748_d_n0;
        locals.var_tx__blk779_dn2 = assign27170_e37748_d_n2;
        locals.var_tx__blk779_dn6 = assign27170_e37748_d_n6;
        locals.var_tx__blk779_dn7 = assign27170_e37748_d_n7;
        locals.var_tx__blk779_dn10 = assign27170_e37748_d_n10;
        locals.var_tx__blk779_dn11 = assign27170_e37748_d_n11;
        locals.var_tx__blk779_dn12 = assign27170_e37748_d_n12;
        locals.var_tx__blk779_dn17 = assign27170_e37748_d_n17;
        locals.var_tx__blk779_rv = 0.0;

        let assign27180_e37752: f64 = (10.0 * 2.220446049250313e-16);
        let assign27180_e37753: f64 = if locals.var_tx__blk779 < assign27180_e37752 { 1.0 } else { 0.0 };
        locals.var_guard876 = assign27180_e37753;
        locals.var_guard876_rv = 0.0;

        let (assign27190_e37768, assign27190_e37768_d_n0, assign27190_e37768_d_n2, assign27190_e37768_d_n6, assign27190_e37768_d_n7, assign27190_e37768_d_n10, assign27190_e37768_d_n11, assign27190_e37768_d_n12, assign27190_e37768_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard876 != 0.0)) {
        let assign27190_e37766: f64 = (10.0 * 2.220446049250313e-16);
        (assign27190_e37766, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk779, locals.var_tx__blk779_dn0, locals.var_tx__blk779_dn2, locals.var_tx__blk779_dn6, locals.var_tx__blk779_dn7, locals.var_tx__blk779_dn10, locals.var_tx__blk779_dn11, locals.var_tx__blk779_dn12, locals.var_tx__blk779_dn17,)
    }
};
        locals.var_tx__blk779 = assign27190_e37768;
        locals.var_tx__blk779_dn0 = assign27190_e37768_d_n0;
        locals.var_tx__blk779_dn2 = assign27190_e37768_d_n2;
        locals.var_tx__blk779_dn6 = assign27190_e37768_d_n6;
        locals.var_tx__blk779_dn7 = assign27190_e37768_d_n7;
        locals.var_tx__blk779_dn10 = assign27190_e37768_d_n10;
        locals.var_tx__blk779_dn11 = assign27190_e37768_d_n11;
        locals.var_tx__blk779_dn12 = assign27190_e37768_d_n12;
        locals.var_tx__blk779_dn17 = assign27190_e37768_d_n17;
        locals.var_tx__blk779_rv = 0.0;

        let (assign27200_e37790, assign27200_e37790_d_n0, assign27200_e37790_d_n2, assign27200_e37790_d_n6, assign27200_e37790_d_n7, assign27200_e37790_d_n10, assign27200_e37790_d_n11, assign27200_e37790_d_n12, assign27200_e37790_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27200_e37780: f64 = (locals.var_fac1p2__blk803 * locals.var_beta);
        let assign27200_e37782: f64 = (assign27200_e37780 / 2.0);
        let assign27200_e37785: f64 = (locals.var_tx__blk779).sqrt();
        let assign27200_e37786: f64 = (1.0 - assign27200_e37785);
        let assign27200_e37787: f64 = (assign27200_e37782 * assign27200_e37786);
        let assign27200_e37788: f64 = (locals.var_vgpld + assign27200_e37787);
        (assign27200_e37788, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2__blk803_dn0 * locals.var_beta) / 2.0) * assign27200_e37786) + (assign27200_e37782 * (-(locals.var_tx__blk779_dn0 / (2.0 * assign27200_e37785)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2__blk803_dn2 * locals.var_beta) / 2.0) * assign27200_e37786) + (assign27200_e37782 * (-(locals.var_tx__blk779_dn2 / (2.0 * assign27200_e37785)))))), (locals.var_vgpld_dn6 + ((((locals.var_fac1p2__blk803_dn6 * locals.var_beta) / 2.0) * assign27200_e37786) + (assign27200_e37782 * (-(locals.var_tx__blk779_dn6 / (2.0 * assign27200_e37785)))))), (locals.var_vgpld_dn7 + ((((locals.var_fac1p2__blk803_dn7 * locals.var_beta) / 2.0) * assign27200_e37786) + (assign27200_e37782 * (-(locals.var_tx__blk779_dn7 / (2.0 * assign27200_e37785)))))), (locals.var_vgpld_dn10 + (((((locals.var_fac1p2__blk803_dn10 * locals.var_beta) + (locals.var_fac1p2__blk803 * locals.var_beta_dn10)) / 2.0) * assign27200_e37786) + (assign27200_e37782 * (-(locals.var_tx__blk779_dn10 / (2.0 * assign27200_e37785)))))), (locals.var_vgpld_dn11 + ((((locals.var_fac1p2__blk803_dn11 * locals.var_beta) / 2.0) * assign27200_e37786) + (assign27200_e37782 * (-(locals.var_tx__blk779_dn11 / (2.0 * assign27200_e37785)))))), (locals.var_vgpld_dn12 + ((((locals.var_fac1p2__blk803_dn12 * locals.var_beta) / 2.0) * assign27200_e37786) + (assign27200_e37782 * (-(locals.var_tx__blk779_dn12 / (2.0 * assign27200_e37785)))))), (locals.var_vgpld_dn17 + ((((locals.var_fac1p2__blk803_dn17 * locals.var_beta) / 2.0) * assign27200_e37786) + (assign27200_e37782 * (-(locals.var_tx__blk779_dn17 / (2.0 * assign27200_e37785)))))),)
    } else {
        (locals.var_ps0_inia__blk819, locals.var_ps0_inia__blk819_dn0, locals.var_ps0_inia__blk819_dn2, locals.var_ps0_inia__blk819_dn6, locals.var_ps0_inia__blk819_dn7, locals.var_ps0_inia__blk819_dn10, locals.var_ps0_inia__blk819_dn11, locals.var_ps0_inia__blk819_dn12, locals.var_ps0_inia__blk819_dn17,)
    }
};
        locals.var_ps0_inia__blk819 = assign27200_e37790;
        locals.var_ps0_inia__blk819_dn0 = assign27200_e37790_d_n0;
        locals.var_ps0_inia__blk819_dn2 = assign27200_e37790_d_n2;
        locals.var_ps0_inia__blk819_dn6 = assign27200_e37790_d_n6;
        locals.var_ps0_inia__blk819_dn7 = assign27200_e37790_d_n7;
        locals.var_ps0_inia__blk819_dn10 = assign27200_e37790_d_n10;
        locals.var_ps0_inia__blk819_dn11 = assign27200_e37790_d_n11;
        locals.var_ps0_inia__blk819_dn12 = assign27200_e37790_d_n12;
        locals.var_ps0_inia__blk819_dn17 = assign27200_e37790_d_n17;
        locals.var_ps0_inia__blk819_rv = 0.0;

        let (assign27210_e37805, assign27210_e37805_d_n0, assign27210_e37805_d_n2, assign27210_e37805_d_n6, assign27210_e37805_d_n7, assign27210_e37805_d_n10, assign27210_e37805_d_n11, assign27210_e37805_d_n12, assign27210_e37805_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27210_e37802: f64 = (locals.var_ps0_inia__blk819 + locals.var_vxbgmtcl);
        let assign27210_e37803: f64 = (locals.var_beta * assign27210_e37802);
        (assign27210_e37803, (locals.var_beta * (locals.var_ps0_inia__blk819_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign27210_e37802) + (locals.var_beta * (locals.var_ps0_inia__blk819_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk819_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk816, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    }
};
        locals.var_chi__blk816 = assign27210_e37805;
        locals.var_chi__blk816_dn0 = assign27210_e37805_d_n0;
        locals.var_chi__blk816_dn2 = assign27210_e37805_d_n2;
        locals.var_chi__blk816_dn6 = assign27210_e37805_d_n6;
        locals.var_chi__blk816_dn7 = assign27210_e37805_d_n7;
        locals.var_chi__blk816_dn10 = assign27210_e37805_d_n10;
        locals.var_chi__blk816_dn11 = assign27210_e37805_d_n11;
        locals.var_chi__blk816_dn12 = assign27210_e37805_d_n12;
        locals.var_chi__blk816_dn17 = assign27210_e37805_d_n17;
        locals.var_chi__blk816_rv = 0.0;

        let assign27220_e37808: f64 = if locals.var_chi__blk816 < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard877 = assign27220_e37808;
        locals.var_guard877_rv = 0.0;

        let (assign27240_e37851,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27240_e37835: f64 = (9.0 * 1.414213562373095);
        let assign27240_e37836: f64 = (1.0 / assign27240_e37835);
        let assign27240_e37840: f64 = (7.0 * 0.049787068367863944);
        let assign27240_e37841: f64 = (5.0 + assign27240_e37840);
        let assign27240_e37845: f64 = (2.0 + 0.049787068367863944);
        let assign27240_e37846: f64 = (assign27240_e37845).sqrt();
        let assign27240_e37847: f64 = (54.0 * assign27240_e37846);
        let assign27240_e37848: f64 = (assign27240_e37841 / assign27240_e37847);
        let assign27240_e37849: f64 = (assign27240_e37836 - assign27240_e37848);
        (assign27240_e37849,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign27240_e37851;
        locals.var_ta_rv = 0.0;

        let (assign27250_e37877,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27250_e37864: f64 = (1.0 + 0.049787068367863944);
        let assign27250_e37868: f64 = (2.0 + 0.049787068367863944);
        let assign27250_e37869: f64 = (assign27250_e37868).sqrt();
        let assign27250_e37870: f64 = (2.0 * assign27250_e37869);
        let assign27250_e37871: f64 = (assign27250_e37864 / assign27250_e37870);
        let assign27250_e37874: f64 = (1.414213562373095 / 3.0);
        let assign27250_e37875: f64 = (assign27250_e37871 - assign27250_e37874);
        (assign27250_e37875,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign27250_e37877;
        locals.var_tb_rv = 0.0;

        let (assign27260_e37898, assign27260_e37898_d_n0, assign27260_e37898_d_n2, assign27260_e37898_d_n6, assign27260_e37898_d_n7, assign27260_e37898_d_n10, assign27260_e37898_d_n11, assign27260_e37898_d_n12, assign27260_e37898_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27260_e37890: f64 = (1.0 / 1.414213562373095);
        let assign27260_e37894: f64 = (locals.var_beta * locals.var_fac1__blk802);
        let assign27260_e37895: f64 = (1.0 / assign27260_e37894);
        let assign27260_e37896: f64 = (assign27260_e37890 + assign27260_e37895);
        (assign27260_e37896, (-((locals.var_beta * locals.var_fac1__blk802_dn0) / (assign27260_e37894 * assign27260_e37894))), (-((locals.var_beta * locals.var_fac1__blk802_dn2) / (assign27260_e37894 * assign27260_e37894))), (-((locals.var_beta * locals.var_fac1__blk802_dn6) / (assign27260_e37894 * assign27260_e37894))), (-((locals.var_beta * locals.var_fac1__blk802_dn7) / (assign27260_e37894 * assign27260_e37894))), (-(((locals.var_beta_dn10 * locals.var_fac1__blk802) + (locals.var_beta * locals.var_fac1__blk802_dn10)) / (assign27260_e37894 * assign27260_e37894))), (-((locals.var_beta * locals.var_fac1__blk802_dn11) / (assign27260_e37894 * assign27260_e37894))), (-((locals.var_beta * locals.var_fac1__blk802_dn12) / (assign27260_e37894 * assign27260_e37894))), (-((locals.var_beta * locals.var_fac1__blk802_dn17) / (assign27260_e37894 * assign27260_e37894))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn12, locals.var_tc_dn17,)
    }
};
        locals.var_tc = assign27260_e37898;
        locals.var_tc_dn0 = assign27260_e37898_d_n0;
        locals.var_tc_dn2 = assign27260_e37898_d_n2;
        locals.var_tc_dn6 = assign27260_e37898_d_n6;
        locals.var_tc_dn7 = assign27260_e37898_d_n7;
        locals.var_tc_dn10 = assign27260_e37898_d_n10;
        locals.var_tc_dn11 = assign27260_e37898_d_n11;
        locals.var_tc_dn12 = assign27260_e37898_d_n12;
        locals.var_tc_dn17 = assign27260_e37898_d_n17;
        locals.var_tc_rv = 0.0;

        let (assign27270_e37916, assign27270_e37916_d_n0, assign27270_e37916_d_n2, assign27270_e37916_d_n6, assign27270_e37916_d_n7, assign27270_e37916_d_n10, assign27270_e37916_d_n11, assign27270_e37916_d_n12, assign27270_e37916_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27270_e37911: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign27270_e37912: f64 = (-assign27270_e37911);
        let assign27270_e37914: f64 = (assign27270_e37912 / locals.var_fac1__blk802);
        (assign27270_e37914, ((((-(locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) * locals.var_fac1__blk802) - (assign27270_e37912 * locals.var_fac1__blk802_dn0)) / (locals.var_fac1__blk802 * locals.var_fac1__blk802)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1__blk802) - (assign27270_e37912 * locals.var_fac1__blk802_dn2)) / (locals.var_fac1__blk802 * locals.var_fac1__blk802)), ((((-(locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) * locals.var_fac1__blk802) - (assign27270_e37912 * locals.var_fac1__blk802_dn6)) / (locals.var_fac1__blk802 * locals.var_fac1__blk802)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1__blk802) - (assign27270_e37912 * locals.var_fac1__blk802_dn7)) / (locals.var_fac1__blk802 * locals.var_fac1__blk802)), ((((-(locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10)) * locals.var_fac1__blk802) - (assign27270_e37912 * locals.var_fac1__blk802_dn10)) / (locals.var_fac1__blk802 * locals.var_fac1__blk802)), ((((-(locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) * locals.var_fac1__blk802) - (assign27270_e37912 * locals.var_fac1__blk802_dn11)) / (locals.var_fac1__blk802 * locals.var_fac1__blk802)), ((((-(locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) * locals.var_fac1__blk802) - (assign27270_e37912 * locals.var_fac1__blk802_dn12)) / (locals.var_fac1__blk802 * locals.var_fac1__blk802)), ((((-(locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) * locals.var_fac1__blk802) - (assign27270_e37912 * locals.var_fac1__blk802_dn17)) / (locals.var_fac1__blk802 * locals.var_fac1__blk802)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn12, locals.var_td_dn17,)
    }
};
        locals.var_td = assign27270_e37916;
        locals.var_td_dn0 = assign27270_e37916_d_n0;
        locals.var_td_dn2 = assign27270_e37916_d_n2;
        locals.var_td_dn6 = assign27270_e37916_d_n6;
        locals.var_td_dn7 = assign27270_e37916_d_n7;
        locals.var_td_dn10 = assign27270_e37916_d_n10;
        locals.var_td_dn11 = assign27270_e37916_d_n11;
        locals.var_td_dn12 = assign27270_e37916_d_n12;
        locals.var_td_dn17 = assign27270_e37916_d_n17;
        locals.var_td_rv = 0.0;

        let (assign27280_e37957, assign27280_e37957_d_n0, assign27280_e37957_d_n2, assign27280_e37957_d_n6, assign27280_e37957_d_n7, assign27280_e37957_d_n10, assign27280_e37957_d_n11, assign27280_e37957_d_n12, assign27280_e37957_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27280_e37929: f64 = (locals.var_tb * locals.var_tb);
        let assign27280_e37931: f64 = (assign27280_e37929 * locals.var_tb);
        let assign27280_e37934: f64 = (27.0 * locals.var_ta);
        let assign27280_e37936: f64 = (assign27280_e37934 * locals.var_ta);
        let assign27280_e37938: f64 = (assign27280_e37936 * locals.var_ta);
        let assign27280_e37939: f64 = (assign27280_e37931 / assign27280_e37938);
        let assign27280_e37942: f64 = (locals.var_tb * locals.var_tc);
        let assign27280_e37945: f64 = (6.0 * locals.var_ta);
        let assign27280_e37947: f64 = (assign27280_e37945 * locals.var_ta);
        let assign27280_e37948: f64 = (assign27280_e37942 / assign27280_e37947);
        let assign27280_e37949: f64 = (assign27280_e37939 - assign27280_e37948);
        let assign27280_e37953: f64 = (2.0 * locals.var_ta);
        let assign27280_e37954: f64 = (locals.var_td / assign27280_e37953);
        let assign27280_e37955: f64 = (assign27280_e37949 + assign27280_e37954);
        (assign27280_e37955, ((-((locals.var_tb * locals.var_tc_dn0) / assign27280_e37947)) + (locals.var_td_dn0 / assign27280_e37953)), ((-((locals.var_tb * locals.var_tc_dn2) / assign27280_e37947)) + (locals.var_td_dn2 / assign27280_e37953)), ((-((locals.var_tb * locals.var_tc_dn6) / assign27280_e37947)) + (locals.var_td_dn6 / assign27280_e37953)), ((-((locals.var_tb * locals.var_tc_dn7) / assign27280_e37947)) + (locals.var_td_dn7 / assign27280_e37953)), ((-((locals.var_tb * locals.var_tc_dn10) / assign27280_e37947)) + (locals.var_td_dn10 / assign27280_e37953)), ((-((locals.var_tb * locals.var_tc_dn11) / assign27280_e37947)) + (locals.var_td_dn11 / assign27280_e37953)), ((-((locals.var_tb * locals.var_tc_dn12) / assign27280_e37947)) + (locals.var_td_dn12 / assign27280_e37953)), ((-((locals.var_tb * locals.var_tc_dn17) / assign27280_e37947)) + (locals.var_td_dn17 / assign27280_e37953)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn12, locals.var_tq_dn17,)
    }
};
        locals.var_tq = assign27280_e37957;
        locals.var_tq_dn0 = assign27280_e37957_d_n0;
        locals.var_tq_dn2 = assign27280_e37957_d_n2;
        locals.var_tq_dn6 = assign27280_e37957_d_n6;
        locals.var_tq_dn7 = assign27280_e37957_d_n7;
        locals.var_tq_dn10 = assign27280_e37957_d_n10;
        locals.var_tq_dn11 = assign27280_e37957_d_n11;
        locals.var_tq_dn12 = assign27280_e37957_d_n12;
        locals.var_tq_dn17 = assign27280_e37957_d_n17;
        locals.var_tq_rv = 0.0;

        let (assign27290_e37984, assign27290_e37984_d_n0, assign27290_e37984_d_n2, assign27290_e37984_d_n6, assign27290_e37984_d_n7, assign27290_e37984_d_n10, assign27290_e37984_d_n11, assign27290_e37984_d_n12, assign27290_e37984_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27290_e37970: f64 = (3.0 * locals.var_ta);
        let assign27290_e37972: f64 = (assign27290_e37970 * locals.var_tc);
        let assign27290_e37975: f64 = (locals.var_tb * locals.var_tb);
        let assign27290_e37976: f64 = (assign27290_e37972 - assign27290_e37975);
        let assign27290_e37979: f64 = (9.0 * locals.var_ta);
        let assign27290_e37981: f64 = (assign27290_e37979 * locals.var_ta);
        let assign27290_e37982: f64 = (assign27290_e37976 / assign27290_e37981);
        (assign27290_e37982, ((assign27290_e37970 * locals.var_tc_dn0) / assign27290_e37981), ((assign27290_e37970 * locals.var_tc_dn2) / assign27290_e37981), ((assign27290_e37970 * locals.var_tc_dn6) / assign27290_e37981), ((assign27290_e37970 * locals.var_tc_dn7) / assign27290_e37981), ((assign27290_e37970 * locals.var_tc_dn10) / assign27290_e37981), ((assign27290_e37970 * locals.var_tc_dn11) / assign27290_e37981), ((assign27290_e37970 * locals.var_tc_dn12) / assign27290_e37981), ((assign27290_e37970 * locals.var_tc_dn17) / assign27290_e37981),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn12, locals.var_tp_dn17,)
    }
};
        locals.var_tp = assign27290_e37984;
        locals.var_tp_dn0 = assign27290_e37984_d_n0;
        locals.var_tp_dn2 = assign27290_e37984_d_n2;
        locals.var_tp_dn6 = assign27290_e37984_d_n6;
        locals.var_tp_dn7 = assign27290_e37984_d_n7;
        locals.var_tp_dn10 = assign27290_e37984_d_n10;
        locals.var_tp_dn11 = assign27290_e37984_d_n11;
        locals.var_tp_dn12 = assign27290_e37984_d_n12;
        locals.var_tp_dn17 = assign27290_e37984_d_n17;
        locals.var_tp_rv = 0.0;

        let (assign27300_e38006, assign27300_e38006_d_n0, assign27300_e38006_d_n2, assign27300_e38006_d_n6, assign27300_e38006_d_n7, assign27300_e38006_d_n10, assign27300_e38006_d_n11, assign27300_e38006_d_n12, assign27300_e38006_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27300_e37997: f64 = (locals.var_tq * locals.var_tq);
        let assign27300_e38000: f64 = (locals.var_tp * locals.var_tp);
        let assign27300_e38002: f64 = (assign27300_e38000 * locals.var_tp);
        let assign27300_e38003: f64 = (assign27300_e37997 + assign27300_e38002);
        let assign27300_e38004: f64 = (assign27300_e38003).sqrt();
        (assign27300_e38004, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign27300_e38000 * locals.var_tp_dn0))) / (2.0 * assign27300_e38004)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign27300_e38000 * locals.var_tp_dn2))) / (2.0 * assign27300_e38004)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign27300_e38000 * locals.var_tp_dn6))) / (2.0 * assign27300_e38004)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign27300_e38000 * locals.var_tp_dn7))) / (2.0 * assign27300_e38004)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign27300_e38000 * locals.var_tp_dn10))) / (2.0 * assign27300_e38004)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign27300_e38000 * locals.var_tp_dn11))) / (2.0 * assign27300_e38004)), ((((locals.var_tq_dn12 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn12)) + ((((locals.var_tp_dn12 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn12)) * locals.var_tp) + (assign27300_e38000 * locals.var_tp_dn12))) / (2.0 * assign27300_e38004)), ((((locals.var_tq_dn17 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn17)) + ((((locals.var_tp_dn17 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn17)) * locals.var_tp) + (assign27300_e38000 * locals.var_tp_dn17))) / (2.0 * assign27300_e38004)),)
    } else {
        (locals.var_t5__blk776, locals.var_t5__blk776_dn0, locals.var_t5__blk776_dn2, locals.var_t5__blk776_dn6, locals.var_t5__blk776_dn7, locals.var_t5__blk776_dn10, locals.var_t5__blk776_dn11, locals.var_t5__blk776_dn12, locals.var_t5__blk776_dn17,)
    }
};
        locals.var_t5__blk776 = assign27300_e38006;
        locals.var_t5__blk776_dn0 = assign27300_e38006_d_n0;
        locals.var_t5__blk776_dn2 = assign27300_e38006_d_n2;
        locals.var_t5__blk776_dn6 = assign27300_e38006_d_n6;
        locals.var_t5__blk776_dn7 = assign27300_e38006_d_n7;
        locals.var_t5__blk776_dn10 = assign27300_e38006_d_n10;
        locals.var_t5__blk776_dn11 = assign27300_e38006_d_n11;
        locals.var_t5__blk776_dn12 = assign27300_e38006_d_n12;
        locals.var_t5__blk776_dn17 = assign27300_e38006_d_n17;
        locals.var_t5__blk776_rv = 0.0;

        let (assign27310_e38024, assign27310_e38024_d_n0, assign27310_e38024_d_n2, assign27310_e38024_d_n6, assign27310_e38024_d_n7, assign27310_e38024_d_n10, assign27310_e38024_d_n11, assign27310_e38024_d_n12, assign27310_e38024_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27310_e38018: f64 = (-locals.var_tq);
        let assign27310_e38020: f64 = (assign27310_e38018 + locals.var_t5__blk776);
        let assign27310_e38022: f64 = (assign27310_e38020).powf(0.3333333333333333);
        (assign27310_e38022, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27310_e38020).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5__blk776_dn0))) } } else { (assign27310_e38022 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5__blk776_dn0) / assign27310_e38020))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27310_e38020).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5__blk776_dn2))) } } else { (assign27310_e38022 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5__blk776_dn2) / assign27310_e38020))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27310_e38020).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5__blk776_dn6))) } } else { (assign27310_e38022 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5__blk776_dn6) / assign27310_e38020))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27310_e38020).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5__blk776_dn7))) } } else { (assign27310_e38022 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5__blk776_dn7) / assign27310_e38020))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27310_e38020).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5__blk776_dn10))) } } else { (assign27310_e38022 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5__blk776_dn10) / assign27310_e38020))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27310_e38020).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5__blk776_dn11))) } } else { (assign27310_e38022 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5__blk776_dn11) / assign27310_e38020))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27310_e38020).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn12) + locals.var_t5__blk776_dn12))) } } else { (assign27310_e38022 * (0.3333333333333333 * (((-locals.var_tq_dn12) + locals.var_t5__blk776_dn12) / assign27310_e38020))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27310_e38020).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn17) + locals.var_t5__blk776_dn17))) } } else { (assign27310_e38022 * (0.3333333333333333 * (((-locals.var_tq_dn17) + locals.var_t5__blk776_dn17) / assign27310_e38020))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn12, locals.var_tu_dn17,)
    }
};
        locals.var_tu = assign27310_e38024;
        locals.var_tu_dn0 = assign27310_e38024_d_n0;
        locals.var_tu_dn2 = assign27310_e38024_d_n2;
        locals.var_tu_dn6 = assign27310_e38024_d_n6;
        locals.var_tu_dn7 = assign27310_e38024_d_n7;
        locals.var_tu_dn10 = assign27310_e38024_d_n10;
        locals.var_tu_dn11 = assign27310_e38024_d_n11;
        locals.var_tu_dn12 = assign27310_e38024_d_n12;
        locals.var_tu_dn17 = assign27310_e38024_d_n17;
        locals.var_tu_rv = 0.0;

        let (assign27320_e38042, assign27320_e38042_d_n0, assign27320_e38042_d_n2, assign27320_e38042_d_n6, assign27320_e38042_d_n7, assign27320_e38042_d_n10, assign27320_e38042_d_n11, assign27320_e38042_d_n12, assign27320_e38042_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27320_e38037: f64 = (locals.var_tq + locals.var_t5__blk776);
        let assign27320_e38039: f64 = (assign27320_e38037).powf(0.3333333333333333);
        let assign27320_e38040: f64 = (-assign27320_e38039);
        (assign27320_e38040, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27320_e38037).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5__blk776_dn0))) } } else { (assign27320_e38039 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5__blk776_dn0) / assign27320_e38037))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27320_e38037).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5__blk776_dn2))) } } else { (assign27320_e38039 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5__blk776_dn2) / assign27320_e38037))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27320_e38037).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5__blk776_dn6))) } } else { (assign27320_e38039 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5__blk776_dn6) / assign27320_e38037))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27320_e38037).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5__blk776_dn7))) } } else { (assign27320_e38039 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5__blk776_dn7) / assign27320_e38037))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27320_e38037).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5__blk776_dn10))) } } else { (assign27320_e38039 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5__blk776_dn10) / assign27320_e38037))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27320_e38037).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5__blk776_dn11))) } } else { (assign27320_e38039 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5__blk776_dn11) / assign27320_e38037))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27320_e38037).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn12 + locals.var_t5__blk776_dn12))) } } else { (assign27320_e38039 * (0.3333333333333333 * ((locals.var_tq_dn12 + locals.var_t5__blk776_dn12) / assign27320_e38037))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27320_e38037).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn17 + locals.var_t5__blk776_dn17))) } } else { (assign27320_e38039 * (0.3333333333333333 * ((locals.var_tq_dn17 + locals.var_t5__blk776_dn17) / assign27320_e38037))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn12, locals.var_tv_dn17,)
    }
};
        locals.var_tv = assign27320_e38042;
        locals.var_tv_dn0 = assign27320_e38042_d_n0;
        locals.var_tv_dn2 = assign27320_e38042_d_n2;
        locals.var_tv_dn6 = assign27320_e38042_d_n6;
        locals.var_tv_dn7 = assign27320_e38042_d_n7;
        locals.var_tv_dn10 = assign27320_e38042_d_n10;
        locals.var_tv_dn11 = assign27320_e38042_d_n11;
        locals.var_tv_dn12 = assign27320_e38042_d_n12;
        locals.var_tv_dn17 = assign27320_e38042_d_n17;
        locals.var_tv_rv = 0.0;

        let (assign27330_e38063, assign27330_e38063_d_n0, assign27330_e38063_d_n2, assign27330_e38063_d_n6, assign27330_e38063_d_n7, assign27330_e38063_d_n10, assign27330_e38063_d_n11, assign27330_e38063_d_n12, assign27330_e38063_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27330_e38055: f64 = (locals.var_tu + locals.var_tv);
        let assign27330_e38059: f64 = (3.0 * locals.var_ta);
        let assign27330_e38060: f64 = (locals.var_tb / assign27330_e38059);
        let assign27330_e38061: f64 = (assign27330_e38055 - assign27330_e38060);
        (assign27330_e38061, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn12 + locals.var_tv_dn12), (locals.var_tu_dn17 + locals.var_tv_dn17),)
    } else {
        (locals.var_tx__blk779, locals.var_tx__blk779_dn0, locals.var_tx__blk779_dn2, locals.var_tx__blk779_dn6, locals.var_tx__blk779_dn7, locals.var_tx__blk779_dn10, locals.var_tx__blk779_dn11, locals.var_tx__blk779_dn12, locals.var_tx__blk779_dn17,)
    }
};
        locals.var_tx__blk779 = assign27330_e38063;
        locals.var_tx__blk779_dn0 = assign27330_e38063_d_n0;
        locals.var_tx__blk779_dn2 = assign27330_e38063_d_n2;
        locals.var_tx__blk779_dn6 = assign27330_e38063_d_n6;
        locals.var_tx__blk779_dn7 = assign27330_e38063_d_n7;
        locals.var_tx__blk779_dn10 = assign27330_e38063_d_n10;
        locals.var_tx__blk779_dn11 = assign27330_e38063_d_n11;
        locals.var_tx__blk779_dn12 = assign27330_e38063_d_n12;
        locals.var_tx__blk779_dn17 = assign27330_e38063_d_n17;
        locals.var_tx__blk779_rv = 0.0;

        let (assign27340_e38080, assign27340_e38080_d_n0, assign27340_e38080_d_n2, assign27340_e38080_d_n6, assign27340_e38080_d_n7, assign27340_e38080_d_n10, assign27340_e38080_d_n11, assign27340_e38080_d_n12, assign27340_e38080_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27340_e38076: f64 = (locals.var_tx__blk779 * locals.var_beta_inv);
        let assign27340_e38078: f64 = (assign27340_e38076 - locals.var_vxbgmtcl);
        (assign27340_e38078, ((locals.var_tx__blk779_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_tx__blk779_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), ((locals.var_tx__blk779_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_tx__blk779_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn7), (((locals.var_tx__blk779_dn10 * locals.var_beta_inv) + (locals.var_tx__blk779 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), ((locals.var_tx__blk779_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_tx__blk779_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12), ((locals.var_tx__blk779_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0_inia__blk819, locals.var_ps0_inia__blk819_dn0, locals.var_ps0_inia__blk819_dn2, locals.var_ps0_inia__blk819_dn6, locals.var_ps0_inia__blk819_dn7, locals.var_ps0_inia__blk819_dn10, locals.var_ps0_inia__blk819_dn11, locals.var_ps0_inia__blk819_dn12, locals.var_ps0_inia__blk819_dn17,)
    }
};
        locals.var_ps0_inia__blk819 = assign27340_e38080;
        locals.var_ps0_inia__blk819_dn0 = assign27340_e38080_d_n0;
        locals.var_ps0_inia__blk819_dn2 = assign27340_e38080_d_n2;
        locals.var_ps0_inia__blk819_dn6 = assign27340_e38080_d_n6;
        locals.var_ps0_inia__blk819_dn7 = assign27340_e38080_d_n7;
        locals.var_ps0_inia__blk819_dn10 = assign27340_e38080_d_n10;
        locals.var_ps0_inia__blk819_dn11 = assign27340_e38080_d_n11;
        locals.var_ps0_inia__blk819_dn12 = assign27340_e38080_d_n12;
        locals.var_ps0_inia__blk819_dn17 = assign27340_e38080_d_n17;
        locals.var_ps0_inia__blk819_rv = 0.0;

        let (assign27350_e38097, assign27350_e38097_d_n0, assign27350_e38097_d_n2, assign27350_e38097_d_n6, assign27350_e38097_d_n7, assign27350_e38097_d_n10, assign27350_e38097_d_n11, assign27350_e38097_d_n12, assign27350_e38097_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27350_e38094: f64 = (locals.var_ps0_inia__blk819 + locals.var_vxbgmtcl);
        let assign27350_e38095: f64 = (locals.var_beta * assign27350_e38094);
        (assign27350_e38095, (locals.var_beta * (locals.var_ps0_inia__blk819_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign27350_e38094) + (locals.var_beta * (locals.var_ps0_inia__blk819_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk819_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk819_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk816, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    }
};
        locals.var_chi__blk816 = assign27350_e38097;
        locals.var_chi__blk816_dn0 = assign27350_e38097_d_n0;
        locals.var_chi__blk816_dn2 = assign27350_e38097_d_n2;
        locals.var_chi__blk816_dn6 = assign27350_e38097_d_n6;
        locals.var_chi__blk816_dn7 = assign27350_e38097_d_n7;
        locals.var_chi__blk816_dn10 = assign27350_e38097_d_n10;
        locals.var_chi__blk816_dn11 = assign27350_e38097_d_n11;
        locals.var_chi__blk816_dn12 = assign27350_e38097_d_n12;
        locals.var_chi__blk816_dn17 = assign27350_e38097_d_n17;
        locals.var_chi__blk816_rv = 0.0;

        let (assign27370_e38125, assign27370_e38125_d_n0, assign27370_e38125_d_n2, assign27370_e38125_d_n6, assign27370_e38125_d_n7, assign27370_e38125_d_n10, assign27370_e38125_d_n11, assign27370_e38125_d_n12, assign27370_e38125_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27370_e38121: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign27370_e38123: f64 = (assign27370_e38121 + 0.1);
        (assign27370_e38123, (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12), (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn12, locals.var_vgpld_shift_dn17,)
    }
};
        locals.var_vgpld_shift = assign27370_e38125;
        locals.var_vgpld_shift_dn0 = assign27370_e38125_d_n0;
        locals.var_vgpld_shift_dn2 = assign27370_e38125_d_n2;
        locals.var_vgpld_shift_dn6 = assign27370_e38125_d_n6;
        locals.var_vgpld_shift_dn7 = assign27370_e38125_d_n7;
        locals.var_vgpld_shift_dn10 = assign27370_e38125_d_n10;
        locals.var_vgpld_shift_dn11 = assign27370_e38125_d_n11;
        locals.var_vgpld_shift_dn12 = assign27370_e38125_d_n12;
        locals.var_vgpld_shift_dn17 = assign27370_e38125_d_n17;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign27380_e38142, assign27380_e38142_d_n0, assign27380_e38142_d_n2, assign27380_e38142_d_n6, assign27380_e38142_d_n7, assign27380_e38142_d_n10, assign27380_e38142_d_n11, assign27380_e38142_d_n12, assign27380_e38142_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27380_e38136: f64 = (-locals.var_vxbgmtcl);
        let assign27380_e38137: f64 = (locals.var_beta * assign27380_e38136);
        let assign27380_e38138: f64 = (assign27380_e38137).exp();
        let assign27380_e38140: f64 = (assign27380_e38138 + 1e-50);
        (assign27380_e38140, (assign27380_e38138 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign27380_e38138 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign27380_e38138 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign27380_e38138 * (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), (assign27380_e38138 * ((locals.var_beta_dn10 * assign27380_e38136) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign27380_e38138 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign27380_e38138 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))), (assign27380_e38138 * (locals.var_beta * (-locals.var_vxbgmtcl_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk835, locals.var_exp_bvbs__blk835_dn0, locals.var_exp_bvbs__blk835_dn2, locals.var_exp_bvbs__blk835_dn6, locals.var_exp_bvbs__blk835_dn7, locals.var_exp_bvbs__blk835_dn10, locals.var_exp_bvbs__blk835_dn11, locals.var_exp_bvbs__blk835_dn12, locals.var_exp_bvbs__blk835_dn17,)
    }
};
        locals.var_exp_bvbs__blk835 = assign27380_e38142;
        locals.var_exp_bvbs__blk835_dn0 = assign27380_e38142_d_n0;
        locals.var_exp_bvbs__blk835_dn2 = assign27380_e38142_d_n2;
        locals.var_exp_bvbs__blk835_dn6 = assign27380_e38142_d_n6;
        locals.var_exp_bvbs__blk835_dn7 = assign27380_e38142_d_n7;
        locals.var_exp_bvbs__blk835_dn10 = assign27380_e38142_d_n10;
        locals.var_exp_bvbs__blk835_dn11 = assign27380_e38142_d_n11;
        locals.var_exp_bvbs__blk835_dn12 = assign27380_e38142_d_n12;
        locals.var_exp_bvbs__blk835_dn17 = assign27380_e38142_d_n17;
        locals.var_exp_bvbs__blk835_rv = 0.0;

        let (assign27390_e38155, assign27390_e38155_d_n0, assign27390_e38155_d_n2, assign27390_e38155_d_n6, assign27390_e38155_d_n7, assign27390_e38155_d_n10, assign27390_e38155_d_n11, assign27390_e38155_d_n12, assign27390_e38155_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27390_e38153: f64 = (locals.var_nin / locals.var_uc_nsubbttub);
        (assign27390_e38153, (((locals.var_nin_dn0 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn0)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn2 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn2)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn6 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn6)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn7 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn7)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn10 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn10)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn11 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn11)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn12 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn12)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn17 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn17)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)),)
    } else {
        (locals.var_t0__blk772, locals.var_t0__blk772_dn0, locals.var_t0__blk772_dn2, locals.var_t0__blk772_dn6, locals.var_t0__blk772_dn7, locals.var_t0__blk772_dn10, locals.var_t0__blk772_dn11, locals.var_t0__blk772_dn12, locals.var_t0__blk772_dn17,)
    }
};
        locals.var_t0__blk772 = assign27390_e38155;
        locals.var_t0__blk772_dn0 = assign27390_e38155_d_n0;
        locals.var_t0__blk772_dn2 = assign27390_e38155_d_n2;
        locals.var_t0__blk772_dn6 = assign27390_e38155_d_n6;
        locals.var_t0__blk772_dn7 = assign27390_e38155_d_n7;
        locals.var_t0__blk772_dn10 = assign27390_e38155_d_n10;
        locals.var_t0__blk772_dn11 = assign27390_e38155_d_n11;
        locals.var_t0__blk772_dn12 = assign27390_e38155_d_n12;
        locals.var_t0__blk772_dn17 = assign27390_e38155_d_n17;
        locals.var_t0__blk772_rv = 0.0;

        let (assign27400_e38168, assign27400_e38168_d_n0, assign27400_e38168_d_n2, assign27400_e38168_d_n6, assign27400_e38168_d_n7, assign27400_e38168_d_n10, assign27400_e38168_d_n11, assign27400_e38168_d_n12, assign27400_e38168_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27400_e38166: f64 = (locals.var_t0__blk772 * locals.var_t0__blk772);
        (assign27400_e38166, ((locals.var_t0__blk772_dn0 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn0)), ((locals.var_t0__blk772_dn2 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn2)), ((locals.var_t0__blk772_dn6 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn6)), ((locals.var_t0__blk772_dn7 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn7)), ((locals.var_t0__blk772_dn10 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn10)), ((locals.var_t0__blk772_dn11 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn11)), ((locals.var_t0__blk772_dn12 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn12)), ((locals.var_t0__blk772_dn17 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn17)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12, locals.var_cnst1over_dn17,)
    }
};
        locals.var_cnst1over = assign27400_e38168;
        locals.var_cnst1over_dn0 = assign27400_e38168_d_n0;
        locals.var_cnst1over_dn2 = assign27400_e38168_d_n2;
        locals.var_cnst1over_dn6 = assign27400_e38168_d_n6;
        locals.var_cnst1over_dn7 = assign27400_e38168_d_n7;
        locals.var_cnst1over_dn10 = assign27400_e38168_d_n10;
        locals.var_cnst1over_dn11 = assign27400_e38168_d_n11;
        locals.var_cnst1over_dn12 = assign27400_e38168_d_n12;
        locals.var_cnst1over_dn17 = assign27400_e38168_d_n17;
        locals.var_cnst1over_rv = 0.0;

        let (assign27410_e38181, assign27410_e38181_d_n0, assign27410_e38181_d_n2, assign27410_e38181_d_n6, assign27410_e38181_d_n7, assign27410_e38181_d_n10, assign27410_e38181_d_n11, assign27410_e38181_d_n12, assign27410_e38181_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27410_e38179: f64 = (locals.var_cnst1over * locals.var_exp_bvbs__blk835);
        (assign27410_e38179, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn2)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn7)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn12)), ((locals.var_cnst1over_dn17 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn17)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn12, locals.var_gammachi_dn17,)
    }
};
        locals.var_gammachi = assign27410_e38181;
        locals.var_gammachi_dn0 = assign27410_e38181_d_n0;
        locals.var_gammachi_dn2 = assign27410_e38181_d_n2;
        locals.var_gammachi_dn6 = assign27410_e38181_d_n6;
        locals.var_gammachi_dn7 = assign27410_e38181_d_n7;
        locals.var_gammachi_dn10 = assign27410_e38181_d_n10;
        locals.var_gammachi_dn11 = assign27410_e38181_d_n11;
        locals.var_gammachi_dn12 = assign27410_e38181_d_n12;
        locals.var_gammachi_dn17 = assign27410_e38181_d_n17;
        locals.var_gammachi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_98(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27420_e38194, assign27420_e38194_d_n0, assign27420_e38194_d_n2, assign27420_e38194_d_n6, assign27420_e38194_d_n7, assign27420_e38194_d_n10, assign27420_e38194_d_n11, assign27420_e38194_d_n12, assign27420_e38194_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27420_e38192: f64 = (locals.var_beta2 * locals.var_fac1p2__blk803);
        (assign27420_e38192, (locals.var_beta2 * locals.var_fac1p2__blk803_dn0), (locals.var_beta2 * locals.var_fac1p2__blk803_dn2), (locals.var_beta2 * locals.var_fac1p2__blk803_dn6), (locals.var_beta2 * locals.var_fac1p2__blk803_dn7), ((locals.var_beta2_dn10 * locals.var_fac1p2__blk803) + (locals.var_beta2 * locals.var_fac1p2__blk803_dn10)), (locals.var_beta2 * locals.var_fac1p2__blk803_dn11), (locals.var_beta2 * locals.var_fac1p2__blk803_dn12), (locals.var_beta2 * locals.var_fac1p2__blk803_dn17),)
    } else {
        (locals.var_t0__blk772, locals.var_t0__blk772_dn0, locals.var_t0__blk772_dn2, locals.var_t0__blk772_dn6, locals.var_t0__blk772_dn7, locals.var_t0__blk772_dn10, locals.var_t0__blk772_dn11, locals.var_t0__blk772_dn12, locals.var_t0__blk772_dn17,)
    }
};
        locals.var_t0__blk772 = assign27420_e38194;
        locals.var_t0__blk772_dn0 = assign27420_e38194_d_n0;
        locals.var_t0__blk772_dn2 = assign27420_e38194_d_n2;
        locals.var_t0__blk772_dn6 = assign27420_e38194_d_n6;
        locals.var_t0__blk772_dn7 = assign27420_e38194_d_n7;
        locals.var_t0__blk772_dn10 = assign27420_e38194_d_n10;
        locals.var_t0__blk772_dn11 = assign27420_e38194_d_n11;
        locals.var_t0__blk772_dn12 = assign27420_e38194_d_n12;
        locals.var_t0__blk772_dn17 = assign27420_e38194_d_n17;
        locals.var_t0__blk772_rv = 0.0;

        let (assign27430_e38207, assign27430_e38207_d_n0, assign27430_e38207_d_n2, assign27430_e38207_d_n6, assign27430_e38207_d_n7, assign27430_e38207_d_n10, assign27430_e38207_d_n11, assign27430_e38207_d_n12, assign27430_e38207_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27430_e38205: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign27430_e38205, (locals.var_beta * locals.var_vgpld_shift_dn0), (locals.var_beta * locals.var_vgpld_shift_dn2), (locals.var_beta * locals.var_vgpld_shift_dn6), (locals.var_beta * locals.var_vgpld_shift_dn7), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), (locals.var_beta * locals.var_vgpld_shift_dn11), (locals.var_beta * locals.var_vgpld_shift_dn12), (locals.var_beta * locals.var_vgpld_shift_dn17),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign27430_e38207;
        locals.var_psi_dn0 = assign27430_e38207_d_n0;
        locals.var_psi_dn2 = assign27430_e38207_d_n2;
        locals.var_psi_dn6 = assign27430_e38207_d_n6;
        locals.var_psi_dn7 = assign27430_e38207_d_n7;
        locals.var_psi_dn10 = assign27430_e38207_d_n10;
        locals.var_psi_dn11 = assign27430_e38207_d_n11;
        locals.var_psi_dn12 = assign27430_e38207_d_n12;
        locals.var_psi_dn17 = assign27430_e38207_d_n17;
        locals.var_psi_rv = 0.0;

        let (assign27440_e38234, assign27440_e38234_d_n0, assign27440_e38234_d_n2, assign27440_e38234_d_n6, assign27440_e38234_d_n7, assign27440_e38234_d_n10, assign27440_e38234_d_n11, assign27440_e38234_d_n12, assign27440_e38234_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27440_e38218: f64 = (locals.var_gammachi * locals.var_t0__blk772);
        let assign27440_e38221: f64 = (locals.var_psi * locals.var_psi);
        let assign27440_e38222: f64 = (assign27440_e38218 + assign27440_e38221);
        let assign27440_e38223: f64 = (assign27440_e38222).ln();
        let assign27440_e38226: f64 = (locals.var_cnst1over * locals.var_t0__blk772);
        let assign27440_e38227: f64 = (assign27440_e38226).ln();
        let assign27440_e38228: f64 = (assign27440_e38223 - assign27440_e38227);
        let assign27440_e38231: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign27440_e38232: f64 = (assign27440_e38228 + assign27440_e38231);
        (assign27440_e38232, ((((((locals.var_gammachi_dn0 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign27440_e38222) - (((locals.var_cnst1over_dn0 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn0)) / assign27440_e38226)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign27440_e38222) - (((locals.var_cnst1over_dn2 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn2)) / assign27440_e38226)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn6 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign27440_e38222) - (((locals.var_cnst1over_dn6 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn6)) / assign27440_e38226)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn7 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign27440_e38222) - (((locals.var_cnst1over_dn7 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn7)) / assign27440_e38226)) + (locals.var_beta * locals.var_vxbgmtcl_dn7)), ((((((locals.var_gammachi_dn10 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign27440_e38222) - (((locals.var_cnst1over_dn10 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn10)) / assign27440_e38226)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign27440_e38222) - (((locals.var_cnst1over_dn11 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn11)) / assign27440_e38226)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign27440_e38222) - (((locals.var_cnst1over_dn12 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn12)) / assign27440_e38226)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)), ((((((locals.var_gammachi_dn17 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn17)) + ((locals.var_psi_dn17 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn17))) / assign27440_e38222) - (((locals.var_cnst1over_dn17 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn17)) / assign27440_e38226)) + (locals.var_beta * locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12, locals.var_chi_1_dn17,)
    }
};
        locals.var_chi_1 = assign27440_e38234;
        locals.var_chi_1_dn0 = assign27440_e38234_d_n0;
        locals.var_chi_1_dn2 = assign27440_e38234_d_n2;
        locals.var_chi_1_dn6 = assign27440_e38234_d_n6;
        locals.var_chi_1_dn7 = assign27440_e38234_d_n7;
        locals.var_chi_1_dn10 = assign27440_e38234_d_n10;
        locals.var_chi_1_dn11 = assign27440_e38234_d_n11;
        locals.var_chi_1_dn12 = assign27440_e38234_d_n12;
        locals.var_chi_1_dn17 = assign27440_e38234_d_n17;
        locals.var_chi_1_rv = 0.0;

        let (assign27450_e38249, assign27450_e38249_d_n0, assign27450_e38249_d_n2, assign27450_e38249_d_n6, assign27450_e38249_d_n7, assign27450_e38249_d_n10, assign27450_e38249_d_n11, assign27450_e38249_d_n12, assign27450_e38249_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27450_e38245: f64 = (locals.var_psi - locals.var_chi_1);
        let assign27450_e38247: f64 = (assign27450_e38245 - 1.0);
        (assign27450_e38247, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12), (locals.var_psi_dn17 - locals.var_chi_1_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign27450_e38249;
        locals.var_tmf1_dn0 = assign27450_e38249_d_n0;
        locals.var_tmf1_dn2 = assign27450_e38249_d_n2;
        locals.var_tmf1_dn6 = assign27450_e38249_d_n6;
        locals.var_tmf1_dn7 = assign27450_e38249_d_n7;
        locals.var_tmf1_dn10 = assign27450_e38249_d_n10;
        locals.var_tmf1_dn11 = assign27450_e38249_d_n11;
        locals.var_tmf1_dn12 = assign27450_e38249_d_n12;
        locals.var_tmf1_dn17 = assign27450_e38249_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign27460_e38264, assign27460_e38264_d_n0, assign27460_e38264_d_n2, assign27460_e38264_d_n6, assign27460_e38264_d_n7, assign27460_e38264_d_n10, assign27460_e38264_d_n11, assign27460_e38264_d_n12, assign27460_e38264_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27460_e38260: f64 = (4.0 * locals.var_psi);
        let assign27460_e38262: f64 = assign27460_e38260;
        (assign27460_e38262, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn12), (4.0 * locals.var_psi_dn17),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27460_e38264;
        locals.var_tmf2_dn0 = assign27460_e38264_d_n0;
        locals.var_tmf2_dn2 = assign27460_e38264_d_n2;
        locals.var_tmf2_dn6 = assign27460_e38264_d_n6;
        locals.var_tmf2_dn7 = assign27460_e38264_d_n7;
        locals.var_tmf2_dn10 = assign27460_e38264_d_n10;
        locals.var_tmf2_dn11 = assign27460_e38264_d_n11;
        locals.var_tmf2_dn12 = assign27460_e38264_d_n12;
        locals.var_tmf2_dn17 = assign27460_e38264_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27470_e38281, assign27470_e38281_d_n0, assign27470_e38281_d_n2, assign27470_e38281_d_n6, assign27470_e38281_d_n7, assign27470_e38281_d_n10, assign27470_e38281_d_n11, assign27470_e38281_d_n12, assign27470_e38281_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let (assign27470_e38279, assign27470_e38279_d_n0, assign27470_e38279_d_n2, assign27470_e38279_d_n6, assign27470_e38279_d_n7, assign27470_e38279_d_n10, assign27470_e38279_d_n11, assign27470_e38279_d_n12, assign27470_e38279_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign27470_e38278: f64 = (-locals.var_tmf2);
                (assign27470_e38278, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign27470_e38279, assign27470_e38279_d_n0, assign27470_e38279_d_n2, assign27470_e38279_d_n6, assign27470_e38279_d_n7, assign27470_e38279_d_n10, assign27470_e38279_d_n11, assign27470_e38279_d_n12, assign27470_e38279_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27470_e38281;
        locals.var_tmf2_dn0 = assign27470_e38281_d_n0;
        locals.var_tmf2_dn2 = assign27470_e38281_d_n2;
        locals.var_tmf2_dn6 = assign27470_e38281_d_n6;
        locals.var_tmf2_dn7 = assign27470_e38281_d_n7;
        locals.var_tmf2_dn10 = assign27470_e38281_d_n10;
        locals.var_tmf2_dn11 = assign27470_e38281_d_n11;
        locals.var_tmf2_dn12 = assign27470_e38281_d_n12;
        locals.var_tmf2_dn17 = assign27470_e38281_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27480_e38297, assign27480_e38297_d_n0, assign27480_e38297_d_n2, assign27480_e38297_d_n6, assign27480_e38297_d_n7, assign27480_e38297_d_n10, assign27480_e38297_d_n11, assign27480_e38297_d_n12, assign27480_e38297_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27480_e38292: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign27480_e38294: f64 = (assign27480_e38292 + locals.var_tmf2);
        let assign27480_e38295: f64 = (assign27480_e38294).sqrt();
        (assign27480_e38295, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign27480_e38295)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign27480_e38295)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign27480_e38295)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign27480_e38295)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign27480_e38295)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign27480_e38295)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign27480_e38295)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign27480_e38295)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27480_e38297;
        locals.var_tmf2_dn0 = assign27480_e38297_d_n0;
        locals.var_tmf2_dn2 = assign27480_e38297_d_n2;
        locals.var_tmf2_dn6 = assign27480_e38297_d_n6;
        locals.var_tmf2_dn7 = assign27480_e38297_d_n7;
        locals.var_tmf2_dn10 = assign27480_e38297_d_n10;
        locals.var_tmf2_dn11 = assign27480_e38297_d_n11;
        locals.var_tmf2_dn12 = assign27480_e38297_d_n12;
        locals.var_tmf2_dn17 = assign27480_e38297_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27490_e38314, assign27490_e38314_d_n0, assign27490_e38314_d_n2, assign27490_e38314_d_n6, assign27490_e38314_d_n7, assign27490_e38314_d_n10, assign27490_e38314_d_n11, assign27490_e38314_d_n12, assign27490_e38314_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27490_e38310: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign27490_e38311: f64 = (1.0 + assign27490_e38310);
        let assign27490_e38312: f64 = (0.5 * assign27490_e38311);
        (assign27490_e38312, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk773, locals.var_t1__blk773_dn0, locals.var_t1__blk773_dn2, locals.var_t1__blk773_dn6, locals.var_t1__blk773_dn7, locals.var_t1__blk773_dn10, locals.var_t1__blk773_dn11, locals.var_t1__blk773_dn12, locals.var_t1__blk773_dn17,)
    }
};
        locals.var_t1__blk773 = assign27490_e38314;
        locals.var_t1__blk773_dn0 = assign27490_e38314_d_n0;
        locals.var_t1__blk773_dn2 = assign27490_e38314_d_n2;
        locals.var_t1__blk773_dn6 = assign27490_e38314_d_n6;
        locals.var_t1__blk773_dn7 = assign27490_e38314_d_n7;
        locals.var_t1__blk773_dn10 = assign27490_e38314_d_n10;
        locals.var_t1__blk773_dn11 = assign27490_e38314_d_n11;
        locals.var_t1__blk773_dn12 = assign27490_e38314_d_n12;
        locals.var_t1__blk773_dn17 = assign27490_e38314_d_n17;
        locals.var_t1__blk773_rv = 0.0;

        let (assign27500_e38335, assign27500_e38335_d_n0, assign27500_e38335_d_n2, assign27500_e38335_d_n6, assign27500_e38335_d_n7, assign27500_e38335_d_n10, assign27500_e38335_d_n11, assign27500_e38335_d_n12, assign27500_e38335_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27500_e38328: f64 = 2.0;
        let assign27500_e38329: f64 = (locals.var_tmf1 + assign27500_e38328);
        let assign27500_e38331: f64 = (assign27500_e38329 / locals.var_tmf2);
        let assign27500_e38332: f64 = (1.0 - assign27500_e38331);
        let assign27500_e38333: f64 = (0.5 * assign27500_e38332);
        (assign27500_e38333, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign27500_e38329 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign27500_e38329 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign27500_e38329 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign27500_e38329 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign27500_e38329 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign27500_e38329 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign27500_e38329 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign27500_e38329 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk774, locals.var_t2__blk774_dn0, locals.var_t2__blk774_dn2, locals.var_t2__blk774_dn6, locals.var_t2__blk774_dn7, locals.var_t2__blk774_dn10, locals.var_t2__blk774_dn11, locals.var_t2__blk774_dn12, locals.var_t2__blk774_dn17,)
    }
};
        locals.var_t2__blk774 = assign27500_e38335;
        locals.var_t2__blk774_dn0 = assign27500_e38335_d_n0;
        locals.var_t2__blk774_dn2 = assign27500_e38335_d_n2;
        locals.var_t2__blk774_dn6 = assign27500_e38335_d_n6;
        locals.var_t2__blk774_dn7 = assign27500_e38335_d_n7;
        locals.var_t2__blk774_dn10 = assign27500_e38335_d_n10;
        locals.var_t2__blk774_dn11 = assign27500_e38335_d_n11;
        locals.var_t2__blk774_dn12 = assign27500_e38335_d_n12;
        locals.var_t2__blk774_dn17 = assign27500_e38335_d_n17;
        locals.var_t2__blk774_rv = 0.0;

        let (assign27510_e38352, assign27510_e38352_d_n0, assign27510_e38352_d_n2, assign27510_e38352_d_n6, assign27510_e38352_d_n7, assign27510_e38352_d_n10, assign27510_e38352_d_n11, assign27510_e38352_d_n12, assign27510_e38352_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27510_e38348: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign27510_e38349: f64 = (0.5 * assign27510_e38348);
        let assign27510_e38350: f64 = (locals.var_psi - assign27510_e38349);
        (assign27510_e38350, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psi_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12, locals.var_chi_1_dn17,)
    }
};
        locals.var_chi_1 = assign27510_e38352;
        locals.var_chi_1_dn0 = assign27510_e38352_d_n0;
        locals.var_chi_1_dn2 = assign27510_e38352_d_n2;
        locals.var_chi_1_dn6 = assign27510_e38352_d_n6;
        locals.var_chi_1_dn7 = assign27510_e38352_d_n7;
        locals.var_chi_1_dn10 = assign27510_e38352_d_n10;
        locals.var_chi_1_dn11 = assign27510_e38352_d_n11;
        locals.var_chi_1_dn12 = assign27510_e38352_d_n12;
        locals.var_chi_1_dn17 = assign27510_e38352_d_n17;
        locals.var_chi_1_rv = 0.0;

        let (assign27520_e38365, assign27520_e38365_d_n0, assign27520_e38365_d_n2, assign27520_e38365_d_n6, assign27520_e38365_d_n7, assign27520_e38365_d_n10, assign27520_e38365_d_n11, assign27520_e38365_d_n12, assign27520_e38365_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27520_e38363: f64 = (locals.var_psi - locals.var_chi_1);
        (assign27520_e38363, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12), (locals.var_psi_dn17 - locals.var_chi_1_dn17),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign27520_e38365;
        locals.var_psi_dn0 = assign27520_e38365_d_n0;
        locals.var_psi_dn2 = assign27520_e38365_d_n2;
        locals.var_psi_dn6 = assign27520_e38365_d_n6;
        locals.var_psi_dn7 = assign27520_e38365_d_n7;
        locals.var_psi_dn10 = assign27520_e38365_d_n10;
        locals.var_psi_dn11 = assign27520_e38365_d_n11;
        locals.var_psi_dn12 = assign27520_e38365_d_n12;
        locals.var_psi_dn17 = assign27520_e38365_d_n17;
        locals.var_psi_rv = 0.0;

        let (assign27530_e38380, assign27530_e38380_d_n0, assign27530_e38380_d_n2, assign27530_e38380_d_n6, assign27530_e38380_d_n7, assign27530_e38380_d_n10, assign27530_e38380_d_n11, assign27530_e38380_d_n12, assign27530_e38380_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27530_e38377: f64 = (locals.var_beta * 0.1);
        let assign27530_e38378: f64 = (locals.var_psi + assign27530_e38377);
        (assign27530_e38378, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign27530_e38380;
        locals.var_psi_dn0 = assign27530_e38380_d_n0;
        locals.var_psi_dn2 = assign27530_e38380_d_n2;
        locals.var_psi_dn6 = assign27530_e38380_d_n6;
        locals.var_psi_dn7 = assign27530_e38380_d_n7;
        locals.var_psi_dn10 = assign27530_e38380_d_n10;
        locals.var_psi_dn11 = assign27530_e38380_d_n11;
        locals.var_psi_dn12 = assign27530_e38380_d_n12;
        locals.var_psi_dn17 = assign27530_e38380_d_n17;
        locals.var_psi_rv = 0.0;

        let (assign27540_e38407, assign27540_e38407_d_n0, assign27540_e38407_d_n2, assign27540_e38407_d_n6, assign27540_e38407_d_n7, assign27540_e38407_d_n10, assign27540_e38407_d_n11, assign27540_e38407_d_n12, assign27540_e38407_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27540_e38391: f64 = (locals.var_gammachi * locals.var_t0__blk772);
        let assign27540_e38394: f64 = (locals.var_psi * locals.var_psi);
        let assign27540_e38395: f64 = (assign27540_e38391 + assign27540_e38394);
        let assign27540_e38396: f64 = (assign27540_e38395).ln();
        let assign27540_e38399: f64 = (locals.var_cnst1over * locals.var_t0__blk772);
        let assign27540_e38400: f64 = (assign27540_e38399).ln();
        let assign27540_e38401: f64 = (assign27540_e38396 - assign27540_e38400);
        let assign27540_e38404: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign27540_e38405: f64 = (assign27540_e38401 + assign27540_e38404);
        (assign27540_e38405, ((((((locals.var_gammachi_dn0 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign27540_e38395) - (((locals.var_cnst1over_dn0 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn0)) / assign27540_e38399)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign27540_e38395) - (((locals.var_cnst1over_dn2 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn2)) / assign27540_e38399)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn6 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign27540_e38395) - (((locals.var_cnst1over_dn6 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn6)) / assign27540_e38399)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn7 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign27540_e38395) - (((locals.var_cnst1over_dn7 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn7)) / assign27540_e38399)) + (locals.var_beta * locals.var_vxbgmtcl_dn7)), ((((((locals.var_gammachi_dn10 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign27540_e38395) - (((locals.var_cnst1over_dn10 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn10)) / assign27540_e38399)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign27540_e38395) - (((locals.var_cnst1over_dn11 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn11)) / assign27540_e38399)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign27540_e38395) - (((locals.var_cnst1over_dn12 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn12)) / assign27540_e38399)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)), ((((((locals.var_gammachi_dn17 * locals.var_t0__blk772) + (locals.var_gammachi * locals.var_t0__blk772_dn17)) + ((locals.var_psi_dn17 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn17))) / assign27540_e38395) - (((locals.var_cnst1over_dn17 * locals.var_t0__blk772) + (locals.var_cnst1over * locals.var_t0__blk772_dn17)) / assign27540_e38399)) + (locals.var_beta * locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn12, locals.var_chi_b_dn17,)
    }
};
        locals.var_chi_b = assign27540_e38407;
        locals.var_chi_b_dn0 = assign27540_e38407_d_n0;
        locals.var_chi_b_dn2 = assign27540_e38407_d_n2;
        locals.var_chi_b_dn6 = assign27540_e38407_d_n6;
        locals.var_chi_b_dn7 = assign27540_e38407_d_n7;
        locals.var_chi_b_dn10 = assign27540_e38407_d_n10;
        locals.var_chi_b_dn11 = assign27540_e38407_d_n11;
        locals.var_chi_b_dn12 = assign27540_e38407_d_n12;
        locals.var_chi_b_dn17 = assign27540_e38407_d_n17;
        locals.var_chi_b_rv = 0.0;

        let (assign27550_e38418, assign27550_e38418_d_n0, assign27550_e38418_d_n2, assign27550_e38418_d_n6, assign27550_e38418_d_n7, assign27550_e38418_d_n10, assign27550_e38418_d_n11, assign27550_e38418_d_n12, assign27550_e38418_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        (locals.var_chi__blk816, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn12, locals.var_chi_a_dn17,)
    }
};
        locals.var_chi_a = assign27550_e38418;
        locals.var_chi_a_dn0 = assign27550_e38418_d_n0;
        locals.var_chi_a_dn2 = assign27550_e38418_d_n2;
        locals.var_chi_a_dn6 = assign27550_e38418_d_n6;
        locals.var_chi_a_dn7 = assign27550_e38418_d_n7;
        locals.var_chi_a_dn10 = assign27550_e38418_d_n10;
        locals.var_chi_a_dn11 = assign27550_e38418_d_n11;
        locals.var_chi_a_dn12 = assign27550_e38418_d_n12;
        locals.var_chi_a_dn17 = assign27550_e38418_d_n17;
        locals.var_chi_a_rv = 0.0;

        let (assign27560_e38435, assign27560_e38435_d_n0, assign27560_e38435_d_n2, assign27560_e38435_d_n6, assign27560_e38435_d_n7, assign27560_e38435_d_n10, assign27560_e38435_d_n11, assign27560_e38435_d_n12, assign27560_e38435_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27560_e38429: f64 = (locals.var_chi_b - locals.var_chi_a);
        let assign27560_e38432: f64 = (0.0008 * 75.0);
        let assign27560_e38433: f64 = (assign27560_e38429 - assign27560_e38432);
        (assign27560_e38433, (locals.var_chi_b_dn0 - locals.var_chi_a_dn0), (locals.var_chi_b_dn2 - locals.var_chi_a_dn2), (locals.var_chi_b_dn6 - locals.var_chi_a_dn6), (locals.var_chi_b_dn7 - locals.var_chi_a_dn7), (locals.var_chi_b_dn10 - locals.var_chi_a_dn10), (locals.var_chi_b_dn11 - locals.var_chi_a_dn11), (locals.var_chi_b_dn12 - locals.var_chi_a_dn12), (locals.var_chi_b_dn17 - locals.var_chi_a_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign27560_e38435;
        locals.var_tmf1_dn0 = assign27560_e38435_d_n0;
        locals.var_tmf1_dn2 = assign27560_e38435_d_n2;
        locals.var_tmf1_dn6 = assign27560_e38435_d_n6;
        locals.var_tmf1_dn7 = assign27560_e38435_d_n7;
        locals.var_tmf1_dn10 = assign27560_e38435_d_n10;
        locals.var_tmf1_dn11 = assign27560_e38435_d_n11;
        locals.var_tmf1_dn12 = assign27560_e38435_d_n12;
        locals.var_tmf1_dn17 = assign27560_e38435_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign27570_e38452, assign27570_e38452_d_n0, assign27570_e38452_d_n2, assign27570_e38452_d_n6, assign27570_e38452_d_n7, assign27570_e38452_d_n10, assign27570_e38452_d_n11, assign27570_e38452_d_n12, assign27570_e38452_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27570_e38446: f64 = (4.0 * locals.var_chi_b);
        let assign27570_e38449: f64 = (0.0008 * 75.0);
        let assign27570_e38450: f64 = (assign27570_e38446 * assign27570_e38449);
        (assign27570_e38450, ((4.0 * locals.var_chi_b_dn0) * assign27570_e38449), ((4.0 * locals.var_chi_b_dn2) * assign27570_e38449), ((4.0 * locals.var_chi_b_dn6) * assign27570_e38449), ((4.0 * locals.var_chi_b_dn7) * assign27570_e38449), ((4.0 * locals.var_chi_b_dn10) * assign27570_e38449), ((4.0 * locals.var_chi_b_dn11) * assign27570_e38449), ((4.0 * locals.var_chi_b_dn12) * assign27570_e38449), ((4.0 * locals.var_chi_b_dn17) * assign27570_e38449),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27570_e38452;
        locals.var_tmf2_dn0 = assign27570_e38452_d_n0;
        locals.var_tmf2_dn2 = assign27570_e38452_d_n2;
        locals.var_tmf2_dn6 = assign27570_e38452_d_n6;
        locals.var_tmf2_dn7 = assign27570_e38452_d_n7;
        locals.var_tmf2_dn10 = assign27570_e38452_d_n10;
        locals.var_tmf2_dn11 = assign27570_e38452_d_n11;
        locals.var_tmf2_dn12 = assign27570_e38452_d_n12;
        locals.var_tmf2_dn17 = assign27570_e38452_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27580_e38469, assign27580_e38469_d_n0, assign27580_e38469_d_n2, assign27580_e38469_d_n6, assign27580_e38469_d_n7, assign27580_e38469_d_n10, assign27580_e38469_d_n11, assign27580_e38469_d_n12, assign27580_e38469_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let (assign27580_e38467, assign27580_e38467_d_n0, assign27580_e38467_d_n2, assign27580_e38467_d_n6, assign27580_e38467_d_n7, assign27580_e38467_d_n10, assign27580_e38467_d_n11, assign27580_e38467_d_n12, assign27580_e38467_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign27580_e38466: f64 = (-locals.var_tmf2);
                (assign27580_e38466, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign27580_e38467, assign27580_e38467_d_n0, assign27580_e38467_d_n2, assign27580_e38467_d_n6, assign27580_e38467_d_n7, assign27580_e38467_d_n10, assign27580_e38467_d_n11, assign27580_e38467_d_n12, assign27580_e38467_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27580_e38469;
        locals.var_tmf2_dn0 = assign27580_e38469_d_n0;
        locals.var_tmf2_dn2 = assign27580_e38469_d_n2;
        locals.var_tmf2_dn6 = assign27580_e38469_d_n6;
        locals.var_tmf2_dn7 = assign27580_e38469_d_n7;
        locals.var_tmf2_dn10 = assign27580_e38469_d_n10;
        locals.var_tmf2_dn11 = assign27580_e38469_d_n11;
        locals.var_tmf2_dn12 = assign27580_e38469_d_n12;
        locals.var_tmf2_dn17 = assign27580_e38469_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27590_e38485, assign27590_e38485_d_n0, assign27590_e38485_d_n2, assign27590_e38485_d_n6, assign27590_e38485_d_n7, assign27590_e38485_d_n10, assign27590_e38485_d_n11, assign27590_e38485_d_n12, assign27590_e38485_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27590_e38480: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign27590_e38482: f64 = (assign27590_e38480 + locals.var_tmf2);
        let assign27590_e38483: f64 = (assign27590_e38482).sqrt();
        (assign27590_e38483, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign27590_e38483)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign27590_e38483)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign27590_e38483)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign27590_e38483)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign27590_e38483)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign27590_e38483)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign27590_e38483)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign27590_e38483)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27590_e38485;
        locals.var_tmf2_dn0 = assign27590_e38485_d_n0;
        locals.var_tmf2_dn2 = assign27590_e38485_d_n2;
        locals.var_tmf2_dn6 = assign27590_e38485_d_n6;
        locals.var_tmf2_dn7 = assign27590_e38485_d_n7;
        locals.var_tmf2_dn10 = assign27590_e38485_d_n10;
        locals.var_tmf2_dn11 = assign27590_e38485_d_n11;
        locals.var_tmf2_dn12 = assign27590_e38485_d_n12;
        locals.var_tmf2_dn17 = assign27590_e38485_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27600_e38502, assign27600_e38502_d_n0, assign27600_e38502_d_n2, assign27600_e38502_d_n6, assign27600_e38502_d_n7, assign27600_e38502_d_n10, assign27600_e38502_d_n11, assign27600_e38502_d_n12, assign27600_e38502_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27600_e38498: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign27600_e38499: f64 = (1.0 + assign27600_e38498);
        let assign27600_e38500: f64 = (0.5 * assign27600_e38499);
        (assign27600_e38500, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk773, locals.var_t1__blk773_dn0, locals.var_t1__blk773_dn2, locals.var_t1__blk773_dn6, locals.var_t1__blk773_dn7, locals.var_t1__blk773_dn10, locals.var_t1__blk773_dn11, locals.var_t1__blk773_dn12, locals.var_t1__blk773_dn17,)
    }
};
        locals.var_t1__blk773 = assign27600_e38502;
        locals.var_t1__blk773_dn0 = assign27600_e38502_d_n0;
        locals.var_t1__blk773_dn2 = assign27600_e38502_d_n2;
        locals.var_t1__blk773_dn6 = assign27600_e38502_d_n6;
        locals.var_t1__blk773_dn7 = assign27600_e38502_d_n7;
        locals.var_t1__blk773_dn10 = assign27600_e38502_d_n10;
        locals.var_t1__blk773_dn11 = assign27600_e38502_d_n11;
        locals.var_t1__blk773_dn12 = assign27600_e38502_d_n12;
        locals.var_t1__blk773_dn17 = assign27600_e38502_d_n17;
        locals.var_t1__blk773_rv = 0.0;

        let (assign27610_e38525, assign27610_e38525_d_n0, assign27610_e38525_d_n2, assign27610_e38525_d_n6, assign27610_e38525_d_n7, assign27610_e38525_d_n10, assign27610_e38525_d_n11, assign27610_e38525_d_n12, assign27610_e38525_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27610_e38516: f64 = (2.0 * 0.0008);
        let assign27610_e38518: f64 = (assign27610_e38516 * 75.0);
        let assign27610_e38519: f64 = (locals.var_tmf1 + assign27610_e38518);
        let assign27610_e38521: f64 = (assign27610_e38519 / locals.var_tmf2);
        let assign27610_e38522: f64 = (1.0 - assign27610_e38521);
        let assign27610_e38523: f64 = (0.5 * assign27610_e38522);
        (assign27610_e38523, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign27610_e38519 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign27610_e38519 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign27610_e38519 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign27610_e38519 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign27610_e38519 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign27610_e38519 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign27610_e38519 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign27610_e38519 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk774, locals.var_t2__blk774_dn0, locals.var_t2__blk774_dn2, locals.var_t2__blk774_dn6, locals.var_t2__blk774_dn7, locals.var_t2__blk774_dn10, locals.var_t2__blk774_dn11, locals.var_t2__blk774_dn12, locals.var_t2__blk774_dn17,)
    }
};
        locals.var_t2__blk774 = assign27610_e38525;
        locals.var_t2__blk774_dn0 = assign27610_e38525_d_n0;
        locals.var_t2__blk774_dn2 = assign27610_e38525_d_n2;
        locals.var_t2__blk774_dn6 = assign27610_e38525_d_n6;
        locals.var_t2__blk774_dn7 = assign27610_e38525_d_n7;
        locals.var_t2__blk774_dn10 = assign27610_e38525_d_n10;
        locals.var_t2__blk774_dn11 = assign27610_e38525_d_n11;
        locals.var_t2__blk774_dn12 = assign27610_e38525_d_n12;
        locals.var_t2__blk774_dn17 = assign27610_e38525_d_n17;
        locals.var_t2__blk774_rv = 0.0;

        let (assign27620_e38542, assign27620_e38542_d_n0, assign27620_e38542_d_n2, assign27620_e38542_d_n6, assign27620_e38542_d_n7, assign27620_e38542_d_n10, assign27620_e38542_d_n11, assign27620_e38542_d_n12, assign27620_e38542_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27620_e38538: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign27620_e38539: f64 = (0.5 * assign27620_e38538);
        let assign27620_e38540: f64 = (locals.var_chi_b - assign27620_e38539);
        (assign27620_e38540, (locals.var_chi_b_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_chi_b_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_chi_b_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi__blk816, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    }
};
        locals.var_chi__blk816 = assign27620_e38542;
        locals.var_chi__blk816_dn0 = assign27620_e38542_d_n0;
        locals.var_chi__blk816_dn2 = assign27620_e38542_d_n2;
        locals.var_chi__blk816_dn6 = assign27620_e38542_d_n6;
        locals.var_chi__blk816_dn7 = assign27620_e38542_d_n7;
        locals.var_chi__blk816_dn10 = assign27620_e38542_d_n10;
        locals.var_chi__blk816_dn11 = assign27620_e38542_d_n11;
        locals.var_chi__blk816_dn12 = assign27620_e38542_d_n12;
        locals.var_chi__blk816_dn17 = assign27620_e38542_d_n17;
        locals.var_chi__blk816_rv = 0.0;

        let (assign27630_e38557, assign27630_e38557_d_n0, assign27630_e38557_d_n2, assign27630_e38557_d_n6, assign27630_e38557_d_n7, assign27630_e38557_d_n10, assign27630_e38557_d_n11, assign27630_e38557_d_n12, assign27630_e38557_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27630_e38553: f64 = (locals.var_chi__blk816 / locals.var_beta);
        let assign27630_e38555: f64 = (assign27630_e38553 - locals.var_vxbgmtcl);
        (assign27630_e38555, ((locals.var_chi__blk816_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk816_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk816_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk816_dn7 / locals.var_beta) - locals.var_vxbgmtcl_dn7), ((((locals.var_chi__blk816_dn10 * locals.var_beta) - (locals.var_chi__blk816 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk816_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk816_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk816_dn17 / locals.var_beta) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
        locals.var_ps0ld = assign27630_e38557;
        locals.var_ps0ld_dn0 = assign27630_e38557_d_n0;
        locals.var_ps0ld_dn2 = assign27630_e38557_d_n2;
        locals.var_ps0ld_dn6 = assign27630_e38557_d_n6;
        locals.var_ps0ld_dn7 = assign27630_e38557_d_n7;
        locals.var_ps0ld_dn10 = assign27630_e38557_d_n10;
        locals.var_ps0ld_dn11 = assign27630_e38557_d_n11;
        locals.var_ps0ld_dn12 = assign27630_e38557_d_n12;
        locals.var_ps0ld_dn17 = assign27630_e38557_d_n17;
        locals.var_ps0ld_rv = 0.0;

        let (assign27640_e38574, assign27640_e38574_d_n0, assign27640_e38574_d_n2, assign27640_e38574_d_n6, assign27640_e38574_d_n7, assign27640_e38574_d_n10, assign27640_e38574_d_n11, assign27640_e38574_d_n12, assign27640_e38574_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27640_e38568: f64 = (locals.var_chi__blk816 - 1.0);
        let assign27640_e38570: f64 = (-locals.var_chi__blk816);
        let assign27640_e38571: f64 = (assign27640_e38570).exp();
        let assign27640_e38572: f64 = (assign27640_e38568 + assign27640_e38571);
        (assign27640_e38572, (locals.var_chi__blk816_dn0 + (assign27640_e38571 * (-locals.var_chi__blk816_dn0))), (locals.var_chi__blk816_dn2 + (assign27640_e38571 * (-locals.var_chi__blk816_dn2))), (locals.var_chi__blk816_dn6 + (assign27640_e38571 * (-locals.var_chi__blk816_dn6))), (locals.var_chi__blk816_dn7 + (assign27640_e38571 * (-locals.var_chi__blk816_dn7))), (locals.var_chi__blk816_dn10 + (assign27640_e38571 * (-locals.var_chi__blk816_dn10))), (locals.var_chi__blk816_dn11 + (assign27640_e38571 * (-locals.var_chi__blk816_dn11))), (locals.var_chi__blk816_dn12 + (assign27640_e38571 * (-locals.var_chi__blk816_dn12))), (locals.var_chi__blk816_dn17 + (assign27640_e38571 * (-locals.var_chi__blk816_dn17))),)
    } else {
        (locals.var_t1__blk773, locals.var_t1__blk773_dn0, locals.var_t1__blk773_dn2, locals.var_t1__blk773_dn6, locals.var_t1__blk773_dn7, locals.var_t1__blk773_dn10, locals.var_t1__blk773_dn11, locals.var_t1__blk773_dn12, locals.var_t1__blk773_dn17,)
    }
};
        locals.var_t1__blk773 = assign27640_e38574;
        locals.var_t1__blk773_dn0 = assign27640_e38574_d_n0;
        locals.var_t1__blk773_dn2 = assign27640_e38574_d_n2;
        locals.var_t1__blk773_dn6 = assign27640_e38574_d_n6;
        locals.var_t1__blk773_dn7 = assign27640_e38574_d_n7;
        locals.var_t1__blk773_dn10 = assign27640_e38574_d_n10;
        locals.var_t1__blk773_dn11 = assign27640_e38574_d_n11;
        locals.var_t1__blk773_dn12 = assign27640_e38574_d_n12;
        locals.var_t1__blk773_dn17 = assign27640_e38574_d_n17;
        locals.var_t1__blk773_rv = 0.0;

        let assign27650_e38578: f64 = (10.0 * 2.220446049250313e-16);
        let assign27650_e38579: f64 = if locals.var_t1__blk773 < assign27650_e38578 { 1.0 } else { 0.0 };
        locals.var_guard878 = assign27650_e38579;
        locals.var_guard878_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_99(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27660_e38594, assign27660_e38594_d_n0, assign27660_e38594_d_n2, assign27660_e38594_d_n6, assign27660_e38594_d_n7, assign27660_e38594_d_n10, assign27660_e38594_d_n11, assign27660_e38594_d_n12, assign27660_e38594_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard878 != 0.0)) {
        let assign27660_e38592: f64 = (10.0 * 2.220446049250313e-16);
        (assign27660_e38592, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk773, locals.var_t1__blk773_dn0, locals.var_t1__blk773_dn2, locals.var_t1__blk773_dn6, locals.var_t1__blk773_dn7, locals.var_t1__blk773_dn10, locals.var_t1__blk773_dn11, locals.var_t1__blk773_dn12, locals.var_t1__blk773_dn17,)
    }
};
        locals.var_t1__blk773 = assign27660_e38594;
        locals.var_t1__blk773_dn0 = assign27660_e38594_d_n0;
        locals.var_t1__blk773_dn2 = assign27660_e38594_d_n2;
        locals.var_t1__blk773_dn6 = assign27660_e38594_d_n6;
        locals.var_t1__blk773_dn7 = assign27660_e38594_d_n7;
        locals.var_t1__blk773_dn10 = assign27660_e38594_d_n10;
        locals.var_t1__blk773_dn11 = assign27660_e38594_d_n11;
        locals.var_t1__blk773_dn12 = assign27660_e38594_d_n12;
        locals.var_t1__blk773_dn17 = assign27660_e38594_d_n17;
        locals.var_t1__blk773_rv = 0.0;

        let (assign27670_e38606, assign27670_e38606_d_n0, assign27670_e38606_d_n2, assign27670_e38606_d_n6, assign27670_e38606_d_n7, assign27670_e38606_d_n10, assign27670_e38606_d_n11, assign27670_e38606_d_n12, assign27670_e38606_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27670_e38604: f64 = (locals.var_t1__blk773).sqrt();
        (assign27670_e38604, (locals.var_t1__blk773_dn0 / (2.0 * assign27670_e38604)), (locals.var_t1__blk773_dn2 / (2.0 * assign27670_e38604)), (locals.var_t1__blk773_dn6 / (2.0 * assign27670_e38604)), (locals.var_t1__blk773_dn7 / (2.0 * assign27670_e38604)), (locals.var_t1__blk773_dn10 / (2.0 * assign27670_e38604)), (locals.var_t1__blk773_dn11 / (2.0 * assign27670_e38604)), (locals.var_t1__blk773_dn12 / (2.0 * assign27670_e38604)), (locals.var_t1__blk773_dn17 / (2.0 * assign27670_e38604)),)
    } else {
        (locals.var_t2__blk774, locals.var_t2__blk774_dn0, locals.var_t2__blk774_dn2, locals.var_t2__blk774_dn6, locals.var_t2__blk774_dn7, locals.var_t2__blk774_dn10, locals.var_t2__blk774_dn11, locals.var_t2__blk774_dn12, locals.var_t2__blk774_dn17,)
    }
};
        locals.var_t2__blk774 = assign27670_e38606;
        locals.var_t2__blk774_dn0 = assign27670_e38606_d_n0;
        locals.var_t2__blk774_dn2 = assign27670_e38606_d_n2;
        locals.var_t2__blk774_dn6 = assign27670_e38606_d_n6;
        locals.var_t2__blk774_dn7 = assign27670_e38606_d_n7;
        locals.var_t2__blk774_dn10 = assign27670_e38606_d_n10;
        locals.var_t2__blk774_dn11 = assign27670_e38606_d_n11;
        locals.var_t2__blk774_dn12 = assign27670_e38606_d_n12;
        locals.var_t2__blk774_dn17 = assign27670_e38606_d_n17;
        locals.var_t2__blk774_rv = 0.0;

        let (assign27680_e38619, assign27680_e38619_d_n0, assign27680_e38619_d_n2, assign27680_e38619_d_n6, assign27680_e38619_d_n7, assign27680_e38619_d_n10, assign27680_e38619_d_n11, assign27680_e38619_d_n12, assign27680_e38619_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27680_e38617: f64 = (locals.var_cnst0over * locals.var_t2__blk774);
        (assign27680_e38617, ((locals.var_cnst0over_dn0 * locals.var_t2__blk774) + (locals.var_cnst0over * locals.var_t2__blk774_dn0)), ((locals.var_cnst0over_dn2 * locals.var_t2__blk774) + (locals.var_cnst0over * locals.var_t2__blk774_dn2)), ((locals.var_cnst0over_dn6 * locals.var_t2__blk774) + (locals.var_cnst0over * locals.var_t2__blk774_dn6)), ((locals.var_cnst0over_dn7 * locals.var_t2__blk774) + (locals.var_cnst0over * locals.var_t2__blk774_dn7)), ((locals.var_cnst0over_dn10 * locals.var_t2__blk774) + (locals.var_cnst0over * locals.var_t2__blk774_dn10)), ((locals.var_cnst0over_dn11 * locals.var_t2__blk774) + (locals.var_cnst0over * locals.var_t2__blk774_dn11)), ((locals.var_cnst0over_dn12 * locals.var_t2__blk774) + (locals.var_cnst0over * locals.var_t2__blk774_dn12)), ((locals.var_cnst0over_dn17 * locals.var_t2__blk774) + (locals.var_cnst0over * locals.var_t2__blk774_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign27680_e38619;
        locals.var_qbuld_dn0 = assign27680_e38619_d_n0;
        locals.var_qbuld_dn2 = assign27680_e38619_d_n2;
        locals.var_qbuld_dn6 = assign27680_e38619_d_n6;
        locals.var_qbuld_dn7 = assign27680_e38619_d_n7;
        locals.var_qbuld_dn10 = assign27680_e38619_d_n10;
        locals.var_qbuld_dn11 = assign27680_e38619_d_n11;
        locals.var_qbuld_dn12 = assign27680_e38619_d_n12;
        locals.var_qbuld_dn17 = assign27680_e38619_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign27690_e38634, assign27690_e38634_d_n0, assign27690_e38634_d_n2, assign27690_e38634_d_n6, assign27690_e38634_d_n7, assign27690_e38634_d_n10, assign27690_e38634_d_n11, assign27690_e38634_d_n12, assign27690_e38634_d_n17,) = {
    if ((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign27690_e38631: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign27690_e38632: f64 = (locals.var_cox0 * assign27690_e38631);
        (assign27690_e38632, (locals.var_cox0 * (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0)), (locals.var_cox0 * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0 * (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6)), (locals.var_cox0 * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0 * (locals.var_vgpld_dn10 - locals.var_ps0ld_dn10)), (locals.var_cox0 * (locals.var_vgpld_dn11 - locals.var_ps0ld_dn11)), (locals.var_cox0 * (locals.var_vgpld_dn12 - locals.var_ps0ld_dn12)), (locals.var_cox0 * (locals.var_vgpld_dn17 - locals.var_ps0ld_dn17)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign27690_e38634;
        locals.var_qsuld_dn0 = assign27690_e38634_d_n0;
        locals.var_qsuld_dn2 = assign27690_e38634_d_n2;
        locals.var_qsuld_dn6 = assign27690_e38634_d_n6;
        locals.var_qsuld_dn7 = assign27690_e38634_d_n7;
        locals.var_qsuld_dn10 = assign27690_e38634_d_n10;
        locals.var_qsuld_dn11 = assign27690_e38634_d_n11;
        locals.var_qsuld_dn12 = assign27690_e38634_d_n12;
        locals.var_qsuld_dn17 = assign27690_e38634_d_n17;
        locals.var_qsuld_rv = 0.0;

        let assign27700_e38637: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard879 = assign27700_e38637;
        locals.var_guard879_rv = 0.0;

        let (assign27710_e38654, assign27710_e38654_d_n0, assign27710_e38654_d_n2, assign27710_e38654_d_n6, assign27710_e38654_d_n7, assign27710_e38654_d_n10, assign27710_e38654_d_n11, assign27710_e38654_d_n12, assign27710_e38654_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27710_e38650: f64 = (-locals.var_vxbgmtcl);
        let assign27710_e38651: f64 = (locals.var_beta * assign27710_e38650);
        let assign27710_e38652: f64 = (assign27710_e38651).exp();
        (assign27710_e38652, (assign27710_e38652 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign27710_e38652 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign27710_e38652 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign27710_e38652 * (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), (assign27710_e38652 * ((locals.var_beta_dn10 * assign27710_e38650) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign27710_e38652 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign27710_e38652 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))), (assign27710_e38652 * (locals.var_beta * (-locals.var_vxbgmtcl_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk835, locals.var_exp_bvbs__blk835_dn0, locals.var_exp_bvbs__blk835_dn2, locals.var_exp_bvbs__blk835_dn6, locals.var_exp_bvbs__blk835_dn7, locals.var_exp_bvbs__blk835_dn10, locals.var_exp_bvbs__blk835_dn11, locals.var_exp_bvbs__blk835_dn12, locals.var_exp_bvbs__blk835_dn17,)
    }
};
        locals.var_exp_bvbs__blk835 = assign27710_e38654;
        locals.var_exp_bvbs__blk835_dn0 = assign27710_e38654_d_n0;
        locals.var_exp_bvbs__blk835_dn2 = assign27710_e38654_d_n2;
        locals.var_exp_bvbs__blk835_dn6 = assign27710_e38654_d_n6;
        locals.var_exp_bvbs__blk835_dn7 = assign27710_e38654_d_n7;
        locals.var_exp_bvbs__blk835_dn10 = assign27710_e38654_d_n10;
        locals.var_exp_bvbs__blk835_dn11 = assign27710_e38654_d_n11;
        locals.var_exp_bvbs__blk835_dn12 = assign27710_e38654_d_n12;
        locals.var_exp_bvbs__blk835_dn17 = assign27710_e38654_d_n17;
        locals.var_exp_bvbs__blk835_rv = 0.0;

        let (assign27720_e38669, assign27720_e38669_d_n0, assign27720_e38669_d_n2, assign27720_e38669_d_n6, assign27720_e38669_d_n7, assign27720_e38669_d_n10, assign27720_e38669_d_n11, assign27720_e38669_d_n12, assign27720_e38669_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27720_e38667: f64 = (locals.var_nin / locals.var_uc_nsubbttub);
        (assign27720_e38667, (((locals.var_nin_dn0 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn0)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn2 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn2)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn6 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn6)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn7 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn7)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn10 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn10)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn11 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn11)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn12 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn12)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn17 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn17)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)),)
    } else {
        (locals.var_t0__blk772, locals.var_t0__blk772_dn0, locals.var_t0__blk772_dn2, locals.var_t0__blk772_dn6, locals.var_t0__blk772_dn7, locals.var_t0__blk772_dn10, locals.var_t0__blk772_dn11, locals.var_t0__blk772_dn12, locals.var_t0__blk772_dn17,)
    }
};
        locals.var_t0__blk772 = assign27720_e38669;
        locals.var_t0__blk772_dn0 = assign27720_e38669_d_n0;
        locals.var_t0__blk772_dn2 = assign27720_e38669_d_n2;
        locals.var_t0__blk772_dn6 = assign27720_e38669_d_n6;
        locals.var_t0__blk772_dn7 = assign27720_e38669_d_n7;
        locals.var_t0__blk772_dn10 = assign27720_e38669_d_n10;
        locals.var_t0__blk772_dn11 = assign27720_e38669_d_n11;
        locals.var_t0__blk772_dn12 = assign27720_e38669_d_n12;
        locals.var_t0__blk772_dn17 = assign27720_e38669_d_n17;
        locals.var_t0__blk772_rv = 0.0;

        let (assign27730_e38684, assign27730_e38684_d_n0, assign27730_e38684_d_n2, assign27730_e38684_d_n6, assign27730_e38684_d_n7, assign27730_e38684_d_n10, assign27730_e38684_d_n11, assign27730_e38684_d_n12, assign27730_e38684_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27730_e38682: f64 = (locals.var_t0__blk772 * locals.var_t0__blk772);
        (assign27730_e38682, ((locals.var_t0__blk772_dn0 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn0)), ((locals.var_t0__blk772_dn2 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn2)), ((locals.var_t0__blk772_dn6 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn6)), ((locals.var_t0__blk772_dn7 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn7)), ((locals.var_t0__blk772_dn10 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn10)), ((locals.var_t0__blk772_dn11 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn11)), ((locals.var_t0__blk772_dn12 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn12)), ((locals.var_t0__blk772_dn17 * locals.var_t0__blk772) + (locals.var_t0__blk772 * locals.var_t0__blk772_dn17)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12, locals.var_cnst1over_dn17,)
    }
};
        locals.var_cnst1over = assign27730_e38684;
        locals.var_cnst1over_dn0 = assign27730_e38684_d_n0;
        locals.var_cnst1over_dn2 = assign27730_e38684_d_n2;
        locals.var_cnst1over_dn6 = assign27730_e38684_d_n6;
        locals.var_cnst1over_dn7 = assign27730_e38684_d_n7;
        locals.var_cnst1over_dn10 = assign27730_e38684_d_n10;
        locals.var_cnst1over_dn11 = assign27730_e38684_d_n11;
        locals.var_cnst1over_dn12 = assign27730_e38684_d_n12;
        locals.var_cnst1over_dn17 = assign27730_e38684_d_n17;
        locals.var_cnst1over_rv = 0.0;

        let (assign27740_e38699, assign27740_e38699_d_n0, assign27740_e38699_d_n2, assign27740_e38699_d_n6, assign27740_e38699_d_n7, assign27740_e38699_d_n10, assign27740_e38699_d_n11, assign27740_e38699_d_n12, assign27740_e38699_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27740_e38697: f64 = (locals.var_cnst1over * locals.var_exp_bvbs__blk835);
        (assign27740_e38697, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn2)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn7)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn12)), ((locals.var_cnst1over_dn17 * locals.var_exp_bvbs__blk835) + (locals.var_cnst1over * locals.var_exp_bvbs__blk835_dn17)),)
    } else {
        (locals.var_cfs1__blk844, locals.var_cfs1__blk844_dn0, locals.var_cfs1__blk844_dn2, locals.var_cfs1__blk844_dn6, locals.var_cfs1__blk844_dn7, locals.var_cfs1__blk844_dn10, locals.var_cfs1__blk844_dn11, locals.var_cfs1__blk844_dn12, locals.var_cfs1__blk844_dn17,)
    }
};
        locals.var_cfs1__blk844 = assign27740_e38699;
        locals.var_cfs1__blk844_dn0 = assign27740_e38699_d_n0;
        locals.var_cfs1__blk844_dn2 = assign27740_e38699_d_n2;
        locals.var_cfs1__blk844_dn6 = assign27740_e38699_d_n6;
        locals.var_cfs1__blk844_dn7 = assign27740_e38699_d_n7;
        locals.var_cfs1__blk844_dn10 = assign27740_e38699_d_n10;
        locals.var_cfs1__blk844_dn11 = assign27740_e38699_d_n11;
        locals.var_cfs1__blk844_dn12 = assign27740_e38699_d_n12;
        locals.var_cfs1__blk844_dn17 = assign27740_e38699_d_n17;
        locals.var_cfs1__blk844_rv = 0.0;

        let (assign27750_e38712,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv__blk789,)
    }
};
        locals.var_flg_conv__blk789 = assign27750_e38712;
        locals.var_flg_conv__blk789_rv = 0.0;

        let (assign27760_e38725,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign27760_e38725;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_100(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign27770_loop_guard: usize = 0;
        while {
            let assign27770_cond_e38739: f64 = (2.0 * 20.0);
            let assign27770_cond_e38741: f64 = (assign27770_cond_e38739 + 1.0);
            let assign27770_cond_e38743: f64 = if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_lp_s0 <= assign27770_cond_e38741)) { 1.0 } else { 0.0 };
            assign27770_cond_e38743 != 0.0
        } {
            assign27770_loop_guard += 1;
            assert!(assign27770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign27770_body0_e38756, assign27770_body0_e38756_d_n0, assign27770_body0_e38756_d_n2, assign27770_body0_e38756_d_n6, assign27770_body0_e38756_d_n7, assign27770_body0_e38756_d_n10, assign27770_body0_e38756_d_n11, assign27770_body0_e38756_d_n12, assign27770_body0_e38756_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb__blk840, locals.var_fb__blk840_dn0, locals.var_fb__blk840_dn2, locals.var_fb__blk840_dn6, locals.var_fb__blk840_dn7, locals.var_fb__blk840_dn10, locals.var_fb__blk840_dn11, locals.var_fb__blk840_dn12, locals.var_fb__blk840_dn17,)
    }
};
            locals.var_fb__blk840 = assign27770_body0_e38756;
            locals.var_fb__blk840_dn0 = assign27770_body0_e38756_d_n0;
            locals.var_fb__blk840_dn2 = assign27770_body0_e38756_d_n2;
            locals.var_fb__blk840_dn6 = assign27770_body0_e38756_d_n6;
            locals.var_fb__blk840_dn7 = assign27770_body0_e38756_d_n7;
            locals.var_fb__blk840_dn10 = assign27770_body0_e38756_d_n10;
            locals.var_fb__blk840_dn11 = assign27770_body0_e38756_d_n11;
            locals.var_fb__blk840_dn12 = assign27770_body0_e38756_d_n12;
            locals.var_fb__blk840_dn17 = assign27770_body0_e38756_d_n17;
            locals.var_fb__blk840_rv = 0.0;
            let (assign27770_body1_e38773, assign27770_body1_e38773_d_n0, assign27770_body1_e38773_d_n2, assign27770_body1_e38773_d_n6, assign27770_body1_e38773_d_n7, assign27770_body1_e38773_d_n10, assign27770_body1_e38773_d_n11, assign27770_body1_e38773_d_n12, assign27770_body1_e38773_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27770_body1_e38770: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        let assign27770_body1_e38771: f64 = (locals.var_beta * assign27770_body1_e38770);
        (assign27770_body1_e38771, (locals.var_beta * (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign27770_body1_e38770) + (locals.var_beta * (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0ld_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0ld_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk816, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    }
};
            locals.var_chi__blk816 = assign27770_body1_e38773;
            locals.var_chi__blk816_dn0 = assign27770_body1_e38773_d_n0;
            locals.var_chi__blk816_dn2 = assign27770_body1_e38773_d_n2;
            locals.var_chi__blk816_dn6 = assign27770_body1_e38773_d_n6;
            locals.var_chi__blk816_dn7 = assign27770_body1_e38773_d_n7;
            locals.var_chi__blk816_dn10 = assign27770_body1_e38773_d_n10;
            locals.var_chi__blk816_dn11 = assign27770_body1_e38773_d_n11;
            locals.var_chi__blk816_dn12 = assign27770_body1_e38773_d_n12;
            locals.var_chi__blk816_dn17 = assign27770_body1_e38773_d_n17;
            locals.var_chi__blk816_rv = 0.0;
            let assign27770_body2_e38776: f64 = if locals.var_chi__blk816 < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard880 = assign27770_body2_e38776;
            locals.var_guard880_rv = 0.0;
            let (assign27770_body3_e38806, assign27770_body3_e38806_d_n0, assign27770_body3_e38806_d_n2, assign27770_body3_e38806_d_n6, assign27770_body3_e38806_d_n7, assign27770_body3_e38806_d_n10, assign27770_body3_e38806_d_n11, assign27770_body3_e38806_d_n12, assign27770_body3_e38806_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard880 != 0.0)) {
        let assign27770_body3_e38791: f64 = (locals.var_chi__blk816 * locals.var_chi__blk816);
        let assign27770_body3_e38793: f64 = (assign27770_body3_e38791 * locals.var_chi__blk816);
        let assign27770_body3_e38797: f64 = (-0.07053654284009761);
        let assign27770_body3_e38800: f64 = (locals.var_chi__blk816 * 0.006115288895133179);
        let assign27770_body3_e38801: f64 = (assign27770_body3_e38797 + assign27770_body3_e38800);
        let assign27770_body3_e38802: f64 = (locals.var_chi__blk816 * assign27770_body3_e38801);
        let assign27770_body3_e38803: f64 = (0.29693154855771 + assign27770_body3_e38802);
        let assign27770_body3_e38804: f64 = (assign27770_body3_e38793 * assign27770_body3_e38803);
        (assign27770_body3_e38804, ((((((locals.var_chi__blk816_dn0 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn0)) * locals.var_chi__blk816) + (assign27770_body3_e38791 * locals.var_chi__blk816_dn0)) * assign27770_body3_e38803) + (assign27770_body3_e38793 * ((locals.var_chi__blk816_dn0 * assign27770_body3_e38801) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn0 * 0.006115288895133179))))), ((((((locals.var_chi__blk816_dn2 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn2)) * locals.var_chi__blk816) + (assign27770_body3_e38791 * locals.var_chi__blk816_dn2)) * assign27770_body3_e38803) + (assign27770_body3_e38793 * ((locals.var_chi__blk816_dn2 * assign27770_body3_e38801) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn2 * 0.006115288895133179))))), ((((((locals.var_chi__blk816_dn6 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn6)) * locals.var_chi__blk816) + (assign27770_body3_e38791 * locals.var_chi__blk816_dn6)) * assign27770_body3_e38803) + (assign27770_body3_e38793 * ((locals.var_chi__blk816_dn6 * assign27770_body3_e38801) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn6 * 0.006115288895133179))))), ((((((locals.var_chi__blk816_dn7 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn7)) * locals.var_chi__blk816) + (assign27770_body3_e38791 * locals.var_chi__blk816_dn7)) * assign27770_body3_e38803) + (assign27770_body3_e38793 * ((locals.var_chi__blk816_dn7 * assign27770_body3_e38801) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn7 * 0.006115288895133179))))), ((((((locals.var_chi__blk816_dn10 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn10)) * locals.var_chi__blk816) + (assign27770_body3_e38791 * locals.var_chi__blk816_dn10)) * assign27770_body3_e38803) + (assign27770_body3_e38793 * ((locals.var_chi__blk816_dn10 * assign27770_body3_e38801) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn10 * 0.006115288895133179))))), ((((((locals.var_chi__blk816_dn11 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn11)) * locals.var_chi__blk816) + (assign27770_body3_e38791 * locals.var_chi__blk816_dn11)) * assign27770_body3_e38803) + (assign27770_body3_e38793 * ((locals.var_chi__blk816_dn11 * assign27770_body3_e38801) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn11 * 0.006115288895133179))))), ((((((locals.var_chi__blk816_dn12 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn12)) * locals.var_chi__blk816) + (assign27770_body3_e38791 * locals.var_chi__blk816_dn12)) * assign27770_body3_e38803) + (assign27770_body3_e38793 * ((locals.var_chi__blk816_dn12 * assign27770_body3_e38801) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn12 * 0.006115288895133179))))), ((((((locals.var_chi__blk816_dn17 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn17)) * locals.var_chi__blk816) + (assign27770_body3_e38791 * locals.var_chi__blk816_dn17)) * assign27770_body3_e38803) + (assign27770_body3_e38793 * ((locals.var_chi__blk816_dn17 * assign27770_body3_e38801) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn17 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi, locals.var_fi_dn0, locals.var_fi_dn2, locals.var_fi_dn6, locals.var_fi_dn7, locals.var_fi_dn10, locals.var_fi_dn11, locals.var_fi_dn12, locals.var_fi_dn17,)
    }
};
            locals.var_fi = assign27770_body3_e38806;
            locals.var_fi_dn0 = assign27770_body3_e38806_d_n0;
            locals.var_fi_dn2 = assign27770_body3_e38806_d_n2;
            locals.var_fi_dn6 = assign27770_body3_e38806_d_n6;
            locals.var_fi_dn7 = assign27770_body3_e38806_d_n7;
            locals.var_fi_dn10 = assign27770_body3_e38806_d_n10;
            locals.var_fi_dn11 = assign27770_body3_e38806_d_n11;
            locals.var_fi_dn12 = assign27770_body3_e38806_d_n12;
            locals.var_fi_dn17 = assign27770_body3_e38806_d_n17;
            locals.var_fi_rv = 0.0;
            let (assign27770_body4_e38840, assign27770_body4_e38840_d_n0, assign27770_body4_e38840_d_n2, assign27770_body4_e38840_d_n6, assign27770_body4_e38840_d_n7, assign27770_body4_e38840_d_n10, assign27770_body4_e38840_d_n11, assign27770_body4_e38840_d_n12, assign27770_body4_e38840_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard880 != 0.0)) {
        let assign27770_body4_e38821: f64 = (locals.var_chi__blk816 * locals.var_chi__blk816);
        let assign27770_body4_e38824: f64 = (3.0 * 0.29693154855771);
        let assign27770_body4_e38828: f64 = (-0.07053654284009761);
        let assign27770_body4_e38829: f64 = (4.0 * assign27770_body4_e38828);
        let assign27770_body4_e38832: f64 = (locals.var_chi__blk816 * 5.0);
        let assign27770_body4_e38834: f64 = (assign27770_body4_e38832 * 0.006115288895133179);
        let assign27770_body4_e38835: f64 = (assign27770_body4_e38829 + assign27770_body4_e38834);
        let assign27770_body4_e38836: f64 = (locals.var_chi__blk816 * assign27770_body4_e38835);
        let assign27770_body4_e38837: f64 = (assign27770_body4_e38824 + assign27770_body4_e38836);
        let assign27770_body4_e38838: f64 = (assign27770_body4_e38821 * assign27770_body4_e38837);
        (assign27770_body4_e38838, ((((locals.var_chi__blk816_dn0 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn0)) * assign27770_body4_e38837) + (assign27770_body4_e38821 * ((locals.var_chi__blk816_dn0 * assign27770_body4_e38835) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk816_dn2 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn2)) * assign27770_body4_e38837) + (assign27770_body4_e38821 * ((locals.var_chi__blk816_dn2 * assign27770_body4_e38835) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk816_dn6 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn6)) * assign27770_body4_e38837) + (assign27770_body4_e38821 * ((locals.var_chi__blk816_dn6 * assign27770_body4_e38835) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk816_dn7 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn7)) * assign27770_body4_e38837) + (assign27770_body4_e38821 * ((locals.var_chi__blk816_dn7 * assign27770_body4_e38835) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk816_dn10 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn10)) * assign27770_body4_e38837) + (assign27770_body4_e38821 * ((locals.var_chi__blk816_dn10 * assign27770_body4_e38835) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk816_dn11 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn11)) * assign27770_body4_e38837) + (assign27770_body4_e38821 * ((locals.var_chi__blk816_dn11 * assign27770_body4_e38835) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk816_dn12 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn12)) * assign27770_body4_e38837) + (assign27770_body4_e38821 * ((locals.var_chi__blk816_dn12 * assign27770_body4_e38835) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn12 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk816_dn17 * locals.var_chi__blk816) + (locals.var_chi__blk816 * locals.var_chi__blk816_dn17)) * assign27770_body4_e38837) + (assign27770_body4_e38821 * ((locals.var_chi__blk816_dn17 * assign27770_body4_e38835) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn17 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi, locals.var_fi_dchi_dn0, locals.var_fi_dchi_dn2, locals.var_fi_dchi_dn6, locals.var_fi_dchi_dn7, locals.var_fi_dchi_dn10, locals.var_fi_dchi_dn11, locals.var_fi_dchi_dn12, locals.var_fi_dchi_dn17,)
    }
};
            locals.var_fi_dchi = assign27770_body4_e38840;
            locals.var_fi_dchi_dn0 = assign27770_body4_e38840_d_n0;
            locals.var_fi_dchi_dn2 = assign27770_body4_e38840_d_n2;
            locals.var_fi_dchi_dn6 = assign27770_body4_e38840_d_n6;
            locals.var_fi_dchi_dn7 = assign27770_body4_e38840_d_n7;
            locals.var_fi_dchi_dn10 = assign27770_body4_e38840_d_n10;
            locals.var_fi_dchi_dn11 = assign27770_body4_e38840_d_n11;
            locals.var_fi_dchi_dn12 = assign27770_body4_e38840_d_n12;
            locals.var_fi_dchi_dn17 = assign27770_body4_e38840_d_n17;
            locals.var_fi_dchi_rv = 0.0;
            let (assign27770_body5_e38859, assign27770_body5_e38859_d_n0, assign27770_body5_e38859_d_n2, assign27770_body5_e38859_d_n6, assign27770_body5_e38859_d_n7, assign27770_body5_e38859_d_n10, assign27770_body5_e38859_d_n11, assign27770_body5_e38859_d_n12, assign27770_body5_e38859_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard880 != 0.0)) {
        let assign27770_body5_e38855: f64 = (locals.var_cfs1__blk844 * locals.var_fi);
        let assign27770_body5_e38857: f64 = (assign27770_body5_e38855 * locals.var_fi);
        (assign27770_body5_e38857, ((((locals.var_cfs1__blk844_dn0 * locals.var_fi) + (locals.var_cfs1__blk844 * locals.var_fi_dn0)) * locals.var_fi) + (assign27770_body5_e38855 * locals.var_fi_dn0)), ((((locals.var_cfs1__blk844_dn2 * locals.var_fi) + (locals.var_cfs1__blk844 * locals.var_fi_dn2)) * locals.var_fi) + (assign27770_body5_e38855 * locals.var_fi_dn2)), ((((locals.var_cfs1__blk844_dn6 * locals.var_fi) + (locals.var_cfs1__blk844 * locals.var_fi_dn6)) * locals.var_fi) + (assign27770_body5_e38855 * locals.var_fi_dn6)), ((((locals.var_cfs1__blk844_dn7 * locals.var_fi) + (locals.var_cfs1__blk844 * locals.var_fi_dn7)) * locals.var_fi) + (assign27770_body5_e38855 * locals.var_fi_dn7)), ((((locals.var_cfs1__blk844_dn10 * locals.var_fi) + (locals.var_cfs1__blk844 * locals.var_fi_dn10)) * locals.var_fi) + (assign27770_body5_e38855 * locals.var_fi_dn10)), ((((locals.var_cfs1__blk844_dn11 * locals.var_fi) + (locals.var_cfs1__blk844 * locals.var_fi_dn11)) * locals.var_fi) + (assign27770_body5_e38855 * locals.var_fi_dn11)), ((((locals.var_cfs1__blk844_dn12 * locals.var_fi) + (locals.var_cfs1__blk844 * locals.var_fi_dn12)) * locals.var_fi) + (assign27770_body5_e38855 * locals.var_fi_dn12)), ((((locals.var_cfs1__blk844_dn17 * locals.var_fi) + (locals.var_cfs1__blk844 * locals.var_fi_dn17)) * locals.var_fi) + (assign27770_body5_e38855 * locals.var_fi_dn17)),)
    } else {
        (locals.var_fs01__blk838, locals.var_fs01__blk838_dn0, locals.var_fs01__blk838_dn2, locals.var_fs01__blk838_dn6, locals.var_fs01__blk838_dn7, locals.var_fs01__blk838_dn10, locals.var_fs01__blk838_dn11, locals.var_fs01__blk838_dn12, locals.var_fs01__blk838_dn17,)
    }
};
            locals.var_fs01__blk838 = assign27770_body5_e38859;
            locals.var_fs01__blk838_dn0 = assign27770_body5_e38859_d_n0;
            locals.var_fs01__blk838_dn2 = assign27770_body5_e38859_d_n2;
            locals.var_fs01__blk838_dn6 = assign27770_body5_e38859_d_n6;
            locals.var_fs01__blk838_dn7 = assign27770_body5_e38859_d_n7;
            locals.var_fs01__blk838_dn10 = assign27770_body5_e38859_d_n10;
            locals.var_fs01__blk838_dn11 = assign27770_body5_e38859_d_n11;
            locals.var_fs01__blk838_dn12 = assign27770_body5_e38859_d_n12;
            locals.var_fs01__blk838_dn17 = assign27770_body5_e38859_d_n17;
            locals.var_fs01__blk838_rv = 0.0;
            let (assign27770_body6_e38882, assign27770_body6_e38882_d_n0, assign27770_body6_e38882_d_n2, assign27770_body6_e38882_d_n6, assign27770_body6_e38882_d_n7, assign27770_body6_e38882_d_n10, assign27770_body6_e38882_d_n11, assign27770_body6_e38882_d_n12, assign27770_body6_e38882_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard880 != 0.0)) {
        let assign27770_body6_e38874: f64 = (locals.var_cfs1__blk844 * locals.var_beta);
        let assign27770_body6_e38876: f64 = (assign27770_body6_e38874 * 2.0);
        let assign27770_body6_e38878: f64 = (assign27770_body6_e38876 * locals.var_fi);
        let assign27770_body6_e38880: f64 = (assign27770_body6_e38878 * locals.var_fi_dchi);
        (assign27770_body6_e38880, ((((((locals.var_cfs1__blk844_dn0 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27770_body6_e38876 * locals.var_fi_dn0)) * locals.var_fi_dchi) + (assign27770_body6_e38878 * locals.var_fi_dchi_dn0)), ((((((locals.var_cfs1__blk844_dn2 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27770_body6_e38876 * locals.var_fi_dn2)) * locals.var_fi_dchi) + (assign27770_body6_e38878 * locals.var_fi_dchi_dn2)), ((((((locals.var_cfs1__blk844_dn6 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27770_body6_e38876 * locals.var_fi_dn6)) * locals.var_fi_dchi) + (assign27770_body6_e38878 * locals.var_fi_dchi_dn6)), ((((((locals.var_cfs1__blk844_dn7 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27770_body6_e38876 * locals.var_fi_dn7)) * locals.var_fi_dchi) + (assign27770_body6_e38878 * locals.var_fi_dchi_dn7)), (((((((locals.var_cfs1__blk844_dn10 * locals.var_beta) + (locals.var_cfs1__blk844 * locals.var_beta_dn10)) * 2.0) * locals.var_fi) + (assign27770_body6_e38876 * locals.var_fi_dn10)) * locals.var_fi_dchi) + (assign27770_body6_e38878 * locals.var_fi_dchi_dn10)), ((((((locals.var_cfs1__blk844_dn11 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27770_body6_e38876 * locals.var_fi_dn11)) * locals.var_fi_dchi) + (assign27770_body6_e38878 * locals.var_fi_dchi_dn11)), ((((((locals.var_cfs1__blk844_dn12 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27770_body6_e38876 * locals.var_fi_dn12)) * locals.var_fi_dchi) + (assign27770_body6_e38878 * locals.var_fi_dchi_dn12)), ((((((locals.var_cfs1__blk844_dn17 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27770_body6_e38876 * locals.var_fi_dn17)) * locals.var_fi_dchi) + (assign27770_body6_e38878 * locals.var_fi_dchi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk839, locals.var_fs01_dps0__blk839_dn0, locals.var_fs01_dps0__blk839_dn2, locals.var_fs01_dps0__blk839_dn6, locals.var_fs01_dps0__blk839_dn7, locals.var_fs01_dps0__blk839_dn10, locals.var_fs01_dps0__blk839_dn11, locals.var_fs01_dps0__blk839_dn12, locals.var_fs01_dps0__blk839_dn17,)
    }
};
            locals.var_fs01_dps0__blk839 = assign27770_body6_e38882;
            locals.var_fs01_dps0__blk839_dn0 = assign27770_body6_e38882_d_n0;
            locals.var_fs01_dps0__blk839_dn2 = assign27770_body6_e38882_d_n2;
            locals.var_fs01_dps0__blk839_dn6 = assign27770_body6_e38882_d_n6;
            locals.var_fs01_dps0__blk839_dn7 = assign27770_body6_e38882_d_n7;
            locals.var_fs01_dps0__blk839_dn10 = assign27770_body6_e38882_d_n10;
            locals.var_fs01_dps0__blk839_dn11 = assign27770_body6_e38882_d_n11;
            locals.var_fs01_dps0__blk839_dn12 = assign27770_body6_e38882_d_n12;
            locals.var_fs01_dps0__blk839_dn17 = assign27770_body6_e38882_d_n17;
            locals.var_fs01_dps0__blk839_rv = 0.0;
            let (assign27770_body7_e38917, assign27770_body7_e38917_d_n0, assign27770_body7_e38917_d_n2, assign27770_body7_e38917_d_n6, assign27770_body7_e38917_d_n7, assign27770_body7_e38917_d_n10, assign27770_body7_e38917_d_n11, assign27770_body7_e38917_d_n12, assign27770_body7_e38917_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard880 != 0.0)) {
        let assign27770_body7_e38899: f64 = (-0.117851130197758);
        let assign27770_body7_e38904: f64 = (-0.00163730162779191);
        let assign27770_body7_e38907: f64 = (locals.var_chi__blk816 * 6.36964918866352e-5);
        let assign27770_body7_e38908: f64 = (assign27770_body7_e38904 + assign27770_body7_e38907);
        let assign27770_body7_e38909: f64 = (locals.var_chi__blk816 * assign27770_body7_e38908);
        let assign27770_body7_e38910: f64 = (0.0178800506338833 + assign27770_body7_e38909);
        let assign27770_body7_e38911: f64 = (locals.var_chi__blk816 * assign27770_body7_e38910);
        let assign27770_body7_e38912: f64 = (assign27770_body7_e38899 + assign27770_body7_e38911);
        let assign27770_body7_e38913: f64 = (locals.var_chi__blk816 * assign27770_body7_e38912);
        let assign27770_body7_e38914: f64 = (0.707106781186548 + assign27770_body7_e38913);
        let assign27770_body7_e38915: f64 = (locals.var_chi__blk816 * assign27770_body7_e38914);
        (assign27770_body7_e38915, ((locals.var_chi__blk816_dn0 * assign27770_body7_e38914) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn0 * assign27770_body7_e38912) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn0 * assign27770_body7_e38910) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn0 * assign27770_body7_e38908) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk816_dn2 * assign27770_body7_e38914) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn2 * assign27770_body7_e38912) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn2 * assign27770_body7_e38910) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn2 * assign27770_body7_e38908) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk816_dn6 * assign27770_body7_e38914) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn6 * assign27770_body7_e38912) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn6 * assign27770_body7_e38910) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn6 * assign27770_body7_e38908) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk816_dn7 * assign27770_body7_e38914) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn7 * assign27770_body7_e38912) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn7 * assign27770_body7_e38910) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn7 * assign27770_body7_e38908) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk816_dn10 * assign27770_body7_e38914) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn10 * assign27770_body7_e38912) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn10 * assign27770_body7_e38910) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn10 * assign27770_body7_e38908) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk816_dn11 * assign27770_body7_e38914) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn11 * assign27770_body7_e38912) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn11 * assign27770_body7_e38910) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn11 * assign27770_body7_e38908) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk816_dn12 * assign27770_body7_e38914) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn12 * assign27770_body7_e38912) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn12 * assign27770_body7_e38910) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn12 * assign27770_body7_e38908) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn12 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk816_dn17 * assign27770_body7_e38914) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn17 * assign27770_body7_e38912) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn17 * assign27770_body7_e38910) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn17 * assign27770_body7_e38908) + (locals.var_chi__blk816 * (locals.var_chi__blk816_dn17 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb__blk840, locals.var_fb__blk840_dn0, locals.var_fb__blk840_dn2, locals.var_fb__blk840_dn6, locals.var_fb__blk840_dn7, locals.var_fb__blk840_dn10, locals.var_fb__blk840_dn11, locals.var_fb__blk840_dn12, locals.var_fb__blk840_dn17,)
    }
};
            locals.var_fb__blk840 = assign27770_body7_e38917;
            locals.var_fb__blk840_dn0 = assign27770_body7_e38917_d_n0;
            locals.var_fb__blk840_dn2 = assign27770_body7_e38917_d_n2;
            locals.var_fb__blk840_dn6 = assign27770_body7_e38917_d_n6;
            locals.var_fb__blk840_dn7 = assign27770_body7_e38917_d_n7;
            locals.var_fb__blk840_dn10 = assign27770_body7_e38917_d_n10;
            locals.var_fb__blk840_dn11 = assign27770_body7_e38917_d_n11;
            locals.var_fb__blk840_dn12 = assign27770_body7_e38917_d_n12;
            locals.var_fb__blk840_dn17 = assign27770_body7_e38917_d_n17;
            locals.var_fb__blk840_rv = 0.0;
            let (assign27770_body8_e38958, assign27770_body8_e38958_d_n0, assign27770_body8_e38958_d_n2, assign27770_body8_e38958_d_n6, assign27770_body8_e38958_d_n7, assign27770_body8_e38958_d_n10, assign27770_body8_e38958_d_n11, assign27770_body8_e38958_d_n12, assign27770_body8_e38958_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard880 != 0.0)) {
        let assign27770_body8_e38934: f64 = (-0.117851130197758);
        let assign27770_body8_e38935: f64 = (2.0 * assign27770_body8_e38934);
        let assign27770_body8_e38939: f64 = (3.0 * 0.0178800506338833);
        let assign27770_body8_e38943: f64 = (-0.00163730162779191);
        let assign27770_body8_e38944: f64 = (4.0 * assign27770_body8_e38943);
        let assign27770_body8_e38947: f64 = (locals.var_chi__blk816 * 5.0);
        let assign27770_body8_e38949: f64 = (assign27770_body8_e38947 * 6.36964918866352e-5);
        let assign27770_body8_e38950: f64 = (assign27770_body8_e38944 + assign27770_body8_e38949);
        let assign27770_body8_e38951: f64 = (locals.var_chi__blk816 * assign27770_body8_e38950);
        let assign27770_body8_e38952: f64 = (assign27770_body8_e38939 + assign27770_body8_e38951);
        let assign27770_body8_e38953: f64 = (locals.var_chi__blk816 * assign27770_body8_e38952);
        let assign27770_body8_e38954: f64 = (assign27770_body8_e38935 + assign27770_body8_e38953);
        let assign27770_body8_e38955: f64 = (locals.var_chi__blk816 * assign27770_body8_e38954);
        let assign27770_body8_e38956: f64 = (0.707106781186548 + assign27770_body8_e38955);
        (assign27770_body8_e38956, ((locals.var_chi__blk816_dn0 * assign27770_body8_e38954) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn0 * assign27770_body8_e38952) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn0 * assign27770_body8_e38950) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk816_dn2 * assign27770_body8_e38954) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn2 * assign27770_body8_e38952) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn2 * assign27770_body8_e38950) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk816_dn6 * assign27770_body8_e38954) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn6 * assign27770_body8_e38952) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn6 * assign27770_body8_e38950) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk816_dn7 * assign27770_body8_e38954) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn7 * assign27770_body8_e38952) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn7 * assign27770_body8_e38950) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk816_dn10 * assign27770_body8_e38954) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn10 * assign27770_body8_e38952) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn10 * assign27770_body8_e38950) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk816_dn11 * assign27770_body8_e38954) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn11 * assign27770_body8_e38952) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn11 * assign27770_body8_e38950) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk816_dn12 * assign27770_body8_e38954) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn12 * assign27770_body8_e38952) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn12 * assign27770_body8_e38950) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn12 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk816_dn17 * assign27770_body8_e38954) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn17 * assign27770_body8_e38952) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn17 * assign27770_body8_e38950) + (locals.var_chi__blk816 * ((locals.var_chi__blk816_dn17 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi, locals.var_fb_dchi_dn0, locals.var_fb_dchi_dn2, locals.var_fb_dchi_dn6, locals.var_fb_dchi_dn7, locals.var_fb_dchi_dn10, locals.var_fb_dchi_dn11, locals.var_fb_dchi_dn12, locals.var_fb_dchi_dn17,)
    }
};
            locals.var_fb_dchi = assign27770_body8_e38958;
            locals.var_fb_dchi_dn0 = assign27770_body8_e38958_d_n0;
            locals.var_fb_dchi_dn2 = assign27770_body8_e38958_d_n2;
            locals.var_fb_dchi_dn6 = assign27770_body8_e38958_d_n6;
            locals.var_fb_dchi_dn7 = assign27770_body8_e38958_d_n7;
            locals.var_fb_dchi_dn10 = assign27770_body8_e38958_d_n10;
            locals.var_fb_dchi_dn11 = assign27770_body8_e38958_d_n11;
            locals.var_fb_dchi_dn12 = assign27770_body8_e38958_d_n12;
            locals.var_fb_dchi_dn17 = assign27770_body8_e38958_d_n17;
            locals.var_fb_dchi_rv = 0.0;
            let (assign27770_body9_e38980, assign27770_body9_e38980_d_n0, assign27770_body9_e38980_d_n2, assign27770_body9_e38980_d_n6, assign27770_body9_e38980_d_n7, assign27770_body9_e38980_d_n10, assign27770_body9_e38980_d_n11, assign27770_body9_e38980_d_n12, assign27770_body9_e38980_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard880 != 0.0)) {
        let assign27770_body9_e38973: f64 = (locals.var_fb__blk840 * locals.var_fb__blk840);
        let assign27770_body9_e38975: f64 = (assign27770_body9_e38973 + locals.var_fs01__blk838);
        let assign27770_body9_e38977: f64 = (assign27770_body9_e38975 + 1e-50);
        let assign27770_body9_e38978: f64 = (assign27770_body9_e38977).sqrt();
        (assign27770_body9_e38978, ((((locals.var_fb__blk840_dn0 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn0)) + locals.var_fs01__blk838_dn0) / (2.0 * assign27770_body9_e38978)), ((((locals.var_fb__blk840_dn2 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn2)) + locals.var_fs01__blk838_dn2) / (2.0 * assign27770_body9_e38978)), ((((locals.var_fb__blk840_dn6 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn6)) + locals.var_fs01__blk838_dn6) / (2.0 * assign27770_body9_e38978)), ((((locals.var_fb__blk840_dn7 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn7)) + locals.var_fs01__blk838_dn7) / (2.0 * assign27770_body9_e38978)), ((((locals.var_fb__blk840_dn10 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn10)) + locals.var_fs01__blk838_dn10) / (2.0 * assign27770_body9_e38978)), ((((locals.var_fb__blk840_dn11 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn11)) + locals.var_fs01__blk838_dn11) / (2.0 * assign27770_body9_e38978)), ((((locals.var_fb__blk840_dn12 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn12)) + locals.var_fs01__blk838_dn12) / (2.0 * assign27770_body9_e38978)), ((((locals.var_fb__blk840_dn17 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn17)) + locals.var_fs01__blk838_dn17) / (2.0 * assign27770_body9_e38978)),)
    } else {
        (locals.var_fs02__blk842, locals.var_fs02__blk842_dn0, locals.var_fs02__blk842_dn2, locals.var_fs02__blk842_dn6, locals.var_fs02__blk842_dn7, locals.var_fs02__blk842_dn10, locals.var_fs02__blk842_dn11, locals.var_fs02__blk842_dn12, locals.var_fs02__blk842_dn17,)
    }
};
            locals.var_fs02__blk842 = assign27770_body9_e38980;
            locals.var_fs02__blk842_dn0 = assign27770_body9_e38980_d_n0;
            locals.var_fs02__blk842_dn2 = assign27770_body9_e38980_d_n2;
            locals.var_fs02__blk842_dn6 = assign27770_body9_e38980_d_n6;
            locals.var_fs02__blk842_dn7 = assign27770_body9_e38980_d_n7;
            locals.var_fs02__blk842_dn10 = assign27770_body9_e38980_d_n10;
            locals.var_fs02__blk842_dn11 = assign27770_body9_e38980_d_n11;
            locals.var_fs02__blk842_dn12 = assign27770_body9_e38980_d_n12;
            locals.var_fs02__blk842_dn17 = assign27770_body9_e38980_d_n17;
            locals.var_fs02__blk842_rv = 0.0;
            let (assign27770_body10_e39007, assign27770_body10_e39007_d_n0, assign27770_body10_e39007_d_n2, assign27770_body10_e39007_d_n6, assign27770_body10_e39007_d_n7, assign27770_body10_e39007_d_n10, assign27770_body10_e39007_d_n11, assign27770_body10_e39007_d_n12, assign27770_body10_e39007_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard880 != 0.0)) {
        let assign27770_body10_e38995: f64 = (locals.var_beta * locals.var_fb_dchi);
        let assign27770_body10_e38997: f64 = (assign27770_body10_e38995 * 2.0);
        let assign27770_body10_e38999: f64 = (assign27770_body10_e38997 * locals.var_fb__blk840);
        let assign27770_body10_e39001: f64 = (assign27770_body10_e38999 + locals.var_fs01_dps0__blk839);
        let assign27770_body10_e39004: f64 = (locals.var_fs02__blk842 + locals.var_fs02__blk842);
        let assign27770_body10_e39005: f64 = (assign27770_body10_e39001 / assign27770_body10_e39004);
        (assign27770_body10_e39005, ((((((((locals.var_beta * locals.var_fb_dchi_dn0) * 2.0) * locals.var_fb__blk840) + (assign27770_body10_e38997 * locals.var_fb__blk840_dn0)) + locals.var_fs01_dps0__blk839_dn0) * assign27770_body10_e39004) - (assign27770_body10_e39001 * (locals.var_fs02__blk842_dn0 + locals.var_fs02__blk842_dn0))) / (assign27770_body10_e39004 * assign27770_body10_e39004)), ((((((((locals.var_beta * locals.var_fb_dchi_dn2) * 2.0) * locals.var_fb__blk840) + (assign27770_body10_e38997 * locals.var_fb__blk840_dn2)) + locals.var_fs01_dps0__blk839_dn2) * assign27770_body10_e39004) - (assign27770_body10_e39001 * (locals.var_fs02__blk842_dn2 + locals.var_fs02__blk842_dn2))) / (assign27770_body10_e39004 * assign27770_body10_e39004)), ((((((((locals.var_beta * locals.var_fb_dchi_dn6) * 2.0) * locals.var_fb__blk840) + (assign27770_body10_e38997 * locals.var_fb__blk840_dn6)) + locals.var_fs01_dps0__blk839_dn6) * assign27770_body10_e39004) - (assign27770_body10_e39001 * (locals.var_fs02__blk842_dn6 + locals.var_fs02__blk842_dn6))) / (assign27770_body10_e39004 * assign27770_body10_e39004)), ((((((((locals.var_beta * locals.var_fb_dchi_dn7) * 2.0) * locals.var_fb__blk840) + (assign27770_body10_e38997 * locals.var_fb__blk840_dn7)) + locals.var_fs01_dps0__blk839_dn7) * assign27770_body10_e39004) - (assign27770_body10_e39001 * (locals.var_fs02__blk842_dn7 + locals.var_fs02__blk842_dn7))) / (assign27770_body10_e39004 * assign27770_body10_e39004)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn10)) * 2.0) * locals.var_fb__blk840) + (assign27770_body10_e38997 * locals.var_fb__blk840_dn10)) + locals.var_fs01_dps0__blk839_dn10) * assign27770_body10_e39004) - (assign27770_body10_e39001 * (locals.var_fs02__blk842_dn10 + locals.var_fs02__blk842_dn10))) / (assign27770_body10_e39004 * assign27770_body10_e39004)), ((((((((locals.var_beta * locals.var_fb_dchi_dn11) * 2.0) * locals.var_fb__blk840) + (assign27770_body10_e38997 * locals.var_fb__blk840_dn11)) + locals.var_fs01_dps0__blk839_dn11) * assign27770_body10_e39004) - (assign27770_body10_e39001 * (locals.var_fs02__blk842_dn11 + locals.var_fs02__blk842_dn11))) / (assign27770_body10_e39004 * assign27770_body10_e39004)), ((((((((locals.var_beta * locals.var_fb_dchi_dn12) * 2.0) * locals.var_fb__blk840) + (assign27770_body10_e38997 * locals.var_fb__blk840_dn12)) + locals.var_fs01_dps0__blk839_dn12) * assign27770_body10_e39004) - (assign27770_body10_e39001 * (locals.var_fs02__blk842_dn12 + locals.var_fs02__blk842_dn12))) / (assign27770_body10_e39004 * assign27770_body10_e39004)), ((((((((locals.var_beta * locals.var_fb_dchi_dn17) * 2.0) * locals.var_fb__blk840) + (assign27770_body10_e38997 * locals.var_fb__blk840_dn17)) + locals.var_fs01_dps0__blk839_dn17) * assign27770_body10_e39004) - (assign27770_body10_e39001 * (locals.var_fs02__blk842_dn17 + locals.var_fs02__blk842_dn17))) / (assign27770_body10_e39004 * assign27770_body10_e39004)),)
    } else {
        (locals.var_fs02_dps0__blk843, locals.var_fs02_dps0__blk843_dn0, locals.var_fs02_dps0__blk843_dn2, locals.var_fs02_dps0__blk843_dn6, locals.var_fs02_dps0__blk843_dn7, locals.var_fs02_dps0__blk843_dn10, locals.var_fs02_dps0__blk843_dn11, locals.var_fs02_dps0__blk843_dn12, locals.var_fs02_dps0__blk843_dn17,)
    }
};
            locals.var_fs02_dps0__blk843 = assign27770_body10_e39007;
            locals.var_fs02_dps0__blk843_dn0 = assign27770_body10_e39007_d_n0;
            locals.var_fs02_dps0__blk843_dn2 = assign27770_body10_e39007_d_n2;
            locals.var_fs02_dps0__blk843_dn6 = assign27770_body10_e39007_d_n6;
            locals.var_fs02_dps0__blk843_dn7 = assign27770_body10_e39007_d_n7;
            locals.var_fs02_dps0__blk843_dn10 = assign27770_body10_e39007_d_n10;
            locals.var_fs02_dps0__blk843_dn11 = assign27770_body10_e39007_d_n11;
            locals.var_fs02_dps0__blk843_dn12 = assign27770_body10_e39007_d_n12;
            locals.var_fs02_dps0__blk843_dn17 = assign27770_body10_e39007_d_n17;
            locals.var_fs02_dps0__blk843_rv = 0.0;
            let assign27770_body11_e39010: f64 = if locals.var_chi__blk816 < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard881 = assign27770_body11_e39010;
            locals.var_guard881_rv = 0.0;
            let (assign27770_body12_e39029, assign27770_body12_e39029_d_n0, assign27770_body12_e39029_d_n2, assign27770_body12_e39029_d_n6, assign27770_body12_e39029_d_n7, assign27770_body12_e39029_d_n10, assign27770_body12_e39029_d_n11, assign27770_body12_e39029_d_n12, assign27770_body12_e39029_d_n17,) = {
    if (((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard880 == 0.0)) && (locals.var_guard881 != 0.0)) {
        let assign27770_body12_e39027: f64 = (locals.var_chi__blk816).exp();
        (assign27770_body12_e39027, (assign27770_body12_e39027 * locals.var_chi__blk816_dn0), (assign27770_body12_e39027 * locals.var_chi__blk816_dn2), (assign27770_body12_e39027 * locals.var_chi__blk816_dn6), (assign27770_body12_e39027 * locals.var_chi__blk816_dn7), (assign27770_body12_e39027 * locals.var_chi__blk816_dn10), (assign27770_body12_e39027 * locals.var_chi__blk816_dn11), (assign27770_body12_e39027 * locals.var_chi__blk816_dn12), (assign27770_body12_e39027 * locals.var_chi__blk816_dn17),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17,)
    }
};
            locals.var_exp_chi = assign27770_body12_e39029;
            locals.var_exp_chi_dn0 = assign27770_body12_e39029_d_n0;
            locals.var_exp_chi_dn2 = assign27770_body12_e39029_d_n2;
            locals.var_exp_chi_dn6 = assign27770_body12_e39029_d_n6;
            locals.var_exp_chi_dn7 = assign27770_body12_e39029_d_n7;
            locals.var_exp_chi_dn10 = assign27770_body12_e39029_d_n10;
            locals.var_exp_chi_dn11 = assign27770_body12_e39029_d_n11;
            locals.var_exp_chi_dn12 = assign27770_body12_e39029_d_n12;
            locals.var_exp_chi_dn17 = assign27770_body12_e39029_d_n17;
            locals.var_exp_chi_rv = 0.0;
            let (assign27770_body13_e39051, assign27770_body13_e39051_d_n0, assign27770_body13_e39051_d_n2, assign27770_body13_e39051_d_n6, assign27770_body13_e39051_d_n7, assign27770_body13_e39051_d_n10, assign27770_body13_e39051_d_n11, assign27770_body13_e39051_d_n12, assign27770_body13_e39051_d_n17,) = {
    if (((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard880 == 0.0)) && (locals.var_guard881 != 0.0)) {
        let assign27770_body13_e39048: f64 = (locals.var_exp_chi - 1.0);
        let assign27770_body13_e39049: f64 = (locals.var_cfs1__blk844 * assign27770_body13_e39048);
        (assign27770_body13_e39049, ((locals.var_cfs1__blk844_dn0 * assign27770_body13_e39048) + (locals.var_cfs1__blk844 * locals.var_exp_chi_dn0)), ((locals.var_cfs1__blk844_dn2 * assign27770_body13_e39048) + (locals.var_cfs1__blk844 * locals.var_exp_chi_dn2)), ((locals.var_cfs1__blk844_dn6 * assign27770_body13_e39048) + (locals.var_cfs1__blk844 * locals.var_exp_chi_dn6)), ((locals.var_cfs1__blk844_dn7 * assign27770_body13_e39048) + (locals.var_cfs1__blk844 * locals.var_exp_chi_dn7)), ((locals.var_cfs1__blk844_dn10 * assign27770_body13_e39048) + (locals.var_cfs1__blk844 * locals.var_exp_chi_dn10)), ((locals.var_cfs1__blk844_dn11 * assign27770_body13_e39048) + (locals.var_cfs1__blk844 * locals.var_exp_chi_dn11)), ((locals.var_cfs1__blk844_dn12 * assign27770_body13_e39048) + (locals.var_cfs1__blk844 * locals.var_exp_chi_dn12)), ((locals.var_cfs1__blk844_dn17 * assign27770_body13_e39048) + (locals.var_cfs1__blk844 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01__blk838, locals.var_fs01__blk838_dn0, locals.var_fs01__blk838_dn2, locals.var_fs01__blk838_dn6, locals.var_fs01__blk838_dn7, locals.var_fs01__blk838_dn10, locals.var_fs01__blk838_dn11, locals.var_fs01__blk838_dn12, locals.var_fs01__blk838_dn17,)
    }
};
            locals.var_fs01__blk838 = assign27770_body13_e39051;
            locals.var_fs01__blk838_dn0 = assign27770_body13_e39051_d_n0;
            locals.var_fs01__blk838_dn2 = assign27770_body13_e39051_d_n2;
            locals.var_fs01__blk838_dn6 = assign27770_body13_e39051_d_n6;
            locals.var_fs01__blk838_dn7 = assign27770_body13_e39051_d_n7;
            locals.var_fs01__blk838_dn10 = assign27770_body13_e39051_d_n10;
            locals.var_fs01__blk838_dn11 = assign27770_body13_e39051_d_n11;
            locals.var_fs01__blk838_dn12 = assign27770_body13_e39051_d_n12;
            locals.var_fs01__blk838_dn17 = assign27770_body13_e39051_d_n17;
            locals.var_fs01__blk838_rv = 0.0;
            let (assign27770_body14_e39073, assign27770_body14_e39073_d_n0, assign27770_body14_e39073_d_n2, assign27770_body14_e39073_d_n6, assign27770_body14_e39073_d_n7, assign27770_body14_e39073_d_n10, assign27770_body14_e39073_d_n11, assign27770_body14_e39073_d_n12, assign27770_body14_e39073_d_n17,) = {
    if (((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard880 == 0.0)) && (locals.var_guard881 != 0.0)) {
        let assign27770_body14_e39069: f64 = (locals.var_cfs1__blk844 * locals.var_beta);
        let assign27770_body14_e39071: f64 = (assign27770_body14_e39069 * locals.var_exp_chi);
        (assign27770_body14_e39071, (((locals.var_cfs1__blk844_dn0 * locals.var_beta) * locals.var_exp_chi) + (assign27770_body14_e39069 * locals.var_exp_chi_dn0)), (((locals.var_cfs1__blk844_dn2 * locals.var_beta) * locals.var_exp_chi) + (assign27770_body14_e39069 * locals.var_exp_chi_dn2)), (((locals.var_cfs1__blk844_dn6 * locals.var_beta) * locals.var_exp_chi) + (assign27770_body14_e39069 * locals.var_exp_chi_dn6)), (((locals.var_cfs1__blk844_dn7 * locals.var_beta) * locals.var_exp_chi) + (assign27770_body14_e39069 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1__blk844_dn10 * locals.var_beta) + (locals.var_cfs1__blk844 * locals.var_beta_dn10)) * locals.var_exp_chi) + (assign27770_body14_e39069 * locals.var_exp_chi_dn10)), (((locals.var_cfs1__blk844_dn11 * locals.var_beta) * locals.var_exp_chi) + (assign27770_body14_e39069 * locals.var_exp_chi_dn11)), (((locals.var_cfs1__blk844_dn12 * locals.var_beta) * locals.var_exp_chi) + (assign27770_body14_e39069 * locals.var_exp_chi_dn12)), (((locals.var_cfs1__blk844_dn17 * locals.var_beta) * locals.var_exp_chi) + (assign27770_body14_e39069 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk839, locals.var_fs01_dps0__blk839_dn0, locals.var_fs01_dps0__blk839_dn2, locals.var_fs01_dps0__blk839_dn6, locals.var_fs01_dps0__blk839_dn7, locals.var_fs01_dps0__blk839_dn10, locals.var_fs01_dps0__blk839_dn11, locals.var_fs01_dps0__blk839_dn12, locals.var_fs01_dps0__blk839_dn17,)
    }
};
            locals.var_fs01_dps0__blk839 = assign27770_body14_e39073;
            locals.var_fs01_dps0__blk839_dn0 = assign27770_body14_e39073_d_n0;
            locals.var_fs01_dps0__blk839_dn2 = assign27770_body14_e39073_d_n2;
            locals.var_fs01_dps0__blk839_dn6 = assign27770_body14_e39073_d_n6;
            locals.var_fs01_dps0__blk839_dn7 = assign27770_body14_e39073_d_n7;
            locals.var_fs01_dps0__blk839_dn10 = assign27770_body14_e39073_d_n10;
            locals.var_fs01_dps0__blk839_dn11 = assign27770_body14_e39073_d_n11;
            locals.var_fs01_dps0__blk839_dn12 = assign27770_body14_e39073_d_n12;
            locals.var_fs01_dps0__blk839_dn17 = assign27770_body14_e39073_d_n17;
            locals.var_fs01_dps0__blk839_rv = 0.0;
            let (assign27770_body15_e39095, assign27770_body15_e39095_d_n0, assign27770_body15_e39095_d_n2, assign27770_body15_e39095_d_n6, assign27770_body15_e39095_d_n7, assign27770_body15_e39095_d_n10, assign27770_body15_e39095_d_n11, assign27770_body15_e39095_d_n12, assign27770_body15_e39095_d_n17,) = {
    if (((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard880 == 0.0)) && (locals.var_guard881 == 0.0)) {
        let assign27770_body15_e39092: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign27770_body15_e39093: f64 = (assign27770_body15_e39092).exp();
        (assign27770_body15_e39093, (assign27770_body15_e39093 * (locals.var_beta * locals.var_ps0ld_dn0)), (assign27770_body15_e39093 * (locals.var_beta * locals.var_ps0ld_dn2)), (assign27770_body15_e39093 * (locals.var_beta * locals.var_ps0ld_dn6)), (assign27770_body15_e39093 * (locals.var_beta * locals.var_ps0ld_dn7)), (assign27770_body15_e39093 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign27770_body15_e39093 * (locals.var_beta * locals.var_ps0ld_dn11)), (assign27770_body15_e39093 * (locals.var_beta * locals.var_ps0ld_dn12)), (assign27770_body15_e39093 * (locals.var_beta * locals.var_ps0ld_dn17)),)
    } else {
        (locals.var_exp_bps0__blk845, locals.var_exp_bps0__blk845_dn0, locals.var_exp_bps0__blk845_dn2, locals.var_exp_bps0__blk845_dn6, locals.var_exp_bps0__blk845_dn7, locals.var_exp_bps0__blk845_dn10, locals.var_exp_bps0__blk845_dn11, locals.var_exp_bps0__blk845_dn12, locals.var_exp_bps0__blk845_dn17,)
    }
};
            locals.var_exp_bps0__blk845 = assign27770_body15_e39095;
            locals.var_exp_bps0__blk845_dn0 = assign27770_body15_e39095_d_n0;
            locals.var_exp_bps0__blk845_dn2 = assign27770_body15_e39095_d_n2;
            locals.var_exp_bps0__blk845_dn6 = assign27770_body15_e39095_d_n6;
            locals.var_exp_bps0__blk845_dn7 = assign27770_body15_e39095_d_n7;
            locals.var_exp_bps0__blk845_dn10 = assign27770_body15_e39095_d_n10;
            locals.var_exp_bps0__blk845_dn11 = assign27770_body15_e39095_d_n11;
            locals.var_exp_bps0__blk845_dn12 = assign27770_body15_e39095_d_n12;
            locals.var_exp_bps0__blk845_dn17 = assign27770_body15_e39095_d_n17;
            locals.var_exp_bps0__blk845_rv = 0.0;
            let (assign27770_body16_e39118, assign27770_body16_e39118_d_n0, assign27770_body16_e39118_d_n2, assign27770_body16_e39118_d_n6, assign27770_body16_e39118_d_n7, assign27770_body16_e39118_d_n10, assign27770_body16_e39118_d_n11, assign27770_body16_e39118_d_n12, assign27770_body16_e39118_d_n17,) = {
    if (((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard880 == 0.0)) && (locals.var_guard881 == 0.0)) {
        let assign27770_body16_e39115: f64 = (locals.var_exp_bps0__blk845 - locals.var_exp_bvbs__blk835);
        let assign27770_body16_e39116: f64 = (locals.var_cnst1over * assign27770_body16_e39115);
        (assign27770_body16_e39116, ((locals.var_cnst1over_dn0 * assign27770_body16_e39115) + (locals.var_cnst1over * (locals.var_exp_bps0__blk845_dn0 - locals.var_exp_bvbs__blk835_dn0))), ((locals.var_cnst1over_dn2 * assign27770_body16_e39115) + (locals.var_cnst1over * (locals.var_exp_bps0__blk845_dn2 - locals.var_exp_bvbs__blk835_dn2))), ((locals.var_cnst1over_dn6 * assign27770_body16_e39115) + (locals.var_cnst1over * (locals.var_exp_bps0__blk845_dn6 - locals.var_exp_bvbs__blk835_dn6))), ((locals.var_cnst1over_dn7 * assign27770_body16_e39115) + (locals.var_cnst1over * (locals.var_exp_bps0__blk845_dn7 - locals.var_exp_bvbs__blk835_dn7))), ((locals.var_cnst1over_dn10 * assign27770_body16_e39115) + (locals.var_cnst1over * (locals.var_exp_bps0__blk845_dn10 - locals.var_exp_bvbs__blk835_dn10))), ((locals.var_cnst1over_dn11 * assign27770_body16_e39115) + (locals.var_cnst1over * (locals.var_exp_bps0__blk845_dn11 - locals.var_exp_bvbs__blk835_dn11))), ((locals.var_cnst1over_dn12 * assign27770_body16_e39115) + (locals.var_cnst1over * (locals.var_exp_bps0__blk845_dn12 - locals.var_exp_bvbs__blk835_dn12))), ((locals.var_cnst1over_dn17 * assign27770_body16_e39115) + (locals.var_cnst1over * (locals.var_exp_bps0__blk845_dn17 - locals.var_exp_bvbs__blk835_dn17))),)
    } else {
        (locals.var_fs01__blk838, locals.var_fs01__blk838_dn0, locals.var_fs01__blk838_dn2, locals.var_fs01__blk838_dn6, locals.var_fs01__blk838_dn7, locals.var_fs01__blk838_dn10, locals.var_fs01__blk838_dn11, locals.var_fs01__blk838_dn12, locals.var_fs01__blk838_dn17,)
    }
};
            locals.var_fs01__blk838 = assign27770_body16_e39118;
            locals.var_fs01__blk838_dn0 = assign27770_body16_e39118_d_n0;
            locals.var_fs01__blk838_dn2 = assign27770_body16_e39118_d_n2;
            locals.var_fs01__blk838_dn6 = assign27770_body16_e39118_d_n6;
            locals.var_fs01__blk838_dn7 = assign27770_body16_e39118_d_n7;
            locals.var_fs01__blk838_dn10 = assign27770_body16_e39118_d_n10;
            locals.var_fs01__blk838_dn11 = assign27770_body16_e39118_d_n11;
            locals.var_fs01__blk838_dn12 = assign27770_body16_e39118_d_n12;
            locals.var_fs01__blk838_dn17 = assign27770_body16_e39118_d_n17;
            locals.var_fs01__blk838_rv = 0.0;
            let (assign27770_body17_e39141, assign27770_body17_e39141_d_n0, assign27770_body17_e39141_d_n2, assign27770_body17_e39141_d_n6, assign27770_body17_e39141_d_n7, assign27770_body17_e39141_d_n10, assign27770_body17_e39141_d_n11, assign27770_body17_e39141_d_n12, assign27770_body17_e39141_d_n17,) = {
    if (((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard880 == 0.0)) && (locals.var_guard881 == 0.0)) {
        let assign27770_body17_e39137: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign27770_body17_e39139: f64 = (assign27770_body17_e39137 * locals.var_exp_bps0__blk845);
        (assign27770_body17_e39139, (((locals.var_cnst1over_dn0 * locals.var_beta) * locals.var_exp_bps0__blk845) + (assign27770_body17_e39137 * locals.var_exp_bps0__blk845_dn0)), (((locals.var_cnst1over_dn2 * locals.var_beta) * locals.var_exp_bps0__blk845) + (assign27770_body17_e39137 * locals.var_exp_bps0__blk845_dn2)), (((locals.var_cnst1over_dn6 * locals.var_beta) * locals.var_exp_bps0__blk845) + (assign27770_body17_e39137 * locals.var_exp_bps0__blk845_dn6)), (((locals.var_cnst1over_dn7 * locals.var_beta) * locals.var_exp_bps0__blk845) + (assign27770_body17_e39137 * locals.var_exp_bps0__blk845_dn7)), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * locals.var_exp_bps0__blk845) + (assign27770_body17_e39137 * locals.var_exp_bps0__blk845_dn10)), (((locals.var_cnst1over_dn11 * locals.var_beta) * locals.var_exp_bps0__blk845) + (assign27770_body17_e39137 * locals.var_exp_bps0__blk845_dn11)), (((locals.var_cnst1over_dn12 * locals.var_beta) * locals.var_exp_bps0__blk845) + (assign27770_body17_e39137 * locals.var_exp_bps0__blk845_dn12)), (((locals.var_cnst1over_dn17 * locals.var_beta) * locals.var_exp_bps0__blk845) + (assign27770_body17_e39137 * locals.var_exp_bps0__blk845_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk839, locals.var_fs01_dps0__blk839_dn0, locals.var_fs01_dps0__blk839_dn2, locals.var_fs01_dps0__blk839_dn6, locals.var_fs01_dps0__blk839_dn7, locals.var_fs01_dps0__blk839_dn10, locals.var_fs01_dps0__blk839_dn11, locals.var_fs01_dps0__blk839_dn12, locals.var_fs01_dps0__blk839_dn17,)
    }
};
            locals.var_fs01_dps0__blk839 = assign27770_body17_e39141;
            locals.var_fs01_dps0__blk839_dn0 = assign27770_body17_e39141_d_n0;
            locals.var_fs01_dps0__blk839_dn2 = assign27770_body17_e39141_d_n2;
            locals.var_fs01_dps0__blk839_dn6 = assign27770_body17_e39141_d_n6;
            locals.var_fs01_dps0__blk839_dn7 = assign27770_body17_e39141_d_n7;
            locals.var_fs01_dps0__blk839_dn10 = assign27770_body17_e39141_d_n10;
            locals.var_fs01_dps0__blk839_dn11 = assign27770_body17_e39141_d_n11;
            locals.var_fs01_dps0__blk839_dn12 = assign27770_body17_e39141_d_n12;
            locals.var_fs01_dps0__blk839_dn17 = assign27770_body17_e39141_d_n17;
            locals.var_fs01_dps0__blk839_rv = 0.0;
            let (assign27770_body18_e39162, assign27770_body18_e39162_d_n0, assign27770_body18_e39162_d_n2, assign27770_body18_e39162_d_n6, assign27770_body18_e39162_d_n7, assign27770_body18_e39162_d_n10, assign27770_body18_e39162_d_n11, assign27770_body18_e39162_d_n12, assign27770_body18_e39162_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard880 == 0.0)) {
        let assign27770_body18_e39157: f64 = (locals.var_chi__blk816 - 1.0);
        let assign27770_body18_e39159: f64 = (assign27770_body18_e39157 + locals.var_fs01__blk838);
        let assign27770_body18_e39160: f64 = (assign27770_body18_e39159).sqrt();
        (assign27770_body18_e39160, ((locals.var_chi__blk816_dn0 + locals.var_fs01__blk838_dn0) / (2.0 * assign27770_body18_e39160)), ((locals.var_chi__blk816_dn2 + locals.var_fs01__blk838_dn2) / (2.0 * assign27770_body18_e39160)), ((locals.var_chi__blk816_dn6 + locals.var_fs01__blk838_dn6) / (2.0 * assign27770_body18_e39160)), ((locals.var_chi__blk816_dn7 + locals.var_fs01__blk838_dn7) / (2.0 * assign27770_body18_e39160)), ((locals.var_chi__blk816_dn10 + locals.var_fs01__blk838_dn10) / (2.0 * assign27770_body18_e39160)), ((locals.var_chi__blk816_dn11 + locals.var_fs01__blk838_dn11) / (2.0 * assign27770_body18_e39160)), ((locals.var_chi__blk816_dn12 + locals.var_fs01__blk838_dn12) / (2.0 * assign27770_body18_e39160)), ((locals.var_chi__blk816_dn17 + locals.var_fs01__blk838_dn17) / (2.0 * assign27770_body18_e39160)),)
    } else {
        (locals.var_fs02__blk842, locals.var_fs02__blk842_dn0, locals.var_fs02__blk842_dn2, locals.var_fs02__blk842_dn6, locals.var_fs02__blk842_dn7, locals.var_fs02__blk842_dn10, locals.var_fs02__blk842_dn11, locals.var_fs02__blk842_dn12, locals.var_fs02__blk842_dn17,)
    }
};
            locals.var_fs02__blk842 = assign27770_body18_e39162;
            locals.var_fs02__blk842_dn0 = assign27770_body18_e39162_d_n0;
            locals.var_fs02__blk842_dn2 = assign27770_body18_e39162_d_n2;
            locals.var_fs02__blk842_dn6 = assign27770_body18_e39162_d_n6;
            locals.var_fs02__blk842_dn7 = assign27770_body18_e39162_d_n7;
            locals.var_fs02__blk842_dn10 = assign27770_body18_e39162_d_n10;
            locals.var_fs02__blk842_dn11 = assign27770_body18_e39162_d_n11;
            locals.var_fs02__blk842_dn12 = assign27770_body18_e39162_d_n12;
            locals.var_fs02__blk842_dn17 = assign27770_body18_e39162_d_n17;
            locals.var_fs02__blk842_rv = 0.0;
            let (assign27770_body19_e39184, assign27770_body19_e39184_d_n0, assign27770_body19_e39184_d_n2, assign27770_body19_e39184_d_n6, assign27770_body19_e39184_d_n7, assign27770_body19_e39184_d_n10, assign27770_body19_e39184_d_n11, assign27770_body19_e39184_d_n12, assign27770_body19_e39184_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard880 == 0.0)) {
        let assign27770_body19_e39178: f64 = (locals.var_beta + locals.var_fs01_dps0__blk839);
        let assign27770_body19_e39180: f64 = (assign27770_body19_e39178 / locals.var_fs02__blk842);
        let assign27770_body19_e39182: f64 = (assign27770_body19_e39180 * 0.5);
        (assign27770_body19_e39182, ((((locals.var_fs01_dps0__blk839_dn0 * locals.var_fs02__blk842) - (assign27770_body19_e39178 * locals.var_fs02__blk842_dn0)) / (locals.var_fs02__blk842 * locals.var_fs02__blk842)) * 0.5), ((((locals.var_fs01_dps0__blk839_dn2 * locals.var_fs02__blk842) - (assign27770_body19_e39178 * locals.var_fs02__blk842_dn2)) / (locals.var_fs02__blk842 * locals.var_fs02__blk842)) * 0.5), ((((locals.var_fs01_dps0__blk839_dn6 * locals.var_fs02__blk842) - (assign27770_body19_e39178 * locals.var_fs02__blk842_dn6)) / (locals.var_fs02__blk842 * locals.var_fs02__blk842)) * 0.5), ((((locals.var_fs01_dps0__blk839_dn7 * locals.var_fs02__blk842) - (assign27770_body19_e39178 * locals.var_fs02__blk842_dn7)) / (locals.var_fs02__blk842 * locals.var_fs02__blk842)) * 0.5), (((((locals.var_beta_dn10 + locals.var_fs01_dps0__blk839_dn10) * locals.var_fs02__blk842) - (assign27770_body19_e39178 * locals.var_fs02__blk842_dn10)) / (locals.var_fs02__blk842 * locals.var_fs02__blk842)) * 0.5), ((((locals.var_fs01_dps0__blk839_dn11 * locals.var_fs02__blk842) - (assign27770_body19_e39178 * locals.var_fs02__blk842_dn11)) / (locals.var_fs02__blk842 * locals.var_fs02__blk842)) * 0.5), ((((locals.var_fs01_dps0__blk839_dn12 * locals.var_fs02__blk842) - (assign27770_body19_e39178 * locals.var_fs02__blk842_dn12)) / (locals.var_fs02__blk842 * locals.var_fs02__blk842)) * 0.5), ((((locals.var_fs01_dps0__blk839_dn17 * locals.var_fs02__blk842) - (assign27770_body19_e39178 * locals.var_fs02__blk842_dn17)) / (locals.var_fs02__blk842 * locals.var_fs02__blk842)) * 0.5),)
    } else {
        (locals.var_fs02_dps0__blk843, locals.var_fs02_dps0__blk843_dn0, locals.var_fs02_dps0__blk843_dn2, locals.var_fs02_dps0__blk843_dn6, locals.var_fs02_dps0__blk843_dn7, locals.var_fs02_dps0__blk843_dn10, locals.var_fs02_dps0__blk843_dn11, locals.var_fs02_dps0__blk843_dn12, locals.var_fs02_dps0__blk843_dn17,)
    }
};
            locals.var_fs02_dps0__blk843 = assign27770_body19_e39184;
            locals.var_fs02_dps0__blk843_dn0 = assign27770_body19_e39184_d_n0;
            locals.var_fs02_dps0__blk843_dn2 = assign27770_body19_e39184_d_n2;
            locals.var_fs02_dps0__blk843_dn6 = assign27770_body19_e39184_d_n6;
            locals.var_fs02_dps0__blk843_dn7 = assign27770_body19_e39184_d_n7;
            locals.var_fs02_dps0__blk843_dn10 = assign27770_body19_e39184_d_n10;
            locals.var_fs02_dps0__blk843_dn11 = assign27770_body19_e39184_d_n11;
            locals.var_fs02_dps0__blk843_dn12 = assign27770_body19_e39184_d_n12;
            locals.var_fs02_dps0__blk843_dn17 = assign27770_body19_e39184_d_n17;
            locals.var_fs02_dps0__blk843_rv = 0.0;
            let (assign27770_body20_e39203, assign27770_body20_e39203_d_n0, assign27770_body20_e39203_d_n2, assign27770_body20_e39203_d_n6, assign27770_body20_e39203_d_n7, assign27770_body20_e39203_d_n10, assign27770_body20_e39203_d_n11, assign27770_body20_e39203_d_n12, assign27770_body20_e39203_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27770_body20_e39197: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign27770_body20_e39200: f64 = (locals.var_fac1__blk802 * locals.var_fs02__blk842);
        let assign27770_body20_e39201: f64 = (assign27770_body20_e39197 - assign27770_body20_e39200);
        (assign27770_body20_e39201, ((locals.var_vgpld_dn0 - locals.var_ps0ld_dn0) - ((locals.var_fac1__blk802_dn0 * locals.var_fs02__blk842) + (locals.var_fac1__blk802 * locals.var_fs02__blk842_dn0))), ((locals.var_vgpld_dn2 - locals.var_ps0ld_dn2) - ((locals.var_fac1__blk802_dn2 * locals.var_fs02__blk842) + (locals.var_fac1__blk802 * locals.var_fs02__blk842_dn2))), ((locals.var_vgpld_dn6 - locals.var_ps0ld_dn6) - ((locals.var_fac1__blk802_dn6 * locals.var_fs02__blk842) + (locals.var_fac1__blk802 * locals.var_fs02__blk842_dn6))), ((locals.var_vgpld_dn7 - locals.var_ps0ld_dn7) - ((locals.var_fac1__blk802_dn7 * locals.var_fs02__blk842) + (locals.var_fac1__blk802 * locals.var_fs02__blk842_dn7))), ((locals.var_vgpld_dn10 - locals.var_ps0ld_dn10) - ((locals.var_fac1__blk802_dn10 * locals.var_fs02__blk842) + (locals.var_fac1__blk802 * locals.var_fs02__blk842_dn10))), ((locals.var_vgpld_dn11 - locals.var_ps0ld_dn11) - ((locals.var_fac1__blk802_dn11 * locals.var_fs02__blk842) + (locals.var_fac1__blk802 * locals.var_fs02__blk842_dn11))), ((locals.var_vgpld_dn12 - locals.var_ps0ld_dn12) - ((locals.var_fac1__blk802_dn12 * locals.var_fs02__blk842) + (locals.var_fac1__blk802 * locals.var_fs02__blk842_dn12))), ((locals.var_vgpld_dn17 - locals.var_ps0ld_dn17) - ((locals.var_fac1__blk802_dn17 * locals.var_fs02__blk842) + (locals.var_fac1__blk802 * locals.var_fs02__blk842_dn17))),)
    } else {
        (locals.var_fs0__blk846, locals.var_fs0__blk846_dn0, locals.var_fs0__blk846_dn2, locals.var_fs0__blk846_dn6, locals.var_fs0__blk846_dn7, locals.var_fs0__blk846_dn10, locals.var_fs0__blk846_dn11, locals.var_fs0__blk846_dn12, locals.var_fs0__blk846_dn17,)
    }
};
            locals.var_fs0__blk846 = assign27770_body20_e39203;
            locals.var_fs0__blk846_dn0 = assign27770_body20_e39203_d_n0;
            locals.var_fs0__blk846_dn2 = assign27770_body20_e39203_d_n2;
            locals.var_fs0__blk846_dn6 = assign27770_body20_e39203_d_n6;
            locals.var_fs0__blk846_dn7 = assign27770_body20_e39203_d_n7;
            locals.var_fs0__blk846_dn10 = assign27770_body20_e39203_d_n10;
            locals.var_fs0__blk846_dn11 = assign27770_body20_e39203_d_n11;
            locals.var_fs0__blk846_dn12 = assign27770_body20_e39203_d_n12;
            locals.var_fs0__blk846_dn17 = assign27770_body20_e39203_d_n17;
            locals.var_fs0__blk846_rv = 0.0;
            let (assign27770_body21_e39221, assign27770_body21_e39221_d_n0, assign27770_body21_e39221_d_n2, assign27770_body21_e39221_d_n6, assign27770_body21_e39221_d_n7, assign27770_body21_e39221_d_n10, assign27770_body21_e39221_d_n11, assign27770_body21_e39221_d_n12, assign27770_body21_e39221_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27770_body21_e39215: f64 = (-1.0);
        let assign27770_body21_e39218: f64 = (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843);
        let assign27770_body21_e39219: f64 = (assign27770_body21_e39215 - assign27770_body21_e39218);
        (assign27770_body21_e39219, (-((locals.var_fac1__blk802_dn0 * locals.var_fs02_dps0__blk843) + (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843_dn0))), (-((locals.var_fac1__blk802_dn2 * locals.var_fs02_dps0__blk843) + (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843_dn2))), (-((locals.var_fac1__blk802_dn6 * locals.var_fs02_dps0__blk843) + (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843_dn6))), (-((locals.var_fac1__blk802_dn7 * locals.var_fs02_dps0__blk843) + (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843_dn7))), (-((locals.var_fac1__blk802_dn10 * locals.var_fs02_dps0__blk843) + (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843_dn10))), (-((locals.var_fac1__blk802_dn11 * locals.var_fs02_dps0__blk843) + (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843_dn11))), (-((locals.var_fac1__blk802_dn12 * locals.var_fs02_dps0__blk843) + (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843_dn12))), (-((locals.var_fac1__blk802_dn17 * locals.var_fs02_dps0__blk843) + (locals.var_fac1__blk802 * locals.var_fs02_dps0__blk843_dn17))),)
    } else {
        (locals.var_fs0_dps0__blk847, locals.var_fs0_dps0__blk847_dn0, locals.var_fs0_dps0__blk847_dn2, locals.var_fs0_dps0__blk847_dn6, locals.var_fs0_dps0__blk847_dn7, locals.var_fs0_dps0__blk847_dn10, locals.var_fs0_dps0__blk847_dn11, locals.var_fs0_dps0__blk847_dn12, locals.var_fs0_dps0__blk847_dn17,)
    }
};
            locals.var_fs0_dps0__blk847 = assign27770_body21_e39221;
            locals.var_fs0_dps0__blk847_dn0 = assign27770_body21_e39221_d_n0;
            locals.var_fs0_dps0__blk847_dn2 = assign27770_body21_e39221_d_n2;
            locals.var_fs0_dps0__blk847_dn6 = assign27770_body21_e39221_d_n6;
            locals.var_fs0_dps0__blk847_dn7 = assign27770_body21_e39221_d_n7;
            locals.var_fs0_dps0__blk847_dn10 = assign27770_body21_e39221_d_n10;
            locals.var_fs0_dps0__blk847_dn11 = assign27770_body21_e39221_d_n11;
            locals.var_fs0_dps0__blk847_dn12 = assign27770_body21_e39221_d_n12;
            locals.var_fs0_dps0__blk847_dn17 = assign27770_body21_e39221_d_n17;
            locals.var_fs0_dps0__blk847_rv = 0.0;
            let assign27770_body22_e39224: f64 = if locals.var_flg_conv__blk789 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard882 = assign27770_body22_e39224;
            locals.var_guard882_rv = 0.0;
            let (assign27770_body23_e39243,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard882 != 0.0)) {
        let assign27770_body23_e39239: f64 = (2.0 * 20.0);
        let assign27770_body23_e39241: f64 = (assign27770_body23_e39239 + 1.0);
        (assign27770_body23_e39241,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign27770_body23_e39243;
            locals.var_lp_s0_rv = 0.0;
            let (assign27770_body24_e39262, assign27770_body24_e39262_d_n0, assign27770_body24_e39262_d_n2, assign27770_body24_e39262_d_n6, assign27770_body24_e39262_d_n7, assign27770_body24_e39262_d_n10, assign27770_body24_e39262_d_n11, assign27770_body24_e39262_d_n12, assign27770_body24_e39262_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard882 == 0.0)) {
        let assign27770_body24_e39258: f64 = (-locals.var_fs0__blk846);
        let assign27770_body24_e39260: f64 = (assign27770_body24_e39258 / locals.var_fs0_dps0__blk847);
        (assign27770_body24_e39260, ((((-locals.var_fs0__blk846_dn0) * locals.var_fs0_dps0__blk847) - (assign27770_body24_e39258 * locals.var_fs0_dps0__blk847_dn0)) / (locals.var_fs0_dps0__blk847 * locals.var_fs0_dps0__blk847)), ((((-locals.var_fs0__blk846_dn2) * locals.var_fs0_dps0__blk847) - (assign27770_body24_e39258 * locals.var_fs0_dps0__blk847_dn2)) / (locals.var_fs0_dps0__blk847 * locals.var_fs0_dps0__blk847)), ((((-locals.var_fs0__blk846_dn6) * locals.var_fs0_dps0__blk847) - (assign27770_body24_e39258 * locals.var_fs0_dps0__blk847_dn6)) / (locals.var_fs0_dps0__blk847 * locals.var_fs0_dps0__blk847)), ((((-locals.var_fs0__blk846_dn7) * locals.var_fs0_dps0__blk847) - (assign27770_body24_e39258 * locals.var_fs0_dps0__blk847_dn7)) / (locals.var_fs0_dps0__blk847 * locals.var_fs0_dps0__blk847)), ((((-locals.var_fs0__blk846_dn10) * locals.var_fs0_dps0__blk847) - (assign27770_body24_e39258 * locals.var_fs0_dps0__blk847_dn10)) / (locals.var_fs0_dps0__blk847 * locals.var_fs0_dps0__blk847)), ((((-locals.var_fs0__blk846_dn11) * locals.var_fs0_dps0__blk847) - (assign27770_body24_e39258 * locals.var_fs0_dps0__blk847_dn11)) / (locals.var_fs0_dps0__blk847 * locals.var_fs0_dps0__blk847)), ((((-locals.var_fs0__blk846_dn12) * locals.var_fs0_dps0__blk847) - (assign27770_body24_e39258 * locals.var_fs0_dps0__blk847_dn12)) / (locals.var_fs0_dps0__blk847 * locals.var_fs0_dps0__blk847)), ((((-locals.var_fs0__blk846_dn17) * locals.var_fs0_dps0__blk847) - (assign27770_body24_e39258 * locals.var_fs0_dps0__blk847_dn17)) / (locals.var_fs0_dps0__blk847 * locals.var_fs0_dps0__blk847)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign27770_body24_e39262;
            locals.var_dps0_dn0 = assign27770_body24_e39262_d_n0;
            locals.var_dps0_dn2 = assign27770_body24_e39262_d_n2;
            locals.var_dps0_dn6 = assign27770_body24_e39262_d_n6;
            locals.var_dps0_dn7 = assign27770_body24_e39262_d_n7;
            locals.var_dps0_dn10 = assign27770_body24_e39262_d_n10;
            locals.var_dps0_dn11 = assign27770_body24_e39262_d_n11;
            locals.var_dps0_dn12 = assign27770_body24_e39262_d_n12;
            locals.var_dps0_dn17 = assign27770_body24_e39262_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign27770_body25_e39291, assign27770_body25_e39291_d_n0, assign27770_body25_e39291_d_n2, assign27770_body25_e39291_d_n6, assign27770_body25_e39291_d_n7, assign27770_body25_e39291_d_n10, assign27770_body25_e39291_d_n11, assign27770_body25_e39291_d_n12, assign27770_body25_e39291_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard882 == 0.0)) {
        let assign27770_body25_e39278: f64 = (0.5 * 0.1);
        let assign27770_body25_e39282: f64 = (locals.var_ps0ld).abs();
        let (assign27770_body25_e39287, assign27770_body25_e39287_d_n0, assign27770_body25_e39287_d_n2, assign27770_body25_e39287_d_n6, assign27770_body25_e39287_d_n7, assign27770_body25_e39287_d_n10, assign27770_body25_e39287_d_n11, assign27770_body25_e39287_d_n12, assign27770_body25_e39287_d_n17,) = {
            if (1.0 >= assign27770_body25_e39282) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign27770_body25_e39286: f64 = (locals.var_ps0ld).abs();
                (assign27770_body25_e39286, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn12 } else { (-locals.var_ps0ld_dn12) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn17 } else { (-locals.var_ps0ld_dn17) },)
            }
        };
        let assign27770_body25_e39288: f64 = (1.0 + assign27770_body25_e39287);
        let assign27770_body25_e39289: f64 = (assign27770_body25_e39278 * assign27770_body25_e39288);
        (assign27770_body25_e39289, (assign27770_body25_e39278 * assign27770_body25_e39287_d_n0), (assign27770_body25_e39278 * assign27770_body25_e39287_d_n2), (assign27770_body25_e39278 * assign27770_body25_e39287_d_n6), (assign27770_body25_e39278 * assign27770_body25_e39287_d_n7), (assign27770_body25_e39278 * assign27770_body25_e39287_d_n10), (assign27770_body25_e39278 * assign27770_body25_e39287_d_n11), (assign27770_body25_e39278 * assign27770_body25_e39287_d_n12), (assign27770_body25_e39278 * assign27770_body25_e39287_d_n17),)
    } else {
        (locals.var_dplim__blk848, locals.var_dplim__blk848_dn0, locals.var_dplim__blk848_dn2, locals.var_dplim__blk848_dn6, locals.var_dplim__blk848_dn7, locals.var_dplim__blk848_dn10, locals.var_dplim__blk848_dn11, locals.var_dplim__blk848_dn12, locals.var_dplim__blk848_dn17,)
    }
};
            locals.var_dplim__blk848 = assign27770_body25_e39291;
            locals.var_dplim__blk848_dn0 = assign27770_body25_e39291_d_n0;
            locals.var_dplim__blk848_dn2 = assign27770_body25_e39291_d_n2;
            locals.var_dplim__blk848_dn6 = assign27770_body25_e39291_d_n6;
            locals.var_dplim__blk848_dn7 = assign27770_body25_e39291_d_n7;
            locals.var_dplim__blk848_dn10 = assign27770_body25_e39291_d_n10;
            locals.var_dplim__blk848_dn11 = assign27770_body25_e39291_d_n11;
            locals.var_dplim__blk848_dn12 = assign27770_body25_e39291_d_n12;
            locals.var_dplim__blk848_dn17 = assign27770_body25_e39291_d_n17;
            locals.var_dplim__blk848_rv = 0.0;
            let assign27770_body26_e39293: f64 = (locals.var_dps0).abs();
            let assign27770_body26_e39295: f64 = if assign27770_body26_e39293 > locals.var_dplim__blk848 { 1.0 } else { 0.0 };
            locals.var_guard883 = assign27770_body26_e39295;
            locals.var_guard883_rv = 0.0;
            let (assign27770_body27_e39321, assign27770_body27_e39321_d_n0, assign27770_body27_e39321_d_n2, assign27770_body27_e39321_d_n6, assign27770_body27_e39321_d_n7, assign27770_body27_e39321_d_n10, assign27770_body27_e39321_d_n11, assign27770_body27_e39321_d_n12, assign27770_body27_e39321_d_n17,) = {
    if (((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard882 == 0.0)) && (locals.var_guard883 != 0.0)) {
        let (assign27770_body27_e39318,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign27770_body27_e39317: f64 = (-1.0);
                (assign27770_body27_e39317,)
            }
        };
        let assign27770_body27_e39319: f64 = (locals.var_dplim__blk848 * assign27770_body27_e39318);
        (assign27770_body27_e39319, (locals.var_dplim__blk848_dn0 * assign27770_body27_e39318), (locals.var_dplim__blk848_dn2 * assign27770_body27_e39318), (locals.var_dplim__blk848_dn6 * assign27770_body27_e39318), (locals.var_dplim__blk848_dn7 * assign27770_body27_e39318), (locals.var_dplim__blk848_dn10 * assign27770_body27_e39318), (locals.var_dplim__blk848_dn11 * assign27770_body27_e39318), (locals.var_dplim__blk848_dn12 * assign27770_body27_e39318), (locals.var_dplim__blk848_dn17 * assign27770_body27_e39318),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign27770_body27_e39321;
            locals.var_dps0_dn0 = assign27770_body27_e39321_d_n0;
            locals.var_dps0_dn2 = assign27770_body27_e39321_d_n2;
            locals.var_dps0_dn6 = assign27770_body27_e39321_d_n6;
            locals.var_dps0_dn7 = assign27770_body27_e39321_d_n7;
            locals.var_dps0_dn10 = assign27770_body27_e39321_d_n10;
            locals.var_dps0_dn11 = assign27770_body27_e39321_d_n11;
            locals.var_dps0_dn12 = assign27770_body27_e39321_d_n12;
            locals.var_dps0_dn17 = assign27770_body27_e39321_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign27770_body28_e39339, assign27770_body28_e39339_d_n0, assign27770_body28_e39339_d_n2, assign27770_body28_e39339_d_n6, assign27770_body28_e39339_d_n7, assign27770_body28_e39339_d_n10, assign27770_body28_e39339_d_n11, assign27770_body28_e39339_d_n12, assign27770_body28_e39339_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard882 == 0.0)) {
        let assign27770_body28_e39337: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign27770_body28_e39337, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn12 + locals.var_dps0_dn12), (locals.var_ps0ld_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
            locals.var_ps0ld = assign27770_body28_e39339;
            locals.var_ps0ld_dn0 = assign27770_body28_e39339_d_n0;
            locals.var_ps0ld_dn2 = assign27770_body28_e39339_d_n2;
            locals.var_ps0ld_dn6 = assign27770_body28_e39339_d_n6;
            locals.var_ps0ld_dn7 = assign27770_body28_e39339_d_n7;
            locals.var_ps0ld_dn10 = assign27770_body28_e39339_d_n10;
            locals.var_ps0ld_dn11 = assign27770_body28_e39339_d_n11;
            locals.var_ps0ld_dn12 = assign27770_body28_e39339_d_n12;
            locals.var_ps0ld_dn17 = assign27770_body28_e39339_d_n17;
            locals.var_ps0ld_rv = 0.0;
            let assign27770_body29_e39341: f64 = (locals.var_dps0).abs();
            let assign27770_body29_e39345: f64 = (locals.var_fs0__blk846).abs();
            let assign27770_body29_e39348: f64 = if ((assign27770_body29_e39341 <= 5e-12) && (assign27770_body29_e39345 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard884 = assign27770_body29_e39348;
            locals.var_guard884_rv = 0.0;
            let (assign27770_body30_e39366,) = {
    if (((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard882 == 0.0)) && (locals.var_guard884 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv__blk789,)
    }
};
            locals.var_flg_conv__blk789 = assign27770_body30_e39366;
            locals.var_flg_conv__blk789_rv = 0.0;
            let (assign27770_body31_e39381,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27770_body31_e39379: f64 = (locals.var_lp_s0 + 1.0);
        (assign27770_body31_e39379,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign27770_body31_e39381;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_101(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign27790_e39387: f64 = if locals.var_chi__blk816 < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard886 = assign27790_e39387;
        locals.var_guard886_rv = 0.0;

        let (assign27830_e39446, assign27830_e39446_d_n0, assign27830_e39446_d_n2, assign27830_e39446_d_n6, assign27830_e39446_d_n7, assign27830_e39446_d_n10, assign27830_e39446_d_n11, assign27830_e39446_d_n12, assign27830_e39446_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard886 != 0.0)) {
        let assign27830_e39440: f64 = (locals.var_fb__blk840 * locals.var_fb__blk840);
        let assign27830_e39443: f64 = (10.0 * 2.220446049250313e-16);
        let assign27830_e39444: f64 = (assign27830_e39440 + assign27830_e39443);
        (assign27830_e39444, ((locals.var_fb__blk840_dn0 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn0)), ((locals.var_fb__blk840_dn2 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn2)), ((locals.var_fb__blk840_dn6 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn6)), ((locals.var_fb__blk840_dn7 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn7)), ((locals.var_fb__blk840_dn10 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn10)), ((locals.var_fb__blk840_dn11 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn11)), ((locals.var_fb__blk840_dn12 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn12)), ((locals.var_fb__blk840_dn17 * locals.var_fb__blk840) + (locals.var_fb__blk840 * locals.var_fb__blk840_dn17)),)
    } else {
        (locals.var_xi0__blk849, locals.var_xi0__blk849_dn0, locals.var_xi0__blk849_dn2, locals.var_xi0__blk849_dn6, locals.var_xi0__blk849_dn7, locals.var_xi0__blk849_dn10, locals.var_xi0__blk849_dn11, locals.var_xi0__blk849_dn12, locals.var_xi0__blk849_dn17,)
    }
};
        locals.var_xi0__blk849 = assign27830_e39446;
        locals.var_xi0__blk849_dn0 = assign27830_e39446_d_n0;
        locals.var_xi0__blk849_dn2 = assign27830_e39446_d_n2;
        locals.var_xi0__blk849_dn6 = assign27830_e39446_d_n6;
        locals.var_xi0__blk849_dn7 = assign27830_e39446_d_n7;
        locals.var_xi0__blk849_dn10 = assign27830_e39446_d_n10;
        locals.var_xi0__blk849_dn11 = assign27830_e39446_d_n11;
        locals.var_xi0__blk849_dn12 = assign27830_e39446_d_n12;
        locals.var_xi0__blk849_dn17 = assign27830_e39446_d_n17;
        locals.var_xi0__blk849_rv = 0.0;

        let (assign27840_e39465, assign27840_e39465_d_n0, assign27840_e39465_d_n2, assign27840_e39465_d_n6, assign27840_e39465_d_n7, assign27840_e39465_d_n10, assign27840_e39465_d_n11, assign27840_e39465_d_n12, assign27840_e39465_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard886 != 0.0)) {
        let assign27840_e39462: f64 = (10.0 * 2.220446049250313e-16);
        let assign27840_e39463: f64 = (locals.var_fb__blk840 + assign27840_e39462);
        (assign27840_e39463, locals.var_fb__blk840_dn0, locals.var_fb__blk840_dn2, locals.var_fb__blk840_dn6, locals.var_fb__blk840_dn7, locals.var_fb__blk840_dn10, locals.var_fb__blk840_dn11, locals.var_fb__blk840_dn12, locals.var_fb__blk840_dn17,)
    } else {
        (locals.var_xi0p12__blk850, locals.var_xi0p12__blk850_dn0, locals.var_xi0p12__blk850_dn2, locals.var_xi0p12__blk850_dn6, locals.var_xi0p12__blk850_dn7, locals.var_xi0p12__blk850_dn10, locals.var_xi0p12__blk850_dn11, locals.var_xi0p12__blk850_dn12, locals.var_xi0p12__blk850_dn17,)
    }
};
        locals.var_xi0p12__blk850 = assign27840_e39465;
        locals.var_xi0p12__blk850_dn0 = assign27840_e39465_d_n0;
        locals.var_xi0p12__blk850_dn2 = assign27840_e39465_d_n2;
        locals.var_xi0p12__blk850_dn6 = assign27840_e39465_d_n6;
        locals.var_xi0p12__blk850_dn7 = assign27840_e39465_d_n7;
        locals.var_xi0p12__blk850_dn10 = assign27840_e39465_d_n10;
        locals.var_xi0p12__blk850_dn11 = assign27840_e39465_d_n11;
        locals.var_xi0p12__blk850_dn12 = assign27840_e39465_d_n12;
        locals.var_xi0p12__blk850_dn17 = assign27840_e39465_d_n17;
        locals.var_xi0p12__blk850_rv = 0.0;

        let (assign27860_e39499, assign27860_e39499_d_n0, assign27860_e39499_d_n2, assign27860_e39499_d_n6, assign27860_e39499_d_n7, assign27860_e39499_d_n10, assign27860_e39499_d_n11, assign27860_e39499_d_n12, assign27860_e39499_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard886 == 0.0)) {
        let assign27860_e39497: f64 = (locals.var_chi__blk816 - 1.0);
        (assign27860_e39497, locals.var_chi__blk816_dn0, locals.var_chi__blk816_dn2, locals.var_chi__blk816_dn6, locals.var_chi__blk816_dn7, locals.var_chi__blk816_dn10, locals.var_chi__blk816_dn11, locals.var_chi__blk816_dn12, locals.var_chi__blk816_dn17,)
    } else {
        (locals.var_xi0__blk849, locals.var_xi0__blk849_dn0, locals.var_xi0__blk849_dn2, locals.var_xi0__blk849_dn6, locals.var_xi0__blk849_dn7, locals.var_xi0__blk849_dn10, locals.var_xi0__blk849_dn11, locals.var_xi0__blk849_dn12, locals.var_xi0__blk849_dn17,)
    }
};
        locals.var_xi0__blk849 = assign27860_e39499;
        locals.var_xi0__blk849_dn0 = assign27860_e39499_d_n0;
        locals.var_xi0__blk849_dn2 = assign27860_e39499_d_n2;
        locals.var_xi0__blk849_dn6 = assign27860_e39499_d_n6;
        locals.var_xi0__blk849_dn7 = assign27860_e39499_d_n7;
        locals.var_xi0__blk849_dn10 = assign27860_e39499_d_n10;
        locals.var_xi0__blk849_dn11 = assign27860_e39499_d_n11;
        locals.var_xi0__blk849_dn12 = assign27860_e39499_d_n12;
        locals.var_xi0__blk849_dn17 = assign27860_e39499_d_n17;
        locals.var_xi0__blk849_rv = 0.0;

        let (assign27870_e39516, assign27870_e39516_d_n0, assign27870_e39516_d_n2, assign27870_e39516_d_n6, assign27870_e39516_d_n7, assign27870_e39516_d_n10, assign27870_e39516_d_n11, assign27870_e39516_d_n12, assign27870_e39516_d_n17,) = {
    if ((((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) && (locals.var_guard886 == 0.0)) {
        let assign27870_e39514: f64 = (locals.var_xi0__blk849).sqrt();
        (assign27870_e39514, (locals.var_xi0__blk849_dn0 / (2.0 * assign27870_e39514)), (locals.var_xi0__blk849_dn2 / (2.0 * assign27870_e39514)), (locals.var_xi0__blk849_dn6 / (2.0 * assign27870_e39514)), (locals.var_xi0__blk849_dn7 / (2.0 * assign27870_e39514)), (locals.var_xi0__blk849_dn10 / (2.0 * assign27870_e39514)), (locals.var_xi0__blk849_dn11 / (2.0 * assign27870_e39514)), (locals.var_xi0__blk849_dn12 / (2.0 * assign27870_e39514)), (locals.var_xi0__blk849_dn17 / (2.0 * assign27870_e39514)),)
    } else {
        (locals.var_xi0p12__blk850, locals.var_xi0p12__blk850_dn0, locals.var_xi0p12__blk850_dn2, locals.var_xi0p12__blk850_dn6, locals.var_xi0p12__blk850_dn7, locals.var_xi0p12__blk850_dn10, locals.var_xi0p12__blk850_dn11, locals.var_xi0p12__blk850_dn12, locals.var_xi0p12__blk850_dn17,)
    }
};
        locals.var_xi0p12__blk850 = assign27870_e39516;
        locals.var_xi0p12__blk850_dn0 = assign27870_e39516_d_n0;
        locals.var_xi0p12__blk850_dn2 = assign27870_e39516_d_n2;
        locals.var_xi0p12__blk850_dn6 = assign27870_e39516_d_n6;
        locals.var_xi0p12__blk850_dn7 = assign27870_e39516_d_n7;
        locals.var_xi0p12__blk850_dn10 = assign27870_e39516_d_n10;
        locals.var_xi0p12__blk850_dn11 = assign27870_e39516_d_n11;
        locals.var_xi0p12__blk850_dn12 = assign27870_e39516_d_n12;
        locals.var_xi0p12__blk850_dn17 = assign27870_e39516_d_n17;
        locals.var_xi0p12__blk850_rv = 0.0;

        let (assign27880_e39531, assign27880_e39531_d_n0, assign27880_e39531_d_n2, assign27880_e39531_d_n6, assign27880_e39531_d_n7, assign27880_e39531_d_n10, assign27880_e39531_d_n11, assign27880_e39531_d_n12, assign27880_e39531_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27880_e39529: f64 = (locals.var_cnst0over * locals.var_xi0p12__blk850);
        (assign27880_e39529, ((locals.var_cnst0over_dn0 * locals.var_xi0p12__blk850) + (locals.var_cnst0over * locals.var_xi0p12__blk850_dn0)), ((locals.var_cnst0over_dn2 * locals.var_xi0p12__blk850) + (locals.var_cnst0over * locals.var_xi0p12__blk850_dn2)), ((locals.var_cnst0over_dn6 * locals.var_xi0p12__blk850) + (locals.var_cnst0over * locals.var_xi0p12__blk850_dn6)), ((locals.var_cnst0over_dn7 * locals.var_xi0p12__blk850) + (locals.var_cnst0over * locals.var_xi0p12__blk850_dn7)), ((locals.var_cnst0over_dn10 * locals.var_xi0p12__blk850) + (locals.var_cnst0over * locals.var_xi0p12__blk850_dn10)), ((locals.var_cnst0over_dn11 * locals.var_xi0p12__blk850) + (locals.var_cnst0over * locals.var_xi0p12__blk850_dn11)), ((locals.var_cnst0over_dn12 * locals.var_xi0p12__blk850) + (locals.var_cnst0over * locals.var_xi0p12__blk850_dn12)), ((locals.var_cnst0over_dn17 * locals.var_xi0p12__blk850) + (locals.var_cnst0over * locals.var_xi0p12__blk850_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign27880_e39531;
        locals.var_qbuld_dn0 = assign27880_e39531_d_n0;
        locals.var_qbuld_dn2 = assign27880_e39531_d_n2;
        locals.var_qbuld_dn6 = assign27880_e39531_d_n6;
        locals.var_qbuld_dn7 = assign27880_e39531_d_n7;
        locals.var_qbuld_dn10 = assign27880_e39531_d_n10;
        locals.var_qbuld_dn11 = assign27880_e39531_d_n11;
        locals.var_qbuld_dn12 = assign27880_e39531_d_n12;
        locals.var_qbuld_dn17 = assign27880_e39531_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign27890_e39548, assign27890_e39548_d_n0, assign27890_e39548_d_n2, assign27890_e39548_d_n6, assign27890_e39548_d_n7, assign27890_e39548_d_n10, assign27890_e39548_d_n11, assign27890_e39548_d_n12, assign27890_e39548_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27890_e39545: f64 = (locals.var_fs02__blk842 + locals.var_xi0p12__blk850);
        let assign27890_e39546: f64 = (1.0 / assign27890_e39545);
        (assign27890_e39546, (-((locals.var_fs02__blk842_dn0 + locals.var_xi0p12__blk850_dn0) / (assign27890_e39545 * assign27890_e39545))), (-((locals.var_fs02__blk842_dn2 + locals.var_xi0p12__blk850_dn2) / (assign27890_e39545 * assign27890_e39545))), (-((locals.var_fs02__blk842_dn6 + locals.var_xi0p12__blk850_dn6) / (assign27890_e39545 * assign27890_e39545))), (-((locals.var_fs02__blk842_dn7 + locals.var_xi0p12__blk850_dn7) / (assign27890_e39545 * assign27890_e39545))), (-((locals.var_fs02__blk842_dn10 + locals.var_xi0p12__blk850_dn10) / (assign27890_e39545 * assign27890_e39545))), (-((locals.var_fs02__blk842_dn11 + locals.var_xi0p12__blk850_dn11) / (assign27890_e39545 * assign27890_e39545))), (-((locals.var_fs02__blk842_dn12 + locals.var_xi0p12__blk850_dn12) / (assign27890_e39545 * assign27890_e39545))), (-((locals.var_fs02__blk842_dn17 + locals.var_xi0p12__blk850_dn17) / (assign27890_e39545 * assign27890_e39545))),)
    } else {
        (locals.var_t1__blk773, locals.var_t1__blk773_dn0, locals.var_t1__blk773_dn2, locals.var_t1__blk773_dn6, locals.var_t1__blk773_dn7, locals.var_t1__blk773_dn10, locals.var_t1__blk773_dn11, locals.var_t1__blk773_dn12, locals.var_t1__blk773_dn17,)
    }
};
        locals.var_t1__blk773 = assign27890_e39548;
        locals.var_t1__blk773_dn0 = assign27890_e39548_d_n0;
        locals.var_t1__blk773_dn2 = assign27890_e39548_d_n2;
        locals.var_t1__blk773_dn6 = assign27890_e39548_d_n6;
        locals.var_t1__blk773_dn7 = assign27890_e39548_d_n7;
        locals.var_t1__blk773_dn10 = assign27890_e39548_d_n10;
        locals.var_t1__blk773_dn11 = assign27890_e39548_d_n11;
        locals.var_t1__blk773_dn12 = assign27890_e39548_d_n12;
        locals.var_t1__blk773_dn17 = assign27890_e39548_d_n17;
        locals.var_t1__blk773_rv = 0.0;

        let (assign27900_e39565, assign27900_e39565_d_n0, assign27900_e39565_d_n2, assign27900_e39565_d_n6, assign27900_e39565_d_n7, assign27900_e39565_d_n10, assign27900_e39565_d_n11, assign27900_e39565_d_n12, assign27900_e39565_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27900_e39561: f64 = (locals.var_cnst0over * locals.var_fs01__blk838);
        let assign27900_e39563: f64 = (assign27900_e39561 * locals.var_t1__blk773);
        (assign27900_e39563, ((((locals.var_cnst0over_dn0 * locals.var_fs01__blk838) + (locals.var_cnst0over * locals.var_fs01__blk838_dn0)) * locals.var_t1__blk773) + (assign27900_e39561 * locals.var_t1__blk773_dn0)), ((((locals.var_cnst0over_dn2 * locals.var_fs01__blk838) + (locals.var_cnst0over * locals.var_fs01__blk838_dn2)) * locals.var_t1__blk773) + (assign27900_e39561 * locals.var_t1__blk773_dn2)), ((((locals.var_cnst0over_dn6 * locals.var_fs01__blk838) + (locals.var_cnst0over * locals.var_fs01__blk838_dn6)) * locals.var_t1__blk773) + (assign27900_e39561 * locals.var_t1__blk773_dn6)), ((((locals.var_cnst0over_dn7 * locals.var_fs01__blk838) + (locals.var_cnst0over * locals.var_fs01__blk838_dn7)) * locals.var_t1__blk773) + (assign27900_e39561 * locals.var_t1__blk773_dn7)), ((((locals.var_cnst0over_dn10 * locals.var_fs01__blk838) + (locals.var_cnst0over * locals.var_fs01__blk838_dn10)) * locals.var_t1__blk773) + (assign27900_e39561 * locals.var_t1__blk773_dn10)), ((((locals.var_cnst0over_dn11 * locals.var_fs01__blk838) + (locals.var_cnst0over * locals.var_fs01__blk838_dn11)) * locals.var_t1__blk773) + (assign27900_e39561 * locals.var_t1__blk773_dn11)), ((((locals.var_cnst0over_dn12 * locals.var_fs01__blk838) + (locals.var_cnst0over * locals.var_fs01__blk838_dn12)) * locals.var_t1__blk773) + (assign27900_e39561 * locals.var_t1__blk773_dn12)), ((((locals.var_cnst0over_dn17 * locals.var_fs01__blk838) + (locals.var_cnst0over * locals.var_fs01__blk838_dn17)) * locals.var_t1__blk773) + (assign27900_e39561 * locals.var_t1__blk773_dn17)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign27900_e39565;
        locals.var_qiuld_dn0 = assign27900_e39565_d_n0;
        locals.var_qiuld_dn2 = assign27900_e39565_d_n2;
        locals.var_qiuld_dn6 = assign27900_e39565_d_n6;
        locals.var_qiuld_dn7 = assign27900_e39565_d_n7;
        locals.var_qiuld_dn10 = assign27900_e39565_d_n10;
        locals.var_qiuld_dn11 = assign27900_e39565_d_n11;
        locals.var_qiuld_dn12 = assign27900_e39565_d_n12;
        locals.var_qiuld_dn17 = assign27900_e39565_d_n17;
        locals.var_qiuld_rv = 0.0;

        let (assign27910_e39580, assign27910_e39580_d_n0, assign27910_e39580_d_n2, assign27910_e39580_d_n6, assign27910_e39580_d_n7, assign27910_e39580_d_n10, assign27910_e39580_d_n11, assign27910_e39580_d_n12, assign27910_e39580_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard873 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27910_e39578: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign27910_e39578, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), (locals.var_qbuld_dn17 + locals.var_qiuld_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign27910_e39580;
        locals.var_qsuld_dn0 = assign27910_e39580_d_n0;
        locals.var_qsuld_dn2 = assign27910_e39580_d_n2;
        locals.var_qsuld_dn6 = assign27910_e39580_d_n6;
        locals.var_qsuld_dn7 = assign27910_e39580_d_n7;
        locals.var_qsuld_dn10 = assign27910_e39580_d_n10;
        locals.var_qsuld_dn11 = assign27910_e39580_d_n11;
        locals.var_qsuld_dn12 = assign27910_e39580_d_n12;
        locals.var_qsuld_dn17 = assign27910_e39580_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign27920_e39590, assign27920_e39590_d_n0, assign27920_e39590_d_n2, assign27920_e39590_d_n6, assign27920_e39590_d_n7, assign27920_e39590_d_n10, assign27920_e39590_d_n11, assign27920_e39590_d_n12, assign27920_e39590_d_n17,) = {
    if (((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) {
        let assign27920_e39588: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign27920_e39588, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), (locals.var_qsuld_dn17 - locals.var_qbuld_dn17),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign27920_e39590;
        locals.var_qiuld_dn0 = assign27920_e39590_d_n0;
        locals.var_qiuld_dn2 = assign27920_e39590_d_n2;
        locals.var_qiuld_dn6 = assign27920_e39590_d_n6;
        locals.var_qiuld_dn7 = assign27920_e39590_d_n7;
        locals.var_qiuld_dn10 = assign27920_e39590_d_n10;
        locals.var_qiuld_dn11 = assign27920_e39590_d_n11;
        locals.var_qiuld_dn12 = assign27920_e39590_d_n12;
        locals.var_qiuld_dn17 = assign27920_e39590_d_n17;
        locals.var_qiuld_rv = 0.0;

        let assign27930_e39593: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard888 = assign27930_e39593;
        locals.var_guard888_rv = 0.0;

        let assign27940_e39596: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard889 = assign27940_e39596;
        locals.var_guard889_rv = 0.0;

        let (assign27950_e39611, assign27950_e39611_d_n0, assign27950_e39611_d_n2, assign27950_e39611_d_n6, assign27950_e39611_d_n7, assign27950_e39611_d_n10, assign27950_e39611_d_n11, assign27950_e39611_d_n12, assign27950_e39611_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard888 != 0.0)) && (locals.var_flg_ovloops != 0.0)) {
        let assign27950_e39607: f64 = (-locals.var_uc_areabt);
        let assign27950_e39609: f64 = (assign27950_e39607 * locals.var_qsuld);
        (assign27950_e39609, (assign27950_e39607 * locals.var_qsuld_dn0), (assign27950_e39607 * locals.var_qsuld_dn2), (assign27950_e39607 * locals.var_qsuld_dn6), (assign27950_e39607 * locals.var_qsuld_dn7), (assign27950_e39607 * locals.var_qsuld_dn10), (assign27950_e39607 * locals.var_qsuld_dn11), (assign27950_e39607 * locals.var_qsuld_dn12), (assign27950_e39607 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_sus, locals.var_qbody_bt_p_sus_dn0, locals.var_qbody_bt_p_sus_dn2, locals.var_qbody_bt_p_sus_dn6, locals.var_qbody_bt_p_sus_dn7, locals.var_qbody_bt_p_sus_dn10, locals.var_qbody_bt_p_sus_dn11, locals.var_qbody_bt_p_sus_dn12, locals.var_qbody_bt_p_sus_dn17,)
    }
};
        locals.var_qbody_bt_p_sus = assign27950_e39611;
        locals.var_qbody_bt_p_sus_dn0 = assign27950_e39611_d_n0;
        locals.var_qbody_bt_p_sus_dn2 = assign27950_e39611_d_n2;
        locals.var_qbody_bt_p_sus_dn6 = assign27950_e39611_d_n6;
        locals.var_qbody_bt_p_sus_dn7 = assign27950_e39611_d_n7;
        locals.var_qbody_bt_p_sus_dn10 = assign27950_e39611_d_n10;
        locals.var_qbody_bt_p_sus_dn11 = assign27950_e39611_d_n11;
        locals.var_qbody_bt_p_sus_dn12 = assign27950_e39611_d_n12;
        locals.var_qbody_bt_p_sus_dn17 = assign27950_e39611_d_n17;
        locals.var_qbody_bt_p_sus_rv = 0.0;

        let (assign27960_e39626, assign27960_e39626_d_n0, assign27960_e39626_d_n2, assign27960_e39626_d_n6, assign27960_e39626_d_n7, assign27960_e39626_d_n10, assign27960_e39626_d_n11, assign27960_e39626_d_n12, assign27960_e39626_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard888 != 0.0)) && (locals.var_flg_ovloops != 0.0)) {
        let assign27960_e39622: f64 = (-locals.var_uc_areabt);
        let assign27960_e39624: f64 = (assign27960_e39622 * locals.var_qiuld);
        (assign27960_e39624, (assign27960_e39622 * locals.var_qiuld_dn0), (assign27960_e39622 * locals.var_qiuld_dn2), (assign27960_e39622 * locals.var_qiuld_dn6), (assign27960_e39622 * locals.var_qiuld_dn7), (assign27960_e39622 * locals.var_qiuld_dn10), (assign27960_e39622 * locals.var_qiuld_dn11), (assign27960_e39622 * locals.var_qiuld_dn12), (assign27960_e39622 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_ius, locals.var_qbody_bt_p_ius_dn0, locals.var_qbody_bt_p_ius_dn2, locals.var_qbody_bt_p_ius_dn6, locals.var_qbody_bt_p_ius_dn7, locals.var_qbody_bt_p_ius_dn10, locals.var_qbody_bt_p_ius_dn11, locals.var_qbody_bt_p_ius_dn12, locals.var_qbody_bt_p_ius_dn17,)
    }
};
        locals.var_qbody_bt_p_ius = assign27960_e39626;
        locals.var_qbody_bt_p_ius_dn0 = assign27960_e39626_d_n0;
        locals.var_qbody_bt_p_ius_dn2 = assign27960_e39626_d_n2;
        locals.var_qbody_bt_p_ius_dn6 = assign27960_e39626_d_n6;
        locals.var_qbody_bt_p_ius_dn7 = assign27960_e39626_d_n7;
        locals.var_qbody_bt_p_ius_dn10 = assign27960_e39626_d_n10;
        locals.var_qbody_bt_p_ius_dn11 = assign27960_e39626_d_n11;
        locals.var_qbody_bt_p_ius_dn12 = assign27960_e39626_d_n12;
        locals.var_qbody_bt_p_ius_dn17 = assign27960_e39626_d_n17;
        locals.var_qbody_bt_p_ius_rv = 0.0;

        let (assign27970_e39641, assign27970_e39641_d_n0, assign27970_e39641_d_n2, assign27970_e39641_d_n6, assign27970_e39641_d_n7, assign27970_e39641_d_n10, assign27970_e39641_d_n11, assign27970_e39641_d_n12, assign27970_e39641_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard888 != 0.0)) && (locals.var_flg_ovloopd != 0.0)) {
        let assign27970_e39637: f64 = (-locals.var_uc_areabt);
        let assign27970_e39639: f64 = (assign27970_e39637 * locals.var_qsuld);
        (assign27970_e39639, (assign27970_e39637 * locals.var_qsuld_dn0), (assign27970_e39637 * locals.var_qsuld_dn2), (assign27970_e39637 * locals.var_qsuld_dn6), (assign27970_e39637 * locals.var_qsuld_dn7), (assign27970_e39637 * locals.var_qsuld_dn10), (assign27970_e39637 * locals.var_qsuld_dn11), (assign27970_e39637 * locals.var_qsuld_dn12), (assign27970_e39637 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_sud, locals.var_qbody_bt_p_sud_dn0, locals.var_qbody_bt_p_sud_dn2, locals.var_qbody_bt_p_sud_dn6, locals.var_qbody_bt_p_sud_dn7, locals.var_qbody_bt_p_sud_dn10, locals.var_qbody_bt_p_sud_dn11, locals.var_qbody_bt_p_sud_dn12, locals.var_qbody_bt_p_sud_dn17,)
    }
};
        locals.var_qbody_bt_p_sud = assign27970_e39641;
        locals.var_qbody_bt_p_sud_dn0 = assign27970_e39641_d_n0;
        locals.var_qbody_bt_p_sud_dn2 = assign27970_e39641_d_n2;
        locals.var_qbody_bt_p_sud_dn6 = assign27970_e39641_d_n6;
        locals.var_qbody_bt_p_sud_dn7 = assign27970_e39641_d_n7;
        locals.var_qbody_bt_p_sud_dn10 = assign27970_e39641_d_n10;
        locals.var_qbody_bt_p_sud_dn11 = assign27970_e39641_d_n11;
        locals.var_qbody_bt_p_sud_dn12 = assign27970_e39641_d_n12;
        locals.var_qbody_bt_p_sud_dn17 = assign27970_e39641_d_n17;
        locals.var_qbody_bt_p_sud_rv = 0.0;

        let (assign27980_e39656, assign27980_e39656_d_n0, assign27980_e39656_d_n2, assign27980_e39656_d_n6, assign27980_e39656_d_n7, assign27980_e39656_d_n10, assign27980_e39656_d_n11, assign27980_e39656_d_n12, assign27980_e39656_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && (locals.var_guard888 != 0.0)) && (locals.var_flg_ovloopd != 0.0)) {
        let assign27980_e39652: f64 = (-locals.var_uc_areabt);
        let assign27980_e39654: f64 = (assign27980_e39652 * locals.var_qiuld);
        (assign27980_e39654, (assign27980_e39652 * locals.var_qiuld_dn0), (assign27980_e39652 * locals.var_qiuld_dn2), (assign27980_e39652 * locals.var_qiuld_dn6), (assign27980_e39652 * locals.var_qiuld_dn7), (assign27980_e39652 * locals.var_qiuld_dn10), (assign27980_e39652 * locals.var_qiuld_dn11), (assign27980_e39652 * locals.var_qiuld_dn12), (assign27980_e39652 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_iud, locals.var_qbody_bt_p_iud_dn0, locals.var_qbody_bt_p_iud_dn2, locals.var_qbody_bt_p_iud_dn6, locals.var_qbody_bt_p_iud_dn7, locals.var_qbody_bt_p_iud_dn10, locals.var_qbody_bt_p_iud_dn11, locals.var_qbody_bt_p_iud_dn12, locals.var_qbody_bt_p_iud_dn17,)
    }
};
        locals.var_qbody_bt_p_iud = assign27980_e39656;
        locals.var_qbody_bt_p_iud_dn0 = assign27980_e39656_d_n0;
        locals.var_qbody_bt_p_iud_dn2 = assign27980_e39656_d_n2;
        locals.var_qbody_bt_p_iud_dn6 = assign27980_e39656_d_n6;
        locals.var_qbody_bt_p_iud_dn7 = assign27980_e39656_d_n7;
        locals.var_qbody_bt_p_iud_dn10 = assign27980_e39656_d_n10;
        locals.var_qbody_bt_p_iud_dn11 = assign27980_e39656_d_n11;
        locals.var_qbody_bt_p_iud_dn12 = assign27980_e39656_d_n12;
        locals.var_qbody_bt_p_iud_dn17 = assign27980_e39656_d_n17;
        locals.var_qbody_bt_p_iud_rv = 0.0;

        let (assign27990_e39674, assign27990_e39674_d_n0, assign27990_e39674_d_n2, assign27990_e39674_d_n6, assign27990_e39674_d_n7, assign27990_e39674_d_n10, assign27990_e39674_d_n11, assign27990_e39674_d_n12, assign27990_e39674_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && ((locals.var_guard889 != 0.0) && (locals.var_guard888 == 0.0))) && (locals.var_flg_ovloops != 0.0)) {
        let assign27990_e39670: f64 = (-locals.var_uc_areabt);
        let assign27990_e39672: f64 = (assign27990_e39670 * locals.var_qsuld);
        (assign27990_e39672, (assign27990_e39670 * locals.var_qsuld_dn0), (assign27990_e39670 * locals.var_qsuld_dn2), (assign27990_e39670 * locals.var_qsuld_dn6), (assign27990_e39670 * locals.var_qsuld_dn7), (assign27990_e39670 * locals.var_qsuld_dn10), (assign27990_e39670 * locals.var_qsuld_dn11), (assign27990_e39670 * locals.var_qsuld_dn12), (assign27990_e39670 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_sus, locals.var_qbody_bt_n_sus_dn0, locals.var_qbody_bt_n_sus_dn2, locals.var_qbody_bt_n_sus_dn6, locals.var_qbody_bt_n_sus_dn7, locals.var_qbody_bt_n_sus_dn10, locals.var_qbody_bt_n_sus_dn11, locals.var_qbody_bt_n_sus_dn12, locals.var_qbody_bt_n_sus_dn17,)
    }
};
        locals.var_qbody_bt_n_sus = assign27990_e39674;
        locals.var_qbody_bt_n_sus_dn0 = assign27990_e39674_d_n0;
        locals.var_qbody_bt_n_sus_dn2 = assign27990_e39674_d_n2;
        locals.var_qbody_bt_n_sus_dn6 = assign27990_e39674_d_n6;
        locals.var_qbody_bt_n_sus_dn7 = assign27990_e39674_d_n7;
        locals.var_qbody_bt_n_sus_dn10 = assign27990_e39674_d_n10;
        locals.var_qbody_bt_n_sus_dn11 = assign27990_e39674_d_n11;
        locals.var_qbody_bt_n_sus_dn12 = assign27990_e39674_d_n12;
        locals.var_qbody_bt_n_sus_dn17 = assign27990_e39674_d_n17;
        locals.var_qbody_bt_n_sus_rv = 0.0;

        let (assign28000_e39692, assign28000_e39692_d_n0, assign28000_e39692_d_n2, assign28000_e39692_d_n6, assign28000_e39692_d_n7, assign28000_e39692_d_n10, assign28000_e39692_d_n11, assign28000_e39692_d_n12, assign28000_e39692_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && ((locals.var_guard889 != 0.0) && (locals.var_guard888 == 0.0))) && (locals.var_flg_ovloops != 0.0)) {
        let assign28000_e39688: f64 = (-locals.var_uc_areabt);
        let assign28000_e39690: f64 = (assign28000_e39688 * locals.var_qiuld);
        (assign28000_e39690, (assign28000_e39688 * locals.var_qiuld_dn0), (assign28000_e39688 * locals.var_qiuld_dn2), (assign28000_e39688 * locals.var_qiuld_dn6), (assign28000_e39688 * locals.var_qiuld_dn7), (assign28000_e39688 * locals.var_qiuld_dn10), (assign28000_e39688 * locals.var_qiuld_dn11), (assign28000_e39688 * locals.var_qiuld_dn12), (assign28000_e39688 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_ius, locals.var_qbody_bt_n_ius_dn0, locals.var_qbody_bt_n_ius_dn2, locals.var_qbody_bt_n_ius_dn6, locals.var_qbody_bt_n_ius_dn7, locals.var_qbody_bt_n_ius_dn10, locals.var_qbody_bt_n_ius_dn11, locals.var_qbody_bt_n_ius_dn12, locals.var_qbody_bt_n_ius_dn17,)
    }
};
        locals.var_qbody_bt_n_ius = assign28000_e39692;
        locals.var_qbody_bt_n_ius_dn0 = assign28000_e39692_d_n0;
        locals.var_qbody_bt_n_ius_dn2 = assign28000_e39692_d_n2;
        locals.var_qbody_bt_n_ius_dn6 = assign28000_e39692_d_n6;
        locals.var_qbody_bt_n_ius_dn7 = assign28000_e39692_d_n7;
        locals.var_qbody_bt_n_ius_dn10 = assign28000_e39692_d_n10;
        locals.var_qbody_bt_n_ius_dn11 = assign28000_e39692_d_n11;
        locals.var_qbody_bt_n_ius_dn12 = assign28000_e39692_d_n12;
        locals.var_qbody_bt_n_ius_dn17 = assign28000_e39692_d_n17;
        locals.var_qbody_bt_n_ius_rv = 0.0;

        let (assign28010_e39710, assign28010_e39710_d_n0, assign28010_e39710_d_n2, assign28010_e39710_d_n6, assign28010_e39710_d_n7, assign28010_e39710_d_n10, assign28010_e39710_d_n11, assign28010_e39710_d_n12, assign28010_e39710_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && ((locals.var_guard889 != 0.0) && (locals.var_guard888 == 0.0))) && (locals.var_flg_ovloopd != 0.0)) {
        let assign28010_e39706: f64 = (-locals.var_uc_areabt);
        let assign28010_e39708: f64 = (assign28010_e39706 * locals.var_qsuld);
        (assign28010_e39708, (assign28010_e39706 * locals.var_qsuld_dn0), (assign28010_e39706 * locals.var_qsuld_dn2), (assign28010_e39706 * locals.var_qsuld_dn6), (assign28010_e39706 * locals.var_qsuld_dn7), (assign28010_e39706 * locals.var_qsuld_dn10), (assign28010_e39706 * locals.var_qsuld_dn11), (assign28010_e39706 * locals.var_qsuld_dn12), (assign28010_e39706 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_sud, locals.var_qbody_bt_n_sud_dn0, locals.var_qbody_bt_n_sud_dn2, locals.var_qbody_bt_n_sud_dn6, locals.var_qbody_bt_n_sud_dn7, locals.var_qbody_bt_n_sud_dn10, locals.var_qbody_bt_n_sud_dn11, locals.var_qbody_bt_n_sud_dn12, locals.var_qbody_bt_n_sud_dn17,)
    }
};
        locals.var_qbody_bt_n_sud = assign28010_e39710;
        locals.var_qbody_bt_n_sud_dn0 = assign28010_e39710_d_n0;
        locals.var_qbody_bt_n_sud_dn2 = assign28010_e39710_d_n2;
        locals.var_qbody_bt_n_sud_dn6 = assign28010_e39710_d_n6;
        locals.var_qbody_bt_n_sud_dn7 = assign28010_e39710_d_n7;
        locals.var_qbody_bt_n_sud_dn10 = assign28010_e39710_d_n10;
        locals.var_qbody_bt_n_sud_dn11 = assign28010_e39710_d_n11;
        locals.var_qbody_bt_n_sud_dn12 = assign28010_e39710_d_n12;
        locals.var_qbody_bt_n_sud_dn17 = assign28010_e39710_d_n17;
        locals.var_qbody_bt_n_sud_rv = 0.0;

        let (assign28020_e39728, assign28020_e39728_d_n0, assign28020_e39728_d_n2, assign28020_e39728_d_n6, assign28020_e39728_d_n7, assign28020_e39728_d_n10, assign28020_e39728_d_n11, assign28020_e39728_d_n12, assign28020_e39728_d_n17,) = {
    if (((((locals.var_guard771 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard853 != 0.0)) && ((locals.var_guard889 != 0.0) && (locals.var_guard888 == 0.0))) && (locals.var_flg_ovloopd != 0.0)) {
        let assign28020_e39724: f64 = (-locals.var_uc_areabt);
        let assign28020_e39726: f64 = (assign28020_e39724 * locals.var_qiuld);
        (assign28020_e39726, (assign28020_e39724 * locals.var_qiuld_dn0), (assign28020_e39724 * locals.var_qiuld_dn2), (assign28020_e39724 * locals.var_qiuld_dn6), (assign28020_e39724 * locals.var_qiuld_dn7), (assign28020_e39724 * locals.var_qiuld_dn10), (assign28020_e39724 * locals.var_qiuld_dn11), (assign28020_e39724 * locals.var_qiuld_dn12), (assign28020_e39724 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_iud, locals.var_qbody_bt_n_iud_dn0, locals.var_qbody_bt_n_iud_dn2, locals.var_qbody_bt_n_iud_dn6, locals.var_qbody_bt_n_iud_dn7, locals.var_qbody_bt_n_iud_dn10, locals.var_qbody_bt_n_iud_dn11, locals.var_qbody_bt_n_iud_dn12, locals.var_qbody_bt_n_iud_dn17,)
    }
};
        locals.var_qbody_bt_n_iud = assign28020_e39728;
        locals.var_qbody_bt_n_iud_dn0 = assign28020_e39728_d_n0;
        locals.var_qbody_bt_n_iud_dn2 = assign28020_e39728_d_n2;
        locals.var_qbody_bt_n_iud_dn6 = assign28020_e39728_d_n6;
        locals.var_qbody_bt_n_iud_dn7 = assign28020_e39728_d_n7;
        locals.var_qbody_bt_n_iud_dn10 = assign28020_e39728_d_n10;
        locals.var_qbody_bt_n_iud_dn11 = assign28020_e39728_d_n11;
        locals.var_qbody_bt_n_iud_dn12 = assign28020_e39728_d_n12;
        locals.var_qbody_bt_n_iud_dn17 = assign28020_e39728_d_n17;
        locals.var_qbody_bt_n_iud_rv = 0.0;

        locals.var_aclm = p.p189;
        locals.var_aclm_rv = 0.0;

        let assign28040_e39732: f64 = if locals.var_flg_noqi != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard892 = assign28040_e39732;
        locals.var_guard892_rv = 0.0;

        let (assign28050_e39738, assign28050_e39738_d_n0, assign28050_e39738_d_n2, assign28050_e39738_d_n6, assign28050_e39738_d_n7, assign28050_e39738_d_n10, assign28050_e39738_d_n11, assign28050_e39738_d_n12, assign28050_e39738_d_n17,) = {
    if (locals.var_guard892 != 0.0) {
        let assign28050_e39736: f64 = (locals.var_vds + locals.var_ps0);
        (assign28050_e39736, (locals.var_vds_dn0 + locals.var_ps0_dn0), (locals.var_vds_dn2 + locals.var_ps0_dn2), (locals.var_vds_dn6 + locals.var_ps0_dn6), (locals.var_vds_dn7 + locals.var_ps0_dn7), (locals.var_vds_dn10 + locals.var_ps0_dn10), (locals.var_vds_dn11 + locals.var_ps0_dn11), (locals.var_vds_dn12 + locals.var_ps0_dn12), (locals.var_vds_dn17 + locals.var_ps0_dn17),)
    } else {
        (locals.var_t2__blk891, locals.var_t2__blk891_dn0, locals.var_t2__blk891_dn2, locals.var_t2__blk891_dn6, locals.var_t2__blk891_dn7, locals.var_t2__blk891_dn10, locals.var_t2__blk891_dn11, locals.var_t2__blk891_dn12, locals.var_t2__blk891_dn17,)
    }
};
        locals.var_t2__blk891 = assign28050_e39738;
        locals.var_t2__blk891_dn0 = assign28050_e39738_d_n0;
        locals.var_t2__blk891_dn2 = assign28050_e39738_d_n2;
        locals.var_t2__blk891_dn6 = assign28050_e39738_d_n6;
        locals.var_t2__blk891_dn7 = assign28050_e39738_d_n7;
        locals.var_t2__blk891_dn10 = assign28050_e39738_d_n10;
        locals.var_t2__blk891_dn11 = assign28050_e39738_d_n11;
        locals.var_t2__blk891_dn12 = assign28050_e39738_d_n12;
        locals.var_t2__blk891_dn17 = assign28050_e39738_d_n17;
        locals.var_t2__blk891_rv = 0.0;

        let (assign28060_e39750, assign28060_e39750_d_n0, assign28060_e39750_d_n2, assign28060_e39750_d_n6, assign28060_e39750_d_n7, assign28060_e39750_d_n10, assign28060_e39750_d_n11, assign28060_e39750_d_n12, assign28060_e39750_d_n17,) = {
    if (locals.var_guard892 != 0.0) {
        let assign28060_e39742: f64 = (locals.var_aclm * locals.var_t2__blk891);
        let assign28060_e39745: f64 = (1.0 - locals.var_aclm);
        let assign28060_e39747: f64 = (assign28060_e39745 * locals.var_psl);
        let assign28060_e39748: f64 = (assign28060_e39742 + assign28060_e39747);
        (assign28060_e39748, ((locals.var_aclm * locals.var_t2__blk891_dn0) + (assign28060_e39745 * locals.var_psl_dn0)), ((locals.var_aclm * locals.var_t2__blk891_dn2) + (assign28060_e39745 * locals.var_psl_dn2)), ((locals.var_aclm * locals.var_t2__blk891_dn6) + (assign28060_e39745 * locals.var_psl_dn6)), ((locals.var_aclm * locals.var_t2__blk891_dn7) + (assign28060_e39745 * locals.var_psl_dn7)), ((locals.var_aclm * locals.var_t2__blk891_dn10) + (assign28060_e39745 * locals.var_psl_dn10)), ((locals.var_aclm * locals.var_t2__blk891_dn11) + (assign28060_e39745 * locals.var_psl_dn11)), ((locals.var_aclm * locals.var_t2__blk891_dn12) + (assign28060_e39745 * locals.var_psl_dn12)), ((locals.var_aclm * locals.var_t2__blk891_dn17) + (assign28060_e39745 * locals.var_psl_dn17)),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign28060_e39750;
        locals.var_psdl_dn0 = assign28060_e39750_d_n0;
        locals.var_psdl_dn2 = assign28060_e39750_d_n2;
        locals.var_psdl_dn6 = assign28060_e39750_d_n6;
        locals.var_psdl_dn7 = assign28060_e39750_d_n7;
        locals.var_psdl_dn10 = assign28060_e39750_d_n10;
        locals.var_psdl_dn11 = assign28060_e39750_d_n11;
        locals.var_psdl_dn12 = assign28060_e39750_d_n12;
        locals.var_psdl_dn17 = assign28060_e39750_d_n17;
        locals.var_psdl_rv = 0.0;

        let assign28070_e39753: f64 = if p.p64 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard893 = assign28070_e39753;
        locals.var_guard893_rv = 0.0;

        let (assign28080_e39759, assign28080_e39759_d_n0, assign28080_e39759_d_n2, assign28080_e39759_d_n6, assign28080_e39759_d_n7, assign28080_e39759_d_n10, assign28080_e39759_d_n11, assign28080_e39759_d_n12, assign28080_e39759_d_n17,) = {
    if ((locals.var_guard892 != 0.0) && (locals.var_guard893 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn12, locals.var_ec_dn17,)
    }
};
        locals.var_ec = assign28080_e39759;
        locals.var_ec_dn0 = assign28080_e39759_d_n0;
        locals.var_ec_dn2 = assign28080_e39759_d_n2;
        locals.var_ec_dn6 = assign28080_e39759_d_n6;
        locals.var_ec_dn7 = assign28080_e39759_d_n7;
        locals.var_ec_dn10 = assign28080_e39759_d_n10;
        locals.var_ec_dn11 = assign28080_e39759_d_n11;
        locals.var_ec_dn12 = assign28080_e39759_d_n12;
        locals.var_ec_dn17 = assign28080_e39759_d_n17;
        locals.var_ec_rv = 0.0;

        let assign28090_e39763: f64 = (locals.var_ps0 + locals.var_vds);
        let assign28090_e39766: f64 = (10.0 * 2.220446049250313e-16);
        let assign28090_e39767: f64 = (assign28090_e39763 - assign28090_e39766);
        let assign28090_e39768: f64 = if locals.var_psdl > assign28090_e39767 { 1.0 } else { 0.0 };
        locals.var_guard894 = assign28090_e39768;
        locals.var_guard894_rv = 0.0;

        let (assign28100_e39780, assign28100_e39780_d_n0, assign28100_e39780_d_n2, assign28100_e39780_d_n6, assign28100_e39780_d_n7, assign28100_e39780_d_n10, assign28100_e39780_d_n11, assign28100_e39780_d_n12, assign28100_e39780_d_n17,) = {
    if ((locals.var_guard892 != 0.0) && (locals.var_guard894 != 0.0)) {
        let assign28100_e39774: f64 = (locals.var_ps0 + locals.var_vds);
        let assign28100_e39777: f64 = (10.0 * 2.220446049250313e-16);
        let assign28100_e39778: f64 = (assign28100_e39774 - assign28100_e39777);
        (assign28100_e39778, (locals.var_ps0_dn0 + locals.var_vds_dn0), (locals.var_ps0_dn2 + locals.var_vds_dn2), (locals.var_ps0_dn6 + locals.var_vds_dn6), (locals.var_ps0_dn7 + locals.var_vds_dn7), (locals.var_ps0_dn10 + locals.var_vds_dn10), (locals.var_ps0_dn11 + locals.var_vds_dn11), (locals.var_ps0_dn12 + locals.var_vds_dn12), (locals.var_ps0_dn17 + locals.var_vds_dn17),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign28100_e39780;
        locals.var_psdl_dn0 = assign28100_e39780_d_n0;
        locals.var_psdl_dn2 = assign28100_e39780_d_n2;
        locals.var_psdl_dn6 = assign28100_e39780_d_n6;
        locals.var_psdl_dn7 = assign28100_e39780_d_n7;
        locals.var_psdl_dn10 = assign28100_e39780_d_n10;
        locals.var_psdl_dn11 = assign28100_e39780_d_n11;
        locals.var_psdl_dn12 = assign28100_e39780_d_n12;
        locals.var_psdl_dn17 = assign28100_e39780_d_n17;
        locals.var_psdl_rv = 0.0;

        let assign28110_e39783: f64 = if p.p64 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard895 = assign28110_e39783;
        locals.var_guard895_rv = 0.0;

        let assign28120_e39786: f64 = if locals.var_idd < 1e-15 { 1.0 } else { 0.0 };
        locals.var_guard896 = assign28120_e39786;
        locals.var_guard896_rv = 0.0;

        let (assign28130_e39795, assign28130_e39795_d_n0, assign28130_e39795_d_n2, assign28130_e39795_d_n6, assign28130_e39795_d_n7, assign28130_e39795_d_n10, assign28130_e39795_d_n11, assign28130_e39795_d_n12, assign28130_e39795_d_n17,) = {
    if (((locals.var_guard892 == 0.0) && (locals.var_guard895 != 0.0)) && (locals.var_guard896 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn12, locals.var_ec_dn17,)
    }
};
        locals.var_ec = assign28130_e39795;
        locals.var_ec_dn0 = assign28130_e39795_d_n0;
        locals.var_ec_dn2 = assign28130_e39795_d_n2;
        locals.var_ec_dn6 = assign28130_e39795_d_n6;
        locals.var_ec_dn7 = assign28130_e39795_d_n7;
        locals.var_ec_dn10 = assign28130_e39795_d_n10;
        locals.var_ec_dn11 = assign28130_e39795_d_n11;
        locals.var_ec_dn12 = assign28130_e39795_d_n12;
        locals.var_ec_dn17 = assign28130_e39795_d_n17;
        locals.var_ec_rv = 0.0;

        let (assign28140_e39807, assign28140_e39807_d_n10,) = {
    if (((locals.var_guard892 == 0.0) && (locals.var_guard895 != 0.0)) && (locals.var_guard896 == 0.0)) {
        let assign28140_e39805: f64 = (locals.var_beta_inv / locals.var_leff);
        (assign28140_e39805, (locals.var_beta_inv_dn10 / locals.var_leff),)
    } else {
        (locals.var_t1__blk890, locals.var_t1__blk890_dn10,)
    }
};
        locals.var_t1__blk890 = assign28140_e39807;
        locals.var_t1__blk890_dn10 = assign28140_e39807_d_n10;
        locals.var_t1__blk890_rv = 0.0;

        let (assign28150_e39819, assign28150_e39819_d_n0, assign28150_e39819_d_n2, assign28150_e39819_d_n6, assign28150_e39819_d_n7, assign28150_e39819_d_n10, assign28150_e39819_d_n11, assign28150_e39819_d_n12, assign28150_e39819_d_n17,) = {
    if (((locals.var_guard892 == 0.0) && (locals.var_guard895 != 0.0)) && (locals.var_guard896 == 0.0)) {
        let assign28150_e39817: f64 = (1.0 / locals.var_qn0);
        (assign28150_e39817, (-(locals.var_qn0_dn0 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn2 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn6 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn7 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn10 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn11 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn12 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn17 / (locals.var_qn0 * locals.var_qn0))),)
    } else {
        (locals.var_t2__blk891, locals.var_t2__blk891_dn0, locals.var_t2__blk891_dn2, locals.var_t2__blk891_dn6, locals.var_t2__blk891_dn7, locals.var_t2__blk891_dn10, locals.var_t2__blk891_dn11, locals.var_t2__blk891_dn12, locals.var_t2__blk891_dn17,)
    }
};
        locals.var_t2__blk891 = assign28150_e39819;
        locals.var_t2__blk891_dn0 = assign28150_e39819_d_n0;
        locals.var_t2__blk891_dn2 = assign28150_e39819_d_n2;
        locals.var_t2__blk891_dn6 = assign28150_e39819_d_n6;
        locals.var_t2__blk891_dn7 = assign28150_e39819_d_n7;
        locals.var_t2__blk891_dn10 = assign28150_e39819_d_n10;
        locals.var_t2__blk891_dn11 = assign28150_e39819_d_n11;
        locals.var_t2__blk891_dn12 = assign28150_e39819_d_n12;
        locals.var_t2__blk891_dn17 = assign28150_e39819_d_n17;
        locals.var_t2__blk891_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_102(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28160_e39833, assign28160_e39833_d_n0, assign28160_e39833_d_n2, assign28160_e39833_d_n6, assign28160_e39833_d_n7, assign28160_e39833_d_n10, assign28160_e39833_d_n11, assign28160_e39833_d_n12, assign28160_e39833_d_n17,) = {
    if (((locals.var_guard892 == 0.0) && (locals.var_guard895 != 0.0)) && (locals.var_guard896 == 0.0)) {
        let assign28160_e39829: f64 = (locals.var_idd * locals.var_t1__blk890);
        let assign28160_e39831: f64 = (assign28160_e39829 * locals.var_t2__blk891);
        (assign28160_e39831, (((locals.var_idd_dn0 * locals.var_t1__blk890) * locals.var_t2__blk891) + (assign28160_e39829 * locals.var_t2__blk891_dn0)), (((locals.var_idd_dn2 * locals.var_t1__blk890) * locals.var_t2__blk891) + (assign28160_e39829 * locals.var_t2__blk891_dn2)), (((locals.var_idd_dn6 * locals.var_t1__blk890) * locals.var_t2__blk891) + (assign28160_e39829 * locals.var_t2__blk891_dn6)), (((locals.var_idd_dn7 * locals.var_t1__blk890) * locals.var_t2__blk891) + (assign28160_e39829 * locals.var_t2__blk891_dn7)), ((((locals.var_idd_dn10 * locals.var_t1__blk890) + (locals.var_idd * locals.var_t1__blk890_dn10)) * locals.var_t2__blk891) + (assign28160_e39829 * locals.var_t2__blk891_dn10)), (((locals.var_idd_dn11 * locals.var_t1__blk890) * locals.var_t2__blk891) + (assign28160_e39829 * locals.var_t2__blk891_dn11)), (((locals.var_idd_dn12 * locals.var_t1__blk890) * locals.var_t2__blk891) + (assign28160_e39829 * locals.var_t2__blk891_dn12)), (((locals.var_idd_dn17 * locals.var_t1__blk890) * locals.var_t2__blk891) + (assign28160_e39829 * locals.var_t2__blk891_dn17)),)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn12, locals.var_ec_dn17,)
    }
};
        locals.var_ec = assign28160_e39833;
        locals.var_ec_dn0 = assign28160_e39833_d_n0;
        locals.var_ec_dn2 = assign28160_e39833_d_n2;
        locals.var_ec_dn6 = assign28160_e39833_d_n6;
        locals.var_ec_dn7 = assign28160_e39833_d_n7;
        locals.var_ec_dn10 = assign28160_e39833_d_n10;
        locals.var_ec_dn11 = assign28160_e39833_d_n11;
        locals.var_ec_dn12 = assign28160_e39833_d_n12;
        locals.var_ec_dn17 = assign28160_e39833_d_n17;
        locals.var_ec_rv = 0.0;

        locals.var_cox0__blk908 = locals.var_c_fox0;
        locals.var_cox0__blk908_rv = 0.0;

        let assign28180_e39837: f64 = (1.0 / locals.var_cox0__blk908);
        locals.var_cox0_inv__blk909 = assign28180_e39837;
        locals.var_cox0_inv__blk909_rv = 0.0;

        locals.var_vgbgmt__blk929 = 0.0;
        locals.var_vgbgmt__blk929_dn0 = 0.0;
        locals.var_vgbgmt__blk929_dn2 = 0.0;
        locals.var_vgbgmt__blk929_dn6 = 0.0;
        locals.var_vgbgmt__blk929_dn7 = 0.0;
        locals.var_vgbgmt__blk929_dn10 = 0.0;
        locals.var_vgbgmt__blk929_dn11 = 0.0;
        locals.var_vgbgmt__blk929_dn12 = 0.0;
        locals.var_vgbgmt__blk929_dn17 = 0.0;
        locals.var_vgbgmt__blk929_rv = 0.0;

        locals.var_fb__blk969 = 0.0;
        locals.var_fb__blk969_dn0 = 0.0;
        locals.var_fb__blk969_dn2 = 0.0;
        locals.var_fb__blk969_dn6 = 0.0;
        locals.var_fb__blk969_dn7 = 0.0;
        locals.var_fb__blk969_dn10 = 0.0;
        locals.var_fb__blk969_dn11 = 0.0;
        locals.var_fb__blk969_dn12 = 0.0;
        locals.var_fb__blk969_dn17 = 0.0;
        locals.var_fb__blk969_rv = 0.0;

        locals.var_fs01__blk967 = 0.0;
        locals.var_fs01__blk967_dn0 = 0.0;
        locals.var_fs01__blk967_dn2 = 0.0;
        locals.var_fs01__blk967_dn6 = 0.0;
        locals.var_fs01__blk967_dn7 = 0.0;
        locals.var_fs01__blk967_dn10 = 0.0;
        locals.var_fs01__blk967_dn11 = 0.0;
        locals.var_fs01__blk967_dn12 = 0.0;
        locals.var_fs01__blk967_dn17 = 0.0;
        locals.var_fs01__blk967_rv = 0.0;

        locals.var_fs02__blk971 = 0.0;
        locals.var_fs02__blk971_dn0 = 0.0;
        locals.var_fs02__blk971_dn2 = 0.0;
        locals.var_fs02__blk971_dn6 = 0.0;
        locals.var_fs02__blk971_dn7 = 0.0;
        locals.var_fs02__blk971_dn10 = 0.0;
        locals.var_fs02__blk971_dn11 = 0.0;
        locals.var_fs02__blk971_dn12 = 0.0;
        locals.var_fs02__blk971_dn17 = 0.0;
        locals.var_fs02__blk971_rv = 0.0;

        let assign28230_e39848: f64 = if ((p.p29 >= 1.0) && (p.p188 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard980 = assign28230_e39848;
        locals.var_guard980_rv = 0.0;

        let (assign28240_e39854,) = {
    if ((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) {
        (p.p171,)
    } else {
        (locals.var_cov_slp,)
    }
};
        locals.var_cov_slp = assign28240_e39854;
        locals.var_cov_slp_rv = 0.0;

        let (assign28250_e39860,) = {
    if ((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) {
        (p.p172,)
    } else {
        (locals.var_cov_mag,)
    }
};
        locals.var_cov_mag = assign28250_e39860;
        locals.var_cov_mag_rv = 0.0;

        let (assign28260_e39866, assign28260_e39866_d_n6, assign28260_e39866_d_n7, assign28260_e39866_d_n11,) = {
    if ((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn11,)
    } else {
        (locals.var_covvg, locals.var_covvg_dn6, locals.var_covvg_dn7, locals.var_covvg_dn11,)
    }
};
        locals.var_covvg = assign28260_e39866;
        locals.var_covvg_dn6 = assign28260_e39866_d_n6;
        locals.var_covvg_dn7 = assign28260_e39866_d_n7;
        locals.var_covvg_dn11 = assign28260_e39866_d_n11;
        locals.var_covvg_rv = 0.0;

        let (assign28270_e39872,) = {
    if ((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) {
        (p.p188,)
    } else {
        (locals.var_lov,)
    }
};
        locals.var_lov = assign28270_e39872;
        locals.var_lov_rv = 0.0;

        let assign28280_e39879: f64 = if ((locals.var_mks_nover == 0.0) && (p.p188 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard981 = assign28280_e39879;
        locals.var_guard981_rv = 0.0;

        let (assign28290_e39896, assign28290_e39896_d_n0, assign28290_e39896_d_n2, assign28290_e39896_d_n6, assign28290_e39896_d_n7, assign28290_e39896_d_n10, assign28290_e39896_d_n11, assign28290_e39896_d_n12, assign28290_e39896_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 != 0.0)) {
        let (assign28290_e39894,) = {
            if (p.p43 == 1.0) {
                let assign28290_e39890: f64 = (locals.var_w_dioscv * locals.var_cox0__blk908);
                (assign28290_e39890,)
            } else {
                let assign28290_e39893: f64 = (locals.var_weffcv_nf * locals.var_cox0__blk908);
                (assign28290_e39893,)
            }
        };
        (assign28290_e39894, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17,)
    }
};
        locals.var_t1__blk898 = assign28290_e39896;
        locals.var_t1__blk898_dn0 = assign28290_e39896_d_n0;
        locals.var_t1__blk898_dn2 = assign28290_e39896_d_n2;
        locals.var_t1__blk898_dn6 = assign28290_e39896_d_n6;
        locals.var_t1__blk898_dn7 = assign28290_e39896_d_n7;
        locals.var_t1__blk898_dn10 = assign28290_e39896_d_n10;
        locals.var_t1__blk898_dn11 = assign28290_e39896_d_n11;
        locals.var_t1__blk898_dn12 = assign28290_e39896_d_n12;
        locals.var_t1__blk898_dn17 = assign28290_e39896_d_n17;
        locals.var_t1__blk898_rv = 0.0;

        let (assign28300_e39910, assign28300_e39910_d_n0, assign28300_e39910_d_n2, assign28300_e39910_d_n6, assign28300_e39910_d_n7, assign28300_e39910_d_n10, assign28300_e39910_d_n11, assign28300_e39910_d_n12, assign28300_e39910_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28300_e39904: f64 = (locals.var_cov_slp * locals.var_t1__blk898);
        let assign28300_e39907: f64 = (locals.var_cov_mag + locals.var_covvg);
        let assign28300_e39908: f64 = (assign28300_e39904 * assign28300_e39907);
        (assign28300_e39908, ((locals.var_cov_slp * locals.var_t1__blk898_dn0) * assign28300_e39907), ((locals.var_cov_slp * locals.var_t1__blk898_dn2) * assign28300_e39907), (((locals.var_cov_slp * locals.var_t1__blk898_dn6) * assign28300_e39907) + (assign28300_e39904 * locals.var_covvg_dn6)), (((locals.var_cov_slp * locals.var_t1__blk898_dn7) * assign28300_e39907) + (assign28300_e39904 * locals.var_covvg_dn7)), ((locals.var_cov_slp * locals.var_t1__blk898_dn10) * assign28300_e39907), (((locals.var_cov_slp * locals.var_t1__blk898_dn11) * assign28300_e39907) + (assign28300_e39904 * locals.var_covvg_dn11)), ((locals.var_cov_slp * locals.var_t1__blk898_dn12) * assign28300_e39907), ((locals.var_cov_slp * locals.var_t1__blk898_dn17) * assign28300_e39907),)
    } else {
        (locals.var_t4__blk901, locals.var_t4__blk901_dn0, locals.var_t4__blk901_dn2, locals.var_t4__blk901_dn6, locals.var_t4__blk901_dn7, locals.var_t4__blk901_dn10, locals.var_t4__blk901_dn11, locals.var_t4__blk901_dn12, locals.var_t4__blk901_dn17,)
    }
};
        locals.var_t4__blk901 = assign28300_e39910;
        locals.var_t4__blk901_dn0 = assign28300_e39910_d_n0;
        locals.var_t4__blk901_dn2 = assign28300_e39910_d_n2;
        locals.var_t4__blk901_dn6 = assign28300_e39910_d_n6;
        locals.var_t4__blk901_dn7 = assign28300_e39910_d_n7;
        locals.var_t4__blk901_dn10 = assign28300_e39910_d_n10;
        locals.var_t4__blk901_dn11 = assign28300_e39910_d_n11;
        locals.var_t4__blk901_dn12 = assign28300_e39910_d_n12;
        locals.var_t4__blk901_dn17 = assign28300_e39910_d_n17;
        locals.var_t4__blk901_rv = 0.0;

        let (assign28310_e39920, assign28310_e39920_d_n0, assign28310_e39920_d_n2, assign28310_e39920_d_n6, assign28310_e39920_d_n7, assign28310_e39920_d_n10, assign28310_e39920_d_n11, assign28310_e39920_d_n12, assign28310_e39920_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28310_e39918: f64 = (locals.var_lov * locals.var_t1__blk898);
        (assign28310_e39918, (locals.var_lov * locals.var_t1__blk898_dn0), (locals.var_lov * locals.var_t1__blk898_dn2), (locals.var_lov * locals.var_t1__blk898_dn6), (locals.var_lov * locals.var_t1__blk898_dn7), (locals.var_lov * locals.var_t1__blk898_dn10), (locals.var_lov * locals.var_t1__blk898_dn11), (locals.var_lov * locals.var_t1__blk898_dn12), (locals.var_lov * locals.var_t1__blk898_dn17),)
    } else {
        (locals.var_t5__blk902, locals.var_t5__blk902_dn0, locals.var_t5__blk902_dn2, locals.var_t5__blk902_dn6, locals.var_t5__blk902_dn7, locals.var_t5__blk902_dn10, locals.var_t5__blk902_dn11, locals.var_t5__blk902_dn12, locals.var_t5__blk902_dn17,)
    }
};
        locals.var_t5__blk902 = assign28310_e39920;
        locals.var_t5__blk902_dn0 = assign28310_e39920_d_n0;
        locals.var_t5__blk902_dn2 = assign28310_e39920_d_n2;
        locals.var_t5__blk902_dn6 = assign28310_e39920_d_n6;
        locals.var_t5__blk902_dn7 = assign28310_e39920_d_n7;
        locals.var_t5__blk902_dn10 = assign28310_e39920_d_n10;
        locals.var_t5__blk902_dn11 = assign28310_e39920_d_n11;
        locals.var_t5__blk902_dn12 = assign28310_e39920_d_n12;
        locals.var_t5__blk902_dn17 = assign28310_e39920_d_n17;
        locals.var_t5__blk902_rv = 0.0;

        let (assign28320_e39928, assign28320_e39928_d_n0, assign28320_e39928_d_n2, assign28320_e39928_d_n6, assign28320_e39928_d_n7, assign28320_e39928_d_n10, assign28320_e39928_d_n11, assign28320_e39928_d_n12, assign28320_e39928_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_tx__blk906, locals.var_tx__blk906_dn0, locals.var_tx__blk906_dn2, locals.var_tx__blk906_dn6, locals.var_tx__blk906_dn7, locals.var_tx__blk906_dn10, locals.var_tx__blk906_dn11, locals.var_tx__blk906_dn12, locals.var_tx__blk906_dn17,)
    }
};
        locals.var_tx__blk906 = assign28320_e39928;
        locals.var_tx__blk906_dn0 = assign28320_e39928_d_n0;
        locals.var_tx__blk906_dn2 = assign28320_e39928_d_n2;
        locals.var_tx__blk906_dn6 = assign28320_e39928_d_n6;
        locals.var_tx__blk906_dn7 = assign28320_e39928_d_n7;
        locals.var_tx__blk906_dn10 = assign28320_e39928_d_n10;
        locals.var_tx__blk906_dn11 = assign28320_e39928_d_n11;
        locals.var_tx__blk906_dn12 = assign28320_e39928_d_n12;
        locals.var_tx__blk906_dn17 = assign28320_e39928_d_n17;
        locals.var_tx__blk906_rv = 0.0;

        let (assign28330_e39938, assign28330_e39938_d_n0, assign28330_e39938_d_n2, assign28330_e39938_d_n6, assign28330_e39938_d_n7, assign28330_e39938_d_n10, assign28330_e39938_d_n11, assign28330_e39938_d_n12, assign28330_e39938_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28330_e39936: f64 = (1.2 - locals.var_tx__blk906);
        (assign28330_e39936, (-locals.var_tx__blk906_dn0), (-locals.var_tx__blk906_dn2), (-locals.var_tx__blk906_dn6), (-locals.var_tx__blk906_dn7), (-locals.var_tx__blk906_dn10), (-locals.var_tx__blk906_dn11), (-locals.var_tx__blk906_dn12), (-locals.var_tx__blk906_dn17),)
    } else {
        (locals.var_t9__blk903, locals.var_t9__blk903_dn0, locals.var_t9__blk903_dn2, locals.var_t9__blk903_dn6, locals.var_t9__blk903_dn7, locals.var_t9__blk903_dn10, locals.var_t9__blk903_dn11, locals.var_t9__blk903_dn12, locals.var_t9__blk903_dn17,)
    }
};
        locals.var_t9__blk903 = assign28330_e39938;
        locals.var_t9__blk903_dn0 = assign28330_e39938_d_n0;
        locals.var_t9__blk903_dn2 = assign28330_e39938_d_n2;
        locals.var_t9__blk903_dn6 = assign28330_e39938_d_n6;
        locals.var_t9__blk903_dn7 = assign28330_e39938_d_n7;
        locals.var_t9__blk903_dn10 = assign28330_e39938_d_n10;
        locals.var_t9__blk903_dn11 = assign28330_e39938_d_n11;
        locals.var_t9__blk903_dn12 = assign28330_e39938_d_n12;
        locals.var_t9__blk903_dn17 = assign28330_e39938_d_n17;
        locals.var_t9__blk903_rv = 0.0;

        let (assign28340_e39952, assign28340_e39952_d_n0, assign28340_e39952_d_n2, assign28340_e39952_d_n6, assign28340_e39952_d_n7, assign28340_e39952_d_n10, assign28340_e39952_d_n11, assign28340_e39952_d_n12, assign28340_e39952_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28340_e39946: f64 = (locals.var_vgs * locals.var_t5__blk902);
        let assign28340_e39949: f64 = (locals.var_t9__blk903 * locals.var_t4__blk901);
        let assign28340_e39950: f64 = (assign28340_e39946 - assign28340_e39949);
        (assign28340_e39950, ((locals.var_vgs * locals.var_t5__blk902_dn0) - ((locals.var_t9__blk903_dn0 * locals.var_t4__blk901) + (locals.var_t9__blk903 * locals.var_t4__blk901_dn0))), ((locals.var_vgs * locals.var_t5__blk902_dn2) - ((locals.var_t9__blk903_dn2 * locals.var_t4__blk901) + (locals.var_t9__blk903 * locals.var_t4__blk901_dn2))), (((locals.var_vgs_dn6 * locals.var_t5__blk902) + (locals.var_vgs * locals.var_t5__blk902_dn6)) - ((locals.var_t9__blk903_dn6 * locals.var_t4__blk901) + (locals.var_t9__blk903 * locals.var_t4__blk901_dn6))), (((locals.var_vgs_dn7 * locals.var_t5__blk902) + (locals.var_vgs * locals.var_t5__blk902_dn7)) - ((locals.var_t9__blk903_dn7 * locals.var_t4__blk901) + (locals.var_t9__blk903 * locals.var_t4__blk901_dn7))), ((locals.var_vgs * locals.var_t5__blk902_dn10) - ((locals.var_t9__blk903_dn10 * locals.var_t4__blk901) + (locals.var_t9__blk903 * locals.var_t4__blk901_dn10))), (((locals.var_vgs_dn11 * locals.var_t5__blk902) + (locals.var_vgs * locals.var_t5__blk902_dn11)) - ((locals.var_t9__blk903_dn11 * locals.var_t4__blk901) + (locals.var_t9__blk903 * locals.var_t4__blk901_dn11))), ((locals.var_vgs * locals.var_t5__blk902_dn12) - ((locals.var_t9__blk903_dn12 * locals.var_t4__blk901) + (locals.var_t9__blk903 * locals.var_t4__blk901_dn12))), ((locals.var_vgs * locals.var_t5__blk902_dn17) - ((locals.var_t9__blk903_dn17 * locals.var_t4__blk901) + (locals.var_t9__blk903 * locals.var_t4__blk901_dn17))),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign28340_e39952;
        locals.var_qgos_dn0 = assign28340_e39952_d_n0;
        locals.var_qgos_dn2 = assign28340_e39952_d_n2;
        locals.var_qgos_dn6 = assign28340_e39952_d_n6;
        locals.var_qgos_dn7 = assign28340_e39952_d_n7;
        locals.var_qgos_dn10 = assign28340_e39952_d_n10;
        locals.var_qgos_dn11 = assign28340_e39952_d_n11;
        locals.var_qgos_dn12 = assign28340_e39952_d_n12;
        locals.var_qgos_dn17 = assign28340_e39952_d_n17;
        locals.var_qgos_rv = 0.0;

        let (assign28350_e39968, assign28350_e39968_d_n0, assign28350_e39968_d_n2, assign28350_e39968_d_n6, assign28350_e39968_d_n7, assign28350_e39968_d_n10, assign28350_e39968_d_n11, assign28350_e39968_d_n12, assign28350_e39968_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28350_e39960: f64 = (locals.var_cov_slp * locals.var_t1__blk898);
        let assign28350_e39963: f64 = (locals.var_cov_mag + locals.var_covvg);
        let assign28350_e39965: f64 = (assign28350_e39963 - locals.var_vds);
        let assign28350_e39966: f64 = (assign28350_e39960 * assign28350_e39965);
        (assign28350_e39966, (((locals.var_cov_slp * locals.var_t1__blk898_dn0) * assign28350_e39965) + (assign28350_e39960 * (-locals.var_vds_dn0))), (((locals.var_cov_slp * locals.var_t1__blk898_dn2) * assign28350_e39965) + (assign28350_e39960 * (-locals.var_vds_dn2))), (((locals.var_cov_slp * locals.var_t1__blk898_dn6) * assign28350_e39965) + (assign28350_e39960 * (locals.var_covvg_dn6 - locals.var_vds_dn6))), (((locals.var_cov_slp * locals.var_t1__blk898_dn7) * assign28350_e39965) + (assign28350_e39960 * (locals.var_covvg_dn7 - locals.var_vds_dn7))), (((locals.var_cov_slp * locals.var_t1__blk898_dn10) * assign28350_e39965) + (assign28350_e39960 * (-locals.var_vds_dn10))), (((locals.var_cov_slp * locals.var_t1__blk898_dn11) * assign28350_e39965) + (assign28350_e39960 * (locals.var_covvg_dn11 - locals.var_vds_dn11))), (((locals.var_cov_slp * locals.var_t1__blk898_dn12) * assign28350_e39965) + (assign28350_e39960 * (-locals.var_vds_dn12))), (((locals.var_cov_slp * locals.var_t1__blk898_dn17) * assign28350_e39965) + (assign28350_e39960 * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_t4__blk901, locals.var_t4__blk901_dn0, locals.var_t4__blk901_dn2, locals.var_t4__blk901_dn6, locals.var_t4__blk901_dn7, locals.var_t4__blk901_dn10, locals.var_t4__blk901_dn11, locals.var_t4__blk901_dn12, locals.var_t4__blk901_dn17,)
    }
};
        locals.var_t4__blk901 = assign28350_e39968;
        locals.var_t4__blk901_dn0 = assign28350_e39968_d_n0;
        locals.var_t4__blk901_dn2 = assign28350_e39968_d_n2;
        locals.var_t4__blk901_dn6 = assign28350_e39968_d_n6;
        locals.var_t4__blk901_dn7 = assign28350_e39968_d_n7;
        locals.var_t4__blk901_dn10 = assign28350_e39968_d_n10;
        locals.var_t4__blk901_dn11 = assign28350_e39968_d_n11;
        locals.var_t4__blk901_dn12 = assign28350_e39968_d_n12;
        locals.var_t4__blk901_dn17 = assign28350_e39968_d_n17;
        locals.var_t4__blk901_rv = 0.0;

        let (assign28360_e39978, assign28360_e39978_d_n0, assign28360_e39978_d_n2, assign28360_e39978_d_n6, assign28360_e39978_d_n7, assign28360_e39978_d_n10, assign28360_e39978_d_n11, assign28360_e39978_d_n12, assign28360_e39978_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28360_e39976: f64 = (locals.var_psl - locals.var_vds);
        (assign28360_e39976, (locals.var_psl_dn0 - locals.var_vds_dn0), (locals.var_psl_dn2 - locals.var_vds_dn2), (locals.var_psl_dn6 - locals.var_vds_dn6), (locals.var_psl_dn7 - locals.var_vds_dn7), (locals.var_psl_dn10 - locals.var_vds_dn10), (locals.var_psl_dn11 - locals.var_vds_dn11), (locals.var_psl_dn12 - locals.var_vds_dn12), (locals.var_psl_dn17 - locals.var_vds_dn17),)
    } else {
        (locals.var_tx__blk906, locals.var_tx__blk906_dn0, locals.var_tx__blk906_dn2, locals.var_tx__blk906_dn6, locals.var_tx__blk906_dn7, locals.var_tx__blk906_dn10, locals.var_tx__blk906_dn11, locals.var_tx__blk906_dn12, locals.var_tx__blk906_dn17,)
    }
};
        locals.var_tx__blk906 = assign28360_e39978;
        locals.var_tx__blk906_dn0 = assign28360_e39978_d_n0;
        locals.var_tx__blk906_dn2 = assign28360_e39978_d_n2;
        locals.var_tx__blk906_dn6 = assign28360_e39978_d_n6;
        locals.var_tx__blk906_dn7 = assign28360_e39978_d_n7;
        locals.var_tx__blk906_dn10 = assign28360_e39978_d_n10;
        locals.var_tx__blk906_dn11 = assign28360_e39978_d_n11;
        locals.var_tx__blk906_dn12 = assign28360_e39978_d_n12;
        locals.var_tx__blk906_dn17 = assign28360_e39978_d_n17;
        locals.var_tx__blk906_rv = 0.0;

        let (assign28370_e39988, assign28370_e39988_d_n0, assign28370_e39988_d_n2, assign28370_e39988_d_n6, assign28370_e39988_d_n7, assign28370_e39988_d_n10, assign28370_e39988_d_n11, assign28370_e39988_d_n12, assign28370_e39988_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28370_e39986: f64 = (1.2 - locals.var_tx__blk906);
        (assign28370_e39986, (-locals.var_tx__blk906_dn0), (-locals.var_tx__blk906_dn2), (-locals.var_tx__blk906_dn6), (-locals.var_tx__blk906_dn7), (-locals.var_tx__blk906_dn10), (-locals.var_tx__blk906_dn11), (-locals.var_tx__blk906_dn12), (-locals.var_tx__blk906_dn17),)
    } else {
        (locals.var_t9__blk903, locals.var_t9__blk903_dn0, locals.var_t9__blk903_dn2, locals.var_t9__blk903_dn6, locals.var_t9__blk903_dn7, locals.var_t9__blk903_dn10, locals.var_t9__blk903_dn11, locals.var_t9__blk903_dn12, locals.var_t9__blk903_dn17,)
    }
};
        locals.var_t9__blk903 = assign28370_e39988;
        locals.var_t9__blk903_dn0 = assign28370_e39988_d_n0;
        locals.var_t9__blk903_dn2 = assign28370_e39988_d_n2;
        locals.var_t9__blk903_dn6 = assign28370_e39988_d_n6;
        locals.var_t9__blk903_dn7 = assign28370_e39988_d_n7;
        locals.var_t9__blk903_dn10 = assign28370_e39988_d_n10;
        locals.var_t9__blk903_dn11 = assign28370_e39988_d_n11;
        locals.var_t9__blk903_dn12 = assign28370_e39988_d_n12;
        locals.var_t9__blk903_dn17 = assign28370_e39988_d_n17;
        locals.var_t9__blk903_rv = 0.0;

        let (assign28380_e40004, assign28380_e40004_d_n0, assign28380_e40004_d_n2, assign28380_e40004_d_n6, assign28380_e40004_d_n7, assign28380_e40004_d_n10, assign28380_e40004_d_n11, assign28380_e40004_d_n12, assign28380_e40004_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28380_e39996: f64 = (locals.var_vgs - locals.var_vds);
        let assign28380_e39998: f64 = (assign28380_e39996 * locals.var_t5__blk902);
        let assign28380_e40001: f64 = (locals.var_t4__blk901 * locals.var_t9__blk903);
        let assign28380_e40002: f64 = (assign28380_e39998 - assign28380_e40001);
        (assign28380_e40002, ((((-locals.var_vds_dn0) * locals.var_t5__blk902) + (assign28380_e39996 * locals.var_t5__blk902_dn0)) - ((locals.var_t4__blk901_dn0 * locals.var_t9__blk903) + (locals.var_t4__blk901 * locals.var_t9__blk903_dn0))), ((((-locals.var_vds_dn2) * locals.var_t5__blk902) + (assign28380_e39996 * locals.var_t5__blk902_dn2)) - ((locals.var_t4__blk901_dn2 * locals.var_t9__blk903) + (locals.var_t4__blk901 * locals.var_t9__blk903_dn2))), ((((locals.var_vgs_dn6 - locals.var_vds_dn6) * locals.var_t5__blk902) + (assign28380_e39996 * locals.var_t5__blk902_dn6)) - ((locals.var_t4__blk901_dn6 * locals.var_t9__blk903) + (locals.var_t4__blk901 * locals.var_t9__blk903_dn6))), ((((locals.var_vgs_dn7 - locals.var_vds_dn7) * locals.var_t5__blk902) + (assign28380_e39996 * locals.var_t5__blk902_dn7)) - ((locals.var_t4__blk901_dn7 * locals.var_t9__blk903) + (locals.var_t4__blk901 * locals.var_t9__blk903_dn7))), ((((-locals.var_vds_dn10) * locals.var_t5__blk902) + (assign28380_e39996 * locals.var_t5__blk902_dn10)) - ((locals.var_t4__blk901_dn10 * locals.var_t9__blk903) + (locals.var_t4__blk901 * locals.var_t9__blk903_dn10))), ((((locals.var_vgs_dn11 - locals.var_vds_dn11) * locals.var_t5__blk902) + (assign28380_e39996 * locals.var_t5__blk902_dn11)) - ((locals.var_t4__blk901_dn11 * locals.var_t9__blk903) + (locals.var_t4__blk901 * locals.var_t9__blk903_dn11))), ((((-locals.var_vds_dn12) * locals.var_t5__blk902) + (assign28380_e39996 * locals.var_t5__blk902_dn12)) - ((locals.var_t4__blk901_dn12 * locals.var_t9__blk903) + (locals.var_t4__blk901 * locals.var_t9__blk903_dn12))), ((((-locals.var_vds_dn17) * locals.var_t5__blk902) + (assign28380_e39996 * locals.var_t5__blk902_dn17)) - ((locals.var_t4__blk901_dn17 * locals.var_t9__blk903) + (locals.var_t4__blk901 * locals.var_t9__blk903_dn17))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign28380_e40004;
        locals.var_qgod_dn0 = assign28380_e40004_d_n0;
        locals.var_qgod_dn2 = assign28380_e40004_d_n2;
        locals.var_qgod_dn6 = assign28380_e40004_d_n6;
        locals.var_qgod_dn7 = assign28380_e40004_d_n7;
        locals.var_qgod_dn10 = assign28380_e40004_d_n10;
        locals.var_qgod_dn11 = assign28380_e40004_d_n11;
        locals.var_qgod_dn12 = assign28380_e40004_d_n12;
        locals.var_qgod_dn17 = assign28380_e40004_d_n17;
        locals.var_qgod_rv = 0.0;

        let (assign28390_e40018, assign28390_e40018_d_n0, assign28390_e40018_d_n2, assign28390_e40018_d_n6, assign28390_e40018_d_n7, assign28390_e40018_d_n10, assign28390_e40018_d_n11, assign28390_e40018_d_n12, assign28390_e40018_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign28390_e40014: f64 = (locals.var_mks_nover / locals.var_nsub);
        let assign28390_e40015: f64 = (assign28390_e40014).sqrt();
        let assign28390_e40016: f64 = (locals.var_cnst0soi * assign28390_e40015);
        (assign28390_e40016, ((locals.var_cnst0soi_dn0 * assign28390_e40015) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28390_e40015)))), ((locals.var_cnst0soi_dn2 * assign28390_e40015) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28390_e40015)))), ((locals.var_cnst0soi_dn6 * assign28390_e40015) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28390_e40015)))), ((locals.var_cnst0soi_dn7 * assign28390_e40015) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28390_e40015)))), ((locals.var_cnst0soi_dn10 * assign28390_e40015) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28390_e40015)))), ((locals.var_cnst0soi_dn11 * assign28390_e40015) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn11) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28390_e40015)))), ((locals.var_cnst0soi_dn12 * assign28390_e40015) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn12) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28390_e40015)))), ((locals.var_cnst0soi_dn17 * assign28390_e40015) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn17) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28390_e40015)))),)
    } else {
        (locals.var_cnst0over__blk930, locals.var_cnst0over__blk930_dn0, locals.var_cnst0over__blk930_dn2, locals.var_cnst0over__blk930_dn6, locals.var_cnst0over__blk930_dn7, locals.var_cnst0over__blk930_dn10, locals.var_cnst0over__blk930_dn11, locals.var_cnst0over__blk930_dn12, locals.var_cnst0over__blk930_dn17,)
    }
};
        locals.var_cnst0over__blk930 = assign28390_e40018;
        locals.var_cnst0over__blk930_dn0 = assign28390_e40018_d_n0;
        locals.var_cnst0over__blk930_dn2 = assign28390_e40018_d_n2;
        locals.var_cnst0over__blk930_dn6 = assign28390_e40018_d_n6;
        locals.var_cnst0over__blk930_dn7 = assign28390_e40018_d_n7;
        locals.var_cnst0over__blk930_dn10 = assign28390_e40018_d_n10;
        locals.var_cnst0over__blk930_dn11 = assign28390_e40018_d_n11;
        locals.var_cnst0over__blk930_dn12 = assign28390_e40018_d_n12;
        locals.var_cnst0over__blk930_dn17 = assign28390_e40018_d_n17;
        locals.var_cnst0over__blk930_rv = 0.0;

        let (assign28400_e40031,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign28400_e40027: f64 = (1.0 - -1.0);
        let assign28400_e40029: f64 = (assign28400_e40027 / 2.0);
        (assign28400_e40029,)
    } else {
        (locals.var_flg_ovloops__blk914,)
    }
};
        locals.var_flg_ovloops__blk914 = assign28400_e40031;
        locals.var_flg_ovloops__blk914_rv = 0.0;

        let (assign28410_e40044,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign28410_e40040: f64 = (1.0 + -1.0);
        let assign28410_e40042: f64 = (assign28410_e40040 / 2.0);
        (assign28410_e40042,)
    } else {
        (locals.var_flg_ovloopd__blk915,)
    }
};
        locals.var_flg_ovloopd__blk915 = assign28410_e40044;
        locals.var_flg_ovloopd__blk915_rv = 0.0;

        let assign28420_e40047: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard982 = assign28420_e40047;
        locals.var_guard982_rv = 0.0;

        let (assign28430_e40066, assign28430_e40066_d_n0, assign28430_e40066_d_n2, assign28430_e40066_d_n6, assign28430_e40066_d_n7, assign28430_e40066_d_n10, assign28430_e40066_d_n11, assign28430_e40066_d_n12, assign28430_e40066_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28430_e40058: f64 = (locals.var_modenml * locals.var_vbs);
        let assign28430_e40062: f64 = (locals.var_vbs - locals.var_vds);
        let assign28430_e40063: f64 = (locals.var_modervs * assign28430_e40062);
        let assign28430_e40064: f64 = (assign28430_e40058 + assign28430_e40063);
        (assign28430_e40064, ((locals.var_modenml * locals.var_vbs_dn0) + (locals.var_modervs * (locals.var_vbs_dn0 - locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vbs_dn2) + (locals.var_modervs * (locals.var_vbs_dn2 - locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vbs_dn6) + (locals.var_modervs * (locals.var_vbs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vbs_dn7) + (locals.var_modervs * (locals.var_vbs_dn7 - locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vbs_dn10) + (locals.var_modervs * (locals.var_vbs_dn10 - locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vbs_dn11) + (locals.var_modervs * (locals.var_vbs_dn11 - locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vbs_dn12) + (locals.var_modervs * (locals.var_vbs_dn12 - locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vbs_dn17) + (locals.var_modervs * (locals.var_vbs_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_vbsgmt__blk924, locals.var_vbsgmt__blk924_dn0, locals.var_vbsgmt__blk924_dn2, locals.var_vbsgmt__blk924_dn6, locals.var_vbsgmt__blk924_dn7, locals.var_vbsgmt__blk924_dn10, locals.var_vbsgmt__blk924_dn11, locals.var_vbsgmt__blk924_dn12, locals.var_vbsgmt__blk924_dn17,)
    }
};
        locals.var_vbsgmt__blk924 = assign28430_e40066;
        locals.var_vbsgmt__blk924_dn0 = assign28430_e40066_d_n0;
        locals.var_vbsgmt__blk924_dn2 = assign28430_e40066_d_n2;
        locals.var_vbsgmt__blk924_dn6 = assign28430_e40066_d_n6;
        locals.var_vbsgmt__blk924_dn7 = assign28430_e40066_d_n7;
        locals.var_vbsgmt__blk924_dn10 = assign28430_e40066_d_n10;
        locals.var_vbsgmt__blk924_dn11 = assign28430_e40066_d_n11;
        locals.var_vbsgmt__blk924_dn12 = assign28430_e40066_d_n12;
        locals.var_vbsgmt__blk924_dn17 = assign28430_e40066_d_n17;
        locals.var_vbsgmt__blk924_rv = 0.0;

        let (assign28440_e40084, assign28440_e40084_d_n0, assign28440_e40084_d_n2, assign28440_e40084_d_n6, assign28440_e40084_d_n7, assign28440_e40084_d_n10, assign28440_e40084_d_n11, assign28440_e40084_d_n12, assign28440_e40084_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28440_e40077: f64 = (locals.var_modenml * locals.var_vds);
        let assign28440_e40080: f64 = (-locals.var_vds);
        let assign28440_e40081: f64 = (locals.var_modervs * assign28440_e40080);
        let assign28440_e40082: f64 = (assign28440_e40077 + assign28440_e40081);
        (assign28440_e40082, ((locals.var_modenml * locals.var_vds_dn0) + (locals.var_modervs * (-locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vds_dn2) + (locals.var_modervs * (-locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vds_dn6) + (locals.var_modervs * (-locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vds_dn7) + (locals.var_modervs * (-locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vds_dn10) + (locals.var_modervs * (-locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vds_dn11) + (locals.var_modervs * (-locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vds_dn12) + (locals.var_modervs * (-locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vds_dn17) + (locals.var_modervs * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_vdsgmt__blk925, locals.var_vdsgmt__blk925_dn0, locals.var_vdsgmt__blk925_dn2, locals.var_vdsgmt__blk925_dn6, locals.var_vdsgmt__blk925_dn7, locals.var_vdsgmt__blk925_dn10, locals.var_vdsgmt__blk925_dn11, locals.var_vdsgmt__blk925_dn12, locals.var_vdsgmt__blk925_dn17,)
    }
};
        locals.var_vdsgmt__blk925 = assign28440_e40084;
        locals.var_vdsgmt__blk925_dn0 = assign28440_e40084_d_n0;
        locals.var_vdsgmt__blk925_dn2 = assign28440_e40084_d_n2;
        locals.var_vdsgmt__blk925_dn6 = assign28440_e40084_d_n6;
        locals.var_vdsgmt__blk925_dn7 = assign28440_e40084_d_n7;
        locals.var_vdsgmt__blk925_dn10 = assign28440_e40084_d_n10;
        locals.var_vdsgmt__blk925_dn11 = assign28440_e40084_d_n11;
        locals.var_vdsgmt__blk925_dn12 = assign28440_e40084_d_n12;
        locals.var_vdsgmt__blk925_dn17 = assign28440_e40084_d_n17;
        locals.var_vdsgmt__blk925_rv = 0.0;

        let (assign28450_e40103, assign28450_e40103_d_n0, assign28450_e40103_d_n2, assign28450_e40103_d_n6, assign28450_e40103_d_n7, assign28450_e40103_d_n10, assign28450_e40103_d_n11, assign28450_e40103_d_n12, assign28450_e40103_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28450_e40095: f64 = (locals.var_modenml * locals.var_vgs);
        let assign28450_e40099: f64 = (locals.var_vgs - locals.var_vds);
        let assign28450_e40100: f64 = (locals.var_modervs * assign28450_e40099);
        let assign28450_e40101: f64 = (assign28450_e40095 + assign28450_e40100);
        (assign28450_e40101, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgsgmt__blk926, locals.var_vgsgmt__blk926_dn0, locals.var_vgsgmt__blk926_dn2, locals.var_vgsgmt__blk926_dn6, locals.var_vgsgmt__blk926_dn7, locals.var_vgsgmt__blk926_dn10, locals.var_vgsgmt__blk926_dn11, locals.var_vgsgmt__blk926_dn12, locals.var_vgsgmt__blk926_dn17,)
    }
};
        locals.var_vgsgmt__blk926 = assign28450_e40103;
        locals.var_vgsgmt__blk926_dn0 = assign28450_e40103_d_n0;
        locals.var_vgsgmt__blk926_dn2 = assign28450_e40103_d_n2;
        locals.var_vgsgmt__blk926_dn6 = assign28450_e40103_d_n6;
        locals.var_vgsgmt__blk926_dn7 = assign28450_e40103_d_n7;
        locals.var_vgsgmt__blk926_dn10 = assign28450_e40103_d_n10;
        locals.var_vgsgmt__blk926_dn11 = assign28450_e40103_d_n11;
        locals.var_vgsgmt__blk926_dn12 = assign28450_e40103_d_n12;
        locals.var_vgsgmt__blk926_dn17 = assign28450_e40103_d_n17;
        locals.var_vgsgmt__blk926_rv = 0.0;

        let (assign28460_e40116, assign28460_e40116_d_n0, assign28460_e40116_d_n2, assign28460_e40116_d_n6, assign28460_e40116_d_n7, assign28460_e40116_d_n10, assign28460_e40116_d_n11, assign28460_e40116_d_n12, assign28460_e40116_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28460_e40114: f64 = (locals.var_vdsgmt__blk925 - locals.var_vbsgmt__blk924);
        (assign28460_e40114, (locals.var_vdsgmt__blk925_dn0 - locals.var_vbsgmt__blk924_dn0), (locals.var_vdsgmt__blk925_dn2 - locals.var_vbsgmt__blk924_dn2), (locals.var_vdsgmt__blk925_dn6 - locals.var_vbsgmt__blk924_dn6), (locals.var_vdsgmt__blk925_dn7 - locals.var_vbsgmt__blk924_dn7), (locals.var_vdsgmt__blk925_dn10 - locals.var_vbsgmt__blk924_dn10), (locals.var_vdsgmt__blk925_dn11 - locals.var_vbsgmt__blk924_dn11), (locals.var_vdsgmt__blk925_dn12 - locals.var_vbsgmt__blk924_dn12), (locals.var_vdsgmt__blk925_dn17 - locals.var_vbsgmt__blk924_dn17),)
    } else {
        (locals.var_vdbgmt__blk927, locals.var_vdbgmt__blk927_dn0, locals.var_vdbgmt__blk927_dn2, locals.var_vdbgmt__blk927_dn6, locals.var_vdbgmt__blk927_dn7, locals.var_vdbgmt__blk927_dn10, locals.var_vdbgmt__blk927_dn11, locals.var_vdbgmt__blk927_dn12, locals.var_vdbgmt__blk927_dn17,)
    }
};
        locals.var_vdbgmt__blk927 = assign28460_e40116;
        locals.var_vdbgmt__blk927_dn0 = assign28460_e40116_d_n0;
        locals.var_vdbgmt__blk927_dn2 = assign28460_e40116_d_n2;
        locals.var_vdbgmt__blk927_dn6 = assign28460_e40116_d_n6;
        locals.var_vdbgmt__blk927_dn7 = assign28460_e40116_d_n7;
        locals.var_vdbgmt__blk927_dn10 = assign28460_e40116_d_n10;
        locals.var_vdbgmt__blk927_dn11 = assign28460_e40116_d_n11;
        locals.var_vdbgmt__blk927_dn12 = assign28460_e40116_d_n12;
        locals.var_vdbgmt__blk927_dn17 = assign28460_e40116_d_n17;
        locals.var_vdbgmt__blk927_rv = 0.0;

        let (assign28470_e40129, assign28470_e40129_d_n0, assign28470_e40129_d_n2, assign28470_e40129_d_n6, assign28470_e40129_d_n7, assign28470_e40129_d_n10, assign28470_e40129_d_n11, assign28470_e40129_d_n12, assign28470_e40129_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28470_e40127: f64 = (locals.var_vgsgmt__blk926 - locals.var_vbsgmt__blk924);
        (assign28470_e40127, (locals.var_vgsgmt__blk926_dn0 - locals.var_vbsgmt__blk924_dn0), (locals.var_vgsgmt__blk926_dn2 - locals.var_vbsgmt__blk924_dn2), (locals.var_vgsgmt__blk926_dn6 - locals.var_vbsgmt__blk924_dn6), (locals.var_vgsgmt__blk926_dn7 - locals.var_vbsgmt__blk924_dn7), (locals.var_vgsgmt__blk926_dn10 - locals.var_vbsgmt__blk924_dn10), (locals.var_vgsgmt__blk926_dn11 - locals.var_vbsgmt__blk924_dn11), (locals.var_vgsgmt__blk926_dn12 - locals.var_vbsgmt__blk924_dn12), (locals.var_vgsgmt__blk926_dn17 - locals.var_vbsgmt__blk924_dn17),)
    } else {
        (locals.var_vgbgmt__blk929, locals.var_vgbgmt__blk929_dn0, locals.var_vgbgmt__blk929_dn2, locals.var_vgbgmt__blk929_dn6, locals.var_vgbgmt__blk929_dn7, locals.var_vgbgmt__blk929_dn10, locals.var_vgbgmt__blk929_dn11, locals.var_vgbgmt__blk929_dn12, locals.var_vgbgmt__blk929_dn17,)
    }
};
        locals.var_vgbgmt__blk929 = assign28470_e40129;
        locals.var_vgbgmt__blk929_dn0 = assign28470_e40129_d_n0;
        locals.var_vgbgmt__blk929_dn2 = assign28470_e40129_d_n2;
        locals.var_vgbgmt__blk929_dn6 = assign28470_e40129_d_n6;
        locals.var_vgbgmt__blk929_dn7 = assign28470_e40129_d_n7;
        locals.var_vgbgmt__blk929_dn10 = assign28470_e40129_d_n10;
        locals.var_vgbgmt__blk929_dn11 = assign28470_e40129_d_n11;
        locals.var_vgbgmt__blk929_dn12 = assign28470_e40129_d_n12;
        locals.var_vgbgmt__blk929_dn17 = assign28470_e40129_d_n17;
        locals.var_vgbgmt__blk929_rv = 0.0;

        let (assign28480_e40141, assign28480_e40141_d_n0, assign28480_e40141_d_n2, assign28480_e40141_d_n6, assign28480_e40141_d_n7, assign28480_e40141_d_n10, assign28480_e40141_d_n11, assign28480_e40141_d_n12, assign28480_e40141_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28480_e40139: f64 = (-locals.var_vbsgmt__blk924);
        (assign28480_e40139, (-locals.var_vbsgmt__blk924_dn0), (-locals.var_vbsgmt__blk924_dn2), (-locals.var_vbsgmt__blk924_dn6), (-locals.var_vbsgmt__blk924_dn7), (-locals.var_vbsgmt__blk924_dn10), (-locals.var_vbsgmt__blk924_dn11), (-locals.var_vbsgmt__blk924_dn12), (-locals.var_vbsgmt__blk924_dn17),)
    } else {
        (locals.var_vsbgmt__blk928, locals.var_vsbgmt__blk928_dn0, locals.var_vsbgmt__blk928_dn2, locals.var_vsbgmt__blk928_dn6, locals.var_vsbgmt__blk928_dn7, locals.var_vsbgmt__blk928_dn10, locals.var_vsbgmt__blk928_dn11, locals.var_vsbgmt__blk928_dn12, locals.var_vsbgmt__blk928_dn17,)
    }
};
        locals.var_vsbgmt__blk928 = assign28480_e40141;
        locals.var_vsbgmt__blk928_dn0 = assign28480_e40141_d_n0;
        locals.var_vsbgmt__blk928_dn2 = assign28480_e40141_d_n2;
        locals.var_vsbgmt__blk928_dn6 = assign28480_e40141_d_n6;
        locals.var_vsbgmt__blk928_dn7 = assign28480_e40141_d_n7;
        locals.var_vsbgmt__blk928_dn10 = assign28480_e40141_d_n10;
        locals.var_vsbgmt__blk928_dn11 = assign28480_e40141_d_n11;
        locals.var_vsbgmt__blk928_dn12 = assign28480_e40141_d_n12;
        locals.var_vsbgmt__blk928_dn17 = assign28480_e40141_d_n17;
        locals.var_vsbgmt__blk928_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_103(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28490_e40158,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28490_e40152: f64 = (locals.var_flg_ovloops__blk914 * locals.var_modenml);
        let assign28490_e40155: f64 = (locals.var_flg_ovloopd__blk915 * locals.var_modervs);
        let assign28490_e40156: f64 = (assign28490_e40152 + assign28490_e40155);
        (assign28490_e40156,)
    } else {
        (locals.var_flg_overs__blk916,)
    }
};
        locals.var_flg_overs__blk916 = assign28490_e40158;
        locals.var_flg_overs__blk916_rv = 0.0;

        let (assign28500_e40175,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28500_e40169: f64 = (locals.var_flg_ovloops__blk914 * locals.var_modervs);
        let assign28500_e40172: f64 = (locals.var_flg_ovloopd__blk915 * locals.var_modenml);
        let assign28500_e40173: f64 = (assign28500_e40169 + assign28500_e40172);
        (assign28500_e40173,)
    } else {
        (locals.var_flg_overd__blk917,)
    }
};
        locals.var_flg_overd__blk917 = assign28500_e40175;
        locals.var_flg_overd__blk917_rv = 0.0;

        let (assign28510_e40196, assign28510_e40196_d_n0, assign28510_e40196_d_n2, assign28510_e40196_d_n6, assign28510_e40196_d_n7, assign28510_e40196_d_n10, assign28510_e40196_d_n11, assign28510_e40196_d_n12, assign28510_e40196_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28510_e40186: f64 = (locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928);
        let assign28510_e40189: f64 = (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927);
        let assign28510_e40190: f64 = (assign28510_e40186 + assign28510_e40189);
        let assign28510_e40193: f64 = (10.0 * 2.220446049250313e-16);
        let assign28510_e40194: f64 = (assign28510_e40190 + assign28510_e40193);
        (assign28510_e40194, ((locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928_dn0) + (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927_dn0)), ((locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928_dn2) + (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927_dn2)), ((locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928_dn6) + (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927_dn6)), ((locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928_dn7) + (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927_dn7)), ((locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928_dn10) + (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927_dn10)), ((locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928_dn11) + (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927_dn11)), ((locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928_dn12) + (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927_dn12)), ((locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928_dn17) + (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927_dn17)),)
    } else {
        (locals.var_vxbgmt__blk922, locals.var_vxbgmt__blk922_dn0, locals.var_vxbgmt__blk922_dn2, locals.var_vxbgmt__blk922_dn6, locals.var_vxbgmt__blk922_dn7, locals.var_vxbgmt__blk922_dn10, locals.var_vxbgmt__blk922_dn11, locals.var_vxbgmt__blk922_dn12, locals.var_vxbgmt__blk922_dn17,)
    }
};
        locals.var_vxbgmt__blk922 = assign28510_e40196;
        locals.var_vxbgmt__blk922_dn0 = assign28510_e40196_d_n0;
        locals.var_vxbgmt__blk922_dn2 = assign28510_e40196_d_n2;
        locals.var_vxbgmt__blk922_dn6 = assign28510_e40196_d_n6;
        locals.var_vxbgmt__blk922_dn7 = assign28510_e40196_d_n7;
        locals.var_vxbgmt__blk922_dn10 = assign28510_e40196_d_n10;
        locals.var_vxbgmt__blk922_dn11 = assign28510_e40196_d_n11;
        locals.var_vxbgmt__blk922_dn12 = assign28510_e40196_d_n12;
        locals.var_vxbgmt__blk922_dn17 = assign28510_e40196_d_n17;
        locals.var_vxbgmt__blk922_rv = 0.0;

        let (assign28520_e40214,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard982 == 0.0)) {
        let assign28520_e40208: f64 = (locals.var_flg_ovloops__blk914 * locals.var_modenml);
        let assign28520_e40211: f64 = (locals.var_flg_ovloopd__blk915 * locals.var_modervs);
        let assign28520_e40212: f64 = (assign28520_e40208 + assign28520_e40211);
        (assign28520_e40212,)
    } else {
        (locals.var_flg_overs__blk916,)
    }
};
        locals.var_flg_overs__blk916 = assign28520_e40214;
        locals.var_flg_overs__blk916_rv = 0.0;

        let (assign28530_e40232,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard982 == 0.0)) {
        let assign28530_e40226: f64 = (locals.var_flg_ovloops__blk914 * locals.var_modervs);
        let assign28530_e40229: f64 = (locals.var_flg_ovloopd__blk915 * locals.var_modenml);
        let assign28530_e40230: f64 = (assign28530_e40226 + assign28530_e40229);
        (assign28530_e40230,)
    } else {
        (locals.var_flg_overd__blk917,)
    }
};
        locals.var_flg_overd__blk917 = assign28530_e40232;
        locals.var_flg_overd__blk917_rv = 0.0;

        let (assign28540_e40254, assign28540_e40254_d_n0, assign28540_e40254_d_n2, assign28540_e40254_d_n6, assign28540_e40254_d_n7, assign28540_e40254_d_n10, assign28540_e40254_d_n11, assign28540_e40254_d_n12, assign28540_e40254_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_flg_ovloops__blk914 != 0.0)) {
        let assign28540_e40246: f64 = (locals.var_modenml * locals.var_vgs);
        let assign28540_e40250: f64 = (locals.var_vgs - locals.var_vds);
        let assign28540_e40251: f64 = (locals.var_modervs * assign28540_e40250);
        let assign28540_e40252: f64 = (assign28540_e40246 + assign28540_e40251);
        (assign28540_e40252, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgbgmt__blk929, locals.var_vgbgmt__blk929_dn0, locals.var_vgbgmt__blk929_dn2, locals.var_vgbgmt__blk929_dn6, locals.var_vgbgmt__blk929_dn7, locals.var_vgbgmt__blk929_dn10, locals.var_vgbgmt__blk929_dn11, locals.var_vgbgmt__blk929_dn12, locals.var_vgbgmt__blk929_dn17,)
    }
};
        locals.var_vgbgmt__blk929 = assign28540_e40254;
        locals.var_vgbgmt__blk929_dn0 = assign28540_e40254_d_n0;
        locals.var_vgbgmt__blk929_dn2 = assign28540_e40254_d_n2;
        locals.var_vgbgmt__blk929_dn6 = assign28540_e40254_d_n6;
        locals.var_vgbgmt__blk929_dn7 = assign28540_e40254_d_n7;
        locals.var_vgbgmt__blk929_dn10 = assign28540_e40254_d_n10;
        locals.var_vgbgmt__blk929_dn11 = assign28540_e40254_d_n11;
        locals.var_vgbgmt__blk929_dn12 = assign28540_e40254_d_n12;
        locals.var_vgbgmt__blk929_dn17 = assign28540_e40254_d_n17;
        locals.var_vgbgmt__blk929_rv = 0.0;

        let (assign28550_e40276, assign28550_e40276_d_n0, assign28550_e40276_d_n2, assign28550_e40276_d_n6, assign28550_e40276_d_n7, assign28550_e40276_d_n10, assign28550_e40276_d_n11, assign28550_e40276_d_n12, assign28550_e40276_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_flg_ovloopd__blk915 != 0.0)) {
        let assign28550_e40268: f64 = (locals.var_modervs * locals.var_vgs);
        let assign28550_e40272: f64 = (locals.var_vgs - locals.var_vds);
        let assign28550_e40273: f64 = (locals.var_modenml * assign28550_e40272);
        let assign28550_e40274: f64 = (assign28550_e40268 + assign28550_e40273);
        (assign28550_e40274, (locals.var_modenml * (-locals.var_vds_dn0)), (locals.var_modenml * (-locals.var_vds_dn2)), ((locals.var_modervs * locals.var_vgs_dn6) + (locals.var_modenml * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modervs * locals.var_vgs_dn7) + (locals.var_modenml * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modenml * (-locals.var_vds_dn10)), ((locals.var_modervs * locals.var_vgs_dn11) + (locals.var_modenml * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modenml * (-locals.var_vds_dn12)), (locals.var_modenml * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgbgmt__blk929, locals.var_vgbgmt__blk929_dn0, locals.var_vgbgmt__blk929_dn2, locals.var_vgbgmt__blk929_dn6, locals.var_vgbgmt__blk929_dn7, locals.var_vgbgmt__blk929_dn10, locals.var_vgbgmt__blk929_dn11, locals.var_vgbgmt__blk929_dn12, locals.var_vgbgmt__blk929_dn17,)
    }
};
        locals.var_vgbgmt__blk929 = assign28550_e40276;
        locals.var_vgbgmt__blk929_dn0 = assign28550_e40276_d_n0;
        locals.var_vgbgmt__blk929_dn2 = assign28550_e40276_d_n2;
        locals.var_vgbgmt__blk929_dn6 = assign28550_e40276_d_n6;
        locals.var_vgbgmt__blk929_dn7 = assign28550_e40276_d_n7;
        locals.var_vgbgmt__blk929_dn10 = assign28550_e40276_d_n10;
        locals.var_vgbgmt__blk929_dn11 = assign28550_e40276_d_n11;
        locals.var_vgbgmt__blk929_dn12 = assign28550_e40276_d_n12;
        locals.var_vgbgmt__blk929_dn17 = assign28550_e40276_d_n17;
        locals.var_vgbgmt__blk929_rv = 0.0;

        let (assign28560_e40288, assign28560_e40288_d_n0, assign28560_e40288_d_n2, assign28560_e40288_d_n6, assign28560_e40288_d_n7, assign28560_e40288_d_n10, assign28560_e40288_d_n11, assign28560_e40288_d_n12, assign28560_e40288_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard982 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt__blk922, locals.var_vxbgmt__blk922_dn0, locals.var_vxbgmt__blk922_dn2, locals.var_vxbgmt__blk922_dn6, locals.var_vxbgmt__blk922_dn7, locals.var_vxbgmt__blk922_dn10, locals.var_vxbgmt__blk922_dn11, locals.var_vxbgmt__blk922_dn12, locals.var_vxbgmt__blk922_dn17,)
    }
};
        locals.var_vxbgmt__blk922 = assign28560_e40288;
        locals.var_vxbgmt__blk922_dn0 = assign28560_e40288_d_n0;
        locals.var_vxbgmt__blk922_dn2 = assign28560_e40288_d_n2;
        locals.var_vxbgmt__blk922_dn6 = assign28560_e40288_d_n6;
        locals.var_vxbgmt__blk922_dn7 = assign28560_e40288_d_n7;
        locals.var_vxbgmt__blk922_dn10 = assign28560_e40288_d_n10;
        locals.var_vxbgmt__blk922_dn11 = assign28560_e40288_d_n11;
        locals.var_vxbgmt__blk922_dn12 = assign28560_e40288_d_n12;
        locals.var_vxbgmt__blk922_dn17 = assign28560_e40288_d_n17;
        locals.var_vxbgmt__blk922_rv = 0.0;

        let (assign28570_e40298, assign28570_e40298_d_n0, assign28570_e40298_d_n2, assign28570_e40298_d_n6, assign28570_e40298_d_n7, assign28570_e40298_d_n10, assign28570_e40298_d_n11, assign28570_e40298_d_n12, assign28570_e40298_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign28570_e40296: f64 = (-locals.var_vxbgmt__blk922);
        (assign28570_e40296, (-locals.var_vxbgmt__blk922_dn0), (-locals.var_vxbgmt__blk922_dn2), (-locals.var_vxbgmt__blk922_dn6), (-locals.var_vxbgmt__blk922_dn7), (-locals.var_vxbgmt__blk922_dn10), (-locals.var_vxbgmt__blk922_dn11), (-locals.var_vxbgmt__blk922_dn12), (-locals.var_vxbgmt__blk922_dn17),)
    } else {
        (locals.var_t0__blk897, locals.var_t0__blk897_dn0, locals.var_t0__blk897_dn2, locals.var_t0__blk897_dn6, locals.var_t0__blk897_dn7, locals.var_t0__blk897_dn10, locals.var_t0__blk897_dn11, locals.var_t0__blk897_dn12, locals.var_t0__blk897_dn17,)
    }
};
        locals.var_t0__blk897 = assign28570_e40298;
        locals.var_t0__blk897_dn0 = assign28570_e40298_d_n0;
        locals.var_t0__blk897_dn2 = assign28570_e40298_d_n2;
        locals.var_t0__blk897_dn6 = assign28570_e40298_d_n6;
        locals.var_t0__blk897_dn7 = assign28570_e40298_d_n7;
        locals.var_t0__blk897_dn10 = assign28570_e40298_d_n10;
        locals.var_t0__blk897_dn11 = assign28570_e40298_d_n11;
        locals.var_t0__blk897_dn12 = assign28570_e40298_d_n12;
        locals.var_t0__blk897_dn17 = assign28570_e40298_d_n17;
        locals.var_t0__blk897_rv = 0.0;

        let assign28580_e40301: f64 = if locals.var_t0__blk897 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard983 = assign28580_e40301;
        locals.var_guard983_rv = 0.0;

        let (assign28590_e40314, assign28590_e40314_d_n0, assign28590_e40314_d_n2, assign28590_e40314_d_n6, assign28590_e40314_d_n7, assign28590_e40314_d_n10, assign28590_e40314_d_n11, assign28590_e40314_d_n12, assign28590_e40314_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28590_e40312: f64 = (locals.var_t0__blk897 - locals.var_vbs_bnd);
        (assign28590_e40312, locals.var_t0__blk897_dn0, locals.var_t0__blk897_dn2, locals.var_t0__blk897_dn6, locals.var_t0__blk897_dn7, locals.var_t0__blk897_dn10, locals.var_t0__blk897_dn11, locals.var_t0__blk897_dn12, locals.var_t0__blk897_dn17,)
    } else {
        (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17,)
    }
};
        locals.var_t1__blk898 = assign28590_e40314;
        locals.var_t1__blk898_dn0 = assign28590_e40314_d_n0;
        locals.var_t1__blk898_dn2 = assign28590_e40314_d_n2;
        locals.var_t1__blk898_dn6 = assign28590_e40314_d_n6;
        locals.var_t1__blk898_dn7 = assign28590_e40314_d_n7;
        locals.var_t1__blk898_dn10 = assign28590_e40314_d_n10;
        locals.var_t1__blk898_dn11 = assign28590_e40314_d_n11;
        locals.var_t1__blk898_dn12 = assign28590_e40314_d_n12;
        locals.var_t1__blk898_dn17 = assign28590_e40314_d_n17;
        locals.var_t1__blk898_rv = 0.0;

        let (assign28600_e40327, assign28600_e40327_d_n0, assign28600_e40327_d_n2, assign28600_e40327_d_n6, assign28600_e40327_d_n7, assign28600_e40327_d_n10, assign28600_e40327_d_n11, assign28600_e40327_d_n12, assign28600_e40327_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28600_e40325: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign28600_e40325, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk899, locals.var_t2__blk899_dn0, locals.var_t2__blk899_dn2, locals.var_t2__blk899_dn6, locals.var_t2__blk899_dn7, locals.var_t2__blk899_dn10, locals.var_t2__blk899_dn11, locals.var_t2__blk899_dn12, locals.var_t2__blk899_dn17,)
    }
};
        locals.var_t2__blk899 = assign28600_e40327;
        locals.var_t2__blk899_dn0 = assign28600_e40327_d_n0;
        locals.var_t2__blk899_dn2 = assign28600_e40327_d_n2;
        locals.var_t2__blk899_dn6 = assign28600_e40327_d_n6;
        locals.var_t2__blk899_dn7 = assign28600_e40327_d_n7;
        locals.var_t2__blk899_dn10 = assign28600_e40327_d_n10;
        locals.var_t2__blk899_dn11 = assign28600_e40327_d_n11;
        locals.var_t2__blk899_dn12 = assign28600_e40327_d_n12;
        locals.var_t2__blk899_dn17 = assign28600_e40327_d_n17;
        locals.var_t2__blk899_rv = 0.0;

        let (assign28610_e40340, assign28610_e40340_d_n0, assign28610_e40340_d_n2, assign28610_e40340_d_n6, assign28610_e40340_d_n7, assign28610_e40340_d_n10, assign28610_e40340_d_n11, assign28610_e40340_d_n12, assign28610_e40340_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28610_e40338: f64 = (locals.var_t1__blk898 / locals.var_t2__blk899);
        (assign28610_e40338, (((locals.var_t1__blk898_dn0 * locals.var_t2__blk899) - (locals.var_t1__blk898 * locals.var_t2__blk899_dn0)) / (locals.var_t2__blk899 * locals.var_t2__blk899)), (((locals.var_t1__blk898_dn2 * locals.var_t2__blk899) - (locals.var_t1__blk898 * locals.var_t2__blk899_dn2)) / (locals.var_t2__blk899 * locals.var_t2__blk899)), (((locals.var_t1__blk898_dn6 * locals.var_t2__blk899) - (locals.var_t1__blk898 * locals.var_t2__blk899_dn6)) / (locals.var_t2__blk899 * locals.var_t2__blk899)), (((locals.var_t1__blk898_dn7 * locals.var_t2__blk899) - (locals.var_t1__blk898 * locals.var_t2__blk899_dn7)) / (locals.var_t2__blk899 * locals.var_t2__blk899)), (((locals.var_t1__blk898_dn10 * locals.var_t2__blk899) - (locals.var_t1__blk898 * locals.var_t2__blk899_dn10)) / (locals.var_t2__blk899 * locals.var_t2__blk899)), (((locals.var_t1__blk898_dn11 * locals.var_t2__blk899) - (locals.var_t1__blk898 * locals.var_t2__blk899_dn11)) / (locals.var_t2__blk899 * locals.var_t2__blk899)), (((locals.var_t1__blk898_dn12 * locals.var_t2__blk899) - (locals.var_t1__blk898 * locals.var_t2__blk899_dn12)) / (locals.var_t2__blk899 * locals.var_t2__blk899)), (((locals.var_t1__blk898_dn17 * locals.var_t2__blk899) - (locals.var_t1__blk898 * locals.var_t2__blk899_dn17)) / (locals.var_t2__blk899 * locals.var_t2__blk899)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign28610_e40340;
        locals.var_tmf1_dn0 = assign28610_e40340_d_n0;
        locals.var_tmf1_dn2 = assign28610_e40340_d_n2;
        locals.var_tmf1_dn6 = assign28610_e40340_d_n6;
        locals.var_tmf1_dn7 = assign28610_e40340_d_n7;
        locals.var_tmf1_dn10 = assign28610_e40340_d_n10;
        locals.var_tmf1_dn11 = assign28610_e40340_d_n11;
        locals.var_tmf1_dn12 = assign28610_e40340_d_n12;
        locals.var_tmf1_dn17 = assign28610_e40340_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign28620_e40353, assign28620_e40353_d_n0, assign28620_e40353_d_n2, assign28620_e40353_d_n6, assign28620_e40353_d_n7, assign28620_e40353_d_n10, assign28620_e40353_d_n11, assign28620_e40353_d_n12, assign28620_e40353_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28620_e40351: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign28620_e40351, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign28620_e40353;
        locals.var_tmf2_dn0 = assign28620_e40353_d_n0;
        locals.var_tmf2_dn2 = assign28620_e40353_d_n2;
        locals.var_tmf2_dn6 = assign28620_e40353_d_n6;
        locals.var_tmf2_dn7 = assign28620_e40353_d_n7;
        locals.var_tmf2_dn10 = assign28620_e40353_d_n10;
        locals.var_tmf2_dn11 = assign28620_e40353_d_n11;
        locals.var_tmf2_dn12 = assign28620_e40353_d_n12;
        locals.var_tmf2_dn17 = assign28620_e40353_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign28630_e40366, assign28630_e40366_d_n0, assign28630_e40366_d_n2, assign28630_e40366_d_n6, assign28630_e40366_d_n7, assign28630_e40366_d_n10, assign28630_e40366_d_n11, assign28630_e40366_d_n12, assign28630_e40366_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28630_e40364: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign28630_e40364, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12, locals.var_tmf3_dn17,)
    }
};
        locals.var_tmf3 = assign28630_e40366;
        locals.var_tmf3_dn0 = assign28630_e40366_d_n0;
        locals.var_tmf3_dn2 = assign28630_e40366_d_n2;
        locals.var_tmf3_dn6 = assign28630_e40366_d_n6;
        locals.var_tmf3_dn7 = assign28630_e40366_d_n7;
        locals.var_tmf3_dn10 = assign28630_e40366_d_n10;
        locals.var_tmf3_dn11 = assign28630_e40366_d_n11;
        locals.var_tmf3_dn12 = assign28630_e40366_d_n12;
        locals.var_tmf3_dn17 = assign28630_e40366_d_n17;
        locals.var_tmf3_rv = 0.0;

        let (assign28640_e40379, assign28640_e40379_d_n0, assign28640_e40379_d_n2, assign28640_e40379_d_n6, assign28640_e40379_d_n7, assign28640_e40379_d_n10, assign28640_e40379_d_n11, assign28640_e40379_d_n12, assign28640_e40379_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28640_e40377: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign28640_e40377, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn17)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn12, locals.var_tmf4_dn17,)
    }
};
        locals.var_tmf4 = assign28640_e40379;
        locals.var_tmf4_dn0 = assign28640_e40379_d_n0;
        locals.var_tmf4_dn2 = assign28640_e40379_d_n2;
        locals.var_tmf4_dn6 = assign28640_e40379_d_n6;
        locals.var_tmf4_dn7 = assign28640_e40379_d_n7;
        locals.var_tmf4_dn10 = assign28640_e40379_d_n10;
        locals.var_tmf4_dn11 = assign28640_e40379_d_n11;
        locals.var_tmf4_dn12 = assign28640_e40379_d_n12;
        locals.var_tmf4_dn17 = assign28640_e40379_d_n17;
        locals.var_tmf4_rv = 0.0;

        let (assign28650_e40400, assign28650_e40400_d_n0, assign28650_e40400_d_n2, assign28650_e40400_d_n6, assign28650_e40400_d_n7, assign28650_e40400_d_n10, assign28650_e40400_d_n11, assign28650_e40400_d_n12, assign28650_e40400_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28650_e40391: f64 = (1.0 + locals.var_tmf1);
        let assign28650_e40393: f64 = (assign28650_e40391 + locals.var_tmf2);
        let assign28650_e40395: f64 = (assign28650_e40393 + locals.var_tmf3);
        let assign28650_e40397: f64 = (assign28650_e40395 + locals.var_tmf4);
        let assign28650_e40398: f64 = (1.0 / assign28650_e40397);
        (assign28650_e40398, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign28650_e40397 * assign28650_e40397))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign28650_e40397 * assign28650_e40397))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign28650_e40397 * assign28650_e40397))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign28650_e40397 * assign28650_e40397))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign28650_e40397 * assign28650_e40397))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign28650_e40397 * assign28650_e40397))), (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign28650_e40397 * assign28650_e40397))), (-((((locals.var_tmf1_dn17 + locals.var_tmf2_dn17) + locals.var_tmf3_dn17) + locals.var_tmf4_dn17) / (assign28650_e40397 * assign28650_e40397))),)
    } else {
        (locals.var_ty__blk907, locals.var_ty__blk907_dn0, locals.var_ty__blk907_dn2, locals.var_ty__blk907_dn6, locals.var_ty__blk907_dn7, locals.var_ty__blk907_dn10, locals.var_ty__blk907_dn11, locals.var_ty__blk907_dn12, locals.var_ty__blk907_dn17,)
    }
};
        locals.var_ty__blk907 = assign28650_e40400;
        locals.var_ty__blk907_dn0 = assign28650_e40400_d_n0;
        locals.var_ty__blk907_dn2 = assign28650_e40400_d_n2;
        locals.var_ty__blk907_dn6 = assign28650_e40400_d_n6;
        locals.var_ty__blk907_dn7 = assign28650_e40400_d_n7;
        locals.var_ty__blk907_dn10 = assign28650_e40400_d_n10;
        locals.var_ty__blk907_dn11 = assign28650_e40400_d_n11;
        locals.var_ty__blk907_dn12 = assign28650_e40400_d_n12;
        locals.var_ty__blk907_dn17 = assign28650_e40400_d_n17;
        locals.var_ty__blk907_rv = 0.0;

        let (assign28670_e40443, assign28670_e40443_d_n0, assign28670_e40443_d_n2, assign28670_e40443_d_n6, assign28670_e40443_d_n7, assign28670_e40443_d_n10, assign28670_e40443_d_n11, assign28670_e40443_d_n12, assign28670_e40443_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28670_e40440: f64 = (1.0 - locals.var_ty__blk907);
        let assign28670_e40441: f64 = (locals.var_t2__blk899 * assign28670_e40440);
        (assign28670_e40441, ((locals.var_t2__blk899_dn0 * assign28670_e40440) + (locals.var_t2__blk899 * (-locals.var_ty__blk907_dn0))), ((locals.var_t2__blk899_dn2 * assign28670_e40440) + (locals.var_t2__blk899 * (-locals.var_ty__blk907_dn2))), ((locals.var_t2__blk899_dn6 * assign28670_e40440) + (locals.var_t2__blk899 * (-locals.var_ty__blk907_dn6))), ((locals.var_t2__blk899_dn7 * assign28670_e40440) + (locals.var_t2__blk899 * (-locals.var_ty__blk907_dn7))), ((locals.var_t2__blk899_dn10 * assign28670_e40440) + (locals.var_t2__blk899 * (-locals.var_ty__blk907_dn10))), ((locals.var_t2__blk899_dn11 * assign28670_e40440) + (locals.var_t2__blk899 * (-locals.var_ty__blk907_dn11))), ((locals.var_t2__blk899_dn12 * assign28670_e40440) + (locals.var_t2__blk899 * (-locals.var_ty__blk907_dn12))), ((locals.var_t2__blk899_dn17 * assign28670_e40440) + (locals.var_t2__blk899 * (-locals.var_ty__blk907_dn17))),)
    } else {
        (locals.var_ty__blk907, locals.var_ty__blk907_dn0, locals.var_ty__blk907_dn2, locals.var_ty__blk907_dn6, locals.var_ty__blk907_dn7, locals.var_ty__blk907_dn10, locals.var_ty__blk907_dn11, locals.var_ty__blk907_dn12, locals.var_ty__blk907_dn17,)
    }
};
        locals.var_ty__blk907 = assign28670_e40443;
        locals.var_ty__blk907_dn0 = assign28670_e40443_d_n0;
        locals.var_ty__blk907_dn2 = assign28670_e40443_d_n2;
        locals.var_ty__blk907_dn6 = assign28670_e40443_d_n6;
        locals.var_ty__blk907_dn7 = assign28670_e40443_d_n7;
        locals.var_ty__blk907_dn10 = assign28670_e40443_d_n10;
        locals.var_ty__blk907_dn11 = assign28670_e40443_d_n11;
        locals.var_ty__blk907_dn12 = assign28670_e40443_d_n12;
        locals.var_ty__blk907_dn17 = assign28670_e40443_d_n17;
        locals.var_ty__blk907_rv = 0.0;

        let (assign28690_e40468, assign28690_e40468_d_n0, assign28690_e40468_d_n2, assign28690_e40468_d_n6, assign28690_e40468_d_n7, assign28690_e40468_d_n10, assign28690_e40468_d_n11, assign28690_e40468_d_n12, assign28690_e40468_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28690_e40466: f64 = (locals.var_vbs_bnd + locals.var_ty__blk907);
        (assign28690_e40466, locals.var_ty__blk907_dn0, locals.var_ty__blk907_dn2, locals.var_ty__blk907_dn6, locals.var_ty__blk907_dn7, locals.var_ty__blk907_dn10, locals.var_ty__blk907_dn11, locals.var_ty__blk907_dn12, locals.var_ty__blk907_dn17,)
    } else {
        (locals.var_t10__blk904, locals.var_t10__blk904_dn0, locals.var_t10__blk904_dn2, locals.var_t10__blk904_dn6, locals.var_t10__blk904_dn7, locals.var_t10__blk904_dn10, locals.var_t10__blk904_dn11, locals.var_t10__blk904_dn12, locals.var_t10__blk904_dn17,)
    }
};
        locals.var_t10__blk904 = assign28690_e40468;
        locals.var_t10__blk904_dn0 = assign28690_e40468_d_n0;
        locals.var_t10__blk904_dn2 = assign28690_e40468_d_n2;
        locals.var_t10__blk904_dn6 = assign28690_e40468_d_n6;
        locals.var_t10__blk904_dn7 = assign28690_e40468_d_n7;
        locals.var_t10__blk904_dn10 = assign28690_e40468_d_n10;
        locals.var_t10__blk904_dn11 = assign28690_e40468_d_n11;
        locals.var_t10__blk904_dn12 = assign28690_e40468_d_n12;
        locals.var_t10__blk904_dn17 = assign28690_e40468_d_n17;
        locals.var_t10__blk904_rv = 0.0;

        let (assign28700_e40480, assign28700_e40480_d_n0, assign28700_e40480_d_n2, assign28700_e40480_d_n6, assign28700_e40480_d_n7, assign28700_e40480_d_n10, assign28700_e40480_d_n11, assign28700_e40480_d_n12, assign28700_e40480_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard983 == 0.0)) {
        (locals.var_t0__blk897, locals.var_t0__blk897_dn0, locals.var_t0__blk897_dn2, locals.var_t0__blk897_dn6, locals.var_t0__blk897_dn7, locals.var_t0__blk897_dn10, locals.var_t0__blk897_dn11, locals.var_t0__blk897_dn12, locals.var_t0__blk897_dn17,)
    } else {
        (locals.var_t10__blk904, locals.var_t10__blk904_dn0, locals.var_t10__blk904_dn2, locals.var_t10__blk904_dn6, locals.var_t10__blk904_dn7, locals.var_t10__blk904_dn10, locals.var_t10__blk904_dn11, locals.var_t10__blk904_dn12, locals.var_t10__blk904_dn17,)
    }
};
        locals.var_t10__blk904 = assign28700_e40480;
        locals.var_t10__blk904_dn0 = assign28700_e40480_d_n0;
        locals.var_t10__blk904_dn2 = assign28700_e40480_d_n2;
        locals.var_t10__blk904_dn6 = assign28700_e40480_d_n6;
        locals.var_t10__blk904_dn7 = assign28700_e40480_d_n7;
        locals.var_t10__blk904_dn10 = assign28700_e40480_d_n10;
        locals.var_t10__blk904_dn11 = assign28700_e40480_d_n11;
        locals.var_t10__blk904_dn12 = assign28700_e40480_d_n12;
        locals.var_t10__blk904_dn17 = assign28700_e40480_d_n17;
        locals.var_t10__blk904_rv = 0.0;

        let (assign28720_e40504, assign28720_e40504_d_n0, assign28720_e40504_d_n2, assign28720_e40504_d_n6, assign28720_e40504_d_n7, assign28720_e40504_d_n10, assign28720_e40504_d_n11, assign28720_e40504_d_n12, assign28720_e40504_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign28720_e40500: f64 = (-locals.var_t10__blk904);
        let assign28720_e40502: f64 = (assign28720_e40500 - 1e-12);
        (assign28720_e40502, (-locals.var_t10__blk904_dn0), (-locals.var_t10__blk904_dn2), (-locals.var_t10__blk904_dn6), (-locals.var_t10__blk904_dn7), (-locals.var_t10__blk904_dn10), (-locals.var_t10__blk904_dn11), (-locals.var_t10__blk904_dn12), (-locals.var_t10__blk904_dn17),)
    } else {
        (locals.var_vxbgmtcl__blk923, locals.var_vxbgmtcl__blk923_dn0, locals.var_vxbgmtcl__blk923_dn2, locals.var_vxbgmtcl__blk923_dn6, locals.var_vxbgmtcl__blk923_dn7, locals.var_vxbgmtcl__blk923_dn10, locals.var_vxbgmtcl__blk923_dn11, locals.var_vxbgmtcl__blk923_dn12, locals.var_vxbgmtcl__blk923_dn17,)
    }
};
        locals.var_vxbgmtcl__blk923 = assign28720_e40504;
        locals.var_vxbgmtcl__blk923_dn0 = assign28720_e40504_d_n0;
        locals.var_vxbgmtcl__blk923_dn2 = assign28720_e40504_d_n2;
        locals.var_vxbgmtcl__blk923_dn6 = assign28720_e40504_d_n6;
        locals.var_vxbgmtcl__blk923_dn7 = assign28720_e40504_d_n7;
        locals.var_vxbgmtcl__blk923_dn10 = assign28720_e40504_d_n10;
        locals.var_vxbgmtcl__blk923_dn11 = assign28720_e40504_d_n11;
        locals.var_vxbgmtcl__blk923_dn12 = assign28720_e40504_d_n12;
        locals.var_vxbgmtcl__blk923_dn17 = assign28720_e40504_d_n17;
        locals.var_vxbgmtcl__blk923_rv = 0.0;

        let (assign28730_e40515, assign28730_e40515_d_n0, assign28730_e40515_d_n2, assign28730_e40515_d_n6, assign28730_e40515_d_n7, assign28730_e40515_d_n10, assign28730_e40515_d_n11, assign28730_e40515_d_n12, assign28730_e40515_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign28730_e40513: f64 = (locals.var_cnst0over__blk930 * locals.var_cox0_inv__blk909);
        (assign28730_e40513, (locals.var_cnst0over__blk930_dn0 * locals.var_cox0_inv__blk909), (locals.var_cnst0over__blk930_dn2 * locals.var_cox0_inv__blk909), (locals.var_cnst0over__blk930_dn6 * locals.var_cox0_inv__blk909), (locals.var_cnst0over__blk930_dn7 * locals.var_cox0_inv__blk909), (locals.var_cnst0over__blk930_dn10 * locals.var_cox0_inv__blk909), (locals.var_cnst0over__blk930_dn11 * locals.var_cox0_inv__blk909), (locals.var_cnst0over__blk930_dn12 * locals.var_cox0_inv__blk909), (locals.var_cnst0over__blk930_dn17 * locals.var_cox0_inv__blk909),)
    } else {
        (locals.var_fac1__blk931, locals.var_fac1__blk931_dn0, locals.var_fac1__blk931_dn2, locals.var_fac1__blk931_dn6, locals.var_fac1__blk931_dn7, locals.var_fac1__blk931_dn10, locals.var_fac1__blk931_dn11, locals.var_fac1__blk931_dn12, locals.var_fac1__blk931_dn17,)
    }
};
        locals.var_fac1__blk931 = assign28730_e40515;
        locals.var_fac1__blk931_dn0 = assign28730_e40515_d_n0;
        locals.var_fac1__blk931_dn2 = assign28730_e40515_d_n2;
        locals.var_fac1__blk931_dn6 = assign28730_e40515_d_n6;
        locals.var_fac1__blk931_dn7 = assign28730_e40515_d_n7;
        locals.var_fac1__blk931_dn10 = assign28730_e40515_d_n10;
        locals.var_fac1__blk931_dn11 = assign28730_e40515_d_n11;
        locals.var_fac1__blk931_dn12 = assign28730_e40515_d_n12;
        locals.var_fac1__blk931_dn17 = assign28730_e40515_d_n17;
        locals.var_fac1__blk931_rv = 0.0;

        let (assign28740_e40526, assign28740_e40526_d_n0, assign28740_e40526_d_n2, assign28740_e40526_d_n6, assign28740_e40526_d_n7, assign28740_e40526_d_n10, assign28740_e40526_d_n11, assign28740_e40526_d_n12, assign28740_e40526_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign28740_e40524: f64 = (locals.var_fac1__blk931 * locals.var_fac1__blk931);
        (assign28740_e40524, ((locals.var_fac1__blk931_dn0 * locals.var_fac1__blk931) + (locals.var_fac1__blk931 * locals.var_fac1__blk931_dn0)), ((locals.var_fac1__blk931_dn2 * locals.var_fac1__blk931) + (locals.var_fac1__blk931 * locals.var_fac1__blk931_dn2)), ((locals.var_fac1__blk931_dn6 * locals.var_fac1__blk931) + (locals.var_fac1__blk931 * locals.var_fac1__blk931_dn6)), ((locals.var_fac1__blk931_dn7 * locals.var_fac1__blk931) + (locals.var_fac1__blk931 * locals.var_fac1__blk931_dn7)), ((locals.var_fac1__blk931_dn10 * locals.var_fac1__blk931) + (locals.var_fac1__blk931 * locals.var_fac1__blk931_dn10)), ((locals.var_fac1__blk931_dn11 * locals.var_fac1__blk931) + (locals.var_fac1__blk931 * locals.var_fac1__blk931_dn11)), ((locals.var_fac1__blk931_dn12 * locals.var_fac1__blk931) + (locals.var_fac1__blk931 * locals.var_fac1__blk931_dn12)), ((locals.var_fac1__blk931_dn17 * locals.var_fac1__blk931) + (locals.var_fac1__blk931 * locals.var_fac1__blk931_dn17)),)
    } else {
        (locals.var_fac1p2__blk932, locals.var_fac1p2__blk932_dn0, locals.var_fac1p2__blk932_dn2, locals.var_fac1p2__blk932_dn6, locals.var_fac1p2__blk932_dn7, locals.var_fac1p2__blk932_dn10, locals.var_fac1p2__blk932_dn11, locals.var_fac1p2__blk932_dn12, locals.var_fac1p2__blk932_dn17,)
    }
};
        locals.var_fac1p2__blk932 = assign28740_e40526;
        locals.var_fac1p2__blk932_dn0 = assign28740_e40526_d_n0;
        locals.var_fac1p2__blk932_dn2 = assign28740_e40526_d_n2;
        locals.var_fac1p2__blk932_dn6 = assign28740_e40526_d_n6;
        locals.var_fac1p2__blk932_dn7 = assign28740_e40526_d_n7;
        locals.var_fac1p2__blk932_dn10 = assign28740_e40526_d_n10;
        locals.var_fac1p2__blk932_dn11 = assign28740_e40526_d_n11;
        locals.var_fac1p2__blk932_dn12 = assign28740_e40526_d_n12;
        locals.var_fac1p2__blk932_dn17 = assign28740_e40526_d_n17;
        locals.var_fac1p2__blk932_rv = 0.0;

        let (assign28750_e40538, assign28750_e40538_d_n0, assign28750_e40538_d_n2, assign28750_e40538_d_n6, assign28750_e40538_d_n7, assign28750_e40538_d_n10, assign28750_e40538_d_n11, assign28750_e40538_d_n12, assign28750_e40538_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign28750_e40534: f64 = (-locals.var_vgbgmt__blk929);
        let assign28750_e40536: f64 = (assign28750_e40534 + locals.var_uc_vfbover);
        (assign28750_e40536, (-locals.var_vgbgmt__blk929_dn0), (-locals.var_vgbgmt__blk929_dn2), (-locals.var_vgbgmt__blk929_dn6), (-locals.var_vgbgmt__blk929_dn7), (-locals.var_vgbgmt__blk929_dn10), (-locals.var_vgbgmt__blk929_dn11), (-locals.var_vgbgmt__blk929_dn12), (-locals.var_vgbgmt__blk929_dn17),)
    } else {
        (locals.var_vgpld__blk933, locals.var_vgpld__blk933_dn0, locals.var_vgpld__blk933_dn2, locals.var_vgpld__blk933_dn6, locals.var_vgpld__blk933_dn7, locals.var_vgpld__blk933_dn10, locals.var_vgpld__blk933_dn11, locals.var_vgpld__blk933_dn12, locals.var_vgpld__blk933_dn17,)
    }
};
        locals.var_vgpld__blk933 = assign28750_e40538;
        locals.var_vgpld__blk933_dn0 = assign28750_e40538_d_n0;
        locals.var_vgpld__blk933_dn2 = assign28750_e40538_d_n2;
        locals.var_vgpld__blk933_dn6 = assign28750_e40538_d_n6;
        locals.var_vgpld__blk933_dn7 = assign28750_e40538_d_n7;
        locals.var_vgpld__blk933_dn10 = assign28750_e40538_d_n10;
        locals.var_vgpld__blk933_dn11 = assign28750_e40538_d_n11;
        locals.var_vgpld__blk933_dn12 = assign28750_e40538_d_n12;
        locals.var_vgpld__blk933_dn17 = assign28750_e40538_d_n17;
        locals.var_vgpld__blk933_rv = 0.0;

        let (assign28760_e40549, assign28760_e40549_d_n0, assign28760_e40549_d_n2, assign28760_e40549_d_n6, assign28760_e40549_d_n7, assign28760_e40549_d_n10, assign28760_e40549_d_n11, assign28760_e40549_d_n12, assign28760_e40549_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign28760_e40547: f64 = (locals.var_mks_nover / locals.var_nin);
        (assign28760_e40547, (-((locals.var_mks_nover * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn12) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn17) / (locals.var_nin * locals.var_nin))),)
    } else {
        (locals.var_t0__blk897, locals.var_t0__blk897_dn0, locals.var_t0__blk897_dn2, locals.var_t0__blk897_dn6, locals.var_t0__blk897_dn7, locals.var_t0__blk897_dn10, locals.var_t0__blk897_dn11, locals.var_t0__blk897_dn12, locals.var_t0__blk897_dn17,)
    }
};
        locals.var_t0__blk897 = assign28760_e40549;
        locals.var_t0__blk897_dn0 = assign28760_e40549_d_n0;
        locals.var_t0__blk897_dn2 = assign28760_e40549_d_n2;
        locals.var_t0__blk897_dn6 = assign28760_e40549_d_n6;
        locals.var_t0__blk897_dn7 = assign28760_e40549_d_n7;
        locals.var_t0__blk897_dn10 = assign28760_e40549_d_n10;
        locals.var_t0__blk897_dn11 = assign28760_e40549_d_n11;
        locals.var_t0__blk897_dn12 = assign28760_e40549_d_n12;
        locals.var_t0__blk897_dn17 = assign28760_e40549_d_n17;
        locals.var_t0__blk897_rv = 0.0;

        let (assign28770_e40563, assign28770_e40563_d_n0, assign28770_e40563_d_n2, assign28770_e40563_d_n6, assign28770_e40563_d_n7, assign28770_e40563_d_n10, assign28770_e40563_d_n11, assign28770_e40563_d_n12, assign28770_e40563_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign28770_e40558: f64 = (2.0 / locals.var_beta);
        let assign28770_e40560: f64 = (locals.var_t0__blk897).ln();
        let assign28770_e40561: f64 = (assign28770_e40558 * assign28770_e40560);
        (assign28770_e40561, (assign28770_e40558 * (locals.var_t0__blk897_dn0 / locals.var_t0__blk897)), (assign28770_e40558 * (locals.var_t0__blk897_dn2 / locals.var_t0__blk897)), (assign28770_e40558 * (locals.var_t0__blk897_dn6 / locals.var_t0__blk897)), (assign28770_e40558 * (locals.var_t0__blk897_dn7 / locals.var_t0__blk897)), (((-((2.0 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign28770_e40560) + (assign28770_e40558 * (locals.var_t0__blk897_dn10 / locals.var_t0__blk897))), (assign28770_e40558 * (locals.var_t0__blk897_dn11 / locals.var_t0__blk897)), (assign28770_e40558 * (locals.var_t0__blk897_dn12 / locals.var_t0__blk897)), (assign28770_e40558 * (locals.var_t0__blk897_dn17 / locals.var_t0__blk897)),)
    } else {
        (locals.var_pb2over__blk934, locals.var_pb2over__blk934_dn0, locals.var_pb2over__blk934_dn2, locals.var_pb2over__blk934_dn6, locals.var_pb2over__blk934_dn7, locals.var_pb2over__blk934_dn10, locals.var_pb2over__blk934_dn11, locals.var_pb2over__blk934_dn12, locals.var_pb2over__blk934_dn17,)
    }
};
        locals.var_pb2over__blk934 = assign28770_e40563;
        locals.var_pb2over__blk934_dn0 = assign28770_e40563_d_n0;
        locals.var_pb2over__blk934_dn2 = assign28770_e40563_d_n2;
        locals.var_pb2over__blk934_dn6 = assign28770_e40563_d_n6;
        locals.var_pb2over__blk934_dn7 = assign28770_e40563_d_n7;
        locals.var_pb2over__blk934_dn10 = assign28770_e40563_d_n10;
        locals.var_pb2over__blk934_dn11 = assign28770_e40563_d_n11;
        locals.var_pb2over__blk934_dn12 = assign28770_e40563_d_n12;
        locals.var_pb2over__blk934_dn17 = assign28770_e40563_d_n17;
        locals.var_pb2over__blk934_rv = 0.0;

        let (assign28780_e40573, assign28780_e40573_d_n0, assign28780_e40573_d_n2, assign28780_e40573_d_n6, assign28780_e40573_d_n7, assign28780_e40573_d_n10, assign28780_e40573_d_n11, assign28780_e40573_d_n12, assign28780_e40573_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign28780_e40571: f64 = (-locals.var_vxbgmtcl__blk923);
        (assign28780_e40571, (-locals.var_vxbgmtcl__blk923_dn0), (-locals.var_vxbgmtcl__blk923_dn2), (-locals.var_vxbgmtcl__blk923_dn6), (-locals.var_vxbgmtcl__blk923_dn7), (-locals.var_vxbgmtcl__blk923_dn10), (-locals.var_vxbgmtcl__blk923_dn11), (-locals.var_vxbgmtcl__blk923_dn12), (-locals.var_vxbgmtcl__blk923_dn17),)
    } else {
        (locals.var_vgb_fb_ld__blk935, locals.var_vgb_fb_ld__blk935_dn0, locals.var_vgb_fb_ld__blk935_dn2, locals.var_vgb_fb_ld__blk935_dn6, locals.var_vgb_fb_ld__blk935_dn7, locals.var_vgb_fb_ld__blk935_dn10, locals.var_vgb_fb_ld__blk935_dn11, locals.var_vgb_fb_ld__blk935_dn12, locals.var_vgb_fb_ld__blk935_dn17,)
    }
};
        locals.var_vgb_fb_ld__blk935 = assign28780_e40573;
        locals.var_vgb_fb_ld__blk935_dn0 = assign28780_e40573_d_n0;
        locals.var_vgb_fb_ld__blk935_dn2 = assign28780_e40573_d_n2;
        locals.var_vgb_fb_ld__blk935_dn6 = assign28780_e40573_d_n6;
        locals.var_vgb_fb_ld__blk935_dn7 = assign28780_e40573_d_n7;
        locals.var_vgb_fb_ld__blk935_dn10 = assign28780_e40573_d_n10;
        locals.var_vgb_fb_ld__blk935_dn11 = assign28780_e40573_d_n11;
        locals.var_vgb_fb_ld__blk935_dn12 = assign28780_e40573_d_n12;
        locals.var_vgb_fb_ld__blk935_dn17 = assign28780_e40573_d_n17;
        locals.var_vgb_fb_ld__blk935_rv = 0.0;

        let assign28790_e40576: f64 = if locals.var_vgpld__blk933 < locals.var_vgb_fb_ld__blk935 { 1.0 } else { 0.0 };
        locals.var_guard984 = assign28790_e40576;
        locals.var_guard984_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_104(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28810_e40603, assign28810_e40603_d_n0, assign28810_e40603_d_n2, assign28810_e40603_d_n6, assign28810_e40603_d_n7, assign28810_e40603_d_n10, assign28810_e40603_d_n11, assign28810_e40603_d_n12, assign28810_e40603_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28810_e40600: f64 = (locals.var_beta * locals.var_cnst0over__blk930);
        let assign28810_e40601: f64 = (1.0 / assign28810_e40600);
        (assign28810_e40601, (-((locals.var_beta * locals.var_cnst0over__blk930_dn0) / (assign28810_e40600 * assign28810_e40600))), (-((locals.var_beta * locals.var_cnst0over__blk930_dn2) / (assign28810_e40600 * assign28810_e40600))), (-((locals.var_beta * locals.var_cnst0over__blk930_dn6) / (assign28810_e40600 * assign28810_e40600))), (-((locals.var_beta * locals.var_cnst0over__blk930_dn7) / (assign28810_e40600 * assign28810_e40600))), (-(((locals.var_beta_dn10 * locals.var_cnst0over__blk930) + (locals.var_beta * locals.var_cnst0over__blk930_dn10)) / (assign28810_e40600 * assign28810_e40600))), (-((locals.var_beta * locals.var_cnst0over__blk930_dn11) / (assign28810_e40600 * assign28810_e40600))), (-((locals.var_beta * locals.var_cnst0over__blk930_dn12) / (assign28810_e40600 * assign28810_e40600))), (-((locals.var_beta * locals.var_cnst0over__blk930_dn17) / (assign28810_e40600 * assign28810_e40600))),)
    } else {
        (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17,)
    }
};
        locals.var_t1__blk898 = assign28810_e40603;
        locals.var_t1__blk898_dn0 = assign28810_e40603_d_n0;
        locals.var_t1__blk898_dn2 = assign28810_e40603_d_n2;
        locals.var_t1__blk898_dn6 = assign28810_e40603_d_n6;
        locals.var_t1__blk898_dn7 = assign28810_e40603_d_n7;
        locals.var_t1__blk898_dn10 = assign28810_e40603_d_n10;
        locals.var_t1__blk898_dn11 = assign28810_e40603_d_n11;
        locals.var_t1__blk898_dn12 = assign28810_e40603_d_n12;
        locals.var_t1__blk898_dn17 = assign28810_e40603_d_n17;
        locals.var_t1__blk898_rv = 0.0;

        let (assign28820_e40616, assign28820_e40616_d_n0, assign28820_e40616_d_n2, assign28820_e40616_d_n6, assign28820_e40616_d_n7, assign28820_e40616_d_n10, assign28820_e40616_d_n11, assign28820_e40616_d_n12, assign28820_e40616_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28820_e40614: f64 = (locals.var_t1__blk898 * locals.var_cox0__blk908);
        (assign28820_e40614, (locals.var_t1__blk898_dn0 * locals.var_cox0__blk908), (locals.var_t1__blk898_dn2 * locals.var_cox0__blk908), (locals.var_t1__blk898_dn6 * locals.var_cox0__blk908), (locals.var_t1__blk898_dn7 * locals.var_cox0__blk908), (locals.var_t1__blk898_dn10 * locals.var_cox0__blk908), (locals.var_t1__blk898_dn11 * locals.var_cox0__blk908), (locals.var_t1__blk898_dn12 * locals.var_cox0__blk908), (locals.var_t1__blk898_dn17 * locals.var_cox0__blk908),)
    } else {
        (locals.var_ty__blk907, locals.var_ty__blk907_dn0, locals.var_ty__blk907_dn2, locals.var_ty__blk907_dn6, locals.var_ty__blk907_dn7, locals.var_ty__blk907_dn10, locals.var_ty__blk907_dn11, locals.var_ty__blk907_dn12, locals.var_ty__blk907_dn17,)
    }
};
        locals.var_ty__blk907 = assign28820_e40616;
        locals.var_ty__blk907_dn0 = assign28820_e40616_d_n0;
        locals.var_ty__blk907_dn2 = assign28820_e40616_d_n2;
        locals.var_ty__blk907_dn6 = assign28820_e40616_d_n6;
        locals.var_ty__blk907_dn7 = assign28820_e40616_d_n7;
        locals.var_ty__blk907_dn10 = assign28820_e40616_d_n10;
        locals.var_ty__blk907_dn11 = assign28820_e40616_d_n11;
        locals.var_ty__blk907_dn12 = assign28820_e40616_d_n12;
        locals.var_ty__blk907_dn17 = assign28820_e40616_d_n17;
        locals.var_ty__blk907_rv = 0.0;

        let (assign28830_e40633, assign28830_e40633_d_n0, assign28830_e40633_d_n2, assign28830_e40633_d_n6, assign28830_e40633_d_n7, assign28830_e40633_d_n10, assign28830_e40633_d_n11, assign28830_e40633_d_n12, assign28830_e40633_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28830_e40628: f64 = (3.0 * 1.414213562373095);
        let assign28830_e40630: f64 = (assign28830_e40628 * locals.var_ty__blk907);
        let assign28830_e40631: f64 = (2.0 + assign28830_e40630);
        (assign28830_e40631, (assign28830_e40628 * locals.var_ty__blk907_dn0), (assign28830_e40628 * locals.var_ty__blk907_dn2), (assign28830_e40628 * locals.var_ty__blk907_dn6), (assign28830_e40628 * locals.var_ty__blk907_dn7), (assign28830_e40628 * locals.var_ty__blk907_dn10), (assign28830_e40628 * locals.var_ty__blk907_dn11), (assign28830_e40628 * locals.var_ty__blk907_dn12), (assign28830_e40628 * locals.var_ty__blk907_dn17),)
    } else {
        (locals.var_ac41__blk936, locals.var_ac41__blk936_dn0, locals.var_ac41__blk936_dn2, locals.var_ac41__blk936_dn6, locals.var_ac41__blk936_dn7, locals.var_ac41__blk936_dn10, locals.var_ac41__blk936_dn11, locals.var_ac41__blk936_dn12, locals.var_ac41__blk936_dn17,)
    }
};
        locals.var_ac41__blk936 = assign28830_e40633;
        locals.var_ac41__blk936_dn0 = assign28830_e40633_d_n0;
        locals.var_ac41__blk936_dn2 = assign28830_e40633_d_n2;
        locals.var_ac41__blk936_dn6 = assign28830_e40633_d_n6;
        locals.var_ac41__blk936_dn7 = assign28830_e40633_d_n7;
        locals.var_ac41__blk936_dn10 = assign28830_e40633_d_n10;
        locals.var_ac41__blk936_dn11 = assign28830_e40633_d_n11;
        locals.var_ac41__blk936_dn12 = assign28830_e40633_d_n12;
        locals.var_ac41__blk936_dn17 = assign28830_e40633_d_n17;
        locals.var_ac41__blk936_rv = 0.0;

        let (assign28840_e40650, assign28840_e40650_d_n0, assign28840_e40650_d_n2, assign28840_e40650_d_n6, assign28840_e40650_d_n7, assign28840_e40650_d_n10, assign28840_e40650_d_n11, assign28840_e40650_d_n12, assign28840_e40650_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28840_e40644: f64 = (8.0 * locals.var_ac41__blk936);
        let assign28840_e40646: f64 = (assign28840_e40644 * locals.var_ac41__blk936);
        let assign28840_e40648: f64 = (assign28840_e40646 * locals.var_ac41__blk936);
        (assign28840_e40648, (((((8.0 * locals.var_ac41__blk936_dn0) * locals.var_ac41__blk936) + (assign28840_e40644 * locals.var_ac41__blk936_dn0)) * locals.var_ac41__blk936) + (assign28840_e40646 * locals.var_ac41__blk936_dn0)), (((((8.0 * locals.var_ac41__blk936_dn2) * locals.var_ac41__blk936) + (assign28840_e40644 * locals.var_ac41__blk936_dn2)) * locals.var_ac41__blk936) + (assign28840_e40646 * locals.var_ac41__blk936_dn2)), (((((8.0 * locals.var_ac41__blk936_dn6) * locals.var_ac41__blk936) + (assign28840_e40644 * locals.var_ac41__blk936_dn6)) * locals.var_ac41__blk936) + (assign28840_e40646 * locals.var_ac41__blk936_dn6)), (((((8.0 * locals.var_ac41__blk936_dn7) * locals.var_ac41__blk936) + (assign28840_e40644 * locals.var_ac41__blk936_dn7)) * locals.var_ac41__blk936) + (assign28840_e40646 * locals.var_ac41__blk936_dn7)), (((((8.0 * locals.var_ac41__blk936_dn10) * locals.var_ac41__blk936) + (assign28840_e40644 * locals.var_ac41__blk936_dn10)) * locals.var_ac41__blk936) + (assign28840_e40646 * locals.var_ac41__blk936_dn10)), (((((8.0 * locals.var_ac41__blk936_dn11) * locals.var_ac41__blk936) + (assign28840_e40644 * locals.var_ac41__blk936_dn11)) * locals.var_ac41__blk936) + (assign28840_e40646 * locals.var_ac41__blk936_dn11)), (((((8.0 * locals.var_ac41__blk936_dn12) * locals.var_ac41__blk936) + (assign28840_e40644 * locals.var_ac41__blk936_dn12)) * locals.var_ac41__blk936) + (assign28840_e40646 * locals.var_ac41__blk936_dn12)), (((((8.0 * locals.var_ac41__blk936_dn17) * locals.var_ac41__blk936) + (assign28840_e40644 * locals.var_ac41__blk936_dn17)) * locals.var_ac41__blk936) + (assign28840_e40646 * locals.var_ac41__blk936_dn17)),)
    } else {
        (locals.var_ac4__blk937, locals.var_ac4__blk937_dn0, locals.var_ac4__blk937_dn2, locals.var_ac4__blk937_dn6, locals.var_ac4__blk937_dn7, locals.var_ac4__blk937_dn10, locals.var_ac4__blk937_dn11, locals.var_ac4__blk937_dn12, locals.var_ac4__blk937_dn17,)
    }
};
        locals.var_ac4__blk937 = assign28840_e40650;
        locals.var_ac4__blk937_dn0 = assign28840_e40650_d_n0;
        locals.var_ac4__blk937_dn2 = assign28840_e40650_d_n2;
        locals.var_ac4__blk937_dn6 = assign28840_e40650_d_n6;
        locals.var_ac4__blk937_dn7 = assign28840_e40650_d_n7;
        locals.var_ac4__blk937_dn10 = assign28840_e40650_d_n10;
        locals.var_ac4__blk937_dn11 = assign28840_e40650_d_n11;
        locals.var_ac4__blk937_dn12 = assign28840_e40650_d_n12;
        locals.var_ac4__blk937_dn17 = assign28840_e40650_d_n17;
        locals.var_ac4__blk937_rv = 0.0;

        let (assign28850_e40663, assign28850_e40663_d_n0, assign28850_e40663_d_n2, assign28850_e40663_d_n6, assign28850_e40663_d_n7, assign28850_e40663_d_n10, assign28850_e40663_d_n11, assign28850_e40663_d_n12, assign28850_e40663_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28850_e40661: f64 = (locals.var_eg - locals.var_pb2over__blk934);
        (assign28850_e40661, (locals.var_eg_dn0 - locals.var_pb2over__blk934_dn0), (locals.var_eg_dn2 - locals.var_pb2over__blk934_dn2), (locals.var_eg_dn6 - locals.var_pb2over__blk934_dn6), (locals.var_eg_dn7 - locals.var_pb2over__blk934_dn7), (locals.var_eg_dn10 - locals.var_pb2over__blk934_dn10), (locals.var_eg_dn11 - locals.var_pb2over__blk934_dn11), (locals.var_eg_dn12 - locals.var_pb2over__blk934_dn12), (locals.var_eg_dn17 - locals.var_pb2over__blk934_dn17),)
    } else {
        (locals.var_ps0_min__blk938, locals.var_ps0_min__blk938_dn0, locals.var_ps0_min__blk938_dn2, locals.var_ps0_min__blk938_dn6, locals.var_ps0_min__blk938_dn7, locals.var_ps0_min__blk938_dn10, locals.var_ps0_min__blk938_dn11, locals.var_ps0_min__blk938_dn12, locals.var_ps0_min__blk938_dn17,)
    }
};
        locals.var_ps0_min__blk938 = assign28850_e40663;
        locals.var_ps0_min__blk938_dn0 = assign28850_e40663_d_n0;
        locals.var_ps0_min__blk938_dn2 = assign28850_e40663_d_n2;
        locals.var_ps0_min__blk938_dn6 = assign28850_e40663_d_n6;
        locals.var_ps0_min__blk938_dn7 = assign28850_e40663_d_n7;
        locals.var_ps0_min__blk938_dn10 = assign28850_e40663_d_n10;
        locals.var_ps0_min__blk938_dn11 = assign28850_e40663_d_n11;
        locals.var_ps0_min__blk938_dn12 = assign28850_e40663_d_n12;
        locals.var_ps0_min__blk938_dn17 = assign28850_e40663_d_n17;
        locals.var_ps0_min__blk938_rv = 0.0;

        let (assign28860_e40678, assign28860_e40678_d_n0, assign28860_e40678_d_n2, assign28860_e40678_d_n6, assign28860_e40678_d_n7, assign28860_e40678_d_n10, assign28860_e40678_d_n11, assign28860_e40678_d_n12, assign28860_e40678_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28860_e40675: f64 = (locals.var_vgpld__blk933 + locals.var_vxbgmtcl__blk923);
        let assign28860_e40676: f64 = (locals.var_beta * assign28860_e40675);
        (assign28860_e40676, (locals.var_beta * (locals.var_vgpld__blk933_dn0 + locals.var_vxbgmtcl__blk923_dn0)), (locals.var_beta * (locals.var_vgpld__blk933_dn2 + locals.var_vxbgmtcl__blk923_dn2)), (locals.var_beta * (locals.var_vgpld__blk933_dn6 + locals.var_vxbgmtcl__blk923_dn6)), (locals.var_beta * (locals.var_vgpld__blk933_dn7 + locals.var_vxbgmtcl__blk923_dn7)), ((locals.var_beta_dn10 * assign28860_e40675) + (locals.var_beta * (locals.var_vgpld__blk933_dn10 + locals.var_vxbgmtcl__blk923_dn10))), (locals.var_beta * (locals.var_vgpld__blk933_dn11 + locals.var_vxbgmtcl__blk923_dn11)), (locals.var_beta * (locals.var_vgpld__blk933_dn12 + locals.var_vxbgmtcl__blk923_dn12)), (locals.var_beta * (locals.var_vgpld__blk933_dn17 + locals.var_vxbgmtcl__blk923_dn17)),)
    } else {
        (locals.var_tx__blk906, locals.var_tx__blk906_dn0, locals.var_tx__blk906_dn2, locals.var_tx__blk906_dn6, locals.var_tx__blk906_dn7, locals.var_tx__blk906_dn10, locals.var_tx__blk906_dn11, locals.var_tx__blk906_dn12, locals.var_tx__blk906_dn17,)
    }
};
        locals.var_tx__blk906 = assign28860_e40678;
        locals.var_tx__blk906_dn0 = assign28860_e40678_d_n0;
        locals.var_tx__blk906_dn2 = assign28860_e40678_d_n2;
        locals.var_tx__blk906_dn6 = assign28860_e40678_d_n6;
        locals.var_tx__blk906_dn7 = assign28860_e40678_d_n7;
        locals.var_tx__blk906_dn10 = assign28860_e40678_d_n10;
        locals.var_tx__blk906_dn11 = assign28860_e40678_d_n11;
        locals.var_tx__blk906_dn12 = assign28860_e40678_d_n12;
        locals.var_tx__blk906_dn17 = assign28860_e40678_d_n17;
        locals.var_tx__blk906_rv = 0.0;

        let (assign28870_e40699, assign28870_e40699_d_n0, assign28870_e40699_d_n2, assign28870_e40699_d_n6, assign28870_e40699_d_n7, assign28870_e40699_d_n10, assign28870_e40699_d_n11, assign28870_e40699_d_n12, assign28870_e40699_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28870_e40689: f64 = (7.0 * 1.414213562373095);
        let assign28870_e40692: f64 = (9.0 * locals.var_ty__blk907);
        let assign28870_e40695: f64 = (locals.var_tx__blk906 - 2.0);
        let assign28870_e40696: f64 = (assign28870_e40692 * assign28870_e40695);
        let assign28870_e40697: f64 = (assign28870_e40689 - assign28870_e40696);
        (assign28870_e40697, (-(((9.0 * locals.var_ty__blk907_dn0) * assign28870_e40695) + (assign28870_e40692 * locals.var_tx__blk906_dn0))), (-(((9.0 * locals.var_ty__blk907_dn2) * assign28870_e40695) + (assign28870_e40692 * locals.var_tx__blk906_dn2))), (-(((9.0 * locals.var_ty__blk907_dn6) * assign28870_e40695) + (assign28870_e40692 * locals.var_tx__blk906_dn6))), (-(((9.0 * locals.var_ty__blk907_dn7) * assign28870_e40695) + (assign28870_e40692 * locals.var_tx__blk906_dn7))), (-(((9.0 * locals.var_ty__blk907_dn10) * assign28870_e40695) + (assign28870_e40692 * locals.var_tx__blk906_dn10))), (-(((9.0 * locals.var_ty__blk907_dn11) * assign28870_e40695) + (assign28870_e40692 * locals.var_tx__blk906_dn11))), (-(((9.0 * locals.var_ty__blk907_dn12) * assign28870_e40695) + (assign28870_e40692 * locals.var_tx__blk906_dn12))), (-(((9.0 * locals.var_ty__blk907_dn17) * assign28870_e40695) + (assign28870_e40692 * locals.var_tx__blk906_dn17))),)
    } else {
        (locals.var_ac31__blk939, locals.var_ac31__blk939_dn0, locals.var_ac31__blk939_dn2, locals.var_ac31__blk939_dn6, locals.var_ac31__blk939_dn7, locals.var_ac31__blk939_dn10, locals.var_ac31__blk939_dn11, locals.var_ac31__blk939_dn12, locals.var_ac31__blk939_dn17,)
    }
};
        locals.var_ac31__blk939 = assign28870_e40699;
        locals.var_ac31__blk939_dn0 = assign28870_e40699_d_n0;
        locals.var_ac31__blk939_dn2 = assign28870_e40699_d_n2;
        locals.var_ac31__blk939_dn6 = assign28870_e40699_d_n6;
        locals.var_ac31__blk939_dn7 = assign28870_e40699_d_n7;
        locals.var_ac31__blk939_dn10 = assign28870_e40699_d_n10;
        locals.var_ac31__blk939_dn11 = assign28870_e40699_d_n11;
        locals.var_ac31__blk939_dn12 = assign28870_e40699_d_n12;
        locals.var_ac31__blk939_dn17 = assign28870_e40699_d_n17;
        locals.var_ac31__blk939_rv = 0.0;

        let (assign28880_e40712, assign28880_e40712_d_n0, assign28880_e40712_d_n2, assign28880_e40712_d_n6, assign28880_e40712_d_n7, assign28880_e40712_d_n10, assign28880_e40712_d_n11, assign28880_e40712_d_n12, assign28880_e40712_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28880_e40710: f64 = (locals.var_ac31__blk939 * locals.var_ac31__blk939);
        (assign28880_e40710, ((locals.var_ac31__blk939_dn0 * locals.var_ac31__blk939) + (locals.var_ac31__blk939 * locals.var_ac31__blk939_dn0)), ((locals.var_ac31__blk939_dn2 * locals.var_ac31__blk939) + (locals.var_ac31__blk939 * locals.var_ac31__blk939_dn2)), ((locals.var_ac31__blk939_dn6 * locals.var_ac31__blk939) + (locals.var_ac31__blk939 * locals.var_ac31__blk939_dn6)), ((locals.var_ac31__blk939_dn7 * locals.var_ac31__blk939) + (locals.var_ac31__blk939 * locals.var_ac31__blk939_dn7)), ((locals.var_ac31__blk939_dn10 * locals.var_ac31__blk939) + (locals.var_ac31__blk939 * locals.var_ac31__blk939_dn10)), ((locals.var_ac31__blk939_dn11 * locals.var_ac31__blk939) + (locals.var_ac31__blk939 * locals.var_ac31__blk939_dn11)), ((locals.var_ac31__blk939_dn12 * locals.var_ac31__blk939) + (locals.var_ac31__blk939 * locals.var_ac31__blk939_dn12)), ((locals.var_ac31__blk939_dn17 * locals.var_ac31__blk939) + (locals.var_ac31__blk939 * locals.var_ac31__blk939_dn17)),)
    } else {
        (locals.var_ac3__blk940, locals.var_ac3__blk940_dn0, locals.var_ac3__blk940_dn2, locals.var_ac3__blk940_dn6, locals.var_ac3__blk940_dn7, locals.var_ac3__blk940_dn10, locals.var_ac3__blk940_dn11, locals.var_ac3__blk940_dn12, locals.var_ac3__blk940_dn17,)
    }
};
        locals.var_ac3__blk940 = assign28880_e40712;
        locals.var_ac3__blk940_dn0 = assign28880_e40712_d_n0;
        locals.var_ac3__blk940_dn2 = assign28880_e40712_d_n2;
        locals.var_ac3__blk940_dn6 = assign28880_e40712_d_n6;
        locals.var_ac3__blk940_dn7 = assign28880_e40712_d_n7;
        locals.var_ac3__blk940_dn10 = assign28880_e40712_d_n10;
        locals.var_ac3__blk940_dn11 = assign28880_e40712_d_n11;
        locals.var_ac3__blk940_dn12 = assign28880_e40712_d_n12;
        locals.var_ac3__blk940_dn17 = assign28880_e40712_d_n17;
        locals.var_ac3__blk940_rv = 0.0;

        let assign28890_e40716: f64 = (locals.var_ac3__blk940 * 1e-8);
        let assign28890_e40717: f64 = if locals.var_ac4__blk937 < assign28890_e40716 { 1.0 } else { 0.0 };
        locals.var_guard985 = assign28890_e40717;
        locals.var_guard985_rv = 0.0;

        let (assign28900_e40749, assign28900_e40749_d_n0, assign28900_e40749_d_n2, assign28900_e40749_d_n6, assign28900_e40749_d_n7, assign28900_e40749_d_n10, assign28900_e40749_d_n11, assign28900_e40749_d_n12, assign28900_e40749_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) && (locals.var_guard985 != 0.0)) {
        let assign28900_e40729: f64 = (-7.0);
        let assign28900_e40731: f64 = (assign28900_e40729 * 1.414213562373095);
        let assign28900_e40733: f64 = (assign28900_e40731 + locals.var_ac31__blk939);
        let assign28900_e40736: f64 = (0.5 * locals.var_ac4__blk937);
        let assign28900_e40738: f64 = (assign28900_e40736 / locals.var_ac31__blk939);
        let assign28900_e40739: f64 = (assign28900_e40733 + assign28900_e40738);
        let assign28900_e40742: f64 = (9.0 * locals.var_ty__blk907);
        let assign28900_e40745: f64 = (locals.var_tx__blk906 - 2.0);
        let assign28900_e40746: f64 = (assign28900_e40742 * assign28900_e40745);
        let assign28900_e40747: f64 = (assign28900_e40739 + assign28900_e40746);
        (assign28900_e40747, ((locals.var_ac31__blk939_dn0 + ((((0.5 * locals.var_ac4__blk937_dn0) * locals.var_ac31__blk939) - (assign28900_e40736 * locals.var_ac31__blk939_dn0)) / (locals.var_ac31__blk939 * locals.var_ac31__blk939))) + (((9.0 * locals.var_ty__blk907_dn0) * assign28900_e40745) + (assign28900_e40742 * locals.var_tx__blk906_dn0))), ((locals.var_ac31__blk939_dn2 + ((((0.5 * locals.var_ac4__blk937_dn2) * locals.var_ac31__blk939) - (assign28900_e40736 * locals.var_ac31__blk939_dn2)) / (locals.var_ac31__blk939 * locals.var_ac31__blk939))) + (((9.0 * locals.var_ty__blk907_dn2) * assign28900_e40745) + (assign28900_e40742 * locals.var_tx__blk906_dn2))), ((locals.var_ac31__blk939_dn6 + ((((0.5 * locals.var_ac4__blk937_dn6) * locals.var_ac31__blk939) - (assign28900_e40736 * locals.var_ac31__blk939_dn6)) / (locals.var_ac31__blk939 * locals.var_ac31__blk939))) + (((9.0 * locals.var_ty__blk907_dn6) * assign28900_e40745) + (assign28900_e40742 * locals.var_tx__blk906_dn6))), ((locals.var_ac31__blk939_dn7 + ((((0.5 * locals.var_ac4__blk937_dn7) * locals.var_ac31__blk939) - (assign28900_e40736 * locals.var_ac31__blk939_dn7)) / (locals.var_ac31__blk939 * locals.var_ac31__blk939))) + (((9.0 * locals.var_ty__blk907_dn7) * assign28900_e40745) + (assign28900_e40742 * locals.var_tx__blk906_dn7))), ((locals.var_ac31__blk939_dn10 + ((((0.5 * locals.var_ac4__blk937_dn10) * locals.var_ac31__blk939) - (assign28900_e40736 * locals.var_ac31__blk939_dn10)) / (locals.var_ac31__blk939 * locals.var_ac31__blk939))) + (((9.0 * locals.var_ty__blk907_dn10) * assign28900_e40745) + (assign28900_e40742 * locals.var_tx__blk906_dn10))), ((locals.var_ac31__blk939_dn11 + ((((0.5 * locals.var_ac4__blk937_dn11) * locals.var_ac31__blk939) - (assign28900_e40736 * locals.var_ac31__blk939_dn11)) / (locals.var_ac31__blk939 * locals.var_ac31__blk939))) + (((9.0 * locals.var_ty__blk907_dn11) * assign28900_e40745) + (assign28900_e40742 * locals.var_tx__blk906_dn11))), ((locals.var_ac31__blk939_dn12 + ((((0.5 * locals.var_ac4__blk937_dn12) * locals.var_ac31__blk939) - (assign28900_e40736 * locals.var_ac31__blk939_dn12)) / (locals.var_ac31__blk939 * locals.var_ac31__blk939))) + (((9.0 * locals.var_ty__blk907_dn12) * assign28900_e40745) + (assign28900_e40742 * locals.var_tx__blk906_dn12))), ((locals.var_ac31__blk939_dn17 + ((((0.5 * locals.var_ac4__blk937_dn17) * locals.var_ac31__blk939) - (assign28900_e40736 * locals.var_ac31__blk939_dn17)) / (locals.var_ac31__blk939 * locals.var_ac31__blk939))) + (((9.0 * locals.var_ty__blk907_dn17) * assign28900_e40745) + (assign28900_e40742 * locals.var_tx__blk906_dn17))),)
    } else {
        (locals.var_ac1__blk942, locals.var_ac1__blk942_dn0, locals.var_ac1__blk942_dn2, locals.var_ac1__blk942_dn6, locals.var_ac1__blk942_dn7, locals.var_ac1__blk942_dn10, locals.var_ac1__blk942_dn11, locals.var_ac1__blk942_dn12, locals.var_ac1__blk942_dn17,)
    }
};
        locals.var_ac1__blk942 = assign28900_e40749;
        locals.var_ac1__blk942_dn0 = assign28900_e40749_d_n0;
        locals.var_ac1__blk942_dn2 = assign28900_e40749_d_n2;
        locals.var_ac1__blk942_dn6 = assign28900_e40749_d_n6;
        locals.var_ac1__blk942_dn7 = assign28900_e40749_d_n7;
        locals.var_ac1__blk942_dn10 = assign28900_e40749_d_n10;
        locals.var_ac1__blk942_dn11 = assign28900_e40749_d_n11;
        locals.var_ac1__blk942_dn12 = assign28900_e40749_d_n12;
        locals.var_ac1__blk942_dn17 = assign28900_e40749_d_n17;
        locals.var_ac1__blk942_rv = 0.0;

        let (assign28910_e40766, assign28910_e40766_d_n0, assign28910_e40766_d_n2, assign28910_e40766_d_n6, assign28910_e40766_d_n7, assign28910_e40766_d_n10, assign28910_e40766_d_n11, assign28910_e40766_d_n12, assign28910_e40766_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) && (locals.var_guard985 == 0.0)) {
        let assign28910_e40763: f64 = (locals.var_ac4__blk937 + locals.var_ac3__blk940);
        let assign28910_e40764: f64 = (assign28910_e40763).sqrt();
        (assign28910_e40764, ((locals.var_ac4__blk937_dn0 + locals.var_ac3__blk940_dn0) / (2.0 * assign28910_e40764)), ((locals.var_ac4__blk937_dn2 + locals.var_ac3__blk940_dn2) / (2.0 * assign28910_e40764)), ((locals.var_ac4__blk937_dn6 + locals.var_ac3__blk940_dn6) / (2.0 * assign28910_e40764)), ((locals.var_ac4__blk937_dn7 + locals.var_ac3__blk940_dn7) / (2.0 * assign28910_e40764)), ((locals.var_ac4__blk937_dn10 + locals.var_ac3__blk940_dn10) / (2.0 * assign28910_e40764)), ((locals.var_ac4__blk937_dn11 + locals.var_ac3__blk940_dn11) / (2.0 * assign28910_e40764)), ((locals.var_ac4__blk937_dn12 + locals.var_ac3__blk940_dn12) / (2.0 * assign28910_e40764)), ((locals.var_ac4__blk937_dn17 + locals.var_ac3__blk940_dn17) / (2.0 * assign28910_e40764)),)
    } else {
        (locals.var_ac2__blk941, locals.var_ac2__blk941_dn0, locals.var_ac2__blk941_dn2, locals.var_ac2__blk941_dn6, locals.var_ac2__blk941_dn7, locals.var_ac2__blk941_dn10, locals.var_ac2__blk941_dn11, locals.var_ac2__blk941_dn12, locals.var_ac2__blk941_dn17,)
    }
};
        locals.var_ac2__blk941 = assign28910_e40766;
        locals.var_ac2__blk941_dn0 = assign28910_e40766_d_n0;
        locals.var_ac2__blk941_dn2 = assign28910_e40766_d_n2;
        locals.var_ac2__blk941_dn6 = assign28910_e40766_d_n6;
        locals.var_ac2__blk941_dn7 = assign28910_e40766_d_n7;
        locals.var_ac2__blk941_dn10 = assign28910_e40766_d_n10;
        locals.var_ac2__blk941_dn11 = assign28910_e40766_d_n11;
        locals.var_ac2__blk941_dn12 = assign28910_e40766_d_n12;
        locals.var_ac2__blk941_dn17 = assign28910_e40766_d_n17;
        locals.var_ac2__blk941_rv = 0.0;

        let (assign28920_e40793, assign28920_e40793_d_n0, assign28920_e40793_d_n2, assign28920_e40793_d_n6, assign28920_e40793_d_n7, assign28920_e40793_d_n10, assign28920_e40793_d_n11, assign28920_e40793_d_n12, assign28920_e40793_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) && (locals.var_guard985 == 0.0)) {
        let assign28920_e40779: f64 = (-7.0);
        let assign28920_e40781: f64 = (assign28920_e40779 * 1.414213562373095);
        let assign28920_e40783: f64 = (assign28920_e40781 + locals.var_ac2__blk941);
        let assign28920_e40786: f64 = (9.0 * locals.var_ty__blk907);
        let assign28920_e40789: f64 = (locals.var_tx__blk906 - 2.0);
        let assign28920_e40790: f64 = (assign28920_e40786 * assign28920_e40789);
        let assign28920_e40791: f64 = (assign28920_e40783 + assign28920_e40790);
        (assign28920_e40791, (locals.var_ac2__blk941_dn0 + (((9.0 * locals.var_ty__blk907_dn0) * assign28920_e40789) + (assign28920_e40786 * locals.var_tx__blk906_dn0))), (locals.var_ac2__blk941_dn2 + (((9.0 * locals.var_ty__blk907_dn2) * assign28920_e40789) + (assign28920_e40786 * locals.var_tx__blk906_dn2))), (locals.var_ac2__blk941_dn6 + (((9.0 * locals.var_ty__blk907_dn6) * assign28920_e40789) + (assign28920_e40786 * locals.var_tx__blk906_dn6))), (locals.var_ac2__blk941_dn7 + (((9.0 * locals.var_ty__blk907_dn7) * assign28920_e40789) + (assign28920_e40786 * locals.var_tx__blk906_dn7))), (locals.var_ac2__blk941_dn10 + (((9.0 * locals.var_ty__blk907_dn10) * assign28920_e40789) + (assign28920_e40786 * locals.var_tx__blk906_dn10))), (locals.var_ac2__blk941_dn11 + (((9.0 * locals.var_ty__blk907_dn11) * assign28920_e40789) + (assign28920_e40786 * locals.var_tx__blk906_dn11))), (locals.var_ac2__blk941_dn12 + (((9.0 * locals.var_ty__blk907_dn12) * assign28920_e40789) + (assign28920_e40786 * locals.var_tx__blk906_dn12))), (locals.var_ac2__blk941_dn17 + (((9.0 * locals.var_ty__blk907_dn17) * assign28920_e40789) + (assign28920_e40786 * locals.var_tx__blk906_dn17))),)
    } else {
        (locals.var_ac1__blk942, locals.var_ac1__blk942_dn0, locals.var_ac1__blk942_dn2, locals.var_ac1__blk942_dn6, locals.var_ac1__blk942_dn7, locals.var_ac1__blk942_dn10, locals.var_ac1__blk942_dn11, locals.var_ac1__blk942_dn12, locals.var_ac1__blk942_dn17,)
    }
};
        locals.var_ac1__blk942 = assign28920_e40793;
        locals.var_ac1__blk942_dn0 = assign28920_e40793_d_n0;
        locals.var_ac1__blk942_dn2 = assign28920_e40793_d_n2;
        locals.var_ac1__blk942_dn6 = assign28920_e40793_d_n6;
        locals.var_ac1__blk942_dn7 = assign28920_e40793_d_n7;
        locals.var_ac1__blk942_dn10 = assign28920_e40793_d_n10;
        locals.var_ac1__blk942_dn11 = assign28920_e40793_d_n11;
        locals.var_ac1__blk942_dn12 = assign28920_e40793_d_n12;
        locals.var_ac1__blk942_dn17 = assign28920_e40793_d_n17;
        locals.var_ac1__blk942_rv = 0.0;

        let (assign28930_e40806, assign28930_e40806_d_n0, assign28930_e40806_d_n2, assign28930_e40806_d_n6, assign28930_e40806_d_n7, assign28930_e40806_d_n10, assign28930_e40806_d_n11, assign28930_e40806_d_n12, assign28930_e40806_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28930_e40804: f64 = (locals.var_ac1__blk942).powf(0.3333333333333333);
        (assign28930_e40804, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk942).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk942_dn0)) } } else { (assign28930_e40804 * (0.3333333333333333 * (locals.var_ac1__blk942_dn0 / locals.var_ac1__blk942))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk942).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk942_dn2)) } } else { (assign28930_e40804 * (0.3333333333333333 * (locals.var_ac1__blk942_dn2 / locals.var_ac1__blk942))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk942).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk942_dn6)) } } else { (assign28930_e40804 * (0.3333333333333333 * (locals.var_ac1__blk942_dn6 / locals.var_ac1__blk942))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk942).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk942_dn7)) } } else { (assign28930_e40804 * (0.3333333333333333 * (locals.var_ac1__blk942_dn7 / locals.var_ac1__blk942))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk942).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk942_dn10)) } } else { (assign28930_e40804 * (0.3333333333333333 * (locals.var_ac1__blk942_dn10 / locals.var_ac1__blk942))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk942).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk942_dn11)) } } else { (assign28930_e40804 * (0.3333333333333333 * (locals.var_ac1__blk942_dn11 / locals.var_ac1__blk942))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk942).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk942_dn12)) } } else { (assign28930_e40804 * (0.3333333333333333 * (locals.var_ac1__blk942_dn12 / locals.var_ac1__blk942))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk942).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk942_dn17)) } } else { (assign28930_e40804 * (0.3333333333333333 * (locals.var_ac1__blk942_dn17 / locals.var_ac1__blk942))) },)
    } else {
        (locals.var_acd__blk943, locals.var_acd__blk943_dn0, locals.var_acd__blk943_dn2, locals.var_acd__blk943_dn6, locals.var_acd__blk943_dn7, locals.var_acd__blk943_dn10, locals.var_acd__blk943_dn11, locals.var_acd__blk943_dn12, locals.var_acd__blk943_dn17,)
    }
};
        locals.var_acd__blk943 = assign28930_e40806;
        locals.var_acd__blk943_dn0 = assign28930_e40806_d_n0;
        locals.var_acd__blk943_dn2 = assign28930_e40806_d_n2;
        locals.var_acd__blk943_dn6 = assign28930_e40806_d_n6;
        locals.var_acd__blk943_dn7 = assign28930_e40806_d_n7;
        locals.var_acd__blk943_dn10 = assign28930_e40806_d_n10;
        locals.var_acd__blk943_dn11 = assign28930_e40806_d_n11;
        locals.var_acd__blk943_dn12 = assign28930_e40806_d_n12;
        locals.var_acd__blk943_dn17 = assign28930_e40806_d_n17;
        locals.var_acd__blk943_rv = 0.0;

        let (assign28940_e40834, assign28940_e40834_d_n0, assign28940_e40834_d_n2, assign28940_e40834_d_n6, assign28940_e40834_d_n7, assign28940_e40834_d_n10, assign28940_e40834_d_n11, assign28940_e40834_d_n12, assign28940_e40834_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28940_e40816: f64 = (-4.0);
        let assign28940_e40818: f64 = (assign28940_e40816 * 1.414213562373095);
        let assign28940_e40821: f64 = (12.0 * locals.var_ty__blk907);
        let assign28940_e40822: f64 = (assign28940_e40818 - assign28940_e40821);
        let assign28940_e40825: f64 = (2.0 * locals.var_acd__blk943);
        let assign28940_e40826: f64 = (assign28940_e40822 + assign28940_e40825);
        let assign28940_e40829: f64 = (1.414213562373095 * locals.var_acd__blk943);
        let assign28940_e40831: f64 = (assign28940_e40829 * locals.var_acd__blk943);
        let assign28940_e40832: f64 = (assign28940_e40826 + assign28940_e40831);
        (assign28940_e40832, (((-(12.0 * locals.var_ty__blk907_dn0)) + (2.0 * locals.var_acd__blk943_dn0)) + (((1.414213562373095 * locals.var_acd__blk943_dn0) * locals.var_acd__blk943) + (assign28940_e40829 * locals.var_acd__blk943_dn0))), (((-(12.0 * locals.var_ty__blk907_dn2)) + (2.0 * locals.var_acd__blk943_dn2)) + (((1.414213562373095 * locals.var_acd__blk943_dn2) * locals.var_acd__blk943) + (assign28940_e40829 * locals.var_acd__blk943_dn2))), (((-(12.0 * locals.var_ty__blk907_dn6)) + (2.0 * locals.var_acd__blk943_dn6)) + (((1.414213562373095 * locals.var_acd__blk943_dn6) * locals.var_acd__blk943) + (assign28940_e40829 * locals.var_acd__blk943_dn6))), (((-(12.0 * locals.var_ty__blk907_dn7)) + (2.0 * locals.var_acd__blk943_dn7)) + (((1.414213562373095 * locals.var_acd__blk943_dn7) * locals.var_acd__blk943) + (assign28940_e40829 * locals.var_acd__blk943_dn7))), (((-(12.0 * locals.var_ty__blk907_dn10)) + (2.0 * locals.var_acd__blk943_dn10)) + (((1.414213562373095 * locals.var_acd__blk943_dn10) * locals.var_acd__blk943) + (assign28940_e40829 * locals.var_acd__blk943_dn10))), (((-(12.0 * locals.var_ty__blk907_dn11)) + (2.0 * locals.var_acd__blk943_dn11)) + (((1.414213562373095 * locals.var_acd__blk943_dn11) * locals.var_acd__blk943) + (assign28940_e40829 * locals.var_acd__blk943_dn11))), (((-(12.0 * locals.var_ty__blk907_dn12)) + (2.0 * locals.var_acd__blk943_dn12)) + (((1.414213562373095 * locals.var_acd__blk943_dn12) * locals.var_acd__blk943) + (assign28940_e40829 * locals.var_acd__blk943_dn12))), (((-(12.0 * locals.var_ty__blk907_dn17)) + (2.0 * locals.var_acd__blk943_dn17)) + (((1.414213562373095 * locals.var_acd__blk943_dn17) * locals.var_acd__blk943) + (assign28940_e40829 * locals.var_acd__blk943_dn17))),)
    } else {
        (locals.var_acn__blk944, locals.var_acn__blk944_dn0, locals.var_acn__blk944_dn2, locals.var_acn__blk944_dn6, locals.var_acn__blk944_dn7, locals.var_acn__blk944_dn10, locals.var_acn__blk944_dn11, locals.var_acn__blk944_dn12, locals.var_acn__blk944_dn17,)
    }
};
        locals.var_acn__blk944 = assign28940_e40834;
        locals.var_acn__blk944_dn0 = assign28940_e40834_d_n0;
        locals.var_acn__blk944_dn2 = assign28940_e40834_d_n2;
        locals.var_acn__blk944_dn6 = assign28940_e40834_d_n6;
        locals.var_acn__blk944_dn7 = assign28940_e40834_d_n7;
        locals.var_acn__blk944_dn10 = assign28940_e40834_d_n10;
        locals.var_acn__blk944_dn11 = assign28940_e40834_d_n11;
        locals.var_acn__blk944_dn12 = assign28940_e40834_d_n12;
        locals.var_acn__blk944_dn17 = assign28940_e40834_d_n17;
        locals.var_acn__blk944_rv = 0.0;

        let (assign28950_e40847, assign28950_e40847_d_n0, assign28950_e40847_d_n2, assign28950_e40847_d_n6, assign28950_e40847_d_n7, assign28950_e40847_d_n10, assign28950_e40847_d_n11, assign28950_e40847_d_n12, assign28950_e40847_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28950_e40845: f64 = (locals.var_acn__blk944 / locals.var_acd__blk943);
        (assign28950_e40845, (((locals.var_acn__blk944_dn0 * locals.var_acd__blk943) - (locals.var_acn__blk944 * locals.var_acd__blk943_dn0)) / (locals.var_acd__blk943 * locals.var_acd__blk943)), (((locals.var_acn__blk944_dn2 * locals.var_acd__blk943) - (locals.var_acn__blk944 * locals.var_acd__blk943_dn2)) / (locals.var_acd__blk943 * locals.var_acd__blk943)), (((locals.var_acn__blk944_dn6 * locals.var_acd__blk943) - (locals.var_acn__blk944 * locals.var_acd__blk943_dn6)) / (locals.var_acd__blk943 * locals.var_acd__blk943)), (((locals.var_acn__blk944_dn7 * locals.var_acd__blk943) - (locals.var_acn__blk944 * locals.var_acd__blk943_dn7)) / (locals.var_acd__blk943 * locals.var_acd__blk943)), (((locals.var_acn__blk944_dn10 * locals.var_acd__blk943) - (locals.var_acn__blk944 * locals.var_acd__blk943_dn10)) / (locals.var_acd__blk943 * locals.var_acd__blk943)), (((locals.var_acn__blk944_dn11 * locals.var_acd__blk943) - (locals.var_acn__blk944 * locals.var_acd__blk943_dn11)) / (locals.var_acd__blk943 * locals.var_acd__blk943)), (((locals.var_acn__blk944_dn12 * locals.var_acd__blk943) - (locals.var_acn__blk944 * locals.var_acd__blk943_dn12)) / (locals.var_acd__blk943 * locals.var_acd__blk943)), (((locals.var_acn__blk944_dn17 * locals.var_acd__blk943) - (locals.var_acn__blk944 * locals.var_acd__blk943_dn17)) / (locals.var_acd__blk943 * locals.var_acd__blk943)),)
    } else {
        (locals.var_chi__blk945, locals.var_chi__blk945_dn0, locals.var_chi__blk945_dn2, locals.var_chi__blk945_dn6, locals.var_chi__blk945_dn7, locals.var_chi__blk945_dn10, locals.var_chi__blk945_dn11, locals.var_chi__blk945_dn12, locals.var_chi__blk945_dn17,)
    }
};
        locals.var_chi__blk945 = assign28950_e40847;
        locals.var_chi__blk945_dn0 = assign28950_e40847_d_n0;
        locals.var_chi__blk945_dn2 = assign28950_e40847_d_n2;
        locals.var_chi__blk945_dn6 = assign28950_e40847_d_n6;
        locals.var_chi__blk945_dn7 = assign28950_e40847_d_n7;
        locals.var_chi__blk945_dn10 = assign28950_e40847_d_n10;
        locals.var_chi__blk945_dn11 = assign28950_e40847_d_n11;
        locals.var_chi__blk945_dn12 = assign28950_e40847_d_n12;
        locals.var_chi__blk945_dn17 = assign28950_e40847_d_n17;
        locals.var_chi__blk945_rv = 0.0;

        let (assign28960_e40862, assign28960_e40862_d_n0, assign28960_e40862_d_n2, assign28960_e40862_d_n6, assign28960_e40862_d_n7, assign28960_e40862_d_n10, assign28960_e40862_d_n11, assign28960_e40862_d_n12, assign28960_e40862_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28960_e40858: f64 = (locals.var_chi__blk945 * locals.var_beta_inv);
        let assign28960_e40860: f64 = (assign28960_e40858 - locals.var_vxbgmtcl__blk923);
        (assign28960_e40860, ((locals.var_chi__blk945_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn0), ((locals.var_chi__blk945_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn2), ((locals.var_chi__blk945_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn6), ((locals.var_chi__blk945_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn7), (((locals.var_chi__blk945_dn10 * locals.var_beta_inv) + (locals.var_chi__blk945 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl__blk923_dn10), ((locals.var_chi__blk945_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn11), ((locals.var_chi__blk945_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn12), ((locals.var_chi__blk945_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn17),)
    } else {
        (locals.var_psa__blk946, locals.var_psa__blk946_dn0, locals.var_psa__blk946_dn2, locals.var_psa__blk946_dn6, locals.var_psa__blk946_dn7, locals.var_psa__blk946_dn10, locals.var_psa__blk946_dn11, locals.var_psa__blk946_dn12, locals.var_psa__blk946_dn17,)
    }
};
        locals.var_psa__blk946 = assign28960_e40862;
        locals.var_psa__blk946_dn0 = assign28960_e40862_d_n0;
        locals.var_psa__blk946_dn2 = assign28960_e40862_d_n2;
        locals.var_psa__blk946_dn6 = assign28960_e40862_d_n6;
        locals.var_psa__blk946_dn7 = assign28960_e40862_d_n7;
        locals.var_psa__blk946_dn10 = assign28960_e40862_d_n10;
        locals.var_psa__blk946_dn11 = assign28960_e40862_d_n11;
        locals.var_psa__blk946_dn12 = assign28960_e40862_d_n12;
        locals.var_psa__blk946_dn17 = assign28960_e40862_d_n17;
        locals.var_psa__blk946_rv = 0.0;

        let (assign28970_e40875, assign28970_e40875_d_n0, assign28970_e40875_d_n2, assign28970_e40875_d_n6, assign28970_e40875_d_n7, assign28970_e40875_d_n10, assign28970_e40875_d_n11, assign28970_e40875_d_n12, assign28970_e40875_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28970_e40873: f64 = (locals.var_psa__blk946 + locals.var_vxbgmtcl__blk923);
        (assign28970_e40873, (locals.var_psa__blk946_dn0 + locals.var_vxbgmtcl__blk923_dn0), (locals.var_psa__blk946_dn2 + locals.var_vxbgmtcl__blk923_dn2), (locals.var_psa__blk946_dn6 + locals.var_vxbgmtcl__blk923_dn6), (locals.var_psa__blk946_dn7 + locals.var_vxbgmtcl__blk923_dn7), (locals.var_psa__blk946_dn10 + locals.var_vxbgmtcl__blk923_dn10), (locals.var_psa__blk946_dn11 + locals.var_vxbgmtcl__blk923_dn11), (locals.var_psa__blk946_dn12 + locals.var_vxbgmtcl__blk923_dn12), (locals.var_psa__blk946_dn17 + locals.var_vxbgmtcl__blk923_dn17),)
    } else {
        (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17,)
    }
};
        locals.var_t1__blk898 = assign28970_e40875;
        locals.var_t1__blk898_dn0 = assign28970_e40875_d_n0;
        locals.var_t1__blk898_dn2 = assign28970_e40875_d_n2;
        locals.var_t1__blk898_dn6 = assign28970_e40875_d_n6;
        locals.var_t1__blk898_dn7 = assign28970_e40875_d_n7;
        locals.var_t1__blk898_dn10 = assign28970_e40875_d_n10;
        locals.var_t1__blk898_dn11 = assign28970_e40875_d_n11;
        locals.var_t1__blk898_dn12 = assign28970_e40875_d_n12;
        locals.var_t1__blk898_dn17 = assign28970_e40875_d_n17;
        locals.var_t1__blk898_rv = 0.0;

        let (assign28980_e40888, assign28980_e40888_d_n0, assign28980_e40888_d_n2, assign28980_e40888_d_n6, assign28980_e40888_d_n7, assign28980_e40888_d_n10, assign28980_e40888_d_n11, assign28980_e40888_d_n12, assign28980_e40888_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28980_e40886: f64 = (locals.var_t1__blk898 / locals.var_ps0_min__blk938);
        (assign28980_e40886, (((locals.var_t1__blk898_dn0 * locals.var_ps0_min__blk938) - (locals.var_t1__blk898 * locals.var_ps0_min__blk938_dn0)) / (locals.var_ps0_min__blk938 * locals.var_ps0_min__blk938)), (((locals.var_t1__blk898_dn2 * locals.var_ps0_min__blk938) - (locals.var_t1__blk898 * locals.var_ps0_min__blk938_dn2)) / (locals.var_ps0_min__blk938 * locals.var_ps0_min__blk938)), (((locals.var_t1__blk898_dn6 * locals.var_ps0_min__blk938) - (locals.var_t1__blk898 * locals.var_ps0_min__blk938_dn6)) / (locals.var_ps0_min__blk938 * locals.var_ps0_min__blk938)), (((locals.var_t1__blk898_dn7 * locals.var_ps0_min__blk938) - (locals.var_t1__blk898 * locals.var_ps0_min__blk938_dn7)) / (locals.var_ps0_min__blk938 * locals.var_ps0_min__blk938)), (((locals.var_t1__blk898_dn10 * locals.var_ps0_min__blk938) - (locals.var_t1__blk898 * locals.var_ps0_min__blk938_dn10)) / (locals.var_ps0_min__blk938 * locals.var_ps0_min__blk938)), (((locals.var_t1__blk898_dn11 * locals.var_ps0_min__blk938) - (locals.var_t1__blk898 * locals.var_ps0_min__blk938_dn11)) / (locals.var_ps0_min__blk938 * locals.var_ps0_min__blk938)), (((locals.var_t1__blk898_dn12 * locals.var_ps0_min__blk938) - (locals.var_t1__blk898 * locals.var_ps0_min__blk938_dn12)) / (locals.var_ps0_min__blk938 * locals.var_ps0_min__blk938)), (((locals.var_t1__blk898_dn17 * locals.var_ps0_min__blk938) - (locals.var_t1__blk898 * locals.var_ps0_min__blk938_dn17)) / (locals.var_ps0_min__blk938 * locals.var_ps0_min__blk938)),)
    } else {
        (locals.var_t2__blk899, locals.var_t2__blk899_dn0, locals.var_t2__blk899_dn2, locals.var_t2__blk899_dn6, locals.var_t2__blk899_dn7, locals.var_t2__blk899_dn10, locals.var_t2__blk899_dn11, locals.var_t2__blk899_dn12, locals.var_t2__blk899_dn17,)
    }
};
        locals.var_t2__blk899 = assign28980_e40888;
        locals.var_t2__blk899_dn0 = assign28980_e40888_d_n0;
        locals.var_t2__blk899_dn2 = assign28980_e40888_d_n2;
        locals.var_t2__blk899_dn6 = assign28980_e40888_d_n6;
        locals.var_t2__blk899_dn7 = assign28980_e40888_d_n7;
        locals.var_t2__blk899_dn10 = assign28980_e40888_d_n10;
        locals.var_t2__blk899_dn11 = assign28980_e40888_d_n11;
        locals.var_t2__blk899_dn12 = assign28980_e40888_d_n12;
        locals.var_t2__blk899_dn17 = assign28980_e40888_d_n17;
        locals.var_t2__blk899_rv = 0.0;

        let (assign28990_e40904, assign28990_e40904_d_n0, assign28990_e40904_d_n2, assign28990_e40904_d_n6, assign28990_e40904_d_n7, assign28990_e40904_d_n10, assign28990_e40904_d_n11, assign28990_e40904_d_n12, assign28990_e40904_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign28990_e40900: f64 = (locals.var_t2__blk899 * locals.var_t2__blk899);
        let assign28990_e40901: f64 = (1.0 + assign28990_e40900);
        let assign28990_e40902: f64 = (assign28990_e40901).sqrt();
        (assign28990_e40902, (((locals.var_t2__blk899_dn0 * locals.var_t2__blk899) + (locals.var_t2__blk899 * locals.var_t2__blk899_dn0)) / (2.0 * assign28990_e40902)), (((locals.var_t2__blk899_dn2 * locals.var_t2__blk899) + (locals.var_t2__blk899 * locals.var_t2__blk899_dn2)) / (2.0 * assign28990_e40902)), (((locals.var_t2__blk899_dn6 * locals.var_t2__blk899) + (locals.var_t2__blk899 * locals.var_t2__blk899_dn6)) / (2.0 * assign28990_e40902)), (((locals.var_t2__blk899_dn7 * locals.var_t2__blk899) + (locals.var_t2__blk899 * locals.var_t2__blk899_dn7)) / (2.0 * assign28990_e40902)), (((locals.var_t2__blk899_dn10 * locals.var_t2__blk899) + (locals.var_t2__blk899 * locals.var_t2__blk899_dn10)) / (2.0 * assign28990_e40902)), (((locals.var_t2__blk899_dn11 * locals.var_t2__blk899) + (locals.var_t2__blk899 * locals.var_t2__blk899_dn11)) / (2.0 * assign28990_e40902)), (((locals.var_t2__blk899_dn12 * locals.var_t2__blk899) + (locals.var_t2__blk899 * locals.var_t2__blk899_dn12)) / (2.0 * assign28990_e40902)), (((locals.var_t2__blk899_dn17 * locals.var_t2__blk899) + (locals.var_t2__blk899 * locals.var_t2__blk899_dn17)) / (2.0 * assign28990_e40902)),)
    } else {
        (locals.var_t3__blk900, locals.var_t3__blk900_dn0, locals.var_t3__blk900_dn2, locals.var_t3__blk900_dn6, locals.var_t3__blk900_dn7, locals.var_t3__blk900_dn10, locals.var_t3__blk900_dn11, locals.var_t3__blk900_dn12, locals.var_t3__blk900_dn17,)
    }
};
        locals.var_t3__blk900 = assign28990_e40904;
        locals.var_t3__blk900_dn0 = assign28990_e40904_d_n0;
        locals.var_t3__blk900_dn2 = assign28990_e40904_d_n2;
        locals.var_t3__blk900_dn6 = assign28990_e40904_d_n6;
        locals.var_t3__blk900_dn7 = assign28990_e40904_d_n7;
        locals.var_t3__blk900_dn10 = assign28990_e40904_d_n10;
        locals.var_t3__blk900_dn11 = assign28990_e40904_d_n11;
        locals.var_t3__blk900_dn12 = assign28990_e40904_d_n12;
        locals.var_t3__blk900_dn17 = assign28990_e40904_d_n17;
        locals.var_t3__blk900_rv = 0.0;

        let (assign29000_e40919, assign29000_e40919_d_n0, assign29000_e40919_d_n2, assign29000_e40919_d_n6, assign29000_e40919_d_n7, assign29000_e40919_d_n10, assign29000_e40919_d_n11, assign29000_e40919_d_n12, assign29000_e40919_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign29000_e40915: f64 = (locals.var_t1__blk898 / locals.var_t3__blk900);
        let assign29000_e40917: f64 = (assign29000_e40915 - locals.var_vxbgmtcl__blk923);
        (assign29000_e40917, ((((locals.var_t1__blk898_dn0 * locals.var_t3__blk900) - (locals.var_t1__blk898 * locals.var_t3__blk900_dn0)) / (locals.var_t3__blk900 * locals.var_t3__blk900)) - locals.var_vxbgmtcl__blk923_dn0), ((((locals.var_t1__blk898_dn2 * locals.var_t3__blk900) - (locals.var_t1__blk898 * locals.var_t3__blk900_dn2)) / (locals.var_t3__blk900 * locals.var_t3__blk900)) - locals.var_vxbgmtcl__blk923_dn2), ((((locals.var_t1__blk898_dn6 * locals.var_t3__blk900) - (locals.var_t1__blk898 * locals.var_t3__blk900_dn6)) / (locals.var_t3__blk900 * locals.var_t3__blk900)) - locals.var_vxbgmtcl__blk923_dn6), ((((locals.var_t1__blk898_dn7 * locals.var_t3__blk900) - (locals.var_t1__blk898 * locals.var_t3__blk900_dn7)) / (locals.var_t3__blk900 * locals.var_t3__blk900)) - locals.var_vxbgmtcl__blk923_dn7), ((((locals.var_t1__blk898_dn10 * locals.var_t3__blk900) - (locals.var_t1__blk898 * locals.var_t3__blk900_dn10)) / (locals.var_t3__blk900 * locals.var_t3__blk900)) - locals.var_vxbgmtcl__blk923_dn10), ((((locals.var_t1__blk898_dn11 * locals.var_t3__blk900) - (locals.var_t1__blk898 * locals.var_t3__blk900_dn11)) / (locals.var_t3__blk900 * locals.var_t3__blk900)) - locals.var_vxbgmtcl__blk923_dn11), ((((locals.var_t1__blk898_dn12 * locals.var_t3__blk900) - (locals.var_t1__blk898 * locals.var_t3__blk900_dn12)) / (locals.var_t3__blk900 * locals.var_t3__blk900)) - locals.var_vxbgmtcl__blk923_dn12), ((((locals.var_t1__blk898_dn17 * locals.var_t3__blk900) - (locals.var_t1__blk898 * locals.var_t3__blk900_dn17)) / (locals.var_t3__blk900 * locals.var_t3__blk900)) - locals.var_vxbgmtcl__blk923_dn17),)
    } else {
        (locals.var_ps0ld__blk947, locals.var_ps0ld__blk947_dn0, locals.var_ps0ld__blk947_dn2, locals.var_ps0ld__blk947_dn6, locals.var_ps0ld__blk947_dn7, locals.var_ps0ld__blk947_dn10, locals.var_ps0ld__blk947_dn11, locals.var_ps0ld__blk947_dn12, locals.var_ps0ld__blk947_dn17,)
    }
};
        locals.var_ps0ld__blk947 = assign29000_e40919;
        locals.var_ps0ld__blk947_dn0 = assign29000_e40919_d_n0;
        locals.var_ps0ld__blk947_dn2 = assign29000_e40919_d_n2;
        locals.var_ps0ld__blk947_dn6 = assign29000_e40919_d_n6;
        locals.var_ps0ld__blk947_dn7 = assign29000_e40919_d_n7;
        locals.var_ps0ld__blk947_dn10 = assign29000_e40919_d_n10;
        locals.var_ps0ld__blk947_dn11 = assign29000_e40919_d_n11;
        locals.var_ps0ld__blk947_dn12 = assign29000_e40919_d_n12;
        locals.var_ps0ld__blk947_dn17 = assign29000_e40919_d_n17;
        locals.var_ps0ld__blk947_rv = 0.0;

        let (assign29010_e40932, assign29010_e40932_d_n0, assign29010_e40932_d_n2, assign29010_e40932_d_n6, assign29010_e40932_d_n7, assign29010_e40932_d_n10, assign29010_e40932_d_n11, assign29010_e40932_d_n12, assign29010_e40932_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign29010_e40930: f64 = (locals.var_vgpld__blk933 - locals.var_ps0ld__blk947);
        (assign29010_e40930, (locals.var_vgpld__blk933_dn0 - locals.var_ps0ld__blk947_dn0), (locals.var_vgpld__blk933_dn2 - locals.var_ps0ld__blk947_dn2), (locals.var_vgpld__blk933_dn6 - locals.var_ps0ld__blk947_dn6), (locals.var_vgpld__blk933_dn7 - locals.var_ps0ld__blk947_dn7), (locals.var_vgpld__blk933_dn10 - locals.var_ps0ld__blk947_dn10), (locals.var_vgpld__blk933_dn11 - locals.var_ps0ld__blk947_dn11), (locals.var_vgpld__blk933_dn12 - locals.var_ps0ld__blk947_dn12), (locals.var_vgpld__blk933_dn17 - locals.var_ps0ld__blk947_dn17),)
    } else {
        (locals.var_t2__blk899, locals.var_t2__blk899_dn0, locals.var_t2__blk899_dn2, locals.var_t2__blk899_dn6, locals.var_t2__blk899_dn7, locals.var_t2__blk899_dn10, locals.var_t2__blk899_dn11, locals.var_t2__blk899_dn12, locals.var_t2__blk899_dn17,)
    }
};
        locals.var_t2__blk899 = assign29010_e40932;
        locals.var_t2__blk899_dn0 = assign29010_e40932_d_n0;
        locals.var_t2__blk899_dn2 = assign29010_e40932_d_n2;
        locals.var_t2__blk899_dn6 = assign29010_e40932_d_n6;
        locals.var_t2__blk899_dn7 = assign29010_e40932_d_n7;
        locals.var_t2__blk899_dn10 = assign29010_e40932_d_n10;
        locals.var_t2__blk899_dn11 = assign29010_e40932_d_n11;
        locals.var_t2__blk899_dn12 = assign29010_e40932_d_n12;
        locals.var_t2__blk899_dn17 = assign29010_e40932_d_n17;
        locals.var_t2__blk899_rv = 0.0;

        let (assign29020_e40945, assign29020_e40945_d_n0, assign29020_e40945_d_n2, assign29020_e40945_d_n6, assign29020_e40945_d_n7, assign29020_e40945_d_n10, assign29020_e40945_d_n11, assign29020_e40945_d_n12, assign29020_e40945_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign29020_e40943: f64 = (locals.var_cox0__blk908 * locals.var_t2__blk899);
        (assign29020_e40943, (locals.var_cox0__blk908 * locals.var_t2__blk899_dn0), (locals.var_cox0__blk908 * locals.var_t2__blk899_dn2), (locals.var_cox0__blk908 * locals.var_t2__blk899_dn6), (locals.var_cox0__blk908 * locals.var_t2__blk899_dn7), (locals.var_cox0__blk908 * locals.var_t2__blk899_dn10), (locals.var_cox0__blk908 * locals.var_t2__blk899_dn11), (locals.var_cox0__blk908 * locals.var_t2__blk899_dn12), (locals.var_cox0__blk908 * locals.var_t2__blk899_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign29020_e40945;
        locals.var_qsuld_dn0 = assign29020_e40945_d_n0;
        locals.var_qsuld_dn2 = assign29020_e40945_d_n2;
        locals.var_qsuld_dn6 = assign29020_e40945_d_n6;
        locals.var_qsuld_dn7 = assign29020_e40945_d_n7;
        locals.var_qsuld_dn10 = assign29020_e40945_d_n10;
        locals.var_qsuld_dn11 = assign29020_e40945_d_n11;
        locals.var_qsuld_dn12 = assign29020_e40945_d_n12;
        locals.var_qsuld_dn17 = assign29020_e40945_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign29030_e40956, assign29030_e40956_d_n0, assign29030_e40956_d_n2, assign29030_e40956_d_n6, assign29030_e40956_d_n7, assign29030_e40956_d_n10, assign29030_e40956_d_n11, assign29030_e40956_d_n12, assign29030_e40956_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign29030_e40956;
        locals.var_qbuld_dn0 = assign29030_e40956_d_n0;
        locals.var_qbuld_dn2 = assign29030_e40956_d_n2;
        locals.var_qbuld_dn6 = assign29030_e40956_d_n6;
        locals.var_qbuld_dn7 = assign29030_e40956_d_n7;
        locals.var_qbuld_dn10 = assign29030_e40956_d_n10;
        locals.var_qbuld_dn11 = assign29030_e40956_d_n11;
        locals.var_qbuld_dn12 = assign29030_e40956_d_n12;
        locals.var_qbuld_dn17 = assign29030_e40956_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign29050_e40980, assign29050_e40980_d_n0, assign29050_e40980_d_n2, assign29050_e40980_d_n6, assign29050_e40980_d_n7, assign29050_e40980_d_n10, assign29050_e40980_d_n11, assign29050_e40980_d_n12, assign29050_e40980_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) {
        (3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi__blk945, locals.var_chi__blk945_dn0, locals.var_chi__blk945_dn2, locals.var_chi__blk945_dn6, locals.var_chi__blk945_dn7, locals.var_chi__blk945_dn10, locals.var_chi__blk945_dn11, locals.var_chi__blk945_dn12, locals.var_chi__blk945_dn17,)
    }
};
        locals.var_chi__blk945 = assign29050_e40980;
        locals.var_chi__blk945_dn0 = assign29050_e40980_d_n0;
        locals.var_chi__blk945_dn2 = assign29050_e40980_d_n2;
        locals.var_chi__blk945_dn6 = assign29050_e40980_d_n6;
        locals.var_chi__blk945_dn7 = assign29050_e40980_d_n7;
        locals.var_chi__blk945_dn10 = assign29050_e40980_d_n10;
        locals.var_chi__blk945_dn11 = assign29050_e40980_d_n11;
        locals.var_chi__blk945_dn12 = assign29050_e40980_d_n12;
        locals.var_chi__blk945_dn17 = assign29050_e40980_d_n17;
        locals.var_chi__blk945_rv = 0.0;

        let (assign29060_e40996, assign29060_e40996_d_n0, assign29060_e40996_d_n2, assign29060_e40996_d_n6, assign29060_e40996_d_n7, assign29060_e40996_d_n10, assign29060_e40996_d_n11, assign29060_e40996_d_n12, assign29060_e40996_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) {
        let assign29060_e40992: f64 = (locals.var_chi__blk945 / locals.var_beta);
        let assign29060_e40994: f64 = (assign29060_e40992 - locals.var_vxbgmtcl__blk923);
        (assign29060_e40994, ((locals.var_chi__blk945_dn0 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn0), ((locals.var_chi__blk945_dn2 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn2), ((locals.var_chi__blk945_dn6 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn6), ((locals.var_chi__blk945_dn7 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn7), ((((locals.var_chi__blk945_dn10 * locals.var_beta) - (locals.var_chi__blk945 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl__blk923_dn10), ((locals.var_chi__blk945_dn11 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn11), ((locals.var_chi__blk945_dn12 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn12), ((locals.var_chi__blk945_dn17 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn17),)
    } else {
        (locals.var_ps0_inia__blk948, locals.var_ps0_inia__blk948_dn0, locals.var_ps0_inia__blk948_dn2, locals.var_ps0_inia__blk948_dn6, locals.var_ps0_inia__blk948_dn7, locals.var_ps0_inia__blk948_dn10, locals.var_ps0_inia__blk948_dn11, locals.var_ps0_inia__blk948_dn12, locals.var_ps0_inia__blk948_dn17,)
    }
};
        locals.var_ps0_inia__blk948 = assign29060_e40996;
        locals.var_ps0_inia__blk948_dn0 = assign29060_e40996_d_n0;
        locals.var_ps0_inia__blk948_dn2 = assign29060_e40996_d_n2;
        locals.var_ps0_inia__blk948_dn6 = assign29060_e40996_d_n6;
        locals.var_ps0_inia__blk948_dn7 = assign29060_e40996_d_n7;
        locals.var_ps0_inia__blk948_dn10 = assign29060_e40996_d_n10;
        locals.var_ps0_inia__blk948_dn11 = assign29060_e40996_d_n11;
        locals.var_ps0_inia__blk948_dn12 = assign29060_e40996_d_n12;
        locals.var_ps0_inia__blk948_dn17 = assign29060_e40996_d_n17;
        locals.var_ps0_inia__blk948_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_105(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29070_e41010, assign29070_e41010_d_n0, assign29070_e41010_d_n2, assign29070_e41010_d_n6, assign29070_e41010_d_n7, assign29070_e41010_d_n10, assign29070_e41010_d_n11, assign29070_e41010_d_n12, assign29070_e41010_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) {
        let assign29070_e41007: f64 = (-locals.var_chi__blk945);
        let assign29070_e41008: f64 = (assign29070_e41007).exp();
        (assign29070_e41008, (assign29070_e41008 * (-locals.var_chi__blk945_dn0)), (assign29070_e41008 * (-locals.var_chi__blk945_dn2)), (assign29070_e41008 * (-locals.var_chi__blk945_dn6)), (assign29070_e41008 * (-locals.var_chi__blk945_dn7)), (assign29070_e41008 * (-locals.var_chi__blk945_dn10)), (assign29070_e41008 * (-locals.var_chi__blk945_dn11)), (assign29070_e41008 * (-locals.var_chi__blk945_dn12)), (assign29070_e41008 * (-locals.var_chi__blk945_dn17)),)
    } else {
        (locals.var_ty__blk907, locals.var_ty__blk907_dn0, locals.var_ty__blk907_dn2, locals.var_ty__blk907_dn6, locals.var_ty__blk907_dn7, locals.var_ty__blk907_dn10, locals.var_ty__blk907_dn11, locals.var_ty__blk907_dn12, locals.var_ty__blk907_dn17,)
    }
};
        locals.var_ty__blk907 = assign29070_e41010;
        locals.var_ty__blk907_dn0 = assign29070_e41010_d_n0;
        locals.var_ty__blk907_dn2 = assign29070_e41010_d_n2;
        locals.var_ty__blk907_dn6 = assign29070_e41010_d_n6;
        locals.var_ty__blk907_dn7 = assign29070_e41010_d_n7;
        locals.var_ty__blk907_dn10 = assign29070_e41010_d_n10;
        locals.var_ty__blk907_dn11 = assign29070_e41010_d_n11;
        locals.var_ty__blk907_dn12 = assign29070_e41010_d_n12;
        locals.var_ty__blk907_dn17 = assign29070_e41010_d_n17;
        locals.var_ty__blk907_rv = 0.0;

        let (assign29080_e41038, assign29080_e41038_d_n0, assign29080_e41038_d_n2, assign29080_e41038_d_n6, assign29080_e41038_d_n7, assign29080_e41038_d_n10, assign29080_e41038_d_n11, assign29080_e41038_d_n12, assign29080_e41038_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) {
        let assign29080_e41025: f64 = (locals.var_vgpld__blk933 + locals.var_vxbgmtcl__blk923);
        let assign29080_e41026: f64 = (locals.var_beta * assign29080_e41025);
        let assign29080_e41028: f64 = (assign29080_e41026 - 1.0);
        let assign29080_e41030: f64 = (assign29080_e41028 + locals.var_ty__blk907);
        let assign29080_e41031: f64 = (4.0 * assign29080_e41030);
        let assign29080_e41034: f64 = (locals.var_fac1p2__blk932 * locals.var_beta2);
        let assign29080_e41035: f64 = (assign29080_e41031 / assign29080_e41034);
        let assign29080_e41036: f64 = (1.0 + assign29080_e41035);
        (assign29080_e41036, ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk933_dn0 + locals.var_vxbgmtcl__blk923_dn0)) + locals.var_ty__blk907_dn0)) * assign29080_e41034) - (assign29080_e41031 * (locals.var_fac1p2__blk932_dn0 * locals.var_beta2))) / (assign29080_e41034 * assign29080_e41034)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk933_dn2 + locals.var_vxbgmtcl__blk923_dn2)) + locals.var_ty__blk907_dn2)) * assign29080_e41034) - (assign29080_e41031 * (locals.var_fac1p2__blk932_dn2 * locals.var_beta2))) / (assign29080_e41034 * assign29080_e41034)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk933_dn6 + locals.var_vxbgmtcl__blk923_dn6)) + locals.var_ty__blk907_dn6)) * assign29080_e41034) - (assign29080_e41031 * (locals.var_fac1p2__blk932_dn6 * locals.var_beta2))) / (assign29080_e41034 * assign29080_e41034)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk933_dn7 + locals.var_vxbgmtcl__blk923_dn7)) + locals.var_ty__blk907_dn7)) * assign29080_e41034) - (assign29080_e41031 * (locals.var_fac1p2__blk932_dn7 * locals.var_beta2))) / (assign29080_e41034 * assign29080_e41034)), ((((4.0 * (((locals.var_beta_dn10 * assign29080_e41025) + (locals.var_beta * (locals.var_vgpld__blk933_dn10 + locals.var_vxbgmtcl__blk923_dn10))) + locals.var_ty__blk907_dn10)) * assign29080_e41034) - (assign29080_e41031 * ((locals.var_fac1p2__blk932_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk932 * locals.var_beta2_dn10)))) / (assign29080_e41034 * assign29080_e41034)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk933_dn11 + locals.var_vxbgmtcl__blk923_dn11)) + locals.var_ty__blk907_dn11)) * assign29080_e41034) - (assign29080_e41031 * (locals.var_fac1p2__blk932_dn11 * locals.var_beta2))) / (assign29080_e41034 * assign29080_e41034)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk933_dn12 + locals.var_vxbgmtcl__blk923_dn12)) + locals.var_ty__blk907_dn12)) * assign29080_e41034) - (assign29080_e41031 * (locals.var_fac1p2__blk932_dn12 * locals.var_beta2))) / (assign29080_e41034 * assign29080_e41034)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk933_dn17 + locals.var_vxbgmtcl__blk923_dn17)) + locals.var_ty__blk907_dn17)) * assign29080_e41034) - (assign29080_e41031 * (locals.var_fac1p2__blk932_dn17 * locals.var_beta2))) / (assign29080_e41034 * assign29080_e41034)),)
    } else {
        (locals.var_tx__blk906, locals.var_tx__blk906_dn0, locals.var_tx__blk906_dn2, locals.var_tx__blk906_dn6, locals.var_tx__blk906_dn7, locals.var_tx__blk906_dn10, locals.var_tx__blk906_dn11, locals.var_tx__blk906_dn12, locals.var_tx__blk906_dn17,)
    }
};
        locals.var_tx__blk906 = assign29080_e41038;
        locals.var_tx__blk906_dn0 = assign29080_e41038_d_n0;
        locals.var_tx__blk906_dn2 = assign29080_e41038_d_n2;
        locals.var_tx__blk906_dn6 = assign29080_e41038_d_n6;
        locals.var_tx__blk906_dn7 = assign29080_e41038_d_n7;
        locals.var_tx__blk906_dn10 = assign29080_e41038_d_n10;
        locals.var_tx__blk906_dn11 = assign29080_e41038_d_n11;
        locals.var_tx__blk906_dn12 = assign29080_e41038_d_n12;
        locals.var_tx__blk906_dn17 = assign29080_e41038_d_n17;
        locals.var_tx__blk906_rv = 0.0;

        let assign29090_e41042: f64 = (10.0 * 2.220446049250313e-16);
        let assign29090_e41043: f64 = if locals.var_tx__blk906 < assign29090_e41042 { 1.0 } else { 0.0 };
        locals.var_guard986 = assign29090_e41043;
        locals.var_guard986_rv = 0.0;

        let (assign29100_e41059, assign29100_e41059_d_n0, assign29100_e41059_d_n2, assign29100_e41059_d_n6, assign29100_e41059_d_n7, assign29100_e41059_d_n10, assign29100_e41059_d_n11, assign29100_e41059_d_n12, assign29100_e41059_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29100_e41057: f64 = (10.0 * 2.220446049250313e-16);
        (assign29100_e41057, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk906, locals.var_tx__blk906_dn0, locals.var_tx__blk906_dn2, locals.var_tx__blk906_dn6, locals.var_tx__blk906_dn7, locals.var_tx__blk906_dn10, locals.var_tx__blk906_dn11, locals.var_tx__blk906_dn12, locals.var_tx__blk906_dn17,)
    }
};
        locals.var_tx__blk906 = assign29100_e41059;
        locals.var_tx__blk906_dn0 = assign29100_e41059_d_n0;
        locals.var_tx__blk906_dn2 = assign29100_e41059_d_n2;
        locals.var_tx__blk906_dn6 = assign29100_e41059_d_n6;
        locals.var_tx__blk906_dn7 = assign29100_e41059_d_n7;
        locals.var_tx__blk906_dn10 = assign29100_e41059_d_n10;
        locals.var_tx__blk906_dn11 = assign29100_e41059_d_n11;
        locals.var_tx__blk906_dn12 = assign29100_e41059_d_n12;
        locals.var_tx__blk906_dn17 = assign29100_e41059_d_n17;
        locals.var_tx__blk906_rv = 0.0;

        let (assign29110_e41082, assign29110_e41082_d_n0, assign29110_e41082_d_n2, assign29110_e41082_d_n6, assign29110_e41082_d_n7, assign29110_e41082_d_n10, assign29110_e41082_d_n11, assign29110_e41082_d_n12, assign29110_e41082_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) {
        let assign29110_e41072: f64 = (locals.var_fac1p2__blk932 * locals.var_beta);
        let assign29110_e41074: f64 = (assign29110_e41072 / 2.0);
        let assign29110_e41077: f64 = (locals.var_tx__blk906).sqrt();
        let assign29110_e41078: f64 = (1.0 - assign29110_e41077);
        let assign29110_e41079: f64 = (assign29110_e41074 * assign29110_e41078);
        let assign29110_e41080: f64 = (locals.var_vgpld__blk933 + assign29110_e41079);
        (assign29110_e41080, (locals.var_vgpld__blk933_dn0 + ((((locals.var_fac1p2__blk932_dn0 * locals.var_beta) / 2.0) * assign29110_e41078) + (assign29110_e41074 * (-(locals.var_tx__blk906_dn0 / (2.0 * assign29110_e41077)))))), (locals.var_vgpld__blk933_dn2 + ((((locals.var_fac1p2__blk932_dn2 * locals.var_beta) / 2.0) * assign29110_e41078) + (assign29110_e41074 * (-(locals.var_tx__blk906_dn2 / (2.0 * assign29110_e41077)))))), (locals.var_vgpld__blk933_dn6 + ((((locals.var_fac1p2__blk932_dn6 * locals.var_beta) / 2.0) * assign29110_e41078) + (assign29110_e41074 * (-(locals.var_tx__blk906_dn6 / (2.0 * assign29110_e41077)))))), (locals.var_vgpld__blk933_dn7 + ((((locals.var_fac1p2__blk932_dn7 * locals.var_beta) / 2.0) * assign29110_e41078) + (assign29110_e41074 * (-(locals.var_tx__blk906_dn7 / (2.0 * assign29110_e41077)))))), (locals.var_vgpld__blk933_dn10 + (((((locals.var_fac1p2__blk932_dn10 * locals.var_beta) + (locals.var_fac1p2__blk932 * locals.var_beta_dn10)) / 2.0) * assign29110_e41078) + (assign29110_e41074 * (-(locals.var_tx__blk906_dn10 / (2.0 * assign29110_e41077)))))), (locals.var_vgpld__blk933_dn11 + ((((locals.var_fac1p2__blk932_dn11 * locals.var_beta) / 2.0) * assign29110_e41078) + (assign29110_e41074 * (-(locals.var_tx__blk906_dn11 / (2.0 * assign29110_e41077)))))), (locals.var_vgpld__blk933_dn12 + ((((locals.var_fac1p2__blk932_dn12 * locals.var_beta) / 2.0) * assign29110_e41078) + (assign29110_e41074 * (-(locals.var_tx__blk906_dn12 / (2.0 * assign29110_e41077)))))), (locals.var_vgpld__blk933_dn17 + ((((locals.var_fac1p2__blk932_dn17 * locals.var_beta) / 2.0) * assign29110_e41078) + (assign29110_e41074 * (-(locals.var_tx__blk906_dn17 / (2.0 * assign29110_e41077)))))),)
    } else {
        (locals.var_ps0_inia__blk948, locals.var_ps0_inia__blk948_dn0, locals.var_ps0_inia__blk948_dn2, locals.var_ps0_inia__blk948_dn6, locals.var_ps0_inia__blk948_dn7, locals.var_ps0_inia__blk948_dn10, locals.var_ps0_inia__blk948_dn11, locals.var_ps0_inia__blk948_dn12, locals.var_ps0_inia__blk948_dn17,)
    }
};
        locals.var_ps0_inia__blk948 = assign29110_e41082;
        locals.var_ps0_inia__blk948_dn0 = assign29110_e41082_d_n0;
        locals.var_ps0_inia__blk948_dn2 = assign29110_e41082_d_n2;
        locals.var_ps0_inia__blk948_dn6 = assign29110_e41082_d_n6;
        locals.var_ps0_inia__blk948_dn7 = assign29110_e41082_d_n7;
        locals.var_ps0_inia__blk948_dn10 = assign29110_e41082_d_n10;
        locals.var_ps0_inia__blk948_dn11 = assign29110_e41082_d_n11;
        locals.var_ps0_inia__blk948_dn12 = assign29110_e41082_d_n12;
        locals.var_ps0_inia__blk948_dn17 = assign29110_e41082_d_n17;
        locals.var_ps0_inia__blk948_rv = 0.0;

        let (assign29120_e41098, assign29120_e41098_d_n0, assign29120_e41098_d_n2, assign29120_e41098_d_n6, assign29120_e41098_d_n7, assign29120_e41098_d_n10, assign29120_e41098_d_n11, assign29120_e41098_d_n12, assign29120_e41098_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) {
        let assign29120_e41095: f64 = (locals.var_ps0_inia__blk948 + locals.var_vxbgmtcl__blk923);
        let assign29120_e41096: f64 = (locals.var_beta * assign29120_e41095);
        (assign29120_e41096, (locals.var_beta * (locals.var_ps0_inia__blk948_dn0 + locals.var_vxbgmtcl__blk923_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn2 + locals.var_vxbgmtcl__blk923_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn6 + locals.var_vxbgmtcl__blk923_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn7 + locals.var_vxbgmtcl__blk923_dn7)), ((locals.var_beta_dn10 * assign29120_e41095) + (locals.var_beta * (locals.var_ps0_inia__blk948_dn10 + locals.var_vxbgmtcl__blk923_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk948_dn11 + locals.var_vxbgmtcl__blk923_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn12 + locals.var_vxbgmtcl__blk923_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn17 + locals.var_vxbgmtcl__blk923_dn17)),)
    } else {
        (locals.var_chi__blk945, locals.var_chi__blk945_dn0, locals.var_chi__blk945_dn2, locals.var_chi__blk945_dn6, locals.var_chi__blk945_dn7, locals.var_chi__blk945_dn10, locals.var_chi__blk945_dn11, locals.var_chi__blk945_dn12, locals.var_chi__blk945_dn17,)
    }
};
        locals.var_chi__blk945 = assign29120_e41098;
        locals.var_chi__blk945_dn0 = assign29120_e41098_d_n0;
        locals.var_chi__blk945_dn2 = assign29120_e41098_d_n2;
        locals.var_chi__blk945_dn6 = assign29120_e41098_d_n6;
        locals.var_chi__blk945_dn7 = assign29120_e41098_d_n7;
        locals.var_chi__blk945_dn10 = assign29120_e41098_d_n10;
        locals.var_chi__blk945_dn11 = assign29120_e41098_d_n11;
        locals.var_chi__blk945_dn12 = assign29120_e41098_d_n12;
        locals.var_chi__blk945_dn17 = assign29120_e41098_d_n17;
        locals.var_chi__blk945_rv = 0.0;

        let (assign29130_e41112, assign29130_e41112_d_n0, assign29130_e41112_d_n2, assign29130_e41112_d_n6, assign29130_e41112_d_n7, assign29130_e41112_d_n10, assign29130_e41112_d_n11, assign29130_e41112_d_n12, assign29130_e41112_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) {
        let assign29130_e41109: f64 = (-locals.var_chi__blk945);
        let assign29130_e41110: f64 = (assign29130_e41109).exp();
        (assign29130_e41110, (assign29130_e41110 * (-locals.var_chi__blk945_dn0)), (assign29130_e41110 * (-locals.var_chi__blk945_dn2)), (assign29130_e41110 * (-locals.var_chi__blk945_dn6)), (assign29130_e41110 * (-locals.var_chi__blk945_dn7)), (assign29130_e41110 * (-locals.var_chi__blk945_dn10)), (assign29130_e41110 * (-locals.var_chi__blk945_dn11)), (assign29130_e41110 * (-locals.var_chi__blk945_dn12)), (assign29130_e41110 * (-locals.var_chi__blk945_dn17)),)
    } else {
        (locals.var_ty__blk907, locals.var_ty__blk907_dn0, locals.var_ty__blk907_dn2, locals.var_ty__blk907_dn6, locals.var_ty__blk907_dn7, locals.var_ty__blk907_dn10, locals.var_ty__blk907_dn11, locals.var_ty__blk907_dn12, locals.var_ty__blk907_dn17,)
    }
};
        locals.var_ty__blk907 = assign29130_e41112;
        locals.var_ty__blk907_dn0 = assign29130_e41112_d_n0;
        locals.var_ty__blk907_dn2 = assign29130_e41112_d_n2;
        locals.var_ty__blk907_dn6 = assign29130_e41112_d_n6;
        locals.var_ty__blk907_dn7 = assign29130_e41112_d_n7;
        locals.var_ty__blk907_dn10 = assign29130_e41112_d_n10;
        locals.var_ty__blk907_dn11 = assign29130_e41112_d_n11;
        locals.var_ty__blk907_dn12 = assign29130_e41112_d_n12;
        locals.var_ty__blk907_dn17 = assign29130_e41112_d_n17;
        locals.var_ty__blk907_rv = 0.0;

        let (assign29140_e41140, assign29140_e41140_d_n0, assign29140_e41140_d_n2, assign29140_e41140_d_n6, assign29140_e41140_d_n7, assign29140_e41140_d_n10, assign29140_e41140_d_n11, assign29140_e41140_d_n12, assign29140_e41140_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) {
        let assign29140_e41127: f64 = (locals.var_vgpld__blk933 + locals.var_vxbgmtcl__blk923);
        let assign29140_e41128: f64 = (locals.var_beta * assign29140_e41127);
        let assign29140_e41130: f64 = (assign29140_e41128 - 1.0);
        let assign29140_e41132: f64 = (assign29140_e41130 + locals.var_ty__blk907);
        let assign29140_e41133: f64 = (4.0 * assign29140_e41132);
        let assign29140_e41136: f64 = (locals.var_fac1p2__blk932 * locals.var_beta2);
        let assign29140_e41137: f64 = (assign29140_e41133 / assign29140_e41136);
        let assign29140_e41138: f64 = (1.0 + assign29140_e41137);
        (assign29140_e41138, ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk933_dn0 + locals.var_vxbgmtcl__blk923_dn0)) + locals.var_ty__blk907_dn0)) * assign29140_e41136) - (assign29140_e41133 * (locals.var_fac1p2__blk932_dn0 * locals.var_beta2))) / (assign29140_e41136 * assign29140_e41136)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk933_dn2 + locals.var_vxbgmtcl__blk923_dn2)) + locals.var_ty__blk907_dn2)) * assign29140_e41136) - (assign29140_e41133 * (locals.var_fac1p2__blk932_dn2 * locals.var_beta2))) / (assign29140_e41136 * assign29140_e41136)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk933_dn6 + locals.var_vxbgmtcl__blk923_dn6)) + locals.var_ty__blk907_dn6)) * assign29140_e41136) - (assign29140_e41133 * (locals.var_fac1p2__blk932_dn6 * locals.var_beta2))) / (assign29140_e41136 * assign29140_e41136)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk933_dn7 + locals.var_vxbgmtcl__blk923_dn7)) + locals.var_ty__blk907_dn7)) * assign29140_e41136) - (assign29140_e41133 * (locals.var_fac1p2__blk932_dn7 * locals.var_beta2))) / (assign29140_e41136 * assign29140_e41136)), ((((4.0 * (((locals.var_beta_dn10 * assign29140_e41127) + (locals.var_beta * (locals.var_vgpld__blk933_dn10 + locals.var_vxbgmtcl__blk923_dn10))) + locals.var_ty__blk907_dn10)) * assign29140_e41136) - (assign29140_e41133 * ((locals.var_fac1p2__blk932_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk932 * locals.var_beta2_dn10)))) / (assign29140_e41136 * assign29140_e41136)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk933_dn11 + locals.var_vxbgmtcl__blk923_dn11)) + locals.var_ty__blk907_dn11)) * assign29140_e41136) - (assign29140_e41133 * (locals.var_fac1p2__blk932_dn11 * locals.var_beta2))) / (assign29140_e41136 * assign29140_e41136)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk933_dn12 + locals.var_vxbgmtcl__blk923_dn12)) + locals.var_ty__blk907_dn12)) * assign29140_e41136) - (assign29140_e41133 * (locals.var_fac1p2__blk932_dn12 * locals.var_beta2))) / (assign29140_e41136 * assign29140_e41136)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk933_dn17 + locals.var_vxbgmtcl__blk923_dn17)) + locals.var_ty__blk907_dn17)) * assign29140_e41136) - (assign29140_e41133 * (locals.var_fac1p2__blk932_dn17 * locals.var_beta2))) / (assign29140_e41136 * assign29140_e41136)),)
    } else {
        (locals.var_tx__blk906, locals.var_tx__blk906_dn0, locals.var_tx__blk906_dn2, locals.var_tx__blk906_dn6, locals.var_tx__blk906_dn7, locals.var_tx__blk906_dn10, locals.var_tx__blk906_dn11, locals.var_tx__blk906_dn12, locals.var_tx__blk906_dn17,)
    }
};
        locals.var_tx__blk906 = assign29140_e41140;
        locals.var_tx__blk906_dn0 = assign29140_e41140_d_n0;
        locals.var_tx__blk906_dn2 = assign29140_e41140_d_n2;
        locals.var_tx__blk906_dn6 = assign29140_e41140_d_n6;
        locals.var_tx__blk906_dn7 = assign29140_e41140_d_n7;
        locals.var_tx__blk906_dn10 = assign29140_e41140_d_n10;
        locals.var_tx__blk906_dn11 = assign29140_e41140_d_n11;
        locals.var_tx__blk906_dn12 = assign29140_e41140_d_n12;
        locals.var_tx__blk906_dn17 = assign29140_e41140_d_n17;
        locals.var_tx__blk906_rv = 0.0;

        let assign29150_e41144: f64 = (10.0 * 2.220446049250313e-16);
        let assign29150_e41145: f64 = if locals.var_tx__blk906 < assign29150_e41144 { 1.0 } else { 0.0 };
        locals.var_guard987 = assign29150_e41145;
        locals.var_guard987_rv = 0.0;

        let (assign29160_e41161, assign29160_e41161_d_n0, assign29160_e41161_d_n2, assign29160_e41161_d_n6, assign29160_e41161_d_n7, assign29160_e41161_d_n10, assign29160_e41161_d_n11, assign29160_e41161_d_n12, assign29160_e41161_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29160_e41159: f64 = (10.0 * 2.220446049250313e-16);
        (assign29160_e41159, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk906, locals.var_tx__blk906_dn0, locals.var_tx__blk906_dn2, locals.var_tx__blk906_dn6, locals.var_tx__blk906_dn7, locals.var_tx__blk906_dn10, locals.var_tx__blk906_dn11, locals.var_tx__blk906_dn12, locals.var_tx__blk906_dn17,)
    }
};
        locals.var_tx__blk906 = assign29160_e41161;
        locals.var_tx__blk906_dn0 = assign29160_e41161_d_n0;
        locals.var_tx__blk906_dn2 = assign29160_e41161_d_n2;
        locals.var_tx__blk906_dn6 = assign29160_e41161_d_n6;
        locals.var_tx__blk906_dn7 = assign29160_e41161_d_n7;
        locals.var_tx__blk906_dn10 = assign29160_e41161_d_n10;
        locals.var_tx__blk906_dn11 = assign29160_e41161_d_n11;
        locals.var_tx__blk906_dn12 = assign29160_e41161_d_n12;
        locals.var_tx__blk906_dn17 = assign29160_e41161_d_n17;
        locals.var_tx__blk906_rv = 0.0;

        let (assign29170_e41184, assign29170_e41184_d_n0, assign29170_e41184_d_n2, assign29170_e41184_d_n6, assign29170_e41184_d_n7, assign29170_e41184_d_n10, assign29170_e41184_d_n11, assign29170_e41184_d_n12, assign29170_e41184_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) {
        let assign29170_e41174: f64 = (locals.var_fac1p2__blk932 * locals.var_beta);
        let assign29170_e41176: f64 = (assign29170_e41174 / 2.0);
        let assign29170_e41179: f64 = (locals.var_tx__blk906).sqrt();
        let assign29170_e41180: f64 = (1.0 - assign29170_e41179);
        let assign29170_e41181: f64 = (assign29170_e41176 * assign29170_e41180);
        let assign29170_e41182: f64 = (locals.var_vgpld__blk933 + assign29170_e41181);
        (assign29170_e41182, (locals.var_vgpld__blk933_dn0 + ((((locals.var_fac1p2__blk932_dn0 * locals.var_beta) / 2.0) * assign29170_e41180) + (assign29170_e41176 * (-(locals.var_tx__blk906_dn0 / (2.0 * assign29170_e41179)))))), (locals.var_vgpld__blk933_dn2 + ((((locals.var_fac1p2__blk932_dn2 * locals.var_beta) / 2.0) * assign29170_e41180) + (assign29170_e41176 * (-(locals.var_tx__blk906_dn2 / (2.0 * assign29170_e41179)))))), (locals.var_vgpld__blk933_dn6 + ((((locals.var_fac1p2__blk932_dn6 * locals.var_beta) / 2.0) * assign29170_e41180) + (assign29170_e41176 * (-(locals.var_tx__blk906_dn6 / (2.0 * assign29170_e41179)))))), (locals.var_vgpld__blk933_dn7 + ((((locals.var_fac1p2__blk932_dn7 * locals.var_beta) / 2.0) * assign29170_e41180) + (assign29170_e41176 * (-(locals.var_tx__blk906_dn7 / (2.0 * assign29170_e41179)))))), (locals.var_vgpld__blk933_dn10 + (((((locals.var_fac1p2__blk932_dn10 * locals.var_beta) + (locals.var_fac1p2__blk932 * locals.var_beta_dn10)) / 2.0) * assign29170_e41180) + (assign29170_e41176 * (-(locals.var_tx__blk906_dn10 / (2.0 * assign29170_e41179)))))), (locals.var_vgpld__blk933_dn11 + ((((locals.var_fac1p2__blk932_dn11 * locals.var_beta) / 2.0) * assign29170_e41180) + (assign29170_e41176 * (-(locals.var_tx__blk906_dn11 / (2.0 * assign29170_e41179)))))), (locals.var_vgpld__blk933_dn12 + ((((locals.var_fac1p2__blk932_dn12 * locals.var_beta) / 2.0) * assign29170_e41180) + (assign29170_e41176 * (-(locals.var_tx__blk906_dn12 / (2.0 * assign29170_e41179)))))), (locals.var_vgpld__blk933_dn17 + ((((locals.var_fac1p2__blk932_dn17 * locals.var_beta) / 2.0) * assign29170_e41180) + (assign29170_e41176 * (-(locals.var_tx__blk906_dn17 / (2.0 * assign29170_e41179)))))),)
    } else {
        (locals.var_ps0_inia__blk948, locals.var_ps0_inia__blk948_dn0, locals.var_ps0_inia__blk948_dn2, locals.var_ps0_inia__blk948_dn6, locals.var_ps0_inia__blk948_dn7, locals.var_ps0_inia__blk948_dn10, locals.var_ps0_inia__blk948_dn11, locals.var_ps0_inia__blk948_dn12, locals.var_ps0_inia__blk948_dn17,)
    }
};
        locals.var_ps0_inia__blk948 = assign29170_e41184;
        locals.var_ps0_inia__blk948_dn0 = assign29170_e41184_d_n0;
        locals.var_ps0_inia__blk948_dn2 = assign29170_e41184_d_n2;
        locals.var_ps0_inia__blk948_dn6 = assign29170_e41184_d_n6;
        locals.var_ps0_inia__blk948_dn7 = assign29170_e41184_d_n7;
        locals.var_ps0_inia__blk948_dn10 = assign29170_e41184_d_n10;
        locals.var_ps0_inia__blk948_dn11 = assign29170_e41184_d_n11;
        locals.var_ps0_inia__blk948_dn12 = assign29170_e41184_d_n12;
        locals.var_ps0_inia__blk948_dn17 = assign29170_e41184_d_n17;
        locals.var_ps0_inia__blk948_rv = 0.0;

        let (assign29180_e41200, assign29180_e41200_d_n0, assign29180_e41200_d_n2, assign29180_e41200_d_n6, assign29180_e41200_d_n7, assign29180_e41200_d_n10, assign29180_e41200_d_n11, assign29180_e41200_d_n12, assign29180_e41200_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) {
        let assign29180_e41197: f64 = (locals.var_ps0_inia__blk948 + locals.var_vxbgmtcl__blk923);
        let assign29180_e41198: f64 = (locals.var_beta * assign29180_e41197);
        (assign29180_e41198, (locals.var_beta * (locals.var_ps0_inia__blk948_dn0 + locals.var_vxbgmtcl__blk923_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn2 + locals.var_vxbgmtcl__blk923_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn6 + locals.var_vxbgmtcl__blk923_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn7 + locals.var_vxbgmtcl__blk923_dn7)), ((locals.var_beta_dn10 * assign29180_e41197) + (locals.var_beta * (locals.var_ps0_inia__blk948_dn10 + locals.var_vxbgmtcl__blk923_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk948_dn11 + locals.var_vxbgmtcl__blk923_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn12 + locals.var_vxbgmtcl__blk923_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn17 + locals.var_vxbgmtcl__blk923_dn17)),)
    } else {
        (locals.var_chi__blk945, locals.var_chi__blk945_dn0, locals.var_chi__blk945_dn2, locals.var_chi__blk945_dn6, locals.var_chi__blk945_dn7, locals.var_chi__blk945_dn10, locals.var_chi__blk945_dn11, locals.var_chi__blk945_dn12, locals.var_chi__blk945_dn17,)
    }
};
        locals.var_chi__blk945 = assign29180_e41200;
        locals.var_chi__blk945_dn0 = assign29180_e41200_d_n0;
        locals.var_chi__blk945_dn2 = assign29180_e41200_d_n2;
        locals.var_chi__blk945_dn6 = assign29180_e41200_d_n6;
        locals.var_chi__blk945_dn7 = assign29180_e41200_d_n7;
        locals.var_chi__blk945_dn10 = assign29180_e41200_d_n10;
        locals.var_chi__blk945_dn11 = assign29180_e41200_d_n11;
        locals.var_chi__blk945_dn12 = assign29180_e41200_d_n12;
        locals.var_chi__blk945_dn17 = assign29180_e41200_d_n17;
        locals.var_chi__blk945_rv = 0.0;

        let assign29190_e41203: f64 = if locals.var_chi__blk945 < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard988 = assign29190_e41203;
        locals.var_guard988_rv = 0.0;

        let (assign29210_e41248,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard988 != 0.0)) {
        let assign29210_e41232: f64 = (9.0 * 1.414213562373095);
        let assign29210_e41233: f64 = (1.0 / assign29210_e41232);
        let assign29210_e41237: f64 = (7.0 * 0.049787068367863944);
        let assign29210_e41238: f64 = (5.0 + assign29210_e41237);
        let assign29210_e41242: f64 = (2.0 + 0.049787068367863944);
        let assign29210_e41243: f64 = (assign29210_e41242).sqrt();
        let assign29210_e41244: f64 = (54.0 * assign29210_e41243);
        let assign29210_e41245: f64 = (assign29210_e41238 / assign29210_e41244);
        let assign29210_e41246: f64 = (assign29210_e41233 - assign29210_e41245);
        (assign29210_e41246,)
    } else {
        (locals.var_ta__blk949,)
    }
};
        locals.var_ta__blk949 = assign29210_e41248;
        locals.var_ta__blk949_rv = 0.0;

        let (assign29220_e41275,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard988 != 0.0)) {
        let assign29220_e41262: f64 = (1.0 + 0.049787068367863944);
        let assign29220_e41266: f64 = (2.0 + 0.049787068367863944);
        let assign29220_e41267: f64 = (assign29220_e41266).sqrt();
        let assign29220_e41268: f64 = (2.0 * assign29220_e41267);
        let assign29220_e41269: f64 = (assign29220_e41262 / assign29220_e41268);
        let assign29220_e41272: f64 = (1.414213562373095 / 3.0);
        let assign29220_e41273: f64 = (assign29220_e41269 - assign29220_e41272);
        (assign29220_e41273,)
    } else {
        (locals.var_tb__blk950,)
    }
};
        locals.var_tb__blk950 = assign29220_e41275;
        locals.var_tb__blk950_rv = 0.0;

        let (assign29230_e41297, assign29230_e41297_d_n0, assign29230_e41297_d_n2, assign29230_e41297_d_n6, assign29230_e41297_d_n7, assign29230_e41297_d_n10, assign29230_e41297_d_n11, assign29230_e41297_d_n12, assign29230_e41297_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard988 != 0.0)) {
        let assign29230_e41289: f64 = (1.0 / 1.414213562373095);
        let assign29230_e41293: f64 = (locals.var_beta * locals.var_fac1__blk931);
        let assign29230_e41294: f64 = (1.0 / assign29230_e41293);
        let assign29230_e41295: f64 = (assign29230_e41289 + assign29230_e41294);
        (assign29230_e41295, (-((locals.var_beta * locals.var_fac1__blk931_dn0) / (assign29230_e41293 * assign29230_e41293))), (-((locals.var_beta * locals.var_fac1__blk931_dn2) / (assign29230_e41293 * assign29230_e41293))), (-((locals.var_beta * locals.var_fac1__blk931_dn6) / (assign29230_e41293 * assign29230_e41293))), (-((locals.var_beta * locals.var_fac1__blk931_dn7) / (assign29230_e41293 * assign29230_e41293))), (-(((locals.var_beta_dn10 * locals.var_fac1__blk931) + (locals.var_beta * locals.var_fac1__blk931_dn10)) / (assign29230_e41293 * assign29230_e41293))), (-((locals.var_beta * locals.var_fac1__blk931_dn11) / (assign29230_e41293 * assign29230_e41293))), (-((locals.var_beta * locals.var_fac1__blk931_dn12) / (assign29230_e41293 * assign29230_e41293))), (-((locals.var_beta * locals.var_fac1__blk931_dn17) / (assign29230_e41293 * assign29230_e41293))),)
    } else {
        (locals.var_tc__blk951, locals.var_tc__blk951_dn0, locals.var_tc__blk951_dn2, locals.var_tc__blk951_dn6, locals.var_tc__blk951_dn7, locals.var_tc__blk951_dn10, locals.var_tc__blk951_dn11, locals.var_tc__blk951_dn12, locals.var_tc__blk951_dn17,)
    }
};
        locals.var_tc__blk951 = assign29230_e41297;
        locals.var_tc__blk951_dn0 = assign29230_e41297_d_n0;
        locals.var_tc__blk951_dn2 = assign29230_e41297_d_n2;
        locals.var_tc__blk951_dn6 = assign29230_e41297_d_n6;
        locals.var_tc__blk951_dn7 = assign29230_e41297_d_n7;
        locals.var_tc__blk951_dn10 = assign29230_e41297_d_n10;
        locals.var_tc__blk951_dn11 = assign29230_e41297_d_n11;
        locals.var_tc__blk951_dn12 = assign29230_e41297_d_n12;
        locals.var_tc__blk951_dn17 = assign29230_e41297_d_n17;
        locals.var_tc__blk951_rv = 0.0;

        let (assign29240_e41316, assign29240_e41316_d_n0, assign29240_e41316_d_n2, assign29240_e41316_d_n6, assign29240_e41316_d_n7, assign29240_e41316_d_n10, assign29240_e41316_d_n11, assign29240_e41316_d_n12, assign29240_e41316_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard988 != 0.0)) {
        let assign29240_e41311: f64 = (locals.var_vgpld__blk933 + locals.var_vxbgmtcl__blk923);
        let assign29240_e41312: f64 = (-assign29240_e41311);
        let assign29240_e41314: f64 = (assign29240_e41312 / locals.var_fac1__blk931);
        (assign29240_e41314, ((((-(locals.var_vgpld__blk933_dn0 + locals.var_vxbgmtcl__blk923_dn0)) * locals.var_fac1__blk931) - (assign29240_e41312 * locals.var_fac1__blk931_dn0)) / (locals.var_fac1__blk931 * locals.var_fac1__blk931)), ((((-(locals.var_vgpld__blk933_dn2 + locals.var_vxbgmtcl__blk923_dn2)) * locals.var_fac1__blk931) - (assign29240_e41312 * locals.var_fac1__blk931_dn2)) / (locals.var_fac1__blk931 * locals.var_fac1__blk931)), ((((-(locals.var_vgpld__blk933_dn6 + locals.var_vxbgmtcl__blk923_dn6)) * locals.var_fac1__blk931) - (assign29240_e41312 * locals.var_fac1__blk931_dn6)) / (locals.var_fac1__blk931 * locals.var_fac1__blk931)), ((((-(locals.var_vgpld__blk933_dn7 + locals.var_vxbgmtcl__blk923_dn7)) * locals.var_fac1__blk931) - (assign29240_e41312 * locals.var_fac1__blk931_dn7)) / (locals.var_fac1__blk931 * locals.var_fac1__blk931)), ((((-(locals.var_vgpld__blk933_dn10 + locals.var_vxbgmtcl__blk923_dn10)) * locals.var_fac1__blk931) - (assign29240_e41312 * locals.var_fac1__blk931_dn10)) / (locals.var_fac1__blk931 * locals.var_fac1__blk931)), ((((-(locals.var_vgpld__blk933_dn11 + locals.var_vxbgmtcl__blk923_dn11)) * locals.var_fac1__blk931) - (assign29240_e41312 * locals.var_fac1__blk931_dn11)) / (locals.var_fac1__blk931 * locals.var_fac1__blk931)), ((((-(locals.var_vgpld__blk933_dn12 + locals.var_vxbgmtcl__blk923_dn12)) * locals.var_fac1__blk931) - (assign29240_e41312 * locals.var_fac1__blk931_dn12)) / (locals.var_fac1__blk931 * locals.var_fac1__blk931)), ((((-(locals.var_vgpld__blk933_dn17 + locals.var_vxbgmtcl__blk923_dn17)) * locals.var_fac1__blk931) - (assign29240_e41312 * locals.var_fac1__blk931_dn17)) / (locals.var_fac1__blk931 * locals.var_fac1__blk931)),)
    } else {
        (locals.var_td__blk952, locals.var_td__blk952_dn0, locals.var_td__blk952_dn2, locals.var_td__blk952_dn6, locals.var_td__blk952_dn7, locals.var_td__blk952_dn10, locals.var_td__blk952_dn11, locals.var_td__blk952_dn12, locals.var_td__blk952_dn17,)
    }
};
        locals.var_td__blk952 = assign29240_e41316;
        locals.var_td__blk952_dn0 = assign29240_e41316_d_n0;
        locals.var_td__blk952_dn2 = assign29240_e41316_d_n2;
        locals.var_td__blk952_dn6 = assign29240_e41316_d_n6;
        locals.var_td__blk952_dn7 = assign29240_e41316_d_n7;
        locals.var_td__blk952_dn10 = assign29240_e41316_d_n10;
        locals.var_td__blk952_dn11 = assign29240_e41316_d_n11;
        locals.var_td__blk952_dn12 = assign29240_e41316_d_n12;
        locals.var_td__blk952_dn17 = assign29240_e41316_d_n17;
        locals.var_td__blk952_rv = 0.0;

        let (assign29250_e41358, assign29250_e41358_d_n0, assign29250_e41358_d_n2, assign29250_e41358_d_n6, assign29250_e41358_d_n7, assign29250_e41358_d_n10, assign29250_e41358_d_n11, assign29250_e41358_d_n12, assign29250_e41358_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard988 != 0.0)) {
        let assign29250_e41330: f64 = (locals.var_tb__blk950 * locals.var_tb__blk950);
        let assign29250_e41332: f64 = (assign29250_e41330 * locals.var_tb__blk950);
        let assign29250_e41335: f64 = (27.0 * locals.var_ta__blk949);
        let assign29250_e41337: f64 = (assign29250_e41335 * locals.var_ta__blk949);
        let assign29250_e41339: f64 = (assign29250_e41337 * locals.var_ta__blk949);
        let assign29250_e41340: f64 = (assign29250_e41332 / assign29250_e41339);
        let assign29250_e41343: f64 = (locals.var_tb__blk950 * locals.var_tc__blk951);
        let assign29250_e41346: f64 = (6.0 * locals.var_ta__blk949);
        let assign29250_e41348: f64 = (assign29250_e41346 * locals.var_ta__blk949);
        let assign29250_e41349: f64 = (assign29250_e41343 / assign29250_e41348);
        let assign29250_e41350: f64 = (assign29250_e41340 - assign29250_e41349);
        let assign29250_e41354: f64 = (2.0 * locals.var_ta__blk949);
        let assign29250_e41355: f64 = (locals.var_td__blk952 / assign29250_e41354);
        let assign29250_e41356: f64 = (assign29250_e41350 + assign29250_e41355);
        (assign29250_e41356, ((-((locals.var_tb__blk950 * locals.var_tc__blk951_dn0) / assign29250_e41348)) + (locals.var_td__blk952_dn0 / assign29250_e41354)), ((-((locals.var_tb__blk950 * locals.var_tc__blk951_dn2) / assign29250_e41348)) + (locals.var_td__blk952_dn2 / assign29250_e41354)), ((-((locals.var_tb__blk950 * locals.var_tc__blk951_dn6) / assign29250_e41348)) + (locals.var_td__blk952_dn6 / assign29250_e41354)), ((-((locals.var_tb__blk950 * locals.var_tc__blk951_dn7) / assign29250_e41348)) + (locals.var_td__blk952_dn7 / assign29250_e41354)), ((-((locals.var_tb__blk950 * locals.var_tc__blk951_dn10) / assign29250_e41348)) + (locals.var_td__blk952_dn10 / assign29250_e41354)), ((-((locals.var_tb__blk950 * locals.var_tc__blk951_dn11) / assign29250_e41348)) + (locals.var_td__blk952_dn11 / assign29250_e41354)), ((-((locals.var_tb__blk950 * locals.var_tc__blk951_dn12) / assign29250_e41348)) + (locals.var_td__blk952_dn12 / assign29250_e41354)), ((-((locals.var_tb__blk950 * locals.var_tc__blk951_dn17) / assign29250_e41348)) + (locals.var_td__blk952_dn17 / assign29250_e41354)),)
    } else {
        (locals.var_tq__blk953, locals.var_tq__blk953_dn0, locals.var_tq__blk953_dn2, locals.var_tq__blk953_dn6, locals.var_tq__blk953_dn7, locals.var_tq__blk953_dn10, locals.var_tq__blk953_dn11, locals.var_tq__blk953_dn12, locals.var_tq__blk953_dn17,)
    }
};
        locals.var_tq__blk953 = assign29250_e41358;
        locals.var_tq__blk953_dn0 = assign29250_e41358_d_n0;
        locals.var_tq__blk953_dn2 = assign29250_e41358_d_n2;
        locals.var_tq__blk953_dn6 = assign29250_e41358_d_n6;
        locals.var_tq__blk953_dn7 = assign29250_e41358_d_n7;
        locals.var_tq__blk953_dn10 = assign29250_e41358_d_n10;
        locals.var_tq__blk953_dn11 = assign29250_e41358_d_n11;
        locals.var_tq__blk953_dn12 = assign29250_e41358_d_n12;
        locals.var_tq__blk953_dn17 = assign29250_e41358_d_n17;
        locals.var_tq__blk953_rv = 0.0;

        let (assign29260_e41386, assign29260_e41386_d_n0, assign29260_e41386_d_n2, assign29260_e41386_d_n6, assign29260_e41386_d_n7, assign29260_e41386_d_n10, assign29260_e41386_d_n11, assign29260_e41386_d_n12, assign29260_e41386_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard988 != 0.0)) {
        let assign29260_e41372: f64 = (3.0 * locals.var_ta__blk949);
        let assign29260_e41374: f64 = (assign29260_e41372 * locals.var_tc__blk951);
        let assign29260_e41377: f64 = (locals.var_tb__blk950 * locals.var_tb__blk950);
        let assign29260_e41378: f64 = (assign29260_e41374 - assign29260_e41377);
        let assign29260_e41381: f64 = (9.0 * locals.var_ta__blk949);
        let assign29260_e41383: f64 = (assign29260_e41381 * locals.var_ta__blk949);
        let assign29260_e41384: f64 = (assign29260_e41378 / assign29260_e41383);
        (assign29260_e41384, ((assign29260_e41372 * locals.var_tc__blk951_dn0) / assign29260_e41383), ((assign29260_e41372 * locals.var_tc__blk951_dn2) / assign29260_e41383), ((assign29260_e41372 * locals.var_tc__blk951_dn6) / assign29260_e41383), ((assign29260_e41372 * locals.var_tc__blk951_dn7) / assign29260_e41383), ((assign29260_e41372 * locals.var_tc__blk951_dn10) / assign29260_e41383), ((assign29260_e41372 * locals.var_tc__blk951_dn11) / assign29260_e41383), ((assign29260_e41372 * locals.var_tc__blk951_dn12) / assign29260_e41383), ((assign29260_e41372 * locals.var_tc__blk951_dn17) / assign29260_e41383),)
    } else {
        (locals.var_tp__blk954, locals.var_tp__blk954_dn0, locals.var_tp__blk954_dn2, locals.var_tp__blk954_dn6, locals.var_tp__blk954_dn7, locals.var_tp__blk954_dn10, locals.var_tp__blk954_dn11, locals.var_tp__blk954_dn12, locals.var_tp__blk954_dn17,)
    }
};
        locals.var_tp__blk954 = assign29260_e41386;
        locals.var_tp__blk954_dn0 = assign29260_e41386_d_n0;
        locals.var_tp__blk954_dn2 = assign29260_e41386_d_n2;
        locals.var_tp__blk954_dn6 = assign29260_e41386_d_n6;
        locals.var_tp__blk954_dn7 = assign29260_e41386_d_n7;
        locals.var_tp__blk954_dn10 = assign29260_e41386_d_n10;
        locals.var_tp__blk954_dn11 = assign29260_e41386_d_n11;
        locals.var_tp__blk954_dn12 = assign29260_e41386_d_n12;
        locals.var_tp__blk954_dn17 = assign29260_e41386_d_n17;
        locals.var_tp__blk954_rv = 0.0;

        let (assign29270_e41409, assign29270_e41409_d_n0, assign29270_e41409_d_n2, assign29270_e41409_d_n6, assign29270_e41409_d_n7, assign29270_e41409_d_n10, assign29270_e41409_d_n11, assign29270_e41409_d_n12, assign29270_e41409_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard988 != 0.0)) {
        let assign29270_e41400: f64 = (locals.var_tq__blk953 * locals.var_tq__blk953);
        let assign29270_e41403: f64 = (locals.var_tp__blk954 * locals.var_tp__blk954);
        let assign29270_e41405: f64 = (assign29270_e41403 * locals.var_tp__blk954);
        let assign29270_e41406: f64 = (assign29270_e41400 + assign29270_e41405);
        let assign29270_e41407: f64 = (assign29270_e41406).sqrt();
        (assign29270_e41407, ((((locals.var_tq__blk953_dn0 * locals.var_tq__blk953) + (locals.var_tq__blk953 * locals.var_tq__blk953_dn0)) + ((((locals.var_tp__blk954_dn0 * locals.var_tp__blk954) + (locals.var_tp__blk954 * locals.var_tp__blk954_dn0)) * locals.var_tp__blk954) + (assign29270_e41403 * locals.var_tp__blk954_dn0))) / (2.0 * assign29270_e41407)), ((((locals.var_tq__blk953_dn2 * locals.var_tq__blk953) + (locals.var_tq__blk953 * locals.var_tq__blk953_dn2)) + ((((locals.var_tp__blk954_dn2 * locals.var_tp__blk954) + (locals.var_tp__blk954 * locals.var_tp__blk954_dn2)) * locals.var_tp__blk954) + (assign29270_e41403 * locals.var_tp__blk954_dn2))) / (2.0 * assign29270_e41407)), ((((locals.var_tq__blk953_dn6 * locals.var_tq__blk953) + (locals.var_tq__blk953 * locals.var_tq__blk953_dn6)) + ((((locals.var_tp__blk954_dn6 * locals.var_tp__blk954) + (locals.var_tp__blk954 * locals.var_tp__blk954_dn6)) * locals.var_tp__blk954) + (assign29270_e41403 * locals.var_tp__blk954_dn6))) / (2.0 * assign29270_e41407)), ((((locals.var_tq__blk953_dn7 * locals.var_tq__blk953) + (locals.var_tq__blk953 * locals.var_tq__blk953_dn7)) + ((((locals.var_tp__blk954_dn7 * locals.var_tp__blk954) + (locals.var_tp__blk954 * locals.var_tp__blk954_dn7)) * locals.var_tp__blk954) + (assign29270_e41403 * locals.var_tp__blk954_dn7))) / (2.0 * assign29270_e41407)), ((((locals.var_tq__blk953_dn10 * locals.var_tq__blk953) + (locals.var_tq__blk953 * locals.var_tq__blk953_dn10)) + ((((locals.var_tp__blk954_dn10 * locals.var_tp__blk954) + (locals.var_tp__blk954 * locals.var_tp__blk954_dn10)) * locals.var_tp__blk954) + (assign29270_e41403 * locals.var_tp__blk954_dn10))) / (2.0 * assign29270_e41407)), ((((locals.var_tq__blk953_dn11 * locals.var_tq__blk953) + (locals.var_tq__blk953 * locals.var_tq__blk953_dn11)) + ((((locals.var_tp__blk954_dn11 * locals.var_tp__blk954) + (locals.var_tp__blk954 * locals.var_tp__blk954_dn11)) * locals.var_tp__blk954) + (assign29270_e41403 * locals.var_tp__blk954_dn11))) / (2.0 * assign29270_e41407)), ((((locals.var_tq__blk953_dn12 * locals.var_tq__blk953) + (locals.var_tq__blk953 * locals.var_tq__blk953_dn12)) + ((((locals.var_tp__blk954_dn12 * locals.var_tp__blk954) + (locals.var_tp__blk954 * locals.var_tp__blk954_dn12)) * locals.var_tp__blk954) + (assign29270_e41403 * locals.var_tp__blk954_dn12))) / (2.0 * assign29270_e41407)), ((((locals.var_tq__blk953_dn17 * locals.var_tq__blk953) + (locals.var_tq__blk953 * locals.var_tq__blk953_dn17)) + ((((locals.var_tp__blk954_dn17 * locals.var_tp__blk954) + (locals.var_tp__blk954 * locals.var_tp__blk954_dn17)) * locals.var_tp__blk954) + (assign29270_e41403 * locals.var_tp__blk954_dn17))) / (2.0 * assign29270_e41407)),)
    } else {
        (locals.var_t5__blk902, locals.var_t5__blk902_dn0, locals.var_t5__blk902_dn2, locals.var_t5__blk902_dn6, locals.var_t5__blk902_dn7, locals.var_t5__blk902_dn10, locals.var_t5__blk902_dn11, locals.var_t5__blk902_dn12, locals.var_t5__blk902_dn17,)
    }
};
        locals.var_t5__blk902 = assign29270_e41409;
        locals.var_t5__blk902_dn0 = assign29270_e41409_d_n0;
        locals.var_t5__blk902_dn2 = assign29270_e41409_d_n2;
        locals.var_t5__blk902_dn6 = assign29270_e41409_d_n6;
        locals.var_t5__blk902_dn7 = assign29270_e41409_d_n7;
        locals.var_t5__blk902_dn10 = assign29270_e41409_d_n10;
        locals.var_t5__blk902_dn11 = assign29270_e41409_d_n11;
        locals.var_t5__blk902_dn12 = assign29270_e41409_d_n12;
        locals.var_t5__blk902_dn17 = assign29270_e41409_d_n17;
        locals.var_t5__blk902_rv = 0.0;

        let (assign29280_e41428, assign29280_e41428_d_n0, assign29280_e41428_d_n2, assign29280_e41428_d_n6, assign29280_e41428_d_n7, assign29280_e41428_d_n10, assign29280_e41428_d_n11, assign29280_e41428_d_n12, assign29280_e41428_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard988 != 0.0)) {
        let assign29280_e41422: f64 = (-locals.var_tq__blk953);
        let assign29280_e41424: f64 = (assign29280_e41422 + locals.var_t5__blk902);
        let assign29280_e41426: f64 = (assign29280_e41424).powf(0.3333333333333333);
        (assign29280_e41426, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29280_e41424).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk953_dn0) + locals.var_t5__blk902_dn0))) } } else { (assign29280_e41426 * (0.3333333333333333 * (((-locals.var_tq__blk953_dn0) + locals.var_t5__blk902_dn0) / assign29280_e41424))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29280_e41424).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk953_dn2) + locals.var_t5__blk902_dn2))) } } else { (assign29280_e41426 * (0.3333333333333333 * (((-locals.var_tq__blk953_dn2) + locals.var_t5__blk902_dn2) / assign29280_e41424))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29280_e41424).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk953_dn6) + locals.var_t5__blk902_dn6))) } } else { (assign29280_e41426 * (0.3333333333333333 * (((-locals.var_tq__blk953_dn6) + locals.var_t5__blk902_dn6) / assign29280_e41424))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29280_e41424).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk953_dn7) + locals.var_t5__blk902_dn7))) } } else { (assign29280_e41426 * (0.3333333333333333 * (((-locals.var_tq__blk953_dn7) + locals.var_t5__blk902_dn7) / assign29280_e41424))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29280_e41424).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk953_dn10) + locals.var_t5__blk902_dn10))) } } else { (assign29280_e41426 * (0.3333333333333333 * (((-locals.var_tq__blk953_dn10) + locals.var_t5__blk902_dn10) / assign29280_e41424))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29280_e41424).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk953_dn11) + locals.var_t5__blk902_dn11))) } } else { (assign29280_e41426 * (0.3333333333333333 * (((-locals.var_tq__blk953_dn11) + locals.var_t5__blk902_dn11) / assign29280_e41424))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29280_e41424).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk953_dn12) + locals.var_t5__blk902_dn12))) } } else { (assign29280_e41426 * (0.3333333333333333 * (((-locals.var_tq__blk953_dn12) + locals.var_t5__blk902_dn12) / assign29280_e41424))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29280_e41424).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk953_dn17) + locals.var_t5__blk902_dn17))) } } else { (assign29280_e41426 * (0.3333333333333333 * (((-locals.var_tq__blk953_dn17) + locals.var_t5__blk902_dn17) / assign29280_e41424))) },)
    } else {
        (locals.var_tu__blk955, locals.var_tu__blk955_dn0, locals.var_tu__blk955_dn2, locals.var_tu__blk955_dn6, locals.var_tu__blk955_dn7, locals.var_tu__blk955_dn10, locals.var_tu__blk955_dn11, locals.var_tu__blk955_dn12, locals.var_tu__blk955_dn17,)
    }
};
        locals.var_tu__blk955 = assign29280_e41428;
        locals.var_tu__blk955_dn0 = assign29280_e41428_d_n0;
        locals.var_tu__blk955_dn2 = assign29280_e41428_d_n2;
        locals.var_tu__blk955_dn6 = assign29280_e41428_d_n6;
        locals.var_tu__blk955_dn7 = assign29280_e41428_d_n7;
        locals.var_tu__blk955_dn10 = assign29280_e41428_d_n10;
        locals.var_tu__blk955_dn11 = assign29280_e41428_d_n11;
        locals.var_tu__blk955_dn12 = assign29280_e41428_d_n12;
        locals.var_tu__blk955_dn17 = assign29280_e41428_d_n17;
        locals.var_tu__blk955_rv = 0.0;

        let (assign29290_e41447, assign29290_e41447_d_n0, assign29290_e41447_d_n2, assign29290_e41447_d_n6, assign29290_e41447_d_n7, assign29290_e41447_d_n10, assign29290_e41447_d_n11, assign29290_e41447_d_n12, assign29290_e41447_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard988 != 0.0)) {
        let assign29290_e41442: f64 = (locals.var_tq__blk953 + locals.var_t5__blk902);
        let assign29290_e41444: f64 = (assign29290_e41442).powf(0.3333333333333333);
        let assign29290_e41445: f64 = (-assign29290_e41444);
        (assign29290_e41445, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29290_e41442).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk953_dn0 + locals.var_t5__blk902_dn0))) } } else { (assign29290_e41444 * (0.3333333333333333 * ((locals.var_tq__blk953_dn0 + locals.var_t5__blk902_dn0) / assign29290_e41442))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29290_e41442).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk953_dn2 + locals.var_t5__blk902_dn2))) } } else { (assign29290_e41444 * (0.3333333333333333 * ((locals.var_tq__blk953_dn2 + locals.var_t5__blk902_dn2) / assign29290_e41442))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29290_e41442).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk953_dn6 + locals.var_t5__blk902_dn6))) } } else { (assign29290_e41444 * (0.3333333333333333 * ((locals.var_tq__blk953_dn6 + locals.var_t5__blk902_dn6) / assign29290_e41442))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29290_e41442).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk953_dn7 + locals.var_t5__blk902_dn7))) } } else { (assign29290_e41444 * (0.3333333333333333 * ((locals.var_tq__blk953_dn7 + locals.var_t5__blk902_dn7) / assign29290_e41442))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29290_e41442).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk953_dn10 + locals.var_t5__blk902_dn10))) } } else { (assign29290_e41444 * (0.3333333333333333 * ((locals.var_tq__blk953_dn10 + locals.var_t5__blk902_dn10) / assign29290_e41442))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29290_e41442).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk953_dn11 + locals.var_t5__blk902_dn11))) } } else { (assign29290_e41444 * (0.3333333333333333 * ((locals.var_tq__blk953_dn11 + locals.var_t5__blk902_dn11) / assign29290_e41442))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29290_e41442).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk953_dn12 + locals.var_t5__blk902_dn12))) } } else { (assign29290_e41444 * (0.3333333333333333 * ((locals.var_tq__blk953_dn12 + locals.var_t5__blk902_dn12) / assign29290_e41442))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29290_e41442).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk953_dn17 + locals.var_t5__blk902_dn17))) } } else { (assign29290_e41444 * (0.3333333333333333 * ((locals.var_tq__blk953_dn17 + locals.var_t5__blk902_dn17) / assign29290_e41442))) }),)
    } else {
        (locals.var_tv__blk956, locals.var_tv__blk956_dn0, locals.var_tv__blk956_dn2, locals.var_tv__blk956_dn6, locals.var_tv__blk956_dn7, locals.var_tv__blk956_dn10, locals.var_tv__blk956_dn11, locals.var_tv__blk956_dn12, locals.var_tv__blk956_dn17,)
    }
};
        locals.var_tv__blk956 = assign29290_e41447;
        locals.var_tv__blk956_dn0 = assign29290_e41447_d_n0;
        locals.var_tv__blk956_dn2 = assign29290_e41447_d_n2;
        locals.var_tv__blk956_dn6 = assign29290_e41447_d_n6;
        locals.var_tv__blk956_dn7 = assign29290_e41447_d_n7;
        locals.var_tv__blk956_dn10 = assign29290_e41447_d_n10;
        locals.var_tv__blk956_dn11 = assign29290_e41447_d_n11;
        locals.var_tv__blk956_dn12 = assign29290_e41447_d_n12;
        locals.var_tv__blk956_dn17 = assign29290_e41447_d_n17;
        locals.var_tv__blk956_rv = 0.0;

        let (assign29300_e41469, assign29300_e41469_d_n0, assign29300_e41469_d_n2, assign29300_e41469_d_n6, assign29300_e41469_d_n7, assign29300_e41469_d_n10, assign29300_e41469_d_n11, assign29300_e41469_d_n12, assign29300_e41469_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard988 != 0.0)) {
        let assign29300_e41461: f64 = (locals.var_tu__blk955 + locals.var_tv__blk956);
        let assign29300_e41465: f64 = (3.0 * locals.var_ta__blk949);
        let assign29300_e41466: f64 = (locals.var_tb__blk950 / assign29300_e41465);
        let assign29300_e41467: f64 = (assign29300_e41461 - assign29300_e41466);
        (assign29300_e41467, (locals.var_tu__blk955_dn0 + locals.var_tv__blk956_dn0), (locals.var_tu__blk955_dn2 + locals.var_tv__blk956_dn2), (locals.var_tu__blk955_dn6 + locals.var_tv__blk956_dn6), (locals.var_tu__blk955_dn7 + locals.var_tv__blk956_dn7), (locals.var_tu__blk955_dn10 + locals.var_tv__blk956_dn10), (locals.var_tu__blk955_dn11 + locals.var_tv__blk956_dn11), (locals.var_tu__blk955_dn12 + locals.var_tv__blk956_dn12), (locals.var_tu__blk955_dn17 + locals.var_tv__blk956_dn17),)
    } else {
        (locals.var_tx__blk906, locals.var_tx__blk906_dn0, locals.var_tx__blk906_dn2, locals.var_tx__blk906_dn6, locals.var_tx__blk906_dn7, locals.var_tx__blk906_dn10, locals.var_tx__blk906_dn11, locals.var_tx__blk906_dn12, locals.var_tx__blk906_dn17,)
    }
};
        locals.var_tx__blk906 = assign29300_e41469;
        locals.var_tx__blk906_dn0 = assign29300_e41469_d_n0;
        locals.var_tx__blk906_dn2 = assign29300_e41469_d_n2;
        locals.var_tx__blk906_dn6 = assign29300_e41469_d_n6;
        locals.var_tx__blk906_dn7 = assign29300_e41469_d_n7;
        locals.var_tx__blk906_dn10 = assign29300_e41469_d_n10;
        locals.var_tx__blk906_dn11 = assign29300_e41469_d_n11;
        locals.var_tx__blk906_dn12 = assign29300_e41469_d_n12;
        locals.var_tx__blk906_dn17 = assign29300_e41469_d_n17;
        locals.var_tx__blk906_rv = 0.0;

        let (assign29310_e41487, assign29310_e41487_d_n0, assign29310_e41487_d_n2, assign29310_e41487_d_n6, assign29310_e41487_d_n7, assign29310_e41487_d_n10, assign29310_e41487_d_n11, assign29310_e41487_d_n12, assign29310_e41487_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard988 != 0.0)) {
        let assign29310_e41483: f64 = (locals.var_tx__blk906 * locals.var_beta_inv);
        let assign29310_e41485: f64 = (assign29310_e41483 - locals.var_vxbgmtcl__blk923);
        (assign29310_e41485, ((locals.var_tx__blk906_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn0), ((locals.var_tx__blk906_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn2), ((locals.var_tx__blk906_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn6), ((locals.var_tx__blk906_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn7), (((locals.var_tx__blk906_dn10 * locals.var_beta_inv) + (locals.var_tx__blk906 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl__blk923_dn10), ((locals.var_tx__blk906_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn11), ((locals.var_tx__blk906_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn12), ((locals.var_tx__blk906_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk923_dn17),)
    } else {
        (locals.var_ps0_inia__blk948, locals.var_ps0_inia__blk948_dn0, locals.var_ps0_inia__blk948_dn2, locals.var_ps0_inia__blk948_dn6, locals.var_ps0_inia__blk948_dn7, locals.var_ps0_inia__blk948_dn10, locals.var_ps0_inia__blk948_dn11, locals.var_ps0_inia__blk948_dn12, locals.var_ps0_inia__blk948_dn17,)
    }
};
        locals.var_ps0_inia__blk948 = assign29310_e41487;
        locals.var_ps0_inia__blk948_dn0 = assign29310_e41487_d_n0;
        locals.var_ps0_inia__blk948_dn2 = assign29310_e41487_d_n2;
        locals.var_ps0_inia__blk948_dn6 = assign29310_e41487_d_n6;
        locals.var_ps0_inia__blk948_dn7 = assign29310_e41487_d_n7;
        locals.var_ps0_inia__blk948_dn10 = assign29310_e41487_d_n10;
        locals.var_ps0_inia__blk948_dn11 = assign29310_e41487_d_n11;
        locals.var_ps0_inia__blk948_dn12 = assign29310_e41487_d_n12;
        locals.var_ps0_inia__blk948_dn17 = assign29310_e41487_d_n17;
        locals.var_ps0_inia__blk948_rv = 0.0;

        let (assign29320_e41505, assign29320_e41505_d_n0, assign29320_e41505_d_n2, assign29320_e41505_d_n6, assign29320_e41505_d_n7, assign29320_e41505_d_n10, assign29320_e41505_d_n11, assign29320_e41505_d_n12, assign29320_e41505_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard988 != 0.0)) {
        let assign29320_e41502: f64 = (locals.var_ps0_inia__blk948 + locals.var_vxbgmtcl__blk923);
        let assign29320_e41503: f64 = (locals.var_beta * assign29320_e41502);
        (assign29320_e41503, (locals.var_beta * (locals.var_ps0_inia__blk948_dn0 + locals.var_vxbgmtcl__blk923_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn2 + locals.var_vxbgmtcl__blk923_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn6 + locals.var_vxbgmtcl__blk923_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn7 + locals.var_vxbgmtcl__blk923_dn7)), ((locals.var_beta_dn10 * assign29320_e41502) + (locals.var_beta * (locals.var_ps0_inia__blk948_dn10 + locals.var_vxbgmtcl__blk923_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk948_dn11 + locals.var_vxbgmtcl__blk923_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn12 + locals.var_vxbgmtcl__blk923_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk948_dn17 + locals.var_vxbgmtcl__blk923_dn17)),)
    } else {
        (locals.var_chi__blk945, locals.var_chi__blk945_dn0, locals.var_chi__blk945_dn2, locals.var_chi__blk945_dn6, locals.var_chi__blk945_dn7, locals.var_chi__blk945_dn10, locals.var_chi__blk945_dn11, locals.var_chi__blk945_dn12, locals.var_chi__blk945_dn17,)
    }
};
        locals.var_chi__blk945 = assign29320_e41505;
        locals.var_chi__blk945_dn0 = assign29320_e41505_d_n0;
        locals.var_chi__blk945_dn2 = assign29320_e41505_d_n2;
        locals.var_chi__blk945_dn6 = assign29320_e41505_d_n6;
        locals.var_chi__blk945_dn7 = assign29320_e41505_d_n7;
        locals.var_chi__blk945_dn10 = assign29320_e41505_d_n10;
        locals.var_chi__blk945_dn11 = assign29320_e41505_d_n11;
        locals.var_chi__blk945_dn12 = assign29320_e41505_d_n12;
        locals.var_chi__blk945_dn17 = assign29320_e41505_d_n17;
        locals.var_chi__blk945_rv = 0.0;

        let assign29330_e41508: f64 = if p.p41 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard989 = assign29330_e41508;
        locals.var_guard989_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_106(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29350_e41542, assign29350_e41542_d_n0, assign29350_e41542_d_n2, assign29350_e41542_d_n6, assign29350_e41542_d_n7, assign29350_e41542_d_n10, assign29350_e41542_d_n11, assign29350_e41542_d_n12, assign29350_e41542_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29350_e41538: f64 = (locals.var_vgpld__blk933 + locals.var_vxbgmtcl__blk923);
        let assign29350_e41540: f64 = (assign29350_e41538 + 0.1);
        (assign29350_e41540, (locals.var_vgpld__blk933_dn0 + locals.var_vxbgmtcl__blk923_dn0), (locals.var_vgpld__blk933_dn2 + locals.var_vxbgmtcl__blk923_dn2), (locals.var_vgpld__blk933_dn6 + locals.var_vxbgmtcl__blk923_dn6), (locals.var_vgpld__blk933_dn7 + locals.var_vxbgmtcl__blk923_dn7), (locals.var_vgpld__blk933_dn10 + locals.var_vxbgmtcl__blk923_dn10), (locals.var_vgpld__blk933_dn11 + locals.var_vxbgmtcl__blk923_dn11), (locals.var_vgpld__blk933_dn12 + locals.var_vxbgmtcl__blk923_dn12), (locals.var_vgpld__blk933_dn17 + locals.var_vxbgmtcl__blk923_dn17),)
    } else {
        (locals.var_vgpld_shift__blk957, locals.var_vgpld_shift__blk957_dn0, locals.var_vgpld_shift__blk957_dn2, locals.var_vgpld_shift__blk957_dn6, locals.var_vgpld_shift__blk957_dn7, locals.var_vgpld_shift__blk957_dn10, locals.var_vgpld_shift__blk957_dn11, locals.var_vgpld_shift__blk957_dn12, locals.var_vgpld_shift__blk957_dn17,)
    }
};
        locals.var_vgpld_shift__blk957 = assign29350_e41542;
        locals.var_vgpld_shift__blk957_dn0 = assign29350_e41542_d_n0;
        locals.var_vgpld_shift__blk957_dn2 = assign29350_e41542_d_n2;
        locals.var_vgpld_shift__blk957_dn6 = assign29350_e41542_d_n6;
        locals.var_vgpld_shift__blk957_dn7 = assign29350_e41542_d_n7;
        locals.var_vgpld_shift__blk957_dn10 = assign29350_e41542_d_n10;
        locals.var_vgpld_shift__blk957_dn11 = assign29350_e41542_d_n11;
        locals.var_vgpld_shift__blk957_dn12 = assign29350_e41542_d_n12;
        locals.var_vgpld_shift__blk957_dn17 = assign29350_e41542_d_n17;
        locals.var_vgpld_shift__blk957_rv = 0.0;

        let (assign29360_e41562, assign29360_e41562_d_n0, assign29360_e41562_d_n2, assign29360_e41562_d_n6, assign29360_e41562_d_n7, assign29360_e41562_d_n10, assign29360_e41562_d_n11, assign29360_e41562_d_n12, assign29360_e41562_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29360_e41556: f64 = (-locals.var_vxbgmtcl__blk923);
        let assign29360_e41557: f64 = (locals.var_beta * assign29360_e41556);
        let assign29360_e41558: f64 = (assign29360_e41557).exp();
        let assign29360_e41560: f64 = (assign29360_e41558 + 1e-50);
        (assign29360_e41560, (assign29360_e41558 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn0))), (assign29360_e41558 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn2))), (assign29360_e41558 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn6))), (assign29360_e41558 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn7))), (assign29360_e41558 * ((locals.var_beta_dn10 * assign29360_e41556) + (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn10)))), (assign29360_e41558 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn11))), (assign29360_e41558 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn12))), (assign29360_e41558 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk964, locals.var_exp_bvbs__blk964_dn0, locals.var_exp_bvbs__blk964_dn2, locals.var_exp_bvbs__blk964_dn6, locals.var_exp_bvbs__blk964_dn7, locals.var_exp_bvbs__blk964_dn10, locals.var_exp_bvbs__blk964_dn11, locals.var_exp_bvbs__blk964_dn12, locals.var_exp_bvbs__blk964_dn17,)
    }
};
        locals.var_exp_bvbs__blk964 = assign29360_e41562;
        locals.var_exp_bvbs__blk964_dn0 = assign29360_e41562_d_n0;
        locals.var_exp_bvbs__blk964_dn2 = assign29360_e41562_d_n2;
        locals.var_exp_bvbs__blk964_dn6 = assign29360_e41562_d_n6;
        locals.var_exp_bvbs__blk964_dn7 = assign29360_e41562_d_n7;
        locals.var_exp_bvbs__blk964_dn10 = assign29360_e41562_d_n10;
        locals.var_exp_bvbs__blk964_dn11 = assign29360_e41562_d_n11;
        locals.var_exp_bvbs__blk964_dn12 = assign29360_e41562_d_n12;
        locals.var_exp_bvbs__blk964_dn17 = assign29360_e41562_d_n17;
        locals.var_exp_bvbs__blk964_rv = 0.0;

        let (assign29370_e41578, assign29370_e41578_d_n0, assign29370_e41578_d_n2, assign29370_e41578_d_n6, assign29370_e41578_d_n7, assign29370_e41578_d_n10, assign29370_e41578_d_n11, assign29370_e41578_d_n12, assign29370_e41578_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29370_e41576: f64 = (locals.var_nin / locals.var_mks_nover);
        (assign29370_e41576, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn7 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover), (locals.var_nin_dn17 / locals.var_mks_nover),)
    } else {
        (locals.var_t0__blk897, locals.var_t0__blk897_dn0, locals.var_t0__blk897_dn2, locals.var_t0__blk897_dn6, locals.var_t0__blk897_dn7, locals.var_t0__blk897_dn10, locals.var_t0__blk897_dn11, locals.var_t0__blk897_dn12, locals.var_t0__blk897_dn17,)
    }
};
        locals.var_t0__blk897 = assign29370_e41578;
        locals.var_t0__blk897_dn0 = assign29370_e41578_d_n0;
        locals.var_t0__blk897_dn2 = assign29370_e41578_d_n2;
        locals.var_t0__blk897_dn6 = assign29370_e41578_d_n6;
        locals.var_t0__blk897_dn7 = assign29370_e41578_d_n7;
        locals.var_t0__blk897_dn10 = assign29370_e41578_d_n10;
        locals.var_t0__blk897_dn11 = assign29370_e41578_d_n11;
        locals.var_t0__blk897_dn12 = assign29370_e41578_d_n12;
        locals.var_t0__blk897_dn17 = assign29370_e41578_d_n17;
        locals.var_t0__blk897_rv = 0.0;

        let (assign29380_e41594, assign29380_e41594_d_n0, assign29380_e41594_d_n2, assign29380_e41594_d_n6, assign29380_e41594_d_n7, assign29380_e41594_d_n10, assign29380_e41594_d_n11, assign29380_e41594_d_n12, assign29380_e41594_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29380_e41592: f64 = (locals.var_t0__blk897 * locals.var_t0__blk897);
        (assign29380_e41592, ((locals.var_t0__blk897_dn0 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn0)), ((locals.var_t0__blk897_dn2 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn2)), ((locals.var_t0__blk897_dn6 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn6)), ((locals.var_t0__blk897_dn7 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn7)), ((locals.var_t0__blk897_dn10 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn10)), ((locals.var_t0__blk897_dn11 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn11)), ((locals.var_t0__blk897_dn12 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn12)), ((locals.var_t0__blk897_dn17 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn17)),)
    } else {
        (locals.var_cnst1over__blk958, locals.var_cnst1over__blk958_dn0, locals.var_cnst1over__blk958_dn2, locals.var_cnst1over__blk958_dn6, locals.var_cnst1over__blk958_dn7, locals.var_cnst1over__blk958_dn10, locals.var_cnst1over__blk958_dn11, locals.var_cnst1over__blk958_dn12, locals.var_cnst1over__blk958_dn17,)
    }
};
        locals.var_cnst1over__blk958 = assign29380_e41594;
        locals.var_cnst1over__blk958_dn0 = assign29380_e41594_d_n0;
        locals.var_cnst1over__blk958_dn2 = assign29380_e41594_d_n2;
        locals.var_cnst1over__blk958_dn6 = assign29380_e41594_d_n6;
        locals.var_cnst1over__blk958_dn7 = assign29380_e41594_d_n7;
        locals.var_cnst1over__blk958_dn10 = assign29380_e41594_d_n10;
        locals.var_cnst1over__blk958_dn11 = assign29380_e41594_d_n11;
        locals.var_cnst1over__blk958_dn12 = assign29380_e41594_d_n12;
        locals.var_cnst1over__blk958_dn17 = assign29380_e41594_d_n17;
        locals.var_cnst1over__blk958_rv = 0.0;

        let (assign29390_e41610, assign29390_e41610_d_n0, assign29390_e41610_d_n2, assign29390_e41610_d_n6, assign29390_e41610_d_n7, assign29390_e41610_d_n10, assign29390_e41610_d_n11, assign29390_e41610_d_n12, assign29390_e41610_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29390_e41608: f64 = (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964);
        (assign29390_e41608, ((locals.var_cnst1over__blk958_dn0 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn0)), ((locals.var_cnst1over__blk958_dn2 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn2)), ((locals.var_cnst1over__blk958_dn6 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn6)), ((locals.var_cnst1over__blk958_dn7 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn7)), ((locals.var_cnst1over__blk958_dn10 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn10)), ((locals.var_cnst1over__blk958_dn11 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn11)), ((locals.var_cnst1over__blk958_dn12 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn12)), ((locals.var_cnst1over__blk958_dn17 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn17)),)
    } else {
        (locals.var_gammachi__blk959, locals.var_gammachi__blk959_dn0, locals.var_gammachi__blk959_dn2, locals.var_gammachi__blk959_dn6, locals.var_gammachi__blk959_dn7, locals.var_gammachi__blk959_dn10, locals.var_gammachi__blk959_dn11, locals.var_gammachi__blk959_dn12, locals.var_gammachi__blk959_dn17,)
    }
};
        locals.var_gammachi__blk959 = assign29390_e41610;
        locals.var_gammachi__blk959_dn0 = assign29390_e41610_d_n0;
        locals.var_gammachi__blk959_dn2 = assign29390_e41610_d_n2;
        locals.var_gammachi__blk959_dn6 = assign29390_e41610_d_n6;
        locals.var_gammachi__blk959_dn7 = assign29390_e41610_d_n7;
        locals.var_gammachi__blk959_dn10 = assign29390_e41610_d_n10;
        locals.var_gammachi__blk959_dn11 = assign29390_e41610_d_n11;
        locals.var_gammachi__blk959_dn12 = assign29390_e41610_d_n12;
        locals.var_gammachi__blk959_dn17 = assign29390_e41610_d_n17;
        locals.var_gammachi__blk959_rv = 0.0;

        let (assign29400_e41626, assign29400_e41626_d_n0, assign29400_e41626_d_n2, assign29400_e41626_d_n6, assign29400_e41626_d_n7, assign29400_e41626_d_n10, assign29400_e41626_d_n11, assign29400_e41626_d_n12, assign29400_e41626_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29400_e41624: f64 = (locals.var_beta2 * locals.var_fac1p2__blk932);
        (assign29400_e41624, (locals.var_beta2 * locals.var_fac1p2__blk932_dn0), (locals.var_beta2 * locals.var_fac1p2__blk932_dn2), (locals.var_beta2 * locals.var_fac1p2__blk932_dn6), (locals.var_beta2 * locals.var_fac1p2__blk932_dn7), ((locals.var_beta2_dn10 * locals.var_fac1p2__blk932) + (locals.var_beta2 * locals.var_fac1p2__blk932_dn10)), (locals.var_beta2 * locals.var_fac1p2__blk932_dn11), (locals.var_beta2 * locals.var_fac1p2__blk932_dn12), (locals.var_beta2 * locals.var_fac1p2__blk932_dn17),)
    } else {
        (locals.var_t0__blk897, locals.var_t0__blk897_dn0, locals.var_t0__blk897_dn2, locals.var_t0__blk897_dn6, locals.var_t0__blk897_dn7, locals.var_t0__blk897_dn10, locals.var_t0__blk897_dn11, locals.var_t0__blk897_dn12, locals.var_t0__blk897_dn17,)
    }
};
        locals.var_t0__blk897 = assign29400_e41626;
        locals.var_t0__blk897_dn0 = assign29400_e41626_d_n0;
        locals.var_t0__blk897_dn2 = assign29400_e41626_d_n2;
        locals.var_t0__blk897_dn6 = assign29400_e41626_d_n6;
        locals.var_t0__blk897_dn7 = assign29400_e41626_d_n7;
        locals.var_t0__blk897_dn10 = assign29400_e41626_d_n10;
        locals.var_t0__blk897_dn11 = assign29400_e41626_d_n11;
        locals.var_t0__blk897_dn12 = assign29400_e41626_d_n12;
        locals.var_t0__blk897_dn17 = assign29400_e41626_d_n17;
        locals.var_t0__blk897_rv = 0.0;

        let (assign29410_e41642, assign29410_e41642_d_n0, assign29410_e41642_d_n2, assign29410_e41642_d_n6, assign29410_e41642_d_n7, assign29410_e41642_d_n10, assign29410_e41642_d_n11, assign29410_e41642_d_n12, assign29410_e41642_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29410_e41640: f64 = (locals.var_beta * locals.var_vgpld_shift__blk957);
        (assign29410_e41640, (locals.var_beta * locals.var_vgpld_shift__blk957_dn0), (locals.var_beta * locals.var_vgpld_shift__blk957_dn2), (locals.var_beta * locals.var_vgpld_shift__blk957_dn6), (locals.var_beta * locals.var_vgpld_shift__blk957_dn7), ((locals.var_beta_dn10 * locals.var_vgpld_shift__blk957) + (locals.var_beta * locals.var_vgpld_shift__blk957_dn10)), (locals.var_beta * locals.var_vgpld_shift__blk957_dn11), (locals.var_beta * locals.var_vgpld_shift__blk957_dn12), (locals.var_beta * locals.var_vgpld_shift__blk957_dn17),)
    } else {
        (locals.var_psi__blk960, locals.var_psi__blk960_dn0, locals.var_psi__blk960_dn2, locals.var_psi__blk960_dn6, locals.var_psi__blk960_dn7, locals.var_psi__blk960_dn10, locals.var_psi__blk960_dn11, locals.var_psi__blk960_dn12, locals.var_psi__blk960_dn17,)
    }
};
        locals.var_psi__blk960 = assign29410_e41642;
        locals.var_psi__blk960_dn0 = assign29410_e41642_d_n0;
        locals.var_psi__blk960_dn2 = assign29410_e41642_d_n2;
        locals.var_psi__blk960_dn6 = assign29410_e41642_d_n6;
        locals.var_psi__blk960_dn7 = assign29410_e41642_d_n7;
        locals.var_psi__blk960_dn10 = assign29410_e41642_d_n10;
        locals.var_psi__blk960_dn11 = assign29410_e41642_d_n11;
        locals.var_psi__blk960_dn12 = assign29410_e41642_d_n12;
        locals.var_psi__blk960_dn17 = assign29410_e41642_d_n17;
        locals.var_psi__blk960_rv = 0.0;

        let (assign29420_e41672, assign29420_e41672_d_n0, assign29420_e41672_d_n2, assign29420_e41672_d_n6, assign29420_e41672_d_n7, assign29420_e41672_d_n10, assign29420_e41672_d_n11, assign29420_e41672_d_n12, assign29420_e41672_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29420_e41656: f64 = (locals.var_gammachi__blk959 * locals.var_t0__blk897);
        let assign29420_e41659: f64 = (locals.var_psi__blk960 * locals.var_psi__blk960);
        let assign29420_e41660: f64 = (assign29420_e41656 + assign29420_e41659);
        let assign29420_e41661: f64 = (assign29420_e41660).ln();
        let assign29420_e41664: f64 = (locals.var_cnst1over__blk958 * locals.var_t0__blk897);
        let assign29420_e41665: f64 = (assign29420_e41664).ln();
        let assign29420_e41666: f64 = (assign29420_e41661 - assign29420_e41665);
        let assign29420_e41669: f64 = (locals.var_beta * locals.var_vxbgmtcl__blk923);
        let assign29420_e41670: f64 = (assign29420_e41666 + assign29420_e41669);
        (assign29420_e41670, ((((((locals.var_gammachi__blk959_dn0 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn0)) + ((locals.var_psi__blk960_dn0 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn0))) / assign29420_e41660) - (((locals.var_cnst1over__blk958_dn0 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn0)) / assign29420_e41664)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn0)), ((((((locals.var_gammachi__blk959_dn2 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn2)) + ((locals.var_psi__blk960_dn2 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn2))) / assign29420_e41660) - (((locals.var_cnst1over__blk958_dn2 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn2)) / assign29420_e41664)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn2)), ((((((locals.var_gammachi__blk959_dn6 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn6)) + ((locals.var_psi__blk960_dn6 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn6))) / assign29420_e41660) - (((locals.var_cnst1over__blk958_dn6 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn6)) / assign29420_e41664)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn6)), ((((((locals.var_gammachi__blk959_dn7 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn7)) + ((locals.var_psi__blk960_dn7 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn7))) / assign29420_e41660) - (((locals.var_cnst1over__blk958_dn7 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn7)) / assign29420_e41664)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn7)), ((((((locals.var_gammachi__blk959_dn10 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn10)) + ((locals.var_psi__blk960_dn10 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn10))) / assign29420_e41660) - (((locals.var_cnst1over__blk958_dn10 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn10)) / assign29420_e41664)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl__blk923) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn10))), ((((((locals.var_gammachi__blk959_dn11 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn11)) + ((locals.var_psi__blk960_dn11 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn11))) / assign29420_e41660) - (((locals.var_cnst1over__blk958_dn11 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn11)) / assign29420_e41664)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn11)), ((((((locals.var_gammachi__blk959_dn12 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn12)) + ((locals.var_psi__blk960_dn12 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn12))) / assign29420_e41660) - (((locals.var_cnst1over__blk958_dn12 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn12)) / assign29420_e41664)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn12)), ((((((locals.var_gammachi__blk959_dn17 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn17)) + ((locals.var_psi__blk960_dn17 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn17))) / assign29420_e41660) - (((locals.var_cnst1over__blk958_dn17 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn17)) / assign29420_e41664)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn17)),)
    } else {
        (locals.var_chi_1__blk961, locals.var_chi_1__blk961_dn0, locals.var_chi_1__blk961_dn2, locals.var_chi_1__blk961_dn6, locals.var_chi_1__blk961_dn7, locals.var_chi_1__blk961_dn10, locals.var_chi_1__blk961_dn11, locals.var_chi_1__blk961_dn12, locals.var_chi_1__blk961_dn17,)
    }
};
        locals.var_chi_1__blk961 = assign29420_e41672;
        locals.var_chi_1__blk961_dn0 = assign29420_e41672_d_n0;
        locals.var_chi_1__blk961_dn2 = assign29420_e41672_d_n2;
        locals.var_chi_1__blk961_dn6 = assign29420_e41672_d_n6;
        locals.var_chi_1__blk961_dn7 = assign29420_e41672_d_n7;
        locals.var_chi_1__blk961_dn10 = assign29420_e41672_d_n10;
        locals.var_chi_1__blk961_dn11 = assign29420_e41672_d_n11;
        locals.var_chi_1__blk961_dn12 = assign29420_e41672_d_n12;
        locals.var_chi_1__blk961_dn17 = assign29420_e41672_d_n17;
        locals.var_chi_1__blk961_rv = 0.0;

        let (assign29430_e41690, assign29430_e41690_d_n0, assign29430_e41690_d_n2, assign29430_e41690_d_n6, assign29430_e41690_d_n7, assign29430_e41690_d_n10, assign29430_e41690_d_n11, assign29430_e41690_d_n12, assign29430_e41690_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29430_e41686: f64 = (locals.var_psi__blk960 - locals.var_chi_1__blk961);
        let assign29430_e41688: f64 = (assign29430_e41686 - 1.0);
        (assign29430_e41688, (locals.var_psi__blk960_dn0 - locals.var_chi_1__blk961_dn0), (locals.var_psi__blk960_dn2 - locals.var_chi_1__blk961_dn2), (locals.var_psi__blk960_dn6 - locals.var_chi_1__blk961_dn6), (locals.var_psi__blk960_dn7 - locals.var_chi_1__blk961_dn7), (locals.var_psi__blk960_dn10 - locals.var_chi_1__blk961_dn10), (locals.var_psi__blk960_dn11 - locals.var_chi_1__blk961_dn11), (locals.var_psi__blk960_dn12 - locals.var_chi_1__blk961_dn12), (locals.var_psi__blk960_dn17 - locals.var_chi_1__blk961_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign29430_e41690;
        locals.var_tmf1_dn0 = assign29430_e41690_d_n0;
        locals.var_tmf1_dn2 = assign29430_e41690_d_n2;
        locals.var_tmf1_dn6 = assign29430_e41690_d_n6;
        locals.var_tmf1_dn7 = assign29430_e41690_d_n7;
        locals.var_tmf1_dn10 = assign29430_e41690_d_n10;
        locals.var_tmf1_dn11 = assign29430_e41690_d_n11;
        locals.var_tmf1_dn12 = assign29430_e41690_d_n12;
        locals.var_tmf1_dn17 = assign29430_e41690_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign29440_e41708, assign29440_e41708_d_n0, assign29440_e41708_d_n2, assign29440_e41708_d_n6, assign29440_e41708_d_n7, assign29440_e41708_d_n10, assign29440_e41708_d_n11, assign29440_e41708_d_n12, assign29440_e41708_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29440_e41704: f64 = (4.0 * locals.var_psi__blk960);
        let assign29440_e41706: f64 = assign29440_e41704;
        (assign29440_e41706, (4.0 * locals.var_psi__blk960_dn0), (4.0 * locals.var_psi__blk960_dn2), (4.0 * locals.var_psi__blk960_dn6), (4.0 * locals.var_psi__blk960_dn7), (4.0 * locals.var_psi__blk960_dn10), (4.0 * locals.var_psi__blk960_dn11), (4.0 * locals.var_psi__blk960_dn12), (4.0 * locals.var_psi__blk960_dn17),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29440_e41708;
        locals.var_tmf2_dn0 = assign29440_e41708_d_n0;
        locals.var_tmf2_dn2 = assign29440_e41708_d_n2;
        locals.var_tmf2_dn6 = assign29440_e41708_d_n6;
        locals.var_tmf2_dn7 = assign29440_e41708_d_n7;
        locals.var_tmf2_dn10 = assign29440_e41708_d_n10;
        locals.var_tmf2_dn11 = assign29440_e41708_d_n11;
        locals.var_tmf2_dn12 = assign29440_e41708_d_n12;
        locals.var_tmf2_dn17 = assign29440_e41708_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29450_e41728, assign29450_e41728_d_n0, assign29450_e41728_d_n2, assign29450_e41728_d_n6, assign29450_e41728_d_n7, assign29450_e41728_d_n10, assign29450_e41728_d_n11, assign29450_e41728_d_n12, assign29450_e41728_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let (assign29450_e41726, assign29450_e41726_d_n0, assign29450_e41726_d_n2, assign29450_e41726_d_n6, assign29450_e41726_d_n7, assign29450_e41726_d_n10, assign29450_e41726_d_n11, assign29450_e41726_d_n12, assign29450_e41726_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign29450_e41725: f64 = (-locals.var_tmf2);
                (assign29450_e41725, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign29450_e41726, assign29450_e41726_d_n0, assign29450_e41726_d_n2, assign29450_e41726_d_n6, assign29450_e41726_d_n7, assign29450_e41726_d_n10, assign29450_e41726_d_n11, assign29450_e41726_d_n12, assign29450_e41726_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29450_e41728;
        locals.var_tmf2_dn0 = assign29450_e41728_d_n0;
        locals.var_tmf2_dn2 = assign29450_e41728_d_n2;
        locals.var_tmf2_dn6 = assign29450_e41728_d_n6;
        locals.var_tmf2_dn7 = assign29450_e41728_d_n7;
        locals.var_tmf2_dn10 = assign29450_e41728_d_n10;
        locals.var_tmf2_dn11 = assign29450_e41728_d_n11;
        locals.var_tmf2_dn12 = assign29450_e41728_d_n12;
        locals.var_tmf2_dn17 = assign29450_e41728_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29460_e41747, assign29460_e41747_d_n0, assign29460_e41747_d_n2, assign29460_e41747_d_n6, assign29460_e41747_d_n7, assign29460_e41747_d_n10, assign29460_e41747_d_n11, assign29460_e41747_d_n12, assign29460_e41747_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29460_e41742: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign29460_e41744: f64 = (assign29460_e41742 + locals.var_tmf2);
        let assign29460_e41745: f64 = (assign29460_e41744).sqrt();
        (assign29460_e41745, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign29460_e41745)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign29460_e41745)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign29460_e41745)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign29460_e41745)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign29460_e41745)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign29460_e41745)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign29460_e41745)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign29460_e41745)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29460_e41747;
        locals.var_tmf2_dn0 = assign29460_e41747_d_n0;
        locals.var_tmf2_dn2 = assign29460_e41747_d_n2;
        locals.var_tmf2_dn6 = assign29460_e41747_d_n6;
        locals.var_tmf2_dn7 = assign29460_e41747_d_n7;
        locals.var_tmf2_dn10 = assign29460_e41747_d_n10;
        locals.var_tmf2_dn11 = assign29460_e41747_d_n11;
        locals.var_tmf2_dn12 = assign29460_e41747_d_n12;
        locals.var_tmf2_dn17 = assign29460_e41747_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29470_e41767, assign29470_e41767_d_n0, assign29470_e41767_d_n2, assign29470_e41767_d_n6, assign29470_e41767_d_n7, assign29470_e41767_d_n10, assign29470_e41767_d_n11, assign29470_e41767_d_n12, assign29470_e41767_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29470_e41763: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign29470_e41764: f64 = (1.0 + assign29470_e41763);
        let assign29470_e41765: f64 = (0.5 * assign29470_e41764);
        (assign29470_e41765, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17,)
    }
};
        locals.var_t1__blk898 = assign29470_e41767;
        locals.var_t1__blk898_dn0 = assign29470_e41767_d_n0;
        locals.var_t1__blk898_dn2 = assign29470_e41767_d_n2;
        locals.var_t1__blk898_dn6 = assign29470_e41767_d_n6;
        locals.var_t1__blk898_dn7 = assign29470_e41767_d_n7;
        locals.var_t1__blk898_dn10 = assign29470_e41767_d_n10;
        locals.var_t1__blk898_dn11 = assign29470_e41767_d_n11;
        locals.var_t1__blk898_dn12 = assign29470_e41767_d_n12;
        locals.var_t1__blk898_dn17 = assign29470_e41767_d_n17;
        locals.var_t1__blk898_rv = 0.0;

        let (assign29480_e41791, assign29480_e41791_d_n0, assign29480_e41791_d_n2, assign29480_e41791_d_n6, assign29480_e41791_d_n7, assign29480_e41791_d_n10, assign29480_e41791_d_n11, assign29480_e41791_d_n12, assign29480_e41791_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29480_e41784: f64 = 2.0;
        let assign29480_e41785: f64 = (locals.var_tmf1 + assign29480_e41784);
        let assign29480_e41787: f64 = (assign29480_e41785 / locals.var_tmf2);
        let assign29480_e41788: f64 = (1.0 - assign29480_e41787);
        let assign29480_e41789: f64 = (0.5 * assign29480_e41788);
        (assign29480_e41789, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign29480_e41785 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign29480_e41785 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign29480_e41785 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign29480_e41785 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign29480_e41785 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign29480_e41785 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign29480_e41785 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign29480_e41785 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk899, locals.var_t2__blk899_dn0, locals.var_t2__blk899_dn2, locals.var_t2__blk899_dn6, locals.var_t2__blk899_dn7, locals.var_t2__blk899_dn10, locals.var_t2__blk899_dn11, locals.var_t2__blk899_dn12, locals.var_t2__blk899_dn17,)
    }
};
        locals.var_t2__blk899 = assign29480_e41791;
        locals.var_t2__blk899_dn0 = assign29480_e41791_d_n0;
        locals.var_t2__blk899_dn2 = assign29480_e41791_d_n2;
        locals.var_t2__blk899_dn6 = assign29480_e41791_d_n6;
        locals.var_t2__blk899_dn7 = assign29480_e41791_d_n7;
        locals.var_t2__blk899_dn10 = assign29480_e41791_d_n10;
        locals.var_t2__blk899_dn11 = assign29480_e41791_d_n11;
        locals.var_t2__blk899_dn12 = assign29480_e41791_d_n12;
        locals.var_t2__blk899_dn17 = assign29480_e41791_d_n17;
        locals.var_t2__blk899_rv = 0.0;

        let (assign29490_e41811, assign29490_e41811_d_n0, assign29490_e41811_d_n2, assign29490_e41811_d_n6, assign29490_e41811_d_n7, assign29490_e41811_d_n10, assign29490_e41811_d_n11, assign29490_e41811_d_n12, assign29490_e41811_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29490_e41807: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign29490_e41808: f64 = (0.5 * assign29490_e41807);
        let assign29490_e41809: f64 = (locals.var_psi__blk960 - assign29490_e41808);
        (assign29490_e41809, (locals.var_psi__blk960_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi__blk960_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi__blk960_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi__blk960_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi__blk960_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi__blk960_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi__blk960_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psi__blk960_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi_1__blk961, locals.var_chi_1__blk961_dn0, locals.var_chi_1__blk961_dn2, locals.var_chi_1__blk961_dn6, locals.var_chi_1__blk961_dn7, locals.var_chi_1__blk961_dn10, locals.var_chi_1__blk961_dn11, locals.var_chi_1__blk961_dn12, locals.var_chi_1__blk961_dn17,)
    }
};
        locals.var_chi_1__blk961 = assign29490_e41811;
        locals.var_chi_1__blk961_dn0 = assign29490_e41811_d_n0;
        locals.var_chi_1__blk961_dn2 = assign29490_e41811_d_n2;
        locals.var_chi_1__blk961_dn6 = assign29490_e41811_d_n6;
        locals.var_chi_1__blk961_dn7 = assign29490_e41811_d_n7;
        locals.var_chi_1__blk961_dn10 = assign29490_e41811_d_n10;
        locals.var_chi_1__blk961_dn11 = assign29490_e41811_d_n11;
        locals.var_chi_1__blk961_dn12 = assign29490_e41811_d_n12;
        locals.var_chi_1__blk961_dn17 = assign29490_e41811_d_n17;
        locals.var_chi_1__blk961_rv = 0.0;

        let (assign29500_e41827, assign29500_e41827_d_n0, assign29500_e41827_d_n2, assign29500_e41827_d_n6, assign29500_e41827_d_n7, assign29500_e41827_d_n10, assign29500_e41827_d_n11, assign29500_e41827_d_n12, assign29500_e41827_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29500_e41825: f64 = (locals.var_psi__blk960 - locals.var_chi_1__blk961);
        (assign29500_e41825, (locals.var_psi__blk960_dn0 - locals.var_chi_1__blk961_dn0), (locals.var_psi__blk960_dn2 - locals.var_chi_1__blk961_dn2), (locals.var_psi__blk960_dn6 - locals.var_chi_1__blk961_dn6), (locals.var_psi__blk960_dn7 - locals.var_chi_1__blk961_dn7), (locals.var_psi__blk960_dn10 - locals.var_chi_1__blk961_dn10), (locals.var_psi__blk960_dn11 - locals.var_chi_1__blk961_dn11), (locals.var_psi__blk960_dn12 - locals.var_chi_1__blk961_dn12), (locals.var_psi__blk960_dn17 - locals.var_chi_1__blk961_dn17),)
    } else {
        (locals.var_psi__blk960, locals.var_psi__blk960_dn0, locals.var_psi__blk960_dn2, locals.var_psi__blk960_dn6, locals.var_psi__blk960_dn7, locals.var_psi__blk960_dn10, locals.var_psi__blk960_dn11, locals.var_psi__blk960_dn12, locals.var_psi__blk960_dn17,)
    }
};
        locals.var_psi__blk960 = assign29500_e41827;
        locals.var_psi__blk960_dn0 = assign29500_e41827_d_n0;
        locals.var_psi__blk960_dn2 = assign29500_e41827_d_n2;
        locals.var_psi__blk960_dn6 = assign29500_e41827_d_n6;
        locals.var_psi__blk960_dn7 = assign29500_e41827_d_n7;
        locals.var_psi__blk960_dn10 = assign29500_e41827_d_n10;
        locals.var_psi__blk960_dn11 = assign29500_e41827_d_n11;
        locals.var_psi__blk960_dn12 = assign29500_e41827_d_n12;
        locals.var_psi__blk960_dn17 = assign29500_e41827_d_n17;
        locals.var_psi__blk960_rv = 0.0;

        let (assign29510_e41845, assign29510_e41845_d_n0, assign29510_e41845_d_n2, assign29510_e41845_d_n6, assign29510_e41845_d_n7, assign29510_e41845_d_n10, assign29510_e41845_d_n11, assign29510_e41845_d_n12, assign29510_e41845_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29510_e41842: f64 = (locals.var_beta * 0.1);
        let assign29510_e41843: f64 = (locals.var_psi__blk960 + assign29510_e41842);
        (assign29510_e41843, locals.var_psi__blk960_dn0, locals.var_psi__blk960_dn2, locals.var_psi__blk960_dn6, locals.var_psi__blk960_dn7, (locals.var_psi__blk960_dn10 + (locals.var_beta_dn10 * 0.1)), locals.var_psi__blk960_dn11, locals.var_psi__blk960_dn12, locals.var_psi__blk960_dn17,)
    } else {
        (locals.var_psi__blk960, locals.var_psi__blk960_dn0, locals.var_psi__blk960_dn2, locals.var_psi__blk960_dn6, locals.var_psi__blk960_dn7, locals.var_psi__blk960_dn10, locals.var_psi__blk960_dn11, locals.var_psi__blk960_dn12, locals.var_psi__blk960_dn17,)
    }
};
        locals.var_psi__blk960 = assign29510_e41845;
        locals.var_psi__blk960_dn0 = assign29510_e41845_d_n0;
        locals.var_psi__blk960_dn2 = assign29510_e41845_d_n2;
        locals.var_psi__blk960_dn6 = assign29510_e41845_d_n6;
        locals.var_psi__blk960_dn7 = assign29510_e41845_d_n7;
        locals.var_psi__blk960_dn10 = assign29510_e41845_d_n10;
        locals.var_psi__blk960_dn11 = assign29510_e41845_d_n11;
        locals.var_psi__blk960_dn12 = assign29510_e41845_d_n12;
        locals.var_psi__blk960_dn17 = assign29510_e41845_d_n17;
        locals.var_psi__blk960_rv = 0.0;

        let (assign29520_e41875, assign29520_e41875_d_n0, assign29520_e41875_d_n2, assign29520_e41875_d_n6, assign29520_e41875_d_n7, assign29520_e41875_d_n10, assign29520_e41875_d_n11, assign29520_e41875_d_n12, assign29520_e41875_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29520_e41859: f64 = (locals.var_gammachi__blk959 * locals.var_t0__blk897);
        let assign29520_e41862: f64 = (locals.var_psi__blk960 * locals.var_psi__blk960);
        let assign29520_e41863: f64 = (assign29520_e41859 + assign29520_e41862);
        let assign29520_e41864: f64 = (assign29520_e41863).ln();
        let assign29520_e41867: f64 = (locals.var_cnst1over__blk958 * locals.var_t0__blk897);
        let assign29520_e41868: f64 = (assign29520_e41867).ln();
        let assign29520_e41869: f64 = (assign29520_e41864 - assign29520_e41868);
        let assign29520_e41872: f64 = (locals.var_beta * locals.var_vxbgmtcl__blk923);
        let assign29520_e41873: f64 = (assign29520_e41869 + assign29520_e41872);
        (assign29520_e41873, ((((((locals.var_gammachi__blk959_dn0 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn0)) + ((locals.var_psi__blk960_dn0 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn0))) / assign29520_e41863) - (((locals.var_cnst1over__blk958_dn0 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn0)) / assign29520_e41867)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn0)), ((((((locals.var_gammachi__blk959_dn2 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn2)) + ((locals.var_psi__blk960_dn2 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn2))) / assign29520_e41863) - (((locals.var_cnst1over__blk958_dn2 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn2)) / assign29520_e41867)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn2)), ((((((locals.var_gammachi__blk959_dn6 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn6)) + ((locals.var_psi__blk960_dn6 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn6))) / assign29520_e41863) - (((locals.var_cnst1over__blk958_dn6 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn6)) / assign29520_e41867)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn6)), ((((((locals.var_gammachi__blk959_dn7 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn7)) + ((locals.var_psi__blk960_dn7 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn7))) / assign29520_e41863) - (((locals.var_cnst1over__blk958_dn7 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn7)) / assign29520_e41867)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn7)), ((((((locals.var_gammachi__blk959_dn10 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn10)) + ((locals.var_psi__blk960_dn10 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn10))) / assign29520_e41863) - (((locals.var_cnst1over__blk958_dn10 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn10)) / assign29520_e41867)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl__blk923) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn10))), ((((((locals.var_gammachi__blk959_dn11 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn11)) + ((locals.var_psi__blk960_dn11 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn11))) / assign29520_e41863) - (((locals.var_cnst1over__blk958_dn11 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn11)) / assign29520_e41867)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn11)), ((((((locals.var_gammachi__blk959_dn12 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn12)) + ((locals.var_psi__blk960_dn12 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn12))) / assign29520_e41863) - (((locals.var_cnst1over__blk958_dn12 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn12)) / assign29520_e41867)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn12)), ((((((locals.var_gammachi__blk959_dn17 * locals.var_t0__blk897) + (locals.var_gammachi__blk959 * locals.var_t0__blk897_dn17)) + ((locals.var_psi__blk960_dn17 * locals.var_psi__blk960) + (locals.var_psi__blk960 * locals.var_psi__blk960_dn17))) / assign29520_e41863) - (((locals.var_cnst1over__blk958_dn17 * locals.var_t0__blk897) + (locals.var_cnst1over__blk958 * locals.var_t0__blk897_dn17)) / assign29520_e41867)) + (locals.var_beta * locals.var_vxbgmtcl__blk923_dn17)),)
    } else {
        (locals.var_chi_b__blk962, locals.var_chi_b__blk962_dn0, locals.var_chi_b__blk962_dn2, locals.var_chi_b__blk962_dn6, locals.var_chi_b__blk962_dn7, locals.var_chi_b__blk962_dn10, locals.var_chi_b__blk962_dn11, locals.var_chi_b__blk962_dn12, locals.var_chi_b__blk962_dn17,)
    }
};
        locals.var_chi_b__blk962 = assign29520_e41875;
        locals.var_chi_b__blk962_dn0 = assign29520_e41875_d_n0;
        locals.var_chi_b__blk962_dn2 = assign29520_e41875_d_n2;
        locals.var_chi_b__blk962_dn6 = assign29520_e41875_d_n6;
        locals.var_chi_b__blk962_dn7 = assign29520_e41875_d_n7;
        locals.var_chi_b__blk962_dn10 = assign29520_e41875_d_n10;
        locals.var_chi_b__blk962_dn11 = assign29520_e41875_d_n11;
        locals.var_chi_b__blk962_dn12 = assign29520_e41875_d_n12;
        locals.var_chi_b__blk962_dn17 = assign29520_e41875_d_n17;
        locals.var_chi_b__blk962_rv = 0.0;

        let (assign29530_e41889, assign29530_e41889_d_n0, assign29530_e41889_d_n2, assign29530_e41889_d_n6, assign29530_e41889_d_n7, assign29530_e41889_d_n10, assign29530_e41889_d_n11, assign29530_e41889_d_n12, assign29530_e41889_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        (locals.var_chi__blk945, locals.var_chi__blk945_dn0, locals.var_chi__blk945_dn2, locals.var_chi__blk945_dn6, locals.var_chi__blk945_dn7, locals.var_chi__blk945_dn10, locals.var_chi__blk945_dn11, locals.var_chi__blk945_dn12, locals.var_chi__blk945_dn17,)
    } else {
        (locals.var_chi_a__blk963, locals.var_chi_a__blk963_dn0, locals.var_chi_a__blk963_dn2, locals.var_chi_a__blk963_dn6, locals.var_chi_a__blk963_dn7, locals.var_chi_a__blk963_dn10, locals.var_chi_a__blk963_dn11, locals.var_chi_a__blk963_dn12, locals.var_chi_a__blk963_dn17,)
    }
};
        locals.var_chi_a__blk963 = assign29530_e41889;
        locals.var_chi_a__blk963_dn0 = assign29530_e41889_d_n0;
        locals.var_chi_a__blk963_dn2 = assign29530_e41889_d_n2;
        locals.var_chi_a__blk963_dn6 = assign29530_e41889_d_n6;
        locals.var_chi_a__blk963_dn7 = assign29530_e41889_d_n7;
        locals.var_chi_a__blk963_dn10 = assign29530_e41889_d_n10;
        locals.var_chi_a__blk963_dn11 = assign29530_e41889_d_n11;
        locals.var_chi_a__blk963_dn12 = assign29530_e41889_d_n12;
        locals.var_chi_a__blk963_dn17 = assign29530_e41889_d_n17;
        locals.var_chi_a__blk963_rv = 0.0;

        let (assign29540_e41909, assign29540_e41909_d_n0, assign29540_e41909_d_n2, assign29540_e41909_d_n6, assign29540_e41909_d_n7, assign29540_e41909_d_n10, assign29540_e41909_d_n11, assign29540_e41909_d_n12, assign29540_e41909_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29540_e41903: f64 = (locals.var_chi_b__blk962 - locals.var_chi_a__blk963);
        let assign29540_e41906: f64 = (0.0008 * 75.0);
        let assign29540_e41907: f64 = (assign29540_e41903 - assign29540_e41906);
        (assign29540_e41907, (locals.var_chi_b__blk962_dn0 - locals.var_chi_a__blk963_dn0), (locals.var_chi_b__blk962_dn2 - locals.var_chi_a__blk963_dn2), (locals.var_chi_b__blk962_dn6 - locals.var_chi_a__blk963_dn6), (locals.var_chi_b__blk962_dn7 - locals.var_chi_a__blk963_dn7), (locals.var_chi_b__blk962_dn10 - locals.var_chi_a__blk963_dn10), (locals.var_chi_b__blk962_dn11 - locals.var_chi_a__blk963_dn11), (locals.var_chi_b__blk962_dn12 - locals.var_chi_a__blk963_dn12), (locals.var_chi_b__blk962_dn17 - locals.var_chi_a__blk963_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign29540_e41909;
        locals.var_tmf1_dn0 = assign29540_e41909_d_n0;
        locals.var_tmf1_dn2 = assign29540_e41909_d_n2;
        locals.var_tmf1_dn6 = assign29540_e41909_d_n6;
        locals.var_tmf1_dn7 = assign29540_e41909_d_n7;
        locals.var_tmf1_dn10 = assign29540_e41909_d_n10;
        locals.var_tmf1_dn11 = assign29540_e41909_d_n11;
        locals.var_tmf1_dn12 = assign29540_e41909_d_n12;
        locals.var_tmf1_dn17 = assign29540_e41909_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign29550_e41929, assign29550_e41929_d_n0, assign29550_e41929_d_n2, assign29550_e41929_d_n6, assign29550_e41929_d_n7, assign29550_e41929_d_n10, assign29550_e41929_d_n11, assign29550_e41929_d_n12, assign29550_e41929_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29550_e41923: f64 = (4.0 * locals.var_chi_b__blk962);
        let assign29550_e41926: f64 = (0.0008 * 75.0);
        let assign29550_e41927: f64 = (assign29550_e41923 * assign29550_e41926);
        (assign29550_e41927, ((4.0 * locals.var_chi_b__blk962_dn0) * assign29550_e41926), ((4.0 * locals.var_chi_b__blk962_dn2) * assign29550_e41926), ((4.0 * locals.var_chi_b__blk962_dn6) * assign29550_e41926), ((4.0 * locals.var_chi_b__blk962_dn7) * assign29550_e41926), ((4.0 * locals.var_chi_b__blk962_dn10) * assign29550_e41926), ((4.0 * locals.var_chi_b__blk962_dn11) * assign29550_e41926), ((4.0 * locals.var_chi_b__blk962_dn12) * assign29550_e41926), ((4.0 * locals.var_chi_b__blk962_dn17) * assign29550_e41926),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29550_e41929;
        locals.var_tmf2_dn0 = assign29550_e41929_d_n0;
        locals.var_tmf2_dn2 = assign29550_e41929_d_n2;
        locals.var_tmf2_dn6 = assign29550_e41929_d_n6;
        locals.var_tmf2_dn7 = assign29550_e41929_d_n7;
        locals.var_tmf2_dn10 = assign29550_e41929_d_n10;
        locals.var_tmf2_dn11 = assign29550_e41929_d_n11;
        locals.var_tmf2_dn12 = assign29550_e41929_d_n12;
        locals.var_tmf2_dn17 = assign29550_e41929_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29560_e41949, assign29560_e41949_d_n0, assign29560_e41949_d_n2, assign29560_e41949_d_n6, assign29560_e41949_d_n7, assign29560_e41949_d_n10, assign29560_e41949_d_n11, assign29560_e41949_d_n12, assign29560_e41949_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let (assign29560_e41947, assign29560_e41947_d_n0, assign29560_e41947_d_n2, assign29560_e41947_d_n6, assign29560_e41947_d_n7, assign29560_e41947_d_n10, assign29560_e41947_d_n11, assign29560_e41947_d_n12, assign29560_e41947_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign29560_e41946: f64 = (-locals.var_tmf2);
                (assign29560_e41946, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign29560_e41947, assign29560_e41947_d_n0, assign29560_e41947_d_n2, assign29560_e41947_d_n6, assign29560_e41947_d_n7, assign29560_e41947_d_n10, assign29560_e41947_d_n11, assign29560_e41947_d_n12, assign29560_e41947_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29560_e41949;
        locals.var_tmf2_dn0 = assign29560_e41949_d_n0;
        locals.var_tmf2_dn2 = assign29560_e41949_d_n2;
        locals.var_tmf2_dn6 = assign29560_e41949_d_n6;
        locals.var_tmf2_dn7 = assign29560_e41949_d_n7;
        locals.var_tmf2_dn10 = assign29560_e41949_d_n10;
        locals.var_tmf2_dn11 = assign29560_e41949_d_n11;
        locals.var_tmf2_dn12 = assign29560_e41949_d_n12;
        locals.var_tmf2_dn17 = assign29560_e41949_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29570_e41968, assign29570_e41968_d_n0, assign29570_e41968_d_n2, assign29570_e41968_d_n6, assign29570_e41968_d_n7, assign29570_e41968_d_n10, assign29570_e41968_d_n11, assign29570_e41968_d_n12, assign29570_e41968_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29570_e41963: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign29570_e41965: f64 = (assign29570_e41963 + locals.var_tmf2);
        let assign29570_e41966: f64 = (assign29570_e41965).sqrt();
        (assign29570_e41966, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign29570_e41966)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign29570_e41966)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign29570_e41966)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign29570_e41966)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign29570_e41966)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign29570_e41966)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign29570_e41966)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign29570_e41966)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29570_e41968;
        locals.var_tmf2_dn0 = assign29570_e41968_d_n0;
        locals.var_tmf2_dn2 = assign29570_e41968_d_n2;
        locals.var_tmf2_dn6 = assign29570_e41968_d_n6;
        locals.var_tmf2_dn7 = assign29570_e41968_d_n7;
        locals.var_tmf2_dn10 = assign29570_e41968_d_n10;
        locals.var_tmf2_dn11 = assign29570_e41968_d_n11;
        locals.var_tmf2_dn12 = assign29570_e41968_d_n12;
        locals.var_tmf2_dn17 = assign29570_e41968_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29580_e41988, assign29580_e41988_d_n0, assign29580_e41988_d_n2, assign29580_e41988_d_n6, assign29580_e41988_d_n7, assign29580_e41988_d_n10, assign29580_e41988_d_n11, assign29580_e41988_d_n12, assign29580_e41988_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29580_e41984: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign29580_e41985: f64 = (1.0 + assign29580_e41984);
        let assign29580_e41986: f64 = (0.5 * assign29580_e41985);
        (assign29580_e41986, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17,)
    }
};
        locals.var_t1__blk898 = assign29580_e41988;
        locals.var_t1__blk898_dn0 = assign29580_e41988_d_n0;
        locals.var_t1__blk898_dn2 = assign29580_e41988_d_n2;
        locals.var_t1__blk898_dn6 = assign29580_e41988_d_n6;
        locals.var_t1__blk898_dn7 = assign29580_e41988_d_n7;
        locals.var_t1__blk898_dn10 = assign29580_e41988_d_n10;
        locals.var_t1__blk898_dn11 = assign29580_e41988_d_n11;
        locals.var_t1__blk898_dn12 = assign29580_e41988_d_n12;
        locals.var_t1__blk898_dn17 = assign29580_e41988_d_n17;
        locals.var_t1__blk898_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_107(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29590_e42014, assign29590_e42014_d_n0, assign29590_e42014_d_n2, assign29590_e42014_d_n6, assign29590_e42014_d_n7, assign29590_e42014_d_n10, assign29590_e42014_d_n11, assign29590_e42014_d_n12, assign29590_e42014_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29590_e42005: f64 = (2.0 * 0.0008);
        let assign29590_e42007: f64 = (assign29590_e42005 * 75.0);
        let assign29590_e42008: f64 = (locals.var_tmf1 + assign29590_e42007);
        let assign29590_e42010: f64 = (assign29590_e42008 / locals.var_tmf2);
        let assign29590_e42011: f64 = (1.0 - assign29590_e42010);
        let assign29590_e42012: f64 = (0.5 * assign29590_e42011);
        (assign29590_e42012, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign29590_e42008 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign29590_e42008 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign29590_e42008 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign29590_e42008 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign29590_e42008 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign29590_e42008 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign29590_e42008 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign29590_e42008 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk899, locals.var_t2__blk899_dn0, locals.var_t2__blk899_dn2, locals.var_t2__blk899_dn6, locals.var_t2__blk899_dn7, locals.var_t2__blk899_dn10, locals.var_t2__blk899_dn11, locals.var_t2__blk899_dn12, locals.var_t2__blk899_dn17,)
    }
};
        locals.var_t2__blk899 = assign29590_e42014;
        locals.var_t2__blk899_dn0 = assign29590_e42014_d_n0;
        locals.var_t2__blk899_dn2 = assign29590_e42014_d_n2;
        locals.var_t2__blk899_dn6 = assign29590_e42014_d_n6;
        locals.var_t2__blk899_dn7 = assign29590_e42014_d_n7;
        locals.var_t2__blk899_dn10 = assign29590_e42014_d_n10;
        locals.var_t2__blk899_dn11 = assign29590_e42014_d_n11;
        locals.var_t2__blk899_dn12 = assign29590_e42014_d_n12;
        locals.var_t2__blk899_dn17 = assign29590_e42014_d_n17;
        locals.var_t2__blk899_rv = 0.0;

        let (assign29600_e42034, assign29600_e42034_d_n0, assign29600_e42034_d_n2, assign29600_e42034_d_n6, assign29600_e42034_d_n7, assign29600_e42034_d_n10, assign29600_e42034_d_n11, assign29600_e42034_d_n12, assign29600_e42034_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29600_e42030: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign29600_e42031: f64 = (0.5 * assign29600_e42030);
        let assign29600_e42032: f64 = (locals.var_chi_b__blk962 - assign29600_e42031);
        (assign29600_e42032, (locals.var_chi_b__blk962_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b__blk962_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b__blk962_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b__blk962_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_chi_b__blk962_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b__blk962_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b__blk962_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_chi_b__blk962_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi__blk945, locals.var_chi__blk945_dn0, locals.var_chi__blk945_dn2, locals.var_chi__blk945_dn6, locals.var_chi__blk945_dn7, locals.var_chi__blk945_dn10, locals.var_chi__blk945_dn11, locals.var_chi__blk945_dn12, locals.var_chi__blk945_dn17,)
    }
};
        locals.var_chi__blk945 = assign29600_e42034;
        locals.var_chi__blk945_dn0 = assign29600_e42034_d_n0;
        locals.var_chi__blk945_dn2 = assign29600_e42034_d_n2;
        locals.var_chi__blk945_dn6 = assign29600_e42034_d_n6;
        locals.var_chi__blk945_dn7 = assign29600_e42034_d_n7;
        locals.var_chi__blk945_dn10 = assign29600_e42034_d_n10;
        locals.var_chi__blk945_dn11 = assign29600_e42034_d_n11;
        locals.var_chi__blk945_dn12 = assign29600_e42034_d_n12;
        locals.var_chi__blk945_dn17 = assign29600_e42034_d_n17;
        locals.var_chi__blk945_rv = 0.0;

        let (assign29610_e42050, assign29610_e42050_d_n0, assign29610_e42050_d_n2, assign29610_e42050_d_n6, assign29610_e42050_d_n7, assign29610_e42050_d_n10, assign29610_e42050_d_n11, assign29610_e42050_d_n12, assign29610_e42050_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) {
        let assign29610_e42046: f64 = (locals.var_chi__blk945 / locals.var_beta);
        let assign29610_e42048: f64 = (assign29610_e42046 - locals.var_vxbgmtcl__blk923);
        (assign29610_e42048, ((locals.var_chi__blk945_dn0 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn0), ((locals.var_chi__blk945_dn2 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn2), ((locals.var_chi__blk945_dn6 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn6), ((locals.var_chi__blk945_dn7 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn7), ((((locals.var_chi__blk945_dn10 * locals.var_beta) - (locals.var_chi__blk945 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl__blk923_dn10), ((locals.var_chi__blk945_dn11 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn11), ((locals.var_chi__blk945_dn12 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn12), ((locals.var_chi__blk945_dn17 / locals.var_beta) - locals.var_vxbgmtcl__blk923_dn17),)
    } else {
        (locals.var_ps0ld__blk947, locals.var_ps0ld__blk947_dn0, locals.var_ps0ld__blk947_dn2, locals.var_ps0ld__blk947_dn6, locals.var_ps0ld__blk947_dn7, locals.var_ps0ld__blk947_dn10, locals.var_ps0ld__blk947_dn11, locals.var_ps0ld__blk947_dn12, locals.var_ps0ld__blk947_dn17,)
    }
};
        locals.var_ps0ld__blk947 = assign29610_e42050;
        locals.var_ps0ld__blk947_dn0 = assign29610_e42050_d_n0;
        locals.var_ps0ld__blk947_dn2 = assign29610_e42050_d_n2;
        locals.var_ps0ld__blk947_dn6 = assign29610_e42050_d_n6;
        locals.var_ps0ld__blk947_dn7 = assign29610_e42050_d_n7;
        locals.var_ps0ld__blk947_dn10 = assign29610_e42050_d_n10;
        locals.var_ps0ld__blk947_dn11 = assign29610_e42050_d_n11;
        locals.var_ps0ld__blk947_dn12 = assign29610_e42050_d_n12;
        locals.var_ps0ld__blk947_dn17 = assign29610_e42050_d_n17;
        locals.var_ps0ld__blk947_rv = 0.0;

        let (assign29620_e42068, assign29620_e42068_d_n0, assign29620_e42068_d_n2, assign29620_e42068_d_n6, assign29620_e42068_d_n7, assign29620_e42068_d_n10, assign29620_e42068_d_n11, assign29620_e42068_d_n12, assign29620_e42068_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) {
        let assign29620_e42062: f64 = (locals.var_chi__blk945 - 1.0);
        let assign29620_e42064: f64 = (-locals.var_chi__blk945);
        let assign29620_e42065: f64 = (assign29620_e42064).exp();
        let assign29620_e42066: f64 = (assign29620_e42062 + assign29620_e42065);
        (assign29620_e42066, (locals.var_chi__blk945_dn0 + (assign29620_e42065 * (-locals.var_chi__blk945_dn0))), (locals.var_chi__blk945_dn2 + (assign29620_e42065 * (-locals.var_chi__blk945_dn2))), (locals.var_chi__blk945_dn6 + (assign29620_e42065 * (-locals.var_chi__blk945_dn6))), (locals.var_chi__blk945_dn7 + (assign29620_e42065 * (-locals.var_chi__blk945_dn7))), (locals.var_chi__blk945_dn10 + (assign29620_e42065 * (-locals.var_chi__blk945_dn10))), (locals.var_chi__blk945_dn11 + (assign29620_e42065 * (-locals.var_chi__blk945_dn11))), (locals.var_chi__blk945_dn12 + (assign29620_e42065 * (-locals.var_chi__blk945_dn12))), (locals.var_chi__blk945_dn17 + (assign29620_e42065 * (-locals.var_chi__blk945_dn17))),)
    } else {
        (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17,)
    }
};
        locals.var_t1__blk898 = assign29620_e42068;
        locals.var_t1__blk898_dn0 = assign29620_e42068_d_n0;
        locals.var_t1__blk898_dn2 = assign29620_e42068_d_n2;
        locals.var_t1__blk898_dn6 = assign29620_e42068_d_n6;
        locals.var_t1__blk898_dn7 = assign29620_e42068_d_n7;
        locals.var_t1__blk898_dn10 = assign29620_e42068_d_n10;
        locals.var_t1__blk898_dn11 = assign29620_e42068_d_n11;
        locals.var_t1__blk898_dn12 = assign29620_e42068_d_n12;
        locals.var_t1__blk898_dn17 = assign29620_e42068_d_n17;
        locals.var_t1__blk898_rv = 0.0;

        let assign29630_e42072: f64 = (10.0 * 2.220446049250313e-16);
        let assign29630_e42073: f64 = if locals.var_t1__blk898 < assign29630_e42072 { 1.0 } else { 0.0 };
        locals.var_guard990 = assign29630_e42073;
        locals.var_guard990_rv = 0.0;

        let (assign29640_e42089, assign29640_e42089_d_n0, assign29640_e42089_d_n2, assign29640_e42089_d_n6, assign29640_e42089_d_n7, assign29640_e42089_d_n10, assign29640_e42089_d_n11, assign29640_e42089_d_n12, assign29640_e42089_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29640_e42087: f64 = (10.0 * 2.220446049250313e-16);
        (assign29640_e42087, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17,)
    }
};
        locals.var_t1__blk898 = assign29640_e42089;
        locals.var_t1__blk898_dn0 = assign29640_e42089_d_n0;
        locals.var_t1__blk898_dn2 = assign29640_e42089_d_n2;
        locals.var_t1__blk898_dn6 = assign29640_e42089_d_n6;
        locals.var_t1__blk898_dn7 = assign29640_e42089_d_n7;
        locals.var_t1__blk898_dn10 = assign29640_e42089_d_n10;
        locals.var_t1__blk898_dn11 = assign29640_e42089_d_n11;
        locals.var_t1__blk898_dn12 = assign29640_e42089_d_n12;
        locals.var_t1__blk898_dn17 = assign29640_e42089_d_n17;
        locals.var_t1__blk898_rv = 0.0;

        let (assign29650_e42102, assign29650_e42102_d_n0, assign29650_e42102_d_n2, assign29650_e42102_d_n6, assign29650_e42102_d_n7, assign29650_e42102_d_n10, assign29650_e42102_d_n11, assign29650_e42102_d_n12, assign29650_e42102_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) {
        let assign29650_e42100: f64 = (locals.var_t1__blk898).sqrt();
        (assign29650_e42100, (locals.var_t1__blk898_dn0 / (2.0 * assign29650_e42100)), (locals.var_t1__blk898_dn2 / (2.0 * assign29650_e42100)), (locals.var_t1__blk898_dn6 / (2.0 * assign29650_e42100)), (locals.var_t1__blk898_dn7 / (2.0 * assign29650_e42100)), (locals.var_t1__blk898_dn10 / (2.0 * assign29650_e42100)), (locals.var_t1__blk898_dn11 / (2.0 * assign29650_e42100)), (locals.var_t1__blk898_dn12 / (2.0 * assign29650_e42100)), (locals.var_t1__blk898_dn17 / (2.0 * assign29650_e42100)),)
    } else {
        (locals.var_t2__blk899, locals.var_t2__blk899_dn0, locals.var_t2__blk899_dn2, locals.var_t2__blk899_dn6, locals.var_t2__blk899_dn7, locals.var_t2__blk899_dn10, locals.var_t2__blk899_dn11, locals.var_t2__blk899_dn12, locals.var_t2__blk899_dn17,)
    }
};
        locals.var_t2__blk899 = assign29650_e42102;
        locals.var_t2__blk899_dn0 = assign29650_e42102_d_n0;
        locals.var_t2__blk899_dn2 = assign29650_e42102_d_n2;
        locals.var_t2__blk899_dn6 = assign29650_e42102_d_n6;
        locals.var_t2__blk899_dn7 = assign29650_e42102_d_n7;
        locals.var_t2__blk899_dn10 = assign29650_e42102_d_n10;
        locals.var_t2__blk899_dn11 = assign29650_e42102_d_n11;
        locals.var_t2__blk899_dn12 = assign29650_e42102_d_n12;
        locals.var_t2__blk899_dn17 = assign29650_e42102_d_n17;
        locals.var_t2__blk899_rv = 0.0;

        let (assign29660_e42116, assign29660_e42116_d_n0, assign29660_e42116_d_n2, assign29660_e42116_d_n6, assign29660_e42116_d_n7, assign29660_e42116_d_n10, assign29660_e42116_d_n11, assign29660_e42116_d_n12, assign29660_e42116_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) {
        let assign29660_e42114: f64 = (locals.var_cnst0over__blk930 * locals.var_t2__blk899);
        (assign29660_e42114, ((locals.var_cnst0over__blk930_dn0 * locals.var_t2__blk899) + (locals.var_cnst0over__blk930 * locals.var_t2__blk899_dn0)), ((locals.var_cnst0over__blk930_dn2 * locals.var_t2__blk899) + (locals.var_cnst0over__blk930 * locals.var_t2__blk899_dn2)), ((locals.var_cnst0over__blk930_dn6 * locals.var_t2__blk899) + (locals.var_cnst0over__blk930 * locals.var_t2__blk899_dn6)), ((locals.var_cnst0over__blk930_dn7 * locals.var_t2__blk899) + (locals.var_cnst0over__blk930 * locals.var_t2__blk899_dn7)), ((locals.var_cnst0over__blk930_dn10 * locals.var_t2__blk899) + (locals.var_cnst0over__blk930 * locals.var_t2__blk899_dn10)), ((locals.var_cnst0over__blk930_dn11 * locals.var_t2__blk899) + (locals.var_cnst0over__blk930 * locals.var_t2__blk899_dn11)), ((locals.var_cnst0over__blk930_dn12 * locals.var_t2__blk899) + (locals.var_cnst0over__blk930 * locals.var_t2__blk899_dn12)), ((locals.var_cnst0over__blk930_dn17 * locals.var_t2__blk899) + (locals.var_cnst0over__blk930 * locals.var_t2__blk899_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign29660_e42116;
        locals.var_qbuld_dn0 = assign29660_e42116_d_n0;
        locals.var_qbuld_dn2 = assign29660_e42116_d_n2;
        locals.var_qbuld_dn6 = assign29660_e42116_d_n6;
        locals.var_qbuld_dn7 = assign29660_e42116_d_n7;
        locals.var_qbuld_dn10 = assign29660_e42116_d_n10;
        locals.var_qbuld_dn11 = assign29660_e42116_d_n11;
        locals.var_qbuld_dn12 = assign29660_e42116_d_n12;
        locals.var_qbuld_dn17 = assign29660_e42116_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign29670_e42132, assign29670_e42132_d_n0, assign29670_e42132_d_n2, assign29670_e42132_d_n6, assign29670_e42132_d_n7, assign29670_e42132_d_n10, assign29670_e42132_d_n11, assign29670_e42132_d_n12, assign29670_e42132_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) {
        let assign29670_e42129: f64 = (locals.var_vgpld__blk933 - locals.var_ps0ld__blk947);
        let assign29670_e42130: f64 = (locals.var_cox0__blk908 * assign29670_e42129);
        (assign29670_e42130, (locals.var_cox0__blk908 * (locals.var_vgpld__blk933_dn0 - locals.var_ps0ld__blk947_dn0)), (locals.var_cox0__blk908 * (locals.var_vgpld__blk933_dn2 - locals.var_ps0ld__blk947_dn2)), (locals.var_cox0__blk908 * (locals.var_vgpld__blk933_dn6 - locals.var_ps0ld__blk947_dn6)), (locals.var_cox0__blk908 * (locals.var_vgpld__blk933_dn7 - locals.var_ps0ld__blk947_dn7)), (locals.var_cox0__blk908 * (locals.var_vgpld__blk933_dn10 - locals.var_ps0ld__blk947_dn10)), (locals.var_cox0__blk908 * (locals.var_vgpld__blk933_dn11 - locals.var_ps0ld__blk947_dn11)), (locals.var_cox0__blk908 * (locals.var_vgpld__blk933_dn12 - locals.var_ps0ld__blk947_dn12)), (locals.var_cox0__blk908 * (locals.var_vgpld__blk933_dn17 - locals.var_ps0ld__blk947_dn17)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign29670_e42132;
        locals.var_qsuld_dn0 = assign29670_e42132_d_n0;
        locals.var_qsuld_dn2 = assign29670_e42132_d_n2;
        locals.var_qsuld_dn6 = assign29670_e42132_d_n6;
        locals.var_qsuld_dn7 = assign29670_e42132_d_n7;
        locals.var_qsuld_dn10 = assign29670_e42132_d_n10;
        locals.var_qsuld_dn11 = assign29670_e42132_d_n11;
        locals.var_qsuld_dn12 = assign29670_e42132_d_n12;
        locals.var_qsuld_dn17 = assign29670_e42132_d_n17;
        locals.var_qsuld_rv = 0.0;

        let assign29680_e42135: f64 = if p.p41 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard991 = assign29680_e42135;
        locals.var_guard991_rv = 0.0;

        let (assign29690_e42153, assign29690_e42153_d_n0, assign29690_e42153_d_n2, assign29690_e42153_d_n6, assign29690_e42153_d_n7, assign29690_e42153_d_n10, assign29690_e42153_d_n11, assign29690_e42153_d_n12, assign29690_e42153_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29690_e42149: f64 = (-locals.var_vxbgmtcl__blk923);
        let assign29690_e42150: f64 = (locals.var_beta * assign29690_e42149);
        let assign29690_e42151: f64 = (assign29690_e42150).exp();
        (assign29690_e42151, (assign29690_e42151 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn0))), (assign29690_e42151 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn2))), (assign29690_e42151 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn6))), (assign29690_e42151 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn7))), (assign29690_e42151 * ((locals.var_beta_dn10 * assign29690_e42149) + (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn10)))), (assign29690_e42151 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn11))), (assign29690_e42151 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn12))), (assign29690_e42151 * (locals.var_beta * (-locals.var_vxbgmtcl__blk923_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk964, locals.var_exp_bvbs__blk964_dn0, locals.var_exp_bvbs__blk964_dn2, locals.var_exp_bvbs__blk964_dn6, locals.var_exp_bvbs__blk964_dn7, locals.var_exp_bvbs__blk964_dn10, locals.var_exp_bvbs__blk964_dn11, locals.var_exp_bvbs__blk964_dn12, locals.var_exp_bvbs__blk964_dn17,)
    }
};
        locals.var_exp_bvbs__blk964 = assign29690_e42153;
        locals.var_exp_bvbs__blk964_dn0 = assign29690_e42153_d_n0;
        locals.var_exp_bvbs__blk964_dn2 = assign29690_e42153_d_n2;
        locals.var_exp_bvbs__blk964_dn6 = assign29690_e42153_d_n6;
        locals.var_exp_bvbs__blk964_dn7 = assign29690_e42153_d_n7;
        locals.var_exp_bvbs__blk964_dn10 = assign29690_e42153_d_n10;
        locals.var_exp_bvbs__blk964_dn11 = assign29690_e42153_d_n11;
        locals.var_exp_bvbs__blk964_dn12 = assign29690_e42153_d_n12;
        locals.var_exp_bvbs__blk964_dn17 = assign29690_e42153_d_n17;
        locals.var_exp_bvbs__blk964_rv = 0.0;

        let (assign29700_e42169, assign29700_e42169_d_n0, assign29700_e42169_d_n2, assign29700_e42169_d_n6, assign29700_e42169_d_n7, assign29700_e42169_d_n10, assign29700_e42169_d_n11, assign29700_e42169_d_n12, assign29700_e42169_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29700_e42167: f64 = (locals.var_nin / locals.var_mks_nover);
        (assign29700_e42167, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn7 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover), (locals.var_nin_dn17 / locals.var_mks_nover),)
    } else {
        (locals.var_t0__blk897, locals.var_t0__blk897_dn0, locals.var_t0__blk897_dn2, locals.var_t0__blk897_dn6, locals.var_t0__blk897_dn7, locals.var_t0__blk897_dn10, locals.var_t0__blk897_dn11, locals.var_t0__blk897_dn12, locals.var_t0__blk897_dn17,)
    }
};
        locals.var_t0__blk897 = assign29700_e42169;
        locals.var_t0__blk897_dn0 = assign29700_e42169_d_n0;
        locals.var_t0__blk897_dn2 = assign29700_e42169_d_n2;
        locals.var_t0__blk897_dn6 = assign29700_e42169_d_n6;
        locals.var_t0__blk897_dn7 = assign29700_e42169_d_n7;
        locals.var_t0__blk897_dn10 = assign29700_e42169_d_n10;
        locals.var_t0__blk897_dn11 = assign29700_e42169_d_n11;
        locals.var_t0__blk897_dn12 = assign29700_e42169_d_n12;
        locals.var_t0__blk897_dn17 = assign29700_e42169_d_n17;
        locals.var_t0__blk897_rv = 0.0;

        let (assign29710_e42185, assign29710_e42185_d_n0, assign29710_e42185_d_n2, assign29710_e42185_d_n6, assign29710_e42185_d_n7, assign29710_e42185_d_n10, assign29710_e42185_d_n11, assign29710_e42185_d_n12, assign29710_e42185_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29710_e42183: f64 = (locals.var_t0__blk897 * locals.var_t0__blk897);
        (assign29710_e42183, ((locals.var_t0__blk897_dn0 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn0)), ((locals.var_t0__blk897_dn2 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn2)), ((locals.var_t0__blk897_dn6 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn6)), ((locals.var_t0__blk897_dn7 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn7)), ((locals.var_t0__blk897_dn10 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn10)), ((locals.var_t0__blk897_dn11 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn11)), ((locals.var_t0__blk897_dn12 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn12)), ((locals.var_t0__blk897_dn17 * locals.var_t0__blk897) + (locals.var_t0__blk897 * locals.var_t0__blk897_dn17)),)
    } else {
        (locals.var_cnst1over__blk958, locals.var_cnst1over__blk958_dn0, locals.var_cnst1over__blk958_dn2, locals.var_cnst1over__blk958_dn6, locals.var_cnst1over__blk958_dn7, locals.var_cnst1over__blk958_dn10, locals.var_cnst1over__blk958_dn11, locals.var_cnst1over__blk958_dn12, locals.var_cnst1over__blk958_dn17,)
    }
};
        locals.var_cnst1over__blk958 = assign29710_e42185;
        locals.var_cnst1over__blk958_dn0 = assign29710_e42185_d_n0;
        locals.var_cnst1over__blk958_dn2 = assign29710_e42185_d_n2;
        locals.var_cnst1over__blk958_dn6 = assign29710_e42185_d_n6;
        locals.var_cnst1over__blk958_dn7 = assign29710_e42185_d_n7;
        locals.var_cnst1over__blk958_dn10 = assign29710_e42185_d_n10;
        locals.var_cnst1over__blk958_dn11 = assign29710_e42185_d_n11;
        locals.var_cnst1over__blk958_dn12 = assign29710_e42185_d_n12;
        locals.var_cnst1over__blk958_dn17 = assign29710_e42185_d_n17;
        locals.var_cnst1over__blk958_rv = 0.0;

        let (assign29720_e42201, assign29720_e42201_d_n0, assign29720_e42201_d_n2, assign29720_e42201_d_n6, assign29720_e42201_d_n7, assign29720_e42201_d_n10, assign29720_e42201_d_n11, assign29720_e42201_d_n12, assign29720_e42201_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29720_e42199: f64 = (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964);
        (assign29720_e42199, ((locals.var_cnst1over__blk958_dn0 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn0)), ((locals.var_cnst1over__blk958_dn2 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn2)), ((locals.var_cnst1over__blk958_dn6 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn6)), ((locals.var_cnst1over__blk958_dn7 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn7)), ((locals.var_cnst1over__blk958_dn10 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn10)), ((locals.var_cnst1over__blk958_dn11 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn11)), ((locals.var_cnst1over__blk958_dn12 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn12)), ((locals.var_cnst1over__blk958_dn17 * locals.var_exp_bvbs__blk964) + (locals.var_cnst1over__blk958 * locals.var_exp_bvbs__blk964_dn17)),)
    } else {
        (locals.var_cfs1__blk973, locals.var_cfs1__blk973_dn0, locals.var_cfs1__blk973_dn2, locals.var_cfs1__blk973_dn6, locals.var_cfs1__blk973_dn7, locals.var_cfs1__blk973_dn10, locals.var_cfs1__blk973_dn11, locals.var_cfs1__blk973_dn12, locals.var_cfs1__blk973_dn17,)
    }
};
        locals.var_cfs1__blk973 = assign29720_e42201;
        locals.var_cfs1__blk973_dn0 = assign29720_e42201_d_n0;
        locals.var_cfs1__blk973_dn2 = assign29720_e42201_d_n2;
        locals.var_cfs1__blk973_dn6 = assign29720_e42201_d_n6;
        locals.var_cfs1__blk973_dn7 = assign29720_e42201_d_n7;
        locals.var_cfs1__blk973_dn10 = assign29720_e42201_d_n10;
        locals.var_cfs1__blk973_dn11 = assign29720_e42201_d_n11;
        locals.var_cfs1__blk973_dn12 = assign29720_e42201_d_n12;
        locals.var_cfs1__blk973_dn17 = assign29720_e42201_d_n17;
        locals.var_cfs1__blk973_rv = 0.0;

        let (assign29730_e42215,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv__blk920,)
    }
};
        locals.var_flg_conv__blk920 = assign29730_e42215;
        locals.var_flg_conv__blk920_rv = 0.0;

        let (assign29740_e42229, assign29740_e42229_d_n0, assign29740_e42229_d_n2, assign29740_e42229_d_n6, assign29740_e42229_d_n7, assign29740_e42229_d_n10, assign29740_e42229_d_n11, assign29740_e42229_d_n12, assign29740_e42229_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01__blk967, locals.var_fs01__blk967_dn0, locals.var_fs01__blk967_dn2, locals.var_fs01__blk967_dn6, locals.var_fs01__blk967_dn7, locals.var_fs01__blk967_dn10, locals.var_fs01__blk967_dn11, locals.var_fs01__blk967_dn12, locals.var_fs01__blk967_dn17,)
    }
};
        locals.var_fs01__blk967 = assign29740_e42229;
        locals.var_fs01__blk967_dn0 = assign29740_e42229_d_n0;
        locals.var_fs01__blk967_dn2 = assign29740_e42229_d_n2;
        locals.var_fs01__blk967_dn6 = assign29740_e42229_d_n6;
        locals.var_fs01__blk967_dn7 = assign29740_e42229_d_n7;
        locals.var_fs01__blk967_dn10 = assign29740_e42229_d_n10;
        locals.var_fs01__blk967_dn11 = assign29740_e42229_d_n11;
        locals.var_fs01__blk967_dn12 = assign29740_e42229_d_n12;
        locals.var_fs01__blk967_dn17 = assign29740_e42229_d_n17;
        locals.var_fs01__blk967_rv = 0.0;

        let (assign29750_e42243, assign29750_e42243_d_n0, assign29750_e42243_d_n2, assign29750_e42243_d_n6, assign29750_e42243_d_n7, assign29750_e42243_d_n10, assign29750_e42243_d_n11, assign29750_e42243_d_n12, assign29750_e42243_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02__blk971, locals.var_fs02__blk971_dn0, locals.var_fs02__blk971_dn2, locals.var_fs02__blk971_dn6, locals.var_fs02__blk971_dn7, locals.var_fs02__blk971_dn10, locals.var_fs02__blk971_dn11, locals.var_fs02__blk971_dn12, locals.var_fs02__blk971_dn17,)
    }
};
        locals.var_fs02__blk971 = assign29750_e42243;
        locals.var_fs02__blk971_dn0 = assign29750_e42243_d_n0;
        locals.var_fs02__blk971_dn2 = assign29750_e42243_d_n2;
        locals.var_fs02__blk971_dn6 = assign29750_e42243_d_n6;
        locals.var_fs02__blk971_dn7 = assign29750_e42243_d_n7;
        locals.var_fs02__blk971_dn10 = assign29750_e42243_d_n10;
        locals.var_fs02__blk971_dn11 = assign29750_e42243_d_n11;
        locals.var_fs02__blk971_dn12 = assign29750_e42243_d_n12;
        locals.var_fs02__blk971_dn17 = assign29750_e42243_d_n17;
        locals.var_fs02__blk971_rv = 0.0;

        let (assign29760_e42257,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign29760_e42257;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_108(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign29770_loop_guard: usize = 0;
        while {
            let assign29770_cond_e42272: f64 = (2.0 * 20.0);
            let assign29770_cond_e42274: f64 = (assign29770_cond_e42272 + 1.0);
            let assign29770_cond_e42276: f64 = if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_lp_s0 <= assign29770_cond_e42274)) { 1.0 } else { 0.0 };
            assign29770_cond_e42276 != 0.0
        } {
            assign29770_loop_guard += 1;
            assert!(assign29770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign29770_body0_e42290, assign29770_body0_e42290_d_n0, assign29770_body0_e42290_d_n2, assign29770_body0_e42290_d_n6, assign29770_body0_e42290_d_n7, assign29770_body0_e42290_d_n10, assign29770_body0_e42290_d_n11, assign29770_body0_e42290_d_n12, assign29770_body0_e42290_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb__blk969, locals.var_fb__blk969_dn0, locals.var_fb__blk969_dn2, locals.var_fb__blk969_dn6, locals.var_fb__blk969_dn7, locals.var_fb__blk969_dn10, locals.var_fb__blk969_dn11, locals.var_fb__blk969_dn12, locals.var_fb__blk969_dn17,)
    }
};
            locals.var_fb__blk969 = assign29770_body0_e42290;
            locals.var_fb__blk969_dn0 = assign29770_body0_e42290_d_n0;
            locals.var_fb__blk969_dn2 = assign29770_body0_e42290_d_n2;
            locals.var_fb__blk969_dn6 = assign29770_body0_e42290_d_n6;
            locals.var_fb__blk969_dn7 = assign29770_body0_e42290_d_n7;
            locals.var_fb__blk969_dn10 = assign29770_body0_e42290_d_n10;
            locals.var_fb__blk969_dn11 = assign29770_body0_e42290_d_n11;
            locals.var_fb__blk969_dn12 = assign29770_body0_e42290_d_n12;
            locals.var_fb__blk969_dn17 = assign29770_body0_e42290_d_n17;
            locals.var_fb__blk969_rv = 0.0;
            let (assign29770_body1_e42308, assign29770_body1_e42308_d_n0, assign29770_body1_e42308_d_n2, assign29770_body1_e42308_d_n6, assign29770_body1_e42308_d_n7, assign29770_body1_e42308_d_n10, assign29770_body1_e42308_d_n11, assign29770_body1_e42308_d_n12, assign29770_body1_e42308_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29770_body1_e42305: f64 = (locals.var_ps0ld__blk947 + locals.var_vxbgmtcl__blk923);
        let assign29770_body1_e42306: f64 = (locals.var_beta * assign29770_body1_e42305);
        (assign29770_body1_e42306, (locals.var_beta * (locals.var_ps0ld__blk947_dn0 + locals.var_vxbgmtcl__blk923_dn0)), (locals.var_beta * (locals.var_ps0ld__blk947_dn2 + locals.var_vxbgmtcl__blk923_dn2)), (locals.var_beta * (locals.var_ps0ld__blk947_dn6 + locals.var_vxbgmtcl__blk923_dn6)), (locals.var_beta * (locals.var_ps0ld__blk947_dn7 + locals.var_vxbgmtcl__blk923_dn7)), ((locals.var_beta_dn10 * assign29770_body1_e42305) + (locals.var_beta * (locals.var_ps0ld__blk947_dn10 + locals.var_vxbgmtcl__blk923_dn10))), (locals.var_beta * (locals.var_ps0ld__blk947_dn11 + locals.var_vxbgmtcl__blk923_dn11)), (locals.var_beta * (locals.var_ps0ld__blk947_dn12 + locals.var_vxbgmtcl__blk923_dn12)), (locals.var_beta * (locals.var_ps0ld__blk947_dn17 + locals.var_vxbgmtcl__blk923_dn17)),)
    } else {
        (locals.var_chi__blk945, locals.var_chi__blk945_dn0, locals.var_chi__blk945_dn2, locals.var_chi__blk945_dn6, locals.var_chi__blk945_dn7, locals.var_chi__blk945_dn10, locals.var_chi__blk945_dn11, locals.var_chi__blk945_dn12, locals.var_chi__blk945_dn17,)
    }
};
            locals.var_chi__blk945 = assign29770_body1_e42308;
            locals.var_chi__blk945_dn0 = assign29770_body1_e42308_d_n0;
            locals.var_chi__blk945_dn2 = assign29770_body1_e42308_d_n2;
            locals.var_chi__blk945_dn6 = assign29770_body1_e42308_d_n6;
            locals.var_chi__blk945_dn7 = assign29770_body1_e42308_d_n7;
            locals.var_chi__blk945_dn10 = assign29770_body1_e42308_d_n10;
            locals.var_chi__blk945_dn11 = assign29770_body1_e42308_d_n11;
            locals.var_chi__blk945_dn12 = assign29770_body1_e42308_d_n12;
            locals.var_chi__blk945_dn17 = assign29770_body1_e42308_d_n17;
            locals.var_chi__blk945_rv = 0.0;
            let assign29770_body2_e42311: f64 = if locals.var_chi__blk945 < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard992 = assign29770_body2_e42311;
            locals.var_guard992_rv = 0.0;
            let (assign29770_body3_e42342, assign29770_body3_e42342_d_n0, assign29770_body3_e42342_d_n2, assign29770_body3_e42342_d_n6, assign29770_body3_e42342_d_n7, assign29770_body3_e42342_d_n10, assign29770_body3_e42342_d_n11, assign29770_body3_e42342_d_n12, assign29770_body3_e42342_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard992 != 0.0)) {
        let assign29770_body3_e42327: f64 = (locals.var_chi__blk945 * locals.var_chi__blk945);
        let assign29770_body3_e42329: f64 = (assign29770_body3_e42327 * locals.var_chi__blk945);
        let assign29770_body3_e42333: f64 = (-0.07053654284009761);
        let assign29770_body3_e42336: f64 = (locals.var_chi__blk945 * 0.006115288895133179);
        let assign29770_body3_e42337: f64 = (assign29770_body3_e42333 + assign29770_body3_e42336);
        let assign29770_body3_e42338: f64 = (locals.var_chi__blk945 * assign29770_body3_e42337);
        let assign29770_body3_e42339: f64 = (0.29693154855771 + assign29770_body3_e42338);
        let assign29770_body3_e42340: f64 = (assign29770_body3_e42329 * assign29770_body3_e42339);
        (assign29770_body3_e42340, ((((((locals.var_chi__blk945_dn0 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn0)) * locals.var_chi__blk945) + (assign29770_body3_e42327 * locals.var_chi__blk945_dn0)) * assign29770_body3_e42339) + (assign29770_body3_e42329 * ((locals.var_chi__blk945_dn0 * assign29770_body3_e42337) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn0 * 0.006115288895133179))))), ((((((locals.var_chi__blk945_dn2 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn2)) * locals.var_chi__blk945) + (assign29770_body3_e42327 * locals.var_chi__blk945_dn2)) * assign29770_body3_e42339) + (assign29770_body3_e42329 * ((locals.var_chi__blk945_dn2 * assign29770_body3_e42337) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn2 * 0.006115288895133179))))), ((((((locals.var_chi__blk945_dn6 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn6)) * locals.var_chi__blk945) + (assign29770_body3_e42327 * locals.var_chi__blk945_dn6)) * assign29770_body3_e42339) + (assign29770_body3_e42329 * ((locals.var_chi__blk945_dn6 * assign29770_body3_e42337) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn6 * 0.006115288895133179))))), ((((((locals.var_chi__blk945_dn7 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn7)) * locals.var_chi__blk945) + (assign29770_body3_e42327 * locals.var_chi__blk945_dn7)) * assign29770_body3_e42339) + (assign29770_body3_e42329 * ((locals.var_chi__blk945_dn7 * assign29770_body3_e42337) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn7 * 0.006115288895133179))))), ((((((locals.var_chi__blk945_dn10 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn10)) * locals.var_chi__blk945) + (assign29770_body3_e42327 * locals.var_chi__blk945_dn10)) * assign29770_body3_e42339) + (assign29770_body3_e42329 * ((locals.var_chi__blk945_dn10 * assign29770_body3_e42337) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn10 * 0.006115288895133179))))), ((((((locals.var_chi__blk945_dn11 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn11)) * locals.var_chi__blk945) + (assign29770_body3_e42327 * locals.var_chi__blk945_dn11)) * assign29770_body3_e42339) + (assign29770_body3_e42329 * ((locals.var_chi__blk945_dn11 * assign29770_body3_e42337) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn11 * 0.006115288895133179))))), ((((((locals.var_chi__blk945_dn12 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn12)) * locals.var_chi__blk945) + (assign29770_body3_e42327 * locals.var_chi__blk945_dn12)) * assign29770_body3_e42339) + (assign29770_body3_e42329 * ((locals.var_chi__blk945_dn12 * assign29770_body3_e42337) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn12 * 0.006115288895133179))))), ((((((locals.var_chi__blk945_dn17 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn17)) * locals.var_chi__blk945) + (assign29770_body3_e42327 * locals.var_chi__blk945_dn17)) * assign29770_body3_e42339) + (assign29770_body3_e42329 * ((locals.var_chi__blk945_dn17 * assign29770_body3_e42337) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn17 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi__blk965, locals.var_fi__blk965_dn0, locals.var_fi__blk965_dn2, locals.var_fi__blk965_dn6, locals.var_fi__blk965_dn7, locals.var_fi__blk965_dn10, locals.var_fi__blk965_dn11, locals.var_fi__blk965_dn12, locals.var_fi__blk965_dn17,)
    }
};
            locals.var_fi__blk965 = assign29770_body3_e42342;
            locals.var_fi__blk965_dn0 = assign29770_body3_e42342_d_n0;
            locals.var_fi__blk965_dn2 = assign29770_body3_e42342_d_n2;
            locals.var_fi__blk965_dn6 = assign29770_body3_e42342_d_n6;
            locals.var_fi__blk965_dn7 = assign29770_body3_e42342_d_n7;
            locals.var_fi__blk965_dn10 = assign29770_body3_e42342_d_n10;
            locals.var_fi__blk965_dn11 = assign29770_body3_e42342_d_n11;
            locals.var_fi__blk965_dn12 = assign29770_body3_e42342_d_n12;
            locals.var_fi__blk965_dn17 = assign29770_body3_e42342_d_n17;
            locals.var_fi__blk965_rv = 0.0;
            let (assign29770_body4_e42377, assign29770_body4_e42377_d_n0, assign29770_body4_e42377_d_n2, assign29770_body4_e42377_d_n6, assign29770_body4_e42377_d_n7, assign29770_body4_e42377_d_n10, assign29770_body4_e42377_d_n11, assign29770_body4_e42377_d_n12, assign29770_body4_e42377_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard992 != 0.0)) {
        let assign29770_body4_e42358: f64 = (locals.var_chi__blk945 * locals.var_chi__blk945);
        let assign29770_body4_e42361: f64 = (3.0 * 0.29693154855771);
        let assign29770_body4_e42365: f64 = (-0.07053654284009761);
        let assign29770_body4_e42366: f64 = (4.0 * assign29770_body4_e42365);
        let assign29770_body4_e42369: f64 = (locals.var_chi__blk945 * 5.0);
        let assign29770_body4_e42371: f64 = (assign29770_body4_e42369 * 0.006115288895133179);
        let assign29770_body4_e42372: f64 = (assign29770_body4_e42366 + assign29770_body4_e42371);
        let assign29770_body4_e42373: f64 = (locals.var_chi__blk945 * assign29770_body4_e42372);
        let assign29770_body4_e42374: f64 = (assign29770_body4_e42361 + assign29770_body4_e42373);
        let assign29770_body4_e42375: f64 = (assign29770_body4_e42358 * assign29770_body4_e42374);
        (assign29770_body4_e42375, ((((locals.var_chi__blk945_dn0 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn0)) * assign29770_body4_e42374) + (assign29770_body4_e42358 * ((locals.var_chi__blk945_dn0 * assign29770_body4_e42372) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk945_dn2 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn2)) * assign29770_body4_e42374) + (assign29770_body4_e42358 * ((locals.var_chi__blk945_dn2 * assign29770_body4_e42372) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk945_dn6 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn6)) * assign29770_body4_e42374) + (assign29770_body4_e42358 * ((locals.var_chi__blk945_dn6 * assign29770_body4_e42372) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk945_dn7 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn7)) * assign29770_body4_e42374) + (assign29770_body4_e42358 * ((locals.var_chi__blk945_dn7 * assign29770_body4_e42372) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk945_dn10 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn10)) * assign29770_body4_e42374) + (assign29770_body4_e42358 * ((locals.var_chi__blk945_dn10 * assign29770_body4_e42372) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk945_dn11 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn11)) * assign29770_body4_e42374) + (assign29770_body4_e42358 * ((locals.var_chi__blk945_dn11 * assign29770_body4_e42372) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk945_dn12 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn12)) * assign29770_body4_e42374) + (assign29770_body4_e42358 * ((locals.var_chi__blk945_dn12 * assign29770_body4_e42372) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn12 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk945_dn17 * locals.var_chi__blk945) + (locals.var_chi__blk945 * locals.var_chi__blk945_dn17)) * assign29770_body4_e42374) + (assign29770_body4_e42358 * ((locals.var_chi__blk945_dn17 * assign29770_body4_e42372) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn17 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi__blk966, locals.var_fi_dchi__blk966_dn0, locals.var_fi_dchi__blk966_dn2, locals.var_fi_dchi__blk966_dn6, locals.var_fi_dchi__blk966_dn7, locals.var_fi_dchi__blk966_dn10, locals.var_fi_dchi__blk966_dn11, locals.var_fi_dchi__blk966_dn12, locals.var_fi_dchi__blk966_dn17,)
    }
};
            locals.var_fi_dchi__blk966 = assign29770_body4_e42377;
            locals.var_fi_dchi__blk966_dn0 = assign29770_body4_e42377_d_n0;
            locals.var_fi_dchi__blk966_dn2 = assign29770_body4_e42377_d_n2;
            locals.var_fi_dchi__blk966_dn6 = assign29770_body4_e42377_d_n6;
            locals.var_fi_dchi__blk966_dn7 = assign29770_body4_e42377_d_n7;
            locals.var_fi_dchi__blk966_dn10 = assign29770_body4_e42377_d_n10;
            locals.var_fi_dchi__blk966_dn11 = assign29770_body4_e42377_d_n11;
            locals.var_fi_dchi__blk966_dn12 = assign29770_body4_e42377_d_n12;
            locals.var_fi_dchi__blk966_dn17 = assign29770_body4_e42377_d_n17;
            locals.var_fi_dchi__blk966_rv = 0.0;
            let (assign29770_body5_e42397, assign29770_body5_e42397_d_n0, assign29770_body5_e42397_d_n2, assign29770_body5_e42397_d_n6, assign29770_body5_e42397_d_n7, assign29770_body5_e42397_d_n10, assign29770_body5_e42397_d_n11, assign29770_body5_e42397_d_n12, assign29770_body5_e42397_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard992 != 0.0)) {
        let assign29770_body5_e42393: f64 = (locals.var_cfs1__blk973 * locals.var_fi__blk965);
        let assign29770_body5_e42395: f64 = (assign29770_body5_e42393 * locals.var_fi__blk965);
        (assign29770_body5_e42395, ((((locals.var_cfs1__blk973_dn0 * locals.var_fi__blk965) + (locals.var_cfs1__blk973 * locals.var_fi__blk965_dn0)) * locals.var_fi__blk965) + (assign29770_body5_e42393 * locals.var_fi__blk965_dn0)), ((((locals.var_cfs1__blk973_dn2 * locals.var_fi__blk965) + (locals.var_cfs1__blk973 * locals.var_fi__blk965_dn2)) * locals.var_fi__blk965) + (assign29770_body5_e42393 * locals.var_fi__blk965_dn2)), ((((locals.var_cfs1__blk973_dn6 * locals.var_fi__blk965) + (locals.var_cfs1__blk973 * locals.var_fi__blk965_dn6)) * locals.var_fi__blk965) + (assign29770_body5_e42393 * locals.var_fi__blk965_dn6)), ((((locals.var_cfs1__blk973_dn7 * locals.var_fi__blk965) + (locals.var_cfs1__blk973 * locals.var_fi__blk965_dn7)) * locals.var_fi__blk965) + (assign29770_body5_e42393 * locals.var_fi__blk965_dn7)), ((((locals.var_cfs1__blk973_dn10 * locals.var_fi__blk965) + (locals.var_cfs1__blk973 * locals.var_fi__blk965_dn10)) * locals.var_fi__blk965) + (assign29770_body5_e42393 * locals.var_fi__blk965_dn10)), ((((locals.var_cfs1__blk973_dn11 * locals.var_fi__blk965) + (locals.var_cfs1__blk973 * locals.var_fi__blk965_dn11)) * locals.var_fi__blk965) + (assign29770_body5_e42393 * locals.var_fi__blk965_dn11)), ((((locals.var_cfs1__blk973_dn12 * locals.var_fi__blk965) + (locals.var_cfs1__blk973 * locals.var_fi__blk965_dn12)) * locals.var_fi__blk965) + (assign29770_body5_e42393 * locals.var_fi__blk965_dn12)), ((((locals.var_cfs1__blk973_dn17 * locals.var_fi__blk965) + (locals.var_cfs1__blk973 * locals.var_fi__blk965_dn17)) * locals.var_fi__blk965) + (assign29770_body5_e42393 * locals.var_fi__blk965_dn17)),)
    } else {
        (locals.var_fs01__blk967, locals.var_fs01__blk967_dn0, locals.var_fs01__blk967_dn2, locals.var_fs01__blk967_dn6, locals.var_fs01__blk967_dn7, locals.var_fs01__blk967_dn10, locals.var_fs01__blk967_dn11, locals.var_fs01__blk967_dn12, locals.var_fs01__blk967_dn17,)
    }
};
            locals.var_fs01__blk967 = assign29770_body5_e42397;
            locals.var_fs01__blk967_dn0 = assign29770_body5_e42397_d_n0;
            locals.var_fs01__blk967_dn2 = assign29770_body5_e42397_d_n2;
            locals.var_fs01__blk967_dn6 = assign29770_body5_e42397_d_n6;
            locals.var_fs01__blk967_dn7 = assign29770_body5_e42397_d_n7;
            locals.var_fs01__blk967_dn10 = assign29770_body5_e42397_d_n10;
            locals.var_fs01__blk967_dn11 = assign29770_body5_e42397_d_n11;
            locals.var_fs01__blk967_dn12 = assign29770_body5_e42397_d_n12;
            locals.var_fs01__blk967_dn17 = assign29770_body5_e42397_d_n17;
            locals.var_fs01__blk967_rv = 0.0;
            let (assign29770_body6_e42421, assign29770_body6_e42421_d_n0, assign29770_body6_e42421_d_n2, assign29770_body6_e42421_d_n6, assign29770_body6_e42421_d_n7, assign29770_body6_e42421_d_n10, assign29770_body6_e42421_d_n11, assign29770_body6_e42421_d_n12, assign29770_body6_e42421_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard992 != 0.0)) {
        let assign29770_body6_e42413: f64 = (locals.var_cfs1__blk973 * locals.var_beta);
        let assign29770_body6_e42415: f64 = (assign29770_body6_e42413 * 2.0);
        let assign29770_body6_e42417: f64 = (assign29770_body6_e42415 * locals.var_fi__blk965);
        let assign29770_body6_e42419: f64 = (assign29770_body6_e42417 * locals.var_fi_dchi__blk966);
        (assign29770_body6_e42419, ((((((locals.var_cfs1__blk973_dn0 * locals.var_beta) * 2.0) * locals.var_fi__blk965) + (assign29770_body6_e42415 * locals.var_fi__blk965_dn0)) * locals.var_fi_dchi__blk966) + (assign29770_body6_e42417 * locals.var_fi_dchi__blk966_dn0)), ((((((locals.var_cfs1__blk973_dn2 * locals.var_beta) * 2.0) * locals.var_fi__blk965) + (assign29770_body6_e42415 * locals.var_fi__blk965_dn2)) * locals.var_fi_dchi__blk966) + (assign29770_body6_e42417 * locals.var_fi_dchi__blk966_dn2)), ((((((locals.var_cfs1__blk973_dn6 * locals.var_beta) * 2.0) * locals.var_fi__blk965) + (assign29770_body6_e42415 * locals.var_fi__blk965_dn6)) * locals.var_fi_dchi__blk966) + (assign29770_body6_e42417 * locals.var_fi_dchi__blk966_dn6)), ((((((locals.var_cfs1__blk973_dn7 * locals.var_beta) * 2.0) * locals.var_fi__blk965) + (assign29770_body6_e42415 * locals.var_fi__blk965_dn7)) * locals.var_fi_dchi__blk966) + (assign29770_body6_e42417 * locals.var_fi_dchi__blk966_dn7)), (((((((locals.var_cfs1__blk973_dn10 * locals.var_beta) + (locals.var_cfs1__blk973 * locals.var_beta_dn10)) * 2.0) * locals.var_fi__blk965) + (assign29770_body6_e42415 * locals.var_fi__blk965_dn10)) * locals.var_fi_dchi__blk966) + (assign29770_body6_e42417 * locals.var_fi_dchi__blk966_dn10)), ((((((locals.var_cfs1__blk973_dn11 * locals.var_beta) * 2.0) * locals.var_fi__blk965) + (assign29770_body6_e42415 * locals.var_fi__blk965_dn11)) * locals.var_fi_dchi__blk966) + (assign29770_body6_e42417 * locals.var_fi_dchi__blk966_dn11)), ((((((locals.var_cfs1__blk973_dn12 * locals.var_beta) * 2.0) * locals.var_fi__blk965) + (assign29770_body6_e42415 * locals.var_fi__blk965_dn12)) * locals.var_fi_dchi__blk966) + (assign29770_body6_e42417 * locals.var_fi_dchi__blk966_dn12)), ((((((locals.var_cfs1__blk973_dn17 * locals.var_beta) * 2.0) * locals.var_fi__blk965) + (assign29770_body6_e42415 * locals.var_fi__blk965_dn17)) * locals.var_fi_dchi__blk966) + (assign29770_body6_e42417 * locals.var_fi_dchi__blk966_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk968, locals.var_fs01_dps0__blk968_dn0, locals.var_fs01_dps0__blk968_dn2, locals.var_fs01_dps0__blk968_dn6, locals.var_fs01_dps0__blk968_dn7, locals.var_fs01_dps0__blk968_dn10, locals.var_fs01_dps0__blk968_dn11, locals.var_fs01_dps0__blk968_dn12, locals.var_fs01_dps0__blk968_dn17,)
    }
};
            locals.var_fs01_dps0__blk968 = assign29770_body6_e42421;
            locals.var_fs01_dps0__blk968_dn0 = assign29770_body6_e42421_d_n0;
            locals.var_fs01_dps0__blk968_dn2 = assign29770_body6_e42421_d_n2;
            locals.var_fs01_dps0__blk968_dn6 = assign29770_body6_e42421_d_n6;
            locals.var_fs01_dps0__blk968_dn7 = assign29770_body6_e42421_d_n7;
            locals.var_fs01_dps0__blk968_dn10 = assign29770_body6_e42421_d_n10;
            locals.var_fs01_dps0__blk968_dn11 = assign29770_body6_e42421_d_n11;
            locals.var_fs01_dps0__blk968_dn12 = assign29770_body6_e42421_d_n12;
            locals.var_fs01_dps0__blk968_dn17 = assign29770_body6_e42421_d_n17;
            locals.var_fs01_dps0__blk968_rv = 0.0;
            let (assign29770_body7_e42457, assign29770_body7_e42457_d_n0, assign29770_body7_e42457_d_n2, assign29770_body7_e42457_d_n6, assign29770_body7_e42457_d_n7, assign29770_body7_e42457_d_n10, assign29770_body7_e42457_d_n11, assign29770_body7_e42457_d_n12, assign29770_body7_e42457_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard992 != 0.0)) {
        let assign29770_body7_e42439: f64 = (-0.117851130197758);
        let assign29770_body7_e42444: f64 = (-0.00163730162779191);
        let assign29770_body7_e42447: f64 = (locals.var_chi__blk945 * 6.36964918866352e-5);
        let assign29770_body7_e42448: f64 = (assign29770_body7_e42444 + assign29770_body7_e42447);
        let assign29770_body7_e42449: f64 = (locals.var_chi__blk945 * assign29770_body7_e42448);
        let assign29770_body7_e42450: f64 = (0.0178800506338833 + assign29770_body7_e42449);
        let assign29770_body7_e42451: f64 = (locals.var_chi__blk945 * assign29770_body7_e42450);
        let assign29770_body7_e42452: f64 = (assign29770_body7_e42439 + assign29770_body7_e42451);
        let assign29770_body7_e42453: f64 = (locals.var_chi__blk945 * assign29770_body7_e42452);
        let assign29770_body7_e42454: f64 = (0.707106781186548 + assign29770_body7_e42453);
        let assign29770_body7_e42455: f64 = (locals.var_chi__blk945 * assign29770_body7_e42454);
        (assign29770_body7_e42455, ((locals.var_chi__blk945_dn0 * assign29770_body7_e42454) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn0 * assign29770_body7_e42452) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn0 * assign29770_body7_e42450) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn0 * assign29770_body7_e42448) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk945_dn2 * assign29770_body7_e42454) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn2 * assign29770_body7_e42452) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn2 * assign29770_body7_e42450) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn2 * assign29770_body7_e42448) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk945_dn6 * assign29770_body7_e42454) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn6 * assign29770_body7_e42452) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn6 * assign29770_body7_e42450) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn6 * assign29770_body7_e42448) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk945_dn7 * assign29770_body7_e42454) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn7 * assign29770_body7_e42452) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn7 * assign29770_body7_e42450) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn7 * assign29770_body7_e42448) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk945_dn10 * assign29770_body7_e42454) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn10 * assign29770_body7_e42452) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn10 * assign29770_body7_e42450) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn10 * assign29770_body7_e42448) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk945_dn11 * assign29770_body7_e42454) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn11 * assign29770_body7_e42452) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn11 * assign29770_body7_e42450) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn11 * assign29770_body7_e42448) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk945_dn12 * assign29770_body7_e42454) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn12 * assign29770_body7_e42452) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn12 * assign29770_body7_e42450) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn12 * assign29770_body7_e42448) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn12 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk945_dn17 * assign29770_body7_e42454) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn17 * assign29770_body7_e42452) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn17 * assign29770_body7_e42450) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn17 * assign29770_body7_e42448) + (locals.var_chi__blk945 * (locals.var_chi__blk945_dn17 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb__blk969, locals.var_fb__blk969_dn0, locals.var_fb__blk969_dn2, locals.var_fb__blk969_dn6, locals.var_fb__blk969_dn7, locals.var_fb__blk969_dn10, locals.var_fb__blk969_dn11, locals.var_fb__blk969_dn12, locals.var_fb__blk969_dn17,)
    }
};
            locals.var_fb__blk969 = assign29770_body7_e42457;
            locals.var_fb__blk969_dn0 = assign29770_body7_e42457_d_n0;
            locals.var_fb__blk969_dn2 = assign29770_body7_e42457_d_n2;
            locals.var_fb__blk969_dn6 = assign29770_body7_e42457_d_n6;
            locals.var_fb__blk969_dn7 = assign29770_body7_e42457_d_n7;
            locals.var_fb__blk969_dn10 = assign29770_body7_e42457_d_n10;
            locals.var_fb__blk969_dn11 = assign29770_body7_e42457_d_n11;
            locals.var_fb__blk969_dn12 = assign29770_body7_e42457_d_n12;
            locals.var_fb__blk969_dn17 = assign29770_body7_e42457_d_n17;
            locals.var_fb__blk969_rv = 0.0;
            let (assign29770_body8_e42499, assign29770_body8_e42499_d_n0, assign29770_body8_e42499_d_n2, assign29770_body8_e42499_d_n6, assign29770_body8_e42499_d_n7, assign29770_body8_e42499_d_n10, assign29770_body8_e42499_d_n11, assign29770_body8_e42499_d_n12, assign29770_body8_e42499_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard992 != 0.0)) {
        let assign29770_body8_e42475: f64 = (-0.117851130197758);
        let assign29770_body8_e42476: f64 = (2.0 * assign29770_body8_e42475);
        let assign29770_body8_e42480: f64 = (3.0 * 0.0178800506338833);
        let assign29770_body8_e42484: f64 = (-0.00163730162779191);
        let assign29770_body8_e42485: f64 = (4.0 * assign29770_body8_e42484);
        let assign29770_body8_e42488: f64 = (locals.var_chi__blk945 * 5.0);
        let assign29770_body8_e42490: f64 = (assign29770_body8_e42488 * 6.36964918866352e-5);
        let assign29770_body8_e42491: f64 = (assign29770_body8_e42485 + assign29770_body8_e42490);
        let assign29770_body8_e42492: f64 = (locals.var_chi__blk945 * assign29770_body8_e42491);
        let assign29770_body8_e42493: f64 = (assign29770_body8_e42480 + assign29770_body8_e42492);
        let assign29770_body8_e42494: f64 = (locals.var_chi__blk945 * assign29770_body8_e42493);
        let assign29770_body8_e42495: f64 = (assign29770_body8_e42476 + assign29770_body8_e42494);
        let assign29770_body8_e42496: f64 = (locals.var_chi__blk945 * assign29770_body8_e42495);
        let assign29770_body8_e42497: f64 = (0.707106781186548 + assign29770_body8_e42496);
        (assign29770_body8_e42497, ((locals.var_chi__blk945_dn0 * assign29770_body8_e42495) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn0 * assign29770_body8_e42493) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn0 * assign29770_body8_e42491) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk945_dn2 * assign29770_body8_e42495) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn2 * assign29770_body8_e42493) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn2 * assign29770_body8_e42491) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk945_dn6 * assign29770_body8_e42495) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn6 * assign29770_body8_e42493) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn6 * assign29770_body8_e42491) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk945_dn7 * assign29770_body8_e42495) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn7 * assign29770_body8_e42493) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn7 * assign29770_body8_e42491) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk945_dn10 * assign29770_body8_e42495) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn10 * assign29770_body8_e42493) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn10 * assign29770_body8_e42491) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk945_dn11 * assign29770_body8_e42495) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn11 * assign29770_body8_e42493) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn11 * assign29770_body8_e42491) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk945_dn12 * assign29770_body8_e42495) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn12 * assign29770_body8_e42493) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn12 * assign29770_body8_e42491) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn12 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk945_dn17 * assign29770_body8_e42495) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn17 * assign29770_body8_e42493) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn17 * assign29770_body8_e42491) + (locals.var_chi__blk945 * ((locals.var_chi__blk945_dn17 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi__blk970, locals.var_fb_dchi__blk970_dn0, locals.var_fb_dchi__blk970_dn2, locals.var_fb_dchi__blk970_dn6, locals.var_fb_dchi__blk970_dn7, locals.var_fb_dchi__blk970_dn10, locals.var_fb_dchi__blk970_dn11, locals.var_fb_dchi__blk970_dn12, locals.var_fb_dchi__blk970_dn17,)
    }
};
            locals.var_fb_dchi__blk970 = assign29770_body8_e42499;
            locals.var_fb_dchi__blk970_dn0 = assign29770_body8_e42499_d_n0;
            locals.var_fb_dchi__blk970_dn2 = assign29770_body8_e42499_d_n2;
            locals.var_fb_dchi__blk970_dn6 = assign29770_body8_e42499_d_n6;
            locals.var_fb_dchi__blk970_dn7 = assign29770_body8_e42499_d_n7;
            locals.var_fb_dchi__blk970_dn10 = assign29770_body8_e42499_d_n10;
            locals.var_fb_dchi__blk970_dn11 = assign29770_body8_e42499_d_n11;
            locals.var_fb_dchi__blk970_dn12 = assign29770_body8_e42499_d_n12;
            locals.var_fb_dchi__blk970_dn17 = assign29770_body8_e42499_d_n17;
            locals.var_fb_dchi__blk970_rv = 0.0;
            let (assign29770_body9_e42522, assign29770_body9_e42522_d_n0, assign29770_body9_e42522_d_n2, assign29770_body9_e42522_d_n6, assign29770_body9_e42522_d_n7, assign29770_body9_e42522_d_n10, assign29770_body9_e42522_d_n11, assign29770_body9_e42522_d_n12, assign29770_body9_e42522_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard992 != 0.0)) {
        let assign29770_body9_e42515: f64 = (locals.var_fb__blk969 * locals.var_fb__blk969);
        let assign29770_body9_e42517: f64 = (assign29770_body9_e42515 + locals.var_fs01__blk967);
        let assign29770_body9_e42519: f64 = (assign29770_body9_e42517 + 1e-50);
        let assign29770_body9_e42520: f64 = (assign29770_body9_e42519).sqrt();
        (assign29770_body9_e42520, ((((locals.var_fb__blk969_dn0 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn0)) + locals.var_fs01__blk967_dn0) / (2.0 * assign29770_body9_e42520)), ((((locals.var_fb__blk969_dn2 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn2)) + locals.var_fs01__blk967_dn2) / (2.0 * assign29770_body9_e42520)), ((((locals.var_fb__blk969_dn6 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn6)) + locals.var_fs01__blk967_dn6) / (2.0 * assign29770_body9_e42520)), ((((locals.var_fb__blk969_dn7 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn7)) + locals.var_fs01__blk967_dn7) / (2.0 * assign29770_body9_e42520)), ((((locals.var_fb__blk969_dn10 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn10)) + locals.var_fs01__blk967_dn10) / (2.0 * assign29770_body9_e42520)), ((((locals.var_fb__blk969_dn11 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn11)) + locals.var_fs01__blk967_dn11) / (2.0 * assign29770_body9_e42520)), ((((locals.var_fb__blk969_dn12 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn12)) + locals.var_fs01__blk967_dn12) / (2.0 * assign29770_body9_e42520)), ((((locals.var_fb__blk969_dn17 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn17)) + locals.var_fs01__blk967_dn17) / (2.0 * assign29770_body9_e42520)),)
    } else {
        (locals.var_fs02__blk971, locals.var_fs02__blk971_dn0, locals.var_fs02__blk971_dn2, locals.var_fs02__blk971_dn6, locals.var_fs02__blk971_dn7, locals.var_fs02__blk971_dn10, locals.var_fs02__blk971_dn11, locals.var_fs02__blk971_dn12, locals.var_fs02__blk971_dn17,)
    }
};
            locals.var_fs02__blk971 = assign29770_body9_e42522;
            locals.var_fs02__blk971_dn0 = assign29770_body9_e42522_d_n0;
            locals.var_fs02__blk971_dn2 = assign29770_body9_e42522_d_n2;
            locals.var_fs02__blk971_dn6 = assign29770_body9_e42522_d_n6;
            locals.var_fs02__blk971_dn7 = assign29770_body9_e42522_d_n7;
            locals.var_fs02__blk971_dn10 = assign29770_body9_e42522_d_n10;
            locals.var_fs02__blk971_dn11 = assign29770_body9_e42522_d_n11;
            locals.var_fs02__blk971_dn12 = assign29770_body9_e42522_d_n12;
            locals.var_fs02__blk971_dn17 = assign29770_body9_e42522_d_n17;
            locals.var_fs02__blk971_rv = 0.0;
            let (assign29770_body10_e42550, assign29770_body10_e42550_d_n0, assign29770_body10_e42550_d_n2, assign29770_body10_e42550_d_n6, assign29770_body10_e42550_d_n7, assign29770_body10_e42550_d_n10, assign29770_body10_e42550_d_n11, assign29770_body10_e42550_d_n12, assign29770_body10_e42550_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard992 != 0.0)) {
        let assign29770_body10_e42538: f64 = (locals.var_beta * locals.var_fb_dchi__blk970);
        let assign29770_body10_e42540: f64 = (assign29770_body10_e42538 * 2.0);
        let assign29770_body10_e42542: f64 = (assign29770_body10_e42540 * locals.var_fb__blk969);
        let assign29770_body10_e42544: f64 = (assign29770_body10_e42542 + locals.var_fs01_dps0__blk968);
        let assign29770_body10_e42547: f64 = (locals.var_fs02__blk971 + locals.var_fs02__blk971);
        let assign29770_body10_e42548: f64 = (assign29770_body10_e42544 / assign29770_body10_e42547);
        (assign29770_body10_e42548, ((((((((locals.var_beta * locals.var_fb_dchi__blk970_dn0) * 2.0) * locals.var_fb__blk969) + (assign29770_body10_e42540 * locals.var_fb__blk969_dn0)) + locals.var_fs01_dps0__blk968_dn0) * assign29770_body10_e42547) - (assign29770_body10_e42544 * (locals.var_fs02__blk971_dn0 + locals.var_fs02__blk971_dn0))) / (assign29770_body10_e42547 * assign29770_body10_e42547)), ((((((((locals.var_beta * locals.var_fb_dchi__blk970_dn2) * 2.0) * locals.var_fb__blk969) + (assign29770_body10_e42540 * locals.var_fb__blk969_dn2)) + locals.var_fs01_dps0__blk968_dn2) * assign29770_body10_e42547) - (assign29770_body10_e42544 * (locals.var_fs02__blk971_dn2 + locals.var_fs02__blk971_dn2))) / (assign29770_body10_e42547 * assign29770_body10_e42547)), ((((((((locals.var_beta * locals.var_fb_dchi__blk970_dn6) * 2.0) * locals.var_fb__blk969) + (assign29770_body10_e42540 * locals.var_fb__blk969_dn6)) + locals.var_fs01_dps0__blk968_dn6) * assign29770_body10_e42547) - (assign29770_body10_e42544 * (locals.var_fs02__blk971_dn6 + locals.var_fs02__blk971_dn6))) / (assign29770_body10_e42547 * assign29770_body10_e42547)), ((((((((locals.var_beta * locals.var_fb_dchi__blk970_dn7) * 2.0) * locals.var_fb__blk969) + (assign29770_body10_e42540 * locals.var_fb__blk969_dn7)) + locals.var_fs01_dps0__blk968_dn7) * assign29770_body10_e42547) - (assign29770_body10_e42544 * (locals.var_fs02__blk971_dn7 + locals.var_fs02__blk971_dn7))) / (assign29770_body10_e42547 * assign29770_body10_e42547)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi__blk970) + (locals.var_beta * locals.var_fb_dchi__blk970_dn10)) * 2.0) * locals.var_fb__blk969) + (assign29770_body10_e42540 * locals.var_fb__blk969_dn10)) + locals.var_fs01_dps0__blk968_dn10) * assign29770_body10_e42547) - (assign29770_body10_e42544 * (locals.var_fs02__blk971_dn10 + locals.var_fs02__blk971_dn10))) / (assign29770_body10_e42547 * assign29770_body10_e42547)), ((((((((locals.var_beta * locals.var_fb_dchi__blk970_dn11) * 2.0) * locals.var_fb__blk969) + (assign29770_body10_e42540 * locals.var_fb__blk969_dn11)) + locals.var_fs01_dps0__blk968_dn11) * assign29770_body10_e42547) - (assign29770_body10_e42544 * (locals.var_fs02__blk971_dn11 + locals.var_fs02__blk971_dn11))) / (assign29770_body10_e42547 * assign29770_body10_e42547)), ((((((((locals.var_beta * locals.var_fb_dchi__blk970_dn12) * 2.0) * locals.var_fb__blk969) + (assign29770_body10_e42540 * locals.var_fb__blk969_dn12)) + locals.var_fs01_dps0__blk968_dn12) * assign29770_body10_e42547) - (assign29770_body10_e42544 * (locals.var_fs02__blk971_dn12 + locals.var_fs02__blk971_dn12))) / (assign29770_body10_e42547 * assign29770_body10_e42547)), ((((((((locals.var_beta * locals.var_fb_dchi__blk970_dn17) * 2.0) * locals.var_fb__blk969) + (assign29770_body10_e42540 * locals.var_fb__blk969_dn17)) + locals.var_fs01_dps0__blk968_dn17) * assign29770_body10_e42547) - (assign29770_body10_e42544 * (locals.var_fs02__blk971_dn17 + locals.var_fs02__blk971_dn17))) / (assign29770_body10_e42547 * assign29770_body10_e42547)),)
    } else {
        (locals.var_fs02_dps0__blk972, locals.var_fs02_dps0__blk972_dn0, locals.var_fs02_dps0__blk972_dn2, locals.var_fs02_dps0__blk972_dn6, locals.var_fs02_dps0__blk972_dn7, locals.var_fs02_dps0__blk972_dn10, locals.var_fs02_dps0__blk972_dn11, locals.var_fs02_dps0__blk972_dn12, locals.var_fs02_dps0__blk972_dn17,)
    }
};
            locals.var_fs02_dps0__blk972 = assign29770_body10_e42550;
            locals.var_fs02_dps0__blk972_dn0 = assign29770_body10_e42550_d_n0;
            locals.var_fs02_dps0__blk972_dn2 = assign29770_body10_e42550_d_n2;
            locals.var_fs02_dps0__blk972_dn6 = assign29770_body10_e42550_d_n6;
            locals.var_fs02_dps0__blk972_dn7 = assign29770_body10_e42550_d_n7;
            locals.var_fs02_dps0__blk972_dn10 = assign29770_body10_e42550_d_n10;
            locals.var_fs02_dps0__blk972_dn11 = assign29770_body10_e42550_d_n11;
            locals.var_fs02_dps0__blk972_dn12 = assign29770_body10_e42550_d_n12;
            locals.var_fs02_dps0__blk972_dn17 = assign29770_body10_e42550_d_n17;
            locals.var_fs02_dps0__blk972_rv = 0.0;
            let assign29770_body11_e42553: f64 = if locals.var_chi__blk945 < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard993 = assign29770_body11_e42553;
            locals.var_guard993_rv = 0.0;
            let (assign29770_body12_e42573, assign29770_body12_e42573_d_n0, assign29770_body12_e42573_d_n2, assign29770_body12_e42573_d_n6, assign29770_body12_e42573_d_n7, assign29770_body12_e42573_d_n10, assign29770_body12_e42573_d_n11, assign29770_body12_e42573_d_n12, assign29770_body12_e42573_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard992 == 0.0)) && (locals.var_guard993 != 0.0)) {
        let assign29770_body12_e42571: f64 = (locals.var_chi__blk945).exp();
        (assign29770_body12_e42571, (assign29770_body12_e42571 * locals.var_chi__blk945_dn0), (assign29770_body12_e42571 * locals.var_chi__blk945_dn2), (assign29770_body12_e42571 * locals.var_chi__blk945_dn6), (assign29770_body12_e42571 * locals.var_chi__blk945_dn7), (assign29770_body12_e42571 * locals.var_chi__blk945_dn10), (assign29770_body12_e42571 * locals.var_chi__blk945_dn11), (assign29770_body12_e42571 * locals.var_chi__blk945_dn12), (assign29770_body12_e42571 * locals.var_chi__blk945_dn17),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17,)
    }
};
            locals.var_exp_chi = assign29770_body12_e42573;
            locals.var_exp_chi_dn0 = assign29770_body12_e42573_d_n0;
            locals.var_exp_chi_dn2 = assign29770_body12_e42573_d_n2;
            locals.var_exp_chi_dn6 = assign29770_body12_e42573_d_n6;
            locals.var_exp_chi_dn7 = assign29770_body12_e42573_d_n7;
            locals.var_exp_chi_dn10 = assign29770_body12_e42573_d_n10;
            locals.var_exp_chi_dn11 = assign29770_body12_e42573_d_n11;
            locals.var_exp_chi_dn12 = assign29770_body12_e42573_d_n12;
            locals.var_exp_chi_dn17 = assign29770_body12_e42573_d_n17;
            locals.var_exp_chi_rv = 0.0;
            let (assign29770_body13_e42596, assign29770_body13_e42596_d_n0, assign29770_body13_e42596_d_n2, assign29770_body13_e42596_d_n6, assign29770_body13_e42596_d_n7, assign29770_body13_e42596_d_n10, assign29770_body13_e42596_d_n11, assign29770_body13_e42596_d_n12, assign29770_body13_e42596_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard992 == 0.0)) && (locals.var_guard993 != 0.0)) {
        let assign29770_body13_e42593: f64 = (locals.var_exp_chi - 1.0);
        let assign29770_body13_e42594: f64 = (locals.var_cfs1__blk973 * assign29770_body13_e42593);
        (assign29770_body13_e42594, ((locals.var_cfs1__blk973_dn0 * assign29770_body13_e42593) + (locals.var_cfs1__blk973 * locals.var_exp_chi_dn0)), ((locals.var_cfs1__blk973_dn2 * assign29770_body13_e42593) + (locals.var_cfs1__blk973 * locals.var_exp_chi_dn2)), ((locals.var_cfs1__blk973_dn6 * assign29770_body13_e42593) + (locals.var_cfs1__blk973 * locals.var_exp_chi_dn6)), ((locals.var_cfs1__blk973_dn7 * assign29770_body13_e42593) + (locals.var_cfs1__blk973 * locals.var_exp_chi_dn7)), ((locals.var_cfs1__blk973_dn10 * assign29770_body13_e42593) + (locals.var_cfs1__blk973 * locals.var_exp_chi_dn10)), ((locals.var_cfs1__blk973_dn11 * assign29770_body13_e42593) + (locals.var_cfs1__blk973 * locals.var_exp_chi_dn11)), ((locals.var_cfs1__blk973_dn12 * assign29770_body13_e42593) + (locals.var_cfs1__blk973 * locals.var_exp_chi_dn12)), ((locals.var_cfs1__blk973_dn17 * assign29770_body13_e42593) + (locals.var_cfs1__blk973 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01__blk967, locals.var_fs01__blk967_dn0, locals.var_fs01__blk967_dn2, locals.var_fs01__blk967_dn6, locals.var_fs01__blk967_dn7, locals.var_fs01__blk967_dn10, locals.var_fs01__blk967_dn11, locals.var_fs01__blk967_dn12, locals.var_fs01__blk967_dn17,)
    }
};
            locals.var_fs01__blk967 = assign29770_body13_e42596;
            locals.var_fs01__blk967_dn0 = assign29770_body13_e42596_d_n0;
            locals.var_fs01__blk967_dn2 = assign29770_body13_e42596_d_n2;
            locals.var_fs01__blk967_dn6 = assign29770_body13_e42596_d_n6;
            locals.var_fs01__blk967_dn7 = assign29770_body13_e42596_d_n7;
            locals.var_fs01__blk967_dn10 = assign29770_body13_e42596_d_n10;
            locals.var_fs01__blk967_dn11 = assign29770_body13_e42596_d_n11;
            locals.var_fs01__blk967_dn12 = assign29770_body13_e42596_d_n12;
            locals.var_fs01__blk967_dn17 = assign29770_body13_e42596_d_n17;
            locals.var_fs01__blk967_rv = 0.0;
            let (assign29770_body14_e42619, assign29770_body14_e42619_d_n0, assign29770_body14_e42619_d_n2, assign29770_body14_e42619_d_n6, assign29770_body14_e42619_d_n7, assign29770_body14_e42619_d_n10, assign29770_body14_e42619_d_n11, assign29770_body14_e42619_d_n12, assign29770_body14_e42619_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard992 == 0.0)) && (locals.var_guard993 != 0.0)) {
        let assign29770_body14_e42615: f64 = (locals.var_cfs1__blk973 * locals.var_beta);
        let assign29770_body14_e42617: f64 = (assign29770_body14_e42615 * locals.var_exp_chi);
        (assign29770_body14_e42617, (((locals.var_cfs1__blk973_dn0 * locals.var_beta) * locals.var_exp_chi) + (assign29770_body14_e42615 * locals.var_exp_chi_dn0)), (((locals.var_cfs1__blk973_dn2 * locals.var_beta) * locals.var_exp_chi) + (assign29770_body14_e42615 * locals.var_exp_chi_dn2)), (((locals.var_cfs1__blk973_dn6 * locals.var_beta) * locals.var_exp_chi) + (assign29770_body14_e42615 * locals.var_exp_chi_dn6)), (((locals.var_cfs1__blk973_dn7 * locals.var_beta) * locals.var_exp_chi) + (assign29770_body14_e42615 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1__blk973_dn10 * locals.var_beta) + (locals.var_cfs1__blk973 * locals.var_beta_dn10)) * locals.var_exp_chi) + (assign29770_body14_e42615 * locals.var_exp_chi_dn10)), (((locals.var_cfs1__blk973_dn11 * locals.var_beta) * locals.var_exp_chi) + (assign29770_body14_e42615 * locals.var_exp_chi_dn11)), (((locals.var_cfs1__blk973_dn12 * locals.var_beta) * locals.var_exp_chi) + (assign29770_body14_e42615 * locals.var_exp_chi_dn12)), (((locals.var_cfs1__blk973_dn17 * locals.var_beta) * locals.var_exp_chi) + (assign29770_body14_e42615 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk968, locals.var_fs01_dps0__blk968_dn0, locals.var_fs01_dps0__blk968_dn2, locals.var_fs01_dps0__blk968_dn6, locals.var_fs01_dps0__blk968_dn7, locals.var_fs01_dps0__blk968_dn10, locals.var_fs01_dps0__blk968_dn11, locals.var_fs01_dps0__blk968_dn12, locals.var_fs01_dps0__blk968_dn17,)
    }
};
            locals.var_fs01_dps0__blk968 = assign29770_body14_e42619;
            locals.var_fs01_dps0__blk968_dn0 = assign29770_body14_e42619_d_n0;
            locals.var_fs01_dps0__blk968_dn2 = assign29770_body14_e42619_d_n2;
            locals.var_fs01_dps0__blk968_dn6 = assign29770_body14_e42619_d_n6;
            locals.var_fs01_dps0__blk968_dn7 = assign29770_body14_e42619_d_n7;
            locals.var_fs01_dps0__blk968_dn10 = assign29770_body14_e42619_d_n10;
            locals.var_fs01_dps0__blk968_dn11 = assign29770_body14_e42619_d_n11;
            locals.var_fs01_dps0__blk968_dn12 = assign29770_body14_e42619_d_n12;
            locals.var_fs01_dps0__blk968_dn17 = assign29770_body14_e42619_d_n17;
            locals.var_fs01_dps0__blk968_rv = 0.0;
            let (assign29770_body15_e42642, assign29770_body15_e42642_d_n0, assign29770_body15_e42642_d_n2, assign29770_body15_e42642_d_n6, assign29770_body15_e42642_d_n7, assign29770_body15_e42642_d_n10, assign29770_body15_e42642_d_n11, assign29770_body15_e42642_d_n12, assign29770_body15_e42642_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard992 == 0.0)) && (locals.var_guard993 == 0.0)) {
        let assign29770_body15_e42639: f64 = (locals.var_beta * locals.var_ps0ld__blk947);
        let assign29770_body15_e42640: f64 = (assign29770_body15_e42639).exp();
        (assign29770_body15_e42640, (assign29770_body15_e42640 * (locals.var_beta * locals.var_ps0ld__blk947_dn0)), (assign29770_body15_e42640 * (locals.var_beta * locals.var_ps0ld__blk947_dn2)), (assign29770_body15_e42640 * (locals.var_beta * locals.var_ps0ld__blk947_dn6)), (assign29770_body15_e42640 * (locals.var_beta * locals.var_ps0ld__blk947_dn7)), (assign29770_body15_e42640 * ((locals.var_beta_dn10 * locals.var_ps0ld__blk947) + (locals.var_beta * locals.var_ps0ld__blk947_dn10))), (assign29770_body15_e42640 * (locals.var_beta * locals.var_ps0ld__blk947_dn11)), (assign29770_body15_e42640 * (locals.var_beta * locals.var_ps0ld__blk947_dn12)), (assign29770_body15_e42640 * (locals.var_beta * locals.var_ps0ld__blk947_dn17)),)
    } else {
        (locals.var_exp_bps0__blk974, locals.var_exp_bps0__blk974_dn0, locals.var_exp_bps0__blk974_dn2, locals.var_exp_bps0__blk974_dn6, locals.var_exp_bps0__blk974_dn7, locals.var_exp_bps0__blk974_dn10, locals.var_exp_bps0__blk974_dn11, locals.var_exp_bps0__blk974_dn12, locals.var_exp_bps0__blk974_dn17,)
    }
};
            locals.var_exp_bps0__blk974 = assign29770_body15_e42642;
            locals.var_exp_bps0__blk974_dn0 = assign29770_body15_e42642_d_n0;
            locals.var_exp_bps0__blk974_dn2 = assign29770_body15_e42642_d_n2;
            locals.var_exp_bps0__blk974_dn6 = assign29770_body15_e42642_d_n6;
            locals.var_exp_bps0__blk974_dn7 = assign29770_body15_e42642_d_n7;
            locals.var_exp_bps0__blk974_dn10 = assign29770_body15_e42642_d_n10;
            locals.var_exp_bps0__blk974_dn11 = assign29770_body15_e42642_d_n11;
            locals.var_exp_bps0__blk974_dn12 = assign29770_body15_e42642_d_n12;
            locals.var_exp_bps0__blk974_dn17 = assign29770_body15_e42642_d_n17;
            locals.var_exp_bps0__blk974_rv = 0.0;
            let (assign29770_body16_e42666, assign29770_body16_e42666_d_n0, assign29770_body16_e42666_d_n2, assign29770_body16_e42666_d_n6, assign29770_body16_e42666_d_n7, assign29770_body16_e42666_d_n10, assign29770_body16_e42666_d_n11, assign29770_body16_e42666_d_n12, assign29770_body16_e42666_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard992 == 0.0)) && (locals.var_guard993 == 0.0)) {
        let assign29770_body16_e42663: f64 = (locals.var_exp_bps0__blk974 - locals.var_exp_bvbs__blk964);
        let assign29770_body16_e42664: f64 = (locals.var_cnst1over__blk958 * assign29770_body16_e42663);
        (assign29770_body16_e42664, ((locals.var_cnst1over__blk958_dn0 * assign29770_body16_e42663) + (locals.var_cnst1over__blk958 * (locals.var_exp_bps0__blk974_dn0 - locals.var_exp_bvbs__blk964_dn0))), ((locals.var_cnst1over__blk958_dn2 * assign29770_body16_e42663) + (locals.var_cnst1over__blk958 * (locals.var_exp_bps0__blk974_dn2 - locals.var_exp_bvbs__blk964_dn2))), ((locals.var_cnst1over__blk958_dn6 * assign29770_body16_e42663) + (locals.var_cnst1over__blk958 * (locals.var_exp_bps0__blk974_dn6 - locals.var_exp_bvbs__blk964_dn6))), ((locals.var_cnst1over__blk958_dn7 * assign29770_body16_e42663) + (locals.var_cnst1over__blk958 * (locals.var_exp_bps0__blk974_dn7 - locals.var_exp_bvbs__blk964_dn7))), ((locals.var_cnst1over__blk958_dn10 * assign29770_body16_e42663) + (locals.var_cnst1over__blk958 * (locals.var_exp_bps0__blk974_dn10 - locals.var_exp_bvbs__blk964_dn10))), ((locals.var_cnst1over__blk958_dn11 * assign29770_body16_e42663) + (locals.var_cnst1over__blk958 * (locals.var_exp_bps0__blk974_dn11 - locals.var_exp_bvbs__blk964_dn11))), ((locals.var_cnst1over__blk958_dn12 * assign29770_body16_e42663) + (locals.var_cnst1over__blk958 * (locals.var_exp_bps0__blk974_dn12 - locals.var_exp_bvbs__blk964_dn12))), ((locals.var_cnst1over__blk958_dn17 * assign29770_body16_e42663) + (locals.var_cnst1over__blk958 * (locals.var_exp_bps0__blk974_dn17 - locals.var_exp_bvbs__blk964_dn17))),)
    } else {
        (locals.var_fs01__blk967, locals.var_fs01__blk967_dn0, locals.var_fs01__blk967_dn2, locals.var_fs01__blk967_dn6, locals.var_fs01__blk967_dn7, locals.var_fs01__blk967_dn10, locals.var_fs01__blk967_dn11, locals.var_fs01__blk967_dn12, locals.var_fs01__blk967_dn17,)
    }
};
            locals.var_fs01__blk967 = assign29770_body16_e42666;
            locals.var_fs01__blk967_dn0 = assign29770_body16_e42666_d_n0;
            locals.var_fs01__blk967_dn2 = assign29770_body16_e42666_d_n2;
            locals.var_fs01__blk967_dn6 = assign29770_body16_e42666_d_n6;
            locals.var_fs01__blk967_dn7 = assign29770_body16_e42666_d_n7;
            locals.var_fs01__blk967_dn10 = assign29770_body16_e42666_d_n10;
            locals.var_fs01__blk967_dn11 = assign29770_body16_e42666_d_n11;
            locals.var_fs01__blk967_dn12 = assign29770_body16_e42666_d_n12;
            locals.var_fs01__blk967_dn17 = assign29770_body16_e42666_d_n17;
            locals.var_fs01__blk967_rv = 0.0;
            let (assign29770_body17_e42690, assign29770_body17_e42690_d_n0, assign29770_body17_e42690_d_n2, assign29770_body17_e42690_d_n6, assign29770_body17_e42690_d_n7, assign29770_body17_e42690_d_n10, assign29770_body17_e42690_d_n11, assign29770_body17_e42690_d_n12, assign29770_body17_e42690_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard992 == 0.0)) && (locals.var_guard993 == 0.0)) {
        let assign29770_body17_e42686: f64 = (locals.var_cnst1over__blk958 * locals.var_beta);
        let assign29770_body17_e42688: f64 = (assign29770_body17_e42686 * locals.var_exp_bps0__blk974);
        (assign29770_body17_e42688, (((locals.var_cnst1over__blk958_dn0 * locals.var_beta) * locals.var_exp_bps0__blk974) + (assign29770_body17_e42686 * locals.var_exp_bps0__blk974_dn0)), (((locals.var_cnst1over__blk958_dn2 * locals.var_beta) * locals.var_exp_bps0__blk974) + (assign29770_body17_e42686 * locals.var_exp_bps0__blk974_dn2)), (((locals.var_cnst1over__blk958_dn6 * locals.var_beta) * locals.var_exp_bps0__blk974) + (assign29770_body17_e42686 * locals.var_exp_bps0__blk974_dn6)), (((locals.var_cnst1over__blk958_dn7 * locals.var_beta) * locals.var_exp_bps0__blk974) + (assign29770_body17_e42686 * locals.var_exp_bps0__blk974_dn7)), ((((locals.var_cnst1over__blk958_dn10 * locals.var_beta) + (locals.var_cnst1over__blk958 * locals.var_beta_dn10)) * locals.var_exp_bps0__blk974) + (assign29770_body17_e42686 * locals.var_exp_bps0__blk974_dn10)), (((locals.var_cnst1over__blk958_dn11 * locals.var_beta) * locals.var_exp_bps0__blk974) + (assign29770_body17_e42686 * locals.var_exp_bps0__blk974_dn11)), (((locals.var_cnst1over__blk958_dn12 * locals.var_beta) * locals.var_exp_bps0__blk974) + (assign29770_body17_e42686 * locals.var_exp_bps0__blk974_dn12)), (((locals.var_cnst1over__blk958_dn17 * locals.var_beta) * locals.var_exp_bps0__blk974) + (assign29770_body17_e42686 * locals.var_exp_bps0__blk974_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk968, locals.var_fs01_dps0__blk968_dn0, locals.var_fs01_dps0__blk968_dn2, locals.var_fs01_dps0__blk968_dn6, locals.var_fs01_dps0__blk968_dn7, locals.var_fs01_dps0__blk968_dn10, locals.var_fs01_dps0__blk968_dn11, locals.var_fs01_dps0__blk968_dn12, locals.var_fs01_dps0__blk968_dn17,)
    }
};
            locals.var_fs01_dps0__blk968 = assign29770_body17_e42690;
            locals.var_fs01_dps0__blk968_dn0 = assign29770_body17_e42690_d_n0;
            locals.var_fs01_dps0__blk968_dn2 = assign29770_body17_e42690_d_n2;
            locals.var_fs01_dps0__blk968_dn6 = assign29770_body17_e42690_d_n6;
            locals.var_fs01_dps0__blk968_dn7 = assign29770_body17_e42690_d_n7;
            locals.var_fs01_dps0__blk968_dn10 = assign29770_body17_e42690_d_n10;
            locals.var_fs01_dps0__blk968_dn11 = assign29770_body17_e42690_d_n11;
            locals.var_fs01_dps0__blk968_dn12 = assign29770_body17_e42690_d_n12;
            locals.var_fs01_dps0__blk968_dn17 = assign29770_body17_e42690_d_n17;
            locals.var_fs01_dps0__blk968_rv = 0.0;
            let (assign29770_body18_e42712, assign29770_body18_e42712_d_n0, assign29770_body18_e42712_d_n2, assign29770_body18_e42712_d_n6, assign29770_body18_e42712_d_n7, assign29770_body18_e42712_d_n10, assign29770_body18_e42712_d_n11, assign29770_body18_e42712_d_n12, assign29770_body18_e42712_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard992 == 0.0)) {
        let assign29770_body18_e42707: f64 = (locals.var_chi__blk945 - 1.0);
        let assign29770_body18_e42709: f64 = (assign29770_body18_e42707 + locals.var_fs01__blk967);
        let assign29770_body18_e42710: f64 = (assign29770_body18_e42709).sqrt();
        (assign29770_body18_e42710, ((locals.var_chi__blk945_dn0 + locals.var_fs01__blk967_dn0) / (2.0 * assign29770_body18_e42710)), ((locals.var_chi__blk945_dn2 + locals.var_fs01__blk967_dn2) / (2.0 * assign29770_body18_e42710)), ((locals.var_chi__blk945_dn6 + locals.var_fs01__blk967_dn6) / (2.0 * assign29770_body18_e42710)), ((locals.var_chi__blk945_dn7 + locals.var_fs01__blk967_dn7) / (2.0 * assign29770_body18_e42710)), ((locals.var_chi__blk945_dn10 + locals.var_fs01__blk967_dn10) / (2.0 * assign29770_body18_e42710)), ((locals.var_chi__blk945_dn11 + locals.var_fs01__blk967_dn11) / (2.0 * assign29770_body18_e42710)), ((locals.var_chi__blk945_dn12 + locals.var_fs01__blk967_dn12) / (2.0 * assign29770_body18_e42710)), ((locals.var_chi__blk945_dn17 + locals.var_fs01__blk967_dn17) / (2.0 * assign29770_body18_e42710)),)
    } else {
        (locals.var_fs02__blk971, locals.var_fs02__blk971_dn0, locals.var_fs02__blk971_dn2, locals.var_fs02__blk971_dn6, locals.var_fs02__blk971_dn7, locals.var_fs02__blk971_dn10, locals.var_fs02__blk971_dn11, locals.var_fs02__blk971_dn12, locals.var_fs02__blk971_dn17,)
    }
};
            locals.var_fs02__blk971 = assign29770_body18_e42712;
            locals.var_fs02__blk971_dn0 = assign29770_body18_e42712_d_n0;
            locals.var_fs02__blk971_dn2 = assign29770_body18_e42712_d_n2;
            locals.var_fs02__blk971_dn6 = assign29770_body18_e42712_d_n6;
            locals.var_fs02__blk971_dn7 = assign29770_body18_e42712_d_n7;
            locals.var_fs02__blk971_dn10 = assign29770_body18_e42712_d_n10;
            locals.var_fs02__blk971_dn11 = assign29770_body18_e42712_d_n11;
            locals.var_fs02__blk971_dn12 = assign29770_body18_e42712_d_n12;
            locals.var_fs02__blk971_dn17 = assign29770_body18_e42712_d_n17;
            locals.var_fs02__blk971_rv = 0.0;
            let (assign29770_body19_e42735, assign29770_body19_e42735_d_n0, assign29770_body19_e42735_d_n2, assign29770_body19_e42735_d_n6, assign29770_body19_e42735_d_n7, assign29770_body19_e42735_d_n10, assign29770_body19_e42735_d_n11, assign29770_body19_e42735_d_n12, assign29770_body19_e42735_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard992 == 0.0)) {
        let assign29770_body19_e42729: f64 = (locals.var_beta + locals.var_fs01_dps0__blk968);
        let assign29770_body19_e42731: f64 = (assign29770_body19_e42729 / locals.var_fs02__blk971);
        let assign29770_body19_e42733: f64 = (assign29770_body19_e42731 * 0.5);
        (assign29770_body19_e42733, ((((locals.var_fs01_dps0__blk968_dn0 * locals.var_fs02__blk971) - (assign29770_body19_e42729 * locals.var_fs02__blk971_dn0)) / (locals.var_fs02__blk971 * locals.var_fs02__blk971)) * 0.5), ((((locals.var_fs01_dps0__blk968_dn2 * locals.var_fs02__blk971) - (assign29770_body19_e42729 * locals.var_fs02__blk971_dn2)) / (locals.var_fs02__blk971 * locals.var_fs02__blk971)) * 0.5), ((((locals.var_fs01_dps0__blk968_dn6 * locals.var_fs02__blk971) - (assign29770_body19_e42729 * locals.var_fs02__blk971_dn6)) / (locals.var_fs02__blk971 * locals.var_fs02__blk971)) * 0.5), ((((locals.var_fs01_dps0__blk968_dn7 * locals.var_fs02__blk971) - (assign29770_body19_e42729 * locals.var_fs02__blk971_dn7)) / (locals.var_fs02__blk971 * locals.var_fs02__blk971)) * 0.5), (((((locals.var_beta_dn10 + locals.var_fs01_dps0__blk968_dn10) * locals.var_fs02__blk971) - (assign29770_body19_e42729 * locals.var_fs02__blk971_dn10)) / (locals.var_fs02__blk971 * locals.var_fs02__blk971)) * 0.5), ((((locals.var_fs01_dps0__blk968_dn11 * locals.var_fs02__blk971) - (assign29770_body19_e42729 * locals.var_fs02__blk971_dn11)) / (locals.var_fs02__blk971 * locals.var_fs02__blk971)) * 0.5), ((((locals.var_fs01_dps0__blk968_dn12 * locals.var_fs02__blk971) - (assign29770_body19_e42729 * locals.var_fs02__blk971_dn12)) / (locals.var_fs02__blk971 * locals.var_fs02__blk971)) * 0.5), ((((locals.var_fs01_dps0__blk968_dn17 * locals.var_fs02__blk971) - (assign29770_body19_e42729 * locals.var_fs02__blk971_dn17)) / (locals.var_fs02__blk971 * locals.var_fs02__blk971)) * 0.5),)
    } else {
        (locals.var_fs02_dps0__blk972, locals.var_fs02_dps0__blk972_dn0, locals.var_fs02_dps0__blk972_dn2, locals.var_fs02_dps0__blk972_dn6, locals.var_fs02_dps0__blk972_dn7, locals.var_fs02_dps0__blk972_dn10, locals.var_fs02_dps0__blk972_dn11, locals.var_fs02_dps0__blk972_dn12, locals.var_fs02_dps0__blk972_dn17,)
    }
};
            locals.var_fs02_dps0__blk972 = assign29770_body19_e42735;
            locals.var_fs02_dps0__blk972_dn0 = assign29770_body19_e42735_d_n0;
            locals.var_fs02_dps0__blk972_dn2 = assign29770_body19_e42735_d_n2;
            locals.var_fs02_dps0__blk972_dn6 = assign29770_body19_e42735_d_n6;
            locals.var_fs02_dps0__blk972_dn7 = assign29770_body19_e42735_d_n7;
            locals.var_fs02_dps0__blk972_dn10 = assign29770_body19_e42735_d_n10;
            locals.var_fs02_dps0__blk972_dn11 = assign29770_body19_e42735_d_n11;
            locals.var_fs02_dps0__blk972_dn12 = assign29770_body19_e42735_d_n12;
            locals.var_fs02_dps0__blk972_dn17 = assign29770_body19_e42735_d_n17;
            locals.var_fs02_dps0__blk972_rv = 0.0;
            let (assign29770_body20_e42755, assign29770_body20_e42755_d_n0, assign29770_body20_e42755_d_n2, assign29770_body20_e42755_d_n6, assign29770_body20_e42755_d_n7, assign29770_body20_e42755_d_n10, assign29770_body20_e42755_d_n11, assign29770_body20_e42755_d_n12, assign29770_body20_e42755_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29770_body20_e42749: f64 = (locals.var_vgpld__blk933 - locals.var_ps0ld__blk947);
        let assign29770_body20_e42752: f64 = (locals.var_fac1__blk931 * locals.var_fs02__blk971);
        let assign29770_body20_e42753: f64 = (assign29770_body20_e42749 - assign29770_body20_e42752);
        (assign29770_body20_e42753, ((locals.var_vgpld__blk933_dn0 - locals.var_ps0ld__blk947_dn0) - ((locals.var_fac1__blk931_dn0 * locals.var_fs02__blk971) + (locals.var_fac1__blk931 * locals.var_fs02__blk971_dn0))), ((locals.var_vgpld__blk933_dn2 - locals.var_ps0ld__blk947_dn2) - ((locals.var_fac1__blk931_dn2 * locals.var_fs02__blk971) + (locals.var_fac1__blk931 * locals.var_fs02__blk971_dn2))), ((locals.var_vgpld__blk933_dn6 - locals.var_ps0ld__blk947_dn6) - ((locals.var_fac1__blk931_dn6 * locals.var_fs02__blk971) + (locals.var_fac1__blk931 * locals.var_fs02__blk971_dn6))), ((locals.var_vgpld__blk933_dn7 - locals.var_ps0ld__blk947_dn7) - ((locals.var_fac1__blk931_dn7 * locals.var_fs02__blk971) + (locals.var_fac1__blk931 * locals.var_fs02__blk971_dn7))), ((locals.var_vgpld__blk933_dn10 - locals.var_ps0ld__blk947_dn10) - ((locals.var_fac1__blk931_dn10 * locals.var_fs02__blk971) + (locals.var_fac1__blk931 * locals.var_fs02__blk971_dn10))), ((locals.var_vgpld__blk933_dn11 - locals.var_ps0ld__blk947_dn11) - ((locals.var_fac1__blk931_dn11 * locals.var_fs02__blk971) + (locals.var_fac1__blk931 * locals.var_fs02__blk971_dn11))), ((locals.var_vgpld__blk933_dn12 - locals.var_ps0ld__blk947_dn12) - ((locals.var_fac1__blk931_dn12 * locals.var_fs02__blk971) + (locals.var_fac1__blk931 * locals.var_fs02__blk971_dn12))), ((locals.var_vgpld__blk933_dn17 - locals.var_ps0ld__blk947_dn17) - ((locals.var_fac1__blk931_dn17 * locals.var_fs02__blk971) + (locals.var_fac1__blk931 * locals.var_fs02__blk971_dn17))),)
    } else {
        (locals.var_fs0__blk975, locals.var_fs0__blk975_dn0, locals.var_fs0__blk975_dn2, locals.var_fs0__blk975_dn6, locals.var_fs0__blk975_dn7, locals.var_fs0__blk975_dn10, locals.var_fs0__blk975_dn11, locals.var_fs0__blk975_dn12, locals.var_fs0__blk975_dn17,)
    }
};
            locals.var_fs0__blk975 = assign29770_body20_e42755;
            locals.var_fs0__blk975_dn0 = assign29770_body20_e42755_d_n0;
            locals.var_fs0__blk975_dn2 = assign29770_body20_e42755_d_n2;
            locals.var_fs0__blk975_dn6 = assign29770_body20_e42755_d_n6;
            locals.var_fs0__blk975_dn7 = assign29770_body20_e42755_d_n7;
            locals.var_fs0__blk975_dn10 = assign29770_body20_e42755_d_n10;
            locals.var_fs0__blk975_dn11 = assign29770_body20_e42755_d_n11;
            locals.var_fs0__blk975_dn12 = assign29770_body20_e42755_d_n12;
            locals.var_fs0__blk975_dn17 = assign29770_body20_e42755_d_n17;
            locals.var_fs0__blk975_rv = 0.0;
            let (assign29770_body21_e42774, assign29770_body21_e42774_d_n0, assign29770_body21_e42774_d_n2, assign29770_body21_e42774_d_n6, assign29770_body21_e42774_d_n7, assign29770_body21_e42774_d_n10, assign29770_body21_e42774_d_n11, assign29770_body21_e42774_d_n12, assign29770_body21_e42774_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29770_body21_e42768: f64 = (-1.0);
        let assign29770_body21_e42771: f64 = (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972);
        let assign29770_body21_e42772: f64 = (assign29770_body21_e42768 - assign29770_body21_e42771);
        (assign29770_body21_e42772, (-((locals.var_fac1__blk931_dn0 * locals.var_fs02_dps0__blk972) + (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972_dn0))), (-((locals.var_fac1__blk931_dn2 * locals.var_fs02_dps0__blk972) + (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972_dn2))), (-((locals.var_fac1__blk931_dn6 * locals.var_fs02_dps0__blk972) + (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972_dn6))), (-((locals.var_fac1__blk931_dn7 * locals.var_fs02_dps0__blk972) + (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972_dn7))), (-((locals.var_fac1__blk931_dn10 * locals.var_fs02_dps0__blk972) + (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972_dn10))), (-((locals.var_fac1__blk931_dn11 * locals.var_fs02_dps0__blk972) + (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972_dn11))), (-((locals.var_fac1__blk931_dn12 * locals.var_fs02_dps0__blk972) + (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972_dn12))), (-((locals.var_fac1__blk931_dn17 * locals.var_fs02_dps0__blk972) + (locals.var_fac1__blk931 * locals.var_fs02_dps0__blk972_dn17))),)
    } else {
        (locals.var_fs0_dps0__blk976, locals.var_fs0_dps0__blk976_dn0, locals.var_fs0_dps0__blk976_dn2, locals.var_fs0_dps0__blk976_dn6, locals.var_fs0_dps0__blk976_dn7, locals.var_fs0_dps0__blk976_dn10, locals.var_fs0_dps0__blk976_dn11, locals.var_fs0_dps0__blk976_dn12, locals.var_fs0_dps0__blk976_dn17,)
    }
};
            locals.var_fs0_dps0__blk976 = assign29770_body21_e42774;
            locals.var_fs0_dps0__blk976_dn0 = assign29770_body21_e42774_d_n0;
            locals.var_fs0_dps0__blk976_dn2 = assign29770_body21_e42774_d_n2;
            locals.var_fs0_dps0__blk976_dn6 = assign29770_body21_e42774_d_n6;
            locals.var_fs0_dps0__blk976_dn7 = assign29770_body21_e42774_d_n7;
            locals.var_fs0_dps0__blk976_dn10 = assign29770_body21_e42774_d_n10;
            locals.var_fs0_dps0__blk976_dn11 = assign29770_body21_e42774_d_n11;
            locals.var_fs0_dps0__blk976_dn12 = assign29770_body21_e42774_d_n12;
            locals.var_fs0_dps0__blk976_dn17 = assign29770_body21_e42774_d_n17;
            locals.var_fs0_dps0__blk976_rv = 0.0;
            let assign29770_body22_e42777: f64 = if locals.var_flg_conv__blk920 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard994 = assign29770_body22_e42777;
            locals.var_guard994_rv = 0.0;
            let (assign29770_body23_e42797,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard994 != 0.0)) {
        let assign29770_body23_e42793: f64 = (2.0 * 20.0);
        let assign29770_body23_e42795: f64 = (assign29770_body23_e42793 + 1.0);
        (assign29770_body23_e42795,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign29770_body23_e42797;
            locals.var_lp_s0_rv = 0.0;
            let (assign29770_body24_e42817, assign29770_body24_e42817_d_n0, assign29770_body24_e42817_d_n2, assign29770_body24_e42817_d_n6, assign29770_body24_e42817_d_n7, assign29770_body24_e42817_d_n10, assign29770_body24_e42817_d_n11, assign29770_body24_e42817_d_n12, assign29770_body24_e42817_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard994 == 0.0)) {
        let assign29770_body24_e42813: f64 = (-locals.var_fs0__blk975);
        let assign29770_body24_e42815: f64 = (assign29770_body24_e42813 / locals.var_fs0_dps0__blk976);
        (assign29770_body24_e42815, ((((-locals.var_fs0__blk975_dn0) * locals.var_fs0_dps0__blk976) - (assign29770_body24_e42813 * locals.var_fs0_dps0__blk976_dn0)) / (locals.var_fs0_dps0__blk976 * locals.var_fs0_dps0__blk976)), ((((-locals.var_fs0__blk975_dn2) * locals.var_fs0_dps0__blk976) - (assign29770_body24_e42813 * locals.var_fs0_dps0__blk976_dn2)) / (locals.var_fs0_dps0__blk976 * locals.var_fs0_dps0__blk976)), ((((-locals.var_fs0__blk975_dn6) * locals.var_fs0_dps0__blk976) - (assign29770_body24_e42813 * locals.var_fs0_dps0__blk976_dn6)) / (locals.var_fs0_dps0__blk976 * locals.var_fs0_dps0__blk976)), ((((-locals.var_fs0__blk975_dn7) * locals.var_fs0_dps0__blk976) - (assign29770_body24_e42813 * locals.var_fs0_dps0__blk976_dn7)) / (locals.var_fs0_dps0__blk976 * locals.var_fs0_dps0__blk976)), ((((-locals.var_fs0__blk975_dn10) * locals.var_fs0_dps0__blk976) - (assign29770_body24_e42813 * locals.var_fs0_dps0__blk976_dn10)) / (locals.var_fs0_dps0__blk976 * locals.var_fs0_dps0__blk976)), ((((-locals.var_fs0__blk975_dn11) * locals.var_fs0_dps0__blk976) - (assign29770_body24_e42813 * locals.var_fs0_dps0__blk976_dn11)) / (locals.var_fs0_dps0__blk976 * locals.var_fs0_dps0__blk976)), ((((-locals.var_fs0__blk975_dn12) * locals.var_fs0_dps0__blk976) - (assign29770_body24_e42813 * locals.var_fs0_dps0__blk976_dn12)) / (locals.var_fs0_dps0__blk976 * locals.var_fs0_dps0__blk976)), ((((-locals.var_fs0__blk975_dn17) * locals.var_fs0_dps0__blk976) - (assign29770_body24_e42813 * locals.var_fs0_dps0__blk976_dn17)) / (locals.var_fs0_dps0__blk976 * locals.var_fs0_dps0__blk976)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign29770_body24_e42817;
            locals.var_dps0_dn0 = assign29770_body24_e42817_d_n0;
            locals.var_dps0_dn2 = assign29770_body24_e42817_d_n2;
            locals.var_dps0_dn6 = assign29770_body24_e42817_d_n6;
            locals.var_dps0_dn7 = assign29770_body24_e42817_d_n7;
            locals.var_dps0_dn10 = assign29770_body24_e42817_d_n10;
            locals.var_dps0_dn11 = assign29770_body24_e42817_d_n11;
            locals.var_dps0_dn12 = assign29770_body24_e42817_d_n12;
            locals.var_dps0_dn17 = assign29770_body24_e42817_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign29770_body25_e42847, assign29770_body25_e42847_d_n0, assign29770_body25_e42847_d_n2, assign29770_body25_e42847_d_n6, assign29770_body25_e42847_d_n7, assign29770_body25_e42847_d_n10, assign29770_body25_e42847_d_n11, assign29770_body25_e42847_d_n12, assign29770_body25_e42847_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard994 == 0.0)) {
        let assign29770_body25_e42834: f64 = (0.5 * 0.1);
        let assign29770_body25_e42838: f64 = (locals.var_ps0ld__blk947).abs();
        let (assign29770_body25_e42843, assign29770_body25_e42843_d_n0, assign29770_body25_e42843_d_n2, assign29770_body25_e42843_d_n6, assign29770_body25_e42843_d_n7, assign29770_body25_e42843_d_n10, assign29770_body25_e42843_d_n11, assign29770_body25_e42843_d_n12, assign29770_body25_e42843_d_n17,) = {
            if (1.0 >= assign29770_body25_e42838) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign29770_body25_e42842: f64 = (locals.var_ps0ld__blk947).abs();
                (assign29770_body25_e42842, if locals.var_ps0ld__blk947 >= 0.0 { locals.var_ps0ld__blk947_dn0 } else { (-locals.var_ps0ld__blk947_dn0) }, if locals.var_ps0ld__blk947 >= 0.0 { locals.var_ps0ld__blk947_dn2 } else { (-locals.var_ps0ld__blk947_dn2) }, if locals.var_ps0ld__blk947 >= 0.0 { locals.var_ps0ld__blk947_dn6 } else { (-locals.var_ps0ld__blk947_dn6) }, if locals.var_ps0ld__blk947 >= 0.0 { locals.var_ps0ld__blk947_dn7 } else { (-locals.var_ps0ld__blk947_dn7) }, if locals.var_ps0ld__blk947 >= 0.0 { locals.var_ps0ld__blk947_dn10 } else { (-locals.var_ps0ld__blk947_dn10) }, if locals.var_ps0ld__blk947 >= 0.0 { locals.var_ps0ld__blk947_dn11 } else { (-locals.var_ps0ld__blk947_dn11) }, if locals.var_ps0ld__blk947 >= 0.0 { locals.var_ps0ld__blk947_dn12 } else { (-locals.var_ps0ld__blk947_dn12) }, if locals.var_ps0ld__blk947 >= 0.0 { locals.var_ps0ld__blk947_dn17 } else { (-locals.var_ps0ld__blk947_dn17) },)
            }
        };
        let assign29770_body25_e42844: f64 = (1.0 + assign29770_body25_e42843);
        let assign29770_body25_e42845: f64 = (assign29770_body25_e42834 * assign29770_body25_e42844);
        (assign29770_body25_e42845, (assign29770_body25_e42834 * assign29770_body25_e42843_d_n0), (assign29770_body25_e42834 * assign29770_body25_e42843_d_n2), (assign29770_body25_e42834 * assign29770_body25_e42843_d_n6), (assign29770_body25_e42834 * assign29770_body25_e42843_d_n7), (assign29770_body25_e42834 * assign29770_body25_e42843_d_n10), (assign29770_body25_e42834 * assign29770_body25_e42843_d_n11), (assign29770_body25_e42834 * assign29770_body25_e42843_d_n12), (assign29770_body25_e42834 * assign29770_body25_e42843_d_n17),)
    } else {
        (locals.var_dplim__blk977, locals.var_dplim__blk977_dn0, locals.var_dplim__blk977_dn2, locals.var_dplim__blk977_dn6, locals.var_dplim__blk977_dn7, locals.var_dplim__blk977_dn10, locals.var_dplim__blk977_dn11, locals.var_dplim__blk977_dn12, locals.var_dplim__blk977_dn17,)
    }
};
            locals.var_dplim__blk977 = assign29770_body25_e42847;
            locals.var_dplim__blk977_dn0 = assign29770_body25_e42847_d_n0;
            locals.var_dplim__blk977_dn2 = assign29770_body25_e42847_d_n2;
            locals.var_dplim__blk977_dn6 = assign29770_body25_e42847_d_n6;
            locals.var_dplim__blk977_dn7 = assign29770_body25_e42847_d_n7;
            locals.var_dplim__blk977_dn10 = assign29770_body25_e42847_d_n10;
            locals.var_dplim__blk977_dn11 = assign29770_body25_e42847_d_n11;
            locals.var_dplim__blk977_dn12 = assign29770_body25_e42847_d_n12;
            locals.var_dplim__blk977_dn17 = assign29770_body25_e42847_d_n17;
            locals.var_dplim__blk977_rv = 0.0;
            let assign29770_body26_e42849: f64 = (locals.var_dps0).abs();
            let assign29770_body26_e42851: f64 = if assign29770_body26_e42849 > locals.var_dplim__blk977 { 1.0 } else { 0.0 };
            locals.var_guard995 = assign29770_body26_e42851;
            locals.var_guard995_rv = 0.0;
            let (assign29770_body27_e42878, assign29770_body27_e42878_d_n0, assign29770_body27_e42878_d_n2, assign29770_body27_e42878_d_n6, assign29770_body27_e42878_d_n7, assign29770_body27_e42878_d_n10, assign29770_body27_e42878_d_n11, assign29770_body27_e42878_d_n12, assign29770_body27_e42878_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard994 == 0.0)) && (locals.var_guard995 != 0.0)) {
        let (assign29770_body27_e42875,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign29770_body27_e42874: f64 = (-1.0);
                (assign29770_body27_e42874,)
            }
        };
        let assign29770_body27_e42876: f64 = (locals.var_dplim__blk977 * assign29770_body27_e42875);
        (assign29770_body27_e42876, (locals.var_dplim__blk977_dn0 * assign29770_body27_e42875), (locals.var_dplim__blk977_dn2 * assign29770_body27_e42875), (locals.var_dplim__blk977_dn6 * assign29770_body27_e42875), (locals.var_dplim__blk977_dn7 * assign29770_body27_e42875), (locals.var_dplim__blk977_dn10 * assign29770_body27_e42875), (locals.var_dplim__blk977_dn11 * assign29770_body27_e42875), (locals.var_dplim__blk977_dn12 * assign29770_body27_e42875), (locals.var_dplim__blk977_dn17 * assign29770_body27_e42875),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign29770_body27_e42878;
            locals.var_dps0_dn0 = assign29770_body27_e42878_d_n0;
            locals.var_dps0_dn2 = assign29770_body27_e42878_d_n2;
            locals.var_dps0_dn6 = assign29770_body27_e42878_d_n6;
            locals.var_dps0_dn7 = assign29770_body27_e42878_d_n7;
            locals.var_dps0_dn10 = assign29770_body27_e42878_d_n10;
            locals.var_dps0_dn11 = assign29770_body27_e42878_d_n11;
            locals.var_dps0_dn12 = assign29770_body27_e42878_d_n12;
            locals.var_dps0_dn17 = assign29770_body27_e42878_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign29770_body28_e42897, assign29770_body28_e42897_d_n0, assign29770_body28_e42897_d_n2, assign29770_body28_e42897_d_n6, assign29770_body28_e42897_d_n7, assign29770_body28_e42897_d_n10, assign29770_body28_e42897_d_n11, assign29770_body28_e42897_d_n12, assign29770_body28_e42897_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard994 == 0.0)) {
        let assign29770_body28_e42895: f64 = (locals.var_ps0ld__blk947 + locals.var_dps0);
        (assign29770_body28_e42895, (locals.var_ps0ld__blk947_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld__blk947_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld__blk947_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld__blk947_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld__blk947_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld__blk947_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld__blk947_dn12 + locals.var_dps0_dn12), (locals.var_ps0ld__blk947_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_ps0ld__blk947, locals.var_ps0ld__blk947_dn0, locals.var_ps0ld__blk947_dn2, locals.var_ps0ld__blk947_dn6, locals.var_ps0ld__blk947_dn7, locals.var_ps0ld__blk947_dn10, locals.var_ps0ld__blk947_dn11, locals.var_ps0ld__blk947_dn12, locals.var_ps0ld__blk947_dn17,)
    }
};
            locals.var_ps0ld__blk947 = assign29770_body28_e42897;
            locals.var_ps0ld__blk947_dn0 = assign29770_body28_e42897_d_n0;
            locals.var_ps0ld__blk947_dn2 = assign29770_body28_e42897_d_n2;
            locals.var_ps0ld__blk947_dn6 = assign29770_body28_e42897_d_n6;
            locals.var_ps0ld__blk947_dn7 = assign29770_body28_e42897_d_n7;
            locals.var_ps0ld__blk947_dn10 = assign29770_body28_e42897_d_n10;
            locals.var_ps0ld__blk947_dn11 = assign29770_body28_e42897_d_n11;
            locals.var_ps0ld__blk947_dn12 = assign29770_body28_e42897_d_n12;
            locals.var_ps0ld__blk947_dn17 = assign29770_body28_e42897_d_n17;
            locals.var_ps0ld__blk947_rv = 0.0;
            let assign29770_body29_e42899: f64 = (locals.var_dps0).abs();
            let assign29770_body29_e42903: f64 = (locals.var_fs0__blk975).abs();
            let assign29770_body29_e42906: f64 = if ((assign29770_body29_e42899 <= 5e-12) && (assign29770_body29_e42903 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard996 = assign29770_body29_e42906;
            locals.var_guard996_rv = 0.0;
            let (assign29770_body30_e42925,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard994 == 0.0)) && (locals.var_guard996 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv__blk920,)
    }
};
            locals.var_flg_conv__blk920 = assign29770_body30_e42925;
            locals.var_flg_conv__blk920_rv = 0.0;
            let (assign29770_body31_e42941,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29770_body31_e42939: f64 = (locals.var_lp_s0 + 1.0);
        (assign29770_body31_e42939,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign29770_body31_e42941;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_109(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign29790_e42947: f64 = if locals.var_chi__blk945 < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard998 = assign29790_e42947;
        locals.var_guard998_rv = 0.0;

        let (assign29830_e43009, assign29830_e43009_d_n0, assign29830_e43009_d_n2, assign29830_e43009_d_n6, assign29830_e43009_d_n7, assign29830_e43009_d_n10, assign29830_e43009_d_n11, assign29830_e43009_d_n12, assign29830_e43009_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard998 != 0.0)) {
        let assign29830_e43003: f64 = (locals.var_fb__blk969 * locals.var_fb__blk969);
        let assign29830_e43006: f64 = (10.0 * 2.220446049250313e-16);
        let assign29830_e43007: f64 = (assign29830_e43003 + assign29830_e43006);
        (assign29830_e43007, ((locals.var_fb__blk969_dn0 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn0)), ((locals.var_fb__blk969_dn2 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn2)), ((locals.var_fb__blk969_dn6 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn6)), ((locals.var_fb__blk969_dn7 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn7)), ((locals.var_fb__blk969_dn10 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn10)), ((locals.var_fb__blk969_dn11 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn11)), ((locals.var_fb__blk969_dn12 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn12)), ((locals.var_fb__blk969_dn17 * locals.var_fb__blk969) + (locals.var_fb__blk969 * locals.var_fb__blk969_dn17)),)
    } else {
        (locals.var_xi0__blk978, locals.var_xi0__blk978_dn0, locals.var_xi0__blk978_dn2, locals.var_xi0__blk978_dn6, locals.var_xi0__blk978_dn7, locals.var_xi0__blk978_dn10, locals.var_xi0__blk978_dn11, locals.var_xi0__blk978_dn12, locals.var_xi0__blk978_dn17,)
    }
};
        locals.var_xi0__blk978 = assign29830_e43009;
        locals.var_xi0__blk978_dn0 = assign29830_e43009_d_n0;
        locals.var_xi0__blk978_dn2 = assign29830_e43009_d_n2;
        locals.var_xi0__blk978_dn6 = assign29830_e43009_d_n6;
        locals.var_xi0__blk978_dn7 = assign29830_e43009_d_n7;
        locals.var_xi0__blk978_dn10 = assign29830_e43009_d_n10;
        locals.var_xi0__blk978_dn11 = assign29830_e43009_d_n11;
        locals.var_xi0__blk978_dn12 = assign29830_e43009_d_n12;
        locals.var_xi0__blk978_dn17 = assign29830_e43009_d_n17;
        locals.var_xi0__blk978_rv = 0.0;

        let (assign29840_e43029, assign29840_e43029_d_n0, assign29840_e43029_d_n2, assign29840_e43029_d_n6, assign29840_e43029_d_n7, assign29840_e43029_d_n10, assign29840_e43029_d_n11, assign29840_e43029_d_n12, assign29840_e43029_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard998 != 0.0)) {
        let assign29840_e43026: f64 = (10.0 * 2.220446049250313e-16);
        let assign29840_e43027: f64 = (locals.var_fb__blk969 + assign29840_e43026);
        (assign29840_e43027, locals.var_fb__blk969_dn0, locals.var_fb__blk969_dn2, locals.var_fb__blk969_dn6, locals.var_fb__blk969_dn7, locals.var_fb__blk969_dn10, locals.var_fb__blk969_dn11, locals.var_fb__blk969_dn12, locals.var_fb__blk969_dn17,)
    } else {
        (locals.var_xi0p12__blk979, locals.var_xi0p12__blk979_dn0, locals.var_xi0p12__blk979_dn2, locals.var_xi0p12__blk979_dn6, locals.var_xi0p12__blk979_dn7, locals.var_xi0p12__blk979_dn10, locals.var_xi0p12__blk979_dn11, locals.var_xi0p12__blk979_dn12, locals.var_xi0p12__blk979_dn17,)
    }
};
        locals.var_xi0p12__blk979 = assign29840_e43029;
        locals.var_xi0p12__blk979_dn0 = assign29840_e43029_d_n0;
        locals.var_xi0p12__blk979_dn2 = assign29840_e43029_d_n2;
        locals.var_xi0p12__blk979_dn6 = assign29840_e43029_d_n6;
        locals.var_xi0p12__blk979_dn7 = assign29840_e43029_d_n7;
        locals.var_xi0p12__blk979_dn10 = assign29840_e43029_d_n10;
        locals.var_xi0p12__blk979_dn11 = assign29840_e43029_d_n11;
        locals.var_xi0p12__blk979_dn12 = assign29840_e43029_d_n12;
        locals.var_xi0p12__blk979_dn17 = assign29840_e43029_d_n17;
        locals.var_xi0p12__blk979_rv = 0.0;

        let (assign29860_e43065, assign29860_e43065_d_n0, assign29860_e43065_d_n2, assign29860_e43065_d_n6, assign29860_e43065_d_n7, assign29860_e43065_d_n10, assign29860_e43065_d_n11, assign29860_e43065_d_n12, assign29860_e43065_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard998 == 0.0)) {
        let assign29860_e43063: f64 = (locals.var_chi__blk945 - 1.0);
        (assign29860_e43063, locals.var_chi__blk945_dn0, locals.var_chi__blk945_dn2, locals.var_chi__blk945_dn6, locals.var_chi__blk945_dn7, locals.var_chi__blk945_dn10, locals.var_chi__blk945_dn11, locals.var_chi__blk945_dn12, locals.var_chi__blk945_dn17,)
    } else {
        (locals.var_xi0__blk978, locals.var_xi0__blk978_dn0, locals.var_xi0__blk978_dn2, locals.var_xi0__blk978_dn6, locals.var_xi0__blk978_dn7, locals.var_xi0__blk978_dn10, locals.var_xi0__blk978_dn11, locals.var_xi0__blk978_dn12, locals.var_xi0__blk978_dn17,)
    }
};
        locals.var_xi0__blk978 = assign29860_e43065;
        locals.var_xi0__blk978_dn0 = assign29860_e43065_d_n0;
        locals.var_xi0__blk978_dn2 = assign29860_e43065_d_n2;
        locals.var_xi0__blk978_dn6 = assign29860_e43065_d_n6;
        locals.var_xi0__blk978_dn7 = assign29860_e43065_d_n7;
        locals.var_xi0__blk978_dn10 = assign29860_e43065_d_n10;
        locals.var_xi0__blk978_dn11 = assign29860_e43065_d_n11;
        locals.var_xi0__blk978_dn12 = assign29860_e43065_d_n12;
        locals.var_xi0__blk978_dn17 = assign29860_e43065_d_n17;
        locals.var_xi0__blk978_rv = 0.0;

        let (assign29870_e43083, assign29870_e43083_d_n0, assign29870_e43083_d_n2, assign29870_e43083_d_n6, assign29870_e43083_d_n7, assign29870_e43083_d_n10, assign29870_e43083_d_n11, assign29870_e43083_d_n12, assign29870_e43083_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) && (locals.var_guard998 == 0.0)) {
        let assign29870_e43081: f64 = (locals.var_xi0__blk978).sqrt();
        (assign29870_e43081, (locals.var_xi0__blk978_dn0 / (2.0 * assign29870_e43081)), (locals.var_xi0__blk978_dn2 / (2.0 * assign29870_e43081)), (locals.var_xi0__blk978_dn6 / (2.0 * assign29870_e43081)), (locals.var_xi0__blk978_dn7 / (2.0 * assign29870_e43081)), (locals.var_xi0__blk978_dn10 / (2.0 * assign29870_e43081)), (locals.var_xi0__blk978_dn11 / (2.0 * assign29870_e43081)), (locals.var_xi0__blk978_dn12 / (2.0 * assign29870_e43081)), (locals.var_xi0__blk978_dn17 / (2.0 * assign29870_e43081)),)
    } else {
        (locals.var_xi0p12__blk979, locals.var_xi0p12__blk979_dn0, locals.var_xi0p12__blk979_dn2, locals.var_xi0p12__blk979_dn6, locals.var_xi0p12__blk979_dn7, locals.var_xi0p12__blk979_dn10, locals.var_xi0p12__blk979_dn11, locals.var_xi0p12__blk979_dn12, locals.var_xi0p12__blk979_dn17,)
    }
};
        locals.var_xi0p12__blk979 = assign29870_e43083;
        locals.var_xi0p12__blk979_dn0 = assign29870_e43083_d_n0;
        locals.var_xi0p12__blk979_dn2 = assign29870_e43083_d_n2;
        locals.var_xi0p12__blk979_dn6 = assign29870_e43083_d_n6;
        locals.var_xi0p12__blk979_dn7 = assign29870_e43083_d_n7;
        locals.var_xi0p12__blk979_dn10 = assign29870_e43083_d_n10;
        locals.var_xi0p12__blk979_dn11 = assign29870_e43083_d_n11;
        locals.var_xi0p12__blk979_dn12 = assign29870_e43083_d_n12;
        locals.var_xi0p12__blk979_dn17 = assign29870_e43083_d_n17;
        locals.var_xi0p12__blk979_rv = 0.0;

        let (assign29880_e43099, assign29880_e43099_d_n0, assign29880_e43099_d_n2, assign29880_e43099_d_n6, assign29880_e43099_d_n7, assign29880_e43099_d_n10, assign29880_e43099_d_n11, assign29880_e43099_d_n12, assign29880_e43099_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29880_e43097: f64 = (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979);
        (assign29880_e43097, ((locals.var_cnst0over__blk930_dn0 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn0)), ((locals.var_cnst0over__blk930_dn2 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn2)), ((locals.var_cnst0over__blk930_dn6 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn6)), ((locals.var_cnst0over__blk930_dn7 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn7)), ((locals.var_cnst0over__blk930_dn10 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn10)), ((locals.var_cnst0over__blk930_dn11 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn11)), ((locals.var_cnst0over__blk930_dn12 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn12)), ((locals.var_cnst0over__blk930_dn17 * locals.var_xi0p12__blk979) + (locals.var_cnst0over__blk930 * locals.var_xi0p12__blk979_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign29880_e43099;
        locals.var_qbuld_dn0 = assign29880_e43099_d_n0;
        locals.var_qbuld_dn2 = assign29880_e43099_d_n2;
        locals.var_qbuld_dn6 = assign29880_e43099_d_n6;
        locals.var_qbuld_dn7 = assign29880_e43099_d_n7;
        locals.var_qbuld_dn10 = assign29880_e43099_d_n10;
        locals.var_qbuld_dn11 = assign29880_e43099_d_n11;
        locals.var_qbuld_dn12 = assign29880_e43099_d_n12;
        locals.var_qbuld_dn17 = assign29880_e43099_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign29890_e43117, assign29890_e43117_d_n0, assign29890_e43117_d_n2, assign29890_e43117_d_n6, assign29890_e43117_d_n7, assign29890_e43117_d_n10, assign29890_e43117_d_n11, assign29890_e43117_d_n12, assign29890_e43117_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29890_e43114: f64 = (locals.var_fs02__blk971 + locals.var_xi0p12__blk979);
        let assign29890_e43115: f64 = (1.0 / assign29890_e43114);
        (assign29890_e43115, (-((locals.var_fs02__blk971_dn0 + locals.var_xi0p12__blk979_dn0) / (assign29890_e43114 * assign29890_e43114))), (-((locals.var_fs02__blk971_dn2 + locals.var_xi0p12__blk979_dn2) / (assign29890_e43114 * assign29890_e43114))), (-((locals.var_fs02__blk971_dn6 + locals.var_xi0p12__blk979_dn6) / (assign29890_e43114 * assign29890_e43114))), (-((locals.var_fs02__blk971_dn7 + locals.var_xi0p12__blk979_dn7) / (assign29890_e43114 * assign29890_e43114))), (-((locals.var_fs02__blk971_dn10 + locals.var_xi0p12__blk979_dn10) / (assign29890_e43114 * assign29890_e43114))), (-((locals.var_fs02__blk971_dn11 + locals.var_xi0p12__blk979_dn11) / (assign29890_e43114 * assign29890_e43114))), (-((locals.var_fs02__blk971_dn12 + locals.var_xi0p12__blk979_dn12) / (assign29890_e43114 * assign29890_e43114))), (-((locals.var_fs02__blk971_dn17 + locals.var_xi0p12__blk979_dn17) / (assign29890_e43114 * assign29890_e43114))),)
    } else {
        (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17,)
    }
};
        locals.var_t1__blk898 = assign29890_e43117;
        locals.var_t1__blk898_dn0 = assign29890_e43117_d_n0;
        locals.var_t1__blk898_dn2 = assign29890_e43117_d_n2;
        locals.var_t1__blk898_dn6 = assign29890_e43117_d_n6;
        locals.var_t1__blk898_dn7 = assign29890_e43117_d_n7;
        locals.var_t1__blk898_dn10 = assign29890_e43117_d_n10;
        locals.var_t1__blk898_dn11 = assign29890_e43117_d_n11;
        locals.var_t1__blk898_dn12 = assign29890_e43117_d_n12;
        locals.var_t1__blk898_dn17 = assign29890_e43117_d_n17;
        locals.var_t1__blk898_rv = 0.0;

        let (assign29900_e43135, assign29900_e43135_d_n0, assign29900_e43135_d_n2, assign29900_e43135_d_n6, assign29900_e43135_d_n7, assign29900_e43135_d_n10, assign29900_e43135_d_n11, assign29900_e43135_d_n12, assign29900_e43135_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29900_e43131: f64 = (locals.var_cnst0over__blk930 * locals.var_fs01__blk967);
        let assign29900_e43133: f64 = (assign29900_e43131 * locals.var_t1__blk898);
        (assign29900_e43133, ((((locals.var_cnst0over__blk930_dn0 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn0)) * locals.var_t1__blk898) + (assign29900_e43131 * locals.var_t1__blk898_dn0)), ((((locals.var_cnst0over__blk930_dn2 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn2)) * locals.var_t1__blk898) + (assign29900_e43131 * locals.var_t1__blk898_dn2)), ((((locals.var_cnst0over__blk930_dn6 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn6)) * locals.var_t1__blk898) + (assign29900_e43131 * locals.var_t1__blk898_dn6)), ((((locals.var_cnst0over__blk930_dn7 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn7)) * locals.var_t1__blk898) + (assign29900_e43131 * locals.var_t1__blk898_dn7)), ((((locals.var_cnst0over__blk930_dn10 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn10)) * locals.var_t1__blk898) + (assign29900_e43131 * locals.var_t1__blk898_dn10)), ((((locals.var_cnst0over__blk930_dn11 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn11)) * locals.var_t1__blk898) + (assign29900_e43131 * locals.var_t1__blk898_dn11)), ((((locals.var_cnst0over__blk930_dn12 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn12)) * locals.var_t1__blk898) + (assign29900_e43131 * locals.var_t1__blk898_dn12)), ((((locals.var_cnst0over__blk930_dn17 * locals.var_fs01__blk967) + (locals.var_cnst0over__blk930 * locals.var_fs01__blk967_dn17)) * locals.var_t1__blk898) + (assign29900_e43131 * locals.var_t1__blk898_dn17)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign29900_e43135;
        locals.var_qiuld_dn0 = assign29900_e43135_d_n0;
        locals.var_qiuld_dn2 = assign29900_e43135_d_n2;
        locals.var_qiuld_dn6 = assign29900_e43135_d_n6;
        locals.var_qiuld_dn7 = assign29900_e43135_d_n7;
        locals.var_qiuld_dn10 = assign29900_e43135_d_n10;
        locals.var_qiuld_dn11 = assign29900_e43135_d_n11;
        locals.var_qiuld_dn12 = assign29900_e43135_d_n12;
        locals.var_qiuld_dn17 = assign29900_e43135_d_n17;
        locals.var_qiuld_rv = 0.0;

        let (assign29910_e43151, assign29910_e43151_d_n0, assign29910_e43151_d_n2, assign29910_e43151_d_n6, assign29910_e43151_d_n7, assign29910_e43151_d_n10, assign29910_e43151_d_n11, assign29910_e43151_d_n12, assign29910_e43151_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard984 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29910_e43149: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign29910_e43149, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), (locals.var_qbuld_dn17 + locals.var_qiuld_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign29910_e43151;
        locals.var_qsuld_dn0 = assign29910_e43151_d_n0;
        locals.var_qsuld_dn2 = assign29910_e43151_d_n2;
        locals.var_qsuld_dn6 = assign29910_e43151_d_n6;
        locals.var_qsuld_dn7 = assign29910_e43151_d_n7;
        locals.var_qsuld_dn10 = assign29910_e43151_d_n10;
        locals.var_qsuld_dn11 = assign29910_e43151_d_n11;
        locals.var_qsuld_dn12 = assign29910_e43151_d_n12;
        locals.var_qsuld_dn17 = assign29910_e43151_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign29920_e43162, assign29920_e43162_d_n0, assign29920_e43162_d_n2, assign29920_e43162_d_n6, assign29920_e43162_d_n7, assign29920_e43162_d_n10, assign29920_e43162_d_n11, assign29920_e43162_d_n12, assign29920_e43162_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign29920_e43160: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign29920_e43160, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), (locals.var_qsuld_dn17 - locals.var_qbuld_dn17),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign29920_e43162;
        locals.var_qiuld_dn0 = assign29920_e43162_d_n0;
        locals.var_qiuld_dn2 = assign29920_e43162_d_n2;
        locals.var_qiuld_dn6 = assign29920_e43162_d_n6;
        locals.var_qiuld_dn7 = assign29920_e43162_d_n7;
        locals.var_qiuld_dn10 = assign29920_e43162_d_n10;
        locals.var_qiuld_dn11 = assign29920_e43162_d_n11;
        locals.var_qiuld_dn12 = assign29920_e43162_d_n12;
        locals.var_qiuld_dn17 = assign29920_e43162_d_n17;
        locals.var_qiuld_rv = 0.0;

        let (assign29930_e43180, assign29930_e43180_d_n0, assign29930_e43180_d_n2, assign29930_e43180_d_n6, assign29930_e43180_d_n7, assign29930_e43180_d_n10, assign29930_e43180_d_n11, assign29930_e43180_d_n12, assign29930_e43180_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let (assign29930_e43178,) = {
            if (p.p43 == 1.0) {
                let assign29930_e43174: f64 = (locals.var_w_dioscv * locals.var_lov);
                (assign29930_e43174,)
            } else {
                let assign29930_e43177: f64 = (locals.var_weffcv_nf * locals.var_lov);
                (assign29930_e43177,)
            }
        };
        (assign29930_e43178, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk901, locals.var_t4__blk901_dn0, locals.var_t4__blk901_dn2, locals.var_t4__blk901_dn6, locals.var_t4__blk901_dn7, locals.var_t4__blk901_dn10, locals.var_t4__blk901_dn11, locals.var_t4__blk901_dn12, locals.var_t4__blk901_dn17,)
    }
};
        locals.var_t4__blk901 = assign29930_e43180;
        locals.var_t4__blk901_dn0 = assign29930_e43180_d_n0;
        locals.var_t4__blk901_dn2 = assign29930_e43180_d_n2;
        locals.var_t4__blk901_dn6 = assign29930_e43180_d_n6;
        locals.var_t4__blk901_dn7 = assign29930_e43180_d_n7;
        locals.var_t4__blk901_dn10 = assign29930_e43180_d_n10;
        locals.var_t4__blk901_dn11 = assign29930_e43180_d_n11;
        locals.var_t4__blk901_dn12 = assign29930_e43180_d_n12;
        locals.var_t4__blk901_dn17 = assign29930_e43180_d_n17;
        locals.var_t4__blk901_rv = 0.0;

        let assign29940_e43191: f64 = if (((locals.var_flg_overs__blk916 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloops__blk914 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard1000 = assign29940_e43191;
        locals.var_guard1000_rv = 0.0;

        let (assign29950_e43204, assign29950_e43204_d_n0, assign29950_e43204_d_n2, assign29950_e43204_d_n6, assign29950_e43204_d_n7, assign29950_e43204_d_n10, assign29950_e43204_d_n11, assign29950_e43204_d_n12, assign29950_e43204_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1000 != 0.0)) {
        let assign29950_e43202: f64 = (locals.var_t4__blk901 * locals.var_qsuld);
        (assign29950_e43202, ((locals.var_t4__blk901_dn0 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn0)), ((locals.var_t4__blk901_dn2 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn2)), ((locals.var_t4__blk901_dn6 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn6)), ((locals.var_t4__blk901_dn7 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn7)), ((locals.var_t4__blk901_dn10 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn10)), ((locals.var_t4__blk901_dn11 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn11)), ((locals.var_t4__blk901_dn12 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn12)), ((locals.var_t4__blk901_dn17 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn17)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn17,)
    }
};
        locals.var_qovs = assign29950_e43204;
        locals.var_qovs_dn0 = assign29950_e43204_d_n0;
        locals.var_qovs_dn2 = assign29950_e43204_d_n2;
        locals.var_qovs_dn6 = assign29950_e43204_d_n6;
        locals.var_qovs_dn7 = assign29950_e43204_d_n7;
        locals.var_qovs_dn10 = assign29950_e43204_d_n10;
        locals.var_qovs_dn11 = assign29950_e43204_d_n11;
        locals.var_qovs_dn12 = assign29950_e43204_d_n12;
        locals.var_qovs_dn17 = assign29950_e43204_d_n17;
        locals.var_qovs_rv = 0.0;

        let (assign29960_e43217, assign29960_e43217_d_n0, assign29960_e43217_d_n2, assign29960_e43217_d_n6, assign29960_e43217_d_n7, assign29960_e43217_d_n10, assign29960_e43217_d_n11, assign29960_e43217_d_n12, assign29960_e43217_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1000 != 0.0)) {
        let assign29960_e43215: f64 = (locals.var_t4__blk901 * locals.var_qbuld);
        (assign29960_e43215, ((locals.var_t4__blk901_dn0 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn0)), ((locals.var_t4__blk901_dn2 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn2)), ((locals.var_t4__blk901_dn6 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn6)), ((locals.var_t4__blk901_dn7 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn7)), ((locals.var_t4__blk901_dn10 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn10)), ((locals.var_t4__blk901_dn11 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn11)), ((locals.var_t4__blk901_dn12 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn12)), ((locals.var_t4__blk901_dn17 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn17)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn10, locals.var_qbsld_dn11, locals.var_qbsld_dn12, locals.var_qbsld_dn17,)
    }
};
        locals.var_qbsld = assign29960_e43217;
        locals.var_qbsld_dn0 = assign29960_e43217_d_n0;
        locals.var_qbsld_dn2 = assign29960_e43217_d_n2;
        locals.var_qbsld_dn6 = assign29960_e43217_d_n6;
        locals.var_qbsld_dn7 = assign29960_e43217_d_n7;
        locals.var_qbsld_dn10 = assign29960_e43217_d_n10;
        locals.var_qbsld_dn11 = assign29960_e43217_d_n11;
        locals.var_qbsld_dn12 = assign29960_e43217_d_n12;
        locals.var_qbsld_dn17 = assign29960_e43217_d_n17;
        locals.var_qbsld_rv = 0.0;

        let assign29970_e43228: f64 = if (((locals.var_flg_overd__blk917 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloopd__blk915 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard1001 = assign29970_e43228;
        locals.var_guard1001_rv = 0.0;

        let (assign29980_e43241, assign29980_e43241_d_n0, assign29980_e43241_d_n2, assign29980_e43241_d_n6, assign29980_e43241_d_n7, assign29980_e43241_d_n10, assign29980_e43241_d_n11, assign29980_e43241_d_n12, assign29980_e43241_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1001 != 0.0)) {
        let assign29980_e43239: f64 = (locals.var_t4__blk901 * locals.var_qsuld);
        (assign29980_e43239, ((locals.var_t4__blk901_dn0 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn0)), ((locals.var_t4__blk901_dn2 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn2)), ((locals.var_t4__blk901_dn6 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn6)), ((locals.var_t4__blk901_dn7 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn7)), ((locals.var_t4__blk901_dn10 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn10)), ((locals.var_t4__blk901_dn11 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn11)), ((locals.var_t4__blk901_dn12 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn12)), ((locals.var_t4__blk901_dn17 * locals.var_qsuld) + (locals.var_t4__blk901 * locals.var_qsuld_dn17)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn17,)
    }
};
        locals.var_qovd = assign29980_e43241;
        locals.var_qovd_dn0 = assign29980_e43241_d_n0;
        locals.var_qovd_dn2 = assign29980_e43241_d_n2;
        locals.var_qovd_dn6 = assign29980_e43241_d_n6;
        locals.var_qovd_dn7 = assign29980_e43241_d_n7;
        locals.var_qovd_dn10 = assign29980_e43241_d_n10;
        locals.var_qovd_dn11 = assign29980_e43241_d_n11;
        locals.var_qovd_dn12 = assign29980_e43241_d_n12;
        locals.var_qovd_dn17 = assign29980_e43241_d_n17;
        locals.var_qovd_rv = 0.0;

        let (assign29990_e43254, assign29990_e43254_d_n0, assign29990_e43254_d_n2, assign29990_e43254_d_n6, assign29990_e43254_d_n7, assign29990_e43254_d_n10, assign29990_e43254_d_n11, assign29990_e43254_d_n12, assign29990_e43254_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1001 != 0.0)) {
        let assign29990_e43252: f64 = (locals.var_t4__blk901 * locals.var_qbuld);
        (assign29990_e43252, ((locals.var_t4__blk901_dn0 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn0)), ((locals.var_t4__blk901_dn2 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn2)), ((locals.var_t4__blk901_dn6 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn6)), ((locals.var_t4__blk901_dn7 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn7)), ((locals.var_t4__blk901_dn10 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn10)), ((locals.var_t4__blk901_dn11 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn11)), ((locals.var_t4__blk901_dn12 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn12)), ((locals.var_t4__blk901_dn17 * locals.var_qbuld) + (locals.var_t4__blk901 * locals.var_qbuld_dn17)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn12, locals.var_qbdld_dn17,)
    }
};
        locals.var_qbdld = assign29990_e43254;
        locals.var_qbdld_dn0 = assign29990_e43254_d_n0;
        locals.var_qbdld_dn2 = assign29990_e43254_d_n2;
        locals.var_qbdld_dn6 = assign29990_e43254_d_n6;
        locals.var_qbdld_dn7 = assign29990_e43254_d_n7;
        locals.var_qbdld_dn10 = assign29990_e43254_d_n10;
        locals.var_qbdld_dn11 = assign29990_e43254_d_n11;
        locals.var_qbdld_dn12 = assign29990_e43254_d_n12;
        locals.var_qbdld_dn17 = assign29990_e43254_d_n17;
        locals.var_qbdld_rv = 0.0;

        let (assign30000_e43267,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign30000_e43263: f64 = (1.0 - 1.0);
        let assign30000_e43265: f64 = (assign30000_e43263 / 2.0);
        (assign30000_e43265,)
    } else {
        (locals.var_flg_ovloops__blk914,)
    }
};
        locals.var_flg_ovloops__blk914 = assign30000_e43267;
        locals.var_flg_ovloops__blk914_rv = 0.0;

        let (assign30010_e43280,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign30010_e43276: f64 = (1.0 + 1.0);
        let assign30010_e43278: f64 = (assign30010_e43276 / 2.0);
        (assign30010_e43278,)
    } else {
        (locals.var_flg_ovloopd__blk915,)
    }
};
        locals.var_flg_ovloopd__blk915 = assign30010_e43280;
        locals.var_flg_ovloopd__blk915_rv = 0.0;

        let assign30020_e43283: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1002 = assign30020_e43283;
        locals.var_guard1002_rv = 0.0;

        let (assign30030_e43302, assign30030_e43302_d_n0, assign30030_e43302_d_n2, assign30030_e43302_d_n6, assign30030_e43302_d_n7, assign30030_e43302_d_n10, assign30030_e43302_d_n11, assign30030_e43302_d_n12, assign30030_e43302_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30030_e43294: f64 = (locals.var_modenml * locals.var_vbs);
        let assign30030_e43298: f64 = (locals.var_vbs - locals.var_vds);
        let assign30030_e43299: f64 = (locals.var_modervs * assign30030_e43298);
        let assign30030_e43300: f64 = (assign30030_e43294 + assign30030_e43299);
        (assign30030_e43300, ((locals.var_modenml * locals.var_vbs_dn0) + (locals.var_modervs * (locals.var_vbs_dn0 - locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vbs_dn2) + (locals.var_modervs * (locals.var_vbs_dn2 - locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vbs_dn6) + (locals.var_modervs * (locals.var_vbs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vbs_dn7) + (locals.var_modervs * (locals.var_vbs_dn7 - locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vbs_dn10) + (locals.var_modervs * (locals.var_vbs_dn10 - locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vbs_dn11) + (locals.var_modervs * (locals.var_vbs_dn11 - locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vbs_dn12) + (locals.var_modervs * (locals.var_vbs_dn12 - locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vbs_dn17) + (locals.var_modervs * (locals.var_vbs_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_vbsgmt__blk924, locals.var_vbsgmt__blk924_dn0, locals.var_vbsgmt__blk924_dn2, locals.var_vbsgmt__blk924_dn6, locals.var_vbsgmt__blk924_dn7, locals.var_vbsgmt__blk924_dn10, locals.var_vbsgmt__blk924_dn11, locals.var_vbsgmt__blk924_dn12, locals.var_vbsgmt__blk924_dn17,)
    }
};
        locals.var_vbsgmt__blk924 = assign30030_e43302;
        locals.var_vbsgmt__blk924_dn0 = assign30030_e43302_d_n0;
        locals.var_vbsgmt__blk924_dn2 = assign30030_e43302_d_n2;
        locals.var_vbsgmt__blk924_dn6 = assign30030_e43302_d_n6;
        locals.var_vbsgmt__blk924_dn7 = assign30030_e43302_d_n7;
        locals.var_vbsgmt__blk924_dn10 = assign30030_e43302_d_n10;
        locals.var_vbsgmt__blk924_dn11 = assign30030_e43302_d_n11;
        locals.var_vbsgmt__blk924_dn12 = assign30030_e43302_d_n12;
        locals.var_vbsgmt__blk924_dn17 = assign30030_e43302_d_n17;
        locals.var_vbsgmt__blk924_rv = 0.0;

        let (assign30040_e43320, assign30040_e43320_d_n0, assign30040_e43320_d_n2, assign30040_e43320_d_n6, assign30040_e43320_d_n7, assign30040_e43320_d_n10, assign30040_e43320_d_n11, assign30040_e43320_d_n12, assign30040_e43320_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30040_e43313: f64 = (locals.var_modenml * locals.var_vds);
        let assign30040_e43316: f64 = (-locals.var_vds);
        let assign30040_e43317: f64 = (locals.var_modervs * assign30040_e43316);
        let assign30040_e43318: f64 = (assign30040_e43313 + assign30040_e43317);
        (assign30040_e43318, ((locals.var_modenml * locals.var_vds_dn0) + (locals.var_modervs * (-locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vds_dn2) + (locals.var_modervs * (-locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vds_dn6) + (locals.var_modervs * (-locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vds_dn7) + (locals.var_modervs * (-locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vds_dn10) + (locals.var_modervs * (-locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vds_dn11) + (locals.var_modervs * (-locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vds_dn12) + (locals.var_modervs * (-locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vds_dn17) + (locals.var_modervs * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_vdsgmt__blk925, locals.var_vdsgmt__blk925_dn0, locals.var_vdsgmt__blk925_dn2, locals.var_vdsgmt__blk925_dn6, locals.var_vdsgmt__blk925_dn7, locals.var_vdsgmt__blk925_dn10, locals.var_vdsgmt__blk925_dn11, locals.var_vdsgmt__blk925_dn12, locals.var_vdsgmt__blk925_dn17,)
    }
};
        locals.var_vdsgmt__blk925 = assign30040_e43320;
        locals.var_vdsgmt__blk925_dn0 = assign30040_e43320_d_n0;
        locals.var_vdsgmt__blk925_dn2 = assign30040_e43320_d_n2;
        locals.var_vdsgmt__blk925_dn6 = assign30040_e43320_d_n6;
        locals.var_vdsgmt__blk925_dn7 = assign30040_e43320_d_n7;
        locals.var_vdsgmt__blk925_dn10 = assign30040_e43320_d_n10;
        locals.var_vdsgmt__blk925_dn11 = assign30040_e43320_d_n11;
        locals.var_vdsgmt__blk925_dn12 = assign30040_e43320_d_n12;
        locals.var_vdsgmt__blk925_dn17 = assign30040_e43320_d_n17;
        locals.var_vdsgmt__blk925_rv = 0.0;

        let (assign30050_e43339, assign30050_e43339_d_n0, assign30050_e43339_d_n2, assign30050_e43339_d_n6, assign30050_e43339_d_n7, assign30050_e43339_d_n10, assign30050_e43339_d_n11, assign30050_e43339_d_n12, assign30050_e43339_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30050_e43331: f64 = (locals.var_modenml * locals.var_vgs);
        let assign30050_e43335: f64 = (locals.var_vgs - locals.var_vds);
        let assign30050_e43336: f64 = (locals.var_modervs * assign30050_e43335);
        let assign30050_e43337: f64 = (assign30050_e43331 + assign30050_e43336);
        (assign30050_e43337, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgsgmt__blk926, locals.var_vgsgmt__blk926_dn0, locals.var_vgsgmt__blk926_dn2, locals.var_vgsgmt__blk926_dn6, locals.var_vgsgmt__blk926_dn7, locals.var_vgsgmt__blk926_dn10, locals.var_vgsgmt__blk926_dn11, locals.var_vgsgmt__blk926_dn12, locals.var_vgsgmt__blk926_dn17,)
    }
};
        locals.var_vgsgmt__blk926 = assign30050_e43339;
        locals.var_vgsgmt__blk926_dn0 = assign30050_e43339_d_n0;
        locals.var_vgsgmt__blk926_dn2 = assign30050_e43339_d_n2;
        locals.var_vgsgmt__blk926_dn6 = assign30050_e43339_d_n6;
        locals.var_vgsgmt__blk926_dn7 = assign30050_e43339_d_n7;
        locals.var_vgsgmt__blk926_dn10 = assign30050_e43339_d_n10;
        locals.var_vgsgmt__blk926_dn11 = assign30050_e43339_d_n11;
        locals.var_vgsgmt__blk926_dn12 = assign30050_e43339_d_n12;
        locals.var_vgsgmt__blk926_dn17 = assign30050_e43339_d_n17;
        locals.var_vgsgmt__blk926_rv = 0.0;

        let (assign30060_e43352, assign30060_e43352_d_n0, assign30060_e43352_d_n2, assign30060_e43352_d_n6, assign30060_e43352_d_n7, assign30060_e43352_d_n10, assign30060_e43352_d_n11, assign30060_e43352_d_n12, assign30060_e43352_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30060_e43350: f64 = (locals.var_vdsgmt__blk925 - locals.var_vbsgmt__blk924);
        (assign30060_e43350, (locals.var_vdsgmt__blk925_dn0 - locals.var_vbsgmt__blk924_dn0), (locals.var_vdsgmt__blk925_dn2 - locals.var_vbsgmt__blk924_dn2), (locals.var_vdsgmt__blk925_dn6 - locals.var_vbsgmt__blk924_dn6), (locals.var_vdsgmt__blk925_dn7 - locals.var_vbsgmt__blk924_dn7), (locals.var_vdsgmt__blk925_dn10 - locals.var_vbsgmt__blk924_dn10), (locals.var_vdsgmt__blk925_dn11 - locals.var_vbsgmt__blk924_dn11), (locals.var_vdsgmt__blk925_dn12 - locals.var_vbsgmt__blk924_dn12), (locals.var_vdsgmt__blk925_dn17 - locals.var_vbsgmt__blk924_dn17),)
    } else {
        (locals.var_vdbgmt__blk927, locals.var_vdbgmt__blk927_dn0, locals.var_vdbgmt__blk927_dn2, locals.var_vdbgmt__blk927_dn6, locals.var_vdbgmt__blk927_dn7, locals.var_vdbgmt__blk927_dn10, locals.var_vdbgmt__blk927_dn11, locals.var_vdbgmt__blk927_dn12, locals.var_vdbgmt__blk927_dn17,)
    }
};
        locals.var_vdbgmt__blk927 = assign30060_e43352;
        locals.var_vdbgmt__blk927_dn0 = assign30060_e43352_d_n0;
        locals.var_vdbgmt__blk927_dn2 = assign30060_e43352_d_n2;
        locals.var_vdbgmt__blk927_dn6 = assign30060_e43352_d_n6;
        locals.var_vdbgmt__blk927_dn7 = assign30060_e43352_d_n7;
        locals.var_vdbgmt__blk927_dn10 = assign30060_e43352_d_n10;
        locals.var_vdbgmt__blk927_dn11 = assign30060_e43352_d_n11;
        locals.var_vdbgmt__blk927_dn12 = assign30060_e43352_d_n12;
        locals.var_vdbgmt__blk927_dn17 = assign30060_e43352_d_n17;
        locals.var_vdbgmt__blk927_rv = 0.0;

        let (assign30070_e43365, assign30070_e43365_d_n0, assign30070_e43365_d_n2, assign30070_e43365_d_n6, assign30070_e43365_d_n7, assign30070_e43365_d_n10, assign30070_e43365_d_n11, assign30070_e43365_d_n12, assign30070_e43365_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30070_e43363: f64 = (locals.var_vgsgmt__blk926 - locals.var_vbsgmt__blk924);
        (assign30070_e43363, (locals.var_vgsgmt__blk926_dn0 - locals.var_vbsgmt__blk924_dn0), (locals.var_vgsgmt__blk926_dn2 - locals.var_vbsgmt__blk924_dn2), (locals.var_vgsgmt__blk926_dn6 - locals.var_vbsgmt__blk924_dn6), (locals.var_vgsgmt__blk926_dn7 - locals.var_vbsgmt__blk924_dn7), (locals.var_vgsgmt__blk926_dn10 - locals.var_vbsgmt__blk924_dn10), (locals.var_vgsgmt__blk926_dn11 - locals.var_vbsgmt__blk924_dn11), (locals.var_vgsgmt__blk926_dn12 - locals.var_vbsgmt__blk924_dn12), (locals.var_vgsgmt__blk926_dn17 - locals.var_vbsgmt__blk924_dn17),)
    } else {
        (locals.var_vgbgmt__blk929, locals.var_vgbgmt__blk929_dn0, locals.var_vgbgmt__blk929_dn2, locals.var_vgbgmt__blk929_dn6, locals.var_vgbgmt__blk929_dn7, locals.var_vgbgmt__blk929_dn10, locals.var_vgbgmt__blk929_dn11, locals.var_vgbgmt__blk929_dn12, locals.var_vgbgmt__blk929_dn17,)
    }
};
        locals.var_vgbgmt__blk929 = assign30070_e43365;
        locals.var_vgbgmt__blk929_dn0 = assign30070_e43365_d_n0;
        locals.var_vgbgmt__blk929_dn2 = assign30070_e43365_d_n2;
        locals.var_vgbgmt__blk929_dn6 = assign30070_e43365_d_n6;
        locals.var_vgbgmt__blk929_dn7 = assign30070_e43365_d_n7;
        locals.var_vgbgmt__blk929_dn10 = assign30070_e43365_d_n10;
        locals.var_vgbgmt__blk929_dn11 = assign30070_e43365_d_n11;
        locals.var_vgbgmt__blk929_dn12 = assign30070_e43365_d_n12;
        locals.var_vgbgmt__blk929_dn17 = assign30070_e43365_d_n17;
        locals.var_vgbgmt__blk929_rv = 0.0;

        let (assign30080_e43377, assign30080_e43377_d_n0, assign30080_e43377_d_n2, assign30080_e43377_d_n6, assign30080_e43377_d_n7, assign30080_e43377_d_n10, assign30080_e43377_d_n11, assign30080_e43377_d_n12, assign30080_e43377_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30080_e43375: f64 = (-locals.var_vbsgmt__blk924);
        (assign30080_e43375, (-locals.var_vbsgmt__blk924_dn0), (-locals.var_vbsgmt__blk924_dn2), (-locals.var_vbsgmt__blk924_dn6), (-locals.var_vbsgmt__blk924_dn7), (-locals.var_vbsgmt__blk924_dn10), (-locals.var_vbsgmt__blk924_dn11), (-locals.var_vbsgmt__blk924_dn12), (-locals.var_vbsgmt__blk924_dn17),)
    } else {
        (locals.var_vsbgmt__blk928, locals.var_vsbgmt__blk928_dn0, locals.var_vsbgmt__blk928_dn2, locals.var_vsbgmt__blk928_dn6, locals.var_vsbgmt__blk928_dn7, locals.var_vsbgmt__blk928_dn10, locals.var_vsbgmt__blk928_dn11, locals.var_vsbgmt__blk928_dn12, locals.var_vsbgmt__blk928_dn17,)
    }
};
        locals.var_vsbgmt__blk928 = assign30080_e43377;
        locals.var_vsbgmt__blk928_dn0 = assign30080_e43377_d_n0;
        locals.var_vsbgmt__blk928_dn2 = assign30080_e43377_d_n2;
        locals.var_vsbgmt__blk928_dn6 = assign30080_e43377_d_n6;
        locals.var_vsbgmt__blk928_dn7 = assign30080_e43377_d_n7;
        locals.var_vsbgmt__blk928_dn10 = assign30080_e43377_d_n10;
        locals.var_vsbgmt__blk928_dn11 = assign30080_e43377_d_n11;
        locals.var_vsbgmt__blk928_dn12 = assign30080_e43377_d_n12;
        locals.var_vsbgmt__blk928_dn17 = assign30080_e43377_d_n17;
        locals.var_vsbgmt__blk928_rv = 0.0;

        let (assign30090_e43394,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30090_e43388: f64 = (locals.var_flg_ovloops__blk914 * locals.var_modenml);
        let assign30090_e43391: f64 = (locals.var_flg_ovloopd__blk915 * locals.var_modervs);
        let assign30090_e43392: f64 = (assign30090_e43388 + assign30090_e43391);
        (assign30090_e43392,)
    } else {
        (locals.var_flg_overs__blk916,)
    }
};
        locals.var_flg_overs__blk916 = assign30090_e43394;
        locals.var_flg_overs__blk916_rv = 0.0;

        let (assign30100_e43411,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30100_e43405: f64 = (locals.var_flg_ovloops__blk914 * locals.var_modervs);
        let assign30100_e43408: f64 = (locals.var_flg_ovloopd__blk915 * locals.var_modenml);
        let assign30100_e43409: f64 = (assign30100_e43405 + assign30100_e43408);
        (assign30100_e43409,)
    } else {
        (locals.var_flg_overd__blk917,)
    }
};
        locals.var_flg_overd__blk917 = assign30100_e43411;
        locals.var_flg_overd__blk917_rv = 0.0;

        let (assign30110_e43432, assign30110_e43432_d_n0, assign30110_e43432_d_n2, assign30110_e43432_d_n6, assign30110_e43432_d_n7, assign30110_e43432_d_n10, assign30110_e43432_d_n11, assign30110_e43432_d_n12, assign30110_e43432_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30110_e43422: f64 = (locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928);
        let assign30110_e43425: f64 = (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927);
        let assign30110_e43426: f64 = (assign30110_e43422 + assign30110_e43425);
        let assign30110_e43429: f64 = (10.0 * 2.220446049250313e-16);
        let assign30110_e43430: f64 = (assign30110_e43426 + assign30110_e43429);
        (assign30110_e43430, ((locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928_dn0) + (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927_dn0)), ((locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928_dn2) + (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927_dn2)), ((locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928_dn6) + (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927_dn6)), ((locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928_dn7) + (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927_dn7)), ((locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928_dn10) + (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927_dn10)), ((locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928_dn11) + (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927_dn11)), ((locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928_dn12) + (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927_dn12)), ((locals.var_flg_overs__blk916 * locals.var_vsbgmt__blk928_dn17) + (locals.var_flg_overd__blk917 * locals.var_vdbgmt__blk927_dn17)),)
    } else {
        (locals.var_vxbgmt__blk922, locals.var_vxbgmt__blk922_dn0, locals.var_vxbgmt__blk922_dn2, locals.var_vxbgmt__blk922_dn6, locals.var_vxbgmt__blk922_dn7, locals.var_vxbgmt__blk922_dn10, locals.var_vxbgmt__blk922_dn11, locals.var_vxbgmt__blk922_dn12, locals.var_vxbgmt__blk922_dn17,)
    }
};
        locals.var_vxbgmt__blk922 = assign30110_e43432;
        locals.var_vxbgmt__blk922_dn0 = assign30110_e43432_d_n0;
        locals.var_vxbgmt__blk922_dn2 = assign30110_e43432_d_n2;
        locals.var_vxbgmt__blk922_dn6 = assign30110_e43432_d_n6;
        locals.var_vxbgmt__blk922_dn7 = assign30110_e43432_d_n7;
        locals.var_vxbgmt__blk922_dn10 = assign30110_e43432_d_n10;
        locals.var_vxbgmt__blk922_dn11 = assign30110_e43432_d_n11;
        locals.var_vxbgmt__blk922_dn12 = assign30110_e43432_d_n12;
        locals.var_vxbgmt__blk922_dn17 = assign30110_e43432_d_n17;
        locals.var_vxbgmt__blk922_rv = 0.0;

        let (assign30120_e43450,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign30120_e43444: f64 = (locals.var_flg_ovloops__blk914 * locals.var_modenml);
        let assign30120_e43447: f64 = (locals.var_flg_ovloopd__blk915 * locals.var_modervs);
        let assign30120_e43448: f64 = (assign30120_e43444 + assign30120_e43447);
        (assign30120_e43448,)
    } else {
        (locals.var_flg_overs__blk916,)
    }
};
        locals.var_flg_overs__blk916 = assign30120_e43450;
        locals.var_flg_overs__blk916_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_110(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30130_e43468,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign30130_e43462: f64 = (locals.var_flg_ovloops__blk914 * locals.var_modervs);
        let assign30130_e43465: f64 = (locals.var_flg_ovloopd__blk915 * locals.var_modenml);
        let assign30130_e43466: f64 = (assign30130_e43462 + assign30130_e43465);
        (assign30130_e43466,)
    } else {
        (locals.var_flg_overd__blk917,)
    }
};
        locals.var_flg_overd__blk917 = assign30130_e43468;
        locals.var_flg_overd__blk917_rv = 0.0;

        let (assign30140_e43490, assign30140_e43490_d_n0, assign30140_e43490_d_n2, assign30140_e43490_d_n6, assign30140_e43490_d_n7, assign30140_e43490_d_n10, assign30140_e43490_d_n11, assign30140_e43490_d_n12, assign30140_e43490_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_flg_ovloops__blk914 != 0.0)) {
        let assign30140_e43482: f64 = (locals.var_modenml * locals.var_vgs);
        let assign30140_e43486: f64 = (locals.var_vgs - locals.var_vds);
        let assign30140_e43487: f64 = (locals.var_modervs * assign30140_e43486);
        let assign30140_e43488: f64 = (assign30140_e43482 + assign30140_e43487);
        (assign30140_e43488, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgbgmt__blk929, locals.var_vgbgmt__blk929_dn0, locals.var_vgbgmt__blk929_dn2, locals.var_vgbgmt__blk929_dn6, locals.var_vgbgmt__blk929_dn7, locals.var_vgbgmt__blk929_dn10, locals.var_vgbgmt__blk929_dn11, locals.var_vgbgmt__blk929_dn12, locals.var_vgbgmt__blk929_dn17,)
    }
};
        locals.var_vgbgmt__blk929 = assign30140_e43490;
        locals.var_vgbgmt__blk929_dn0 = assign30140_e43490_d_n0;
        locals.var_vgbgmt__blk929_dn2 = assign30140_e43490_d_n2;
        locals.var_vgbgmt__blk929_dn6 = assign30140_e43490_d_n6;
        locals.var_vgbgmt__blk929_dn7 = assign30140_e43490_d_n7;
        locals.var_vgbgmt__blk929_dn10 = assign30140_e43490_d_n10;
        locals.var_vgbgmt__blk929_dn11 = assign30140_e43490_d_n11;
        locals.var_vgbgmt__blk929_dn12 = assign30140_e43490_d_n12;
        locals.var_vgbgmt__blk929_dn17 = assign30140_e43490_d_n17;
        locals.var_vgbgmt__blk929_rv = 0.0;

        let (assign30150_e43512, assign30150_e43512_d_n0, assign30150_e43512_d_n2, assign30150_e43512_d_n6, assign30150_e43512_d_n7, assign30150_e43512_d_n10, assign30150_e43512_d_n11, assign30150_e43512_d_n12, assign30150_e43512_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_flg_ovloopd__blk915 != 0.0)) {
        let assign30150_e43504: f64 = (locals.var_modervs * locals.var_vgs);
        let assign30150_e43508: f64 = (locals.var_vgs - locals.var_vds);
        let assign30150_e43509: f64 = (locals.var_modenml * assign30150_e43508);
        let assign30150_e43510: f64 = (assign30150_e43504 + assign30150_e43509);
        (assign30150_e43510, (locals.var_modenml * (-locals.var_vds_dn0)), (locals.var_modenml * (-locals.var_vds_dn2)), ((locals.var_modervs * locals.var_vgs_dn6) + (locals.var_modenml * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modervs * locals.var_vgs_dn7) + (locals.var_modenml * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modenml * (-locals.var_vds_dn10)), ((locals.var_modervs * locals.var_vgs_dn11) + (locals.var_modenml * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modenml * (-locals.var_vds_dn12)), (locals.var_modenml * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgbgmt__blk929, locals.var_vgbgmt__blk929_dn0, locals.var_vgbgmt__blk929_dn2, locals.var_vgbgmt__blk929_dn6, locals.var_vgbgmt__blk929_dn7, locals.var_vgbgmt__blk929_dn10, locals.var_vgbgmt__blk929_dn11, locals.var_vgbgmt__blk929_dn12, locals.var_vgbgmt__blk929_dn17,)
    }
};
        locals.var_vgbgmt__blk929 = assign30150_e43512;
        locals.var_vgbgmt__blk929_dn0 = assign30150_e43512_d_n0;
        locals.var_vgbgmt__blk929_dn2 = assign30150_e43512_d_n2;
        locals.var_vgbgmt__blk929_dn6 = assign30150_e43512_d_n6;
        locals.var_vgbgmt__blk929_dn7 = assign30150_e43512_d_n7;
        locals.var_vgbgmt__blk929_dn10 = assign30150_e43512_d_n10;
        locals.var_vgbgmt__blk929_dn11 = assign30150_e43512_d_n11;
        locals.var_vgbgmt__blk929_dn12 = assign30150_e43512_d_n12;
        locals.var_vgbgmt__blk929_dn17 = assign30150_e43512_d_n17;
        locals.var_vgbgmt__blk929_rv = 0.0;

        let (assign30160_e43524, assign30160_e43524_d_n0, assign30160_e43524_d_n2, assign30160_e43524_d_n6, assign30160_e43524_d_n7, assign30160_e43524_d_n10, assign30160_e43524_d_n11, assign30160_e43524_d_n12, assign30160_e43524_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt__blk922, locals.var_vxbgmt__blk922_dn0, locals.var_vxbgmt__blk922_dn2, locals.var_vxbgmt__blk922_dn6, locals.var_vxbgmt__blk922_dn7, locals.var_vxbgmt__blk922_dn10, locals.var_vxbgmt__blk922_dn11, locals.var_vxbgmt__blk922_dn12, locals.var_vxbgmt__blk922_dn17,)
    }
};
        locals.var_vxbgmt__blk922 = assign30160_e43524;
        locals.var_vxbgmt__blk922_dn0 = assign30160_e43524_d_n0;
        locals.var_vxbgmt__blk922_dn2 = assign30160_e43524_d_n2;
        locals.var_vxbgmt__blk922_dn6 = assign30160_e43524_d_n6;
        locals.var_vxbgmt__blk922_dn7 = assign30160_e43524_d_n7;
        locals.var_vxbgmt__blk922_dn10 = assign30160_e43524_d_n10;
        locals.var_vxbgmt__blk922_dn11 = assign30160_e43524_d_n11;
        locals.var_vxbgmt__blk922_dn12 = assign30160_e43524_d_n12;
        locals.var_vxbgmt__blk922_dn17 = assign30160_e43524_d_n17;
        locals.var_vxbgmt__blk922_rv = 0.0;

        let (assign30170_e43534, assign30170_e43534_d_n0, assign30170_e43534_d_n2, assign30170_e43534_d_n6, assign30170_e43534_d_n7, assign30170_e43534_d_n10, assign30170_e43534_d_n11, assign30170_e43534_d_n12, assign30170_e43534_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign30170_e43532: f64 = (-locals.var_vxbgmt__blk922);
        (assign30170_e43532, (-locals.var_vxbgmt__blk922_dn0), (-locals.var_vxbgmt__blk922_dn2), (-locals.var_vxbgmt__blk922_dn6), (-locals.var_vxbgmt__blk922_dn7), (-locals.var_vxbgmt__blk922_dn10), (-locals.var_vxbgmt__blk922_dn11), (-locals.var_vxbgmt__blk922_dn12), (-locals.var_vxbgmt__blk922_dn17),)
    } else {
        (locals.var_t0__blk897, locals.var_t0__blk897_dn0, locals.var_t0__blk897_dn2, locals.var_t0__blk897_dn6, locals.var_t0__blk897_dn7, locals.var_t0__blk897_dn10, locals.var_t0__blk897_dn11, locals.var_t0__blk897_dn12, locals.var_t0__blk897_dn17,)
    }
};
        locals.var_t0__blk897 = assign30170_e43534;
        locals.var_t0__blk897_dn0 = assign30170_e43534_d_n0;
        locals.var_t0__blk897_dn2 = assign30170_e43534_d_n2;
        locals.var_t0__blk897_dn6 = assign30170_e43534_d_n6;
        locals.var_t0__blk897_dn7 = assign30170_e43534_d_n7;
        locals.var_t0__blk897_dn10 = assign30170_e43534_d_n10;
        locals.var_t0__blk897_dn11 = assign30170_e43534_d_n11;
        locals.var_t0__blk897_dn12 = assign30170_e43534_d_n12;
        locals.var_t0__blk897_dn17 = assign30170_e43534_d_n17;
        locals.var_t0__blk897_rv = 0.0;

        let assign30180_e43537: f64 = if locals.var_t0__blk897 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard1003 = assign30180_e43537;
        locals.var_guard1003_rv = 0.0;

        let (assign30190_e43550, assign30190_e43550_d_n0, assign30190_e43550_d_n2, assign30190_e43550_d_n6, assign30190_e43550_d_n7, assign30190_e43550_d_n10, assign30190_e43550_d_n11, assign30190_e43550_d_n12, assign30190_e43550_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1003 != 0.0)) {
        let assign30190_e43548: f64 = (locals.var_t0__blk897 - locals.var_vbs_bnd);
        (assign30190_e43548, locals.var_t0__blk897_dn0, locals.var_t0__blk897_dn2, locals.var_t0__blk897_dn6, locals.var_t0__blk897_dn7, locals.var_t0__blk897_dn10, locals.var_t0__blk897_dn11, locals.var_t0__blk897_dn12, locals.var_t0__blk897_dn17,)
    } else {
        (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17,)
    }
};
        locals.var_t1__blk898 = assign30190_e43550;
        locals.var_t1__blk898_dn0 = assign30190_e43550_d_n0;
        locals.var_t1__blk898_dn2 = assign30190_e43550_d_n2;
        locals.var_t1__blk898_dn6 = assign30190_e43550_d_n6;
        locals.var_t1__blk898_dn7 = assign30190_e43550_d_n7;
        locals.var_t1__blk898_dn10 = assign30190_e43550_d_n10;
        locals.var_t1__blk898_dn11 = assign30190_e43550_d_n11;
        locals.var_t1__blk898_dn12 = assign30190_e43550_d_n12;
        locals.var_t1__blk898_dn17 = assign30190_e43550_d_n17;
        locals.var_t1__blk898_rv = 0.0;

        let (assign30200_e43563, assign30200_e43563_d_n0, assign30200_e43563_d_n2, assign30200_e43563_d_n6, assign30200_e43563_d_n7, assign30200_e43563_d_n10, assign30200_e43563_d_n11, assign30200_e43563_d_n12, assign30200_e43563_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1003 != 0.0)) {
        let assign30200_e43561: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign30200_e43561, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk899, locals.var_t2__blk899_dn0, locals.var_t2__blk899_dn2, locals.var_t2__blk899_dn6, locals.var_t2__blk899_dn7, locals.var_t2__blk899_dn10, locals.var_t2__blk899_dn11, locals.var_t2__blk899_dn12, locals.var_t2__blk899_dn17,)
    }
};
        locals.var_t2__blk899 = assign30200_e43563;
        locals.var_t2__blk899_dn0 = assign30200_e43563_d_n0;
        locals.var_t2__blk899_dn2 = assign30200_e43563_d_n2;
        locals.var_t2__blk899_dn6 = assign30200_e43563_d_n6;
        locals.var_t2__blk899_dn7 = assign30200_e43563_d_n7;
        locals.var_t2__blk899_dn10 = assign30200_e43563_d_n10;
        locals.var_t2__blk899_dn11 = assign30200_e43563_d_n11;
        locals.var_t2__blk899_dn12 = assign30200_e43563_d_n12;
        locals.var_t2__blk899_dn17 = assign30200_e43563_d_n17;
        locals.var_t2__blk899_rv = 0.0;

        let (assign30210_e43576, assign30210_e43576_d_n0, assign30210_e43576_d_n2, assign30210_e43576_d_n6, assign30210_e43576_d_n7, assign30210_e43576_d_n10, assign30210_e43576_d_n11, assign30210_e43576_d_n12, assign30210_e43576_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1003 != 0.0)) {
        let assign30210_e43574: f64 = (locals.var_t1__blk898 / locals.var_t2__blk899);
        (assign30210_e43574, (((locals.var_t1__blk898_dn0 * locals.var_t2__blk899) - (locals.var_t1__blk898 * locals.var_t2__blk899_dn0)) / (locals.var_t2__blk899 * locals.var_t2__blk899)), (((locals.var_t1__blk898_dn2 * locals.var_t2__blk899) - (locals.var_t1__blk898 * locals.var_t2__blk899_dn2)) / (locals.var_t2__blk899 * locals.var_t2__blk899)), (((locals.var_t1__blk898_dn6 * locals.var_t2__blk899) - (locals.var_t1__blk898 * locals.var_t2__blk899_dn6)) / (locals.var_t2__blk899 * locals.var_t2__blk899)), (((locals.var_t1__blk898_dn7 * locals.var_t2__blk899) - (locals.var_t1__blk898 * locals.var_t2__blk899_dn7)) / (locals.var_t2__blk899 * locals.var_t2__blk899)), (((locals.var_t1__blk898_dn10 * locals.var_t2__blk899) - (locals.var_t1__blk898 * locals.var_t2__blk899_dn10)) / (locals.var_t2__blk899 * locals.var_t2__blk899)), (((locals.var_t1__blk898_dn11 * locals.var_t2__blk899) - (locals.var_t1__blk898 * locals.var_t2__blk899_dn11)) / (locals.var_t2__blk899 * locals.var_t2__blk899)), (((locals.var_t1__blk898_dn12 * locals.var_t2__blk899) - (locals.var_t1__blk898 * locals.var_t2__blk899_dn12)) / (locals.var_t2__blk899 * locals.var_t2__blk899)), (((locals.var_t1__blk898_dn17 * locals.var_t2__blk899) - (locals.var_t1__blk898 * locals.var_t2__blk899_dn17)) / (locals.var_t2__blk899 * locals.var_t2__blk899)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign30210_e43576;
        locals.var_tmf1_dn0 = assign30210_e43576_d_n0;
        locals.var_tmf1_dn2 = assign30210_e43576_d_n2;
        locals.var_tmf1_dn6 = assign30210_e43576_d_n6;
        locals.var_tmf1_dn7 = assign30210_e43576_d_n7;
        locals.var_tmf1_dn10 = assign30210_e43576_d_n10;
        locals.var_tmf1_dn11 = assign30210_e43576_d_n11;
        locals.var_tmf1_dn12 = assign30210_e43576_d_n12;
        locals.var_tmf1_dn17 = assign30210_e43576_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign30220_e43589, assign30220_e43589_d_n0, assign30220_e43589_d_n2, assign30220_e43589_d_n6, assign30220_e43589_d_n7, assign30220_e43589_d_n10, assign30220_e43589_d_n11, assign30220_e43589_d_n12, assign30220_e43589_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1003 != 0.0)) {
        let assign30220_e43587: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign30220_e43587, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign30220_e43589;
        locals.var_tmf2_dn0 = assign30220_e43589_d_n0;
        locals.var_tmf2_dn2 = assign30220_e43589_d_n2;
        locals.var_tmf2_dn6 = assign30220_e43589_d_n6;
        locals.var_tmf2_dn7 = assign30220_e43589_d_n7;
        locals.var_tmf2_dn10 = assign30220_e43589_d_n10;
        locals.var_tmf2_dn11 = assign30220_e43589_d_n11;
        locals.var_tmf2_dn12 = assign30220_e43589_d_n12;
        locals.var_tmf2_dn17 = assign30220_e43589_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign30230_e43602, assign30230_e43602_d_n0, assign30230_e43602_d_n2, assign30230_e43602_d_n6, assign30230_e43602_d_n7, assign30230_e43602_d_n10, assign30230_e43602_d_n11, assign30230_e43602_d_n12, assign30230_e43602_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1003 != 0.0)) {
        let assign30230_e43600: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign30230_e43600, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12, locals.var_tmf3_dn17,)
    }
};
        locals.var_tmf3 = assign30230_e43602;
        locals.var_tmf3_dn0 = assign30230_e43602_d_n0;
        locals.var_tmf3_dn2 = assign30230_e43602_d_n2;
        locals.var_tmf3_dn6 = assign30230_e43602_d_n6;
        locals.var_tmf3_dn7 = assign30230_e43602_d_n7;
        locals.var_tmf3_dn10 = assign30230_e43602_d_n10;
        locals.var_tmf3_dn11 = assign30230_e43602_d_n11;
        locals.var_tmf3_dn12 = assign30230_e43602_d_n12;
        locals.var_tmf3_dn17 = assign30230_e43602_d_n17;
        locals.var_tmf3_rv = 0.0;

        let (assign30240_e43615, assign30240_e43615_d_n0, assign30240_e43615_d_n2, assign30240_e43615_d_n6, assign30240_e43615_d_n7, assign30240_e43615_d_n10, assign30240_e43615_d_n11, assign30240_e43615_d_n12, assign30240_e43615_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1003 != 0.0)) {
        let assign30240_e43613: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign30240_e43613, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn17)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn12, locals.var_tmf4_dn17,)
    }
};
        locals.var_tmf4 = assign30240_e43615;
        locals.var_tmf4_dn0 = assign30240_e43615_d_n0;
        locals.var_tmf4_dn2 = assign30240_e43615_d_n2;
        locals.var_tmf4_dn6 = assign30240_e43615_d_n6;
        locals.var_tmf4_dn7 = assign30240_e43615_d_n7;
        locals.var_tmf4_dn10 = assign30240_e43615_d_n10;
        locals.var_tmf4_dn11 = assign30240_e43615_d_n11;
        locals.var_tmf4_dn12 = assign30240_e43615_d_n12;
        locals.var_tmf4_dn17 = assign30240_e43615_d_n17;
        locals.var_tmf4_rv = 0.0;

        let (assign30250_e43636, assign30250_e43636_d_n0, assign30250_e43636_d_n2, assign30250_e43636_d_n6, assign30250_e43636_d_n7, assign30250_e43636_d_n10, assign30250_e43636_d_n11, assign30250_e43636_d_n12, assign30250_e43636_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1003 != 0.0)) {
        let assign30250_e43627: f64 = (1.0 + locals.var_tmf1);
        let assign30250_e43629: f64 = (assign30250_e43627 + locals.var_tmf2);
        let assign30250_e43631: f64 = (assign30250_e43629 + locals.var_tmf3);
        let assign30250_e43633: f64 = (assign30250_e43631 + locals.var_tmf4);
        let assign30250_e43634: f64 = (1.0 / assign30250_e43633);
        (assign30250_e43634, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign30250_e43633 * assign30250_e43633))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign30250_e43633 * assign30250_e43633))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign30250_e43633 * assign30250_e43633))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign30250_e43633 * assign30250_e43633))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign30250_e43633 * assign30250_e43633))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign30250_e43633 * assign30250_e43633))), (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign30250_e43633 * assign30250_e43633))), (-((((locals.var_tmf1_dn17 + locals.var_tmf2_dn17) + locals.var_tmf3_dn17) + locals.var_tmf4_dn17) / (assign30250_e43633 * assign30250_e43633))),)
    } else {
        (locals.var_ty__blk907, locals.var_ty__blk907_dn0, locals.var_ty__blk907_dn2, locals.var_ty__blk907_dn6, locals.var_ty__blk907_dn7, locals.var_ty__blk907_dn10, locals.var_ty__blk907_dn11, locals.var_ty__blk907_dn12, locals.var_ty__blk907_dn17,)
    }
};
        locals.var_ty__blk907 = assign30250_e43636;
        locals.var_ty__blk907_dn0 = assign30250_e43636_d_n0;
        locals.var_ty__blk907_dn2 = assign30250_e43636_d_n2;
        locals.var_ty__blk907_dn6 = assign30250_e43636_d_n6;
        locals.var_ty__blk907_dn7 = assign30250_e43636_d_n7;
        locals.var_ty__blk907_dn10 = assign30250_e43636_d_n10;
        locals.var_ty__blk907_dn11 = assign30250_e43636_d_n11;
        locals.var_ty__blk907_dn12 = assign30250_e43636_d_n12;
        locals.var_ty__blk907_dn17 = assign30250_e43636_d_n17;
        locals.var_ty__blk907_rv = 0.0;

        let (assign30270_e43679, assign30270_e43679_d_n0, assign30270_e43679_d_n2, assign30270_e43679_d_n6, assign30270_e43679_d_n7, assign30270_e43679_d_n10, assign30270_e43679_d_n11, assign30270_e43679_d_n12, assign30270_e43679_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1003 != 0.0)) {
        let assign30270_e43676: f64 = (1.0 - locals.var_ty__blk907);
        let assign30270_e43677: f64 = (locals.var_t2__blk899 * assign30270_e43676);
        (assign30270_e43677, ((locals.var_t2__blk899_dn0 * assign30270_e43676) + (locals.var_t2__blk899 * (-locals.var_ty__blk907_dn0))), ((locals.var_t2__blk899_dn2 * assign30270_e43676) + (locals.var_t2__blk899 * (-locals.var_ty__blk907_dn2))), ((locals.var_t2__blk899_dn6 * assign30270_e43676) + (locals.var_t2__blk899 * (-locals.var_ty__blk907_dn6))), ((locals.var_t2__blk899_dn7 * assign30270_e43676) + (locals.var_t2__blk899 * (-locals.var_ty__blk907_dn7))), ((locals.var_t2__blk899_dn10 * assign30270_e43676) + (locals.var_t2__blk899 * (-locals.var_ty__blk907_dn10))), ((locals.var_t2__blk899_dn11 * assign30270_e43676) + (locals.var_t2__blk899 * (-locals.var_ty__blk907_dn11))), ((locals.var_t2__blk899_dn12 * assign30270_e43676) + (locals.var_t2__blk899 * (-locals.var_ty__blk907_dn12))), ((locals.var_t2__blk899_dn17 * assign30270_e43676) + (locals.var_t2__blk899 * (-locals.var_ty__blk907_dn17))),)
    } else {
        (locals.var_ty__blk907, locals.var_ty__blk907_dn0, locals.var_ty__blk907_dn2, locals.var_ty__blk907_dn6, locals.var_ty__blk907_dn7, locals.var_ty__blk907_dn10, locals.var_ty__blk907_dn11, locals.var_ty__blk907_dn12, locals.var_ty__blk907_dn17,)
    }
};
        locals.var_ty__blk907 = assign30270_e43679;
        locals.var_ty__blk907_dn0 = assign30270_e43679_d_n0;
        locals.var_ty__blk907_dn2 = assign30270_e43679_d_n2;
        locals.var_ty__blk907_dn6 = assign30270_e43679_d_n6;
        locals.var_ty__blk907_dn7 = assign30270_e43679_d_n7;
        locals.var_ty__blk907_dn10 = assign30270_e43679_d_n10;
        locals.var_ty__blk907_dn11 = assign30270_e43679_d_n11;
        locals.var_ty__blk907_dn12 = assign30270_e43679_d_n12;
        locals.var_ty__blk907_dn17 = assign30270_e43679_d_n17;
        locals.var_ty__blk907_rv = 0.0;

        let (assign30290_e43704, assign30290_e43704_d_n0, assign30290_e43704_d_n2, assign30290_e43704_d_n6, assign30290_e43704_d_n7, assign30290_e43704_d_n10, assign30290_e43704_d_n11, assign30290_e43704_d_n12, assign30290_e43704_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1003 != 0.0)) {
        let assign30290_e43702: f64 = (locals.var_vbs_bnd + locals.var_ty__blk907);
        (assign30290_e43702, locals.var_ty__blk907_dn0, locals.var_ty__blk907_dn2, locals.var_ty__blk907_dn6, locals.var_ty__blk907_dn7, locals.var_ty__blk907_dn10, locals.var_ty__blk907_dn11, locals.var_ty__blk907_dn12, locals.var_ty__blk907_dn17,)
    } else {
        (locals.var_t10__blk904, locals.var_t10__blk904_dn0, locals.var_t10__blk904_dn2, locals.var_t10__blk904_dn6, locals.var_t10__blk904_dn7, locals.var_t10__blk904_dn10, locals.var_t10__blk904_dn11, locals.var_t10__blk904_dn12, locals.var_t10__blk904_dn17,)
    }
};
        locals.var_t10__blk904 = assign30290_e43704;
        locals.var_t10__blk904_dn0 = assign30290_e43704_d_n0;
        locals.var_t10__blk904_dn2 = assign30290_e43704_d_n2;
        locals.var_t10__blk904_dn6 = assign30290_e43704_d_n6;
        locals.var_t10__blk904_dn7 = assign30290_e43704_d_n7;
        locals.var_t10__blk904_dn10 = assign30290_e43704_d_n10;
        locals.var_t10__blk904_dn11 = assign30290_e43704_d_n11;
        locals.var_t10__blk904_dn12 = assign30290_e43704_d_n12;
        locals.var_t10__blk904_dn17 = assign30290_e43704_d_n17;
        locals.var_t10__blk904_rv = 0.0;

        let (assign30300_e43716, assign30300_e43716_d_n0, assign30300_e43716_d_n2, assign30300_e43716_d_n6, assign30300_e43716_d_n7, assign30300_e43716_d_n10, assign30300_e43716_d_n11, assign30300_e43716_d_n12, assign30300_e43716_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1003 == 0.0)) {
        (locals.var_t0__blk897, locals.var_t0__blk897_dn0, locals.var_t0__blk897_dn2, locals.var_t0__blk897_dn6, locals.var_t0__blk897_dn7, locals.var_t0__blk897_dn10, locals.var_t0__blk897_dn11, locals.var_t0__blk897_dn12, locals.var_t0__blk897_dn17,)
    } else {
        (locals.var_t10__blk904, locals.var_t10__blk904_dn0, locals.var_t10__blk904_dn2, locals.var_t10__blk904_dn6, locals.var_t10__blk904_dn7, locals.var_t10__blk904_dn10, locals.var_t10__blk904_dn11, locals.var_t10__blk904_dn12, locals.var_t10__blk904_dn17,)
    }
};
        locals.var_t10__blk904 = assign30300_e43716;
        locals.var_t10__blk904_dn0 = assign30300_e43716_d_n0;
        locals.var_t10__blk904_dn2 = assign30300_e43716_d_n2;
        locals.var_t10__blk904_dn6 = assign30300_e43716_d_n6;
        locals.var_t10__blk904_dn7 = assign30300_e43716_d_n7;
        locals.var_t10__blk904_dn10 = assign30300_e43716_d_n10;
        locals.var_t10__blk904_dn11 = assign30300_e43716_d_n11;
        locals.var_t10__blk904_dn12 = assign30300_e43716_d_n12;
        locals.var_t10__blk904_dn17 = assign30300_e43716_d_n17;
        locals.var_t10__blk904_rv = 0.0;

        let (assign30320_e43740, assign30320_e43740_d_n0, assign30320_e43740_d_n2, assign30320_e43740_d_n6, assign30320_e43740_d_n7, assign30320_e43740_d_n10, assign30320_e43740_d_n11, assign30320_e43740_d_n12, assign30320_e43740_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign30320_e43736: f64 = (-locals.var_t10__blk904);
        let assign30320_e43738: f64 = (assign30320_e43736 - 1e-12);
        (assign30320_e43738, (-locals.var_t10__blk904_dn0), (-locals.var_t10__blk904_dn2), (-locals.var_t10__blk904_dn6), (-locals.var_t10__blk904_dn7), (-locals.var_t10__blk904_dn10), (-locals.var_t10__blk904_dn11), (-locals.var_t10__blk904_dn12), (-locals.var_t10__blk904_dn17),)
    } else {
        (locals.var_vxbgmtcl__blk923, locals.var_vxbgmtcl__blk923_dn0, locals.var_vxbgmtcl__blk923_dn2, locals.var_vxbgmtcl__blk923_dn6, locals.var_vxbgmtcl__blk923_dn7, locals.var_vxbgmtcl__blk923_dn10, locals.var_vxbgmtcl__blk923_dn11, locals.var_vxbgmtcl__blk923_dn12, locals.var_vxbgmtcl__blk923_dn17,)
    }
};
        locals.var_vxbgmtcl__blk923 = assign30320_e43740;
        locals.var_vxbgmtcl__blk923_dn0 = assign30320_e43740_d_n0;
        locals.var_vxbgmtcl__blk923_dn2 = assign30320_e43740_d_n2;
        locals.var_vxbgmtcl__blk923_dn6 = assign30320_e43740_d_n6;
        locals.var_vxbgmtcl__blk923_dn7 = assign30320_e43740_d_n7;
        locals.var_vxbgmtcl__blk923_dn10 = assign30320_e43740_d_n10;
        locals.var_vxbgmtcl__blk923_dn11 = assign30320_e43740_d_n11;
        locals.var_vxbgmtcl__blk923_dn12 = assign30320_e43740_d_n12;
        locals.var_vxbgmtcl__blk923_dn17 = assign30320_e43740_d_n17;
        locals.var_vxbgmtcl__blk923_rv = 0.0;

        let (assign30330_e43751, assign30330_e43751_d_n0, assign30330_e43751_d_n2, assign30330_e43751_d_n6, assign30330_e43751_d_n7, assign30330_e43751_d_n10, assign30330_e43751_d_n11, assign30330_e43751_d_n12, assign30330_e43751_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign30330_e43749: f64 = (locals.var_cnst0over__blk930 * locals.var_cox0_inv__blk909);
        (assign30330_e43749, (locals.var_cnst0over__blk930_dn0 * locals.var_cox0_inv__blk909), (locals.var_cnst0over__blk930_dn2 * locals.var_cox0_inv__blk909), (locals.var_cnst0over__blk930_dn6 * locals.var_cox0_inv__blk909), (locals.var_cnst0over__blk930_dn7 * locals.var_cox0_inv__blk909), (locals.var_cnst0over__blk930_dn10 * locals.var_cox0_inv__blk909), (locals.var_cnst0over__blk930_dn11 * locals.var_cox0_inv__blk909), (locals.var_cnst0over__blk930_dn12 * locals.var_cox0_inv__blk909), (locals.var_cnst0over__blk930_dn17 * locals.var_cox0_inv__blk909),)
    } else {
        (locals.var_fac1__blk931, locals.var_fac1__blk931_dn0, locals.var_fac1__blk931_dn2, locals.var_fac1__blk931_dn6, locals.var_fac1__blk931_dn7, locals.var_fac1__blk931_dn10, locals.var_fac1__blk931_dn11, locals.var_fac1__blk931_dn12, locals.var_fac1__blk931_dn17,)
    }
};
        locals.var_fac1__blk931 = assign30330_e43751;
        locals.var_fac1__blk931_dn0 = assign30330_e43751_d_n0;
        locals.var_fac1__blk931_dn2 = assign30330_e43751_d_n2;
        locals.var_fac1__blk931_dn6 = assign30330_e43751_d_n6;
        locals.var_fac1__blk931_dn7 = assign30330_e43751_d_n7;
        locals.var_fac1__blk931_dn10 = assign30330_e43751_d_n10;
        locals.var_fac1__blk931_dn11 = assign30330_e43751_d_n11;
        locals.var_fac1__blk931_dn12 = assign30330_e43751_d_n12;
        locals.var_fac1__blk931_dn17 = assign30330_e43751_d_n17;
        locals.var_fac1__blk931_rv = 0.0;

        let (assign30340_e43762, assign30340_e43762_d_n0, assign30340_e43762_d_n2, assign30340_e43762_d_n6, assign30340_e43762_d_n7, assign30340_e43762_d_n10, assign30340_e43762_d_n11, assign30340_e43762_d_n12, assign30340_e43762_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign30340_e43760: f64 = (locals.var_fac1__blk931 * locals.var_fac1__blk931);
        (assign30340_e43760, ((locals.var_fac1__blk931_dn0 * locals.var_fac1__blk931) + (locals.var_fac1__blk931 * locals.var_fac1__blk931_dn0)), ((locals.var_fac1__blk931_dn2 * locals.var_fac1__blk931) + (locals.var_fac1__blk931 * locals.var_fac1__blk931_dn2)), ((locals.var_fac1__blk931_dn6 * locals.var_fac1__blk931) + (locals.var_fac1__blk931 * locals.var_fac1__blk931_dn6)), ((locals.var_fac1__blk931_dn7 * locals.var_fac1__blk931) + (locals.var_fac1__blk931 * locals.var_fac1__blk931_dn7)), ((locals.var_fac1__blk931_dn10 * locals.var_fac1__blk931) + (locals.var_fac1__blk931 * locals.var_fac1__blk931_dn10)), ((locals.var_fac1__blk931_dn11 * locals.var_fac1__blk931) + (locals.var_fac1__blk931 * locals.var_fac1__blk931_dn11)), ((locals.var_fac1__blk931_dn12 * locals.var_fac1__blk931) + (locals.var_fac1__blk931 * locals.var_fac1__blk931_dn12)), ((locals.var_fac1__blk931_dn17 * locals.var_fac1__blk931) + (locals.var_fac1__blk931 * locals.var_fac1__blk931_dn17)),)
    } else {
        (locals.var_fac1p2__blk932, locals.var_fac1p2__blk932_dn0, locals.var_fac1p2__blk932_dn2, locals.var_fac1p2__blk932_dn6, locals.var_fac1p2__blk932_dn7, locals.var_fac1p2__blk932_dn10, locals.var_fac1p2__blk932_dn11, locals.var_fac1p2__blk932_dn12, locals.var_fac1p2__blk932_dn17,)
    }
};
        locals.var_fac1p2__blk932 = assign30340_e43762;
        locals.var_fac1p2__blk932_dn0 = assign30340_e43762_d_n0;
        locals.var_fac1p2__blk932_dn2 = assign30340_e43762_d_n2;
        locals.var_fac1p2__blk932_dn6 = assign30340_e43762_d_n6;
        locals.var_fac1p2__blk932_dn7 = assign30340_e43762_d_n7;
        locals.var_fac1p2__blk932_dn10 = assign30340_e43762_d_n10;
        locals.var_fac1p2__blk932_dn11 = assign30340_e43762_d_n11;
        locals.var_fac1p2__blk932_dn12 = assign30340_e43762_d_n12;
        locals.var_fac1p2__blk932_dn17 = assign30340_e43762_d_n17;
        locals.var_fac1p2__blk932_rv = 0.0;

        let (assign30350_e43774, assign30350_e43774_d_n0, assign30350_e43774_d_n2, assign30350_e43774_d_n6, assign30350_e43774_d_n7, assign30350_e43774_d_n10, assign30350_e43774_d_n11, assign30350_e43774_d_n12, assign30350_e43774_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign30350_e43770: f64 = (-locals.var_vgbgmt__blk929);
        let assign30350_e43772: f64 = (assign30350_e43770 + locals.var_uc_vfbover);
        (assign30350_e43772, (-locals.var_vgbgmt__blk929_dn0), (-locals.var_vgbgmt__blk929_dn2), (-locals.var_vgbgmt__blk929_dn6), (-locals.var_vgbgmt__blk929_dn7), (-locals.var_vgbgmt__blk929_dn10), (-locals.var_vgbgmt__blk929_dn11), (-locals.var_vgbgmt__blk929_dn12), (-locals.var_vgbgmt__blk929_dn17),)
    } else {
        (locals.var_vgpld__blk933, locals.var_vgpld__blk933_dn0, locals.var_vgpld__blk933_dn2, locals.var_vgpld__blk933_dn6, locals.var_vgpld__blk933_dn7, locals.var_vgpld__blk933_dn10, locals.var_vgpld__blk933_dn11, locals.var_vgpld__blk933_dn12, locals.var_vgpld__blk933_dn17,)
    }
};
        locals.var_vgpld__blk933 = assign30350_e43774;
        locals.var_vgpld__blk933_dn0 = assign30350_e43774_d_n0;
        locals.var_vgpld__blk933_dn2 = assign30350_e43774_d_n2;
        locals.var_vgpld__blk933_dn6 = assign30350_e43774_d_n6;
        locals.var_vgpld__blk933_dn7 = assign30350_e43774_d_n7;
        locals.var_vgpld__blk933_dn10 = assign30350_e43774_d_n10;
        locals.var_vgpld__blk933_dn11 = assign30350_e43774_d_n11;
        locals.var_vgpld__blk933_dn12 = assign30350_e43774_d_n12;
        locals.var_vgpld__blk933_dn17 = assign30350_e43774_d_n17;
        locals.var_vgpld__blk933_rv = 0.0;

        let (assign30360_e43785, assign30360_e43785_d_n0, assign30360_e43785_d_n2, assign30360_e43785_d_n6, assign30360_e43785_d_n7, assign30360_e43785_d_n10, assign30360_e43785_d_n11, assign30360_e43785_d_n12, assign30360_e43785_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign30360_e43783: f64 = (locals.var_mks_nover / locals.var_nin);
        (assign30360_e43783, (-((locals.var_mks_nover * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn12) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn17) / (locals.var_nin * locals.var_nin))),)
    } else {
        (locals.var_t0__blk897, locals.var_t0__blk897_dn0, locals.var_t0__blk897_dn2, locals.var_t0__blk897_dn6, locals.var_t0__blk897_dn7, locals.var_t0__blk897_dn10, locals.var_t0__blk897_dn11, locals.var_t0__blk897_dn12, locals.var_t0__blk897_dn17,)
    }
};
        locals.var_t0__blk897 = assign30360_e43785;
        locals.var_t0__blk897_dn0 = assign30360_e43785_d_n0;
        locals.var_t0__blk897_dn2 = assign30360_e43785_d_n2;
        locals.var_t0__blk897_dn6 = assign30360_e43785_d_n6;
        locals.var_t0__blk897_dn7 = assign30360_e43785_d_n7;
        locals.var_t0__blk897_dn10 = assign30360_e43785_d_n10;
        locals.var_t0__blk897_dn11 = assign30360_e43785_d_n11;
        locals.var_t0__blk897_dn12 = assign30360_e43785_d_n12;
        locals.var_t0__blk897_dn17 = assign30360_e43785_d_n17;
        locals.var_t0__blk897_rv = 0.0;

        let (assign30370_e43799, assign30370_e43799_d_n0, assign30370_e43799_d_n2, assign30370_e43799_d_n6, assign30370_e43799_d_n7, assign30370_e43799_d_n10, assign30370_e43799_d_n11, assign30370_e43799_d_n12, assign30370_e43799_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign30370_e43794: f64 = (2.0 / locals.var_beta);
        let assign30370_e43796: f64 = (locals.var_t0__blk897).ln();
        let assign30370_e43797: f64 = (assign30370_e43794 * assign30370_e43796);
        (assign30370_e43797, (assign30370_e43794 * (locals.var_t0__blk897_dn0 / locals.var_t0__blk897)), (assign30370_e43794 * (locals.var_t0__blk897_dn2 / locals.var_t0__blk897)), (assign30370_e43794 * (locals.var_t0__blk897_dn6 / locals.var_t0__blk897)), (assign30370_e43794 * (locals.var_t0__blk897_dn7 / locals.var_t0__blk897)), (((-((2.0 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign30370_e43796) + (assign30370_e43794 * (locals.var_t0__blk897_dn10 / locals.var_t0__blk897))), (assign30370_e43794 * (locals.var_t0__blk897_dn11 / locals.var_t0__blk897)), (assign30370_e43794 * (locals.var_t0__blk897_dn12 / locals.var_t0__blk897)), (assign30370_e43794 * (locals.var_t0__blk897_dn17 / locals.var_t0__blk897)),)
    } else {
        (locals.var_pb2over__blk934, locals.var_pb2over__blk934_dn0, locals.var_pb2over__blk934_dn2, locals.var_pb2over__blk934_dn6, locals.var_pb2over__blk934_dn7, locals.var_pb2over__blk934_dn10, locals.var_pb2over__blk934_dn11, locals.var_pb2over__blk934_dn12, locals.var_pb2over__blk934_dn17,)
    }
};
        locals.var_pb2over__blk934 = assign30370_e43799;
        locals.var_pb2over__blk934_dn0 = assign30370_e43799_d_n0;
        locals.var_pb2over__blk934_dn2 = assign30370_e43799_d_n2;
        locals.var_pb2over__blk934_dn6 = assign30370_e43799_d_n6;
        locals.var_pb2over__blk934_dn7 = assign30370_e43799_d_n7;
        locals.var_pb2over__blk934_dn10 = assign30370_e43799_d_n10;
        locals.var_pb2over__blk934_dn11 = assign30370_e43799_d_n11;
        locals.var_pb2over__blk934_dn12 = assign30370_e43799_d_n12;
        locals.var_pb2over__blk934_dn17 = assign30370_e43799_d_n17;
        locals.var_pb2over__blk934_rv = 0.0;

        let (assign30380_e43809, assign30380_e43809_d_n0, assign30380_e43809_d_n2, assign30380_e43809_d_n6, assign30380_e43809_d_n7, assign30380_e43809_d_n10, assign30380_e43809_d_n11, assign30380_e43809_d_n12, assign30380_e43809_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) {
        let assign30380_e43807: f64 = (-locals.var_vxbgmtcl__blk923);
        (assign30380_e43807, (-locals.var_vxbgmtcl__blk923_dn0), (-locals.var_vxbgmtcl__blk923_dn2), (-locals.var_vxbgmtcl__blk923_dn6), (-locals.var_vxbgmtcl__blk923_dn7), (-locals.var_vxbgmtcl__blk923_dn10), (-locals.var_vxbgmtcl__blk923_dn11), (-locals.var_vxbgmtcl__blk923_dn12), (-locals.var_vxbgmtcl__blk923_dn17),)
    } else {
        (locals.var_vgb_fb_ld__blk935, locals.var_vgb_fb_ld__blk935_dn0, locals.var_vgb_fb_ld__blk935_dn2, locals.var_vgb_fb_ld__blk935_dn6, locals.var_vgb_fb_ld__blk935_dn7, locals.var_vgb_fb_ld__blk935_dn10, locals.var_vgb_fb_ld__blk935_dn11, locals.var_vgb_fb_ld__blk935_dn12, locals.var_vgb_fb_ld__blk935_dn17,)
    }
};
        locals.var_vgb_fb_ld__blk935 = assign30380_e43809;
        locals.var_vgb_fb_ld__blk935_dn0 = assign30380_e43809_d_n0;
        locals.var_vgb_fb_ld__blk935_dn2 = assign30380_e43809_d_n2;
        locals.var_vgb_fb_ld__blk935_dn6 = assign30380_e43809_d_n6;
        locals.var_vgb_fb_ld__blk935_dn7 = assign30380_e43809_d_n7;
        locals.var_vgb_fb_ld__blk935_dn10 = assign30380_e43809_d_n10;
        locals.var_vgb_fb_ld__blk935_dn11 = assign30380_e43809_d_n11;
        locals.var_vgb_fb_ld__blk935_dn12 = assign30380_e43809_d_n12;
        locals.var_vgb_fb_ld__blk935_dn17 = assign30380_e43809_d_n17;
        locals.var_vgb_fb_ld__blk935_rv = 0.0;

        let assign30390_e43812: f64 = if locals.var_vgpld__blk933 < locals.var_vgb_fb_ld__blk935 { 1.0 } else { 0.0 };
        locals.var_guard1004 = assign30390_e43812;
        locals.var_guard1004_rv = 0.0;

        let (assign30410_e43839, assign30410_e43839_d_n0, assign30410_e43839_d_n2, assign30410_e43839_d_n6, assign30410_e43839_d_n7, assign30410_e43839_d_n10, assign30410_e43839_d_n11, assign30410_e43839_d_n12, assign30410_e43839_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 != 0.0)) {
        let assign30410_e43836: f64 = (locals.var_beta * locals.var_cnst0over__blk930);
        let assign30410_e43837: f64 = (1.0 / assign30410_e43836);
        (assign30410_e43837, (-((locals.var_beta * locals.var_cnst0over__blk930_dn0) / (assign30410_e43836 * assign30410_e43836))), (-((locals.var_beta * locals.var_cnst0over__blk930_dn2) / (assign30410_e43836 * assign30410_e43836))), (-((locals.var_beta * locals.var_cnst0over__blk930_dn6) / (assign30410_e43836 * assign30410_e43836))), (-((locals.var_beta * locals.var_cnst0over__blk930_dn7) / (assign30410_e43836 * assign30410_e43836))), (-(((locals.var_beta_dn10 * locals.var_cnst0over__blk930) + (locals.var_beta * locals.var_cnst0over__blk930_dn10)) / (assign30410_e43836 * assign30410_e43836))), (-((locals.var_beta * locals.var_cnst0over__blk930_dn11) / (assign30410_e43836 * assign30410_e43836))), (-((locals.var_beta * locals.var_cnst0over__blk930_dn12) / (assign30410_e43836 * assign30410_e43836))), (-((locals.var_beta * locals.var_cnst0over__blk930_dn17) / (assign30410_e43836 * assign30410_e43836))),)
    } else {
        (locals.var_t1__blk898, locals.var_t1__blk898_dn0, locals.var_t1__blk898_dn2, locals.var_t1__blk898_dn6, locals.var_t1__blk898_dn7, locals.var_t1__blk898_dn10, locals.var_t1__blk898_dn11, locals.var_t1__blk898_dn12, locals.var_t1__blk898_dn17,)
    }
};
        locals.var_t1__blk898 = assign30410_e43839;
        locals.var_t1__blk898_dn0 = assign30410_e43839_d_n0;
        locals.var_t1__blk898_dn2 = assign30410_e43839_d_n2;
        locals.var_t1__blk898_dn6 = assign30410_e43839_d_n6;
        locals.var_t1__blk898_dn7 = assign30410_e43839_d_n7;
        locals.var_t1__blk898_dn10 = assign30410_e43839_d_n10;
        locals.var_t1__blk898_dn11 = assign30410_e43839_d_n11;
        locals.var_t1__blk898_dn12 = assign30410_e43839_d_n12;
        locals.var_t1__blk898_dn17 = assign30410_e43839_d_n17;
        locals.var_t1__blk898_rv = 0.0;

        let (assign30420_e43852, assign30420_e43852_d_n0, assign30420_e43852_d_n2, assign30420_e43852_d_n6, assign30420_e43852_d_n7, assign30420_e43852_d_n10, assign30420_e43852_d_n11, assign30420_e43852_d_n12, assign30420_e43852_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 != 0.0)) {
        let assign30420_e43850: f64 = (locals.var_t1__blk898 * locals.var_cox0__blk908);
        (assign30420_e43850, (locals.var_t1__blk898_dn0 * locals.var_cox0__blk908), (locals.var_t1__blk898_dn2 * locals.var_cox0__blk908), (locals.var_t1__blk898_dn6 * locals.var_cox0__blk908), (locals.var_t1__blk898_dn7 * locals.var_cox0__blk908), (locals.var_t1__blk898_dn10 * locals.var_cox0__blk908), (locals.var_t1__blk898_dn11 * locals.var_cox0__blk908), (locals.var_t1__blk898_dn12 * locals.var_cox0__blk908), (locals.var_t1__blk898_dn17 * locals.var_cox0__blk908),)
    } else {
        (locals.var_ty__blk907, locals.var_ty__blk907_dn0, locals.var_ty__blk907_dn2, locals.var_ty__blk907_dn6, locals.var_ty__blk907_dn7, locals.var_ty__blk907_dn10, locals.var_ty__blk907_dn11, locals.var_ty__blk907_dn12, locals.var_ty__blk907_dn17,)
    }
};
        locals.var_ty__blk907 = assign30420_e43852;
        locals.var_ty__blk907_dn0 = assign30420_e43852_d_n0;
        locals.var_ty__blk907_dn2 = assign30420_e43852_d_n2;
        locals.var_ty__blk907_dn6 = assign30420_e43852_d_n6;
        locals.var_ty__blk907_dn7 = assign30420_e43852_d_n7;
        locals.var_ty__blk907_dn10 = assign30420_e43852_d_n10;
        locals.var_ty__blk907_dn11 = assign30420_e43852_d_n11;
        locals.var_ty__blk907_dn12 = assign30420_e43852_d_n12;
        locals.var_ty__blk907_dn17 = assign30420_e43852_d_n17;
        locals.var_ty__blk907_rv = 0.0;

        let (assign30430_e43869, assign30430_e43869_d_n0, assign30430_e43869_d_n2, assign30430_e43869_d_n6, assign30430_e43869_d_n7, assign30430_e43869_d_n10, assign30430_e43869_d_n11, assign30430_e43869_d_n12, assign30430_e43869_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard980 != 0.0)) && (locals.var_guard981 == 0.0)) && (locals.var_guard1004 != 0.0)) {
        let assign30430_e43864: f64 = (3.0 * 1.414213562373095);
        let assign30430_e43866: f64 = (assign30430_e43864 * locals.var_ty__blk907);
        let assign30430_e43867: f64 = (2.0 + assign30430_e43866);
        (assign30430_e43867, (assign30430_e43864 * locals.var_ty__blk907_dn0), (assign30430_e43864 * locals.var_ty__blk907_dn2), (assign30430_e43864 * locals.var_ty__blk907_dn6), (assign30430_e43864 * locals.var_ty__blk907_dn7), (assign30430_e43864 * locals.var_ty__blk907_dn10), (assign30430_e43864 * locals.var_ty__blk907_dn11), (assign30430_e43864 * locals.var_ty__blk907_dn12), (assign30430_e43864 * locals.var_ty__blk907_dn17),)
    } else {
        (locals.var_ac41__blk936, locals.var_ac41__blk936_dn0, locals.var_ac41__blk936_dn2, locals.var_ac41__blk936_dn6, locals.var_ac41__blk936_dn7, locals.var_ac41__blk936_dn10, locals.var_ac41__blk936_dn11, locals.var_ac41__blk936_dn12, locals.var_ac41__blk936_dn17,)
    }
};
        locals.var_ac41__blk936 = assign30430_e43869;
        locals.var_ac41__blk936_dn0 = assign30430_e43869_d_n0;
        locals.var_ac41__blk936_dn2 = assign30430_e43869_d_n2;
        locals.var_ac41__blk936_dn6 = assign30430_e43869_d_n6;
        locals.var_ac41__blk936_dn7 = assign30430_e43869_d_n7;
        locals.var_ac41__blk936_dn10 = assign30430_e43869_d_n10;
        locals.var_ac41__blk936_dn11 = assign30430_e43869_d_n11;
        locals.var_ac41__blk936_dn12 = assign30430_e43869_d_n12;
        locals.var_ac41__blk936_dn17 = assign30430_e43869_d_n17;
        locals.var_ac41__blk936_rv = 0.0;

    }
}
