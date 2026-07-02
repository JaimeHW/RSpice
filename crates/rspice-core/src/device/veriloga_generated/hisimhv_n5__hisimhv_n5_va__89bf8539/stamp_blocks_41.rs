#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_264(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign70370_e106840, assign70370_e106840_d_n0, assign70370_e106840_d_n2, assign70370_e106840_d_n4, assign70370_e106840_d_n5, assign70370_e106840_d_n6, assign70370_e106840_d_n7, assign70370_e106840_d_n8, assign70370_e106840_d_n9, assign70370_e106840_d_n10, assign70370_e106840_d_n11, assign70370_e106840_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1654 != 0.0)) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_over, locals.var_vbs_max_over_dn0, locals.var_vbs_max_over_dn2, locals.var_vbs_max_over_dn4, locals.var_vbs_max_over_dn5, locals.var_vbs_max_over_dn6, locals.var_vbs_max_over_dn7, locals.var_vbs_max_over_dn8, locals.var_vbs_max_over_dn9, locals.var_vbs_max_over_dn10, locals.var_vbs_max_over_dn11, locals.var_vbs_max_over_dn14,)
    }
};
        locals.var_vbs_max_over = assign70370_e106840;
        locals.var_vbs_max_over_dn0 = assign70370_e106840_d_n0;
        locals.var_vbs_max_over_dn2 = assign70370_e106840_d_n2;
        locals.var_vbs_max_over_dn4 = assign70370_e106840_d_n4;
        locals.var_vbs_max_over_dn5 = assign70370_e106840_d_n5;
        locals.var_vbs_max_over_dn6 = assign70370_e106840_d_n6;
        locals.var_vbs_max_over_dn7 = assign70370_e106840_d_n7;
        locals.var_vbs_max_over_dn8 = assign70370_e106840_d_n8;
        locals.var_vbs_max_over_dn9 = assign70370_e106840_d_n9;
        locals.var_vbs_max_over_dn10 = assign70370_e106840_d_n10;
        locals.var_vbs_max_over_dn11 = assign70370_e106840_d_n11;
        locals.var_vbs_max_over_dn14 = assign70370_e106840_d_n14;
        locals.var_vbs_max_over_rv = 0.0;

        let assign70380_e106842: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard1655 = assign70380_e106842;
        locals.var_guard1655_rv = 0.0;

        let (assign70390_e106848, assign70390_e106848_d_n0, assign70390_e106848_d_n2, assign70390_e106848_d_n4, assign70390_e106848_d_n5, assign70390_e106848_d_n6, assign70390_e106848_d_n7, assign70390_e106848_d_n8, assign70390_e106848_d_n9, assign70390_e106848_d_n10, assign70390_e106848_d_n11, assign70390_e106848_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1655 != 0.0)) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn11, locals.var_vbs_bnd_over_dn14,)
    }
};
        locals.var_vbs_bnd_over = assign70390_e106848;
        locals.var_vbs_bnd_over_dn0 = assign70390_e106848_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70390_e106848_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70390_e106848_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70390_e106848_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70390_e106848_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70390_e106848_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70390_e106848_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70390_e106848_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70390_e106848_d_n10;
        locals.var_vbs_bnd_over_dn11 = assign70390_e106848_d_n11;
        locals.var_vbs_bnd_over_dn14 = assign70390_e106848_d_n14;
        locals.var_vbs_bnd_over_rv = 0.0;

        let assign70400_e106850: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1656 = assign70400_e106850;
        locals.var_guard1656_rv = 0.0;

        let (assign70410_e106861, assign70410_e106861_d_n0, assign70410_e106861_d_n2, assign70410_e106861_d_n4, assign70410_e106861_d_n5, assign70410_e106861_d_n6, assign70410_e106861_d_n7, assign70410_e106861_d_n8, assign70410_e106861_d_n9, assign70410_e106861_d_n10, assign70410_e106861_d_n11, assign70410_e106861_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1655 == 0.0)) && (locals.var_guard1656 != 0.0)) {
        let assign70410_e106859: f64 = (0.5 * locals.var_vbs_max_over);
        (assign70410_e106859, (0.5 * locals.var_vbs_max_over_dn0), (0.5 * locals.var_vbs_max_over_dn2), (0.5 * locals.var_vbs_max_over_dn4), (0.5 * locals.var_vbs_max_over_dn5), (0.5 * locals.var_vbs_max_over_dn6), (0.5 * locals.var_vbs_max_over_dn7), (0.5 * locals.var_vbs_max_over_dn8), (0.5 * locals.var_vbs_max_over_dn9), (0.5 * locals.var_vbs_max_over_dn10), (0.5 * locals.var_vbs_max_over_dn11), (0.5 * locals.var_vbs_max_over_dn14),)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn11, locals.var_vbs_bnd_over_dn14,)
    }
};
        locals.var_vbs_bnd_over = assign70410_e106861;
        locals.var_vbs_bnd_over_dn0 = assign70410_e106861_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70410_e106861_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70410_e106861_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70410_e106861_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70410_e106861_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70410_e106861_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70410_e106861_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70410_e106861_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70410_e106861_d_n10;
        locals.var_vbs_bnd_over_dn11 = assign70410_e106861_d_n11;
        locals.var_vbs_bnd_over_dn14 = assign70410_e106861_d_n14;
        locals.var_vbs_bnd_over_rv = 0.0;

        let assign70420_e106865: f64 = (locals.var_vbs_max_over * 0.5);
        let assign70420_e106866: f64 = if locals.var_vbs_bnd_over > assign70420_e106865 { 1.0 } else { 0.0 };
        locals.var_guard1657 = assign70420_e106866;
        locals.var_guard1657_rv = 0.0;

        let (assign70430_e106874, assign70430_e106874_d_n0, assign70430_e106874_d_n2, assign70430_e106874_d_n4, assign70430_e106874_d_n5, assign70430_e106874_d_n6, assign70430_e106874_d_n7, assign70430_e106874_d_n8, assign70430_e106874_d_n9, assign70430_e106874_d_n10, assign70430_e106874_d_n11, assign70430_e106874_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1657 != 0.0)) {
        let assign70430_e106872: f64 = (0.5 * locals.var_vbs_max_over);
        (assign70430_e106872, (0.5 * locals.var_vbs_max_over_dn0), (0.5 * locals.var_vbs_max_over_dn2), (0.5 * locals.var_vbs_max_over_dn4), (0.5 * locals.var_vbs_max_over_dn5), (0.5 * locals.var_vbs_max_over_dn6), (0.5 * locals.var_vbs_max_over_dn7), (0.5 * locals.var_vbs_max_over_dn8), (0.5 * locals.var_vbs_max_over_dn9), (0.5 * locals.var_vbs_max_over_dn10), (0.5 * locals.var_vbs_max_over_dn11), (0.5 * locals.var_vbs_max_over_dn14),)
    } else {
        (locals.var_vbs_bnd_over, locals.var_vbs_bnd_over_dn0, locals.var_vbs_bnd_over_dn2, locals.var_vbs_bnd_over_dn4, locals.var_vbs_bnd_over_dn5, locals.var_vbs_bnd_over_dn6, locals.var_vbs_bnd_over_dn7, locals.var_vbs_bnd_over_dn8, locals.var_vbs_bnd_over_dn9, locals.var_vbs_bnd_over_dn10, locals.var_vbs_bnd_over_dn11, locals.var_vbs_bnd_over_dn14,)
    }
};
        locals.var_vbs_bnd_over = assign70430_e106874;
        locals.var_vbs_bnd_over_dn0 = assign70430_e106874_d_n0;
        locals.var_vbs_bnd_over_dn2 = assign70430_e106874_d_n2;
        locals.var_vbs_bnd_over_dn4 = assign70430_e106874_d_n4;
        locals.var_vbs_bnd_over_dn5 = assign70430_e106874_d_n5;
        locals.var_vbs_bnd_over_dn6 = assign70430_e106874_d_n6;
        locals.var_vbs_bnd_over_dn7 = assign70430_e106874_d_n7;
        locals.var_vbs_bnd_over_dn8 = assign70430_e106874_d_n8;
        locals.var_vbs_bnd_over_dn9 = assign70430_e106874_d_n9;
        locals.var_vbs_bnd_over_dn10 = assign70430_e106874_d_n10;
        locals.var_vbs_bnd_over_dn11 = assign70430_e106874_d_n11;
        locals.var_vbs_bnd_over_dn14 = assign70430_e106874_d_n14;
        locals.var_vbs_bnd_over_rv = 0.0;

        let assign70440_e106877: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1658 = assign70440_e106877;
        locals.var_guard1658_rv = 0.0;

        let (assign70450_e106884, assign70450_e106884_d_n0, assign70450_e106884_d_n2, assign70450_e106884_d_n4, assign70450_e106884_d_n5, assign70450_e106884_d_n6, assign70450_e106884_d_n7, assign70450_e106884_d_n8, assign70450_e106884_d_n9, assign70450_e106884_d_n10, assign70450_e106884_d_n11, assign70450_e106884_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) {
        let assign70450_e106882: f64 = (-locals.var_vxbgmt);
        (assign70450_e106882, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn4), (-locals.var_vxbgmt_dn5), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn8), (-locals.var_vxbgmt_dn9), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn11), (-locals.var_vxbgmt_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70450_e106884;
        locals.var_t0_dn0 = assign70450_e106884_d_n0;
        locals.var_t0_dn2 = assign70450_e106884_d_n2;
        locals.var_t0_dn4 = assign70450_e106884_d_n4;
        locals.var_t0_dn5 = assign70450_e106884_d_n5;
        locals.var_t0_dn6 = assign70450_e106884_d_n6;
        locals.var_t0_dn7 = assign70450_e106884_d_n7;
        locals.var_t0_dn8 = assign70450_e106884_d_n8;
        locals.var_t0_dn9 = assign70450_e106884_d_n9;
        locals.var_t0_dn10 = assign70450_e106884_d_n10;
        locals.var_t0_dn11 = assign70450_e106884_d_n11;
        locals.var_t0_dn14 = assign70450_e106884_d_n14;
        locals.var_t0_rv = 0.0;

        let assign70460_e106887: f64 = if locals.var_t0 > locals.var_vbs_bnd_over { 1.0 } else { 0.0 };
        locals.var_guard1659 = assign70460_e106887;
        locals.var_guard1659_rv = 0.0;

        let (assign70470_e106897, assign70470_e106897_d_n0, assign70470_e106897_d_n2, assign70470_e106897_d_n4, assign70470_e106897_d_n5, assign70470_e106897_d_n6, assign70470_e106897_d_n7, assign70470_e106897_d_n8, assign70470_e106897_d_n9, assign70470_e106897_d_n10, assign70470_e106897_d_n11, assign70470_e106897_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70470_e106895: f64 = (locals.var_t0 - locals.var_vbs_bnd_over);
        (assign70470_e106895, (locals.var_t0_dn0 - locals.var_vbs_bnd_over_dn0), (locals.var_t0_dn2 - locals.var_vbs_bnd_over_dn2), (locals.var_t0_dn4 - locals.var_vbs_bnd_over_dn4), (locals.var_t0_dn5 - locals.var_vbs_bnd_over_dn5), (locals.var_t0_dn6 - locals.var_vbs_bnd_over_dn6), (locals.var_t0_dn7 - locals.var_vbs_bnd_over_dn7), (locals.var_t0_dn8 - locals.var_vbs_bnd_over_dn8), (locals.var_t0_dn9 - locals.var_vbs_bnd_over_dn9), (locals.var_t0_dn10 - locals.var_vbs_bnd_over_dn10), (locals.var_t0_dn11 - locals.var_vbs_bnd_over_dn11), (locals.var_t0_dn14 - locals.var_vbs_bnd_over_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign70470_e106897;
        locals.var_t1_dn0 = assign70470_e106897_d_n0;
        locals.var_t1_dn2 = assign70470_e106897_d_n2;
        locals.var_t1_dn4 = assign70470_e106897_d_n4;
        locals.var_t1_dn5 = assign70470_e106897_d_n5;
        locals.var_t1_dn6 = assign70470_e106897_d_n6;
        locals.var_t1_dn7 = assign70470_e106897_d_n7;
        locals.var_t1_dn8 = assign70470_e106897_d_n8;
        locals.var_t1_dn9 = assign70470_e106897_d_n9;
        locals.var_t1_dn10 = assign70470_e106897_d_n10;
        locals.var_t1_dn11 = assign70470_e106897_d_n11;
        locals.var_t1_dn14 = assign70470_e106897_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign70480_e106907, assign70480_e106907_d_n0, assign70480_e106907_d_n2, assign70480_e106907_d_n4, assign70480_e106907_d_n5, assign70480_e106907_d_n6, assign70480_e106907_d_n7, assign70480_e106907_d_n8, assign70480_e106907_d_n9, assign70480_e106907_d_n10, assign70480_e106907_d_n11, assign70480_e106907_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70480_e106905: f64 = (locals.var_vbs_max_over - locals.var_vbs_bnd_over);
        (assign70480_e106905, (locals.var_vbs_max_over_dn0 - locals.var_vbs_bnd_over_dn0), (locals.var_vbs_max_over_dn2 - locals.var_vbs_bnd_over_dn2), (locals.var_vbs_max_over_dn4 - locals.var_vbs_bnd_over_dn4), (locals.var_vbs_max_over_dn5 - locals.var_vbs_bnd_over_dn5), (locals.var_vbs_max_over_dn6 - locals.var_vbs_bnd_over_dn6), (locals.var_vbs_max_over_dn7 - locals.var_vbs_bnd_over_dn7), (locals.var_vbs_max_over_dn8 - locals.var_vbs_bnd_over_dn8), (locals.var_vbs_max_over_dn9 - locals.var_vbs_bnd_over_dn9), (locals.var_vbs_max_over_dn10 - locals.var_vbs_bnd_over_dn10), (locals.var_vbs_max_over_dn11 - locals.var_vbs_bnd_over_dn11), (locals.var_vbs_max_over_dn14 - locals.var_vbs_bnd_over_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign70480_e106907;
        locals.var_t2_dn0 = assign70480_e106907_d_n0;
        locals.var_t2_dn2 = assign70480_e106907_d_n2;
        locals.var_t2_dn4 = assign70480_e106907_d_n4;
        locals.var_t2_dn5 = assign70480_e106907_d_n5;
        locals.var_t2_dn6 = assign70480_e106907_d_n6;
        locals.var_t2_dn7 = assign70480_e106907_d_n7;
        locals.var_t2_dn8 = assign70480_e106907_d_n8;
        locals.var_t2_dn9 = assign70480_e106907_d_n9;
        locals.var_t2_dn10 = assign70480_e106907_d_n10;
        locals.var_t2_dn11 = assign70480_e106907_d_n11;
        locals.var_t2_dn14 = assign70480_e106907_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign70490_e106917, assign70490_e106917_d_n0, assign70490_e106917_d_n2, assign70490_e106917_d_n4, assign70490_e106917_d_n5, assign70490_e106917_d_n6, assign70490_e106917_d_n7, assign70490_e106917_d_n8, assign70490_e106917_d_n9, assign70490_e106917_d_n10, assign70490_e106917_d_n11, assign70490_e106917_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70490_e106915: f64 = (locals.var_t1 / locals.var_t2);
        (assign70490_e106915, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign70490_e106917;
        locals.var_tmf1_dn0 = assign70490_e106917_d_n0;
        locals.var_tmf1_dn2 = assign70490_e106917_d_n2;
        locals.var_tmf1_dn4 = assign70490_e106917_d_n4;
        locals.var_tmf1_dn5 = assign70490_e106917_d_n5;
        locals.var_tmf1_dn6 = assign70490_e106917_d_n6;
        locals.var_tmf1_dn7 = assign70490_e106917_d_n7;
        locals.var_tmf1_dn8 = assign70490_e106917_d_n8;
        locals.var_tmf1_dn9 = assign70490_e106917_d_n9;
        locals.var_tmf1_dn10 = assign70490_e106917_d_n10;
        locals.var_tmf1_dn11 = assign70490_e106917_d_n11;
        locals.var_tmf1_dn14 = assign70490_e106917_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign70500_e106927, assign70500_e106927_d_n0, assign70500_e106927_d_n2, assign70500_e106927_d_n4, assign70500_e106927_d_n5, assign70500_e106927_d_n6, assign70500_e106927_d_n7, assign70500_e106927_d_n8, assign70500_e106927_d_n9, assign70500_e106927_d_n10, assign70500_e106927_d_n11, assign70500_e106927_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70500_e106925: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign70500_e106925, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70500_e106927;
        locals.var_tmf2_dn0 = assign70500_e106927_d_n0;
        locals.var_tmf2_dn2 = assign70500_e106927_d_n2;
        locals.var_tmf2_dn4 = assign70500_e106927_d_n4;
        locals.var_tmf2_dn5 = assign70500_e106927_d_n5;
        locals.var_tmf2_dn6 = assign70500_e106927_d_n6;
        locals.var_tmf2_dn7 = assign70500_e106927_d_n7;
        locals.var_tmf2_dn8 = assign70500_e106927_d_n8;
        locals.var_tmf2_dn9 = assign70500_e106927_d_n9;
        locals.var_tmf2_dn10 = assign70500_e106927_d_n10;
        locals.var_tmf2_dn11 = assign70500_e106927_d_n11;
        locals.var_tmf2_dn14 = assign70500_e106927_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign70510_e106937, assign70510_e106937_d_n0, assign70510_e106937_d_n2, assign70510_e106937_d_n4, assign70510_e106937_d_n5, assign70510_e106937_d_n6, assign70510_e106937_d_n7, assign70510_e106937_d_n8, assign70510_e106937_d_n9, assign70510_e106937_d_n10, assign70510_e106937_d_n11, assign70510_e106937_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70510_e106935: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign70510_e106935, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign70510_e106937;
        locals.var_tmf3_dn0 = assign70510_e106937_d_n0;
        locals.var_tmf3_dn2 = assign70510_e106937_d_n2;
        locals.var_tmf3_dn4 = assign70510_e106937_d_n4;
        locals.var_tmf3_dn5 = assign70510_e106937_d_n5;
        locals.var_tmf3_dn6 = assign70510_e106937_d_n6;
        locals.var_tmf3_dn7 = assign70510_e106937_d_n7;
        locals.var_tmf3_dn8 = assign70510_e106937_d_n8;
        locals.var_tmf3_dn9 = assign70510_e106937_d_n9;
        locals.var_tmf3_dn10 = assign70510_e106937_d_n10;
        locals.var_tmf3_dn11 = assign70510_e106937_d_n11;
        locals.var_tmf3_dn14 = assign70510_e106937_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign70520_e106947, assign70520_e106947_d_n0, assign70520_e106947_d_n2, assign70520_e106947_d_n4, assign70520_e106947_d_n5, assign70520_e106947_d_n6, assign70520_e106947_d_n7, assign70520_e106947_d_n8, assign70520_e106947_d_n9, assign70520_e106947_d_n10, assign70520_e106947_d_n11, assign70520_e106947_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70520_e106945: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign70520_e106945, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign70520_e106947;
        locals.var_tmf4_dn0 = assign70520_e106947_d_n0;
        locals.var_tmf4_dn2 = assign70520_e106947_d_n2;
        locals.var_tmf4_dn4 = assign70520_e106947_d_n4;
        locals.var_tmf4_dn5 = assign70520_e106947_d_n5;
        locals.var_tmf4_dn6 = assign70520_e106947_d_n6;
        locals.var_tmf4_dn7 = assign70520_e106947_d_n7;
        locals.var_tmf4_dn8 = assign70520_e106947_d_n8;
        locals.var_tmf4_dn9 = assign70520_e106947_d_n9;
        locals.var_tmf4_dn10 = assign70520_e106947_d_n10;
        locals.var_tmf4_dn11 = assign70520_e106947_d_n11;
        locals.var_tmf4_dn14 = assign70520_e106947_d_n14;
        locals.var_tmf4_rv = 0.0;

        let (assign70530_e106965, assign70530_e106965_d_n0, assign70530_e106965_d_n2, assign70530_e106965_d_n4, assign70530_e106965_d_n5, assign70530_e106965_d_n6, assign70530_e106965_d_n7, assign70530_e106965_d_n8, assign70530_e106965_d_n9, assign70530_e106965_d_n10, assign70530_e106965_d_n11, assign70530_e106965_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70530_e106956: f64 = (1.0 + locals.var_tmf1);
        let assign70530_e106958: f64 = (assign70530_e106956 + locals.var_tmf2);
        let assign70530_e106960: f64 = (assign70530_e106958 + locals.var_tmf3);
        let assign70530_e106962: f64 = (assign70530_e106960 + locals.var_tmf4);
        let assign70530_e106963: f64 = (1.0 / assign70530_e106962);
        (assign70530_e106963, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign70530_e106962 * assign70530_e106962))), (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign70530_e106962 * assign70530_e106962))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign70530_e106965;
        locals.var_tmf0_dn0 = assign70530_e106965_d_n0;
        locals.var_tmf0_dn2 = assign70530_e106965_d_n2;
        locals.var_tmf0_dn4 = assign70530_e106965_d_n4;
        locals.var_tmf0_dn5 = assign70530_e106965_d_n5;
        locals.var_tmf0_dn6 = assign70530_e106965_d_n6;
        locals.var_tmf0_dn7 = assign70530_e106965_d_n7;
        locals.var_tmf0_dn8 = assign70530_e106965_d_n8;
        locals.var_tmf0_dn9 = assign70530_e106965_d_n9;
        locals.var_tmf0_dn10 = assign70530_e106965_d_n10;
        locals.var_tmf0_dn11 = assign70530_e106965_d_n11;
        locals.var_tmf0_dn14 = assign70530_e106965_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign70540_e106990, assign70540_e106990_d_n0, assign70540_e106990_d_n2, assign70540_e106990_d_n4, assign70540_e106990_d_n5, assign70540_e106990_d_n6, assign70540_e106990_d_n7, assign70540_e106990_d_n8, assign70540_e106990_d_n9, assign70540_e106990_d_n10, assign70540_e106990_d_n11, assign70540_e106990_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70540_e106974: f64 = (2.0 * locals.var_tmf1);
        let assign70540_e106975: f64 = (1.0 + assign70540_e106974);
        let assign70540_e106978: f64 = (3.0 * locals.var_tmf2);
        let assign70540_e106979: f64 = (assign70540_e106975 + assign70540_e106978);
        let assign70540_e106982: f64 = (4.0 * locals.var_tmf3);
        let assign70540_e106983: f64 = (assign70540_e106979 + assign70540_e106982);
        let assign70540_e106984: f64 = (-assign70540_e106983);
        let assign70540_e106986: f64 = (assign70540_e106984 * locals.var_tmf0);
        let assign70540_e106988: f64 = (assign70540_e106986 * locals.var_tmf0);
        (assign70540_e106988, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn11)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn11)), (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tmf0) + (assign70540_e106984 * locals.var_tmf0_dn14)) * locals.var_tmf0) + (assign70540_e106986 * locals.var_tmf0_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign70540_e106990;
        locals.var_t11_dn0 = assign70540_e106990_d_n0;
        locals.var_t11_dn2 = assign70540_e106990_d_n2;
        locals.var_t11_dn4 = assign70540_e106990_d_n4;
        locals.var_t11_dn5 = assign70540_e106990_d_n5;
        locals.var_t11_dn6 = assign70540_e106990_d_n6;
        locals.var_t11_dn7 = assign70540_e106990_d_n7;
        locals.var_t11_dn8 = assign70540_e106990_d_n8;
        locals.var_t11_dn9 = assign70540_e106990_d_n9;
        locals.var_t11_dn10 = assign70540_e106990_d_n10;
        locals.var_t11_dn11 = assign70540_e106990_d_n11;
        locals.var_t11_dn14 = assign70540_e106990_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign70550_e107002, assign70550_e107002_d_n0, assign70550_e107002_d_n2, assign70550_e107002_d_n4, assign70550_e107002_d_n5, assign70550_e107002_d_n6, assign70550_e107002_d_n7, assign70550_e107002_d_n8, assign70550_e107002_d_n9, assign70550_e107002_d_n10, assign70550_e107002_d_n11, assign70550_e107002_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70550_e106999: f64 = (1.0 - locals.var_tmf0);
        let assign70550_e107000: f64 = (locals.var_t2 * assign70550_e106999);
        (assign70550_e107000, ((locals.var_t2_dn0 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn11 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn11))), ((locals.var_t2_dn14 * assign70550_e106999) + (locals.var_t2 * (-locals.var_tmf0_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign70550_e107002;
        locals.var_ty_dn0 = assign70550_e107002_d_n0;
        locals.var_ty_dn2 = assign70550_e107002_d_n2;
        locals.var_ty_dn4 = assign70550_e107002_d_n4;
        locals.var_ty_dn5 = assign70550_e107002_d_n5;
        locals.var_ty_dn6 = assign70550_e107002_d_n6;
        locals.var_ty_dn7 = assign70550_e107002_d_n7;
        locals.var_ty_dn8 = assign70550_e107002_d_n8;
        locals.var_ty_dn9 = assign70550_e107002_d_n9;
        locals.var_ty_dn10 = assign70550_e107002_d_n10;
        locals.var_ty_dn11 = assign70550_e107002_d_n11;
        locals.var_ty_dn14 = assign70550_e107002_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign70560_e107016, assign70560_e107016_d_n0, assign70560_e107016_d_n2, assign70560_e107016_d_n4, assign70560_e107016_d_n5, assign70560_e107016_d_n6, assign70560_e107016_d_n7, assign70560_e107016_d_n8, assign70560_e107016_d_n9, assign70560_e107016_d_n10, assign70560_e107016_d_n11, assign70560_e107016_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70560_e107010: f64 = (1.0 - locals.var_tmf0);
        let assign70560_e107013: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign70560_e107014: f64 = (assign70560_e107010 + assign70560_e107013);
        (assign70560_e107014, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn11) + ((locals.var_tmf1_dn11 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn11))), ((-locals.var_tmf0_dn14) + ((locals.var_tmf1_dn14 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70560_e107016;
        locals.var_t0_dn0 = assign70560_e107016_d_n0;
        locals.var_t0_dn2 = assign70560_e107016_d_n2;
        locals.var_t0_dn4 = assign70560_e107016_d_n4;
        locals.var_t0_dn5 = assign70560_e107016_d_n5;
        locals.var_t0_dn6 = assign70560_e107016_d_n6;
        locals.var_t0_dn7 = assign70560_e107016_d_n7;
        locals.var_t0_dn8 = assign70560_e107016_d_n8;
        locals.var_t0_dn9 = assign70560_e107016_d_n9;
        locals.var_t0_dn10 = assign70560_e107016_d_n10;
        locals.var_t0_dn11 = assign70560_e107016_d_n11;
        locals.var_t0_dn14 = assign70560_e107016_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign70570_e107025, assign70570_e107025_d_n0, assign70570_e107025_d_n2, assign70570_e107025_d_n4, assign70570_e107025_d_n5, assign70570_e107025_d_n6, assign70570_e107025_d_n7, assign70570_e107025_d_n8, assign70570_e107025_d_n9, assign70570_e107025_d_n10, assign70570_e107025_d_n11, assign70570_e107025_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70570_e107023: f64 = (-locals.var_t11);
        (assign70570_e107023, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11), (-locals.var_t11_dn14),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign70570_e107025;
        locals.var_t11_dn0 = assign70570_e107025_d_n0;
        locals.var_t11_dn2 = assign70570_e107025_d_n2;
        locals.var_t11_dn4 = assign70570_e107025_d_n4;
        locals.var_t11_dn5 = assign70570_e107025_d_n5;
        locals.var_t11_dn6 = assign70570_e107025_d_n6;
        locals.var_t11_dn7 = assign70570_e107025_d_n7;
        locals.var_t11_dn8 = assign70570_e107025_d_n8;
        locals.var_t11_dn9 = assign70570_e107025_d_n9;
        locals.var_t11_dn10 = assign70570_e107025_d_n10;
        locals.var_t11_dn11 = assign70570_e107025_d_n11;
        locals.var_t11_dn14 = assign70570_e107025_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign70580_e107035, assign70580_e107035_d_n0, assign70580_e107035_d_n2, assign70580_e107035_d_n4, assign70580_e107035_d_n5, assign70580_e107035_d_n6, assign70580_e107035_d_n7, assign70580_e107035_d_n8, assign70580_e107035_d_n9, assign70580_e107035_d_n10, assign70580_e107035_d_n11, assign70580_e107035_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 != 0.0)) {
        let assign70580_e107033: f64 = (locals.var_vbs_bnd_over + locals.var_ty);
        (assign70580_e107033, (locals.var_vbs_bnd_over_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over_dn11 + locals.var_ty_dn11), (locals.var_vbs_bnd_over_dn14 + locals.var_ty_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign70580_e107035;
        locals.var_t10_dn0 = assign70580_e107035_d_n0;
        locals.var_t10_dn2 = assign70580_e107035_d_n2;
        locals.var_t10_dn4 = assign70580_e107035_d_n4;
        locals.var_t10_dn5 = assign70580_e107035_d_n5;
        locals.var_t10_dn6 = assign70580_e107035_d_n6;
        locals.var_t10_dn7 = assign70580_e107035_d_n7;
        locals.var_t10_dn8 = assign70580_e107035_d_n8;
        locals.var_t10_dn9 = assign70580_e107035_d_n9;
        locals.var_t10_dn10 = assign70580_e107035_d_n10;
        locals.var_t10_dn11 = assign70580_e107035_d_n11;
        locals.var_t10_dn14 = assign70580_e107035_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign70590_e107044, assign70590_e107044_d_n0, assign70590_e107044_d_n2, assign70590_e107044_d_n4, assign70590_e107044_d_n5, assign70590_e107044_d_n6, assign70590_e107044_d_n7, assign70590_e107044_d_n8, assign70590_e107044_d_n9, assign70590_e107044_d_n10, assign70590_e107044_d_n11, assign70590_e107044_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) && (locals.var_guard1659 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign70590_e107044;
        locals.var_t10_dn0 = assign70590_e107044_d_n0;
        locals.var_t10_dn2 = assign70590_e107044_d_n2;
        locals.var_t10_dn4 = assign70590_e107044_d_n4;
        locals.var_t10_dn5 = assign70590_e107044_d_n5;
        locals.var_t10_dn6 = assign70590_e107044_d_n6;
        locals.var_t10_dn7 = assign70590_e107044_d_n7;
        locals.var_t10_dn8 = assign70590_e107044_d_n8;
        locals.var_t10_dn9 = assign70590_e107044_d_n9;
        locals.var_t10_dn10 = assign70590_e107044_d_n10;
        locals.var_t10_dn11 = assign70590_e107044_d_n11;
        locals.var_t10_dn14 = assign70590_e107044_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign70600_e107051, assign70600_e107051_d_n0, assign70600_e107051_d_n2, assign70600_e107051_d_n4, assign70600_e107051_d_n5, assign70600_e107051_d_n6, assign70600_e107051_d_n7, assign70600_e107051_d_n8, assign70600_e107051_d_n9, assign70600_e107051_d_n10, assign70600_e107051_d_n11, assign70600_e107051_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 != 0.0)) {
        let assign70600_e107049: f64 = (-locals.var_t10);
        (assign70600_e107049, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn11), (-locals.var_t10_dn14),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign70600_e107051;
        locals.var_vxbgmtcl_dn0 = assign70600_e107051_d_n0;
        locals.var_vxbgmtcl_dn2 = assign70600_e107051_d_n2;
        locals.var_vxbgmtcl_dn4 = assign70600_e107051_d_n4;
        locals.var_vxbgmtcl_dn5 = assign70600_e107051_d_n5;
        locals.var_vxbgmtcl_dn6 = assign70600_e107051_d_n6;
        locals.var_vxbgmtcl_dn7 = assign70600_e107051_d_n7;
        locals.var_vxbgmtcl_dn8 = assign70600_e107051_d_n8;
        locals.var_vxbgmtcl_dn9 = assign70600_e107051_d_n9;
        locals.var_vxbgmtcl_dn10 = assign70600_e107051_d_n10;
        locals.var_vxbgmtcl_dn11 = assign70600_e107051_d_n11;
        locals.var_vxbgmtcl_dn14 = assign70600_e107051_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign70610_e107058, assign70610_e107058_d_n0, assign70610_e107058_d_n2, assign70610_e107058_d_n4, assign70610_e107058_d_n5, assign70610_e107058_d_n6, assign70610_e107058_d_n7, assign70610_e107058_d_n8, assign70610_e107058_d_n9, assign70610_e107058_d_n10, assign70610_e107058_d_n11, assign70610_e107058_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1658 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign70610_e107058;
        locals.var_vxbgmtcl_dn0 = assign70610_e107058_d_n0;
        locals.var_vxbgmtcl_dn2 = assign70610_e107058_d_n2;
        locals.var_vxbgmtcl_dn4 = assign70610_e107058_d_n4;
        locals.var_vxbgmtcl_dn5 = assign70610_e107058_d_n5;
        locals.var_vxbgmtcl_dn6 = assign70610_e107058_d_n6;
        locals.var_vxbgmtcl_dn7 = assign70610_e107058_d_n7;
        locals.var_vxbgmtcl_dn8 = assign70610_e107058_d_n8;
        locals.var_vxbgmtcl_dn9 = assign70610_e107058_d_n9;
        locals.var_vxbgmtcl_dn10 = assign70610_e107058_d_n10;
        locals.var_vxbgmtcl_dn11 = assign70610_e107058_d_n11;
        locals.var_vxbgmtcl_dn14 = assign70610_e107058_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign70620_e107064, assign70620_e107064_d_n0, assign70620_e107064_d_n2, assign70620_e107064_d_n4, assign70620_e107064_d_n5, assign70620_e107064_d_n6, assign70620_e107064_d_n7, assign70620_e107064_d_n8, assign70620_e107064_d_n9, assign70620_e107064_d_n10, assign70620_e107064_d_n11, assign70620_e107064_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70620_e107062: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign70620_e107062, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn11 / locals.var_cox0_func), (locals.var_cnst0over_func_dn14 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn11, locals.var_fac1_dn14,)
    }
};
        locals.var_fac1 = assign70620_e107064;
        locals.var_fac1_dn0 = assign70620_e107064_d_n0;
        locals.var_fac1_dn2 = assign70620_e107064_d_n2;
        locals.var_fac1_dn4 = assign70620_e107064_d_n4;
        locals.var_fac1_dn5 = assign70620_e107064_d_n5;
        locals.var_fac1_dn6 = assign70620_e107064_d_n6;
        locals.var_fac1_dn7 = assign70620_e107064_d_n7;
        locals.var_fac1_dn8 = assign70620_e107064_d_n8;
        locals.var_fac1_dn9 = assign70620_e107064_d_n9;
        locals.var_fac1_dn10 = assign70620_e107064_d_n10;
        locals.var_fac1_dn11 = assign70620_e107064_d_n11;
        locals.var_fac1_dn14 = assign70620_e107064_d_n14;
        locals.var_fac1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_265(
        locals: &mut StampLocals,
    ) {
        let (assign70630_e107070, assign70630_e107070_d_n0, assign70630_e107070_d_n2, assign70630_e107070_d_n4, assign70630_e107070_d_n5, assign70630_e107070_d_n6, assign70630_e107070_d_n7, assign70630_e107070_d_n8, assign70630_e107070_d_n9, assign70630_e107070_d_n10, assign70630_e107070_d_n11, assign70630_e107070_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70630_e107068: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign70630_e107068, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11)), ((locals.var_fac1_dn14 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn14)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn11, locals.var_fac1p2_dn14,)
    }
};
        locals.var_fac1p2 = assign70630_e107070;
        locals.var_fac1p2_dn0 = assign70630_e107070_d_n0;
        locals.var_fac1p2_dn2 = assign70630_e107070_d_n2;
        locals.var_fac1p2_dn4 = assign70630_e107070_d_n4;
        locals.var_fac1p2_dn5 = assign70630_e107070_d_n5;
        locals.var_fac1p2_dn6 = assign70630_e107070_d_n6;
        locals.var_fac1p2_dn7 = assign70630_e107070_d_n7;
        locals.var_fac1p2_dn8 = assign70630_e107070_d_n8;
        locals.var_fac1p2_dn9 = assign70630_e107070_d_n9;
        locals.var_fac1p2_dn10 = assign70630_e107070_d_n10;
        locals.var_fac1p2_dn11 = assign70630_e107070_d_n11;
        locals.var_fac1p2_dn14 = assign70630_e107070_d_n14;
        locals.var_fac1p2_rv = 0.0;

        let (assign70640_e107077, assign70640_e107077_d_n2, assign70640_e107077_d_n7, assign70640_e107077_d_n8, assign70640_e107077_d_n9,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70640_e107073: f64 = (-locals.var_vgbgmt);
        let assign70640_e107075: f64 = (assign70640_e107073 + locals.var_uc_vfbover);
        (assign70640_e107075, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8), (-locals.var_vgbgmt_dn9),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn7, locals.var_vgpld_dn8, locals.var_vgpld_dn9,)
    }
};
        locals.var_vgpld = assign70640_e107077;
        locals.var_vgpld_dn2 = assign70640_e107077_d_n2;
        locals.var_vgpld_dn7 = assign70640_e107077_d_n7;
        locals.var_vgpld_dn8 = assign70640_e107077_d_n8;
        locals.var_vgpld_dn9 = assign70640_e107077_d_n9;
        locals.var_vgpld_rv = 0.0;

        let (assign70650_e107086, assign70650_e107086_d_n0, assign70650_e107086_d_n2, assign70650_e107086_d_n4, assign70650_e107086_d_n5, assign70650_e107086_d_n6, assign70650_e107086_d_n7, assign70650_e107086_d_n8, assign70650_e107086_d_n9, assign70650_e107086_d_n10, assign70650_e107086_d_n11, assign70650_e107086_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70650_e107080: f64 = (-locals.var_vxbgmtcl);
        let assign70650_e107083: f64 = (10.0 * 2.220446049250313e-16);
        let assign70650_e107084: f64 = (assign70650_e107080 + assign70650_e107083);
        (assign70650_e107084, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn14,)
    }
};
        locals.var_vgb_fb_ld = assign70650_e107086;
        locals.var_vgb_fb_ld_dn0 = assign70650_e107086_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign70650_e107086_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign70650_e107086_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign70650_e107086_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign70650_e107086_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign70650_e107086_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign70650_e107086_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign70650_e107086_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign70650_e107086_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign70650_e107086_d_n11;
        locals.var_vgb_fb_ld_dn14 = assign70650_e107086_d_n14;
        locals.var_vgb_fb_ld_rv = 0.0;

        let (assign70660_e107090, assign70660_e107090_d_n0, assign70660_e107090_d_n2, assign70660_e107090_d_n4, assign70660_e107090_d_n5, assign70660_e107090_d_n6, assign70660_e107090_d_n7, assign70660_e107090_d_n8, assign70660_e107090_d_n9, assign70660_e107090_d_n10, assign70660_e107090_d_n11, assign70660_e107090_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld, locals.var_q_dep_ld_dn0, locals.var_q_dep_ld_dn2, locals.var_q_dep_ld_dn4, locals.var_q_dep_ld_dn5, locals.var_q_dep_ld_dn6, locals.var_q_dep_ld_dn7, locals.var_q_dep_ld_dn8, locals.var_q_dep_ld_dn9, locals.var_q_dep_ld_dn10, locals.var_q_dep_ld_dn11, locals.var_q_dep_ld_dn14,)
    }
};
        locals.var_q_dep_ld = assign70660_e107090;
        locals.var_q_dep_ld_dn0 = assign70660_e107090_d_n0;
        locals.var_q_dep_ld_dn2 = assign70660_e107090_d_n2;
        locals.var_q_dep_ld_dn4 = assign70660_e107090_d_n4;
        locals.var_q_dep_ld_dn5 = assign70660_e107090_d_n5;
        locals.var_q_dep_ld_dn6 = assign70660_e107090_d_n6;
        locals.var_q_dep_ld_dn7 = assign70660_e107090_d_n7;
        locals.var_q_dep_ld_dn8 = assign70660_e107090_d_n8;
        locals.var_q_dep_ld_dn9 = assign70660_e107090_d_n9;
        locals.var_q_dep_ld_dn10 = assign70660_e107090_d_n10;
        locals.var_q_dep_ld_dn11 = assign70660_e107090_d_n11;
        locals.var_q_dep_ld_dn14 = assign70660_e107090_d_n14;
        locals.var_q_dep_ld_rv = 0.0;

        let (assign70670_e107096,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70670_e107094: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign70670_e107094,)
    } else {
        (locals.var_q_nsubld,)
    }
};
        locals.var_q_nsubld = assign70670_e107096;
        locals.var_q_nsubld_rv = 0.0;

        let (assign70680_e107102, assign70680_e107102_d_n0, assign70680_e107102_d_n2, assign70680_e107102_d_n4, assign70680_e107102_d_n5, assign70680_e107102_d_n6, assign70680_e107102_d_n7, assign70680_e107102_d_n8, assign70680_e107102_d_n9, assign70680_e107102_d_n10, assign70680_e107102_d_n11, assign70680_e107102_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70680_e107100: f64 = (locals.var_nin / locals.var_nover_func);
        (assign70680_e107100, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn11 / locals.var_nover_func), (locals.var_nin_dn14 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70680_e107102;
        locals.var_t0_dn0 = assign70680_e107102_d_n0;
        locals.var_t0_dn2 = assign70680_e107102_d_n2;
        locals.var_t0_dn4 = assign70680_e107102_d_n4;
        locals.var_t0_dn5 = assign70680_e107102_d_n5;
        locals.var_t0_dn6 = assign70680_e107102_d_n6;
        locals.var_t0_dn7 = assign70680_e107102_d_n7;
        locals.var_t0_dn8 = assign70680_e107102_d_n8;
        locals.var_t0_dn9 = assign70680_e107102_d_n9;
        locals.var_t0_dn10 = assign70680_e107102_d_n10;
        locals.var_t0_dn11 = assign70680_e107102_d_n11;
        locals.var_t0_dn14 = assign70680_e107102_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign70690_e107108, assign70690_e107108_d_n0, assign70690_e107108_d_n2, assign70690_e107108_d_n4, assign70690_e107108_d_n5, assign70690_e107108_d_n6, assign70690_e107108_d_n7, assign70690_e107108_d_n8, assign70690_e107108_d_n9, assign70690_e107108_d_n10, assign70690_e107108_d_n11, assign70690_e107108_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign70690_e107106: f64 = (locals.var_t0 * locals.var_t0);
        (assign70690_e107106, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn14,)
    }
};
        locals.var_cnst1over = assign70690_e107108;
        locals.var_cnst1over_dn0 = assign70690_e107108_d_n0;
        locals.var_cnst1over_dn2 = assign70690_e107108_d_n2;
        locals.var_cnst1over_dn4 = assign70690_e107108_d_n4;
        locals.var_cnst1over_dn5 = assign70690_e107108_d_n5;
        locals.var_cnst1over_dn6 = assign70690_e107108_d_n6;
        locals.var_cnst1over_dn7 = assign70690_e107108_d_n7;
        locals.var_cnst1over_dn8 = assign70690_e107108_d_n8;
        locals.var_cnst1over_dn9 = assign70690_e107108_d_n9;
        locals.var_cnst1over_dn10 = assign70690_e107108_d_n10;
        locals.var_cnst1over_dn11 = assign70690_e107108_d_n11;
        locals.var_cnst1over_dn14 = assign70690_e107108_d_n14;
        locals.var_cnst1over_rv = 0.0;

        let assign70700_e107111: f64 = (-locals.var_vxbgmtcl);
        let assign70700_e107112: f64 = (locals.var_beta * assign70700_e107111);
        let assign70700_e107114: f64 = if assign70700_e107112 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1660 = assign70700_e107114;
        locals.var_guard1660_rv = 0.0;

        let (assign70710_e107129, assign70710_e107129_d_n0, assign70710_e107129_d_n2, assign70710_e107129_d_n4, assign70710_e107129_d_n5, assign70710_e107129_d_n6, assign70710_e107129_d_n7, assign70710_e107129_d_n8, assign70710_e107129_d_n9, assign70710_e107129_d_n10, assign70710_e107129_d_n11, assign70710_e107129_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) {
        let assign70710_e107122: f64 = (-locals.var_vxbgmtcl);
        let assign70710_e107123: f64 = (locals.var_beta * assign70710_e107122);
        let assign70710_e107124: f64 = (1.0 + assign70710_e107123);
        let assign70710_e107126: f64 = (assign70710_e107124 - 500.0);
        let assign70710_e107127: f64 = (1.403592217853e217 * assign70710_e107126);
        (assign70710_e107127, (1.403592217853e217 * ((locals.var_beta_dn0 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn11 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11)))), (1.403592217853e217 * ((locals.var_beta_dn14 * assign70710_e107122) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign70710_e107129;
        locals.var_exp_bvbs_dn0 = assign70710_e107129_d_n0;
        locals.var_exp_bvbs_dn2 = assign70710_e107129_d_n2;
        locals.var_exp_bvbs_dn4 = assign70710_e107129_d_n4;
        locals.var_exp_bvbs_dn5 = assign70710_e107129_d_n5;
        locals.var_exp_bvbs_dn6 = assign70710_e107129_d_n6;
        locals.var_exp_bvbs_dn7 = assign70710_e107129_d_n7;
        locals.var_exp_bvbs_dn8 = assign70710_e107129_d_n8;
        locals.var_exp_bvbs_dn9 = assign70710_e107129_d_n9;
        locals.var_exp_bvbs_dn10 = assign70710_e107129_d_n10;
        locals.var_exp_bvbs_dn11 = assign70710_e107129_d_n11;
        locals.var_exp_bvbs_dn14 = assign70710_e107129_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign70720_e107135, assign70720_e107135_d_n0, assign70720_e107135_d_n2, assign70720_e107135_d_n4, assign70720_e107135_d_n5, assign70720_e107135_d_n6, assign70720_e107135_d_n7, assign70720_e107135_d_n8, assign70720_e107135_d_n9, assign70720_e107135_d_n10, assign70720_e107135_d_n11, assign70720_e107135_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70720_e107135;
        locals.var_t0_dn0 = assign70720_e107135_d_n0;
        locals.var_t0_dn2 = assign70720_e107135_d_n2;
        locals.var_t0_dn4 = assign70720_e107135_d_n4;
        locals.var_t0_dn5 = assign70720_e107135_d_n5;
        locals.var_t0_dn6 = assign70720_e107135_d_n6;
        locals.var_t0_dn7 = assign70720_e107135_d_n7;
        locals.var_t0_dn8 = assign70720_e107135_d_n8;
        locals.var_t0_dn9 = assign70720_e107135_d_n9;
        locals.var_t0_dn10 = assign70720_e107135_d_n10;
        locals.var_t0_dn11 = assign70720_e107135_d_n11;
        locals.var_t0_dn14 = assign70720_e107135_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign70730_e107145, assign70730_e107145_d_n0, assign70730_e107145_d_n2, assign70730_e107145_d_n4, assign70730_e107145_d_n5, assign70730_e107145_d_n6, assign70730_e107145_d_n7, assign70730_e107145_d_n8, assign70730_e107145_d_n9, assign70730_e107145_d_n10, assign70730_e107145_d_n11, assign70730_e107145_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 == 0.0)) {
        let assign70730_e107142: f64 = (-locals.var_vxbgmtcl);
        let assign70730_e107143: f64 = (locals.var_beta * assign70730_e107142);
        (assign70730_e107143, ((locals.var_beta_dn0 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign70730_e107142) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign70730_e107145;
        locals.var_tmf1_dn0 = assign70730_e107145_d_n0;
        locals.var_tmf1_dn2 = assign70730_e107145_d_n2;
        locals.var_tmf1_dn4 = assign70730_e107145_d_n4;
        locals.var_tmf1_dn5 = assign70730_e107145_d_n5;
        locals.var_tmf1_dn6 = assign70730_e107145_d_n6;
        locals.var_tmf1_dn7 = assign70730_e107145_d_n7;
        locals.var_tmf1_dn8 = assign70730_e107145_d_n8;
        locals.var_tmf1_dn9 = assign70730_e107145_d_n9;
        locals.var_tmf1_dn10 = assign70730_e107145_d_n10;
        locals.var_tmf1_dn11 = assign70730_e107145_d_n11;
        locals.var_tmf1_dn14 = assign70730_e107145_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign70740_e107152, assign70740_e107152_d_n0, assign70740_e107152_d_n2, assign70740_e107152_d_n4, assign70740_e107152_d_n5, assign70740_e107152_d_n6, assign70740_e107152_d_n7, assign70740_e107152_d_n8, assign70740_e107152_d_n9, assign70740_e107152_d_n10, assign70740_e107152_d_n11, assign70740_e107152_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign70740_e107152;
        locals.var_exp_bvbs_dn0 = assign70740_e107152_d_n0;
        locals.var_exp_bvbs_dn2 = assign70740_e107152_d_n2;
        locals.var_exp_bvbs_dn4 = assign70740_e107152_d_n4;
        locals.var_exp_bvbs_dn5 = assign70740_e107152_d_n5;
        locals.var_exp_bvbs_dn6 = assign70740_e107152_d_n6;
        locals.var_exp_bvbs_dn7 = assign70740_e107152_d_n7;
        locals.var_exp_bvbs_dn8 = assign70740_e107152_d_n8;
        locals.var_exp_bvbs_dn9 = assign70740_e107152_d_n9;
        locals.var_exp_bvbs_dn10 = assign70740_e107152_d_n10;
        locals.var_exp_bvbs_dn11 = assign70740_e107152_d_n11;
        locals.var_exp_bvbs_dn14 = assign70740_e107152_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let mut assign70750_loop_guard: usize = 0;
        while {
            let assign70750_cond_e107160: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign70750_cond_e107160 != 0.0
        } {
            assign70750_loop_guard += 1;
            assert!(assign70750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign70750_body0_e107169, assign70750_body0_e107169_d_n0, assign70750_body0_e107169_d_n2, assign70750_body0_e107169_d_n4, assign70750_body0_e107169_d_n5, assign70750_body0_e107169_d_n6, assign70750_body0_e107169_d_n7, assign70750_body0_e107169_d_n8, assign70750_body0_e107169_d_n9, assign70750_body0_e107169_d_n10, assign70750_body0_e107169_d_n11, assign70750_body0_e107169_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 == 0.0)) {
        let assign70750_body0_e107167: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign70750_body0_e107167, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn11 * 1.14200738981568e26), (locals.var_exp_bvbs_dn14 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
            locals.var_exp_bvbs = assign70750_body0_e107169;
            locals.var_exp_bvbs_dn0 = assign70750_body0_e107169_d_n0;
            locals.var_exp_bvbs_dn2 = assign70750_body0_e107169_d_n2;
            locals.var_exp_bvbs_dn4 = assign70750_body0_e107169_d_n4;
            locals.var_exp_bvbs_dn5 = assign70750_body0_e107169_d_n5;
            locals.var_exp_bvbs_dn6 = assign70750_body0_e107169_d_n6;
            locals.var_exp_bvbs_dn7 = assign70750_body0_e107169_d_n7;
            locals.var_exp_bvbs_dn8 = assign70750_body0_e107169_d_n8;
            locals.var_exp_bvbs_dn9 = assign70750_body0_e107169_d_n9;
            locals.var_exp_bvbs_dn10 = assign70750_body0_e107169_d_n10;
            locals.var_exp_bvbs_dn11 = assign70750_body0_e107169_d_n11;
            locals.var_exp_bvbs_dn14 = assign70750_body0_e107169_d_n14;
            locals.var_exp_bvbs_rv = 0.0;
            let (assign70750_body1_e107178, assign70750_body1_e107178_d_n0, assign70750_body1_e107178_d_n2, assign70750_body1_e107178_d_n4, assign70750_body1_e107178_d_n5, assign70750_body1_e107178_d_n6, assign70750_body1_e107178_d_n7, assign70750_body1_e107178_d_n8, assign70750_body1_e107178_d_n9, assign70750_body1_e107178_d_n10, assign70750_body1_e107178_d_n11, assign70750_body1_e107178_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 == 0.0)) {
        let assign70750_body1_e107176: f64 = (locals.var_tmf1 - 60.0);
        (assign70750_body1_e107176, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign70750_body1_e107178;
            locals.var_tmf1_dn0 = assign70750_body1_e107178_d_n0;
            locals.var_tmf1_dn2 = assign70750_body1_e107178_d_n2;
            locals.var_tmf1_dn4 = assign70750_body1_e107178_d_n4;
            locals.var_tmf1_dn5 = assign70750_body1_e107178_d_n5;
            locals.var_tmf1_dn6 = assign70750_body1_e107178_d_n6;
            locals.var_tmf1_dn7 = assign70750_body1_e107178_d_n7;
            locals.var_tmf1_dn8 = assign70750_body1_e107178_d_n8;
            locals.var_tmf1_dn9 = assign70750_body1_e107178_d_n9;
            locals.var_tmf1_dn10 = assign70750_body1_e107178_d_n10;
            locals.var_tmf1_dn11 = assign70750_body1_e107178_d_n11;
            locals.var_tmf1_dn14 = assign70750_body1_e107178_d_n14;
            locals.var_tmf1_rv = 0.0;
        }

        let (assign70760_e107188, assign70760_e107188_d_n0, assign70760_e107188_d_n2, assign70760_e107188_d_n4, assign70760_e107188_d_n5, assign70760_e107188_d_n6, assign70760_e107188_d_n7, assign70760_e107188_d_n8, assign70760_e107188_d_n9, assign70760_e107188_d_n10, assign70760_e107188_d_n11, assign70760_e107188_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 == 0.0)) {
        let assign70760_e107185: f64 = (locals.var_tmf1).exp();
        let assign70760_e107186: f64 = (locals.var_exp_bvbs * assign70760_e107185);
        (assign70760_e107186, ((locals.var_exp_bvbs_dn0 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn11 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn11))), ((locals.var_exp_bvbs_dn14 * assign70760_e107185) + (locals.var_exp_bvbs * (assign70760_e107185 * locals.var_tmf1_dn14))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign70760_e107188;
        locals.var_exp_bvbs_dn0 = assign70760_e107188_d_n0;
        locals.var_exp_bvbs_dn2 = assign70760_e107188_d_n2;
        locals.var_exp_bvbs_dn4 = assign70760_e107188_d_n4;
        locals.var_exp_bvbs_dn5 = assign70760_e107188_d_n5;
        locals.var_exp_bvbs_dn6 = assign70760_e107188_d_n6;
        locals.var_exp_bvbs_dn7 = assign70760_e107188_d_n7;
        locals.var_exp_bvbs_dn8 = assign70760_e107188_d_n8;
        locals.var_exp_bvbs_dn9 = assign70760_e107188_d_n9;
        locals.var_exp_bvbs_dn10 = assign70760_e107188_d_n10;
        locals.var_exp_bvbs_dn11 = assign70760_e107188_d_n11;
        locals.var_exp_bvbs_dn14 = assign70760_e107188_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign70770_e107195, assign70770_e107195_d_n0, assign70770_e107195_d_n2, assign70770_e107195_d_n4, assign70770_e107195_d_n5, assign70770_e107195_d_n6, assign70770_e107195_d_n7, assign70770_e107195_d_n8, assign70770_e107195_d_n9, assign70770_e107195_d_n10, assign70770_e107195_d_n11, assign70770_e107195_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1660 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70770_e107195;
        locals.var_t0_dn0 = assign70770_e107195_d_n0;
        locals.var_t0_dn2 = assign70770_e107195_d_n2;
        locals.var_t0_dn4 = assign70770_e107195_d_n4;
        locals.var_t0_dn5 = assign70770_e107195_d_n5;
        locals.var_t0_dn6 = assign70770_e107195_d_n6;
        locals.var_t0_dn7 = assign70770_e107195_d_n7;
        locals.var_t0_dn8 = assign70770_e107195_d_n8;
        locals.var_t0_dn9 = assign70770_e107195_d_n9;
        locals.var_t0_dn10 = assign70770_e107195_d_n10;
        locals.var_t0_dn11 = assign70770_e107195_d_n11;
        locals.var_t0_dn14 = assign70770_e107195_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign70780_e107208, assign70780_e107208_d_n0, assign70780_e107208_d_n2, assign70780_e107208_d_n4, assign70780_e107208_d_n5, assign70780_e107208_d_n6, assign70780_e107208_d_n7, assign70780_e107208_d_n8, assign70780_e107208_d_n9, assign70780_e107208_d_n10, assign70780_e107208_d_n11, assign70780_e107208_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70780_e107200: f64 = (-locals.var_vgpld);
        let assign70780_e107202: f64 = (assign70780_e107200 * 0.5);
        let assign70780_e107204: f64 = (assign70780_e107202 - 0.5);
        let assign70780_e107206: f64 = (assign70780_e107204 - 1.0);
        (assign70780_e107206, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, 0.0, ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), ((-locals.var_vgpld_dn9) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign70780_e107208;
        locals.var_tmf1_dn0 = assign70780_e107208_d_n0;
        locals.var_tmf1_dn2 = assign70780_e107208_d_n2;
        locals.var_tmf1_dn4 = assign70780_e107208_d_n4;
        locals.var_tmf1_dn5 = assign70780_e107208_d_n5;
        locals.var_tmf1_dn6 = assign70780_e107208_d_n6;
        locals.var_tmf1_dn7 = assign70780_e107208_d_n7;
        locals.var_tmf1_dn8 = assign70780_e107208_d_n8;
        locals.var_tmf1_dn9 = assign70780_e107208_d_n9;
        locals.var_tmf1_dn10 = assign70780_e107208_d_n10;
        locals.var_tmf1_dn11 = assign70780_e107208_d_n11;
        locals.var_tmf1_dn14 = assign70780_e107208_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign70790_e107218, assign70790_e107218_d_n0, assign70790_e107218_d_n2, assign70790_e107218_d_n4, assign70790_e107218_d_n5, assign70790_e107218_d_n6, assign70790_e107218_d_n7, assign70790_e107218_d_n8, assign70790_e107218_d_n9, assign70790_e107218_d_n10, assign70790_e107218_d_n11, assign70790_e107218_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70790_e107214: f64 = (4.0 * 0.5);
        let assign70790_e107216: f64 = assign70790_e107214;
        (assign70790_e107216, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70790_e107218;
        locals.var_tmf2_dn0 = assign70790_e107218_d_n0;
        locals.var_tmf2_dn2 = assign70790_e107218_d_n2;
        locals.var_tmf2_dn4 = assign70790_e107218_d_n4;
        locals.var_tmf2_dn5 = assign70790_e107218_d_n5;
        locals.var_tmf2_dn6 = assign70790_e107218_d_n6;
        locals.var_tmf2_dn7 = assign70790_e107218_d_n7;
        locals.var_tmf2_dn8 = assign70790_e107218_d_n8;
        locals.var_tmf2_dn9 = assign70790_e107218_d_n9;
        locals.var_tmf2_dn10 = assign70790_e107218_d_n10;
        locals.var_tmf2_dn11 = assign70790_e107218_d_n11;
        locals.var_tmf2_dn14 = assign70790_e107218_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign70800_e107230, assign70800_e107230_d_n0, assign70800_e107230_d_n2, assign70800_e107230_d_n4, assign70800_e107230_d_n5, assign70800_e107230_d_n6, assign70800_e107230_d_n7, assign70800_e107230_d_n8, assign70800_e107230_d_n9, assign70800_e107230_d_n10, assign70800_e107230_d_n11, assign70800_e107230_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign70800_e107228, assign70800_e107228_d_n0, assign70800_e107228_d_n2, assign70800_e107228_d_n4, assign70800_e107228_d_n5, assign70800_e107228_d_n6, assign70800_e107228_d_n7, assign70800_e107228_d_n8, assign70800_e107228_d_n9, assign70800_e107228_d_n10, assign70800_e107228_d_n11, assign70800_e107228_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign70800_e107227: f64 = (-locals.var_tmf2);
                (assign70800_e107227, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign70800_e107228, assign70800_e107228_d_n0, assign70800_e107228_d_n2, assign70800_e107228_d_n4, assign70800_e107228_d_n5, assign70800_e107228_d_n6, assign70800_e107228_d_n7, assign70800_e107228_d_n8, assign70800_e107228_d_n9, assign70800_e107228_d_n10, assign70800_e107228_d_n11, assign70800_e107228_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70800_e107230;
        locals.var_tmf2_dn0 = assign70800_e107230_d_n0;
        locals.var_tmf2_dn2 = assign70800_e107230_d_n2;
        locals.var_tmf2_dn4 = assign70800_e107230_d_n4;
        locals.var_tmf2_dn5 = assign70800_e107230_d_n5;
        locals.var_tmf2_dn6 = assign70800_e107230_d_n6;
        locals.var_tmf2_dn7 = assign70800_e107230_d_n7;
        locals.var_tmf2_dn8 = assign70800_e107230_d_n8;
        locals.var_tmf2_dn9 = assign70800_e107230_d_n9;
        locals.var_tmf2_dn10 = assign70800_e107230_d_n10;
        locals.var_tmf2_dn11 = assign70800_e107230_d_n11;
        locals.var_tmf2_dn14 = assign70800_e107230_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign70810_e107241, assign70810_e107241_d_n0, assign70810_e107241_d_n2, assign70810_e107241_d_n4, assign70810_e107241_d_n5, assign70810_e107241_d_n6, assign70810_e107241_d_n7, assign70810_e107241_d_n8, assign70810_e107241_d_n9, assign70810_e107241_d_n10, assign70810_e107241_d_n11, assign70810_e107241_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70810_e107236: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign70810_e107238: f64 = (assign70810_e107236 + locals.var_tmf2);
        let assign70810_e107239: f64 = (assign70810_e107238).sqrt();
        (assign70810_e107239, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign70810_e107239)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign70810_e107239)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign70810_e107241;
        locals.var_tmf2_dn0 = assign70810_e107241_d_n0;
        locals.var_tmf2_dn2 = assign70810_e107241_d_n2;
        locals.var_tmf2_dn4 = assign70810_e107241_d_n4;
        locals.var_tmf2_dn5 = assign70810_e107241_d_n5;
        locals.var_tmf2_dn6 = assign70810_e107241_d_n6;
        locals.var_tmf2_dn7 = assign70810_e107241_d_n7;
        locals.var_tmf2_dn8 = assign70810_e107241_d_n8;
        locals.var_tmf2_dn9 = assign70810_e107241_d_n9;
        locals.var_tmf2_dn10 = assign70810_e107241_d_n10;
        locals.var_tmf2_dn11 = assign70810_e107241_d_n11;
        locals.var_tmf2_dn14 = assign70810_e107241_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign70820_e107253, assign70820_e107253_d_n0, assign70820_e107253_d_n2, assign70820_e107253_d_n4, assign70820_e107253_d_n5, assign70820_e107253_d_n6, assign70820_e107253_d_n7, assign70820_e107253_d_n8, assign70820_e107253_d_n9, assign70820_e107253_d_n10, assign70820_e107253_d_n11, assign70820_e107253_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70820_e107249: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign70820_e107250: f64 = (1.0 + assign70820_e107249);
        let assign70820_e107251: f64 = (0.5 * assign70820_e107250);
        (assign70820_e107251, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign70820_e107253;
        locals.var_t0_dn0 = assign70820_e107253_d_n0;
        locals.var_t0_dn2 = assign70820_e107253_d_n2;
        locals.var_t0_dn4 = assign70820_e107253_d_n4;
        locals.var_t0_dn5 = assign70820_e107253_d_n5;
        locals.var_t0_dn6 = assign70820_e107253_d_n6;
        locals.var_t0_dn7 = assign70820_e107253_d_n7;
        locals.var_t0_dn8 = assign70820_e107253_d_n8;
        locals.var_t0_dn9 = assign70820_e107253_d_n9;
        locals.var_t0_dn10 = assign70820_e107253_d_n10;
        locals.var_t0_dn11 = assign70820_e107253_d_n11;
        locals.var_t0_dn14 = assign70820_e107253_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign70830_e107265, assign70830_e107265_d_n0, assign70830_e107265_d_n2, assign70830_e107265_d_n4, assign70830_e107265_d_n5, assign70830_e107265_d_n6, assign70830_e107265_d_n7, assign70830_e107265_d_n8, assign70830_e107265_d_n9, assign70830_e107265_d_n10, assign70830_e107265_d_n11, assign70830_e107265_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign70830_e107261: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign70830_e107262: f64 = (0.5 * assign70830_e107261);
        let assign70830_e107263: f64 = (0.5 + assign70830_e107262);
        (assign70830_e107263, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign70830_e107265;
        locals.var_t1_dn0 = assign70830_e107265_d_n0;
        locals.var_t1_dn2 = assign70830_e107265_d_n2;
        locals.var_t1_dn4 = assign70830_e107265_d_n4;
        locals.var_t1_dn5 = assign70830_e107265_d_n5;
        locals.var_t1_dn6 = assign70830_e107265_d_n6;
        locals.var_t1_dn7 = assign70830_e107265_d_n7;
        locals.var_t1_dn8 = assign70830_e107265_d_n8;
        locals.var_t1_dn9 = assign70830_e107265_d_n9;
        locals.var_t1_dn10 = assign70830_e107265_d_n10;
        locals.var_t1_dn11 = assign70830_e107265_d_n11;
        locals.var_t1_dn14 = assign70830_e107265_d_n14;
        locals.var_t1_rv = 0.0;

        let assign70840_e107268: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign70840_e107271: f64 = (-locals.var_t1);
        let assign70840_e107276: f64 = if ((assign70840_e107268 > assign70840_e107271) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1661 = assign70840_e107276;
        locals.var_guard1661_rv = 0.0;

        let (assign70850_e107290, assign70850_e107290_d_n0, assign70850_e107290_d_n2, assign70850_e107290_d_n4, assign70850_e107290_d_n5, assign70850_e107290_d_n6, assign70850_e107290_d_n7, assign70850_e107290_d_n8, assign70850_e107290_d_n9, assign70850_e107290_d_n10, assign70850_e107290_d_n11, assign70850_e107290_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70850_e107284: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign70850_e107286: f64 = assign70850_e107284;
        let assign70850_e107288: f64 = (assign70850_e107286 + locals.var_t1);
        (assign70850_e107288, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), (locals.var_vxbgmtcl_dn6 + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), ((locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9) + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn11 + locals.var_t1_dn11), (locals.var_vxbgmtcl_dn14 + locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign70850_e107290;
        locals.var_tmf1_dn0 = assign70850_e107290_d_n0;
        locals.var_tmf1_dn2 = assign70850_e107290_d_n2;
        locals.var_tmf1_dn4 = assign70850_e107290_d_n4;
        locals.var_tmf1_dn5 = assign70850_e107290_d_n5;
        locals.var_tmf1_dn6 = assign70850_e107290_d_n6;
        locals.var_tmf1_dn7 = assign70850_e107290_d_n7;
        locals.var_tmf1_dn8 = assign70850_e107290_d_n8;
        locals.var_tmf1_dn9 = assign70850_e107290_d_n9;
        locals.var_tmf1_dn10 = assign70850_e107290_d_n10;
        locals.var_tmf1_dn11 = assign70850_e107290_d_n11;
        locals.var_tmf1_dn14 = assign70850_e107290_d_n14;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_266(
        locals: &mut StampLocals,
    ) {
        let (assign70860_e107300, assign70860_e107300_d_n0, assign70860_e107300_d_n2, assign70860_e107300_d_n4, assign70860_e107300_d_n5, assign70860_e107300_d_n6, assign70860_e107300_d_n7, assign70860_e107300_d_n8, assign70860_e107300_d_n9, assign70860_e107300_d_n10, assign70860_e107300_d_n11, assign70860_e107300_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70860_e107298: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign70860_e107298, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign70860_e107300;
        locals.var_x2_dn0 = assign70860_e107300_d_n0;
        locals.var_x2_dn2 = assign70860_e107300_d_n2;
        locals.var_x2_dn4 = assign70860_e107300_d_n4;
        locals.var_x2_dn5 = assign70860_e107300_d_n5;
        locals.var_x2_dn6 = assign70860_e107300_d_n6;
        locals.var_x2_dn7 = assign70860_e107300_d_n7;
        locals.var_x2_dn8 = assign70860_e107300_d_n8;
        locals.var_x2_dn9 = assign70860_e107300_d_n9;
        locals.var_x2_dn10 = assign70860_e107300_d_n10;
        locals.var_x2_dn11 = assign70860_e107300_d_n11;
        locals.var_x2_dn14 = assign70860_e107300_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign70870_e107310, assign70870_e107310_d_n0, assign70870_e107310_d_n2, assign70870_e107310_d_n4, assign70870_e107310_d_n5, assign70870_e107310_d_n6, assign70870_e107310_d_n7, assign70870_e107310_d_n8, assign70870_e107310_d_n9, assign70870_e107310_d_n10, assign70870_e107310_d_n11, assign70870_e107310_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70870_e107308: f64 = (locals.var_t1 * locals.var_t1);
        (assign70870_e107308, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign70870_e107310;
        locals.var_xmax2_dn0 = assign70870_e107310_d_n0;
        locals.var_xmax2_dn2 = assign70870_e107310_d_n2;
        locals.var_xmax2_dn4 = assign70870_e107310_d_n4;
        locals.var_xmax2_dn5 = assign70870_e107310_d_n5;
        locals.var_xmax2_dn6 = assign70870_e107310_d_n6;
        locals.var_xmax2_dn7 = assign70870_e107310_d_n7;
        locals.var_xmax2_dn8 = assign70870_e107310_d_n8;
        locals.var_xmax2_dn9 = assign70870_e107310_d_n9;
        locals.var_xmax2_dn10 = assign70870_e107310_d_n10;
        locals.var_xmax2_dn11 = assign70870_e107310_d_n11;
        locals.var_xmax2_dn14 = assign70870_e107310_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign70880_e107318, assign70880_e107318_d_n0, assign70880_e107318_d_n2, assign70880_e107318_d_n4, assign70880_e107318_d_n5, assign70880_e107318_d_n6, assign70880_e107318_d_n7, assign70880_e107318_d_n8, assign70880_e107318_d_n9, assign70880_e107318_d_n10, assign70880_e107318_d_n11, assign70880_e107318_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign70880_e107318;
        locals.var_xp_dn0 = assign70880_e107318_d_n0;
        locals.var_xp_dn2 = assign70880_e107318_d_n2;
        locals.var_xp_dn4 = assign70880_e107318_d_n4;
        locals.var_xp_dn5 = assign70880_e107318_d_n5;
        locals.var_xp_dn6 = assign70880_e107318_d_n6;
        locals.var_xp_dn7 = assign70880_e107318_d_n7;
        locals.var_xp_dn8 = assign70880_e107318_d_n8;
        locals.var_xp_dn9 = assign70880_e107318_d_n9;
        locals.var_xp_dn10 = assign70880_e107318_d_n10;
        locals.var_xp_dn11 = assign70880_e107318_d_n11;
        locals.var_xp_dn14 = assign70880_e107318_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign70890_e107326, assign70890_e107326_d_n0, assign70890_e107326_d_n2, assign70890_e107326_d_n4, assign70890_e107326_d_n5, assign70890_e107326_d_n6, assign70890_e107326_d_n7, assign70890_e107326_d_n8, assign70890_e107326_d_n9, assign70890_e107326_d_n10, assign70890_e107326_d_n11, assign70890_e107326_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign70890_e107326;
        locals.var_xmp_dn0 = assign70890_e107326_d_n0;
        locals.var_xmp_dn2 = assign70890_e107326_d_n2;
        locals.var_xmp_dn4 = assign70890_e107326_d_n4;
        locals.var_xmp_dn5 = assign70890_e107326_d_n5;
        locals.var_xmp_dn6 = assign70890_e107326_d_n6;
        locals.var_xmp_dn7 = assign70890_e107326_d_n7;
        locals.var_xmp_dn8 = assign70890_e107326_d_n8;
        locals.var_xmp_dn9 = assign70890_e107326_d_n9;
        locals.var_xmp_dn10 = assign70890_e107326_d_n10;
        locals.var_xmp_dn11 = assign70890_e107326_d_n11;
        locals.var_xmp_dn14 = assign70890_e107326_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign70900_e107334,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign70900_e107334;
        locals.var_m0_rv = 0.0;

        let (assign70910_e107342,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign70910_e107342;
        locals.var_mm_rv = 0.0;

        let (assign70920_e107350, assign70920_e107350_d_n0, assign70920_e107350_d_n2, assign70920_e107350_d_n4, assign70920_e107350_d_n5, assign70920_e107350_d_n6, assign70920_e107350_d_n7, assign70920_e107350_d_n8, assign70920_e107350_d_n9, assign70920_e107350_d_n10, assign70920_e107350_d_n11, assign70920_e107350_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign70920_e107350;
        locals.var_arg_dn0 = assign70920_e107350_d_n0;
        locals.var_arg_dn2 = assign70920_e107350_d_n2;
        locals.var_arg_dn4 = assign70920_e107350_d_n4;
        locals.var_arg_dn5 = assign70920_e107350_d_n5;
        locals.var_arg_dn6 = assign70920_e107350_d_n6;
        locals.var_arg_dn7 = assign70920_e107350_d_n7;
        locals.var_arg_dn8 = assign70920_e107350_d_n8;
        locals.var_arg_dn9 = assign70920_e107350_d_n9;
        locals.var_arg_dn10 = assign70920_e107350_d_n10;
        locals.var_arg_dn11 = assign70920_e107350_d_n11;
        locals.var_arg_dn14 = assign70920_e107350_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign70930_e107358, assign70930_e107358_d_n0, assign70930_e107358_d_n2, assign70930_e107358_d_n4, assign70930_e107358_d_n5, assign70930_e107358_d_n6, assign70930_e107358_d_n7, assign70930_e107358_d_n8, assign70930_e107358_d_n9, assign70930_e107358_d_n10, assign70930_e107358_d_n11, assign70930_e107358_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign70930_e107358;
        locals.var_dnm_dn0 = assign70930_e107358_d_n0;
        locals.var_dnm_dn2 = assign70930_e107358_d_n2;
        locals.var_dnm_dn4 = assign70930_e107358_d_n4;
        locals.var_dnm_dn5 = assign70930_e107358_d_n5;
        locals.var_dnm_dn6 = assign70930_e107358_d_n6;
        locals.var_dnm_dn7 = assign70930_e107358_d_n7;
        locals.var_dnm_dn8 = assign70930_e107358_d_n8;
        locals.var_dnm_dn9 = assign70930_e107358_d_n9;
        locals.var_dnm_dn10 = assign70930_e107358_d_n10;
        locals.var_dnm_dn11 = assign70930_e107358_d_n11;
        locals.var_dnm_dn14 = assign70930_e107358_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign70940_e107368, assign70940_e107368_d_n0, assign70940_e107368_d_n2, assign70940_e107368_d_n4, assign70940_e107368_d_n5, assign70940_e107368_d_n6, assign70940_e107368_d_n7, assign70940_e107368_d_n8, assign70940_e107368_d_n9, assign70940_e107368_d_n10, assign70940_e107368_d_n11, assign70940_e107368_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70940_e107366: f64 = (locals.var_xp * locals.var_x2);
        (assign70940_e107366, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign70940_e107368;
        locals.var_xp_dn0 = assign70940_e107368_d_n0;
        locals.var_xp_dn2 = assign70940_e107368_d_n2;
        locals.var_xp_dn4 = assign70940_e107368_d_n4;
        locals.var_xp_dn5 = assign70940_e107368_d_n5;
        locals.var_xp_dn6 = assign70940_e107368_d_n6;
        locals.var_xp_dn7 = assign70940_e107368_d_n7;
        locals.var_xp_dn8 = assign70940_e107368_d_n8;
        locals.var_xp_dn9 = assign70940_e107368_d_n9;
        locals.var_xp_dn10 = assign70940_e107368_d_n10;
        locals.var_xp_dn11 = assign70940_e107368_d_n11;
        locals.var_xp_dn14 = assign70940_e107368_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign70950_e107378, assign70950_e107378_d_n0, assign70950_e107378_d_n2, assign70950_e107378_d_n4, assign70950_e107378_d_n5, assign70950_e107378_d_n6, assign70950_e107378_d_n7, assign70950_e107378_d_n8, assign70950_e107378_d_n9, assign70950_e107378_d_n10, assign70950_e107378_d_n11, assign70950_e107378_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70950_e107376: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign70950_e107376, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign70950_e107378;
        locals.var_xmp_dn0 = assign70950_e107378_d_n0;
        locals.var_xmp_dn2 = assign70950_e107378_d_n2;
        locals.var_xmp_dn4 = assign70950_e107378_d_n4;
        locals.var_xmp_dn5 = assign70950_e107378_d_n5;
        locals.var_xmp_dn6 = assign70950_e107378_d_n6;
        locals.var_xmp_dn7 = assign70950_e107378_d_n7;
        locals.var_xmp_dn8 = assign70950_e107378_d_n8;
        locals.var_xmp_dn9 = assign70950_e107378_d_n9;
        locals.var_xmp_dn10 = assign70950_e107378_d_n10;
        locals.var_xmp_dn11 = assign70950_e107378_d_n11;
        locals.var_xmp_dn14 = assign70950_e107378_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign70960_e107388, assign70960_e107388_d_n0, assign70960_e107388_d_n2, assign70960_e107388_d_n4, assign70960_e107388_d_n5, assign70960_e107388_d_n6, assign70960_e107388_d_n7, assign70960_e107388_d_n8, assign70960_e107388_d_n9, assign70960_e107388_d_n10, assign70960_e107388_d_n11, assign70960_e107388_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign70960_e107386: f64 = (locals.var_xp + locals.var_xmp);
        (assign70960_e107386, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign70960_e107388;
        locals.var_arg_dn0 = assign70960_e107388_d_n0;
        locals.var_arg_dn2 = assign70960_e107388_d_n2;
        locals.var_arg_dn4 = assign70960_e107388_d_n4;
        locals.var_arg_dn5 = assign70960_e107388_d_n5;
        locals.var_arg_dn6 = assign70960_e107388_d_n6;
        locals.var_arg_dn7 = assign70960_e107388_d_n7;
        locals.var_arg_dn8 = assign70960_e107388_d_n8;
        locals.var_arg_dn9 = assign70960_e107388_d_n9;
        locals.var_arg_dn10 = assign70960_e107388_d_n10;
        locals.var_arg_dn11 = assign70960_e107388_d_n11;
        locals.var_arg_dn14 = assign70960_e107388_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign70970_e107396, assign70970_e107396_d_n0, assign70970_e107396_d_n2, assign70970_e107396_d_n4, assign70970_e107396_d_n5, assign70970_e107396_d_n6, assign70970_e107396_d_n7, assign70970_e107396_d_n8, assign70970_e107396_d_n9, assign70970_e107396_d_n10, assign70970_e107396_d_n11, assign70970_e107396_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign70970_e107396;
        locals.var_dnm_dn0 = assign70970_e107396_d_n0;
        locals.var_dnm_dn2 = assign70970_e107396_d_n2;
        locals.var_dnm_dn4 = assign70970_e107396_d_n4;
        locals.var_dnm_dn5 = assign70970_e107396_d_n5;
        locals.var_dnm_dn6 = assign70970_e107396_d_n6;
        locals.var_dnm_dn7 = assign70970_e107396_d_n7;
        locals.var_dnm_dn8 = assign70970_e107396_d_n8;
        locals.var_dnm_dn9 = assign70970_e107396_d_n9;
        locals.var_dnm_dn10 = assign70970_e107396_d_n10;
        locals.var_dnm_dn11 = assign70970_e107396_d_n11;
        locals.var_dnm_dn14 = assign70970_e107396_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign70980_e107411: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1662 = assign70980_e107411;
        locals.var_guard1662_rv = 0.0;

        let assign70990_e107414: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1663 = assign70990_e107414;
        locals.var_guard1663_rv = 0.0;

        let (assign71000_e107426,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign71000_e107426;
        locals.var_mm_rv = 0.0;

        let assign71010_e107429: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1664 = assign71010_e107429;
        locals.var_guard1664_rv = 0.0;

        let (assign71020_e107444,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) && (locals.var_guard1664 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign71020_e107444;
        locals.var_mm_rv = 0.0;

        let assign71030_e107447: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1665 = assign71030_e107447;
        locals.var_guard1665_rv = 0.0;

        let (assign71040_e107465,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) && (locals.var_guard1664 == 0.0)) && (locals.var_guard1665 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign71040_e107465;
        locals.var_mm_rv = 0.0;

        let assign71050_e107468: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1666 = assign71050_e107468;
        locals.var_guard1666_rv = 0.0;

        let (assign71060_e107489,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 != 0.0)) && (locals.var_guard1663 == 0.0)) && (locals.var_guard1664 == 0.0)) && (locals.var_guard1665 == 0.0)) && (locals.var_guard1666 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign71060_e107489;
        locals.var_mm_rv = 0.0;

        let (assign71070_e107499,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign71070_e107499;
        locals.var_m0_rv = 0.0;

        let mut assign71080_loop_guard: usize = 0;
        while {
            let assign71080_cond_e107510: f64 = if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign71080_cond_e107510 != 0.0
        } {
            assign71080_loop_guard += 1;
            assert!(assign71080_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign71080_body0_e107521, assign71080_body0_e107521_d_n0, assign71080_body0_e107521_d_n2, assign71080_body0_e107521_d_n4, assign71080_body0_e107521_d_n5, assign71080_body0_e107521_d_n6, assign71080_body0_e107521_d_n7, assign71080_body0_e107521_d_n8, assign71080_body0_e107521_d_n9, assign71080_body0_e107521_d_n10, assign71080_body0_e107521_d_n11, assign71080_body0_e107521_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 != 0.0)) {
        let assign71080_body0_e107519: f64 = (locals.var_dnm).sqrt();
        (assign71080_body0_e107519, (locals.var_dnm_dn0 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn2 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn4 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn5 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn6 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn7 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn8 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn9 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn10 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn11 / (2.0 * assign71080_body0_e107519)), (locals.var_dnm_dn14 / (2.0 * assign71080_body0_e107519)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign71080_body0_e107521;
            locals.var_dnm_dn0 = assign71080_body0_e107521_d_n0;
            locals.var_dnm_dn2 = assign71080_body0_e107521_d_n2;
            locals.var_dnm_dn4 = assign71080_body0_e107521_d_n4;
            locals.var_dnm_dn5 = assign71080_body0_e107521_d_n5;
            locals.var_dnm_dn6 = assign71080_body0_e107521_d_n6;
            locals.var_dnm_dn7 = assign71080_body0_e107521_d_n7;
            locals.var_dnm_dn8 = assign71080_body0_e107521_d_n8;
            locals.var_dnm_dn9 = assign71080_body0_e107521_d_n9;
            locals.var_dnm_dn10 = assign71080_body0_e107521_d_n10;
            locals.var_dnm_dn11 = assign71080_body0_e107521_d_n11;
            locals.var_dnm_dn14 = assign71080_body0_e107521_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign71080_body1_e107533,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 != 0.0)) {
        let assign71080_body1_e107531: f64 = (locals.var_m0 + 1.0);
        (assign71080_body1_e107531,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign71080_body1_e107533;
            locals.var_m0_rv = 0.0;
        }

        let (assign71090_e107555, assign71090_e107555_d_n0, assign71090_e107555_d_n2, assign71090_e107555_d_n4, assign71090_e107555_d_n5, assign71090_e107555_d_n6, assign71090_e107555_d_n7, assign71090_e107555_d_n8, assign71090_e107555_d_n9, assign71090_e107555_d_n10, assign71090_e107555_d_n11, assign71090_e107555_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) && (locals.var_guard1662 == 0.0)) {
        let (assign71090_e107553, assign71090_e107553_d_n0, assign71090_e107553_d_n2, assign71090_e107553_d_n4, assign71090_e107553_d_n5, assign71090_e107553_d_n6, assign71090_e107553_d_n7, assign71090_e107553_d_n8, assign71090_e107553_d_n9, assign71090_e107553_d_n10, assign71090_e107553_d_n11, assign71090_e107553_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign71090_e107550: f64 = 2.0;
                let assign71090_e107551: f64 = (1.0 / assign71090_e107550);
                let assign71090_e107552: f64 = (locals.var_dnm).powf(assign71090_e107551);
                (assign71090_e107552, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn0)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn2)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn4)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn5)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn6)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn7)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn8)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn9)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn10)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn11)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign71090_e107551) as f64).is_finite() && ((assign71090_e107551) as f64).fract() == 0.0 { if assign71090_e107551 == 0.0 { 0.0 } else { (assign71090_e107551 * ((locals.var_dnm).powf(assign71090_e107551 - 1.0) * locals.var_dnm_dn14)) } } else { (assign71090_e107552 * (assign71090_e107551 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign71090_e107553, assign71090_e107553_d_n0, assign71090_e107553_d_n2, assign71090_e107553_d_n4, assign71090_e107553_d_n5, assign71090_e107553_d_n6, assign71090_e107553_d_n7, assign71090_e107553_d_n8, assign71090_e107553_d_n9, assign71090_e107553_d_n10, assign71090_e107553_d_n11, assign71090_e107553_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign71090_e107555;
        locals.var_dnm_dn0 = assign71090_e107555_d_n0;
        locals.var_dnm_dn2 = assign71090_e107555_d_n2;
        locals.var_dnm_dn4 = assign71090_e107555_d_n4;
        locals.var_dnm_dn5 = assign71090_e107555_d_n5;
        locals.var_dnm_dn6 = assign71090_e107555_d_n6;
        locals.var_dnm_dn7 = assign71090_e107555_d_n7;
        locals.var_dnm_dn8 = assign71090_e107555_d_n8;
        locals.var_dnm_dn9 = assign71090_e107555_d_n9;
        locals.var_dnm_dn10 = assign71090_e107555_d_n10;
        locals.var_dnm_dn11 = assign71090_e107555_d_n11;
        locals.var_dnm_dn14 = assign71090_e107555_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign71100_e107565, assign71100_e107565_d_n0, assign71100_e107565_d_n2, assign71100_e107565_d_n4, assign71100_e107565_d_n5, assign71100_e107565_d_n6, assign71100_e107565_d_n7, assign71100_e107565_d_n8, assign71100_e107565_d_n9, assign71100_e107565_d_n10, assign71100_e107565_d_n11, assign71100_e107565_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign71100_e107563: f64 = (1.0 / locals.var_dnm);
        (assign71100_e107563, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign71100_e107565;
        locals.var_dnm_dn0 = assign71100_e107565_d_n0;
        locals.var_dnm_dn2 = assign71100_e107565_d_n2;
        locals.var_dnm_dn4 = assign71100_e107565_d_n4;
        locals.var_dnm_dn5 = assign71100_e107565_d_n5;
        locals.var_dnm_dn6 = assign71100_e107565_d_n6;
        locals.var_dnm_dn7 = assign71100_e107565_d_n7;
        locals.var_dnm_dn8 = assign71100_e107565_d_n8;
        locals.var_dnm_dn9 = assign71100_e107565_d_n9;
        locals.var_dnm_dn10 = assign71100_e107565_d_n10;
        locals.var_dnm_dn11 = assign71100_e107565_d_n11;
        locals.var_dnm_dn14 = assign71100_e107565_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign71110_e107577, assign71110_e107577_d_n0, assign71110_e107577_d_n2, assign71110_e107577_d_n4, assign71110_e107577_d_n5, assign71110_e107577_d_n6, assign71110_e107577_d_n7, assign71110_e107577_d_n8, assign71110_e107577_d_n9, assign71110_e107577_d_n10, assign71110_e107577_d_n11, assign71110_e107577_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign71110_e107573: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign71110_e107575: f64 = (assign71110_e107573 * locals.var_dnm);
        (assign71110_e107575, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn11)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn14)) * locals.var_dnm) + (assign71110_e107573 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign71110_e107577;
        locals.var_tmf0_dn0 = assign71110_e107577_d_n0;
        locals.var_tmf0_dn2 = assign71110_e107577_d_n2;
        locals.var_tmf0_dn4 = assign71110_e107577_d_n4;
        locals.var_tmf0_dn5 = assign71110_e107577_d_n5;
        locals.var_tmf0_dn6 = assign71110_e107577_d_n6;
        locals.var_tmf0_dn7 = assign71110_e107577_d_n7;
        locals.var_tmf0_dn8 = assign71110_e107577_d_n8;
        locals.var_tmf0_dn9 = assign71110_e107577_d_n9;
        locals.var_tmf0_dn10 = assign71110_e107577_d_n10;
        locals.var_tmf0_dn11 = assign71110_e107577_d_n11;
        locals.var_tmf0_dn14 = assign71110_e107577_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign71120_e107591, assign71120_e107591_d_n0, assign71120_e107591_d_n2, assign71120_e107591_d_n4, assign71120_e107591_d_n5, assign71120_e107591_d_n6, assign71120_e107591_d_n7, assign71120_e107591_d_n8, assign71120_e107591_d_n9, assign71120_e107591_d_n10, assign71120_e107591_d_n11, assign71120_e107591_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign71120_e107585: f64 = (locals.var_t1 * locals.var_xmp);
        let assign71120_e107587: f64 = (assign71120_e107585 * locals.var_dnm);
        let assign71120_e107589: f64 = (assign71120_e107587 / locals.var_arg);
        (assign71120_e107589, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn0)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn2)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn4)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn5)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn6)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn7)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn8)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn9)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn10)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn11 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn11)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn14 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign71120_e107585 * locals.var_dnm_dn14)) * locals.var_arg) - (assign71120_e107587 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign71120_e107591;
        locals.var_t0_dn0 = assign71120_e107591_d_n0;
        locals.var_t0_dn2 = assign71120_e107591_d_n2;
        locals.var_t0_dn4 = assign71120_e107591_d_n4;
        locals.var_t0_dn5 = assign71120_e107591_d_n5;
        locals.var_t0_dn6 = assign71120_e107591_d_n6;
        locals.var_t0_dn7 = assign71120_e107591_d_n7;
        locals.var_t0_dn8 = assign71120_e107591_d_n8;
        locals.var_t0_dn9 = assign71120_e107591_d_n9;
        locals.var_t0_dn10 = assign71120_e107591_d_n10;
        locals.var_t0_dn11 = assign71120_e107591_d_n11;
        locals.var_t0_dn14 = assign71120_e107591_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign71130_e107603, assign71130_e107603_d_n0, assign71130_e107603_d_n2, assign71130_e107603_d_n4, assign71130_e107603_d_n5, assign71130_e107603_d_n6, assign71130_e107603_d_n7, assign71130_e107603_d_n8, assign71130_e107603_d_n9, assign71130_e107603_d_n10, assign71130_e107603_d_n11, assign71130_e107603_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        let assign71130_e107599: f64 = (-locals.var_t1);
        let assign71130_e107601: f64 = (assign71130_e107599 + locals.var_tmf0);
        (assign71130_e107601, ((-locals.var_t1_dn0) + locals.var_tmf0_dn0), ((-locals.var_t1_dn2) + locals.var_tmf0_dn2), ((-locals.var_t1_dn4) + locals.var_tmf0_dn4), ((-locals.var_t1_dn5) + locals.var_tmf0_dn5), ((-locals.var_t1_dn6) + locals.var_tmf0_dn6), ((-locals.var_t1_dn7) + locals.var_tmf0_dn7), ((-locals.var_t1_dn8) + locals.var_tmf0_dn8), ((-locals.var_t1_dn9) + locals.var_tmf0_dn9), ((-locals.var_t1_dn10) + locals.var_tmf0_dn10), ((-locals.var_t1_dn11) + locals.var_tmf0_dn11), ((-locals.var_t1_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign71130_e107603;
        locals.var_t1_dn0 = assign71130_e107603_d_n0;
        locals.var_t1_dn2 = assign71130_e107603_d_n2;
        locals.var_t1_dn4 = assign71130_e107603_d_n4;
        locals.var_t1_dn5 = assign71130_e107603_d_n5;
        locals.var_t1_dn6 = assign71130_e107603_d_n6;
        locals.var_t1_dn7 = assign71130_e107603_d_n7;
        locals.var_t1_dn8 = assign71130_e107603_d_n8;
        locals.var_t1_dn9 = assign71130_e107603_d_n9;
        locals.var_t1_dn10 = assign71130_e107603_d_n10;
        locals.var_t1_dn11 = assign71130_e107603_d_n11;
        locals.var_t1_dn14 = assign71130_e107603_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign71140_e107611, assign71140_e107611_d_n0, assign71140_e107611_d_n2, assign71140_e107611_d_n4, assign71140_e107611_d_n5, assign71140_e107611_d_n6, assign71140_e107611_d_n7, assign71140_e107611_d_n8, assign71140_e107611_d_n9, assign71140_e107611_d_n10, assign71140_e107611_d_n11, assign71140_e107611_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign71140_e107611;
        locals.var_t0_dn0 = assign71140_e107611_d_n0;
        locals.var_t0_dn2 = assign71140_e107611_d_n2;
        locals.var_t0_dn4 = assign71140_e107611_d_n4;
        locals.var_t0_dn5 = assign71140_e107611_d_n5;
        locals.var_t0_dn6 = assign71140_e107611_d_n6;
        locals.var_t0_dn7 = assign71140_e107611_d_n7;
        locals.var_t0_dn8 = assign71140_e107611_d_n8;
        locals.var_t0_dn9 = assign71140_e107611_d_n9;
        locals.var_t0_dn10 = assign71140_e107611_d_n10;
        locals.var_t0_dn11 = assign71140_e107611_d_n11;
        locals.var_t0_dn14 = assign71140_e107611_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign71150_e107622, assign71150_e107622_d_n0, assign71150_e107622_d_n2, assign71150_e107622_d_n4, assign71150_e107622_d_n5, assign71150_e107622_d_n6, assign71150_e107622_d_n7, assign71150_e107622_d_n8, assign71150_e107622_d_n9, assign71150_e107622_d_n10, assign71150_e107622_d_n11, assign71150_e107622_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 == 0.0)) {
        let assign71150_e107620: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        (assign71150_e107620, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign71150_e107622;
        locals.var_t1_dn0 = assign71150_e107622_d_n0;
        locals.var_t1_dn2 = assign71150_e107622_d_n2;
        locals.var_t1_dn4 = assign71150_e107622_d_n4;
        locals.var_t1_dn5 = assign71150_e107622_d_n5;
        locals.var_t1_dn6 = assign71150_e107622_d_n6;
        locals.var_t1_dn7 = assign71150_e107622_d_n7;
        locals.var_t1_dn8 = assign71150_e107622_d_n8;
        locals.var_t1_dn9 = assign71150_e107622_d_n9;
        locals.var_t1_dn10 = assign71150_e107622_d_n10;
        locals.var_t1_dn11 = assign71150_e107622_d_n11;
        locals.var_t1_dn14 = assign71150_e107622_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_267(
        locals: &mut StampLocals,
    ) {
        let (assign71160_e107631, assign71160_e107631_d_n0, assign71160_e107631_d_n2, assign71160_e107631_d_n4, assign71160_e107631_d_n5, assign71160_e107631_d_n6, assign71160_e107631_d_n7, assign71160_e107631_d_n8, assign71160_e107631_d_n9, assign71160_e107631_d_n10, assign71160_e107631_d_n11, assign71160_e107631_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1661 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign71160_e107631;
        locals.var_t0_dn0 = assign71160_e107631_d_n0;
        locals.var_t0_dn2 = assign71160_e107631_d_n2;
        locals.var_t0_dn4 = assign71160_e107631_d_n4;
        locals.var_t0_dn5 = assign71160_e107631_d_n5;
        locals.var_t0_dn6 = assign71160_e107631_d_n6;
        locals.var_t0_dn7 = assign71160_e107631_d_n7;
        locals.var_t0_dn8 = assign71160_e107631_d_n8;
        locals.var_t0_dn9 = assign71160_e107631_d_n9;
        locals.var_t0_dn10 = assign71160_e107631_d_n10;
        locals.var_t0_dn11 = assign71160_e107631_d_n11;
        locals.var_t0_dn14 = assign71160_e107631_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign71170_e107639, assign71170_e107639_d_n0, assign71170_e107639_d_n2, assign71170_e107639_d_n4, assign71170_e107639_d_n5, assign71170_e107639_d_n6, assign71170_e107639_d_n7, assign71170_e107639_d_n8, assign71170_e107639_d_n9, assign71170_e107639_d_n10, assign71170_e107639_d_n11, assign71170_e107639_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign71170_e107637: f64 = (locals.var_t1 - locals.var_vgpld);
        (assign71170_e107637, locals.var_t1_dn0, (locals.var_t1_dn2 - locals.var_vgpld_dn2), locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, (locals.var_t1_dn7 - locals.var_vgpld_dn7), (locals.var_t1_dn8 - locals.var_vgpld_dn8), (locals.var_t1_dn9 - locals.var_vgpld_dn9), locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign71170_e107639;
        locals.var_vxbgmtcl_dn0 = assign71170_e107639_d_n0;
        locals.var_vxbgmtcl_dn2 = assign71170_e107639_d_n2;
        locals.var_vxbgmtcl_dn4 = assign71170_e107639_d_n4;
        locals.var_vxbgmtcl_dn5 = assign71170_e107639_d_n5;
        locals.var_vxbgmtcl_dn6 = assign71170_e107639_d_n6;
        locals.var_vxbgmtcl_dn7 = assign71170_e107639_d_n7;
        locals.var_vxbgmtcl_dn8 = assign71170_e107639_d_n8;
        locals.var_vxbgmtcl_dn9 = assign71170_e107639_d_n9;
        locals.var_vxbgmtcl_dn10 = assign71170_e107639_d_n10;
        locals.var_vxbgmtcl_dn11 = assign71170_e107639_d_n11;
        locals.var_vxbgmtcl_dn14 = assign71170_e107639_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign71180_e107650, assign71180_e107650_d_n0, assign71180_e107650_d_n2, assign71180_e107650_d_n4, assign71180_e107650_d_n5, assign71180_e107650_d_n6, assign71180_e107650_d_n7, assign71180_e107650_d_n8, assign71180_e107650_d_n9, assign71180_e107650_d_n10, assign71180_e107650_d_n11, assign71180_e107650_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign71180_e107644: f64 = (-locals.var_vxbgmtcl);
        let assign71180_e107647: f64 = (10.0 * 2.220446049250313e-16);
        let assign71180_e107648: f64 = (assign71180_e107644 + assign71180_e107647);
        (assign71180_e107648, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn14,)
    }
};
        locals.var_vgb_fb_ld = assign71180_e107650;
        locals.var_vgb_fb_ld_dn0 = assign71180_e107650_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign71180_e107650_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign71180_e107650_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign71180_e107650_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign71180_e107650_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign71180_e107650_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign71180_e107650_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign71180_e107650_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign71180_e107650_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign71180_e107650_d_n11;
        locals.var_vgb_fb_ld_dn14 = assign71180_e107650_d_n14;
        locals.var_vgb_fb_ld_rv = 0.0;

        let assign71190_e107653: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard1667 = assign71190_e107653;
        locals.var_guard1667_rv = 0.0;

        let (assign71210_e107674, assign71210_e107674_d_n0, assign71210_e107674_d_n2, assign71210_e107674_d_n4, assign71210_e107674_d_n5, assign71210_e107674_d_n6, assign71210_e107674_d_n7, assign71210_e107674_d_n8, assign71210_e107674_d_n9, assign71210_e107674_d_n10, assign71210_e107674_d_n11, assign71210_e107674_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71210_e107666: f64 = (2.0 * locals.var_beta_inv);
        let assign71210_e107668: f64 = (-locals.var_vgs_min);
        let assign71210_e107670: f64 = (assign71210_e107668 / locals.var_fac1);
        let assign71210_e107671: f64 = (assign71210_e107670).ln();
        let assign71210_e107672: f64 = (assign71210_e107666 * assign71210_e107671);
        (assign71210_e107672, (((2.0 * locals.var_beta_inv_dn0) * assign71210_e107671) + (assign71210_e107666 * ((-((assign71210_e107668 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign71210_e107670))), (((2.0 * locals.var_beta_inv_dn2) * assign71210_e107671) + (assign71210_e107666 * ((-((assign71210_e107668 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign71210_e107670))), (((2.0 * locals.var_beta_inv_dn4) * assign71210_e107671) + (assign71210_e107666 * ((-((assign71210_e107668 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign71210_e107670))), (((2.0 * locals.var_beta_inv_dn5) * assign71210_e107671) + (assign71210_e107666 * ((-((assign71210_e107668 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign71210_e107670))), (((2.0 * locals.var_beta_inv_dn6) * assign71210_e107671) + (assign71210_e107666 * ((-((assign71210_e107668 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign71210_e107670))), (((2.0 * locals.var_beta_inv_dn7) * assign71210_e107671) + (assign71210_e107666 * ((-((assign71210_e107668 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign71210_e107670))), (((2.0 * locals.var_beta_inv_dn8) * assign71210_e107671) + (assign71210_e107666 * ((-((assign71210_e107668 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign71210_e107670))), (((2.0 * locals.var_beta_inv_dn9) * assign71210_e107671) + (assign71210_e107666 * ((-((assign71210_e107668 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign71210_e107670))), (((2.0 * locals.var_beta_inv_dn10) * assign71210_e107671) + (assign71210_e107666 * ((-((assign71210_e107668 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign71210_e107670))), (((2.0 * locals.var_beta_inv_dn11) * assign71210_e107671) + (assign71210_e107666 * ((-((assign71210_e107668 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign71210_e107670))), (((2.0 * locals.var_beta_inv_dn14) * assign71210_e107671) + (assign71210_e107666 * ((-((assign71210_e107668 * locals.var_fac1_dn14) / (locals.var_fac1 * locals.var_fac1))) / assign71210_e107670))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn14,)
    }
};
        locals.var_ps0_min = assign71210_e107674;
        locals.var_ps0_min_dn0 = assign71210_e107674_d_n0;
        locals.var_ps0_min_dn2 = assign71210_e107674_d_n2;
        locals.var_ps0_min_dn4 = assign71210_e107674_d_n4;
        locals.var_ps0_min_dn5 = assign71210_e107674_d_n5;
        locals.var_ps0_min_dn6 = assign71210_e107674_d_n6;
        locals.var_ps0_min_dn7 = assign71210_e107674_d_n7;
        locals.var_ps0_min_dn8 = assign71210_e107674_d_n8;
        locals.var_ps0_min_dn9 = assign71210_e107674_d_n9;
        locals.var_ps0_min_dn10 = assign71210_e107674_d_n10;
        locals.var_ps0_min_dn11 = assign71210_e107674_d_n11;
        locals.var_ps0_min_dn14 = assign71210_e107674_d_n14;
        locals.var_ps0_min_rv = 0.0;

        let (assign71220_e107684, assign71220_e107684_d_n0, assign71220_e107684_d_n2, assign71220_e107684_d_n4, assign71220_e107684_d_n5, assign71220_e107684_d_n6, assign71220_e107684_d_n7, assign71220_e107684_d_n8, assign71220_e107684_d_n9, assign71220_e107684_d_n10, assign71220_e107684_d_n11, assign71220_e107684_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71220_e107681: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71220_e107682: f64 = (locals.var_beta * assign71220_e107681);
        (assign71220_e107682, ((locals.var_beta_dn0 * assign71220_e107681) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((locals.var_beta_dn2 * assign71220_e107681) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign71220_e107681) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), ((locals.var_beta_dn5 * assign71220_e107681) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((locals.var_beta_dn6 * assign71220_e107681) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((locals.var_beta_dn7 * assign71220_e107681) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign71220_e107681) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign71220_e107681) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign71220_e107681) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((locals.var_beta_dn11 * assign71220_e107681) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((locals.var_beta_dn14 * assign71220_e107681) + (locals.var_beta * locals.var_vxbgmtcl_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign71220_e107684;
        locals.var_tx_dn0 = assign71220_e107684_d_n0;
        locals.var_tx_dn2 = assign71220_e107684_d_n2;
        locals.var_tx_dn4 = assign71220_e107684_d_n4;
        locals.var_tx_dn5 = assign71220_e107684_d_n5;
        locals.var_tx_dn6 = assign71220_e107684_d_n6;
        locals.var_tx_dn7 = assign71220_e107684_d_n7;
        locals.var_tx_dn8 = assign71220_e107684_d_n8;
        locals.var_tx_dn9 = assign71220_e107684_d_n9;
        locals.var_tx_dn10 = assign71220_e107684_d_n10;
        locals.var_tx_dn11 = assign71220_e107684_d_n11;
        locals.var_tx_dn14 = assign71220_e107684_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign71230_e107694, assign71230_e107694_d_n0, assign71230_e107694_d_n2, assign71230_e107694_d_n4, assign71230_e107694_d_n5, assign71230_e107694_d_n6, assign71230_e107694_d_n7, assign71230_e107694_d_n8, assign71230_e107694_d_n9, assign71230_e107694_d_n10, assign71230_e107694_d_n11, assign71230_e107694_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71230_e107691: f64 = (locals.var_beta * locals.var_cnst0over_func);
        let assign71230_e107692: f64 = (1.0 / assign71230_e107691);
        (assign71230_e107692, (-(((locals.var_beta_dn0 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn0)) / (assign71230_e107691 * assign71230_e107691))), (-(((locals.var_beta_dn2 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn2)) / (assign71230_e107691 * assign71230_e107691))), (-(((locals.var_beta_dn4 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn4)) / (assign71230_e107691 * assign71230_e107691))), (-(((locals.var_beta_dn5 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn5)) / (assign71230_e107691 * assign71230_e107691))), (-(((locals.var_beta_dn6 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn6)) / (assign71230_e107691 * assign71230_e107691))), (-(((locals.var_beta_dn7 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn7)) / (assign71230_e107691 * assign71230_e107691))), (-(((locals.var_beta_dn8 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn8)) / (assign71230_e107691 * assign71230_e107691))), (-(((locals.var_beta_dn9 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn9)) / (assign71230_e107691 * assign71230_e107691))), (-(((locals.var_beta_dn10 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn10)) / (assign71230_e107691 * assign71230_e107691))), (-(((locals.var_beta_dn11 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn11)) / (assign71230_e107691 * assign71230_e107691))), (-(((locals.var_beta_dn14 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn14)) / (assign71230_e107691 * assign71230_e107691))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign71230_e107694;
        locals.var_t1_dn0 = assign71230_e107694_d_n0;
        locals.var_t1_dn2 = assign71230_e107694_d_n2;
        locals.var_t1_dn4 = assign71230_e107694_d_n4;
        locals.var_t1_dn5 = assign71230_e107694_d_n5;
        locals.var_t1_dn6 = assign71230_e107694_d_n6;
        locals.var_t1_dn7 = assign71230_e107694_d_n7;
        locals.var_t1_dn8 = assign71230_e107694_d_n8;
        locals.var_t1_dn9 = assign71230_e107694_d_n9;
        locals.var_t1_dn10 = assign71230_e107694_d_n10;
        locals.var_t1_dn11 = assign71230_e107694_d_n11;
        locals.var_t1_dn14 = assign71230_e107694_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign71240_e107702, assign71240_e107702_d_n0, assign71240_e107702_d_n2, assign71240_e107702_d_n4, assign71240_e107702_d_n5, assign71240_e107702_d_n6, assign71240_e107702_d_n7, assign71240_e107702_d_n8, assign71240_e107702_d_n9, assign71240_e107702_d_n10, assign71240_e107702_d_n11, assign71240_e107702_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71240_e107700: f64 = (locals.var_t1 * locals.var_cox0_func);
        (assign71240_e107700, (locals.var_t1_dn0 * locals.var_cox0_func), (locals.var_t1_dn2 * locals.var_cox0_func), (locals.var_t1_dn4 * locals.var_cox0_func), (locals.var_t1_dn5 * locals.var_cox0_func), (locals.var_t1_dn6 * locals.var_cox0_func), (locals.var_t1_dn7 * locals.var_cox0_func), (locals.var_t1_dn8 * locals.var_cox0_func), (locals.var_t1_dn9 * locals.var_cox0_func), (locals.var_t1_dn10 * locals.var_cox0_func), (locals.var_t1_dn11 * locals.var_cox0_func), (locals.var_t1_dn14 * locals.var_cox0_func),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign71240_e107702;
        locals.var_ty_dn0 = assign71240_e107702_d_n0;
        locals.var_ty_dn2 = assign71240_e107702_d_n2;
        locals.var_ty_dn4 = assign71240_e107702_d_n4;
        locals.var_ty_dn5 = assign71240_e107702_d_n5;
        locals.var_ty_dn6 = assign71240_e107702_d_n6;
        locals.var_ty_dn7 = assign71240_e107702_d_n7;
        locals.var_ty_dn8 = assign71240_e107702_d_n8;
        locals.var_ty_dn9 = assign71240_e107702_d_n9;
        locals.var_ty_dn10 = assign71240_e107702_d_n10;
        locals.var_ty_dn11 = assign71240_e107702_d_n11;
        locals.var_ty_dn14 = assign71240_e107702_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign71250_e107714, assign71250_e107714_d_n0, assign71250_e107714_d_n2, assign71250_e107714_d_n4, assign71250_e107714_d_n5, assign71250_e107714_d_n6, assign71250_e107714_d_n7, assign71250_e107714_d_n8, assign71250_e107714_d_n9, assign71250_e107714_d_n10, assign71250_e107714_d_n11, assign71250_e107714_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71250_e107709: f64 = (3.0 * 1.414213562373095);
        let assign71250_e107711: f64 = (assign71250_e107709 * locals.var_ty);
        let assign71250_e107712: f64 = (2.0 + assign71250_e107711);
        (assign71250_e107712, (assign71250_e107709 * locals.var_ty_dn0), (assign71250_e107709 * locals.var_ty_dn2), (assign71250_e107709 * locals.var_ty_dn4), (assign71250_e107709 * locals.var_ty_dn5), (assign71250_e107709 * locals.var_ty_dn6), (assign71250_e107709 * locals.var_ty_dn7), (assign71250_e107709 * locals.var_ty_dn8), (assign71250_e107709 * locals.var_ty_dn9), (assign71250_e107709 * locals.var_ty_dn10), (assign71250_e107709 * locals.var_ty_dn11), (assign71250_e107709 * locals.var_ty_dn14),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn14,)
    }
};
        locals.var_ac41 = assign71250_e107714;
        locals.var_ac41_dn0 = assign71250_e107714_d_n0;
        locals.var_ac41_dn2 = assign71250_e107714_d_n2;
        locals.var_ac41_dn4 = assign71250_e107714_d_n4;
        locals.var_ac41_dn5 = assign71250_e107714_d_n5;
        locals.var_ac41_dn6 = assign71250_e107714_d_n6;
        locals.var_ac41_dn7 = assign71250_e107714_d_n7;
        locals.var_ac41_dn8 = assign71250_e107714_d_n8;
        locals.var_ac41_dn9 = assign71250_e107714_d_n9;
        locals.var_ac41_dn10 = assign71250_e107714_d_n10;
        locals.var_ac41_dn11 = assign71250_e107714_d_n11;
        locals.var_ac41_dn14 = assign71250_e107714_d_n14;
        locals.var_ac41_rv = 0.0;

        let (assign71260_e107726, assign71260_e107726_d_n0, assign71260_e107726_d_n2, assign71260_e107726_d_n4, assign71260_e107726_d_n5, assign71260_e107726_d_n6, assign71260_e107726_d_n7, assign71260_e107726_d_n8, assign71260_e107726_d_n9, assign71260_e107726_d_n10, assign71260_e107726_d_n11, assign71260_e107726_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71260_e107720: f64 = (8.0 * locals.var_ac41);
        let assign71260_e107722: f64 = (assign71260_e107720 * locals.var_ac41);
        let assign71260_e107724: f64 = (assign71260_e107722 * locals.var_ac41);
        (assign71260_e107724, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign71260_e107720 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign71260_e107722 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign71260_e107720 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign71260_e107722 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign71260_e107720 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign71260_e107722 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign71260_e107720 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign71260_e107722 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign71260_e107720 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign71260_e107722 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign71260_e107720 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign71260_e107722 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign71260_e107720 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign71260_e107722 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign71260_e107720 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign71260_e107722 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign71260_e107720 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign71260_e107722 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign71260_e107720 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign71260_e107722 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn14) * locals.var_ac41) + (assign71260_e107720 * locals.var_ac41_dn14)) * locals.var_ac41) + (assign71260_e107722 * locals.var_ac41_dn14)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn14,)
    }
};
        locals.var_ac4 = assign71260_e107726;
        locals.var_ac4_dn0 = assign71260_e107726_d_n0;
        locals.var_ac4_dn2 = assign71260_e107726_d_n2;
        locals.var_ac4_dn4 = assign71260_e107726_d_n4;
        locals.var_ac4_dn5 = assign71260_e107726_d_n5;
        locals.var_ac4_dn6 = assign71260_e107726_d_n6;
        locals.var_ac4_dn7 = assign71260_e107726_d_n7;
        locals.var_ac4_dn8 = assign71260_e107726_d_n8;
        locals.var_ac4_dn9 = assign71260_e107726_d_n9;
        locals.var_ac4_dn10 = assign71260_e107726_d_n10;
        locals.var_ac4_dn11 = assign71260_e107726_d_n11;
        locals.var_ac4_dn14 = assign71260_e107726_d_n14;
        locals.var_ac4_rv = 0.0;

        let (assign71270_e107742, assign71270_e107742_d_n0, assign71270_e107742_d_n2, assign71270_e107742_d_n4, assign71270_e107742_d_n5, assign71270_e107742_d_n6, assign71270_e107742_d_n7, assign71270_e107742_d_n8, assign71270_e107742_d_n9, assign71270_e107742_d_n10, assign71270_e107742_d_n11, assign71270_e107742_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71270_e107732: f64 = (7.0 * 1.414213562373095);
        let assign71270_e107735: f64 = (9.0 * locals.var_ty);
        let assign71270_e107738: f64 = (locals.var_tx - 2.0);
        let assign71270_e107739: f64 = (assign71270_e107735 * assign71270_e107738);
        let assign71270_e107740: f64 = (assign71270_e107732 - assign71270_e107739);
        (assign71270_e107740, (-(((9.0 * locals.var_ty_dn0) * assign71270_e107738) + (assign71270_e107735 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign71270_e107738) + (assign71270_e107735 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign71270_e107738) + (assign71270_e107735 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign71270_e107738) + (assign71270_e107735 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign71270_e107738) + (assign71270_e107735 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn7) * assign71270_e107738) + (assign71270_e107735 * locals.var_tx_dn7))), (-(((9.0 * locals.var_ty_dn8) * assign71270_e107738) + (assign71270_e107735 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn9) * assign71270_e107738) + (assign71270_e107735 * locals.var_tx_dn9))), (-(((9.0 * locals.var_ty_dn10) * assign71270_e107738) + (assign71270_e107735 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn11) * assign71270_e107738) + (assign71270_e107735 * locals.var_tx_dn11))), (-(((9.0 * locals.var_ty_dn14) * assign71270_e107738) + (assign71270_e107735 * locals.var_tx_dn14))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn14,)
    }
};
        locals.var_ac31 = assign71270_e107742;
        locals.var_ac31_dn0 = assign71270_e107742_d_n0;
        locals.var_ac31_dn2 = assign71270_e107742_d_n2;
        locals.var_ac31_dn4 = assign71270_e107742_d_n4;
        locals.var_ac31_dn5 = assign71270_e107742_d_n5;
        locals.var_ac31_dn6 = assign71270_e107742_d_n6;
        locals.var_ac31_dn7 = assign71270_e107742_d_n7;
        locals.var_ac31_dn8 = assign71270_e107742_d_n8;
        locals.var_ac31_dn9 = assign71270_e107742_d_n9;
        locals.var_ac31_dn10 = assign71270_e107742_d_n10;
        locals.var_ac31_dn11 = assign71270_e107742_d_n11;
        locals.var_ac31_dn14 = assign71270_e107742_d_n14;
        locals.var_ac31_rv = 0.0;

        let (assign71280_e107750, assign71280_e107750_d_n0, assign71280_e107750_d_n2, assign71280_e107750_d_n4, assign71280_e107750_d_n5, assign71280_e107750_d_n6, assign71280_e107750_d_n7, assign71280_e107750_d_n8, assign71280_e107750_d_n9, assign71280_e107750_d_n10, assign71280_e107750_d_n11, assign71280_e107750_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71280_e107748: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign71280_e107748, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn14 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn14)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn14,)
    }
};
        locals.var_ac3 = assign71280_e107750;
        locals.var_ac3_dn0 = assign71280_e107750_d_n0;
        locals.var_ac3_dn2 = assign71280_e107750_d_n2;
        locals.var_ac3_dn4 = assign71280_e107750_d_n4;
        locals.var_ac3_dn5 = assign71280_e107750_d_n5;
        locals.var_ac3_dn6 = assign71280_e107750_d_n6;
        locals.var_ac3_dn7 = assign71280_e107750_d_n7;
        locals.var_ac3_dn8 = assign71280_e107750_d_n8;
        locals.var_ac3_dn9 = assign71280_e107750_d_n9;
        locals.var_ac3_dn10 = assign71280_e107750_d_n10;
        locals.var_ac3_dn11 = assign71280_e107750_d_n11;
        locals.var_ac3_dn14 = assign71280_e107750_d_n14;
        locals.var_ac3_rv = 0.0;

        let assign71290_e107754: f64 = (locals.var_ac3 * 1e-8);
        let assign71290_e107755: f64 = if locals.var_ac4 < assign71290_e107754 { 1.0 } else { 0.0 };
        locals.var_guard1668 = assign71290_e107755;
        locals.var_guard1668_rv = 0.0;

        let (assign71310_e107776, assign71310_e107776_d_n0, assign71310_e107776_d_n2, assign71310_e107776_d_n4, assign71310_e107776_d_n5, assign71310_e107776_d_n6, assign71310_e107776_d_n7, assign71310_e107776_d_n8, assign71310_e107776_d_n9, assign71310_e107776_d_n10, assign71310_e107776_d_n11, assign71310_e107776_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) && (locals.var_guard1668 != 0.0)) {
        let assign71310_e107772: f64 = (0.5 * locals.var_ac4);
        let assign71310_e107774: f64 = (assign71310_e107772 / locals.var_ac31);
        (assign71310_e107774, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign71310_e107772 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign71310_e107772 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign71310_e107772 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign71310_e107772 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign71310_e107772 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign71310_e107772 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign71310_e107772 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign71310_e107772 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign71310_e107772 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign71310_e107772 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn14) * locals.var_ac31) - (assign71310_e107772 * locals.var_ac31_dn14)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign71310_e107776;
        locals.var_ac1_dn0 = assign71310_e107776_d_n0;
        locals.var_ac1_dn2 = assign71310_e107776_d_n2;
        locals.var_ac1_dn4 = assign71310_e107776_d_n4;
        locals.var_ac1_dn5 = assign71310_e107776_d_n5;
        locals.var_ac1_dn6 = assign71310_e107776_d_n6;
        locals.var_ac1_dn7 = assign71310_e107776_d_n7;
        locals.var_ac1_dn8 = assign71310_e107776_d_n8;
        locals.var_ac1_dn9 = assign71310_e107776_d_n9;
        locals.var_ac1_dn10 = assign71310_e107776_d_n10;
        locals.var_ac1_dn11 = assign71310_e107776_d_n11;
        locals.var_ac1_dn14 = assign71310_e107776_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign71320_e107788, assign71320_e107788_d_n0, assign71320_e107788_d_n2, assign71320_e107788_d_n4, assign71320_e107788_d_n5, assign71320_e107788_d_n6, assign71320_e107788_d_n7, assign71320_e107788_d_n8, assign71320_e107788_d_n9, assign71320_e107788_d_n10, assign71320_e107788_d_n11, assign71320_e107788_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign71320_e107785: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign71320_e107786: f64 = (assign71320_e107785).sqrt();
        (assign71320_e107786, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign71320_e107786)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign71320_e107786)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign71320_e107786)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign71320_e107786)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign71320_e107786)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign71320_e107786)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign71320_e107786)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign71320_e107786)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign71320_e107786)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign71320_e107786)), ((locals.var_ac4_dn14 + locals.var_ac3_dn14) / (2.0 * assign71320_e107786)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn14,)
    }
};
        locals.var_ac2 = assign71320_e107788;
        locals.var_ac2_dn0 = assign71320_e107788_d_n0;
        locals.var_ac2_dn2 = assign71320_e107788_d_n2;
        locals.var_ac2_dn4 = assign71320_e107788_d_n4;
        locals.var_ac2_dn5 = assign71320_e107788_d_n5;
        locals.var_ac2_dn6 = assign71320_e107788_d_n6;
        locals.var_ac2_dn7 = assign71320_e107788_d_n7;
        locals.var_ac2_dn8 = assign71320_e107788_d_n8;
        locals.var_ac2_dn9 = assign71320_e107788_d_n9;
        locals.var_ac2_dn10 = assign71320_e107788_d_n10;
        locals.var_ac2_dn11 = assign71320_e107788_d_n11;
        locals.var_ac2_dn14 = assign71320_e107788_d_n14;
        locals.var_ac2_rv = 0.0;

        let (assign71330_e107800, assign71330_e107800_d_n0, assign71330_e107800_d_n2, assign71330_e107800_d_n4, assign71330_e107800_d_n5, assign71330_e107800_d_n6, assign71330_e107800_d_n7, assign71330_e107800_d_n8, assign71330_e107800_d_n9, assign71330_e107800_d_n10, assign71330_e107800_d_n11, assign71330_e107800_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) && (locals.var_guard1668 == 0.0)) {
        let assign71330_e107796: f64 = (-locals.var_ac31);
        let assign71330_e107798: f64 = (assign71330_e107796 + locals.var_ac2);
        (assign71330_e107798, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn11) + locals.var_ac2_dn11), ((-locals.var_ac31_dn14) + locals.var_ac2_dn14),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign71330_e107800;
        locals.var_ac1_dn0 = assign71330_e107800_d_n0;
        locals.var_ac1_dn2 = assign71330_e107800_d_n2;
        locals.var_ac1_dn4 = assign71330_e107800_d_n4;
        locals.var_ac1_dn5 = assign71330_e107800_d_n5;
        locals.var_ac1_dn6 = assign71330_e107800_d_n6;
        locals.var_ac1_dn7 = assign71330_e107800_d_n7;
        locals.var_ac1_dn8 = assign71330_e107800_d_n8;
        locals.var_ac1_dn9 = assign71330_e107800_d_n9;
        locals.var_ac1_dn10 = assign71330_e107800_d_n10;
        locals.var_ac1_dn11 = assign71330_e107800_d_n11;
        locals.var_ac1_dn14 = assign71330_e107800_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign71340_e107808, assign71340_e107808_d_n0, assign71340_e107808_d_n2, assign71340_e107808_d_n4, assign71340_e107808_d_n5, assign71340_e107808_d_n6, assign71340_e107808_d_n7, assign71340_e107808_d_n8, assign71340_e107808_d_n9, assign71340_e107808_d_n10, assign71340_e107808_d_n11, assign71340_e107808_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71340_e107806: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign71340_e107806, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign71340_e107806 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign71340_e107806 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign71340_e107806 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign71340_e107806 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign71340_e107806 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign71340_e107806 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign71340_e107806 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign71340_e107806 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign71340_e107806 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign71340_e107806 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn14)) } } else { (assign71340_e107806 * (0.3333333333333333 * (locals.var_ac1_dn14 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn14,)
    }
};
        locals.var_acd = assign71340_e107808;
        locals.var_acd_dn0 = assign71340_e107808_d_n0;
        locals.var_acd_dn2 = assign71340_e107808_d_n2;
        locals.var_acd_dn4 = assign71340_e107808_d_n4;
        locals.var_acd_dn5 = assign71340_e107808_d_n5;
        locals.var_acd_dn6 = assign71340_e107808_d_n6;
        locals.var_acd_dn7 = assign71340_e107808_d_n7;
        locals.var_acd_dn8 = assign71340_e107808_d_n8;
        locals.var_acd_dn9 = assign71340_e107808_d_n9;
        locals.var_acd_dn10 = assign71340_e107808_d_n10;
        locals.var_acd_dn11 = assign71340_e107808_d_n11;
        locals.var_acd_dn14 = assign71340_e107808_d_n14;
        locals.var_acd_rv = 0.0;

        let (assign71350_e107831, assign71350_e107831_d_n0, assign71350_e107831_d_n2, assign71350_e107831_d_n4, assign71350_e107831_d_n5, assign71350_e107831_d_n6, assign71350_e107831_d_n7, assign71350_e107831_d_n8, assign71350_e107831_d_n9, assign71350_e107831_d_n10, assign71350_e107831_d_n11, assign71350_e107831_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71350_e107813: f64 = (-4.0);
        let assign71350_e107815: f64 = (assign71350_e107813 * 1.414213562373095);
        let assign71350_e107818: f64 = (12.0 * locals.var_ty);
        let assign71350_e107819: f64 = (assign71350_e107815 - assign71350_e107818);
        let assign71350_e107822: f64 = (2.0 * locals.var_acd);
        let assign71350_e107823: f64 = (assign71350_e107819 + assign71350_e107822);
        let assign71350_e107826: f64 = (1.414213562373095 * locals.var_acd);
        let assign71350_e107828: f64 = (assign71350_e107826 * locals.var_acd);
        let assign71350_e107829: f64 = (assign71350_e107823 + assign71350_e107828);
        (assign71350_e107829, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign71350_e107826 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign71350_e107826 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign71350_e107826 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign71350_e107826 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign71350_e107826 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign71350_e107826 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign71350_e107826 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign71350_e107826 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign71350_e107826 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign71350_e107826 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn14)) + (2.0 * locals.var_acd_dn14)) + (((1.414213562373095 * locals.var_acd_dn14) * locals.var_acd) + (assign71350_e107826 * locals.var_acd_dn14))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn14,)
    }
};
        locals.var_acn = assign71350_e107831;
        locals.var_acn_dn0 = assign71350_e107831_d_n0;
        locals.var_acn_dn2 = assign71350_e107831_d_n2;
        locals.var_acn_dn4 = assign71350_e107831_d_n4;
        locals.var_acn_dn5 = assign71350_e107831_d_n5;
        locals.var_acn_dn6 = assign71350_e107831_d_n6;
        locals.var_acn_dn7 = assign71350_e107831_d_n7;
        locals.var_acn_dn8 = assign71350_e107831_d_n8;
        locals.var_acn_dn9 = assign71350_e107831_d_n9;
        locals.var_acn_dn10 = assign71350_e107831_d_n10;
        locals.var_acn_dn11 = assign71350_e107831_d_n11;
        locals.var_acn_dn14 = assign71350_e107831_d_n14;
        locals.var_acn_rv = 0.0;

        let (assign71360_e107839, assign71360_e107839_d_n0, assign71360_e107839_d_n2, assign71360_e107839_d_n4, assign71360_e107839_d_n5, assign71360_e107839_d_n6, assign71360_e107839_d_n7, assign71360_e107839_d_n8, assign71360_e107839_d_n9, assign71360_e107839_d_n10, assign71360_e107839_d_n11, assign71360_e107839_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71360_e107837: f64 = (locals.var_acn / locals.var_acd);
        (assign71360_e107837, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn7 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn7)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn9 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn9)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn11 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn11)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn14 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn14)) / (locals.var_acd * locals.var_acd)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign71360_e107839;
        locals.var_chi_dn0 = assign71360_e107839_d_n0;
        locals.var_chi_dn2 = assign71360_e107839_d_n2;
        locals.var_chi_dn4 = assign71360_e107839_d_n4;
        locals.var_chi_dn5 = assign71360_e107839_d_n5;
        locals.var_chi_dn6 = assign71360_e107839_d_n6;
        locals.var_chi_dn7 = assign71360_e107839_d_n7;
        locals.var_chi_dn8 = assign71360_e107839_d_n8;
        locals.var_chi_dn9 = assign71360_e107839_d_n9;
        locals.var_chi_dn10 = assign71360_e107839_d_n10;
        locals.var_chi_dn11 = assign71360_e107839_d_n11;
        locals.var_chi_dn14 = assign71360_e107839_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign71370_e107847, assign71370_e107847_d_n0, assign71370_e107847_d_n2, assign71370_e107847_d_n4, assign71370_e107847_d_n5, assign71370_e107847_d_n6, assign71370_e107847_d_n7, assign71370_e107847_d_n8, assign71370_e107847_d_n9, assign71370_e107847_d_n10, assign71370_e107847_d_n11, assign71370_e107847_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71370_e107845: f64 = (locals.var_chi * locals.var_beta_inv);
        (assign71370_e107845, ((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)), ((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)), ((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)), ((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)), ((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)), ((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)), ((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)), ((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)), ((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)), ((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)), ((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign71370_e107847;
        locals.var_t1_dn0 = assign71370_e107847_d_n0;
        locals.var_t1_dn2 = assign71370_e107847_d_n2;
        locals.var_t1_dn4 = assign71370_e107847_d_n4;
        locals.var_t1_dn5 = assign71370_e107847_d_n5;
        locals.var_t1_dn6 = assign71370_e107847_d_n6;
        locals.var_t1_dn7 = assign71370_e107847_d_n7;
        locals.var_t1_dn8 = assign71370_e107847_d_n8;
        locals.var_t1_dn9 = assign71370_e107847_d_n9;
        locals.var_t1_dn10 = assign71370_e107847_d_n10;
        locals.var_t1_dn11 = assign71370_e107847_d_n11;
        locals.var_t1_dn14 = assign71370_e107847_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign71380_e107855, assign71380_e107855_d_n0, assign71380_e107855_d_n2, assign71380_e107855_d_n4, assign71380_e107855_d_n5, assign71380_e107855_d_n6, assign71380_e107855_d_n7, assign71380_e107855_d_n8, assign71380_e107855_d_n9, assign71380_e107855_d_n10, assign71380_e107855_d_n11, assign71380_e107855_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71380_e107853: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign71380_e107853, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn14 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn14)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign71380_e107855;
        locals.var_t2_dn0 = assign71380_e107855_d_n0;
        locals.var_t2_dn2 = assign71380_e107855_d_n2;
        locals.var_t2_dn4 = assign71380_e107855_d_n4;
        locals.var_t2_dn5 = assign71380_e107855_d_n5;
        locals.var_t2_dn6 = assign71380_e107855_d_n6;
        locals.var_t2_dn7 = assign71380_e107855_d_n7;
        locals.var_t2_dn8 = assign71380_e107855_d_n8;
        locals.var_t2_dn9 = assign71380_e107855_d_n9;
        locals.var_t2_dn10 = assign71380_e107855_d_n10;
        locals.var_t2_dn11 = assign71380_e107855_d_n11;
        locals.var_t2_dn14 = assign71380_e107855_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign71390_e107866, assign71390_e107866_d_n0, assign71390_e107866_d_n2, assign71390_e107866_d_n4, assign71390_e107866_d_n5, assign71390_e107866_d_n6, assign71390_e107866_d_n7, assign71390_e107866_d_n8, assign71390_e107866_d_n9, assign71390_e107866_d_n10, assign71390_e107866_d_n11, assign71390_e107866_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71390_e107862: f64 = (locals.var_t2 * locals.var_t2);
        let assign71390_e107863: f64 = (1.0 + assign71390_e107862);
        let assign71390_e107864: f64 = (assign71390_e107863).sqrt();
        (assign71390_e107864, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign71390_e107864)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign71390_e107864)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign71390_e107864)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign71390_e107864)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign71390_e107864)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign71390_e107864)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign71390_e107864)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign71390_e107864)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign71390_e107864)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign71390_e107864)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign71390_e107864)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign71390_e107866;
        locals.var_t3_dn0 = assign71390_e107866_d_n0;
        locals.var_t3_dn2 = assign71390_e107866_d_n2;
        locals.var_t3_dn4 = assign71390_e107866_d_n4;
        locals.var_t3_dn5 = assign71390_e107866_d_n5;
        locals.var_t3_dn6 = assign71390_e107866_d_n6;
        locals.var_t3_dn7 = assign71390_e107866_d_n7;
        locals.var_t3_dn8 = assign71390_e107866_d_n8;
        locals.var_t3_dn9 = assign71390_e107866_d_n9;
        locals.var_t3_dn10 = assign71390_e107866_d_n10;
        locals.var_t3_dn11 = assign71390_e107866_d_n11;
        locals.var_t3_dn14 = assign71390_e107866_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign71400_e107876, assign71400_e107876_d_n0, assign71400_e107876_d_n2, assign71400_e107876_d_n4, assign71400_e107876_d_n5, assign71400_e107876_d_n6, assign71400_e107876_d_n7, assign71400_e107876_d_n8, assign71400_e107876_d_n9, assign71400_e107876_d_n10, assign71400_e107876_d_n11, assign71400_e107876_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71400_e107872: f64 = (locals.var_t1 / locals.var_t3);
        let assign71400_e107874: f64 = (assign71400_e107872 - locals.var_vxbgmtcl);
        (assign71400_e107874, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1_dn14 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign71400_e107876;
        locals.var_ps0ld_dn0 = assign71400_e107876_d_n0;
        locals.var_ps0ld_dn2 = assign71400_e107876_d_n2;
        locals.var_ps0ld_dn4 = assign71400_e107876_d_n4;
        locals.var_ps0ld_dn5 = assign71400_e107876_d_n5;
        locals.var_ps0ld_dn6 = assign71400_e107876_d_n6;
        locals.var_ps0ld_dn7 = assign71400_e107876_d_n7;
        locals.var_ps0ld_dn8 = assign71400_e107876_d_n8;
        locals.var_ps0ld_dn9 = assign71400_e107876_d_n9;
        locals.var_ps0ld_dn10 = assign71400_e107876_d_n10;
        locals.var_ps0ld_dn11 = assign71400_e107876_d_n11;
        locals.var_ps0ld_dn14 = assign71400_e107876_d_n14;
        locals.var_ps0ld_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_268(
        locals: &mut StampLocals,
    ) {
        let (assign71410_e107884, assign71410_e107884_d_n0, assign71410_e107884_d_n2, assign71410_e107884_d_n4, assign71410_e107884_d_n5, assign71410_e107884_d_n6, assign71410_e107884_d_n7, assign71410_e107884_d_n8, assign71410_e107884_d_n9, assign71410_e107884_d_n10, assign71410_e107884_d_n11, assign71410_e107884_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71410_e107882: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign71410_e107882, (-locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8), (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn11), (-locals.var_ps0ld_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign71410_e107884;
        locals.var_t2_dn0 = assign71410_e107884_d_n0;
        locals.var_t2_dn2 = assign71410_e107884_d_n2;
        locals.var_t2_dn4 = assign71410_e107884_d_n4;
        locals.var_t2_dn5 = assign71410_e107884_d_n5;
        locals.var_t2_dn6 = assign71410_e107884_d_n6;
        locals.var_t2_dn7 = assign71410_e107884_d_n7;
        locals.var_t2_dn8 = assign71410_e107884_d_n8;
        locals.var_t2_dn9 = assign71410_e107884_d_n9;
        locals.var_t2_dn10 = assign71410_e107884_d_n10;
        locals.var_t2_dn11 = assign71410_e107884_d_n11;
        locals.var_t2_dn14 = assign71410_e107884_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign71420_e107892, assign71420_e107892_d_n0, assign71420_e107892_d_n2, assign71420_e107892_d_n4, assign71420_e107892_d_n5, assign71420_e107892_d_n6, assign71420_e107892_d_n7, assign71420_e107892_d_n8, assign71420_e107892_d_n9, assign71420_e107892_d_n10, assign71420_e107892_d_n11, assign71420_e107892_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        let assign71420_e107890: f64 = (locals.var_cox0_func * locals.var_t2);
        (assign71420_e107890, (locals.var_cox0_func * locals.var_t2_dn0), (locals.var_cox0_func * locals.var_t2_dn2), (locals.var_cox0_func * locals.var_t2_dn4), (locals.var_cox0_func * locals.var_t2_dn5), (locals.var_cox0_func * locals.var_t2_dn6), (locals.var_cox0_func * locals.var_t2_dn7), (locals.var_cox0_func * locals.var_t2_dn8), (locals.var_cox0_func * locals.var_t2_dn9), (locals.var_cox0_func * locals.var_t2_dn10), (locals.var_cox0_func * locals.var_t2_dn11), (locals.var_cox0_func * locals.var_t2_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign71420_e107892;
        locals.var_qsuld_dn0 = assign71420_e107892_d_n0;
        locals.var_qsuld_dn2 = assign71420_e107892_d_n2;
        locals.var_qsuld_dn4 = assign71420_e107892_d_n4;
        locals.var_qsuld_dn5 = assign71420_e107892_d_n5;
        locals.var_qsuld_dn6 = assign71420_e107892_d_n6;
        locals.var_qsuld_dn7 = assign71420_e107892_d_n7;
        locals.var_qsuld_dn8 = assign71420_e107892_d_n8;
        locals.var_qsuld_dn9 = assign71420_e107892_d_n9;
        locals.var_qsuld_dn10 = assign71420_e107892_d_n10;
        locals.var_qsuld_dn11 = assign71420_e107892_d_n11;
        locals.var_qsuld_dn14 = assign71420_e107892_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign71430_e107898, assign71430_e107898_d_n0, assign71430_e107898_d_n2, assign71430_e107898_d_n4, assign71430_e107898_d_n5, assign71430_e107898_d_n6, assign71430_e107898_d_n7, assign71430_e107898_d_n8, assign71430_e107898_d_n9, assign71430_e107898_d_n10, assign71430_e107898_d_n11, assign71430_e107898_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign71430_e107898;
        locals.var_qbuld_dn0 = assign71430_e107898_d_n0;
        locals.var_qbuld_dn2 = assign71430_e107898_d_n2;
        locals.var_qbuld_dn4 = assign71430_e107898_d_n4;
        locals.var_qbuld_dn5 = assign71430_e107898_d_n5;
        locals.var_qbuld_dn6 = assign71430_e107898_d_n6;
        locals.var_qbuld_dn7 = assign71430_e107898_d_n7;
        locals.var_qbuld_dn8 = assign71430_e107898_d_n8;
        locals.var_qbuld_dn9 = assign71430_e107898_d_n9;
        locals.var_qbuld_dn10 = assign71430_e107898_d_n10;
        locals.var_qbuld_dn11 = assign71430_e107898_d_n11;
        locals.var_qbuld_dn14 = assign71430_e107898_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign71440_e107904, assign71440_e107904_d_n0, assign71440_e107904_d_n2, assign71440_e107904_d_n4, assign71440_e107904_d_n5, assign71440_e107904_d_n6, assign71440_e107904_d_n7, assign71440_e107904_d_n8, assign71440_e107904_d_n9, assign71440_e107904_d_n10, assign71440_e107904_d_n11, assign71440_e107904_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_ini, locals.var_ps0ld_ini_dn0, locals.var_ps0ld_ini_dn2, locals.var_ps0ld_ini_dn4, locals.var_ps0ld_ini_dn5, locals.var_ps0ld_ini_dn6, locals.var_ps0ld_ini_dn7, locals.var_ps0ld_ini_dn8, locals.var_ps0ld_ini_dn9, locals.var_ps0ld_ini_dn10, locals.var_ps0ld_ini_dn11, locals.var_ps0ld_ini_dn14,)
    }
};
        locals.var_ps0ld_ini = assign71440_e107904;
        locals.var_ps0ld_ini_dn0 = assign71440_e107904_d_n0;
        locals.var_ps0ld_ini_dn2 = assign71440_e107904_d_n2;
        locals.var_ps0ld_ini_dn4 = assign71440_e107904_d_n4;
        locals.var_ps0ld_ini_dn5 = assign71440_e107904_d_n5;
        locals.var_ps0ld_ini_dn6 = assign71440_e107904_d_n6;
        locals.var_ps0ld_ini_dn7 = assign71440_e107904_d_n7;
        locals.var_ps0ld_ini_dn8 = assign71440_e107904_d_n8;
        locals.var_ps0ld_ini_dn9 = assign71440_e107904_d_n9;
        locals.var_ps0ld_ini_dn10 = assign71440_e107904_d_n10;
        locals.var_ps0ld_ini_dn11 = assign71440_e107904_d_n11;
        locals.var_ps0ld_ini_dn14 = assign71440_e107904_d_n14;
        locals.var_ps0ld_ini_rv = 0.0;

        let assign71450_e107908: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71450_e107909: f64 = (locals.var_beta * assign71450_e107908);
        let assign71450_e107913: f64 = (10.0 * 2.220446049250313e-16);
        let assign71450_e107915: f64 = (assign71450_e107913 - 1.0);
        let assign71450_e107917: f64 = (assign71450_e107915 * locals.var_fac1p2);
        let assign71450_e107919: f64 = (assign71450_e107917 * locals.var_beta2);
        let assign71450_e107921: f64 = (assign71450_e107919 / 4.0);
        let assign71450_e107922: f64 = (1.0 + assign71450_e107921);
        let assign71450_e107923: f64 = if assign71450_e107909 < assign71450_e107922 { 1.0 } else { 0.0 };
        locals.var_guard1669 = assign71450_e107923;
        locals.var_guard1669_rv = 0.0;

        let (assign71460_e107938, assign71460_e107938_d_n0, assign71460_e107938_d_n2, assign71460_e107938_d_n4, assign71460_e107938_d_n5, assign71460_e107938_d_n6, assign71460_e107938_d_n7, assign71460_e107938_d_n8, assign71460_e107938_d_n9, assign71460_e107938_d_n10, assign71460_e107938_d_n11, assign71460_e107938_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1669 != 0.0)) {
        let assign71460_e107933: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign71460_e107935: f64 = (assign71460_e107933 / 2.0);
        let assign71460_e107936: f64 = (locals.var_vgpld + assign71460_e107935);
        (assign71460_e107936, (((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0), (locals.var_vgpld_dn2 + (((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0)), (((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0), (((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0), (((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0), (locals.var_vgpld_dn7 + (((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0)), (locals.var_vgpld_dn8 + (((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0)), (locals.var_vgpld_dn9 + (((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0)), (((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0), (((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0), (((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign71460_e107938;
        locals.var_ps0_inia_dn0 = assign71460_e107938_d_n0;
        locals.var_ps0_inia_dn2 = assign71460_e107938_d_n2;
        locals.var_ps0_inia_dn4 = assign71460_e107938_d_n4;
        locals.var_ps0_inia_dn5 = assign71460_e107938_d_n5;
        locals.var_ps0_inia_dn6 = assign71460_e107938_d_n6;
        locals.var_ps0_inia_dn7 = assign71460_e107938_d_n7;
        locals.var_ps0_inia_dn8 = assign71460_e107938_d_n8;
        locals.var_ps0_inia_dn9 = assign71460_e107938_d_n9;
        locals.var_ps0_inia_dn10 = assign71460_e107938_d_n10;
        locals.var_ps0_inia_dn11 = assign71460_e107938_d_n11;
        locals.var_ps0_inia_dn14 = assign71460_e107938_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign71470_e107962, assign71470_e107962_d_n0, assign71470_e107962_d_n2, assign71470_e107962_d_n4, assign71470_e107962_d_n5, assign71470_e107962_d_n6, assign71470_e107962_d_n7, assign71470_e107962_d_n8, assign71470_e107962_d_n9, assign71470_e107962_d_n10, assign71470_e107962_d_n11, assign71470_e107962_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1669 == 0.0)) {
        let assign71470_e107951: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71470_e107952: f64 = (locals.var_beta * assign71470_e107951);
        let assign71470_e107954: f64 = (assign71470_e107952 - 1.0);
        let assign71470_e107955: f64 = (4.0 * assign71470_e107954);
        let assign71470_e107958: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign71470_e107959: f64 = (assign71470_e107955 / assign71470_e107958);
        let assign71470_e107960: f64 = (1.0 + assign71470_e107959);
        (assign71470_e107960, ((((4.0 * ((locals.var_beta_dn0 * assign71470_e107951) + (locals.var_beta * locals.var_vxbgmtcl_dn0))) * assign71470_e107958) - (assign71470_e107955 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign71470_e107958 * assign71470_e107958)), ((((4.0 * ((locals.var_beta_dn2 * assign71470_e107951) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)))) * assign71470_e107958) - (assign71470_e107955 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign71470_e107958 * assign71470_e107958)), ((((4.0 * ((locals.var_beta_dn4 * assign71470_e107951) + (locals.var_beta * locals.var_vxbgmtcl_dn4))) * assign71470_e107958) - (assign71470_e107955 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign71470_e107958 * assign71470_e107958)), ((((4.0 * ((locals.var_beta_dn5 * assign71470_e107951) + (locals.var_beta * locals.var_vxbgmtcl_dn5))) * assign71470_e107958) - (assign71470_e107955 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign71470_e107958 * assign71470_e107958)), ((((4.0 * ((locals.var_beta_dn6 * assign71470_e107951) + (locals.var_beta * locals.var_vxbgmtcl_dn6))) * assign71470_e107958) - (assign71470_e107955 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign71470_e107958 * assign71470_e107958)), ((((4.0 * ((locals.var_beta_dn7 * assign71470_e107951) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)))) * assign71470_e107958) - (assign71470_e107955 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign71470_e107958 * assign71470_e107958)), ((((4.0 * ((locals.var_beta_dn8 * assign71470_e107951) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)))) * assign71470_e107958) - (assign71470_e107955 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign71470_e107958 * assign71470_e107958)), ((((4.0 * ((locals.var_beta_dn9 * assign71470_e107951) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)))) * assign71470_e107958) - (assign71470_e107955 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign71470_e107958 * assign71470_e107958)), ((((4.0 * ((locals.var_beta_dn10 * assign71470_e107951) + (locals.var_beta * locals.var_vxbgmtcl_dn10))) * assign71470_e107958) - (assign71470_e107955 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign71470_e107958 * assign71470_e107958)), ((((4.0 * ((locals.var_beta_dn11 * assign71470_e107951) + (locals.var_beta * locals.var_vxbgmtcl_dn11))) * assign71470_e107958) - (assign71470_e107955 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign71470_e107958 * assign71470_e107958)), ((((4.0 * ((locals.var_beta_dn14 * assign71470_e107951) + (locals.var_beta * locals.var_vxbgmtcl_dn14))) * assign71470_e107958) - (assign71470_e107955 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign71470_e107958 * assign71470_e107958)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign71470_e107962;
        locals.var_tx_dn0 = assign71470_e107962_d_n0;
        locals.var_tx_dn2 = assign71470_e107962_d_n2;
        locals.var_tx_dn4 = assign71470_e107962_d_n4;
        locals.var_tx_dn5 = assign71470_e107962_d_n5;
        locals.var_tx_dn6 = assign71470_e107962_d_n6;
        locals.var_tx_dn7 = assign71470_e107962_d_n7;
        locals.var_tx_dn8 = assign71470_e107962_d_n8;
        locals.var_tx_dn9 = assign71470_e107962_d_n9;
        locals.var_tx_dn10 = assign71470_e107962_d_n10;
        locals.var_tx_dn11 = assign71470_e107962_d_n11;
        locals.var_tx_dn14 = assign71470_e107962_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign71480_e107983, assign71480_e107983_d_n0, assign71480_e107983_d_n2, assign71480_e107983_d_n4, assign71480_e107983_d_n5, assign71480_e107983_d_n6, assign71480_e107983_d_n7, assign71480_e107983_d_n8, assign71480_e107983_d_n9, assign71480_e107983_d_n10, assign71480_e107983_d_n11, assign71480_e107983_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1669 == 0.0)) {
        let assign71480_e107973: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign71480_e107975: f64 = (assign71480_e107973 / 2.0);
        let assign71480_e107978: f64 = (locals.var_tx).sqrt();
        let assign71480_e107979: f64 = (1.0 - assign71480_e107978);
        let assign71480_e107980: f64 = (assign71480_e107975 * assign71480_e107979);
        let assign71480_e107981: f64 = (locals.var_vgpld + assign71480_e107980);
        (assign71480_e107981, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign71480_e107979) + (assign71480_e107975 * (-(locals.var_tx_dn0 / (2.0 * assign71480_e107978))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign71480_e107979) + (assign71480_e107975 * (-(locals.var_tx_dn2 / (2.0 * assign71480_e107978)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign71480_e107979) + (assign71480_e107975 * (-(locals.var_tx_dn4 / (2.0 * assign71480_e107978))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign71480_e107979) + (assign71480_e107975 * (-(locals.var_tx_dn5 / (2.0 * assign71480_e107978))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign71480_e107979) + (assign71480_e107975 * (-(locals.var_tx_dn6 / (2.0 * assign71480_e107978))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign71480_e107979) + (assign71480_e107975 * (-(locals.var_tx_dn7 / (2.0 * assign71480_e107978)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign71480_e107979) + (assign71480_e107975 * (-(locals.var_tx_dn8 / (2.0 * assign71480_e107978)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign71480_e107979) + (assign71480_e107975 * (-(locals.var_tx_dn9 / (2.0 * assign71480_e107978)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign71480_e107979) + (assign71480_e107975 * (-(locals.var_tx_dn10 / (2.0 * assign71480_e107978))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign71480_e107979) + (assign71480_e107975 * (-(locals.var_tx_dn11 / (2.0 * assign71480_e107978))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign71480_e107979) + (assign71480_e107975 * (-(locals.var_tx_dn14 / (2.0 * assign71480_e107978))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign71480_e107983;
        locals.var_ps0_inia_dn0 = assign71480_e107983_d_n0;
        locals.var_ps0_inia_dn2 = assign71480_e107983_d_n2;
        locals.var_ps0_inia_dn4 = assign71480_e107983_d_n4;
        locals.var_ps0_inia_dn5 = assign71480_e107983_d_n5;
        locals.var_ps0_inia_dn6 = assign71480_e107983_d_n6;
        locals.var_ps0_inia_dn7 = assign71480_e107983_d_n7;
        locals.var_ps0_inia_dn8 = assign71480_e107983_d_n8;
        locals.var_ps0_inia_dn9 = assign71480_e107983_d_n9;
        locals.var_ps0_inia_dn10 = assign71480_e107983_d_n10;
        locals.var_ps0_inia_dn11 = assign71480_e107983_d_n11;
        locals.var_ps0_inia_dn14 = assign71480_e107983_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign71490_e107994, assign71490_e107994_d_n0, assign71490_e107994_d_n2, assign71490_e107994_d_n4, assign71490_e107994_d_n5, assign71490_e107994_d_n6, assign71490_e107994_d_n7, assign71490_e107994_d_n8, assign71490_e107994_d_n9, assign71490_e107994_d_n10, assign71490_e107994_d_n11, assign71490_e107994_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) {
        let assign71490_e107991: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign71490_e107992: f64 = (locals.var_beta * assign71490_e107991);
        (assign71490_e107992, ((locals.var_beta_dn0 * assign71490_e107991) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign71490_e107991) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign71490_e107991) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign71490_e107991) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign71490_e107991) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign71490_e107991) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign71490_e107991) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign71490_e107991) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign71490_e107991) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign71490_e107991) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign71490_e107991) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign71490_e107994;
        locals.var_chi_dn0 = assign71490_e107994_d_n0;
        locals.var_chi_dn2 = assign71490_e107994_d_n2;
        locals.var_chi_dn4 = assign71490_e107994_d_n4;
        locals.var_chi_dn5 = assign71490_e107994_d_n5;
        locals.var_chi_dn6 = assign71490_e107994_d_n6;
        locals.var_chi_dn7 = assign71490_e107994_d_n7;
        locals.var_chi_dn8 = assign71490_e107994_d_n8;
        locals.var_chi_dn9 = assign71490_e107994_d_n9;
        locals.var_chi_dn10 = assign71490_e107994_d_n10;
        locals.var_chi_dn11 = assign71490_e107994_d_n11;
        locals.var_chi_dn14 = assign71490_e107994_d_n14;
        locals.var_chi_rv = 0.0;

        let assign71500_e107997: f64 = if locals.var_chi >= 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1670 = assign71500_e107997;
        locals.var_guard1670_rv = 0.0;

        let (assign71520_e108017, assign71520_e108017_d_n0, assign71520_e108017_d_n2, assign71520_e108017_d_n4, assign71520_e108017_d_n5, assign71520_e108017_d_n6, assign71520_e108017_d_n7, assign71520_e108017_d_n8, assign71520_e108017_d_n9, assign71520_e108017_d_n10, assign71520_e108017_d_n11, assign71520_e108017_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 != 0.0)) {
        let assign71520_e108014: f64 = (-locals.var_chi);
        let assign71520_e108015: f64 = (assign71520_e108014).exp();
        (assign71520_e108015, (assign71520_e108015 * (-locals.var_chi_dn0)), (assign71520_e108015 * (-locals.var_chi_dn2)), (assign71520_e108015 * (-locals.var_chi_dn4)), (assign71520_e108015 * (-locals.var_chi_dn5)), (assign71520_e108015 * (-locals.var_chi_dn6)), (assign71520_e108015 * (-locals.var_chi_dn7)), (assign71520_e108015 * (-locals.var_chi_dn8)), (assign71520_e108015 * (-locals.var_chi_dn9)), (assign71520_e108015 * (-locals.var_chi_dn10)), (assign71520_e108015 * (-locals.var_chi_dn11)), (assign71520_e108015 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign71520_e108017;
        locals.var_ty_dn0 = assign71520_e108017_d_n0;
        locals.var_ty_dn2 = assign71520_e108017_d_n2;
        locals.var_ty_dn4 = assign71520_e108017_d_n4;
        locals.var_ty_dn5 = assign71520_e108017_d_n5;
        locals.var_ty_dn6 = assign71520_e108017_d_n6;
        locals.var_ty_dn7 = assign71520_e108017_d_n7;
        locals.var_ty_dn8 = assign71520_e108017_d_n8;
        locals.var_ty_dn9 = assign71520_e108017_d_n9;
        locals.var_ty_dn10 = assign71520_e108017_d_n10;
        locals.var_ty_dn11 = assign71520_e108017_d_n11;
        locals.var_ty_dn14 = assign71520_e108017_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign71530_e108042, assign71530_e108042_d_n0, assign71530_e108042_d_n2, assign71530_e108042_d_n4, assign71530_e108042_d_n5, assign71530_e108042_d_n6, assign71530_e108042_d_n7, assign71530_e108042_d_n8, assign71530_e108042_d_n9, assign71530_e108042_d_n10, assign71530_e108042_d_n11, assign71530_e108042_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 != 0.0)) {
        let assign71530_e108029: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71530_e108030: f64 = (locals.var_beta * assign71530_e108029);
        let assign71530_e108032: f64 = (assign71530_e108030 - 1.0);
        let assign71530_e108034: f64 = (assign71530_e108032 + locals.var_ty);
        let assign71530_e108035: f64 = (4.0 * assign71530_e108034);
        let assign71530_e108038: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign71530_e108039: f64 = (assign71530_e108035 / assign71530_e108038);
        let assign71530_e108040: f64 = (1.0 + assign71530_e108039);
        (assign71530_e108040, ((((4.0 * (((locals.var_beta_dn0 * assign71530_e108029) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign71530_e108038) - (assign71530_e108035 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign71530_e108038 * assign71530_e108038)), ((((4.0 * (((locals.var_beta_dn2 * assign71530_e108029) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign71530_e108038) - (assign71530_e108035 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign71530_e108038 * assign71530_e108038)), ((((4.0 * (((locals.var_beta_dn4 * assign71530_e108029) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign71530_e108038) - (assign71530_e108035 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign71530_e108038 * assign71530_e108038)), ((((4.0 * (((locals.var_beta_dn5 * assign71530_e108029) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign71530_e108038) - (assign71530_e108035 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign71530_e108038 * assign71530_e108038)), ((((4.0 * (((locals.var_beta_dn6 * assign71530_e108029) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign71530_e108038) - (assign71530_e108035 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign71530_e108038 * assign71530_e108038)), ((((4.0 * (((locals.var_beta_dn7 * assign71530_e108029) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign71530_e108038) - (assign71530_e108035 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign71530_e108038 * assign71530_e108038)), ((((4.0 * (((locals.var_beta_dn8 * assign71530_e108029) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign71530_e108038) - (assign71530_e108035 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign71530_e108038 * assign71530_e108038)), ((((4.0 * (((locals.var_beta_dn9 * assign71530_e108029) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign71530_e108038) - (assign71530_e108035 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign71530_e108038 * assign71530_e108038)), ((((4.0 * (((locals.var_beta_dn10 * assign71530_e108029) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign71530_e108038) - (assign71530_e108035 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign71530_e108038 * assign71530_e108038)), ((((4.0 * (((locals.var_beta_dn11 * assign71530_e108029) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign71530_e108038) - (assign71530_e108035 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign71530_e108038 * assign71530_e108038)), ((((4.0 * (((locals.var_beta_dn14 * assign71530_e108029) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign71530_e108038) - (assign71530_e108035 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign71530_e108038 * assign71530_e108038)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign71530_e108042;
        locals.var_tx_dn0 = assign71530_e108042_d_n0;
        locals.var_tx_dn2 = assign71530_e108042_d_n2;
        locals.var_tx_dn4 = assign71530_e108042_d_n4;
        locals.var_tx_dn5 = assign71530_e108042_d_n5;
        locals.var_tx_dn6 = assign71530_e108042_d_n6;
        locals.var_tx_dn7 = assign71530_e108042_d_n7;
        locals.var_tx_dn8 = assign71530_e108042_d_n8;
        locals.var_tx_dn9 = assign71530_e108042_d_n9;
        locals.var_tx_dn10 = assign71530_e108042_d_n10;
        locals.var_tx_dn11 = assign71530_e108042_d_n11;
        locals.var_tx_dn14 = assign71530_e108042_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign71540_e108062, assign71540_e108062_d_n0, assign71540_e108062_d_n2, assign71540_e108062_d_n4, assign71540_e108062_d_n5, assign71540_e108062_d_n6, assign71540_e108062_d_n7, assign71540_e108062_d_n8, assign71540_e108062_d_n9, assign71540_e108062_d_n10, assign71540_e108062_d_n11, assign71540_e108062_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 != 0.0)) {
        let assign71540_e108052: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign71540_e108054: f64 = (assign71540_e108052 / 2.0);
        let assign71540_e108057: f64 = (locals.var_tx).sqrt();
        let assign71540_e108058: f64 = (1.0 - assign71540_e108057);
        let assign71540_e108059: f64 = (assign71540_e108054 * assign71540_e108058);
        let assign71540_e108060: f64 = (locals.var_vgpld + assign71540_e108059);
        (assign71540_e108060, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign71540_e108058) + (assign71540_e108054 * (-(locals.var_tx_dn0 / (2.0 * assign71540_e108057))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign71540_e108058) + (assign71540_e108054 * (-(locals.var_tx_dn2 / (2.0 * assign71540_e108057)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign71540_e108058) + (assign71540_e108054 * (-(locals.var_tx_dn4 / (2.0 * assign71540_e108057))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign71540_e108058) + (assign71540_e108054 * (-(locals.var_tx_dn5 / (2.0 * assign71540_e108057))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign71540_e108058) + (assign71540_e108054 * (-(locals.var_tx_dn6 / (2.0 * assign71540_e108057))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign71540_e108058) + (assign71540_e108054 * (-(locals.var_tx_dn7 / (2.0 * assign71540_e108057)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign71540_e108058) + (assign71540_e108054 * (-(locals.var_tx_dn8 / (2.0 * assign71540_e108057)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign71540_e108058) + (assign71540_e108054 * (-(locals.var_tx_dn9 / (2.0 * assign71540_e108057)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign71540_e108058) + (assign71540_e108054 * (-(locals.var_tx_dn10 / (2.0 * assign71540_e108057))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign71540_e108058) + (assign71540_e108054 * (-(locals.var_tx_dn11 / (2.0 * assign71540_e108057))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign71540_e108058) + (assign71540_e108054 * (-(locals.var_tx_dn14 / (2.0 * assign71540_e108057))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign71540_e108062;
        locals.var_ps0_inia_dn0 = assign71540_e108062_d_n0;
        locals.var_ps0_inia_dn2 = assign71540_e108062_d_n2;
        locals.var_ps0_inia_dn4 = assign71540_e108062_d_n4;
        locals.var_ps0_inia_dn5 = assign71540_e108062_d_n5;
        locals.var_ps0_inia_dn6 = assign71540_e108062_d_n6;
        locals.var_ps0_inia_dn7 = assign71540_e108062_d_n7;
        locals.var_ps0_inia_dn8 = assign71540_e108062_d_n8;
        locals.var_ps0_inia_dn9 = assign71540_e108062_d_n9;
        locals.var_ps0_inia_dn10 = assign71540_e108062_d_n10;
        locals.var_ps0_inia_dn11 = assign71540_e108062_d_n11;
        locals.var_ps0_inia_dn14 = assign71540_e108062_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign71550_e108075, assign71550_e108075_d_n0, assign71550_e108075_d_n2, assign71550_e108075_d_n4, assign71550_e108075_d_n5, assign71550_e108075_d_n6, assign71550_e108075_d_n7, assign71550_e108075_d_n8, assign71550_e108075_d_n9, assign71550_e108075_d_n10, assign71550_e108075_d_n11, assign71550_e108075_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 != 0.0)) {
        let assign71550_e108072: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign71550_e108073: f64 = (locals.var_beta * assign71550_e108072);
        (assign71550_e108073, ((locals.var_beta_dn0 * assign71550_e108072) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign71550_e108072) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign71550_e108072) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign71550_e108072) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign71550_e108072) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign71550_e108072) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign71550_e108072) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign71550_e108072) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign71550_e108072) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign71550_e108072) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign71550_e108072) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign71550_e108075;
        locals.var_chi_dn0 = assign71550_e108075_d_n0;
        locals.var_chi_dn2 = assign71550_e108075_d_n2;
        locals.var_chi_dn4 = assign71550_e108075_d_n4;
        locals.var_chi_dn5 = assign71550_e108075_d_n5;
        locals.var_chi_dn6 = assign71550_e108075_d_n6;
        locals.var_chi_dn7 = assign71550_e108075_d_n7;
        locals.var_chi_dn8 = assign71550_e108075_d_n8;
        locals.var_chi_dn9 = assign71550_e108075_d_n9;
        locals.var_chi_dn10 = assign71550_e108075_d_n10;
        locals.var_chi_dn11 = assign71550_e108075_d_n11;
        locals.var_chi_dn14 = assign71550_e108075_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign71560_e108086, assign71560_e108086_d_n0, assign71560_e108086_d_n2, assign71560_e108086_d_n4, assign71560_e108086_d_n5, assign71560_e108086_d_n6, assign71560_e108086_d_n7, assign71560_e108086_d_n8, assign71560_e108086_d_n9, assign71560_e108086_d_n10, assign71560_e108086_d_n11, assign71560_e108086_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 != 0.0)) {
        let assign71560_e108083: f64 = (-locals.var_chi);
        let assign71560_e108084: f64 = (assign71560_e108083).exp();
        (assign71560_e108084, (assign71560_e108084 * (-locals.var_chi_dn0)), (assign71560_e108084 * (-locals.var_chi_dn2)), (assign71560_e108084 * (-locals.var_chi_dn4)), (assign71560_e108084 * (-locals.var_chi_dn5)), (assign71560_e108084 * (-locals.var_chi_dn6)), (assign71560_e108084 * (-locals.var_chi_dn7)), (assign71560_e108084 * (-locals.var_chi_dn8)), (assign71560_e108084 * (-locals.var_chi_dn9)), (assign71560_e108084 * (-locals.var_chi_dn10)), (assign71560_e108084 * (-locals.var_chi_dn11)), (assign71560_e108084 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign71560_e108086;
        locals.var_ty_dn0 = assign71560_e108086_d_n0;
        locals.var_ty_dn2 = assign71560_e108086_d_n2;
        locals.var_ty_dn4 = assign71560_e108086_d_n4;
        locals.var_ty_dn5 = assign71560_e108086_d_n5;
        locals.var_ty_dn6 = assign71560_e108086_d_n6;
        locals.var_ty_dn7 = assign71560_e108086_d_n7;
        locals.var_ty_dn8 = assign71560_e108086_d_n8;
        locals.var_ty_dn9 = assign71560_e108086_d_n9;
        locals.var_ty_dn10 = assign71560_e108086_d_n10;
        locals.var_ty_dn11 = assign71560_e108086_d_n11;
        locals.var_ty_dn14 = assign71560_e108086_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign71570_e108111, assign71570_e108111_d_n0, assign71570_e108111_d_n2, assign71570_e108111_d_n4, assign71570_e108111_d_n5, assign71570_e108111_d_n6, assign71570_e108111_d_n7, assign71570_e108111_d_n8, assign71570_e108111_d_n9, assign71570_e108111_d_n10, assign71570_e108111_d_n11, assign71570_e108111_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 != 0.0)) {
        let assign71570_e108098: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71570_e108099: f64 = (locals.var_beta * assign71570_e108098);
        let assign71570_e108101: f64 = (assign71570_e108099 - 1.0);
        let assign71570_e108103: f64 = (assign71570_e108101 + locals.var_ty);
        let assign71570_e108104: f64 = (4.0 * assign71570_e108103);
        let assign71570_e108107: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign71570_e108108: f64 = (assign71570_e108104 / assign71570_e108107);
        let assign71570_e108109: f64 = (1.0 + assign71570_e108108);
        (assign71570_e108109, ((((4.0 * (((locals.var_beta_dn0 * assign71570_e108098) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign71570_e108107) - (assign71570_e108104 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign71570_e108107 * assign71570_e108107)), ((((4.0 * (((locals.var_beta_dn2 * assign71570_e108098) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign71570_e108107) - (assign71570_e108104 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign71570_e108107 * assign71570_e108107)), ((((4.0 * (((locals.var_beta_dn4 * assign71570_e108098) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign71570_e108107) - (assign71570_e108104 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign71570_e108107 * assign71570_e108107)), ((((4.0 * (((locals.var_beta_dn5 * assign71570_e108098) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign71570_e108107) - (assign71570_e108104 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign71570_e108107 * assign71570_e108107)), ((((4.0 * (((locals.var_beta_dn6 * assign71570_e108098) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign71570_e108107) - (assign71570_e108104 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign71570_e108107 * assign71570_e108107)), ((((4.0 * (((locals.var_beta_dn7 * assign71570_e108098) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign71570_e108107) - (assign71570_e108104 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign71570_e108107 * assign71570_e108107)), ((((4.0 * (((locals.var_beta_dn8 * assign71570_e108098) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign71570_e108107) - (assign71570_e108104 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign71570_e108107 * assign71570_e108107)), ((((4.0 * (((locals.var_beta_dn9 * assign71570_e108098) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign71570_e108107) - (assign71570_e108104 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign71570_e108107 * assign71570_e108107)), ((((4.0 * (((locals.var_beta_dn10 * assign71570_e108098) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign71570_e108107) - (assign71570_e108104 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign71570_e108107 * assign71570_e108107)), ((((4.0 * (((locals.var_beta_dn11 * assign71570_e108098) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign71570_e108107) - (assign71570_e108104 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign71570_e108107 * assign71570_e108107)), ((((4.0 * (((locals.var_beta_dn14 * assign71570_e108098) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign71570_e108107) - (assign71570_e108104 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign71570_e108107 * assign71570_e108107)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign71570_e108111;
        locals.var_tx_dn0 = assign71570_e108111_d_n0;
        locals.var_tx_dn2 = assign71570_e108111_d_n2;
        locals.var_tx_dn4 = assign71570_e108111_d_n4;
        locals.var_tx_dn5 = assign71570_e108111_d_n5;
        locals.var_tx_dn6 = assign71570_e108111_d_n6;
        locals.var_tx_dn7 = assign71570_e108111_d_n7;
        locals.var_tx_dn8 = assign71570_e108111_d_n8;
        locals.var_tx_dn9 = assign71570_e108111_d_n9;
        locals.var_tx_dn10 = assign71570_e108111_d_n10;
        locals.var_tx_dn11 = assign71570_e108111_d_n11;
        locals.var_tx_dn14 = assign71570_e108111_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign71580_e108131, assign71580_e108131_d_n0, assign71580_e108131_d_n2, assign71580_e108131_d_n4, assign71580_e108131_d_n5, assign71580_e108131_d_n6, assign71580_e108131_d_n7, assign71580_e108131_d_n8, assign71580_e108131_d_n9, assign71580_e108131_d_n10, assign71580_e108131_d_n11, assign71580_e108131_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 != 0.0)) {
        let assign71580_e108121: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign71580_e108123: f64 = (assign71580_e108121 / 2.0);
        let assign71580_e108126: f64 = (locals.var_tx).sqrt();
        let assign71580_e108127: f64 = (1.0 - assign71580_e108126);
        let assign71580_e108128: f64 = (assign71580_e108123 * assign71580_e108127);
        let assign71580_e108129: f64 = (locals.var_vgpld + assign71580_e108128);
        (assign71580_e108129, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign71580_e108127) + (assign71580_e108123 * (-(locals.var_tx_dn0 / (2.0 * assign71580_e108126))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign71580_e108127) + (assign71580_e108123 * (-(locals.var_tx_dn2 / (2.0 * assign71580_e108126)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign71580_e108127) + (assign71580_e108123 * (-(locals.var_tx_dn4 / (2.0 * assign71580_e108126))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign71580_e108127) + (assign71580_e108123 * (-(locals.var_tx_dn5 / (2.0 * assign71580_e108126))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign71580_e108127) + (assign71580_e108123 * (-(locals.var_tx_dn6 / (2.0 * assign71580_e108126))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign71580_e108127) + (assign71580_e108123 * (-(locals.var_tx_dn7 / (2.0 * assign71580_e108126)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign71580_e108127) + (assign71580_e108123 * (-(locals.var_tx_dn8 / (2.0 * assign71580_e108126)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign71580_e108127) + (assign71580_e108123 * (-(locals.var_tx_dn9 / (2.0 * assign71580_e108126)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign71580_e108127) + (assign71580_e108123 * (-(locals.var_tx_dn10 / (2.0 * assign71580_e108126))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign71580_e108127) + (assign71580_e108123 * (-(locals.var_tx_dn11 / (2.0 * assign71580_e108126))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign71580_e108127) + (assign71580_e108123 * (-(locals.var_tx_dn14 / (2.0 * assign71580_e108126))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign71580_e108131;
        locals.var_ps0_inia_dn0 = assign71580_e108131_d_n0;
        locals.var_ps0_inia_dn2 = assign71580_e108131_d_n2;
        locals.var_ps0_inia_dn4 = assign71580_e108131_d_n4;
        locals.var_ps0_inia_dn5 = assign71580_e108131_d_n5;
        locals.var_ps0_inia_dn6 = assign71580_e108131_d_n6;
        locals.var_ps0_inia_dn7 = assign71580_e108131_d_n7;
        locals.var_ps0_inia_dn8 = assign71580_e108131_d_n8;
        locals.var_ps0_inia_dn9 = assign71580_e108131_d_n9;
        locals.var_ps0_inia_dn10 = assign71580_e108131_d_n10;
        locals.var_ps0_inia_dn11 = assign71580_e108131_d_n11;
        locals.var_ps0_inia_dn14 = assign71580_e108131_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign71590_e108144, assign71590_e108144_d_n0, assign71590_e108144_d_n2, assign71590_e108144_d_n4, assign71590_e108144_d_n5, assign71590_e108144_d_n6, assign71590_e108144_d_n7, assign71590_e108144_d_n8, assign71590_e108144_d_n9, assign71590_e108144_d_n10, assign71590_e108144_d_n11, assign71590_e108144_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 != 0.0)) {
        let assign71590_e108141: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign71590_e108142: f64 = (locals.var_beta * assign71590_e108141);
        (assign71590_e108142, ((locals.var_beta_dn0 * assign71590_e108141) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign71590_e108141) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign71590_e108141) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign71590_e108141) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign71590_e108141) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign71590_e108141) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign71590_e108141) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign71590_e108141) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign71590_e108141) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign71590_e108141) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign71590_e108141) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign71590_e108144;
        locals.var_chi_dn0 = assign71590_e108144_d_n0;
        locals.var_chi_dn2 = assign71590_e108144_d_n2;
        locals.var_chi_dn4 = assign71590_e108144_d_n4;
        locals.var_chi_dn5 = assign71590_e108144_d_n5;
        locals.var_chi_dn6 = assign71590_e108144_d_n6;
        locals.var_chi_dn7 = assign71590_e108144_d_n7;
        locals.var_chi_dn8 = assign71590_e108144_d_n8;
        locals.var_chi_dn9 = assign71590_e108144_d_n9;
        locals.var_chi_dn10 = assign71590_e108144_d_n10;
        locals.var_chi_dn11 = assign71590_e108144_d_n11;
        locals.var_chi_dn14 = assign71590_e108144_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign71610_e108186,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 == 0.0)) {
        let assign71610_e108165: f64 = (2.0_f64).sqrt();
        let assign71610_e108166: f64 = (9.0 * assign71610_e108165);
        let assign71610_e108167: f64 = (1.0 / assign71610_e108166);
        let assign71610_e108171: f64 = (-3.0);
        let assign71610_e108172: f64 = (assign71610_e108171).exp();
        let assign71610_e108173: f64 = (7.0 * assign71610_e108172);
        let assign71610_e108174: f64 = (5.0 + assign71610_e108173);
        let assign71610_e108178: f64 = (-3.0);
        let assign71610_e108179: f64 = (assign71610_e108178).exp();
        let assign71610_e108180: f64 = (2.0 + assign71610_e108179);
        let assign71610_e108181: f64 = (assign71610_e108180).sqrt();
        let assign71610_e108182: f64 = (54.0 * assign71610_e108181);
        let assign71610_e108183: f64 = (assign71610_e108174 / assign71610_e108182);
        let assign71610_e108184: f64 = (assign71610_e108167 - assign71610_e108183);
        (assign71610_e108184,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign71610_e108186;
        locals.var_ta_rv = 0.0;

        let (assign71620_e108214,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 == 0.0)) {
        let assign71620_e108196: f64 = (-3.0);
        let assign71620_e108197: f64 = (assign71620_e108196).exp();
        let assign71620_e108198: f64 = (1.0 + assign71620_e108197);
        let assign71620_e108202: f64 = (-3.0);
        let assign71620_e108203: f64 = (assign71620_e108202).exp();
        let assign71620_e108204: f64 = (2.0 + assign71620_e108203);
        let assign71620_e108205: f64 = (assign71620_e108204).sqrt();
        let assign71620_e108206: f64 = (2.0 * assign71620_e108205);
        let assign71620_e108207: f64 = (assign71620_e108198 / assign71620_e108206);
        let assign71620_e108209: f64 = (2.0_f64).sqrt();
        let assign71620_e108211: f64 = (assign71620_e108209 / 3.0);
        let assign71620_e108212: f64 = (assign71620_e108207 - assign71620_e108211);
        (assign71620_e108212,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign71620_e108214;
        locals.var_tb_rv = 0.0;

        let (assign71630_e108233, assign71630_e108233_d_n0, assign71630_e108233_d_n2, assign71630_e108233_d_n4, assign71630_e108233_d_n5, assign71630_e108233_d_n6, assign71630_e108233_d_n7, assign71630_e108233_d_n8, assign71630_e108233_d_n9, assign71630_e108233_d_n10, assign71630_e108233_d_n11, assign71630_e108233_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 == 0.0)) {
        let assign71630_e108224: f64 = (2.0_f64).sqrt();
        let assign71630_e108225: f64 = (1.0 / assign71630_e108224);
        let assign71630_e108229: f64 = (locals.var_beta * locals.var_fac1);
        let assign71630_e108230: f64 = (1.0 / assign71630_e108229);
        let assign71630_e108231: f64 = (assign71630_e108225 + assign71630_e108230);
        (assign71630_e108231, (-(((locals.var_beta_dn0 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn0)) / (assign71630_e108229 * assign71630_e108229))), (-(((locals.var_beta_dn2 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn2)) / (assign71630_e108229 * assign71630_e108229))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign71630_e108229 * assign71630_e108229))), (-(((locals.var_beta_dn5 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn5)) / (assign71630_e108229 * assign71630_e108229))), (-(((locals.var_beta_dn6 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn6)) / (assign71630_e108229 * assign71630_e108229))), (-(((locals.var_beta_dn7 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn7)) / (assign71630_e108229 * assign71630_e108229))), (-(((locals.var_beta_dn8 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn8)) / (assign71630_e108229 * assign71630_e108229))), (-(((locals.var_beta_dn9 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn9)) / (assign71630_e108229 * assign71630_e108229))), (-(((locals.var_beta_dn10 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn10)) / (assign71630_e108229 * assign71630_e108229))), (-(((locals.var_beta_dn11 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn11)) / (assign71630_e108229 * assign71630_e108229))), (-(((locals.var_beta_dn14 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn14)) / (assign71630_e108229 * assign71630_e108229))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn8, locals.var_tc_dn9, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn14,)
    }
};
        locals.var_tc = assign71630_e108233;
        locals.var_tc_dn0 = assign71630_e108233_d_n0;
        locals.var_tc_dn2 = assign71630_e108233_d_n2;
        locals.var_tc_dn4 = assign71630_e108233_d_n4;
        locals.var_tc_dn5 = assign71630_e108233_d_n5;
        locals.var_tc_dn6 = assign71630_e108233_d_n6;
        locals.var_tc_dn7 = assign71630_e108233_d_n7;
        locals.var_tc_dn8 = assign71630_e108233_d_n8;
        locals.var_tc_dn9 = assign71630_e108233_d_n9;
        locals.var_tc_dn10 = assign71630_e108233_d_n10;
        locals.var_tc_dn11 = assign71630_e108233_d_n11;
        locals.var_tc_dn14 = assign71630_e108233_d_n14;
        locals.var_tc_rv = 0.0;

        let (assign71640_e108248, assign71640_e108248_d_n0, assign71640_e108248_d_n2, assign71640_e108248_d_n4, assign71640_e108248_d_n5, assign71640_e108248_d_n6, assign71640_e108248_d_n7, assign71640_e108248_d_n8, assign71640_e108248_d_n9, assign71640_e108248_d_n10, assign71640_e108248_d_n11, assign71640_e108248_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 == 0.0)) {
        let assign71640_e108243: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71640_e108244: f64 = (-assign71640_e108243);
        let assign71640_e108246: f64 = (assign71640_e108244 / locals.var_fac1);
        (assign71640_e108246, ((((-locals.var_vxbgmtcl_dn0) * locals.var_fac1) - (assign71640_e108244 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign71640_e108244 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign71640_e108244 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn5) * locals.var_fac1) - (assign71640_e108244 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn6) * locals.var_fac1) - (assign71640_e108244 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1) - (assign71640_e108244 * locals.var_fac1_dn7)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)) * locals.var_fac1) - (assign71640_e108244 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)) * locals.var_fac1) - (assign71640_e108244 * locals.var_fac1_dn9)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign71640_e108244 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn11) * locals.var_fac1) - (assign71640_e108244 * locals.var_fac1_dn11)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn14) * locals.var_fac1) - (assign71640_e108244 * locals.var_fac1_dn14)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn8, locals.var_td_dn9, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn14,)
    }
};
        locals.var_td = assign71640_e108248;
        locals.var_td_dn0 = assign71640_e108248_d_n0;
        locals.var_td_dn2 = assign71640_e108248_d_n2;
        locals.var_td_dn4 = assign71640_e108248_d_n4;
        locals.var_td_dn5 = assign71640_e108248_d_n5;
        locals.var_td_dn6 = assign71640_e108248_d_n6;
        locals.var_td_dn7 = assign71640_e108248_d_n7;
        locals.var_td_dn8 = assign71640_e108248_d_n8;
        locals.var_td_dn9 = assign71640_e108248_d_n9;
        locals.var_td_dn10 = assign71640_e108248_d_n10;
        locals.var_td_dn11 = assign71640_e108248_d_n11;
        locals.var_td_dn14 = assign71640_e108248_d_n14;
        locals.var_td_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_269(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign71650_e108286, assign71650_e108286_d_n0, assign71650_e108286_d_n2, assign71650_e108286_d_n4, assign71650_e108286_d_n5, assign71650_e108286_d_n6, assign71650_e108286_d_n7, assign71650_e108286_d_n8, assign71650_e108286_d_n9, assign71650_e108286_d_n10, assign71650_e108286_d_n11, assign71650_e108286_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 == 0.0)) {
        let assign71650_e108258: f64 = (locals.var_tb * locals.var_tb);
        let assign71650_e108260: f64 = (assign71650_e108258 * locals.var_tb);
        let assign71650_e108263: f64 = (27.0 * locals.var_ta);
        let assign71650_e108265: f64 = (assign71650_e108263 * locals.var_ta);
        let assign71650_e108267: f64 = (assign71650_e108265 * locals.var_ta);
        let assign71650_e108268: f64 = (assign71650_e108260 / assign71650_e108267);
        let assign71650_e108271: f64 = (locals.var_tb * locals.var_tc);
        let assign71650_e108274: f64 = (6.0 * locals.var_ta);
        let assign71650_e108276: f64 = (assign71650_e108274 * locals.var_ta);
        let assign71650_e108277: f64 = (assign71650_e108271 / assign71650_e108276);
        let assign71650_e108278: f64 = (assign71650_e108268 - assign71650_e108277);
        let assign71650_e108282: f64 = (2.0 * locals.var_ta);
        let assign71650_e108283: f64 = (locals.var_td / assign71650_e108282);
        let assign71650_e108284: f64 = (assign71650_e108278 + assign71650_e108283);
        (assign71650_e108284, ((-((locals.var_tb * locals.var_tc_dn0) / assign71650_e108276)) + (locals.var_td_dn0 / assign71650_e108282)), ((-((locals.var_tb * locals.var_tc_dn2) / assign71650_e108276)) + (locals.var_td_dn2 / assign71650_e108282)), ((-((locals.var_tb * locals.var_tc_dn4) / assign71650_e108276)) + (locals.var_td_dn4 / assign71650_e108282)), ((-((locals.var_tb * locals.var_tc_dn5) / assign71650_e108276)) + (locals.var_td_dn5 / assign71650_e108282)), ((-((locals.var_tb * locals.var_tc_dn6) / assign71650_e108276)) + (locals.var_td_dn6 / assign71650_e108282)), ((-((locals.var_tb * locals.var_tc_dn7) / assign71650_e108276)) + (locals.var_td_dn7 / assign71650_e108282)), ((-((locals.var_tb * locals.var_tc_dn8) / assign71650_e108276)) + (locals.var_td_dn8 / assign71650_e108282)), ((-((locals.var_tb * locals.var_tc_dn9) / assign71650_e108276)) + (locals.var_td_dn9 / assign71650_e108282)), ((-((locals.var_tb * locals.var_tc_dn10) / assign71650_e108276)) + (locals.var_td_dn10 / assign71650_e108282)), ((-((locals.var_tb * locals.var_tc_dn11) / assign71650_e108276)) + (locals.var_td_dn11 / assign71650_e108282)), ((-((locals.var_tb * locals.var_tc_dn14) / assign71650_e108276)) + (locals.var_td_dn14 / assign71650_e108282)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn8, locals.var_tq_dn9, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn14,)
    }
};
        locals.var_tq = assign71650_e108286;
        locals.var_tq_dn0 = assign71650_e108286_d_n0;
        locals.var_tq_dn2 = assign71650_e108286_d_n2;
        locals.var_tq_dn4 = assign71650_e108286_d_n4;
        locals.var_tq_dn5 = assign71650_e108286_d_n5;
        locals.var_tq_dn6 = assign71650_e108286_d_n6;
        locals.var_tq_dn7 = assign71650_e108286_d_n7;
        locals.var_tq_dn8 = assign71650_e108286_d_n8;
        locals.var_tq_dn9 = assign71650_e108286_d_n9;
        locals.var_tq_dn10 = assign71650_e108286_d_n10;
        locals.var_tq_dn11 = assign71650_e108286_d_n11;
        locals.var_tq_dn14 = assign71650_e108286_d_n14;
        locals.var_tq_rv = 0.0;

        let (assign71660_e108310, assign71660_e108310_d_n0, assign71660_e108310_d_n2, assign71660_e108310_d_n4, assign71660_e108310_d_n5, assign71660_e108310_d_n6, assign71660_e108310_d_n7, assign71660_e108310_d_n8, assign71660_e108310_d_n9, assign71660_e108310_d_n10, assign71660_e108310_d_n11, assign71660_e108310_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 == 0.0)) {
        let assign71660_e108296: f64 = (3.0 * locals.var_ta);
        let assign71660_e108298: f64 = (assign71660_e108296 * locals.var_tc);
        let assign71660_e108301: f64 = (locals.var_tb * locals.var_tb);
        let assign71660_e108302: f64 = (assign71660_e108298 - assign71660_e108301);
        let assign71660_e108305: f64 = (9.0 * locals.var_ta);
        let assign71660_e108307: f64 = (assign71660_e108305 * locals.var_ta);
        let assign71660_e108308: f64 = (assign71660_e108302 / assign71660_e108307);
        (assign71660_e108308, ((assign71660_e108296 * locals.var_tc_dn0) / assign71660_e108307), ((assign71660_e108296 * locals.var_tc_dn2) / assign71660_e108307), ((assign71660_e108296 * locals.var_tc_dn4) / assign71660_e108307), ((assign71660_e108296 * locals.var_tc_dn5) / assign71660_e108307), ((assign71660_e108296 * locals.var_tc_dn6) / assign71660_e108307), ((assign71660_e108296 * locals.var_tc_dn7) / assign71660_e108307), ((assign71660_e108296 * locals.var_tc_dn8) / assign71660_e108307), ((assign71660_e108296 * locals.var_tc_dn9) / assign71660_e108307), ((assign71660_e108296 * locals.var_tc_dn10) / assign71660_e108307), ((assign71660_e108296 * locals.var_tc_dn11) / assign71660_e108307), ((assign71660_e108296 * locals.var_tc_dn14) / assign71660_e108307),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn14,)
    }
};
        locals.var_tp = assign71660_e108310;
        locals.var_tp_dn0 = assign71660_e108310_d_n0;
        locals.var_tp_dn2 = assign71660_e108310_d_n2;
        locals.var_tp_dn4 = assign71660_e108310_d_n4;
        locals.var_tp_dn5 = assign71660_e108310_d_n5;
        locals.var_tp_dn6 = assign71660_e108310_d_n6;
        locals.var_tp_dn7 = assign71660_e108310_d_n7;
        locals.var_tp_dn8 = assign71660_e108310_d_n8;
        locals.var_tp_dn9 = assign71660_e108310_d_n9;
        locals.var_tp_dn10 = assign71660_e108310_d_n10;
        locals.var_tp_dn11 = assign71660_e108310_d_n11;
        locals.var_tp_dn14 = assign71660_e108310_d_n14;
        locals.var_tp_rv = 0.0;

        let (assign71670_e108329, assign71670_e108329_d_n0, assign71670_e108329_d_n2, assign71670_e108329_d_n4, assign71670_e108329_d_n5, assign71670_e108329_d_n6, assign71670_e108329_d_n7, assign71670_e108329_d_n8, assign71670_e108329_d_n9, assign71670_e108329_d_n10, assign71670_e108329_d_n11, assign71670_e108329_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 == 0.0)) {
        let assign71670_e108320: f64 = (locals.var_tq * locals.var_tq);
        let assign71670_e108323: f64 = (locals.var_tp * locals.var_tp);
        let assign71670_e108325: f64 = (assign71670_e108323 * locals.var_tp);
        let assign71670_e108326: f64 = (assign71670_e108320 + assign71670_e108325);
        let assign71670_e108327: f64 = (assign71670_e108326).sqrt();
        (assign71670_e108327, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign71670_e108323 * locals.var_tp_dn0))) / (2.0 * assign71670_e108327)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign71670_e108323 * locals.var_tp_dn2))) / (2.0 * assign71670_e108327)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign71670_e108323 * locals.var_tp_dn4))) / (2.0 * assign71670_e108327)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign71670_e108323 * locals.var_tp_dn5))) / (2.0 * assign71670_e108327)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign71670_e108323 * locals.var_tp_dn6))) / (2.0 * assign71670_e108327)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign71670_e108323 * locals.var_tp_dn7))) / (2.0 * assign71670_e108327)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign71670_e108323 * locals.var_tp_dn8))) / (2.0 * assign71670_e108327)), ((((locals.var_tq_dn9 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn9)) + ((((locals.var_tp_dn9 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn9)) * locals.var_tp) + (assign71670_e108323 * locals.var_tp_dn9))) / (2.0 * assign71670_e108327)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign71670_e108323 * locals.var_tp_dn10))) / (2.0 * assign71670_e108327)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign71670_e108323 * locals.var_tp_dn11))) / (2.0 * assign71670_e108327)), ((((locals.var_tq_dn14 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn14)) + ((((locals.var_tp_dn14 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn14)) * locals.var_tp) + (assign71670_e108323 * locals.var_tp_dn14))) / (2.0 * assign71670_e108327)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign71670_e108329;
        locals.var_t5_dn0 = assign71670_e108329_d_n0;
        locals.var_t5_dn2 = assign71670_e108329_d_n2;
        locals.var_t5_dn4 = assign71670_e108329_d_n4;
        locals.var_t5_dn5 = assign71670_e108329_d_n5;
        locals.var_t5_dn6 = assign71670_e108329_d_n6;
        locals.var_t5_dn7 = assign71670_e108329_d_n7;
        locals.var_t5_dn8 = assign71670_e108329_d_n8;
        locals.var_t5_dn9 = assign71670_e108329_d_n9;
        locals.var_t5_dn10 = assign71670_e108329_d_n10;
        locals.var_t5_dn11 = assign71670_e108329_d_n11;
        locals.var_t5_dn14 = assign71670_e108329_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign71680_e108344, assign71680_e108344_d_n0, assign71680_e108344_d_n2, assign71680_e108344_d_n4, assign71680_e108344_d_n5, assign71680_e108344_d_n6, assign71680_e108344_d_n7, assign71680_e108344_d_n8, assign71680_e108344_d_n9, assign71680_e108344_d_n10, assign71680_e108344_d_n11, assign71680_e108344_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 == 0.0)) {
        let assign71680_e108338: f64 = (-locals.var_tq);
        let assign71680_e108340: f64 = (assign71680_e108338 + locals.var_t5);
        let assign71680_e108342: f64 = (assign71680_e108340).powf(0.3333333333333333);
        (assign71680_e108342, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71680_e108340).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign71680_e108342 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign71680_e108340))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71680_e108340).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign71680_e108342 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign71680_e108340))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71680_e108340).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign71680_e108342 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign71680_e108340))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71680_e108340).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign71680_e108342 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign71680_e108340))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71680_e108340).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign71680_e108342 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign71680_e108340))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71680_e108340).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5_dn7))) } } else { (assign71680_e108342 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5_dn7) / assign71680_e108340))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71680_e108340).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign71680_e108342 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign71680_e108340))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71680_e108340).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn9) + locals.var_t5_dn9))) } } else { (assign71680_e108342 * (0.3333333333333333 * (((-locals.var_tq_dn9) + locals.var_t5_dn9) / assign71680_e108340))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71680_e108340).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign71680_e108342 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign71680_e108340))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71680_e108340).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5_dn11))) } } else { (assign71680_e108342 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5_dn11) / assign71680_e108340))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71680_e108340).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn14) + locals.var_t5_dn14))) } } else { (assign71680_e108342 * (0.3333333333333333 * (((-locals.var_tq_dn14) + locals.var_t5_dn14) / assign71680_e108340))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn8, locals.var_tu_dn9, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn14,)
    }
};
        locals.var_tu = assign71680_e108344;
        locals.var_tu_dn0 = assign71680_e108344_d_n0;
        locals.var_tu_dn2 = assign71680_e108344_d_n2;
        locals.var_tu_dn4 = assign71680_e108344_d_n4;
        locals.var_tu_dn5 = assign71680_e108344_d_n5;
        locals.var_tu_dn6 = assign71680_e108344_d_n6;
        locals.var_tu_dn7 = assign71680_e108344_d_n7;
        locals.var_tu_dn8 = assign71680_e108344_d_n8;
        locals.var_tu_dn9 = assign71680_e108344_d_n9;
        locals.var_tu_dn10 = assign71680_e108344_d_n10;
        locals.var_tu_dn11 = assign71680_e108344_d_n11;
        locals.var_tu_dn14 = assign71680_e108344_d_n14;
        locals.var_tu_rv = 0.0;

        let (assign71690_e108359, assign71690_e108359_d_n0, assign71690_e108359_d_n2, assign71690_e108359_d_n4, assign71690_e108359_d_n5, assign71690_e108359_d_n6, assign71690_e108359_d_n7, assign71690_e108359_d_n8, assign71690_e108359_d_n9, assign71690_e108359_d_n10, assign71690_e108359_d_n11, assign71690_e108359_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 == 0.0)) {
        let assign71690_e108354: f64 = (locals.var_tq + locals.var_t5);
        let assign71690_e108356: f64 = (assign71690_e108354).powf(0.3333333333333333);
        let assign71690_e108357: f64 = (-assign71690_e108356);
        (assign71690_e108357, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71690_e108354).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign71690_e108356 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign71690_e108354))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71690_e108354).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign71690_e108356 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign71690_e108354))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71690_e108354).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign71690_e108356 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign71690_e108354))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71690_e108354).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign71690_e108356 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign71690_e108354))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71690_e108354).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign71690_e108356 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign71690_e108354))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71690_e108354).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5_dn7))) } } else { (assign71690_e108356 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5_dn7) / assign71690_e108354))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71690_e108354).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign71690_e108356 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign71690_e108354))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71690_e108354).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn9 + locals.var_t5_dn9))) } } else { (assign71690_e108356 * (0.3333333333333333 * ((locals.var_tq_dn9 + locals.var_t5_dn9) / assign71690_e108354))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71690_e108354).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign71690_e108356 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign71690_e108354))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71690_e108354).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5_dn11))) } } else { (assign71690_e108356 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5_dn11) / assign71690_e108354))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign71690_e108354).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn14 + locals.var_t5_dn14))) } } else { (assign71690_e108356 * (0.3333333333333333 * ((locals.var_tq_dn14 + locals.var_t5_dn14) / assign71690_e108354))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn8, locals.var_tv_dn9, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn14,)
    }
};
        locals.var_tv = assign71690_e108359;
        locals.var_tv_dn0 = assign71690_e108359_d_n0;
        locals.var_tv_dn2 = assign71690_e108359_d_n2;
        locals.var_tv_dn4 = assign71690_e108359_d_n4;
        locals.var_tv_dn5 = assign71690_e108359_d_n5;
        locals.var_tv_dn6 = assign71690_e108359_d_n6;
        locals.var_tv_dn7 = assign71690_e108359_d_n7;
        locals.var_tv_dn8 = assign71690_e108359_d_n8;
        locals.var_tv_dn9 = assign71690_e108359_d_n9;
        locals.var_tv_dn10 = assign71690_e108359_d_n10;
        locals.var_tv_dn11 = assign71690_e108359_d_n11;
        locals.var_tv_dn14 = assign71690_e108359_d_n14;
        locals.var_tv_rv = 0.0;

        let (assign71700_e108377, assign71700_e108377_d_n0, assign71700_e108377_d_n2, assign71700_e108377_d_n4, assign71700_e108377_d_n5, assign71700_e108377_d_n6, assign71700_e108377_d_n7, assign71700_e108377_d_n8, assign71700_e108377_d_n9, assign71700_e108377_d_n10, assign71700_e108377_d_n11, assign71700_e108377_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 == 0.0)) {
        let assign71700_e108369: f64 = (locals.var_tu + locals.var_tv);
        let assign71700_e108373: f64 = (3.0 * locals.var_ta);
        let assign71700_e108374: f64 = (locals.var_tb / assign71700_e108373);
        let assign71700_e108375: f64 = (assign71700_e108369 - assign71700_e108374);
        (assign71700_e108375, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn9 + locals.var_tv_dn9), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn14 + locals.var_tv_dn14),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign71700_e108377;
        locals.var_chi_dn0 = assign71700_e108377_d_n0;
        locals.var_chi_dn2 = assign71700_e108377_d_n2;
        locals.var_chi_dn4 = assign71700_e108377_d_n4;
        locals.var_chi_dn5 = assign71700_e108377_d_n5;
        locals.var_chi_dn6 = assign71700_e108377_d_n6;
        locals.var_chi_dn7 = assign71700_e108377_d_n7;
        locals.var_chi_dn8 = assign71700_e108377_d_n8;
        locals.var_chi_dn9 = assign71700_e108377_d_n9;
        locals.var_chi_dn10 = assign71700_e108377_d_n10;
        locals.var_chi_dn11 = assign71700_e108377_d_n11;
        locals.var_chi_dn14 = assign71700_e108377_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign71710_e108391, assign71710_e108391_d_n0, assign71710_e108391_d_n2, assign71710_e108391_d_n4, assign71710_e108391_d_n5, assign71710_e108391_d_n6, assign71710_e108391_d_n7, assign71710_e108391_d_n8, assign71710_e108391_d_n9, assign71710_e108391_d_n10, assign71710_e108391_d_n11, assign71710_e108391_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1670 == 0.0)) {
        let assign71710_e108387: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign71710_e108389: f64 = (assign71710_e108387 - locals.var_vxbgmtcl);
        (assign71710_e108389, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign71710_e108391;
        locals.var_ps0_inia_dn0 = assign71710_e108391_d_n0;
        locals.var_ps0_inia_dn2 = assign71710_e108391_d_n2;
        locals.var_ps0_inia_dn4 = assign71710_e108391_d_n4;
        locals.var_ps0_inia_dn5 = assign71710_e108391_d_n5;
        locals.var_ps0_inia_dn6 = assign71710_e108391_d_n6;
        locals.var_ps0_inia_dn7 = assign71710_e108391_d_n7;
        locals.var_ps0_inia_dn8 = assign71710_e108391_d_n8;
        locals.var_ps0_inia_dn9 = assign71710_e108391_d_n9;
        locals.var_ps0_inia_dn10 = assign71710_e108391_d_n10;
        locals.var_ps0_inia_dn11 = assign71710_e108391_d_n11;
        locals.var_ps0_inia_dn14 = assign71710_e108391_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let assign71720_e108394: f64 = if p.p33 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1671 = assign71720_e108394;
        locals.var_guard1671_rv = 0.0;

        let (assign71730_e108407, assign71730_e108407_d_n0, assign71730_e108407_d_n2, assign71730_e108407_d_n4, assign71730_e108407_d_n5, assign71730_e108407_d_n6, assign71730_e108407_d_n7, assign71730_e108407_d_n8, assign71730_e108407_d_n9, assign71730_e108407_d_n10, assign71730_e108407_d_n11, assign71730_e108407_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) {
        let assign71730_e108403: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign71730_e108405: f64 = (assign71730_e108403 + 0.1);
        (assign71730_e108405, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn9, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn14,)
    }
};
        locals.var_vgpld_shift = assign71730_e108407;
        locals.var_vgpld_shift_dn0 = assign71730_e108407_d_n0;
        locals.var_vgpld_shift_dn2 = assign71730_e108407_d_n2;
        locals.var_vgpld_shift_dn4 = assign71730_e108407_d_n4;
        locals.var_vgpld_shift_dn5 = assign71730_e108407_d_n5;
        locals.var_vgpld_shift_dn6 = assign71730_e108407_d_n6;
        locals.var_vgpld_shift_dn7 = assign71730_e108407_d_n7;
        locals.var_vgpld_shift_dn8 = assign71730_e108407_d_n8;
        locals.var_vgpld_shift_dn9 = assign71730_e108407_d_n9;
        locals.var_vgpld_shift_dn10 = assign71730_e108407_d_n10;
        locals.var_vgpld_shift_dn11 = assign71730_e108407_d_n11;
        locals.var_vgpld_shift_dn14 = assign71730_e108407_d_n14;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign71740_e108418, assign71740_e108418_d_n0, assign71740_e108418_d_n2, assign71740_e108418_d_n4, assign71740_e108418_d_n5, assign71740_e108418_d_n6, assign71740_e108418_d_n7, assign71740_e108418_d_n8, assign71740_e108418_d_n9, assign71740_e108418_d_n10, assign71740_e108418_d_n11, assign71740_e108418_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) {
        let assign71740_e108416: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign71740_e108416, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn14,)
    }
};
        locals.var_cfs1 = assign71740_e108418;
        locals.var_cfs1_dn0 = assign71740_e108418_d_n0;
        locals.var_cfs1_dn2 = assign71740_e108418_d_n2;
        locals.var_cfs1_dn4 = assign71740_e108418_d_n4;
        locals.var_cfs1_dn5 = assign71740_e108418_d_n5;
        locals.var_cfs1_dn6 = assign71740_e108418_d_n6;
        locals.var_cfs1_dn7 = assign71740_e108418_d_n7;
        locals.var_cfs1_dn8 = assign71740_e108418_d_n8;
        locals.var_cfs1_dn9 = assign71740_e108418_d_n9;
        locals.var_cfs1_dn10 = assign71740_e108418_d_n10;
        locals.var_cfs1_dn11 = assign71740_e108418_d_n11;
        locals.var_cfs1_dn14 = assign71740_e108418_d_n14;
        locals.var_cfs1_rv = 0.0;

        let (assign71750_e108429, assign71750_e108429_d_n0, assign71750_e108429_d_n2, assign71750_e108429_d_n4, assign71750_e108429_d_n5, assign71750_e108429_d_n6, assign71750_e108429_d_n7, assign71750_e108429_d_n8, assign71750_e108429_d_n9, assign71750_e108429_d_n10, assign71750_e108429_d_n11, assign71750_e108429_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) {
        let assign71750_e108427: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign71750_e108427, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn8, locals.var_gammachi_dn9, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn14,)
    }
};
        locals.var_gammachi = assign71750_e108429;
        locals.var_gammachi_dn0 = assign71750_e108429_d_n0;
        locals.var_gammachi_dn2 = assign71750_e108429_d_n2;
        locals.var_gammachi_dn4 = assign71750_e108429_d_n4;
        locals.var_gammachi_dn5 = assign71750_e108429_d_n5;
        locals.var_gammachi_dn6 = assign71750_e108429_d_n6;
        locals.var_gammachi_dn7 = assign71750_e108429_d_n7;
        locals.var_gammachi_dn8 = assign71750_e108429_d_n8;
        locals.var_gammachi_dn9 = assign71750_e108429_d_n9;
        locals.var_gammachi_dn10 = assign71750_e108429_d_n10;
        locals.var_gammachi_dn11 = assign71750_e108429_d_n11;
        locals.var_gammachi_dn14 = assign71750_e108429_d_n14;
        locals.var_gammachi_rv = 0.0;

        let (assign71760_e108440, assign71760_e108440_d_n0, assign71760_e108440_d_n2, assign71760_e108440_d_n4, assign71760_e108440_d_n5, assign71760_e108440_d_n6, assign71760_e108440_d_n7, assign71760_e108440_d_n8, assign71760_e108440_d_n9, assign71760_e108440_d_n10, assign71760_e108440_d_n11, assign71760_e108440_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) {
        let assign71760_e108438: f64 = (locals.var_beta2 * locals.var_fac1p2);
        (assign71760_e108438, ((locals.var_beta2_dn0 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn0)), ((locals.var_beta2_dn2 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn2)), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), ((locals.var_beta2_dn5 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn5)), ((locals.var_beta2_dn6 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn6)), ((locals.var_beta2_dn7 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn7)), ((locals.var_beta2_dn8 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn8)), ((locals.var_beta2_dn9 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn9)), ((locals.var_beta2_dn10 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn10)), ((locals.var_beta2_dn11 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn11)), ((locals.var_beta2_dn14 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign71760_e108440;
        locals.var_t0_dn0 = assign71760_e108440_d_n0;
        locals.var_t0_dn2 = assign71760_e108440_d_n2;
        locals.var_t0_dn4 = assign71760_e108440_d_n4;
        locals.var_t0_dn5 = assign71760_e108440_d_n5;
        locals.var_t0_dn6 = assign71760_e108440_d_n6;
        locals.var_t0_dn7 = assign71760_e108440_d_n7;
        locals.var_t0_dn8 = assign71760_e108440_d_n8;
        locals.var_t0_dn9 = assign71760_e108440_d_n9;
        locals.var_t0_dn10 = assign71760_e108440_d_n10;
        locals.var_t0_dn11 = assign71760_e108440_d_n11;
        locals.var_t0_dn14 = assign71760_e108440_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign71770_e108451, assign71770_e108451_d_n0, assign71770_e108451_d_n2, assign71770_e108451_d_n4, assign71770_e108451_d_n5, assign71770_e108451_d_n6, assign71770_e108451_d_n7, assign71770_e108451_d_n8, assign71770_e108451_d_n9, assign71770_e108451_d_n10, assign71770_e108451_d_n11, assign71770_e108451_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) {
        let assign71770_e108449: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign71770_e108449, ((locals.var_beta_dn0 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn0)), ((locals.var_beta_dn2 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn2)), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), ((locals.var_beta_dn5 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn5)), ((locals.var_beta_dn6 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn6)), ((locals.var_beta_dn7 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn7)), ((locals.var_beta_dn8 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn8)), ((locals.var_beta_dn9 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn9)), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), ((locals.var_beta_dn11 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn11)), ((locals.var_beta_dn14 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn14)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign71770_e108451;
        locals.var_psi_dn0 = assign71770_e108451_d_n0;
        locals.var_psi_dn2 = assign71770_e108451_d_n2;
        locals.var_psi_dn4 = assign71770_e108451_d_n4;
        locals.var_psi_dn5 = assign71770_e108451_d_n5;
        locals.var_psi_dn6 = assign71770_e108451_d_n6;
        locals.var_psi_dn7 = assign71770_e108451_d_n7;
        locals.var_psi_dn8 = assign71770_e108451_d_n8;
        locals.var_psi_dn9 = assign71770_e108451_d_n9;
        locals.var_psi_dn10 = assign71770_e108451_d_n10;
        locals.var_psi_dn11 = assign71770_e108451_d_n11;
        locals.var_psi_dn14 = assign71770_e108451_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign71780_e108476, assign71780_e108476_d_n0, assign71780_e108476_d_n2, assign71780_e108476_d_n4, assign71780_e108476_d_n5, assign71780_e108476_d_n6, assign71780_e108476_d_n7, assign71780_e108476_d_n8, assign71780_e108476_d_n9, assign71780_e108476_d_n10, assign71780_e108476_d_n11, assign71780_e108476_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) {
        let assign71780_e108460: f64 = (locals.var_gammachi * locals.var_t0);
        let assign71780_e108463: f64 = (locals.var_psi * locals.var_psi);
        let assign71780_e108464: f64 = (assign71780_e108460 + assign71780_e108463);
        let assign71780_e108465: f64 = (assign71780_e108464).ln();
        let assign71780_e108468: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign71780_e108469: f64 = (assign71780_e108468).ln();
        let assign71780_e108470: f64 = (assign71780_e108465 - assign71780_e108469);
        let assign71780_e108473: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign71780_e108474: f64 = (assign71780_e108470 + assign71780_e108473);
        (assign71780_e108474, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign71780_e108464) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign71780_e108468)) + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign71780_e108464) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign71780_e108468)) + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign71780_e108464) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign71780_e108468)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign71780_e108464) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign71780_e108468)) + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign71780_e108464) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign71780_e108468)) + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), ((((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign71780_e108464) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign71780_e108468)) + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign71780_e108464) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign71780_e108468)) + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), ((((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign71780_e108464) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign71780_e108468)) + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign71780_e108464) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign71780_e108468)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign71780_e108464) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign71780_e108468)) + ((locals.var_beta_dn11 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn11))), ((((((locals.var_gammachi_dn14 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn14)) + ((locals.var_psi_dn14 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn14))) / assign71780_e108464) - (((locals.var_cnst1over_dn14 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn14)) / assign71780_e108468)) + ((locals.var_beta_dn14 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign71780_e108476;
        locals.var_chi_1_dn0 = assign71780_e108476_d_n0;
        locals.var_chi_1_dn2 = assign71780_e108476_d_n2;
        locals.var_chi_1_dn4 = assign71780_e108476_d_n4;
        locals.var_chi_1_dn5 = assign71780_e108476_d_n5;
        locals.var_chi_1_dn6 = assign71780_e108476_d_n6;
        locals.var_chi_1_dn7 = assign71780_e108476_d_n7;
        locals.var_chi_1_dn8 = assign71780_e108476_d_n8;
        locals.var_chi_1_dn9 = assign71780_e108476_d_n9;
        locals.var_chi_1_dn10 = assign71780_e108476_d_n10;
        locals.var_chi_1_dn11 = assign71780_e108476_d_n11;
        locals.var_chi_1_dn14 = assign71780_e108476_d_n14;
        locals.var_chi_1_rv = 0.0;

        let assign71790_e108479: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1672 = assign71790_e108479;
        locals.var_guard1672_rv = 0.0;

        let (assign71800_e108494, assign71800_e108494_d_n0, assign71800_e108494_d_n2, assign71800_e108494_d_n4, assign71800_e108494_d_n5, assign71800_e108494_d_n6, assign71800_e108494_d_n7, assign71800_e108494_d_n8, assign71800_e108494_d_n9, assign71800_e108494_d_n10, assign71800_e108494_d_n11, assign71800_e108494_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign71800_e108490: f64 = (locals.var_psi - locals.var_chi_1);
        let assign71800_e108492: f64 = (assign71800_e108490 - 1.0);
        (assign71800_e108492, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn14 - locals.var_chi_1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign71800_e108494;
        locals.var_tmf1_dn0 = assign71800_e108494_d_n0;
        locals.var_tmf1_dn2 = assign71800_e108494_d_n2;
        locals.var_tmf1_dn4 = assign71800_e108494_d_n4;
        locals.var_tmf1_dn5 = assign71800_e108494_d_n5;
        locals.var_tmf1_dn6 = assign71800_e108494_d_n6;
        locals.var_tmf1_dn7 = assign71800_e108494_d_n7;
        locals.var_tmf1_dn8 = assign71800_e108494_d_n8;
        locals.var_tmf1_dn9 = assign71800_e108494_d_n9;
        locals.var_tmf1_dn10 = assign71800_e108494_d_n10;
        locals.var_tmf1_dn11 = assign71800_e108494_d_n11;
        locals.var_tmf1_dn14 = assign71800_e108494_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign71810_e108509, assign71810_e108509_d_n0, assign71810_e108509_d_n2, assign71810_e108509_d_n4, assign71810_e108509_d_n5, assign71810_e108509_d_n6, assign71810_e108509_d_n7, assign71810_e108509_d_n8, assign71810_e108509_d_n9, assign71810_e108509_d_n10, assign71810_e108509_d_n11, assign71810_e108509_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign71810_e108505: f64 = (4.0 * locals.var_psi);
        let assign71810_e108507: f64 = assign71810_e108505;
        (assign71810_e108507, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn9), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn14),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign71810_e108509;
        locals.var_tmf2_dn0 = assign71810_e108509_d_n0;
        locals.var_tmf2_dn2 = assign71810_e108509_d_n2;
        locals.var_tmf2_dn4 = assign71810_e108509_d_n4;
        locals.var_tmf2_dn5 = assign71810_e108509_d_n5;
        locals.var_tmf2_dn6 = assign71810_e108509_d_n6;
        locals.var_tmf2_dn7 = assign71810_e108509_d_n7;
        locals.var_tmf2_dn8 = assign71810_e108509_d_n8;
        locals.var_tmf2_dn9 = assign71810_e108509_d_n9;
        locals.var_tmf2_dn10 = assign71810_e108509_d_n10;
        locals.var_tmf2_dn11 = assign71810_e108509_d_n11;
        locals.var_tmf2_dn14 = assign71810_e108509_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign71820_e108526, assign71820_e108526_d_n0, assign71820_e108526_d_n2, assign71820_e108526_d_n4, assign71820_e108526_d_n5, assign71820_e108526_d_n6, assign71820_e108526_d_n7, assign71820_e108526_d_n8, assign71820_e108526_d_n9, assign71820_e108526_d_n10, assign71820_e108526_d_n11, assign71820_e108526_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let (assign71820_e108524, assign71820_e108524_d_n0, assign71820_e108524_d_n2, assign71820_e108524_d_n4, assign71820_e108524_d_n5, assign71820_e108524_d_n6, assign71820_e108524_d_n7, assign71820_e108524_d_n8, assign71820_e108524_d_n9, assign71820_e108524_d_n10, assign71820_e108524_d_n11, assign71820_e108524_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign71820_e108523: f64 = (-locals.var_tmf2);
                (assign71820_e108523, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign71820_e108524, assign71820_e108524_d_n0, assign71820_e108524_d_n2, assign71820_e108524_d_n4, assign71820_e108524_d_n5, assign71820_e108524_d_n6, assign71820_e108524_d_n7, assign71820_e108524_d_n8, assign71820_e108524_d_n9, assign71820_e108524_d_n10, assign71820_e108524_d_n11, assign71820_e108524_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign71820_e108526;
        locals.var_tmf2_dn0 = assign71820_e108526_d_n0;
        locals.var_tmf2_dn2 = assign71820_e108526_d_n2;
        locals.var_tmf2_dn4 = assign71820_e108526_d_n4;
        locals.var_tmf2_dn5 = assign71820_e108526_d_n5;
        locals.var_tmf2_dn6 = assign71820_e108526_d_n6;
        locals.var_tmf2_dn7 = assign71820_e108526_d_n7;
        locals.var_tmf2_dn8 = assign71820_e108526_d_n8;
        locals.var_tmf2_dn9 = assign71820_e108526_d_n9;
        locals.var_tmf2_dn10 = assign71820_e108526_d_n10;
        locals.var_tmf2_dn11 = assign71820_e108526_d_n11;
        locals.var_tmf2_dn14 = assign71820_e108526_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign71830_e108542, assign71830_e108542_d_n0, assign71830_e108542_d_n2, assign71830_e108542_d_n4, assign71830_e108542_d_n5, assign71830_e108542_d_n6, assign71830_e108542_d_n7, assign71830_e108542_d_n8, assign71830_e108542_d_n9, assign71830_e108542_d_n10, assign71830_e108542_d_n11, assign71830_e108542_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign71830_e108537: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign71830_e108539: f64 = (assign71830_e108537 + locals.var_tmf2);
        let assign71830_e108540: f64 = (assign71830_e108539).sqrt();
        (assign71830_e108540, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign71830_e108540)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign71830_e108540)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign71830_e108540)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign71830_e108540)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign71830_e108540)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign71830_e108540)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign71830_e108540)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign71830_e108540)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign71830_e108540)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign71830_e108540)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign71830_e108540)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign71830_e108542;
        locals.var_tmf2_dn0 = assign71830_e108542_d_n0;
        locals.var_tmf2_dn2 = assign71830_e108542_d_n2;
        locals.var_tmf2_dn4 = assign71830_e108542_d_n4;
        locals.var_tmf2_dn5 = assign71830_e108542_d_n5;
        locals.var_tmf2_dn6 = assign71830_e108542_d_n6;
        locals.var_tmf2_dn7 = assign71830_e108542_d_n7;
        locals.var_tmf2_dn8 = assign71830_e108542_d_n8;
        locals.var_tmf2_dn9 = assign71830_e108542_d_n9;
        locals.var_tmf2_dn10 = assign71830_e108542_d_n10;
        locals.var_tmf2_dn11 = assign71830_e108542_d_n11;
        locals.var_tmf2_dn14 = assign71830_e108542_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign71840_e108559, assign71840_e108559_d_n0, assign71840_e108559_d_n2, assign71840_e108559_d_n4, assign71840_e108559_d_n5, assign71840_e108559_d_n6, assign71840_e108559_d_n7, assign71840_e108559_d_n8, assign71840_e108559_d_n9, assign71840_e108559_d_n10, assign71840_e108559_d_n11, assign71840_e108559_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign71840_e108555: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign71840_e108556: f64 = (1.0 + assign71840_e108555);
        let assign71840_e108557: f64 = (0.5 * assign71840_e108556);
        (assign71840_e108557, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign71840_e108559;
        locals.var_t1_dn0 = assign71840_e108559_d_n0;
        locals.var_t1_dn2 = assign71840_e108559_d_n2;
        locals.var_t1_dn4 = assign71840_e108559_d_n4;
        locals.var_t1_dn5 = assign71840_e108559_d_n5;
        locals.var_t1_dn6 = assign71840_e108559_d_n6;
        locals.var_t1_dn7 = assign71840_e108559_d_n7;
        locals.var_t1_dn8 = assign71840_e108559_d_n8;
        locals.var_t1_dn9 = assign71840_e108559_d_n9;
        locals.var_t1_dn10 = assign71840_e108559_d_n10;
        locals.var_t1_dn11 = assign71840_e108559_d_n11;
        locals.var_t1_dn14 = assign71840_e108559_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign71850_e108576, assign71850_e108576_d_n0, assign71850_e108576_d_n2, assign71850_e108576_d_n4, assign71850_e108576_d_n5, assign71850_e108576_d_n6, assign71850_e108576_d_n7, assign71850_e108576_d_n8, assign71850_e108576_d_n9, assign71850_e108576_d_n10, assign71850_e108576_d_n11, assign71850_e108576_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 != 0.0)) {
        let assign71850_e108572: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign71850_e108573: f64 = (0.5 * assign71850_e108572);
        let assign71850_e108574: f64 = (locals.var_psi - assign71850_e108573);
        (assign71850_e108574, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign71850_e108576;
        locals.var_chi_1_dn0 = assign71850_e108576_d_n0;
        locals.var_chi_1_dn2 = assign71850_e108576_d_n2;
        locals.var_chi_1_dn4 = assign71850_e108576_d_n4;
        locals.var_chi_1_dn5 = assign71850_e108576_d_n5;
        locals.var_chi_1_dn6 = assign71850_e108576_d_n6;
        locals.var_chi_1_dn7 = assign71850_e108576_d_n7;
        locals.var_chi_1_dn8 = assign71850_e108576_d_n8;
        locals.var_chi_1_dn9 = assign71850_e108576_d_n9;
        locals.var_chi_1_dn10 = assign71850_e108576_d_n10;
        locals.var_chi_1_dn11 = assign71850_e108576_d_n11;
        locals.var_chi_1_dn14 = assign71850_e108576_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign71860_e108593, assign71860_e108593_d_n0, assign71860_e108593_d_n2, assign71860_e108593_d_n4, assign71860_e108593_d_n5, assign71860_e108593_d_n6, assign71860_e108593_d_n7, assign71860_e108593_d_n8, assign71860_e108593_d_n9, assign71860_e108593_d_n10, assign71860_e108593_d_n11, assign71860_e108593_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1672 == 0.0)) {
        let (assign71860_e108591, assign71860_e108591_d_n0, assign71860_e108591_d_n2, assign71860_e108591_d_n4, assign71860_e108591_d_n5, assign71860_e108591_d_n6, assign71860_e108591_d_n7, assign71860_e108591_d_n8, assign71860_e108591_d_n9, assign71860_e108591_d_n10, assign71860_e108591_d_n11, assign71860_e108591_d_n14,) = {
            if (locals.var_chi_1 <= locals.var_psi) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
            } else {
                (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
            }
        };
        (assign71860_e108591, assign71860_e108591_d_n0, assign71860_e108591_d_n2, assign71860_e108591_d_n4, assign71860_e108591_d_n5, assign71860_e108591_d_n6, assign71860_e108591_d_n7, assign71860_e108591_d_n8, assign71860_e108591_d_n9, assign71860_e108591_d_n10, assign71860_e108591_d_n11, assign71860_e108591_d_n14,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign71860_e108593;
        locals.var_chi_1_dn0 = assign71860_e108593_d_n0;
        locals.var_chi_1_dn2 = assign71860_e108593_d_n2;
        locals.var_chi_1_dn4 = assign71860_e108593_d_n4;
        locals.var_chi_1_dn5 = assign71860_e108593_d_n5;
        locals.var_chi_1_dn6 = assign71860_e108593_d_n6;
        locals.var_chi_1_dn7 = assign71860_e108593_d_n7;
        locals.var_chi_1_dn8 = assign71860_e108593_d_n8;
        locals.var_chi_1_dn9 = assign71860_e108593_d_n9;
        locals.var_chi_1_dn10 = assign71860_e108593_d_n10;
        locals.var_chi_1_dn11 = assign71860_e108593_d_n11;
        locals.var_chi_1_dn14 = assign71860_e108593_d_n14;
        locals.var_chi_1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_270(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign71870_e108607, assign71870_e108607_d_n0, assign71870_e108607_d_n2, assign71870_e108607_d_n4, assign71870_e108607_d_n5, assign71870_e108607_d_n6, assign71870_e108607_d_n7, assign71870_e108607_d_n8, assign71870_e108607_d_n9, assign71870_e108607_d_n10, assign71870_e108607_d_n11, assign71870_e108607_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) {
        let (assign71870_e108605, assign71870_e108605_d_n0, assign71870_e108605_d_n2, assign71870_e108605_d_n4, assign71870_e108605_d_n5, assign71870_e108605_d_n6, assign71870_e108605_d_n7, assign71870_e108605_d_n8, assign71870_e108605_d_n9, assign71870_e108605_d_n10, assign71870_e108605_d_n11, assign71870_e108605_d_n14,) = {
            if (locals.var_chi_1 >= 0.0) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign71870_e108605, assign71870_e108605_d_n0, assign71870_e108605_d_n2, assign71870_e108605_d_n4, assign71870_e108605_d_n5, assign71870_e108605_d_n6, assign71870_e108605_d_n7, assign71870_e108605_d_n8, assign71870_e108605_d_n9, assign71870_e108605_d_n10, assign71870_e108605_d_n11, assign71870_e108605_d_n14,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign71870_e108607;
        locals.var_chi_1_dn0 = assign71870_e108607_d_n0;
        locals.var_chi_1_dn2 = assign71870_e108607_d_n2;
        locals.var_chi_1_dn4 = assign71870_e108607_d_n4;
        locals.var_chi_1_dn5 = assign71870_e108607_d_n5;
        locals.var_chi_1_dn6 = assign71870_e108607_d_n6;
        locals.var_chi_1_dn7 = assign71870_e108607_d_n7;
        locals.var_chi_1_dn8 = assign71870_e108607_d_n8;
        locals.var_chi_1_dn9 = assign71870_e108607_d_n9;
        locals.var_chi_1_dn10 = assign71870_e108607_d_n10;
        locals.var_chi_1_dn11 = assign71870_e108607_d_n11;
        locals.var_chi_1_dn14 = assign71870_e108607_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign71880_e108618, assign71880_e108618_d_n0, assign71880_e108618_d_n2, assign71880_e108618_d_n4, assign71880_e108618_d_n5, assign71880_e108618_d_n6, assign71880_e108618_d_n7, assign71880_e108618_d_n8, assign71880_e108618_d_n9, assign71880_e108618_d_n10, assign71880_e108618_d_n11, assign71880_e108618_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) {
        let assign71880_e108616: f64 = (locals.var_psi - locals.var_chi_1);
        (assign71880_e108616, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn14 - locals.var_chi_1_dn14),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign71880_e108618;
        locals.var_psi_dn0 = assign71880_e108618_d_n0;
        locals.var_psi_dn2 = assign71880_e108618_d_n2;
        locals.var_psi_dn4 = assign71880_e108618_d_n4;
        locals.var_psi_dn5 = assign71880_e108618_d_n5;
        locals.var_psi_dn6 = assign71880_e108618_d_n6;
        locals.var_psi_dn7 = assign71880_e108618_d_n7;
        locals.var_psi_dn8 = assign71880_e108618_d_n8;
        locals.var_psi_dn9 = assign71880_e108618_d_n9;
        locals.var_psi_dn10 = assign71880_e108618_d_n10;
        locals.var_psi_dn11 = assign71880_e108618_d_n11;
        locals.var_psi_dn14 = assign71880_e108618_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign71890_e108631, assign71890_e108631_d_n0, assign71890_e108631_d_n2, assign71890_e108631_d_n4, assign71890_e108631_d_n5, assign71890_e108631_d_n6, assign71890_e108631_d_n7, assign71890_e108631_d_n8, assign71890_e108631_d_n9, assign71890_e108631_d_n10, assign71890_e108631_d_n11, assign71890_e108631_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) {
        let assign71890_e108628: f64 = (locals.var_beta * 0.1);
        let assign71890_e108629: f64 = (locals.var_psi + assign71890_e108628);
        (assign71890_e108629, (locals.var_psi_dn0 + (locals.var_beta_dn0 * 0.1)), (locals.var_psi_dn2 + (locals.var_beta_dn2 * 0.1)), (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), (locals.var_psi_dn5 + (locals.var_beta_dn5 * 0.1)), (locals.var_psi_dn6 + (locals.var_beta_dn6 * 0.1)), (locals.var_psi_dn7 + (locals.var_beta_dn7 * 0.1)), (locals.var_psi_dn8 + (locals.var_beta_dn8 * 0.1)), (locals.var_psi_dn9 + (locals.var_beta_dn9 * 0.1)), (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), (locals.var_psi_dn11 + (locals.var_beta_dn11 * 0.1)), (locals.var_psi_dn14 + (locals.var_beta_dn14 * 0.1)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign71890_e108631;
        locals.var_psi_dn0 = assign71890_e108631_d_n0;
        locals.var_psi_dn2 = assign71890_e108631_d_n2;
        locals.var_psi_dn4 = assign71890_e108631_d_n4;
        locals.var_psi_dn5 = assign71890_e108631_d_n5;
        locals.var_psi_dn6 = assign71890_e108631_d_n6;
        locals.var_psi_dn7 = assign71890_e108631_d_n7;
        locals.var_psi_dn8 = assign71890_e108631_d_n8;
        locals.var_psi_dn9 = assign71890_e108631_d_n9;
        locals.var_psi_dn10 = assign71890_e108631_d_n10;
        locals.var_psi_dn11 = assign71890_e108631_d_n11;
        locals.var_psi_dn14 = assign71890_e108631_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign71900_e108652, assign71900_e108652_d_n0, assign71900_e108652_d_n2, assign71900_e108652_d_n4, assign71900_e108652_d_n5, assign71900_e108652_d_n6, assign71900_e108652_d_n7, assign71900_e108652_d_n8, assign71900_e108652_d_n9, assign71900_e108652_d_n10, assign71900_e108652_d_n11, assign71900_e108652_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) {
        let assign71900_e108640: f64 = (locals.var_gammachi * locals.var_t0);
        let assign71900_e108643: f64 = (locals.var_psi * locals.var_psi);
        let assign71900_e108644: f64 = (assign71900_e108640 + assign71900_e108643);
        let assign71900_e108645: f64 = (assign71900_e108644).ln();
        let assign71900_e108648: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign71900_e108649: f64 = (assign71900_e108648).ln();
        let assign71900_e108650: f64 = (assign71900_e108645 - assign71900_e108649);
        (assign71900_e108650, (((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign71900_e108644) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign71900_e108648)), (((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign71900_e108644) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign71900_e108648)), (((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign71900_e108644) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign71900_e108648)), (((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign71900_e108644) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign71900_e108648)), (((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign71900_e108644) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign71900_e108648)), (((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign71900_e108644) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign71900_e108648)), (((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign71900_e108644) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign71900_e108648)), (((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign71900_e108644) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign71900_e108648)), (((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign71900_e108644) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign71900_e108648)), (((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign71900_e108644) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign71900_e108648)), (((((locals.var_gammachi_dn14 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn14)) + ((locals.var_psi_dn14 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn14))) / assign71900_e108644) - (((locals.var_cnst1over_dn14 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn14)) / assign71900_e108648)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign71900_e108652;
        locals.var_t1_dn0 = assign71900_e108652_d_n0;
        locals.var_t1_dn2 = assign71900_e108652_d_n2;
        locals.var_t1_dn4 = assign71900_e108652_d_n4;
        locals.var_t1_dn5 = assign71900_e108652_d_n5;
        locals.var_t1_dn6 = assign71900_e108652_d_n6;
        locals.var_t1_dn7 = assign71900_e108652_d_n7;
        locals.var_t1_dn8 = assign71900_e108652_d_n8;
        locals.var_t1_dn9 = assign71900_e108652_d_n9;
        locals.var_t1_dn10 = assign71900_e108652_d_n10;
        locals.var_t1_dn11 = assign71900_e108652_d_n11;
        locals.var_t1_dn14 = assign71900_e108652_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign71910_e108665, assign71910_e108665_d_n0, assign71910_e108665_d_n2, assign71910_e108665_d_n4, assign71910_e108665_d_n5, assign71910_e108665_d_n6, assign71910_e108665_d_n7, assign71910_e108665_d_n8, assign71910_e108665_d_n9, assign71910_e108665_d_n10, assign71910_e108665_d_n11, assign71910_e108665_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) {
        let assign71910_e108662: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign71910_e108663: f64 = (locals.var_t1 + assign71910_e108662);
        (assign71910_e108663, (locals.var_t1_dn0 + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), (locals.var_t1_dn2 + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), (locals.var_t1_dn4 + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), (locals.var_t1_dn5 + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), (locals.var_t1_dn6 + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), (locals.var_t1_dn7 + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), (locals.var_t1_dn8 + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), (locals.var_t1_dn9 + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), (locals.var_t1_dn10 + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), (locals.var_t1_dn11 + ((locals.var_beta_dn11 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn11))), (locals.var_t1_dn14 + ((locals.var_beta_dn14 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign71910_e108665;
        locals.var_chi_b_dn0 = assign71910_e108665_d_n0;
        locals.var_chi_b_dn2 = assign71910_e108665_d_n2;
        locals.var_chi_b_dn4 = assign71910_e108665_d_n4;
        locals.var_chi_b_dn5 = assign71910_e108665_d_n5;
        locals.var_chi_b_dn6 = assign71910_e108665_d_n6;
        locals.var_chi_b_dn7 = assign71910_e108665_d_n7;
        locals.var_chi_b_dn8 = assign71910_e108665_d_n8;
        locals.var_chi_b_dn9 = assign71910_e108665_d_n9;
        locals.var_chi_b_dn10 = assign71910_e108665_d_n10;
        locals.var_chi_b_dn11 = assign71910_e108665_d_n11;
        locals.var_chi_b_dn14 = assign71910_e108665_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign71920_e108679, assign71920_e108679_d_n0, assign71920_e108679_d_n2, assign71920_e108679_d_n4, assign71920_e108679_d_n5, assign71920_e108679_d_n6, assign71920_e108679_d_n7, assign71920_e108679_d_n8, assign71920_e108679_d_n9, assign71920_e108679_d_n10, assign71920_e108679_d_n11, assign71920_e108679_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) {
        let (assign71920_e108677, assign71920_e108677_d_n0, assign71920_e108677_d_n2, assign71920_e108677_d_n4, assign71920_e108677_d_n5, assign71920_e108677_d_n6, assign71920_e108677_d_n7, assign71920_e108677_d_n8, assign71920_e108677_d_n9, assign71920_e108677_d_n10, assign71920_e108677_d_n11, assign71920_e108677_d_n14,) = {
            if (locals.var_chi_b >= 0.0) {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign71920_e108677, assign71920_e108677_d_n0, assign71920_e108677_d_n2, assign71920_e108677_d_n4, assign71920_e108677_d_n5, assign71920_e108677_d_n6, assign71920_e108677_d_n7, assign71920_e108677_d_n8, assign71920_e108677_d_n9, assign71920_e108677_d_n10, assign71920_e108677_d_n11, assign71920_e108677_d_n14,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign71920_e108679;
        locals.var_chi_b_dn0 = assign71920_e108679_d_n0;
        locals.var_chi_b_dn2 = assign71920_e108679_d_n2;
        locals.var_chi_b_dn4 = assign71920_e108679_d_n4;
        locals.var_chi_b_dn5 = assign71920_e108679_d_n5;
        locals.var_chi_b_dn6 = assign71920_e108679_d_n6;
        locals.var_chi_b_dn7 = assign71920_e108679_d_n7;
        locals.var_chi_b_dn8 = assign71920_e108679_d_n8;
        locals.var_chi_b_dn9 = assign71920_e108679_d_n9;
        locals.var_chi_b_dn10 = assign71920_e108679_d_n10;
        locals.var_chi_b_dn11 = assign71920_e108679_d_n11;
        locals.var_chi_b_dn14 = assign71920_e108679_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign71930_e108688, assign71930_e108688_d_n0, assign71930_e108688_d_n2, assign71930_e108688_d_n4, assign71930_e108688_d_n5, assign71930_e108688_d_n6, assign71930_e108688_d_n7, assign71930_e108688_d_n8, assign71930_e108688_d_n9, assign71930_e108688_d_n10, assign71930_e108688_d_n11, assign71930_e108688_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    }
};
        locals.var_chi_a = assign71930_e108688;
        locals.var_chi_a_dn0 = assign71930_e108688_d_n0;
        locals.var_chi_a_dn2 = assign71930_e108688_d_n2;
        locals.var_chi_a_dn4 = assign71930_e108688_d_n4;
        locals.var_chi_a_dn5 = assign71930_e108688_d_n5;
        locals.var_chi_a_dn6 = assign71930_e108688_d_n6;
        locals.var_chi_a_dn7 = assign71930_e108688_d_n7;
        locals.var_chi_a_dn8 = assign71930_e108688_d_n8;
        locals.var_chi_a_dn9 = assign71930_e108688_d_n9;
        locals.var_chi_a_dn10 = assign71930_e108688_d_n10;
        locals.var_chi_a_dn11 = assign71930_e108688_d_n11;
        locals.var_chi_a_dn14 = assign71930_e108688_d_n14;
        locals.var_chi_a_rv = 0.0;

        let assign71940_e108691: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1673 = assign71940_e108691;
        locals.var_guard1673_rv = 0.0;

        let assign71950_e108696: f64 = (0.2 * locals.var_chi_b);
        let assign71950_e108697: f64 = (locals.var_chi_b - assign71950_e108696);
        let assign71950_e108701: f64 = (0.2 * locals.var_chi_b);
        let assign71950_e108704: f64 = if ((locals.var_chi_a > assign71950_e108697) && (assign71950_e108701 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1674 = assign71950_e108704;
        locals.var_guard1674_rv = 0.0;

        let (assign71960_e108723, assign71960_e108723_d_n0, assign71960_e108723_d_n2, assign71960_e108723_d_n4, assign71960_e108723_d_n5, assign71960_e108723_d_n6, assign71960_e108723_d_n7, assign71960_e108723_d_n8, assign71960_e108723_d_n9, assign71960_e108723_d_n10, assign71960_e108723_d_n11, assign71960_e108723_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        let assign71960_e108717: f64 = (locals.var_chi_a - locals.var_chi_b);
        let assign71960_e108720: f64 = (0.2 * locals.var_chi_b);
        let assign71960_e108721: f64 = (assign71960_e108717 + assign71960_e108720);
        (assign71960_e108721, ((locals.var_chi_a_dn0 - locals.var_chi_b_dn0) + (0.2 * locals.var_chi_b_dn0)), ((locals.var_chi_a_dn2 - locals.var_chi_b_dn2) + (0.2 * locals.var_chi_b_dn2)), ((locals.var_chi_a_dn4 - locals.var_chi_b_dn4) + (0.2 * locals.var_chi_b_dn4)), ((locals.var_chi_a_dn5 - locals.var_chi_b_dn5) + (0.2 * locals.var_chi_b_dn5)), ((locals.var_chi_a_dn6 - locals.var_chi_b_dn6) + (0.2 * locals.var_chi_b_dn6)), ((locals.var_chi_a_dn7 - locals.var_chi_b_dn7) + (0.2 * locals.var_chi_b_dn7)), ((locals.var_chi_a_dn8 - locals.var_chi_b_dn8) + (0.2 * locals.var_chi_b_dn8)), ((locals.var_chi_a_dn9 - locals.var_chi_b_dn9) + (0.2 * locals.var_chi_b_dn9)), ((locals.var_chi_a_dn10 - locals.var_chi_b_dn10) + (0.2 * locals.var_chi_b_dn10)), ((locals.var_chi_a_dn11 - locals.var_chi_b_dn11) + (0.2 * locals.var_chi_b_dn11)), ((locals.var_chi_a_dn14 - locals.var_chi_b_dn14) + (0.2 * locals.var_chi_b_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign71960_e108723;
        locals.var_tmf1_dn0 = assign71960_e108723_d_n0;
        locals.var_tmf1_dn2 = assign71960_e108723_d_n2;
        locals.var_tmf1_dn4 = assign71960_e108723_d_n4;
        locals.var_tmf1_dn5 = assign71960_e108723_d_n5;
        locals.var_tmf1_dn6 = assign71960_e108723_d_n6;
        locals.var_tmf1_dn7 = assign71960_e108723_d_n7;
        locals.var_tmf1_dn8 = assign71960_e108723_d_n8;
        locals.var_tmf1_dn9 = assign71960_e108723_d_n9;
        locals.var_tmf1_dn10 = assign71960_e108723_d_n10;
        locals.var_tmf1_dn11 = assign71960_e108723_d_n11;
        locals.var_tmf1_dn14 = assign71960_e108723_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign71970_e108738, assign71970_e108738_d_n0, assign71970_e108738_d_n2, assign71970_e108738_d_n4, assign71970_e108738_d_n5, assign71970_e108738_d_n6, assign71970_e108738_d_n7, assign71970_e108738_d_n8, assign71970_e108738_d_n9, assign71970_e108738_d_n10, assign71970_e108738_d_n11, assign71970_e108738_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        let assign71970_e108736: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign71970_e108736, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign71970_e108738;
        locals.var_x2_dn0 = assign71970_e108738_d_n0;
        locals.var_x2_dn2 = assign71970_e108738_d_n2;
        locals.var_x2_dn4 = assign71970_e108738_d_n4;
        locals.var_x2_dn5 = assign71970_e108738_d_n5;
        locals.var_x2_dn6 = assign71970_e108738_d_n6;
        locals.var_x2_dn7 = assign71970_e108738_d_n7;
        locals.var_x2_dn8 = assign71970_e108738_d_n8;
        locals.var_x2_dn9 = assign71970_e108738_d_n9;
        locals.var_x2_dn10 = assign71970_e108738_d_n10;
        locals.var_x2_dn11 = assign71970_e108738_d_n11;
        locals.var_x2_dn14 = assign71970_e108738_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign71980_e108757, assign71980_e108757_d_n0, assign71980_e108757_d_n2, assign71980_e108757_d_n4, assign71980_e108757_d_n5, assign71980_e108757_d_n6, assign71980_e108757_d_n7, assign71980_e108757_d_n8, assign71980_e108757_d_n9, assign71980_e108757_d_n10, assign71980_e108757_d_n11, assign71980_e108757_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        let assign71980_e108751: f64 = (0.2 * locals.var_chi_b);
        let assign71980_e108754: f64 = (0.2 * locals.var_chi_b);
        let assign71980_e108755: f64 = (assign71980_e108751 * assign71980_e108754);
        (assign71980_e108755, (((0.2 * locals.var_chi_b_dn0) * assign71980_e108754) + (assign71980_e108751 * (0.2 * locals.var_chi_b_dn0))), (((0.2 * locals.var_chi_b_dn2) * assign71980_e108754) + (assign71980_e108751 * (0.2 * locals.var_chi_b_dn2))), (((0.2 * locals.var_chi_b_dn4) * assign71980_e108754) + (assign71980_e108751 * (0.2 * locals.var_chi_b_dn4))), (((0.2 * locals.var_chi_b_dn5) * assign71980_e108754) + (assign71980_e108751 * (0.2 * locals.var_chi_b_dn5))), (((0.2 * locals.var_chi_b_dn6) * assign71980_e108754) + (assign71980_e108751 * (0.2 * locals.var_chi_b_dn6))), (((0.2 * locals.var_chi_b_dn7) * assign71980_e108754) + (assign71980_e108751 * (0.2 * locals.var_chi_b_dn7))), (((0.2 * locals.var_chi_b_dn8) * assign71980_e108754) + (assign71980_e108751 * (0.2 * locals.var_chi_b_dn8))), (((0.2 * locals.var_chi_b_dn9) * assign71980_e108754) + (assign71980_e108751 * (0.2 * locals.var_chi_b_dn9))), (((0.2 * locals.var_chi_b_dn10) * assign71980_e108754) + (assign71980_e108751 * (0.2 * locals.var_chi_b_dn10))), (((0.2 * locals.var_chi_b_dn11) * assign71980_e108754) + (assign71980_e108751 * (0.2 * locals.var_chi_b_dn11))), (((0.2 * locals.var_chi_b_dn14) * assign71980_e108754) + (assign71980_e108751 * (0.2 * locals.var_chi_b_dn14))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign71980_e108757;
        locals.var_xmax2_dn0 = assign71980_e108757_d_n0;
        locals.var_xmax2_dn2 = assign71980_e108757_d_n2;
        locals.var_xmax2_dn4 = assign71980_e108757_d_n4;
        locals.var_xmax2_dn5 = assign71980_e108757_d_n5;
        locals.var_xmax2_dn6 = assign71980_e108757_d_n6;
        locals.var_xmax2_dn7 = assign71980_e108757_d_n7;
        locals.var_xmax2_dn8 = assign71980_e108757_d_n8;
        locals.var_xmax2_dn9 = assign71980_e108757_d_n9;
        locals.var_xmax2_dn10 = assign71980_e108757_d_n10;
        locals.var_xmax2_dn11 = assign71980_e108757_d_n11;
        locals.var_xmax2_dn14 = assign71980_e108757_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign71990_e108770, assign71990_e108770_d_n0, assign71990_e108770_d_n2, assign71990_e108770_d_n4, assign71990_e108770_d_n5, assign71990_e108770_d_n6, assign71990_e108770_d_n7, assign71990_e108770_d_n8, assign71990_e108770_d_n9, assign71990_e108770_d_n10, assign71990_e108770_d_n11, assign71990_e108770_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign71990_e108770;
        locals.var_xp_dn0 = assign71990_e108770_d_n0;
        locals.var_xp_dn2 = assign71990_e108770_d_n2;
        locals.var_xp_dn4 = assign71990_e108770_d_n4;
        locals.var_xp_dn5 = assign71990_e108770_d_n5;
        locals.var_xp_dn6 = assign71990_e108770_d_n6;
        locals.var_xp_dn7 = assign71990_e108770_d_n7;
        locals.var_xp_dn8 = assign71990_e108770_d_n8;
        locals.var_xp_dn9 = assign71990_e108770_d_n9;
        locals.var_xp_dn10 = assign71990_e108770_d_n10;
        locals.var_xp_dn11 = assign71990_e108770_d_n11;
        locals.var_xp_dn14 = assign71990_e108770_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign72000_e108783, assign72000_e108783_d_n0, assign72000_e108783_d_n2, assign72000_e108783_d_n4, assign72000_e108783_d_n5, assign72000_e108783_d_n6, assign72000_e108783_d_n7, assign72000_e108783_d_n8, assign72000_e108783_d_n9, assign72000_e108783_d_n10, assign72000_e108783_d_n11, assign72000_e108783_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign72000_e108783;
        locals.var_xmp_dn0 = assign72000_e108783_d_n0;
        locals.var_xmp_dn2 = assign72000_e108783_d_n2;
        locals.var_xmp_dn4 = assign72000_e108783_d_n4;
        locals.var_xmp_dn5 = assign72000_e108783_d_n5;
        locals.var_xmp_dn6 = assign72000_e108783_d_n6;
        locals.var_xmp_dn7 = assign72000_e108783_d_n7;
        locals.var_xmp_dn8 = assign72000_e108783_d_n8;
        locals.var_xmp_dn9 = assign72000_e108783_d_n9;
        locals.var_xmp_dn10 = assign72000_e108783_d_n10;
        locals.var_xmp_dn11 = assign72000_e108783_d_n11;
        locals.var_xmp_dn14 = assign72000_e108783_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign72010_e108796,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign72010_e108796;
        locals.var_m0_rv = 0.0;

        let (assign72020_e108809,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72020_e108809;
        locals.var_mm_rv = 0.0;

        let (assign72030_e108822, assign72030_e108822_d_n0, assign72030_e108822_d_n2, assign72030_e108822_d_n4, assign72030_e108822_d_n5, assign72030_e108822_d_n6, assign72030_e108822_d_n7, assign72030_e108822_d_n8, assign72030_e108822_d_n9, assign72030_e108822_d_n10, assign72030_e108822_d_n11, assign72030_e108822_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign72030_e108822;
        locals.var_arg_dn0 = assign72030_e108822_d_n0;
        locals.var_arg_dn2 = assign72030_e108822_d_n2;
        locals.var_arg_dn4 = assign72030_e108822_d_n4;
        locals.var_arg_dn5 = assign72030_e108822_d_n5;
        locals.var_arg_dn6 = assign72030_e108822_d_n6;
        locals.var_arg_dn7 = assign72030_e108822_d_n7;
        locals.var_arg_dn8 = assign72030_e108822_d_n8;
        locals.var_arg_dn9 = assign72030_e108822_d_n9;
        locals.var_arg_dn10 = assign72030_e108822_d_n10;
        locals.var_arg_dn11 = assign72030_e108822_d_n11;
        locals.var_arg_dn14 = assign72030_e108822_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign72040_e108835, assign72040_e108835_d_n0, assign72040_e108835_d_n2, assign72040_e108835_d_n4, assign72040_e108835_d_n5, assign72040_e108835_d_n6, assign72040_e108835_d_n7, assign72040_e108835_d_n8, assign72040_e108835_d_n9, assign72040_e108835_d_n10, assign72040_e108835_d_n11, assign72040_e108835_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign72040_e108835;
        locals.var_dnm_dn0 = assign72040_e108835_d_n0;
        locals.var_dnm_dn2 = assign72040_e108835_d_n2;
        locals.var_dnm_dn4 = assign72040_e108835_d_n4;
        locals.var_dnm_dn5 = assign72040_e108835_d_n5;
        locals.var_dnm_dn6 = assign72040_e108835_d_n6;
        locals.var_dnm_dn7 = assign72040_e108835_d_n7;
        locals.var_dnm_dn8 = assign72040_e108835_d_n8;
        locals.var_dnm_dn9 = assign72040_e108835_d_n9;
        locals.var_dnm_dn10 = assign72040_e108835_d_n10;
        locals.var_dnm_dn11 = assign72040_e108835_d_n11;
        locals.var_dnm_dn14 = assign72040_e108835_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign72050_e108850, assign72050_e108850_d_n0, assign72050_e108850_d_n2, assign72050_e108850_d_n4, assign72050_e108850_d_n5, assign72050_e108850_d_n6, assign72050_e108850_d_n7, assign72050_e108850_d_n8, assign72050_e108850_d_n9, assign72050_e108850_d_n10, assign72050_e108850_d_n11, assign72050_e108850_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        let assign72050_e108848: f64 = (locals.var_xp * locals.var_x2);
        (assign72050_e108848, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign72050_e108850;
        locals.var_xp_dn0 = assign72050_e108850_d_n0;
        locals.var_xp_dn2 = assign72050_e108850_d_n2;
        locals.var_xp_dn4 = assign72050_e108850_d_n4;
        locals.var_xp_dn5 = assign72050_e108850_d_n5;
        locals.var_xp_dn6 = assign72050_e108850_d_n6;
        locals.var_xp_dn7 = assign72050_e108850_d_n7;
        locals.var_xp_dn8 = assign72050_e108850_d_n8;
        locals.var_xp_dn9 = assign72050_e108850_d_n9;
        locals.var_xp_dn10 = assign72050_e108850_d_n10;
        locals.var_xp_dn11 = assign72050_e108850_d_n11;
        locals.var_xp_dn14 = assign72050_e108850_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign72060_e108865, assign72060_e108865_d_n0, assign72060_e108865_d_n2, assign72060_e108865_d_n4, assign72060_e108865_d_n5, assign72060_e108865_d_n6, assign72060_e108865_d_n7, assign72060_e108865_d_n8, assign72060_e108865_d_n9, assign72060_e108865_d_n10, assign72060_e108865_d_n11, assign72060_e108865_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        let assign72060_e108863: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign72060_e108863, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign72060_e108865;
        locals.var_xmp_dn0 = assign72060_e108865_d_n0;
        locals.var_xmp_dn2 = assign72060_e108865_d_n2;
        locals.var_xmp_dn4 = assign72060_e108865_d_n4;
        locals.var_xmp_dn5 = assign72060_e108865_d_n5;
        locals.var_xmp_dn6 = assign72060_e108865_d_n6;
        locals.var_xmp_dn7 = assign72060_e108865_d_n7;
        locals.var_xmp_dn8 = assign72060_e108865_d_n8;
        locals.var_xmp_dn9 = assign72060_e108865_d_n9;
        locals.var_xmp_dn10 = assign72060_e108865_d_n10;
        locals.var_xmp_dn11 = assign72060_e108865_d_n11;
        locals.var_xmp_dn14 = assign72060_e108865_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign72070_e108880, assign72070_e108880_d_n0, assign72070_e108880_d_n2, assign72070_e108880_d_n4, assign72070_e108880_d_n5, assign72070_e108880_d_n6, assign72070_e108880_d_n7, assign72070_e108880_d_n8, assign72070_e108880_d_n9, assign72070_e108880_d_n10, assign72070_e108880_d_n11, assign72070_e108880_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        let assign72070_e108878: f64 = (locals.var_xp * locals.var_x2);
        (assign72070_e108878, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign72070_e108880;
        locals.var_xp_dn0 = assign72070_e108880_d_n0;
        locals.var_xp_dn2 = assign72070_e108880_d_n2;
        locals.var_xp_dn4 = assign72070_e108880_d_n4;
        locals.var_xp_dn5 = assign72070_e108880_d_n5;
        locals.var_xp_dn6 = assign72070_e108880_d_n6;
        locals.var_xp_dn7 = assign72070_e108880_d_n7;
        locals.var_xp_dn8 = assign72070_e108880_d_n8;
        locals.var_xp_dn9 = assign72070_e108880_d_n9;
        locals.var_xp_dn10 = assign72070_e108880_d_n10;
        locals.var_xp_dn11 = assign72070_e108880_d_n11;
        locals.var_xp_dn14 = assign72070_e108880_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign72080_e108895, assign72080_e108895_d_n0, assign72080_e108895_d_n2, assign72080_e108895_d_n4, assign72080_e108895_d_n5, assign72080_e108895_d_n6, assign72080_e108895_d_n7, assign72080_e108895_d_n8, assign72080_e108895_d_n9, assign72080_e108895_d_n10, assign72080_e108895_d_n11, assign72080_e108895_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        let assign72080_e108893: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign72080_e108893, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign72080_e108895;
        locals.var_xmp_dn0 = assign72080_e108895_d_n0;
        locals.var_xmp_dn2 = assign72080_e108895_d_n2;
        locals.var_xmp_dn4 = assign72080_e108895_d_n4;
        locals.var_xmp_dn5 = assign72080_e108895_d_n5;
        locals.var_xmp_dn6 = assign72080_e108895_d_n6;
        locals.var_xmp_dn7 = assign72080_e108895_d_n7;
        locals.var_xmp_dn8 = assign72080_e108895_d_n8;
        locals.var_xmp_dn9 = assign72080_e108895_d_n9;
        locals.var_xmp_dn10 = assign72080_e108895_d_n10;
        locals.var_xmp_dn11 = assign72080_e108895_d_n11;
        locals.var_xmp_dn14 = assign72080_e108895_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign72090_e108910, assign72090_e108910_d_n0, assign72090_e108910_d_n2, assign72090_e108910_d_n4, assign72090_e108910_d_n5, assign72090_e108910_d_n6, assign72090_e108910_d_n7, assign72090_e108910_d_n8, assign72090_e108910_d_n9, assign72090_e108910_d_n10, assign72090_e108910_d_n11, assign72090_e108910_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        let assign72090_e108908: f64 = (locals.var_xp + locals.var_xmp);
        (assign72090_e108908, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign72090_e108910;
        locals.var_arg_dn0 = assign72090_e108910_d_n0;
        locals.var_arg_dn2 = assign72090_e108910_d_n2;
        locals.var_arg_dn4 = assign72090_e108910_d_n4;
        locals.var_arg_dn5 = assign72090_e108910_d_n5;
        locals.var_arg_dn6 = assign72090_e108910_d_n6;
        locals.var_arg_dn7 = assign72090_e108910_d_n7;
        locals.var_arg_dn8 = assign72090_e108910_d_n8;
        locals.var_arg_dn9 = assign72090_e108910_d_n9;
        locals.var_arg_dn10 = assign72090_e108910_d_n10;
        locals.var_arg_dn11 = assign72090_e108910_d_n11;
        locals.var_arg_dn14 = assign72090_e108910_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign72100_e108923, assign72100_e108923_d_n0, assign72100_e108923_d_n2, assign72100_e108923_d_n4, assign72100_e108923_d_n5, assign72100_e108923_d_n6, assign72100_e108923_d_n7, assign72100_e108923_d_n8, assign72100_e108923_d_n9, assign72100_e108923_d_n10, assign72100_e108923_d_n11, assign72100_e108923_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign72100_e108923;
        locals.var_dnm_dn0 = assign72100_e108923_d_n0;
        locals.var_dnm_dn2 = assign72100_e108923_d_n2;
        locals.var_dnm_dn4 = assign72100_e108923_d_n4;
        locals.var_dnm_dn5 = assign72100_e108923_d_n5;
        locals.var_dnm_dn6 = assign72100_e108923_d_n6;
        locals.var_dnm_dn7 = assign72100_e108923_d_n7;
        locals.var_dnm_dn8 = assign72100_e108923_d_n8;
        locals.var_dnm_dn9 = assign72100_e108923_d_n9;
        locals.var_dnm_dn10 = assign72100_e108923_d_n10;
        locals.var_dnm_dn11 = assign72100_e108923_d_n11;
        locals.var_dnm_dn14 = assign72100_e108923_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign72110_e108938: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1675 = assign72110_e108938;
        locals.var_guard1675_rv = 0.0;

        let assign72120_e108941: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1676 = assign72120_e108941;
        locals.var_guard1676_rv = 0.0;

        let (assign72130_e108958,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) && (locals.var_guard1675 != 0.0)) && (locals.var_guard1676 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72130_e108958;
        locals.var_mm_rv = 0.0;

        let assign72140_e108961: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1677 = assign72140_e108961;
        locals.var_guard1677_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_271(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign72150_e108981,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) && (locals.var_guard1675 != 0.0)) && (locals.var_guard1676 == 0.0)) && (locals.var_guard1677 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72150_e108981;
        locals.var_mm_rv = 0.0;

        let assign72160_e108984: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1678 = assign72160_e108984;
        locals.var_guard1678_rv = 0.0;

        let (assign72170_e109007,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) && (locals.var_guard1675 != 0.0)) && (locals.var_guard1676 == 0.0)) && (locals.var_guard1677 == 0.0)) && (locals.var_guard1678 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72170_e109007;
        locals.var_mm_rv = 0.0;

        let assign72180_e109010: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1679 = assign72180_e109010;
        locals.var_guard1679_rv = 0.0;

        let (assign72190_e109036,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) && (locals.var_guard1675 != 0.0)) && (locals.var_guard1676 == 0.0)) && (locals.var_guard1677 == 0.0)) && (locals.var_guard1678 == 0.0)) && (locals.var_guard1679 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72190_e109036;
        locals.var_mm_rv = 0.0;

        let (assign72200_e109051,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) && (locals.var_guard1675 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign72200_e109051;
        locals.var_m0_rv = 0.0;

        let mut assign72210_loop_guard: usize = 0;
        while {
            let assign72210_cond_e109067: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) && (locals.var_guard1675 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign72210_cond_e109067 != 0.0
        } {
            assign72210_loop_guard += 1;
            assert!(assign72210_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign72210_body0_e109083, assign72210_body0_e109083_d_n0, assign72210_body0_e109083_d_n2, assign72210_body0_e109083_d_n4, assign72210_body0_e109083_d_n5, assign72210_body0_e109083_d_n6, assign72210_body0_e109083_d_n7, assign72210_body0_e109083_d_n8, assign72210_body0_e109083_d_n9, assign72210_body0_e109083_d_n10, assign72210_body0_e109083_d_n11, assign72210_body0_e109083_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) && (locals.var_guard1675 != 0.0)) {
        let assign72210_body0_e109081: f64 = (locals.var_dnm).sqrt();
        (assign72210_body0_e109081, (locals.var_dnm_dn0 / (2.0 * assign72210_body0_e109081)), (locals.var_dnm_dn2 / (2.0 * assign72210_body0_e109081)), (locals.var_dnm_dn4 / (2.0 * assign72210_body0_e109081)), (locals.var_dnm_dn5 / (2.0 * assign72210_body0_e109081)), (locals.var_dnm_dn6 / (2.0 * assign72210_body0_e109081)), (locals.var_dnm_dn7 / (2.0 * assign72210_body0_e109081)), (locals.var_dnm_dn8 / (2.0 * assign72210_body0_e109081)), (locals.var_dnm_dn9 / (2.0 * assign72210_body0_e109081)), (locals.var_dnm_dn10 / (2.0 * assign72210_body0_e109081)), (locals.var_dnm_dn11 / (2.0 * assign72210_body0_e109081)), (locals.var_dnm_dn14 / (2.0 * assign72210_body0_e109081)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign72210_body0_e109083;
            locals.var_dnm_dn0 = assign72210_body0_e109083_d_n0;
            locals.var_dnm_dn2 = assign72210_body0_e109083_d_n2;
            locals.var_dnm_dn4 = assign72210_body0_e109083_d_n4;
            locals.var_dnm_dn5 = assign72210_body0_e109083_d_n5;
            locals.var_dnm_dn6 = assign72210_body0_e109083_d_n6;
            locals.var_dnm_dn7 = assign72210_body0_e109083_d_n7;
            locals.var_dnm_dn8 = assign72210_body0_e109083_d_n8;
            locals.var_dnm_dn9 = assign72210_body0_e109083_d_n9;
            locals.var_dnm_dn10 = assign72210_body0_e109083_d_n10;
            locals.var_dnm_dn11 = assign72210_body0_e109083_d_n11;
            locals.var_dnm_dn14 = assign72210_body0_e109083_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign72210_body1_e109100,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) && (locals.var_guard1675 != 0.0)) {
        let assign72210_body1_e109098: f64 = (locals.var_m0 + 1.0);
        (assign72210_body1_e109098,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign72210_body1_e109100;
            locals.var_m0_rv = 0.0;
        }

        let (assign72220_e109127, assign72220_e109127_d_n0, assign72220_e109127_d_n2, assign72220_e109127_d_n4, assign72220_e109127_d_n5, assign72220_e109127_d_n6, assign72220_e109127_d_n7, assign72220_e109127_d_n8, assign72220_e109127_d_n9, assign72220_e109127_d_n10, assign72220_e109127_d_n11, assign72220_e109127_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) && (locals.var_guard1675 == 0.0)) {
        let (assign72220_e109125, assign72220_e109125_d_n0, assign72220_e109125_d_n2, assign72220_e109125_d_n4, assign72220_e109125_d_n5, assign72220_e109125_d_n6, assign72220_e109125_d_n7, assign72220_e109125_d_n8, assign72220_e109125_d_n9, assign72220_e109125_d_n10, assign72220_e109125_d_n11, assign72220_e109125_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign72220_e109122: f64 = (2.0 * 2.0);
                let assign72220_e109123: f64 = (1.0 / assign72220_e109122);
                let assign72220_e109124: f64 = (locals.var_dnm).powf(assign72220_e109123);
                (assign72220_e109124, if 0.0 == 0.0 && ((assign72220_e109123) as f64).is_finite() && ((assign72220_e109123) as f64).fract() == 0.0 { if assign72220_e109123 == 0.0 { 0.0 } else { (assign72220_e109123 * ((locals.var_dnm).powf(assign72220_e109123 - 1.0) * locals.var_dnm_dn0)) } } else { (assign72220_e109124 * (assign72220_e109123 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72220_e109123) as f64).is_finite() && ((assign72220_e109123) as f64).fract() == 0.0 { if assign72220_e109123 == 0.0 { 0.0 } else { (assign72220_e109123 * ((locals.var_dnm).powf(assign72220_e109123 - 1.0) * locals.var_dnm_dn2)) } } else { (assign72220_e109124 * (assign72220_e109123 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72220_e109123) as f64).is_finite() && ((assign72220_e109123) as f64).fract() == 0.0 { if assign72220_e109123 == 0.0 { 0.0 } else { (assign72220_e109123 * ((locals.var_dnm).powf(assign72220_e109123 - 1.0) * locals.var_dnm_dn4)) } } else { (assign72220_e109124 * (assign72220_e109123 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72220_e109123) as f64).is_finite() && ((assign72220_e109123) as f64).fract() == 0.0 { if assign72220_e109123 == 0.0 { 0.0 } else { (assign72220_e109123 * ((locals.var_dnm).powf(assign72220_e109123 - 1.0) * locals.var_dnm_dn5)) } } else { (assign72220_e109124 * (assign72220_e109123 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72220_e109123) as f64).is_finite() && ((assign72220_e109123) as f64).fract() == 0.0 { if assign72220_e109123 == 0.0 { 0.0 } else { (assign72220_e109123 * ((locals.var_dnm).powf(assign72220_e109123 - 1.0) * locals.var_dnm_dn6)) } } else { (assign72220_e109124 * (assign72220_e109123 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72220_e109123) as f64).is_finite() && ((assign72220_e109123) as f64).fract() == 0.0 { if assign72220_e109123 == 0.0 { 0.0 } else { (assign72220_e109123 * ((locals.var_dnm).powf(assign72220_e109123 - 1.0) * locals.var_dnm_dn7)) } } else { (assign72220_e109124 * (assign72220_e109123 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72220_e109123) as f64).is_finite() && ((assign72220_e109123) as f64).fract() == 0.0 { if assign72220_e109123 == 0.0 { 0.0 } else { (assign72220_e109123 * ((locals.var_dnm).powf(assign72220_e109123 - 1.0) * locals.var_dnm_dn8)) } } else { (assign72220_e109124 * (assign72220_e109123 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72220_e109123) as f64).is_finite() && ((assign72220_e109123) as f64).fract() == 0.0 { if assign72220_e109123 == 0.0 { 0.0 } else { (assign72220_e109123 * ((locals.var_dnm).powf(assign72220_e109123 - 1.0) * locals.var_dnm_dn9)) } } else { (assign72220_e109124 * (assign72220_e109123 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72220_e109123) as f64).is_finite() && ((assign72220_e109123) as f64).fract() == 0.0 { if assign72220_e109123 == 0.0 { 0.0 } else { (assign72220_e109123 * ((locals.var_dnm).powf(assign72220_e109123 - 1.0) * locals.var_dnm_dn10)) } } else { (assign72220_e109124 * (assign72220_e109123 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72220_e109123) as f64).is_finite() && ((assign72220_e109123) as f64).fract() == 0.0 { if assign72220_e109123 == 0.0 { 0.0 } else { (assign72220_e109123 * ((locals.var_dnm).powf(assign72220_e109123 - 1.0) * locals.var_dnm_dn11)) } } else { (assign72220_e109124 * (assign72220_e109123 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign72220_e109123) as f64).is_finite() && ((assign72220_e109123) as f64).fract() == 0.0 { if assign72220_e109123 == 0.0 { 0.0 } else { (assign72220_e109123 * ((locals.var_dnm).powf(assign72220_e109123 - 1.0) * locals.var_dnm_dn14)) } } else { (assign72220_e109124 * (assign72220_e109123 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign72220_e109125, assign72220_e109125_d_n0, assign72220_e109125_d_n2, assign72220_e109125_d_n4, assign72220_e109125_d_n5, assign72220_e109125_d_n6, assign72220_e109125_d_n7, assign72220_e109125_d_n8, assign72220_e109125_d_n9, assign72220_e109125_d_n10, assign72220_e109125_d_n11, assign72220_e109125_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign72220_e109127;
        locals.var_dnm_dn0 = assign72220_e109127_d_n0;
        locals.var_dnm_dn2 = assign72220_e109127_d_n2;
        locals.var_dnm_dn4 = assign72220_e109127_d_n4;
        locals.var_dnm_dn5 = assign72220_e109127_d_n5;
        locals.var_dnm_dn6 = assign72220_e109127_d_n6;
        locals.var_dnm_dn7 = assign72220_e109127_d_n7;
        locals.var_dnm_dn8 = assign72220_e109127_d_n8;
        locals.var_dnm_dn9 = assign72220_e109127_d_n9;
        locals.var_dnm_dn10 = assign72220_e109127_d_n10;
        locals.var_dnm_dn11 = assign72220_e109127_d_n11;
        locals.var_dnm_dn14 = assign72220_e109127_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign72230_e109142, assign72230_e109142_d_n0, assign72230_e109142_d_n2, assign72230_e109142_d_n4, assign72230_e109142_d_n5, assign72230_e109142_d_n6, assign72230_e109142_d_n7, assign72230_e109142_d_n8, assign72230_e109142_d_n9, assign72230_e109142_d_n10, assign72230_e109142_d_n11, assign72230_e109142_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        let assign72230_e109140: f64 = (1.0 / locals.var_dnm);
        (assign72230_e109140, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign72230_e109142;
        locals.var_dnm_dn0 = assign72230_e109142_d_n0;
        locals.var_dnm_dn2 = assign72230_e109142_d_n2;
        locals.var_dnm_dn4 = assign72230_e109142_d_n4;
        locals.var_dnm_dn5 = assign72230_e109142_d_n5;
        locals.var_dnm_dn6 = assign72230_e109142_d_n6;
        locals.var_dnm_dn7 = assign72230_e109142_d_n7;
        locals.var_dnm_dn8 = assign72230_e109142_d_n8;
        locals.var_dnm_dn9 = assign72230_e109142_d_n9;
        locals.var_dnm_dn10 = assign72230_e109142_d_n10;
        locals.var_dnm_dn11 = assign72230_e109142_d_n11;
        locals.var_dnm_dn14 = assign72230_e109142_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign72240_e109161, assign72240_e109161_d_n0, assign72240_e109161_d_n2, assign72240_e109161_d_n4, assign72240_e109161_d_n5, assign72240_e109161_d_n6, assign72240_e109161_d_n7, assign72240_e109161_d_n8, assign72240_e109161_d_n9, assign72240_e109161_d_n10, assign72240_e109161_d_n11, assign72240_e109161_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        let assign72240_e109156: f64 = (0.2 * locals.var_chi_b);
        let assign72240_e109157: f64 = (locals.var_tmf1 * assign72240_e109156);
        let assign72240_e109159: f64 = (assign72240_e109157 * locals.var_dnm);
        (assign72240_e109159, ((((locals.var_tmf1_dn0 * assign72240_e109156) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn0))) * locals.var_dnm) + (assign72240_e109157 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign72240_e109156) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn2))) * locals.var_dnm) + (assign72240_e109157 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign72240_e109156) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn4))) * locals.var_dnm) + (assign72240_e109157 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign72240_e109156) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn5))) * locals.var_dnm) + (assign72240_e109157 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign72240_e109156) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn6))) * locals.var_dnm) + (assign72240_e109157 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign72240_e109156) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn7))) * locals.var_dnm) + (assign72240_e109157 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign72240_e109156) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn8))) * locals.var_dnm) + (assign72240_e109157 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign72240_e109156) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn9))) * locals.var_dnm) + (assign72240_e109157 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign72240_e109156) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn10))) * locals.var_dnm) + (assign72240_e109157 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign72240_e109156) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn11))) * locals.var_dnm) + (assign72240_e109157 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign72240_e109156) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn14))) * locals.var_dnm) + (assign72240_e109157 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign72240_e109161;
        locals.var_tmf0_dn0 = assign72240_e109161_d_n0;
        locals.var_tmf0_dn2 = assign72240_e109161_d_n2;
        locals.var_tmf0_dn4 = assign72240_e109161_d_n4;
        locals.var_tmf0_dn5 = assign72240_e109161_d_n5;
        locals.var_tmf0_dn6 = assign72240_e109161_d_n6;
        locals.var_tmf0_dn7 = assign72240_e109161_d_n7;
        locals.var_tmf0_dn8 = assign72240_e109161_d_n8;
        locals.var_tmf0_dn9 = assign72240_e109161_d_n9;
        locals.var_tmf0_dn10 = assign72240_e109161_d_n10;
        locals.var_tmf0_dn11 = assign72240_e109161_d_n11;
        locals.var_tmf0_dn14 = assign72240_e109161_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign72250_e109182, assign72250_e109182_d_n0, assign72250_e109182_d_n2, assign72250_e109182_d_n4, assign72250_e109182_d_n5, assign72250_e109182_d_n6, assign72250_e109182_d_n7, assign72250_e109182_d_n8, assign72250_e109182_d_n9, assign72250_e109182_d_n10, assign72250_e109182_d_n11, assign72250_e109182_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        let assign72250_e109174: f64 = (0.2 * locals.var_chi_b);
        let assign72250_e109176: f64 = (assign72250_e109174 * locals.var_xmp);
        let assign72250_e109178: f64 = (assign72250_e109176 * locals.var_dnm);
        let assign72250_e109180: f64 = (assign72250_e109178 / locals.var_arg);
        (assign72250_e109180, ((((((((0.2 * locals.var_chi_b_dn0) * locals.var_xmp) + (assign72250_e109174 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign72250_e109176 * locals.var_dnm_dn0)) * locals.var_arg) - (assign72250_e109178 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn2) * locals.var_xmp) + (assign72250_e109174 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign72250_e109176 * locals.var_dnm_dn2)) * locals.var_arg) - (assign72250_e109178 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn4) * locals.var_xmp) + (assign72250_e109174 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign72250_e109176 * locals.var_dnm_dn4)) * locals.var_arg) - (assign72250_e109178 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn5) * locals.var_xmp) + (assign72250_e109174 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign72250_e109176 * locals.var_dnm_dn5)) * locals.var_arg) - (assign72250_e109178 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn6) * locals.var_xmp) + (assign72250_e109174 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign72250_e109176 * locals.var_dnm_dn6)) * locals.var_arg) - (assign72250_e109178 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn7) * locals.var_xmp) + (assign72250_e109174 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign72250_e109176 * locals.var_dnm_dn7)) * locals.var_arg) - (assign72250_e109178 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn8) * locals.var_xmp) + (assign72250_e109174 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign72250_e109176 * locals.var_dnm_dn8)) * locals.var_arg) - (assign72250_e109178 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn9) * locals.var_xmp) + (assign72250_e109174 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign72250_e109176 * locals.var_dnm_dn9)) * locals.var_arg) - (assign72250_e109178 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn10) * locals.var_xmp) + (assign72250_e109174 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign72250_e109176 * locals.var_dnm_dn10)) * locals.var_arg) - (assign72250_e109178 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn11) * locals.var_xmp) + (assign72250_e109174 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign72250_e109176 * locals.var_dnm_dn11)) * locals.var_arg) - (assign72250_e109178 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn14) * locals.var_xmp) + (assign72250_e109174 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign72250_e109176 * locals.var_dnm_dn14)) * locals.var_arg) - (assign72250_e109178 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign72250_e109182;
        locals.var_t1_dn0 = assign72250_e109182_d_n0;
        locals.var_t1_dn2 = assign72250_e109182_d_n2;
        locals.var_t1_dn4 = assign72250_e109182_d_n4;
        locals.var_t1_dn5 = assign72250_e109182_d_n5;
        locals.var_t1_dn6 = assign72250_e109182_d_n6;
        locals.var_t1_dn7 = assign72250_e109182_d_n7;
        locals.var_t1_dn8 = assign72250_e109182_d_n8;
        locals.var_t1_dn9 = assign72250_e109182_d_n9;
        locals.var_t1_dn10 = assign72250_e109182_d_n10;
        locals.var_t1_dn11 = assign72250_e109182_d_n11;
        locals.var_t1_dn14 = assign72250_e109182_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign72260_e109201, assign72260_e109201_d_n0, assign72260_e109201_d_n2, assign72260_e109201_d_n4, assign72260_e109201_d_n5, assign72260_e109201_d_n6, assign72260_e109201_d_n7, assign72260_e109201_d_n8, assign72260_e109201_d_n9, assign72260_e109201_d_n10, assign72260_e109201_d_n11, assign72260_e109201_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        let assign72260_e109196: f64 = (0.2 * locals.var_chi_b);
        let assign72260_e109197: f64 = (locals.var_chi_b - assign72260_e109196);
        let assign72260_e109199: f64 = (assign72260_e109197 + locals.var_tmf0);
        (assign72260_e109199, ((locals.var_chi_b_dn0 - (0.2 * locals.var_chi_b_dn0)) + locals.var_tmf0_dn0), ((locals.var_chi_b_dn2 - (0.2 * locals.var_chi_b_dn2)) + locals.var_tmf0_dn2), ((locals.var_chi_b_dn4 - (0.2 * locals.var_chi_b_dn4)) + locals.var_tmf0_dn4), ((locals.var_chi_b_dn5 - (0.2 * locals.var_chi_b_dn5)) + locals.var_tmf0_dn5), ((locals.var_chi_b_dn6 - (0.2 * locals.var_chi_b_dn6)) + locals.var_tmf0_dn6), ((locals.var_chi_b_dn7 - (0.2 * locals.var_chi_b_dn7)) + locals.var_tmf0_dn7), ((locals.var_chi_b_dn8 - (0.2 * locals.var_chi_b_dn8)) + locals.var_tmf0_dn8), ((locals.var_chi_b_dn9 - (0.2 * locals.var_chi_b_dn9)) + locals.var_tmf0_dn9), ((locals.var_chi_b_dn10 - (0.2 * locals.var_chi_b_dn10)) + locals.var_tmf0_dn10), ((locals.var_chi_b_dn11 - (0.2 * locals.var_chi_b_dn11)) + locals.var_tmf0_dn11), ((locals.var_chi_b_dn14 - (0.2 * locals.var_chi_b_dn14)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign72260_e109201;
        locals.var_chi_dn0 = assign72260_e109201_d_n0;
        locals.var_chi_dn2 = assign72260_e109201_d_n2;
        locals.var_chi_dn4 = assign72260_e109201_d_n4;
        locals.var_chi_dn5 = assign72260_e109201_d_n5;
        locals.var_chi_dn6 = assign72260_e109201_d_n6;
        locals.var_chi_dn7 = assign72260_e109201_d_n7;
        locals.var_chi_dn8 = assign72260_e109201_d_n8;
        locals.var_chi_dn9 = assign72260_e109201_d_n9;
        locals.var_chi_dn10 = assign72260_e109201_d_n10;
        locals.var_chi_dn11 = assign72260_e109201_d_n11;
        locals.var_chi_dn14 = assign72260_e109201_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign72270_e109214, assign72270_e109214_d_n0, assign72270_e109214_d_n2, assign72270_e109214_d_n4, assign72270_e109214_d_n5, assign72270_e109214_d_n6, assign72270_e109214_d_n7, assign72270_e109214_d_n8, assign72270_e109214_d_n9, assign72270_e109214_d_n10, assign72270_e109214_d_n11, assign72270_e109214_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign72270_e109214;
        locals.var_t1_dn0 = assign72270_e109214_d_n0;
        locals.var_t1_dn2 = assign72270_e109214_d_n2;
        locals.var_t1_dn4 = assign72270_e109214_d_n4;
        locals.var_t1_dn5 = assign72270_e109214_d_n5;
        locals.var_t1_dn6 = assign72270_e109214_d_n6;
        locals.var_t1_dn7 = assign72270_e109214_d_n7;
        locals.var_t1_dn8 = assign72270_e109214_d_n8;
        locals.var_t1_dn9 = assign72270_e109214_d_n9;
        locals.var_t1_dn10 = assign72270_e109214_d_n10;
        locals.var_t1_dn11 = assign72270_e109214_d_n11;
        locals.var_t1_dn14 = assign72270_e109214_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign72280_e109228, assign72280_e109228_d_n0, assign72280_e109228_d_n2, assign72280_e109228_d_n4, assign72280_e109228_d_n5, assign72280_e109228_d_n6, assign72280_e109228_d_n7, assign72280_e109228_d_n8, assign72280_e109228_d_n9, assign72280_e109228_d_n10, assign72280_e109228_d_n11, assign72280_e109228_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 == 0.0)) {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign72280_e109228;
        locals.var_chi_dn0 = assign72280_e109228_d_n0;
        locals.var_chi_dn2 = assign72280_e109228_d_n2;
        locals.var_chi_dn4 = assign72280_e109228_d_n4;
        locals.var_chi_dn5 = assign72280_e109228_d_n5;
        locals.var_chi_dn6 = assign72280_e109228_d_n6;
        locals.var_chi_dn7 = assign72280_e109228_d_n7;
        locals.var_chi_dn8 = assign72280_e109228_d_n8;
        locals.var_chi_dn9 = assign72280_e109228_d_n9;
        locals.var_chi_dn10 = assign72280_e109228_d_n10;
        locals.var_chi_dn11 = assign72280_e109228_d_n11;
        locals.var_chi_dn14 = assign72280_e109228_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign72290_e109242, assign72290_e109242_d_n0, assign72290_e109242_d_n2, assign72290_e109242_d_n4, assign72290_e109242_d_n5, assign72290_e109242_d_n6, assign72290_e109242_d_n7, assign72290_e109242_d_n8, assign72290_e109242_d_n9, assign72290_e109242_d_n10, assign72290_e109242_d_n11, assign72290_e109242_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 != 0.0)) && (locals.var_guard1674 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign72290_e109242;
        locals.var_t1_dn0 = assign72290_e109242_d_n0;
        locals.var_t1_dn2 = assign72290_e109242_d_n2;
        locals.var_t1_dn4 = assign72290_e109242_d_n4;
        locals.var_t1_dn5 = assign72290_e109242_d_n5;
        locals.var_t1_dn6 = assign72290_e109242_d_n6;
        locals.var_t1_dn7 = assign72290_e109242_d_n7;
        locals.var_t1_dn8 = assign72290_e109242_d_n8;
        locals.var_t1_dn9 = assign72290_e109242_d_n9;
        locals.var_t1_dn10 = assign72290_e109242_d_n10;
        locals.var_t1_dn11 = assign72290_e109242_d_n11;
        locals.var_t1_dn14 = assign72290_e109242_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign72300_e109259, assign72300_e109259_d_n0, assign72300_e109259_d_n2, assign72300_e109259_d_n4, assign72300_e109259_d_n5, assign72300_e109259_d_n6, assign72300_e109259_d_n7, assign72300_e109259_d_n8, assign72300_e109259_d_n9, assign72300_e109259_d_n10, assign72300_e109259_d_n11, assign72300_e109259_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1671 != 0.0)) && (locals.var_guard1673 == 0.0)) {
        let (assign72300_e109257, assign72300_e109257_d_n0, assign72300_e109257_d_n2, assign72300_e109257_d_n4, assign72300_e109257_d_n5, assign72300_e109257_d_n6, assign72300_e109257_d_n7, assign72300_e109257_d_n8, assign72300_e109257_d_n9, assign72300_e109257_d_n10, assign72300_e109257_d_n11, assign72300_e109257_d_n14,) = {
            if (locals.var_chi_a <= locals.var_chi_b) {
                (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
            } else {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
            }
        };
        (assign72300_e109257, assign72300_e109257_d_n0, assign72300_e109257_d_n2, assign72300_e109257_d_n4, assign72300_e109257_d_n5, assign72300_e109257_d_n6, assign72300_e109257_d_n7, assign72300_e109257_d_n8, assign72300_e109257_d_n9, assign72300_e109257_d_n10, assign72300_e109257_d_n11, assign72300_e109257_d_n14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign72300_e109259;
        locals.var_chi_dn0 = assign72300_e109259_d_n0;
        locals.var_chi_dn2 = assign72300_e109259_d_n2;
        locals.var_chi_dn4 = assign72300_e109259_d_n4;
        locals.var_chi_dn5 = assign72300_e109259_d_n5;
        locals.var_chi_dn6 = assign72300_e109259_d_n6;
        locals.var_chi_dn7 = assign72300_e109259_d_n7;
        locals.var_chi_dn8 = assign72300_e109259_d_n8;
        locals.var_chi_dn9 = assign72300_e109259_d_n9;
        locals.var_chi_dn10 = assign72300_e109259_d_n10;
        locals.var_chi_dn11 = assign72300_e109259_d_n11;
        locals.var_chi_dn14 = assign72300_e109259_d_n14;
        locals.var_chi_rv = 0.0;

        let assign72310_e109262: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1680 = assign72310_e109262;
        locals.var_guard1680_rv = 0.0;

        let (assign72320_e109275, assign72320_e109275_d_n0, assign72320_e109275_d_n2, assign72320_e109275_d_n4, assign72320_e109275_d_n5, assign72320_e109275_d_n6, assign72320_e109275_d_n7, assign72320_e109275_d_n8, assign72320_e109275_d_n9, assign72320_e109275_d_n10, assign72320_e109275_d_n11, assign72320_e109275_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) {
        let assign72320_e109271: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign72320_e109273: f64 = (assign72320_e109271 - locals.var_vxbgmtcl);
        (assign72320_e109273, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign72320_e109275;
        locals.var_ps0ld_dn0 = assign72320_e109275_d_n0;
        locals.var_ps0ld_dn2 = assign72320_e109275_d_n2;
        locals.var_ps0ld_dn4 = assign72320_e109275_d_n4;
        locals.var_ps0ld_dn5 = assign72320_e109275_d_n5;
        locals.var_ps0ld_dn6 = assign72320_e109275_d_n6;
        locals.var_ps0ld_dn7 = assign72320_e109275_d_n7;
        locals.var_ps0ld_dn8 = assign72320_e109275_d_n8;
        locals.var_ps0ld_dn9 = assign72320_e109275_d_n9;
        locals.var_ps0ld_dn10 = assign72320_e109275_d_n10;
        locals.var_ps0ld_dn11 = assign72320_e109275_d_n11;
        locals.var_ps0ld_dn14 = assign72320_e109275_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let assign72330_e109278: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1681 = assign72330_e109278;
        locals.var_guard1681_rv = 0.0;

        let (assign72340_e109291, assign72340_e109291_d_n0, assign72340_e109291_d_n2, assign72340_e109291_d_n4, assign72340_e109291_d_n5, assign72340_e109291_d_n6, assign72340_e109291_d_n7, assign72340_e109291_d_n8, assign72340_e109291_d_n9, assign72340_e109291_d_n10, assign72340_e109291_d_n11, assign72340_e109291_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1681 != 0.0)) {
        let assign72340_e109289: f64 = (p.p334 - locals.var_wdep_func);
        (assign72340_e109289, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn11), (-locals.var_wdep_func_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign72340_e109291;
        locals.var_t2_dn0 = assign72340_e109291_d_n0;
        locals.var_t2_dn2 = assign72340_e109291_d_n2;
        locals.var_t2_dn4 = assign72340_e109291_d_n4;
        locals.var_t2_dn5 = assign72340_e109291_d_n5;
        locals.var_t2_dn6 = assign72340_e109291_d_n6;
        locals.var_t2_dn7 = assign72340_e109291_d_n7;
        locals.var_t2_dn8 = assign72340_e109291_d_n8;
        locals.var_t2_dn9 = assign72340_e109291_d_n9;
        locals.var_t2_dn10 = assign72340_e109291_d_n10;
        locals.var_t2_dn11 = assign72340_e109291_d_n11;
        locals.var_t2_dn14 = assign72340_e109291_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign72350_e109316, assign72350_e109316_d_n0, assign72350_e109316_d_n2, assign72350_e109316_d_n4, assign72350_e109316_d_n5, assign72350_e109316_d_n6, assign72350_e109316_d_n7, assign72350_e109316_d_n8, assign72350_e109316_d_n9, assign72350_e109316_d_n10, assign72350_e109316_d_n11, assign72350_e109316_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1681 == 0.0)) {
        let assign72350_e109303: f64 = (locals.var_vdsi + p.p137);
        let assign72350_e109306: f64 = (locals.var_vdsi + p.p137);
        let assign72350_e109307: f64 = (assign72350_e109303 * assign72350_e109306);
        let assign72350_e109310: f64 = (4.0 * 0.1);
        let assign72350_e109312: f64 = (assign72350_e109310 * 0.1);
        let assign72350_e109313: f64 = (assign72350_e109307 + assign72350_e109312);
        let assign72350_e109314: f64 = (assign72350_e109313).sqrt();
        (assign72350_e109314, 0.0, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn6 * assign72350_e109306) + (assign72350_e109303 * locals.var_vdsi_dn6)) / (2.0 * assign72350_e109314)), 0.0, (((locals.var_vdsi_dn8 * assign72350_e109306) + (assign72350_e109303 * locals.var_vdsi_dn8)) / (2.0 * assign72350_e109314)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign72350_e109316;
        locals.var_tmf2_dn0 = assign72350_e109316_d_n0;
        locals.var_tmf2_dn2 = assign72350_e109316_d_n2;
        locals.var_tmf2_dn4 = assign72350_e109316_d_n4;
        locals.var_tmf2_dn5 = assign72350_e109316_d_n5;
        locals.var_tmf2_dn6 = assign72350_e109316_d_n6;
        locals.var_tmf2_dn7 = assign72350_e109316_d_n7;
        locals.var_tmf2_dn8 = assign72350_e109316_d_n8;
        locals.var_tmf2_dn9 = assign72350_e109316_d_n9;
        locals.var_tmf2_dn10 = assign72350_e109316_d_n10;
        locals.var_tmf2_dn11 = assign72350_e109316_d_n11;
        locals.var_tmf2_dn14 = assign72350_e109316_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign72360_e109336, assign72360_e109336_d_n0, assign72360_e109336_d_n2, assign72360_e109336_d_n4, assign72360_e109336_d_n5, assign72360_e109336_d_n6, assign72360_e109336_d_n7, assign72360_e109336_d_n8, assign72360_e109336_d_n9, assign72360_e109336_d_n10, assign72360_e109336_d_n11, assign72360_e109336_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1681 == 0.0)) {
        let assign72360_e109330: f64 = (locals.var_vdsi + p.p137);
        let assign72360_e109332: f64 = (assign72360_e109330 / locals.var_tmf2);
        let assign72360_e109333: f64 = (1.0 + assign72360_e109332);
        let assign72360_e109334: f64 = (0.5 * assign72360_e109333);
        (assign72360_e109334, (0.5 * (-((assign72360_e109330 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign72360_e109330 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign72360_e109330 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign72360_e109330 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn6 * locals.var_tmf2) - (assign72360_e109330 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign72360_e109330 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn8 * locals.var_tmf2) - (assign72360_e109330 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign72360_e109330 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign72360_e109330 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign72360_e109330 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign72360_e109330 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign72360_e109336;
        locals.var_t9_dn0 = assign72360_e109336_d_n0;
        locals.var_t9_dn2 = assign72360_e109336_d_n2;
        locals.var_t9_dn4 = assign72360_e109336_d_n4;
        locals.var_t9_dn5 = assign72360_e109336_d_n5;
        locals.var_t9_dn6 = assign72360_e109336_d_n6;
        locals.var_t9_dn7 = assign72360_e109336_d_n7;
        locals.var_t9_dn8 = assign72360_e109336_d_n8;
        locals.var_t9_dn9 = assign72360_e109336_d_n9;
        locals.var_t9_dn10 = assign72360_e109336_d_n10;
        locals.var_t9_dn11 = assign72360_e109336_d_n11;
        locals.var_t9_dn14 = assign72360_e109336_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign72370_e109354, assign72370_e109354_d_n0, assign72370_e109354_d_n2, assign72370_e109354_d_n4, assign72370_e109354_d_n5, assign72370_e109354_d_n6, assign72370_e109354_d_n7, assign72370_e109354_d_n8, assign72370_e109354_d_n9, assign72370_e109354_d_n10, assign72370_e109354_d_n11, assign72370_e109354_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1681 == 0.0)) {
        let assign72370_e109349: f64 = (locals.var_vdsi + p.p137);
        let assign72370_e109351: f64 = (assign72370_e109349 + locals.var_tmf2);
        let assign72370_e109352: f64 = (0.5 * assign72370_e109351);
        (assign72370_e109352, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * (locals.var_vdsi_dn6 + locals.var_tmf2_dn6)), (0.5 * locals.var_tmf2_dn7), (0.5 * (locals.var_vdsi_dn8 + locals.var_tmf2_dn8)), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign72370_e109354;
        locals.var_t2_dn0 = assign72370_e109354_d_n0;
        locals.var_t2_dn2 = assign72370_e109354_d_n2;
        locals.var_t2_dn4 = assign72370_e109354_d_n4;
        locals.var_t2_dn5 = assign72370_e109354_d_n5;
        locals.var_t2_dn6 = assign72370_e109354_d_n6;
        locals.var_t2_dn7 = assign72370_e109354_d_n7;
        locals.var_t2_dn8 = assign72370_e109354_d_n8;
        locals.var_t2_dn9 = assign72370_e109354_d_n9;
        locals.var_t2_dn10 = assign72370_e109354_d_n10;
        locals.var_t2_dn11 = assign72370_e109354_d_n11;
        locals.var_t2_dn14 = assign72370_e109354_d_n14;
        locals.var_t2_rv = 0.0;

        let assign72380_e109357: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1682 = assign72380_e109357;
        locals.var_guard1682_rv = 0.0;

        let (assign72390_e109371, assign72390_e109371_d_n0, assign72390_e109371_d_n2, assign72390_e109371_d_n4, assign72390_e109371_d_n5, assign72390_e109371_d_n6, assign72390_e109371_d_n7, assign72390_e109371_d_n8, assign72390_e109371_d_n9, assign72390_e109371_d_n10, assign72390_e109371_d_n11, assign72390_e109371_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1681 == 0.0)) && (locals.var_guard1682 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign72390_e109371;
        locals.var_t2_dn0 = assign72390_e109371_d_n0;
        locals.var_t2_dn2 = assign72390_e109371_d_n2;
        locals.var_t2_dn4 = assign72390_e109371_d_n4;
        locals.var_t2_dn5 = assign72390_e109371_d_n5;
        locals.var_t2_dn6 = assign72390_e109371_d_n6;
        locals.var_t2_dn7 = assign72390_e109371_d_n7;
        locals.var_t2_dn8 = assign72390_e109371_d_n8;
        locals.var_t2_dn9 = assign72390_e109371_d_n9;
        locals.var_t2_dn10 = assign72390_e109371_d_n10;
        locals.var_t2_dn11 = assign72390_e109371_d_n11;
        locals.var_t2_dn14 = assign72390_e109371_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign72400_e109385, assign72400_e109385_d_n0, assign72400_e109385_d_n2, assign72400_e109385_d_n4, assign72400_e109385_d_n5, assign72400_e109385_d_n6, assign72400_e109385_d_n7, assign72400_e109385_d_n8, assign72400_e109385_d_n9, assign72400_e109385_d_n10, assign72400_e109385_d_n11, assign72400_e109385_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1681 == 0.0)) && (locals.var_guard1682 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign72400_e109385;
        locals.var_t9_dn0 = assign72400_e109385_d_n0;
        locals.var_t9_dn2 = assign72400_e109385_d_n2;
        locals.var_t9_dn4 = assign72400_e109385_d_n4;
        locals.var_t9_dn5 = assign72400_e109385_d_n5;
        locals.var_t9_dn6 = assign72400_e109385_d_n6;
        locals.var_t9_dn7 = assign72400_e109385_d_n7;
        locals.var_t9_dn8 = assign72400_e109385_d_n8;
        locals.var_t9_dn9 = assign72400_e109385_d_n9;
        locals.var_t9_dn10 = assign72400_e109385_d_n10;
        locals.var_t9_dn11 = assign72400_e109385_d_n11;
        locals.var_t9_dn14 = assign72400_e109385_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign72410_e109402, assign72410_e109402_d_n0, assign72410_e109402_d_n2, assign72410_e109402_d_n4, assign72410_e109402_d_n5, assign72410_e109402_d_n6, assign72410_e109402_d_n7, assign72410_e109402_d_n8, assign72410_e109402_d_n9, assign72410_e109402_d_n10, assign72410_e109402_d_n11, assign72410_e109402_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1681 == 0.0)) {
        let assign72410_e109397: f64 = (locals.var_kjunc * locals.var_t2);
        let assign72410_e109398: f64 = (assign72410_e109397).sqrt();
        let assign72410_e109400: f64 = (assign72410_e109398 * p.p432);
        (assign72410_e109400, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign72410_e109398)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign72410_e109398)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign72410_e109398)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign72410_e109398)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign72410_e109398)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign72410_e109398)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign72410_e109398)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign72410_e109398)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign72410_e109398)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign72410_e109398)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign72410_e109398)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign72410_e109402;
        locals.var_wjunc0_dn0 = assign72410_e109402_d_n0;
        locals.var_wjunc0_dn2 = assign72410_e109402_d_n2;
        locals.var_wjunc0_dn4 = assign72410_e109402_d_n4;
        locals.var_wjunc0_dn5 = assign72410_e109402_d_n5;
        locals.var_wjunc0_dn6 = assign72410_e109402_d_n6;
        locals.var_wjunc0_dn7 = assign72410_e109402_d_n7;
        locals.var_wjunc0_dn8 = assign72410_e109402_d_n8;
        locals.var_wjunc0_dn9 = assign72410_e109402_d_n9;
        locals.var_wjunc0_dn10 = assign72410_e109402_d_n10;
        locals.var_wjunc0_dn11 = assign72410_e109402_d_n11;
        locals.var_wjunc0_dn14 = assign72410_e109402_d_n14;
        locals.var_wjunc0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_272(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign72420_e109416, assign72420_e109416_d_n0, assign72420_e109416_d_n2, assign72420_e109416_d_n4, assign72420_e109416_d_n5, assign72420_e109416_d_n6, assign72420_e109416_d_n7, assign72420_e109416_d_n8, assign72420_e109416_d_n9, assign72420_e109416_d_n10, assign72420_e109416_d_n11, assign72420_e109416_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1681 == 0.0)) {
        let assign72420_e109414: f64 = (p.p334 - locals.var_wjunc0);
        (assign72420_e109414, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign72420_e109416;
        locals.var_t2_dn0 = assign72420_e109416_d_n0;
        locals.var_t2_dn2 = assign72420_e109416_d_n2;
        locals.var_t2_dn4 = assign72420_e109416_d_n4;
        locals.var_t2_dn5 = assign72420_e109416_d_n5;
        locals.var_t2_dn6 = assign72420_e109416_d_n6;
        locals.var_t2_dn7 = assign72420_e109416_d_n7;
        locals.var_t2_dn8 = assign72420_e109416_d_n8;
        locals.var_t2_dn9 = assign72420_e109416_d_n9;
        locals.var_t2_dn10 = assign72420_e109416_d_n10;
        locals.var_t2_dn11 = assign72420_e109416_d_n11;
        locals.var_t2_dn14 = assign72420_e109416_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign72430_e109438, assign72430_e109438_d_n0, assign72430_e109438_d_n2, assign72430_e109438_d_n4, assign72430_e109438_d_n5, assign72430_e109438_d_n6, assign72430_e109438_d_n7, assign72430_e109438_d_n8, assign72430_e109438_d_n9, assign72430_e109438_d_n10, assign72430_e109438_d_n11, assign72430_e109438_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) {
        let assign72430_e109425: f64 = (locals.var_t2 * locals.var_t2);
        let assign72430_e109429: f64 = (p.p334 * 0.01);
        let assign72430_e109430: f64 = (4.0 * assign72430_e109429);
        let assign72430_e109433: f64 = (p.p334 * 0.01);
        let assign72430_e109434: f64 = (assign72430_e109430 * assign72430_e109433);
        let assign72430_e109435: f64 = (assign72430_e109425 + assign72430_e109434);
        let assign72430_e109436: f64 = (assign72430_e109435).sqrt();
        (assign72430_e109436, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign72430_e109436)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign72430_e109436)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign72430_e109436)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign72430_e109436)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign72430_e109436)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign72430_e109436)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign72430_e109436)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign72430_e109436)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign72430_e109436)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign72430_e109436)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign72430_e109436)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign72430_e109438;
        locals.var_tmf2_dn0 = assign72430_e109438_d_n0;
        locals.var_tmf2_dn2 = assign72430_e109438_d_n2;
        locals.var_tmf2_dn4 = assign72430_e109438_d_n4;
        locals.var_tmf2_dn5 = assign72430_e109438_d_n5;
        locals.var_tmf2_dn6 = assign72430_e109438_d_n6;
        locals.var_tmf2_dn7 = assign72430_e109438_d_n7;
        locals.var_tmf2_dn8 = assign72430_e109438_d_n8;
        locals.var_tmf2_dn9 = assign72430_e109438_d_n9;
        locals.var_tmf2_dn10 = assign72430_e109438_d_n10;
        locals.var_tmf2_dn11 = assign72430_e109438_d_n11;
        locals.var_tmf2_dn14 = assign72430_e109438_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign72440_e109453, assign72440_e109453_d_n0, assign72440_e109453_d_n2, assign72440_e109453_d_n4, assign72440_e109453_d_n5, assign72440_e109453_d_n6, assign72440_e109453_d_n7, assign72440_e109453_d_n8, assign72440_e109453_d_n9, assign72440_e109453_d_n10, assign72440_e109453_d_n11, assign72440_e109453_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) {
        let assign72440_e109449: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign72440_e109450: f64 = (1.0 + assign72440_e109449);
        let assign72440_e109451: f64 = (0.5 * assign72440_e109450);
        (assign72440_e109451, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign72440_e109453;
        locals.var_t9_dn0 = assign72440_e109453_d_n0;
        locals.var_t9_dn2 = assign72440_e109453_d_n2;
        locals.var_t9_dn4 = assign72440_e109453_d_n4;
        locals.var_t9_dn5 = assign72440_e109453_d_n5;
        locals.var_t9_dn6 = assign72440_e109453_d_n6;
        locals.var_t9_dn7 = assign72440_e109453_d_n7;
        locals.var_t9_dn8 = assign72440_e109453_d_n8;
        locals.var_t9_dn9 = assign72440_e109453_d_n9;
        locals.var_t9_dn10 = assign72440_e109453_d_n10;
        locals.var_t9_dn11 = assign72440_e109453_d_n11;
        locals.var_t9_dn14 = assign72440_e109453_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign72450_e109466, assign72450_e109466_d_n0, assign72450_e109466_d_n2, assign72450_e109466_d_n4, assign72450_e109466_d_n5, assign72450_e109466_d_n6, assign72450_e109466_d_n7, assign72450_e109466_d_n8, assign72450_e109466_d_n9, assign72450_e109466_d_n10, assign72450_e109466_d_n11, assign72450_e109466_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) {
        let assign72450_e109463: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign72450_e109464: f64 = (0.5 * assign72450_e109463);
        (assign72450_e109464, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign72450_e109466;
        locals.var_t2_dn0 = assign72450_e109466_d_n0;
        locals.var_t2_dn2 = assign72450_e109466_d_n2;
        locals.var_t2_dn4 = assign72450_e109466_d_n4;
        locals.var_t2_dn5 = assign72450_e109466_d_n5;
        locals.var_t2_dn6 = assign72450_e109466_d_n6;
        locals.var_t2_dn7 = assign72450_e109466_d_n7;
        locals.var_t2_dn8 = assign72450_e109466_d_n8;
        locals.var_t2_dn9 = assign72450_e109466_d_n9;
        locals.var_t2_dn10 = assign72450_e109466_d_n10;
        locals.var_t2_dn11 = assign72450_e109466_d_n11;
        locals.var_t2_dn14 = assign72450_e109466_d_n14;
        locals.var_t2_rv = 0.0;

        let assign72460_e109469: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1683 = assign72460_e109469;
        locals.var_guard1683_rv = 0.0;

        let (assign72470_e109480, assign72470_e109480_d_n0, assign72470_e109480_d_n2, assign72470_e109480_d_n4, assign72470_e109480_d_n5, assign72470_e109480_d_n6, assign72470_e109480_d_n7, assign72470_e109480_d_n8, assign72470_e109480_d_n9, assign72470_e109480_d_n10, assign72470_e109480_d_n11, assign72470_e109480_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1683 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign72470_e109480;
        locals.var_t2_dn0 = assign72470_e109480_d_n0;
        locals.var_t2_dn2 = assign72470_e109480_d_n2;
        locals.var_t2_dn4 = assign72470_e109480_d_n4;
        locals.var_t2_dn5 = assign72470_e109480_d_n5;
        locals.var_t2_dn6 = assign72470_e109480_d_n6;
        locals.var_t2_dn7 = assign72470_e109480_d_n7;
        locals.var_t2_dn8 = assign72470_e109480_d_n8;
        locals.var_t2_dn9 = assign72470_e109480_d_n9;
        locals.var_t2_dn10 = assign72470_e109480_d_n10;
        locals.var_t2_dn11 = assign72470_e109480_d_n11;
        locals.var_t2_dn14 = assign72470_e109480_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign72480_e109491, assign72480_e109491_d_n0, assign72480_e109491_d_n2, assign72480_e109491_d_n4, assign72480_e109491_d_n5, assign72480_e109491_d_n6, assign72480_e109491_d_n7, assign72480_e109491_d_n8, assign72480_e109491_d_n9, assign72480_e109491_d_n10, assign72480_e109491_d_n11, assign72480_e109491_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1683 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign72480_e109491;
        locals.var_t9_dn0 = assign72480_e109491_d_n0;
        locals.var_t9_dn2 = assign72480_e109491_d_n2;
        locals.var_t9_dn4 = assign72480_e109491_d_n4;
        locals.var_t9_dn5 = assign72480_e109491_d_n5;
        locals.var_t9_dn6 = assign72480_e109491_d_n6;
        locals.var_t9_dn7 = assign72480_e109491_d_n7;
        locals.var_t9_dn8 = assign72480_e109491_d_n8;
        locals.var_t9_dn9 = assign72480_e109491_d_n9;
        locals.var_t9_dn10 = assign72480_e109491_d_n10;
        locals.var_t9_dn11 = assign72480_e109491_d_n11;
        locals.var_t9_dn14 = assign72480_e109491_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign72490_e109500, assign72490_e109500_d_n0, assign72490_e109500_d_n2, assign72490_e109500_d_n4, assign72490_e109500_d_n5, assign72490_e109500_d_n6, assign72490_e109500_d_n7, assign72490_e109500_d_n8, assign72490_e109500_d_n9, assign72490_e109500_d_n10, assign72490_e109500_d_n11, assign72490_e109500_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign72490_e109500;
        locals.var_ddriftldc_dn0 = assign72490_e109500_d_n0;
        locals.var_ddriftldc_dn2 = assign72490_e109500_d_n2;
        locals.var_ddriftldc_dn4 = assign72490_e109500_d_n4;
        locals.var_ddriftldc_dn5 = assign72490_e109500_d_n5;
        locals.var_ddriftldc_dn6 = assign72490_e109500_d_n6;
        locals.var_ddriftldc_dn7 = assign72490_e109500_d_n7;
        locals.var_ddriftldc_dn8 = assign72490_e109500_d_n8;
        locals.var_ddriftldc_dn9 = assign72490_e109500_d_n9;
        locals.var_ddriftldc_dn10 = assign72490_e109500_d_n10;
        locals.var_ddriftldc_dn11 = assign72490_e109500_d_n11;
        locals.var_ddriftldc_dn14 = assign72490_e109500_d_n14;
        locals.var_ddriftldc_rv = 0.0;

        let (assign72500_e109517, assign72500_e109517_d_n0, assign72500_e109517_d_n2, assign72500_e109517_d_n4, assign72500_e109517_d_n5, assign72500_e109517_d_n6, assign72500_e109517_d_n7, assign72500_e109517_d_n8, assign72500_e109517_d_n9, assign72500_e109517_d_n10, assign72500_e109517_d_n11, assign72500_e109517_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) {
        let assign72500_e109509: f64 = (locals.var_q_nsubld * locals.var_ddriftldc);
        let assign72500_e109511: f64 = (assign72500_e109509 * locals.var_ddriftldc);
        let assign72500_e109513: f64 = (assign72500_e109511 / 2.0);
        let assign72500_e109515: f64 = (assign72500_e109513 / 1.034943e-10);
        (assign72500_e109515, (((((locals.var_q_nsubld * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign72500_e109509 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign72500_e109509 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign72500_e109509 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign72500_e109509 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign72500_e109509 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign72500_e109509 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign72500_e109509 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign72500_e109509 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign72500_e109509 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign72500_e109509 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign72500_e109509 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign72500_e109517;
        locals.var_dphi_sb_dn0 = assign72500_e109517_d_n0;
        locals.var_dphi_sb_dn2 = assign72500_e109517_d_n2;
        locals.var_dphi_sb_dn4 = assign72500_e109517_d_n4;
        locals.var_dphi_sb_dn5 = assign72500_e109517_d_n5;
        locals.var_dphi_sb_dn6 = assign72500_e109517_d_n6;
        locals.var_dphi_sb_dn7 = assign72500_e109517_d_n7;
        locals.var_dphi_sb_dn8 = assign72500_e109517_d_n8;
        locals.var_dphi_sb_dn9 = assign72500_e109517_d_n9;
        locals.var_dphi_sb_dn10 = assign72500_e109517_d_n10;
        locals.var_dphi_sb_dn11 = assign72500_e109517_d_n11;
        locals.var_dphi_sb_dn14 = assign72500_e109517_d_n14;
        locals.var_dphi_sb_rv = 0.0;

        let (assign72510_e109531, assign72510_e109531_d_n0, assign72510_e109531_d_n2, assign72510_e109531_d_n4, assign72510_e109531_d_n5, assign72510_e109531_d_n6, assign72510_e109531_d_n7, assign72510_e109531_d_n8, assign72510_e109531_d_n9, assign72510_e109531_d_n10, assign72510_e109531_d_n11, assign72510_e109531_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) {
        let assign72510_e109526: f64 = (2.0 * locals.var_beta);
        let assign72510_e109528: f64 = (assign72510_e109526 * locals.var_dphi_sb);
        let assign72510_e109529: f64 = (assign72510_e109528).sqrt();
        (assign72510_e109529, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign72510_e109526 * locals.var_dphi_sb_dn0)) / (2.0 * assign72510_e109529)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign72510_e109526 * locals.var_dphi_sb_dn2)) / (2.0 * assign72510_e109529)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign72510_e109526 * locals.var_dphi_sb_dn4)) / (2.0 * assign72510_e109529)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign72510_e109526 * locals.var_dphi_sb_dn5)) / (2.0 * assign72510_e109529)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign72510_e109526 * locals.var_dphi_sb_dn6)) / (2.0 * assign72510_e109529)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign72510_e109526 * locals.var_dphi_sb_dn7)) / (2.0 * assign72510_e109529)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign72510_e109526 * locals.var_dphi_sb_dn8)) / (2.0 * assign72510_e109529)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign72510_e109526 * locals.var_dphi_sb_dn9)) / (2.0 * assign72510_e109529)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign72510_e109526 * locals.var_dphi_sb_dn10)) / (2.0 * assign72510_e109529)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign72510_e109526 * locals.var_dphi_sb_dn11)) / (2.0 * assign72510_e109529)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign72510_e109526 * locals.var_dphi_sb_dn14)) / (2.0 * assign72510_e109529)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign72510_e109531;
        locals.var_t0_dn0 = assign72510_e109531_d_n0;
        locals.var_t0_dn2 = assign72510_e109531_d_n2;
        locals.var_t0_dn4 = assign72510_e109531_d_n4;
        locals.var_t0_dn5 = assign72510_e109531_d_n5;
        locals.var_t0_dn6 = assign72510_e109531_d_n6;
        locals.var_t0_dn7 = assign72510_e109531_d_n7;
        locals.var_t0_dn8 = assign72510_e109531_d_n8;
        locals.var_t0_dn9 = assign72510_e109531_d_n9;
        locals.var_t0_dn10 = assign72510_e109531_d_n10;
        locals.var_t0_dn11 = assign72510_e109531_d_n11;
        locals.var_t0_dn14 = assign72510_e109531_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign72520_e109547, assign72520_e109547_d_n0, assign72520_e109547_d_n2, assign72520_e109547_d_n4, assign72520_e109547_d_n5, assign72520_e109547_d_n6, assign72520_e109547_d_n7, assign72520_e109547_d_n8, assign72520_e109547_d_n9, assign72520_e109547_d_n10, assign72520_e109547_d_n11, assign72520_e109547_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) {
        let assign72520_e109539: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign72520_e109541: f64 = (-locals.var_t0);
        let assign72520_e109542: f64 = { let limited_exp_arg = assign72520_e109541; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign72520_e109543: f64 = (assign72520_e109539 + assign72520_e109542);
        let assign72520_e109545: f64 = (assign72520_e109543 / 2.0);
        (assign72520_e109545, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign72520_e109541; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign72520_e109541; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign72520_e109541; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign72520_e109541; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign72520_e109541; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign72520_e109541; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign72520_e109541; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign72520_e109541; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign72520_e109541; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign72520_e109541; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign72520_e109541; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign72520_e109547;
        locals.var_t1_dn0 = assign72520_e109547_d_n0;
        locals.var_t1_dn2 = assign72520_e109547_d_n2;
        locals.var_t1_dn4 = assign72520_e109547_d_n4;
        locals.var_t1_dn5 = assign72520_e109547_d_n5;
        locals.var_t1_dn6 = assign72520_e109547_d_n6;
        locals.var_t1_dn7 = assign72520_e109547_d_n7;
        locals.var_t1_dn8 = assign72520_e109547_d_n8;
        locals.var_t1_dn9 = assign72520_e109547_d_n9;
        locals.var_t1_dn10 = assign72520_e109547_d_n10;
        locals.var_t1_dn11 = assign72520_e109547_d_n11;
        locals.var_t1_dn14 = assign72520_e109547_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign72530_e109559, assign72530_e109559_d_n0, assign72530_e109559_d_n2, assign72530_e109559_d_n4, assign72530_e109559_d_n5, assign72530_e109559_d_n6, assign72530_e109559_d_n7, assign72530_e109559_d_n8, assign72530_e109559_d_n9, assign72530_e109559_d_n10, assign72530_e109559_d_n11, assign72530_e109559_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) {
        let assign72530_e109555: f64 = (locals.var_t1).ln();
        let assign72530_e109557: f64 = (assign72530_e109555 / locals.var_dphi_sb);
        (assign72530_e109557, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign72530_e109555 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign72530_e109555 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign72530_e109555 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign72530_e109555 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign72530_e109555 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign72530_e109555 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign72530_e109555 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign72530_e109555 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign72530_e109555 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign72530_e109555 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign72530_e109555 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign72530_e109559;
        locals.var_c_sb_dn0 = assign72530_e109559_d_n0;
        locals.var_c_sb_dn2 = assign72530_e109559_d_n2;
        locals.var_c_sb_dn4 = assign72530_e109559_d_n4;
        locals.var_c_sb_dn5 = assign72530_e109559_d_n5;
        locals.var_c_sb_dn6 = assign72530_e109559_d_n6;
        locals.var_c_sb_dn7 = assign72530_e109559_d_n7;
        locals.var_c_sb_dn8 = assign72530_e109559_d_n8;
        locals.var_c_sb_dn9 = assign72530_e109559_d_n9;
        locals.var_c_sb_dn10 = assign72530_e109559_d_n10;
        locals.var_c_sb_dn11 = assign72530_e109559_d_n11;
        locals.var_c_sb_dn14 = assign72530_e109559_d_n14;
        locals.var_c_sb_rv = 0.0;

        let (assign72540_e109570, assign72540_e109570_d_n0, assign72540_e109570_d_n2, assign72540_e109570_d_n4, assign72540_e109570_d_n5, assign72540_e109570_d_n6, assign72540_e109570_d_n7, assign72540_e109570_d_n8, assign72540_e109570_d_n9, assign72540_e109570_d_n10, assign72540_e109570_d_n11, assign72540_e109570_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) {
        let assign72540_e109568: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign72540_e109568, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
        locals.var_ps0ld_vxb = assign72540_e109570;
        locals.var_ps0ld_vxb_dn0 = assign72540_e109570_d_n0;
        locals.var_ps0ld_vxb_dn2 = assign72540_e109570_d_n2;
        locals.var_ps0ld_vxb_dn4 = assign72540_e109570_d_n4;
        locals.var_ps0ld_vxb_dn5 = assign72540_e109570_d_n5;
        locals.var_ps0ld_vxb_dn6 = assign72540_e109570_d_n6;
        locals.var_ps0ld_vxb_dn7 = assign72540_e109570_d_n7;
        locals.var_ps0ld_vxb_dn8 = assign72540_e109570_d_n8;
        locals.var_ps0ld_vxb_dn9 = assign72540_e109570_d_n9;
        locals.var_ps0ld_vxb_dn10 = assign72540_e109570_d_n10;
        locals.var_ps0ld_vxb_dn11 = assign72540_e109570_d_n11;
        locals.var_ps0ld_vxb_dn14 = assign72540_e109570_d_n14;
        locals.var_ps0ld_vxb_rv = 0.0;

        let (assign72550_e109583, assign72550_e109583_d_n0, assign72550_e109583_d_n2, assign72550_e109583_d_n4, assign72550_e109583_d_n5, assign72550_e109583_d_n6, assign72550_e109583_d_n7, assign72550_e109583_d_n8, assign72550_e109583_d_n9, assign72550_e109583_d_n10, assign72550_e109583_d_n11, assign72550_e109583_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) {
        let assign72550_e109580: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign72550_e109581: f64 = (locals.var_c_sb * assign72550_e109580);
        (assign72550_e109581, ((locals.var_c_sb_dn0 * assign72550_e109580) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign72550_e109580) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign72550_e109580) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign72550_e109580) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign72550_e109580) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign72550_e109580) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign72550_e109580) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign72550_e109580) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign72550_e109580) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign72550_e109580) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign72550_e109580) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign72550_e109583;
        locals.var_ty_dn0 = assign72550_e109583_d_n0;
        locals.var_ty_dn2 = assign72550_e109583_d_n2;
        locals.var_ty_dn4 = assign72550_e109583_d_n4;
        locals.var_ty_dn5 = assign72550_e109583_d_n5;
        locals.var_ty_dn6 = assign72550_e109583_d_n6;
        locals.var_ty_dn7 = assign72550_e109583_d_n7;
        locals.var_ty_dn8 = assign72550_e109583_d_n8;
        locals.var_ty_dn9 = assign72550_e109583_d_n9;
        locals.var_ty_dn10 = assign72550_e109583_d_n10;
        locals.var_ty_dn11 = assign72550_e109583_d_n11;
        locals.var_ty_dn14 = assign72550_e109583_d_n14;
        locals.var_ty_rv = 0.0;

        let assign72560_e109586: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
        locals.var_guard1684 = assign72560_e109586;
        locals.var_guard1684_rv = 0.0;

        let (assign72570_e109598, assign72570_e109598_d_n0, assign72570_e109598_d_n2, assign72570_e109598_d_n4, assign72570_e109598_d_n5, assign72570_e109598_d_n6, assign72570_e109598_d_n7, assign72570_e109598_d_n8, assign72570_e109598_d_n9, assign72570_e109598_d_n10, assign72570_e109598_d_n11, assign72570_e109598_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1684 != 0.0)) {
        let assign72570_e109596: f64 = (locals.var_ty).exp();
        (assign72570_e109596, (assign72570_e109596 * locals.var_ty_dn0), (assign72570_e109596 * locals.var_ty_dn2), (assign72570_e109596 * locals.var_ty_dn4), (assign72570_e109596 * locals.var_ty_dn5), (assign72570_e109596 * locals.var_ty_dn6), (assign72570_e109596 * locals.var_ty_dn7), (assign72570_e109596 * locals.var_ty_dn8), (assign72570_e109596 * locals.var_ty_dn9), (assign72570_e109596 * locals.var_ty_dn10), (assign72570_e109596 * locals.var_ty_dn11), (assign72570_e109596 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign72570_e109598;
        locals.var_t1_dn0 = assign72570_e109598_d_n0;
        locals.var_t1_dn2 = assign72570_e109598_d_n2;
        locals.var_t1_dn4 = assign72570_e109598_d_n4;
        locals.var_t1_dn5 = assign72570_e109598_d_n5;
        locals.var_t1_dn6 = assign72570_e109598_d_n6;
        locals.var_t1_dn7 = assign72570_e109598_d_n7;
        locals.var_t1_dn8 = assign72570_e109598_d_n8;
        locals.var_t1_dn9 = assign72570_e109598_d_n9;
        locals.var_t1_dn10 = assign72570_e109598_d_n10;
        locals.var_t1_dn11 = assign72570_e109598_d_n11;
        locals.var_t1_dn14 = assign72570_e109598_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign72580_e109613, assign72580_e109613_d_n0, assign72580_e109613_d_n2, assign72580_e109613_d_n4, assign72580_e109613_d_n5, assign72580_e109613_d_n6, assign72580_e109613_d_n7, assign72580_e109613_d_n8, assign72580_e109613_d_n9, assign72580_e109613_d_n10, assign72580_e109613_d_n11, assign72580_e109613_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1684 != 0.0)) {
        let assign72580_e109608: f64 = (-locals.var_c_sb);
        let assign72580_e109610: f64 = (assign72580_e109608 * locals.var_dphi_sb);
        let assign72580_e109611: f64 = (assign72580_e109610).exp();
        (assign72580_e109611, (assign72580_e109611 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign72580_e109608 * locals.var_dphi_sb_dn0))), (assign72580_e109611 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign72580_e109608 * locals.var_dphi_sb_dn2))), (assign72580_e109611 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign72580_e109608 * locals.var_dphi_sb_dn4))), (assign72580_e109611 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign72580_e109608 * locals.var_dphi_sb_dn5))), (assign72580_e109611 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign72580_e109608 * locals.var_dphi_sb_dn6))), (assign72580_e109611 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign72580_e109608 * locals.var_dphi_sb_dn7))), (assign72580_e109611 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign72580_e109608 * locals.var_dphi_sb_dn8))), (assign72580_e109611 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign72580_e109608 * locals.var_dphi_sb_dn9))), (assign72580_e109611 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign72580_e109608 * locals.var_dphi_sb_dn10))), (assign72580_e109611 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign72580_e109608 * locals.var_dphi_sb_dn11))), (assign72580_e109611 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign72580_e109608 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign72580_e109613;
        locals.var_t0_dn0 = assign72580_e109613_d_n0;
        locals.var_t0_dn2 = assign72580_e109613_d_n2;
        locals.var_t0_dn4 = assign72580_e109613_d_n4;
        locals.var_t0_dn5 = assign72580_e109613_d_n5;
        locals.var_t0_dn6 = assign72580_e109613_d_n6;
        locals.var_t0_dn7 = assign72580_e109613_d_n7;
        locals.var_t0_dn8 = assign72580_e109613_d_n8;
        locals.var_t0_dn9 = assign72580_e109613_d_n9;
        locals.var_t0_dn10 = assign72580_e109613_d_n10;
        locals.var_t0_dn11 = assign72580_e109613_d_n11;
        locals.var_t0_dn14 = assign72580_e109613_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign72590_e109626, assign72590_e109626_d_n0, assign72590_e109626_d_n2, assign72590_e109626_d_n4, assign72590_e109626_d_n5, assign72590_e109626_d_n6, assign72590_e109626_d_n7, assign72590_e109626_d_n8, assign72590_e109626_d_n9, assign72590_e109626_d_n10, assign72590_e109626_d_n11, assign72590_e109626_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1684 != 0.0)) {
        let assign72590_e109624: f64 = (locals.var_t1 - locals.var_t0);
        (assign72590_e109624, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign72590_e109626;
        locals.var_t2_dn0 = assign72590_e109626_d_n0;
        locals.var_t2_dn2 = assign72590_e109626_d_n2;
        locals.var_t2_dn4 = assign72590_e109626_d_n4;
        locals.var_t2_dn5 = assign72590_e109626_d_n5;
        locals.var_t2_dn6 = assign72590_e109626_d_n6;
        locals.var_t2_dn7 = assign72590_e109626_d_n7;
        locals.var_t2_dn8 = assign72590_e109626_d_n8;
        locals.var_t2_dn9 = assign72590_e109626_d_n9;
        locals.var_t2_dn10 = assign72590_e109626_d_n10;
        locals.var_t2_dn11 = assign72590_e109626_d_n11;
        locals.var_t2_dn14 = assign72590_e109626_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign72600_e109642, assign72600_e109642_d_n0, assign72600_e109642_d_n2, assign72600_e109642_d_n4, assign72600_e109642_d_n5, assign72600_e109642_d_n6, assign72600_e109642_d_n7, assign72600_e109642_d_n8, assign72600_e109642_d_n9, assign72600_e109642_d_n10, assign72600_e109642_d_n11, assign72600_e109642_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1684 != 0.0)) {
        let assign72600_e109637: f64 = (1.0 + locals.var_t2);
        let assign72600_e109638: f64 = (assign72600_e109637).ln();
        let assign72600_e109640: f64 = (assign72600_e109638 / locals.var_c_sb);
        (assign72600_e109640, ((((locals.var_t2_dn0 / assign72600_e109637) * locals.var_c_sb) - (assign72600_e109638 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign72600_e109637) * locals.var_c_sb) - (assign72600_e109638 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign72600_e109637) * locals.var_c_sb) - (assign72600_e109638 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign72600_e109637) * locals.var_c_sb) - (assign72600_e109638 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign72600_e109637) * locals.var_c_sb) - (assign72600_e109638 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign72600_e109637) * locals.var_c_sb) - (assign72600_e109638 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign72600_e109637) * locals.var_c_sb) - (assign72600_e109638 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign72600_e109637) * locals.var_c_sb) - (assign72600_e109638 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign72600_e109637) * locals.var_c_sb) - (assign72600_e109638 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign72600_e109637) * locals.var_c_sb) - (assign72600_e109638 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign72600_e109637) * locals.var_c_sb) - (assign72600_e109638 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
        locals.var_phi_b = assign72600_e109642;
        locals.var_phi_b_dn0 = assign72600_e109642_d_n0;
        locals.var_phi_b_dn2 = assign72600_e109642_d_n2;
        locals.var_phi_b_dn4 = assign72600_e109642_d_n4;
        locals.var_phi_b_dn5 = assign72600_e109642_d_n5;
        locals.var_phi_b_dn6 = assign72600_e109642_d_n6;
        locals.var_phi_b_dn7 = assign72600_e109642_d_n7;
        locals.var_phi_b_dn8 = assign72600_e109642_d_n8;
        locals.var_phi_b_dn9 = assign72600_e109642_d_n9;
        locals.var_phi_b_dn10 = assign72600_e109642_d_n10;
        locals.var_phi_b_dn11 = assign72600_e109642_d_n11;
        locals.var_phi_b_dn14 = assign72600_e109642_d_n14;
        locals.var_phi_b_rv = 0.0;

        let (assign72610_e109656, assign72610_e109656_d_n0, assign72610_e109656_d_n2, assign72610_e109656_d_n4, assign72610_e109656_d_n5, assign72610_e109656_d_n6, assign72610_e109656_d_n7, assign72610_e109656_d_n8, assign72610_e109656_d_n9, assign72610_e109656_d_n10, assign72610_e109656_d_n11, assign72610_e109656_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1684 == 0.0)) {
        let assign72610_e109654: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign72610_e109654, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
        locals.var_phi_b = assign72610_e109656;
        locals.var_phi_b_dn0 = assign72610_e109656_d_n0;
        locals.var_phi_b_dn2 = assign72610_e109656_d_n2;
        locals.var_phi_b_dn4 = assign72610_e109656_d_n4;
        locals.var_phi_b_dn5 = assign72610_e109656_d_n5;
        locals.var_phi_b_dn6 = assign72610_e109656_d_n6;
        locals.var_phi_b_dn7 = assign72610_e109656_d_n7;
        locals.var_phi_b_dn8 = assign72610_e109656_d_n8;
        locals.var_phi_b_dn9 = assign72610_e109656_d_n9;
        locals.var_phi_b_dn10 = assign72610_e109656_d_n10;
        locals.var_phi_b_dn11 = assign72610_e109656_d_n11;
        locals.var_phi_b_dn14 = assign72610_e109656_d_n14;
        locals.var_phi_b_rv = 0.0;

        let (assign72620_e109667, assign72620_e109667_d_n0, assign72620_e109667_d_n2, assign72620_e109667_d_n4, assign72620_e109667_d_n5, assign72620_e109667_d_n6, assign72620_e109667_d_n7, assign72620_e109667_d_n8, assign72620_e109667_d_n9, assign72620_e109667_d_n10, assign72620_e109667_d_n11, assign72620_e109667_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) {
        let assign72620_e109665: f64 = (locals.var_beta * locals.var_phi_b);
        (assign72620_e109665, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
        locals.var_chib = assign72620_e109667;
        locals.var_chib_dn0 = assign72620_e109667_d_n0;
        locals.var_chib_dn2 = assign72620_e109667_d_n2;
        locals.var_chib_dn4 = assign72620_e109667_d_n4;
        locals.var_chib_dn5 = assign72620_e109667_d_n5;
        locals.var_chib_dn6 = assign72620_e109667_d_n6;
        locals.var_chib_dn7 = assign72620_e109667_d_n7;
        locals.var_chib_dn8 = assign72620_e109667_d_n8;
        locals.var_chib_dn9 = assign72620_e109667_d_n9;
        locals.var_chib_dn10 = assign72620_e109667_d_n10;
        locals.var_chib_dn11 = assign72620_e109667_d_n11;
        locals.var_chib_dn14 = assign72620_e109667_d_n14;
        locals.var_chib_rv = 0.0;

        let assign72630_e109671: f64 = (locals.var_chi / 100.0);
        let assign72630_e109676: f64 = if ((locals.var_chib > assign72630_e109671) && (locals.var_chib > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1685 = assign72630_e109676;
        locals.var_guard1685_rv = 0.0;

        let (assign72640_e109689,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1685 != 0.0)) {
        let assign72640_e109687: f64 = (locals.var_flg_fd_mode + 1.0);
        (assign72640_e109687,)
    } else {
        (locals.var_flg_fd_mode,)
    }
};
        locals.var_flg_fd_mode = assign72640_e109689;
        locals.var_flg_fd_mode_rv = 0.0;

        let (assign72650_e109700, assign72650_e109700_d_n0, assign72650_e109700_d_n2, assign72650_e109700_d_n4, assign72650_e109700_d_n5, assign72650_e109700_d_n6, assign72650_e109700_d_n7, assign72650_e109700_d_n8, assign72650_e109700_d_n9, assign72650_e109700_d_n10, assign72650_e109700_d_n11, assign72650_e109700_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1680 != 0.0)) && (locals.var_guard1685 != 0.0)) {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign72650_e109700;
        locals.var_chi_dn0 = assign72650_e109700_d_n0;
        locals.var_chi_dn2 = assign72650_e109700_d_n2;
        locals.var_chi_dn4 = assign72650_e109700_d_n4;
        locals.var_chi_dn5 = assign72650_e109700_d_n5;
        locals.var_chi_dn6 = assign72650_e109700_d_n6;
        locals.var_chi_dn7 = assign72650_e109700_d_n7;
        locals.var_chi_dn8 = assign72650_e109700_d_n8;
        locals.var_chi_dn9 = assign72650_e109700_d_n9;
        locals.var_chi_dn10 = assign72650_e109700_d_n10;
        locals.var_chi_dn11 = assign72650_e109700_d_n11;
        locals.var_chi_dn14 = assign72650_e109700_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign72660_e109711, assign72660_e109711_d_n0, assign72660_e109711_d_n2, assign72660_e109711_d_n4, assign72660_e109711_d_n5, assign72660_e109711_d_n6, assign72660_e109711_d_n7, assign72660_e109711_d_n8, assign72660_e109711_d_n9, assign72660_e109711_d_n10, assign72660_e109711_d_n11, assign72660_e109711_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) {
        let assign72660_e109707: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign72660_e109709: f64 = (assign72660_e109707 - locals.var_vxbgmtcl);
        (assign72660_e109709, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign72660_e109711;
        locals.var_ps0ld_dn0 = assign72660_e109711_d_n0;
        locals.var_ps0ld_dn2 = assign72660_e109711_d_n2;
        locals.var_ps0ld_dn4 = assign72660_e109711_d_n4;
        locals.var_ps0ld_dn5 = assign72660_e109711_d_n5;
        locals.var_ps0ld_dn6 = assign72660_e109711_d_n6;
        locals.var_ps0ld_dn7 = assign72660_e109711_d_n7;
        locals.var_ps0ld_dn8 = assign72660_e109711_d_n8;
        locals.var_ps0ld_dn9 = assign72660_e109711_d_n9;
        locals.var_ps0ld_dn10 = assign72660_e109711_d_n10;
        locals.var_ps0ld_dn11 = assign72660_e109711_d_n11;
        locals.var_ps0ld_dn14 = assign72660_e109711_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let assign72670_e109713: f64 = (locals.var_chi).abs();
        let assign72670_e109715: f64 = if assign72670_e109713 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1686 = assign72670_e109715;
        locals.var_guard1686_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_273(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign72680_e109730, assign72680_e109730_d_n0, assign72680_e109730_d_n2, assign72680_e109730_d_n4, assign72680_e109730_d_n5, assign72680_e109730_d_n6, assign72680_e109730_d_n7, assign72680_e109730_d_n8, assign72680_e109730_d_n9, assign72680_e109730_d_n10, assign72680_e109730_d_n11, assign72680_e109730_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1686 != 0.0)) {
        let assign72680_e109724: f64 = (locals.var_chi - 1.0);
        let assign72680_e109726: f64 = (-locals.var_chi);
        let assign72680_e109727: f64 = (assign72680_e109726).exp();
        let assign72680_e109728: f64 = (assign72680_e109724 + assign72680_e109727);
        (assign72680_e109728, (locals.var_chi_dn0 + (assign72680_e109727 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign72680_e109727 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign72680_e109727 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign72680_e109727 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign72680_e109727 * (-locals.var_chi_dn6))), (locals.var_chi_dn7 + (assign72680_e109727 * (-locals.var_chi_dn7))), (locals.var_chi_dn8 + (assign72680_e109727 * (-locals.var_chi_dn8))), (locals.var_chi_dn9 + (assign72680_e109727 * (-locals.var_chi_dn9))), (locals.var_chi_dn10 + (assign72680_e109727 * (-locals.var_chi_dn10))), (locals.var_chi_dn11 + (assign72680_e109727 * (-locals.var_chi_dn11))), (locals.var_chi_dn14 + (assign72680_e109727 * (-locals.var_chi_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign72680_e109730;
        locals.var_t1_dn0 = assign72680_e109730_d_n0;
        locals.var_t1_dn2 = assign72680_e109730_d_n2;
        locals.var_t1_dn4 = assign72680_e109730_d_n4;
        locals.var_t1_dn5 = assign72680_e109730_d_n5;
        locals.var_t1_dn6 = assign72680_e109730_d_n6;
        locals.var_t1_dn7 = assign72680_e109730_d_n7;
        locals.var_t1_dn8 = assign72680_e109730_d_n8;
        locals.var_t1_dn9 = assign72680_e109730_d_n9;
        locals.var_t1_dn10 = assign72680_e109730_d_n10;
        locals.var_t1_dn11 = assign72680_e109730_d_n11;
        locals.var_t1_dn14 = assign72680_e109730_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign72690_e109740, assign72690_e109740_d_n0, assign72690_e109740_d_n2, assign72690_e109740_d_n4, assign72690_e109740_d_n5, assign72690_e109740_d_n6, assign72690_e109740_d_n7, assign72690_e109740_d_n8, assign72690_e109740_d_n9, assign72690_e109740_d_n10, assign72690_e109740_d_n11, assign72690_e109740_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1686 != 0.0)) {
        let assign72690_e109738: f64 = (locals.var_t1).sqrt();
        (assign72690_e109738, (locals.var_t1_dn0 / (2.0 * assign72690_e109738)), (locals.var_t1_dn2 / (2.0 * assign72690_e109738)), (locals.var_t1_dn4 / (2.0 * assign72690_e109738)), (locals.var_t1_dn5 / (2.0 * assign72690_e109738)), (locals.var_t1_dn6 / (2.0 * assign72690_e109738)), (locals.var_t1_dn7 / (2.0 * assign72690_e109738)), (locals.var_t1_dn8 / (2.0 * assign72690_e109738)), (locals.var_t1_dn9 / (2.0 * assign72690_e109738)), (locals.var_t1_dn10 / (2.0 * assign72690_e109738)), (locals.var_t1_dn11 / (2.0 * assign72690_e109738)), (locals.var_t1_dn14 / (2.0 * assign72690_e109738)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign72690_e109740;
        locals.var_t2_dn0 = assign72690_e109740_d_n0;
        locals.var_t2_dn2 = assign72690_e109740_d_n2;
        locals.var_t2_dn4 = assign72690_e109740_d_n4;
        locals.var_t2_dn5 = assign72690_e109740_d_n5;
        locals.var_t2_dn6 = assign72690_e109740_d_n6;
        locals.var_t2_dn7 = assign72690_e109740_d_n7;
        locals.var_t2_dn8 = assign72690_e109740_d_n8;
        locals.var_t2_dn9 = assign72690_e109740_d_n9;
        locals.var_t2_dn10 = assign72690_e109740_d_n10;
        locals.var_t2_dn11 = assign72690_e109740_d_n11;
        locals.var_t2_dn14 = assign72690_e109740_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign72710_e109771, assign72710_e109771_d_n0, assign72710_e109771_d_n2, assign72710_e109771_d_n4, assign72710_e109771_d_n5, assign72710_e109771_d_n6, assign72710_e109771_d_n7, assign72710_e109771_d_n8, assign72710_e109771_d_n9, assign72710_e109771_d_n10, assign72710_e109771_d_n11, assign72710_e109771_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1686 == 0.0)) {
        let assign72710_e109762: f64 = (0.7071067811865475 * locals.var_chi);
        let assign72710_e109766: f64 = (locals.var_chi * 0.3333333333333333);
        let assign72710_e109767: f64 = (1.0 - assign72710_e109766);
        let assign72710_e109768: f64 = (assign72710_e109767).sqrt();
        let assign72710_e109769: f64 = (assign72710_e109762 * assign72710_e109768);
        (assign72710_e109769, (((0.7071067811865475 * locals.var_chi_dn0) * assign72710_e109768) + (assign72710_e109762 * ((-(locals.var_chi_dn0 * 0.3333333333333333)) / (2.0 * assign72710_e109768)))), (((0.7071067811865475 * locals.var_chi_dn2) * assign72710_e109768) + (assign72710_e109762 * ((-(locals.var_chi_dn2 * 0.3333333333333333)) / (2.0 * assign72710_e109768)))), (((0.7071067811865475 * locals.var_chi_dn4) * assign72710_e109768) + (assign72710_e109762 * ((-(locals.var_chi_dn4 * 0.3333333333333333)) / (2.0 * assign72710_e109768)))), (((0.7071067811865475 * locals.var_chi_dn5) * assign72710_e109768) + (assign72710_e109762 * ((-(locals.var_chi_dn5 * 0.3333333333333333)) / (2.0 * assign72710_e109768)))), (((0.7071067811865475 * locals.var_chi_dn6) * assign72710_e109768) + (assign72710_e109762 * ((-(locals.var_chi_dn6 * 0.3333333333333333)) / (2.0 * assign72710_e109768)))), (((0.7071067811865475 * locals.var_chi_dn7) * assign72710_e109768) + (assign72710_e109762 * ((-(locals.var_chi_dn7 * 0.3333333333333333)) / (2.0 * assign72710_e109768)))), (((0.7071067811865475 * locals.var_chi_dn8) * assign72710_e109768) + (assign72710_e109762 * ((-(locals.var_chi_dn8 * 0.3333333333333333)) / (2.0 * assign72710_e109768)))), (((0.7071067811865475 * locals.var_chi_dn9) * assign72710_e109768) + (assign72710_e109762 * ((-(locals.var_chi_dn9 * 0.3333333333333333)) / (2.0 * assign72710_e109768)))), (((0.7071067811865475 * locals.var_chi_dn10) * assign72710_e109768) + (assign72710_e109762 * ((-(locals.var_chi_dn10 * 0.3333333333333333)) / (2.0 * assign72710_e109768)))), (((0.7071067811865475 * locals.var_chi_dn11) * assign72710_e109768) + (assign72710_e109762 * ((-(locals.var_chi_dn11 * 0.3333333333333333)) / (2.0 * assign72710_e109768)))), (((0.7071067811865475 * locals.var_chi_dn14) * assign72710_e109768) + (assign72710_e109762 * ((-(locals.var_chi_dn14 * 0.3333333333333333)) / (2.0 * assign72710_e109768)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign72710_e109771;
        locals.var_t2_dn0 = assign72710_e109771_d_n0;
        locals.var_t2_dn2 = assign72710_e109771_d_n2;
        locals.var_t2_dn4 = assign72710_e109771_d_n4;
        locals.var_t2_dn5 = assign72710_e109771_d_n5;
        locals.var_t2_dn6 = assign72710_e109771_d_n6;
        locals.var_t2_dn7 = assign72710_e109771_d_n7;
        locals.var_t2_dn8 = assign72710_e109771_d_n8;
        locals.var_t2_dn9 = assign72710_e109771_d_n9;
        locals.var_t2_dn10 = assign72710_e109771_d_n10;
        locals.var_t2_dn11 = assign72710_e109771_d_n11;
        locals.var_t2_dn14 = assign72710_e109771_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign72720_e109780, assign72720_e109780_d_n0, assign72720_e109780_d_n2, assign72720_e109780_d_n4, assign72720_e109780_d_n5, assign72720_e109780_d_n6, assign72720_e109780_d_n7, assign72720_e109780_d_n8, assign72720_e109780_d_n9, assign72720_e109780_d_n10, assign72720_e109780_d_n11, assign72720_e109780_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) {
        let assign72720_e109778: f64 = (locals.var_cnst0over_func * locals.var_t2);
        (assign72720_e109778, ((locals.var_cnst0over_func_dn0 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign72720_e109780;
        locals.var_qbuld_dn0 = assign72720_e109780_d_n0;
        locals.var_qbuld_dn2 = assign72720_e109780_d_n2;
        locals.var_qbuld_dn4 = assign72720_e109780_d_n4;
        locals.var_qbuld_dn5 = assign72720_e109780_d_n5;
        locals.var_qbuld_dn6 = assign72720_e109780_d_n6;
        locals.var_qbuld_dn7 = assign72720_e109780_d_n7;
        locals.var_qbuld_dn8 = assign72720_e109780_d_n8;
        locals.var_qbuld_dn9 = assign72720_e109780_d_n9;
        locals.var_qbuld_dn10 = assign72720_e109780_d_n10;
        locals.var_qbuld_dn11 = assign72720_e109780_d_n11;
        locals.var_qbuld_dn14 = assign72720_e109780_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign72730_e109791, assign72730_e109791_d_n0, assign72730_e109791_d_n2, assign72730_e109791_d_n4, assign72730_e109791_d_n5, assign72730_e109791_d_n6, assign72730_e109791_d_n7, assign72730_e109791_d_n8, assign72730_e109791_d_n9, assign72730_e109791_d_n10, assign72730_e109791_d_n11, assign72730_e109791_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) {
        let assign72730_e109788: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign72730_e109789: f64 = (locals.var_cox0_func * assign72730_e109788);
        (assign72730_e109789, (locals.var_cox0_func * (-locals.var_ps0ld_dn0)), (locals.var_cox0_func * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0_func * (-locals.var_ps0ld_dn4)), (locals.var_cox0_func * (-locals.var_ps0ld_dn5)), (locals.var_cox0_func * (-locals.var_ps0ld_dn6)), (locals.var_cox0_func * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0_func * (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8)), (locals.var_cox0_func * (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9)), (locals.var_cox0_func * (-locals.var_ps0ld_dn10)), (locals.var_cox0_func * (-locals.var_ps0ld_dn11)), (locals.var_cox0_func * (-locals.var_ps0ld_dn14)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign72730_e109791;
        locals.var_qsuld_dn0 = assign72730_e109791_d_n0;
        locals.var_qsuld_dn2 = assign72730_e109791_d_n2;
        locals.var_qsuld_dn4 = assign72730_e109791_d_n4;
        locals.var_qsuld_dn5 = assign72730_e109791_d_n5;
        locals.var_qsuld_dn6 = assign72730_e109791_d_n6;
        locals.var_qsuld_dn7 = assign72730_e109791_d_n7;
        locals.var_qsuld_dn8 = assign72730_e109791_d_n8;
        locals.var_qsuld_dn9 = assign72730_e109791_d_n9;
        locals.var_qsuld_dn10 = assign72730_e109791_d_n10;
        locals.var_qsuld_dn11 = assign72730_e109791_d_n11;
        locals.var_qsuld_dn14 = assign72730_e109791_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign72740_e109800, assign72740_e109800_d_n0, assign72740_e109800_d_n2, assign72740_e109800_d_n4, assign72740_e109800_d_n5, assign72740_e109800_d_n6, assign72740_e109800_d_n7, assign72740_e109800_d_n8, assign72740_e109800_d_n9, assign72740_e109800_d_n10, assign72740_e109800_d_n11, assign72740_e109800_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) {
        let assign72740_e109798: f64 = (locals.var_qbuld / locals.var_q_nsubld);
        (assign72740_e109798, (locals.var_qbuld_dn0 / locals.var_q_nsubld), (locals.var_qbuld_dn2 / locals.var_q_nsubld), (locals.var_qbuld_dn4 / locals.var_q_nsubld), (locals.var_qbuld_dn5 / locals.var_q_nsubld), (locals.var_qbuld_dn6 / locals.var_q_nsubld), (locals.var_qbuld_dn7 / locals.var_q_nsubld), (locals.var_qbuld_dn8 / locals.var_q_nsubld), (locals.var_qbuld_dn9 / locals.var_q_nsubld), (locals.var_qbuld_dn10 / locals.var_q_nsubld), (locals.var_qbuld_dn11 / locals.var_q_nsubld), (locals.var_qbuld_dn14 / locals.var_q_nsubld),)
    } else {
        (locals.var_wdld0, locals.var_wdld0_dn0, locals.var_wdld0_dn2, locals.var_wdld0_dn4, locals.var_wdld0_dn5, locals.var_wdld0_dn6, locals.var_wdld0_dn7, locals.var_wdld0_dn8, locals.var_wdld0_dn9, locals.var_wdld0_dn10, locals.var_wdld0_dn11, locals.var_wdld0_dn14,)
    }
};
        locals.var_wdld0 = assign72740_e109800;
        locals.var_wdld0_dn0 = assign72740_e109800_d_n0;
        locals.var_wdld0_dn2 = assign72740_e109800_d_n2;
        locals.var_wdld0_dn4 = assign72740_e109800_d_n4;
        locals.var_wdld0_dn5 = assign72740_e109800_d_n5;
        locals.var_wdld0_dn6 = assign72740_e109800_d_n6;
        locals.var_wdld0_dn7 = assign72740_e109800_d_n7;
        locals.var_wdld0_dn8 = assign72740_e109800_d_n8;
        locals.var_wdld0_dn9 = assign72740_e109800_d_n9;
        locals.var_wdld0_dn10 = assign72740_e109800_d_n10;
        locals.var_wdld0_dn11 = assign72740_e109800_d_n11;
        locals.var_wdld0_dn14 = assign72740_e109800_d_n14;
        locals.var_wdld0_rv = 0.0;

        let assign72750_e109803: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1689 = assign72750_e109803;
        locals.var_guard1689_rv = 0.0;

        let assign72760_e109808: f64 = (locals.var_ddriftldc * 0.1);
        let assign72760_e109809: f64 = (locals.var_ddriftldc - assign72760_e109808);
        let assign72760_e109813: f64 = (locals.var_ddriftldc * 0.1);
        let assign72760_e109816: f64 = if ((locals.var_wdld0 > assign72760_e109809) && (assign72760_e109813 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1690 = assign72760_e109816;
        locals.var_guard1690_rv = 0.0;

        let (assign72770_e109833, assign72770_e109833_d_n0, assign72770_e109833_d_n2, assign72770_e109833_d_n4, assign72770_e109833_d_n5, assign72770_e109833_d_n6, assign72770_e109833_d_n7, assign72770_e109833_d_n8, assign72770_e109833_d_n9, assign72770_e109833_d_n10, assign72770_e109833_d_n11, assign72770_e109833_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        let assign72770_e109827: f64 = (locals.var_wdld0 - locals.var_ddriftldc);
        let assign72770_e109830: f64 = (locals.var_ddriftldc * 0.1);
        let assign72770_e109831: f64 = (assign72770_e109827 + assign72770_e109830);
        (assign72770_e109831, ((locals.var_wdld0_dn0 - locals.var_ddriftldc_dn0) + (locals.var_ddriftldc_dn0 * 0.1)), ((locals.var_wdld0_dn2 - locals.var_ddriftldc_dn2) + (locals.var_ddriftldc_dn2 * 0.1)), ((locals.var_wdld0_dn4 - locals.var_ddriftldc_dn4) + (locals.var_ddriftldc_dn4 * 0.1)), ((locals.var_wdld0_dn5 - locals.var_ddriftldc_dn5) + (locals.var_ddriftldc_dn5 * 0.1)), ((locals.var_wdld0_dn6 - locals.var_ddriftldc_dn6) + (locals.var_ddriftldc_dn6 * 0.1)), ((locals.var_wdld0_dn7 - locals.var_ddriftldc_dn7) + (locals.var_ddriftldc_dn7 * 0.1)), ((locals.var_wdld0_dn8 - locals.var_ddriftldc_dn8) + (locals.var_ddriftldc_dn8 * 0.1)), ((locals.var_wdld0_dn9 - locals.var_ddriftldc_dn9) + (locals.var_ddriftldc_dn9 * 0.1)), ((locals.var_wdld0_dn10 - locals.var_ddriftldc_dn10) + (locals.var_ddriftldc_dn10 * 0.1)), ((locals.var_wdld0_dn11 - locals.var_ddriftldc_dn11) + (locals.var_ddriftldc_dn11 * 0.1)), ((locals.var_wdld0_dn14 - locals.var_ddriftldc_dn14) + (locals.var_ddriftldc_dn14 * 0.1)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign72770_e109833;
        locals.var_tmf1_dn0 = assign72770_e109833_d_n0;
        locals.var_tmf1_dn2 = assign72770_e109833_d_n2;
        locals.var_tmf1_dn4 = assign72770_e109833_d_n4;
        locals.var_tmf1_dn5 = assign72770_e109833_d_n5;
        locals.var_tmf1_dn6 = assign72770_e109833_d_n6;
        locals.var_tmf1_dn7 = assign72770_e109833_d_n7;
        locals.var_tmf1_dn8 = assign72770_e109833_d_n8;
        locals.var_tmf1_dn9 = assign72770_e109833_d_n9;
        locals.var_tmf1_dn10 = assign72770_e109833_d_n10;
        locals.var_tmf1_dn11 = assign72770_e109833_d_n11;
        locals.var_tmf1_dn14 = assign72770_e109833_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign72780_e109846, assign72780_e109846_d_n0, assign72780_e109846_d_n2, assign72780_e109846_d_n4, assign72780_e109846_d_n5, assign72780_e109846_d_n6, assign72780_e109846_d_n7, assign72780_e109846_d_n8, assign72780_e109846_d_n9, assign72780_e109846_d_n10, assign72780_e109846_d_n11, assign72780_e109846_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        let assign72780_e109844: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign72780_e109844, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign72780_e109846;
        locals.var_x2_dn0 = assign72780_e109846_d_n0;
        locals.var_x2_dn2 = assign72780_e109846_d_n2;
        locals.var_x2_dn4 = assign72780_e109846_d_n4;
        locals.var_x2_dn5 = assign72780_e109846_d_n5;
        locals.var_x2_dn6 = assign72780_e109846_d_n6;
        locals.var_x2_dn7 = assign72780_e109846_d_n7;
        locals.var_x2_dn8 = assign72780_e109846_d_n8;
        locals.var_x2_dn9 = assign72780_e109846_d_n9;
        locals.var_x2_dn10 = assign72780_e109846_d_n10;
        locals.var_x2_dn11 = assign72780_e109846_d_n11;
        locals.var_x2_dn14 = assign72780_e109846_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign72790_e109863, assign72790_e109863_d_n0, assign72790_e109863_d_n2, assign72790_e109863_d_n4, assign72790_e109863_d_n5, assign72790_e109863_d_n6, assign72790_e109863_d_n7, assign72790_e109863_d_n8, assign72790_e109863_d_n9, assign72790_e109863_d_n10, assign72790_e109863_d_n11, assign72790_e109863_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        let assign72790_e109857: f64 = (locals.var_ddriftldc * 0.1);
        let assign72790_e109860: f64 = (locals.var_ddriftldc * 0.1);
        let assign72790_e109861: f64 = (assign72790_e109857 * assign72790_e109860);
        (assign72790_e109861, (((locals.var_ddriftldc_dn0 * 0.1) * assign72790_e109860) + (assign72790_e109857 * (locals.var_ddriftldc_dn0 * 0.1))), (((locals.var_ddriftldc_dn2 * 0.1) * assign72790_e109860) + (assign72790_e109857 * (locals.var_ddriftldc_dn2 * 0.1))), (((locals.var_ddriftldc_dn4 * 0.1) * assign72790_e109860) + (assign72790_e109857 * (locals.var_ddriftldc_dn4 * 0.1))), (((locals.var_ddriftldc_dn5 * 0.1) * assign72790_e109860) + (assign72790_e109857 * (locals.var_ddriftldc_dn5 * 0.1))), (((locals.var_ddriftldc_dn6 * 0.1) * assign72790_e109860) + (assign72790_e109857 * (locals.var_ddriftldc_dn6 * 0.1))), (((locals.var_ddriftldc_dn7 * 0.1) * assign72790_e109860) + (assign72790_e109857 * (locals.var_ddriftldc_dn7 * 0.1))), (((locals.var_ddriftldc_dn8 * 0.1) * assign72790_e109860) + (assign72790_e109857 * (locals.var_ddriftldc_dn8 * 0.1))), (((locals.var_ddriftldc_dn9 * 0.1) * assign72790_e109860) + (assign72790_e109857 * (locals.var_ddriftldc_dn9 * 0.1))), (((locals.var_ddriftldc_dn10 * 0.1) * assign72790_e109860) + (assign72790_e109857 * (locals.var_ddriftldc_dn10 * 0.1))), (((locals.var_ddriftldc_dn11 * 0.1) * assign72790_e109860) + (assign72790_e109857 * (locals.var_ddriftldc_dn11 * 0.1))), (((locals.var_ddriftldc_dn14 * 0.1) * assign72790_e109860) + (assign72790_e109857 * (locals.var_ddriftldc_dn14 * 0.1))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign72790_e109863;
        locals.var_xmax2_dn0 = assign72790_e109863_d_n0;
        locals.var_xmax2_dn2 = assign72790_e109863_d_n2;
        locals.var_xmax2_dn4 = assign72790_e109863_d_n4;
        locals.var_xmax2_dn5 = assign72790_e109863_d_n5;
        locals.var_xmax2_dn6 = assign72790_e109863_d_n6;
        locals.var_xmax2_dn7 = assign72790_e109863_d_n7;
        locals.var_xmax2_dn8 = assign72790_e109863_d_n8;
        locals.var_xmax2_dn9 = assign72790_e109863_d_n9;
        locals.var_xmax2_dn10 = assign72790_e109863_d_n10;
        locals.var_xmax2_dn11 = assign72790_e109863_d_n11;
        locals.var_xmax2_dn14 = assign72790_e109863_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign72800_e109874, assign72800_e109874_d_n0, assign72800_e109874_d_n2, assign72800_e109874_d_n4, assign72800_e109874_d_n5, assign72800_e109874_d_n6, assign72800_e109874_d_n7, assign72800_e109874_d_n8, assign72800_e109874_d_n9, assign72800_e109874_d_n10, assign72800_e109874_d_n11, assign72800_e109874_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign72800_e109874;
        locals.var_xp_dn0 = assign72800_e109874_d_n0;
        locals.var_xp_dn2 = assign72800_e109874_d_n2;
        locals.var_xp_dn4 = assign72800_e109874_d_n4;
        locals.var_xp_dn5 = assign72800_e109874_d_n5;
        locals.var_xp_dn6 = assign72800_e109874_d_n6;
        locals.var_xp_dn7 = assign72800_e109874_d_n7;
        locals.var_xp_dn8 = assign72800_e109874_d_n8;
        locals.var_xp_dn9 = assign72800_e109874_d_n9;
        locals.var_xp_dn10 = assign72800_e109874_d_n10;
        locals.var_xp_dn11 = assign72800_e109874_d_n11;
        locals.var_xp_dn14 = assign72800_e109874_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign72810_e109885, assign72810_e109885_d_n0, assign72810_e109885_d_n2, assign72810_e109885_d_n4, assign72810_e109885_d_n5, assign72810_e109885_d_n6, assign72810_e109885_d_n7, assign72810_e109885_d_n8, assign72810_e109885_d_n9, assign72810_e109885_d_n10, assign72810_e109885_d_n11, assign72810_e109885_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign72810_e109885;
        locals.var_xmp_dn0 = assign72810_e109885_d_n0;
        locals.var_xmp_dn2 = assign72810_e109885_d_n2;
        locals.var_xmp_dn4 = assign72810_e109885_d_n4;
        locals.var_xmp_dn5 = assign72810_e109885_d_n5;
        locals.var_xmp_dn6 = assign72810_e109885_d_n6;
        locals.var_xmp_dn7 = assign72810_e109885_d_n7;
        locals.var_xmp_dn8 = assign72810_e109885_d_n8;
        locals.var_xmp_dn9 = assign72810_e109885_d_n9;
        locals.var_xmp_dn10 = assign72810_e109885_d_n10;
        locals.var_xmp_dn11 = assign72810_e109885_d_n11;
        locals.var_xmp_dn14 = assign72810_e109885_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign72820_e109896,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign72820_e109896;
        locals.var_m0_rv = 0.0;

        let (assign72830_e109907,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72830_e109907;
        locals.var_mm_rv = 0.0;

        let (assign72840_e109918, assign72840_e109918_d_n0, assign72840_e109918_d_n2, assign72840_e109918_d_n4, assign72840_e109918_d_n5, assign72840_e109918_d_n6, assign72840_e109918_d_n7, assign72840_e109918_d_n8, assign72840_e109918_d_n9, assign72840_e109918_d_n10, assign72840_e109918_d_n11, assign72840_e109918_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign72840_e109918;
        locals.var_arg_dn0 = assign72840_e109918_d_n0;
        locals.var_arg_dn2 = assign72840_e109918_d_n2;
        locals.var_arg_dn4 = assign72840_e109918_d_n4;
        locals.var_arg_dn5 = assign72840_e109918_d_n5;
        locals.var_arg_dn6 = assign72840_e109918_d_n6;
        locals.var_arg_dn7 = assign72840_e109918_d_n7;
        locals.var_arg_dn8 = assign72840_e109918_d_n8;
        locals.var_arg_dn9 = assign72840_e109918_d_n9;
        locals.var_arg_dn10 = assign72840_e109918_d_n10;
        locals.var_arg_dn11 = assign72840_e109918_d_n11;
        locals.var_arg_dn14 = assign72840_e109918_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign72850_e109929, assign72850_e109929_d_n0, assign72850_e109929_d_n2, assign72850_e109929_d_n4, assign72850_e109929_d_n5, assign72850_e109929_d_n6, assign72850_e109929_d_n7, assign72850_e109929_d_n8, assign72850_e109929_d_n9, assign72850_e109929_d_n10, assign72850_e109929_d_n11, assign72850_e109929_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign72850_e109929;
        locals.var_dnm_dn0 = assign72850_e109929_d_n0;
        locals.var_dnm_dn2 = assign72850_e109929_d_n2;
        locals.var_dnm_dn4 = assign72850_e109929_d_n4;
        locals.var_dnm_dn5 = assign72850_e109929_d_n5;
        locals.var_dnm_dn6 = assign72850_e109929_d_n6;
        locals.var_dnm_dn7 = assign72850_e109929_d_n7;
        locals.var_dnm_dn8 = assign72850_e109929_d_n8;
        locals.var_dnm_dn9 = assign72850_e109929_d_n9;
        locals.var_dnm_dn10 = assign72850_e109929_d_n10;
        locals.var_dnm_dn11 = assign72850_e109929_d_n11;
        locals.var_dnm_dn14 = assign72850_e109929_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign72860_e109942, assign72860_e109942_d_n0, assign72860_e109942_d_n2, assign72860_e109942_d_n4, assign72860_e109942_d_n5, assign72860_e109942_d_n6, assign72860_e109942_d_n7, assign72860_e109942_d_n8, assign72860_e109942_d_n9, assign72860_e109942_d_n10, assign72860_e109942_d_n11, assign72860_e109942_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        let assign72860_e109940: f64 = (locals.var_xp * locals.var_x2);
        (assign72860_e109940, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign72860_e109942;
        locals.var_xp_dn0 = assign72860_e109942_d_n0;
        locals.var_xp_dn2 = assign72860_e109942_d_n2;
        locals.var_xp_dn4 = assign72860_e109942_d_n4;
        locals.var_xp_dn5 = assign72860_e109942_d_n5;
        locals.var_xp_dn6 = assign72860_e109942_d_n6;
        locals.var_xp_dn7 = assign72860_e109942_d_n7;
        locals.var_xp_dn8 = assign72860_e109942_d_n8;
        locals.var_xp_dn9 = assign72860_e109942_d_n9;
        locals.var_xp_dn10 = assign72860_e109942_d_n10;
        locals.var_xp_dn11 = assign72860_e109942_d_n11;
        locals.var_xp_dn14 = assign72860_e109942_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign72870_e109955, assign72870_e109955_d_n0, assign72870_e109955_d_n2, assign72870_e109955_d_n4, assign72870_e109955_d_n5, assign72870_e109955_d_n6, assign72870_e109955_d_n7, assign72870_e109955_d_n8, assign72870_e109955_d_n9, assign72870_e109955_d_n10, assign72870_e109955_d_n11, assign72870_e109955_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        let assign72870_e109953: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign72870_e109953, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign72870_e109955;
        locals.var_xmp_dn0 = assign72870_e109955_d_n0;
        locals.var_xmp_dn2 = assign72870_e109955_d_n2;
        locals.var_xmp_dn4 = assign72870_e109955_d_n4;
        locals.var_xmp_dn5 = assign72870_e109955_d_n5;
        locals.var_xmp_dn6 = assign72870_e109955_d_n6;
        locals.var_xmp_dn7 = assign72870_e109955_d_n7;
        locals.var_xmp_dn8 = assign72870_e109955_d_n8;
        locals.var_xmp_dn9 = assign72870_e109955_d_n9;
        locals.var_xmp_dn10 = assign72870_e109955_d_n10;
        locals.var_xmp_dn11 = assign72870_e109955_d_n11;
        locals.var_xmp_dn14 = assign72870_e109955_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign72880_e109968, assign72880_e109968_d_n0, assign72880_e109968_d_n2, assign72880_e109968_d_n4, assign72880_e109968_d_n5, assign72880_e109968_d_n6, assign72880_e109968_d_n7, assign72880_e109968_d_n8, assign72880_e109968_d_n9, assign72880_e109968_d_n10, assign72880_e109968_d_n11, assign72880_e109968_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        let assign72880_e109966: f64 = (locals.var_xp * locals.var_x2);
        (assign72880_e109966, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign72880_e109968;
        locals.var_xp_dn0 = assign72880_e109968_d_n0;
        locals.var_xp_dn2 = assign72880_e109968_d_n2;
        locals.var_xp_dn4 = assign72880_e109968_d_n4;
        locals.var_xp_dn5 = assign72880_e109968_d_n5;
        locals.var_xp_dn6 = assign72880_e109968_d_n6;
        locals.var_xp_dn7 = assign72880_e109968_d_n7;
        locals.var_xp_dn8 = assign72880_e109968_d_n8;
        locals.var_xp_dn9 = assign72880_e109968_d_n9;
        locals.var_xp_dn10 = assign72880_e109968_d_n10;
        locals.var_xp_dn11 = assign72880_e109968_d_n11;
        locals.var_xp_dn14 = assign72880_e109968_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign72890_e109981, assign72890_e109981_d_n0, assign72890_e109981_d_n2, assign72890_e109981_d_n4, assign72890_e109981_d_n5, assign72890_e109981_d_n6, assign72890_e109981_d_n7, assign72890_e109981_d_n8, assign72890_e109981_d_n9, assign72890_e109981_d_n10, assign72890_e109981_d_n11, assign72890_e109981_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        let assign72890_e109979: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign72890_e109979, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign72890_e109981;
        locals.var_xmp_dn0 = assign72890_e109981_d_n0;
        locals.var_xmp_dn2 = assign72890_e109981_d_n2;
        locals.var_xmp_dn4 = assign72890_e109981_d_n4;
        locals.var_xmp_dn5 = assign72890_e109981_d_n5;
        locals.var_xmp_dn6 = assign72890_e109981_d_n6;
        locals.var_xmp_dn7 = assign72890_e109981_d_n7;
        locals.var_xmp_dn8 = assign72890_e109981_d_n8;
        locals.var_xmp_dn9 = assign72890_e109981_d_n9;
        locals.var_xmp_dn10 = assign72890_e109981_d_n10;
        locals.var_xmp_dn11 = assign72890_e109981_d_n11;
        locals.var_xmp_dn14 = assign72890_e109981_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign72900_e109994, assign72900_e109994_d_n0, assign72900_e109994_d_n2, assign72900_e109994_d_n4, assign72900_e109994_d_n5, assign72900_e109994_d_n6, assign72900_e109994_d_n7, assign72900_e109994_d_n8, assign72900_e109994_d_n9, assign72900_e109994_d_n10, assign72900_e109994_d_n11, assign72900_e109994_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        let assign72900_e109992: f64 = (locals.var_xp + locals.var_xmp);
        (assign72900_e109992, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign72900_e109994;
        locals.var_arg_dn0 = assign72900_e109994_d_n0;
        locals.var_arg_dn2 = assign72900_e109994_d_n2;
        locals.var_arg_dn4 = assign72900_e109994_d_n4;
        locals.var_arg_dn5 = assign72900_e109994_d_n5;
        locals.var_arg_dn6 = assign72900_e109994_d_n6;
        locals.var_arg_dn7 = assign72900_e109994_d_n7;
        locals.var_arg_dn8 = assign72900_e109994_d_n8;
        locals.var_arg_dn9 = assign72900_e109994_d_n9;
        locals.var_arg_dn10 = assign72900_e109994_d_n10;
        locals.var_arg_dn11 = assign72900_e109994_d_n11;
        locals.var_arg_dn14 = assign72900_e109994_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign72910_e110005, assign72910_e110005_d_n0, assign72910_e110005_d_n2, assign72910_e110005_d_n4, assign72910_e110005_d_n5, assign72910_e110005_d_n6, assign72910_e110005_d_n7, assign72910_e110005_d_n8, assign72910_e110005_d_n9, assign72910_e110005_d_n10, assign72910_e110005_d_n11, assign72910_e110005_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign72910_e110005;
        locals.var_dnm_dn0 = assign72910_e110005_d_n0;
        locals.var_dnm_dn2 = assign72910_e110005_d_n2;
        locals.var_dnm_dn4 = assign72910_e110005_d_n4;
        locals.var_dnm_dn5 = assign72910_e110005_d_n5;
        locals.var_dnm_dn6 = assign72910_e110005_d_n6;
        locals.var_dnm_dn7 = assign72910_e110005_d_n7;
        locals.var_dnm_dn8 = assign72910_e110005_d_n8;
        locals.var_dnm_dn9 = assign72910_e110005_d_n9;
        locals.var_dnm_dn10 = assign72910_e110005_d_n10;
        locals.var_dnm_dn11 = assign72910_e110005_d_n11;
        locals.var_dnm_dn14 = assign72910_e110005_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign72920_e110020: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1691 = assign72920_e110020;
        locals.var_guard1691_rv = 0.0;

        let assign72930_e110023: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1692 = assign72930_e110023;
        locals.var_guard1692_rv = 0.0;

        let (assign72940_e110038,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) && (locals.var_guard1691 != 0.0)) && (locals.var_guard1692 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72940_e110038;
        locals.var_mm_rv = 0.0;

        let assign72950_e110041: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1693 = assign72950_e110041;
        locals.var_guard1693_rv = 0.0;

        let (assign72960_e110059,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) && (locals.var_guard1691 != 0.0)) && (locals.var_guard1692 == 0.0)) && (locals.var_guard1693 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72960_e110059;
        locals.var_mm_rv = 0.0;

        let assign72970_e110062: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1694 = assign72970_e110062;
        locals.var_guard1694_rv = 0.0;

        let (assign72980_e110083,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) && (locals.var_guard1691 != 0.0)) && (locals.var_guard1692 == 0.0)) && (locals.var_guard1693 == 0.0)) && (locals.var_guard1694 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign72980_e110083;
        locals.var_mm_rv = 0.0;

        let assign72990_e110086: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1695 = assign72990_e110086;
        locals.var_guard1695_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_274(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign73000_e110110,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) && (locals.var_guard1691 != 0.0)) && (locals.var_guard1692 == 0.0)) && (locals.var_guard1693 == 0.0)) && (locals.var_guard1694 == 0.0)) && (locals.var_guard1695 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign73000_e110110;
        locals.var_mm_rv = 0.0;

        let (assign73010_e110123,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) && (locals.var_guard1691 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign73010_e110123;
        locals.var_m0_rv = 0.0;

        let mut assign73020_loop_guard: usize = 0;
        while {
            let assign73020_cond_e110137: f64 = if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) && (locals.var_guard1691 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign73020_cond_e110137 != 0.0
        } {
            assign73020_loop_guard += 1;
            assert!(assign73020_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign73020_body0_e110151, assign73020_body0_e110151_d_n0, assign73020_body0_e110151_d_n2, assign73020_body0_e110151_d_n4, assign73020_body0_e110151_d_n5, assign73020_body0_e110151_d_n6, assign73020_body0_e110151_d_n7, assign73020_body0_e110151_d_n8, assign73020_body0_e110151_d_n9, assign73020_body0_e110151_d_n10, assign73020_body0_e110151_d_n11, assign73020_body0_e110151_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) && (locals.var_guard1691 != 0.0)) {
        let assign73020_body0_e110149: f64 = (locals.var_dnm).sqrt();
        (assign73020_body0_e110149, (locals.var_dnm_dn0 / (2.0 * assign73020_body0_e110149)), (locals.var_dnm_dn2 / (2.0 * assign73020_body0_e110149)), (locals.var_dnm_dn4 / (2.0 * assign73020_body0_e110149)), (locals.var_dnm_dn5 / (2.0 * assign73020_body0_e110149)), (locals.var_dnm_dn6 / (2.0 * assign73020_body0_e110149)), (locals.var_dnm_dn7 / (2.0 * assign73020_body0_e110149)), (locals.var_dnm_dn8 / (2.0 * assign73020_body0_e110149)), (locals.var_dnm_dn9 / (2.0 * assign73020_body0_e110149)), (locals.var_dnm_dn10 / (2.0 * assign73020_body0_e110149)), (locals.var_dnm_dn11 / (2.0 * assign73020_body0_e110149)), (locals.var_dnm_dn14 / (2.0 * assign73020_body0_e110149)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign73020_body0_e110151;
            locals.var_dnm_dn0 = assign73020_body0_e110151_d_n0;
            locals.var_dnm_dn2 = assign73020_body0_e110151_d_n2;
            locals.var_dnm_dn4 = assign73020_body0_e110151_d_n4;
            locals.var_dnm_dn5 = assign73020_body0_e110151_d_n5;
            locals.var_dnm_dn6 = assign73020_body0_e110151_d_n6;
            locals.var_dnm_dn7 = assign73020_body0_e110151_d_n7;
            locals.var_dnm_dn8 = assign73020_body0_e110151_d_n8;
            locals.var_dnm_dn9 = assign73020_body0_e110151_d_n9;
            locals.var_dnm_dn10 = assign73020_body0_e110151_d_n10;
            locals.var_dnm_dn11 = assign73020_body0_e110151_d_n11;
            locals.var_dnm_dn14 = assign73020_body0_e110151_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign73020_body1_e110166,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) && (locals.var_guard1691 != 0.0)) {
        let assign73020_body1_e110164: f64 = (locals.var_m0 + 1.0);
        (assign73020_body1_e110164,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign73020_body1_e110166;
            locals.var_m0_rv = 0.0;
        }

        let (assign73030_e110191, assign73030_e110191_d_n0, assign73030_e110191_d_n2, assign73030_e110191_d_n4, assign73030_e110191_d_n5, assign73030_e110191_d_n6, assign73030_e110191_d_n7, assign73030_e110191_d_n8, assign73030_e110191_d_n9, assign73030_e110191_d_n10, assign73030_e110191_d_n11, assign73030_e110191_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) && (locals.var_guard1691 == 0.0)) {
        let (assign73030_e110189, assign73030_e110189_d_n0, assign73030_e110189_d_n2, assign73030_e110189_d_n4, assign73030_e110189_d_n5, assign73030_e110189_d_n6, assign73030_e110189_d_n7, assign73030_e110189_d_n8, assign73030_e110189_d_n9, assign73030_e110189_d_n10, assign73030_e110189_d_n11, assign73030_e110189_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign73030_e110186: f64 = (2.0 * 2.0);
                let assign73030_e110187: f64 = (1.0 / assign73030_e110186);
                let assign73030_e110188: f64 = (locals.var_dnm).powf(assign73030_e110187);
                (assign73030_e110188, if 0.0 == 0.0 && ((assign73030_e110187) as f64).is_finite() && ((assign73030_e110187) as f64).fract() == 0.0 { if assign73030_e110187 == 0.0 { 0.0 } else { (assign73030_e110187 * ((locals.var_dnm).powf(assign73030_e110187 - 1.0) * locals.var_dnm_dn0)) } } else { (assign73030_e110188 * (assign73030_e110187 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73030_e110187) as f64).is_finite() && ((assign73030_e110187) as f64).fract() == 0.0 { if assign73030_e110187 == 0.0 { 0.0 } else { (assign73030_e110187 * ((locals.var_dnm).powf(assign73030_e110187 - 1.0) * locals.var_dnm_dn2)) } } else { (assign73030_e110188 * (assign73030_e110187 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73030_e110187) as f64).is_finite() && ((assign73030_e110187) as f64).fract() == 0.0 { if assign73030_e110187 == 0.0 { 0.0 } else { (assign73030_e110187 * ((locals.var_dnm).powf(assign73030_e110187 - 1.0) * locals.var_dnm_dn4)) } } else { (assign73030_e110188 * (assign73030_e110187 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73030_e110187) as f64).is_finite() && ((assign73030_e110187) as f64).fract() == 0.0 { if assign73030_e110187 == 0.0 { 0.0 } else { (assign73030_e110187 * ((locals.var_dnm).powf(assign73030_e110187 - 1.0) * locals.var_dnm_dn5)) } } else { (assign73030_e110188 * (assign73030_e110187 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73030_e110187) as f64).is_finite() && ((assign73030_e110187) as f64).fract() == 0.0 { if assign73030_e110187 == 0.0 { 0.0 } else { (assign73030_e110187 * ((locals.var_dnm).powf(assign73030_e110187 - 1.0) * locals.var_dnm_dn6)) } } else { (assign73030_e110188 * (assign73030_e110187 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73030_e110187) as f64).is_finite() && ((assign73030_e110187) as f64).fract() == 0.0 { if assign73030_e110187 == 0.0 { 0.0 } else { (assign73030_e110187 * ((locals.var_dnm).powf(assign73030_e110187 - 1.0) * locals.var_dnm_dn7)) } } else { (assign73030_e110188 * (assign73030_e110187 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73030_e110187) as f64).is_finite() && ((assign73030_e110187) as f64).fract() == 0.0 { if assign73030_e110187 == 0.0 { 0.0 } else { (assign73030_e110187 * ((locals.var_dnm).powf(assign73030_e110187 - 1.0) * locals.var_dnm_dn8)) } } else { (assign73030_e110188 * (assign73030_e110187 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73030_e110187) as f64).is_finite() && ((assign73030_e110187) as f64).fract() == 0.0 { if assign73030_e110187 == 0.0 { 0.0 } else { (assign73030_e110187 * ((locals.var_dnm).powf(assign73030_e110187 - 1.0) * locals.var_dnm_dn9)) } } else { (assign73030_e110188 * (assign73030_e110187 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73030_e110187) as f64).is_finite() && ((assign73030_e110187) as f64).fract() == 0.0 { if assign73030_e110187 == 0.0 { 0.0 } else { (assign73030_e110187 * ((locals.var_dnm).powf(assign73030_e110187 - 1.0) * locals.var_dnm_dn10)) } } else { (assign73030_e110188 * (assign73030_e110187 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73030_e110187) as f64).is_finite() && ((assign73030_e110187) as f64).fract() == 0.0 { if assign73030_e110187 == 0.0 { 0.0 } else { (assign73030_e110187 * ((locals.var_dnm).powf(assign73030_e110187 - 1.0) * locals.var_dnm_dn11)) } } else { (assign73030_e110188 * (assign73030_e110187 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73030_e110187) as f64).is_finite() && ((assign73030_e110187) as f64).fract() == 0.0 { if assign73030_e110187 == 0.0 { 0.0 } else { (assign73030_e110187 * ((locals.var_dnm).powf(assign73030_e110187 - 1.0) * locals.var_dnm_dn14)) } } else { (assign73030_e110188 * (assign73030_e110187 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign73030_e110189, assign73030_e110189_d_n0, assign73030_e110189_d_n2, assign73030_e110189_d_n4, assign73030_e110189_d_n5, assign73030_e110189_d_n6, assign73030_e110189_d_n7, assign73030_e110189_d_n8, assign73030_e110189_d_n9, assign73030_e110189_d_n10, assign73030_e110189_d_n11, assign73030_e110189_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign73030_e110191;
        locals.var_dnm_dn0 = assign73030_e110191_d_n0;
        locals.var_dnm_dn2 = assign73030_e110191_d_n2;
        locals.var_dnm_dn4 = assign73030_e110191_d_n4;
        locals.var_dnm_dn5 = assign73030_e110191_d_n5;
        locals.var_dnm_dn6 = assign73030_e110191_d_n6;
        locals.var_dnm_dn7 = assign73030_e110191_d_n7;
        locals.var_dnm_dn8 = assign73030_e110191_d_n8;
        locals.var_dnm_dn9 = assign73030_e110191_d_n9;
        locals.var_dnm_dn10 = assign73030_e110191_d_n10;
        locals.var_dnm_dn11 = assign73030_e110191_d_n11;
        locals.var_dnm_dn14 = assign73030_e110191_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign73040_e110204, assign73040_e110204_d_n0, assign73040_e110204_d_n2, assign73040_e110204_d_n4, assign73040_e110204_d_n5, assign73040_e110204_d_n6, assign73040_e110204_d_n7, assign73040_e110204_d_n8, assign73040_e110204_d_n9, assign73040_e110204_d_n10, assign73040_e110204_d_n11, assign73040_e110204_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        let assign73040_e110202: f64 = (1.0 / locals.var_dnm);
        (assign73040_e110202, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign73040_e110204;
        locals.var_dnm_dn0 = assign73040_e110204_d_n0;
        locals.var_dnm_dn2 = assign73040_e110204_d_n2;
        locals.var_dnm_dn4 = assign73040_e110204_d_n4;
        locals.var_dnm_dn5 = assign73040_e110204_d_n5;
        locals.var_dnm_dn6 = assign73040_e110204_d_n6;
        locals.var_dnm_dn7 = assign73040_e110204_d_n7;
        locals.var_dnm_dn8 = assign73040_e110204_d_n8;
        locals.var_dnm_dn9 = assign73040_e110204_d_n9;
        locals.var_dnm_dn10 = assign73040_e110204_d_n10;
        locals.var_dnm_dn11 = assign73040_e110204_d_n11;
        locals.var_dnm_dn14 = assign73040_e110204_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign73050_e110221, assign73050_e110221_d_n0, assign73050_e110221_d_n2, assign73050_e110221_d_n4, assign73050_e110221_d_n5, assign73050_e110221_d_n6, assign73050_e110221_d_n7, assign73050_e110221_d_n8, assign73050_e110221_d_n9, assign73050_e110221_d_n10, assign73050_e110221_d_n11, assign73050_e110221_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        let assign73050_e110216: f64 = (locals.var_ddriftldc * 0.1);
        let assign73050_e110217: f64 = (locals.var_tmf1 * assign73050_e110216);
        let assign73050_e110219: f64 = (assign73050_e110217 * locals.var_dnm);
        (assign73050_e110219, ((((locals.var_tmf1_dn0 * assign73050_e110216) + (locals.var_tmf1 * (locals.var_ddriftldc_dn0 * 0.1))) * locals.var_dnm) + (assign73050_e110217 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign73050_e110216) + (locals.var_tmf1 * (locals.var_ddriftldc_dn2 * 0.1))) * locals.var_dnm) + (assign73050_e110217 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign73050_e110216) + (locals.var_tmf1 * (locals.var_ddriftldc_dn4 * 0.1))) * locals.var_dnm) + (assign73050_e110217 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign73050_e110216) + (locals.var_tmf1 * (locals.var_ddriftldc_dn5 * 0.1))) * locals.var_dnm) + (assign73050_e110217 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign73050_e110216) + (locals.var_tmf1 * (locals.var_ddriftldc_dn6 * 0.1))) * locals.var_dnm) + (assign73050_e110217 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign73050_e110216) + (locals.var_tmf1 * (locals.var_ddriftldc_dn7 * 0.1))) * locals.var_dnm) + (assign73050_e110217 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign73050_e110216) + (locals.var_tmf1 * (locals.var_ddriftldc_dn8 * 0.1))) * locals.var_dnm) + (assign73050_e110217 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign73050_e110216) + (locals.var_tmf1 * (locals.var_ddriftldc_dn9 * 0.1))) * locals.var_dnm) + (assign73050_e110217 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign73050_e110216) + (locals.var_tmf1 * (locals.var_ddriftldc_dn10 * 0.1))) * locals.var_dnm) + (assign73050_e110217 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign73050_e110216) + (locals.var_tmf1 * (locals.var_ddriftldc_dn11 * 0.1))) * locals.var_dnm) + (assign73050_e110217 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign73050_e110216) + (locals.var_tmf1 * (locals.var_ddriftldc_dn14 * 0.1))) * locals.var_dnm) + (assign73050_e110217 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign73050_e110221;
        locals.var_tmf0_dn0 = assign73050_e110221_d_n0;
        locals.var_tmf0_dn2 = assign73050_e110221_d_n2;
        locals.var_tmf0_dn4 = assign73050_e110221_d_n4;
        locals.var_tmf0_dn5 = assign73050_e110221_d_n5;
        locals.var_tmf0_dn6 = assign73050_e110221_d_n6;
        locals.var_tmf0_dn7 = assign73050_e110221_d_n7;
        locals.var_tmf0_dn8 = assign73050_e110221_d_n8;
        locals.var_tmf0_dn9 = assign73050_e110221_d_n9;
        locals.var_tmf0_dn10 = assign73050_e110221_d_n10;
        locals.var_tmf0_dn11 = assign73050_e110221_d_n11;
        locals.var_tmf0_dn14 = assign73050_e110221_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign73060_e110240, assign73060_e110240_d_n0, assign73060_e110240_d_n2, assign73060_e110240_d_n4, assign73060_e110240_d_n5, assign73060_e110240_d_n6, assign73060_e110240_d_n7, assign73060_e110240_d_n8, assign73060_e110240_d_n9, assign73060_e110240_d_n10, assign73060_e110240_d_n11, assign73060_e110240_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        let assign73060_e110232: f64 = (locals.var_ddriftldc * 0.1);
        let assign73060_e110234: f64 = (assign73060_e110232 * locals.var_xmp);
        let assign73060_e110236: f64 = (assign73060_e110234 * locals.var_dnm);
        let assign73060_e110238: f64 = (assign73060_e110236 / locals.var_arg);
        (assign73060_e110238, ((((((((locals.var_ddriftldc_dn0 * 0.1) * locals.var_xmp) + (assign73060_e110232 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign73060_e110234 * locals.var_dnm_dn0)) * locals.var_arg) - (assign73060_e110236 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn2 * 0.1) * locals.var_xmp) + (assign73060_e110232 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign73060_e110234 * locals.var_dnm_dn2)) * locals.var_arg) - (assign73060_e110236 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn4 * 0.1) * locals.var_xmp) + (assign73060_e110232 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign73060_e110234 * locals.var_dnm_dn4)) * locals.var_arg) - (assign73060_e110236 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn5 * 0.1) * locals.var_xmp) + (assign73060_e110232 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign73060_e110234 * locals.var_dnm_dn5)) * locals.var_arg) - (assign73060_e110236 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn6 * 0.1) * locals.var_xmp) + (assign73060_e110232 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign73060_e110234 * locals.var_dnm_dn6)) * locals.var_arg) - (assign73060_e110236 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn7 * 0.1) * locals.var_xmp) + (assign73060_e110232 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign73060_e110234 * locals.var_dnm_dn7)) * locals.var_arg) - (assign73060_e110236 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn8 * 0.1) * locals.var_xmp) + (assign73060_e110232 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign73060_e110234 * locals.var_dnm_dn8)) * locals.var_arg) - (assign73060_e110236 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn9 * 0.1) * locals.var_xmp) + (assign73060_e110232 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign73060_e110234 * locals.var_dnm_dn9)) * locals.var_arg) - (assign73060_e110236 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn10 * 0.1) * locals.var_xmp) + (assign73060_e110232 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign73060_e110234 * locals.var_dnm_dn10)) * locals.var_arg) - (assign73060_e110236 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn11 * 0.1) * locals.var_xmp) + (assign73060_e110232 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign73060_e110234 * locals.var_dnm_dn11)) * locals.var_arg) - (assign73060_e110236 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn14 * 0.1) * locals.var_xmp) + (assign73060_e110232 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign73060_e110234 * locals.var_dnm_dn14)) * locals.var_arg) - (assign73060_e110236 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign73060_e110240;
        locals.var_t0_dn0 = assign73060_e110240_d_n0;
        locals.var_t0_dn2 = assign73060_e110240_d_n2;
        locals.var_t0_dn4 = assign73060_e110240_d_n4;
        locals.var_t0_dn5 = assign73060_e110240_d_n5;
        locals.var_t0_dn6 = assign73060_e110240_d_n6;
        locals.var_t0_dn7 = assign73060_e110240_d_n7;
        locals.var_t0_dn8 = assign73060_e110240_d_n8;
        locals.var_t0_dn9 = assign73060_e110240_d_n9;
        locals.var_t0_dn10 = assign73060_e110240_d_n10;
        locals.var_t0_dn11 = assign73060_e110240_d_n11;
        locals.var_t0_dn14 = assign73060_e110240_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign73070_e110257, assign73070_e110257_d_n0, assign73070_e110257_d_n2, assign73070_e110257_d_n4, assign73070_e110257_d_n5, assign73070_e110257_d_n6, assign73070_e110257_d_n7, assign73070_e110257_d_n8, assign73070_e110257_d_n9, assign73070_e110257_d_n10, assign73070_e110257_d_n11, assign73070_e110257_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        let assign73070_e110252: f64 = (locals.var_ddriftldc * 0.1);
        let assign73070_e110253: f64 = (locals.var_ddriftldc - assign73070_e110252);
        let assign73070_e110255: f64 = (assign73070_e110253 + locals.var_tmf0);
        (assign73070_e110255, ((locals.var_ddriftldc_dn0 - (locals.var_ddriftldc_dn0 * 0.1)) + locals.var_tmf0_dn0), ((locals.var_ddriftldc_dn2 - (locals.var_ddriftldc_dn2 * 0.1)) + locals.var_tmf0_dn2), ((locals.var_ddriftldc_dn4 - (locals.var_ddriftldc_dn4 * 0.1)) + locals.var_tmf0_dn4), ((locals.var_ddriftldc_dn5 - (locals.var_ddriftldc_dn5 * 0.1)) + locals.var_tmf0_dn5), ((locals.var_ddriftldc_dn6 - (locals.var_ddriftldc_dn6 * 0.1)) + locals.var_tmf0_dn6), ((locals.var_ddriftldc_dn7 - (locals.var_ddriftldc_dn7 * 0.1)) + locals.var_tmf0_dn7), ((locals.var_ddriftldc_dn8 - (locals.var_ddriftldc_dn8 * 0.1)) + locals.var_tmf0_dn8), ((locals.var_ddriftldc_dn9 - (locals.var_ddriftldc_dn9 * 0.1)) + locals.var_tmf0_dn9), ((locals.var_ddriftldc_dn10 - (locals.var_ddriftldc_dn10 * 0.1)) + locals.var_tmf0_dn10), ((locals.var_ddriftldc_dn11 - (locals.var_ddriftldc_dn11 * 0.1)) + locals.var_tmf0_dn11), ((locals.var_ddriftldc_dn14 - (locals.var_ddriftldc_dn14 * 0.1)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign73070_e110257;
        locals.var_t1_dn0 = assign73070_e110257_d_n0;
        locals.var_t1_dn2 = assign73070_e110257_d_n2;
        locals.var_t1_dn4 = assign73070_e110257_d_n4;
        locals.var_t1_dn5 = assign73070_e110257_d_n5;
        locals.var_t1_dn6 = assign73070_e110257_d_n6;
        locals.var_t1_dn7 = assign73070_e110257_d_n7;
        locals.var_t1_dn8 = assign73070_e110257_d_n8;
        locals.var_t1_dn9 = assign73070_e110257_d_n9;
        locals.var_t1_dn10 = assign73070_e110257_d_n10;
        locals.var_t1_dn11 = assign73070_e110257_d_n11;
        locals.var_t1_dn14 = assign73070_e110257_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign73080_e110268, assign73080_e110268_d_n0, assign73080_e110268_d_n2, assign73080_e110268_d_n4, assign73080_e110268_d_n5, assign73080_e110268_d_n6, assign73080_e110268_d_n7, assign73080_e110268_d_n8, assign73080_e110268_d_n9, assign73080_e110268_d_n10, assign73080_e110268_d_n11, assign73080_e110268_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign73080_e110268;
        locals.var_t0_dn0 = assign73080_e110268_d_n0;
        locals.var_t0_dn2 = assign73080_e110268_d_n2;
        locals.var_t0_dn4 = assign73080_e110268_d_n4;
        locals.var_t0_dn5 = assign73080_e110268_d_n5;
        locals.var_t0_dn6 = assign73080_e110268_d_n6;
        locals.var_t0_dn7 = assign73080_e110268_d_n7;
        locals.var_t0_dn8 = assign73080_e110268_d_n8;
        locals.var_t0_dn9 = assign73080_e110268_d_n9;
        locals.var_t0_dn10 = assign73080_e110268_d_n10;
        locals.var_t0_dn11 = assign73080_e110268_d_n11;
        locals.var_t0_dn14 = assign73080_e110268_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign73090_e110280, assign73090_e110280_d_n0, assign73090_e110280_d_n2, assign73090_e110280_d_n4, assign73090_e110280_d_n5, assign73090_e110280_d_n6, assign73090_e110280_d_n7, assign73090_e110280_d_n8, assign73090_e110280_d_n9, assign73090_e110280_d_n10, assign73090_e110280_d_n11, assign73090_e110280_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 == 0.0)) {
        (locals.var_wdld0, locals.var_wdld0_dn0, locals.var_wdld0_dn2, locals.var_wdld0_dn4, locals.var_wdld0_dn5, locals.var_wdld0_dn6, locals.var_wdld0_dn7, locals.var_wdld0_dn8, locals.var_wdld0_dn9, locals.var_wdld0_dn10, locals.var_wdld0_dn11, locals.var_wdld0_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign73090_e110280;
        locals.var_t1_dn0 = assign73090_e110280_d_n0;
        locals.var_t1_dn2 = assign73090_e110280_d_n2;
        locals.var_t1_dn4 = assign73090_e110280_d_n4;
        locals.var_t1_dn5 = assign73090_e110280_d_n5;
        locals.var_t1_dn6 = assign73090_e110280_d_n6;
        locals.var_t1_dn7 = assign73090_e110280_d_n7;
        locals.var_t1_dn8 = assign73090_e110280_d_n8;
        locals.var_t1_dn9 = assign73090_e110280_d_n9;
        locals.var_t1_dn10 = assign73090_e110280_d_n10;
        locals.var_t1_dn11 = assign73090_e110280_d_n11;
        locals.var_t1_dn14 = assign73090_e110280_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign73100_e110292, assign73100_e110292_d_n0, assign73100_e110292_d_n2, assign73100_e110292_d_n4, assign73100_e110292_d_n5, assign73100_e110292_d_n6, assign73100_e110292_d_n7, assign73100_e110292_d_n8, assign73100_e110292_d_n9, assign73100_e110292_d_n10, assign73100_e110292_d_n11, assign73100_e110292_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1690 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign73100_e110292;
        locals.var_t0_dn0 = assign73100_e110292_d_n0;
        locals.var_t0_dn2 = assign73100_e110292_d_n2;
        locals.var_t0_dn4 = assign73100_e110292_d_n4;
        locals.var_t0_dn5 = assign73100_e110292_d_n5;
        locals.var_t0_dn6 = assign73100_e110292_d_n6;
        locals.var_t0_dn7 = assign73100_e110292_d_n7;
        locals.var_t0_dn8 = assign73100_e110292_d_n8;
        locals.var_t0_dn9 = assign73100_e110292_d_n9;
        locals.var_t0_dn10 = assign73100_e110292_d_n10;
        locals.var_t0_dn11 = assign73100_e110292_d_n11;
        locals.var_t0_dn14 = assign73100_e110292_d_n14;
        locals.var_t0_rv = 0.0;

        let assign73110_e110295: f64 = if locals.var_t0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1696 = assign73110_e110295;
        locals.var_guard1696_rv = 0.0;

        let (assign73120_e110308,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 != 0.0)) && (locals.var_guard1696 != 0.0)) {
        let assign73120_e110306: f64 = (locals.var_flg_fd_mode + 2.0);
        (assign73120_e110306,)
    } else {
        (locals.var_flg_fd_mode,)
    }
};
        locals.var_flg_fd_mode = assign73120_e110308;
        locals.var_flg_fd_mode_rv = 0.0;

        let (assign73130_e110323, assign73130_e110323_d_n0, assign73130_e110323_d_n2, assign73130_e110323_d_n4, assign73130_e110323_d_n5, assign73130_e110323_d_n6, assign73130_e110323_d_n7, assign73130_e110323_d_n8, assign73130_e110323_d_n9, assign73130_e110323_d_n10, assign73130_e110323_d_n11, assign73130_e110323_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 == 0.0)) {
        let (assign73130_e110321, assign73130_e110321_d_n0, assign73130_e110321_d_n2, assign73130_e110321_d_n4, assign73130_e110321_d_n5, assign73130_e110321_d_n6, assign73130_e110321_d_n7, assign73130_e110321_d_n8, assign73130_e110321_d_n9, assign73130_e110321_d_n10, assign73130_e110321_d_n11, assign73130_e110321_d_n14,) = {
            if (locals.var_wdld0 <= locals.var_ddriftldc) {
                (locals.var_wdld0, locals.var_wdld0_dn0, locals.var_wdld0_dn2, locals.var_wdld0_dn4, locals.var_wdld0_dn5, locals.var_wdld0_dn6, locals.var_wdld0_dn7, locals.var_wdld0_dn8, locals.var_wdld0_dn9, locals.var_wdld0_dn10, locals.var_wdld0_dn11, locals.var_wdld0_dn14,)
            } else {
                (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
            }
        };
        (assign73130_e110321, assign73130_e110321_d_n0, assign73130_e110321_d_n2, assign73130_e110321_d_n4, assign73130_e110321_d_n5, assign73130_e110321_d_n6, assign73130_e110321_d_n7, assign73130_e110321_d_n8, assign73130_e110321_d_n9, assign73130_e110321_d_n10, assign73130_e110321_d_n11, assign73130_e110321_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign73130_e110323;
        locals.var_t1_dn0 = assign73130_e110323_d_n0;
        locals.var_t1_dn2 = assign73130_e110323_d_n2;
        locals.var_t1_dn4 = assign73130_e110323_d_n4;
        locals.var_t1_dn5 = assign73130_e110323_d_n5;
        locals.var_t1_dn6 = assign73130_e110323_d_n6;
        locals.var_t1_dn7 = assign73130_e110323_d_n7;
        locals.var_t1_dn8 = assign73130_e110323_d_n8;
        locals.var_t1_dn9 = assign73130_e110323_d_n9;
        locals.var_t1_dn10 = assign73130_e110323_d_n10;
        locals.var_t1_dn11 = assign73130_e110323_d_n11;
        locals.var_t1_dn14 = assign73130_e110323_d_n14;
        locals.var_t1_rv = 0.0;

        let assign73140_e110326: f64 = if locals.var_wdld0 >= locals.var_ddriftldc { 1.0 } else { 0.0 };
        locals.var_guard1697 = assign73140_e110326;
        locals.var_guard1697_rv = 0.0;

        let (assign73150_e110340,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1689 == 0.0)) && (locals.var_guard1697 != 0.0)) {
        let assign73150_e110338: f64 = (locals.var_flg_fd_mode + 2.0);
        (assign73150_e110338,)
    } else {
        (locals.var_flg_fd_mode,)
    }
};
        locals.var_flg_fd_mode = assign73150_e110340;
        locals.var_flg_fd_mode_rv = 0.0;

        let assign73160_e110343: f64 = if locals.var_flg_fd_mode >= 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1698 = assign73160_e110343;
        locals.var_guard1698_rv = 0.0;

        let (assign73170_e110352, assign73170_e110352_d_n0, assign73170_e110352_d_n2, assign73170_e110352_d_n4, assign73170_e110352_d_n5, assign73170_e110352_d_n6, assign73170_e110352_d_n7, assign73170_e110352_d_n8, assign73170_e110352_d_n9, assign73170_e110352_d_n10, assign73170_e110352_d_n11, assign73170_e110352_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_bef1, locals.var_ps0ld_bef1_dn0, locals.var_ps0ld_bef1_dn2, locals.var_ps0ld_bef1_dn4, locals.var_ps0ld_bef1_dn5, locals.var_ps0ld_bef1_dn6, locals.var_ps0ld_bef1_dn7, locals.var_ps0ld_bef1_dn8, locals.var_ps0ld_bef1_dn9, locals.var_ps0ld_bef1_dn10, locals.var_ps0ld_bef1_dn11, locals.var_ps0ld_bef1_dn14,)
    }
};
        locals.var_ps0ld_bef1 = assign73170_e110352;
        locals.var_ps0ld_bef1_dn0 = assign73170_e110352_d_n0;
        locals.var_ps0ld_bef1_dn2 = assign73170_e110352_d_n2;
        locals.var_ps0ld_bef1_dn4 = assign73170_e110352_d_n4;
        locals.var_ps0ld_bef1_dn5 = assign73170_e110352_d_n5;
        locals.var_ps0ld_bef1_dn6 = assign73170_e110352_d_n6;
        locals.var_ps0ld_bef1_dn7 = assign73170_e110352_d_n7;
        locals.var_ps0ld_bef1_dn8 = assign73170_e110352_d_n8;
        locals.var_ps0ld_bef1_dn9 = assign73170_e110352_d_n9;
        locals.var_ps0ld_bef1_dn10 = assign73170_e110352_d_n10;
        locals.var_ps0ld_bef1_dn11 = assign73170_e110352_d_n11;
        locals.var_ps0ld_bef1_dn14 = assign73170_e110352_d_n14;
        locals.var_ps0ld_bef1_rv = 0.0;

        let (assign73180_e110363, assign73180_e110363_d_n0, assign73180_e110363_d_n2, assign73180_e110363_d_n4, assign73180_e110363_d_n5, assign73180_e110363_d_n6, assign73180_e110363_d_n7, assign73180_e110363_d_n8, assign73180_e110363_d_n9, assign73180_e110363_d_n10, assign73180_e110363_d_n11, assign73180_e110363_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73180_e110361: f64 = (locals.var_t1 * locals.var_q_nsubld);
        (assign73180_e110361, (locals.var_t1_dn0 * locals.var_q_nsubld), (locals.var_t1_dn2 * locals.var_q_nsubld), (locals.var_t1_dn4 * locals.var_q_nsubld), (locals.var_t1_dn5 * locals.var_q_nsubld), (locals.var_t1_dn6 * locals.var_q_nsubld), (locals.var_t1_dn7 * locals.var_q_nsubld), (locals.var_t1_dn8 * locals.var_q_nsubld), (locals.var_t1_dn9 * locals.var_q_nsubld), (locals.var_t1_dn10 * locals.var_q_nsubld), (locals.var_t1_dn11 * locals.var_q_nsubld), (locals.var_t1_dn14 * locals.var_q_nsubld),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign73180_e110363;
        locals.var_qbuld_dn0 = assign73180_e110363_d_n0;
        locals.var_qbuld_dn2 = assign73180_e110363_d_n2;
        locals.var_qbuld_dn4 = assign73180_e110363_d_n4;
        locals.var_qbuld_dn5 = assign73180_e110363_d_n5;
        locals.var_qbuld_dn6 = assign73180_e110363_d_n6;
        locals.var_qbuld_dn7 = assign73180_e110363_d_n7;
        locals.var_qbuld_dn8 = assign73180_e110363_d_n8;
        locals.var_qbuld_dn9 = assign73180_e110363_d_n9;
        locals.var_qbuld_dn10 = assign73180_e110363_d_n10;
        locals.var_qbuld_dn11 = assign73180_e110363_d_n11;
        locals.var_qbuld_dn14 = assign73180_e110363_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign73190_e110376, assign73190_e110376_d_n0, assign73190_e110376_d_n2, assign73190_e110376_d_n4, assign73190_e110376_d_n5, assign73190_e110376_d_n6, assign73190_e110376_d_n7, assign73190_e110376_d_n8, assign73190_e110376_d_n9, assign73190_e110376_d_n10, assign73190_e110376_d_n11, assign73190_e110376_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) {
        let assign73190_e110373: f64 = (locals.var_qbuld / locals.var_cox0_func);
        let assign73190_e110374: f64 = (locals.var_vgpld - assign73190_e110373);
        (assign73190_e110374, (-(locals.var_qbuld_dn0 / locals.var_cox0_func)), (locals.var_vgpld_dn2 - (locals.var_qbuld_dn2 / locals.var_cox0_func)), (-(locals.var_qbuld_dn4 / locals.var_cox0_func)), (-(locals.var_qbuld_dn5 / locals.var_cox0_func)), (-(locals.var_qbuld_dn6 / locals.var_cox0_func)), (locals.var_vgpld_dn7 - (locals.var_qbuld_dn7 / locals.var_cox0_func)), (locals.var_vgpld_dn8 - (locals.var_qbuld_dn8 / locals.var_cox0_func)), (locals.var_vgpld_dn9 - (locals.var_qbuld_dn9 / locals.var_cox0_func)), (-(locals.var_qbuld_dn10 / locals.var_cox0_func)), (-(locals.var_qbuld_dn11 / locals.var_cox0_func)), (-(locals.var_qbuld_dn14 / locals.var_cox0_func)),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign73190_e110376;
        locals.var_ps0ld_dn0 = assign73190_e110376_d_n0;
        locals.var_ps0ld_dn2 = assign73190_e110376_d_n2;
        locals.var_ps0ld_dn4 = assign73190_e110376_d_n4;
        locals.var_ps0ld_dn5 = assign73190_e110376_d_n5;
        locals.var_ps0ld_dn6 = assign73190_e110376_d_n6;
        locals.var_ps0ld_dn7 = assign73190_e110376_d_n7;
        locals.var_ps0ld_dn8 = assign73190_e110376_d_n8;
        locals.var_ps0ld_dn9 = assign73190_e110376_d_n9;
        locals.var_ps0ld_dn10 = assign73190_e110376_d_n10;
        locals.var_ps0ld_dn11 = assign73190_e110376_d_n11;
        locals.var_ps0ld_dn14 = assign73190_e110376_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let assign73200_e110379: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1699 = assign73200_e110379;
        locals.var_guard1699_rv = 0.0;

        let assign73210_e110383: f64 = (locals.var_ps0ld_bef1 - 0.1);
        let assign73210_e110388: f64 = if ((locals.var_ps0ld > assign73210_e110383) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1700 = assign73210_e110388;
        locals.var_guard1700_rv = 0.0;

        let (assign73220_e110405, assign73220_e110405_d_n0, assign73220_e110405_d_n2, assign73220_e110405_d_n4, assign73220_e110405_d_n5, assign73220_e110405_d_n6, assign73220_e110405_d_n7, assign73220_e110405_d_n8, assign73220_e110405_d_n9, assign73220_e110405_d_n10, assign73220_e110405_d_n11, assign73220_e110405_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        let assign73220_e110401: f64 = (locals.var_ps0ld - locals.var_ps0ld_bef1);
        let assign73220_e110403: f64 = (assign73220_e110401 + 0.1);
        (assign73220_e110403, (locals.var_ps0ld_dn0 - locals.var_ps0ld_bef1_dn0), (locals.var_ps0ld_dn2 - locals.var_ps0ld_bef1_dn2), (locals.var_ps0ld_dn4 - locals.var_ps0ld_bef1_dn4), (locals.var_ps0ld_dn5 - locals.var_ps0ld_bef1_dn5), (locals.var_ps0ld_dn6 - locals.var_ps0ld_bef1_dn6), (locals.var_ps0ld_dn7 - locals.var_ps0ld_bef1_dn7), (locals.var_ps0ld_dn8 - locals.var_ps0ld_bef1_dn8), (locals.var_ps0ld_dn9 - locals.var_ps0ld_bef1_dn9), (locals.var_ps0ld_dn10 - locals.var_ps0ld_bef1_dn10), (locals.var_ps0ld_dn11 - locals.var_ps0ld_bef1_dn11), (locals.var_ps0ld_dn14 - locals.var_ps0ld_bef1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign73220_e110405;
        locals.var_tmf1_dn0 = assign73220_e110405_d_n0;
        locals.var_tmf1_dn2 = assign73220_e110405_d_n2;
        locals.var_tmf1_dn4 = assign73220_e110405_d_n4;
        locals.var_tmf1_dn5 = assign73220_e110405_d_n5;
        locals.var_tmf1_dn6 = assign73220_e110405_d_n6;
        locals.var_tmf1_dn7 = assign73220_e110405_d_n7;
        locals.var_tmf1_dn8 = assign73220_e110405_d_n8;
        locals.var_tmf1_dn9 = assign73220_e110405_d_n9;
        locals.var_tmf1_dn10 = assign73220_e110405_d_n10;
        locals.var_tmf1_dn11 = assign73220_e110405_d_n11;
        locals.var_tmf1_dn14 = assign73220_e110405_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign73230_e110420, assign73230_e110420_d_n0, assign73230_e110420_d_n2, assign73230_e110420_d_n4, assign73230_e110420_d_n5, assign73230_e110420_d_n6, assign73230_e110420_d_n7, assign73230_e110420_d_n8, assign73230_e110420_d_n9, assign73230_e110420_d_n10, assign73230_e110420_d_n11, assign73230_e110420_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        let assign73230_e110418: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign73230_e110418, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign73230_e110420;
        locals.var_x2_dn0 = assign73230_e110420_d_n0;
        locals.var_x2_dn2 = assign73230_e110420_d_n2;
        locals.var_x2_dn4 = assign73230_e110420_d_n4;
        locals.var_x2_dn5 = assign73230_e110420_d_n5;
        locals.var_x2_dn6 = assign73230_e110420_d_n6;
        locals.var_x2_dn7 = assign73230_e110420_d_n7;
        locals.var_x2_dn8 = assign73230_e110420_d_n8;
        locals.var_x2_dn9 = assign73230_e110420_d_n9;
        locals.var_x2_dn10 = assign73230_e110420_d_n10;
        locals.var_x2_dn11 = assign73230_e110420_d_n11;
        locals.var_x2_dn14 = assign73230_e110420_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign73240_e110435, assign73240_e110435_d_n0, assign73240_e110435_d_n2, assign73240_e110435_d_n4, assign73240_e110435_d_n5, assign73240_e110435_d_n6, assign73240_e110435_d_n7, assign73240_e110435_d_n8, assign73240_e110435_d_n9, assign73240_e110435_d_n10, assign73240_e110435_d_n11, assign73240_e110435_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        let assign73240_e110433: f64 = (0.1 * 0.1);
        (assign73240_e110433, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign73240_e110435;
        locals.var_xmax2_dn0 = assign73240_e110435_d_n0;
        locals.var_xmax2_dn2 = assign73240_e110435_d_n2;
        locals.var_xmax2_dn4 = assign73240_e110435_d_n4;
        locals.var_xmax2_dn5 = assign73240_e110435_d_n5;
        locals.var_xmax2_dn6 = assign73240_e110435_d_n6;
        locals.var_xmax2_dn7 = assign73240_e110435_d_n7;
        locals.var_xmax2_dn8 = assign73240_e110435_d_n8;
        locals.var_xmax2_dn9 = assign73240_e110435_d_n9;
        locals.var_xmax2_dn10 = assign73240_e110435_d_n10;
        locals.var_xmax2_dn11 = assign73240_e110435_d_n11;
        locals.var_xmax2_dn14 = assign73240_e110435_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign73250_e110448, assign73250_e110448_d_n0, assign73250_e110448_d_n2, assign73250_e110448_d_n4, assign73250_e110448_d_n5, assign73250_e110448_d_n6, assign73250_e110448_d_n7, assign73250_e110448_d_n8, assign73250_e110448_d_n9, assign73250_e110448_d_n10, assign73250_e110448_d_n11, assign73250_e110448_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign73250_e110448;
        locals.var_xp_dn0 = assign73250_e110448_d_n0;
        locals.var_xp_dn2 = assign73250_e110448_d_n2;
        locals.var_xp_dn4 = assign73250_e110448_d_n4;
        locals.var_xp_dn5 = assign73250_e110448_d_n5;
        locals.var_xp_dn6 = assign73250_e110448_d_n6;
        locals.var_xp_dn7 = assign73250_e110448_d_n7;
        locals.var_xp_dn8 = assign73250_e110448_d_n8;
        locals.var_xp_dn9 = assign73250_e110448_d_n9;
        locals.var_xp_dn10 = assign73250_e110448_d_n10;
        locals.var_xp_dn11 = assign73250_e110448_d_n11;
        locals.var_xp_dn14 = assign73250_e110448_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign73260_e110461, assign73260_e110461_d_n0, assign73260_e110461_d_n2, assign73260_e110461_d_n4, assign73260_e110461_d_n5, assign73260_e110461_d_n6, assign73260_e110461_d_n7, assign73260_e110461_d_n8, assign73260_e110461_d_n9, assign73260_e110461_d_n10, assign73260_e110461_d_n11, assign73260_e110461_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign73260_e110461;
        locals.var_xmp_dn0 = assign73260_e110461_d_n0;
        locals.var_xmp_dn2 = assign73260_e110461_d_n2;
        locals.var_xmp_dn4 = assign73260_e110461_d_n4;
        locals.var_xmp_dn5 = assign73260_e110461_d_n5;
        locals.var_xmp_dn6 = assign73260_e110461_d_n6;
        locals.var_xmp_dn7 = assign73260_e110461_d_n7;
        locals.var_xmp_dn8 = assign73260_e110461_d_n8;
        locals.var_xmp_dn9 = assign73260_e110461_d_n9;
        locals.var_xmp_dn10 = assign73260_e110461_d_n10;
        locals.var_xmp_dn11 = assign73260_e110461_d_n11;
        locals.var_xmp_dn14 = assign73260_e110461_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign73270_e110474,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign73270_e110474;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_275(
        locals: &mut StampLocals,
    ) {
        let (assign73280_e110487,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign73280_e110487;
        locals.var_mm_rv = 0.0;

        let (assign73290_e110500, assign73290_e110500_d_n0, assign73290_e110500_d_n2, assign73290_e110500_d_n4, assign73290_e110500_d_n5, assign73290_e110500_d_n6, assign73290_e110500_d_n7, assign73290_e110500_d_n8, assign73290_e110500_d_n9, assign73290_e110500_d_n10, assign73290_e110500_d_n11, assign73290_e110500_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign73290_e110500;
        locals.var_arg_dn0 = assign73290_e110500_d_n0;
        locals.var_arg_dn2 = assign73290_e110500_d_n2;
        locals.var_arg_dn4 = assign73290_e110500_d_n4;
        locals.var_arg_dn5 = assign73290_e110500_d_n5;
        locals.var_arg_dn6 = assign73290_e110500_d_n6;
        locals.var_arg_dn7 = assign73290_e110500_d_n7;
        locals.var_arg_dn8 = assign73290_e110500_d_n8;
        locals.var_arg_dn9 = assign73290_e110500_d_n9;
        locals.var_arg_dn10 = assign73290_e110500_d_n10;
        locals.var_arg_dn11 = assign73290_e110500_d_n11;
        locals.var_arg_dn14 = assign73290_e110500_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign73300_e110513, assign73300_e110513_d_n0, assign73300_e110513_d_n2, assign73300_e110513_d_n4, assign73300_e110513_d_n5, assign73300_e110513_d_n6, assign73300_e110513_d_n7, assign73300_e110513_d_n8, assign73300_e110513_d_n9, assign73300_e110513_d_n10, assign73300_e110513_d_n11, assign73300_e110513_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign73300_e110513;
        locals.var_dnm_dn0 = assign73300_e110513_d_n0;
        locals.var_dnm_dn2 = assign73300_e110513_d_n2;
        locals.var_dnm_dn4 = assign73300_e110513_d_n4;
        locals.var_dnm_dn5 = assign73300_e110513_d_n5;
        locals.var_dnm_dn6 = assign73300_e110513_d_n6;
        locals.var_dnm_dn7 = assign73300_e110513_d_n7;
        locals.var_dnm_dn8 = assign73300_e110513_d_n8;
        locals.var_dnm_dn9 = assign73300_e110513_d_n9;
        locals.var_dnm_dn10 = assign73300_e110513_d_n10;
        locals.var_dnm_dn11 = assign73300_e110513_d_n11;
        locals.var_dnm_dn14 = assign73300_e110513_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign73310_e110528, assign73310_e110528_d_n0, assign73310_e110528_d_n2, assign73310_e110528_d_n4, assign73310_e110528_d_n5, assign73310_e110528_d_n6, assign73310_e110528_d_n7, assign73310_e110528_d_n8, assign73310_e110528_d_n9, assign73310_e110528_d_n10, assign73310_e110528_d_n11, assign73310_e110528_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        let assign73310_e110526: f64 = (locals.var_xp * locals.var_x2);
        (assign73310_e110526, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign73310_e110528;
        locals.var_xp_dn0 = assign73310_e110528_d_n0;
        locals.var_xp_dn2 = assign73310_e110528_d_n2;
        locals.var_xp_dn4 = assign73310_e110528_d_n4;
        locals.var_xp_dn5 = assign73310_e110528_d_n5;
        locals.var_xp_dn6 = assign73310_e110528_d_n6;
        locals.var_xp_dn7 = assign73310_e110528_d_n7;
        locals.var_xp_dn8 = assign73310_e110528_d_n8;
        locals.var_xp_dn9 = assign73310_e110528_d_n9;
        locals.var_xp_dn10 = assign73310_e110528_d_n10;
        locals.var_xp_dn11 = assign73310_e110528_d_n11;
        locals.var_xp_dn14 = assign73310_e110528_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign73320_e110543, assign73320_e110543_d_n0, assign73320_e110543_d_n2, assign73320_e110543_d_n4, assign73320_e110543_d_n5, assign73320_e110543_d_n6, assign73320_e110543_d_n7, assign73320_e110543_d_n8, assign73320_e110543_d_n9, assign73320_e110543_d_n10, assign73320_e110543_d_n11, assign73320_e110543_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        let assign73320_e110541: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign73320_e110541, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign73320_e110543;
        locals.var_xmp_dn0 = assign73320_e110543_d_n0;
        locals.var_xmp_dn2 = assign73320_e110543_d_n2;
        locals.var_xmp_dn4 = assign73320_e110543_d_n4;
        locals.var_xmp_dn5 = assign73320_e110543_d_n5;
        locals.var_xmp_dn6 = assign73320_e110543_d_n6;
        locals.var_xmp_dn7 = assign73320_e110543_d_n7;
        locals.var_xmp_dn8 = assign73320_e110543_d_n8;
        locals.var_xmp_dn9 = assign73320_e110543_d_n9;
        locals.var_xmp_dn10 = assign73320_e110543_d_n10;
        locals.var_xmp_dn11 = assign73320_e110543_d_n11;
        locals.var_xmp_dn14 = assign73320_e110543_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign73330_e110558, assign73330_e110558_d_n0, assign73330_e110558_d_n2, assign73330_e110558_d_n4, assign73330_e110558_d_n5, assign73330_e110558_d_n6, assign73330_e110558_d_n7, assign73330_e110558_d_n8, assign73330_e110558_d_n9, assign73330_e110558_d_n10, assign73330_e110558_d_n11, assign73330_e110558_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        let assign73330_e110556: f64 = (locals.var_xp * locals.var_x2);
        (assign73330_e110556, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign73330_e110558;
        locals.var_xp_dn0 = assign73330_e110558_d_n0;
        locals.var_xp_dn2 = assign73330_e110558_d_n2;
        locals.var_xp_dn4 = assign73330_e110558_d_n4;
        locals.var_xp_dn5 = assign73330_e110558_d_n5;
        locals.var_xp_dn6 = assign73330_e110558_d_n6;
        locals.var_xp_dn7 = assign73330_e110558_d_n7;
        locals.var_xp_dn8 = assign73330_e110558_d_n8;
        locals.var_xp_dn9 = assign73330_e110558_d_n9;
        locals.var_xp_dn10 = assign73330_e110558_d_n10;
        locals.var_xp_dn11 = assign73330_e110558_d_n11;
        locals.var_xp_dn14 = assign73330_e110558_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign73340_e110573, assign73340_e110573_d_n0, assign73340_e110573_d_n2, assign73340_e110573_d_n4, assign73340_e110573_d_n5, assign73340_e110573_d_n6, assign73340_e110573_d_n7, assign73340_e110573_d_n8, assign73340_e110573_d_n9, assign73340_e110573_d_n10, assign73340_e110573_d_n11, assign73340_e110573_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        let assign73340_e110571: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign73340_e110571, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign73340_e110573;
        locals.var_xmp_dn0 = assign73340_e110573_d_n0;
        locals.var_xmp_dn2 = assign73340_e110573_d_n2;
        locals.var_xmp_dn4 = assign73340_e110573_d_n4;
        locals.var_xmp_dn5 = assign73340_e110573_d_n5;
        locals.var_xmp_dn6 = assign73340_e110573_d_n6;
        locals.var_xmp_dn7 = assign73340_e110573_d_n7;
        locals.var_xmp_dn8 = assign73340_e110573_d_n8;
        locals.var_xmp_dn9 = assign73340_e110573_d_n9;
        locals.var_xmp_dn10 = assign73340_e110573_d_n10;
        locals.var_xmp_dn11 = assign73340_e110573_d_n11;
        locals.var_xmp_dn14 = assign73340_e110573_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign73350_e110588, assign73350_e110588_d_n0, assign73350_e110588_d_n2, assign73350_e110588_d_n4, assign73350_e110588_d_n5, assign73350_e110588_d_n6, assign73350_e110588_d_n7, assign73350_e110588_d_n8, assign73350_e110588_d_n9, assign73350_e110588_d_n10, assign73350_e110588_d_n11, assign73350_e110588_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        let assign73350_e110586: f64 = (locals.var_xp + locals.var_xmp);
        (assign73350_e110586, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign73350_e110588;
        locals.var_arg_dn0 = assign73350_e110588_d_n0;
        locals.var_arg_dn2 = assign73350_e110588_d_n2;
        locals.var_arg_dn4 = assign73350_e110588_d_n4;
        locals.var_arg_dn5 = assign73350_e110588_d_n5;
        locals.var_arg_dn6 = assign73350_e110588_d_n6;
        locals.var_arg_dn7 = assign73350_e110588_d_n7;
        locals.var_arg_dn8 = assign73350_e110588_d_n8;
        locals.var_arg_dn9 = assign73350_e110588_d_n9;
        locals.var_arg_dn10 = assign73350_e110588_d_n10;
        locals.var_arg_dn11 = assign73350_e110588_d_n11;
        locals.var_arg_dn14 = assign73350_e110588_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign73360_e110601, assign73360_e110601_d_n0, assign73360_e110601_d_n2, assign73360_e110601_d_n4, assign73360_e110601_d_n5, assign73360_e110601_d_n6, assign73360_e110601_d_n7, assign73360_e110601_d_n8, assign73360_e110601_d_n9, assign73360_e110601_d_n10, assign73360_e110601_d_n11, assign73360_e110601_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign73360_e110601;
        locals.var_dnm_dn0 = assign73360_e110601_d_n0;
        locals.var_dnm_dn2 = assign73360_e110601_d_n2;
        locals.var_dnm_dn4 = assign73360_e110601_d_n4;
        locals.var_dnm_dn5 = assign73360_e110601_d_n5;
        locals.var_dnm_dn6 = assign73360_e110601_d_n6;
        locals.var_dnm_dn7 = assign73360_e110601_d_n7;
        locals.var_dnm_dn8 = assign73360_e110601_d_n8;
        locals.var_dnm_dn9 = assign73360_e110601_d_n9;
        locals.var_dnm_dn10 = assign73360_e110601_d_n10;
        locals.var_dnm_dn11 = assign73360_e110601_d_n11;
        locals.var_dnm_dn14 = assign73360_e110601_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign73370_e110616: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1701 = assign73370_e110616;
        locals.var_guard1701_rv = 0.0;

        let assign73380_e110619: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1702 = assign73380_e110619;
        locals.var_guard1702_rv = 0.0;

        let (assign73390_e110636,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) && (locals.var_guard1701 != 0.0)) && (locals.var_guard1702 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign73390_e110636;
        locals.var_mm_rv = 0.0;

        let assign73400_e110639: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1703 = assign73400_e110639;
        locals.var_guard1703_rv = 0.0;

        let (assign73410_e110659,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) && (locals.var_guard1701 != 0.0)) && (locals.var_guard1702 == 0.0)) && (locals.var_guard1703 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign73410_e110659;
        locals.var_mm_rv = 0.0;

        let assign73420_e110662: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1704 = assign73420_e110662;
        locals.var_guard1704_rv = 0.0;

        let (assign73430_e110685,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) && (locals.var_guard1701 != 0.0)) && (locals.var_guard1702 == 0.0)) && (locals.var_guard1703 == 0.0)) && (locals.var_guard1704 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign73430_e110685;
        locals.var_mm_rv = 0.0;

        let assign73440_e110688: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1705 = assign73440_e110688;
        locals.var_guard1705_rv = 0.0;

        let (assign73450_e110714,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) && (locals.var_guard1701 != 0.0)) && (locals.var_guard1702 == 0.0)) && (locals.var_guard1703 == 0.0)) && (locals.var_guard1704 == 0.0)) && (locals.var_guard1705 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign73450_e110714;
        locals.var_mm_rv = 0.0;

        let (assign73460_e110729,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) && (locals.var_guard1701 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign73460_e110729;
        locals.var_m0_rv = 0.0;

        let mut assign73470_loop_guard: usize = 0;
        while {
            let assign73470_cond_e110745: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) && (locals.var_guard1701 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign73470_cond_e110745 != 0.0
        } {
            assign73470_loop_guard += 1;
            assert!(assign73470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign73470_body0_e110761, assign73470_body0_e110761_d_n0, assign73470_body0_e110761_d_n2, assign73470_body0_e110761_d_n4, assign73470_body0_e110761_d_n5, assign73470_body0_e110761_d_n6, assign73470_body0_e110761_d_n7, assign73470_body0_e110761_d_n8, assign73470_body0_e110761_d_n9, assign73470_body0_e110761_d_n10, assign73470_body0_e110761_d_n11, assign73470_body0_e110761_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) && (locals.var_guard1701 != 0.0)) {
        let assign73470_body0_e110759: f64 = (locals.var_dnm).sqrt();
        (assign73470_body0_e110759, (locals.var_dnm_dn0 / (2.0 * assign73470_body0_e110759)), (locals.var_dnm_dn2 / (2.0 * assign73470_body0_e110759)), (locals.var_dnm_dn4 / (2.0 * assign73470_body0_e110759)), (locals.var_dnm_dn5 / (2.0 * assign73470_body0_e110759)), (locals.var_dnm_dn6 / (2.0 * assign73470_body0_e110759)), (locals.var_dnm_dn7 / (2.0 * assign73470_body0_e110759)), (locals.var_dnm_dn8 / (2.0 * assign73470_body0_e110759)), (locals.var_dnm_dn9 / (2.0 * assign73470_body0_e110759)), (locals.var_dnm_dn10 / (2.0 * assign73470_body0_e110759)), (locals.var_dnm_dn11 / (2.0 * assign73470_body0_e110759)), (locals.var_dnm_dn14 / (2.0 * assign73470_body0_e110759)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign73470_body0_e110761;
            locals.var_dnm_dn0 = assign73470_body0_e110761_d_n0;
            locals.var_dnm_dn2 = assign73470_body0_e110761_d_n2;
            locals.var_dnm_dn4 = assign73470_body0_e110761_d_n4;
            locals.var_dnm_dn5 = assign73470_body0_e110761_d_n5;
            locals.var_dnm_dn6 = assign73470_body0_e110761_d_n6;
            locals.var_dnm_dn7 = assign73470_body0_e110761_d_n7;
            locals.var_dnm_dn8 = assign73470_body0_e110761_d_n8;
            locals.var_dnm_dn9 = assign73470_body0_e110761_d_n9;
            locals.var_dnm_dn10 = assign73470_body0_e110761_d_n10;
            locals.var_dnm_dn11 = assign73470_body0_e110761_d_n11;
            locals.var_dnm_dn14 = assign73470_body0_e110761_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign73470_body1_e110778,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) && (locals.var_guard1701 != 0.0)) {
        let assign73470_body1_e110776: f64 = (locals.var_m0 + 1.0);
        (assign73470_body1_e110776,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign73470_body1_e110778;
            locals.var_m0_rv = 0.0;
        }

        let (assign73480_e110805, assign73480_e110805_d_n0, assign73480_e110805_d_n2, assign73480_e110805_d_n4, assign73480_e110805_d_n5, assign73480_e110805_d_n6, assign73480_e110805_d_n7, assign73480_e110805_d_n8, assign73480_e110805_d_n9, assign73480_e110805_d_n10, assign73480_e110805_d_n11, assign73480_e110805_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) && (locals.var_guard1701 == 0.0)) {
        let (assign73480_e110803, assign73480_e110803_d_n0, assign73480_e110803_d_n2, assign73480_e110803_d_n4, assign73480_e110803_d_n5, assign73480_e110803_d_n6, assign73480_e110803_d_n7, assign73480_e110803_d_n8, assign73480_e110803_d_n9, assign73480_e110803_d_n10, assign73480_e110803_d_n11, assign73480_e110803_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign73480_e110800: f64 = (2.0 * 2.0);
                let assign73480_e110801: f64 = (1.0 / assign73480_e110800);
                let assign73480_e110802: f64 = (locals.var_dnm).powf(assign73480_e110801);
                (assign73480_e110802, if 0.0 == 0.0 && ((assign73480_e110801) as f64).is_finite() && ((assign73480_e110801) as f64).fract() == 0.0 { if assign73480_e110801 == 0.0 { 0.0 } else { (assign73480_e110801 * ((locals.var_dnm).powf(assign73480_e110801 - 1.0) * locals.var_dnm_dn0)) } } else { (assign73480_e110802 * (assign73480_e110801 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73480_e110801) as f64).is_finite() && ((assign73480_e110801) as f64).fract() == 0.0 { if assign73480_e110801 == 0.0 { 0.0 } else { (assign73480_e110801 * ((locals.var_dnm).powf(assign73480_e110801 - 1.0) * locals.var_dnm_dn2)) } } else { (assign73480_e110802 * (assign73480_e110801 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73480_e110801) as f64).is_finite() && ((assign73480_e110801) as f64).fract() == 0.0 { if assign73480_e110801 == 0.0 { 0.0 } else { (assign73480_e110801 * ((locals.var_dnm).powf(assign73480_e110801 - 1.0) * locals.var_dnm_dn4)) } } else { (assign73480_e110802 * (assign73480_e110801 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73480_e110801) as f64).is_finite() && ((assign73480_e110801) as f64).fract() == 0.0 { if assign73480_e110801 == 0.0 { 0.0 } else { (assign73480_e110801 * ((locals.var_dnm).powf(assign73480_e110801 - 1.0) * locals.var_dnm_dn5)) } } else { (assign73480_e110802 * (assign73480_e110801 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73480_e110801) as f64).is_finite() && ((assign73480_e110801) as f64).fract() == 0.0 { if assign73480_e110801 == 0.0 { 0.0 } else { (assign73480_e110801 * ((locals.var_dnm).powf(assign73480_e110801 - 1.0) * locals.var_dnm_dn6)) } } else { (assign73480_e110802 * (assign73480_e110801 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73480_e110801) as f64).is_finite() && ((assign73480_e110801) as f64).fract() == 0.0 { if assign73480_e110801 == 0.0 { 0.0 } else { (assign73480_e110801 * ((locals.var_dnm).powf(assign73480_e110801 - 1.0) * locals.var_dnm_dn7)) } } else { (assign73480_e110802 * (assign73480_e110801 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73480_e110801) as f64).is_finite() && ((assign73480_e110801) as f64).fract() == 0.0 { if assign73480_e110801 == 0.0 { 0.0 } else { (assign73480_e110801 * ((locals.var_dnm).powf(assign73480_e110801 - 1.0) * locals.var_dnm_dn8)) } } else { (assign73480_e110802 * (assign73480_e110801 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73480_e110801) as f64).is_finite() && ((assign73480_e110801) as f64).fract() == 0.0 { if assign73480_e110801 == 0.0 { 0.0 } else { (assign73480_e110801 * ((locals.var_dnm).powf(assign73480_e110801 - 1.0) * locals.var_dnm_dn9)) } } else { (assign73480_e110802 * (assign73480_e110801 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73480_e110801) as f64).is_finite() && ((assign73480_e110801) as f64).fract() == 0.0 { if assign73480_e110801 == 0.0 { 0.0 } else { (assign73480_e110801 * ((locals.var_dnm).powf(assign73480_e110801 - 1.0) * locals.var_dnm_dn10)) } } else { (assign73480_e110802 * (assign73480_e110801 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73480_e110801) as f64).is_finite() && ((assign73480_e110801) as f64).fract() == 0.0 { if assign73480_e110801 == 0.0 { 0.0 } else { (assign73480_e110801 * ((locals.var_dnm).powf(assign73480_e110801 - 1.0) * locals.var_dnm_dn11)) } } else { (assign73480_e110802 * (assign73480_e110801 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign73480_e110801) as f64).is_finite() && ((assign73480_e110801) as f64).fract() == 0.0 { if assign73480_e110801 == 0.0 { 0.0 } else { (assign73480_e110801 * ((locals.var_dnm).powf(assign73480_e110801 - 1.0) * locals.var_dnm_dn14)) } } else { (assign73480_e110802 * (assign73480_e110801 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign73480_e110803, assign73480_e110803_d_n0, assign73480_e110803_d_n2, assign73480_e110803_d_n4, assign73480_e110803_d_n5, assign73480_e110803_d_n6, assign73480_e110803_d_n7, assign73480_e110803_d_n8, assign73480_e110803_d_n9, assign73480_e110803_d_n10, assign73480_e110803_d_n11, assign73480_e110803_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign73480_e110805;
        locals.var_dnm_dn0 = assign73480_e110805_d_n0;
        locals.var_dnm_dn2 = assign73480_e110805_d_n2;
        locals.var_dnm_dn4 = assign73480_e110805_d_n4;
        locals.var_dnm_dn5 = assign73480_e110805_d_n5;
        locals.var_dnm_dn6 = assign73480_e110805_d_n6;
        locals.var_dnm_dn7 = assign73480_e110805_d_n7;
        locals.var_dnm_dn8 = assign73480_e110805_d_n8;
        locals.var_dnm_dn9 = assign73480_e110805_d_n9;
        locals.var_dnm_dn10 = assign73480_e110805_d_n10;
        locals.var_dnm_dn11 = assign73480_e110805_d_n11;
        locals.var_dnm_dn14 = assign73480_e110805_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign73490_e110820, assign73490_e110820_d_n0, assign73490_e110820_d_n2, assign73490_e110820_d_n4, assign73490_e110820_d_n5, assign73490_e110820_d_n6, assign73490_e110820_d_n7, assign73490_e110820_d_n8, assign73490_e110820_d_n9, assign73490_e110820_d_n10, assign73490_e110820_d_n11, assign73490_e110820_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        let assign73490_e110818: f64 = (1.0 / locals.var_dnm);
        (assign73490_e110818, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign73490_e110820;
        locals.var_dnm_dn0 = assign73490_e110820_d_n0;
        locals.var_dnm_dn2 = assign73490_e110820_d_n2;
        locals.var_dnm_dn4 = assign73490_e110820_d_n4;
        locals.var_dnm_dn5 = assign73490_e110820_d_n5;
        locals.var_dnm_dn6 = assign73490_e110820_d_n6;
        locals.var_dnm_dn7 = assign73490_e110820_d_n7;
        locals.var_dnm_dn8 = assign73490_e110820_d_n8;
        locals.var_dnm_dn9 = assign73490_e110820_d_n9;
        locals.var_dnm_dn10 = assign73490_e110820_d_n10;
        locals.var_dnm_dn11 = assign73490_e110820_d_n11;
        locals.var_dnm_dn14 = assign73490_e110820_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign73500_e110837, assign73500_e110837_d_n0, assign73500_e110837_d_n2, assign73500_e110837_d_n4, assign73500_e110837_d_n5, assign73500_e110837_d_n6, assign73500_e110837_d_n7, assign73500_e110837_d_n8, assign73500_e110837_d_n9, assign73500_e110837_d_n10, assign73500_e110837_d_n11, assign73500_e110837_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        let assign73500_e110833: f64 = (locals.var_tmf1 * 0.1);
        let assign73500_e110835: f64 = (assign73500_e110833 * locals.var_dnm);
        (assign73500_e110835, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign73500_e110833 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign73500_e110833 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign73500_e110833 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign73500_e110833 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign73500_e110833 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign73500_e110833 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign73500_e110833 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign73500_e110833 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign73500_e110833 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign73500_e110833 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign73500_e110833 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign73500_e110837;
        locals.var_tmf0_dn0 = assign73500_e110837_d_n0;
        locals.var_tmf0_dn2 = assign73500_e110837_d_n2;
        locals.var_tmf0_dn4 = assign73500_e110837_d_n4;
        locals.var_tmf0_dn5 = assign73500_e110837_d_n5;
        locals.var_tmf0_dn6 = assign73500_e110837_d_n6;
        locals.var_tmf0_dn7 = assign73500_e110837_d_n7;
        locals.var_tmf0_dn8 = assign73500_e110837_d_n8;
        locals.var_tmf0_dn9 = assign73500_e110837_d_n9;
        locals.var_tmf0_dn10 = assign73500_e110837_d_n10;
        locals.var_tmf0_dn11 = assign73500_e110837_d_n11;
        locals.var_tmf0_dn14 = assign73500_e110837_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign73510_e110856, assign73510_e110856_d_n0, assign73510_e110856_d_n2, assign73510_e110856_d_n4, assign73510_e110856_d_n5, assign73510_e110856_d_n6, assign73510_e110856_d_n7, assign73510_e110856_d_n8, assign73510_e110856_d_n9, assign73510_e110856_d_n10, assign73510_e110856_d_n11, assign73510_e110856_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        let assign73510_e110850: f64 = (0.1 * locals.var_xmp);
        let assign73510_e110852: f64 = (assign73510_e110850 * locals.var_dnm);
        let assign73510_e110854: f64 = (assign73510_e110852 / locals.var_arg);
        (assign73510_e110854, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign73510_e110850 * locals.var_dnm_dn0)) * locals.var_arg) - (assign73510_e110852 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign73510_e110850 * locals.var_dnm_dn2)) * locals.var_arg) - (assign73510_e110852 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign73510_e110850 * locals.var_dnm_dn4)) * locals.var_arg) - (assign73510_e110852 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign73510_e110850 * locals.var_dnm_dn5)) * locals.var_arg) - (assign73510_e110852 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign73510_e110850 * locals.var_dnm_dn6)) * locals.var_arg) - (assign73510_e110852 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign73510_e110850 * locals.var_dnm_dn7)) * locals.var_arg) - (assign73510_e110852 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign73510_e110850 * locals.var_dnm_dn8)) * locals.var_arg) - (assign73510_e110852 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign73510_e110850 * locals.var_dnm_dn9)) * locals.var_arg) - (assign73510_e110852 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign73510_e110850 * locals.var_dnm_dn10)) * locals.var_arg) - (assign73510_e110852 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign73510_e110850 * locals.var_dnm_dn11)) * locals.var_arg) - (assign73510_e110852 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign73510_e110850 * locals.var_dnm_dn14)) * locals.var_arg) - (assign73510_e110852 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign73510_e110856;
        locals.var_t0_dn0 = assign73510_e110856_d_n0;
        locals.var_t0_dn2 = assign73510_e110856_d_n2;
        locals.var_t0_dn4 = assign73510_e110856_d_n4;
        locals.var_t0_dn5 = assign73510_e110856_d_n5;
        locals.var_t0_dn6 = assign73510_e110856_d_n6;
        locals.var_t0_dn7 = assign73510_e110856_d_n7;
        locals.var_t0_dn8 = assign73510_e110856_d_n8;
        locals.var_t0_dn9 = assign73510_e110856_d_n9;
        locals.var_t0_dn10 = assign73510_e110856_d_n10;
        locals.var_t0_dn11 = assign73510_e110856_d_n11;
        locals.var_t0_dn14 = assign73510_e110856_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign73520_e110873, assign73520_e110873_d_n0, assign73520_e110873_d_n2, assign73520_e110873_d_n4, assign73520_e110873_d_n5, assign73520_e110873_d_n6, assign73520_e110873_d_n7, assign73520_e110873_d_n8, assign73520_e110873_d_n9, assign73520_e110873_d_n10, assign73520_e110873_d_n11, assign73520_e110873_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        let assign73520_e110869: f64 = (locals.var_ps0ld_bef1 - 0.1);
        let assign73520_e110871: f64 = (assign73520_e110869 + locals.var_tmf0);
        (assign73520_e110871, (locals.var_ps0ld_bef1_dn0 + locals.var_tmf0_dn0), (locals.var_ps0ld_bef1_dn2 + locals.var_tmf0_dn2), (locals.var_ps0ld_bef1_dn4 + locals.var_tmf0_dn4), (locals.var_ps0ld_bef1_dn5 + locals.var_tmf0_dn5), (locals.var_ps0ld_bef1_dn6 + locals.var_tmf0_dn6), (locals.var_ps0ld_bef1_dn7 + locals.var_tmf0_dn7), (locals.var_ps0ld_bef1_dn8 + locals.var_tmf0_dn8), (locals.var_ps0ld_bef1_dn9 + locals.var_tmf0_dn9), (locals.var_ps0ld_bef1_dn10 + locals.var_tmf0_dn10), (locals.var_ps0ld_bef1_dn11 + locals.var_tmf0_dn11), (locals.var_ps0ld_bef1_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign73520_e110873;
        locals.var_ps0ld_dn0 = assign73520_e110873_d_n0;
        locals.var_ps0ld_dn2 = assign73520_e110873_d_n2;
        locals.var_ps0ld_dn4 = assign73520_e110873_d_n4;
        locals.var_ps0ld_dn5 = assign73520_e110873_d_n5;
        locals.var_ps0ld_dn6 = assign73520_e110873_d_n6;
        locals.var_ps0ld_dn7 = assign73520_e110873_d_n7;
        locals.var_ps0ld_dn8 = assign73520_e110873_d_n8;
        locals.var_ps0ld_dn9 = assign73520_e110873_d_n9;
        locals.var_ps0ld_dn10 = assign73520_e110873_d_n10;
        locals.var_ps0ld_dn11 = assign73520_e110873_d_n11;
        locals.var_ps0ld_dn14 = assign73520_e110873_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let (assign73530_e110886, assign73530_e110886_d_n0, assign73530_e110886_d_n2, assign73530_e110886_d_n4, assign73530_e110886_d_n5, assign73530_e110886_d_n6, assign73530_e110886_d_n7, assign73530_e110886_d_n8, assign73530_e110886_d_n9, assign73530_e110886_d_n10, assign73530_e110886_d_n11, assign73530_e110886_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign73530_e110886;
        locals.var_t0_dn0 = assign73530_e110886_d_n0;
        locals.var_t0_dn2 = assign73530_e110886_d_n2;
        locals.var_t0_dn4 = assign73530_e110886_d_n4;
        locals.var_t0_dn5 = assign73530_e110886_d_n5;
        locals.var_t0_dn6 = assign73530_e110886_d_n6;
        locals.var_t0_dn7 = assign73530_e110886_d_n7;
        locals.var_t0_dn8 = assign73530_e110886_d_n8;
        locals.var_t0_dn9 = assign73530_e110886_d_n9;
        locals.var_t0_dn10 = assign73530_e110886_d_n10;
        locals.var_t0_dn11 = assign73530_e110886_d_n11;
        locals.var_t0_dn14 = assign73530_e110886_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign73540_e110900, assign73540_e110900_d_n0, assign73540_e110900_d_n2, assign73540_e110900_d_n4, assign73540_e110900_d_n5, assign73540_e110900_d_n6, assign73540_e110900_d_n7, assign73540_e110900_d_n8, assign73540_e110900_d_n9, assign73540_e110900_d_n10, assign73540_e110900_d_n11, assign73540_e110900_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign73540_e110900;
        locals.var_ps0ld_dn0 = assign73540_e110900_d_n0;
        locals.var_ps0ld_dn2 = assign73540_e110900_d_n2;
        locals.var_ps0ld_dn4 = assign73540_e110900_d_n4;
        locals.var_ps0ld_dn5 = assign73540_e110900_d_n5;
        locals.var_ps0ld_dn6 = assign73540_e110900_d_n6;
        locals.var_ps0ld_dn7 = assign73540_e110900_d_n7;
        locals.var_ps0ld_dn8 = assign73540_e110900_d_n8;
        locals.var_ps0ld_dn9 = assign73540_e110900_d_n9;
        locals.var_ps0ld_dn10 = assign73540_e110900_d_n10;
        locals.var_ps0ld_dn11 = assign73540_e110900_d_n11;
        locals.var_ps0ld_dn14 = assign73540_e110900_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let (assign73550_e110914, assign73550_e110914_d_n0, assign73550_e110914_d_n2, assign73550_e110914_d_n4, assign73550_e110914_d_n5, assign73550_e110914_d_n6, assign73550_e110914_d_n7, assign73550_e110914_d_n8, assign73550_e110914_d_n9, assign73550_e110914_d_n10, assign73550_e110914_d_n11, assign73550_e110914_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 != 0.0)) && (locals.var_guard1700 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign73550_e110914;
        locals.var_t0_dn0 = assign73550_e110914_d_n0;
        locals.var_t0_dn2 = assign73550_e110914_d_n2;
        locals.var_t0_dn4 = assign73550_e110914_d_n4;
        locals.var_t0_dn5 = assign73550_e110914_d_n5;
        locals.var_t0_dn6 = assign73550_e110914_d_n6;
        locals.var_t0_dn7 = assign73550_e110914_d_n7;
        locals.var_t0_dn8 = assign73550_e110914_d_n8;
        locals.var_t0_dn9 = assign73550_e110914_d_n9;
        locals.var_t0_dn10 = assign73550_e110914_d_n10;
        locals.var_t0_dn11 = assign73550_e110914_d_n11;
        locals.var_t0_dn14 = assign73550_e110914_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign73560_e110931, assign73560_e110931_d_n0, assign73560_e110931_d_n2, assign73560_e110931_d_n4, assign73560_e110931_d_n5, assign73560_e110931_d_n6, assign73560_e110931_d_n7, assign73560_e110931_d_n8, assign73560_e110931_d_n9, assign73560_e110931_d_n10, assign73560_e110931_d_n11, assign73560_e110931_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1698 != 0.0)) && (locals.var_guard1699 == 0.0)) {
        let (assign73560_e110929, assign73560_e110929_d_n0, assign73560_e110929_d_n2, assign73560_e110929_d_n4, assign73560_e110929_d_n5, assign73560_e110929_d_n6, assign73560_e110929_d_n7, assign73560_e110929_d_n8, assign73560_e110929_d_n9, assign73560_e110929_d_n10, assign73560_e110929_d_n11, assign73560_e110929_d_n14,) = {
            if (locals.var_ps0ld <= locals.var_ps0ld_bef1) {
                (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
            } else {
                (locals.var_ps0ld_bef1, locals.var_ps0ld_bef1_dn0, locals.var_ps0ld_bef1_dn2, locals.var_ps0ld_bef1_dn4, locals.var_ps0ld_bef1_dn5, locals.var_ps0ld_bef1_dn6, locals.var_ps0ld_bef1_dn7, locals.var_ps0ld_bef1_dn8, locals.var_ps0ld_bef1_dn9, locals.var_ps0ld_bef1_dn10, locals.var_ps0ld_bef1_dn11, locals.var_ps0ld_bef1_dn14,)
            }
        };
        (assign73560_e110929, assign73560_e110929_d_n0, assign73560_e110929_d_n2, assign73560_e110929_d_n4, assign73560_e110929_d_n5, assign73560_e110929_d_n6, assign73560_e110929_d_n7, assign73560_e110929_d_n8, assign73560_e110929_d_n9, assign73560_e110929_d_n10, assign73560_e110929_d_n11, assign73560_e110929_d_n14,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign73560_e110931;
        locals.var_ps0ld_dn0 = assign73560_e110931_d_n0;
        locals.var_ps0ld_dn2 = assign73560_e110931_d_n2;
        locals.var_ps0ld_dn4 = assign73560_e110931_d_n4;
        locals.var_ps0ld_dn5 = assign73560_e110931_d_n5;
        locals.var_ps0ld_dn6 = assign73560_e110931_d_n6;
        locals.var_ps0ld_dn7 = assign73560_e110931_d_n7;
        locals.var_ps0ld_dn8 = assign73560_e110931_d_n8;
        locals.var_ps0ld_dn9 = assign73560_e110931_d_n9;
        locals.var_ps0ld_dn10 = assign73560_e110931_d_n10;
        locals.var_ps0ld_dn11 = assign73560_e110931_d_n11;
        locals.var_ps0ld_dn14 = assign73560_e110931_d_n14;
        locals.var_ps0ld_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_276(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign73570_e110938, assign73570_e110938_d_n0, assign73570_e110938_d_n2, assign73570_e110938_d_n4, assign73570_e110938_d_n5, assign73570_e110938_d_n6, assign73570_e110938_d_n7, assign73570_e110938_d_n8, assign73570_e110938_d_n9, assign73570_e110938_d_n10, assign73570_e110938_d_n11, assign73570_e110938_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_ini, locals.var_ps0ld_ini_dn0, locals.var_ps0ld_ini_dn2, locals.var_ps0ld_ini_dn4, locals.var_ps0ld_ini_dn5, locals.var_ps0ld_ini_dn6, locals.var_ps0ld_ini_dn7, locals.var_ps0ld_ini_dn8, locals.var_ps0ld_ini_dn9, locals.var_ps0ld_ini_dn10, locals.var_ps0ld_ini_dn11, locals.var_ps0ld_ini_dn14,)
    }
};
        locals.var_ps0ld_ini = assign73570_e110938;
        locals.var_ps0ld_ini_dn0 = assign73570_e110938_d_n0;
        locals.var_ps0ld_ini_dn2 = assign73570_e110938_d_n2;
        locals.var_ps0ld_ini_dn4 = assign73570_e110938_d_n4;
        locals.var_ps0ld_ini_dn5 = assign73570_e110938_d_n5;
        locals.var_ps0ld_ini_dn6 = assign73570_e110938_d_n6;
        locals.var_ps0ld_ini_dn7 = assign73570_e110938_d_n7;
        locals.var_ps0ld_ini_dn8 = assign73570_e110938_d_n8;
        locals.var_ps0ld_ini_dn9 = assign73570_e110938_d_n9;
        locals.var_ps0ld_ini_dn10 = assign73570_e110938_d_n10;
        locals.var_ps0ld_ini_dn11 = assign73570_e110938_d_n11;
        locals.var_ps0ld_ini_dn14 = assign73570_e110938_d_n14;
        locals.var_ps0ld_ini_rv = 0.0;

        let assign73580_e110941: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1706 = assign73580_e110941;
        locals.var_guard1706_rv = 0.0;

        let (assign73590_e110950,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign73590_e110950;
        locals.var_flg_conv_rv = 0.0;

        let (assign73600_e110966, assign73600_e110966_d_n0, assign73600_e110966_d_n2, assign73600_e110966_d_n4, assign73600_e110966_d_n5, assign73600_e110966_d_n6, assign73600_e110966_d_n7, assign73600_e110966_d_n8, assign73600_e110966_d_n9, assign73600_e110966_d_n10, assign73600_e110966_d_n11, assign73600_e110966_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73600_e110960: f64 = (1.034943e-10 / locals.var_q_nsubld);
        let assign73600_e110962: f64 = (assign73600_e110960 * locals.var_beta_inv);
        let assign73600_e110963: f64 = (2.0 * assign73600_e110962);
        let assign73600_e110964: f64 = (assign73600_e110963).sqrt();
        (assign73600_e110964, ((2.0 * (assign73600_e110960 * locals.var_beta_inv_dn0)) / (2.0 * assign73600_e110964)), ((2.0 * (assign73600_e110960 * locals.var_beta_inv_dn2)) / (2.0 * assign73600_e110964)), ((2.0 * (assign73600_e110960 * locals.var_beta_inv_dn4)) / (2.0 * assign73600_e110964)), ((2.0 * (assign73600_e110960 * locals.var_beta_inv_dn5)) / (2.0 * assign73600_e110964)), ((2.0 * (assign73600_e110960 * locals.var_beta_inv_dn6)) / (2.0 * assign73600_e110964)), ((2.0 * (assign73600_e110960 * locals.var_beta_inv_dn7)) / (2.0 * assign73600_e110964)), ((2.0 * (assign73600_e110960 * locals.var_beta_inv_dn8)) / (2.0 * assign73600_e110964)), ((2.0 * (assign73600_e110960 * locals.var_beta_inv_dn9)) / (2.0 * assign73600_e110964)), ((2.0 * (assign73600_e110960 * locals.var_beta_inv_dn10)) / (2.0 * assign73600_e110964)), ((2.0 * (assign73600_e110960 * locals.var_beta_inv_dn11)) / (2.0 * assign73600_e110964)), ((2.0 * (assign73600_e110960 * locals.var_beta_inv_dn14)) / (2.0 * assign73600_e110964)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn11, locals.var_c_w_ld_dn14,)
    }
};
        locals.var_c_w_ld = assign73600_e110966;
        locals.var_c_w_ld_dn0 = assign73600_e110966_d_n0;
        locals.var_c_w_ld_dn2 = assign73600_e110966_d_n2;
        locals.var_c_w_ld_dn4 = assign73600_e110966_d_n4;
        locals.var_c_w_ld_dn5 = assign73600_e110966_d_n5;
        locals.var_c_w_ld_dn6 = assign73600_e110966_d_n6;
        locals.var_c_w_ld_dn7 = assign73600_e110966_d_n7;
        locals.var_c_w_ld_dn8 = assign73600_e110966_d_n8;
        locals.var_c_w_ld_dn9 = assign73600_e110966_d_n9;
        locals.var_c_w_ld_dn10 = assign73600_e110966_d_n10;
        locals.var_c_w_ld_dn11 = assign73600_e110966_d_n11;
        locals.var_c_w_ld_dn14 = assign73600_e110966_d_n14;
        locals.var_c_w_ld_rv = 0.0;

        let assign73610_e110969: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1707 = assign73610_e110969;
        locals.var_guard1707_rv = 0.0;

        let (assign73620_e110982, assign73620_e110982_d_n0, assign73620_e110982_d_n2, assign73620_e110982_d_n4, assign73620_e110982_d_n5, assign73620_e110982_d_n6, assign73620_e110982_d_n7, assign73620_e110982_d_n8, assign73620_e110982_d_n9, assign73620_e110982_d_n10, assign73620_e110982_d_n11, assign73620_e110982_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1707 != 0.0)) {
        let assign73620_e110980: f64 = (p.p334 - locals.var_wdep_func);
        (assign73620_e110980, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn11), (-locals.var_wdep_func_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign73620_e110982;
        locals.var_t2_dn0 = assign73620_e110982_d_n0;
        locals.var_t2_dn2 = assign73620_e110982_d_n2;
        locals.var_t2_dn4 = assign73620_e110982_d_n4;
        locals.var_t2_dn5 = assign73620_e110982_d_n5;
        locals.var_t2_dn6 = assign73620_e110982_d_n6;
        locals.var_t2_dn7 = assign73620_e110982_d_n7;
        locals.var_t2_dn8 = assign73620_e110982_d_n8;
        locals.var_t2_dn9 = assign73620_e110982_d_n9;
        locals.var_t2_dn10 = assign73620_e110982_d_n10;
        locals.var_t2_dn11 = assign73620_e110982_d_n11;
        locals.var_t2_dn14 = assign73620_e110982_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign73630_e111007, assign73630_e111007_d_n0, assign73630_e111007_d_n2, assign73630_e111007_d_n4, assign73630_e111007_d_n5, assign73630_e111007_d_n6, assign73630_e111007_d_n7, assign73630_e111007_d_n8, assign73630_e111007_d_n9, assign73630_e111007_d_n10, assign73630_e111007_d_n11, assign73630_e111007_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1707 == 0.0)) {
        let assign73630_e110994: f64 = (locals.var_vdsi + p.p137);
        let assign73630_e110997: f64 = (locals.var_vdsi + p.p137);
        let assign73630_e110998: f64 = (assign73630_e110994 * assign73630_e110997);
        let assign73630_e111001: f64 = (4.0 * 0.1);
        let assign73630_e111003: f64 = (assign73630_e111001 * 0.1);
        let assign73630_e111004: f64 = (assign73630_e110998 + assign73630_e111003);
        let assign73630_e111005: f64 = (assign73630_e111004).sqrt();
        (assign73630_e111005, 0.0, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn6 * assign73630_e110997) + (assign73630_e110994 * locals.var_vdsi_dn6)) / (2.0 * assign73630_e111005)), 0.0, (((locals.var_vdsi_dn8 * assign73630_e110997) + (assign73630_e110994 * locals.var_vdsi_dn8)) / (2.0 * assign73630_e111005)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign73630_e111007;
        locals.var_tmf2_dn0 = assign73630_e111007_d_n0;
        locals.var_tmf2_dn2 = assign73630_e111007_d_n2;
        locals.var_tmf2_dn4 = assign73630_e111007_d_n4;
        locals.var_tmf2_dn5 = assign73630_e111007_d_n5;
        locals.var_tmf2_dn6 = assign73630_e111007_d_n6;
        locals.var_tmf2_dn7 = assign73630_e111007_d_n7;
        locals.var_tmf2_dn8 = assign73630_e111007_d_n8;
        locals.var_tmf2_dn9 = assign73630_e111007_d_n9;
        locals.var_tmf2_dn10 = assign73630_e111007_d_n10;
        locals.var_tmf2_dn11 = assign73630_e111007_d_n11;
        locals.var_tmf2_dn14 = assign73630_e111007_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign73640_e111027, assign73640_e111027_d_n0, assign73640_e111027_d_n2, assign73640_e111027_d_n4, assign73640_e111027_d_n5, assign73640_e111027_d_n6, assign73640_e111027_d_n7, assign73640_e111027_d_n8, assign73640_e111027_d_n9, assign73640_e111027_d_n10, assign73640_e111027_d_n11, assign73640_e111027_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1707 == 0.0)) {
        let assign73640_e111021: f64 = (locals.var_vdsi + p.p137);
        let assign73640_e111023: f64 = (assign73640_e111021 / locals.var_tmf2);
        let assign73640_e111024: f64 = (1.0 + assign73640_e111023);
        let assign73640_e111025: f64 = (0.5 * assign73640_e111024);
        (assign73640_e111025, (0.5 * (-((assign73640_e111021 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign73640_e111021 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign73640_e111021 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign73640_e111021 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn6 * locals.var_tmf2) - (assign73640_e111021 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign73640_e111021 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn8 * locals.var_tmf2) - (assign73640_e111021 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign73640_e111021 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign73640_e111021 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign73640_e111021 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign73640_e111021 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign73640_e111027;
        locals.var_t9_dn0 = assign73640_e111027_d_n0;
        locals.var_t9_dn2 = assign73640_e111027_d_n2;
        locals.var_t9_dn4 = assign73640_e111027_d_n4;
        locals.var_t9_dn5 = assign73640_e111027_d_n5;
        locals.var_t9_dn6 = assign73640_e111027_d_n6;
        locals.var_t9_dn7 = assign73640_e111027_d_n7;
        locals.var_t9_dn8 = assign73640_e111027_d_n8;
        locals.var_t9_dn9 = assign73640_e111027_d_n9;
        locals.var_t9_dn10 = assign73640_e111027_d_n10;
        locals.var_t9_dn11 = assign73640_e111027_d_n11;
        locals.var_t9_dn14 = assign73640_e111027_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign73650_e111045, assign73650_e111045_d_n0, assign73650_e111045_d_n2, assign73650_e111045_d_n4, assign73650_e111045_d_n5, assign73650_e111045_d_n6, assign73650_e111045_d_n7, assign73650_e111045_d_n8, assign73650_e111045_d_n9, assign73650_e111045_d_n10, assign73650_e111045_d_n11, assign73650_e111045_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1707 == 0.0)) {
        let assign73650_e111040: f64 = (locals.var_vdsi + p.p137);
        let assign73650_e111042: f64 = (assign73650_e111040 + locals.var_tmf2);
        let assign73650_e111043: f64 = (0.5 * assign73650_e111042);
        (assign73650_e111043, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * (locals.var_vdsi_dn6 + locals.var_tmf2_dn6)), (0.5 * locals.var_tmf2_dn7), (0.5 * (locals.var_vdsi_dn8 + locals.var_tmf2_dn8)), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign73650_e111045;
        locals.var_t2_dn0 = assign73650_e111045_d_n0;
        locals.var_t2_dn2 = assign73650_e111045_d_n2;
        locals.var_t2_dn4 = assign73650_e111045_d_n4;
        locals.var_t2_dn5 = assign73650_e111045_d_n5;
        locals.var_t2_dn6 = assign73650_e111045_d_n6;
        locals.var_t2_dn7 = assign73650_e111045_d_n7;
        locals.var_t2_dn8 = assign73650_e111045_d_n8;
        locals.var_t2_dn9 = assign73650_e111045_d_n9;
        locals.var_t2_dn10 = assign73650_e111045_d_n10;
        locals.var_t2_dn11 = assign73650_e111045_d_n11;
        locals.var_t2_dn14 = assign73650_e111045_d_n14;
        locals.var_t2_rv = 0.0;

        let assign73660_e111048: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1708 = assign73660_e111048;
        locals.var_guard1708_rv = 0.0;

        let (assign73670_e111062, assign73670_e111062_d_n0, assign73670_e111062_d_n2, assign73670_e111062_d_n4, assign73670_e111062_d_n5, assign73670_e111062_d_n6, assign73670_e111062_d_n7, assign73670_e111062_d_n8, assign73670_e111062_d_n9, assign73670_e111062_d_n10, assign73670_e111062_d_n11, assign73670_e111062_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1707 == 0.0)) && (locals.var_guard1708 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign73670_e111062;
        locals.var_t2_dn0 = assign73670_e111062_d_n0;
        locals.var_t2_dn2 = assign73670_e111062_d_n2;
        locals.var_t2_dn4 = assign73670_e111062_d_n4;
        locals.var_t2_dn5 = assign73670_e111062_d_n5;
        locals.var_t2_dn6 = assign73670_e111062_d_n6;
        locals.var_t2_dn7 = assign73670_e111062_d_n7;
        locals.var_t2_dn8 = assign73670_e111062_d_n8;
        locals.var_t2_dn9 = assign73670_e111062_d_n9;
        locals.var_t2_dn10 = assign73670_e111062_d_n10;
        locals.var_t2_dn11 = assign73670_e111062_d_n11;
        locals.var_t2_dn14 = assign73670_e111062_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign73680_e111076, assign73680_e111076_d_n0, assign73680_e111076_d_n2, assign73680_e111076_d_n4, assign73680_e111076_d_n5, assign73680_e111076_d_n6, assign73680_e111076_d_n7, assign73680_e111076_d_n8, assign73680_e111076_d_n9, assign73680_e111076_d_n10, assign73680_e111076_d_n11, assign73680_e111076_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1707 == 0.0)) && (locals.var_guard1708 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign73680_e111076;
        locals.var_t9_dn0 = assign73680_e111076_d_n0;
        locals.var_t9_dn2 = assign73680_e111076_d_n2;
        locals.var_t9_dn4 = assign73680_e111076_d_n4;
        locals.var_t9_dn5 = assign73680_e111076_d_n5;
        locals.var_t9_dn6 = assign73680_e111076_d_n6;
        locals.var_t9_dn7 = assign73680_e111076_d_n7;
        locals.var_t9_dn8 = assign73680_e111076_d_n8;
        locals.var_t9_dn9 = assign73680_e111076_d_n9;
        locals.var_t9_dn10 = assign73680_e111076_d_n10;
        locals.var_t9_dn11 = assign73680_e111076_d_n11;
        locals.var_t9_dn14 = assign73680_e111076_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign73690_e111093, assign73690_e111093_d_n0, assign73690_e111093_d_n2, assign73690_e111093_d_n4, assign73690_e111093_d_n5, assign73690_e111093_d_n6, assign73690_e111093_d_n7, assign73690_e111093_d_n8, assign73690_e111093_d_n9, assign73690_e111093_d_n10, assign73690_e111093_d_n11, assign73690_e111093_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1707 == 0.0)) {
        let assign73690_e111088: f64 = (locals.var_kjunc * locals.var_t2);
        let assign73690_e111089: f64 = (assign73690_e111088).sqrt();
        let assign73690_e111091: f64 = (assign73690_e111089 * p.p432);
        (assign73690_e111091, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign73690_e111089)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign73690_e111089)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign73690_e111089)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign73690_e111089)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign73690_e111089)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign73690_e111089)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign73690_e111089)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign73690_e111089)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign73690_e111089)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign73690_e111089)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign73690_e111089)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign73690_e111093;
        locals.var_wjunc0_dn0 = assign73690_e111093_d_n0;
        locals.var_wjunc0_dn2 = assign73690_e111093_d_n2;
        locals.var_wjunc0_dn4 = assign73690_e111093_d_n4;
        locals.var_wjunc0_dn5 = assign73690_e111093_d_n5;
        locals.var_wjunc0_dn6 = assign73690_e111093_d_n6;
        locals.var_wjunc0_dn7 = assign73690_e111093_d_n7;
        locals.var_wjunc0_dn8 = assign73690_e111093_d_n8;
        locals.var_wjunc0_dn9 = assign73690_e111093_d_n9;
        locals.var_wjunc0_dn10 = assign73690_e111093_d_n10;
        locals.var_wjunc0_dn11 = assign73690_e111093_d_n11;
        locals.var_wjunc0_dn14 = assign73690_e111093_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign73700_e111107, assign73700_e111107_d_n0, assign73700_e111107_d_n2, assign73700_e111107_d_n4, assign73700_e111107_d_n5, assign73700_e111107_d_n6, assign73700_e111107_d_n7, assign73700_e111107_d_n8, assign73700_e111107_d_n9, assign73700_e111107_d_n10, assign73700_e111107_d_n11, assign73700_e111107_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1707 == 0.0)) {
        let assign73700_e111105: f64 = (p.p334 - locals.var_wjunc0);
        (assign73700_e111105, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign73700_e111107;
        locals.var_t2_dn0 = assign73700_e111107_d_n0;
        locals.var_t2_dn2 = assign73700_e111107_d_n2;
        locals.var_t2_dn4 = assign73700_e111107_d_n4;
        locals.var_t2_dn5 = assign73700_e111107_d_n5;
        locals.var_t2_dn6 = assign73700_e111107_d_n6;
        locals.var_t2_dn7 = assign73700_e111107_d_n7;
        locals.var_t2_dn8 = assign73700_e111107_d_n8;
        locals.var_t2_dn9 = assign73700_e111107_d_n9;
        locals.var_t2_dn10 = assign73700_e111107_d_n10;
        locals.var_t2_dn11 = assign73700_e111107_d_n11;
        locals.var_t2_dn14 = assign73700_e111107_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign73710_e111129, assign73710_e111129_d_n0, assign73710_e111129_d_n2, assign73710_e111129_d_n4, assign73710_e111129_d_n5, assign73710_e111129_d_n6, assign73710_e111129_d_n7, assign73710_e111129_d_n8, assign73710_e111129_d_n9, assign73710_e111129_d_n10, assign73710_e111129_d_n11, assign73710_e111129_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73710_e111116: f64 = (locals.var_t2 * locals.var_t2);
        let assign73710_e111120: f64 = (p.p334 * 0.01);
        let assign73710_e111121: f64 = (4.0 * assign73710_e111120);
        let assign73710_e111124: f64 = (p.p334 * 0.01);
        let assign73710_e111125: f64 = (assign73710_e111121 * assign73710_e111124);
        let assign73710_e111126: f64 = (assign73710_e111116 + assign73710_e111125);
        let assign73710_e111127: f64 = (assign73710_e111126).sqrt();
        (assign73710_e111127, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign73710_e111127)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign73710_e111127)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign73710_e111127)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign73710_e111127)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign73710_e111127)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign73710_e111127)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign73710_e111127)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign73710_e111127)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign73710_e111127)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign73710_e111127)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign73710_e111127)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign73710_e111129;
        locals.var_tmf2_dn0 = assign73710_e111129_d_n0;
        locals.var_tmf2_dn2 = assign73710_e111129_d_n2;
        locals.var_tmf2_dn4 = assign73710_e111129_d_n4;
        locals.var_tmf2_dn5 = assign73710_e111129_d_n5;
        locals.var_tmf2_dn6 = assign73710_e111129_d_n6;
        locals.var_tmf2_dn7 = assign73710_e111129_d_n7;
        locals.var_tmf2_dn8 = assign73710_e111129_d_n8;
        locals.var_tmf2_dn9 = assign73710_e111129_d_n9;
        locals.var_tmf2_dn10 = assign73710_e111129_d_n10;
        locals.var_tmf2_dn11 = assign73710_e111129_d_n11;
        locals.var_tmf2_dn14 = assign73710_e111129_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign73720_e111144, assign73720_e111144_d_n0, assign73720_e111144_d_n2, assign73720_e111144_d_n4, assign73720_e111144_d_n5, assign73720_e111144_d_n6, assign73720_e111144_d_n7, assign73720_e111144_d_n8, assign73720_e111144_d_n9, assign73720_e111144_d_n10, assign73720_e111144_d_n11, assign73720_e111144_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73720_e111140: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign73720_e111141: f64 = (1.0 + assign73720_e111140);
        let assign73720_e111142: f64 = (0.5 * assign73720_e111141);
        (assign73720_e111142, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign73720_e111144;
        locals.var_t9_dn0 = assign73720_e111144_d_n0;
        locals.var_t9_dn2 = assign73720_e111144_d_n2;
        locals.var_t9_dn4 = assign73720_e111144_d_n4;
        locals.var_t9_dn5 = assign73720_e111144_d_n5;
        locals.var_t9_dn6 = assign73720_e111144_d_n6;
        locals.var_t9_dn7 = assign73720_e111144_d_n7;
        locals.var_t9_dn8 = assign73720_e111144_d_n8;
        locals.var_t9_dn9 = assign73720_e111144_d_n9;
        locals.var_t9_dn10 = assign73720_e111144_d_n10;
        locals.var_t9_dn11 = assign73720_e111144_d_n11;
        locals.var_t9_dn14 = assign73720_e111144_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign73730_e111157, assign73730_e111157_d_n0, assign73730_e111157_d_n2, assign73730_e111157_d_n4, assign73730_e111157_d_n5, assign73730_e111157_d_n6, assign73730_e111157_d_n7, assign73730_e111157_d_n8, assign73730_e111157_d_n9, assign73730_e111157_d_n10, assign73730_e111157_d_n11, assign73730_e111157_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73730_e111154: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign73730_e111155: f64 = (0.5 * assign73730_e111154);
        (assign73730_e111155, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign73730_e111157;
        locals.var_t2_dn0 = assign73730_e111157_d_n0;
        locals.var_t2_dn2 = assign73730_e111157_d_n2;
        locals.var_t2_dn4 = assign73730_e111157_d_n4;
        locals.var_t2_dn5 = assign73730_e111157_d_n5;
        locals.var_t2_dn6 = assign73730_e111157_d_n6;
        locals.var_t2_dn7 = assign73730_e111157_d_n7;
        locals.var_t2_dn8 = assign73730_e111157_d_n8;
        locals.var_t2_dn9 = assign73730_e111157_d_n9;
        locals.var_t2_dn10 = assign73730_e111157_d_n10;
        locals.var_t2_dn11 = assign73730_e111157_d_n11;
        locals.var_t2_dn14 = assign73730_e111157_d_n14;
        locals.var_t2_rv = 0.0;

        let assign73740_e111160: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1709 = assign73740_e111160;
        locals.var_guard1709_rv = 0.0;

        let (assign73750_e111171, assign73750_e111171_d_n0, assign73750_e111171_d_n2, assign73750_e111171_d_n4, assign73750_e111171_d_n5, assign73750_e111171_d_n6, assign73750_e111171_d_n7, assign73750_e111171_d_n8, assign73750_e111171_d_n9, assign73750_e111171_d_n10, assign73750_e111171_d_n11, assign73750_e111171_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1709 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign73750_e111171;
        locals.var_t2_dn0 = assign73750_e111171_d_n0;
        locals.var_t2_dn2 = assign73750_e111171_d_n2;
        locals.var_t2_dn4 = assign73750_e111171_d_n4;
        locals.var_t2_dn5 = assign73750_e111171_d_n5;
        locals.var_t2_dn6 = assign73750_e111171_d_n6;
        locals.var_t2_dn7 = assign73750_e111171_d_n7;
        locals.var_t2_dn8 = assign73750_e111171_d_n8;
        locals.var_t2_dn9 = assign73750_e111171_d_n9;
        locals.var_t2_dn10 = assign73750_e111171_d_n10;
        locals.var_t2_dn11 = assign73750_e111171_d_n11;
        locals.var_t2_dn14 = assign73750_e111171_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign73760_e111182, assign73760_e111182_d_n0, assign73760_e111182_d_n2, assign73760_e111182_d_n4, assign73760_e111182_d_n5, assign73760_e111182_d_n6, assign73760_e111182_d_n7, assign73760_e111182_d_n8, assign73760_e111182_d_n9, assign73760_e111182_d_n10, assign73760_e111182_d_n11, assign73760_e111182_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1709 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign73760_e111182;
        locals.var_t9_dn0 = assign73760_e111182_d_n0;
        locals.var_t9_dn2 = assign73760_e111182_d_n2;
        locals.var_t9_dn4 = assign73760_e111182_d_n4;
        locals.var_t9_dn5 = assign73760_e111182_d_n5;
        locals.var_t9_dn6 = assign73760_e111182_d_n6;
        locals.var_t9_dn7 = assign73760_e111182_d_n7;
        locals.var_t9_dn8 = assign73760_e111182_d_n8;
        locals.var_t9_dn9 = assign73760_e111182_d_n9;
        locals.var_t9_dn10 = assign73760_e111182_d_n10;
        locals.var_t9_dn11 = assign73760_e111182_d_n11;
        locals.var_t9_dn14 = assign73760_e111182_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign73770_e111191, assign73770_e111191_d_n0, assign73770_e111191_d_n2, assign73770_e111191_d_n4, assign73770_e111191_d_n5, assign73770_e111191_d_n6, assign73770_e111191_d_n7, assign73770_e111191_d_n8, assign73770_e111191_d_n9, assign73770_e111191_d_n10, assign73770_e111191_d_n11, assign73770_e111191_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign73770_e111191;
        locals.var_ddriftldc_dn0 = assign73770_e111191_d_n0;
        locals.var_ddriftldc_dn2 = assign73770_e111191_d_n2;
        locals.var_ddriftldc_dn4 = assign73770_e111191_d_n4;
        locals.var_ddriftldc_dn5 = assign73770_e111191_d_n5;
        locals.var_ddriftldc_dn6 = assign73770_e111191_d_n6;
        locals.var_ddriftldc_dn7 = assign73770_e111191_d_n7;
        locals.var_ddriftldc_dn8 = assign73770_e111191_d_n8;
        locals.var_ddriftldc_dn9 = assign73770_e111191_d_n9;
        locals.var_ddriftldc_dn10 = assign73770_e111191_d_n10;
        locals.var_ddriftldc_dn11 = assign73770_e111191_d_n11;
        locals.var_ddriftldc_dn14 = assign73770_e111191_d_n14;
        locals.var_ddriftldc_rv = 0.0;

        let (assign73780_e111208, assign73780_e111208_d_n0, assign73780_e111208_d_n2, assign73780_e111208_d_n4, assign73780_e111208_d_n5, assign73780_e111208_d_n6, assign73780_e111208_d_n7, assign73780_e111208_d_n8, assign73780_e111208_d_n9, assign73780_e111208_d_n10, assign73780_e111208_d_n11, assign73780_e111208_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73780_e111200: f64 = (locals.var_q_nsubld * locals.var_ddriftldc);
        let assign73780_e111202: f64 = (assign73780_e111200 * locals.var_ddriftldc);
        let assign73780_e111204: f64 = (assign73780_e111202 / 2.0);
        let assign73780_e111206: f64 = (assign73780_e111204 / 1.034943e-10);
        (assign73780_e111206, (((((locals.var_q_nsubld * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign73780_e111200 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign73780_e111200 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign73780_e111200 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign73780_e111200 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign73780_e111200 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign73780_e111200 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign73780_e111200 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign73780_e111200 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign73780_e111200 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign73780_e111200 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign73780_e111200 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign73780_e111208;
        locals.var_dphi_sb_dn0 = assign73780_e111208_d_n0;
        locals.var_dphi_sb_dn2 = assign73780_e111208_d_n2;
        locals.var_dphi_sb_dn4 = assign73780_e111208_d_n4;
        locals.var_dphi_sb_dn5 = assign73780_e111208_d_n5;
        locals.var_dphi_sb_dn6 = assign73780_e111208_d_n6;
        locals.var_dphi_sb_dn7 = assign73780_e111208_d_n7;
        locals.var_dphi_sb_dn8 = assign73780_e111208_d_n8;
        locals.var_dphi_sb_dn9 = assign73780_e111208_d_n9;
        locals.var_dphi_sb_dn10 = assign73780_e111208_d_n10;
        locals.var_dphi_sb_dn11 = assign73780_e111208_d_n11;
        locals.var_dphi_sb_dn14 = assign73780_e111208_d_n14;
        locals.var_dphi_sb_rv = 0.0;

        let (assign73790_e111222, assign73790_e111222_d_n0, assign73790_e111222_d_n2, assign73790_e111222_d_n4, assign73790_e111222_d_n5, assign73790_e111222_d_n6, assign73790_e111222_d_n7, assign73790_e111222_d_n8, assign73790_e111222_d_n9, assign73790_e111222_d_n10, assign73790_e111222_d_n11, assign73790_e111222_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73790_e111217: f64 = (2.0 * locals.var_beta);
        let assign73790_e111219: f64 = (assign73790_e111217 * locals.var_dphi_sb);
        let assign73790_e111220: f64 = (assign73790_e111219).sqrt();
        (assign73790_e111220, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign73790_e111217 * locals.var_dphi_sb_dn0)) / (2.0 * assign73790_e111220)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign73790_e111217 * locals.var_dphi_sb_dn2)) / (2.0 * assign73790_e111220)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign73790_e111217 * locals.var_dphi_sb_dn4)) / (2.0 * assign73790_e111220)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign73790_e111217 * locals.var_dphi_sb_dn5)) / (2.0 * assign73790_e111220)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign73790_e111217 * locals.var_dphi_sb_dn6)) / (2.0 * assign73790_e111220)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign73790_e111217 * locals.var_dphi_sb_dn7)) / (2.0 * assign73790_e111220)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign73790_e111217 * locals.var_dphi_sb_dn8)) / (2.0 * assign73790_e111220)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign73790_e111217 * locals.var_dphi_sb_dn9)) / (2.0 * assign73790_e111220)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign73790_e111217 * locals.var_dphi_sb_dn10)) / (2.0 * assign73790_e111220)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign73790_e111217 * locals.var_dphi_sb_dn11)) / (2.0 * assign73790_e111220)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign73790_e111217 * locals.var_dphi_sb_dn14)) / (2.0 * assign73790_e111220)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign73790_e111222;
        locals.var_t0_dn0 = assign73790_e111222_d_n0;
        locals.var_t0_dn2 = assign73790_e111222_d_n2;
        locals.var_t0_dn4 = assign73790_e111222_d_n4;
        locals.var_t0_dn5 = assign73790_e111222_d_n5;
        locals.var_t0_dn6 = assign73790_e111222_d_n6;
        locals.var_t0_dn7 = assign73790_e111222_d_n7;
        locals.var_t0_dn8 = assign73790_e111222_d_n8;
        locals.var_t0_dn9 = assign73790_e111222_d_n9;
        locals.var_t0_dn10 = assign73790_e111222_d_n10;
        locals.var_t0_dn11 = assign73790_e111222_d_n11;
        locals.var_t0_dn14 = assign73790_e111222_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign73800_e111238, assign73800_e111238_d_n0, assign73800_e111238_d_n2, assign73800_e111238_d_n4, assign73800_e111238_d_n5, assign73800_e111238_d_n6, assign73800_e111238_d_n7, assign73800_e111238_d_n8, assign73800_e111238_d_n9, assign73800_e111238_d_n10, assign73800_e111238_d_n11, assign73800_e111238_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73800_e111230: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign73800_e111232: f64 = (-locals.var_t0);
        let assign73800_e111233: f64 = { let limited_exp_arg = assign73800_e111232; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign73800_e111234: f64 = (assign73800_e111230 + assign73800_e111233);
        let assign73800_e111236: f64 = (assign73800_e111234 / 2.0);
        (assign73800_e111236, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign73800_e111232; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign73800_e111232; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign73800_e111232; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign73800_e111232; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign73800_e111232; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign73800_e111232; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign73800_e111232; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign73800_e111232; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign73800_e111232; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign73800_e111232; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign73800_e111232; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign73800_e111238;
        locals.var_t1_dn0 = assign73800_e111238_d_n0;
        locals.var_t1_dn2 = assign73800_e111238_d_n2;
        locals.var_t1_dn4 = assign73800_e111238_d_n4;
        locals.var_t1_dn5 = assign73800_e111238_d_n5;
        locals.var_t1_dn6 = assign73800_e111238_d_n6;
        locals.var_t1_dn7 = assign73800_e111238_d_n7;
        locals.var_t1_dn8 = assign73800_e111238_d_n8;
        locals.var_t1_dn9 = assign73800_e111238_d_n9;
        locals.var_t1_dn10 = assign73800_e111238_d_n10;
        locals.var_t1_dn11 = assign73800_e111238_d_n11;
        locals.var_t1_dn14 = assign73800_e111238_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign73810_e111250, assign73810_e111250_d_n0, assign73810_e111250_d_n2, assign73810_e111250_d_n4, assign73810_e111250_d_n5, assign73810_e111250_d_n6, assign73810_e111250_d_n7, assign73810_e111250_d_n8, assign73810_e111250_d_n9, assign73810_e111250_d_n10, assign73810_e111250_d_n11, assign73810_e111250_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73810_e111246: f64 = (locals.var_t1).ln();
        let assign73810_e111248: f64 = (assign73810_e111246 / locals.var_dphi_sb);
        (assign73810_e111248, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign73810_e111246 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign73810_e111246 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign73810_e111246 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign73810_e111246 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign73810_e111246 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign73810_e111246 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign73810_e111246 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign73810_e111246 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign73810_e111246 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign73810_e111246 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign73810_e111246 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign73810_e111250;
        locals.var_c_sb_dn0 = assign73810_e111250_d_n0;
        locals.var_c_sb_dn2 = assign73810_e111250_d_n2;
        locals.var_c_sb_dn4 = assign73810_e111250_d_n4;
        locals.var_c_sb_dn5 = assign73810_e111250_d_n5;
        locals.var_c_sb_dn6 = assign73810_e111250_d_n6;
        locals.var_c_sb_dn7 = assign73810_e111250_d_n7;
        locals.var_c_sb_dn8 = assign73810_e111250_d_n8;
        locals.var_c_sb_dn9 = assign73810_e111250_d_n9;
        locals.var_c_sb_dn10 = assign73810_e111250_d_n10;
        locals.var_c_sb_dn11 = assign73810_e111250_d_n11;
        locals.var_c_sb_dn14 = assign73810_e111250_d_n14;
        locals.var_c_sb_rv = 0.0;

        let (assign73820_e111259,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign73820_e111259;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_277(
        locals: &mut StampLocals,
    ) {
        let mut assign73830_loop_guard: usize = 0;
        while {
            let assign73830_cond_e111269: f64 = (locals.var_lp_s0_max + 1.0);
            let assign73830_cond_e111271: f64 = if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_lp_s0 <= assign73830_cond_e111269)) { 1.0 } else { 0.0 };
            assign73830_cond_e111271 != 0.0
        } {
            assign73830_loop_guard += 1;
            assert!(assign73830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign73830_body3_e111307, assign73830_body3_e111307_d_n0, assign73830_body3_e111307_d_n2, assign73830_body3_e111307_d_n4, assign73830_body3_e111307_d_n5, assign73830_body3_e111307_d_n6, assign73830_body3_e111307_d_n7, assign73830_body3_e111307_d_n8, assign73830_body3_e111307_d_n9, assign73830_body3_e111307_d_n10, assign73830_body3_e111307_d_n11, assign73830_body3_e111307_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73830_body3_e111305: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign73830_body3_e111305, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
            locals.var_ps0ld_vxb = assign73830_body3_e111307;
            locals.var_ps0ld_vxb_dn0 = assign73830_body3_e111307_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign73830_body3_e111307_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign73830_body3_e111307_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign73830_body3_e111307_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign73830_body3_e111307_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign73830_body3_e111307_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign73830_body3_e111307_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign73830_body3_e111307_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign73830_body3_e111307_d_n10;
            locals.var_ps0ld_vxb_dn11 = assign73830_body3_e111307_d_n11;
            locals.var_ps0ld_vxb_dn14 = assign73830_body3_e111307_d_n14;
            locals.var_ps0ld_vxb_rv = 0.0;
            let (assign73830_body4_e111318, assign73830_body4_e111318_d_n0, assign73830_body4_e111318_d_n2, assign73830_body4_e111318_d_n4, assign73830_body4_e111318_d_n5, assign73830_body4_e111318_d_n6, assign73830_body4_e111318_d_n7, assign73830_body4_e111318_d_n8, assign73830_body4_e111318_d_n9, assign73830_body4_e111318_d_n10, assign73830_body4_e111318_d_n11, assign73830_body4_e111318_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73830_body4_e111316: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign73830_body4_e111316, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn11 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn11)), ((locals.var_beta_dn14 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn14)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
            locals.var_chi = assign73830_body4_e111318;
            locals.var_chi_dn0 = assign73830_body4_e111318_d_n0;
            locals.var_chi_dn2 = assign73830_body4_e111318_d_n2;
            locals.var_chi_dn4 = assign73830_body4_e111318_d_n4;
            locals.var_chi_dn5 = assign73830_body4_e111318_d_n5;
            locals.var_chi_dn6 = assign73830_body4_e111318_d_n6;
            locals.var_chi_dn7 = assign73830_body4_e111318_d_n7;
            locals.var_chi_dn8 = assign73830_body4_e111318_d_n8;
            locals.var_chi_dn9 = assign73830_body4_e111318_d_n9;
            locals.var_chi_dn10 = assign73830_body4_e111318_d_n10;
            locals.var_chi_dn11 = assign73830_body4_e111318_d_n11;
            locals.var_chi_dn14 = assign73830_body4_e111318_d_n14;
            locals.var_chi_rv = 0.0;
            let (assign73830_body5_e111331, assign73830_body5_e111331_d_n0, assign73830_body5_e111331_d_n2, assign73830_body5_e111331_d_n4, assign73830_body5_e111331_d_n5, assign73830_body5_e111331_d_n6, assign73830_body5_e111331_d_n7, assign73830_body5_e111331_d_n8, assign73830_body5_e111331_d_n9, assign73830_body5_e111331_d_n10, assign73830_body5_e111331_d_n11, assign73830_body5_e111331_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73830_body5_e111328: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign73830_body5_e111329: f64 = (locals.var_c_sb * assign73830_body5_e111328);
        (assign73830_body5_e111329, ((locals.var_c_sb_dn0 * assign73830_body5_e111328) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign73830_body5_e111328) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign73830_body5_e111328) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign73830_body5_e111328) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign73830_body5_e111328) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign73830_body5_e111328) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign73830_body5_e111328) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign73830_body5_e111328) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign73830_body5_e111328) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign73830_body5_e111328) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign73830_body5_e111328) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
            locals.var_ty = assign73830_body5_e111331;
            locals.var_ty_dn0 = assign73830_body5_e111331_d_n0;
            locals.var_ty_dn2 = assign73830_body5_e111331_d_n2;
            locals.var_ty_dn4 = assign73830_body5_e111331_d_n4;
            locals.var_ty_dn5 = assign73830_body5_e111331_d_n5;
            locals.var_ty_dn6 = assign73830_body5_e111331_d_n6;
            locals.var_ty_dn7 = assign73830_body5_e111331_d_n7;
            locals.var_ty_dn8 = assign73830_body5_e111331_d_n8;
            locals.var_ty_dn9 = assign73830_body5_e111331_d_n9;
            locals.var_ty_dn10 = assign73830_body5_e111331_d_n10;
            locals.var_ty_dn11 = assign73830_body5_e111331_d_n11;
            locals.var_ty_dn14 = assign73830_body5_e111331_d_n14;
            locals.var_ty_rv = 0.0;
            let assign73830_body6_e111334: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1711 = assign73830_body6_e111334;
            locals.var_guard1711_rv = 0.0;
            let (assign73830_body7_e111346, assign73830_body7_e111346_d_n0, assign73830_body7_e111346_d_n2, assign73830_body7_e111346_d_n4, assign73830_body7_e111346_d_n5, assign73830_body7_e111346_d_n6, assign73830_body7_e111346_d_n7, assign73830_body7_e111346_d_n8, assign73830_body7_e111346_d_n9, assign73830_body7_e111346_d_n10, assign73830_body7_e111346_d_n11, assign73830_body7_e111346_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1711 != 0.0)) {
        let assign73830_body7_e111344: f64 = (locals.var_ty).exp();
        (assign73830_body7_e111344, (assign73830_body7_e111344 * locals.var_ty_dn0), (assign73830_body7_e111344 * locals.var_ty_dn2), (assign73830_body7_e111344 * locals.var_ty_dn4), (assign73830_body7_e111344 * locals.var_ty_dn5), (assign73830_body7_e111344 * locals.var_ty_dn6), (assign73830_body7_e111344 * locals.var_ty_dn7), (assign73830_body7_e111344 * locals.var_ty_dn8), (assign73830_body7_e111344 * locals.var_ty_dn9), (assign73830_body7_e111344 * locals.var_ty_dn10), (assign73830_body7_e111344 * locals.var_ty_dn11), (assign73830_body7_e111344 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign73830_body7_e111346;
            locals.var_t1_dn0 = assign73830_body7_e111346_d_n0;
            locals.var_t1_dn2 = assign73830_body7_e111346_d_n2;
            locals.var_t1_dn4 = assign73830_body7_e111346_d_n4;
            locals.var_t1_dn5 = assign73830_body7_e111346_d_n5;
            locals.var_t1_dn6 = assign73830_body7_e111346_d_n6;
            locals.var_t1_dn7 = assign73830_body7_e111346_d_n7;
            locals.var_t1_dn8 = assign73830_body7_e111346_d_n8;
            locals.var_t1_dn9 = assign73830_body7_e111346_d_n9;
            locals.var_t1_dn10 = assign73830_body7_e111346_d_n10;
            locals.var_t1_dn11 = assign73830_body7_e111346_d_n11;
            locals.var_t1_dn14 = assign73830_body7_e111346_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign73830_body8_e111361, assign73830_body8_e111361_d_n0, assign73830_body8_e111361_d_n2, assign73830_body8_e111361_d_n4, assign73830_body8_e111361_d_n5, assign73830_body8_e111361_d_n6, assign73830_body8_e111361_d_n7, assign73830_body8_e111361_d_n8, assign73830_body8_e111361_d_n9, assign73830_body8_e111361_d_n10, assign73830_body8_e111361_d_n11, assign73830_body8_e111361_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1711 != 0.0)) {
        let assign73830_body8_e111356: f64 = (-locals.var_c_sb);
        let assign73830_body8_e111358: f64 = (assign73830_body8_e111356 * locals.var_dphi_sb);
        let assign73830_body8_e111359: f64 = (assign73830_body8_e111358).exp();
        (assign73830_body8_e111359, (assign73830_body8_e111359 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign73830_body8_e111356 * locals.var_dphi_sb_dn0))), (assign73830_body8_e111359 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign73830_body8_e111356 * locals.var_dphi_sb_dn2))), (assign73830_body8_e111359 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign73830_body8_e111356 * locals.var_dphi_sb_dn4))), (assign73830_body8_e111359 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign73830_body8_e111356 * locals.var_dphi_sb_dn5))), (assign73830_body8_e111359 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign73830_body8_e111356 * locals.var_dphi_sb_dn6))), (assign73830_body8_e111359 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign73830_body8_e111356 * locals.var_dphi_sb_dn7))), (assign73830_body8_e111359 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign73830_body8_e111356 * locals.var_dphi_sb_dn8))), (assign73830_body8_e111359 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign73830_body8_e111356 * locals.var_dphi_sb_dn9))), (assign73830_body8_e111359 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign73830_body8_e111356 * locals.var_dphi_sb_dn10))), (assign73830_body8_e111359 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign73830_body8_e111356 * locals.var_dphi_sb_dn11))), (assign73830_body8_e111359 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign73830_body8_e111356 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign73830_body8_e111361;
            locals.var_t0_dn0 = assign73830_body8_e111361_d_n0;
            locals.var_t0_dn2 = assign73830_body8_e111361_d_n2;
            locals.var_t0_dn4 = assign73830_body8_e111361_d_n4;
            locals.var_t0_dn5 = assign73830_body8_e111361_d_n5;
            locals.var_t0_dn6 = assign73830_body8_e111361_d_n6;
            locals.var_t0_dn7 = assign73830_body8_e111361_d_n7;
            locals.var_t0_dn8 = assign73830_body8_e111361_d_n8;
            locals.var_t0_dn9 = assign73830_body8_e111361_d_n9;
            locals.var_t0_dn10 = assign73830_body8_e111361_d_n10;
            locals.var_t0_dn11 = assign73830_body8_e111361_d_n11;
            locals.var_t0_dn14 = assign73830_body8_e111361_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign73830_body9_e111374, assign73830_body9_e111374_d_n0, assign73830_body9_e111374_d_n2, assign73830_body9_e111374_d_n4, assign73830_body9_e111374_d_n5, assign73830_body9_e111374_d_n6, assign73830_body9_e111374_d_n7, assign73830_body9_e111374_d_n8, assign73830_body9_e111374_d_n9, assign73830_body9_e111374_d_n10, assign73830_body9_e111374_d_n11, assign73830_body9_e111374_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1711 != 0.0)) {
        let assign73830_body9_e111372: f64 = (locals.var_t1 - locals.var_t0);
        (assign73830_body9_e111372, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign73830_body9_e111374;
            locals.var_t2_dn0 = assign73830_body9_e111374_d_n0;
            locals.var_t2_dn2 = assign73830_body9_e111374_d_n2;
            locals.var_t2_dn4 = assign73830_body9_e111374_d_n4;
            locals.var_t2_dn5 = assign73830_body9_e111374_d_n5;
            locals.var_t2_dn6 = assign73830_body9_e111374_d_n6;
            locals.var_t2_dn7 = assign73830_body9_e111374_d_n7;
            locals.var_t2_dn8 = assign73830_body9_e111374_d_n8;
            locals.var_t2_dn9 = assign73830_body9_e111374_d_n9;
            locals.var_t2_dn10 = assign73830_body9_e111374_d_n10;
            locals.var_t2_dn11 = assign73830_body9_e111374_d_n11;
            locals.var_t2_dn14 = assign73830_body9_e111374_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign73830_body10_e111390, assign73830_body10_e111390_d_n0, assign73830_body10_e111390_d_n2, assign73830_body10_e111390_d_n4, assign73830_body10_e111390_d_n5, assign73830_body10_e111390_d_n6, assign73830_body10_e111390_d_n7, assign73830_body10_e111390_d_n8, assign73830_body10_e111390_d_n9, assign73830_body10_e111390_d_n10, assign73830_body10_e111390_d_n11, assign73830_body10_e111390_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1711 != 0.0)) {
        let assign73830_body10_e111385: f64 = (1.0 + locals.var_t2);
        let assign73830_body10_e111386: f64 = (assign73830_body10_e111385).ln();
        let assign73830_body10_e111388: f64 = (assign73830_body10_e111386 / locals.var_c_sb);
        (assign73830_body10_e111388, ((((locals.var_t2_dn0 / assign73830_body10_e111385) * locals.var_c_sb) - (assign73830_body10_e111386 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign73830_body10_e111385) * locals.var_c_sb) - (assign73830_body10_e111386 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign73830_body10_e111385) * locals.var_c_sb) - (assign73830_body10_e111386 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign73830_body10_e111385) * locals.var_c_sb) - (assign73830_body10_e111386 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign73830_body10_e111385) * locals.var_c_sb) - (assign73830_body10_e111386 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign73830_body10_e111385) * locals.var_c_sb) - (assign73830_body10_e111386 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign73830_body10_e111385) * locals.var_c_sb) - (assign73830_body10_e111386 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign73830_body10_e111385) * locals.var_c_sb) - (assign73830_body10_e111386 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign73830_body10_e111385) * locals.var_c_sb) - (assign73830_body10_e111386 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign73830_body10_e111385) * locals.var_c_sb) - (assign73830_body10_e111386 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign73830_body10_e111385) * locals.var_c_sb) - (assign73830_body10_e111386 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign73830_body10_e111390;
            locals.var_phi_b_dn0 = assign73830_body10_e111390_d_n0;
            locals.var_phi_b_dn2 = assign73830_body10_e111390_d_n2;
            locals.var_phi_b_dn4 = assign73830_body10_e111390_d_n4;
            locals.var_phi_b_dn5 = assign73830_body10_e111390_d_n5;
            locals.var_phi_b_dn6 = assign73830_body10_e111390_d_n6;
            locals.var_phi_b_dn7 = assign73830_body10_e111390_d_n7;
            locals.var_phi_b_dn8 = assign73830_body10_e111390_d_n8;
            locals.var_phi_b_dn9 = assign73830_body10_e111390_d_n9;
            locals.var_phi_b_dn10 = assign73830_body10_e111390_d_n10;
            locals.var_phi_b_dn11 = assign73830_body10_e111390_d_n11;
            locals.var_phi_b_dn14 = assign73830_body10_e111390_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign73830_body11_e111405, assign73830_body11_e111405_d_n0, assign73830_body11_e111405_d_n2, assign73830_body11_e111405_d_n4, assign73830_body11_e111405_d_n5, assign73830_body11_e111405_d_n6, assign73830_body11_e111405_d_n7, assign73830_body11_e111405_d_n8, assign73830_body11_e111405_d_n9, assign73830_body11_e111405_d_n10, assign73830_body11_e111405_d_n11, assign73830_body11_e111405_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1711 != 0.0)) {
        let assign73830_body11_e111402: f64 = (1.0 + locals.var_t2);
        let assign73830_body11_e111403: f64 = (locals.var_t1 / assign73830_body11_e111402);
        (assign73830_body11_e111403, (((locals.var_t1_dn0 * assign73830_body11_e111402) - (locals.var_t1 * locals.var_t2_dn0)) / (assign73830_body11_e111402 * assign73830_body11_e111402)), (((locals.var_t1_dn2 * assign73830_body11_e111402) - (locals.var_t1 * locals.var_t2_dn2)) / (assign73830_body11_e111402 * assign73830_body11_e111402)), (((locals.var_t1_dn4 * assign73830_body11_e111402) - (locals.var_t1 * locals.var_t2_dn4)) / (assign73830_body11_e111402 * assign73830_body11_e111402)), (((locals.var_t1_dn5 * assign73830_body11_e111402) - (locals.var_t1 * locals.var_t2_dn5)) / (assign73830_body11_e111402 * assign73830_body11_e111402)), (((locals.var_t1_dn6 * assign73830_body11_e111402) - (locals.var_t1 * locals.var_t2_dn6)) / (assign73830_body11_e111402 * assign73830_body11_e111402)), (((locals.var_t1_dn7 * assign73830_body11_e111402) - (locals.var_t1 * locals.var_t2_dn7)) / (assign73830_body11_e111402 * assign73830_body11_e111402)), (((locals.var_t1_dn8 * assign73830_body11_e111402) - (locals.var_t1 * locals.var_t2_dn8)) / (assign73830_body11_e111402 * assign73830_body11_e111402)), (((locals.var_t1_dn9 * assign73830_body11_e111402) - (locals.var_t1 * locals.var_t2_dn9)) / (assign73830_body11_e111402 * assign73830_body11_e111402)), (((locals.var_t1_dn10 * assign73830_body11_e111402) - (locals.var_t1 * locals.var_t2_dn10)) / (assign73830_body11_e111402 * assign73830_body11_e111402)), (((locals.var_t1_dn11 * assign73830_body11_e111402) - (locals.var_t1 * locals.var_t2_dn11)) / (assign73830_body11_e111402 * assign73830_body11_e111402)), (((locals.var_t1_dn14 * assign73830_body11_e111402) - (locals.var_t1 * locals.var_t2_dn14)) / (assign73830_body11_e111402 * assign73830_body11_e111402)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign73830_body11_e111405;
            locals.var_phi_b_dpss_dn0 = assign73830_body11_e111405_d_n0;
            locals.var_phi_b_dpss_dn2 = assign73830_body11_e111405_d_n2;
            locals.var_phi_b_dpss_dn4 = assign73830_body11_e111405_d_n4;
            locals.var_phi_b_dpss_dn5 = assign73830_body11_e111405_d_n5;
            locals.var_phi_b_dpss_dn6 = assign73830_body11_e111405_d_n6;
            locals.var_phi_b_dpss_dn7 = assign73830_body11_e111405_d_n7;
            locals.var_phi_b_dpss_dn8 = assign73830_body11_e111405_d_n8;
            locals.var_phi_b_dpss_dn9 = assign73830_body11_e111405_d_n9;
            locals.var_phi_b_dpss_dn10 = assign73830_body11_e111405_d_n10;
            locals.var_phi_b_dpss_dn11 = assign73830_body11_e111405_d_n11;
            locals.var_phi_b_dpss_dn14 = assign73830_body11_e111405_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign73830_body13_e111433, assign73830_body13_e111433_d_n0, assign73830_body13_e111433_d_n2, assign73830_body13_e111433_d_n4, assign73830_body13_e111433_d_n5, assign73830_body13_e111433_d_n6, assign73830_body13_e111433_d_n7, assign73830_body13_e111433_d_n8, assign73830_body13_e111433_d_n9, assign73830_body13_e111433_d_n10, assign73830_body13_e111433_d_n11, assign73830_body13_e111433_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1711 == 0.0)) {
        let assign73830_body13_e111431: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign73830_body13_e111431, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign73830_body13_e111433;
            locals.var_phi_b_dn0 = assign73830_body13_e111433_d_n0;
            locals.var_phi_b_dn2 = assign73830_body13_e111433_d_n2;
            locals.var_phi_b_dn4 = assign73830_body13_e111433_d_n4;
            locals.var_phi_b_dn5 = assign73830_body13_e111433_d_n5;
            locals.var_phi_b_dn6 = assign73830_body13_e111433_d_n6;
            locals.var_phi_b_dn7 = assign73830_body13_e111433_d_n7;
            locals.var_phi_b_dn8 = assign73830_body13_e111433_d_n8;
            locals.var_phi_b_dn9 = assign73830_body13_e111433_d_n9;
            locals.var_phi_b_dn10 = assign73830_body13_e111433_d_n10;
            locals.var_phi_b_dn11 = assign73830_body13_e111433_d_n11;
            locals.var_phi_b_dn14 = assign73830_body13_e111433_d_n14;
            locals.var_phi_b_rv = 0.0;
            let (assign73830_body14_e111445, assign73830_body14_e111445_d_n0, assign73830_body14_e111445_d_n2, assign73830_body14_e111445_d_n4, assign73830_body14_e111445_d_n5, assign73830_body14_e111445_d_n6, assign73830_body14_e111445_d_n7, assign73830_body14_e111445_d_n8, assign73830_body14_e111445_d_n9, assign73830_body14_e111445_d_n10, assign73830_body14_e111445_d_n11, assign73830_body14_e111445_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1711 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign73830_body14_e111445;
            locals.var_phi_b_dpss_dn0 = assign73830_body14_e111445_d_n0;
            locals.var_phi_b_dpss_dn2 = assign73830_body14_e111445_d_n2;
            locals.var_phi_b_dpss_dn4 = assign73830_body14_e111445_d_n4;
            locals.var_phi_b_dpss_dn5 = assign73830_body14_e111445_d_n5;
            locals.var_phi_b_dpss_dn6 = assign73830_body14_e111445_d_n6;
            locals.var_phi_b_dpss_dn7 = assign73830_body14_e111445_d_n7;
            locals.var_phi_b_dpss_dn8 = assign73830_body14_e111445_d_n8;
            locals.var_phi_b_dpss_dn9 = assign73830_body14_e111445_d_n9;
            locals.var_phi_b_dpss_dn10 = assign73830_body14_e111445_d_n10;
            locals.var_phi_b_dpss_dn11 = assign73830_body14_e111445_d_n11;
            locals.var_phi_b_dpss_dn14 = assign73830_body14_e111445_d_n14;
            locals.var_phi_b_dpss_rv = 0.0;
            let (assign73830_body15_e111456, assign73830_body15_e111456_d_n0, assign73830_body15_e111456_d_n2, assign73830_body15_e111456_d_n4, assign73830_body15_e111456_d_n5, assign73830_body15_e111456_d_n6, assign73830_body15_e111456_d_n7, assign73830_body15_e111456_d_n8, assign73830_body15_e111456_d_n9, assign73830_body15_e111456_d_n10, assign73830_body15_e111456_d_n11, assign73830_body15_e111456_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73830_body15_e111454: f64 = (locals.var_beta * locals.var_phi_b);
        (assign73830_body15_e111454, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
            locals.var_chib = assign73830_body15_e111456;
            locals.var_chib_dn0 = assign73830_body15_e111456_d_n0;
            locals.var_chib_dn2 = assign73830_body15_e111456_d_n2;
            locals.var_chib_dn4 = assign73830_body15_e111456_d_n4;
            locals.var_chib_dn5 = assign73830_body15_e111456_d_n5;
            locals.var_chib_dn6 = assign73830_body15_e111456_d_n6;
            locals.var_chib_dn7 = assign73830_body15_e111456_d_n7;
            locals.var_chib_dn8 = assign73830_body15_e111456_d_n8;
            locals.var_chib_dn9 = assign73830_body15_e111456_d_n9;
            locals.var_chib_dn10 = assign73830_body15_e111456_d_n10;
            locals.var_chib_dn11 = assign73830_body15_e111456_d_n11;
            locals.var_chib_dn14 = assign73830_body15_e111456_d_n14;
            locals.var_chib_rv = 0.0;
            let assign73830_body16_e111459: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1712 = assign73830_body16_e111459;
            locals.var_guard1712_rv = 0.0;
            let (assign73830_body18_e111484, assign73830_body18_e111484_d_n0, assign73830_body18_e111484_d_n2, assign73830_body18_e111484_d_n4, assign73830_body18_e111484_d_n5, assign73830_body18_e111484_d_n6, assign73830_body18_e111484_d_n7, assign73830_body18_e111484_d_n8, assign73830_body18_e111484_d_n9, assign73830_body18_e111484_d_n10, assign73830_body18_e111484_d_n11, assign73830_body18_e111484_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 != 0.0)) {
        let assign73830_body18_e111482: f64 = (-0.7071067811865475);
        (assign73830_body18_e111482, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign73830_body18_e111484;
            locals.var_t0_dn0 = assign73830_body18_e111484_d_n0;
            locals.var_t0_dn2 = assign73830_body18_e111484_d_n2;
            locals.var_t0_dn4 = assign73830_body18_e111484_d_n4;
            locals.var_t0_dn5 = assign73830_body18_e111484_d_n5;
            locals.var_t0_dn6 = assign73830_body18_e111484_d_n6;
            locals.var_t0_dn7 = assign73830_body18_e111484_d_n7;
            locals.var_t0_dn8 = assign73830_body18_e111484_d_n8;
            locals.var_t0_dn9 = assign73830_body18_e111484_d_n9;
            locals.var_t0_dn10 = assign73830_body18_e111484_d_n10;
            locals.var_t0_dn11 = assign73830_body18_e111484_d_n11;
            locals.var_t0_dn14 = assign73830_body18_e111484_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign73830_body19_e111497, assign73830_body19_e111497_d_n0, assign73830_body19_e111497_d_n2, assign73830_body19_e111497_d_n4, assign73830_body19_e111497_d_n5, assign73830_body19_e111497_d_n6, assign73830_body19_e111497_d_n7, assign73830_body19_e111497_d_n8, assign73830_body19_e111497_d_n9, assign73830_body19_e111497_d_n10, assign73830_body19_e111497_d_n11, assign73830_body19_e111497_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 != 0.0)) {
        let assign73830_body19_e111495: f64 = (locals.var_chi * locals.var_t0);
        (assign73830_body19_e111495, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn14 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn14)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign73830_body19_e111497;
            locals.var_fb_dn0 = assign73830_body19_e111497_d_n0;
            locals.var_fb_dn2 = assign73830_body19_e111497_d_n2;
            locals.var_fb_dn4 = assign73830_body19_e111497_d_n4;
            locals.var_fb_dn5 = assign73830_body19_e111497_d_n5;
            locals.var_fb_dn6 = assign73830_body19_e111497_d_n6;
            locals.var_fb_dn7 = assign73830_body19_e111497_d_n7;
            locals.var_fb_dn8 = assign73830_body19_e111497_d_n8;
            locals.var_fb_dn9 = assign73830_body19_e111497_d_n9;
            locals.var_fb_dn10 = assign73830_body19_e111497_d_n10;
            locals.var_fb_dn11 = assign73830_body19_e111497_d_n11;
            locals.var_fb_dn14 = assign73830_body19_e111497_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign73830_body20_e111510, assign73830_body20_e111510_d_n0, assign73830_body20_e111510_d_n2, assign73830_body20_e111510_d_n4, assign73830_body20_e111510_d_n5, assign73830_body20_e111510_d_n6, assign73830_body20_e111510_d_n7, assign73830_body20_e111510_d_n8, assign73830_body20_e111510_d_n9, assign73830_body20_e111510_d_n10, assign73830_body20_e111510_d_n11, assign73830_body20_e111510_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 != 0.0)) {
        let assign73830_body20_e111508: f64 = (locals.var_beta * locals.var_t0);
        (assign73830_body20_e111508, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn11 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn11)), ((locals.var_beta_dn14 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn14)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign73830_body20_e111510;
            locals.var_fb_dpss_dn0 = assign73830_body20_e111510_d_n0;
            locals.var_fb_dpss_dn2 = assign73830_body20_e111510_d_n2;
            locals.var_fb_dpss_dn4 = assign73830_body20_e111510_d_n4;
            locals.var_fb_dpss_dn5 = assign73830_body20_e111510_d_n5;
            locals.var_fb_dpss_dn6 = assign73830_body20_e111510_d_n6;
            locals.var_fb_dpss_dn7 = assign73830_body20_e111510_d_n7;
            locals.var_fb_dpss_dn8 = assign73830_body20_e111510_d_n8;
            locals.var_fb_dpss_dn9 = assign73830_body20_e111510_d_n9;
            locals.var_fb_dpss_dn10 = assign73830_body20_e111510_d_n10;
            locals.var_fb_dpss_dn11 = assign73830_body20_e111510_d_n11;
            locals.var_fb_dpss_dn14 = assign73830_body20_e111510_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let assign73830_body21_e111513: f64 = if locals.var_chi < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard1713 = assign73830_body21_e111513;
            locals.var_guard1713_rv = 0.0;
            let (assign73830_body23_e111565, assign73830_body23_e111565_d_n0, assign73830_body23_e111565_d_n2, assign73830_body23_e111565_d_n4, assign73830_body23_e111565_d_n5, assign73830_body23_e111565_d_n6, assign73830_body23_e111565_d_n7, assign73830_body23_e111565_d_n8, assign73830_body23_e111565_d_n9, assign73830_body23_e111565_d_n10, assign73830_body23_e111565_d_n11, assign73830_body23_e111565_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 == 0.0)) && (locals.var_guard1713 != 0.0)) {
        let assign73830_body23_e111543: f64 = (locals.var_chi * locals.var_chi);
        let assign73830_body23_e111545: f64 = (assign73830_body23_e111543 / 2.0);
        let assign73830_body23_e111549: f64 = (locals.var_chi / 3.0);
        let assign73830_body23_e111553: f64 = (locals.var_chi / 4.0);
        let assign73830_body23_e111557: f64 = (locals.var_chi / 5.0);
        let assign73830_body23_e111558: f64 = (1.0 - assign73830_body23_e111557);
        let assign73830_body23_e111559: f64 = (assign73830_body23_e111553 * assign73830_body23_e111558);
        let assign73830_body23_e111560: f64 = (1.0 - assign73830_body23_e111559);
        let assign73830_body23_e111561: f64 = (assign73830_body23_e111549 * assign73830_body23_e111560);
        let assign73830_body23_e111562: f64 = (1.0 - assign73830_body23_e111561);
        let assign73830_body23_e111563: f64 = (assign73830_body23_e111545 * assign73830_body23_e111562);
        (assign73830_body23_e111563, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign73830_body23_e111562) + (assign73830_body23_e111545 * (-(((locals.var_chi_dn0 / 3.0) * assign73830_body23_e111560) + (assign73830_body23_e111549 * (-(((locals.var_chi_dn0 / 4.0) * assign73830_body23_e111558) + (assign73830_body23_e111553 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign73830_body23_e111562) + (assign73830_body23_e111545 * (-(((locals.var_chi_dn2 / 3.0) * assign73830_body23_e111560) + (assign73830_body23_e111549 * (-(((locals.var_chi_dn2 / 4.0) * assign73830_body23_e111558) + (assign73830_body23_e111553 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign73830_body23_e111562) + (assign73830_body23_e111545 * (-(((locals.var_chi_dn4 / 3.0) * assign73830_body23_e111560) + (assign73830_body23_e111549 * (-(((locals.var_chi_dn4 / 4.0) * assign73830_body23_e111558) + (assign73830_body23_e111553 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign73830_body23_e111562) + (assign73830_body23_e111545 * (-(((locals.var_chi_dn5 / 3.0) * assign73830_body23_e111560) + (assign73830_body23_e111549 * (-(((locals.var_chi_dn5 / 4.0) * assign73830_body23_e111558) + (assign73830_body23_e111553 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign73830_body23_e111562) + (assign73830_body23_e111545 * (-(((locals.var_chi_dn6 / 3.0) * assign73830_body23_e111560) + (assign73830_body23_e111549 * (-(((locals.var_chi_dn6 / 4.0) * assign73830_body23_e111558) + (assign73830_body23_e111553 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign73830_body23_e111562) + (assign73830_body23_e111545 * (-(((locals.var_chi_dn7 / 3.0) * assign73830_body23_e111560) + (assign73830_body23_e111549 * (-(((locals.var_chi_dn7 / 4.0) * assign73830_body23_e111558) + (assign73830_body23_e111553 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign73830_body23_e111562) + (assign73830_body23_e111545 * (-(((locals.var_chi_dn8 / 3.0) * assign73830_body23_e111560) + (assign73830_body23_e111549 * (-(((locals.var_chi_dn8 / 4.0) * assign73830_body23_e111558) + (assign73830_body23_e111553 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign73830_body23_e111562) + (assign73830_body23_e111545 * (-(((locals.var_chi_dn9 / 3.0) * assign73830_body23_e111560) + (assign73830_body23_e111549 * (-(((locals.var_chi_dn9 / 4.0) * assign73830_body23_e111558) + (assign73830_body23_e111553 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign73830_body23_e111562) + (assign73830_body23_e111545 * (-(((locals.var_chi_dn10 / 3.0) * assign73830_body23_e111560) + (assign73830_body23_e111549 * (-(((locals.var_chi_dn10 / 4.0) * assign73830_body23_e111558) + (assign73830_body23_e111553 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign73830_body23_e111562) + (assign73830_body23_e111545 * (-(((locals.var_chi_dn11 / 3.0) * assign73830_body23_e111560) + (assign73830_body23_e111549 * (-(((locals.var_chi_dn11 / 4.0) * assign73830_body23_e111558) + (assign73830_body23_e111553 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign73830_body23_e111562) + (assign73830_body23_e111545 * (-(((locals.var_chi_dn14 / 3.0) * assign73830_body23_e111560) + (assign73830_body23_e111549 * (-(((locals.var_chi_dn14 / 4.0) * assign73830_body23_e111558) + (assign73830_body23_e111553 * (-(locals.var_chi_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign73830_body23_e111565;
            locals.var_t0_dn0 = assign73830_body23_e111565_d_n0;
            locals.var_t0_dn2 = assign73830_body23_e111565_d_n2;
            locals.var_t0_dn4 = assign73830_body23_e111565_d_n4;
            locals.var_t0_dn5 = assign73830_body23_e111565_d_n5;
            locals.var_t0_dn6 = assign73830_body23_e111565_d_n6;
            locals.var_t0_dn7 = assign73830_body23_e111565_d_n7;
            locals.var_t0_dn8 = assign73830_body23_e111565_d_n8;
            locals.var_t0_dn9 = assign73830_body23_e111565_d_n9;
            locals.var_t0_dn10 = assign73830_body23_e111565_d_n10;
            locals.var_t0_dn11 = assign73830_body23_e111565_d_n11;
            locals.var_t0_dn14 = assign73830_body23_e111565_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign73830_body24_e111597, assign73830_body24_e111597_d_n0, assign73830_body24_e111597_d_n2, assign73830_body24_e111597_d_n4, assign73830_body24_e111597_d_n5, assign73830_body24_e111597_d_n6, assign73830_body24_e111597_d_n7, assign73830_body24_e111597_d_n8, assign73830_body24_e111597_d_n9, assign73830_body24_e111597_d_n10, assign73830_body24_e111597_d_n11, assign73830_body24_e111597_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 == 0.0)) && (locals.var_guard1713 != 0.0)) {
        let assign73830_body24_e111581: f64 = (locals.var_chi / 2.0);
        let assign73830_body24_e111585: f64 = (locals.var_chi / 3.0);
        let assign73830_body24_e111589: f64 = (locals.var_chi / 4.0);
        let assign73830_body24_e111590: f64 = (1.0 - assign73830_body24_e111589);
        let assign73830_body24_e111591: f64 = (assign73830_body24_e111585 * assign73830_body24_e111590);
        let assign73830_body24_e111592: f64 = (1.0 - assign73830_body24_e111591);
        let assign73830_body24_e111593: f64 = (assign73830_body24_e111581 * assign73830_body24_e111592);
        let assign73830_body24_e111594: f64 = (1.0 - assign73830_body24_e111593);
        let assign73830_body24_e111595: f64 = (locals.var_chi * assign73830_body24_e111594);
        (assign73830_body24_e111595, ((locals.var_chi_dn0 * assign73830_body24_e111594) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign73830_body24_e111592) + (assign73830_body24_e111581 * (-(((locals.var_chi_dn0 / 3.0) * assign73830_body24_e111590) + (assign73830_body24_e111585 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign73830_body24_e111594) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign73830_body24_e111592) + (assign73830_body24_e111581 * (-(((locals.var_chi_dn2 / 3.0) * assign73830_body24_e111590) + (assign73830_body24_e111585 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign73830_body24_e111594) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign73830_body24_e111592) + (assign73830_body24_e111581 * (-(((locals.var_chi_dn4 / 3.0) * assign73830_body24_e111590) + (assign73830_body24_e111585 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign73830_body24_e111594) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign73830_body24_e111592) + (assign73830_body24_e111581 * (-(((locals.var_chi_dn5 / 3.0) * assign73830_body24_e111590) + (assign73830_body24_e111585 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign73830_body24_e111594) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign73830_body24_e111592) + (assign73830_body24_e111581 * (-(((locals.var_chi_dn6 / 3.0) * assign73830_body24_e111590) + (assign73830_body24_e111585 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign73830_body24_e111594) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign73830_body24_e111592) + (assign73830_body24_e111581 * (-(((locals.var_chi_dn7 / 3.0) * assign73830_body24_e111590) + (assign73830_body24_e111585 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign73830_body24_e111594) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign73830_body24_e111592) + (assign73830_body24_e111581 * (-(((locals.var_chi_dn8 / 3.0) * assign73830_body24_e111590) + (assign73830_body24_e111585 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign73830_body24_e111594) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign73830_body24_e111592) + (assign73830_body24_e111581 * (-(((locals.var_chi_dn9 / 3.0) * assign73830_body24_e111590) + (assign73830_body24_e111585 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign73830_body24_e111594) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign73830_body24_e111592) + (assign73830_body24_e111581 * (-(((locals.var_chi_dn10 / 3.0) * assign73830_body24_e111590) + (assign73830_body24_e111585 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign73830_body24_e111594) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign73830_body24_e111592) + (assign73830_body24_e111581 * (-(((locals.var_chi_dn11 / 3.0) * assign73830_body24_e111590) + (assign73830_body24_e111585 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn14 * assign73830_body24_e111594) + (locals.var_chi * (-(((locals.var_chi_dn14 / 2.0) * assign73830_body24_e111592) + (assign73830_body24_e111581 * (-(((locals.var_chi_dn14 / 3.0) * assign73830_body24_e111590) + (assign73830_body24_e111585 * (-(locals.var_chi_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign73830_body24_e111597;
            locals.var_t1_dn0 = assign73830_body24_e111597_d_n0;
            locals.var_t1_dn2 = assign73830_body24_e111597_d_n2;
            locals.var_t1_dn4 = assign73830_body24_e111597_d_n4;
            locals.var_t1_dn5 = assign73830_body24_e111597_d_n5;
            locals.var_t1_dn6 = assign73830_body24_e111597_d_n6;
            locals.var_t1_dn7 = assign73830_body24_e111597_d_n7;
            locals.var_t1_dn8 = assign73830_body24_e111597_d_n8;
            locals.var_t1_dn9 = assign73830_body24_e111597_d_n9;
            locals.var_t1_dn10 = assign73830_body24_e111597_d_n10;
            locals.var_t1_dn11 = assign73830_body24_e111597_d_n11;
            locals.var_t1_dn14 = assign73830_body24_e111597_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign73830_body25_e111633, assign73830_body25_e111633_d_n0, assign73830_body25_e111633_d_n2, assign73830_body25_e111633_d_n4, assign73830_body25_e111633_d_n5, assign73830_body25_e111633_d_n6, assign73830_body25_e111633_d_n7, assign73830_body25_e111633_d_n8, assign73830_body25_e111633_d_n9, assign73830_body25_e111633_d_n10, assign73830_body25_e111633_d_n11, assign73830_body25_e111633_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 == 0.0)) && (locals.var_guard1713 != 0.0)) {
        let assign73830_body25_e111611: f64 = (locals.var_chib * locals.var_chib);
        let assign73830_body25_e111613: f64 = (assign73830_body25_e111611 / 2.0);
        let assign73830_body25_e111617: f64 = (locals.var_chib / 3.0);
        let assign73830_body25_e111621: f64 = (locals.var_chib / 4.0);
        let assign73830_body25_e111625: f64 = (locals.var_chib / 5.0);
        let assign73830_body25_e111626: f64 = (1.0 - assign73830_body25_e111625);
        let assign73830_body25_e111627: f64 = (assign73830_body25_e111621 * assign73830_body25_e111626);
        let assign73830_body25_e111628: f64 = (1.0 - assign73830_body25_e111627);
        let assign73830_body25_e111629: f64 = (assign73830_body25_e111617 * assign73830_body25_e111628);
        let assign73830_body25_e111630: f64 = (1.0 - assign73830_body25_e111629);
        let assign73830_body25_e111631: f64 = (assign73830_body25_e111613 * assign73830_body25_e111630);
        (assign73830_body25_e111631, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign73830_body25_e111630) + (assign73830_body25_e111613 * (-(((locals.var_chib_dn0 / 3.0) * assign73830_body25_e111628) + (assign73830_body25_e111617 * (-(((locals.var_chib_dn0 / 4.0) * assign73830_body25_e111626) + (assign73830_body25_e111621 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign73830_body25_e111630) + (assign73830_body25_e111613 * (-(((locals.var_chib_dn2 / 3.0) * assign73830_body25_e111628) + (assign73830_body25_e111617 * (-(((locals.var_chib_dn2 / 4.0) * assign73830_body25_e111626) + (assign73830_body25_e111621 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign73830_body25_e111630) + (assign73830_body25_e111613 * (-(((locals.var_chib_dn4 / 3.0) * assign73830_body25_e111628) + (assign73830_body25_e111617 * (-(((locals.var_chib_dn4 / 4.0) * assign73830_body25_e111626) + (assign73830_body25_e111621 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign73830_body25_e111630) + (assign73830_body25_e111613 * (-(((locals.var_chib_dn5 / 3.0) * assign73830_body25_e111628) + (assign73830_body25_e111617 * (-(((locals.var_chib_dn5 / 4.0) * assign73830_body25_e111626) + (assign73830_body25_e111621 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign73830_body25_e111630) + (assign73830_body25_e111613 * (-(((locals.var_chib_dn6 / 3.0) * assign73830_body25_e111628) + (assign73830_body25_e111617 * (-(((locals.var_chib_dn6 / 4.0) * assign73830_body25_e111626) + (assign73830_body25_e111621 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign73830_body25_e111630) + (assign73830_body25_e111613 * (-(((locals.var_chib_dn7 / 3.0) * assign73830_body25_e111628) + (assign73830_body25_e111617 * (-(((locals.var_chib_dn7 / 4.0) * assign73830_body25_e111626) + (assign73830_body25_e111621 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign73830_body25_e111630) + (assign73830_body25_e111613 * (-(((locals.var_chib_dn8 / 3.0) * assign73830_body25_e111628) + (assign73830_body25_e111617 * (-(((locals.var_chib_dn8 / 4.0) * assign73830_body25_e111626) + (assign73830_body25_e111621 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign73830_body25_e111630) + (assign73830_body25_e111613 * (-(((locals.var_chib_dn9 / 3.0) * assign73830_body25_e111628) + (assign73830_body25_e111617 * (-(((locals.var_chib_dn9 / 4.0) * assign73830_body25_e111626) + (assign73830_body25_e111621 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign73830_body25_e111630) + (assign73830_body25_e111613 * (-(((locals.var_chib_dn10 / 3.0) * assign73830_body25_e111628) + (assign73830_body25_e111617 * (-(((locals.var_chib_dn10 / 4.0) * assign73830_body25_e111626) + (assign73830_body25_e111621 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign73830_body25_e111630) + (assign73830_body25_e111613 * (-(((locals.var_chib_dn11 / 3.0) * assign73830_body25_e111628) + (assign73830_body25_e111617 * (-(((locals.var_chib_dn11 / 4.0) * assign73830_body25_e111626) + (assign73830_body25_e111621 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn14 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn14)) / 2.0) * assign73830_body25_e111630) + (assign73830_body25_e111613 * (-(((locals.var_chib_dn14 / 3.0) * assign73830_body25_e111628) + (assign73830_body25_e111617 * (-(((locals.var_chib_dn14 / 4.0) * assign73830_body25_e111626) + (assign73830_body25_e111621 * (-(locals.var_chib_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign73830_body25_e111633;
            locals.var_t2_dn0 = assign73830_body25_e111633_d_n0;
            locals.var_t2_dn2 = assign73830_body25_e111633_d_n2;
            locals.var_t2_dn4 = assign73830_body25_e111633_d_n4;
            locals.var_t2_dn5 = assign73830_body25_e111633_d_n5;
            locals.var_t2_dn6 = assign73830_body25_e111633_d_n6;
            locals.var_t2_dn7 = assign73830_body25_e111633_d_n7;
            locals.var_t2_dn8 = assign73830_body25_e111633_d_n8;
            locals.var_t2_dn9 = assign73830_body25_e111633_d_n9;
            locals.var_t2_dn10 = assign73830_body25_e111633_d_n10;
            locals.var_t2_dn11 = assign73830_body25_e111633_d_n11;
            locals.var_t2_dn14 = assign73830_body25_e111633_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign73830_body26_e111665, assign73830_body26_e111665_d_n0, assign73830_body26_e111665_d_n2, assign73830_body26_e111665_d_n4, assign73830_body26_e111665_d_n5, assign73830_body26_e111665_d_n6, assign73830_body26_e111665_d_n7, assign73830_body26_e111665_d_n8, assign73830_body26_e111665_d_n9, assign73830_body26_e111665_d_n10, assign73830_body26_e111665_d_n11, assign73830_body26_e111665_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 == 0.0)) && (locals.var_guard1713 != 0.0)) {
        let assign73830_body26_e111649: f64 = (locals.var_chib / 2.0);
        let assign73830_body26_e111653: f64 = (locals.var_chib / 3.0);
        let assign73830_body26_e111657: f64 = (locals.var_chib / 4.0);
        let assign73830_body26_e111658: f64 = (1.0 - assign73830_body26_e111657);
        let assign73830_body26_e111659: f64 = (assign73830_body26_e111653 * assign73830_body26_e111658);
        let assign73830_body26_e111660: f64 = (1.0 - assign73830_body26_e111659);
        let assign73830_body26_e111661: f64 = (assign73830_body26_e111649 * assign73830_body26_e111660);
        let assign73830_body26_e111662: f64 = (1.0 - assign73830_body26_e111661);
        let assign73830_body26_e111663: f64 = (locals.var_chib * assign73830_body26_e111662);
        (assign73830_body26_e111663, ((locals.var_chib_dn0 * assign73830_body26_e111662) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign73830_body26_e111660) + (assign73830_body26_e111649 * (-(((locals.var_chib_dn0 / 3.0) * assign73830_body26_e111658) + (assign73830_body26_e111653 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign73830_body26_e111662) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign73830_body26_e111660) + (assign73830_body26_e111649 * (-(((locals.var_chib_dn2 / 3.0) * assign73830_body26_e111658) + (assign73830_body26_e111653 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign73830_body26_e111662) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign73830_body26_e111660) + (assign73830_body26_e111649 * (-(((locals.var_chib_dn4 / 3.0) * assign73830_body26_e111658) + (assign73830_body26_e111653 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign73830_body26_e111662) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign73830_body26_e111660) + (assign73830_body26_e111649 * (-(((locals.var_chib_dn5 / 3.0) * assign73830_body26_e111658) + (assign73830_body26_e111653 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign73830_body26_e111662) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign73830_body26_e111660) + (assign73830_body26_e111649 * (-(((locals.var_chib_dn6 / 3.0) * assign73830_body26_e111658) + (assign73830_body26_e111653 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign73830_body26_e111662) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign73830_body26_e111660) + (assign73830_body26_e111649 * (-(((locals.var_chib_dn7 / 3.0) * assign73830_body26_e111658) + (assign73830_body26_e111653 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign73830_body26_e111662) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign73830_body26_e111660) + (assign73830_body26_e111649 * (-(((locals.var_chib_dn8 / 3.0) * assign73830_body26_e111658) + (assign73830_body26_e111653 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign73830_body26_e111662) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign73830_body26_e111660) + (assign73830_body26_e111649 * (-(((locals.var_chib_dn9 / 3.0) * assign73830_body26_e111658) + (assign73830_body26_e111653 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign73830_body26_e111662) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign73830_body26_e111660) + (assign73830_body26_e111649 * (-(((locals.var_chib_dn10 / 3.0) * assign73830_body26_e111658) + (assign73830_body26_e111653 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign73830_body26_e111662) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign73830_body26_e111660) + (assign73830_body26_e111649 * (-(((locals.var_chib_dn11 / 3.0) * assign73830_body26_e111658) + (assign73830_body26_e111653 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn14 * assign73830_body26_e111662) + (locals.var_chib * (-(((locals.var_chib_dn14 / 2.0) * assign73830_body26_e111660) + (assign73830_body26_e111649 * (-(((locals.var_chib_dn14 / 3.0) * assign73830_body26_e111658) + (assign73830_body26_e111653 * (-(locals.var_chib_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign73830_body26_e111665;
            locals.var_t3_dn0 = assign73830_body26_e111665_d_n0;
            locals.var_t3_dn2 = assign73830_body26_e111665_d_n2;
            locals.var_t3_dn4 = assign73830_body26_e111665_d_n4;
            locals.var_t3_dn5 = assign73830_body26_e111665_d_n5;
            locals.var_t3_dn6 = assign73830_body26_e111665_d_n6;
            locals.var_t3_dn7 = assign73830_body26_e111665_d_n7;
            locals.var_t3_dn8 = assign73830_body26_e111665_d_n8;
            locals.var_t3_dn9 = assign73830_body26_e111665_d_n9;
            locals.var_t3_dn10 = assign73830_body26_e111665_d_n10;
            locals.var_t3_dn11 = assign73830_body26_e111665_d_n11;
            locals.var_t3_dn14 = assign73830_body26_e111665_d_n14;
            locals.var_t3_rv = 0.0;
            let (assign73830_body27_e111681, assign73830_body27_e111681_d_n0, assign73830_body27_e111681_d_n2, assign73830_body27_e111681_d_n4, assign73830_body27_e111681_d_n5, assign73830_body27_e111681_d_n6, assign73830_body27_e111681_d_n7, assign73830_body27_e111681_d_n8, assign73830_body27_e111681_d_n9, assign73830_body27_e111681_d_n10, assign73830_body27_e111681_d_n11, assign73830_body27_e111681_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 == 0.0)) && (locals.var_guard1713 != 0.0)) {
        let assign73830_body27_e111679: f64 = (locals.var_t0 - locals.var_t2);
        (assign73830_body27_e111679, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn11 - locals.var_t2_dn11), (locals.var_t0_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign73830_body27_e111681;
            locals.var_t4_dn0 = assign73830_body27_e111681_d_n0;
            locals.var_t4_dn2 = assign73830_body27_e111681_d_n2;
            locals.var_t4_dn4 = assign73830_body27_e111681_d_n4;
            locals.var_t4_dn5 = assign73830_body27_e111681_d_n5;
            locals.var_t4_dn6 = assign73830_body27_e111681_d_n6;
            locals.var_t4_dn7 = assign73830_body27_e111681_d_n7;
            locals.var_t4_dn8 = assign73830_body27_e111681_d_n8;
            locals.var_t4_dn9 = assign73830_body27_e111681_d_n9;
            locals.var_t4_dn10 = assign73830_body27_e111681_d_n10;
            locals.var_t4_dn11 = assign73830_body27_e111681_d_n11;
            locals.var_t4_dn14 = assign73830_body27_e111681_d_n14;
            locals.var_t4_rv = 0.0;
            let assign73830_body28_e111684: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1714 = assign73830_body28_e111684;
            locals.var_guard1714_rv = 0.0;
            let (assign73830_body29_e111701, assign73830_body29_e111701_d_n0, assign73830_body29_e111701_d_n2, assign73830_body29_e111701_d_n4, assign73830_body29_e111701_d_n5, assign73830_body29_e111701_d_n6, assign73830_body29_e111701_d_n7, assign73830_body29_e111701_d_n8, assign73830_body29_e111701_d_n9, assign73830_body29_e111701_d_n10, assign73830_body29_e111701_d_n11, assign73830_body29_e111701_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 == 0.0)) && (locals.var_guard1713 != 0.0)) && (locals.var_guard1714 != 0.0)) {
        let assign73830_body29_e111699: f64 = (locals.var_t4).sqrt();
        (assign73830_body29_e111699, (locals.var_t4_dn0 / (2.0 * assign73830_body29_e111699)), (locals.var_t4_dn2 / (2.0 * assign73830_body29_e111699)), (locals.var_t4_dn4 / (2.0 * assign73830_body29_e111699)), (locals.var_t4_dn5 / (2.0 * assign73830_body29_e111699)), (locals.var_t4_dn6 / (2.0 * assign73830_body29_e111699)), (locals.var_t4_dn7 / (2.0 * assign73830_body29_e111699)), (locals.var_t4_dn8 / (2.0 * assign73830_body29_e111699)), (locals.var_t4_dn9 / (2.0 * assign73830_body29_e111699)), (locals.var_t4_dn10 / (2.0 * assign73830_body29_e111699)), (locals.var_t4_dn11 / (2.0 * assign73830_body29_e111699)), (locals.var_t4_dn14 / (2.0 * assign73830_body29_e111699)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign73830_body29_e111701;
            locals.var_fb_dn0 = assign73830_body29_e111701_d_n0;
            locals.var_fb_dn2 = assign73830_body29_e111701_d_n2;
            locals.var_fb_dn4 = assign73830_body29_e111701_d_n4;
            locals.var_fb_dn5 = assign73830_body29_e111701_d_n5;
            locals.var_fb_dn6 = assign73830_body29_e111701_d_n6;
            locals.var_fb_dn7 = assign73830_body29_e111701_d_n7;
            locals.var_fb_dn8 = assign73830_body29_e111701_d_n8;
            locals.var_fb_dn9 = assign73830_body29_e111701_d_n9;
            locals.var_fb_dn10 = assign73830_body29_e111701_d_n10;
            locals.var_fb_dn11 = assign73830_body29_e111701_d_n11;
            locals.var_fb_dn14 = assign73830_body29_e111701_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign73830_body30_e111727, assign73830_body30_e111727_d_n0, assign73830_body30_e111727_d_n2, assign73830_body30_e111727_d_n4, assign73830_body30_e111727_d_n5, assign73830_body30_e111727_d_n6, assign73830_body30_e111727_d_n7, assign73830_body30_e111727_d_n8, assign73830_body30_e111727_d_n9, assign73830_body30_e111727_d_n10, assign73830_body30_e111727_d_n11, assign73830_body30_e111727_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 == 0.0)) && (locals.var_guard1713 != 0.0)) && (locals.var_guard1714 != 0.0)) {
        let assign73830_body30_e111717: f64 = (locals.var_beta * 0.5);
        let assign73830_body30_e111721: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign73830_body30_e111722: f64 = (locals.var_t1 - assign73830_body30_e111721);
        let assign73830_body30_e111723: f64 = (assign73830_body30_e111717 * assign73830_body30_e111722);
        let assign73830_body30_e111725: f64 = (assign73830_body30_e111723 / locals.var_fb);
        (assign73830_body30_e111725, ((((((locals.var_beta_dn0 * 0.5) * assign73830_body30_e111722) + (assign73830_body30_e111717 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))) * locals.var_fb) - (assign73830_body30_e111723 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign73830_body30_e111722) + (assign73830_body30_e111717 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))) * locals.var_fb) - (assign73830_body30_e111723 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign73830_body30_e111722) + (assign73830_body30_e111717 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))) * locals.var_fb) - (assign73830_body30_e111723 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign73830_body30_e111722) + (assign73830_body30_e111717 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))) * locals.var_fb) - (assign73830_body30_e111723 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign73830_body30_e111722) + (assign73830_body30_e111717 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))) * locals.var_fb) - (assign73830_body30_e111723 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign73830_body30_e111722) + (assign73830_body30_e111717 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))) * locals.var_fb) - (assign73830_body30_e111723 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign73830_body30_e111722) + (assign73830_body30_e111717 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))) * locals.var_fb) - (assign73830_body30_e111723 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign73830_body30_e111722) + (assign73830_body30_e111717 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))) * locals.var_fb) - (assign73830_body30_e111723 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign73830_body30_e111722) + (assign73830_body30_e111717 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign73830_body30_e111723 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn11 * 0.5) * assign73830_body30_e111722) + (assign73830_body30_e111717 * (locals.var_t1_dn11 - ((locals.var_phi_b_dpss_dn11 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn11))))) * locals.var_fb) - (assign73830_body30_e111723 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn14 * 0.5) * assign73830_body30_e111722) + (assign73830_body30_e111717 * (locals.var_t1_dn14 - ((locals.var_phi_b_dpss_dn14 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn14))))) * locals.var_fb) - (assign73830_body30_e111723 * locals.var_fb_dn14)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign73830_body30_e111727;
            locals.var_fb_dpss_dn0 = assign73830_body30_e111727_d_n0;
            locals.var_fb_dpss_dn2 = assign73830_body30_e111727_d_n2;
            locals.var_fb_dpss_dn4 = assign73830_body30_e111727_d_n4;
            locals.var_fb_dpss_dn5 = assign73830_body30_e111727_d_n5;
            locals.var_fb_dpss_dn6 = assign73830_body30_e111727_d_n6;
            locals.var_fb_dpss_dn7 = assign73830_body30_e111727_d_n7;
            locals.var_fb_dpss_dn8 = assign73830_body30_e111727_d_n8;
            locals.var_fb_dpss_dn9 = assign73830_body30_e111727_d_n9;
            locals.var_fb_dpss_dn10 = assign73830_body30_e111727_d_n10;
            locals.var_fb_dpss_dn11 = assign73830_body30_e111727_d_n11;
            locals.var_fb_dpss_dn14 = assign73830_body30_e111727_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let (assign73830_body32_e111763, assign73830_body32_e111763_d_n0, assign73830_body32_e111763_d_n2, assign73830_body32_e111763_d_n4, assign73830_body32_e111763_d_n5, assign73830_body32_e111763_d_n6, assign73830_body32_e111763_d_n7, assign73830_body32_e111763_d_n8, assign73830_body32_e111763_d_n9, assign73830_body32_e111763_d_n10, assign73830_body32_e111763_d_n11, assign73830_body32_e111763_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 == 0.0)) && (locals.var_guard1713 != 0.0)) && (locals.var_guard1714 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign73830_body32_e111763;
            locals.var_fb_dn0 = assign73830_body32_e111763_d_n0;
            locals.var_fb_dn2 = assign73830_body32_e111763_d_n2;
            locals.var_fb_dn4 = assign73830_body32_e111763_d_n4;
            locals.var_fb_dn5 = assign73830_body32_e111763_d_n5;
            locals.var_fb_dn6 = assign73830_body32_e111763_d_n6;
            locals.var_fb_dn7 = assign73830_body32_e111763_d_n7;
            locals.var_fb_dn8 = assign73830_body32_e111763_d_n8;
            locals.var_fb_dn9 = assign73830_body32_e111763_d_n9;
            locals.var_fb_dn10 = assign73830_body32_e111763_d_n10;
            locals.var_fb_dn11 = assign73830_body32_e111763_d_n11;
            locals.var_fb_dn14 = assign73830_body32_e111763_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign73830_body33_e111780, assign73830_body33_e111780_d_n0, assign73830_body33_e111780_d_n2, assign73830_body33_e111780_d_n4, assign73830_body33_e111780_d_n5, assign73830_body33_e111780_d_n6, assign73830_body33_e111780_d_n7, assign73830_body33_e111780_d_n8, assign73830_body33_e111780_d_n9, assign73830_body33_e111780_d_n10, assign73830_body33_e111780_d_n11, assign73830_body33_e111780_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 == 0.0)) && (locals.var_guard1713 != 0.0)) && (locals.var_guard1714 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign73830_body33_e111780;
            locals.var_fb_dpss_dn0 = assign73830_body33_e111780_d_n0;
            locals.var_fb_dpss_dn2 = assign73830_body33_e111780_d_n2;
            locals.var_fb_dpss_dn4 = assign73830_body33_e111780_d_n4;
            locals.var_fb_dpss_dn5 = assign73830_body33_e111780_d_n5;
            locals.var_fb_dpss_dn6 = assign73830_body33_e111780_d_n6;
            locals.var_fb_dpss_dn7 = assign73830_body33_e111780_d_n7;
            locals.var_fb_dpss_dn8 = assign73830_body33_e111780_d_n8;
            locals.var_fb_dpss_dn9 = assign73830_body33_e111780_d_n9;
            locals.var_fb_dpss_dn10 = assign73830_body33_e111780_d_n10;
            locals.var_fb_dpss_dn11 = assign73830_body33_e111780_d_n11;
            locals.var_fb_dpss_dn14 = assign73830_body33_e111780_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let (assign73830_body34_e111797, assign73830_body34_e111797_d_n0, assign73830_body34_e111797_d_n2, assign73830_body34_e111797_d_n4, assign73830_body34_e111797_d_n5, assign73830_body34_e111797_d_n6, assign73830_body34_e111797_d_n7, assign73830_body34_e111797_d_n8, assign73830_body34_e111797_d_n9, assign73830_body34_e111797_d_n10, assign73830_body34_e111797_d_n11, assign73830_body34_e111797_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 == 0.0)) && (locals.var_guard1713 == 0.0)) {
        let assign73830_body34_e111794: f64 = (-locals.var_chi);
        let assign73830_body34_e111795: f64 = (assign73830_body34_e111794).exp();
        (assign73830_body34_e111795, (assign73830_body34_e111795 * (-locals.var_chi_dn0)), (assign73830_body34_e111795 * (-locals.var_chi_dn2)), (assign73830_body34_e111795 * (-locals.var_chi_dn4)), (assign73830_body34_e111795 * (-locals.var_chi_dn5)), (assign73830_body34_e111795 * (-locals.var_chi_dn6)), (assign73830_body34_e111795 * (-locals.var_chi_dn7)), (assign73830_body34_e111795 * (-locals.var_chi_dn8)), (assign73830_body34_e111795 * (-locals.var_chi_dn9)), (assign73830_body34_e111795 * (-locals.var_chi_dn10)), (assign73830_body34_e111795 * (-locals.var_chi_dn11)), (assign73830_body34_e111795 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign73830_body34_e111797;
            locals.var_t0_dn0 = assign73830_body34_e111797_d_n0;
            locals.var_t0_dn2 = assign73830_body34_e111797_d_n2;
            locals.var_t0_dn4 = assign73830_body34_e111797_d_n4;
            locals.var_t0_dn5 = assign73830_body34_e111797_d_n5;
            locals.var_t0_dn6 = assign73830_body34_e111797_d_n6;
            locals.var_t0_dn7 = assign73830_body34_e111797_d_n7;
            locals.var_t0_dn8 = assign73830_body34_e111797_d_n8;
            locals.var_t0_dn9 = assign73830_body34_e111797_d_n9;
            locals.var_t0_dn10 = assign73830_body34_e111797_d_n10;
            locals.var_t0_dn11 = assign73830_body34_e111797_d_n11;
            locals.var_t0_dn14 = assign73830_body34_e111797_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign73830_body35_e111814, assign73830_body35_e111814_d_n0, assign73830_body35_e111814_d_n2, assign73830_body35_e111814_d_n4, assign73830_body35_e111814_d_n5, assign73830_body35_e111814_d_n6, assign73830_body35_e111814_d_n7, assign73830_body35_e111814_d_n8, assign73830_body35_e111814_d_n9, assign73830_body35_e111814_d_n10, assign73830_body35_e111814_d_n11, assign73830_body35_e111814_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 == 0.0)) && (locals.var_guard1713 == 0.0)) {
        let assign73830_body35_e111811: f64 = (-locals.var_chib);
        let assign73830_body35_e111812: f64 = (assign73830_body35_e111811).exp();
        (assign73830_body35_e111812, (assign73830_body35_e111812 * (-locals.var_chib_dn0)), (assign73830_body35_e111812 * (-locals.var_chib_dn2)), (assign73830_body35_e111812 * (-locals.var_chib_dn4)), (assign73830_body35_e111812 * (-locals.var_chib_dn5)), (assign73830_body35_e111812 * (-locals.var_chib_dn6)), (assign73830_body35_e111812 * (-locals.var_chib_dn7)), (assign73830_body35_e111812 * (-locals.var_chib_dn8)), (assign73830_body35_e111812 * (-locals.var_chib_dn9)), (assign73830_body35_e111812 * (-locals.var_chib_dn10)), (assign73830_body35_e111812 * (-locals.var_chib_dn11)), (assign73830_body35_e111812 * (-locals.var_chib_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign73830_body35_e111814;
            locals.var_t1_dn0 = assign73830_body35_e111814_d_n0;
            locals.var_t1_dn2 = assign73830_body35_e111814_d_n2;
            locals.var_t1_dn4 = assign73830_body35_e111814_d_n4;
            locals.var_t1_dn5 = assign73830_body35_e111814_d_n5;
            locals.var_t1_dn6 = assign73830_body35_e111814_d_n6;
            locals.var_t1_dn7 = assign73830_body35_e111814_d_n7;
            locals.var_t1_dn8 = assign73830_body35_e111814_d_n8;
            locals.var_t1_dn9 = assign73830_body35_e111814_d_n9;
            locals.var_t1_dn10 = assign73830_body35_e111814_d_n10;
            locals.var_t1_dn11 = assign73830_body35_e111814_d_n11;
            locals.var_t1_dn14 = assign73830_body35_e111814_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign73830_body36_e111835, assign73830_body36_e111835_d_n0, assign73830_body36_e111835_d_n2, assign73830_body36_e111835_d_n4, assign73830_body36_e111835_d_n5, assign73830_body36_e111835_d_n6, assign73830_body36_e111835_d_n7, assign73830_body36_e111835_d_n8, assign73830_body36_e111835_d_n9, assign73830_body36_e111835_d_n10, assign73830_body36_e111835_d_n11, assign73830_body36_e111835_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 == 0.0)) && (locals.var_guard1713 == 0.0)) {
        let assign73830_body36_e111829: f64 = (locals.var_chi - locals.var_chib);
        let assign73830_body36_e111832: f64 = (locals.var_t0 - locals.var_t1);
        let assign73830_body36_e111833: f64 = (assign73830_body36_e111829 + assign73830_body36_e111832);
        (assign73830_body36_e111833, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)), ((locals.var_chi_dn14 - locals.var_chib_dn14) + (locals.var_t0_dn14 - locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign73830_body36_e111835;
            locals.var_t4_dn0 = assign73830_body36_e111835_d_n0;
            locals.var_t4_dn2 = assign73830_body36_e111835_d_n2;
            locals.var_t4_dn4 = assign73830_body36_e111835_d_n4;
            locals.var_t4_dn5 = assign73830_body36_e111835_d_n5;
            locals.var_t4_dn6 = assign73830_body36_e111835_d_n6;
            locals.var_t4_dn7 = assign73830_body36_e111835_d_n7;
            locals.var_t4_dn8 = assign73830_body36_e111835_d_n8;
            locals.var_t4_dn9 = assign73830_body36_e111835_d_n9;
            locals.var_t4_dn10 = assign73830_body36_e111835_d_n10;
            locals.var_t4_dn11 = assign73830_body36_e111835_d_n11;
            locals.var_t4_dn14 = assign73830_body36_e111835_d_n14;
            locals.var_t4_rv = 0.0;
            let assign73830_body37_e111838: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1715 = assign73830_body37_e111838;
            locals.var_guard1715_rv = 0.0;
            let (assign73830_body38_e111856, assign73830_body38_e111856_d_n0, assign73830_body38_e111856_d_n2, assign73830_body38_e111856_d_n4, assign73830_body38_e111856_d_n5, assign73830_body38_e111856_d_n6, assign73830_body38_e111856_d_n7, assign73830_body38_e111856_d_n8, assign73830_body38_e111856_d_n9, assign73830_body38_e111856_d_n10, assign73830_body38_e111856_d_n11, assign73830_body38_e111856_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 == 0.0)) && (locals.var_guard1713 == 0.0)) && (locals.var_guard1715 != 0.0)) {
        let assign73830_body38_e111854: f64 = (locals.var_t4).sqrt();
        (assign73830_body38_e111854, (locals.var_t4_dn0 / (2.0 * assign73830_body38_e111854)), (locals.var_t4_dn2 / (2.0 * assign73830_body38_e111854)), (locals.var_t4_dn4 / (2.0 * assign73830_body38_e111854)), (locals.var_t4_dn5 / (2.0 * assign73830_body38_e111854)), (locals.var_t4_dn6 / (2.0 * assign73830_body38_e111854)), (locals.var_t4_dn7 / (2.0 * assign73830_body38_e111854)), (locals.var_t4_dn8 / (2.0 * assign73830_body38_e111854)), (locals.var_t4_dn9 / (2.0 * assign73830_body38_e111854)), (locals.var_t4_dn10 / (2.0 * assign73830_body38_e111854)), (locals.var_t4_dn11 / (2.0 * assign73830_body38_e111854)), (locals.var_t4_dn14 / (2.0 * assign73830_body38_e111854)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign73830_body38_e111856;
            locals.var_fb_dn0 = assign73830_body38_e111856_d_n0;
            locals.var_fb_dn2 = assign73830_body38_e111856_d_n2;
            locals.var_fb_dn4 = assign73830_body38_e111856_d_n4;
            locals.var_fb_dn5 = assign73830_body38_e111856_d_n5;
            locals.var_fb_dn6 = assign73830_body38_e111856_d_n6;
            locals.var_fb_dn7 = assign73830_body38_e111856_d_n7;
            locals.var_fb_dn8 = assign73830_body38_e111856_d_n8;
            locals.var_fb_dn9 = assign73830_body38_e111856_d_n9;
            locals.var_fb_dn10 = assign73830_body38_e111856_d_n10;
            locals.var_fb_dn11 = assign73830_body38_e111856_d_n11;
            locals.var_fb_dn14 = assign73830_body38_e111856_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign73830_body39_e111887, assign73830_body39_e111887_d_n0, assign73830_body39_e111887_d_n2, assign73830_body39_e111887_d_n4, assign73830_body39_e111887_d_n5, assign73830_body39_e111887_d_n6, assign73830_body39_e111887_d_n7, assign73830_body39_e111887_d_n8, assign73830_body39_e111887_d_n9, assign73830_body39_e111887_d_n10, assign73830_body39_e111887_d_n11, assign73830_body39_e111887_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 == 0.0)) && (locals.var_guard1713 == 0.0)) && (locals.var_guard1715 != 0.0)) {
        let assign73830_body39_e111873: f64 = (locals.var_beta * 0.5);
        let assign73830_body39_e111876: f64 = (1.0 - locals.var_t0);
        let assign73830_body39_e111880: f64 = (1.0 - locals.var_t1);
        let assign73830_body39_e111881: f64 = (locals.var_phi_b_dpss * assign73830_body39_e111880);
        let assign73830_body39_e111882: f64 = (assign73830_body39_e111876 - assign73830_body39_e111881);
        let assign73830_body39_e111883: f64 = (assign73830_body39_e111873 * assign73830_body39_e111882);
        let assign73830_body39_e111885: f64 = (assign73830_body39_e111883 / locals.var_fb);
        (assign73830_body39_e111885, ((((((locals.var_beta_dn0 * 0.5) * assign73830_body39_e111882) + (assign73830_body39_e111873 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign73830_body39_e111880) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign73830_body39_e111883 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign73830_body39_e111882) + (assign73830_body39_e111873 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign73830_body39_e111880) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign73830_body39_e111883 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign73830_body39_e111882) + (assign73830_body39_e111873 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign73830_body39_e111880) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign73830_body39_e111883 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign73830_body39_e111882) + (assign73830_body39_e111873 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign73830_body39_e111880) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign73830_body39_e111883 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign73830_body39_e111882) + (assign73830_body39_e111873 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign73830_body39_e111880) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign73830_body39_e111883 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign73830_body39_e111882) + (assign73830_body39_e111873 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign73830_body39_e111880) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign73830_body39_e111883 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign73830_body39_e111882) + (assign73830_body39_e111873 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign73830_body39_e111880) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign73830_body39_e111883 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign73830_body39_e111882) + (assign73830_body39_e111873 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign73830_body39_e111880) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign73830_body39_e111883 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign73830_body39_e111882) + (assign73830_body39_e111873 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign73830_body39_e111880) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign73830_body39_e111883 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn11 * 0.5) * assign73830_body39_e111882) + (assign73830_body39_e111873 * ((-locals.var_t0_dn11) - ((locals.var_phi_b_dpss_dn11 * assign73830_body39_e111880) + (locals.var_phi_b_dpss * (-locals.var_t1_dn11)))))) * locals.var_fb) - (assign73830_body39_e111883 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn14 * 0.5) * assign73830_body39_e111882) + (assign73830_body39_e111873 * ((-locals.var_t0_dn14) - ((locals.var_phi_b_dpss_dn14 * assign73830_body39_e111880) + (locals.var_phi_b_dpss * (-locals.var_t1_dn14)))))) * locals.var_fb) - (assign73830_body39_e111883 * locals.var_fb_dn14)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign73830_body39_e111887;
            locals.var_fb_dpss_dn0 = assign73830_body39_e111887_d_n0;
            locals.var_fb_dpss_dn2 = assign73830_body39_e111887_d_n2;
            locals.var_fb_dpss_dn4 = assign73830_body39_e111887_d_n4;
            locals.var_fb_dpss_dn5 = assign73830_body39_e111887_d_n5;
            locals.var_fb_dpss_dn6 = assign73830_body39_e111887_d_n6;
            locals.var_fb_dpss_dn7 = assign73830_body39_e111887_d_n7;
            locals.var_fb_dpss_dn8 = assign73830_body39_e111887_d_n8;
            locals.var_fb_dpss_dn9 = assign73830_body39_e111887_d_n9;
            locals.var_fb_dpss_dn10 = assign73830_body39_e111887_d_n10;
            locals.var_fb_dpss_dn11 = assign73830_body39_e111887_d_n11;
            locals.var_fb_dpss_dn14 = assign73830_body39_e111887_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let (assign73830_body41_e111925, assign73830_body41_e111925_d_n0, assign73830_body41_e111925_d_n2, assign73830_body41_e111925_d_n4, assign73830_body41_e111925_d_n5, assign73830_body41_e111925_d_n6, assign73830_body41_e111925_d_n7, assign73830_body41_e111925_d_n8, assign73830_body41_e111925_d_n9, assign73830_body41_e111925_d_n10, assign73830_body41_e111925_d_n11, assign73830_body41_e111925_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 == 0.0)) && (locals.var_guard1713 == 0.0)) && (locals.var_guard1715 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign73830_body41_e111925;
            locals.var_fb_dn0 = assign73830_body41_e111925_d_n0;
            locals.var_fb_dn2 = assign73830_body41_e111925_d_n2;
            locals.var_fb_dn4 = assign73830_body41_e111925_d_n4;
            locals.var_fb_dn5 = assign73830_body41_e111925_d_n5;
            locals.var_fb_dn6 = assign73830_body41_e111925_d_n6;
            locals.var_fb_dn7 = assign73830_body41_e111925_d_n7;
            locals.var_fb_dn8 = assign73830_body41_e111925_d_n8;
            locals.var_fb_dn9 = assign73830_body41_e111925_d_n9;
            locals.var_fb_dn10 = assign73830_body41_e111925_d_n10;
            locals.var_fb_dn11 = assign73830_body41_e111925_d_n11;
            locals.var_fb_dn14 = assign73830_body41_e111925_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign73830_body42_e111943, assign73830_body42_e111943_d_n0, assign73830_body42_e111943_d_n2, assign73830_body42_e111943_d_n4, assign73830_body42_e111943_d_n5, assign73830_body42_e111943_d_n6, assign73830_body42_e111943_d_n7, assign73830_body42_e111943_d_n8, assign73830_body42_e111943_d_n9, assign73830_body42_e111943_d_n10, assign73830_body42_e111943_d_n11, assign73830_body42_e111943_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1712 == 0.0)) && (locals.var_guard1713 == 0.0)) && (locals.var_guard1715 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign73830_body42_e111943;
            locals.var_fb_dpss_dn0 = assign73830_body42_e111943_d_n0;
            locals.var_fb_dpss_dn2 = assign73830_body42_e111943_d_n2;
            locals.var_fb_dpss_dn4 = assign73830_body42_e111943_d_n4;
            locals.var_fb_dpss_dn5 = assign73830_body42_e111943_d_n5;
            locals.var_fb_dpss_dn6 = assign73830_body42_e111943_d_n6;
            locals.var_fb_dpss_dn7 = assign73830_body42_e111943_d_n7;
            locals.var_fb_dpss_dn8 = assign73830_body42_e111943_d_n8;
            locals.var_fb_dpss_dn9 = assign73830_body42_e111943_d_n9;
            locals.var_fb_dpss_dn10 = assign73830_body42_e111943_d_n10;
            locals.var_fb_dpss_dn11 = assign73830_body42_e111943_d_n11;
            locals.var_fb_dpss_dn14 = assign73830_body42_e111943_d_n14;
            locals.var_fb_dpss_rv = 0.0;
            let assign73830_body43_e111946: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1716 = assign73830_body43_e111946;
            locals.var_guard1716_rv = 0.0;
            let (assign73830_body45_e111970, assign73830_body45_e111970_d_n0, assign73830_body45_e111970_d_n2, assign73830_body45_e111970_d_n4, assign73830_body45_e111970_d_n5, assign73830_body45_e111970_d_n6, assign73830_body45_e111970_d_n7, assign73830_body45_e111970_d_n8, assign73830_body45_e111970_d_n9, assign73830_body45_e111970_d_n10, assign73830_body45_e111970_d_n11, assign73830_body45_e111970_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign73830_body45_e111970;
            locals.var_fs01_dn0 = assign73830_body45_e111970_d_n0;
            locals.var_fs01_dn2 = assign73830_body45_e111970_d_n2;
            locals.var_fs01_dn4 = assign73830_body45_e111970_d_n4;
            locals.var_fs01_dn5 = assign73830_body45_e111970_d_n5;
            locals.var_fs01_dn6 = assign73830_body45_e111970_d_n6;
            locals.var_fs01_dn7 = assign73830_body45_e111970_d_n7;
            locals.var_fs01_dn8 = assign73830_body45_e111970_d_n8;
            locals.var_fs01_dn9 = assign73830_body45_e111970_d_n9;
            locals.var_fs01_dn10 = assign73830_body45_e111970_d_n10;
            locals.var_fs01_dn11 = assign73830_body45_e111970_d_n11;
            locals.var_fs01_dn14 = assign73830_body45_e111970_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign73830_body46_e111981, assign73830_body46_e111981_d_n0, assign73830_body46_e111981_d_n2, assign73830_body46_e111981_d_n4, assign73830_body46_e111981_d_n5, assign73830_body46_e111981_d_n6, assign73830_body46_e111981_d_n7, assign73830_body46_e111981_d_n8, assign73830_body46_e111981_d_n9, assign73830_body46_e111981_d_n10, assign73830_body46_e111981_d_n11, assign73830_body46_e111981_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign73830_body46_e111981;
            locals.var_fs01_dps0_dn0 = assign73830_body46_e111981_d_n0;
            locals.var_fs01_dps0_dn2 = assign73830_body46_e111981_d_n2;
            locals.var_fs01_dps0_dn4 = assign73830_body46_e111981_d_n4;
            locals.var_fs01_dps0_dn5 = assign73830_body46_e111981_d_n5;
            locals.var_fs01_dps0_dn6 = assign73830_body46_e111981_d_n6;
            locals.var_fs01_dps0_dn7 = assign73830_body46_e111981_d_n7;
            locals.var_fs01_dps0_dn8 = assign73830_body46_e111981_d_n8;
            locals.var_fs01_dps0_dn9 = assign73830_body46_e111981_d_n9;
            locals.var_fs01_dps0_dn10 = assign73830_body46_e111981_d_n10;
            locals.var_fs01_dps0_dn11 = assign73830_body46_e111981_d_n11;
            locals.var_fs01_dps0_dn14 = assign73830_body46_e111981_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign73830_body47_e111993, assign73830_body47_e111993_d_n0, assign73830_body47_e111993_d_n2, assign73830_body47_e111993_d_n4, assign73830_body47_e111993_d_n5, assign73830_body47_e111993_d_n6, assign73830_body47_e111993_d_n7, assign73830_body47_e111993_d_n8, assign73830_body47_e111993_d_n9, assign73830_body47_e111993_d_n10, assign73830_body47_e111993_d_n11, assign73830_body47_e111993_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 != 0.0)) {
        let assign73830_body47_e111991: f64 = (-locals.var_fb);
        (assign73830_body47_e111991, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn14),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign73830_body47_e111993;
            locals.var_fs02_dn0 = assign73830_body47_e111993_d_n0;
            locals.var_fs02_dn2 = assign73830_body47_e111993_d_n2;
            locals.var_fs02_dn4 = assign73830_body47_e111993_d_n4;
            locals.var_fs02_dn5 = assign73830_body47_e111993_d_n5;
            locals.var_fs02_dn6 = assign73830_body47_e111993_d_n6;
            locals.var_fs02_dn7 = assign73830_body47_e111993_d_n7;
            locals.var_fs02_dn8 = assign73830_body47_e111993_d_n8;
            locals.var_fs02_dn9 = assign73830_body47_e111993_d_n9;
            locals.var_fs02_dn10 = assign73830_body47_e111993_d_n10;
            locals.var_fs02_dn11 = assign73830_body47_e111993_d_n11;
            locals.var_fs02_dn14 = assign73830_body47_e111993_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign73830_body48_e112005, assign73830_body48_e112005_d_n0, assign73830_body48_e112005_d_n2, assign73830_body48_e112005_d_n4, assign73830_body48_e112005_d_n5, assign73830_body48_e112005_d_n6, assign73830_body48_e112005_d_n7, assign73830_body48_e112005_d_n8, assign73830_body48_e112005_d_n9, assign73830_body48_e112005_d_n10, assign73830_body48_e112005_d_n11, assign73830_body48_e112005_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 != 0.0)) {
        let assign73830_body48_e112003: f64 = (-locals.var_fb_dpss);
        (assign73830_body48_e112003, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn4), (-locals.var_fb_dpss_dn5), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn8), (-locals.var_fb_dpss_dn9), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn14),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign73830_body48_e112005;
            locals.var_fs02_dps0_dn0 = assign73830_body48_e112005_d_n0;
            locals.var_fs02_dps0_dn2 = assign73830_body48_e112005_d_n2;
            locals.var_fs02_dps0_dn4 = assign73830_body48_e112005_d_n4;
            locals.var_fs02_dps0_dn5 = assign73830_body48_e112005_d_n5;
            locals.var_fs02_dps0_dn6 = assign73830_body48_e112005_d_n6;
            locals.var_fs02_dps0_dn7 = assign73830_body48_e112005_d_n7;
            locals.var_fs02_dps0_dn8 = assign73830_body48_e112005_d_n8;
            locals.var_fs02_dps0_dn9 = assign73830_body48_e112005_d_n9;
            locals.var_fs02_dps0_dn10 = assign73830_body48_e112005_d_n10;
            locals.var_fs02_dps0_dn11 = assign73830_body48_e112005_d_n11;
            locals.var_fs02_dps0_dn14 = assign73830_body48_e112005_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let assign73830_body49_e112008: f64 = if locals.var_chi < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1717 = assign73830_body49_e112008;
            locals.var_guard1717_rv = 0.0;
            let assign73830_body50_e112011: f64 = if locals.var_chi < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard1718 = assign73830_body50_e112011;
            locals.var_guard1718_rv = 0.0;
            let (assign73830_body51_e112049, assign73830_body51_e112049_d_n0, assign73830_body51_e112049_d_n2, assign73830_body51_e112049_d_n4, assign73830_body51_e112049_d_n5, assign73830_body51_e112049_d_n6, assign73830_body51_e112049_d_n7, assign73830_body51_e112049_d_n8, assign73830_body51_e112049_d_n9, assign73830_body51_e112049_d_n10, assign73830_body51_e112049_d_n11, assign73830_body51_e112049_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 == 0.0)) && (locals.var_guard1717 != 0.0)) && (locals.var_guard1718 != 0.0)) {
        let assign73830_body51_e112027: f64 = (locals.var_chi * locals.var_chi);
        let assign73830_body51_e112029: f64 = (assign73830_body51_e112027 / 2.0);
        let assign73830_body51_e112033: f64 = (locals.var_chi / 3.0);
        let assign73830_body51_e112037: f64 = (locals.var_chi / 4.0);
        let assign73830_body51_e112041: f64 = (locals.var_chi / 5.0);
        let assign73830_body51_e112042: f64 = (1.0 + assign73830_body51_e112041);
        let assign73830_body51_e112043: f64 = (assign73830_body51_e112037 * assign73830_body51_e112042);
        let assign73830_body51_e112044: f64 = (1.0 + assign73830_body51_e112043);
        let assign73830_body51_e112045: f64 = (assign73830_body51_e112033 * assign73830_body51_e112044);
        let assign73830_body51_e112046: f64 = (1.0 + assign73830_body51_e112045);
        let assign73830_body51_e112047: f64 = (assign73830_body51_e112029 * assign73830_body51_e112046);
        (assign73830_body51_e112047, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign73830_body51_e112046) + (assign73830_body51_e112029 * (((locals.var_chi_dn0 / 3.0) * assign73830_body51_e112044) + (assign73830_body51_e112033 * (((locals.var_chi_dn0 / 4.0) * assign73830_body51_e112042) + (assign73830_body51_e112037 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign73830_body51_e112046) + (assign73830_body51_e112029 * (((locals.var_chi_dn2 / 3.0) * assign73830_body51_e112044) + (assign73830_body51_e112033 * (((locals.var_chi_dn2 / 4.0) * assign73830_body51_e112042) + (assign73830_body51_e112037 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign73830_body51_e112046) + (assign73830_body51_e112029 * (((locals.var_chi_dn4 / 3.0) * assign73830_body51_e112044) + (assign73830_body51_e112033 * (((locals.var_chi_dn4 / 4.0) * assign73830_body51_e112042) + (assign73830_body51_e112037 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign73830_body51_e112046) + (assign73830_body51_e112029 * (((locals.var_chi_dn5 / 3.0) * assign73830_body51_e112044) + (assign73830_body51_e112033 * (((locals.var_chi_dn5 / 4.0) * assign73830_body51_e112042) + (assign73830_body51_e112037 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign73830_body51_e112046) + (assign73830_body51_e112029 * (((locals.var_chi_dn6 / 3.0) * assign73830_body51_e112044) + (assign73830_body51_e112033 * (((locals.var_chi_dn6 / 4.0) * assign73830_body51_e112042) + (assign73830_body51_e112037 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign73830_body51_e112046) + (assign73830_body51_e112029 * (((locals.var_chi_dn7 / 3.0) * assign73830_body51_e112044) + (assign73830_body51_e112033 * (((locals.var_chi_dn7 / 4.0) * assign73830_body51_e112042) + (assign73830_body51_e112037 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign73830_body51_e112046) + (assign73830_body51_e112029 * (((locals.var_chi_dn8 / 3.0) * assign73830_body51_e112044) + (assign73830_body51_e112033 * (((locals.var_chi_dn8 / 4.0) * assign73830_body51_e112042) + (assign73830_body51_e112037 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign73830_body51_e112046) + (assign73830_body51_e112029 * (((locals.var_chi_dn9 / 3.0) * assign73830_body51_e112044) + (assign73830_body51_e112033 * (((locals.var_chi_dn9 / 4.0) * assign73830_body51_e112042) + (assign73830_body51_e112037 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign73830_body51_e112046) + (assign73830_body51_e112029 * (((locals.var_chi_dn10 / 3.0) * assign73830_body51_e112044) + (assign73830_body51_e112033 * (((locals.var_chi_dn10 / 4.0) * assign73830_body51_e112042) + (assign73830_body51_e112037 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign73830_body51_e112046) + (assign73830_body51_e112029 * (((locals.var_chi_dn11 / 3.0) * assign73830_body51_e112044) + (assign73830_body51_e112033 * (((locals.var_chi_dn11 / 4.0) * assign73830_body51_e112042) + (assign73830_body51_e112037 * (locals.var_chi_dn11 / 5.0))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign73830_body51_e112046) + (assign73830_body51_e112029 * (((locals.var_chi_dn14 / 3.0) * assign73830_body51_e112044) + (assign73830_body51_e112033 * (((locals.var_chi_dn14 / 4.0) * assign73830_body51_e112042) + (assign73830_body51_e112037 * (locals.var_chi_dn14 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign73830_body51_e112049;
            locals.var_t0_dn0 = assign73830_body51_e112049_d_n0;
            locals.var_t0_dn2 = assign73830_body51_e112049_d_n2;
            locals.var_t0_dn4 = assign73830_body51_e112049_d_n4;
            locals.var_t0_dn5 = assign73830_body51_e112049_d_n5;
            locals.var_t0_dn6 = assign73830_body51_e112049_d_n6;
            locals.var_t0_dn7 = assign73830_body51_e112049_d_n7;
            locals.var_t0_dn8 = assign73830_body51_e112049_d_n8;
            locals.var_t0_dn9 = assign73830_body51_e112049_d_n9;
            locals.var_t0_dn10 = assign73830_body51_e112049_d_n10;
            locals.var_t0_dn11 = assign73830_body51_e112049_d_n11;
            locals.var_t0_dn14 = assign73830_body51_e112049_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign73830_body52_e112083, assign73830_body52_e112083_d_n0, assign73830_body52_e112083_d_n2, assign73830_body52_e112083_d_n4, assign73830_body52_e112083_d_n5, assign73830_body52_e112083_d_n6, assign73830_body52_e112083_d_n7, assign73830_body52_e112083_d_n8, assign73830_body52_e112083_d_n9, assign73830_body52_e112083_d_n10, assign73830_body52_e112083_d_n11, assign73830_body52_e112083_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 == 0.0)) && (locals.var_guard1717 != 0.0)) && (locals.var_guard1718 != 0.0)) {
        let assign73830_body52_e112067: f64 = (locals.var_chi / 2.0);
        let assign73830_body52_e112071: f64 = (locals.var_chi / 3.0);
        let assign73830_body52_e112075: f64 = (locals.var_chi / 4.0);
        let assign73830_body52_e112076: f64 = (1.0 + assign73830_body52_e112075);
        let assign73830_body52_e112077: f64 = (assign73830_body52_e112071 * assign73830_body52_e112076);
        let assign73830_body52_e112078: f64 = (1.0 + assign73830_body52_e112077);
        let assign73830_body52_e112079: f64 = (assign73830_body52_e112067 * assign73830_body52_e112078);
        let assign73830_body52_e112080: f64 = (1.0 + assign73830_body52_e112079);
        let assign73830_body52_e112081: f64 = (locals.var_chi * assign73830_body52_e112080);
        (assign73830_body52_e112081, ((locals.var_chi_dn0 * assign73830_body52_e112080) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign73830_body52_e112078) + (assign73830_body52_e112067 * (((locals.var_chi_dn0 / 3.0) * assign73830_body52_e112076) + (assign73830_body52_e112071 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign73830_body52_e112080) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign73830_body52_e112078) + (assign73830_body52_e112067 * (((locals.var_chi_dn2 / 3.0) * assign73830_body52_e112076) + (assign73830_body52_e112071 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign73830_body52_e112080) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign73830_body52_e112078) + (assign73830_body52_e112067 * (((locals.var_chi_dn4 / 3.0) * assign73830_body52_e112076) + (assign73830_body52_e112071 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign73830_body52_e112080) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign73830_body52_e112078) + (assign73830_body52_e112067 * (((locals.var_chi_dn5 / 3.0) * assign73830_body52_e112076) + (assign73830_body52_e112071 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign73830_body52_e112080) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign73830_body52_e112078) + (assign73830_body52_e112067 * (((locals.var_chi_dn6 / 3.0) * assign73830_body52_e112076) + (assign73830_body52_e112071 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign73830_body52_e112080) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign73830_body52_e112078) + (assign73830_body52_e112067 * (((locals.var_chi_dn7 / 3.0) * assign73830_body52_e112076) + (assign73830_body52_e112071 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign73830_body52_e112080) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign73830_body52_e112078) + (assign73830_body52_e112067 * (((locals.var_chi_dn8 / 3.0) * assign73830_body52_e112076) + (assign73830_body52_e112071 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign73830_body52_e112080) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign73830_body52_e112078) + (assign73830_body52_e112067 * (((locals.var_chi_dn9 / 3.0) * assign73830_body52_e112076) + (assign73830_body52_e112071 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign73830_body52_e112080) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign73830_body52_e112078) + (assign73830_body52_e112067 * (((locals.var_chi_dn10 / 3.0) * assign73830_body52_e112076) + (assign73830_body52_e112071 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn11 * assign73830_body52_e112080) + (locals.var_chi * (((locals.var_chi_dn11 / 2.0) * assign73830_body52_e112078) + (assign73830_body52_e112067 * (((locals.var_chi_dn11 / 3.0) * assign73830_body52_e112076) + (assign73830_body52_e112071 * (locals.var_chi_dn11 / 4.0))))))), ((locals.var_chi_dn14 * assign73830_body52_e112080) + (locals.var_chi * (((locals.var_chi_dn14 / 2.0) * assign73830_body52_e112078) + (assign73830_body52_e112067 * (((locals.var_chi_dn14 / 3.0) * assign73830_body52_e112076) + (assign73830_body52_e112071 * (locals.var_chi_dn14 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign73830_body52_e112083;
            locals.var_t1_dn0 = assign73830_body52_e112083_d_n0;
            locals.var_t1_dn2 = assign73830_body52_e112083_d_n2;
            locals.var_t1_dn4 = assign73830_body52_e112083_d_n4;
            locals.var_t1_dn5 = assign73830_body52_e112083_d_n5;
            locals.var_t1_dn6 = assign73830_body52_e112083_d_n6;
            locals.var_t1_dn7 = assign73830_body52_e112083_d_n7;
            locals.var_t1_dn8 = assign73830_body52_e112083_d_n8;
            locals.var_t1_dn9 = assign73830_body52_e112083_d_n9;
            locals.var_t1_dn10 = assign73830_body52_e112083_d_n10;
            locals.var_t1_dn11 = assign73830_body52_e112083_d_n11;
            locals.var_t1_dn14 = assign73830_body52_e112083_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign73830_body53_e112101, assign73830_body53_e112101_d_n0, assign73830_body53_e112101_d_n2, assign73830_body53_e112101_d_n4, assign73830_body53_e112101_d_n5, assign73830_body53_e112101_d_n6, assign73830_body53_e112101_d_n7, assign73830_body53_e112101_d_n8, assign73830_body53_e112101_d_n9, assign73830_body53_e112101_d_n10, assign73830_body53_e112101_d_n11, assign73830_body53_e112101_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 == 0.0)) && (locals.var_guard1717 != 0.0)) && (locals.var_guard1718 != 0.0)) {
        let assign73830_body53_e112099: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign73830_body53_e112099, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn11 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn11)), ((locals.var_cfs1_dn14 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign73830_body53_e112101;
            locals.var_fs01_dn0 = assign73830_body53_e112101_d_n0;
            locals.var_fs01_dn2 = assign73830_body53_e112101_d_n2;
            locals.var_fs01_dn4 = assign73830_body53_e112101_d_n4;
            locals.var_fs01_dn5 = assign73830_body53_e112101_d_n5;
            locals.var_fs01_dn6 = assign73830_body53_e112101_d_n6;
            locals.var_fs01_dn7 = assign73830_body53_e112101_d_n7;
            locals.var_fs01_dn8 = assign73830_body53_e112101_d_n8;
            locals.var_fs01_dn9 = assign73830_body53_e112101_d_n9;
            locals.var_fs01_dn10 = assign73830_body53_e112101_d_n10;
            locals.var_fs01_dn11 = assign73830_body53_e112101_d_n11;
            locals.var_fs01_dn14 = assign73830_body53_e112101_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign73830_body54_e112121, assign73830_body54_e112121_d_n0, assign73830_body54_e112121_d_n2, assign73830_body54_e112121_d_n4, assign73830_body54_e112121_d_n5, assign73830_body54_e112121_d_n6, assign73830_body54_e112121_d_n7, assign73830_body54_e112121_d_n8, assign73830_body54_e112121_d_n9, assign73830_body54_e112121_d_n10, assign73830_body54_e112121_d_n11, assign73830_body54_e112121_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 == 0.0)) && (locals.var_guard1717 != 0.0)) && (locals.var_guard1718 != 0.0)) {
        let assign73830_body54_e112117: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign73830_body54_e112119: f64 = (assign73830_body54_e112117 * locals.var_beta);
        (assign73830_body54_e112119, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign73830_body54_e112117 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign73830_body54_e112117 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign73830_body54_e112117 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign73830_body54_e112117 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign73830_body54_e112117 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign73830_body54_e112117 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign73830_body54_e112117 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign73830_body54_e112117 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign73830_body54_e112117 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn11 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn11)) * locals.var_beta) + (assign73830_body54_e112117 * locals.var_beta_dn11)), ((((locals.var_cfs1_dn14 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn14)) * locals.var_beta) + (assign73830_body54_e112117 * locals.var_beta_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign73830_body54_e112121;
            locals.var_fs01_dps0_dn0 = assign73830_body54_e112121_d_n0;
            locals.var_fs01_dps0_dn2 = assign73830_body54_e112121_d_n2;
            locals.var_fs01_dps0_dn4 = assign73830_body54_e112121_d_n4;
            locals.var_fs01_dps0_dn5 = assign73830_body54_e112121_d_n5;
            locals.var_fs01_dps0_dn6 = assign73830_body54_e112121_d_n6;
            locals.var_fs01_dps0_dn7 = assign73830_body54_e112121_d_n7;
            locals.var_fs01_dps0_dn8 = assign73830_body54_e112121_d_n8;
            locals.var_fs01_dps0_dn9 = assign73830_body54_e112121_d_n9;
            locals.var_fs01_dps0_dn10 = assign73830_body54_e112121_d_n10;
            locals.var_fs01_dps0_dn11 = assign73830_body54_e112121_d_n11;
            locals.var_fs01_dps0_dn14 = assign73830_body54_e112121_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign73830_body55_e112139, assign73830_body55_e112139_d_n0, assign73830_body55_e112139_d_n2, assign73830_body55_e112139_d_n4, assign73830_body55_e112139_d_n5, assign73830_body55_e112139_d_n6, assign73830_body55_e112139_d_n7, assign73830_body55_e112139_d_n8, assign73830_body55_e112139_d_n9, assign73830_body55_e112139_d_n10, assign73830_body55_e112139_d_n11, assign73830_body55_e112139_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 == 0.0)) && (locals.var_guard1717 != 0.0)) && (locals.var_guard1718 == 0.0)) {
        let assign73830_body55_e112137: f64 = (locals.var_chi).exp();
        (assign73830_body55_e112137, (assign73830_body55_e112137 * locals.var_chi_dn0), (assign73830_body55_e112137 * locals.var_chi_dn2), (assign73830_body55_e112137 * locals.var_chi_dn4), (assign73830_body55_e112137 * locals.var_chi_dn5), (assign73830_body55_e112137 * locals.var_chi_dn6), (assign73830_body55_e112137 * locals.var_chi_dn7), (assign73830_body55_e112137 * locals.var_chi_dn8), (assign73830_body55_e112137 * locals.var_chi_dn9), (assign73830_body55_e112137 * locals.var_chi_dn10), (assign73830_body55_e112137 * locals.var_chi_dn11), (assign73830_body55_e112137 * locals.var_chi_dn14),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    }
};
            locals.var_exp_chi = assign73830_body55_e112139;
            locals.var_exp_chi_dn0 = assign73830_body55_e112139_d_n0;
            locals.var_exp_chi_dn2 = assign73830_body55_e112139_d_n2;
            locals.var_exp_chi_dn4 = assign73830_body55_e112139_d_n4;
            locals.var_exp_chi_dn5 = assign73830_body55_e112139_d_n5;
            locals.var_exp_chi_dn6 = assign73830_body55_e112139_d_n6;
            locals.var_exp_chi_dn7 = assign73830_body55_e112139_d_n7;
            locals.var_exp_chi_dn8 = assign73830_body55_e112139_d_n8;
            locals.var_exp_chi_dn9 = assign73830_body55_e112139_d_n9;
            locals.var_exp_chi_dn10 = assign73830_body55_e112139_d_n10;
            locals.var_exp_chi_dn11 = assign73830_body55_e112139_d_n11;
            locals.var_exp_chi_dn14 = assign73830_body55_e112139_d_n14;
            locals.var_exp_chi_rv = 0.0;
            let (assign73830_body56_e112158, assign73830_body56_e112158_d_n0, assign73830_body56_e112158_d_n2, assign73830_body56_e112158_d_n4, assign73830_body56_e112158_d_n5, assign73830_body56_e112158_d_n6, assign73830_body56_e112158_d_n7, assign73830_body56_e112158_d_n8, assign73830_body56_e112158_d_n9, assign73830_body56_e112158_d_n10, assign73830_body56_e112158_d_n11, assign73830_body56_e112158_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 == 0.0)) && (locals.var_guard1717 != 0.0)) && (locals.var_guard1718 == 0.0)) {
        let assign73830_body56_e112156: f64 = (locals.var_exp_chi - 1.0);
        (assign73830_body56_e112156, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign73830_body56_e112158;
            locals.var_t1_dn0 = assign73830_body56_e112158_d_n0;
            locals.var_t1_dn2 = assign73830_body56_e112158_d_n2;
            locals.var_t1_dn4 = assign73830_body56_e112158_d_n4;
            locals.var_t1_dn5 = assign73830_body56_e112158_d_n5;
            locals.var_t1_dn6 = assign73830_body56_e112158_d_n6;
            locals.var_t1_dn7 = assign73830_body56_e112158_d_n7;
            locals.var_t1_dn8 = assign73830_body56_e112158_d_n8;
            locals.var_t1_dn9 = assign73830_body56_e112158_d_n9;
            locals.var_t1_dn10 = assign73830_body56_e112158_d_n10;
            locals.var_t1_dn11 = assign73830_body56_e112158_d_n11;
            locals.var_t1_dn14 = assign73830_body56_e112158_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign73830_body57_e112179, assign73830_body57_e112179_d_n0, assign73830_body57_e112179_d_n2, assign73830_body57_e112179_d_n4, assign73830_body57_e112179_d_n5, assign73830_body57_e112179_d_n6, assign73830_body57_e112179_d_n7, assign73830_body57_e112179_d_n8, assign73830_body57_e112179_d_n9, assign73830_body57_e112179_d_n10, assign73830_body57_e112179_d_n11, assign73830_body57_e112179_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 == 0.0)) && (locals.var_guard1717 != 0.0)) && (locals.var_guard1718 == 0.0)) {
        let assign73830_body57_e112176: f64 = (locals.var_t1 - locals.var_chi);
        let assign73830_body57_e112177: f64 = (locals.var_cfs1 * assign73830_body57_e112176);
        (assign73830_body57_e112177, ((locals.var_cfs1_dn0 * assign73830_body57_e112176) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign73830_body57_e112176) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign73830_body57_e112176) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign73830_body57_e112176) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign73830_body57_e112176) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign73830_body57_e112176) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign73830_body57_e112176) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign73830_body57_e112176) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign73830_body57_e112176) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn11 * assign73830_body57_e112176) + (locals.var_cfs1 * (locals.var_t1_dn11 - locals.var_chi_dn11))), ((locals.var_cfs1_dn14 * assign73830_body57_e112176) + (locals.var_cfs1 * (locals.var_t1_dn14 - locals.var_chi_dn14))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign73830_body57_e112179;
            locals.var_fs01_dn0 = assign73830_body57_e112179_d_n0;
            locals.var_fs01_dn2 = assign73830_body57_e112179_d_n2;
            locals.var_fs01_dn4 = assign73830_body57_e112179_d_n4;
            locals.var_fs01_dn5 = assign73830_body57_e112179_d_n5;
            locals.var_fs01_dn6 = assign73830_body57_e112179_d_n6;
            locals.var_fs01_dn7 = assign73830_body57_e112179_d_n7;
            locals.var_fs01_dn8 = assign73830_body57_e112179_d_n8;
            locals.var_fs01_dn9 = assign73830_body57_e112179_d_n9;
            locals.var_fs01_dn10 = assign73830_body57_e112179_d_n10;
            locals.var_fs01_dn11 = assign73830_body57_e112179_d_n11;
            locals.var_fs01_dn14 = assign73830_body57_e112179_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign73830_body58_e112200, assign73830_body58_e112200_d_n0, assign73830_body58_e112200_d_n2, assign73830_body58_e112200_d_n4, assign73830_body58_e112200_d_n5, assign73830_body58_e112200_d_n6, assign73830_body58_e112200_d_n7, assign73830_body58_e112200_d_n8, assign73830_body58_e112200_d_n9, assign73830_body58_e112200_d_n10, assign73830_body58_e112200_d_n11, assign73830_body58_e112200_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 == 0.0)) && (locals.var_guard1717 != 0.0)) && (locals.var_guard1718 == 0.0)) {
        let assign73830_body58_e112196: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign73830_body58_e112198: f64 = (assign73830_body58_e112196 * locals.var_t1);
        (assign73830_body58_e112198, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign73830_body58_e112196 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign73830_body58_e112196 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign73830_body58_e112196 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign73830_body58_e112196 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign73830_body58_e112196 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign73830_body58_e112196 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign73830_body58_e112196 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign73830_body58_e112196 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign73830_body58_e112196 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn11 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn11)) * locals.var_t1) + (assign73830_body58_e112196 * locals.var_t1_dn11)), ((((locals.var_cfs1_dn14 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn14)) * locals.var_t1) + (assign73830_body58_e112196 * locals.var_t1_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign73830_body58_e112200;
            locals.var_fs01_dps0_dn0 = assign73830_body58_e112200_d_n0;
            locals.var_fs01_dps0_dn2 = assign73830_body58_e112200_d_n2;
            locals.var_fs01_dps0_dn4 = assign73830_body58_e112200_d_n4;
            locals.var_fs01_dps0_dn5 = assign73830_body58_e112200_d_n5;
            locals.var_fs01_dps0_dn6 = assign73830_body58_e112200_d_n6;
            locals.var_fs01_dps0_dn7 = assign73830_body58_e112200_d_n7;
            locals.var_fs01_dps0_dn8 = assign73830_body58_e112200_d_n8;
            locals.var_fs01_dps0_dn9 = assign73830_body58_e112200_d_n9;
            locals.var_fs01_dps0_dn10 = assign73830_body58_e112200_d_n10;
            locals.var_fs01_dps0_dn11 = assign73830_body58_e112200_d_n11;
            locals.var_fs01_dps0_dn14 = assign73830_body58_e112200_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign73830_body60_e112235, assign73830_body60_e112235_d_n0, assign73830_body60_e112235_d_n2, assign73830_body60_e112235_d_n4, assign73830_body60_e112235_d_n5, assign73830_body60_e112235_d_n6, assign73830_body60_e112235_d_n7, assign73830_body60_e112235_d_n8, assign73830_body60_e112235_d_n9, assign73830_body60_e112235_d_n10, assign73830_body60_e112235_d_n11, assign73830_body60_e112235_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 == 0.0)) && (locals.var_guard1717 == 0.0)) {
        let assign73830_body60_e112232: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign73830_body60_e112233: f64 = (assign73830_body60_e112232).exp();
        (assign73830_body60_e112233, (assign73830_body60_e112233 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign73830_body60_e112233 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign73830_body60_e112233 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign73830_body60_e112233 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign73830_body60_e112233 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign73830_body60_e112233 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign73830_body60_e112233 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign73830_body60_e112233 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign73830_body60_e112233 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign73830_body60_e112233 * ((locals.var_beta_dn11 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn11))), (assign73830_body60_e112233 * ((locals.var_beta_dn14 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn14))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn14,)
    }
};
            locals.var_exp_bps0 = assign73830_body60_e112235;
            locals.var_exp_bps0_dn0 = assign73830_body60_e112235_d_n0;
            locals.var_exp_bps0_dn2 = assign73830_body60_e112235_d_n2;
            locals.var_exp_bps0_dn4 = assign73830_body60_e112235_d_n4;
            locals.var_exp_bps0_dn5 = assign73830_body60_e112235_d_n5;
            locals.var_exp_bps0_dn6 = assign73830_body60_e112235_d_n6;
            locals.var_exp_bps0_dn7 = assign73830_body60_e112235_d_n7;
            locals.var_exp_bps0_dn8 = assign73830_body60_e112235_d_n8;
            locals.var_exp_bps0_dn9 = assign73830_body60_e112235_d_n9;
            locals.var_exp_bps0_dn10 = assign73830_body60_e112235_d_n10;
            locals.var_exp_bps0_dn11 = assign73830_body60_e112235_d_n11;
            locals.var_exp_bps0_dn14 = assign73830_body60_e112235_d_n14;
            locals.var_exp_bps0_rv = 0.0;
            let (assign73830_body61_e112258, assign73830_body61_e112258_d_n0, assign73830_body61_e112258_d_n2, assign73830_body61_e112258_d_n4, assign73830_body61_e112258_d_n5, assign73830_body61_e112258_d_n6, assign73830_body61_e112258_d_n7, assign73830_body61_e112258_d_n8, assign73830_body61_e112258_d_n9, assign73830_body61_e112258_d_n10, assign73830_body61_e112258_d_n11, assign73830_body61_e112258_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 == 0.0)) && (locals.var_guard1717 == 0.0)) {
        let assign73830_body61_e112253: f64 = (locals.var_chi + 1.0);
        let assign73830_body61_e112254: f64 = (locals.var_exp_bvbs * assign73830_body61_e112253);
        let assign73830_body61_e112255: f64 = (locals.var_exp_bps0 - assign73830_body61_e112254);
        let assign73830_body61_e112256: f64 = (locals.var_cnst1over * assign73830_body61_e112255);
        (assign73830_body61_e112256, ((locals.var_cnst1over_dn0 * assign73830_body61_e112255) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign73830_body61_e112253) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign73830_body61_e112255) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign73830_body61_e112253) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign73830_body61_e112255) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign73830_body61_e112253) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign73830_body61_e112255) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign73830_body61_e112253) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign73830_body61_e112255) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign73830_body61_e112253) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign73830_body61_e112255) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign73830_body61_e112253) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign73830_body61_e112255) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign73830_body61_e112253) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign73830_body61_e112255) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign73830_body61_e112253) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign73830_body61_e112255) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign73830_body61_e112253) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn11 * assign73830_body61_e112255) + (locals.var_cnst1over * (locals.var_exp_bps0_dn11 - ((locals.var_exp_bvbs_dn11 * assign73830_body61_e112253) + (locals.var_exp_bvbs * locals.var_chi_dn11))))), ((locals.var_cnst1over_dn14 * assign73830_body61_e112255) + (locals.var_cnst1over * (locals.var_exp_bps0_dn14 - ((locals.var_exp_bvbs_dn14 * assign73830_body61_e112253) + (locals.var_exp_bvbs * locals.var_chi_dn14))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign73830_body61_e112258;
            locals.var_fs01_dn0 = assign73830_body61_e112258_d_n0;
            locals.var_fs01_dn2 = assign73830_body61_e112258_d_n2;
            locals.var_fs01_dn4 = assign73830_body61_e112258_d_n4;
            locals.var_fs01_dn5 = assign73830_body61_e112258_d_n5;
            locals.var_fs01_dn6 = assign73830_body61_e112258_d_n6;
            locals.var_fs01_dn7 = assign73830_body61_e112258_d_n7;
            locals.var_fs01_dn8 = assign73830_body61_e112258_d_n8;
            locals.var_fs01_dn9 = assign73830_body61_e112258_d_n9;
            locals.var_fs01_dn10 = assign73830_body61_e112258_d_n10;
            locals.var_fs01_dn11 = assign73830_body61_e112258_d_n11;
            locals.var_fs01_dn14 = assign73830_body61_e112258_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign73830_body62_e112279, assign73830_body62_e112279_d_n0, assign73830_body62_e112279_d_n2, assign73830_body62_e112279_d_n4, assign73830_body62_e112279_d_n5, assign73830_body62_e112279_d_n6, assign73830_body62_e112279_d_n7, assign73830_body62_e112279_d_n8, assign73830_body62_e112279_d_n9, assign73830_body62_e112279_d_n10, assign73830_body62_e112279_d_n11, assign73830_body62_e112279_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 == 0.0)) && (locals.var_guard1717 == 0.0)) {
        let assign73830_body62_e112273: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign73830_body62_e112276: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign73830_body62_e112277: f64 = (assign73830_body62_e112273 * assign73830_body62_e112276);
        (assign73830_body62_e112277, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign73830_body62_e112276) + (assign73830_body62_e112273 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign73830_body62_e112276) + (assign73830_body62_e112273 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign73830_body62_e112276) + (assign73830_body62_e112273 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign73830_body62_e112276) + (assign73830_body62_e112273 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign73830_body62_e112276) + (assign73830_body62_e112273 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign73830_body62_e112276) + (assign73830_body62_e112273 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign73830_body62_e112276) + (assign73830_body62_e112273 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign73830_body62_e112276) + (assign73830_body62_e112273 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign73830_body62_e112276) + (assign73830_body62_e112273 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn11 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn11)) * assign73830_body62_e112276) + (assign73830_body62_e112273 * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), ((((locals.var_cnst1over_dn14 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn14)) * assign73830_body62_e112276) + (assign73830_body62_e112273 * (locals.var_exp_bps0_dn14 - locals.var_exp_bvbs_dn14))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign73830_body62_e112279;
            locals.var_fs01_dps0_dn0 = assign73830_body62_e112279_d_n0;
            locals.var_fs01_dps0_dn2 = assign73830_body62_e112279_d_n2;
            locals.var_fs01_dps0_dn4 = assign73830_body62_e112279_d_n4;
            locals.var_fs01_dps0_dn5 = assign73830_body62_e112279_d_n5;
            locals.var_fs01_dps0_dn6 = assign73830_body62_e112279_d_n6;
            locals.var_fs01_dps0_dn7 = assign73830_body62_e112279_d_n7;
            locals.var_fs01_dps0_dn8 = assign73830_body62_e112279_d_n8;
            locals.var_fs01_dps0_dn9 = assign73830_body62_e112279_d_n9;
            locals.var_fs01_dps0_dn10 = assign73830_body62_e112279_d_n10;
            locals.var_fs01_dps0_dn11 = assign73830_body62_e112279_d_n11;
            locals.var_fs01_dps0_dn14 = assign73830_body62_e112279_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let assign73830_body63_e112282: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1719 = assign73830_body63_e112282;
            locals.var_guard1719_rv = 0.0;
            let (assign73830_body64_e112301, assign73830_body64_e112301_d_n0, assign73830_body64_e112301_d_n2, assign73830_body64_e112301_d_n4, assign73830_body64_e112301_d_n5, assign73830_body64_e112301_d_n6, assign73830_body64_e112301_d_n7, assign73830_body64_e112301_d_n8, assign73830_body64_e112301_d_n9, assign73830_body64_e112301_d_n10, assign73830_body64_e112301_d_n11, assign73830_body64_e112301_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 == 0.0)) && (locals.var_guard1719 != 0.0)) {
        let assign73830_body64_e112296: f64 = (locals.var_fb * locals.var_fb);
        let assign73830_body64_e112298: f64 = (assign73830_body64_e112296 + locals.var_fs01);
        let assign73830_body64_e112299: f64 = (assign73830_body64_e112298).sqrt();
        (assign73830_body64_e112299, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign73830_body64_e112299)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign73830_body64_e112299)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign73830_body64_e112299)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign73830_body64_e112299)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign73830_body64_e112299)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign73830_body64_e112299)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign73830_body64_e112299)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign73830_body64_e112299)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign73830_body64_e112299)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign73830_body64_e112299)), ((((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)) + locals.var_fs01_dn14) / (2.0 * assign73830_body64_e112299)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign73830_body64_e112301;
            locals.var_fs02_dn0 = assign73830_body64_e112301_d_n0;
            locals.var_fs02_dn2 = assign73830_body64_e112301_d_n2;
            locals.var_fs02_dn4 = assign73830_body64_e112301_d_n4;
            locals.var_fs02_dn5 = assign73830_body64_e112301_d_n5;
            locals.var_fs02_dn6 = assign73830_body64_e112301_d_n6;
            locals.var_fs02_dn7 = assign73830_body64_e112301_d_n7;
            locals.var_fs02_dn8 = assign73830_body64_e112301_d_n8;
            locals.var_fs02_dn9 = assign73830_body64_e112301_d_n9;
            locals.var_fs02_dn10 = assign73830_body64_e112301_d_n10;
            locals.var_fs02_dn11 = assign73830_body64_e112301_d_n11;
            locals.var_fs02_dn14 = assign73830_body64_e112301_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign73830_body65_e112325, assign73830_body65_e112325_d_n0, assign73830_body65_e112325_d_n2, assign73830_body65_e112325_d_n4, assign73830_body65_e112325_d_n5, assign73830_body65_e112325_d_n6, assign73830_body65_e112325_d_n7, assign73830_body65_e112325_d_n8, assign73830_body65_e112325_d_n9, assign73830_body65_e112325_d_n10, assign73830_body65_e112325_d_n11, assign73830_body65_e112325_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 == 0.0)) && (locals.var_guard1719 != 0.0)) {
        let assign73830_body65_e112316: f64 = (2.0 * locals.var_fb_dpss);
        let assign73830_body65_e112318: f64 = (assign73830_body65_e112316 * locals.var_fb);
        let assign73830_body65_e112320: f64 = (assign73830_body65_e112318 + locals.var_fs01_dps0);
        let assign73830_body65_e112321: f64 = (0.5 * assign73830_body65_e112320);
        let assign73830_body65_e112323: f64 = (assign73830_body65_e112321 / locals.var_fs02);
        (assign73830_body65_e112323, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign73830_body65_e112316 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign73830_body65_e112321 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign73830_body65_e112316 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign73830_body65_e112321 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn4) * locals.var_fb) + (assign73830_body65_e112316 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign73830_body65_e112321 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn5) * locals.var_fb) + (assign73830_body65_e112316 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign73830_body65_e112321 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign73830_body65_e112316 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign73830_body65_e112321 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign73830_body65_e112316 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign73830_body65_e112321 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn8) * locals.var_fb) + (assign73830_body65_e112316 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign73830_body65_e112321 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn9) * locals.var_fb) + (assign73830_body65_e112316 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign73830_body65_e112321 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign73830_body65_e112316 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign73830_body65_e112321 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn11) * locals.var_fb) + (assign73830_body65_e112316 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign73830_body65_e112321 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn14) * locals.var_fb) + (assign73830_body65_e112316 * locals.var_fb_dn14)) + locals.var_fs01_dps0_dn14)) * locals.var_fs02) - (assign73830_body65_e112321 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign73830_body65_e112325;
            locals.var_fs02_dps0_dn0 = assign73830_body65_e112325_d_n0;
            locals.var_fs02_dps0_dn2 = assign73830_body65_e112325_d_n2;
            locals.var_fs02_dps0_dn4 = assign73830_body65_e112325_d_n4;
            locals.var_fs02_dps0_dn5 = assign73830_body65_e112325_d_n5;
            locals.var_fs02_dps0_dn6 = assign73830_body65_e112325_d_n6;
            locals.var_fs02_dps0_dn7 = assign73830_body65_e112325_d_n7;
            locals.var_fs02_dps0_dn8 = assign73830_body65_e112325_d_n8;
            locals.var_fs02_dps0_dn9 = assign73830_body65_e112325_d_n9;
            locals.var_fs02_dps0_dn10 = assign73830_body65_e112325_d_n10;
            locals.var_fs02_dps0_dn11 = assign73830_body65_e112325_d_n11;
            locals.var_fs02_dps0_dn14 = assign73830_body65_e112325_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign73830_body67_e112357, assign73830_body67_e112357_d_n0, assign73830_body67_e112357_d_n2, assign73830_body67_e112357_d_n4, assign73830_body67_e112357_d_n5, assign73830_body67_e112357_d_n6, assign73830_body67_e112357_d_n7, assign73830_body67_e112357_d_n8, assign73830_body67_e112357_d_n9, assign73830_body67_e112357_d_n10, assign73830_body67_e112357_d_n11, assign73830_body67_e112357_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 == 0.0)) && (locals.var_guard1719 == 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign73830_body67_e112357;
            locals.var_fs02_dn0 = assign73830_body67_e112357_d_n0;
            locals.var_fs02_dn2 = assign73830_body67_e112357_d_n2;
            locals.var_fs02_dn4 = assign73830_body67_e112357_d_n4;
            locals.var_fs02_dn5 = assign73830_body67_e112357_d_n5;
            locals.var_fs02_dn6 = assign73830_body67_e112357_d_n6;
            locals.var_fs02_dn7 = assign73830_body67_e112357_d_n7;
            locals.var_fs02_dn8 = assign73830_body67_e112357_d_n8;
            locals.var_fs02_dn9 = assign73830_body67_e112357_d_n9;
            locals.var_fs02_dn10 = assign73830_body67_e112357_d_n10;
            locals.var_fs02_dn11 = assign73830_body67_e112357_d_n11;
            locals.var_fs02_dn14 = assign73830_body67_e112357_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign73830_body68_e112372, assign73830_body68_e112372_d_n0, assign73830_body68_e112372_d_n2, assign73830_body68_e112372_d_n4, assign73830_body68_e112372_d_n5, assign73830_body68_e112372_d_n6, assign73830_body68_e112372_d_n7, assign73830_body68_e112372_d_n8, assign73830_body68_e112372_d_n9, assign73830_body68_e112372_d_n10, assign73830_body68_e112372_d_n11, assign73830_body68_e112372_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1716 == 0.0)) && (locals.var_guard1719 == 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign73830_body68_e112372;
            locals.var_fs02_dps0_dn0 = assign73830_body68_e112372_d_n0;
            locals.var_fs02_dps0_dn2 = assign73830_body68_e112372_d_n2;
            locals.var_fs02_dps0_dn4 = assign73830_body68_e112372_d_n4;
            locals.var_fs02_dps0_dn5 = assign73830_body68_e112372_d_n5;
            locals.var_fs02_dps0_dn6 = assign73830_body68_e112372_d_n6;
            locals.var_fs02_dps0_dn7 = assign73830_body68_e112372_d_n7;
            locals.var_fs02_dps0_dn8 = assign73830_body68_e112372_d_n8;
            locals.var_fs02_dps0_dn9 = assign73830_body68_e112372_d_n9;
            locals.var_fs02_dps0_dn10 = assign73830_body68_e112372_d_n10;
            locals.var_fs02_dps0_dn11 = assign73830_body68_e112372_d_n11;
            locals.var_fs02_dps0_dn14 = assign73830_body68_e112372_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign73830_body69_e112388, assign73830_body69_e112388_d_n0, assign73830_body69_e112388_d_n2, assign73830_body69_e112388_d_n4, assign73830_body69_e112388_d_n5, assign73830_body69_e112388_d_n6, assign73830_body69_e112388_d_n7, assign73830_body69_e112388_d_n8, assign73830_body69_e112388_d_n9, assign73830_body69_e112388_d_n10, assign73830_body69_e112388_d_n11, assign73830_body69_e112388_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73830_body69_e112380: f64 = (-locals.var_vgpld);
        let assign73830_body69_e112382: f64 = (assign73830_body69_e112380 + locals.var_ps0ld);
        let assign73830_body69_e112385: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign73830_body69_e112386: f64 = (assign73830_body69_e112382 + assign73830_body69_e112385);
        (assign73830_body69_e112386, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (locals.var_ps0ld_dn6 + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (((-locals.var_vgpld_dn9) + locals.var_ps0ld_dn9) + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn11 + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), (locals.var_ps0ld_dn14 + ((locals.var_fac1_dn14 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn14))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
            locals.var_fs0 = assign73830_body69_e112388;
            locals.var_fs0_dn0 = assign73830_body69_e112388_d_n0;
            locals.var_fs0_dn2 = assign73830_body69_e112388_d_n2;
            locals.var_fs0_dn4 = assign73830_body69_e112388_d_n4;
            locals.var_fs0_dn5 = assign73830_body69_e112388_d_n5;
            locals.var_fs0_dn6 = assign73830_body69_e112388_d_n6;
            locals.var_fs0_dn7 = assign73830_body69_e112388_d_n7;
            locals.var_fs0_dn8 = assign73830_body69_e112388_d_n8;
            locals.var_fs0_dn9 = assign73830_body69_e112388_d_n9;
            locals.var_fs0_dn10 = assign73830_body69_e112388_d_n10;
            locals.var_fs0_dn11 = assign73830_body69_e112388_d_n11;
            locals.var_fs0_dn14 = assign73830_body69_e112388_d_n14;
            locals.var_fs0_rv = 0.0;
            let (assign73830_body70_e112401, assign73830_body70_e112401_d_n0, assign73830_body70_e112401_d_n2, assign73830_body70_e112401_d_n4, assign73830_body70_e112401_d_n5, assign73830_body70_e112401_d_n6, assign73830_body70_e112401_d_n7, assign73830_body70_e112401_d_n8, assign73830_body70_e112401_d_n9, assign73830_body70_e112401_d_n10, assign73830_body70_e112401_d_n11, assign73830_body70_e112401_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73830_body70_e112398: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign73830_body70_e112399: f64 = (1.0 + assign73830_body70_e112398);
        (assign73830_body70_e112399, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn14 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn14)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
            locals.var_fs0_dps0 = assign73830_body70_e112401;
            locals.var_fs0_dps0_dn0 = assign73830_body70_e112401_d_n0;
            locals.var_fs0_dps0_dn2 = assign73830_body70_e112401_d_n2;
            locals.var_fs0_dps0_dn4 = assign73830_body70_e112401_d_n4;
            locals.var_fs0_dps0_dn5 = assign73830_body70_e112401_d_n5;
            locals.var_fs0_dps0_dn6 = assign73830_body70_e112401_d_n6;
            locals.var_fs0_dps0_dn7 = assign73830_body70_e112401_d_n7;
            locals.var_fs0_dps0_dn8 = assign73830_body70_e112401_d_n8;
            locals.var_fs0_dps0_dn9 = assign73830_body70_e112401_d_n9;
            locals.var_fs0_dps0_dn10 = assign73830_body70_e112401_d_n10;
            locals.var_fs0_dps0_dn11 = assign73830_body70_e112401_d_n11;
            locals.var_fs0_dps0_dn14 = assign73830_body70_e112401_d_n14;
            locals.var_fs0_dps0_rv = 0.0;
            let assign73830_body71_e112404: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1720 = assign73830_body71_e112404;
            locals.var_guard1720_rv = 0.0;
            let (assign73830_body72_e112417,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1720 != 0.0)) {
        let assign73830_body72_e112415: f64 = (locals.var_lp_s0_max + 1.0);
        (assign73830_body72_e112415,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign73830_body72_e112417;
            locals.var_lp_s0_rv = 0.0;
            let (assign73830_body73_e112432, assign73830_body73_e112432_d_n0, assign73830_body73_e112432_d_n2, assign73830_body73_e112432_d_n4, assign73830_body73_e112432_d_n5, assign73830_body73_e112432_d_n6, assign73830_body73_e112432_d_n7, assign73830_body73_e112432_d_n8, assign73830_body73_e112432_d_n9, assign73830_body73_e112432_d_n10, assign73830_body73_e112432_d_n11, assign73830_body73_e112432_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1720 == 0.0)) {
        let assign73830_body73_e112428: f64 = (-locals.var_fs0);
        let assign73830_body73_e112430: f64 = (assign73830_body73_e112428 / locals.var_fs0_dps0);
        (assign73830_body73_e112430, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign73830_body73_e112428 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign73830_body73_e112428 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign73830_body73_e112428 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign73830_body73_e112428 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign73830_body73_e112428 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign73830_body73_e112428 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign73830_body73_e112428 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign73830_body73_e112428 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign73830_body73_e112428 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign73830_body73_e112428 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn14) * locals.var_fs0_dps0) - (assign73830_body73_e112428 * locals.var_fs0_dps0_dn14)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign73830_body73_e112432;
            locals.var_dps0_dn0 = assign73830_body73_e112432_d_n0;
            locals.var_dps0_dn2 = assign73830_body73_e112432_d_n2;
            locals.var_dps0_dn4 = assign73830_body73_e112432_d_n4;
            locals.var_dps0_dn5 = assign73830_body73_e112432_d_n5;
            locals.var_dps0_dn6 = assign73830_body73_e112432_d_n6;
            locals.var_dps0_dn7 = assign73830_body73_e112432_d_n7;
            locals.var_dps0_dn8 = assign73830_body73_e112432_d_n8;
            locals.var_dps0_dn9 = assign73830_body73_e112432_d_n9;
            locals.var_dps0_dn10 = assign73830_body73_e112432_d_n10;
            locals.var_dps0_dn11 = assign73830_body73_e112432_d_n11;
            locals.var_dps0_dn14 = assign73830_body73_e112432_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign73830_body74_e112457, assign73830_body74_e112457_d_n0, assign73830_body74_e112457_d_n2, assign73830_body74_e112457_d_n4, assign73830_body74_e112457_d_n5, assign73830_body74_e112457_d_n6, assign73830_body74_e112457_d_n7, assign73830_body74_e112457_d_n8, assign73830_body74_e112457_d_n9, assign73830_body74_e112457_d_n10, assign73830_body74_e112457_d_n11, assign73830_body74_e112457_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1720 == 0.0)) {
        let assign73830_body74_e112444: f64 = (0.5 * 0.1);
        let assign73830_body74_e112448: f64 = (locals.var_ps0ld).abs();
        let (assign73830_body74_e112453, assign73830_body74_e112453_d_n0, assign73830_body74_e112453_d_n2, assign73830_body74_e112453_d_n4, assign73830_body74_e112453_d_n5, assign73830_body74_e112453_d_n6, assign73830_body74_e112453_d_n7, assign73830_body74_e112453_d_n8, assign73830_body74_e112453_d_n9, assign73830_body74_e112453_d_n10, assign73830_body74_e112453_d_n11, assign73830_body74_e112453_d_n14,) = {
            if (1.0 >= assign73830_body74_e112448) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign73830_body74_e112452: f64 = (locals.var_ps0ld).abs();
                (assign73830_body74_e112452, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn14 } else { (-locals.var_ps0ld_dn14) },)
            }
        };
        let assign73830_body74_e112454: f64 = (1.0 + assign73830_body74_e112453);
        let assign73830_body74_e112455: f64 = (assign73830_body74_e112444 * assign73830_body74_e112454);
        (assign73830_body74_e112455, (assign73830_body74_e112444 * assign73830_body74_e112453_d_n0), (assign73830_body74_e112444 * assign73830_body74_e112453_d_n2), (assign73830_body74_e112444 * assign73830_body74_e112453_d_n4), (assign73830_body74_e112444 * assign73830_body74_e112453_d_n5), (assign73830_body74_e112444 * assign73830_body74_e112453_d_n6), (assign73830_body74_e112444 * assign73830_body74_e112453_d_n7), (assign73830_body74_e112444 * assign73830_body74_e112453_d_n8), (assign73830_body74_e112444 * assign73830_body74_e112453_d_n9), (assign73830_body74_e112444 * assign73830_body74_e112453_d_n10), (assign73830_body74_e112444 * assign73830_body74_e112453_d_n11), (assign73830_body74_e112444 * assign73830_body74_e112453_d_n14),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn14,)
    }
};
            locals.var_dplim = assign73830_body74_e112457;
            locals.var_dplim_dn0 = assign73830_body74_e112457_d_n0;
            locals.var_dplim_dn2 = assign73830_body74_e112457_d_n2;
            locals.var_dplim_dn4 = assign73830_body74_e112457_d_n4;
            locals.var_dplim_dn5 = assign73830_body74_e112457_d_n5;
            locals.var_dplim_dn6 = assign73830_body74_e112457_d_n6;
            locals.var_dplim_dn7 = assign73830_body74_e112457_d_n7;
            locals.var_dplim_dn8 = assign73830_body74_e112457_d_n8;
            locals.var_dplim_dn9 = assign73830_body74_e112457_d_n9;
            locals.var_dplim_dn10 = assign73830_body74_e112457_d_n10;
            locals.var_dplim_dn11 = assign73830_body74_e112457_d_n11;
            locals.var_dplim_dn14 = assign73830_body74_e112457_d_n14;
            locals.var_dplim_rv = 0.0;
            let assign73830_body75_e112459: f64 = (locals.var_dps0).abs();
            let assign73830_body75_e112461: f64 = if assign73830_body75_e112459 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1721 = assign73830_body75_e112461;
            locals.var_guard1721_rv = 0.0;
            let (assign73830_body76_e112483, assign73830_body76_e112483_d_n0, assign73830_body76_e112483_d_n2, assign73830_body76_e112483_d_n4, assign73830_body76_e112483_d_n5, assign73830_body76_e112483_d_n6, assign73830_body76_e112483_d_n7, assign73830_body76_e112483_d_n8, assign73830_body76_e112483_d_n9, assign73830_body76_e112483_d_n10, assign73830_body76_e112483_d_n11, assign73830_body76_e112483_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1720 == 0.0)) && (locals.var_guard1721 != 0.0)) {
        let (assign73830_body76_e112480,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign73830_body76_e112479: f64 = (-1.0);
                (assign73830_body76_e112479,)
            }
        };
        let assign73830_body76_e112481: f64 = (locals.var_dplim * assign73830_body76_e112480);
        (assign73830_body76_e112481, (locals.var_dplim_dn0 * assign73830_body76_e112480), (locals.var_dplim_dn2 * assign73830_body76_e112480), (locals.var_dplim_dn4 * assign73830_body76_e112480), (locals.var_dplim_dn5 * assign73830_body76_e112480), (locals.var_dplim_dn6 * assign73830_body76_e112480), (locals.var_dplim_dn7 * assign73830_body76_e112480), (locals.var_dplim_dn8 * assign73830_body76_e112480), (locals.var_dplim_dn9 * assign73830_body76_e112480), (locals.var_dplim_dn10 * assign73830_body76_e112480), (locals.var_dplim_dn11 * assign73830_body76_e112480), (locals.var_dplim_dn14 * assign73830_body76_e112480),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign73830_body76_e112483;
            locals.var_dps0_dn0 = assign73830_body76_e112483_d_n0;
            locals.var_dps0_dn2 = assign73830_body76_e112483_d_n2;
            locals.var_dps0_dn4 = assign73830_body76_e112483_d_n4;
            locals.var_dps0_dn5 = assign73830_body76_e112483_d_n5;
            locals.var_dps0_dn6 = assign73830_body76_e112483_d_n6;
            locals.var_dps0_dn7 = assign73830_body76_e112483_d_n7;
            locals.var_dps0_dn8 = assign73830_body76_e112483_d_n8;
            locals.var_dps0_dn9 = assign73830_body76_e112483_d_n9;
            locals.var_dps0_dn10 = assign73830_body76_e112483_d_n10;
            locals.var_dps0_dn11 = assign73830_body76_e112483_d_n11;
            locals.var_dps0_dn14 = assign73830_body76_e112483_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign73830_body77_e112497, assign73830_body77_e112497_d_n0, assign73830_body77_e112497_d_n2, assign73830_body77_e112497_d_n4, assign73830_body77_e112497_d_n5, assign73830_body77_e112497_d_n6, assign73830_body77_e112497_d_n7, assign73830_body77_e112497_d_n8, assign73830_body77_e112497_d_n9, assign73830_body77_e112497_d_n10, assign73830_body77_e112497_d_n11, assign73830_body77_e112497_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1720 == 0.0)) {
        let assign73830_body77_e112495: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign73830_body77_e112495, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn14 + locals.var_dps0_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
            locals.var_ps0ld = assign73830_body77_e112497;
            locals.var_ps0ld_dn0 = assign73830_body77_e112497_d_n0;
            locals.var_ps0ld_dn2 = assign73830_body77_e112497_d_n2;
            locals.var_ps0ld_dn4 = assign73830_body77_e112497_d_n4;
            locals.var_ps0ld_dn5 = assign73830_body77_e112497_d_n5;
            locals.var_ps0ld_dn6 = assign73830_body77_e112497_d_n6;
            locals.var_ps0ld_dn7 = assign73830_body77_e112497_d_n7;
            locals.var_ps0ld_dn8 = assign73830_body77_e112497_d_n8;
            locals.var_ps0ld_dn9 = assign73830_body77_e112497_d_n9;
            locals.var_ps0ld_dn10 = assign73830_body77_e112497_d_n10;
            locals.var_ps0ld_dn11 = assign73830_body77_e112497_d_n11;
            locals.var_ps0ld_dn14 = assign73830_body77_e112497_d_n14;
            locals.var_ps0ld_rv = 0.0;
            let assign73830_body78_e112499: f64 = (locals.var_dps0).abs();
            let assign73830_body78_e112503: f64 = (locals.var_fs0).abs();
            let assign73830_body78_e112506: f64 = if ((assign73830_body78_e112499 <= 1e-12) && (assign73830_body78_e112503 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1722 = assign73830_body78_e112506;
            locals.var_guard1722_rv = 0.0;
            let (assign73830_body79_e112520,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) && (locals.var_guard1720 == 0.0)) && (locals.var_guard1722 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign73830_body79_e112520;
            locals.var_flg_conv_rv = 0.0;
            let (assign73830_body80_e112531,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73830_body80_e112529: f64 = (locals.var_lp_s0 + 1.0);
        (assign73830_body80_e112529,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign73830_body80_e112531;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_278(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign73850_e112545, assign73850_e112545_d_n0, assign73850_e112545_d_n2, assign73850_e112545_d_n4, assign73850_e112545_d_n5, assign73850_e112545_d_n6, assign73850_e112545_d_n7, assign73850_e112545_d_n8, assign73850_e112545_d_n9, assign73850_e112545_d_n10, assign73850_e112545_d_n11, assign73850_e112545_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73850_e112543: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign73850_e112543, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn11 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn11)), ((locals.var_c_w_ld_dn14 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn14)),)
    } else {
        (locals.var_wdld, locals.var_wdld_dn0, locals.var_wdld_dn2, locals.var_wdld_dn4, locals.var_wdld_dn5, locals.var_wdld_dn6, locals.var_wdld_dn7, locals.var_wdld_dn8, locals.var_wdld_dn9, locals.var_wdld_dn10, locals.var_wdld_dn11, locals.var_wdld_dn14,)
    }
};
        locals.var_wdld = assign73850_e112545;
        locals.var_wdld_dn0 = assign73850_e112545_d_n0;
        locals.var_wdld_dn2 = assign73850_e112545_d_n2;
        locals.var_wdld_dn4 = assign73850_e112545_d_n4;
        locals.var_wdld_dn5 = assign73850_e112545_d_n5;
        locals.var_wdld_dn6 = assign73850_e112545_d_n6;
        locals.var_wdld_dn7 = assign73850_e112545_d_n7;
        locals.var_wdld_dn8 = assign73850_e112545_d_n8;
        locals.var_wdld_dn9 = assign73850_e112545_d_n9;
        locals.var_wdld_dn10 = assign73850_e112545_d_n10;
        locals.var_wdld_dn11 = assign73850_e112545_d_n11;
        locals.var_wdld_dn14 = assign73850_e112545_d_n14;
        locals.var_wdld_rv = 0.0;

        let (assign73860_e112556, assign73860_e112556_d_n0, assign73860_e112556_d_n2, assign73860_e112556_d_n4, assign73860_e112556_d_n5, assign73860_e112556_d_n6, assign73860_e112556_d_n7, assign73860_e112556_d_n8, assign73860_e112556_d_n9, assign73860_e112556_d_n10, assign73860_e112556_d_n11, assign73860_e112556_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73860_e112554: f64 = (locals.var_q_nsubld * locals.var_wdld);
        (assign73860_e112554, (locals.var_q_nsubld * locals.var_wdld_dn0), (locals.var_q_nsubld * locals.var_wdld_dn2), (locals.var_q_nsubld * locals.var_wdld_dn4), (locals.var_q_nsubld * locals.var_wdld_dn5), (locals.var_q_nsubld * locals.var_wdld_dn6), (locals.var_q_nsubld * locals.var_wdld_dn7), (locals.var_q_nsubld * locals.var_wdld_dn8), (locals.var_q_nsubld * locals.var_wdld_dn9), (locals.var_q_nsubld * locals.var_wdld_dn10), (locals.var_q_nsubld * locals.var_wdld_dn11), (locals.var_q_nsubld * locals.var_wdld_dn14),)
    } else {
        (locals.var_q_dep_ld, locals.var_q_dep_ld_dn0, locals.var_q_dep_ld_dn2, locals.var_q_dep_ld_dn4, locals.var_q_dep_ld_dn5, locals.var_q_dep_ld_dn6, locals.var_q_dep_ld_dn7, locals.var_q_dep_ld_dn8, locals.var_q_dep_ld_dn9, locals.var_q_dep_ld_dn10, locals.var_q_dep_ld_dn11, locals.var_q_dep_ld_dn14,)
    }
};
        locals.var_q_dep_ld = assign73860_e112556;
        locals.var_q_dep_ld_dn0 = assign73860_e112556_d_n0;
        locals.var_q_dep_ld_dn2 = assign73860_e112556_d_n2;
        locals.var_q_dep_ld_dn4 = assign73860_e112556_d_n4;
        locals.var_q_dep_ld_dn5 = assign73860_e112556_d_n5;
        locals.var_q_dep_ld_dn6 = assign73860_e112556_d_n6;
        locals.var_q_dep_ld_dn7 = assign73860_e112556_d_n7;
        locals.var_q_dep_ld_dn8 = assign73860_e112556_d_n8;
        locals.var_q_dep_ld_dn9 = assign73860_e112556_d_n9;
        locals.var_q_dep_ld_dn10 = assign73860_e112556_d_n10;
        locals.var_q_dep_ld_dn11 = assign73860_e112556_d_n11;
        locals.var_q_dep_ld_dn14 = assign73860_e112556_d_n14;
        locals.var_q_dep_ld_rv = 0.0;

        let (assign73870_e112571, assign73870_e112571_d_n0, assign73870_e112571_d_n2, assign73870_e112571_d_n4, assign73870_e112571_d_n5, assign73870_e112571_d_n6, assign73870_e112571_d_n7, assign73870_e112571_d_n8, assign73870_e112571_d_n9, assign73870_e112571_d_n10, assign73870_e112571_d_n11, assign73870_e112571_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73870_e112565: f64 = (locals.var_q_dep_ld / locals.var_cnst0over_func);
        let assign73870_e112568: f64 = (10.0 * 2.220446049250313e-16);
        let assign73870_e112569: f64 = (assign73870_e112565 + assign73870_e112568);
        (assign73870_e112569, (((locals.var_q_dep_ld_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn11 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn11)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld_dn14 * locals.var_cnst0over_func) - (locals.var_q_dep_ld * locals.var_cnst0over_func_dn14)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn14,)
    }
};
        locals.var_xi0p12 = assign73870_e112571;
        locals.var_xi0p12_dn0 = assign73870_e112571_d_n0;
        locals.var_xi0p12_dn2 = assign73870_e112571_d_n2;
        locals.var_xi0p12_dn4 = assign73870_e112571_d_n4;
        locals.var_xi0p12_dn5 = assign73870_e112571_d_n5;
        locals.var_xi0p12_dn6 = assign73870_e112571_d_n6;
        locals.var_xi0p12_dn7 = assign73870_e112571_d_n7;
        locals.var_xi0p12_dn8 = assign73870_e112571_d_n8;
        locals.var_xi0p12_dn9 = assign73870_e112571_d_n9;
        locals.var_xi0p12_dn10 = assign73870_e112571_d_n10;
        locals.var_xi0p12_dn11 = assign73870_e112571_d_n11;
        locals.var_xi0p12_dn14 = assign73870_e112571_d_n14;
        locals.var_xi0p12_rv = 0.0;

        let (assign73880_e112582, assign73880_e112582_d_n0, assign73880_e112582_d_n2, assign73880_e112582_d_n4, assign73880_e112582_d_n5, assign73880_e112582_d_n6, assign73880_e112582_d_n7, assign73880_e112582_d_n8, assign73880_e112582_d_n9, assign73880_e112582_d_n10, assign73880_e112582_d_n11, assign73880_e112582_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73880_e112580: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign73880_e112580, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign73880_e112582;
        locals.var_qbuld_dn0 = assign73880_e112582_d_n0;
        locals.var_qbuld_dn2 = assign73880_e112582_d_n2;
        locals.var_qbuld_dn4 = assign73880_e112582_d_n4;
        locals.var_qbuld_dn5 = assign73880_e112582_d_n5;
        locals.var_qbuld_dn6 = assign73880_e112582_d_n6;
        locals.var_qbuld_dn7 = assign73880_e112582_d_n7;
        locals.var_qbuld_dn8 = assign73880_e112582_d_n8;
        locals.var_qbuld_dn9 = assign73880_e112582_d_n9;
        locals.var_qbuld_dn10 = assign73880_e112582_d_n10;
        locals.var_qbuld_dn11 = assign73880_e112582_d_n11;
        locals.var_qbuld_dn14 = assign73880_e112582_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign73890_e112595, assign73890_e112595_d_n0, assign73890_e112595_d_n2, assign73890_e112595_d_n4, assign73890_e112595_d_n5, assign73890_e112595_d_n6, assign73890_e112595_d_n7, assign73890_e112595_d_n8, assign73890_e112595_d_n9, assign73890_e112595_d_n10, assign73890_e112595_d_n11, assign73890_e112595_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73890_e112592: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign73890_e112593: f64 = (1.0 / assign73890_e112592);
        (assign73890_e112593, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign73890_e112592 * assign73890_e112592))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign73890_e112592 * assign73890_e112592))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign73890_e112592 * assign73890_e112592))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign73890_e112592 * assign73890_e112592))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign73890_e112592 * assign73890_e112592))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign73890_e112592 * assign73890_e112592))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign73890_e112592 * assign73890_e112592))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign73890_e112592 * assign73890_e112592))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign73890_e112592 * assign73890_e112592))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign73890_e112592 * assign73890_e112592))), (-((locals.var_fs02_dn14 + locals.var_xi0p12_dn14) / (assign73890_e112592 * assign73890_e112592))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign73890_e112595;
        locals.var_t1_dn0 = assign73890_e112595_d_n0;
        locals.var_t1_dn2 = assign73890_e112595_d_n2;
        locals.var_t1_dn4 = assign73890_e112595_d_n4;
        locals.var_t1_dn5 = assign73890_e112595_d_n5;
        locals.var_t1_dn6 = assign73890_e112595_d_n6;
        locals.var_t1_dn7 = assign73890_e112595_d_n7;
        locals.var_t1_dn8 = assign73890_e112595_d_n8;
        locals.var_t1_dn9 = assign73890_e112595_d_n9;
        locals.var_t1_dn10 = assign73890_e112595_d_n10;
        locals.var_t1_dn11 = assign73890_e112595_d_n11;
        locals.var_t1_dn14 = assign73890_e112595_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign73900_e112608, assign73900_e112608_d_n0, assign73900_e112608_d_n2, assign73900_e112608_d_n4, assign73900_e112608_d_n5, assign73900_e112608_d_n6, assign73900_e112608_d_n7, assign73900_e112608_d_n8, assign73900_e112608_d_n9, assign73900_e112608_d_n10, assign73900_e112608_d_n11, assign73900_e112608_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73900_e112604: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign73900_e112606: f64 = (assign73900_e112604 * locals.var_t1);
        (assign73900_e112606, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign73900_e112604 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign73900_e112604 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign73900_e112604 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign73900_e112604 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign73900_e112604 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign73900_e112604 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign73900_e112604 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign73900_e112604 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign73900_e112604 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn11 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn11)) * locals.var_t1) + (assign73900_e112604 * locals.var_t1_dn11)), ((((locals.var_cnst0over_func_dn14 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn14)) * locals.var_t1) + (assign73900_e112604 * locals.var_t1_dn14)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn14,)
    }
};
        locals.var_qiuld = assign73900_e112608;
        locals.var_qiuld_dn0 = assign73900_e112608_d_n0;
        locals.var_qiuld_dn2 = assign73900_e112608_d_n2;
        locals.var_qiuld_dn4 = assign73900_e112608_d_n4;
        locals.var_qiuld_dn5 = assign73900_e112608_d_n5;
        locals.var_qiuld_dn6 = assign73900_e112608_d_n6;
        locals.var_qiuld_dn7 = assign73900_e112608_d_n7;
        locals.var_qiuld_dn8 = assign73900_e112608_d_n8;
        locals.var_qiuld_dn9 = assign73900_e112608_d_n9;
        locals.var_qiuld_dn10 = assign73900_e112608_d_n10;
        locals.var_qiuld_dn11 = assign73900_e112608_d_n11;
        locals.var_qiuld_dn14 = assign73900_e112608_d_n14;
        locals.var_qiuld_rv = 0.0;

        let (assign73910_e112619, assign73910_e112619_d_n0, assign73910_e112619_d_n2, assign73910_e112619_d_n4, assign73910_e112619_d_n5, assign73910_e112619_d_n6, assign73910_e112619_d_n7, assign73910_e112619_d_n8, assign73910_e112619_d_n9, assign73910_e112619_d_n10, assign73910_e112619_d_n11, assign73910_e112619_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1667 == 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign73910_e112617: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign73910_e112617, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn14 + locals.var_qiuld_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign73910_e112619;
        locals.var_qsuld_dn0 = assign73910_e112619_d_n0;
        locals.var_qsuld_dn2 = assign73910_e112619_d_n2;
        locals.var_qsuld_dn4 = assign73910_e112619_d_n4;
        locals.var_qsuld_dn5 = assign73910_e112619_d_n5;
        locals.var_qsuld_dn6 = assign73910_e112619_d_n6;
        locals.var_qsuld_dn7 = assign73910_e112619_d_n7;
        locals.var_qsuld_dn8 = assign73910_e112619_d_n8;
        locals.var_qsuld_dn9 = assign73910_e112619_d_n9;
        locals.var_qsuld_dn10 = assign73910_e112619_d_n10;
        locals.var_qsuld_dn11 = assign73910_e112619_d_n11;
        locals.var_qsuld_dn14 = assign73910_e112619_d_n14;
        locals.var_qsuld_rv = 0.0;

        let assign73920_e112622: f64 = if p.p33 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1724 = assign73920_e112622;
        locals.var_guard1724_rv = 0.0;

        let (assign73930_e112632, assign73930_e112632_d_n0, assign73930_e112632_d_n2, assign73930_e112632_d_n4, assign73930_e112632_d_n5, assign73930_e112632_d_n6, assign73930_e112632_d_n7, assign73930_e112632_d_n8, assign73930_e112632_d_n9, assign73930_e112632_d_n10, assign73930_e112632_d_n11, assign73930_e112632_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) {
        let assign73930_e112628: f64 = (-locals.var_vxbgmtcl);
        let assign73930_e112629: f64 = (locals.var_beta * assign73930_e112628);
        let assign73930_e112630: f64 = (assign73930_e112629).exp();
        (assign73930_e112630, (assign73930_e112630 * ((locals.var_beta_dn0 * assign73930_e112628) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (assign73930_e112630 * ((locals.var_beta_dn2 * assign73930_e112628) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (assign73930_e112630 * ((locals.var_beta_dn4 * assign73930_e112628) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign73930_e112630 * ((locals.var_beta_dn5 * assign73930_e112628) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (assign73930_e112630 * ((locals.var_beta_dn6 * assign73930_e112628) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (assign73930_e112630 * ((locals.var_beta_dn7 * assign73930_e112628) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (assign73930_e112630 * ((locals.var_beta_dn8 * assign73930_e112628) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (assign73930_e112630 * ((locals.var_beta_dn9 * assign73930_e112628) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (assign73930_e112630 * ((locals.var_beta_dn10 * assign73930_e112628) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign73930_e112630 * ((locals.var_beta_dn11 * assign73930_e112628) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11)))), (assign73930_e112630 * ((locals.var_beta_dn14 * assign73930_e112628) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign73930_e112632;
        locals.var_exp_bvbs_dn0 = assign73930_e112632_d_n0;
        locals.var_exp_bvbs_dn2 = assign73930_e112632_d_n2;
        locals.var_exp_bvbs_dn4 = assign73930_e112632_d_n4;
        locals.var_exp_bvbs_dn5 = assign73930_e112632_d_n5;
        locals.var_exp_bvbs_dn6 = assign73930_e112632_d_n6;
        locals.var_exp_bvbs_dn7 = assign73930_e112632_d_n7;
        locals.var_exp_bvbs_dn8 = assign73930_e112632_d_n8;
        locals.var_exp_bvbs_dn9 = assign73930_e112632_d_n9;
        locals.var_exp_bvbs_dn10 = assign73930_e112632_d_n10;
        locals.var_exp_bvbs_dn11 = assign73930_e112632_d_n11;
        locals.var_exp_bvbs_dn14 = assign73930_e112632_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign73940_e112640, assign73940_e112640_d_n0, assign73940_e112640_d_n2, assign73940_e112640_d_n4, assign73940_e112640_d_n5, assign73940_e112640_d_n6, assign73940_e112640_d_n7, assign73940_e112640_d_n8, assign73940_e112640_d_n9, assign73940_e112640_d_n10, assign73940_e112640_d_n11, assign73940_e112640_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) {
        let assign73940_e112638: f64 = (locals.var_nin / locals.var_nover_func);
        (assign73940_e112638, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn11 / locals.var_nover_func), (locals.var_nin_dn14 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign73940_e112640;
        locals.var_t0_dn0 = assign73940_e112640_d_n0;
        locals.var_t0_dn2 = assign73940_e112640_d_n2;
        locals.var_t0_dn4 = assign73940_e112640_d_n4;
        locals.var_t0_dn5 = assign73940_e112640_d_n5;
        locals.var_t0_dn6 = assign73940_e112640_d_n6;
        locals.var_t0_dn7 = assign73940_e112640_d_n7;
        locals.var_t0_dn8 = assign73940_e112640_d_n8;
        locals.var_t0_dn9 = assign73940_e112640_d_n9;
        locals.var_t0_dn10 = assign73940_e112640_d_n10;
        locals.var_t0_dn11 = assign73940_e112640_d_n11;
        locals.var_t0_dn14 = assign73940_e112640_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign73950_e112648, assign73950_e112648_d_n0, assign73950_e112648_d_n2, assign73950_e112648_d_n4, assign73950_e112648_d_n5, assign73950_e112648_d_n6, assign73950_e112648_d_n7, assign73950_e112648_d_n8, assign73950_e112648_d_n9, assign73950_e112648_d_n10, assign73950_e112648_d_n11, assign73950_e112648_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) {
        let assign73950_e112646: f64 = (locals.var_t0 * locals.var_t0);
        (assign73950_e112646, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn14,)
    }
};
        locals.var_cnst1over = assign73950_e112648;
        locals.var_cnst1over_dn0 = assign73950_e112648_d_n0;
        locals.var_cnst1over_dn2 = assign73950_e112648_d_n2;
        locals.var_cnst1over_dn4 = assign73950_e112648_d_n4;
        locals.var_cnst1over_dn5 = assign73950_e112648_d_n5;
        locals.var_cnst1over_dn6 = assign73950_e112648_d_n6;
        locals.var_cnst1over_dn7 = assign73950_e112648_d_n7;
        locals.var_cnst1over_dn8 = assign73950_e112648_d_n8;
        locals.var_cnst1over_dn9 = assign73950_e112648_d_n9;
        locals.var_cnst1over_dn10 = assign73950_e112648_d_n10;
        locals.var_cnst1over_dn11 = assign73950_e112648_d_n11;
        locals.var_cnst1over_dn14 = assign73950_e112648_d_n14;
        locals.var_cnst1over_rv = 0.0;

        let (assign73960_e112656, assign73960_e112656_d_n0, assign73960_e112656_d_n2, assign73960_e112656_d_n4, assign73960_e112656_d_n5, assign73960_e112656_d_n6, assign73960_e112656_d_n7, assign73960_e112656_d_n8, assign73960_e112656_d_n9, assign73960_e112656_d_n10, assign73960_e112656_d_n11, assign73960_e112656_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) {
        let assign73960_e112654: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign73960_e112654, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn14,)
    }
};
        locals.var_cfs1 = assign73960_e112656;
        locals.var_cfs1_dn0 = assign73960_e112656_d_n0;
        locals.var_cfs1_dn2 = assign73960_e112656_d_n2;
        locals.var_cfs1_dn4 = assign73960_e112656_d_n4;
        locals.var_cfs1_dn5 = assign73960_e112656_d_n5;
        locals.var_cfs1_dn6 = assign73960_e112656_d_n6;
        locals.var_cfs1_dn7 = assign73960_e112656_d_n7;
        locals.var_cfs1_dn8 = assign73960_e112656_d_n8;
        locals.var_cfs1_dn9 = assign73960_e112656_d_n9;
        locals.var_cfs1_dn10 = assign73960_e112656_d_n10;
        locals.var_cfs1_dn11 = assign73960_e112656_d_n11;
        locals.var_cfs1_dn14 = assign73960_e112656_d_n14;
        locals.var_cfs1_rv = 0.0;

        let (assign73970_e112662, assign73970_e112662_d_n0, assign73970_e112662_d_n2, assign73970_e112662_d_n4, assign73970_e112662_d_n5, assign73970_e112662_d_n6, assign73970_e112662_d_n7, assign73970_e112662_d_n8, assign73970_e112662_d_n9, assign73970_e112662_d_n10, assign73970_e112662_d_n11, assign73970_e112662_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) {
        (locals.var_ps0ld_ini, locals.var_ps0ld_ini_dn0, locals.var_ps0ld_ini_dn2, locals.var_ps0ld_ini_dn4, locals.var_ps0ld_ini_dn5, locals.var_ps0ld_ini_dn6, locals.var_ps0ld_ini_dn7, locals.var_ps0ld_ini_dn8, locals.var_ps0ld_ini_dn9, locals.var_ps0ld_ini_dn10, locals.var_ps0ld_ini_dn11, locals.var_ps0ld_ini_dn14,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign73970_e112662;
        locals.var_ps0ld_dn0 = assign73970_e112662_d_n0;
        locals.var_ps0ld_dn2 = assign73970_e112662_d_n2;
        locals.var_ps0ld_dn4 = assign73970_e112662_d_n4;
        locals.var_ps0ld_dn5 = assign73970_e112662_d_n5;
        locals.var_ps0ld_dn6 = assign73970_e112662_d_n6;
        locals.var_ps0ld_dn7 = assign73970_e112662_d_n7;
        locals.var_ps0ld_dn8 = assign73970_e112662_d_n8;
        locals.var_ps0ld_dn9 = assign73970_e112662_d_n9;
        locals.var_ps0ld_dn10 = assign73970_e112662_d_n10;
        locals.var_ps0ld_dn11 = assign73970_e112662_d_n11;
        locals.var_ps0ld_dn14 = assign73970_e112662_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let (assign73980_e112668,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign73980_e112668;
        locals.var_flg_conv_rv = 0.0;

        let (assign73990_e112681, assign73990_e112681_d_n0, assign73990_e112681_d_n2, assign73990_e112681_d_n4, assign73990_e112681_d_n5, assign73990_e112681_d_n6, assign73990_e112681_d_n7, assign73990_e112681_d_n8, assign73990_e112681_d_n9, assign73990_e112681_d_n10, assign73990_e112681_d_n11, assign73990_e112681_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) {
        let assign73990_e112675: f64 = (1.034943e-10 / locals.var_q_nsubld);
        let assign73990_e112677: f64 = (assign73990_e112675 * locals.var_beta_inv);
        let assign73990_e112678: f64 = (2.0 * assign73990_e112677);
        let assign73990_e112679: f64 = (assign73990_e112678).sqrt();
        (assign73990_e112679, ((2.0 * (assign73990_e112675 * locals.var_beta_inv_dn0)) / (2.0 * assign73990_e112679)), ((2.0 * (assign73990_e112675 * locals.var_beta_inv_dn2)) / (2.0 * assign73990_e112679)), ((2.0 * (assign73990_e112675 * locals.var_beta_inv_dn4)) / (2.0 * assign73990_e112679)), ((2.0 * (assign73990_e112675 * locals.var_beta_inv_dn5)) / (2.0 * assign73990_e112679)), ((2.0 * (assign73990_e112675 * locals.var_beta_inv_dn6)) / (2.0 * assign73990_e112679)), ((2.0 * (assign73990_e112675 * locals.var_beta_inv_dn7)) / (2.0 * assign73990_e112679)), ((2.0 * (assign73990_e112675 * locals.var_beta_inv_dn8)) / (2.0 * assign73990_e112679)), ((2.0 * (assign73990_e112675 * locals.var_beta_inv_dn9)) / (2.0 * assign73990_e112679)), ((2.0 * (assign73990_e112675 * locals.var_beta_inv_dn10)) / (2.0 * assign73990_e112679)), ((2.0 * (assign73990_e112675 * locals.var_beta_inv_dn11)) / (2.0 * assign73990_e112679)), ((2.0 * (assign73990_e112675 * locals.var_beta_inv_dn14)) / (2.0 * assign73990_e112679)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn11, locals.var_c_w_ld_dn14,)
    }
};
        locals.var_c_w_ld = assign73990_e112681;
        locals.var_c_w_ld_dn0 = assign73990_e112681_d_n0;
        locals.var_c_w_ld_dn2 = assign73990_e112681_d_n2;
        locals.var_c_w_ld_dn4 = assign73990_e112681_d_n4;
        locals.var_c_w_ld_dn5 = assign73990_e112681_d_n5;
        locals.var_c_w_ld_dn6 = assign73990_e112681_d_n6;
        locals.var_c_w_ld_dn7 = assign73990_e112681_d_n7;
        locals.var_c_w_ld_dn8 = assign73990_e112681_d_n8;
        locals.var_c_w_ld_dn9 = assign73990_e112681_d_n9;
        locals.var_c_w_ld_dn10 = assign73990_e112681_d_n10;
        locals.var_c_w_ld_dn11 = assign73990_e112681_d_n11;
        locals.var_c_w_ld_dn14 = assign73990_e112681_d_n14;
        locals.var_c_w_ld_rv = 0.0;

        let assign74000_e112684: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1725 = assign74000_e112684;
        locals.var_guard1725_rv = 0.0;

        let (assign74010_e112694, assign74010_e112694_d_n0, assign74010_e112694_d_n2, assign74010_e112694_d_n4, assign74010_e112694_d_n5, assign74010_e112694_d_n6, assign74010_e112694_d_n7, assign74010_e112694_d_n8, assign74010_e112694_d_n9, assign74010_e112694_d_n10, assign74010_e112694_d_n11, assign74010_e112694_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) && (locals.var_guard1725 != 0.0)) {
        let assign74010_e112692: f64 = (p.p334 - locals.var_wdep_func);
        (assign74010_e112692, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn11), (-locals.var_wdep_func_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign74010_e112694;
        locals.var_t2_dn0 = assign74010_e112694_d_n0;
        locals.var_t2_dn2 = assign74010_e112694_d_n2;
        locals.var_t2_dn4 = assign74010_e112694_d_n4;
        locals.var_t2_dn5 = assign74010_e112694_d_n5;
        locals.var_t2_dn6 = assign74010_e112694_d_n6;
        locals.var_t2_dn7 = assign74010_e112694_d_n7;
        locals.var_t2_dn8 = assign74010_e112694_d_n8;
        locals.var_t2_dn9 = assign74010_e112694_d_n9;
        locals.var_t2_dn10 = assign74010_e112694_d_n10;
        locals.var_t2_dn11 = assign74010_e112694_d_n11;
        locals.var_t2_dn14 = assign74010_e112694_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign74020_e112716, assign74020_e112716_d_n0, assign74020_e112716_d_n2, assign74020_e112716_d_n4, assign74020_e112716_d_n5, assign74020_e112716_d_n6, assign74020_e112716_d_n7, assign74020_e112716_d_n8, assign74020_e112716_d_n9, assign74020_e112716_d_n10, assign74020_e112716_d_n11, assign74020_e112716_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) && (locals.var_guard1725 == 0.0)) {
        let assign74020_e112703: f64 = (locals.var_vdsi + p.p137);
        let assign74020_e112706: f64 = (locals.var_vdsi + p.p137);
        let assign74020_e112707: f64 = (assign74020_e112703 * assign74020_e112706);
        let assign74020_e112710: f64 = (4.0 * 0.1);
        let assign74020_e112712: f64 = (assign74020_e112710 * 0.1);
        let assign74020_e112713: f64 = (assign74020_e112707 + assign74020_e112712);
        let assign74020_e112714: f64 = (assign74020_e112713).sqrt();
        (assign74020_e112714, 0.0, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn6 * assign74020_e112706) + (assign74020_e112703 * locals.var_vdsi_dn6)) / (2.0 * assign74020_e112714)), 0.0, (((locals.var_vdsi_dn8 * assign74020_e112706) + (assign74020_e112703 * locals.var_vdsi_dn8)) / (2.0 * assign74020_e112714)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign74020_e112716;
        locals.var_tmf2_dn0 = assign74020_e112716_d_n0;
        locals.var_tmf2_dn2 = assign74020_e112716_d_n2;
        locals.var_tmf2_dn4 = assign74020_e112716_d_n4;
        locals.var_tmf2_dn5 = assign74020_e112716_d_n5;
        locals.var_tmf2_dn6 = assign74020_e112716_d_n6;
        locals.var_tmf2_dn7 = assign74020_e112716_d_n7;
        locals.var_tmf2_dn8 = assign74020_e112716_d_n8;
        locals.var_tmf2_dn9 = assign74020_e112716_d_n9;
        locals.var_tmf2_dn10 = assign74020_e112716_d_n10;
        locals.var_tmf2_dn11 = assign74020_e112716_d_n11;
        locals.var_tmf2_dn14 = assign74020_e112716_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign74030_e112733, assign74030_e112733_d_n0, assign74030_e112733_d_n2, assign74030_e112733_d_n4, assign74030_e112733_d_n5, assign74030_e112733_d_n6, assign74030_e112733_d_n7, assign74030_e112733_d_n8, assign74030_e112733_d_n9, assign74030_e112733_d_n10, assign74030_e112733_d_n11, assign74030_e112733_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) && (locals.var_guard1725 == 0.0)) {
        let assign74030_e112727: f64 = (locals.var_vdsi + p.p137);
        let assign74030_e112729: f64 = (assign74030_e112727 / locals.var_tmf2);
        let assign74030_e112730: f64 = (1.0 + assign74030_e112729);
        let assign74030_e112731: f64 = (0.5 * assign74030_e112730);
        (assign74030_e112731, (0.5 * (-((assign74030_e112727 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign74030_e112727 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign74030_e112727 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign74030_e112727 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn6 * locals.var_tmf2) - (assign74030_e112727 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign74030_e112727 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn8 * locals.var_tmf2) - (assign74030_e112727 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign74030_e112727 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign74030_e112727 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign74030_e112727 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign74030_e112727 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign74030_e112733;
        locals.var_t9_dn0 = assign74030_e112733_d_n0;
        locals.var_t9_dn2 = assign74030_e112733_d_n2;
        locals.var_t9_dn4 = assign74030_e112733_d_n4;
        locals.var_t9_dn5 = assign74030_e112733_d_n5;
        locals.var_t9_dn6 = assign74030_e112733_d_n6;
        locals.var_t9_dn7 = assign74030_e112733_d_n7;
        locals.var_t9_dn8 = assign74030_e112733_d_n8;
        locals.var_t9_dn9 = assign74030_e112733_d_n9;
        locals.var_t9_dn10 = assign74030_e112733_d_n10;
        locals.var_t9_dn11 = assign74030_e112733_d_n11;
        locals.var_t9_dn14 = assign74030_e112733_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign74040_e112748, assign74040_e112748_d_n0, assign74040_e112748_d_n2, assign74040_e112748_d_n4, assign74040_e112748_d_n5, assign74040_e112748_d_n6, assign74040_e112748_d_n7, assign74040_e112748_d_n8, assign74040_e112748_d_n9, assign74040_e112748_d_n10, assign74040_e112748_d_n11, assign74040_e112748_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) && (locals.var_guard1725 == 0.0)) {
        let assign74040_e112743: f64 = (locals.var_vdsi + p.p137);
        let assign74040_e112745: f64 = (assign74040_e112743 + locals.var_tmf2);
        let assign74040_e112746: f64 = (0.5 * assign74040_e112745);
        (assign74040_e112746, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * (locals.var_vdsi_dn6 + locals.var_tmf2_dn6)), (0.5 * locals.var_tmf2_dn7), (0.5 * (locals.var_vdsi_dn8 + locals.var_tmf2_dn8)), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign74040_e112748;
        locals.var_t2_dn0 = assign74040_e112748_d_n0;
        locals.var_t2_dn2 = assign74040_e112748_d_n2;
        locals.var_t2_dn4 = assign74040_e112748_d_n4;
        locals.var_t2_dn5 = assign74040_e112748_d_n5;
        locals.var_t2_dn6 = assign74040_e112748_d_n6;
        locals.var_t2_dn7 = assign74040_e112748_d_n7;
        locals.var_t2_dn8 = assign74040_e112748_d_n8;
        locals.var_t2_dn9 = assign74040_e112748_d_n9;
        locals.var_t2_dn10 = assign74040_e112748_d_n10;
        locals.var_t2_dn11 = assign74040_e112748_d_n11;
        locals.var_t2_dn14 = assign74040_e112748_d_n14;
        locals.var_t2_rv = 0.0;

        let assign74050_e112751: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1726 = assign74050_e112751;
        locals.var_guard1726_rv = 0.0;

        let (assign74060_e112762, assign74060_e112762_d_n0, assign74060_e112762_d_n2, assign74060_e112762_d_n4, assign74060_e112762_d_n5, assign74060_e112762_d_n6, assign74060_e112762_d_n7, assign74060_e112762_d_n8, assign74060_e112762_d_n9, assign74060_e112762_d_n10, assign74060_e112762_d_n11, assign74060_e112762_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) && (locals.var_guard1725 == 0.0)) && (locals.var_guard1726 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign74060_e112762;
        locals.var_t2_dn0 = assign74060_e112762_d_n0;
        locals.var_t2_dn2 = assign74060_e112762_d_n2;
        locals.var_t2_dn4 = assign74060_e112762_d_n4;
        locals.var_t2_dn5 = assign74060_e112762_d_n5;
        locals.var_t2_dn6 = assign74060_e112762_d_n6;
        locals.var_t2_dn7 = assign74060_e112762_d_n7;
        locals.var_t2_dn8 = assign74060_e112762_d_n8;
        locals.var_t2_dn9 = assign74060_e112762_d_n9;
        locals.var_t2_dn10 = assign74060_e112762_d_n10;
        locals.var_t2_dn11 = assign74060_e112762_d_n11;
        locals.var_t2_dn14 = assign74060_e112762_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign74070_e112773, assign74070_e112773_d_n0, assign74070_e112773_d_n2, assign74070_e112773_d_n4, assign74070_e112773_d_n5, assign74070_e112773_d_n6, assign74070_e112773_d_n7, assign74070_e112773_d_n8, assign74070_e112773_d_n9, assign74070_e112773_d_n10, assign74070_e112773_d_n11, assign74070_e112773_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) && (locals.var_guard1725 == 0.0)) && (locals.var_guard1726 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign74070_e112773;
        locals.var_t9_dn0 = assign74070_e112773_d_n0;
        locals.var_t9_dn2 = assign74070_e112773_d_n2;
        locals.var_t9_dn4 = assign74070_e112773_d_n4;
        locals.var_t9_dn5 = assign74070_e112773_d_n5;
        locals.var_t9_dn6 = assign74070_e112773_d_n6;
        locals.var_t9_dn7 = assign74070_e112773_d_n7;
        locals.var_t9_dn8 = assign74070_e112773_d_n8;
        locals.var_t9_dn9 = assign74070_e112773_d_n9;
        locals.var_t9_dn10 = assign74070_e112773_d_n10;
        locals.var_t9_dn11 = assign74070_e112773_d_n11;
        locals.var_t9_dn14 = assign74070_e112773_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign74080_e112787, assign74080_e112787_d_n0, assign74080_e112787_d_n2, assign74080_e112787_d_n4, assign74080_e112787_d_n5, assign74080_e112787_d_n6, assign74080_e112787_d_n7, assign74080_e112787_d_n8, assign74080_e112787_d_n9, assign74080_e112787_d_n10, assign74080_e112787_d_n11, assign74080_e112787_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) && (locals.var_guard1725 == 0.0)) {
        let assign74080_e112782: f64 = (locals.var_kjunc * locals.var_t2);
        let assign74080_e112783: f64 = (assign74080_e112782).sqrt();
        let assign74080_e112785: f64 = (assign74080_e112783 * p.p432);
        (assign74080_e112785, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign74080_e112783)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign74080_e112783)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign74080_e112783)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign74080_e112783)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign74080_e112783)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign74080_e112783)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign74080_e112783)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign74080_e112783)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign74080_e112783)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign74080_e112783)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign74080_e112783)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign74080_e112787;
        locals.var_wjunc0_dn0 = assign74080_e112787_d_n0;
        locals.var_wjunc0_dn2 = assign74080_e112787_d_n2;
        locals.var_wjunc0_dn4 = assign74080_e112787_d_n4;
        locals.var_wjunc0_dn5 = assign74080_e112787_d_n5;
        locals.var_wjunc0_dn6 = assign74080_e112787_d_n6;
        locals.var_wjunc0_dn7 = assign74080_e112787_d_n7;
        locals.var_wjunc0_dn8 = assign74080_e112787_d_n8;
        locals.var_wjunc0_dn9 = assign74080_e112787_d_n9;
        locals.var_wjunc0_dn10 = assign74080_e112787_d_n10;
        locals.var_wjunc0_dn11 = assign74080_e112787_d_n11;
        locals.var_wjunc0_dn14 = assign74080_e112787_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign74090_e112798, assign74090_e112798_d_n0, assign74090_e112798_d_n2, assign74090_e112798_d_n4, assign74090_e112798_d_n5, assign74090_e112798_d_n6, assign74090_e112798_d_n7, assign74090_e112798_d_n8, assign74090_e112798_d_n9, assign74090_e112798_d_n10, assign74090_e112798_d_n11, assign74090_e112798_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) && (locals.var_guard1725 == 0.0)) {
        let assign74090_e112796: f64 = (p.p334 - locals.var_wjunc0);
        (assign74090_e112796, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign74090_e112798;
        locals.var_t2_dn0 = assign74090_e112798_d_n0;
        locals.var_t2_dn2 = assign74090_e112798_d_n2;
        locals.var_t2_dn4 = assign74090_e112798_d_n4;
        locals.var_t2_dn5 = assign74090_e112798_d_n5;
        locals.var_t2_dn6 = assign74090_e112798_d_n6;
        locals.var_t2_dn7 = assign74090_e112798_d_n7;
        locals.var_t2_dn8 = assign74090_e112798_d_n8;
        locals.var_t2_dn9 = assign74090_e112798_d_n9;
        locals.var_t2_dn10 = assign74090_e112798_d_n10;
        locals.var_t2_dn11 = assign74090_e112798_d_n11;
        locals.var_t2_dn14 = assign74090_e112798_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_279(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign74100_e112817, assign74100_e112817_d_n0, assign74100_e112817_d_n2, assign74100_e112817_d_n4, assign74100_e112817_d_n5, assign74100_e112817_d_n6, assign74100_e112817_d_n7, assign74100_e112817_d_n8, assign74100_e112817_d_n9, assign74100_e112817_d_n10, assign74100_e112817_d_n11, assign74100_e112817_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) {
        let assign74100_e112804: f64 = (locals.var_t2 * locals.var_t2);
        let assign74100_e112808: f64 = (p.p334 * 0.01);
        let assign74100_e112809: f64 = (4.0 * assign74100_e112808);
        let assign74100_e112812: f64 = (p.p334 * 0.01);
        let assign74100_e112813: f64 = (assign74100_e112809 * assign74100_e112812);
        let assign74100_e112814: f64 = (assign74100_e112804 + assign74100_e112813);
        let assign74100_e112815: f64 = (assign74100_e112814).sqrt();
        (assign74100_e112815, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign74100_e112815)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign74100_e112815)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign74100_e112815)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign74100_e112815)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign74100_e112815)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign74100_e112815)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign74100_e112815)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign74100_e112815)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign74100_e112815)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign74100_e112815)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign74100_e112815)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign74100_e112817;
        locals.var_tmf2_dn0 = assign74100_e112817_d_n0;
        locals.var_tmf2_dn2 = assign74100_e112817_d_n2;
        locals.var_tmf2_dn4 = assign74100_e112817_d_n4;
        locals.var_tmf2_dn5 = assign74100_e112817_d_n5;
        locals.var_tmf2_dn6 = assign74100_e112817_d_n6;
        locals.var_tmf2_dn7 = assign74100_e112817_d_n7;
        locals.var_tmf2_dn8 = assign74100_e112817_d_n8;
        locals.var_tmf2_dn9 = assign74100_e112817_d_n9;
        locals.var_tmf2_dn10 = assign74100_e112817_d_n10;
        locals.var_tmf2_dn11 = assign74100_e112817_d_n11;
        locals.var_tmf2_dn14 = assign74100_e112817_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign74110_e112829, assign74110_e112829_d_n0, assign74110_e112829_d_n2, assign74110_e112829_d_n4, assign74110_e112829_d_n5, assign74110_e112829_d_n6, assign74110_e112829_d_n7, assign74110_e112829_d_n8, assign74110_e112829_d_n9, assign74110_e112829_d_n10, assign74110_e112829_d_n11, assign74110_e112829_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) {
        let assign74110_e112825: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign74110_e112826: f64 = (1.0 + assign74110_e112825);
        let assign74110_e112827: f64 = (0.5 * assign74110_e112826);
        (assign74110_e112827, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign74110_e112829;
        locals.var_t9_dn0 = assign74110_e112829_d_n0;
        locals.var_t9_dn2 = assign74110_e112829_d_n2;
        locals.var_t9_dn4 = assign74110_e112829_d_n4;
        locals.var_t9_dn5 = assign74110_e112829_d_n5;
        locals.var_t9_dn6 = assign74110_e112829_d_n6;
        locals.var_t9_dn7 = assign74110_e112829_d_n7;
        locals.var_t9_dn8 = assign74110_e112829_d_n8;
        locals.var_t9_dn9 = assign74110_e112829_d_n9;
        locals.var_t9_dn10 = assign74110_e112829_d_n10;
        locals.var_t9_dn11 = assign74110_e112829_d_n11;
        locals.var_t9_dn14 = assign74110_e112829_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign74120_e112839, assign74120_e112839_d_n0, assign74120_e112839_d_n2, assign74120_e112839_d_n4, assign74120_e112839_d_n5, assign74120_e112839_d_n6, assign74120_e112839_d_n7, assign74120_e112839_d_n8, assign74120_e112839_d_n9, assign74120_e112839_d_n10, assign74120_e112839_d_n11, assign74120_e112839_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) {
        let assign74120_e112836: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign74120_e112837: f64 = (0.5 * assign74120_e112836);
        (assign74120_e112837, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign74120_e112839;
        locals.var_t2_dn0 = assign74120_e112839_d_n0;
        locals.var_t2_dn2 = assign74120_e112839_d_n2;
        locals.var_t2_dn4 = assign74120_e112839_d_n4;
        locals.var_t2_dn5 = assign74120_e112839_d_n5;
        locals.var_t2_dn6 = assign74120_e112839_d_n6;
        locals.var_t2_dn7 = assign74120_e112839_d_n7;
        locals.var_t2_dn8 = assign74120_e112839_d_n8;
        locals.var_t2_dn9 = assign74120_e112839_d_n9;
        locals.var_t2_dn10 = assign74120_e112839_d_n10;
        locals.var_t2_dn11 = assign74120_e112839_d_n11;
        locals.var_t2_dn14 = assign74120_e112839_d_n14;
        locals.var_t2_rv = 0.0;

        let assign74130_e112842: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1727 = assign74130_e112842;
        locals.var_guard1727_rv = 0.0;

        let (assign74140_e112850, assign74140_e112850_d_n0, assign74140_e112850_d_n2, assign74140_e112850_d_n4, assign74140_e112850_d_n5, assign74140_e112850_d_n6, assign74140_e112850_d_n7, assign74140_e112850_d_n8, assign74140_e112850_d_n9, assign74140_e112850_d_n10, assign74140_e112850_d_n11, assign74140_e112850_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) && (locals.var_guard1727 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign74140_e112850;
        locals.var_t2_dn0 = assign74140_e112850_d_n0;
        locals.var_t2_dn2 = assign74140_e112850_d_n2;
        locals.var_t2_dn4 = assign74140_e112850_d_n4;
        locals.var_t2_dn5 = assign74140_e112850_d_n5;
        locals.var_t2_dn6 = assign74140_e112850_d_n6;
        locals.var_t2_dn7 = assign74140_e112850_d_n7;
        locals.var_t2_dn8 = assign74140_e112850_d_n8;
        locals.var_t2_dn9 = assign74140_e112850_d_n9;
        locals.var_t2_dn10 = assign74140_e112850_d_n10;
        locals.var_t2_dn11 = assign74140_e112850_d_n11;
        locals.var_t2_dn14 = assign74140_e112850_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign74150_e112858, assign74150_e112858_d_n0, assign74150_e112858_d_n2, assign74150_e112858_d_n4, assign74150_e112858_d_n5, assign74150_e112858_d_n6, assign74150_e112858_d_n7, assign74150_e112858_d_n8, assign74150_e112858_d_n9, assign74150_e112858_d_n10, assign74150_e112858_d_n11, assign74150_e112858_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) && (locals.var_guard1727 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign74150_e112858;
        locals.var_t9_dn0 = assign74150_e112858_d_n0;
        locals.var_t9_dn2 = assign74150_e112858_d_n2;
        locals.var_t9_dn4 = assign74150_e112858_d_n4;
        locals.var_t9_dn5 = assign74150_e112858_d_n5;
        locals.var_t9_dn6 = assign74150_e112858_d_n6;
        locals.var_t9_dn7 = assign74150_e112858_d_n7;
        locals.var_t9_dn8 = assign74150_e112858_d_n8;
        locals.var_t9_dn9 = assign74150_e112858_d_n9;
        locals.var_t9_dn10 = assign74150_e112858_d_n10;
        locals.var_t9_dn11 = assign74150_e112858_d_n11;
        locals.var_t9_dn14 = assign74150_e112858_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign74160_e112864, assign74160_e112864_d_n0, assign74160_e112864_d_n2, assign74160_e112864_d_n4, assign74160_e112864_d_n5, assign74160_e112864_d_n6, assign74160_e112864_d_n7, assign74160_e112864_d_n8, assign74160_e112864_d_n9, assign74160_e112864_d_n10, assign74160_e112864_d_n11, assign74160_e112864_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign74160_e112864;
        locals.var_ddriftldc_dn0 = assign74160_e112864_d_n0;
        locals.var_ddriftldc_dn2 = assign74160_e112864_d_n2;
        locals.var_ddriftldc_dn4 = assign74160_e112864_d_n4;
        locals.var_ddriftldc_dn5 = assign74160_e112864_d_n5;
        locals.var_ddriftldc_dn6 = assign74160_e112864_d_n6;
        locals.var_ddriftldc_dn7 = assign74160_e112864_d_n7;
        locals.var_ddriftldc_dn8 = assign74160_e112864_d_n8;
        locals.var_ddriftldc_dn9 = assign74160_e112864_d_n9;
        locals.var_ddriftldc_dn10 = assign74160_e112864_d_n10;
        locals.var_ddriftldc_dn11 = assign74160_e112864_d_n11;
        locals.var_ddriftldc_dn14 = assign74160_e112864_d_n14;
        locals.var_ddriftldc_rv = 0.0;

        let (assign74170_e112878, assign74170_e112878_d_n0, assign74170_e112878_d_n2, assign74170_e112878_d_n4, assign74170_e112878_d_n5, assign74170_e112878_d_n6, assign74170_e112878_d_n7, assign74170_e112878_d_n8, assign74170_e112878_d_n9, assign74170_e112878_d_n10, assign74170_e112878_d_n11, assign74170_e112878_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) {
        let assign74170_e112870: f64 = (locals.var_q_nsubld * locals.var_ddriftldc);
        let assign74170_e112872: f64 = (assign74170_e112870 * locals.var_ddriftldc);
        let assign74170_e112874: f64 = (assign74170_e112872 / 2.0);
        let assign74170_e112876: f64 = (assign74170_e112874 / 1.034943e-10);
        (assign74170_e112876, (((((locals.var_q_nsubld * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign74170_e112870 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign74170_e112870 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign74170_e112870 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign74170_e112870 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign74170_e112870 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign74170_e112870 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign74170_e112870 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign74170_e112870 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign74170_e112870 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign74170_e112870 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign74170_e112870 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign74170_e112878;
        locals.var_dphi_sb_dn0 = assign74170_e112878_d_n0;
        locals.var_dphi_sb_dn2 = assign74170_e112878_d_n2;
        locals.var_dphi_sb_dn4 = assign74170_e112878_d_n4;
        locals.var_dphi_sb_dn5 = assign74170_e112878_d_n5;
        locals.var_dphi_sb_dn6 = assign74170_e112878_d_n6;
        locals.var_dphi_sb_dn7 = assign74170_e112878_d_n7;
        locals.var_dphi_sb_dn8 = assign74170_e112878_d_n8;
        locals.var_dphi_sb_dn9 = assign74170_e112878_d_n9;
        locals.var_dphi_sb_dn10 = assign74170_e112878_d_n10;
        locals.var_dphi_sb_dn11 = assign74170_e112878_d_n11;
        locals.var_dphi_sb_dn14 = assign74170_e112878_d_n14;
        locals.var_dphi_sb_rv = 0.0;

        let (assign74180_e112889, assign74180_e112889_d_n0, assign74180_e112889_d_n2, assign74180_e112889_d_n4, assign74180_e112889_d_n5, assign74180_e112889_d_n6, assign74180_e112889_d_n7, assign74180_e112889_d_n8, assign74180_e112889_d_n9, assign74180_e112889_d_n10, assign74180_e112889_d_n11, assign74180_e112889_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) {
        let assign74180_e112884: f64 = (2.0 * locals.var_beta);
        let assign74180_e112886: f64 = (assign74180_e112884 * locals.var_dphi_sb);
        let assign74180_e112887: f64 = (assign74180_e112886).sqrt();
        (assign74180_e112887, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign74180_e112884 * locals.var_dphi_sb_dn0)) / (2.0 * assign74180_e112887)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign74180_e112884 * locals.var_dphi_sb_dn2)) / (2.0 * assign74180_e112887)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign74180_e112884 * locals.var_dphi_sb_dn4)) / (2.0 * assign74180_e112887)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign74180_e112884 * locals.var_dphi_sb_dn5)) / (2.0 * assign74180_e112887)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign74180_e112884 * locals.var_dphi_sb_dn6)) / (2.0 * assign74180_e112887)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign74180_e112884 * locals.var_dphi_sb_dn7)) / (2.0 * assign74180_e112887)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign74180_e112884 * locals.var_dphi_sb_dn8)) / (2.0 * assign74180_e112887)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign74180_e112884 * locals.var_dphi_sb_dn9)) / (2.0 * assign74180_e112887)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign74180_e112884 * locals.var_dphi_sb_dn10)) / (2.0 * assign74180_e112887)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign74180_e112884 * locals.var_dphi_sb_dn11)) / (2.0 * assign74180_e112887)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign74180_e112884 * locals.var_dphi_sb_dn14)) / (2.0 * assign74180_e112887)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign74180_e112889;
        locals.var_t0_dn0 = assign74180_e112889_d_n0;
        locals.var_t0_dn2 = assign74180_e112889_d_n2;
        locals.var_t0_dn4 = assign74180_e112889_d_n4;
        locals.var_t0_dn5 = assign74180_e112889_d_n5;
        locals.var_t0_dn6 = assign74180_e112889_d_n6;
        locals.var_t0_dn7 = assign74180_e112889_d_n7;
        locals.var_t0_dn8 = assign74180_e112889_d_n8;
        locals.var_t0_dn9 = assign74180_e112889_d_n9;
        locals.var_t0_dn10 = assign74180_e112889_d_n10;
        locals.var_t0_dn11 = assign74180_e112889_d_n11;
        locals.var_t0_dn14 = assign74180_e112889_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign74190_e112902, assign74190_e112902_d_n0, assign74190_e112902_d_n2, assign74190_e112902_d_n4, assign74190_e112902_d_n5, assign74190_e112902_d_n6, assign74190_e112902_d_n7, assign74190_e112902_d_n8, assign74190_e112902_d_n9, assign74190_e112902_d_n10, assign74190_e112902_d_n11, assign74190_e112902_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) {
        let assign74190_e112894: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign74190_e112896: f64 = (-locals.var_t0);
        let assign74190_e112897: f64 = { let limited_exp_arg = assign74190_e112896; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign74190_e112898: f64 = (assign74190_e112894 + assign74190_e112897);
        let assign74190_e112900: f64 = (assign74190_e112898 / 2.0);
        (assign74190_e112900, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign74190_e112896; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign74190_e112896; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign74190_e112896; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign74190_e112896; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign74190_e112896; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign74190_e112896; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign74190_e112896; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign74190_e112896; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign74190_e112896; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign74190_e112896; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign74190_e112896; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign74190_e112902;
        locals.var_t1_dn0 = assign74190_e112902_d_n0;
        locals.var_t1_dn2 = assign74190_e112902_d_n2;
        locals.var_t1_dn4 = assign74190_e112902_d_n4;
        locals.var_t1_dn5 = assign74190_e112902_d_n5;
        locals.var_t1_dn6 = assign74190_e112902_d_n6;
        locals.var_t1_dn7 = assign74190_e112902_d_n7;
        locals.var_t1_dn8 = assign74190_e112902_d_n8;
        locals.var_t1_dn9 = assign74190_e112902_d_n9;
        locals.var_t1_dn10 = assign74190_e112902_d_n10;
        locals.var_t1_dn11 = assign74190_e112902_d_n11;
        locals.var_t1_dn14 = assign74190_e112902_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign74200_e112911, assign74200_e112911_d_n0, assign74200_e112911_d_n2, assign74200_e112911_d_n4, assign74200_e112911_d_n5, assign74200_e112911_d_n6, assign74200_e112911_d_n7, assign74200_e112911_d_n8, assign74200_e112911_d_n9, assign74200_e112911_d_n10, assign74200_e112911_d_n11, assign74200_e112911_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) {
        let assign74200_e112907: f64 = (locals.var_t1).ln();
        let assign74200_e112909: f64 = (assign74200_e112907 / locals.var_dphi_sb);
        (assign74200_e112909, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign74200_e112907 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign74200_e112907 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign74200_e112907 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign74200_e112907 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign74200_e112907 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign74200_e112907 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign74200_e112907 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign74200_e112907 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign74200_e112907 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign74200_e112907 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign74200_e112907 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign74200_e112911;
        locals.var_c_sb_dn0 = assign74200_e112911_d_n0;
        locals.var_c_sb_dn2 = assign74200_e112911_d_n2;
        locals.var_c_sb_dn4 = assign74200_e112911_d_n4;
        locals.var_c_sb_dn5 = assign74200_e112911_d_n5;
        locals.var_c_sb_dn6 = assign74200_e112911_d_n6;
        locals.var_c_sb_dn7 = assign74200_e112911_d_n7;
        locals.var_c_sb_dn8 = assign74200_e112911_d_n8;
        locals.var_c_sb_dn9 = assign74200_e112911_d_n9;
        locals.var_c_sb_dn10 = assign74200_e112911_d_n10;
        locals.var_c_sb_dn11 = assign74200_e112911_d_n11;
        locals.var_c_sb_dn14 = assign74200_e112911_d_n14;
        locals.var_c_sb_rv = 0.0;

        let (assign74210_e112917,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1724 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign74210_e112917;
        locals.var_lp_s0_rv = 0.0;

    }
}
